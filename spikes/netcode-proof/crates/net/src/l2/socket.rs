//! `ImpairSocket`: L2's impairment under quinn, a `quinn::AsyncUdpSocket` over a real UDP socket (E15; R5-TEST-OQ2;
//! u3-scout-summary.md's confirmed shape; the advisor's M1-M5, M9-M15, M19).
//!
//! It behaves like a network, never like a host:
//! - **Never WouldBlock** (M11): `try_send` takes every datagram in and returns Ok (a drop is a fate, never an error),
//!   and its poller's `poll_writable` is always Ready, so quinn never holds a transmit back.
//! - **One datagram per Transmit** (M11): `max_transmit_segments` and `max_receive_segments` are 1, so quinn's GSO and
//!   GRO are off; a Transmit that carries segments anyway is split into its datagrams. `may_fragment` is the real
//!   socket's (quinn-udp sets DF on Linux), so quinn's MTU discovery runs as on the plain path.
//! - **Two threads** of its own: the reader drains the kernel socket as soon as epoll says readable (a current-thread
//!   tokio runtime on its own OS thread, never quinn's workers) and stamps each datagram there (quinn-udp 0.5.16 has no
//!   kernel receive stamp, M1); the release thread (M2, M5, M14) waits on a Condvar until the earliest due time in the
//!   one queue, sends due outbound datagrams with quinn-udp's `try_send` (never `send`, which turns every error but
//!   WouldBlock into Ok), logs each error kind as a fate, and wakes quinn when an inbound datagram is due. It sets its
//!   own timer slack to 1 ns and reads it back, and reads it again whenever the header is built (only the thread
//!   itself may read its own `/proc/<tid>/timerslack_ns` without CAP_SYS_NICE: fs/proc/base.c checks the capability
//!   when the task is not current), so the header pairs the policy and the slack as they are then; a real-time
//!   policy is applied from outside and read from /proc.
//! - **poll_recv** hands quinn only due datagrams, one per buffer, and stores quinn's waker otherwise: never Ok(0),
//!   never an error for an empty queue (M11). Inbound lateness is measured here, where quinn receives the datagram
//!   (M1); outbound at `try_send`'s return on the real socket.
//! - **No allocation per datagram** (M11): every queued datagram is copied into a fixed pool of 1,500 B buffers made
//!   at bind; the queue, the ready list and the release batch have fixed capacities; an empty pool is a counted drop.
//! - **The pass-through** (M12): with every impairment off the same socket, threads and slack carry the clean runs.
//!
//! The control plane (E23, U6) keeps the `Arc<ImpairSocket>` and sets per-peer settings through `set_peer` and
//! `set_default`; npt-server's Join deadline reads `first_seen` (U3-Q8). Carried to U6: it pins every joined client
//! with `set_peer`, so datagrams from new (possibly spoofed) addresses can never evict a real client's slot.

use super::config::{L2Config, queue_ns};
use super::impair::{DOWN, Entry, Impairer, Intake, Settings, UP};
use super::names;
use super::stats::{Counters, Kind};
use super::thread;
use quinn::udp::{EcnCodepoint, RecvMeta, Transmit, UdpSocketState};
use std::collections::VecDeque;
use std::fmt;
use std::io::{self, IoSliceMut};
use std::net::{IpAddr, SocketAddr};
use std::pin::Pin;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

const BUF: usize = L2Config::BUFFER_BYTES;
/// Outbound datagrams the release thread sends per lock.
const BATCH: usize = 64;
/// Receive buffers per drain of the kernel socket.
const RECV_BATCH: usize = 8;

/// A queued datagram: its pool buffer and what quinn-udp needs to send or deliver it.
struct Packet {
    buf: Box<[u8; BUF]>,
    len: usize,
    /// The peer's address as the kernel or quinn gave it (never the normalised key).
    addr: SocketAddr,
    ecn: Option<EcnCodepoint>,
    /// Outbound: quinn's source address; inbound: the destination address the kernel reported.
    ip: Option<IpAddr>,
}

struct State {
    im: Impairer<Packet>,
    // Boxed on purpose: a buffer moves between the pool, the queue's entries and the ready list as a pointer, never
    // as 1,500 copied bytes (the heap's sifts move entries).
    #[allow(clippy::vec_box)]
    pool: Vec<Box<[u8; BUF]>>,
    /// Inbound datagrams due and waiting for quinn's poll_recv.
    ready: VecDeque<Entry<Packet>>,
    waker: Option<Waker>,
    /// The release thread sleeps until this (u64::MAX: no deadline; 0: awake), so a new earlier datagram wakes it.
    wait_until: u64,
    stop: bool,
    /// The last per-second window's start: (wall ns, the release thread's CPU ns).
    second_mark: (u64, u64),
    /// Requests for the release thread to read its own slack (a count; it answers the latest), and a value for it to
    /// write first (tests stand in for an outside change with it).
    slack_ask: u64,
    slack_write: Option<u64>,
}

/// The threads' facts: the release thread's tid and its slack (as set at its start, and its latest answer to a
/// header-time request), and the reader's tid.
#[derive(Debug, Default, Clone)]
struct ReleaseInfo {
    tid: Option<u32>,
    slack: thread::Slack,
    /// (the request answered, the slack the thread read then).
    slack_now: (u64, Option<u64>),
    reader_tid: Option<u32>,
}

/// How long a header waits for the release thread's own slack reading before reporting it unreadable.
const SLACK_ANSWER: Duration = Duration::from_secs(1);

struct Shared {
    epoch: Instant,
    state: Mutex<State>,
    cv: Condvar,
    udp: UdpSocketState,
    /// A second handle on the bound socket for the release thread's sends (the reader's is inside its runtime).
    send_sock: std::net::UdpSocket,
    reader_stop: tokio::sync::Notify,
    /// Locked after `state` when both are held (the release thread's answer), never before it.
    release: Mutex<ReleaseInfo>,
    /// Signalled when the release thread answers a slack request.
    release_cv: Condvar,
    cfg: L2Config,
}

impl Shared {
    fn now_ns(&self) -> u64 {
        self.epoch.elapsed().as_nanos() as u64
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Takes one datagram in under the lock; true if it was queued earlier than the release thread's deadline.
    fn intake(
        st: &mut State,
        dir: usize,
        addr: SocketAddr,
        t: u64,
        data: &[u8],
        ecn: Option<EcnCodepoint>,
        ip: Option<IpAddr>,
    ) -> bool {
        let State { im, pool, .. } = st;
        let r = im.intake(dir, addr, t, data.len(), || {
            // Called only after the size check, so the datagram fits the buffer.
            let mut buf = pool.pop()?;
            buf[..data.len()].copy_from_slice(data);
            Some(Packet {
                buf,
                len: data.len(),
                addr,
                ecn,
                ip,
            })
        });
        match r {
            Intake::Queued { due_ns } => due_ns < st.wait_until,
            Intake::Dropped { item, .. } => {
                if let Some(p) = item {
                    st.pool.push(p.buf);
                }
                false
            }
        }
    }
}

/// L2's impairment layer under quinn (plan E15; §4.7's one socket type).
pub struct ImpairSocket {
    shared: Arc<Shared>,
    local: SocketAddr,
    may_fragment: bool,
}

impl fmt::Debug for ImpairSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImpairSocket")
            .field("local", &self.local)
            .field("pass_through", &self.shared.cfg.pass_through)
            .finish_non_exhaustive()
    }
}

impl ImpairSocket {
    /// Binds a UDP socket at `addr` (quinn-udp's settings: non-blocking, DF, GRO where the kernel has it), makes the
    /// buffer pool, and starts the reader and release threads. Call it inside a tokio runtime or outside one: the
    /// reader runs its own.
    pub fn bind(addr: SocketAddr, cfg: L2Config) -> io::Result<Arc<ImpairSocket>> {
        if cfg.max_datagram_bytes > BUF || cfg.pool_buffers == 0 || cfg.peer_capacity == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "L2Config: max_datagram_bytes above 1,500 B, or no pool buffers, or no peer slots",
            ));
        }
        let sock = std::net::UdpSocket::bind(addr)?;
        let udp = UdpSocketState::new((&sock).into())?;
        let local = sock.local_addr()?;
        let send_sock = sock.try_clone()?;
        let may_fragment = udp.may_fragment();
        let state = State {
            im: Impairer::new(&cfg),
            pool: (0..cfg.pool_buffers)
                .map(|_| Box::new([0u8; BUF]))
                .collect(),
            ready: VecDeque::with_capacity(cfg.pool_buffers),
            waker: None,
            wait_until: 0,
            stop: false,
            second_mark: (0, 0),
            slack_ask: 0,
            slack_write: None,
        };
        let shared = Arc::new(Shared {
            epoch: Instant::now(),
            state: Mutex::new(state),
            cv: Condvar::new(),
            udp,
            send_sock,
            reader_stop: tokio::sync::Notify::new(),
            release: Mutex::new(ReleaseInfo::default()),
            release_cv: Condvar::new(),
            cfg,
        });
        let (tx, rx) = std::sync::mpsc::sync_channel(2);
        let (sh, tx1) = (Arc::clone(&shared), tx.clone());
        std::thread::Builder::new()
            .name("npt-l2-reader".into())
            .spawn(move || reader(sh, sock, tx1))?;
        let sh = Arc::clone(&shared);
        std::thread::Builder::new()
            .name("npt-l2-release".into())
            .spawn(move || release(sh, tx))?;
        for _ in 0..2 {
            let started = rx
                .recv()
                .map_err(|_| io::Error::other("an L2 thread ended before it started"))
                .and_then(|r| r);
            if let Err(e) = started {
                // Stop whichever thread did start: nothing else holds the shared state.
                shared.lock().stop = true;
                shared.cv.notify_all();
                shared.reader_stop.notify_one();
                return Err(e);
            }
        }
        Ok(Arc::new(ImpairSocket {
            shared,
            local,
            may_fragment,
        }))
    }

    /// Sets one peer's settings (the per-peer part of `cfg`: profile with its reorder step and bursts, base RTT,
    /// pass-through, shaper, UDP block) and pins it in the table. False if every slot is pinned already. U6's control
    /// plane calls it for every joined client (carried), so no unpinned address can evict one.
    pub fn set_peer(&self, peer: SocketAddr, cfg: &L2Config) -> bool {
        let sh = &self.shared;
        let t = sh.now_ns();
        sh.lock().im.set_peer(peer, Settings::of(cfg), t)
    }

    /// Sets the per-peer part of `cfg` for every peer not set by [`ImpairSocket::set_peer`], now and later.
    pub fn set_default(&self, cfg: &L2Config) {
        self.shared.lock().im.set_default(Settings::of(cfg));
    }

    /// The socket's clock: ns since bind, the clock bursts and the fate log are on.
    pub fn now_ns(&self) -> u64 {
        self.shared.now_ns()
    }

    /// When the first raw packet from `peer` was drained from the kernel socket (M10; U3-Q8: npt-server's Join
    /// deadline starts here, never at the end of a delay). None if never seen, or its slot was evicted.
    pub fn first_seen(&self, peer: SocketAddr) -> Option<Instant> {
        let ns = self.shared.lock().im.first_seen(peer)?;
        Some(self.shared.epoch + Duration::from_nanos(ns))
    }

    /// The release thread's id, for `chrt -p` from outside (the recorded real-time switch, M14).
    pub fn release_thread_id(&self) -> Option<u32> {
        self.shared
            .release
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .tid
    }

    /// The reader thread's id (the drop test watches it end).
    #[cfg(test)]
    pub(crate) fn reader_thread_id(&self) -> Option<u32> {
        self.release_info().reader_tid
    }

    /// Asks the release thread to read its own timer slack now (writing `write` first, if given) and waits up to
    /// [`SLACK_ANSWER`] for the value; None if unreadable or unanswered. Only the thread itself may read its slack
    /// without CAP_SYS_NICE, so the request goes to it.
    pub(crate) fn ask_slack(&self, write: Option<u64>) -> Option<u64> {
        let sh = &self.shared;
        let ask = {
            let mut st = sh.lock();
            st.slack_ask += 1;
            if write.is_some() {
                st.slack_write = write;
            }
            st.slack_ask
        };
        sh.cv.notify_all();
        let end = Instant::now() + SLACK_ANSWER;
        let mut info = sh.release.lock().unwrap_or_else(PoisonError::into_inner);
        while info.slack_now.0 < ask {
            let left = end.checked_duration_since(Instant::now())?;
            info = sh
                .release_cv
                .wait_timeout(info, left)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
        info.slack_now.1
    }

    /// The header's numbers (`header.params.<name>`; SERIES.md): the timer slack as the release thread reads it now
    /// and as it read it back at its start, the switches and bounds, the default peer's injected delay and jitter, the
    /// shaper.
    pub fn header_numbers(&self) -> Vec<(&'static str, u64)> {
        let c = &self.shared.cfg;
        let s = Settings::of(c);
        let slack_now = self.ask_slack(None);
        let info = self.release_info();
        let (rate, queue) = s.shaper.unwrap_or((0, 0));
        vec![
            ("l2_timer_slack_ns", slack_now.unwrap_or(u64::MAX)),
            (
                "l2_timer_slack_start_ns",
                info.slack.read_ns.unwrap_or(u64::MAX),
            ),
            (
                "l2_timer_slack_set",
                u64::from(info.slack.write_error.is_none()),
            ),
            ("l2_migration", u64::from(super::MIGRATION)),
            ("l2_gso", 0),
            ("l2_max_datagram_bytes", c.max_datagram_bytes as u64),
            ("l2_pool_buffers_count", c.pool_buffers as u64),
            ("l2_peer_queue_bytes", c.peer_queue_bytes),
            ("l2_total_queue_bytes", c.total_queue_bytes),
            ("l2_peer_capacity_count", c.peer_capacity as u64),
            ("l2_fate_log_capacity_count", c.fate_log_capacity as u64),
            ("l2_one_way_ns", s.one_way_ns),
            ("l2_half_jitter_ns", s.half_jitter_ns),
            ("base_rtt_ns", c.base_rtt_ns),
            ("reorder_step_permille", u64::from(s.reorder_permille)),
            ("l2_shaper_rate_bits", rate),
            ("l2_shaper_queue_bytes", queue),
            (
                "l2_shaper_queue_ns",
                if rate == 0 { 0 } else { queue_ns(queue, rate) },
            ),
            ("l2_udp_block", u64::from(s.udp_block)),
        ]
    }

    /// The header's strings: the mode (M12) and the release thread's scheduling policy and priority (M14), read now.
    pub fn header_texts(&self) -> Vec<(&'static str, String)> {
        let info = self.release_info();
        let mode = if self.shared.cfg.pass_through {
            "pass-through"
        } else {
            "impaired"
        };
        let sched = match info.tid.and_then(thread::sched) {
            Some((policy, prio)) => format!("{} {prio}", thread::policy_name(policy)),
            None => "unknown".to_owned(),
        };
        let mut v = vec![("l2_mode", mode.to_owned()), ("l2_sched_policy", sched)];
        if let Some(e) = info.slack.write_error {
            v.push(("l2_timer_slack_error", e));
        }
        v
    }

    /// The per-second window since the last call (`second.v.<name>`), which it closes: fates, lateness per direction,
    /// the table's counters, the pool and queue levels, and the release thread's CPU in permille of one core.
    pub fn take_second(&self) -> Vec<(&'static str, u64)> {
        let tid = self.release_info().tid;
        let cpu = tid.and_then(thread::cpu_ns).unwrap_or(0);
        let mut st = self.shared.lock();
        let now = self.shared.now_ns();
        let late = [
            st.im.late_second[UP].summary(),
            st.im.late_second[DOWN].summary(),
        ];
        let c: Counters = st.im.take_second();
        let (wall0, cpu0) = st.second_mark;
        st.second_mark = (now, cpu);
        let permille = (cpu.saturating_sub(cpu0) as u128 * 1000
            / u128::from(now.saturating_sub(wall0).max(1))) as u64;
        let mut out = Vec::with_capacity(64);
        names::window(&c, late, &mut out);
        Self::levels(&st, &mut out);
        out.push((names::CPU_PERMILLE, permille));
        out
    }

    /// The whole run so far (the `l2_summary` event's fields): the same names over the run, the release thread's CPU
    /// in ns, and the impairment layer's measured reorder shares (RFC 4737 per 1,000 delivered; M3).
    pub fn run_summary(&self) -> Vec<(&'static str, u64)> {
        let cpu = self
            .release_info()
            .tid
            .and_then(thread::cpu_ns)
            .unwrap_or(0);
        let st = self.shared.lock();
        let late = [st.im.late_run[UP].summary(), st.im.late_run[DOWN].summary()];
        let mut out = Vec::with_capacity(64);
        names::window(&st.im.run, late, &mut out);
        Self::levels(&st, &mut out);
        out.push((names::CPU_NS, cpu));
        let share = |d: usize| {
            let delivered = st.im.run.fates[d][Kind::Delivered as usize];
            st.im.run.reordered[d] * 1000 / delivered.max(1)
        };
        let (up, down) = (share(UP), share(DOWN));
        out.push(("reorder_up_permille", up));
        out.push(("reorder_down_permille", down));
        out.push(("reorder_permille", up.min(down)));
        out
    }

    fn levels(st: &State, out: &mut Vec<(&'static str, u64)>) {
        let q = st.im.queued_bytes();
        out.push((names::PEERS, st.im.peers() as u64));
        out.push((names::POOL_FREE, st.pool.len() as u64));
        out.push((names::QUEUED[UP], q[UP]));
        out.push((names::QUEUED[DOWN], q[DOWN]));
    }

    /// Feeds the fate log's canonical bytes to `f` (its tag, then 56 bytes per entry in the order of each datagram's
    /// final fate) and empties it.
    pub fn drain_fate_log(&self, f: impl FnMut(&[u8])) {
        self.shared.lock().im.fates.drain(f);
    }

    fn release_info(&self) -> ReleaseInfo {
        self.shared
            .release
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

impl Drop for ImpairSocket {
    fn drop(&mut self) {
        self.shared.lock().stop = true;
        self.shared.cv.notify_all();
        self.shared.reader_stop.notify_one();
    }
}

/// The poller quinn asks before each send: always writable, so quinn never waits on the host (M11).
#[derive(Debug)]
struct AlwaysWritable;

impl quinn::UdpPoller for AlwaysWritable {
    fn poll_writable(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl quinn::AsyncUdpSocket for ImpairSocket {
    fn create_io_poller(self: Arc<Self>) -> Pin<Box<dyn quinn::UdpPoller>> {
        Box::pin(AlwaysWritable)
    }

    fn try_send(&self, t: &Transmit<'_>) -> io::Result<()> {
        let sh = &self.shared;
        let now = sh.now_ns();
        let mut st = sh.lock();
        let mut wake = false;
        match t.segment_size {
            Some(seg) if seg > 0 && seg < t.contents.len() => {
                for chunk in t.contents.chunks(seg) {
                    wake |=
                        Shared::intake(&mut st, DOWN, t.destination, now, chunk, t.ecn, t.src_ip);
                }
            }
            _ => {
                wake = Shared::intake(
                    &mut st,
                    DOWN,
                    t.destination,
                    now,
                    t.contents,
                    t.ecn,
                    t.src_ip,
                )
            }
        }
        drop(st);
        if wake {
            sh.cv.notify_one();
        }
        Ok(())
    }

    fn poll_recv(
        &self,
        cx: &mut Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
        meta: &mut [RecvMeta],
    ) -> Poll<io::Result<usize>> {
        let sh = &self.shared;
        let mut guard = sh.lock();
        let st = &mut *guard;
        let now = sh.now_ns();
        let max = bufs.len().min(meta.len());
        let mut n = 0;
        while n < max {
            let Some(mut e) = st.ready.pop_front() else {
                break;
            };
            let Some(p) = e.take_item() else {
                continue;
            };
            if p.len > bufs[n].len() {
                // quinn's buffer is smaller than the datagram: a drop, never a truncated datagram.
                st.im.finish(&e, Kind::Oversize, now, 0);
                st.pool.push(p.buf);
                continue;
            }
            bufs[n][..p.len].copy_from_slice(&p.buf[..p.len]);
            meta[n] = RecvMeta {
                addr: p.addr,
                len: p.len,
                stride: p.len,
                ecn: p.ecn,
                dst_ip: p.ip,
            };
            st.im.finish(&e, Kind::Delivered, now, 0);
            st.pool.push(p.buf);
            n += 1;
        }
        if n > 0 {
            return Poll::Ready(Ok(n));
        }
        match &st.waker {
            Some(w) if w.will_wake(cx.waker()) => {}
            _ => st.waker = Some(cx.waker().clone()),
        }
        Poll::Pending
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.local)
    }

    fn max_transmit_segments(&self) -> usize {
        1
    }

    fn max_receive_segments(&self) -> usize {
        1
    }

    fn may_fragment(&self) -> bool {
        self.may_fragment
    }
}

/// The reader: drains the kernel socket when epoll says readable, stamps the drain, splits GRO buffers by stride and
/// takes each datagram in (one lock per drain).
fn reader(sh: Arc<Shared>, sock: std::net::UdpSocket, started: SyncSender<io::Result<()>>) {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            let _ = started.send(Err(e));
            return;
        }
    };
    rt.block_on(async move {
        let sock = match tokio::net::UdpSocket::from_std(sock) {
            Ok(s) => s,
            Err(e) => {
                let _ = started.send(Err(e));
                return;
            }
        };
        sh.release
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .reader_tid = thread::own_tid().ok();
        let _ = started.send(Ok(()));
        // Large enough for any UDP datagram and for a GRO batch, so nothing is ever truncated.
        let size = (1_472 * sh.udp.gro_segments().max(1)).max(65_536);
        let mut bufs: [Vec<u8>; RECV_BATCH] = std::array::from_fn(|_| vec![0u8; size]);
        let mut metas = [RecvMeta::default(); RECV_BATCH];
        loop {
            tokio::select! {
                biased;
                () = sh.reader_stop.notified() => return,
                r = sock.readable() => {
                    if r.is_err() {
                        return;
                    }
                }
            }
            loop {
                let res = {
                    let mut iovs = bufs.each_mut().map(|b| IoSliceMut::new(b.as_mut_slice()));
                    sock.try_io(tokio::io::Interest::READABLE, || {
                        sh.udp.recv((&sock).into(), &mut iovs, &mut metas)
                    })
                };
                match res {
                    Ok(n) => {
                        let t = sh.now_ns();
                        let mut st = sh.lock();
                        let mut wake = false;
                        for (m, b) in metas[..n].iter().zip(&bufs) {
                            for chunk in b[..m.len].chunks(m.stride.max(1)) {
                                wake |=
                                    Shared::intake(&mut st, UP, m.addr, t, chunk, m.ecn, m.dst_ip);
                            }
                        }
                        drop(st);
                        if wake {
                            sh.cv.notify_one();
                        }
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                    Err(_) => {
                        // An ICMP error reported on the socket, or the like: counted, never fatal.
                        let mut st = sh.lock();
                        st.im.run.recv_err += 1;
                        st.im.second.recv_err += 1;
                        break;
                    }
                }
            }
        }
    });
}

/// A send's fate from quinn-udp's raw result (M5).
fn send_fate(r: &io::Result<()>) -> (Kind, i32) {
    match r {
        Ok(()) => (Kind::Delivered, 0),
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
            (Kind::WouldBlock, e.raw_os_error().unwrap_or(11))
        }
        Err(e) => (Kind::SendError, e.raw_os_error().unwrap_or(-1)),
    }
}

/// The release clock: one dedicated OS thread with 1 ns timer slack, waiting on a Condvar until the earliest due time
/// in the one queue (M2, M13, M14).
fn release(sh: Arc<Shared>, started: SyncSender<io::Result<()>>) {
    let tid = thread::own_tid().ok();
    let slack = tid.map_or_else(thread::Slack::default, |t| {
        thread::set_own_slack(t, sh.cfg.timer_slack_ns)
    });
    {
        let mut info = sh.release.lock().unwrap_or_else(PoisonError::into_inner);
        info.tid = tid;
        info.slack = slack;
    }
    let _ = started.send(Ok(()));
    let mut batch: Vec<Entry<Packet>> = Vec::with_capacity(BATCH);
    let mut results: Vec<(Kind, i32, u64)> = Vec::with_capacity(BATCH);
    let mut answered = 0u64;
    let mut st = sh.lock();
    loop {
        if st.stop {
            return;
        }
        if st.slack_ask != answered {
            // A header is being built: read the slack now, as the thread itself (a /proc read, at header time only).
            answered = st.slack_ask;
            let write = st.slack_write.take().unwrap_or(0);
            let read = tid.and_then(|t| thread::set_own_slack(t, write).read_ns);
            sh.release
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .slack_now = (answered, read);
            sh.release_cv.notify_all();
        }
        let now = sh.now_ns();
        let mut wake = None;
        while batch.len() < BATCH {
            match st.im.pop_due(now) {
                Some(e) if e.dir == UP => {
                    st.ready.push_back(e);
                    if wake.is_none() {
                        wake = st.waker.take();
                    }
                }
                Some(e) => batch.push(e),
                None => break,
            }
        }
        if batch.is_empty() && wake.is_none() {
            match st.im.next_due() {
                Some(d) if d > now => {
                    st.wait_until = d;
                    st = sh
                        .cv
                        .wait_timeout(st, Duration::from_nanos(d - now))
                        .unwrap_or_else(PoisonError::into_inner)
                        .0;
                }
                Some(_) => {}
                None => {
                    st.wait_until = u64::MAX;
                    st = sh.cv.wait(st).unwrap_or_else(PoisonError::into_inner);
                }
            }
            st.wait_until = 0;
            continue;
        }
        drop(st);
        if let Some(w) = wake {
            w.wake();
        }
        for e in &batch {
            if let Some(p) = e.item() {
                let t = Transmit {
                    destination: p.addr,
                    ecn: p.ecn,
                    contents: &p.buf[..p.len],
                    segment_size: None,
                    src_ip: p.ip,
                };
                let r = sh.udp.try_send((&sh.send_sock).into(), &t);
                let (kind, errno) = send_fate(&r);
                results.push((kind, errno, sh.now_ns()));
            }
        }
        st = sh.lock();
        for (mut e, (kind, errno, out)) in batch.drain(..).zip(results.drain(..)) {
            st.im.finish(&e, kind, out, errno);
            if let Some(p) = e.take_item() {
                st.pool.push(p.buf);
            }
        }
    }
}

#[cfg(test)]
#[path = "socket_tests.rs"]
mod tests;
