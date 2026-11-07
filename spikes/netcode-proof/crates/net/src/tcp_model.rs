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
//! - S1: segments of at most 1,448 B (1,460 B without the timestamp option), Nagle off; each transmission's delay is
//!   the profile's half latency plus a uniform jitter of ± half its jitter, and its loss is drawn at the profile's
//!   rate; any transmission sent inside a burst is lost. Each direction's path is first in, first out, so the stream
//!   never reorders.
//! - S2: at most cwnd segments in flight, 10 at first. App-limited flows neither grow nor shrink it (Cubic's growth
//!   and PRR are not modelled); after a timeout, slow start from 1 to ssthresh = max(0.7 cwnd, 2), held there.
//! - S3: the receiver acknowledges at once an out-of-order segment, one that fills a hole, and a duplicate, with SACK
//!   blocks for every out-of-order range; in-order data at the earliest of 40 ms after it arrived, a second
//!   unacknowledged segment, or the next segment its own end sends (the acknowledgement rides on it).
//! - S4: Linux's RTT estimator in integer microseconds; RTO = srtt + rttvar, rttvar never below 200 ms; 1 s before the
//!   first sample (3 s once a handshake ended without one: tcp_init_metrics). One sample per acknowledgement, taken as
//!   tcp_clean_rtx_queue and tcp_ack_update_rtt take it: from the oldest segment it newly delivers in order, none if
//!   any segment it delivers in order was ever resent (Karn); else from the first segment it newly SACKs that was never
//!   resent; else, with timestamps, from the echoed timestamp (TSecr, whole milliseconds) when it delivers data in
//!   order. The minimum RTT comes from the newest such segment. The skeptic of 2026-11-07 found the first version's
//!   rule (the newest segment, even beside a resent one) taking ~1.1 s samples after an outage.
//! - S5: RACK (RFC 8985): a segment is lost once one sent after it is delivered and its own send plus RACK's RTT plus
//!   the reordering window, min(min_rtt / 4, srtt) (0 out of the Open state or once 3 segments are SACKed), has
//!   passed; else the reorder timer fires then, plus 2 ms.
//! - S6: the tail-loss probe, in the Open state with data out: 2 x srtt plus 200 ms with one segment out (else 2 ms),
//!   at most the RTO's remaining time; it sends one new segment if there is one, past cwnd, else resends the last;
//!   one per episode; then the RTO restarts.
//! - S7: the RTO, at the head's last transmission plus RTO: every outstanding segment lost except those sent within
//!   RACK's RTT (never the head: tcp_timeout_mark_lost), cwnd = segments still in flight + 1 (tcp_enter_loss), the
//!   head resent, the RTO doubled up to 120 s; the next valid RTT sample resets the backoff.
//! - S8: every resend draws its own fate. S9: bytes are released in order; the write side holds at most 64 KB unsent
//!   or unacknowledged.
//! - S10: fates from one ChaCha8 stream per (seed, connection, direction), four words per transmission.
//! - The handshake (for reconnects): the SYN retried at a linear 1 s for 4 tries, then doubling; the SYN-ACK retried
//!   from 1 s, doubling per timer retry, and resent at once on a repeated SYN without moving that schedule
//!   (tcp_check_req), echoing the first SYN's timestamp; each end primed by the handshake's RTT, by the timestamp echo
//!   if it resent; cwnd 1 after more than one handshake resend (tcp_init_transfer).
//! - Not modelled (S11), recorded in each WebSocket run's header: Cubic's growth curve and PRR, F-RTO (its new data
//!   first after an RTO, and its undo), DSACK's reordering window growth, the delayed-ACK timer above 40 ms, quick-ack
//!   mode, TSO and GSO, ECN, the persist timer, the per-destination metrics cache, the rate limit on answers to
//!   repeated SYNs, the delayed-ACK filter of the minimum RTT, jiffy rounding.

use npt_l1::Profile;
use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};
use std::collections::{BTreeMap, VecDeque};

/// The model's name and version, for each WebSocket run's header (S11); DelayStream's header writes it from U3c.
#[allow(dead_code)]
pub(crate) const MODEL: &str = "linux-6.8-tcp v2";
/// 1,500 B MTU less IPv4 (20 B), TCP (20 B) and the timestamp option (12 B).
pub(crate) const MSS: u64 = 1_448;
/// The same without the timestamp option (a peer that does not offer it, as Windows does by default).
const MSS_NO_TS: u64 = 1_460;
/// The write side's bound (ADR 0001:1467, the Profiles key of part 13): unsent plus unacknowledged bytes.
pub(crate) const WRITE_BUDGET: u64 = 64 * 1024;
const INIT_CWND: u64 = 10;
const RTO_MIN_US: i64 = 200_000;
const RTO_MAX_US: u64 = 120_000_000;
const RTO_INIT_US: u64 = 1_000_000;
/// TCP_TIMEOUT_FALLBACK (include/net/tcp.h:151): the RTO after a handshake that gave no RTT sample.
const RTO_FALLBACK_US: u64 = 3_000_000;
/// The timestamp option's clock (TCP_TS_HZ = 1,000: whole milliseconds).
const TS_TICK_NS: u64 = 1_000_000;
const DELACK_NS: u64 = 40_000_000;
const TIMEOUT_MIN_US: u64 = 2_000;
const CUBIC_BETA: u64 = 717;
const BETA_SCALE: u64 = 1_024;
/// tcp_reordering: this many SACKed segments set RACK's reordering window to 0.
const REORDERING: usize = 3;
/// tcp_stream_is_thin: fewer than this many segments out (include/net/tcp.h:2119).
const THIN_PACKETS: usize = 4;
/// TCP_THIN_LINEAR_RETRIES (include/net/tcp.h:238): linear timeouts for this many retries, then doubling.
const THIN_LINEAR_RETRIES: u32 = 6;
/// tcp_syn_linear_timeouts' default (net/ipv4/tcp_ipv4.c:3489): this many SYN retries at a linear 1 s.
const SYN_LINEAR_TIMEOUTS: u32 = 4;
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
    /// SYNs (client) or SYN-ACKs (server) sent, and lost.
    pub handshake_packets: u64,
    pub handshake_lost: u64,
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

/// The server's TCP settings and the room's write budget: Linux's defaults, or the fallback's fixes (Mustafa,
/// 2026-11-06: newest snapshot only with a small send buffer; thin-stream linear timeouts and a lower minimum RTO).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TcpConfig {
    /// Unsent plus unacknowledged bytes the writer may hold (S9; a socket's SO_SNDBUF in payload bytes).
    pub write_budget: u64,
    /// tcp_thin_linear_timeouts / TCP_THIN_LINEAR_TIMEOUTS: no doubling while the stream is thin, for 6 retries
    /// (net/ipv4/tcp_timer.c:640-648).
    pub thin_linear_timeouts: bool,
    /// The minimum RTO (TCP_RTO_MIN, or a route's locked rto_min metric: include/net/tcp.h:741-748).
    pub rto_min_us: u64,
    /// The timestamp option on the connection (tcp_timestamps = 1 on Linux, the default, when the peer offers it too):
    /// TSecr RTT samples and a 1,448 B MSS; without it 1,460 B and Karn's rule alone.
    pub timestamps: bool,
}

impl Default for TcpConfig {
    /// Linux 6.8's defaults and the plan's 64 KB write bound.
    fn default() -> TcpConfig {
        TcpConfig {
            write_budget: WRITE_BUDGET,
            thin_linear_timeouts: false,
            rto_min_us: RTO_MIN_US.unsigned_abs(),
            timestamps: true,
        }
    }
}

/// A connection's three-way handshake (for the client's reconnect attempts): the client's SYN, retried at a linear
/// 1 s for its first 4 tries (tcp_syn_linear_timeouts) and doubling after; the server's SYN-ACK, retried from 1 s,
/// doubling per timer retry, and resent at once on a repeated SYN with the timer reset to the same step
/// (tcp_check_req: reqsk_timeout counts timer retries only); each end primed with the handshake's RTT, through the
/// timestamp echo if it resent (Karn).
#[derive(Debug, Clone, Copy, Default)]
struct Setup {
    syn_sent_ns: u64,
    syn_tries: u32,
    syn_timer: Option<u64>,
    synack_sent_ns: u64,
    /// SYN-ACKs sent (1 + req->num_retrans), and the timer's retries among them (req->num_timeout).
    synack_tries: u32,
    synack_timeouts: u32,
    synack_timer: Option<u64>,
    client_est: Option<u64>,
    server_est: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ctl {
    None,
    Syn,
    SynAck,
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
    /// The handshake gave no sample: the RTO is 3 s until the first one (tcp_init_metrics).
    fallback: bool,
}

impl Rtt {
    fn has(&self) -> bool {
        self.srtt8 != 0
    }

    fn srtt_us(&self) -> u64 {
        u64::try_from(self.srtt8 >> 3).unwrap_or(0)
    }

    /// One valid sample, as tcp_ack_update_rtt applies it: the minimum RTT from `ca_us`, the estimator from `mrtt_us`.
    fn sample(&mut self, mrtt_us: i64, ca_us: i64, snd_una: u64, snd_nxt: u64, rto_min_us: i64) {
        let ca = u64::try_from(ca_us.max(1)).unwrap_or(1);
        self.min_rtt = Some(self.min_rtt.map_or(ca, |old| old.min(ca)));
        self.estimate(mrtt_us, snd_una, snd_nxt, rto_min_us);
    }

    /// tcp_rtt_estimator (tcp_input.c:861-927), line for line.
    fn estimate(&mut self, mrtt_us: i64, snd_una: u64, snd_nxt: u64, rto_min_us: i64) {
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
                self.mdev_max = rto_min_us;
            }
        } else {
            self.srtt8 = m << 3;
            self.mdev = m << 1;
            self.rttvar = self.mdev.max(rto_min_us);
            self.mdev_max = self.rttvar;
            self.rtt_seq = snd_nxt;
        }
        self.srtt8 = self.srtt8.max(1);
    }

    /// __tcp_set_rto and tcp_bound_rto: srtt + rttvar, at most 120 s; 1 s before the first sample, 3 s after a
    /// handshake without one.
    fn rto_us(&self) -> u64 {
        if !self.has() {
            return if self.fallback {
                RTO_FALLBACK_US
            } else {
                RTO_INIT_US
            };
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
    /// The settings that differ from Linux's defaults when the fixes are on (TcpConfig).
    rto_min_us: i64,
    thin_lt: bool,
    /// icsk_retransmits: retransmission timeouts in this episode, reset when snd_una advances (tcp_input.c:3888).
    retransmits: u32,
    /// The constant-repair bite's fixed timeout (tests only).
    #[cfg(test)]
    fixed_rto_ns: Option<u64>,
}

impl Sender {
    fn new(cfg: &TcpConfig) -> Sender {
        Sender {
            rto_min_us: i64::try_from(cfg.rto_min_us).unwrap_or(RTO_MIN_US),
            thin_lt: cfg.thin_linear_timeouts,
            retransmits: 0,
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
                self.rto_min_us.unsigned_abs()
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
    /// rcv_nxt as of this end's last transmission (tcp_select_window sets rcv_wup there).
    rcv_wup: u64,
    /// The timestamp to echo: from the last arrival whose sequence was at most rcv_wup (tcp_replace_ts_recent).
    ts_recent: Option<u64>,
}

impl Receiver {
    fn ack(&self) -> Ack {
        Ack {
            cum: self.rcv_nxt,
            sacks: self.ooo.iter().map(|(&s, &e)| (s, e)).collect(),
            tsecr: self.ts_recent,
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
    /// The echoed timestamp (TSecr), in the option's milliseconds; `None` before the receiver has one.
    tsecr: Option<u64>,
}

#[derive(Debug, Clone)]
struct Packet {
    arrival_ns: u64,
    /// The sender's sequence: the data's start, or its snd_nxt for a packet without data.
    seq: u64,
    data: Option<(u64, u64)>,
    ack: Ack,
    ctl: Ctl,
    /// The timestamp option's value (TSval): the send time in whole milliseconds.
    tsval: u64,
}

/// The timestamp option's clock at `ns`.
fn ts_of(ns: u64) -> u64 {
    ns / TS_TICK_NS
}

/// tcp_rtt_tsopt_us: the echo's RTT in microseconds, whole milliseconds and at least one.
fn tsopt_rtt_us(now: u64, tsecr: u64) -> i64 {
    let delta = ts_of(now).saturating_sub(tsecr).max(1);
    i64::try_from(delta * (TS_TICK_NS / US)).unwrap_or(i64::MAX)
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
    write_budget: u64,
    /// The timestamp option (TcpConfig.timestamps) and the segment size it leaves.
    timestamps: bool,
    mss: u64,
    ends: [End; 2],
    paths: [Path; 2],
    now: u64,
    /// The handshake, for a connection opened by `connect`; `None` for one that starts established.
    setup: Option<Setup>,
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
    /// An established connection under `profile` (its half latency, half jitter, loss and bursts each way), with
    /// Linux's default settings and fates from `seed` for connection number `conn`.
    pub(crate) fn new(profile: &Profile, seed: u64, conn: u32) -> TcpModel {
        TcpModel::with_config(profile, seed, conn, TcpConfig::default())
    }

    /// A connection the client opens at `now`: its SYN leaves at once, and data may flow once `established` says so.
    pub(crate) fn connect(
        profile: &Profile,
        seed: u64,
        conn: u32,
        cfg: TcpConfig,
        now: u64,
    ) -> TcpModel {
        let mut m = TcpModel::with_config(profile, seed, conn, cfg);
        m.now = now;
        m.setup = Some(Setup {
            syn_sent_ns: now,
            syn_tries: 1,
            syn_timer: Some(now + RTO_INIT_US * US),
            ..Setup::default()
        });
        m.transmit_ctl(1, Ctl::Syn, now);
        m
    }

    /// Whether `end` has finished the handshake (always, for a connection that starts established).
    pub(crate) fn established(&self, end: usize) -> bool {
        self.setup.is_none_or(|s| {
            if end == 0 {
                s.server_est.is_some()
            } else {
                s.client_est.is_some()
            }
        })
    }

    /// An established connection with `cfg`'s settings.
    pub(crate) fn with_config(profile: &Profile, seed: u64, conn: u32, cfg: TcpConfig) -> TcpModel {
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
            snd: Sender::new(&cfg),
            rcv: Receiver::default(),
            counters: Counters::default(),
        };
        TcpModel {
            one_way: profile.one_way_ns(),
            half_jitter: profile.half_jitter_ns(),
            loss_ppm: profile.loss_ppm,
            bursts: profile.bursts_ns.clone(),
            write_budget: cfg.write_budget,
            timestamps: cfg.timestamps,
            mss: if cfg.timestamps { MSS } else { MSS_NO_TS },
            ends: [end(), end()],
            paths: [path(0), path(1)],
            now: 0,
            setup: None,
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
        self.write_budget.saturating_sub(s.write_seq - s.snd_una)
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
    /// `end`'s smoothed RTT, once it has a sample (what TCP_INFO's tcpi_rtt reports on a real socket).
    pub(crate) fn srtt_ns(&self, end: usize) -> Option<u64> {
        let r = &self.ends[end].snd.rtt;
        r.has().then(|| r.srtt_us() * US)
    }

    /// Bytes `end` has written and not yet had acknowledged (SIOCOUTQ on a real socket).
    pub(crate) fn outstanding(&self, end: usize) -> u64 {
        let s = &self.ends[end].snd;
        s.write_seq - s.snd_una
    }

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
        if let Some(s) = self.setup {
            if let Some(t) = s.syn_timer {
                consider(t, 6, Event::SynTimer);
            }
            if let Some(t) = s.synack_timer {
                consider(t, 7, Event::SynAckTimer);
            }
        }
        best.map(|(t, _, ev)| (t, ev))
    }

    /// A SYN or SYN-ACK from `end`: a transmission with its own fate, carrying no data.
    fn transmit_ctl(&mut self, end: usize, ctl: Ctl, now: u64) {
        self.transmit_any(end, None, ctl, 0, now);
    }

    /// The handshake's packets and timers (Setup's rules).
    fn on_ctl(&mut self, to: usize, pk: &Packet, now: u64) {
        let Some(mut s) = self.setup else {
            return;
        };
        match (to, pk.ctl) {
            (0, Ctl::Syn) if s.server_est.is_none() => {
                if s.synack_tries == 0 {
                    // The first SYN makes the request; every SYN-ACK echoes its timestamp (tcp_minisocks.c:678-716).
                    s.synack_sent_ns = now;
                    if self.timestamps {
                        self.ends[0].rcv.ts_recent = Some(pk.tsval);
                    }
                }
                // A first SYN, or a repeated one: the SYN-ACK goes (again) at once, the timer at its current step.
                s.synack_tries += 1;
                s.synack_timer = Some(now + (RTO_INIT_US << s.synack_timeouts.min(7)) * US);
                self.setup = Some(s);
                self.transmit_ctl(0, Ctl::SynAck, now);
            }
            (1, Ctl::SynAck) => {
                if s.client_est.is_none() {
                    s.client_est = Some(now);
                    s.syn_timer = None;
                    self.setup = Some(s);
                    if self.timestamps {
                        self.ends[1].rcv.ts_recent = Some(pk.tsval);
                    }
                    // The SYN's own RTT if it went once; else, with timestamps, the SYN-ACK's echo (Karn).
                    let sample = if s.syn_tries == 1 {
                        Some(i64::try_from((now - s.syn_sent_ns) / US).unwrap_or(i64::MAX))
                    } else if self.timestamps {
                        pk.ack.tsecr.map(|e| tsopt_rtt_us(now, e))
                    } else {
                        None
                    };
                    // The client's SYN timer ran in Loss, so it has an undo marker (tcp_enter_loss).
                    self.establish(1, sample, s.syn_tries - 1, true);
                }
                // The handshake's ACK (again, for a repeated SYN-ACK).
                self.transmit(1, None, 0, now);
            }
            _ => {}
        }
    }

    /// Any other packet from the client completes the server's side of the handshake (tcp_synack_rtt_meas).
    fn server_heard_client(&mut self, pk: &Packet, now: u64) {
        let Some(mut s) = self.setup else {
            return;
        };
        if s.server_est.is_none() && s.synack_tries > 0 {
            s.server_est = Some(now);
            s.synack_timer = None;
            self.setup = Some(s);
            let sample = if s.synack_tries == 1 {
                Some(i64::try_from((now - s.synack_sent_ns) / US).unwrap_or(i64::MAX))
            } else if self.timestamps {
                pk.ack.tsecr.map(|e| tsopt_rtt_us(now, e))
            } else {
                None
            };
            // The child gets an undo marker only if the request's timer resent (tcp_minisocks.c:594-596).
            self.establish(0, sample, s.synack_tries - 1, s.synack_timeouts > 0);
        }
    }

    /// One end's handshake done (tcp_init_transfer, tcp_init_metrics): primed by `sample` (else the RTO falls back to
    /// 3 s), and cwnd 1 after more than one resend when there is an undo marker.
    fn establish(&mut self, end: usize, sample: Option<i64>, resends: u32, undo: bool) {
        let snd = &mut self.ends[end].snd;
        match sample {
            Some(m) => snd.rtt.sample(m, m, 0, 0, snd.rto_min_us),
            None => snd.rtt.fallback = true,
        }
        if resends > 1 && undo {
            snd.cwnd = 1;
        }
    }

    fn dispatch(&mut self, t: u64, ev: Event) {
        match ev {
            Event::Arrival { to } => {
                let from = 1 - to;
                let Some(pk) = self.paths[from].queue.pop_front() else {
                    return;
                };
                if pk.ctl != Ctl::None {
                    self.on_ctl(to, &pk, t);
                    return;
                }
                if to == 0 {
                    self.server_heard_client(&pk, t);
                }
                if !self.established(to) {
                    return;
                }
                // tcp_replace_ts_recent: an in-window arrival (tcp_sequence: its end not below rcv_wup) that starts at
                // or below rcv_wup carries the timestamp to echo (PAWS-ordered).
                let r = &mut self.ends[to].rcv;
                let end = pk.data.map_or(pk.seq, |(_, e)| e);
                if self.timestamps
                    && pk.seq <= r.rcv_wup
                    && end >= r.rcv_wup
                    && r.ts_recent.is_none_or(|v| pk.tsval >= v)
                {
                    r.ts_recent = Some(pk.tsval);
                }
                self.on_ack(to, &pk.ack, t);
                if let Some((s, e)) = pk.data {
                    self.on_data(to, s, e, t);
                }
                self.push(to, t);
            }
            Event::SynTimer => {
                if let Some(mut s) = self.setup {
                    s.syn_tries += 1;
                    let linear = s.syn_tries - 1 <= SYN_LINEAR_TIMEOUTS;
                    let wait = if linear {
                        RTO_INIT_US
                    } else {
                        RTO_INIT_US << (s.syn_tries - 1 - SYN_LINEAR_TIMEOUTS).min(7)
                    };
                    s.syn_timer = Some(t + wait * US);
                    self.setup = Some(s);
                    self.transmit_ctl(1, Ctl::Syn, t);
                }
            }
            Event::SynAckTimer => {
                if let Some(mut s) = self.setup {
                    s.synack_tries += 1;
                    s.synack_timeouts += 1;
                    s.synack_timer = Some(t + (RTO_INIT_US << s.synack_timeouts.min(7)) * US);
                    self.setup = Some(s);
                    self.transmit_ctl(0, Ctl::SynAck, t);
                }
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
        self.transmit_any(end, data, Ctl::None, attempt, now)
    }

    /// Every transmission draws its four words here, whatever it carries (S10).
    fn transmit_any(
        &mut self,
        end: usize,
        data: Option<(u64, u64)>,
        ctl: Ctl,
        attempt: u32,
        now: u64,
    ) -> (u64, bool) {
        let ack = self.ends[end].rcv.ack();
        let seq = data.map_or(self.ends[end].snd.snd_nxt, |(s, _)| s);
        let rcv = &mut self.ends[end].rcv;
        rcv.ack_now = false;
        rcv.delack_at = None;
        rcv.unacked_segments = 0;
        rcv.rcv_wup = rcv.rcv_nxt;
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
        } else if ctl == Ctl::None {
            c.pure_acks += 1;
            c.pure_acks_lost += u64::from(lost);
        } else {
            c.handshake_packets += 1;
            c.handshake_lost += u64::from(lost);
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
                seq,
                data,
                ack,
                ctl,
                tsval: ts_of(now),
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
        let end = (start + self.mss).min(s.write_seq);
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

    /// The acknowledgement in an arrival at `e`, for the data `e` sent, in tcp_ack's order: the SACK scoreboard
    /// (tcp_sacktag_write_queue), then the segments the cumulative point passes (tcp_clean_rtx_queue), then the RTT
    /// sample (tcp_ack_update_rtt).
    fn on_ack(&mut self, e: usize, ack: &Ack, now: u64) {
        let tsecr = ack.tsecr.filter(|_| self.timestamps);
        let s = &mut self.ends[e].snd;
        let prior_una = s.snd_una;
        // (last transmission, ever resent, end) of each newly SACKed segment above the new cumulative point, in order.
        let mut sacked_new: Vec<(u64, bool, u64)> = Vec::new();
        for g in s.segs.iter_mut().filter(|g| !g.sacked && g.end > ack.cum) {
            if ack.sacks.iter().any(|&(a, b)| a <= g.start && g.end <= b) {
                g.sacked = true;
                g.outstanding = false;
                g.lost = false;
                sacked_new.push((g.xmit_ns, g.retrans, g.end));
            }
        }
        // The same for the segments delivered in order and not SACKed before; any of them ever resent rules out the
        // sequence sample (FLAG_RETRANS_DATA_ACKED), which comes from the oldest never-resent one (first_ackt).
        let mut cum_new: Vec<(u64, bool, u64)> = Vec::new();
        let (mut any_acked, mut retrans_acked) = (false, false);
        let (mut first_ackt, mut last_ackt) = (None, None);
        while s.segs.front().is_some_and(|g| g.end <= ack.cum) {
            let Some(g) = s.segs.pop_front() else {
                break;
            };
            any_acked = true;
            if g.retrans {
                retrans_acked = true;
            } else if !g.sacked {
                first_ackt.get_or_insert(g.xmit_ns);
                last_ackt = Some(g.xmit_ns);
            }
            if !g.sacked {
                cum_new.push((g.xmit_ns, g.retrans, g.end));
            }
        }
        if ack.cum > s.snd_una {
            s.snd_una = ack.cum.min(s.snd_nxt);
            s.retransmits = 0;
        }
        if sacked_new.is_empty() && cum_new.is_empty() && s.snd_una == prior_una {
            return;
        }
        // RACK (tcp_rack_advance), with the minimum RTT as it was before this sample: SACKed segments first, then those
        // delivered in order. A resent segment counts unless its RTT is below the minimum, or (in order, with
        // timestamps) the echo predates its last transmission (tcp_skb_spurious_retrans). Every advance sets RACK's
        // RTT; only a later send moves its reference.
        let min_rtt = s.rtt.min_rtt;
        let sacked_path = sacked_new.iter().map(|&d| (d, false));
        let cum_path = cum_new.iter().map(|&d| (d, true));
        for ((x, retrans, end), in_order) in sacked_path.chain(cum_path) {
            if in_order && retrans && tsecr.is_some_and(|ts| ts < ts_of(x)) {
                continue;
            }
            let rtt_us = (now - x) / US;
            if retrans && min_rtt.is_some_and(|m| rtt_us < m) {
                continue;
            }
            s.rack.rtt_us = rtt_us;
            if !s.rack.set || sent_after(x, s.rack.mstamp_ns, end, s.rack.end_seq) {
                s.rack.mstamp_ns = x;
                s.rack.end_seq = end;
            }
            s.rack.set = true;
        }
        // tcp_ack_update_rtt: the sequence sample, else the first never-resent SACKed segment's, else the echo when
        // data was delivered in order; the minimum RTT from the newest (last_ackt, last_sackt, or the echo).
        let us = |x: u64| i64::try_from((now - x) / US).unwrap_or(i64::MAX);
        let (mut seq_rtt, mut ca_rtt) = (None, None);
        if let (Some(f), Some(l)) = (first_ackt, last_ackt)
            && !retrans_acked
        {
            seq_rtt = Some(us(f));
            ca_rtt = Some(us(l));
        }
        let mut fresh_sacks = sacked_new.iter().filter(|d| !d.1).map(|d| d.0);
        let sack_rtt = fresh_sacks.next().map(us);
        if let Some(l) = sacked_new.iter().rev().find(|d| !d.1).map(|d| d.0) {
            ca_rtt = Some(us(l));
        }
        let mut rtt = seq_rtt.or(sack_rtt);
        if rtt.is_none()
            && any_acked
            && let Some(ts) = tsecr
        {
            let m = tsopt_rtt_us(now, ts);
            rtt = Some(m);
            ca_rtt = Some(m);
        }
        if let (Some(m), Some(ca)) = (rtt, ca_rtt) {
            s.rtt.sample(m, ca, s.snd_una, s.snd_nxt, s.rto_min_us);
            s.backoff = 0;
        }
        let delivered = sacked_new.len() + cum_new.len();
        if s.tlp_high_seq.is_some_and(|h| ack.cum > h) {
            s.tlp_high_seq = None;
        }
        if s.state != State::Open && s.snd_una >= s.high_seq {
            s.state = State::Open;
        }
        // Slow start below ssthresh while cwnd-limited (tcp_cong_avoid's guard); growth above it is not modelled (S2).
        if s.cwnd < s.ssthresh && s.cwnd_limited() {
            s.cwnd = (s.cwnd + delivered as u64).min(s.ssthresh);
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
        let rack_on = self.rack_on();
        let s = &mut self.ends[e].snd;
        if s.segs.is_empty() {
            return;
        }
        // tcp_enter_loss recomputes ssthresh on the episode's first timeout only (icsk_retransmits == 0 in Loss).
        if s.state == State::Open || (s.state == State::Loss && s.retransmits == 0) {
            s.cut_ssthresh();
        }
        // tcp_timeout_mark_lost: every segment but those sent within RACK's RTT (the head always), then cwnd = what is
        // still in flight + 1 (tcp_input.c:2160-2168, :2194).
        let rack_rtt_us = if s.rack.set && rack_on {
            s.rack.rtt_us
        } else {
            0
        };
        for (i, g) in s.segs.iter_mut().enumerate().filter(|(_, g)| !g.sacked) {
            if i > 0 && rack_rtt_us > (now - g.xmit_ns) / US {
                continue;
            }
            g.lost = true;
            g.outstanding = false;
            g.cause = Some(Repair::Timeout);
        }
        s.cwnd = s.in_flight() + 1;
        s.state = State::Loss;
        s.high_seq = s.snd_nxt;
        s.tlp_high_seq = None;
        s.retransmits += 1;
        // tcp_retransmit_timer's out_reset_timer: a thin stream (fewer than 4 segments out, past its initial slow
        // start) keeps a linear timeout for its first 6 retries when the setting is on; else the RTO doubles.
        let thin = s.thin_lt
            && s.segs.len() < THIN_PACKETS
            && s.ssthresh != u64::MAX
            && s.retransmits <= THIN_LINEAR_RETRIES;
        if thin {
            s.backoff = 0;
        } else if doubling {
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
    SynTimer,
    SynAckTimer,
}

#[cfg(test)]
mod fixes;
#[cfg(test)]
mod tests;
