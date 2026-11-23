//! C-U6-18's test (EA-U4F-9; u6a-brief.md Block 8): `Behaviour::host_step` runs input, the endpoint's step and
//! outgoing in that order. The never-acker's hook rewrites the `Input` the step made, so its acknowledgement is gone;
//! an honest behaviour's step sends its acknowledgement as made; a kind-11 forger's reliable hook adds its message.

use npt_bots::Behaviour;
use npt_client::ClientEndpoint;
use npt_link::Channel;
use npt_proto::{
    Direction, EntityDelta, FullState, Join, JoinAccepted, Message, Snapshot, decode, encode,
};
use npt_sim::{EntityKind, EntityRecord, Scene};

fn bytes(m: &Message) -> Vec<u8> {
    let mut out = Vec::new();
    encode(m, &mut out).unwrap_or_else(|e| panic!("{}: {e}", m.name()));
    out
}

fn soldier(index: u16, x: i32) -> EntityRecord {
    let mut grid = [0; EntityRecord::FIELDS];
    grid[0] = x;
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

/// The room's `JoinAccepted` for slot 0 and full state at tick 100, then one snapshot group at tick 101.
fn room() -> [Vec<(Channel, Vec<u8>, u64)>; 2] {
    let joined = vec![
        (
            Channel::Control,
            bytes(&Message::JoinAccepted(JoinAccepted {
                slot: 0,
                tick: 100,
                ..JoinAccepted::default()
            })),
            0,
        ),
        (
            Channel::FullState,
            bytes(&Message::FullState(FullState {
                tick: 100,
                last_run: None,
                own: vec![soldier(0, 0)],
                entities: vec![soldier(1, 100)],
            })),
            0,
        ),
    ];
    let delta = EntityDelta::between(&soldier(1, 150), Some(&soldier(1, 100))).expect("a change");
    let snap = Message::Snapshot(Snapshot {
        tick: 101,
        baseline_age: 1,
        group: 0,
        groups: 1,
        entities: vec![delta],
        ..Snapshot::default()
    });
    [joined, vec![(Channel::Datagram, bytes(&snap), 0)]]
}

/// The `Input` a host step sent: its acknowledged tick (the wire's low 16 bits) and groups, and its commands' count.
fn input(datagrams: &[Vec<u8>]) -> (u16, u8, usize) {
    datagrams
        .iter()
        .find_map(
            |d| match decode(Direction::ClientToRoom, Channel::Datagram, d) {
                Ok(Message::Input(i)) => Some((i.ack_tick, i.ack_groups, i.commands.len())),
                _ => None,
            },
        )
        .expect("the step sent an Input")
}

#[test]
fn c_u6_18_host_step_runs_input_then_the_endpoints_step_then_outgoing() {
    let scene = Scene::stress();
    let [joined, snap] = room();
    for never_acks in [false, true] {
        let mut b = Behaviour::brawl(&scene, 0, 7);
        if never_acks {
            b = b.never_acks();
        }
        let mut e = ClientEndpoint::new(Join::default(), npt_sim::TICK_HZ).with_lead(1);
        let (given, _) = b.host_step(&mut e, &joined, 0, &[]);
        assert!(given, "the full state's tick is the newest: a command");
        let (given, out) = b.host_step(&mut e, &snap, 33, &[]);
        assert!(given, "the newest tick moved: a command");
        let (ack_tick, ack_groups, commands) = input(&out.datagrams);
        assert!(
            commands >= 1,
            "input ran before the endpoint's step: its command is in the Input"
        );
        if never_acks {
            assert_eq!(
                (ack_tick, ack_groups),
                (0, 0),
                "outgoing ran after the endpoint's step: the never-acker's hook rewrote its Input"
            );
        } else {
            assert_eq!(
                (ack_tick, ack_groups),
                (101, 1),
                "an honest step acknowledges tick 101's group"
            );
        }
    }
    // The reliable hook acts after the endpoint's step too (TR-U6A-10): a kind-11 forger's second command brings its
    // `Ping` framed for the control stream, which host_step puts last on the step's reliable messages.
    let mut b = Behaviour::brawl(&scene, 0, 7)
        .forger(11, 2, 12)
        .expect("a kind-11 forger");
    let mut e = ClientEndpoint::new(Join::default(), npt_sim::TICK_HZ).with_lead(1);
    let (given, _) = b.host_step(&mut e, &joined, 0, &[]);
    assert!(given, "the first command");
    let (given, out) = b.host_step(&mut e, &snap, 33, &[]);
    assert!(given, "the second command: the forged one");
    let (channel, framed) = out.reliable.last().expect("a reliable message");
    assert_eq!(*channel, Channel::Control);
    assert!(
        matches!(
            decode(
                Direction::ClientToRoom,
                Channel::Datagram,
                framed.get(2..).unwrap_or_default()
            ),
            Ok(Message::Ping(_))
        ),
        "the forged Ping, framed for the control stream, in out.reliable"
    );
}
