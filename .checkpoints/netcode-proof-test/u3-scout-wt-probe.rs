// Throwaway probe for the U3 scout (R5-TEST-OQ2), netcode-proof-test. Not engine code; never shipped.
// Built from .gate/netcode-proof-test/u3-wt-probe/Cargo.toml (offline, the workspace's pinned versions).
// Proves: an ImpairSocket-lite (fixed one-way delay each way, released by a dedicated OS thread) implements
// quinn::AsyncUdpSocket; wtransport 0.7.2 and web-transport-quinn 0.13.6 servers accept sessions over it; quinn's RTT
// shows the injected delay; and the release lateness of the thread clock against tokio's sleep, in this container.

use std::collections::VecDeque;
use std::fmt;
use std::io::{self, IoSliceMut};
use std::net::{IpAddr, SocketAddr};
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

use quinn::udp::{EcnCodepoint, RecvMeta, Transmit, UdpSocketState};
use tokio::io::Interest;

struct Pkt {
    due: Instant,
    addr: SocketAddr,
    ecn: Option<EcnCodepoint>,
    ip: Option<IpAddr>,
    data: Vec<u8>,
}

#[derive(Default)]
struct Shared {
    out_q: VecDeque<Pkt>,
    in_q: VecDeque<Pkt>,
    recv_waker: Option<Waker>,
    late_out_ns: Vec<u64>,
    late_in_ns: Vec<u64>,
    sent: u64,
    recvd: u64,
    stop: bool,
}

struct ImpairSocket {
    io: tokio::net::UdpSocket,
    state: UdpSocketState,
    sh: Mutex<Shared>,
    cv: Condvar,
    delay_out: Duration,
    delay_in: Duration,
}

impl fmt::Debug for ImpairSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImpairSocket").finish_non_exhaustive()
    }
}

impl ImpairSocket {
    fn bind(addr: SocketAddr, delay_out: Duration, delay_in: Duration) -> io::Result<Arc<Self>> {
        let std_sock = std::net::UdpSocket::bind(addr)?;
        let state = UdpSocketState::new((&std_sock).into())?; // sets non-blocking, GRO, DF (as TokioRuntime does)
        let io = tokio::net::UdpSocket::from_std(std_sock)?;
        let s = Arc::new(Self { io, state, sh: Mutex::new(Shared::default()), cv: Condvar::new(), delay_out, delay_in });
        let clock = s.clone();
        std::thread::Builder::new().name("impair-clock".into()).spawn(move || clock.clock_loop())?;
        let reader = s.clone();
        tokio::spawn(async move { reader.read_loop().await });
        Ok(s)
    }

    // The dedicated release clock: outbound datagrams go to the wire at their due time; quinn is woken when the head
    // of the inbound queue is due.
    fn clock_loop(self: Arc<Self>) {
        let mode = std::env::var("IMPAIR_MODE").unwrap_or_default();
        if mode.contains("slack") {
            let r = std::fs::write("/proc/thread-self/timerslack_ns", "1");
            println!("clock thread: timerslack_ns=1 -> {:?}; now {:?}", r.is_ok(), std::fs::read_to_string("/proc/thread-self/timerslack_ns").ok());
        }
        let spin = mode.contains("spin");
        let mut g = self.sh.lock().unwrap();
        loop {
            if g.stop {
                return;
            }
            let now = Instant::now();
            let mut due_out = Vec::new();
            while g.out_q.front().is_some_and(|p| p.due <= now) {
                due_out.push(g.out_q.pop_front().unwrap());
            }
            if g.in_q.front().is_some_and(|p| p.due <= now) {
                if let Some(w) = g.recv_waker.take() {
                    w.wake();
                }
            }
            if !due_out.is_empty() {
                drop(g);
                for p in &due_out {
                    let t = Transmit { destination: p.addr, ecn: p.ecn, contents: &p.data, segment_size: None, src_ip: p.ip };
                    loop {
                        match self.state.send((&self.io).into(), &t) {
                            Ok(()) => break,
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => std::thread::sleep(Duration::from_micros(50)),
                            Err(_) => break,
                        }
                    }
                    let late = Instant::now().saturating_duration_since(p.due).as_nanos() as u64;
                    let mut g2 = self.sh.lock().unwrap();
                    g2.late_out_ns.push(late);
                    g2.sent += 1;
                }
                g = self.sh.lock().unwrap();
                continue;
            }
            let mut next = g.out_q.front().map(|p| p.due);
            if g.recv_waker.is_some() {
                if let Some(d) = g.in_q.front().map(|p| p.due) {
                    next = Some(next.map_or(d, |n| n.min(d)));
                }
            }
            g = match next {
                Some(t) => {
                    let now = Instant::now();
                    let tail = Duration::from_micros(200);
                    if spin && t.saturating_duration_since(now) <= tail {
                        drop(g);
                        while Instant::now() < t {
                            std::hint::spin_loop();
                        }
                        self.sh.lock().unwrap()
                    } else if spin {
                        self.cv.wait_timeout(g, t.saturating_duration_since(now) - tail).unwrap().0
                    } else {
                        self.cv.wait_timeout(g, t.saturating_duration_since(now)).unwrap().0
                    }
                }
                None => self.cv.wait(g).unwrap(),
            };
        }
    }

    // The reader: drains the real socket as soon as epoll says readable, stamps arrival on this host's monotonic clock,
    // splits GRO buffers by stride, and queues each datagram with its due time.
    async fn read_loop(self: Arc<Self>) {
        let mut bufs: Vec<Vec<u8>> = (0..4).map(|_| vec![0u8; 1472 * 64]).collect();
        let mut metas = [RecvMeta::default(); 4];
        loop {
            if self.io.readable().await.is_err() {
                return;
            }
            let res = {
                let mut iovs: Vec<IoSliceMut<'_>> = bufs.iter_mut().map(|b| IoSliceMut::new(b)).collect();
                self.io.try_io(Interest::READABLE, || self.state.recv((&self.io).into(), &mut iovs, &mut metas))
            };
            let n = match res {
                Ok(n) => n,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(_) => continue,
            };
            let now = Instant::now();
            let mut g = self.sh.lock().unwrap();
            for i in 0..n {
                let m = metas[i];
                for chunk in bufs[i][..m.len].chunks(m.stride.max(1)) {
                    g.in_q.push_back(Pkt { due: now + self.delay_in, addr: m.addr, ecn: m.ecn, ip: m.dst_ip, data: chunk.to_vec() });
                }
            }
            drop(g);
            self.cv.notify_one();
        }
    }

    fn stats(&self) -> (Vec<u64>, Vec<u64>, u64, u64) {
        let g = self.sh.lock().unwrap();
        (g.late_out_ns.clone(), g.late_in_ns.clone(), g.sent, g.recvd)
    }
}

#[derive(Debug)]
struct AlwaysWritable;
impl quinn::UdpPoller for AlwaysWritable {
    fn poll_writable(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl quinn::AsyncUdpSocket for ImpairSocket {
    fn create_io_poller(self: Arc<Self>) -> Pin<Box<dyn quinn::UdpPoller>> {
        Box::pin(AlwaysWritable)
    }

    fn try_send(&self, t: &Transmit<'_>) -> io::Result<()> {
        let now = Instant::now();
        let seg = t.segment_size.unwrap_or(t.contents.len()).max(1);
        let mut g = self.sh.lock().unwrap();
        for chunk in t.contents.chunks(seg) {
            g.out_q.push_back(Pkt { due: now + self.delay_out, addr: t.destination, ecn: t.ecn, ip: t.src_ip, data: chunk.to_vec() });
        }
        drop(g);
        self.cv.notify_one();
        Ok(())
    }

    fn poll_recv(&self, cx: &mut Context<'_>, bufs: &mut [IoSliceMut<'_>], meta: &mut [RecvMeta]) -> Poll<io::Result<usize>> {
        let now = Instant::now();
        let mut g = self.sh.lock().unwrap();
        let mut n = 0;
        while n < bufs.len().min(meta.len()) && g.in_q.front().is_some_and(|p| p.due <= now) {
            let p = g.in_q.pop_front().unwrap();
            let len = p.data.len().min(bufs[n].len());
            bufs[n][..len].copy_from_slice(&p.data[..len]);
            let mut m = RecvMeta::default();
            m.addr = p.addr;
            m.len = len;
            m.stride = len;
            m.ecn = p.ecn;
            m.dst_ip = p.ip;
            meta[n] = m;
            g.late_in_ns.push(now.saturating_duration_since(p.due).as_nanos() as u64);
            g.recvd += 1;
            n += 1;
        }
        if n > 0 {
            return Poll::Ready(Ok(n));
        }
        g.recv_waker = Some(cx.waker().clone());
        drop(g);
        self.cv.notify_one();
        Poll::Pending
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.io.local_addr()
    }

    fn max_transmit_segments(&self) -> usize {
        1
    }

    fn max_receive_segments(&self) -> usize {
        1
    }

    fn may_fragment(&self) -> bool {
        self.state.may_fragment()
    }
}

fn pct(v: &mut [u64], p: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_unstable();
    let i = ((v.len() as f64 - 1.0) * p).round() as usize;
    v[i] as f64 / 1000.0
}

fn report(name: &str, v: &mut Vec<u64>) {
    let n = v.len();
    let mean = if n == 0 { f64::NAN } else { v.iter().sum::<u64>() as f64 / n as f64 / 1000.0 };
    println!(
        "{name}: n={n} mean={mean:.1}us p50={:.1}us p99={:.1}us p999={:.1}us max={:.1}us",
        pct(v, 0.5),
        pct(v, 0.99),
        pct(v, 0.999),
        pct(v, 1.0)
    );
}

async fn timer_bench() {
    let mut tk = Vec::new();
    for i in 0..300u64 {
        let due = Instant::now() + Duration::from_micros(2_000 + (i * 37) % 1000);
        tokio::time::sleep_until(due.into()).await;
        tk.push(Instant::now().saturating_duration_since(due).as_nanos() as u64);
    }
    report("tokio sleep_until lateness", &mut tk);
    let slack = std::env::var("IMPAIR_MODE").unwrap_or_default().contains("slack");
    let th = std::thread::spawn(move || {
        if slack {
            let _ = std::fs::write("/proc/thread-self/timerslack_ns", "1");
        }
        let m = Mutex::new(());
        let cv = Condvar::new();
        let mut v = Vec::new();
        for i in 0..300u64 {
            let due = Instant::now() + Duration::from_micros(2_000 + (i * 37) % 1000);
            let mut g = m.lock().unwrap();
            loop {
                let now = Instant::now();
                if now >= due {
                    break;
                }
                g = cv.wait_timeout(g, due - now).unwrap().0;
            }
            v.push(Instant::now().saturating_duration_since(due).as_nanos() as u64);
        }
        v
    });
    let mut v = th.join().unwrap();
    report("thread Condvar::wait_timeout lateness", &mut v);
}

// Raw UDP through ImpairSocket without quinn, as PM-0001-3's probe would drive it: 10,000 datagrams at about
// 10 kpps toward a local sink, outbound release lateness measured.
async fn raw_probe() -> io::Result<()> {
    let sock = ImpairSocket::bind("127.0.0.1:0".parse().unwrap(), Duration::from_millis(25), Duration::from_millis(25))?;
    let sink = tokio::net::UdpSocket::bind("127.0.0.1:0").await?;
    let dest = sink.local_addr()?;
    let recv_task = tokio::spawn(async move {
        let mut buf = [0u8; 2048];
        let mut got = 0u64;
        while let Ok(Ok(_)) = tokio::time::timeout(Duration::from_millis(500), sink.recv_from(&mut buf)).await {
            got += 1;
        }
        got
    });
    let payload = [7u8; 100];
    let start = Instant::now();
    for i in 0..10_000u32 {
        let t = Transmit { destination: dest, ecn: None, contents: &payload, segment_size: None, src_ip: None };
        quinn::AsyncUdpSocket::try_send(&*sock, &t)?;
        if i % 10 == 9 {
            tokio::time::sleep_until((start + Duration::from_micros(100 * (i as u64 + 1))).into()).await;
        }
    }
    let got = recv_task.await.unwrap();
    let (mut lo, _, sent, _) = sock.stats();
    println!("raw probe: handed 10000, released {sent}, sink got {got}, elapsed send phase {:?}", start.elapsed());
    report("raw probe outbound release lateness (10 kpps)", &mut lo);
    sock.sh.lock().unwrap().stop = true;
    sock.cv.notify_all();
    Ok(())
}

async fn wtransport_route(identity: wtransport::Identity, hash: wtransport::tls::Sha256Digest) -> Result<(), Box<dyn std::error::Error>> {
    let one_way = Duration::from_millis(20);
    let cfg = wtransport::ServerConfig::builder()
        .with_bind_address("127.0.0.1:0".parse().unwrap()) // placeholder: never bound, Endpoint::server is not used
        .with_identity(identity)
        .build();
    let sock = ImpairSocket::bind("127.0.0.1:0".parse().unwrap(), one_way, one_way)?;
    let ep = quinn::Endpoint::new_with_abstract_socket(
        cfg.quic_endpoint_config().clone(),
        Some(cfg.quic_config().clone()),
        sock.clone(),
        Arc::new(quinn::TokioRuntime),
    )?;
    let port = ep.local_addr()?.port();
    let server = tokio::spawn(async move {
        let incoming = ep.accept().await.expect("endpoint open");
        let peer = incoming.remote_address();
        let req = wtransport::endpoint::IncomingSessionFuture::with_quic_incoming(incoming).await.expect("session request");
        println!("wtransport server: request from {peer}, path {}", req.path());
        let conn = req.accept().await.expect("accept");
        for _ in 0..30 {
            let d = conn.receive_datagram().await.expect("dgram");
            conn.send_datagram(d.payload()).expect("send");
        }
        println!("wtransport server: quinn rtt {:?}", conn.rtt());
        (ep, conn)
    });
    let ccfg = wtransport::ClientConfig::builder()
        .with_bind_address("127.0.0.1:0".parse().unwrap())
        .with_server_certificate_hashes([hash])
        .build();
    let client = wtransport::Endpoint::client(ccfg)?;
    let t0 = Instant::now();
    let conn = client.connect(format!("https://127.0.0.1:{port}/npt")).await?;
    println!("wtransport client: session up in {:?}", t0.elapsed());
    let mut rtts = Vec::new();
    for i in 0..30u8 {
        let s = Instant::now();
        conn.send_datagram([i; 64])?;
        let _ = conn.receive_datagram().await?;
        rtts.push(s.elapsed().as_nanos() as u64);
        tokio::time::sleep(Duration::from_millis(15)).await;
    }
    report("wtransport client datagram echo RTT (injected 2 x 20 ms)", &mut rtts);
    println!("wtransport client: quinn rtt {:?}", conn.rtt());
    let (_ep, _sconn) = server.await?;
    let (mut lo, mut li, sent, recvd) = sock.stats();
    println!("wtransport ImpairSocket: released out {sent}, delivered in {recvd}");
    report("  outbound release lateness", &mut lo);
    report("  inbound delivery lateness (due -> quinn poll_recv)", &mut li);
    Ok(())
}

async fn wtq_route(cfg_src: &wtransport::ServerConfig, ccfg_src: &wtransport::ClientConfig) -> Result<(), Box<dyn std::error::Error>> {
    let one_way = Duration::from_millis(20);
    let sock = ImpairSocket::bind("127.0.0.1:0".parse().unwrap(), one_way, one_way)?;
    let ep = quinn::Endpoint::new_with_abstract_socket(
        quinn::EndpointConfig::default(),
        Some(cfg_src.quic_config().clone()), // TLS 1.3, ALPN h3: what web-transport-quinn's Server::new asks for
        sock.clone(),
        Arc::new(quinn::TokioRuntime),
    )?;
    let port = ep.local_addr()?.port();
    let server = tokio::spawn(async move {
        let mut server = web_transport_quinn::Server::new(ep);
        let req = server.accept().await.expect("request");
        println!("web-transport-quinn server: request url {}", req.url);
        let session = req.ok().await.expect("ok");
        for _ in 0..30 {
            let d = session.read_datagram().await.expect("dgram");
            session.send_datagram(d).expect("send");
        }
        println!("web-transport-quinn server: quinn rtt {:?}", req_rtt(&session));
        (server, session)
    });
    let qep = quinn::Endpoint::client("127.0.0.1:0".parse().unwrap())?;
    let client = web_transport_quinn::Client::new(qep, ccfg_src.quic_config().clone());
    let url: url_crate::Url = format!("https://127.0.0.1:{port}/npt").parse()?;
    let t0 = Instant::now();
    let session = client.connect(url).await?;
    println!("web-transport-quinn client: session up in {:?}", t0.elapsed());
    let mut rtts = Vec::new();
    for i in 0..30u8 {
        let s = Instant::now();
        session.send_datagram(bytes_crate::Bytes::from(vec![i; 64]))?;
        let _ = session.read_datagram().await?;
        rtts.push(s.elapsed().as_nanos() as u64);
        tokio::time::sleep(Duration::from_millis(15)).await;
    }
    report("web-transport-quinn client datagram echo RTT (injected 2 x 20 ms)", &mut rtts);
    let (_server, _s) = server.await?;
    let (mut lo, mut li, sent, recvd) = sock.stats();
    println!("web-transport-quinn ImpairSocket: released out {sent}, delivered in {recvd}");
    report("  outbound release lateness", &mut lo);
    report("  inbound delivery lateness (due -> quinn poll_recv)", &mut li);
    Ok(())
}

fn req_rtt(s: &web_transport_quinn::Session) -> Duration {
    // Session derefs to quinn::Connection in 0.13 if available; fall back to the trait-free path below.
    quinn_conn(s).rtt()
}

fn quinn_conn(s: &web_transport_quinn::Session) -> &quinn::Connection {
    s
}

use bytes as bytes_crate;
use url as url_crate;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("IMPAIR_MODE={:?}; default timerslack_ns of main thread: {:?}", std::env::var("IMPAIR_MODE").unwrap_or_default(), std::fs::read_to_string("/proc/thread-self/timerslack_ns").ok());
    timer_bench().await;
    raw_probe().await?;
    if std::env::var("SKIP_ROUTES").is_ok() {
        println!("PROBE DONE (routes skipped)");
        return Ok(());
    }
    let identity = wtransport::Identity::self_signed(["localhost", "127.0.0.1"]).unwrap();
    let hash = identity.certificate_chain().as_slice()[0].hash();
    let cfg_src = wtransport::ServerConfig::builder()
        .with_bind_address("127.0.0.1:0".parse().unwrap())
        .with_identity(identity.clone_identity())
        .build();
    let ccfg_src = wtransport::ClientConfig::builder()
        .with_bind_address("127.0.0.1:0".parse().unwrap())
        .with_server_certificate_hashes([hash.clone()])
        .build();
    wtransport_route(identity, hash).await?;
    wtq_route(&cfg_src, &ccfg_src).await?;
    println!("PROBE DONE");
    Ok(())
}
