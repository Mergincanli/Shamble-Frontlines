//! `L1Scenario` (u4-build-plan.md §5 U4r; plan.md §4.7, RQ8; u4r-brief.md 2.1): one server and its clients on one
//! simulated clock, [`SimClock`] (integer ns), in RQ8's order at each instant:
//! 1. packets due: L1 applies every event due at or before the instant at the instant's first call (here the
//!    listener's `poll_accept`, which also hands out each new link's server end), and never delivers a packet in the
//!    instant it was sent, P0 included (l1/src/lib.rs:65-70);
//! 2. the server's I/O step: per session `Decoder::pump`, then the room's `join` (after `Ok`, `Decoder::accept`; on
//!    `Err`, `Decoder::reject`), the pump called again after a `Join` (it returns after one), `receive` for every other
//!    message of a joined session, and `pump_report` with the bytes the pump sent;
//! 3. the server tick if due, through U4b's `Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)` (carry 6), then
//!    each session's datagrams and reliable messages onto its link, each reliable message on the one stream channel
//!    that decodes it (`stream_of`; the paragraph below);
//! 4. the clients' 60 Hz frames: no work until U4g's `Interp`, so the runner adds no instant for them;
//! 5. each client whose next step is due, in slot order: what its link delivered since its last step (polled into its
//!    inbox at every instant: the socket's buffer), its `Behaviour::step`, then `ClientEndpoint::step`, whose datagrams
//!    and reliable messages go onto its link.
//!
//! The runner wakes at the next of: a tick due, a client's step due, the net's next internal event
//! (`L1Net::next_event_ns`, L-c5). Each client's host clock is the SimClock plus a seeded offset (rate 1: the 1.5 rate
//! is U4f's cheat), and each arrival is stamped on it. L1's RTT is its own RFC 9002 estimate.
//!
//! **Session handling** (carry 76): when `Room::join` reports a replacement, the lab sends the old session
//! `Kick(replaced)` and drops its `Decoder` and link before it routes anything else (the `Kick(replaced)` is sent once;
//! on L1 the old link is dropped in the same instant, so a lost first transmission is not resent: best effort); when
//! the room kicks a client or takes its `Leave`, the lab drops that session; a closed decoder or link ends the session
//! and frees the room's slot (`Room::leave`, as its doc has it: "a closed link"). **Reliable messages' channel:** each
//! is decoded on both stream channels with the `Quant` it was coded with, and exactly one must take it (never a silent
//! fall-back to Control). **Tokens** are seeded per slot and never logged.
//!
//! **The link record** (C62H-BOUND): at every room tick, around each slot's send, the lab reads npt-l1's fate log
//! (`FateLog::server_datagrams_since` from the log's length before the send). The server-to-client datagram fates the
//! send pushed must be exactly one per snapshot group the room coded that slot, and a slot with groups must have a
//! session and a server hash (`link_lost`); otherwise the run fails. How many of them the link lost, by its loss draw
//! or in a burst, is the room log's `tick.s.snap_link_lost_count` for that slot.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;

use npt_bots::Behaviour;
use npt_client::ClientEndpoint;
use npt_l1::{DatagramCounts, L1Net, Profile};
use npt_link::{Channel, Link, LinkState, Listener};
use npt_log::{Ended, EventRecord, Phase};
use npt_proto::{Direction, Join, Kick, Message, Quant};
use npt_room::{ClientCounters, CmdTiming, Room};
use npt_server::{Counters, Decoder, RttSource, Schedule, due_ticks};
use npt_sim::replay::Recording;
use npt_sim::{Placement, Scene, StandIns, World};
use serde_json::Value;

use crate::forged::Attribution;
use crate::hash::Failure;
use crate::logs::{Logs, Meta};
use crate::mirror::{Mirror, Records, SetStats};

/// U4f's controller setting for every endpoint of a controller run (u4f-brief.md 2.2): the lead bound's 12 ticks less
/// a margin, and at most 9; one setting under both options (f6). The cap of 9 keeps every honest command inside option
/// B's buffer of 8 only while the up delay is at least about 2 ticks whenever the cap binds, which L1's bounded jitter
/// always gives; other links are C-U6-17's (NC-U4F-3).
const CONTROLLER_MAX_LEAD: u32 = 9;
// EA-U4F-5: the cap at most option B's buffer plus one, and below the room's lead bound in ticks.
const _: () = assert!(
    CONTROLLER_MAX_LEAD as usize <= npt_room::InputBuffer::CMD_CATCHUP + 1
        && CONTROLLER_MAX_LEAD
            < npt_sim::ticks_for_ms_at(npt_sim::TICK_HZ, Room::<Sim>::LEAD_BOUND_MS)
);

/// The simulation the lab's rooms run (advisor-u4r-r1, EA-U4R-2; U4-SEAM: npt-lab names `npt_sim::World` once, here;
/// hash.rs's bite list and dump checks name npt-sim's World as the tool they check, not a room's).
pub(crate) type Sim = World;

/// The protocol version and content hash the lab's room and clients share.
pub(crate) const VERSION: u16 = 1;
pub(crate) const CONTENT: [u8; 32] = *b"npt-lab scenario content hash v1";
/// Each L1 link's datagram limit: QUIC's smallest maximum datagram size (RFC 9000 §14; the room's groups fit it).
pub(crate) const MAX_DATAGRAM: usize = 1_200;
/// One second, ns.
pub(crate) const SEC: u64 = 1_000_000_000;
/// The span of the room's hit-volume ring, ms: a copy of npt-room's private `HISTORY_MS` (room/src/room.rs:33,
/// provisional until U4h), which a runner recording's header records (carry 14's U4r half).
pub(crate) const ROOM_HISTORY_MS: u32 = 2_000;
/// -21's forged suite's forgers (2.6): slots 0 to 10, slot `s` forging kind `s + 1`; the rest are honest.
pub(crate) const FORGERS: u8 = 11;
/// Carry 75's bound: every client is admitted before the room has stamped this many events past its start.
pub(crate) const ADMIT_EVENTS_BOUND: u64 = 32_768;
/// The highest feet a scripted spawn or teleport may set, cm: World's stated clamp bound (TUNE-2; world.rs:653-660),
/// the position grid's top less the fire origin's 140 cm above the feet, a copy of npt-sim's private `CHEST_CM`
/// (sim/src/world.rs:79; its `FEET_Z_BOUND_CM`, :71), with the grid's top read from npt-sim's grid.
pub(crate) const PLACEMENT_Z_BOUND_CM: i32 = npt_sim::grid::POSITION_Z.max() - 140;

/// The simulated clock (plan.md:617): integer ns, which only moves forward.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SimClock {
    now_ns: u64,
}

impl SimClock {
    pub(crate) fn now(self) -> u64 {
        self.now_ns
    }

    /// Moves to `t`, which is never earlier.
    pub(crate) fn advance_to(&mut self, t: u64) {
        assert!(t >= self.now_ns, "the SimClock never goes back");
        self.now_ns = t;
    }
}

/// splitmix64's step: the lab's seeded draws (offsets, tokens, burst offsets).
pub(crate) fn mix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// A uniform integer in `[0, n)`.
pub(crate) fn below(word: u64, n: u64) -> u64 {
    ((u128::from(word) * u128::from(n)) >> 64) as u64
}

fn sixteen(state: &mut u64) -> [u8; 16] {
    let mut out = [0u8; 16];
    out[..8].copy_from_slice(&mix(state).to_le_bytes());
    out[8..].copy_from_slice(&mix(state).to_le_bytes());
    out
}

/// Slot `slot`'s token and bind value, seeded (minting is U6's: plan :716).
pub(crate) fn credentials(seed: u64, slot: u8) -> ([u8; 16], [u8; 16]) {
    let mut s = seed ^ 0x746f_6b65_6e73_0000 ^ (u64::from(slot) << 40);
    (sixteen(&mut s), sixteen(&mut s))
}

/// The join a client of slot `slot` sends.
pub(crate) fn join_of(seed: u64, slot: u8) -> Join {
    let (token, bind) = credentials(seed, slot);
    Join {
        token,
        bind,
        version: VERSION,
        content_hash: CONTENT,
    }
}

/// The lead of a client on `profile` (carry 74 as U4R-Q14 amended it): the round trip plus the jitter in ticks,
/// rounded up, at least 1, plus 1: P0 2, P1 3, P2 and B1 5, P3 7.
pub(crate) fn lead_for(profile: &Profile) -> u32 {
    let ms = (profile.rtt_ns + profile.jitter_ns) / 1_000_000;
    npt_sim::ticks_for_ms(u32::try_from(ms).unwrap_or(u32::MAX)).max(1) + 1
}

/// When the room's tick `k` ticks after its first is due on the SimClock (U4b's schedule, base 0).
pub(crate) fn due_ns(k: u64) -> u64 {
    (u128::from(k) * u128::from(SEC) / u128::from(npt_sim::TICK_HZ)) as u64
}

/// The channel a reliable message the room encoded travels on: decoded on both stream channels with `quant`, the
/// one that takes it (the module text); an error when none does or both do.
pub(crate) fn stream_of(bytes: &[u8], quant: Quant) -> Result<Channel, String> {
    let ok = |ch| quant.decode(Direction::RoomToClient, ch, bytes).is_ok();
    match (ok(Channel::Control), ok(Channel::FullState)) {
        (true, false) => Ok(Channel::Control),
        (false, true) => Ok(Channel::FullState),
        (a, b) => Err(format!(
            "a reliable message of {} B decodes on Control {a} and on FullState {b}: its channel is not known",
            bytes.len()
        )),
    }
}

/// Carry 49 (TUNE-2): a scripted spawn or teleport above World's clamp bound is refused as a scenario error.
pub(crate) fn check_placements(
    spawns: &[Placement],
    teleports: &[Placement],
) -> Result<(), String> {
    for (what, p) in spawns
        .iter()
        .map(|p| ("spawn", p))
        .chain(teleports.iter().map(|p| ("teleport", p)))
    {
        if p.position[2] > PLACEMENT_Z_BOUND_CM {
            return Err(format!(
                "a scripted {what} of slot {} to z {} cm is above World's clamp bound {} cm (TUNE-2): refused",
                p.slot, p.position[2], PLACEMENT_Z_BOUND_CM
            ));
        }
    }
    Ok(())
}

/// The planted defects of one run, by the instance each goes to (u4r-brief.md 2.1: the `--bite` dispatch).
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    not(feature = "bites"),
    allow(dead_code, reason = "a measured build reads no bite")
)]
pub(crate) struct Bites {
    pub(crate) id: Option<String>,
    pub(crate) world: bool,
    pub(crate) room: bool,
    pub(crate) client: bool,
    pub(crate) decoder: bool,
    pub(crate) l1: bool,
    pub(crate) schedule: bool,
    /// npt-server's pump (each session's `Decoder::with_bite`: PM-0001-30/wallclock, U4R-Q6 (a)).
    pub(crate) pump: bool,
    /// npt-lab's own: carry 62 (h)'s client writer, PM-0001-17/whole-as-lost (the advisor's U4r build ruling, point 4).
    pub(crate) whole_as_lost: bool,
    /// npt-lab's own: the -30 pre-check's late-command replay client (one leg's, one client's: precheck.rs).
    pub(crate) late_command: bool,
    /// npt-lab's own: C62H-BOUND's link record, PM-0001-17/link-lost-overstated: on every slot, its first measured room
    /// tick at which the room sent it a snapshot datagram and the link lost none written
    /// `tick.s.snap_link_lost_count` 1 (the summary's totals keep the fate log's counts).
    pub(crate) link_lost_overstated: bool,
    /// npt-lab's model bite, chosen by its unit tests only (r1): the clients connect in a hash map's order.
    #[cfg_attr(
        not(all(test, feature = "bites")),
        allow(dead_code, reason = "only the bites build's unit tests choose it")
    )]
    pub(crate) client_order: bool,
}

/// npt-lab's own bite ids (bites builds only): carry 62 (h)'s client writer (U4R-Q2 (c); its form and id the advisor's
/// U4r build ruling, point 4: "a client writer that files a tick it decoded whole as lost: null and incomplete 1"),
/// the -30 pre-check's late-command replay client (u4r-brief.md 2.5, 7.2; marks.json:6567), and C62H-BOUND's link
/// record that overstates the link's loss (appended: precheck.rs reads index 1).
#[cfg(feature = "bites")]
pub(crate) const LAB_BITES: [&str; 3] = [
    "PM-0001-17/whole-as-lost",
    "PM-0001-30/late-command",
    "PM-0001-17/link-lost-overstated",
];
/// npt-lab's model bite (r1's), chosen only by its unit tests (l1's `with_model_bite` form).
#[cfg(all(test, feature = "bites"))]
pub(crate) const CLIENT_ORDER_BITE: &str = "npt-lab/client-order";

impl Bites {
    /// Each instance whose crate's list holds `id` (an id no instance knows is a usage error).
    #[cfg(feature = "bites")]
    pub(crate) fn of(id: &str) -> Result<Bites, String> {
        let proto = npt_proto::BITES.contains(&id);
        let b = Bites {
            id: Some(id.to_owned()),
            room: npt_room::BITES.contains(&id) || proto,
            client: npt_client::BITES.contains(&id) || proto,
            decoder: proto,
            l1: L1Net::BITES.contains(&id),
            // npt-server's list also holds the pump's id, which the schedule never plants (advisor-u4r-r1, FP-6 item 2:
            // EA-U4R-3 = RR3-L8): the schedule's own answer decides.
            schedule: Schedule::with_bite(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP, id).is_some(),
            pump: Decoder::new(
                Quant::default(),
                npt_sim::TICK_HZ,
                0,
                VERSION,
                CONTENT,
                RttSource::Link,
            )
            .with_bite(id)
            .is_some(),
            // npt-sim's two -21 check bites are a list apart from `World::BITES`, planted the same way (F2).
            world: Sim::BITES.contains(&id) || Sim::CHECK_BITES.contains(&id),
            whole_as_lost: LAB_BITES[0] == id,
            late_command: LAB_BITES[1] == id,
            link_lost_overstated: LAB_BITES[2] == id,
            client_order: false,
        };
        let any = b.world
            || b.room
            || b.client
            || b.l1
            || b.schedule
            || b.pump
            || b.whole_as_lost
            || b.late_command
            || b.link_lost_overstated;
        if !any {
            return Err(format!(
                "--bite {id:?}: no instance of this build plants it"
            ));
        }
        Ok(b)
    }

    #[cfg(not(feature = "bites"))]
    pub(crate) fn of(id: &str) -> Result<Bites, String> {
        Err(format!(
            "--bite {id}: this is a measured build, which holds no bites (build npt-lab with --features bites)"
        ))
    }

    /// The `Quant` the decoders and the clients code the wire with.
    pub(crate) fn quant(&self) -> Quant {
        #[cfg(feature = "bites")]
        if self.decoder
            && let Some(q) = self.id.as_deref().and_then(Quant::with_bite)
        {
            return q;
        }
        Quant::default()
    }

    fn world(&self, scene: &Scene, seed: u64, first_tick: u32, arm_tick: u32) -> Sim {
        #[cfg(feature = "bites")]
        if self.world
            && let Some(id) = self.id.as_deref()
        {
            return Sim::with_bite(scene, seed, first_tick, id, arm_tick)
                .expect("Bites::of checked that this instance plants it");
        }
        let _ = arm_tick;
        Sim::new(scene, seed, first_tick)
    }

    fn room(&self, room: Room<Sim>) -> Room<Sim> {
        #[cfg(feature = "bites")]
        if self.room
            && let Some(id) = self.id.as_deref()
        {
            return room
                .with_bite(id)
                .expect("the room plants its own and npt-proto's ids");
        }
        room
    }

    pub(crate) fn client(&self, e: ClientEndpoint) -> ClientEndpoint {
        #[cfg(feature = "bites")]
        if self.client
            && let Some(id) = self.id.as_deref()
        {
            return e
                .with_bite(id)
                .expect("the client plants its own and npt-proto's ids");
        }
        e
    }

    fn net(&self, seed: u64) -> L1Net {
        #[cfg(feature = "bites")]
        if self.l1
            && let Some(id) = self.id.as_deref()
        {
            return L1Net::with_bite(seed, id)
                .expect("Bites::of checked that this instance plants it");
        }
        L1Net::new(seed)
    }

    fn schedule(&self) -> Schedule {
        #[cfg(feature = "bites")]
        if self.schedule
            && let Some(id) = self.id.as_deref()
        {
            return Schedule::with_bite(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP, id)
                .expect("Bites::of checked that this instance plants it");
        }
        Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)
    }
}

/// One counted field: its event name and its reader.
type Field<T> = (&'static str, fn(&T) -> u64);

/// The room's per-client counters a session's counts write (2.5; SA-5): every refusal field of `ClientCounters` (the
/// kinds, the repeats, the late and the window's, the acknowledgement's, the resync and the kick's) and its queue's.
const ROOM_COUNTS: [Field<ClientCounters>; 20] = [
    ("late_count", |k| k.late),
    ("press_refused_count", |k| k.press_refused),
    ("buffer_full_count", |k| k.buffer_full),
    ("rewinds_refused_count", |k| k.rewinds_refused),
    ("events_queued_count", |k| k.events_queued),
    ("overflow_resyncs_count", |k| k.overflow_resyncs),
    ("lead_refused_count", |k| k.lead_refused),
    ("not_newer_count", |k| k.not_newer),
    ("seq_refused_count", |k| k.seq_refused),
    ("move_refused_count", |k| k.move_refused),
    ("aim_refused_count", |k| k.aim_refused),
    ("ack_refused_count", |k| k.ack_refused),
    ("ack_refused_commands_count", |k| k.ack_refused_commands),
    ("acks_old_count", |k| k.acks_old),
    ("seq_outside_window_count", |k| k.seq_outside_window),
    ("rejoin_overlap_count", |k| k.rejoin_overlap),
    ("commands_repeat_count", |k| k.commands_repeat),
    ("resyncs_dropped_count", |k| k.resyncs_dropped),
    ("resyncs_limited_count", |k| k.resyncs_limited),
    ("kicked_count", |k| k.kicked),
];

/// Every refusal field of npt-server's per-session `Counters` (2.5), each written `pump_<field>_count`.
const PUMP_COUNTS: [Field<Counters>; 21] = [
    ("pump_too_large_count", |d| d.too_large),
    ("pump_truncated_count", |d| d.truncated),
    ("pump_bad_length_count", |d| d.bad_length),
    ("pump_unknown_type_count", |d| d.unknown_type),
    ("pump_wrong_direction_count", |d| d.wrong_direction),
    ("pump_wrong_channel_count", |d| d.wrong_channel),
    ("pump_trailing_bytes_count", |d| d.trailing_bytes),
    ("pump_unknown_field_count", |d| d.unknown_field),
    ("pump_out_of_range_count", |d| d.out_of_range),
    ("pump_malformed_count", |d| d.malformed),
    ("pump_before_join_count", |d| d.before_join),
    ("pump_input_limited_count", |d| d.input_limited),
    ("pump_ping_limited_count", |d| d.ping_limited),
    ("pump_pong_unknown_count", |d| d.pong_unknown),
    ("pump_first_not_join_count", |d| d.first_not_join),
    ("pump_joining_dropped_count", |d| d.joining_dropped),
    ("pump_stream_malformed_count", |d| d.stream_malformed),
    ("pump_join_deadline_count", |d| d.join_deadline),
    ("pump_wrong_version_count", |d| d.wrong_version),
    ("pump_wrong_content_hash_count", |d| d.wrong_content_hash),
    ("pump_join_again_count", |d| d.join_again),
];

/// `e` with a session's counts: the room slot's (`ROOM_COUNTS`) and the pump's (`PUMP_COUNTS`), each when given.
pub(crate) fn counts(
    mut e: EventRecord,
    room: Option<&ClientCounters>,
    pump: Option<&Counters>,
) -> EventRecord {
    for (name, get) in ROOM_COUNTS {
        if let Some(k) = room {
            e = e.with(name, get(k));
        }
    }
    for (name, get) in PUMP_COUNTS {
        if let Some(d) = pump {
            e = e.with(name, get(d));
        }
    }
    e
}

/// One session's counts at its end (2.5; NA-15), read before the room frees its slot or its `Decoder` drops: how it
/// ended (`replaced`, `left`, `closed`, `kicked`), its room slot's counters (none when another session holds the slot)
/// and its pump's.
#[derive(Clone, Debug)]
pub(crate) struct SessionEnd {
    pub(crate) slot: u8,
    pub(crate) ended: &'static str,
    pub(crate) room: Option<ClientCounters>,
    pub(crate) pump: Counters,
}

/// One server-side session: its link, its decoder and, once joined, its slot.
pub(crate) struct Session {
    pub(crate) id: u64,
    pub(crate) link: Box<dyn Link>,
    pub(crate) decoder: Decoder,
    pub(crate) slot: Option<u8>,
    /// Messages this session's pump handed the room (r8 and r9 read them).
    pub(crate) routed: u64,
    /// The RTT rule's records (u4r-brief.md 7.2): the first pump instant after which `rtt_ns` is `Some`, and the
    /// first pump instant that decoded an `Input` with a command.
    pub(crate) first_sample_ns: Option<u64>,
    pub(crate) first_input_ns: Option<u64>,
}

/// The server half (the room behind one decoder per session), the same on L1 and on Loopback.
pub(crate) struct Server {
    pub(crate) room: Room<Sim>,
    pub(crate) sessions: Vec<Option<Session>>,
    quant: Quant,
    rtt: RttSource,
    next_id: u64,
    /// Each admission since the last take: (slot, session, tick it runs before).
    pub(crate) admitted: Vec<(u8, u64, u32)>,
    /// Each session ended since the last take: (slot, why).
    pub(crate) ended: Vec<(u8, &'static str)>,
    /// Each joined session's counts at its end since the last take (2.5), for the lab log's `session_end`.
    pub(crate) session_ends: Vec<SessionEnd>,
    /// The sessions the room's slots are held by (r8 reads which).
    pub(crate) owner: BTreeMap<u8, u64>,
    /// The pump's planted defect every session's decoder takes (`Bites::pump`; bites builds only).
    #[cfg_attr(
        not(feature = "bites"),
        allow(dead_code, reason = "a measured build plants no bite")
    )]
    pub(crate) decoder_bite: Option<String>,
}

impl Server {
    /// The lab's one server (advisor-u4r-r1, EA-U4R-2), for `run` and the pre-check's `run_leg`: a room on a new
    /// [`Sim`] of `scene` (the caller's, stand-ins off), `seed` and `first_tick`, with every slot's seeded token,
    /// `clients` client slots, the lab's content hash and `event_start`, behind one decoder per session on `bites`'
    /// `Quant`; `bites` plants its World bite (armed at `arm_tick`), its room bite and its pump bite. `run_leg` passes
    /// `Bites::default()` and event start 0, then sets its own decoder bite.
    pub(crate) fn build(
        scene: &Scene,
        seed: u64,
        first_tick: u32,
        clients: usize,
        event_start: u32,
        arm_tick: u32,
        bites: &Bites,
    ) -> Server {
        let world = bites.world(scene, seed, first_tick, arm_tick);
        let greybox = world.greybox().clone();
        let tokens = (0..scene.slots())
            .map(|slot| credentials(seed, slot).0)
            .collect();
        let room = Room::new(world, scene, &greybox, tokens, clients, npt_sim::TICK_HZ)
            .with_hashes(CONTENT, [0; 32])
            .with_event_start(event_start);
        let mut server = Server::new(bites.room(room), bites.quant(), RttSource::Link);
        if bites.pump {
            server.decoder_bite = bites.id.clone();
        }
        server
    }

    fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {
        Server {
            room,
            sessions: Vec::new(),
            quant,
            rtt,
            next_id: 0,
            admitted: Vec::new(),
            ended: Vec::new(),
            session_ends: Vec::new(),
            owner: BTreeMap::new(),
            decoder_bite: None,
        }
    }

    /// A new link's server end, accepted at `now`.
    pub(crate) fn accept(&mut self, link: Box<dyn Link>, now: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let decoder = Decoder::new(
            self.quant,
            npt_sim::TICK_HZ,
            now,
            VERSION,
            CONTENT,
            self.rtt,
        );
        #[cfg(feature = "bites")]
        let decoder = match self.decoder_bite.as_deref() {
            Some(id) => decoder
                .with_bite(id)
                .expect("Bites::pump checked that the pump plants it"),
            None => decoder,
        };
        self.sessions.push(Some(Session {
            id,
            link,
            decoder,
            slot: None,
            routed: 0,
            first_sample_ns: None,
            first_input_ns: None,
        }));
        id
    }

    /// Drops every other session on `slot` (a replacement), its counts kept first (2.5): `room` is the slot's counters
    /// read before the replacing join reset them.
    fn drop_slot(
        &mut self,
        slot: u8,
        keep: u64,
        why: &'static str,
        now: u64,
        room: Option<ClientCounters>,
    ) {
        for s in &mut self.sessions {
            let Some(x) = s else { continue };
            if x.slot == Some(slot) && x.id != keep {
                if why == "replaced" {
                    let mut out = Vec::new();
                    if npt_proto::encode(&Message::Kick(Kick::Replaced), &mut out).is_ok() {
                        x.link.send_reliable(Channel::Control, &out, now);
                    }
                }
                self.session_ends.push(SessionEnd {
                    slot,
                    ended: why,
                    room: room.clone(),
                    pump: x.decoder.counters().clone(),
                });
                *s = None;
                self.ended.push((slot, why));
            }
        }
    }

    /// RQ8's step 2 at `now` (the module text).
    pub(crate) fn io(&mut self, now: u64) {
        let tick = self.room.last_tick().wrapping_add(1);
        for i in 0..self.sessions.len() {
            while let Some(x) = self.sessions[i].as_mut() {
                let mut msgs = Vec::new();
                x.decoder.pump(x.link.as_mut(), now, tick, &mut msgs);
                if x.first_sample_ns.is_none() && x.decoder.rtt_ns(x.link.as_ref()).is_some() {
                    x.first_sample_ns = Some(now);
                }
                let mut joined = false;
                let mut replaced = None;
                let mut left = false;
                for (m, _) in msgs {
                    let x = self.sessions[i].as_mut().expect("present");
                    match (&m, x.slot) {
                        (Message::Join(j), _) => {
                            joined = true;
                            // A replacing join resets its slot's counters: each held slot's, read first (2.5).
                            let held: Vec<(u8, ClientCounters)> = self
                                .owner
                                .keys()
                                .filter_map(|&s| {
                                    self.room.client(s).map(|c| (s, c.counters().clone()))
                                })
                                .collect();
                            match self.room.join(j) {
                                Ok((slot, r)) => {
                                    x.decoder.accept();
                                    x.slot = Some(slot);
                                    self.admitted.push((slot, x.id, tick));
                                    self.owner.insert(slot, x.id);
                                    if r {
                                        let old =
                                            held.into_iter().find(|h| h.0 == slot).map(|h| h.1);
                                        replaced = Some((slot, x.id, old));
                                    }
                                }
                                Err(r) => x.decoder.reject(x.link.as_mut(), r, now),
                            }
                        }
                        (_, Some(slot)) => {
                            if let Message::Input(inp) = &m
                                && !inp.commands.is_empty()
                                && x.first_input_ns.is_none()
                            {
                                x.first_input_ns = Some(now);
                            }
                            x.routed += 1;
                            left = matches!(m, Message::Leave(_));
                            if left {
                                // 2.5: its counts before its `Leave` frees the room's slot.
                                self.session_ends.push(SessionEnd {
                                    slot,
                                    ended: "left",
                                    room: self.room.client(slot).map(|c| c.counters().clone()),
                                    pump: x.decoder.counters().clone(),
                                });
                            }
                            self.room.receive(slot, &m);
                            if left {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                if let Some((slot, keep, old)) = replaced {
                    // Carry 76: the old session is told and dropped before anything else is routed.
                    self.drop_slot(slot, keep, "replaced", now, old);
                }
                if left {
                    let slot = self.sessions[i].as_ref().and_then(|x| x.slot);
                    self.sessions[i] = None;
                    if let Some(slot) = slot {
                        self.owner.remove(&slot);
                        self.ended.push((slot, "left"));
                    }
                    break;
                }
                if !joined {
                    break;
                }
            }
            let Some(x) = self.sessions[i].as_mut() else {
                continue;
            };
            // Carry 76 (advisor-u4r-r1, FP-1): after the pump drained what had arrived, a closed decoder or link ends
            // the session, and the room's slot is freed when this session holds it (`Room::leave`: "a closed link").
            if x.decoder.is_closed() || x.link.state() == LinkState::Closed {
                let (id, slot) = (x.id, x.slot);
                if let Some(slot) = slot {
                    // 2.5: its counts before `Room::leave` frees the slot and the `Decoder` drops.
                    let holds = self.owner.get(&slot) == Some(&id);
                    self.session_ends.push(SessionEnd {
                        slot,
                        ended: "closed",
                        room: holds
                            .then(|| self.room.client(slot).map(|c| c.counters().clone()))
                            .flatten(),
                        pump: x.decoder.counters().clone(),
                    });
                }
                self.sessions[i] = None;
                if let Some(slot) = slot {
                    if self.owner.get(&slot) == Some(&id) {
                        self.room.leave(slot);
                        self.owner.remove(&slot);
                    }
                    self.ended.push((slot, "closed"));
                }
                continue;
            }
            if let Some(slot) = x.slot {
                let sent = x.decoder.take_sent_bytes();
                self.room
                    .pump_report(slot, sent, x.link.max_datagram_size());
            }
        }
    }

    /// The session holding `slot`, if any.
    pub(crate) fn session(&mut self, slot: u8) -> Option<&mut Session> {
        self.sessions
            .iter_mut()
            .flatten()
            .find(|x| x.slot == Some(slot))
    }

    /// Sends `slot`'s tick output (RQ8's step 3, after the tick) on its session's link, then drops the session if the
    /// room kicked it (carry 76).
    pub(crate) fn send(
        &mut self,
        slot: u8,
        datagrams: &[Vec<u8>],
        control: &[Vec<u8>],
        now: u64,
    ) -> Result<(), String> {
        let quant = self.quant;
        let kicked = self.room.client(slot).is_some_and(|c| c.kicked());
        let Some(x) = self.session(slot) else {
            return Ok(());
        };
        for d in datagrams {
            x.link.send_unreliable(d, now);
        }
        for b in control {
            x.link.send_reliable(stream_of(b, quant)?, b, now);
        }
        if kicked {
            // 2.5: a kicked slot keeps its `ClientSlot`, so its counts are read at the kick, with its pump's.
            let room = self.room.client(slot).map(|c| c.counters().clone());
            for s in &mut self.sessions {
                if let Some(x) = s.take_if(|x| x.slot == Some(slot)) {
                    self.session_ends.push(SessionEnd {
                        slot,
                        ended: "kicked",
                        room: room.clone(),
                        pump: x.decoder.counters().clone(),
                    });
                }
            }
            self.owner.remove(&slot);
            self.ended.push((slot, "kicked"));
        }
        Ok(())
    }
}

/// C62H-BOUND's join (TE-CB-1, C7; the module text): one slot's send at one room tick against npt-l1's fate log.
/// `groups` is how many snapshot datagrams the room coded that slot, `has_session` whether a session held the slot
/// at the send, `server_hash` the room's hash for it at the tick, and `c` the server-to-client datagram fates the send
/// pushed. The link's loss of them (lost by the draw or in a burst), or why the record cannot be joined: groups for a
/// slot with no session, fates other than one per group (with no session, groups is 0 here: other than none), or
/// groups with no server hash.
pub(crate) fn link_lost(
    groups: usize,
    has_session: bool,
    server_hash: Option<u64>,
    c: DatagramCounts,
) -> Result<u32, String> {
    if !has_session && groups > 0 {
        return Err(format!(
            "{groups} snapshot groups for a slot with no session"
        ));
    }
    if c.sent as usize != groups {
        return Err(format!(
            "{groups} snapshot groups, but the send pushed {} server-to-client datagram fates",
            c.sent
        ));
    }
    if groups > 0 && server_hash.is_none() {
        return Err(format!("{groups} snapshot groups with no server hash"));
    }
    Ok(c.lost + c.burst)
}

/// What one instant's RQ8 steps 1 and 2 hand a loop, with step 3's due count (advisor-u4r-r1, EA-U4R-1, the narrow
/// form): the admissions and the ended sessions since the last take, each for the caller's own sinks (the runner's
/// `Logs::admit` and `Logs::gone`; the pre-check's maps and its outcome's `ended`), and the ticks due now.
pub(crate) struct Wake {
    pub(crate) admitted: Vec<(u8, u64, u32)>,
    pub(crate) ended: Vec<(u8, &'static str)>,
    pub(crate) to_run: u32,
}

/// RQ8's steps 1 and 2 at `now`, shared by [`run`] and the pre-check's `run_leg` (advisor-u4r-r1, EA-U4R-1, the narrow
/// form): every new link's server end the listener hands out accepted (its `poll_accept` is the instant's first net
/// call, which applies the packets due), the server's I/O step, and its admissions and ended sessions taken; then step
/// 3's due count from `schedule`, which moves on. Each caller keeps its own per-tick body.
pub(crate) fn wake(
    server: &mut Server,
    listener: &mut dyn Listener,
    schedule: &mut Schedule,
    now: u64,
) -> Wake {
    while let Some((link, _)) = listener.poll_accept(now) {
        server.accept(link, now);
    }
    server.io(now);
    let admitted = std::mem::take(&mut server.admitted);
    let ended = std::mem::take(&mut server.ended);
    let due = due_ticks(now, schedule);
    *schedule = due.next;
    Wake {
        admitted,
        ended,
        to_run: due.to_run,
    }
}

/// One run's settings (the `scenario` subcommand's options).
#[derive(Clone, Debug)]
pub(crate) struct Settings {
    pub(crate) meta: Meta,
    pub(crate) out: PathBuf,
    pub(crate) scene: Scene,
    pub(crate) profile: Profile,
    pub(crate) clients: u8,
    pub(crate) seed: u64,
    pub(crate) tick_start: u32,
    pub(crate) event_start: u32,
    pub(crate) cmd_start: u32,
    pub(crate) warmup_ticks: u64,
    pub(crate) measured_ticks: u64,
    pub(crate) after_max_ticks: u64,
    /// B1's scheduled interval between burst starts, ns, before each burst's offset (`--burst-every-s`); 0 on another
    /// profile.
    pub(crate) burst_spacing_ns: u64,
    pub(crate) behaviour: String,
    /// The brawl's fire press on every n-th command (`--brawl-press-every`; U4F-Q11 (i) (a): 2 in -20's runs); 0 off.
    pub(crate) brawl_press_every: u64,
    pub(crate) every_step: bool,
    pub(crate) bites: Bites,
    pub(crate) arm_tick: u32,
    pub(crate) lead: u32,
    /// OQ-0001-6's option (`--cmd-timing`), recorded as `params.cmd_timing` (README :191-192).
    pub(crate) cmd_timing: CmdTiming,
    /// U4f's tick-lead controller on every endpoint (`--controller 1`); else the fixed lead `lead` (U4F-Q7 (i)).
    pub(crate) controller: bool,
    /// The room's event cap set as data (`--event-cap`: 2.8's measurement, and a short never-ack test); else the room's.
    pub(crate) event_cap: Option<u32>,
    /// The run's suite (`--suite`, `params.suite`): `clock` (-47) reconnects its clients, `never-ack` (-22) makes slot
    /// 0 the never-acker.
    pub(crate) suite: String,
    /// -47's connects (`--connects`, the clock suite's), counted over every slot's sessions.
    pub(crate) connects: u64,
    /// The clamp twin's source (`--twin-logs`, with `--twin` and role twin): the clamp run's folder, whose room log's
    /// clamp reports give the twin's points (2.12).
    pub(crate) twin_logs: Option<PathBuf>,
    /// The lag switch's hold and spacing, ms (`--lag-hold-ms`, 200; `--lag-every-ms`, 6,000: 100 bursts in 600 s).
    pub(crate) lag_hold_ms: u64,
    pub(crate) lag_every_ms: u64,
    /// The fast clock's episode length and its idle from the measured start, ns of simulated time
    /// (`--fast-clock-episode-s`, 60; `--fast-clock-idle-s`, 0: 5.3's p20 only).
    pub(crate) fast_episode_ns: u64,
    pub(crate) fast_idle_ns: u64,
    /// -21's forgers' interval (`--forge-every`, with `--suite forged`): one forged item per that many commands.
    pub(crate) forge_every: u64,
    pub(crate) record: Option<PathBuf>,
    pub(crate) bytes: Option<PathBuf>,
    /// Whether the run writes its logs (`false`: a runner recording regenerated inside `npt-lab hash`).
    pub(crate) logs: bool,
    /// Whether the run hands its recording back in memory (carry 82's regeneration).
    pub(crate) record_mem: bool,
}

/// A runner recording made again (carry 82; u4r-brief.md 2.1 (ii)): `npt-lab scenario --record` at these settings, the
/// brawl, no log kept, its bytes handed back. Its argv holds these flags only, each value one item (never a line split
/// on whitespace: advisor-u4r-r1, FP-3), so no manifest value can add a flag, and the advisor's U4r build ruling 1,
/// point 9 (a)'s refusal of `--lead` and `--every-step` holds on this path by construction.
pub(crate) fn record(
    scene: &Scene,
    profile: &str,
    seed: u64,
    clients: u8,
    first_tick: u32,
    warmup_s: u64,
    measured_s: u64,
) -> Result<Vec<u8>, String> {
    let (commit, seed, clients, first_tick, warmup_s, measured_s) = (
        "0".repeat(40),
        seed.to_string(),
        clients.to_string(),
        first_tick.to_string(),
        warmup_s.to_string(),
        measured_s.to_string(),
    );
    let argv: Vec<String> = [
        "--run",
        "regenerate",
        "--set",
        "regenerate",
        "--date",
        "2000-01-01",
        "--commit",
        &commit,
        "--out",
        "-",
        "--scene",
        scene.name(),
        "--profile",
        profile,
        "--seed",
        &seed,
        "--clients",
        &clients,
        "--tick-start",
        &first_tick,
        "--warmup-s",
        &warmup_s,
        "--measured-s",
        &measured_s,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    let mut s = parse(&argv)?;
    s.logs = false;
    s.record_mem = true;
    let o = run(&s).map_err(|f| match f {
        Failure::Usage(m) | Failure::Failed(m) => m,
    })?;
    o.recording_bytes
        .ok_or_else(|| "the scenario kept no recording".to_owned())
}

/// One client: its endpoint and link, its behaviour, its host clock's offset and its next step.
struct Client {
    slot: u8,
    endpoint: ClientEndpoint,
    link: Box<dyn Link>,
    behaviour: Behaviour,
    offset_ns: u64,
    next_step_ns: u64,
    inbox: Vec<(Channel, Vec<u8>, u64)>,
    last_stamp: Option<u32>,
    duplicates: u64,
    bytes_log: Option<std::io::BufWriter<std::fs::File>>,
    /// The clock suite (-47): this connect's number in the run, the host instant of its first clock sample, and whether
    /// its estimate was read (its slot then reconnects while connects remain).
    seq: u64,
    first_sample_ns: Option<u64>,
    read: bool,
    /// -21's fast clock: the SimClock instant (the measured start) from which its host clock runs at 1.5, continuous.
    fast_from: Option<u64>,
}

impl Client {
    /// The client's host clock at SimClock `ns`: its offset plus `ns`, at rate 1.5 from `fast_from` on.
    fn host(&self, ns: u64) -> u64 {
        self.offset_ns
            + match self.fast_from {
                Some(m) if ns > m => m + (ns - m) * 3 / 2,
                _ => ns,
            }
    }

    /// The first SimClock instant at which its host clock reads at least `host` (the inverse of [`Client::host`]).
    fn sim(&self, host: u64) -> u64 {
        let h = host.saturating_sub(self.offset_ns);
        match self.fast_from {
            Some(m) if h > m => m + ((h - m) * 2).div_ceil(3),
            _ => h,
        }
    }
}

/// The fast clock's episodes (2.6's schedule; f37): the measured window's back-to-back spans of `episode_ns` of
/// simulated time from its start, whole ones only, each as the room ticks it holds, [first, end) counted from the run's
/// first tick (tick k is due at `due_ns(k)`).
pub(crate) fn episodes(warmup_ticks: u64, measured_ticks: u64, episode_ns: u64) -> Vec<(u64, u64)> {
    let hz = u128::from(npt_sim::TICK_HZ);
    let at = |ns: u64| (u128::from(ns) * hz).div_ceil(u128::from(SEC)) as u64;
    let (start, end) = (due_ns(warmup_ticks), due_ns(warmup_ticks + measured_ticks));
    let e = episode_ns.max(1);
    (0..(end - start) / e)
        .map(|i| (at(start + i * e), at(start + (i + 1) * e)))
        .collect()
}

/// -21's fast clock's tally (2.6; U4F-Q11 (iii) (c); A1): slot 0's episodes, the room's lead bound in ticks, its
/// soldier's last record ([x, y, z, health]), its kind-1 and kind-2 refusal counts and its copies left out after the
/// last tick and at the episode's start, and the episode's sums: commands run, movement (µm), live ticks and ticks
/// counted apart.
#[derive(Default)]
struct FastClock {
    episodes: Vec<(u64, u64)>,
    bound: i64,
    prev: Option<[i32; 4]>,
    refused: [u64; 3],
    base: [u64; 3],
    sums: [i64; 4],
}

impl FastClock {
    /// Room tick `k` ran: `me` the fast client's soldier record, `run` its commands run, `refused` its kind-1 and kind-2
    /// counts and its copies left out (`Behaviour::fast_left_out`). At an episode's last tick, that episode's record:
    /// excess commands (run less the episode's ticks and the bound), excess movement (moved less the honest arithmetic,
    /// the live ticks plus the bound at full walking speed: U4F-Q11 (iii) (c)), the episode's refusals (SA-6), its copies
    /// left out (0 expected at P0: EA-U4F-1) and the ticks counted apart (dead at the start or the end: A1).
    fn tick(
        &mut self,
        k: u64,
        me: Option<[i32; 4]>,
        run: u32,
        refused: [u64; 3],
    ) -> Option<EventRecord> {
        let moved = self
            .prev
            .zip(me)
            .and_then(|(a, b)| crate::logs::moved_um(a, b));
        self.prev = me;
        let before = std::mem::replace(&mut self.refused, refused);
        let &(a, b) = self.episodes.iter().find(|(a, b)| (*a..*b).contains(&k))?;
        if k == a {
            (self.sums, self.base) = ([0; 4], before);
        }
        self.sums[0] += i64::from(run);
        match moved {
            Some(m) => (self.sums[1], self.sums[2]) = (self.sums[1] + m as i64, self.sums[2] + 1),
            None => self.sums[3] += 1,
        }
        if k + 1 < b {
            return None;
        }
        let top = crate::logs::MOVE_CM_S * 10_000 / i64::from(npt_sim::TICK_HZ);
        let [cmds, moved, live, apart] = self.sums;
        // RR-U4F-5 (1): never an underflow, should a counter restart inside an episode.
        let episode = [0, 1, 2].map(|i| refused[i].saturating_sub(self.base[i]));
        Some(
            EventRecord::new("fast_clock", Phase::Measured)
                .with("slot", 0)
                .with("excess_cmds_count", cmds - ((b - a) as i64 + self.bound))
                .with("excess_move_um", moved - (live + self.bound) * top)
                .with("lead_refused_count", episode[0])
                .with("not_newer_count", episode[1])
                .with("left_out_count", episode[2])
                .with("move_apart_count", apart),
        )
    }
}

/// A fresh endpoint of `slot` for a run: its fixed lead or U4f's controller, its starts and its bite.
fn endpoint_of(s: &Settings, slot: u8) -> ClientEndpoint {
    let e = ClientEndpoint::new(join_of(s.seed, slot), npt_sim::TICK_HZ)
        .with_lead(s.lead)
        .with_starts(s.cmd_start, s.event_start);
    s.bites.client(if s.controller {
        e.with_clock(CONTROLLER_MAX_LEAD)
    } else {
        e
    })
}

/// One burst of B1 for one client: (start, return), its heal, and what -18's record carries beside it.
#[derive(Clone, Debug, Default)]
struct Burst {
    start_ns: u64,
    end_ns: u64,
    healed: bool,
    full_state: bool,
    queue_max: u32,
    eviction_logged: bool,
}

/// What one run gives its caller: the summary line's numbers.
#[derive(Clone, Debug, Default)]
pub(crate) struct Summary {
    pub(crate) ticks: u64,
    pub(crate) after_ticks: u64,
    pub(crate) fate_log_sha256: String,
    pub(crate) world_hashes: Vec<(u32, u64)>,
    pub(crate) commands: u64,
    pub(crate) skipped: u64,
    pub(crate) duplicates: u64,
    pub(crate) late: u64,
    pub(crate) press_refused: u64,
    pub(crate) buffer_full: u64,
    pub(crate) rewinds_refused: u64,
    pub(crate) stale: u64,
    pub(crate) overflow_resyncs: u64,
    pub(crate) heals: u64,
    /// C62H-BOUND's measured totals, from npt-l1's fate-log counts at each send (never the written record): the
    /// snapshot datagrams the room sent, those the link lost (by its loss draw or in a burst) and, of those, the ones
    /// lost in a burst.
    pub(crate) link_sent: u64,
    pub(crate) link_lost: u64,
    pub(crate) link_burst: u64,
    pub(crate) audit: crate::mirror::Audit,
    pub(crate) counts: crate::logs::Counts,
    pub(crate) recording: Option<(usize, String)>,
    pub(crate) recording_bytes: Option<Vec<u8>>,
    pub(crate) order_digest: u64,
    /// A bite run whose after phase reached its cap with measured ticks unresolved (carry 73).
    pub(crate) after_capped: bool,
    /// f16's view of the fate log (test builds only): each entry's canonical first 32 bytes (link, direction, flow,
    /// outcome, overtaken flag, index in its flow, send time and arrival), its size, resends and stream fields left out
    /// (TE3-5: -21's twins differ in size by design).
    #[cfg(test)]
    pub(crate) fates: Vec<[u8; 32]>,
    /// f16's culprit (FP-7; test builds only): each tick's Soldier records, by slot, their grid fields.
    #[cfg(test)]
    pub(crate) soldiers: Vec<Vec<(u16, [i32; npt_sim::EntityRecord::FIELDS])>>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn fate_sha256(net: &L1Net) -> String {
    use sha2::{Digest, Sha256};
    net.with_fate_log(|log| {
        let mut h = Sha256::new();
        log.for_each_canonical(|b| h.update(b));
        h.finalize().iter().map(|b| format!("{b:02x}")).collect()
    })
}

/// The order the clients connect in: slot order, or under r1's model bite a hash map's (bites test builds only).
fn connect_order(n: u8, b: &Bites) -> Vec<u8> {
    #[cfg(all(test, feature = "bites"))]
    if b.client_order {
        return client_order_bitten(n);
    }
    let _ = b;
    (0..n).collect()
}

/// r1's bite (marks F29): the clients connect in a `HashMap`'s iteration order, which a process's random state seeds.
#[cfg(all(test, feature = "bites"))]
#[allow(clippy::disallowed_types, clippy::iter_over_hash_type)]
fn client_order_bitten(n: u8) -> Vec<u8> {
    let slots: std::collections::HashMap<u8, ()> = (0..n).map(|s| (s, ())).collect();
    slots.keys().copied().collect()
}

/// The digest of a connect order (each process logs it: equal digests mean the bite was not exercised).
fn order_digest(order: &[u8]) -> u64 {
    let mut s = 0x6f72_6465_7200_0000u64;
    for o in order {
        s ^= u64::from(*o);
        s = mix(&mut s);
    }
    s
}

/// Runs one scenario (the module text) and writes its three logs into `s.out`.
pub(crate) fn run(s: &Settings) -> Result<Summary, Failure> {
    let fail = Failure::Failed;
    let scene = s.scene.clone().with_stand_ins(StandIns::OFF);
    let n = s.clients.min(scene.slots());
    let mut server = Server::build(
        &scene,
        s.seed,
        s.tick_start,
        usize::from(n),
        s.event_start,
        s.arm_tick,
        &s.bites,
    );
    // OQ-0001-6's option, before any join (a start-up setting); the event cap opened for 2.8's measurement.
    server.room.set_cmd_timing(s.cmd_timing);
    if let Some(cap) = s.event_cap {
        server.room = server.room.clone().with_event_cap(cap);
    }
    let quant = s.bites.quant();
    let mut schedule = s.bites.schedule();
    let net = s.bites.net(s.seed);
    let mut listener = net.listener();
    if s.logs {
        std::fs::create_dir_all(&s.out).map_err(|e| fail(format!("{}: {e}", s.out.display())))?;
    }
    let mut meta = s.meta.clone();
    let (cap, provisional) = server.room.event_cap();
    meta.params
        .insert("event_queue_cap_count".into(), cap.into());
    meta.params
        .insert("event_queue_cap_provisional".into(), provisional.into());
    // LEAD-BOUND (Mustafa, 2026-11-18): the bound the room runs, in ms (12 ticks at 30 Hz), report-only (5.6 B2), read
    // from the live room (SEC-U4F-2).
    let lead_bound_ms = server.room.lead_bound().0;
    meta.params
        .insert("lead_bound_ms".into(), lead_bound_ms.into());
    // OQ-0001-6's option as the live room runs it (C-U4F2-8: `Room::cmd_timing`'s consumer; f6 pins it).
    let timing = match server.room.cmd_timing() {
        CmdTiming::A => "A",
        CmdTiming::B => "B",
    };
    meta.params.insert("cmd_timing".into(), timing.into());
    let mut logs = Logs::open(
        s.logs.then_some(s.out.as_path()),
        &meta,
        s.tick_start,
        s.bites.whole_as_lost,
    )
    .map_err(fail)?;
    logs.starved = (s.suite == "never-ack").then_some(0);
    logs.lag = (s.suite == "lag-switch").then_some(0);
    let mut rng = s.seed ^ 0x6c61_622d_6f66_6673;
    let order = connect_order(n, &s.bites);
    let mut clients: Vec<Client> = Vec::new();
    // -21's 1.5x clock (2.6): slot 0 on a fixed-lead endpoint (its stamps are its own rule's), its host clock at 1.5
    // from the measured start.
    let fast_from = (s.suite == "fast-clock").then(|| due_ns(s.warmup_ticks));
    let mut fast = fast_from.map(|_| FastClock {
        episodes: episodes(s.warmup_ticks, s.measured_ticks, s.fast_episode_ns),
        bound: i64::from(server.room.lead_bound().1),
        ..FastClock::default()
    });
    // -21's forgers (2.6): slot s < 11 forges kind s + 1 on a fixed-lead endpoint (one command a step, so each
    // forger's interval counts its commands), its twin's by role (2.12); the rest run the honest script.
    let forger = |slot: u8| (s.suite == "forged" && slot < FORGERS).then_some(slot + 1);
    for &slot in &order {
        let link = net.connect(s.profile.clone(), MAX_DATAGRAM, 0);
        let endpoint = match fast_from {
            Some(_) if slot == 0 => endpoint_of(s, slot).with_lead(s.lead),
            _ if forger(slot).is_some() => endpoint_of(s, slot).with_lead(s.lead),
            _ => endpoint_of(s, slot),
        };
        let behaviour = if s.suite == "clock" {
            Behaviour::never_fires(&scene, slot, s.seed)
        } else if s.suite == "clamp" && slot == 0 {
            // -21's clamp cheat (2.6): slot 0's open-loop script aims beyond reach inside the map; its twin takes the
            // clamp run's reported points (2.12).
            let points = match &s.twin_logs {
                Some(dir) => twin_points(dir, slot).map_err(fail)?,
                None => BTreeMap::new(),
            };
            Behaviour::script(&scene, slot, s.seed, "clamp")
                .expect("a script")
                .with_points(points)
        } else if s.suite == "lag-switch" && slot == 0 {
            // FP-4 (NC-U4F2-3): the lag client walks the fast clock's lane, so its movement can fail the cap's check.
            Behaviour::lane_walk(&scene, slot, s.seed)
        } else if s.behaviour == "brawl" {
            Behaviour::brawl(&scene, slot, s.seed).with_press_every(s.brawl_press_every)
        } else {
            Behaviour::script(&scene, slot, s.seed, &s.behaviour)
                .ok_or_else(|| Failure::Usage(format!("--behaviour {:?}", s.behaviour)))?
        };
        let behaviour = if s.every_step {
            behaviour.every_step()
        } else {
            behaviour
        };
        // Under the controller each step gives the commands the endpoint says are due (U4F-Q7 (iii) (c)).
        let behaviour = if s.controller {
            behaviour.controlled()
        } else {
            behaviour
        };
        // -22's suite: slot 0 brawls and never acknowledges (2.6), every other client brawls.
        let behaviour = if s.suite == "never-ack" && slot == 0 {
            behaviour.never_acks()
        } else if s.suite == "lag-switch" && slot == 0 {
            // -21's lag switch (2.6): slot 0's `Input`s held, then sent in a burst.
            behaviour.lag_switch(s.lag_hold_ms * 1_000_000, s.lag_every_ms * 1_000_000)
        } else if fast_from.is_some() && slot == 0 {
            // A command at every step, its lane's walk, its stamps by U4F-Q11 (v) (b) as F1 amends it.
            Behaviour::fast_clock(&scene, slot, s.seed, server.room.lead_bound().1)
        } else if let Some(kind) = forger(slot) {
            // FP-3, F-G: kind 7's forger needs the interval above its rest (the weapon's interval in commands).
            let rest = npt_bots::fire_rest_at(npt_sim::TICK_HZ, scene.weapon().interval_us)
                .map_or("none (an interval under two ticks)".to_owned(), |r| {
                    r.to_string()
                });
            let b = Behaviour::script(&scene, slot, s.seed, "steady")
                .and_then(|b| b.forger(kind, s.forge_every, server.room.lead_bound().1))
                .ok_or_else(|| {
                    Failure::Usage(format!(
                        "--forge-every {}: kind {kind}'s forger refuses it (kind 7 needs an interval above its \
                         rest, {rest})",
                        s.forge_every
                    ))
                })?;
            let b = if s.cmd_timing == CmdTiming::A {
                b.catchup_under_a()
            } else {
                b
            };
            if s.meta.role == "twin" { b.twin() } else { b }
        } else {
            behaviour
        };
        let bytes_log = match &s.bytes {
            None => None,
            Some(dir) => {
                std::fs::create_dir_all(dir)
                    .map_err(|e| fail(format!("{}: {e}", dir.display())))?;
                let p = dir.join(format!("client-{slot}.bytes"));
                let f = std::fs::File::create_new(&p)
                    .map_err(|e| fail(format!("{}: {e}", p.display())))?;
                Some(std::io::BufWriter::new(f))
            }
        };
        clients.push(Client {
            slot,
            endpoint,
            link,
            behaviour,
            offset_ns: below(mix(&mut rng), 1_000 * SEC),
            next_step_ns: 1 + below(mix(&mut rng), due_ns(1) - 1),
            inbox: Vec::new(),
            last_stamp: None,
            duplicates: 0,
            bytes_log,
            seq: u64::from(slot),
            first_sample_ns: None,
            read: false,
            fast_from: fast_from.filter(|_| slot == 0),
        });
    }
    // The clock suite's connects begun (each slot's first, then one per reconnect).
    let mut connects = u64::from(n);
    clients.sort_by_key(|c| c.slot);
    let digest = order_digest(&order);
    let mut mirror = Mirror::new(scene.slots(), scene.effect_rows(), scene.name() == "ES");
    // -57's three series only on a run whose header lists PM-0001-57 (the advisor's U4r build ruling, point 3): the
    // judge reads a run for a mark only when a header lists it, so no other run needs them.
    let with_57 = s.meta.marks.iter().any(|m| m == "PM-0001-57");
    let mut bursts: Vec<Vec<Burst>> = (0..n)
        .map(|_| {
            s.profile
                .bursts_ns
                .iter()
                .map(|&(start, len)| Burst {
                    start_ns: start,
                    end_ns: start + len,
                    ..Burst::default()
                })
                .collect()
        })
        .collect();
    let mut full_states: Vec<Vec<(u32, u64)>> = vec![Vec::new(); usize::from(n)];
    let recording_on = s.record.is_some() || s.record_mem;
    let mut recording = recording_on.then(|| Recording {
        scene: scene.id(),
        seed: s.seed,
        first_tick: s.tick_start,
        span_ms: ROOM_HISTORY_MS,
        generator: 0,
        ticks: Vec::new(),
    });
    let mut clock = SimClock::default();
    let mut k: u64 = 0;
    let measured_end = s.warmup_ticks + s.measured_ticks;
    let last_measured = logs.wide(s.tick_start.wrapping_add((measured_end.max(1) - 1) as u32));
    let mut events_stamped: u64 = 0;
    let mut clamp = ClampTally::default();
    // -21's attribution (2.4 item 6) of a forged run (not its twin): the items forged before the cutoff, the measured
    // end less the room's lead bound in ticks, so each one's verdict falls inside the run.
    let mut attribution = (s.suite == "forged" && s.meta.role != "twin")
        .then(|| Attribution::new((0..FORGERS.min(n)).map(|slot| (slot, slot + 1))));
    let cutoff = measured_end.saturating_sub(u64::from(server.room.lead_bound().1));
    // FP-2: the forgers' slots whose soldier is dead before the coming tick's command phase (every soldier starts
    // alive).
    let mut dead_before: Vec<u8> = Vec::new();
    let mut inputs: Vec<i64> = Vec::new();
    // The link-lost-overstated bite: the slots whose record it has written (bites builds only).
    #[cfg(feature = "bites")]
    let mut overstated = vec![false; usize::from(n)];
    let mut view = Vec::new();
    let mut summary = Summary {
        order_digest: digest,
        ..Summary::default()
    };
    let phase = |k: u64| {
        if k < s.warmup_ticks {
            Phase::Warmup
        } else if k < measured_end {
            Phase::Measured
        } else {
            Phase::After
        }
    };
    let mut stop = false;
    while !stop {
        let now = clock.now();
        // (1) packets due, and each new link's server end; (2) the server's I/O step; then (3)'s due count.
        let woke = wake(&mut server, listener.as_mut(), &mut schedule, now);
        for (slot, session, tick) in woke.admitted {
            // Carry 75's bound is checked on logged runs only: `events_stamped` counts what `room_tick` returns, which
            // a no-log run (--logs 0, or `npt-lab hash`'s regeneration) does not call, so there it stays 0
            // (advisor-u4r-r1, FP-7 item 6: RR3-L2 (b)).
            if events_stamped >= ADMIT_EVENTS_BOUND {
                return Err(fail(format!(
                    "slot {slot} admitted after {events_stamped} events past the event start (carry 75's bound \
                     {ADMIT_EVENTS_BOUND}): comes to the advisor first"
                )));
            }
            logs.admit(slot, session, tick, phase(k)).map_err(fail)?;
            // U4F-Q4 (b): the clock suite files no client hash, so no slot waits on one.
            if s.suite == "clock" {
                logs.gone(slot);
            }
        }
        for (slot, _) in woke.ended {
            logs.gone(slot);
        }
        let ends = std::mem::take(&mut server.session_ends);
        session_ends(&mut logs, ends, server.room.last_tick(), phase(k)).map_err(fail)?;
        // (3) the server tick, if due.
        for _ in 0..woke.to_run {
            let ph = phase(k);
            let t = server.room.last_tick().wrapping_add(1);
            server.room.tick();
            if let Some(r) = recording.as_mut() {
                r.ticks.push(server.room.tick_input().clone());
            }
            let records = npt_sim::SimCore::entity_records(server.room.sim()).to_vec();
            let index = Records::new(&records);
            // -21's clamp (2.4's last row): every clamp report in the room log; the clamp suite's cheat's tally.
            let rep = server.room.reports();
            for r in &rep.clamps {
                if s.logs {
                    let um = |cm: i32| i64::from(cm) * 10_000;
                    let mut e = EventRecord::new("clamp_report", ph)
                        .with("slot", r.slot)
                        .with("seq", r.seq)
                        .with("x_um", um(r.point[0]))
                        .with("y_um", um(r.point[1]))
                        .with("z_um", um(r.point[2]));
                    e.t = Some(logs.wide(t));
                    logs.room_event(&e).map_err(fail)?;
                }
                if s.suite == "clamp" && r.slot == 0 && !clamp.reported.insert(r.seq) {
                    clamp.reruns += 1;
                }
            }
            if s.suite == "clamp" {
                let reach = i64::from(scene.weapon().reach_cm);
                let me = records
                    .iter()
                    .find(|r| r.kind == npt_sim::EntityKind::Soldier && r.index == 0);
                for (c, _) in server.room.tick_input().commands.iter() {
                    // Beyond reach of slot 0's fire origin (140 cm above its feet: npt-sim's private CHEST_CM).
                    let d = me.map(|m| [0, 1, 2].map(|i| i64::from(c.aim[i] - m.grid[i])));
                    if c.slot == 0
                        && let Some([x, y, z]) = d
                        && x * x + y * y + (z - 140) * (z - 140) > reach * reach
                    {
                        clamp.sent.insert(c.seq);
                    }
                }
                clamp
                    .refused
                    .extend(rep.refusals.iter().filter(|x| x.slot == 0).map(|x| x.seq));
            }
            if let Some(a) = attribution.as_mut() {
                let input = server.room.tick_input();
                let (room, sim) = (server.room.refusals(), &rep.refusals);
                a.tick(room, sim, &input.commands, &dead_before);
                // FP-2: the forgers' soldiers dead after this tick (health 0), so before the next one's commands.
                dead_before = (records.iter())
                    .filter(|r| r.kind == npt_sim::EntityKind::Soldier && r.grid[6] == 0)
                    .filter_map(|r| u8::try_from(r.index).ok().filter(|s| *s < FORGERS))
                    .collect();
            }
            // -21's fast clock: slot 0's episode sums, each episode's record at its end (lab log).
            if let Some(f) = fast.as_mut() {
                let me = records
                    .iter()
                    .find(|r| r.kind == npt_sim::EntityKind::Soldier && r.index == 0)
                    .map(|r| [r.grid[0], r.grid[1], r.grid[2], r.grid[6]]);
                let c = server.room.client(0);
                let run = c.map_or(0, |c| c.commands_run());
                let me0 = clients.iter().find(|c| c.slot == 0);
                let left = me0.map_or(0, |c| c.behaviour.fast_left_out());
                let refused = c.map_or([0, 0, left], |c| {
                    [c.counters().lead_refused, c.counters().not_newer, left]
                });
                if let Some(mut e) = f.tick(k, me, run, refused) {
                    e.t = Some(logs.wide(t));
                    logs.lab_event(&e).map_err(fail)?;
                }
            }
            let mut sizes = vec![Vec::new(); usize::from(n)];
            let mut sets = vec![SetStats::default(); usize::from(n)];
            let mut link = vec![0u32; usize::from(n)];
            for slot in 0..n {
                let datagrams = server.room.take_datagrams(slot);
                let control = server.room.take_control(slot);
                let acked = server.room.client(slot).and_then(|c| c.last_acked());
                let errs = mirror.tick(slot, t, &datagrams, &control, quant, &index, acked);
                if s.logs && with_57 {
                    logs.errors(slot, t, errs);
                }
                for b in &control {
                    if let Ok(Message::FullState(f)) =
                        quant.decode(Direction::RoomToClient, Channel::FullState, b)
                    {
                        full_states[usize::from(slot)].push((f.tick, now));
                        for burst in &mut bursts[usize::from(slot)] {
                            if now >= burst.start_ns && !burst.healed {
                                burst.full_state = true;
                            }
                        }
                    }
                }
                sizes[usize::from(slot)] = datagrams.iter().map(Vec::len).collect();
                sets[usize::from(slot)] = mirror.latest(slot);
                // C62H-BOUND's link record: the fate-log entries this send pushes, joined one to one to its groups.
                let from = net.with_fate_log(|l| l.len());
                let has_session = server.session(slot).is_some();
                let hash = server.room.client(slot).and_then(|c| c.server_hash());
                server.send(slot, &datagrams, &control, now).map_err(fail)?;
                let c = net.with_fate_log(|l| l.server_datagrams_since(from));
                let lost = link_lost(datagrams.len(), has_session, hash, c).map_err(|e| {
                    fail(format!(
                        "slot {slot}, tick {t}: {e} (C62H-BOUND's link record cannot be joined)"
                    ))
                })?;
                if ph == Phase::Measured {
                    summary.link_sent += u64::from(c.sent);
                    summary.link_lost += u64::from(c.lost + c.burst);
                    summary.link_burst += u64::from(c.burst);
                }
                #[cfg(feature = "bites")]
                let lost = if s.bites.link_lost_overstated
                    && ph == Phase::Measured
                    && c.sent >= 1
                    && c.lost + c.burst == 0
                    && !overstated[usize::from(slot)]
                {
                    overstated[usize::from(slot)] = true;
                    1
                } else {
                    lost
                };
                link[usize::from(slot)] = lost;
                if let Some(c) = server.room.client(slot) {
                    for w in c.wraps() {
                        logs.wrapped(w, t, ph).map_err(fail)?;
                    }
                    if c.overflow_resync() {
                        summary.overflow_resyncs += 1;
                        let mut e = EventRecord::new("overflow_resync", ph).with("slot", slot);
                        e.t = Some(logs.wide(t));
                        logs.lab_event(&e).map_err(fail)?;
                    }
                    for b in &mut bursts[usize::from(slot)] {
                        if now >= b.start_ns && !b.healed {
                            b.queue_max = b.queue_max.max(c.queue().0);
                        }
                        if now >= b.end_ns && !b.eviction_logged {
                            b.eviction_logged = true;
                            let i = bursts_index(&s.profile, b.start_ns);
                            let mut e = EventRecord::new("burst_eviction", ph)
                                .with("slot", slot)
                                .with("burst", i)
                                .with("evicted", i64::from(c.burst_eviction()))
                                .with("baseline_age", c.baseline_age().map_or(-1, i64::from));
                            e.t = Some(logs.wide(t));
                            logs.room_event(&e).map_err(fail)?;
                        }
                    }
                }
            }
            for (slot, _) in std::mem::take(&mut server.ended) {
                logs.gone(slot);
            }
            session_ends(&mut logs, std::mem::take(&mut server.session_ends), t, ph)
                .map_err(fail)?;
            if s.logs {
                events_stamped += logs
                    .room_tick(&server.room, t, ph, &sets, &sizes, &inputs, &link)
                    .map_err(fail)?;
            }
            inputs.clear();
            summary
                .world_hashes
                .push((t, npt_sim::SimCore::state_hash(server.room.sim())));
            #[cfg(test)]
            summary.soldiers.push(
                npt_sim::SimCore::entity_records(server.room.sim())
                    .iter()
                    .filter(|r| r.kind == npt_sim::EntityKind::Soldier)
                    .map(|r| (r.index, r.grid))
                    .collect(),
            );
            k += 1;
            // Kinds 8, 9 and 11 by count at the cutoff: every item before it was received by now (P0), none after.
            if let Some(a) = attribution.as_mut().filter(|_| k == cutoff) {
                for slot in 0..FORGERS.min(n) {
                    let room = server
                        .room
                        .client(slot)
                        .map_or(0, |c| c.counters().ack_refused);
                    let pump = server.session(slot).map(|x| x.decoder.counters().clone());
                    let pump = pump.unwrap_or_default();
                    let counted = match slot + 1 {
                        8 => room,
                        9 => pump.trailing_bytes + pump.unknown_field,
                        11 => pump.wrong_channel,
                        _ => 0,
                    };
                    a.counted(slot, counted);
                }
            }
            if k == s.warmup_ticks && s.suite != "clock" {
                for c in &clients {
                    if !logs.started(c.slot) {
                        return Err(fail(format!(
                            "slot {} was not admitted, or resolved no tick, in the warm-up (carry 62 (d))",
                            c.slot
                        )));
                    }
                }
            }
        }
        // (5) the clients whose step is due, in slot order.
        for c in &mut clients {
            if c.next_step_ns > now {
                continue;
            }
            let host = c.host(now);
            let mut received = std::mem::take(&mut c.inbox);
            while let Some(r) = c.link.poll_recv(now) {
                received.push((r.channel, r.bytes, c.host(r.arrival_ns)));
            }
            if let Some(w) = c.bytes_log.as_mut() {
                write_step(w, host, &received).map_err(fail)?;
            }
            c.endpoint.input(None, &received, host);
            if s.suite == "clock" && !c.read {
                let clock = c.endpoint.clock();
                if c.first_sample_ns.is_none() && !clock.samples().is_empty() {
                    c.first_sample_ns = Some(host);
                }
                if let (Some(f), Some(est)) = (c.first_sample_ns, clock.offset_ns())
                    && host >= f + 2 * SEC
                {
                    // -47 (2.1's read instant): |estimate - true| at this connect's first step at or after 2 s from its
                    // first sample; the true offset is the host clock's seeded one.
                    let mut e = EventRecord::new("connect_clock", phase(k))
                        .with("slot", c.slot)
                        .with("seq", c.seq)
                        .with("offset_err_ns", est.abs_diff(c.offset_ns as i64));
                    e.ns = Some(now);
                    logs.lab_event(&e).map_err(fail)?;
                    c.read = true;
                    if connects < s.connects {
                        // The slot's next connect: a fresh link (the old one dropped, so the room frees the slot),
                        // endpoint and host offset.
                        c.link = net.connect(s.profile.clone(), MAX_DATAGRAM, 0);
                        c.endpoint = endpoint_of(s, c.slot);
                        c.offset_ns = below(mix(&mut rng), 1_000 * SEC);
                        c.inbox.clear();
                        (c.seq, c.first_sample_ns, c.read) = (connects, None, false);
                        connects += 1;
                        c.next_step_ns = now + 1;
                        continue;
                    }
                }
            }
            if let Some(newest) = c.endpoint.newest_tick() {
                mirror.view(c.slot, newest, &mut view);
            } else {
                view.clear();
            }
            // 5.3's p20: the fast client gives no command for the idle's span from the measured start.
            let idle = c
                .fast_from
                .is_some_and(|m| (m..m + s.fast_idle_ns).contains(&now));
            if !idle && c.behaviour.step(&mut c.endpoint, &[], host, &view) && !s.controller {
                let stamp = c.endpoint.newest_tick().map(|t| t.wrapping_add(s.lead));
                if stamp.is_some() && stamp == c.last_stamp {
                    c.duplicates += 1;
                }
                c.last_stamp = stamp;
            }
            let mut out = c.endpoint.step();
            c.behaviour.outgoing(host, &mut out.datagrams);
            c.behaviour.reliable(&mut out.reliable);
            let (seqs, copies) = (c.behaviour.take_forged(), c.behaviour.take_copies());
            if let Some(a) = attribution.as_mut().filter(|_| k < cutoff) {
                a.forged(c.slot, &seqs, &copies, phase(k) == Phase::Measured);
            }
            for held in c.behaviour.take_bursts() {
                let mut e = EventRecord::new("lag_burst", phase(k))
                    .with("slot", c.slot)
                    .with("held_ns", held);
                e.ns = Some(now);
                logs.lab_event(&e).map_err(fail)?;
            }
            for d in &out.datagrams {
                if matches!(
                    npt_proto::decode(Direction::ClientToRoom, Channel::Datagram, d),
                    Ok(Message::Input(_))
                ) {
                    inputs.push(d.len() as i64);
                }
                c.link.send_unreliable(d, now);
            }
            for (ch, b) in &out.reliable {
                c.link.send_reliable(*ch, b, now);
            }
            if s.suite != "clock" {
                logs.filed(c.slot, &out.hashes, &out.lost);
            }
            for w in &out.wraps {
                let t = c.endpoint.newest_tick().unwrap_or(s.tick_start);
                logs.wrapped(w, t, phase(k)).map_err(fail)?;
            }
            for b in &mut bursts[usize::from(c.slot)] {
                if b.healed || now < b.end_ns {
                    continue;
                }
                let fs = &full_states[usize::from(c.slot)];
                let fresh = out.hashes.iter().any(|&(tick, h)| {
                    let k_of = u64::from(tick.wrapping_sub(s.tick_start));
                    h != 0
                        && due_ns(k_of) >= b.end_ns
                        && !fs.iter().any(|&(f, at)| f == tick && at >= b.start_ns)
                });
                if fresh {
                    b.healed = true;
                    summary.heals += 1;
                    let i = bursts_index(&s.profile, b.start_ns);
                    let mut e = EventRecord::new("burst_heal", phase(k))
                        .with("after_link_ns", now - b.end_ns)
                        .with("slot", c.slot)
                        .with("burst", i)
                        .with("full_state", i64::from(b.full_state))
                        .with("queue_max_count", b.queue_max);
                    e.ns = Some(now);
                    logs.lab_event(&e).map_err(fail)?;
                }
            }
            c.next_step_ns = c.sim(out.next_due_ns).max(now + 1);
        }
        // Every client link's arrivals into its inbox (the socket's buffer), stamped on its host clock.
        for c in &mut clients {
            while let Some(r) = c.link.poll_recv(now) {
                let at = c.host(r.arrival_ns);
                c.inbox.push((r.channel, r.bytes, at));
            }
        }
        logs.flush().map_err(fail)?;
        // A recording run holds exactly the warm-up and the measured ticks: it stops at the measured phase's end.
        if recording_on && k >= measured_end {
            stop = true;
            continue;
        }
        if k >= measured_end && logs.written_through(last_measured) {
            summary.after_ticks = k - measured_end;
            stop = true;
            continue;
        }
        if k >= measured_end + s.after_max_ticks {
            if s.bites.id.is_none() {
                return Err(fail(format!(
                    "the measured ticks were not all resolved within the after phase's {} ticks (carry 62 (e))",
                    s.after_max_ticks
                )));
            }
            // A bite run ends at the cap (carry 73: tick16 past the wrap files its rebuilds under ticks the room never
            // ran, so the real ticks never resolve): the stop decides each client's unresolved ticks inside its span
            // lost, the wrong rebuilds are written after every tick the room ran, and the logs end clean, so the judge
            // reads the mismatch, never a cut log. A measured run still fails here.
            summary.after_ticks = k - measured_end;
            summary.after_capped = true;
            stop = true;
            continue;
        }
        // The next instant: a tick due, a client's step due, or the net's next internal event.
        let mut next = schedule.next_due_ns();
        for c in &clients {
            next = next.min(c.next_step_ns);
        }
        if let Some(e) = net.next_event_ns() {
            next = next.min(e.max(now + 1));
        }
        clock.advance_to(next.max(now + 1));
    }
    summary.ticks = k;
    summary.fate_log_sha256 = fate_sha256(&net);
    #[cfg(test)]
    net.with_fate_log(|l| {
        l.for_each_canonical(|b| {
            if let Some(head) = b.get(..32).filter(|_| b.len() >= 40) {
                summary.fates.push(head.try_into().expect("32 bytes"));
            }
        });
    });
    let mut last = Vec::new();
    for c in &mut clients {
        if let Some(w) = c.bytes_log.as_mut() {
            w.flush()
                .map_err(|e| fail(format!("a client's bytes log: {e}")))?;
        }
        let counters = server.room.client(c.slot).map(|x| x.counters().clone());
        let k = counters.unwrap_or_default();
        summary.commands += c.behaviour.given();
        summary.skipped += c.behaviour.skipped();
        summary.duplicates += c.duplicates;
        summary.late += k.late;
        summary.press_refused += k.press_refused;
        summary.buffer_full += k.buffer_full;
        summary.rewinds_refused += k.rewinds_refused;
        summary.stale += c.endpoint.stale();
        let e = EventRecord::new("client_counts", Phase::After)
            .with("slot", c.slot)
            .with("commands_count", c.behaviour.given())
            .with("steps_without_command_count", c.behaviour.skipped())
            .with("duplicate_stamps_count", c.duplicates)
            .with("stale_count", c.endpoint.stale())
            // U4f: the endpoint's controller counts, at the stop.
            .with("stamps_skipped_count", c.endpoint.stamps_skipped())
            .with(
                "unencodable_commands_count",
                c.endpoint.unencodable_commands(),
            )
            .with("lead_now_count", c.endpoint.lead_now().unwrap_or(-1))
            .with("unknown_pongs_count", c.endpoint.unknown_pongs());
        // The forged suite: each endpoint's commands left out (the twin's skip, the codec's pair rule) and each
        // forger's `Input`s whose forged form would not encode (2.12; 0 expected at P0).
        let e = if s.suite == "forged" {
            e.with("twin_left_out_count", c.endpoint.twin_left_out())
                .with("forge_unencodable_count", c.behaviour.forge_unencodable())
        } else {
            e
        };
        // 2.5 (SA-5): the room's refusal fields, and the live session's pump counters, at the stop.
        let d = server.session(c.slot).map(|x| x.decoder.counters().clone());
        let mut e = counts(e, Some(&k), d.as_ref());
        e.t = Some(logs.wide(server.room.last_tick()));
        last.push(e);
    }
    // -21's clamp kind (2.7; the sim's one weapon kind, hitscan): the cheat's points beyond reach run, of them those
    // with any sim refusal record (0 expected: it holds fire, no press edge), and option B's rerun reports (SA-10).
    if s.suite == "clamp" && s.meta.role != "twin" {
        last.push(
            EventRecord::new("clamp_kind", Phase::After)
                .with("sent_count", clamp.sent.len())
                .with(
                    "counted_count",
                    clamp.sent.intersection(&clamp.refused).count(),
                )
                .with("rerun_reports_count", clamp.reruns),
        );
    }
    // -21's forged kinds (2.4 item 6), with the room's dropped records beside them (C-U4F2-SEC-1).
    if let Some(a) = &attribution {
        last.extend(a.records(server.room.refusals_dropped()));
    }
    // D3-1 (f40): the run's measured reorder share from npt-l1's fate log, as the -3 probe computes it (main.rs's
    // `permille`): each direction's reordered x 1000 / delivered, floored; `permille` the lower of the two.
    let [up, down] = net.with_fate_log(|l| l.reorder_counts());
    let permille =
        |(delivered, reordered): (u64, u64)| (reordered * 1000).checked_div(delivered).unwrap_or(0);
    last.push(
        EventRecord::new("reorder", Phase::After)
            .with("permille", permille(up).min(permille(down)))
            .with("up_permille", permille(up))
            .with("down_permille", permille(down)),
    );
    summary.audit = mirror.audit.clone();
    last.push(
        EventRecord::new("fate_log", Phase::After).with("sha256", summary.fate_log_sha256.clone()),
    );
    last.push(
        EventRecord::new("run_lengths", Phase::After)
            .with("warmup_ticks_count", s.warmup_ticks)
            .with("measured_ticks_count", s.measured_ticks)
            .with("after_ticks_count", summary.after_ticks)
            .with("after_capped", summary.after_capped)
            .with("order_digest", format!("{digest:016x}")),
    );
    if let Some(r) = recording {
        let bytes = r.encode();
        let sha = sha256_hex(&bytes);
        if let Some(path) = &s.record {
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)
                    .map_err(|e| fail(format!("{}: {e}", dir.display())))?;
            }
            let mut f = std::fs::File::create_new(path).map_err(|e| {
                fail(format!(
                    "{}: {e} (a recording is never overwritten)",
                    path.display()
                ))
            })?;
            f.write_all(&bytes)
                .map_err(|e| fail(format!("{}: {e}", path.display())))?;
        }
        last.push(
            EventRecord::new("recording", Phase::After)
                .with("ticks_count", r.ticks.len())
                .with("bytes", bytes.len())
                .with("sha256", sha.clone())
                .with("span_ms", r.span_ms),
        );
        summary.recording = Some((bytes.len(), sha));
        if s.record_mem {
            summary.recording_bytes = Some(bytes);
        }
    }
    let a = &summary.audit;
    last.push(
        EventRecord::new("class_audit", Phase::After)
            .with("owner_count", a.owner)
            .with("spatial_count", a.spatial)
            .with("spatial_unchecked_count", a.spatial_unchecked)
            .with("public_count", a.public)
            .with("events_count", a.events)
            .with("team_count", 0)
            .with("breaches_count", a.breaches_count)
            .with(
                "first_breach",
                a.breaches.first().cloned().unwrap_or_default(),
            ),
    );
    let counts = logs.finish(Ended::Clean, &last).map_err(fail)?;
    summary.counts = counts;
    Ok(summary)
}

/// The clamp twin's points (2.12): each clamp report of `slot`'s in the clamp run's room log in `dir`, by its
/// sequence, in grid cm (the report's µm / 10,000; a rerun's first).
fn twin_points(dir: &std::path::Path, slot: u8) -> Result<BTreeMap<u16, [i32; 3]>, String> {
    let path = dir.join("room.jsonl");
    let file = std::fs::File::open(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let log = npt_log::read_log(std::io::BufReader::new(file))
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = BTreeMap::new();
    for e in log.events.iter().filter(|e| e.ev == "clamp_report") {
        let get = |k: &str| e.f.get(k).and_then(Value::as_i64);
        if let (Some(s), Some(seq), Some(x), Some(y), Some(z)) = (
            get("slot"),
            get("seq"),
            get("x_um"),
            get("y_um"),
            get("z_um"),
        ) && s == i64::from(slot)
        {
            let cm = |um: i64| (um / 10_000) as i32;
            out.entry(seq as u16).or_insert([cm(x), cm(y), cm(z)]);
        }
    }
    Ok(out)
}

/// The clamp suite's tally of its cheat's commands (slot 0; 2.4's last row, line 12 of the advisor's ruling): the
/// sequences run with a point beyond reach, those with any sim refusal record (whatever its kind), those the sim
/// reported clamped, and the reports of a sequence already reported (option B's reruns: SA-10).
#[derive(Default)]
struct ClampTally {
    sent: std::collections::BTreeSet<u16>,
    refused: std::collections::BTreeSet<u16>,
    reported: std::collections::BTreeSet<u16>,
    reruns: u64,
}

/// The lab log's `session_end` records (2.5; SA-5): each ended session's counts, at the room's tick `t`.
fn session_ends(logs: &mut Logs, ends: Vec<SessionEnd>, t: u32, ph: Phase) -> Result<(), String> {
    for x in ends {
        let e = EventRecord::new("session_end", ph)
            .with("slot", x.slot)
            .with("ended", x.ended);
        let mut e = counts(e, x.room.as_ref(), Some(&x.pump));
        e.t = Some(logs.wide(t));
        logs.lab_event(&e)?;
    }
    Ok(())
}

/// Which burst of the profile starts at `start` (its index, 0 to 9).
fn bursts_index(p: &Profile, start: u64) -> usize {
    p.bursts_ns.iter().position(|b| b.0 == start).unwrap_or(0)
}

/// One client step's received bytes, for `replay` (r3): host instant u64, count u32, then per message channel u8,
/// arrival u64, length u32 and the bytes, little-endian (replay.rs `read_steps` reads it back).
pub(crate) fn write_step(
    w: &mut impl Write,
    host_ns: u64,
    received: &[(Channel, Vec<u8>, u64)],
) -> Result<(), String> {
    let e = |e: std::io::Error| format!("a client's bytes log: {e}");
    w.write_all(&host_ns.to_le_bytes()).map_err(e)?;
    w.write_all(&(received.len() as u32).to_le_bytes())
        .map_err(e)?;
    for (ch, b, at) in received {
        let code: u8 = match ch {
            Channel::Datagram => 0,
            Channel::Control => 1,
            Channel::FullState => 2,
        };
        w.write_all(&[code]).map_err(e)?;
        w.write_all(&at.to_le_bytes()).map_err(e)?;
        w.write_all(&(b.len() as u32).to_le_bytes()).map_err(e)?;
        w.write_all(b).map_err(e)?;
    }
    Ok(())
}

/// The header params of a run (u4r-brief.md 2.1): the plan's, the carries' and the lead.
pub(crate) fn params(s: &Settings, suite: &str, twin: Option<&str>) -> BTreeMap<String, Value> {
    let mut p: BTreeMap<String, Value> = BTreeMap::new();
    let w = s.scene.weapon();
    p.insert("suite".into(), suite.into());
    // `params.cmd_timing` is the live room's, written by `run` (C-U4F2-8).
    // The one lead every client stamps with, only where `with_lead` fixes it (U4F-Q7 (i); TE-20); under U4f's
    // controller each client's lead is its own (`event.client_counts.lead_now_count`), and the cap it works under.
    if s.controller {
        p.insert(
            "controller_max_lead_count".into(),
            CONTROLLER_MAX_LEAD.into(),
        );
    } else {
        p.insert("lead_ticks_count".into(), s.lead.into());
    }
    p.insert("tick_start".into(), s.tick_start.into());
    p.insert(
        "ticks_below_wrap".into(),
        ((65_536 - s.tick_start % 65_536) % 65_536).into(),
    );
    p.insert("event_start_count".into(), s.event_start.into());
    p.insert("cmd_start_count".into(), s.cmd_start.into());
    p.insert("stand_ins".into(), Value::Array(Vec::new()));
    p.insert("stand_in_burn_ns".into(), 0.into());
    p.insert("stand_in_top_up_ns".into(), 0.into());
    p.insert("weapon_interval_us".into(), w.interval_us.into());
    p.insert("weapon_damage_count".into(), w.damage.into());
    p.insert("weapon_reach_cm".into(), w.reach_cm.into());
    p.insert("weapon_magazine_count".into(), w.magazine.into());
    p.insert("weapon_reload_ms".into(), w.reload_ms.into());
    p.insert("weapon_spread_urad".into(), w.spread_urad.into());
    p.insert("soldier_health_count".into(), s.scene.health().into());
    p.insert("warmup_ticks_count".into(), s.warmup_ticks.into());
    p.insert("measured_ticks_count".into(), s.measured_ticks.into());
    // The clock suite's clients run npt-bots' never-firing script, which `behaviour`'s row does not list (F4's rule):
    // the suite and its connect count name it.
    if s.suite == "lag-switch" {
        p.insert("lag_hold_ns".into(), (s.lag_hold_ms * 1_000_000).into());
        p.insert("lag_every_ns".into(), (s.lag_every_ms * 1_000_000).into());
    }
    if s.suite == "fast-clock" {
        p.insert("fast_clock_episode_ns".into(), s.fast_episode_ns.into());
        if s.fast_idle_ns > 0 {
            p.insert("fast_clock_idle_ns".into(), s.fast_idle_ns.into());
        }
    }
    if s.suite == "forged" {
        p.insert("forge_every_count".into(), s.forge_every.into());
    }
    if s.suite == "clock" {
        p.insert("connects_count".into(), s.connects.into());
    } else {
        p.insert("behaviour".into(), s.behaviour.clone().into());
    }
    if s.brawl_press_every > 0 {
        p.insert("brawl_press_every_count".into(), s.brawl_press_every.into());
    }
    p.insert(
        "features".into(),
        Value::Array(if cfg!(feature = "bites") {
            vec!["bites".into()]
        } else {
            Vec::new()
        }),
    );
    p.insert(
        "reorder_step_permille".into(),
        s.profile.reorder.permille.into(),
    );
    if !s.profile.bursts_ns.is_empty() {
        let starts: Vec<u64> = s.profile.bursts_ns.iter().map(|b| b.0).collect();
        p.insert("burst_starts_ns".into(), starts.into());
        // SERIES.md's row names npt-lab (L1) as its writer (the advisor's U4r build ruling 3, point 8 (D)).
        p.insert("burst_spacing_ns".into(), s.burst_spacing_ns.into());
    }
    if let Some(t) = twin {
        p.insert("twin".into(), t.into());
    }
    if s.every_step {
        p.insert("every_step".into(), true.into());
    }
    p
}

/// `scenario`'s usage line.
pub(crate) const USAGE: &str = "npt-lab scenario --run <id> --set <set> --date <YYYY-MM-DD> --commit <sha> \
--out <folder> --scene SS|ES --profile P0..P4|B1 --seed <n> [--role measured|twin|bite] [--marks <id,id...>] \
[--twin <run>] [--suite <name>] [--clients <n>] [--tick-start <n>] [--event-start <n>] [--cmd-start <n>] \
[--warmup-s <s>] [--measured-s <s>] [--after-max-s <s>] [--behaviour brawl|presses|clamp|steady] [--bursts <n>] \
[--burst-first-s <s>] [--burst-every-s <s>] [--bite <id> [--arm-tick <n>]] [--record <file>] [--bytes <folder>] \
[--every-step 1] [--lead <1..8>] [--logs 0 (with --record)] [--cmd-timing A|B] [--controller 0|1] \
[--brawl-press-every <n> (the brawl)] [--event-cap <n>] [--connects <n> (with --suite clock)] \
[--twin-logs <folder> (the clamp twin)] [--fast-clock-episode-s <s>] [--fast-clock-idle-s <s>] (with --suite fast-clock) \
[--lag-hold-ms <ms>] [--lag-every-ms <ms>] (with --suite lag-switch) \
[--forge-every <n, 2 to 1000000> (with --suite forged, at P0, no --lead)]";

/// B1's burst starts (u4r-brief.md 7.1, -18's row): `count` bursts, the first `first_s` after the measured phase's
/// first tick and one every `every_s`, each starting a seeded offset in [0, 1 tick) after its tick boundary.
pub(crate) fn burst_starts(
    seed: u64,
    warmup_ticks: u64,
    count: u64,
    first_s: u64,
    every_s: u64,
) -> Vec<u64> {
    let hz = u64::from(npt_sim::TICK_HZ);
    let mut rng = seed ^ 0x6275_7273_7473_0000;
    (0..count)
        .map(|i| {
            let boundary = due_ns(warmup_ticks + first_s * hz + i * every_s * hz);
            boundary + below(mix(&mut rng), due_ns(1))
        })
        .collect()
}

/// The `scenario` subcommand's options (after `scenario`), as `--name value` pairs, each once.
pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
    let mut it = argv.iter();
    while let Some(flag) = it.next() {
        let name = flag
            .strip_prefix("--")
            .ok_or(format!("unexpected argument {flag:?}"))?;
        let value = it.next().ok_or(format!("{flag} needs a value"))?;
        if map.insert(name, value).is_some() {
            return Err(format!("{flag} given twice"));
        }
    }
    let mut take = |k: &str| map.remove(k).map(str::to_owned);
    let need = |v: Option<String>, k: &str| v.ok_or(format!("--{k} is required"));
    let num = |v: Option<String>, k: &str, default: u64| match v {
        None => Ok(default),
        Some(s) => s
            .parse::<u64>()
            .map_err(|_| format!("--{k} {s:?} is not a whole number")),
    };
    let small =
        |v: u64, k: &str| u32::try_from(v).map_err(|_| format!("--{k} {v} is above 32 bits"));
    let run = need(take("run"), "run")?;
    let set = need(take("set"), "set")?;
    let date = need(take("date"), "date")?;
    let commit = need(take("commit"), "commit")?;
    let out = PathBuf::from(need(take("out"), "out")?);
    let scene_name = need(take("scene"), "scene")?;
    let scene = match scene_name.as_str() {
        "SS" => Scene::stress(),
        "ES" => Scene::estimate(),
        "CO" => Scene::course(),
        "TC" => Scene::targets(),
        other => return Err(format!("--scene {other:?}: SS, ES, CO or TC")),
    };
    let profile_name = need(take("profile"), "profile")?;
    let seed = num(Some(need(take("seed"), "seed")?), "seed", 0)?;
    let role = take("role").unwrap_or_else(|| "measured".to_owned());
    let marks: Vec<String> = take("marks")
        .map(|m| m.split(',').map(str::to_owned).collect())
        .unwrap_or_default();
    let twin = take("twin");
    let suite = take("suite").unwrap_or_else(|| "brawl".to_owned());
    let clients = num(take("clients"), "clients", u64::from(scene.slots()))?;
    let clients = u8::try_from(clients)
        .ok()
        .filter(|c| (1..=scene.slots()).contains(c))
        .ok_or(format!("--clients {clients}: 1 to {}", scene.slots()))?;
    let tick_start = small(num(take("tick-start"), "tick-start", 0)?, "tick-start")?;
    let event_start = small(num(take("event-start"), "event-start", 0)?, "event-start")?;
    let cmd_start = small(num(take("cmd-start"), "cmd-start", 0)?, "cmd-start")?;
    let hz = u64::from(npt_sim::TICK_HZ);
    // Checked products and sums (advisor-u4r-r1, FP-3: SEC-U4R-3, RR3-L3): each overflow a usage error naming its
    // option, so `run`'s plain sums of these lengths cannot overflow.
    let mut ticks_of = |k: &str, default: u64| -> Result<u64, String> {
        let s = num(take(k), k, default)?;
        s.checked_mul(hz).ok_or(format!(
            "--{k} {s}: its ticks ({s} x {hz}) overflow 64 bits"
        ))
    };
    let warmup_ticks = ticks_of("warmup-s", 10)?;
    let measured_ticks = ticks_of("measured-s", 60)?;
    let after_max_ticks = ticks_of("after-max-s", 30)?;
    let measured_end = warmup_ticks
        .checked_add(measured_ticks)
        .ok_or("--warmup-s and --measured-s: their ticks' sum overflows 64 bits".to_owned())?;
    let last_tick = measured_end
        .checked_add(after_max_ticks)
        .ok_or("--after-max-s: the run's last tick overflows 64 bits".to_owned())?;
    // advisor-u4r-r2, RR4-L2, after the run-length 64-bit checks (which keep their messages): the run's last tick,
    // counted from its first (the warm-up, measured and after phases' ticks), fits 32 bits, on every profile. The
    // burst block's own 64-bit check comes below it, then its 32-bit cap.
    if last_tick > u64::from(u32::MAX) {
        return Err(format!(
            "--warmup-s, --measured-s and --after-max-s: the run's last tick, {last_tick} ticks from its first, is \
             above {} (32 bits)",
            u32::MAX
        ));
    }
    let behaviour = take("behaviour").unwrap_or_else(|| "brawl".to_owned());
    if !["brawl", "presses", "clamp", "steady"].contains(&behaviour.as_str()) {
        return Err(format!(
            "--behaviour {behaviour:?}: brawl, presses, clamp or steady"
        ));
    }
    let brawl_press_every = num(take("brawl-press-every"), "brawl-press-every", 0)?;
    if brawl_press_every > 0 && behaviour != "brawl" {
        return Err("--brawl-press-every is the brawl's (--behaviour brawl)".to_owned());
    }
    let bursts = num(take("bursts"), "bursts", 10)?;
    let burst_first_s = num(take("burst-first-s"), "burst-first-s", 30)?;
    let burst_every_s = num(take("burst-every-s"), "burst-every-s", 60)?;
    let bite = take("bite");
    let arm_tick = small(
        num(take("arm-tick"), "arm-tick", u64::from(tick_start))?,
        "arm-tick",
    )?;
    let record = take("record").map(PathBuf::from);
    let bytes = take("bytes").map(PathBuf::from);
    let every_step = match take("every-step").as_deref() {
        None => false,
        Some("1") => true,
        Some(v) => return Err(format!("--every-step {v:?}: 1 or absent")),
    };
    let logs = match take("logs").as_deref() {
        None | Some("1") => true,
        Some("0") => false,
        Some(v) => return Err(format!("--logs {v:?}: 0 or 1")),
    };
    let cmd_timing = match take("cmd-timing").as_deref() {
        None | Some("B") => CmdTiming::B,
        Some("A") => CmdTiming::A,
        Some(v) => return Err(format!("--cmd-timing {v:?}: A or B")),
    };
    let controller = match take("controller").as_deref() {
        None | Some("0") => false,
        Some("1") => true,
        Some(v) => return Err(format!("--controller {v:?}: 0 or 1")),
    };
    let twin_logs = take("twin-logs").map(PathBuf::from);
    if twin_logs.is_some() && (twin.is_none() || role.as_str() != "twin" || suite != "clamp") {
        return Err(
            "--twin-logs is the clamp twin's: with --suite clamp, --twin and --role twin"
                .to_owned(),
        );
    }
    let lag_hold_ms = num(take("lag-hold-ms"), "lag-hold-ms", 200)?;
    let lag_every_ms = num(take("lag-every-ms"), "lag-every-ms", 6_000)?;
    if lag_hold_ms >= lag_every_ms || lag_hold_ms.max(lag_every_ms) > 3_600_000 {
        return Err("--lag-hold-ms below --lag-every-ms, each at most an hour".to_owned());
    }
    let fast_episode_s = num(take("fast-clock-episode-s"), "fast-clock-episode-s", 60)?;
    let fast_idle_s = num(take("fast-clock-idle-s"), "fast-clock-idle-s", 0)?;
    if fast_episode_s == 0 || fast_episode_s.max(fast_idle_s) > 3_600 {
        return Err(
            "--fast-clock-episode-s from 1 and --fast-clock-idle-s, each at most an hour"
                .to_owned(),
        );
    }
    let connects = num(take("connects"), "connects", 0)?;
    if (suite == "clock") != (connects > 0) {
        return Err("--suite clock and --connects <n> (from 1) go together".to_owned());
    }
    // C-U4F2-1 (SEC-U4F-1's rest): kinds 2 and 5 forge against an accepted command, so the interval is 2 or more;
    // at most 1,000,000 (FP-8, SA-U4F2-1: kind 10's episode is 8 intervals).
    let forge_every = match (suite == "forged", take("forge-every")) {
        (true, Some(v)) => match v.parse::<u64>() {
            Ok(k) if (2..=1_000_000).contains(&k) => k,
            _ => {
                return Err(format!(
                    "--forge-every {v:?}: a whole number from 2 to 1000000"
                ));
            }
        },
        (false, None) => 0,
        _ => return Err("--suite forged and --forge-every <n> go together".to_owned()),
    };
    if suite == "forged" && clients < FORGERS {
        return Err(format!(
            "--suite forged: at least {FORGERS} clients (a forger per kind)"
        ));
    }
    let event_cap = take("event-cap")
        .map(|v| {
            v.parse::<u32>()
                .ok()
                .filter(|c| *c >= 1)
                .ok_or(format!("--event-cap {v:?}: a whole number from 1"))
        })
        .transpose()?;
    let lead_override = take("lead")
        .map(|v| {
            v.parse::<u32>()
                .ok()
                .filter(|l| (1..=8).contains(l))
                .ok_or(format!("--lead {v:?}: 1 to 8"))
        })
        .transpose()?;
    // FP-8 (RR-U4F2-3): the forgers stamp at P0's fixed lead, and kind 10's burst needs P0's in-order arrival.
    if suite == "forged" && (profile_name != "P0" || lead_override.is_some()) {
        return Err(
            "--suite forged runs at --profile P0 on its forgers' fixed lead (no --lead)".to_owned(),
        );
    }
    if let Some(k) = map.keys().next() {
        return Err(format!("unknown option --{k}"));
    }
    if !["measured", "twin", "bite"].contains(&role.as_str()) {
        return Err(format!("--role {role:?}: measured, twin or bite"));
    }
    if (role == "bite") != bite.is_some() && role != "twin" {
        return Err("--role bite and --bite go together".to_owned());
    }
    if !logs && record.is_none() {
        return Err("--logs 0 is for a recording run (--record): a run keeps its logs".to_owned());
    }
    if (every_step || lead_override.is_some()) && !marks.is_empty() {
        return Err(
            "--every-step and --lead are D13's test-only settings: a run with them lists no mark"
                .to_owned(),
        );
    }
    if (every_step || lead_override.is_some()) && record.is_some() {
        // A recording's settings are its manifest entry's, which has neither key (hash.rs `Kind::Runner`; the
        // advisor's U4r build ruling, point 9 (a)).
        return Err(
            "--every-step and --lead are D13's test-only settings: a recording run (--record) takes neither"
                .to_owned(),
        );
    }
    // Where bursts are used (B1 with at least one: `burst_starts` runs only there), what is bounded: the last burst's
    // tick boundary counted from the run's first tick, ((bursts - 1) x every_s + first_s) x TICK_HZ + the warm-up's
    // ticks, first in 64 bits, which bounds every sum `burst_starts` makes (FP-3: RR3-L3); then (advisor-u4r-r2,
    // RR4-L2) that tick within 32 bits, --bursts at most 1,000, and --burst-every-s above 0 with more than one burst.
    if profile_name == "B1" && bursts > 0 {
        let last_burst = (bursts - 1)
            .checked_mul(burst_every_s)
            .and_then(|x| x.checked_add(burst_first_s))
            .and_then(|x| x.checked_mul(hz))
            .and_then(|x| x.checked_add(warmup_ticks))
            .ok_or(format!(
                "--bursts {bursts}, --burst-first-s {burst_first_s} and --burst-every-s {burst_every_s}: the last \
                 burst's tick overflows 64 bits"
            ))?;
        if last_burst > u64::from(u32::MAX) {
            return Err(format!(
                "--bursts {bursts}, --burst-first-s {burst_first_s} and --burst-every-s {burst_every_s}: the last \
                 burst's tick, {last_burst} ticks from the run's first, is above {} (32 bits)",
                u32::MAX
            ));
        }
        if bursts > 1_000 {
            return Err(format!("--bursts {bursts}: above 1000"));
        }
        if bursts > 1 && burst_every_s == 0 {
            return Err(format!(
                "--burst-every-s 0 with --bursts {bursts}: more than one burst needs an interval above 0 s"
            ));
        }
    }
    let profile = match profile_name.as_str() {
        "B1" => Profile::b1(&burst_starts(
            seed,
            warmup_ticks,
            bursts,
            burst_first_s,
            burst_every_s,
        )),
        p => Profile::by_name(p).ok_or(format!("--profile {p:?}: P0 to P4 or B1"))?,
    };
    let bites = match &bite {
        None => Bites::default(),
        Some(id) => Bites::of(id)?,
    };
    // advisor-u4r-r1, FP-6 item 1 (NC-U4R-2): an id whose only instance is the -30 pre-check's late-command replay
    // client is planted by no instance of a scenario run, so it is a usage error here (u4r-brief.md 2.1).
    if let (true, Some(id)) = (bites.late_command, &bite) {
        return Err(format!(
            "--bite {id} is the -30 pre-check's (npt-lab precheck)"
        ));
    }
    let lead = lead_override.unwrap_or_else(|| lead_for(&profile));
    let burst_spacing_ns = if profile.bursts_ns.is_empty() {
        0
    } else {
        burst_every_s.saturating_mul(SEC)
    };
    let mut s = Settings {
        meta: Meta {
            run,
            set,
            role,
            marks,
            date,
            commit,
            layer: "L1".to_owned(),
            profile: profile.name.to_owned(),
            seed,
            scene: scene.name().to_owned(),
            clients: u32::from(clients),
            bite,
            params: BTreeMap::new(),
        },
        out,
        scene,
        profile,
        clients,
        seed,
        tick_start,
        event_start,
        cmd_start,
        warmup_ticks,
        measured_ticks,
        after_max_ticks,
        burst_spacing_ns,
        behaviour,
        brawl_press_every,
        every_step,
        bites,
        arm_tick,
        lead,
        cmd_timing,
        // The clock suite's estimate is ClockSync's, which runs only under the controller.
        controller: controller || suite == "clock",
        event_cap,
        suite: suite.clone(),
        connects,
        twin_logs,
        lag_hold_ms,
        lag_every_ms,
        fast_episode_ns: fast_episode_s * SEC,
        fast_idle_ns: fast_idle_s * SEC,
        forge_every,
        record,
        bytes,
        logs,
        record_mem: false,
    };
    s.meta.params = params(&s, &suite, twin.as_deref());
    Ok(s)
}

/// `part / whole` as a decimal for a report line (0 when `whole` is 0): carry 49's figures, reported, never judged.
fn share(part: u64, whole: u64) -> f64 {
    if whole == 0 {
        0.0
    } else {
        part as f64 / whole as f64
    }
}

/// The summary line of a run. A `--logs 0` run prints "not counted (--logs 0)" for every figure taken from the logs'
/// counts or from `room_tick`, never 0 (the advisor's U4r build ruling, point 9 (b)); the counts that need no log stay.
pub(crate) fn summary_line(s: &Settings, o: &Summary) -> String {
    let c = &o.counts;
    let logged = |v: String| {
        if s.logs {
            v
        } else {
            "not counted (--logs 0)".to_owned()
        }
    };
    format!(
        "{} {} {} seed {}: {} ticks ({} after{}); lead {}; commands {} (steps without one {}, duplicate stamps {}); \
         late {}; press refused {}; buffer full {}; rewinds refused {}; stale {}; overflow resyncs {}; heals {}; \
         client hashes {} (sentinels {}); measured incomplete {}; decided lost at the stop {}; measured snapshot \
         datagrams {}, lost by the link {} (in bursts {}); measured client-ticks filed lost: the room sent nothing \
         {}, the link lost a datagram {}, neither {}; shots per measured second {}; carry 49: measured client sets \
         {}, holding every very large unit {}, projectiles per set {}; \
         audit {} owner, {} spatial checked against the radius, {} spatial not checked, {} breaches; fate log {}; \
         connect order digest {:016x}; in a building: {}",
        s.meta.run,
        s.scene.name(),
        s.profile.name,
        s.seed,
        o.ticks,
        o.after_ticks,
        if o.after_capped {
            ", the after phase capped with measured ticks unresolved: a bite run, carry 73"
        } else {
            ""
        },
        // A controller run has no one lead: the controller's cap (F10).
        if s.controller {
            format!("the controller's, at most {CONTROLLER_MAX_LEAD}")
        } else {
            s.lead.to_string()
        },
        o.commands,
        o.skipped,
        o.duplicates,
        o.late,
        o.press_refused,
        o.buffer_full,
        o.rewinds_refused,
        o.stale,
        o.overflow_resyncs,
        o.heals,
        logged(c.hashes.to_string()),
        logged(c.sentinels.to_string()),
        logged(c.incomplete_measured.to_string()),
        logged(c.lost_decided_at_stop.to_string()),
        o.link_sent,
        o.link_lost,
        o.link_burst,
        logged(c.lost_room_sent_nothing.to_string()),
        logged(c.lost_link.to_string()),
        logged(c.lost_unexplained.to_string()),
        logged(
            c.shots_measured
                .checked_div(c.measured_seconds)
                .unwrap_or(0)
                .to_string()
        ),
        logged(c.sets_measured.to_string()),
        logged(format!(
            "{} ({:.4})",
            c.large_measured,
            share(c.large_measured, c.sets_measured)
        )),
        logged(format!(
            "{:.2}",
            share(c.projectiles_measured, c.sets_measured)
        )),
        o.audit.owner,
        o.audit.spatial,
        o.audit.spatial_unchecked,
        o.audit.breaches_count,
        &o.fate_log_sha256[..16.min(o.fate_log_sha256.len())],
        o.order_digest,
        // BRAWL-1's figure: measured slot-ticks with feet in a building area, and their slots.
        logged(format!(
            "{} slot-ticks (slots {})",
            c.in_building_measured,
            if c.in_building_slots.is_empty() {
                "none".to_owned()
            } else {
                c.in_building_slots
                    .iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        )),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::path::Path;

    pub(crate) const COMMIT: &str = "0000000000000000000000000000000000000000";

    /// A test run's settings, through `scenario`'s own parser.
    pub(crate) fn settings(out: &Path, args: &str) -> Settings {
        let line = format!(
            "--run t --set t --date 2026-11-13 --commit {COMMIT} --out {} --scene SS {args}",
            out.display()
        );
        let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
        parse(&argv).unwrap_or_else(|e| panic!("{line}: {e}"))
    }

    /// One test's scratch folder; removed when the test passes, kept when it fails (the advisor's U4r build ruling 2,
    /// point 3 (ii)).
    pub(crate) struct Scratch(PathBuf);

    impl std::ops::Deref for Scratch {
        type Target = PathBuf;

        fn deref(&self) -> &PathBuf {
            &self.0
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                // The test's own teardown of the folder it made (absent if the test wrote nothing).
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
    }

    /// A fresh folder for one test's logs, named by the test and the process, inside the cargo target dir the test
    /// binary runs from: target/<profile>/npt-lab-tests/, beside deps/ (so under .gate/, plan :762, and removable by
    /// `cargo clean --target-dir`), never the system's temporary folder.
    pub(crate) fn scratch(name: &str) -> Scratch {
        let exe = std::env::current_exe().expect("the test binary");
        let profile = exe
            .parent()
            .and_then(Path::parent)
            .expect("the test binary lies in target/<profile>/deps/");
        let d = profile
            .join("npt-lab-tests")
            .join(format!("{name}-{}", std::process::id()));
        if d.exists() {
            std::fs::remove_dir_all(&d).expect("an old scratch folder of this process");
        }
        Scratch(d)
    }

    pub(crate) fn read(dir: &Path, process: &str) -> npt_log::Log {
        let f = std::fs::File::open(dir.join(format!("{process}.jsonl"))).expect("the log");
        npt_log::read_log(std::io::BufReader::new(f)).expect("the log reads")
    }

    /// r1's leg settings: 16 clients on P2 (its loss and jitter make each link's fates its own), 6 s of warm-up, 2 s
    /// measured.
    const R1_ARGS: &str =
        "--seed 1 --profile P2 --clients 16 --warmup-s 6 --measured-s 2 --after-max-s 10";

    /// One leg of r1, run as its own process (D-2's form: this test binary re-run through `current_exe`): a no-op
    /// unless `NPT_LAB_R1_LEG` names its folder; `NPT_LAB_R1_BITE` set selects the client-order bite.
    #[test]
    fn r1_leg() {
        let Ok(dir) = std::env::var("NPT_LAB_R1_LEG") else {
            return;
        };
        #[allow(unused_mut, reason = "the bite is set in bites builds only")]
        let mut s = settings(Path::new(&dir), R1_ARGS);
        #[cfg(feature = "bites")]
        if std::env::var("NPT_LAB_R1_BITE").is_ok() {
            s.bites.client_order = true;
            s.bites.id = Some(CLIENT_ORDER_BITE.to_owned());
        }
        run(&s).unwrap_or_else(|e| panic!("the leg: {e:?}"));
    }

    /// Runs two legs at once, each its own process, and returns their folders.
    fn two_legs(name: &str, bite: bool) -> [Scratch; 2] {
        let exe = std::env::current_exe().expect("the test binary");
        let dirs = [scratch(&format!("{name}-1")), scratch(&format!("{name}-2"))];
        let children: Vec<_> = dirs
            .iter()
            .map(|d| {
                let mut c = std::process::Command::new(&exe);
                c.args(["--exact", "scenario::tests::r1_leg", "--test-threads", "1"])
                    .env("NPT_LAB_R1_LEG", d.as_os_str());
                if bite {
                    c.env("NPT_LAB_R1_BITE", "1");
                }
                c.stdout(std::process::Stdio::null())
                    .spawn()
                    .expect("a leg starts")
            })
            .collect();
        for mut c in children {
            assert!(c.wait().expect("a leg ends").success(), "a leg failed");
        }
        dirs
    }

    /// r1's comparison: the fate logs' SHA-256s, the room's per-tick World hash `h` and `hs.server_hash`, and the
    /// clients' `hs.decoded_hash` and `tick.s.snap_incomplete`, over the same ticks. `Ok` with the tick records compared
    /// per process (room, client), or the first difference.
    fn twin_differences(a: &Path, b: &Path) -> Result<[usize; 2], String> {
        if let Some(e) = fate_difference(a, b) {
            return Err(e);
        }
        tick_differences(a, b)
    }

    /// A run's `fate_log` record's SHA-256, if its lab log holds one.
    fn fate_of(d: &Path) -> Option<serde_json::Value> {
        read(d, "lab")
            .events
            .iter()
            .find(|e| e.ev == "fate_log")
            .map(|e| e.f["sha256"].clone())
    }

    fn fate_difference(a: &Path, b: &Path) -> Option<String> {
        (fate_of(a) != fate_of(b))
            .then(|| format!("fate logs {:?} and {:?}", fate_of(a), fate_of(b)))
    }

    /// The per-tick half of r1's comparison: the tick records compared per process (room, client).
    fn tick_differences(a: &Path, b: &Path) -> Result<[usize; 2], String> {
        let mut compared = [0; 2];
        for (i, process) in ["room", "client"].into_iter().enumerate() {
            let (x, y) = (read(a, process), read(b, process));
            if x.ticks.len() != y.ticks.len() {
                return Err(format!(
                    "{process}: {} and {} tick records",
                    x.ticks.len(),
                    y.ticks.len()
                ));
            }
            for (p, q) in x.ticks.iter().zip(&y.ticks) {
                if p.t != q.t
                    || p.h != q.h
                    || p.hs != q.hs
                    || p.s.get("snap_incomplete") != q.s.get("snap_incomplete")
                {
                    return Err(format!("{process}: tick {} (and {}) differs", p.t, q.t));
                }
                compared[i] += 1;
            }
        }
        Ok(compared)
    }

    fn digest(d: &Path) -> String {
        read(d, "lab")
            .events
            .iter()
            .find(|e| e.ev == "run_lengths")
            .map(|e| e.f["order_digest"].to_string())
            .unwrap_or_default()
    }

    /// r1, the runner's twin test (§4.11 :406-407; carry 62 (f)): two processes, one seed and scenario, give equal
    /// fate logs and equal per-tick server and client hashes over the same ticks.
    #[test]
    fn r1_two_processes_of_one_seed_give_equal_fate_logs_and_per_tick_hashes() {
        let [a, b] = two_legs("r1", false);
        // advisor-u4r-r1, FP-6 item 5 (T-4): both fates are there (None against None is no comparison), and each
        // process's half compared records in both legs (its count, not the sum).
        assert!(
            fate_of(&a).is_some(),
            "the first leg's lab log holds its fate_log record"
        );
        let [room, client] = twin_differences(&a, &b).unwrap_or_else(|e| panic!("r1: {e}"));
        assert!(
            room > 0 && client > 0,
            "tick records compared: room {room}, client {client}"
        );
        assert!(room + client > 200, "ticks compared: {room} + {client}");
        assert_eq!(digest(&a), digest(&b), "slot order in both");
        println!(
            "r1: {room} room and {client} client tick records compared, equal; fate log {:?}; connect order digest {}",
            fate_of(&a),
            digest(&a)
        );
    }

    /// r1's bite (marks F29; bites builds only): the clients connect in a `HashMap`'s order, which each process's
    /// random state seeds, so the links' fates and then the per-tick hashes differ; equal order digests mean not
    /// exercised, never caught or missed (§4.14 :476).
    #[cfg(feature = "bites")]
    #[test]
    fn r1_bite_a_hash_map_client_order_makes_the_two_processes_differ() {
        let [a, b] = two_legs("r1-bite", true);
        let (da, db) = (digest(&a), digest(&b));
        println!("r1 bite: order digests {da} and {db}");
        assert_ne!(da, db, "not exercised: both processes drew one order");
        let fate = fate_difference(&a, &b).expect("r1 must catch the bite in the fate logs");
        let ticks =
            tick_differences(&a, &b).expect_err("r1 must catch the bite in the per-tick hashes");
        println!("r1 bite: caught: {fate}; {ticks}");
    }

    /// Carry 49 (TUNE-2), this brief's unit test: a scripted teleport above World's clamp bound is refused by name, one
    /// at the bound is accepted, and so are spawns at it.
    #[test]
    fn s1_a_scripted_placement_above_the_clamp_bound_is_refused_one_at_it_accepted() {
        assert_eq!(
            PLACEMENT_Z_BOUND_CM, 3_955,
            "World's stated bound: feet at 39.55 m"
        );
        let at = |z| Placement {
            slot: 3,
            position: [100, -200, z],
        };
        let e =
            check_placements(&[], &[at(PLACEMENT_Z_BOUND_CM + 1)]).expect_err("above the bound");
        assert!(e.contains("teleport") && e.contains("TUNE-2"), "{e}");
        assert!(check_placements(&[at(PLACEMENT_Z_BOUND_CM)], &[at(PLACEMENT_Z_BOUND_CM)]).is_ok());
        assert!(check_placements(&[at(4_000)], &[]).is_err(), "a spawn too");
    }

    /// Carry 74 as U4R-Q14 ruled: the round trip plus the jitter in ticks, rounded up, at least 1, plus 1.
    #[test]
    fn s2_each_profile_s_lead() {
        let lead = |p: Option<Profile>| lead_for(&p.expect("a profile"));
        assert_eq!(lead(Profile::by_name("P0")), 2);
        assert_eq!(lead(Profile::by_name("P1")), 3);
        assert_eq!(lead(Profile::by_name("P2")), 5);
        assert_eq!(lead(Some(Profile::b1(&[0]))), 5);
        assert_eq!(lead(Profile::by_name("P3")), 7);
        assert!(lead(Profile::by_name("P3")) <= 8, "within CMD_CATCHUP");
    }

    /// -18's ten bursts (u4r-brief.md 7.1): each a seeded offset in [0, 1 tick) after a tick boundary, 60 s apart from
    /// 30 s into the measured phase, and the same for the same seed.
    #[test]
    fn s3_burst_starts_lie_within_a_tick_of_their_boundaries() {
        let b = burst_starts(3, 300, 10, 30, 60);
        assert_eq!(b.len(), 10);
        for (i, s) in b.iter().enumerate() {
            let boundary = due_ns(300 + 900 + 1_800 * i as u64);
            assert!(*s >= boundary && *s < boundary + due_ns(1), "burst {i}");
        }
        assert_eq!(b, burst_starts(3, 300, 10, 30, 60));
        assert_ne!(b, burst_starts(4, 300, 10, 30, 60));
    }

    /// Carry 82 and §4.15: a runner recording (the room's `tick_input()` after each tick, generator 0, the room's
    /// span) regenerates byte for byte, holds exactly the warm-up and measured ticks, and replays through npt-sim's
    /// `Replayer` to the room's own per-tick World hashes.
    #[test]
    fn s5_a_runner_recording_regenerates_byte_for_byte_and_replays_to_the_room_s_hashes() {
        let scene = Scene::stress();
        let a = record(&scene, "P0", 1, 8, 100, 4, 2).expect("a recording");
        let b = record(&scene, "P0", 1, 8, 100, 4, 2).expect("again");
        assert_eq!(a, b, "regenerated byte for byte");
        let r = Recording::decode(&a).expect("it decodes");
        assert_eq!(
            (r.ticks.len(), r.first_tick, r.generator, r.span_ms),
            (180, 100, 0, ROOM_HISTORY_MS)
        );
        let dir = scratch("s5");
        let mut s = settings(
            &dir,
            "--seed 1 --profile P0 --clients 8 --tick-start 100 --warmup-s 4 --measured-s 2",
        );
        s.record_mem = true;
        let o = run(&s).expect("the run");
        assert_eq!(
            o.recording_bytes.as_deref(),
            Some(a.as_slice()),
            "the logged run records the same bytes"
        );
        let words: Vec<u32> = a
            .chunks(4)
            .map(|c| {
                let mut w = [0u8; 4];
                w[..c.len()].copy_from_slice(c);
                u32::from_le_bytes(w)
            })
            .collect();
        let mut rep = npt_sim::replay::Replayer::new();
        for w in words {
            rep.feed(w);
        }
        let mut replayed = Vec::new();
        while let npt_sim::replay::Step::Stepped(tick) = rep.step() {
            replayed.push((tick, rep.state_hash().expect("a hash after a step")));
        }
        assert_eq!(
            replayed, o.world_hashes,
            "the replay's hash at every tick is the room's"
        );
    }

    /// r4's run: SS, 48 clients in 3 x 16, P2, 6 s of warm-up and 2 s measured; its scratch guard is handed back, so a
    /// failing caller keeps the logs (advisor-u4r-r1, FP-6 item 6: T-5).
    fn r4_run(seed: u64, bite: &str) -> (Scratch, Summary) {
        let dir = scratch(&format!(
            "r4-{seed}{}",
            if bite.is_empty() { "" } else { "-bite" }
        ));
        let s = settings(
            &dir,
            &format!("--seed {seed} --profile P2 --warmup-s 6 --measured-s 2 {bite}"),
        );
        let o = run(&s).unwrap_or_else(|e| panic!("r4 seed {seed}: {e:?}"));
        (dir, o)
    }

    /// r4, the class audit (catalogue §6 at U4, item 5): every group the encoder writes into each client's snapshots
    /// and full states is one its class lets that client see, on 3 seeds; `team` and `public` sections do not exist in
    /// the spike (not exercised).
    #[test]
    fn r4_every_group_is_one_its_class_lets_that_client_see_on_three_seeds() {
        for seed in 1..=3 {
            let (_guard, o) = r4_run(seed, "");
            let a = &o.audit;
            println!(
                "r4 seed {seed}: {} ticks; audited owner {} (own-state records), spatial {} checked against the \
                 radius and {} not checked (entity records), public {} (headers), events {}; team 0 and public \
                 sections 0: not exercised; breaches {}",
                o.ticks,
                a.owner,
                a.spatial,
                a.spatial_unchecked,
                a.public,
                a.events,
                a.breaches_count
            );
            assert!(
                a.owner > 0 && a.spatial > 0 && a.public > 0,
                "seed {seed}: audited"
            );
            assert!(a.breaches.is_empty(), "seed {seed}: {:?}", a.breaches);
        }
    }

    /// r4's bite (U4E-Q13 (a)): `catalogue-5/owner-to-all`, its run's headers listing no mark, fails the audit by name.
    #[cfg(feature = "bites")]
    #[test]
    fn r4_bite_owner_to_all_fails_the_audit_by_name() {
        let (_guard, o) = r4_run(1, "--role bite --bite catalogue-5/owner-to-all");
        let first = o.audit.breaches.first().cloned().unwrap_or_default();
        println!(
            "r4 bite: {} breaches (16 texts kept); the first: {first}",
            o.audit.breaches_count
        );
        assert!(
            first.contains("own-state section holds"),
            "the owner rule fires: {first:?}"
        );
    }

    /// A joined Loopback session to `server` for slot `slot`'s credentials: its client link and endpoint, the room's
    /// answer delivered.
    fn joined(
        server: &mut Server,
        slot: u8,
        now: &mut u64,
    ) -> (npt_link::Loopback, ClientEndpoint, u64) {
        let (mut c, mut e, id) = join_only(server, slot, now);
        deliver(server, &mut c, &mut e, *now);
        (c, e, id)
    }

    /// `joined`'s join alone: the session accepted and its `Join` taken by the server's I/O step, before the room's
    /// next tick.
    fn join_only(
        server: &mut Server,
        slot: u8,
        now: &mut u64,
    ) -> (npt_link::Loopback, ClientEndpoint, u64) {
        let (s, mut c) = npt_link::Loopback::pair(MAX_DATAGRAM);
        let id = server.accept(Box::new(s), *now);
        let mut e = ClientEndpoint::new(join_of(1, slot), npt_sim::TICK_HZ);
        e.input(None, &[], *now);
        for (ch, b) in e.step().reliable {
            c.send_reliable(ch, &b, *now);
        }
        *now += 1;
        server.io(*now);
        (c, e, id)
    }

    /// One tick of the room, its output sent, and what reached `c` handed to `e`.
    fn deliver(server: &mut Server, c: &mut npt_link::Loopback, e: &mut ClientEndpoint, now: u64) {
        server.room.tick();
        for slot in 0..server.room.sim().scene().slots() {
            let d = server.room.take_datagrams(slot);
            let k = server.room.take_control(slot);
            server.send(slot, &d, &k, now).expect("sent");
        }
        let mut got = Vec::new();
        while let Some(r) = c.poll_recv(now + 1) {
            got.push((r.channel, r.bytes, r.arrival_ns));
        }
        e.input(None, &got, now + 1);
    }

    fn send_input(c: &mut npt_link::Loopback, e: &mut ClientEndpoint, now: u64) {
        e.input(Some(npt_sim::Command::default()), &[], now);
        for d in e.step().datagrams {
            c.send_unreliable(&d, now);
        }
    }

    fn send_leave(c: &mut npt_link::Loopback, now: u64) {
        let mut out = Vec::new();
        npt_proto::encode(&Message::Leave(npt_proto::Leave::Quit), &mut out).expect("a Leave");
        c.send_reliable(Channel::Control, &out, now);
    }

    fn lab_room(n: u8) -> Server {
        let scene = Scene::stress().with_stand_ins(StandIns::OFF);
        Server::build(&scene, 1, 0, usize::from(n), 0, 0, &Bites::default())
    }

    /// r8 (carry 76): a second endpoint joins with slot 0's token and bind while the first is live; the first is sent
    /// `Kick(replaced)` once; after it the first's Inputs and `Leave` reach no room call (the slot's whole counters
    /// unchanged), and the second keeps the slot.
    #[test]
    fn r8_a_replaced_session_s_inputs_and_leave_reach_no_room_call() {
        let mut server = lab_room(4);
        let mut now = 1_000;
        let (mut c1, mut e1, first) = joined(&mut server, 0, &mut now);
        assert_eq!(server.owner.get(&0), Some(&first));
        // The second join, read right after it, before the room's next tick (FP-2 (b)).
        let (mut c2, mut e2, second) = join_only(&mut server, 0, &mut now);
        assert_eq!(
            server.owner.get(&0),
            Some(&second),
            "the second holds slot 0"
        );
        assert!(
            server.ended.contains(&(0, "replaced")),
            "the first was told and dropped"
        );
        // advisor-u4r-r1, FP-2 (b): the first is told once, on Control, `Kick(replaced)`.
        let mut control = Vec::new();
        while let Some(r) = c1.poll_recv(now) {
            if r.channel == Channel::Control {
                control.push(r.bytes);
            }
        }
        assert_eq!(control.len(), 1, "one Control-channel message to the first");
        assert_eq!(
            Quant::default().decode(Direction::RoomToClient, Channel::Control, &control[0]),
            Ok(Message::Kick(Kick::Replaced)),
            "the first's Control message is Kick(replaced)"
        );
        deliver(&mut server, &mut c2, &mut e2, now);
        // FP-2 (a): the slot's whole counters (`ClientCounters`, ack_refused and ack_refused_commands among them);
        // nothing ticks inside the loop.
        let before = server.room.client(0).map(|c| c.counters().clone());
        for _ in 0..5 {
            now += 33_000_000;
            send_input(&mut c1, &mut e1, now);
            server.io(now + 1);
        }
        assert_eq!(
            server.room.client(0).map(|c| c.counters().clone()),
            before,
            "the first's Inputs reach no room call"
        );
        send_leave(&mut c1, now);
        server.io(now + 2);
        assert!(
            server.room.client(0).is_some(),
            "the first's Leave freed nothing"
        );
        assert_eq!(server.owner.get(&0), Some(&second));
    }

    /// r9 (carry 76): a session the room kicks (`Kick(overflow)`: a client that never acknowledges, under fire, with an
    /// event cap of 2) is dropped, and its later datagrams and its `Leave` reach no room call.
    #[test]
    fn r9_a_kicked_session_s_later_datagrams_reach_no_room_call() {
        let mut server = lab_room(48);
        server.room = server.room.clone().with_event_cap(2);
        let mut now = 1_000;
        let (mut c0, mut e0, _) = joined(&mut server, 0, &mut now);
        let (mut c1, mut e1, _) = joined(&mut server, 16, &mut now);
        let mut kicked = false;
        for _ in 0..600 {
            now += 33_333_333;
            // Slot 16 fires at slot 0 (another team); slot 0 never acknowledges.
            let target = npt_sim::SimCore::entity_records(server.room.sim())
                .iter()
                .find(|r| r.kind == npt_sim::EntityKind::Soldier && r.index == 0)
                .map(|r| [r.grid[0], r.grid[1], r.grid[2] + 100])
                .expect("slot 0's soldier");
            e1.input(
                Some(npt_sim::Command {
                    buttons: npt_sim::Command::FIRE,
                    aim: target,
                    ..npt_sim::Command::default()
                }),
                &[],
                now,
            );
            for d in e1.step().datagrams {
                c1.send_unreliable(&d, now);
            }
            server.io(now + 1);
            deliver(&mut server, &mut c1, &mut e1, now + 2);
            let mut drain = Vec::new();
            while let Some(r) = c0.poll_recv(now + 3) {
                drain.push((r.channel, r.bytes, r.arrival_ns));
            }
            // The kick read from the room itself, never from the glue under test.
            if server.room.client(0).is_some_and(|c| c.kicked()) {
                kicked = true;
                break;
            }
        }
        assert!(kicked, "the room kicked the client that never acknowledged");
        assert!(
            server.session(0).is_none(),
            "the lab dropped the kicked session"
        );
        send_input(&mut c0, &mut e0, now + 10);
        send_leave(&mut c0, now + 10);
        server.io(now + 11);
        assert!(
            server.room.client(0).is_some_and(|c| c.kicked()),
            "its Leave reached no room call: the kicked slot is not left"
        );
    }

    /// FP-1's four checks of a session that ended closed on `slot`: the room's slot freed, no owner, `(slot, "closed")`
    /// in `ended`, and the roster's slot not live (advisor-u4r-r2, N2-1 = TE-R2-1: a rejoin then is no replacement);
    /// each that fails, by name.
    fn closed_checks(server: &Server, slot: u8) -> Vec<String> {
        let mut failed = Vec::new();
        if server.room.client(slot).is_some() {
            failed.push(format!(
                "the room still holds slot {slot}: its slot was not freed"
            ));
        }
        if server.owner.contains_key(&slot) {
            failed.push(format!("the lab's owner map still holds slot {slot}"));
        }
        if !server.ended.contains(&(slot, "closed")) {
            failed.push(format!("ended holds no ({slot}, \"closed\")"));
        }
        if server.room.roster().is_live(slot) {
            failed.push(format!(
                "the roster still holds slot {slot}: a rejoin would be a replacement"
            ));
        }
        failed
    }

    /// Carry 76's boundary (advisor-u4r-r1, FP-1: NC-U4R-1 = SEC-U4R-1): a session whose decoder closes, (a) on a
    /// control-stream frame it refuses as malformed, or whose link closes, (b) its client end dropped, ends, and the
    /// room's slot is freed (`Room::leave`: "a closed link"); after (a) a new endpoint with that slot's token and bind is
    /// admitted. It is not a replacement because the roster freed the slot (`closed_checks`' roster check, read before
    /// the rejoin: `Room::join` reports a replacement only for a slot the roster holds live). Each case's checks are
    /// collected, so a failure names its case.
    #[test]
    fn fp1_a_closed_decoder_or_link_ends_the_session_and_frees_the_room_s_slot() {
        // (a) A frame the pump refuses with an error other than WrongChannel or WrongDirection: `close_malformed`.
        let mut a = lab_room(4);
        let mut now = 1_000;
        let (mut c, _e, _) = joined(&mut a, 0, &mut now);
        let bad = [0xffu8; 3];
        let refused = Quant::default()
            .decode(Direction::ClientToRoom, Channel::Control, &bad)
            .expect_err("the decoder refuses the frame");
        assert!(
            !matches!(
                refused,
                npt_proto::DecodeError::WrongChannel | npt_proto::DecodeError::WrongDirection
            ),
            "refused as {refused:?}: a forged kind, which the pump counts and keeps"
        );
        c.send_reliable(Channel::Control, &bad, now);
        now += 1;
        a.io(now);
        let case_a = closed_checks(&a, 0);
        let (_c2, _e2, again) = joined(&mut a, 0, &mut now);
        // (b) The client end dropped: the server end's link reads Closed.
        let mut b = lab_room(4);
        let mut now_b = 1_000;
        let (c, _e, _) = joined(&mut b, 0, &mut now_b);
        drop(c);
        b.io(now_b + 1);
        let case_b = closed_checks(&b, 0);
        println!("fp1: case (a) {case_a:?}; case (b) {case_b:?}; (a)'s new session {again}");
        assert!(
            case_a.is_empty() && case_b.is_empty(),
            "case (a), the decoder closed: {case_a:?}; case (b), the link closed: {case_b:?}"
        );
        assert_eq!(
            a.owner.get(&0),
            Some(&again),
            "(a): the new endpoint is admitted"
        );
    }

    /// The reliable messages' channel (the module text): exactly one stream channel takes each, never a fall-back.
    #[test]
    fn s4_a_reliable_message_goes_on_the_one_channel_that_takes_it() {
        let q = Quant::default();
        let mut ja = Vec::new();
        npt_proto::encode(
            &Message::JoinAccepted(npt_proto::JoinAccepted::default()),
            &mut ja,
        )
        .expect("encodes");
        assert_eq!(stream_of(&ja, q), Ok(Channel::Control));
        let mut fs = Vec::new();
        npt_proto::encode(
            &Message::FullState(npt_proto::FullState {
                tick: 7,
                last_run: None,
                own: Vec::new(),
                entities: Vec::new(),
            }),
            &mut fs,
        )
        .expect("encodes");
        assert_eq!(stream_of(&fs, q), Ok(Channel::FullState));
        assert!(stream_of(&[0, 1, 99], q).is_err(), "no channel takes it");
    }

    /// The advisor's U4r build ruling, point 5: `ROOM_HISTORY_MS` (a copy of npt-room's private `HISTORY_MS`) pinned to
    /// the room by behaviour. With the ring full, a firing command run at tick T claiming view tick T - C, C being
    /// `HitHistory::for_ms(ROOM_HISTORY_MS)`'s capacity, gets a Rewind (the ring then holds T - C to T - 1), and one
    /// claiming T - C - 1 counts in `rewinds_refused`.
    #[test]
    fn p5_the_room_s_ring_holds_room_history_ms_s_capacity_and_no_more() {
        let c = npt_sim::HitHistory::for_ms(ROOM_HISTORY_MS)
            .expect("a ring")
            .capacity();
        let mut server = lab_room(4);
        let mut now = 1_000;
        let (mut link, mut e, _) = joined(&mut server, 0, &mut now);
        for _ in 0..c + 10 {
            now += 33_333_333;
            deliver(&mut server, &mut link, &mut e, now);
        }
        let mut seen = Vec::new();
        for (seq, d) in [(0u16, c), (1, c + 1)] {
            let t = server.room.last_tick().wrapping_add(1);
            let cmd = npt_sim::Command {
                slot: 0,
                tick: t,
                seq,
                buttons: npt_sim::Command::FIRE,
                aim: [0, 0, 100],
                view_tick: t.wrapping_sub(d),
                view_fraction: 0,
                ..npt_sim::Command::default()
            };
            let mut b = Vec::new();
            npt_proto::encode(
                &Message::Input(npt_proto::Input {
                    commands: vec![cmd],
                    ..npt_proto::Input::default()
                }),
                &mut b,
            )
            .expect("an Input");
            now += 33_333_333;
            link.send_unreliable(&b, now);
            server.io(now + 1);
            let refused = |s: &Server| s.room.client(0).map_or(0, |x| x.counters().rewinds_refused);
            let before = refused(&server);
            deliver(&mut server, &mut link, &mut e, now + 2);
            let rewind = server
                .room
                .tick_input()
                .commands
                .iter()
                .find(|(x, _)| x.slot == 0 && x.seq == seq)
                .map(|(_, r)| r.is_some());
            seen.push((d, rewind, refused(&server) - before));
        }
        println!("p5: capacity {c} ticks; (view age, Rewind given, refused) {seen:?}");
        assert_eq!(seen, [(c, Some(true), 0), (c + 1, Some(false), 1)]);
    }

    /// The advisor's U4r build ruling, point 2: mirror.rs's `INTEREST_RADIUS_CM` (a copy of npt-room's private
    /// `RADIUS_CM`, room.rs:29) pinned to the room by placement. Two soldiers teleported (through `check_placements`)
    /// to exactly the copy's radius and one cm beyond it from slot 0's soldier, in slot 0's own area; slot 0's join
    /// FullState, decoded by the mirror, holds the first and not the second. `None` when no direction lands both
    /// placements exactly in that area (the copy then unpinned).
    fn radius_pin() -> Option<(bool, bool)> {
        use crate::mirror::INTEREST_RADIUS_CM;
        let r = i32::try_from(INTEREST_RADIUS_CM).expect("a radius in i32");
        let mut server = lab_room(4);
        server.room.tick();
        let records = npt_sim::SimCore::entity_records(server.room.sim()).to_vec();
        let at = |rs: &[npt_sim::EntityRecord], s: u16| {
            rs.iter()
                .find(|x| x.kind == npt_sim::EntityKind::Soldier && x.index == s)
                .copied()
                .expect("a soldier")
        };
        let me = at(&records, 0);
        let p = [me.grid[0], me.grid[1], me.grid[2]];
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for &(ax, ay) in &dirs {
            for &(bx, by) in &dirs {
                let mut s = lab_room(4);
                s.room.tick();
                let a = Placement {
                    slot: 20,
                    position: [p[0] + ax * r, p[1] + ay * r, p[2]],
                };
                let b = Placement {
                    slot: 21,
                    position: [p[0] + bx * (r + 1), p[1] + by * (r + 1), p[2]],
                };
                check_placements(&[], &[a, b]).expect("inside carry 49's bound");
                s.room.script(&[], &[a, b], &[]);
                s.room.tick();
                let recs = npt_sim::SimCore::entity_records(s.room.sim()).to_vec();
                let (ra, rb, rm) = (at(&recs, 20), at(&recs, 21), at(&recs, 0));
                let landed =
                    |x: &npt_sim::EntityRecord, q: [i32; 3]| x.grid[0] == q[0] && x.grid[1] == q[1];
                if !(landed(&ra, a.position)
                    && landed(&rb, b.position)
                    && ra.area == rm.area
                    && rb.area == rm.area)
                {
                    continue;
                }
                let (srv, mut cl) = npt_link::Loopback::pair(MAX_DATAGRAM);
                s.accept(Box::new(srv), 1);
                let mut e = ClientEndpoint::new(join_of(1, 0), npt_sim::TICK_HZ);
                e.input(None, &[], 1);
                for (ch, bytes) in e.step().reliable {
                    cl.send_reliable(ch, &bytes, 1);
                }
                s.io(2);
                let t = s.room.last_tick().wrapping_add(1);
                s.room.tick();
                let recs = npt_sim::SimCore::entity_records(s.room.sim()).to_vec();
                let control = s.room.take_control(0);
                let datagrams = s.room.take_datagrams(0);
                let mut mirror = Mirror::new(48, Scene::stress().effect_rows(), false);
                mirror.tick(
                    0,
                    t,
                    &datagrams,
                    &control,
                    Quant::default(),
                    &Records::new(&recs),
                    s.room.client(0).and_then(|c| c.last_acked()),
                );
                let mut view = Vec::new();
                mirror.view(0, t, &mut view);
                let holds = |slot: u8| view.iter().any(|(x, _)| *x == slot);
                println!(
                    "p2: slot 0 at {p:?} (area {}); slot 20 at {:?}, slot 21 at {:?}; the join FullState of tick {t} \
                     holds slot 20 {}, slot 21 {} ({} soldiers in view)",
                    rm.area,
                    a.position,
                    b.position,
                    holds(20),
                    holds(21),
                    view.len()
                );
                return Some((holds(20), holds(21)));
            }
        }
        None
    }

    #[test]
    fn p2_the_mirror_s_radius_copy_is_the_room_s_by_placement() {
        match radius_pin() {
            Some(held) => assert_eq!(held, (true, false), "at the radius held, one cm beyond not"),
            // The placement exists at the head, so a None is a failure (the advisor's U4r build ruling 2, point 4 (a)).
            None => panic!(
                "no direction lands both placements in slot 0's area: the copy is no longer pinned"
            ),
        }
    }

    /// The advisor's U4r build ruling, point 9 (a): `--lead` and `--every-step` are refused with `--record` too.
    #[test]
    fn p9_lead_and_every_step_are_refused_with_record() {
        let dir = scratch("p9");
        let line = |extra: &str| {
            format!(
                "--run t --set t --date 2026-11-13 --commit {COMMIT} --out {} --scene SS --seed 1 --profile P0 \
                 --record {}/r.rec {extra}",
                dir.display(),
                dir.display()
            )
        };
        let argv = |l: String| l.split_whitespace().map(str::to_owned).collect::<Vec<_>>();
        assert!(parse(&argv(line(""))).is_ok(), "a recording run");
        for extra in ["--lead 3", "--every-step 1"] {
            let e = parse(&argv(line(extra))).expect_err(extra);
            assert!(
                e.contains("a recording run (--record) takes neither"),
                "{extra}: {e}"
            );
        }
    }

    /// advisor-u4r-r1, FP-3 (SEC-U4R-3, RR3-L3): a length whose ticks overflow 64 bits is a usage error naming its
    /// option, never a wrapped length. advisor-u4r-r2, TE-R2-2: each 64-bit refusal's exact text (the last burst's
    /// tick on a B1 line, the warm-up and measured sum, the after phase); RR4-L2: a run's last tick and the last
    /// burst's tick above u32::MAX, more than 1,000 bursts and an interval of 0 s between bursts, each a usage error
    /// naming its options, and each bound's own value accepted.
    #[test]
    fn fp3_a_length_whose_ticks_overflow_is_a_usage_error_naming_its_option() {
        let parsed = |profile: &str, extra: &str| {
            let line = format!(
                "--run t --set t --date 2026-11-14 --commit {COMMIT} --out x --scene SS --seed 1 --profile {profile} \
                 {extra}"
            );
            let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
            parse(&argv)
        };
        let e = parsed("P0", "--measured-s 1000000000000000000")
            .expect_err("--measured-s 10^18 x 30 overflows");
        assert!(e.starts_with("--measured-s 1000000000000000000: "), "{e}");
        // TE-R2-2 (1), on a B1 line: the 64-bit check comes before RR4-L2's bounds.
        let e = parsed(
            "B1",
            "--bursts 2 --burst-first-s 0 --burst-every-s 1000000000000000000",
        )
        .expect_err("the last burst's tick overflows");
        assert!(
            e.starts_with(
                "--bursts 2, --burst-first-s 0 and --burst-every-s 1000000000000000000: the last burst's tick \
                 overflows 64 bits"
            ),
            "{e}"
        );
        // TE-R2-2 (2): each product fits, their sum does not.
        let e = parsed(
            "P0",
            "--warmup-s 310000000000000000 --measured-s 310000000000000000",
        )
        .expect_err("the sum overflows");
        assert_eq!(
            e,
            "--warmup-s and --measured-s: their ticks' sum overflows 64 bits"
        );
        // TE-R2-2 (3), in full digits (`num` parses u64): the sum fits, the after phase's addition does not.
        let e = parsed(
            "P0",
            "--warmup-s 300000000000000000 --measured-s 300000000000000000 --after-max-s 100000000000000000",
        )
        .expect_err("the run's last tick overflows");
        assert_eq!(e, "--after-max-s: the run's last tick overflows 64 bits");
        // RR4-L2's ruled case: the last burst's tick fits 64 bits, not 32.
        let e = parsed("B1", "--bursts 2 --burst-every-s 600000000000")
            .expect_err("the last burst's tick above 32 bits");
        assert!(e.contains("--burst-every-s 600000000000"), "{e}");
        assert_eq!(
            e,
            "--bursts 2, --burst-first-s 30 and --burst-every-s 600000000000: the last burst's tick, 18000000001200 \
             ticks from the run's first, is above 4294967295 (32 bits)"
        );
        // RR4-L2: more than 1,000 bursts; 1,000 accepted.
        let e = parsed("B1", "--bursts 1001").expect_err("1,001 bursts");
        assert_eq!(e, "--bursts 1001: above 1000");
        assert!(parsed("B1", "--bursts 1000").is_ok(), "1,000 bursts");
        // RR4-L2: an interval of 0 s with more than one burst; one burst at 0 s accepted.
        let e = parsed("B1", "--bursts 2 --burst-every-s 0").expect_err("two bursts 0 s apart");
        assert_eq!(
            e,
            "--burst-every-s 0 with --bursts 2: more than one burst needs an interval above 0 s"
        );
        assert!(
            parsed("B1", "--bursts 1 --burst-every-s 0").is_ok(),
            "one burst"
        );
        // RR4-L2: a run's last tick past 32 bits that fits 64 (the defaults' 300 + 1,800 ticks, then the after phase);
        // u32::MAX is odd, so never a whole-second length; the largest below it (4,294,967,280 ticks) accepted.
        let e =
            parsed("P0", "--after-max-s 143165507").expect_err("the run's last tick above 32 bits");
        assert_eq!(
            e,
            "--warmup-s, --measured-s and --after-max-s: the run's last tick, 4294967310 ticks from its first, is \
             above 4294967295 (32 bits)"
        );
        assert!(
            parsed("P0", "--after-max-s 143165506").is_ok(),
            "4,294,967,280 ticks"
        );
        // BRAWL-1 round 1, RR-B1-4: the last burst's 32-bit cap at its boundary, (1 x 143,165,566 + 0) x 30 + 300 =
        // 4,294,967,280 ticks accepted (`burst_starts` has no in-run check), 30 ticks more refused.
        assert!(
            parsed(
                "B1",
                "--bursts 2 --burst-first-s 0 --burst-every-s 143165566"
            )
            .is_ok(),
            "the last burst at 4,294,967,280 ticks"
        );
        let e = parsed(
            "B1",
            "--bursts 2 --burst-first-s 0 --burst-every-s 143165567",
        )
        .expect_err("the last burst's tick above 32 bits");
        assert_eq!(
            e,
            "--bursts 2, --burst-first-s 0 and --burst-every-s 143165567: the last burst's tick, 4294967310 ticks from \
             the run's first, is above 4294967295 (32 bits)"
        );
    }

    /// advisor-u4r-r1, FP-6 items 1 and 2 (NC-U4R-2; EA-U4R-3 = RR3-L8; bites builds only): `scenario` refuses the
    /// pre-check's late-command id as a usage error, and `Bites::of` gives the pump's wallclock id to the pump alone,
    /// never to the schedule (whose `with_bite` refuses it).
    #[cfg(feature = "bites")]
    #[test]
    fn fp6_the_late_command_id_is_refused_by_scenario_and_the_wallclock_id_is_the_pump_s_alone() {
        let line = format!(
            "--run t --set t --date 2026-11-14 --commit {COMMIT} --out x --scene SS --seed 1 --profile P0 \
             --role bite --bite PM-0001-30/late-command"
        );
        let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
        let e = parse(&argv).expect_err("the pre-check's id");
        assert_eq!(
            e,
            "--bite PM-0001-30/late-command is the -30 pre-check's (npt-lab precheck)"
        );
        let b = Bites::of("PM-0001-30/wallclock").expect("the pump's id");
        assert!(!b.schedule && b.pump, "{b:?}");
    }

    /// The measured phase's sum of `tick.s.snap_incomplete` in a run's client log, and whether its lab log holds -57's
    /// series.
    fn incomplete_and_57(dir: &Path) -> (i64, bool) {
        let client = read(dir, "client");
        let sum = client
            .ticks
            .iter()
            .filter(|t| t.ph == Phase::Measured)
            .filter_map(|t| t.s.get("snap_incomplete"))
            .flatten()
            .sum();
        let lab = read(dir, "lab");
        (
            sum,
            lab.ticks.iter().any(|t| t.s.contains_key("pos_err_um")),
        )
    }

    /// A short L1 P0 run: SS, 16 clients, 6 s of warm-up, 2 s measured.
    const P0_SHORT: &str =
        "--seed 2 --profile P0 --clients 16 --warmup-s 6 --measured-s 2 --after-max-s 10";

    /// The advisor's U4r build ruling, points 3 and 4 (the clean half): a short L1 P0 run whose header lists
    /// PM-0001-57 writes -57's series and files no tick lost in the measured phase (sum 0); one whose header does not
    /// list it writes none of them.
    #[test]
    fn p3_p4_a_short_p0_run_writes_57_s_series_only_when_listed_and_files_no_tick_lost() {
        let with = scratch("p3-with");
        run(&settings(
            &with,
            &format!("{P0_SHORT} --marks PM-0001-17,PM-0001-57"),
        ))
        .expect("the run");
        let without = scratch("p3-without");
        run(&settings(
            &without,
            &format!("{P0_SHORT} --marks PM-0001-17"),
        ))
        .expect("the run");
        let (sum_with, series_with) = incomplete_and_57(&with);
        let (sum_without, series_without) = incomplete_and_57(&without);
        println!(
            "p3/p4 clean: listing -57: measured incomplete {sum_with}, -57 series {series_with}; not listing it: \
             {sum_without}, {series_without}"
        );
        assert_eq!((sum_with, sum_without), (0, 0));
        assert!(series_with && !series_without);
    }

    /// The advisor's U4r build ruling, point 4 (carry 62 (h)'s bite, PM-0001-17/whole-as-lost; bites builds only): the
    /// bitten writer files each client's first tick resolved whole in the measured phase as lost, so the measured
    /// phase's sum of `tick.s.snap_incomplete` is the client count (the clean run's is 0: the test above).
    #[cfg(feature = "bites")]
    #[test]
    fn p4_bite_whole_as_lost_makes_the_measured_incomplete_sum_the_client_count() {
        let dir = scratch("p4-bite");
        let s = settings(
            &dir,
            &format!("{P0_SHORT} --marks PM-0001-17 --role bite --bite PM-0001-17/whole-as-lost"),
        );
        run(&s).expect("the run");
        let (sum, _) = incomplete_and_57(&dir);
        println!("p4 bite: measured incomplete {sum} of 16 clients");
        assert_eq!(sum, 16);
    }

    fn fates(sent: u32, lost: u32, burst: u32) -> DatagramCounts {
        DatagramCounts { sent, lost, burst }
    }

    /// C62H-BOUND's join (c62h-bound-brief.md 3.1 item 3), each check alone: a send that pushed one fate more than its
    /// groups.
    #[test]
    fn c62hb_join_one_datagram_too_many() {
        assert_eq!(
            link_lost(2, true, Some(7), fates(3, 0, 0)),
            Err(
                "2 snapshot groups, but the send pushed 3 server-to-client datagram fates"
                    .to_owned()
            )
        );
    }

    #[test]
    fn c62hb_join_one_datagram_too_few() {
        assert_eq!(
            link_lost(2, true, Some(7), fates(1, 0, 0)),
            Err(
                "2 snapshot groups, but the send pushed 1 server-to-client datagram fates"
                    .to_owned()
            )
        );
    }

    #[test]
    fn c62hb_join_groups_with_a_null_server_hash() {
        assert_eq!(
            link_lost(2, true, None, fates(2, 0, 0)),
            Err("2 snapshot groups with no server hash".to_owned())
        );
    }

    #[test]
    fn c62hb_join_groups_with_no_session() {
        assert_eq!(
            link_lost(2, false, Some(7), fates(2, 0, 0)),
            Err("2 snapshot groups for a slot with no session".to_owned())
        );
    }

    #[test]
    fn c62hb_join_ok_counts_lost_and_burst() {
        assert_eq!(link_lost(3, true, Some(7), fates(3, 1, 1)), Ok(2));
        // Nothing sent: a slot with no session, and a session the room sent no snapshot datagram (a full state, or
        // nothing).
        assert_eq!(link_lost(0, false, None, fates(0, 0, 0)), Ok(0));
        assert_eq!(link_lost(0, true, None, fates(0, 0, 0)), Ok(0));
    }

    /// C62H-BOUND's short P2 run (3.1 item 10): R1_ARGS' form, 4 s measured, its header listing -17.
    const P2_SHORT: &str = "--seed 1 --profile P2 --clients 16 --warmup-s 6 --measured-s 4 --after-max-s 10 \
                            --marks PM-0001-17";

    fn c62hb_p2_short(name: &str, bite: &str) -> (Scratch, Summary) {
        let dir = scratch(name);
        let s = settings(&dir, &format!("{P2_SHORT} {bite}"));
        let o = run(&s).unwrap_or_else(|e| panic!("{name}: {e:?}"));
        println!("{name}: {}", summary_line(&s, &o));
        (dir, o)
    }

    /// The two C62H-BOUND figures as the two frozen SERIES.md rows word them, read the test's own way from a run's
    /// client and room logs (npt-lab does not depend on the judge): (unexplained, whole despite the link's loss). Over
    /// the client log's measured tick records, a (slot, tick) filed lost (`snap_incomplete` 1) whose room record gives
    /// that slot a non-null `hs.server_hash` and `snap_link_lost_count` 0 is unexplained; one with a whole
    /// `hs.decoded_hash` (not null, not the sentinel 0) whose room record gives it `snap_link_lost_count` 1 or more is
    /// whole despite the link's loss. What the rows call a gap panics here.
    fn c62hb_join(dir: &Path) -> (u64, u64) {
        let room = read(dir, "room");
        let client = read(dir, "client");
        let n = client.header.clients as usize;
        let at: BTreeMap<u64, &npt_log::TickRecord> = room.ticks.iter().map(|r| (r.t, r)).collect();
        let (mut unexplained, mut whole) = (0, 0);
        for c in client.ticks.iter().filter(|c| c.ph == Phase::Measured) {
            let filed = c.s.get("snap_incomplete").expect("gap: no snap_incomplete");
            assert_eq!(filed.len(), n, "client tick {}: snap_incomplete", c.t);
            let r = at.get(&c.t).map(|r| {
                let l =
                    r.s.get("snap_link_lost_count")
                        .expect("gap: no link record");
                let h = r.hs.get("server_hash").expect("gap: no server hash");
                assert!(l.len() == n && h.len() == n, "room tick {}: entries", c.t);
                (l, h)
            });
            for (s, &f) in filed.iter().enumerate() {
                assert!(f == 0 || f == 1, "client tick {}: snap_incomplete {f}", c.t);
                if f == 1 {
                    let (link, server) =
                        r.unwrap_or_else(|| panic!("gap: tick {} has no room record", c.t));
                    unexplained += u64::from(server[s].is_some() && link[s] == 0);
                }
                let hash =
                    c.hs.get("decoded_hash")
                        .and_then(|d| d.get(s))
                        .cloned()
                        .flatten();
                if let (Some(h), Some((link, _))) = (hash, r)
                    && u64::from_str_radix(&h, 16).expect("a hash") != 0
                    && link[s] >= 1
                {
                    whole += 1;
                }
            }
        }
        (unexplained, whole)
    }

    /// C62H-BOUND's short P2 test (3.1 item 10): every room tick record holds the link record, its measured sum is above
    /// 0, the test-side join gives 0 unexplained and 0 whole despite the link's loss, and the lab's own tally splits the
    /// measured incomplete count into E1 + E2 + N with E2 above 0 and N the join's 0. An unexplained tick here is a stop
    /// rule (the brief's section 10), never a test to adjust.
    #[test]
    fn c62hb_p2_short_every_measured_lost_tick_is_explained() {
        let (dir, o) = c62hb_p2_short("c62hb-p2", "");
        let room = read(&dir, "room");
        let mut sum = 0i64;
        for r in &room.ticks {
            let l = r.s.get("snap_link_lost_count");
            let l = l.unwrap_or_else(|| panic!("room tick {}: no snap_link_lost_count", r.t));
            assert!(
                l.len() == 16 && l.iter().all(|&x| x >= 0),
                "room tick {}: {l:?}",
                r.t
            );
            if r.ph == Phase::Measured {
                sum += l.iter().sum::<i64>();
            }
        }
        let (a, b) = c62hb_join(&dir);
        let c = &o.counts;
        println!(
            "c62hb p2: {} room tick records; measured link-lost sum {sum}; the join: unexplained {a}, whole despite \
             the link's loss {b}; E1 {}, E2 {}, N {} of measured incomplete {}",
            room.ticks.len(),
            c.lost_room_sent_nothing,
            c.lost_link,
            c.lost_unexplained,
            c.incomplete_measured
        );
        assert!(sum > 0, "the measured link-lost sum is {sum}");
        assert_eq!(
            (a, b),
            (0, 0),
            "the join: (unexplained, whole despite the link's loss)"
        );
        assert_eq!(
            c.lost_room_sent_nothing + c.lost_link + c.lost_unexplained,
            c.incomplete_measured,
            "E1 + E2 + N against the measured incomplete count"
        );
        assert!(c.lost_link > 0, "E2 is {}", c.lost_link);
        assert_eq!(
            c.lost_unexplained, a,
            "N against the join's unexplained count"
        );
        assert_eq!(c.lost_unexplained, 0, "N");
    }

    /// C62H-BOUND's first bite on the short P2 run (bites builds only): whole-as-lost files each client's first whole
    /// measured tick lost, which the room sent and the link lost none of: 16 unexplained, by the join and by the lab.
    #[cfg(feature = "bites")]
    #[test]
    fn c62hb_p2_short_bite_whole_as_lost_gives_16_unexplained() {
        let bite = "--role bite --bite PM-0001-17/whole-as-lost";
        let (dir, o) = c62hb_p2_short("c62hb-p2-whole-as-lost", bite);
        let (a, b) = c62hb_join(&dir);
        println!(
            "c62hb p2 whole-as-lost: the join {a} unexplained, {b} whole despite the link's loss; N {}",
            o.counts.lost_unexplained
        );
        assert_eq!((a, b, o.counts.lost_unexplained), (16, 0, 16));
    }

    /// C62H-BOUND's second bite (bites builds only): link-lost-overstated writes each slot's first measured tick whose
    /// datagrams all arrived as lost by the link: 16 whole despite the link's loss, none unexplained, and the summary's
    /// link loss L, from the fate log's counts, equal to a clean run's with the same arguments.
    #[cfg(feature = "bites")]
    #[test]
    fn c62hb_p2_short_bite_link_lost_overstated_gives_16_whole_despite_loss() {
        let (_clean_dir, clean) = c62hb_p2_short("c62hb-p2-overstated-clean", "");
        let bite = "--role bite --bite PM-0001-17/link-lost-overstated";
        let (dir, o) = c62hb_p2_short("c62hb-p2-overstated", bite);
        let (a, b) = c62hb_join(&dir);
        println!(
            "c62hb p2 link-lost-overstated: the join {a} unexplained, {b} whole despite the link's loss; N {}; L {} \
             (the clean run's {})",
            o.counts.lost_unexplained, o.link_lost, clean.link_lost
        );
        assert_eq!((a, b, o.counts.lost_unexplained), (0, 16, 0));
        assert_eq!(
            o.fate_log_sha256, clean.fate_log_sha256,
            "the clean run's link fates"
        );
        assert_eq!(
            o.link_lost, clean.link_lost,
            "L from the counts, never the written record"
        );
    }

    /// U4f's short controller run (u4f-brief.md 2.10's scale: 16 clients, P2_SHORT's lengths) on P3 under option
    /// `timing`, the brawl with a press every second command: made once per test process and option, its three logs
    /// (room, client, lab) read by every test that shares it (the advisor's F11); its folder removed once read.
    fn u4f_short(timing: &'static str) -> &'static [npt_log::Log] {
        static RUNS: [std::sync::OnceLock<Vec<npt_log::Log>>; 2] =
            [std::sync::OnceLock::new(), std::sync::OnceLock::new()];
        RUNS[usize::from(timing == "B")].get_or_init(|| {
            let name = format!("u4f-short-{timing}");
            let dir = scratch(&name);
            let s = settings(
                &dir,
                &format!(
                    "--seed 1 --profile P3 --clients 16 --warmup-s 6 --measured-s 4 --after-max-s 10 \
                     --controller 1 --cmd-timing {timing} --brawl-press-every 2"
                ),
            );
            let o = run(&s).unwrap_or_else(|e| panic!("{name}: {e:?}"));
            println!("{name}: {}", summary_line(&s, &o));
            crate::contract::read(&dir).expect("its logs")
        })
    }

    /// f6 (TE-14), one option: the lab builds the endpoints with the one controller setting (the header records the
    /// option as the live room runs it, C-U4F2-8, the controller's cap and LEAD-BOUND's 400 ms, and no
    /// `lead_ticks_count`: TE-20), and npt-client reads
    /// no `cmd_timing` (README :191-192 makes it the room's and the scenario's).
    fn f6(timing: &'static str) {
        let p = &u4f_short(timing)[0].header.params;
        assert_eq!(p.get("cmd_timing").and_then(|v| v.as_str()), Some(timing));
        assert_eq!(
            p.get("controller_max_lead_count").and_then(|v| v.as_u64()),
            Some(u64::from(CONTROLLER_MAX_LEAD))
        );
        assert!(!p.contains_key("lead_ticks_count"), "TE-20");
        assert_eq!(p.get("lead_bound_ms").and_then(|v| v.as_u64()), Some(400));
        for src in [
            include_str!("../../client/src/lib.rs"),
            include_str!("../../client/src/clock.rs"),
        ] {
            assert!(!src.contains("cmd_timing"));
        }
    }

    #[test]
    fn f6_option_a_takes_the_one_controller_setting_and_the_client_reads_no_cmd_timing() {
        f6("A");
    }

    #[test]
    fn f6_option_b_takes_the_one_controller_setting_and_the_client_reads_no_cmd_timing() {
        f6("B");
    }

    /// f28's -20 half (and f34's lab half), one option: the contract checker on the run's logs against SERIES.md finds
    /// no failure (every written name has its row: F3), `lead_ticks_count` is absent (TE-20), every client's pump
    /// counters (`input_limited` among them) are written, -20's series are written by stamp (`tick.s.cmd_on_time` and
    /// `tick.s.input_missing`, one entry each per eligible client, complements) with the press series, presses seen;
    /// P3's on-time share on this short run is reported and above 99 % (f36 holds the 99.9 %).
    fn f28_20(timing: &'static str) {
        let logs = u4f_short(timing);
        let c = crate::contract::check(logs, include_str!("../../../marks/SERIES.md"));
        assert_eq!(c.failures, Vec::<String>::new(), "{timing}: {:?}", c.lines);
        assert!(!logs[0].header.params.contains_key("lead_ticks_count"));
        let counts = logs[2].events.iter().filter(|e| e.ev == "client_counts");
        assert_eq!(
            counts
                .filter(|e| e.f.contains_key("pump_input_limited_count"))
                .count(),
            16
        );
        let (mut on, mut miss, mut presses, mut lost) = (0u64, 0u64, 0i64, 0i64);
        for t in logs[0].ticks.iter().filter(|r| r.ph == Phase::Measured) {
            let a = t.s.get("cmd_on_time").expect("gap: cmd_on_time");
            let b = t.s.get("input_missing").expect("gap: input_missing");
            assert_eq!(a.len(), b.len(), "tick {}", t.t);
            assert!(a.iter().zip(b).all(|(x, y)| x + y == 1));
            on += a.iter().filter(|x| **x == 1).count() as u64;
            miss += b.iter().filter(|x| **x == 1).count() as u64;
            presses += t.v.get("press_count").copied().unwrap_or(0);
            lost += t.v.get("press_lost_count").copied().unwrap_or(0);
        }
        let share = on as f64 / (on + miss).max(1) as f64;
        println!(
            "u4f f28 {timing}: on time {on} of {}, {share:.5}; presses {presses}, lost {lost}",
            on + miss
        );
        assert!(on + miss > 16 * 100 && share > 0.99, "{timing}: {share}");
        assert!(presses > 0);
        // U4F-Q3 (c): the depth per client each second, 0 under A; the ticks each command ran before its stamp, A only.
        let depth: Vec<i64> = logs[0]
            .seconds
            .iter()
            .flat_map(|s| {
                s.s.get("buffer_depth_count")
                    .expect("gap: buffer_depth_count")
            })
            .copied()
            .collect();
        let early: Vec<i64> = logs[0]
            .ticks
            .iter()
            .flat_map(|t| t.s.get("cmd_ran_early_count").expect("gap: cmd_ran_early"))
            .copied()
            .collect();
        assert!(!depth.is_empty());
        assert_eq!(depth.iter().all(|d| *d == 0), timing == "A");
        assert_eq!(early.is_empty(), timing == "B");
    }

    #[test]
    fn f28_option_a_a_controller_run_writes_20s_series_each_with_its_row() {
        f28_20("A");
    }

    #[test]
    fn f28_option_b_a_controller_run_writes_20s_series_each_with_its_row() {
        f28_20("B");
    }

    /// The clock suite's short run at f29's scale (2.10: 16 slots, 32 connects, two rounds) on P2, made once per test
    /// process and read by f29 and f28's -47 half.
    fn clock_short() -> &'static [npt_log::Log] {
        static RUN: std::sync::OnceLock<Vec<npt_log::Log>> = std::sync::OnceLock::new();
        RUN.get_or_init(|| {
            let dir = scratch("u4f-clock-short");
            let s = settings(
                &dir,
                "--seed 1 --profile P2 --clients 16 --warmup-s 1 --measured-s 8 --after-max-s 10 --suite clock \
                 --connects 32",
            );
            let o = run(&s).unwrap_or_else(|e| panic!("clock: {e:?}"));
            println!("u4f clock short: {}", summary_line(&s, &o));
            crate::contract::read(&dir).expect("its logs")
        })
    }

    /// f29 (-47; U4F-Q4 (b), NA-16): the clock suite at 16 slots and 32 connects writes one `connect_clock` event per
    /// connect (each connect number once, its error read at 2 s), every connect admitted (32 `admit` events, the run
    /// ends clean, so no admission met carry 75's bound), no event stamped (no shot, hit or kill in any second) and no
    /// client hash filed.
    #[test]
    fn f29_the_clock_suite_reads_each_connect_once_and_stamps_no_event() {
        let logs = clock_short();
        let read: Vec<&npt_log::EventRecord> = logs[2]
            .events
            .iter()
            .filter(|e| e.ev == "connect_clock")
            .collect();
        let seqs: std::collections::BTreeSet<u64> = read
            .iter()
            .filter_map(|e| e.f.get("seq").and_then(|v| v.as_u64()))
            .collect();
        let mut errs: Vec<u64> = read
            .iter()
            .filter_map(|e| e.f.get("offset_err_ns").and_then(|v| v.as_u64()))
            .collect();
        errs.sort_unstable();
        println!("u4f f29: {} reads, errors (ns) {errs:?}", read.len());
        assert_eq!((read.len(), seqs, errs.len()), (32, (0..32).collect(), 32));
        let admits = logs[0].events.iter().filter(|e| e.ev == "admit").count();
        assert_eq!(admits, 32);
        assert!(logs[0].seconds.iter().all(|s| {
            s.v.get("shots_count") == Some(&0) && s.v.get("reliable_events_count") == Some(&0)
        }));
        assert!(logs[1].ticks.is_empty(), "no client hash filed");
    }

    /// f28's -47 half: every name the clock suite's run writes has its SERIES.md row (the contract checker's (b); its
    /// one-admit rule does not hold for a suite that reconnects), the header holds `connects_count` and no `behaviour`.
    #[test]
    fn f28_the_clock_suite_writes_each_name_with_its_row() {
        let logs = clock_short();
        let c = crate::contract::check(logs, include_str!("../../../marks/SERIES.md"));
        let names: Vec<&String> = c.failures.iter().filter(|f| f.starts_with("(b)")).collect();
        assert!(names.is_empty(), "{names:?}");
        let p = &logs[0].header.params;
        assert_eq!(p.get("connects_count").and_then(|v| v.as_u64()), Some(32));
        assert!(!p.contains_key("behaviour"));
    }

    /// The never-ack suite's short run (2.10's scale: 16 clients, P0, 2 s + 8 s, U4f's controller as -22's set:
    /// NC-U4F-2), its event cap 256 so the never-acker (slot 0) meets it within seconds; made once per test process and
    /// read by f21, f14 (iii) and f28's -22 half.
    fn never_ack_short() -> &'static [npt_log::Log] {
        static RUN: std::sync::OnceLock<Vec<npt_log::Log>> = std::sync::OnceLock::new();
        RUN.get_or_init(|| {
            let dir = scratch("u4f-never-ack-short");
            let s = settings(
                &dir,
                "--seed 1 --profile P0 --controller 1 --clients 16 --warmup-s 2 --measured-s 8 --after-max-s 10 \
                 --suite never-ack --event-cap 256",
            );
            let o = run(&s).unwrap_or_else(|e| panic!("never-ack: {e:?}"));
            println!("u4f never-ack short: {}", summary_line(&s, &o));
            crate::contract::read(&dir).expect("its logs")
        })
    }

    /// f21 (-22; TE-17): the never-acker's headroom, on every room tick record, reaches 0 and never goes below; the room
    /// resolves it twice, one forced resync (`ack_starved` dropped 0) and then `Kick(overflow)` (dropped 1) within
    /// `FULL_STATE_ACK_TICKS` of it (its later datagrams reaching no room call is r9's, kept).
    #[test]
    fn f21_the_never_acker_reaches_the_cap_resyncs_once_then_is_kicked() {
        let room = &never_ack_short()[0];
        let h: Vec<i64> = room
            .ticks
            .iter()
            .map(|t| {
                t.v.get("ack_starved_headroom_count")
                    .copied()
                    .expect("gap: headroom")
            })
            .collect();
        assert_eq!(h.iter().min(), Some(&0), "{h:?}");
        let starved: Vec<(u64, Option<u64>)> = room
            .events
            .iter()
            .filter(|e| e.ev == "ack_starved")
            .map(|e| {
                (
                    e.t.unwrap_or(0),
                    e.f.get("dropped").and_then(|v| v.as_u64()),
                )
            })
            .collect();
        println!("u4f f21: ack_starved {starved:?}");
        assert_eq!(starved.len(), 2, "{starved:?}");
        assert_eq!((starved[0].1, starved[1].1), (Some(0), Some(1)));
        assert!(starved[1].0 - starved[0].0 <= u64::from(npt_sim::FULL_STATE_ACK_TICKS));
    }

    /// f14 (SA-5), the room's struct: `counts` writes every refusal field of `ClientCounters` under its own name and with
    /// its own value (each field set apart, 1 to 20, in `ROOM_COUNTS`' order).
    #[test]
    fn f14_every_refusal_field_of_client_counters_is_written() {
        let k = ClientCounters {
            late: 1,
            press_refused: 2,
            buffer_full: 3,
            rewinds_refused: 4,
            events_queued: 5,
            overflow_resyncs: 6,
            lead_refused: 7,
            not_newer: 8,
            seq_refused: 9,
            move_refused: 10,
            aim_refused: 11,
            ack_refused: 12,
            ack_refused_commands: 13,
            acks_old: 14,
            seq_outside_window: 15,
            rejoin_overlap: 16,
            commands_repeat: 17,
            resyncs_dropped: 18,
            resyncs_limited: 19,
            kicked: 20,
            ..ClientCounters::default()
        };
        let e = counts(EventRecord::new("t", Phase::After), Some(&k), None);
        let got: Vec<u64> = ROOM_COUNTS
            .iter()
            .filter_map(|(n, _)| e.f.get(*n).and_then(|v| v.as_u64()))
            .collect();
        assert_eq!((got, e.f.len()), ((1..=20).collect(), 20));
    }

    /// f14 (SA-5), the pump's struct: `counts` writes every refusal field of npt-server's `Counters` (all but the pings
    /// and pongs sent), each `pump_<field>_count` with its own value (1 to 21, in `PUMP_COUNTS`' order).
    #[test]
    fn f14_every_refusal_field_of_the_pumps_counters_is_written() {
        let d = Counters {
            too_large: 1,
            truncated: 2,
            bad_length: 3,
            unknown_type: 4,
            wrong_direction: 5,
            wrong_channel: 6,
            trailing_bytes: 7,
            unknown_field: 8,
            out_of_range: 9,
            malformed: 10,
            before_join: 11,
            input_limited: 12,
            ping_limited: 13,
            pong_unknown: 14,
            first_not_join: 15,
            joining_dropped: 16,
            stream_malformed: 17,
            join_deadline: 18,
            wrong_version: 19,
            wrong_content_hash: 20,
            join_again: 21,
            ..Counters::default()
        };
        let e = counts(EventRecord::new("t", Phase::After), None, Some(&d));
        let got: Vec<u64> = PUMP_COUNTS
            .iter()
            .filter_map(|(n, _)| e.f.get(*n).and_then(|v| v.as_u64()))
            .collect();
        assert_eq!((got, e.f.len()), ((1..=21).collect(), 21));
    }

    /// f14 (iii) (SA-5) and f28's -22 half: the kicked never-acker's counts are written once, at its kick (a
    /// `session_end` of slot 0, `kicked`, its room counts with the kick and the forced resync); every name the run
    /// writes has its SERIES.md row (the contract checker's (b)).
    #[test]
    fn f14_a_kicked_sessions_counts_are_written_at_its_kick() {
        let logs = never_ack_short();
        let ends: Vec<&npt_log::EventRecord> = logs[2]
            .events
            .iter()
            .filter(|e| e.ev == "session_end")
            .collect();
        assert_eq!(ends.len(), 1, "{ends:?}");
        let f = |k: &str| ends[0].f.get(k).cloned();
        assert_eq!(f("slot"), Some(0.into()));
        assert_eq!(f("ended"), Some("kicked".into()));
        assert_eq!(
            (f("kicked_count"), f("overflow_resyncs_count")),
            (Some(1.into()), Some(1.into()))
        );
        let c = crate::contract::check(logs, include_str!("../../../marks/SERIES.md"));
        let names: Vec<&String> = c.failures.iter().filter(|f| f.starts_with("(b)")).collect();
        assert!(names.is_empty(), "{names:?}");
    }

    /// f35 (NA-15): a session that sends `Leave`, one whose link closes and one replaced: each one's counts are kept
    /// once, at its end, with the room slot's counters as they were before the room freed the slot (its commands
    /// buffered) and its pump's (its datagrams' `Input`s decoded), never after.
    #[test]
    fn f35_each_ended_sessions_counts_are_kept_once_before_its_slot_is_freed() {
        let mut server = lab_room(4);
        let mut now = 1_000;
        let mut live = Vec::new();
        for (slot, how) in [(0u8, "left"), (1, "closed"), (2, "replaced")] {
            let (mut c, mut e, _) = joined(&mut server, slot, &mut now);
            now += 33_000_000;
            send_input(&mut c, &mut e, now);
            server.io(now + 1);
            let before = server.room.client(slot).map(|x| x.counters().clone());
            assert!(
                before.as_ref().is_some_and(|k| k.commands_buffered == 1),
                "{how}"
            );
            now += 2;
            match how {
                "left" => {
                    send_leave(&mut c, now);
                    server.io(now + 1);
                }
                "closed" => {
                    drop(c);
                    server.io(now + 1);
                }
                _ => {
                    live.push(join_only(&mut server, slot, &mut now));
                    live.push((c, e, 0));
                }
            }
            let ends: Vec<&SessionEnd> = server
                .session_ends
                .iter()
                .filter(|x| x.slot == slot)
                .collect();
            assert_eq!(ends.len(), 1, "{how}");
            assert_eq!((ends[0].ended, &ends[0].room), (how, &before), "{how}");
            assert_eq!(ends[0].pump.too_large + ends[0].pump.malformed, 0, "{how}");
            now += 1;
        }
        assert!(server.room.client(0).is_none() && server.room.client(1).is_none());
        assert!(
            server.room.client(2).is_some(),
            "the replacing session holds slot 2"
        );
    }

    /// One short run's three logs (room, client, lab) and its summary.
    pub(crate) struct Ran {
        pub(crate) logs: Vec<npt_log::Log>,
        pub(crate) sum: Summary,
    }

    /// -21's short pair of `suite` (`clamp` or `forged`) under option `timing` (2.10's scale: 16 clients, P0, 2 s + 4
    /// s, U4f's controller, the honest clients on the steady script; the clamp suite's slot 0 the cheat, the forged
    /// suite's slots 0 to 10 the 11 forgers at the judged interval, 12): the run and its twin (role twin; the clamp
    /// twin's points from the run's clamp reports, `--twin-logs`), made once per test process, suite and option, read
    /// by every test that shares it (line 6); the folders removed once read.
    pub(crate) fn pair(suite: &'static str, timing: &'static str) -> &'static [Ran; 2] {
        static PAIRS: [std::sync::OnceLock<[Ran; 2]>; 4] =
            [const { std::sync::OnceLock::new() }; 4];
        let i = 2 * usize::from(suite == "forged") + usize::from(timing == "B");
        PAIRS[i].get_or_init(|| {
            let every = if suite == "forged" {
                " --forge-every 12"
            } else {
                ""
            };
            let args = format!(
                "--seed 1 --profile P0 --clients 16 --warmup-s 2 --measured-s 4 --after-max-s 10 --controller 1 \
                 --cmd-timing {timing} --suite {suite} --behaviour steady{every}"
            );
            let one = |name: String, args: &str| {
                let dir = scratch(&name);
                let s = settings(&dir, args);
                let sum = run(&s).unwrap_or_else(|e| panic!("{name}: {e:?}"));
                println!("{name}: {}", summary_line(&s, &sum));
                let logs = crate::contract::read(&dir).expect("its logs");
                (dir, Ran { logs, sum })
            };
            let (dir, a) = one(format!("u4f2-{suite}-{timing}"), &args);
            let mut twin = format!("{args} --role twin --twin t");
            if suite == "clamp" {
                twin = format!("{twin} --twin-logs {}", dir.display());
            }
            let (_twin_dir, b) = one(format!("u4f2-{suite}-{timing}-twin"), &twin);
            [a, b]
        })
    }

    /// The names a run writes that have no SERIES.md row (the contract checker's (b)).
    fn unnamed(logs: &[npt_log::Log]) -> Vec<String> {
        let c = crate::contract::check(logs, include_str!("../../../marks/SERIES.md"));
        c.failures
            .into_iter()
            .filter(|f| f.starts_with("(b)"))
            .collect()
    }

    /// f18 (ii), line 12 of the advisor's ruling and f28's clamp half under option A: the short clamp pair (option A)
    /// counts its cheat's points beyond reach (`sent_count` above 0) with no sim refusal on them (`counted_count` 0: it
    /// holds fire, no press edge) and no rerun report; every name it writes has its row; its twin, sending the run's
    /// clamp reports' points (`--twin-logs`), hashes the same ticks to the same state hashes.
    #[test]
    fn f18_a_clamp_run_and_its_twin_give_equal_hashes_and_count_no_refusal() {
        let [run, twin] = pair("clamp", "A");
        let hashes = |r: &Ran| -> Vec<(u64, Option<String>)> {
            r.logs[0].ticks.iter().map(|t| (t.t, t.h.clone())).collect()
        };
        assert_eq!(hashes(run), hashes(twin), "the twin's state hashes");
        assert_eq!(run.sum.world_hashes, twin.sum.world_hashes, "every tick's");
        let k = run.logs[2]
            .events
            .iter()
            .find(|e| e.ev == "clamp_kind")
            .expect("clamp_kind");
        let f = |n: &str| k.f.get(n).and_then(|v| v.as_u64());
        let reports = run.logs[0]
            .events
            .iter()
            .filter(|e| {
                e.ev == "clamp_report" && e.f.get("slot").and_then(|v| v.as_u64()) == Some(0)
            })
            .count();
        println!("u4f f18: {:?}; slot 0's clamp reports {reports}", k.f);
        assert!(f("sent_count") > Some(0) && reports > 0);
        assert_eq!(
            (f("counted_count"), f("rerun_reports_count")),
            (Some(0), Some(0))
        );
        assert_eq!(unnamed(&run.logs), Vec::<String>::new());
    }

    /// f28's clamp half under option B (R3 (ii); line 6: one test per suite and option): every name the short clamp
    /// pair's run (option B) writes has its SERIES.md row, its header names the suite and option B, and its lab log one
    /// `clamp_kind` record.
    #[test]
    fn f28_option_b_the_clamp_suite_writes_each_name_with_its_row() {
        let logs = &pair("clamp", "B")[0].logs;
        assert_eq!(unnamed(logs), Vec::<String>::new());
        let p = &logs[0].header.params;
        let s = |k: &str| p.get(k).and_then(|v| v.as_str());
        assert_eq!((s("suite"), s("cmd_timing")), (Some("clamp"), Some("B")));
        let kinds = logs[2].events.iter().filter(|e| e.ev == "clamp_kind");
        assert_eq!(kinds.count(), 1);
    }

    /// f20 (2.6; U4F-Q15 (c); A1) and f28's lag-switch half: a short lag-switch run at P1 (16 clients, 2 s + 6 s, option
    /// A, a hold every 2 s) writes its bursts (each hold at least the 200 ms), runs at most `CMD_CATCHUP` commands a
    /// tick for the lag client, whose movement stays within the catch-up cap's (excess at most 0) while a burst tick
    /// moves it (FP-4: it walks the lane, never into a building), and every name it
    /// writes has its row; a respawn's placement is no movement (`moved_um` on hand-made records: alive to alive 20 cm is
    /// 200,000 µm, a death or a respawn none, so the tick is counted apart).
    #[test]
    fn f20_the_lag_switch_runs_at_most_the_cap_a_tick_and_moves_within_it() {
        let alive = |x: i32, health: i32| [x, 0, 0, health];
        assert_eq!(
            crate::logs::moved_um(alive(0, 100), alive(20, 100)),
            Some(200_000)
        );
        assert_eq!(crate::logs::moved_um(alive(0, 0), alive(5_000, 100)), None);
        assert_eq!(crate::logs::moved_um(alive(0, 100), alive(10, 0)), None);
        let dir = lag_short("A");
        let held: Vec<u64> = read(&dir, "lab")
            .events
            .iter()
            .filter(|e| e.ev == "lag_burst")
            .filter_map(|e| e.f.get("held_ns").and_then(|v| v.as_u64()))
            .collect();
        let room = read(&dir, "room");
        let most = |k: &str| room.ticks.iter().filter_map(|t| t.v.get(k).copied()).max();
        println!(
            "u4f f20: bursts held {held:?}; commands a tick at most {:?}; excess at most {:?}",
            most("lagswitch_cmds_count"),
            most("lagswitch_excess_move_um")
        );
        assert!(held.len() >= 3 && held.iter().all(|h| *h >= 200_000_000));
        let cap = npt_room::InputBuffer::CMD_CATCHUP as i64;
        assert!(most("lagswitch_cmds_count").is_some_and(|m| (1..=cap).contains(&m)));
        assert!(most("lagswitch_excess_move_um").is_some_and(|m| m <= 0));
        // FP-4: the lag client walks the lane, so a burst tick moves it: its excess above the standing value (no
        // movement less 8 ticks at full speed), and it never enters a building.
        let standing = -(crate::logs::MOVE_CM_S * 10_000 / i64::from(npt_sim::TICK_HZ) * cap);
        let v = |t: &npt_log::TickRecord, k: &str| t.v.get(k).copied().unwrap_or(0);
        let ticks = room.ticks.iter();
        let burst = ticks.filter(|t| v(t, "lagswitch_cmds_count") >= 2);
        let burst = burst.map(|t| v(t, "lagswitch_excess_move_um")).max();
        println!("u4f f20: standing {standing}; a burst tick's excess at most {burst:?}");
        assert!(burst.is_some_and(|m| m > standing), "a burst tick moves");
        let inside = room.ticks.iter().filter(|t| {
            let pairs = t.s.get("in_building_slot_area");
            pairs.is_some_and(|p| p.chunks(2).any(|x| x[0] == 0))
        });
        assert_eq!(inside.count(), 0, "the lag client in a building");
        let logs = crate::contract::read(&dir).expect("its logs");
        assert_eq!(unnamed(&logs), Vec::<String>::new());
    }

    /// The lag-switch suite's short run (f20's settings: 16 clients, P1, 2 s + 6 s, a hold every 2 s) under `timing`.
    fn lag_short(timing: &str) -> Scratch {
        let dir = scratch(&format!("u4f-lag-switch-{timing}"));
        let s = settings(
            &dir,
            &format!(
                "--seed 1 --profile P1 --clients 16 --warmup-s 2 --measured-s 6 --after-max-s 10 --controller 1 \
                 --cmd-timing {timing} --suite lag-switch --behaviour steady --lag-every-ms 2000"
            ),
        );
        run(&s).expect("the run");
        dir
    }

    /// f28's lag-switch half under option B (R3 (ii); line 6): every name the short lag-switch run (option B) writes
    /// has its SERIES.md row, its header holds the hold and spacing, and its lab log the bursts.
    #[test]
    fn f28_option_b_the_lag_switch_suite_writes_each_name_with_its_row() {
        let dir = lag_short("B");
        let logs = crate::contract::read(&dir).expect("its logs");
        assert_eq!(unnamed(&logs), Vec::<String>::new());
        let p = &logs[0].header.params;
        assert_eq!(p.get("cmd_timing").and_then(|v| v.as_str()), Some("B"));
        assert!(p.contains_key("lag_hold_ns") && p.contains_key("lag_every_ns"));
        assert!(logs[2].events.iter().any(|e| e.ev == "lag_burst"));
    }

    /// C-U4F2-1's parser half (SEC-U4F-1's rest): `--forge-every` below 2 (or not a number) is a usage error naming
    /// it, so is the option without the forged suite or the suite without it, and so is a forged suite with fewer
    /// clients than forgers; 2 is taken and lands in the header, and the run then refuses it, a usage error naming kind
    /// 7's rest (FP-3: the parser's floor stays 2). FP-8: above 1,000,000 refused (1,000,000 taken), and the suite off
    /// P0 or with `--lead` refused.
    #[test]
    fn c_u4f2_1_a_forge_interval_below_2_is_a_usage_error() {
        let dir = scratch("u4f2-forge-every");
        let at = |profile: &str, args: &str| -> Vec<String> {
            format!(
                "--run t --set t --date 2026-11-20 --commit {COMMIT} --out {} --scene SS --profile {profile} --seed 1 \
                 {args}",
                dir.display()
            )
            .split_whitespace()
            .map(str::to_owned)
            .collect()
        };
        let line = |args: &str| at("P0", args);
        for v in ["0", "1", "x", "1000001", "18446744073709551615"] {
            let e = parse(&line(&format!("--suite forged --forge-every {v}"))).expect_err(v);
            assert!(e.starts_with("--forge-every"), "{v}: {e}");
        }
        let top = parse(&line("--suite forged --forge-every 1000000")).expect("1,000,000");
        assert_eq!(top.forge_every, 1_000_000);
        let off = "--suite forged runs at --profile P0 on its forgers' fixed lead (no --lead)";
        let forged = "--suite forged --forge-every 12";
        assert_eq!(parse(&at("P1", forged)).err().as_deref(), Some(off), "P1");
        let lead = format!("{forged} --lead 5");
        assert_eq!(parse(&line(&lead)).err().as_deref(), Some(off), "--lead");
        for args in ["--suite forged", "--forge-every 2"] {
            let e = parse(&line(args)).expect_err(args);
            assert_eq!(e, "--suite forged and --forge-every <n> go together");
        }
        let e = parse(&line("--suite forged --forge-every 2 --clients 10")).expect_err("10");
        assert!(e.contains("at least 11 clients"), "{e}");
        let s = parse(&line("--suite forged --forge-every 2")).expect("2 is taken");
        let p = s
            .meta
            .params
            .get("forge_every_count")
            .and_then(|v| v.as_u64());
        assert_eq!((s.forge_every, p), (2, Some(2)));
        // FP-3, F-G: the run refuses it as a usage error, kind 7's forger needing an interval above its rest (3 at 30
        // Hz).
        let rest = npt_bots::fire_rest_at(npt_sim::TICK_HZ, s.scene.weapon().interval_us);
        let want = format!(
            "--forge-every 2: kind 7's forger refuses it (kind 7 needs an interval above its rest, {})",
            rest.expect("SS's rest")
        );
        assert_eq!(run(&s).err(), Some(Failure::Usage(want)));
    }

    /// One client's `client_counts` record in a run's lab log, by slot.
    fn counts_of(logs: &[npt_log::Log], slot: u8) -> &npt_log::EventRecord {
        logs[2]
            .events
            .iter()
            .find(|e| {
                e.ev == "client_counts"
                    && e.f.get("slot").and_then(|v| v.as_u64()) == Some(u64::from(slot))
            })
            .expect("its client_counts")
    }

    /// f22 (neg), one option: in the short forged pair's run, the catch-up forger (slot 9, kind 10) has its excess
    /// refused by the catch-up check alone (A: the per-tick cap, 8 an episode; B: the full buffer, 4), never by the
    /// bucket (`input_limited` 0), the lead bound, the not-newer rule or the past window (`late` 0).
    fn f22(timing: &'static str) {
        let c = counts_of(&pair("forged", timing)[0].logs, 9);
        let n = |k: &str| c.f.get(k).and_then(|v| v.as_u64()).expect(k);
        let per = if timing == "A" { 8 } else { 4 };
        println!("u4f2 f22 {timing}: {:?}", c.f);
        assert!(n("buffer_full_count") >= per && n("buffer_full_count") % per == 0);
        for k in [
            "pump_input_limited_count",
            "lead_refused_count",
            "not_newer_count",
            "late_count",
        ] {
            assert_eq!(n(k), 0, "{timing}: {k}");
        }
    }

    #[test]
    fn f22_option_a_the_catch_up_check_refuses_the_excess_never_the_bucket_or_the_bound() {
        f22("A");
    }

    #[test]
    fn f22_option_b_the_catch_up_check_refuses_the_excess_never_the_bucket_or_the_bound() {
        f22("B");
    }

    /// f23 (neg; catalogue :87, :99): in the short forged pair's run (option A), the wrong-channel forger's (slot 10,
    /// kind 11) messages on the control stream are refused and counted by the pump (`wrong_channel`), and its session
    /// is kept: no `session_end` of its slot, no kick, its pump's counts read live at the stop.
    #[test]
    fn f23_a_known_type_on_the_control_stream_is_refused_and_counted_and_the_session_kept() {
        let logs = &pair("forged", "A")[0].logs;
        let c = counts_of(logs, 10);
        let n = |k: &str| c.f.get(k).and_then(|v| v.as_u64()).expect(k);
        assert!(n("pump_wrong_channel_count") > 0);
        assert_eq!(
            (n("kicked_count"), n("pump_stream_malformed_count")),
            (0, 0)
        );
        let ends = logs[2].events.iter().filter(|e| e.ev == "session_end");
        assert_eq!(ends.count(), 0, "every session kept");
    }

    /// One lab-log record's field as a number.
    fn num(e: &npt_log::EventRecord, k: &str) -> u64 {
        e.f.get(k)
            .and_then(|v| v.as_u64())
            .unwrap_or_else(|| panic!("{k}"))
    }

    /// f16 (TE3-5, TE31-5; 2.12; C-U4F2-1 at the judged interval for kinds 2 and 5), one kind (`suite` forged: 1 to 11
    /// on slot kind - 1; `clamp`: slot 0) under option `timing`, on the short pair (2.10's scale, P0): the run and its
    /// twin give equal fate logs entry by entry (link, direction, flow, outcome, overtaken flag, index in its flow, send
    /// time and arrival; size and resends left out), the first difference named; equal snapshot datagram counts; the
    /// same ticks hashed to the same hashes in the room log and in every tick's state hash; every endpoint left out no
    /// command and every forged `Input` encodes; and the pair carries its kind: inside the measured window at least one
    /// forged item (kind 10: one whole episode, 8 under A, 4 under B; the clamp: one point beyond reach), none of them
    /// unrejected, no refusal record dropped, and the forger's own link's entries in both runs alike. A failure names
    /// its culprit (FP-7): a differing fate entry's link (slot and kind), or at the first differing state hash the
    /// slots whose Soldier records differ.
    fn f16(suite: &'static str, kind: u8, timing: &'static str) {
        let [run, twin] = pair(suite, timing);
        let (a, b) = (&run.sum.fates, &twin.sum.fates);
        // FP-7: a failure names the culprit, the slot and its forged kind (the shared pair carries all 11).
        let culprit = |s: u32| match s {
            _ if suite == "clamp" && s == 0 => "slot 0 (the clamp)".to_owned(),
            _ if suite == "forged" && s < u32::from(FORGERS) => {
                format!("slot {s} (kind {})", s + 1)
            }
            _ => format!("slot {s} (honest)"),
        };
        if let Some(i) = (0..a.len().min(b.len())).find(|&i| a[i] != b[i]) {
            let link = u32::from_le_bytes([a[i][0], a[i][1], a[i][2], a[i][3]]);
            panic!(
                "{suite} {kind} {timing}: fate entry {i} differs, on {}'s link: {:?} / {:?}",
                culprit(link),
                a[i],
                b[i]
            );
        }
        assert_eq!(a.len(), b.len(), "{suite} {kind} {timing}: fate entries");
        assert_eq!(run.sum.link_sent, twin.sum.link_sent, "snapshot datagrams");
        let (w, tw) = (&run.sum.world_hashes, &twin.sum.world_hashes);
        if let Some(i) = (0..w.len().min(tw.len())).find(|&i| w[i] != tw[i]) {
            let (x, y) = (&run.sum.soldiers[i], &twin.sum.soldiers[i]);
            let differ: Vec<String> = x
                .iter()
                .zip(y)
                .filter(|(p, q)| p != q)
                .map(|(p, _)| culprit(u32::from(p.0)))
                .collect();
            panic!(
                "{suite} {kind} {timing}: the first state hash that differs: {:?} / {:?}; Soldier records differ \
                 there on {differ:?}",
                w[i], tw[i]
            );
        }
        let hashes = |r: &Ran| -> Vec<(u64, Option<String>)> {
            r.logs[0].ticks.iter().map(|t| (t.t, t.h.clone())).collect()
        };
        assert_eq!(
            hashes(run),
            hashes(twin),
            "the ticks hashed and their hashes"
        );
        let slot = if suite == "clamp" { 0 } else { kind - 1 };
        let link = |f: &[[u8; 32]]| {
            f.iter()
                .filter(|e| e[..4] == u32::from(slot).to_le_bytes())
                .count()
        };
        assert!(link(a) > 0 && link(a) == link(b), "the forger's link alike");
        // FP-8 (TE-U4F2-12; 2.5): in the run every client's refusal counters are 0 but a forger's own kind's; in the
        // clamp pair every client's (FP-10, TE-U4F2-R2-4: the exemption is the forged suite's alone).
        let lab = &run.logs[2].events;
        for e in lab.iter().filter(|e| e.ev == "client_counts") {
            let s = num(e, "slot");
            let own: &[&str] = match (suite == "forged", s + 1) {
                (true, 1) => &["lead_refused_count"],
                (true, 2) => &["not_newer_count"],
                (true, 3) => &["seq_refused_count"],
                (true, 4) => &["move_refused_count"],
                (true, 5) => &["press_refused_count"],
                (true, 6) => &["aim_refused_count"],
                (true, 8) => &["ack_refused_count", "ack_refused_commands_count"],
                (true, 9) => &["pump_trailing_bytes_count", "pump_unknown_field_count"],
                (true, 10) => &["buffer_full_count"],
                (true, 11) => &["pump_wrong_channel_count"],
                _ => &[],
            };
            let traffic = ["events_queued_count", "commands_repeat_count"];
            let names = ROOM_COUNTS.iter().map(|x| x.0);
            let names = names.chain(PUMP_COUNTS.iter().map(|x| x.0));
            let names = names.filter(|k| !own.contains(k) && !traffic.contains(k));
            let nonzero: Vec<&str> = names.filter(|k| num(e, k) != 0).collect();
            assert!(
                nonzero.is_empty(),
                "slot {s}, outside its kind: {nonzero:?}"
            );
        }
        if suite == "clamp" {
            let k = run.logs[2]
                .events
                .iter()
                .find(|e| e.ev == "clamp_kind")
                .expect("clamp_kind");
            assert!(num(k, "sent_count") >= 1 && num(k, "counted_count") == 0);
            return;
        }
        for r in [run, twin] {
            for e in r.logs[2].events.iter().filter(|e| e.ev == "client_counts") {
                let k = [
                    "twin_left_out_count",
                    "forge_unencodable_count",
                    "unencodable_commands_count",
                ];
                assert_eq!(k.map(|k| num(e, k)), [0; 3], "slot {}", num(e, "slot"));
            }
        }
        let f = run.logs[2]
            .events
            .iter()
            .find(|e| e.ev == "forged_kind" && num(e, "kind") == u64::from(kind))
            .expect("its forged_kind record");
        println!("u4f2 f16 {kind} {timing}: {:?}", f.f);
        let least = match (kind, timing) {
            (10, "A") => 8,
            (10, _) => 4,
            _ => 1,
        };
        assert_eq!(num(f, "slot"), u64::from(slot));
        assert!(num(f, "measured_sent_count") >= least && num(f, "sent_count") >= least);
        let counts = [
            "unrejected_count",
            "refusals_dropped_count",
            "dead_left_out_count",
        ];
        assert_eq!(counts.map(|k| num(f, k)), [0; 3], "{counts:?}");
        if matches!(kind, 8 | 9 | 11) {
            // FP-8 (SA-U4F2-3): at P0 the counter is the forged count, no drop beside it.
            assert_eq!(num(f, "counted_count"), num(f, "sent_count"), "counted");
        }
        assert!(
            !twin.logs[2].events.iter().any(|e| e.ev == "forged_kind"),
            "a twin forges nothing"
        );
    }

    #[test]
    fn f16_option_a_kind_1_and_its_twin_match() {
        f16("forged", 1, "A");
    }

    #[test]
    fn f16_option_b_kind_1_and_its_twin_match() {
        f16("forged", 1, "B");
    }

    #[test]
    fn f16_option_a_kind_2_and_its_twin_match() {
        f16("forged", 2, "A");
    }

    #[test]
    fn f16_option_b_kind_2_and_its_twin_match() {
        f16("forged", 2, "B");
    }

    #[test]
    fn f16_option_a_kind_3_and_its_twin_match() {
        f16("forged", 3, "A");
    }

    #[test]
    fn f16_option_b_kind_3_and_its_twin_match() {
        f16("forged", 3, "B");
    }

    #[test]
    fn f16_option_a_kind_4_and_its_twin_match() {
        f16("forged", 4, "A");
    }

    #[test]
    fn f16_option_b_kind_4_and_its_twin_match() {
        f16("forged", 4, "B");
    }

    #[test]
    fn f16_option_a_kind_5_and_its_twin_match() {
        f16("forged", 5, "A");
    }

    #[test]
    fn f16_option_b_kind_5_and_its_twin_match() {
        f16("forged", 5, "B");
    }

    #[test]
    fn f16_option_a_kind_6_and_its_twin_match() {
        f16("forged", 6, "A");
    }

    #[test]
    fn f16_option_b_kind_6_and_its_twin_match() {
        f16("forged", 6, "B");
    }

    #[test]
    fn f16_option_a_kind_7_and_its_twin_match() {
        f16("forged", 7, "A");
    }

    #[test]
    fn f16_option_b_kind_7_and_its_twin_match() {
        f16("forged", 7, "B");
    }

    #[test]
    fn f16_option_a_kind_8_and_its_twin_match() {
        f16("forged", 8, "A");
    }

    #[test]
    fn f16_option_b_kind_8_and_its_twin_match() {
        f16("forged", 8, "B");
    }

    #[test]
    fn f16_option_a_kind_9_and_its_twin_match() {
        f16("forged", 9, "A");
    }

    #[test]
    fn f16_option_b_kind_9_and_its_twin_match() {
        f16("forged", 9, "B");
    }

    #[test]
    fn f16_option_a_kind_10_and_its_twin_match() {
        f16("forged", 10, "A");
    }

    #[test]
    fn f16_option_b_kind_10_and_its_twin_match() {
        f16("forged", 10, "B");
    }

    #[test]
    fn f16_option_a_kind_11_and_its_twin_match() {
        f16("forged", 11, "A");
    }

    #[test]
    fn f16_option_b_kind_11_and_its_twin_match() {
        f16("forged", 11, "B");
    }

    #[test]
    fn f16_option_a_the_clamp_and_its_twin_match() {
        f16("clamp", 0, "A");
    }

    #[test]
    fn f16_option_b_the_clamp_and_its_twin_match() {
        f16("clamp", 0, "B");
    }

    /// f37 (TE3-6, TE31-10): the lab's own episode function at the judged settings (a 10 s warm-up, 610 s measured)
    /// gives 10 back-to-back episodes of 60 s, the first from the measured start, all inside the measured window; at
    /// 5.3's (10 s, 70 s), 1.
    #[test]
    fn f37_the_fast_clocks_judged_schedule_is_ten_episodes_of_60_s() {
        let tick = |ms: u32| u64::from(npt_sim::ticks_for_ms(ms));
        let (w, e) = (tick(10_000), tick(60_000));
        let want: Vec<(u64, u64)> = (0..10).map(|i| (w + i * e, w + (i + 1) * e)).collect();
        assert_eq!(episodes(w, tick(610_000), 60 * SEC), want);
        assert!(want[9].1 <= w + tick(610_000));
        assert_eq!(episodes(w, tick(70_000), 60 * SEC), vec![(w, w + e)]);
    }

    /// The fast-clock suite's short run (2.10's scale: 16 clients, P0, 2 s + 4 s, 2 s episodes), option `timing`,
    /// slot 0 the 1.5x clock and the rest the steady script under the controller: made once per test process and
    /// option, read by f19 and f28's fast-clock half.
    fn fast_short(timing: &'static str) -> &'static [npt_log::Log] {
        static RUNS: [std::sync::OnceLock<Vec<npt_log::Log>>; 2] =
            [std::sync::OnceLock::new(), std::sync::OnceLock::new()];
        RUNS[usize::from(timing == "B")].get_or_init(|| {
            let name = format!("u4f-fast-{timing}");
            let dir = scratch(&name);
            let s = settings(
                &dir,
                &format!(
                    "--seed 1 --profile P0 --clients 16 --warmup-s 2 --measured-s 4 --after-max-s 10 \
                     --controller 1 --cmd-timing {timing} --suite fast-clock --behaviour steady \
                     --fast-clock-episode-s 2"
                ),
            );
            let o = run(&s).unwrap_or_else(|e| panic!("{name}: {e:?}"));
            println!("{name}: {}", summary_line(&s, &o));
            crate::contract::read(&dir).expect("its logs")
        })
    }

    /// One fast-clock record's field, signed.
    fn field(e: &npt_log::EventRecord, k: &str) -> i64 {
        e.f.get(k).and_then(|v| v.as_i64()).expect("a field")
    }

    /// f19 (SA-6; A1): the tally on hand-made records (a walk, a death, a respawn: the two ticks with health 0 at
    /// their start or end move nothing and are counted apart, the respawn's placement no movement), then the short
    /// fast-clock run under A: each of its two 2 s episodes has lead-bound and not-newer refusals above 0, its excess
    /// commands at most 0, its excess movement at most 0 and no copy left out (EA-U4F-1).
    #[test]
    fn f19_the_fast_clock_runs_within_the_bound_and_is_refused_by_kinds_1_and_2_each_episode() {
        let mut f = FastClock {
            episodes: vec![(1, 4)],
            bound: 12,
            ..FastClock::default()
        };
        let at = |x: i32, health: i32| Some([x, 0, 0, health]);
        assert!(f.tick(0, at(0, 100), 1, [0, 0, 0]).is_none());
        assert!(f.tick(1, at(20, 100), 1, [1, 0, 0]).is_none());
        assert!(f.tick(2, at(20, 0), 1, [1, 2, 0]).is_none());
        let e = f
            .tick(3, at(5_000, 100), 1, [3, 2, 1])
            .expect("the episode's end");
        let got = [
            "excess_cmds_count",
            "excess_move_um",
            "lead_refused_count",
            "not_newer_count",
            "left_out_count",
            "move_apart_count",
        ]
        .map(|k| field(&e, k));
        assert_eq!(got, [3 - 15, 200_000 - 13 * 200_000, 3, 2, 1, 2]);
        let ep: Vec<&npt_log::EventRecord> = fast_short("A")[2]
            .events
            .iter()
            .filter(|e| e.ev == "fast_clock")
            .collect();
        println!("u4f f19: {:?}", ep.iter().map(|e| &e.f).collect::<Vec<_>>());
        assert_eq!(ep.len(), 2);
        for e in ep {
            assert!(field(e, "lead_refused_count") > 0 && field(e, "not_newer_count") > 0);
            assert!(field(e, "excess_cmds_count") <= 0 && field(e, "excess_move_um") <= 0);
            assert_eq!(field(e, "left_out_count"), 0);
        }
    }

    /// f28's fast-clock half, one option: every name the short fast-clock run writes has its row (the contract
    /// checker's (b)), its header holds the 2 s episode, and its lab log one `fast_clock` record per episode.
    fn f28_fast(timing: &'static str) {
        let logs = fast_short(timing);
        let c = crate::contract::check(logs, include_str!("../../../marks/SERIES.md"));
        let names: Vec<&String> = c.failures.iter().filter(|f| f.starts_with("(b)")).collect();
        assert!(names.is_empty(), "{timing}: {names:?}");
        let p = &logs[0].header.params;
        assert_eq!(
            p.get("fast_clock_episode_ns").and_then(|v| v.as_u64()),
            Some(2 * SEC)
        );
        assert_eq!(
            logs[2]
                .events
                .iter()
                .filter(|e| e.ev == "fast_clock")
                .count(),
            2
        );
    }

    #[test]
    fn f28_option_a_the_fast_clock_suite_writes_each_name_with_its_row() {
        f28_fast("A");
    }

    #[test]
    fn f28_option_b_the_fast_clock_suite_writes_each_name_with_its_row() {
        f28_fast("B");
    }

    /// F2: npt-sim's two -21 check bites (`World::CHECK_BITES`, a list apart from `World::BITES`) are accepted by
    /// `--bite` and routed to the world alone (bites builds only); named here by their literal ids (TE-U4F-7), so an id
    /// dropped from that list fails by name.
    #[cfg(feature = "bites")]
    #[test]
    fn f2_the_sims_check_bites_are_routed_to_the_world_alone() {
        for id in ["PM-0001-21/no-cooldown", "PM-0001-21/refuse-not-clamp"] {
            let b = Bites::of(id).unwrap_or_else(|e| panic!("{id}: {e}"));
            assert!(b.world, "{id}");
            assert!(
                !(b.room || b.client || b.decoder || b.l1 || b.schedule || b.pump),
                "{id}"
            );
        }
    }

    /// f40 (D3-1): a short L1 P3 run's lab log holds one `reorder` event whose `permille` is above 0 and is the lower
    /// of its two directions' shares (npt-l1's fate log, as the -3 probe computes them).
    #[test]
    fn f40_a_p3_run_writes_its_measured_reorder_share_as_an_event() {
        let lab = &u4f_short("B")[2];
        let r: Vec<&npt_log::EventRecord> =
            lab.events.iter().filter(|e| e.ev == "reorder").collect();
        assert_eq!(r.len(), 1);
        let f = |k: &str| r[0].f.get(k).and_then(|v| v.as_u64()).expect("a field");
        println!(
            "u4f f40: permille {}, up {}, down {}",
            f("permille"),
            f("up_permille"),
            f("down_permille")
        );
        assert!(f("permille") > 0);
        assert_eq!(f("permille"), f("up_permille").min(f("down_permille")));
    }

    /// EA-U4F-6: the two -21 movement checks' copy of npt-sim's walk (`logs::MOVE_CM_S`) against the sim itself. TC's
    /// shooter (slot 0, in the open field 60 m west of the centre) walks west at full length, no button held, for 1 s of
    /// `World` steps: it moves exactly `MOVE_CM_S` (600 cm), so a change to the sim's walk fails here by name.
    #[test]
    fn ea_u4f_6_one_second_of_the_sims_full_walk_is_move_cm_s() {
        use npt_sim::{EntityKind, HitHistory, SimCore, TickInput};
        let mut world = World::new(&Scene::targets(), 1, 100);
        let history = HitHistory::for_ms(2_000).expect("a ring");
        let me = |w: &World| {
            let r = w
                .entity_records()
                .iter()
                .find(|r| r.kind == EntityKind::Soldier && r.index == 0);
            r.map(|r| [r.grid[0], r.grid[1], r.grid[2], r.grid[6]])
                .expect("slot 0's soldier")
        };
        let start = me(&world);
        for k in 0..npt_sim::TICK_HZ {
            let tick = world.next_tick();
            let c = npt_sim::Command {
                tick,
                seq: k as u16,
                move_xy: [-127, 0],
                view_tick: tick.wrapping_sub(1),
                ..npt_sim::Command::default()
            };
            let input = TickInput {
                tick,
                commands: vec![(c, None)],
                ..TickInput::default()
            };
            world.step(&input, &history);
        }
        let want = crate::logs::MOVE_CM_S as u64 * 10_000;
        let moved = crate::logs::moved_um(start, me(&world));
        assert_eq!(moved, Some(want), "{start:?}");
    }
}
