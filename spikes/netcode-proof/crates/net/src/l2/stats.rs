//! What ImpairSocket measures: release lateness per direction (M1), counters per fate, and the fate log.
//!
//! **Lateness** L is a datagram's release minus its due time: outbound at `try_send`'s return on the real socket,
//! inbound at `poll_recv`'s return to quinn (M1). Each window keeps a fixed histogram (1 µs buckets to 8.192 ms, then 8
//! buckets per power of two), with each bucket's sample count and sum, so memory is fixed and nothing allocates per
//! datagram. It reports p50, p99, p99.9 and max, and U3-Q1's statistic E[(L − L0 − 0.25 ms)+] with L0 the window's
//! own p50 (the constant offset the impairer may subtract: u3-scout-timing.md F1e; u3-verify-timing.md C15): exact
//! for every bucket wholly above the threshold, and within 1 µs per sample for the one bucket it falls in.

/// Linear buckets of 1 µs each, up to 8.192 ms.
const LINEAR: usize = 8_192;
/// Above that, 8 buckets per power of two of µs, from 2^13 to 2^43 µs (beyond any run).
const LOG_SUB: usize = 8;
const LOG_OCTAVES: usize = 31;
const BUCKETS: usize = LINEAR + LOG_SUB * LOG_OCTAVES;
const US: u64 = 1_000;
/// U3-Q1's allowance beyond L0.
pub(crate) const ALLOWANCE_NS: u64 = 250_000;

fn bucket_of(ns: u64) -> usize {
    let us = ns / US;
    if us < LINEAR as u64 {
        return us as usize;
    }
    let octave = 63 - us.leading_zeros() as usize; // >= 13
    let sub = ((us >> (octave - 3)) & 7) as usize;
    (LINEAR + (octave - 13) * LOG_SUB + sub).min(BUCKETS - 1)
}

/// One window's lateness samples.
pub(crate) struct Lateness {
    count: Box<[u64; BUCKETS]>,
    sum: Box<[u64; BUCKETS]>,
    n: u64,
    max: u64,
}

impl std::fmt::Debug for Lateness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lateness")
            .field("n", &self.n)
            .field("max", &self.max)
            .finish_non_exhaustive()
    }
}

/// A window's summary, in ns (0 for an empty window).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Summary {
    pub(crate) n: u64,
    pub(crate) p50: u64,
    pub(crate) p99: u64,
    pub(crate) p999: u64,
    pub(crate) max: u64,
    /// E[(L − p50 − 0.25 ms)+], rounded to whole ns.
    pub(crate) excess: u64,
    /// Samples with L − p50 > 0.25 ms.
    pub(crate) over: u64,
}

impl Lateness {
    pub(crate) fn new() -> Lateness {
        Lateness {
            count: Box::new([0; BUCKETS]),
            sum: Box::new([0; BUCKETS]),
            n: 0,
            max: 0,
        }
    }

    pub(crate) fn record(&mut self, ns: u64) {
        let b = bucket_of(ns);
        self.count[b] += 1;
        self.sum[b] = self.sum[b].saturating_add(ns);
        self.n += 1;
        self.max = self.max.max(ns);
    }

    pub(crate) fn clear(&mut self) {
        if self.n > 0 {
            self.count.fill(0);
            self.sum.fill(0);
        }
        self.n = 0;
        self.max = 0;
    }

    /// Every non-empty bucket in order: (bucket, count, sum).
    fn buckets(&self) -> impl Iterator<Item = (usize, u64, u64)> + '_ {
        (0..BUCKETS)
            .filter(|&b| self.count[b] > 0)
            .map(|b| (b, self.count[b], self.sum[b]))
    }

    pub(crate) fn summary(&self) -> Summary {
        summarize(self.n, self.max, || self.buckets())
    }
}

/// A window's summary from its sample count, its maximum and its non-empty buckets in ascending order (`buckets`
/// gives a fresh pass on each call): the nearest-rank quantiles, each the mean of its bucket's samples (exact within
/// 1 µs below 8.192 ms), and E[(L − c)+] with c = p50 + 0.25 ms, with the count above c (the threshold's own bucket
/// taken at its samples' mean). One routine for every window, so a window kept as a difference (`Running`) and a
/// whole histogram give the same answer for the same samples.
fn summarize<I: Iterator<Item = (usize, u64, u64)>>(
    n: u64,
    max: u64,
    buckets: impl Fn() -> I,
) -> Summary {
    if n == 0 {
        return Summary::default();
    }
    let quantile = |num: u64, den: u64| -> u64 {
        let rank = (u128::from(n) * u128::from(num))
            .div_ceil(u128::from(den))
            .max(1) as u64;
        let mut seen = 0u64;
        for (_, k, s) in buckets() {
            seen += k;
            if seen >= rank {
                return s / k;
            }
        }
        max
    };
    let p50 = quantile(1, 2);
    let c = p50 + ALLOWANCE_NS;
    let first = bucket_of(c);
    let (mut total, mut over) = (0u128, 0u64);
    for (b, k, s) in buckets() {
        if b < first {
            continue;
        }
        let s = u128::from(s);
        let ck = u128::from(c) * u128::from(k);
        if b == first {
            // The threshold's own bucket: its samples taken at their mean.
            if s > ck {
                total += s - ck;
                over += k;
            }
        } else {
            total += s - ck;
            over += k;
        }
    }
    Summary {
        n,
        p50,
        p99: quantile(99, 100),
        p999: quantile(999, 1000),
        max,
        excess: (total / u128::from(n)) as u64,
        over,
    }
}

/// Words of `Running`'s touched-bucket bitmap.
const WORDS: usize = BUCKETS.div_ceil(64);
/// `Running`'s sparse mark: at most this many buckets per window (reserved once), beyond which a window switches to a
/// dense mark.
const SPARSE: usize = 256;

/// The dense mark: every bucket's count and sum at the window's start.
struct Dense {
    count: Box<[u64; BUCKETS]>,
    sum: Box<[u64; BUCKETS]>,
}

/// One direction's lateness over a connection's life and its current window (DelayStream; F7), in one histogram: the
/// window is the difference between the whole and a mark of it taken at the window's start. The mark is sparse, only
/// the buckets the window touched with their count and sum as they stood before (a bitmap says which), so a window
/// costs nothing beyond its touched buckets; a window that touches more than 256 buckets copies the whole into a dense
/// mark (allocated once, at the first such window, and kept), so the memory never exceeds the two whole histograms
/// this replaces plus the 6 KB sparse part. Every summary goes through `summarize`, as `Lateness`'s does: a window's
/// answers equal a fresh `Lateness` fed the same samples (the test).
pub(crate) struct Running {
    all: Lateness,
    touched: Box<[u64; WORDS]>,
    sparse: Vec<(u16, u64, u64)>,
    dense: Option<Dense>,
    dense_on: bool,
    win_n: u64,
    win_max: u64,
}

impl std::fmt::Debug for Running {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Running")
            .field("all", &self.all)
            .field("win_n", &self.win_n)
            .field("dense_on", &self.dense_on)
            .finish_non_exhaustive()
    }
}

impl Running {
    pub(crate) fn new() -> Running {
        Running {
            all: Lateness::new(),
            touched: Box::new([0; WORDS]),
            sparse: Vec::with_capacity(SPARSE),
            dense: None,
            dense_on: false,
            win_n: 0,
            win_max: 0,
        }
    }

    pub(crate) fn record(&mut self, ns: u64) {
        let b = bucket_of(ns);
        if !self.dense_on {
            let (w, bit) = (b / 64, 1u64 << (b % 64));
            if self.touched[w] & bit == 0 {
                if self.sparse.len() == SPARSE {
                    self.go_dense();
                } else {
                    self.touched[w] |= bit;
                    self.sparse
                        .push((b as u16, self.all.count[b], self.all.sum[b]));
                }
            }
        }
        self.all.record(ns);
        self.win_n += 1;
        self.win_max = self.win_max.max(ns);
    }

    /// The window's mark in full: the whole as it stands, with each touched bucket set back to its mark.
    fn go_dense(&mut self) {
        let d = self.dense.get_or_insert_with(|| Dense {
            count: Box::new([0; BUCKETS]),
            sum: Box::new([0; BUCKETS]),
        });
        d.count.copy_from_slice(&self.all.count[..]);
        d.sum.copy_from_slice(&self.all.sum[..]);
        for &(b, c, s) in &self.sparse {
            d.count[usize::from(b)] = c;
            d.sum[usize::from(b)] = s;
        }
        self.dense_on = true;
    }

    /// The current window's summary.
    pub(crate) fn window(&mut self) -> Summary {
        self.sparse.sort_unstable_by_key(|e| e.0);
        let all = &self.all;
        let diff = move |b: usize, c: u64, s: u64| {
            let k = all.count[b] - c;
            (k > 0).then(|| (b, k, all.sum[b].saturating_sub(s)))
        };
        match (&self.dense, self.dense_on) {
            (Some(d), true) => summarize(self.win_n, self.win_max, || {
                (0..BUCKETS).filter_map(move |b| diff(b, d.count[b], d.sum[b]))
            }),
            _ => {
                let sparse = &self.sparse;
                summarize(self.win_n, self.win_max, || {
                    sparse
                        .iter()
                        .filter_map(move |&(b, c, s)| diff(usize::from(b), c, s))
                })
            }
        }
    }

    /// Ends the window: the next one starts empty.
    pub(crate) fn close_window(&mut self) {
        for &(b, _, _) in &self.sparse {
            let b = usize::from(b);
            self.touched[b / 64] &= !(1u64 << (b % 64));
        }
        self.sparse.clear();
        self.dense_on = false;
        self.win_n = 0;
        self.win_max = 0;
    }

    /// The summary over every sample.
    pub(crate) fn whole(&self) -> Summary {
        self.all.summary()
    }

    /// The heap bytes it holds: the whole histogram, the bitmap, the sparse mark's reserve and the dense mark once
    /// allocated.
    pub(crate) fn heap_bytes(&self) -> u64 {
        let hist = 2 * BUCKETS * std::mem::size_of::<u64>();
        let sparse = self.sparse.capacity() * std::mem::size_of::<(u16, u64, u64)>();
        let dense = if self.dense.is_some() { hist } else { 0 };
        (hist + WORDS * std::mem::size_of::<u64>() + sparse + dense) as u64
    }
}

/// What became of a datagram (one fate-log entry's kind; a counter each, per direction).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    /// Released on time's terms: sent (outbound) or handed to quinn (inbound).
    Delivered = 0,
    /// The profile's loss draw.
    Lost = 1,
    /// Inside one of the profile's B1 bursts.
    Burst = 2,
    /// PM-0001-29's UDP block.
    Blocked = 3,
    /// The shaper's drop-tail queue was full (M19).
    ShaperDrop = 4,
    /// A byte bound (the peer's or the total) was full: a tail drop apart from the profile's loss (M10).
    QueueDrop = 5,
    /// No pool buffer was free (M11).
    PoolDrop = 6,
    /// Above the size limit, dropped before it was queued (M10).
    Oversize = 7,
    /// The kernel refused the send with an error other than WouldBlock (M5): the errno is in the entry.
    SendError = 8,
    /// The kernel's send buffer was full (EAGAIN): a host drop, never retried (a network drops).
    WouldBlock = 9,
}

pub(crate) const KINDS: usize = 10;

impl Kind {
    pub(crate) const ALL: [Kind; KINDS] = [
        Kind::Delivered,
        Kind::Lost,
        Kind::Burst,
        Kind::Blocked,
        Kind::ShaperDrop,
        Kind::QueueDrop,
        Kind::PoolDrop,
        Kind::Oversize,
        Kind::SendError,
        Kind::WouldBlock,
    ];
}

/// The send errors by class (M5: each error kind a logged fate; the exact errno is in the fate log).
pub(crate) const ERR_CLASSES: [&str; 6] =
    ["msgsize", "nobufs", "perm", "unreach", "refused", "other"];

/// The class of a send error by its errno (Linux numbers).
pub(crate) fn err_class(errno: i32) -> usize {
    match errno {
        90 => 0,             // EMSGSIZE
        105 => 1,            // ENOBUFS
        1 | 13 => 2,         // EPERM (a firewall rule), EACCES
        101 | 113 | 99 => 3, // ENETUNREACH, EHOSTUNREACH, EADDRNOTAVAIL
        111 => 4,            // ECONNREFUSED
        _ => 5,
    }
}

/// One window's counters.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Counters {
    /// `[direction][kind]`, direction 0 up (inbound), 1 down (outbound).
    pub(crate) fates: [[u64; KINDS]; 2],
    /// Datagrams the reorder step held and their successor overtook.
    pub(crate) overtaken: [u64; 2],
    /// Delivered datagrams released after a later-indexed datagram of their flow (RFC 4737's reordered packet, in
    /// release order: the impairment layer's reorder share, M3).
    pub(crate) reordered: [u64; 2],
    pub(crate) send_err: [u64; ERR_CLASSES.len()],
    pub(crate) peers_evicted: u64,
    pub(crate) peer_overflow: u64,
    pub(crate) fate_log_full: u64,
    pub(crate) recv_err: u64,
}

/// One fate-log entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Fate {
    /// The peer's stable slot id (its first-seen ordinal; u64::MAX for the overflow flow and for drops before the
    /// table).
    pub(crate) ordinal: u64,
    pub(crate) dir: u8,
    pub(crate) kind: Kind,
    pub(crate) overtaken: bool,
    /// The index in its (peer, direction) flow; u64::MAX for a drop before the fate draws.
    pub(crate) index: u64,
    /// Outbound: try_send's call; inbound: the drain from the kernel socket. ns on the socket's clock.
    pub(crate) t_in_ns: u64,
    /// When it was due (0 if dropped before).
    pub(crate) due_ns: u64,
    /// When it was released (0 if dropped).
    pub(crate) out_ns: u64,
    pub(crate) len: u32,
    pub(crate) errno: u32,
}

/// The canonical form's first bytes: the format and its version.
pub(crate) const FATE_TAG: &[u8] = b"npt-l2 fate log v1\n";
pub(crate) const FATE_BYTES: usize = 56;

impl Fate {
    /// 56 bytes, every field fixed-width little-endian: ordinal, index, t_in, due, out (u64); len, errno (u32);
    /// direction, kind, overtaken (u8); 5 zero bytes.
    pub(crate) fn canonical(&self) -> [u8; FATE_BYTES] {
        let mut b = [0u8; FATE_BYTES];
        b[0..8].copy_from_slice(&self.ordinal.to_le_bytes());
        b[8..16].copy_from_slice(&self.index.to_le_bytes());
        b[16..24].copy_from_slice(&self.t_in_ns.to_le_bytes());
        b[24..32].copy_from_slice(&self.due_ns.to_le_bytes());
        b[32..40].copy_from_slice(&self.out_ns.to_le_bytes());
        b[40..44].copy_from_slice(&self.len.to_le_bytes());
        b[44..48].copy_from_slice(&self.errno.to_le_bytes());
        b[48] = self.dir;
        b[49] = self.kind as u8;
        b[50] = u8::from(self.overtaken);
        b
    }
}

/// The fate log: a fixed capacity, filled in order of each datagram's final fate, emptied by draining.
#[derive(Debug)]
pub(crate) struct FateLog {
    entries: Vec<Fate>,
    cap: usize,
}

impl FateLog {
    pub(crate) fn new(cap: usize) -> FateLog {
        FateLog {
            entries: Vec::with_capacity(cap),
            cap,
        }
    }

    /// Keeps `f` if there is room; false when full (the caller counts it).
    pub(crate) fn push(&mut self, f: Fate) -> bool {
        if self.cap == 0 {
            return true;
        }
        if self.entries.len() >= self.cap {
            return false;
        }
        self.entries.push(f);
        true
    }

    pub(crate) fn drain(&mut self, mut f: impl FnMut(&[u8])) {
        f(FATE_TAG);
        for e in self.entries.drain(..) {
            f(&e.canonical());
        }
    }

    #[cfg(test)]
    pub(crate) fn entries(&self) -> &[Fate] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::ChaCha8Rng;
    use rand_core::{Rng, SeedableRng};

    #[test]
    fn buckets_are_1_us_then_8_per_octave() {
        assert_eq!(bucket_of(0), 0);
        assert_eq!(bucket_of(999), 0);
        assert_eq!(bucket_of(1_000), 1);
        assert_eq!(bucket_of(8_191_999), 8_191);
        assert_eq!(bucket_of(8_192_000), LINEAR);
        assert_eq!(
            bucket_of(9_216_000),
            LINEAR + 1,
            "8,192 + 1,024 us: the next eighth of the octave"
        );
        assert_eq!(bucket_of(16_384_000), LINEAR + LOG_SUB);
        assert_eq!(bucket_of(u64::MAX), BUCKETS - 1);
    }

    #[test]
    fn an_empty_window_reports_zeros() {
        assert_eq!(Lateness::new().summary(), Summary::default());
    }

    /// The bite for the statistic: brute force on the raw samples against the histogram's answers.
    #[test]
    fn the_summary_matches_brute_force_within_a_microsecond() {
        let mut rng = ChaCha8Rng::seed_from_u64(7);
        for round in 0..4 {
            let mut l = Lateness::new();
            let mut raw = Vec::new();
            for _ in 0..20_000 {
                let u = rng.next_u64();
                // Mostly 20-120 us, 2 % 0.3-3 ms, 0.1 % 5-60 ms: the container's shape.
                let ns = match u % 1000 {
                    0 => 5_000_000 + (u >> 20) % 55_000_000,
                    1..=20 => 300_000 + (u >> 20) % 2_700_000,
                    _ => 20_000 + (u >> 20) % 100_000,
                } + round * 13;
                l.record(ns);
                raw.push(ns);
            }
            raw.sort_unstable();
            let n = raw.len();
            let q = |num: usize, den: usize| raw[(n * num).div_ceil(den).max(1) - 1];
            let s = l.summary();
            assert_eq!(s.n, n as u64);
            assert_eq!(s.max, raw[n - 1]);
            assert!(
                s.p50.abs_diff(q(1, 2)) < 1_000,
                "p50 {} vs {}",
                s.p50,
                q(1, 2)
            );
            assert!(
                s.p99.abs_diff(q(99, 100)) < 1_000,
                "p99 {} vs {}",
                s.p99,
                q(99, 100)
            );
            // Above 8.192 ms buckets are an eighth of an octave wide: within 12.5 %.
            let p999 = q(999, 1000);
            assert!(
                s.p999.abs_diff(p999) <= p999 / 8 + 1_000,
                "p999 {} vs {p999}",
                s.p999
            );
            let c = s.p50 + ALLOWANCE_NS;
            let brute: u128 = raw.iter().map(|&x| u128::from(x.saturating_sub(c))).sum();
            let brute = (brute / n as u128) as u64;
            assert!(
                s.excess.abs_diff(brute) <= 1_000,
                "excess {} vs {brute}",
                s.excess
            );
            // The threshold's own bucket counts whole, by its mean: at most its few samples apart.
            let over = raw.iter().filter(|&&x| x > c).count() as u64;
            assert!(s.over.abs_diff(over) <= 2, "over {} vs {over}", s.over);
        }
    }

    #[test]
    fn excess_counts_only_beyond_the_allowance() {
        let mut l = Lateness::new();
        for _ in 0..999 {
            l.record(50_000);
        }
        // One release 1.3 ms late: 1 ms beyond p50 + 0.25 ms, over 1,000 samples = 1 us.
        l.record(1_300_000);
        let s = l.summary();
        assert_eq!((s.p50, s.max, s.over), (50_000, 1_300_000, 1));
        assert_eq!(s.excess, 1_000);
        l.clear();
        assert_eq!(l.summary(), Summary::default());
    }

    /// F7: `Running`'s window, kept as a difference against its mark, gives exactly what a fresh `Lateness` fed the
    /// same window's samples gives, and its whole exactly what one `Lateness` fed every sample gives, over windows that
    /// stay sparse, one that goes dense (more than 256 buckets touched) and the sparse windows after it; an empty
    /// window reports zeros. The memory: one whole histogram, the bitmap and the sparse reserve, plus the dense mark
    /// only once a window needed it.
    #[test]
    fn a_running_window_equals_a_fresh_histogram_of_its_samples() {
        let mut rng = ChaCha8Rng::seed_from_u64(11);
        let mut r = Running::new();
        let mut whole = Lateness::new();
        assert!(r.sparse.capacity() >= SPARSE);
        let base = (2 * BUCKETS + WORDS) as u64 * 8 + r.sparse.capacity() as u64 * 24;
        assert_eq!(r.heap_bytes(), base);
        assert_eq!(r.window(), Summary::default());
        let mut went_dense = false;
        for (i, (samples, spread)) in [
            (40, 300_000u64),
            (1_000, 2_000_000),
            (5_000, 9_000_000),
            (30, 200_000),
            (0, 1),
            (2_000, 60_000_000),
            (300, 150_000),
        ]
        .into_iter()
        .enumerate()
        {
            let mut fresh = Lateness::new();
            for _ in 0..samples {
                let ns = 20_000 + rng.next_u64() % spread;
                r.record(ns);
                fresh.record(ns);
                whole.record(ns);
            }
            went_dense |= r.dense_on;
            assert_eq!(r.window(), fresh.summary(), "window {i}");
            assert_eq!(r.whole(), whole.summary(), "whole after window {i}");
            r.close_window();
            assert_eq!(r.window(), Summary::default(), "a new window is empty");
            assert!(r.touched.iter().all(|&w| w == 0) && !r.dense_on);
        }
        assert!(went_dense, "a window went dense");
        assert_eq!(r.heap_bytes(), base + (2 * BUCKETS * 8) as u64);
    }

    #[test]
    fn send_errors_fall_in_their_classes() {
        assert_eq!(ERR_CLASSES[err_class(90)], "msgsize");
        assert_eq!(ERR_CLASSES[err_class(105)], "nobufs");
        assert_eq!(ERR_CLASSES[err_class(1)], "perm");
        assert_eq!(ERR_CLASSES[err_class(13)], "perm");
        assert_eq!(ERR_CLASSES[err_class(113)], "unreach");
        assert_eq!(ERR_CLASSES[err_class(111)], "refused");
        assert_eq!(ERR_CLASSES[err_class(97)], "other");
    }

    #[test]
    fn the_fate_log_is_bounded_and_canonical() {
        let mut log = FateLog::new(2);
        let f = Fate {
            ordinal: 3,
            dir: 1,
            kind: Kind::SendError,
            overtaken: true,
            index: 9,
            t_in_ns: 10,
            due_ns: 20,
            out_ns: 30,
            len: 1_200,
            errno: 105,
        };
        assert!(log.push(f) && log.push(f));
        assert!(!log.push(f), "full at its capacity");
        let mut bytes = Vec::new();
        log.drain(|b| bytes.extend_from_slice(b));
        assert_eq!(&bytes[..FATE_TAG.len()], FATE_TAG);
        let e = &bytes[FATE_TAG.len()..];
        assert_eq!(e.len(), 2 * FATE_BYTES);
        assert_eq!(&e[0..8], &3u64.to_le_bytes());
        assert_eq!(&e[40..44], &1_200u32.to_le_bytes());
        assert_eq!(&e[44..48], &105u32.to_le_bytes());
        assert_eq!(&e[48..56], &[1, 8, 1, 0, 0, 0, 0, 0]);
        assert!(log.push(f), "draining frees the room");
        assert!(
            FateLog::new(0).push(f),
            "capacity 0: no log, nothing counted as full"
        );
    }
}
