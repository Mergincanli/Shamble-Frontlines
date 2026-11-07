//! The shaper of PM-0001-23 and -24 (M19; u3-scout-wt.md F5.1-F5.5): a FIFO serialising at rate R with a drop-tail
//! limit in bytes, each datagram counted as its UDP payload plus 28 B (ADR 0001's per-client cap convention), placed
//! before the profile's delay. A shaper, not a policer: a datagram waits its turn, and only one that would overflow
//! the queue is dropped.
//!
//! The queue is the fluid backlog: the bytes not yet serialised at time t, `(free_at − t) × R ÷ 8 s`, the one in
//! service included. A datagram of b bytes (payload + 28) offered at t is dropped if backlog(t) + b exceeds the limit;
//! else it starts at max(t, free_at) and leaves b × 8 s ÷ R later, which becomes free_at. The state is kept exactly,
//! as time × R (bit-ns per s), so no rounding moves a drop: a limit of 4 full datagrams takes 4 offered at once and
//! drops the 5th, and takes it at the very ns the first has left, not 1 ns before. Departures are rounded up to whole
//! ns.
//!
//! A shaper made when the settings change mid-run (a throttle's start or end) starts busy until the flow's last
//! departure ([`Shaper::busy_until`]): the time still owed at that instant counts as backlog at the new rate, so its
//! first departure is never before the old shaper's last.

/// The bytes each datagram counts beyond its UDP payload: IPv4 (20 B) and UDP (8 B) headers.
pub(crate) const OVERHEAD_BYTES: u64 = 28;

const NS_PER_S: u128 = 1_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Shaper {
    pub(crate) bits_per_s: u64,
    pub(crate) limit_bytes: u64,
    /// When the last accepted datagram's last bit leaves, as ns × the rate (exact).
    free_at_scaled: u128,
}

impl Shaper {
    pub(crate) fn new(bits_per_s: u64, limit_bytes: u64) -> Shaper {
        Shaper {
            bits_per_s: bits_per_s.max(1),
            limit_bytes,
            free_at_scaled: 0,
        }
    }

    /// A shaper whose line is busy until `free_at_ns` (the flow's last departure when the settings changed).
    pub(crate) fn busy_until(bits_per_s: u64, limit_bytes: u64, free_at_ns: u64) -> Shaper {
        let mut s = Shaper::new(bits_per_s, limit_bytes);
        s.free_at_scaled = u128::from(free_at_ns) * u128::from(s.bits_per_s);
        s
    }

    /// The time to serialise `bytes` at the rate, rounded up to whole ns.
    #[cfg(test)]
    pub(crate) fn tx_ns(&self, bytes: u64) -> u64 {
        let num = u128::from(bytes) * 8 * NS_PER_S;
        num.div_ceil(u128::from(self.bits_per_s)) as u64
    }

    /// The unserialised work at `t_ns`, as bytes × 8 × 10^9 (exact).
    fn backlog_scaled(&self, t_ns: u64) -> u128 {
        self.free_at_scaled
            .saturating_sub(u128::from(t_ns) * u128::from(self.bits_per_s))
    }

    /// The bytes not yet serialised at `t_ns`, rounded down.
    #[cfg(test)]
    pub(crate) fn backlog_bytes(&self, t_ns: u64) -> u64 {
        (self.backlog_scaled(t_ns) / (8 * NS_PER_S)) as u64
    }

    /// Offers a datagram of `payload` bytes at `t_ns`: its departure (its last bit out, rounded up to whole ns), or
    /// `None` for a tail drop.
    pub(crate) fn offer(&mut self, t_ns: u64, payload: u64) -> Option<u64> {
        let b = u128::from(payload + OVERHEAD_BYTES) * 8 * NS_PER_S;
        if self.backlog_scaled(t_ns) + b > u128::from(self.limit_bytes) * 8 * NS_PER_S {
            return None;
        }
        let now = u128::from(t_ns) * u128::from(self.bits_per_s);
        self.free_at_scaled = self.free_at_scaled.max(now) + b;
        Some(self.free_at_scaled.div_ceil(u128::from(self.bits_per_s)) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::super::L2Config;
    use super::super::config::{bottleneck_queue_bytes, queue_ns, throttle_queue_bytes};
    use super::*;

    const MS: u64 = 1_000_000;

    #[test]
    fn the_rulings_queue_depths() {
        // The advisor's ruling: a full datagram is 1,452 B + 28 B = 1,480 B; -23 holds 4 of them.
        assert_eq!(L2Config::FULL_DATAGRAM_BYTES, 1_452 + OVERHEAD_BYTES);
        assert_eq!(throttle_queue_bytes(), 5_920);
        // -24: max(200 ms at R, 5,920 B). Under the 256 kbps cap (R <= 128 kbps) the 4-datagram floor wins.
        assert_eq!(bottleneck_queue_bytes(64_000), 5_920);
        assert_eq!(bottleneck_queue_bytes(128_000), 5_920);
        // 200 ms at 236.8 kbps is exactly 5,920 B; above it the time term wins.
        assert_eq!(bottleneck_queue_bytes(236_800), 5_920);
        assert_eq!(bottleneck_queue_bytes(400_000), 10_000);
        assert_eq!(bottleneck_queue_bytes(1_000_000), 25_000);
        // As time at the rate: 5,920 B at 128 kbps is 370 ms; 10,000 B at 400 kbps is 200 ms.
        assert_eq!(queue_ns(5_920, 128_000), 370 * MS);
        assert_eq!(queue_ns(10_000, 400_000), 200 * MS);
        let c = L2Config::new(1, npt_l1::Profile::p2()).with_throttle();
        assert_eq!(c.shaper, Some((128_000, 5_920)));
        let c = L2Config::new(1, npt_l1::Profile::p2()).with_bottleneck(64_000);
        assert_eq!(c.shaper, Some((64_000, 5_920)));
    }

    #[test]
    fn serialisation_counts_payload_plus_28_bytes() {
        let s = Shaper::new(128_000, 5_920);
        // 1,480 B at 16,000 B/s: 92.5 ms.
        assert_eq!(s.tx_ns(1_480), 92_500_000);
        let mut s = Shaper::new(128_000, 5_920);
        assert_eq!(s.offer(0, 1_452), Some(92_500_000));
        // A 100 B payload is 128 B: 8 ms at 16,000 B/s, after the first.
        assert_eq!(s.offer(0, 100), Some(100_500_000));
    }

    #[test]
    fn four_full_datagrams_fit_and_the_fifth_is_a_tail_drop() {
        let mut s = Shaper::new(128_000, throttle_queue_bytes());
        for k in 1..=4u64 {
            assert_eq!(
                s.offer(0, 1_452),
                Some(k * 92_500_000),
                "datagram {k} queued"
            );
        }
        assert_eq!(s.offer(0, 1_452), None, "the 5th overflows 5,920 B");
        assert_eq!(s.backlog_bytes(0), 5_920);
        // Once one datagram's time has passed, one more fits; a ns earlier it does not.
        assert_eq!(s.offer(92_500_000 - 1, 1_452), None);
        assert_eq!(s.offer(92_500_000, 1_452), Some(5 * 92_500_000));
        // The queue drains at the rate: half way through the next datagram, 740 B of it are left.
        assert_eq!(s.backlog_bytes(4 * 92_500_000 + 46_250_000), 740);
        assert_eq!(s.backlog_bytes(5 * 92_500_000), 0);
    }

    #[test]
    fn a_shaper_made_mid_run_starts_busy_until_the_last_departure() {
        // -23's throttle made at 100 ms while the flow's last departure is at 300 ms: 200 ms owed, 3,200 B of backlog.
        let mut s = Shaper::busy_until(128_000, throttle_queue_bytes(), 300 * MS);
        assert_eq!(s.backlog_bytes(100 * MS), 3_200);
        assert_eq!(
            s.offer(100 * MS, 1_452),
            Some(300 * MS + 92_500_000),
            "after the last departure, at the rate"
        );
        assert_eq!(s.offer(100 * MS, 1_452), None, "3,200 + 2 x 1,480 > 5,920");
        // Busy until a time already past: as a new shaper.
        let mut s = Shaper::busy_until(128_000, throttle_queue_bytes(), 10 * MS);
        assert_eq!(s.offer(20 * MS, 1_452), Some(20 * MS + 92_500_000));
    }

    #[test]
    fn an_idle_shaper_sends_at_once_after_its_serialisation() {
        let mut s = Shaper::new(1_000_000, 10_000);
        // 1,000 B payload = 1,028 B at 125,000 B/s = 8.224 ms.
        assert_eq!(s.offer(5 * MS, 1_000), Some(5 * MS + 8_224_000));
        // Long after, the queue is empty and the next starts at its own time.
        assert_eq!(s.offer(100 * MS, 1_000), Some(100 * MS + 8_224_000));
    }

    #[test]
    fn a_steady_overload_holds_the_queue_at_its_limit() {
        // Offer 2x the rate for 10 s: the departures keep the rate, and the drops are about half.
        let rate = 64_000u64; // 8,000 B/s
        let mut s = Shaper::new(rate, bottleneck_queue_bytes(rate));
        let (mut sent, mut dropped, mut last) = (0u64, 0u64, 0u64);
        for i in 0..160u64 {
            // 1,000 B payload every 62.5 ms = 16,448 B/s offered.
            match s.offer(i * 62_500_000, 1_000) {
                Some(d) => {
                    assert!(d >= last, "FIFO: departures in order");
                    last = d;
                    sent += 1;
                }
                None => dropped += 1,
            }
            assert!(s.backlog_bytes(i * 62_500_000) <= 5_920);
        }
        // 10 s at 8,000 B/s is 77.8 datagrams of 1,028 B, plus the 5 the queue held at the end.
        assert!((78..=84).contains(&sent), "sent {sent}");
        assert_eq!(sent + dropped, 160);
    }
}
