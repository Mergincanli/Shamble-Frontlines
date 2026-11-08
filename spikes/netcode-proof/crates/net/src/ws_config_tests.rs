//! The WebSocket server's configurations, proven (the completeness critic's list): tungstenite's limits come from one
//! source, L2Config's `ws_max_message_bytes`, and bite on the plain path; the TLS refuses a TLS 1.2-only client.

use super::*;
use crate::L2Config;
use crate::l2::tls_test::self_signed;
use futures_util::{SinkExt, StreamExt};
use npt_l1::Profile;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::tungstenite::error::{CapacityError, Error as WsError};
use tokio_tungstenite::tungstenite::protocol::Message;

/// One source, field by field: ws_config(L2Config's limit) reads 513 B for both size limits, a 128 KB write buffer
/// bound and a write buffer of 0 (every message written through). The bite: tungstenite's default configuration fails
/// every one of those fields.
#[test]
fn the_tungstenite_config_comes_from_l2config() {
    let l2 = L2Config::new(1, Profile::p2());
    assert_eq!(l2.ws_max_message_bytes, 513);
    assert_eq!(one_source(&ws_config(l2.ws_max_message_bytes), 513), Ok(()));
    let e = one_source(&WebSocketConfig::default(), 513).expect_err("the default is caught");
    println!("the default config: {e}");
}

/// The plain path (no DelayStream) through tokio-tungstenite: a server on ws_config(513) takes a 513 B message and
/// closes on a 514 B one with tungstenite's capacity error; on the default configuration (the bite) the 514 B message
/// passes.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tungstenite_enforces_the_limit_on_the_plain_path() {
    async fn run(cfg: WebSocketConfig) -> Vec<Result<usize, String>> {
        let (c, s) = tokio::io::duplex(1 << 16);
        let server = tokio::spawn(async move {
            let mut ws = tokio_tungstenite::accept_async_with_config(s, Some(cfg))
                .await
                .expect("the server's handshake");
            let mut got = Vec::new();
            for _ in 0..2 {
                match ws.next().await {
                    Some(Ok(Message::Binary(b))) => got.push(Ok(b.len())),
                    Some(Ok(m)) => got.push(Err(format!("{m:?}"))),
                    Some(Err(WsError::Capacity(CapacityError::MessageTooLong {
                        size,
                        max_size,
                    }))) => {
                        got.push(Err(format!("too long: {size} > {max_size}")));
                        break;
                    }
                    Some(Err(e)) => {
                        got.push(Err(e.to_string()));
                        break;
                    }
                    None => break,
                }
            }
            got
        });
        let (mut ws, _) = tokio_tungstenite::client_async("ws://localhost/", c)
            .await
            .expect("the client's handshake");
        for n in [513, 514] {
            let _ = ws.send(Message::Binary(vec![7u8; n].into())).await;
        }
        tokio::time::timeout(Duration::from_secs(30), server)
            .await
            .expect("a hang guard only")
            .expect("the server task")
    }
    let limited = run(ws_config(L2Config::WS_MAX_MESSAGE_BYTES)).await;
    println!("ws_config(513): {limited:?}");
    assert_eq!(
        limited,
        vec![Ok(513), Err("too long: 514 > 513".to_owned())]
    );
    let default = run(WebSocketConfig::default()).await;
    println!("the default (the bite): {default:?}");
    assert_eq!(default, vec![Ok(513), Ok(514)]);
}

/// A raw ClientHello record (RFC 8446 §4.1.2, and RFC 5246's ClientHello) with x25519, Ed25519 signatures and ALPN
/// http/1.1.
/// `tls13`: the TLS 1.3 offer (supported_versions [TLS 1.3], a key share, TLS 1.3's suites). Otherwise a TLS 1.2-only
/// client: TLS 1.2's suites, and the versions in `tls12_versions` (None: no supported_versions extension at all, the
/// usual TLS 1.2 client; Some: an extension offering exactly those).
fn client_hello(tls13: bool, tls12_versions: Option<&[u16]>) -> Vec<u8> {
    let mut ext = Vec::new();
    let mut push = |ty: u16, body: &[u8]| {
        ext.extend(ty.to_be_bytes());
        ext.extend((body.len() as u16).to_be_bytes());
        ext.extend(body);
    };
    push(0x000a, &[0x00, 0x02, 0x00, 0x1d]);
    push(0x000b, &[0x01, 0x00]);
    push(0x000d, &[0x00, 0x04, 0x08, 0x07, 0x04, 0x03]);
    push(
        0x0010,
        &[
            0x00, 0x09, 0x08, b'h', b't', b't', b'p', b'/', b'1', b'.', b'1',
        ],
    );
    let suites: &[u16] = if tls13 {
        push(0x002b, &[0x02, 0x03, 0x04]);
        let mut ks = vec![0x00, 0x24, 0x00, 0x1d, 0x00, 0x20];
        ks.extend((0u8..32).map(|i| i.wrapping_mul(29).wrapping_add(9)));
        push(0x0033, &ks);
        &[0x1301, 0x1302, 0x1303]
    } else {
        if let Some(v) = tls12_versions {
            let mut b = vec![(2 * v.len()) as u8];
            for x in v {
                b.extend(x.to_be_bytes());
            }
            push(0x002b, &b);
        }
        &[0xc02b, 0xc02c, 0xcca9, 0xc02f]
    };
    let mut body = vec![0x03, 0x03];
    body.extend([0x42; 32]);
    body.push(0);
    body.extend(((suites.len() * 2) as u16).to_be_bytes());
    for s in suites {
        body.extend(s.to_be_bytes());
    }
    body.extend([0x01, 0x00]);
    body.extend((ext.len() as u16).to_be_bytes());
    body.extend(ext);
    let mut hs = vec![0x01];
    hs.extend(&(body.len() as u32).to_be_bytes()[1..]);
    hs.extend(body);
    let mut rec = vec![0x16, 0x03, 0x01];
    rec.extend((hs.len() as u16).to_be_bytes());
    rec.extend(hs);
    rec
}

/// Sends `hello` to the WebSocket server's TLS and returns the first record it answers with (content type, its first
/// two bytes) and the acceptor's error, if it gave up.
async fn answer(hello: Vec<u8>) -> ((u8, [u8; 2]), Option<String>) {
    let (cert, key) = self_signed(&provider());
    let acceptor = acceptor(vec![cert], key).expect("the acceptor");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a port");
    let addr = listener.local_addr().expect("its address");
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.expect("accepted");
        acceptor.accept(tcp).await.err().map(|e| e.to_string())
    });
    let mut tcp = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connected");
    tcp.write_all(&hello).await.expect("the hello");
    let mut head = [0u8; 7];
    tokio::time::timeout(Duration::from_secs(30), tcp.read_exact(&mut head))
        .await
        .expect("a hang guard only")
        .expect("an answer");
    drop(tcp);
    let err = tokio::time::timeout(Duration::from_secs(30), server)
        .await
        .expect("a hang guard only")
        .expect("the server task");
    ((head[0], [head[5], head[6]]), err)
}

/// "TLS 1.3 only": a TLS 1.2-only client is refused with a fatal protocol_version alert (RFC 8446 §6.2: content type
/// 21, level 2, description 70), both the usual kind (no supported_versions extension) and one offering exactly TLS
/// 1.2 in it, and the acceptor gives up. The control, from the same builder: the TLS 1.3 offer gets a ServerHello
/// (content type 22, handshake type 2), so the refusal check can fail.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_tls12_only_client_is_refused() {
    for (name, hello) in [
        ("no supported_versions", client_hello(false, None)),
        (
            "supported_versions [TLS 1.2]",
            client_hello(false, Some(&[0x0303])),
        ),
    ] {
        let ((ty, alert), err) = answer(hello).await;
        println!("TLS 1.2 client ({name}): record type {ty}, alert {alert:?}; the server: {err:?}");
        assert_eq!(
            (ty, alert),
            (21, [2, 70]),
            "{name}: a fatal protocol_version alert"
        );
        assert!(err.is_some(), "{name}: the acceptor gave up");
    }
    let ((ty, first), _) = answer(client_hello(true, None)).await;
    println!("TLS 1.3 client (the control): record type {ty}, first bytes {first:?}");
    assert_eq!((ty, first[0]), (22, 2), "a ServerHello");
}
