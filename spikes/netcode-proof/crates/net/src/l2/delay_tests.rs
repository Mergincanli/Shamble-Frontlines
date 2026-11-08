//! DelayStream's own tests (u3c-plan.md 1.5) and mechanism test (c) (1.4, C9).
//!
//! Real-time tests are judged on model state and identities (a release never before its model stamp, a receipt never
//! before its release, counts equal to the model's), never on wall-clock thresholds (C16); each states its counts, its
//! duration and its band's level. Waits are hang guards only (`HANG`): a test that passes never reaches one. Every
//! reading a judgement compares is taken in one atomic snapshot (`DelayStream::snapshot`, W2). Lateness is printed with
//! condition 10's trigger for the main session's evidence run, never asserted.
//!
//! The WebSocket ends run through tokio-tungstenite's own adapter (its Sink and Stream, futures-util a dev-dependency:
//! the advisor's ruling of 2026-11-08), so DelayStream's Pending, wakes and flushes meet the library as WsServer will.

use super::super::config::L2Config;
use super::super::impair::Settings;
use super::super::tls_test::{client_tls, self_signed};
use super::*;
use crate::tcp_model::Counters;
use crate::ws_config;
use crate::ws_gate::{Bite, Inputs, Snap, TickWriter, snapshot_budget};
use futures_util::{FutureExt, Sink, SinkExt, Stream, StreamExt};
use npt_l1::{Profile, ReorderStep};
use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};
use tokio_rustls::rustls::pki_types::ServerName;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::{self, Message};

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;
/// The room's 30 Hz tick (fixes.rs's TICK).
const TICK: u64 = 33_333_333;
/// A 1,024 B snapshot as the model counts it: 1,024 + 4 B frame header + 22 B (fixes.rs's SNAP).
const SNAP: u64 = 1_050;
const BUDGET: u64 = 64 * 1024;
/// A hang guard: how long a wait may take before the test fails. Never a judged threshold.
const HANG: Duration = Duration::from_secs(30);

const REQUEST: &[u8] =
    b"GET / HTTP/1.1\r\nHost: localhost\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n";

fn noop() -> Context<'static> {
    Context::from_waker(Waker::noop())
}

fn cfg(p: Profile, seed: u64) -> L2Config {
    L2Config::new(seed, p)
}

fn quiet(mut p: Profile) -> Profile {
    p.loss_ppm = 0;
    p
}

/// No jitter, no loss: `rtt_ms` round trip.
fn fixed(rtt_ms: u64) -> Profile {
    Profile {
        name: "T",
        rtt_ns: rtt_ms * MS,
        jitter_ns: 0,
        loss_ppm: 0,
        bursts_ns: Vec::new(),
        reorder: ReorderStep::OFF,
    }
}

/// A client frame (masked unless `mask` is false), with a 64-bit length when `long`.
fn frame(fin: bool, opcode: u8, mask: bool, payload: &[u8], long: bool) -> Vec<u8> {
    let mut v = vec![(u8::from(fin) << 7) | opcode];
    let m = if mask { 0x80 } else { 0 };
    let n = payload.len();
    if long || n > 0xffff {
        v.push(m | 127);
        v.extend((n as u64).to_be_bytes());
    } else if n >= 126 {
        v.push(m | 126);
        v.extend((n as u16).to_be_bytes());
    } else {
        v.push(m | n as u8);
    }
    if mask {
        let key = [0x11, 0x22, 0x33, 0x44];
        v.extend(key);
        v.extend(payload.iter().enumerate().map(|(i, b)| b ^ key[i % 4]));
    } else {
        v.extend(payload);
    }
    v
}

/// Waits until `f` holds, for at most `limit` (a hang guard); whether it held.
async fn until(limit: Duration, mut f: impl FnMut() -> bool) -> bool {
    let end = Instant::now() + limit;
    loop {
        if f() {
            return true;
        }
        if Instant::now() >= end {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
}

fn now() -> u64 {
    DelayStream::now_ns()
}

/// The model bytes of every write at `end`, in order.
fn writes(l: &Logs, end: usize) -> Vec<u64> {
    l.steps
        .iter()
        .filter_map(|&(_, s)| match s {
            Step::Write { end: e, bytes } if e == end => Some(bytes),
            _ => None,
        })
        .collect()
}

/// Every burst added after the wrap, as logged (model axis): (start, length).
fn bursts_of(l: &Logs) -> Vec<(u64, u64)> {
    l.steps
        .iter()
        .filter_map(|&(_, s)| match s {
            Step::Burst { start, len } => Some((start, len)),
            _ => None,
        })
        .collect()
}

/// The data transmissions each way stamped inside [start, start + len) (model axis), by the model's own rule.
fn inside(sent: &[(usize, u64, u32, u64, bool)], start: u64, len: u64) -> [u64; 2] {
    let mut n = [0u64; 2];
    for &(end, _, _, t, _) in sent {
        if t >= start && t - start < len {
            n[end] += 1;
        }
    }
    n
}

/// Adds a burst of `len_ns` starting 1 ms from now on DelayStream's clock, again with a fresh now if the model passed
/// that start before the call stepped it (C4's refusal), so a stalled test thread cannot place it in the past.
fn add_burst_soon<S>(ds: &DelayStream<S>, len_ns: u64) {
    for _ in 0..1_000 {
        if ds.add_burst(now() + MS, len_ns).is_ok() {
            return;
        }
    }
    panic!("no burst accepted in 1,000 tries");
}

fn value(rows: &[(&'static str, u64)], name: &str) -> u64 {
    rows.iter()
        .find(|r| r.0 == name)
        .unwrap_or_else(|| panic!("no {name}"))
        .1
}

/// The exact two-sided `level` band of Binomial(n, p): the smallest lo and hi with P(X < lo) <= (1 − level) / 2 and
/// P(X > hi) <= (1 − level) / 2.
fn band(n: u64, p: f64, level: f64) -> (u64, u64) {
    let tail = (1.0 - level) / 2.0;
    let q = 1.0 - p;
    let mut pmf = vec![q.powi(n as i32)];
    for k in 1..=n {
        let prev = pmf[k as usize - 1];
        pmf.push(prev * (n - k + 1) as f64 / k as f64 * p / q);
    }
    let mut below = 0.0;
    let mut lo = 0;
    for (k, x) in pmf.iter().enumerate() {
        if below + x > tail {
            lo = k as u64;
            break;
        }
        below += x;
    }
    let mut above = 0.0;
    let mut hi = n;
    for k in (0..=n as usize).rev() {
        if above + pmf[k] > tail {
            hi = k as u64;
            break;
        }
        above += pmf[k];
    }
    (lo, hi)
}

/// Condition 10's trigger: p99.9 above 1 ms or max above 10 ms.
fn trigger(s: &Summary) -> bool {
    s.p999 > MS || s.max > 10 * MS
}

fn late_line(name: &str, s: &Summary) -> String {
    format!(
        "{name}: n {} p50 {:.3} ms p99 {:.3} ms p99.9 {:.3} ms max {:.3} ms E {:.1} µs over {}{}",
        s.n,
        s.p50 as f64 / 1e6,
        s.p99 as f64 / 1e6,
        s.p999 as f64 / 1e6,
        s.max as f64 / 1e6,
        s.excess as f64 / 1e3,
        s.over,
        if trigger(s) {
            "  ** CONDITION 10's TRIGGER: bring to the advisor **"
        } else {
            ""
        }
    )
}

/// p50, p99 and the largest of `v` in ms.
fn quantiles(v: &mut [u64]) -> String {
    v.sort_unstable();
    let q = |p: usize| v[(v.len() * p / 1_000).min(v.len() - 1)] as f64 / 1e6;
    format!(
        "p50 {:.3} ms p99 {:.3} ms max {:.3} ms",
        q(500),
        q(990),
        q(1_000)
    )
}

// ---------------------------------------------------------------------------------------------------------------
// tokio-tungstenite over the loopback TLS streams.

pub(crate) type TlsServer = tokio_rustls::server::TlsStream<tokio::net::TcpStream>;
pub(crate) type TlsClient = tokio_rustls::client::TlsStream<tokio::net::TcpStream>;
type ServerWs = WebSocketStream<DelayStream<TlsServer>>;
type ClientWs = WebSocketStream<TlsClient>;

/// A message's receipt: the stamps on DelayStream's clock at the start and at the end of the poll that yielded it.
#[derive(Debug, Clone, Copy)]
struct Recv {
    t0: u64,
    t1: u64,
}

/// The next message on `ws` with its receipt stamps; None once the connection ends.
async fn next_stamped<T: AsyncRead + AsyncWrite + Unpin>(
    ws: &mut WebSocketStream<T>,
) -> Option<(Message, Recv)> {
    std::future::poll_fn(|cx| {
        let t0 = now();
        match ws.poll_next_unpin(cx) {
            Poll::Ready(Some(Ok(m))) => Poll::Ready(Some((m, Recv { t0, t1: now() }))),
            Poll::Ready(_) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    })
    .await
}

/// Reads every message `ws` has (each binary payload to `got` with its receipt) until `deadline`; false once the
/// connection ends.
async fn read_until<T: AsyncRead + AsyncWrite + Unpin>(
    ws: &mut WebSocketStream<T>,
    deadline: tokio::time::Instant,
    got: &mut impl FnMut(&[u8], Recv),
) -> bool {
    loop {
        tokio::select! {
            biased;
            _ = tokio::time::sleep_until(deadline) => return true,
            m = next_stamped(ws) => match m {
                Some((Message::Binary(b), r)) => got(&b, r),
                Some(_) => {}
                None => return false,
            },
        }
    }
}

/// `ticks` ticks of TICK: reads all the while, and at each tick k (k = 0 at once) sends what `tick(k, ws)` gives, if
/// anything (awaited: a full budget parks the send). Then reads for `drain` more. Writes happen only here.
async fn run_ticks<T: AsyncRead + AsyncWrite + Unpin>(
    ws: &mut WebSocketStream<T>,
    ticks: u64,
    drain: Duration,
    mut got: impl FnMut(&[u8], Recv),
    mut tick: impl FnMut(u64, &WebSocketStream<T>) -> Option<Vec<u8>>,
) {
    let start = tokio::time::Instant::now();
    for k in 0..ticks {
        if !read_until(ws, start + Duration::from_nanos(k * TICK), &mut got).await {
            return;
        }
        if let Some(m) = tick(k, ws) {
            // Errors are left to the judgement of what arrived.
            let _ = ws.send(Message::Binary(m.into())).await;
        }
    }
    read_until(ws, tokio::time::Instant::now() + drain, &mut got).await;
}

/// A message: its index and its send stamp (DelayStream's clock), padded to `len`.
fn stamped(k: u64, len: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(len);
    v.extend(k.to_le_bytes());
    v.extend(now().to_le_bytes());
    v.resize(len, 0x5a);
    v
}

fn unstamp(b: &[u8]) -> (u64, u64) {
    let k = u64::from_le_bytes(b[..8].try_into().expect("8 B"));
    let at = u64::from_le_bytes(b[8..16].try_into().expect("8 B"));
    (k, at)
}

/// One loopback WebSocket over TLS 1.3 (tokio-rustls, ring) with DelayStream in the server's accept path, both ends
/// through tokio-tungstenite: (the server's WebSocket over DelayStream, the client's, when the client's handshake
/// ended, the TLS version and ALPN the server negotiated).
pub(crate) async fn pair(cfg: &L2Config, conn: u32) -> (ServerWs, ClientWs, u64, String) {
    let (cert, key) = self_signed(&ws_config::provider());
    let acceptor = ws_config::acceptor(vec![cert.clone()], key).expect("the server's TLS");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a port");
    let addr = listener.local_addr().expect("its address");
    let c = cfg.clone();
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.expect("accepted");
        let accepted = now();
        tcp.set_nodelay(true).expect("TCP_NODELAY");
        let tls = acceptor.accept(tcp).await.expect("the TLS handshake");
        let (_, state) = tls.get_ref();
        let negotiated = format!(
            "{:?} {:?}",
            state.protocol_version(),
            state.alpn_protocol().map(<[u8]>::to_vec)
        );
        let ds = DelayStream::wrap(tls, &c, conn, accepted).expect("wrapped");
        assert_eq!(ds.accepted_ns(), accepted);
        let ws = tokio_tungstenite::accept_async_with_config(
            ds,
            Some(ws_config::ws_config(c.ws_max_message_bytes)),
        )
        .await
        .expect("the server's WebSocket handshake");
        (ws, negotiated)
    });
    let tcp = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connected");
    tcp.set_nodelay(true).expect("TCP_NODELAY");
    let connector = tokio_rustls::TlsConnector::from(Arc::new(client_tls(
        ws_config::provider(),
        cert,
        b"http/1.1",
    )));
    let name = ServerName::try_from("localhost").expect("a name");
    let tls = connector
        .connect(name, tcp)
        .await
        .expect("the client's TLS");
    let (client, _) = tokio_tungstenite::client_async("wss://localhost/", tls)
        .await
        .expect("the client's WebSocket handshake");
    let at = now();
    let (server, negotiated) = server.await.expect("the server task");
    (server, client, at, negotiated)
}

/// One WebSocket session over `pair` exchanging `ticks` messages each way at 30 Hz (60 B up, 1,024 B down): (up
/// received, down received, the TLS version and ALPN negotiated). The one-process test runs it beside both
/// WebTransport servers (wt_tests).
#[cfg(all(feature = "wt-wtransport", feature = "wt-wtq"))]
pub(crate) async fn ws_session(cfg: L2Config, ticks: u64) -> (u64, u64, String) {
    let (mut server, mut client, _, negotiated) = pair(&cfg, 0).await;
    let client_task = tokio::spawn(async move {
        let mut n = 0u64;
        run_ticks(
            &mut client,
            ticks,
            Duration::from_secs(2),
            |_, _| n += 1,
            |k, _| Some(stamped(k, 60)),
        )
        .await;
        n
    });
    let mut up = 0u64;
    run_ticks(
        &mut server,
        ticks,
        Duration::from_secs(2),
        |_, _| up += 1,
        |k, _| Some(stamped(k, 1_024)),
    )
    .await;
    let down = client_task.await.expect("the client");
    (up, down, negotiated)
}

// ---------------------------------------------------------------------------------------------------------------
// Unit tests: S an in-memory duplex.

/// Order and integrity both ways (hashes of 200 random frames up and 200 random writes down, at P2 with its loss and
/// jitter), and every release no earlier than its model stamp: a byte reaches the client side of S, or tungstenite's
/// side of DelayStream, only after the model released its record. About 2 s.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn order_and_integrity_both_ways_and_no_release_before_its_stamp() {
    let (client, server) = tokio::io::duplex(1 << 20);
    let mut ds = DelayStream::wrap(server, &cfg(Profile::p2(), 11), 0, now()).expect("wrapped");
    let mut rng = ChaCha8Rng::seed_from_u64(5);
    let mut below = |n: u64| ((u128::from(rng.next_u64()) * u128::from(n)) >> 64) as u64;
    let mut up = REQUEST.to_vec();
    for _ in 0..200 {
        let p: Vec<u8> = (0..1 + below(513)).map(|_| below(256) as u8).collect();
        up.extend(frame(true, 2, true, &p, false));
    }
    let mut chunks: Vec<Vec<u8>> = Vec::new();
    for _ in 0..200 {
        chunks.push((0..1 + below(20_000)).map(|_| below(256) as u8).collect());
    }
    let down: Vec<u8> = chunks.concat();
    let (mut cr, mut cw) = tokio::io::split(client);
    let up2 = up.clone();
    let writer = tokio::spawn(async move { cw.write_all(&up2).await.expect("written") });
    let want = down.len();
    let reader = tokio::spawn(async move {
        let mut got = Vec::new();
        let mut seen = Vec::new();
        let mut buf = vec![0u8; 1 << 16];
        while got.len() < want {
            let n = cr.read(&mut buf).await.expect("read");
            assert!(n > 0, "S ended early");
            got.extend_from_slice(&buf[..n]);
            seen.push((now(), got.len() as u64));
        }
        (got, seen)
    });
    let mut got_up = Vec::new();
    let mut seen_up = Vec::new();
    let mut buf = vec![0u8; 4_096];
    let mut chunk = chunks.iter();
    while got_up.len() < up.len() {
        if let Some(c) = chunk.next() {
            ds.write_all(c).await.expect("written down");
        }
        let n = tokio::time::timeout(Duration::from_millis(50), ds.read(&mut buf))
            .await
            .unwrap_or(Ok(usize::MAX));
        match n {
            Ok(usize::MAX) => {}
            Ok(0) => panic!("EOF"),
            Ok(n) => {
                got_up.extend_from_slice(&buf[..n]);
                seen_up.push((now(), got_up.len() as u64));
            }
            Err(e) => panic!("{e}"),
        }
    }
    for c in chunk {
        ds.write_all(c).await.expect("written down");
    }
    writer.await.expect("the client wrote");
    let (got_down, seen_down) = reader.await.expect("the client read");
    assert!(got_up == up, "up: the bytes arrive whole and in order");
    assert!(
        got_down == down,
        "down: the bytes arrive whole and in order"
    );
    // No release before its stamp: each record's plaintext end, first seen at or after origin + its stamp.
    let s = ds.snapshot();
    for (dir, seen) in [(DOWN, &seen_down), (UP, &seen_up)] {
        let rels: Vec<(u64, u64)> = s
            .logs
            .releases
            .iter()
            .filter(|r| r.0 == dir)
            .map(|r| (r.1, r.2))
            .collect();
        assert!(!rels.is_empty());
        for (k, &(model_end, stamp)) in rels.iter().enumerate() {
            let plain_end = model_end - OVERHEAD * (k as u64 + 1);
            let first = seen
                .iter()
                .find(|x| x.1 >= plain_end)
                .expect("every released byte was seen");
            assert!(
                first.0 >= s.origin + stamp,
                "dir {dir} record {k}: seen at {} before its release {}",
                first.0,
                s.origin + stamp
            );
        }
        println!(
            "dir {dir}: {} records released, each seen at or after its model stamp",
            rels.len()
        );
    }
}

/// The one-record rule (C1): with 30,000 B free, a 40,000 B write takes one record of 16,384 B, the next only what
/// leaves 22 B for its own record, and the budget is never exceeded; then Pending (never Ok(0)), with tungstenite's
/// rest owed. A 2 s round trip keeps every byte in flight.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn one_record_per_write_never_beyond_the_budget() {
    let (_client, server) = tokio::io::duplex(1 << 20);
    let mut ds = DelayStream::wrap(server, &cfg(fixed(2_000), 1), 0, now()).expect("wrapped");
    let mut w = |len: usize| match Pin::new(&mut ds).poll_write(&mut noop(), &vec![0x82; len]) {
        Poll::Ready(Ok(n)) => Some(n),
        Poll::Ready(Err(e)) => panic!("{e}"),
        Poll::Pending => None,
    };
    assert_eq!(w(16_384), Some(16_384));
    assert_eq!(w(20_000), Some(16_384));
    assert_eq!(w(2_702), Some(2_702));
    assert_eq!(ds.free_bytes(), 30_000);
    assert_eq!(ds.outstanding_bytes(), 35_536);
    let mut w = |len: usize| match Pin::new(&mut ds).poll_write(&mut noop(), &vec![0x82; len]) {
        Poll::Ready(Ok(n)) => Some(n),
        Poll::Ready(Err(e)) => panic!("{e}"),
        Poll::Pending => None,
    };
    assert_eq!(w(40_000), Some(16_384), "one record per call");
    assert_eq!(w(23_616), Some(13_572), "free − 22 B");
    assert_eq!(w(10_044), None, "Pending at a full budget, never Ok(0)");
    assert_eq!(
        ds.outstanding_bytes(),
        BUDGET,
        "exactly the budget, never beyond"
    );
    assert_eq!(ds.free_bytes(), 0);
    assert_eq!(ds.owed_bytes(), 10_044);
    let l = ds.logs();
    assert_eq!(
        writes(&l, DOWN),
        vec![16_406, 16_406, 2_724, 16_406, 13_594]
    );
}

/// Pending at a full budget and woken when it frees; the held-bytes bounds under a client that never reads (down:
/// S takes nothing, so released bytes stay untaken) and under a tungstenite that never reads (up: the client floods
/// 150 KB of frames). Quiet P2, about 2 s.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded() {
    let (client, server) = tokio::io::duplex(1_024);
    let mut ds =
        DelayStream::wrap(server, &cfg(quiet(Profile::p2()), 2), 0, now()).expect("wrapped");
    let (mut cr, mut cw) = tokio::io::split(client);
    let flood = tokio::spawn(async move {
        cw.write_all(REQUEST).await.expect("the request");
        for _ in 0..300 {
            // Ends when S is dropped at the test's end.
            if cw
                .write_all(&frame(true, 2, true, &[7; 500], false))
                .await
                .is_err()
            {
                break;
            }
        }
    });
    let mut written = 0u64;
    loop {
        match Pin::new(&mut ds).poll_write(&mut noop(), &[0x82; 8_192]) {
            Poll::Ready(Ok(n)) => {
                assert!(n > 0, "never Ok(0)");
                written += n as u64;
            }
            Poll::Ready(Err(e)) => panic!("{e}"),
            Poll::Pending => break,
        }
    }
    // The model delivers, S takes 1 KB and stops: released bytes hold the budget, so writes stay Pending.
    tokio::time::sleep(Duration::from_millis(1_500)).await;
    assert!(matches!(
        Pin::new(&mut ds).poll_write(&mut noop(), &[0x82; 100]),
        Poll::Pending
    ));
    let (_, _, held, max) = ds.probe();
    println!("written {written} B before Pending; held now {held:?}, largest {max:?}");
    assert!(written <= BUDGET);
    assert!(max[DOWN] <= BUDGET, "down held {} B", max[DOWN]);
    assert!(
        max[UP] <= BUDGET + L2Config::WS_MAX_REQUEST_BYTES as u64,
        "up held {} B",
        max[UP]
    );
    assert!(
        !flood.is_finished(),
        "the client still had frames to send: S was not read beyond the bound"
    );
    // The client reads: the pending write is woken and completes.
    let drain = tokio::spawn(async move {
        let mut buf = vec![0u8; 1 << 16];
        while cr.read(&mut buf).await.is_ok_and(|n| n > 0) {}
    });
    tokio::time::timeout(HANG, ds.write_all(&[0x82; 8_192]))
        .await
        .expect("woken when the budget frees")
        .expect("written");
    drop(ds);
    drain.await.expect("drained");
    flood.abort();
}

/// The request is one record, whatever its chunking (the client writes it a byte at a time).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_request_is_one_record() {
    let (mut client, server) = tokio::io::duplex(1 << 16);
    let ds = DelayStream::wrap(server, &cfg(quiet(Profile::p1()), 3), 0, now()).expect("wrapped");
    for b in REQUEST {
        client.write_all(&[*b]).await.expect("a byte");
        tokio::task::yield_now().await;
    }
    assert!(until(HANG, || !writes(&ds.logs(), UP).is_empty()).await);
    assert_eq!(
        writes(&ds.logs(), UP),
        vec![REQUEST.len() as u64 + OVERHEAD]
    );
}

/// One case of the limits: `head`, then `bad`, with the checks in `off` turned off (the bites); waits until the
/// connection closes or more than `before` model bytes entered up. Returns the close reason, the up model bytes
/// written, and whether the client side saw S closed.
async fn limit_case(
    cfg: &L2Config,
    head: &[u8],
    bad: &[u8],
    off: &[Reason],
    before: u64,
) -> (Option<Reason>, u64, bool) {
    let (mut client, server) = tokio::io::duplex(1 << 16);
    let mut ds = DelayStream::wrap(server, cfg, 0, now()).expect("wrapped");
    ds.bite_check_off(off);
    // A close may drop S before the last bytes are written: what was written is what counts.
    let _ = client.write_all(head).await;
    let _ = client.write_all(bad).await;
    let up = |ds: &DelayStream<DuplexStream>| writes(&ds.logs(), UP).iter().sum::<u64>();
    assert!(
        until(HANG, || ds.probe().0 || up(&ds) > before).await,
        "hang guard: neither a close nor the payload"
    );
    let (_, closed, _, _) = ds.probe();
    let entered = up(&ds);
    let mut eof = false;
    if closed.is_some() {
        // S closed: the client's read ends.
        let mut buf = [0u8; 64];
        eof = matches!(
            tokio::time::timeout(HANG, client.read(&mut buf)).await,
            Ok(Ok(0)) | Ok(Err(_))
        );
        // tungstenite's side reads what was released, then the close.
        let mut sink = vec![0u8; 16_384];
        let mut err = None;
        for _ in 0..100 {
            match tokio::time::timeout(HANG, ds.read(&mut sink)).await {
                Ok(Ok(0)) => break,
                Ok(Ok(_)) => {}
                Ok(Err(e)) => {
                    err = Some(e.kind());
                    break;
                }
                Err(_) => break,
            }
        }
        assert_eq!(err, Some(io::ErrorKind::InvalidData));
    }
    (closed, entered, eof)
}

/// What a bite's input then does: its payload enters the model, or the close is counted under another reason (the
/// check that also guards it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Then {
    Enters,
    CountedAs(Reason),
}

/// C6: each limit closes the connection, counted by reason, before any of the bad frame's payload enters the model
/// (only the records before it were written up), and S is dropped. Bites (M2), one or more per reason, on the same
/// input: with the check off the payload enters the model, so each "nothing entered" assertion can fail. Where a
/// second check also guards the input, turning off the first alone moves the close to the second (so the reason
/// assertion fails), and the payload enters with both off:
/// - a data frame above the limit is also a message above it (Frame, then Frame and Message off);
/// - a 64-bit length with its top bit set is always above the frame limit too (wrap bounds the limit to 16,370 B):
///   with the length check off the frame check closes it, and with both off such a frame can never end, so nothing can
///   enter however the checks are set;
/// - the request cap is also the scanner's read limit, so the request bite lifts both.
///
/// Plus the limit raised in L2Config (one source) letting the 514 B frame in.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn each_limit_closes_before_buffering_and_is_counted() {
    use Reason::{Control, Frame, Length, Message, Request, Unmasked};
    type Case = (Vec<u8>, Vec<u8>, Reason, u64, Vec<(Vec<Reason>, Then)>);
    let c = cfg(quiet(Profile::p1()), 4);
    let request = REQUEST.len() as u64 + OVERHEAD;
    let mut top = frame(true, 2, true, &[], true);
    top[2] = 0x80;
    // A whole request over the 8,192 B cap.
    let mut big = b"GET / HTTP/1.1\r\nHost: localhost\r\nX-Pad: ".to_vec();
    big.resize(9_000, b'a');
    big.extend(b"\r\n\r\n");
    let fragment = frame(false, 2, true, &[1; 300], false);
    let cases: Vec<Case> = vec![
        (
            REQUEST.to_vec(),
            frame(true, 2, true, &[1; 514], false),
            Frame,
            request,
            vec![
                (vec![Frame], Then::CountedAs(Message)),
                (vec![Frame, Message], Then::Enters),
            ],
        ),
        (
            REQUEST.to_vec(),
            [fragment.clone(), frame(true, 0, true, &[1; 214], false)].concat(),
            Message,
            // The fragmented message's first frame was within the limit and entered.
            request + fragment.len() as u64 + OVERHEAD,
            vec![(vec![Message], Then::Enters)],
        ),
        (
            REQUEST.to_vec(),
            frame(true, 2, false, &[1; 10], false),
            Unmasked,
            request,
            vec![(vec![Unmasked], Then::Enters)],
        ),
        (
            REQUEST.to_vec(),
            frame(true, 9, true, &[1; 126], false),
            Control,
            request,
            vec![(vec![Control], Then::Enters)],
        ),
        (
            REQUEST.to_vec(),
            frame(false, 9, true, &[1; 5], false),
            Control,
            request,
            vec![(vec![Control], Then::Enters)],
        ),
        (
            REQUEST.to_vec(),
            top,
            Length,
            request,
            vec![(vec![Length], Then::CountedAs(Frame))],
        ),
        (
            big,
            Vec::new(),
            Request,
            0,
            vec![(vec![Request], Then::Enters)],
        ),
    ];
    for (head, bad, why, before, bites) in &cases {
        let (closed, entered, eof) = limit_case(&c, head, bad, &[], *before).await;
        assert_eq!(closed, Some(*why), "{why:?}");
        assert_eq!(
            entered, *before,
            "{why:?}: nothing of the bad frame entered"
        );
        assert!(eof, "{why:?}: S was dropped");
        for (off, then) in bites {
            let (closed, entered, _) = limit_case(&c, head, bad, off, *before).await;
            match then {
                Then::Enters => {
                    assert_eq!(closed, None, "{why:?} bite {off:?}: no close");
                    assert!(
                        entered > *before,
                        "{why:?} bite {off:?}: the payload entered ({entered} B > {before} B)"
                    );
                }
                Then::CountedAs(r) => assert_eq!(
                    (closed, entered),
                    (Some(*r), *before),
                    "{why:?} bite {off:?}"
                ),
            }
            println!(
                "{why:?}: closed before buffering ({before} B up); with {off:?} off: {then:?}, closed {closed:?}, \
                 {entered} B up"
            );
        }
    }
    // One source: the limit raised to 16,000 B in L2Config lets the 514 B frame's payload into the model.
    let mut open = c.clone();
    open.ws_max_message_bytes = 16_000;
    let (closed, entered, _) = limit_case(
        &open,
        REQUEST,
        &frame(true, 2, true, &[1; 514], false),
        &[],
        request,
    )
    .await;
    assert_eq!(closed, None);
    assert!(entered > request, "the payload entered: {entered} B");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn wrap_refuses_a_pass_through_config_and_bad_limits() {
    let (_c, s) = tokio::io::duplex(64);
    let e = DelayStream::wrap(s, &L2Config::clean(1), 0, now()).expect_err("refused");
    assert_eq!(e.kind(), io::ErrorKind::InvalidInput);
    let (_c, s) = tokio::io::duplex(64);
    let mut bad = cfg(Profile::p2(), 1);
    bad.ws_max_message_bytes = 0;
    assert!(DelayStream::wrap(s, &bad, 0, now()).is_err());
}

#[test]
fn wrap_needs_a_tokio_runtime() {
    let (_c, s) = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("a runtime")
        .block_on(async { tokio::io::duplex(64) });
    assert!(DelayStream::wrap(s, &cfg(Profile::p2(), 1), 0, 0).is_err());
}

/// C5: the base round trip is netted out on DelayStream's path through the helper ImpairSocket uses: a 100 ms profile
/// over a measured 10 ms base gives a one-way delay of (100 − 10) / 2 = 45 ms in the model, in the header and in
/// ImpairSocket's settings alike, and a lone write down (no jitter, no loss) is released exactly 45 ms after it on the
/// model's own stamps. Without the base the same write takes 50 ms (the netting is what moved it).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_base_rtt_is_netted_out_through_the_shared_helper() {
    let mut c = cfg(fixed(100), 14);
    c.base_rtt_ns = 10 * MS;
    let lone = |c: L2Config| async move {
        let (_client, server) = tokio::io::duplex(1 << 16);
        let mut ds = DelayStream::wrap(server, &c, 0, now()).expect("wrapped");
        ds.write_all(&[0x82; 100]).await.expect("written");
        assert!(until(HANG, || !ds.logs().releases.is_empty()).await);
        let s = ds.snapshot();
        let wrote = s
            .logs
            .steps
            .iter()
            .find_map(|&(t, st)| matches!(st, Step::Write { end: DOWN, .. }).then_some(t))
            .expect("the write");
        let released = s
            .logs
            .releases
            .iter()
            .find(|r| r.0 == DOWN)
            .expect("its release")
            .2;
        (
            value(&ds.header_numbers(), "ws_one_way_ns"),
            released - wrote,
        )
    };
    assert_eq!(one_way_ns(&c), 45 * MS);
    assert_eq!(Settings::of(&c).one_way_ns, 45 * MS, "ImpairSocket's");
    assert_eq!(lone(c.clone()).await, (45 * MS, 45 * MS));
    let mut no_base = c;
    no_base.base_rtt_ns = 0;
    assert_eq!(lone(no_base).await, (50 * MS, 50 * MS));
}

/// C4 on the model alone: a burst at or after the model's now is added; an earlier one, or one of no length, is
/// refused; ended bursts are dropped and at most 64 stay pending.
#[test]
fn the_model_adds_bursts_after_its_start_only() {
    let mut m = TcpModel::new(&quiet(Profile::p2()), 1, 0);
    m.advance(5 * SEC);
    assert!(!m.add_burst(4 * SEC, SEC), "before now");
    assert!(!m.add_burst(6 * SEC, 0), "no length");
    assert!(m.add_burst(5 * SEC, SEC));
    for i in 0..63 {
        assert!(m.add_burst(10 * SEC + i * SEC, MS));
    }
    assert!(!m.add_burst(100 * SEC, MS), "64 pending");
    m.advance(7 * SEC);
    assert!(m.add_burst(100 * SEC, MS), "the ended one was dropped");
    assert_eq!(m.bursts().len(), 64);
}

/// The burst and replay tests' traffic, run until `each` says stop (`max_ticks` is a hang guard): the client writes
/// the request and then a 60 B frame every tick and reads S throughout; the test writes 1,000 B down every tick and
/// reads what DelayStream releases up, then calls `each(k, ds)`. Returns the ticks run.
async fn drive(
    ds: &mut DelayStream<DuplexStream>,
    client: DuplexStream,
    max_ticks: u64,
    mut each: impl FnMut(u64, &DelayStream<DuplexStream>) -> bool,
) -> u64 {
    let (mut cr, mut cw) = tokio::io::split(client);
    let stop = Arc::new(AtomicBool::new(false));
    let stop2 = Arc::clone(&stop);
    let up = tokio::spawn(async move {
        cw.write_all(REQUEST).await.expect("the request");
        let mut iv = tokio::time::interval(Duration::from_nanos(TICK));
        while !stop2.load(Ordering::Relaxed) {
            iv.tick().await;
            if cw
                .write_all(&frame(true, 2, true, &[9; 60], false))
                .await
                .is_err()
            {
                break;
            }
        }
        cw
    });
    let rd = tokio::spawn(async move {
        let mut buf = vec![0u8; 1 << 16];
        while cr.read(&mut buf).await.is_ok_and(|n| n > 0) {}
    });
    let mut iv = tokio::time::interval(Duration::from_nanos(TICK));
    let mut buf = vec![0u8; 1 << 16];
    let mut k = 0;
    loop {
        iv.tick().await;
        ds.write_all(&[0x82; 1_000]).await.expect("down");
        while let Ok(Ok(n)) = tokio::time::timeout(Duration::ZERO, ds.read(&mut buf)).await {
            if n == 0 {
                break;
            }
        }
        if each(k, ds) {
            break;
        }
        k += 1;
        assert!(k < max_ticks, "hang guard: {max_ticks} ticks");
    }
    stop.store(true, Ordering::Relaxed);
    let _cw = up.await.expect("the client's writes");
    rd.abort();
    k + 1
}

/// C4 (W1): a burst added after the wrap loses every transmission inside it, both ways, and nothing outside it is lost
/// (quiet P2). The burst is placed and judged on the model's own stamps: it is added at the clock's now (a start the
/// model has passed is refused and retried), read back from the log, and traffic runs until the model's now is past
/// its end; if the scheduling left one direction without a transmission inside it, another is added (counted; every
/// burst judged). One atomic snapshot is judged: every logged transmission's fate against every logged burst, and the
/// model's burst-loss counters against those counts. Refused before the model's now. Typically 1 burst of 600 ms and
/// 50 ticks (1.7 s).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_added_burst_loses_every_transmission_inside_it_both_ways() {
    let (client, server) = tokio::io::duplex(1 << 20);
    let mut ds =
        DelayStream::wrap(server, &cfg(quiet(Profile::p2()), 5), 0, now()).expect("wrapped");
    assert_eq!(
        ds.add_burst(now().saturating_sub(MS), SEC)
            .expect_err("refused")
            .kind(),
        io::ErrorKind::InvalidInput
    );
    let mut added = 0u32;
    let mut after: Option<u64> = None;
    let ticks = drive(&mut ds, client, 3_000, |k, ds| {
        if let Some(a) = after {
            // Half a second of repairs after a covering burst, judged too.
            return k >= a + 15;
        }
        if k < 15 {
            return false;
        }
        let s = ds.snapshot();
        match bursts_of(&s.logs).last() {
            Some(&(start, len)) if s.now < start + len => {}
            Some(&(start, len)) if inside(&s.sent_log, start, len).iter().all(|&n| n > 0) => {
                after = Some(k);
            }
            _ => {
                assert!(added < 10, "10 bursts and no direction covered");
                add_burst_soon(ds, 600 * MS);
                added += 1;
            }
        }
        false
    })
    .await;
    let s = ds.snapshot();
    let bursts = bursts_of(&s.logs);
    assert_eq!(bursts.len() as u32, added);
    let in_any = |t: u64| bursts.iter().any(|&(b, l)| t >= b && t - b < l);
    let mut lost_inside = [0u64; 2];
    for &(end, _, _, t, lost) in &s.sent_log {
        if in_any(t) {
            assert!(lost, "a transmission at {t} inside a burst got through");
            lost_inside[end] += 1;
        } else {
            assert!(
                !lost,
                "a transmission at {t} outside the bursts was lost (quiet profile)"
            );
        }
    }
    let per: Vec<[u64; 2]> = bursts
        .iter()
        .map(|&(b, l)| inside(&s.sent_log, b, l))
        .collect();
    println!(
        "{added} burst(s) {bursts:?} over {ticks} ticks; transmissions inside each (down, up): {per:?}; all lost"
    );
    assert!(
        per.iter().any(|n| n[DOWN] > 0 && n[UP] > 0),
        "a burst covered both ways: {per:?}"
    );
    assert_eq!(
        [s.counters[DOWN].lost_burst, s.counters[UP].lost_burst],
        lost_inside
    );
}

/// The replay (S10, W2): the run's model counters and delivery stamps equal those of a pure TcpModel driven by the
/// logged writes and added bursts at their stamps: the real path moves no fate. Everything compared comes from one
/// atomic snapshot (the pump steps the model between separate readings). P2 with its loss and jitter, plus a burst
/// at tick 30; traffic runs at least 90 ticks (3 s) and until the snapshot holds a burst loss down and a profile loss,
/// so the replay always covers both (a burst that the scheduling left with no down transmission is followed by
/// another).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_replay_on_a_pure_model_gives_the_same_counters_and_stamps() {
    let c = cfg(Profile::p2(), 31);
    let (client, server) = tokio::io::duplex(1 << 20);
    let mut ds = DelayStream::wrap(server, &c, 3, now()).expect("wrapped");
    let mut added = 0u32;
    let ticks = drive(&mut ds, client, 3_000, |k, ds| {
        let s = ds.snapshot();
        let bursts = bursts_of(&s.logs);
        let ended = bursts.iter().all(|&(b, l)| s.now >= b + l);
        let k_ = s.counters;
        if k == 30 || (k > 30 && ended && k_[DOWN].lost_burst == 0) {
            assert!(added < 10, "10 bursts and no down transmission in one");
            add_burst_soon(ds, 400 * MS);
            added += 1;
            return false;
        }
        k >= 90
            && ended
            && k_[DOWN].lost_burst > 0
            && k_[DOWN].lost_profile + k_[UP].lost_profile > 0
    })
    .await;
    let s = ds.snapshot();
    let mut m = TcpModel::new(&c.profile, c.seed, 3).with_one_way_ns(one_way_ns(&c));
    let mut ends: [VecDeque<u64>; 2] = [VecDeque::new(), VecDeque::new()];
    let mut written = [0u64; 2];
    let mut releases = Vec::new();
    /// DelayStream's stepping on a pure model: event by event, each delivered record released at the event's stamp.
    fn step(
        m: &mut TcpModel,
        ends: &mut [VecDeque<u64>; 2],
        t: u64,
        out: &mut Vec<(usize, u64, u64)>,
    ) {
        while let Some(x) = m.next_event().filter(|&x| x <= t) {
            m.advance(x);
            for dir in [DOWN, UP] {
                while ends[dir]
                    .front()
                    .is_some_and(|&e| e <= m.delivered(1 - dir))
                {
                    out.push((dir, ends[dir].pop_front().unwrap_or_default(), x));
                }
            }
        }
        m.advance(t);
    }
    for &(t, st) in &s.logs.steps {
        step(&mut m, &mut ends, t, &mut releases);
        match st {
            Step::Write { end, bytes } => {
                assert_eq!(m.write(end, bytes, t), bytes);
                written[end] += bytes;
                ends[end].push_back(written[end]);
            }
            Step::Burst { start, len } => assert!(m.add_burst(start, len)),
        }
    }
    step(&mut m, &mut ends, s.now, &mut releases);
    let of = |v: &[(usize, u64, u64)], d: usize| -> Vec<(u64, u64)> {
        v.iter().filter(|r| r.0 == d).map(|r| (r.1, r.2)).collect()
    };
    for d in [DOWN, UP] {
        assert_eq!(of(&releases, d), of(&s.logs.releases, d), "dir {d}");
    }
    assert_eq!([m.counters(DOWN), m.counters(UP)], s.counters);
    assert_eq!([m.delivered(1), m.delivered(0)], s.delivered);
    let n = |c: &Counters| (c.transmissions, c.lost_profile, c.lost_burst, c.stalls);
    println!(
        "replay: {ticks} ticks, {added} burst(s), {} steps, {} releases; down {:?} up {:?} (transmissions, lost, burst, \
         stalls)",
        s.logs.steps.len(),
        s.logs.releases.len(),
        n(&s.counters[DOWN]),
        n(&s.counters[UP])
    );
    assert!(
        s.counters[DOWN].lost_burst > 0
            && s.counters[DOWN].lost_profile + s.counters[UP].lost_profile > 0
    );
}

/// C3 (W5): dropping DelayStream ends the pump and drops S at once, and its clock entry is gone. Judged on state: the
/// client reads exactly the down bytes S took before the drop (none with the 1 s one-way delay, unless the process
/// stalled past it and the model released the record first), then EOF, which only S's drop can give; no clock time
/// is judged.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn dropping_it_ends_the_pump_and_drops_s() {
    let (mut client, server) = tokio::io::duplex(1 << 16);
    let mut ds = DelayStream::wrap(server, &cfg(fixed(2_000), 6), 0, now()).expect("wrapped");
    ds.write_all(&[0x82; 4_000]).await.expect("written");
    let (sh, key) = (Arc::clone(&ds.sh), ds.clock_key());
    assert!(until(HANG, || delay_clock::clock().census(key).3).await);
    drop(ds);
    assert!(until(HANG, || sh.lock().pump_done).await);
    let taken = {
        let c = sh.lock();
        c.written[DOWN] - c.bytes[DOWN].len() as u64
    };
    let mut got = 0u64;
    let mut buf = [0u8; 4_096];
    loop {
        let n = tokio::time::timeout(HANG, client.read(&mut buf))
            .await
            .expect("hang guard: S closed")
            .expect("read");
        if n == 0 {
            break;
        }
        got += n as u64;
    }
    println!("S took {taken} B before the drop; the client read {got} B, then EOF");
    assert_eq!(got, taken, "the client read what S took, then EOF");
    assert!(!delay_clock::clock().census(key).3);
}

/// C3: shutdown with every byte delivered and taken shuts S down at once (no timeout counted); with a client that
/// never reads, it gives up at the deadline (300 ms here), drops S and counts it.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn shutdown_waits_for_delivery_and_at_most_its_deadline() {
    let (client, server) = tokio::io::duplex(1 << 20);
    let mut ds =
        DelayStream::wrap(server, &cfg(quiet(Profile::p1()), 7), 0, now()).expect("wrapped");
    let (mut cr, _cw) = tokio::io::split(client);
    let rd = tokio::spawn(async move {
        let mut all = Vec::new();
        let mut buf = vec![0u8; 1 << 16];
        while let Ok(n) = cr.read(&mut buf).await {
            if n == 0 {
                break;
            }
            all.extend_from_slice(&buf[..n]);
        }
        all.len()
    });
    ds.write_all(&[0x82; 10_000]).await.expect("written");
    tokio::time::timeout(HANG, ds.shutdown())
        .await
        .expect("a hang guard only")
        .expect("shut down");
    assert_eq!(rd.await.expect("read"), 10_000, "every byte, then EOF");
    assert_eq!(value(&ds.run_summary(), names::SHUTDOWN_TIMEOUT), 0);

    let (_client, server) = tokio::io::duplex(512);
    let mut ds =
        DelayStream::wrap(server, &cfg(quiet(Profile::p1()), 8), 0, now()).expect("wrapped");
    ds.set_shutdown_wait(300 * MS);
    ds.write_all(&[0x82; 20_000]).await.expect("written");
    tokio::time::timeout(HANG, ds.shutdown())
        .await
        .expect("a hang guard only")
        .expect("given up");
    assert_eq!(value(&ds.run_summary(), names::SHUTDOWN_TIMEOUT), 1);
    assert!(ds.probe().0, "S dropped, the pump ended");
}

/// An S that takes every byte but whose own shutdown never completes (Pending, and it never wakes anyone); counts the
/// calls to its poll_shutdown.
struct NeverShut {
    io: DuplexStream,
    calls: Arc<AtomicU64>,
}

impl AsyncRead for NeverShut {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.io).poll_read(cx, buf)
    }
}

impl AsyncWrite for NeverShut {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.io).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.io).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        Poll::Pending
    }
}

/// C3 (W3, M1): the 10 s bound covers S's own shutdown, without spinning. S takes every byte (all delivered and
/// acknowledged: the model is idle) and never completes its shutdown. The shutdown ends at its deadline (300 ms here)
/// with a counted timeout, S dropped (the client reads EOF); the pump ended at its first run at or after the deadline,
/// having given S the whole wait. Counted, not timed: while it waits, nothing but the shutdown's own wake and the
/// deadline's can run the pump (the model is idle, S never wakes), so the pump runs at most 3 times (2 expected; one
/// more allowed for a wake already queued when the wait began) and S's poll_shutdown no more often. The old code
/// spun here, re-arming the past deadline at once: thousands of runs and no end.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning() {
    let (client, server) = tokio::io::duplex(1 << 20);
    let calls = Arc::new(AtomicU64::new(0));
    let s = NeverShut {
        io: server,
        calls: Arc::clone(&calls),
    };
    let mut ds = DelayStream::wrap(s, &cfg(quiet(Profile::p1()), 13), 0, now()).expect("wrapped");
    ds.set_shutdown_wait(300 * MS);
    let (mut cr, _cw) = tokio::io::split(client);
    let rd = tokio::spawn(async move {
        let mut n = 0usize;
        let mut buf = vec![0u8; 1 << 16];
        while let Ok(k) = cr.read(&mut buf).await {
            if k == 0 {
                break;
            }
            n += k;
        }
        n
    });
    ds.write_all(&[0x82; 10_000]).await.expect("written");
    assert!(
        until(HANG, || ds.model_idle()
            && ds.snapshot().taken_down == 10_000)
        .await,
        "hang guard: every byte taken and acknowledged"
    );
    let before = ds.life();
    let shut = tokio::time::timeout(HANG, ds.shutdown()).await;
    let life = ds.life();
    let polls = life.polls - before.polls;
    let s_calls = calls.load(Ordering::Relaxed);
    shut.unwrap_or_else(|_| {
        panic!("hang guard: no end in 30 s; the pump ran {polls} times, S's poll_shutdown {s_calls} times")
    })
    .expect("given up at the deadline");
    println!(
        "S never shuts down: the pump ran {polls} times during the wait ({} at or after the deadline), S's \
         poll_shutdown {s_calls} times; ended {} ns after the deadline",
        life.polls_after_deadline,
        life.ended_ns.unwrap_or(0) - life.deadline.unwrap_or(0)
    );
    assert_eq!(value(&ds.run_summary(), names::SHUTDOWN_TIMEOUT), 1);
    assert!(ds.probe().0, "the pump ended");
    let deadline = life.deadline.expect("a deadline");
    let ended = life.ended_ns.expect("an end stamp");
    assert!(ended >= deadline, "S had the whole wait");
    assert_eq!(
        life.polls_after_deadline, 1,
        "ended at its first run at or after the deadline"
    );
    assert!(polls <= 3, "no spin while waiting: {polls} runs");
    assert!(s_calls <= polls, "{s_calls} shutdown calls");
    assert_eq!(rd.await.expect("read"), 10_000, "every byte, then EOF");
}

/// C3: 1,000 connections opened, given traffic and closed: every pump ends, none of their keys is left in the
/// clock's schedule or table, and the clock holds at most one entry per live connection throughout (other tests may
/// run beside this one in the process, so the check is on these keys and on the invariant).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_thousand_opens_and_closes_leave_the_clock_tables_empty() {
    let clock = delay_clock::clock();
    let mut keys = Vec::new();
    for i in 0..1_000u32 {
        let (_client, server) = tokio::io::duplex(1 << 12);
        let mut ds =
            DelayStream::wrap(server, &cfg(quiet(Profile::p2()), 9), i, now()).expect("wrapped");
        ds.write_all(&[0x82; 100]).await.expect("written");
        let (sh, key) = (Arc::clone(&ds.sh), ds.clock_key());
        let (sched, table, live, _) = clock.census(key);
        assert!(
            sched == table && (sched as u64) <= live,
            "{sched} {table} {live}"
        );
        drop(ds);
        for _ in 0..1_000 {
            if sh.lock().pump_done {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert!(until(HANG, || sh.lock().pump_done).await);
        keys.push(key);
    }
    for key in keys {
        assert!(!clock.census(key).3, "key {key} left in the clock");
    }
    let (sched, table, live, _) = clock.census(0);
    assert!(sched == table && (sched as u64) <= live);
}

/// Every row DelayStream writes is named in SERIES.md through delay_names (whose own test reads SERIES.md), and the
/// rows carry every name once.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn its_rows_use_the_named_series() {
    let (_client, server) = tokio::io::duplex(1 << 12);
    let ds = DelayStream::wrap(server, &cfg(Profile::p2(), 10), 7, 42).expect("wrapped");
    let second = ds.take_second();
    let summary = ds.run_summary();
    let mut want = Vec::new();
    names::window(
        [[0; 20]; 2],
        [[0; 2]; 2],
        [Summary::default(); 2],
        &[0; REASONS + 3],
        &mut want,
    );
    let names_of = |v: &[(&'static str, u64)]| v.iter().map(|x| x.0).collect::<Vec<_>>();
    assert_eq!(names_of(&second), names_of(&want));
    assert_eq!(&names_of(&summary)[..want.len()], &names_of(&want)[..]);
    assert_eq!(value(&summary, names::CONNS_BEFORE), 7);
    assert_eq!(value(&summary, names::ACCEPTED), 42);
    let h = ds.header_numbers();
    assert_eq!(names_of(&h), names::HEADER_NUMBERS.to_vec());
    assert_eq!(value(&h, "ws_wrapper"), 1);
    assert_eq!(value(&h, "ws_one_way_ns"), 50 * MS);
    assert_eq!(value(&h, "ws_max_message_bytes"), 513);
    assert_eq!(value(&h, "ws_up_read_floor_bytes"), 549);
    let t = ds.header_texts();
    assert_eq!(t[0], ("ws_tcp_model", "linux-6.8-tcp v3".to_owned()));
    for (n, _) in &t {
        assert!(names::HEADER_TEXTS.contains(n));
    }
    let r = Return {
        at_ns: 1,
        outstanding: 2,
        srtt_ns: None,
        untaken: 3,
        delivered: [0; 2],
        first_byte_ns: [Some(4), None],
        first_message_ns: [None, Some(5)],
    };
    assert_eq!(names_of(&r.row()), names::RETURN.to_vec());
}

// ---------------------------------------------------------------------------------------------------------------
// Over real TLS and tokio-tungstenite on loopback.

/// TLS 1.3 only, ALPN http/1.1 (a client offering only h3 is refused), the ring provider, no process default; and the
/// tungstenite configuration the server runs is ws_config's from L2Config (the one source: ws_config's own tests).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_websocket_tls_is_tls13_with_http11_on_ring_and_no_process_default() {
    use tokio_rustls::rustls::crypto::CryptoProvider;
    assert!(CryptoProvider::get_default().is_none());
    let (cert, key) = self_signed(&ws_config::provider());
    let tls = ws_config::tls_config(vec![cert.clone()], key.clone_key()).expect("built");
    assert_eq!(tls.alpn_protocols, vec![b"http/1.1".to_vec()]);
    let groups = |p: &CryptoProvider| p.kx_groups.iter().map(|g| g.name()).collect::<Vec<_>>();
    let suites = |p: &CryptoProvider| {
        p.cipher_suites
            .iter()
            .map(|s| s.suite())
            .collect::<Vec<_>>()
    };
    let ring = ws_config::provider();
    assert_eq!(groups(tls.crypto_provider()), groups(&ring));
    assert_eq!(suites(tls.crypto_provider()), suites(&ring));
    // A client offering only h3 is refused.
    let acceptor = ws_config::acceptor(vec![cert.clone()], key).expect("acceptor");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a port");
    let addr = listener.local_addr().expect("addr");
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.expect("accepted");
        acceptor.accept(tcp).await.is_ok()
    });
    let tcp = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connected");
    let connector =
        tokio_rustls::TlsConnector::from(Arc::new(client_tls(ws_config::provider(), cert, b"h3")));
    let name = ServerName::try_from("localhost").expect("a name");
    assert!(
        connector.connect(name, tcp).await.is_err(),
        "ALPN h3 refused"
    );
    assert!(!server.await.expect("the server task"));
    // A real session negotiates TLS 1.3 and http/1.1, and its server runs ws_config's limits.
    let (s, _c, _, negotiated) = pair(&cfg(quiet(Profile::p1()), 12), 0).await;
    assert_eq!(
        negotiated,
        "Some(TLSv1_3) Some([104, 116, 116, 112, 47, 49, 46, 49])"
    );
    assert_eq!(ws_config::one_source(s.get_config(), 513), Ok(()));
    assert!(CryptoProvider::get_default().is_none(), "still none");
}

/// 1.5's loopback run: TLS 1.3 (tokio-rustls, ring) + DelayStream + tokio-tungstenite at P2, 30 Hz both ways for
/// `ticks` ticks (1,024 B down, 60 B up). Judged on identities:
/// - every message arrives, in order; the 101 left the model one least delay after the request's release, and the
///   client had it only after;
/// - each message's release is at or after its write plus the profile's least one-way delay;
/// - up, the receipt identity: each message's receipt equals its model delivery stamp plus the release lateness
///   DelayStream measured for it, within the measurement's own resolution. DelayStream hands one record per read, so
///   the poll that yields a message is the one that took its last byte: the hand-over stamp (delivery + lateness) lies
///   inside that poll's own [start, end] stamps, and the receipt is the poll's end;
/// - down, the bytes leave DelayStream into the kernel's real TCP and the client's TLS, which no in-process stamp
///   sees, so the receipt is a bound (never before the release) and the rest of the path is printed;
/// - every stall is a first-transmission loss and their count sits in its 99.9 % two-sided binomial band at 2 %.
///
/// Lateness is printed with condition 10's trigger. Returns the number of lateness samples per direction.
async fn loopback(ticks: u64) -> [u64; 2] {
    let c = cfg(Profile::p2(), 21);
    let request_at = now();
    let (mut server, mut client, got101, _) = pair(&c, 0).await;
    let client_task = tokio::spawn(async move {
        let mut recv = Vec::new();
        run_ticks(
            &mut client,
            ticks,
            Duration::from_secs(3),
            |b, r| recv.push((unstamp(b), r)),
            |k, _| Some(stamped(k, 60)),
        )
        .await;
        recv
    });
    let mut recv_up = Vec::new();
    run_ticks(
        &mut server,
        ticks,
        Duration::from_secs(3),
        |b, r| recv_up.push((unstamp(b), r)),
        |k, _| Some(stamped(k, 1_024)),
    )
    .await;
    let recv_down = client_task.await.expect("the client");
    let ds = server.get_ref();
    let s = ds.snapshot();
    let logs = &s.logs;
    let least = c.profile.one_way_ns() - c.profile.half_jitter_ns();
    // The 101: written after the request's release, released at least one least delay later, received after that.
    let first = |d: usize| {
        logs.releases
            .iter()
            .find(|r| r.0 == d)
            .expect("a release")
            .2
    };
    let (req, resp) = (first(UP), first(DOWN));
    assert!(s.origin + req >= request_at);
    assert!(
        resp >= req + least,
        "the 101 left the model a delay after the request"
    );
    assert!(
        got101 >= s.origin + resp,
        "the client had the 101 only after its release"
    );
    for (dir, recv) in [(DOWN, &recv_down), (UP, &recv_up)] {
        let idx: Vec<u64> = recv.iter().map(|r| r.0.0).collect();
        assert_eq!(
            idx,
            (0..ticks).collect::<Vec<_>>(),
            "dir {dir}: every message, in order"
        );
        let rel = &logs.messages[dir];
        assert!(rel.len() as u64 >= ticks);
        let (mut rest, mut window) = (Vec::new(), Vec::new());
        for (k, &((_, wrote), got)) in recv.iter().enumerate() {
            let r = rel[k];
            assert!(
                r >= wrote + least,
                "dir {dir} message {k}: released before a least delay"
            );
            assert!(
                got.t1 >= r,
                "dir {dir} message {k}: received before its release"
            );
            if dir == UP {
                let (handed, late) = logs.handed_up[k];
                assert_eq!(
                    handed,
                    r + late,
                    "message {k}: hand-over = delivery + lateness"
                );
                assert!(
                    got.t0 <= handed && handed <= got.t1,
                    "message {k}: handed at {handed}, outside the poll that yielded it [{}, {}]",
                    got.t0,
                    got.t1
                );
                rest.push(got.t1 - handed);
                window.push(got.t1 - got.t0);
            } else {
                rest.push(got.t1 - r);
            }
        }
        if dir == UP {
            println!(
                "dir {dir}: receipt − (delivery + lateness): {}; the poll's own window: {}",
                quantiles(&mut rest),
                quantiles(&mut window)
            );
        } else {
            println!(
                "dir {dir}: receipt − release (lateness, the kernel's TCP, TLS and tungstenite): {}",
                quantiles(&mut rest)
            );
        }
        let k = &s.counters[dir];
        assert_eq!(
            k.stalls, k.first_lost,
            "dir {dir}: every first-transmission loss stalls, nothing else does"
        );
        let (lo, hi) = band(k.segments, 0.02, 0.999);
        println!(
            "dir {dir}: {} segments, {} stalls (99.9 % band {lo}-{hi}), repairs rack {} probe {} timeout {}",
            k.segments, k.stalls, k.repairs_rack, k.repairs_probe, k.repairs_timeout
        );
        assert!(
            (lo..=hi).contains(&k.stalls),
            "dir {dir}: {} stalls outside {lo}-{hi}",
            k.stalls
        );
    }
    let sum = ds.run_summary();
    let mut n = [0u64; 2];
    for (d, name) in [(DOWN, "down"), (UP, "up")] {
        let get = |i: usize| value(&sum, names::LATE[d][i]);
        let l = Summary {
            n: get(0),
            p50: get(1),
            p99: get(2),
            p999: get(3),
            max: get(4),
            excess: get(5),
            over: get(6),
        };
        n[d] = l.n;
        println!(
            "{}",
            late_line(
                &format!(
                    "lateness {name} (P2, 30 Hz, {:.1} s)",
                    (ticks * TICK) as f64 / 1e9
                ),
                &l
            )
        );
    }
    n
}

/// The loopback run as check.mjs runs it: 300 ticks (10 s).
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn loopback_tls_tungstenite_at_p2_30_hz_both_ways() {
    loopback(300).await;
}

/// Condition 10's evidence run (the main session's, quiet, nothing else alongside): the same run with at least 10,000
/// lateness samples per direction. `NPT_WS_LATENESS_TICKS` sets the ticks (default 10,050, about 5.6 min). The load
/// average is printed at the start and the end.
///   cargo +1.99.1 test --release --locked -p npt-net --lib l2::delay::tests::lateness_evidence_run -- --ignored
///   --nocapture --exact
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "an evidence run of about 5.6 min, for the main session (condition 10)"]
async fn lateness_evidence_run() {
    let ticks = std::env::var("NPT_WS_LATENESS_TICKS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10_050u64);
    let load = || std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    println!("loadavg at the start: {}", load().trim());
    let n = loopback(ticks).await;
    println!("loadavg at the end: {}", load().trim());
    println!("lateness samples: down {} up {}", n[DOWN], n[UP]);
    assert!(n.iter().all(|&x| x >= 10_000.min(ticks)), "{n:?}");
}

/// The tick writer and reader of the split test: `n` messages of `len` B at 30 Hz into `tx`.
async fn tick_writer<T>(mut tx: T, n: u64, len: usize) -> T
where
    T: Sink<Message> + Unpin,
    T::Error: std::fmt::Debug,
{
    let mut iv = tokio::time::interval(Duration::from_nanos(TICK));
    for k in 0..n {
        iv.tick().await;
        tx.send(Message::Binary(stamped(k, len).into()))
            .await
            .expect("sent");
    }
    tx
}

/// Reads `n` binary messages from `rx`: each one's (index, send stamp) and its receipt.
async fn tick_reader<T>(mut rx: T, n: u64) -> (T, Vec<((u64, u64), u64)>)
where
    T: Stream<Item = Result<Message, tungstenite::Error>> + Unpin,
{
    let mut got = Vec::new();
    while (got.len() as u64) < n {
        match tokio::time::timeout(HANG, rx.next())
            .await
            .expect("hang guard")
        {
            Some(Ok(Message::Binary(b))) => got.push((unstamp(&b), now())),
            Some(Ok(_)) => {}
            other => panic!("the connection ended: {other:?}"),
        }
    }
    (rx, got)
}

/// The advisor's split test (2026-11-08): each end's WebSocket split into its read and write halves, each driven by
/// its own task (as a WsServer likely will), over TLS and DelayStream at P2 with its loss: 90 messages each way at
/// 30 Hz (3 s). Every message arrives, in order; none is released before a least delay after its write nor received
/// before its release, and each up message's hand-over is delivery + lateness, before its receipt. DelayStream keeps
/// separate reader, writer and shutter wakers, so neither half's wake is lost to the other.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn split_halves_in_separate_tasks_both_ways() {
    const TICKS: u64 = 90;
    let c = cfg(Profile::p2(), 23);
    let least = c.profile.one_way_ns() - c.profile.half_jitter_ns();
    let (server, client, _, _) = pair(&c, 0).await;
    let (s_tx, s_rx) = server.split();
    let (c_tx, c_rx) = client.split();
    let tasks = (
        tokio::spawn(tick_writer(s_tx, TICKS, 1_024)),
        tokio::spawn(tick_reader(s_rx, TICKS)),
        tokio::spawn(tick_writer(c_tx, TICKS, 60)),
        tokio::spawn(tick_reader(c_rx, TICKS)),
    );
    let s_tx = tasks.0.await.expect("the server's writer");
    let (s_rx, up) = tasks.1.await.expect("the server's reader");
    let _c_tx = tasks.2.await.expect("the client's writer");
    let (_c_rx, down) = tasks.3.await.expect("the client's reader");
    let server = s_tx.reunite(s_rx).expect("one stream");
    let s = server.get_ref().snapshot();
    for (dir, got) in [(DOWN, &down), (UP, &up)] {
        let idx: Vec<u64> = got.iter().map(|g| g.0.0).collect();
        assert_eq!(idx, (0..TICKS).collect::<Vec<_>>(), "dir {dir}: in order");
        for (k, &((_, wrote), at)) in got.iter().enumerate() {
            let r = s.logs.messages[dir][k];
            assert!(r >= wrote + least, "dir {dir} message {k}: early release");
            assert!(
                at >= r,
                "dir {dir} message {k}: received before its release"
            );
            if dir == UP {
                let (handed, late) = s.logs.handed_up[k];
                assert_eq!(handed, r + late);
                assert!(handed <= at, "message {k}: received before its hand-over");
            }
        }
    }
    println!(
        "split: {TICKS} messages each way in order; segments down {} up {}, stalls down {} up {}",
        s.counters[DOWN].segments,
        s.counters[UP].segments,
        s.counters[DOWN].stalls,
        s.counters[UP].stalls
    );
}

/// Mechanism test (c)'s connection: the server's shipped per-tick writer (with `bite`, or reading unsent bytes only)
/// writes one 1,024 B snapshot (1,050 B in the model) per tick over TLS and tokio-tungstenite at P2, through two 1 s
/// bursts added after the wrap at `bursts` from the start. Each write goes through the Sink without waiting (fed, then
/// one flush attempt), as a WsServer's tick must never park: what tungstenite cannot pass on stays in its buffer, and
/// DelayStream counts it as owed. Returns the per-return records with N at each return's srtt, the writes, the skips,
/// and the writes after which bytes were left owed.
async fn writer_run(
    seed: u64,
    conn: u32,
    bite: Option<Bite>,
    unsent_only: bool,
    bursts: [u64; 2],
    ticks: u64,
) -> (Vec<(Return, u64)>, u64, u64, u64) {
    let c = cfg(Profile::p2(), seed);
    let (mut server, mut client, _, _) = pair(&c, conn).await;
    let reader = tokio::spawn(async move { while let Some(Ok(_)) = client.next().await {} });
    let start = now();
    for b in bursts {
        server.get_ref().add_burst(start + b, SEC).expect("a burst");
    }
    let mut w = TickWriter::new(TICK);
    if let Some(b) = bite {
        w = w.with_bite(b);
    }
    let mut owed_after = 0;
    let t0 = tokio::time::Instant::now();
    for k in 0..ticks {
        read_until(
            &mut server,
            t0 + Duration::from_nanos(k * TICK),
            &mut |_, _| {},
        )
        .await;
        // Whatever tungstenite still holds gets one more chance to go (none is expected: the gate needs owed 0).
        let _ = server.flush().now_or_never();
        let ds = server.get_ref();
        let inp = Inputs {
            srtt_ns: ds.srtt_ns(),
            outstanding: if unsent_only {
                ds.unsent_bytes()
            } else {
                ds.outstanding_bytes()
            },
            owed: ds.owed_bytes(),
            free: ds.free_bytes(),
        };
        if w.on_tick(Snap { id: k, size: SNAP }, inp).is_some() {
            let _ = server
                .feed(Message::Binary(vec![0x5a; 1_024].into()))
                .now_or_never();
            let _ = server.flush().now_or_never();
            owed_after += u64::from(server.get_ref().owed_bytes() != 0);
        }
    }
    let rets = server
        .get_ref()
        .returns()
        .into_iter()
        .map(|r| (r, snapshot_budget(r.srtt_ns, TICK)))
        .collect();
    drop(server);
    reader.abort();
    (rets, w.written(), w.skipped(), owed_after)
}

/// Mechanism test (c) (u3c-plan.md 1.4, C9): on DelayStream at P2 under B1, through TLS 1.3 and tokio-tungstenite on
/// loopback, the shipped writer keeps outstanding <= N(srtt then) x size at every return (from the per-return record)
/// and leaves nothing owed after any write. Counts: 2 faithful connections and one per bite, run together; 2 bursts
/// each (1 s, at 1.5 s and 4.5 s plus a phase of 0, 11, 17 or 29 ms); 230 ticks (7.7 s). Identities, so no band. Bites
/// (d): budget_scale 2 and "outstanding = unsent only" (SIOCOUTQNSD) each break it at a return.
///
/// What can fail and what cannot (the critic's note): "owed = 0 before a write" is not asserted, since the gate
/// (may_write) requires it and it could never fail here; what is asserted is that nothing is owed AFTER each write,
/// which fails if a snapshot the gate let through did not fit DelayStream's room whole (SNAP counting the frame
/// wrongly, or a record rule that takes less than the room allows).
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn c_the_writer_keeps_outstanding_within_n_snapshots_at_every_return() {
    const TICKS: u64 = 230;
    let at = |phase_ms: u64| [1_500 * MS + phase_ms * MS, 4_500 * MS + phase_ms * MS];
    let (a, b, x2, unsent) = tokio::join!(
        writer_run(41, 0, None, false, at(0), TICKS),
        writer_run(42, 1, None, false, at(11), TICKS),
        writer_run(43, 2, Some(Bite::BudgetX2), false, at(17), TICKS),
        writer_run(44, 3, None, true, at(29), TICKS),
    );
    let over = |rets: &[(Return, u64)]| {
        rets.iter()
            .filter(|(r, n)| r.outstanding > n * SNAP)
            .count()
    };
    for (name, (rets, written, skipped, owed)) in [
        ("faithful A", &a),
        ("faithful B", &b),
        ("budget x2", &x2),
        ("unsent only", &unsent),
    ] {
        let shown: Vec<String> = rets
            .iter()
            .map(|(r, n)| format!("{} B vs {} B (N {n})", r.outstanding, n * SNAP))
            .collect();
        println!(
            "(c) {name}: {written} written, {skipped} skipped, owed after a write {owed}; at each return: {}",
            shown.join("; ")
        );
    }
    let least = Profile::p2().one_way_ns() - Profile::p2().half_jitter_ns();
    for (rets, _, _, owed) in [&a, &b] {
        assert_eq!(rets.len(), 2, "both bursts returned");
        assert_eq!(over(rets), 0, "outstanding within N x size at every return");
        assert_eq!(*owed, 0, "nothing owed after any write");
        // S12's per-burst times (C8): the first in-order byte down comes at least one least delay after the return
        // (anything sent in the burst was lost), and the first whole message no earlier than it.
        for (r, _) in rets.iter() {
            let byte = r.first_byte_ns[DOWN].expect("a byte after the return");
            let message = r.first_message_ns[DOWN].expect("a message after the return");
            println!(
                "(c) return at {} ns: first byte {:.1} ms, first message {:.1} ms after it",
                r.at_ns,
                byte as f64 / 1e6,
                message as f64 / 1e6
            );
            assert!(byte >= least && message >= byte, "{byte} {message}");
        }
    }
    assert!(over(&x2.0) > 0, "the budget x2 bite breaks (c)");
    assert!(over(&unsent.0) > 0, "the unsent-only bite breaks (c)");
}
