//! The WebSocket server's configurations (u3c-plan.md 1.1 and 1.2; C6, C7):
//! - **TLS** (M16; Mustafa's U3-Q5: tokio-rustls pinned in U3, audited at U5, plan §4.8): tokio-rustls 0.26.6 over the
//!   pinned rustls 0.23.46, the ring provider chosen explicitly for this configuration with `builder_with_provider`
//!   (never a process default, so it lives beside wt-wtq's aws-lc-rs and wtransport's ring in one process: M7), TLS 1.3
//!   only, ALPN `http/1.1`. Ring because it is the one provider npt-net always has (quinn's default rustls-ring).
//!   DelayStream sits above the stream it makes, tokio-tungstenite above DelayStream (U3-Q4 (b)).
//! - **tungstenite** (the catalogue, docs/design/netcode-proof-messages.md:61-78): one source for the size limits,
//!   L2Config's `ws_max_message_bytes` (513 B), as both `max_frame_size` and `max_message_size`; a 128 KB write buffer
//!   bound (it must hold a 65,541 B FullState); `write_buffer_size` 0, so each message is written through to the
//!   stream at once (fix 1's gate then sees what tungstenite still holds as owed bytes).

use std::io;
use std::sync::Arc;
use tokio_rustls::rustls;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

/// The catalogue's write buffer bound: the largest room message (a 65,541 B FullState) with room to spare.
pub(crate) const MAX_WRITE_BUFFER_BYTES: usize = 128 * 1024;

/// The provider: ring, explicitly.
pub(crate) fn provider() -> rustls::crypto::CryptoProvider {
    rustls::crypto::ring::default_provider()
}

/// The server's TLS: TLS 1.3 only, ALPN `http/1.1`, the ring provider (no process default).
pub(crate) fn tls_config(
    chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
) -> io::Result<rustls::ServerConfig> {
    let mut tls = rustls::ServerConfig::builder_with_provider(Arc::new(provider()))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(io::Error::other)?
        .with_no_client_auth()
        .with_single_cert(chain, key)
        .map_err(io::Error::other)?;
    tls.alpn_protocols = vec![b"http/1.1".to_vec()];
    Ok(tls)
}

/// The accept path's TLS acceptor over `tls_config`.
pub(crate) fn acceptor(
    chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
) -> io::Result<tokio_rustls::TlsAcceptor> {
    Ok(tokio_rustls::TlsAcceptor::from(Arc::new(tls_config(
        chain, key,
    )?)))
}

/// tungstenite's server configuration with the size limit `max_message_bytes` (L2Config's `ws_max_message_bytes`).
pub(crate) fn ws_config(max_message_bytes: usize) -> WebSocketConfig {
    WebSocketConfig::default()
        .max_frame_size(Some(max_message_bytes))
        .max_message_size(Some(max_message_bytes))
        .max_write_buffer_size(MAX_WRITE_BUFFER_BYTES)
        .write_buffer_size(0)
}

/// The "one source" claim, field by field: `c` takes both size limits from `max` (L2Config's `ws_max_message_bytes`),
/// bounds its write buffer at 128 KB, writes each message through (write buffer 0), and refuses unmasked client frames.
#[cfg(test)]
pub(crate) fn one_source(c: &WebSocketConfig, max: usize) -> Result<(), String> {
    let got = (
        c.max_frame_size,
        c.max_message_size,
        c.max_write_buffer_size,
        c.write_buffer_size,
        c.accept_unmasked_frames,
    );
    let want = (Some(max), Some(max), MAX_WRITE_BUFFER_BYTES, 0, false);
    if got == want {
        Ok(())
    } else {
        Err(format!(
            "(frame, message, write buffer bound, write buffer, unmasked) {got:?}, not {want:?}"
        ))
    }
}

#[cfg(test)]
#[path = "ws_config_tests.rs"]
mod tests;
