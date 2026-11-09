//! ImpairSocket on loopback, through quinn's socket trait itself (the code quinn calls), without quinn: M1, M5, M10,
//! M11, M12, M13, M14, and U3d's timing self-test. The WebTransport routes are in `wt`'s tests.

use super::super::stats::{ALLOWANCE_NS, FATE_BYTES, Fate};
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

// U3d: the timing self-test (u3d-plan.md §3.1, §3.3 and §4; the advisor's APPROVE WITH CONDITIONS, 2026-11-09). How
// late ImpairSocket hands datagrams over at P1 and 20,000 releases per second, the inbound side polled from a spawned
// tokio task as quinn polls. Reported here, judged on RS at R1 (U8): nothing about lateness is asserted.

/// Up and down as the impairer indexes them: every per-direction value is a `[_; 2]` array, never a HashMap (R-F10).
const DIRS: [(usize, &str); 2] = [(UP, "up"), (DOWN, "down")];

/// U3D-BAR (the advisor's ruling (a), 2026-11-09): E ≤ 10,000 ns per direction, inclusive. It is the derivation's
/// exact value at P1, 0.001 × 2a with a = 5 ms (u3-verify-timing.md:42-48), not a tolerance.
const BAR_NS: u64 = 10_000;

/// What a poller saw: the datagrams `poll_recv` handed it, and when it first returned one.
#[derive(Debug)]
struct Polled {
    got: u64,
    first_at: Option<Instant>,
}

/// A multi-thread runtime with `workers` workers, as npt-server's will be (4 on RS's 4 vCPUs).
fn runtime(workers: usize) -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(workers)
        .enable_all()
        .build()
        .unwrap()
}

/// Starts the inbound poller (U3D-POLL (b); R-F1): a task spawned on `rt`, as quinn spawns its endpoint driver
/// (quinn-0.11.12 endpoint.rs:152-160). Never the test's own thread and never `block_on`: either polls from a parked
/// thread, M13's hop and not quinn's. The poller is defined here and nowhere else (the advisor's condition 1), so the
/// self-test, T1, T2 and T7 all poll through this one helper, and a defect planted here reaches the self-test's own
/// poller. On each wake it calls `poll_recv` with quinn's 32 buffers of 1,472 B until Pending, as quinn's `poll_socket`
/// does (endpoint.rs:772ff, its loop :793). The stop is polled only after Pending, so its lock never comes before a
/// stamp (R-F9); `notify_one` keeps a permit, so no stop is lost.
fn start_poller(
    rt: &tokio::runtime::Handle,
    s: &Arc<ImpairSocket>,
    stop: &Arc<tokio::sync::Notify>,
) -> tokio::task::JoinHandle<Polled> {
    const SIZE: usize = L2Config::DEFAULT_MAX_DATAGRAM_BYTES;
    const BATCH: usize = quinn::udp::BATCH_SIZE;
    let (s, stop) = (Arc::clone(s), Arc::clone(stop));
    let poller = async move {
        // One buffer cut into quinn's 32, made once (quinn's recv_buf: endpoint.rs:754-759, :781-791).
        let mut recv_buf = vec![0u8; SIZE * BATCH];
        let mut metas = [RecvMeta::default(); BATCH];
        let mut stopped = std::pin::pin!(stop.notified());
        let (mut got, mut first_at) = (0u64, None);
        std::future::poll_fn(|cx| {
            let mut chunks = recv_buf.chunks_mut(SIZE).map(IoSliceMut::new);
            let mut iovs: [IoSliceMut<'_>; BATCH] =
                std::array::from_fn(|_| chunks.next().expect("BATCH chunks"));
            loop {
                match s.poll_recv(cx, &mut iovs, &mut metas) {
                    Poll::Ready(Ok(n)) => {
                        if first_at.is_none() {
                            first_at = Some(Instant::now());
                        }
                        got += n as u64;
                    }
                    Poll::Ready(Err(e)) => panic!("poll_recv errored: {e}"),
                    Poll::Pending => break,
                }
            }
            stopped.as_mut().poll(cx)
        })
        .await;
        Polled { got, first_at }
    };
    rt.spawn(poller)
}

/// The lateness samples' set: every finished fate but Oversize, the kinds the histogram records (impair.rs:680-684).
fn in_e_set(kind: Kind) -> bool {
    matches!(kind, Kind::Delivered | Kind::SendError | Kind::WouldBlock)
}

/// One direction's samples from the fate log, each list sorted ascending.
#[derive(Debug, Default)]
struct Samples {
    /// L = out − due of every fate in the histogram's set (`in_e_set`), as the histogram records it.
    late: Vec<u64>,
    /// -3's delay, out − t_in (the profile's delay D plus L), of every Delivered fate only (R-F5).
    delay: Vec<u64>,
    /// Every fate of the direction, one per datagram sent: -3's count (`second.s.probe_*_lost`, gap 4).
    fates: u64,
    /// The profile's loss draws.
    lost: u64,
    /// Delivered fates released before they arrived (out < t_in): never a delay, counted so the self-test can assert
    /// none after its prints (one monotonic clock makes it an invariant).
    early: u64,
}

fn samples(log: &[Fate], dir: usize) -> Samples {
    let mut s = Samples::default();
    for f in log.iter().filter(|f| usize::from(f.dir) == dir) {
        s.fates += 1;
        if f.kind == Kind::Lost {
            s.lost += 1;
        }
        if in_e_set(f.kind) {
            s.late.push(f.out_ns.saturating_sub(f.due_ns));
        }
        if f.kind == Kind::Delivered {
            // Only after the filter: a dropped fate has out 0, and `0 − t_in` would underflow (R-F5).
            match f.out_ns.checked_sub(f.t_in_ns) {
                Some(d) => s.delay.push(d),
                None => s.early += 1,
            }
        }
    }
    s.late.sort_unstable();
    s.delay.sort_unstable();
    s
}

/// npt-judge's nearest rank (eval.rs:261-265; plan §6.1): ⌈q × n⌉ in integers, at least 1 and at most n (1-based).
fn nearest_rank(n: usize, num: u64, den: u64) -> usize {
    let n = n as u128;
    (n * u128::from(num))
        .div_ceil(u128::from(den))
        .clamp(1, n.max(1)) as usize
}

/// The nearest-rank quantile num/den of `sorted` (ascending); 0 when empty.
fn quantile(sorted: &[u64], num: u64, den: u64) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    sorted[nearest_rank(sorted.len(), num, den) - 1]
}

/// E[(L − L0 − 0.25 ms)+] over `late`, in whole ns rounded down, as `summarize` rounds (stats.rs:156); u128 sums.
fn excess(late: &[u64], l0: u64) -> u64 {
    if late.is_empty() {
        return 0;
    }
    let c = l0 + ALLOWANCE_NS;
    let total: u128 = late.iter().map(|&l| u128::from(l.saturating_sub(c))).sum();
    (total / late.len() as u128) as u64
}

/// The fate log's E forms over one direction's sorted samples (U3D-L0): the judged one, with L0 = 0 (the impairer
/// subtracts nothing, so -3 sees L itself: impair.rs:119-123), and M1's, with L0 = the exact nearest-rank p50.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Forms {
    p50: u64,
    at_p50: u64,
    at_zero: u64,
}

fn forms(late: &[u64]) -> Forms {
    debug_assert!(late.is_sorted());
    let p50 = quantile(late, 1, 2);
    Forms {
        p50,
        at_p50: excess(late, p50),
        at_zero: excess(late, 0),
    }
}

/// One E against U3D-BAR's bar, inclusive.
fn would_one(e_ns: u64) -> &'static str {
    if e_ns <= BAR_NS { "PASS" } else { "MISS" }
}

/// The would-words per direction, each direction against the bar on its own (never pooled).
fn would(e: [u64; 2]) -> [&'static str; 2] {
    e.map(would_one)
}

/// The run's loss band (R-F10, gap 6): centre n × ppm / 10⁶ ± 5 standard deviations of Binomial(n, ppm / 10⁶), in u128
/// integers with every division and the square root rounding down. Not delay_tests.rs's exact `band`, which starts from
/// q^n and underflows to (0, n) at this n.
fn loss_band(n: u64, ppm: u64) -> (u64, u64) {
    let (n, p) = (u128::from(n), u128::from(ppm));
    let centre = n * p / 1_000_000;
    let variance = n * p * (1_000_000 - p) / 1_000_000_000_000;
    let half = 5 * variance.isqrt();
    (centre.saturating_sub(half) as u64, (centre + half) as u64)
}

/// PM-0001-3's bands for one profile on L2, derived by the rule each check's name states: the count; the loss share
/// and the mean delay within 5 %; p001, p90 and p99.9 at d − a, d + 0.8a and d + a, each within 0.05a.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bands {
    count_min: u64,
    loss_ppm: (u64, u64),
    mean: (u64, u64),
    p001: (u64, u64),
    p90: (u64, u64),
    p999: (u64, u64),
}

fn bands(p: &Profile) -> Bands {
    let (d, a, ppm) = (p.one_way_ns(), p.half_jitter_ns(), u64::from(p.loss_ppm));
    let tol = a / 20;
    let at = |v: u64| (v - tol, v + tol);
    Bands {
        count_min: 1_000_000,
        loss_ppm: (ppm - ppm / 20, ppm + ppm / 20),
        mean: (d - d / 20, d + d / 20),
        p001: at(d - a),
        p90: at(d + 4 * a / 5),
        p999: at(d + a),
    }
}

/// -3's checks' names, in `Cross::pass`'s order.
const CROSS: [&str; 6] = ["count", "loss", "mean", "p001", "p90", "p99.9"];

/// -3's L2 checks on one direction's samples (U3D-XCHECK): the values and whether each would PASS, by nearest rank
/// and exact integer comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cross {
    /// The delays' count (Delivered).
    n: u64,
    /// Every fate of the direction.
    count: u64,
    lost: u64,
    p001: u64,
    p90: u64,
    p999: u64,
    /// The mean delay rounded down; its check compares the exact sum.
    mean: u64,
    pass: [bool; 6],
}

fn cross_check(s: &Samples, b: &Bands) -> Cross {
    let n = s.delay.len() as u64;
    let (n128, count128) = (u128::from(n), u128::from(s.fates));
    let sum: u128 = s.delay.iter().map(|&x| u128::from(x)).sum();
    let q = |num, den| quantile(&s.delay, num, den);
    let (p001, p90, p999) = (q(1, 1_000), q(9, 10), q(999, 1_000));
    let within = |v: u64, (lo, hi): (u64, u64)| n > 0 && (lo..=hi).contains(&v);
    let lost = u128::from(s.lost) * 1_000_000;
    let (lo, hi) = (u128::from(b.loss_ppm.0), u128::from(b.loss_ppm.1));
    let (mlo, mhi) = (u128::from(b.mean.0), u128::from(b.mean.1));
    Cross {
        n,
        count: s.fates,
        lost: s.lost,
        p001,
        p90,
        p999,
        mean: sum.checked_div(n128).unwrap_or(0) as u64,
        pass: [
            s.fates >= b.count_min,
            count128 > 0 && (lo * count128..=hi * count128).contains(&lost),
            n > 0 && (mlo * n128..=mhi * n128).contains(&sum),
            within(p001, b.p001),
            within(p90, b.p90),
            within(p999, b.p999),
        ],
    }
}

fn pass_fail(ok: bool) -> &'static str {
    if ok { "PASS" } else { "FAIL" }
}

/// Whether an E line and -3's p99.9 line say the same.
fn agree(e_would: &str, p999_pass: bool) -> &'static str {
    if (e_would == "PASS") == p999_pass {
        "agree"
    } else {
        "disagree"
    }
}

/// T2's and T7's load: `n` datagrams each way at `rate` per second from dedicated pacers, a sink on the peer, polled
/// through `start_poller` on a 4-worker runtime; `during` runs on this thread meanwhile, given the pacers' start. These
/// tests are outside the self-test's window rule, so they wait until `run_summary` shows every datagram accounted for
/// in both directions (up to 10 s), assert it and the host drops' absence, and only then stop the poller (gap 5).
/// Returns the run summary and the poller's count.
fn drive(
    s: &Arc<ImpairSocket>,
    rate: u64,
    n: u64,
    during: impl FnOnce(Instant),
) -> (Vec<(&'static str, u64)>, u64) {
    let rt = runtime(4);
    let stop = Arc::new(tokio::sync::Notify::new());
    let poller = start_poller(rt.handle(), s, &stop);
    let peer = Arc::new(std::net::UdpSocket::bind(local()).unwrap());
    peer.set_read_timeout(Some(Duration::from_millis(200)))
        .unwrap();
    let (s_addr, p_addr) = (s.local_addr().unwrap(), peer.local_addr().unwrap());
    let start = Instant::now() + Duration::from_millis(20);
    std::thread::scope(|sc| {
        let down = {
            let s = Arc::clone(s);
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
                while peer.recv_from(&mut buf).is_ok() {}
            })
        };
        during(start);
        down.join().unwrap();
        up.join().unwrap();
        sink.join().unwrap();
    });
    let accounted = |r: &[(&'static str, u64)]| {
        DIRS.iter().all(|(_, dir)| {
            value(r, &format!("l2_{dir}_delivered_count"))
                + value(r, &format!("l2_{dir}_lost_count"))
                == n
        })
    };
    let end = Instant::now() + Duration::from_secs(10);
    let mut r = s.run_summary();
    while !accounted(&r) && Instant::now() < end {
        std::thread::sleep(Duration::from_millis(5));
        r = s.run_summary();
    }
    for (_, dir) in DIRS {
        for k in ["queue_drop", "pool_drop", "oversize", "shaper_drop"] {
            assert_eq!(value(&r, &format!("l2_{dir}_{k}_count")), 0, "{dir} {k}");
        }
    }
    assert!(
        accounted(&r),
        "every datagram accounted for in both directions within 10 s: {r:?}"
    );
    stop.notify_one();
    let polled = rt.block_on(poller).unwrap();
    (s.run_summary(), polled.got)
}

/// The fate log's entries as they stand (the test module is socket's child: `fates` is pub(crate), impair.rs:248).
fn fate_entries(s: &ImpairSocket) -> Vec<Fate> {
    s.shared.lock().im.fates.entries().to_vec()
}

/// T1 (U3d; R-F1, T-1): `start_poller`, the self-test's own poller, polls where a spawned task polls, so a busy worker's
/// hop enters the inbound lateness (RQ2). One worker: after the poller's first Pending, a blocker task takes the
/// worker and holds it until 110 ms after the datagram's drain, and only then is the datagram sent, so the poller cannot
/// run at the due time (drain + 20 ms) whatever order the threads start in: the lateness is at least 90 ms by
/// construction. The bites, planted in `start_poller`: the poller driven on a thread of its own with a parked-thread
/// waker, or under `block_on` (each polls at the due time: lateness near 0).
#[test]
fn the_task_poller_takes_lateness_where_the_task_runs() {
    let s = ImpairSocket::bind(local(), cfg(fixed(20))).unwrap();
    let rt = runtime(1);
    let stop = Arc::new(tokio::sync::Notify::new());
    let poller = start_poller(rt.handle(), &s, &stop);
    // (1) The poller's first Pending stored its waker (condition 6 (c): read here, nothing test-only in the poller).
    let end = Instant::now() + Duration::from_secs(2);
    while s.shared.lock().waker.is_none() {
        assert!(Instant::now() < end, "the poller never polled");
        std::thread::sleep(Duration::from_millis(1));
    }
    // (2) The blocker: says it runs, then holds the only worker until 110 ms after the drain, or gives up after 2 s.
    let peer = std::net::UdpSocket::bind(local()).unwrap();
    let p_addr = peer.local_addr().unwrap();
    let (running, is_running) = std::sync::mpsc::channel();
    let blocker = {
        let s = Arc::clone(&s);
        rt.spawn(async move {
            running.send(()).unwrap();
            let give_up = Instant::now() + Duration::from_secs(2);
            loop {
                match s.first_seen(p_addr) {
                    Some(first) if Instant::now() >= first + Duration::from_millis(110) => {
                        return true;
                    }
                    None if Instant::now() >= give_up => return false,
                    _ => std::thread::sleep(Duration::from_millis(1)),
                }
            }
        })
    };
    is_running
        .recv_timeout(Duration::from_secs(2))
        .expect("the blocker runs on the worker");
    // (3) Only now the datagram, due 20 ms after its drain.
    peer.send_to(b"late", s.local_addr().unwrap()).unwrap();
    assert!(rt.block_on(blocker).unwrap(), "no drain");
    let r = summary_when(&s, "l2_up_delivered_count", 1);
    stop.notify_one();
    let polled = rt.block_on(poller).unwrap();
    assert_eq!(polled.got, 1, "the poller took the one datagram");
    let first = s.first_seen(p_addr).expect("stamped at the drain");
    let at = polled.first_at.expect("stamped by the poller on Ready");
    let late = value(&r, "l2_late_up_max_ns");
    let bound = at.duration_since(first).as_nanos() as u64 - 20 * MS;
    assert!(
        (50 * MS..=bound).contains(&late),
        "inbound lateness {late} ns: at least 50 ms (the worker was held ~90 ms past due), at most {bound} ns"
    );
}

/// T2 (U3d; T-2, T-12a, gap 5): the fate log's samples, which the E forms and -3's cross-check read, are the instrument's
/// own: at P1 (jitter, 1 % loss, the reorder step) through `start_poller`, the E set's count, the Delivered count and
/// the loss count equal the run summary's, and the fate log's E with L0 = the summary's p50 is the summary's E within
/// 1,000 ns (stats.rs:3-8). Identities only, no timing assert. The bites: L taken as out − t_in (E off by ~25 ms);
/// lost fates not filtered out (the count differs).
#[test]
fn the_fate_log_and_the_histogram_agree() {
    let n = 2_000u64;
    let c = L2Config {
        fate_log_capacity: (2 * n + 1_000) as usize,
        ..L2Config::new(5, Profile::p1())
    };
    let s = ImpairSocket::bind(local(), c).unwrap();
    let (r, got) = drive(&s, 2_000, n, |_| {});
    assert_eq!(
        got,
        value(&r, "l2_up_delivered_count"),
        "the poller's count"
    );
    assert_eq!(value(&r, names::FATE_LOG_FULL), 0);
    // Send errors and would-blocks enter the histogram (impair.rs:680-684); only down has them (names.rs:83-85).
    assert_eq!(
        value(&r, "l2_down_send_err_count") + value(&r, "l2_down_would_block_count"),
        0,
        "no send error or would-block"
    );
    let log = fate_entries(&s);
    for (d, dir) in DIRS {
        let v = |k: &str| value(&r, &format!("l2_{dir}_{k}_count"));
        let sm = samples(&log, d);
        assert!(
            v("lost") > 0,
            "{dir}: some loss (fixed by the seed), so a count bite shows"
        );
        assert_eq!(
            sm.late.len() as u64,
            value(&r, &format!("l2_late_{dir}_count")),
            "{dir}: the E set is the histogram's samples"
        );
        assert_eq!(sm.delay.len() as u64, v("delivered"), "{dir}: Delivered");
        assert_eq!(
            (sm.lost, sm.fates),
            (v("lost"), n),
            "{dir}: lost, every fate"
        );
        let p50 = value(&r, &format!("l2_late_{dir}_p50_ns"));
        let (e, hist) = (
            excess(&sm.late, p50),
            value(&r, &format!("l2_late_{dir}_excess_ns")),
        );
        assert!(
            e.abs_diff(hist) <= 1_000,
            "{dir}: the fate log's E {e} ns against the histogram's {hist} ns"
        );
    }
}

/// T3 (U3d; T-12b): the would-line applies U3D-BAR's "≤ 10,000 ns" per direction, never pooled. The bites: `<` for `≤`
/// (10,000 reads MISS); a pooled mean (the mixed case reads PASS twice).
#[test]
fn the_verdict_line_is_per_direction_and_inclusive() {
    assert_eq!(would([10_000, 10_000]), ["PASS", "PASS"]);
    assert_eq!(would([10_001, 10_001]), ["MISS", "MISS"]);
    assert_eq!(would([10_001, 0]), ["MISS", "PASS"]);
    assert_eq!(would([0, 10_001]), ["PASS", "MISS"]);
}

/// T4 (U3d; gap 6): -3's cross-check ranks as npt-judge does, ⌈q × n⌉ in integers (eval.rs:261-265, :1595-1597;
/// marks/README.md:103-104), and the run's loss band is the integer rule of §3.1. The bites: ⌊q × n⌋ or an interpolated
/// quantile; a band of (0, n); the standard deviation rounded up (±500) or taken in f64 (±497).
#[test]
fn nearest_rank_and_loss_band_on_fixed_inputs() {
    assert_eq!(nearest_rank(36_000, 99, 100), 35_640);
    assert_eq!(nearest_rank(1_001, 999, 1_000), 1_000, "not the 999th");
    assert_eq!(nearest_rank(1_000_000, 1, 1_000), 1_000);
    // The k-th of these values is 10 × k: 10 apart, so an interpolated quantile lands between two (p001 at 1,000,000:
    // 10,009.99 by the common (n − 1) × q rule) where the nearest rank takes one.
    let tens = |n: u64| (1..=n).map(|k| 10 * k).collect::<Vec<u64>>();
    assert_eq!(quantile(&tens(36_000), 99, 100), 356_400, "the 35,640th");
    assert_eq!(quantile(&tens(1_001), 999, 1_000), 10_000, "the 1,000th");
    assert_eq!(quantile(&tens(1_000_000), 1, 1_000), 10_000, "the 1,000th");
    assert_eq!(quantile(&tens(1), 1, 1_000), 10, "at least the first");
    assert_eq!(quantile(&[], 1, 2), 0);
    // n = 1,000,000 at 1 %: variance 9,900, isqrt 99, ±495; n = 10,000: variance 99, isqrt 9, ±45.
    assert_eq!(loss_band(1_000_000, 10_000), (9_505, 10_495));
    assert_eq!(loss_band(10_000, 10_000), (55, 145));
}

/// T5 (U3d; T-2, gap 7): the exact E forms on fixed samples: 999 of 100,000 ns and one of 10,350,000 ns, so the exact
/// p50 is 100,000; E with L0 = p50 is 10,000,000 / 1,000 = 10,000 ns (would PASS, inclusive) and with L0 = 0 is
/// 10,100,000 / 1,000 = 10,100 ns (would MISS). The bites: L0 swapped; divided by the over count (10,000,000); L0 = the
/// mean (9,989); the allowance not subtracted above the threshold (10,250).
#[test]
fn the_exact_excess_on_fixed_samples() {
    let mut late = vec![100_000u64; 999];
    late.push(10_350_000);
    let f = forms(&late);
    assert_eq!(
        f,
        Forms {
            p50: 100_000,
            at_p50: 10_000,
            at_zero: 10_100,
        }
    );
    assert_eq!(
        (would_one(f.at_p50), would_one(f.at_zero)),
        ("PASS", "MISS")
    );
}

/// T6 (U3d; T-3, gap 4): -3's P1 cross-check reads delay = out − t_in (D + L) of Delivered fates only, ranks by nearest
/// rank, counts every fate, and applies P1's bands, which equal marks.json's (PM-0001-3's "L2 P1" checks, :752-1030).
/// Up: 1,000 Delivered with D_i = 20,000,000 + 10,000 × i ns, L 0 but for i = 998 and 999 (L 300,000 ns), and 10 Lost;
/// down: 3 Delivered (the direction filter) and one Delivered released before it arrived (counted, never a delay). By
/// hand: p001 20,000,000; p90 28,990,000; p99.9 30,280,000 (FAIL: above 30,250,000); mean 24,995,600; loss 10 / 1,010;
/// count 1,010; E 100 ns in both forms, so E and p99.9 disagree; `agree`'s two "agree" cases. The loss share's
/// denominator is every fate: 106 lost of 10,106 fates with 10,000 Delivered is 1.049 % (PASS), where over the
/// Delivered it would be 1.06 % (FAIL). The bites: delay = due − t_in (p99.9 29,980,000 passes); lost entries kept (n
/// 1,010, or counted as released before they arrived); the count over Delivered (1,000); a band rule off by a term;
/// `agree` saying "disagree" always; the loss share over the Delivered.
#[test]
fn the_cross_check_reads_delay_plus_lateness() {
    let p1 = bands(&Profile::p1());
    // marks.json's numbers for L2 P1, as cited literals (the drift test there pins marks.json itself).
    assert_eq!(
        p1,
        Bands {
            count_min: 1_000_000,
            loss_ppm: (9_500, 10_500),
            mean: (23_750_000, 26_250_000),
            p001: (19_750_000, 20_250_000),
            p90: (28_750_000, 29_250_000),
            p999: (29_750_000, 30_250_000),
        }
    );
    let fate = |dir: usize, kind: Kind, t_in: u64, due: u64, out: u64| Fate {
        ordinal: 0,
        dir: dir as u8,
        kind,
        overtaken: false,
        index: 0,
        t_in_ns: t_in,
        due_ns: due,
        out_ns: out,
        len: 40,
        errno: 0,
    };
    let mut log = Vec::new();
    for i in 0..1_000u64 {
        let t_in = 1_000_000 + 100_000 * i;
        let due = t_in + 20_000_000 + 10_000 * i;
        let late = if i >= 998 { 300_000 } else { 0 };
        log.push(fate(UP, Kind::Delivered, t_in, due, due + late));
        if i % 100 == 0 {
            log.push(fate(UP, Kind::Lost, t_in + 1, 0, 0));
        }
        if i < 3 {
            log.push(fate(
                DOWN,
                Kind::Delivered,
                t_in,
                t_in + 90 * MS,
                t_in + 90 * MS,
            ));
        }
    }
    log.push(fate(DOWN, Kind::Delivered, 5 * MS, 4 * MS, 4 * MS));
    let up = samples(&log, UP);
    assert_eq!(up.early, 0, "up: a fate released before it arrived");
    let x = cross_check(&up, &p1);
    assert_eq!(
        x,
        Cross {
            n: 1_000,
            count: 1_010,
            lost: 10,
            p001: 20_000_000,
            p90: 28_990_000,
            p999: 30_280_000,
            mean: 24_995_600,
            pass: [false, true, true, true, true, false],
        }
    );
    let f = forms(&up.late);
    assert_eq!((f.at_p50, f.at_zero), (100, 100));
    assert_eq!(agree(would_one(f.at_zero), x.pass[5]), "disagree");
    assert_eq!(agree("MISS", false), "agree", "E MISS and p99.9 FAIL");
    assert_eq!(agree("PASS", true), "agree", "E PASS and p99.9 PASS");
    let down_s = samples(&log, DOWN);
    assert_eq!(
        down_s.early, 1,
        "down: the one fate released before it arrived is counted"
    );
    let down = cross_check(&down_s, &p1);
    assert_eq!((down.n, down.count, down.p999), (3, 4, 90 * MS));
    // The loss share over every fate of the direction (as -3 counts `probe_*_lost`), never over the Delivered.
    let mut loss_log: Vec<Fate> = (0..10_000u64)
        .map(|i| fate(UP, Kind::Delivered, i, i + 25 * MS, i + 25 * MS))
        .collect();
    loss_log.extend((0..106u64).map(|i| fate(UP, Kind::Lost, i, 0, 0)));
    let y = cross_check(&samples(&loss_log, UP), &p1);
    assert_eq!(
        (y.n, y.count, y.lost, y.pass[1]),
        (10_000, 10_106, 106, true),
        "106 lost of 10,106 fates is 1.049 %, inside 0.95-1.05 % (over the 10,000 Delivered it would be 1.06 %)"
    );
}

/// T7 (U3d; T-12c, F-O): a stall of the release path enters L and the over count in both directions, through the
/// stamps (up at `poll_recv` after its lock: socket.rs:501-503; down after `try_send`: :724-726), the fate log and
/// the histogram. A fixed 100 ms each way with the default pool (about 200 datagrams in flight per direction), 2,000
/// per second each way for 1 s through `start_poller`; about 400 ms in, this thread holds the state lock for 50 ms
/// between two reads of the socket's clock, t0 and t1. Nothing due in (t0, t1] can leave before t1 (the release
/// thread reads its clock and pops under the lock: :678-681), so with m = p50 + 0.25 ms + 1 µs every datagram due in
/// (t0, t1 − m] is at least m late, and `over` (every bucket above the threshold's: 1 µs buckets below 8.192 ms)
/// counts at least every sample at least m late. The bites: down's release stamp taken as its due (fails naming down);
/// the same for up (naming up); `summarize` skipping the log buckets (`over` falls under that count).
#[test]
fn a_held_lock_makes_both_directions_late() {
    let n = 2_000u64;
    let c = L2Config {
        fate_log_capacity: (2 * n + 1_000) as usize,
        ..L2Config::new(7, fixed(100))
    };
    let s = ImpairSocket::bind(local(), c).unwrap();
    let mut held = (0u64, 0u64);
    let (r, got) = drive(&s, 2_000, n, |start| {
        std::thread::sleep(
            (start + Duration::from_millis(400)).saturating_duration_since(Instant::now()),
        );
        let guard = s.shared.lock();
        let t0 = s.now_ns();
        std::thread::sleep(Duration::from_millis(50));
        let t1 = s.now_ns();
        drop(guard);
        held = (t0, t1);
    });
    assert_eq!(
        got,
        value(&r, "l2_up_delivered_count"),
        "the poller's count"
    );
    assert_eq!(value(&r, names::FATE_LOG_FULL), 0);
    let (t0, t1) = held;
    let log = fate_entries(&s);
    for (d, dir) in DIRS {
        let p50 = value(&r, &format!("l2_late_{dir}_p50_ns"));
        let m = p50 + ALLOWANCE_NS + 1_000;
        assert!(
            m < 8_191_000,
            "{dir}: p50 {p50} ns too high for the bound (p50 + 0.25 ms + 1 µs must stay under 8.191 ms)"
        );
        let set: Vec<&Fate> = log
            .iter()
            .filter(|f| usize::from(f.dir) == d && in_e_set(f.kind))
            .collect();
        let late = |f: &Fate| f.out_ns.saturating_sub(f.due_ns);
        let b: Vec<&Fate> = set
            .iter()
            .copied()
            .filter(|f| f.due_ns > t0 && f.due_ns <= t1.saturating_sub(m))
            .collect();
        assert!(
            b.len() >= 20,
            "{dir}: {} datagrams due in the held window ({t0}-{t1} ns), about 99 expected",
            b.len()
        );
        for f in &b {
            assert!(
                late(f) >= m,
                "{dir}: due at {} ns inside the held lock ({t0}-{t1} ns), released only {} ns late (under {m} ns): \
                 the release stamp does not see the stall",
                f.due_ns,
                late(f)
            );
        }
        let a = set.iter().filter(|f| late(f) >= m).count() as u64;
        // `over` takes every bucket above the threshold's whole, so every sample at least m late is in it.
        let over = value(&r, &format!("l2_late_{dir}_over_count"));
        assert!(
            over >= a,
            "{dir}: over {over}, under the {a} samples at least {m} ns late"
        );
    }
}

/// U3d's self-test (u3d-plan.md §3.1): ImpairSocket at P1 (seed 11; the reorder step on), 10,000 datagrams per second
/// each way (20,000 releases per second), the inbound side polled by `start_poller`'s task on a 4-worker runtime as
/// quinn polls, the fate log on and pre-touched. Reported, never judged here (RS judges at R1: U8): every print comes
/// before any assert, and the asserts are run conditions only, each failure making the run INVALID, never a lateness
/// verdict. From the first send to the poller's stop the only socket calls are `try_send` and `poll_recv` (R-F3); every
/// other read comes after the stop. `NPT_L2_SELFTEST_SECS` sets the length, 1-600 s (default 100: -3's 1,000,000 per
/// direction). Run for the evidence through spikes/netcode-proof/results/u3d-timing/run.sh, or:
///   NPT_L2_SELFTEST_SECS=100 cargo +1.99.1 test --release --locked -p npt-net --lib
///   l2::socket::tests::l2_timing_self_test -- --ignored --nocapture --exact
#[test]
#[ignore = "timing-heavy: 100 s at 20,000 releases per second; run in release for U3d's evidence"]
fn l2_timing_self_test() {
    let secs = match std::env::var("NPT_L2_SELFTEST_SECS") {
        Err(std::env::VarError::NotPresent) => 100u64,
        Err(std::env::VarError::NotUnicode(v)) => panic!(
            "INVALID: NPT_L2_SELFTEST_SECS={v:?} is not UTF-8, so not a whole number of seconds in 1-600; the run is \
             INVALID, not a lateness verdict"
        ),
        Ok(v) => v
            .parse()
            .ok()
            .filter(|s| (1..=600).contains(s))
            .unwrap_or_else(|| {
                panic!(
                    "INVALID: NPT_L2_SELFTEST_SECS={v:?} is not a whole number of seconds in 1-600 (the fate log \
                     reserves about 1.12 MB per second); the run is INVALID, not a lateness verdict"
                )
            }),
    };
    let rate = 10_000u64;
    let n = rate * secs;
    let profile = Profile::p1();
    assert_eq!(profile.reorder, ReorderStep::PART_13);
    let ppm = u64::from(profile.loss_ppm);
    let b = bands(&profile);
    // P1's largest one-way delay, d + a: nothing sent is due later than this after the last send.
    let longest = Duration::from_nanos(profile.one_way_ns() + profile.half_jitter_ns());
    let cap = (2 * n + 1_000) as usize;
    let c = L2Config {
        fate_log_capacity: cap,
        ..L2Config::new(11, profile)
    };
    let s = ImpairSocket::bind(local(), c).unwrap();
    // The pre-touch (R-F2): fill the log to its capacity and empty it, so every page is resident before the run and
    // none is first touched under the lock; `Vec::drain` keeps the capacity and the pages (stats.rs:495-500).
    let touched = {
        let mut st = s.shared.lock();
        let zero = Fate {
            ordinal: 0,
            dir: 0,
            kind: Kind::Delivered,
            overtaken: false,
            index: 0,
            t_in_ns: 0,
            due_ns: 0,
            out_ns: 0,
            len: 0,
            errno: 0,
        };
        let mut k = 0usize;
        while k < cap && st.im.fates.push(zero) {
            k += 1;
        }
        k
    };
    s.drain_fate_log(|_| {});
    let rt = runtime(4);
    let stop = Arc::new(tokio::sync::Notify::new());
    let poller = start_poller(rt.handle(), &s, &stop);
    let load = || std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let load_start = load();
    let peer = Arc::new(std::net::UdpSocket::bind(local()).unwrap());
    peer.set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    let (s_addr, p_addr) = (s.local_addr().unwrap(), peer.local_addr().unwrap());
    let start = Instant::now() + Duration::from_millis(50);
    std::thread::sleep(Duration::from_millis(50));
    let wall = Instant::now();
    let (send_ns, got_down, polled) = std::thread::scope(|sc| {
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
        down.join().unwrap();
        up.join().unwrap();
        let send_ns = wall.elapsed().as_nanos();
        // The stop: the pacers joined, plus d + a, plus the thread poller's 500 ms idle limit (M13's :565); never
        // found by polling `run_summary` (R-F3).
        std::thread::sleep(longest + Duration::from_millis(500));
        stop.notify_one();
        let polled = rt.block_on(poller).unwrap();
        (send_ns, sink.join().unwrap(), polled)
    });
    // Every read after the stop.
    let total = wall.elapsed().as_secs_f64();
    let r = s.run_summary();
    let h = s.header_numbers();
    let texts = s.header_texts();
    let load_end = load();
    let (sm, log_len) = {
        let st = s.shared.lock();
        let e = st.im.fates.entries();
        ([samples(e, UP), samples(e, DOWN)], e.len())
    };
    let hist = |dir: &str, k: &str| value(&r, &format!("l2_late_{dir}_{k}"));
    let f = sm.each_ref().map(|x| forms(&x.late));
    let at_hist = DIRS.map(|(d, dir)| excess(&sm[d].late, hist(dir, "p50_ns")));
    let x = sm.each_ref().map(|dir| cross_check(dir, &b));
    let sched = texts
        .iter()
        .find(|t| t.0 == "l2_sched_policy")
        .map_or("unknown", |t| t.1.as_str());
    let rt_policy = sched.starts_with("SCHED_FIFO") || sched.starts_with("SCHED_RR");
    // The slack condition (plan §3.1, §5.2; thread.rs:8-13). The release thread reads its start value at its birth:
    // 1, and still 1 when the recorded switch (`chrt -f -p` on its tid) comes from outside after that; 0 only when the
    // whole process was born under a real-time policy on a kernel with ed4fb6d7ef68. The value read now is 1 under
    // SCHED_OTHER; under a real-time policy a kernel with that commit reports 0 and one without keeps 1. So under
    // SCHED_FIFO or SCHED_RR both reads may be 0 or 1; under any other policy both must be 1.
    let slack_ok: &[u64] = if rt_policy { &[0, 1] } else { &[1] };
    let slack = [
        value(&h, "l2_timer_slack_start_ns"),
        value(&h, "l2_timer_slack_ns"),
    ];
    let cpu_ns = value(&r, "l2_release_cpu_ns");
    let send_secs = send_ns as f64 / 1e9;

    // ---- Prints, every one before any assert.
    println!(
        "U3d self-test: P1 (seed 11), {n} per direction in {send_secs:.3} s = {:.0} pps each way; received down \
         {got_down} (the sink), up {} (the poller)",
        n as f64 / send_secs,
        polled.got
    );
    println!(
        "  poller: a tokio task spawned on a 4-worker multi-thread runtime, quinn's {} buffers of {} B, polled until \
         Pending",
        quinn::udp::BATCH_SIZE,
        L2Config::DEFAULT_MAX_DATAGRAM_BYTES
    );
    println!(
        "  fate log: capacity {cap} entries ({:.1} MB), pre-touched with {touched} entries before the pacers; {log_len} \
         entries after the stop",
        (cap * FATE_BYTES) as f64 / 1e6
    );
    println!("  loadavg at the start: {}", load_start.trim());
    for (_, dir) in DIRS {
        let v = |k: &str| value(&r, &format!("l2_{dir}_{k}_count"));
        let l = |k: &str| hist(dir, k) as f64 / 1_000.0;
        println!(
            "  {dir}: delivered {} lost {} queue {} pool {} shaper {} overtaken {} reordered {}; lateness us: n {} \
             p50 {:.1} p99 {:.1} p99.9 {:.1} max {:.1} E[(L-L0-0.25ms)+] {:.3} over {}",
            v("delivered"),
            v("lost"),
            v("queue_drop"),
            v("pool_drop"),
            v("shaper_drop"),
            v("overtaken"),
            v("reordered"),
            hist(dir, "count"),
            l("p50_ns"),
            l("p99_ns"),
            l("p999_ns"),
            l("max_ns"),
            l("excess_ns"),
            hist(dir, "over_count"),
        );
    }
    for (d, dir) in DIRS {
        println!(
            "  {dir} fate log: n {} (finished fates but Oversize); E with L0 = the histogram's p50 ({} ns) {} ns (the \
             histogram's own {} ns); E with L0 = 0 {} ns; E with L0 = the exact p50 ({} ns) {} ns",
            sm[d].late.len(),
            hist(dir, "p50_ns"),
            at_hist[d],
            hist(dir, "excess_ns"),
            f[d].at_zero,
            f[d].p50,
            f[d].at_p50,
        );
    }
    let judged = [f[UP].at_zero, f[DOWN].at_zero];
    let lines = [
        ("L0 = 0, the judged form (U3D-L0)", judged),
        ("L0 = the exact p50", [f[UP].at_p50, f[DOWN].at_p50]),
        ("L0 = the histogram's p50", at_hist),
    ];
    for (form, e) in lines {
        let w = would(e);
        for (d, dir) in DIRS {
            println!(
                "  {dir}: E with {form}: {} ns vs bar {BAR_NS} ns: would {} (reported, not judged here)",
                e[d], w[d]
            );
        }
    }
    for (d, dir) in DIRS {
        let x = &x[d];
        let shown = [
            format!(
                "{} (every fate of the direction; at least {})",
                x.count, b.count_min
            ),
            format!(
                "{} lost of {} ({:.4} %; band {}-{} ppm)",
                x.lost,
                x.count,
                x.lost as f64 * 100.0 / x.count.max(1) as f64,
                b.loss_ppm.0,
                b.loss_ppm.1
            ),
            format!(
                "{} ns, rounded down (band {}-{})",
                x.mean, b.mean.0, b.mean.1
            ),
            format!("{} ns (band {}-{})", x.p001, b.p001.0, b.p001.1),
            format!("{} ns (band {}-{})", x.p90, b.p90.0, b.p90.1),
            format!("{} ns (band {}-{})", x.p999, b.p999.0, b.p999.1),
        ];
        println!(
            "  -3 L2 P1 {dir} on the same samples (reported): delays n {} (Delivered only, out − t_in)",
            x.n
        );
        for ((name, text), ok) in CROSS.iter().zip(&shown).zip(x.pass) {
            println!("  -3 L2 P1 {dir} {name}: {text}: would {}", pass_fail(ok));
        }
        for (form, e) in lines {
            let w = would(e)[d];
            println!(
                "  {dir}: E with {form}: would {w}; -3's p99.9: would {}; {}",
                pass_fail(x.pass[5]),
                agree(w, x.pass[5])
            );
        }
    }
    println!("  -3's RTT mean check: not computable here (no echo)");
    if secs < 100 {
        println!(
            "  short run ({secs} s): under -3's 1,000,000 per direction, so the count's line reads FAIL by length (it \
             is report-only at any length)"
        );
    }
    println!(
        "  release thread CPU {:.3} s over {total:.3} s = {:.3} core (M13's 0.27 core: advisor-u3q.md:269; c10's 0.393 \
         and 0.414); reorder up {} down {} permille",
        cpu_ns as f64 / 1e9,
        cpu_ns as f64 / 1e9 / total,
        value(&r, "reorder_up_permille"),
        value(&r, "reorder_down_permille"),
    );
    println!(
        "  slack now {} ns, at start {} ns, set {}, error {:?}; policy {sched}: the run condition accepts {slack_ok:?} ns \
         at both reads ({})",
        slack[1],
        slack[0],
        value(&h, "l2_timer_slack_set"),
        texts
            .iter()
            .find(|t| t.0 == "l2_timer_slack_error")
            .map(|t| t.1.as_str()),
        if rt_policy {
            "a real-time policy: a kernel with ed4fb6d7ef68 reports 0"
        } else {
            "not SCHED_FIFO or SCHED_RR"
        },
    );
    println!("  loadavg at the end: {}", load_end.trim());

    // ---- Run conditions only: a failure makes the run INVALID, never a lateness verdict.
    const INVALID: &str = "the run is INVALID, not a lateness verdict";
    assert!(
        u128::from(n) * 1_000_000_000 > 9_900 * send_ns,
        "the rate fell to {:.0} per second per direction: {INVALID}",
        n as f64 / send_secs
    );
    assert_eq!(
        touched, cap,
        "the fate log pre-touched with {touched} of its {cap} entries: {INVALID}"
    );
    // The log complete before the loss band, which counts the log's own fates.
    assert_eq!(
        value(&r, names::FATE_LOG_FULL),
        0,
        "the fate log filled: {INVALID}"
    );
    for (d, dir) in DIRS {
        let v = |k: &str| value(&r, &format!("l2_{dir}_{k}_count"));
        for k in ["queue_drop", "pool_drop", "oversize", "shaper_drop"] {
            assert_eq!(v(k), 0, "{dir} {k}: {INVALID}");
        }
        assert_eq!(
            v("delivered") + v("lost"),
            n,
            "{dir}: every datagram accounted for: {INVALID}"
        );
        assert_eq!(
            sm[d].fates, n,
            "{dir}: the fate log holds {} fates of the {n} sent: {INVALID}",
            sm[d].fates
        );
        assert_eq!(
            sm[d].early, 0,
            "{dir}: Delivered fates released before they arrived: {INVALID}"
        );
        let (lo, hi) = loss_band(sm[d].fates, ppm);
        assert!(
            (lo..=hi).contains(&v("lost")),
            "{dir}: {} lost, outside P1's band {lo}-{hi}: {INVALID}",
            v("lost")
        );
    }
    assert_eq!(
        value(&r, "l2_down_send_err_count") + value(&r, "l2_down_would_block_count"),
        0,
        "send errors or would-blocks: {INVALID}"
    );
    assert_eq!(
        polled.got,
        value(&r, "l2_up_delivered_count"),
        "the poller's count: {INVALID}"
    );
    assert!(
        slack.iter().all(|v| slack_ok.contains(v)),
        "the slack read back {} ns at start and {} ns now under {sched}, outside {slack_ok:?}: {INVALID}",
        slack[0],
        slack[1]
    );
}
