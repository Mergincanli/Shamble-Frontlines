//! npt-proto's planted defects (bites builds only; u4-build-plan.md §4.14, §5 U4a): each chosen per caller through
//! `Quant::with_bite`, each shown here to change what the codec does where its test (a4, a5, a6, a7) and its mark
//! look, and nowhere a clean `Quant` does.
#![cfg(feature = "bites")]

use npt_link::Channel;
use npt_proto::{
    BITES, DecodeError, Direction, EntityDelta, Input, Message, Quant, Snapshot, client_hash,
    decode, encode,
};
use npt_sim::grid::{PITCH, POSITION_XY, POSITION_Z, VELOCITY, YAW};
use npt_sim::{Command, EntityKind, EntityRecord};

fn bitten(id: &str) -> Quant {
    Quant::with_bite(id).unwrap_or_else(|| panic!("npt-proto plants {id}"))
}

fn bytes_with(q: Quant, m: &Message) -> Vec<u8> {
    let mut out = Vec::new();
    q.encode(m, &mut out).expect("encodes");
    out
}

#[test]
fn npt_proto_lists_its_seven_ids() {
    assert_eq!(
        BITES,
        [
            "PM-0001-57/pos-4cm",
            "PM-0001-57/yaw-12bit",
            "PM-0001-57/pitch-8bit",
            "PM-0001-17/tick16",
            "PM-0001-58/tick16",
            "PM-0001-21/no-trailing",
            "PM-0001-21/no-channel",
        ]
    );
    for other in [
        "PM-0001-17/client-rounding",
        "PM-0001-41/server-quant-differs",
        "PM-0001-58/lt-ack",
    ] {
        assert!(Quant::with_bite(other).is_none(), "{other}");
    }
}

/// A soldier at tick `tick`, every field at its value from `pick`.
fn soldier(pick: impl Fn(npt_sim::grid::Field) -> i32) -> EntityRecord {
    let mut grid = [0; EntityRecord::FIELDS];
    for (g, f) in grid.iter_mut().zip(EntityKind::Soldier.layout()) {
        *g = pick(*f);
    }
    EntityRecord {
        kind: EntityKind::Soldier,
        index: 4,
        generation: 1,
        area: 0,
        grid,
        raw: [0; EntityRecord::FIELDS],
    }
}

/// A soldier's owner-only record (the advisor's W1-S1: its velocity and its weapon's state), every field at its value
/// from `pick`.
fn own_state(pick: impl Fn(npt_sim::grid::Field) -> i32) -> EntityRecord {
    let mut grid = [0; EntityRecord::FIELDS];
    for (g, f) in grid.iter_mut().zip(EntityKind::OwnState.layout()) {
        *g = pick(*f);
    }
    EntityRecord {
        kind: EntityKind::OwnState,
        index: 4,
        generation: 1,
        area: 0,
        grid,
        raw: [0; EntityRecord::FIELDS],
    }
}

/// `r` sent in the section that carries it (the own-state section for the owner-only record, the entity section for a
/// soldier) and decoded.
fn sent_and_decoded(q: Quant, r: EntityRecord) -> EntityRecord {
    let own = r.kind == EntityKind::OwnState;
    let mut s = Snapshot {
        tick: 100,
        ..Snapshot::default()
    };
    if own {
        s.own = vec![EntityDelta::Add(r)];
    } else {
        s.entities = vec![EntityDelta::Add(r)];
    }
    let b = bytes_with(q, &Message::Snapshot(s));
    let Ok(Message::Snapshot(back)) = decode(Direction::RoomToClient, Channel::Datagram, &b) else {
        panic!("decodes")
    };
    let EntityDelta::Add(got) = (if own { back.own[0] } else { back.entities[0] }) else {
        panic!("an add")
    };
    got
}

#[test]
fn pm_0001_57s_bites_send_coarse_integers_the_client_decodes_as_they_are() {
    let layout = EntityKind::Soldier.layout();
    for (id, fields, step) in [
        ("PM-0001-57/pos-4cm", &[POSITION_XY, POSITION_Z][..], 4),
        ("PM-0001-57/yaw-12bit", &[YAW][..], 16),
        ("PM-0001-57/pitch-8bit", &[PITCH][..], 4),
    ] {
        let q = bitten(id);
        // An interior tie (half a coarse step off a multiple) is off by half the step; the top end by the nearest
        // multiple inside (for yaw, around the turn); every other field arrives as sent, and so does the owner-only
        // record, velocity included (the advisor's W1-S1 moved velocity out of the soldier's record into it).
        for what in ["half a step", "the top end"] {
            let r = soldier(|f| match what {
                "the top end" => f.max(),
                _ if fields.contains(&f) => step / 2,
                _ => 3,
            });
            let got = sent_and_decoded(q, r);
            let clean = sent_and_decoded(Quant::default(), r);
            assert_eq!(clean, r, "the wire's Quant sends every integer as it is");
            for (i, f) in layout.iter().enumerate() {
                if !fields.contains(f) {
                    assert_eq!(got.grid[i], r.grid[i], "{id} {what}: field {i} touched");
                    continue;
                }
                let e = (i64::from(got.grid[i]) - i64::from(r.grid[i])).abs();
                let e = if f.wraps() { e.min(65_536 - e) } else { e };
                assert!(
                    e > 0 && e < i64::from(step),
                    "{id} {what}: field {i} off by {e}"
                );
                assert_eq!(
                    got.grid[i].rem_euclid(step),
                    0,
                    "{id} {what}: field {i} is {}",
                    got.grid[i]
                );
            }
            // The owner-only record in its own section: velocity at half the coarse step or its top end, the rest at
            // 3 or theirs; nothing is touched.
            let o = own_state(|f| match what {
                "the top end" => f.max(),
                _ if f == VELOCITY => step / 2,
                _ => 3,
            });
            assert_eq!(
                sent_and_decoded(q, o),
                o,
                "{id} {what}: the owner-only record"
            );
            assert!(
                EntityKind::OwnState.layout().contains(&VELOCITY),
                "velocity is in the owner-only record"
            );
        }
    }
    // VELOCITY shares POSITION_XY's range and is another kind of field: pos-4cm leaves it alone.
    assert_eq!(VELOCITY.min(), POSITION_XY.min());
    assert_ne!(VELOCITY, POSITION_XY);
}

#[test]
fn tick16_writes_the_header_tick_in_16_bits_so_a_frame_past_the_wrap_is_labelled_wrongly() {
    for id in ["PM-0001-17/tick16", "PM-0001-58/tick16"] {
        let q = bitten(id);
        for (tick, seen) in [
            (65_537u32, 1u32),
            (63_736 + 2_000, 63_736 + 2_000 - 65_536),
            (1_000, 1_000),
        ] {
            let s = Snapshot {
                tick,
                ..Snapshot::default()
            };
            let b = bytes_with(q, &Message::Snapshot(s));
            let Ok(Message::Snapshot(back)) =
                decode(Direction::RoomToClient, Channel::Datagram, &b)
            else {
                panic!("decodes")
            };
            assert_eq!(back.tick, seen, "{id} at {tick}");
            // §4.14's unit proof: past the wrap the client's frame hashes differently from the server's.
            assert_eq!(
                client_hash(back.tick, &[], &[], &[]) == client_hash(tick, &[], &[], &[]),
                tick < 65_536,
                "{id} at {tick}"
            );
        }
    }
}

fn one_command_input() -> Vec<u8> {
    let m = Message::Input(Input {
        commands: vec![Command {
            tick: 9,
            view_tick: 7,
            ..Command::default()
        }],
        ..Input::default()
    });
    let mut out = Vec::new();
    encode(&m, &mut out).expect("encodes");
    out
}

#[test]
fn no_trailing_lets_trailing_bytes_and_set_reserved_bits_through_as_a6_refuses_them() {
    let q = bitten("PM-0001-21/no-trailing");
    let mut trailing = one_command_input();
    trailing.extend_from_slice(&[0xde, 0xad]);
    assert_eq!(
        decode(Direction::ClientToRoom, Channel::Datagram, &trailing),
        Err(DecodeError::TrailingBytes)
    );
    assert_eq!(
        q.decode(Direction::ClientToRoom, Channel::Datagram, &trailing)
            .map(|m| m.name()),
        Ok("Input")
    );
    let mut pad = one_command_input();
    *pad.last_mut().expect("bytes") |= 0x80;
    assert_eq!(
        decode(Direction::ClientToRoom, Channel::Datagram, &pad),
        Err(DecodeError::UnknownField)
    );
    assert_eq!(
        q.decode(Direction::ClientToRoom, Channel::Datagram, &pad)
            .map(|m| m.name()),
        Ok("Input")
    );
}

#[test]
fn no_channel_lets_an_input_through_on_the_control_stream_as_a7_refuses_it() {
    let q = bitten("PM-0001-21/no-channel");
    let datagram = one_command_input();
    let mut on_stream = (datagram.len() as u16).to_le_bytes().to_vec();
    on_stream.extend_from_slice(&datagram);
    assert_eq!(
        decode(Direction::ClientToRoom, Channel::Control, &on_stream),
        Err(DecodeError::WrongChannel)
    );
    let Ok(Message::Input(i)) = q.decode(Direction::ClientToRoom, Channel::Control, &on_stream)
    else {
        panic!("the forged kind gets through")
    };
    assert_eq!(i.commands[0].tick, 9);
    // The direction check stays: a room type from a client is still refused.
    let mut snap = Vec::new();
    encode(&Message::Snapshot(Snapshot::default()), &mut snap).expect("encodes");
    assert_eq!(
        q.decode(Direction::ClientToRoom, Channel::Datagram, &snap),
        Err(DecodeError::WrongDirection)
    );
}
