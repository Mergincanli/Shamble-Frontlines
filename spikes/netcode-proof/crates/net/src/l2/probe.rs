//! The -3 L2 probe's shared pieces (u6c-brief.md Block 1; SERIES.md:497-501; lab/src/probe.rs:1-11, whose design the
//! L2 probe keeps): npt-server's probe mode runs the legs, npt-load's probe mode is LG's end, both through this module.
//!
//! - **Promoted from U3d's test code** (C-U6-1, C-U6-5), so the self-test, T1, T2, T5 and T7 run on the probe's own
//!   code: [`start_poller`] (quinn's way of polling, a task on the caller's multi-thread runtime), [`FateEntry`] (the
//!   fate log's canonical entries), [`samples`], [`nearest_rank`], [`quantile`], [`excess`] and [`forms`].
//! - **The datagram** both ends share: [`ProbeDatagram`], one fixed length.
//! - **The pacer** ([`Pacer`]): a dedicated std thread with 1 ns timer slack, never tokio (M13), calling its closure
//!   at start + interval / 2 + i × interval as L1 schedules (lab/src/probe.rs:113).
//! - **LG's end** ([`probe_peer`]): one UDP socket for the whole probe, sending up at the rate from its start to its
//!   duration without a pause, each up datagram carrying the newest down datagram it has received and not yet echoed
//!   with its hold, or none (C2, C3); errors counted, never fatal. It writes no probe series.
//! - **RS's down stream** ([`send_down`]) and **the leg's series** ([`leg_series`]): every probe series from the
//!   socket's stamps (the drain stamp and each release), the round trip joined in delivery order.

use super::socket::ImpairSocket;
use super::stats::{ALLOWANCE_NS, FATE_BYTES, FATE_TAG, KINDS};
use super::thread;
use quinn::AsyncUdpSocket;
use quinn::udp::{RecvMeta, Transmit};
use std::io::{self, IoSliceMut};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering::Relaxed};
use std::sync::{Arc, Mutex, PoisonError};
use std::task::Poll;
use std::time::{Duration, Instant};

/// What a poller saw: the datagrams `poll_recv` handed it, and when it first returned one.
#[derive(Debug, Clone, Copy)]
pub struct Polled {
    pub got: u64,
    pub first_at: Option<Instant>,
}

/// Starts the inbound poller (C-U6-1; U3D-POLL (b), R-F1): a task spawned on `rt` as quinn spawns its endpoint driver
/// (quinn-0.11.12 endpoint.rs:152-160), never `block_on` and never a thread of its own. On each wake it calls
/// `poll_recv` with quinn's 32 buffers of 1,472 B until Pending (endpoint.rs:772ff), handing each datagram to `each`
/// with `state`, in the order `poll_recv` released it. The stop is polled only after Pending, so its lock never comes
/// before a stamp (R-F9); `notify_one` keeps a permit, so no stop is lost. The task gives `state` back with its count.
pub fn start_poller<S, F>(
    rt: &tokio::runtime::Handle,
    s: &Arc<ImpairSocket>,
    stop: &Arc<tokio::sync::Notify>,
    mut state: S,
    mut each: F,
) -> tokio::task::JoinHandle<(Polled, S)>
where
    S: Send + 'static,
    F: FnMut(&mut S, &[u8], SocketAddr) + Send + 'static,
{
    const SIZE: usize = super::L2Config::DEFAULT_MAX_DATAGRAM_BYTES;
    const BATCH: usize = quinn::udp::BATCH_SIZE;
    let (s, stop) = (Arc::clone(s), Arc::clone(stop));
    rt.spawn(async move {
        // One buffer cut into quinn's 32, made once (quinn's recv_buf: endpoint.rs:754-759, :781-791).
        let mut recv_buf = vec![0u8; SIZE * BATCH];
        let mut metas = [RecvMeta::default(); BATCH];
        let mut stopped = std::pin::pin!(stop.notified());
        let mut polled = Polled {
            got: 0,
            first_at: None,
        };
        std::future::poll_fn(|cx| {
            let mut chunks = recv_buf.chunks_mut(SIZE).map(IoSliceMut::new);
            let mut iovs: [IoSliceMut<'_>; BATCH] =
                std::array::from_fn(|_| chunks.next().expect("BATCH chunks"));
            loop {
                match s.poll_recv(cx, &mut iovs, &mut metas) {
                    Poll::Ready(Ok(n)) => {
                        polled.first_at.get_or_insert_with(Instant::now);
                        polled.got += n as u64;
                        for (iov, m) in iovs.iter().zip(&metas).take(n) {
                            each(&mut state, &iov[..m.len], m.addr);
                        }
                    }
                    Poll::Ready(Err(e)) => panic!("poll_recv errored: {e}"),
                    Poll::Pending => break,
                }
            }
            stopped.as_mut().poll(cx)
        })
        .await;
        (polled, state)
    })
}

/// One fate-log entry as `ImpairSocket::drain_fate_log`'s canonical bytes carry it (stats.rs:499-516: 56 B, every
/// field little-endian), the probe's only view of the log.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FateEntry {
    /// The peer's stable slot id (u64::MAX for the overflow flow and for drops before the table).
    pub ordinal: u64,
    /// The index in its (peer, direction) flow; u64::MAX for a drop before the fate draws.
    pub index: u64,
    /// Down: `try_send`'s call; up: the drain from the kernel socket (the socket's clock, ns).
    pub t_in_ns: u64,
    pub due_ns: u64,
    /// The release (0 if dropped).
    pub out_ns: u64,
    pub len: u32,
    pub errno: u32,
    /// 0 up (inbound), 1 down.
    pub dir: u8,
    /// stats.rs's `Kind` by number: 0 Delivered, 1 Lost, 2 Burst, 3 Blocked, 4 ShaperDrop, 5 QueueDrop, 6 PoolDrop,
    /// 7 Oversize, 8 SendError, 9 WouldBlock.
    pub kind: u8,
    pub overtaken: bool,
}

impl FateEntry {
    pub const DELIVERED: u8 = 0;
    pub const LOST: u8 = 1;

    /// One entry from its canonical bytes; `None` for another length, a direction or kind out of range, or padding
    /// that is not zero.
    pub fn parse(b: &[u8]) -> Option<FateEntry> {
        if b.len() != FATE_BYTES
            || b[48] > 1
            || usize::from(b[49]) >= KINDS
            || b[50] > 1
            || b[51..].iter().any(|&x| x != 0)
        {
            return None;
        }
        let w = |at: usize| u64::from_le_bytes(b[at..at + 8].try_into().unwrap_or_default());
        let h = |at: usize| u32::from_le_bytes(b[at..at + 4].try_into().unwrap_or_default());
        Some(FateEntry {
            ordinal: w(0),
            index: w(8),
            t_in_ns: w(16),
            due_ns: w(24),
            out_ns: w(32),
            len: h(40),
            errno: h(44),
            dir: b[48],
            kind: b[49],
            overtaken: b[50] == 1,
        })
    }

    /// A whole canonical log (its tag, then 56 B per entry), refused for another tag or a length off the entries.
    pub fn parse_log(bytes: &[u8]) -> Result<Vec<FateEntry>, String> {
        let body = bytes
            .strip_prefix(FATE_TAG)
            .ok_or("the fate log's tag is not npt-l2's v1")?;
        if !body.len().is_multiple_of(FATE_BYTES) {
            return Err(format!(
                "{} bytes after the tag: not whole entries",
                body.len()
            ));
        }
        body.chunks(FATE_BYTES)
            .map(|c| FateEntry::parse(c).ok_or_else(|| "an entry out of range".to_owned()))
            .collect()
    }

    /// Drains `s`'s fate log into `out` (C-U6-3: the closure only copies into the caller's buffer, reserved by the
    /// caller, no I/O under the socket's lock), the tag checked first; returns the drain's hold of the lock, ns, read
    /// around the call. An entry or a tag that does not parse is an error (the entries that parsed are kept).
    pub fn drain_into(s: &ImpairSocket, out: &mut Vec<FateEntry>) -> Result<u64, String> {
        let (mut tag, mut bad) = (None, 0u64);
        let t0 = Instant::now();
        s.drain_fate_log(|b| match tag {
            None => tag = Some(b == FATE_TAG),
            Some(_) => match FateEntry::parse(b) {
                Some(e) => out.push(e),
                None => bad += 1,
            },
        });
        let hold = u64::try_from(t0.elapsed().as_nanos()).unwrap_or(u64::MAX);
        match (tag, bad) {
            (Some(true), 0) => Ok(hold),
            (Some(true), n) => Err(format!("{n} fate entries did not parse")),
            _ => Err("the fate log's tag is not npt-l2's v1".to_owned()),
        }
    }

    /// The lateness histogram's sample set: every finished fate but Oversize (impair.rs:680-684).
    pub fn in_e_set(&self) -> bool {
        matches!(self.kind, 0 | 8 | 9)
    }
}

/// One direction's samples from the fate log, each list sorted ascending.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Samples {
    /// L = out − due of every fate in the histogram's set ([`FateEntry::in_e_set`]).
    pub late: Vec<u64>,
    /// -3's delay, out − t_in (the profile's delay D plus L), of every Delivered fate only (R-F5).
    pub delay: Vec<u64>,
    /// Every fate of the direction, one per datagram: -3's count.
    pub fates: u64,
    /// The profile's loss draws.
    pub lost: u64,
    /// Delivered fates released before they arrived (out < t_in): never a delay, counted.
    pub early: u64,
}

/// The samples of direction `dir` (0 up, 1 down) among `log`'s entries.
pub fn samples<'a>(log: impl IntoIterator<Item = &'a FateEntry>, dir: u8) -> Samples {
    let mut s = Samples::default();
    for f in log.into_iter().filter(|f| f.dir == dir) {
        s.fates += 1;
        s.lost += u64::from(f.kind == FateEntry::LOST);
        if f.in_e_set() {
            s.late.push(f.out_ns.saturating_sub(f.due_ns));
        }
        if f.kind == FateEntry::DELIVERED {
            // Only after the filter: a dropped fate has out 0, and `0 − t_in` would underflow (R-F5).
            match f.out_ns.checked_sub(f.t_in_ns) {
                Some(d) => s.delay.push(d),
                None => s.early += 1,
            }
        }
    }
    s.late.sort_unstable();
    s.delay.sort_unstable();
    s
}

/// npt-judge's nearest rank (eval.rs:261-265; plan §6.1): ⌈q × n⌉ in integers, at least 1 and at most n (1-based).
pub fn nearest_rank(n: usize, num: u64, den: u64) -> usize {
    let n = n as u128;
    (n * u128::from(num))
        .div_ceil(u128::from(den))
        .clamp(1, n.max(1)) as usize
}

/// The nearest-rank quantile num/den of `sorted` (ascending); 0 when empty.
pub fn quantile(sorted: &[u64], num: u64, den: u64) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    sorted[nearest_rank(sorted.len(), num, den) - 1]
}

/// E[(L − L0 − 0.25 ms)+] over `late`, in whole ns rounded down, as `summarize` rounds (stats.rs:168); u128 sums.
pub fn excess(late: &[u64], l0: u64) -> u64 {
    if late.is_empty() {
        return 0;
    }
    let c = l0 + ALLOWANCE_NS;
    let total: u128 = late.iter().map(|&l| u128::from(l.saturating_sub(c))).sum();
    (total / late.len() as u128) as u64
}

/// The E forms over one direction's sorted samples (U3D-L0): the judged one, with L0 = 0 (the impairer subtracts
/// nothing: impair.rs:119-123), and M1's, with L0 = the exact nearest-rank p50.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Forms {
    pub p50: u64,
    pub at_p50: u64,
    pub at_zero: u64,
}

/// [`Forms`] of `late`, sorted ascending.
pub fn forms(late: &[u64]) -> Forms {
    debug_assert!(late.is_sorted());
    let p50 = quantile(late, 1, 2);
    Forms {
        p50,
        at_p50: excess(late, p50),
        at_zero: excess(late, 0),
    }
}

/// The probe's datagram, both directions (one source for both ends): its leg (0-5: P0, P1, P2, P3, P4, B1), its
/// index in its sender's stream, and the echo (up only): the down datagram's index and how long LG held it, ns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProbeDatagram {
    pub leg: u8,
    pub index: u64,
    pub echo: Option<u64>,
    pub hold_ns: u64,
}

impl ProbeDatagram {
    /// The one length, bytes.
    pub const LEN: usize = 40;
    const MAGIC: &[u8; 4] = b"NPT2";
    const NONE: u64 = u64::MAX;

    pub fn encode(&self) -> [u8; Self::LEN] {
        let mut b = [0u8; Self::LEN];
        b[..4].copy_from_slice(Self::MAGIC);
        b[4] = self.leg;
        b[8..16].copy_from_slice(&self.index.to_le_bytes());
        b[16..24].copy_from_slice(&self.echo.unwrap_or(Self::NONE).to_le_bytes());
        b[24..32].copy_from_slice(&self.hold_ns.to_le_bytes());
        b
    }

    /// A datagram from its bytes; `None` (the caller counts it) for another length or magic, a leg above 5, or another
    /// leg than `leg` when one is given.
    pub fn decode(b: &[u8], leg: Option<u8>) -> Option<ProbeDatagram> {
        if b.len() != Self::LEN
            || &b[..4] != Self::MAGIC
            || b[4] > 5
            || leg.is_some_and(|l| l != b[4])
        {
            return None;
        }
        let w = |at: usize| u64::from_le_bytes(b[at..at + 8].try_into().unwrap_or_default());
        Some(ProbeDatagram {
            leg: b[4],
            index: w(8),
            echo: Some(w(16)).filter(|&e| e != Self::NONE),
            hold_ns: w(24),
        })
    }
}

/// A dedicated pacer thread (M13: never tokio), 1 ns timer slack written by the thread itself and read back.
#[derive(Debug)]
pub struct Pacer {
    stop: Arc<AtomicBool>,
    sent: Arc<AtomicU64>,
    handle: Option<std::thread::JoinHandle<()>>,
    slack_ns: Option<u64>,
    policy: String,
}

impl Pacer {
    /// Starts the thread: `f(i)` for i = 0, 1, … at now + interval / 2 + i × interval (L1's schedule), catching up in a
    /// burst when it wakes late, until `count` calls or [`Pacer::stop`]. Returns once the thread has set its slack.
    pub fn start(
        interval_ns: u64,
        count: u64,
        mut f: impl FnMut(u64) + Send + 'static,
    ) -> io::Result<Pacer> {
        let (stop, sent) = (
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicU64::new(0)),
        );
        let (st, se) = (Arc::clone(&stop), Arc::clone(&sent));
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        let origin = Instant::now();
        let gap = interval_ns.max(1);
        let handle = std::thread::Builder::new()
            .name("npt-pacer".into())
            .spawn(move || {
                let tid = thread::own_tid().ok();
                let slack = tid.and_then(|t| thread::set_own_slack(t, 1).read_ns);
                let policy = tid
                    .and_then(thread::sched)
                    .map_or("unknown".to_owned(), |(p, r)| {
                        format!("{} {r}", thread::policy_name(p))
                    });
                let _ = tx.send((slack, policy));
                let at = |i: u64| gap / 2 + i.saturating_mul(gap);
                let mut i = 0;
                while i < count && !st.load(Relaxed) {
                    let now = origin.elapsed().as_nanos() as u64;
                    while i < count && at(i) <= now {
                        f(i);
                        i += 1;
                        se.store(i, Relaxed);
                    }
                    let now = origin.elapsed().as_nanos() as u64;
                    if i < count && at(i) > now {
                        std::thread::sleep(Duration::from_nanos(at(i) - now));
                    }
                }
            })?;
        let (slack_ns, policy) = rx
            .recv()
            .map_err(|_| io::Error::other("the pacer thread ended before it started"))?;
        Ok(Pacer {
            stop,
            sent,
            handle: Some(handle),
            slack_ns,
            policy,
        })
    }

    /// The timer slack the thread read back at its start (1 written; 0 under a real-time policy since ed4fb6d7ef68).
    pub fn slack_ns(&self) -> Option<u64> {
        self.slack_ns
    }

    /// The thread's scheduling policy and priority at its start (`SCHED_OTHER 0`).
    pub fn policy(&self) -> &str {
        &self.policy
    }

    /// The calls made so far.
    pub fn sent(&self) -> u64 {
        self.sent.load(Relaxed)
    }

    /// Whether the thread has made all its calls.
    pub fn finished(&self) -> bool {
        self.handle.as_ref().is_none_or(|h| h.is_finished())
    }

    /// Stops the thread and joins it; the calls made.
    pub fn stop(mut self) -> u64 {
        self.stop.store(true, Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
        self.sent()
    }
}

impl Drop for Pacer {
    fn drop(&mut self) {
        self.stop.store(true, Relaxed);
    }
}

/// RS's down stream (Block 3): a [`Pacer`] sending probe datagrams of `leg` to `peer` through the socket's own
/// `try_send` (the only socket call it makes), datagram i at the pacer's start + interval / 2 + i × interval, until
/// stopped; returned with the socket's clock read at the pacer's start, the leg's zero.
pub fn send_down(
    s: &Arc<ImpairSocket>,
    peer: SocketAddr,
    leg: u8,
    interval_ns: u64,
) -> io::Result<(Pacer, u64)> {
    let sock = Arc::clone(s);
    let zero = s.now_ns();
    let pacer = Pacer::start(interval_ns, u64::MAX, move |i| {
        let d = ProbeDatagram {
            leg,
            index: i,
            echo: None,
            hold_ns: 0,
        }
        .encode();
        let t = Transmit {
            destination: peer,
            ecn: None,
            contents: &d,
            segment_size: None,
            src_ip: None,
        };
        // ImpairSocket's `try_send` never fails (M11): a refusal is a fate.
        let _ = sock.try_send(&t);
    })?;
    Ok((pacer, zero))
}

/// LG's counts (`probe_peer_*`), atomics shared by the pacer and the reader.
#[derive(Debug, Default)]
struct PeerCounts {
    sent: AtomicU64,
    send_err: AtomicU64,
    received: AtomicU64,
    recv_err: AtomicU64,
    refused: AtomicU64,
    echoed: AtomicU64,
}

/// LG's end of the probe ([`probe_peer`]).
#[derive(Debug)]
pub struct ProbePeer {
    pacer: Option<Pacer>,
    reader: Option<std::thread::JoinHandle<()>>,
    stop: Arc<AtomicBool>,
    counts: Arc<PeerCounts>,
    local: SocketAddr,
}

/// The newest down datagram received and not yet echoed: (leg, index, arrival).
type Newest = Arc<Mutex<Option<(u8, u64, Instant)>>>;

/// LG's end (C2, C3; SERIES.md:501): one UDP socket, connected to `target` (so the kernel takes datagrams from RS
/// alone), for the whole probe; a [`Pacer`] sending `rate_pps` up datagrams a second from its start for `duration_ns`
/// without a pause, each carrying the newest down datagram received and not yet echoed, with its hold on LG's clock,
/// or none (a down datagram is echoed at most once); a reader thread taking the down datagrams. Send and receive
/// errors (ICMP unreachable, ECONNREFUSED while RS's socket is between legs) are counted, never fatal.
pub fn probe_peer(target: SocketAddr, rate_pps: u64, duration_ns: u64) -> io::Result<ProbePeer> {
    let local = if target.is_ipv4() {
        SocketAddr::from(([0, 0, 0, 0], 0))
    } else {
        SocketAddr::from(([0u16; 8], 0))
    };
    let sock = Arc::new(std::net::UdpSocket::bind(local)?);
    sock.connect(target)?;
    sock.set_read_timeout(Some(Duration::from_millis(50)))?;
    let local = sock.local_addr()?;
    let (stop, counts) = (
        Arc::new(AtomicBool::new(false)),
        Arc::new(PeerCounts::default()),
    );
    let newest: Newest = Arc::default();
    let leg = Arc::new(AtomicU8::new(0));
    let reader = {
        let (sock, stop, counts, newest, leg) = (
            Arc::clone(&sock),
            Arc::clone(&stop),
            Arc::clone(&counts),
            Arc::clone(&newest),
            Arc::clone(&leg),
        );
        std::thread::Builder::new()
            .name("npt-peer-reader".into())
            .spawn(move || {
                let mut buf = [0u8; 64];
                while !stop.load(Relaxed) {
                    match sock.recv(&mut buf) {
                        Ok(n) => match ProbeDatagram::decode(&buf[..n], None) {
                            Some(d) => {
                                counts.received.fetch_add(1, Relaxed);
                                leg.store(d.leg, Relaxed);
                                *newest.lock().unwrap_or_else(PoisonError::into_inner) =
                                    Some((d.leg, d.index, Instant::now()));
                            }
                            None => {
                                counts.refused.fetch_add(1, Relaxed);
                            }
                        },
                        Err(e)
                            if matches!(
                                e.kind(),
                                io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                            ) => {}
                        Err(_) => {
                            counts.recv_err.fetch_add(1, Relaxed);
                        }
                    }
                }
            })?
    };
    let interval = 1_000_000_000 / rate_pps.max(1);
    let count = duration_ns / interval;
    let pc = Arc::clone(&counts);
    let pacer = Pacer::start(interval, count, move |i| {
        let echo = newest.lock().unwrap_or_else(PoisonError::into_inner).take();
        let d = match echo {
            Some((l, e, at)) => {
                pc.echoed.fetch_add(1, Relaxed);
                ProbeDatagram {
                    leg: l,
                    index: i,
                    echo: Some(e),
                    hold_ns: at.elapsed().as_nanos() as u64,
                }
            }
            None => ProbeDatagram {
                leg: leg.load(Relaxed),
                index: i,
                echo: None,
                hold_ns: 0,
            },
        };
        match sock.send(&d.encode()) {
            Ok(_) => pc.sent.fetch_add(1, Relaxed),
            Err(_) => pc.send_err.fetch_add(1, Relaxed),
        };
    })?;
    Ok(ProbePeer {
        pacer: Some(pacer),
        reader: Some(reader),
        stop,
        counts,
        local,
    })
}

impl ProbePeer {
    /// The socket's local address (one for the whole probe).
    pub fn local_addr(&self) -> SocketAddr {
        self.local
    }

    /// Whether the pacer has sent for its whole duration.
    pub fn finished(&self) -> bool {
        self.pacer.as_ref().is_none_or(Pacer::finished)
    }

    /// The counts so far (`probe_peer_*`).
    pub fn counts(&self) -> Vec<(&'static str, u64)> {
        let c = &self.counts;
        [
            ("probe_peer_sent_count", &c.sent),
            ("probe_peer_send_err_count", &c.send_err),
            ("probe_peer_received_count", &c.received),
            ("probe_peer_recv_err_count", &c.recv_err),
            ("probe_peer_refused_count", &c.refused),
            ("probe_peer_echoed_count", &c.echoed),
        ]
        .map(|(n, v)| (n, v.load(Relaxed)))
        .to_vec()
    }

    /// Stops both threads (the pacer first) and joins them; the counts.
    pub fn stop(mut self) -> Vec<(&'static str, u64)> {
        if let Some(p) = self.pacer.take() {
            p.stop();
        }
        self.stop.store(true, Relaxed);
        if let Some(r) = self.reader.take() {
            let _ = r.join();
        }
        self.counts()
    }
}

impl Drop for ProbePeer {
    fn drop(&mut self) {
        self.stop.store(true, Relaxed);
    }
}

/// One leg's probe series and its summary fields ([`leg_series`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegSeries {
    /// Per second of the window (seconds since the leg's zero), the seven lists in [`LegSeries::NAMES`]' order.
    pub seconds: Vec<(u64, [Vec<i64>; 7])>,
    /// C-U6-5's fields and the leg's counts (`probe_*`), for the leg's `l2_summary` event.
    pub summary: Vec<(&'static str, u64)>,
}

impl LegSeries {
    /// SERIES.md's seven `second.s.probe_*` rows (:111-117).
    pub const NAMES: [&str; 7] = [
        "probe_up_lost",
        "probe_up_delay_ns",
        "probe_burst_up_lost",
        "probe_down_lost",
        "probe_down_delay_ns",
        "probe_burst_down_lost",
        "probe_rtt_ns",
    ];
    /// An echo record: a datagram from the peer that echoes nothing (or did not decode for this leg).
    pub const NO_ECHO: u64 = u64::MAX - 1;
    /// An echo record: a datagram from another source (dropped and counted).
    pub const OTHER: u64 = u64::MAX;
}

/// C-U6-5's names per direction, in [`e_fields`]' order.
const E_NAMES: [[&str; 8]; 2] = [
    [
        "probe_up_count",
        "probe_up_excess_ns",
        "probe_up_excess_p50_ns",
        "probe_up_over_count",
        "probe_up_p50_ns",
        "probe_up_p99_ns",
        "probe_up_p999_ns",
        "probe_up_max_ns",
    ],
    [
        "probe_down_count",
        "probe_down_excess_ns",
        "probe_down_excess_p50_ns",
        "probe_down_over_count",
        "probe_down_p50_ns",
        "probe_down_p99_ns",
        "probe_down_p999_ns",
        "probe_down_max_ns",
    ],
];

/// C-U6-5's eight values over sorted lateness samples: n, E with L0 = 0 (judged), E with L0 = the nearest-rank p50,
/// the count with L above 0.25 ms, p50, p99, p99.9 and max.
fn e_fields(late: &[u64]) -> [u64; 8] {
    let f = forms(late);
    let over = late.iter().filter(|&&l| l > ALLOWANCE_NS).count() as u64;
    [
        late.len() as u64,
        f.at_zero,
        f.at_p50,
        over,
        f.p50,
        quantile(late, 99, 100),
        quantile(late, 999, 1_000),
        late.last().copied().unwrap_or(0),
    ]
}

/// One leg's series from its fate log (C-U6-5; SERIES.md:111-117, :499-500) on L1's count-based records
/// (lab/src/probe.rs:124-132; advisor-u6c-r1.md FP-1): each direction's records are its first `packets` fates outside
/// `bursts` whose t_in (the socket's clock) is at or after `start_ns`, in t_in order (ties in log order; every fate
/// sharing the last kept fate's t_in is kept too), up for the peer only; the in-burst fates among them go to
/// `probe_burst_*`. Fewer than `packets` there: the records are what there is. For each record, per direction, a loss
/// flag (0 Delivered, else 1) and, for a Delivered fate, its delay out − t_in; and each round trip: the release of the
/// up datagram carrying the echo minus the echoed down datagram's t_in minus LG's hold, filed under the echoing
/// datagram's second. The rows run from `start_ns`'s second to the last kept fate's. `echoes` holds one record per
/// datagram the poller was handed, in release order ((down index or [`LegSeries::NO_ECHO`] or [`LegSeries::OTHER`],
/// hold)), so the k-th Delivered up fate is the k-th record; the peer is the ordinal of the first Delivered up fate that
/// is not [`LegSeries::OTHER`]; down fates are found by their flow index, which is the down datagram's index. A repeated
/// echo is counted and only its first sample kept. C-U6-5's fields come from the records' E set, per direction;
/// `probe_up_window_ns` and `probe_down_window_ns` are each direction's last kept t_in minus `start_ns` plus one
/// `interval_ns` (0 with no record).
pub fn leg_series(
    fates: &[FateEntry],
    echoes: &[(u64, u64)],
    zero_ns: u64,
    start_ns: u64,
    packets: u64,
    interval_ns: u64,
    bursts: &[(u64, u64)],
) -> LegSeries {
    let in_burst = |t: u64| bursts.iter().any(|&(b, len)| t >= b && t - b < len);
    let sec = |t: u64| t.saturating_sub(zero_ns) / 1_000_000_000;
    // The peer: the first Delivered up fate whose poller record is not another source's.
    let mut peer = None;
    let ups = fates
        .iter()
        .filter(|f| f.dir == 0 && f.kind == FateEntry::DELIVERED);
    for (f, r) in ups.zip(echoes) {
        if r.0 != LegSeries::OTHER {
            peer = Some(f.ordinal);
            break;
        }
    }
    // The records: per direction, in t_in order from the start, until `packets` lie outside the bursts.
    let mut kept = vec![false; fates.len()];
    let mut last = [None::<u64>; 2];
    for d in 0..2u8 {
        let mut order: Vec<usize> = (0..fates.len())
            .filter(|&i| {
                let f = &fates[i];
                f.dir == d && f.t_in_ns >= start_ns && (d == 1 || Some(f.ordinal) == peer)
            })
            .collect();
        order.sort_by_key(|&i| fates[i].t_in_ns);
        let mut outside = 0u64;
        for i in order {
            let t = fates[i].t_in_ns;
            if outside >= packets && last[usize::from(d)] != Some(t) {
                break;
            }
            kept[i] = true;
            last[usize::from(d)] = Some(t);
            outside += u64::from(!in_burst(t));
        }
    }
    let first = sec(start_ns);
    let mut seconds: Vec<(u64, [Vec<i64>; 7])> = match last.iter().flatten().max() {
        Some(&t) => (first..=sec(t)).map(|s| (s, Default::default())).collect(),
        None => Vec::new(),
    };
    let mut down_t_in = Vec::new();
    // NC-U6C-3: QueueDrop and PoolDrop fates take no index (impair.rs:489-512), which shifts the RTT join: counted.
    let unindexed = fates
        .iter()
        .filter(|f| f.dir == 1 && f.index == u64::MAX)
        .count() as u64;
    for f in fates.iter().filter(|f| f.dir == 1 && f.index != u64::MAX) {
        let i = usize::try_from(f.index).unwrap_or(usize::MAX);
        if i < fates.len() {
            if down_t_in.len() <= i {
                down_t_in.resize(i + 1, u64::MAX);
            }
            down_t_in[i] = f.t_in_ns;
        }
    }
    let mut echoed = vec![false; down_t_in.len()];
    let mut k = 0usize;
    let (mut repeats, mut unmatched, mut rtts, mut early_rtt) = (0u64, 0u64, 0u64, 0u64);
    let mut window_count = [0u64; 2];
    let mut set: [Vec<FateEntry>; 2] = Default::default();
    for (f, &keep) in fates.iter().zip(&kept) {
        let rec = if f.dir == 0 && f.kind == FateEntry::DELIVERED {
            let r = echoes.get(k).copied();
            k += 1;
            if r.is_none() {
                unmatched += 1;
            }
            r
        } else {
            None
        };
        if !keep {
            continue;
        }
        let d = usize::from(f.dir);
        window_count[d] += 1;
        set[d].push(*f);
        let row = &mut seconds[(sec(f.t_in_ns) - first) as usize].1;
        let delivered = f.kind == FateEntry::DELIVERED;
        let list = if in_burst(f.t_in_ns) { 2 } else { 0 } + 3 * d;
        row[list].push(i64::from(!delivered));
        if delivered {
            row[1 + 3 * d].push(int(f.out_ns.saturating_sub(f.t_in_ns)));
        }
        if let Some((e, hold)) = rec.filter(|r| r.0 < LegSeries::NO_ECHO) {
            let i = usize::try_from(e).unwrap_or(usize::MAX);
            match down_t_in.get(i).copied().filter(|&t| t != u64::MAX) {
                Some(_) if echoed[i] => repeats += 1,
                Some(t) => {
                    echoed[i] = true;
                    match f.out_ns.checked_sub(t).and_then(|x| x.checked_sub(hold)) {
                        Some(rtt) => {
                            row[6].push(int(rtt));
                            rtts += 1;
                        }
                        None => early_rtt += 1,
                    }
                }
                None => unmatched += 1,
            }
        }
    }
    let mut summary = Vec::with_capacity(32);
    for (d, names) in E_NAMES.iter().enumerate() {
        let late = samples(&set[d], d as u8).late;
        summary.extend(names.iter().copied().zip(e_fields(&late)));
    }
    let span = |d: usize| last[d].map_or(0, |t| t - start_ns + interval_ns);
    summary.extend([
        ("probe_up_window_ns", span(0)),
        ("probe_down_window_ns", span(1)),
        ("probe_up_window_count", window_count[0]),
        ("probe_down_window_count", window_count[1]),
        ("probe_rtt_count", rtts),
        ("probe_down_unindexed_count", unindexed),
        ("probe_echo_repeat_count", repeats),
        ("probe_echo_unmatched_count", unmatched + early_rtt),
        (
            "probe_echo_records_left_count",
            echoes.len().saturating_sub(k) as u64,
        ),
    ]);
    LegSeries { seconds, summary }
}

fn int(x: u64) -> i64 {
    i64::try_from(x).unwrap_or(i64::MAX)
}

#[cfg(test)]
#[path = "probe_tests.rs"]
mod tests;
