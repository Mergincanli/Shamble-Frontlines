//! U4r's tests of npt-bots' `Behaviour` (u4r-brief.md 2.2): a real `ClientEndpoint` fed crafted room messages, each
//! encoded with npt-proto as the room sends it, and the commands it then sends read back from its `Input` datagrams.

use npt_bots::Behaviour;
use npt_client::ClientEndpoint;
use npt_link::Channel;
use npt_proto::{
    Direction, EntityDelta, FullState, Join, JoinAccepted, Message, Snapshot, decode, encode,
};
use npt_sim::{Command, EntityKind, EntityRecord, Scene};

fn bytes(m: &Message) -> Vec<u8> {
    let mut out = Vec::new();
    encode(m, &mut out).unwrap_or_else(|e| panic!("{}: {e}", m.name()));
    out
}

fn soldier(index: u16, x: i32) -> EntityRecord {
    let mut grid = [0; EntityRecord::FIELDS];
    grid[0] = x;
    grid[2] = 0;
    grid[6] = 1_000;
    EntityRecord {
        kind: EntityKind::Soldier,
        index,
        generation: 0,
        area: 0,
        grid,
        raw: [0; EntityRecord::FIELDS],
    }
}

/// The room's `JoinAccepted` for slot 0 at tick `t`.
fn accepted(t: u32) -> (Channel, Vec<u8>, u64) {
    let m = Message::JoinAccepted(JoinAccepted {
        slot: 0,
        tick: t,
        ..JoinAccepted::default()
    });
    (Channel::Control, bytes(&m), 0)
}

/// The full state of tick `t`: the client's own soldier and soldiers 1 and 20.
fn full(t: u32) -> (Channel, Vec<u8>, u64) {
    let m = Message::FullState(FullState {
        tick: t,
        last_run: None,
        own: vec![soldier(0, 0)],
        entities: vec![soldier(1, 100), soldier(20, 2_000)],
    });
    (Channel::FullState, bytes(&m), 0)
}

/// One snapshot group of tick `t` against the full state `age` ticks back: soldier 1 moved to `x`.
fn snap(t: u32, age: u16, x: i32) -> (Channel, Vec<u8>, u64) {
    let delta = EntityDelta::between(&soldier(1, x), Some(&soldier(1, 100))).expect("a change");
    let m = Message::Snapshot(Snapshot {
        tick: t,
        baseline_age: age,
        group: 0,
        groups: 1,
        entities: vec![delta],
        ..Snapshot::default()
    });
    (Channel::Datagram, bytes(&m), 0)
}

/// The commands of the `Input` the endpoint's next step sends (newest first), or none.
fn sent(e: &mut ClientEndpoint) -> Vec<Command> {
    let out = e.step();
    out.datagrams
        .iter()
        .find_map(
            |d| match decode(Direction::ClientToRoom, Channel::Datagram, d) {
                Ok(Message::Input(i)) => Some(i.commands),
                _ => None,
            },
        )
        .unwrap_or_default()
}

fn endpoint(lead: u32) -> ClientEndpoint {
    ClientEndpoint::new(Join::default(), npt_sim::TICK_HZ).with_lead(lead)
}

#[test]
fn b1_no_command_before_a_snapshot_then_one_per_step_whose_newest_tick_moved() {
    let scene = Scene::stress();
    let mut b = Behaviour::brawl(&scene, 0, 7);
    let mut e = endpoint(5);
    assert!(
        !b.step(&mut e, &[accepted(100)], 0, &[]),
        "joined, no snapshot yet: no command"
    );
    assert!(sent(&mut e).is_empty());
    assert!(
        b.step(&mut e, &[full(100)], 33, &[]),
        "the full state's tick is the newest: a command"
    );
    let c = sent(&mut e);
    assert_eq!(c.len(), 1);
    assert_eq!(c[0].tick, 105, "stamped newest + lead");
    assert_eq!(
        (c[0].view_tick, c[0].view_fraction),
        (100, 0),
        "the view is the newest tick"
    );
    assert!(
        !b.step(&mut e, &[], 66, &[]),
        "the newest tick did not move: no command"
    );
    assert_eq!(b.skipped(), 1);
    assert!(b.step(&mut e, &[snap(101, 1, 150)], 99, &[]));
    let c = sent(&mut e);
    assert_eq!(c[0].tick, 106);
    assert_eq!(
        c[0].seq,
        c[1].seq.wrapping_add(1),
        "one new command, the older resent"
    );
    assert_eq!((b.given(), b.skipped()), (2, 1));
    assert_eq!(
        c[0].buttons & Command::FIRE,
        Command::FIRE,
        "the brawl holds fire"
    );
}

#[test]
fn b2_every_step_gives_a_command_at_each_step_after_the_first_snapshot() {
    let scene = Scene::stress();
    let mut b = Behaviour::brawl(&scene, 0, 7).every_step();
    let mut e = endpoint(5);
    assert!(!b.step(&mut e, &[accepted(100)], 0, &[]));
    for i in 0..5 {
        let r = if i == 0 { vec![full(100)] } else { vec![] };
        assert!(b.step(&mut e, &r, 33 * (i + 1), &[]), "step {i}");
    }
    assert_eq!((b.given(), b.skipped()), (5, 0));
    let c = sent(&mut e);
    assert!(
        c.iter().all(|c| c.tick == 105),
        "every one stamped for the same tick"
    );
}

/// Every pressable counter moves by at most one edge from one command to the next (U4R-Q14, A4).
fn at_most_one_edge(cmds: &[Command]) -> bool {
    cmds.windows(2).all(|w| {
        w[0].presses
            .iter()
            .zip(&w[1].presses)
            .all(|(a, b)| b.wrapping_sub(*a) <= 1)
    })
}

/// The commands a behaviour gives over `n` steps, each newest tick moving by one.
fn commands(mut b: Behaviour, n: u32) -> Vec<Command> {
    let mut e = endpoint(2);
    b.step(&mut e, &[accepted(100)], 0, &[]);
    b.step(&mut e, &[full(100)], 1, &[]);
    let mut out = sent(&mut e);
    for i in 1..n {
        let view = [(1, [100 + i as i32, 0, 0]), (20, [2_000, i as i32, 0])];
        b.step(
            &mut e,
            &[snap(100 + i, 1, 100 + i as i32)],
            u64::from(i) * 33,
            &view,
        );
        out.push(sent(&mut e)[0]);
    }
    out
}

#[test]
fn b3_each_press_counter_moves_by_at_most_one_edge_per_command() {
    let scene = Scene::stress();
    for name in ["presses", "clamp", "steady"] {
        let b = Behaviour::script(&scene, 3, 11, name).expect("a script");
        let cmds = commands(b, 40);
        assert!(at_most_one_edge(&cmds), "{name}");
        if name == "presses" {
            let edges = cmds
                .windows(2)
                .filter(|w| w[0].presses != w[1].presses)
                .count();
            assert!(edges >= 15, "the presses script presses: {edges} edges");
        }
    }
    assert!(at_most_one_edge(&commands(
        Behaviour::brawl(&scene, 3, 11),
        40
    )));
    assert!(Behaviour::script(&scene, 3, 11, "cheat").is_none());
}

#[test]
fn b4_the_same_seed_gives_the_same_commands_and_another_seed_others() {
    let scene = Scene::stress();
    let a = commands(Behaviour::brawl(&scene, 5, 1), 60);
    let b = commands(Behaviour::brawl(&scene, 5, 1), 60);
    let c = commands(Behaviour::brawl(&scene, 5, 2), 60);
    assert_eq!(a, b);
    assert_ne!(a, c);
    let room_check = |c: &Command| {
        let [x, y] = c.move_xy.map(i32::from);
        x * x + y * y <= 127 * 127
    };
    assert!(a.iter().all(room_check), "every move inside x² + y² ≤ 127²");
}

#[test]
fn b5_the_brawl_aims_at_an_enemy_in_view_never_a_teammate() {
    let scene = Scene::stress();
    // Slot 0 is team 0: slot 1 is its teammate, slot 20 an enemy (3 x 16).
    assert_eq!(
        (scene.team(0), scene.team(1), scene.team(20)),
        (Some(0), Some(0), Some(1))
    );
    let cmds = commands(Behaviour::brawl(&scene, 0, 9), 90);
    for (i, c) in cmds.iter().enumerate().skip(1) {
        let enemy = [2_000, i as i32, 100];
        assert_eq!(
            c.aim, enemy,
            "command {i} aims at slot 20, raised into its box"
        );
    }
    let clamp = commands(Behaviour::script(&scene, 0, 9, "clamp").expect("clamp"), 5);
    assert!(
        clamp
            .iter()
            .all(|c| c.aim[0].abs() == 32_000 && c.aim[1].abs() == 32_000),
        "the clamp script aims far beyond the reach"
    );
}
