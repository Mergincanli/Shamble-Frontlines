//! Block 3's tests (u6c-brief.md section 3): the leg-edge parsers on fixed texts, the window's count and the leg logs'
//! names, the window loop's calls (C-U6-7), and one probe on loopback, P0 then P1, against npt-net's `probe_peer`.

use super::*;
use npt_log::{Build, Machine, read_log};
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;

const UDP: &str = "   sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops\n\
  611: 00000000:7533 00000000:0000 07 00000000:00000000 00:00000000 00000000   997        0 81234 2 0000000000000000 17\n\
  612: 0100007F:7534 00000000:0000 07 00000000:00000000 00:00000000 00000000   997        0 81235 2 0000000000000000 0\n";

/// P9's test: the socket's own row by its local port gives its drops (17, and 0 for a row that reads 0); a port with no
/// row is a failed read, never 0 drops.
#[test]
fn u6c_the_udp_row_is_found_by_its_local_port_and_a_missing_one_is_a_failed_read() {
    assert_eq!(udp_drops(UDP, 30003), Some(17));
    assert_eq!(udp_drops(UDP, 30004), Some(0));
    assert_eq!(udp_drops(UDP, 30005), None, "no row: a failed read");
    assert_eq!(udp_drops("", 30003), None);
}

/// /proc/net/snmp's Udp lines with the newer columns and without them (an older kernel's six), and snmp6's Udp6 lines.
#[test]
fn u6c_the_snmp_udp_lines_read_with_and_without_the_newer_columns() {
    let new = "Ip: Forwarding DefaultTTL\nIp: 1 64\nUdp: InDatagrams NoPorts InErrors OutDatagrams RcvbufErrors \
               SndbufErrors InCsumErrors IgnoredMulti MemErrors\nUdp: 10 2 3 40 5 6 7 8 9\nUdpLite: InDatagrams\nUdpLite: 0\n";
    let got = snmp_udp(new);
    assert_eq!(got.len(), 9);
    assert_eq!(got[0], ("snmp_udp_in_datagrams_count".to_owned(), 10));
    assert_eq!(got[4], ("snmp_udp_rcvbuf_errors_count".to_owned(), 5));
    assert_eq!(got[6], ("snmp_udp_in_csum_errors_count".to_owned(), 7));
    assert_eq!(got[8], ("snmp_udp_mem_errors_count".to_owned(), 9));
    let old = "Udp: InDatagrams NoPorts InErrors OutDatagrams RcvbufErrors SndbufErrors\nUdp: 1 0 0 2 0 0\n";
    assert_eq!(snmp_udp(old).len(), 6, "the older kernel's six");
    assert!(snmp_udp("Ip: 1\n").is_empty());
    let six = "Ip6InReceives  5\nUdp6InDatagrams  11\nUdp6RcvbufErrors  1\nUdpLite6InDatagrams 0\n";
    assert_eq!(
        snmp6_udp(six),
        [
            ("snmp6_udp_in_datagrams_count".to_owned(), 11),
            ("snmp6_udp_rcvbuf_errors_count".to_owned(), 1)
        ]
    );
}

/// C-U8-2: every `^cpu` line of /proc/stat, the summed line as `cpu_*` and each vCPU's as `cpu<N>_*`, ten fields each.
#[test]
fn u6c_the_cpu_lines_of_proc_stat_read_per_field() {
    let stat = "cpu  10 1 20 300 4 0 5 6 0 0\ncpu0 5 0 10 150 2 0 3 3 0 0\ncpu1 5 1 10 150 2 0 2 3 0 0\nintr 9 9\nctxt 4\n";
    let got = cpu_stat(stat);
    assert_eq!(got.len(), 30);
    assert_eq!(got[0], ("cpu_user_count".to_owned(), 10));
    assert_eq!(got[7], ("cpu_steal_count".to_owned(), 6));
    assert_eq!(got[13], ("cpu0_idle_count".to_owned(), 150));
    assert_eq!(got[29], ("cpu1_guest_nice_count".to_owned(), 0));
}

/// NC-SB-1's names (the leg before the extension) and NC-SB-16's count: `packets` outside B1's burst, as L1 counts.
#[test]
fn u6c_leg_logs_and_the_windows_count() {
    assert_eq!(
        leg_log("/var/r/room-02.jsonl", "P0"),
        "/var/r/room-02-P0.jsonl"
    );
    assert_eq!(leg_log("/a.b/room", "B1"), "/a.b/room-B1");
    assert_eq!(window_total(1_000_000, 100_000, None), 1_000_000);
    assert_eq!(
        window_total(1_000_000, 100_000, Some((50 * SEC, SEC))),
        1_010_000,
        "B1: the burst's 10,000 on top"
    );
    assert_eq!(
        window_total(2_000, 500_000, Some((50 * SEC, SEC))),
        2_000,
        "a window before the burst"
    );
}

/// C-U6-7: inside the leg the loop reaches the socket only through `take` (once a second, and once at the end) and
/// `edge` (once, before the window); `steal` once a second; on a fake clock. `give_up` ends it early.
#[test]
fn u6c_c_u6_7_the_window_loop_calls_take_second_once_a_second_and_nothing_else() {
    let t = Cell::new(1_000u64);
    let calls = RefCell::new(Vec::new());
    let now = || t.get();
    let sleep = |ns: u64| t.set(t.get() + ns);
    let mut take = || {
        calls.borrow_mut().push("take");
        Vec::new()
    };
    let mut steal = || Some(3);
    let mut edge = || calls.borrow_mut().push("edge");
    let never = |_| None;
    let mut out = Vec::new();
    let mut h = Hooks {
        now: &now,
        sleep: &sleep,
        take: &mut take,
        steal: &mut steal,
        edge: &mut edge,
        give_up: &never,
    };
    seconds_loop(
        1_000,
        1_000 + 2 * SEC - 10_000_000,
        1_000 + 4 * SEC + SEC / 2,
        &mut h,
        &mut out,
    )
    .expect("runs");
    // Second 0 closes at 1 s, the edge read at 1.99 s (10 ms before the window at 2 s), then 2, 3, 4 s and the end.
    assert_eq!(
        *calls.borrow(),
        ["take", "edge", "take", "take", "take", "take"]
    );
    assert_eq!(
        out.iter().map(|s| s.sec).collect::<Vec<_>>(),
        [0, 1, 2, 3, 4]
    );
    assert!(out.iter().all(|s| s.steal == Some(3)));
    let at = Cell::new(0u64);
    let stopped = |t: u64| (t >= 1_000 + SEC + SEC / 2).then(|| "stopped".to_owned());
    let mut take = || {
        at.set(at.get() + 1);
        Vec::new()
    };
    let mut h = Hooks {
        now: &now,
        sleep: &sleep,
        take: &mut take,
        steal: &mut steal,
        edge: &mut edge,
        give_up: &stopped,
    };
    t.set(1_000);
    let mut out = Vec::new();
    assert_eq!(
        seconds_loop(1_000, u64::MAX, 1_000 + 10 * SEC, &mut h, &mut out),
        Err("stopped".to_owned())
    );
    assert_eq!(at.get(), 1, "one second closed before the stop");
}

/// F-NO-PEER (C2's fail-closed rule): a leg with no datagram from the peer within `peer_wait_ns` ends the run with
/// Err((1, …)) and writes no leg log.
#[test]
fn u6c_a_leg_with_no_peer_ends_non_zero_and_writes_no_log() {
    let dir = std::env::temp_dir().join(format!("npt-u6c-no-peer-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a folder");
    let port = std::net::UdpSocket::bind("127.0.0.1:0")
        .and_then(|s| s.local_addr())
        .expect("a port")
        .port();
    let log = dir
        .join("room-02.jsonl")
        .to_str()
        .expect("utf-8")
        .to_owned();
    let s = ProbeSettings {
        header: header(),
        bind: SocketAddr::from(([127, 0, 0, 1], port)),
        peer: Ipv4Addr::LOCALHOST,
        legs: vec![0],
        packets: 100,
        warmup_s: 0,
        rate_pps: 1_000,
        seed: 7,
        log: log.clone(),
        peer_wait_ns: 200_000_000,
    };
    let ran = run_probe(&s, &AtomicBool::new(false), &mut || {});
    assert!(
        matches!(&ran, Err((1, why)) if why.contains("no datagram from the peer")),
        "{ran:?}"
    );
    assert!(
        !std::path::Path::new(&leg_log(&log, "P0")).exists(),
        "no leg log"
    );
    std::fs::remove_dir_all(&dir).expect("removed");
}

fn header() -> Header {
    Header {
        schema: npt_log::SCHEMA,
        run: "C-u6c-probe".into(),
        set: "C-u6c".into(),
        role: "measured".into(),
        marks: vec!["PM-0001-3".into()],
        machine: Machine::read_local("C", "C", "container", "here", "", 1),
        build: Build {
            commit: "0123456789abcdef0123456789abcdef01234567".into(),
            binary_sha256: "ab".repeat(32),
            features: Vec::new(),
            profile: "debug".into(),
        },
        bite: None,
        layer: "L2".into(),
        profile: "P0".into(),
        seed: 7,
        tick_hz: 30,
        clients: 1,
        scene: "SS".into(),
        process: "room".into(),
        slot: 0,
        date: "2026-11-24".into(),
        params: BTreeMap::new(),
    }
}

/// One probe on loopback (section 3; C1 (b), C2, NC-SB-1, NC-SB-3): P0 then P1, 1 s windows at 2,000 a second each
/// way, no warm-up, LG's end by `probe_peer`. Another source (127.0.0.2) speaks first: it is dropped and counted and
/// gets no down datagram (P18's test: the pacer never starts before the peer); P1 binds the port P0 used (LG's one
/// target). Each leg's log `room-02-<leg>.jsonl`: run `<run>-<leg>`, profile the leg, process `probe`, layer `L2`;
/// both edges' `leg_edge` (the socket's row found) and `cpu_stat`; the steal each second; P1's header carries
/// P0's measured base round trip (P10's test); at least `packets` records a direction (FP-1's count), P1's whole-life
/// down deliveries at most its own sends (P2's shipped-code test: a socket carried from P0 exceeds them) and C-U6-5's
/// fields in P1's `l2_summary`; every `probe_*` name has a SERIES.md row.
#[test]
fn u6c_one_probe_on_loopback_p0_then_p1_with_the_peer_pinned() {
    let dir = std::env::temp_dir().join(format!("npt-u6c-probe-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a folder");
    let port = std::net::UdpSocket::bind("127.0.0.1:0")
        .and_then(|s| s.local_addr())
        .expect("a port")
        .port();
    let target = SocketAddr::from(([127, 0, 0, 1], port));
    let s = ProbeSettings {
        header: header(),
        bind: target,
        peer: Ipv4Addr::LOCALHOST,
        legs: vec![0, 1],
        packets: 2_000,
        warmup_s: 0,
        rate_pps: 2_000,
        seed: 7,
        log: dir
            .join("room-02.jsonl")
            .to_str()
            .expect("utf-8")
            .to_owned(),
        peer_wait_ns: 5 * SEC,
    };
    let stranger = std::net::UdpSocket::bind("127.0.0.2:0").expect("127.0.0.2");
    stranger
        .set_read_timeout(Some(Duration::from_millis(100)))
        .expect("a timeout");
    let (stop, opened) = (AtomicBool::new(false), AtomicBool::new(false));
    let ran = std::thread::scope(|sc| {
        let h = sc.spawn(|| run_probe(&s, &stop, &mut || opened.store(true, Relaxed)));
        let until = clock::now_ns() + 3 * SEC;
        while !opened.load(Relaxed) && clock::now_ns() < until {
            std::thread::sleep(Duration::from_millis(1));
        }
        // Probe datagrams, so only the source check keeps the stranger out (P18; SEC-U6C-4 pins on a decode).
        let d = ProbeDatagram {
            leg: 0,
            index: 0,
            echo: None,
            hold_ns: 0,
        };
        for _ in 0..5 {
            stranger.send_to(&d.encode(), target).expect("sent");
        }
        std::thread::sleep(Duration::from_millis(50));
        let lg = npt_net::probe_peer(target, 2_000, 10 * SEC).expect("LG's end");
        let ran = h.join().expect("the probe thread");
        let _ = lg.stop();
        ran
    });
    ran.expect("the probe ran");
    let mut buf = [0u8; 64];
    assert!(
        stranger.recv_from(&mut buf).is_err(),
        "no down datagram to another source"
    );
    let read = |leg: &str| {
        let text = std::fs::read_to_string(dir.join(format!("room-02-{leg}.jsonl")))
            .expect("the leg's log");
        read_log(text.as_bytes()).expect("reads back")
    };
    let (p0, p1) = (read("P0"), read("P1"));
    let series = include_str!("../../../marks/SERIES.md");
    for (log, leg) in [(&p0, "P0"), (&p1, "P1")] {
        let h = &log.header;
        assert_eq!(
            (
                h.run.as_str(),
                h.profile.as_str(),
                h.process.as_str(),
                h.layer.as_str()
            ),
            (format!("C-u6c-probe-{leg}").as_str(), leg, "probe", "L2")
        );
        assert_eq!(log.footer.map(|f| f.ended), Some(Ended::Clean));
        for ph in [Phase::Warmup, Phase::After] {
            let e = |ev: &str| {
                log.events
                    .iter()
                    .find(|e| e.ev == ev && e.ph == ph)
                    .expect(ev)
            };
            assert_eq!(
                e("leg_edge").f["udp_row_found"],
                1,
                "{leg} {ph:?}: the socket's row, the socket bound"
            );
            assert!(e("leg_edge").f.contains_key("udp_drops_count"));
            assert!(e("cpu_stat").f.contains_key("cpu_user_count"));
        }
        assert!(
            !log.seconds.is_empty()
                && log
                    .seconds
                    .iter()
                    .all(|r| r.v.contains_key("steal_permille")),
            "{leg}: the steal each second"
        );
        let names = log
            .events
            .iter()
            .flat_map(|e| e.f.keys())
            .chain(h.params.keys());
        for n in names.filter(|n| n.starts_with("probe_")) {
            assert!(
                series.contains(&format!("`{n}`")),
                "no SERIES.md row for {n}"
            );
        }
    }
    let summary = |log: &npt_log::Log, k: &str| {
        let e = log
            .events
            .iter()
            .find(|e| e.ev == "l2_summary")
            .expect("l2_summary");
        e.f[k].as_u64().unwrap_or_else(|| panic!("{k}"))
    };
    assert!(
        summary(&p0, "probe_other_source_count") >= 5,
        "another source dropped and counted"
    );
    let mut rtts: Vec<u64> = p0
        .seconds
        .iter()
        .flat_map(|r| {
            r.s.get("probe_rtt_ns")
                .into_iter()
                .flatten()
                .map(|&v| v as u64)
        })
        .collect();
    rtts.sort_unstable();
    assert!(!rtts.is_empty(), "P0's round trips");
    assert_eq!(
        p1.header.params["base_rtt_ns"],
        quantile(&rtts, 1, 2),
        "P0's measured base round trip"
    );
    for k in ["probe_up_window_count", "probe_down_window_count"] {
        let n = summary(&p1, k);
        assert!(n >= s.packets, "{k} {n}: at least `packets`, L1's count");
    }
    // FP-3 (C-U6-2, P2 in shipped code): P1's socket is its own, so its whole-life deliveries are its own sends at most.
    let (delivered, sent) = (
        summary(&p1, "l2_down_delivered_count"),
        summary(&p1, "probe_down_sent_count"),
    );
    assert!(
        delivered <= sent,
        "P1 delivered {delivered} down, sent {sent}"
    );
    assert!(summary(&p1, "probe_up_count") > 1_500 && summary(&p1, "probe_rtt_count") > 0);
    for k in [
        "probe_up_excess_ns",
        "probe_down_excess_p50_ns",
        "probe_up_p999_ns",
        "probe_drain_hold_ns",
    ] {
        summary(&p1, k);
    }
    std::fs::remove_dir_all(&dir).expect("removed");
}
