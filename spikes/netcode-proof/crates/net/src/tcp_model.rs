//! The Linux-faithful TCP model behind DelayStream (unit U3a; the advisor's spec S0-S14 in
//! `.checkpoints/netcode-proof-test/advisor-u3q.md`; Mustafa's U3-Q3 (b), 2026-11-06: "model Linux TCP faithfully, so
//! the result shows how the fallback really behaves").
//!
//! One WebSocket connection, both directions, as pure logic on nanosecond stamps: no I/O and no clock. DelayStream
//! drives it from the release clock (U3c); the tests drive it on simulated time. End 0 is RS (it sends down), end 1
//! the client (it sends up). It models Linux 6.8 with its default sysctls (tcp_sack 1, tcp_dsack 1, tcp_frto 2,
//! tcp_recovery 1 (RACK), tcp_early_retrans 3, tcp_reordering 3, Cubic). Every constant and rule was read in v6.8:
//! include/net/tcp.h, net/ipv4/tcp_input.c, tcp_output.c, tcp_recovery.c, tcp_timer.c, tcp_cong.c, tcp_cubic.c,
//! tcp_minisocks.c and tcp_metrics.c; each rule below cites its function and lines in the code.
//! - S1: segments of at most 1,448 B (1,460 B without the timestamp option), Nagle off; each transmission's delay is
//!   the profile's half latency plus a uniform jitter of ± half its jitter, and its loss is drawn at the profile's
//!   rate; any transmission sent inside a burst is lost. Each direction's path is first in, first out, so the stream
//!   never reorders, and no two packets arrive at the same instant (1 ns apart at least: the kernel handles each
//!   arrival at its own clock reading, so RACK never compares sends of two arrivals at one stamp).
//! - S2: at most cwnd segments in flight (packets out, less SACKed and lost, plus resends in flight), 10 at first.
//!   Cubic grows cwnd only while the flow is cwnd-limited (tcp_is_cwnd_limited), from ACKs that deliver data in order:
//!   slow start below ssthresh (with HyStart at 16 segments and above), Cubic's curve above it, in the kernel's fixed
//!   point. Entering Recovery sets ssthresh = max(0.7 cwnd, 2) and runs PRR (RFC 6937) until it ends; leaving it sets
//!   cwnd = ssthresh unless the recovery was undone, so an app-limited flow does shrink to 0.7 of its window at each
//!   repaired loss (the first version said it did not). After a timeout cwnd = segments still in flight + 1, then slow
//!   start. A resent loss probe that repaired a loss cuts cwnd to ssthresh at once (CWR, tcp_process_tlp_ack).
//! - S3: the receiver acknowledges at once an out-of-order segment, one that fills a hole, and a duplicate; in-order
//!   data at the earliest of 40 ms after it arrived, a second unacknowledged segment, or the next segment its own end
//!   sends (the acknowledgement rides on it). SACK blocks as Linux keeps them (most recent first, 4 remembered, 3 in
//!   the option with timestamps, 4 without), and a D-SACK block first, once, for a duplicate (tcp_dsack 1). A segment
//!   whose end is below rcv_wup (an old duplicate) is discarded whole, its acknowledgement field unread, and answered
//!   with a duplicate ACK and a D-SACK (tcp_validate_incoming); a duplicate inside the window is processed.
//! - S4: Linux's RTT estimator in integer microseconds; RTO = srtt + rttvar, rttvar never below 200 ms; 1 s before the
//!   first sample (3 s once a handshake ended without one: tcp_init_metrics). One sample per acknowledgement, taken as
//!   tcp_clean_rtx_queue and tcp_ack_update_rtt take it: from the oldest segment it newly delivers in order, none if
//!   any segment it delivers in order was ever resent (Karn); else from the first segment it newly SACKs that was never
//!   resent; else, with timestamps, from the echoed timestamp (TSecr, whole milliseconds) when it delivers data in
//!   order. The minimum RTT comes from the newest such segment. A valid sample resets the RTO's backoff.
//! - S5: RACK (RFC 8985), run on every doubtful acknowledgement (tcp_fastretrans_alert) and by its reorder timer: a
//!   segment is lost once one sent after it is delivered and its own send plus RACK's RTT plus the reordering window
//!   has passed; else the reorder timer fires then, plus 2 ms. The window is min(min_rtt / 4, srtt), or 0 in Recovery
//!   and Loss, or once as many segments are SACKed as tp->reordering (3), unless reordering was seen
//!   (tcp_rack_reo_wnd).
//! - S6: the tail-loss probe, in the Open (or CWR) state only, with data out: 2 x srtt plus 200 ms with one segment
//!   out (else 2 ms), at most the RTO's remaining time; it sends one new segment if there is one, past cwnd, else
//!   resends the last; one per episode; then the RTO restarts. The Disorder state (SACKed, lost or resent segments
//!   out) arms no probe.
//! - S7: the RTO, at the head's last transmission plus RTO: every outstanding segment lost except those sent within
//!   RACK's RTT (never the head: tcp_timeout_mark_lost), cwnd = segments still in flight + 1, ssthresh cut on the
//!   episode's first timeout only (tcp_enter_loss), the head resent, the RTO doubled up to 120 s. F-RTO (tcp_frto 2):
//!   on the next ACK that advances, new data goes first if there is any; a later ACK that delivers a segment never
//!   resent (sent before the timeout) proves the timeout spurious and undoes it. The undo (Eifel, RFC 3522): an ACK
//!   whose echoed timestamp predates the first resend, or D-SACKs for every resend, restores cwnd and ssthresh and
//!   forgets the loss marks (tcp_try_undo_loss, _recovery, _partial, _dsack); a resend an undo leaves marked stays
//!   counted in flight, and an RTO that resends it again counts it twice until nothing is out, as the kernel's
//!   retrans_out does ("retrans_out leaked").
//! - S8: every resend draws its own fate. S9: bytes are released in order; the write side holds at most 64 KB unsent
//!   or unacknowledged.
//! - S10: fates from one ChaCha8 stream per (seed, connection, direction), four words per transmission.
//! - The handshake (for reconnects): the SYN retried at a linear 1 s for 4 tries, then doubling (TcpConfig sets
//!   another client's: the first timeout and its linear tries); the SYN-ACK retried from 1 s, doubling per timer
//!   retry, and resent at once on a repeated SYN without moving that schedule (tcp_check_req), echoing the first SYN's
//!   timestamp; each end primed by the handshake's RTT, by the timestamp echo if it resent; cwnd 1 after more than one
//!   handshake resend (tcp_init_transfer) unless the echo shows the first SYN (or SYN-ACK) arrived
//!   (tcp_try_undo_spurious_syn).
//! - Not modelled (S11), recorded in each WebSocket run's header: RACK's reordering window growth from D-SACKs
//!   (reo_wnd_steps, dsack_seen: the window stays at one step), the delayed-ACK timer above 40 ms, quick-ack mode, SACK
//!   compression (at most 1 ms, after 3 duplicate ACKs), RFC 2861's cwnd decay when app-limited and the restart after
//!   idle (both act only above the initial window), the receive window and the persist timer (the 64 KB write budget
//!   stays inside it), SACK reneging, TSO and GSO, ECN, the per-destination metrics cache, the rate limit on answers
//!   to repeated SYNs and to old data-less packets, the delayed-ACK filter of the minimum RTT, jiffy rounding of the
//!   timers (Cubic's own clock runs at HZ = 1,000: RS's CONFIG_HZ is recorded on the run day), the merging of small
//!   segments when they are resent (tcp_retrans_try_collapse; only the client's inputs are that small), and a
//!   D-SACK counted twice against undo_retrans in one sacktag ordering (the model counts it once: slightly slower).

use npt_l1::Profile;
use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};
use std::collections::{BTreeMap, VecDeque};

/// The model's name and version, for each WebSocket run's header (S11); DelayStream's header writes it.
pub(crate) const MODEL: &str = "linux-6.8-tcp v3";
/// What the model leaves out (S11; the module documentation's last item), for the same header.
pub(crate) const NOT_MODELLED: &str = "RACK's reordering window growth from D-SACKs; the delayed-ACK timer above \
     40 ms; quick-ack mode; SACK compression; RFC 2861's cwnd decay when app-limited and the restart after idle; the \
     receive window and the persist timer; SACK reneging; TSO and GSO; ECN; the per-destination metrics cache; the \
     rate limit on answers to repeated SYNs and old data-less packets; the delayed-ACK filter of the minimum RTT; \
     jiffy rounding of the timers (Cubic's clock at HZ 1,000); the merging of small segments when resent; a D-SACK \
     counted twice against undo_retrans in one sacktag ordering";
/// 1,500 B MTU less IPv4 (20 B), TCP (20 B) and the timestamp option (12 B).
pub(crate) const MSS: u64 = 1_448;
/// The same without the timestamp option (a peer that does not offer it, as Windows does by default).
const MSS_NO_TS: u64 = 1_460;
/// The write side's bound (ADR 0001:1467, the Profiles key of part 13): unsent plus unacknowledged bytes.
pub(crate) const WRITE_BUDGET: u64 = 64 * 1024;
/// The most bursts a connection holds that have not yet ended, the profile's and those added later (`add_burst`).
pub(crate) const MAX_BURSTS: usize = 64;
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
/// TCP_INFINITE_SSTHRESH (include/net/tcp.h:1258).
const INFINITE_SSTHRESH: u64 = 0x7fff_ffff;
/// tcp_reordering's and tcp_max_reordering's defaults (Documentation/networking/ip-sysctl.rst:655-666).
const REORDERING: u64 = 3;
const MAX_REORDERING: u64 = 300;
/// tcp_stream_is_thin: fewer than this many segments out (include/net/tcp.h:2119).
const THIN_PACKETS: usize = 4;
/// TCP_THIN_LINEAR_RETRIES (include/net/tcp.h:238): linear timeouts for this many retries, then doubling.
const THIN_LINEAR_RETRIES: u32 = 6;
/// tcp_syn_linear_timeouts' default (net/ipv4/tcp_ipv4.c:3489): this many SYN retries at a linear 1 s.
const SYN_LINEAR_TIMEOUTS: u32 = 4;
/// TCP_NUM_SACKS: the SACK blocks a receiver remembers (selective_acks[4]).
const NUM_SACKS: usize = 4;
const WORDS: usize = 4;
const KEY_TAG: &[u8; 24] = b"npt-net tcp model, v1\0\0\0";
const US: u64 = 1_000;

/// Cubic's clock: CONFIG_HZ (assumed 1,000; RS's kernel config is read at R1). Only Cubic reads jiffies here.
const HZ: u64 = 1_000;
const JIFFY_NS: u64 = 1_000_000_000 / HZ;
/// tcp_cubic.c:34-58 and cubictcp_register (:516-538): beta 717/1024, bic_scale 41, fast convergence and TCP
/// friendliness on, HyStart on with both detectors, low window 16, ACK-train spacing 2 ms.
const BICTCP_BETA_SCALE: u64 = 1_024;
const CUBIC_BETA: u64 = 717;
const BICTCP_HZ: u32 = 10;
const BIC_SCALE: u64 = 41;
const CUBIC_BETA_SCALE: u64 =
    8 * (BICTCP_BETA_SCALE + CUBIC_BETA) / 3 / (BICTCP_BETA_SCALE - CUBIC_BETA);
const CUBE_RTT_SCALE: u64 = BIC_SCALE * 10;
const CUBE_FACTOR: u64 = (1u64 << (10 + 3 * BICTCP_HZ)) / (BIC_SCALE * 10);
const HYSTART_LOW_WINDOW: u64 = 16;
const HYSTART_ACK_DELTA_US: u64 = 2_000;
const HYSTART_MIN_SAMPLES: u32 = 8;
const HYSTART_DELAY_MIN_US: u64 = 4_000;
const HYSTART_DELAY_MAX_US: u64 = 16_000;
/// GSO_LEGACY_MAX_SIZE: sk_gso_max_size behind hystart_ack_delay on a NIC without BIG TCP.
const GSO_MAX_SIZE: u64 = 65_536;

/// tcp_ack's flags (tcp_input.c:86-108).
const FLAG_DATA: u32 = 0x01;
const FLAG_WIN_UPDATE: u32 = 0x02;
const FLAG_DATA_ACKED: u32 = 0x04;
const FLAG_RETRANS_DATA_ACKED: u32 = 0x08;
const FLAG_DATA_SACKED: u32 = 0x20;
const FLAG_LOST_RETRANS: u32 = 0x80;
const FLAG_ORIG_SACK_ACKED: u32 = 0x200;
const FLAG_SND_UNA_ADVANCED: u32 = 0x400;
const FLAG_DSACKING_ACK: u32 = 0x800;
const FLAG_SET_XMIT_TIMER: u32 = 0x1000;
const FLAG_DSACK_TLP: u32 = 0x20000;
const FLAG_ACKED: u32 = FLAG_DATA_ACKED;
const FLAG_NOT_DUP: u32 = FLAG_DATA | FLAG_WIN_UPDATE | FLAG_ACKED;
const FLAG_CA_ALERT: u32 = FLAG_DATA_SACKED | FLAG_DSACKING_ACK;
const FLAG_FORWARD_PROGRESS: u32 = FLAG_ACKED | FLAG_DATA_SACKED;

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
    /// This direction's old duplicates the receiver discarded unread (tcp_validate_incoming).
    pub discards_old: u64,
    /// D-SACK blocks the sender accepted (tcp_dsack_seen).
    pub dsacks: u64,
    /// Undone losses: in the Loss state by the timestamps or D-SACKs (tcp_try_undo_loss), by F-RTO (its step 3.b),
    /// and in Recovery or Disorder (tcp_try_undo_recovery, _partial, _dsack).
    pub undos_loss: u64,
    pub undos_frto: u64,
    pub undos_recovery: u64,
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
    /// The client's first SYN timeout (Linux: TCP_TIMEOUT_INIT, 1 s; a Windows client: 3 s, or 1 s with some
    /// settings) and how many SYN retries keep it before it doubles (Linux: tcp_syn_linear_timeouts, 4; Windows: 0).
    pub syn_rto_us: u64,
    pub syn_linear_timeouts: u32,
}

impl Default for TcpConfig {
    /// Linux 6.8's defaults and the plan's 64 KB write bound.
    fn default() -> TcpConfig {
        TcpConfig {
            write_budget: WRITE_BUDGET,
            thin_linear_timeouts: false,
            rto_min_us: RTO_MIN_US.unsigned_abs(),
            timestamps: true,
            syn_rto_us: RTO_INIT_US,
            syn_linear_timeouts: SYN_LINEAR_TIMEOUTS,
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

/// The planted defects of U3's own tests (S13's bites, and one per rule); never in a build that runs.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variant {
    Faithful,
    /// E15's old rule in place of RACK and the probe: every repair one fixed timeout, max(200 ms, 2 x RTT).
    ConstantRepair,
    /// The RTO never backs off.
    NoDoubling,
    /// Karn's rule off: the oldest segment delivered in order gives a sample even when one beside it was resent.
    NoKarn,
    /// No sample from the echoed timestamp.
    NoTsecrFallback,
    /// ts_recent taken from every arrival, not only those at or below rcv_wup.
    TsRecentAlways,
    /// The RTO marks every segment lost, the recent ones included.
    RtoMarksAll,
    /// The RTO sets cwnd to 1, not to what is in flight plus 1.
    RtoCwndOne,
    /// Old duplicates processed like any segment (their ACK field read).
    OldSeqProcessed,
    /// The receiver never sends a D-SACK.
    NoDsack,
    /// No undo of any kind.
    NoUndo,
    /// F-RTO never armed.
    NoFrto,
    /// Leaving Recovery keeps cwnd (no tcp_end_cwnd_reduction).
    NoExitCwnd,
    /// No growth above ssthresh (the first version's "not modelled").
    NoCubic,
    /// The probe armed in the Disorder state too.
    ProbeInDisorder,
    /// No tcp_try_undo_spurious_syn.
    NoSpuriousSyn,
}

/// The congestion-avoidance states (icsk_ca_state), in the kernel's order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Open,
    Disorder,
    Cwr,
    Recovery,
    Loss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimerKind {
    Rto,
    Probe,
    Reorder,
}

/// What tcp_ack asks tcp_xmit_recovery to send.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rexmit {
    None,
    Lost,
    New,
}

#[derive(Debug, Clone)]
struct Seg {
    start: u64,
    end: u64,
    /// The last transmission's time.
    xmit_ns: u64,
    /// The first transmission's arrival as drawn, lost or not: the stall reference.
    first_arrival_ns: u64,
    /// TCPCB_SACKED_ACKED.
    sacked: bool,
    /// TCPCB_LOST.
    lost: bool,
    /// TCPCB_SACKED_RETRANS: a resend of it is in flight.
    sretrans: bool,
    /// TCPCB_EVER_RETRANS: resent at least once, the probe's resend included (Karn's rule).
    ever: bool,
    /// On tsorted_sent_queue, the list RACK scans: off once SACKed or marked lost by RACK, back at each send.
    tsorted: bool,
    attempts: u32,
    cause: Option<Repair>,
    received: bool,
}

impl Seg {
    /// TCPCB_RETRANS.
    fn retrans(&self) -> bool {
        self.sretrans || self.ever
    }
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

    /// tcp_min_rtt: ~0U before the first sample.
    fn min_rtt_us(&self) -> u64 {
        self.min_rtt.unwrap_or(u64::from(u32::MAX))
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

/// tp->rack (include/linux/tcp.h): the reference send time and sequence, RACK's RTT, and whether an ACK advanced it.
#[derive(Debug, Clone, Copy, Default)]
struct Rack {
    set: bool,
    rtt_us: u64,
    mstamp_ns: u64,
    end_seq: u64,
    advanced: bool,
}

/// RACK's "sent after": a later send, or the same send of a later sequence (tcp_skb_sent_after).
fn sent_after(t1: u64, t2: u64, seq1: u64, seq2: u64) -> bool {
    t1 > t2 || (t1 == t2 && seq1 > seq2)
}

/// struct bictcp (tcp_cubic.c:86-105); epoch_start's 0 ("no epoch") is `None` here.
#[derive(Debug, Clone, Copy, Default)]
struct Cubic {
    cnt: u64,
    last_max_cwnd: u64,
    last_cwnd: u64,
    last_time: u64,
    bic_origin_point: u64,
    bic_k: u64,
    delay_min: u64,
    epoch_start: Option<u64>,
    ack_cnt: u64,
    tcp_cwnd: u64,
    sample_cnt: u32,
    found: bool,
    round_start: u64,
    end_seq: u64,
    last_ack: u64,
    curr_rtt: u64,
}

/// The kernel's jiffies at `ns`.
fn jiffies(ns: u64) -> u64 {
    ns / JIFFY_NS
}

/// usecs_to_jiffies: rounded up.
fn usecs_to_jiffies(us: u64) -> u64 {
    us.div_ceil(1_000_000 / HZ)
}

/// cubic_root (tcp_cubic.c:167-209): a table lookup and one Newton-Raphson step, in u32 as the kernel computes it.
fn cubic_root(a: u64) -> u64 {
    const V: [u8; 64] = [
        0, 54, 54, 54, 118, 118, 118, 118, 123, 129, 134, 138, 143, 147, 151, 156, 157, 161, 164,
        168, 170, 173, 176, 179, 181, 185, 187, 190, 192, 194, 197, 199, 200, 202, 204, 206, 209,
        211, 213, 215, 217, 219, 221, 222, 224, 225, 227, 229, 231, 232, 234, 236, 237, 239, 240,
        242, 244, 245, 246, 248, 250, 251, 252, 254,
    ];
    let b = 64 - a.leading_zeros();
    if b < 7 {
        return (u64::from(V[usize::try_from(a).unwrap_or(0)]) + 35) >> 6;
    }
    let b = ((b * 84) >> 8) - 1;
    let shift = usize::try_from(a >> (b * 3)).unwrap_or(63).min(63);
    let mut x = u64::from((u32::from(V[shift]) + 10) << b) >> 6;
    x = (2 * x + a / (x * (x - 1))) & u64::from(u32::MAX);
    (x * 341) >> 10
}

/// The SACK scoreboard walk's running state (struct tcp_sacktag_state).
#[derive(Debug, Clone, Copy)]
struct SackState {
    reord: u64,
    first_sackt: Option<u64>,
    last_sackt: Option<u64>,
    sack_delivered: u64,
}

#[derive(Debug, Clone)]
struct Sender {
    mss: u64,
    write_seq: u64,
    snd_una: u64,
    snd_nxt: u64,
    segs: VecDeque<Seg>,
    cwnd: u64,
    ssthresh: u64,
    /// snd_cwnd_cnt: ACKs counted toward the next additive step.
    cwnd_cnt: u64,
    prior_cwnd: u64,
    /// 0 when there is none (the kernel's convention).
    prior_ssthresh: u64,
    rtt: Rtt,
    backoff: u32,
    rack: Rack,
    state: State,
    high_seq: u64,
    tlp_high_seq: Option<u64>,
    /// The probe resent a segment (not new data).
    tlp_retrans: bool,
    timer: Option<(u64, TimerKind)>,
    /// tcp_cwnd_validate's window: whether cwnd held a send back, the most packets out, and the sequence that ends
    /// the window (tcp_is_cwnd_limited reads them).
    is_cwnd_limited: bool,
    max_packets_out: u64,
    cwnd_usage_seq: u64,
    /// The settings that differ from Linux's defaults when the fixes are on (TcpConfig).
    rto_min_us: i64,
    thin_lt: bool,
    /// icsk_retransmits: retransmission timeouts in this episode, reset when snd_una advances (tcp_input.c:3888).
    retransmits: u32,
    /// The undo state: undo_marker (snd_una when the reduction began), undo_retrans (resends not yet D-SACKed; -1
    /// for none), retrans_stamp (the first resend's timestamp, in the option's milliseconds), F-RTO's flag.
    undo_marker: Option<u64>,
    undo_retrans: i64,
    retrans_stamp: Option<u64>,
    frto: bool,
    total_retrans: u64,
    dsack_dups: u64,
    /// tp->reordering and tp->reord_seen (tcp_check_sack_reordering).
    reordering: u64,
    reord_seen: u64,
    /// PRR's counts (tcp_cwnd_reduction).
    prr_delivered: u64,
    prr_out: u64,
    /// tp->lost: segments marked lost, ever (an ACK's newly lost are its difference).
    lost: u64,
    /// retrans_out beyond the segments marked as resends in flight: tcp_retransmit_skb adds one even when the segment
    /// already carries TCPCB_SACKED_RETRANS (tcp_output.c:3413-3420 only logs "retrans_out leaked"), which happens when
    /// an RTO resends a head an undo left marked; FASTRETRANS_DEBUG's check zeroes it once nothing is out
    /// (tcp_input.c:3459-3478).
    retrans_leak: u64,
    /// The sequence of the last segment that updated the send window (snd_wl1).
    snd_wl1: Option<u64>,
    /// The echoed timestamp of the segment being processed (rx_opt.rcv_tsecr; `None` without the option).
    rcv_tsecr: Option<u64>,
    cubic: Cubic,
    /// lsndtime: the last data send, for Cubic's CA_EVENT_TX_START.
    lsndtime_ns: u64,
    /// sk_pacing_rate as tcp_update_pacing_rate leaves it (bytes per second), for HyStart's ACK delay.
    pacing_rate: u64,
    undos_loss: u64,
    undos_frto: u64,
    undos_recovery: u64,
    dsacks: u64,
    /// The constant-repair bite's fixed timeout (tests only).
    #[cfg(test)]
    fixed_rto_ns: Option<u64>,
    #[cfg(test)]
    variant: Variant,
}

impl Sender {
    fn new(cfg: &TcpConfig) -> Sender {
        let mut s = Sender {
            mss: if cfg.timestamps { MSS } else { MSS_NO_TS },
            rto_min_us: i64::try_from(cfg.rto_min_us).unwrap_or(RTO_MIN_US),
            thin_lt: cfg.thin_linear_timeouts,
            retransmits: 0,
            write_seq: 0,
            snd_una: 0,
            snd_nxt: 0,
            segs: VecDeque::new(),
            cwnd: INIT_CWND,
            ssthresh: INFINITE_SSTHRESH,
            cwnd_cnt: 0,
            prior_cwnd: 0,
            prior_ssthresh: 0,
            rtt: Rtt::default(),
            backoff: 0,
            rack: Rack::default(),
            state: State::Open,
            high_seq: 0,
            tlp_high_seq: None,
            tlp_retrans: false,
            timer: None,
            is_cwnd_limited: false,
            max_packets_out: 0,
            cwnd_usage_seq: 0,
            undo_marker: None,
            undo_retrans: 0,
            retrans_stamp: None,
            frto: false,
            total_retrans: 0,
            dsack_dups: 0,
            reordering: REORDERING,
            reord_seen: 0,
            prr_delivered: 0,
            prr_out: 0,
            lost: 0,
            retrans_leak: 0,
            snd_wl1: None,
            rcv_tsecr: None,
            cubic: Cubic::default(),
            lsndtime_ns: 0,
            pacing_rate: u64::MAX,
            undos_loss: 0,
            undos_frto: 0,
            undos_recovery: 0,
            dsacks: 0,
            #[cfg(test)]
            fixed_rto_ns: None,
            #[cfg(test)]
            variant: Variant::Faithful,
        };
        s.cubic_init(0);
        s
    }

    #[cfg(test)]
    fn bitten(&self, v: Variant) -> bool {
        self.variant == v
    }
    #[cfg(test)]
    fn rack_on(&self) -> bool {
        self.variant != Variant::ConstantRepair
    }
    #[cfg(not(test))]
    fn rack_on(&self) -> bool {
        true
    }

    fn packets_out(&self) -> u64 {
        self.segs.len() as u64
    }

    fn sacked_out(&self) -> u64 {
        self.segs.iter().filter(|g| g.sacked).count() as u64
    }

    fn lost_out(&self) -> u64 {
        self.segs.iter().filter(|g| g.lost).count() as u64
    }

    fn retrans_out(&self) -> u64 {
        self.segs.iter().filter(|g| g.sretrans).count() as u64 + self.retrans_leak
    }

    /// tcp_packets_in_flight (include/net/tcp.h:1253-1256): packets out, less SACKed and lost, plus resends in flight.
    fn in_flight(&self) -> u64 {
        let n: i64 = self
            .segs
            .iter()
            .map(|g| 1 - i64::from(g.sacked) - i64::from(g.lost) + i64::from(g.sretrans))
            .sum::<i64>()
            + self.retrans_leak as i64;
        u64::try_from(n).unwrap_or(0)
    }

    /// tcp_highest_sack_seq (include/net/tcp.h:2069-2078): the end of the highest SACKed segment, or snd_una.
    fn highest_sack_seq(&self) -> u64 {
        self.segs
            .iter()
            .filter(|g| g.sacked)
            .map(|g| g.end)
            .max()
            .unwrap_or(self.snd_una)
    }

    fn in_cwnd_reduction(&self) -> bool {
        matches!(self.state, State::Cwr | State::Recovery)
    }

    /// tcp_is_cwnd_limited (include/net/tcp.h:1336-1348).
    fn cwnd_limited(&self) -> bool {
        self.is_cwnd_limited || (self.cwnd < self.ssthresh && self.cwnd < 2 * self.max_packets_out)
    }

    /// tcp_cwnd_validate's first half (tcp_output.c:1905-1922). Its second half, RFC 2861's decay of an unused
    /// window, is not modelled (S11).
    fn cwnd_validate(&mut self, is_cwnd_limited: bool) {
        let packets = self.packets_out();
        if self.snd_una >= self.cwnd_usage_seq
            || is_cwnd_limited
            || (!self.is_cwnd_limited && packets > self.max_packets_out)
        {
            self.is_cwnd_limited = is_cwnd_limited;
            self.max_packets_out = packets;
            self.cwnd_usage_seq = self.snd_nxt;
        }
    }

    fn rto_ns(&self) -> u64 {
        #[cfg(test)]
        if let Some(f) = self.fixed_rto_ns {
            return f;
        }
        // Each doubling is capped at TCP_RTO_MAX (tcp_timer.c:655), so the product never wraps: min(base << backoff,
        // RTO_MAX) without forming an overflowing shift (the third skeptic round's latent finding).
        let base = self.rtt.rto_us();
        let backed = if self.backoff >= 64 || base > (RTO_MAX_US >> self.backoff) {
            RTO_MAX_US
        } else {
            base << self.backoff
        };
        backed.min(RTO_MAX_US) * US
    }

    /// tcp_rearm_rto (tcp_input.c:3200-3226).
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

    /// tcp_schedule_loss_probe (tcp_output.c:2829-2876): only in Open or CWR; false when no probe may be armed.
    fn schedule_probe(&mut self, now: u64, advancing_rto: bool) -> bool {
        #[allow(unused_mut)]
        let mut allowed = matches!(self.state, State::Open | State::Cwr);
        #[cfg(test)]
        if self.bitten(Variant::ProbeInDisorder) && self.state == State::Disorder {
            allowed = true;
        }
        if self.segs.is_empty() || !allowed {
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

    // ---- Cubic (tcp_cubic.c) ----

    /// cubictcp_init (:129-140): bictcp_reset, then bictcp_hystart_reset.
    fn cubic_init(&mut self, now: u64) {
        self.cubic = Cubic::default();
        self.hystart_reset(now);
    }

    /// bictcp_reset (:107-111): every field before `unused`, and `found`.
    fn bictcp_reset(&mut self) {
        let c = self.cubic;
        self.cubic = Cubic {
            sample_cnt: c.sample_cnt,
            round_start: c.round_start,
            end_seq: c.end_seq,
            last_ack: c.last_ack,
            curr_rtt: c.curr_rtt,
            ..Cubic::default()
        };
    }

    /// bictcp_hystart_reset (:118-127).
    fn hystart_reset(&mut self, now: u64) {
        let c = &mut self.cubic;
        c.round_start = now / US;
        c.last_ack = now / US;
        c.end_seq = self.snd_nxt;
        c.curr_rtt = u64::from(u32::MAX);
        c.sample_cnt = 0;
    }

    /// cubictcp_recalc_ssthresh (:341-356), with fast convergence.
    fn cubic_ssthresh(&mut self) -> u64 {
        let cwnd = self.cwnd;
        let c = &mut self.cubic;
        c.epoch_start = None;
        c.last_max_cwnd = if cwnd < c.last_max_cwnd {
            (cwnd * (BICTCP_BETA_SCALE + CUBIC_BETA)) / (2 * BICTCP_BETA_SCALE)
        } else {
            cwnd
        };
        ((cwnd * CUBIC_BETA) / BICTCP_BETA_SCALE).max(2)
    }

    /// cubictcp_cwnd_event's CA_EVENT_TX_START (:142-161): after an idle spell, the epoch moves by its length.
    fn cubic_tx_start(&mut self, now: u64) {
        let now_j = jiffies(now);
        let delta = now_j.saturating_sub(jiffies(self.lsndtime_ns));
        if let Some(e) = self.cubic.epoch_start
            && delta > 0
        {
            self.cubic.epoch_start = Some((e + delta).min(now_j));
        }
    }

    /// bictcp_update (:214-322), in the kernel's fixed point and u32 wrap-around.
    fn bictcp_update(&mut self, cwnd: u64, acked: u64, now: u64) {
        let now_j = jiffies(now);
        let c = &mut self.cubic;
        c.ack_cnt += acked;
        if c.last_cwnd == cwnd && now_j.saturating_sub(c.last_time) <= HZ / 32 {
            return;
        }
        if !(c.epoch_start.is_some() && now_j == c.last_time) {
            c.last_cwnd = cwnd;
            c.last_time = now_j;
            let epoch = match c.epoch_start {
                Some(e) => e,
                None => {
                    c.epoch_start = Some(now_j);
                    c.ack_cnt = acked;
                    c.tcp_cwnd = cwnd;
                    if c.last_max_cwnd <= cwnd {
                        c.bic_k = 0;
                        c.bic_origin_point = cwnd;
                    } else {
                        c.bic_k = cubic_root(CUBE_FACTOR.wrapping_mul(c.last_max_cwnd - cwnd));
                        c.bic_origin_point = c.last_max_cwnd;
                    }
                    now_j
                }
            };
            let mut t = now_j.saturating_sub(epoch) + usecs_to_jiffies(c.delay_min);
            t <<= BICTCP_HZ;
            t /= HZ;
            let offs = c.bic_k.abs_diff(t);
            let delta = u64::from(
                (CUBE_RTT_SCALE
                    .wrapping_mul(offs)
                    .wrapping_mul(offs)
                    .wrapping_mul(offs)
                    >> (10 + 3 * BICTCP_HZ)) as u32,
            );
            let origin = c.bic_origin_point as u32;
            let bic_target = u64::from(if t < c.bic_k {
                origin.wrapping_sub(delta as u32)
            } else {
                origin.wrapping_add(delta as u32)
            });
            c.cnt = if bic_target > cwnd {
                cwnd / (bic_target - cwnd)
            } else {
                100 * cwnd
            };
            if c.last_max_cwnd == 0 && c.cnt > 20 {
                c.cnt = 20;
            }
        }
        // TCP friendliness (:299-316).
        let d = ((cwnd * CUBIC_BETA_SCALE) >> 3).max(1);
        while c.ack_cnt > d {
            c.ack_cnt -= d;
            c.tcp_cwnd += 1;
        }
        if c.tcp_cwnd > cwnd {
            let max_cnt = cwnd / (c.tcp_cwnd - cwnd);
            if c.cnt > max_cnt {
                c.cnt = max_cnt;
            }
        }
        c.cnt = c.cnt.max(2);
    }

    /// tcp_slow_start (tcp_cong.c:458-466): returns the ACKs left over above ssthresh.
    fn slow_start(&mut self, acked: u64) -> u64 {
        let cwnd = (self.cwnd + acked).min(self.ssthresh);
        let left = acked - (cwnd - self.cwnd);
        self.cwnd = cwnd;
        left
    }

    /// tcp_cong_avoid_ai (tcp_cong.c:472-488).
    fn cong_avoid_ai(&mut self, w: u64, acked: u64) {
        if self.cwnd_cnt >= w {
            self.cwnd_cnt = 0;
            self.cwnd += 1;
        }
        self.cwnd_cnt += acked;
        if self.cwnd_cnt >= w {
            let delta = self.cwnd_cnt / w;
            self.cwnd_cnt -= delta * w;
            self.cwnd += delta;
        }
    }

    /// cubictcp_cong_avoid (tcp_cubic.c:324-339).
    fn cong_avoid(&mut self, acked: u64, now: u64) {
        if !self.cwnd_limited() {
            return;
        }
        let mut acked = acked;
        if self.cwnd < self.ssthresh {
            acked = self.slow_start(acked);
            if acked == 0 {
                return;
            }
        }
        #[cfg(test)]
        if self.bitten(Variant::NoCubic) {
            return;
        }
        let cwnd = self.cwnd;
        self.bictcp_update(cwnd, acked, now);
        let cnt = self.cubic.cnt;
        self.cong_avoid_ai(cnt, acked);
    }

    /// hystart_ack_delay (tcp_cubic.c:375-384), from the pacing rate.
    fn hystart_ack_delay(&self) -> u64 {
        if self.pacing_rate == 0 {
            return 0;
        }
        1_000u64.min(GSO_MAX_SIZE * 4 * 1_000_000 / self.pacing_rate)
    }

    /// cubictcp_acked (tcp_cubic.c:448-474) and hystart_update (:386-446).
    fn cubic_acked(&mut self, rtt_us: Option<i64>, now: u64) {
        let Some(r) = rtt_us.filter(|&r| r >= 0) else {
            return;
        };
        if let Some(e) = self.cubic.epoch_start
            && jiffies(now).saturating_sub(e) < HZ
        {
            return;
        }
        let delay = r.unsigned_abs().max(1);
        if self.cubic.delay_min == 0 || self.cubic.delay_min > delay {
            self.cubic.delay_min = delay;
        }
        if self.cubic.found || self.cwnd >= self.ssthresh || self.cwnd < HYSTART_LOW_WINDOW {
            return;
        }
        if self.snd_una > self.cubic.end_seq {
            self.hystart_reset(now);
        }
        let now_us = now / US;
        if now_us.saturating_sub(self.cubic.last_ack) <= HYSTART_ACK_DELTA_US {
            self.cubic.last_ack = now_us;
            // SK_PACING_NONE without the fq qdisc: the threshold is halved.
            let threshold = (self.cubic.delay_min + self.hystart_ack_delay()) >> 1;
            if now_us.saturating_sub(self.cubic.round_start) > threshold {
                self.cubic.found = true;
                self.ssthresh = self.cwnd;
            }
        }
        let c = &mut self.cubic;
        if c.curr_rtt > delay {
            c.curr_rtt = delay;
        }
        if c.sample_cnt < HYSTART_MIN_SAMPLES {
            c.sample_cnt += 1;
        } else if c.curr_rtt
            > c.delay_min + (c.delay_min >> 3).clamp(HYSTART_DELAY_MIN_US, HYSTART_DELAY_MAX_US)
        {
            c.found = true;
            self.ssthresh = self.cwnd;
        }
    }

    /// tcp_update_pacing_rate (tcp_input.c:906-940's caller, tcp_cong_control): 200 % of cwnd x mss / srtt in slow
    /// start below half ssthresh, else 120 %.
    fn update_pacing_rate(&mut self) {
        let ratio = if self.cwnd < self.ssthresh / 2 {
            200
        } else {
            120
        };
        let mut rate = self.mss * 80_000 * ratio * self.cwnd.max(self.packets_out());
        if let Ok(srtt8) = u64::try_from(self.rtt.srtt8)
            && srtt8 > 0
        {
            rate /= srtt8;
        }
        self.pacing_rate = rate;
    }

    // ---- States, reductions and undo (tcp_input.c) ----

    /// tcp_set_ca_state (tcp_cong.c:37-45) with Cubic's set_state (tcp_cubic.c:358-364).
    fn set_state(&mut self, s: State, now: u64) {
        if s == State::Loss {
            self.bictcp_reset();
            self.hystart_reset(now);
        }
        self.state = s;
    }

    /// tcp_current_ssthresh (include/net/tcp.h:1290-1300).
    fn current_ssthresh(&self) -> u64 {
        if self.in_cwnd_reduction() {
            self.ssthresh
        } else {
            self.ssthresh.max((self.cwnd >> 1) + (self.cwnd >> 2))
        }
    }

    /// tcp_init_undo (:2126-2131).
    fn init_undo(&mut self) {
        self.undo_marker = Some(self.snd_una);
        let r = self.retrans_out();
        self.undo_retrans = if r > 0 { r as i64 } else { -1 };
    }

    /// tcp_any_retrans_done (:2479-2492).
    fn any_retrans_done(&self) -> bool {
        self.retrans_out() > 0 || self.segs.front().is_some_and(|g| g.ever)
    }

    /// tcp_packet_delayed (:2457-2461) with tcp_tsopt_ecr_before (:2438-2442): the echo predates the first resend.
    fn packet_delayed(&self) -> bool {
        self.retrans_stamp
            .is_some_and(|rs| self.rcv_tsecr.is_some_and(|ts| ts < rs))
    }

    /// tcp_may_undo (:2550-2553).
    fn may_undo(&self) -> bool {
        #[cfg(test)]
        if self.bitten(Variant::NoUndo) {
            return false;
        }
        self.undo_marker.is_some() && (self.undo_retrans == 0 || self.packet_delayed())
    }

    /// tcp_undo_cwnd_reduction (:2521-2548), Cubic's undo_cwnd being tcp_reno_undo_cwnd (tcp_cong.c:525-530).
    fn undo_cwnd_reduction(&mut self, unmark_loss: bool) {
        if unmark_loss {
            for g in &mut self.segs {
                g.lost = false;
            }
        }
        if self.prior_ssthresh != 0 {
            self.cwnd = self.cwnd.max(self.prior_cwnd);
            if self.prior_ssthresh > self.ssthresh {
                self.ssthresh = self.prior_ssthresh;
            }
        }
        self.undo_marker = None;
        self.rack.advanced = true;
    }

    /// tcp_try_undo_recovery (:2571-2598), for a SACK connection: it always leaves for Open.
    fn try_undo_recovery(&mut self, now: u64) {
        if self.may_undo() {
            self.undo_cwnd_reduction(false);
            if self.state == State::Loss {
                self.undos_loss += 1;
            } else {
                self.undos_recovery += 1;
            }
        }
        self.set_state(State::Open, now);
    }

    /// tcp_try_undo_dsack (:2600-2613).
    fn try_undo_dsack(&mut self) -> bool {
        #[cfg(test)]
        if self.bitten(Variant::NoUndo) {
            return false;
        }
        if self.undo_marker.is_some() && self.undo_retrans == 0 {
            self.undo_cwnd_reduction(false);
            self.undos_recovery += 1;
            return true;
        }
        false
    }

    /// tcp_try_undo_loss (:2616-2639).
    fn try_undo_loss(&mut self, frto_undo: bool, now: u64) -> bool {
        #[cfg(test)]
        if self.bitten(Variant::NoUndo) {
            return false;
        }
        if frto_undo || self.may_undo() {
            self.undo_cwnd_reduction(true);
            if frto_undo {
                self.undos_frto += 1;
            } else {
                self.undos_loss += 1;
            }
            self.retransmits = 0;
            self.set_state(State::Open, now);
            return true;
        }
        false
    }

    /// tcp_try_undo_partial (:2935-2966): true when the ACK's processing stops here.
    fn try_undo_partial(&mut self, prior_snd_una: u64) -> bool {
        #[cfg(test)]
        if self.bitten(Variant::NoUndo) {
            return false;
        }
        if self.undo_marker.is_some() && self.packet_delayed() {
            self.check_sack_reordering(prior_snd_una);
            if self.retrans_out() > 0 {
                return true;
            }
            if !self.any_retrans_done() {
                self.retrans_stamp = None;
            }
            self.undo_cwnd_reduction(true);
            self.undos_recovery += 1;
            self.try_keep_open();
        }
        false
    }

    /// tcp_init_cwnd_reduction (:2649-2661).
    fn init_cwnd_reduction(&mut self) {
        self.high_seq = self.snd_nxt;
        self.tlp_high_seq = None;
        self.cwnd_cnt = 0;
        self.prior_cwnd = self.cwnd;
        self.prr_delivered = 0;
        self.prr_out = 0;
        self.ssthresh = self.cubic_ssthresh();
    }

    /// tcp_cwnd_reduction (:2663-2687): PRR, RFC 6937.
    fn cwnd_reduction(&mut self, newly_acked_sacked: u64, newly_lost: u64, flag: u32) {
        if newly_acked_sacked == 0 || self.prior_cwnd == 0 {
            return;
        }
        let in_flight = self.in_flight() as i64;
        let delta = self.ssthresh as i64 - in_flight;
        self.prr_delivered += newly_acked_sacked;
        let mut sndcnt = if delta < 0 {
            let dividend = self.ssthresh * self.prr_delivered + self.prior_cwnd - 1;
            (dividend / self.prior_cwnd) as i64 - self.prr_out as i64
        } else {
            let mut n =
                (self.prr_delivered as i64 - self.prr_out as i64).max(newly_acked_sacked as i64);
            if flag & FLAG_SND_UNA_ADVANCED != 0 && newly_lost == 0 {
                n += 1;
            }
            n.min(delta)
        };
        sndcnt = sndcnt.max(i64::from(self.prr_out == 0));
        self.cwnd = u64::try_from(in_flight + sndcnt).unwrap_or(0);
    }

    /// tcp_end_cwnd_reduction (:2689-2703): cwnd = ssthresh in CWR, or when Recovery ends not undone.
    fn end_cwnd_reduction(&mut self) {
        #[cfg(test)]
        if self.bitten(Variant::NoExitCwnd) {
            return;
        }
        if self.ssthresh < INFINITE_SSTHRESH
            && (self.state == State::Cwr || self.undo_marker.is_some())
        {
            self.cwnd = self.ssthresh;
        }
    }

    /// tcp_try_keep_open (:2719-2731): Disorder while SACKed, lost or resent segments are out.
    fn try_keep_open(&mut self) {
        let left_out = self.sacked_out() + self.lost_out();
        let s = if left_out > 0 || self.any_retrans_done() {
            State::Disorder
        } else {
            State::Open
        };
        if self.state != s {
            self.state = s;
            self.high_seq = self.snd_nxt;
        }
    }

    /// tcp_try_to_open (:2733-2748).
    fn try_to_open(&mut self) {
        if !self.any_retrans_done() {
            self.retrans_stamp = None;
        }
        if self.state != State::Cwr {
            self.try_keep_open();
        }
    }

    /// tcp_enter_recovery (:2839-2859).
    fn enter_recovery(&mut self, now: u64) {
        self.prior_ssthresh = 0;
        self.init_undo();
        if !self.in_cwnd_reduction() {
            self.prior_ssthresh = self.current_ssthresh();
            self.init_cwnd_reduction();
        }
        self.set_state(State::Recovery, now);
    }

    /// tcp_mark_skb_lost (:1118-1139); `cause` attributes the resend it calls for.
    fn mark_lost(&mut self, i: usize, cause: Repair) {
        let g = &mut self.segs[i];
        if g.sacked {
            return;
        }
        if g.lost {
            if g.sretrans {
                g.sretrans = false;
                g.cause = Some(cause);
                self.lost += 1;
            }
        } else {
            g.lost = true;
            g.cause = Some(cause);
            self.lost += 1;
        }
    }

    /// tcp_timeout_mark_lost (:2143-2171): every segment but those sent within RACK's RTT (never the head). The
    /// head is never SACKed on these paths, so reneging does not arise.
    fn timeout_mark_lost(&mut self, now: u64) {
        let rack_rtt_us = if self.rack.set && self.rack_on() {
            self.rack.rtt_us
        } else {
            0
        };
        for i in 0..self.segs.len() {
            #[allow(unused_mut)]
            let mut recent = i > 0 && rack_rtt_us > (now - self.segs[i].xmit_ns) / US;
            #[cfg(test)]
            if self.bitten(Variant::RtoMarksAll) {
                recent = false;
            }
            if recent {
                continue;
            }
            self.mark_lost(i, Repair::Timeout);
        }
    }

    /// tcp_enter_loss (:2174-2218).
    fn enter_loss(&mut self, now: u64) {
        let new_recovery = self.state < State::Recovery;
        self.timeout_mark_lost(now);
        if self.state <= State::Disorder
            || self.high_seq <= self.snd_una
            || (self.state == State::Loss && self.retransmits == 0)
        {
            self.prior_ssthresh = self.current_ssthresh();
            self.prior_cwnd = self.cwnd;
            self.ssthresh = self.cubic_ssthresh();
            self.init_undo();
        }
        self.cwnd = self.in_flight() + 1;
        #[cfg(test)]
        if self.bitten(Variant::RtoCwndOne) {
            self.cwnd = 1;
        }
        self.cwnd_cnt = 0;
        if self.state <= State::Disorder && self.sacked_out() >= REORDERING {
            self.reordering = self.reordering.min(REORDERING);
        }
        self.set_state(State::Loss, now);
        self.high_seq = self.snd_nxt;
        // F-RTO (RFC 5682 3.1 step 1): unless a recovery was already running, bar repeated timeouts.
        self.frto = new_recovery || self.retransmits > 0;
        #[cfg(test)]
        if self.bitten(Variant::NoFrto) {
            self.frto = false;
        }
    }

    // ---- RACK (tcp_recovery.c) ----

    /// tcp_rack_reo_wnd (:5-30), reo_wnd_steps at 1 (its growth from D-SACKs is not modelled, S11).
    fn rack_reo_wnd(&self) -> u64 {
        if self.reord_seen == 0
            && (self.state >= State::Recovery || self.sacked_out() >= self.reordering)
        {
            return 0;
        }
        (self.rtt.min_rtt_us() >> 2).min(self.rtt.srtt_us())
    }

    /// tcp_rack_advance (:118-144).
    fn rack_advance(&mut self, retrans: bool, end_seq: u64, xmit_ns: u64, now: u64) {
        let rtt_us = (now - xmit_ns) / US;
        if rtt_us < self.rtt.min_rtt_us() && retrans {
            return;
        }
        self.rack.advanced = true;
        self.rack.rtt_us = rtt_us;
        if !self.rack.set || sent_after(xmit_ns, self.rack.mstamp_ns, end_seq, self.rack.end_seq) {
            self.rack.mstamp_ns = xmit_ns;
            self.rack.end_seq = end_seq;
        }
        self.rack.set = true;
    }

    /// tcp_rack_detect_loss (:58-93): marks what is past its deadline and returns the longest wait left (us).
    fn rack_detect_loss(&mut self, now: u64) -> u64 {
        let reo_wnd = self.rack_reo_wnd();
        let rack = self.rack;
        let mut reo_timeout = 0;
        for i in 0..self.segs.len() {
            let g = &self.segs[i];
            if !g.tsorted || g.sacked || (g.lost && !g.sretrans) {
                continue;
            }
            if !sent_after(rack.mstamp_ns, g.xmit_ns, rack.end_seq, g.end) {
                continue;
            }
            let elapsed = (now - g.xmit_ns) / US;
            let deadline = rack.rtt_us + reo_wnd;
            if elapsed >= deadline {
                self.mark_lost(i, Repair::Rack);
                self.segs[i].tsorted = false;
            } else {
                reo_timeout = reo_timeout.max(deadline - elapsed);
            }
        }
        reo_timeout
    }

    /// tcp_rack_mark_lost (:95-112): only after an ACK advanced RACK; arms the reorder timer (capped at the RTO).
    fn rack_mark_lost(&mut self, now: u64) -> bool {
        if !self.rack.advanced || !self.rack_on() {
            return false;
        }
        self.rack.advanced = false;
        let timeout = self.rack_detect_loss(now);
        if timeout > 0 {
            let when = ((timeout + TIMEOUT_MIN_US) * US).min(self.rto_ns());
            self.timer = Some((now + when, TimerKind::Reorder));
        }
        timeout > 0
    }

    /// tcp_identify_packet_loss (:2968-2985).
    fn identify_packet_loss(&mut self, flag: &mut u32, now: u64) {
        if self.segs.is_empty() {
            return;
        }
        let prior_retrans = self.retrans_out();
        if self.rack_mark_lost(now) {
            *flag &= !FLAG_SET_XMIT_TIMER;
        }
        if prior_retrans > self.retrans_out() {
            *flag |= FLAG_LOST_RETRANS;
        }
    }

    /// tcp_time_to_recover (:2359-2372) with RACK: a loss is marked.
    fn time_to_recover(&self) -> bool {
        self.lost_out() > 0
    }

    /// tcp_check_sack_reordering (:1065-1094).
    fn check_sack_reordering(&mut self, low_seq: u64) {
        let fack = self.highest_sack_seq();
        if low_seq >= fack {
            return;
        }
        let metric = fack - low_seq;
        if metric > self.reordering * self.mss {
            self.reordering = metric.div_ceil(self.mss).min(MAX_REORDERING);
        }
        self.reord_seen += 1;
    }

    // ---- The SACK scoreboard (tcp_input.c) ----

    /// tcp_dsack_seen (:1022-1057): the D-SACKed segments, or 0 for a dubious D-SACK (more D-SACKed than resent).
    fn dsack_seen(&mut self, start: u64, end: u64, st: &mut SackState, flag: &mut u32) -> u64 {
        if start >= end {
            return 0;
        }
        let len = end - start;
        let mut dup = 1;
        if len > self.mss {
            dup = len.div_ceil(self.mss);
        } else if self.tlp_high_seq == Some(end) {
            *flag |= FLAG_DSACK_TLP;
        }
        self.dsack_dups += dup;
        if self.dsack_dups > self.total_retrans {
            return 0;
        }
        *flag |= FLAG_DSACKING_ACK;
        st.sack_delivered += dup;
        self.dsacks += 1;
        dup
    }

    /// tcp_is_sackblock_valid (:1245-1280); the max_window bounds never bind on one-segment ranges.
    fn sackblock_valid(&self, is_dsack: bool, start: u64, end: u64) -> bool {
        if end > self.snd_nxt || start >= end || start >= self.snd_nxt {
            return false;
        }
        if start > self.snd_una {
            return true;
        }
        let Some(marker) = self.undo_marker.filter(|_| is_dsack) else {
            return false;
        };
        if end > self.snd_una {
            return false;
        }
        if start >= marker {
            return true;
        }
        end > marker
    }

    /// tcp_sacktag_one (:1378-1458).
    fn sacktag_one(
        &mut self,
        i: usize,
        dup_sack: bool,
        st: &mut SackState,
        flag: &mut u32,
        now: u64,
    ) {
        let g = self.segs[i].clone();
        if dup_sack && g.retrans() {
            if let Some(m) = self.undo_marker
                && self.undo_retrans > 0
                && g.end > m
            {
                self.undo_retrans = (self.undo_retrans - 1).max(0);
            }
            if g.sacked && g.start < st.reord {
                st.reord = g.start;
            }
        }
        if g.end <= self.snd_una {
            return;
        }
        if !g.sacked {
            self.rack_advance(g.retrans(), g.end, g.xmit_ns, now);
            let highest = self.highest_sack_seq();
            let s = &mut self.segs[i];
            if g.sretrans {
                // A resend still believed in flight keeps its mark unless it was lost (:1405-1414).
                if g.lost {
                    s.lost = false;
                    s.sretrans = false;
                }
            } else {
                if !g.retrans() {
                    if g.start < highest && g.start < st.reord {
                        st.reord = g.start;
                    }
                    if g.end <= self.high_seq {
                        *flag |= FLAG_ORIG_SACK_ACKED;
                    }
                    st.first_sackt.get_or_insert(g.xmit_ns);
                    st.last_sackt = Some(g.xmit_ns);
                }
                s.lost = false;
            }
            s.sacked = true;
            s.tsorted = false;
            *flag |= FLAG_DATA_SACKED;
            st.sack_delivered += 1;
        }
        if dup_sack && self.segs[i].sretrans {
            self.segs[i].sretrans = false;
        }
    }

    /// tcp_sacktag_write_queue (:1842-2035) after tcp_check_dsack (:1282-1323). Each segment is tagged once, as a
    /// duplicate when it lies in the valid D-SACK block (the walk's next_dup path), in ascending order.
    fn sacktag(&mut self, ack: &Ack, prior_snd_una: u64, st: &mut SackState, now: u64) -> u32 {
        let mut flag = 0;
        let blocks = &ack.sacks;
        let mut found_dup = false;
        if let Some(&(s0, e0)) = blocks.first() {
            let looks_dup =
                s0 < ack.cum || blocks.get(1).is_some_and(|&(s1, e1)| !(e0 > e1 || s0 < s1));
            if looks_dup {
                let dup_segs = self.dsack_seen(s0, e0, st, &mut flag);
                if dup_segs > 0 {
                    found_dup = true;
                    // D-SACK for already forgotten data: dumb counting.
                    if let Some(m) = self.undo_marker
                        && self.undo_retrans > 0
                        && e0 <= prior_snd_una
                        && e0 > m
                    {
                        self.undo_retrans = (self.undo_retrans - dup_segs as i64).max(0);
                    }
                }
            }
        }
        if self.segs.is_empty() {
            return flag;
        }
        let mut dsack_block = None;
        let mut regular = Vec::new();
        for (i, &(a, b)) in blocks.iter().enumerate() {
            let dup = i == 0 && found_dup;
            if !self.sackblock_valid(dup, a, b) || b <= prior_snd_una {
                continue;
            }
            if dup {
                dsack_block = Some((a, b));
            } else {
                regular.push((a, b));
            }
        }
        for i in 0..self.segs.len() {
            let (gs, ge) = (self.segs[i].start, self.segs[i].end);
            let in_dup = dsack_block.is_some_and(|(a, b)| a <= gs && ge <= b);
            if in_dup || regular.iter().any(|&(a, b)| a <= gs && ge <= b) {
                self.sacktag_one(i, in_dup, st, &mut flag, now);
            }
        }
        if self.state != State::Loss || self.undo_marker.is_some() {
            self.check_sack_reordering(st.reord);
        }
        flag
    }

    /// tcp_clean_rtx_queue (:3279-3483) with tcp_ack_update_rtt (:3135-3174) and Cubic's pkts_acked. Returns its
    /// flags and the segments it newly delivered.
    fn clean_rtx_queue(&mut self, prior_fack: u64, st: &SackState, now: u64) -> (u32, u64) {
        let tsecr = self.rcv_tsecr;
        let mut flag = 0;
        let mut reord = self.snd_nxt;
        let (mut first_ackt, mut last_ackt) = (None, None);
        let mut delivered = 0;
        while self.segs.front().is_some_and(|g| g.end <= self.snd_una) {
            let Some(g) = self.segs.pop_front() else {
                break;
            };
            #[allow(unused_mut)]
            let mut retrans = g.retrans();
            #[cfg(test)]
            if self.bitten(Variant::NoKarn) {
                retrans = false;
            }
            if retrans {
                flag |= FLAG_RETRANS_DATA_ACKED;
            } else if !g.sacked {
                last_ackt = Some(g.xmit_ns);
                first_ackt.get_or_insert(g.xmit_ns);
                if g.start < reord {
                    reord = g.start;
                }
                if g.end <= self.high_seq {
                    flag |= FLAG_ORIG_SACK_ACKED;
                }
            }
            if !g.sacked {
                delivered += 1;
                // tcp_skb_spurious_retrans (:2447-2452): a resend whose echo predates it advances nothing.
                let spurious = g.retrans() && tsecr.is_some_and(|ts| ts < ts_of(g.xmit_ns));
                if !spurious {
                    self.rack_advance(g.retrans(), g.end, g.xmit_ns, now);
                }
            }
            flag |= FLAG_DATA_ACKED;
        }
        if self.segs.is_empty() {
            self.retrans_leak = 0;
        }
        let us = |x: u64| i64::try_from((now - x) / US).unwrap_or(i64::MAX);
        let (mut seq_rtt, mut ca_rtt) = (None, None);
        if let (Some(f), Some(l)) = (first_ackt, last_ackt)
            && flag & FLAG_RETRANS_DATA_ACKED == 0
        {
            seq_rtt = Some(us(f));
            ca_rtt = Some(us(l));
        }
        let sack_rtt = st.first_sackt.map(us);
        if let Some(l) = st.last_sackt {
            ca_rtt = Some(us(l));
        }
        // tcp_ack_update_rtt: the sequence sample, else the SACK sample, else the echo when data was delivered.
        let mut rtt = seq_rtt.or(sack_rtt);
        #[allow(unused_mut)]
        let mut ts_fallback = true;
        #[cfg(test)]
        if self.bitten(Variant::NoTsecrFallback) {
            ts_fallback = false;
        }
        if rtt.is_none()
            && flag & FLAG_ACKED != 0
            && ts_fallback
            && let Some(ts) = tsecr
        {
            let m = tsopt_rtt_us(now, ts);
            rtt = Some(m);
            ca_rtt = Some(m);
        }
        let rs_rtt = ca_rtt;
        let rtt_update = if let (Some(m), Some(ca)) = (rtt, ca_rtt) {
            self.rtt
                .sample(m, ca, self.snd_una, self.snd_nxt, self.rto_min_us);
            self.backoff = 0;
            true
        } else {
            false
        };
        if flag & FLAG_ACKED != 0 {
            flag |= FLAG_SET_XMIT_TIMER;
            if reord < prior_fack {
                self.check_sack_reordering(reord);
            }
        } else if let Some(head) = self.segs.front()
            && rtt_update
            && sack_rtt.is_some_and(|r| r > us(head.xmit_ns))
        {
            // Not from data sent after the head's last transmission: the timer may move (:3420-3428).
            flag |= FLAG_SET_XMIT_TIMER;
        }
        self.cubic_acked(rs_rtt, now);
        (flag, delivered)
    }

    /// tcp_process_tlp_ack (:3764-3792).
    fn process_tlp_ack(&mut self, ack: u64, flag: u32) {
        let Some(h) = self.tlp_high_seq else {
            return;
        };
        if ack < h {
            return;
        }
        if !self.tlp_retrans || flag & FLAG_DSACK_TLP != 0 {
            self.tlp_high_seq = None;
        } else if ack > h {
            // The probe repaired a loss: reduce at once (CWR, then its end, then Open or Disorder).
            self.init_cwnd_reduction();
            self.state = State::Cwr;
            self.end_cwnd_reduction();
            self.try_keep_open();
        } else if flag & (FLAG_SND_UNA_ADVANCED | FLAG_NOT_DUP | FLAG_DATA_SACKED) == 0 {
            self.tlp_high_seq = None;
        }
    }

    /// tcp_process_loss (:2873-2924), with F-RTO's steps 2.b, 3.a and 3.b.
    fn process_loss(&mut self, flag: u32, num_dupack: u64, rexmit: &mut Rexmit, now: u64) {
        let recovered = self.snd_una >= self.high_seq;
        if flag & FLAG_SND_UNA_ADVANCED != 0 && self.try_undo_loss(false, now) {
            return;
        }
        if self.frto {
            // Step 3.b: data never resent was (s)acked, so not everything was lost.
            if flag & FLAG_ORIG_SACK_ACKED != 0 && self.try_undo_loss(true, now) {
                return;
            }
            if self.snd_nxt > self.high_seq {
                if flag & FLAG_DATA_SACKED != 0 || num_dupack > 0 {
                    self.frto = false; // Step 3.a: the loss was real.
                }
            } else if flag & FLAG_SND_UNA_ADVANCED != 0 && !recovered {
                self.high_seq = self.snd_nxt;
                // Step 2.b: new data first, when there is some (the receive window is never the limit here).
                if self.snd_nxt < self.write_seq {
                    *rexmit = Rexmit::New;
                    return;
                }
                self.frto = false;
            }
        }
        if recovered {
            self.try_undo_recovery(now);
            return;
        }
        *rexmit = Rexmit::Lost;
    }

    /// tcp_fastretrans_alert (:2999-3117), for a SACK connection with RACK and without ECN.
    fn fastretrans_alert(
        &mut self,
        prior_snd_una: u64,
        num_dupack: u64,
        flag: &mut u32,
        rexmit: &mut Rexmit,
        now: u64,
    ) {
        // D. The state's exit when high_seq is acknowledged.
        if self.state == State::Open {
            self.retrans_stamp = None;
        } else if self.snd_una >= self.high_seq {
            match self.state {
                State::Cwr if self.snd_una != self.high_seq => {
                    self.end_cwnd_reduction();
                    self.set_state(State::Open, now);
                }
                State::Recovery => {
                    self.try_undo_recovery(now);
                    self.end_cwnd_reduction();
                }
                _ => {}
            }
        }
        // E. The state's processing.
        let to_default = match self.state {
            State::Recovery => {
                if *flag & FLAG_SND_UNA_ADVANCED != 0 && self.try_undo_partial(prior_snd_una) {
                    return;
                }
                if self.try_undo_dsack() {
                    self.try_keep_open();
                }
                self.identify_packet_loss(flag, now);
                if self.state != State::Recovery {
                    if !self.time_to_recover() {
                        return;
                    }
                    self.enter_recovery(now);
                }
                false
            }
            State::Loss => {
                self.process_loss(*flag, num_dupack, rexmit, now);
                self.identify_packet_loss(flag, now);
                if !(self.state == State::Open || *flag & FLAG_LOST_RETRANS != 0) {
                    return;
                }
                true
            }
            _ => true,
        };
        if to_default {
            if self.state <= State::Disorder {
                self.try_undo_dsack();
            }
            self.identify_packet_loss(flag, now);
            if !self.time_to_recover() {
                self.try_to_open();
                return;
            }
            self.enter_recovery(now);
        }
        *rexmit = Rexmit::Lost;
    }

    /// tcp_may_raise_cwnd (:3516-3528).
    fn may_raise_cwnd(&self, flag: u32) -> bool {
        if self.reordering > REORDERING {
            flag & FLAG_FORWARD_PROGRESS != 0
        } else {
            flag & FLAG_DATA_ACKED != 0
        }
    }

    /// tcp_cong_control (:3536-3553): PRR in CWR and Recovery, else Cubic when the ACK may raise cwnd.
    fn cong_control(&mut self, delivered: u64, losses: u64, flag: u32, now: u64) {
        if self.in_cwnd_reduction() {
            self.cwnd_reduction(delivered, losses, flag);
        } else if self.may_raise_cwnd(flag) {
            self.cong_avoid(delivered, now);
        }
        self.update_pacing_rate();
    }

    /// tcp_rack_reo_timeout's loss marking (tcp_recovery.c:149-167); true when it changed what is in flight.
    fn reo_timeout_mark(&mut self, now: u64) -> bool {
        let prior = self.in_flight();
        let prior_lost = self.lost;
        if self.rack_on() {
            self.rack_detect_loss(now);
        }
        if self.in_flight() == prior {
            return false;
        }
        if self.state != State::Recovery {
            self.enter_recovery(now);
            let newly_lost = self.lost - prior_lost;
            self.cwnd_reduction(1, newly_lost, 0);
        }
        true
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
    /// selective_acks: the SACK blocks, most recently changed first, at most 4 (tcp_sack_new_ofo_skb).
    sacks: Vec<(u64, u64)>,
    /// duplicate_sack[0] while rx_opt.dsack is set (tcp_dsack_set).
    dsack: Option<(u64, u64)>,
    /// The SACK blocks the option has room for: 3 beside the timestamp option, 4 without (tcp_established_options).
    max_blocks: usize,
    /// The receiver never sends a D-SACK (a bite).
    #[cfg(test)]
    no_dsack: bool,
}

/// tcp_sack_extend: grows a block over an overlapping or adjacent range.
fn sack_extend(sp: &mut (u64, u64), seq: u64, end: u64) -> bool {
    if sp.0 <= end && seq <= sp.1 {
        sp.0 = sp.0.min(seq);
        sp.1 = sp.1.max(end);
        return true;
    }
    false
}

impl Receiver {
    /// The ACK this end's next transmission carries. Writing the SACK option clears the D-SACK (tcp_options_write,
    /// tcp_output.c:709-726): the D-SACK first, then the remembered blocks, as many as the option has room for.
    fn ack(&mut self) -> Ack {
        let mut sacks = Vec::new();
        if let Some(d) = self.dsack.take() {
            sacks.push(d);
        }
        let room = self.max_blocks.saturating_sub(sacks.len());
        sacks.extend(self.sacks.iter().take(room).copied());
        Ack {
            cum: self.rcv_nxt,
            sacks,
            tsecr: self.ts_recent,
        }
    }

    fn held_bytes(&self) -> u64 {
        self.ooo.iter().map(|(s, e)| e - s).sum()
    }

    /// tcp_dsack_set (tcp_input.c:4565-4583).
    fn dsack_set(&mut self, s: u64, e: u64) {
        #[cfg(test)]
        if self.no_dsack {
            return;
        }
        self.dsack = Some((s, e));
    }

    /// tcp_sack_new_ofo_skb (:4696-4743) with tcp_sack_maybe_coalesce (:4643-4666).
    fn sack_new_ofo(&mut self, seq: u64, end: u64) {
        for i in 0..self.sacks.len() {
            if sack_extend(&mut self.sacks[i], seq, end) {
                let b = self.sacks.remove(i);
                self.sacks.insert(0, b);
                let mut j = 1;
                while j < self.sacks.len() {
                    let other = self.sacks[j];
                    if sack_extend(&mut self.sacks[0], other.0, other.1) {
                        self.sacks.remove(j);
                    } else {
                        j += 1;
                    }
                }
                return;
            }
        }
        if self.sacks.len() >= NUM_SACKS {
            self.sacks.pop();
        }
        self.sacks.insert(0, (seq, end));
    }

    /// tcp_sack_remove (:4745-4772): rcv_nxt advanced.
    fn sack_remove(&mut self) {
        if self.ooo.is_empty() {
            self.sacks.clear();
            return;
        }
        let nxt = self.rcv_nxt;
        self.sacks.retain(|&(s, _)| nxt < s);
    }

    /// tcp_data_queue (:5146-5265) and tcp_data_queue_ofo (:4924-5069) for [s, e): where it lands, and the D-SACK it
    /// calls for.
    fn data_queue(&mut self, s: u64, e: u64) -> Arrival {
        self.dsack = None;
        if e <= self.rcv_nxt {
            self.dsack_set(s, e);
            return Arrival::Duplicate;
        }
        if s <= self.rcv_nxt {
            if s < self.rcv_nxt {
                let nxt = self.rcv_nxt;
                self.dsack_set(s, nxt);
            }
            let had_hole = !self.ooo.is_empty();
            self.rcv_nxt = e;
            while let Some((&os, &oe)) = self.ooo.iter().next() {
                if os > self.rcv_nxt {
                    break;
                }
                self.ooo.remove(&os);
                self.rcv_nxt = self.rcv_nxt.max(oe);
            }
            if !self.sacks.is_empty() {
                self.sack_remove();
            }
            return if had_hole {
                Arrival::FilledHole
            } else {
                Arrival::InOrder
            };
        }
        // Out of order: a range already held is a duplicate (D-SACK); else merge it with every range it touches.
        let held = self
            .ooo
            .range(..=s)
            .next_back()
            .is_some_and(|(_, &oe)| oe >= e);
        if held {
            self.dsack_set(s, e);
        } else {
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
        }
        self.sack_new_ofo(s, e);
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
    /// The SACK option's blocks in the order sent: a D-SACK first when there is one.
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

/// One end's sending path: its fate stream and its first-in, first-out queue (arrivals strictly increase).
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
    /// The client's SYN timer (TcpConfig).
    syn_rto_us: u64,
    syn_linear_timeouts: u32,
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
    /// (end, from, to): every transmission `end` sends in [from, to) is lost (the tests' blackouts).
    #[cfg(test)]
    blackouts: Vec<(usize, u64, u64)>,
    /// Whose transmissions the profile's loss applies to (both, unless `with_loss_from` picks one).
    #[cfg(test)]
    loss_from: [bool; 2],
    /// Every data transmission: (end, segment start, attempt, time, lost).
    #[cfg(test)]
    sent_log: Vec<(usize, u64, u32, u64, bool)>,
    /// A diagnostic trace of end 0's sender, times relative to this stamp (tests only).
    #[cfg(test)]
    trace: Option<u64>,
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
            syn_timer: Some(now + m.syn_rto_us.min(RTO_MAX_US) * US),
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
            rcv: Receiver {
                max_blocks: if cfg.timestamps { 3 } else { 4 },
                ..Receiver::default()
            },
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
            syn_rto_us: cfg.syn_rto_us,
            syn_linear_timeouts: cfg.syn_linear_timeouts,
            ends: [end(), end()],
            paths: [path(0), path(1)],
            now: 0,
            setup: None,
            #[cfg(test)]
            variant: Variant::Faithful,
            #[cfg(test)]
            forced: Vec::new(),
            #[cfg(test)]
            blackouts: Vec::new(),
            #[cfg(test)]
            loss_from: [true, true],
            #[cfg(test)]
            sent_log: Vec::new(),
            #[cfg(test)]
            trace: None,
        }
    }

    /// The same connection with each direction's mean delay set to `ns`: the profile's half round trip net of the
    /// path's measured base round trip (L2Config's `base_rtt_ns`, through `l2::config::one_way_ns`, the helper
    /// ImpairSocket uses too: u3c-plan.md C5). Set before any traffic.
    pub(crate) fn with_one_way_ns(mut self, ns: u64) -> TcpModel {
        self.one_way = ns;
        self
    }

    /// The model's clock: the latest stamp it was advanced or written to.
    pub(crate) fn now_ns(&self) -> u64 {
        self.now
    }

    /// Each direction's mean one-way delay and its jitter's half-width (S1).
    pub(crate) fn one_way_ns(&self) -> u64 {
        self.one_way
    }
    pub(crate) fn half_jitter_ns(&self) -> u64 {
        self.half_jitter
    }

    /// The segment size: 1,448 B with timestamps, 1,460 B without.
    pub(crate) fn mss(&self) -> u64 {
        self.mss
    }

    /// Adds a burst of total loss, both ways, from `start_ns` for `len_ns` (u3c-plan.md C4: per-session B1 on the
    /// WebSocket fallback, plan E23; ADR 0001's check item 11). The start must be at or after the model's now: an
    /// earlier one is refused (false), since the model has already sent through that time and the burst could not be
    /// applied as written. Bursts that ended at or before now are dropped first (they can lose nothing more), and at
    /// most [`MAX_BURSTS`] stay pending, so the list never grows with calls; beyond that, and for a zero length, the
    /// burst is refused too.
    pub(crate) fn add_burst(&mut self, start_ns: u64, len_ns: u64) -> bool {
        let now = self.now;
        if start_ns < now || len_ns == 0 {
            return false;
        }
        self.bursts.retain(|&(s, l)| s.saturating_add(l) > now);
        if self.bursts.len() >= MAX_BURSTS {
            return false;
        }
        self.bursts.push((start_ns, len_ns));
        true
    }

    /// The bursts the model holds, as (start, length): the profile's and those added, less those that had ended when
    /// the latest burst was added (only `add_burst` prunes, so some may have ended since; an ended burst loses nothing
    /// more, since a transmission is never stamped before the model's now).
    pub(crate) fn bursts(&self) -> &[(u64, u64)] {
        &self.bursts
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

    /// Bytes `end` has written and not yet sent (SIOCOUTQNSD on a real socket): the mechanism tests' bite input
    /// ("outstanding = unsent only", u3c-plan.md 1.4 (d)).
    #[cfg(test)]
    pub(crate) fn unsent(&self, end: usize) -> u64 {
        let s = &self.ends[end].snd;
        s.write_seq - s.snd_nxt
    }

    /// Every data transmission so far: (end, segment start, attempt, time, lost).
    #[cfg(test)]
    pub(crate) fn sent_log(&self) -> &[(usize, u64, u32, u64, bool)] {
        &self.sent_log
    }

    /// The kernel cross-check's pattern (u3c-plan.md 1.6): the profile's loss on `end`'s transmissions only, the other
    /// direction lossless (its fate words are still drawn, so the fates stay where they were).
    #[cfg(test)]
    pub(crate) fn with_loss_from(mut self, end: usize) -> TcpModel {
        self.loss_from = [end == 0, end == 1];
        self
    }

    /// The counts of the direction `end` sends.
    pub(crate) fn counters(&self, end: usize) -> Counters {
        let s = &self.ends[end].snd;
        Counters {
            dsacks: s.dsacks,
            undos_loss: s.undos_loss,
            undos_frto: s.undos_frto,
            undos_recovery: s.undos_recovery,
            ..self.ends[end].counters
        }
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
                s.synack_timer =
                    Some(now + (RTO_INIT_US << s.synack_timeouts.min(7)).min(RTO_MAX_US) * US);
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
                    // The SYN timer ran tcp_enter_loss, so the client has an undo marker, unless the echo is the
                    // first SYN's timestamp (retrans_stamp, tcp_connect): tcp_try_undo_spurious_syn
                    // (tcp_input.c:6319-6332, called at :6411 before tcp_init_transfer). Each SYN resend added one
                    // to undo_retrans after tcp_init_undo's -1.
                    let mut undo = s.syn_tries > 1;
                    if self.spurious_syn(pk.ack.tsecr, s.syn_sent_ns) {
                        undo = false;
                    }
                    let resends = s.syn_tries - 1;
                    self.establish(1, sample, resends, undo, i64::from(resends), now);
                }
                // The handshake's ACK (again, for a repeated SYN-ACK).
                self.transmit(1, None, 0, now);
            }
            _ => {}
        }
    }

    /// tcp_try_undo_spurious_syn's test: the echo equals the first SYN's (or SYN-ACK's) timestamp.
    fn spurious_syn(&self, tsecr: Option<u64>, first_sent_ns: u64) -> bool {
        #[cfg(test)]
        if self.variant == Variant::NoSpuriousSyn {
            return false;
        }
        self.timestamps && tsecr == Some(ts_of(first_sent_ns))
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
            // The child gets an undo marker only if the request's timer resent (tcp_minisocks.c:594-606), with
            // retrans_stamp the first SYN-ACK's; tcp_try_undo_spurious_syn clears it when the echo is that stamp
            // (tcp_input.c:6722). Its undo_retrans starts at 0.
            let undo = s.synack_timeouts > 0 && !self.spurious_syn(pk.ack.tsecr, s.synack_sent_ns);
            self.establish(0, sample, s.synack_tries - 1, undo, 0, now);
        }
    }

    /// One end's handshake done (tcp_init_transfer, tcp_init_metrics, tcp_init_congestion_control, tcp_input.c:
    /// 6187-6213): primed by `sample` (else the RTO falls back to 3 s); `resends` handshake packets resent
    /// (total_retrans); cwnd 1 after more than one when an undo marker survived; ssthresh back to infinite; Cubic
    /// starts afresh.
    fn establish(
        &mut self,
        end: usize,
        sample: Option<i64>,
        resends: u32,
        undo: bool,
        undo_retrans: i64,
        now: u64,
    ) {
        let snd = &mut self.ends[end].snd;
        match sample {
            Some(m) => snd.rtt.sample(m, m, 0, 0, snd.rto_min_us),
            None => snd.rtt.fallback = true,
        }
        snd.total_retrans = u64::from(resends);
        snd.undo_marker = undo.then_some(0);
        snd.undo_retrans = undo_retrans;
        snd.prior_ssthresh = 0;
        snd.ssthresh = INFINITE_SSTHRESH;
        snd.cwnd = if resends > 1 && undo { 1 } else { INIT_CWND };
        snd.lsndtime_ns = now;
        snd.cubic_init(now);
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
                let end = pk.data.map_or(pk.seq, |(_, e)| e);
                // tcp_validate_incoming (tcp_input.c:5857-5900): PAWS first (never fails on these first-in, first-out
                // paths with rising TSvals), then tcp_sequence (:4424-4434): a segment ending before rcv_wup is old,
                // discarded whole with its ACK field unread, and answered by tcp_send_dupack. Its right edge is the
                // receive window, which the 64 KB write budget keeps every segment inside.
                #[allow(unused_mut)]
                let mut old = end < self.ends[to].rcv.rcv_wup;
                #[cfg(test)]
                if self.variant == Variant::OldSeqProcessed {
                    old = false;
                }
                if old {
                    self.send_dupack(to, &pk, t);
                    return;
                }
                // tcp_replace_ts_recent: an in-window arrival (tcp_sequence: its end not below rcv_wup) that starts at
                // or below rcv_wup carries the timestamp to echo (PAWS-ordered).
                let r = &mut self.ends[to].rcv;
                #[allow(unused_mut)]
                let mut takes_ts = pk.seq <= r.rcv_wup;
                #[cfg(test)]
                if self.variant == Variant::TsRecentAlways {
                    takes_ts = true;
                }
                if self.timestamps && takes_ts && r.ts_recent.is_none_or(|v| pk.tsval >= v) {
                    r.ts_recent = Some(pk.tsval);
                }
                self.on_ack(to, &pk, t);
                if let Some((s, e)) = pk.data {
                    self.on_data(to, s, e, t);
                }
                self.push(to, t);
            }
            Event::SynTimer => {
                if let Some(mut s) = self.setup {
                    // tcp_retransmit_timer in SYN_SENT (tcp_timer.c:640-660): the RTO doubles once the timeouts
                    // so far exceed the linear ones (tcp_syn_linear_timeouts; TcpConfig for another client).
                    let timeouts = s.syn_tries;
                    s.syn_tries += 1;
                    let doublings = timeouts.saturating_sub(self.syn_linear_timeouts).min(16);
                    let wait = (self.syn_rto_us << doublings).min(RTO_MAX_US);
                    s.syn_timer = Some(t + wait * US);
                    self.setup = Some(s);
                    self.transmit_ctl(1, Ctl::Syn, t);
                }
            }
            Event::SynAckTimer => {
                if let Some(mut s) = self.setup {
                    s.synack_tries += 1;
                    s.synack_timeouts += 1;
                    s.synack_timer =
                        Some(t + (RTO_INIT_US << s.synack_timeouts.min(7)).min(RTO_MAX_US) * US);
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

    /// tcp_send_dupack (tcp_input.c:4618-4637): a D-SACK for the old data, an ACK at once.
    fn send_dupack(&mut self, q: usize, pk: &Packet, now: u64) {
        let r = &mut self.ends[q].rcv;
        if let Some((s, e)) = pk.data
            && s < r.rcv_nxt
        {
            let nxt = r.rcv_nxt;
            r.dsack_set(s, e.min(nxt));
        }
        self.ends[1 - q].counters.discards_old += 1;
        #[cfg(test)]
        if let Some(b) = self.trace {
            eprintln!(
                "{:+9.1} DISCARD at {q}: {:?}",
                (now as i64 - b as i64) as f64 / 1e6,
                pk.data
            );
        }
        self.transmit(q, None, 0, now);
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
        // First in, first out, and no two packets arrive at the same instant: the kernel processes each arrival at its
        // own clock reading (tcp_mstamp_refresh per segment), so sends from separate arrivals never share a stamp,
        // which RACK's tie-break by sequence (tcp_skb_sent_after) would otherwise misorder.
        let arrival = (now + delay).max(self.paths[end].last_arrival + 1);
        let burst = self.in_burst(now);
        #[allow(unused_mut)]
        let mut drawn = below(w[0], 1_000_000) < u64::from(self.loss_ppm);
        #[cfg(test)]
        if !self.loss_from[end] {
            drawn = false;
        }
        #[allow(unused_mut)]
        let mut lost = burst || drawn;
        #[cfg(test)]
        if let Some((s, _)) = data
            && self.forced.contains(&(end, s, attempt))
        {
            lost = true;
        }
        #[cfg(test)]
        if self
            .blackouts
            .iter()
            .any(|&(e, from, to)| e == end && now >= from && now < to)
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
        #[cfg(test)]
        if let Some(b) = self.trace {
            let rel = |t: u64| (t as i64 - b as i64) as f64 / 1e6;
            if end == 0 && data.is_some() {
                let (s, e) = data.unwrap_or_default();
                eprintln!(
                    "{:+9.1} S> [{s},{e}) att {attempt} lost {lost} arr {:+.1} draw {} burst {burst}",
                    rel(now),
                    rel(arrival),
                    below(w[0], 1_000_000)
                );
            } else if end == 1 {
                eprintln!(
                    "{:+9.1} C> {} cum {} sacks {:?} tsecr {:?} lost {lost} arr {:+.1}",
                    rel(now),
                    if data.is_some() { "data" } else { "ack " },
                    ack.cum,
                    ack.sacks,
                    ack.tsecr,
                    rel(arrival)
                );
            }
        }
        let _ = attempt;
        if data.is_some() {
            // tcp_event_data_sent (tcp_output.c:161-177): Cubic's CA_EVENT_TX_START when nothing was in flight.
            let snd = &mut self.ends[end].snd;
            if snd.in_flight() == 0 {
                snd.cubic_tx_start(now);
            }
            snd.lsndtime_ns = now;
        }
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

    /// tcp_data_snd_check then tcp_ack_snd_check: new data as the window allows, then an acknowledgement still owed.
    fn push(&mut self, e: usize, now: u64) {
        if self.ends[e].snd.snd_nxt < self.ends[e].snd.write_seq {
            self.write_xmit(e, now, false);
        }
        if self.ends[e].rcv.ack_now {
            self.transmit(e, None, 0, now);
        }
    }

    /// tcp_write_xmit (tcp_output.c:2700-2827): new segments while cwnd allows (one past it for the probe), then the
    /// window's bookkeeping, PRR's count and the probe. Returns how many it sent.
    fn write_xmit(&mut self, e: usize, now: u64, probe: bool) -> u64 {
        let mut sent = 0;
        loop {
            let s = &self.ends[e].snd;
            if s.snd_nxt >= s.write_seq {
                break;
            }
            if s.in_flight() >= s.cwnd && !(probe && sent == 0) {
                break;
            }
            // tcp_event_new_data_sent (:65-85): the first segment out, or one sent while the probe is pending,
            // rearms the RTO.
            let prior_packets = s.segs.len();
            self.send_new(e, now);
            let s = &mut self.ends[e].snd;
            if prior_packets == 0 || matches!(s.timer, Some((_, TimerKind::Probe))) {
                s.rearm_rto(now);
            }
            sent += 1;
            if probe {
                break;
            }
        }
        let probes_on = self.probes_on();
        let s = &mut self.ends[e].snd;
        let is_cwnd_limited = s.in_flight() >= s.cwnd;
        if sent > 0 || is_cwnd_limited {
            s.cwnd_validate(is_cwnd_limited);
        }
        if sent > 0 {
            if s.in_cwnd_reduction() {
                s.prr_out += sent;
            }
            if !probe && probes_on {
                s.schedule_probe(now, false);
            }
        }
        sent
    }

    fn send_new(&mut self, e: usize, now: u64) {
        let s = &mut self.ends[e].snd;
        let start = s.snd_nxt;
        let end = (start + self.mss).min(s.write_seq);
        let (arrival, lost) = self.transmit(e, Some((start, end)), 1, now);
        let c = &mut self.ends[e].counters;
        c.segments += 1;
        c.first_lost += u64::from(lost);
        let s = &mut self.ends[e].snd;
        s.snd_nxt = end;
        s.segs.push_back(Seg {
            start,
            end,
            xmit_ns: now,
            first_arrival_ns: arrival,
            sacked: false,
            lost: false,
            sretrans: false,
            ever: false,
            tsorted: true,
            attempts: 1,
            cause: None,
            received: false,
        });
    }

    /// __tcp_retransmit_skb (tcp_output.c:3281-3405), and with `marked`, tcp_retransmit_skb (:3407-3430): the resend
    /// in flight (TCPCB_SACKED_RETRANS), retrans_stamp at the first, undo_retrans. The probe's resend is unmarked.
    fn retransmit(&mut self, e: usize, i: usize, marked: bool, now: u64) {
        let (start, end, attempt, cause) = {
            let g = &mut self.ends[e].snd.segs[i];
            g.attempts += 1;
            (g.start, g.end, g.attempts, g.cause)
        };
        self.transmit(e, Some((start, end)), attempt, now);
        let s = &mut self.ends[e].snd;
        s.total_retrans += 1;
        let g = &mut s.segs[i];
        g.xmit_ns = now;
        g.ever = true;
        g.tsorted = true;
        if marked {
            if g.sretrans {
                s.retrans_leak += 1;
            }
            g.sretrans = true;
            s.retrans_stamp.get_or_insert(ts_of(now));
            s.undo_retrans = s.undo_retrans.max(0) + 1;
        }
        let c = &mut self.ends[e].counters;
        match cause {
            Some(Repair::Rack) => c.repairs_rack += 1,
            Some(Repair::Probe) => c.repairs_probe += 1,
            Some(Repair::Timeout) | None => c.repairs_timeout += 1,
        }
    }

    /// tcp_xmit_retransmit_queue (tcp_output.c:3437-3509): the lost segments not yet resent, in order, while cwnd
    /// allows and resends in flight are fewer than the lost; resending the head restarts the RTO unless the reorder
    /// timer is pending.
    fn xmit_retransmit_queue(&mut self, e: usize, now: u64) {
        let mut rearm = false;
        for i in 0..self.ends[e].snd.segs.len() {
            let s = &self.ends[e].snd;
            if s.cwnd <= s.in_flight() || s.retrans_out() >= s.lost_out() {
                break;
            }
            let g = &s.segs[i];
            if !g.lost || g.sacked || g.sretrans {
                continue;
            }
            self.retransmit(e, i, true, now);
            let s = &mut self.ends[e].snd;
            if s.in_cwnd_reduction() {
                s.prr_out += 1;
            }
            if i == 0 && !matches!(s.timer, Some((_, TimerKind::Reorder))) {
                rearm = true;
            }
        }
        if rearm {
            let s = &mut self.ends[e].snd;
            s.timer = Some((now + s.rto_ns(), TimerKind::Rto));
        }
    }

    /// tcp_xmit_recovery (tcp_input.c:3806-3820).
    fn xmit_recovery(&mut self, e: usize, rexmit: Rexmit, now: u64) {
        match rexmit {
            Rexmit::None => return,
            Rexmit::New => {
                self.write_xmit(e, now, false);
                let s = &mut self.ends[e].snd;
                if s.snd_nxt > s.high_seq {
                    return;
                }
                s.frto = false;
            }
            Rexmit::Lost => {}
        }
        self.xmit_retransmit_queue(e, now);
    }

    /// tcp_ack (tcp_input.c:3839-4033) for the ACK in an arrival at `e`, for the data `e` sent.
    fn on_ack(&mut self, e: usize, pk: &Packet, now: u64) {
        let ack = &pk.ack;
        let tsecr = ack.tsecr.filter(|_| self.timestamps);
        let probes_on = self.probes_on();
        let s = &mut self.ends[e].snd;
        s.rcv_tsecr = tsecr;
        let prior_snd_una = s.snd_una;
        // An ACK below snd_una (old_ack) or above snd_nxt never arrives on these first-in, first-out paths.
        if ack.cum < prior_snd_una || ack.cum > s.snd_nxt {
            return;
        }
        let prior_packets = s.segs.len();
        let prior_lost = s.lost;
        let mut flag = 0;
        let mut rexmit = Rexmit::None;
        let mut st = SackState {
            reord: s.snd_nxt,
            first_sackt: None,
            last_sackt: None,
            sack_delivered: 0,
        };
        if ack.cum > prior_snd_una {
            flag |= FLAG_SND_UNA_ADVANCED;
            s.retransmits = 0;
        }
        let prior_fack = s.highest_sack_seq();
        // tcp_ack_update_window (:3625-3655) under a constant advertised window: an update when snd_una advances or
        // the segment is newer than the last update (tcp_may_update_window, :3559-3565).
        if pk.data.is_some() {
            flag |= FLAG_DATA;
        }
        if ack.cum > prior_snd_una || s.snd_wl1.is_none_or(|w| pk.seq > w) {
            flag |= FLAG_WIN_UPDATE;
            s.snd_wl1 = Some(pk.seq);
        }
        s.snd_una = ack.cum;
        if !ack.sacks.is_empty() {
            flag |= s.sacktag(ack, prior_snd_una, &mut st, now);
        }
        if prior_packets == 0 {
            // no_queue: only a D-SACK may undo something.
            if flag & FLAG_DSACKING_ACK != 0 {
                s.fastretrans_alert(prior_snd_una, 0, &mut flag, &mut rexmit, now);
            }
            s.process_tlp_ack(ack.cum, flag);
            return;
        }
        let (f, cum_delivered) = s.clean_rtx_queue(prior_fack, &st, now);
        flag |= f;
        s.process_tlp_ack(ack.cum, flag);
        // tcp_ack_is_dubious (:3509-3513).
        if flag & FLAG_NOT_DUP == 0 || flag & FLAG_CA_ALERT != 0 || s.state != State::Open {
            let num_dupack =
                u64::from(flag & (FLAG_SND_UNA_ADVANCED | FLAG_NOT_DUP | FLAG_DSACKING_ACK) == 0);
            s.fastretrans_alert(prior_snd_una, num_dupack, &mut flag, &mut rexmit, now);
        }
        // tcp_set_xmit_timer (:3229-3233).
        if flag & FLAG_SET_XMIT_TIMER != 0 && !(probes_on && s.schedule_probe(now, true)) {
            s.rearm_rto(now);
        }
        let delivered = cum_delivered + st.sack_delivered;
        let losses = s.lost - prior_lost;
        #[cfg(test)]
        if let Some(b) = self.trace.filter(|_| e == 0) {
            eprintln!(
                "{:+9.1} S< cum {} una {}->{} flag {:#x} state {:?} cwnd {} ssth {} inflight {} lost {} retr {} sacked {} \
                 undo {:?}/{} rstamp {:?} frto {} srtt {} rto {} backoff {} rexmit {:?} deliv {}",
                (now as i64 - b as i64) as f64 / 1e6,
                ack.cum,
                prior_snd_una,
                s.snd_una,
                flag,
                s.state,
                s.cwnd,
                s.ssthresh,
                s.in_flight(),
                s.lost_out(),
                s.retrans_out(),
                s.sacked_out(),
                s.undo_marker,
                s.undo_retrans,
                s.retrans_stamp,
                s.frto,
                s.rtt.srtt_us() / 1000,
                s.rto_ns() / 1_000_000,
                s.backoff,
                rexmit,
                cum_delivered + st.sack_delivered
            );
        }
        s.cong_control(delivered, losses, flag, now);
        self.xmit_recovery(e, rexmit, now);
    }

    /// tcp_retransmit_timer (tcp_timer.c:506-660).
    fn on_rto(&mut self, e: usize, now: u64) {
        #[cfg(test)]
        if let Some(b) = self.trace.filter(|_| e == 0) {
            let s = &self.ends[0].snd;
            eprintln!(
                "{:+9.1} S RTO state {:?} cwnd {} inflight {} segs {}",
                (now as i64 - b as i64) as f64 / 1e6,
                s.state,
                s.cwnd,
                s.in_flight(),
                s.segs.len()
            );
        }
        let doubling = self.doubling_on();
        let s = &mut self.ends[e].snd;
        if s.segs.is_empty() {
            return;
        }
        s.tlp_high_seq = None;
        s.enter_loss(now);
        s.retransmits += 1;
        self.retransmit(e, 0, true, now);
        // out_reset_timer: a thin stream (fewer than 4 segments out, past its initial slow start) keeps a linear
        // timeout for its first 6 retries when the setting is on; else the RTO doubles.
        let s = &mut self.ends[e].snd;
        let thin = s.thin_lt
            && s.segs.len() < THIN_PACKETS
            && s.ssthresh < INFINITE_SSTHRESH
            && s.retransmits <= THIN_LINEAR_RETRIES;
        if thin {
            s.backoff = 0;
        } else if doubling {
            s.backoff += 1;
        }
        s.timer = Some((now + s.rto_ns(), TimerKind::Rto));
        let b = s.backoff;
        let c = &mut self.ends[e].counters;
        c.max_backoff = c.max_backoff.max(b);
    }

    /// tcp_send_loss_probe (tcp_output.c:2900-2961). The probe timer counts as pending while it runs, as icsk_pending
    /// does in the kernel.
    fn on_probe(&mut self, e: usize, now: u64) {
        #[cfg(test)]
        if let Some(b) = self.trace.filter(|_| e == 0) {
            eprintln!("{:+9.1} S PROBE", (now as i64 - b as i64) as f64 / 1e6);
        }
        let s = &mut self.ends[e].snd;
        s.timer = Some((now, TimerKind::Probe));
        if s.tlp_high_seq.is_some() || s.segs.is_empty() {
            s.rearm_rto(now);
            return;
        }
        s.tlp_retrans = false;
        if s.snd_nxt < s.write_seq {
            if self.write_xmit(e, now, true) == 0 {
                self.ends[e].snd.rearm_rto(now);
                return;
            }
        } else {
            let i = self.ends[e].snd.segs.len() - 1;
            self.ends[e].snd.segs[i].cause = Some(Repair::Probe);
            self.retransmit(e, i, false, now);
            self.ends[e].snd.tlp_retrans = true;
        }
        let s = &mut self.ends[e].snd;
        s.tlp_high_seq = Some(s.snd_nxt);
        s.timer = None;
        s.rearm_rto(now);
    }

    /// tcp_rack_reo_timeout (tcp_recovery.c:149-167). The reorder timer counts as pending while it runs, so resending
    /// the head does not restart the RTO and the rearm counts from the head.
    fn on_reorder(&mut self, e: usize, now: u64) {
        #[cfg(test)]
        if let Some(b) = self.trace.filter(|_| e == 0) {
            eprintln!("{:+9.1} S REO", (now as i64 - b as i64) as f64 / 1e6);
        }
        self.ends[e].snd.timer = Some((now, TimerKind::Reorder));
        if self.ends[e].snd.reo_timeout_mark(now) {
            self.xmit_retransmit_queue(e, now);
        }
        let s = &mut self.ends[e].snd;
        if !matches!(s.timer, Some((_, TimerKind::Rto))) {
            s.rearm_rto(now);
        }
    }

    /// Data [s, x) arriving at `q` from the other end (tcp_data_queue and the ACK it calls for).
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
        match r.data_queue(s, x) {
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
    fn probes_on(&self) -> bool {
        self.variant != Variant::ConstantRepair
    }
    #[cfg(not(test))]
    fn probes_on(&self) -> bool {
        true
    }
    #[cfg(test)]
    fn doubling_on(&self) -> bool {
        self.variant != Variant::NoDoubling && self.variant != Variant::ConstantRepair
    }
    #[cfg(not(test))]
    fn doubling_on(&self) -> bool {
        true
    }

    /// Plants one of the bites: the constant-repair bite gets E15's fixed timeout, max(200 ms, 2 x the round trip).
    #[cfg(test)]
    fn with_variant(mut self, v: Variant, profile: &Profile) -> TcpModel {
        self.variant = v;
        for end in &mut self.ends {
            end.snd.variant = v;
            end.rcv.no_dsack = v == Variant::NoDsack;
        }
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
