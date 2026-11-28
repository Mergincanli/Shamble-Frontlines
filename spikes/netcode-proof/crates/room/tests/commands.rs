//! U4f's room tests of the command checks (u4f-brief.md 2.3, 2.4, 2.10): option A beside option B, 2.4's order with
//! one kind per refusal, the kept window, the join anchor and the slot's kept stamp across joins, the signed-distance
//! compares at room tick 1, `tick.s.cmd_on_time` by stamp, and (bites builds) each -21 room bite's unit proof. Every
//! case runs `Room<World>` through its own API with hand-made `Input`s (no endpoint).

use npt_proto::{Input, Join, Message};
use npt_room::{CmdTiming, Room};
use npt_sim::{Command, Scene, SimCore, World};

const BOTH: [CmdTiming; 2] = [CmdTiming::A, CmdTiming::B];

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

/// A course-scene room (one slot) under `timing` whose first tick is `first`, its slot joined: (room, join tick).
fn room(timing: CmdTiming, first: u32) -> (Room<World>, u32) {
    room_at(timing, first, |r| r)
}

fn room_at(
    timing: CmdTiming,
    first: u32,
    prep: impl FnOnce(Room<World>) -> Room<World>,
) -> (Room<World>, u32) {
    let scene = Scene::course();
    let world = World::new(&scene, 1, first);
    let greybox = world.greybox().clone();
    let r = Room::new(world, &scene, &greybox, vec![token(0)], 1, 30).with_cmd_timing(timing);
    let mut r = prep(r);
    let at = r.last_tick().wrapping_add(1);
    assert_eq!(r.join(&join(0)).map(|j| j.0), Ok(0));
    (r, at)
}

/// A plain command of slot 0: sequence `seq`, stamped `tick`, its view the tick before.
fn cmd(seq: u16, tick: u32) -> Command {
    Command {
        tick,
        seq,
        view_tick: tick.wrapping_sub(1),
        ..Command::default()
    }
}

/// One `Input` with `oldest_first`'s commands (the wire's order is newest first), no acknowledgement.
fn send(room: &mut Room<World>, oldest_first: &[Command]) {
    let commands = oldest_first.iter().rev().copied().collect();
    room.receive(
        0,
        &Message::Input(Input {
            commands,
            ..Input::default()
        }),
    );
}

/// One tick: the sequences of slot 0's commands it ran, in order.
fn tick(room: &mut Room<World>) -> Vec<u16> {
    room.tick();
    room.take_datagrams(0);
    room.take_control(0);
    room.tick_input()
        .commands
        .iter()
        .filter(|(c, _)| c.slot == 0)
        .map(|(c, _)| c.seq)
        .collect()
}

/// The refusal records of the receives before the last tick, as (seq, kind number).
fn refused(room: &Room<World>) -> Vec<(u16, u8)> {
    room.refusals()
        .iter()
        .map(|r| (r.seq, r.kind.kind()))
        .collect()
}

fn counters(room: &Room<World>) -> npt_room::ClientCounters {
    room.client(0)
        .map(|c| c.counters().clone())
        .unwrap_or_default()
}

/// The forged-kind counters, in 2.4's order of kinds 1, 2, 3, 4, 5, 6, 8 (datagrams) and 10.
fn forged(k: &npt_room::ClientCounters) -> [u64; 8] {
    [
        k.lead_refused,
        k.not_newer,
        k.seq_refused,
        k.move_refused,
        k.press_refused,
        k.aim_refused,
        k.ack_refused,
        k.buffer_full,
    ]
}

/// f7: option A runs several commands of one slot in a tick; the 9th for the coming tick is refused on arrival and
/// counted as kind 10, and the room's list for the coming tick never holds more than 8 under a flood of 3 datagrams of 8.
#[test]
fn f7_option_a_runs_several_commands_of_a_slot_in_a_tick_and_caps_the_coming_tick_at_8() {
    let (mut r, j) = room(CmdTiming::A, 100);
    // Idle 20 ticks, so 8 stamps behind the next tick fit the past window and the rest inside the lead bound.
    for _ in 0..20 {
        tick(&mut r);
    }
    let next = r.last_tick().wrapping_add(1);
    assert_eq!(next, j + 20);
    let mut seq = 0u16;
    for d in 0..3u32 {
        let batch: Vec<Command> = (0..8u32)
            .map(|i| {
                seq += 1;
                cmd(seq, next - 7 + d * 8 + i)
            })
            .collect();
        send(&mut r, &batch);
    }
    let ran = tick(&mut r);
    assert_eq!(
        ran,
        (1..=8).collect::<Vec<u16>>(),
        "the first 8 run in one tick"
    );
    let k = counters(&r);
    // Datagram 2's 8 and datagram 3's stamps inside the bound (up to next + 12) are kind 10; the rest kind 1.
    assert_eq!(k.buffer_full, 8 + 4, "{k:?}");
    assert_eq!(k.lead_refused, 4, "{k:?}");
    assert!(refused(&r).contains(&(9, 10)), "the 9th is kind 10");
}

/// f8 (room half): SA-1's triple on a fresh slot under option A (ticks T - 2^31 + 1, T - 2^30 and T + 1 in one
/// datagram): no panic, the two before the join are kind 2, the third runs; and under no option does a slot's tick list
/// hold two ticks 2^31 apart unless a bite let them in (the canonical key's own test is in room.rs).
#[test]
fn f8_sa_1s_triple_on_a_fresh_slot_under_option_a_gives_one_order_and_no_panic() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 500);
        let triple = [
            cmd(1, j.wrapping_sub(1 << 31).wrapping_add(1)),
            cmd(2, j.wrapping_sub(1 << 30)),
            cmd(3, j.wrapping_add(1)),
        ];
        send(&mut r, &triple);
        let mut ran = tick(&mut r);
        ran.extend(tick(&mut r));
        assert_eq!(ran.iter().filter(|s| **s == 3).count(), 1, "{timing:?}");
        assert!(!ran.contains(&1) && !ran.contains(&2), "{timing:?}");
        assert_eq!(counters(&r).not_newer, 2, "{timing:?}");
    }
}

/// f10, kind 1: a command stamped more than the lead bound (400 ms: 12 ticks at 30 Hz) ahead of the room's next tick is
/// refused and counted as kind 1; one at the bound runs; under both options.
#[test]
fn f10_kind_1_a_tick_past_the_lead_bound_is_refused_and_counted() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        assert_eq!(r.lead_bound(), (400, 12));
        send(&mut r, &[cmd(1, j + 13)]);
        send(&mut r, &[cmd(2, j + 12)]);
        tick(&mut r);
        assert_eq!(refused(&r), [(1, 1)], "{timing:?}");
        assert_eq!(forged(&counters(&r)), [1, 0, 0, 0, 0, 0, 0, 0]);
    }
}

/// f10, kind 2: a stamp equal to or older than the newest accepted is refused and counted as kind 2, never late.
#[test]
fn f10_kind_2_a_tick_not_newer_than_the_newest_accepted_is_refused_and_counted() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        send(&mut r, &[cmd(1, j + 3), cmd(2, j + 3), cmd(3, j + 2)]);
        tick(&mut r);
        assert_eq!(refused(&r), [(2, 2), (3, 2)], "{timing:?}");
        let k = counters(&r);
        assert_eq!((forged(&k), k.late), ([0, 2, 0, 0, 0, 0, 0, 0], 0));
    }
}

/// f10, kind 3: a resent copy, with changed content, of a command the room accepted is refused as kind 3; the honest
/// command it copies runs once.
#[test]
fn f10_kind_3_a_changed_copy_of_an_accepted_command_is_refused_and_counted() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        send(&mut r, &[cmd(1, j + 1), cmd(2, j + 2)]);
        let mut copy = cmd(1, j + 1);
        copy.move_xy = [40, 0];
        send(&mut r, &[copy, cmd(2, j + 2)]);
        let mut ran = Vec::new();
        let mut moves = Vec::new();
        for i in 0..3 {
            ran.extend(tick(&mut r));
            if i == 0 {
                assert_eq!(refused(&r), [(1, 3)], "{timing:?}");
            }
            moves.extend(r.tick_input().commands.iter().map(|c| c.0.move_xy));
        }
        let k = counters(&r);
        assert_eq!(forged(&k), [0, 0, 1, 0, 0, 0, 0, 0], "{timing:?}");
        assert_eq!(k.commands_repeat, 1, "the second's copy is a repeat");
        assert_eq!(ran.iter().filter(|s| **s == 1).count(), 1);
        assert!(!moves.contains(&[40, 0]), "the forged copy never runs");
    }
}

/// f10, kind 4: a move longer than 1 is refused and counted as kind 4, [-128, -128] and [-128, 0] among them, with no
/// panic in a debug or test build (each part widened before it is squared); [127, 0] and [-89, -89] pass.
#[test]
fn f10_kind_4_a_long_move_is_refused_and_counted_with_no_overflow() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        let moves = [[-128, -128], [-128, 0], [127, 127], [127, 0], [-89, -89]];
        let cmds: Vec<Command> = moves
            .iter()
            .zip(1u16..)
            .map(|(m, s)| Command {
                move_xy: *m,
                ..cmd(s, j + u32::from(s))
            })
            .collect();
        send(&mut r, &cmds);
        tick(&mut r);
        assert_eq!(refused(&r), [(1, 4), (2, 4), (3, 4)], "{timing:?}");
        assert_eq!(forged(&counters(&r)), [0, 0, 0, 3, 0, 0, 0, 0]);
    }
}

/// f10, kind 5: more press edges than the ticks a command covers is refused and counted as kind 5.
#[test]
fn f10_kind_5_more_press_edges_than_ticks_covered_is_refused_and_counted() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        let mut two = cmd(2, j + 2);
        two.presses[Command::PRESS_FIRE] = 2;
        let mut one = cmd(3, j + 2);
        one.presses[Command::PRESS_FIRE] = 1;
        send(&mut r, &[cmd(1, j + 1), two, one]);
        tick(&mut r);
        assert_eq!(refused(&r), [(2, 5)], "{timing:?}");
        assert_eq!(forged(&counters(&r)), [0, 0, 0, 0, 1, 0, 0, 0]);
    }
}

/// f10, kind 6: an aim outside the map's x or y extent (±12,800 cm) is refused and counted as kind 6; the edge passes.
#[test]
fn f10_kind_6_an_aim_outside_the_map_is_refused_and_counted() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        let aims = [[12_801, 0, 0], [0, -12_801, 0], [12_800, -12_800, 30_000]];
        let cmds: Vec<Command> = aims
            .iter()
            .zip(1u16..)
            .map(|(a, s)| Command {
                aim: *a,
                ..cmd(s, j + u32::from(s))
            })
            .collect();
        send(&mut r, &cmds);
        tick(&mut r);
        assert_eq!(refused(&r), [(1, 6), (2, 6)], "{timing:?}");
        assert_eq!(forged(&counters(&r)), [0, 0, 0, 0, 0, 2, 0, 0]);
    }
}

/// f10, kind 8: a datagram whose acknowledgement names a snapshot never sent has its commands refused whole and is
/// counted once as kind 8 (by datagram), each command recorded.
#[test]
fn f10_kind_8_an_ack_never_sent_refuses_the_datagram_and_is_counted() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        r.receive(
            0,
            &Message::Input(Input {
                ack_tick: 7,
                ack_groups: 1,
                commands: vec![cmd(2, j + 2), cmd(1, j + 1)],
            }),
        );
        let ran = tick(&mut r);
        assert!(ran.is_empty(), "{timing:?}");
        assert_eq!(refused(&r), [(1, 8), (2, 8)]);
        let k = counters(&r);
        assert_eq!(
            (forged(&k), k.ack_refused_commands),
            ([0, 0, 0, 0, 0, 0, 1, 0], 2)
        );
    }
}

/// f10, kind 10: under option B more commands than the buffer holds (8), under option A more than 8 for one tick, are
/// refused and counted as kind 10, all inside the lead bound.
#[test]
fn f10_kind_10_more_commands_than_the_catch_up_cap_is_refused_and_counted() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        let cmds: Vec<Command> = (1..=10u16).map(|s| cmd(s, j + u32::from(s))).collect();
        send(&mut r, &cmds);
        tick(&mut r);
        assert_eq!(refused(&r), [(9, 10), (10, 10)], "{timing:?}");
        assert_eq!(forged(&counters(&r)), [0, 0, 0, 0, 0, 0, 0, 2]);
    }
}

/// f11 (i): one kind per refusal: a command breaking kinds 1, 4 and 6 is counted once, under kind 1 (the first).
#[test]
fn f11_a_command_breaking_two_checks_is_counted_once_under_the_first() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        let bad = Command {
            move_xy: [127, 127],
            aim: [20_000, 0, 0],
            ..cmd(1, j + 40)
        };
        let worse = Command {
            move_xy: [127, 127],
            aim: [20_000, 0, 0],
            ..cmd(2, j + 1)
        };
        send(&mut r, &[bad, worse]);
        tick(&mut r);
        assert_eq!(refused(&r), [(1, 1), (2, 4)], "{timing:?}");
        assert_eq!(forged(&counters(&r)), [1, 0, 0, 1, 0, 0, 0, 0]);
    }
}

/// f11 (ii): an honest late command (its stamp newer than the newest accepted, but for a tick already run under B, or
/// more than W = 8 ticks behind under A) is counted `late`, never kind 2; and a command as late that also breaks kind 4
/// (a long move), or kind 6 (an off-map aim), is counted under its forged kind, never `late` (the advisor's R2 (b)).
#[test]
fn f11_an_honest_late_command_is_counted_late_never_kind_2() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        send(&mut r, &[cmd(1, j + 1)]);
        for _ in 0..12 {
            tick(&mut r);
        }
        let behind = if timing == CmdTiming::B { 2 } else { 3 };
        let long = Command {
            move_xy: [127, 127],
            ..cmd(3, j + behind)
        };
        let off = Command {
            aim: [0, 12_801, 0],
            ..cmd(4, j + behind)
        };
        send(&mut r, &[cmd(2, j + behind), long, off]);
        let ran = tick(&mut r);
        assert!(!ran.iter().any(|s| (2..=4).contains(s)), "{timing:?}");
        assert_eq!(refused(&r), [(3, 4), (4, 6)], "{timing:?}");
        let k = counters(&r);
        assert_eq!((k.late, k.not_newer), (1, 0), "{timing:?}: {k:?}");
        assert_eq!(forged(&k), [0, 0, 0, 1, 0, 1, 0, 0], "{timing:?}");
    }
}

/// f12 (i): an honest redundant copy is refused under no kind (a repeat).
#[test]
fn f12_an_honest_redundant_copy_is_a_repeat_under_no_kind() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        send(&mut r, &[cmd(1, j + 1)]);
        send(&mut r, &[cmd(1, j + 1), cmd(2, j + 2)]);
        send(&mut r, &[cmd(1, j + 1), cmd(2, j + 2), cmd(3, j + 3)]);
        tick(&mut r);
        let k = counters(&r);
        assert_eq!((forged(&k), k.commands_repeat), ([0; 8], 3), "{timing:?}");
    }
}

/// f12 (ii): a backward sequence with content other than the accepted command kept under it, inside the window, is
/// kind 3; a later copy of a refused sequence with other content is a repeat.
#[test]
fn f12_a_changed_backward_copy_is_kind_3_and_a_copy_of_a_refused_sequence_is_a_repeat() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        // Sequence 2 is refused (kind 4); sequence 1 accepted.
        let long = Command {
            move_xy: [127, 127],
            ..cmd(2, j + 2)
        };
        send(&mut r, &[cmd(1, j + 1), long, cmd(3, j + 3)]);
        let changed1 = Command {
            aim: [500, 0, 0],
            ..cmd(1, j + 1)
        };
        let changed2 = Command {
            move_xy: [10, 0],
            ..cmd(2, j + 2)
        };
        send(&mut r, &[changed1, changed2]);
        tick(&mut r);
        let k = counters(&r);
        assert_eq!(
            (k.seq_refused, k.move_refused, k.commands_repeat),
            (1, 1, 1),
            "{timing:?}"
        );
    }
}

/// splitmix64 (bots/src/lib.rs's step), for the seeded traces.
fn mix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// f12 (iii): an honest P3-like trace, redundancy 8 (every `Input` carries the last 8 commands), 5 % of datagrams
/// lost and adjacent datagrams swapped about 5 % of the time (one held back a tick), stamped 4 ahead, gives 0
/// refusals of any kind under both options. Seed 0x5eed_0f12, 3,000 ticks.
#[test]
fn f12_an_honest_lossy_reordered_trace_with_redundancy_8_gives_no_refusal() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        let mut s: u64 = 0x5eed_0f12;
        let mut held: Option<Vec<Command>> = None;
        let mut made: Vec<Command> = Vec::new();
        for n in 0..3_000u32 {
            let next = r.last_tick().wrapping_add(1);
            let seq = n as u16 + 1;
            made.push(cmd(seq, next + 4));
            let from = made.len().saturating_sub(8);
            let datagram = made[from..].to_vec();
            let roll = mix(&mut s) % 100;
            if roll < 5 {
                // lost
            } else if roll < 10 && held.is_none() {
                held = Some(datagram);
            } else {
                send(&mut r, &datagram);
                if let Some(h) = held.take() {
                    send(&mut r, &h);
                }
            }
            tick(&mut r);
        }
        let k = counters(&r);
        assert_eq!(forged(&k), [0; 8], "{timing:?}: {k:?}");
        assert_eq!((k.rejoin_overlap, k.late), (0, 0), "{k:?}");
        // An overtaken datagram's one command the overtaking one lacked (after a long loss) is outside the window by
        // design, never forged: reported.
        println!(
            "f12 (iii) {timing:?}: seq_outside_window {}",
            k.seq_outside_window
        );
        assert!(k.commands_run > 2_900, "{k:?}");
        let _ = j;
    }
}

/// f12 (iv): a sequence older than the kept window (K = 16) is dropped and counted `seq_outside_window`, never forged;
/// the edge by sequence (TE-U4F-2): with 1 to 20 offered the window keeps 5 to 20, so a changed copy of 5 (the oldest
/// kept) is kind 3 and one of 4 is outside the window.
#[test]
fn f12_a_sequence_older_than_the_window_is_outside_the_window_never_forged() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        for s in 1..=20u16 {
            send(&mut r, &[cmd(s, j + u32::from(s))]);
            tick(&mut r);
        }
        for s in [5, 4] {
            let changed = Command {
                move_xy: [5, 5],
                ..cmd(s, j + u32::from(s))
            };
            send(&mut r, &[changed]);
        }
        tick(&mut r);
        let k = counters(&r);
        assert_eq!(
            (forged(&k), k.seq_outside_window),
            ([0, 0, 1, 0, 0, 0, 0, 0], 1),
            "{timing:?}"
        );
    }
}

/// f12 (v): a sequence inside the window with no command kept under it (an overtaken datagram carrying the one
/// sequence the overtaking one lacked) is counted `seq_outside_window`, never kind 3.
#[test]
fn f12_a_gap_inside_the_window_is_outside_the_window_never_kind_3() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        send(&mut r, &[cmd(1, j + 1)]);
        send(&mut r, &[cmd(3, j + 3), cmd(4, j + 4)]);
        let gap = Command {
            move_xy: [9, 9],
            ..cmd(2, j + 2)
        };
        send(&mut r, &[gap]);
        tick(&mut r);
        let k = counters(&r);
        assert_eq!(
            (forged(&k), k.seq_outside_window),
            ([0; 8], 1),
            "{timing:?}"
        );
    }
}

/// f13 (i): 8 commands far ahead (past the lead bound), then an honest command: the 8 are kind 1, nothing is
/// pre-loaded, and the honest one runs (NA-7).
#[test]
fn f13_far_ahead_commands_preload_nothing_and_the_honest_one_runs() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        let far: Vec<Command> = (1..=8u16).map(|s| cmd(s, j + 100 + u32::from(s))).collect();
        send(&mut r, &far);
        send(&mut r, &[cmd(10, j + 1)]);
        let mut ran = Vec::new();
        for _ in 0..3 {
            ran.extend(tick(&mut r));
        }
        assert!(ran.contains(&10), "{timing:?}: {ran:?}");
        assert!(ran.iter().all(|s| *s == 10), "{ran:?}");
        assert_eq!(counters(&r).lead_refused, 8);
    }
}

/// f13 (ii): under option B a command stamped equal to a buffered one is kind 2, never late (NA-13).
#[test]
fn f13_under_option_b_a_stamp_equal_to_a_buffered_one_is_kind_2_never_late() {
    let (mut r, j) = room(CmdTiming::B, 100);
    send(&mut r, &[cmd(1, j + 5), cmd(2, j + 5)]);
    tick(&mut r);
    let k = counters(&r);
    assert_eq!((k.not_newer, k.late), (1, 0), "{k:?}");
}

/// f13 (iii): the join anchor, under both options: a fresh slot whose first stamp is the join's tick - 1,000, and one
/// at the join's tick - 2^31 + 1, are each kind 2 and nothing runs; at room tick 1 too, with no underflow or panic.
#[test]
fn f13_the_join_anchor_refuses_a_first_stamp_before_the_join_as_kind_2() {
    for timing in BOTH {
        for first in [1u32, 5_000] {
            let (mut r, j) = room(timing, first);
            assert_eq!(j, first, "the room's first tick");
            send(&mut r, &[cmd(1, j.wrapping_sub(1_000))]);
            assert!(tick(&mut r).is_empty());
            send(&mut r, &[cmd(2, j.wrapping_sub(1 << 31).wrapping_add(1))]);
            assert!(tick(&mut r).is_empty());
            assert_eq!(counters(&r).not_newer, 2, "{timing:?} at {first}");
            assert_eq!(counters(&r).late, 0);
        }
    }
}

/// f13 (iv): a first stamp equal to the join's tick is kind 2; a first stamp of the join's tick + 1 is accepted and
/// runs, so an honest client stamped by 2.2's first-stamp rule counts 0 as kind 2.
#[test]
fn f13_a_first_stamp_at_the_join_tick_is_kind_2_and_one_after_it_runs() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 1);
        send(&mut r, &[cmd(1, j)]);
        send(&mut r, &[cmd(2, j + 1)]);
        let mut ran = tick(&mut r);
        ran.extend(tick(&mut r));
        assert_eq!(ran, [2], "{timing:?}");
        assert_eq!(counters(&r).not_newer, 1);
    }
}

/// f17 (carry 46, at the room's level): a forged press stream (a fire edge every tick, so the weapon's cooldown refuses
/// some) and its twin (the forged commands with only the refused presses removed: the fire counter one lower from each
/// refused command on, every other field equal) give equal state hashes on every tick.
#[test]
fn f17_a_forged_press_stream_and_its_twin_give_equal_state_hashes_on_every_tick() {
    let ticks = 90u32;
    let (mut forged_room, j) = room(CmdTiming::B, 100);
    let mut refused_at: Vec<u16> = Vec::new();
    let mut forged_hashes = Vec::new();
    let make = |s: u16, presses: u8| Command {
        buttons: 0,
        presses: [presses, 0, 0, 0],
        aim: [400, 0, 100],
        ..cmd(s, j + u32::from(s))
    };
    for s in 1..=ticks as u16 {
        send(&mut forged_room, &[make(s, s as u8)]);
        tick(&mut forged_room);
        refused_at.extend(forged_room.reports().refusals.iter().map(|x| x.seq));
        forged_hashes.push(forged_room.sim().state_hash());
    }
    assert!(!refused_at.is_empty(), "the cooldown refused some edges");
    let (mut twin, _) = room(CmdTiming::B, 100);
    let mut twin_hashes = Vec::new();
    for s in 1..=ticks as u16 {
        let removed = refused_at.iter().filter(|r| **r <= s).count() as u8;
        send(&mut twin, &[make(s, (s as u8).wrapping_sub(removed))]);
        tick(&mut twin);
        assert!(
            twin.reports().refusals.is_empty(),
            "the twin's presses are all applied"
        );
        twin_hashes.push(twin.sim().state_hash());
    }
    assert_eq!(forged_hashes, twin_hashes);
}

/// U4f-F1's T1 (ADV-U4F20-1; ADR 0001:226, :234-237): under option B two commands that arrive late (their ticks run, B
/// repeating the last) each carry a fire press, so the next on-time command's counter is 2 edges on from the last
/// command run (kind 5 accepts it: it covers 3 ticks from the newest accepted). Its two edges are seen, none lost and
/// none doubled, and the sim takes both: the first fires, the second (one shot per command) is refused as Cooldown
/// with that command's sequence.
#[test]
fn t1_under_option_b_two_late_drops_then_a_command_two_edges_on_lose_no_press() {
    let (mut r, j) = room(CmdTiming::B, 100);
    send(&mut r, &[cmd(1, j + 1)]);
    let mut ran = Vec::new();
    for _ in 0..4 {
        ran.extend(tick(&mut r));
    }
    assert_eq!(ran, [1, 1, 1], "j + 1 runs it, j + 2 and j + 3 repeat it");
    let pressed = |seq: u16, fire: u8| {
        let mut c = cmd(seq, j + u32::from(seq));
        c.presses[Command::PRESS_FIRE] = fire;
        c.aim = [400, 0, 100];
        c
    };
    send(&mut r, &[pressed(2, 1), pressed(3, 2), pressed(4, 2)]);
    assert_eq!(tick(&mut r), [4]);
    assert_eq!(counters(&r).late, 2, "the commands for j + 2 and j + 3");
    // The sim's side first (TE-F1-3): the room's count below uses the same rule, so only this shows the sim's take.
    let second = npt_sim::Refusal {
        slot: 0,
        seq: 4,
        kind: npt_sim::RefusalKind::Cooldown,
    };
    assert_eq!(r.reports().refusals, [second], "the second edge, refused");
    let presses = r.client(0).map(|c| c.presses());
    assert_eq!(presses, Some((2, 0, 0)), "seen, lost, doubled");
}

/// f34 (NA-14; the advisor's A3): under option A a trace whose commands arrive bunched (two in one tick, none the next)
/// gives `cmd_on_time` 1 for every tick whose stamped command arrived before it ran; a stamped command that never
/// arrived, and one that arrived after its tick, give 0; under both options.
#[test]
fn f34_cmd_on_time_and_input_missing_are_read_by_stamp() {
    for timing in BOTH {
        let (mut r, j) = room(timing, 100);
        let mut on = Vec::new();
        // Before the first stamp: no entry.
        tick(&mut r);
        on.push(r.client(0).and_then(|c| c.on_time()));
        // Ticks j+1 and j+2's commands arrive together before j+1; none before j+2 (bunched).
        send(&mut r, &[cmd(1, j + 1), cmd(2, j + 2)]);
        for _ in 0..2 {
            tick(&mut r);
            on.push(r.client(0).and_then(|c| c.on_time()));
        }
        // Tick j+3's command never arrives; tick j+4's arrives after j+4 ran (late).
        tick(&mut r);
        on.push(r.client(0).and_then(|c| c.on_time()));
        tick(&mut r);
        on.push(r.client(0).and_then(|c| c.on_time()));
        send(&mut r, &[cmd(4, j + 4), cmd(5, j + 6)]);
        tick(&mut r);
        on.push(r.client(0).and_then(|c| c.on_time()));
        tick(&mut r);
        on.push(r.client(0).and_then(|c| c.on_time()));
        assert_eq!(
            on,
            [
                None,
                Some(true),
                Some(true),
                Some(false),
                Some(false),
                Some(false),
                Some(true)
            ],
            "{timing:?}"
        );
    }
}

/// f39 (SA31-1, by U4F-Q18 (ii) (d)), under both options: a client stamps up to the tick run plus the lead bound, then
/// its slot rejoins (a `Leave` and a new join, then a replacing join on a live session): no command at or before the
/// slot's kept stamp runs, each is counted `rejoin_overlap`, and the honest rejoin's forged counts are 0.
#[test]
fn f39_a_rejoin_keeps_the_slots_newest_stamp_and_counts_the_overlap_apart() {
    for timing in BOTH {
        let (mut r, _) = room(timing, 100);
        for (round, how) in ["leave", "replace"].into_iter().enumerate() {
            // Stamp up to the next tick plus the lead bound (12), in the live session.
            let base = 100 * (round as u16 + 1);
            let next = r.last_tick().wrapping_add(1);
            let ahead: Vec<Command> = (0..8u16)
                .map(|i| cmd(base + i, next + 5 + u32::from(i)))
                .collect();
            send(&mut r, &ahead);
            tick(&mut r);
            let kept = next + 12;
            if how == "leave" {
                r.receive(0, &Message::Leave(npt_proto::Leave::Quit));
            }
            let at = r.last_tick().wrapping_add(1);
            assert_eq!(r.join(&join(0)).map(|x| x.0), Ok(0), "{how}");
            // From the new join's tick + 1 up to the kept stamp: the overlap; the one past it runs.
            let fresh: Vec<Command> = (at + 1..=kept + 1)
                .zip(base + 50..)
                .map(|(t, s)| cmd(s, t))
                .collect();
            let n = fresh.len() as u16;
            assert!(n > 1, "{how}: an overlap to test");
            send(&mut r, &fresh);
            let mut ran = Vec::new();
            for _ in 0..16 {
                ran.extend(tick(&mut r));
            }
            let k = counters(&r);
            assert_eq!(
                k.rejoin_overlap,
                u64::from(n - 1),
                "{timing:?} {how}: {k:?}"
            );
            assert_eq!(forged(&k), [0; 8], "{timing:?} {how}");
            let last = base + 50 + n - 1;
            assert!(
                !ran.is_empty() && ran.iter().all(|s| *s == last),
                "{timing:?} {how}: {ran:?}"
            );
        }
        kicked_rejoin(timing);
    }
}

/// f39's kicked-slot case (the advisor's F9, line 19: +0): in the stress scene slot 0, which never acknowledges (these
/// tests' `Input`s carry none: the never-acker's form), fires at an enemy under an event cap of 2, so the room forces a
/// resync and then kicks it (`Kick(overflow)`); the kicked slot rejoins, and its commands after the new join's tick and
/// at or before its kept stamp are counted `rejoin_overlap`, never forged, the one past it run.
fn kicked_rejoin(timing: CmdTiming) {
    let scene = Scene::stress();
    let world = World::new(&scene, 1, 100);
    let greybox = world.greybox().clone();
    let mut r = Room::new(world, &scene, &greybox, vec![token(0)], 1, 30)
        .with_cmd_timing(timing)
        .with_event_cap(2);
    assert_eq!(r.join(&join(0)).map(|j| j.0), Ok(0));
    let (mut seq, mut kept) = (0u16, 0u32);
    for _ in 0..600 {
        let enemy = r
            .sim()
            .entity_records()
            .iter()
            .find(|e| {
                e.kind == npt_sim::EntityKind::Soldier && scene.team(e.index as u8) != scene.team(0)
            })
            .map(|e| [e.grid[0], e.grid[1], e.grid[2] + 100])
            .expect("an enemy");
        seq += 1;
        kept = r.last_tick().wrapping_add(3);
        let fire = Command {
            buttons: Command::FIRE,
            aim: enemy,
            view_tick: r.last_tick(),
            ..cmd(seq, kept)
        };
        send(&mut r, &[fire]);
        tick(&mut r);
        if r.client(0).is_some_and(|c| c.kicked()) {
            break;
        }
    }
    let k = counters(&r);
    assert!(
        k.kicked == 1 && k.overflow_resyncs == 1,
        "{timing:?}: {k:?}"
    );
    let at = r.last_tick().wrapping_add(1);
    assert_eq!(
        r.join(&join(0)).map(|j| j.0),
        Ok(0),
        "the kicked slot rejoins"
    );
    let fresh: Vec<Command> = (at + 1..=kept + 1)
        .zip(seq + 50..)
        .map(|(t, s)| cmd(s, t))
        .collect();
    let n = fresh.len() as u16;
    send(&mut r, &fresh);
    let mut ran = Vec::new();
    for _ in 0..8 {
        ran.extend(tick(&mut r));
    }
    let k = counters(&r);
    assert_eq!(k.rejoin_overlap, u64::from(n - 1), "{timing:?} kick: {k:?}");
    assert_eq!(forged(&k), [0; 8], "{timing:?} kick");
    assert!(
        ran.contains(&(seq + 50 + n - 1)),
        "{timing:?} kick: {ran:?}"
    );
}

/// f18 (iii) (SA-10): under option B a clamp command (fire held, a point beyond reach inside the map, a move) run again
/// on the ticks with none stamped for them re-clamps from its moved fire origin: two or more clamp reports for its one
/// sequence, at different points (a lossy profile's clamp twin needs a per-tick rule first: 2.6).
#[test]
fn f18_under_option_b_a_rerun_clamp_command_gives_two_reports_for_one_sequence() {
    let scene = Scene::stress();
    let world = World::new(&scene, 1, 100);
    let greybox = world.greybox().clone();
    let mut r =
        Room::new(world, &scene, &greybox, vec![token(0)], 1, 30).with_cmd_timing(CmdTiming::B);
    let j = r.last_tick().wrapping_add(1);
    assert_eq!(r.join(&join(0)).map(|x| x.0), Ok(0));
    let far = Command {
        buttons: Command::FIRE,
        aim: [11_000, 11_000, 100],
        move_xy: [127, 0],
        view_tick: r.last_tick(),
        ..cmd(1, j + 1)
    };
    send(&mut r, &[far]);
    let mut points = Vec::new();
    for _ in 0..12 {
        tick(&mut r);
        points.extend(
            r.reports()
                .clamps
                .iter()
                .filter(|c| c.slot == 0 && c.seq == 1)
                .map(|c| c.point),
        );
    }
    points.dedup();
    assert!(points.len() >= 2, "{points:?}");
}

/// f31 (carry 39; U4F-Q8 (b)): the lead bound is a room setting in ms turned into ticks with the rate passed in,
/// pinned at 30 and 60 Hz: LEAD-BOUND's 400 ms is 12 ticks at 30 Hz and 24 at 60 Hz (600 ms: 18 and 36).
#[test]
fn f31_the_lead_bounds_ticks_are_pinned_at_30_and_60_hz() {
    let scene = Scene::course();
    for (hz, ticks) in [(30u32, 12u32), (60, 24)] {
        let world = World::new(&scene, 1, 100);
        let greybox = world.greybox().clone();
        let r = Room::new(world, &scene, &greybox, vec![token(0)], 1, hz);
        assert_eq!(r.lead_bound(), (400, ticks));
        assert_eq!(r.with_lead_bound_ms(600).lead_bound(), (600, ticks * 3 / 2));
    }
}

/// The room's -21 bites (bites builds only): each one's unit proof shows its forged kind gets through, beside the clean
/// room refusing it (u4f-brief.md 2.11).
#[cfg(feature = "bites")]
mod bites {
    use super::*;

    /// f30 (plan :399-401: bites per instance, never a static), the room half: a bitten and a clean room on parallel
    /// threads each keep their own behaviour.
    #[test]
    fn f30_a_bitten_and_a_clean_room_on_parallel_threads_each_keep_their_own() {
        let ran = |bite: Option<&'static str>| {
            move || {
                let (mut r, j) = match bite {
                    Some(b) => bitten(b, CmdTiming::A),
                    None => room(CmdTiming::A, 100),
                };
                send(&mut r, &[cmd(1, j + 13)]);
                tick(&mut r).contains(&1)
            }
        };
        let (clean, bit) = std::thread::scope(|sc| {
            let a = sc.spawn(ran(None));
            let b = sc.spawn(ran(Some("PM-0001-21/no-lead-bound")));
            (a.join().unwrap_or(true), b.join().unwrap_or(false))
        });
        assert_eq!((clean, bit), (false, true));
    }

    fn bitten(id: &str, timing: CmdTiming) -> (Room<World>, u32) {
        room_at(timing, 100, |r| r.with_bite(id).expect("a room bite"))
    }

    /// A course room over a `World` with npt-sim's bite `id` armed from its first tick (or none).
    fn sim_room(id: Option<&str>) -> (Room<World>, u32) {
        let scene = Scene::course();
        let world = match id {
            Some(b) => World::with_bite(&scene, 1, 100, b, 100).expect("a sim bite"),
            None => World::new(&scene, 1, 100),
        };
        let greybox = world.greybox().clone();
        let mut r = Room::new(world, &scene, &greybox, vec![token(0)], 1, 30);
        assert_eq!(r.join(&join(0)).map(|j| j.0), Ok(0));
        (r, 100)
    }

    /// PM-0001-21/no-cooldown (npt-sim's shot phase): a fire edge every tick meets the weapon's cooldown; clean, some
    /// are refused (Cooldown) and fire nothing; bitten, none is refused, so the edges in the cooldown fire and the
    /// world's state parts from the clean one's.
    #[test]
    fn pm_0001_21_no_cooldown_lets_a_cooldown_breaking_edge_fire() {
        let mut seen = Vec::new();
        for id in [None, Some("PM-0001-21/no-cooldown")] {
            let (mut r, j) = sim_room(id);
            let mut refused = 0;
            for s in 1..=60u16 {
                let mut c = cmd(s, j + u32::from(s));
                c.presses[Command::PRESS_FIRE] = s as u8;
                c.aim = [500, 0, 100];
                send(&mut r, &[c]);
                r.tick();
                // The cooldown's refusals (the bitten weapon fires faster, so its magazine empties: ammunition's
                // refusals are the check that stays).
                refused += r
                    .reports()
                    .refusals
                    .iter()
                    .filter(|x| x.kind == npt_sim::RefusalKind::Cooldown)
                    .count();
            }
            seen.push((refused, r.sim().state_hash()));
        }
        assert!(seen[0].0 > 0 && seen[1].0 == 0, "{seen:?}");
        assert_ne!(seen[0].1, seen[1].1, "the bitten edges fired");
    }

    /// PM-0001-21/refuse-not-clamp (npt-sim's clamp): a held trigger aimed beyond the 60 m reach (inside the map);
    /// clean, each shot is clamped and reported, never refused; bitten, refused and counted, no clamp report.
    #[test]
    fn pm_0001_21_refuse_not_clamp_refuses_and_counts_a_beyond_reach_point() {
        let mut seen = Vec::new();
        for id in [None, Some("PM-0001-21/refuse-not-clamp")] {
            let (mut r, j) = sim_room(id);
            let (mut refused, mut clamped) = (0, 0);
            for s in 1..=60u16 {
                let c = Command {
                    buttons: Command::FIRE,
                    aim: [12_000, 0, 100],
                    ..cmd(s, j + u32::from(s))
                };
                send(&mut r, &[c]);
                r.tick();
                refused += r.reports().refusals.len();
                clamped += r.reports().clamps.len();
            }
            seen.push((refused, clamped));
        }
        assert!(seen[0].0 == 0 && seen[0].1 > 0, "{seen:?}");
        assert!(seen[1].0 > 0 && seen[1].1 == 0, "{seen:?}");
    }

    /// Runs `forge` on a clean and a bitten room under `timing`; the forged sequence `seq` runs only in the bitten one.
    fn shows(id: &str, timing: CmdTiming, seq: u16, forge: impl Fn(&mut Room<World>, u32)) {
        let mut runs = Vec::new();
        for bite in [None, Some(id)] {
            let (mut r, j) = match bite {
                Some(b) => bitten(b, timing),
                None => room(timing, 100),
            };
            forge(&mut r, j);
            let mut ran = Vec::new();
            for _ in 0..16 {
                ran.extend(tick(&mut r));
            }
            runs.push(ran.contains(&seq));
        }
        assert_eq!(runs, [false, true], "{id} under {timing:?}");
    }

    #[test]
    fn pm_0001_21_no_lead_bound_lets_a_far_ahead_command_run() {
        shows("PM-0001-21/no-lead-bound", CmdTiming::A, 1, |r, j| {
            send(r, &[cmd(1, j + 13)])
        });
    }

    /// The equal-tick command comes a tick after the first ran, so the bitten tick's list stays one the sim's debug
    /// order assert takes (world.rs:2084; the judged bite runs are release builds, where two equal ticks of one slot in
    /// one tick are the bite's own effect).
    fn equal_tick(r: &mut Room<World>, j: u32) {
        send(r, &[cmd(1, j + 2)]);
        tick(r);
        send(r, &[cmd(2, j + 2)]);
    }

    #[test]
    fn pm_0001_21_no_newer_tick_lets_an_equal_tick_command_run() {
        shows("PM-0001-21/no-newer-tick", CmdTiming::A, 2, equal_tick);
    }

    /// TE-U4F-3: the bite is `>=` in place of `>`, not kind 2 removed: an older stamp (j + 1 after j + 2) is still kind 2.
    #[test]
    fn pm_0001_21_not_older_than_lets_an_equal_tick_command_run() {
        shows("PM-0001-21/not-older-than", CmdTiming::A, 2, equal_tick);
        let (mut r, j) = bitten("PM-0001-21/not-older-than", CmdTiming::A);
        send(&mut r, &[cmd(1, j + 2)]);
        tick(&mut r);
        send(&mut r, &[cmd(2, j + 1)]);
        tick(&mut r);
        assert_eq!(refused(&r), [(2, 2)], "{:?}", counters(&r));
    }

    #[test]
    fn pm_0001_21_no_seq_forward_lets_a_changed_backward_copy_run() {
        // Under option B the accepted command still waits in the buffer, so the changed copy takes its place.
        let id = "PM-0001-21/no-seq-forward";
        let mut moved = Vec::new();
        for bite in [None, Some(id)] {
            let (mut r, j) = match bite {
                Some(b) => bitten(b, CmdTiming::B),
                None => room(CmdTiming::B, 100),
            };
            send(&mut r, &[cmd(1, j + 3), cmd(2, j + 4)]);
            let copy = Command {
                move_xy: [60, 0],
                ..cmd(1, j + 3)
            };
            send(&mut r, &[copy, cmd(2, j + 4)]);
            let mut ran = false;
            for _ in 0..6 {
                tick(&mut r);
                ran |= r
                    .tick_input()
                    .commands
                    .iter()
                    .any(|c| c.0.move_xy == [60, 0]);
            }
            moved.push(ran);
        }
        assert_eq!(moved, [false, true]);
    }

    #[test]
    fn pm_0001_21_no_move_length_lets_a_long_move_run() {
        shows("PM-0001-21/no-move-length", CmdTiming::A, 1, |r, j| {
            send(
                r,
                &[Command {
                    move_xy: [127, 127],
                    ..cmd(1, j + 1)
                }],
            )
        });
    }

    #[test]
    fn pm_0001_21_no_press_edge_lets_two_edges_in_one_covered_tick_run() {
        shows("PM-0001-21/no-press-edge", CmdTiming::A, 2, |r, j| {
            let mut two = cmd(2, j + 2);
            two.presses[Command::PRESS_FIRE] = 2;
            send(r, &[cmd(1, j + 1), two])
        });
    }

    #[test]
    fn pm_0001_21_no_aim_bounds_lets_an_off_map_aim_run() {
        shows("PM-0001-21/no-aim-bounds", CmdTiming::A, 1, |r, j| {
            send(
                r,
                &[Command {
                    aim: [20_000, 0, 0],
                    ..cmd(1, j + 1)
                }],
            )
        });
    }

    #[test]
    fn pm_0001_21_no_ack_sent_lets_a_never_sent_acks_commands_run() {
        shows("PM-0001-21/no-ack-sent", CmdTiming::A, 1, |r, j| {
            r.receive(
                0,
                &Message::Input(Input {
                    ack_tick: 7,
                    ack_groups: 1,
                    commands: vec![cmd(1, j + 1)],
                }),
            )
        });
    }

    #[test]
    fn pm_0001_21_no_catchup_cap_lets_16_commands_run_in_one_tick() {
        let id = "PM-0001-21/no-catchup-cap";
        let mut most = Vec::new();
        for bite in [None, Some(id)] {
            let (mut r, j) = match bite {
                Some(b) => bitten(b, CmdTiming::A),
                None => room(CmdTiming::A, 100),
            };
            for _ in 0..10 {
                tick(&mut r);
            }
            let next = r.last_tick().wrapping_add(1);
            let first: Vec<Command> = (1..=8u16)
                .map(|s| cmd(s, next - 8 + u32::from(s)))
                .collect();
            let second: Vec<Command> = (9..=16u16)
                .map(|s| cmd(s, next - 8 + u32::from(s)))
                .collect();
            send(&mut r, &first);
            send(&mut r, &second);
            most.push(tick(&mut r).len());
            let _ = j;
        }
        assert_eq!(most, [8, 16]);
    }
}
