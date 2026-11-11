//! npt-l1: L1, the seeded link simulator on simulated time (plan E15 and §6.4; ADR 0002:515-521; ADR 0001 part 13).
//!
//! It implements npt-link's [`Link`] and [`Listener`], so the room, the clients and the probe run on it unchanged.
//!
//! # The model
//! - **Profiles.** Each link has a [`Profile`]: per direction, half its round trip plus a uniform jitter of ± half its
//!   jitter, and its loss, drawn for each direction on its own (ADR 0001 part 13; OQ-0001-16, answered by the Advisor,
//!   2026-11-03). [`Profile::p0`] to [`Profile::p4`] and [`Profile::b1`] are part 13's named set.
//! - **Fates are drawn per (seed, link, direction, flow, packet index).** Each link direction has four flows: its
//!   datagrams, its control stream's packets, its full-state stream's packets and its acknowledgements (these last
//!   since U4c: the advisor's Q15 ruling, so that datagram fates are what they were before). Each flow has its own
//!   ChaCha8 stream (plan §4.5's generator; ADR 0004 part 8), and every packet takes the same 16 words from it, used or
//!   not. So traffic on one link or flow never moves another's fates, and both command-timing options see the same
//!   losses (plan §4.7, npt-lab). A send refused as too large takes no index.
//! - **Datagrams** arrive at their send time plus their own delay, so the jitter reorders them where it is wider than
//!   their spacing. **The reorder step** (R-5; part 13, the advisor's reading, 2026-11-06) adds what the jitter cannot:
//!   a datagram it draws (1 % of a flow's datagrams) is overtaken by the next datagram of its flow: it is held until
//!   1 ns after that one arrives, if that one is sent before it would have arrived and is not lost. A held datagram is
//!   only ever later than its draw, so no delay falls below the profile's floor; but it may arrive above the profile's
//!   ceiling, by up to one spacing: at P2 to P4 at 30 Hz about 1 % of datagrams do (P2's highest is about 93 ms against
//!   its 60 ms ceiling; the advisor's U2 check). Where the spacing is wider than the delay (P0; one datagram per tick at
//!   P1) the step finds nothing to swap, and the run's measured share says so. The step is on in every run.
//! - **Bursts** (B1): every packet sent inside a burst is lost, in both directions, acknowledgements included.
//! - **Packets and acknowledgements** (U4c: the advisor's C1', advisor-u3q.md:160-173, and Q15 ruling). Every packet of
//!   a link and direction, datagrams included, takes the next number of that direction's one packet-number space:
//!   datagrams are ack-eliciting (RFC 9221 §5.2), so they count as later packets in the loss rules below. The receiving
//!   end acknowledges each packet in the instant it arrives; the acknowledgement travels the other way with a delay
//!   drawn as that direction's, is lost only inside a burst, and covers every packet of the space that had arrived
//!   before it was sent (QUIC's ACK ranges), so a later acknowledgement stands in for a lost one.
//! - **Reliable streams** (Control, FullState) are QUIC's as quinn runs them (C1' (a) to (e); RFC 9002 §5 and §6):
//!   - (a) a message is cut into packets of at most 1,200 B (QUIC's smallest maximum datagram size, RFC 9000 §14;
//!     quinn's starting MTU), and each packet draws its own fate;
//!   - (b) a packet is declared lost once a later packet of its link and direction is acknowledged and either the
//!     largest acknowledged number is at least 3 above its own (kPacketThreshold) or max(9/8 × max(smoothed RTT,
//!     latest RTT), 1 ms) has passed since it was sent (kTimeThreshold, kGranularity: RFC 9002 §6.1.1, §6.1.2;
//!     quinn-proto config/transport.rs:382-383); its stream data is resent then;
//!   - (c) else at the probe timeout (PTO): smoothed RTT + max(4 × RTT variance, 1 ms) + 25 ms (max_ack_delay's
//!     default, RFC 9000 §18.2), counted from the last ack-eliciting packet sent, doubled at each consecutive expiry
//!     and reset by an acknowledgement that acknowledges anything new (RFC 9002 §6.2.1; quinn-proto
//!     connection/paths.rs:331-333, connection/mod.rs:1831-1858). An expiry resends, as its probe, the data of the
//!     oldest unacknowledged stream packet;
//!   - **the starting state** (RFC 9002 §6.2.2, §5.3): before the first RTT sample the estimate is a smoothed RTT of
//!     333 ms (kInitialRtt) and a variance of 166.5 ms, so the first PTO is 333 + 4 × 166.5 + 25 = 1,024 ms; the
//!     first sample sets both;
//!   - (d) every resend is a new packet, with its own number and its own fate, and a burst takes it;
//!   - (e) each stream hands out its messages in order: a message once all its packets have arrived and every earlier
//!     message of its stream has been handed out, stamped with that instant.
//! - **What L1 leaves out of QUIC** (C1' (g)): there is no congestion window, so after a burst L1 sends at once, while
//!   quinn, its window full of lost packets, waits for its next backed-off PTO (about 0.15 to 0.6 s after a burst at P2
//!   by quinn's formula, the phase deciding). Also left out: a second probe per PTO (quinn sends two; RFC 9002 §6.2.4
//!   allows up to two); a probe while only datagrams are in flight (quinn sends a PING; L1 arms no PTO then), or when
//!   every stream packet in flight carries data already acknowledged through another copy (L1 counts the expiry and
//!   sends nothing; quinn sends a PING); the acknowledgement's own packet number, its delay and its frequency (L1
//!   acknowledges every packet at once); and any undo of a spurious loss. And the handshake: in QUIC the handshake's
//!   acknowledgements give RTT samples before any application data (RFC 9002 §6.2.1 arms no PTO for application data
//!   until the handshake is confirmed), while an L1 link starts from the starting state, so on a quiet link a stream
//!   message lost before the first sample waits for the 1,024 ms PTO (L-c9).
//! - **RTT** (the advisor's C2, advisor-u2.md:79-83) is each end's RFC 9002 §5.3 smoothed estimate from its
//!   acknowledgements, never the profile's value (plan §4.7, npt-lab). A sample comes only from the acknowledgement of
//!   the newest packet of its link and direction to arrive, when that acknowledgement newly acknowledges it at the
//!   sender (§5.1's "largest acknowledged newly acknowledged"): an overtaken packet gives none, and neither does an
//!   acknowledgement that arrives after a newer one has covered its packet. A stream packet samples from its own
//!   arrival, never from its message's in-order hand-out. A sample is the acknowledgement's arrival minus the packet's
//!   send. [`Link::rtt_ns`] is `None` until the first sample.
//! - **Events.** Arrivals, acknowledgements and the loss and probe timers are internal events. Every call that takes
//!   `now_ns` first applies every event of the whole net due at or before it, each at its own time, in (time, fate-log
//!   index) order, and only then does its own work: at one instant, internal events come before application sends, and
//!   polling more or less often changes nothing. [`L1Net::next_event_ns`] says when the next one falls.
//! - **Delivery.** `poll_recv(now)` returns what has arrived by `now` in (arrival, fate-log index) order, never in the
//!   instant it was sent (P0 included: plan §4.7's step order), stamped with its arrival time.
//! - **The fate log** ([`FateLog`]) records every packet and every acknowledgement sent: its link, direction, flow,
//!   index in its flow, send time, size and fate (delivered at a time, overtaken by the reorder step or not, resends;
//!   lost; lost in a burst), and for a stream packet its packet number and piece. The same seed, profile and sends give
//!   the same log, byte for byte: PM-0001-3's identity. Its canonical form keeps v1's datagram entries unchanged.
//! - **Not built:** a rate limit and duplication. No L1 row of the netcode proof test needs them (the throttle and the
//!   bottleneck are L2's, PM-0001-23 and -24).
//!
//! Real time never enters: the clock ban holds (`clippy.toml`), apart from the one planted defect that reads the clock
//! on purpose (`bites` builds only).

use npt_link::{Channel, Link, LinkState, Listener, Recv, SendResult};
use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};
use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, VecDeque};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::rc::Rc;

#[cfg(test)]
mod tests;

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;

/// The 64-bit draws every packet and every acknowledgement takes from its flow's stream, used or not. A datagram uses
/// words 0 to 2 (word 3 held its acknowledgement's delay until U4c gave acknowledgements a flow of their own), a stream
/// packet words 0 and 1, an acknowledgement word 1.
const WORDS: usize = 16;
const W_LOSS: usize = 0;
const W_JITTER: usize = 1;
const W_REORDER: usize = 2;
/// Six words: twelve 32-bit uniforms for the normal-jitter bite.
#[cfg(feature = "bites")]
const W_NORMAL: usize = 4;

/// The fixed part of the 32-byte ChaCha8 key; the run's seed, little-endian, fills the first 8 bytes.
const KEY_TAG: &[u8; 24] = b"npt-l1 fate draws, v1\0\0\0";

/// A stream packet's largest payload (C1' (a)): QUIC's smallest maximum datagram size (RFC 9000 §14), quinn's starting
/// MTU.
const MAX_PACKET: usize = 1_200;
/// RFC 9002 §6.1.1's kPacketThreshold.
const PACKET_THRESHOLD: u64 = 3;
/// RFC 9002 §6.1.2's kGranularity.
const GRANULARITY_NS: u64 = MS;
/// RFC 9002 §6.2.2's kInitialRtt.
const INITIAL_RTT_NS: u64 = 333 * MS;
/// RFC 9000 §18.2's default max_ack_delay, which RFC 9002 §6.2.1's PTO adds.
const MAX_ACK_DELAY_NS: u64 = 25 * MS;

/// Whether a planted defect is on (`bites` builds only); in a measured build every defect is off.
#[cfg(feature = "bites")]
macro_rules! bitten {
    ($bite:expr, $name:ident) => {
        $bite == Some(Bite::$name)
    };
}
#[cfg(not(feature = "bites"))]
macro_rules! bitten {
    ($bite:expr, $name:ident) => {
        false
    };
}

/// The explicit reorder step per flow and direction (R-5; ADR 0001 part 13).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReorderStep {
    /// The share of a flow's datagrams drawn to be overtaken by their successor, in permille.
    pub permille: u32,
}

impl ReorderStep {
    pub const OFF: ReorderStep = ReorderStep { permille: 0 };
    /// Part 13's step: 1 % of adjacent datagram pairs swapped (the advisor's reading, 2026-11-06).
    pub const PART_13: ReorderStep = ReorderStep { permille: 10 };
}

/// A network profile (ADR 0001 part 13). Latency and jitter are the round trip's; each direction carries half the
/// latency, a uniform jitter of ± half the jitter, and the loss.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    /// `P0` to `P4` or `B1`, as the results log's header names it.
    pub name: &'static str,
    pub rtt_ns: u64,
    /// The round trip's ± jitter: P2's ±20 ms is 20,000,000.
    pub jitter_ns: u64,
    /// Each direction's loss, in parts per million.
    pub loss_ppm: u32,
    /// Bursts of total loss, as (start, length) on the simulated clock.
    pub bursts_ns: Vec<(u64, u64)>,
    pub reorder: ReorderStep,
}

impl Profile {
    fn named(name: &'static str, rtt_ms: u64, jitter_ms: u64, loss_ppm: u32) -> Profile {
        Profile {
            name,
            rtt_ns: rtt_ms * MS,
            jitter_ns: jitter_ms * MS,
            loss_ppm,
            bursts_ns: Vec::new(),
            reorder: ReorderStep::PART_13,
        }
    }

    /// 0 ms, no jitter, no loss.
    pub fn p0() -> Profile {
        Profile::named("P0", 0, 0, 0)
    }
    /// 50 ms, ±10 ms, 1 %.
    pub fn p1() -> Profile {
        Profile::named("P1", 50, 10, 10_000)
    }
    /// 100 ms, ±20 ms, 2 %: the proof test's "100 ms of simulated latency" (OQ-0001-3).
    pub fn p2() -> Profile {
        Profile::named("P2", 100, 20, 20_000)
    }
    /// 150 ms, ±30 ms, 5 %.
    pub fn p3() -> Profile {
        Profile::named("P3", 150, 30, 50_000)
    }
    /// 200 ms, ±40 ms, 5 %.
    pub fn p4() -> Profile {
        Profile::named("P4", 200, 40, 50_000)
    }
    /// P2 plus a 1 s burst of total loss at each start (part 13 has one; PM-0001-18 runs ten).
    pub fn b1(burst_starts_ns: &[u64]) -> Profile {
        let mut p = Profile::named("B1", 100, 20, 20_000);
        p.bursts_ns = burst_starts_ns.iter().map(|&s| (s, SEC)).collect();
        p
    }
    /// `P0` to `P4` by name; B1 needs its burst times, so it comes from [`Profile::b1`].
    pub fn by_name(name: &str) -> Option<Profile> {
        match name {
            "P0" => Some(Profile::p0()),
            "P1" => Some(Profile::p1()),
            "P2" => Some(Profile::p2()),
            "P3" => Some(Profile::p3()),
            "P4" => Some(Profile::p4()),
            _ => None,
        }
    }
    /// Each direction's mean delay: half the round trip.
    pub fn one_way_ns(&self) -> u64 {
        self.rtt_ns / 2
    }
    /// Each direction's jitter: ± half the round trip's.
    pub fn half_jitter_ns(&self) -> u64 {
        self.jitter_ns / 2
    }
    fn in_burst(&self, t: u64) -> bool {
        self.bursts_ns
            .iter()
            .any(|&(start, len)| t >= start && t - start < len)
    }
}

/// A uniform integer in `[0, n)` from one 64-bit draw, by multiply and shift (bias below n / 2^64).
fn below(word: u64, n: u64) -> u64 {
    ((u128::from(word) * u128::from(n)) >> 64) as u64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    /// Client to server.
    Up = 0,
    /// Server to client.
    Down = 1,
}

const CLIENT: usize = 0;
const SERVER: usize = 1;

fn send_dir(end: usize) -> Dir {
    if end == CLIENT { Dir::Up } else { Dir::Down }
}

fn recv_dir(end: usize) -> Dir {
    if end == CLIENT { Dir::Down } else { Dir::Up }
}

/// The end that receives a direction's packets.
fn receiver(dir: Dir) -> usize {
    if dir == Dir::Up { SERVER } else { CLIENT }
}

fn other(dir: Dir) -> Dir {
    if dir == Dir::Up { Dir::Down } else { Dir::Up }
}

fn channel_code(c: Channel) -> u8 {
    match c {
        Channel::Datagram => 0,
        Channel::Control => 1,
        Channel::FullState => 2,
    }
}

/// The acknowledgements' flow code, after the three channels'.
const ACK_CODE: u8 = 3;

/// A reliable channel's stream slot: Control 0, FullState 1 (a datagram never reaches a stream).
fn stream_slot(c: Channel) -> usize {
    usize::from(c == Channel::FullState)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Delivered { arrival_ns: u64 },
    Lost,
    Burst,
}

/// What a fate-log entry carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Carries {
    Datagram,
    /// One piece of a stream message: the packet's number in its space, the message's index in its stream, the piece's
    /// index in the message and the message's piece count.
    Stream {
        channel: Channel,
        pn: u64,
        message: u64,
        part: u32,
        parts: u32,
    },
    /// An acknowledgement: the number of the packet whose arrival sent it (in the other direction's space) and the
    /// largest number received in that space by then.
    Ack {
        acked: u64,
        largest: u64,
    },
}

/// One packet's or acknowledgement's entry in the fate log.
#[derive(Debug, Clone)]
struct Fate {
    link: u32,
    /// The direction it travels.
    dir: Dir,
    carries: Carries,
    /// Its index in its flow.
    seq: u64,
    sent_ns: u64,
    len: u32,
    outcome: Outcome,
    overtaken: bool,
    /// A stream packet: how many sends of its piece came before it.
    retransmits: u32,
}

/// Every packet's and acknowledgement's fate, in send order (the fate-log index is the position).
#[derive(Debug, Clone, Default)]
pub struct FateLog {
    entries: Vec<Fate>,
}

/// The canonical form's first bytes: the format and its version (v2 since U4c).
const CANONICAL_TAG: &[u8] = b"npt-l1 fate log v2\n";

impl FateLog {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Feeds the log's canonical form to `f`: one call with `CANONICAL_TAG` (`npt-l1 fate log v2` and a newline), then
    /// one call per entry, in index order, every field fixed-width little-endian. Every entry starts with the same 40
    /// bytes: link u32; direction, flow code (0 datagram, 1 control, 2 full state, 3 acknowledgement), kind (0
    /// delivered, 1 lost, 2 lost in a burst) and overtaken u8; index in its flow, send time and arrival u64, arrival 0
    /// unless delivered; size and resends u32. A datagram's entry is exactly those 40 bytes, v1's form, so SHA-256 of
    /// (v1's tag `npt-l1 fate log v1` and a newline, then a log's datagram entries) is what v1 gave the same datagrams
    /// (L-c8). A stream packet's entry adds its packet number and its message's index u64, then its piece's index and
    /// its message's piece count u32 (64 bytes; resends counts its piece's earlier sends). An acknowledgement's adds
    /// the number of the packet whose arrival sent it and the largest number received in that space by then, u64 (56
    /// bytes; size and resends 0). Two logs are the same exactly when these bytes are.
    pub fn for_each_canonical(&self, mut f: impl FnMut(&[u8])) {
        f(CANONICAL_TAG);
        let mut b = [0u8; 64];
        for e in &self.entries {
            let (kind, arrival) = match e.outcome {
                Outcome::Delivered { arrival_ns } => (0u8, arrival_ns),
                Outcome::Lost => (1, 0),
                Outcome::Burst => (2, 0),
            };
            let code = match e.carries {
                Carries::Datagram => 0,
                Carries::Stream { channel, .. } => channel_code(channel),
                Carries::Ack { .. } => ACK_CODE,
            };
            b[0..4].copy_from_slice(&e.link.to_le_bytes());
            b[4] = e.dir as u8;
            b[5] = code;
            b[6] = kind;
            b[7] = u8::from(e.overtaken);
            b[8..16].copy_from_slice(&e.seq.to_le_bytes());
            b[16..24].copy_from_slice(&e.sent_ns.to_le_bytes());
            b[24..32].copy_from_slice(&arrival.to_le_bytes());
            b[32..36].copy_from_slice(&e.len.to_le_bytes());
            b[36..40].copy_from_slice(&e.retransmits.to_le_bytes());
            let n = match e.carries {
                Carries::Datagram => 40,
                Carries::Stream {
                    pn,
                    message,
                    part,
                    parts,
                    ..
                } => {
                    b[40..48].copy_from_slice(&pn.to_le_bytes());
                    b[48..56].copy_from_slice(&message.to_le_bytes());
                    b[56..60].copy_from_slice(&part.to_le_bytes());
                    b[60..64].copy_from_slice(&parts.to_le_bytes());
                    64
                }
                Carries::Ack { acked, largest } => {
                    b[40..48].copy_from_slice(&acked.to_le_bytes());
                    b[48..56].copy_from_slice(&largest.to_le_bytes());
                    56
                }
            };
            f(&b[..n]);
        }
    }

    /// Per direction (`[0]` up, client to server; `[1]` down): (datagrams delivered, of them reordered). A datagram is
    /// reordered when it arrives after a later-sent datagram of its flow (RFC 4737's reordered packet), in the order
    /// `poll_recv` hands them out.
    pub fn reorder_counts(&self) -> [(u64, u64); 2] {
        // (link, direction) -> (arrival, index, seq) of each delivered datagram.
        type Delivered = ((u32, Dir), (u64, usize, u64));
        let mut flows: Vec<Delivered> = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| match e.outcome {
                Outcome::Delivered { arrival_ns } if e.carries == Carries::Datagram => {
                    Some(((e.link, e.dir), (arrival_ns, i, e.seq)))
                }
                _ => None,
            })
            .collect();
        flows.sort_unstable();
        let mut out = [(0u64, 0u64); 2];
        let mut current: Option<(u32, Dir)> = None;
        let mut highest = 0u64;
        for (flow, (_, _, seq)) in flows {
            if current != Some(flow) {
                current = Some(flow);
                highest = seq;
                out[flow.1 as usize].0 += 1;
                continue;
            }
            let o = &mut out[flow.1 as usize];
            o.0 += 1;
            if seq < highest {
                o.1 += 1;
            } else {
                highest = seq;
            }
        }
        out
    }
}

/// One piece of a stream message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Piece {
    channel: Channel,
    message: u64,
    part: u32,
}

#[derive(Debug)]
enum Body {
    Datagram(Vec<u8>),
    Piece(Piece),
}

/// A packet on its way to the receiving end.
#[derive(Debug)]
struct Packet {
    pn: u64,
    /// Its packet-number space: 0, or 1 for a datagram under the separate-spaces bite.
    space: usize,
    /// Its fate-log index.
    index: u64,
    sent_ns: u64,
    body: Body,
}

/// An acknowledgement on its way back: what its space's receiving end held when it was sent.
#[derive(Debug, Clone, Copy)]
struct AckInfo {
    /// It covers every arrival up to this one, counted per space from 0.
    ordinal: u64,
    /// The largest packet number received in the space by then.
    largest: u64,
    /// Its packet was the newest to arrive (it raised `largest`), so it may give an RTT sample (C2).
    eligible: bool,
    /// The packet whose arrival sent it.
    acked: u64,
    acked_sent_ns: u64,
}

#[derive(Debug)]
enum What {
    Arrive(Packet),
    /// The reorder step's held datagram reaches its own arrival, overtaken by nothing.
    HeldDue,
    Ack(AckInfo),
    /// A space's loss-detection timer: the time threshold's or the PTO.
    Timer {
        generation: u64,
    },
}

/// An internal event, applied in (time, fate-log index, kind) order; the rest only makes the order total.
#[derive(Debug)]
struct Event {
    at_ns: u64,
    /// An arrival's packet, an acknowledgement's own entry, a timer's packet (the one it counts from).
    key: u64,
    order: u8,
    link: u32,
    /// An arrival's direction; an acknowledgement's and a timer's are those of the packets they are about.
    dir: Dir,
    space: usize,
    tie: u64,
    what: What,
}

impl Event {
    fn rank(&self) -> (u64, u64, u8, u32, Dir, usize, u64) {
        (
            self.at_ns, self.key, self.order, self.link, self.dir, self.space, self.tie,
        )
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.rank() == other.rank()
    }
}
impl Eq for Event {}
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

/// Something an end can poll, ordered by (arrival, fate-log index, place among those one arrival released).
#[derive(Debug)]
struct Ready {
    arrival_ns: u64,
    index: u64,
    sub: u32,
    channel: Channel,
    /// Its send time: it is never handed out in that instant.
    gate_ns: u64,
    bytes: Vec<u8>,
}

impl Ready {
    fn rank(&self) -> (u64, u64, u32) {
        (self.arrival_ns, self.index, self.sub)
    }
}

impl PartialEq for Ready {
    fn eq(&self, other: &Self) -> bool {
        self.rank() == other.rank()
    }
}
impl Eq for Ready {}
impl PartialOrd for Ready {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Ready {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

/// One flow's draws.
struct Flow {
    rng: ChaCha8Rng,
    next_seq: u64,
}

/// A packet sent and neither acknowledged nor declared lost.
#[derive(Debug, Clone, Copy)]
struct Sent {
    sent_ns: u64,
    index: u64,
    piece: Option<Piece>,
    /// It still carries its piece's data for a resend (a probe takes it away: quinn's retransmits).
    retransmittable: bool,
}

/// One packet-number space of a link direction: the sender's side and the receiver's.
#[derive(Debug, Default)]
struct Space {
    next_pn: u64,
    unacked: BTreeMap<u64, Sent>,
    /// How many of `unacked` are retransmittable.
    retransmittable: u64,
    /// The receiver's arrivals, (ordinal, packet number), not yet covered by an acknowledgement that reached the sender.
    arrived: VecDeque<(u64, u64)>,
    received: u64,
    largest_received: Option<u64>,
    largest_acked: Option<u64>,
    /// (time, fate-log index) of the last ack-eliciting packet sent.
    last_sent: Option<(u64, u64)>,
    /// The time threshold's next instant, with its packet's fate-log index.
    loss_time: Option<(u64, u64)>,
    /// The armed timer, (instant, key), and its generation: an event of an older generation is stale.
    timer: Option<(u64, u64)>,
    generation: u64,
}

/// RFC 9002 §5.3's estimator, in integer nanoseconds, with no acknowledgement delay, from §6.2.2's starting state.
#[derive(Debug, Clone, Copy)]
struct Rtt {
    smoothed: Option<u64>,
    var: u64,
    latest: u64,
}

impl Rtt {
    const START: Rtt = Rtt {
        smoothed: None,
        var: INITIAL_RTT_NS / 2,
        latest: 0,
    };

    fn sample(&mut self, s: u64) {
        self.latest = s;
        match self.smoothed {
            None => {
                self.smoothed = Some(s);
                self.var = s / 2;
            }
            Some(sm) => {
                self.var = (3 * self.var + sm.abs_diff(s)) / 4;
                self.smoothed = Some((7 * sm + s) / 8);
            }
        }
    }

    fn smoothed_ns(&self) -> u64 {
        self.smoothed.unwrap_or(INITIAL_RTT_NS)
    }

    /// RFC 9002 §6.2.1's PTO before backoff.
    fn pto_ns(&self) -> u64 {
        self.smoothed_ns() + (4 * self.var).max(GRANULARITY_NS) + MAX_ACK_DELAY_NS
    }

    /// RFC 9002 §6.1.2's time threshold.
    fn loss_delay_ns(&self) -> u64 {
        (self.smoothed_ns().max(self.latest) * 9 / 8).max(GRANULARITY_NS)
    }
}

/// A sender's piece not yet acknowledged.
#[derive(Debug, Clone, Copy)]
struct Outstanding {
    sends: u32,
    len: u32,
    parts: u32,
}

/// A message on its way through its stream.
#[derive(Debug)]
struct Message {
    bytes: Vec<u8>,
    sent_ns: u64,
    arrived: Vec<bool>,
    missing: u32,
}

/// One reliable stream of a link direction.
#[derive(Debug, Default)]
struct Stream {
    next_message: u64,
    /// The receiver: the messages not yet handed out, from message `base` on.
    base: u64,
    messages: VecDeque<Message>,
    /// The sender: its pieces not yet acknowledged.
    outstanding: BTreeMap<(u64, u32), Outstanding>,
    /// The stream-sample-in-order bite: acknowledgements held until their message is handed out.
    #[cfg(feature = "bites")]
    deferred: Vec<(u64, AckInfo)>,
}

/// What the unit tests plant: a send whose loss draw says lost.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Forced {
    Datagram { dir: Dir, seq: u64 },
    Piece { dir: Dir, piece: Piece, send: u32 },
}

/// What the unit tests read: every loss declared, every PTO and every RTT sample.
#[cfg(test)]
#[derive(Debug, Default)]
struct Trace {
    declared: Vec<Declared>,
    ptos: Vec<Pto>,
    samples: Vec<Sample>,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy)]
struct Declared {
    link: u32,
    dir: Dir,
    pn: u64,
    at_ns: u64,
    by_count: bool,
    delay_ns: u64,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy)]
struct Pto {
    link: u32,
    dir: Dir,
    at_ns: u64,
    /// The consecutive expiries, this one counted.
    count: u32,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy)]
struct Sample {
    link: u32,
    dir: Dir,
    at_ns: u64,
    sample_ns: u64,
    pn: u64,
}

#[cfg(feature = "bites")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bite {
    RttEachWay,
    LossPerRoundTrip,
    NormalJitter,
    ClockSeed,
    NoPtoDoubling,
    AcksInBursts,
    EveryPacketSampled,
    StreamSampleInOrder,
    OneDrawPerMessage,
    SeparateSpaces,
    EventsAtPolls,
    SendsBeforeEvents,
}

/// The planted defects of PM-0001-3's L1 half (its row's bites; marks.json's ids).
#[cfg(feature = "bites")]
const BITE_IDS: [(&str, Bite); 4] = [
    ("PM-0001-3/rtt-each-way", Bite::RttEachWay),
    ("PM-0001-3/loss-per-round-trip", Bite::LossPerRoundTrip),
    ("PM-0001-3/normal-jitter", Bite::NormalJitter),
    ("PM-0001-3/clock-seed", Bite::ClockSeed),
];

/// The link model's own planted defects (U4c: C1' and C2, each failing its L-c test). No mark names them, so only this
/// crate's unit tests choose them (`L1Net::with_model_bite`); the planted code is in every `bites` build, as the marks'
/// bites are.
#[cfg(all(test, feature = "bites"))]
const MODEL_BITE_IDS: [(&str, Bite); 8] = [
    ("npt-l1/no-pto-doubling", Bite::NoPtoDoubling),
    ("npt-l1/acks-in-bursts", Bite::AcksInBursts),
    ("npt-l1/every-packet-sampled", Bite::EveryPacketSampled),
    ("npt-l1/stream-sample-in-order", Bite::StreamSampleInOrder),
    ("npt-l1/one-draw-per-message", Bite::OneDrawPerMessage),
    ("npt-l1/separate-spaces", Bite::SeparateSpaces),
    ("npt-l1/events-at-polls", Bite::EventsAtPolls),
    ("npt-l1/sends-before-events", Bite::SendsBeforeEvents),
];

/// The bite PM-0001-3/clock-seed itself: the seed read from the wall clock, the defect the fate-log identity catches.
#[cfg(feature = "bites")]
#[allow(clippy::disallowed_methods, clippy::disallowed_types)]
fn clock_seed() -> u64 {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as u64)
}

/// What a link's draws turn into: the profile's numbers and the planted defect, if any.
#[derive(Debug, Clone, Copy)]
struct Model {
    one_way: u64,
    half_jitter: u64,
    loss_ppm: u32,
    reorder_permille: u32,
    #[cfg(feature = "bites")]
    bite: Option<Bite>,
}

impl Model {
    /// One direction's delay from a packet's draws: d − a + uniform [0, 2a].
    fn delay(&self, w: &[u64; WORDS], word: usize) -> u64 {
        #[allow(unused_mut)]
        let mut d = self.one_way;
        let a = self.half_jitter;
        #[cfg(feature = "bites")]
        {
            if self.bite == Some(Bite::RttEachWay) {
                // The defect: the full round trip applied each way.
                d *= 2;
            }
            if self.bite == Some(Bite::NormalJitter) && word == W_JITTER {
                // The defect: a normal jitter (twelve uniforms summed) in place of the uniform, with its 0.1st and
                // 99.9th percentiles near d -+ a, so only the shape check can catch it.
                let mut sum: i128 = 0;
                for x in &w[W_NORMAL..W_NORMAL + 6] {
                    sum += i128::from(*x as u32) + i128::from(*x >> 32);
                }
                let z = sum - 6 * (1i128 << 32); // mean 0, standard deviation 2^32
                let sigma = i128::from(a) * 100 / 309;
                let off = (z * sigma) >> 32;
                return (i128::from(d) + off).max(0) as u64;
            }
        }
        (d + below(w[word], 2 * a + 1)).saturating_sub(a)
    }
}

/// A simulated link: two ends, two directions.
struct LinkSim {
    profile: Profile,
    model: Model,
    created_ns: u64,
    addr: SocketAddr,
    max_datagram: usize,
    /// `flows[direction][flow code]`: datagrams, control packets, full-state packets, acknowledgements.
    flows: [[Flow; 4]; 2],
    /// `held[direction]`: the reorder step's held datagram, with its own arrival.
    held: [Option<(u64, Packet)>; 2],
    /// `spaces[direction][space]`: space 0 holds every packet; space 1 only datagrams, under the separate-spaces bite.
    spaces: [[Space; 2]; 2],
    /// `rtt[direction]`: the estimate of the end that sends that direction's packets (RFC 9002: one per path).
    rtt: [Rtt; 2],
    /// `pto_count[direction]`: consecutive PTO expiries, as RFC 9002 counts them per path.
    pto_count: [u32; 2],
    /// `streams[direction][slot]`: Control, FullState.
    streams: [[Stream; 2]; 2],
    /// `inbox[direction]`: what has arrived that way and waits for the receiving end's poll.
    inbox: [BinaryHeap<Reverse<Ready>>; 2],
    /// `open[end]`: client end 0, server end 1.
    open: [bool; 2],
    /// The loss-per-round-trip bite's shared draws, by index in flow.
    #[cfg(feature = "bites")]
    round_trip: [ChaCha8Rng; 3],
    #[cfg(test)]
    forced: Vec<Forced>,
}

struct Net {
    key: [u8; 32],
    links: Vec<LinkSim>,
    log: FateLog,
    accepts: VecDeque<u32>,
    events: BinaryHeap<Reverse<Event>>,
    #[cfg(feature = "bites")]
    bite: Option<Bite>,
    #[cfg(test)]
    trace: Trace,
    /// Plants for links not yet connected, (link, plant): each moves to its link at the connect.
    #[cfg(test)]
    forced: Vec<(u32, Forced)>,
}

fn stream_rng(key: &[u8; 32], stream: u64) -> ChaCha8Rng {
    let mut rng = ChaCha8Rng::from_seed(*key);
    rng.set_stream(stream);
    rng
}

fn flow_stream(link: u32, dir: Dir, code: u8) -> u64 {
    (u64::from(link) << 16) | ((dir as u64) << 8) | u64::from(code)
}

fn draw(rng: &mut ChaCha8Rng) -> [u64; WORDS] {
    let mut w = [0u64; WORDS];
    for x in &mut w {
        *x = rng.next_u64();
    }
    w
}

fn take(flow: &mut Flow) -> (u64, [u64; WORDS]) {
    let seq = flow.next_seq;
    flow.next_seq += 1;
    (seq, draw(&mut flow.rng))
}

impl LinkSim {
    fn up(&self) -> bool {
        self.open[CLIENT] && self.open[SERVER]
    }

    /// The defect-free loss draw is the packet's own; the loss-per-round-trip bite draws once per index in flow for
    /// both directions and gives the loss to one of them.
    fn lost(&mut self, dir: Dir, channel: Channel, seq: u64, w: &[u64; WORDS]) -> bool {
        #[cfg(feature = "bites")]
        if self.model.bite == Some(Bite::LossPerRoundTrip) {
            let rng = &mut self.round_trip[usize::from(channel_code(channel))];
            rng.set_word_pos(u128::from(seq) * 4);
            let (u, v) = (rng.next_u64(), rng.next_u64());
            return below(u, 1_000_000) < u64::from(self.model.loss_ppm) && (v & 1) == dir as u64;
        }
        let _ = (dir, channel, seq);
        below(w[W_LOSS], 1_000_000) < u64::from(self.model.loss_ppm)
    }

    /// The PTO with its backoff, from the direction's estimate.
    fn pto_ns(&self, d: usize) -> u64 {
        let base = self.rtt[d].pto_ns();
        if bitten!(self.model.bite, NoPtoDoubling) {
            return base;
        }
        base.saturating_mul(1u64.checked_shl(self.pto_count[d]).unwrap_or(u64::MAX))
    }
}

/// Which kind of call enters the net: a poll, or any other call that takes `now_ns`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Call {
    Send,
    Poll,
}

impl Net {
    /// What a call does before its own work: apply every event due at or before `now_ns` (the events-at-polls bite: only
    /// in polls, each at the poll's instant; the sends-before-events bite: in a send, only those before its instant).
    fn enter(&mut self, now_ns: u64, call: Call) {
        if call == Call::Send {
            if bitten!(self.bite, EventsAtPolls) {
                return;
            }
            if bitten!(self.bite, SendsBeforeEvents) {
                if now_ns > 0 {
                    self.advance(now_ns - 1);
                }
                return;
            }
        }
        self.advance(now_ns);
    }

    /// What a send does after its own work (the sends-before-events bite: the events of its instant).
    fn leave(&mut self, now_ns: u64) {
        if bitten!(self.bite, SendsBeforeEvents) {
            self.advance(now_ns);
        }
    }

    fn advance(&mut self, now_ns: u64) {
        while self.events.peek().is_some_and(|e| e.0.at_ns <= now_ns) {
            let Some(Reverse(e)) = self.events.pop() else {
                break;
            };
            let t = if bitten!(self.bite, EventsAtPolls) {
                now_ns
            } else {
                e.at_ns
            };
            self.apply(e, t);
        }
    }

    /// Whether an event can no longer do anything: a timer re-armed since, or a held datagram already released.
    fn stale(&self, e: &Event) -> bool {
        let l = &self.links[e.link as usize];
        match e.what {
            What::Timer { generation } => {
                let sp = &l.spaces[e.dir as usize][e.space];
                sp.generation != generation || sp.timer.is_none()
            }
            What::HeldDue => !l.held[e.dir as usize]
                .as_ref()
                .is_some_and(|(_, p)| p.index == e.key),
            _ => false,
        }
    }

    fn apply(&mut self, e: Event, t: u64) {
        if self.stale(&e) {
            return;
        }
        let (link, dir, space) = (e.link as usize, e.dir, e.space);
        match e.what {
            What::Arrive(p) => self.arrive(link, dir, p, t),
            What::HeldDue => {
                if let Some((_, p)) = self.links[link].held[dir as usize].take() {
                    self.arrive(link, dir, p, t);
                }
            }
            What::Ack(a) => self.on_ack(link, dir, space, a, t),
            What::Timer { .. } => self.on_timer(link, dir, space, t),
        }
    }

    /// The earliest instant at which a call can change anything: the next internal event, or the next instant at which
    /// a poll hands something out.
    fn next_event(&mut self) -> Option<u64> {
        while self.events.peek().is_some_and(|e| self.stale(&e.0)) {
            self.events.pop();
        }
        let event = self.events.peek().map(|e| e.0.at_ns);
        let ready = self
            .links
            .iter()
            .flat_map(|l| l.inbox.iter())
            .filter_map(|q| q.peek())
            .map(|r| r.0.arrival_ns.max(r.0.gate_ns.saturating_add(1)))
            .min();
        match (event, ready) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }

    fn connect(&mut self, profile: Profile, max_datagram: usize, now_ns: u64) -> u32 {
        self.enter(now_ns, Call::Send);
        let id = u32::try_from(self.links.len()).expect("fewer than 2^32 links");
        let key = self.key;
        let flows = [Dir::Up, Dir::Down].map(|dir| {
            [0u8, 1, 2, ACK_CODE].map(|code| Flow {
                rng: stream_rng(&key, flow_stream(id, dir, code)),
                next_seq: 0,
            })
        });
        let model = Model {
            one_way: profile.one_way_ns(),
            half_jitter: profile.half_jitter_ns(),
            loss_ppm: profile.loss_ppm,
            reorder_permille: profile.reorder.permille,
            #[cfg(feature = "bites")]
            bite: self.bite,
        };
        let [a, b, c, d] = id.to_be_bytes();
        let addr = SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(10, b, c, d),
            40_000 + u16::from(a),
        ));
        #[cfg(test)]
        let forced: Vec<Forced> = {
            let (mine, rest) = std::mem::take(&mut self.forced)
                .into_iter()
                .partition(|(l, _)| *l == id);
            self.forced = rest;
            mine.into_iter().map(|(_, f)| f).collect()
        };
        self.links.push(LinkSim {
            profile,
            model,
            created_ns: now_ns,
            addr,
            max_datagram,
            flows,
            held: [None, None],
            spaces: Default::default(),
            rtt: [Rtt::START; 2],
            pto_count: [0; 2],
            streams: Default::default(),
            inbox: [BinaryHeap::new(), BinaryHeap::new()],
            open: [true, true],
            #[cfg(feature = "bites")]
            round_trip: [0u64, 1, 2]
                .map(|ch| stream_rng(&key, (u64::from(id) << 16) | (0xFF << 8) | ch)),
            #[cfg(test)]
            forced,
        });
        self.accepts.push_back(id);
        id
    }

    fn send_datagram(&mut self, link: u32, end: usize, bytes: &[u8], now_ns: u64) -> SendResult {
        self.enter(now_ns, Call::Send);
        let r = self.datagram(link as usize, end, bytes, now_ns);
        if r == SendResult::Sent && self.links[link as usize].up() {
            let space = usize::from(bitten!(self.bite, SeparateSpaces));
            self.set_timer(link as usize, send_dir(end), space, now_ns);
        }
        self.leave(now_ns);
        r
    }

    fn datagram(&mut self, link: usize, end: usize, bytes: &[u8], now_ns: u64) -> SendResult {
        let Net {
            links, log, events, ..
        } = self;
        let l = &mut links[link];
        if !l.up() {
            return SendResult::Sent;
        }
        if bytes.len() > l.max_datagram {
            return SendResult::TooLarge;
        }
        let dir = send_dir(end);
        let d = dir as usize;
        let (seq, w) = take(&mut l.flows[d][0]);
        #[cfg(test)]
        let forced = l.forced.contains(&Forced::Datagram { dir, seq });
        #[cfg(not(test))]
        let forced = false;
        let outcome = if l.profile.in_burst(now_ns) {
            Outcome::Burst
        } else if forced || l.lost(dir, Channel::Datagram, seq, &w) {
            Outcome::Lost
        } else {
            Outcome::Delivered {
                arrival_ns: now_ns + l.model.delay(&w, W_JITTER),
            }
        };
        let index = log.entries.len() as u64;
        log.entries.push(Fate {
            link: link as u32,
            dir,
            carries: Carries::Datagram,
            seq,
            sent_ns: now_ns,
            len: u32::try_from(bytes.len()).unwrap_or(u32::MAX),
            outcome,
            overtaken: false,
            retransmits: 0,
        });
        let space = usize::from(bitten!(l.model.bite, SeparateSpaces));
        let sp = &mut l.spaces[d][space];
        let pn = sp.next_pn;
        sp.next_pn += 1;
        sp.unacked.insert(
            pn,
            Sent {
                sent_ns: now_ns,
                index,
                piece: None,
                retransmittable: false,
            },
        );
        sp.last_sent = Some((now_ns, index));
        // The reorder step: the flow's held datagram is overtaken by this one if this one is sent before the held one
        // would have arrived and is not lost; then it arrives 1 ns after this one. Either way it is released now.
        if let Some((own, held)) = l.held[d].take() {
            let mut at = own;
            if let Outcome::Delivered { arrival_ns } = outcome
                && now_ns < own
                && arrival_ns + 1 > own
            {
                at = arrival_ns + 1;
                let e = &mut log.entries[held.index as usize];
                e.outcome = Outcome::Delivered { arrival_ns: at };
                e.overtaken = true;
            }
            events.push(arrival(link, dir, at, held));
        }
        if let Outcome::Delivered { arrival_ns } = outcome {
            let p = Packet {
                pn,
                space,
                index,
                sent_ns: now_ns,
                body: Body::Datagram(bytes.to_vec()),
            };
            if below(w[W_REORDER], 1000) < u64::from(l.model.reorder_permille) {
                l.held[d] = Some((arrival_ns, p));
                events.push(Reverse(Event {
                    at_ns: arrival_ns,
                    key: index,
                    order: 1,
                    link: link as u32,
                    dir,
                    space,
                    tie: 0,
                    what: What::HeldDue,
                }));
            } else {
                events.push(arrival(link, dir, arrival_ns, p));
            }
        }
        SendResult::Sent
    }

    fn send_stream(
        &mut self,
        link: u32,
        end: usize,
        channel: Channel,
        bytes: &[u8],
        now_ns: u64,
    ) -> SendResult {
        self.enter(now_ns, Call::Send);
        let link = link as usize;
        if self.links[link].up() {
            let dir = send_dir(end);
            let parts = bytes.len().div_ceil(MAX_PACKET).max(1);
            let parts32 = u32::try_from(parts).expect("fewer than 2^32 pieces");
            let s = &mut self.links[link].streams[dir as usize][stream_slot(channel)];
            let message = s.next_message;
            s.next_message += 1;
            for part in 0..parts32 {
                let at = part as usize * MAX_PACKET;
                let len = (bytes.len() - at.min(bytes.len())).min(MAX_PACKET);
                s.outstanding.insert(
                    (message, part),
                    Outstanding {
                        sends: 0,
                        len: len as u32,
                        parts: parts32,
                    },
                );
            }
            s.messages.push_back(Message {
                bytes: bytes.to_vec(),
                sent_ns: now_ns,
                arrived: vec![false; parts],
                missing: parts32,
            });
            let mut first = None;
            for part in 0..parts32 {
                let piece = Piece {
                    channel,
                    message,
                    part,
                };
                // The one-draw-per-message bite: every piece takes the first piece's loss draw.
                let shared = if bitten!(self.bite, OneDrawPerMessage) {
                    first
                } else {
                    None
                };
                let lost = self.send_piece(link, dir, piece, now_ns, shared);
                if part == 0 {
                    first = Some(lost);
                }
            }
            self.set_timer(link, dir, 0, now_ns);
        }
        self.leave(now_ns);
        SendResult::Sent
    }

    /// Sends one piece of a stream message as a new packet, with its own number and fate; returns whether its loss draw
    /// (or `shared`, the one-draw-per-message bite's) said lost. A piece already acknowledged is not sent.
    fn send_piece(
        &mut self,
        link: usize,
        dir: Dir,
        piece: Piece,
        now_ns: u64,
        shared: Option<bool>,
    ) -> bool {
        let Net {
            links, log, events, ..
        } = self;
        let l = &mut links[link];
        let d = dir as usize;
        let key = (piece.message, piece.part);
        let Some(o) = l.streams[d][stream_slot(piece.channel)]
            .outstanding
            .get_mut(&key)
        else {
            return false;
        };
        let retransmits = o.sends;
        o.sends += 1;
        let (len, parts) = (o.len, o.parts);
        let (seq, w) = take(&mut l.flows[d][usize::from(channel_code(piece.channel))]);
        #[cfg(test)]
        let forced = l.forced.contains(&Forced::Piece {
            dir,
            piece,
            send: retransmits,
        });
        #[cfg(not(test))]
        let forced = false;
        let lost = match shared {
            Some(x) => x,
            None => forced || l.lost(dir, piece.channel, seq, &w),
        };
        let outcome = if l.profile.in_burst(now_ns) {
            Outcome::Burst
        } else if lost {
            Outcome::Lost
        } else {
            Outcome::Delivered {
                arrival_ns: now_ns + l.model.delay(&w, W_JITTER),
            }
        };
        let sp = &mut l.spaces[d][0];
        let pn = sp.next_pn;
        sp.next_pn += 1;
        let index = log.entries.len() as u64;
        log.entries.push(Fate {
            link: link as u32,
            dir,
            carries: Carries::Stream {
                channel: piece.channel,
                pn,
                message: piece.message,
                part: piece.part,
                parts,
            },
            seq,
            sent_ns: now_ns,
            len,
            outcome,
            overtaken: false,
            retransmits,
        });
        sp.unacked.insert(
            pn,
            Sent {
                sent_ns: now_ns,
                index,
                piece: Some(piece),
                retransmittable: true,
            },
        );
        sp.retransmittable += 1;
        sp.last_sent = Some((now_ns, index));
        if let Outcome::Delivered { arrival_ns } = outcome {
            events.push(arrival(
                link,
                dir,
                arrival_ns,
                Packet {
                    pn,
                    space: 0,
                    index,
                    sent_ns: now_ns,
                    body: Body::Piece(piece),
                },
            ));
        }
        lost
    }

    /// A packet reaches its receiving end at `t`: it is handed to the end's inbox (a stream piece through its stream's
    /// in-order reassembly) and acknowledged at once.
    fn arrive(&mut self, link: usize, dir: Dir, p: Packet, t: u64) {
        let l = &mut self.links[link];
        if !l.open[receiver(dir)] {
            return;
        }
        let d = dir as usize;
        let sp = &mut l.spaces[d][p.space];
        let ordinal = sp.received;
        sp.received += 1;
        let eligible = sp.largest_received.is_none_or(|lr| p.pn > lr);
        if eligible {
            sp.largest_received = Some(p.pn);
        }
        sp.arrived.push_back((ordinal, p.pn));
        let ack = AckInfo {
            ordinal,
            largest: sp.largest_received.unwrap_or(p.pn),
            eligible,
            acked: p.pn,
            acked_sent_ns: p.sent_ns,
        };
        #[allow(unused_mut)]
        let mut own = Some(ack);
        #[cfg(feature = "bites")]
        let mut released: Vec<AckInfo> = Vec::new();
        match p.body {
            Body::Datagram(bytes) => l.inbox[d].push(Reverse(Ready {
                arrival_ns: t,
                index: p.index,
                sub: 0,
                channel: Channel::Datagram,
                gate_ns: p.sent_ns,
                bytes,
            })),
            Body::Piece(piece) => {
                let s = &mut l.streams[d][stream_slot(piece.channel)];
                if piece.message >= s.base {
                    let m = &mut s.messages[(piece.message - s.base) as usize];
                    let part = piece.part as usize;
                    if !m.arrived[part] {
                        m.arrived[part] = true;
                        m.missing -= 1;
                    }
                }
                let mut sub = 0;
                while s.messages.front().is_some_and(|m| m.missing == 0) {
                    let Some(m) = s.messages.pop_front() else {
                        break;
                    };
                    s.base += 1;
                    l.inbox[d].push(Reverse(Ready {
                        arrival_ns: t,
                        index: p.index,
                        sub,
                        channel: piece.channel,
                        gate_ns: m.sent_ns,
                        bytes: m.bytes,
                    }));
                    sub += 1;
                }
                #[cfg(feature = "bites")]
                if l.model.bite == Some(Bite::StreamSampleInOrder) {
                    // The defect: a stream packet is acknowledged at its message's in-order hand-out, not its arrival.
                    let base = s.base;
                    let (go, wait): (Vec<_>, Vec<_>) =
                        s.deferred.drain(..).partition(|(m, _)| *m < base);
                    s.deferred = wait;
                    released = go.into_iter().map(|(_, a)| a).collect();
                    if piece.message >= base {
                        s.deferred.push((piece.message, ack));
                        own = None;
                    }
                }
            }
        }
        if !l.up() {
            return;
        }
        #[cfg(feature = "bites")]
        for a in released {
            self.send_ack(link, dir, p.space, a, t);
        }
        if let Some(a) = own {
            self.send_ack(link, dir, p.space, a, t);
        }
    }

    /// The receiving end of `dir` sends an acknowledgement back at `t`, from the acknowledgements' own flow.
    fn send_ack(&mut self, link: usize, dir: Dir, space: usize, a: AckInfo, t: u64) {
        let Net {
            links, log, events, ..
        } = self;
        let l = &mut links[link];
        let back = other(dir);
        let (seq, w) = take(&mut l.flows[back as usize][usize::from(ACK_CODE)]);
        let lost = l.profile.in_burst(t);
        // The acks-in-bursts bite: an acknowledgement sent inside a burst arrives anyway.
        #[cfg(feature = "bites")]
        let lost = lost && l.model.bite != Some(Bite::AcksInBursts);
        let outcome = if lost {
            Outcome::Burst
        } else {
            Outcome::Delivered {
                arrival_ns: t + l.model.delay(&w, W_JITTER),
            }
        };
        let index = log.entries.len() as u64;
        log.entries.push(Fate {
            link: link as u32,
            dir: back,
            carries: Carries::Ack {
                acked: a.acked,
                largest: a.largest,
            },
            seq,
            sent_ns: t,
            len: 0,
            outcome,
            overtaken: false,
            retransmits: 0,
        });
        if let Outcome::Delivered { arrival_ns } = outcome {
            events.push(Reverse(Event {
                at_ns: arrival_ns,
                key: index,
                order: 2,
                link: link as u32,
                dir,
                space,
                tie: 0,
                what: What::Ack(a),
            }));
        }
    }

    /// An acknowledgement reaches the end that sent `dir`'s packets (RFC 9002 §A.7's OnAckReceived).
    fn on_ack(&mut self, link: usize, dir: Dir, space: usize, a: AckInfo, t: u64) {
        let d = dir as usize;
        let l = &mut self.links[link];
        if !l.up() {
            return;
        }
        let every_sampled = bitten!(l.model.bite, EveryPacketSampled);
        let LinkSim {
            spaces,
            streams,
            rtt,
            pto_count,
            ..
        } = l;
        let sp = &mut spaces[d][space];
        sp.largest_acked = Some(sp.largest_acked.map_or(a.largest, |x| x.max(a.largest)));
        let mut newly = false;
        let mut largest_sent = None;
        while let Some(&(ordinal, pn)) = sp.arrived.front() {
            if ordinal > a.ordinal {
                break;
            }
            sp.arrived.pop_front();
            let Some(s) = sp.unacked.remove(&pn) else {
                continue;
            };
            newly = true;
            if s.retransmittable {
                sp.retransmittable -= 1;
            }
            if pn == a.largest {
                largest_sent = Some(s.sent_ns);
            }
            if let Some(piece) = s.piece {
                streams[d][stream_slot(piece.channel)]
                    .outstanding
                    .remove(&(piece.message, piece.part));
            }
        }
        // C2: a sample only from the acknowledgement of the newest packet to arrive, when it newly acknowledges it (the
        // every-packet-sampled bite: from every acknowledgement's own packet).
        let sample = if every_sampled {
            Some((t - a.acked_sent_ns, a.acked))
        } else if a.eligible {
            largest_sent.map(|s| (t - s, a.largest))
        } else {
            None
        };
        if let Some((s, _)) = sample {
            rtt[d].sample(s);
        }
        if newly {
            pto_count[d] = 0;
        }
        #[cfg(test)]
        if let Some((sample_ns, pn)) = sample {
            self.trace.samples.push(Sample {
                link: link as u32,
                dir,
                at_ns: t,
                sample_ns,
                pn,
            });
        }
        if !newly {
            return;
        }
        self.detect_lost(link, dir, space, t);
        for s in 0..2 {
            self.set_timer(link, dir, s, t);
        }
    }

    /// RFC 9002 §A.10's DetectAndRemoveLostPackets: declares lost, in number order, every unacknowledged packet below
    /// the largest acknowledged that meets the packet or the time threshold, resends the stream data of each, and sets
    /// the time threshold's next instant from the first that meets neither.
    fn detect_lost(&mut self, link: usize, dir: Dir, space: usize, t: u64) {
        let d = dir as usize;
        #[cfg(test)]
        let trace = &mut self.trace;
        let l = &mut self.links[link];
        let delay = l.rtt[d].loss_delay_ns();
        let sp = &mut l.spaces[d][space];
        sp.loss_time = None;
        let Some(largest) = sp.largest_acked else {
            return;
        };
        let mut resend = Vec::new();
        while let Some((&pn, &s)) = sp.unacked.first_key_value() {
            if pn > largest {
                break;
            }
            let by_count = largest >= pn + PACKET_THRESHOLD;
            if !by_count && s.sent_ns + delay > t {
                sp.loss_time = Some((s.sent_ns + delay, s.index));
                break;
            }
            sp.unacked.remove(&pn);
            if s.retransmittable {
                sp.retransmittable -= 1;
                if let Some(piece) = s.piece {
                    resend.push(piece);
                }
            }
            #[cfg(test)]
            trace.declared.push(Declared {
                link: link as u32,
                dir,
                pn,
                at_ns: t,
                by_count,
                delay_ns: delay,
            });
        }
        for piece in resend {
            self.send_piece(link, dir, piece, t, None);
        }
    }

    /// RFC 9002 §A.8's SetLossDetectionTimer: the time threshold's instant if one is due, else the PTO while stream
    /// data is in flight, else none; an instant already passed fires at `t`.
    fn set_timer(&mut self, link: usize, dir: Dir, space: usize, t: u64) {
        let d = dir as usize;
        let l = &mut self.links[link];
        let pto = l.pto_ns(d);
        let sp = &mut l.spaces[d][space];
        let due = if let Some(lt) = sp.loss_time {
            Some(lt)
        } else if sp.retransmittable > 0 {
            sp.last_sent.map(|(at, key)| (at.saturating_add(pto), key))
        } else {
            None
        };
        let want = due.map(|(at, key)| (at.max(t), key));
        if want == sp.timer {
            return;
        }
        sp.timer = want;
        sp.generation += 1;
        if let Some((at_ns, key)) = want {
            self.events.push(Reverse(Event {
                at_ns,
                key,
                order: 3,
                link: link as u32,
                dir,
                space,
                tie: sp.generation,
                what: What::Timer {
                    generation: sp.generation,
                },
            }));
        }
    }

    /// RFC 9002 §A.9's OnLossDetectionTimeout: the time threshold's instant runs the loss rules; a PTO counts its
    /// expiry and resends, as its probe, the oldest unacknowledged packet's stream data.
    fn on_timer(&mut self, link: usize, dir: Dir, space: usize, t: u64) {
        let d = dir as usize;
        let l = &mut self.links[link];
        l.spaces[d][space].timer = None;
        if l.spaces[d][space].loss_time.is_some() {
            self.detect_lost(link, dir, space, t);
            self.set_timer(link, dir, space, t);
            return;
        }
        l.pto_count[d] = l.pto_count[d].saturating_add(1);
        #[cfg(test)]
        self.trace.ptos.push(Pto {
            link: link as u32,
            dir,
            at_ns: t,
            count: self.links[link].pto_count[d],
        });
        let LinkSim {
            spaces, streams, ..
        } = &mut self.links[link];
        let sp = &mut spaces[d][space];
        let mut probe = None;
        for s in sp.unacked.values_mut() {
            if !s.retransmittable {
                continue;
            }
            s.retransmittable = false;
            sp.retransmittable -= 1;
            if let Some(piece) = s.piece
                && streams[d][stream_slot(piece.channel)]
                    .outstanding
                    .contains_key(&(piece.message, piece.part))
            {
                probe = Some(piece);
                break;
            }
        }
        if let Some(piece) = probe {
            self.send_piece(link, dir, piece, t, None);
        }
        self.set_timer(link, dir, space, t);
    }

    fn poll(&mut self, link: u32, end: usize, now_ns: u64) -> Option<Recv> {
        self.enter(now_ns, Call::Poll);
        let inbox = &mut self.links[link as usize].inbox[recv_dir(end) as usize];
        let due = inbox
            .peek()
            .is_some_and(|r| r.0.arrival_ns <= now_ns && r.0.gate_ns < now_ns);
        if !due {
            return None;
        }
        inbox.pop().map(|Reverse(r)| Recv {
            channel: r.channel,
            bytes: r.bytes,
            arrival_ns: r.arrival_ns,
        })
    }

    /// Closes one end: what travels toward it is dropped, and the link sends nothing more, resends and
    /// acknowledgements included; what travels toward the open end still arrives.
    fn close(&mut self, link: u32, end: usize) {
        let l = &mut self.links[link as usize];
        l.open[end] = false;
        let toward = recv_dir(end) as usize;
        l.inbox[toward].clear();
        l.held[toward] = None;
        for sp in l.spaces.iter_mut().flatten() {
            sp.timer = None;
            sp.generation += 1;
        }
    }
}

/// An arrival event for packet `p` of `dir` at `at_ns`.
fn arrival(link: usize, dir: Dir, at_ns: u64, p: Packet) -> Reverse<Event> {
    Reverse(Event {
        at_ns,
        key: p.index,
        order: 0,
        link: link as u32,
        dir,
        space: p.space,
        tie: 0,
        what: What::Arrive(p),
    })
}

/// One end of a simulated link.
struct End {
    net: Rc<RefCell<Net>>,
    link: u32,
    end: usize,
}

impl Link for End {
    fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> SendResult {
        self.net
            .borrow_mut()
            .send_datagram(self.link, self.end, bytes, now_ns)
    }

    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult {
        assert!(
            channel != Channel::Datagram,
            "send_reliable on Channel::Datagram: datagrams go through send_unreliable"
        );
        self.net
            .borrow_mut()
            .send_stream(self.link, self.end, channel, bytes, now_ns)
    }

    fn poll_recv(&mut self, now_ns: u64) -> Option<Recv> {
        self.net.borrow_mut().poll(self.link, self.end, now_ns)
    }

    fn state(&self) -> LinkState {
        if self.net.borrow().links[self.link as usize].up() {
            LinkState::Open
        } else {
            LinkState::Closed
        }
    }

    fn rtt_ns(&self) -> Option<u64> {
        self.net.borrow().links[self.link as usize].rtt[send_dir(self.end) as usize].smoothed
    }

    fn max_datagram_size(&self) -> usize {
        self.net.borrow().links[self.link as usize].max_datagram
    }

    fn unsent_unreliable(&self) -> usize {
        0
    }
}

impl Drop for End {
    fn drop(&mut self) {
        self.net.borrow_mut().close(self.link, self.end);
    }
}

struct L1Listener {
    net: Rc<RefCell<Net>>,
}

impl Listener for L1Listener {
    fn poll_accept(&mut self, now_ns: u64) -> Option<(Box<dyn Link>, SocketAddr)> {
        let mut net = self.net.borrow_mut();
        net.enter(now_ns, Call::Send);
        let &id = net.accepts.front()?;
        let l = &net.links[id as usize];
        if l.created_ns >= now_ns {
            return None;
        }
        let addr = l.addr;
        net.accepts.pop_front();
        drop(net);
        let end: Box<dyn Link> = Box::new(End {
            net: Rc::clone(&self.net),
            link: id,
            end: SERVER,
        });
        Some((end, addr))
    }
}

/// The simulated network: its links, their flows and the fate log. One server per net: every link's server end is
/// accepted through [`L1Net::listener`].
pub struct L1Net {
    net: Rc<RefCell<Net>>,
}

impl L1Net {
    /// A net whose every draw comes from `seed` (the host passes it in: plan §4.5).
    pub fn new(seed: u64) -> L1Net {
        let mut key = [0u8; 32];
        key[..8].copy_from_slice(&seed.to_le_bytes());
        key[8..].copy_from_slice(KEY_TAG);
        L1Net {
            net: Rc::new(RefCell::new(Net {
                key,
                links: Vec::new(),
                log: FateLog::default(),
                accepts: VecDeque::new(),
                events: BinaryHeap::new(),
                #[cfg(feature = "bites")]
                bite: None,
                #[cfg(test)]
                trace: Trace::default(),
                #[cfg(test)]
                forced: Vec::new(),
            })),
        }
    }

    /// The bite ids this crate plants (`bites` builds only).
    #[cfg(feature = "bites")]
    pub const BITES: [&'static str; 4] =
        [BITE_IDS[0].0, BITE_IDS[1].0, BITE_IDS[2].0, BITE_IDS[3].0];

    /// A net with one planted defect, by its marks.json id (`bites` builds only); `None` for an id it does not plant.
    #[cfg(feature = "bites")]
    pub fn with_bite(seed: u64, bite: &str) -> Option<L1Net> {
        let (_, b) = BITE_IDS.iter().find(|(id, _)| *id == bite)?;
        let seed = if *b == Bite::ClockSeed {
            clock_seed()
        } else {
            seed
        };
        let net = L1Net::new(seed);
        net.net.borrow_mut().bite = Some(*b);
        Some(net)
    }

    /// A net with one of the link model's planted defects, by its `npt-l1/` id (the unit tests' only); `None` for an id
    /// it does not plant.
    #[cfg(all(test, feature = "bites"))]
    fn with_model_bite(seed: u64, bite: &str) -> Option<L1Net> {
        let (_, b) = MODEL_BITE_IDS.iter().find(|(id, _)| *id == bite)?;
        let net = L1Net::new(seed);
        net.net.borrow_mut().bite = Some(*b);
        Some(net)
    }

    /// Opens a link under `profile` and returns its client end; its server end waits for the listener from the next
    /// instant on.
    pub fn connect(
        &self,
        profile: Profile,
        max_datagram_size: usize,
        now_ns: u64,
    ) -> Box<dyn Link> {
        let link = self
            .net
            .borrow_mut()
            .connect(profile, max_datagram_size, now_ns);
        Box::new(End {
            net: Rc::clone(&self.net),
            link,
            end: CLIENT,
        })
    }

    /// The server's listener: it yields each link's server end, in connect order, with the client's address.
    pub fn listener(&self) -> Box<dyn Listener> {
        Box::new(L1Listener {
            net: Rc::clone(&self.net),
        })
    }

    /// When the next internal event falls (an arrival, an acknowledgement, a loss or probe timer), or the next instant
    /// at which a poll would hand something out, whichever is first; `None` when nothing is pending. An instant at or
    /// before the last call's `now_ns` means something waits to be polled now. A host that calls the net only at these
    /// instants and at its own sends sees what it would see polling every instant (L-c5).
    pub fn next_event_ns(&self) -> Option<u64> {
        self.net.borrow_mut().next_event()
    }

    /// Reads the fate log so far.
    pub fn with_fate_log<R>(&self, f: impl FnOnce(&FateLog) -> R) -> R {
        f(&self.net.borrow().log)
    }

    /// Plants a send's loss for a unit test (link by connect order), before or after its connect.
    #[cfg(test)]
    fn force(&self, link: u32, f: Forced) {
        let mut net = self.net.borrow_mut();
        match net.links.get_mut(link as usize) {
            Some(l) => l.forced.push(f),
            None => net.forced.push((link, f)),
        }
    }
}
