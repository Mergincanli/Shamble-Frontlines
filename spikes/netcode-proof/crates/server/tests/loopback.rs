//! U4e's Loopback tests (u4-build-plan.md §5 U4e): the room, the pump and the real client endpoint end to end on an
//! injected clock, in npt-server's own tests (the advisor's condition 10 (b)).

mod common;

use common::{Lab, endpoints, recorded, room};
use npt_room::ClientCounters;
use npt_sim::Scene;

/// Every client's counters, summed field by field.
fn totals(lab: &Lab) -> ClientCounters {
    let mut t = ClientCounters::default();
    for k in lab
        .conns
        .iter()
        .filter_map(|c| c.slot.and_then(|s| lab.room.client(s)))
        .map(|c| c.counters())
    {
        t.snapshots += k.snapshots;
        t.groups += k.groups;
        t.split_ticks += k.split_ticks;
        t.defaults_coded += k.defaults_coded;
        t.skipped += k.skipped;
        t.budget_short += k.budget_short;
        t.refused_out_of_range += k.refused_out_of_range;
        t.refused_too_many += k.refused_too_many;
        t.refused_too_large += k.refused_too_large;
        t.refused_not_canonical += k.refused_not_canonical;
        t.events_queued += k.events_queued;
        t.events_sent += k.events_sent;
        t.events_resent += k.events_resent;
        t.cosmetic_sent += k.cosmetic_sent;
        t.cosmetic_skipped += k.cosmetic_skipped;
        t.commands_buffered += k.commands_buffered;
        t.commands_run += k.commands_run;
        t.commands_rerun += k.commands_rerun;
        t.late += k.late;
        t.rewinds_refused += k.rewinds_refused;
    }
    t
}

/// e7 (u4-build-plan.md :719; §4.6): the mirror equals the client's hash over Loopback at P0 for 1,000 ticks, every
/// pair equal, none the sentinel, for all 48 clients of the stress scene (SS: the scene every judged -17 run carries,
/// its 384 effect rows and 300 trajectories included), the clients' commands npt-sim's synthetic recording's (seed 1).
/// The parts the mirror models, counted: entities the priority accumulator held at B's value, gameplay events carried,
/// snapshots coded against a baseline. On zero-delay Loopback a client's acknowledgement of tick t always reaches the
/// room before tick t + 1 is coded, so no event is resent and, at the window's budget, no tick splits (one group holds
/// up to 1,000 B of the about 1,067 B: the advisor's A2); both counts are printed, and the gap test (gap.rs) compares the
/// mirror over resent events, e2 over a forced split.
#[test]
fn e7_the_mirror_equals_the_clients_hash_over_loopback_at_p0_for_1_000_ticks() {
    let scene = Scene::stress();
    let (seed, first) = (1, 1_000);
    let ticks = 1_010;
    let script = recorded(&scene, seed, first, ticks + 10);
    let offsets: Vec<u64> = (0..48u64).map(|s| 1_000_000 + s * 500_000).collect();
    let mut lab = Lab::new(
        room(&scene, seed, first, 48),
        endpoints(48),
        &offsets,
        script,
    );
    let mut nonzero_age = 0u64;
    lab.run(u64::from(ticks), |lab| {
        for c in &lab.conns {
            if let Some(age) = c
                .slot
                .and_then(|s| lab.room.client(s))
                .and_then(|c| c.baseline_age())
                && age > 0
            {
                nonzero_age += 1;
            }
        }
    });
    let (compared, bad) = lab.compare();
    let per_client: Vec<usize> = (0..48u8)
        .map(|s| lab.client.keys().filter(|k| k.0 == s).count())
        .collect();
    let t = totals(&lab);
    println!(
        "e7: {compared} pairs compared, {bad} differ or are the sentinel; per client at least {}; split ticks {}, \
         skipped {}, events carried {}, resent {}, non-default baselines {nonzero_age}; {t:?}",
        per_client.iter().min().unwrap_or(&0),
        t.split_ticks,
        t.skipped,
        t.events_sent,
        t.events_resent,
    );
    assert_eq!(bad, 0);
    assert!(per_client.iter().all(|n| *n >= 1_000), "{per_client:?}");
    assert!(
        t.skipped >= 1,
        "an entity held at B's value by the priority accumulator"
    );
    assert!(t.events_sent >= 1, "a gameplay event carried");
    assert!(nonzero_age >= 1, "a snapshot coded against a baseline");
    assert!(t.commands_run >= 1, "the clients' commands ran");
}

/// e2 (u4-build-plan.md :716-717): a forced group split (each session's link takes datagrams of at most 300 B, so the
/// room's groups are bounded by it): ticks go out as several groups, each decoding alone against the one baseline, and
/// the mirror equals the client's hash on every split tick (the advisor's A2: e7's split part). One group of one split
/// tick dropped: the other groups still decode and are acknowledged, the tick is never filed and is resolved lost (e3).
#[test]
fn e2_a_forced_split_decodes_group_by_group_and_the_mirror_equals_the_clients_hash() {
    let scene = Scene::stress();
    let script = recorded(&scene, 2, 300, 80);
    let offsets: Vec<u64> = (0..48u64).map(|s| 1_000_000 + s * 500_000).collect();
    let mut lab = common::Lab::sized(
        room(&scene, 2, 300, 48),
        endpoints(48),
        &offsets,
        script,
        300,
    );
    let drop_at = 330u32;
    lab.tamper = Some(Box::new(move |slot, tick, group| {
        slot == 0 && tick == drop_at && group == 1
    }));
    let mut split: Vec<(u8, u32)> = Vec::new();
    lab.run(65, |lab| {
        for s in 0..48 {
            if lab.room.client(s).is_some_and(|c| c.groups() > 1) {
                split.push((s, lab.room.last_tick()));
            }
        }
    });
    let mut compared_splits = 0;
    for k in split.iter().filter(|k| !(k.0 == 0 && k.1 == drop_at)) {
        let c = lab.client.get(k);
        assert!(
            c.is_some_and(|h| *h != 0) && c == lab.server.get(k),
            "{k:?}"
        );
        compared_splits += 1;
    }
    println!(
        "e2: {} split ticks, {compared_splits} compared equal",
        split.len()
    );
    assert!(compared_splits >= 10, "{split:?}");
    assert!(split.contains(&(0, drop_at)), "the dropped tick was split");
    assert!(
        !lab.client.contains_key(&(0, drop_at)),
        "a tick with a lost group is never filed"
    );
    let lost = &lab.conns[0].lost;
    assert!(
        lost.iter().any(|&(a, n)| a <= drop_at && drop_at < a + n),
        "{lost:?}"
    );
    let (compared, bad) = lab.compare();
    assert_eq!(bad, 0, "{compared} compared");
}

/// e6 (u4-build-plan.md :718-719; §4.5 :300-302): the window counts the pump's bytes: on a tick the pump reported k
/// pings (payload plus 28 B each), the encoder's budget is the per-tick budget less them, and the snapshot sent that
/// tick stays within that remainder.
#[test]
fn e6_the_window_counts_the_pumps_bytes() {
    let scene = Scene::stress();
    let script = recorded(&scene, 3, 300, 120);
    let mut lab = Lab::new(
        room(&scene, 3, 300, 4),
        endpoints(4),
        &[2_000_000; 4],
        script,
    );
    let mut seen: Vec<(u32, u32, u32, u32)> = Vec::new();
    lab.run(100, |lab| {
        let t = lab.room.last_tick();
        for s in 0..4u8 {
            if let (Some(c), Some(&p)) = (lab.room.client(s), lab.pumped.get(&(s, t)))
                && c.groups() > 0
            {
                let (budget, left) = c.budget();
                seen.push((p, budget, left, c.bytes()));
            }
        }
    });
    let pinged: Vec<_> = seen.iter().filter(|s| s.0 > 0).collect();
    println!(
        "e6: {} snapshots, {} on ticks with pump bytes, e.g. {:?}",
        seen.len(),
        pinged.len(),
        pinged.first()
    );
    assert!(pinged.len() >= 4, "the pump pings a few times a second");
    for &(p, budget, left, bytes) in &seen {
        assert_eq!(left, budget - p);
        assert!(bytes <= left, "{bytes} B sent of {left}");
    }
    assert!(
        pinged.iter().all(|s| s.0 == 10 + 28),
        "one 10 B ping plus 28 B"
    );
}

/// TE-2 (advisor-u4e-r1 R6; e6's binding case, §4.5 :300-302): on one tick, the same room state with the pump's report
/// raised by `P` bytes sends at most the remainder (the per-tick budget less every pump byte), while its twin with
/// nothing added sends more than that remainder: the remainder, not the whole budget, bounds what the encoder packs.
#[test]
fn e6_the_pumps_bytes_shrink_the_snapshot_to_the_remainder() {
    const P: u32 = 600;
    let scene = Scene::stress();
    let run = |p: u32| {
        let script = recorded(&scene, 3, 300, 40);
        let offsets: Vec<u64> = (0..48u64).map(|s| 1_000_000 + s * 500_000).collect();
        let mut lab = Lab::new(room(&scene, 3, 300, 48), endpoints(48), &offsets, script);
        lab.run(20, |_| {});
        lab.room.pump_report(0, p, common::MAX_DATAGRAM);
        let mut at = (0, 0, 0, 0);
        lab.run(1, |lab| {
            if let Some(c) = lab.room.client(0) {
                let (budget, left) = c.budget();
                at = (budget, left, c.bytes(), c.groups());
            }
        });
        (at, lab.compare())
    };
    let ((budget, left, bytes, groups), (compared, bad)) = run(P);
    let ((twin_budget, _, twin_bytes, _), _) = run(0);
    println!(
        "e6 (binding): budget {budget} B, remainder {left} B after {P} B more of the pump's; sent {bytes} B in \
         {groups} groups; the twin with none added sent {twin_bytes} B; {compared} pairs compared, {bad} bad"
    );
    assert_eq!(budget, twin_budget, "the same tick of the window");
    assert!(left + P <= budget, "the remainder counts the added bytes");
    assert!(groups >= 1 && bytes <= left, "{bytes} B sent of {left}");
    assert!(twin_bytes > left, "the twin's snapshot would not fit it");
    assert_eq!(bad, 0, "the shrunk snapshot decodes to the mirror's hash");
}

/// The first tick `tick16_lab`'s room runs.
const TICK16_FIRST: u32 = 65_520;

/// -17/tick16 and -58/tick16's unit proof through the room's encoder and the endpoint (U4E-Q4 (a); §4.14 :459): over
/// Loopback with the room's tick past 65,535, the client files tick 65,537 under itself with the mirror's hash.
fn tick16_case(lab: &mut Lab) -> (Option<u64>, Option<u64>) {
    lab.run(30, |_| {});
    let k = (0u8, 65_537u32);
    (lab.server.get(&k).copied(), lab.client.get(&k).copied())
}

fn tick16_lab(bite: Option<&str>) -> Lab {
    let scene = Scene::stress();
    let script = recorded(&scene, 1, TICK16_FIRST, 40);
    let mut r = room(&scene, 1, TICK16_FIRST, 2);
    let mut e = endpoints(2);
    #[cfg(feature = "bites")]
    if let Some(id) = bite {
        r = r.with_bite(id).expect("npt-proto plants it");
        e = e
            .into_iter()
            .map(|c| c.with_bite(id).expect("npt-proto plants it"))
            .collect();
    }
    let _ = (&mut r, &mut e, bite);
    Lab::new(r, e, &[3_000_000; 2], script)
}

/// The clean half (advisor-u4e-r1 R1, what the judge counts): every pair compared equal (at least 30), tick 65,537's
/// pair among them, and no client hash outside the ticks the room ran (65,520 on).
#[test]
fn pm_0001_17_and_58_tick16_the_client_files_tick_65_537_under_itself() {
    let mut lab = tick16_lab(None);
    let (server, client) = tick16_case(&mut lab);
    let (compared, bad) = lab.compare();
    let below: Vec<(u8, u32)> = lab
        .client
        .keys()
        .filter(|k| k.1 < TICK16_FIRST)
        .copied()
        .collect();
    println!(
        "tick16 clean: the server's hash at 65,537 {server:?}, the client's {client:?}; {compared} pairs compared, \
         {bad} bad; client keys below 65,520 {below:?}; client 0 stale {}",
        lab.conns[0].endpoint.stale()
    );
    assert!(
        server.is_some() && server == client,
        "{server:?} {client:?}"
    );
    assert!(compared >= 30 && bad == 0, "{compared} compared, {bad} bad");
    assert!(below.is_empty(), "{below:?}");
}

/// The bites that need every crate's list (bites builds only, under `--workspace --all-features`: npt-server's
/// `bites` feature does not turn on npt-client's).
#[cfg(feature = "bites")]
mod bites {
    use super::*;

    /// Part (i) (advisor-u4e-r1 R1: what the judge counts): under either bite the header names the tick's low 16 bits,
    /// so after 65,535 the client reads 0, 1, 2 ..., ticks before its first header and at least the ring behind its
    /// newest: bad headers, each filed as the sentinel under the tick it names, a client hash the server never logged
    /// (an orphan). The figure fails (a bad pair), slot 0 holds an orphan below 65,520, and nothing is filed at or
    /// past 65,536.
    #[test]
    fn pm_0001_17_and_58_tick16_post_wrap_frames_are_filed_as_orphan_sentinels() {
        for id in ["PM-0001-17/tick16", "PM-0001-58/tick16"] {
            let mut lab = tick16_lab(Some(id));
            let (server, client) = tick16_case(&mut lab);
            let (compared, bad) = lab.compare();
            let slot0: Vec<(u32, u64)> = lab
                .client
                .iter()
                .filter(|(k, _)| k.0 == 0)
                .map(|(k, h)| (k.1, *h))
                .collect();
            let orphans: Vec<u32> = slot0
                .iter()
                .filter(|(t, _)| *t < TICK16_FIRST && !lab.server.contains_key(&(0, *t)))
                .map(|(t, _)| *t)
                .collect();
            let post: Vec<u32> = slot0
                .iter()
                .filter(|(t, _)| *t >= 65_536)
                .map(|(t, _)| *t)
                .collect();
            println!(
                "{id}: the server's hash at 65,537 {server:?}, the client's {client:?}; {compared} pairs compared, \
                 {bad} bad; client 0's orphans {orphans:?}; at or past 65,536 {post:?}; client 0 stale {}",
                lab.conns[0].endpoint.stale()
            );
            assert!(bad >= 1, "{id}: the figure fails");
            assert!(!orphans.is_empty() && post.is_empty(), "{id}");
            assert!(server.is_some() && server != client, "{id}");
        }
    }

    /// U4E-Q13 (a): `catalogue-5/owner-to-all` is in npt-room's list and no other crate's (check.mjs's bite-home grep
    /// reads only the `PM-<ADR>-<row>/<name>` form, so this test holds its one home); no id is in two lists.
    #[test]
    fn catalogue_5_owner_to_all_lives_in_npt_room_alone_and_no_id_is_in_two_lists() {
        let id = "catalogue-5/owner-to-all";
        let lists: [(&str, Vec<&str>); 6] = [
            ("npt_room::BITES", npt_room::BITES.to_vec()),
            ("npt_proto::BITES", npt_proto::BITES.to_vec()),
            ("npt_sim::World::BITES", npt_sim::World::BITES.to_vec()),
            (
                "npt_sim::OWN_COPY_SNAP_BITES",
                npt_sim::OWN_COPY_SNAP_BITES.to_vec(),
            ),
            ("npt_server::BITES", npt_server::BITES.to_vec()),
            ("npt_client::BITES", npt_client::BITES.to_vec()),
        ];
        let homes: Vec<&str> = lists
            .iter()
            .filter(|l| l.1.contains(&id))
            .map(|l| l.0)
            .collect();
        assert_eq!(homes, ["npt_room::BITES"]);
        let mut all: Vec<&str> = lists
            .iter()
            .filter(|l| l.0 != "npt_sim::World::BITES")
            .flat_map(|l| l.1.clone())
            .collect();
        // World::BITES lists the own-copy snap's id beside OWN_COPY_SNAP_BITES (world.rs:605): one site, two lists.
        all.extend(
            npt_sim::World::BITES
                .iter()
                .filter(|b| !npt_sim::OWN_COPY_SNAP_BITES.contains(b)),
        );
        let n = all.len();
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), n, "an id in two lists");
    }
}
