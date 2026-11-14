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

use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;

use npt_bots::Behaviour;
use npt_client::ClientEndpoint;
use npt_l1::{L1Net, Profile};
use npt_link::{Channel, Link, LinkState};
use npt_log::{Ended, EventRecord, Phase};
use npt_proto::{Direction, Join, Kick, Message, Quant};
use npt_room::Room;
use npt_server::{Decoder, RttSource, Schedule, due_ticks};
use npt_sim::replay::Recording;
use npt_sim::{Placement, Scene, StandIns, World};
use serde_json::Value;

use crate::hash::Failure;
use crate::logs::{Logs, Meta};
use crate::mirror::{Mirror, Records, SetStats};

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
    /// npt-lab's model bite, chosen by its unit tests only (r1): the clients connect in a hash map's order.
    #[cfg_attr(
        not(all(test, feature = "bites")),
        allow(dead_code, reason = "only the bites build's unit tests choose it")
    )]
    pub(crate) client_order: bool,
}

/// npt-lab's own bite ids (bites builds only): carry 62 (h)'s client writer (U4R-Q2 (c); its form and id the advisor's
/// U4r build ruling, point 4: "a client writer that files a tick it decoded whole as lost: null and incomplete 1"), and
/// the -30 pre-check's late-command replay client (u4r-brief.md 2.5, 7.2; marks.json:6567).
#[cfg(feature = "bites")]
pub(crate) const LAB_BITES: [&str; 2] = ["PM-0001-17/whole-as-lost", "PM-0001-30/late-command"];
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
            world: World::BITES.contains(&id),
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
            whole_as_lost: LAB_BITES[0] == id,
            late_command: LAB_BITES[1] == id,
            client_order: false,
        };
        let any = b.world
            || b.room
            || b.client
            || b.l1
            || b.schedule
            || b.pump
            || b.whole_as_lost
            || b.late_command;
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

    fn world(&self, scene: &Scene, seed: u64, first_tick: u32, arm_tick: u32) -> World {
        #[cfg(feature = "bites")]
        if self.world
            && let Some(id) = self.id.as_deref()
        {
            return World::with_bite(scene, seed, first_tick, id, arm_tick)
                .expect("Bites::of checked that this instance plants it");
        }
        let _ = arm_tick;
        World::new(scene, seed, first_tick)
    }

    fn room(&self, room: Room<World>) -> Room<World> {
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
    pub(crate) room: Room<World>,
    pub(crate) sessions: Vec<Option<Session>>,
    quant: Quant,
    rtt: RttSource,
    next_id: u64,
    /// Each admission since the last take: (slot, session, tick it runs before).
    pub(crate) admitted: Vec<(u8, u64, u32)>,
    /// Each session ended since the last take: (slot, why).
    pub(crate) ended: Vec<(u8, &'static str)>,
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
    pub(crate) fn new(room: Room<World>, quant: Quant, rtt: RttSource) -> Server {
        Server {
            room,
            sessions: Vec::new(),
            quant,
            rtt,
            next_id: 0,
            admitted: Vec::new(),
            ended: Vec::new(),
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

    fn drop_slot(&mut self, slot: u8, keep: u64, why: &'static str, now: u64) {
        for s in &mut self.sessions {
            let Some(x) = s else { continue };
            if x.slot == Some(slot) && x.id != keep {
                if why == "replaced" {
                    let mut out = Vec::new();
                    if npt_proto::encode(&Message::Kick(Kick::Replaced), &mut out).is_ok() {
                        x.link.send_reliable(Channel::Control, &out, now);
                    }
                }
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
                            match self.room.join(j) {
                                Ok((slot, r)) => {
                                    x.decoder.accept();
                                    x.slot = Some(slot);
                                    self.admitted.push((slot, x.id, tick));
                                    self.owner.insert(slot, x.id);
                                    if r {
                                        replaced = Some((slot, x.id));
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
                            self.room.receive(slot, &m);
                            if left {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                if let Some((slot, keep)) = replaced {
                    // Carry 76: the old session is told and dropped before anything else is routed.
                    self.drop_slot(slot, keep, "replaced", now);
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
            self.sessions
                .iter_mut()
                .filter(|s| s.as_ref().is_some_and(|x| x.slot == Some(slot)))
                .for_each(|s| *s = None);
            self.owner.remove(&slot);
            self.ended.push((slot, "kicked"));
        }
        Ok(())
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
    pub(crate) every_step: bool,
    pub(crate) bites: Bites,
    pub(crate) arm_tick: u32,
    pub(crate) lead: u32,
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
    pub(crate) audit: crate::mirror::Audit,
    pub(crate) counts: crate::logs::Counts,
    pub(crate) recording: Option<(usize, String)>,
    pub(crate) recording_bytes: Option<Vec<u8>>,
    pub(crate) order_digest: u64,
    /// A bite run whose after phase reached its cap with measured ticks unresolved (carry 73).
    pub(crate) after_capped: bool,
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
    let world = s.bites.world(&scene, s.seed, s.tick_start, s.arm_tick);
    let greybox = world.greybox().clone();
    let tokens = (0..scene.slots())
        .map(|slot| credentials(s.seed, slot).0)
        .collect();
    let room = Room::new(
        world,
        &scene,
        &greybox,
        tokens,
        usize::from(n),
        npt_sim::TICK_HZ,
    )
    .with_hashes(CONTENT, [0; 32])
    .with_event_start(s.event_start);
    let room = s.bites.room(room);
    let quant = s.bites.quant();
    let mut server = Server::new(room, quant, RttSource::Link);
    if s.bites.pump {
        server.decoder_bite = s.bites.id.clone();
    }
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
    let mut logs = Logs::open(
        s.logs.then_some(s.out.as_path()),
        &meta,
        s.tick_start,
        s.bites.whole_as_lost,
    )
    .map_err(fail)?;
    let mut rng = s.seed ^ 0x6c61_622d_6f66_6673;
    let order = connect_order(n, &s.bites);
    let mut clients: Vec<Client> = Vec::new();
    for &slot in &order {
        let link = net.connect(s.profile.clone(), MAX_DATAGRAM, 0);
        let endpoint = ClientEndpoint::new(join_of(s.seed, slot), npt_sim::TICK_HZ)
            .with_lead(s.lead)
            .with_starts(s.cmd_start, s.event_start);
        let endpoint = s.bites.client(endpoint);
        let behaviour = if s.behaviour == "brawl" {
            Behaviour::brawl(&scene, slot, s.seed)
        } else {
            Behaviour::script(&scene, slot, s.seed, &s.behaviour)
                .ok_or_else(|| Failure::Usage(format!("--behaviour {:?}", s.behaviour)))?
        };
        let behaviour = if s.every_step {
            behaviour.every_step()
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
        });
    }
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
    let mut inputs: Vec<i64> = Vec::new();
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
        // (1) packets due, and each new link's server end.
        while let Some((link, _)) = listener.poll_accept(now) {
            server.accept(link, now);
        }
        // (2) the server's I/O step.
        server.io(now);
        for (slot, session, tick) in std::mem::take(&mut server.admitted) {
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
        }
        for (slot, _) in std::mem::take(&mut server.ended) {
            logs.gone(slot);
        }
        // (3) the server tick, if due.
        let due = due_ticks(now, &schedule);
        schedule = due.next;
        for _ in 0..due.to_run {
            let ph = phase(k);
            let t = server.room.last_tick().wrapping_add(1);
            server.room.tick();
            if let Some(r) = recording.as_mut() {
                r.ticks.push(server.room.tick_input().clone());
            }
            let records = npt_sim::SimCore::entity_records(server.room.sim()).to_vec();
            let index = Records::new(&records);
            let mut sizes = vec![Vec::new(); usize::from(n)];
            let mut sets = vec![SetStats::default(); usize::from(n)];
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
                server.send(slot, &datagrams, &control, now).map_err(fail)?;
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
            if s.logs {
                events_stamped += logs
                    .room_tick(&server.room, t, ph, &sets, &sizes, &inputs)
                    .map_err(fail)?;
            }
            inputs.clear();
            summary
                .world_hashes
                .push((t, npt_sim::SimCore::state_hash(server.room.sim())));
            k += 1;
            if k == s.warmup_ticks {
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
            let host = now + c.offset_ns;
            let mut received = std::mem::take(&mut c.inbox);
            while let Some(r) = c.link.poll_recv(now) {
                received.push((r.channel, r.bytes, r.arrival_ns + c.offset_ns));
            }
            if let Some(w) = c.bytes_log.as_mut() {
                write_step(w, host, &received).map_err(fail)?;
            }
            c.endpoint.input(None, &received, host);
            if let Some(newest) = c.endpoint.newest_tick() {
                mirror.view(c.slot, newest, &mut view);
            } else {
                view.clear();
            }
            if c.behaviour.step(&mut c.endpoint, &[], host, &view) {
                let stamp = c.endpoint.newest_tick().map(|t| t.wrapping_add(s.lead));
                if stamp.is_some() && stamp == c.last_stamp {
                    c.duplicates += 1;
                }
                c.last_stamp = stamp;
            }
            let out = c.endpoint.step();
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
            logs.filed(c.slot, &out.hashes, &out.lost);
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
            c.next_step_ns = out.next_due_ns.saturating_sub(c.offset_ns).max(now + 1);
        }
        // Every client link's arrivals into its inbox (the socket's buffer), stamped on its host clock.
        for c in &mut clients {
            while let Some(r) = c.link.poll_recv(now) {
                c.inbox
                    .push((r.channel, r.bytes, r.arrival_ns + c.offset_ns));
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
        let mut e = EventRecord::new("client_counts", Phase::After)
            .with("slot", c.slot)
            .with("commands_count", c.behaviour.given())
            .with("steps_without_command_count", c.behaviour.skipped())
            .with("duplicate_stamps_count", c.duplicates)
            .with("late_count", k.late)
            .with("press_refused_count", k.press_refused)
            .with("buffer_full_count", k.buffer_full)
            .with("rewinds_refused_count", k.rewinds_refused)
            .with("stale_count", c.endpoint.stale())
            .with("events_queued_count", k.events_queued)
            .with("overflow_resyncs_count", k.overflow_resyncs);
        e.t = Some(logs.wide(server.room.last_tick()));
        last.push(e);
    }
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
    p.insert("cmd_timing".into(), "B".into());
    p.insert("lead_ticks_count".into(), s.lead.into());
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
    p.insert("behaviour".into(), s.behaviour.clone().into());
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
[--every-step 1] [--lead <1..8>] [--logs 0 (with --record)]";

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
    measured_end
        .checked_add(after_max_ticks)
        .ok_or("--after-max-s: the run's last tick overflows 64 bits".to_owned())?;
    let behaviour = take("behaviour").unwrap_or_else(|| "brawl".to_owned());
    if !["brawl", "presses", "clamp", "steady"].contains(&behaviour.as_str()) {
        return Err(format!(
            "--behaviour {behaviour:?}: brawl, presses, clamp or steady"
        ));
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
    let lead_override = take("lead")
        .map(|v| {
            v.parse::<u32>()
                .ok()
                .filter(|l| (1..=8).contains(l))
                .ok_or(format!("--lead {v:?}: 1 to 8"))
        })
        .transpose()?;
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
    // The last burst's tick, checked before `burst_starts` sums it (FP-3: RR3-L3), which bounds every sum there.
    if profile_name == "B1" && bursts > 0 {
        (bursts - 1)
            .checked_mul(burst_every_s)
            .and_then(|x| x.checked_add(burst_first_s))
            .and_then(|x| x.checked_mul(hz))
            .and_then(|x| x.checked_add(warmup_ticks))
            .ok_or(format!(
                "--bursts {bursts}, --burst-first-s {burst_first_s} and --burst-every-s {burst_every_s}: the last \
                 burst's tick overflows 64 bits"
            ))?;
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
        every_step,
        bites,
        arm_tick,
        lead,
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
         client hashes {} (sentinels {}); measured incomplete {}; decided lost at the stop {}; shots per measured \
         second {}; carry 49: measured client sets {}, holding every very large unit {}, projectiles per set {}; \
         audit {} owner, {} spatial checked against the radius, {} spatial not checked, {} breaches; fate log {}; \
         connect order digest {:016x}",
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
        s.lead,
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
        let world = World::new(&scene, 1, 0);
        let greybox = world.greybox().clone();
        let tokens = (0..scene.slots()).map(|s| credentials(1, s).0).collect();
        let room = Room::new(
            world,
            &scene,
            &greybox,
            tokens,
            usize::from(n),
            npt_sim::TICK_HZ,
        )
        .with_hashes(CONTENT, [0; 32]);
        Server::new(room, Quant::default(), RttSource::Link)
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

    /// FP-1's three checks of a session that ended closed on `slot`: the room's slot freed, no owner, and `(slot,
    /// "closed")` in `ended`; each that fails, by name.
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
        failed
    }

    /// Carry 76's boundary (advisor-u4r-r1, FP-1: NC-U4R-1 = SEC-U4R-1): a session whose decoder closes, (a) on a
    /// control-stream frame it refuses as malformed, or whose link closes, (b) its client end dropped, ends, and the
    /// room's slot is freed (`Room::leave`: "a closed link"); after (a) a new endpoint with that slot's token and bind is
    /// admitted, and not as a replacement. Each case's checks are collected, so a failure names its case.
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
        assert!(
            !a.ended.contains(&(0, "replaced")),
            "(a): the new endpoint is no replacement"
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
    /// option, never a wrapped length.
    #[test]
    fn fp3_a_length_whose_ticks_overflow_is_a_usage_error_naming_its_option() {
        let line = format!(
            "--run t --set t --date 2026-11-14 --commit {COMMIT} --out x --scene SS --seed 1 --profile P0 \
             --measured-s 1000000000000000000"
        );
        let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
        let e = parse(&argv).expect_err("--measured-s 10^18 x 30 overflows");
        assert!(e.starts_with("--measured-s 1000000000000000000: "), "{e}");
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
}
