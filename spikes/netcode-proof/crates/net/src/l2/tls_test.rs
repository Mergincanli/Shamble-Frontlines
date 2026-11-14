//! The loopback tests' TLS, shared by the WebTransport routes (wt_tests) and DelayStream's WebSocket path
//! (delay_tests); moved here from wt_tests.rs at U3c so the WebSocket tests need no WebTransport feature.
//!
//! The certificate is test-only and made here (no new dependency: rcgen is not in web-transport-quinn's closure): an
//! Ed25519 key from a fixed seed, loaded and used through the caller's own rustls provider, and an X.509 v3
//! certificate written in DER by hand. The client pins it (as `serverCertificateHashes` would) with a verifier that
//! still checks the handshake's signature. rustls is the one pinned 0.23.46, reached through tokio-rustls (quinn's
//! re-export is the same crate).

use std::sync::Arc;
use tokio_rustls::rustls;
use tokio_rustls::rustls::client::danger::{
    HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier,
};
use tokio_rustls::rustls::crypto::CryptoProvider;
use tokio_rustls::rustls::pki_types::{
    CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName, UnixTime,
};
use tokio_rustls::rustls::{DigitallySignedStruct, SignatureScheme};

/// A DER element: tag, definite length, content.
fn der(tag: u8, content: &[u8]) -> Vec<u8> {
    let mut v = vec![tag];
    let n = content.len();
    if n < 0x80 {
        v.push(n as u8);
    } else {
        let bytes: Vec<u8> = n
            .to_be_bytes()
            .into_iter()
            .skip_while(|&b| b == 0)
            .collect();
        v.push(0x80 | bytes.len() as u8);
        v.extend(bytes);
    }
    v.extend_from_slice(content);
    v
}

fn seq(parts: &[&[u8]]) -> Vec<u8> {
    der(0x30, &parts.concat())
}

/// A self-signed Ed25519 certificate and its key, signed through `provider`'s own key provider.
pub(crate) fn self_signed(
    provider: &CryptoProvider,
) -> (CertificateDer<'static>, PrivateKeyDer<'static>) {
    // PKCS#8 v1 for Ed25519 (RFC 8410): a fixed prefix and the 32-byte seed.
    let mut pkcs8 = vec![
        0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22, 0x04,
        0x20,
    ];
    pkcs8.extend((0u8..32).map(|i| i.wrapping_mul(37).wrapping_add(11)));
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(pkcs8));
    let signing = provider
        .key_provider
        .load_private_key(key.clone_key())
        .expect("the provider loads an Ed25519 key");
    let spki = signing.public_key().expect("its public key").to_vec();
    let ed25519 = seq(&[&der(0x06, &[0x2b, 0x65, 0x70])]);
    let cn = der(0x0c, b"npt-l2 loopback test");
    let name = seq(&[&der(0x31, &seq(&[&der(0x06, &[0x55, 0x04, 0x03]), &cn]))]);
    // SubjectAltName: IP 127.0.0.1, DNS localhost.
    let san = seq(&[&der(0x87, &[127, 0, 0, 1]), &der(0x82, b"localhost")]);
    let ext = der(
        0xa3,
        &seq(&[&seq(&[&der(0x06, &[0x55, 0x1d, 0x11]), &der(0x04, &san)])]),
    );
    let validity = seq(&[&der(0x17, b"260101000000Z"), &der(0x17, b"360101000000Z")]);
    let tbs = seq(&[
        &der(0xa0, &der(0x02, &[2])),
        &der(0x02, &[0x01, 0x23, 0x45]),
        &ed25519,
        &name,
        &validity,
        &name,
        &spki,
        &ext,
    ]);
    let signer = signing
        .choose_scheme(&[SignatureScheme::ED25519])
        .expect("an Ed25519 signer");
    let sig = signer.sign(&tbs).expect("signed");
    let mut bits = vec![0u8];
    bits.extend(sig);
    let cert = seq(&[&tbs, &ed25519, &der(0x03, &bits)]);
    (CertificateDer::from(cert), key)
}

/// Accepts exactly one certificate and checks the handshake's signature with the provider's algorithms.
#[derive(Debug)]
struct Pinned {
    cert: CertificateDer<'static>,
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
        if end_entity.as_ref() == self.cert.as_ref() {
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

/// The client's TLS: TLS 1.3, ALPN `alpn`, the server's certificate pinned.
pub(crate) fn client_tls(
    provider: CryptoProvider,
    cert: CertificateDer<'static>,
    alpn: &[u8],
) -> rustls::ClientConfig {
    let provider = Arc::new(provider);
    let mut tls = rustls::ClientConfig::builder_with_provider(Arc::clone(&provider))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .expect("TLS 1.3")
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(Pinned { cert, provider }))
        .with_no_client_auth();
    tls.alpn_protocols = vec![alpn.to_vec()];
    tls
}
