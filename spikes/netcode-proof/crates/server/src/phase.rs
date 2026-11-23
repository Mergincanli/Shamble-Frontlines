//! [`PhaseTimers`] (plan.md:601's list; plan E2 "per-phase timers and counters", E18 "monotonic reads at tick and phase
//! edges"): the tick thread's fixed-size record of one tick, its clock read at the tick's start and at each phase edge
//! (the inputs handed to the room, the room's tick, the sends), with the wake's schedule values beside them. It reads
//! no clock: the tick thread passes each read in (`clock.rs`'s one item), and a test passes its own.
//!
//! U6a writes the loop's five series from it (SERIES.md: `tick.v.work_ns`, `start_jitter_ns`, `dropped_ns`, `overrun`,
//! `wake_ticks_count`); the room-phase series are U6d's and U6e's, from the same edges.

/// One tick's timer record (the module text). Every time is ns on the loop's clock, whose zero is the loop's start.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhaseTimers {
    scheduled_ns: u64,
    start_ns: u64,
    inputs_ns: u64,
    room_ns: u64,
    end_ns: u64,
    wake_ticks: u32,
    dropped_ns: u64,
    overrun: bool,
}

impl PhaseTimers {
    /// The record of a tick that fell due at `scheduled_ns` and started at `start_ns`, run in a wake of `wake_ticks`
    /// ticks. The wake's dropped time and overrun go on its first tick only (`first`), so each series' sum over a run
    /// is the loop's own counter (SERIES.md: "the loop's overrun counter's increment"; -4 and PM-0004-67 sum them).
    pub fn start(
        scheduled_ns: u64,
        start_ns: u64,
        wake_ticks: u32,
        first: bool,
        dropped_ns: u64,
        overrun: bool,
    ) -> PhaseTimers {
        PhaseTimers {
            scheduled_ns,
            start_ns,
            inputs_ns: start_ns,
            room_ns: start_ns,
            end_ns: start_ns,
            wake_ticks,
            dropped_ns: if first { dropped_ns } else { 0 },
            overrun: first && overrun,
        }
    }

    /// The edge after the tick's inputs (decoded messages and reports) were handed to the room.
    pub fn inputs_done(&mut self, now_ns: u64) {
        self.inputs_ns = now_ns.max(self.start_ns);
        self.room_ns = self.inputs_ns;
        self.end_ns = self.inputs_ns;
    }

    /// The edge after the room's tick (commands, the step, interest, encoding).
    pub fn room_done(&mut self, now_ns: u64) {
        self.room_ns = now_ns.max(self.inputs_ns);
        self.end_ns = self.room_ns;
    }

    /// The tick's end: its sends handed to the I/O runtime.
    pub fn end(&mut self, now_ns: u64) {
        self.end_ns = now_ns.max(self.room_ns);
    }

    /// The tick's start on the loop's clock, ns.
    pub fn start_ns(&self) -> u64 {
        self.start_ns
    }

    /// `tick.v.work_ns`: the tick thread's work from the tick's start to its end, measured directly.
    pub fn work_ns(&self) -> u64 {
        self.end_ns - self.start_ns
    }

    /// `tick.v.start_jitter_ns`: the actual minus the scheduled start (never below 0: a tick runs only once due).
    pub fn start_jitter_ns(&self) -> u64 {
        self.start_ns.saturating_sub(self.scheduled_ns)
    }

    /// `tick.v.dropped_ns`: the time the wake dropped, on its first tick; 0 on the rest.
    pub fn dropped_ns(&self) -> u64 {
        self.dropped_ns
    }

    /// `tick.v.overrun`: whether the wake found more than one tick due, on its first tick; false on the rest.
    pub fn overrun(&self) -> bool {
        self.overrun
    }

    /// `tick.v.wake_ticks_count`: the ticks run in this tick's wake.
    pub fn wake_ticks(&self) -> u32 {
        self.wake_ticks
    }

    /// The phases' lengths, ns: the inputs, the room's tick and the sends (for U6d's and U6e's series).
    pub fn phases_ns(&self) -> [u64; 3] {
        [
            self.inputs_ns - self.start_ns,
            self.room_ns - self.inputs_ns,
            self.end_ns - self.room_ns,
        ]
    }
}
