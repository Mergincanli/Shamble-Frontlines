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
use crate::ws_gate::{Bite, Snap, TickWriter, snapshot_budget};
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
/// that start before the call stepped it (C4's refusal), so a stalled test thread cannot place it in the past. Returns
/// the start it asked for (DelayStream's clock).
fn add_burst_soon<S>(ds: &DelayStream<S>, len_ns: u64) -> u64 {
    for _ in 0..1_000 {
        let start = now() + MS;
        if ds.add_burst(start, len_ns).is_ok() {
            return start;
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

/// One part of the per-release split, from the rows: reported, never judged (no trigger).
fn part_line(name: &str, s: &Summary) -> String {
    format!(
        "{name}: p50 {:.3} ms p99 {:.3} ms p99.9 {:.3} ms max {:.3} ms E {:.1} µs over {}",
        s.p50 as f64 / 1e6,
        s.p99 as f64 / 1e6,
        s.p999 as f64 / 1e6,
        s.max as f64 / 1e6,
        s.excess as f64 / 1e3,
        s.over,
    )
}

/// The split's lines for direction `d` from the run's split rows (`DelayStream::split_rows`: pooled over the process's
/// connections), as the evidence run prints them.
fn split_lines(rows: &[(&'static str, u64)], d: usize) -> Vec<String> {
    let class = |p: usize| value(rows, names::SPLIT_COUNT[d][usize::from(p >= WAIT)]);
    let part = |p: usize| {
        let g = |i: usize| value(rows, names::SPLIT[d][p][i]);
        Summary {
            n: class(p),
            p50: g(0),
            p99: g(1),
            p999: g(2),
            max: g(3),
            excess: g(4),
            over: g(5),
        }
    };
    let kind = |k: usize| value(rows, names::SPLIT_KIND[d][k]);
    let mut v = vec![format!(
        "split (pooled): woken n {} (clock + hop + work = L), other n {} (wait + work = L; open {} mid {} app {}); \
         violations {}",
        class(CLOCK),
        class(WAIT),
        kind(OPEN),
        kind(MID),
        kind(APP),
        value(rows, names::SPLIT_VIOLATIONS[d])
    )];
    for (p, label) in [
        (CLOCK, "woken clock W − M"),
        (HOP, "woken hop P − W"),
        (WORK, "woken work (3) − P"),
        (WAIT, "other wait T − M"),
        (OTHER_WORK, "other work (3) − T"),
    ] {
        v.push(part_line(label, &part(p)));
    }
    v
}

/// Direction `d`'s `k` largest releases by L from the per-release log, each with its class, its kind and its parts in
/// ms: which part made the rare maxima (printed, never judged).
fn largest(splits: &[SplitLog], d: usize, k: usize) -> Vec<String> {
    let mut v: Vec<&SplitLog> = splits.iter().filter(|r| r.dir == d).collect();
    v.sort_unstable_by_key(|r| std::cmp::Reverse(r.total));
    let ms = |ns: u64| ns as f64 / 1e6;
    v.iter()
        .take(k)
        .map(|r| match r.woke {
            Some(_) => format!(
                "largest: L {:.3} ms = woken: clock {:.3} + hop {:.3} + work {:.3} ms",
                ms(r.total),
                ms(r.parts[0]),
                ms(r.parts[1]),
                ms(r.parts[2])
            ),
            None => format!(
                "largest: L {:.3} ms = other ({}): wait {:.3} + work {:.3} ms",
                ms(r.total),
                ["open", "mid", "app"][r.kind],
                ms(r.parts[0]),
                ms(r.parts[1])
            ),
        })
        .collect()
}

/// One connection's per-release split as its log records it, per direction: the (woken, other) class counts, the
/// other class's kinds, and the releases whose recorded parts do not sum to their total.
#[derive(Debug, Default, Clone, Copy)]
struct Tally {
    n: [[u64; 2]; 2],
    kinds: [[u64; KINDS]; 2],
    broken: [u64; 2],
}

fn tally(s: &Snapshot) -> Tally {
    let mut t = Tally::default();
    for r in &s.logs.splits {
        match r.woke {
            Some(_) => t.n[r.dir][0] += 1,
            None => {
                t.n[r.dir][1] += 1;
                t.kinds[r.dir][r.kind] += 1;
            }
        }
        if r.parts.iter().sum::<u64>() != r.total {
            t.broken[r.dir] += 1;
        }
    }
    t
}

/// The per-release split's identity (the advisor's "condition 10 again", 2026-11-08) over one atomic snapshot: every
/// release logged has its parts summing to its total in integer ns (woken: clock + hop + work; other: wait + work, no
/// third part), its total equal to (3) − M, and its stamps chained (woken: M <= W <= P <= (3), from an opening step;
/// other: M <= T <= (3)), so no part saturated; per direction the classes partition the releases the connection's
/// total histogram counted, and the connection counted no violation. Returns `[direction][woken, other]`, or the first
/// release or count that breaks it. (The parts' histograms are the process's pool, shared by every connection: the
/// identity test checks they grew by at least what the log holds.)
fn split_identity(s: &Snapshot) -> Result<[[u64; 2]; 2], String> {
    let mut n = [[0u64; 2]; 2];
    for (i, r) in s.logs.splits.iter().enumerate() {
        let sum: u64 = r.parts.iter().sum();
        if sum != r.total {
            return Err(format!(
                "release {i} {r:?}: parts sum to {sum} ns, total {} ns",
                r.total
            ));
        }
        if r.due > r.at || r.total != r.at - r.due {
            return Err(format!("release {i} {r:?}: total is not (3) − M"));
        }
        match r.woke {
            Some(w) => {
                if !(r.due <= w && w <= r.step && r.step <= r.at) || r.kind != OPEN {
                    return Err(format!("release {i} {r:?}: woken stamps out of order"));
                }
                n[r.dir][0] += 1;
            }
            None => {
                if !(r.due <= r.step && r.step <= r.at) || r.parts[2] != 0 {
                    return Err(format!("release {i} {r:?}: other stamps out of order"));
                }
                n[r.dir][1] += 1;
            }
        }
    }
    for d in [DOWN, UP] {
        if n[d][0] + n[d][1] != s.late[d].n {
            return Err(format!(
                "dir {d}: woken {} + other {} != the total's {}",
                n[d][0], n[d][1], s.late[d].n
            ));
        }
    }
    if s.split_violations != [0, 0] {
        return Err(format!("violations counted: {:?}", s.split_violations));
    }
    Ok(n)
}

/// The process's pool grew, from `before` to `after`, by at least the releases in `t` (other connections in the process
/// only add): each part's count by its class's, the kinds by theirs, the violations by the broken releases.
fn pool_grew(before: &[Split; 2], after: &[Split; 2], t: &Tally) -> Result<(), String> {
    for d in [DOWN, UP] {
        let grew = |p: usize| after[d].parts[p].n - before[d].parts[p].n;
        let want = [t.n[d][0], t.n[d][0], t.n[d][0], t.n[d][1], t.n[d][1]];
        let got = [
            grew(CLOCK),
            grew(HOP),
            grew(WORK),
            grew(WAIT),
            grew(OTHER_WORK),
        ];
        if got.iter().zip(want).any(|(&g, w)| g < w) {
            return Err(format!(
                "dir {d}: the pool's parts grew by {got:?}, the log holds {want:?}"
            ));
        }
        for k in 0..KINDS {
            if after[d].kinds[k] - before[d].kinds[k] < t.kinds[d][k] {
                return Err(format!("dir {d}: kind {k} grew less than the log's"));
            }
        }
        if after[d].violations - before[d].violations < t.broken[d] {
            return Err(format!("dir {d}: violations grew less than the log's"));
        }
    }
    Ok(())
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
/// jitter), and no byte seen before its record's release: a byte reaches the client side of S, or tungstenite's side
/// of DelayStream, only after the release logged for its record. That release is stamped at the step that made it,
/// never after now, so this half holds by construction and cannot catch a release made too early (F-O): the replay
/// test proves the releases' stamps equal the model's deliveries. About 2 s.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn order_and_integrity_both_ways_and_no_byte_seen_before_its_release() {
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
/// rest owed; a return taken then records that owed amount (G-NC4). The exact numbers need every byte still in flight
/// when they are read, so the round trip is 20 s (F-I): no acknowledgement can arrive within the test's few
/// milliseconds unless the process freezes for 10 s.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn one_record_per_write_never_beyond_the_budget() {
    let (_client, server) = tokio::io::duplex(1 << 20);
    let mut ds = DelayStream::wrap(server, &cfg(fixed(20_000), 1), 0, now()).expect("wrapped");
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
    // G-NC4: a return taken while tungstenite's write is Pending records what it owes (`Return.owed`, r2:379 (3)'s
    // part above this socket): a 1 ms burst ending soon, passed by the next step.
    let start = add_burst_soon(&ds, MS);
    assert!(
        until(HANG, || !ds.returns().is_empty()).await,
        "hang guard: the burst's end"
    );
    let r = ds.returns()[0];
    assert_eq!(
        (r.at_ns, r.owed, r.outstanding),
        (start + MS, 10_044, BUDGET)
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
    // F-H: the bounds were reached, not merely respected. Down (G-FH: the bound at its own derivation): the writes
    // stopped with at most 22 B free, so the records written (at most 9: 8 of 8,214 B already exceed the budget) carry
    // at least 64 KB − 22 B − 9 x 22 B = 65,316 B of plaintext (65,360 B here), of which S has taken at most the
    // duplex's 1 KB when the largest is read: 64,292 B. A writer that went Pending with 8 KB or more still free fails
    // it. Up: the client sends far more than the budget, S is read until the up room is below the 549 B floor, so the
    // records in the model or released hold more than 64 KB − 549 B, less 22 B for each of at most 125 records of
    // 506 + 22 B.
    let down_least = BUDGET - OVERHEAD * 10 - 1_024;
    assert!(
        max[DOWN] >= down_least,
        "down reached only {} B of {down_least} B",
        max[DOWN]
    );
    let floor = 513 + 14 + OVERHEAD;
    let up_least = BUDGET - floor - OVERHEAD * (BUDGET / (506 + OVERHEAD) + 1);
    assert!(
        max[UP] >= up_least,
        "up reached only {} B of {up_least} B",
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

/// The request is one record, whatever its chunking (the client writes it a byte at a time). And F5: once it is
/// released, a zero-capacity read is answered at once (Ready, nothing taken), and the next read takes it whole.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_request_is_one_record() {
    let (mut client, server) = tokio::io::duplex(1 << 16);
    let mut ds =
        DelayStream::wrap(server, &cfg(quiet(Profile::p1()), 3), 0, now()).expect("wrapped");
    for b in REQUEST {
        client.write_all(&[*b]).await.expect("a byte");
        tokio::task::yield_now().await;
    }
    assert!(until(HANG, || !writes(&ds.logs(), UP).is_empty()).await);
    assert_eq!(
        writes(&ds.logs(), UP),
        vec![REQUEST.len() as u64 + OVERHEAD]
    );
    assert!(until(HANG, || !ds.logs().releases.is_empty()).await);
    let mut empty = ReadBuf::new(&mut []);
    assert!(
        matches!(
            Pin::new(&mut ds).poll_read(&mut noop(), &mut empty),
            Poll::Ready(Ok(()))
        ),
        "a zero-capacity read with a record released is Ready at once"
    );
    let mut buf = vec![0u8; 4_096];
    let n = ds.read(&mut buf).await.expect("the request");
    assert_eq!(&buf[..n], REQUEST, "nothing was taken by the empty read");
}

/// What one limit case saw: the close reason, the up model bytes that entered, whether the client side saw S closed,
/// the plaintext handed to tungstenite's side before the close's error, and the plaintext of the up records the model
/// released (from the log, after the pump ended).
#[derive(Debug, Clone, Copy)]
struct Limit {
    closed: Option<Reason>,
    entered: u64,
    eof: bool,
    handed: u64,
    released: u64,
}

/// One case of the limits: `head`, then `bad`, with the checks in `off` turned off (the bites); waits until the
/// connection closes or more than `before` model bytes entered up. After a close, reads tungstenite's side until the
/// close's error and checks F-F's identity: what is handed before it is exactly the up records the model released,
/// never a record that entered and was not released when the close ended the pump.
async fn limit_case(cfg: &L2Config, head: &[u8], bad: &[u8], off: &[Reason], before: u64) -> Limit {
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
    let (mut eof, mut handed, mut released) = (false, 0, 0);
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
                Ok(Ok(n)) => handed += n as u64,
                Ok(Err(e)) => {
                    err = Some(e.kind());
                    break;
                }
                Err(_) => break,
            }
        }
        assert_eq!(err, Some(io::ErrorKind::InvalidData));
        assert!(ds.probe().0, "the close ended the pump");
        let mut last = 0;
        for &(dir, model_end, _) in &ds.snapshot().logs.releases {
            if dir == UP {
                released += model_end - last - OVERHEAD;
                last = model_end;
            }
        }
        assert_eq!(
            handed, released,
            "F-F: tungstenite's side got exactly the released records before the close"
        );
    }
    Limit {
        closed,
        entered,
        eof,
        handed,
        released,
    }
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
/// S2: the request's end is where tungstenite's parser (httparse) puts it, so the limits apply to the same bytes it
/// reads as frames: a head ending in bare LFs, one with mixed line ends and one after leading blank lines each end
/// where httparse ends them, and the over-limit frame after them closes the connection, counted, with only the head
/// entered. The reproduced input among them: a head of exactly 4,096 B (tungstenite's read chunk) ending in "\n\n",
/// then a 604 B masked frame (before the fix it passed uncounted and entered the model).
///
/// F-F: after each close, tungstenite's side gets exactly the up records the model released, then the error; with a
/// 10 s one-way delay the request entered and is never released, so it is never handed (discarded by the close).
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
    // S2's heads (httparse 1.10.1 ends a head at its first empty line, an LF or CR LF line, after a non-empty one,
    // leading empty lines skipped: lib.rs:558-581, :1105-1116).
    let mut lf = b"GET / HTTP/1.1\nHost: localhost\nUpgrade: websocket\nConnection: Upgrade\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\nSec-WebSocket-Version: 13\nX-Pad: "
        .to_vec();
    lf.resize(4_096 - 2, b'a');
    lf.extend(b"\n\n");
    assert_eq!(
        lf.len(),
        4_096,
        "the reproduced head: exactly one tungstenite read"
    );
    let mixed = b"GET / HTTP/1.1\r\nHost: localhost\nUpgrade: websocket\r\nConnection: Upgrade\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\n\r\n"
        .to_vec();
    // Leading empty lines, "\r\n\r\n" among them (the old scanner ended the head there).
    let leading = [&b"\r\n\r\n\n"[..], REQUEST].concat();
    let cases: Vec<Case> = vec![
        (
            lf.clone(),
            frame(true, 2, true, &[1; 604], false),
            Frame,
            lf.len() as u64 + OVERHEAD,
            Vec::new(),
        ),
        (
            mixed.clone(),
            frame(true, 2, true, &[1; 604], false),
            Frame,
            mixed.len() as u64 + OVERHEAD,
            Vec::new(),
        ),
        (
            leading.clone(),
            frame(true, 2, true, &[1; 604], false),
            Frame,
            leading.len() as u64 + OVERHEAD,
            Vec::new(),
        ),
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
        let l = limit_case(&c, head, bad, &[], *before).await;
        assert_eq!(l.closed, Some(*why), "{why:?}");
        assert_eq!(
            l.entered, *before,
            "{why:?}: nothing of the bad frame entered"
        );
        assert!(l.eof, "{why:?}: S was dropped");
        println!(
            "{why:?} after a {} B head: closed before buffering ({before} B up); {} B handed, {} B released",
            head.len(),
            l.handed,
            l.released
        );
        for (off, then) in bites {
            let Limit {
                closed, entered, ..
            } = limit_case(&c, head, bad, off, *before).await;
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
    // F-F, pinned: with a 10 s one-way delay the request entered and was not released when the frame closed the
    // connection, so tungstenite's side is handed nothing before the error.
    let slow = cfg(fixed(20_000), 4);
    let l = limit_case(
        &slow,
        REQUEST,
        &frame(true, 2, true, &[1; 514], false),
        &[],
        request,
    )
    .await;
    assert_eq!(l.closed, Some(Frame));
    assert_eq!((l.entered, l.released, l.handed), (request, 0, 0));
    // One source: the limit raised to 16,000 B in L2Config lets the 514 B frame's payload into the model.
    let mut open = c.clone();
    open.ws_max_message_bytes = 16_000;
    let l = limit_case(
        &open,
        REQUEST,
        &frame(true, 2, true, &[1; 514], false),
        &[],
        request,
    )
    .await;
    assert_eq!(l.closed, None);
    assert!(l.entered > request, "the payload entered: {} B", l.entered);
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

/// The model's bursts and DelayStream's burst ends (model axis).
fn burst_state<S>(ds: &DelayStream<S>) -> (Vec<(u64, u64)>, Vec<u64>) {
    let c = ds.sh.lock();
    (c.model.bursts().to_vec(), c.burst_ends.clone())
}

/// F1 + S7: a burst whose end is past DelayStream's clock's range (start + length above u64::MAX; a "forever" length
/// of u64::MAX is one) is refused, leaving the model's bursts and the burst ends as they were; the longest burst from a
/// start (u64::MAX − start) is accepted, and the pump's next due time is then its end, never a time wrapped into the
/// past (the old code panicked here in a debug build and spun in a release one); the idle pump does not run. Also
/// NC-U3c-8: a start at the model's now is refused, the next nanosecond accepted (the model 10 s ahead of the clock, so
/// no stall decides it: G-NC8).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_burst_past_the_clocks_range_is_refused_and_never_wraps() {
    let (_client, server) = tokio::io::duplex(1 << 12);
    let ds = DelayStream::wrap(server, &cfg(quiet(Profile::p2()), 15), 0, now()).expect("wrapped");
    let before = burst_state(&ds);
    let start = now() + SEC;
    for len in [u64::MAX, u64::MAX - start + 1] {
        let e = ds.add_burst(start, len).expect_err("refused");
        assert_eq!(e.kind(), io::ErrorKind::InvalidInput, "len {len}");
        assert_eq!(burst_state(&ds), before, "nothing added for len {len}");
    }
    ds.add_burst(start, u64::MAX - start)
        .expect("the longest burst from its start");
    let origin = ds.snapshot().origin;
    let (bursts, ends) = burst_state(&ds);
    assert_eq!(bursts, vec![(start - origin, u64::MAX - start)]);
    assert_eq!(ends, vec![u64::MAX - origin]);
    let polls = ds.life().polls;
    {
        let c = ds.sh.lock();
        assert_eq!(c.model.next_event(), None, "an idle connection");
        assert_eq!(
            c.due(),
            Some(u64::MAX),
            "due at the burst's end, on the clock"
        );
    }
    tokio::time::sleep(Duration::from_millis(50)).await;
    let runs = ds.life().polls - polls;
    assert!(runs <= 1, "the pump ran {runs} times on an idle connection");
    // NC-U3c-8 on the model's own now, stepped 10 s ahead of the clock (G-NC8): add_burst steps the model to the
    // clock's now, which stays behind it unless the test thread stalls 10 s, so no stall can flip the second answer.
    let (_c2, s2) = tokio::io::duplex(1 << 12);
    let d2 = DelayStream::wrap(s2, &cfg(fixed(2_000), 16), 0, now()).expect("wrapped");
    let at_now = {
        let mut c = d2.sh.lock();
        let t = c.model_t(delay_clock::clock().now_ns() + 10 * SEC);
        c.step_to(t, By::App);
        c.origin + c.model.now_ns()
    };
    assert!(d2.add_burst(at_now, SEC).is_err(), "at the model's now");
    assert!(d2.add_burst(at_now + 1, SEC).is_ok(), "just after it");
}

/// F-A: wrap moves the profile's bursts from DelayStream's clock to the model's axis (less the origin), drops those
/// already ended, clips one already running at the origin (it starts at the model's 0 and keeps its end), records each
/// end for the per-return records, and refuses more than 64 still to come. A write at once falls in the clipped burst
/// and is lost; nothing outside the bursts is (quiet profile).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn wrap_moves_the_profiles_bursts_to_the_models_axis() {
    while now() < 20 * MS {
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    let o = now();
    let mut c = cfg(quiet(Profile::p2()), 17);
    c.profile.bursts_ns = vec![
        (o - 10 * MS, 5 * MS),
        (o - 2 * MS, SEC),
        (o + 20 * SEC, 500 * MS),
    ];
    let (client, server) = tokio::io::duplex(1 << 16);
    let mut ds = DelayStream::wrap(server, &c, 0, now()).expect("wrapped");
    let origin = ds.snapshot().origin;
    let running_end = (o - 2 * MS + SEC)
        .checked_sub(origin)
        .expect("the wrap ran within the running burst (no 1 s stall)");
    let (bursts, ends) = burst_state(&ds);
    assert_eq!(
        bursts,
        vec![(0, running_end), (o + 20 * SEC - origin, 500 * MS)],
        "ended: dropped; running: clipped at the origin; future: shifted"
    );
    assert_eq!(ends, vec![running_end, o + 20 * SEC + 500 * MS - origin]);
    // A write now is sent inside the clipped burst: lost; the rest of the traffic outside it gets through.
    ds.write_all(&[0x82; 100]).await.expect("written");
    let (mut cr, _cw) = tokio::io::split(client);
    let mut buf = [0u8; 100];
    tokio::time::timeout(HANG, cr.read_exact(&mut buf))
        .await
        .expect("hang guard: delivered after the burst")
        .expect("read");
    let s = ds.snapshot();
    let first = s.sent_log.first().copied().expect("a transmission");
    assert!(
        first.3 < running_end && first.4,
        "the first, inside the burst, was lost"
    );
    for &(_, _, _, t, lost) in &s.sent_log {
        assert_eq!(
            lost,
            t < running_end,
            "a transmission at {t}: lost only inside the burst"
        );
    }
    // More than 64 still to come: refused; ended ones do not count.
    let mut many = cfg(quiet(Profile::p2()), 18);
    many.profile.bursts_ns = (0..65).map(|i| (o + 30 * SEC + i * SEC, MS)).collect();
    let (_c, s2) = tokio::io::duplex(64);
    let e = DelayStream::wrap(s2, &many, 0, now()).expect_err("65 to come");
    assert_eq!(e.kind(), io::ErrorKind::InvalidInput);
    many.profile.bursts_ns[0] = (1, 1);
    let (_c, s3) = tokio::io::duplex(64);
    let d = DelayStream::wrap(s3, &many, 0, now()).expect("64 to come and one ended");
    assert_eq!(burst_state(&d).0.len(), 64);
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
    let mut asked = Vec::new();
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
                asked.push(add_burst_soon(ds, 600 * MS));
                added += 1;
            }
        }
        false
    })
    .await;
    let s = ds.snapshot();
    let bursts = bursts_of(&s.logs);
    assert_eq!(bursts.len() as u32, added);
    // F-B (C5): each burst asked for on DelayStream's clock sits on the model's axis at the start less the origin.
    assert!(s.origin > 0);
    assert_eq!(
        bursts.iter().map(|b| b.0).collect::<Vec<_>>(),
        asked.iter().map(|a| a - s.origin).collect::<Vec<_>>(),
        "logged start = asked start - origin {}",
        s.origin
    );
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

/// What a probe S saw.
#[derive(Debug, Default)]
struct ProbeSeen {
    /// S's calls that found the connection's lock held: the pump called S under it (F3).
    held: AtomicU64,
    /// poll_read calls, and those made while the up room was below the read floor (C2's floor; F-R2).
    reads: AtomicU64,
    under_floor: AtomicU64,
    writes: AtomicU64,
}

/// An S for a pump polled by hand: at each of its calls it tries the connection's lock and counts it held (F3), and at
/// each read it checks the up room against the read floor (F-R2). Its writes answer Pending `pending` times, then take
/// at most `chunk` bytes per call into `out`; its reads come from the duplex.
struct ProbeS {
    io: DuplexStream,
    sh: Arc<std::sync::OnceLock<Arc<Shared>>>,
    seen: Arc<ProbeSeen>,
    pending: u32,
    chunk: usize,
    out: Arc<Mutex<Vec<u8>>>,
}

impl ProbeS {
    fn check(&self, f: impl FnOnce(&Conn)) {
        if let Some(sh) = self.sh.get() {
            match sh.conn.try_lock() {
                Ok(c) => f(&c),
                Err(_) => {
                    self.seen.held.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }
}

impl AsyncRead for ProbeS {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        self.seen.reads.fetch_add(1, Ordering::Relaxed);
        let seen = Arc::clone(&self.seen);
        self.check(|c| {
            if c.up_room() < c.read_floor {
                seen.under_floor.fetch_add(1, Ordering::Relaxed);
            }
        });
        Pin::new(&mut self.io).poll_read(cx, buf)
    }
}

impl AsyncWrite for ProbeS {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.seen.writes.fetch_add(1, Ordering::Relaxed);
        self.check(|_| {});
        if self.pending > 0 {
            self.pending -= 1;
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        let n = buf.len().min(self.chunk);
        self.out.lock().expect("out").extend_from_slice(&buf[..n]);
        Poll::Ready(Ok(n))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.check(|_| {});
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.check(|_| {});
        Poll::Ready(Ok(()))
    }
}

/// A waker that counts its wakes from the test's own thread (the release clock's thread wakes it too, unjudged) and
/// those that found the connection's lock still held by the waker (F2).
struct ProbeWake {
    sh: Arc<Shared>,
    thread: std::thread::ThreadId,
    wakes: AtomicU64,
    held: AtomicU64,
}

impl ProbeWake {
    fn new(sh: &Arc<Shared>) -> Arc<ProbeWake> {
        Arc::new(ProbeWake {
            sh: Arc::clone(sh),
            thread: std::thread::current().id(),
            wakes: AtomicU64::new(0),
            held: AtomicU64::new(0),
        })
    }

    fn seen(&self) -> (u64, u64) {
        (
            self.wakes.load(Ordering::Relaxed),
            self.held.load(Ordering::Relaxed),
        )
    }
}

impl std::task::Wake for ProbeWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if std::thread::current().id() == self.thread {
            self.wakes.fetch_add(1, Ordering::Relaxed);
            if self.sh.conn.try_lock().is_err() {
                self.held.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

/// F3, F4, F2 and F-R2 on a pump polled by hand (nothing else runs: one thread, no spawned pump, so every reading is
/// exact):
/// - F3: none of S's calls (write, flush, read, shutdown) finds the connection's lock held by the pump;
/// - F4: a record S answered Pending is offered again from its staged bytes, not copied again (a byte changed in the
///   connection's buffer after the staging never reaches S), and S taking it 500 B at a time takes it whole (one split
///   record counted);
/// - F2: the wakes owed by `current` (the pump's for a released down record, the reader's for a released up one) and
///   by `poll_shutdown` (the pump's) fire after the lock is released;
/// - F-R2: S is never read while the up room is below the 549 B read floor, and once the room is below it the pump
///   stops reading although the client has more to send.
#[tokio::test(flavor = "current_thread")]
async fn the_pump_never_calls_s_under_the_lock_and_wakes_come_after_it() {
    let (mut client, server) = tokio::io::duplex(1 << 20);
    let cell = Arc::new(std::sync::OnceLock::new());
    let seen = Arc::new(ProbeSeen::default());
    let out = Arc::new(Mutex::new(Vec::new()));
    let probe = ProbeS {
        io: server,
        sh: Arc::clone(&cell),
        seen: Arc::clone(&seen),
        pending: 1,
        chunk: 500,
        out: Arc::clone(&out),
    };
    let (mut ds, mut pump) =
        DelayStream::build(probe, &cfg(fixed(200), 20), 0, now()).expect("built");
    assert!(cell.set(Arc::clone(&ds.sh)).is_ok(), "set once");
    let pw = ProbeWake::new(&ds.sh);
    let pump_waker = Waker::from(Arc::clone(&pw));
    let run = |p: &mut Pump<ProbeS>| {
        assert!(
            Pin::new(p)
                .poll(&mut Context::from_waker(&pump_waker))
                .is_pending()
        );
    };
    run(&mut pump);
    // A 2,000 B record down, released after 100 ms by a `current` step: the pump's wake comes after the lock (the
    // 100 ms one-way delay keeps each release after the steps that must come before it).
    let record: Vec<u8> = (0..2_000u32).map(|i| (i % 251) as u8).collect();
    assert!(matches!(
        Pin::new(&mut ds).poll_write(&mut noop(), &record),
        Poll::Ready(Ok(2_000))
    ));
    let end = Instant::now() + HANG;
    while ds.sh.lock().released[DOWN] == 0 {
        assert!(
            Instant::now() < end,
            "hang guard: the down record's release"
        );
        tokio::time::sleep(Duration::from_millis(2)).await;
        let _ = ds.outstanding_bytes();
    }
    assert_eq!(
        pw.seen(),
        (1, 0),
        "current woke the pump once, after the lock"
    );
    // S answers Pending: the record stays staged. A byte changed in the connection's buffer now must not reach S.
    run(&mut pump);
    assert_eq!(seen.writes.load(Ordering::Relaxed), 1);
    ds.sh.lock().bytes[DOWN][0] ^= 0xff;
    run(&mut pump);
    assert_eq!(*out.lock().expect("out"), record, "the staged bytes, whole");
    assert_eq!(
        seen.writes.load(Ordering::Relaxed),
        1 + 4,
        "Pending, then 4 calls of 500 B"
    );
    assert_eq!(
        seen.held.load(Ordering::Relaxed),
        0,
        "S's writes never ran under the lock"
    );
    {
        let c = ds.sh.lock();
        assert!(c.recs[DOWN].is_empty() && c.bytes[DOWN].is_empty());
        assert_eq!(
            c.own.split_writes, 1,
            "one record S took in more than one call"
        );
    }
    // The request up, then the reader's wake from a `current` step.
    client.write_all(REQUEST).await.expect("the request");
    run(&mut pump);
    assert_eq!(
        writes(&ds.logs(), UP),
        vec![REQUEST.len() as u64 + OVERHEAD]
    );
    let rw = ProbeWake::new(&ds.sh);
    let reader = Waker::from(Arc::clone(&rw));
    let mut buf = vec![0u8; 4_096];
    let mut rb = ReadBuf::new(&mut buf);
    assert!(
        Pin::new(&mut ds)
            .poll_read(&mut Context::from_waker(&reader), &mut rb)
            .is_pending()
    );
    let end = Instant::now() + HANG;
    while rw.seen().0 == 0 {
        assert!(Instant::now() < end, "hang guard: the request's release");
        tokio::time::sleep(Duration::from_millis(2)).await;
        let _ = ds.srtt_ns();
    }
    assert_eq!(
        rw.seen(),
        (1, 0),
        "current woke the reader once, after the lock"
    );
    // F-R2: 150 KB of frames at once; nobody reads tungstenite's side, so the up room fills.
    let mut flood = Vec::new();
    for _ in 0..300 {
        flood.extend(frame(true, 2, true, &[7; 500], false));
    }
    client.write_all(&flood).await.expect("the flood");
    run(&mut pump);
    let reads = seen.reads.load(Ordering::Relaxed);
    let (room, floor, held_up) = {
        let c = ds.sh.lock();
        (c.up_room(), c.read_floor, c.bytes[UP].len() + c.parse_len)
    };
    assert!(room < floor, "the room fell below the floor: {room} B");
    for _ in 0..3 {
        tokio::time::sleep(Duration::from_millis(15)).await;
        run(&mut pump);
    }
    assert_eq!(
        seen.reads.load(Ordering::Relaxed),
        reads,
        "no read below the floor, with {} B still unread",
        flood.len() + REQUEST.len() - held_up
    );
    assert_eq!(
        seen.under_floor.load(Ordering::Relaxed),
        0,
        "never read below the floor"
    );
    // poll_shutdown's wake of the pump, after the lock; then S's own shutdown, outside it.
    let before = pw.seen().0;
    assert!(Pin::new(&mut ds).poll_shutdown(&mut noop()).is_pending());
    assert_eq!(
        pw.seen(),
        (before + 1, 0),
        "poll_shutdown woke the pump after the lock"
    );
    run(&mut pump);
    assert!(ds.sh.lock().down_done, "S shut down");
    assert_eq!(
        seen.held.load(Ordering::Relaxed),
        0,
        "S never ran under the lock"
    );
    println!(
        "pump by hand: {} S writes, {} S reads, room {room} B < floor {floor} B with {held_up} B held up",
        seen.writes.load(Ordering::Relaxed),
        seen.reads.load(Ordering::Relaxed)
    );
}

/// An S whose every write takes `slow` (the thread sleeps, as rustls and a slow syscall would hold it) and takes the
/// whole offer, logging (stamp, bytes); its reads come from the duplex.
struct SlowS {
    io: DuplexStream,
    slow: Duration,
    out: Arc<Mutex<Vec<(u64, usize)>>>,
}

impl AsyncRead for SlowS {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.io).poll_read(cx, buf)
    }
}

impl AsyncWrite for SlowS {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        std::thread::sleep(self.slow);
        self.out.lock().expect("out").push((now(), buf.len()));
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

/// A waker that stamps each of its wakes on DelayStream's clock, from any thread.
struct StampWake(Mutex<Vec<u64>>);

impl std::task::Wake for StampWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.lock().expect("stamps").push(now());
    }
}

/// R1 (the fix review's regression; C3: released down records go into S at once), the critic's probe 1 on a pump
/// polled by hand (one thread, nothing else steps the model): records A and B written 20 ms apart, fixed(400) (200 ms
/// one way, no jitter, no loss), the pump polled 1 ms after A's delivery while S takes 30 ms per write. The pass's
/// opening step releases A and writes it; B's delivery falls inside that write, so the pass's up loop releases it
/// (By::Mid), after the down loop ended. Judged on model state and wake stamps: after the pass, a released down record
/// left unwritten must have had the pump's waker woken between its release step and the pass's end (the clock owes no
/// wake for it: its entry fired at A's delivery, and the next model event, an acknowledgement, is over 100 ms after the
/// pass), and the pass that wake asks for writes it. Vacuity: B was released by the up loop's step; a thread stalled
/// past B's delivery before the poll releases both at the opening step, so such an attempt is retried (5 at most).
#[tokio::test(flavor = "current_thread")]
async fn a_down_record_released_in_a_pump_pass_is_written_or_owed_a_wake() {
    for attempt in 1..=5 {
        let (_client, server) = tokio::io::duplex(1 << 20);
        let out = Arc::new(Mutex::new(Vec::new()));
        let s = SlowS {
            io: server,
            slow: Duration::from_millis(30),
            out: Arc::clone(&out),
        };
        let c = cfg(fixed(400), 20);
        let one_way = one_way_ns(&c);
        let (mut ds, mut pump) = DelayStream::build(s, &c, 0, now()).expect("built");
        let sw = Arc::new(StampWake(Mutex::new(Vec::new())));
        let waker = Waker::from(Arc::clone(&sw));
        let run = |p: &mut Pump<SlowS>| {
            assert!(
                Pin::new(p)
                    .poll(&mut Context::from_waker(&waker))
                    .is_pending()
            );
        };
        run(&mut pump);
        let rec = vec![0x82u8; 1_000];
        assert!(matches!(
            Pin::new(&mut ds).poll_write(&mut noop(), &rec),
            Poll::Ready(Ok(1_000))
        ));
        std::thread::sleep(Duration::from_millis(20));
        assert!(matches!(
            Pin::new(&mut ds).poll_write(&mut noop(), &rec),
            Poll::Ready(Ok(1_000))
        ));
        let a_due = {
            let c = ds.sh.lock();
            c.origin + c.logs.steps[0].0 + one_way
        };
        while now() < a_due + MS {
            std::thread::sleep(Duration::from_micros(200));
        }
        run(&mut pump);
        let after = now();
        let (left, front) = {
            let c = ds.sh.lock();
            (c.released[DOWN], c.recs[DOWN].front().map(|r| r.freed))
        };
        let wakes = sw.0.lock().expect("stamps").clone();
        if left == 0 {
            println!(
                "attempt {attempt}: both records released at the pass's opening step (a stalled thread); again"
            );
            continue;
        }
        let f = front.expect("the unwritten record");
        assert_eq!(
            (left, f.kind),
            (1, MID),
            "B, released by the up loop's step"
        );
        let owed: Vec<u64> = wakes
            .iter()
            .copied()
            .filter(|&w| w >= f.step && w <= after)
            .collect();
        println!(
            "attempt {attempt}: B released by the up loop at {} ns, left unwritten; the pass ended at {after} ns; \
             the pump's wakes between {owed:?}",
            f.step
        );
        assert!(
            !owed.is_empty(),
            "a released down record left unwritten with no wake owed to the pump (R1)"
        );
        // The pass the wake asks for writes B.
        run(&mut pump);
        assert_eq!(ds.sh.lock().released[DOWN], 0, "B written");
        assert_eq!(out.lock().expect("out").len(), 2, "A and B, one write each");
        return;
    }
    panic!("5 attempts, each stalled past B's delivery before the poll");
}

/// G-PRE-WAKE: an acknowledgement at the client's end that reopens the up room, stepped by an application task
/// (`current`, `add_burst`) rather than by the pump, wakes the pump; the old code woke nobody, and its clock entry
/// moved past the event, so the pump stopped reading S until its next wake. On a pump polled by hand (one thread,
/// fixed(400): 200 ms one way): one pass reads the request and a 150 KB flood until the up room is below what it waits
/// for (the read floor, or the front frame the scanner holds); after the first flight's delivery (one way after the
/// writes) and before any acknowledgement (a round trip) tungstenite's side reads every released record (each read
/// frees its record and wakes the pump, unjudged), so only acknowledgements can reopen the room; then the test's own
/// `current` calls step them. Judged on model state and wakes from the test's
/// thread only (the clock thread's are not counted): the call whose step lifts the room across what the pump waits for
/// woke the pump, and the pass that wake asks for takes more of the client's bytes into the model. Vacuity: the crossing
/// is a `current` step, not a read's (a thread stalled past the first acknowledgement during the reads); such an attempt
/// is retried (3 at most).
#[tokio::test(flavor = "current_thread")]
async fn an_acknowledgement_an_application_task_steps_that_reopens_the_up_room_wakes_the_pump() {
    for attempt in 1..=3 {
        let (mut client, server) = tokio::io::duplex(1 << 20);
        let c = cfg(fixed(400), 22);
        let one_way = one_way_ns(&c);
        let (mut ds, mut pump) = DelayStream::build(server, &c, 0, now()).expect("built");
        let pw = ProbeWake::new(&ds.sh);
        let pump_waker = Waker::from(Arc::clone(&pw));
        let run = |p: &mut Pump<DuplexStream>| {
            assert!(
                Pin::new(p)
                    .poll(&mut Context::from_waker(&pump_waker))
                    .is_pending()
            );
        };
        let mut flood = REQUEST.to_vec();
        for _ in 0..300 {
            flood.extend(frame(true, 2, true, &[7; 500], false));
        }
        client.write_all(&flood).await.expect("the flood");
        run(&mut pump);
        let (room, need, last, written) = {
            let c = ds.sh.lock();
            let last = c.logs.steps.last().expect("records in").0;
            (c.up_room(), c.up_need(), last, c.written[UP])
        };
        assert!(
            room < need,
            "the pass stopped on the up room: {room} B < {need} B"
        );
        // The first flight (the initial window) is delivered one way after the writes; nothing is acknowledged before a
        // round trip.
        let delivered = ds.sh.lock().origin + last + one_way + MS;
        while now() < delivered {
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
        let mut buf = vec![0u8; 4_096];
        let mut rb = ReadBuf::new(&mut buf);
        loop {
            match Pin::new(&mut ds).poll_read(&mut noop(), &mut rb) {
                Poll::Ready(Ok(())) => {
                    assert!(!rb.filled().is_empty(), "no EOF here");
                    rb.clear();
                }
                Poll::Ready(Err(e)) => panic!("{e}"),
                Poll::Pending => break,
            }
        }
        let (room, need) = {
            let c = ds.sh.lock();
            (c.up_room(), c.up_need())
        };
        if room >= need {
            println!(
                "attempt {attempt}: the room reopened during the reads (a stalled thread); again"
            );
            continue;
        }
        let end = Instant::now() + HANG;
        let woke = loop {
            assert!(
                Instant::now() < end,
                "hang guard: no acknowledgement reopened the up room"
            );
            tokio::time::sleep(Duration::from_millis(2)).await;
            let (before, need) = {
                let c = ds.sh.lock();
                (c.up_room(), c.up_need())
            };
            let w0 = pw.seen().0;
            let _ = ds.outstanding_bytes();
            let after = ds.sh.lock().up_room();
            if before < need && after >= need {
                println!(
                    "attempt {attempt}: a current() step lifted the up room {before} -> {after} B across {need} B"
                );
                break pw.seen().0 - w0;
            }
        };
        assert_eq!(
            woke, 1,
            "the step that reopened the up room woke the pump once"
        );
        run(&mut pump);
        let more = ds.sh.lock().written[UP];
        assert!(
            more > written,
            "the woken pass took more of the client's bytes: {written} -> {more} B"
        );
        return;
    }
    panic!("3 attempts, each reopened during the reads");
}

/// F-G (C3): an EOF from S ends the up direction after its last delivered byte. The client writes the request and 40
/// frames at once, then shuts its write half; tungstenite's side reads every byte the client wrote, in order, and only
/// then Ok(0), with nothing left in the model or the scanner then. P2 with its loss, so repairs stand between the EOF
/// read from S and the last byte's delivery.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_eof_from_s_ends_the_up_direction_after_its_last_byte() {
    let (client, server) = tokio::io::duplex(1 << 20);
    let mut ds = DelayStream::wrap(server, &cfg(Profile::p2(), 19), 0, now()).expect("wrapped");
    let (_cr, mut cw) = tokio::io::split(client);
    let mut sent = REQUEST.to_vec();
    for i in 0..40u8 {
        sent.extend(frame(true, 2, true, &[i; 300], false));
    }
    cw.write_all(&sent).await.expect("written");
    cw.shutdown().await.expect("the client's write half shut");
    let mut got = Vec::new();
    let mut buf = vec![0u8; 4_096];
    loop {
        let n = tokio::time::timeout(HANG, ds.read(&mut buf))
            .await
            .expect("hang guard")
            .expect("read");
        if n == 0 {
            break;
        }
        got.extend_from_slice(&buf[..n]);
    }
    assert!(
        got == sent,
        "every byte, in order, before the EOF ({} of {} B)",
        got.len(),
        sent.len()
    );
    let c = ds.sh.lock();
    assert!(c.up_eof, "the EOF came from S");
    assert!(c.recs[UP].is_empty() && c.bounds.is_empty() && c.parse_len == 0);
    assert_eq!(
        c.written[UP],
        sent.len() as u64,
        "every record entered the model"
    );
    println!(
        "EOF after {} B in {} records; up stalls {}",
        got.len(),
        41,
        c.model.counters(UP).stalls
    );
}

/// G-F10 (F10's compaction; the critic's probe 4): tiny frames fill the up room before anyone reads, then tungstenite's
/// side reads to the client's EOF, and every byte arrives in order. The client writes the request and 6,000 frames of
/// 10 B (16 B on the wire, 38 B in the model each) at once, then shuts its write half; nobody reads until the up room
/// is below the read floor (model state). Draining then frees one record per read, and each pump pass moves the next
/// waiting record into the model and compacts the scanner's buffer with the records behind it still pending, so
/// move_up's shift of their bounds decides every later byte (with the shift dropped, the pump panicked on a slice index
/// and the reader saw the end early). Vacuity: such a compaction happened (a test count). fixed(20); a few seconds in
/// a debug build.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tiny_frames_that_fill_the_up_room_drain_whole_and_in_order() {
    let (client, server) = tokio::io::duplex(1 << 20);
    let mut ds = DelayStream::wrap(server, &cfg(fixed(20), 31), 0, now()).expect("wrapped");
    let (_cr, mut cw) = tokio::io::split(client);
    let mut sent = REQUEST.to_vec();
    for i in 0..6_000u32 {
        sent.extend(frame(true, 2, true, &[(i % 251) as u8; 10], false));
    }
    cw.write_all(&sent).await.expect("written");
    cw.shutdown().await.expect("the client's write half shut");
    assert!(
        until(HANG, || {
            let c = ds.sh.lock();
            c.up_room() < c.read_floor
        })
        .await,
        "hang guard: the up room filled"
    );
    let mut got = Vec::new();
    let mut buf = vec![0u8; 4_096];
    loop {
        let n = tokio::time::timeout(HANG, ds.read(&mut buf))
            .await
            .expect("hang guard")
            .expect("read");
        if n == 0 {
            break;
        }
        got.extend_from_slice(&buf[..n]);
    }
    let shifted = ds.life().shifted;
    println!(
        "{} of {} B, in order; compactions with records still pending: {shifted}",
        got.len(),
        sent.len()
    );
    assert!(got == sent, "every byte, in order");
    assert!(shifted > 0, "move_up shifted pending bounds (not vacuous)");
}

/// G-NC2 (NC-U3c-2's corrected bound, under loss): the down plaintext DelayStream holds never exceeds the 64 KB budget
/// plus the acknowledged prefix of the front record not yet released, checked on every held-bytes sample
/// (`Conn::check_held_bound`), and that prefix does carry it beyond 64 KB. P2 with its loss and jitter; the writer keeps
/// the budget full with 16 KB records while the client drains S. It runs until 100 records are written, a first
/// transmission was lost, and a sample held more than 64 KB (model state; a hang guard only). Then: no sample above the
/// bound either way, the largest excess over 64 KB is below one record (16,406 B), and first transmissions were lost.
/// A few seconds.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn under_loss_the_held_bytes_stay_within_64_kb_and_the_front_records_acknowledged_prefix() {
    let (client, server) = tokio::io::duplex(1 << 20);
    let mut ds = DelayStream::wrap(server, &cfg(Profile::p2(), 25), 0, now()).expect("wrapped");
    let (mut cr, _cw) = tokio::io::split(client);
    let drain = tokio::spawn(async move {
        let mut buf = vec![0u8; 1 << 16];
        while cr.read(&mut buf).await.is_ok_and(|n| n > 0) {}
    });
    let end = Instant::now() + HANG;
    let mut records = 0u64;
    loop {
        let beyond = ds.life().held_beyond_budget[DOWN];
        let lost = ds.snapshot().counters[DOWN].first_lost;
        if records >= 100 && beyond > 0 && lost > 0 {
            break;
        }
        assert!(
            Instant::now() < end,
            "hang guard: {records} records, largest excess {beyond} B, {lost} first losses"
        );
        ds.write_all(&[0x82; RECORD_MAX]).await.expect("written");
        records += 1;
    }
    let life = ds.life();
    let s = ds.snapshot();
    println!(
        "{records} writes of 16 KB at P2: largest down excess over 64 KB {} B; samples above the bound {:?}; first \
         losses {}, stalls {}",
        life.held_beyond_budget[DOWN],
        life.held_over_bound,
        s.counters[DOWN].first_lost,
        s.counters[DOWN].stalls
    );
    assert_eq!(
        life.held_over_bound,
        [0, 0],
        "held <= 64 KB + the front record's acknowledged prefix"
    );
    assert!(life.held_beyond_budget[DOWN] < RECORD_MAX as u64 + OVERHEAD);
    drop(ds);
    drain.await.expect("drained");
}

/// An S that serves reads from the duplex and panics at its first write.
struct PanicS {
    io: DuplexStream,
}

impl AsyncRead for PanicS {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.io).poll_read(cx, buf)
    }
}

impl AsyncWrite for PanicS {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        panic!("PanicS: the test's S panics at its first write (G-PRE-PANIC)");
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

/// G-PRE-PANIC: a pump that panics gives tungstenite's side an error, never a clean EOF (which tungstenite would take
/// for a normal close). A spawned pump over an S that panics at its first write: the client's request is read whole
/// through DelayStream; then a down record is written, and once the model releases it the pump's write into S panics.
/// The next reads give an error of kind Other (the pump ended while panicking), never Ok(0), and a write is refused
/// with the same kind. fixed(20), well under 1 s; the waits are hang guards.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_pump_that_panics_gives_the_reader_an_error_never_a_clean_eof() {
    let (mut client, server) = tokio::io::duplex(1 << 16);
    let mut ds =
        DelayStream::wrap(PanicS { io: server }, &cfg(fixed(20), 24), 0, now()).expect("wrapped");
    client.write_all(REQUEST).await.expect("the request");
    let mut got = vec![0u8; REQUEST.len()];
    tokio::time::timeout(HANG, ds.read_exact(&mut got))
        .await
        .expect("hang guard: the request")
        .expect("read");
    assert_eq!(got, REQUEST);
    ds.write_all(&[0x82; 100]).await.expect("written");
    let mut buf = [0u8; 64];
    let r = tokio::time::timeout(HANG, ds.read(&mut buf))
        .await
        .expect("hang guard: the pump's end");
    let (done, ..) = ds.probe();
    assert!(done, "the pump ended");
    match r {
        Ok(0) => panic!("a panicked pump read as a clean EOF"),
        Ok(n) => panic!("{n} B after the request"),
        Err(e) => assert_eq!(e.kind(), io::ErrorKind::Other, "{e}"),
    }
    let e = ds.write_all(&[0x82; 10]).await.expect_err("refused");
    assert_eq!(e.kind(), io::ErrorKind::Other);
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
    // The split's rows are the run's (the `ws_split` event), not a connection's.
    let mut split = Vec::new();
    names::split_rows([Split::default(); 2], &mut split);
    assert_eq!(names_of(&DelayStream::split_rows()), names_of(&split));
    assert_eq!(value(&summary, names::CONNS_BEFORE), 7);
    assert_eq!(value(&summary, names::ACCEPTED), 42);
    // header_numbers blocks on the clock thread's answer (F6): off the runtime's workers, as U6's writer will call it.
    let ds = Arc::new(ds);
    let d = Arc::clone(&ds);
    let h = tokio::task::spawn_blocking(move || d.header_numbers())
        .await
        .expect("the header");
    assert_eq!(names_of(&h), names::HEADER_NUMBERS.to_vec());
    assert_eq!(value(&h, "ws_wrapper"), 1);
    assert_eq!(value(&h, "ws_one_way_ns"), 50 * MS);
    assert_eq!(value(&h, "ws_max_message_bytes"), 513);
    assert_eq!(value(&h, "ws_up_read_floor_bytes"), 549);
    assert_eq!(
        value(&h, "ws_shutdown_wait_ns"),
        10 * SEC,
        "SHUTDOWN_WAIT_NS"
    );
    // F7 and the split's memory (b): the heap a connection allocates at wrap at the defaults, as SERIES.md states it:
    // two lateness histograms (one per direction) with their sparse marks (2 x (135,040 + 1,056 + 256 x 24) B), the
    // scanner's buffer (8,192 B), the down staging buffer (16,384 B) and the read buffer (527 B); and once per process
    // the split's pool, ten histograms of 135,040 B (per direction its five parts).
    assert_eq!(
        value(&h, "ws_conn_fixed_bytes"),
        2 * (135_040 + 1_056 + 256 * 24) + 8_192 + 16_384 + 527
    );
    assert_eq!(value(&h, "ws_conn_fixed_bytes"), 309_583);
    assert_eq!(
        value(&h, "ws_split_fixed_bytes"),
        2 * PARTS as u64 * 135_040
    );
    assert_eq!(value(&h, "ws_split_fixed_bytes"), 1_350_400);
    // F-M: the release clock's 1 ns slack, written by the thread and read back, at its start and now (a real-time
    // policy reads 0, as ImpairSocket's test allows; socket_tests.rs).
    let texts = ds.header_texts();
    let sched = &texts
        .iter()
        .find(|t| t.0 == "ws_clock_sched_policy")
        .expect("the clock's policy")
        .1;
    let slack = if sched.starts_with("SCHED_FIFO") || sched.starts_with("SCHED_RR") {
        0
    } else {
        1
    };
    assert_eq!(
        value(&h, "ws_clock_timer_slack_ns"),
        slack,
        "the clock thread's own slack, read back now ({sched})"
    );
    assert_eq!(value(&h, "ws_clock_timer_slack_start_ns"), slack);
    assert_eq!(value(&h, "ws_clock_timer_slack_set"), 1);
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
        owed: 6,
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
    // ring named directly, not through ws_config::provider() (F-C): a provider() moved to another crate fails here.
    let ring = tokio_rustls::rustls::crypto::ring::default_provider();
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
///   inside that poll's own [start, end] stamps, and the receipt is the poll's end. This is the release half of 1.5's
///   identity (delay = model delivery − write stamp + lateness) only (NC-U3c-3): the intake, from the client's send
///   to the record's model write stamp (the client's kernel TCP and TLS, the pump's wake and its read of S), is not
///   in it; it is printed, never judged;
/// - down, the bytes leave DelayStream into the kernel's real TCP and the client's TLS, which no in-process stamp
///   sees, so the receipt is a bound (never before the release), not the identity, and the rest of the path is
///   printed;
/// - every stall is a first-transmission loss and their count sits in its 99.9 % two-sided binomial band at 2 %;
/// - every release's lateness split sums to its total exactly, and no violation is counted (`split_identity`).
///
/// Lateness is printed with condition 10's trigger, and below it the per-release split from the run's rows, pooled per
/// direction over the process's connections (the advisor's "condition 10 again" and (b): reported, never judged), and
/// this connection's five largest releases. Returns the number of lateness samples per direction.
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
            // The intake (NC-U3c-3): each up message's model write stamp less its send stamp. The up records after the
            // request are the client's frames in order, one per message.
            let up_writes: Vec<u64> = logs
                .steps
                .iter()
                .filter_map(|&(t, st)| matches!(st, Step::Write { end: UP, .. }).then_some(t))
                .skip(1)
                .collect();
            let mut intake: Vec<u64> = recv
                .iter()
                .zip(&up_writes)
                .map(|(&((_, wrote), _), &t)| (s.origin + t).saturating_sub(wrote))
                .collect();
            println!(
                "dir {dir}: receipt − (delivery + lateness): {}; the poll's own window: {}; intake (model write − \
                 send): {}",
                quantiles(&mut rest),
                quantiles(&mut window),
                quantiles(&mut intake)
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
    // The split's parts are the run's, pooled over the process's connections (the advisor's (b)): in the evidence run
    // (one test, one connection) they are this connection's; in check.mjs's suite other tests add theirs.
    let pooled = DelayStream::split_rows();
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
        for line in split_lines(&pooled, d) {
            println!("  {name} {line}");
        }
        for line in largest(&s.logs.splits, d, 5) {
            println!("  {name} {line}");
        }
    }
    // The per-release split's identity on this run's releases (the snapshot above: the traffic and its drain ended).
    split_identity(&s).unwrap_or_else(|e| panic!("the split's identity: {e}"));
    n
}

/// The loopback run as check.mjs runs it: 300 ticks (10 s).
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn loopback_tls_tungstenite_at_p2_30_hz_both_ways() {
    loopback(300).await;
}

/// One short real session for the split's identity (TLS 1.3, DelayStream, tokio-tungstenite at P2, 30 Hz both ways,
/// `ticks` ticks of 1,024 B down and 60 B up, then 1 s of drain), with `bite` on from just after the handshakes: its
/// atomic snapshot.
async fn split_session(bite: SplitBite, ticks: u64) -> Snapshot {
    let (mut server, mut client, _, _) = pair(&cfg(Profile::p2(), 23), 0).await;
    server.get_ref().bite_split(bite);
    let client_task = tokio::spawn(async move {
        run_ticks(
            &mut client,
            ticks,
            Duration::from_secs(1),
            |_, _| {},
            |k, _| Some(stamped(k, 60)),
        )
        .await;
    });
    run_ticks(
        &mut server,
        ticks,
        Duration::from_secs(1),
        |_, _| {},
        |k, _| Some(stamped(k, 1_024)),
    )
    .await;
    client_task.await.expect("the client");
    server.get_ref().snapshot()
}

/// The per-release split's identity (the advisor's "condition 10 again", 2026-11-08), over three short real loopback
/// sessions side by side (`split_session`, 60 ticks each, about 3 s). With no bite, every release's parts sum to its
/// total in integer ns, its stamps chain, the classes partition the releases the total counted, no violation is
/// counted, and the woken class is not empty either way. The bites, on the production path (`late`) from just after
/// the handshakes: a part dropped (the woken hop recorded as 0) and a part misattributed (the woken work measured from
/// the clock's wake (1) instead of the pass's start (2)); each must break the sum, and `late`'s own check must count
/// each broken release as a violation (exactly the log's broken releases). A woken release's hop is the clock thread's
/// wake call plus tokio's turn, never 0 ns, so each bite bites on every woken release after the handshake. The split's
/// histograms are the process's pool (the advisor's (b)): over the three sessions it grew by at least what their logs
/// hold, part by part, kind by kind, and in violations.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_lateness_split_sums_to_the_total_per_release_and_its_bites_break_it() {
    let before = pool().read();
    let runs = [SplitBite::Off, SplitBite::DropHop, SplitBite::WorkFromWake]
        .map(|b| (b, tokio::spawn(split_session(b, 60))));
    let mut all = Tally::default();
    for (bite, run) in runs {
        let s = run.await.expect("the session");
        let t = tally(&s);
        for d in [DOWN, UP] {
            for c in 0..2 {
                all.n[d][c] += t.n[d][c];
            }
            for k in 0..KINDS {
                all.kinds[d][k] += t.kinds[d][k];
            }
            all.broken[d] += t.broken[d];
        }
        assert_eq!(
            s.split_violations, t.broken,
            "{bite:?}: the violations late() counted are the log's broken releases"
        );
        let got = split_identity(&s);
        if bite == SplitBite::Off {
            let n = got.unwrap_or_else(|e| panic!("the identity: {e}"));
            println!(
                "split identity over {} releases: down woken {} other {}, up woken {} other {}",
                s.logs.splits.len(),
                n[DOWN][0],
                n[DOWN][1],
                n[UP][0],
                n[UP][1]
            );
            for d in [DOWN, UP] {
                assert!(n[d][0] > 0, "dir {d}: no woken release");
            }
        } else {
            let e = got.expect_err(&format!("the bite {bite:?} must break the identity"));
            assert!(e.contains("parts sum"), "{bite:?}: {e}");
            assert!(
                t.broken.iter().sum::<u64>() > 0,
                "{bite:?}: violations counted"
            );
            println!(
                "bite {bite:?} caught: {e}; violations counted {:?}",
                s.split_violations
            );
        }
    }
    let after = pool().read();
    pool_grew(&before, &after, &all).unwrap_or_else(|e| panic!("the pool: {e}"));
}

/// Condition 10's evidence run (the main session's, quiet, nothing else alongside): the same run with at least 10,000
/// lateness samples per direction, each direction's per-release split printed under its lateness, and the split's
/// identity checked on every release. `NPT_WS_LATENESS_TICKS` sets the ticks (default 10,050, about 5.6 min). The load
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

/// One run of mechanism test (c)'s connection.
#[derive(Debug)]
struct WriterRun {
    /// Each per-return record with N at the writer's last write before it (the formula's, from the srtt the gate read
    /// then) and N at the return's own srtt.
    judged: Vec<(Return, u64, u64)>,
    /// The bursts' starts asked for (DelayStream's clock).
    starts: [u64; 2],
    written: u64,
    skipped: u64,
    /// Writes after which tungstenite still owed bytes.
    owed_after: u64,
    ticks: u64,
}

/// Mechanism test (c)'s connection: the server's shipped per-tick writer (with `bite`, or reading unsent bytes only)
/// writes one 1,024 B snapshot (1,050 B in the model) per tick over TLS and tokio-tungstenite at P2, through two 1 s
/// bursts added after the wrap at `bursts` from the start. Each write goes through the Sink without waiting (fed, then
/// one flush attempt), as a WsServer's tick must never park: what tungstenite cannot pass on stays in its buffer, and
/// DelayStream counts it as owed. The gate reads its inputs in one reading (`gate_inputs`: one lock, one step; F11).
///
/// It runs until the model has recorded both returns and, after each, its first in-order byte and first whole message
/// down (S12; model state, F-K), with a hang guard of `hang_ticks`. N at each write is recorded (the formula's, from the
/// srtt the gate read), and each write is matched to its record in the log (one record of SNAP model bytes per gated
/// write, asserted), so each return is judged against N at the last write whose model stamp is before it (NC-U3c-1).
async fn writer_run(
    seed: u64,
    conn: u32,
    bite: Option<Bite>,
    unsent_only: bool,
    bursts: [u64; 2],
    hang_ticks: u64,
) -> WriterRun {
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
    let mut n_at_write = Vec::new();
    let t0 = tokio::time::Instant::now();
    let done = |rets: &[Return]| {
        rets.len() == 2
            && rets
                .iter()
                .all(|r| r.first_byte_ns[DOWN].is_some() && r.first_message_ns[DOWN].is_some())
    };
    let mut ticks = 0;
    while !done(&server.get_ref().returns()) {
        assert!(
            ticks < hang_ticks,
            "hang guard: {hang_ticks} ticks without both returns' first byte and message"
        );
        read_until(
            &mut server,
            t0 + Duration::from_nanos(ticks * TICK),
            &mut |_, _| {},
        )
        .await;
        // Whatever tungstenite still holds gets one more chance to go (none is expected: the gate needs owed 0).
        let _ = server.flush().now_or_never();
        let ds = server.get_ref();
        let mut inp = ds.gate_inputs();
        if unsent_only {
            inp.outstanding = ds.unsent_bytes();
        }
        if w.on_tick(
            Snap {
                id: ticks,
                size: SNAP,
            },
            inp,
        )
        .is_some()
        {
            n_at_write.push(snapshot_budget(inp.srtt_ns, TICK));
            let _ = server
                .feed(Message::Binary(vec![0x5a; 1_024].into()))
                .now_or_never();
            let _ = server.flush().now_or_never();
            owed_after += u64::from(server.get_ref().owed_bytes() != 0);
        }
        ticks += 1;
    }
    let ds = server.get_ref();
    let rets = ds.returns();
    let s = ds.snapshot();
    let wrote: Vec<u64> = s
        .logs
        .steps
        .iter()
        .filter_map(|&(t, st)| {
            (st == Step::Write {
                end: DOWN,
                bytes: SNAP,
            })
            .then_some(t)
        })
        .collect();
    assert_eq!(
        wrote.len(),
        n_at_write.len(),
        "each gated write is one record of {SNAP} model bytes"
    );
    let judged = rets
        .into_iter()
        .map(|r| {
            let at = r.at_ns - s.origin;
            let last = wrote.iter().filter(|&&t| t < at).count();
            assert!(last > 0, "a write before the return at {}", r.at_ns);
            (r, n_at_write[last - 1], snapshot_budget(r.srtt_ns, TICK))
        })
        .collect();
    drop(server);
    reader.abort();
    WriterRun {
        judged,
        starts: bursts.map(|b| start + b),
        written: w.written(),
        skipped: w.skipped(),
        owed_after,
        ticks,
    }
}

/// Mechanism test (c) (u3c-plan.md 1.4, C9): on DelayStream at P2 under B1, through TLS 1.3 and tokio-tungstenite on
/// loopback, the shipped writer keeps outstanding <= N x size at every return, N being the one in force at the
/// writer's last write before the return (NC-U3c-1: the gate guarantees outstanding <= N(srtt at a write) x size after
/// each write, and outstanding never grows between writes; srtt, and so N, can fall after the budget filled, so N at
/// the return's own srtt is not the bound), and leaves nothing owed after any write. Identities on model state:
/// - each return lands at its burst's end on DelayStream's clock (start + burst offset + 1 s: F-B, C5);
/// - not vacuous (NC-U3c-10): at each faithful return the gate would refuse on outstanding, outstanding > (N(srtt at
///   the return) − 1) x size: the budget was reached (no acknowledgement arrives late in a 1 s burst, so outstanding
///   and srtt stand as at the last refusal);
/// - S12's per-burst times (C8): the first in-order byte down at least one least delay after the return, the first
///   whole message no earlier.
///
/// Counts: 2 faithful connections and one per bite, run together; 2 bursts each (1 s, at 1.5 s and 4.5 s plus a phase
/// of 0, 11, 17 or 29 ms); each runs until both returns have their first byte and message down (model state, F-K;
/// typically about 230 ticks, 7.7 s; hang guard 900 ticks, 30 s). Identities, so no band. Bites (d): budget_scale 2
/// and "outstanding = unsent only" (SIOCOUTQNSD) each break the bound at a return.
///
/// What can fail and what cannot (the critic's note): "owed = 0 before a write" is not asserted, since the gate
/// (may_write) requires it and it could never fail here; what is asserted is that nothing is owed AFTER each write,
/// which fails if a snapshot the gate let through did not fit DelayStream's room whole (SNAP counting the frame
/// wrongly, or a record rule that takes less than the room allows).
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn c_the_writer_keeps_outstanding_within_n_snapshots_at_every_return() {
    const HANG_TICKS: u64 = 900;
    let at = |phase_ms: u64| [1_500 * MS + phase_ms * MS, 4_500 * MS + phase_ms * MS];
    let (a, b, x2, unsent) = tokio::join!(
        writer_run(41, 0, None, false, at(0), HANG_TICKS),
        writer_run(42, 1, None, false, at(11), HANG_TICKS),
        writer_run(43, 2, Some(Bite::BudgetX2), false, at(17), HANG_TICKS),
        writer_run(44, 3, None, true, at(29), HANG_TICKS),
    );
    let over = |run: &WriterRun| {
        run.judged
            .iter()
            .filter(|(r, n, _)| r.outstanding > n * SNAP)
            .count()
    };
    for (name, run) in [
        ("faithful A", &a),
        ("faithful B", &b),
        ("budget x2", &x2),
        ("unsent only", &unsent),
    ] {
        let shown: Vec<String> = run
            .judged
            .iter()
            .map(|(r, n, n_ret)| {
                format!(
                    "{} B vs {} B (N {n} at the last write, {n_ret} at the return)",
                    r.outstanding,
                    n * SNAP
                )
            })
            .collect();
        println!(
            "(c) {name}: {} ticks, {} written, {} skipped, owed after a write {}; at each return: {}",
            run.ticks,
            run.written,
            run.skipped,
            run.owed_after,
            shown.join("; ")
        );
        assert_eq!(run.judged.len(), 2, "{name}: both bursts returned");
        for ((r, _, _), start) in run.judged.iter().zip(run.starts) {
            assert_eq!(
                r.at_ns,
                start + SEC,
                "{name}: the return on DelayStream's clock"
            );
        }
    }
    let least = Profile::p2().one_way_ns() - Profile::p2().half_jitter_ns();
    for run in [&a, &b] {
        assert_eq!(over(run), 0, "outstanding within N x size at every return");
        assert_eq!(run.owed_after, 0, "nothing owed after any write");
        for (r, _, n_ret) in &run.judged {
            assert!(
                r.outstanding > (n_ret - 1) * SNAP,
                "the budget was reached: {} B > ({n_ret} - 1) x {SNAP} B",
                r.outstanding
            );
            // G-NC4: nothing owed above DelayStream at a faithful return (the gate writes only with owed 0, and every
            // write it let through was taken whole); the Pending case is one_record_per_write's.
            assert_eq!(r.owed, 0, "owed at the return");
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
    assert!(over(&x2) > 0, "the budget x2 bite breaks (c)");
    assert!(over(&unsent) > 0, "the unsent-only bite breaks (c)");
}
