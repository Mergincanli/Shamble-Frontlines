//! The loopback routes (R5-TEST-OQ2 on real quinn): each crate's server on a quinn endpoint over ImpairSocket at P2,
//! a native client of the same crate, datagrams both ways, the injected delay in quinn's RTT, failed handshakes
//! counted by reason, and both servers in one process (M7's `--all-features` test).
//!
//! The certificate and the pinning client are `l2::tls_test`'s (moved there at U3c, shared with DelayStream's tests):
//! each feature signs and pins through its own rustls provider.

use super::super::config::L2Config;
use super::super::tls_test::{client_tls, self_signed};
use super::*;
use npt_l1::Profile;
use rustls::crypto::CryptoProvider;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

fn local() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 0))
}

fn value(list: &[(&'static str, u64)], name: &str) -> u64 {
    list.iter().find(|(n, _)| *n == name).map_or(0, |x| x.1)
}

/// What one route measured.
#[derive(Debug)]
struct Route {
    setup: Duration,
    up_received: u32,
    down_received: u32,
    server_rtt: Duration,
    client_rtt: Duration,
}

const DATAGRAMS: u32 = 200;

/// Waits for `n` datagrams or `idle` without one.
async fn count<F, Fut>(mut recv: F, idle: Duration) -> u32
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let mut got = 0;
    while got < DATAGRAMS {
        match tokio::time::timeout(idle, recv()).await {
            Ok(true) => got += 1,
            _ => break,
        }
    }
    got
}

fn check_p2(name: &str, r: &Route, s: &ImpairSocket) {
    println!("{name} at P2: {r:?}");
    assert!(
        r.setup < Duration::from_secs(10),
        "{name}: the session took {:?}",
        r.setup
    );
    // 2 % loss each way on 200 datagrams: at least 90 % arrive.
    assert!(
        r.up_received >= DATAGRAMS * 9 / 10,
        "{name}: up {}",
        r.up_received
    );
    assert!(
        r.down_received >= DATAGRAMS * 9 / 10,
        "{name}: down {}",
        r.down_received
    );
    // The injected 2 x 50 ms (± 2 x 10 ms) is in quinn's RTT on both ends; loopback adds well under 10 ms.
    for rtt in [r.server_rtt, r.client_rtt] {
        assert!(
            rtt >= Duration::from_millis(75) && rtt <= Duration::from_millis(160),
            "{name}: rtt {rtt:?}"
        );
    }
    let run = s.run_summary();
    for dir in ["up", "down"] {
        // Every QUIC packet goes through ImpairSocket (quinn may pack two datagrams queued together into one packet,
        // so packets can be fewer than datagrams: 147 packets for 197 datagrams in one run).
        let delivered = value(&run, &format!("l2_{dir}_delivered_count"));
        let received = u64::from(if dir == "up" {
            r.up_received
        } else {
            r.down_received
        });
        assert!(
            delivered >= received / 2,
            "{name} {dir}: {delivered} packets for {received} datagrams"
        );
        // Every release is a lateness sample (outbound: each try_send, whatever the kernel answered).
        let refused = value(&run, &format!("l2_{dir}_send_err_count"))
            + value(&run, &format!("l2_{dir}_would_block_count"));
        assert_eq!(
            value(&run, &format!("l2_late_{dir}_count")),
            delivered + refused
        );
    }
    let lost = value(&run, "l2_up_lost_count") + value(&run, "l2_down_lost_count");
    assert!(lost > 0, "{name}: P2's loss drew nothing over every packet");
    for k in ["queue_drop", "pool_drop", "oversize", "send_err"] {
        assert_eq!(value(&run, &format!("l2_down_{k}_count")), 0, "{name}: {k}");
    }
}

#[cfg(feature = "wt-wtransport")]
mod wtransport_route {
    use super::wtransport_server as srv;
    use super::*;

    pub(super) async fn start(
        cfg: L2Config,
    ) -> (
        Arc<ImpairSocket>,
        u16,
        CertificateDer<'static>,
        Arc<Handshakes>,
        tokio::sync::mpsc::Receiver<wtransport::Connection>,
    ) {
        let (cert, key) = self_signed(&srv::provider());
        let sock = ImpairSocket::bind(local(), cfg).unwrap();
        let ep = endpoint(
            Arc::clone(&sock),
            srv::config(vec![cert.clone()], key).unwrap(),
        )
        .unwrap();
        let port = ep.local_addr().unwrap().port();
        let counts = Arc::new(Handshakes::default());
        let (tx, rx) = tokio::sync::mpsc::channel(PENDING_SESSIONS);
        tokio::spawn(srv::accept_loop(ep, Arc::clone(&counts), tx));
        (sock, port, cert, counts, rx)
    }

    /// A native wtransport client: its session and its own port.
    pub(super) async fn client(
        port: u16,
        cert: CertificateDer<'static>,
        alpn: &[u8],
    ) -> Result<(wtransport::Connection, u16), wtransport::error::ConnectingError> {
        let tls = client_tls(srv::provider(), cert, alpn);
        let cfg = wtransport::ClientConfig::builder()
            .with_bind_address(local())
            .with_custom_tls(tls)
            .build();
        let ep = wtransport::Endpoint::client(cfg).unwrap();
        let own = ep.local_addr().unwrap().port();
        let conn = ep.connect(format!("https://127.0.0.1:{port}/npt")).await?;
        Ok((conn, own))
    }

    pub(super) async fn run(cfg: L2Config) -> (Route, Arc<ImpairSocket>, Arc<Handshakes>) {
        let (sock, port, cert, counts, mut rx) = start(cfg).await;
        let t0 = Instant::now();
        let (client, own) = client(port, cert, b"h3").await.expect("session");
        let server = rx.recv().await.expect("the session reached the room");
        let setup = t0.elapsed();
        assert_eq!(server.remote_address().port(), own, "the real peer address");
        let (s2, c2) = (server.clone(), client.clone());
        let up = tokio::spawn(async move {
            count(
                || async { s2.receive_datagram().await.is_ok() },
                Duration::from_secs(2),
            )
            .await
        });
        let down = tokio::spawn(async move {
            count(
                || async { c2.receive_datagram().await.is_ok() },
                Duration::from_secs(2),
            )
            .await
        });
        for i in 0..DATAGRAMS {
            let payload = [i as u8; 100];
            client.send_datagram(payload).unwrap();
            server.send_datagram(payload).unwrap();
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        let route = Route {
            setup,
            up_received: up.await.unwrap(),
            down_received: down.await.unwrap(),
            server_rtt: server.rtt(),
            client_rtt: client.rtt(),
        };
        (route, sock, counts)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn wtransport_serves_a_session_over_impair_socket_at_p2() {
        let cfg = L2Config::new(5, Profile::p2());
        let (r, sock, counts) = tokio::time::timeout(Duration::from_secs(30), run(cfg))
            .await
            .expect("in time");
        check_p2("wtransport", &r, &sock);
        assert_eq!(counts.accepted(), 1);
        assert_eq!(counts.failed(Failure::Quic) + counts.failed(Failure::H3), 0);
        // The clean reference through the same socket: no injected delay in the RTT.
        let (clean, _, _) = tokio::time::timeout(Duration::from_secs(30), run(L2Config::clean(5)))
            .await
            .expect("in time");
        println!("wtransport pass-through: {clean:?}");
        assert!(clean.server_rtt < Duration::from_millis(20), "{clean:?}");
        assert_eq!(
            (clean.up_received, clean.down_received),
            (DATAGRAMS, DATAGRAMS)
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn wtransport_counts_a_failed_handshake() {
        let (_sock, port, cert, counts, _rx) = start(L2Config::clean(6)).await;
        // A client offering the wrong ALPN: the TLS handshake fails (no_application_protocol).
        let r = tokio::time::timeout(Duration::from_secs(10), client(port, cert, b"h2"))
            .await
            .expect("in time");
        assert!(r.is_err());
        let end = Instant::now() + Duration::from_secs(5);
        while counts.failed(Failure::Quic) == 0 && Instant::now() < end {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        assert_eq!((counts.failed(Failure::Quic), counts.accepted()), (1, 0));
    }

    /// M8's reasons: a client that completes QUIC's handshake (ALPN h3) and then closes the connection at the
    /// application level, before HTTP/3's SETTINGS and CONNECT, is an HTTP/3-stage failure. wtransport reports it as
    /// ConnectionError::ApplicationClosed (error.rs:16-18, 312-316). The bite: counting that variant as the QUIC stage.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn wtransport_counts_a_close_after_quic_as_h3() {
        let (_sock, port, cert, counts, _rx) = start(L2Config::clean(7)).await;
        let tls = client_tls(srv::provider(), cert, b"h3");
        let crypto = quinn::crypto::rustls::QuicClientConfig::try_from(tls).unwrap();
        let ep = quinn::Endpoint::client(local()).unwrap();
        let server = SocketAddr::from(([127, 0, 0, 1], port));
        let conn = tokio::time::timeout(
            Duration::from_secs(10),
            ep.connect_with(
                quinn::ClientConfig::new(Arc::new(crypto)),
                server,
                "127.0.0.1",
            )
            .unwrap(),
        )
        .await
        .expect("in time")
        .expect("QUIC's handshake completes");
        // Let the server reach its HTTP/3 stage, then close without SETTINGS or CONNECT.
        tokio::time::sleep(Duration::from_millis(100)).await;
        conn.close(0u32.into(), b"gone");
        let end = Instant::now() + Duration::from_secs(5);
        while counts.failed(Failure::H3) + counts.failed(Failure::Quic) == 0 && Instant::now() < end
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        assert_eq!(
            (
                counts.failed(Failure::H3),
                counts.failed(Failure::Quic),
                counts.accepted()
            ),
            (1, 0, 0)
        );
        ep.wait_idle().await;
    }
}

#[cfg(feature = "wt-wtq")]
mod wtq_route {
    use super::wtq_server as srv;
    use super::*;

    pub(super) async fn start(
        cfg: L2Config,
    ) -> (
        Arc<ImpairSocket>,
        u16,
        CertificateDer<'static>,
        Arc<Handshakes>,
        tokio::sync::mpsc::Receiver<web_transport_quinn::Session>,
    ) {
        let (cert, key) = self_signed(&srv::provider());
        let sock = ImpairSocket::bind(local(), cfg).unwrap();
        let ep = endpoint(
            Arc::clone(&sock),
            srv::config(vec![cert.clone()], key).unwrap(),
        )
        .unwrap();
        let port = ep.local_addr().unwrap().port();
        let counts = Arc::new(Handshakes::default());
        let (tx, rx) = tokio::sync::mpsc::channel(PENDING_SESSIONS);
        tokio::spawn(srv::accept_loop(ep, Arc::clone(&counts), tx));
        (sock, port, cert, counts, rx)
    }

    /// The URL as the crate's own type, named through ConnectRequest's field (url is not a dependency here).
    fn url<T: std::str::FromStr>(
        s: &str,
        _field: impl Fn(&web_transport_quinn::proto::ConnectRequest) -> &T,
    ) -> T
    where
        T::Err: std::fmt::Debug,
    {
        s.parse().expect("a URL")
    }

    /// A native web-transport-quinn client on its own quinn endpoint: its session and its own port.
    pub(super) async fn client(
        port: u16,
        cert: CertificateDer<'static>,
        alpn: &[u8],
    ) -> Result<(web_transport_quinn::Session, u16), web_transport_quinn::ClientError> {
        let tls = client_tls(srv::provider(), cert, alpn);
        let crypto = quinn::crypto::rustls::QuicClientConfig::try_from(tls).unwrap();
        let ep = quinn::Endpoint::client(local()).unwrap();
        let own = ep.local_addr().unwrap().port();
        let client =
            web_transport_quinn::Client::new(ep, quinn::ClientConfig::new(Arc::new(crypto)));
        let u = url(&format!("https://127.0.0.1:{port}/npt"), |r| &r.url);
        let session = client
            .connect(web_transport_quinn::proto::ConnectRequest::new(u))
            .await?;
        Ok((session, own))
    }

    pub(super) async fn run(cfg: L2Config) -> (Route, Arc<ImpairSocket>, Arc<Handshakes>) {
        let (sock, port, cert, counts, mut rx) = start(cfg).await;
        let t0 = Instant::now();
        let (client, own) = client(port, cert, b"h3").await.expect("session");
        let server = rx.recv().await.expect("the session reached the room");
        let setup = t0.elapsed();
        assert_eq!(server.remote_address().port(), own, "the real peer address");
        let (s2, c2) = (server.clone(), client.clone());
        let up = tokio::spawn(async move {
            count(
                || async { s2.read_datagram().await.is_ok() },
                Duration::from_secs(2),
            )
            .await
        });
        let down = tokio::spawn(async move {
            count(
                || async { c2.read_datagram().await.is_ok() },
                Duration::from_secs(2),
            )
            .await
        });
        for i in 0..DATAGRAMS {
            let payload = vec![i as u8; 100];
            client.send_datagram(payload.clone().into()).unwrap();
            server.send_datagram(payload.into()).unwrap();
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        let route = Route {
            setup,
            up_received: up.await.unwrap(),
            down_received: down.await.unwrap(),
            server_rtt: server.rtt(),
            client_rtt: client.rtt(),
        };
        (route, sock, counts)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn web_transport_quinn_serves_a_session_over_impair_socket_at_p2() {
        let cfg = L2Config::new(5, Profile::p2());
        let (r, sock, counts) = tokio::time::timeout(Duration::from_secs(30), run(cfg))
            .await
            .expect("in time");
        check_p2("web-transport-quinn", &r, &sock);
        assert_eq!(counts.accepted(), 1);
        assert_eq!(counts.failed(Failure::Quic) + counts.failed(Failure::H3), 0);
        let (clean, _, _) = tokio::time::timeout(Duration::from_secs(30), run(L2Config::clean(5)))
            .await
            .expect("in time");
        println!("web-transport-quinn pass-through: {clean:?}");
        assert!(clean.server_rtt < Duration::from_millis(20), "{clean:?}");
        assert_eq!(
            (clean.up_received, clean.down_received),
            (DATAGRAMS, DATAGRAMS)
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn web_transport_quinn_counts_a_failed_handshake() {
        let (_sock, port, cert, counts, _rx) = start(L2Config::clean(6)).await;
        let r = tokio::time::timeout(Duration::from_secs(10), client(port, cert, b"h2"))
            .await
            .expect("in time");
        assert!(r.is_err());
        let end = Instant::now() + Duration::from_secs(5);
        while counts.failed(Failure::Quic) == 0 && Instant::now() < end {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        assert_eq!((counts.failed(Failure::Quic), counts.accepted()), (1, 0));
    }
}

/// M7: an `--all-features` build starts both servers in one process: each feature's own provider, no process
/// default, so no dual-provider panic; a session and a datagram each way on both.
#[cfg(all(feature = "wt-wtransport", feature = "wt-wtq"))]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn both_servers_run_in_one_process() {
    assert!(
        CryptoProvider::get_default().is_none(),
        "no process default is installed"
    );
    let both = async {
        let (a, b) = tokio::join!(
            wtransport_route::run(L2Config::clean(8)),
            wtq_route::run(L2Config::clean(9))
        );
        (a.0, b.0)
    };
    let (a, b) = tokio::time::timeout(Duration::from_secs(30), both)
        .await
        .expect("in time");
    println!("one process: wtransport {a:?}; web-transport-quinn {b:?}");
    assert_eq!(
        (
            a.up_received,
            a.down_received,
            b.up_received,
            b.down_received
        ),
        (DATAGRAMS, DATAGRAMS, DATAGRAMS, DATAGRAMS)
    );
    assert!(
        CryptoProvider::get_default().is_none(),
        "still none: each crate used its own"
    );
}

/// M7's one-process check extended to the WebSocket server (u3c-plan.md 1.5): both WebTransport servers and the
/// WebSocket server (TLS 1.3 through tokio-rustls on ring, DelayStream at P1, tungstenite) in one process, each with
/// its own provider and none as the process default; a session each, messages both ways on all three.
#[cfg(all(feature = "wt-wtransport", feature = "wt-wtq"))]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn all_three_servers_run_in_one_process() {
    assert!(CryptoProvider::get_default().is_none());
    let all = async {
        tokio::join!(
            wtransport_route::run(L2Config::clean(8)),
            wtq_route::run(L2Config::clean(9)),
            super::super::delay::tests::ws_session(L2Config::new(10, Profile::p1()), 30)
        )
    };
    let (a, b, ws) = tokio::time::timeout(Duration::from_secs(60), all)
        .await
        .expect("in time");
    println!(
        "one process: wtransport {:?}; web-transport-quinn {:?}; WebSocket {ws:?}",
        a.0, b.0
    );
    assert_eq!(
        (
            a.0.up_received,
            a.0.down_received,
            b.0.up_received,
            b.0.down_received
        ),
        (DATAGRAMS, DATAGRAMS, DATAGRAMS, DATAGRAMS)
    );
    assert_eq!((ws.0, ws.1), (30, 30), "the WebSocket session, both ways");
    assert!(ws.2.starts_with("Some(TLSv1_3)"));
    assert!(
        CryptoProvider::get_default().is_none(),
        "still none: each server used its own"
    );
}

/// M9 and M8 in the configurations each feature builds: migration off, datagrams on (quinn's Debug shows both).
#[test]
fn the_server_configs_turn_migration_off_and_keep_datagrams_on() {
    let mut built = Vec::new();
    #[cfg(feature = "wt-wtransport")]
    {
        let (cert, key) = self_signed(&wtransport_server::provider());
        built.push((
            "wtransport",
            wtransport_server::config(vec![cert], key).unwrap().1,
        ));
    }
    #[cfg(feature = "wt-wtq")]
    {
        let (cert, key) = self_signed(&wtq_server::provider());
        built.push((
            "web-transport-quinn",
            wtq_server::config(vec![cert], key).unwrap().1,
        ));
    }
    assert!(!built.is_empty());
    for (name, sc) in built {
        let d = format!("{sc:?}");
        assert!(d.contains("migration: false"), "{name}: {d}");
        assert!(
            d.contains("datagram_receive_buffer_size: Some("),
            "{name}: datagrams disabled"
        );
    }
}

#[test]
fn the_test_certificate_is_well_formed() {
    let p = rustls::crypto::ring::default_provider();
    let (cert, key) = self_signed(&p);
    // rustls parses it and matches the key to it (CertifiedKey's keys_match), with TLS 1.3 and the QUIC suite.
    let tls = tls_config(p, vec![cert], key).expect("a server config");
    assert_eq!(tls.alpn_protocols, vec![b"h3".to_vec()]);
    // A provider without the QUIC suite is refused (wtransport's build() would panic on it).
    let mut thin = rustls::crypto::ring::default_provider();
    thin.cipher_suites
        .retain(|s| s.suite() != rustls::CipherSuite::TLS13_AES_128_GCM_SHA256);
    let (cert, key) = self_signed(&thin);
    assert!(tls_config(thin, vec![cert], key).is_err());
    #[cfg(feature = "wt-wtq")]
    {
        let p = rustls::crypto::aws_lc_rs::default_provider();
        let (cert, key) = self_signed(&p);
        assert!(
            tls_config(p, vec![cert], key).is_ok(),
            "aws-lc-rs takes it too"
        );
    }
}
