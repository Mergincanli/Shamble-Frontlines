//! T1 `WtServer` and T2 `WtClient` (u6b-brief.md Block 2): WebTransport on web-transport-quinn 0.13.6 (the runs'
//! one crate; with both features on, this is still web-transport-quinn's), ring (Block 0), over our own quinn
//! endpoint on ImpairSocket (l2/wt.rs's `endpoint` and `wtq_server::config_with`).
//!
//! - **Accept** (`WtServer::start`): quinn's `Endpoint::accept` driven here; each `Incoming` takes a permit of the
//!   process's [`ConnCap`] at once (none free: `Incoming::refuse`, counted); then the whole handshake (QUIC, HTTP/3's
//!   SETTINGS and CONNECT, the answer) and the hand-off to the room run under one `timeout_at(first packet +
//!   deadline)`, which closes l2/wt.rs's carry (a client's keep-alives no longer hold an HTTP/3 stage open). One
//!   handshake-failure count (the three-reason split stays the loopback tests'); a miss of the deadline is counted
//!   apart. Migration off (M9); quinn's congestion controller from the run's `cc` (`newreno` or `cubic`, E6).
//! - **`WtLink`**: datagrams by quinn's `send_datagram` after `datagram_send_buffer_space()` (ADR 0001:439-442):
//!   `BufferFull` when short (quinn would drop older datagrams instead). The client's first bidirectional stream is the
//!   control stream: each message its 2 B length first, a length above 512 B closing the session as malformed
//!   (counted); each FullState on its own server unidirectional stream with its 4 B length (catalogue :68-72).
//!   `unsent_unreliable` reads quinn's datagram send buffer: its bytes in use (quinn reports only its free space), not
//!   a datagram count. The inbox is bounded (a full one drops a datagram, counted, and stops reading the control stream,
//!   so QUIC's flow control holds the peer).
//! - **Close:** dropping a `WtLink` hands the connection to its closing task: the queued control messages are written
//!   (`Kick(replaced)` among them), the control stream finished and its acknowledgement awaited, all within 2 s of the
//!   drop (`session::CLOSE_WAIT`, SEC-U6B-1: a peer that grants no stream credit is cut there), then the session
//!   closed; the permit returns when that task ends (advisor-u6a-brief.md:202-205: an immediate QUIC close can drop
//!   unsent stream data).
//! - **`WtClient`**: web-transport-quinn's `Client::new` on our own quinn endpoint (its builder binds `[::]:0` with an
//!   `unwrap`: replan-units.md:76), ring, ALPN h3, the certificate pinned by its SHA-256 (`pin`); `Connecting` (sends
//!   answered `BufferFull`) until the session opens; the client half of the framing, a FullState at most 65,541 B.

use std::io;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use npt_link::{Channel, Link, LinkState, Recv, SendResult};
use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio::sync::{mpsc, oneshot};
use web_transport_quinn::{RecvStream, SendStream, Session};

use crate::cap::{Permit, Via};
use crate::l2::wt::{endpoint, wtq_server};
use crate::session::{
    Accepted, Counts, Flags, InboxTx, Listen, PENDING, SEND_QUEUE_BYTES, bounded, inbox,
};
use crate::{ImpairSocket, L2Config};

/// The largest control-stream message either end takes: its 2 B length, its type and its body (catalogue :68-71).
pub(crate) const CONTROL_MAX: usize = 512;
/// The largest full-state message the client takes, its 4 B length included (u6b-brief.md Block 2: 65,541 B).
pub(crate) const FULL_STATE_MAX: usize = 65_541;
/// quinn's datagram send buffer (its TransportConfig default, 1 MiB), which `unsent_unreliable` reads.
const DATAGRAM_BUFFER: usize = 1 << 20;
/// The session's close code when the transport's framing refused a message (an HTTP/3 application code).
const MALFORMED: u32 = 1;
/// Messages a send queue holds (beside its byte bound).
const SEND_QUEUE: usize = 256;

/// quinn's transport for `cc` (`newreno` or `cubic`), datagrams on with a send buffer of `buffer` bytes.
fn transport(cc: &str, buffer: usize) -> io::Result<quinn::TransportConfig> {
    let mut t = quinn::TransportConfig::default();
    match cc {
        "newreno" => {
            t.congestion_controller_factory(Arc::new(quinn::congestion::NewRenoConfig::default()))
        }
        "cubic" => {
            t.congestion_controller_factory(Arc::new(quinn::congestion::CubicConfig::default()))
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cc is newreno or cubic",
            ));
        }
    };
    t.datagram_send_buffer_size(buffer);
    Ok(t)
}

/// The server's stream limits (SEC-U6B-2; WtClient keeps quinn's defaults): what a client may open, and leave unread, on
/// a server that reads one bidirectional stream (the control stream) after the CONNECT and no unidirectional one.
/// Bidirectional: the CONNECT request's stream and the control stream, plus 2 of margin (a stream counts until its close
/// is acknowledged).
const SERVER_BIDI: u32 = 4;
/// Unidirectional: HTTP/3's control stream and QPACK's encoder and decoder streams (RFC 9114 §6.2; RFC 9204 §4.2), plus
/// 3 of margin for reserved stream types (RFC 9114 §6.2.3's grease); U7's headless Chromium run confirms the count.
const SERVER_UNI: u32 = 6;
/// Per stream: 64 KiB, 128 of the largest control message (512 B); the CONNECT's headers are far smaller.
const SERVER_STREAM_WINDOW: u32 = 64 * 1024;
/// Per connection: 1 MiB of unread stream data (datagrams are not flow controlled), 56 MiB over the cap's 56 sessions
/// at N = 48, where quinn's defaults (100 + 100 streams x 1.25 MB, no connection bound) allow about 250 MB each.
const SERVER_WINDOW: u32 = 1 << 20;

/// The server's transport: [`transport`] with the stream limits above.
fn server_transport(cc: &str) -> io::Result<quinn::TransportConfig> {
    let mut t = transport(cc, DATAGRAM_BUFFER)?;
    t.max_concurrent_bidi_streams(SERVER_BIDI.into())
        .max_concurrent_uni_streams(SERVER_UNI.into())
        .stream_receive_window(SERVER_STREAM_WINDOW.into())
        .receive_window(SERVER_WINDOW.into());
    Ok(t)
}

/// The WebTransport server (the module text).
pub struct WtServer {
    socket: Arc<ImpairSocket>,
    configs: Mutex<Option<(quinn::EndpointConfig, quinn::ServerConfig)>>,
    endpoint: Mutex<Option<quinn::Endpoint>>,
    listen: Listen,
    counts: Arc<Counts>,
    tx: mpsc::Sender<Accepted>,
    rx: tokio::sync::Mutex<mpsc::Receiver<Accepted>>,
    buffer: usize,
}

impl WtServer {
    /// Binds `listen.addr` (UDP, through ImpairSocket with `cfg`) with the certificate `chain` (DER, end entity first)
    /// and its PKCS#8 `key` (DER), quinn's congestion controller `cc`. Nothing is served until [`WtServer::start`]; it
    /// needs no tokio runtime (ImpairSocket runs its own threads, started here: the caller's CPU pin is theirs).
    ///
    /// # Errors
    /// The socket, the certificate or `cc` (`newreno` or `cubic`).
    pub fn bind(
        listen: Listen,
        cfg: &L2Config,
        chain: &[Vec<u8>],
        key: &[u8],
        cc: &str,
    ) -> io::Result<WtServer> {
        Self::bind_with(listen, cfg, chain, key, server_transport(cc)?)
    }

    fn bind_with(
        listen: Listen,
        cfg: &L2Config,
        chain: &[Vec<u8>],
        key: &[u8],
        transport: quinn::TransportConfig,
    ) -> io::Result<WtServer> {
        let chain = chain
            .iter()
            .map(|c| CertificateDer::from(c.clone()))
            .collect();
        let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key.to_vec()));
        let configs = wtq_server::config_with(chain, key, transport)?;
        let socket = ImpairSocket::bind(listen.addr, cfg.clone())?;
        let (tx, rx) = mpsc::channel(PENDING);
        Ok(WtServer {
            socket,
            configs: Mutex::new(Some(configs)),
            endpoint: Mutex::new(None),
            listen,
            counts: Arc::new(Counts::default()),
            tx,
            rx: tokio::sync::Mutex::new(rx),
            buffer: DATAGRAM_BUFFER,
        })
    }

    /// The bound address.
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        quinn::AsyncUdpSocket::local_addr(&*self.socket)
    }

    /// The bound ImpairSocket (its header numbers and per-second rows: `l2_*`).
    pub fn socket(&self) -> &ImpairSocket {
        &self.socket
    }

    /// Starts serving: the quinn endpoint over the socket and the accept loop, on the current tokio runtime (the room's
    /// I/O runtime: call it inside `TickLoop::io_handle().enter()`). Once only.
    ///
    /// # Errors
    /// No runtime, a second call, or the endpoint.
    pub fn start(&self) -> io::Result<()> {
        let handle = tokio::runtime::Handle::try_current().map_err(io::Error::other)?;
        let configs = self
            .configs
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
            .ok_or_else(|| io::Error::other("started already"))?;
        let ep = endpoint(Arc::clone(&self.socket), configs)?;
        *self.endpoint.lock().unwrap_or_else(PoisonError::into_inner) = Some(ep.clone());
        handle.spawn(accept_loop(
            ep,
            self.listen.clone(),
            Arc::clone(&self.counts),
            self.tx.clone(),
            self.buffer,
        ));
        Ok(())
    }

    /// The next session, in the order their handshakes ended; `None` once the server is gone.
    pub async fn accept(&self) -> Option<Accepted> {
        self.rx.lock().await.recv().await
    }

    /// The transport's event at the run's end (`event.wt_accept`): sessions accepted, handshake failures, refused at
    /// the cap, closed at the deadline before a session, closed as malformed, datagrams a full inbox dropped.
    pub fn event_rows(&self) -> Vec<(&'static str, u64)> {
        self.counts.rows(&self.listen.cap, true)
    }
}

impl Drop for WtServer {
    fn drop(&mut self) {
        if let Some(ep) = self
            .endpoint
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
        {
            ep.close(0u32.into(), b"");
        }
    }
}

/// The accept loop (the module text).
async fn accept_loop(
    ep: quinn::Endpoint,
    listen: Listen,
    counts: Arc<Counts>,
    tx: mpsc::Sender<Accepted>,
    buffer: usize,
) {
    while let Some(incoming) = ep.accept().await {
        let accept_ns = (listen.clock)();
        let start = tokio::time::Instant::now();
        let Some(permit) = listen.cap.take(Via::Wt) else {
            incoming.refuse();
            continue;
        };
        let peer = incoming.remote_address();
        let (counts, tx) = (Arc::clone(&counts), tx.clone());
        let (clock, deadline) = (listen.clock, Duration::from_nanos(listen.deadline_ns));
        tokio::spawn(async move {
            let hand_off = async {
                let conn = incoming.await.map_err(drop)?;
                let req = web_transport_quinn::Request::accept(conn)
                    .await
                    .map_err(drop)?;
                let session = req.ok().await.map_err(drop)?;
                let link = WtLink::server(session, permit, clock, &counts, buffer);
                tx.send(Accepted {
                    link: Box::new(link),
                    peer,
                    accept_ns,
                })
                .await
                .map_err(drop)
            };
            match tokio::time::timeout_at(start + deadline, hand_off).await {
                Ok(Ok(())) => counts.accepted.fetch_add(1, Relaxed),
                Ok(Err(())) => counts.failed.fetch_add(1, Relaxed),
                Err(_) => counts.deadline.fetch_add(1, Relaxed),
            };
        });
    }
}

/// What a link queues for its writer task.
enum WtOut {
    Control(Vec<u8>),
    FullState(Vec<u8>),
}

/// One WebTransport session as a `Link` (the module text).
pub(crate) struct WtLink {
    session: Session,
    inbox: mpsc::Receiver<Recv>,
    out: Option<mpsc::Sender<WtOut>>,
    queued: Arc<AtomicU64>,
    flags: Arc<Flags>,
    buffer: usize,
}

impl WtLink {
    /// The server's link over `session`: its datagram reader now, and a task that takes the client's first
    /// bidirectional stream as the control stream, then writes what the link queues and, at its drop, closes.
    fn server(
        session: Session,
        permit: Permit,
        clock: fn() -> u64,
        counts: &Arc<Counts>,
        buffer: usize,
    ) -> WtLink {
        let (link, itx, orx) = WtLink::parts(session, clock, counts, buffer);
        let (s, flags, queued, counts) = (
            link.session.clone(),
            Arc::clone(&link.flags),
            Arc::clone(&link.queued),
            Arc::clone(counts),
        );
        tokio::spawn(async move {
            let mut orx = orx;
            let pair = loop {
                tokio::select! {
                    bi = s.accept_bi() => break bi.ok(),
                    o = orx.recv() => match o {
                        // Nothing can be written before the control stream exists (the room answers a Join that came
                        // on it): dropped.
                        Some(o) => queued.fetch_sub(o.len(), Relaxed),
                        None => break None,
                    },
                };
            };
            match pair {
                Some((send, recv)) => {
                    tokio::spawn(control_reader(
                        recv,
                        itx,
                        s.clone(),
                        Arc::clone(&flags),
                        Arc::clone(&counts),
                        CONTROL_MAX,
                    ));
                    writer(s, send, orx, queued, flags).await;
                }
                None => s.close(0, b""),
            }
            drop(permit);
        });
        link
    }

    /// The link's parts and its datagram reader.
    fn parts(
        session: Session,
        clock: fn() -> u64,
        counts: &Arc<Counts>,
        buffer: usize,
    ) -> (WtLink, InboxTx, mpsc::Receiver<WtOut>) {
        let flags = Arc::new(Flags::default());
        let (itx, irx) = inbox(clock, Arc::clone(&counts.dropped));
        let (otx, orx) = mpsc::channel(SEND_QUEUE);
        tokio::spawn(datagram_reader(
            session.clone(),
            itx.clone(),
            Arc::clone(&flags),
        ));
        let link = WtLink {
            session,
            inbox: irx,
            out: Some(otx),
            queued: Arc::new(AtomicU64::new(0)),
            flags,
            buffer,
        };
        (link, itx, orx)
    }

    fn queue(&mut self, o: WtOut) -> SendResult {
        let len = o.len();
        let Some(out) = &self.out else {
            return SendResult::Sent;
        };
        if self.queued.load(Relaxed) + len > SEND_QUEUE_BYTES {
            return SendResult::BufferFull;
        }
        self.queued.fetch_add(len, Relaxed);
        match out.try_send(o) {
            Ok(()) => SendResult::Sent,
            Err(mpsc::error::TrySendError::Full(_)) => {
                self.queued.fetch_sub(len, Relaxed);
                SendResult::BufferFull
            }
            // The writer ended (the session is gone): dropped, as a send to a vanished peer.
            Err(mpsc::error::TrySendError::Closed(_)) => SendResult::Sent,
        }
    }
}

impl WtOut {
    fn len(&self) -> u64 {
        match self {
            WtOut::Control(b) | WtOut::FullState(b) => b.len() as u64,
        }
    }
}

impl Link for WtLink {
    fn send_unreliable(&mut self, bytes: &[u8], _now_ns: u64) -> SendResult {
        if self.flags.closed() {
            return SendResult::Sent;
        }
        if bytes.len() > self.session.max_datagram_size() {
            return SendResult::TooLarge;
        }
        if self.session.datagram_send_buffer_space() < bytes.len() {
            return SendResult::BufferFull;
        }
        match self.session.send_datagram(bytes.to_vec().into()) {
            Ok(()) => SendResult::Sent,
            Err(_) if self.session.close_reason().is_some() => {
                self.flags.close();
                SendResult::Sent
            }
            Err(_) => SendResult::BufferFull,
        }
    }

    fn send_reliable(&mut self, channel: Channel, bytes: &[u8], _now_ns: u64) -> SendResult {
        match channel {
            Channel::Datagram => {
                panic!("send_reliable on Channel::Datagram: datagrams go through send_unreliable")
            }
            Channel::Control if bytes.len() > CONTROL_MAX => SendResult::TooLarge,
            Channel::Control => self.queue(WtOut::Control(bytes.to_vec())),
            Channel::FullState if bytes.len() > FULL_STATE_MAX => SendResult::TooLarge,
            Channel::FullState => self.queue(WtOut::FullState(bytes.to_vec())),
        }
    }

    fn poll_recv(&mut self, _now_ns: u64) -> Option<Recv> {
        self.inbox.try_recv().ok()
    }

    fn state(&self) -> LinkState {
        if self.flags.closed() || self.session.close_reason().is_some() {
            LinkState::Closed
        } else {
            LinkState::Open
        }
    }

    fn rtt_ns(&self) -> Option<u64> {
        u64::try_from(self.session.rtt().as_nanos()).ok()
    }

    fn max_datagram_size(&self) -> usize {
        self.session.max_datagram_size()
    }

    fn unsent_unreliable(&self) -> usize {
        self.buffer
            .saturating_sub(self.session.datagram_send_buffer_space())
    }
}

impl Drop for WtLink {
    /// Hands the connection to its closing task (the module text): the writer drains the queue, then closes, within
    /// CLOSE_WAIT from here.
    fn drop(&mut self) {
        self.out.take();
        self.flags.dropped();
    }
}

/// Every datagram the peer sends, stamped as it arrives; the session's end closes the link.
async fn datagram_reader(session: Session, itx: InboxTx, flags: Arc<Flags>) {
    while let Ok(d) = session.read_datagram().await {
        itx.datagram(d.to_vec());
    }
    flags.peer_closed();
}

/// The control stream's messages, each read whole (its 2 B length first; above `max` the session closes as
/// malformed, counted) and stamped as it arrives.
async fn control_reader(
    mut recv: RecvStream,
    itx: InboxTx,
    session: Session,
    flags: Arc<Flags>,
    counts: Arc<Counts>,
    max: usize,
) {
    loop {
        let mut len = [0u8; 2];
        if recv.read_exact(&mut len).await.is_err() {
            return;
        }
        let n = usize::from(u16::from_le_bytes(len));
        if 2 + n > max {
            counts.malformed.fetch_add(1, Relaxed);
            session.close(MALFORMED, b"malformed");
            flags.close();
            return;
        }
        let mut msg = vec![0u8; 2 + n];
        msg[..2].copy_from_slice(&len);
        if recv.read_exact(&mut msg[2..]).await.is_err() || !itx.stream(Channel::Control, msg).await
        {
            return;
        }
    }
}

/// The client's full-state streams, one message each (its 4 B length first; above 65,541 B the session closes as
/// malformed, counted).
async fn full_state_reader(session: Session, itx: InboxTx, flags: Arc<Flags>, counts: Arc<Counts>) {
    while let Ok(mut r) = session.accept_uni().await {
        let mut len = [0u8; 4];
        if r.read_exact(&mut len).await.is_err() {
            continue;
        }
        let n = u32::from_le_bytes(len) as usize;
        if n > FULL_STATE_MAX - 4 {
            counts.malformed.fetch_add(1, Relaxed);
            session.close(MALFORMED, b"malformed");
            flags.close();
            return;
        }
        let mut msg = vec![0u8; 4 + n];
        msg[..4].copy_from_slice(&len);
        if r.read_exact(&mut msg[4..]).await.is_err() {
            continue;
        }
        if !itx.stream(Channel::FullState, msg).await {
            return;
        }
    }
}

/// Writes what the link queues, in order (a FullState on its own unidirectional stream); when the link is dropped,
/// the closing sequence (the module text).
async fn writer(
    session: Session,
    mut send: SendStream,
    mut orx: mpsc::Receiver<WtOut>,
    queued: Arc<AtomicU64>,
    flags: Arc<Flags>,
) {
    let work = async {
        while let Some(o) = orx.recv().await {
            let len = o.len();
            let ok = match o {
                WtOut::Control(b) => send.write_all(&b).await.is_ok(),
                WtOut::FullState(b) => match session.open_uni().await {
                    Ok(mut s) => s.write_all(&b).await.is_ok() && s.finish().is_ok(),
                    Err(_) => false,
                },
            };
            queued.fetch_sub(len, Relaxed);
            if !ok {
                break;
            }
        }
        // The closing sequence: the control stream finished and its acknowledgement awaited.
        if send.finish().is_ok() {
            let _ = send.stopped().await;
        }
    };
    // From the link's drop, the drain and the sequence above under one bound (SEC-U6B-1), then the close.
    bounded(&flags, work).await;
    session.close(0, b"");
    flags.close();
}

/// A native WebTransport client (the module text): a `Link` that is `Connecting` until its session opens.
pub struct WtClient {
    pending: Option<oneshot::Receiver<io::Result<WtLink>>>,
    link: Option<WtLink>,
    counts: Arc<Counts>,
}

impl WtClient {
    /// Starts a session to `addr` on the current tokio runtime, the server's certificate pinned by its SHA-256 `hash`,
    /// each message stamped on `clock`.
    ///
    /// # Errors
    /// No tokio runtime.
    pub fn connect(addr: SocketAddr, hash: [u8; 32], clock: fn() -> u64) -> io::Result<WtClient> {
        let handle = tokio::runtime::Handle::try_current().map_err(io::Error::other)?;
        let counts = Arc::new(Counts::default());
        let (tx, rx) = oneshot::channel();
        let c = Arc::clone(&counts);
        handle.spawn(async move {
            let _ = tx.send(client(addr, hash, clock, c).await);
        });
        Ok(WtClient {
            pending: Some(rx),
            link: None,
            counts,
        })
    }

    /// Messages the client refused as malformed (a control message above 512 B, a FullState above 65,541 B).
    pub fn malformed(&self) -> u64 {
        self.counts.malformed.load(Relaxed)
    }

    /// The link once its session is open.
    fn link(&mut self) -> Option<&mut WtLink> {
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

/// The client's session: our own quinn endpoint, web-transport-quinn's client, the pin; then the control stream.
async fn client(
    addr: SocketAddr,
    hash: [u8; 32],
    clock: fn() -> u64,
    counts: Arc<Counts>,
) -> io::Result<WtLink> {
    let tls = crate::pin::client_config(wtq_server::provider(), hash, b"h3")?;
    let crypto =
        quinn::crypto::rustls::QuicClientConfig::try_from(tls).map_err(io::Error::other)?;
    let local = if addr.is_ipv4() {
        SocketAddr::from(([0, 0, 0, 0], 0))
    } else {
        SocketAddr::from(([0u16; 8], 0))
    };
    let ep = quinn::Endpoint::client(local)?;
    let mut config = quinn::ClientConfig::new(Arc::new(crypto));
    config.transport_config(Arc::new(transport("cubic", DATAGRAM_BUFFER)?));
    let client = web_transport_quinn::Client::new(ep, config);
    let url = url(&format!("https://{addr}/npt"), |r| &r.url)?;
    let session = client
        .connect(web_transport_quinn::proto::ConnectRequest::new(url))
        .await
        .map_err(io::Error::other)?;
    let (send, recv) = session.open_bi().await.map_err(io::Error::other)?;
    let (link, itx, orx) = WtLink::parts(session, clock, &counts, DATAGRAM_BUFFER);
    let (s, flags, queued) = (
        link.session.clone(),
        Arc::clone(&link.flags),
        Arc::clone(&link.queued),
    );
    tokio::spawn(control_reader(
        recv,
        itx.clone(),
        s.clone(),
        Arc::clone(&flags),
        Arc::clone(&counts),
        CONTROL_MAX,
    ));
    tokio::spawn(full_state_reader(
        s.clone(),
        itx,
        Arc::clone(&flags),
        counts,
    ));
    tokio::spawn(writer(s, send, orx, queued, flags));
    Ok(link)
}

/// The URL as web-transport-quinn's own type, named through `ConnectRequest`'s field (url is no dependency here).
fn url<T: std::str::FromStr>(
    s: &str,
    _field: impl Fn(&web_transport_quinn::proto::ConnectRequest) -> &T,
) -> io::Result<T> {
    s.parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "not a URL"))
}

impl Link for WtClient {
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
        self.link.as_ref()?.rtt_ns()
    }

    fn max_datagram_size(&self) -> usize {
        self.link.as_ref().map_or(0, WtLink::max_datagram_size)
    }

    fn unsent_unreliable(&self) -> usize {
        self.link.as_ref().map_or(0, WtLink::unsent_unreliable)
    }
}

#[cfg(test)]
#[path = "wt_server_tests.rs"]
pub(crate) mod tests;
