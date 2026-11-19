//! `Room<S: SimCore>` (u4-build-plan.md §4.1, §4.2; plan.md:596-598): the sim driven through the seam alone, its hit
//! history, the roster, and per client its [`ClientSlot`]. Each tick it fills one `TickInput` (the commands in the
//! canonical order, each with its Rewind, and the tick's scripted spawns, teleports and attribute changes), steps,
//! copies the step's events into its stamped list (carry 26) before it reads anything else, pushes the hit volumes,
//! re-floods on a portal change, keeps the reports for the logs, and codes each client's snapshot.

use npt_proto::{
    EncodeError, EntityDelta, FullState, Input, Join, JoinAccepted, JoinRejected, Kick, Limits,
    Message, Quant, Snapshot, Wrap, client_hash,
};
use npt_sim::{
    AttributeChange, Command, CoverVariant, EntityKind, EntityRecord, Greybox, HitHistory,
    InputReports, Placement, Rewind, Scene, SimCore, SimEvent, TickInput,
};

use crate::ack::{AckRing, Acked, Frame, find, key};
use crate::encoder::{
    DATAGRAM_OVERHEAD, Encoder, Packer, Section, delta_bits, event_bits, group_bytes, view,
};
use crate::events::{EventQueue, Queued};
use crate::input::{CMD_SEQ, Checks, CmdTiming, InputBuffer, Offered, PRESS, Refusal, Relax, dist};
use crate::interest::{AreaFlood, InterestGrid, Priority};
use crate::rewind::rewind_at;
use crate::roster::Roster;

/// The ack rebuild's counter name in `Wrap` (PM-0001-58/lt-ack plants a plain `<` on it).
pub(crate) const ACK: &str = "ack";
/// The relevance radius round a client's soldier, cm (plan silent on its value: provisional data, 64 m).
const RADIUS_CM: i32 = 6_400;
/// The interest grid's cell, cm.
const CELL_CM: i32 = 1_600;
/// The hit history's span, ms: provisional until U4h measures its test cap (u4-build-plan.md §4.9).
const HISTORY_MS: u32 = 2_000;
/// Refusal records held between two ticks, at most (a bound on memory; more are counted in `refusals_dropped`).
const REFUSALS_HELD: usize = 4_096;

/// The room's planted defects (bites builds only), chosen per instance by [`Room::with_bite`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RoomBite {
    NoFallback,
    LtAck,
    LtCmdSeq,
    LtPress,
    ServerEcho,
    Unbounded,
    OwnerToAll,
    NoLeadBound,
    NoNewerTick,
    NoSeqForward,
    NoMoveLength,
    NoPressEdge,
    NoAimBounds,
    NoAckSent,
    NoCatchupCap,
    NotOlderThan,
}

#[cfg(feature = "bites")]
impl RoomBite {
    fn of(id: &str) -> Option<RoomBite> {
        let all = [
            RoomBite::NoFallback,
            RoomBite::LtAck,
            RoomBite::LtCmdSeq,
            RoomBite::LtPress,
            RoomBite::ServerEcho,
            RoomBite::Unbounded,
            RoomBite::OwnerToAll,
            RoomBite::NoLeadBound,
            RoomBite::NoNewerTick,
            RoomBite::NoSeqForward,
            RoomBite::NoMoveLength,
            RoomBite::NoPressEdge,
            RoomBite::NoAimBounds,
            RoomBite::NoAckSent,
            RoomBite::NoCatchupCap,
            RoomBite::NotOlderThan,
        ];
        crate::BITES.iter().position(|b| *b == id).map(|i| all[i])
    }
}

/// Each client's running counts, for U4r's logs and the -21 and -22 counters U4f adds (U4E-Q7, U4E-Q11).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClientCounters {
    /// Ticks a snapshot went out (one or more groups).
    pub snapshots: u64,
    /// Snapshot datagrams (groups) sent.
    pub groups: u64,
    /// Ticks sent as 2 or more groups.
    pub split_ticks: u64,
    /// Full states sent (at join, on a `Resync`, on an overflow).
    pub full_states: u64,
    /// Snapshots coded against the per-type defaults (`baseline_age` 0).
    pub defaults_coded: u64,
    /// Snapshots coded against a baseline older than the ring (only under PM-0001-18/no-fallback).
    pub stale_coded: u64,
    /// Changed entities the priority accumulator held at the baseline's value, summed over ticks.
    pub skipped: u64,
    /// Ticks with no snapshot because the remainder could not hold the own-state group (plan silent on the case).
    pub budget_short: u64,
    /// Ticks whose snapshot the encoder refused, by `EncodeError` kind (carry 54; the advisor's A4): out of range ...
    pub refused_out_of_range: u64,
    /// ... too many ...
    pub refused_too_many: u64,
    /// ... too large ...
    pub refused_too_large: u64,
    /// ... not canonical.
    pub refused_not_canonical: u64,
    /// Relevant gameplay events queued (U4E-Q6 condition 1).
    pub events_queued: u64,
    /// Gameplay events carried for the first time.
    pub events_sent: u64,
    /// Gameplay events carried again (resent until acknowledged).
    pub events_resent: u64,
    /// Gameplay events dropped from the resend at age 256, before the overflow resync came (carry 50).
    pub events_expired: u64,
    /// Cosmetic events sent (once each).
    pub cosmetic_sent: u64,
    /// Cosmetic events the remainder had no room for (never resent).
    pub cosmetic_skipped: u64,
    /// Overflow resyncs (carry 57); the tick of each is [`ClientSlot::overflow_resync`] on that tick.
    pub overflow_resyncs: u64,
    /// `Resync`s answered with a full state.
    pub resyncs: u64,
    /// A second `Resync` before the full state's acknowledgement: dropped (catalogue :90).
    pub resyncs_dropped: u64,
    /// A `Resync` whose answer would come sooner than `npt_sim::FULL_STATE_ACK_TICKS` ticks after the last full state
    /// sent the client: dropped (SEC-U4E-4: an acknowledgement, the client's own claim, never gates one by itself).
    pub resyncs_limited: u64,
    /// `Kick(overflow)`s sent: the client was dropped (§4.7).
    pub kicked: u64,
    /// Commands accepted (buffered for their tick under option B, held for the coming tick under option A).
    pub commands_buffered: u64,
    /// Redundant copies: a sequence not after the newest received whose kept command is equal, or was refused: not
    /// run again, never forged.
    pub commands_repeat: u64,
    /// Kind 1: commands stamped more than the lead bound ahead of the room's next tick.
    pub lead_refused: u64,
    /// Kind 2: commands not strictly newer than the newest accepted (on a fresh slot: at or before the join's tick).
    pub not_newer: u64,
    /// Kind 3: resent copies, with changed content, of a command the room accepted.
    pub seq_refused: u64,
    /// Kind 4: commands whose move vector is longer than 1.
    pub move_refused: u64,
    /// Kind 5: commands refused by the press-edge compare.
    pub press_refused: u64,
    /// Kind 6: commands aiming outside the map's x and y extent.
    pub aim_refused: u64,
    /// Commands stamped for a tick already run (option B), or more than `CMD_CATCHUP` ticks before the tick they
    /// would run at (option A's past window): dropped as late, never forged.
    pub late: u64,
    /// Kind 10: commands refused by the catch-up cap (option B: the buffer held `CMD_CATCHUP` already; option A:
    /// `CMD_CATCHUP` were held for the coming tick).
    pub buffer_full: u64,
    /// A sequence not after the newest received with no command kept under it (older than the kept window, or a
    /// gap): dropped, never forged.
    pub seq_outside_window: u64,
    /// A fresh session's command after its join's tick but at or before the slot's newest stamp from an earlier
    /// session: dropped, never forged (U4F-Q18 (ii) (d)).
    pub rejoin_overlap: u64,
    /// Kind 8: datagrams whose acknowledgement named no snapshot sent to this client: their commands refused.
    pub ack_refused: u64,
    /// The commands those datagrams carried.
    pub ack_refused_commands: u64,
    /// Acknowledgements of a sent tick older than the ring.
    pub acks_old: u64,
    /// Commands run at the tick stamped on them.
    pub commands_run: u64,
    /// Ticks on which option B ran the last command again (none was stamped for the tick).
    pub commands_rerun: u64,
    /// Commands holding or pressing fire whose view time the ring did not hold: run with no Rewind (U4E-Q5 (a)),
    /// a replacement in -30's setup condition (carry 40).
    pub rewinds_refused: u64,
}

/// What one tick did for one client (the per-tick values U4r's logs write).
#[derive(Clone, Debug, Default)]
struct TickValues {
    server_hash: Option<u64>,
    burst_eviction: bool,
    baseline_age: Option<u16>,
    groups: u8,
    bytes: u32,
    budget: u32,
    remainder: u32,
    own_bits: u32,
    own_state_bits: u32,
    own_state_changed: bool,
    overflow_resync: bool,
    headroom: i64,
    wraps: Vec<&'static str>,
    rewinds_refused: Vec<u16>,
    skipped: u32,
    /// Whether a command stamped for this tick had arrived before it ran (`None` before the first accepted stamp).
    on_time: Option<bool>,
    /// Press edges the commands run this tick carried, of them lost, and doubled (-20's and -58's press series).
    presses: [u32; 3],
    /// Option A: for each command run this tick, its stamp's signed distance from the tick (ticks run before it).
    ran_early: Vec<i32>,
    /// Commands handed to the sim for this client this tick (option B's repeat included).
    cmds_run: u32,
}

/// One client's slot in the room: its session's ring, queue, buffer, encoder and priorities, its outbox, and what the
/// last tick did for it.
#[derive(Clone, Debug)]
pub struct ClientSlot {
    slot: u8,
    team: u8,
    ack: AckRing,
    queue: EventQueue,
    input: InputBuffer,
    encoder: Encoder,
    priority: Priority,
    /// A full state is due this tick: `Some(forced)`, forced by an overflow.
    full_state_due: Option<bool>,
    /// The full state sent and not yet acknowledged: its tick, and whether an overflow forced it.
    in_flight: Option<(u32, bool)>,
    /// The tick of the last full state sent (the join's, a forced one or an answered one): the `Resync` limit's.
    last_full_state: Option<u32>,
    kicked: bool,
    server_bytes: u32,
    max_datagram: usize,
    datagrams: Vec<Vec<u8>>,
    /// The reliable messages, each encoded once with the room's `Quant` (plan.md:508-509).
    control: Vec<Vec<u8>>,
    counters: ClientCounters,
    last_ack_hi: Option<u16>,
    pending_wraps: Vec<&'static str>,
    /// The fire press counter of the last command run (a fire edge asks for a Rewind).
    fire_presses: Option<u8>,
    t: TickValues,
    relevant: Vec<usize>,
    sees: Vec<bool>,
    nearest: Vec<u16>,
    cosmetic: Vec<(u32, SimEvent)>,
}

impl ClientSlot {
    /// The slot.
    pub fn slot(&self) -> u8 {
        self.slot
    }

    /// The running counts.
    pub fn counters(&self) -> &ClientCounters {
        &self.counters
    }

    /// The last tick's server hash (marks/README.md §1's `server_hash`): `None` when the room sent this client
    /// nothing at that tick (carry 61 (a)).
    pub fn server_hash(&self) -> Option<u64> {
        self.t.server_hash
    }

    /// Whether the client's last acknowledged baseline had left the ring at the last tick (u4-build-plan.md §4.5;
    /// SERIES.md's `event.burst_eviction.evicted`, written at the first tick after a burst).
    pub fn burst_eviction(&self) -> bool {
        self.t.burst_eviction
    }

    /// The last tick's snapshot's baseline age (0: the per-type defaults); `None` when none was sent.
    pub fn baseline_age(&self) -> Option<u16> {
        self.t.baseline_age
    }

    /// The last tick's snapshot groups.
    pub fn groups(&self) -> u8 {
        self.t.groups
    }

    /// The last tick's snapshot bytes, payload plus 28 B each.
    pub fn bytes(&self) -> u32 {
        self.t.bytes
    }

    /// The last tick's window budget, and its remainder after the pump's bytes (the encoder's budget: §4.5).
    pub fn budget(&self) -> (u32, u32) {
        (self.t.budget, self.t.remainder)
    }

    /// The newest tick whose every group the client acknowledged (baseline B's tick while the ring holds it), held or
    /// evicted.
    pub fn last_acked(&self) -> Option<u32> {
        self.ack.last_acked()
    }

    /// The own-state section's bits in the last tick's snapshot (carry 56), and the `OwnState` record's share of
    /// them, and whether that record changed against the baseline.
    pub fn own_section(&self) -> (u32, u32, bool) {
        (
            self.t.own_bits,
            self.t.own_state_bits,
            self.t.own_state_changed,
        )
    }

    /// Whether the last tick forced an overflow resync (carry 57: its tick is that tick).
    pub fn overflow_resync(&self) -> bool {
        self.t.overflow_resync
    }

    /// The counters that wrapped since the tick before, by their `Wrap` names (-58's `event.counter_wrap`).
    pub fn wraps(&self) -> &[&'static str] {
        &self.t.wraps
    }

    /// The sequences of the last tick's commands that holding or pressing fire got no Rewind (U4E-Q5 (a); carry 40's
    /// per-shot log).
    pub fn rewinds_refused(&self) -> &[u16] {
        &self.t.rewinds_refused
    }

    /// The changed entities the last tick's snapshot held at the baseline's value.
    #[allow(
        dead_code,
        reason = "no consumer yet (advisor-u4e-r1 R2, A5): crate-private until a unit needs it public"
    )]
    pub(crate) fn skipped(&self) -> u32 {
        self.t.skipped
    }

    /// The gameplay events queued now and the most queued at once (U4E-Q6 condition 1).
    pub fn queue(&self) -> (u32, u32) {
        (self.queue.len(), self.queue.high_water())
    }

    /// The event queue's cap less its length, signed, at its lowest during the last tick (before an overflow resync
    /// cleared it): SERIES.md's `tick.v.ack_starved_headroom_count` (U4E-Q12 (c)); below 0 only under
    /// PM-0001-22/unbounded.
    pub fn headroom(&self) -> i64 {
        self.t.headroom
    }

    /// The input buffer's depth, in ticks ahead, at the last tick (`buffer_depth_ticks`).
    pub fn buffer_depth_ticks(&self) -> u32 {
        self.input.buffer_depth_ticks()
    }

    /// Whether the room dropped the client (`Kick(overflow)`).
    pub fn kicked(&self) -> bool {
        self.kicked
    }

    /// The last tick's `tick.s.cmd_on_time` (and `tick.s.input_missing`, its complement), by stamp (NA-14; the
    /// advisor's A3): `None` until the client's first accepted stamp is at or before the tick, then whether a command
    /// stamped for that tick had arrived before it ran (a command lost in every copy, or late, gives `false`).
    pub fn on_time(&self) -> Option<bool> {
        self.t.on_time
    }

    /// The last tick's press edges: (seen, lost, doubled), over the commands run (-20's and -58's
    /// `tick.v.press_count`, `.press_lost_count`, `.press_doubled_count`). Seen: each button's counter advance over the
    /// last command run on this slot; lost: the edges beyond the one the sim takes from a changed counter (npt-sim
    /// world.rs: an edge is a counter that differs from the one it last claimed); doubled: an edge on option B's repeat.
    pub fn presses(&self) -> (u32, u32, u32) {
        (self.t.presses[0], self.t.presses[1], self.t.presses[2])
    }

    /// Option A: for each command the last tick ran, its stamp less the tick (positive: run before its stamp; U4F-Q3
    /// (c)'s report). Empty under option B.
    pub fn ran_early(&self) -> &[i32] {
        &self.t.ran_early
    }

    /// The commands the last tick handed the sim for this client (option B's repeat included): the lag switch's
    /// `tick.v.lagswitch_cmds_count`.
    pub fn commands_run(&self) -> u32 {
        self.t.cmds_run
    }

    fn begin_tick(&mut self) {
        self.t.server_hash = None;
        self.t.burst_eviction = false;
        self.t.baseline_age = None;
        self.t.groups = 0;
        self.t.bytes = 0;
        self.t.budget = 0;
        self.t.remainder = 0;
        self.t.own_bits = 0;
        self.t.own_state_bits = 0;
        self.t.own_state_changed = false;
        self.t.overflow_resync = false;
        self.t.skipped = 0;
        self.t.on_time = None;
        self.t.presses = [0; 3];
        self.t.ran_early.clear();
        self.t.cmds_run = 0;
        self.t.rewinds_refused.clear();
        self.t.wraps.clear();
        self.t.wraps.append(&mut self.pending_wraps);
        self.cosmetic.clear();
    }

    fn kick(&mut self, roster: &mut Roster) {
        self.send_control(&Message::Kick(Kick::Overflow));
        self.kicked = true;
        self.counters.kicked += 1;
        roster.leave(self.slot);
    }

    /// Queues `msg` for the control outbox, encoded here, once, with the room's `Quant` (plan.md:508-509: "Encoding
    /// stays on the tick"; the advisor-u4e-r1 R2, A1).
    fn send_control(&mut self, msg: &Message) {
        let mut out = Vec::new();
        if self.encoder.quant.encode(msg, &mut out).is_ok() {
            self.control.push(out);
        }
    }
}

/// `r` as the wire carries it: `area` and `raw` 0.
fn wire(r: &EntityRecord) -> EntityRecord {
    EntityRecord {
        area: 0,
        raw: [0; EntityRecord::FIELDS],
        ..*r
    }
}

/// A record's point for interest (cm): a soldier's or hull's position, a trajectory's start; none for the rest.
fn point(r: &EntityRecord) -> Option<[i32; 2]> {
    match r.kind {
        EntityKind::Soldier | EntityKind::Vehicle => Some([r.grid[0], r.grid[1]]),
        EntityKind::Projectile => Some([r.grid[1], r.grid[2]]),
        _ => None,
    }
}

fn dist2(a: [i32; 2], b: [i32; 2]) -> i64 {
    let (x, y) = (
        i64::from(a[0]) - i64::from(b[0]),
        i64::from(a[1]) - i64::from(b[1]),
    );
    x * x + y * y
}

/// Sorts a tick's commands into the canonical order (seam.rs `TickInput::commands`; carry 21): slot ascending, then
/// within a slot by the command tick's signed distance from the tick `t` being run, so every arrival order gives one
/// `TickInput`. The key is total (U4F-Q17 (a); carry 80 (a)): it orders any set of u32 ticks, even ticks 2^31 or more
/// apart, so the sort never meets a non-total order, and for ticks within 2^31 of `t` it is the wrap-aware order. The
/// sort is stable, so commands equal in both keys keep their arrival order.
pub(crate) fn canonical(commands: &mut [(Command, Option<Rewind>)], t: u32) {
    commands.sort_by_key(|(c, _)| (c.slot, c.tick.wrapping_sub(t) as i32));
}

/// The tick's records by (kind, index), and the records interest places by a point or by their hull or area.
#[derive(Clone, Debug, Default)]
struct Index {
    by_key: Vec<((EntityKind, u16), usize)>,
    parts: Vec<usize>,
    pieces: Vec<usize>,
}

impl Index {
    fn get(&self, k: (EntityKind, u16)) -> Option<usize> {
        self.by_key
            .binary_search_by_key(&k, |e| e.0)
            .ok()
            .map(|i| self.by_key[i].1)
    }
}

/// The room (plan.md:596-598): generic over npt-sim's `SimCore`; it owns its `HitHistory`, its roster and a
/// [`ClientSlot`] per joined client.
#[derive(Clone, Debug)]
pub struct Room<S: SimCore> {
    sim: S,
    history: HitHistory,
    roster: Roster,
    clients: Vec<Option<ClientSlot>>,
    teams: Vec<u8>,
    effect_rows: u16,
    mode_id: u16,
    content_hash: [u8; 32],
    mode_hash: [u8; 32],
    tick_hz: u32,
    next_tick: u32,
    next_event: u32,
    event_cap: u32,
    event_cap_provisional: bool,
    cover: CoverVariant,
    grid: InterestGrid,
    flood: AreaFlood,
    quant: Quant,
    #[cfg(feature = "bites")]
    bite: Option<RoomBite>,
    input: TickInput,
    /// The next tick's scripted spawns, teleports and attribute changes, until `fill_input` moves them into `input`.
    scripted_spawns: Vec<Placement>,
    scripted_teleports: Vec<Placement>,
    scripted_attributes: Vec<AttributeChange>,
    stamped: Vec<(u32, u32, SimEvent)>,
    reports: InputReports,
    index: Index,
    /// OQ-0001-6's option, a start-up setting.
    timing: CmdTiming,
    /// The lead bound in ms (a room setting: LEAD-BOUND) and in ticks at the room's rate (`ticks_for_ms_at`).
    lead_bound_ms: u32,
    lead_bound: u32,
    /// The map's x and y extent, cm ([x0, y0, x1, y1]): kind 6's bounds.
    extent: [i32; 4],
    /// Per slot index, the newest accepted stamp across join, replace, leave and kick (U4F-Q18 (ii) (d)): a fixed
    /// array of the room's slots, never keyed by client data.
    kept_stamps: Vec<Option<u32>>,
    /// Per slot index, the press counters of the last command run (the sim's own claimed counters, which outlive a
    /// session): the press series' reference.
    run_presses: Vec<[u8; 4]>,
    /// The refusal records (slot, seq, kind) of the receives since the last tick began (`incoming`), and those of the
    /// receives before it (`refusals`, what [`Room::refusals`] gives).
    incoming: Vec<(u8, u16, Refusal)>,
    refusals: Vec<(u8, u16, Refusal)>,
    refusals_dropped: u64,
    /// Option A's arrival numbers (its run order before `canonical`), and the tick's scratch list.
    arrivals: u64,
    drained: Vec<(u64, Command)>,
}

impl<S: SimCore> Room<S> {
    /// The lead bound's value, in ms: Mustafa's LEAD-BOUND (a), 400 ms (history.md, 2026-11-18 11:31), frozen: 12 ticks
    /// at 30 Hz, 24 at 60 Hz (`ticks_for_ms_at`). A command stamped more than this ahead of the room's next tick is
    /// refused and counted as kind 1.
    pub const LEAD_BOUND_MS: u32 = 400;

    /// A room over `sim` running `scene`, its static graph read from `greybox` (`areas()` and `portals()`: the
    /// tests pass `World::greybox()`), a roster of `tokens` admitting at most `n` sessions, at `tick_hz` ticks a second
    /// (production callers pass `npt_sim::TICK_HZ`: the window's rate is an argument, the F2 ruling). Its first tick
    /// is the one after the tick `sim`'s hit volumes name (a new `World`'s name the tick before its first step).
    ///
    /// # Panics
    /// If a hit history of 2 s (`HISTORY_MS`, provisional until U4h) does not fit npt-sim's 1 MB bound (it does at
    /// every rate npt-sim allows).
    pub fn new(
        sim: S,
        scene: &Scene,
        greybox: &Greybox,
        tokens: Vec<[u8; 16]>,
        n: usize,
        tick_hz: u32,
    ) -> Room<S> {
        let next_tick = sim.hit_volumes().tick.wrapping_add(1);
        let areas = greybox.areas();
        let extent = areas.rects.first().copied().unwrap_or([0, 0, 1, 1]);
        let mut flood = AreaFlood::new(areas, greybox.portals());
        flood.update(&sim.portal_states().open);
        let slots = usize::from(scene.slots());
        Room {
            history: HitHistory::for_ms(HISTORY_MS)
                .expect("a 2 s hit history fits npt-sim's 1 MB bound"),
            roster: Roster::new(tokens, n),
            clients: vec![None; slots.max(1)],
            teams: (0..scene.slots())
                .map(|s| scene.team(s).unwrap_or(0))
                .collect(),
            effect_rows: u16::from(scene.effect_rows()),
            mode_id: u16::from(scene.id()),
            content_hash: [0; 32],
            mode_hash: [0; 32],
            tick_hz,
            next_tick,
            next_event: 0,
            event_cap: EventQueue::CAP,
            event_cap_provisional: false,
            cover: CoverVariant::Rewound,
            grid: InterestGrid::new(extent, CELL_CM),
            flood,
            quant: Quant::default(),
            #[cfg(feature = "bites")]
            bite: None,
            input: TickInput::default(),
            scripted_spawns: Vec::new(),
            scripted_teleports: Vec::new(),
            scripted_attributes: Vec::new(),
            stamped: Vec::new(),
            reports: InputReports::default(),
            index: Index::default(),
            timing: CmdTiming::B,
            lead_bound_ms: Room::<S>::LEAD_BOUND_MS,
            lead_bound: npt_sim::ticks_for_ms_at(tick_hz, Room::<S>::LEAD_BOUND_MS),
            extent,
            kept_stamps: vec![None; slots.max(1)],
            run_presses: vec![[0; 4]; slots.max(1)],
            incoming: Vec::new(),
            refusals: Vec::new(),
            refusals_dropped: 0,
            arrivals: 0,
            drained: Vec::new(),
            sim,
        }
    }

    /// The same room under OQ-0001-6's option `timing` (a start-up setting: set before any join, never in a run).
    pub fn with_cmd_timing(mut self, timing: CmdTiming) -> Room<S> {
        self.timing = timing;
        self
    }

    /// The same room with its lead bound set as data, in ms, turned into ticks at the room's rate (carry 39's pattern:
    /// `ticks_for_ms_at` with the rate passed in). The judged runs keep [`Room::LEAD_BOUND_MS`].
    pub fn with_lead_bound_ms(mut self, ms: u32) -> Room<S> {
        self.lead_bound_ms = ms;
        self.lead_bound = npt_sim::ticks_for_ms_at(self.tick_hz, ms);
        self
    }

    /// [`Room::with_cmd_timing`] in place, for a caller that holds the room inside its own server (npt-lab): a start-up
    /// setting, given before any join.
    ///
    /// # Panics
    /// In a debug or test build, once any slot has joined (the option never changes in a run: F7).
    pub fn set_cmd_timing(&mut self, timing: CmdTiming) {
        debug_assert!(
            self.clients.iter().all(Option::is_none),
            "the command-timing option is set before any join"
        );
        self.timing = timing;
    }

    /// The room's command-timing option.
    pub fn cmd_timing(&self) -> CmdTiming {
        self.timing
    }

    /// The lead bound: (ms, ticks at the room's rate), for the run's header.
    pub fn lead_bound(&self) -> (u32, u32) {
        (self.lead_bound_ms, self.lead_bound)
    }

    /// The refusal records of the commands received before the last tick began, as (slot, seq, kind), one per refused
    /// command (kind 8: one per command of the refused datagram), in arrival order (plan §4.2 :246-247's form).
    pub fn refusals(&self) -> &[(u8, u16, Refusal)] {
        &self.refusals
    }

    /// Refusal records not kept because more than 4,096 came between two ticks (their counters still count them).
    pub fn refusals_dropped(&self) -> u64 {
        self.refusals_dropped
    }

    /// The checks a planted -21 bite relaxes (none outside bites builds).
    fn relax(&self) -> Relax {
        Relax {
            no_lead_bound: self.bitten(RoomBite::NoLeadBound),
            no_newer_tick: self.bitten(RoomBite::NoNewerTick),
            not_older_than: self.bitten(RoomBite::NotOlderThan),
            no_seq_forward: self.bitten(RoomBite::NoSeqForward),
            no_move_length: self.bitten(RoomBite::NoMoveLength),
            no_press_edge: self.bitten(RoomBite::NoPressEdge),
            no_aim_bounds: self.bitten(RoomBite::NoAimBounds),
            no_catchup_cap: self.bitten(RoomBite::NoCatchupCap),
        }
    }

    /// The same room with the event queue's cap set as data (for the run's header; a test may set a small one). The
    /// cap then is no longer the provisional one.
    pub fn with_event_cap(mut self, cap: u32) -> Room<S> {
        self.event_cap = cap;
        self.event_cap_provisional = false;
        self
    }

    /// The same room with its 32-bit event counter started at `start` (-58's injectable start: carry 23).
    pub fn with_event_start(mut self, start: u32) -> Room<S> {
        self.next_event = start;
        self
    }

    /// The same room with its content and mode hashes for `JoinAccepted` (the scene data and the build).
    pub fn with_hashes(mut self, content_hash: [u8; 32], mode_hash: [u8; 32]) -> Room<S> {
        self.content_hash = content_hash;
        self.mode_hash = mode_hash;
        self
    }

    /// The same room with a cover variant for every Rewind (R-10's two; the room's setting).
    pub fn with_cover(mut self, cover: CoverVariant) -> Room<S> {
        self.cover = cover;
        self
    }

    /// The same room with one planted defect by its id (bites builds only; u4-build-plan.md §4.11: per instance): one
    /// of [`crate::BITES`], or one of npt-proto's, whose `Quant` the room's encoder then codes with (U4E-Q4: the room
    /// forwards the id, never spells it); `None` for an id neither plants.
    #[cfg(feature = "bites")]
    pub fn with_bite(mut self, bite: &str) -> Option<Room<S>> {
        if let Some(b) = RoomBite::of(bite) {
            self.bite = Some(b);
            return Some(self);
        }
        self.quant = Quant::with_bite(bite)?;
        Some(self)
    }

    fn bitten(&self, b: RoomBite) -> bool {
        #[cfg(feature = "bites")]
        {
            self.bite == Some(b)
        }
        #[cfg(not(feature = "bites"))]
        {
            let _ = b;
            false
        }
    }

    /// The counter one of -58's room bites compares with a plain `<` (u4-build-plan.md §4.14 :467, :468, :471): each id
    /// maps to one `Wrap` counter name.
    fn plain_counter(&self) -> Option<&'static str> {
        [
            (RoomBite::LtAck, ACK),
            (RoomBite::LtCmdSeq, CMD_SEQ),
            (RoomBite::LtPress, PRESS),
        ]
        .into_iter()
        .find(|(b, _)| self.bitten(*b))
        .map(|(_, counter)| counter)
    }

    /// The `Wrap` the room's compares run through: the rule, or under one of -58's room bites a plain `<` on its one
    /// counter.
    fn wrap(&self) -> Wrap {
        let plain = self.plain_counter();
        #[cfg(feature = "bites")]
        if let Some(counter) = plain {
            return Wrap::plain_lt(counter);
        }
        let _ = plain;
        Wrap::default()
    }

    /// The event queue's cap, and whether it is still the provisional value (U4E-Q6 (a); carry 57: for the run's
    /// header).
    pub fn event_cap(&self) -> (u32, bool) {
        (self.event_cap, self.event_cap_provisional)
    }

    /// The sim.
    pub fn sim(&self) -> &S {
        &self.sim
    }

    /// The roster.
    pub fn roster(&self) -> &Roster {
        &self.roster
    }

    /// The tick the last [`Room::tick`] ran (the one before the first, before any).
    pub fn last_tick(&self) -> u32 {
        self.next_tick.wrapping_sub(1)
    }

    /// The last tick's `TickInput`, whole (what a recording holds: the commands in the canonical order with their
    /// Rewinds, and the tick's scripted spawns, teleports and attribute changes).
    pub fn tick_input(&self) -> &TickInput {
        &self.input
    }

    /// The last step's clamp reports, refusal records and running counters, read through `SimCore::reports` and kept
    /// for the logs, never in a hash.
    pub fn reports(&self) -> &InputReports {
        &self.reports
    }

    /// A client's slot, while it is joined (or kicked and not yet gone).
    pub fn client(&self, slot: u8) -> Option<&ClientSlot> {
        self.clients.get(usize::from(slot)).and_then(Option::as_ref)
    }

    /// The datagrams (snapshot groups) the last tick coded for `slot`, in group order.
    pub fn take_datagrams(&mut self, slot: u8) -> Vec<Vec<u8>> {
        self.clients
            .get_mut(usize::from(slot))
            .and_then(Option::as_mut)
            .map(|c| core::mem::take(&mut c.datagrams))
            .unwrap_or_default()
    }

    /// The reliable messages for `slot` (`JoinAccepted`, `FullState`, `Kick`), each encoded once, when the room queued
    /// it, with the room's `Quant` (plan.md:508-509: "Encoding stays on the tick"). The caller sends each on the
    /// channel its type byte's row of `Limits::TYPES` names (npt-proto's rule; the type byte follows the stream's
    /// length prefix).
    pub fn take_control(&mut self, slot: u8) -> Vec<Vec<u8>> {
        self.clients
            .get_mut(usize::from(slot))
            .and_then(Option::as_mut)
            .map(|c| core::mem::take(&mut c.control))
            .unwrap_or_default()
    }

    /// The next tick's scripted spawns, teleports and attribute changes (integers only: seam.rs): they wait here and
    /// go into that tick's `TickInput` (so [`Room::tick_input`] after the tick holds them).
    pub fn script(
        &mut self,
        spawns: &[Placement],
        teleports: &[Placement],
        attributes: &[AttributeChange],
    ) {
        self.scripted_spawns.clear();
        self.scripted_spawns.extend_from_slice(spawns);
        self.scripted_teleports.clear();
        self.scripted_teleports.extend_from_slice(teleports);
        self.scripted_attributes.clear();
        self.scripted_attributes.extend_from_slice(attributes);
    }

    /// A `Join` the pump passed (its version and content hash already checked there): the roster's checks, then a
    /// fresh slot with `JoinAccepted` and a full state to follow. Returns the slot and whether it replaced a live
    /// session (whose connection then gets `Kick(replaced)` from the caller).
    ///
    /// # Errors
    /// The roster's [`JoinRejected`] reason: bad token, bind mismatch, roster full.
    pub fn join(&mut self, join: &Join) -> Result<(u8, bool), JoinRejected> {
        let a = self.roster.admit(&join.token, &join.bind)?;
        let i = usize::from(a.slot);
        if i >= self.clients.len() {
            self.roster.leave(a.slot);
            return Err(JoinRejected::RosterFull);
        }
        let mut ack = AckRing::new();
        if self.bitten(RoomBite::NoFallback) {
            ack.keep_stale();
        }
        let mut c = ClientSlot {
            slot: a.slot,
            team: self.teams.get(i).copied().unwrap_or(0),
            ack,
            queue: EventQueue::new(self.event_cap, self.bitten(RoomBite::Unbounded)),
            input: InputBuffer::new(self.timing, self.next_tick, self.kept_stamps[i]),
            encoder: Encoder::new(self.tick_hz, self.quant),
            priority: Priority::default(),
            full_state_due: Some(false),
            in_flight: None,
            last_full_state: None,
            kicked: false,
            server_bytes: 0,
            max_datagram: Limits::SNAPSHOT,
            datagrams: Vec::new(),
            control: Vec::new(),
            counters: ClientCounters::default(),
            last_ack_hi: None,
            pending_wraps: Vec::new(),
            fire_presses: None,
            t: TickValues::default(),
            relevant: Vec::new(),
            sees: Vec::new(),
            nearest: Vec::new(),
            cosmetic: Vec::new(),
        };
        c.send_control(&Message::JoinAccepted(JoinAccepted {
            slot: a.slot,
            team: c.team,
            squad: 0,
            mode_id: self.mode_id,
            mode_hash: self.mode_hash,
            content_hash: self.content_hash,
            tick: self.next_tick,
            tick_hz: u16::try_from(self.tick_hz).unwrap_or(u16::MAX),
            snapshot_hz: u16::try_from(self.tick_hz).unwrap_or(u16::MAX),
        }));
        self.clients[i] = Some(c);
        Ok((a.slot, a.replaced))
    }

    /// The slot's session ended (a `Leave`, a closed link): the slot is free, its token and bind value kept.
    pub fn leave(&mut self, slot: u8) {
        self.roster.leave(slot);
        if let Some(c) = self.clients.get_mut(usize::from(slot)) {
            *c = None;
        }
    }

    /// The pump's report for `slot` before a tick (u4-build-plan.md §4.5; plan.md §4.4): the bytes the server side
    /// sent it since the last tick (pings and pongs, payload plus 28 B each), which the window counts, and the link's
    /// largest datagram now, which bounds each group.
    pub fn pump_report(&mut self, slot: u8, sent_bytes: u32, max_datagram: usize) {
        if let Some(c) = self
            .clients
            .get_mut(usize::from(slot))
            .and_then(Option::as_mut)
        {
            c.server_bytes = c.server_bytes.saturating_add(sent_bytes);
            c.max_datagram = max_datagram;
        }
    }

    /// One decoded message from `slot`'s session (the pump decoded it; npt-room sees only decoded messages): `Input`'s
    /// acknowledgement and commands, `Resync`, `Leave`; the other client types change nothing here at U4e.
    pub fn receive(&mut self, slot: u8, msg: &Message) {
        match msg {
            Message::Input(input) => self.receive_input(slot, input),
            Message::Resync(_) => {
                let next = self.next_tick;
                if let Some(c) = self
                    .clients
                    .get_mut(usize::from(slot))
                    .and_then(Option::as_mut)
                    && !c.kicked
                {
                    if c.in_flight.is_some() || c.full_state_due.is_some() {
                        c.counters.resyncs_dropped += 1;
                    } else if c
                        .last_full_state
                        .is_some_and(|f| next.wrapping_sub(f) < npt_sim::FULL_STATE_ACK_TICKS)
                    {
                        // SEC-U4E-4's room half (advisor-u4e-r1 R3): the answer, at the next tick, would come sooner
                        // than FULL_STATE_ACK_TICKS ticks after the last full state: at most one in that span.
                        c.counters.resyncs_limited += 1;
                    } else {
                        c.full_state_due = Some(false);
                        c.counters.resyncs += 1;
                    }
                }
            }
            Message::Leave(_) => self.leave(slot),
            _ => {}
        }
    }

    fn receive_input(&mut self, slot: u8, input: &Input) {
        let w = self.wrap();
        let mut checks = Checks {
            next_tick: self.next_tick,
            lead_bound: self.lead_bound,
            extent: self.extent,
            arrival: self.arrivals,
            relax: self.relax(),
        };
        let no_ack_sent = self.bitten(RoomBite::NoAckSent);
        let records = &mut self.incoming;
        let dropped = &mut self.refusals_dropped;
        let mut record = |seq: u16, kind: Refusal| {
            if records.len() < REFUSALS_HELD {
                records.push((slot, seq, kind));
            } else {
                *dropped += 1;
            }
        };
        let Some(c) = self
            .clients
            .get_mut(usize::from(slot))
            .and_then(Option::as_mut)
        else {
            return;
        };
        if c.kicked {
            return;
        }
        if input.ack_groups != 0 {
            let mut acked = Vec::new();
            let answer = match c.ack.newest() {
                Some(near) => {
                    let tick = w.widen16(ACK, near, input.ack_tick);
                    let a = c.ack.ack(tick, input.ack_groups, &mut acked);
                    if a != Acked::NeverSent {
                        let hi = (tick >> 16) as u16;
                        if c.last_ack_hi.is_some_and(|h| h != hi) {
                            c.pending_wraps.push(ACK);
                        }
                        c.last_ack_hi = Some(hi);
                    }
                    a
                }
                None => Acked::NeverSent,
            };
            match answer {
                Acked::Held => {
                    c.queue.ack(&acked);
                    if let Some((f, _)) = c.in_flight
                        && c.ack.acked(f)
                    {
                        c.in_flight = None;
                    }
                }
                Acked::Old => c.counters.acks_old += 1,
                Acked::NeverSent if !no_ack_sent => {
                    // Kind 8: the acknowledgement names a snapshot never sent: the datagram's commands are refused
                    // whole (§4.14 :471), counted by datagram, each command recorded.
                    c.counters.ack_refused += 1;
                    c.counters.ack_refused_commands += input.commands.len() as u64;
                    for cmd in input.commands.iter().rev() {
                        record(cmd.seq, Refusal::AckNeverSent);
                    }
                    return;
                }
                // PM-0001-21/no-ack-sent: the check removed, the commands go on.
                Acked::NeverSent => {}
            }
        }
        // Per command, oldest first (2.4 item 2).
        for cmd in input.commands.iter().rev() {
            let cmd = Command { slot, ..*cmd };
            let k = &mut c.counters;
            match c.input.offer(cmd, w, &mut c.pending_wraps, &checks) {
                Offered::Accepted => {
                    k.commands_buffered += 1;
                    let kept = &mut self.kept_stamps[usize::from(slot)];
                    if kept.is_none_or(|s| dist(cmd.tick, s) > 0) {
                        *kept = Some(cmd.tick);
                    }
                }
                Offered::Repeat => k.commands_repeat += 1,
                Offered::OutsideWindow => k.seq_outside_window += 1,
                Offered::RejoinOverlap => k.rejoin_overlap += 1,
                Offered::Late => k.late += 1,
                Offered::Refused(r) => {
                    match r {
                        Refusal::LeadBound => k.lead_refused += 1,
                        Refusal::NotNewer => k.not_newer += 1,
                        Refusal::SeqNotForward => k.seq_refused += 1,
                        Refusal::MoveLength => k.move_refused += 1,
                        Refusal::PressEdge => k.press_refused += 1,
                        Refusal::AimBounds => k.aim_refused += 1,
                        Refusal::Catchup => k.buffer_full += 1,
                        Refusal::AckNeverSent => k.ack_refused_commands += 1,
                    }
                    record(cmd.seq, r);
                }
            }
            checks.arrival += 1;
        }
        self.arrivals = checks.arrival;
    }

    /// Runs one tick: the commands, the step, the history, the flood, the reports, interest, the events and every
    /// client's snapshot (the module text).
    pub fn tick(&mut self) {
        let t = self.next_tick;
        for c in self.clients.iter_mut().flatten() {
            c.begin_tick();
        }
        self.refusals.clear();
        core::mem::swap(&mut self.incoming, &mut self.refusals);
        self.fill_input(t);
        // Carry 26: the step's events are copied (stamped with the tick and their 32-bit numbers: carry 23) before
        // anything else is read from the sim.
        self.stamped.clear();
        let events = self.sim.step(&self.input, &self.history);
        for e in events {
            self.stamped.push((self.next_event, t, *e));
            self.next_event = self.next_event.wrapping_add(1);
        }
        self.history.push(self.sim.hit_volumes());
        self.flood.update(&self.sim.portal_states().open);
        self.reports.clone_from(self.sim.reports());
        self.index_records();
        self.find_relevant();
        self.route_events(t);
        self.encode_all(t);
        self.next_tick = t.wrapping_add(1);
    }

    /// The tick's `TickInput`: the script waiting for it, and per joined client the commands its option runs (option B:
    /// the one stamped for the tick, or the last again; option A: every command accepted since the last tick, taken
    /// in arrival order), each with its slot the session's and its Rewind when it holds or presses fire, then the
    /// canonical order. Before it, each client's `tick.s.cmd_on_time` by stamp.
    fn fill_input(&mut self, t: u32) {
        self.input.tick = t;
        self.input.spawns.clear();
        self.input.spawns.append(&mut self.scripted_spawns);
        self.input.teleports.clear();
        self.input.teleports.append(&mut self.scripted_teleports);
        self.input.attributes.clear();
        self.input.attributes.append(&mut self.scripted_attributes);
        self.input.commands.clear();
        let mut drained = core::mem::take(&mut self.drained);
        drained.clear();
        for c in self.clients.iter_mut().flatten() {
            if c.kicked {
                continue;
            }
            c.t.on_time = c.input.due(t);
            if self.timing == CmdTiming::A {
                c.input.drain(t, &mut drained);
                continue;
            }
            let taken = c.input.take(t);
            c.counters.late += u64::from(taken.late);
            let Some(cmd) = taken.command else {
                continue;
            };
            if taken.repeated {
                c.counters.commands_rerun += 1;
            } else {
                c.counters.commands_run += 1;
            }
            let presses = &mut self.run_presses[usize::from(c.slot)];
            let run = run_one(c, cmd, taken.repeated, presses, &self.history, self.cover);
            self.input.commands.push(run);
        }
        // Option A: the commands in the order they arrived (the arrival numbers are unique), each slot's in its
        // accepted order; `canonical` then gives every arrival order one list.
        drained.sort_unstable_by_key(|(arrival, _)| *arrival);
        for &(_, cmd) in &drained {
            let Some(c) = self
                .clients
                .get_mut(usize::from(cmd.slot))
                .and_then(Option::as_mut)
            else {
                continue;
            };
            c.counters.commands_run += 1;
            c.t.ran_early.push(cmd.tick.wrapping_sub(t) as i32);
            let presses = &mut self.run_presses[usize::from(c.slot)];
            let run = run_one(c, cmd, false, presses, &self.history, self.cover);
            self.input.commands.push(run);
        }
        self.drained = drained;
        canonical(&mut self.input.commands, t);
    }

    fn index_records(&mut self) {
        let records = self.sim.entity_records();
        let ix = &mut self.index;
        ix.by_key.clear();
        ix.parts.clear();
        ix.pieces.clear();
        self.grid.clear();
        for (i, r) in records.iter().enumerate() {
            ix.by_key.push((key(r), i));
            match r.kind {
                EntityKind::VehiclePart => ix.parts.push(i),
                EntityKind::Door | EntityKind::Wall => ix.pieces.push(i),
                _ => {}
            }
            if let Some(p) = point(r) {
                self.grid.insert(p, i as u32);
            }
        }
        ix.by_key.sort_by_key(|e| e.0);
    }

    /// Each client's relevant records (catalogue §3's spatial rows): soldiers, hulls and trajectories within the
    /// radius whose area connects to the client's; a part with its hull; a door or the wall in a connected area (a
    /// portal counts as inside both areas it joins). The nearest 8 enemy soldiers among them, for the priority.
    fn find_relevant(&mut self) {
        let records = self.sim.entity_records();
        let r2 = i64::from(RADIUS_CM) * i64::from(RADIUS_CM);
        let mut near = Vec::new();
        let slots = self.teams.len();
        for c in self.clients.iter_mut().flatten() {
            c.relevant.clear();
            c.nearest.clear();
            c.sees.clear();
            c.sees.resize(slots, false);
            if c.kicked {
                continue;
            }
            let Some(me) = self.index.get((EntityKind::Soldier, u16::from(c.slot))) else {
                continue;
            };
            let me = &records[me];
            let at = [me.grid[0], me.grid[1]];
            let mut hulls = Vec::new();
            let mut enemies = Vec::new();
            self.grid.near(at, RADIUS_CM, &mut near);
            for &i in &near {
                let r = &records[i as usize];
                if r.kind == EntityKind::Soldier && r.index == u16::from(c.slot) {
                    continue;
                }
                let Some(p) = point(r) else { continue };
                let d = dist2(at, p);
                if d > r2 || !self.flood.reaches(me.area, r.area) {
                    continue;
                }
                c.relevant.push(i as usize);
                match r.kind {
                    EntityKind::Soldier => {
                        if let Some(s) = c.sees.get_mut(usize::from(r.index)) {
                            *s = true;
                        }
                        if self
                            .teams
                            .get(usize::from(r.index))
                            .is_some_and(|t| *t != c.team)
                        {
                            enemies.push((d, r.index));
                        }
                    }
                    EntityKind::Vehicle => hulls.push(r.index),
                    _ => {}
                }
            }
            for &i in &self.index.parts {
                if u16::try_from(records[i].grid[5]).is_ok_and(|h| hulls.contains(&h)) {
                    c.relevant.push(i);
                }
            }
            for &i in &self.index.pieces {
                let r = &records[i];
                let seen = match r.kind {
                    EntityKind::Door => self.flood.sees_portal(me.area, usize::from(r.index)),
                    _ => self.flood.reaches(me.area, r.area),
                };
                if seen {
                    c.relevant.push(i);
                }
            }
            c.relevant.sort_by_key(|&i| key(&records[i]));
            enemies.sort_unstable();
            c.nearest
                .extend(enemies.iter().take(Priority::NEAREST_ENEMIES).map(|e| e.1));
        }
    }

    /// The tick's events to each client (catalogue §3): a gameplay event (hit, kill) into the queue of every client it
    /// concerns (shooter or victim) or whose interest holds a soldier it names; a cosmetic shot to every client whose
    /// interest holds the shooter, never the shooter's own (but under PM-0001-62/server-echo). An overflow forces one
    /// resync, or, inside an unacknowledged forced one, kicks (§4.7).
    fn route_events(&mut self, t: u32) {
        let echo = self.bitten(RoomBite::ServerEcho);
        for c in self.clients.iter_mut().flatten() {
            if c.kicked {
                continue;
            }
            c.counters.events_expired += u64::from(c.queue.expire(t));
            let me = c.slot;
            let sees = |s: u8| c.sees.get(usize::from(s)).copied().unwrap_or(false);
            let mut gameplay = Vec::new();
            for &(number, tick, event) in &self.stamped {
                match event {
                    SimEvent::Shot { slot, .. } => {
                        if (slot != me || echo) && (sees(slot) || slot == me) {
                            c.cosmetic.push((number, event));
                        }
                    }
                    SimEvent::Hit {
                        shooter, victim, ..
                    }
                    | SimEvent::Kill {
                        killer: shooter,
                        victim,
                    } => {
                        if shooter == me || victim == me || sees(shooter) || sees(victim) {
                            gameplay.push(Queued {
                                number,
                                tick,
                                event,
                                sent: 0,
                            });
                        }
                    }
                }
            }
            c.t.headroom = c.queue.headroom();
            for q in gameplay {
                if c.queue.push(q).is_ok() {
                    c.counters.events_queued += 1;
                    c.t.headroom = c.t.headroom.min(c.queue.headroom());
                    continue;
                }
                // At the cap: the headroom is 0 at the overflow.
                c.t.headroom = c.t.headroom.min(0);
                if c.full_state_due == Some(true) || c.in_flight.is_some_and(|f| f.1) {
                    // A second overflow inside an unacknowledged forced resync: Kick(overflow), the client dropped.
                    c.kick(&mut self.roster);
                } else {
                    // One resync, sent this tick: the full state holds this tick's results and clears the queue
                    // (catalogue Q8), so the rest of this tick's events are not queued.
                    c.queue.clear();
                    c.full_state_due = Some(true);
                    c.in_flight = None;
                    c.counters.overflow_resyncs += 1;
                    c.t.overflow_resync = true;
                }
                break;
            }
        }
    }

    fn encode_all(&mut self, t: u32) {
        let owner_to_all = self.bitten(RoomBite::OwnerToAll);
        let no_fallback = self.bitten(RoomBite::NoFallback);
        let records = self.sim.entity_records();
        let slots = self.teams.len();
        let full_ack = npt_sim::FULL_STATE_ACK_TICKS;
        for c in self.clients.iter_mut().flatten() {
            if c.kicked {
                continue;
            }
            // The window moves on every tick, and the pump's bytes count against this tick's whether or not a
            // snapshot goes out (u4-build-plan.md §4.5).
            let (budget, left) = c.encoder.remainder(c.server_bytes);
            c.server_bytes = 0;
            c.t.budget = budget;
            c.t.remainder = left;
            c.t.burst_eviction = c.ack.evicted(t);
            if let Some((f, true)) = c.in_flight
                && t.wrapping_sub(f) > full_ack
                && !c.ack.acked(f)
            {
                c.kick(&mut self.roster);
                continue;
            }
            // The own-state section's slot: the client's own (carry 53), or under the owner-to-all bite the next one's.
            let owner = if owner_to_all && slots > 1 {
                ((usize::from(c.slot) + 1) % slots) as u16
            } else {
                u16::from(c.slot)
            };
            let mut own = Vec::new();
            let mut push = |k: (EntityKind, u16)| {
                if let Some(i) = self.index.get(k) {
                    own.push(records[i]);
                }
            };
            push((EntityKind::Soldier, owner));
            // Carry 51: effect row i belongs to slot i / Scene::effect_rows().
            let rows = self.effect_rows;
            for i in owner.saturating_mul(rows)..owner.saturating_add(1).saturating_mul(rows) {
                push((EntityKind::Effect, i));
            }
            push((EntityKind::OwnState, owner));
            if let Some(forced) = c.full_state_due {
                full_state(c, t, forced, records, &own);
                continue;
            }
            if c.in_flight.is_some() {
                continue;
            }
            snapshot(c, t, records, &own, no_fallback);
        }
    }
}

/// One command run for `c` this tick (option B's repeat when `repeated`): its press edges counted against the slot's
/// last run counters `presses` (seen, lost beyond the one edge the sim takes, doubled on a repeat), the fire-press
/// tracking, and its Rewind when it holds or presses fire.
fn run_one(
    c: &mut ClientSlot,
    cmd: Command,
    repeated: bool,
    presses: &mut [u8; 4],
    history: &HitHistory,
    cover: CoverVariant,
) -> (Command, Option<Rewind>) {
    for (k, &p) in cmd.presses.iter().enumerate() {
        let edges = u32::from(p.wrapping_sub(presses[k]));
        if repeated {
            c.t.presses[2] += edges;
        } else {
            c.t.presses[0] += edges;
            c.t.presses[1] += edges.saturating_sub(1);
        }
    }
    if !repeated {
        *presses = cmd.presses;
    }
    c.t.cmds_run += 1;
    let press = cmd.presses[Command::PRESS_FIRE];
    let pressed =
        c.fire_presses.is_some_and(|p| p != press) || (c.fire_presses.is_none() && press != 0);
    c.fire_presses = Some(press);
    let rewind = if cmd.buttons & Command::FIRE != 0 || pressed {
        let r = rewind_at(&cmd, history, cover);
        if r.is_none() {
            c.counters.rewinds_refused += 1;
            c.t.rewinds_refused.push(cmd.seq);
        }
        r
    } else {
        None
    };
    (
        Command {
            slot: c.slot,
            ..cmd
        },
        rewind,
    )
}

/// Sends `c` a full state at tick `t`: its own-state records and its relevant records whole (catalogue §3's
/// `FullState` rows), recorded in the ring as one acknowledged-group tick, the queue cleared.
fn full_state(
    c: &mut ClientSlot,
    t: u32,
    forced: bool,
    records: &[EntityRecord],
    own: &[EntityRecord],
) {
    let mut frame = Frame {
        own: own.iter().map(wire).collect(),
        entities: c.relevant.iter().map(|&i| wire(&records[i])).collect(),
    };
    frame.own.sort_by_key(key);
    frame.entities.sort_by_key(key);
    let fs = FullState {
        tick: t,
        last_run: c.input.last_run().map(|cmd| cmd.seq),
        own: frame.own.clone(),
        entities: frame.entities.clone(),
    };
    let msg = Message::FullState(fs);
    // Encoded once, here, with the room's `Quant`: the bytes the client gets (A1), and a refusal counted (carry 54).
    let mut bytes = Vec::new();
    if let Err(e) = c.encoder.quant.encode(&msg, &mut bytes) {
        count_refusal(&mut c.counters, e);
        return;
    }
    c.full_state_due = None;
    c.in_flight = Some((t, forced));
    c.last_full_state = Some(t);
    c.queue.clear();
    c.counters.full_states += 1;
    c.t.server_hash = Some(client_hash(t, &frame.entities, &frame.own, &[]));
    c.ack.record(t, 1, frame, Vec::new());
    c.control.push(bytes);
}

fn count_refusal(counters: &mut ClientCounters, e: EncodeError) {
    match e {
        EncodeError::OutOfRange => counters.refused_out_of_range += 1,
        EncodeError::TooMany => counters.refused_too_many += 1,
        EncodeError::TooLarge => counters.refused_too_large += 1,
        EncodeError::NotCanonical => counters.refused_not_canonical += 1,
    }
}

/// One entity change waiting for room in the snapshot, with its priority.
struct Candidate {
    delta: EntityDelta,
    id: (EntityKind, u16),
    nearest: bool,
    skipped: u32,
    dist: i64,
}

/// Codes `c`'s snapshot of tick `t` against its baseline (u4-build-plan.md §4.5), within the window's remainder, and
/// keeps the mirror's hash of what it means the client to hold (§4.6). A refusal by the encoder skips this tick's
/// snapshot for this client, counted by kind (carry 54), and the next tick codes afresh.
fn snapshot(
    c: &mut ClientSlot,
    t: u32,
    records: &[EntityRecord],
    own: &[EntityRecord],
    no_fallback: bool,
) {
    let left = c.t.remainder;
    let empty = Frame::default();
    let (base, age) = match c.ack.baseline(t) {
        Some(b) => (&b.frame, t.wrapping_sub(b.tick) as u16),
        None => match c.ack.stale() {
            // PM-0001-18/no-fallback: never fall back to the defaults, coding against the evicted baseline.
            Some(s) if no_fallback => (&s.frame, t.wrapping_sub(s.tick).max(1) as u16),
            _ => (&empty, 0),
        },
    };
    let from_defaults = age == 0;
    let limit = c.max_datagram.min(Limits::SNAPSHOT) as u32;
    let Some(mut p) = Packer::new(limit, left) else {
        c.counters.budget_short += 1;
        return;
    };
    let mut groups_own = Vec::new();
    let mut own_bits = 0;
    for r in own {
        if let Some(d) = EntityDelta::between(r, find(&base.own, key(r))) {
            let bits = delta_bits(&d);
            own_bits += bits;
            if r.kind == EntityKind::OwnState {
                c.t.own_state_bits = bits;
                c.t.own_state_changed = true;
            }
            groups_own.push((d, bits));
        }
    }
    for b in &base.own {
        if !own.iter().any(|r| key(r) == key(b)) {
            let d = EntityDelta::Remove {
                kind: b.kind,
                index: b.index,
            };
            own_bits += delta_bits(&d);
            groups_own.push((d, delta_bits(&d)));
        }
    }
    for (d, bits) in groups_own {
        if p.place(bits, Section::Own).is_none() {
            c.counters.budget_short += 1;
            return;
        }
        p.groups[0].own.push(d);
    }
    c.t.own_bits = own_bits;
    // Gameplay events, oldest first, resent until acknowledged.
    let (mut sent, mut resent) = (0, 0);
    for q in c.queue.iter_mut() {
        // Carry 50 (RR-2): `route_events` dropped every event older than 255 ticks this tick, so the age fits the
        // wire's 8 bits; one that does not is never coded.
        let event_age = u8::try_from(t.wrapping_sub(q.tick));
        debug_assert!(
            event_age.is_ok(),
            "an event older than 255 ticks reached the encoder (carry 50)"
        );
        let Ok(event_age) = event_age else {
            continue;
        };
        let Some(g) = p.place(event_bits(&q.event), Section::Events) else {
            break;
        };
        p.groups[g]
            .events
            .push((q.number as u16, event_age, q.event));
        p.groups[g].numbers.push(q.number);
        p.groups[g].gameplay.push(q.number);
        q.sent += 1;
        if q.sent == 1 {
            sent += 1;
        } else {
            resent += 1;
        }
    }
    // Entity changes: the nearest 8 enemies first, then by snapshots skipped and distance; then the removals.
    let me = own.first().map_or([0, 0], |r| [r.grid[0], r.grid[1]]);
    let mut candidates = Vec::new();
    let mut held = Vec::new();
    for &i in &c.relevant {
        let r = &records[i];
        let id = key(r);
        match EntityDelta::between(r, find(&base.entities, id)) {
            Some(delta) => candidates.push(Candidate {
                delta,
                id,
                nearest: r.kind == EntityKind::Soldier && c.nearest.contains(&r.index),
                skipped: c.priority.skipped(id),
                dist: point(r).map_or(0, |q| dist2(me, q)),
            }),
            None => held.push(id),
        }
    }
    for b in &base.entities {
        let id = key(b);
        if c.relevant
            .binary_search_by_key(&id, |&i| key(&records[i]))
            .is_err()
        {
            candidates.push(Candidate {
                delta: EntityDelta::Remove {
                    kind: b.kind,
                    index: b.index,
                },
                id,
                nearest: false,
                skipped: Priority::EVERY,
                dist: 0,
            });
        }
    }
    candidates.sort_by(|a, b| {
        b.nearest
            .cmp(&a.nearest)
            .then((b.skipped >= Priority::EVERY - 1).cmp(&(a.skipped >= Priority::EVERY - 1)))
            .then(b.skipped.cmp(&a.skipped))
            .then(a.dist.cmp(&b.dist))
            .then(a.id.cmp(&b.id))
    });
    let mut skipped = 0;
    for cand in candidates {
        match p.place(delta_bits(&cand.delta), Section::Entities) {
            Some(g) => {
                p.groups[g].entities.push(cand.delta);
                c.priority.sent(cand.id);
            }
            None => {
                c.priority.skip(cand.id);
                skipped += 1;
            }
        }
    }
    for id in held {
        c.priority.sent(id);
    }
    // The accumulator keeps no entity the client no longer has in view (bounded by the sim's entities).
    c.priority.retain(|id| {
        c.relevant
            .binary_search_by_key(&id, |&i| key(&records[i]))
            .is_ok()
    });
    // Cosmetic events, sent once if there is room.
    for &(number, event) in &c.cosmetic {
        match p.place(event_bits(&event), Section::Events) {
            Some(g) => {
                p.groups[g].events.push((number as u16, 0, event));
                p.groups[g].numbers.push(number);
                c.counters.cosmetic_sent += 1;
            }
            None => c.counters.cosmetic_skipped += 1,
        }
    }
    // Code each group; any refusal skips the whole tick for this client (carry 54).
    let n = p.groups.len() as u8;
    let mut datagrams = Vec::with_capacity(p.groups.len());
    let mut bytes = 0;
    let last_run = c.input.last_run().map(|cmd| cmd.seq);
    for (i, g) in p.groups.iter().enumerate() {
        let msg = Message::Snapshot(Snapshot {
            tick: t,
            baseline_age: age,
            newest_received: c.input.newest_received(),
            last_run,
            group: i as u8,
            groups: n,
            own: g.own.clone(),
            entities: g.entities.clone(),
            events: g.events.clone(),
        });
        let mut out = Vec::new();
        let coded = c.encoder.quant.encode(&msg, &mut out).and_then(|()| {
            debug_assert_eq!(
                out.len() as u32,
                group_bytes(g.bits),
                "the size arithmetic is the encoding's"
            );
            if out.len() as u32 > limit {
                Err(EncodeError::TooLarge)
            } else {
                Ok(())
            }
        });
        if let Err(e) = coded {
            count_refusal(&mut c.counters, e);
            return;
        }
        bytes += out.len() as u32 + DATAGRAM_OVERHEAD;
        datagrams.push(out);
    }
    // The mirror (§4.6's server side): the baseline plus every group's delta, the events as sent.
    let frame = view(base, &p.groups);
    let events: Vec<(u32, SimEvent)> = p
        .groups
        .iter()
        .flat_map(|g| g.numbers.iter().zip(&g.events).map(|(n, e)| (*n, e.2)))
        .collect();
    c.t.server_hash = Some(client_hash(t, &frame.entities, &frame.own, &events));
    let acks: Vec<(u8, u32)> = p
        .groups
        .iter()
        .enumerate()
        .flat_map(|(i, g)| g.gameplay.iter().map(move |n| (i as u8, *n)))
        .collect();
    if from_defaults {
        c.counters.defaults_coded += 1;
    } else if c.ack.baseline(t).is_none() {
        c.counters.stale_coded += 1;
    }
    c.ack.record(t, n, frame, acks);
    c.counters.snapshots += 1;
    c.counters.groups += u64::from(n);
    c.counters.split_ticks += u64::from(n > 1);
    c.counters.skipped += u64::from(skipped);
    c.counters.events_sent += sent;
    c.counters.events_resent += resent;
    c.t.skipped = skipped;
    c.t.groups = n;
    c.t.bytes = bytes;
    c.t.baseline_age = Some(age);
    c.datagrams = datagrams;
}

#[cfg(test)]
mod tests {
    use super::*;
    use npt_sim::World;

    fn cmd(slot: u8, tick: u32) -> (Command, Option<Rewind>) {
        (
            Command {
                slot,
                tick,
                ..Command::default()
            },
            None,
        )
    }

    /// Carry 21: commands received in two orders give one canonical list, two of one slot whose ticks straddle the
    /// 32-bit tick wrap among them (seam.rs `TickInput::commands`: wrap-aware within a slot).
    #[test]
    fn carry_21_two_arrival_orders_sort_to_one_canonical_list_across_the_32_bit_wrap() {
        let mut a = vec![cmd(3, 1), cmd(0, 7), cmd(3, u32::MAX), cmd(1, 5), cmd(3, 0)];
        let mut b = a.clone();
        b.reverse();
        canonical(&mut a, 0);
        canonical(&mut b, 0);
        assert_eq!(a, b);
        let order: Vec<(u8, u32)> = a.iter().map(|(c, _)| (c.slot, c.tick)).collect();
        assert_eq!(order, [(0, 7), (1, 5), (3, u32::MAX), (3, 0), (3, 1)]);
    }

    /// f8 (carry 80 (a)): one slot's commands 2^31 ticks apart sort to one order in both arrival orders, with no panic
    /// (the wrap-aware comparator had no order for them: U4F-Q17).
    #[test]
    fn f8_one_slots_commands_2_pow_31_ticks_apart_sort_to_one_order() {
        for t in [0u32, 1, 1 << 31, u32::MAX] {
            let mut a = vec![cmd(2, t), cmd(2, t.wrapping_add(1 << 31)), cmd(1, t)];
            let mut b = a.clone();
            b.reverse();
            canonical(&mut a, t);
            canonical(&mut b, t);
            assert_eq!(a, b, "at {t}");
            assert_eq!(
                a[1].0.tick,
                t.wrapping_add(1 << 31),
                "the far tick first: -2^31 from t"
            );
        }
    }

    /// f8 (carry 80 (a); U4F-Q17 (a)), the property half: 10,000 seeded random triples of one slot's u32 ticks (any
    /// distance apart, 2^31 included), each in all six arrival orders, sort to one order with no panic, and that
    /// order is by signed distance from the tick run. Seeded splitmix64 (seed 0x5eed_00f8), 10,000 cases.
    #[test]
    fn f8_one_slots_random_u32_ticks_sort_to_one_order_in_every_arrival_order() {
        let mut s: u64 = 0x5eed_00f8;
        let mut next = || {
            s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = s;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        };
        let perms = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];
        for case in 0..10_000 {
            let t = next() as u32;
            // Every fourth case a triple with two ticks exactly 2^31 apart (no wrap-aware order exists there).
            let a = next() as u32;
            let b = if case % 4 == 0 {
                a.wrapping_add(1 << 31)
            } else {
                next() as u32
            };
            let ticks = [a, b, next() as u32];
            let mut first: Option<Vec<u32>> = None;
            for p in perms {
                let mut list: Vec<(Command, Option<Rewind>)> =
                    p.iter().map(|&i| cmd(4, ticks[i])).collect();
                canonical(&mut list, t);
                let got: Vec<u32> = list.iter().map(|(c, _)| c.tick).collect();
                let keys: Vec<i32> = got.iter().map(|x| x.wrapping_sub(t) as i32).collect();
                assert!(
                    keys.windows(2).all(|w| w[0] <= w[1]),
                    "case {case}: {got:?} at {t}"
                );
                match &first {
                    None => first = Some(got),
                    Some(f) => {
                        assert_eq!(f, &got, "case {case}: one order for every arrival order")
                    }
                }
            }
        }
    }

    /// Carry 23: the room stamps each event with the `TickInput::tick` it ran and consecutive 32-bit numbers, across a
    /// 16-bit wrap from an injectable start a few below it; each client's queue holds them with those stamps.
    #[test]
    fn carry_23_events_carry_their_tick_and_consecutive_32_bit_numbers_across_a_16_bit_wrap() {
        let scene = Scene::stress();
        let world = World::new(&scene, 7, 900);
        let greybox = world.greybox().clone();
        let start = u32::from(u16::MAX) - 20;
        let tokens: Vec<[u8; 16]> = (0..48u8).map(|s| [s; 16]).collect();
        let mut room = Room::new(world, &scene, &greybox, tokens, 48, 30).with_event_start(start);
        for s in 0..48u8 {
            assert!(
                room.join(&Join {
                    token: [s; 16],
                    ..Join::default()
                })
                .is_ok()
            );
        }
        let rec = npt_sim::replay::Recording::synthetic(&scene, 7, 900, 100, 2_000);
        let mut next = start;
        let mut stamps: Vec<(u32, u32)> = Vec::new();
        let mut checked = 0u32;
        for _ in 0..60 {
            let t = room.last_tick().wrapping_add(1);
            for input in rec.ticks.iter().filter(|i| i.tick == t) {
                for (c, _) in &input.commands {
                    let msg = Message::Input(Input {
                        commands: vec![*c],
                        ..Input::default()
                    });
                    room.receive(c.slot, &msg);
                }
            }
            room.tick();
            let t = room.last_tick();
            for &(number, tick, _) in &room.stamped {
                assert_eq!(
                    (number, tick),
                    (next, t),
                    "consecutive numbers, the tick run"
                );
                next = next.wrapping_add(1);
                stamps.push((number, tick));
            }
            if let Some(c) = room.clients[0].as_mut() {
                for q in c.queue.iter_mut() {
                    assert!(stamps.contains(&(q.number, q.tick)), "{q:?}");
                    checked += 1;
                }
            }
        }
        assert!(
            next > u32::from(u16::MAX) + 1,
            "the numbers crossed the 16-bit wrap: {next}"
        );
        assert!(checked > 0, "the queue half checked a queued event (RR-10)");
    }
}
