//! The room and the endpoint take whatever the link hands them (ADR 0001 part 5): each is fed valid, changed and
//! random messages between its steps, from a fixed seed, and must not panic, must keep its own rules, and must go
//! on writing valid messages. The decoders have their own, larger test (`protocol/tests/decoder_inputs.rs`); this
//! one is for the checks behind them: the acks, the queue, the press check, the echo check and the join.
//!
//! Here and not in `room` or `client-endpoint`, because this crate is where both sides and the codec meet.

use client_endpoint::endpoint::{Endpoint, EndpointState};
use client_endpoint::layout::LAYOUT_HASH;
use client_endpoint::post::Post;
use protocol::codec::{
    Control, KIND_INPUT, KIND_SNAPSHOT, decode_control, decode_input, decode_snapshot,
    encode_control, encode_input, encode_snapshot,
};
use protocol::messages::{
    AimPoint, Command, Input, Join, JoinAccepted, JoinRejected, OwnState, PROTOCOL_VERSION,
    REDUNDANCY_CAP, RejectReason, Snapshot, SnapshotAck, ViewTime,
};
use protocol::wrap::press_edges;
use room::{CATCH_UP_CAP, Room};
use sim::{BUTTONS, MatchSetup, SEED_BYTES};

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

/// The room against a client that keeps the rules, lies in one field of one message in five, and whose bytes are
/// changed on the way one time in five. Whatever arrives: every snapshot the room writes is valid and one tick
/// after the last; at most `CATCH_UP_CAP` commands run in a tick, and never the one that ran last; and no button
/// is pressed more often than once per tick since the join.
#[test]
fn the_room_keeps_its_rules_whatever_arrives() {
    let mut random = Random(0x3B);
    let join = control_bytes(&Control::Join(Join {
        token: Vec::new(),
        protocol_version: PROTOCOL_VERSION,
        content_hash: HASH,
    }));
    let (mut bytes, mut reply, mut written) = (Vec::new(), Vec::new(), Vec::new());
    let (acks, commands) = (Vec::new(), Vec::new());
    let mut input = Input { acks, commands };
    let (mut totals, mut pressed) = ([0u64; 11], 0u64);
    for life in 0..MATCHES {
        // The first match crosses a multiple of 65,536 ticks, and the sequences start somewhere else each time.
        let (seed, start_tick) = ([3; SEED_BYTES], 65_000 + 100_000 * life);
        let mut room = Room::new(&MatchSetup { seed, start_tick }, HASH);
        // The client: its newest commands, the newest first, and what the next one continues from.
        let mut sent: Vec<Command> = Vec::new();
        let (mut sequence, mut next_tick) = ((life * 7919) as u16, start_tick);
        let mut presses = [0u8; BUTTONS];
        // What the snapshots said: the tick of the first, the last one, and the presses granted per button.
        let (mut first, mut last, mut granted) = (None, None::<Snapshot>, [0u64; BUTTONS]);
        for call in 0..CALLS {
            let tick = room.world().tick();
            match random.below(100) {
                // A client ahead of the room waits; one behind catches up.
                0..=34 if next_tick <= tick + 4 => {
                    (sequence, next_tick) = (sequence.wrapping_add(1), next_tick.max(tick) + 1);
                    for count in &mut presses {
                        *count = count.wrapping_add(u8::from(random.chance(15)));
                    }
                    sent.truncate(REDUNDANCY_CAP - 1);
                    sent.insert(
                        0,
                        Command {
                            tick: next_tick,
                            sequence,
                            move_x: random.next() as i8,
                            move_y: random.next() as i8,
                            buttons: random.next() as u16,
                            presses,
                            aim: AimPoint { x: 1, y: 2, z: 3 },
                            view_time: ViewTime { tick, fraction: 0 },
                        },
                    );
                    let carried = 1 + random.below(sent.len() as u64) as usize;
                    input.commands.clear();
                    input.commands.extend_from_slice(&sent[..carried]);
                    input.acks.clear();
                    let seen = last.as_ref().map(|last| last.server_tick);
                    input
                        .acks
                        .extend(seen.map(|tick| SnapshotAck { group: 0, tick }));
                    let lied = random.below(carried as u64) as usize;
                    match (random.below(25), &mut input.commands[lied]) {
                        (0, command) => command.sequence = random.near(sequence.into(), 20) as u16,
                        (1, command) => command.tick = random.near(command.tick, 12),
                        (2, command) => random.press(&mut command.presses),
                        (3, _) => input.acks.push(SnapshotAck {
                            group: random.next() as u8 & 1,
                            tick: random.near(tick, 3),
                        }),
                        (4, _) => input.commands.truncate(lied),
                        _ => {}
                    }
                    if encode_input(&input, &mut bytes).is_err() {
                        random.fill(&mut bytes, KIND_INPUT);
                    }
                    if random.chance(20) {
                        random.change(&mut bytes);
                    }
                    room.on_datagram(&bytes);
                }
                0..=42 => {
                    random.fill(&mut bytes, KIND_INPUT);
                    room.on_datagram(&bytes);
                }
                43..=47 => {
                    bytes.clone_from(&join);
                    if random.chance(60) {
                        random.change(&mut bytes);
                    }
                    if room.on_control(&bytes, &mut reply) {
                        decode_control(&reply).expect("the room writes a valid answer");
                    }
                }
                _ => {
                    let ran_before = room.counters().commands_run;
                    if !room.step(&mut written) {
                        continue;
                    }
                    let at = format!("match {life}, call {call}");
                    let now = decode_snapshot(&written).expect("the room writes a valid snapshot");
                    assert_eq!(now.server_tick, tick + 1, "{at}");
                    let ran = room.counters().commands_run - ran_before;
                    assert!(
                        ran as usize <= CATCH_UP_CAP,
                        "{at}: {ran} commands in one tick"
                    );
                    let own = now.own.as_ref().expect("the player's own state");
                    let before = last.as_ref();
                    if ran > 0 {
                        let processed = before.map(|before| before.last_processed_command);
                        assert_ne!(Some(now.last_processed_command), processed, "{at}");
                    }
                    let echoed = before.and_then(|before| before.own.as_ref());
                    let echoed = echoed.map_or([0; BUTTONS], |own| own.presses);
                    let ticks = u64::from(now.server_tick - *first.get_or_insert(tick));
                    for ((granted, old), new) in granted.iter_mut().zip(echoed).zip(own.presses) {
                        *granted += u64::from(press_edges(old, new));
                        assert!(
                            *granted <= ticks,
                            "{at}: {granted} presses in {ticks} ticks"
                        );
                    }
                    last = Some(now);
                }
            }
        }
        let counters = room.counters();
        let counted = [
            counters.commands_run,
            counters.late,
            counters.joins,
            counters.rejections,
            counters.decode_errors,
            counters.refused_ack,
            counters.refused_ahead,
            counters.refused_not_newer,
            counters.refused_presses,
            counters.duplicates,
            counters.stale,
        ];
        for (total, count) in totals.iter_mut().zip(counted) {
            *total += u64::from(count);
        }
        pressed += granted.iter().sum::<u64>();
    }
    // The run reached both sides of every check: commands ran in number, on time and late, presses were
    // granted, every match was joined once, and each kind of refusal came up.
    let [ran, late, joins, turned_away @ ..] = totals;
    println!(
        "the room: {ran} commands run ({late} late), {pressed} presses granted, turned away {turned_away:?}"
    );
    assert!(
        ran > 30_000 && late > 100 && pressed > 30_000,
        "{totals:?} {pressed}"
    );
    assert_eq!(joins, u64::from(MATCHES));
    assert!(turned_away.iter().all(|count| *count > 100), "{totals:?}");
}

/// The endpoint against a room that keeps the rules and lies in one field of one snapshot in five, with bytes that
/// are seldom changed on the way, and a page that posts anything. Whatever arrives: every input it writes is valid, carries the next sequence for the next tick
/// first and at most the cap, moves no press counter by more than one, and the block stays whole.
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
        assert_eq!(readout.unsent, 0);
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
