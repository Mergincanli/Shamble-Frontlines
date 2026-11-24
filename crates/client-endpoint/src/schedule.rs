//! The local schedule: when the next tick is due in local play (ADR 0004 part 5: "in local play, the plain
//! accumulator"). It reads no clock: the Worker's host passes its own time in, in milliseconds, and sets its timer
//! from the due time it gets back, never from a count of its own (ADR 0002 part 5). Online, the client endpoint's
//! server-time estimate takes this place (Slice 3).

/// When ticks are due, counted from an anchor time. Every due time is computed from the anchor and the tick count,
/// never by adding a period to the last due time, so rounding never adds up.
pub struct LocalSchedule {
    anchor_ms: f64,
    ticks_since_anchor: u32,
    tick_hz: u32,
}

impl LocalSchedule {
    /// A schedule anchored at `now_ms`: the first tick is due one tick period later.
    ///
    /// # Panics
    /// When `tick_hz` is 0; it is the simulation's build-time constant.
    #[must_use]
    pub fn new(now_ms: f64, tick_hz: u32) -> Self {
        assert!(
            tick_hz > 0,
            "the tick rate is a build-time constant above 0"
        );
        Self {
            anchor_ms: now_ms,
            ticks_since_anchor: 0,
            tick_hz,
        }
    }

    /// The time, in the host's milliseconds, at which the next tick is due.
    #[must_use]
    pub fn next_due_ms(&self) -> f64 {
        let ticks = f64::from(self.ticks_since_anchor) + 1.0;
        self.anchor_ms + ticks * 1000.0 / f64::from(self.tick_hz)
    }

    /// Records that the tick that was due has run.
    ///
    /// # Panics
    /// When `u32::MAX` ticks have run since the anchor (4.5 years at 30 Hz), as the world's own counter does.
    pub fn ticked(&mut self) {
        self.ticks_since_anchor = self
            .ticks_since_anchor
            .checked_add(1)
            .expect("the schedule's tick count is at u32::MAX and cannot advance");
    }

    /// Starts counting again from `now_ms`: the time between the last tick and now is dropped, not caught up. The
    /// host calls it after a pause (a hidden tab) or a stall longer than one wake may catch up.
    pub fn rebase(&mut self, now_ms: f64) {
        self.anchor_ms = now_ms;
        self.ticks_since_anchor = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_tick_is_due_one_period_after_the_anchor() {
        let schedule = LocalSchedule::new(1000.0, 30);
        assert_eq!(schedule.next_due_ms(), 1000.0 + 1000.0 / 30.0);
        let schedule = LocalSchedule::new(0.0, 60);
        assert_eq!(schedule.next_due_ms(), 1000.0 / 60.0);
    }

    /// At 30 Hz every third tick falls on a whole 100 ms. A schedule that added 33.33... ms per tick would be off by
    /// then; this one is exact after 1,000,002 ticks, and never goes backwards on the way.
    #[test]
    fn a_million_ticks_add_no_drift() {
        let mut schedule = LocalSchedule::new(0.0, 30);
        let mut last = 0.0;
        for tick in 1..=1_000_002_u32 {
            let due = schedule.next_due_ms();
            assert!(due > last, "tick {tick} is due after tick {}", tick - 1);
            if tick % 3 == 0 {
                assert_eq!(due, f64::from(tick / 3) * 100.0, "tick {tick}");
            }
            last = due;
            schedule.ticked();
        }
        assert_eq!(last, 33_333_400.0);
    }

    #[test]
    fn a_rebase_drops_the_gap() {
        let mut schedule = LocalSchedule::new(0.0, 30);
        for _ in 0..30 {
            schedule.ticked();
        }
        assert_eq!(schedule.next_due_ms(), 31.0 * 1000.0 / 30.0);
        // The tab was hidden for a minute: the next tick is due one period after the return, not 1,800 ticks ago.
        schedule.rebase(61_000.0);
        assert_eq!(schedule.next_due_ms(), 61_000.0 + 1000.0 / 30.0);
        schedule.ticked();
        assert_eq!(schedule.next_due_ms(), 61_000.0 + 2000.0 / 30.0);
    }

    #[test]
    #[should_panic(expected = "the tick rate is a build-time constant above 0")]
    fn a_tick_rate_of_zero_is_refused() {
        let _ = LocalSchedule::new(0.0, 0);
    }

    #[test]
    #[should_panic(expected = "the schedule's tick count is at u32::MAX")]
    fn the_tick_count_stops_loudly_and_never_wraps() {
        let mut schedule = LocalSchedule::new(0.0, 30);
        schedule.ticks_since_anchor = u32::MAX;
        // The due time itself is still computed without overflow.
        assert!(schedule.next_due_ms().is_finite());
        schedule.ticked();
    }
}
