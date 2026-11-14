//! U4e's endpoint tests (u4-build-plan.md §5 U4e; u4e-brief.md 2.3, 2.4): crafted room messages fed to a real
//! `ClientEndpoint`, each encoded with npt-proto as the room sends it, and what the endpoint files read back.

use npt_client::{ClientEndpoint, StepOut};
use npt_link::Channel;
use npt_proto::{
    Direction, EntityDelta, FullState, Join, JoinAccepted, Limits, Message, Snapshot, client_hash,
    decode, encode,
};
use npt_sim::{Command, EntityKind, EntityRecord, SimEvent};

fn bytes(m: &Message) -> Vec<u8> {
    let mut out = Vec::new();
    encode(m, &mut out).unwrap_or_else(|e| panic!("{}: {e}", m.name()));
    out
}

fn soldier(index: u16, x: i32) -> EntityRecord {
    let mut grid = [0; EntityRecord::FIELDS];
    grid[0] = x;
    grid[1] = -x;
    grid[2] = 150;
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

/// A joined endpoint (slot 0) holding the full state of tick `t`: soldiers 1 and 2 at x = 100 and 200, its own soldier.
fn joined(e: ClientEndpoint, t: u32) -> ClientEndpoint {
    let mut e = e;
    let accepted = bytes(&Message::JoinAccepted(JoinAccepted {
        slot: 0,
        tick: t,
        ..JoinAccepted::default()
    }));
    let full = bytes(&Message::FullState(FullState {
        tick: t,
        last_run: None,
        own: vec![soldier(0, 0)],
        entities: vec![soldier(1, 100), soldier(2, 200)],
    }));
    e.input(
        None,
        &[
            (Channel::Control, accepted, 0),
            (Channel::FullState, full, 0),
        ],
        0,
    );
    let out = e.step();
    assert_eq!(out.hashes.len(), 1, "the full state's tick is filed");
    e
}

fn endpoint() -> ClientEndpoint {
    ClientEndpoint::new(Join::default(), 30)
}

/// A snapshot group of tick `t` against the baseline `age` ticks back: soldier 1 moved to `x`.
fn moved(t: u32, age: u16, group: u8, groups: u8, x: i32) -> Snapshot {
    let base = soldier(1, 100);
    Snapshot {
        tick: t,
        baseline_age: age,
        group,
        groups,
        entities: vec![EntityDelta::between(&soldier(1, x), Some(&base)).expect("a change")],
        ..Snapshot::default()
    }
}

fn datagram(s: Snapshot) -> (Channel, Vec<u8>, u64) {
    (Channel::Datagram, bytes(&Message::Snapshot(s)), 0)
}

fn feed(e: &mut ClientEndpoint, msgs: Vec<(Channel, Vec<u8>, u64)>) -> StepOut {
    e.input(None, &msgs, 0);
    e.step()
}

/// What a client holding `entities` and its own soldier hashes at `t` with no events.
fn hash_of(t: u32, entities: &[EntityRecord]) -> u64 {
    client_hash(t, entities, &[soldier(0, 0)], &[])
}

/// e8 (§4.11 :401-402): `ClientEndpoint` is `Send`.
#[test]
fn e8_the_client_endpoint_is_send() {
    fn send<T: Send>() {}
    send::<ClientEndpoint>();
}

/// e9 (catalogue item 2's "on both sides"; nit d): a message on a channel its type does not travel on is dropped and
/// counted, never filed; a datagram above 1,150 B is refused by its size (the sentinel under its header's tick).
#[test]
fn e9_the_endpoint_drops_a_wrong_channel_message_and_refuses_a_datagram_above_1_150_b() {
    let mut e = joined(endpoint(), 100);
    let s = bytes(&Message::Snapshot(moved(101, 1, 0, 1, 150)));
    // The snapshot framed as a control-stream message: its 2 B length, then its type and body.
    let mut framed = (s.len() as u16).to_le_bytes().to_vec();
    framed.extend_from_slice(&s);
    let out = feed(&mut e, vec![(Channel::Control, framed, 0)]);
    assert_eq!((e.wrong_channel(), out.hashes.len()), (1, 0));
    let mut big = s;
    big.resize(Limits::SNAPSHOT_RECEIVE + 1, 0);
    big[1..5].copy_from_slice(&102u32.to_le_bytes());
    let out = feed(&mut e, vec![(Channel::Datagram, big, 0)]);
    assert_eq!(out.hashes, [(102, 0)], "refused by its size: the sentinel");
    assert_eq!(e.refused(), 1);
}

/// e3 (u4-build-plan.md :717-718): a lost group leaves its tick unlogged (and resolved lost, counted, once it has left
/// the frame ring); a refused group logs the sentinel.
#[test]
fn e3_a_lost_group_leaves_its_tick_unlogged_and_a_refused_one_logs_the_sentinel() {
    let mut e = joined(endpoint(), 100);
    let out = feed(&mut e, vec![datagram(moved(101, 1, 0, 2, 150))]);
    assert!(
        out.hashes.is_empty(),
        "one group of two: not complete, nothing filed"
    );
    // A group naming a baseline the client does not hold (tick 90): refused, the sentinel.
    let out = feed(&mut e, vec![datagram(moved(102, 12, 0, 1, 150))]);
    assert_eq!(out.hashes, [(102, 0)]);
    // Corrupt bytes after the last field: refused by the decoder, the sentinel under the header's tick.
    let mut b = bytes(&Message::Snapshot(moved(103, 3, 0, 1, 150)));
    b.push(0);
    let out = feed(&mut e, vec![(Channel::Datagram, b, 0)]);
    assert_eq!(out.hashes, [(103, 0)]);
    // Tick 101 is resolved lost once it has left the ring, never filed.
    let len = npt_sim::ACK_RING_TICKS;
    let last = 101 + len;
    let fresh = Snapshot {
        tick: last,
        entities: vec![EntityDelta::Add(soldier(1, 100))],
        ..Snapshot::default()
    };
    let out = feed(&mut e, vec![datagram(fresh)]);
    assert!(
        out.lost.iter().any(|&(a, n)| a <= 101 && 101 < a + n),
        "{:?}",
        out.lost
    );
    assert!(!out.hashes.iter().any(|h| h.0 == 101));
}

/// The acknowledgements (tick, group mask) in the datagrams one step sent.
fn acks(out: &StepOut) -> Vec<(u16, u8)> {
    out.datagrams
        .iter()
        .filter_map(
            |d| match decode(Direction::ClientToRoom, Channel::Datagram, d) {
                Ok(Message::Input(i)) => Some((i.ack_tick, i.ack_groups)),
                _ => None,
            },
        )
        .collect()
}

/// TE-4 (advisor-u4e-r1 R6; plan :716-717's "each group decoding alone"): group 1 of a split tick arrives and group 0
/// never does. Group 1 decodes alone against the baseline and is acknowledged (the mask without bit 0), nothing is
/// filed for the tick, and it is resolved lost once it has left the frame ring, never the sentinel.
#[test]
fn e3_a_split_tick_without_group_0_acknowledges_its_other_group_and_is_resolved_lost() {
    let mut e = joined(endpoint(), 100);
    let out = feed(&mut e, vec![datagram(moved(101, 1, 1, 2, 150))]);
    assert!(out.hashes.is_empty(), "one group of two: nothing filed");
    assert_eq!(acks(&out), [(101, 0b10)], "group 1 acknowledged alone");
    assert_eq!(e.refused(), 0);
    let fresh = Snapshot {
        tick: 101 + npt_sim::ACK_RING_TICKS,
        entities: vec![EntityDelta::Add(soldier(1, 100))],
        ..Snapshot::default()
    };
    let out = feed(&mut e, vec![datagram(fresh)]);
    assert!(
        out.lost.iter().any(|&(a, n)| a <= 101 && 101 < a + n),
        "{:?}",
        out.lost
    );
    assert!(
        !out.hashes.iter().any(|h| h.0 == 101),
        "lost, never the sentinel"
    );
}

/// TE-7 (carry 61 (b): "two different hashes for one tick give the sentinel"): one group received again with other
/// bytes, and a tick completed by its full state and then sent again as a snapshot of other content: each files the
/// sentinel after the tick's first hash.
#[test]
fn carry_61_b_a_second_different_hash_for_one_tick_gives_the_sentinel() {
    let mut e = joined(endpoint(), 100);
    let out = feed(&mut e, vec![datagram(moved(101, 1, 0, 1, 150))]);
    assert_eq!(
        out.hashes,
        [(101, hash_of(101, &[soldier(1, 150), soldier(2, 200)]))]
    );
    let out = feed(&mut e, vec![datagram(moved(101, 1, 0, 1, 151))]);
    assert_eq!(out.hashes, [(101, 0)], "the same group with other bytes");
    let full = bytes(&Message::FullState(FullState {
        tick: 102,
        last_run: None,
        own: vec![soldier(0, 0)],
        entities: vec![soldier(1, 100)],
    }));
    let out = feed(&mut e, vec![(Channel::FullState, full, 0)]);
    assert_eq!(out.hashes, [(102, hash_of(102, &[soldier(1, 100)]))]);
    let other = Snapshot {
        tick: 102,
        entities: vec![EntityDelta::Add(soldier(1, 300))],
        ..Snapshot::default()
    };
    let out = feed(&mut e, vec![datagram(other)]);
    assert_eq!(out.hashes, [(102, 0)], "tick 102 again, with another hash");
}

/// SEC-U4E-7 (advisor-u4e-r1 R5): four incomplete groups 2^31 - 1 ticks apart in one input (a hostile server's) never
/// overflow a lost run's count: a sum past `u32::MAX` starts a new run, and the runs, each continuing the one before,
/// cover every tick resolved (from 101, the tick after the full state, to the last header's ring cutoff).
#[test]
fn sec_u4e_7_lost_runs_2_pow_31_apart_never_overflow_and_cover_the_ticks() {
    let mut e = joined(endpoint(), 100);
    let apart = (1u32 << 31) - 1;
    let groups = (0..4u32)
        .map(|k| {
            datagram(Snapshot {
                tick: 101u32.wrapping_add(k.wrapping_mul(apart)),
                groups: 2,
                ..Snapshot::default()
            })
        })
        .collect();
    let out = feed(&mut e, groups);
    let runs = &out.lost;
    println!("SEC-U4E-7: lost runs {runs:?}");
    assert!(runs.len() >= 2, "{runs:?}");
    assert_eq!(runs.first().map(|r| r.0), Some(101));
    for w in runs.windows(2) {
        assert_eq!(w[0].0.wrapping_add(w[0].1), w[1].0, "{runs:?}");
    }
    let covered: u64 = runs.iter().map(|r| u64::from(r.1)).sum();
    let want = 3 * u64::from(apart) - u64::from(npt_sim::ACK_RING_TICKS - 1);
    assert_eq!(covered, want);
}

/// Carry 61: tick t + 1's snapshot fed before tick t's (as L1's reorder step delivers them) files t under t with its
/// hash (the client decodes a stale group: the advisor's lean), never the sentinel; one datagram received twice files
/// one hash; a tick with a group missing is not resolved lost while younger than `npt_sim::ACK_RING_TICKS` ticks (by
/// the newest tick held), and is once it is that old.
#[test]
fn carry_61_reordered_and_duplicated_datagrams_file_one_hash_under_each_tick() {
    let mut e = joined(endpoint(), 100);
    let later = moved(102, 2, 0, 1, 170);
    let earlier = moved(101, 1, 0, 1, 160);
    let out = feed(
        &mut e,
        vec![datagram(later.clone()), datagram(earlier.clone())],
    );
    let want = [
        (102, hash_of(102, &[soldier(1, 170), soldier(2, 200)])),
        (101, hash_of(101, &[soldier(1, 160), soldier(2, 200)])),
    ];
    assert_eq!(
        out.hashes, want,
        "each under its own tick, neither the sentinel"
    );
    let out = feed(&mut e, vec![datagram(earlier)]);
    assert!(out.hashes.is_empty(), "a duplicate is filed once");
    assert_eq!(e.duplicates(), 1);
    // Tick 103 of two groups, one lost: not resolved while younger than the ring, resolved once that old.
    feed(&mut e, vec![datagram(moved(103, 3, 0, 2, 180))]);
    let len = npt_sim::ACK_RING_TICKS;
    let at = |t: u32| Snapshot {
        tick: t,
        entities: vec![EntityDelta::Add(soldier(1, 100))],
        ..Snapshot::default()
    };
    let mut lost = Vec::new();
    for t in 104..103 + len {
        lost.extend(feed(&mut e, vec![datagram(at(t))]).lost);
    }
    assert!(
        !lost.iter().any(|&(a, n)| a <= 103 && 103 < a + n),
        "103 is {} old: {lost:?}",
        len - 1
    );
    let out = feed(&mut e, vec![datagram(at(103 + len))]);
    assert!(
        out.lost.iter().any(|&(a, n)| a <= 103 && 103 < a + n),
        "{:?}",
        out.lost
    );
    assert!(
        !out.hashes.iter().any(|h| h.0 == 103 && h.1 == 0),
        "lost, never the sentinel"
    );
}

/// The endpoint after stamping `n` commands from a counter started at `start`, at newest tick 100.
fn commanding(e: ClientEndpoint, start: u32, n: u32) -> ClientEndpoint {
    let mut e = joined(e.with_starts(start, 0), 100);
    for _ in 0..n {
        e.input(Some(Command::default()), &[], 0);
    }
    e.step();
    e
}

/// The last-run rebuild the endpoint makes of a header naming command `seq` (16 bits), after 8 commands from 65,530.
fn last_run_case(e: ClientEndpoint, seq: u16) -> (Option<u32>, Vec<&'static str>) {
    let mut e = commanding(e, 65_530, 8);
    let s = Snapshot {
        last_run: Some(seq),
        ..moved(101, 1, 0, 1, 150)
    };
    let out = feed(&mut e, vec![datagram(s)]);
    let s = Snapshot {
        last_run: Some(seq.wrapping_add(2)),
        ..moved(102, 2, 0, 1, 150)
    };
    let mut wraps = out.wraps;
    wraps.extend(feed(&mut e, vec![datagram(s)]).wraps);
    (e.last_run_command(), wraps)
}

/// -58's last-run rebuild in the clean build (U4E-Q2 (c)): the header's 16-bit last-run sequence names the client's own
/// command across the wrap, and that counter's wrap is exposed once by name (W1-N2).
#[test]
fn pm_0001_58_the_last_run_rebuild_names_the_right_command_across_the_wrap() {
    let (last, wraps) = last_run_case(endpoint(), 65_535);
    assert_eq!(last, Some(65_537), "65,535 then 65,537 (low bits 1)");
    assert_eq!(wraps, ["tick_offset"]);
}

/// The hash the endpoint files at tick 102 for events numbered 65,533 to 65,537 (two snapshots), with its event
/// counter's start at `event_start`, and the hash a client that numbered them right would file.
fn event_case(e: ClientEndpoint) -> (u64, u64, Vec<&'static str>) {
    let mut e = joined(e.with_starts(0, 65_533), 100);
    let ev = SimEvent::Kill {
        killer: 1,
        victim: 2,
    };
    let s1 = Snapshot {
        events: [65_533u32, 65_534, 65_535]
            .iter()
            .map(|n| (*n as u16, 0, ev))
            .collect(),
        ..moved(101, 1, 0, 1, 150)
    };
    let s2 = Snapshot {
        events: [65_536u32, 65_537]
            .iter()
            .map(|n| (*n as u16, 0, ev))
            .collect(),
        ..moved(102, 2, 0, 1, 150)
    };
    let out = feed(&mut e, vec![datagram(s1), datagram(s2)]);
    let got = out.hashes.iter().find(|h| h.0 == 102).map_or(0, |h| h.1);
    let want = client_hash(
        102,
        &[soldier(1, 150), soldier(2, 200)],
        &[soldier(0, 0)],
        &[(65_536, ev), (65_537, ev)],
    );
    (got, want, out.wraps)
}

/// -58's event-number rebuild in the clean build (§4.14 :470): post-wrap event numbers are rebuilt to their 32 bits
/// (the hash reads them), and the counter's wrap is exposed once by name.
#[test]
fn pm_0001_58_event_numbers_are_rebuilt_across_the_16_bit_wrap() {
    let (got, want, wraps) = event_case(endpoint());
    assert_eq!(got, want);
    assert_eq!(wraps, ["event_seq"]);
}

/// npt-client's planted defects (bites builds only), each in wave 1's two parts: a bites-only twin asserting the clean
/// test's check fails on the bitten endpoint.
#[cfg(feature = "bites")]
mod bites {
    use super::*;

    fn bitten(id: &str) -> ClientEndpoint {
        endpoint().with_bite(id).expect("npt-client plants it")
    }

    #[test]
    fn pm_0001_58_lt_tick_offset_names_the_wrong_command() {
        let (last, _) = last_run_case(bitten("PM-0001-58/lt-tick-offset"), 65_535);
        assert_ne!(
            last,
            Some(65_537),
            "the rebuilt sequence names the wrong command: {last:?}"
        );
    }

    #[test]
    fn pm_0001_58_lt_event_seq_rebuilds_a_post_wrap_event_wrong() {
        let (got, want, _) = event_case(bitten("PM-0001-58/lt-event-seq"));
        assert_ne!(got, want);
    }

    /// TE-3 (advisor-u4e-r1 R6; §5 U4e's "client-rounding (the client's call site)", carry 24's first clause): one
    /// decoded snapshot fed to a clean endpoint and to one built with PM-0001-17/client-rounding: the clean one files
    /// the mirror's hash, the bitten one another hash.
    #[test]
    fn pm_0001_17_client_rounding_at_the_endpoints_call_site_files_another_hash() {
        let entities: Vec<EntityRecord> = (1..=16u16)
            .map(|i| soldier(i, i32::from(i) * 37 + 11))
            .collect();
        let snap = Snapshot {
            tick: 101,
            entities: entities.iter().map(|r| EntityDelta::Add(*r)).collect(),
            ..Snapshot::default()
        };
        let mirror = client_hash(101, &entities, &[], &[]);
        let filed = |e: ClientEndpoint| {
            let mut e = joined(e, 100);
            feed(&mut e, vec![datagram(snap.clone())]).hashes
        };
        assert_eq!(filed(endpoint()), [(101, mirror)]);
        let got = filed(bitten("PM-0001-17/client-rounding"));
        println!(
            "client-rounding at the endpoint: the mirror's {mirror}, the bitten endpoint's {got:?}"
        );
        assert!(
            got.len() == 1 && got[0].0 == 101 && got[0].1 != mirror && got[0].1 != 0,
            "{got:?}"
        );
    }

    #[test]
    fn the_endpoint_takes_npt_protos_ids_through_quant_and_no_other() {
        assert!(endpoint().with_bite("PM-0001-17/tick16").is_some());
        assert!(endpoint().with_bite("PM-0001-18/no-fallback").is_none());
    }
}
