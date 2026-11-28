//! T3 `WsServer`'s accept path, T2 `WsClient` and T4 the send path with fix 1 (u6b-brief.md Blocks 3 and 4): the
//! WebSocket fallback over TLS 1.3 (tokio-rustls, ring), tokio-tungstenite, and DelayStream on impaired runs.
//!
//! - **Accept** (`WsServer::start`): the TCP accept takes the accept stamp and a permit of the process's [`ConnCap`]
//!   (none free: the connection is dropped at once, counted), before any TLS; then under `timeout_at(accept +
//!   deadline)` (S5 b): TLS (ws_config.rs's acceptor), `DelayStream::wrap` unless `cfg.pass_through` (R20: clean runs
//!   go plain), tokio-tungstenite's accept with `ws_config(cfg.ws_max_message_bytes)`, so tungstenite's limits and
//!   DelayStream's come from one `&L2Config` (S5 a), and the hand-off to the room. The connection's index is its
//!   place in accept order (delay.rs's CONN_INDEX_RULE). One handshake-failure count (TLS or the upgrade); a miss of the
//!   deadline is counted apart. The wrapper is on or off for the process (`header.params.ws_wrapper`). No Origin check
//!   (cut: replan-units.md:19); nothing reads the request's URL or headers, and none is logged (E22).
//! - **`WsLink`**: one binary message per channel message behind its 1 B channel tag (`Channel::ws_tag`; catalogue
//!   :73-76); an unknown tag, or a text message, closes the session as malformed, counted. Its reader runs on its own
//!   task over the WebSocket's split halves, stamping each message as it arrives; its writer on another.
//! - **Fix 1** (Block 4; ws_gate.rs, called never copied): one `TickWriter` per connection, its N counted in the
//!   caller's tick period (`Listen::tick_ns`, NR-U6B-2). `Link::begin_tick` reads
//!   the gate's inputs once (F11): DelayStream's `gate_inputs` (one lock, one model step) on L2, or on the plain path
//!   the kernel's srtt (TCP_INFO) and outstanding bytes (SIOCOUTQ), owed = tungstenite's bytes not yet taken below it
//!   plus rustls's TLS bytes not yet taken by the kernel; on both, plus what this link's own send queue holds. The plain
//!   path's free value is `u64::MAX`: its reason (S5 d), the kernel's send buffer (autotuned to megabytes) is never
//!   the binding condition there; the third condition, outstanding + the snapshot within N snapshots, binds first. The
//!   tick's whole snapshot is gated all or none in the model's bytes (each message's payload, tag, frame header and
//!   TLS's 22 B); a skipped tick answers each of its datagrams `BufferFull`, and a datagram past the announced count
//!   too. A datagram outside any announced tick (R-PING, the brief's condition 1, rev 1.2: the room's own Pings and
//!   Pongs, which its I/O step sends between ticks) is never held to the snapshot budget (a 10 B Ping against N x 10 B
//!   would be refused whenever a snapshot is in flight) and never written ungated: only when it is at most
//!   `Listen::outside_max` (the largest Ping or Pong, 32 B) and fix 1's first two conditions hold (nothing owed below
//!   this link's own bounded queue, which only this check leaves out, and room in the socket); otherwise
//!   `BufferFull`; both counted (`event.ws_accept`'s `outside_tick_*`). Control and FullState messages are
//!   not gated (the 128 KB queue bound, tungstenite's own). The per-tick reads (and their syscalls) run where
//!   `begin_tick` is called: the session's I/O task on the room's I/O runtime, never the tick thread.
//! - **Close:** the queued messages (`Kick(replaced)` first), then a Close frame, the peer's, and the stream's
//!   shutdown, all within 2 s of the link's drop (`session::CLOSE_WAIT`, SEC-U6B-1; it was 10 s for the handshake
//!   alone, DelayStream's own wait, with no bound on the drain: a peer that stops reading is cut at 2 s); an error in
//!   that close after the peer had closed is a normal end (C1c), else counted (a cut counts as an error). The permit
//!   returns, and the connection's `ws_summary` is kept for the room's log, when that ends.
//! - **`WsClient`**: TCP, tokio-rustls's client (ring, ALPN http/1.1, SNI, the pin), tungstenite's client handshake
//!   with the client's own limits (`L2Config::WS_CLIENT_MAX_MESSAGE_BYTES`, never the server's 513 B), masked frames,
//!   the tag. `Connecting` until the upgrade ends.

use std::collections::VecDeque;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicU32, AtomicU64};
use std::sync::{Arc, Mutex, PoisonError};
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use npt_link::{Channel, Link, LinkState, Recv, SendResult};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, ReadBuf};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use tokio_rustls::rustls::pki_types::{
    CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName,
};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

use crate::cap::{Permit, Via};
use crate::session::{
    Accepted, Counts, Flags, InboxTx, Listen, PENDING, SEND_QUEUE_BYTES, bounded, inbox,
};
// The tests' 30 Hz tick (the runs' comes from the caller: `Listen::tick_ns`).
#[cfg(test)]
pub(crate) const TICK_NS: u64 = 33_333_333;
use crate::ws_gate::{Inputs, Snap, TickWriter};
use crate::{DelayStream, L2Config, tcp_kernel, ws_config};

/// The largest datagram a WebSocket link takes: npt-client's receive cap (catalogue's `Snapshot` row: 1,150 B).
const MAX_DATAGRAM: usize = 1_150;
/// One TLS 1.3 record's plaintext and its overhead (S0), for the plain path's TLS bytes and the gate's sizes.
const RECORD: u64 = 16_384;
const TLS_OVERHEAD: u64 = 22;
/// Connection summaries kept for the room's log between its reads.
const SUMMARIES: usize = 1_024;
/// Messages a send queue holds (beside its byte bound).
const SEND_QUEUE: usize = 256;

/// The caller's two values for a connection's gate ([`Listen`]'s `tick_ns` and `outside_max`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct GateCfg {
    pub(crate) tick_ns: u64,
    pub(crate) outside_max: usize,
}

impl GateCfg {
    pub(crate) fn of(l: &Listen) -> GateCfg {
        GateCfg {
            tick_ns: l.tick_ns,
            outside_max: l.outside_max,
        }
    }
}

/// A server-to-client frame's bytes for `payload` bytes (unmasked: 2, 4 or 10 B of header, RFC 6455 §5.2).
fn frame_bytes(payload: u64) -> u64 {
    payload
        + match payload {
            0..=125 => 2,
            126..=65_535 => 4,
            _ => 10,
        }
}

/// One message of `len` bytes in the gate's model bytes: its tag, its frame and TLS's 22 B per record (S0).
fn model_bytes(len: usize) -> u64 {
    let f = frame_bytes(len as u64 + 1);
    f + TLS_OVERHEAD * f.div_ceil(RECORD)
}

/// The plain path's byte counts (S5 d): what tungstenite handed down, what rustls took of it, the TLS bytes those make
/// and what the kernel took of them.
#[derive(Debug, Default)]
struct PlainCounts {
    framed: AtomicU64,
    plain: AtomicU64,
    tls: AtomicU64,
    sock: AtomicU64,
}

/// A stream that counts what its writes took: `took` the bytes, `tls` (above rustls) the TLS bytes they make.
struct Counted<S> {
    inner: S,
    counts: Arc<PlainCounts>,
    above_tls: bool,
}

impl<S: AsyncRead + Unpin> AsyncRead for Counted<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_read(cx, buf)
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for Counted<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        let r = Pin::new(&mut this.inner).poll_write(cx, buf);
        if let Poll::Ready(Ok(n)) = r {
            let n = n as u64;
            if this.above_tls {
                this.counts.plain.fetch_add(n, Relaxed);
                this.counts
                    .tls
                    .fetch_add(n + TLS_OVERHEAD * n.div_ceil(RECORD), Relaxed);
            } else {
                this.counts.sock.fetch_add(n, Relaxed);
            }
        }
        r
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_shutdown(cx)
    }
}

/// Where fix 1's inputs come from.
enum Source {
    /// DelayStream's model (L2).
    Delay(DelayStream<()>),
    /// The plain path: the connection's socket (a second descriptor, for the kernel's readings only) and the counts,
    /// with their values when the link began.
    Plain {
        kernel: TcpStream,
        counts: Arc<PlainCounts>,
        base: [u64; 3],
    },
    /// The tests' fixed inputs.
    #[cfg(test)]
    Fixed(Arc<Mutex<Inputs>>),
}

/// The plain path's stream: tungstenite over TLS, each counted (S5 d).
type PlainWs = WebSocketStream<Counted<tokio_rustls::server::TlsStream<Counted<TcpStream>>>>;

/// The plain path (R20): a second descriptor for the kernel's readings, TLS between the two counters, the upgrade, and
/// fix 1's source with the counts as the upgrade left them.
async fn plain(
    tcp: TcpStream,
    acceptor: &tokio_rustls::TlsAcceptor,
    ws: WebSocketConfig,
) -> io::Result<(PlainWs, Source, Arc<PlainCounts>)> {
    let std = tcp.into_std()?;
    let kernel = TcpStream::from_std(std.try_clone()?)?;
    let tcp = TcpStream::from_std(std)?;
    let counts = Arc::new(PlainCounts::default());
    let below = Counted {
        inner: tcp,
        counts: Arc::clone(&counts),
        above_tls: false,
    };
    let tls = acceptor.accept(below).await?;
    let above = Counted {
        inner: tls,
        counts: Arc::clone(&counts),
        above_tls: true,
    };
    let ws = tokio_tungstenite::accept_async_with_config(above, Some(ws))
        .await
        .map_err(io::Error::other)?;
    let base = [
        counts.plain.load(Relaxed),
        counts.tls.load(Relaxed),
        counts.sock.load(Relaxed),
    ];
    let source = Source::Plain {
        kernel,
        counts: Arc::clone(&counts),
        base,
    };
    Ok((ws, source, counts))
}

/// The server's connections' end summaries (DelayStream's `run_summary`) for the room's log, bounded.
#[derive(Debug, Default)]
struct Summaries {
    kept: Mutex<VecDeque<Vec<(&'static str, u64)>>>,
    dropped: AtomicU64,
}

impl Summaries {
    fn push(&self, s: Vec<(&'static str, u64)>) {
        let mut k = self.kept.lock().unwrap_or_else(PoisonError::into_inner);
        if k.len() < SUMMARIES {
            k.push_back(s);
        } else {
            self.dropped.fetch_add(1, Relaxed);
        }
    }
}

/// The WebSocket server (the module text).
pub struct WsServer {
    listener: Mutex<Option<std::net::TcpListener>>,
    local: SocketAddr,
    cfg: L2Config,
    ws: WebSocketConfig,
    acceptor: tokio_rustls::TlsAcceptor,
    listen: Listen,
    counts: Arc<Counts>,
    summaries: Arc<Summaries>,
    index: Arc<AtomicU32>,
    tx: mpsc::Sender<Accepted>,
    rx: tokio::sync::Mutex<mpsc::Receiver<Accepted>>,
}

impl WsServer {
    /// Binds `listen.addr` (TCP, listening at once) with `cfg` (its WebSocket limits for tungstenite and DelayStream
    /// alike; the wrapper on unless `cfg.pass_through`) and the certificate `chain` (DER, end entity first) with its
    /// PKCS#8 `key` (DER). Nothing is accepted until [`WsServer::start`]; it needs no tokio runtime.
    ///
    /// # Errors
    /// The socket, the certificate, or WebSocket limits DelayStream refuses.
    pub fn bind(
        listen: Listen,
        cfg: &L2Config,
        chain: &[Vec<u8>],
        key: &[u8],
    ) -> io::Result<WsServer> {
        let chain = chain
            .iter()
            .map(|c| CertificateDer::from(c.clone()))
            .collect();
        let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key.to_vec()));
        Self::bind_parts(listen, cfg, chain, key)
    }

    /// [`WsServer::bind`] with the certificate chain and its key as PEM files' contents (U6c's `npt-cert` writes the
    /// public chain and key as PEM; the binary reads each under a size cap).
    ///
    /// # Errors
    /// As `bind`, or PEM that holds no certificate or no key.
    pub fn bind_pem(
        listen: Listen,
        cfg: &L2Config,
        chain_pem: &[u8],
        key_pem: &[u8],
    ) -> io::Result<WsServer> {
        use tokio_rustls::rustls::pki_types::pem::PemObject;
        let bad = |what: &str| io::Error::new(io::ErrorKind::InvalidInput, what.to_owned());
        let chain: Vec<CertificateDer<'static>> = CertificateDer::pem_slice_iter(chain_pem)
            .collect::<Result<_, _>>()
            .map_err(|_| bad("the chain's PEM"))?;
        if chain.is_empty() {
            return Err(bad("the chain's PEM holds no certificate"));
        }
        let key = PrivateKeyDer::from_pem_slice(key_pem).map_err(|_| bad("the key's PEM"))?;
        Self::bind_parts(listen, cfg, chain, key)
    }

    fn bind_parts(
        listen: Listen,
        cfg: &L2Config,
        chain: Vec<CertificateDer<'static>>,
        key: PrivateKeyDer<'static>,
    ) -> io::Result<WsServer> {
        if !(1..=16_370).contains(&cfg.ws_max_message_bytes) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "ws_max_message_bytes is 1 to 16,370",
            ));
        }
        let acceptor = ws_config::acceptor(chain, key)?;
        let listener = std::net::TcpListener::bind(listen.addr)?;
        listener.set_nonblocking(true)?;
        let local = listener.local_addr()?;
        let (tx, rx) = mpsc::channel(PENDING);
        Ok(WsServer {
            listener: Mutex::new(Some(listener)),
            local,
            ws: ws_config::ws_config(cfg.ws_max_message_bytes),
            cfg: cfg.clone(),
            acceptor,
            listen,
            counts: Arc::new(Counts::default()),
            summaries: Arc::new(Summaries::default()),
            index: Arc::new(AtomicU32::new(0)),
            tx,
            rx: tokio::sync::Mutex::new(rx),
        })
    }

    /// The bound address.
    pub fn local_addr(&self) -> SocketAddr {
        self.local
    }

    /// Whether the WebSocket path runs through DelayStream (`header.params.ws_wrapper`).
    pub fn wrapper(&self) -> bool {
        !self.cfg.pass_through
    }

    /// Starts accepting on the current tokio runtime (the room's I/O runtime). Once only.
    ///
    /// # Errors
    /// No runtime, or a second call.
    pub fn start(&self) -> io::Result<()> {
        let handle = tokio::runtime::Handle::try_current().map_err(io::Error::other)?;
        let std = self
            .listener
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
            .ok_or_else(|| io::Error::other("started already"))?;
        let _g = handle.enter();
        let listener = tokio::net::TcpListener::from_std(std)?;
        let a = Acceptor {
            cfg: self.cfg.clone(),
            ws: self.ws,
            acceptor: self.acceptor.clone(),
            listen: self.listen.clone(),
            counts: Arc::clone(&self.counts),
            summaries: Arc::clone(&self.summaries),
            index: Arc::clone(&self.index),
            tx: self.tx.clone(),
        };
        handle.spawn(a.run(listener));
        Ok(())
    }

    /// The next session, in the order their handshakes ended; `None` once the server is gone.
    pub async fn accept(&self) -> Option<Accepted> {
        self.rx.lock().await.recv().await
    }

    /// The transport's event at the run's end (`event.ws_accept`), as [`crate::WtServer::event_rows`]'s, with the
    /// connection summaries a full store dropped.
    pub fn event_rows(&self) -> Vec<(&'static str, u64)> {
        let mut rows = self.counts.rows(&self.listen.cap, false);
        rows.push((
            "summaries_dropped_count",
            self.summaries.dropped.load(Relaxed),
        ));
        rows
    }

    /// The `ws_summary` of each connection that ended since the last call (DelayStream's `run_summary`; none on the
    /// plain path), for the room's log.
    pub fn take_summaries(&self) -> Vec<Vec<(&'static str, u64)>> {
        self.summaries
            .kept
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .drain(..)
            .collect()
    }
}

/// What the accept loop and its handshake tasks share.
#[derive(Clone)]
struct Acceptor {
    cfg: L2Config,
    ws: WebSocketConfig,
    acceptor: tokio_rustls::TlsAcceptor,
    listen: Listen,
    counts: Arc<Counts>,
    summaries: Arc<Summaries>,
    index: Arc<AtomicU32>,
    tx: mpsc::Sender<Accepted>,
}

impl Acceptor {
    async fn run(self, listener: tokio::net::TcpListener) {
        loop {
            let (tcp, peer) = match listener.accept().await {
                Ok(x) => x,
                // Out of descriptors or a connection reset before its accept: try again shortly.
                Err(_) => {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    continue;
                }
            };
            let accept_ns = (self.listen.clock)();
            let ds_ns = DelayStream::now_ns();
            let start = tokio::time::Instant::now();
            let Some(permit) = self.listen.cap.take(Via::Ws) else {
                drop(tcp);
                continue;
            };
            let index = self.index.fetch_add(1, Relaxed);
            let a = self.clone();
            tokio::spawn(async move {
                let deadline = start + Duration::from_nanos(a.listen.deadline_ns);
                let counts = Arc::clone(&a.counts);
                let hand_off = a.handshake(tcp, peer, (accept_ns, ds_ns), index, permit);
                match tokio::time::timeout_at(deadline, hand_off).await {
                    Ok(Ok(())) => counts.accepted.fetch_add(1, Relaxed),
                    Ok(Err(())) => counts.failed.fetch_add(1, Relaxed),
                    Err(_) => counts.deadline.fetch_add(1, Relaxed),
                };
            });
        }
    }

    /// TLS, the wrapper or the plain path's counts, the upgrade, the hand-off (all under the caller's deadline).
    async fn handshake(
        self,
        tcp: TcpStream,
        peer: SocketAddr,
        (accept_ns, ds_ns): (u64, u64),
        index: u32,
        permit: Permit,
    ) -> Result<(), ()> {
        let _ = tcp.set_nodelay(true);
        let (clock, gate) = (self.listen.clock, GateCfg::of(&self.listen));
        let link = if self.cfg.pass_through {
            let (ws, source, counts) = plain(tcp, &self.acceptor, self.ws).await.map_err(drop)?;
            WsLink::start(
                ws,
                Some((source, gate)),
                clock,
                &self.counts,
                Some(permit),
                None,
                Some(counts),
            )
        } else {
            let tls = self.acceptor.accept(tcp).await.map_err(drop)?;
            let ds = DelayStream::wrap(tls, &self.cfg, index, ds_ns).map_err(drop)?;
            let handle = ds.handle();
            let summary = (ds.handle(), Arc::clone(&self.summaries));
            let ws = tokio_tungstenite::accept_async_with_config(ds, Some(self.ws))
                .await
                .map_err(drop)?;
            WsLink::start(
                ws,
                Some((Source::Delay(handle), gate)),
                clock,
                &self.counts,
                Some(permit),
                Some(summary),
                None,
            )
        };
        self.tx
            .send(Accepted {
                link: Box::new(link),
                peer,
                accept_ns,
            })
            .await
            .map_err(drop)
    }
}

/// Fix 1's state on one connection (Block 4).
struct Gate {
    source: Source,
    writer: TickWriter,
    tick: u64,
    /// The datagrams the current tick announced and has not yet sent, and whether the gate let that tick through.
    announced: usize,
    allowed: bool,
    srtt_ns: Option<u64>,
    /// The largest datagram written outside any announced tick (or past its count: the two are one case).
    outside_max: usize,
    /// The server's counts, where those are counted, sent and refused (`event.ws_accept`).
    counts: Arc<Counts>,
    /// The tests' count of input readings.
    #[cfg(test)]
    reads: std::cell::Cell<u64>,
}

/// One WebSocket session as a `Link` (the module text).
pub(crate) struct WsLink {
    out: Option<mpsc::Sender<Vec<u8>>>,
    queued: Arc<AtomicU64>,
    inbox: mpsc::Receiver<Recv>,
    flags: Arc<Flags>,
    gate: Option<Gate>,
}

impl WsLink {
    /// The link over `ws`: its reader and writer tasks on the current runtime. `gate` is the server's (fix 1: its
    /// inputs' source and the caller's values), none for a client; `permit` returns, and `summary` is kept, when the
    /// closing ends; `framed` counts the frames the writer hands tungstenite (the plain path's owed bytes).
    fn start<S>(
        ws: WebSocketStream<S>,
        gate: Option<(Source, GateCfg)>,
        clock: fn() -> u64,
        counts: &Arc<Counts>,
        permit: Option<Permit>,
        summary: Option<(DelayStream<()>, Arc<Summaries>)>,
        framed: Option<Arc<PlainCounts>>,
    ) -> WsLink
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        let flags = Arc::new(Flags::default());
        let (itx, irx) = inbox(clock, Arc::clone(&counts.dropped));
        let (otx, orx) = mpsc::channel(SEND_QUEUE);
        let queued = Arc::new(AtomicU64::new(0));
        let (sink, stream) = ws.split();
        let (back_tx, back_rx) = oneshot::channel();
        let reader = tokio::spawn(reader(
            stream,
            itx,
            Arc::clone(&flags),
            Arc::clone(counts),
            back_tx,
        ));
        let w = Writer {
            queued: Arc::clone(&queued),
            framed,
            flags: Arc::clone(&flags),
            counts: Arc::clone(counts),
            permit,
            summary,
        };
        tokio::spawn(w.run(sink, orx, back_rx, reader));
        WsLink {
            out: Some(otx),
            queued,
            inbox: irx,
            flags,
            gate: gate.map(|(source, c)| Gate {
                source,
                writer: TickWriter::new(c.tick_ns),
                tick: 0,
                announced: 0,
                allowed: false,
                srtt_ns: None,
                outside_max: c.outside_max,
                counts: Arc::clone(counts),
                #[cfg(test)]
                reads: std::cell::Cell::new(0),
            }),
        }
    }

    /// Queues one tagged message for the writer; `BufferFull` past the queue's bound.
    fn queue(&mut self, channel: Channel, bytes: &[u8]) -> SendResult {
        let Some(out) = &self.out else {
            return SendResult::Sent;
        };
        let len = bytes.len() as u64 + 1;
        if self.queued.load(Relaxed) + len > SEND_QUEUE_BYTES {
            return SendResult::BufferFull;
        }
        let mut m = Vec::with_capacity(bytes.len() + 1);
        m.push(channel.ws_tag());
        m.extend_from_slice(bytes);
        self.queued.fetch_add(len, Relaxed);
        match out.try_send(m) {
            Ok(()) => SendResult::Sent,
            Err(mpsc::error::TrySendError::Full(_)) => {
                self.queued.fetch_sub(len, Relaxed);
                SendResult::BufferFull
            }
            Err(mpsc::error::TrySendError::Closed(_)) => SendResult::Sent,
        }
    }

    /// Fix 1's inputs in one reading (F11), with this link's own queue counted as owed when `own_queue` (a tick's; a
    /// datagram outside a tick leaves that bounded queue out: R-PING).
    fn inputs(&self, own_queue: bool) -> Option<Inputs> {
        let g = self.gate.as_ref()?;
        #[cfg(test)]
        g.reads.set(g.reads.get() + 1);
        let queued = self.queued.load(Relaxed);
        let mut inp = match &g.source {
            #[cfg(test)]
            Source::Fixed(f) => *f.lock().unwrap_or_else(PoisonError::into_inner),
            Source::Delay(ds) => ds.gate_inputs(),
            Source::Plain {
                kernel,
                counts,
                base,
            } => {
                let framed = counts.framed.load(Relaxed);
                let plain = counts.plain.load(Relaxed).saturating_sub(base[0]);
                let tls = counts.tls.load(Relaxed).saturating_sub(base[1]);
                let sock = counts.sock.load(Relaxed).saturating_sub(base[2]);
                Inputs {
                    srtt_ns: tcp_kernel::srtt_ns(kernel).ok().flatten(),
                    // A failed read refuses every write (never a best value).
                    outstanding: tcp_kernel::outstanding_bytes(kernel).unwrap_or(u64::MAX),
                    owed: framed.saturating_sub(plain) + tls.saturating_sub(sock),
                    free: u64::MAX,
                }
            }
        };
        if own_queue {
            inp.owed += queued;
        }
        Some(inp)
    }

    /// The gate's counts: (ticks written, ticks skipped, datagrams outside a tick sent, and refused).
    #[cfg(test)]
    pub(crate) fn gate_counts(&self) -> Option<[u64; 4]> {
        let g = self.gate.as_ref()?;
        Some([
            g.writer.written(),
            g.writer.skipped(),
            g.counts.outside[0].load(Relaxed),
            g.counts.outside[1].load(Relaxed),
        ])
    }
}

impl Link for WsLink {
    fn begin_tick(&mut self, lens: &[usize], _now_ns: u64) -> SendResult {
        // NR-U6B-4: an empty tick is no tick (never gated as a 0 B snapshot, never counted skipped).
        if lens.is_empty() {
            return SendResult::Sent;
        }
        let Some(inp) = self.inputs(true) else {
            return SendResult::Sent;
        };
        let size = lens.iter().map(|&l| model_bytes(l)).sum();
        let Some(g) = self.gate.as_mut() else {
            return SendResult::Sent;
        };
        let snap = Snap { id: g.tick, size };
        g.tick += 1;
        g.srtt_ns = inp.srtt_ns;
        g.allowed = g.writer.on_tick(snap, inp).is_some();
        g.announced = lens.len();
        if g.allowed {
            SendResult::Sent
        } else {
            SendResult::BufferFull
        }
    }

    fn send_unreliable(&mut self, bytes: &[u8], _now_ns: u64) -> SendResult {
        let outside = match self.gate.as_mut() {
            None => false,
            Some(g) if g.announced > 0 => {
                g.announced -= 1;
                if !g.allowed {
                    return SendResult::BufferFull;
                }
                false
            }
            Some(_) => true,
        };
        // After the count (NR-U6B-4): a refused datagram still takes its tick's slot.
        if bytes.len() > MAX_DATAGRAM {
            return SendResult::TooLarge;
        }
        if outside {
            // Outside an announced tick (R-PING, the module text): a Ping or Pong at most, with nothing owed below this
            // link's own queue and room in the socket; never the snapshot budget.
            let max = self.gate.as_ref().map_or(0, |g| g.outside_max);
            let ok = bytes.len() <= max
                && self
                    .inputs(false)
                    .is_some_and(|i| i.owed == 0 && i.free >= model_bytes(bytes.len()));
            if let Some(g) = &self.gate {
                g.counts.outside[usize::from(!ok)].fetch_add(1, Relaxed);
            }
            if !ok {
                return SendResult::BufferFull;
            }
        }
        self.queue(Channel::Datagram, bytes)
    }

    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], _now_ns: u64) -> SendResult {
        assert!(
            channel != Channel::Datagram,
            "send_reliable on Channel::Datagram: datagrams go through send_unreliable"
        );
        self.queue(channel, bytes)
    }

    fn poll_recv(&mut self, _now_ns: u64) -> Option<Recv> {
        self.inbox.try_recv().ok()
    }

    fn state(&self) -> LinkState {
        if self.flags.closed() {
            LinkState::Closed
        } else {
            LinkState::Open
        }
    }

    fn rtt_ns(&self) -> Option<u64> {
        self.gate.as_ref()?.srtt_ns
    }

    fn max_datagram_size(&self) -> usize {
        MAX_DATAGRAM
    }

    fn unsent_unreliable(&self) -> usize {
        0
    }
}

impl Drop for WsLink {
    /// Hands the connection to its closing (the module text): the writer drains the queue, then closes, within
    /// CLOSE_WAIT from here.
    fn drop(&mut self) {
        self.out.take();
        self.flags.dropped();
    }
}

/// Every message the peer sends, by its tag, stamped as it arrives; an unknown tag or a text message closes the
/// session as malformed (counted). At its end it hands its half back for the writer's shutdown.
async fn reader<S>(
    mut stream: SplitStream<WebSocketStream<S>>,
    itx: InboxTx,
    flags: Arc<Flags>,
    counts: Arc<Counts>,
    back: oneshot::Sender<SplitStream<WebSocketStream<S>>>,
) where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut open = true;
    loop {
        match stream.next().await {
            Some(Ok(Message::Binary(b))) => {
                let Some((&tag, body)) = b.split_first() else {
                    counts.malformed.fetch_add(1, Relaxed);
                    flags.close();
                    break;
                };
                match Channel::from_ws_tag(tag) {
                    Some(Channel::Datagram) => {
                        if open {
                            itx.datagram(body.to_vec());
                        }
                    }
                    Some(ch) => {
                        open = open && itx.stream(ch, body.to_vec()).await;
                    }
                    None => {
                        counts.malformed.fetch_add(1, Relaxed);
                        flags.close();
                        break;
                    }
                }
            }
            Some(Ok(Message::Text(_))) => {
                counts.malformed.fetch_add(1, Relaxed);
                flags.close();
                break;
            }
            // Pings, pongs and frames tungstenite answers itself.
            Some(Ok(Message::Ping(_) | Message::Pong(_) | Message::Frame(_))) => {}
            Some(Ok(Message::Close(_))) | Some(Err(_)) | None => {
                flags.peer_closed();
                break;
            }
        }
    }
    let _ = back.send(stream);
}

/// The writer task's parts.
struct Writer {
    queued: Arc<AtomicU64>,
    framed: Option<Arc<PlainCounts>>,
    flags: Arc<Flags>,
    counts: Arc<Counts>,
    permit: Option<Permit>,
    summary: Option<(DelayStream<()>, Arc<Summaries>)>,
}

impl Writer {
    /// Writes what the link queues, in order; when the link is dropped, the closing (the module text).
    async fn run<S>(
        self,
        mut sink: SplitSink<WebSocketStream<S>, Message>,
        mut orx: mpsc::Receiver<Vec<u8>>,
        back: oneshot::Receiver<SplitStream<WebSocketStream<S>>>,
        reader: tokio::task::JoinHandle<()>,
    ) where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let work = async {
            let mut failed = false;
            while let Some(m) = orx.recv().await {
                let len = m.len() as u64;
                if let Some(f) = &self.framed {
                    f.framed.fetch_add(frame_bytes(len), Relaxed);
                }
                let ok = sink.send(Message::Binary(m.into())).await.is_ok();
                self.queued.fetch_sub(len, Relaxed);
                if !ok {
                    failed = true;
                    break;
                }
            }
            failed |= sink.send(Message::Close(None)).await.is_err();
            match back.await.ok().map(|st| sink.reunite(st)) {
                Some(Ok(mut ws)) => failed |= ws.get_mut().shutdown().await.is_err(),
                _ => failed = true,
            }
            failed
        };
        // From the link's drop, the drain and the closing handshake under one bound (SEC-U6B-1).
        let failed = bounded(&self.flags, work).await.unwrap_or(true);
        reader.abort();
        if failed && !self.flags.by_peer() {
            self.counts.close_failed.fetch_add(1, Relaxed);
        }
        self.flags.close();
        if let Some((ds, s)) = &self.summary {
            s.push(ds.run_summary());
        }
        drop(self.permit);
    }
}

/// A native WebSocket client (the module text): a `Link` that is `Connecting` until its upgrade ends.
pub struct WsClient {
    pending: Option<oneshot::Receiver<io::Result<WsLink>>>,
    link: Option<WsLink>,
    counts: Arc<Counts>,
}

impl WsClient {
    /// Starts a session to `addr` on the current tokio runtime: TLS with SNI `name` and the server's certificate pinned
    /// by its SHA-256 `hash`, each message stamped on `clock`.
    ///
    /// # Errors
    /// No tokio runtime, or `name` is not a server name.
    pub fn connect(
        addr: SocketAddr,
        name: &str,
        hash: [u8; 32],
        clock: fn() -> u64,
    ) -> io::Result<WsClient> {
        let handle = tokio::runtime::Handle::try_current().map_err(io::Error::other)?;
        let sni = ServerName::try_from(name.to_owned())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "not a server name"))?;
        let url = format!("wss://{name}/npt");
        let counts = Arc::new(Counts::default());
        let (tx, rx) = oneshot::channel();
        let c = Arc::clone(&counts);
        handle.spawn(async move {
            let _ = tx.send(ws_client(addr, sni, url, hash, clock, c).await);
        });
        Ok(WsClient {
            pending: Some(rx),
            link: None,
            counts,
        })
    }

    /// Messages the client refused as malformed (an unknown tag, a text message).
    pub fn malformed(&self) -> u64 {
        self.counts.malformed.load(Relaxed)
    }

    fn link(&mut self) -> Option<&mut WsLink> {
        if let Some(rx) = &mut self.pending {
            match rx.try_recv() {
                Ok(r) => {
                    self.pending = None;
                    self.link = r.ok();
                }
                Err(oneshot::error::TryRecvError::Empty) => return None,
                Err(oneshot::error::TryRecvError::Closed) => self.pending = None,
            }
        }
        self.link.as_mut()
    }
}

/// tungstenite's client configuration: the client's own limits (condition 7), the same 128 KB write bound.
fn client_config() -> WebSocketConfig {
    WebSocketConfig::default()
        .max_frame_size(Some(L2Config::WS_CLIENT_MAX_MESSAGE_BYTES))
        .max_message_size(Some(L2Config::WS_CLIENT_MAX_MESSAGE_BYTES))
        .max_write_buffer_size(ws_config::MAX_WRITE_BUFFER_BYTES)
        .write_buffer_size(0)
}

async fn ws_client(
    addr: SocketAddr,
    sni: ServerName<'static>,
    url: String,
    hash: [u8; 32],
    clock: fn() -> u64,
    counts: Arc<Counts>,
) -> io::Result<WsLink> {
    let tcp = TcpStream::connect(addr).await?;
    tcp.set_nodelay(true)?;
    let tls = crate::pin::client_config(ws_config::provider(), hash, b"http/1.1")?;
    let connector = tokio_rustls::TlsConnector::from(Arc::new(tls));
    let stream = connector.connect(sni, tcp).await?;
    let (ws, _) = tokio_tungstenite::client_async_with_config(url, stream, Some(client_config()))
        .await
        .map_err(io::Error::other)?;
    Ok(WsLink::start(ws, None, clock, &counts, None, None, None))
}

impl Link for WsClient {
    fn send_unreliable(&mut self, bytes: &[u8], now_ns: u64) -> SendResult {
        match self.link() {
            Some(l) => l.send_unreliable(bytes, now_ns),
            None => SendResult::BufferFull,
        }
    }

    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], now_ns: u64) -> SendResult {
        match self.link() {
            Some(l) => l.send_reliable(channel, bytes, now_ns),
            None => SendResult::BufferFull,
        }
    }

    fn poll_recv(&mut self, now_ns: u64) -> Option<Recv> {
        self.link()?.poll_recv(now_ns)
    }

    fn state(&self) -> LinkState {
        match (&self.pending, &self.link) {
            (Some(_), _) => LinkState::Connecting,
            (None, Some(l)) => l.state(),
            (None, None) => LinkState::Closed,
        }
    }

    fn rtt_ns(&self) -> Option<u64> {
        None
    }

    fn max_datagram_size(&self) -> usize {
        MAX_DATAGRAM
    }

    fn unsent_unreliable(&self) -> usize {
        0
    }
}

#[cfg(test)]
#[path = "ws_server_tests.rs"]
mod tests;
