//! ImpairSocket on loopback, through quinn's socket trait itself (the code quinn calls), without quinn: M1, M5, M10,
//! M11, M12, M13, M14. The WebTransport routes are in `wt`'s tests.

use super::*;
use npt_l1::{Profile, ReorderStep};
use quinn::AsyncUdpSocket;
use std::sync::Arc;
use std::task::Wake;
use std::time::{Duration, Instant};

const MS: u64 = 1_000_000;

fn local() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 0))
}

fn fixed(one_way_ms: u64) -> Profile {
    Profile {
        name: "T",
        rtt_ns: 2 * one_way_ms * MS,
        jitter_ns: 0,
        loss_ppm: 0,
        bursts_ns: Vec::new(),
        reorder: ReorderStep::OFF,
    }
}

fn cfg(profile: Profile) -> L2Config {
    L2Config {
        pool_buffers: 256,
        ..L2Config::new(7, profile)
    }
}

fn value(list: &[(&'static str, u64)], name: &str) -> u64 {
    list.iter()
        .find(|(n, _)| *n == name)
        .unwrap_or_else(|| panic!("no {name}"))
        .1
}

/// The run summary once `name` reaches `at_least` (the release thread records a send just after the kernel took it),
/// or after 5 s.
fn summary_when(s: &ImpairSocket, name: &str, at_least: u64) -> Vec<(&'static str, u64)> {
    let end = Instant::now() + Duration::from_secs(5);
    loop {
        let r = s.run_summary();
        if value(&r, name) >= at_least || Instant::now() >= end {
            return r;
        }
        std::thread::sleep(Duration::from_millis(2));
    }
}

struct Unpark(std::thread::Thread);
impl Wake for Unpark {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Polls `poll_recv` as quinn's driver does: Pending stores the waker; the thread parks until woken.
fn recv(s: &ImpairSocket, timeout: Duration) -> Option<(Vec<u8>, SocketAddr, Instant)> {
    let waker = Waker::from(Arc::new(Unpark(std::thread::current())));
    let mut cx = Context::from_waker(&waker);
    let end = Instant::now() + timeout;
    let mut buf = [0u8; 1_500];
    loop {
        let mut iov = [IoSliceMut::new(&mut buf)];
        let mut meta = [RecvMeta::default()];
        match s.poll_recv(&mut cx, &mut iov, &mut meta) {
            Poll::Ready(Ok(n)) => {
                assert_eq!(n, 1, "never Ok(0); one buffer, one datagram");
                let m = meta[0];
                assert_eq!(m.stride, m.len, "one datagram per buffer");
                return Some((buf[..m.len].to_vec(), m.addr, Instant::now()));
            }
            Poll::Ready(Err(e)) => panic!("poll_recv errored: {e}"),
            Poll::Pending => {
                let now = Instant::now();
                if now >= end {
                    return None;
                }
                std::thread::park_timeout(end - now);
            }
        }
    }
}

fn transmit(to: SocketAddr, data: &[u8]) -> Transmit<'_> {
    Transmit {
        destination: to,
        ecn: None,
        contents: data,
        segment_size: None,
        src_ip: None,
    }
}

/// M11: never WouldBlock, one datagram per Transmit, may_fragment passed through.
#[test]
fn it_never_pushes_back_and_takes_one_datagram_per_transmit() {
    let mut c = cfg(fixed(500));
    c.pool_buffers = 2;
    c.max_datagram_bytes = 1_200;
    let s = ImpairSocket::bind(local(), c).unwrap();
    assert_eq!(
        (s.max_transmit_segments(), s.max_receive_segments()),
        (1, 1)
    );
    // The real socket's: quinn-udp sets DF on Linux, so false, and quinn runs MTU discovery as on the plain path.
    let probe = std::net::UdpSocket::bind(local()).unwrap();
    let state = UdpSocketState::new((&probe).into()).unwrap();
    assert_eq!(s.may_fragment(), state.may_fragment());
    let mut poller = Arc::clone(&s).create_io_poller();
    let mut cx = Context::from_waker(Waker::noop());
    assert!(matches!(
        poller.as_mut().poll_writable(&mut cx),
        Poll::Ready(Ok(()))
    ));
    let peer = probe.local_addr().unwrap();
    // A Transmit with segments is split into its datagrams (2 x 600 B): both queued, the pool now empty.
    let two = [1u8; 1_200];
    let t = Transmit {
        segment_size: Some(600),
        ..transmit(peer, &two)
    };
    s.try_send(&t).unwrap();
    // Every one of these is dropped, and each still returns Ok: a pool drop, then an oversize drop.
    s.try_send(&transmit(peer, &[2u8; 100])).unwrap();
    s.try_send(&transmit(peer, &[3u8; 1_201])).unwrap();
    assert!(matches!(
        poller.as_mut().poll_writable(&mut cx),
        Poll::Ready(Ok(()))
    ));
    let r = s.run_summary();
    assert_eq!(value(&r, "l2_down_pool_drop_count"), 1);
    assert_eq!(value(&r, "l2_down_oversize_count"), 1);
    assert_eq!(value(&r, "l2_pool_free_count"), 0);
    // Each queued datagram counts its whole 1,500 B buffer (M10's charge), whatever its payload.
    assert_eq!(value(&r, "l2_down_queued_bytes"), 2 * 1_500);
}

/// M11 and M1: poll_recv is Pending (waker stored) until a datagram is due, then hands it over, never Ok(0); the
/// arrival is stamped at the drain, the lateness where quinn receives it.
#[test]
fn poll_recv_hands_over_only_due_datagrams() {
    let s = ImpairSocket::bind(local(), cfg(fixed(200))).unwrap();
    let peer = std::net::UdpSocket::bind(local()).unwrap();
    let sent = Instant::now();
    peer.send_to(b"hello", s.local_addr().unwrap()).unwrap();
    let mut first = None;
    while first.is_none() && sent.elapsed() < Duration::from_secs(2) {
        first = s.first_seen(peer.local_addr().unwrap());
        std::thread::sleep(Duration::from_millis(1));
    }
    let first = first.expect("stamped at the drain");
    assert!(
        first.duration_since(sent) < Duration::from_millis(100),
        "the first raw packet's stamp is its arrival, not the end of its 200 ms delay"
    );
    let (data, from, at) = recv(&s, Duration::from_secs(2)).expect("delivered");
    assert_eq!(
        (&data[..], from),
        (&b"hello"[..], peer.local_addr().unwrap())
    );
    assert!(
        at.duration_since(first) >= Duration::from_millis(200),
        "released at its due time, not before"
    );
    assert!(
        recv(&s, Duration::from_millis(50)).is_none(),
        "Pending on an empty queue, never Ok(0)"
    );
    let r = s.run_summary();
    assert_eq!(value(&r, "l2_up_delivered_count"), 1);
    assert_eq!(value(&r, "l2_late_up_count"), 1);
    assert!(value(&r, "l2_late_up_max_ns") < 1_000 * MS);
}

/// M1's inbound point: the lateness is measured at poll_recv's return, where quinn takes the datagram, not when the
/// release thread made it ready. Here quinn polls about 90 ms after the datagram was due, and the lateness says so.
/// The bite: a lateness taken at the release (or forced to 0) stays near 0.
#[test]
fn inbound_lateness_is_measured_where_quinn_receives_it() {
    let s = ImpairSocket::bind(local(), cfg(fixed(20))).unwrap();
    let peer = std::net::UdpSocket::bind(local()).unwrap();
    let sent = Instant::now();
    peer.send_to(b"late", s.local_addr().unwrap()).unwrap();
    let mut first = None;
    while first.is_none() && sent.elapsed() < Duration::from_secs(2) {
        first = s.first_seen(peer.local_addr().unwrap());
        std::thread::sleep(Duration::from_millis(1));
    }
    let first = first.expect("stamped at the drain");
    // Due at first + 20 ms; quinn does not poll until first + 110 ms.
    let poll_at = first + Duration::from_millis(110);
    std::thread::sleep(poll_at.saturating_duration_since(Instant::now()));
    assert_eq!(
        value(&s.run_summary(), "l2_up_delivered_count"),
        0,
        "nothing handed over before quinn polls"
    );
    let (data, _, at) = recv(&s, Duration::from_secs(2)).expect("delivered");
    assert_eq!(&data[..], b"late");
    let late = value(&s.run_summary(), "l2_late_up_max_ns");
    let bound = at.duration_since(first).as_nanos() as u64 - 20 * MS;
    assert!(
        (50 * MS..=bound).contains(&late),
        "inbound lateness {late} ns: at least 50 ms (polled ~90 ms after due), at most {bound} ns"
    );
}

/// The outbound path: delayed by the profile, sent by the release thread with try_send.
#[test]
fn outbound_datagrams_leave_at_their_due_time() {
    let s = ImpairSocket::bind(local(), cfg(fixed(20))).unwrap();
    let peer = std::net::UdpSocket::bind(local()).unwrap();
    peer.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    let t0 = Instant::now();
    s.try_send(&transmit(peer.local_addr().unwrap(), b"down"))
        .unwrap();
    let mut buf = [0u8; 64];
    let (n, from) = peer.recv_from(&mut buf).unwrap();
    let took = t0.elapsed();
    assert_eq!((&buf[..n], from), (&b"down"[..], s.local_addr().unwrap()));
    assert!(
        took >= Duration::from_millis(20) && took < Duration::from_millis(500),
        "{took:?}"
    );
    let r = summary_when(&s, "l2_down_delivered_count", 1);
    assert_eq!(value(&r, "l2_down_delivered_count"), 1);
    assert_eq!(value(&r, "l2_late_down_count"), 1);
    assert_eq!(value(&r, "l2_pool_free_count"), 256, "the buffer came back");
}

/// M5: a kernel send error is a logged fate, never "delivered". The bite: quinn-udp's `send` turns the same error into
/// Ok (unix.rs:195-219), so the release thread must use `try_send`.
#[test]
fn send_errors_are_fates_not_deliveries() {
    let mut c = cfg(fixed(1));
    c.fate_log_capacity = 8;
    let s = ImpairSocket::bind(local(), c).unwrap();
    // An IPv6 destination from an IPv4 socket: the kernel refuses it.
    let v6: SocketAddr = "[::1]:9".parse().unwrap();
    s.try_send(&transmit(v6, b"x")).unwrap();
    let r = summary_when(&s, "l2_down_send_err_count", 1);
    assert_eq!(value(&r, "l2_down_delivered_count"), 0);
    assert_eq!(value(&r, "l2_down_send_err_count"), 1);
    let classes: u64 = names::SEND_ERR.iter().map(|n| value(&r, n)).sum();
    assert_eq!(classes, 1);
    let mut bytes = Vec::new();
    s.drain_fate_log(|b| bytes.extend_from_slice(b));
    let e = &bytes[super::super::stats::FATE_TAG.len()..];
    assert_eq!(e.len(), super::super::stats::FATE_BYTES);
    assert_eq!(e[49], Kind::SendError as u8);
    assert_ne!(
        u32::from_le_bytes(e[44..48].try_into().unwrap()),
        0,
        "the errno is logged"
    );
    // The bite, on a plain socket: send says Ok, try_send says what happened.
    let plain = std::net::UdpSocket::bind(local()).unwrap();
    let state = UdpSocketState::new((&plain).into()).unwrap();
    assert!(state.send((&plain).into(), &transmit(v6, b"x")).is_ok());
    assert!(
        state
            .try_send((&plain).into(), &transmit(v6, b"x"))
            .is_err()
    );
}

/// M10: an inbound datagram above the size limit is dropped and counted before it is queued (no pool buffer).
#[test]
fn oversize_inbound_is_dropped_before_the_queue() {
    let mut c = cfg(fixed(200));
    c.max_datagram_bytes = 100;
    let s = ImpairSocket::bind(local(), c).unwrap();
    let peer = std::net::UdpSocket::bind(local()).unwrap();
    peer.send_to(&[0u8; 101], s.local_addr().unwrap()).unwrap();
    peer.send_to(&[0u8; 100], s.local_addr().unwrap()).unwrap();
    let r = summary_when(&s, "l2_up_queued_bytes", 1_500);
    assert_eq!(value(&r, "l2_up_oversize_count"), 1);
    assert_eq!(
        value(&r, "l2_up_queued_bytes"),
        1_500,
        "only the one within the limit is queued (its buffer's 1,500 B)"
    );
    assert_eq!(value(&r, "l2_pool_free_count"), 255);
    assert!(s.first_seen(peer.local_addr().unwrap()).is_some());
}

/// M12 and M14: the pass-through runs through the same threads (lateness recorded by the release path) with the same
/// 1 ns slack, and the header records the mode, the slack read back and the switches.
#[test]
fn the_pass_through_takes_the_same_path_and_the_header_says_so() {
    let s = ImpairSocket::bind(local(), L2Config::clean(3)).unwrap();
    let peer = std::net::UdpSocket::bind(local()).unwrap();
    peer.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    for i in 0..20u8 {
        s.try_send(&transmit(peer.local_addr().unwrap(), &[i]))
            .unwrap();
        peer.send_to(&[i], s.local_addr().unwrap()).unwrap();
    }
    let mut buf = [0u8; 8];
    for i in 0..20u8 {
        let (n, _) = peer.recv_from(&mut buf).unwrap();
        assert_eq!(&buf[..n], &[i], "in order, nothing dropped");
        assert_eq!(recv(&s, Duration::from_secs(2)).expect("up").0, vec![i]);
    }
    let r = summary_when(&s, "l2_late_down_count", 20);
    assert_eq!(
        value(&r, "l2_late_down_count"),
        20,
        "every datagram went through the release thread"
    );
    assert_eq!(value(&r, "l2_late_up_count"), 20);
    let texts = s.header_texts();
    assert!(texts.contains(&("l2_mode", "pass-through".to_owned())));
    let sched = &texts.iter().find(|t| t.0 == "l2_sched_policy").unwrap().1;
    let h = s.header_numbers();
    let expected = if sched.starts_with("SCHED_FIFO") || sched.starts_with("SCHED_RR") {
        0
    } else {
        1
    };
    assert_eq!(
        value(&h, "l2_timer_slack_ns"),
        expected,
        "the release thread's own slack, read back ({sched})"
    );
    assert_eq!(value(&h, "l2_timer_slack_start_ns"), expected);
    assert_eq!(value(&h, "l2_timer_slack_set"), 1);
    assert_eq!((value(&h, "l2_migration"), value(&h, "l2_gso")), (0, 0));
    assert_eq!(value(&h, "l2_one_way_ns"), 0);
    let impaired = ImpairSocket::bind(local(), cfg(Profile::p2())).unwrap();
    assert!(
        impaired
            .header_texts()
            .contains(&("l2_mode", "impaired".to_owned()))
    );
    assert_eq!(value(&impaired.header_numbers(), "l2_one_way_ns"), 50 * MS);
    assert_eq!(
        value(&impaired.header_numbers(), "reorder_step_permille"),
        10
    );
}

/// M14 at header time: the header's slack is the one the release thread reads when the header is built (only the
/// thread itself may read it without CAP_SYS_NICE), beside the value it read back at its start, so a change after the
/// start shows: an outside `chrt -f` zeroes it, and a return to SCHED_OTHER restores 50 µs, not 1 ns. Here the thread
/// itself writes 7 µs, standing in for that outside change. The bite: a header reporting the start value keeps 1.
#[test]
fn the_header_reads_the_slack_when_it_is_built() {
    let s = ImpairSocket::bind(local(), cfg(fixed(1))).unwrap();
    let sched = s
        .header_texts()
        .into_iter()
        .find(|t| t.0 == "l2_sched_policy")
        .unwrap()
        .1;
    let rt = sched.starts_with("SCHED_FIFO") || sched.starts_with("SCHED_RR");
    let h = s.header_numbers();
    let start = if rt { 0 } else { 1 };
    assert_eq!(
        (
            value(&h, "l2_timer_slack_ns"),
            value(&h, "l2_timer_slack_start_ns")
        ),
        (start, start)
    );
    // Under a real-time policy the kernel keeps it at 0 whatever is written (ed4fb6d7ef68).
    let moved = if rt { 0 } else { 7_000 };
    assert_eq!(s.ask_slack(Some(7_000)), Some(moved));
    let h = s.header_numbers();
    assert_eq!(
        (
            value(&h, "l2_timer_slack_ns"),
            value(&h, "l2_timer_slack_start_ns")
        ),
        (moved, start),
        "the header's slack is read when it is built ({sched})"
    );
}

/// M15: every header name has its SERIES.md row, and the run summary's extra names are named there.
#[test]
fn every_header_name_has_a_series_row() {
    let series = include_str!("../../../../marks/SERIES.md");
    let s = ImpairSocket::bind(local(), cfg(Profile::p2()).with_throttle()).unwrap();
    let mut names: Vec<&str> = s.header_numbers().iter().map(|x| x.0).collect();
    names.extend(s.header_texts().iter().map(|x| x.0));
    names.push("l2_timer_slack_error");
    for n in names {
        assert!(
            // A row of its own, or one in the table of header params the parts select on.
            series.contains(&format!("| `header.params.{n}` |"))
                || series.contains(&format!("| `{n}` |")),
            "no SERIES.md row for header.params.{n}"
        );
    }
    // The l2_summary event's fields: each named in SERIES.md, and the per-second CPU share is not one of them (the
    // summary carries the CPU in ns instead).
    let summary: Vec<&str> = s.run_summary().iter().map(|x| x.0).collect();
    assert!(!summary.contains(&"l2_release_cpu_permille"));
    for n in summary {
        assert!(
            series.contains(&format!("`second.v.{n}`")) || series.contains(&format!("`{n}`")),
            "l2_summary's {n}"
        );
    }
    for n in [
        "reorder_up_permille",
        "reorder_down_permille",
        "reorder_permille",
        "l2_release_cpu_ns",
    ] {
        assert!(series.contains(&format!("`{n}`")), "{n}");
    }
    let h = s.header_numbers();
    assert_eq!(
        (
            value(&h, "l2_shaper_rate_bits"),
            value(&h, "l2_shaper_queue_bytes"),
            value(&h, "l2_shaper_queue_ns")
        ),
        (128_000, 5_920, 370 * MS)
    );
}

/// The per-second window closes on each call; the run keeps the totals.
#[test]
fn seconds_are_windows_and_the_run_is_the_total() {
    let s = ImpairSocket::bind(local(), cfg(fixed(1))).unwrap();
    let peer = std::net::UdpSocket::bind(local()).unwrap();
    for _ in 0..3 {
        s.try_send(&transmit(peer.local_addr().unwrap(), b"x"))
            .unwrap();
    }
    summary_when(&s, "l2_down_delivered_count", 3);
    let a = s.take_second();
    assert_eq!(value(&a, "l2_down_delivered_count"), 3);
    assert!(value(&a, "l2_release_cpu_permille") <= 1_000);
    let b = s.take_second();
    assert_eq!(value(&b, "l2_down_delivered_count"), 0, "a new window");
    assert_eq!(value(&s.run_summary(), "l2_down_delivered_count"), 3);
    assert!(value(&s.run_summary(), "l2_release_cpu_ns") > 0);
}

/// Dropping the socket stops both of its threads, and the port is free again (the reader held the bound socket). The
/// bite: a Drop that does not stop the reader leaves its thread and the port behind.
#[test]
fn dropping_the_socket_ends_its_threads() {
    let s = ImpairSocket::bind(local(), cfg(fixed(1))).unwrap();
    let port = s.local_addr().unwrap();
    let threads = [
        ("release", s.release_thread_id().expect("a release tid")),
        ("reader", s.reader_thread_id().expect("a reader tid")),
    ];
    for (_, tid) in threads {
        assert!(std::fs::metadata(format!("/proc/self/task/{tid}")).is_ok());
    }
    drop(s);
    let end = Instant::now() + Duration::from_secs(2);
    for (name, tid) in threads {
        let path = format!("/proc/self/task/{tid}");
        while std::fs::metadata(&path).is_ok() && Instant::now() < end {
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(
            std::fs::metadata(&path).is_err(),
            "the {name} thread still runs"
        );
    }
    let mut again = std::net::UdpSocket::bind(port);
    while again.is_err() && Instant::now() < end {
        std::thread::sleep(Duration::from_millis(5));
        again = std::net::UdpSocket::bind(port);
    }
    assert!(again.is_ok(), "the port {port} is still bound: {again:?}");
}

/// A dedicated pacer thread (M13: never tokio): calls `f(i)` for datagram i at `start + i / rate`, catching up in
/// a burst when it wakes late, with its own 1 ns slack.
fn pace(rate: u64, n: u64, start: Instant, mut f: impl FnMut(u64)) {
    if let Ok(tid) = thread::own_tid() {
        thread::set_own_slack(tid, 1);
    }
    let gap = 1_000_000_000 / rate;
    let mut i = 0;
    while i < n {
        let now = start.elapsed().as_nanos() as u64;
        while i < n && i * gap <= now {
            f(i);
            i += 1;
        }
        if i < n {
            let next = i * gap;
            let now = start.elapsed().as_nanos() as u64;
            if next > now {
                std::thread::sleep(Duration::from_nanos(next - now));
            }
        }
    }
}

/// M13: 10,000 datagrams per second per direction sustained on loopback with jitter, loss and the reorder step on
/// (P2: 20,000 releases per second), the release thread's CPU and the lateness per direction recorded. Run for the
/// evidence: `cargo test --release -p npt-net l2_sustains -- --ignored --nocapture` (NPT_L2_SECS sets the length).
#[test]
#[ignore = "timing-heavy: 10 s at 20,000 releases per second; run in release for M13's evidence"]
fn l2_sustains_10000_per_second_each_way() {
    let secs: u64 = std::env::var("NPT_L2_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let rate = 10_000u64;
    let n = rate * secs;
    let profile = Profile::p2();
    assert_eq!(profile.reorder, ReorderStep::PART_13);
    let s = ImpairSocket::bind(local(), L2Config::new(11, profile)).unwrap();
    let peer = Arc::new(std::net::UdpSocket::bind(local()).unwrap());
    peer.set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    let (s_addr, p_addr) = (s.local_addr().unwrap(), peer.local_addr().unwrap());
    let start = Instant::now() + Duration::from_millis(50);
    std::thread::sleep(Duration::from_millis(50));
    let wall = Instant::now();
    std::thread::scope(|sc| {
        let down = {
            let s = Arc::clone(&s);
            sc.spawn(move || {
                let mut payload = [0u8; 40];
                pace(rate, n, start, |i| {
                    payload[..8].copy_from_slice(&i.to_le_bytes());
                    s.try_send(&transmit(p_addr, &payload)).unwrap();
                });
            })
        };
        let up = {
            let peer = Arc::clone(&peer);
            sc.spawn(move || {
                let mut payload = [0u8; 40];
                pace(rate, n, start, |i| {
                    payload[..8].copy_from_slice(&i.to_le_bytes());
                    peer.send_to(&payload, s_addr).unwrap();
                });
            })
        };
        let sink = {
            let peer = Arc::clone(&peer);
            sc.spawn(move || {
                let mut buf = [0u8; 64];
                let mut got = 0u64;
                while peer.recv_from(&mut buf).is_ok() {
                    got += 1;
                }
                got
            })
        };
        let quinn_side = {
            let s = Arc::clone(&s);
            sc.spawn(move || {
                let mut got = 0u64;
                while recv(&s, Duration::from_millis(500)).is_some() {
                    got += 1;
                }
                got
            })
        };
        down.join().unwrap();
        up.join().unwrap();
        let send_secs = wall.elapsed().as_secs_f64();
        let got_down = sink.join().unwrap();
        let got_up = quinn_side.join().unwrap();
        let r = s.run_summary();
        let total = wall.elapsed().as_secs_f64();
        let cpu_ns = value(&r, "l2_release_cpu_ns");
        println!(
            "M13: {n} per direction in {send_secs:.3} s = {:.0} pps each way; received down {got_down}, up {got_up}",
            n as f64 / send_secs
        );
        for dir in ["up", "down"] {
            let v = |k: &str| value(&r, &format!("l2_{dir}_{k}_count"));
            let l = |k: &str| value(&r, &format!("l2_late_{dir}_{k}")) as f64 / 1_000.0;
            println!(
                "  {dir}: delivered {} lost {} queue {} pool {} shaper {} overtaken {} reordered {}; lateness us: \
                 n {} p50 {:.1} p99 {:.1} p99.9 {:.1} max {:.1} E[(L-L0-0.25ms)+] {:.3} over {}",
                v("delivered"),
                v("lost"),
                v("queue_drop"),
                v("pool_drop"),
                v("shaper_drop"),
                v("overtaken"),
                v("reordered"),
                value(&r, &format!("l2_late_{dir}_count")),
                l("p50_ns"),
                l("p99_ns"),
                l("p999_ns"),
                l("max_ns"),
                l("excess_ns"),
                value(&r, &format!("l2_late_{dir}_over_count")),
            );
        }
        println!(
            "  release thread CPU {:.3} s over {total:.3} s = {:.3} core; reorder up {} down {} permille; slack {:?}",
            cpu_ns as f64 / 1e9,
            cpu_ns as f64 / 1e9 / total,
            value(&r, "reorder_up_permille"),
            value(&r, "reorder_down_permille"),
            s.header_numbers()
                .iter()
                .find(|x| x.0 == "l2_timer_slack_ns"),
        );
        // Sustained: the pacers kept 10,000 per second each way, and nothing was dropped but by the profile.
        assert!(n as f64 / send_secs > 9_900.0);
        for dir in ["up", "down"] {
            for k in ["queue_drop", "pool_drop", "oversize", "shaper_drop"] {
                assert_eq!(value(&r, &format!("l2_{dir}_{k}_count")), 0, "{dir} {k}");
            }
            let delivered = value(&r, &format!("l2_{dir}_delivered_count"));
            let lost = value(&r, &format!("l2_{dir}_lost_count"));
            assert_eq!(delivered + lost, n, "{dir}: every datagram accounted for");
            assert!(
                (lost as f64 / n as f64 - 0.02).abs() < 0.004,
                "{dir}: P2's 2 % loss"
            );
        }
        assert_eq!(
            value(&r, "l2_down_send_err_count") + value(&r, "l2_down_would_block_count"),
            0
        );
        assert_eq!(got_up, value(&r, "l2_up_delivered_count"));
        assert!(
            got_down + 50 >= value(&r, "l2_down_delivered_count"),
            "the kernel may drop a few on loopback"
        );
    });
}
