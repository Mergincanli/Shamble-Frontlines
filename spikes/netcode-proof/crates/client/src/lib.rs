//! npt-client: the client endpoint of the netcode proof test (plan E7-E10, §4.7; ADR 0002:190, 313-373).
//!
//! One `input` call and one `step` call per tick: received messages arrive with their channel and the host's arrival
//! stamp, and `step` returns the datagrams and reliable messages to send, the block for the page, and when the next
//! tick is due. Prediction and reconciliation, interpolation, clock sync, the aim cast and the block writer live here.
//! No bindings, no transport, no clock; native for the bots and wasm32 for the Worker. Built in units U4 and U7.
//!
//! U4e (u4-build-plan.md §5 U4e) builds a minimal [`ClientEndpoint`]: decoding into integer frames and held floats,
//! the complete-tick rule and the sentinel (§4.6), the client side of `client_hash`, the acknowledgement with its
//! group mask, the rebuilds of the header's 16-bit sequences and of the event numbers, and the answers to the room's
//! pings. Prediction, interpolation and the clock are U4f's and U4g's.

use std::collections::VecDeque;

use npt_link::Channel;
use npt_proto::{
    DecodeError, Direction, EntityDelta, FullState, Input, Join, Limits, Message, Pong, Quant,
    Snapshot, Wrap, client_hash,
};
use npt_sim::grid::{Config, Field, PITCH, POSITION_XY, POSITION_Z, VELOCITY, YAW};
use npt_sim::{Command, EntityKind, EntityRecord, SimEvent};

/// The bite ids this crate plants (bites builds only). u4-build-plan.md §4.14 plants each bite where its code lives:
/// PM-0001-17/client-rounding's site is the client's call into the wire grid, so its id lives here and not in npt-sim's
/// grid; the endpoint takes the grid for its every call into it from [`grid_with_bite`]. PM-0001-58/lt-tick-offset and
/// PM-0001-58/lt-event-seq are the endpoint's two rebuilds (carry 24).
#[cfg(feature = "bites")]
pub const BITES: [&str; 3] = [
    "PM-0001-17/client-rounding",
    "PM-0001-58/lt-tick-offset",
    "PM-0001-58/lt-event-seq",
];

/// The grid a client with the planted defect `bite` uses for its every call into it (bites builds only): for
/// PM-0001-17/client-rounding ("a client quantiser that rounds differently from the server's") the grid's one defect,
/// `npt_sim::grid::Config::floor_rounding`; `None` for an id this crate does not plant there.
#[cfg(feature = "bites")]
pub fn grid_with_bite(bite: &str) -> Option<npt_sim::grid::Config> {
    (bite == BITES[0]).then(npt_sim::grid::Config::floor_rounding)
}

/// The last-run rebuild's counter name in `Wrap` (PM-0001-58/lt-tick-offset: u4-build-plan.md §4.14 :469).
const TICK_OFFSET: &str = "tick_offset";
/// The event-number rebuild's counter name (PM-0001-58/lt-event-seq: §4.14 :470).
const EVENT_SEQ: &str = "event_seq";
/// The newest-received rebuild's counter name (no bite of its own).
const CMD_SEQ: &str = "cmd_seq";

/// The fields that hold a float on the client (the grid's five float kinds); every other field is an integer.
fn is_float(f: Field) -> bool {
    [POSITION_XY, POSITION_Z, YAW, PITCH, VELOCITY].contains(&f)
}

/// One record as the client holds it: its integers as decoded (the delta baseline), and, for each float field, the
/// float it holds for interpolation and prediction (`from_grid` of the integer; 0 for an integer field).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Held {
    rec: EntityRecord,
    held: [f32; EntityRecord::FIELDS],
}

impl Held {
    fn of(rec: EntityRecord, grid: Config) -> Held {
        let mut held = [0.0; EntityRecord::FIELDS];
        for ((h, f), &k) in held.iter_mut().zip(rec.kind.layout()).zip(&rec.grid) {
            if is_float(*f) {
                *h = grid.from_grid(*f, k);
            }
        }
        Held { rec, held }
    }

    /// The record the hash reads: each float field's held value turned back into a grid integer through `grid`
    /// (u4-build-plan.md §4.6: "the floats it holds ... turned back into grid integers through grid").
    fn hashed(&self, grid: Config) -> EntityRecord {
        let mut r = self.rec;
        for ((g, f), &h) in r
            .grid
            .iter_mut()
            .zip(self.rec.kind.layout())
            .zip(&self.held)
        {
            if is_float(*f) {
                *g = grid.to_grid(*f, h);
            }
        }
        r
    }
}

fn key(h: &Held) -> (EntityKind, u16) {
    (h.rec.kind, h.rec.index)
}

/// A view at one tick, each list sorted by (kind, index).
#[derive(Clone, Debug, Default, PartialEq)]
struct Frame {
    entities: Vec<Held>,
    own: Vec<Held>,
}

/// `list` after one delta, an update read against `base`'s record (each group decodes alone against one baseline).
fn apply(
    list: &mut Vec<Held>,
    base: &[Held],
    d: &EntityDelta,
    grid: Config,
) -> Result<(), DecodeError> {
    let at = |list: &[Held], k: (EntityKind, u16)| list.binary_search_by_key(&k, key);
    match d {
        EntityDelta::Remove { kind, index } => {
            if let Ok(i) = at(list, (*kind, *index)) {
                list.remove(i);
            }
            Ok(())
        }
        EntityDelta::Add(r) => {
            let h = Held::of(d.apply(None)?.unwrap_or(*r), grid);
            match at(list, key(&h)) {
                Ok(i) => list[i] = h,
                Err(i) => list.insert(i, h),
            }
            Ok(())
        }
        EntityDelta::Update { kind, index, .. } => {
            let b = at(base, (*kind, *index)).ok().map(|i| &base[i].rec);
            let r = d.apply(b)?.ok_or(DecodeError::Malformed)?;
            let h = Held::of(r, grid);
            match at(list, key(&h)) {
                Ok(i) => list[i] = h,
                Err(i) => list.insert(i, h),
            }
            Ok(())
        }
    }
}

/// What the client has made of one tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Filed {
    #[default]
    Pending,
    Hash(u64),
    Sentinel,
}

/// One tick in the client's frame ring.
#[derive(Clone, Debug, Default)]
struct Tick {
    tick: u32,
    used: bool,
    groups: u8,
    got: u8,
    sigs: [u64; Limits::GROUPS],
    base_age: Option<u16>,
    frame: Frame,
    events: Vec<(u32, SimEvent)>,
    filed: Filed,
}

impl Tick {
    fn complete(&self) -> bool {
        self.groups > 0 && u16::from(self.got) == (1u16 << self.groups) - 1
    }
}

// The group masks (`Tick::got`, the acknowledgement's) are a u8 and `Tick::sigs` holds one entry per group: they rely
// on at most 8 groups a tick (RR-3; the wire's 3-bit group).
const _: () = assert!(
    Limits::GROUPS <= 8,
    "a u8 group mask holds at most 8 groups"
);

/// FNV-1a 64 of a datagram's bytes: a duplicated datagram is the same bytes (carry 61 (b): filed once).
fn signature(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// What one `step` gives the host: the datagrams and reliable messages to send, when the next step is due, and what
/// U4r's logs write for this client (U4E-Q7): each hash filed under the tick it rebuilt (the sentinel 0 for a tick with
/// a received group it could not decode), the ticks resolved lost (`tick.s.snap_incomplete` 1) as (first tick,
/// count), and the counters that wrapped by their `Wrap` names (-58's `event.counter_wrap`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StepOut {
    /// The datagrams to send, in order (the `Input`, then the `Pong`s).
    pub datagrams: Vec<Vec<u8>>,
    /// The reliable messages, each with its channel (the `Join`).
    pub reliable: Vec<(Channel, Vec<u8>)>,
    /// When the next step is due, in ns on the host's clock.
    pub next_due_ns: u64,
    /// (tick, hash) for each tick filed since the last step; hash 0 is the sentinel.
    pub hashes: Vec<(u32, u64)>,
    /// (first tick, count) for each run of ticks resolved lost since the last step.
    pub lost: Vec<(u32, u32)>,
    /// The counters that wrapped since the last step.
    pub wraps: Vec<&'static str>,
}

/// The client endpoint (plan.md:486-489): `input` with the page's command and every message received since the last
/// tick as (channel, bytes, arrival ns), then `step`. Every time is integer ns on the host's clock; it reads no clock.
/// npt-client names nothing from npt-link but `Channel` (check.mjs; catalogue item 2).
#[derive(Clone, Debug)]
pub struct ClientEndpoint {
    join: Join,
    join_sent: bool,
    slot: Option<u8>,
    closed: bool,
    tick_hz: u32,
    lead: u32,
    quant: Quant,
    grid: Config,
    wrap: Wrap,
    ring: Vec<Tick>,
    newest: Option<u32>,
    /// The tick of the first header taken (its first group's, full state's or refused datagram's): a header before it
    /// and at least the ring behind the newest names a tick the client was never sent (advisor-u4e-r1 R1, case 3).
    first: Option<u32>,
    /// The bad headers' ticks already given the sentinel (case 3), at most `BAD_HEADERS_HELD`, oldest first.
    bad_headers: VecDeque<u32>,
    low: u32,
    ack: Option<(u32, u8)>,
    next_cmd: u32,
    cmd_sent: bool,
    unacked: VecDeque<(u32, Command)>,
    newest_received: Option<u32>,
    last_run: Option<u32>,
    event_near: u32,
    pings: Vec<(u8, u64, u64)>,
    now_ns: u64,
    base_ns: Option<u64>,
    steps: u64,
    wrong_channel: u64,
    refused: u64,
    stale: u64,
    duplicates: u64,
    unknown_pongs: u64,
    out: StepOut,
}

/// Pings the endpoint holds to answer at its next step, at most (the room sends a few a second).
const PINGS_HELD: usize = 8;
/// Bad headers' ticks the endpoint remembers, so each gets the sentinel once (the groups of one tick arrive together).
const BAD_HEADERS_HELD: usize = npt_sim::ACK_RING_TICKS as usize;

impl ClientEndpoint {
    /// An endpoint that joins with `join` (its first step sends it), stepping at `tick_hz` (production callers pass
    /// `npt_sim::TICK_HZ`), on the wire's `Quant`, grid and `Wrap`.
    pub fn new(join: Join, tick_hz: u32) -> ClientEndpoint {
        ClientEndpoint {
            join,
            join_sent: false,
            slot: None,
            closed: false,
            tick_hz: tick_hz.max(1),
            lead: 1,
            quant: Quant::default(),
            grid: Config::default(),
            wrap: Wrap::default(),
            ring: vec![Tick::default(); npt_sim::ACK_RING_TICKS as usize],
            newest: None,
            first: None,
            bad_headers: VecDeque::new(),
            low: 0,
            ack: None,
            next_cmd: 0,
            cmd_sent: false,
            unacked: VecDeque::new(),
            newest_received: None,
            last_run: None,
            event_near: 0,
            pings: Vec::new(),
            now_ns: 0,
            base_ns: None,
            steps: 0,
            wrong_channel: 0,
            refused: 0,
            stale: 0,
            duplicates: 0,
            unknown_pongs: 0,
            out: StepOut::default(),
        }
    }

    /// The same endpoint stamping each command `lead` ticks after the newest tick it holds (the option-B buffer's
    /// lead; U4f's controller replaces it).
    pub fn with_lead(mut self, lead: u32) -> ClientEndpoint {
        self.lead = lead;
        self
    }

    /// The same endpoint with its command counter started at `start` (its low 16 bits the first sequence) and its
    /// event rebuild started near `event_start`, the room's event counter's start (-58's injectable starts).
    pub fn with_starts(mut self, start: u32, event_start: u32) -> ClientEndpoint {
        self.next_cmd = start;
        self.event_near = event_start;
        self
    }

    /// The same endpoint with one planted defect by its id (bites builds only; per instance): one of [`BITES`]
    /// (the grid of [`grid_with_bite`], or a plain `<` on the rebuild it names), or one of npt-proto's, whose `Quant`
    /// the endpoint decodes with (U4E-Q4); `None` for an id neither plants.
    #[cfg(feature = "bites")]
    pub fn with_bite(mut self, bite: &str) -> Option<ClientEndpoint> {
        if let Some(grid) = grid_with_bite(bite) {
            self.grid = grid;
        } else if bite == BITES[1] {
            self.wrap = Wrap::plain_lt(TICK_OFFSET);
        } else if bite == BITES[2] {
            self.wrap = Wrap::plain_lt(EVENT_SEQ);
        } else {
            self.quant = Quant::with_bite(bite)?;
        }
        Some(self)
    }

    /// The slot `JoinAccepted` gave, once joined.
    pub fn slot(&self) -> Option<u8> {
        self.slot
    }

    /// Whether the room ended the session (`JoinRejected`, `Kick` or `RoomEnded`).
    #[allow(
        dead_code,
        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
    )]
    pub(crate) fn closed(&self) -> bool {
        self.closed
    }

    /// The newest snapshot tick the client has read in a header.
    pub fn newest_tick(&self) -> Option<u32> {
        self.newest
    }

    /// The command the header's last-run sequence names, rebuilt to the client's own 32-bit command counter (U4E-Q2
    /// (c): where U4g's replay starts).
    pub fn last_run_command(&self) -> Option<u32> {
        self.last_run
    }

    /// Messages dropped for arriving on a channel their type does not travel on (catalogue item 2: "on both sides").
    pub fn wrong_channel(&self) -> u64 {
        self.wrong_channel
    }

    /// Received messages refused by the decoder, by the baseline rule, or as a bad header (a tick before the first
    /// header taken, at least the ring behind the newest); a snapshot's tick then gets the sentinel.
    pub fn refused(&self) -> u64 {
        self.refused
    }

    /// Snapshot groups and full states of a tick at least the ring behind the newest, from the first header taken on
    /// (a tick resolved when it left the ring), dropped.
    pub fn stale(&self) -> u64 {
        self.stale
    }

    /// Duplicated datagrams (a group already decoded, the same bytes), filed once.
    pub fn duplicates(&self) -> u64 {
        self.duplicates
    }

    /// `Pong`s whose ping id is not one of the client's own (it sends none at U4e), dropped.
    #[allow(
        dead_code,
        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
    )]
    pub(crate) fn unknown_pongs(&self) -> u64 {
        self.unknown_pongs
    }

    /// The page's command for the next step (its slot, tick and sequence are the endpoint's to stamp; `None`: no new
    /// command), and every message received since the last call, each as (channel, bytes, arrival ns), at `now_ns`.
    pub fn input(
        &mut self,
        command: Option<Command>,
        received: &[(Channel, Vec<u8>, u64)],
        now_ns: u64,
    ) {
        self.now_ns = now_ns;
        self.base_ns.get_or_insert(now_ns);
        for (channel, bytes, arrival_ns) in received {
            self.receive(*channel, bytes, *arrival_ns);
        }
        if let (Some(c), Some(slot)) = (command, self.slot) {
            let index = self.next_cmd;
            self.next_cmd = index.wrapping_add(1);
            self.cmd_sent = true;
            let tick = self.newest.map_or(0, |n| n.wrapping_add(self.lead));
            let mut c = Command {
                slot,
                tick,
                seq: index as u16,
                ..c
            };
            // A view time the wire cannot carry (after the command's tick, or more than 65,535 ticks before it: the
            // `Input` codec's view offset) is taken as the command's own tick, so every `Input` encodes.
            if tick.wrapping_sub(c.view_tick) > u32::from(u16::MAX) {
                c.view_tick = tick;
                c.view_fraction = 0;
            }
            self.unacked.push_front((index, c));
            self.unacked.truncate(Limits::REDUNDANCY_CAP);
        }
    }

    /// The step: the `Join` first, then each tick the `Input` (the newest command and every unacknowledged one, and the
    /// acknowledgement of the newest snapshot tick a group of which decoded, with its group mask), the `Pong`s, and the
    /// next due time.
    pub fn step(&mut self) -> StepOut {
        if !self.join_sent {
            let mut out = Vec::new();
            if npt_proto::encode(&Message::Join(self.join), &mut out).is_ok() {
                self.out.reliable.push((Channel::Control, out));
            }
            self.join_sent = true;
        }
        if self.slot.is_some() && !self.closed {
            let (ack_tick, ack_groups) = self.ack.map_or((0, 0), |(t, m)| (t as u16, m));
            if ack_groups != 0 || !self.unacked.is_empty() {
                let input = Input {
                    ack_tick,
                    ack_groups,
                    commands: self.unacked.iter().map(|(_, c)| *c).collect(),
                };
                let mut out = Vec::new();
                if npt_proto::encode(&Message::Input(input), &mut out).is_ok() {
                    self.out.datagrams.push(out);
                }
            }
            for (id, echo_ns, arrival_ns) in self.pings.drain(..) {
                let hold = self.now_ns.saturating_sub(arrival_ns) / 1_000;
                let pong = Pong {
                    id,
                    echo_ns,
                    tick: self.newest.unwrap_or(0),
                    hold_us: u16::try_from(hold).unwrap_or(u16::MAX),
                };
                let mut out = Vec::new();
                if npt_proto::encode(&Message::Pong(pong), &mut out).is_ok() {
                    self.out.datagrams.push(out);
                }
            }
        }
        self.steps += 1;
        let base = self.base_ns.unwrap_or(self.now_ns);
        let since = u128::from(self.steps) * 1_000_000_000 / u128::from(self.tick_hz);
        self.out.next_due_ns = base.saturating_add(u64::try_from(since).unwrap_or(u64::MAX));
        core::mem::take(&mut self.out)
    }

    fn receive(&mut self, channel: Channel, bytes: &[u8], arrival_ns: u64) {
        match self.quant.decode(Direction::RoomToClient, channel, bytes) {
            Ok(Message::Snapshot(s)) => self.group(s, bytes),
            Ok(Message::FullState(f)) => self.full_state(f),
            Ok(Message::JoinAccepted(a)) => self.slot = Some(a.slot),
            Ok(Message::JoinRejected(_) | Message::Kick(_) | Message::RoomEnded(_)) => {
                self.closed = true
            }
            Ok(Message::Ping(p)) => {
                if self.pings.len() < PINGS_HELD {
                    self.pings.push((p.id, p.sent_ns, arrival_ns));
                }
            }
            Ok(Message::Pong(_)) => self.unknown_pongs += 1,
            Ok(_) => {}
            Err(DecodeError::WrongChannel) => self.wrong_channel += 1,
            Err(_) => {
                self.refused += 1;
                // A snapshot datagram the client could not decode (a size or decode error): the sentinel under the
                // tick its header names, read whole from its first four bytes after the type (snapshot.rs's wire text).
                let snapshot = Limits::TYPES
                    .iter()
                    .position(|r| r.0 == "Snapshot")
                    .map(|i| i as u8 + 1);
                if channel == Channel::Datagram
                    && bytes.first().copied() == snapshot
                    && let Some(t) = bytes.get(1..5)
                {
                    let t = u32::from_le_bytes([t[0], t[1], t[2], t[3]]);
                    self.sentinel(t);
                }
            }
        }
    }

    /// Whether `t` is too old for the ring: at least its length older than the newest tick held.
    fn too_old(&self, t: u32) -> bool {
        self.newest.is_some_and(|n| {
            !self.wrap.after32("tick", n, t) && n.wrapping_sub(t) >= npt_sim::ACK_RING_TICKS
        })
    }

    /// Whether a header naming tick `t` is too old for the ring, so nothing is decoded from it (advisor-u4e-r1 R1). A
    /// tick from the first header taken on was resolved when it left the ring: dropped and counted `stale` (case 2).
    /// A tick before the first header taken was never sent to the client: a bad header (§4.6), the sentinel under the
    /// tick it names (case 3), counted in `refused` when `count` (a decoder refusal is counted already).
    fn old_header(&mut self, t: u32, count: bool) -> bool {
        if !self.too_old(t) {
            return false;
        }
        if self
            .first
            .is_some_and(|f| Wrap::default().after32("tick", t, f))
        {
            self.refused += u64::from(count);
            self.bad_header(t);
        } else {
            self.stale += 1;
        }
        true
    }

    /// The sentinel for a bad header's tick `t` (case 3), once while `BAD_HEADERS_HELD` such ticks are remembered: no
    /// frame and no ring slot (slot `t % 32` may hold a live tick), never acknowledged, the newest tick unmoved.
    fn bad_header(&mut self, t: u32) {
        if self.bad_headers.contains(&t) {
            return;
        }
        if self.bad_headers.len() >= BAD_HEADERS_HELD {
            self.bad_headers.pop_front();
        }
        self.bad_headers.push_back(t);
        self.out.hashes.push((t, 0));
    }

    /// The ring slot for `t`, reset for it if it held another tick.
    fn slot_for(&mut self, t: u32) -> usize {
        let i = (t % npt_sim::ACK_RING_TICKS) as usize;
        if !self.ring[i].used || self.ring[i].tick != t {
            self.ring[i] = Tick {
                tick: t,
                used: true,
                ..Tick::default()
            };
        }
        i
    }

    /// A header with tick `t`: if it is the newest, every tick it pushes out of the ring is resolved, lost unless filed
    /// (carry 61 (c): only once none of its groups can still decode), and every tick between the old newest and it
    /// that no group named, too (a tick none of whose groups arrived counts, known from a later header's tick).
    fn see(&mut self, t: u32) {
        let len = npt_sim::ACK_RING_TICKS;
        let w = Wrap::default();
        let Some(n) = self.newest else {
            self.newest = Some(t);
            self.first = Some(t);
            self.low = t;
            return;
        };
        if !w.after32("tick", n, t) {
            return;
        }
        self.newest = Some(t);
        let cutoff = t.wrapping_sub(len - 1);
        let seen_end = n.wrapping_add(1);
        let ring_end = if w.after32("tick", seen_end, cutoff) {
            seen_end
        } else {
            cutoff
        };
        while w.after32("tick", self.low, ring_end) {
            let k = self.low;
            let e = &self.ring[(k % len) as usize];
            if !(e.used && e.tick == k && e.filed != Filed::Pending) {
                self.lost(k, 1);
            }
            self.low = k.wrapping_add(1);
        }
        if w.after32("tick", self.low, cutoff) {
            self.lost(self.low, cutoff.wrapping_sub(self.low));
            self.low = cutoff;
        }
    }

    /// A run of `count` ticks resolved lost from `first`: merged into the last run when it continues it and the sum
    /// fits a u32, else a new run (SEC-U4E-7: never an overflow, whatever the header ticks).
    fn lost(&mut self, first: u32, count: u32) {
        if let Some(last) = self.out.lost.last_mut()
            && last.0.wrapping_add(last.1) == first
            && let Some(sum) = last.1.checked_add(count)
        {
            last.1 = sum;
            return;
        }
        self.out.lost.push((first, count));
    }

    /// The sentinel for tick `t` (a received group it could not decode: §4.6), once.
    fn sentinel(&mut self, t: u32) {
        if self.old_header(t, false) {
            return;
        }
        self.see(t);
        let i = self.slot_for(t);
        if self.ring[i].filed != Filed::Sentinel {
            self.ring[i].filed = Filed::Sentinel;
            self.out.hashes.push((t, 0));
        }
    }

    /// Files `hash` for the tick in slot `i`: once for one hash; a second, different hash makes it the sentinel
    /// (carry 61 (b)).
    fn file(&mut self, i: usize, hash: u64) {
        let t = self.ring[i].tick;
        match self.ring[i].filed {
            Filed::Pending => {
                self.ring[i].filed = Filed::Hash(hash);
                self.out.hashes.push((t, hash));
            }
            Filed::Hash(h) if h != hash => {
                self.ring[i].filed = Filed::Sentinel;
                self.out.hashes.push((t, 0));
            }
            _ => {}
        }
    }

    fn acknowledge(&mut self, t: u32, group: u8) {
        let w = Wrap::default();
        match self.ack {
            Some((a, m)) if a == t => self.ack = Some((a, m | 1 << group)),
            Some((a, _)) if !w.after32("tick", a, t) => {}
            _ => self.ack = Some((t, 1 << group)),
        }
    }

    /// The header's two command sequences rebuilt to the client's own 32-bit command counter: the newest received
    /// ends the repeats; the last run names the command U4g's replay starts from (U4E-Q2 (c); W1-N2).
    fn rebuild_sequences(&mut self, newest_received: Option<u16>, last_run: Option<u16>) {
        if !self.cmd_sent {
            return;
        }
        // Nearest the newest command the client has stamped.
        let near = self.next_cmd.wrapping_sub(1);
        let w = self.wrap;
        if let Some(low) = newest_received {
            let n = w.widen16(CMD_SEQ, near, low);
            if self
                .newest_received
                .is_none_or(|p| Wrap::default().after32("tick", p, n))
            {
                self.newest_received = Some(n);
                self.unacked
                    .retain(|(i, _)| Wrap::default().after32("tick", n, *i));
            }
        }
        if let Some(low) = last_run {
            let r = w.widen16(TICK_OFFSET, near, low);
            if self
                .last_run
                .is_none_or(|p| Wrap::default().after32("tick", p, r))
            {
                if self.last_run.is_some_and(|p| p >> 16 != r >> 16) {
                    self.out.wraps.push(TICK_OFFSET);
                }
                self.last_run = Some(r);
            }
        }
    }

    /// An event's 32-bit number from its 16 bits, nearest the newest number the client knows (`Wrap::widen16`).
    fn event_number(&mut self, low: u16) -> u32 {
        let n = self.wrap.widen16(EVENT_SEQ, self.event_near, low);
        if Wrap::default().after32("tick", self.event_near, n) {
            if n >> 16 != self.event_near >> 16 {
                self.out.wraps.push(EVENT_SEQ);
            }
            self.event_near = n;
        }
        n
    }

    /// One snapshot group (u4-build-plan.md §4.6): decoded alone against its baseline (the defaults at age 0), its
    /// tick complete once every group has; a group it cannot decode gives the tick the sentinel.
    fn group(&mut self, s: Snapshot, bytes: &[u8]) {
        let t = s.tick;
        if self.old_header(t, true) {
            return;
        }
        self.see(t);
        let i = self.slot_for(t);
        if self.ring[i].filed == Filed::Sentinel {
            return;
        }
        let g = usize::from(s.group);
        let sig = signature(bytes);
        if self.ring[i].got & (1 << g) != 0 {
            if self.ring[i].sigs[g] == sig {
                self.duplicates += 1;
            } else {
                self.refused += 1;
                self.sentinel(t);
            }
            return;
        }
        let e = &self.ring[i];
        if (e.groups != 0 && e.groups != s.groups)
            || e.base_age.is_some_and(|a| a != s.baseline_age)
        {
            self.refused += 1;
            self.sentinel(t);
            return;
        }
        // The baseline: the defaults at age 0, else the complete frame `baseline_age` ticks before, if the ring holds
        // it; else the group is refused (no baseline) and the tick gets the sentinel.
        let base = if s.baseline_age == 0 {
            Some(Frame::default())
        } else {
            let b = t.wrapping_sub(u32::from(s.baseline_age));
            let j = (b % npt_sim::ACK_RING_TICKS) as usize;
            let held = &self.ring[j];
            (u32::from(s.baseline_age) < npt_sim::ACK_RING_TICKS
                && held.used
                && held.tick == b
                && held.complete()
                && matches!(held.filed, Filed::Hash(_)))
            .then(|| held.frame.clone())
        };
        let Some(base) = base else {
            self.refused += 1;
            self.sentinel(t);
            return;
        };
        let mut tick = core::mem::take(&mut self.ring[i]);
        if tick.got == 0 {
            tick.frame = base.clone();
        }
        let grid = self.grid;
        let decoded = s
            .own
            .iter()
            .try_for_each(|d| apply(&mut tick.frame.own, &base.own, d, grid))
            .and_then(|()| {
                s.entities
                    .iter()
                    .try_for_each(|d| apply(&mut tick.frame.entities, &base.entities, d, grid))
            });
        if decoded.is_err() {
            self.ring[i] = tick;
            self.refused += 1;
            self.sentinel(t);
            return;
        }
        for &(low, _age, event) in &s.events {
            // The event's tick is the header's less its age (snapshot.rs's wire text); the hash reads its number.
            let number = self.event_number(low);
            tick.events.push((number, event));
        }
        tick.got |= 1 << g;
        tick.groups = s.groups;
        tick.sigs[g] = sig;
        tick.base_age = Some(s.baseline_age);
        self.ring[i] = tick;
        self.acknowledge(t, s.group);
        self.rebuild_sequences(s.newest_received, s.last_run);
        if self.ring[i].complete() {
            let e = &self.ring[i];
            let grid = self.grid;
            let entities: Vec<EntityRecord> =
                e.frame.entities.iter().map(|h| h.hashed(grid)).collect();
            let own: Vec<EntityRecord> = e.frame.own.iter().map(|h| h.hashed(grid)).collect();
            let hash = client_hash(t, &entities, &own, &e.events);
            self.file(i, hash);
        }
    }

    /// A full state: the client's whole view at its tick, a complete frame, acknowledged as group 0 of that tick.
    fn full_state(&mut self, f: FullState) {
        let t = f.tick;
        if self.old_header(t, true) {
            return;
        }
        self.see(t);
        let i = self.slot_for(t);
        if self.ring[i].got != 0 {
            self.duplicates += 1;
            return;
        }
        let grid = self.grid;
        let mut frame = Frame {
            entities: f.entities.iter().map(|r| Held::of(*r, grid)).collect(),
            own: f.own.iter().map(|r| Held::of(*r, grid)).collect(),
        };
        frame.entities.sort_by_key(key);
        frame.own.sort_by_key(key);
        let entities: Vec<EntityRecord> = frame.entities.iter().map(|h| h.hashed(grid)).collect();
        let own: Vec<EntityRecord> = frame.own.iter().map(|h| h.hashed(grid)).collect();
        let hash = client_hash(t, &entities, &own, &[]);
        let e = &mut self.ring[i];
        e.frame = frame;
        e.groups = 1;
        e.got = 1;
        e.base_age = Some(0);
        self.file(i, hash);
        self.acknowledge(t, 0);
        self.rebuild_sequences(None, f.last_run);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The endpoint's call site into the grid (u4-build-plan.md §4.14 :459): its held floats turned back into grid
    /// integers. A sweep of position values with half-centimetre ties and negatives, against the server's wire grid.
    fn differ_share(grid: Config) -> (u32, u32) {
        let wire = Config::default();
        let (mut differ, mut all) = (0, 0);
        for k in -2_000i32..2_000 {
            for half in [0.0f64, 0.25, 0.5, 0.75] {
                let x = ((f64::from(k) + half) / 100.0) as f32;
                let r = EntityRecord {
                    kind: EntityKind::Soldier,
                    index: 0,
                    generation: 0,
                    area: 0,
                    grid: [0; EntityRecord::FIELDS],
                    raw: [0; EntityRecord::FIELDS],
                };
                let mut h = Held::of(r, grid);
                h.held[0] = x;
                differ += u32::from(h.hashed(grid).grid[0] != wire.to_grid(POSITION_XY, x));
                all += 1;
            }
        }
        (differ, all)
    }

    #[test]
    fn the_endpoints_grid_integers_equal_the_servers_over_a_sweep_with_ties_and_negatives() {
        assert_eq!(differ_share(Config::default()).0, 0);
    }

    /// PM-0001-17/client-rounding at the endpoint's call site (part (i)): the floor grid breaks a share of the sweep.
    #[cfg(feature = "bites")]
    #[test]
    fn pm_0001_17_client_rounding_breaks_a_share_of_the_endpoints_grid_integers() {
        let bitten = grid_with_bite(BITES[0]).expect("planted here");
        let (differ, all) = differ_share(bitten);
        println!("client-rounding at the endpoint: {differ} of {all} differ");
        assert!(differ > 0 && differ < all, "{differ} of {all}");
        // The share over values decoded from the wire (every position integer from -20 m to 20 m held as its float
        // and turned back), which the brief's 2.6 asks to state beside the sweep's.
        let (mut wire_differ, mut wire_all) = (0u32, 0u32);
        for k in -2_000..2_000 {
            let held = Config::default().from_grid(POSITION_XY, k);
            wire_differ += u32::from(bitten.to_grid(POSITION_XY, held) != k);
            wire_all += 1;
        }
        println!(
            "client-rounding over decoded values: {wire_differ} of {wire_all} differ (reported)"
        );
    }
}
