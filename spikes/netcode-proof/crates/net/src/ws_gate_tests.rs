//! Mechanism tests (a) and (b) on the shipped gate (u3c-plan.md 1.4, C9), with their bites. (c) runs on DelayStream
//! through TLS and tungstenite (l2::delay_tests), (e) in tcp_model::fixes.

use super::*;
use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};
use std::collections::BTreeSet;

const MS: u64 = 1_000_000;
/// The room's 30 Hz tick (fixes.rs's TICK).
const TICK: u64 = 33_333_333;
const BUDGET: u64 = 64 * 1024;
const TABLE: [u64; 6] = [6, 3, 4, 6, 8, 15];

/// N at no sample, 20, 50, 100, 150 and 300 ms.
fn table(budget: impl Fn(Option<u64>) -> u64) -> [u64; 6] {
    [
        None,
        Some(20 * MS),
        Some(50 * MS),
        Some(100 * MS),
        Some(150 * MS),
        Some(300 * MS),
    ]
    .map(budget)
}

/// (a) The table at TICK = 33,333,333 ns (advisor-ws-docs-r2.md:385; V10 recomputed it: 2 x TICK = 66,666,666).
#[test]
fn a_the_budget_table_at_30_hz() {
    assert_eq!(table(|s| snapshot_budget(s, TICK)), TABLE);
    // The writer's own N is the same function.
    let w = TickWriter::new(TICK);
    assert_eq!(table(|s| w.budget(s)), TABLE);
}

/// (d) budget_scale 2 fails (a).
#[test]
fn bite_a_budget_twice_the_formula_fails_a() {
    let w = TickWriter::new(TICK).with_bite(Bite::BudgetX2);
    assert_ne!(table(|s| w.budget(s)), TABLE);
}

/// (b) N never decreases as srtt grows and never falls below 3: every whole µs from 0 to 3 s, and seeded large values.
#[test]
fn b_the_budget_never_decreases_and_never_falls_below_3() {
    let mut last = 0;
    for us in 0..=3_000_000u64 {
        let n = snapshot_budget(Some(us * 1_000), TICK);
        assert!(n >= MIN_SNAPSHOTS, "{n} at {us} µs");
        assert!(n >= last, "N fell from {last} to {n} at {us} µs");
        last = n;
    }
    let mut rng = ChaCha8Rng::seed_from_u64(0x6e70_745f_6761_7465);
    let mut v: Vec<u64> = (0..100_000).map(|_| rng.next_u64() >> 20).collect();
    v.sort_unstable();
    let ns: Vec<u64> = v.iter().map(|&s| snapshot_budget(Some(s), TICK)).collect();
    assert!(ns.windows(2).all(|w| w[0] <= w[1]));
    assert!(ns.iter().all(|&n| n >= MIN_SNAPSHOTS));
}

/// What one seeded sweep saw: writes, and the skips by the first condition that failed.
#[derive(Debug, Default)]
struct Seen {
    writes: u64,
    skip_owed: u64,
    skip_free: u64,
    skip_budget: u64,
}

/// (b) One seeded sweep of the shipped writer over `ticks` ticks of a simulated socket: the network acknowledges a
/// random share each tick, srtt has no sample at first and then wanders from 5 ms to 2 s, a resume's FullState now and
/// then leaves bytes owed above the socket for a few ticks, the room is sometimes short, and each tick's snapshot is
/// 100 B to 4 KB. Returns the first broken property, if any.
///
/// What can fail here (the critic's note): "a write iff the three conditions hold" (the conditions restated
/// independently below; the budget x2 bite breaks it), "only the newest" and "a skipped snapshot never written later"
/// (the queueing bite breaks them), the counts, and -31's "at most one above the socket". Two of plan 1.4 (b)'s
/// properties hold by construction and are not asserted, since nothing could make them fail: "outstanding <= N x
/// size after a write" is the third condition (outstanding + size <= N x size) read after the write, so it follows
/// from the "iff" check with the same N; and "writes happen only on a tick" holds because `on_tick` is TickWriter's
/// only way to write.
fn sweep(bite: Option<Bite>, seed: u64, ticks: u64) -> Result<Seen, String> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut w = TickWriter::new(TICK);
    if let Some(b) = bite {
        w = w.with_bite(b);
    }
    let mut below = |n: u64| ((u128::from(rng.next_u64()) * u128::from(n)) >> 64) as u64;
    let (mut outstanding, mut owed) = (0u64, 0u64);
    let mut srtt: Option<u64> = None;
    let mut skipped = BTreeSet::new();
    let mut seen = Seen::default();
    for tick in 0..ticks {
        // The network: up to everything outstanding acknowledged this tick (often nothing: a burst).
        if below(4) != 0 {
            outstanding -= below(outstanding + 1);
        }
        if tick > 30 && (srtt.is_none() || below(10) == 0) {
            srtt = Some(5 * MS + below(2_000 * MS));
        }
        if below(60) == 0 {
            owed = 1 + below(70_000);
        } else {
            owed = owed.saturating_sub(below(20_000));
        }
        let size = 100 + below(4_000);
        let room = BUDGET.saturating_sub(outstanding);
        let free = if below(15) == 0 {
            below(room + 1)
        } else {
            room
        };
        let inp = Inputs {
            srtt_ns: srtt,
            outstanding,
            owed,
            free,
        };
        // The three conditions, restated here (fixes.rs's room step; W22).
        let n = snapshot_budget(srtt, TICK);
        let cond = owed == 0 && free >= size && outstanding + size <= n * size;
        let got = w.on_tick(Snap { id: tick, size }, inp);
        if got.is_some() != cond {
            return Err(format!(
                "tick {tick}: wrote {} but the conditions say {cond} ({inp:?}, size {size}, N {n})",
                got.is_some()
            ));
        }
        match got {
            Some(s) => {
                if s.id != tick {
                    return Err(format!(
                        "tick {tick}: wrote snapshot {}, not the newest",
                        s.id
                    ));
                }
                if skipped.contains(&s.id) {
                    return Err(format!("tick {tick}: wrote skipped snapshot {}", s.id));
                }
                outstanding += s.size;
                seen.writes += 1;
            }
            None => {
                skipped.insert(tick);
                if owed > 0 {
                    seen.skip_owed += 1;
                } else if free < size {
                    seen.skip_free += 1;
                } else {
                    seen.skip_budget += 1;
                }
            }
        }
        // PM-0001-31's "at most one snapshot above the socket": the writer holds none; the socket may owe one.
        let above = w.held() + usize::from(owed > 0);
        if above > 1 {
            return Err(format!("tick {tick}: {above} snapshots above the socket"));
        }
    }
    if w.written() != seen.writes || w.skipped() != ticks - seen.writes {
        return Err(format!(
            "counts {} written, {} skipped; seen {seen:?}",
            w.written(),
            w.skipped()
        ));
    }
    Ok(seen)
}

/// (b) Seeded sweeps (proptest is not in the workspace): 64 seeds x 20,000 ticks. Every property holds, and each
/// skip reason is exercised.
#[test]
fn b_the_writer_writes_iff_the_three_conditions_hold_and_only_the_newest() {
    let mut total = Seen::default();
    for seed in 0..64 {
        let s = sweep(None, seed, 20_000).unwrap_or_else(|e| panic!("seed {seed}: {e}"));
        total.writes += s.writes;
        total.skip_owed += s.skip_owed;
        total.skip_free += s.skip_free;
        total.skip_budget += s.skip_budget;
    }
    println!("(b) 64 x 20,000 ticks: {total:?}");
    assert!(total.writes > 100_000, "{total:?}");
    assert!(
        total.skip_owed > 1_000 && total.skip_free > 1_000 && total.skip_budget > 1_000,
        "every reason exercised: {total:?}"
    );
}

/// (d) A queueing writer fails (b) (a skipped snapshot written later, not the newest) and -31's "at most one snapshot
/// above the socket".
#[test]
fn bite_a_queueing_writer_fails_b() {
    let e = sweep(Some(Bite::Queueing), 1, 20_000).expect_err("the queueing writer is caught");
    assert!(
        e.contains("not the newest")
            || e.contains("skipped snapshot")
            || e.contains("above the socket"),
        "{e}"
    );
    // And -31's check on its own: a writer that queues holds more than one snapshot once two ticks are skipped.
    let mut w = TickWriter::new(TICK).with_bite(Bite::Queueing);
    let full = Inputs {
        srtt_ns: None,
        outstanding: BUDGET,
        owed: 0,
        free: 0,
    };
    for id in 0..3 {
        assert_eq!(w.on_tick(Snap { id, size: 1_050 }, full), None);
    }
    assert_eq!(w.held(), 3);
    // The faithful writer holds none.
    let mut f = TickWriter::new(TICK);
    for id in 0..3 {
        assert_eq!(f.on_tick(Snap { id, size: 1_050 }, full), None);
    }
    assert_eq!((f.held(), f.skipped()), (0, 3));
}

/// (d) budget_scale 2 also breaks (b)'s "write iff the three conditions hold".
#[test]
fn bite_a_budget_twice_the_formula_fails_b() {
    let e = sweep(Some(Bite::BudgetX2), 2, 20_000).expect_err("caught");
    assert!(e.contains("conditions say false"), "{e}");
}

/// The three conditions one by one (W22).
#[test]
fn may_write_needs_all_three_conditions() {
    let n = 6;
    assert!(may_write(5 * 1_050, 0, 1_050, 1_050, n));
    assert!(!may_write(5 * 1_050 + 1, 0, 1_050, 1_050, n), "the budget");
    assert!(!may_write(0, 1, 65_536, 1_050, n), "owed");
    assert!(!may_write(0, 0, 1_049, 1_050, n), "the room");
    // W22's case: N x size above 64 KB (N >= 63 at 1,050 B) and the room short: the room decides.
    assert!(!may_write(60_000, 0, 1_000, 1_050, 70));
}
