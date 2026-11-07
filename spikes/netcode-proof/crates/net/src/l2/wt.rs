//! Each WebTransport crate's server on our own quinn endpoint over ImpairSocket (R5-TEST-OQ2: yes for both crates;
//! the advisor's M7, M8, M9). Used by the loopback tests here and by WtServer at U6.
//!
//! - **TLS (M7):** each feature builds its own rustls server configuration with `builder_with_provider` and its own
//!   crate's provider (wtransport: ring, its default; web-transport-quinn: aws-lc-rs, its default), never the process
//!   default, so an `--all-features` build has no dual-provider panic; TLS 1.3 only; ALPN `h3` set explicitly
//!   (wtransport's `with_custom_tls*` sets none); TLS13_AES_128_GCM_SHA256 checked present (quinn's QUIC initial
//!   suite, which wtransport's `build()` expects). The wt-wtq path never touches wtransport.
//! - **Transport:** quinn's defaults, as each crate's own builder has them, datagrams enabled (wtransport panics on
//!   `SendDatagramError::Disabled`: M8); migration off in both (M9; `super::MIGRATION`).
//! - **Accept loop (M8):** we drive `quinn::Endpoint::accept` ourselves and call each crate's per-connection accept,
//!   so every failed handshake is counted by its reason (web-transport-quinn's own `Server` drops them silently; E6
//!   counts them). Sessions come out of a bounded channel.
//! - **Carried to U6, before WtServer reuses this file:** the accept loops start one task per `Incoming` with no cap
//!   on concurrent handshakes once QUIC is up, and no deadline on the HTTP/3 stage (wtransport awaits SETTINGS and
//!   CONNECT, endpoint.rs:621-634; quinn's 30 s idle timeout is defeated by a client's keep-alives); quinn bounds only
//!   the `Incoming` queue (max_incoming). Both must be bounded there (.claude/rules/netcode.md: nothing unbounded keyed
//!   by client data), and a task blocked on `tx.send` when the channel is full needs the same deadline. Fine for the
//!   loopback tests here.

use super::MIGRATION;
use super::socket::ImpairSocket;
use quinn::rustls;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Why a handshake failed (M8), each its own counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Failure {
    /// The QUIC or TLS handshake (a wrong ALPN, a timeout, a peer's close before the connection was up).
    Quic = 0,
    /// The HTTP/3 SETTINGS or CONNECT exchange: no session request formed.
    H3 = 1,
    /// Answering the session request.
    Accept = 2,
}

/// Handshakes per outcome (E6's count).
#[derive(Debug, Default)]
pub(crate) struct Handshakes {
    pub(crate) accepted: AtomicU64,
    failed: [AtomicU64; 3],
}

impl Handshakes {
    pub(crate) fn fail(&self, why: Failure) {
        self.failed[why as usize].fetch_add(1, Ordering::Relaxed);
    }
    pub(crate) fn failed(&self, why: Failure) -> u64 {
        self.failed[why as usize].load(Ordering::Relaxed)
    }
    pub(crate) fn accepted(&self) -> u64 {
        self.accepted.load(Ordering::Relaxed)
    }
}

/// Sessions waiting for the room, at most this many (the channel is bounded: nothing grows with client data).
pub(crate) const PENDING_SESSIONS: usize = 64;

/// One TLS 1.3 server configuration with ALPN `h3`, built with `provider` (M7).
fn tls_config(
    provider: rustls::crypto::CryptoProvider,
    chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
) -> io::Result<rustls::ServerConfig> {
    let quic_suite = provider
        .cipher_suites
        .iter()
        .any(|s| s.suite() == rustls::CipherSuite::TLS13_AES_128_GCM_SHA256);
    if !quic_suite {
        return Err(io::Error::other(
            "the provider lacks TLS13_AES_128_GCM_SHA256",
        ));
    }
    let mut tls = rustls::ServerConfig::builder_with_provider(Arc::new(provider))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(io::Error::other)?
        .with_no_client_auth()
        .with_single_cert(chain, key)
        .map_err(io::Error::other)?;
    tls.alpn_protocols = vec![b"h3".to_vec()];
    Ok(tls)
}

/// The endpoint over `socket`, quinn's driver on tokio (call inside a tokio runtime).
pub(crate) fn endpoint(
    socket: Arc<ImpairSocket>,
    (endpoint_config, server_config): (quinn::EndpointConfig, quinn::ServerConfig),
) -> io::Result<quinn::Endpoint> {
    quinn::Endpoint::new_with_abstract_socket(
        endpoint_config,
        Some(server_config),
        socket,
        Arc::new(quinn::TokioRuntime),
    )
}

/// wtransport 0.7.2 through its `quinn` feature.
#[cfg(feature = "wt-wtransport")]
pub(crate) mod wtransport_server {
    use super::*;
    use tokio::sync::mpsc;
    use wtransport::endpoint::IncomingSessionFuture;

    /// The crate's own provider: ring (its default feature).
    pub(crate) fn provider() -> rustls::crypto::CryptoProvider {
        rustls::crypto::ring::default_provider()
    }

    /// The endpoint and server configurations, through wtransport's builder (its `build()` checks the QUIC suite and
    /// sets migration), with our TLS (M7) and quinn's default transport (datagrams on).
    pub(crate) fn config(
        chain: Vec<CertificateDer<'static>>,
        key: PrivateKeyDer<'static>,
    ) -> io::Result<(quinn::EndpointConfig, quinn::ServerConfig)> {
        let tls = tls_config(provider(), chain, key)?;
        // The bind address is a placeholder: wtransport binds only in Endpoint::server, which is never called.
        let cfg = wtransport::ServerConfig::builder()
            .with_bind_address(std::net::SocketAddr::from(([127, 0, 0, 1], 0)))
            .with_custom_tls_and_transport(tls, quinn::TransportConfig::default())
            .allow_migration(MIGRATION)
            .build();
        Ok((
            cfg.quic_endpoint_config().clone(),
            cfg.quic_config().clone(),
        ))
    }

    fn reason(e: &wtransport::error::ConnectionError) -> Failure {
        // wtransport runs the QUIC handshake and the HTTP/3 exchange in one future. Two of its errors are surely the
        // second stage: a local HTTP/3 error, and the peer's application close (wtransport 0.7.2 error.rs:16-18; it
        // comes from quinn's ApplicationClosed through error.rs:312-316, or from the driver's ApplicationClosed
        // through with_driver_error, error.rs:48-51). An application CONNECTION_CLOSE travels only in 0-RTT or 1-RTT
        // packets (RFC 9000 §12.4; §10.2.3: during the handshake it is sent as a transport close) and our TLS
        // configuration accepts no 0-RTT (rustls's max_early_data_size stays 0; quinn-proto's crypto/rustls.rs:435-436),
        // so the peer had finished QUIC's handshake. Every other error counts as the QUIC stage.
        match e {
            wtransport::error::ConnectionError::LocalH3Error(_)
            | wtransport::error::ConnectionError::ApplicationClosed(_) => Failure::H3,
            _ => Failure::Quic,
        }
    }

    /// Drives `endpoint.accept()`; each connection's handshake runs in its own task, accepted sessions go to `tx`.
    pub(crate) async fn accept_loop(
        endpoint: quinn::Endpoint,
        counts: Arc<Handshakes>,
        tx: mpsc::Sender<wtransport::Connection>,
    ) {
        while let Some(incoming) = endpoint.accept().await {
            let (counts, tx) = (Arc::clone(&counts), tx.clone());
            tokio::spawn(async move {
                let req = match IncomingSessionFuture::with_quic_incoming(incoming).await {
                    Ok(req) => req,
                    Err(e) => return counts.fail(reason(&e)),
                };
                match req.accept().await {
                    Ok(conn) => {
                        counts.accepted.fetch_add(1, Ordering::Relaxed);
                        let _ = tx.send(conn).await;
                    }
                    Err(_) => counts.fail(Failure::Accept),
                }
            });
        }
    }
}

/// web-transport-quinn 0.13.6: `Request::accept` per connection.
#[cfg(feature = "wt-wtq")]
pub(crate) mod wtq_server {
    use super::*;
    use tokio::sync::mpsc;

    /// The crate's own provider: aws-lc-rs (its default feature).
    pub(crate) fn provider() -> rustls::crypto::CryptoProvider {
        rustls::crypto::aws_lc_rs::default_provider()
    }

    /// The endpoint and server configurations, built directly (as the crate's own builder does), with our TLS (M7),
    /// quinn's default transport (datagrams on) and migration off (M9).
    pub(crate) fn config(
        chain: Vec<CertificateDer<'static>>,
        key: PrivateKeyDer<'static>,
    ) -> io::Result<(quinn::EndpointConfig, quinn::ServerConfig)> {
        let tls = tls_config(provider(), chain, key)?;
        debug_assert_eq!(
            tls.alpn_protocols,
            vec![web_transport_quinn::ALPN.as_bytes().to_vec()]
        );
        let crypto =
            quinn::crypto::rustls::QuicServerConfig::try_from(tls).map_err(io::Error::other)?;
        let mut sc = quinn::ServerConfig::with_crypto(Arc::new(crypto));
        sc.transport_config(Arc::new(quinn::TransportConfig::default()));
        sc.migration(MIGRATION);
        Ok((quinn::EndpointConfig::default(), sc))
    }

    /// Drives `endpoint.accept()`; each connection's QUIC handshake, HTTP/3 exchange and answer are counted apart.
    pub(crate) async fn accept_loop(
        endpoint: quinn::Endpoint,
        counts: Arc<Handshakes>,
        tx: mpsc::Sender<web_transport_quinn::Session>,
    ) {
        while let Some(incoming) = endpoint.accept().await {
            let (counts, tx) = (Arc::clone(&counts), tx.clone());
            tokio::spawn(async move {
                let conn = match incoming.await {
                    Ok(c) => c,
                    Err(_) => return counts.fail(Failure::Quic),
                };
                let req = match web_transport_quinn::Request::accept(conn).await {
                    Ok(r) => r,
                    Err(_) => return counts.fail(Failure::H3),
                };
                match req.ok().await {
                    Ok(session) => {
                        counts.accepted.fetch_add(1, Ordering::Relaxed);
                        let _ = tx.send(session).await;
                    }
                    Err(_) => counts.fail(Failure::Accept),
                }
            });
        }
    }
}

#[cfg(test)]
#[path = "wt_tests.rs"]
mod tests;
