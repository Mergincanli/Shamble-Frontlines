//! Blocks 1, 3 and 4's tests (u6b-brief.md section 3): WsServer and WsClient on 127.0.0.1, the cap and the pin across
//! both transports, the stamps, and fix 1's send path through `WsLink`. Real-clock waits of at most 5 s each, no timing
//! bar.

use super::*;
use crate::cap::ConnCap;
use crate::l2::tls_test::self_signed;
use crate::ws_gate::{Bite, snapshot_budget};
use npt_l1::Profile;
use std::sync::OnceLock;
use std::time::Instant;
use tokio::io::AsyncReadExt;

fn clock() -> u64 {
    static T0: OnceLock<Instant> = OnceLock::new();
    T0.get_or_init(Instant::now).elapsed().as_nanos() as u64
}

fn local() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 0))
}

/// The test certificate: its chain, its key (both DER) and its SHA-256.
fn cert() -> (Vec<Vec<u8>>, Vec<u8>, [u8; 32]) {
    let (c, k) = self_signed(&ws_config::provider());
    let hash = crate::pin::sha256(c.as_ref());
    let PrivateKeyDer::Pkcs8(k) = k else {
        unreachable!("PKCS#8")
    };
    (vec![c.to_vec()], k.secret_pkcs8_der().to_vec(), hash)
}

fn listen(cap: &Arc<ConnCap>, deadline_ms: u64) -> Listen {
    Listen {
        addr: local(),
        clock,
        deadline_ns: deadline_ms * 1_000_000,
        cap: Arc::clone(cap),
        tick_ns: TICK_NS,
        outside_max: 32,
    }
}

/// The tests' gate: 30 Hz, a Ping or Pong of at most 32 B outside a tick.
const GATE: GateCfg = GateCfg {
    tick_ns: TICK_NS,
    outside_max: 32,
};

/// A started server at `cfg`, its address and its certificate's hash.
fn server(
    cfg: &L2Config,
    cap: &Arc<ConnCap>,
    deadline_ms: u64,
) -> (WsServer, SocketAddr, [u8; 32]) {
    let (chain, key, hash) = cert();
    let s = WsServer::bind(listen(cap, deadline_ms), cfg, &chain, &key).expect("bound");
    s.start().expect("started");
    let addr = s.local_addr();
    (s, addr, hash)
}

async fn until<T>(limit: Duration, mut f: impl FnMut() -> Option<T>) -> Option<T> {
    let end = tokio::time::Instant::now() + limit;
    loop {
        if let Some(t) = f() {
            return Some(t);
        }
        if tokio::time::Instant::now() >= end {
            return None;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}

async fn next<L: Link + ?Sized>(l: &mut L, limit: Duration) -> Option<Recv> {
    until(limit, || l.poll_recv(clock())).await
}

/// A WsClient open to `addr` and the server's side of its session.
async fn pair(s: &WsServer, addr: SocketAddr, hash: [u8; 32]) -> (WsClient, Accepted) {
    let mut c = WsClient::connect(addr, "localhost", hash, clock).expect("a runtime");
    let a = tokio::time::timeout(Duration::from_secs(5), s.accept())
        .await
        .expect("in time")
        .expect("a session");
    let open = until(Duration::from_secs(5), || {
        c.poll_recv(clock());
        (c.state() == LinkState::Open).then_some(())
    })
    .await;
    assert!(open.is_some(), "the client opened");
    (c, a)
}

/// A raw TLS stream to `addr` (the pin, ALPN http/1.1), from a socket with receive buffer `rcvbuf` if given.
async fn raw_tls(
    addr: SocketAddr,
    hash: [u8; 32],
    rcvbuf: Option<u32>,
) -> tokio_rustls::client::TlsStream<TcpStream> {
    let sock = tokio::net::TcpSocket::new_v4().expect("a socket");
    if let Some(b) = rcvbuf {
        sock.set_recv_buffer_size(b).expect("SO_RCVBUF");
    }
    let tcp = sock.connect(addr).await.expect("connected");
    tcp.set_nodelay(true).expect("TCP_NODELAY");
    tls_on(tcp, hash).await
}

/// TLS (the pin, ALPN http/1.1) on a connected `tcp`.
async fn tls_on(tcp: TcpStream, hash: [u8; 32]) -> tokio_rustls::client::TlsStream<TcpStream> {
    let tls = crate::pin::client_config(ws_config::provider(), hash, b"http/1.1").expect("TLS");
    tokio_rustls::TlsConnector::from(Arc::new(tls))
        .connect(ServerName::try_from("localhost").expect("a name"), tcp)
        .await
        .expect("the TLS handshake")
}

type RawWs = WebSocketStream<tokio_rustls::client::TlsStream<TcpStream>>;

/// A raw tungstenite client (no tag of ours) over `raw_tls`.
async fn raw_ws(addr: SocketAddr, hash: [u8; 32], rcvbuf: Option<u32>) -> RawWs {
    let tls = raw_tls(addr, hash, rcvbuf).await;
    tokio_tungstenite::client_async("wss://localhost/npt", tls)
        .await
        .expect("the upgrade")
        .0
}

// ------------------------------------------------------------------------------------------------------------------
// Block 1.

/// Block 1 (P1's test): one cap of N + 8 across both transports: with N = 1, nine sessions (five WebTransport, four
/// WebSocket) take every permit; a tenth is refused on either, counted per transport; once one session's closing
/// ends, its permit is back and a new session opens.
#[cfg(feature = "wt-wtq")]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn b1_one_cap_of_n_plus_8_across_both_transports() {
    use crate::wt_server::tests as wt;
    let cap = ConnCap::new(1);
    assert_eq!(cap.permits(), 9);
    let (ws, ws_addr, hash) = server(&L2Config::clean(1), &cap, 2_000);
    let (wts, wt_addr, wt_hash) = wt::server(L2Config::clean(1), &cap, "cubic", 2_000);
    let mut held = Vec::new();
    for _ in 0..5 {
        held.push(wt::pair(&wts, wt_addr, wt_hash).await);
    }
    let mut held_ws = Vec::new();
    for _ in 0..4 {
        held_ws.push(pair(&ws, ws_addr, hash).await);
    }
    assert_eq!(cap.free(), 0);
    let mut late_wt = crate::WtClient::connect(wt_addr, wt_hash, clock).expect("a runtime");
    let mut late_ws = WsClient::connect(ws_addr, "localhost", hash, clock).expect("a runtime");
    let refused = until(Duration::from_secs(5), || {
        late_wt.poll_recv(clock());
        late_ws.poll_recv(clock());
        (late_wt.state() == LinkState::Closed && late_ws.state() == LinkState::Closed).then_some(())
    })
    .await;
    assert!(refused.is_some(), "both refused at the cap");
    assert_eq!((cap.refused(true), cap.refused(false)), (1, 1));
    drop(held_ws.pop());
    let back = until(Duration::from_secs(5), || (cap.free() == 1).then_some(())).await;
    assert!(
        back.is_some(),
        "the permit came back when the closing ended"
    );
    let _again = pair(&ws, ws_addr, hash).await;
    assert_eq!(cap.free(), 0);
}

/// Block 1: the pin on both clients: the server's certificate hash is accepted; another hash is refused, and the
/// client never opens.
#[cfg(feature = "wt-wtq")]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn b1_the_pin_takes_the_hash_and_refuses_another_on_both_clients() {
    use crate::wt_server::tests as wt;
    let cap = ConnCap::new(4);
    let (ws, ws_addr, hash) = server(&L2Config::clean(2), &cap, 2_000);
    let (wts, wt_addr, wt_hash) = wt::server(L2Config::clean(2), &cap, "cubic", 2_000);
    let _ok_ws = pair(&ws, ws_addr, hash).await;
    let _ok_wt = wt::pair(&wts, wt_addr, wt_hash).await;
    let mut wrong = hash;
    wrong[0] ^= 1;
    let mut bad_ws = WsClient::connect(ws_addr, "localhost", wrong, clock).expect("a runtime");
    let mut bad_wt = crate::WtClient::connect(wt_addr, wrong, clock).expect("a runtime");
    let closed = until(Duration::from_secs(5), || {
        bad_ws.poll_recv(clock());
        bad_wt.poll_recv(clock());
        (bad_ws.state() == LinkState::Closed && bad_wt.state() == LinkState::Closed).then_some(())
    })
    .await;
    assert!(closed.is_some(), "neither client opened with another hash");
}

/// The injected clock of the stamp test: the test sets it, and counts the server's reads of it (the clock read first,
/// then the count, both SeqCst: a read the test has counted saw the value before the test's next store).
static STAMP_NOW: AtomicU64 = AtomicU64::new(0);
static STAMP_READS: AtomicU64 = AtomicU64::new(0);

fn stamp_clock() -> u64 {
    use std::sync::atomic::Ordering::SeqCst;
    let t = STAMP_NOW.load(SeqCst);
    STAMP_READS.fetch_add(1, SeqCst);
    t
}

/// Block 1 (P2's test; T-2, T-3): the session's accept stamp is the clock's at the TCP accept: the clock moves from 500
/// to 700 after the accept took its permit and before TLS and the upgrade, and the session carries 500. Each message's
/// arrival stamp is read by the reader task when it arrives, never at `poll_recv`: messages that arrived at 1,000 (a
/// bounded wait on the reader's own two clock reads, never a sleep) are polled with the clock at 9,000 and keep 1,000.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b1_arrival_stamps_are_taken_at_arrival_never_at_poll() {
    use std::sync::atomic::Ordering::SeqCst;
    STAMP_NOW.store(500, SeqCst);
    let cap = ConnCap::new(2);
    let (chain, key, hash) = cert();
    let l = Listen {
        clock: stamp_clock,
        ..listen(&cap, 2_000)
    };
    let s = WsServer::bind(l, &L2Config::clean(3), &chain, &key).expect("bound");
    s.start().expect("started");
    let tcp = TcpStream::connect(s.local_addr()).await.expect("connected");
    let accepted = until(Duration::from_secs(5), || {
        (cap.free() < cap.permits()).then_some(())
    })
    .await;
    assert!(accepted.is_some(), "the TCP accept took its permit");
    STAMP_NOW.store(700, SeqCst);
    let tls = tls_on(tcp, hash).await;
    let (mut ws, _) = tokio_tungstenite::client_async("wss://localhost/npt", tls)
        .await
        .expect("the upgrade");
    let mut a = tokio::time::timeout(Duration::from_secs(5), s.accept())
        .await
        .expect("in time")
        .expect("a session");
    assert_eq!(a.accept_ns, 500, "the accept stamp, at the TCP accept");
    STAMP_NOW.store(1_000, SeqCst);
    let reads = STAMP_READS.load(SeqCst);
    for (ch, b) in [(Channel::Control, b'c'), (Channel::Datagram, b'd')] {
        let m = Message::Binary(vec![ch.ws_tag(), b].into());
        ws.send(m).await.expect("sent");
    }
    let arrived = until(Duration::from_secs(5), || {
        (STAMP_READS.load(SeqCst) >= reads + 2).then_some(())
    })
    .await;
    assert!(
        arrived.is_some(),
        "both arrived, the link not polled meanwhile"
    );
    STAMP_NOW.store(9_000, SeqCst);
    let mut stamps = Vec::new();
    let _ = until(Duration::from_secs(5), || {
        stamps.extend(a.link.poll_recv(9_000).map(|r| r.arrival_ns));
        (stamps.len() == 2).then_some(())
    })
    .await;
    assert_eq!(stamps, [1_000, 1_000], "stamped at arrival");
}

// ------------------------------------------------------------------------------------------------------------------
// Block 3.

/// Block 3 (P7's test): `one_source` on WsServer's own tungstenite configuration: both limits from the one
/// `&L2Config` (400 B here, not the catalogue's 513 B), and DelayStream's limit, read from its header numbers, the same.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b3_one_source_for_tungstenite_and_delaystream_limits() {
    let mut cfg = L2Config::new(4, Profile::p1());
    cfg.ws_max_message_bytes = 400;
    let cap = ConnCap::new(2);
    let (s, addr, hash) = server(&cfg, &cap, 2_000);
    crate::ws_config::one_source(&s.ws, 400).expect("one source");
    let (_c, _a) = pair(&s, addr, hash).await;
    let numbers = DelayStream::header_numbers_from(&cfg);
    assert!(
        numbers.contains(&("ws_max_message_bytes", 400)),
        "{numbers:?}"
    );
}

/// Block 3 (P4's test): the deadline covers TLS and the upgrade: a TCP client that never starts TLS, and one that
/// finishes TLS but never sends its upgrade request, are each closed at the deadline (the elapsed time at least it),
/// counted; neither reaches the room.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b3_the_deadline_covers_tls_and_the_upgrade() {
    const DEADLINE_MS: u64 = 300;
    let cap = ConnCap::new(2);
    let (s, addr, hash) = server(&L2Config::new(5, Profile::p1()), &cap, DEADLINE_MS);
    let t0 = Instant::now();
    let mut silent = TcpStream::connect(addr).await.expect("connected");
    let mut buf = [0u8; 16];
    let n = tokio::time::timeout(Duration::from_secs(5), silent.read(&mut buf))
        .await
        .expect("closed in time");
    let took = t0.elapsed();
    assert!(matches!(n, Ok(0) | Err(_)), "{n:?}");
    assert!(took >= Duration::from_millis(DEADLINE_MS), "TLS: {took:?}");
    let t0 = Instant::now();
    let mut tls = raw_tls(addr, hash, None).await;
    let n = tokio::time::timeout(Duration::from_secs(5), tls.read(&mut buf))
        .await
        .expect("closed in time");
    let took = t0.elapsed();
    assert!(matches!(n, Ok(0) | Err(_)), "{n:?}");
    assert!(
        took >= Duration::from_millis(DEADLINE_MS),
        "the upgrade: {took:?}"
    );
    let counted = until(Duration::from_secs(2), || {
        (s.event_rows()[3].1 == 2).then_some(())
    })
    .await;
    assert!(counted.is_some(), "{:?}", s.event_rows());
    assert_eq!(s.event_rows()[0], ("accepted_count", 0));
    assert_eq!(cap.free(), cap.permits(), "both permits back");
}

/// Block 3: a pass-through configuration goes the plain path (R20): no wrapper (`ws_wrapper` 0), fix 1's inputs from
/// the kernel, messages both ways.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b3_the_plain_path_when_pass_through() {
    let cap = ConnCap::new(2);
    let (s, addr, hash) = server(&L2Config::clean(6), &cap, 2_000);
    assert!(!s.wrapper());
    let (mut c, mut a) = pair(&s, addr, hash).await;
    // A snapshot-sized tick once the upgrade's bytes are acknowledged (a 5 B one could not pass 3 x its own size).
    let snap = vec![7u8; 1_000];
    let sent = until(Duration::from_secs(5), || {
        (a.link.begin_tick(&[1_000], 0) == SendResult::Sent).then_some(())
    })
    .await;
    assert!(sent.is_some(), "the gate let a tick through");
    assert_eq!(a.link.send_unreliable(&snap, 0), SendResult::Sent);
    let r = next(&mut c, Duration::from_secs(5)).await.expect("down");
    assert_eq!((r.channel, r.bytes), (Channel::Datagram, snap));
    assert_eq!(c.send_unreliable(b"up", 0), SendResult::Sent);
    let r = next(&mut *a.link, Duration::from_secs(5))
        .await
        .expect("up");
    assert_eq!(r.bytes, b"up");
    assert!(
        s.take_summaries().is_empty(),
        "no DelayStream, no ws_summary"
    );
}

/// Block 3: on the wrapper's path each connection's index is its place in accept order (CONN_INDEX_RULE), as its
/// `ws_summary` reports at its end.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn b3_connection_indexes_follow_accept_order() {
    let cap = ConnCap::new(4);
    let (s, addr, hash) = server(&L2Config::new(7, Profile::p1()), &cap, 2_000);
    for _ in 0..3 {
        let (c, a) = pair(&s, addr, hash).await;
        drop(a);
        drop(c);
    }
    let mut got = Vec::new();
    let all = until(Duration::from_secs(5), || {
        got.extend(s.take_summaries());
        (got.len() == 3).then_some(())
    })
    .await;
    assert!(all.is_some(), "three summaries");
    let mut idx: Vec<u64> = got
        .iter()
        .map(|r| {
            r.iter()
                .find(|(n, _)| *n == "ws_conns_before_count")
                .expect("an index")
                .1
        })
        .collect();
    idx.sort_unstable();
    assert_eq!(idx, [0, 1, 2]);
}

/// Block 3: tags route to their channels; an unknown tag closes the session as malformed, counted, and what follows it
/// never reaches the room.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b3_tags_route_and_an_unknown_tag_closes_as_malformed() {
    let cap = ConnCap::new(2);
    let (s, addr, hash) = server(&L2Config::clean(8), &cap, 2_000);
    let mut raw = raw_ws(addr, hash, None).await;
    let mut a = s.accept().await.expect("a session");
    for m in [&[0u8, 1][..], &[1, 2], &[2, 3], &[9, 4], &[1, 5]] {
        raw.send(Message::Binary(m.to_vec().into()))
            .await
            .expect("sent");
    }
    let mut got = Vec::new();
    let closed = until(Duration::from_secs(5), || {
        while let Some(r) = a.link.poll_recv(clock()) {
            got.push((r.channel, r.bytes));
        }
        (a.link.state() == LinkState::Closed).then_some(())
    })
    .await;
    assert!(closed.is_some());
    while let Some(r) = a.link.poll_recv(clock()) {
        got.push((r.channel, r.bytes));
    }
    assert_eq!(
        got,
        vec![
            (Channel::Datagram, vec![1]),
            (Channel::Control, vec![2]),
            (Channel::FullState, vec![3])
        ]
    );
    assert_eq!(s.event_rows()[4], ("malformed_closed_count", 1));
}

/// Block 3 (P6's test; U6a's carry): `Kick(replaced)`, queued just before the server's link is dropped, reaches the
/// client before the Close, 20 times at once on DelayStream at P2 with its loss. Delivery only (C-U6-19's line).
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn b3_a_kick_queued_before_the_drop_is_read_before_the_close() {
    const RUNS: usize = 20;
    let cap = ConnCap::new(RUNS);
    let (s, addr, hash) = server(&L2Config::new(9, Profile::p2()), &cap, 4_000);
    let kick = [3u8, 0, 12, 0, 0];
    let mut clients = Vec::new();
    let mut servers = Vec::new();
    for _ in 0..RUNS {
        clients.push(WsClient::connect(addr, "localhost", hash, clock).expect("a runtime"));
    }
    for _ in 0..RUNS {
        servers.push(
            tokio::time::timeout(Duration::from_secs(5), s.accept())
                .await
                .expect("in time")
                .expect("a session"),
        );
    }
    for a in &mut servers {
        assert_eq!(
            a.link.send_reliable(Channel::Control, &kick, 0),
            SendResult::Sent
        );
    }
    drop(servers);
    let reads = clients.into_iter().map(|mut c| {
        tokio::spawn(async move {
            let mut got = None;
            let _ = until(Duration::from_secs(4), || {
                if let Some(r) = c.poll_recv(clock()) {
                    got = Some(r.bytes);
                    return Some(());
                }
                (c.state() == LinkState::Closed).then_some(())
            })
            .await;
            got
        })
    });
    let mut kicked = 0;
    for r in reads {
        kicked += usize::from(r.await.expect("a reader").as_deref() == Some(&kick[..]));
    }
    assert_eq!(kicked, RUNS, "every client read the kick before the close");
}

/// Block 3: the client masks its frames and tags its messages: over DelayStream (whose scanner closes an unmasked
/// client frame before buffering it) each of its channels reaches the server as itself.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b3_the_client_masks_and_tags() {
    let cap = ConnCap::new(2);
    let (s, addr, hash) = server(&L2Config::new(10, Profile::p1()), &cap, 2_000);
    let (mut c, mut a) = pair(&s, addr, hash).await;
    assert_eq!(c.send_unreliable(b"d", 0), SendResult::Sent);
    assert_eq!(c.send_reliable(Channel::Control, b"c", 0), SendResult::Sent);
    let mut got = Vec::new();
    while got.len() < 2 {
        let r = next(&mut *a.link, Duration::from_secs(5))
            .await
            .expect("up");
        got.push((r.channel, r.bytes));
    }
    got.sort();
    assert_eq!(
        got,
        vec![
            (Channel::Datagram, b"d".to_vec()),
            (Channel::Control, b"c".to_vec())
        ]
    );
    assert_eq!(
        a.link.state(),
        LinkState::Open,
        "no unmasked frame closed it"
    );
}

/// Condition 7 (T-9): a FullState of the largest size the room sends, exactly (65,536 + 5 B with its 4 B length:
/// catalogue :73-76, wt_server.rs's FULL_STATE_MAX), reaches the WsClient: the client's own limits, never the server's
/// 513 B.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b3_a_largest_full_state_reaches_the_client() {
    const FULL_STATE_MAX: usize = 65_536 + 5;
    let cap = ConnCap::new(2);
    let (s, addr, hash) = server(&L2Config::new(11, Profile::p1()), &cap, 2_000);
    let (mut c, mut a) = pair(&s, addr, hash).await;
    let mut full = vec![0u8; FULL_STATE_MAX];
    full[..4].copy_from_slice(&(FULL_STATE_MAX as u32 - 4).to_le_bytes());
    assert_eq!(
        a.link.send_reliable(Channel::FullState, &full, 0),
        SendResult::Sent
    );
    let r = next(&mut c, Duration::from_secs(5))
        .await
        .expect("the full state");
    assert_eq!(
        (r.channel, r.bytes.len()),
        (Channel::FullState, FULL_STATE_MAX)
    );
    assert_eq!(c.malformed(), 0);
}

/// Block 4 (P11's test; C1c): a shutdown error after the peer's close is a normal end: a client that drops its TCP
/// at once (a reset, no TLS close_notify, no Close frame) makes the server's close fail, uncounted; the same failure
/// with the peer still there would count.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b4_a_close_error_after_the_peers_close_is_a_normal_end() {
    let cap = ConnCap::new(2);
    let (s, addr, hash) = server(&L2Config::clean(12), &cap, 2_000);
    let raw = raw_ws(addr, hash, None).await;
    let a = s.accept().await.expect("a session");
    let tcp = raw.into_inner().into_inner().0;
    tcp.set_zero_linger().expect("SO_LINGER 0");
    drop(tcp);
    let seen = until(Duration::from_secs(5), || {
        (a.link.state() == LinkState::Closed).then_some(())
    })
    .await;
    assert!(seen.is_some(), "the reader saw the peer go");
    drop(a);
    let back = until(Duration::from_secs(5), || {
        (cap.free() == cap.permits()).then_some(())
    })
    .await;
    assert!(back.is_some(), "the closing ended");
    assert_eq!(s.event_rows()[6], ("close_failed_count", 0));
}

// ------------------------------------------------------------------------------------------------------------------
// Block 4: fix 1 through WsLink.

/// A server-side WsLink over a loopback WebSocket on DelayStream (`cfg`), the DelayStream's handle, and the client.
async fn delay_link(
    cfg: &L2Config,
    conn: u32,
) -> (WsLink, DelayStream<()>, crate::l2::delay::tests::ClientWs) {
    let (server, client, _, _) = crate::l2::delay::tests::pair(cfg, conn).await;
    let ds = server.get_ref().handle();
    let counts = Arc::new(Counts::default());
    let link = WsLink::start(
        server,
        Some((Source::Delay(ds.handle()), GATE)),
        clock,
        &counts,
        None,
        None,
        None,
    );
    (link, ds, client)
}

/// A server-side WsLink whose gate reads fixed inputs (the tests set them), and its raw client.
async fn fixed_link(
    seed: u64,
) -> (
    WsLink,
    Arc<Mutex<Inputs>>,
    crate::l2::delay::tests::ClientWs,
) {
    fixed_link_at(seed, GATE).await
}

/// [`fixed_link`] with the gate's caller values `gate`.
async fn fixed_link_at(
    seed: u64,
    gate: GateCfg,
) -> (
    WsLink,
    Arc<Mutex<Inputs>>,
    crate::l2::delay::tests::ClientWs,
) {
    let (server, client, _, _) =
        crate::l2::delay::tests::pair(&L2Config::new(seed, Profile::p1()), 0).await;
    let fixed = Arc::new(Mutex::new(Inputs {
        srtt_ns: Some(TICK_NS),
        outstanding: 0,
        owed: 0,
        free: u64::MAX,
    }));
    let counts = Arc::new(Counts::default());
    let link = WsLink::start(
        server,
        Some((Source::Fixed(Arc::clone(&fixed)), gate)),
        clock,
        &counts,
        None,
        None,
        None,
    );
    (link, fixed, client)
}

/// Block 4 (P8's test, RC-1): the gate's inputs are read once per tick, whatever the tick's datagrams.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b4_the_inputs_are_read_once_per_tick() {
    let (mut l, _fixed, _client) = fixed_link(13).await;
    for _ in 0..5 {
        assert_eq!(l.begin_tick(&[100, 200, 300], 0), SendResult::Sent);
        for n in [100, 200, 300] {
            assert_eq!(l.send_unreliable(&vec![1; n], 0), SendResult::Sent);
        }
        // The link's own queue is owed until its writer passes it down.
        drained(&l).await;
    }
    assert_eq!(
        l.gate.as_ref().expect("a gate").reads.get(),
        5,
        "one reading per tick"
    );
}

/// Block 4 (P9's test): a tick's whole snapshot is gated all or none in the model's bytes: with 2,000 B outstanding
/// and N = 6, a tick of a 1,000 B and a 40 B datagram passes as one (2,000 <= 5 x 1,094 B) though the 40 B one alone
/// would not (5 x 65 B); a tick the gate refuses answers each of its datagrams `BufferFull`, counted as skipped.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b4_a_tick_is_written_all_or_none() {
    let (mut l, fixed, _client) = fixed_link(14).await;
    {
        let mut f = fixed.lock().expect("inputs");
        f.srtt_ns = None;
        f.outstanding = 2_000;
    }
    assert_eq!(snapshot_budget(None, TICK_NS), 6);
    assert_eq!(l.begin_tick(&[1_000, 40], 0), SendResult::Sent);
    assert_eq!(l.send_unreliable(&[1; 1_000], 0), SendResult::Sent);
    assert_eq!(l.send_unreliable(&[1; 40], 0), SendResult::Sent);
    fixed.lock().expect("inputs").owed = 1;
    assert_eq!(l.begin_tick(&[1_000, 40], 0), SendResult::BufferFull);
    assert_eq!(l.send_unreliable(&[1; 1_000], 0), SendResult::BufferFull);
    assert_eq!(l.send_unreliable(&[1; 40], 0), SendResult::BufferFull);
    assert_eq!(
        l.gate_counts(),
        Some([1, 1, 0, 0]),
        "one written, one skipped"
    );
}

/// Block 4: a datagram outside any announced tick (the room's pings and pongs) passes fix 1's first two conditions
/// only: sent with nothing owed whatever is outstanding, refused (`BufferFull`, counted) while bytes are owed.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b4_a_datagram_outside_a_tick_passes_the_first_two_conditions_only() {
    let (mut l, fixed, _client) = fixed_link(15).await;
    fixed.lock().expect("inputs").outstanding = 60_000;
    assert_eq!(l.send_unreliable(&[1; 32], 0), SendResult::Sent, "a ping");
    fixed.lock().expect("inputs").owed = 10;
    assert_eq!(l.send_unreliable(&[1; 32], 0), SendResult::BufferFull);
    assert_eq!(l.gate_counts(), Some([0, 0, 1, 1]));
}

/// R-PING (P17's test; condition 1, rev 1.2): outside an announced tick, 33 B (above the largest Ping or Pong, 32 B
/// here) is refused and counted, 32 B with nothing owed is sent; a Ping in the same poll after an allowed tick's batch
/// is sent though that batch still waits in the link's own queue (the writer task has not run: one thread); both counts
/// read from `ws_accept`'s rows.
#[tokio::test(flavor = "current_thread")]
async fn b4_outside_a_tick_only_a_ping_or_pong_with_nothing_owed_below_the_queue_is_sent() {
    let (mut l, _fixed, _client) = fixed_link(17).await;
    assert_eq!(l.send_unreliable(&[1; 33], 0), SendResult::BufferFull);
    assert_eq!(l.send_unreliable(&[1; 32], 0), SendResult::Sent);
    // A tick counts the link's own queue as owed: the writer passes the 32 B down first.
    drained(&l).await;
    assert_eq!(l.begin_tick(&[1_000], 0), SendResult::Sent);
    assert_eq!(l.send_unreliable(&[1; 1_000], 0), SendResult::Sent);
    assert!(
        l.queued.load(Relaxed) > 1_000,
        "the batch in the link's queue"
    );
    assert_eq!(l.send_unreliable(&[1; 10], 0), SendResult::Sent, "a Ping");
    let counts = Arc::clone(&l.gate.as_ref().expect("a gate").counts);
    let rows = counts.rows(&ConnCap::new(1), false);
    let row = |k: &str| rows.iter().find(|r| r.0 == k).map(|r| r.1);
    assert_eq!(row("outside_tick_sent_count"), Some(2));
    assert_eq!(row("outside_tick_refused_count"), Some(1));
}

/// NR-U6B-4: an empty tick is no tick (not gated, not counted skipped, with bytes outstanding); a tick's datagram
/// refused `TooLarge` still takes its slot, so the next one is outside the tick and counted there.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b4_an_empty_tick_is_none_and_a_too_large_datagram_takes_its_slot() {
    let (mut l, fixed, _client) = fixed_link(19).await;
    fixed.lock().expect("inputs").outstanding = 60_000;
    assert_eq!(l.begin_tick(&[], 0), SendResult::Sent);
    fixed.lock().expect("inputs").outstanding = 0;
    assert_eq!(l.begin_tick(&[2_000], 0), SendResult::Sent);
    assert_eq!(l.send_unreliable(&[1; 2_000], 0), SendResult::TooLarge);
    assert_eq!(l.send_unreliable(&[1; 32], 0), SendResult::Sent);
    assert_eq!(l.gate_counts(), Some([1, 0, 1, 0]));
}

/// NR-U6B-2: the gate counts N in the caller's tick period (`Listen::tick_ns`, as WsServer's accept path takes it): at
/// 60 Hz (the period rounded up, as npt-server passes it), N = 10 at srtt 100 ms (6 at the 30 Hz literal it replaced).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b4_the_gate_counts_n_in_the_callers_tick_60_hz_gives_10_at_100_ms() {
    let cap = ConnCap::new(1);
    let l60 = Listen {
        tick_ns: 1_000_000_000u64.div_ceil(60),
        ..listen(&cap, 2_000)
    };
    let (l, _fixed, _client) = fixed_link_at(18, GateCfg::of(&l60)).await;
    let n = |l: &WsLink| {
        l.gate
            .as_ref()
            .expect("a gate")
            .writer
            .budget(Some(100 * MS))
    };
    assert_eq!(n(&l), 10);
    assert_eq!(n(&fixed_link(18).await.0), 6);
}

/// Condition 1's test (P16's): `begin_tick` through `Box<dyn Link + Send>` (TickLoop's sessions' type) reaches
/// WsLink's gate and sees its refusal.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b4_begin_tick_through_a_boxed_link_sees_the_gates_refusal() {
    let (l, fixed, _client) = fixed_link(16).await;
    fixed.lock().expect("inputs").owed = 1;
    let mut boxed: Box<dyn Link + Send> = Box::new(l);
    assert_eq!(boxed.begin_tick(&[100], 0), SendResult::BufferFull);
    assert_eq!(boxed.send_unreliable(&[1; 100], 0), SendResult::BufferFull);
}

/// The model's bytes of one 1,023 B datagram (1,024 B with its tag): fixes.rs's SNAP, 1,050 B.
const SNAP: u64 = 1_050;
const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;

/// One run of mechanism test (c) through WsLink: the returns, each with N at the last write before it, and the run's
/// (written, skipped, the writer's held snapshots at most).
async fn writer_run_through_ws_link(
    seed: u64,
    conn: u32,
    bite: Option<Bite>,
    at: [u64; 2],
) -> (Vec<(crate::l2::delay::Return, u64)>, [u64; 2], usize) {
    let (mut l, ds, mut client) = delay_link(&L2Config::new(seed, Profile::p2()), conn).await;
    if let Some(b) = bite {
        let g = l.gate.as_mut().expect("a gate");
        g.writer = TickWriter::new(TICK_NS).with_bite(b);
    }
    let reader = tokio::spawn(async move { while let Some(Ok(_)) = client.next().await {} });
    let start = DelayStream::now_ns();
    for b in at {
        ds.add_burst(start + b, SEC).expect("a burst");
    }
    let t0 = tokio::time::Instant::now();
    let mut n_at_write = Vec::new();
    let mut held = 0;
    let mut ticks = 0u64;
    let done = |ds: &DelayStream<()>| {
        let r = ds.returns();
        r.len() == 2
            && r.iter()
                .all(|r| r.first_byte_ns[0].is_some() && r.first_message_ns[0].is_some())
    };
    while !done(&ds) {
        assert!(ticks < 900, "hang guard");
        tokio::time::sleep_until(t0 + Duration::from_nanos(ticks * TICK_NS)).await;
        if l.begin_tick(&[1_023], 0) == SendResult::Sent {
            let srtt = l.gate.as_ref().and_then(|g| g.srtt_ns);
            n_at_write.push(snapshot_budget(srtt, TICK_NS));
            assert_eq!(l.send_unreliable(&[0x5a; 1_023], 0), SendResult::Sent);
        } else {
            assert_eq!(l.send_unreliable(&[0x5a; 1_023], 0), SendResult::BufferFull);
        }
        held = held.max(l.gate.as_ref().expect("a gate").writer.held());
        ticks += 1;
    }
    let s = ds.snapshot();
    let wrote: Vec<u64> = s
        .logs
        .steps
        .iter()
        .filter_map(|&(t, st)| {
            (st == crate::l2::delay::Step::Write {
                end: 0,
                bytes: SNAP,
            })
            .then_some(t)
        })
        .collect();
    let judged = ds
        .returns()
        .into_iter()
        .map(|r| {
            let last = wrote.iter().filter(|&&t| t < r.at_ns - s.origin).count();
            assert!(
                last > 0 && last <= n_at_write.len(),
                "a write before the return"
            );
            (r, n_at_write[last - 1])
        })
        .collect();
    let counts = l.gate_counts().expect("a gate");
    drop(l);
    reader.abort();
    (judged, [counts[0], counts[1]], held)
}

/// Block 4 (SP-2, RC-1; U3c's mechanism test (c) and writer properties (b), with the queueing bite (d), re-run through
/// WsServer's writer, `WsLink`): on DelayStream at P2 under two 1 s B1 bursts, a 1,023 B datagram a tick (1,050 B in
/// the model): at every return outstanding <= N x size, N the one in force at the last write before it; the writer
/// holds no snapshot (newest only, a skip never written later); the queueing bite holds them (fails (b)).
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn b4_the_writer_keeps_outstanding_within_n_snapshots_through_ws_link() {
    // T-4: the bursts at 0.5 s and 2.5 s keep the run inside the 5 s real-clock cap (it took 6.5 s at 1.5 s and 4.5 s).
    let at = |phase: u64| [500 * MS + phase * MS, 2_500 * MS + phase * MS];
    let (faithful, queueing) = tokio::join!(
        writer_run_through_ws_link(51, 0, None, at(0)),
        writer_run_through_ws_link(52, 1, Some(Bite::Queueing), at(11)),
    );
    let (judged, counts, held) = faithful;
    println!("(c) through WsLink: {counts:?} written/skipped, held {held}");
    assert_eq!(judged.len(), 2);
    for (r, n) in &judged {
        assert!(
            r.outstanding <= n * SNAP,
            "{} B > {n} x {SNAP} B",
            r.outstanding
        );
        assert_eq!(r.owed, 0, "nothing owed at the return");
    }
    assert!(counts[1] > 0, "the bursts made the gate skip");
    assert_eq!(held, 0, "(b): the writer holds nothing");
    assert!(queueing.2 > 0, "(d): the queueing bite holds snapshots");
}

/// A plain-path WsLink (R20) to a raw client whose socket's receive buffer is `rcvbuf` and which never reads (the
/// server's send buffer `sndbuf` when given), and the link's counts.
async fn plain_link(rcvbuf: u32, sndbuf: Option<u32>) -> (WsLink, Arc<PlainCounts>, RawWs) {
    let (chain, key, hash) = cert();
    let chain = chain.into_iter().map(CertificateDer::from).collect();
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key));
    let acceptor = ws_config::acceptor(chain, key).expect("TLS");
    let sock = tokio::net::TcpSocket::new_v4().expect("a socket");
    if let Some(b) = sndbuf {
        sock.set_send_buffer_size(b).expect("SO_SNDBUF");
    }
    sock.bind(local()).expect("bound");
    let listener = sock.listen(16).expect("listening");
    let addr = listener.local_addr().expect("its address");
    let client = tokio::spawn(raw_ws(addr, hash, Some(rcvbuf)));
    let (tcp, _) = listener.accept().await.expect("accepted");
    tcp.set_nodelay(true).expect("TCP_NODELAY");
    let (ws, source, counts) = plain(tcp, &acceptor, ws_config::ws_config(513))
        .await
        .expect("the plain path");
    let link = WsLink::start(
        ws,
        Some((source, GATE)),
        clock,
        &Arc::new(Counts::default()),
        None,
        None,
        Some(Arc::clone(&counts)),
    );
    (link, counts, client.await.expect("the client"))
}

/// Waits until the link's writer has passed its queue down to tungstenite.
async fn drained(l: &WsLink) {
    let _ = until(Duration::from_secs(2), || {
        (l.queued.load(Relaxed) == 0).then_some(())
    })
    .await;
}

/// Block 5's loopback self-test (UP-STAMP item 3; RC-12): 8 WsLink connections on DelayStream at P2 for 3 s, each with
/// its reader on its own task over the WebSocket's split halves (WsLink's), a 1,023 B datagram down a tick through fix
/// 1 and 60 B up a tick: up's woken releases carry all four parts (clock, hop, pass, reader), every reader split's
/// identity is exact, and no connection counts a violation. No timing bar.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn b5_the_loopback_self_test_has_the_four_parts_and_an_exact_identity() {
    const CONNS: u32 = 8;
    const TICKS: u64 = 90;
    let mut runs = Vec::new();
    for conn in 0..CONNS {
        runs.push(tokio::spawn(async move {
            let (mut l, ds, client) =
                delay_link(&L2Config::new(60 + u64::from(conn), Profile::p2()), conn).await;
            let (mut tx, mut rx) = client.split();
            let up = tokio::spawn(async move {
                let mut iv = tokio::time::interval(Duration::from_nanos(TICK_NS));
                for _ in 0..TICKS {
                    iv.tick().await;
                    let mut m = vec![0u8; 61];
                    m[0] = Channel::Datagram.ws_tag();
                    if tx.send(Message::Binary(m.into())).await.is_err() {
                        break;
                    }
                }
            });
            let down = tokio::spawn(async move { while let Some(Ok(_)) = rx.next().await {} });
            let mut iv = tokio::time::interval(Duration::from_nanos(TICK_NS));
            let mut got = 0u64;
            for _ in 0..TICKS + 15 {
                iv.tick().await;
                if l.begin_tick(&[1_023], 0) == SendResult::Sent {
                    l.send_unreliable(&[0x5a; 1_023], 0);
                }
                while l.poll_recv(clock()).is_some() {
                    got += 1;
                }
            }
            up.await.expect("the client's writer");
            let s = ds.snapshot();
            drop(l);
            down.abort();
            (got, s)
        }));
    }
    let mut woken = 0;
    for r in runs {
        let (got, s) = r.await.expect("a connection");
        let n = crate::l2::delay::tests::reader_split(&s)
            .unwrap_or_else(|e| panic!("the identity: {e}"));
        assert!(got > TICKS / 2, "up messages reached the room: {got}");
        assert_eq!(s.split_violations, [0, 0], "no violation");
        woken += n[0];
    }
    println!("self-test: {woken} woken up releases with all four parts over {CONNS} connections");
    assert!(woken > 0, "the four parts present");
}

/// Block 4 (SP-3): the plain path's check: a client with a small receive buffer stops reading; after every write the
/// kernel's outstanding bytes (SIOCOUTQ) are within N x size (N from the kernel's srtt read in the same tick), and
/// the writes stop.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b4_the_plain_path_keeps_siocoutq_within_n_snapshots_and_stops() {
    let (mut l, _counts, _raw) = plain_link(4_096, None).await;
    let size = model_bytes(1_023);
    let (mut written, mut stopped) = (0, 0);
    for _ in 0..200 {
        if l.begin_tick(&[1_023], 0) == SendResult::Sent {
            let n = snapshot_budget(l.gate.as_ref().and_then(|g| g.srtt_ns), TICK_NS);
            assert_eq!(l.send_unreliable(&[1; 1_023], 0), SendResult::Sent);
            written += 1;
            drained(&l).await;
            let out = l.inputs(true).expect("a gate").outstanding;
            assert!(
                out <= n * size,
                "after write {written}: SIOCOUTQ {out} > {n} x {size}"
            );
        } else {
            assert_eq!(l.send_unreliable(&[1; 1_023], 0), SendResult::BufferFull);
            stopped += 1;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    println!("plain path: {written} written, {stopped} ticks stopped");
    assert!(written > 0 && stopped > 0, "the writes stopped");
}

/// Block 4 (P10's test): on the plain path owed counts rustls's unsent TLS bytes: with the client not reading, large
/// messages fill the kernel's buffer and rustls holds TLS bytes the kernel has not taken; the gate's owed is then
/// tungstenite's untaken bytes plus rustls's (above 0) plus the link's own queue.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b4_owed_counts_rustls_unsent_bytes() {
    let (mut l, counts, _raw) = plain_link(4_096, Some(4_096)).await;
    for _ in 0..4 {
        assert_eq!(
            l.send_reliable(Channel::FullState, &vec![1; 30_000], 0),
            SendResult::Sent
        );
    }
    let mut seen = None;
    for _ in 0..50 {
        tokio::time::sleep(Duration::from_millis(20)).await;
        let read = || {
            let Some(Source::Plain { base, .. }) = l.gate.as_ref().map(|g| &g.source) else {
                unreachable!("the plain path")
            };
            let tung = counts
                .framed
                .load(Relaxed)
                .saturating_sub(counts.plain.load(Relaxed) - base[0]);
            let rustls = (counts.tls.load(Relaxed) - base[1])
                .saturating_sub(counts.sock.load(Relaxed) - base[2]);
            (tung, rustls, l.queued.load(Relaxed))
        };
        let before = read();
        let owed = l.inputs(true).expect("a gate").owed;
        if read() == before && before.1 > 0 {
            seen = Some((before, owed));
            break;
        }
    }
    let ((tung, rustls, queued), owed) =
        seen.expect("rustls held TLS bytes the kernel had not taken");
    println!("owed {owed}: tungstenite {tung}, rustls {rustls}, the queue {queued}");
    assert_eq!(owed, tung + rustls + queued);
    l.out.take();
}
