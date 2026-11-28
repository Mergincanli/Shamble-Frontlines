//! The certificate pin (u6b-brief.md Block 1; tls_test.rs's `Pinned` promoted to shipped code): a client accepts the
//! one server certificate whose SHA-256 equals a given 32 B hash, as a browser's `serverCertificateHashes` does, and
//! still checks the handshake's signature through the provider's own algorithms. Both native clients (WtClient,
//! WsClient) use it, on ring, TLS 1.3 only.

use std::sync::Arc;

use tokio_rustls::rustls;
use tokio_rustls::rustls::client::danger::{
    HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier,
};
use tokio_rustls::rustls::crypto::CryptoProvider;
use tokio_rustls::rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use tokio_rustls::rustls::{DigitallySignedStruct, SignatureScheme};

/// SHA-256 of `bytes`, through ring's TLS 1.3 suite's own hash (no hashing crate of npt-net's own).
pub(crate) fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    if let Some(suite) = rustls::crypto::ring::cipher_suite::TLS13_AES_128_GCM_SHA256.tls13() {
        out.copy_from_slice(suite.common.hash_provider.hash(bytes).as_ref());
    }
    out
}

/// Accepts exactly the certificate whose SHA-256 is `hash`, and checks the handshake's signature with the provider's
/// algorithms.
#[derive(Debug)]
struct Pinned {
    hash: [u8; 32],
    provider: Arc<CryptoProvider>,
}

impl ServerCertVerifier for Pinned {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        if sha256(end_entity.as_ref()) == self.hash {
            Ok(ServerCertVerified::assertion())
        } else {
            Err(rustls::Error::General("not the pinned certificate".into()))
        }
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Err(rustls::Error::General("TLS 1.2 is not offered".into()))
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}

/// A client's TLS: TLS 1.3 through `provider`, ALPN `alpn`, the server's certificate pinned by its SHA-256 `hash`.
pub(crate) fn client_config(
    provider: CryptoProvider,
    hash: [u8; 32],
    alpn: &[u8],
) -> std::io::Result<rustls::ClientConfig> {
    let provider = Arc::new(provider);
    let mut tls = rustls::ClientConfig::builder_with_provider(Arc::clone(&provider))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(std::io::Error::other)?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(Pinned { hash, provider }))
        .with_no_client_auth();
    tls.alpn_protocols = vec![alpn.to_vec()];
    Ok(tls)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FIPS 180-2's "abc" vector, its first and last 8 bytes (a whole 64-digit literal reads as a secret to
    /// redact-check): the hash the pin compares is SHA-256.
    #[test]
    fn u6b_the_pin_hashes_with_sha_256() {
        let got = sha256(b"abc");
        assert_eq!(got[..8], [0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea]);
        assert_eq!(got[24..], [0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad]);
    }

    /// SEC-U6B-4 (P19's test): the pin still checks the handshake's signature: a server presenting the pinned
    /// certificate with its own key is taken; with another key (a replayed public certificate) it is refused.
    #[tokio::test(flavor = "current_thread")]
    async fn u6b_the_pin_refuses_the_pinned_certificate_signed_with_another_key() {
        use rustls::pki_types::{PrivateKeyDer, PrivatePkcs8KeyDer};
        use rustls::server::{ClientHello, ResolvesServerCert};
        use rustls::sign::CertifiedKey;
        #[derive(Debug)]
        struct Fixed(Arc<CertifiedKey>);
        impl ResolvesServerCert for Fixed {
            fn resolve(&self, _: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
                Some(Arc::clone(&self.0))
            }
        }
        let provider = rustls::crypto::ring::default_provider();
        let (cert, own) = crate::l2::tls_test::self_signed(&provider);
        // Another Ed25519 key: the same PKCS#8 prefix (RFC 8410), another seed.
        let mut other = own.secret_der()[..16].to_vec();
        other.extend([7u8; 32]);
        let other = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(other));
        for (key, taken) in [(own, true), (other, false)] {
            let signer = provider.key_provider.load_private_key(key).expect("a key");
            let ck = Arc::new(CertifiedKey::new(vec![cert.clone()], signer));
            let server = rustls::ServerConfig::builder_with_provider(Arc::new(provider.clone()))
                .with_protocol_versions(&[&rustls::version::TLS13])
                .expect("TLS 1.3")
                .with_no_client_auth()
                .with_cert_resolver(Arc::new(Fixed(ck)));
            let client =
                client_config(provider.clone(), sha256(cert.as_ref()), b"h3").expect("TLS");
            let (c, s) = tokio::io::duplex(16_384);
            let name = ServerName::try_from("localhost").expect("a name");
            let (got, _) = tokio::join!(
                tokio_rustls::TlsConnector::from(Arc::new(client)).connect(name, c),
                tokio_rustls::TlsAcceptor::from(Arc::new(server)).accept(s),
            );
            assert_eq!(got.is_ok(), taken, "own key: {taken}");
        }
    }
}
