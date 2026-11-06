//! The Linux-faithful TCP model behind DelayStream (unit U3a; the advisor's spec S0-S14 in
//! `.checkpoints/netcode-proof-test/advisor-u3q.md`; Mustafa's U3-Q3 (b), 2026-11-06: "model Linux TCP faithfully, so
//! the result shows how the fallback really behaves").
//!
//! One WebSocket connection, both directions, as pure logic on nanosecond stamps: no I/O and no clock. DelayStream
//! drives it from the release clock (U3c); the tests drive it on simulated time. End 0 is RS (it sends down), end 1
//! the client (it sends up). It models Linux 6.8 with its default sysctls. Every constant was read in v6.8:
//! include/net/tcp.h (TCP_RTO_MIN = HZ/5, TCP_RTO_MAX = 120 s, TCP_TIMEOUT_INIT = 1 s, TCP_DELACK_MIN = HZ/25,
//! TCP_TIMEOUT_MIN_US = 2 ms, TCP_INIT_CWND = 10), net/ipv4/tcp_cubic.c (beta = 717/1024), and the rules below in
//! tcp_input.c (tcp_rtt_estimator, tcp_rearm_rto, tcp_set_xmit_timer), tcp_output.c (tcp_schedule_loss_probe,
//! tcp_send_loss_probe), tcp_recovery.c (RACK) and tcp_timer.c (the retransmission timer).
//! - S1: segments of at most 1,448 B, Nagle off; each transmission's delay is the profile's half latency plus a uniform
//!   jitter of ± half its jitter, and its loss is drawn at the profile's rate; any transmission sent inside a burst is
//!   lost. Each direction's path is first in, first out, so the stream never reorders.
//! - S2: at most cwnd segments in flight, 10 at first. App-limited flows neither grow nor shrink it (Cubic's growth
//!   and PRR are not modelled); after a timeout, slow start from 1 to ssthresh = max(0.7 cwnd, 2), held there.
//! - S3: the receiver acknowledges at once an out-of-order segment, one that fills a hole, and a duplicate, with SACK
//!   blocks for every out-of-order range; in-order data at the earliest of 40 ms after it arrived, a second
//!   unacknowledged segment, or the next segment its own end sends (the acknowledgement rides on it).
//! - S4: Linux's RTT estimator in integer microseconds; RTO = srtt + rttvar, rttvar never below 200 ms; 1 s before the
//!   first sample; Karn's rule; one sample per acknowledgement, from the newest segment it newly delivers.
//! - S5: RACK (RFC 8985): a segment is lost once one sent after it is delivered and its own send plus RACK's RTT plus
//!   the reordering window, min(min_rtt / 4, srtt) (0 out of the Open state or once 3 segments are SACKed), has
//!   passed; else the reorder timer fires then, plus 2 ms.
//! - S6: the tail-loss probe, in the Open state with data out: 2 x srtt plus 200 ms with one segment out (else 2 ms),
//!   at most the RTO's remaining time; it sends one new segment if there is one, past cwnd, else resends the last;
//!   one per episode; then the RTO restarts.
//! - S7: the RTO, at the head's last transmission plus RTO: every outstanding segment lost, the head resent, the RTO
//!   doubled up to 120 s; the next valid RTT sample resets the backoff.
//! - S8: every resend draws its own fate. S9: bytes are released in order; the write side holds at most 64 KB unsent
//!   or unacknowledged.
//! - S10: fates from one ChaCha8 stream per (seed, connection, direction), four words per transmission.
//! - Not modelled (S11), recorded in each WebSocket run's header: Cubic's growth curve and PRR, F-RTO's undo, DSACK's
//!   reordering window growth, the delayed-ACK timer above 40 ms, quick-ack mode, TSO and GSO, ECN, the persist timer,
//!   RTT samples from the timestamp option, jiffy rounding.

use npt_l1::Profile;
use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};
use std::collections::{BTreeMap, VecDeque};

/// The model's name and version, for each WebSocket run's header (S11); DelayStream's header writes it from U3c.
#[allow(dead_code)]
pub(crate) const MODEL: &str = "linux-6.8-tcp v1";
/// 1,500 B MTU less IPv4 (20 B), TCP (20 B) and the timestamp option (12 B).
pub(crate) const MSS: u64 = 1_448;
/// The write side's bound (ADR 0001:442): unsent plus unacknowledged bytes.
pub(crate) const WRITE_BUDGET: u64 = 64 * 1024;
const INIT_CWND: u64 = 10;
const RTO_MIN_US: i64 = 200_000;
const RTO_MAX_US: u64 = 120_000_000;
const RTO_INIT_US: u64 = 1_000_000;
const DELACK_NS: u64 = 40_000_000;
const TIMEOUT_MIN_US: u64 = 2_000;
const CUBIC_BETA: u64 = 717;
const BETA_SCALE: u64 = 1_024;
/// tcp_reordering: this many SACKed segments set RACK's reordering window to 0.
const REORDERING: usize = 3;
const WORDS: usize = 4;
const KEY_TAG: &[u8; 24] = b"npt-net tcp model, v1\0\0\0";
const US: u64 = 1_000;

/// Why a segment was resent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Repair {
    Rack,
    Probe,
    Timeout,
}

/// One direction's counts (S12), kept by the end that sends it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Counters {
    /// Data segments formed.
    pub segments: u64,
    /// Data transmissions, resends included.
    pub transmissions: u64,
    pub lost_profile: u64,
    pub lost_burst: u64,
    /// Segments whose first transmission was lost.
    pub first_lost: u64,
    pub pure_acks: u64,
    pub pure_acks_lost: u64,
    pub repairs_rack: u64,
    pub repairs_probe: u64,
    pub repairs_timeout: u64,
    pub max_backoff: u32,
    /// Segments first received later than their first transmission's drawn arrival, and how late.
    pub stalls: u64,
    pub stall_ns_max: u64,
    pub stall_ns_sum: u64,
    /// Out-of-order bytes held at the receiver, the most at once.
    pub held_bytes_max: u64,
}

/// The planted defects of U3's own tests (S13's bites); never in a build that runs.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variant {
    Faithful,
    /// E15's old rule in place of RACK and the probe: every repair one fixed timeout, max(200 ms, 2 x RTT).
    ConstantRepair,
    /// The RTO never backs off.
    NoDoubling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Open,
    Recovery,
    Loss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimerKind {
    Rto,
    Probe,
    Reorder,
}

#[derive(Debug, Clone)]
struct Seg {
    start: u64,
    end: u64,
    /// The last transmission's time.
    xmit_ns: u64,
    /// The first transmission's arrival as drawn, lost or not: the stall reference.
    first_arrival_ns: u64,
    sacked: bool,
    /// Marked lost, waiting for its resend.
    lost: bool,
    /// A transmission of it counts as in flight.
    outstanding: bool,
    /// Resent at least once (Karn's rule).
    retrans: bool,
    attempts: u32,
    cause: Option<Repair>,
    received: bool,
}

/// Linux's tcp_rtt_estimator state, in microseconds (srtt8 is 8 x srtt, as the kernel stores it).
#[derive(Debug, Clone, Copy, Default)]
struct Rtt {
    srtt8: i64,
    mdev: i64,
    mdev_max: i64,
    rttvar: i64,
    rtt_seq: u64,
    min_rtt: Option<u64>,
}

impl Rtt {
    fn has(&self) -> bool {
        self.srtt8 != 0
    }

    fn srtt_us(&self) -> u64 {
        u64::try_from(self.srtt8 >> 3).unwrap_or(0)
    }

    /// tcp_rtt_estimator (tcp_input.c:861-927), line for line.
    fn sample(&mut self, mrtt_us: i64, snd_una: u64, snd_nxt: u64) {
        let mut m = mrtt_us;
        if self.srtt8 != 0 {
            m -= self.srtt8 >> 3;
            self.srtt8 += m;
            if m < 0 {
                m = -m;
                m -= self.mdev >> 2;
                if m > 0 {
                    m >>= 3;
                }
            } else {
                m -= self.mdev >> 2;
            }
            self.mdev += m;
            if self.mdev > self.mdev_max {
                self.mdev_max = self.mdev;
                if self.mdev_max > self.rttvar {
                    self.rttvar = self.mdev_max;
                }
            }
            if snd_una > self.rtt_seq {
                if self.mdev_max < self.rttvar {
                    self.rttvar -= (self.rttvar - self.mdev_max) >> 2;
                }
                self.rtt_seq = snd_nxt;
                self.mdev_max = RTO_MIN_US;
            }
        } else {
            self.srtt8 = m << 3;
            self.mdev = m << 1;
            self.rttvar = self.mdev.max(RTO_MIN_US);
            self.mdev_max = self.rttvar;
            self.rtt_seq = snd_nxt;
        }
        self.srtt8 = self.srtt8.max(1);
        let m = u64::try_from(mrtt_us).unwrap_or(0);
        self.min_rtt = Some(self.min_rtt.map_or(m, |old| old.min(m)));
    }

    /// __tcp_set_rto and tcp_bound_rto: srtt + rttvar, at most 120 s; 1 s before the first sample.
    fn rto_us(&self) -> u64 {
        if !self.has() {
            return RTO_INIT_US;
        }
        u64::try_from((self.srtt8 >> 3) + self.rttvar)
            .unwrap_or(RTO_MAX_US)
            .min(RTO_MAX_US)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Rack {
    set: bool,
    rtt_us: u64,
    mstamp_ns: u64,
    end_seq: u64,
}

/// RACK's "sent after": a later send, or the same send of a later sequence (tcp_rack_sent_after).
fn sent_after(t1: u64, t2: u64, seq1: u64, seq2: u64) -> bool {
    t1 > t2 || (t1 == t2 && seq1 > seq2)
}

#[derive(Debug, Clone)]
struct Sender {
    write_seq: u64,
    snd_una: u64,
    snd_nxt: u64,
    segs: VecDeque<Seg>,
    cwnd: u64,
    ssthresh: u64,
    rtt: Rtt,
    backoff: u32,
    rack: Rack,
    state: State,
    high_seq: u64,
    tlp_high_seq: Option<u64>,
    timer: Option<(u64, TimerKind)>,
    /// tcp_cwnd_validate's window: the most segments out in it, the sequence that ends it, and whether cwnd held a
    /// send back in it (tcp_is_cwnd_limited reads them: slow start grows cwnd only when the flow is cwnd-limited).
    max_out: u64,
    max_out_seq: u64,
    cwnd_blocked: bool,
    /// The constant-repair bite's fixed timeout (tests only).
    #[cfg(test)]
    fixed_rto_ns: Option<u64>,
}

impl Sender {
    fn new() -> Sender {
        Sender {
            write_seq: 0,
            snd_una: 0,
            snd_nxt: 0,
            segs: VecDeque::new(),
            cwnd: INIT_CWND,
            ssthresh: u64::MAX,
            rtt: Rtt::default(),
            backoff: 0,
            rack: Rack::default(),
            state: State::Open,
            high_seq: 0,
            tlp_high_seq: None,
            timer: None,
            max_out: 0,
            max_out_seq: 0,
            cwnd_blocked: false,
            #[cfg(test)]
            fixed_rto_ns: None,
        }
    }

    /// tcp_is_cwnd_limited.
    fn cwnd_limited(&self) -> bool {
        self.cwnd_blocked || self.cwnd < 2 * self.max_out
    }

    /// tcp_cwnd_validate, after sends and acknowledgements.
    fn validate_window(&mut self) {
        let out = self.segs.len() as u64;
        if self.snd_una >= self.max_out_seq || out > self.max_out {
            self.max_out = out;
            self.max_out_seq = self.snd_nxt;
            self.cwnd_blocked = false;
        }
    }

    fn in_flight(&self) -> u64 {
        self.segs.iter().filter(|g| g.outstanding).count() as u64
    }

    fn sacked_out(&self) -> usize {
        self.segs.iter().filter(|g| g.sacked).count()
    }

    fn rto_ns(&self) -> u64 {
        #[cfg(test)]
        if let Some(f) = self.fixed_rto_ns {
            return f;
        }
        let base = self.rtt.rto_us();
        let backed = base.checked_shl(self.backoff).unwrap_or(RTO_MAX_US);
        backed.min(RTO_MAX_US) * US
    }

    /// tcp_rearm_rto.
    fn rearm_rto(&mut self, now: u64) {
        let Some(head) = self.segs.front() else {
            self.timer = None;
            return;
        };
        let rto = self.rto_ns();
        let at = match self.timer {
            Some((_, TimerKind::Reorder | TimerKind::Probe)) => (head.xmit_ns + rto).max(now + US),
            _ => now + rto,
        };
        self.timer = Some((at, TimerKind::Rto));
    }

    /// tcp_schedule_loss_probe; false when no probe may be armed.
    fn schedule_probe(&mut self, now: u64, advancing_rto: bool) -> bool {
        if self.segs.is_empty() || self.state != State::Open {
            return false;
        }
        let mut timeout = if self.rtt.has() {
            let extra = if self.segs.len() == 1 {
                RTO_MIN_US.unsigned_abs()
            } else {
                TIMEOUT_MIN_US
            };
            (2 * self.rtt.srtt_us() + extra) * US
        } else {
            RTO_INIT_US * US
        };
        let rto_delta = if advancing_rto {
            self.rto_ns()
        } else {
            let head = self.segs.front().map_or(now, |h| h.xmit_ns);
            (head + self.rto_ns()).saturating_sub(now)
        };
        if rto_delta > 0 {
            timeout = timeout.min(rto_delta);
        }
        self.timer = Some((now + timeout, TimerKind::Probe));
        true
    }

    /// The first RTO of an episode, or entering recovery, sets ssthresh (tcp_enter_loss, tcp_enter_recovery).
    fn cut_ssthresh(&mut self) {
        self.ssthresh = (self.cwnd * CUBIC_BETA / BETA_SCALE).max(2);
    }
}

#[derive(Debug, Clone, Default)]
struct Receiver {
    rcv_nxt: u64,
    ooo: BTreeMap<u64, u64>,
    unacked_segments: u32,
    ack_now: bool,
    delack_at: Option<u64>,
}

impl Receiver {
    fn ack(&self) -> Ack {
        Ack {
            cum: self.rcv_nxt,
            sacks: self.ooo.iter().map(|(&s, &e)| (s, e)).collect(),
        }
    }

    fn held_bytes(&self) -> u64 {
        self.ooo.iter().map(|(s, e)| e - s).sum()
    }

    /// Accepts [s, e) and says whether it arrived out of order or filled a hole.
    fn accept(&mut self, s: u64, e: u64) -> Arrival {
        if e <= self.rcv_nxt {
            return Arrival::Duplicate;
        }
        if s <= self.rcv_nxt {
            let had_hole = !self.ooo.is_empty();
            self.rcv_nxt = e;
            while let Some((&os, &oe)) = self.ooo.iter().next() {
                if os > self.rcv_nxt {
                    break;
                }
                self.ooo.remove(&os);
                self.rcv_nxt = self.rcv_nxt.max(oe);
            }
            return if had_hole {
                Arrival::FilledHole
            } else {
                Arrival::InOrder
            };
        }
        // Out of order: merge [s, e) with every range it touches.
        let (mut ns, mut ne) = (s, e);
        let touching: Vec<(u64, u64)> = self
            .ooo
            .range(..=ne)
            .filter(|&(&os, &oe)| oe >= ns && os <= ne)
            .map(|(&os, &oe)| (os, oe))
            .collect();
        for (os, oe) in touching {
            self.ooo.remove(&os);
            ns = ns.min(os);
            ne = ne.max(oe);
        }
        self.ooo.insert(ns, ne);
        Arrival::OutOfOrder
    }
}

enum Arrival {
    Duplicate,
    InOrder,
    FilledHole,
    OutOfOrder,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ack {
    cum: u64,
    sacks: Vec<(u64, u64)>,
}

#[derive(Debug, Clone)]
struct Packet {
    arrival_ns: u64,
    data: Option<(u64, u64)>,
    ack: Ack,
}

/// One end's sending path: its fate stream and its first-in, first-out queue (arrivals never decrease).
#[derive(Debug, Clone)]
struct Path {
    rng: ChaCha8Rng,
    last_arrival: u64,
    queue: VecDeque<Packet>,
}

#[derive(Debug, Clone)]
struct End {
    snd: Sender,
    rcv: Receiver,
    counters: Counters,
}

/// One modelled connection.
#[derive(Debug, Clone)]
pub(crate) struct TcpModel {
    one_way: u64,
    half_jitter: u64,
    loss_ppm: u32,
    bursts: Vec<(u64, u64)>,
    ends: [End; 2],
    paths: [Path; 2],
    now: u64,
    #[cfg(test)]
    variant: Variant,
    /// (end, segment start, attempt) transmissions the tests force lost.
    #[cfg(test)]
    forced: Vec<(usize, u64, u32)>,
    /// Every data transmission: (end, segment start, attempt, time, lost).
    #[cfg(test)]
    sent_log: Vec<(usize, u64, u32, u64, bool)>,
}

/// A uniform integer in [0, n) from one 64-bit draw (multiply and shift, as npt-l1 draws).
fn below(word: u64, n: u64) -> u64 {
    ((u128::from(word) * u128::from(n)) >> 64) as u64
}

impl TcpModel {
    /// A connection under `profile` (its half latency, half jitter, loss and bursts each way), with fates from `seed`
    /// for connection number `conn`.
    pub(crate) fn new(profile: &Profile, seed: u64, conn: u32) -> TcpModel {
        let mut key = [0u8; 32];
        key[..8].copy_from_slice(&seed.to_le_bytes());
        key[8..].copy_from_slice(KEY_TAG);
        let path = |dir: u64| {
            let mut rng = ChaCha8Rng::from_seed(key);
            rng.set_stream((u64::from(conn) << 8) | dir);
            Path {
                rng,
                last_arrival: 0,
                queue: VecDeque::new(),
            }
        };
        let end = || End {
            snd: Sender::new(),
            rcv: Receiver::default(),
            counters: Counters::default(),
        };
        TcpModel {
            one_way: profile.one_way_ns(),
            half_jitter: profile.half_jitter_ns(),
            loss_ppm: profile.loss_ppm,
            bursts: profile.bursts_ns.clone(),
            ends: [end(), end()],
            paths: [path(0), path(1)],
            now: 0,
            #[cfg(test)]
            variant: Variant::Faithful,
            #[cfg(test)]
            forced: Vec::new(),
            #[cfg(test)]
            sent_log: Vec::new(),
        }
    }

    /// Bytes `end` may write now (S9).
    pub(crate) fn free_budget(&self, end: usize) -> u64 {
        let s = &self.ends[end].snd;
        WRITE_BUDGET.saturating_sub(s.write_seq - s.snd_una)
    }

    /// Writes up to `bytes` at `now` on `end`'s side and returns how many were taken (at most the free budget).
    pub(crate) fn write(&mut self, end: usize, bytes: u64, now: u64) -> u64 {
        self.advance(now);
        let n = bytes.min(self.free_budget(end));
        if n > 0 {
            self.ends[end].snd.write_seq += n;
            self.push(end, now);
        }
        n
    }

    /// Bytes released in order at `end` so far (sent by the other end).
    pub(crate) fn delivered(&self, end: usize) -> u64 {
        self.ends[end].rcv.rcv_nxt
    }

    /// The counts of the direction `end` sends.
    pub(crate) fn counters(&self, end: usize) -> Counters {
        self.ends[end].counters
    }

    /// When the next internal event is due, if any.
    pub(crate) fn next_event(&self) -> Option<u64> {
        self.next().map(|(t, _)| t)
    }

    /// Runs every event due by `now`, in order.
    pub(crate) fn advance(&mut self, now: u64) {
        while let Some((t, ev)) = self.next() {
            if t > now {
                break;
            }
            self.dispatch(t, ev);
        }
        self.now = self.now.max(now);
    }

    /// The earliest event: arrivals first, then delayed acknowledgements, then the senders' timers.
    fn next(&self) -> Option<(u64, Event)> {
        let mut best: Option<(u64, u8, Event)> = None;
        let mut consider = |t: u64, rank: u8, ev: Event| {
            if best.is_none_or(|(bt, br, _)| (t, rank) < (bt, br)) {
                best = Some((t, rank, ev));
            }
        };
        for p in 0..2 {
            if let Some(pk) = self.paths[p].queue.front() {
                consider(pk.arrival_ns, p as u8, Event::Arrival { to: 1 - p });
            }
        }
        for e in 0..2 {
            if let Some(t) = self.ends[e].rcv.delack_at {
                consider(t, 2 + e as u8, Event::DelayedAck { end: e });
            }
            if let Some((t, _)) = self.ends[e].snd.timer {
                consider(t, 4 + e as u8, Event::Timer { end: e });
            }
        }
        best.map(|(t, _, ev)| (t, ev))
    }

    fn dispatch(&mut self, t: u64, ev: Event) {
        match ev {
            Event::Arrival { to } => {
                let from = 1 - to;
                let Some(pk) = self.paths[from].queue.pop_front() else {
                    return;
                };
                self.on_ack(to, &pk.ack, t);
                if let Some((s, e)) = pk.data {
                    self.on_data(to, s, e, t);
                }
                self.push(to, t);
            }
            Event::DelayedAck { end } => {
                self.ends[end].rcv.delack_at = None;
                self.transmit(end, None, 0, t);
            }
            Event::Timer { end } => {
                let Some((_, kind)) = self.ends[end].snd.timer.take() else {
                    return;
                };
                match kind {
                    TimerKind::Rto => self.on_rto(end, t),
                    TimerKind::Probe => self.on_probe(end, t),
                    TimerKind::Reorder => self.on_reorder(end, t),
                }
            }
        }
    }

    fn in_burst(&self, t: u64) -> bool {
        self.bursts
            .iter()
            .any(|&(start, len)| t >= start && t - start < len)
    }

    /// One transmission from `end`: data or a pure acknowledgement, carrying `end`'s acknowledgement state. Returns its
    /// drawn arrival and whether it was lost.
    fn transmit(
        &mut self,
        end: usize,
        data: Option<(u64, u64)>,
        attempt: u32,
        now: u64,
    ) -> (u64, bool) {
        let ack = self.ends[end].rcv.ack();
        let rcv = &mut self.ends[end].rcv;
        rcv.ack_now = false;
        rcv.delack_at = None;
        rcv.unacked_segments = 0;
        let mut w = [0u64; WORDS];
        for x in &mut w {
            *x = self.paths[end].rng.next_u64();
        }
        let a = self.half_jitter;
        let delay = (self.one_way + below(w[1], 2 * a + 1)).saturating_sub(a);
        let arrival = (now + delay).max(self.paths[end].last_arrival);
        let burst = self.in_burst(now);
        #[allow(unused_mut)]
        let mut lost = burst || below(w[0], 1_000_000) < u64::from(self.loss_ppm);
        #[cfg(test)]
        if let Some((s, _)) = data
            && self.forced.contains(&(end, s, attempt))
        {
            lost = true;
        }
        let c = &mut self.ends[end].counters;
        if data.is_some() {
            c.transmissions += 1;
            if burst {
                c.lost_burst += 1;
            } else if lost {
                c.lost_profile += 1;
            }
        } else {
            c.pure_acks += 1;
            c.pure_acks_lost += u64::from(lost);
        }
        #[cfg(test)]
        if let Some((s, _)) = data {
            self.sent_log.push((end, s, attempt, now, lost));
        }
        let _ = attempt;
        if !lost {
            let path = &mut self.paths[end];
            path.last_arrival = arrival;
            path.queue.push_back(Packet {
                arrival_ns: arrival,
                data,
                ack,
            });
        }
        (arrival, lost)
    }

    /// Sends what the window allows (resends first, then new data), arms the timers, and acknowledges if still owed.
    fn push(&mut self, e: usize, now: u64) {
        let prior_packets = self.ends[e].snd.segs.len();
        let mut sent_new = false;
        loop {
            let s = &mut self.ends[e].snd;
            if s.in_flight() >= s.cwnd {
                if s.segs.iter().any(|g| g.lost) || s.snd_nxt < s.write_seq {
                    s.cwnd_blocked = true;
                }
                break;
            }
            if let Some(i) = s.segs.iter().position(|g| g.lost) {
                self.resend(e, i, now);
            } else if s.snd_nxt < s.write_seq {
                self.send_new(e, now);
                sent_new = true;
            } else {
                break;
            }
        }
        if sent_new {
            // tcp_event_new_data_sent, then tcp_write_xmit's probe.
            let s = &mut self.ends[e].snd;
            if prior_packets == 0 || matches!(s.timer, Some((_, TimerKind::Probe))) {
                s.rearm_rto(now);
            }
            if self.probes_on() {
                self.ends[e].snd.schedule_probe(now, false);
            }
        }
        if self.ends[e].snd.timer.is_none() && !self.ends[e].snd.segs.is_empty() {
            self.ends[e].snd.rearm_rto(now);
        }
        self.ends[e].snd.validate_window();
        let r = &self.ends[e].rcv;
        if r.ack_now {
            self.transmit(e, None, 0, now);
        }
    }

    fn send_new(&mut self, e: usize, now: u64) {
        let s = &mut self.ends[e].snd;
        let start = s.snd_nxt;
        let end = (start + MSS).min(s.write_seq);
        s.snd_nxt = end;
        let (arrival, lost) = self.transmit(e, Some((start, end)), 1, now);
        let c = &mut self.ends[e].counters;
        c.segments += 1;
        c.first_lost += u64::from(lost);
        self.ends[e].snd.segs.push_back(Seg {
            start,
            end,
            xmit_ns: now,
            first_arrival_ns: arrival,
            sacked: false,
            lost: false,
            outstanding: true,
            retrans: false,
            attempts: 1,
            cause: None,
            received: false,
        });
    }

    fn resend(&mut self, e: usize, i: usize, now: u64) {
        let (start, end, attempt, cause) = {
            let g = &mut self.ends[e].snd.segs[i];
            g.attempts += 1;
            (g.start, g.end, g.attempts, g.cause)
        };
        self.transmit(e, Some((start, end)), attempt, now);
        let s = &mut self.ends[e].snd;
        let g = &mut s.segs[i];
        g.xmit_ns = now;
        g.lost = false;
        g.outstanding = true;
        g.retrans = true;
        let c = &mut self.ends[e].counters;
        match cause {
            Some(Repair::Rack) => c.repairs_rack += 1,
            Some(Repair::Probe) => c.repairs_probe += 1,
            Some(Repair::Timeout) | None => c.repairs_timeout += 1,
        }
        // tcp_xmit_retransmit_queue: resending the head restarts the RTO, unless the reorder timer is pending.
        let s = &mut self.ends[e].snd;
        if i == 0 && !matches!(s.timer, Some((_, TimerKind::Reorder))) {
            s.timer = Some((now + s.rto_ns(), TimerKind::Rto));
        }
    }

    /// The acknowledgement in an arrival at `e`, for the data `e` sent.
    fn on_ack(&mut self, e: usize, ack: &Ack, now: u64) {
        let s = &mut self.ends[e].snd;
        let prior_una = s.snd_una;
        // (xmit, retrans, end) of every segment this acknowledgement newly delivers.
        let mut delivered: Vec<(u64, bool, u64)> = Vec::new();
        while s.segs.front().is_some_and(|g| g.end <= ack.cum) {
            if let Some(g) = s.segs.pop_front()
                && !g.sacked
            {
                delivered.push((g.xmit_ns, g.retrans, g.end));
            }
        }
        if ack.cum > s.snd_una {
            s.snd_una = ack.cum.min(s.snd_nxt);
        }
        for g in s.segs.iter_mut().filter(|g| !g.sacked) {
            if ack.sacks.iter().any(|&(a, b)| a <= g.start && g.end <= b) {
                g.sacked = true;
                g.outstanding = false;
                g.lost = false;
                delivered.push((g.xmit_ns, g.retrans, g.end));
            }
        }
        if delivered.is_empty() && s.snd_una == prior_una {
            return;
        }
        // RTT: the newest newly delivered segment that was never resent (Karn).
        if let Some(&(x, _, _)) = delivered.iter().filter(|d| !d.1).max_by_key(|d| (d.0, d.2)) {
            let m = i64::try_from((now - x) / US).unwrap_or(i64::MAX);
            s.rtt.sample(m, s.snd_una, s.snd_nxt);
            s.backoff = 0;
        }
        // RACK: the most recently sent delivered segment (tcp_rack_advance).
        for &(x, retrans, end) in &delivered {
            let rtt_us = (now - x) / US;
            if retrans && s.rtt.min_rtt.is_some_and(|m| rtt_us < m) {
                continue;
            }
            if !s.rack.set || sent_after(x, s.rack.mstamp_ns, end, s.rack.end_seq) {
                s.rack = Rack {
                    set: true,
                    rtt_us,
                    mstamp_ns: x,
                    end_seq: end,
                };
            }
        }
        if s.tlp_high_seq.is_some_and(|h| ack.cum > h) {
            s.tlp_high_seq = None;
        }
        if s.state != State::Open && s.snd_una >= s.high_seq {
            s.state = State::Open;
        }
        // Slow start below ssthresh while cwnd-limited (tcp_cong_avoid's guard); growth above it is not modelled (S2).
        if s.cwnd < s.ssthresh && s.cwnd_limited() {
            s.cwnd = (s.cwnd + delivered.len() as u64).min(s.ssthresh);
        }
        s.validate_window();
        let reorder_armed = self.rack_mark_lost(e, now, true);
        if self.ends[e].snd.snd_una > prior_una && !reorder_armed {
            // tcp_set_xmit_timer.
            if !(self.probes_on() && self.ends[e].snd.schedule_probe(now, true)) {
                self.ends[e].snd.rearm_rto(now);
            }
        }
    }

    /// RACK's loss detection; with `arm`, sets the reorder timer for a segment not yet past its deadline. Returns
    /// whether it set that timer.
    fn rack_mark_lost(&mut self, e: usize, now: u64, arm: bool) -> bool {
        if !self.rack_on() {
            return false;
        }
        let s = &mut self.ends[e].snd;
        if !s.rack.set {
            return false;
        }
        let reo_wnd = if s.state != State::Open || s.sacked_out() >= REORDERING {
            0
        } else {
            (s.rtt.min_rtt.unwrap_or(0) / 4).min(s.rtt.srtt_us())
        };
        let rack = s.rack;
        let mut reo_timeout: u64 = 0;
        let mut marked = false;
        for g in s.segs.iter_mut().filter(|g| g.outstanding && !g.sacked) {
            if !sent_after(rack.mstamp_ns, g.xmit_ns, rack.end_seq, g.end) {
                continue;
            }
            let elapsed = (now - g.xmit_ns) / US;
            let deadline = rack.rtt_us + reo_wnd;
            if elapsed >= deadline {
                g.lost = true;
                g.outstanding = false;
                g.cause = Some(Repair::Rack);
                marked = true;
            } else {
                reo_timeout = reo_timeout.max(deadline - elapsed);
            }
        }
        if marked && s.state == State::Open {
            s.cut_ssthresh();
            s.state = State::Recovery;
            s.high_seq = s.snd_nxt;
        }
        if arm && reo_timeout > 0 {
            s.timer = Some((
                now + (reo_timeout + TIMEOUT_MIN_US) * US,
                TimerKind::Reorder,
            ));
            return true;
        }
        false
    }

    /// tcp_retransmit_timer.
    fn on_rto(&mut self, e: usize, now: u64) {
        let doubling = self.doubling_on();
        let s = &mut self.ends[e].snd;
        if s.segs.is_empty() {
            return;
        }
        if s.state == State::Open || (s.state == State::Loss && s.backoff == 0) {
            s.cut_ssthresh();
        }
        for g in s.segs.iter_mut().filter(|g| !g.sacked) {
            g.lost = true;
            g.outstanding = false;
            g.cause = Some(Repair::Timeout);
        }
        s.cwnd = 1;
        s.state = State::Loss;
        s.high_seq = s.snd_nxt;
        s.tlp_high_seq = None;
        if doubling {
            s.backoff += 1;
        }
        let b = s.backoff;
        let c = &mut self.ends[e].counters;
        c.max_backoff = c.max_backoff.max(b);
        self.push(e, now);
    }

    /// tcp_send_loss_probe. The probe timer counts as pending while it runs, as icsk_pending does in the kernel.
    fn on_probe(&mut self, e: usize, now: u64) {
        let s = &mut self.ends[e].snd;
        s.timer = Some((now, TimerKind::Probe));
        if s.tlp_high_seq.is_some() || s.segs.is_empty() {
            s.rearm_rto(now);
            return;
        }
        if s.snd_nxt < s.write_seq {
            self.send_new(e, now);
        } else if let Some(i) = self.ends[e].snd.segs.iter().rposition(|g| !g.sacked) {
            self.ends[e].snd.segs[i].cause = Some(Repair::Probe);
            self.resend(e, i, now);
        }
        let s = &mut self.ends[e].snd;
        s.tlp_high_seq = Some(s.snd_nxt);
        s.timer = None;
        s.rearm_rto(now);
        let r = &self.ends[e].rcv;
        if r.ack_now {
            self.transmit(e, None, 0, now);
        }
    }

    /// tcp_rack_reo_timeout. The reorder timer counts as pending while it runs, so resending the head does not restart
    /// the RTO and the rearm counts from the head (icsk_pending stays REO_TIMEOUT in the kernel's handler).
    fn on_reorder(&mut self, e: usize, now: u64) {
        self.ends[e].snd.timer = Some((now, TimerKind::Reorder));
        let prior = self.ends[e].snd.in_flight();
        self.rack_mark_lost(e, now, false);
        if self.ends[e].snd.in_flight() != prior {
            self.push(e, now);
        }
        let s = &mut self.ends[e].snd;
        if !matches!(s.timer, Some((_, TimerKind::Rto))) {
            s.rearm_rto(now);
        }
    }

    /// Data [s, x) arriving at `q` from the other end.
    fn on_data(&mut self, q: usize, s: u64, x: u64, now: u64) {
        let p = 1 - q;
        if let Ok(i) = self.ends[p].snd.segs.binary_search_by_key(&s, |g| g.start) {
            let g = &mut self.ends[p].snd.segs[i];
            if !g.received {
                g.received = true;
                if now > g.first_arrival_ns {
                    let late = now - g.first_arrival_ns;
                    let c = &mut self.ends[p].counters;
                    c.stalls += 1;
                    c.stall_ns_max = c.stall_ns_max.max(late);
                    c.stall_ns_sum += late;
                }
            }
        }
        let r = &mut self.ends[q].rcv;
        match r.accept(s, x) {
            Arrival::Duplicate | Arrival::FilledHole | Arrival::OutOfOrder => r.ack_now = true,
            Arrival::InOrder => {
                r.unacked_segments += 1;
                if r.unacked_segments >= 2 {
                    r.ack_now = true;
                } else if r.delack_at.is_none() {
                    r.delack_at = Some(now + DELACK_NS);
                }
            }
        }
        let held = r.held_bytes();
        let c = &mut self.ends[p].counters;
        c.held_bytes_max = c.held_bytes_max.max(held);
    }

    #[cfg(test)]
    fn rack_on(&self) -> bool {
        self.variant != Variant::ConstantRepair
    }
    #[cfg(not(test))]
    fn rack_on(&self) -> bool {
        true
    }
    #[cfg(test)]
    fn probes_on(&self) -> bool {
        self.variant != Variant::ConstantRepair
    }
    #[cfg(not(test))]
    fn probes_on(&self) -> bool {
        true
    }
    #[cfg(test)]
    fn doubling_on(&self) -> bool {
        self.variant == Variant::Faithful
    }
    #[cfg(not(test))]
    fn doubling_on(&self) -> bool {
        true
    }

    /// Plants one of S13's bites: the constant-repair bite gets E15's fixed timeout, max(200 ms, 2 x the round trip).
    #[cfg(test)]
    fn with_variant(mut self, v: Variant, profile: &Profile) -> TcpModel {
        self.variant = v;
        if v == Variant::ConstantRepair {
            let fixed = (200_000 * US).max(2 * profile.rtt_ns);
            for end in &mut self.ends {
                end.snd.fixed_rto_ns = Some(fixed);
            }
        }
        self
    }
}

#[derive(Debug, Clone, Copy)]
enum Event {
    Arrival { to: usize },
    DelayedAck { end: usize },
    Timer { end: usize },
}

#[cfg(test)]
mod tests;
