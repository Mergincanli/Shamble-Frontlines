//! U4b's tests on a fake clock (u4-build-plan.md §5 U4b): the S17 loop policy at 30 Hz with catch-up 7 and at 60 Hz
//! with 15, the caps always taken from npt-sim (u4s-carries.md entry 6). Each scenario is a check that says what went
//! wrong, so the bites build shows each of PM-0004-67's bites failing the very check the clean schedule passes.

use npt_server::{Due, Schedule, due_ticks};

/// The two rates the plan pins, each with its catch-up cap from npt-sim, never a literal (entry 6).
fn rates() -> [(u32, u32); 2] {
    [
        (30, npt_sim::loop_catchup_at(30)),
        (60, npt_sim::loop_catchup_at(60)),
    ]
}

/// When tick `k` of a run based at 0 falls due at `hz`: ⌊k × 10⁹ / hz⌋ ns (the plan's formula, worked here on its own).
fn at(k: u64, hz: u32) -> u64 {
    u64::try_from(u128::from(k) * 1_000_000_000 / u128::from(hz)).expect("fits")
}

/// A millisecond in ns.
const MS: u64 = 1_000_000;

/// One wake, checked against what the scenario expects to run; returns the answer.
fn wake(s: &Schedule, now: u64, expect_run: u32, what: &str) -> Result<Due, String> {
    let due = due_ticks(now, s);
    if due.to_run != expect_run {
        return Err(format!(
            "{what}: the wake at {now} ns ran {} ticks, not {expect_run}",
            due.to_run
        ));
    }
    if due.next.next_due_ns() <= now {
        return Err(format!(
            "{what}: after the wake at {now} ns a tick is still owed (next due {})",
            due.next.next_due_ns()
        ));
    }
    Ok(due)
}

/// Every wake exactly on time for 3 s: one tick each, nothing counted.
fn on_schedule(mut s: Schedule) -> Result<(), String> {
    let hz = s.tick_hz();
    for k in 0..=u64::from(3 * hz) {
        let due = wake(&s, at(k, hz), 1, "on schedule")?;
        if due.next.next_due_ns() != at(k + 1, hz) || due.dropped != Default::default() {
            return Err(format!("on schedule: tick {k} left {due:?}"));
        }
        s = due.next;
    }
    if s.counters() != Default::default() {
        return Err(format!("on schedule: counters {:?}", s.counters()));
    }
    Ok(())
}

/// A wake late by half a period runs one tick and keeps the schedule's own times.
fn late_under_a_period(s: Schedule) -> Result<(), String> {
    let hz = s.tick_hz();
    let s = wake(&s, 0, 1, "late wake")?.next;
    let late = at(1, hz) + (at(2, hz) - at(1, hz)) / 2;
    let due = wake(&s, late, 1, "late wake")?;
    if due.next.next_due_ns() != at(2, hz) || due.next.counters() != Default::default() {
        return Err(format!("late wake: {due:?}"));
    }
    Ok(())
}

/// A 200 ms stall: the next wake runs every tick owed (6 at 30 Hz, 12 at 60 Hz: under the cap), counts one overrun,
/// drops nothing, and every later wake on the schedule's times runs one tick (back on schedule at once).
fn stall_200ms(s: Schedule) -> Result<(), String> {
    let hz = s.tick_hz();
    let owed = 200 * hz / 1_000;
    let s = wake(&s, 0, 1, "200 ms stall")?.next;
    let due = wake(&s, 200 * MS, owed, "200 ms stall")?;
    let c = due.next.counters();
    if due.dropped != Default::default() || c.overruns != 1 || c.drops != 0 || c.dropped_ns != 0 {
        return Err(format!(
            "200 ms stall: dropped {:?}, counters {c:?}",
            due.dropped
        ));
    }
    let mut s = due.next;
    for k in u64::from(owed) + 1..=u64::from(owed + hz) {
        if s.next_due_ns() != at(k, hz) {
            return Err(format!(
                "200 ms stall: tick {k} due at {}, not {}",
                s.next_due_ns(),
                at(k, hz)
            ));
        }
        s = wake(&s, at(k, hz), 1, "200 ms stall, after")?.next;
    }
    Ok(())
}

/// A 1 s stall: the next wake runs the cap (7 at 30 Hz, 15 at 60 Hz), drops the rest (23 and 45 ticks) with their time,
/// counts it, and rebases: the next tick falls due when the tick after the last one owed would have, and the loop is on
/// schedule from there.
fn stall_1s(s: Schedule) -> Result<(), String> {
    let (hz, cap) = (s.tick_hz(), s.catchup());
    let s = wake(&s, 0, 1, "1 s stall")?.next;
    let due = wake(&s, 1_000 * MS, cap, "1 s stall")?;
    let (owed, run) = (u64::from(hz), u64::from(cap));
    let dropped_ns = at(owed + 1, hz) - at(run + 1, hz);
    if due.dropped.ticks != owed - run || due.dropped.ns != dropped_ns {
        return Err(format!(
            "1 s stall: dropped {:?}, not {} ticks and {dropped_ns} ns",
            due.dropped,
            owed - run
        ));
    }
    let c = due.next.counters();
    if (c.overruns, c.drops, c.dropped_ticks, c.dropped_ns) != (1, 1, owed - run, dropped_ns) {
        return Err(format!("1 s stall: the counters {c:?}"));
    }
    let base = at(owed + 1, hz);
    let mut s = due.next;
    for i in 0..=u64::from(hz) {
        if s.next_due_ns() != base + at(i, hz) {
            return Err(format!(
                "1 s stall: after the rebase, tick {i} due at {}",
                s.next_due_ns()
            ));
        }
        s = wake(&s, base + at(i, hz), 1, "1 s stall, after")?.next;
    }
    Ok(())
}

/// The same policy worked tick by tick: steps the run's due times one at a time where `due_ticks` uses a closed form.
struct Reference {
    hz: u32,
    cap: u64,
    base: u64,
    next: u64,
}

impl Reference {
    /// (ticks run, ticks dropped, time dropped) at a wake at `now`.
    fn wake(&mut self, now: u64) -> (u64, u64, u64) {
        let mut owed = 0;
        while self.base + at(self.next + owed, self.hz) <= now {
            owed += 1;
        }
        let run = owed.min(self.cap);
        if owed == run {
            self.next += run;
            return (run, 0, 0);
        }
        let resume = at(self.next + owed, self.hz);
        let dropped_ns = resume - at(self.next + run, self.hz);
        self.base += resume;
        self.next = 0;
        (run, owed - run, dropped_ns)
    }
}

/// 20,000 wakes at seeded random gaps of 0 to 3 s: never more than the cap in one wake, never a tick left owed, each
/// answer equal to the tick-by-tick reference's, and the counters equal to the wakes' own reports.
fn random_wakes(mut s: Schedule, seed: u64) -> Result<(), String> {
    let (hz, cap) = (s.tick_hz(), s.catchup());
    let mut reference = Reference {
        hz,
        cap: u64::from(cap),
        base: 0,
        next: 0,
    };
    let (mut x, mut now) = (seed | 1, 0u64);
    let (mut overruns, mut drops, mut dropped_ticks, mut dropped_ns) = (0, 0, 0, 0);
    for w in 0..20_000 {
        // xorshift64: a fixed sequence per seed, no clock and no shared state.
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        now += x % (3_000 * MS);
        let due = due_ticks(now, &s);
        if due.to_run > cap {
            return Err(format!(
                "random wakes: wake {w} ran {} ticks, over the cap {cap}",
                due.to_run
            ));
        }
        if due.next.next_due_ns() <= now {
            return Err(format!("random wakes: wake {w} left a tick owed"));
        }
        let want = reference.wake(now);
        let got = (u64::from(due.to_run), due.dropped.ticks, due.dropped.ns);
        if got != want {
            return Err(format!(
                "random wakes: wake {w} at {now} ns gave {got:?}, the reference {want:?}"
            ));
        }
        overruns += u64::from(want.0 + want.1 > 1);
        drops += u64::from(want.1 > 0);
        dropped_ticks += want.1;
        dropped_ns += want.2;
        s = due.next;
    }
    let c = s.counters();
    if (c.overruns, c.drops, c.dropped_ticks, c.dropped_ns)
        != (overruns, drops, dropped_ticks, dropped_ns)
    {
        return Err(format!("random wakes: the counters {c:?}"));
    }
    Ok(())
}

#[test]
fn the_server_schedule_takes_its_rate_and_cap_from_npt_sim() {
    // Entry 6: the server's schedule is built from npt-sim's constants, and its cap reads back as LOOP_CATCHUP.
    let s = Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP);
    assert_eq!(s.catchup(), npt_sim::LOOP_CATCHUP);
    assert_eq!(s.tick_hz(), npt_sim::TICK_HZ);
    assert_eq!(
        npt_sim::LOOP_CATCHUP,
        npt_sim::loop_catchup_at(npt_sim::TICK_HZ)
    );
    // The cap the schedule acts on, not only the one it stores (the round-1 test finding T-B3): after a 1 s stall
    // the server's schedule runs exactly LOOP_CATCHUP ticks and drops the rest of the TICK_HZ owed.
    let stalled = due_ticks(1_000 * MS, &due_ticks(0, &s).next);
    assert_eq!(stalled.to_run, npt_sim::LOOP_CATCHUP);
    assert_eq!(
        stalled.dropped.ticks,
        u64::from(npt_sim::TICK_HZ - npt_sim::LOOP_CATCHUP)
    );
    // The two caps the plan pins (7 and 15), read from npt-sim's function.
    assert_eq!(rates(), [(30, 7), (60, 15)]);
}

#[test]
fn on_schedule_one_tick_per_period() {
    for (hz, cap) in rates() {
        on_schedule(Schedule::new(hz, cap)).unwrap();
    }
}

#[test]
fn a_late_wake_under_a_period_runs_one_tick() {
    for (hz, cap) in rates() {
        late_under_a_period(Schedule::new(hz, cap)).unwrap();
    }
}

#[test]
fn a_200_ms_stall_runs_what_is_owed_and_is_back_on_schedule() {
    for (hz, cap) in rates() {
        stall_200ms(Schedule::new(hz, cap)).unwrap();
    }
}

#[test]
fn a_1_s_stall_runs_the_cap_drops_and_counts_the_rest_and_rebases() {
    for (hz, cap) in rates() {
        stall_1s(Schedule::new(hz, cap)).unwrap();
    }
    // The plan's numbers: 7 run and 23 dropped at 30 Hz, 15 and 45 at 60 Hz, with the dropped time.
    let at30 = due_ticks(
        1_000 * MS,
        &due_ticks(0, &Schedule::new(30, npt_sim::loop_catchup_at(30))).next,
    );
    assert_eq!(
        (at30.to_run, at30.dropped.ticks, at30.dropped.ns),
        (7, 23, 766_666_667)
    );
    let at60 = due_ticks(
        1_000 * MS,
        &due_ticks(0, &Schedule::new(60, npt_sim::loop_catchup_at(60))).next,
    );
    assert_eq!(
        (at60.to_run, at60.dropped.ticks, at60.dropped.ns),
        (15, 45, 750_000_000)
    );
}

#[test]
fn never_more_than_the_cap_per_wake_and_the_counters_add_up() {
    for (hz, cap) in rates() {
        for seed in [1, 0x9e37_79b9_7f4a_7c15, 0xdead_beef] {
            random_wakes(Schedule::new(hz, cap), seed).unwrap();
        }
    }
}

#[test]
fn an_early_wake_runs_nothing_and_the_answer_is_pure() {
    for (hz, cap) in rates() {
        let s = due_ticks(0, &Schedule::new(hz, cap)).next;
        let early = due_ticks(at(1, hz) - 1, &s);
        assert_eq!(
            (early.to_run, early.dropped, early.next),
            (0, Default::default(), s)
        );
        // N4's purity is held by the types (a &Schedule in, a Copy Due out, no statics) and by the clock ban; this
        // equality only documents it (the round-1 test finding T-B2: no implementation without hidden state fails it).
        assert_eq!(due_ticks(5 * at(1, hz), &s), due_ticks(5 * at(1, hz), &s));
    }
}

#[test]
fn a_wake_1_ns_before_an_exact_due_time_does_not_run_that_tick() {
    // Where k × 10⁹ / hz is a whole number (tick 3 at 30 and at 60 Hz), the closed form's "- 1" decides whether
    // tick k is due 1 ns early. The round-1 test finding T-B1: a plant that dropped it passed every other test.
    for (hz, cap) in rates() {
        assert_eq!(u64::from(hz) * at(3, hz), 3_000_000_000);
        let s = due_ticks(0, &Schedule::new(hz, cap)).next;
        let before = due_ticks(at(3, hz) - 1, &s);
        assert_eq!((before.to_run, before.next.next_due_ns()), (2, at(3, hz)));
        let on_time = due_ticks(at(3, hz), &s);
        assert_eq!((on_time.to_run, on_time.next.next_due_ns()), (3, at(4, hz)));
    }
}

#[test]
#[should_panic(expected = "the catch-up cap is at least one tick per wake")]
fn a_zero_cap_is_refused() {
    let _ = Schedule::new(30, 0);
}

#[test]
#[should_panic(expected = "tick_hz is 1 to 1,000 ticks per second")]
fn a_zero_rate_is_refused() {
    let _ = Schedule::new(0, 7);
}

#[test]
#[should_panic(expected = "tick_hz is 1 to 1,000 ticks per second")]
fn a_rate_above_1000_is_refused() {
    let _ = Schedule::new(1_001, 7);
}

/// PM-0004-67's three bites, each shown failing the check it targets (bites builds only; T10: per instance).
#[cfg(feature = "bites")]
mod bites {
    use super::*;

    const TIMER_PERIOD: &str = "PM-0004-67/timer-period";
    const UNCAPPED: &str = "PM-0004-67/uncapped-catchup";
    const UNCOUNTED: &str = "PM-0004-67/uncounted-drop";

    fn bitten(hz: u32, cap: u32, id: &str) -> Schedule {
        Schedule::with_bite(hz, cap, id).expect("npt-server plants it")
    }

    #[test]
    fn the_three_bites_are_planted_here_under_their_marks_json_ids() {
        assert_eq!(npt_server::BITES, [TIMER_PERIOD, UNCAPPED, UNCOUNTED]);
        let marks = include_str!("../../../marks/marks.json");
        for id in npt_server::BITES {
            assert!(
                marks.contains(&format!("\"{id}\"")),
                "{id} is not in marks.json"
            );
        }
        for other in ["PM-0004-67/no-such-bite", "PM-0001-17/client-rounding", ""] {
            assert!(Schedule::with_bite(30, 7, other).is_none(), "{other}");
        }
    }

    #[test]
    fn timer_period_counting_fails_the_200_ms_stall_and_the_1_s_stall() {
        for (hz, cap) in rates() {
            let err = stall_200ms(bitten(hz, cap, TIMER_PERIOD)).unwrap_err();
            assert!(err.contains("ran 1 ticks"), "{err}");
            assert!(stall_1s(bitten(hz, cap, TIMER_PERIOD)).is_err());
            // It drifts: a late wake's lateness moves every later tick.
            assert!(late_under_a_period(bitten(hz, cap, TIMER_PERIOD)).is_err());
            assert!(random_wakes(bitten(hz, cap, TIMER_PERIOD), 1).is_err());
        }
    }

    #[test]
    fn an_uncapped_catch_up_fails_the_cap() {
        for (hz, cap) in rates() {
            let err = stall_1s(bitten(hz, cap, UNCAPPED)).unwrap_err();
            assert!(err.contains(&format!("ran {hz} ticks, not {cap}")), "{err}");
            let err = random_wakes(bitten(hz, cap, UNCAPPED), 1).unwrap_err();
            assert!(err.contains("over the cap"), "{err}");
        }
    }

    #[test]
    fn an_uncounted_drop_fails_the_dropped_time_counter() {
        for (hz, cap) in rates() {
            let err = stall_1s(bitten(hz, cap, UNCOUNTED)).unwrap_err();
            assert!(err.contains("dropped"), "{err}");
            let due = due_ticks(1_000 * MS, &due_ticks(0, &bitten(hz, cap, UNCOUNTED)).next);
            assert_eq!(
                (due.to_run, due.dropped.ns, due.next.counters().dropped_ns),
                (cap, 0, 0)
            );
        }
    }

    #[test]
    fn each_bite_leaves_the_checks_it_does_not_target_passing() {
        for (hz, cap) in rates() {
            on_schedule(bitten(hz, cap, UNCAPPED)).unwrap();
            on_schedule(bitten(hz, cap, UNCOUNTED)).unwrap();
            stall_200ms(bitten(hz, cap, UNCOUNTED)).unwrap();
            stall_200ms(bitten(hz, cap, UNCAPPED)).unwrap();
            late_under_a_period(bitten(hz, cap, UNCAPPED)).unwrap();
            late_under_a_period(bitten(hz, cap, UNCOUNTED)).unwrap();
        }
    }

    #[test]
    fn a_bitten_instance_and_a_clean_one_run_side_by_side() {
        // T10: the bite is the instance's, never a static, so a clean schedule on another thread is untouched.
        let (hz, cap) = rates()[0];
        let clean = std::thread::spawn(move || stall_1s(Schedule::new(hz, cap)));
        let bite = std::thread::spawn(move || stall_1s(bitten(hz, cap, UNCAPPED)));
        assert!(clean.join().unwrap().is_ok());
        assert!(bite.join().unwrap().is_err());
    }
}
