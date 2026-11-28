//! e5 and -18's bite end to end (u4e-brief.md 2.4 e5, 2.5; u4-build-plan.md §4.14 :461, §5 U4e): a gap in delivery,
//! every datagram dropped both ways by the lab's `Gate` on the injected clock, of `npt_sim::ACK_RING_TICKS` tick
//! periods (U4E-Q14 (b)), at two alignments against the server's send of tick T; the clients step 5 ms after each
//! server tick. Its length in ns is computed here from that constant by path and the rate the test passes in.

mod common;

use std::collections::BTreeMap;

use common::{Lab, endpoints, recorded, room};
use npt_room::Room;
use npt_sim::{Scene, World};

const CLIENTS: u8 = 4;
const OFFSET_NS: u64 = 5_000_000;

/// Where the gap starts against the server's send of tick T.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Align {
    /// 1 ns after it: T is delivered, the client's acknowledgement of T is the first datagram lost.
    AfterSend,
    /// 1 ns before it: T's snapshot is the first datagram lost.
    BeforeSend,
}

/// What one gap run showed for client 0.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Gap {
    align: Align,
    /// Tick T and the gap's start and end, ns.
    t: u32,
    from_ns: u64,
    to_ns: u64,
    /// The first tick the server runs at or after the gap's end, F.
    f: u32,
    /// B's tick (the last tick fully acknowledged) and B's age at F, and the room's per-tick eviction flag at F.
    b_tick: Option<u32>,
    b_age: Option<u32>,
    evicted: bool,
    /// The baseline age F's snapshot was coded at (0: the per-type defaults).
    f_age: Option<u16>,
    /// The first tick at or after F the client decoded with the server's hash, and when the server sent it.
    fresh: Option<(u32, u64)>,
    /// The sentinels the client filed from F on.
    sentinels: usize,
    /// Every pair compared over the run, and how many differ or are the sentinel; the events resent.
    compared: usize,
    bad: usize,
    resent: u64,
    /// The room's hashes, for e11.
    server: BTreeMap<(u8, u32), u64>,
}

/// The gap's length: `npt_sim::ACK_RING_TICKS` tick periods at `tick_hz` (1.067 s at 30 Hz), in ns.
fn ring_gap_ns(tick_hz: u32) -> u64 {
    u64::from(npt_sim::ACK_RING_TICKS) * 1_000_000_000 / u64::from(tick_hz)
}

fn gap_case(room: Room<World>, align: Align, gap_ns: u64) -> Gap {
    let scene = Scene::stress();
    let script = recorded(&scene, 1, 300, 200);
    let offsets = vec![OFFSET_NS; usize::from(CLIENTS)];
    let mut lab = Lab::new(room, endpoints(CLIENTS), &offsets, script);
    let mut per_tick: BTreeMap<u32, (Option<u32>, bool, Option<u16>)> = BTreeMap::new();
    let mut record = |lab: &Lab| {
        if let Some(c) = lab.room.client(0) {
            per_tick.insert(
                lab.room.last_tick(),
                (c.last_acked(), c.burst_eviction(), c.baseline_age()),
            );
        }
    };
    lab.run(30, &mut record);
    let t = lab.room.last_tick() + 1;
    let send = lab.tick_ns(lab.k);
    let from_ns = match align {
        Align::AfterSend => send + 1,
        Align::BeforeSend => send - 1,
    };
    let to_ns = from_ns + gap_ns;
    lab.window.set(from_ns, to_ns);
    lab.run(32 + 30, &mut record);
    let f = *lab
        .tick_at
        .iter()
        .find(|(_, at)| **at >= to_ns)
        .map(|(t, _)| t)
        .expect("a tick after the gap");
    let (b_tick, evicted, f_age) = per_tick[&f];
    let fresh = lab
        .client
        .iter()
        .filter(|((s, k), h)| {
            *s == 0 && *k >= f && **h != 0 && lab.server.get(&(0, *k)) == Some(*h)
        })
        .map(|((_, k), _)| (*k, lab.tick_at[k]))
        .next();
    let sentinels = lab
        .client
        .iter()
        .filter(|((s, k), h)| *s == 0 && *k >= f && **h == 0)
        .count();
    let (compared, bad) = lab.compare();
    let resent = lab.room.client(0).map_or(0, |c| c.counters().events_resent);
    Gap {
        align,
        t,
        from_ns,
        to_ns,
        f,
        b_tick,
        b_age: b_tick.map(|b| f - b),
        evicted,
        f_age,
        fresh,
        sentinels,
        compared,
        bad,
        resent,
        server: lab.server.clone(),
    }
}

fn clean_room() -> Room<World> {
    room(&Scene::stress(), 1, 300, CLIENTS)
}

/// e5a (u4e-brief.md 2.4; U4E-Q14 (b)): at both alignments the gap leaves B at age ≥ `npt_sim::ACK_RING_TICKS` at the
/// first tick after it, and `burst_eviction` is 1 there; a 1 s gap's B is reported beside it (not asserted).
#[test]
fn e5a_a_gap_of_ack_ring_ticks_periods_evicts_b_at_both_alignments() {
    for align in [Align::AfterSend, Align::BeforeSend] {
        let g = gap_case(clean_room(), align, ring_gap_ns(npt_sim::TICK_HZ));
        println!(
            "e5a {align:?}: T {} gap {}..{} ns, F {}, B {:?} at age {:?}, evicted {}",
            g.t, g.from_ns, g.to_ns, g.f, g.b_tick, g.b_age, g.evicted
        );
        assert!(
            g.b_age.is_some_and(|a| a >= npt_sim::ACK_RING_TICKS),
            "{align:?}: {g:?}"
        );
        assert!(g.evicted, "{align:?}: burst_eviction = 1");
        let one = gap_case(clean_room(), align, 1_000_000_000);
        println!(
            "e5a {align:?}, a 1 s gap: F {}, B {:?} at age {:?}, evicted {} (reported, not asserted)",
            one.f, one.b_tick, one.b_age, one.evicted
        );
    }
}

/// e5b: the first snapshot after the gap is coded against the per-type defaults (`baseline_age` 0: §4.5).
fn e5b_check(room: impl Fn() -> Room<World>) -> Result<(), String> {
    for align in [Align::AfterSend, Align::BeforeSend] {
        let g = gap_case(room(), align, ring_gap_ns(npt_sim::TICK_HZ));
        if g.f_age != Some(0) {
            return Err(format!("{align:?}: F {} coded at age {:?}", g.f, g.f_age));
        }
    }
    Ok(())
}

#[test]
fn e5b_the_first_snapshot_after_the_gap_is_coded_against_the_defaults() {
    e5b_check(clean_room).unwrap_or_else(|e| panic!("{e}"));
}

/// 2.5: end to end, a fresh delta decodes within 500 ms of the gap's end on the injected clock, at both alignments;
/// over the whole run every compared pair is equal (the mirror across events resent after the gap: e7's resent part).
fn end_to_end_check(room: impl Fn() -> Room<World>) -> Result<Vec<Gap>, String> {
    let mut out = Vec::new();
    for align in [Align::AfterSend, Align::BeforeSend] {
        let g = gap_case(room(), align, ring_gap_ns(npt_sim::TICK_HZ));
        match g.fresh {
            Some((_, at)) if at <= g.to_ns + 500_000_000 => {}
            _ => {
                return Err(format!(
                    "{align:?}: no fresh delta within 500 ms of {} ns: {:?}",
                    g.to_ns, g.fresh
                ));
            }
        }
        out.push(g);
    }
    Ok(out)
}

#[test]
fn pm_0001_18_a_fresh_delta_decodes_within_500_ms_of_the_gaps_end() {
    let runs = end_to_end_check(clean_room).unwrap_or_else(|e| panic!("{e}"));
    for g in &runs {
        let (tick, at) = g.fresh.unwrap_or_default();
        println!(
            "2.5 {:?}: gap ends {} ns; the first fresh delta is tick {tick}, sent at {at} ns ({} ms after); {} pairs, {} \
             differ; {} events resent",
            g.align,
            g.to_ns,
            (at - g.to_ns) / 1_000_000,
            g.compared,
            g.bad,
            g.resent
        );
        assert_eq!(g.bad, 0, "{:?}", g.align);
        assert!(g.resent >= 1, "{:?}: events resent after the gap", g.align);
    }
}

/// -18's bite (bites builds only), in wave 1's two parts: the bites-only twins assert the defect on the bitten room.
#[cfg(feature = "bites")]
mod bites {
    use super::*;

    const NO_FALLBACK: &str = "PM-0001-18/no-fallback";

    fn bitten() -> Room<World> {
        clean_room()
            .with_bite(NO_FALLBACK)
            .expect("npt-room plants it")
    }

    #[test]
    fn pm_0001_18_no_fallback_codes_the_first_snapshot_after_the_gap_against_an_evicted_baseline() {
        assert!(e5b_check(bitten).is_err());
    }

    #[test]
    fn pm_0001_18_no_fallback_leaves_no_fresh_delta_within_500_ms() {
        assert!(end_to_end_check(bitten).is_err());
        for align in [Align::AfterSend, Align::BeforeSend] {
            let g = gap_case(bitten(), align, ring_gap_ns(npt_sim::TICK_HZ));
            println!(
                "2.5 bitten {align:?}: F {} coded at age {:?} (B {:?}, age {:?}); fresh {:?}; {} sentinels from F",
                g.f, g.f_age, g.b_tick, g.b_age, g.fresh, g.sentinels
            );
            assert!(
                g.fresh.is_none() && g.sentinels >= 1,
                "the client refuses every delta after the gap"
            );
        }
    }

    /// e11 (§4.11 :398-400): a bitten instance and a clean one on parallel threads in one process: the bitten one shows
    /// its defect, the clean one none, its result equal to the same run alone.
    #[test]
    fn e11_a_bitten_and_a_clean_instance_run_side_by_side_on_two_threads() {
        let gap = ring_gap_ns(npt_sim::TICK_HZ);
        let alone = gap_case(clean_room(), Align::AfterSend, gap);
        let a = std::thread::spawn(move || gap_case(bitten(), Align::AfterSend, gap));
        let b = std::thread::spawn(move || gap_case(clean_room(), Align::AfterSend, gap));
        let (a, b) = (
            a.join().expect("bitten thread"),
            b.join().expect("clean thread"),
        );
        assert!(
            a.fresh.is_none() && a.f_age != Some(0),
            "the bitten instance shows its defect"
        );
        assert!(
            b.fresh.is_some() && b.f_age == Some(0),
            "the clean instance shows none"
        );
        assert_eq!(b, alone, "the clean result equals the same run alone");
    }
}
