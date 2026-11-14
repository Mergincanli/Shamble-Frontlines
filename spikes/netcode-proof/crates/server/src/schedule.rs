//! The loop policy (S17): how many ticks a wake runs, as a pure function of the loop's clock (u4-build-plan.md §4.3
//! and §5 U4b; plan §4.6; ADR 0004 part 5).
//!
//! The room's loop wakes on a timer but takes the number of due ticks from its monotonic clock, never from a count of
//! its wakes. Tick k of a run falls due at `base + ⌊k × 10⁹ / tick_hz⌋` ns on the loop's clock, whose zero is the
//! loop's start: the first run's base is 0. A wake runs every due tick up to the catch-up cap, which production callers
//! take from `npt_sim::LOOP_CATCHUP` (⌊0.25 s × tick_hz⌋: 7 at 30 Hz, 15 at 60 Hz). A stall longer than the cap covers
//! drops the ticks owed beyond it, counts them and their time, and rebases: the next run's tick 0 falls due when the
//! tick after the last one owed would have, so the loop is on schedule again at once. Every overrun (a wake that finds
//! more than one tick due) and every dropped interval is a counter (ADR 0004 part 5, for monitoring: ADR 0006, I-12).
//!
//! [`due_ticks`] reads no clock and does no I/O (N4): U6's tick loop reads its monotonic clock and passes the time in,
//! and npt-lab passes its simulated clock. The schedule takes the tick rate as an argument and derives nothing from
//! `npt_sim::TICK_HZ` itself (u4s-carries.md entries 6 and 35): its tests pin it at 30 and at 60 Hz.

/// Nanoseconds per second: the loop's clock counts nanoseconds.
const NS_PER_S: u128 = 1_000_000_000;

/// The loop's schedule: its tick rate, its catch-up cap, where its current run started, the next tick to run and the
/// counters so far. [`Schedule::new`] builds it; each wake's [`due_ticks`] returns the next one in [`Due::next`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Schedule {
    tick_hz: u32,
    catchup: u32,
    /// When tick 0 of the current run fell due, in ns on the loop's clock: 0 until the first drop rebases it.
    base_ns: u64,
    /// The index, in the current run, of the first tick not yet run.
    next: u64,
    counters: LoopCounters,
    /// The planted defect this instance carries (bites builds only; per instance, never a static: §4.11, T10).
    #[cfg(feature = "bites")]
    bite: Option<Bite>,
}

/// The loop's monitoring counters, each cumulative since the loop started (ADR 0004 part 5: every overrun and every
/// dropped interval is a counter).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoopCounters {
    /// Wakes that found more than one tick due: the loop overran at least one period (a slow tick or a late wake).
    pub overruns: u64,
    /// Wakes that dropped time: stalls longer than the catch-up cap covers.
    pub drops: u64,
    /// The ticks dropped, in all.
    pub dropped_ticks: u64,
    /// The time dropped, in ns, in all: the dropped-time counter (PM-0004-67's "dropped time in the counter").
    pub dropped_ns: u64,
}

/// What one wake dropped: the ticks owed beyond the catch-up cap, and their time.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dropped {
    /// The ticks dropped.
    pub ticks: u64,
    /// Their time in ns: from when the first of them fell due to when the tick after the last of them would have.
    pub ns: u64,
}

/// One wake's answer from [`due_ticks`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Due {
    /// The ticks to run now, one after another: at most the catch-up cap.
    pub to_run: u32,
    /// What this wake dropped: zero unless the stall outran the cap.
    pub dropped: Dropped,
    /// The schedule after this wake: the ticks run, any rebase, and the counters.
    pub next: Schedule,
}

impl Schedule {
    /// A schedule at `tick_hz` ticks per second that runs at most `catchup` ticks per wake, its tick 0 due at 0 ns on
    /// the loop's clock. npt-server's and npt-lab's callers pass `npt_sim::TICK_HZ` and `npt_sim::LOOP_CATCHUP`,
    /// never literals (u4s-carries.md entry 6).
    ///
    /// # Panics
    /// If `tick_hz` is not 1 to 1,000 (npt-sim's own range for `TICK_HZ`) or `catchup` is 0 (a loop that could
    /// never run a tick).
    pub const fn new(tick_hz: u32, catchup: u32) -> Schedule {
        assert!(
            tick_hz >= 1 && tick_hz <= 1_000,
            "tick_hz is 1 to 1,000 ticks per second"
        );
        assert!(
            catchup >= 1,
            "the catch-up cap is at least one tick per wake"
        );
        Schedule {
            tick_hz,
            catchup,
            base_ns: 0,
            next: 0,
            counters: LoopCounters {
                overruns: 0,
                drops: 0,
                dropped_ticks: 0,
                dropped_ns: 0,
            },
            #[cfg(feature = "bites")]
            bite: None,
        }
    }

    /// The tick rate this schedule runs at, in ticks per second.
    pub const fn tick_hz(&self) -> u32 {
        self.tick_hz
    }

    /// The most ticks one wake runs.
    pub const fn catchup(&self) -> u32 {
        self.catchup
    }

    /// The counters so far.
    pub const fn counters(&self) -> LoopCounters {
        self.counters
    }

    /// When the next tick falls due, in ns on the loop's clock. The loop sets its timer from this, never from a count
    /// of its own (ADR 0004 part 5); a wake before it runs nothing.
    pub const fn next_due_ns(&self) -> u64 {
        self.base_ns
            .saturating_add(offset_ns(self.next, self.tick_hz))
    }
}

/// When tick `k` of a run falls due, in ns after the run's base: ⌊k × 10⁹ / tick_hz⌋, worked in u128 and saturated at
/// `u64::MAX`.
const fn offset_ns(k: u64, tick_hz: u32) -> u64 {
    let ns = k as u128 * NS_PER_S / tick_hz as u128;
    if ns > u64::MAX as u128 {
        u64::MAX
    } else {
        ns as u64
    }
}

/// The last tick of a run due at `elapsed_ns` after its base: the largest k with ⌊k × 10⁹ / tick_hz⌋ ≤ elapsed_ns,
/// which is ⌊((elapsed_ns + 1) × tick_hz − 1) / 10⁹⌋ (⌊x⌋ ≤ e holds exactly when x < e + 1). At most about 1.8 × 10¹³
/// for any u64 time, so it fits in u64.
const fn last_due(elapsed_ns: u64, tick_hz: u32) -> u64 {
    (((elapsed_ns as u128 + 1) * tick_hz as u128 - 1) / NS_PER_S) as u64
}

/// The ticks a wake at `now_ns` (ns on the loop's monotonic clock since the loop started) runs under `schedule`, what
/// it drops, and the schedule after it. Pure: the same arguments always give the same answer (N4).
///
/// Every tick due by `now_ns` and not yet run is owed. A wake runs all that is owed up to the catch-up cap. If more is
/// owed, the rest is dropped and counted, and the schedule rebases so that its next tick falls due when the tick after
/// the last one owed would have: after any wake, the next tick is due after `now_ns`.
pub fn due_ticks(now_ns: u64, schedule: &Schedule) -> Due {
    let mut next = *schedule;
    let nothing = Due {
        to_run: 0,
        dropped: Dropped::default(),
        next,
    };
    if now_ns < schedule.base_ns {
        return nothing;
    }
    let last = last_due(now_ns - schedule.base_ns, schedule.tick_hz);
    if last < schedule.next {
        return nothing;
    }
    let owed = last - schedule.next + 1;
    #[cfg(feature = "bites")]
    if schedule.bite == Some(Bite::TimerPeriod) {
        // PM-0004-67/timer-period: the wake counts one tick from its timer period, whatever the clock says. The rest
        // of what is owed is lost uncounted, and the next tick falls one period after this wake.
        next.base_ns = now_ns;
        next.next = 1;
        return Due {
            to_run: 1,
            dropped: Dropped::default(),
            next,
        };
    }
    if owed > 1 {
        next.counters.overruns += 1;
    }
    let run = owed.min(cap(schedule, owed));
    next.next = schedule.next + run;
    let mut dropped = Dropped::default();
    if owed > run {
        let resume = offset_ns(last + 1, schedule.tick_hz);
        dropped = Dropped {
            ticks: owed - run,
            ns: resume - offset_ns(schedule.next + run, schedule.tick_hz),
        };
        next.base_ns = schedule.base_ns.saturating_add(resume);
        next.next = 0;
        if counts_drops(schedule) {
            next.counters.drops += 1;
            next.counters.dropped_ticks += dropped.ticks;
            next.counters.dropped_ns += dropped.ns;
        } else {
            dropped = Dropped::default();
        }
    }
    Due {
        to_run: u32::try_from(run).unwrap_or(u32::MAX),
        dropped,
        next,
    }
}

/// The most ticks this wake may run: the schedule's catch-up cap (`owed` itself under the uncapped-catchup bite).
#[cfg_attr(not(feature = "bites"), allow(unused_variables))]
fn cap(schedule: &Schedule, owed: u64) -> u64 {
    #[cfg(feature = "bites")]
    if schedule.bite == Some(Bite::UncappedCatchup) {
        // PM-0004-67/uncapped-catchup: every tick owed runs in this one wake, however many.
        return owed;
    }
    u64::from(schedule.catchup)
}

/// Whether this schedule counts what it drops (it does not under the uncounted-drop bite).
#[cfg_attr(not(feature = "bites"), allow(unused_variables))]
fn counts_drops(schedule: &Schedule) -> bool {
    #[cfg(feature = "bites")]
    if schedule.bite == Some(Bite::UncountedDrop) {
        // PM-0004-67/uncounted-drop: the excess is dropped and the schedule rebases, but nothing is counted or
        // reported.
        return false;
    }
    true
}

/// PM-0004-67's planted defects (bites builds only): each is chosen per instance by [`Schedule::with_bite`].
#[cfg(feature = "bites")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Bite {
    /// "a loop that counts ticks from its timer period instead of elapsed time"
    TimerPeriod,
    /// "an uncapped catch-up (more than 7 ticks in one wake)"
    UncappedCatchup,
    /// "a drop that is not counted in the dropped-time counter"
    UncountedDrop,
}

/// Each bite's marks.json id with its defect.
#[cfg(feature = "bites")]
const BITE_IDS: [(&str, Bite); 3] = [
    ("PM-0004-67/timer-period", Bite::TimerPeriod),
    ("PM-0004-67/uncapped-catchup", Bite::UncappedCatchup),
    ("PM-0004-67/uncounted-drop", Bite::UncountedDrop),
];

/// The bite ids npt-server plants (bites builds only): PM-0004-67's three, whose site is `due_ticks` (u4-build-plan.md
/// §5 U4b; §4.14 plants each bite where its code lives), and PM-0001-30/wallclock, whose site is the pump's command
/// path (`Decoder::with_bite`; U4R-Q6 (a)).
#[cfg(feature = "bites")]
pub const BITES: [&str; 4] = [
    BITE_IDS[0].0,
    BITE_IDS[1].0,
    BITE_IDS[2].0,
    crate::pump::WALLCLOCK_BITE,
];

#[cfg(feature = "bites")]
impl Schedule {
    /// A schedule like [`Schedule::new`]'s that carries one planted defect, by its marks.json id (bites builds only);
    /// `None` for an id this crate does not plant.
    pub fn with_bite(tick_hz: u32, catchup: u32, bite: &str) -> Option<Schedule> {
        let (_, b) = BITE_IDS.iter().find(|(id, _)| *id == bite)?;
        let mut schedule = Schedule::new(tick_hz, catchup);
        schedule.bite = Some(*b);
        Some(schedule)
    }
}
