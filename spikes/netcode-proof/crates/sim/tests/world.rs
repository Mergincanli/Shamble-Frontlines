//! U4d's tests of `World` (u4-build-plan.md §5 U4d): D-1, D-3 to D-6 and D-8 to D-10, and the carries of
//! u4s-carries.md that hold `World` (7, 8, 9, 10, 13, 14, 15, 37, 38); in bites builds, the unit proofs of the sites
//! `World` plants (§4.14: PM-0001-41's two; PM-0004-21's std `sin` site) and bites per instance (§4.11, T10). D-2 and
//! D-7 are tests/replay.rs's.

use npt_sim::grid::{Config, Field, PITCH, POSITION_XY, POSITION_Z, VELOCITY, YAW};
use npt_sim::replay::Recording;
use npt_sim::{
    Attribute, AttributeChange, Command, CoverVariant, EntityKind, EntityRecord, HitHistory,
    HitVolumes, InputReports, Placement, Ray, RayHit, Refusal, RefusalKind, Rewind, Scene, SimCore,
    SimEvent, StateDump, TICK_HZ, TickInput, VolumeBox, World, ticks_for_ms,
};

/// The ring span the tests lend: 2 s, 60 ticks at 30 Hz.
const SPAN_MS: u32 = 2_000;
/// The courses' weapon reach (Scene::course and Scene::targets), cm, and the clamp's slack: one grid cell's
/// half-diagonal, rounded up (u4-build-plan.md §4.2: 0.87 cm).
const REACH_CM: f64 = 6_000.0;
const SLACK_M: f64 = 0.0087;
/// The fire origin's height above the feet (World's chest point), m.
const CHEST: f32 = 1.4;

fn ring() -> HitHistory {
    HitHistory::for_ms(SPAN_MS).expect("2 s of ticks")
}

/// One room tick (u4-build-plan.md §4.2): step on the lent ring, then push the new volumes into it.
fn room_tick(world: &mut World, history: &mut HitHistory, input: &TickInput) -> Vec<SimEvent> {
    let events = world.step(input, history).to_vec();
    history.push(world.hit_volumes());
    events
}

/// A small integer generator for the tests' scripts (xorshift32): no float, no std random.
struct Dice(u32);

impl Dice {
    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    fn below(&mut self, n: u32) -> u32 {
        ((u64::from(self.next()) * u64::from(n)) >> 32) as u32
    }

    fn between(&mut self, lo: i32, hi: i32) -> i32 {
        lo + self.below((hi - lo + 1) as u32) as i32
    }
}

/// Slot `slot`'s record of `kind`.
fn record_of(world: &World, kind: EntityKind, slot: u8) -> EntityRecord {
    *world
        .entity_records()
        .iter()
        .find(|r| r.kind == kind && r.index == u16::from(slot))
        .expect("every slot has a record of each soldier kind")
}

/// Slot `slot`'s soldier record (the remote row: position 0-2, yaw 3, pitch 4, flags 5, health 6, weapon 7).
fn soldier(world: &World, slot: u8) -> EntityRecord {
    record_of(world, EntityKind::Soldier, slot)
}

/// Slot `slot`'s own-state record (W1-S1: velocity 0-2, ammunition 3, fire timer 4, reload left 5, move speed 6, fire
/// interval 7).
fn own_state(world: &World, slot: u8) -> EntityRecord {
    record_of(world, EntityKind::OwnState, slot)
}

/// The dump's words whose names start with `prefix`, by index.
fn named(d: &StateDump, prefix: &str) -> Vec<usize> {
    (0..d.words().len())
        .filter(|&i| d.field(i).is_some_and(|f| f.starts_with(prefix)))
        .collect()
}

/// The dump word named exactly `name`.
fn word(d: &StateDump, name: &str) -> u32 {
    let i = (0..d.words().len())
        .find(|&i| d.field(i).as_deref() == Some(name))
        .unwrap_or_else(|| panic!("the dump has no {name}"));
    d.words()[i]
}

/// An input of `commands` alone, for the world's next tick.
fn input(world: &World, commands: Vec<(Command, Option<Rewind>)>) -> TickInput {
    TickInput {
        tick: world.next_tick(),
        commands,
        ..TickInput::default()
    }
}

/// The eight headings the scripts walk, at full speed.
const HEADINGS: [[i8; 2]; 8] = [
    [127, 0],
    [89, 89],
    [0, 127],
    [-89, 89],
    [-127, 0],
    [-89, -89],
    [0, -127],
    [89, -89],
];

/// Slot `slot`'s scripted command at step `k` from `at` (its feet, cm): a new heading every 45 steps, its aim 10 m
/// ahead at chest height, fire held, crouch or sprint by stretches, a fire press every 13 steps and a reload press every
/// 97 (no throw: the authority alone throws).
fn walk(slot: u8, tick: u32, k: u32, at: [i32; 3], presses: &mut [u8; 4]) -> Command {
    let h = HEADINGS[((k / 45) % 8) as usize];
    let buttons = match (k / 45) % 4 {
        0 => 0,
        1 => Command::FIRE,
        2 => Command::CROUCH | Command::FIRE,
        _ => Command::SPRINT,
    };
    if k % 13 == 5 {
        presses[Command::PRESS_FIRE] = presses[Command::PRESS_FIRE].wrapping_add(1);
    }
    if k % 97 == 50 {
        presses[Command::PRESS_RELOAD] = presses[Command::PRESS_RELOAD].wrapping_add(1);
    }
    Command {
        slot,
        tick,
        seq: k as u16,
        buttons,
        presses: *presses,
        move_xy: h,
        aim: [
            at[0] + i32::from(h[0]) * 8,
            at[1] + i32::from(h[1]) * 8,
            at[2] + 140,
        ],
        view_tick: tick.wrapping_sub(3),
        view_fraction: 0,
    }
}

/// Each scene, a seed and a first tick (one crossing the u32 wrap), for the tests that run all four.
fn scenes() -> [(Scene, u64, u32); 4] {
    [
        (Scene::stress(), 11, 1_000),
        (Scene::estimate(), 12, u32::MAX - 299),
        (Scene::course(), 13, 0),
        (Scene::targets(), 14, 77),
    ]
}

#[test]
fn d1_two_worlds_on_one_recording_agree_every_tick() {
    // World and the replay driver are Send (u4-build-plan.md §4.11, A25: U6 moves the room's world to its tick thread).
    fn send<T: Send>() {}
    send::<World>();
    send::<npt_sim::replay::Replayer>();
    let (mut events, mut hits, mut clamps) = (0usize, 0usize, 0usize);
    for (scene, seed, first) in scenes() {
        let rec = Recording::synthetic(&scene, seed, first, 600, SPAN_MS);
        let mut a = World::new(&scene, seed, first);
        let mut b = World::new(&scene, seed, first);
        let (mut ha, mut hb) = (ring(), ring());
        assert_eq!(a.state_hash(), b.state_hash(), "{}", scene.name());
        for (k, t) in rec.ticks.iter().enumerate() {
            let ea = room_tick(&mut a, &mut ha, t);
            let eb = room_tick(&mut b, &mut hb, t);
            let at = format!("{} tick {k}", scene.name());
            assert_eq!(ea, eb, "{at}: events");
            assert_eq!(a.state_hash(), b.state_hash(), "{at}: state hash");
            assert_eq!(a.entity_records(), b.entity_records(), "{at}: records");
            assert_eq!(a.hit_volumes(), b.hit_volumes(), "{at}: volumes");
            assert_eq!(a.portal_states(), b.portal_states(), "{at}: portals");
            assert_eq!(a.reports(), b.reports(), "{at}: reports");
            // The dump is the hashed state, word for word (carry 38).
            assert_eq!(a.dump().hash(), a.state_hash(), "{at}: dump hash");
            events += ea.len();
            hits += ea
                .iter()
                .filter(|e| matches!(e, SimEvent::Hit { .. }))
                .count();
            clamps += a.reports().clamps.len();
        }
    }
    // Not vacuous: the runs shot, hit and clamped.
    assert!(
        events > 0 && hits > 0 && clamps > 0,
        "{events} events, {hits} hits, {clamps} clamps"
    );
}

#[test]
fn d3_prediction_equals_authority_for_one_slot_bit_for_bit() {
    // Each step the authority runs slot 0's command(s) and a clone of it before the step predicts them; both of the
    // slot's records (its soldier record and its own-state record, W1-S1) and its state words agree bit for bit. Every
    // ninth step is a catch-up tick (the advisor's W1-U4D N-M3): two commands of the slot in one tick, the first holding
    // fire, which the client predicts as two calls; a per-tick rule in the step that predict applies per command (P11's
    // form) differs there.
    let (mut blocked, mut catch_ups) = (0u32, 0u32);
    for scene in [Scene::course(), Scene::stress()] {
        let mut authority = World::new(&scene, 7, 500);
        let mut history = ring();
        let mut presses = [0u8; 4];
        let mut fired = 0u32;
        let mut k = 0u32;
        for step in 0..1_200u32 {
            let at = soldier(&authority, 0).grid;
            let feet = [at[0], at[1], at[2]];
            let tick = authority.next_tick();
            let mut commands = vec![walk(0, tick, k, feet, &mut presses)];
            k += 1;
            if step % 9 == 4 {
                commands[0].tick = tick.wrapping_sub(1);
                commands[0].buttons |= Command::FIRE;
                commands.push(walk(0, tick, k, feet, &mut presses));
                k += 1;
            }
            let mut client = authority.clone();
            let before = client.dump();
            let with_rewinds = commands
                .iter()
                .map(|c| {
                    let fire_held = c.buttons & Command::FIRE != 0;
                    let rewind = fire_held.then(|| Rewind {
                        tick_a: tick.wrapping_sub(2),
                        fraction: 0,
                        cover: CoverVariant::Present,
                    });
                    (*c, rewind)
                })
                .collect();
            let t = input(&authority, with_rewinds);
            room_tick(&mut authority, &mut history, &t);
            // The client: the movers of the snapshot it decoded (here exact: the authority's records), then predict.
            client.load_movers(authority.entity_records());
            let shots: Vec<bool> = commands.iter().map(|c| client.predict(c)).collect();
            fired += shots.iter().filter(|s| **s).count() as u32;
            catch_ups += u32::from(shots.len() == 2 && shots[0]);
            let at = format!("{} step {step}", scene.name());
            // Bit for bit, the snap included: the records' raw (pre-snap) and grid words, and the snapped state.
            assert_eq!(soldier(&client, 0), soldier(&authority, 0), "{at}: record");
            assert_eq!(
                own_state(&client, 0),
                own_state(&authority, 0),
                "{at}: own-state record"
            );
            let (a, p) = (authority.dump(), client.dump());
            for i in named(&a, "soldier[0].") {
                assert_eq!(a.words()[i], p.words()[i], "{at}: {:?}", a.field(i));
            }
            // The predicting world drew from no random stream.
            for i in named(&before, "rng[") {
                assert_eq!(before.words()[i], p.words()[i], "{at}: {:?}", p.field(i));
            }
            // A move the world stopped: velocity 0 on an axis the last command moved along.
            let c = commands[commands.len() - 1];
            let vel = own_state(&authority, 0).grid;
            blocked +=
                u32::from((c.move_xy[0] != 0 && vel[0] == 0) || (c.move_xy[1] != 0 && vel[1] == 0));
        }
        assert!(fired > 50, "{}: {fired} predicted shots", scene.name());
    }
    assert!(blocked > 20, "{blocked} blocked moves: the collision ran");
    assert!(
        catch_ups > 40,
        "{catch_ups} catch-up ticks whose first command fired"
    );
}

/// `Command`'s fields' types, in order: integers only.
type CommandFields = (u8, u32, u16, u16, [u8; 4], [i8; 2], [i32; 3], u32, u8);
/// `TickInput`'s fields' types, in order: integers and integer types only.
type TickInputFields = (
    u32,
    Vec<(Command, Option<Rewind>)>,
    Vec<Placement>,
    Vec<Placement>,
    Vec<AttributeChange>,
);

/// Each type's fields, by an exhaustive destructuring, bound to a tuple of integer types: a field added, or one whose
/// type is a float, fails to compile here (D-4: "finite by type"; carry 37).
#[test]
fn d4_every_value_that_enters_the_sim_is_an_integer_by_type() {
    let Command {
        slot,
        tick,
        seq,
        buttons,
        presses,
        move_xy,
        aim,
        view_tick,
        view_fraction,
    } = Command::default();
    let _: CommandFields = (
        slot,
        tick,
        seq,
        buttons,
        presses,
        move_xy,
        aim,
        view_tick,
        view_fraction,
    );
    let Rewind {
        tick_a,
        fraction,
        cover,
    } = Rewind {
        tick_a: 0,
        fraction: 0,
        cover: CoverVariant::Rewound,
    };
    let _: (u32, u8) = (tick_a, fraction);
    match cover {
        CoverVariant::Rewound | CoverVariant::Present => {}
    }
    let TickInput {
        tick,
        commands,
        spawns,
        teleports,
        attributes,
    } = TickInput::default();
    let _: TickInputFields = (tick, commands, spawns, teleports, attributes);
    let Placement { slot, position } = Placement::default();
    let _: (u8, [i32; 3]) = (slot, position);
    let AttributeChange {
        slot,
        attribute,
        value,
    } = AttributeChange {
        slot: 0,
        attribute: Attribute::MoveSpeed,
        value: 0,
    };
    let _: (u8, u32) = (slot, value);
    match attribute {
        Attribute::MoveSpeed | Attribute::FireInterval => {}
    }
    // So nothing is refused or counted as non-finite: InputReports has no such counter (NF-D4), and a refusal's kind is
    // one of the five.
    let InputReports {
        clamps,
        refusals,
        cooldown,
        ammunition,
        reload,
        rewind_outside,
        pool_full,
    } = InputReports::default();
    for r in &refusals {
        match r.kind {
            RefusalKind::Cooldown
            | RefusalKind::Ammunition
            | RefusalKind::Reload
            | RefusalKind::RewindOutside
            | RefusalKind::PoolFull => {}
        }
    }
    assert!(clamps.is_empty() && refusals.is_empty());
    assert_eq!(
        [cooldown, ammunition, reload, rewind_outside, pool_full],
        [0; 5]
    );
}

#[test]
#[cfg_attr(
    not(debug_assertions),
    ignore = "World's finite assert is a debug assertion"
)]
fn d4_the_state_holds_no_non_finite_value_on_any_tick() {
    // World::step debug-asserts, after every tick, that no float of the state is NaN or infinite (each read by its
    // bits, as the hash reads it); this drives it over every scene. A planted 0.0 / 0.0 in a phase trips it at that
    // tick (the scratch plant, u4d-build.md). Ignored where debug assertions are off.
    for (scene, seed, first) in scenes() {
        let rec = Recording::synthetic(&scene, seed, first, 300, SPAN_MS);
        let mut world = World::new(&scene, seed, first);
        let mut history = ring();
        for t in &rec.ticks {
            room_tick(&mut world, &mut history, t);
        }
    }
}

#[test]
fn d4_extreme_recorded_commands_step_without_an_overflow() {
    // Carry 15: every integer a recording can hold, at its ends, steps with overflow checks on (the dev profile's, and
    // the release profile's for npt-sim: Cargo.toml) and without a panic: move_xy at -128 and 127 (the room never
    // sends -128), aim at i32::MIN and i32::MAX, counters and sequences wrapping, Rewinds at the u32 ends, placements at
    // i32's ends, attributes at u32's.
    let moves = [
        [-128, -128],
        [127, 127],
        [-128, 127],
        [127, -128],
        [0, -128],
        [-128, 0],
    ];
    let aims = [
        [i32::MIN; 3],
        [i32::MAX; 3],
        [i32::MIN, i32::MAX, i32::MIN],
        [i32::MAX, i32::MIN, 0],
        [0, 0, i32::MAX],
    ];
    for scene in [Scene::course(), Scene::stress(), Scene::targets()] {
        let mut world = World::new(&scene, 3, u32::MAX - 60);
        let mut history = ring();
        for k in 0..120u32 {
            let mut t = input(&world, Vec::new());
            for slot in 0..scene.slots().min(6) {
                let i = (k + u32::from(slot)) as usize;
                let c = Command {
                    slot,
                    tick: t.tick,
                    seq: u16::MAX.wrapping_add(k as u16),
                    buttons: u16::MAX,
                    presses: [(255u32.wrapping_add(k) & 0xff) as u8; 4],
                    move_xy: moves[i % moves.len()],
                    aim: aims[i % aims.len()],
                    view_tick: u32::MAX,
                    view_fraction: u8::MAX,
                };
                let rewind = Rewind {
                    tick_a: [u32::MAX, 0, t.tick.wrapping_sub(1)][i % 3],
                    fraction: u8::MAX,
                    cover: CoverVariant::Rewound,
                };
                t.commands.push((c, Some(rewind)));
            }
            t.commands.push((
                Command {
                    slot: u8::MAX,
                    tick: t.tick,
                    ..Command::default()
                },
                None,
            ));
            if k % 10 == 0 {
                let ends = [i32::MIN, i32::MAX, i32::MIN];
                t.spawns.push(Placement {
                    slot: 0,
                    position: ends,
                });
                t.teleports.push(Placement {
                    slot: 1,
                    position: [i32::MAX, i32::MIN, i32::MAX],
                });
                t.spawns.push(Placement {
                    slot: u8::MAX,
                    position: ends,
                });
                t.attributes.push(AttributeChange {
                    slot: 0,
                    attribute: Attribute::MoveSpeed,
                    value: u32::MAX,
                });
                t.attributes.push(AttributeChange {
                    slot: 1,
                    attribute: [Attribute::FireInterval, Attribute::MoveSpeed]
                        [(k / 10 % 2) as usize],
                    value: [0, u32::MAX][(k / 20 % 2) as usize],
                });
            }
            room_tick(&mut world, &mut history, &t);
        }
        // Each soldier's grid entries stay in their fields' ranges (the snap brings every float into its field).
        for r in world.entity_records() {
            for (f, k) in r.kind.layout().iter().zip(r.grid) {
                assert!((f.min()..=f.max()).contains(&k), "{r:?}");
            }
        }
    }
}

#[test]
fn d5_a_full_pool_refuses_and_counts_and_leaves_no_trace() {
    // CO's projectile pool holds 16 (no stand-ins): a throw each tick fills it in 16 ticks (a grenade's fuse is 2 s),
    // and the next 4 throws are refused and counted. The twin's stream is the forged one with the 4 refused presses
    // removed (the throw counter one lower from each on): equal state hashes on every tick, past the next accepted
    // throw at step 80.
    let scene = Scene::course();
    let (mut forged, mut twin) = (Vec::new(), Vec::new());
    let (mut pf, mut pt) = ([0u8; 4], [0u8; 4]);
    for k in 0..120u32 {
        let press = k < 20 || k == 80;
        if press {
            pf[Command::PRESS_THROW] = pf[Command::PRESS_THROW].wrapping_add(1);
        }
        if press && !(16..20).contains(&k) {
            pt[Command::PRESS_THROW] = pt[Command::PRESS_THROW].wrapping_add(1);
        }
        let c = |presses| Command {
            slot: 0,
            tick: k,
            seq: k as u16,
            presses,
            move_xy: [0, 40],
            aim: [0, -500, 140],
            ..Command::default()
        };
        forged.push(TickInput {
            tick: k,
            commands: vec![(c(pf), None)],
            ..TickInput::default()
        });
        twin.push(TickInput {
            tick: k,
            commands: vec![(c(pt), None)],
            ..TickInput::default()
        });
    }
    let (rf, rt, worlds) = twins(&scene, &forged, &twin, |k, w| {
        if k == 19 {
            let live = w
                .entity_records()
                .iter()
                .filter(|r| r.kind == EntityKind::Projectile)
                .count();
            assert_eq!(live, 16, "the pool full at step 19");
        }
    });
    assert_eq!(rf.pool_full, 4, "four throws refused and counted");
    assert_eq!(rt.pool_full, 0);
    // Each refused throw is listed for its tick by slot, sequence and kind: what the twin removed (W1-U4D).
    let refused: Vec<Refusal> = (16..20)
        .map(|seq| Refusal {
            slot: 0,
            seq,
            kind: RefusalKind::PoolFull,
        })
        .collect();
    assert_eq!(rf.refusals, refused);
    assert!(rt.refusals.is_empty());
    let thrown = worlds
        .entity_records()
        .iter()
        .filter(|r| r.kind == EntityKind::Projectile)
        .count();
    assert_eq!(
        thrown, 1,
        "the throw at step 80 accepted, the first 16 blown"
    );
}

/// Runs `forged` and `twin` on two worlds of `scene` (seed 5, each its own ring) and asserts equal state hashes and
/// equal event lists on every tick (u4-build-plan.md §4.2: refused input leaves no trace; D-10). `each` sees the forged
/// world after each tick. Returns both worlds' last reports, with every tick's clamp reports and refusals gathered in
/// them, and the forged world.
fn twins(
    scene: &Scene,
    forged: &[TickInput],
    twin: &[TickInput],
    mut each: impl FnMut(usize, &World),
) -> (InputReports, InputReports, World) {
    let first = forged[0].tick;
    let (mut f, mut t) = (World::new(scene, 5, first), World::new(scene, 5, first));
    let (mut hf, mut ht) = (ring(), ring());
    let (mut clamps_f, mut clamps_t) = (Vec::new(), Vec::new());
    let (mut refusals_f, mut refusals_t) = (Vec::new(), Vec::new());
    for (k, (a, b)) in forged.iter().zip(twin).enumerate() {
        let ef = room_tick(&mut f, &mut hf, a);
        let et = room_tick(&mut t, &mut ht, b);
        assert_eq!(ef, et, "step {k}: the event lists");
        assert_eq!(f.state_hash(), t.state_hash(), "step {k}: the state hashes");
        assert_eq!(f.dump(), t.dump(), "step {k}: the dumps");
        clamps_f.extend_from_slice(&f.reports().clamps);
        clamps_t.extend_from_slice(&t.reports().clamps);
        refusals_f.extend_from_slice(&f.reports().refusals);
        refusals_t.extend_from_slice(&t.reports().refusals);
        each(k, &f);
    }
    let mut rf = f.reports().clone();
    let mut rt = t.reports().clone();
    (rf.clamps, rt.clamps) = (clamps_f, clamps_t);
    (rf.refusals, rt.refusals) = (refusals_f, refusals_t);
    (rf, rt, f)
}

/// Statistics of a stress-scene run, per tick and per second (PM-0001-2's series).
#[derive(Default)]
struct Load {
    ticks: u32,
    min_soldiers: usize,
    min_team: usize,
    min_vehicles: usize,
    min_large: usize,
    min_projectiles: usize,
    effects: [usize; 2],
    shots: u64,
    kills: u64,
    portal_changes: u64,
    /// The farthest a very large unit's hull came from the cluster's centre, and the nearest any other came, cm.
    large_out: i64,
    other_in: i64,
    /// The farthest a stand-in projectile started from the centre in x or y, cm.
    start_out: i32,
}

#[test]
fn d6_the_stress_scene_meets_pm_0001_2_on_every_tick() {
    // PM-0001-2's thresholds exactly (marks.json; the advisor's condition 5): on every tick 48 soldiers (alive or
    // awaiting respawn), three teams of at least 16, at least 15 big vehicles of which at least 4 have 20 parts or more,
    // at least 300 projectiles, and 384 effect rows; the run's means of shots per second in 665-735 and of portal
    // changes per second in 0.95-1.05. Over the synthetic SS recording, 3,000 ticks (100 s). In the fight (the advisor's
    // W1-U4D N-M4; §4.13, F113): every tick the 4 very large units' hulls within 14 m of the cluster's centre (their
    // parts within 3.71 m of them) and the other 11 at least 52 m out; every stand-in projectile starts within 12 m in
    // x and y. The magazine (N-M1): SS and ES script no reloads, so magazine × interval covers an S30 plus 5 minutes.
    let s30_plus_5_us = 35 * 60 * 1_000_000u64;
    for s in [Scene::stress(), Scene::estimate()] {
        let w = s.weapon();
        let covered = u64::from(w.magazine) * u64::from(w.interval_us);
        assert!(covered >= s30_plus_5_us, "{}: {w:?}", s.name());
        println!(
            "D-6: {} weapon {w:?}, health {}; the magazine lasts {} s at one soldier's rate",
            s.name(),
            s.health(),
            covered / 1_000_000
        );
    }
    let scene = Scene::stress();
    let seconds = 100u32;
    let rec = Recording::synthetic(&scene, 1, 0, seconds * TICK_HZ, SPAN_MS);
    let mut world = World::new(&scene, 1, 0);
    let mut history = ring();
    let mut load = Load {
        min_soldiers: usize::MAX,
        min_team: usize::MAX,
        min_vehicles: usize::MAX,
        min_large: usize::MAX,
        min_projectiles: usize::MAX,
        effects: [usize::MAX, 0],
        other_in: i64::MAX,
        ..Load::default()
    };
    let mut open = world.portal_states().open.clone();
    for t in &rec.ticks {
        let events = room_tick(&mut world, &mut history, t);
        let records = world.entity_records();
        let of = |k: EntityKind| records.iter().filter(move |r| r.kind == k);
        let soldiers: Vec<&EntityRecord> = of(EntityKind::Soldier).collect();
        let mut teams = [0usize; 3];
        for s in &soldiers {
            let team = scene
                .team(s.index as u8)
                .expect("a soldier's slot is the scene's");
            teams[usize::from(team)] += 1;
        }
        let mut parts = [0usize; 64];
        for p in of(EntityKind::VehiclePart) {
            parts[p.grid[5] as usize] += 1;
        }
        for v in of(EntityKind::Vehicle) {
            let (x, y) = (i64::from(v.grid[0]), i64::from(v.grid[1]));
            let out = (((x * x + y * y) as f64).sqrt()) as i64;
            if parts[usize::from(v.index)] >= 20 {
                load.large_out = load.large_out.max(out);
            } else {
                load.other_in = load.other_in.min(out);
            }
        }
        for p in of(EntityKind::Projectile).filter(|p| p.grid[7] == 0) {
            load.start_out = load.start_out.max(p.grid[1].abs()).max(p.grid[2].abs());
        }
        load.ticks += 1;
        load.min_soldiers = load.min_soldiers.min(soldiers.len());
        load.min_team = load.min_team.min(teams.into_iter().min().unwrap_or(0));
        load.min_vehicles = load.min_vehicles.min(of(EntityKind::Vehicle).count());
        load.min_large = load
            .min_large
            .min(parts.iter().filter(|&&n| n >= 20).count());
        load.min_projectiles = load.min_projectiles.min(of(EntityKind::Projectile).count());
        let effects = of(EntityKind::Effect).count();
        load.effects = [load.effects[0].min(effects), load.effects[1].max(effects)];
        load.shots += events
            .iter()
            .filter(|e| matches!(e, SimEvent::Shot { .. }))
            .count() as u64;
        load.kills += events
            .iter()
            .filter(|e| matches!(e, SimEvent::Kill { .. }))
            .count() as u64;
        let now = &world.portal_states().open;
        load.portal_changes += now.iter().zip(&open).filter(|(a, b)| a != b).count() as u64;
        open.clone_from(now);
    }
    assert_eq!(load.ticks, seconds * TICK_HZ);
    println!(
        "D-6: very large units at most {} cm out, the others at least {} cm out; stand-ins start within {} cm; {} \
         kills in {seconds} s",
        load.large_out, load.other_in, load.start_out, load.kills
    );
    assert!(
        load.large_out <= 1_401,
        "the very large units: {} cm out",
        load.large_out
    );
    assert!(
        load.other_in >= 5_199,
        "the other vehicles: {} cm out",
        load.other_in
    );
    assert!(
        load.start_out <= 1_200,
        "stand-ins start {} cm out",
        load.start_out
    );
    assert!(load.min_soldiers >= 48, "soldiers: {}", load.min_soldiers);
    assert!(load.min_team >= 16, "smallest team: {}", load.min_team);
    assert!(
        load.min_vehicles >= 15,
        "big vehicles: {}",
        load.min_vehicles
    );
    assert!(load.min_large >= 4, "units of 20 parts: {}", load.min_large);
    assert!(
        load.min_projectiles >= 300,
        "projectiles: {}",
        load.min_projectiles
    );
    assert_eq!(load.effects, [384, 384], "effect rows");
    // Means per second, in integers: shots × 1,000 / seconds against 665,000-735,000; portal changes × 100 against
    // 95-105 per second.
    let shots_milli = load.shots * 1_000 / u64::from(seconds);
    let portals_centi = load.portal_changes * 100 / u64::from(seconds);
    println!(
        "D-6: {} shots in {seconds} s, mean {}.{:03} a second; {} portal changes, mean {}.{:02}",
        load.shots,
        shots_milli / 1_000,
        shots_milli % 1_000,
        load.portal_changes,
        portals_centi / 100,
        portals_centi % 100
    );
    assert!(
        (665_000..=735_000).contains(&shots_milli),
        "shots a second: {shots_milli} thousandths"
    );
    assert!(
        (95..=105).contains(&portals_centi),
        "portal changes a second: {portals_centi} hundredths"
    );
}

/// `world`'s records of `kind`, in order.
fn records_of(world: &World, kind: EntityKind) -> Vec<EntityRecord> {
    world
        .entity_records()
        .iter()
        .filter(|r| r.kind == kind)
        .copied()
        .collect()
}

#[test]
fn h1_a_hull_that_moves_and_turns_leaves_its_parts_records_unchanged() {
    // The advisor's W1-U4D H1 (ADR 0001:313-314; the catalogue's vehicles row: "parts travel with their parent"): a
    // part's record holds its transform relative to its hull, so while nothing hits it, it stays the same as its hull
    // drives and turns, and its world box, its hull's record composed with it, moves with the hull. The stress scene's
    // 124 parts (4 × 20 + 11 × 4), 300 ticks with no command.
    let scene = Scene::stress();
    let mut world = World::new(&scene, 3, 0);
    let mut history = ring();
    let parts = records_of(&world, EntityKind::VehiclePart);
    let hulls = records_of(&world, EntityKind::Vehicle);
    let cover = world.hit_volumes().cover.clone();
    assert_eq!((parts.len(), hulls.len()), (124, 15));
    for k in 0..300 {
        let t = input(&world, Vec::new());
        room_tick(&mut world, &mut history, &t);
        assert_eq!(
            records_of(&world, EntityKind::VehiclePart),
            parts,
            "tick {k}: the parts' records"
        );
    }
    for (a, b) in hulls.iter().zip(&records_of(&world, EntityKind::Vehicle)) {
        assert_ne!(a.grid[..2], b.grid[..2], "hull {} moved", a.index);
        assert_ne!(a.grid[3], b.grid[3], "hull {} turned", a.index);
    }
    let now = &world.hit_volumes().cover;
    for i in 0..parts.len() {
        assert!(now[i].live, "part {i}");
        assert_ne!(
            now[i].center, cover[i].center,
            "part {i}'s box moved with its hull"
        );
    }
}

#[test]
fn h1_part_records_change_only_when_their_health_does() {
    // Probe K's count re-run (the test review; W1-U4D H1): over a stress run, on every tick the part records that
    // changed number at most the parts whose health changed. The other kinds' changes a tick are counted beside them,
    // for U4e's budget (reported, not judged).
    let scene = Scene::stress();
    let ticks = 600u32;
    let rec = Recording::synthetic(&scene, 1, 0, ticks, SPAN_MS);
    let mut world = World::new(&scene, 1, 0);
    let mut history = ring();
    let key = |r: &EntityRecord| (r.kind, r.index, r.generation);
    let mut before: std::collections::BTreeMap<_, EntityRecord> = world
        .entity_records()
        .iter()
        .map(|r| (key(r), *r))
        .collect();
    let mut health = world.dump();
    let mut changed = std::collections::BTreeMap::<EntityKind, u64>::new();
    let (mut hit_parts, mut changed_parts) = (0u64, 0u64);
    for (k, t) in rec.ticks.iter().enumerate() {
        room_tick(&mut world, &mut history, t);
        let now: std::collections::BTreeMap<_, EntityRecord> = world
            .entity_records()
            .iter()
            .map(|r| (key(r), *r))
            .collect();
        let mut parts = 0;
        for (id, r) in &now {
            if before.get(id) != Some(r) {
                *changed.entry(id.0).or_default() += 1;
                parts += u64::from(id.0 == EntityKind::VehiclePart);
            }
        }
        let d = world.dump();
        let hit = named(&d, "part[")
            .into_iter()
            .filter(|&i| d.words()[i] != health.words()[i])
            .count() as u64;
        assert!(
            parts <= hit,
            "tick {k}: {parts} part records changed, {hit} parts' health"
        );
        (hit_parts, changed_parts) = (hit_parts + hit, changed_parts + parts);
        (before, health) = (now, d);
    }
    assert!(
        hit_parts > 0,
        "not vacuous: shots hit the parts in the fight"
    );
    let per_tick: Vec<String> = changed
        .iter()
        .map(|(k, n)| {
            format!(
                "{k:?} {}.{:02}",
                n / u64::from(ticks),
                n * 100 / u64::from(ticks) % 100
            )
        })
        .collect();
    println!(
        "H1: records changed a tick over {ticks} ticks: {}; parts hit {hit_parts}, part records changed {changed_parts}",
        per_tick.join(", ")
    );
}

#[test]
fn n_m5_an_effect_rows_record_changes_only_when_the_row_starts_or_ends() {
    // The advisor's W1-U4D N-M5: an effect row's record holds the tick it ends, so it changes only at the tick the row
    // ends and is rolled again (its end then 0.5 to 3 s on: 15 to 90 ticks at 30 Hz), or when its owner's generation or
    // area changes. The per-tick expiry check stays (S1's load). Over a stress run every changed row is one of those,
    // and a few change a tick, where all 384 did when the record counted down.
    let scene = Scene::stress();
    let ticks = 600u32;
    let rec = Recording::synthetic(&scene, 1, 0, ticks, SPAN_MS);
    let mut world = World::new(&scene, 1, 0);
    let mut history = ring();
    let mut before = records_of(&world, EntityKind::Effect);
    assert_eq!(before.len(), 384);
    let (lo, hi) = (ticks_for_ms(500), ticks_for_ms(3_000));
    let mut changed = 0u64;
    for (k, t) in rec.ticks.iter().enumerate() {
        let tick = world.next_tick();
        room_tick(&mut world, &mut history, t);
        let now = records_of(&world, EntityKind::Effect);
        for (a, b) in before.iter().zip(&now) {
            if a == b {
                continue;
            }
            changed += 1;
            let (end_a, end_b) = (a.grid[1] as u32, b.grid[1] as u32);
            let rolled = end_a == tick && (lo..=hi).contains(&end_b.wrapping_sub(tick));
            let owner = a.generation != b.generation || a.area != b.area;
            assert!(rolled || owner, "tick {k}: row {} {a:?} -> {b:?}", a.index);
        }
        before = now;
    }
    let milli = changed * 1_000 / u64::from(ticks);
    println!(
        "N-M5: {changed} effect records changed in {ticks} ticks, {}.{:03} a tick of 384",
        milli / 1_000,
        milli % 1_000
    );
    assert!(milli < 20_000, "{milli} thousandths of a row a tick");
}

#[test]
fn a_m1_the_authoritys_volumes_are_the_function_of_its_records() {
    // The advisor's W1-U4D A-M1: `World::hit_volumes_from` is the one function from records to HitVolumes. On every
    // tick of a stress run the authority's volumes equal the function applied to its own records, and another World of
    // the scene (a client) given the same records (what it decoded) builds the same volumes, into its own storage,
    // which it keeps; a client's `load_movers` of those records makes its present volumes the same boxes, labelled with
    // its own tick counter (carry 15). 1,200 ticks: SS's kills are rare (about 0.3 a second), and a dead soldier's box
    // must show.
    let scene = Scene::stress();
    let rec = Recording::synthetic(&scene, 5, 0, 1_200, SPAN_MS);
    let mut world = World::new(&scene, 5, 0);
    let client = World::new(&scene, 77, 9);
    let mut loaded = World::new(&scene, 77, 9);
    let mut history = ring();
    let (mut own, mut decoded) = (HitVolumes::default(), HitVolumes::default());
    let mut storage = None;
    let mut dead = 0;
    for (k, t) in rec.ticks.iter().enumerate() {
        room_tick(&mut world, &mut history, t);
        let tick = world.hit_volumes().tick;
        world.hit_volumes_from(world.entity_records(), tick, &mut own);
        assert_eq!(&own, world.hit_volumes(), "tick {k}: the authority's");
        client.hit_volumes_from(world.entity_records(), tick, &mut decoded);
        assert_eq!(&decoded, world.hit_volumes(), "tick {k}: a client's");
        let at = (decoded.soldiers.as_ptr(), decoded.cover.as_ptr());
        assert_eq!(*storage.get_or_insert(at), at, "tick {k}: the storage kept");
        dead += decoded.soldiers.iter().filter(|b| !b.live).count();
        loaded.load_movers(world.entity_records());
        let v = loaded.hit_volumes();
        assert_eq!(
            (v.tick, &v.soldiers, &v.cover),
            (8, &decoded.soldiers, &decoded.cover),
            "tick {k}: a client's volumes after load_movers"
        );
    }
    assert_eq!(world.hit_volumes().cover.len(), 124 + 6 + 1);
    assert!(dead > 0, "not vacuous: dead soldiers' boxes are not live");
}

#[test]
fn r_m1_a_record_of_an_entity_the_world_has_not_changes_nothing() {
    // The rust review's M1 = the test review's M5 (probe C): `load_movers` and `hit_volumes_from` skip a record whose
    // index the world has no entity for, and grow nothing from decoded data. CO (no vehicle, 6 doors, the wall): probe
    // C's part record of index 400, a hull of index 9, a door of index 99, a wall of index 3; then a predict and a step.
    // Its volumes keep CO's 7 cover boxes and equal a world's that never loaded them, and so do its state and records.
    // (The movers' own sizes are world.rs's test.)
    let bogus = |kind, index, values: &[i32]| {
        let mut grid = [0; EntityRecord::FIELDS];
        grid[..values.len()].copy_from_slice(values);
        EntityRecord {
            kind,
            index,
            generation: 0,
            area: 0,
            grid,
            raw: [0; EntityRecord::FIELDS],
        }
    };
    let records = [
        bogus(EntityKind::VehiclePart, 400, &[0, -1_500, 100, 0, 200, 0]),
        bogus(EntityKind::Vehicle, 9, &[0, -1_400, 0, 0]),
        bogus(EntityKind::Door, 99, &[0]),
        bogus(EntityKind::Wall, 3, &[500, 0]),
        bogus(EntityKind::Soldier, 7, &[0, -1_400, 0, 0, 0, 0, 100, 0]),
    ];
    let scene = Scene::course();
    let (mut a, mut b) = (World::new(&scene, 1, 0), World::new(&scene, 1, 0));
    a.load_movers(&records);
    let (mut ha, mut hb) = (ring(), ring());
    for k in 0..40u32 {
        let c = Command {
            slot: 0,
            tick: k,
            seq: k as u16,
            buttons: Command::FIRE,
            move_xy: [0, 127],
            aim: [0, -500, 140],
            ..Command::default()
        };
        let (mut pa, mut pb) = (a.clone(), b.clone());
        assert_eq!(pa.predict(&c), pb.predict(&c), "step {k}: predicted");
        assert_eq!(
            pa.entity_records(),
            pb.entity_records(),
            "step {k}: predicted records"
        );
        let t = input(&a, vec![(c, None)]);
        room_tick(&mut a, &mut ha, &t);
        room_tick(&mut b, &mut hb, &t);
        assert_eq!(a.hit_volumes(), b.hit_volumes(), "step {k}: volumes");
        assert_eq!(a.hit_volumes().cover.len(), 7, "step {k}: CO's cover");
        assert_eq!(a.state_hash(), b.state_hash(), "step {k}: state");
        a.load_movers(&records);
    }
    let mut v = HitVolumes::default();
    a.hit_volumes_from(&records, 0, &mut v);
    assert_eq!((v.soldiers.len(), v.cover.len()), (1, 7));
    assert!(v.soldiers.iter().chain(&v.cover).all(|b| !b.live), "{v:?}");
}

#[test]
fn t_m1_the_estimate_scene_meets_pm_0001_2s_es_bands() {
    // The advisor's W1-U4D T-M1 (the test review's M1; PM-0001-2's ES checks, marks.json; ADR 0001:1474-1479): over
    // ES's synthetic recording (seed 2, 9,000 ticks: tests/replay.rs's), every tick has 48 soldiers and 15 vehicles,
    // and the run's means lie in PM-0001-2's bands exactly: 72 shots a second (68.4-75.6), 10 gameplay events (hits and
    // kills) a second (9.5-10.5), 32 live projectiles a tick (30.4-33.6) and 21.12 spawns a second (20.064-22.176).
    // Thrown grenades count toward both projectile bands; the stand-in spawner fills the rest, and every projectile
    // fuses after 1.515 s. The events come from the recording's aim (one command in 11 among the other teams, the rest
    // into the air) and from the blasts.
    let scene = Scene::estimate();
    let ticks = 9_000u32;
    let seconds = u64::from(ticks / TICK_HZ);
    let rec = Recording::synthetic(&scene, 2, 1_000_000, ticks, SPAN_MS);
    let mut world = World::new(&scene, 2, 1_000_000);
    let mut history = ring();
    let (mut shots, mut events, mut live, mut spawns, mut thrown) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut min_soldiers, mut min_vehicles) = (usize::MAX, usize::MAX);
    let mut before = std::collections::BTreeSet::new();
    for t in &rec.ticks {
        let e = room_tick(&mut world, &mut history, t);
        shots += e
            .iter()
            .filter(|e| matches!(e, SimEvent::Shot { .. }))
            .count() as u64;
        events += e
            .iter()
            .filter(|e| matches!(e, SimEvent::Hit { .. } | SimEvent::Kill { .. }))
            .count() as u64;
        let projectiles = records_of(&world, EntityKind::Projectile);
        live += projectiles.len() as u64;
        let now: std::collections::BTreeSet<(u16, u16)> = projectiles
            .iter()
            .map(|p| (p.index, p.generation))
            .collect();
        for p in &projectiles {
            if !before.contains(&(p.index, p.generation)) {
                spawns += 1;
                thrown += u64::from(p.grid[7] == 1);
            }
        }
        before = now;
        min_soldiers = min_soldiers.min(records_of(&world, EntityKind::Soldier).len());
        min_vehicles = min_vehicles.min(records_of(&world, EntityKind::Vehicle).len());
    }
    let per_s = |n: u64| n * 1_000 / seconds;
    let (shots_m, events_m, spawns_m) = (per_s(shots), per_s(events), per_s(spawns));
    let live_m = live * 1_000 / u64::from(ticks);
    println!(
        "T-M1: ES over {seconds} s: shots {shots_m}, gameplay events {events_m}, spawns {spawns_m} ({thrown} \
         thrown) thousandths a second; live projectiles {live_m} thousandths a tick"
    );
    assert!(
        min_soldiers >= 48 && min_vehicles >= 15,
        "{min_soldiers} soldiers, {min_vehicles} vehicles"
    );
    assert!((68_400..=75_600).contains(&shots_m), "shots: {shots_m}");
    assert!(
        (9_500..=10_500).contains(&events_m),
        "gameplay events: {events_m}"
    );
    assert!(
        (30_400..=33_600).contains(&live_m),
        "live projectiles: {live_m}"
    );
    assert!((20_064..=22_176).contains(&spawns_m), "spawns: {spawns_m}");
}

/// The course's one soldier spawned at `feet` (cm), and the fire origin a still command fires from, m.
fn still_soldier(feet: [i32; 3]) -> (World, HitHistory, [f64; 3]) {
    let scene = Scene::course();
    let mut world = World::new(&scene, 9, 0);
    let mut history = ring();
    let t = TickInput {
        tick: 0,
        spawns: vec![Placement {
            slot: 0,
            position: feet,
        }],
        ..TickInput::default()
    };
    room_tick(&mut world, &mut history, &t);
    let wire = Config::default();
    let origin = [
        f64::from(wire.from_grid(POSITION_XY, feet[0])),
        f64::from(wire.from_grid(POSITION_XY, feet[1])),
        f64::from(wire.from_grid(POSITION_Z, feet[2]) + CHEST),
    ];
    (world, history, origin)
}

/// A point's distance from `o`, m, as the clamp computes it (f64, from integer cm).
fn dist(o: [f64; 3], p: [i32; 3]) -> f64 {
    let d = [
        f64::from(p[0]) / 100.0 - o[0],
        f64::from(p[1]) / 100.0 - o[1],
        f64::from(p[2]) / 100.0 - o[2],
    ];
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

/// One still, firing command of slot 0 aimed at `aim`; returns the clamp reports of that tick.
fn aim_at(world: &mut World, history: &mut HitHistory, k: u32, aim: [i32; 3]) -> Vec<[i32; 3]> {
    let c = Command {
        slot: 0,
        tick: world.next_tick(),
        seq: k as u16,
        buttons: Command::FIRE,
        aim,
        ..Command::default()
    };
    let t = input(world, vec![(c, None)]);
    room_tick(world, history, &t);
    world.reports().clamps.iter().map(|r| r.point).collect()
}

#[test]
fn d8_the_clamp_moves_a_far_point_to_the_reach_and_never_again() {
    // u4-build-plan.md §5 U4d's D-8, for every fire origin below the bound World's doc states (no scene places one
    // above it: world.rs's test); above it a reach point saturates in z and is not claimed to lie at the reach (the
    // advisor's W1-U4D TUNE-2). The bound: a fire origin at or below the grid's top, 40.95 m (feet at or below 39.55 m),
    // for which a reach point toward any aim point in the grid's range is in that range too. First from feet on the
    // ground, with far points in every direction (some outside the grid's range, as no wire aim is); then from feet
    // at the bound, with far points inside the grid's range, as an aim from the wire is.
    let feet = [0, -1_500, 0];
    let (mut world, mut history, o) = still_soldier(feet);
    let reach = REACH_CM / 100.0;
    let oc = [0, -1_500, 140];
    let mut dice = Dice(0x5eed);
    let mut k = 0u32;
    let mut on_reach = 0;
    for case in 0..400 {
        // A far point in a random direction: 1.01 to 1,000 reaches away.
        let scale = [1.01, 2.0, 30.0, 1_000.0][case % 4];
        let d = [
            dice.between(-1_000, 1_000),
            dice.between(-1_000, 1_000),
            dice.between(-1_000, 1_000),
        ];
        let n = f64::from(d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if n < 1.0 {
            continue;
        }
        let far = d.map(|x| (f64::from(x) / n * REACH_CM * scale) as i32);
        let aim = [oc[0] + far[0], oc[1] + far[1], oc[2] + far[2]];
        let reports = aim_at(&mut world, &mut history, k, aim);
        k += 1;
        assert_eq!(reports.len(), 1, "case {case}: {aim:?} is beyond the reach");
        let p = reports[0];
        // The point at the reach along the aim's own ray, m.
        let along = [
            f64::from(aim[0]) / 100.0 - o[0],
            f64::from(aim[1]) / 100.0 - o[1],
            f64::from(aim[2]) / 100.0 - o[2],
        ];
        let len = dist(o, aim);
        let on = [
            o[0] + along[0] / len * reach,
            o[1] + along[1] / len * reach,
            o[2] + along[2] / len * reach,
        ];
        // At the reach on the grid, where the reach point lies inside the grid's ranges (z from 0 to 40.95 m; outside
        // them the grid saturates the point, as it does every position): within one cell's half-diagonal of the reach,
        // and of the ray.
        if (0.01..40.9).contains(&on[2]) {
            on_reach += 1;
            let r = dist(o, p);
            assert!((r - reach).abs() <= SLACK_M, "case {case}: {p:?} at {r} m");
            let off = [0, 1, 2].map(|a| f64::from(p[a]) / 100.0 - on[a]);
            let off = (off[0] * off[0] + off[1] * off[1] + off[2] * off[2]).sqrt();
            assert!(off <= SLACK_M, "case {case}: {p:?} is {off} m off the ray");
        } else {
            assert!(
                p[2] == 0 || p[2] == 4_095,
                "case {case}: {p:?} saturated in z"
            );
        }
        // Sent again, the clamped point is not moved (clamp-then-grid is idempotent: §4.10).
        assert!(
            aim_at(&mut world, &mut history, k, p).is_empty(),
            "case {case}: {p:?} clamped twice"
        );
        k += 1;
    }
    assert!(
        on_reach > 80,
        "{on_reach} cases with the reach point in range"
    );
    // At the boundary: every integer point within reach + 0.87 cm is untouched, every one beyond it is clamped.
    let (mut inside, mut beyond) = (0, 0);
    for case in 0..600 {
        let d = [
            dice.between(-1_000, 1_000),
            dice.between(-1_000, 1_000),
            dice.between(-1_000, 1_000),
        ];
        let n = f64::from(d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if n < 1.0 {
            continue;
        }
        let out = REACH_CM + f64::from(dice.between(-300, 300)) / 100.0;
        let aim = [0, 1, 2].map(|a| oc[a] + (f64::from(d[a]) / n * out).round() as i32);
        let r = dist(o, aim);
        if (r - (reach + SLACK_M)).abs() < 1e-9 {
            continue;
        }
        let reports = aim_at(&mut world, &mut history, k, aim);
        k += 1;
        if r <= reach + SLACK_M {
            inside += 1;
            assert!(reports.is_empty(), "case {case}: {aim:?} at {r} m clamped");
        } else {
            beyond += 1;
            assert_eq!(
                reports.len(),
                1,
                "case {case}: {aim:?} at {r} m not clamped"
            );
        }
    }
    assert!(
        inside > 100 && beyond > 100,
        "{inside} inside, {beyond} beyond"
    );
    // The soldier never moved: every shot left from the same origin.
    assert_eq!(&soldier(&world, 0).grid[..3], &feet);
    // At the bound: feet at 39.55 m, the fire origin at the grid's top. Every far aim point inside the grid's range
    // (z 0 to 40.95 m) is clamped to the reach and, sent again, not moved.
    let top = [0, -1_500, 3_955];
    let (mut world, mut history, o) = still_soldier(top);
    let oc = [top[0], top[1], top[2] + 140];
    for case in 0..400 {
        let d = [
            dice.between(-1_000, 1_000),
            dice.between(-1_000, 1_000),
            dice.between(-1_000, 0),
        ];
        let n = f64::from(d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if n < 1.0 {
            continue;
        }
        let scale = [1.01, 1.5, 2.0, 3.0][case % 4];
        let far = d.map(|x| (f64::from(x) / n * REACH_CM * scale) as i32);
        let aim = [
            oc[0] + far[0],
            oc[1] + far[1],
            (oc[2] + far[2]).clamp(0, 4_095),
        ];
        if dist(o, aim) <= reach * 1.001 {
            continue;
        }
        let reports = aim_at(&mut world, &mut history, k, aim);
        k += 1;
        assert_eq!(reports.len(), 1, "at the bound, case {case}: {aim:?}");
        let p = reports[0];
        let r = dist(o, p);
        assert!(
            (r - reach).abs() <= SLACK_M,
            "at the bound, case {case}: {p:?} at {r} m"
        );
        assert!(
            aim_at(&mut world, &mut history, k, p).is_empty(),
            "at the bound, case {case}: {p:?} clamped twice"
        );
        k += 1;
    }
    assert_eq!(&soldier(&world, 0).grid[..3], &top);
}

/// The target course's shooter spawned at (-60 m, -10 m): its world, and its fire origin, m.
fn shooter() -> (World, [f32; 3]) {
    let mut world = World::new(&Scene::targets(), 21, 500);
    let t = TickInput {
        tick: 500,
        spawns: vec![Placement {
            slot: 0,
            position: [-6_000, -1_000, 0],
        }],
        ..TickInput::default()
    };
    world.step(&t, &ring());
    (world, [-60.0, -10.0, CHEST])
}

/// A ring holding ticks 99, 100 and 101, with slot 1's box 2 m ahead of the shooter (5 cm wide, 1 m tall, at chest
/// height), moved `dy` cm along y from tick 100 to tick 101, and `cover` as each tick's cover boxes.
fn moving_target(o: [f32; 3], dy: i32, cover: Vec<VolumeBox>) -> HitHistory {
    let mut history = ring();
    let c = [
        (o[0] * 100.0) as i32 + 200,
        (o[1] * 100.0) as i32,
        (o[2] * 100.0) as i32,
    ];
    for (tick, y) in [(99, 0), (100, 0), (101, dy)] {
        let target = VolumeBox {
            center: [c[0], c[1] + y, c[2]],
            half: [3, 3, 50],
            live: true,
        };
        let away = VolumeBox::default();
        history.push(&HitVolumes {
            tick,
            soldiers: vec![away, target, away, away, away],
            cover: cover.clone(),
        });
    }
    history
}

/// Fires slot 0 at `aim` (cm) with `rewind`: whether the shot hit slot 1, and the world after.
fn shoot(history: &HitHistory, aim: [i32; 3], rewind: Option<Rewind>) -> (bool, World) {
    let (mut world, _) = shooter();
    let c = Command {
        slot: 0,
        tick: world.next_tick(),
        seq: 1,
        buttons: Command::FIRE,
        aim,
        ..Command::default()
    };
    let t = input(&world, vec![(c, rewind)]);
    let events = world.step(&t, history).to_vec();
    assert!(
        events.contains(&SimEvent::Shot { slot: 0, seq: 1 }),
        "the shot ran"
    );
    let hit = events
        .iter()
        .any(|e| matches!(e, SimEvent::Hit { victim: 1, .. }));
    (hit, world)
}

#[test]
fn d9_a_rewind_blends_tick_a_and_tick_a_plus_one_by_its_fraction() {
    // Slot 1 moves 256 cm along y from tick 100 to 101, so at fraction f (in 256ths) the blend puts it f cm along: the
    // reference, a + (b − a) × f / 256, as the client's Interp draws it. A shot aimed at the reference hits; one aimed
    // at tick 100's place misses once the blend has moved the 6 cm box past it (the spread is about ±1 cm at 2 m).
    let (_, o) = shooter();
    let history = moving_target(o, 256, Vec::new());
    let base = [
        (o[0] * 100.0) as i32 + 200,
        (o[1] * 100.0) as i32,
        (o[2] * 100.0) as i32,
    ];
    for f in [0u8, 1, 32, 64, 100, 128, 192, 255] {
        let r = Rewind {
            tick_a: 100,
            fraction: f,
            cover: CoverVariant::Rewound,
        };
        let reference = [base[0], base[1] + i32::from(f), base[2]];
        let (hit, world) = shoot(&history, reference, Some(r));
        assert!(hit, "fraction {f}: the shot at the reference blend hits");
        assert_eq!(world.reports().rewind_outside, 0);
        // The reference itself, cast through World::ray_cast: it puts slot 1 where the blend did.
        let mut movers = history.get(100).expect("held").clone();
        movers.soldiers[1].center[1] += i32::from(f);
        let ray = Ray {
            origin: o,
            to: [o[0] + 4.0, o[1] + f32::from(f) / 50.0, o[2]],
            skip: Some(0),
        };
        assert!(
            matches!(
                world.ray_cast(&ray, &movers),
                RayHit::Soldier { slot: 1, .. }
            ),
            "fraction {f}: the reference volumes put slot 1 on the ray"
        );
        let (hit, _) = shoot(&history, base, Some(r));
        assert_eq!(hit, f < 4, "fraction {f}: the shot at tick 100's place");
    }
    // The cover variants: a box between shooter and target in the ring blocks a Rewound shot; a Present one takes the
    // cover as it is now (the course's doors and wall, far away) and hits.
    let wall = VolumeBox {
        center: [base[0] - 100, base[1], base[2]],
        half: [2, 50, 50],
        live: true,
    };
    let history = moving_target(o, 0, vec![wall]);
    for (cover, hits) in [
        (CoverVariant::Rewound, false),
        (CoverVariant::Present, true),
    ] {
        let r = Rewind {
            tick_a: 100,
            fraction: 128,
            cover,
        };
        assert_eq!(shoot(&history, base, Some(r)).0, hits, "{cover:?}");
    }
}

#[test]
fn d9_a_rewind_outside_the_ring_is_refused_and_counted() {
    // The ring holds 99, 100 and 101. Refused, counted in reports() and run as None (carry 10): tick_a not held;
    // fraction > 0 with tick_a held and tick_a + 1 not held; tick_a not held though tick_a + 1 is. Accepted: fraction 0
    // at the newest tick (only tick_a is read), any fraction between held ticks.
    let (_, o) = shooter();
    let history = moving_target(o, 256, Vec::new());
    let aim = [
        (o[0] * 100.0) as i32 + 200,
        (o[1] * 100.0) as i32 + 128,
        (o[2] * 100.0) as i32,
    ];
    let rewind = |tick_a, fraction| {
        Some(Rewind {
            tick_a,
            fraction,
            cover: CoverVariant::Rewound,
        })
    };
    let (_, none) = shoot(&history, aim, None);
    for (tick_a, fraction, refused) in [
        (98, 0, true),
        (102, 0, true),
        (101, 1, true),
        (101, 255, true),
        (98, 128, true),
        (u32::MAX, 0, true),
        (101, 0, false),
        (100, 255, false),
        (99, 128, false),
    ] {
        let (_, world) = shoot(&history, aim, rewind(tick_a, fraction));
        let at = format!("tick_a {tick_a}, fraction {fraction}");
        assert_eq!(
            world.reports().rewind_outside,
            u64::from(refused),
            "{at}: counted"
        );
        if refused {
            // One rule for None and a refused Rewind: the same state, the same events.
            assert_eq!(world.state_hash(), none.state_hash(), "{at}: as None");
        }
    }
}

/// A course stream of `n` steps for slot 0: still, aimed 10 m north, with `buttons` held at each step and the press
/// counters advanced by `press` at each step (index, steps).
fn course_stream(n: u32, buttons: impl Fn(u32) -> u16, press: &[(usize, u32)]) -> Vec<TickInput> {
    let mut presses = [0u8; 4];
    (0..n)
        .map(|k| {
            for &(i, at) in press {
                if at == k {
                    presses[i] = presses[i].wrapping_add(1);
                }
            }
            let c = Command {
                slot: 0,
                tick: k,
                seq: k as u16,
                buttons: buttons(k),
                presses,
                move_xy: [0, 0],
                aim: [0, -500, 140],
                ..Command::default()
            };
            TickInput {
                tick: k,
                commands: vec![(c, None)],
                ..TickInput::default()
            }
        })
        .collect()
}

/// The steps of `world`'s run at which slot 0 fired: its Shot events.
fn shots_of(stream: &[TickInput]) -> Vec<usize> {
    let scene = Scene::course();
    let mut world = World::new(&scene, 5, stream[0].tick);
    let mut history = ring();
    stream
        .iter()
        .enumerate()
        .filter(|(_, t)| {
            room_tick(&mut world, &mut history, t)
                .iter()
                .any(|e| matches!(e, SimEvent::Shot { slot: 0, .. }))
        })
        .map(|(k, _)| k)
        .collect()
}

#[test]
fn d10_a_fire_press_in_the_cooldown_leaves_no_trace() {
    // The course's weapon fires every 3 ticks: a press at 10 fires, the press at 11 is in the cooldown (refused,
    // counted), the next press at 20 fires. The twin is the forged stream with the press at 11 removed.
    let forged = course_stream(60, |_| 0, &[(0, 10), (0, 11), (0, 20)]);
    let twin = course_stream(60, |_| 0, &[(0, 10), (0, 20)]);
    assert_eq!(shots_of(&forged), [10, 20], "the presses at 10 and 20 fire");
    let (rf, rt, _) = twins(&Scene::course(), &forged, &twin, |_, _| {});
    assert_eq!((rf.cooldown, rt.cooldown), (1, 0));
    assert_eq!(rf.clamps, rt.clamps);
    // The refused command's record: what the twin removed (the advisor's W1-U4D ruling).
    assert_eq!(rf.refusals, [refusal(11, RefusalKind::Cooldown)]);
    assert!(rt.refusals.is_empty());
}

/// Slot 0's refusal of its command `seq`.
fn refusal(seq: u16, kind: RefusalKind) -> Refusal {
    Refusal { slot: 0, seq, kind }
}

#[test]
fn d10_a_fire_press_with_no_ammunition_leaves_no_trace() {
    // Fire held from 0 to 94 empties the 30-round magazine (a shot every 3 ticks); the press at 100 finds it empty
    // (refused, counted); the reload press at 105 runs 2 s; the press at 170 fires.
    let held = |k: u32| if k < 95 { Command::FIRE } else { 0 };
    let forged = course_stream(200, held, &[(0, 100), (1, 105), (0, 170)]);
    let twin = course_stream(200, held, &[(1, 105), (0, 170)]);
    assert!(
        shots_of(&forged).ends_with(&[170]),
        "the press at 170 fires"
    );
    let (rf, rt, _) = twins(&Scene::course(), &forged, &twin, |_, _| {});
    assert_eq!((rf.ammunition, rt.ammunition), (1, 0));
    assert_eq!(rf.refusals, [refusal(100, RefusalKind::Ammunition)]);
    assert!(rt.refusals.is_empty());
}

#[test]
fn d10_a_reload_press_the_weapon_refuses_leaves_no_trace() {
    // A reload press at 5 with the magazine full is refused and counted; fire held from 10 to 39 spends ten rounds; the
    // reload press at 50 runs.
    let held = |k: u32| {
        if (10..40).contains(&k) {
            Command::FIRE
        } else {
            0
        }
    };
    let forged = course_stream(140, held, &[(1, 5), (1, 50)]);
    let twin = course_stream(140, held, &[(1, 50)]);
    let (rf, rt, world) = twins(&Scene::course(), &forged, &twin, |k, w| {
        // Reloading from 50 to 110 (reload left above 0 in the own-state record, and the remote row's reloading flag,
        // bit 2), and not before.
        let (left, flags) = (own_state(w, 0).grid[5], soldier(w, 0).grid[5]);
        let reloading = (50..110).contains(&k);
        assert_eq!(left > 0, reloading, "step {k}: reload left {left}");
        assert_eq!(flags & 1 << 2 != 0, reloading, "step {k}: flags {flags:#b}");
    });
    assert_eq!((rf.reload, rt.reload), (1, 0));
    assert_eq!(rf.refusals, [refusal(5, RefusalKind::Reload)]);
    assert!(rt.refusals.is_empty());
    let own = own_state(&world, 0).grid;
    assert_eq!(
        (own[3], own[5]),
        (30, 0),
        "reloaded: a full magazine, no reload left"
    );
}

#[test]
fn d10_a_clamped_aim_point_leaves_no_trace() {
    // The forged command at step 10 aims 3 km away; its twin sends the clamped point the forged run's report gives, at
    // the same instant (§4.10). A fire press at 30 fires in both.
    let far = [300_000, -500, 140];
    let make = |aim10: [i32; 3]| {
        let mut s = course_stream(60, |k| if k == 10 { Command::FIRE } else { 0 }, &[(0, 30)]);
        s[10].commands[0].0.aim = aim10;
        s
    };
    let forged = make(far);
    let mut world = World::new(&Scene::course(), 5, 0);
    let mut history = ring();
    let mut clamped = None;
    for t in &forged {
        room_tick(&mut world, &mut history, t);
        if let Some(r) = world.reports().clamps.first() {
            clamped = Some(r.point);
        }
    }
    let point = clamped.expect("the far point is clamped");
    let twin = make(point);
    let (rf, rt, _) = twins(&Scene::course(), &forged, &twin, |_, _| {});
    assert_eq!(rf.clamps.len(), 1, "the forged run's one clamp report");
    assert_eq!((rf.clamps[0].seq, rf.clamps[0].point), (10, point));
    assert!(
        rt.clamps.is_empty(),
        "the twin's point is not clamped again"
    );
    assert_eq!(
        [
            rf.cooldown,
            rf.ammunition,
            rf.reload,
            rf.rewind_outside,
            rf.pool_full
        ],
        [0; 5],
        "a clamp is never a refusal"
    );
    assert!(rf.refusals.is_empty() && rt.refusals.is_empty());
}

#[test]
fn d10_a_rewind_outside_the_ring_leaves_no_trace() {
    // Fire held from 10 to 40, every command carrying a Rewind; the forged run's at 15 names a future tick and at 25
    // blends toward one (refused, counted); its twin has None there. A press at 50 fires in both.
    let held = |k: u32| {
        if (10..=40).contains(&k) {
            Command::FIRE
        } else {
            0
        }
    };
    let mut forged = course_stream(80, held, &[(0, 50)]);
    for (k, t) in forged.iter_mut().enumerate() {
        let k = k as u32;
        if (10..=40).contains(&k) {
            t.commands[0].1 = Some(Rewind {
                tick_a: k - 3,
                fraction: 77,
                cover: CoverVariant::Rewound,
            });
        }
    }
    let mut twin = forged.clone();
    forged[15].commands[0].1 = Some(Rewind {
        tick_a: 20,
        fraction: 0,
        cover: CoverVariant::Present,
    });
    forged[25].commands[0].1 = Some(Rewind {
        tick_a: 24,
        fraction: 1,
        cover: CoverVariant::Rewound,
    });
    twin[15].commands[0].1 = None;
    twin[25].commands[0].1 = None;
    let (rf, rt, _) = twins(&Scene::course(), &forged, &twin, |_, _| {});
    assert_eq!((rf.rewind_outside, rt.rewind_outside), (2, 0));
    assert_eq!(
        rf.refusals,
        [
            refusal(15, RefusalKind::RewindOutside),
            refusal(25, RefusalKind::RewindOutside)
        ]
    );
    assert!(rt.refusals.is_empty());
}

#[test]
fn carry7_tick_view_tick_and_view_fraction_never_reach_the_state() {
    // Two streams that differ only in each command's tick, view_tick and view_fraction (equal Rewinds) step to equal
    // state hashes on every tick (netcode F1 as the U4s ruling worded it). The ticks move by one offset per tick, so the
    // canonical order holds in both.
    for (scene, seed, first) in scenes() {
        let rec = Recording::synthetic(&scene, seed, first, 300, SPAN_MS);
        let mut dice = Dice(0xfeed ^ seed as u32);
        let mut other = rec.clone();
        for t in &mut other.ticks {
            let shift = dice.next();
            for (c, _) in &mut t.commands {
                c.tick = c.tick.wrapping_add(shift);
                c.view_tick = dice.next();
                c.view_fraction = dice.below(256) as u8;
            }
        }
        let (mut a, mut b) = (
            World::new(&scene, seed, first),
            World::new(&scene, seed, first),
        );
        let (mut ha, mut hb) = (ring(), ring());
        let differing = rec
            .ticks
            .iter()
            .zip(&other.ticks)
            .filter(|(x, y)| x != y)
            .count();
        assert!(
            differing > 250,
            "{}: {differing} ticks differ",
            scene.name()
        );
        for (k, (x, y)) in rec.ticks.iter().zip(&other.ticks).enumerate() {
            let (ea, eb) = (room_tick(&mut a, &mut ha, x), room_tick(&mut b, &mut hb, y));
            assert_eq!(ea, eb, "{} tick {k}", scene.name());
            assert_eq!(a.state_hash(), b.state_hash(), "{} tick {k}", scene.name());
            assert_eq!(a.reports(), b.reports(), "{} tick {k}", scene.name());
        }
    }
}

#[test]
#[cfg(debug_assertions)]
fn carry8_an_out_of_order_tick_input_panics_in_a_debug_build() {
    // Netcode F2: slot ascending; within a slot, command tick strictly ascending, wrap-aware (b after a when
    // b.wrapping_sub(a) as i32 > 0).
    let c = |slot, tick| {
        (
            Command {
                slot,
                tick,
                ..Command::default()
            },
            None,
        )
    };
    let cases = [
        (vec![c(1, 5), c(0, 6)], false),
        (vec![c(0, 5), c(0, 5)], false),
        (vec![c(0, 6), c(0, 5)], false),
        (vec![c(0, 0), c(0, u32::MAX)], false),
        (vec![c(0, 0), c(0, 1 << 31)], false),
        (vec![c(0, u32::MAX), c(0, 0)], true),
        (vec![c(0, 5), c(0, 9), c(1, 2), c(1, 3)], true),
        (vec![c(0, 1), c(0, (1 << 31) - 1)], true),
        (vec![c(2, 0), c(5, 0)], true),
    ];
    for (i, (commands, ordered)) in cases.into_iter().enumerate() {
        let mut world = World::new(&Scene::stress(), 1, 0);
        let t = TickInput {
            tick: 0,
            commands,
            ..TickInput::default()
        };
        let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            world.step(&t, &ring());
        }));
        match run {
            Ok(()) => assert!(ordered, "case {i}: an out-of-order input stepped"),
            Err(why) => {
                let text = why
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| why.downcast_ref::<&str>().map(|s| (*s).to_string()))
                    .unwrap_or_default();
                assert!(!ordered, "case {i}: an ordered input panicked: {text}");
                assert!(text.contains("canonical order"), "case {i}: {text}");
            }
        }
    }
}

#[test]
fn carry9_each_record_holds_its_pre_snap_value_and_the_wire_grids_integer_of_it() {
    // Netcode F4: raw is each float field's value before the own-copy snap (f32 bits), grid is the wire grid's
    // integer of that value, and the snapped copy the state holds is from_grid of that integer (a clean build). The
    // raw value is the pre-snap one: on moving ticks it is off the grid.
    let wire = Config::default();
    let scene = Scene::stress();
    let rec = Recording::synthetic(&scene, 4, 0, 300, SPAN_MS);
    let mut world = World::new(&scene, 4, 0);
    let mut history = ring();
    // The float fields of a soldier's two records, by the state word each snaps: its soldier record's position, yaw
    // and pitch, its own-state record's velocity (W1-S1).
    let names = [
        "pos.x", "pos.y", "pos.z", "yaw", "pitch", "vel.x", "vel.y", "vel.z",
    ];
    let first_name = |kind: EntityKind| match kind {
        EntityKind::Soldier => Some(0),
        EntityKind::OwnState => Some(5),
        _ => None,
    };
    let (mut off_grid, mut floats) = ([0u32; 8], 0u32);
    for (k, t) in rec.ticks.iter().enumerate() {
        room_tick(&mut world, &mut history, t);
        let dump = world.dump();
        for r in world.entity_records() {
            for (i, f) in r.kind.layout().iter().enumerate() {
                if is_integer(*f) {
                    assert_eq!(r.raw[i], 0, "tick {k} {r:?}: an integer field's raw");
                    continue;
                }
                floats += 1;
                let x = f32::from_bits(r.raw[i]);
                assert_eq!(r.grid[i], wire.to_grid(*f, x), "tick {k} {r:?}: field {i}");
            }
            for i in r.kind.layout().len()..EntityRecord::FIELDS {
                assert_eq!((r.grid[i], r.raw[i]), (0, 0), "tick {k}: past the layout");
            }
            if let Some(first) = first_name(r.kind) {
                let floats = r.kind.layout().iter().take_while(|f| !is_integer(**f));
                for (i, field) in floats.enumerate() {
                    let n = first + i;
                    let name = names[n];
                    let held = word(&dump, &format!("soldier[{}].{name}", r.index));
                    let sent = wire.from_grid(*field, r.grid[i]).to_bits();
                    assert_eq!(held, sent, "tick {k}: {:?} {} {name}", r.kind, r.index);
                    off_grid[n] += u32::from(r.raw[i] != held);
                }
            }
        }
    }
    assert!(floats > 100_000, "{floats} float fields read");
    // Each field a moving soldier changes has raw values off the grid, so each is taken before the snap (height and
    // vertical speed stay 0 on flat ground, always on the grid).
    for (i, name) in names.iter().enumerate() {
        let moves = !matches!(*name, "pos.z" | "vel.z");
        assert_eq!(
            off_grid[i] > 1_000,
            moves,
            "{name}: {} raw values off the grid",
            off_grid[i]
        );
    }
}

/// Whether a layout field is one of `Field::integer`'s (not a grid field).
fn is_integer(f: Field) -> bool {
    ![POSITION_XY, POSITION_Z, YAW, PITCH, VELOCITY].contains(&f)
}

#[test]
fn w1_s1_the_own_state_record_holds_the_soldiers_weapon_and_attributes_exactly() {
    // The advisor's W1-S1: each soldier's own-state record holds what decides its next ticks and is not in its remote
    // row: its ammunition, fire timer, reload left, move speed and fire interval, each exactly as the state holds it
    // (U4g's reset sets a predicting world's soldier from them), on every tick of a course run (reloads, refusals,
    // spawns and attribute changes), of a stress run and of the extreme script's attributes (u32::MAX, 0: brought into
    // the fields' ranges). Its index is the slot and its generation the soldier's; it comes last, in slot order.
    let mut runs: Vec<(Scene, Recording)> = [Scene::course(), Scene::stress()]
        .into_iter()
        .map(|s| {
            let rec = Recording::synthetic(&s, 31, 0, 1_200, SPAN_MS);
            (s, rec)
        })
        .collect();
    // Extreme attributes, every few ticks, on the course.
    let mut extreme = runs[0].1.clone();
    for (k, t) in extreme.ticks.iter_mut().enumerate() {
        let value = [u32::MAX, 0, 1, 10_000_001][k % 4];
        let attribute = [Attribute::MoveSpeed, Attribute::FireInterval][k / 4 % 2];
        t.attributes.push(AttributeChange {
            slot: 0,
            attribute,
            value,
        });
    }
    runs.push((Scene::course(), extreme));
    let mut seen = [0u64; 3];
    for (scene, rec) in &runs {
        let mut world = World::new(scene, 31, 0);
        let mut history = ring();
        let slots = usize::from(scene.slots());
        // Each soldier's state words, found by name once (the soldiers' section leads the dump, so they keep their
        // places; the names are checked again at every tick).
        let fields = [
            "generation",
            "ammo",
            "wait",
            "reload_left",
            "move_speed",
            "fire_interval",
        ];
        let first = world.dump();
        let at_of = |slot: usize, f: &str| {
            let name = format!("soldier[{slot}].{f}");
            (0..first.words().len())
                .find(|&i| first.field(i).as_deref() == Some(name.as_str()))
                .unwrap_or_else(|| panic!("the dump has no {name}"))
        };
        let places: Vec<Vec<usize>> = (0..slots)
            .map(|s| fields.iter().map(|f| at_of(s, f)).collect())
            .collect();
        for (k, t) in rec.ticks.iter().enumerate() {
            room_tick(&mut world, &mut history, t);
            let d = world.dump();
            let records = world.entity_records();
            let own = &records[records.len() - slots..];
            for (slot, r) in own.iter().enumerate() {
                let at = format!("{} tick {k} slot {slot}", scene.name());
                assert_eq!(
                    (r.kind, usize::from(r.index)),
                    (EntityKind::OwnState, slot),
                    "{at}"
                );
                let p = &places[slot];
                let w = |i: usize| d.words()[p[i]];
                let wide =
                    |i: usize| (u64::from(w(i)) | u64::from(d.words()[p[i] + 1]) << 32) as i64;
                assert_eq!(
                    d.field(p[2] + 1),
                    Some(format!("soldier[{slot}].wait")),
                    "{at}"
                );
                assert_eq!(u32::from(r.generation), w(0), "{at}: generation");
                assert_eq!(r.grid[3] as u32, w(1), "{at}: ammunition");
                assert_eq!(i64::from(r.grid[4]), wide(2), "{at}: fire timer");
                assert_eq!(i64::from(r.grid[5]), wide(3), "{at}: reload left");
                assert_eq!(r.grid[6] as u32, w(4), "{at}: move speed");
                assert_eq!(r.grid[7] as u32, w(5), "{at}: fire interval");
                for (f, g) in r.kind.layout().iter().zip(r.grid) {
                    assert!((f.min()..=f.max()).contains(&g), "{at}: {g} in {f:?}");
                }
                seen[0] += u64::from(r.grid[5] > 0);
                seen[1] += u64::from(r.grid[4] > 0);
                seen[2] += u64::from(r.grid[6] == 10_000 || r.grid[7] == 10_000_000);
            }
        }
    }
    // Not vacuous: reloads ran, the fire timer cooled down, and the extreme attributes were brought to their tops.
    assert!(seen.iter().all(|n| *n > 10), "{seen:?}");
}

#[test]
fn carry13_a_push_into_a_full_hit_history_reuses_the_evicted_entrys_storage() {
    // Rust 2: HitVolumes's hand-written clone_from copies into the evicted entry's vectors, so a push into a full ring
    // allocates no buffer. A counting allocator cannot be written in npt-sim (the workspace forbids unsafe_code on every
    // target, and GlobalAlloc is an unsafe trait), so this shows it by storage identity: after each push the newest
    // entry's buffers are the evicted entry's own. A derived clone_from (`*self = source.clone()`) clones before it
    // drops the old value, so its buffers are always new ones: the control below shows the test tells the two apart.
    let scene = Scene::stress();
    let rec = Recording::synthetic(&scene, 2, 0, 80, SPAN_MS);
    let mut world = World::new(&scene, 2, 0);
    let mut history = HitHistory::for_ms(66).expect("2 ticks");
    assert_eq!(history.capacity(), 2);
    for (k, t) in rec.ticks.iter().enumerate() {
        world.step(t, &history);
        let tick = world.hit_volumes().tick;
        let evicted = history.get(tick.wrapping_sub(2)).map(|v| {
            (
                v.soldiers.as_ptr(),
                v.soldiers.capacity(),
                v.cover.as_ptr(),
                v.cover.capacity(),
            )
        });
        history.push(world.hit_volumes());
        let newest = history.get(tick).expect("the newest is held");
        assert_eq!(newest, world.hit_volumes(), "tick {k}: copied");
        if let Some((soldiers, sc, cover, cc)) = evicted {
            assert_eq!(
                newest.soldiers.as_ptr(),
                soldiers,
                "tick {k}: soldiers' buffer"
            );
            assert_eq!(newest.cover.as_ptr(), cover, "tick {k}: cover's buffer");
            assert_eq!(
                (newest.soldiers.capacity(), newest.cover.capacity()),
                (sc, cc)
            );
        } else {
            assert!(k < 2, "tick {k}: the ring is full after two pushes");
        }
    }
    // The control: replacing the value allocates new buffers.
    let mut copy = world.hit_volumes().clone();
    let old = copy.soldiers.as_ptr();
    copy = world.hit_volumes().clone();
    assert_ne!(copy.soldiers.as_ptr(), old, "a replacement allocates");
    // And clone_from into a value with room keeps its buffers.
    let mut into = world.hit_volumes().clone();
    let kept = (into.soldiers.as_ptr(), into.cover.as_ptr());
    into.clone_from(world.hit_volumes());
    assert_eq!((into.soldiers.as_ptr(), into.cover.as_ptr()), kept);
}

#[test]
fn carry14_the_ring_is_refused_above_one_megabyte_at_the_edge() {
    // Rust 3: capacity × one tick's HitVolumes bytes at most (HitVolumes::MAX_BYTES, the same on every target) stays
    // within 1 MB, 10^6 B as PM-0001-7 judges it (the advisor's W1-U4D T-M2). At 30 Hz: 220 ticks, a span of 7,333 ms;
    // 7,334 ms rounds up to 221 ticks and is refused.
    const MB: usize = 1_000_000;
    let most = MB / HitVolumes::MAX_BYTES;
    let edge_ms = (most as u64 * 1_000 / u64::from(TICK_HZ)) as u32;
    let at_edge = HitHistory::for_ms(edge_ms).expect("the edge is accepted");
    assert_eq!(at_edge.capacity() as usize, most);
    assert!(most * HitVolumes::MAX_BYTES <= MB && (most + 1) * HitVolumes::MAX_BYTES > MB);
    assert!(
        HitHistory::for_ms(edge_ms + 1).is_none(),
        "one ms past the edge"
    );
    assert!(HitHistory::for_ms(u32::MAX).is_none());
    assert_eq!(
        (HitVolumes::MAX_BYTES, most, edge_ms),
        (4_536, 220, 7_333),
        "at 30 Hz"
    );
    assert_eq!(ticks_for_ms(edge_ms + 1) as usize, most + 1);
    // Every scene's volumes fit MAX_BYTES's maxima.
    for (scene, seed, first) in scenes() {
        let world = World::new(&scene, seed, first);
        let v = world.hit_volumes();
        assert!(
            v.soldiers.len() <= HitVolumes::MAX_SOLDIERS && v.cover.len() <= HitVolumes::MAX_COVER,
            "{}: {} soldiers, {} cover",
            scene.name(),
            v.soldiers.len(),
            v.cover.len()
        );
    }
}

#[test]
fn carry15_hit_volumes_take_the_worlds_own_tick_never_the_inputs() {
    // Rust 7: World sets HitVolumes::tick from its own tick counter, never from TickInput::tick, so a wrong label
    // changes nothing: HitHistory::push (which debug-asserts consecutive ticks) takes every push, and the states agree.
    let scene = Scene::course();
    let rec = Recording::synthetic(&scene, 6, u32::MAX - 10, 200, SPAN_MS);
    let (mut a, mut b) = (
        World::new(&scene, 6, u32::MAX - 10),
        World::new(&scene, 6, u32::MAX - 10),
    );
    let (mut ha, mut hb) = (ring(), ring());
    let mut dice = Dice(77);
    for (k, t) in rec.ticks.iter().enumerate() {
        let mut wrong = t.clone();
        wrong.tick = dice.next();
        let expected = a.next_tick();
        room_tick(&mut a, &mut ha, t);
        room_tick(&mut b, &mut hb, &wrong);
        assert_eq!(a.hit_volumes().tick, expected, "tick {k}");
        assert_eq!(
            b.hit_volumes().tick,
            expected,
            "tick {k}: the label ignored"
        );
        assert_eq!(a.state_hash(), b.state_hash(), "tick {k}");
    }
}

#[test]
fn carry38_the_dump_holds_exactly_the_hashed_state() {
    // Condition 1's rule: StateDump holds exactly what state_hash hashes, never InputReports, so a twin's field diff
    // never shows a counter. Its sections are the state's; its hash is the state hash; two worlds that differ only in
    // their reports (D-10's twins) dump alike.
    let scene = Scene::stress();
    let rec = Recording::synthetic(&scene, 8, 0, 120, SPAN_MS);
    let mut world = World::new(&scene, 8, 0);
    let mut history = ring();
    let sections = [
        "world",
        "soldier",
        "effect",
        "vehicle",
        "part",
        "projectile",
        "door",
        "wall",
        "spawner",
        "rng",
        "events",
        "event",
    ];
    let mut clamps = 0;
    for t in &rec.ticks {
        room_tick(&mut world, &mut history, t);
        clamps += world.reports().clamps.len();
        let d = world.dump();
        assert_eq!(d.hash(), world.state_hash());
        for i in 0..d.words().len() {
            let f = d.field(i).expect("every word is named");
            let section = f.split('[').next().unwrap_or_default();
            assert!(sections.contains(&section), "{f}");
        }
        assert!(d.field(d.words().len()).is_none());
    }
    // Not vacuous: the run had refusals and clamps, which the dump never names.
    let r = world.reports();
    assert!(r.cooldown > 0 && clamps > 0, "{r:?}, {clamps} clamps");
}

#[test]
fn ray_cast_meets_the_nearest_live_box_and_changes_nothing() {
    // World::ray_cast (ADR 0001:258-260): the static world from the world, the movers supplied; it passes through the
    // skipped soldier and every box that is not live, and a ray that is not finite misses.
    let (world, o) = shooter();
    let before = world.dump();
    let b = |x: i32, live| VolumeBox {
        center: [x, -1_000, 140],
        half: [10, 10, 10],
        live,
    };
    let movers = HitVolumes {
        tick: 0,
        soldiers: vec![b(-6_000, true), b(-5_700, false), b(-5_500, true)],
        cover: vec![b(-5_200, true)],
    };
    let ray = |to: [f32; 3], skip| Ray {
        origin: o,
        to,
        skip,
    };
    let ahead = [o[0] + 10.0, o[1], o[2]];
    match world.ray_cast(&ray(ahead, Some(0)), &movers) {
        RayHit::Soldier { slot: 2, at } => assert!((at - 0.49).abs() < 0.001, "{at}"),
        other => panic!("{other:?}"),
    }
    assert!(matches!(
        world.ray_cast(&ray(ahead, None), &movers),
        RayHit::Soldier { slot: 0, .. }
    ));
    let mut no_soldier = movers.clone();
    no_soldier.soldiers[2].live = false;
    assert!(matches!(
        world.ray_cast(&ray(ahead, Some(0)), &no_soldier),
        RayHit::Cover { index: 0, .. }
    ));
    // The map's east edge, 188 m on.
    assert!(matches!(
        world.ray_cast(
            &ray([o[0] + 200.0, o[1], o[2]], Some(0)),
            &HitVolumes::default()
        ),
        RayHit::World { .. }
    ));
    assert_eq!(
        world.ray_cast(
            &ray([o[0] + 1.0, o[1], o[2]], Some(0)),
            &HitVolumes::default()
        ),
        RayHit::Miss
    );
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(
            world.ray_cast(&ray([bad, o[1], o[2]], Some(0)), &movers),
            RayHit::Miss
        );
    }
    assert_eq!(
        world.dump(),
        before,
        "ray_cast takes &self and changes nothing"
    );
}

#[cfg(feature = "bites")]
mod bites {
    //! The unit proofs of the sites `World` plants (u4-build-plan.md §4.14; bites builds only), each shown to change
    //! state here and run judged in the sub-unit that moves its mark (-41's in U4g; PM-0004-21's in U4d2).

    use super::*;

    #[test]
    fn world_plants_its_four_bite_ids_and_no_other() {
        assert_eq!(
            World::BITES,
            [
                "PM-0004-21/hash-map",
                "PM-0004-21/std-sin",
                "PM-0001-41/server-speed-1pct",
                "PM-0001-41/server-quant-differs",
            ]
        );
        // The own-copy snap's id is lib.rs's list (carry 11): World's list includes it.
        assert!(
            npt_sim::OWN_COPY_SNAP_BITES
                .iter()
                .all(|id| World::BITES.contains(id))
        );
        let scene = Scene::course();
        for id in World::BITES {
            assert!(World::with_bite(&scene, 1, 0, id, 0).is_some(), "{id}");
            assert!(
                npt_sim::replay::Replayer::with_bite(id, 0).is_some(),
                "{id}"
            );
        }
        for other in ["PM-0001-17/client-rounding", "PM-0001-41/no-such-bite", ""] {
            assert!(
                World::with_bite(&scene, 1, 0, other, 0).is_none(),
                "{other}"
            );
            assert!(
                npt_sim::replay::Replayer::with_bite(other, 0).is_none(),
                "{other}"
            );
        }
    }

    /// The course's soldier walking under `world`, with a predicting clone each step: the steps at which the
    /// authority's slot 0 differs from its prediction, and the steps whose snapped copy differs from the record's grid.
    fn walk_both(mut authority: World, steps: u32) -> (Vec<u32>, Vec<u32>) {
        let wire = Config::default();
        let mut history = ring();
        let mut presses = [0u8; 4];
        let (mut predicted, mut snapped) = (Vec::new(), Vec::new());
        for k in 0..steps {
            let at = soldier(&authority, 0).grid;
            let c = walk(
                0,
                authority.next_tick(),
                k,
                [at[0], at[1], at[2]],
                &mut presses,
            );
            let mut client = authority.clone();
            let t = input(&authority, vec![(c, None)]);
            room_tick(&mut authority, &mut history, &t);
            client.load_movers(authority.entity_records());
            client.predict(&c);
            if soldier(&client, 0) != soldier(&authority, 0) {
                predicted.push(k);
            }
            let r = soldier(&authority, 0);
            let held = word(&authority.dump(), "soldier[0].pos.x");
            if held != wire.from_grid(POSITION_XY, r.grid[0]).to_bits() {
                snapped.push(k);
            }
        }
        (predicted, snapped)
    }

    #[test]
    fn pm_0001_41_server_speed_1pct_moves_the_authority_step_and_not_predict() {
        // Q24: the authority step runs at 1.01 × the speed, predict does not, so a moving tick differs from the arming
        // tick on; before it, none does. The clean world (D-3) never differs.
        let scene = Scene::course();
        let id = "PM-0001-41/server-speed-1pct";
        let bitten = World::with_bite(&scene, 7, 0, id, 100).expect("World plants it");
        let (predicted, snapped) = walk_both(bitten, 300);
        assert!(predicted.iter().all(|&k| k >= 100), "{predicted:?}");
        assert!(
            predicted.len() > 100,
            "{} moving ticks differ",
            predicted.len()
        );
        assert!(snapped.is_empty(), "the speed bite leaves the snap alone");
        let (predicted, _) = walk_both(World::new(&scene, 7, 0), 300);
        assert!(predicted.is_empty(), "the clean world: {predicted:?}");
    }

    #[test]
    fn pm_0001_41_server_quant_differs_snaps_the_authoritys_own_copy_off_the_records_grid() {
        // Carry 9's unit proof and arch F1: under the bite the authority's own-copy snap takes its grid from
        // own_copy_snap_with_bite (floor rounding), while the records keep the wire grid: on moving ticks the snapped
        // copy differs from from_grid of the record's grid entry (about half the time), and the prediction (on the
        // wire grid) differs from the authority.
        let scene = Scene::course();
        let id = "PM-0001-41/server-quant-differs";
        let bitten = World::with_bite(&scene, 7, 0, id, 50).expect("World plants it");
        let (predicted, snapped) = walk_both(bitten, 300);
        assert!(snapped.iter().all(|&k| k >= 50), "{snapped:?}");
        assert!(
            snapped.len() > 40,
            "{} ticks off the record's grid",
            snapped.len()
        );
        assert!(!predicted.is_empty() && predicted.iter().all(|&k| k >= 50));
        let (_, snapped) = walk_both(World::new(&scene, 7, 0), 300);
        assert!(snapped.is_empty(), "the clean world: {snapped:?}");
    }

    #[test]
    fn world_bites_are_per_instance_a_bitten_and_a_clean_world_on_parallel_threads() {
        // u4-build-plan.md §4.11 (T10): a bite is a constructor argument, never a static, so a bitten world and a clean
        // one run side by side in one process, and the clean one's hashes are a clean run's.
        let scene = Scene::stress();
        let rec = Recording::synthetic(&scene, 3, 0, 200, SPAN_MS);
        let run = |world: World| {
            let rec = rec.clone();
            std::thread::spawn(move || {
                let mut world = world;
                let mut history = ring();
                rec.ticks
                    .iter()
                    .map(|t| {
                        room_tick(&mut world, &mut history, t);
                        world.state_hash()
                    })
                    .collect::<Vec<u64>>()
            })
        };
        let bitten = World::with_bite(&scene, 3, 0, "PM-0001-41/server-speed-1pct", 20)
            .expect("World plants it");
        let (b, c) = (run(bitten), run(World::new(&scene, 3, 0)));
        let (b, c) = (b.join().expect("bitten"), c.join().expect("clean"));
        let reference = run(World::new(&scene, 3, 0)).join().expect("reference");
        assert_eq!(c, reference, "the clean world beside the bitten one");
        assert_eq!(b[..20], reference[..20], "equal before the arming tick");
        assert_ne!(
            b[20..],
            reference[20..],
            "the bite changes the state from its tick"
        );
    }

    #[test]
    fn pm_0004_21_std_sin_is_planted_at_the_vehicles_loops_from_its_arming_tick() {
        // The site only: from the arming tick the vehicles' loops take std's sin (the target's libm) in place of the
        // pinned libm crate. Before it the state is the clean one's; whether this target's std sin differs from libm's
        // on these inputs is U4d2's scan and cross-target proof (u4-build-plan.md §5 U4d2), so this test only reports it.
        let scene = Scene::stress();
        let rec = Recording::synthetic(&scene, 5, 0, 300, SPAN_MS);
        let mut bitten =
            World::with_bite(&scene, 5, 0, "PM-0004-21/std-sin", 100).expect("World plants it");
        let mut clean = World::new(&scene, 5, 0);
        let (mut hb, mut hc) = (ring(), ring());
        let mut first = None;
        for (k, t) in rec.ticks.iter().enumerate() {
            room_tick(&mut bitten, &mut hb, t);
            room_tick(&mut clean, &mut hc, t);
            if bitten.state_hash() != clean.state_hash() {
                assert!(k >= 100, "tick {k}: a difference before the arming tick");
                first.get_or_insert(k);
            }
        }
        println!("PM-0004-21/std-sin on this target: first differing tick {first:?}");
    }
}
