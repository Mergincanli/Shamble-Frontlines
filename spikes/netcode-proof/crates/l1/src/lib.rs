//! npt-l1: L1, the seeded link simulator on simulated time (plan E15 and §6.4; ADR 0002:515-521; ADR 0001 part 13).
//!
//! It implements npt-link's [`Link`] and [`Listener`], so the room, the clients and the probe run on it unchanged.
//!
//! # The model
//! - **Profiles.** Each link has a [`Profile`]: per direction, half its round trip plus a uniform jitter of ± half its
//!   jitter, and its loss, drawn for each direction on its own (ADR 0001 part 13; OQ-0001-16, answered by the Advisor,
//!   2026-11-03). [`Profile::p0`] to [`Profile::p4`] and [`Profile::b1`] are part 13's named set.
//! - **Fates are drawn per (seed, link, direction, channel, packet index).** Each such flow has its own ChaCha8 stream
//!   (plan §4.5's generator; ADR 0004 part 8), and every packet takes the same 16 words from it, used or not. So traffic
//!   on one link or channel never moves another's fates, and both command-timing options see the same losses (plan
//!   §4.7, npt-lab). A send refused as too large takes no index.
//! - **Datagrams** arrive at their send time plus their own delay, so the jitter reorders them where it is wider than
//!   their spacing. **The reorder step** (R-5; part 13, the advisor's reading, 2026-11-06) adds what the jitter cannot:
//!   a datagram it draws (1 % of a flow's datagrams) is overtaken by the next datagram of its flow: it is held until
//!   1 ns after that one arrives, if that one is sent before it would have arrived and is not lost. A held datagram is
//!   only ever later than its draw, so no delay falls below the profile's floor; where the spacing is wider than the
//!   delay (P0; P1 at 30 Hz) the step finds nothing to swap, and the run's measured share says so.
//! - **Bursts** (B1): every packet sent inside a burst is lost, in both directions.
//! - **Reliable streams** (Control, FullState) lose nothing and keep their order. A message the loss draw takes is
//!   delivered one retransmission timeout later, max(200 ms, 2 × the profile's round trip) (E15's rule for the
//!   WebSocket stream, proposed there); one sent inside a burst is retried every timeout until a retry falls outside it;
//!   a stream then delivers each message at the later of its own time and its predecessor's. This is L1's model of a
//!   stream; L2 measures the real ones.
//! - **Delivery.** `poll_recv(now)` returns what has arrived by `now` in (arrival, fate-log index) order, never in the
//!   instant it was sent (P0 included: plan §4.7's step order), stamped with its arrival time.
//! - **RTT** is each end's RFC 9002-style smoothed estimate, not the profile's value (plan §4.7, npt-lab): each
//!   delivered packet returns an implicit acknowledgement with a delay drawn for the other direction (never lost), whose
//!   sample, the acknowledgement's arrival minus the packet's send time, the end reads once its clock reaches it. A
//!   retransmitted message gives no sample (Karn's rule).
//! - **The fate log** ([`FateLog`]) records every packet: its link, direction, channel, index in its flow, send time,
//!   size and fate (delivered at a time, overtaken by the reorder step or not, retransmissions; lost; lost in a burst).
//!   The same seed, profile and sends give the same log, byte for byte: PM-0001-3's identity.
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
use std::collections::{BinaryHeap, VecDeque};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::rc::Rc;

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;

/// The 64-bit draws every packet takes from its flow's stream, used or not.
const WORDS: usize = 16;
const W_LOSS: usize = 0;
const W_JITTER: usize = 1;
const W_REORDER: usize = 2;
const W_ACK: usize = 3;
/// Six words: twelve 32-bit uniforms for the normal-jitter bite.
#[cfg(feature = "bites")]
const W_NORMAL: usize = 4;

/// The fixed part of the 32-byte ChaCha8 key; the run's seed, little-endian, fills the first 8 bytes.
const KEY_TAG: &[u8; 24] = b"npt-l1 fate draws, v1\0\0\0";

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
    /// A reliable message's retransmission timeout on L1: max(200 ms, 2 × the round trip) (E15, proposed there).
    pub fn rto_ns(&self) -> u64 {
        (200 * MS).max(2 * self.rtt_ns)
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

fn channel_code(c: Channel) -> u8 {
    match c {
        Channel::Datagram => 0,
        Channel::Control => 1,
        Channel::FullState => 2,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Delivered { arrival_ns: u64 },
    Lost,
    Burst,
}

/// One packet's entry in the fate log.
#[derive(Debug, Clone)]
struct Fate {
    link: u32,
    dir: Dir,
    channel: Channel,
    seq: u64,
    sent_ns: u64,
    len: u32,
    outcome: Outcome,
    overtaken: bool,
    retransmits: u32,
}

/// Every packet's fate, in send order (the fate-log index is the position).
#[derive(Debug, Clone, Default)]
pub struct FateLog {
    entries: Vec<Fate>,
}

/// The canonical form's first bytes: the format and its version.
const CANONICAL_TAG: &[u8] = b"npt-l1 fate log v1\n";

impl FateLog {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Feeds the log's canonical form to `f`: [`CANONICAL_TAG`], then 40 bytes per entry in index order, every field
    /// fixed-width little-endian (link u32; direction, channel, kind and overtaken u8; index in flow, send time and
    /// arrival u64, arrival 0 unless delivered; size and retransmissions u32). Two logs are the same exactly when these
    /// bytes are.
    pub fn for_each_canonical(&self, mut f: impl FnMut(&[u8])) {
        f(CANONICAL_TAG);
        let mut b = [0u8; 40];
        for e in &self.entries {
            let (kind, arrival) = match e.outcome {
                Outcome::Delivered { arrival_ns } => (0u8, arrival_ns),
                Outcome::Lost => (1, 0),
                Outcome::Burst => (2, 0),
            };
            b[0..4].copy_from_slice(&e.link.to_le_bytes());
            b[4] = e.dir as u8;
            b[5] = channel_code(e.channel);
            b[6] = kind;
            b[7] = u8::from(e.overtaken);
            b[8..16].copy_from_slice(&e.seq.to_le_bytes());
            b[16..24].copy_from_slice(&e.sent_ns.to_le_bytes());
            b[24..32].copy_from_slice(&arrival.to_le_bytes());
            b[32..36].copy_from_slice(&e.len.to_le_bytes());
            b[36..40].copy_from_slice(&e.retransmits.to_le_bytes());
            f(&b);
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
                Outcome::Delivered { arrival_ns } if e.channel == Channel::Datagram => {
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

/// A packet on its way, ordered by (arrival, fate-log index).
#[derive(Debug)]
struct InFlight {
    arrival_ns: u64,
    index: u64,
    channel: Channel,
    sent_ns: u64,
    bytes: Vec<u8>,
    /// The acknowledgement's return delay, drawn with the packet.
    ack_back_ns: u64,
}

impl PartialEq for InFlight {
    fn eq(&self, other: &Self) -> bool {
        (self.arrival_ns, self.index) == (other.arrival_ns, other.index)
    }
}
impl Eq for InFlight {}
impl PartialOrd for InFlight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for InFlight {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.arrival_ns, self.index).cmp(&(other.arrival_ns, other.index))
    }
}

/// One flow: a link's direction and channel.
struct Flow {
    rng: ChaCha8Rng,
    next_seq: u64,
    /// Reliable streams: the last message's delivery, which the next may not precede.
    last_arrival_ns: u64,
    /// The datagram flow: the last datagram, drawn by the reorder step, held out of flight until its successor is
    /// sent or its own arrival comes.
    pending: Option<InFlight>,
}

/// RFC 9002 §5.3's smoothed RTT, in integer nanoseconds, with no acknowledgement delay.
#[derive(Debug, Clone, Copy, Default)]
struct Rtt {
    smoothed: Option<u64>,
    var: u64,
}

impl Rtt {
    fn sample(&mut self, s: u64) {
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
}

/// An acknowledgement on its way back to the end that sent the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Ack {
    at_ns: u64,
    index: u64,
    sample_ns: u64,
}

#[cfg(feature = "bites")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bite {
    RttEachWay,
    LossPerRoundTrip,
    NormalJitter,
    ClockSeed,
}

/// The planted defects of PM-0001-3's L1 half (its row's bites; marks.json's ids).
#[cfg(feature = "bites")]
const BITE_IDS: [(&str, Bite); 4] = [
    ("PM-0001-3/rtt-each-way", Bite::RttEachWay),
    ("PM-0001-3/loss-per-round-trip", Bite::LossPerRoundTrip),
    ("PM-0001-3/normal-jitter", Bite::NormalJitter),
    ("PM-0001-3/clock-seed", Bite::ClockSeed),
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

/// A simulated link: two ends and six flows.
struct LinkSim {
    profile: Profile,
    model: Model,
    created_ns: u64,
    addr: SocketAddr,
    max_datagram: usize,
    /// `flows[direction][channel]`.
    flows: [[Flow; 3]; 2],
    /// `inbox[direction]`: packets travelling that way (up: read by the server end).
    inbox: [BinaryHeap<Reverse<InFlight>>; 2],
    /// `open[end]`: client end 0, server end 1.
    open: [bool; 2],
    rtt: [Rtt; 2],
    /// `acks[end]`: acknowledgements on their way back to that end.
    acks: [BinaryHeap<Reverse<Ack>>; 2],
    /// The loss-per-round-trip bite's shared draws, by index in flow.
    #[cfg(feature = "bites")]
    round_trip: [ChaCha8Rng; 3],
}

struct Net {
    key: [u8; 32],
    links: Vec<LinkSim>,
    log: FateLog,
    accepts: VecDeque<u32>,
    #[cfg(feature = "bites")]
    bite: Option<Bite>,
}

fn stream_rng(key: &[u8; 32], stream: u64) -> ChaCha8Rng {
    let mut rng = ChaCha8Rng::from_seed(*key);
    rng.set_stream(stream);
    rng
}

fn flow_stream(link: u32, dir: Dir, channel: Channel) -> u64 {
    (u64::from(link) << 16) | ((dir as u64) << 8) | u64::from(channel_code(channel))
}

fn draw(rng: &mut ChaCha8Rng) -> [u64; WORDS] {
    let mut w = [0u64; WORDS];
    for x in &mut w {
        *x = rng.next_u64();
    }
    w
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

    /// Puts a packet in flight toward its receiver and starts its acknowledgement back to its sender.
    fn launch(&mut self, dir: Dir, p: InFlight, retransmitted: bool) {
        if !retransmitted {
            let at_ns = p.arrival_ns + p.ack_back_ns;
            let sender = if dir == Dir::Up { CLIENT } else { SERVER };
            self.acks[sender].push(Reverse(Ack {
                at_ns,
                index: p.index,
                sample_ns: at_ns - p.sent_ns,
            }));
        }
        self.inbox[dir as usize].push(Reverse(p));
    }

    fn advance_acks(&mut self, end: usize, now_ns: u64) {
        while self.acks[end].peek().is_some_and(|a| a.0.at_ns <= now_ns) {
            if let Some(Reverse(a)) = self.acks[end].pop() {
                self.rtt[end].sample(a.sample_ns);
            }
        }
    }
}

impl Net {
    fn connect(&mut self, profile: Profile, max_datagram: usize, now_ns: u64) -> u32 {
        let id = u32::try_from(self.links.len()).expect("fewer than 2^32 links");
        let key = self.key;
        let flows = [Dir::Up, Dir::Down].map(|dir| {
            [Channel::Datagram, Channel::Control, Channel::FullState].map(|ch| Flow {
                rng: stream_rng(&key, flow_stream(id, dir, ch)),
                next_seq: 0,
                last_arrival_ns: 0,
                pending: None,
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
        self.links.push(LinkSim {
            profile,
            model,
            created_ns: now_ns,
            addr,
            max_datagram,
            flows,
            inbox: [BinaryHeap::new(), BinaryHeap::new()],
            open: [true, true],
            rtt: [Rtt::default(); 2],
            acks: [BinaryHeap::new(), BinaryHeap::new()],
            #[cfg(feature = "bites")]
            round_trip: [0u64, 1, 2]
                .map(|ch| stream_rng(&key, (u64::from(id) << 16) | (0xFF << 8) | ch)),
        });
        self.accepts.push_back(id);
        id
    }

    fn send_datagram(&mut self, link: u32, end: usize, bytes: &[u8], now_ns: u64) -> SendResult {
        let Net { links, log, .. } = self;
        let l = &mut links[link as usize];
        l.advance_acks(end, now_ns);
        if !l.up() {
            return SendResult::Sent;
        }
        if bytes.len() > l.max_datagram {
            return SendResult::TooLarge;
        }
        let dir = send_dir(end);
        let (seq, w) = {
            let flow = &mut l.flows[dir as usize][0];
            let seq = flow.next_seq;
            flow.next_seq += 1;
            (seq, draw(&mut flow.rng))
        };
        let outcome = if l.profile.in_burst(now_ns) {
            Outcome::Burst
        } else if l.lost(dir, Channel::Datagram, seq, &w) {
            Outcome::Lost
        } else {
            Outcome::Delivered {
                arrival_ns: now_ns + l.model.delay(&w, W_JITTER),
            }
        };
        let index = log.entries.len() as u64;
        log.entries.push(Fate {
            link,
            dir,
            channel: Channel::Datagram,
            seq,
            sent_ns: now_ns,
            len: u32::try_from(bytes.len()).unwrap_or(u32::MAX),
            outcome,
            overtaken: false,
            retransmits: 0,
        });
        // The reorder step: the flow's held datagram is overtaken by this one if this one is sent before the held one
        // would have arrived and is not lost; then it arrives 1 ns after this one. Either way it is released now.
        if let Some(mut held) = l.flows[dir as usize][0].pending.take() {
            if let Outcome::Delivered { arrival_ns } = outcome
                && now_ns < held.arrival_ns
                && arrival_ns + 1 > held.arrival_ns
            {
                held.arrival_ns = arrival_ns + 1;
                let e = &mut log.entries[held.index as usize];
                e.outcome = Outcome::Delivered {
                    arrival_ns: held.arrival_ns,
                };
                e.overtaken = true;
            }
            l.launch(dir, held, false);
        }
        if let Outcome::Delivered { arrival_ns } = outcome {
            let p = InFlight {
                arrival_ns,
                index,
                channel: Channel::Datagram,
                sent_ns: now_ns,
                bytes: bytes.to_vec(),
                ack_back_ns: l.model.delay(&w, W_ACK),
            };
            if below(w[W_REORDER], 1000) < u64::from(l.model.reorder_permille) {
                l.flows[dir as usize][0].pending = Some(p);
            } else {
                l.launch(dir, p, false);
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
        let Net { links, log, .. } = self;
        let l = &mut links[link as usize];
        l.advance_acks(end, now_ns);
        if !l.up() {
            return SendResult::Sent;
        }
        let dir = send_dir(end);
        let ch = usize::from(channel_code(channel));
        let (seq, w) = {
            let flow = &mut l.flows[dir as usize][ch];
            let seq = flow.next_seq;
            flow.next_seq += 1;
            (seq, draw(&mut flow.rng))
        };
        let rto = l.profile.rto_ns();
        let mut t = now_ns;
        let mut retransmits = 0u32;
        if l.profile.in_burst(t) || l.lost(dir, channel, seq, &w) {
            loop {
                t += rto;
                retransmits += 1;
                if !l.profile.in_burst(t) {
                    break;
                }
            }
        }
        let flow = &mut l.flows[dir as usize][ch];
        let arrival_ns = (t + l.model.delay(&w, W_JITTER)).max(flow.last_arrival_ns);
        flow.last_arrival_ns = arrival_ns;
        let index = log.entries.len() as u64;
        log.entries.push(Fate {
            link,
            dir,
            channel,
            seq,
            sent_ns: now_ns,
            len: u32::try_from(bytes.len()).unwrap_or(u32::MAX),
            outcome: Outcome::Delivered { arrival_ns },
            overtaken: false,
            retransmits,
        });
        let p = InFlight {
            arrival_ns,
            index,
            channel,
            sent_ns: now_ns,
            bytes: bytes.to_vec(),
            ack_back_ns: l.model.delay(&w, W_ACK),
        };
        l.launch(dir, p, retransmits > 0);
        SendResult::Sent
    }

    fn poll(&mut self, link: u32, end: usize, now_ns: u64) -> Option<Recv> {
        let l = &mut self.links[link as usize];
        l.advance_acks(end, now_ns);
        let dir = recv_dir(end);
        // A held datagram whose own arrival has come can no longer be overtaken: any later send is at now or after.
        if l.flows[dir as usize][0]
            .pending
            .as_ref()
            .is_some_and(|p| p.arrival_ns <= now_ns)
            && let Some(p) = l.flows[dir as usize][0].pending.take()
        {
            l.launch(dir, p, false);
        }
        let inbox = &mut l.inbox[dir as usize];
        let due = inbox
            .peek()
            .is_some_and(|p| p.0.arrival_ns <= now_ns && p.0.sent_ns < now_ns);
        if !due {
            return None;
        }
        inbox.pop().map(|Reverse(p)| Recv {
            channel: p.channel,
            bytes: p.bytes,
            arrival_ns: p.arrival_ns,
        })
    }

    fn close(&mut self, link: u32, end: usize) {
        let l = &mut self.links[link as usize];
        l.open[end] = false;
        let toward = recv_dir(end) as usize;
        l.inbox[toward].clear();
        l.flows[toward][0].pending = None;
    }
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
        self.net.borrow().links[self.link as usize].rtt[self.end].smoothed
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
                #[cfg(feature = "bites")]
                bite: None,
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

    /// Reads the fate log so far.
    pub fn with_fate_log<R>(&self, f: impl FnOnce(&FateLog) -> R) -> R {
        f(&self.net.borrow().log)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn below_maps_the_whole_range_evenly_at_its_ends() {
        assert_eq!(below(0, 10), 0);
        assert_eq!(below(u64::MAX, 10), 9);
        assert_eq!(below(1 << 63, 10), 5);
        assert_eq!(below(u64::MAX, 1), 0);
    }

    #[test]
    fn the_named_profiles_are_part_13s() {
        let rows = [
            (Profile::p0(), "P0", 0, 0, 0),
            (Profile::p1(), "P1", 50, 10, 10_000),
            (Profile::p2(), "P2", 100, 20, 20_000),
            (Profile::p3(), "P3", 150, 30, 50_000),
            (Profile::p4(), "P4", 200, 40, 50_000),
            (Profile::b1(&[5 * SEC]), "B1", 100, 20, 20_000),
        ];
        for (p, name, rtt_ms, jitter_ms, ppm) in rows {
            assert_eq!(
                (p.name, p.rtt_ns, p.jitter_ns, p.loss_ppm),
                (name, rtt_ms * MS, jitter_ms * MS, ppm)
            );
            assert_eq!(
                p.one_way_ns(),
                rtt_ms * MS / 2,
                "{name}: half the latency each way"
            );
            assert_eq!(
                p.half_jitter_ns(),
                jitter_ms * MS / 2,
                "{name}: half the jitter each way"
            );
            assert_eq!(
                p.reorder,
                ReorderStep::PART_13,
                "{name}: every run also reordering"
            );
        }
        let b1 = Profile::b1(&[5 * SEC]);
        assert_eq!(b1.bursts_ns, vec![(5 * SEC, SEC)]);
        assert!(!b1.in_burst(5 * SEC - 1) && b1.in_burst(5 * SEC) && b1.in_burst(6 * SEC - 1));
        assert!(!b1.in_burst(6 * SEC));
        assert_eq!(Profile::by_name("P3"), Some(Profile::p3()));
        assert_eq!(Profile::by_name("B1"), None, "B1 needs its burst times");
        assert_eq!(Profile::p2().rto_ns(), 200 * MS);
        assert_eq!(Profile::p4().rto_ns(), 400 * MS);
    }

    #[test]
    fn the_rtt_estimator_follows_rfc_9002() {
        let mut r = Rtt::default();
        assert_eq!(r.smoothed, None);
        r.sample(100);
        assert_eq!((r.smoothed, r.var), (Some(100), 50));
        r.sample(200);
        // var = (3 * 50 + |100 - 200|) / 4 = 62; smoothed = (7 * 100 + 200) / 8 = 112.
        assert_eq!((r.smoothed, r.var), (Some(112), 62));
    }

    #[test]
    fn the_canonical_form_is_the_tag_then_40_bytes_per_entry() {
        let log = FateLog {
            entries: vec![
                Fate {
                    link: 1,
                    dir: Dir::Down,
                    channel: Channel::Control,
                    seq: 2,
                    sent_ns: 3,
                    len: 4,
                    outcome: Outcome::Delivered { arrival_ns: 5 },
                    overtaken: true,
                    retransmits: 6,
                },
                Fate {
                    link: 0,
                    dir: Dir::Up,
                    channel: Channel::Datagram,
                    seq: 0,
                    sent_ns: 9,
                    len: 1,
                    outcome: Outcome::Burst,
                    overtaken: false,
                    retransmits: 0,
                },
            ],
        };
        let mut bytes = Vec::new();
        log.for_each_canonical(|b| bytes.extend_from_slice(b));
        assert_eq!(&bytes[..CANONICAL_TAG.len()], CANONICAL_TAG);
        let e = &bytes[CANONICAL_TAG.len()..];
        assert_eq!(e.len(), 80);
        assert_eq!(&e[0..8], &[1, 0, 0, 0, 1, 1, 0, 1]);
        assert_eq!(&e[8..16], &2u64.to_le_bytes());
        assert_eq!(&e[24..32], &5u64.to_le_bytes());
        assert_eq!(&e[36..40], &6u32.to_le_bytes());
        assert_eq!(
            &e[40..48],
            &[0, 0, 0, 0, 0, 0, 2, 0],
            "a burst loss has kind 2"
        );
        assert_eq!(
            &e[64..72],
            &0u64.to_le_bytes(),
            "no arrival for a lost packet"
        );
    }

    #[test]
    fn reorder_counts_follow_rfc_4737() {
        let mk = |seq: u64, arrival_ns: u64| Fate {
            link: 0,
            dir: Dir::Up,
            channel: Channel::Datagram,
            seq,
            sent_ns: seq,
            len: 1,
            outcome: Outcome::Delivered { arrival_ns },
            overtaken: false,
            retransmits: 0,
        };
        // Arrival order 0, 2, 1, 3, 5, 4: seq 1 and seq 4 arrive after a later-sent datagram.
        let log = FateLog {
            entries: vec![
                mk(0, 10),
                mk(1, 31),
                mk(2, 30),
                mk(3, 40),
                mk(4, 61),
                mk(5, 60),
            ],
        };
        assert_eq!(log.reorder_counts(), [(6, 2), (0, 0)]);
    }
}
