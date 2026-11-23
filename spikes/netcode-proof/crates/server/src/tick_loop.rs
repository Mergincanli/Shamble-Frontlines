//! [`TickLoop`] (plan.md:601's list; plan E2, §4.4 and §4.6; u6a-brief.md Block 2): the room process's loop.
//!
//! - **The tick thread** (`npt-tick`) alone owns the `Room` and runs [`due_ticks`] on the monotonic clock (`clock.rs`'s
//!   one item): at most `npt_sim::LOOP_CATCHUP` ticks a wake, the time owed beyond it dropped and counted. Before each
//!   tick it takes what the I/O runtime decoded (each message with its arrival stamp) and each session's report (the
//!   bytes the server side sent since the last tick, the largest datagram, and `Decoder::rtt_ns`: register 96), hands
//!   them to the room, runs the tick, and gives each session its datagrams and its reliable messages, each on the
//!   stream its own framing names (npt-proto's `channel_of`: C-U6-16 (c)).
//! - **The I/O runtime** (tokio, multi-thread, `npt-io`) owns each session's `Box<dyn Link + Send>` and its
//!   [`Decoder`], and decodes there (plan §4.4: the sim thread never parses raw bytes); it answers pings and sends what
//!   the tick thread queued.
//! - **Bounded channels both ways** (E2): the sessions' one queue to the tick thread, each session's queue from it, and
//!   the tick thread's queue to the log's writer thread. A full one is counted and its item dropped; nothing on the
//!   tick thread ever waits on one. A reliable message the session's queue cannot take ends that session (counted).
//! - **Sessions end** (C-U6-14's first clause): when `Room::join` reports a replacement, the old session is sent
//!   `Kick(replaced)`, unrouted at once (nothing it sent later reaches the room) and closed; a session that sent
//!   `Leave`, or that the room kicked, is closed after its last messages went out.
//! - **The panic catch** (plan E2; `panic = "unwind"`, the workspace's default): a panic on the tick thread sends
//!   `RoomEnded(crash)` to every session, writes the crash record (`event.tick_panic`) and a `crash` footer, and the
//!   caller's [`TickLoop::stop`] returns [`Ended::Crash`] (the binary's non-zero exit).
//! - **The log** (marks/README.md §1: never written on the tick thread): the tick thread hands a fixed-size row per
//!   tick to the writer thread (`npt-log`), which builds the records: the loop's five series from [`PhaseTimers`]
//!   (`tick.v.work_ns`, `start_jitter_ns`, `dropped_ns`, `overrun`, `wake_ticks_count`).
//!
//! The room's own inputs take no arrival stamp and no RTT at this base (`Room::receive`, `Room::pump_report`): both
//! reach the tick thread, which keeps each slot's RTT for the room's later reader (U6a's report, finding F1).

use std::io::{self, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::Duration;

use npt_link::{Channel, Link, LinkState};
use npt_log::{Ended, EventRecord, Phase, TickRecord, Writer};
use npt_proto::{EndReason, JoinRejected, Kick, Message, Quant, RoomEnded};
use npt_room::{CmdTiming, Room};
use npt_sim::{
    EntityRecord, HitHistory, HitVolumes, InputReports, PortalStates, Scene, SimCore, SimEvent,
    TickInput, World,
};

use crate::clock;
use crate::phase::PhaseTimers;
use crate::pump::{Decoder, RttSource};
use crate::schedule::{Schedule, due_ticks};

/// The sessions' one queue to the tick thread: decoded messages, reports, opens and closes.
pub(crate) const INBOUND_CAP: usize = 4_096;
/// Each session's queue from the tick thread: its datagrams and reliable messages (8 s of an 8-group snapshot).
pub(crate) const OUTBOUND_CAP: usize = 2_048;
/// The tick thread's queue to the writer thread: about 34 s of tick rows at 30 Hz.
pub(crate) const ROWS_CAP: usize = 1_024;
/// Live sessions at most (a structural bound; U6b's connection cap of N + 8 sits in front of it).
pub(crate) const SESSIONS_MAX: usize = 256;
/// How often each session's I/O task polls its link (a `Link` is poll-based).
const POLL: Duration = Duration::from_millis(1);
/// The longest the tick thread sleeps at once, so a stop is seen within it.
const MAX_SLEEP_NS: u64 = 50_000_000;
/// How long [`TickLoop::stop`] lets the sessions send their last messages.
const FLUSH: Duration = Duration::from_millis(500);
/// The tick thread's name; the I/O runtime's threads are `npt-io` and the writer's `npt-log`.
pub(crate) const TICK_THREAD: &str = "npt-tick";

/// What [`TickLoop::start`] builds the room from, and the loop's own settings. No `Debug`: it holds the roster's
/// tokens, which never reach a log, a panic message or a debug print (SEC-U6AD-3, as roster.rs's `Entry`).
pub struct LoopSettings {
    /// The scene the room runs (the header's `scene`).
    pub scene: Scene,
    /// The run's seed and the room's first tick.
    pub seed: u64,
    pub first_tick: u32,
    /// The roster: one token per slot, in slot order (the binary mints them).
    pub tokens: Vec<[u8; 16]>,
    /// OQ-0001-6's option, a start-up setting (`params.cmd_timing`).
    pub cmd_timing: CmdTiming,
    /// What each session's `Join` must carry: the protocol version and the content hash; the mode hash its
    /// `JoinAccepted` names.
    pub version: u16,
    pub content_hash: [u8; 32],
    pub mode_hash: [u8; 32],
    /// The ticks logged as warm-up before the measured ones.
    pub warmup_ticks: u64,
    /// The I/O runtime's worker threads (at least 1).
    pub io_threads: usize,
    /// The room's log, its header written; `None` writes no log.
    pub log: Option<Writer<Box<dyn Write + Send>>>,
    /// The tick thread's vCPU and the I/O vCPUs, when the run pins (Q-U6A-5 (a)): the starting thread and every thread
    /// made after it go to `io_vcpus`, then the tick thread to `tick_vcpu`, each pin read back; the log gets the
    /// affinity reads at the start and the end, and the tick vCPU's steal each second. `None` pins nothing.
    pub tick_vcpu: Option<u32>,
    pub io_vcpus: Vec<u32>,
    /// The machine's SMT sibling lists (the header's `machine.smt`), for `sibling_free`.
    pub smt: Vec<Vec<u32>>,
    /// The planted defect (bites builds only): `PM-0001-1/shared-vcpu`, or one this crate plants elsewhere.
    #[cfg(feature = "bites")]
    pub bite: Option<String>,
}

/// PM-0001-1/shared-vcpu (marks.json: "a single-room run with a second room's tick thread on the same vCPU: the
/// affinity read must say not alone"; bites builds only, per instance): a second thread pinned to the tick vCPU before
/// the start's affinity read, kept until the loop stops.
#[cfg(feature = "bites")]
const SHARED_VCPU_BITE: &str = "PM-0001-1/shared-vcpu";

/// Client slots the per-tick row and the per-second record list (the stress scene's 48 and more).
pub(crate) const LOGGED_SLOTS: usize = 64;

/// The room's sim with its step's events counted as it steps (finding F3: the room keeps the tick's events
/// private, and `World::dump` would copy the whole state each tick): shots, and the reliable gameplay events (hits and
/// kills), for the room log's per-second counts. Every call is the World's own.
pub(crate) struct Tallied {
    pub(crate) world: World,
    pub(crate) shots: u32,
    pub(crate) reliable: u32,
}

impl SimCore for Tallied {
    fn step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent] {
        let events = self.world.step(input, history);
        let (mut shots, mut reliable) = (0, 0);
        for e in events {
            match e {
                SimEvent::Shot { .. } => shots += 1,
                SimEvent::Hit { .. } | SimEvent::Kill { .. } => reliable += 1,
            }
        }
        self.shots = shots;
        self.reliable = reliable;
        events
    }
    fn state_hash(&self) -> u64 {
        self.world.state_hash()
    }
    fn entity_records(&self) -> &[EntityRecord] {
        self.world.entity_records()
    }
    fn hit_volumes(&self) -> &HitVolumes {
        self.world.hit_volumes()
    }
    fn portal_states(&self) -> &PortalStates {
        self.world.portal_states()
    }
    fn reports(&self) -> &InputReports {
        self.world.reports()
    }
}

/// A very large unit's parts, at least (PM-0001-2: "4 very large units of at least 20 parts"; npt-lab's mirror.rs keeps
/// its own copy).
const LARGE_UNIT_PARTS: u16 = 20;

/// One tick's PM-0001-2 counts (Block 5, Q-U6A-2 (a); SERIES.md's rows; npt-lab's computation, logs.rs:577-715 and
/// :779-860, from the room after the tick): fixed size, made on the tick thread after the tick's end.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SceneRow {
    pub(crate) soldiers: u32,
    pub(crate) smallest_team: u32,
    pub(crate) snapshot_clients: u32,
    pub(crate) big_vehicles: u32,
    pub(crate) very_large_units: u32,
    pub(crate) projectiles: u32,
    pub(crate) effects_max: u32,
    /// The tick's share of the per-second counts: shots, reliable gameplay events (hits and kills), portal changes,
    /// projectile spawns, and the stand-in projectiles' changes and spawns.
    pub(crate) per_second: [u32; 6],
    /// `tick.s.everyone_in_view`, bit `s` for slot `s`.
    pub(crate) everyone: u64,
}

/// What the -2 counts keep from the tick before, in vectors indexed by entity index (grown only when an index is new),
/// and their per-tick scratch.
#[derive(Default)]
pub(crate) struct SceneTrack {
    doors: Vec<Option<i32>>,
    /// Each projectile index's generation and grid when last seen, and the tick count it was seen at.
    projectiles: Vec<Option<(u16, [i32; EntityRecord::FIELDS], u64)>>,
    parts: Vec<u16>,
    effects: Vec<u16>,
    teams: Vec<Option<u32>>,
    run: u64,
    /// ES's flag also needs every vehicle (PM-0001-2's ES row, carry 59).
    with_vehicles: bool,
}

fn grow<T: Clone + Default>(v: &mut Vec<T>, i: usize) -> &mut T {
    if v.len() <= i {
        v.resize(i + 1, T::default());
    }
    &mut v[i]
}

impl SceneTrack {
    /// The counts of the tick the room just ran, `clients` slots listed for `everyone_in_view`.
    pub(crate) fn count(&mut self, room: &Room<Tallied>, clients: usize) -> SceneRow {
        use npt_sim::EntityKind as K;
        let world = &room.sim().world;
        let scene = world.scene();
        let records = world.entity_records();
        let t = room.last_tick();
        let rows = u16::from(scene.effect_rows().max(1));
        self.parts.fill(0);
        self.effects.fill(0);
        self.teams.fill(None);
        let mut s = SceneRow::default();
        let [shots, reliable, portal, spawns, stand_changes, stand_spawns] = &mut s.per_second;
        *shots = room.sim().shots;
        *reliable = room.sim().reliable;
        for r in records {
            let i = usize::from(r.index);
            match r.kind {
                K::Soldier => {
                    s.soldiers += 1;
                    if let Some(team) = u8::try_from(r.index).ok().and_then(|x| scene.team(x)) {
                        *grow(&mut self.teams, usize::from(team)).get_or_insert(0) += 1;
                    }
                }
                K::Vehicle => s.big_vehicles += 1,
                K::VehiclePart => {
                    if let Ok(hull) = u16::try_from(r.grid[5]) {
                        *grow(&mut self.parts, usize::from(hull)) += 1;
                    }
                }
                K::Projectile => {
                    s.projectiles += 1;
                    let stand_in = r.grid[7] == 0;
                    let last = grow(&mut self.projectiles, i);
                    // Seen at the tick before with the same generation: the same projectile (npt-lab keeps only the
                    // tick before's live ones).
                    match last {
                        Some((g, grid, seen)) if *g == r.generation && *seen + 1 == self.run => {
                            if stand_in && *grid != r.grid {
                                *stand_changes += 1;
                            }
                        }
                        _ => {
                            *spawns += 1;
                            *stand_spawns += u32::from(stand_in);
                        }
                    }
                    *last = Some((r.generation, r.grid, self.run));
                }
                K::Door => {
                    let last = grow(&mut self.doors, i);
                    if last.is_some_and(|o| o != r.grid[0]) {
                        *portal += 1;
                    }
                    *last = Some(r.grid[0]);
                }
                // An effect still active after this tick (its end tick in grid[1]).
                K::Effect if (r.grid[1] as u32).wrapping_sub(t) as i32 > 0 => {
                    *grow(&mut self.effects, usize::from(r.index / rows)) += 1;
                }
                _ => {}
            }
        }
        self.run += 1;
        s.very_large_units = self
            .parts
            .iter()
            .filter(|&&p| p >= LARGE_UNIT_PARTS)
            .count() as u32;
        s.effects_max = self.effects.iter().copied().max().map_or(0, u32::from);
        s.smallest_team = self.teams.iter().flatten().copied().min().unwrap_or(0);
        for slot in 0..scene.slots() {
            let Some(c) = room.client(slot) else {
                continue;
            };
            if !c.kicked() && c.counters().snapshots > 0 {
                s.snapshot_clients += 1;
            }
            if usize::from(slot) >= clients.min(LOGGED_SLOTS) || c.kicked() {
                continue;
            }
            let own = records
                .iter()
                .any(|r| r.kind == K::Soldier && r.index == u16::from(slot));
            let (mut others, mut vehicles) = (0u32, 0u32);
            for &i in c.relevant() {
                match records.get(i) {
                    Some(r) if r.kind == K::Soldier && r.index != u16::from(slot) => others += 1,
                    Some(r) if r.kind == K::Vehicle => vehicles += 1,
                    _ => {}
                }
            }
            let all_others = s.soldiers - u32::from(own);
            if own && others == all_others && (!self.with_vehicles || vehicles == s.big_vehicles) {
                s.everyone |= 1 << slot;
            }
        }
        s
    }
}

/// What the I/O runtime hands the tick thread.
enum Inbound {
    /// A new session: its queue from the tick thread and its close flag.
    Open {
        session: u32,
        out: OutTx,
        close: Arc<AtomicBool>,
    },
    /// One decoded message, with its arrival stamp on the link's clock (the room takes none at this base: F1).
    Msg {
        session: u32,
        msg: Message,
        _arrival_ns: u64,
    },
    /// The session's report since its last one (once each room tick at most).
    Report {
        session: u32,
        sent_bytes: u32,
        max_datagram: usize,
        rtt_ns: Option<u64>,
    },
    /// The session's link or decoder closed on its own.
    Closed { session: u32 },
}

/// What the tick thread queues for one session.
enum Outbound {
    Datagram(Vec<u8>),
    Reliable(Channel, Vec<u8>),
    /// The room accepted the session's `Join`: datagrams are taken from now on.
    Accept,
    /// The room refused it: `JoinRejected`, then the session ends.
    Reject(JoinRejected),
}

/// The bounded queue to the tick thread (P1's site): an offer never waits; a full or closed queue says no.
#[derive(Clone)]
struct ToTick(SyncSender<Inbound>);

impl ToTick {
    fn offer(&self, x: Inbound) -> bool {
        self.0.try_send(x).is_ok()
    }
}

fn to_tick() -> (ToTick, Receiver<Inbound>) {
    let (tx, rx) = sync_channel(INBOUND_CAP);
    (ToTick(tx), rx)
}

/// One session's bounded queue from the tick thread: an offer never waits.
#[derive(Clone)]
struct OutTx(tokio::sync::mpsc::Sender<Outbound>);

impl OutTx {
    fn offer(&self, x: Outbound) -> bool {
        self.0.try_send(x).is_ok()
    }

    /// Items waiting (`second.s.out_queue_count`).
    fn len(&self) -> usize {
        self.0.max_capacity() - self.0.capacity()
    }
}

/// The counters and values both sides share.
#[derive(Default)]
pub(crate) struct Shared {
    /// The room's next tick to run (a `Pong`'s `tick`).
    pub(crate) next_tick: AtomicU32,
    /// Items a full queue to the tick thread dropped (decoded messages and reports).
    pub(crate) inbound_full: AtomicU64,
    /// Items a full session queue dropped.
    pub(crate) out_full: AtomicU64,
    /// Sessions ended because their queue could not take a reliable message, or a `Join` was lost.
    pub(crate) sessions_dropped: AtomicU64,
    /// Tick rows a full writer queue dropped.
    pub(crate) rows_dropped: AtomicU64,
    /// Ticks run.
    pub(crate) ticks: AtomicU64,
    /// The tick thread's id (0 until it starts).
    pub(crate) tick_tid: AtomicU32,
    /// The I/O runtime's threads' ids, as each starts (at most [`TIDS_MAX`]).
    pub(crate) io_tids: Mutex<Vec<u32>>,
    /// The shared-vcpu bite's thread's id (0 without it).
    #[cfg(feature = "bites")]
    pub(crate) bite_tid: AtomicU32,
}

/// Thread ids [`Shared::io_tids`] keeps at most.
const TIDS_MAX: usize = 256;

/// One tick's row for the writer thread: fixed size, nothing allocated on the tick thread.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TickRow {
    pub(crate) t: u64,
    pub(crate) ph: Phase,
    pub(crate) timers: PhaseTimers,
    /// The tick's start on the loop's clock (the writer's seconds).
    pub(crate) start_ns: u64,
    /// Each slot's items waiting in its session's queue after the tick's sends, by slot.
    pub(crate) out_queue: [u16; LOGGED_SLOTS],
    /// The tick's PM-0001-2 counts.
    pub(crate) scene: SceneRow,
}

/// What the writer thread is given.
pub(crate) enum Row {
    Tick(TickRow),
    Event(EventRecord),
    /// The last row: how the process ended.
    End(Ended),
}

/// A session as the tick thread holds it.
struct Session {
    id: u32,
    out: OutTx,
    close: Arc<AtomicBool>,
    slot: Option<u8>,
    /// The room accepted its `Join`: `Outbound::Accept` follows the tick's `JoinAccepted` and full state, so the
    /// decoder's first ping never reaches the client before them.
    accept_due: bool,
    sent: u32,
    max_datagram: usize,
    rtt_ns: Option<u64>,
}

/// The tick thread's state (the module text): the room, the schedule, the sessions.
pub(crate) struct Core {
    pub(crate) room: Room<Tallied>,
    schedule: Schedule,
    inbound: Receiver<Inbound>,
    sessions: Vec<Session>,
    /// Each slot's RTT as its session's last report gave it (register 96).
    pub(crate) rtt_ns: Vec<Option<u64>>,
    shared: Arc<Shared>,
    rows: Option<SyncSender<Row>>,
    warmup_ticks: u64,
    /// The log's tick number of the room's first tick, and the ticks run.
    base_t: u64,
    pub(crate) run: u64,
    /// The -2 counts' memory, and the slots `everyone_in_view` lists (the roster's).
    track: SceneTrack,
    clients: usize,
    /// A test's panic on this tick (test builds only: U6e's injections are the run's).
    #[cfg(test)]
    pub(crate) panic_at: Option<u64>,
}

/// The room a [`LoopSettings`] describes, over the event-counting sim.
pub(crate) fn build_room(s: &mut LoopSettings) -> Room<Tallied> {
    let world = World::new(&s.scene, s.seed, s.first_tick);
    let greybox = world.greybox().clone();
    let tokens = core::mem::take(&mut s.tokens);
    let n = tokens.len();
    let sim = Tallied {
        world,
        shots: 0,
        reliable: 0,
    };
    Room::new(sim, &s.scene, &greybox, tokens, n, npt_sim::TICK_HZ)
        .with_hashes(s.content_hash, s.mode_hash)
        .with_cmd_timing(s.cmd_timing)
}

impl Core {
    fn new(
        room: Room<Tallied>,
        inbound: Receiver<Inbound>,
        shared: Arc<Shared>,
        rows: Option<SyncSender<Row>>,
        (warmup_ticks, clients): (u64, usize),
    ) -> Core {
        let next = room.last_tick().wrapping_add(1);
        shared.next_tick.store(next, Ordering::Release);
        let track = SceneTrack {
            with_vehicles: room.sim().world.scene().name() == "ES",
            ..SceneTrack::default()
        };
        Core {
            track,
            clients,
            room,
            schedule: Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP),
            inbound,
            sessions: Vec::new(),
            // One entry per possible slot (a slot is a u8).
            rtt_ns: vec![None; usize::from(u8::MAX) + 1],
            shared,
            rows,
            warmup_ticks,
            base_t: u64::from(next),
            run: 0,
            #[cfg(test)]
            panic_at: None,
        }
    }

    /// One wake at `now_ns` on the loop's clock: every due tick up to the catch-up cap, each timed by `clock` (the
    /// loop's clock, read at each tick and phase edge). Returns when the next tick falls due.
    pub(crate) fn wake(&mut self, now_ns: u64, clock: &mut dyn FnMut() -> u64) -> u64 {
        let due = due_ticks(now_ns, &self.schedule);
        let overrun = due.next.counters().overruns > self.schedule.counters().overruns;
        let mut walk = self.schedule;
        for j in 0..due.to_run {
            let scheduled = walk.next_due_ns();
            walk = due_ticks(scheduled, &walk).next;
            let mut timers = PhaseTimers::start(
                scheduled,
                clock(),
                due.to_run,
                j == 0,
                due.dropped.ns,
                overrun,
            );
            self.tick(&mut timers, clock);
        }
        self.schedule = due.next;
        self.schedule.next_due_ns()
    }

    /// One tick: the inputs, the room's tick, the sends, the row.
    fn tick(&mut self, timers: &mut PhaseTimers, clock: &mut dyn FnMut() -> u64) {
        for _ in 0..INBOUND_CAP {
            let Ok(x) = self.inbound.try_recv() else {
                break;
            };
            self.apply(x);
        }
        for s in &mut self.sessions {
            if let Some(slot) = s.slot {
                self.room
                    .pump_report(slot, core::mem::take(&mut s.sent), s.max_datagram);
                if let Some(r) = self.rtt_ns.get_mut(usize::from(slot)) {
                    *r = s.rtt_ns;
                }
            }
        }
        timers.inputs_done(clock());
        #[cfg(test)]
        if self.panic_at == Some(self.run) {
            panic!("a test's tick-thread panic at tick {}", self.run);
        }
        self.room.tick();
        timers.room_done(clock());
        self.send_outputs();
        self.shared
            .next_tick
            .store(self.room.last_tick().wrapping_add(1), Ordering::Release);
        timers.end(clock());
        let ph = if self.run < self.warmup_ticks {
            Phase::Warmup
        } else {
            Phase::Measured
        };
        let mut out_queue = [0u16; LOGGED_SLOTS];
        for s in &self.sessions {
            if let Some(q) = s.slot.and_then(|slot| out_queue.get_mut(usize::from(slot))) {
                *q = u16::try_from(s.out.len()).unwrap_or(u16::MAX);
            }
        }
        // The -2 counts, after the tick's end (outside `work_ns`), from the room as the tick left it.
        let scene = self.track.count(&self.room, self.clients);
        let row = TickRow {
            t: self.base_t + self.run,
            ph,
            timers: *timers,
            start_ns: timers.start_ns(),
            out_queue,
            scene,
        };
        self.offer_row(Row::Tick(row));
        self.run += 1;
        self.shared.ticks.fetch_add(1, Ordering::Relaxed);
    }

    fn offer_row(&self, row: Row) {
        if let Some(tx) = &self.rows
            && tx.try_send(row).is_err()
        {
            self.shared.rows_dropped.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn index(&self, session: u32) -> Option<usize> {
        self.sessions.iter().position(|s| s.id == session)
    }

    fn apply(&mut self, x: Inbound) {
        match x {
            Inbound::Open {
                session,
                out,
                close,
            } => {
                if self.sessions.len() >= SESSIONS_MAX {
                    close.store(true, Ordering::Release);
                    return;
                }
                self.sessions.push(Session {
                    id: session,
                    out,
                    close,
                    slot: None,
                    accept_due: false,
                    sent: 0,
                    max_datagram: 0,
                    rtt_ns: None,
                });
            }
            Inbound::Msg { session, msg, .. } => {
                let Some(i) = self.index(session) else {
                    return;
                };
                if let Message::Join(j) = &msg {
                    self.join(i, j);
                    return;
                }
                let Some(slot) = self.sessions[i].slot else {
                    return;
                };
                self.room.receive(slot, &msg);
                if matches!(msg, Message::Leave(_)) {
                    // `Room::receive` freed the slot; the session closes.
                    self.close(i);
                }
            }
            Inbound::Report {
                session,
                sent_bytes,
                max_datagram,
                rtt_ns,
            } => {
                if let Some(i) = self.index(session) {
                    let s = &mut self.sessions[i];
                    s.sent = s.sent.saturating_add(sent_bytes);
                    s.max_datagram = max_datagram;
                    s.rtt_ns = rtt_ns;
                }
            }
            Inbound::Closed { session } => {
                if let Some(i) = self.index(session) {
                    if let Some(slot) = self.sessions[i].slot {
                        self.room.leave(slot);
                    }
                    self.sessions.remove(i);
                }
            }
        }
    }

    /// The room's answer to session `i`'s `Join`; a replacement's old session is kicked and closed first, before
    /// anything else is routed (C-U6-14's first clause).
    fn join(&mut self, i: usize, j: &npt_proto::Join) {
        match self.room.join(j) {
            Ok((slot, replaced)) => {
                let id = self.sessions[i].id;
                if replaced
                    && let Some(old) = self
                        .sessions
                        .iter()
                        .position(|s| s.slot == Some(slot) && s.id != id)
                {
                    let mut kick = Vec::new();
                    if npt_proto::encode(&Message::Kick(Kick::Replaced), &mut kick).is_ok()
                        && !self.sessions[old]
                            .out
                            .offer(Outbound::Reliable(Channel::Control, kick))
                    {
                        // Counted (NR-U6A-8); the old session is closed below either way.
                        self.shared.out_full.fetch_add(1, Ordering::Relaxed);
                    }
                    self.close(old);
                }
                let i = self.index(id).unwrap_or(i);
                self.sessions[i].slot = Some(slot);
                self.sessions[i].accept_due = true;
            }
            Err(reason) => {
                self.sessions[i].out.offer(Outbound::Reject(reason));
                self.close(i);
            }
        }
    }

    /// Ends session `i` on the tick thread's side: unrouted at once, its close flag set after everything queued for
    /// it (its I/O task sends those, then drops the link).
    fn close(&mut self, i: usize) {
        let s = self.sessions.remove(i);
        s.close.store(true, Ordering::Release);
    }

    /// A session whose queue could not take what it must get: its slot freed, counted, closed.
    fn drop_session(&mut self, i: usize) {
        if let Some(slot) = self.sessions[i].slot {
            self.room.leave(slot);
        }
        self.shared.sessions_dropped.fetch_add(1, Ordering::Relaxed);
        self.close(i);
    }

    /// Each joined session's datagrams and reliable messages from the tick; a session the room kicked is closed after
    /// its last messages (its `Kick` among them).
    fn send_outputs(&mut self) {
        let mut i = 0;
        while i < self.sessions.len() {
            let Some(slot) = self.sessions[i].slot else {
                i += 1;
                continue;
            };
            for d in self.room.take_datagrams(slot) {
                if !self.sessions[i].out.offer(Outbound::Datagram(d)) {
                    self.shared.out_full.fetch_add(1, Ordering::Relaxed);
                }
            }
            let mut lost = false;
            for b in self.room.take_control(slot) {
                let channel = reliable_channel(&b);
                if !self.sessions[i].out.offer(Outbound::Reliable(channel, b)) {
                    self.shared.out_full.fetch_add(1, Ordering::Relaxed);
                    lost = true;
                }
            }
            if core::mem::take(&mut self.sessions[i].accept_due)
                && !self.sessions[i].out.offer(Outbound::Accept)
            {
                self.shared.out_full.fetch_add(1, Ordering::Relaxed);
                lost = true;
            }
            if lost {
                self.drop_session(i);
                continue;
            }
            if self.room.client(slot).is_some_and(|c| c.kicked()) {
                self.room.leave(slot);
                self.close(i);
                continue;
            }
            i += 1;
        }
    }

    /// `RoomEnded` with `reason` to every session, each then closed; the writer's last row.
    fn end(&mut self, reason: EndReason, ended: Ended) {
        let final_tick = self
            .shared
            .next_tick
            .load(Ordering::Acquire)
            .wrapping_sub(1);
        let mut bytes = Vec::new();
        let encoded = npt_proto::encode(
            &Message::RoomEnded(RoomEnded { reason, final_tick }),
            &mut bytes,
        )
        .is_ok();
        while !self.sessions.is_empty() {
            if encoded
                && !self.sessions[0]
                    .out
                    .offer(Outbound::Reliable(Channel::Control, bytes.clone()))
            {
                // Counted (NR-U6A-8); the session is closed either way.
                self.shared.out_full.fetch_add(1, Ordering::Relaxed);
            }
            self.close(0);
        }
        if ended == Ended::Crash {
            let mut e =
                EventRecord::new("tick_panic", Phase::After).with("server_ns", clock::now_ns());
            e.t = Some(u64::from(final_tick));
            self.offer_row(Row::Event(e));
        }
        if let Some(tx) = &self.rows {
            // The last row waits for room: a full queue must not lose the footer's verdict.
            let _ = tx.send(Row::End(ended));
        }
    }
}

/// The stream each of `Room::take_control`'s messages goes out on (P2's site): the one its own framing names, by
/// npt-proto's `channel_of` (C-U6-16 (c)), never a copy of the lab's `stream_of`; the control stream for bytes that name
/// none (the room encodes only stream messages there).
fn reliable_channel(bytes: &[u8]) -> Channel {
    npt_proto::channel_of(bytes).unwrap_or(Channel::Control)
}

/// One session's I/O step on the runtime (the module text).
struct SessionIo {
    id: u32,
    link: Box<dyn Link + Send>,
    decoder: Decoder,
    to_tick: ToTick,
    from_tick: tokio::sync::mpsc::Receiver<Outbound>,
    close: Arc<AtomicBool>,
    shared: Arc<Shared>,
    reported: Option<u32>,
    unreported: u32,
    msgs: Vec<(Message, u64)>,
}

impl SessionIo {
    /// A session over `link`, accepted at `now_ns`, registered with the tick thread through `to_tick`, its queue from
    /// the tick thread holding `out_cap` items (TickLoop's: [`OUTBOUND_CAP`]); `None` when the queue to the tick thread
    /// cannot take its opening.
    #[allow(
        clippy::too_many_arguments,
        reason = "a session's parts, each from its own source"
    )]
    fn open(
        to_tick: &ToTick,
        shared: &Arc<Shared>,
        id: u32,
        link: Box<dyn Link + Send>,
        rtt: RttSource,
        join: (u16, [u8; 32]),
        now_ns: u64,
        out_cap: usize,
    ) -> Option<SessionIo> {
        let (tx, rx) = tokio::sync::mpsc::channel(out_cap.max(1));
        let close = Arc::new(AtomicBool::new(false));
        let open = Inbound::Open {
            session: id,
            out: OutTx(tx),
            close: Arc::clone(&close),
        };
        if !to_tick.offer(open) {
            return None;
        }
        Some(SessionIo {
            id,
            link,
            decoder: Decoder::new(
                Quant::default(),
                npt_sim::TICK_HZ,
                now_ns,
                join.0,
                join.1,
                rtt,
            ),
            to_tick: to_tick.clone(),
            from_tick: rx,
            close,
            shared: Arc::clone(shared),
            reported: None,
            unreported: 0,
            msgs: Vec::new(),
        })
    }

    /// One poll at `now_ns`: what the tick thread queued goes out, then the pump, then the report once a tick.
    /// Returns false once the session is over (its link then dropped).
    fn poll(&mut self, now_ns: u64) -> bool {
        let closing = self.close.load(Ordering::Acquire);
        while let Ok(o) = self.from_tick.try_recv() {
            // Rate adaptation reads each send's result (U6d); U6a sends and goes on.
            match o {
                Outbound::Datagram(b) => {
                    self.link.send_unreliable(&b, now_ns);
                }
                Outbound::Reliable(channel, b) => {
                    self.link.send_reliable(channel, &b, now_ns);
                }
                Outbound::Accept => self.decoder.accept(),
                Outbound::Reject(r) => self.decoder.reject(&mut *self.link, r, now_ns),
            }
        }
        if closing {
            return false;
        }
        if self.decoder.is_closed() || self.link.state() == LinkState::Closed {
            // Kept until the tick thread has the notice, so its slot is always freed.
            return !self.to_tick.offer(Inbound::Closed { session: self.id });
        }
        let tick = self.shared.next_tick.load(Ordering::Acquire);
        self.decoder
            .pump(&mut *self.link, now_ns, tick, &mut self.msgs);
        for (msg, arrival_ns) in self.msgs.drain(..) {
            let join = matches!(msg, Message::Join(_));
            let offered = self.to_tick.offer(Inbound::Msg {
                session: self.id,
                msg,
                _arrival_ns: arrival_ns,
            });
            if !offered {
                self.shared.inbound_full.fetch_add(1, Ordering::Relaxed);
                if join {
                    // A `Join` the tick thread never sees would hold the session in the decoder's joining state.
                    self.shared.sessions_dropped.fetch_add(1, Ordering::Relaxed);
                    let _ = self.to_tick.offer(Inbound::Closed { session: self.id });
                    return false;
                }
            }
        }
        self.unreported = self
            .unreported
            .saturating_add(self.decoder.take_sent_bytes());
        if self.reported != Some(tick) {
            let report = Inbound::Report {
                session: self.id,
                sent_bytes: self.unreported,
                max_datagram: self.link.max_datagram_size(),
                rtt_ns: self.decoder.rtt_ns(&*self.link),
            };
            if self.to_tick.offer(report) {
                self.unreported = 0;
                self.reported = Some(tick);
            } else {
                self.shared.inbound_full.fetch_add(1, Ordering::Relaxed);
            }
        }
        true
    }
}

async fn run_session(mut io: SessionIo) {
    while io.poll(clock::now_ns()) {
        tokio::time::sleep(POLL).await;
    }
}

/// The writer thread: one record per row, in order, with a second record each time the ticks' start enters a new
/// second of the loop's clock (and one for the last second at the end); the footer from the last row (`stopped` if the
/// rows ended without one).
fn write_rows(
    mut w: Writer<Box<dyn Write + Send>>,
    rows: Receiver<Row>,
    mut seconds: Seconds,
) -> io::Result<()> {
    let mut ended = Ended::Stopped;
    while let Ok(row) = rows.recv() {
        match row {
            Row::Tick(r) => {
                seconds.tick(&mut w, &r)?;
                w.tick(&tick_record(&r, seconds.clients))?;
            }
            Row::Event(e) => w.event(&e)?,
            Row::End(e) => {
                ended = e;
                break;
            }
        }
    }
    seconds.close(&mut w)?;
    w.finish(ended)?;
    Ok(())
}

/// The writer thread's per-second reads (Block 4; SERIES.md): the tick vCPU's steal (`second.v.steal_permille`, when
/// the run pins), the process's resident memory and CPU (`rss_bytes`, `proc_cpu_permille`), the I/O runtime's CPU
/// (`io_cpu_permille`) and each slot's session queue (`second.s.out_queue_count`). Read here, never on the tick thread.
/// A read that fails leaves its key out of that record, with one stderr line at its first failure (NR-U6A-4: the
/// judge sees a gap, never a best value).
pub(crate) struct Seconds {
    clients: usize,
    steal: Option<crate::proc_read::StealReader>,
    shared: Arc<Shared>,
    /// The second being gathered and its phase (its last tick's), where it began on the loop's clock, and the CPU
    /// times (the process's, the I/O runtime's) read then.
    sec: Option<(u64, Phase)>,
    at_ns: u64,
    cpu: Option<(u64, u64)>,
    last_ns: u64,
    out_queue: [u16; LOGGED_SLOTS],
    /// PM-0001-2's per-second counts gathered so far this second.
    sums: [u64; 6],
    /// The reads ([`SECOND_READS`]: steal, RSS, CPU times) that have failed at least once.
    failed: [bool; 3],
}

/// The per-second reads that can fail, for their stderr lines.
const SECOND_READS: [&str; 3] = [
    "steal_permille",
    "rss_bytes",
    "proc_cpu_permille and io_cpu_permille",
];

/// `/proc`'s text at `path`, empty when it cannot be read (each parser then finds nothing: a failed read).
fn proc_text(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

impl Seconds {
    pub(crate) fn new(clients: usize, tick_vcpu: Option<u32>, shared: Arc<Shared>) -> Seconds {
        Seconds {
            clients: clients.min(LOGGED_SLOTS),
            steal: tick_vcpu.map(crate::proc_read::StealReader::new),
            shared,
            sec: None,
            at_ns: 0,
            cpu: None,
            last_ns: 0,
            out_queue: [0; LOGGED_SLOTS],
            sums: [0; 6],
            failed: [false; 3],
        }
    }

    fn io_tids(&self) -> Vec<u32> {
        self.shared
            .io_tids
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Reads the CPU times now and the steal from `stat` (`/proc/stat`'s text): the next second's baseline. The steal
    /// is `Err` for a failed read (no line for the tick vCPU), `Ok(None)` for no share without a failure (unpinned, the
    /// first read, or no time since the last: FP-13).
    fn baseline(&mut self, stat: &str) -> Result<Option<u32>, ()> {
        self.cpu = crate::proc_read::cpu_ns(&self.io_tids());
        match self.steal.as_mut() {
            Some(s) if !s.has_line(stat) => Err(()),
            s => Ok(s.and_then(|s| s.read(stat))),
        }
    }

    /// Puts each read into `v`, or leaves its keys out when it failed (`None`, `Err`), with one stderr line at that
    /// read's first failure (the struct's text). `steal` counts only when the run pins, and `Ok(None)` (no time) leaves
    /// its key out with no line and no flag; `cpu` is the second's (process, I/O runtime) CPU time, ns, over `wall_ns`.
    fn values(
        &mut self,
        v: &mut std::collections::BTreeMap<String, i64>,
        steal: Result<Option<u32>, ()>,
        rss: Option<u64>,
        cpu: Option<(u64, u64)>,
        wall_ns: u64,
    ) {
        let share = |ns| int(crate::proc_read::permille(ns, wall_ns));
        let reads = [
            (
                self.steal.is_some() && steal != Ok(None),
                steal
                    .ok()
                    .flatten()
                    .map(|s| vec![("steal_permille", i64::from(s))]),
            ),
            (true, rss.map(|r| vec![("rss_bytes", int(r))])),
            (
                true,
                cpu.map(|(p, io)| {
                    vec![
                        ("proc_cpu_permille", share(p)),
                        ("io_cpu_permille", share(io)),
                    ]
                }),
            ),
        ];
        for (i, (asked, read)) in reads.into_iter().enumerate() {
            match read {
                _ if !asked => {}
                Some(kv) => v.extend(kv.into_iter().map(|(k, x)| (k.to_owned(), x))),
                None if !core::mem::replace(&mut self.failed[i], true) => eprintln!(
                    "npt-server: the per-second read of {} failed; its key is left out of each record it fails in",
                    SECOND_READS[i]
                ),
                None => {}
            }
        }
    }

    fn tick(&mut self, w: &mut Writer<Box<dyn Write + Send>>, r: &TickRow) -> io::Result<()> {
        let sec = r.start_ns / 1_000_000_000;
        match self.sec {
            None => {
                let _ = self.baseline(&proc_text("/proc/stat"));
                self.at_ns = r.start_ns;
            }
            Some((s, _)) if sec > s => {
                self.write(w, r.start_ns)?;
                self.at_ns = r.start_ns;
            }
            _ => {}
        }
        self.sec = Some((sec, r.ph));
        self.last_ns = r.start_ns;
        self.out_queue = r.out_queue;
        for (sum, x) in self.sums.iter_mut().zip(r.scene.per_second) {
            *sum += u64::from(x);
        }
        Ok(())
    }

    /// The gathered second's record, over the wall time to `now_ns`; the reads also start the next second.
    fn write(&mut self, w: &mut Writer<Box<dyn Write + Send>>, now_ns: u64) -> io::Result<()> {
        let Some((sec, ph)) = self.sec else {
            return Ok(());
        };
        let wall = now_ns.saturating_sub(self.at_ns);
        let before = self.cpu;
        let steal = self.baseline(&proc_text("/proc/stat"));
        let rss = crate::proc_read::rss_bytes(&proc_text("/proc/self/status"));
        let cpu = before
            .zip(self.cpu)
            .map(|(b, a)| (a.0.saturating_sub(b.0), a.1.saturating_sub(b.1)));
        let mut rec = npt_log::SecondRecord::new(sec, ph);
        self.values(&mut rec.v, steal, rss, cpu, wall);
        rec.s.insert(
            "out_queue_count".into(),
            self.out_queue[..self.clients]
                .iter()
                .map(|&q| i64::from(q))
                .collect(),
        );
        for (name, sum) in PER_SECOND.iter().zip(core::mem::take(&mut self.sums)) {
            rec.v.insert((*name).into(), int(sum));
        }
        w.second(&rec)
    }

    /// The last second's record at the end.
    fn close(&mut self, w: &mut Writer<Box<dyn Write + Send>>) -> io::Result<()> {
        self.write(w, self.last_ns)
    }
}

fn int(x: u64) -> i64 {
    i64::try_from(x).unwrap_or(i64::MAX)
}

/// A tick row's record (SERIES.md), every name on every record: the loop's five series, PM-0001-2's seven room counts,
/// and `everyone_in_view` with one flag per slot of the `clients` (index = slot; 0 for a slot with no client).
pub(crate) fn tick_record(r: &TickRow, clients: usize) -> TickRecord {
    let mut rec = TickRecord::new(r.t, r.ph);
    let v = &mut rec.v;
    v.insert("work_ns".into(), int(r.timers.work_ns()));
    v.insert("start_jitter_ns".into(), int(r.timers.start_jitter_ns()));
    v.insert("dropped_ns".into(), int(r.timers.dropped_ns()));
    v.insert("overrun".into(), i64::from(r.timers.overrun()));
    v.insert("wake_ticks_count".into(), i64::from(r.timers.wake_ticks()));
    let s = &r.scene;
    for (name, x) in [
        ("soldiers_count", s.soldiers),
        ("smallest_team_count", s.smallest_team),
        ("snapshot_clients_count", s.snapshot_clients),
        ("big_vehicles_count", s.big_vehicles),
        ("very_large_units_count", s.very_large_units),
        ("projectiles_count", s.projectiles),
        ("effects_per_soldier_max_count", s.effects_max),
    ] {
        v.insert(name.into(), i64::from(x));
    }
    let flags = (0..clients.min(LOGGED_SLOTS))
        .map(|slot| i64::from(s.everyone >> slot & 1 == 1))
        .collect();
    rec.s.insert("everyone_in_view".into(), flags);
    rec
}

/// The per-second counts' names, in [`SceneRow::per_second`]'s order.
const PER_SECOND: [&str; 6] = [
    "shots_count",
    "reliable_events_count",
    "portal_changes_count",
    "projectile_spawns_count",
    "stand_in_changes_count",
    "stand_in_spawns_count",
];

/// The tick thread: wakes on the loop's clock until stopped, under the top-level catch (the module text).
fn tick_thread(mut core: Core, stop: Arc<AtomicBool>) -> Ended {
    core.shared.tick_tid.store(
        crate::proc_read::thread_tid().unwrap_or(0),
        Ordering::Release,
    );
    let origin = clock::now_ns();
    let mut loop_clock = move || clock::now_ns().saturating_sub(origin);
    let run = catch_unwind(AssertUnwindSafe(|| {
        while !stop.load(Ordering::Acquire) {
            let now = loop_clock();
            let next = core.wake(now, &mut loop_clock);
            let now = loop_clock();
            if next > now {
                std::thread::sleep(Duration::from_nanos((next - now).min(MAX_SLEEP_NS)));
            }
        }
    }));
    match run {
        Ok(()) => {
            core.end(EndReason::Ended, Ended::Clean);
            Ended::Clean
        }
        Err(_) => {
            core.end(EndReason::Crash, Ended::Crash);
            Ended::Crash
        }
    }
}

/// The room process's loop (the module text): started by [`TickLoop::start`], given sessions by
/// [`TickLoop::connect`], ended by [`TickLoop::stop`].
pub struct TickLoop {
    tick: Option<JoinHandle<Ended>>,
    writer: Option<JoinHandle<io::Result<()>>>,
    runtime: Option<tokio::runtime::Runtime>,
    tasks: Mutex<Vec<tokio::task::JoinHandle<()>>>,
    stop: Arc<AtomicBool>,
    to_tick: ToTick,
    events: Option<SyncSender<Row>>,
    shared: Arc<Shared>,
    next_session: AtomicU32,
    join: (u16, [u8; 32]),
    tick_vcpu: Option<u32>,
    smt: Vec<Vec<u32>>,
    io_threads: usize,
    #[cfg(feature = "bites")]
    bite: Option<String>,
    #[cfg(feature = "bites")]
    bite_thread: Mutex<Option<JoinHandle<()>>>,
}

impl TickLoop {
    /// Builds the room `settings` describe, starts the I/O runtime, the writer thread (with a log) and the tick thread.
    ///
    /// # Errors
    /// The runtime or a thread could not be started.
    pub fn start(settings: LoopSettings) -> io::Result<TickLoop> {
        TickLoop::start_with(settings, |_| {})
    }

    /// [`TickLoop::start`] with `prepare` run on the tick thread's core before the thread starts (a test's panic tick;
    /// U6c's pinning reads).
    pub(crate) fn start_with(
        mut settings: LoopSettings,
        prepare: impl FnOnce(&mut Core),
    ) -> io::Result<TickLoop> {
        let clients = settings.tokens.len();
        let room = build_room(&mut settings);
        if settings.tick_vcpu.is_some() {
            // The starting thread first: every thread made after it (the runtime's, the writer's, the tick thread
            // until its own pin) inherits the I/O vCPUs.
            let me = crate::proc_read::thread_tid()
                .ok_or_else(|| io::Error::other("this thread's id is not in /proc"))?;
            crate::proc_read::pin(me, &settings.io_vcpus)?;
        }
        let shared = Arc::new(Shared::default());
        let tids = Arc::clone(&shared);
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(settings.io_threads.max(1))
            .thread_name("npt-io")
            .enable_time()
            .on_thread_start(move || {
                let mut t = tids.io_tids.lock().unwrap_or_else(PoisonError::into_inner);
                if let Some(tid) = crate::proc_read::thread_tid()
                    && t.len() < TIDS_MAX
                {
                    t.push(tid);
                }
            })
            .build()?;
        let (to_tick, inbound) = to_tick();
        let (rows, writer) = match settings.log.take() {
            Some(w) => {
                let (tx, rx) = sync_channel(ROWS_CAP);
                let seconds = Seconds::new(clients, settings.tick_vcpu, Arc::clone(&shared));
                let h = std::thread::Builder::new()
                    .name("npt-log".into())
                    .spawn(move || write_rows(w, rx, seconds))?;
                (Some(tx), Some(h))
            }
            None => (None, None),
        };
        let mut core = Core::new(
            room,
            inbound,
            Arc::clone(&shared),
            rows.clone(),
            (settings.warmup_ticks, clients),
        );
        prepare(&mut core);
        let tl = Self::launch(core, runtime, to_tick, rows, writer, shared, settings)?;
        if let Some(vcpu) = tl.tick_vcpu {
            tl.pin_tick(vcpu)?;
        }
        if let Some(e) = tl.affinity("start") {
            tl.event(e);
        }
        Ok(tl)
    }

    /// Waits (at most 2 s) for the tick thread's id and the I/O runtime's threads, then pins the tick thread to
    /// `vcpu`, read back; with the shared-vcpu bite, its second thread on `vcpu` too.
    fn pin_tick(&self, vcpu: u32) -> io::Result<()> {
        let mut tid = 0;
        for _ in 0..2_000 {
            tid = self.shared.tick_tid.load(Ordering::Acquire);
            let io = self
                .shared
                .io_tids
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .len();
            if tid != 0 && io >= self.io_threads {
                break;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        if tid == 0 {
            return Err(io::Error::other("the tick thread's id never came"));
        }
        crate::proc_read::pin(tid, &[vcpu])?;
        #[cfg(feature = "bites")]
        if self.bite.as_deref() == Some(SHARED_VCPU_BITE) {
            let stop = Arc::clone(&self.stop);
            let shared = Arc::clone(&self.shared);
            let h = std::thread::Builder::new()
                .name("npt-bite".into())
                .spawn(move || {
                    shared.bite_tid.store(
                        crate::proc_read::thread_tid().unwrap_or(0),
                        Ordering::Release,
                    );
                    while !stop.load(Ordering::Acquire) {
                        std::thread::sleep(Duration::from_millis(10));
                    }
                })?;
            *self
                .bite_thread
                .lock()
                .unwrap_or_else(PoisonError::into_inner) = Some(h);
            let mut bite = 0;
            for _ in 0..2_000 {
                bite = self.shared.bite_tid.load(Ordering::Acquire);
                if bite != 0 {
                    break;
                }
                std::thread::sleep(Duration::from_millis(1));
            }
            crate::proc_read::pin(bite, &[vcpu])?;
        }
        Ok(())
    }

    /// `event.affinity` (README §1; -1, PM-0006-11) when the run pins: `alone` (no other task may run on the tick
    /// vCPU, the module text of proc_read.rs) and `sibling_free` (no I/O runtime thread may run on its SMT sibling),
    /// read now; `when` is `start` or `end`. A value whose read failed or was partial is 0, with one stderr line
    /// (NR-U6A-3).
    pub(crate) fn affinity(&self, when: &str) -> Option<EventRecord> {
        let vcpu = self.tick_vcpu?;
        let tick = self.shared.tick_tid.load(Ordering::Acquire);
        let tasks = crate::proc_read::tasks("/proc");
        let alone = crate::proc_read::alone(&tasks, tick, vcpu);
        let io: Vec<Option<Vec<u32>>> = self
            .shared
            .io_tids
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .iter()
            .map(|&t| crate::proc_read::allowed(t))
            .collect();
        let free = crate::proc_read::sibling_free_read(&self.smt, vcpu, &io);
        let [alone, free] = [("alone", alone), ("sibling_free", free)].map(|(k, r)| {
            r.unwrap_or_else(|why| {
                eprintln!("npt-server: affinity at {when}: {k} written as 0: {why}");
                false
            })
        });
        let ph = if when == "start" {
            Phase::Warmup
        } else {
            Phase::After
        };
        Some(
            EventRecord::new("affinity", ph)
                .with("when", when)
                .with("alone", i64::from(alone))
                .with("sibling_free", i64::from(free)),
        )
    }

    fn launch(
        core: Core,
        runtime: tokio::runtime::Runtime,
        to_tick: ToTick,
        events: Option<SyncSender<Row>>,
        writer: Option<JoinHandle<io::Result<()>>>,
        shared: Arc<Shared>,
        settings: LoopSettings,
    ) -> io::Result<TickLoop> {
        let stop = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&stop);
        let tick = std::thread::Builder::new()
            .name(TICK_THREAD.into())
            .spawn(move || tick_thread(core, flag))?;
        Ok(TickLoop {
            tick: Some(tick),
            writer,
            runtime: Some(runtime),
            tasks: Mutex::new(Vec::new()),
            stop,
            to_tick,
            events,
            shared,
            next_session: AtomicU32::new(0),
            join: (settings.version, settings.content_hash),
            tick_vcpu: settings.tick_vcpu,
            smt: settings.smt,
            io_threads: settings.io_threads.max(1),
            #[cfg(feature = "bites")]
            bite: settings.bite,
            #[cfg(feature = "bites")]
            bite_thread: Mutex::new(None),
        })
    }

    /// Hands a new session's link to the I/O runtime, its RTT from `rtt` (the session's kind). False when the loop
    /// holds 256 sessions (`SESSIONS_MAX`), has stopped, or its queue is full.
    pub fn connect(&self, link: Box<dyn Link + Send>, rtt: RttSource) -> bool {
        let Some(runtime) = &self.runtime else {
            return false;
        };
        let mut tasks = self.tasks.lock().unwrap_or_else(PoisonError::into_inner);
        tasks.retain(|t| !t.is_finished());
        if tasks.len() >= SESSIONS_MAX || self.stop.load(Ordering::Acquire) {
            return false;
        }
        let id = self.next_session.fetch_add(1, Ordering::Relaxed);
        let Some(io) = SessionIo::open(
            &self.to_tick,
            &self.shared,
            id,
            link,
            rtt,
            self.join,
            clock::now_ns(),
            OUTBOUND_CAP,
        ) else {
            return false;
        };
        tasks.push(runtime.spawn(run_session(io)));
        true
    }

    /// Writes `event` to the room's log, through the writer thread; false with no log or a full queue.
    pub fn event(&self, event: EventRecord) -> bool {
        self.events
            .as_ref()
            .is_some_and(|tx| tx.try_send(Row::Event(event)).is_ok())
    }

    /// The ticks run so far.
    pub fn ticks(&self) -> u64 {
        self.shared.ticks.load(Ordering::Relaxed)
    }

    /// Whether the tick thread has ended (after a panic on it, before any stop): the caller then stops the loop.
    pub fn finished(&self) -> bool {
        self.tick.as_ref().is_none_or(JoinHandle::is_finished)
    }

    /// The monotonic clock's resolution on this machine, ns (the header's `machine.clock_res_ns`; E18): the smallest
    /// non-zero step between two reads of the loop's clock over 1,000 steps.
    pub fn clock_resolution_ns() -> u64 {
        clock::resolution_ns(1_000)
    }

    /// Q-U6A-5's refusal (the binary's exit 2, before anything starts): whether one of `io_vcpus` is an SMT sibling of
    /// `tick_vcpu` on the map `smt` (the header's sibling lists), by `event.affinity.sibling_free`'s own rule.
    pub fn io_on_a_tick_sibling(smt: &[Vec<u32>], tick_vcpu: u32, io_vcpus: &[u32]) -> bool {
        !crate::proc_read::sibling_free(smt, tick_vcpu, &[io_vcpus.to_vec()])
    }

    /// Ends the loop: `RoomEnded` to every session (the crash reason after a tick-thread panic), their last messages
    /// sent, the log's footer written. [`Ended::Clean`], or [`Ended::Crash`] after a panic on the tick thread.
    ///
    /// # Errors
    /// The log could not be written.
    pub fn stop(mut self) -> io::Result<Ended> {
        self.shutdown()
    }

    fn shutdown(&mut self) -> io::Result<Ended> {
        if self.tick.as_ref().is_some_and(|h| !h.is_finished())
            && let Some(e) = self.affinity("end")
        {
            self.event(e);
        }
        self.stop.store(true, Ordering::Release);
        #[cfg(feature = "bites")]
        if let Some(h) = self
            .bite_thread
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
        {
            let _ = h.join();
        }
        let ended = match self.tick.take() {
            Some(h) => h.join().unwrap_or(Ended::Crash),
            None => Ended::Stopped,
        };
        let tasks =
            core::mem::take(&mut *self.tasks.lock().unwrap_or_else(PoisonError::into_inner));
        if let Some(runtime) = self.runtime.take() {
            runtime.block_on(async {
                let all = async {
                    for t in tasks {
                        let _ = t.await;
                    }
                };
                let _ = tokio::time::timeout(FLUSH, all).await;
            });
            runtime.shutdown_timeout(FLUSH);
        }
        self.events = None;
        if let Some(w) = self.writer.take() {
            w.join()
                .map_err(|_| io::Error::other("the log's writer thread panicked"))??;
        }
        Ok(ended)
    }
}

impl Drop for TickLoop {
    fn drop(&mut self) {
        if self.tick.is_some() {
            let _ = self.shutdown();
        }
    }
}

#[cfg(test)]
mod tests;
