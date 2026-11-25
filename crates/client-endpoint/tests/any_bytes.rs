//! The client endpoint takes whatever the link and the page hand it (ADR 0001 part 5): it is fed valid, changed
//! and random snapshots and join answers between its commands, from a fixed seed, and must not panic, must keep
//! its own rules, and must go on writing valid inputs. The decoders have their own, larger test
//! (`protocol/tests/decoder_inputs.rs`); this one is for what lies behind them: the join, the echo check, the
//! repeats and the press pacer. The room has the same kind of test against a client that lies
//! (`room/tests/any_bytes.rs`).

use client_endpoint::endpoint::{Endpoint, EndpointState};
use client_endpoint::layout::LAYOUT_HASH;
use client_endpoint::post::Post;
use protocol::codec::{Control, KIND_SNAPSHOT, decode_input, encode_control, encode_snapshot};
use protocol::messages::{
    Command, Input, JoinAccepted, JoinRejected, OwnState, REDUNDANCY_CAP, RejectReason, Snapshot,
};
use protocol::wrap::press_edges;
use sim::BUTTONS;

const HASH: [u8; 32] = [0xC4; 32];
/// Each test runs this many matches of this many calls, about a second in a debug build: a match that a lie has
/// spoilt (a sequence far ahead that ran, a rejected join) ends with its match, and the next one starts clean.
const MATCHES: u32 = 100;
const CALLS: u32 = 3_000;

/// splitmix64: a fixed stream, so a failure names its call and comes again.
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mixed = (self.0 ^ (self.0 >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        let mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        mixed ^ (mixed >> 31)
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }

    fn chance(&mut self, per_100: u64) -> bool {
        self.below(100) < per_100
    }

    /// Within `spread` of `base` either way; one time in ten anywhere.
    fn near(&mut self, base: u32, spread: u32) -> u32 {
        let anywhere = self.next() as u32;
        let by = self.below(2 * u64::from(spread) + 1) as u32;
        if self.chance(10) {
            anywhere
        } else {
            base.wrapping_sub(spread).wrapping_add(by)
        }
    }

    /// A sequence or none, as a snapshot carries them.
    fn sequence(&mut self, near: u16) -> Option<u16> {
        let sequence = self.near(u32::from(near), 4) as u16;
        self.chance(90).then_some(sequence)
    }

    /// Changes the bytes a little: a bit, a byte, a cut, a byte more.
    fn change(&mut self, bytes: &mut Vec<u8>) {
        for _ in 0..=self.below(3) {
            let at = self.below(bytes.len().max(1) as u64) as usize;
            match (self.below(4), bytes.get_mut(at)) {
                (0, Some(byte)) => *byte ^= 1 << self.below(8),
                (1, Some(byte)) => *byte = self.next() as u8,
                (2, _) => bytes.truncate(at),
                _ => bytes.insert(at, self.next() as u8),
            }
        }
    }

    /// Up to 300 random bytes, half the time behind this kind byte.
    fn fill(&mut self, bytes: &mut Vec<u8>, kind: u8) {
        bytes.clear();
        for _ in 0..self.below(300) {
            bytes.push(self.next() as u8);
        }
        if let (true, Some(first)) = (self.chance(50), bytes.first_mut()) {
            *first = kind;
        }
    }

    /// Moves some press counts by a little, and now and then anywhere.
    fn press(&mut self, presses: &mut [u8; BUTTONS]) {
        for count in presses {
            match self.below(100) {
                0..=14 => *count = count.wrapping_add(self.below(3) as u8),
                15 => *count = self.next() as u8,
                _ => {}
            }
        }
    }
}

fn control_bytes(control: &Control) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_control(control, &mut bytes).expect("it has a wire form");
    bytes
}

/// The endpoint against a room that keeps the rules and lies in one field of one snapshot in five, with bytes
/// that are seldom changed on the way, and a page that posts anything. Whatever arrives: every input it writes
/// is valid, carries the next sequence for the next tick first and at most the cap, moves no press counter by
/// more than one, and the block stays whole.
#[test]
fn the_endpoint_keeps_its_rules_whatever_arrives() {
    let mut random = Random(0xE9);
    let answers = [
        control_bytes(&Control::JoinAccepted(JoinAccepted {
            slot: 0,
            team: 0,
            squad: 0,
            mode_id: "skeleton".to_owned(),
            mode_hash: [0; 32],
            content_hash: HASH,
            server_tick: 9,
            tick_rate_hz: 30,
            snapshot_rate_hz: 30,
        })),
        control_bytes(&Control::JoinRejected(JoinRejected {
            reason: RejectReason::RoomFull,
        })),
    ];
    let (mut bytes, mut written) = (Vec::new(), Vec::new());
    let (acks, commands) = (Vec::new(), Vec::new());
    let mut input = Input { acks, commands };
    let (mut built, mut rejected, mut counts) = (0u32, 0u32, [0u64; 5]);
    for life in 0..MATCHES {
        let first_sequence = (life * 7919) as u16;
        let mut endpoint = Endpoint::new(HASH, first_sequence, 0).expect("no padding is allowed");
        let (mut post, mut server_tick) = (Post::NONE, 1_000_000 * life);
        // The newest command of the input built last.
        let mut last: Option<Command> = None;
        for call in 0..CALLS {
            if matches!(endpoint.state(), EndpointState::Rejected(_)) {
                rejected += 1;
                break;
            }
            let at = format!("match {life}, call {call}");
            let sequence = last.as_ref().map_or(first_sequence, |last| last.sequence);
            match random.below(100) {
                0..=34 => {
                    post.number = random.near(post.number, 2);
                    post.buttons = random.next() as u16;
                    random.press(&mut post.presses);
                    endpoint.set_post(post);
                    if !endpoint.build_input(&mut written) {
                        continue;
                    }
                    decode_input(&written, &mut input).expect("the endpoint writes a valid input");
                    let carried = input.commands.len();
                    assert!(
                        (1..=REDUNDANCY_CAP).contains(&carried),
                        "{at}: {carried} commands"
                    );
                    let newest = input.commands.swap_remove(0);
                    if let Some(last) = &last {
                        assert_eq!(newest.sequence, last.sequence.wrapping_add(1), "{at}");
                        assert_eq!(newest.tick, last.tick + 1, "{at}");
                        let moved = last.presses.iter().zip(newest.presses);
                        let most = moved.map(|(old, new)| press_edges(*old, new)).max();
                        assert!(most <= Some(1), "{at}: a counter moved by {most:?}");
                    }
                    (last, built) = (Some(newest), built + 1);
                }
                35..=74 => {
                    server_tick += 1;
                    let mut snapshot = Snapshot {
                        server_tick,
                        baseline_tick: None,
                        newest_command_received: Some(sequence),
                        last_processed_command: Some(sequence.wrapping_sub(1)),
                        group: 0,
                        own: Some(OwnState {
                            buttons: random.next() as u16,
                            presses: last.as_ref().map_or([0; BUTTONS], |last| last.presses),
                        }),
                    };
                    match random.below(25) {
                        0 => snapshot.server_tick = random.near(server_tick, 3),
                        1 => snapshot.newest_command_received = random.sequence(sequence),
                        2 => snapshot.last_processed_command = random.sequence(sequence),
                        3 => {
                            snapshot.own = random.chance(50).then(|| OwnState {
                                buttons: 0,
                                presses: [random.next() as u8; BUTTONS],
                            })
                        }
                        4 => snapshot.group = random.next() as u8,
                        _ => {}
                    }
                    if encode_snapshot(&snapshot, &mut bytes).is_err() {
                        random.fill(&mut bytes, KIND_SNAPSHOT);
                    }
                    // Seldom: one changed bit in the tick can put it far ahead, and every snapshot after that
                    // one is late for the rest of the match.
                    if random.chance(3) {
                        random.change(&mut bytes);
                    }
                    endpoint.on_datagram(&bytes);
                }
                75..=84 => {
                    random.fill(&mut bytes, KIND_SNAPSHOT);
                    endpoint.on_datagram(&bytes);
                }
                85..=94 => {
                    bytes.clone_from(&answers[usize::from(random.chance(30))]);
                    if random.chance(60) {
                        random.change(&mut bytes);
                    }
                    let _ = endpoint.on_control(&bytes);
                }
                _ => {
                    endpoint.write_block(random.next() as u32);
                    assert_eq!(endpoint.block()[4..8], LAYOUT_HASH.to_le_bytes(), "{at}");
                }
            }
        }
        let readout = endpoint.readout();
        assert_eq!(readout.unwritten, 0);
        let counted = [
            readout.received,
            readout.late,
            readout.refused,
            readout.presses_lost,
            readout.presses_doubled,
        ];
        for (total, count) in counts.iter_mut().zip(counted) {
            *total += u64::from(count);
        }
    }
    // The run did its work: many inputs were built, some matches were refused at the join, and snapshots were
    // taken, came late, were refused, and echoed too few presses and too many.
    println!(
        "the endpoint: {built} inputs built, {rejected} joins rejected, snapshots and presses {counts:?}"
    );
    assert!(
        built > 30_000 && rejected > 10,
        "{built} inputs, {rejected} rejections"
    );
    assert!(counts.iter().all(|count| *count > 1000), "{counts:?}");
}
