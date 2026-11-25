//! Block 1's tests (u6c-brief.md section 3): the probe's shipped pieces on loopback and on fixed fate entries.

use super::super::config::L2Config;
use super::super::stats::FATE_TAG;
use super::*;
use npt_l1::{Profile, ReorderStep};

const MS: u64 = 1_000_000;
const SEC: u64 = 1_000_000_000;

fn local() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 0))
}

/// A fixed one-way delay, no jitter, loss or reorder step, with a fate log of `cap` entries.
fn fixed(one_way_ms: u64, cap: usize) -> L2Config {
    let p = Profile {
        name: "T",
        rtt_ns: 2 * one_way_ms * MS,
        jitter_ns: 0,
        loss_ppm: 0,
        bursts_ns: Vec::new(),
        reorder: ReorderStep::OFF,
    };
    L2Config {
        fate_log_capacity: cap,
        pool_buffers: 256,
        ..L2Config::new(7, p)
    }
}

fn value(list: &[(&'static str, u64)], name: &str) -> u64 {
    list.iter()
        .find(|(n, _)| *n == name)
        .unwrap_or_else(|| panic!("no {name}"))
        .1
}

/// The run summary once `name` reaches `n`, or after 3 s.
fn when(s: &ImpairSocket, name: &str, n: u64) -> Vec<(&'static str, u64)> {
    let end = Instant::now() + Duration::from_secs(3);
    loop {
        let r = s.run_summary();
        if value(&r, name) >= n || Instant::now() >= end {
            return r;
        }
        std::thread::sleep(Duration::from_millis(2));
    }
}

fn send(s: &ImpairSocket, to: SocketAddr, n: usize) {
    for i in 0..n {
        let b = [i as u8; 40];
        let t = Transmit {
            destination: to,
            ecn: None,
            contents: &b,
            segment_size: None,
            src_ip: None,
        };
        s.try_send(&t).expect("never fails");
    }
}

/// A fate of direction `dir` and `kind` with its three stamps.
fn fate(dir: u8, kind: u8, index: u64, t_in: u64, due: u64, out: u64) -> FateEntry {
    FateEntry {
        ordinal: 0,
        index,
        t_in_ns: t_in,
        due_ns: due,
        out_ns: out,
        len: 40,
        errno: 0,
        dir,
        kind,
        overtaken: false,
    }
}

/// `FateEntry` from the canonical bytes equals the socket's own fates (index, direction, kind, length, stamps in
/// order); the drain's own copy gives the same entries; another tag, a cut entry or a kind out of range is refused.
#[test]
fn u6c_fate_entries_parse_the_canonical_bytes_and_refuse_a_bad_tag_or_length() {
    let s = ImpairSocket::bind(local(), fixed(1, 64)).unwrap();
    let sink = std::net::UdpSocket::bind(local()).unwrap();
    send(&s, sink.local_addr().unwrap(), 3);
    when(&s, "l2_down_delivered_count", 3);
    let mut bytes = Vec::new();
    s.drain_fate_log(|b| bytes.extend_from_slice(b));
    let e = FateEntry::parse_log(&bytes).expect("parses");
    assert_eq!(e.len(), 3);
    for (i, f) in e.iter().enumerate() {
        assert_eq!(
            (f.index, f.dir, f.kind, f.len),
            (i as u64, 1, FateEntry::DELIVERED, 40)
        );
        assert!(f.t_in_ns <= f.due_ns && f.due_ns <= f.out_ns, "{f:?}");
    }
    send(&s, sink.local_addr().unwrap(), 2);
    when(&s, "l2_down_delivered_count", 5);
    let mut more = Vec::new();
    FateEntry::drain_into(&s, &mut more).expect("drains");
    assert_eq!(more.iter().map(|f| f.index).collect::<Vec<_>>(), [3, 4]);
    let mut bad = bytes.clone();
    bad[0] ^= 1;
    assert!(FateEntry::parse_log(&bad).is_err(), "another tag");
    assert!(
        FateEntry::parse_log(&bytes[..bytes.len() - 1]).is_err(),
        "a cut entry"
    );
    let mut kind = bytes[FATE_TAG.len()..FATE_TAG.len() + 56].to_vec();
    assert!(FateEntry::parse(&kind).is_some());
    kind[49] = 10;
    assert_eq!(FateEntry::parse(&kind), None, "a kind out of range");
    assert_eq!(FateEntry::parse(&kind[..55]), None, "55 B");
}

/// C-U6-2: one socket a leg, so two legs' summaries and fate logs never mix: each socket counts its own datagrams
/// only, and the next leg's socket on the same port starts from nothing (no reset method, no `set_default` carried).
#[test]
fn u6c_two_legs_on_two_sockets_never_mix() {
    let sink = std::net::UdpSocket::bind(local()).unwrap();
    let to = sink.local_addr().unwrap();
    let a = ImpairSocket::bind(local(), fixed(1, 64)).unwrap();
    let b = ImpairSocket::bind(local(), fixed(2, 64)).unwrap();
    send(&a, to, 3);
    send(&b, to, 5);
    assert_eq!(
        value(
            &when(&a, "l2_down_delivered_count", 3),
            "l2_down_delivered_count"
        ),
        3
    );
    assert_eq!(
        value(
            &when(&b, "l2_down_delivered_count", 5),
            "l2_down_delivered_count"
        ),
        5
    );
    let (mut fa, mut fb) = (Vec::new(), Vec::new());
    FateEntry::drain_into(&a, &mut fa).unwrap();
    FateEntry::drain_into(&b, &mut fb).unwrap();
    assert_eq!((fa.len(), fb.len()), (3, 5));
    let port = a.local_addr().unwrap();
    drop(a);
    let end = Instant::now() + Duration::from_secs(2);
    let next = loop {
        match ImpairSocket::bind(port, fixed(1, 64)) {
            Ok(s) => break s,
            Err(_) if Instant::now() < end => std::thread::sleep(Duration::from_millis(5)),
            Err(e) => panic!("the leg's port did not come back within 2 s: {e}"),
        }
    };
    let r = next.run_summary();
    assert_eq!(
        value(&r, "l2_down_delivered_count"),
        0,
        "a new leg starts empty"
    );
    assert_eq!(value(&b.run_summary(), "l2_down_delivered_count"), 5);
}

/// C-U6-3: the pre-touch fills the log to its capacity and empties it (the count touched is the capacity; nothing is
/// counted as a full log; the log takes entries again), and the drain's hold of the lock is measured.
#[test]
fn u6c_the_pretouch_fills_and_empties_the_log_to_its_capacity_and_the_drain_hold_is_measured() {
    let s = ImpairSocket::bind(local(), fixed(1, 1_000)).unwrap();
    assert_eq!(s.pretouch_fate_log(), 1_000);
    let mut out = Vec::with_capacity(1_000);
    let hold = FateEntry::drain_into(&s, &mut out).expect("drains");
    assert!(out.is_empty(), "emptied");
    assert!(hold > 0, "the hold is read");
    assert_eq!(value(&s.run_summary(), "l2_fate_log_full_count"), 0);
    let sink = std::net::UdpSocket::bind(local()).unwrap();
    send(&s, sink.local_addr().unwrap(), 4);
    when(&s, "l2_down_delivered_count", 4);
    FateEntry::drain_into(&s, &mut out).expect("drains");
    assert_eq!(out.len(), 4, "the log takes entries again");
    let none = ImpairSocket::bind(local(), fixed(1, 0)).unwrap();
    assert_eq!(none.pretouch_fate_log(), 0, "no log, nothing touched");
}

/// The up fates of T5's samples (999 with L = 100,000 ns, one with L = 10,350,000 ns) inside a window [10 s, 20 s),
/// around them a warm-up fate 50 ms late, an Oversize fate and a Lost one.
fn t5_window() -> (Vec<FateEntry>, Vec<(u64, u64)>) {
    let mut log = vec![fate(0, 0, 0, 5 * SEC, 5 * SEC, 5 * SEC + 50 * MS)];
    for i in 0..1_000u64 {
        let t = 10 * SEC + i * MS;
        let late = if i == 500 { 10_350_000 } else { 100_000 };
        log.push(fate(0, 0, i + 1, t, t + 20 * MS, t + 20 * MS + late));
    }
    log.push(fate(0, 7, 2_000, 11 * SEC, 11 * SEC, 11 * SEC + 9 * MS));
    log.push(fate(0, 1, 2_001, 12 * SEC, 0, 0));
    // One record per Delivered up fate: the peer's, echoing nothing.
    let echoes = vec![(LegSeries::NO_ECHO, 0); 1_001];
    (log, echoes)
}

/// C-U6-5 on fixed fate entries: a late datagram in the warm-up stays out of E; the window's 1,002 records from 10 s
/// take T5's samples into E with exactly their excess over n in both forms (10,100 ns with L0 = 0, judged; 10,000 ns
/// with L0 = the p50); the Oversize fate stays out of the sample set, the Lost one is in the window's count only.
#[test]
fn u6c_c_u6_5_the_window_takes_exactly_its_fates_into_both_e_forms() {
    let (log, echoes) = t5_window();
    let l = leg_series(&log, &echoes, 0, 10 * SEC, 1_002, MS, &[]);
    let v = |n: &str| value(&l.summary, n);
    assert_eq!(v("probe_up_count"), 1_000, "n: the window's E set only");
    assert_eq!(v("probe_up_excess_ns"), 10_100, "L0 = 0, the judged form");
    assert_eq!(v("probe_up_excess_p50_ns"), 10_000, "L0 = the p50");
    assert_eq!(
        (
            v("probe_up_p50_ns"),
            v("probe_up_max_ns"),
            v("probe_up_over_count")
        ),
        (100_000, 10_350_000, 1)
    );
    assert_eq!(
        v("probe_up_window_count"),
        1_002,
        "every fate of the window"
    );
    assert_eq!(
        (v("probe_down_count"), v("probe_up_window_ns")),
        (0, 2 * SEC + MS),
        "the Lost fate at 12 s is the last record"
    );
    let f = forms(&samples(&log[1..1_001], 0).late);
    assert_eq!(
        (f.at_zero, f.at_p50),
        (10_100, 10_000),
        "the same through forms"
    );
}

/// `leg_series`: one loss flag a fate (0 Delivered, 1 else), B1's burst apart, delays of Delivered fates only, the round
/// trip joined in delivery order with LG's hold taken off, another source's fates left out, and a repeated echo counted
/// with its first sample kept.
#[test]
fn u6c_leg_series_files_loss_delay_burst_and_the_round_trip_with_the_hold_off() {
    let zero = 100 * SEC;
    let t = |i: u64| zero + SEC + i * MS;
    let mut log = Vec::new();
    for i in 0..5u64 {
        let kind = match i {
            2 => 1,
            3 => 2,
            _ => 0,
        };
        let out = if kind == 0 { t(i) + 10 * MS } else { 0 };
        log.push(fate(1, kind, i, t(i), t(i) + 10 * MS, out));
    }
    // Another source's datagram first (record OTHER, ordinal 9), then the peer's echoes of 0, 1, 4 and 0 again.
    let mut other = fate(0, 0, 0, t(0), t(0), t(0) + MS);
    other.ordinal = 9;
    log.push(other);
    let mut echoes = vec![(LegSeries::OTHER, 0)];
    for (k, down) in [0u64, 1, 4, 0].into_iter().enumerate() {
        let back = t(down) + 23 * MS;
        log.push(fate(0, 0, k as u64, back - 5 * MS, back - 5 * MS, back));
        echoes.push((down, 3 * MS));
    }
    let l = leg_series(&log, &echoes, zero, zero + SEC, 4, MS, &[(t(3), MS)]);
    assert_eq!(l.seconds.len(), 1);
    let (sec, rows) = &l.seconds[0];
    assert_eq!(*sec, 1);
    let row = |n: &str| &rows[LegSeries::NAMES.iter().position(|x| *x == n).expect(n)];
    assert_eq!(row("probe_down_lost"), &[0, 0, 1, 0], "outside the burst");
    assert_eq!(row("probe_burst_down_lost"), &[1], "inside it");
    assert_eq!(
        row("probe_down_delay_ns"),
        &[10_000_000; 3],
        "Delivered only"
    );
    assert_eq!(
        row("probe_up_lost"),
        &[0; 4],
        "the peer's four, not the other source's"
    );
    assert_eq!(row("probe_rtt_ns"), &[20_000_000; 3], "out − t_in − hold");
    assert_eq!(value(&l.summary, "probe_echo_repeat_count"), 1);
    assert_eq!(value(&l.summary, "probe_rtt_count"), 3);
}

/// FP-1 (NC-U6C-1; L1's count-based records, lab/src/probe.rs:124-132), P22's test: each direction's records are its
/// first `packets` fates outside the burst from the window's start whatever the clocks, with up spaced 100 µs × (1 + 5 ×
/// 10⁻⁶) (LG's clock 5 ppm slow) and down's last three 60 µs late (a late pacer), with and without a B1 burst; the
/// burst's fates apart; each span the last kept t_in minus the start plus one interval. A span of RS's clock, [start,
/// start + packets × interval), loses up's last record and down's last one here.
#[test]
fn u6c_fp1_each_direction_keeps_exactly_packets_records_whatever_the_clocks() {
    const N: u64 = 120_000;
    const IV: u64 = 100_000;
    let start = 10 * SEC;
    // Three fates of each direction fall in the warm-up (i = 0, 1, 2), so the records are i = 3 to N + 2 and the burst's.
    let base = start - 250_000;
    let up_t = |i: u64| base + i * IV + i / 2;
    for burst in [None, Some((start + SEC, 10 * MS))] {
        let bursts: Vec<(u64, u64)> = burst.into_iter().collect();
        let inb = |t: u64| bursts.iter().any(|&(b, len)| t >= b && t - b < len);
        let nb_up = (0..N + 300).filter(|&i| inb(up_t(i))).count() as u64;
        let nb_down = (0..N + 300).filter(|&i| inb(base + i * IV)).count() as u64;
        assert_eq!(burst.is_some(), nb_up > 0 && nb_down > 0);
        let (last_up, last_down) = (N + 2 + nb_up, N + 2 + nb_down);
        let late = |i: u64| {
            if (last_down - 2..=last_down).contains(&i) {
                60_000
            } else {
                0
            }
        };
        let down_t = |i: u64| base + i * IV + late(i);
        let (mut log, mut echoes) = (Vec::new(), Vec::new());
        for i in 0..N + 300 {
            let (u, d) = (up_t(i), down_t(i));
            log.push(fate(0, 0, i, u, u + 20 * MS, u + 20 * MS));
            echoes.push((LegSeries::NO_ECHO, 0));
            log.push(fate(1, 0, i, d, d + 20 * MS, d + 20 * MS));
        }
        let l = leg_series(&log, &echoes, 0, start, N, IV, &bursts);
        let total = |n: &str| {
            let at = LegSeries::NAMES.iter().position(|x| *x == n).expect(n);
            l.seconds
                .iter()
                .map(|(_, r)| r[at].len() as u64)
                .sum::<u64>()
        };
        assert_eq!(
            (total("probe_up_lost"), total("probe_down_lost")),
            (N, N),
            "{burst:?}: exactly `packets` a direction outside the burst"
        );
        assert_eq!(
            (total("probe_burst_up_lost"), total("probe_burst_down_lost")),
            (nb_up, nb_down),
            "{burst:?}: the burst's fates apart"
        );
        assert_eq!(
            (
                value(&l.summary, "probe_up_window_ns"),
                value(&l.summary, "probe_down_window_ns")
            ),
            (up_t(last_up) - start + IV, down_t(last_down) - start + IV),
            "{burst:?}: the spans"
        );
        let end = up_t(last_up).max(down_t(last_down)) / SEC;
        assert_eq!(l.seconds.first().map(|s| s.0), Some(10));
        assert_eq!(
            l.seconds.last().map(|s| s.0),
            Some(end),
            "to the last record's second"
        );
    }
}

/// `ProbeDatagram` round trips, with and without an echo; another length, magic, leg above 5 or another leg is refused.
#[test]
fn u6c_a_probe_datagram_round_trips_and_another_length_or_leg_is_refused() {
    for d in [
        ProbeDatagram {
            leg: 1,
            index: 7,
            echo: Some(9),
            hold_ns: 11,
        },
        ProbeDatagram {
            leg: 5,
            index: u64::MAX - 3,
            echo: None,
            hold_ns: 0,
        },
    ] {
        let b = d.encode();
        assert_eq!(b.len(), ProbeDatagram::LEN);
        assert_eq!(ProbeDatagram::decode(&b, None), Some(d));
        assert_eq!(ProbeDatagram::decode(&b, Some(d.leg)), Some(d));
        assert_eq!(
            ProbeDatagram::decode(&b, Some(d.leg ^ 2)),
            None,
            "another leg"
        );
        assert_eq!(ProbeDatagram::decode(&b[..39], None), None, "39 B");
        let mut x = b;
        x[0] = b'X';
        assert_eq!(ProbeDatagram::decode(&x, None), None, "magic");
        x = b;
        x[4] = 6;
        assert_eq!(ProbeDatagram::decode(&x, None), None, "leg 6");
    }
}

/// The pacer: a thread of its own that writes its slack to 1 ns and reads it back (0 under a real-time policy: thread.rs
/// :8-11), and calls its closure for 0, 1, … in order, `count` times.
#[test]
fn u6c_the_pacer_reads_back_1_ns_and_calls_in_order() {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let s = Arc::clone(&seen);
    let p = Pacer::start(100_000, 50, move |i| s.lock().unwrap().push(i)).expect("starts");
    let rt = p.policy().starts_with("SCHED_FIFO") || p.policy().starts_with("SCHED_RR");
    assert_eq!(p.slack_ns(), Some(u64::from(!rt)), "policy {}", p.policy());
    let end = Instant::now() + Duration::from_secs(2);
    while !p.finished() && Instant::now() < end {
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(p.stop(), 50);
    assert_eq!(*seen.lock().unwrap(), (0..50).collect::<Vec<u64>>());
}

/// `start_poller` hands its state each datagram's bytes and source in release order and gives the state back.
#[test]
fn u6c_the_poller_hands_each_datagram_to_its_state_in_release_order() {
    let s = ImpairSocket::bind(local(), fixed(1, 0)).unwrap();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let stop = Arc::new(tokio::sync::Notify::new());
    let push = |v: &mut Vec<(u8, SocketAddr)>, b: &[u8], a| v.push((b[0], a));
    let poller = start_poller(rt.handle(), &s, &stop, Vec::new(), push);
    let peer = std::net::UdpSocket::bind(local()).unwrap();
    for i in 0..5u8 {
        peer.send_to(&[i; 8], s.local_addr().unwrap()).unwrap();
    }
    when(&s, "l2_up_delivered_count", 5);
    stop.notify_one();
    let (polled, seen) = rt.block_on(poller).unwrap();
    let from = peer.local_addr().unwrap();
    assert_eq!(polled.got, 5);
    assert_eq!(seen, (0..5u8).map(|i| (i, from)).collect::<Vec<_>>());
}

/// `send_down`: the pacer sends the leg's datagrams through the socket's `try_send`, index i from 0, so each down fate's
/// flow index is the datagram's own index, at or after the leg's zero on the socket's clock.
#[test]
fn u6c_send_down_sends_the_legs_datagrams_from_its_zero() {
    let s = ImpairSocket::bind(local(), fixed(1, 1_000)).unwrap();
    let peer = std::net::UdpSocket::bind(local()).unwrap();
    peer.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
    let (p, zero) = send_down(&s, peer.local_addr().unwrap(), 3, 1_000_000).expect("starts");
    let mut buf = [0u8; 64];
    let mut got = Vec::new();
    while got.len() < 5 {
        let n = peer.recv(&mut buf).expect("a down datagram");
        got.push(ProbeDatagram::decode(&buf[..n], Some(3)).expect("leg 3"));
    }
    let sent = p.stop();
    assert_eq!(
        got.iter().map(|d| d.index).collect::<Vec<_>>(),
        [0, 1, 2, 3, 4]
    );
    assert!(got.iter().all(|d| d.echo.is_none()));
    when(&s, "l2_down_delivered_count", sent);
    let mut f = Vec::new();
    FateEntry::drain_into(&s, &mut f).unwrap();
    assert_eq!(f.len() as u64, sent);
    for (i, e) in f.iter().enumerate() {
        assert_eq!(e.index, i as u64);
        assert!(e.t_in_ns >= zero);
    }
}

/// LG's end (C2, C3): it speaks first, from one socket for the whole probe; each up datagram carries the newest down
/// datagram received and not yet echoed, with its hold, or none: an up datagram sent with no new down carries none
/// (P19's test: echoing the newest whether echoed or not fails it), and of two downs only the newer is echoed. With RS's
/// socket gone the errors are counted and the sends go on.
#[test]
fn u6c_probe_peer_echoes_the_newest_down_once_with_its_hold() {
    let rs = std::net::UdpSocket::bind(local()).unwrap();
    rs.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
    let lg = probe_peer(rs.local_addr().unwrap(), 200, 3 * SEC).expect("starts");
    let mut buf = [0u8; 64];
    let mut up = || {
        let (n, from) = rs.recv_from(&mut buf).expect("an up datagram");
        (
            ProbeDatagram::decode(&buf[..n], None).expect("decodes"),
            from,
        )
    };
    let (first, from) = up();
    assert_eq!(first.echo, None, "LG speaks first: nothing to echo");
    assert_eq!(from.port(), lg.local_addr().port(), "one socket");
    let down = |i: u64| {
        let d = ProbeDatagram {
            leg: 2,
            index: i,
            echo: None,
            hold_ns: 0,
        };
        rs.send_to(&d.encode(), from).unwrap();
    };
    down(7);
    let echo = loop {
        let (d, f) = up();
        assert_eq!(f, from, "one socket");
        if d.echo.is_some() {
            break d;
        }
    };
    assert_eq!((echo.echo, echo.leg), (Some(7), 2));
    assert!(echo.hold_ns < SEC, "its hold: {} ns", echo.hold_ns);
    for _ in 0..10 {
        assert_eq!(up().0.echo, None, "no new down: no echo");
    }
    down(8);
    down(9);
    // FP-4 (c): read until 9's echo or 1 s, so a stalled reader or test thread is no failure.
    let (mut echoes, until) = (Vec::new(), Instant::now() + Duration::from_secs(1));
    while echoes.last() != Some(&9) && Instant::now() < until {
        echoes.extend(up().0.echo);
    }
    // 8 is echoed only if an up datagram left between the two arrivals (microseconds of a 5 ms spacing): each once.
    assert!(
        echoes == [9] || echoes == [8, 9],
        "the newest, each at most once: {echoes:?}"
    );
    for _ in 0..10 {
        assert_eq!(up().0.echo, None, "9 echoed: no echo again");
    }
    drop(rs);
    std::thread::sleep(Duration::from_millis(200));
    assert!(!lg.finished(), "the sends go on");
    let c = lg.stop();
    assert_eq!(value(&c, "probe_peer_received_count"), 3);
    assert_eq!(
        value(&c, "probe_peer_echoed_count"),
        1 + echoes.len() as u64
    );
    assert!(
        value(&c, "probe_peer_send_err_count") + value(&c, "probe_peer_recv_err_count") > 0,
        "the errors counted: {c:?}"
    );
}
