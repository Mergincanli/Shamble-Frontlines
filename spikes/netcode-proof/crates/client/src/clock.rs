//! ClockSync, the command clock (u4f-brief.md 2.1; u4-build-plan.md §4.8; ADR 0001:239-251): the offset from the
//! client's host clock to the server's, in signed integer ns, from the client's own pings (ADR 0001:240: "from its own
//! ping samples"). Each sample is timed by its `Pong`'s arrival stamp, never by the step that reads it, and carries the
//! room's exact time at the `Pong`'s send (U4F-Q6 (b)). The estimate averages the samples of the last 2 s (the mean:
//! D4's model put its p99 at 2.35 ms at P2, against a lowest-RTT pick's 4.856 ms) and moves toward that mean by Quake
//! III's rules (ADR 0001:240-242): at most 2 ms a step, an error above 100 ms halved, one above 500 ms reset. It reads
//! no clock: every time is a host instant passed in. The interpolation clock is U4g's (`Interp`); nothing here is for it.
//!
//! **The sample** (NTP's): the ping left at host time t0, the room took it at server time s - hold and answered at s
//! (its `Pong`'s `server_ns`), and the `Pong` arrived at host time t1; with the two one-way delays equal, the offset is
//! (t0 + t1 + hold) / 2 - s, and the round trip t1 - t0 - hold.

use std::collections::VecDeque;

/// Outstanding pings kept, at most (as the pump's `OUTSTANDING`): an older one's `Pong` is then unknown.
const OUTSTANDING: usize = 8;
/// Samples kept, at most: a bound on memory (the 2 s window holds about 20 at the honest rate).
const SAMPLES_HELD: usize = 64;
/// The filter's window: the samples that arrived in the last 2 s of host time, ns.
const WINDOW_NS: u64 = 2_000_000_000;
/// The honest ping interval, ns of host time: 10 a second, the room's limit (catalogue :91; the pump's bucket). The
/// first step at or after each interval sends one, so at 30 and 60 Hz it is every 3rd and 6th step.
const PING_EVERY_NS: u64 = 100_000_000;
/// Quake III's drift (ADR 0001:241: "at most 1–2 ms per frame"), ns per step.
const DRIFT_NS: i64 = 2_000_000;
/// An error above this is halved at each step (ADR 0001:241: "halving errors over 100 ms"), ns.
const HALVE_NS: i64 = 100_000_000;
/// An error above this resets the estimate (ADR 0001:241: "resetting over 500 ms"), ns.
const RESET_NS: i64 = 500_000_000;

/// One clock sample.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockSample {
    /// The offset, host time minus server time, ns.
    pub offset_ns: i64,
    /// The round trip less the room's hold time, ns.
    pub rtt_ns: u64,
    /// The host time the `Pong` arrived, ns.
    pub at_ns: u64,
    /// The room's next tick to run at its send, and the room's time then (ns).
    pub tick: u32,
    pub server_ns: u64,
}

/// The command clock (the module text).
#[derive(Clone, Debug)]
pub struct ClockSync {
    period_ns: u64,
    outstanding: VecDeque<(u8, u64)>,
    samples: VecDeque<ClockSample>,
    next_id: u8,
    next_ping_ns: Option<u64>,
    estimate: Option<i64>,
    unknown_pongs: u64,
    resets: u64,
    /// PM-0001-47/newest-sample (bites builds only): the estimate is the newest raw sample.
    newest_sample: bool,
}

impl ClockSync {
    /// A clock for an endpoint stepping at `tick_hz` (production callers pass `npt_sim::TICK_HZ`): its tick period in
    /// ns from that rate (carry 39: no local tick constant).
    pub fn new(tick_hz: u32) -> ClockSync {
        ClockSync {
            period_ns: 1_000_000_000 / u64::from(tick_hz.max(1)),
            outstanding: VecDeque::new(),
            samples: VecDeque::new(),
            next_id: 0,
            next_ping_ns: None,
            estimate: None,
            unknown_pongs: 0,
            resets: 0,
            newest_sample: false,
        }
    }

    /// The same clock under PM-0001-47/newest-sample (the endpoint's bite; bites builds only).
    #[cfg(feature = "bites")]
    pub(crate) fn with_newest_sample(mut self) -> ClockSync {
        self.newest_sample = true;
        self
    }

    /// The server's tick period, ns (10^9 / the rate passed in, truncated).
    pub fn period_ns(&self) -> u64 {
        self.period_ns
    }

    /// The honest ping interval, ns of host time.
    pub fn ping_every_ns() -> u64 {
        PING_EVERY_NS
    }

    /// The offset estimate, host minus server, ns: `None` before the first sample.
    pub fn offset_ns(&self) -> Option<i64> {
        self.estimate
    }

    /// The samples kept now.
    pub fn samples(&self) -> &VecDeque<ClockSample> {
        &self.samples
    }

    /// `Pong`s naming none of the client's outstanding pings, dropped.
    pub fn unknown_pongs(&self) -> u64 {
        self.unknown_pongs
    }

    /// The resets (errors above 500 ms).
    pub fn resets(&self) -> u64 {
        self.resets
    }

    /// The ping to send at a step at host time `now_ns`, if one is due (the first step at or after each interval since
    /// the first): its id, now outstanding (the oldest dropped past 8).
    pub(crate) fn ping(&mut self, now_ns: u64) -> Option<u8> {
        let due = self.next_ping_ns.unwrap_or(now_ns);
        if now_ns < due {
            return None;
        }
        let mut next = due;
        while next <= now_ns {
            next = next.saturating_add(PING_EVERY_NS);
        }
        self.next_ping_ns = Some(next);
        let id = self.next_id;
        self.next_id = id.wrapping_add(1);
        if self.outstanding.len() >= OUTSTANDING {
            self.outstanding.pop_front();
        }
        self.outstanding.push_back((id, now_ns));
        Some(id)
    }

    /// A `Pong` that arrived at host time `arrival_ns`: one sample when it answers an outstanding ping (its id and its
    /// echoed send time), else dropped and counted.
    pub(crate) fn pong(
        &mut self,
        id: u8,
        echo_ns: u64,
        tick: u32,
        hold_us: u16,
        server_ns: u64,
        arrival_ns: u64,
    ) {
        let Some(i) = self
            .outstanding
            .iter()
            .position(|&(o, sent)| o == id && sent == echo_ns)
        else {
            self.unknown_pongs += 1;
            return;
        };
        self.outstanding.remove(i);
        let hold = u64::from(hold_us) * 1_000;
        let (t0, t1) = (i128::from(echo_ns), i128::from(arrival_ns));
        let offset = (t0 + t1 + i128::from(hold)) / 2 - i128::from(server_ns);
        let rtt = arrival_ns.saturating_sub(echo_ns).saturating_sub(hold);
        if self.samples.len() >= SAMPLES_HELD {
            self.samples.pop_front();
        }
        self.samples.push_back(ClockSample {
            offset_ns: i64::try_from(offset).unwrap_or(i64::MAX),
            rtt_ns: rtt,
            at_ns: arrival_ns,
            tick,
            server_ns,
        });
    }

    /// One step at host time `now_ns`: samples older than the 2 s window go, then the estimate moves toward the
    /// window's mean by the three rules (the first sample sets it; under the newest-sample bite, the newest sample).
    pub(crate) fn step(&mut self, now_ns: u64) {
        while self
            .samples
            .front()
            .is_some_and(|s| now_ns.saturating_sub(s.at_ns) > WINDOW_NS)
        {
            self.samples.pop_front();
        }
        let Some(target) = self.mean() else {
            return;
        };
        if self.newest_sample {
            self.estimate = self.samples.back().map(|s| s.offset_ns);
            return;
        }
        self.estimate = Some(match self.estimate {
            None => target,
            Some(e) => {
                let err = target.saturating_sub(e);
                if err.unsigned_abs() > RESET_NS.unsigned_abs() {
                    self.resets += 1;
                    target
                } else if err.unsigned_abs() > HALVE_NS.unsigned_abs() {
                    e + err / 2
                } else {
                    e + err.clamp(-DRIFT_NS, DRIFT_NS)
                }
            }
        });
    }

    /// The window's mean offset, ns.
    fn mean(&self) -> Option<i64> {
        let n = self.samples.len() as i128;
        (n > 0).then(|| {
            let sum: i128 = self.samples.iter().map(|s| i128::from(s.offset_ns)).sum();
            i64::try_from(sum.div_euclid(n)).unwrap_or(i64::MAX)
        })
    }

    /// The server's next tick to run at host time `host_ns`, from the estimate and the newest sample's (tick, server
    /// time): at most one above the true one (the grid's phase is known to one period), never below it.
    pub fn next_tick_at(&self, host_ns: u64) -> Option<u32> {
        let e = self.estimate?;
        let s = self.samples.back()?;
        let server = i128::from(host_ns) - i128::from(e);
        let ticks = (server - i128::from(s.server_ns)).div_euclid(i128::from(self.period_ns)) + 1;
        Some(s.tick.wrapping_add(ticks as u32))
    }

    /// An upper estimate of the one-way delay up, ns: half the largest round trip kept plus half the spread of the
    /// round trips (the jitter margin), from the client's own samples.
    pub fn one_way_hi_ns(&self) -> Option<u64> {
        let hi = self.samples.iter().map(|s| s.rtt_ns).max()?;
        let lo = self.samples.iter().map(|s| s.rtt_ns).min()?;
        Some(hi / 2 + (hi - lo) / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MS: u64 = 1_000_000;

    /// One sample of offset `offset`: a ping sent at host `at` (forced due), its `Pong` back at `at + rtt` (hold 0),
    /// the server's send time chosen so that (t0 + t1) / 2 - s is `offset`. Host times are `at` as given; the server's
    /// stays positive for |offset| up to 10 s (the 10 s shift below).
    fn sample(c: &mut ClockSync, at: u64, rtt: u64, offset: i64) {
        let id = c.ping(at).or_else(|| {
            c.next_ping_ns = Some(at);
            c.ping(at)
        });
        let id = id.expect("a ping");
        let server = (10_000 * MS as i64 + at as i64 + (rtt / 2) as i64 - offset) as u64;
        c.pong(id, at, 7, 0, server, at + rtt);
    }

    /// The estimate less the 10 s shift `sample` puts on the server's time.
    fn est(c: &ClockSync) -> Option<i64> {
        c.offset_ns().map(|e| e + 10_000 * MS as i64)
    }

    /// f2 (ADR 0001:241, drift): an error under 100 ms moves the estimate at most 2 ms a step, toward the mean.
    #[test]
    fn f2_an_error_under_100_ms_drifts_at_most_2_ms_a_step() {
        let mut c = ClockSync::new(30);
        sample(&mut c, 0, 40 * MS, 1_000 * MS as i64);
        c.step(40 * MS);
        assert_eq!(est(&c), Some(1_000 * MS as i64), "the first sample sets it");
        // A new sample 60 ms higher: the mean moves 30 ms; the estimate drifts 2 ms a step.
        sample(&mut c, 50 * MS, 40 * MS, 1_060 * MS as i64);
        let mut last = est(&c).unwrap_or(0);
        for k in 0..20 {
            c.step(90 * MS + k * 33 * MS);
            let e = est(&c).unwrap_or(0);
            assert!((e - last).abs() <= 2 * MS as i64, "step {k}: {last} to {e}");
            last = e;
        }
        assert_eq!(last, 1_030 * MS as i64, "settled on the mean");
    }

    /// f2 (halve): an error above 100 ms and at most 500 ms is halved at each step.
    #[test]
    fn f2_an_error_over_100_ms_is_halved_each_step() {
        let mut c = ClockSync::new(30);
        sample(&mut c, 0, 40 * MS, 0);
        c.step(40 * MS);
        // Replace the window with samples 400 ms off (the old one ages out).
        let at = 2_100 * MS;
        sample(&mut c, at, 40 * MS, 400 * MS as i64);
        c.step(at + 40 * MS);
        assert_eq!(est(&c), Some(200 * MS as i64), "400 ms halved");
        c.step(at + 73 * MS);
        assert_eq!(est(&c), Some(300 * MS as i64), "200 ms halved");
        assert_eq!(c.resets(), 0);
    }

    /// f2 (reset): an error above 500 ms resets the estimate to the mean at once, counted.
    #[test]
    fn f2_an_error_over_500_ms_resets_the_estimate() {
        let mut c = ClockSync::new(30);
        sample(&mut c, 0, 40 * MS, 0);
        c.step(40 * MS);
        let at = 2_100 * MS;
        sample(&mut c, at, 40 * MS, -700 * MS as i64);
        c.step(at + 40 * MS);
        assert_eq!(est(&c), Some(-700 * MS as i64));
        assert_eq!(c.resets(), 1);
    }

    /// f3 (.claude/rules/netcode.md: no unbounded buffer keyed by client data): at most 8 pings outstanding (an
    /// older one's `Pong` is then unknown) and at most 64 samples kept, however many arrive.
    #[test]
    fn f3_outstanding_pings_and_samples_are_capped() {
        let mut c = ClockSync::new(30);
        let ids: Vec<u8> = (0..12u64)
            .filter_map(|k| c.ping(k * PING_EVERY_NS))
            .collect();
        assert_eq!((ids.len(), c.outstanding.len()), (12, OUTSTANDING));
        c.pong(ids[0], 0, 0, 0, 0, 5 * MS);
        assert_eq!(c.unknown_pongs(), 1, "the oldest is no longer outstanding");
        for k in 0..200u64 {
            sample(&mut c, 2 * MS * k, MS, 0);
        }
        assert_eq!(c.samples().len(), SAMPLES_HELD);
    }

    /// f3 (neg): a `Pong` naming none of the client's pings (another id, or its id with another echoed time) is
    /// dropped and counted, and adds no sample.
    #[test]
    fn f3_an_unknown_pong_is_dropped_and_counted() {
        let mut c = ClockSync::new(30);
        let id = c.ping(0).expect("a ping");
        c.pong(id.wrapping_add(1), 0, 0, 0, 0, MS);
        c.pong(id, 999, 0, 0, 0, MS);
        assert_eq!((c.unknown_pongs(), c.samples().len()), (2, 0));
        c.pong(id, 0, 0, 0, 0, MS);
        assert_eq!((c.unknown_pongs(), c.samples().len()), (2, 1));
    }

    /// f31 (carry 39): the clock's rate-derived values from the rate passed in, pinned at 30 and 60 Hz: the tick period
    /// in ns, and the honest ping interval as steps (every 3rd step at 30 Hz, every 6th at 60 Hz, 10 a second).
    #[test]
    fn f31_the_clocks_period_and_ping_interval_are_pinned_at_30_and_60_hz() {
        for (hz, period, every) in [(30u32, 33_333_333u64, 3u64), (60, 16_666_666, 6)] {
            let mut c = ClockSync::new(hz);
            assert_eq!(c.period_ns(), period);
            let steps: Vec<u64> = (0..61u64)
                .filter(|k| c.ping(k * 1_000_000_000 / u64::from(hz)).is_some())
                .collect();
            let gaps: Vec<u64> = steps.windows(2).map(|w| w[1] - w[0]).collect();
            assert!(gaps.iter().all(|g| *g == every), "{hz} Hz: {gaps:?}");
        }
    }
}
