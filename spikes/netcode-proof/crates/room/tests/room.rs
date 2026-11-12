//! U4e's room tests (u4-build-plan.md §5 U4e; u4e-brief.md 2.4): `Room<World>` driven through its own API by fake
//! clients that acknowledge what they are sent (each tick's snapshot decoded with npt-proto, as a client would), with
//! npt-sim's synthetic recording's commands. The Loopback tests with the real client endpoint are npt-server's.

use std::collections::BTreeMap;

use npt_proto::{
    Direction, EntityDelta, FullState, Input, Join, Limits, Message, Resync, Snapshot,
};
use npt_room::{Room, rewind_at};
use npt_sim::replay::Recording;
use npt_sim::{
    Command, CoverVariant, EntityKind, EntityRecord, HitHistory, HitVolumes, InputReports,
    Placement, PortalStates, Scene, SimCore, SimEvent, TickInput, World,
};

fn token(slot: u8) -> [u8; 16] {
    [slot.wrapping_add(1); 16]
}

fn join(slot: u8) -> Join {
    Join {
        token: token(slot),
        bind: [slot ^ 0x3c; 16],
        version: 1,
        content_hash: [0; 32],
    }
}

/// A room over `sim` on `scene` (its settings and bite from `prep`, before anyone joins), `n` slots joined.
fn room_with<S: SimCore>(
    sim: S,
    scene: &Scene,
    n: u8,
    prep: impl FnOnce(Room<S>) -> Room<S>,
) -> Room<S> {
    let greybox = World::new(scene, 0, 0).greybox().clone();
    let room = Room::new(
        sim,
        scene,
        &greybox,
        (0..n).map(token).collect(),
        usize::from(n),
        30,
    );
    let mut room = prep(room);
    for s in 0..n {
        assert_eq!(room.join(&join(s)).map(|j| j.0), Ok(s));
    }
    room
}

fn room_of<S: SimCore>(sim: S, scene: &Scene, n: u8) -> Room<S> {
    room_with(sim, scene, n, |r| r)
}

fn world_room(scene: &Scene, seed: u64, first: u32, n: u8) -> Room<World> {
    room_of(World::new(scene, seed, first), scene, n)
}

/// The commands of npt-sim's synthetic recording, by (slot, command tick).
fn recorded(scene: &Scene, seed: u64, first: u32, ticks: u32) -> BTreeMap<(u8, u32), Command> {
    let rec = Recording::synthetic(scene, seed, first, ticks, 2_000);
    let mut by = BTreeMap::new();
    for t in &rec.ticks {
        for (c, _) in &t.commands {
            by.entry((c.slot, c.tick)).or_insert(*c);
        }
    }
    by
}

/// What one tick sent one client: its snapshot groups (decoded), their bytes, and its control messages.
#[derive(Default)]
struct Out {
    tick: u32,
    snapshots: Vec<Snapshot>,
    bytes: Vec<Vec<u8>>,
    full: Option<FullState>,
    control: Vec<Message>,
}

/// The datagram channel, read from `Snapshot`'s row (npt-room's tests do not depend on npt-link).
fn snapshot_of(bytes: &[u8]) -> Snapshot {
    let (name, channel, _, _) = Limits::TYPES[1];
    assert_eq!(name, "Snapshot");
    match npt_proto::decode(Direction::RoomToClient, channel, bytes) {
        Ok(Message::Snapshot(s)) => s,
        other => panic!("the room sent a datagram that is no snapshot: {other:?}"),
    }
}

/// A reliable message the room encoded, decoded on the stream its type travels on (the control stream or the
/// full-state stream, each read from a row of `Limits::TYPES`: npt-room's tests name no npt-link type); `decode`
/// refuses it on the other (its length prefix and its type byte's row disagree).
fn control_of(bytes: &[u8]) -> Message {
    ["JoinAccepted", "FullState"]
        .iter()
        .filter_map(|name| Limits::TYPES.iter().find(|r| r.0 == *name))
        .find_map(|r| npt_proto::decode(Direction::RoomToClient, r.1, bytes).ok())
        .unwrap_or_else(|| panic!("a control message no stream decodes: {bytes:?}"))
}

fn collect<S: SimCore>(room: &mut Room<S>, slot: u8) -> Out {
    let mut out = Out {
        tick: room.last_tick(),
        ..Out::default()
    };
    for d in room.take_datagrams(slot) {
        out.snapshots.push(snapshot_of(&d));
        out.bytes.push(d);
    }
    for b in room.take_control(slot) {
        let m = control_of(&b);
        if let Message::FullState(f) = &m {
            out.full = Some(f.clone());
        }
        out.control.push(m);
    }
    out
}

/// A fake client's acknowledgement state: the newest tick it got and the groups of it.
#[derive(Clone, Copy, Default)]
struct Ack(Option<(u32, u8)>);

impl Ack {
    fn take(&mut self, out: &Out) {
        if out.full.is_some() {
            self.0 = Some((out.tick, 1));
        }
        for s in &out.snapshots {
            let m = match self.0 {
                Some((t, m)) if t == s.tick => m,
                _ => 0,
            };
            self.0 = Some((s.tick, m | 1 << s.group));
        }
    }
    fn input(&self, commands: Vec<Command>) -> Input {
        let (ack_tick, ack_groups) = self.0.map_or((0, 0), |(t, m)| (t as u16, m));
        Input {
            ack_tick,
            ack_groups,
            commands,
        }
    }
}

/// Runs `ticks` ticks of `room`'s `n` clients: before each tick every client whose `acks` says so acknowledges the
/// newest tick it got, with its command from `command` for the tick about to run; after each, `seen` gets each client's
/// output.
fn drive<S: SimCore>(
    room: &mut Room<S>,
    n: u8,
    ticks: u32,
    acks: impl Fn(u8, u32) -> bool,
    command: impl FnMut(u8, u32) -> Option<Command>,
    seen: impl FnMut(&Room<S>, u8, &Out),
) {
    Clients::new(n).run(room, ticks, acks, command, seen);
}

/// The fake clients' acknowledgement states, kept across runs.
struct Clients {
    ack: Vec<Ack>,
}

impl Clients {
    fn new(n: u8) -> Clients {
        Clients {
            ack: vec![Ack::default(); usize::from(n)],
        }
    }

    fn run<S: SimCore>(
        &mut self,
        room: &mut Room<S>,
        ticks: u32,
        acks: impl Fn(u8, u32) -> bool,
        mut command: impl FnMut(u8, u32) -> Option<Command>,
        mut seen: impl FnMut(&Room<S>, u8, &Out),
    ) {
        let n = self.ack.len() as u8;
        for _ in 0..ticks {
            let next = room.last_tick().wrapping_add(1);
            for s in 0..n {
                let cmds: Vec<Command> = command(s, next).into_iter().collect();
                let input = if acks(s, next) {
                    self.ack[usize::from(s)].input(cmds)
                } else {
                    Input {
                        commands: cmds,
                        ..Input::default()
                    }
                };
                room.receive(s, &Message::Input(input));
            }
            room.tick();
            for s in 0..n {
                let out = collect(room, s);
                self.ack[usize::from(s)].take(&out);
                seen(room, s, &out);
            }
        }
    }
}

/// Plan silent (built and tested here): a `Join` on a slot with a live session replaces it (catalogue :93): the room
/// reports the replacement, so the caller sends the old connection `Kick(replaced)`, and gives the slot a fresh
/// session with its own `JoinAccepted` and full state.
#[test]
fn a_join_on_a_slot_with_a_live_session_replaces_it() {
    let scene = Scene::course();
    let mut room = world_room(&scene, 1, 100, 1);
    room.tick();
    assert!(collect(&mut room, 0).full.is_some());
    assert_eq!(room.join(&join(0)), Ok((0, true)));
    let out = collect(&mut room, 0);
    assert!(matches!(out.control.as_slice(), [Message::JoinAccepted(_)]));
    assert_eq!(room.roster().live(), 1);
    room.tick();
    assert!(collect(&mut room, 0).full.is_some(), "a fresh full state");
}

/// e8 (§4.11 :401-402): `Room<World>` is `Send`, so U6 can move it to the tick thread.
#[test]
fn e8_a_room_of_world_is_send() {
    fn send<T: Send>() {}
    send::<Room<World>>();
}

/// Carry 21: commands received in two orders give one `TickInput` (the canonical order) and one state.
#[test]
fn carry_21_commands_received_in_two_orders_give_one_tick_input() {
    let scene = Scene::stress();
    let script = recorded(&scene, 3, 500, 80);
    let mut a = world_room(&scene, 3, 500, 48);
    let mut b = world_room(&scene, 3, 500, 48);
    for _ in 0..60 {
        let next = a.last_tick().wrapping_add(1);
        let inputs: Vec<(u8, Input)> = (0..48u8)
            .map(|s| {
                let now = script.get(&(s, next)).copied();
                let after = script.get(&(s, next.wrapping_add(1))).copied();
                // Each client's newest command first, then the one before: two of one slot in one datagram.
                let commands = after.into_iter().chain(now).collect();
                (
                    s,
                    Input {
                        commands,
                        ..Input::default()
                    },
                )
            })
            .collect();
        for (s, i) in &inputs {
            a.receive(*s, &Message::Input(i.clone()));
        }
        for (s, i) in inputs.iter().rev() {
            // The other room gets them in reverse, and each datagram twice.
            b.receive(*s, &Message::Input(i.clone()));
            b.receive(*s, &Message::Input(i.clone()));
        }
        a.tick();
        b.tick();
        assert_eq!(a.tick_input(), b.tick_input(), "tick {next}");
        assert_eq!(a.sim().state_hash(), b.sim().state_hash(), "tick {next}");
        for s in 0..48 {
            collect(&mut a, s);
            collect(&mut b, s);
        }
    }
    let ran: u64 = (0..48u8)
        .filter_map(|s| a.client(s))
        .map(|c| c.counters().commands_run)
        .sum();
    assert!(ran > 1_000, "{ran} commands ran");
}

fn volumes(tick: u32) -> HitVolumes {
    HitVolumes {
        tick,
        ..HitVolumes::default()
    }
}

/// Carry 22 and U4E-Q5 (a): with N the newest tick pushed, a view time at N with a fraction above 0 gives no Rewind
/// naming N + 1; the accepting edges give Rewinds equal to the command's view time with the room's cover.
#[test]
fn carry_22_rewind_at_never_names_tick_n_plus_1_and_accepts_every_held_view_time() {
    let mut h = HitHistory::for_ms(200).expect("a small ring");
    let n0 = 1_000;
    for t in n0..n0 + 20 {
        h.push(&volumes(t));
    }
    let n = n0 + 19;
    let oldest = n + 1 - h.capacity();
    let at = |view_tick: u32, view_fraction: u8| Command {
        view_tick,
        view_fraction,
        ..Command::default()
    };
    let cover = CoverVariant::Present;
    assert_eq!(rewind_at(&at(n, 1), &h, cover), None);
    assert_eq!(rewind_at(&at(n, 255), &h, cover), None);
    for (tick, fraction) in [(n, 0), (n - 1, 1), (n - 1, 255), (oldest, 0), (oldest, 7)] {
        let r = rewind_at(&at(tick, fraction), &h, cover).expect("held");
        assert_eq!((r.tick_a, r.fraction, r.cover), (tick, fraction, cover));
    }
    assert_eq!(rewind_at(&at(oldest - 1, 0), &h, cover), None);
    assert_eq!(rewind_at(&at(n + 1, 0), &h, cover), None);
}

/// Carry 22 in the room (U4E-Q5 (a)): a firing command whose view time the ring does not hold runs with no Rewind and
/// the room counts it, with its sequence for the per-shot log; a held one changes no count.
#[test]
fn carry_22_the_room_counts_each_firing_command_its_ring_cannot_rewind() {
    let scene = Scene::course();
    let mut room = world_room(&scene, 1, 100, 1);
    let fire = |tick: u32, seq: u16, view_tick: u32, view_fraction: u8| Command {
        tick,
        seq,
        buttons: Command::FIRE,
        view_tick,
        view_fraction,
        ..Command::default()
    };
    for _ in 0..5 {
        room.tick();
    }
    // The newest tick in the ring is the last tick run; the next tick's command views it.
    let mut seq = 0;
    let mut send = |room: &mut Room<World>, f: &dyn Fn(u32, u16) -> Command| -> u64 {
        let next = room.last_tick() + 1;
        seq += 1;
        room.receive(
            0,
            &Message::Input(Input {
                commands: vec![f(next, seq)],
                ..Input::default()
            }),
        );
        room.tick();
        room.client(0).map_or(0, |c| c.counters().rewinds_refused)
    };
    let held = send(&mut room, &|t, s| fire(t, s, t - 1, 0));
    assert_eq!(held, 0);
    let refused = send(&mut room, &|t, s| fire(t, s, t - 1, 9));
    assert_eq!(refused, 1);
    assert_eq!(
        room.client(0).map(|c| c.rewinds_refused().to_vec()),
        Some(vec![2])
    );
    let held_again = send(&mut room, &|t, s| fire(t, s, t - 2, 9));
    assert_eq!(held_again, 1);
}

/// e9: a second `Resync` before the full state's acknowledgement is dropped and counted (catalogue :90); once it is
/// acknowledged, a new one is limited until `npt_sim::FULL_STATE_ACK_TICKS` ticks after that full state, then taken
/// (SEC-U4E-4). The first `Resync`s come that many ticks after the join's full state.
#[test]
fn e9_a_second_resync_before_the_full_states_ack_is_dropped_and_counted() {
    let scene = Scene::course();
    let mut room = world_room(&scene, 1, 100, 1);
    let mut ack = Ack::default();
    for _ in 0..npt_sim::FULL_STATE_ACK_TICKS {
        room.receive(0, &Message::Input(ack.input(Vec::new())));
        room.tick();
        ack.take(&collect(&mut room, 0));
    }
    let c = |room: &Room<World>| {
        room.client(0)
            .map(|c| c.counters().clone())
            .unwrap_or_default()
    };
    assert_eq!((c(&room).resyncs, c(&room).resyncs_dropped), (0, 0));
    room.receive(0, &Message::Resync(Resync));
    room.receive(0, &Message::Resync(Resync));
    assert_eq!((c(&room).resyncs, c(&room).resyncs_dropped), (1, 1));
    room.tick();
    let out = collect(&mut room, 0);
    assert!(out.full.is_some() && out.snapshots.is_empty());
    room.receive(0, &Message::Resync(Resync));
    assert_eq!(c(&room).resyncs_dropped, 2, "still in flight");
    ack.take(&out);
    room.receive(0, &Message::Input(ack.input(Vec::new())));
    room.receive(0, &Message::Resync(Resync));
    let counts = |room: &Room<World>| {
        let k = c(room);
        (k.resyncs, k.resyncs_dropped, k.resyncs_limited)
    };
    assert_eq!(
        counts(&room),
        (1, 2, 1),
        "acknowledged, but sooner than the window after that full state: limited"
    );
    let f = out.tick;
    while room.last_tick().wrapping_add(1).wrapping_sub(f) < npt_sim::FULL_STATE_ACK_TICKS {
        room.tick();
        ack.take(&collect(&mut room, 0));
        room.receive(0, &Message::Input(ack.input(Vec::new())));
    }
    room.receive(0, &Message::Resync(Resync));
    assert_eq!(
        counts(&room),
        (2, 2, 1),
        "acknowledged and the window past: a new one is taken"
    );
}

/// SEC-U4E-4's room half (advisor-u4e-r1 R3; the test seat's Q-6): a client that acknowledges each full state and asks
/// again gets no `Resync` answered sooner than `npt_sim::FULL_STATE_ACK_TICKS` ticks after the last full state it was
/// sent (here its join's): each such `Resync` is dropped and counted `resyncs_limited`; the first whose answer falls
/// that many ticks after it is answered.
#[test]
fn sec_u4e_4_a_resync_sooner_than_full_state_ack_ticks_after_the_last_full_state_is_limited() {
    let scene = Scene::stress();
    let mut room = world_room(&scene, 1, 100, 1);
    let mut ack = Ack::default();
    room.tick();
    let out = collect(&mut room, 0);
    assert!(out.full.is_some(), "the join's full state");
    let joined_at = out.tick;
    ack.take(&out);
    room.receive(0, &Message::Input(ack.input(Vec::new())));
    let counters = |room: &Room<World>| {
        room.client(0)
            .map(|c| c.counters().clone())
            .unwrap_or_default()
    };
    let window = npt_sim::FULL_STATE_ACK_TICKS;
    let (mut asked, mut answered) = (0, 0);
    while room.last_tick().wrapping_add(1).wrapping_sub(joined_at) < window {
        room.receive(0, &Message::Resync(Resync));
        asked += 1;
        room.tick();
        let out = collect(&mut room, 0);
        answered += u64::from(out.full.is_some());
        ack.take(&out);
        room.receive(0, &Message::Input(ack.input(Vec::new())));
    }
    let k = counters(&room);
    println!(
        "SEC-U4E-4: {asked} Resyncs within {window} ticks of the join's full state, {answered} answered; resyncs {}, \
         limited {}, dropped {}, full states {}",
        k.resyncs, k.resyncs_limited, k.resyncs_dropped, k.full_states
    );
    assert!(asked >= 30, "{asked}");
    assert_eq!(
        (
            answered,
            k.resyncs,
            k.resyncs_limited,
            k.resyncs_dropped,
            k.full_states
        ),
        (0, 0, asked, 0, 1)
    );
    room.receive(0, &Message::Resync(Resync));
    room.tick();
    let out = collect(&mut room, 0);
    assert_eq!(out.tick.wrapping_sub(joined_at), window);
    assert!(out.full.is_some(), "answered at the window's end");
    let k = counters(&room);
    assert_eq!((k.resyncs, k.resyncs_limited, k.full_states), (1, asked, 2));
}

/// A4 (advisor-u4e-r1 R2): `tick_input()` after `tick()` is that tick's whole input ("what a recording holds"): a
/// teleport scripted for the next tick is in it, and the tick after, with nothing scripted, holds none.
#[test]
fn a4_tick_input_after_a_tick_is_that_ticks_whole_input() {
    let scene = Scene::course();
    let mut room = world_room(&scene, 1, 100, 1);
    room.tick();
    let teleport = Placement {
        slot: 0,
        position: [100, 200, 0],
    };
    room.script(&[], &[teleport], &[]);
    room.tick();
    assert_eq!(room.tick_input().tick, room.last_tick());
    assert_eq!(room.tick_input().teleports, [teleport]);
    room.tick();
    assert!(room.tick_input().teleports.is_empty(), "nothing scripted");
}

/// A `SimCore` over `World` that keeps the cover of every Rewind it is handed (A5's `with_cover` test).
struct Covers {
    world: World,
    seen: Vec<CoverVariant>,
}

impl SimCore for Covers {
    fn step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent] {
        self.seen.extend(
            input
                .commands
                .iter()
                .filter_map(|(_, r)| r.map(|r| r.cover)),
        );
        self.world.step(input, history)
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

/// A5 (advisor-u4e-r1 R2; u4-build-plan.md §4.9's two cover variants, §4.11's rule): a room built
/// `with_cover(CoverVariant::Present)` hands the sim each Rewind with `cover` Present; a room left at its default,
/// Rewound.
#[test]
fn a5_a_room_with_cover_present_hands_the_sim_each_rewind_with_cover_present() {
    let scene = Scene::course();
    let covers = |prep: fn(Room<Covers>) -> Room<Covers>| {
        let sim = Covers {
            world: World::new(&scene, 1, 100),
            seen: Vec::new(),
        };
        let mut room = room_with(sim, &scene, 1, prep);
        for _ in 0..5 {
            room.tick();
        }
        for seq in 1..=3u16 {
            let next = room.last_tick() + 1;
            let fire = Command {
                tick: next,
                seq,
                buttons: Command::FIRE,
                view_tick: next - 1,
                ..Command::default()
            };
            room.receive(
                0,
                &Message::Input(Input {
                    commands: vec![fire],
                    ..Input::default()
                }),
            );
            room.tick();
        }
        room.sim().seen.clone()
    };
    let present = covers(|r| r.with_cover(CoverVariant::Present));
    assert!(
        present.len() >= 3 && present.iter().all(|c| *c == CoverVariant::Present),
        "{present:?}"
    );
    let default = covers(|r| r);
    assert!(
        !default.is_empty() && default.iter().all(|c| *c == CoverVariant::Rewound),
        "{default:?}"
    );
}

/// TE-8 with RR-2 (advisor-u4e-r1 R5; carry 50, brief e12's "an event at age 255 is still sent, none at 256", for
/// what the room sends): a client that acknowledges its join's full state and nothing after keeps its gameplay events
/// queued and resent; the first is carried at age 255 and by no snapshot from age 256 on, when the room drops it from
/// the resend and counts it expired.
#[test]
fn carry_50_the_room_sends_an_event_at_age_255_and_none_at_256() {
    let scene = Scene::stress();
    let (seed, first) = (4, 400);
    let script = recorded(&scene, seed, first, 320);
    let mut room = room_with(World::new(&scene, seed, first), &scene, 8, |r| {
        r.with_event_cap(1 << 16)
    });
    let mut oldest: Option<(u16, u32)> = None;
    let mut ages: Vec<u32> = Vec::new();
    let mut last = 0;
    drive(
        &mut room,
        8,
        300,
        |s, next| s != 0 || next == first + 1,
        |s, t| script.get(&(s, t)).copied(),
        |_, s, out| {
            if s != 0 {
                return;
            }
            last = out.tick;
            for snap in &out.snapshots {
                for &(n, age, e) in &snap.events {
                    if !matches!(e, SimEvent::Hit { .. } | SimEvent::Kill { .. }) {
                        continue;
                    }
                    let at = out.tick.wrapping_sub(u32::from(age));
                    if oldest.is_none() {
                        oldest = Some((n, at));
                    }
                    if oldest == Some((n, at)) {
                        ages.push(u32::from(age));
                    }
                }
            }
        },
    );
    let (n, at) = oldest.expect("a gameplay event for slot 0");
    let k = room
        .client(0)
        .map(|c| c.counters().clone())
        .unwrap_or_default();
    println!(
        "carry 50: event {n} of tick {at} carried {} times, at ages {:?} to {:?}; last tick {last}; events expired {}, \
         overflow resyncs {}",
        ages.len(),
        ages.first(),
        ages.last(),
        k.events_expired,
        k.overflow_resyncs
    );
    assert!(last >= at + 260, "the run reaches past age 256: {last}");
    assert_eq!(k.overflow_resyncs, 0, "no resync cleared the queue");
    assert!(ages.contains(&255), "carried at age 255: {ages:?}");
    assert_eq!(ages.iter().max(), Some(&255), "none at 256 or older");
    assert!(k.events_expired >= 1);
}

/// e9: an acknowledgement naming a tick never sent refuses its datagram's commands (§4.14 :471).
#[test]
fn e9_an_ack_naming_a_never_sent_tick_refuses_its_datagrams_commands() {
    let scene = Scene::course();
    let mut room = world_room(&scene, 1, 100, 1);
    let mut ack = Ack::default();
    for _ in 0..4 {
        room.receive(0, &Message::Input(ack.input(Vec::new())));
        room.tick();
        ack.take(&collect(&mut room, 0));
    }
    let next = room.last_tick() + 1;
    let cmd = Command {
        tick: next,
        seq: 1,
        ..Command::default()
    };
    let ahead = Input {
        ack_tick: (next + 5) as u16,
        ack_groups: 1,
        commands: vec![cmd],
    };
    room.receive(0, &Message::Input(ahead));
    let k = room
        .client(0)
        .map(|c| c.counters().clone())
        .unwrap_or_default();
    assert_eq!(
        (k.ack_refused, k.ack_refused_commands, k.commands_buffered),
        (1, 1, 0)
    );
    room.receive(0, &Message::Input(ack.input(vec![cmd])));
    let k = room
        .client(0)
        .map(|c| c.counters().clone())
        .unwrap_or_default();
    assert_eq!((k.ack_refused, k.commands_buffered), (1, 1));
}

/// The own-state section's identity check (carry 53): every own-section record of every snapshot and full state a
/// client gets is its own slot's, with its soldier's generation, and the section it holds after each is exactly its
/// soldier, its owner-only record and its `Scene::effect_rows()` effect rows (carry 51: row i belongs to slot
/// i / effect_rows).
struct OwnCheck {
    rows: u16,
    /// Per client, its own-state view by tick.
    views: Vec<BTreeMap<u32, Vec<EntityRecord>>>,
}

fn key(r: &EntityRecord) -> (EntityKind, u16) {
    (r.kind, r.index)
}

impl OwnCheck {
    fn new(scene: &Scene, n: u8) -> OwnCheck {
        OwnCheck {
            rows: u16::from(scene.effect_rows()),
            views: vec![BTreeMap::new(); usize::from(n)],
        }
    }

    fn owns(&self, slot: u8, r: &EntityRecord) -> bool {
        match r.kind {
            EntityKind::Soldier | EntityKind::OwnState => r.index == u16::from(slot),
            EntityKind::Effect => self.rows > 0 && r.index / self.rows == u16::from(slot),
            _ => false,
        }
    }

    /// Checks one client's output at its tick against the generation of its soldier.
    fn check(&mut self, slot: u8, out: &Out, generation: u16) -> Result<(), String> {
        let t = out.tick;
        let view = if let Some(f) = &out.full {
            for r in &f.own {
                if !self.owns(slot, r) || r.generation != generation {
                    return Err(format!(
                        "tick {t}: client {slot}'s full state holds {:?} {} gen {}",
                        r.kind, r.index, r.generation
                    ));
                }
            }
            let mut v = f.own.clone();
            v.sort_by_key(key);
            v
        } else if let Some(first) = out.snapshots.first() {
            let base: Vec<EntityRecord> = if first.baseline_age == 0 {
                Vec::new()
            } else {
                let b = t.wrapping_sub(u32::from(first.baseline_age));
                self.views[usize::from(slot)]
                    .get(&b)
                    .cloned()
                    .ok_or_else(|| format!("tick {t}: client {slot}'s baseline {b} unknown"))?
            };
            let mut v = base.clone();
            for s in &out.snapshots {
                for d in &s.own {
                    let (kind, index, added) = match d {
                        EntityDelta::Add(r) => (r.kind, r.index, Some(r.generation)),
                        EntityDelta::Update { kind, index, .. }
                        | EntityDelta::Remove { kind, index } => (*kind, *index, None),
                    };
                    let probe = EntityRecord {
                        kind,
                        index,
                        generation: added.unwrap_or(generation),
                        area: 0,
                        grid: [0; EntityRecord::FIELDS],
                        raw: [0; EntityRecord::FIELDS],
                    };
                    if !self.owns(slot, &probe) || added.is_some_and(|g| g != generation) {
                        return Err(format!(
                            "tick {t}: client {slot}'s snapshot holds {kind:?} {index} generation {added:?}"
                        ));
                    }
                    let b = base.iter().find(|r| key(r) == (kind, index));
                    let at = v.binary_search_by_key(&(kind, index), key);
                    match d.apply(b) {
                        Ok(Some(r)) => match at {
                            Ok(i) => v[i] = r,
                            Err(i) => v.insert(i, r),
                        },
                        Ok(None) => {
                            if let Ok(i) = at {
                                v.remove(i);
                            }
                        }
                        Err(e) => return Err(format!("tick {t}: client {slot}: {e}")),
                    }
                }
            }
            v
        } else {
            return Err(format!("tick {t}: client {slot} got no snapshot"));
        };
        let want: Vec<(EntityKind, u16)> = [(EntityKind::Soldier, u16::from(slot))]
            .into_iter()
            .chain((0..self.rows).map(|i| (EntityKind::Effect, u16::from(slot) * self.rows + i)))
            .chain([(EntityKind::OwnState, u16::from(slot))])
            .collect();
        let got: Vec<(EntityKind, u16)> = view.iter().map(key).collect();
        if got != want || view.iter().any(|r| r.generation != generation) {
            return Err(format!(
                "tick {t}: client {slot} holds {got:?}, wants {want:?} at gen {generation}"
            ));
        }
        let views = &mut self.views[usize::from(slot)];
        views.insert(t, view);
        while views.len() > 40 {
            views.pop_first();
        }
        Ok(())
    }
}

fn generations(room: &Room<World>) -> Vec<u16> {
    let mut g = vec![0; 64];
    for r in room.sim().entity_records() {
        if r.kind == EntityKind::Soldier {
            g[usize::from(r.index)] = r.generation;
        }
    }
    g
}

/// Runs carry 53's check on an SS room of 48 clients (driven by the recording of `seed`) for at least `min` ticks and
/// until a generation change has been compared, at most `max`; the ticks run, the generation changes, and per
/// client the own-state section's bits per snapshot and the `OwnState` record's bits and changes (carry 56).
struct OwnRun {
    ticks: u32,
    changes: u32,
    own_bits: Vec<u32>,
    own_state_bits: Vec<u32>,
    own_state_changed: u64,
    snapshots: u64,
}

fn own_run(
    mut room: Room<World>,
    scene: &Scene,
    seed: u64,
    first: u32,
    min: u32,
    max: u32,
) -> Result<OwnRun, String> {
    let script = recorded(scene, seed, first, max + 10);
    let mut check = OwnCheck::new(scene, 48);
    let mut last = generations(&room);
    let mut run = OwnRun {
        ticks: 0,
        changes: 0,
        own_bits: Vec::new(),
        own_state_bits: Vec::new(),
        own_state_changed: 0,
        snapshots: 0,
    };
    let mut err = None;
    let mut clients = Clients::new(48);
    while run.ticks < max && (run.ticks < min || run.changes == 0) && err.is_none() {
        let gens = generations(&room);
        clients.run(
            &mut room,
            1,
            |_, _| true,
            |s, t| script.get(&(s, t)).copied(),
            |room, s, out| {
                if err.is_some() {
                    return;
                }
                let g = generations(room)[usize::from(s)];
                if let Err(e) = check.check(s, out, g) {
                    err = Some(e);
                }
                if let Some(c) = room.client(s)
                    && !out.snapshots.is_empty()
                {
                    let (own, own_state, changed) = c.own_section();
                    run.own_bits.push(own);
                    run.own_state_bits.push(own_state);
                    run.own_state_changed += u64::from(changed);
                    run.snapshots += 1;
                }
            },
        );
        let now = generations(&room);
        run.changes += now.iter().zip(&gens).filter(|(a, b)| a != b).count() as u32;
        last = now;
        run.ticks += 1;
    }
    let _ = last;
    match err {
        Some(e) => Err(e),
        None => Ok(run),
    }
}

fn stats(bits: &[u32]) -> (f64, u32, u32) {
    let mut v: Vec<u32> = bits.iter().map(|b| b.div_ceil(8)).collect();
    v.sort_unstable();
    let mean = v.iter().map(|b| f64::from(*b)).sum::<f64>() / v.len().max(1) as f64;
    let p99 = v
        .get((v.len() * 99 / 100).min(v.len().saturating_sub(1)))
        .copied()
        .unwrap_or(0);
    (mean, p99, v.last().copied().unwrap_or(0))
}

/// Carries 51, 53 and 56: for every client of an SS room (48 clients), on each of at least 300 ticks and through a
/// generation change, every own-section record is its own slot's with its soldier's generation and its effect rows
/// exactly its slot's (`Scene::effect_rows()`); each client gets a snapshot on every tick. The own-state section's
/// size is measured over the same run (carry 56's SS half).
/// A world of `scene` from `seed`, stepped through `warm` ticks of its synthetic recording from `first` (with its own
/// hit history), so a room built round it starts where a stress-scene soldier is about to die: a soldier dies only
/// after about 1,000 hits (a hit takes 1 of 1,000 health), about 36 s into SS for seed 1 (tick 1,287 from 200).
fn warmed(scene: &Scene, seed: u64, first: u32, warm: u32) -> World {
    let rec = Recording::synthetic(scene, seed, first, warm, 2_000);
    let mut w = World::new(scene, seed, first);
    let mut h = HitHistory::for_ms(2_000).expect("a 2 s ring");
    for t in &rec.ticks {
        w.step(t, &h);
        h.push(w.hit_volumes());
    }
    w
}

#[test]
fn carries_51_53_and_56_ss_each_client_gets_exactly_its_own_state_rows() {
    let scene = Scene::stress();
    let (seed, first, warm) = (1, 200, 850);
    let world = warmed(&scene, seed, first, warm);
    let start = world.next_tick();
    let run = own_run(room_of(world, &scene, 48), &scene, seed, first, 300, 1_000)
        .unwrap_or_else(|e| panic!("{e}"));
    let (mean, p99, max) = stats(&run.own_bits);
    let (smean, _, _) = stats(&run.own_state_bits);
    let first = start;
    println!(
        "carry 53: SS seed {seed}, {} ticks from tick {first}, 48 clients, {} generation changes; carry 56 (SS): \
         own-state section {mean:.2} B mean, {p99} B p99, {max} B max per snapshot ({} snapshots); OwnState record \
         {smean:.2} B mean, changed in {:.4} of snapshots",
        run.ticks,
        run.changes,
        run.snapshots,
        run.own_state_changed as f64 / run.snapshots.max(1) as f64
    );
    assert!(
        run.ticks >= 300 && run.changes >= 1,
        "{} ticks, {} changes",
        run.ticks,
        run.changes
    );
}

/// Carry 56's ES half (after U4d-F, merged at `<base>`): the own-state section's size in ES, 48 clients, 300 ticks.
#[test]
fn carry_56_es_the_own_state_sections_size_is_measured() {
    let scene = Scene::estimate();
    let (seed, first) = (1, 200);
    let run = own_run(
        world_room(&scene, seed, first, 48),
        &scene,
        seed,
        first,
        300,
        300,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let (mean, p99, max) = stats(&run.own_bits);
    let (smean, _, _) = stats(&run.own_state_bits);
    println!(
        "carry 56 (ES): seed {seed}, {} ticks from tick {first}, 48 clients: own-state section {mean:.2} B mean, \
         {p99} B p99, {max} B max per snapshot ({} snapshots); OwnState record {smean:.2} B mean, changed in {:.4} \
         of snapshots",
        run.ticks,
        run.snapshots,
        run.own_state_changed as f64 / run.snapshots.max(1) as f64
    );
    assert!(run.snapshots > 0);
}

/// A `SimCore` over `World` that plants one out-of-range field in one slot's `OwnState` record on one tick (carry
/// 54): the ammunition above its 16-bit field.
struct Planted {
    world: World,
    at: u32,
    slot: u16,
    events: Vec<SimEvent>,
    records: Vec<EntityRecord>,
    planted: bool,
}

impl SimCore for Planted {
    fn step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent] {
        let tick = self.world.next_tick();
        self.events = self.world.step(input, history).to_vec();
        self.planted = tick == self.at;
        if self.planted {
            self.records = self.world.entity_records().to_vec();
            for r in &mut self.records {
                if r.kind == EntityKind::OwnState && r.index == self.slot {
                    r.grid[3] = 70_000;
                }
            }
        }
        &self.events
    }
    fn state_hash(&self) -> u64 {
        self.world.state_hash()
    }
    fn entity_records(&self) -> &[EntityRecord] {
        if self.planted {
            &self.records
        } else {
            self.world.entity_records()
        }
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

/// Carry 54 (and carry 61 (a)): one out-of-range record planted for one client on one tick: that client's snapshot is
/// skipped and counted, with no server hash, no panic; every other client's snapshot that tick is byte-equal to the
/// run without the plant; the client's next tick is coded afresh.
#[test]
fn carry_54_an_out_of_range_record_skips_that_clients_snapshot_alone() {
    let scene = Scene::stress();
    let (seed, first, n) = (2, 300, 8u8);
    let (at, k) = (first + 12, 5u8);
    let script = recorded(&scene, seed, first, 40);
    let planted = Planted {
        world: World::new(&scene, seed, first),
        at,
        slot: u16::from(k),
        events: Vec::new(),
        records: Vec::new(),
        planted: false,
    };
    let mut a = room_of(planted, &scene, n);
    let mut b = world_room(&scene, seed, first, n);
    let mut outs: BTreeMap<(char, u8, u32), Vec<Vec<u8>>> = BTreeMap::new();
    let mut hashes: BTreeMap<(char, u8, u32), Option<u64>> = BTreeMap::new();
    drive(
        &mut a,
        n,
        20,
        |_, _| true,
        |s, t| script.get(&(s, t)).copied(),
        |room, s, out| {
            outs.insert(('a', s, out.tick), out.bytes.clone());
            hashes.insert(
                ('a', s, out.tick),
                room.client(s).and_then(|c| c.server_hash()),
            );
        },
    );
    drive(
        &mut b,
        n,
        20,
        |_, _| true,
        |s, t| script.get(&(s, t)).copied(),
        |room, s, out| {
            outs.insert(('b', s, out.tick), out.bytes.clone());
            hashes.insert(
                ('b', s, out.tick),
                room.client(s).and_then(|c| c.server_hash()),
            );
        },
    );
    let ka = a
        .client(k)
        .map(|c| c.counters().clone())
        .unwrap_or_default();
    assert_eq!(ka.refused_out_of_range, 1);
    assert!(
        outs[&('a', k, at)].is_empty(),
        "client {k}'s snapshot at the planted tick is skipped"
    );
    assert_eq!(
        hashes[&('a', k, at)],
        None,
        "carry 61 (a): no server hash for a snapshot not sent"
    );
    assert!(
        !outs[&('a', k, at + 1)].is_empty(),
        "the next tick codes afresh"
    );
    assert!(hashes[&('a', k, at + 1)].is_some());
    for s in (0..n).filter(|s| *s != k) {
        assert_eq!(
            outs[&('a', s, at)],
            outs[&('b', s, at)],
            "client {s} at the planted tick"
        );
        assert!(!outs[&('a', s, at)].is_empty());
        assert_eq!(
            a.client(s).map(|c| c.counters().refused_out_of_range),
            Some(0)
        );
    }
    assert_eq!(a.sim().state_hash(), b.sim().state_hash());
}

/// Carry 57, first part (and -22's clean half): a never-acking client overflows its queue: one resync, counted with
/// its tick; its headroom falls to 0 and never below; the cap is the room's data, its provisional mark exposed.
/// Then a second overflow before the full state's acknowledgement sends `Kick(overflow)` and drops the client. Returns
/// the overflow resyncs' ticks, the lowest headroom, and the tick of the first `Kick(overflow)`.
fn never_acker(
    room: &mut Room<World>,
    script: &BTreeMap<(u8, u32), Command>,
    ticks: u32,
) -> (Vec<u32>, i64, Option<u32>) {
    let mut resyncs = Vec::new();
    let mut min_headroom = i64::MAX;
    let mut kicked = None;
    drive(
        room,
        8,
        ticks,
        |s, _| s != 0,
        |s, t| script.get(&(s, t)).copied(),
        |room, s, out| {
            if s != 0 {
                return;
            }
            if let Some(c) = room.client(0) {
                if c.overflow_resync() {
                    resyncs.push(out.tick);
                }
                min_headroom = min_headroom.min(c.headroom());
            }
            if kicked.is_none()
                && out
                    .control
                    .iter()
                    .any(|m| matches!(m, Message::Kick(npt_proto::Kick::Overflow)))
            {
                kicked = Some(out.tick);
            }
        },
    );
    (resyncs, min_headroom, kicked)
}

#[test]
fn carry_57_a_never_acker_overflows_once_then_a_second_overflow_kicks() {
    let scene = Scene::stress();
    let script = recorded(&scene, 4, 400, 400);
    assert_eq!(
        world_room(&scene, 4, 400, 1).event_cap(),
        (1_024, true),
        "the provisional cap"
    );
    let mut room = room_with(World::new(&scene, 4, 400), &scene, 8, |r| {
        r.with_event_cap(48)
    });
    assert_eq!(room.event_cap(), (48, false));
    let (resyncs, min_headroom, kicked) = never_acker(&mut room, &script, 300);
    let c = room.client(0).expect("slot 0");
    assert_eq!(resyncs.len(), 1, "one resync: {resyncs:?}");
    assert_eq!(c.counters().overflow_resyncs, 1);
    assert_eq!(
        min_headroom, 0,
        "the headroom reaches 0 and never goes below"
    );
    // TE-6 (advisor-u4e-r1 R6): the second overflow kicks, before the full state's timeout would (its first tick past
    // the window), so the timeout cannot stand in for this edge.
    let timeout = resyncs[0] + npt_sim::FULL_STATE_ACK_TICKS + 1;
    println!(
        "carry 57: first overflow resync at {}, Kick(overflow) at {kicked:?}, the timeout's tick {timeout}",
        resyncs[0]
    );
    assert!(kicked.is_some_and(|k| k < timeout), "{kicked:?}");
    assert!(c.kicked() && c.counters().kicked == 1);
    assert!(!room.roster().is_live(0), "the client is dropped");
    assert!(
        room.client(1).is_some_and(|c| !c.kicked()),
        "an acking client stays"
    );
}

/// Carry 57's other edge: a forced full state not acknowledged within `npt_sim::FULL_STATE_ACK_TICKS` kicks; one
/// acknowledged on the last tick of that window does not.
#[test]
fn carry_57_a_forced_full_state_not_acked_within_full_state_ack_ticks_kicks() {
    let scene = Scene::stress();
    let script = recorded(&scene, 4, 400, 600);
    let window = npt_sim::FULL_STATE_ACK_TICKS;
    // A cap of 256: the queue refills far slower than the window, so the window decides.
    let run = |ack_at: Option<u32>| {
        let mut room = room_with(World::new(&scene, 4, 400), &scene, 8, |r| {
            r.with_event_cap(256)
        });
        let mut forced = None;
        let mut kicked_at = None;
        drive(
            &mut room,
            8,
            500,
            |s, next| s != 0 || Some(next) == ack_at,
            |s, t| script.get(&(s, t)).copied(),
            |room, s, out| {
                if s != 0 || kicked_at.is_some() {
                    return;
                }
                if forced.is_none() && room.client(0).is_some_and(|c| c.overflow_resync()) {
                    forced = Some(out.tick);
                }
                if out.control.iter().any(|m| matches!(m, Message::Kick(_))) {
                    kicked_at = Some(out.tick);
                }
            },
        );
        (forced, kicked_at)
    };
    let (forced, kicked_at) = run(None);
    let f = forced.expect("an overflow resync");
    assert_eq!(
        kicked_at,
        Some(f + window + 1),
        "kicked on the first tick past the window"
    );
    // The same run, the client acknowledging the forced full state just before tick f + window + 1 runs.
    let (again, kicked_at) = run(Some(f + window + 1));
    assert_eq!(again, Some(f));
    assert!(
        kicked_at.is_none_or(|k| k > f + window + 1),
        "acknowledged within the window: kept ({kicked_at:?})"
    );
}

/// How many of the shots a room's clients were sent were their own (catalogue §3's cosmetic-events row: "never the
/// client whose own shot it is"), and how many were other clients'.
fn echo_case(mut room: Room<World>, scene: &Scene, seed: u64, first: u32) -> (u64, u64) {
    let script = recorded(scene, seed, first, 80);
    let (mut own, mut others) = (0, 0);
    drive(
        &mut room,
        8,
        60,
        |_, _| true,
        |s, t| script.get(&(s, t)).copied(),
        |_, s, out| {
            for snap in &out.snapshots {
                for (_, _, e) in &snap.events {
                    if let SimEvent::Shot { slot, .. } = e {
                        if *slot == s {
                            own += 1;
                        } else {
                            others += 1;
                        }
                    }
                }
            }
        },
    );
    (own, others)
}

/// -62/server-echo's site, clean (U4E-Q10 (a)): the room's routing sends a cosmetic shot to the clients that see it,
/// never to the shooter's own client.
#[test]
fn pm_0001_62_the_room_never_sends_a_shooter_its_own_cosmetic_shot() {
    let scene = Scene::stress();
    let (own, others) = echo_case(world_room(&scene, 5, 600, 8), &scene, 5, 600);
    assert!(others > 0, "shots were routed");
    assert_eq!(own, 0, "{own} own shots sent");
}

/// The wrap runs (-58's room counters, e9's "each counter accepted across its wrap in the clean build"): one client of
/// CO whose room's tick, command sequence and melee press counter each start a few steps below their wraps. Returns
/// the client's counters and every counter wrap the room exposed, by name.
fn wrap_case(mut room: Room<World>) -> (npt_room::ClientCounters, Vec<&'static str>) {
    let mut ack = Ack::default();
    // The client acknowledges two ticks late, so an acknowledgement from before the wrap reaches a room past it.
    let mut late = [Ack::default(); 2];
    let mut wraps = Vec::new();
    for k in 0..40u16 {
        let next = room.last_tick().wrapping_add(1);
        let mut presses = [0; 4];
        presses[Command::PRESS_MELEE] = 250u8.wrapping_add(k as u8);
        let cmd = Command {
            tick: next,
            seq: 65_520u16.wrapping_add(k),
            presses,
            view_tick: next.wrapping_sub(1),
            ..Command::default()
        };
        room.receive(0, &Message::Input(late[0].input(vec![cmd])));
        room.tick();
        ack.take(&collect(&mut room, 0));
        late = [late[1], ack];
        if let Some(c) = room.client(0) {
            wraps.extend_from_slice(c.wraps());
        }
    }
    let counters = room
        .client(0)
        .map(|c| c.counters().clone())
        .unwrap_or_default();
    (counters, wraps)
}

/// -58's three room-side compares in the clean build (u4-build-plan.md §4.14 :467, :468, :471): the acknowledgement,
/// the command sequence and a press counter each cross their wraps and are accepted, each wrap exposed once by name.
#[test]
fn pm_0001_58_the_rooms_ack_sequence_and_press_compares_accept_across_their_wraps() {
    let scene = Scene::course();
    let (k, wraps) = wrap_case(world_room(&scene, 1, 65_520, 1));
    assert_eq!(
        (k.ack_refused, k.commands_repeat, k.press_refused),
        (0, 0, 0),
        "{k:?}"
    );
    assert_eq!(k.commands_buffered, 40);
    for name in ["ack", "cmd_seq", "press"] {
        assert_eq!(
            wraps.iter().filter(|w| **w == name).count(),
            1,
            "{name}: {wraps:?}"
        );
    }
}

/// PM-0001-22/unbounded's clean half and its headroom (U4E-Q12 (c)): the never-acker's headroom never goes below 0.
fn unbounded_case(mut room: Room<World>, scene: &Scene) -> (Vec<u32>, i64, Option<u32>) {
    let script = recorded(scene, 4, 400, 400);
    never_acker(&mut room, &script, 150)
}

/// The room's planted defects (bites builds only), each in wave 1's two parts (u4e-brief.md 2.6): a bites-only twin
/// that asserts the clean test's check fails on the bitten instance; part (ii) is a scratch switch of the clean test.
#[cfg(feature = "bites")]
mod bites {
    use super::*;

    fn bitten(scene: &Scene, seed: u64, first: u32, n: u8, id: &str) -> Room<World> {
        room_with(World::new(scene, seed, first), scene, n, |r| {
            r.with_bite(id).expect("npt-room plants it")
        })
    }

    #[test]
    fn pm_0001_62_server_echo_sends_the_owner_its_own_cosmetic_shots() {
        let scene = Scene::stress();
        let (own, _) = echo_case(
            bitten(&scene, 5, 600, 8, "PM-0001-62/server-echo"),
            &scene,
            5,
            600,
        );
        assert!(own >= 1, "one event sent: {own}");
    }

    #[test]
    fn pm_0001_58_lt_ack_refuses_a_post_wrap_ack() {
        let (k, _) = wrap_case(bitten(&Scene::course(), 1, 65_520, 1, "PM-0001-58/lt-ack"));
        assert!(k.ack_refused >= 1, "{k:?}");
    }

    #[test]
    fn pm_0001_58_lt_cmd_seq_refuses_post_wrap_commands() {
        let (k, _) = wrap_case(bitten(
            &Scene::course(),
            1,
            65_520,
            1,
            "PM-0001-58/lt-cmd-seq",
        ));
        assert!(k.commands_repeat >= 1 && k.commands_buffered < 40, "{k:?}");
    }

    #[test]
    fn pm_0001_58_lt_press_loses_an_edge() {
        let (k, _) = wrap_case(bitten(
            &Scene::course(),
            1,
            65_520,
            1,
            "PM-0001-58/lt-press",
        ));
        assert!(k.press_refused >= 1, "{k:?}");
    }

    #[test]
    fn pm_0001_22_unbounded_keeps_the_never_acker_past_its_cap() {
        let scene = Scene::stress();
        let room = room_with(World::new(&scene, 4, 400), &scene, 8, |r| {
            r.with_event_cap(48)
                .with_bite("PM-0001-22/unbounded")
                .expect("planted")
        });
        let (resyncs, min_headroom, kicked) = unbounded_case(room, &scene);
        assert!(min_headroom < 0, "headroom below 0: {min_headroom}");
        assert!(
            resyncs.is_empty() && kicked.is_none(),
            "no resync and no kick"
        );
    }

    /// A1 (advisor-u4e-r1 R2; plan.md:508-509, "Encoding stays on the tick"): `take_control` hands out the full state
    /// the room encoded with its own `Quant`. Under PM-0001-57/pos-4cm (npt-proto's, forwarded by `Room::with_bite`)
    /// its bytes are that `Quant`'s encoding of the message they decode to, and differ from the clean twin room's,
    /// which are `npt_proto::encode`'s (a re-encoding of the decoded bitten message with `npt_proto::encode` would equal
    /// them: -57 coarsens the integers before packing, and coarse integers pack unchanged).
    #[test]
    fn a1_take_control_hands_out_the_full_state_encoded_with_the_rooms_quant() {
        let id = "PM-0001-57/pos-4cm";
        let scene = Scene::stress();
        let full = |prep: &dyn Fn(Room<World>) -> Room<World>| {
            let mut room = room_with(warmed(&scene, 1, 200, 30), &scene, 1, prep);
            room.tick();
            room.take_control(0)
                .into_iter()
                .find(|b| matches!(control_of(b), Message::FullState(_)))
                .expect("the join's full state")
        };
        let bitten = full(&|r| r.with_bite(id).expect("npt-proto plants it"));
        let clean = full(&|r| r);
        let mut again = Vec::new();
        npt_proto::Quant::with_bite(id)
            .expect("npt-proto plants it")
            .encode(&control_of(&bitten), &mut again)
            .expect("it encodes");
        assert_eq!(again, bitten, "the room's Quant's encoding");
        let mut wire = Vec::new();
        npt_proto::encode(&control_of(&clean), &mut wire).expect("it encodes");
        assert_eq!(
            wire, clean,
            "the clean room's bytes are npt_proto::encode's"
        );
        assert_ne!(bitten, clean, "the bite reached the full state");
    }

    #[test]
    fn catalogue_5_owner_to_all_fails_carry_53s_check() {
        let scene = Scene::stress();
        let room = bitten(&scene, 1, 200, 48, "catalogue-5/owner-to-all");
        let err = own_run(room, &scene, 1, 200, 3, 3).err();
        assert!(
            err.as_deref().is_some_and(|e| e.contains("client 0")),
            "{err:?}"
        );
    }

    #[test]
    fn the_room_takes_npt_protos_ids_through_quant_and_no_other() {
        let scene = Scene::course();
        let room = || {
            Room::new(
                World::new(&scene, 1, 0),
                &scene,
                World::new(&scene, 1, 0).greybox(),
                vec![],
                1,
                30,
            )
        };
        assert!(room().with_bite("PM-0001-17/tick16").is_some());
        assert!(room().with_bite("PM-0001-17/client-rounding").is_none());
        assert!(room().with_bite("PM-0001-18/no-such-bite").is_none());
    }
}

/// -22's clean half with the same cap as its twin: the headroom reaches 0 and never goes below, and one resync follows.
#[test]
fn pm_0001_22_the_never_ackers_headroom_never_goes_below_0() {
    let scene = Scene::stress();
    let room = room_with(World::new(&scene, 4, 400), &scene, 8, |r| {
        r.with_event_cap(48)
    });
    let (resyncs, min_headroom, _) = unbounded_case(room, &scene);
    assert_eq!(min_headroom, 0);
    assert!(!resyncs.is_empty());
}
