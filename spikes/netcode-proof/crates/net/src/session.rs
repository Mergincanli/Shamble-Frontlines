//! What both transports share (u6b-brief.md Blocks 1-3): the listener's settings, an accepted session, a session's
//! bounded inbox, its closed flag and a server's counts.
//!
//! **Stamps (Block 1):** both servers take the caller's clock as `fn() -> u64` (npt-server passes its room clock, the
//! tests an injected one) and the deadline as a value. Each message's `Recv::arrival_ns` is read by the session's
//! reader task when the message has arrived, never at `poll_recv`; each session carries its accept stamp (the first
//! QUIC packet or the TCP accept) to the room, whose Join deadline runs from it (plan.md:516). DelayStream keeps its own
//! clock for its own stamps; the two are never mixed.

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::time::Duration;

use npt_link::{Channel, Link, Recv};
use tokio::sync::mpsc;

use crate::cap::ConnCap;

/// How a server listens: its address, the room's clock, the handshake deadline (ns, from the accept stamp: TLS, the
/// upgrade or HTTP/3, and the hand-off to the room) and the process's connection cap.
/// The last two are the caller's values for fix 1's gate (WebSocket only; npt-net names no npt-sim or npt-proto item):
/// `tick_ns` the room's tick period, in which the snapshot budget N counts (NR-U6B-2; npt-server passes
/// 1,000,000,000 / `npt_sim::TICK_HZ`), and `outside_max` the largest datagram written outside an announced tick, the
/// room's own Ping or Pong (R-PING; npt-server passes npt-proto's `Limits::PING` and `Limits::PONG`, 32 B).
#[derive(Debug, Clone)]
pub struct Listen {
    pub addr: SocketAddr,
    pub clock: fn() -> u64,
    pub deadline_ns: u64,
    pub cap: Arc<ConnCap>,
    pub tick_ns: u64,
    pub outside_max: usize,
}

/// One session a server accepted: its link, its peer's address and its accept stamp on the room's clock.
pub struct Accepted {
    pub link: Box<dyn Link + Send>,
    pub peer: SocketAddr,
    pub accept_ns: u64,
}

impl std::fmt::Debug for Accepted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Accepted")
            .field("peer", &self.peer)
            .field("accept_ns", &self.accept_ns)
            .finish_non_exhaustive()
    }
}

/// Accepted sessions waiting for the room, at most this many per server.
pub(crate) const PENDING: usize = 64;
/// Messages a session's inbox holds: a full one drops a datagram (counted) and stops reading a stream (the transport's
/// flow control then holds the peer).
pub(crate) const INBOX: usize = 256;
/// The bytes a session's send queue holds above the transport (tungstenite's 128 KB bound, the catalogue's: it holds
/// a 65,541 B FullState); a reliable message beyond it is `BufferFull`.
pub(crate) const SEND_QUEUE_BYTES: u64 = 128 * 1024;

/// The reader tasks' end of a session's inbox.
#[derive(Clone)]
pub(crate) struct InboxTx {
    tx: mpsc::Sender<Recv>,
    clock: fn() -> u64,
    dropped: Arc<AtomicU64>,
}

impl InboxTx {
    /// A datagram that has just arrived, stamped now: dropped and counted when the inbox is full.
    pub(crate) fn datagram(&self, bytes: Vec<u8>) {
        let r = Recv {
            channel: Channel::Datagram,
            bytes,
            arrival_ns: (self.clock)(),
        };
        if self.tx.try_send(r).is_err() {
            self.dropped.fetch_add(1, Relaxed);
        }
    }

    /// A stream message that has just arrived, stamped now; waits for room (so the reader stops reading). False once
    /// the link is gone.
    pub(crate) async fn stream(&self, channel: Channel, bytes: Vec<u8>) -> bool {
        let r = Recv {
            channel,
            bytes,
            arrival_ns: (self.clock)(),
        };
        self.tx.send(r).await.is_ok()
    }
}

/// A session's inbox (the link's end).
pub(crate) fn inbox(
    clock: fn() -> u64,
    dropped: Arc<AtomicU64>,
) -> (InboxTx, mpsc::Receiver<Recv>) {
    let (tx, rx) = mpsc::channel(INBOX);
    (InboxTx { tx, clock, dropped }, rx)
}

/// The one bound on a link's closing, from its drop (SEC-U6B-1): the drain of its queue, the closing handshake (the
/// control stream's finish and acknowledgement; the Close frame, the peer's, TLS's close_notify) and the close, on both
/// transports. 2 s (WebTransport's wait as built; WebSocket's was 10 s, for its closing handshake alone): a queued
/// `Kick` and the handshake take two or three round trips, 0.6 s at P4's 200 ms before a loss's retransmission (the kick
/// tests run at P2), while a peer that withholds stream credit or stops reading holds a permit at most 2 s past the
/// drop, the handshake deadline's own scale.
pub(crate) const CLOSE_WAIT: Duration = Duration::from_secs(2);

/// Runs a closing task's `work` (its drain, then its closing handshake) until it ends, or until [`CLOSE_WAIT`] after
/// the link's drop, whichever comes first (SEC-U6B-1). `None`: the bound cut it. While the link lives, `work` waits
/// as long as it must (the link's own bounded queue answers the room `BufferFull`).
pub(crate) async fn bounded<T>(
    flags: &Flags,
    work: impl std::future::Future<Output = T>,
) -> Option<T> {
    tokio::select! {
        t = work => Some(t),
        () = async {
            flags.gone.notified().await;
            tokio::time::sleep(CLOSE_WAIT).await;
        } => None,
    }
}

/// A session's closed flag, set by whichever task sees the end first.
#[derive(Debug, Default)]
pub(crate) struct Flags {
    closed: AtomicBool,
    /// The peer closed first (its close frame, its EOF or its connection close): a later error on our side's close is
    /// a normal end (C1c).
    peer_closed: AtomicBool,
    /// The link was dropped: [`bounded`]'s bound runs from here (a stored permit if the closing task is not yet
    /// waiting).
    gone: tokio::sync::Notify,
}

impl Flags {
    /// The link's drop (its `Drop`): the closing's bound starts.
    pub(crate) fn dropped(&self) {
        self.gone.notify_one();
    }
    pub(crate) fn close(&self) {
        self.closed.store(true, Relaxed);
    }
    pub(crate) fn closed(&self) -> bool {
        self.closed.load(Relaxed)
    }
    pub(crate) fn peer_closed(&self) {
        self.peer_closed.store(true, Relaxed);
        self.close();
    }
    pub(crate) fn by_peer(&self) -> bool {
        self.peer_closed.load(Relaxed)
    }
}

/// A server's counts for its transport's event at the run's end (Block 5).
#[derive(Debug, Default)]
pub(crate) struct Counts {
    /// Sessions handed to the room.
    pub(crate) accepted: AtomicU64,
    /// Handshakes that failed before the deadline (one count: QUIC, HTTP/3 or the answer; TLS or the upgrade).
    pub(crate) failed: AtomicU64,
    /// Connections closed at the deadline before a session reached the room.
    pub(crate) deadline: AtomicU64,
    /// Sessions closed as malformed by the transport's framing (a control length above 512 B, an unknown tag).
    pub(crate) malformed: AtomicU64,
    /// Datagrams a full inbox dropped.
    pub(crate) dropped: Arc<AtomicU64>,
    /// Closes whose own end failed while the peer had not closed (C1c: after the peer's close it is a normal end).
    pub(crate) close_failed: AtomicU64,
    /// WebSocket only (R-PING): datagrams outside an announced tick, sent and refused.
    pub(crate) outside: [AtomicU64; 2],
}

impl Counts {
    /// The event's fields (`event.<transport>_accept.<name>`; SERIES.md), with the cap's refusals for `wt`; the
    /// WebSocket's adds its datagrams outside a tick.
    pub(crate) fn rows(&self, cap: &ConnCap, wt: bool) -> Vec<(&'static str, u64)> {
        let mut rows = vec![
            ("accepted_count", self.accepted.load(Relaxed)),
            ("handshake_failed_count", self.failed.load(Relaxed)),
            ("cap_refused_count", cap.refused(wt)),
            ("deadline_closed_count", self.deadline.load(Relaxed)),
            ("malformed_closed_count", self.malformed.load(Relaxed)),
            ("datagrams_dropped_count", self.dropped.load(Relaxed)),
            ("close_failed_count", self.close_failed.load(Relaxed)),
        ];
        if !wt {
            rows.push(("outside_tick_sent_count", self.outside[0].load(Relaxed)));
            rows.push(("outside_tick_refused_count", self.outside[1].load(Relaxed)));
        }
        rows
    }
}
