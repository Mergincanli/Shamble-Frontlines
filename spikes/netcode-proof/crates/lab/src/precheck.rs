//! The -30 pre-check (u4r-brief.md 7.2; plan :772-796; the advisor's conditions 2 and 3; carry 40): a recorded npt-lab
//! test, never a judged id. Replay clients send the SS P0 recording's commands to one room over Loopback and to another
//! over L1 P0, in RQ8's order, each leg on its own simulated clock (integer ns), and `HashCompare::of` compares the two
//! rooms' per-tick World hashes.
//!
//! - **The tick base:** both legs' `World::new(scene, recording.seed, recording.first_tick)`, so each leg's first tick
//!   is the recording's first (`run_leg` refuses a tick that is not the recording's), run at SimClock instant 0 (U4b's
//!   schedule, base 0); the k-th tick is due at `due_ns(k)`.
//! - **The encoder** (R16): `ClientEndpoint::input` stamps its own ticks and sequences, so it cannot send the
//!   recording's. Each replay client runs a `ClientEndpoint` only to decode (`input(None, received, now)`) and to send
//!   its `Join` and `Pong`s; the endpoint's own `Input` (the acknowledgement only: it is given no command) is read back
//!   for its `ack_tick` and `ack_groups` and replaced by the replay client's own `Input`, encoded with npt-proto: that
//!   acknowledgement and the recorded commands (their slot, tick, sequence, presses and view time as the room ran them),
//!   newest first. So the acknowledgement names only snapshots the room sent it. No npt-client change.
//! - **Pacing** (plan silent): the command stamped T is sent at the instant tick T − L runs (step 5, after the tick), L
//!   the lead (`buffer_depth_ticks_count`, 1 to 7). On Loopback it is received at the next instant's step 2, the instant
//!   tick T − L + 1 is due; on L1 P0 at the next net instant after the send (never the same instant: l1/src/lib.rs:65-70);
//!   either way before tick T − L + 1 runs, so it waits at most L ticks, within `CMD_CATCHUP` (8). Each `Input` carries
//!   every sent command whose tick has not run (at most L, newest first, consecutive sequences), so a lost datagram
//!   would be covered by the next; at P0 and on Loopback none is lost. A command the recording repeats (option B's rerun:
//!   its sequence again) is sent once: the room reruns it as the recording shows.
//! - **Admission** does not enter the World's hashed state: only a tick's `TickInput` does (room.rs's join adds nothing
//!   to it; scenario.rs s5 replays a runner recording's `TickInput`s alone to the room's every World hash), so the two
//!   legs may admit a client at different ticks; the records give each client's admit tick per leg.
//! - **A replay client sends its first `Input` only after it decodes `JoinAccepted`** (`ClientEndpoint::slot`).
//! - **Scripts:** each recorded tick's spawns, teleports and attribute changes go to the leg's room by `Room::script`
//!   before that tick, through carry 49's `check_placements`.
//! - **The comparer:** `HashCompare::of` over each leg's `TickHash { recording, tick, hash }` list, the recording's
//!   manifest name on both, from the recording's first tick, each hash npt-sim's `SimCore::state_hash` of the room's
//!   World after the tick (the value logs.rs writes as the room log's `h`), never the mirror hash.
//! - **The record** (the advisor's U4r build ruling 2, points 1 and 2): no per-leg npt-log log; the result lines
//!   (runs.txt) carry what a header would: the build (`--commit`, required), the recording checked, each leg's room
//!   configuration, `buffer_depth_ticks_count` L beside the room's measured depth (L - 1 by definition after each
//!   client's first take), and the refused Rewinds per command.
//! - **The setup condition and the RTT rule:** per leg, `late_count` (the room's `late`), `rewinds_refused`, the commands
//!   run off their stamp, and [`rtt_rule`]; the tolerance's replacements and the cap's clips are not built at U4r (0 by
//!   construction: rewind.rs:1-2). Any of the first three above 0, a refused Rewind logged per command, a session ended
//!   on the leg (its counters cannot be read), or the RTT rule on the L1 leg, makes the pair VOID.

use std::collections::BTreeMap;

use npt_client::ClientEndpoint;
use npt_l1::{L1Net, Profile};
use npt_lab::{HashCompare, TickHash};
use npt_link::{Channel, Link};
use npt_proto::{Direction, Input, Limits, Message, Quant};
use npt_room::Room;
use npt_server::{RttSource, Schedule, due_ticks};
use npt_sim::replay::Recording;
use npt_sim::{Command, Scene, StandIns, World};

use crate::scenario::{
    Bites, CONTENT, MAX_DATAGRAM, Server, check_placements, credentials, join_of,
};

/// The pre-check's suite name (7.2).
pub(crate) const SUITE: &str = "precheck-30";

/// A pre-check leg.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Leg {
    Loopback,
    L1,
}

impl Leg {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Leg::Loopback => "loopback",
            Leg::L1 => "L1",
        }
    }
}

/// One client's records on one leg (7.2: the RTT rule's three, and the admit tick).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct ClientRecord {
    pub(crate) slot: u8,
    pub(crate) admit_tick: Option<u32>,
    /// The SimClock instant of the first pump call after which the session's `Decoder::rtt_ns(link)` is `Some`.
    pub(crate) first_sample_ns: Option<u64>,
    /// The instant of the pump call that decoded its first `Input` with a command.
    pub(crate) first_input_ns: Option<u64>,
    /// The tick its first command ran at, and that tick's run instant.
    pub(crate) first_run_tick: Option<u32>,
    pub(crate) first_run_ns: Option<u64>,
    /// That tick's due instant (base 0 + floor(k x 10^9 / TICK_HZ), k its index from the leg's first tick: 7.2 (3)),
    /// beside the run instant (the advisor's U4r build ruling 2, its Low finding).
    pub(crate) first_run_due_ns: Option<u64>,
    /// The room's measured depth at this client's first take (the take that ran its first command).
    pub(crate) first_take_depth: Option<u32>,
}

/// The RTT rule (7.2; plan :781-790; U4R-Q10 (b')): on the L1 leg, a client whose first command runs strictly before
/// its first-sample instant, or that has no first-sample instant when its first command runs, makes the leg VOID; equal
/// instants are not VOID (at one instant the pump comes before the tick). The Loopback leg's `rtt_ns` is Some(0) from
/// the start (link/src/lib.rs:203-205), so the rule binds the L1 leg only. `Some(reason)` for VOID.
pub(crate) fn rtt_rule(leg: Leg, records: &[ClientRecord]) -> Option<String> {
    if leg == Leg::Loopback {
        return None;
    }
    for r in records {
        let Some(run) = r.first_run_ns else { continue };
        match r.first_sample_ns {
            None => {
                return Some(format!(
                    "client {}: no first-sample instant on the L1 leg when its first command runs ({run} ns)",
                    r.slot
                ));
            }
            Some(sample) if run < sample => {
                return Some(format!(
                    "client {}: its first command runs at {run} ns, before its first sample at {sample} ns",
                    r.slot
                ));
            }
            Some(_) => {}
        }
    }
    None
}

/// One leg's outcome (the module text).
#[derive(Clone, Debug, Default)]
pub(crate) struct LegOutcome {
    /// The lead L this leg's replay clients paced by (its configured lead: `run_leg`'s `lead`).
    pub(crate) lead: u32,
    pub(crate) hashes: Vec<TickHash>,
    pub(crate) late: u64,
    pub(crate) rewinds_refused: u64,
    /// New commands the room ran at another tick than their stamp.
    pub(crate) off_stamp: u64,
    /// The room's measured buffer depth (`ClientSlot::buffer_depth_ticks`: "how many ticks ahead the newest buffered
    /// command was at the last take", room/src/input.rs:28-29), read after each tick, over each client's takes after
    /// its first take only (that one is the client record's): depth to the number of (client, tick) reads. Under 7.2's
    /// pacing it is L - 1 by definition (the advisor's U4r build ruling 2, point 1).
    pub(crate) depth: BTreeMap<u32, u64>,
    /// The takes after each client's first at which the measured depth is not L - 1, and the first 64 as (client,
    /// tick, measured): reported, never a VOID cause.
    pub(crate) depth_deviations: u64,
    pub(crate) deviations: Vec<(u8, u32, u32)>,
    /// The refused Rewinds per command (`ClientSlot::rewinds_refused`, read after each tick: 7.2 "per leg and per
    /// command"): the total and the first 64 as (client, tick, sequence).
    pub(crate) refused_logged: u64,
    pub(crate) refused: Vec<(u8, u32, u16)>,
    pub(crate) records: Vec<ClientRecord>,
    /// Sessions ended on this leg: (slot, why).
    pub(crate) ended: Vec<(u8, &'static str)>,
    /// Scripted placements and attribute changes put into the room.
    pub(crate) scripted: usize,
    /// The late-command bite's delayed command's tick, on its leg.
    pub(crate) delayed: Option<u32>,
    pub(crate) rtt_void: Option<String>,
}

/// One replay client (the module text).
struct ReplayClient {
    slot: u8,
    endpoint: ClientEndpoint,
    link: Box<dyn Link>,
    inbox: Vec<(Channel, Vec<u8>, u64)>,
    /// The recorded commands of this slot, in sequence order (each once).
    cmds: Vec<Command>,
    /// Commands sent so far: `cmds[..next]`; `cmds[unrun..next]` sent and not yet run.
    next: usize,
    unrun: usize,
    /// The late-command bite: the tick T whose command and every later one are held until T has run. Bites builds
    /// only, so the measured binary holds no bite code (advisor-u4r-r1, FP-6 item 3: RR3-L4).
    #[cfg(feature = "bites")]
    hold: Option<u32>,
    #[cfg(feature = "bites")]
    released: bool,
}

impl ReplayClient {
    /// The late-command bite's hold (bites builds only): the next command, stamped T or later, waits while T has not run.
    #[cfg(feature = "bites")]
    fn held(&self, ran: u32) -> bool {
        self.hold
            .is_some_and(|t| ran < t && self.cmds[self.next].tick >= t)
    }

    /// The late-command bite's release (bites builds only): once T has run, T first (it ran already: option B reran the
    /// last command there), the later ones after; (the first command to send, the delayed command's tick).
    #[cfg(feature = "bites")]
    fn release(&mut self, ran: u32, from: usize) -> (usize, Option<u32>) {
        if let Some(t) = self.hold
            && ran >= t
            && !self.released
        {
            self.released = true;
            if let Some(i) = self.cmds.iter().position(|c| c.tick >= t) {
                return (from.min(i), Some(self.cmds[i].tick));
            }
        }
        (from, None)
    }

    /// RQ8's step 5 at `now`, after tick `ran` ran: the endpoint decodes what arrived and sends its `Join` and `Pong`s;
    /// once joined, the commands due (stamped at most `ran + lead`) go in this client's own `Input`.
    fn step(&mut self, now: u64, ran: u32, lead: u32) -> Option<u32> {
        let mut got = std::mem::take(&mut self.inbox);
        while let Some(r) = self.link.poll_recv(now) {
            got.push((r.channel, r.bytes, r.arrival_ns));
        }
        self.endpoint.input(None, &got, now);
        let out = self.endpoint.step();
        for (ch, b) in &out.reliable {
            self.link.send_reliable(*ch, b, now);
        }
        let mut ack = (0u16, 0u8);
        for d in &out.datagrams {
            match npt_proto::decode(Direction::ClientToRoom, Channel::Datagram, d) {
                Ok(Message::Input(i)) => ack = (i.ack_tick, i.ack_groups),
                _ => {
                    self.link.send_unreliable(d, now);
                }
            }
        }
        self.endpoint.slot()?;
        while self.next < self.cmds.len() && self.cmds[self.next].tick <= ran.wrapping_add(lead) {
            #[cfg(feature = "bites")]
            if self.held(ran) {
                break;
            }
            self.next += 1;
        }
        while self.unrun < self.next && self.cmds[self.unrun].tick <= ran {
            self.unrun += 1;
        }
        #[cfg(feature = "bites")]
        let (from, delayed) = self.release(ran, self.unrun);
        #[cfg(not(feature = "bites"))]
        let (from, delayed) = (self.unrun, None);
        let mut commands: Vec<Command> = Vec::new();
        for c in self.cmds[from..self.next].iter().rev() {
            if commands.len() == Limits::REDUNDANCY_CAP
                || commands
                    .last()
                    .is_some_and(|n: &Command| n.seq != c.seq.wrapping_add(1))
            {
                break;
            }
            commands.push(*c);
        }
        if commands.is_empty() && ack == (0, 0) {
            return delayed;
        }
        let input = Input {
            ack_tick: ack.0,
            ack_groups: ack.1,
            commands,
        };
        let mut b = Vec::new();
        if npt_proto::encode(&Message::Input(input), &mut b).is_ok() {
            self.link.send_unreliable(&b, now);
        }
        delayed
    }
}

/// Each slot's recorded commands, each once in sequence order (a rerun, its sequence again, skipped), checked to have
/// run at their stamped tick in the recording.
pub(crate) fn commands_of(rec: &Recording) -> Result<BTreeMap<u8, Vec<Command>>, String> {
    let mut out: BTreeMap<u8, Vec<Command>> = BTreeMap::new();
    for ti in &rec.ticks {
        for (c, _) in &ti.commands {
            let list = out.entry(c.slot).or_default();
            if list.last().is_some_and(|l| l.seq == c.seq) {
                continue;
            }
            if c.tick != ti.tick {
                return Err(format!(
                    "the recording's command {} of slot {} is stamped {} but ran at {}: not a clean recording",
                    c.seq, c.slot, c.tick, ti.tick
                ));
            }
            list.push(*c);
        }
    }
    Ok(out)
}

/// Runs one leg (the module text) on the recording `rec` named `name`, at lead `lead`, with `bite` (an id) on it.
pub(crate) fn run_leg(
    rec: &Recording,
    name: &str,
    leg: Leg,
    lead: u32,
    bite: Option<&str>,
) -> Result<LegOutcome, String> {
    let scene = Scene::by_id(rec.scene)
        .ok_or(format!("no scene {}", rec.scene))?
        .with_stand_ins(StandIns::OFF);
    let cmds = commands_of(rec)?;
    let world = World::new(&scene, rec.seed, rec.first_tick);
    let greybox = world.greybox().clone();
    let tokens = (0..scene.slots())
        .map(|slot| credentials(rec.seed, slot).0)
        .collect();
    let room = Room::new(
        world,
        &scene,
        &greybox,
        tokens,
        cmds.len(),
        npt_sim::TICK_HZ,
    )
    .with_hashes(CONTENT, [0; 32]);
    let mut server = Server::new(room, Quant::default(), RttSource::Link);
    if let Some(id) = bite {
        let b = Bites::of(id)?;
        if b.pump {
            server.decoder_bite = Some(id.to_owned());
        } else if !b.late_command {
            return Err(format!("--bite {id}: not a -30 pre-check bite"));
        }
    }
    // The late-command bite's replay client (bites builds only: a measured build's `Bites::of` refused every id above).
    #[cfg(feature = "bites")]
    let late_bite = bite == Some(crate::scenario::LAB_BITES[1]);
    let mut schedule = Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP);
    let net = L1Net::new(rec.seed);
    let mut listener = net.listener();
    let p0 = Profile::by_name("P0").ok_or("no profile P0")?;
    let mut clients: Vec<ReplayClient> = Vec::new();
    for (&slot, list) in &cmds {
        let link: Box<dyn Link> = match leg {
            Leg::L1 => net.connect(p0.clone(), MAX_DATAGRAM, 0),
            Leg::Loopback => {
                let (s, c) = npt_link::Loopback::pair(MAX_DATAGRAM);
                server.accept(Box::new(s), 0);
                Box::new(c)
            }
        };
        #[cfg(feature = "bites")]
        let hold = if late_bite && clients.is_empty() {
            let t0 = rec.ticks.first().map_or(rec.first_tick, |t| t.tick);
            Some(
                list.iter()
                    .map(|c| c.tick)
                    .find(|t| t.wrapping_sub(t0) >= 60 && t.wrapping_sub(t0) < 1 << 31)
                    .ok_or("the late-command bite: slot's commands end before 60 ticks")?,
            )
        } else {
            None
        };
        clients.push(ReplayClient {
            slot,
            endpoint: ClientEndpoint::new(join_of(rec.seed, slot), npt_sim::TICK_HZ)
                .with_lead(lead),
            link,
            inbox: Vec::new(),
            cmds: list.clone(),
            next: 0,
            unrun: 0,
            #[cfg(feature = "bites")]
            hold,
            #[cfg(feature = "bites")]
            released: false,
        });
    }
    let mut o = LegOutcome {
        lead,
        ..LegOutcome::default()
    };
    let mut admitted: BTreeMap<u8, u32> = BTreeMap::new();
    let mut first_run: BTreeMap<u8, (u32, u64)> = BTreeMap::new();
    let mut first_take: BTreeMap<u8, u32> = BTreeMap::new();
    let t0 = rec.ticks.first().map_or(rec.first_tick, |t| t.tick);
    let mut last_seq: BTreeMap<u8, u16> = BTreeMap::new();
    let mut now = 0u64;
    let mut idx = 0usize;
    while idx < rec.ticks.len() {
        // (1) packets due, and each new link's server end (L1).
        while let Some((link, _)) = listener.poll_accept(now) {
            server.accept(link, now);
        }
        // (2) the server's I/O step.
        server.io(now);
        for (slot, _, tick) in std::mem::take(&mut server.admitted) {
            admitted.entry(slot).or_insert(tick);
        }
        o.ended.append(&mut server.ended);
        // (3) the server tick, if due.
        let due = due_ticks(now, &schedule);
        schedule = due.next;
        let mut ran = None;
        for _ in 0..due.to_run {
            let Some(ti) = rec.ticks.get(idx) else { break };
            check_placements(&ti.spawns, &ti.teleports)?;
            o.scripted += ti.spawns.len() + ti.teleports.len() + ti.attributes.len();
            server
                .room
                .script(&ti.spawns, &ti.teleports, &ti.attributes);
            let t = server.room.last_tick().wrapping_add(1);
            if t != ti.tick {
                return Err(format!(
                    "{} leg: the room's tick {t} is not the recording's {} (the tick base)",
                    leg.name(),
                    ti.tick
                ));
            }
            server.room.tick();
            o.hashes.push(TickHash {
                recording: name.to_owned(),
                tick: t,
                hash: npt_sim::SimCore::state_hash(server.room.sim()),
            });
            let mut first_now = Vec::new();
            for (c, _) in &server.room.tick_input().commands {
                if last_seq.insert(c.slot, c.seq) != Some(c.seq) {
                    o.off_stamp += u64::from(c.tick != t);
                    if let std::collections::btree_map::Entry::Vacant(v) = first_run.entry(c.slot) {
                        v.insert((t, now));
                        first_now.push(c.slot);
                    }
                }
            }
            for &slot in first_run.keys() {
                let Some(c) = server.room.client(slot) else {
                    continue;
                };
                let d = c.buffer_depth_ticks();
                if first_now.contains(&slot) {
                    first_take.insert(slot, d);
                    continue;
                }
                *o.depth.entry(d).or_default() += 1;
                if d != lead - 1 {
                    o.depth_deviations += 1;
                    if o.deviations.len() < 64 {
                        o.deviations.push((slot, t, d));
                    }
                }
            }
            for c in &clients {
                for &seq in server
                    .room
                    .client(c.slot)
                    .map_or(&[][..], |x| x.rewinds_refused())
                {
                    o.refused_logged += 1;
                    if o.refused.len() < 64 {
                        o.refused.push((c.slot, t, seq));
                    }
                }
            }
            for c in &clients {
                let d = server.room.take_datagrams(c.slot);
                let k = server.room.take_control(c.slot);
                server.send(c.slot, &d, &k, now)?;
            }
            o.ended.append(&mut server.ended);
            idx += 1;
            ran = Some(t);
        }
        // (5) each replay client, after a tick ran.
        if let Some(t) = ran {
            for c in &mut clients {
                if let Some(d) = c.step(now, t, lead) {
                    o.delayed = Some(d);
                }
            }
        }
        for c in &mut clients {
            while let Some(r) = c.link.poll_recv(now) {
                c.inbox.push((r.channel, r.bytes, r.arrival_ns));
            }
        }
        let mut next = schedule.next_due_ns();
        if let Some(e) = net.next_event_ns() {
            next = next.min(e.max(now + 1));
        }
        now = next.max(now + 1);
    }
    for c in &clients {
        let k = server
            .room
            .client(c.slot)
            .map(|x| x.counters().clone())
            .unwrap_or_default();
        o.late += k.late;
        o.rewinds_refused += k.rewinds_refused;
        let session = server.session(c.slot);
        let (sample, input) =
            session.map_or((None, None), |s| (s.first_sample_ns, s.first_input_ns));
        o.records.push(ClientRecord {
            slot: c.slot,
            admit_tick: admitted.get(&c.slot).copied(),
            first_sample_ns: sample,
            first_input_ns: input,
            first_run_tick: first_run.get(&c.slot).map(|r| r.0),
            first_run_ns: first_run.get(&c.slot).map(|r| r.1),
            first_run_due_ns: first_run
                .get(&c.slot)
                .map(|r| crate::scenario::due_ns(u64::from(r.0.wrapping_sub(t0)))),
            first_take_depth: first_take.get(&c.slot).copied(),
        });
    }
    o.rtt_void = rtt_rule(leg, &o.records);
    Ok(o)
}

/// The pair's result (7.2): VOID if either leg breaks the setup condition (late, rewinds refused, refused Rewinds logged
/// per command, a command off its stamp) or the RTT rule, or a session ended on it (its counters, read at the leg's end,
/// cannot be read once its slot is gone: advisor-u4r-r1, FP-4), or nothing was compared; else PASS when the comparer
/// names no differing tick, FAIL when it names one. Returns (result, reason).
pub(crate) fn verdict(a: &LegOutcome, b: &LegOutcome, cmp: &HashCompare) -> (&'static str, String) {
    let mut void = Vec::new();
    for (leg, o) in [("loopback", a), ("L1", b)] {
        if o.late > 0 {
            void.push(format!("{leg} late_count {}", o.late));
        }
        if o.rewinds_refused > 0 {
            void.push(format!("{leg} rewinds_refused {}", o.rewinds_refused));
        }
        if o.refused_logged > 0 {
            void.push(format!("{leg} refused Rewinds logged {}", o.refused_logged));
        }
        if !o.ended.is_empty() {
            void.push(format!(
                "{leg} sessions ended on the leg {:?}: their counters cannot be read",
                o.ended
            ));
        }
        if o.off_stamp > 0 {
            void.push(format!("{leg} commands off their stamp {}", o.off_stamp));
        }
        if let Some(r) = &o.rtt_void {
            void.push(format!("{leg} RTT rule: {r}"));
        }
    }
    if !void.is_empty() {
        return ("VOID", void.join("; "));
    }
    match cmp {
        HashCompare::Equal { entries } => {
            ("PASS", format!("{entries} ticks, every World hash equal"))
        }
        HashCompare::Empty => (
            "VOID",
            "the comparer compared nothing (Empty is never equal)".to_owned(),
        ),
        HashCompare::Mismatch {
            index,
            ours,
            theirs,
            compared,
            mismatching,
        } => (
            "FAIL",
            format!(
                "first differing index {index} (loopback {:?}, L1 {:?}); {mismatching} of {compared} differ",
                ours.as_ref().map(|t| (t.tick, format!("{:016x}", t.hash))),
                theirs
                    .as_ref()
                    .map(|t| (t.tick, format!("{:016x}", t.hash)))
            ),
        ),
    }
}

/// `precheck`'s usage line.
pub(crate) const USAGE: &str = "npt-lab precheck --manifest <precheck-recordings.json> --recording <file> \
--commit <sha> [--lead <1..7>] [--bite <id> --bite-leg loopback|L1]";

/// The `precheck` subcommand: the recording checked against its manifest entry (its byte length and SHA-256), both legs
/// run, and the result lines printed (the run script writes them to runs.txt). `Err` for a usage error (2) or a failure
/// to run (1).
pub(crate) fn main(argv: &[String]) -> Result<Vec<String>, (u8, String)> {
    let usage = |m: String| (2u8, m);
    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
    let mut it = argv.iter();
    while let Some(flag) = it.next() {
        let k = flag
            .strip_prefix("--")
            .ok_or(usage(format!("unexpected argument {flag:?}")))?;
        let v = it.next().ok_or(usage(format!("{flag} needs a value")))?;
        if map.insert(k, v).is_some() {
            return Err(usage(format!("{flag} given twice")));
        }
    }
    let manifest = map
        .remove("manifest")
        .ok_or(usage("--manifest is required".into()))?;
    let file = map
        .remove("recording")
        .ok_or(usage("--recording is required".into()))?;
    // No per-leg log is written (the advisor's U4r build ruling 2, point 2): the result lines carry what a header
    // would, the build first.
    let commit = map
        .remove("commit")
        .ok_or(usage("--commit <40 hex> is required".into()))?;
    let lead = match map.remove("lead") {
        None => crate::scenario::lead_for(&Profile::by_name("P0").ok_or((1, "no P0".to_owned()))?),
        Some(v) => v
            .parse::<u32>()
            .ok()
            .filter(|l| (1..=7).contains(l))
            .ok_or(usage(format!("--lead {v:?}: 1 to 7")))?,
    };
    let bite = map.remove("bite");
    let bite_leg = match (bite, map.remove("bite-leg")) {
        (None, None) => None,
        (Some(_), Some("loopback")) => Some(Leg::Loopback),
        (Some(_), Some("L1")) => Some(Leg::L1),
        _ => {
            return Err(usage(
                "--bite and --bite-leg loopback|L1 go together".into(),
            ));
        }
    };
    if let Some(k) = map.keys().next() {
        return Err(usage(format!("unknown option --{k}")));
    }
    let build = crate::machine::build(commit).map_err(usage)?;
    let m = crate::machine::container();
    let build_line = format!(
        "build: commit {}, binary sha256 {}, features {:?}, profile {}; machine: {} {}, {} vCPUs, {} MB, kernel {}",
        build.commit,
        build.binary_sha256,
        build.features,
        build.profile,
        m.arch,
        m.cpu,
        m.vcpus,
        m.mem_mb,
        m.kernel
    );
    let text = std::fs::read_to_string(manifest).map_err(|e| (1, format!("{manifest}: {e}")))?;
    let entries = crate::hash::manifest(&text).map_err(|e| (1, format!("{manifest}: {e}")))?;
    let [entry] = entries.as_slice() else {
        return Err((
            1,
            format!("{manifest}: the pre-check's manifest holds one entry"),
        ));
    };
    let bytes = std::fs::read(file).map_err(|e| (1, format!("{file}: {e}")))?;
    let sha = {
        use sha2::{Digest, Sha256};
        Sha256::digest(&bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    };
    if bytes.len() as u64 != entry.bytes || sha != entry.sha256 {
        return Err((
            1,
            format!(
                "{file}: {} bytes, sha256 {sha}: not the manifest's {} ({} bytes, {})",
                bytes.len(),
                entry.name,
                entry.bytes,
                entry.sha256
            ),
        ));
    }
    let rec = Recording::decode(&bytes).map_err(|e| (1, format!("{file}: {e}")))?;
    let mut lines = vec![
        build_line,
        format!(
            "recording {}: {} bytes, sha256 {sha}, as the manifest's; generator {}, first_tick {}, span_ms {}, ticks {}",
            entry.name,
            bytes.len(),
            rec.generator,
            rec.first_tick,
            rec.span_ms,
            rec.ticks.len()
        ),
    ];
    lines.extend(pair(&rec, &entry.name, lead, bite.zip(bite_leg)).map_err(|e| (1, e))?);
    Ok(lines)
}

/// Both legs and the result lines (the module text).
pub(crate) fn pair(
    rec: &Recording,
    name: &str,
    lead: u32,
    bite: Option<(&str, Leg)>,
) -> Result<Vec<String>, String> {
    let on = |leg: Leg| bite.filter(|b| b.1 == leg).map(|b| b.0);
    let a = run_leg(rec, name, Leg::Loopback, lead, on(Leg::Loopback))?;
    let b = run_leg(rec, name, Leg::L1, lead, on(Leg::L1))?;
    let cmp = HashCompare::of(&a.hashes, &b.hashes);
    let (result, why) = verdict(&a, &b, &cmp);
    let ring =
        npt_sim::HitHistory::for_ms(crate::scenario::ROOM_HISTORY_MS).map_or(0, |h| h.capacity());
    let mut lines = vec![format!(
        "{SUITE} {name}: {} ticks from tick {} (the tick base, both legs: World::new(scene, {}, {})); lead {lead}; \
         buffer_depth_ticks_count {lead} (the pacing lead, the value set), both legs; cmd_timing B; bite {}",
        rec.ticks.len(),
        rec.ticks.first().map_or(rec.first_tick, |t| t.tick),
        rec.seed,
        rec.first_tick,
        bite.map_or("none".to_owned(), |(id, leg)| format!(
            "{id} on the {} leg",
            leg.name()
        ))
    )];
    for (leg, o) in [(Leg::Loopback, &a), (Leg::L1, &b)] {
        let admits: Vec<u32> = o.records.iter().filter_map(|r| r.admit_tick).collect();
        lines.push(format!(
            "leg {}: room configuration: layer {}, profile {}, cmd_timing B, lead {}, tick base World::new(scene, \
             {}, {}), seed {}, ring {} ms = {ring} ticks (HitHistory::for_ms)",
            leg.name(),
            leg.name(),
            if leg == Leg::L1 { "P0" } else { "none" },
            o.lead,
            rec.seed,
            rec.first_tick,
            rec.seed,
            crate::scenario::ROOM_HISTORY_MS
        ));
        lines.push(format!(
            "leg {}: buffer_depth_ticks_count {} (the pacing lead, the value set); the room's measured depth at its \
             take (input.rs:28-29: the newest buffered command's tick minus the tick taken) is L - 1 by definition under \
             this pacing: over the takes after each client's first (depth: takes) {:?}; takes where it is not L - 1: {}, \
             the first 64 (client, tick, measured) {:?}",
            leg.name(),
            o.lead,
            o.depth,
            o.depth_deviations,
            o.deviations
        ));
        lines.push(format!(
            "leg {}: refused Rewinds per command: {}, the first 64 (client, tick, sequence) {:?}",
            leg.name(),
            o.refused_logged,
            o.refused
        ));
        lines.push(format!(
            "leg {}: clients admitted {} (admit ticks {}..{}); ended {:?}; late_count {}; rewinds_refused {}; commands \
             off their stamp {}; scripted {}; view-time replacements and clipped shots: not built, 0 by construction; \
             delayed command {}; RTT rule: {}",
            leg.name(),
            admits.len(),
            admits.iter().min().copied().unwrap_or(0),
            admits.iter().max().copied().unwrap_or(0),
            o.ended,
            o.late,
            o.rewinds_refused,
            o.off_stamp,
            o.scripted,
            o.delayed.map_or("none".to_owned(), |t| format!("tick {t}")),
            o.rtt_void.as_deref().unwrap_or(if leg == Leg::Loopback {
                "Some(0) from the start"
            } else {
                "holds"
            })
        ));
        for r in &o.records {
            lines.push(format!(
                "  {} client {}: admit tick {:?}; first sample {:?} ns; first input {:?} ns; first run tick {:?} at {:?} \
                 ns (its due instant {:?} ns); measured depth at its first take {:?}",
                leg.name(),
                r.slot,
                r.admit_tick,
                r.first_sample_ns,
                r.first_input_ns,
                r.first_run_tick,
                r.first_run_ns,
                r.first_run_due_ns,
                r.first_take_depth
            ));
        }
    }
    lines.push(format!(
        "comparer (HashCompare::of, loopback against L1): {cmp:?}"
    ));
    lines.push(format!("result: {result} ({why})"));
    Ok(lines)
}

/// Each leg's first run tick check (tests and the bites' records): the result line of `pair`'s output.
#[cfg(test)]
fn result_of(lines: &[String]) -> &str {
    lines.last().map_or("", String::as_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The SS P0 seed 4 recording's first 120 ticks (its runner entry's settings, precheck-recordings.json, with a 4 s
    /// warm-up and no measured phase: the run is the same, cut sooner), enough for every client's first commands.
    fn short_recording() -> Recording {
        let b =
            crate::scenario::record(&Scene::stress(), "P0", 4, 48, 0, 4, 0).expect("a recording");
        Recording::decode(&b).expect("it decodes")
    }

    fn rec(slot: u8, sample: Option<u64>, run: Option<u64>) -> ClientRecord {
        ClientRecord {
            slot,
            first_sample_ns: sample,
            first_run_ns: run,
            ..ClientRecord::default()
        }
    }

    /// r7 (U4R-Q10 (b')), case 1: a first-sample instant after the first command's run instant gives VOID.
    #[test]
    fn r7_a_first_sample_after_the_first_command_s_run_is_void() {
        let v = rtt_rule(
            Leg::L1,
            &[rec(0, Some(10), Some(10)), rec(1, Some(101), Some(100))],
        );
        assert!(v.is_some_and(|r| r.contains("client 1") && r.contains("before its first sample")));
    }

    /// r7, case 2: equal instants are not VOID (the pump comes before the tick).
    #[test]
    fn r7_equal_instants_are_not_void() {
        assert_eq!(rtt_rule(Leg::L1, &[rec(0, Some(100), Some(100))]), None);
        assert_eq!(rtt_rule(Leg::L1, &[rec(0, Some(99), Some(100))]), None);
    }

    /// r7, case 3: no first-sample instant on the L1 leg when the first command runs gives VOID.
    #[test]
    fn r7_no_first_sample_on_the_l1_leg_is_void() {
        let v = rtt_rule(
            Leg::L1,
            &[rec(0, Some(1), Some(5)), rec(7, None, Some(100))],
        );
        assert!(v.is_some_and(|r| r.contains("client 7") && r.contains("no first-sample")));
        assert_eq!(
            rtt_rule(Leg::L1, &[rec(7, None, None)]),
            None,
            "no command ran: nothing to judge"
        );
    }

    /// r7, case 4: the Loopback leg ("Some(0) from the start") never gives VOID, whatever its records say.
    #[test]
    fn r7_the_loopback_leg_never_gives_void() {
        assert_eq!(
            rtt_rule(
                Leg::Loopback,
                &[rec(0, None, Some(100)), rec(1, Some(200), Some(100))]
            ),
            None
        );
    }

    /// advisor-u4r-r1, FP-4 (NC-U4R-3, T-6's verdict half): `verdict` on doctored outcomes, one VOID cause each on the
    /// L1 leg (late, rewinds refused, refused Rewinds logged alone, off-stamp, the RTT rule, a session ended), each named;
    /// and on clean legs the comparer decides: Empty VOID, Equal PASS, Mismatch FAIL.
    #[test]
    fn fp4_each_void_cause_alone_gives_void_by_name_and_the_comparer_decides_a_clean_pair() {
        let clean = LegOutcome::default();
        let equal = HashCompare::Equal { entries: 3 };
        type Doctor = fn(&mut LegOutcome);
        let causes: [(&str, Doctor); 6] = [
            ("L1 late_count 1", |o| o.late = 1),
            ("L1 rewinds_refused 2", |o| o.rewinds_refused = 2),
            ("L1 refused Rewinds logged 3", |o| o.refused_logged = 3),
            ("L1 commands off their stamp 4", |o| o.off_stamp = 4),
            ("L1 RTT rule: client 5: planted", |o| {
                o.rtt_void = Some("client 5: planted".to_owned());
            }),
            (
                "L1 sessions ended on the leg [(6, \"closed\")]: their counters cannot be read",
                |o| o.ended = vec![(6, "closed")],
            ),
        ];
        for (named, doctor) in causes {
            let mut l1 = LegOutcome::default();
            doctor(&mut l1);
            let (result, why) = verdict(&clean, &l1, &equal);
            assert_eq!(
                (result, why.as_str()),
                ("VOID", named),
                "the cause {named:?}"
            );
        }
        assert_eq!(verdict(&clean, &clean, &HashCompare::Empty).0, "VOID");
        assert_eq!(
            verdict(&clean, &clean, &equal),
            ("PASS", "3 ticks, every World hash equal".to_owned())
        );
        let mismatch = HashCompare::Mismatch {
            index: 1,
            ours: None,
            theirs: None,
            compared: 3,
            mismatching: 1,
        };
        let (result, why) = verdict(&clean, &clean, &mismatch);
        assert_eq!(result, "FAIL", "{why}");
        assert!(why.starts_with("first differing index 1 "), "{why}");
    }

    /// r6, the pre-check's pacing test (7.2's pacing rule): a short clean replay of the SS P0 recording over both legs
    /// gives `late_count` 0 and `rewinds_refused` 0 on both, every command run at its stamped tick, the room's measured
    /// depth L - 1 at every take after each client's first (input.rs:28-29: the newest buffered command's tick minus
    /// the tick taken; the advisor's U4r build ruling 2, point 1), each first command run at its tick's due instant,
    /// every client admitted, the RTT rule holding, and equal per-tick World hashes (PASS).
    #[test]
    fn r6_a_clean_replay_runs_every_command_at_its_stamp_on_both_legs() {
        let r = short_recording();
        // `precheck`'s default lead, P0's (main: `lead_for`).
        let p0 = crate::scenario::lead_for(&Profile::by_name("P0").expect("P0"));
        let lines = pair(&r, "SS-P0-s4", p0, None).expect("both legs run");
        for l in lines.iter().filter(|l| !l.starts_with("  ")) {
            println!("r6: {l}");
        }
        let a = run_leg(&r, "SS-P0-s4", Leg::Loopback, p0, None).expect("loopback");
        let b = run_leg(&r, "SS-P0-s4", Leg::L1, p0, None).expect("L1");
        for (leg, o) in [("loopback", &a), ("L1", &b)] {
            assert_eq!((o.late, o.rewinds_refused, o.off_stamp), (0, 0, 0), "{leg}");
            assert_eq!(o.records.len(), 48, "{leg}");
            assert!(
                o.records
                    .iter()
                    .all(|c| c.admit_tick.is_some() && c.first_run_tick.is_some()),
                "{leg}"
            );
            println!(
                "r6 {leg}: the room's measured depth at its take {:?}; {} scripted; ticks {}",
                o.depth,
                o.scripted,
                o.hashes.len()
            );
            assert!(o.ended.is_empty(), "{leg}: {:?}", o.ended);
            assert_eq!(o.rtt_void, None, "{leg}");
            // Point 1 (4): L - 1 at every take after each client's first, but at the recording's last L - 1 ticks,
            // where the recording holds no command stamped j + L - 1, so the newest held is its last tick: there the
            // measured depth is exactly (last tick - j), one take per client and tick, and nothing else deviates. L is
            // the leg's configured lead, and the list compared is whole (at most 64 kept): the advisor's U4r build
            // ruling 3, point 5.
            let lead = o.lead;
            assert_eq!(lead, p0, "{leg}: the leg's configured lead");
            assert!(
                o.depth_deviations <= 64 && o.deviations.len() as u64 == o.depth_deviations,
                "{leg}: {} deviations, {} kept: the list compared must be whole",
                o.depth_deviations,
                o.deviations.len()
            );
            let last = r.ticks.last().map(|t| t.tick).expect("ticks");
            let slots: Vec<u8> = o.records.iter().map(|c| c.slot).collect();
            let end: Vec<(u8, u32, u32)> = (1..lead)
                .rev()
                .flat_map(|back| {
                    let j = last - (back - 1);
                    slots.iter().map(move |&s| (s, j, last - j))
                })
                .collect();
            let mut seen = o.deviations.clone();
            seen.sort_by_key(|&(s, j, _)| (j, s));
            assert_eq!(
                (o.depth_deviations, seen),
                (end.len() as u64, end),
                "{leg}: the deviations are the recording's end alone"
            );
            assert_eq!(
                o.depth.get(&(lead - 1)).copied().unwrap_or(0) + o.depth_deviations,
                o.depth.values().sum::<u64>(),
                "{leg}: {:?}",
                o.depth
            );
            let first_takes: BTreeMap<Option<u32>, usize> =
                o.records.iter().fold(BTreeMap::new(), |mut m, r| {
                    *m.entry(r.first_take_depth).or_default() += 1;
                    m
                });
            println!(
                "r6 {leg}: the measured depth at each client's first take (depth: clients) {first_takes:?}; after it \
                 {:?}; not L - 1 only at the recording's last tick {last} ({} takes)",
                o.depth, o.depth_deviations
            );
            // The Low finding: each first command ran at its tick's due instant (no catch-up at P0).
            assert!(
                o.records
                    .iter()
                    .all(|r| r.first_run_ns == r.first_run_due_ns),
                "{leg}"
            );
            assert_eq!(o.refused_logged, 0, "{leg}: {:?}", o.refused);
        }
        assert_eq!(
            HashCompare::of(&a.hashes, &b.hashes),
            HashCompare::Equal {
                entries: r.ticks.len()
            }
        );
        assert_eq!(result_of(&lines).split(' ').nth(1), Some("PASS"));
    }

    /// The late-command bite (2.5, 7.2; bites builds only): one replay client on the L1 leg holds T and every later
    /// command until T has run, then sends T first: that leg's `late_count` is exactly 1, and the pair is VOID by it alone.
    #[cfg(feature = "bites")]
    #[test]
    fn precheck_bite_late_command_makes_the_pair_void_by_late_count_alone() {
        let r = short_recording();
        let lines = pair(
            &r,
            "SS-P0-s4",
            2,
            Some(("PM-0001-30/late-command", Leg::L1)),
        )
        .expect("both legs run");
        for l in lines.iter().filter(|l| !l.starts_with("  ")) {
            println!("late-command bite: {l}");
        }
        let b = run_leg(&r, "SS-P0-s4", Leg::L1, 2, Some("PM-0001-30/late-command")).expect("L1");
        assert_eq!(b.late, 1, "exactly the one delayed command");
        assert!(b.delayed.is_some());
        assert_eq!((b.rewinds_refused, b.rtt_void.clone()), (0, None));
        // The advisor's U4r build ruling 2, point 4 (b): VOID by late_count alone, exactly.
        assert_eq!(result_of(&lines), "result: VOID (L1 late_count 1)");
        assert_eq!(b.off_stamp, 0, "the L1 leg runs no command off its stamp");
        let a = run_leg(&r, "SS-P0-s4", Leg::Loopback, 2, None).expect("loopback");
        assert_eq!(
            (a.late, a.rewinds_refused, a.off_stamp, a.rtt_void.clone()),
            (0, 0, 0, None),
            "the Loopback leg is clean"
        );
        // Point 1 (5): the bitten client's depth deviations around T are the bite's.
        println!(
            "late-command bite: L1 depth deviations {} (the bite's, around tick {:?}): {:?}",
            b.depth_deviations, b.delayed, b.deviations
        );
    }

    /// The advisor's U4r build ruling 2, point 2 (1): `--commit` is required; without it `precheck` is a usage error
    /// (exit 2) before any file is read.
    #[test]
    fn precheck_without_commit_is_a_usage_error() {
        let argv: Vec<String> = ["--manifest", "m.json", "--recording", "r.rec"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        let e = main(&argv).expect_err("no --commit");
        assert_eq!(e.0, 2, "{}", e.1);
        assert!(e.1.contains("--commit"), "{}", e.1);
    }

    /// The wallclock bite (2.5, 7.2; bites builds only): the L1 leg's pump changes each command's aim by the wall clock,
    /// so the pair is valid and the comparer names a differing tick (FAIL); a PASS would be a missed bite.
    #[cfg(feature = "bites")]
    #[test]
    fn precheck_bite_wallclock_gives_a_valid_pair_whose_comparer_names_a_tick() {
        let r = short_recording();
        let lines = pair(&r, "SS-P0-s4", 2, Some(("PM-0001-30/wallclock", Leg::L1)))
            .expect("both legs run");
        for l in lines.iter().filter(|l| !l.starts_with("  ")) {
            println!("wallclock bite: {l}");
        }
        let result = result_of(&lines);
        assert!(
            result.starts_with("result: FAIL (first differing index"),
            "{result}"
        );
    }
}
