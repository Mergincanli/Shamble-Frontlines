//! Carry 80 (b)-(e), the client's half (u4f-brief.md 2.9; carries :520-536): each an endpoint test fed hand-made room
//! datagrams; f26's two halves and f27 each fail under their own plant (the plants' runs: u4f-build.md).

use npt_client::ClientEndpoint;
use npt_link::Channel;
use npt_proto::{Direction, FullState, Input, JoinAccepted, Message, Snapshot, decode, encode};
use npt_sim::Command;

fn bytes(m: &Message) -> Vec<u8> {
    let mut out = Vec::new();
    encode(m, &mut out).expect("encodes");
    out
}

/// An endpoint joined at `join` (a fixed lead of 2), then fed one empty snapshot of each tick in `ticks`.
fn fed(join: u32, ticks: std::ops::RangeInclusive<u32>) -> ClientEndpoint {
    let mut e = ClientEndpoint::new(npt_proto::Join::default(), 30).with_lead(2);
    let ja = Message::JoinAccepted(JoinAccepted {
        slot: 0,
        tick: join,
        ..JoinAccepted::default()
    });
    e.input(None, &[(Channel::Control, bytes(&ja), 0)], 0);
    e.step();
    for t in ticks {
        e.input(None, &[(Channel::Datagram, snap(t, 0), 0)], 0);
        e.step();
    }
    e
}

fn snap(tick: u32, baseline_age: u16) -> Vec<u8> {
    bytes(&Message::Snapshot(Snapshot {
        tick,
        baseline_age,
        groups: 1,
        ..Snapshot::default()
    }))
}

/// A snapshot datagram that does not decode (its header's tick, then nothing): the client refuses it.
fn refused_snap(tick: u32) -> Vec<u8> {
    let mut b = vec![2u8];
    b.extend_from_slice(&tick.to_le_bytes());
    assert!(decode(Direction::RoomToClient, Channel::Datagram, &b).is_err());
    b
}

/// The acknowledgement the endpoint's next `Input` carries, and its commands.
fn next_input(e: &mut ClientEndpoint) -> Option<Input> {
    e.step().datagrams.iter().find_map(|d| {
        match decode(Direction::ClientToRoom, Channel::Datagram, d) {
            Ok(Message::Input(i)) => Some(i),
            _ => None,
        }
    })
}

/// f24 (carry 80 (b), by U4F-Q16 (b)): a refused snapshot datagram whose header reads more than `ACK_RING_TICKS` (32)
/// past the newest tick is dropped and counted, and moves nothing (no sentinel, no lost run, the newest unchanged); one
/// within the ring gets the sentinel and moves the newest tick as before (the span it skips is resolved as each tick
/// leaves the ring). An honest stream's `lost` is unchanged.
#[test]
fn f24_a_refused_far_ahead_header_moves_the_newest_tick_by_at_most_the_ring() {
    let mut e = fed(100, 100..=110);
    let refused = e.refused();
    e.input(None, &[(Channel::Datagram, refused_snap(110 + 40), 0)], 0);
    let out = e.step();
    assert_eq!(e.refused(), refused + 1);
    assert_eq!(e.newest_tick(), Some(110));
    assert!(out.hashes.is_empty() && out.lost.is_empty(), "{out:?}");
    e.input(None, &[(Channel::Datagram, refused_snap(110 + 20), 0)], 0);
    let out = e.step();
    assert_eq!(e.newest_tick(), Some(130));
    assert_eq!(out.hashes, [(130, 0)]);
    assert!(out.lost.is_empty(), "{:?}", out.lost);
    // TE-U4F-4: the bound itself, 32 (`> 32` dropped): newest + 33 dropped, newest + 32 taken.
    let refused = e.refused();
    e.input(None, &[(Channel::Datagram, refused_snap(130 + 33), 0)], 0);
    let out = e.step();
    assert_eq!((e.refused(), e.newest_tick()), (refused + 1, Some(130)));
    assert!(out.hashes.is_empty(), "{out:?}");
    e.input(None, &[(Channel::Datagram, refused_snap(130 + 32), 0)], 0);
    assert_eq!(e.step().hashes, [(162, 0)]);
    assert_eq!(e.newest_tick(), Some(162));
    // The honest stream: every tick filed, none lost.
    let mut honest = fed(100, 100..=150);
    assert!(honest.step().lost.is_empty());
}

/// f25 (carry 80 (c)): a page command whose aim lies outside the `Input` codec's range (x 40,000 cm against
/// `POSITION_XY`'s 16 bits) is dropped and counted (`unencodable_commands`), and the `Input` keeps its acknowledgement;
/// the next command goes out.
#[test]
fn f25_an_unencodable_command_is_dropped_and_counted_and_the_acks_kept() {
    let mut e = fed(100, 100..=105);
    let bad = Command {
        aim: [40_000, 0, 100],
        ..Command::default()
    };
    e.input(Some(bad), &[], 0);
    let i = next_input(&mut e).expect("an Input with the acknowledgement");
    assert!(i.ack_groups != 0 && i.commands.is_empty(), "{i:?}");
    assert_eq!(e.unencodable_commands(), 1);
    e.input(Some(Command::default()), &[], 0);
    let i = next_input(&mut e).expect("an Input");
    assert_eq!(i.commands.len(), 1);
}

/// f25 (ii) (carry 80 (c), RR-U4F-1): the codec's other pair rule. One command unacknowledged (stamped 107), then a
/// snapshot moves the newest tick by +300 and the next command is stamped 407: the older one lies 300 ticks below its
/// newer neighbour, outside the codec's 0..=255, so it is left out (counted) and the `Input` carries the newest.
#[test]
fn f25_ii_an_older_command_past_the_codecs_tick_step_is_left_out() {
    let mut e = fed(100, 100..=105);
    e.input(Some(Command::default()), &[], 0);
    assert_eq!(next_input(&mut e).expect("an Input").commands.len(), 1);
    let jump = [(Channel::Datagram, snap(105 + 300, 0), 0)];
    e.input(Some(Command::default()), &jump, 0);
    let i = next_input(&mut e).expect("an Input");
    assert_eq!((i.ack_tick, i.commands.len()), (405, 1), "{i:?}");
    assert_eq!(i.commands[0].tick, 405 + 2);
    assert_eq!((e.twin_left_out(), e.unencodable_commands()), (1, 0));
}

/// f26 (carry 80 (d), as worded): joined at tick 100 and fed to 140, a group of tick 60 (before the first snapshot: a
/// bad header) fed twice files `(60, 0)` once, the next `Input` acknowledges no tick 60, `newest_tick()` is unchanged,
/// `refused()` rose by 2 and a delta on a live baseline still decodes; then a group of tick 105 (from the first snapshot
/// on, already out of the ring) files nothing and adds 1 to `stale()`, never the sentinel.
#[test]
fn f26_the_clients_two_too_old_cases() {
    let mut e = fed(100, 100..=140);
    let (refused, stale) = (e.refused(), e.stale());
    e.input(None, &[(Channel::Datagram, snap(60, 0), 0)], 0);
    e.input(None, &[(Channel::Datagram, snap(60, 0), 0)], 0);
    let out = e.step();
    assert_eq!(out.hashes, [(60, 0)], "filed once");
    assert_eq!(e.refused(), refused + 2);
    assert_eq!(e.newest_tick(), Some(140));
    e.input(None, &[(Channel::Datagram, snap(141, 1), 0)], 0);
    let i = next_input(&mut e).expect("an Input");
    assert_eq!(
        i.ack_tick, 141,
        "the delta on tick 140's baseline decoded; no tick 60"
    );
    // The second half: a tick from the first snapshot on, out of the ring.
    e.input(None, &[(Channel::Datagram, snap(105, 0), 0)], 0);
    let out = e.step();
    assert!(out.hashes.is_empty(), "nothing filed: {:?}", out.hashes);
    assert_eq!(e.stale(), stale + 1);
}

/// f27 (carry 80 (e)): a tick completed by a snapshot and then sent as a full state of another hash files the
/// sentinel for it; the same tick's full state of the same hash is a duplicate.
#[test]
fn f27_a_full_state_of_another_hash_after_a_snapshot_files_the_sentinel() {
    let mut e = fed(100, 100..=105);
    let full = Message::FullState(FullState {
        tick: 105,
        last_run: None,
        own: Vec::new(),
        entities: vec![npt_sim::EntityRecord {
            kind: npt_sim::EntityKind::Soldier,
            index: 3,
            generation: 0,
            area: 0,
            grid: [0; npt_sim::EntityRecord::FIELDS],
            raw: [0; npt_sim::EntityRecord::FIELDS],
        }],
    });
    e.input(None, &[(Channel::FullState, bytes(&full), 0)], 0);
    assert_eq!(e.step().hashes, [(105, 0)]);
    let same = Message::FullState(FullState {
        tick: 104,
        last_run: None,
        own: Vec::new(),
        entities: Vec::new(),
    });
    let dup = e.duplicates();
    e.input(None, &[(Channel::FullState, bytes(&same), 0)], 0);
    assert!(e.step().hashes.is_empty());
    assert_eq!(e.duplicates(), dup + 1);
}
