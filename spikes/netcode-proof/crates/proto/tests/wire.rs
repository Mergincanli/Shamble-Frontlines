//! npt-proto's codec tests (u4-build-plan.md §5 U4a): a1 round trips, a2 totality, a3 the size check first, a6 the
//! `Input` field table, a8 `client_hash`'s pinned vectors, and the `EntityRecord` ↔ wire mapping's round trip. Every
//! case comes from a fixed seed (tests/common), so a failure names its type and case and repeats.

mod common;

use std::panic::{AssertUnwindSafe, catch_unwind};

use common::{Gen, encoded, framed_limit, framing, hex, row};
use npt_link::Channel;
use npt_proto::{
    DecodeError, Direction, EncodeError, EntityDelta, FullState, Input, Join, Limits, Message,
    ProfileId, RoomControl, SessionControl, Snapshot, StepResult, client_hash, decode, encode,
};
use npt_sim::grid::{POSITION_XY, POSITION_Z};
use npt_sim::{Command, EntityKind, EntityRecord, SimEvent};

const TYPES: usize = Limits::TYPES.len();

/// Decodes `bytes` as type `t`'s receiver does, a panic caught and reported as the test's failure.
fn decode_at(t: usize, bytes: &[u8]) -> Result<Message, DecodeError> {
    let (name, channel, way, _) = row(t);
    catch_unwind(AssertUnwindSafe(|| decode(way, channel, bytes)))
        .unwrap_or_else(|_| panic!("{name}: the decoder panicked on {}", hex(bytes)))
}

fn decode_control(bytes: &[u8]) -> Result<RoomControl, DecodeError> {
    catch_unwind(AssertUnwindSafe(|| RoomControl::decode(bytes)))
        .unwrap_or_else(|_| panic!("RoomControl: the decoder panicked on {}", hex(bytes)))
}

fn control_bytes(c: &RoomControl) -> Vec<u8> {
    let mut out = Vec::new();
    c.encode(&mut out).unwrap_or_else(|e| panic!("{c:?}: {e}"));
    out
}

#[test]
fn a1_every_message_type_round_trips_on_fixed_seeds_10_000_cases_each() {
    for t in 1..=TYPES {
        let name = row(t).0;
        let mut g = Gen::new(0xa10000 + t as u64);
        for case in 0..10_000 {
            let m = g.message(t);
            let bytes =
                encoded(&m).unwrap_or_else(|| panic!("{name} case {case} did not encode: {m:?}"));
            assert_eq!(m.name(), name);
            assert_eq!(
                decode_at(t, &bytes).as_ref(),
                Ok(&m),
                "{name} case {case}: {}",
                hex(&bytes)
            );
        }
    }
    let mut g = Gen::new(0xa1c0);
    for case in 0..10_000 {
        let c = g.room_control();
        let bytes = control_bytes(&c);
        assert_eq!(
            decode_control(&bytes).as_ref(),
            Ok(&c),
            "RoomControl case {case}: {}",
            hex(&bytes)
        );
    }
}

#[test]
fn a1_the_entity_record_mapping_round_trips_for_every_kind() {
    let mut g = Gen::new(0xa1e0);
    for kind in EntityKind::ALL {
        for case in 0..2_000 {
            let now = g.record(kind);
            // The baseline: none, the same entity with other values, or another generation at its index.
            let base = match g.below(3) {
                0 => None,
                1 => {
                    let mut b = g.record(kind);
                    (b.index, b.generation) = (now.index, now.generation);
                    if g.coin() {
                        // Some fields equal, so an update carries part of the layout.
                        for i in 0..kind.layout().len() {
                            if g.coin() {
                                b.grid[i] = now.grid[i];
                            }
                        }
                    }
                    Some(b)
                }
                _ => {
                    let mut b = now;
                    b.generation = now.generation.wrapping_add(1);
                    Some(b)
                }
            };
            let what = format!("{kind:?} case {case}");
            // The room's own fields never reach the wire.
            let mut sent = now;
            sent.area = g.u16();
            sent.raw = [g.u32(); EntityRecord::FIELDS];
            match EntityDelta::between(&sent, base.as_ref()) {
                None => assert_eq!(
                    base,
                    Some(now),
                    "{what}: no delta, yet the baseline differs"
                ),
                Some(d) => {
                    let id = match d {
                        EntityDelta::Add(r) => (r.kind, r.index),
                        EntityDelta::Update { kind, index, .. }
                        | EntityDelta::Remove { kind, index } => (kind, index),
                    };
                    assert_eq!(id, (kind, now.index), "{what}");
                    assert_eq!(d.apply(base.as_ref()), Ok(Some(now)), "{what}: {d:?}");
                    // Through the wire: a snapshot against a baseline (age 1), or against the defaults for an add.
                    let s = Snapshot {
                        baseline_age: if matches!(d, EntityDelta::Add(_)) {
                            0
                        } else {
                            1
                        },
                        entities: vec![d],
                        ..Snapshot::default()
                    };
                    let bytes = encoded(&Message::Snapshot(s.clone())).expect("one entity fits");
                    let Ok(Message::Snapshot(back)) = decode_at(2, &bytes) else {
                        panic!("{what}: {}", hex(&bytes));
                    };
                    assert_eq!(back, s, "{what}");
                    assert_eq!(
                        back.entities[0].apply(base.as_ref()),
                        Ok(Some(now)),
                        "{what}"
                    );
                }
            }
        }
        // Whole records, as a full state carries them.
        let f = FullState {
            own: (0..3).map(|_| g.record(kind)).collect(),
            entities: (0..50).map(|_| g.record(kind)).collect(),
            ..FullState::default()
        };
        let bytes = encoded(&Message::FullState(f.clone())).expect("fits");
        assert_eq!(decode_at(3, &bytes), Ok(Message::FullState(f)), "{kind:?}");
    }
    // An update without its baseline entity, or against another entity, is refused when applied.
    let r = g.record(EntityKind::Soldier);
    let mut moved = r;
    moved.grid[0] = if r.grid[0] == 0 { 1 } else { 0 };
    let d = EntityDelta::between(&moved, Some(&r)).expect("x changed");
    assert_eq!(d.apply(None), Err(DecodeError::Malformed));
    let mut other = r;
    other.index = r.index.wrapping_add(1);
    assert_eq!(d.apply(Some(&other)), Err(DecodeError::Malformed));
    assert_eq!(
        EntityDelta::Remove {
            kind: r.kind,
            index: r.index
        }
        .apply(Some(&r)),
        Ok(None)
    );
}

/// How many valid encodings per type a2 cuts at every byte.
const PREFIX_CASES: usize = 1_000;

#[test]
fn a2_every_proper_prefix_of_every_valid_encoding_is_an_error() {
    for t in 1..=TYPES {
        let name = row(t).0;
        let mut g = Gen::new(0xa20000 + t as u64);
        let cases = if matches!(name, "Snapshot" | "FullState") {
            PREFIX_CASES / 5
        } else {
            PREFIX_CASES
        };
        for case in 0..cases {
            let bytes = encoded(&g.message(t)).expect("valid");
            for cut in 0..bytes.len() {
                let r = decode_at(t, &bytes[..cut]);
                assert!(
                    r.is_err(),
                    "{name} case {case} cut at {cut} of {} decoded: {r:?}",
                    bytes.len()
                );
            }
        }
    }
    let mut g = Gen::new(0xa2c0);
    for case in 0..PREFIX_CASES {
        let bytes = control_bytes(&g.room_control());
        for cut in 0..bytes.len() {
            assert!(
                decode_control(&bytes[..cut]).is_err(),
                "RoomControl case {case} cut at {cut}"
            );
        }
    }
}

/// A random buffer for type `t`: random length (small half the time; at most 1,200 B, which holds a whole snapshot
/// datagram and a full state's first records: a3 covers the sizes at each limit) and bytes, the type byte in its
/// place, and for a stream half the time a length prefix that counts its bytes, so the body is reached.
fn random_buffer(g: &mut Gen, t: usize) -> Vec<u8> {
    let channel = row(t).1;
    let frame = framing(channel);
    let max = (framed_limit(t) + 1).min(1_200);
    let len = if g.coin() {
        g.below(64.min(max as u64) + 1)
    } else {
        g.below(max as u64 + 1)
    } as usize;
    let mut b = vec![0u8; len];
    g.fill(&mut b);
    if let Some(x) = b.get_mut(frame) {
        *x = t as u8;
    }
    if len >= frame && frame > 0 && g.coin() {
        let n = (len - frame) as u32;
        b[..frame].copy_from_slice(&n.to_le_bytes()[..frame]);
    }
    b
}

/// What `encode` writes for a message decoded from `b`: those bytes, or nothing for a `Snapshot` above its 1,000 B
/// send cap (a client takes one up to 1,150 B; the encoder refuses it).
fn as_sent<'a>(name: &str, b: &'a [u8]) -> Option<&'a [u8]> {
    (name != "Snapshot" || b.len() <= Limits::SNAPSHOT).then_some(b)
}

#[test]
fn a2_random_buffers_are_errors_or_the_message_that_encodes_to_them_never_a_panic() {
    // 100,000 fixed-seed buffers per type (u4-build-plan.md §5 U4a a2). A random buffer can be a valid message (any
    // ten bytes after a Ping's type byte are one), so the rule is: an error, or a message whose encoding is exactly
    // those bytes (one encoding per message: nothing decodes that the encoder would not write), but a snapshot between
    // the send cap and the receive limit, which the encoder refuses (as_sent).
    for t in 1..=TYPES {
        let name = row(t).0;
        let mut g = Gen::new(0xa2b000 + t as u64);
        let mut seen = std::collections::BTreeMap::new();
        for case in 0..100_000 {
            let b = random_buffer(&mut g, t);
            let r = decode_at(t, &b);
            if let Ok(m) = &r {
                assert_eq!(
                    encoded(m).as_deref(),
                    as_sent(name, &b),
                    "{name} case {case} decoded {}",
                    hex(&b)
                );
            }
            *seen
                .entry(format!("{:?}", r.map(|_| "decoded")))
                .or_insert(0u32) += 1;
        }
        // Random bytes rarely pass a header, so valid encodings with one to three bits flipped reach every body field
        // too (10,000 per type).
        for case in 0..10_000 {
            let mut b = encoded(&g.message(t)).expect("valid");
            for _ in 0..1 + g.below(3) {
                let bit = g.below(b.len() as u64 * 8) as usize;
                b[bit / 8] ^= 1 << (bit % 8);
            }
            let r = decode_at(t, &b);
            if let Ok(m) = &r {
                assert_eq!(
                    encoded(m).as_deref(),
                    as_sent(name, &b),
                    "{name} flipped case {case} decoded {}",
                    hex(&b)
                );
            }
            *seen
                .entry(format!("flipped {:?}", r.map(|_| "decoded")))
                .or_insert(0u32) += 1;
        }
        // What the buffers met (shown with --nocapture): the size, framing and body checks all reached.
        eprintln!("a2 {name}: {seen:?}");
    }
    let mut g = Gen::new(0xa2bc0);
    for case in 0..100_000 {
        let len = if g.coin() {
            g.below(40)
        } else {
            g.below(Limits::ROOM_CONTROL as u64 + 2)
        } as usize;
        let mut b = vec![0u8; len];
        g.fill(&mut b);
        if len >= 3 {
            b[2] = 1 + g.below(11) as u8;
            if g.coin() {
                b[..2].copy_from_slice(&((len - 2) as u16).to_le_bytes());
            }
        }
        if let Ok(c) = decode_control(&b) {
            assert_eq!(
                control_bytes(&c),
                b,
                "RoomControl case {case} decoded {}",
                hex(&b)
            );
        }
    }
}

#[test]
fn a3_one_byte_over_its_types_limit_is_the_size_error_whatever_the_buffer_holds() {
    for t in 1..=TYPES {
        let (name, channel, way, _) = row(t);
        let frame = framing(channel);
        let over = framed_limit(t) + 1;
        let mut g = Gen::new(0xa30000 + t as u64);
        for case in 0..1_000 {
            // A valid encoding with random bytes after it, its length prefix counting them: malformed too.
            let mut b = encoded(&g.message(t)).expect("valid");
            let mut rest = vec![0u8; over - b.len()];
            g.fill(&mut rest);
            b.extend_from_slice(&rest);
            if frame > 0 {
                let n = (over - frame) as u32;
                b[..frame].copy_from_slice(&n.to_le_bytes()[..frame]);
            }
            assert_eq!(
                decode_at(t, &b),
                Err(DecodeError::TooLarge),
                "{name} case {case} (a padded encoding)"
            );
            // Random bytes with only the type byte in its place: its prefix is garbage.
            let mut b = vec![0u8; over];
            g.fill(&mut b);
            b[frame] = t as u8;
            assert_eq!(
                decode_at(t, &b),
                Err(DecodeError::TooLarge),
                "{name} case {case} (random)"
            );
            // At exactly the limit the size passes, and the next check decides.
            b.pop();
            assert_ne!(
                decode_at(t, &b),
                Err(DecodeError::TooLarge),
                "{name} case {case} (at the limit)"
            );
        }
        let _ = way;
    }
    // Above the largest message a receiving end takes on a channel, any bytes at all (an unknown type byte, or none
    // that is readable) are the size error: the catalogue's 166 B and 512 B for the room, 1,150 B, 256 B and a full
    // state's 65,536 + 4 B for a client.
    let mut g = Gen::new(0xa3c4);
    for (way, channel, max) in [
        (Direction::ClientToRoom, Channel::Datagram, 166),
        (Direction::ClientToRoom, Channel::Control, 512),
        (Direction::ClientToRoom, Channel::FullState, 4 + 510),
        (Direction::RoomToClient, Channel::Datagram, 1_150),
        (Direction::RoomToClient, Channel::Control, 256),
        (Direction::RoomToClient, Channel::FullState, 4 + 65_536),
    ] {
        for _ in 0..100 {
            let mut b = vec![0u8; max + 1];
            g.fill(&mut b);
            assert_eq!(
                decode(way, channel, &b),
                Err(DecodeError::TooLarge),
                "{way:?} {channel:?}"
            );
        }
        let unknown = vec![0xffu8; max + 1];
        assert_eq!(
            decode(way, channel, &unknown),
            Err(DecodeError::TooLarge),
            "{way:?} {channel:?}"
        );
    }
    let mut b = vec![0u8; Limits::ROOM_CONTROL + 1];
    g.fill(&mut b);
    assert_eq!(decode_control(&b), Err(DecodeError::TooLarge));
    // And the encoder never writes what the receiver refuses.
    let s = Snapshot {
        baseline_age: 0,
        entities: (0..200)
            .map(|_| EntityDelta::Add(g.record(EntityKind::Soldier)))
            .collect(),
        ..Snapshot::default()
    };
    let mut out = vec![7u8];
    assert_eq!(
        encode(&Message::Snapshot(s), &mut out),
        Err(EncodeError::TooLarge)
    );
    assert_eq!(out, [7], "nothing appended");
}

/// Eight commands, each older one changing every field its repeat can carry: the field table's maxima.
fn widest_commands() -> Vec<Command> {
    let mut out = vec![Command {
        slot: 0,
        tick: 1_000_000,
        seq: 3,
        buttons: 0xffff,
        presses: [9, 8, 7, 6],
        move_xy: [127, -127],
        aim: [POSITION_XY.max(), POSITION_XY.min(), POSITION_Z.max()],
        view_tick: 1_000_000 - 65_535,
        view_fraction: 255,
    }];
    for i in 1..Limits::REDUNDANCY_CAP {
        let n = out[i - 1];
        let tick = n.tick - 255;
        out.push(Command {
            slot: 0,
            tick,
            seq: n.seq.wrapping_sub(1),
            buttons: !n.buttons,
            presses: n.presses.map(|p| p.wrapping_add(1)),
            move_xy: n.move_xy.map(|m| m.wrapping_neg()),
            aim: [n.aim[0] - 1, n.aim[1] + 1, n.aim[2] - 1],
            view_tick: tick - (i as u32),
            view_fraction: n.view_fraction.wrapping_sub(1),
        });
    }
    out
}

fn input_bytes(commands: &[Command]) -> Vec<u8> {
    encoded(&Message::Input(Input {
        ack_tick: 65_535,
        ack_groups: 0xff,
        commands: commands.to_vec(),
    }))
    .expect("encodes")
}

#[test]
fn a6_input_whole_and_repeats_at_their_maxima_eight_within_166_b_and_trailing_bytes_refused() {
    let widest = widest_commands();
    // The field table: header 5 B, the whole command 23 B, each repeat at most 19 B.
    assert_eq!(input_bytes(&[]).len(), 5);
    for k in 1..=widest.len() {
        assert_eq!(
            input_bytes(&widest[..k]).len(),
            5 + 23 + (k - 1) * 19,
            "{k} commands"
        );
    }
    let full = input_bytes(&widest);
    assert_eq!(full.len(), 161);
    assert!(full.len() <= Limits::INPUT);
    assert_eq!(decode_at(1, &full).map(|m| m.name()), Ok("Input"));
    // A repeat is never larger than a whole command, over 10,000 random command lists.
    let mut g = Gen::new(0xa6);
    for case in 0..10_000 {
        let cs = g.commands(Limits::REDUNDANCY_CAP);
        let whole = input_bytes(&cs[..1]).len() - 5;
        assert_eq!(whole, 23);
        for k in 2..=cs.len() {
            let repeat = input_bytes(&cs[..k]).len() - input_bytes(&cs[..k - 1]).len();
            assert!(
                (2..=19).contains(&repeat) && repeat <= whole,
                "case {case}: repeat {k} is {repeat} B"
            );
        }
    }
    // Trailing bytes give TrailingBytes; a reserved mask bit or a pad bit set gives UnknownField.
    let two = input_bytes(&widest[..2]);
    let mut trailing = two.clone();
    trailing.push(0);
    assert_eq!(decode_at(1, &trailing), Err(DecodeError::TrailingBytes));
    let mut reserved = two.clone();
    reserved[5 + 23] |= 1 << 5;
    assert_eq!(decode_at(1, &reserved), Err(DecodeError::UnknownField));
    let mut pad = input_bytes(&widest[..1]);
    *pad.last_mut().expect("bytes") |= 0x80;
    assert_eq!(decode_at(1, &pad), Err(DecodeError::UnknownField));
    // A count above the cap is refused by both ends; so is a value the wire cannot carry.
    let mut nine = full.clone();
    nine[4] = 9;
    assert_eq!(decode_at(1, &nine), Err(DecodeError::OutOfRange));
    let too_many = Input {
        commands: g.commands(9),
        ..Input::default()
    };
    assert_eq!(
        encode(&Message::Input(too_many), &mut Vec::new()),
        Err(EncodeError::TooMany)
    );
    let mut c = widest[..2].to_vec();
    c[0].view_tick = c[0].tick - 65_536;
    assert_eq!(
        encode(
            &Message::Input(Input {
                commands: c,
                ..Input::default()
            }),
            &mut Vec::new()
        ),
        Err(EncodeError::OutOfRange)
    );
    let mut c = widest[..2].to_vec();
    c[1].tick = c[0].tick - 256;
    c[1].view_tick = c[1].tick;
    assert_eq!(
        encode(
            &Message::Input(Input {
                commands: c,
                ..Input::default()
            }),
            &mut Vec::new()
        ),
        Err(EncodeError::OutOfRange)
    );
    let mut c = widest[..2].to_vec();
    c[1].seq = c[0].seq;
    assert_eq!(
        encode(
            &Message::Input(Input {
                commands: c,
                ..Input::default()
            }),
            &mut Vec::new()
        ),
        Err(EncodeError::NotCanonical)
    );
    let mut c = widest[..1].to_vec();
    c[0].aim[2] = POSITION_Z.max() + 1;
    assert_eq!(
        encode(
            &Message::Input(Input {
                commands: c,
                ..Input::default()
            }),
            &mut Vec::new()
        ),
        Err(EncodeError::OutOfRange)
    );
    // The slot is the session's, never on the wire.
    let mut c = widest[..1].to_vec();
    c[0].slot = 47;
    let Ok(Message::Input(back)) = decode_at(1, &input_bytes(&c)) else {
        panic!("decodes")
    };
    assert_eq!(back.commands[0].slot, 0);
    // "No snapshot decoded" has one encoding, a group mask of 0 with a tick of 0: a tick with no group is an unknown
    // field (as Snapshot's flags rule), and the encoder never writes one.
    let none = Input {
        commands: widest[..1].to_vec(),
        ..Input::default()
    };
    let mut b = encoded(&Message::Input(none.clone())).expect("encodes");
    assert_eq!(decode_at(1, &b), Ok(Message::Input(none.clone())));
    b[1..3].copy_from_slice(&5u16.to_le_bytes());
    assert_eq!(decode_at(1, &b), Err(DecodeError::UnknownField));
    assert_eq!(
        encode(
            &Message::Input(Input {
                ack_tick: 5,
                ..none
            }),
            &mut Vec::new()
        ),
        Err(EncodeError::NotCanonical)
    );
}

/// Whether `text` shows `byte` in decimal or in hex.
fn shows(text: &str, byte: u8) -> bool {
    let text = text.to_lowercase();
    text.contains(&byte.to_string()) || text.contains(&format!("{byte:02x}"))
}

#[test]
fn debug_output_shows_a_secrets_length_never_its_bytes() {
    // The catalogue's "never the data" (§2's notes; plan E22): a `{:?}` in a log writes no test token, bind value or
    // capability, only its length. Each secret is a byte nothing else in these values prints.
    let (token, bind, cap) = (0xa7u8, 0xb3u8, 0xc9u8);
    let join = Join {
        token: [token; 16],
        bind: [bind; 16],
        ..Join::default()
    };
    let mut texts = vec![format!("{join:?}"), format!("{:?}", Message::Join(join))];
    for c in [
        SessionControl::SetProfile {
            cap: [cap; 16],
            profile: ProfileId::P2,
        },
        SessionControl::TriggerBurst { cap: [cap; 16] },
        SessionControl::BlockUdp {
            cap: [cap; 16],
            on: true,
        },
        SessionControl::RecordStep {
            cap: [cap; 16],
            step: 3,
            result: StepResult::NotYet,
            note: "ok".to_owned(),
        },
    ] {
        texts.push(format!("{c:?}"));
        texts.push(format!("{:?}", Message::SessionControl(c)));
    }
    for text in &texts {
        for secret in [token, bind, cap] {
            assert!(!shows(text, secret), "{text}");
        }
        assert!(text.contains("16 B"), "{text}");
    }
}

fn hash_case() -> (Vec<EntityRecord>, Vec<EntityRecord>, Vec<(u32, SimEvent)>) {
    let mut g = Gen::new(0xa8);
    let entities = EntityKind::ALL.iter().map(|&k| g.record(k)).collect();
    let own = vec![g.record(EntityKind::Soldier), g.record(EntityKind::Effect)];
    let events = vec![
        (70_000, SimEvent::Shot { slot: 3, seq: 9 }),
        (
            70_001,
            SimEvent::Hit {
                shooter: 3,
                seq: 9,
                victim: 4,
                damage: 25,
            },
        ),
        (
            70_002,
            SimEvent::Kill {
                killer: 3,
                victim: 4,
            },
        ),
    ];
    (entities, own, events)
}

/// hash.rs's byte string, spelled out from its module text, and FNV-1a-64 over it, written apart from the crate's: the
/// reference the records' vector is checked against (a record's bytes follow its kind's layout, which U4d completes,
/// so the vectors with records are pinned through this rather than as numbers).
fn reference(
    tick: u32,
    entities: &[EntityRecord],
    own: &[EntityRecord],
    events: &[(u32, SimEvent)],
) -> u64 {
    let mut b = tick.to_le_bytes().to_vec();
    for list in [entities, own] {
        let mut sorted = list.to_vec();
        let code = |k: EntityKind| {
            EntityKind::ALL
                .iter()
                .position(|x| *x == k)
                .expect("a kind") as u8
        };
        sorted.sort_by_key(|r| (code(r.kind), r.index, r.generation));
        b.extend((sorted.len() as u32).to_le_bytes());
        for r in &sorted {
            b.push(code(r.kind));
            b.extend(r.index.to_le_bytes());
            b.extend(r.generation.to_le_bytes());
            for g in &r.grid[..r.kind.layout().len()] {
                b.extend(g.to_le_bytes());
            }
        }
    }
    let kind = |e: &SimEvent| match e {
        SimEvent::Shot { .. } => 0u8,
        SimEvent::Hit { .. } => 1,
        SimEvent::Kill { .. } => 2,
    };
    let mut ev: Vec<(u32, u8)> = events.iter().map(|(n, e)| (*n, kind(e))).collect();
    ev.sort();
    b.extend((ev.len() as u32).to_le_bytes());
    for (n, k) in ev {
        b.extend(n.to_le_bytes());
        b.push(k);
    }
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for x in b {
        h = (h ^ u64::from(x)).wrapping_mul(0x0000_0100_0000_01b3);
    }
    if h == 0 { 1 } else { h }
}

#[test]
fn a8_client_hash_vectors_are_pinned_with_the_32_bit_tick_and_event_numbers_inside_and_never_0() {
    let (entities, own, events) = hash_case();
    let h = client_hash(65_537, &entities, &own, &events);
    // The canonical vectors: without records, as numbers; with records, as the reference's byte string.
    assert_eq!(client_hash(0, &[], &[], &[]), PINNED_EMPTY);
    assert_eq!(client_hash(65_537, &[], &[], &events), PINNED_EVENTS);
    assert_eq!(reference(0, &[], &[], &[]), PINNED_EMPTY);
    assert_eq!(h, reference(65_537, &entities, &own, &events), "{h:#018x}");
    // The full 32-bit tick and event numbers are inside: 16 bits of either would not tell these apart.
    assert_ne!(client_hash(1, &entities, &own, &events), h);
    let mut wrapped = events.clone();
    wrapped[1].0 += 65_536;
    assert_ne!(client_hash(65_537, &entities, &own, &wrapped), h);
    // So are each record's integers, and the own-state section apart from the entity set.
    let mut moved = entities.clone();
    moved[0].grid[0] = moved[0].grid[0].wrapping_add(1);
    assert_ne!(client_hash(65_537, &moved, &own, &events), h);
    assert_ne!(client_hash(65_537, &own, &entities, &events), h);
    // Order does not matter (entity-id order, number order); the room's own fields do not either.
    let mut shuffled = entities.clone();
    shuffled.reverse();
    let mut late = events.clone();
    late.rotate_left(1);
    let mut roomside = own.clone();
    roomside[0].area = 9;
    roomside[0].raw[0] = 0x3f80_0000;
    assert_eq!(client_hash(65_537, &shuffled, &roomside, &late), h);
    // Never 0: over 100,000 random views.
    let mut g = Gen::new(0xa80);
    for _ in 0..100_000 {
        let k = g.kind();
        let e = [g.record(k)];
        assert_ne!(client_hash(g.u32(), &e, &[], &[(g.u32(), g.event())]), 0);
    }
}

/// The two vectors without records, each computed apart by a Python FNV-1a-64 over the byte string (u4a-build.md):
/// 16 zero bytes; and tick 65,537, no records, the three events numbered 70,000 to 70,002 (shot, hit, kill).
const PINNED_EMPTY: u64 = 0x8820_1fb9_60ff_6465;
const PINNED_EVENTS: u64 = 0x8d9e_8dda_afef_41e8;
