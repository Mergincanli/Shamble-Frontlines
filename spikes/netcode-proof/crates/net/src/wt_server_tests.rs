//! Block 2's tests (u6b-brief.md section 3): WtServer and WtClient on 127.0.0.1, the test certificate of `tls_test`,
//! real-clock waits of at most 5 s each and no timing bar.

use super::*;
use crate::cap::ConnCap;
use crate::l2::tls_test::self_signed;
use npt_l1::Profile;
use std::sync::OnceLock;
use std::time::Instant;

/// The tests' room clock: ns since its first read.
pub(crate) fn clock() -> u64 {
    static T0: OnceLock<Instant> = OnceLock::new();
    T0.get_or_init(Instant::now).elapsed().as_nanos() as u64
}

pub(crate) fn local() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 0))
}

/// The test certificate: its chain, its key (both DER) and its SHA-256.
pub(crate) fn cert() -> (Vec<Vec<u8>>, Vec<u8>, [u8; 32]) {
    let (c, k) = self_signed(&wtq_server::provider());
    let hash = crate::pin::sha256(c.as_ref());
    let PrivateKeyDer::Pkcs8(k) = k else {
        unreachable!("PKCS#8")
    };
    (vec![c.to_vec()], k.secret_pkcs8_der().to_vec(), hash)
}

pub(crate) fn listen(cap: &Arc<ConnCap>, deadline_ms: u64) -> Listen {
    Listen {
        addr: local(),
        clock,
        deadline_ns: deadline_ms * 1_000_000,
        cap: Arc::clone(cap),
        tick_ns: 33_333_333,
        outside_max: 32,
    }
}

/// A started server at `cfg` with `cc`, and its port and certificate hash.
pub(crate) fn server(
    cfg: L2Config,
    cap: &Arc<ConnCap>,
    cc: &str,
    deadline_ms: u64,
) -> (WtServer, SocketAddr, [u8; 32]) {
    let (chain, key, hash) = cert();
    let s = WtServer::bind(listen(cap, deadline_ms), &cfg, &chain, &key, cc).expect("bound");
    s.start().expect("started");
    let addr = s.local_addr().expect("its address");
    (s, addr, hash)
}

/// Polls `f` every millisecond until it gives `Some`, at most `limit`.
pub(crate) async fn until<T>(limit: Duration, mut f: impl FnMut() -> Option<T>) -> Option<T> {
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

/// A client connected to `addr` and the server's side of its session, both open.
pub(crate) async fn pair(s: &WtServer, addr: SocketAddr, hash: [u8; 32]) -> (WtClient, Accepted) {
    let mut c = WtClient::connect(addr, hash, clock).expect("a runtime");
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

/// The next message on `l`, within `limit`.
pub(crate) async fn next<L: Link + ?Sized>(l: &mut L, limit: Duration) -> Option<Recv> {
    until(limit, || l.poll_recv(clock())).await
}

/// Block 2: every channel the room uses both ways with its framing: the server's control message, its FullState (one
/// unidirectional stream, its 4 B length) and a datagram reach the client; the client's control message and datagram
/// reach the server; each byte for byte, each on its own channel.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b2_every_channel_both_ways_with_its_framing() {
    let cap = ConnCap::new(4);
    let (s, addr, hash) = server(L2Config::clean(1), &cap, "cubic", 2_000);
    let (mut c, mut a) = pair(&s, addr, hash).await;
    let control = [3u8, 0, 9, 1, 2];
    let mut full = vec![0u8; 4];
    full.extend((0..20_000u32).map(|i| i as u8));
    full[..4].copy_from_slice(&20_000u32.to_le_bytes());
    assert_eq!(
        c.send_reliable(Channel::Control, &control, 0),
        SendResult::Sent
    );
    assert_eq!(c.send_unreliable(b"up", 0), SendResult::Sent);
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
            (Channel::Datagram, b"up".to_vec()),
            (Channel::Control, control.to_vec())
        ]
    );
    assert_eq!(
        a.link.send_reliable(Channel::Control, &control, 0),
        SendResult::Sent
    );
    assert_eq!(
        a.link.send_reliable(Channel::FullState, &full, 0),
        SendResult::Sent
    );
    assert_eq!(a.link.send_unreliable(b"down", 0), SendResult::Sent);
    let mut got = Vec::new();
    while got.len() < 3 {
        let r = next(&mut c, Duration::from_secs(5)).await.expect("down");
        got.push((r.channel, r.bytes));
    }
    got.sort();
    assert_eq!(
        got,
        vec![
            (Channel::Datagram, b"down".to_vec()),
            (Channel::Control, control.to_vec()),
            (Channel::FullState, full),
        ]
    );
    assert!(a.link.max_datagram_size() > 1_000 && a.link.rtt_ns().is_some());
}

/// A raw web-transport-quinn session to `addr` (no framing of ours) and its control stream.
async fn raw(addr: SocketAddr, hash: [u8; 32]) -> (Session, SendStream) {
    let tls = crate::pin::client_config(wtq_server::provider(), hash, b"h3").expect("TLS");
    let crypto = quinn::crypto::rustls::QuicClientConfig::try_from(tls).expect("QUIC");
    let ep = quinn::Endpoint::client(local()).expect("an endpoint");
    let client = web_transport_quinn::Client::new(ep, quinn::ClientConfig::new(Arc::new(crypto)));
    let u = url(&format!("https://{addr}/npt"), |r| &r.url).expect("a URL");
    let session = client
        .connect(web_transport_quinn::proto::ConnectRequest::new(u))
        .await
        .expect("a session");
    let (send, _recv) = session.open_bi().await.expect("the control stream");
    (session, send)
}

/// Block 2: a control message whose 2 B length says above 512 B closes the session as malformed, counted, before its
/// body is read.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b2_a_control_length_above_512_b_closes_the_session_as_malformed() {
    let cap = ConnCap::new(4);
    let (s, addr, hash) = server(L2Config::clean(2), &cap, "cubic", 2_000);
    let (session, mut send) = raw(addr, hash).await;
    let mut a = tokio::time::timeout(Duration::from_secs(5), s.accept())
        .await
        .expect("in time")
        .expect("a session");
    send.write_all(&511u16.to_le_bytes())
        .await
        .expect("written");
    send.write_all(&[1u8; 8]).await.expect("written");
    let closed = until(Duration::from_secs(5), || {
        (a.link.state() == LinkState::Closed).then_some(())
    })
    .await;
    assert!(closed.is_some(), "closed");
    assert!(
        a.link.poll_recv(clock()).is_none(),
        "nothing reached the room"
    );
    assert_eq!(s.event_rows()[4], ("malformed_closed_count", 1));
    let e = tokio::time::timeout(Duration::from_secs(5), session.closed())
        .await
        .expect("the client saw the close");
    println!("the client's view of the close: {e}");
}

/// Block 2: `BufferFull` when quinn's datagram send buffer is short of the datagram (ADR 0001:439-442), before quinn
/// would drop older datagrams to take it; one that fits is sent.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b2_buffer_full_when_quinn_has_no_datagram_space() {
    let cap = ConnCap::new(4);
    let (chain, key, hash) = cert();
    let s = WtServer::bind_with(
        listen(&cap, 2_000),
        &L2Config::clean(3),
        &chain,
        &key,
        transport("cubic", 500).expect("a transport"),
    )
    .expect("bound");
    s.start().expect("started");
    let addr = s.local_addr().expect("its address");
    let (_c, mut a) = pair(&s, addr, hash).await;
    assert!(a.link.max_datagram_size() > 600);
    assert_eq!(a.link.send_unreliable(&[7; 600], 0), SendResult::BufferFull);
    assert_eq!(a.link.send_unreliable(&[7; 100], 0), SendResult::Sent);
    assert_eq!(
        a.link
            .send_unreliable(&vec![7; a.link.max_datagram_size() + 1], 0),
        SendResult::TooLarge
    );
}

/// Block 2: a client offering ALPN h2 fails the handshake: one failure counted, no session.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b2_one_failure_is_counted_for_a_wrong_alpn() {
    let cap = ConnCap::new(4);
    let (s, addr, hash) = server(L2Config::clean(4), &cap, "cubic", 2_000);
    let tls = crate::pin::client_config(wtq_server::provider(), hash, b"h2").expect("TLS");
    let crypto = quinn::crypto::rustls::QuicClientConfig::try_from(tls).expect("QUIC");
    let ep = quinn::Endpoint::client(local()).expect("an endpoint");
    let r = ep
        .connect_with(
            quinn::ClientConfig::new(Arc::new(crypto)),
            addr,
            "localhost",
        )
        .expect("connecting")
        .await;
    assert!(r.is_err(), "no ALPN in common");
    let counted = until(Duration::from_secs(5), || {
        (s.event_rows()[1].1 == 1).then_some(())
    })
    .await;
    assert!(counted.is_some(), "{:?}", s.event_rows());
    assert_eq!(s.event_rows()[0], ("accepted_count", 0));
    assert_eq!(cap.free(), cap.permits(), "the permit came back");
}

/// Block 2 (P3's test): the deadline covers QUIC and HTTP/3 alike. A client that completes QUIC's handshake (ALPN h3)
/// and never sends HTTP/3's SETTINGS or CONNECT is closed at the deadline passed in and counted; the elapsed time is at
/// least the deadline (no upper bar).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b2_the_deadline_covers_http3_a_client_that_never_sends_connect_is_closed() {
    const DEADLINE_MS: u64 = 300;
    let cap = ConnCap::new(4);
    let (s, addr, hash) = server(L2Config::clean(5), &cap, "cubic", DEADLINE_MS);
    let tls = crate::pin::client_config(wtq_server::provider(), hash, b"h3").expect("TLS");
    let crypto = quinn::crypto::rustls::QuicClientConfig::try_from(tls).expect("QUIC");
    let ep = quinn::Endpoint::client(local()).expect("an endpoint");
    let t0 = Instant::now();
    let conn = ep
        .connect_with(
            quinn::ClientConfig::new(Arc::new(crypto)),
            addr,
            "localhost",
        )
        .expect("connecting")
        .await
        .expect("QUIC's handshake completes");
    let e = tokio::time::timeout(Duration::from_secs(5), conn.closed())
        .await
        .expect("closed by the server in time");
    let took = t0.elapsed();
    println!("closed after {took:?}: {e}");
    assert!(took >= Duration::from_millis(DEADLINE_MS), "{took:?}");
    assert_eq!(s.event_rows()[3], ("deadline_closed_count", 1));
    assert_eq!(s.event_rows()[0].1 + s.event_rows()[1].1, 0);
}

/// Block 2 (P5's test; U6a's carry): `Kick(replaced)`'s bytes, queued just before the server's link is dropped, are
/// read by the client before the close, 20 times at once on ImpairSocket at P2 with 5 % loss. Delivery only: no press
/// or shot is counted (C-U6-19's line).
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn b2_a_kick_queued_before_the_drop_is_read_before_the_close() {
    const RUNS: usize = 20;
    let mut p = Profile::p2();
    p.loss_ppm = 50_000;
    let cap = ConnCap::new(RUNS);
    let (s, addr, hash) = server(L2Config::new(6, p), &cap, "cubic", 4_000);
    let kick = [3u8, 0, 12, 0, 0];
    let mut pairs = Vec::new();
    for _ in 0..RUNS {
        pairs.push(WtClient::connect(addr, hash, clock).expect("a runtime"));
    }
    let mut servers = Vec::new();
    for _ in 0..RUNS {
        let a = tokio::time::timeout(Duration::from_secs(5), s.accept())
            .await
            .expect("in time")
            .expect("a session");
        servers.push(a);
    }
    // Each client sends once so its server side has its control stream, then the server queues the kick and drops.
    for c in &mut pairs {
        let up = until(Duration::from_secs(5), || {
            (c.send_reliable(Channel::Control, &[3, 0, 9, 0, 0], clock()) == SendResult::Sent)
                .then_some(())
        })
        .await;
        assert!(up.is_some(), "the client opened");
    }
    for a in &mut servers {
        assert!(next(&mut *a.link, Duration::from_secs(5)).await.is_some());
        assert_eq!(
            a.link.send_reliable(Channel::Control, &kick, clock()),
            SendResult::Sent
        );
    }
    drop(servers);
    let reads = pairs.into_iter().map(|mut c| {
        tokio::spawn(async move {
            let got = next(&mut c, Duration::from_secs(4)).await;
            (got.map(|r| r.bytes), c.state())
        })
    });
    let mut kicked = 0;
    for r in reads {
        let (got, _) = r.await.expect("a reader");
        kicked += usize::from(got.as_deref() == Some(&kick[..]));
    }
    assert_eq!(kicked, RUNS, "every client read the kick before the close");
}

/// T-2's WebTransport half (Block 1's stamp): the accept stamp is the injected clock's at the first QUIC packet: the
/// clock moves from 500 to 700 after QUIC's handshake and before HTTP/3's SETTINGS and CONNECT, and the session carries
/// 500 (no QUIC-stage hook needed: web-transport-quinn's `Session::connect` takes the client's own QUIC connection).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b2_the_accept_stamp_is_the_first_packets_never_the_answers() {
    static NOW: AtomicU64 = AtomicU64::new(500);
    fn now() -> u64 {
        NOW.load(std::sync::atomic::Ordering::SeqCst)
    }
    let cap = ConnCap::new(2);
    let (chain, key, hash) = cert();
    let l = Listen {
        clock: now,
        ..listen(&cap, 2_000)
    };
    let s = WtServer::bind(l, &L2Config::clean(9), &chain, &key, "cubic").expect("bound");
    s.start().expect("started");
    let addr = s.local_addr().expect("its address");
    let tls = crate::pin::client_config(wtq_server::provider(), hash, b"h3").expect("TLS");
    let crypto = quinn::crypto::rustls::QuicClientConfig::try_from(tls).expect("QUIC");
    let ep = quinn::Endpoint::client(local()).expect("an endpoint");
    let conn = ep
        .connect_with(
            quinn::ClientConfig::new(Arc::new(crypto)),
            addr,
            "localhost",
        )
        .expect("connecting")
        .await
        .expect("QUIC's handshake");
    NOW.store(700, std::sync::atomic::Ordering::SeqCst);
    let u = url(&format!("https://{addr}/npt"), |r| &r.url).expect("a URL");
    let request = web_transport_quinn::proto::ConnectRequest::new(u);
    let _session = web_transport_quinn::Session::connect(conn, request)
        .await
        .expect("a session");
    let a = tokio::time::timeout(Duration::from_secs(5), s.accept())
        .await
        .expect("in time")
        .expect("a session");
    assert_eq!(
        a.accept_ns, 500,
        "the accept stamp, at the first QUIC packet"
    );
}

/// SEC-U6B-1 (P18's test; T-13): a peer that never reads its control stream (so grants it no credit past its 256 B
/// stream window) and keeps its connection alive; the server queues 2 x 500 B on it and drops the link: the closing is
/// cut at its bound from the drop, and the established session's permit comes back (within 4 s).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b2_a_peer_that_grants_no_credit_gets_its_permit_back_within_the_bound() {
    let cap = ConnCap::new(1);
    let (s, addr, hash) = server(L2Config::clean(7), &cap, "cubic", 2_000);
    let tls = crate::pin::client_config(wtq_server::provider(), hash, b"h3").expect("TLS");
    let crypto = quinn::crypto::rustls::QuicClientConfig::try_from(tls).expect("QUIC");
    let mut t = quinn::TransportConfig::default();
    t.stream_receive_window(256u32.into())
        .keep_alive_interval(Some(Duration::from_millis(100)));
    let mut config = quinn::ClientConfig::new(Arc::new(crypto));
    config.transport_config(Arc::new(t));
    let ep = quinn::Endpoint::client(local()).expect("an endpoint");
    let u = url(&format!("https://{addr}/npt"), |r| &r.url).expect("a URL");
    let session = web_transport_quinn::Client::new(ep, config)
        .connect(web_transport_quinn::proto::ConnectRequest::new(u))
        .await
        .expect("a session");
    let (mut send, _recv) = session.open_bi().await.expect("the control stream");
    send.write_all(&[3, 0, 9, 0, 0]).await.expect("written");
    let mut a = tokio::time::timeout(Duration::from_secs(5), s.accept())
        .await
        .expect("in time")
        .expect("a session");
    assert!(next(&mut *a.link, Duration::from_secs(5)).await.is_some());
    for _ in 0..2 {
        let r = a.link.send_reliable(Channel::Control, &[1; 500], clock());
        assert_eq!(r, SendResult::Sent);
    }
    assert_eq!(cap.free(), cap.permits() - 1);
    let t0 = Instant::now();
    drop(a);
    let back = until(Duration::from_secs(4), || {
        (cap.free() == cap.permits()).then_some(())
    })
    .await;
    println!("the permit back after {:?}", t0.elapsed());
    assert!(back.is_some(), "the permit back within the bound");
    drop((send, session));
}

/// Block 2: the run's `cc` reaches quinn: a session accepted from a server bound with `newreno` runs NewReno, one with
/// `cubic` Cubic (quinn's controller read back on the live connection); any other name is refused.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b2_cc_reaches_quinn() {
    let cap = ConnCap::new(4);
    let (chain, key, hash) = cert();
    assert!(
        WtServer::bind(
            listen(&cap, 2_000),
            &L2Config::clean(7),
            &chain,
            &key,
            "bbr"
        )
        .is_err()
    );
    for cc in ["newreno", "cubic"] {
        let s = WtServer::bind(listen(&cap, 2_000), &L2Config::clean(7), &chain, &key, cc)
            .expect("bound");
        let configs = s
            .configs
            .lock()
            .expect("configs")
            .take()
            .expect("not started");
        let ep = endpoint(Arc::clone(&s.socket), configs).expect("an endpoint");
        let addr = ep.local_addr().expect("its address");
        let server = tokio::spawn(async move {
            let incoming = ep.accept().await.expect("an incoming");
            let conn = incoming.await.expect("QUIC");
            let req = web_transport_quinn::Request::accept(conn)
                .await
                .expect("HTTP/3");
            let session = req.ok().await.expect("answered");
            let state = session.congestion_state().into_any();
            (
                state.is::<quinn::congestion::NewReno>(),
                state.is::<quinn::congestion::Cubic>(),
                session,
            )
        });
        let (_session, _send) = raw(addr, hash).await;
        let (newreno, cubic, _s) = server.await.expect("the server");
        assert_eq!((newreno, cubic), (cc == "newreno", cc == "cubic"), "{cc}");
    }
}

/// SEC-U6B-2: the four stream limits, read off the bound server's own QUIC config (quinn's Debug of its
/// TransportConfig); WtClient's transport keeps quinn's defaults.
#[test]
fn b2_the_servers_quic_config_bounds_its_streams_and_windows() {
    let cap = ConnCap::new(4);
    let (chain, key, _) = cert();
    let s = WtServer::bind(
        listen(&cap, 2_000),
        &L2Config::clean(8),
        &chain,
        &key,
        "cubic",
    )
    .expect("bound");
    let configs = s.configs.lock().expect("configs");
    let t = format!("{:?}", configs.as_ref().expect("not started").1.transport);
    for (k, v) in [
        ("max_concurrent_bidi_streams", 4),
        ("max_concurrent_uni_streams", 6),
        ("stream_receive_window", 65_536),
        ("receive_window", 1_048_576),
    ] {
        assert!(t.contains(&format!(" {k}: {v},")), "{k} {v}: {t}");
    }
    let c = format!("{:?}", transport("cubic", DATAGRAM_BUFFER).expect("cubic"));
    assert!(c.contains(" max_concurrent_bidi_streams: 100,"), "{c}");
}

/// Block 2: a client is `Connecting` until its session opens, and its sends are answered `BufferFull` then.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b2_the_client_is_connecting_until_its_session_opens() {
    let cap = ConnCap::new(4);
    let (s, addr, hash) = server(L2Config::new(8, Profile::p1()), &cap, "cubic", 2_000);
    let mut c = WtClient::connect(addr, hash, clock).expect("a runtime");
    assert_eq!(c.state(), LinkState::Connecting);
    assert_eq!(c.send_unreliable(b"x", 0), SendResult::BufferFull);
    assert_eq!(
        c.send_reliable(Channel::Control, b"x", 0),
        SendResult::BufferFull
    );
    assert_eq!(c.max_datagram_size(), 0);
    let _a = s.accept().await.expect("a session");
    let open = until(Duration::from_secs(5), || {
        c.poll_recv(clock());
        (c.state() == LinkState::Open).then_some(())
    })
    .await;
    assert!(open.is_some());
    assert_eq!(c.send_unreliable(b"x", 0), SendResult::Sent);
}

/// Block 2 (term (2)): with both WebTransport features on, WtServer is web-transport-quinn's on ring and installs no
/// process default: a WtClient session opens through it.
#[cfg(all(feature = "wt-wtransport", feature = "wt-wtq"))]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn b2_both_features_on_give_web_transport_quinn_on_ring() {
    let cap = ConnCap::new(4);
    let (s, addr, hash) = server(L2Config::clean(9), &cap, "cubic", 2_000);
    let (_c, _a) = pair(&s, addr, hash).await;
    assert!(quinn::rustls::crypto::CryptoProvider::get_default().is_none());
    let ring = quinn::rustls::crypto::ring::default_provider();
    let suites = |p: &quinn::rustls::crypto::CryptoProvider| {
        p.cipher_suites
            .iter()
            .map(|s| s.suite())
            .collect::<Vec<_>>()
    };
    assert_eq!(suites(&wtq_server::provider()), suites(&ring));
}
