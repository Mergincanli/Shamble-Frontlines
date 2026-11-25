//! npt-load's binary against npt-server's binary on loopback (u6c-brief.md section 3, Block 4; C1 (c)): the room's own
//! token file and hash file, as npt-load-ctl copies them to LG. The room runs as a process because npt-bots' tests have
//! no way to make the room's certificate in process (no rcgen or hashing crate here); its binary is the one `cargo test
//! --workspace` builds beside npt-load's. A2's readings (48 clients at P2, WebSocket and WebTransport) are the two
//! ignored tests at the end.

use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use npt_log::{Ended, Log, Phase, read_log};

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("npt-u6c-load-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a folder");
    dir
}

fn free(udp: bool) -> u16 {
    let a = if udp {
        std::net::UdpSocket::bind("127.0.0.1:0").and_then(|s| s.local_addr())
    } else {
        std::net::TcpListener::bind("127.0.0.1:0").and_then(|s| s.local_addr())
    };
    a.expect("a port").port()
}

fn path(dir: &Path, name: &str) -> String {
    dir.join(name).to_str().expect("utf-8").to_owned()
}

/// The room's settings in `dir` (binary.rs's shape): `clients` slots of SS at `profile`, ending `duration_ms` after OPEN.
fn room_settings(
    dir: &Path,
    ports: (u16, u16),
    clients: u32,
    duration_ms: u64,
    extra: &[(&str, serde_json::Value)],
) -> PathBuf {
    let mut v = serde_json::json!({
        "run": "C-u6c-load-1", "set": "C-u6c-load", "role": "measured", "marks": ["PM-0001-1"],
        "date": "2026-11-24", "commit": "0123456789abcdef0123456789abcdef01234567", "layer": "L2",
        "profile": "P2", "seed": 7, "scene": "SS", "clients": clients, "cmd_timing": "B", "tick_hz": 30,
        "machine_role": "C", "machine_name": "C", "machine_plan": "container", "machine_location": "here",
        "machine_address": "", "tick_vcpu": null, "io_vcpus": [], "io_threads": 2, "duration_ms": duration_ms,
        "warmup_ticks": 30, "first_cpu_model": "", "transport": "both",
        "wt_bind": format!("127.0.0.1:{}", ports.0), "ws_bind": format!("127.0.0.1:{}", ports.1),
        "ws_cert_chain": "", "ws_cert_key": "", "ws_split_mark_s": null,
        "log": path(dir, "room.jsonl"), "token_file": path(dir, "tokens"), "hash_path": path(dir, "hash.json")
    });
    for (k, x) in extra {
        v[*k] = x.clone();
    }
    let p = dir.join("room.json");
    std::fs::write(&p, serde_json::to_vec(&v).expect("json")).expect("written");
    p
}

/// npt-load's settings in `dir` (U8's load_settings shape), the room's hash as the pin.
fn load_settings(
    dir: &Path,
    mode: &str,
    transport: &str,
    clients: u32,
    duration_s: u64,
    ports: (u16, u16),
    pin: &str,
) -> PathBuf {
    let v = serde_json::json!({
        "room": "02", "server": "127.0.0.1", "wt_port": ports.0, "ws_port": ports.1, "mode": mode,
        "transport": transport, "clients": clients, "duration_s": duration_s, "token_path": path(dir, "tokens"),
        "log_path": path(dir, "load.jsonl"), "run": "C-u6c-load-1", "set": "C-u6c-load", "marks": ["PM-0001-1"],
        "date": "2026-11-24", "commit": "0123456789abcdef0123456789abcdef01234567", "role": "measured",
        "layer": "L2", "profile": "P2", "scene": "SS", "tick_hz": 30,
        "machine": {"role": "LG", "name": "LG1", "plan": "container", "location": "here", "address": "127.0.0.1"},
        "behaviour": "brawl", "seed": 7, "pin_hex": pin
    });
    let p = dir.join("load-settings.json");
    std::fs::write(&p, serde_json::to_vec(&v).expect("json")).expect("written");
    p
}

/// The room started from its binary, its `OPEN` read within 5 s. The binary beside npt-load's is this tree's when
/// `cargo test --workspace` (the gate) built it; a `-p npt-bots` run on the shared target may run another worktree's
/// npt-server unless `-p npt-server` was built first (F-STALE-BIN).
fn room(settings: &Path) -> Child {
    let bin = Path::new(env!("CARGO_BIN_EXE_npt-load")).with_file_name("npt-server");
    assert!(
        bin.exists(),
        "{bin:?}: npt-server's binary, built by `cargo test --workspace`"
    );
    let mut child = Command::new(bin)
        .args(["--settings", settings.to_str().expect("utf-8")])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the room starts");
    let out = child.stdout.take().expect("piped");
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut first = String::new();
        let _ = BufReader::new(out).read_line(&mut first);
        let _ = tx.send(first);
    });
    match rx.recv_timeout(Duration::from_secs(5)) {
        Ok(l) if l == "OPEN\n" => child,
        r => {
            let _ = child.kill();
            panic!("the room did not open: {r:?}")
        }
    }
}

fn hash_hex(dir: &Path) -> String {
    let v: serde_json::Value =
        serde_json::from_slice(&std::fs::read(dir.join("hash.json")).expect("the hash file"))
            .expect("json");
    v["value_hex"].as_str().expect("value_hex").to_owned()
}

fn log(file: &Path) -> (Log, String) {
    let text = std::fs::read_to_string(file).expect("a log");
    (read_log(text.as_bytes()).expect("reads back"), text)
}

/// Section 3 and C1 (c), on one transport: npt-load hosts 4 honest brawlers against the room at P2: all join (READY,
/// which alone shows F-JOIN-ONCE (4): a session closed at the room's Join deadline never gets JoinAccepted, and the
/// room's decoder counts are in no log), each gives commands in every whole measured second (from the second one, since
/// READY comes mid-second, to the last but one), `inputs_count` lists the four in slot order; the room ends first and
/// npt-load ends with no rejoin, at the room's close (WebSocket: the close frame; its last second before its duration's)
/// or at its own duration (WebTransport: its last second the duration's, since the room's close reaches the client only
/// at quinn's 30 s idle timeout, F-WT-CLOSE), its footer clean, `DONE`, exit 0; no token in its log, stdout or stderr.
fn hosts_four(transport: &str, load_s: u64) {
    let dir = folder(transport);
    let ports = (free(true), free(false));
    let mut rs = room(&room_settings(&dir, ports, 4, 3_000, &[]));
    let ls = load_settings(&dir, "run", transport, 4, load_s, ports, &hash_hex(&dir));
    let out = Command::new(env!("CARGO_BIN_EXE_npt-load"))
        .arg(&ls)
        .output()
        .expect("npt-load runs");
    let (stdout, stderr) = (
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        (out.status.code(), stdout.as_ref()),
        (Some(0), "READY\nDONE\n"),
        "stderr: {stderr}"
    );
    drop(rs.stdin.take());
    assert_eq!(rs.wait().expect("the room ends").code(), Some(0));
    let (load, text) = log(&dir.join("load.jsonl"));
    assert_eq!(
        (
            load.header.process.as_str(),
            load.header.machine.role.as_str()
        ),
        ("load", "LG")
    );
    assert_eq!(load.footer.map(|f| f.ended), Some(Ended::Clean));
    let measured: Vec<&Vec<i64>> = load
        .seconds
        .iter()
        .filter(|r| r.ph == Phase::Measured)
        .filter_map(|r| r.s.get("inputs_count"))
        .collect();
    assert!(measured.len() >= 2, "{} measured seconds", measured.len());
    assert!(
        measured.iter().all(|v| v.len() == 4),
        "four playing clients a second"
    );
    // F-NAMES, TE-U6C-R2-4: every measured second but the first (READY's) and the last (WebSocket: the close's part
    // second; WebTransport: it starts after the room's close, so only the clients' lead gives commands in it).
    let whole = &measured[1..measured.len() - 1];
    assert!(!whole.is_empty(), "{} measured seconds", measured.len());
    for i in 0..4 {
        assert!(
            whole.iter().all(|v| v[i] > 0),
            "client {i} gave commands every whole measured second: {whole:?}"
        );
    }
    let last = load.seconds.last().map(|r| r.sec);
    if transport == "wt" {
        assert_eq!(last, Some(load_s - 1), "ended at its duration");
    } else {
        assert!(
            last.is_some_and(|k| k + 1 < load_s),
            "ended at the room's close, before its duration: {last:?}"
        );
    }
    assert!(
        load.seconds
            .iter()
            .any(|r| r.v.contains_key("cpu_permille"))
    );
    let tokens = std::fs::read_to_string(dir.join("tokens")).expect("tokens");
    for t in tokens
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|x| x.1.trim_start_matches("nptj_"))
    {
        for (what, s) in [
            ("log", text.as_str()),
            ("stdout", &stdout),
            ("stderr", &stderr),
        ] {
            assert!(!s.contains(t), "a token in the {what}");
        }
    }
    let (room_log, _) = log(&dir.join("room.jsonl"));
    let ev = if transport == "wt" {
        "wt_accept"
    } else {
        "ws_accept"
    };
    // Beside READY: `deadline_closed_count` is the transport's handshake deadline (net session.rs:167-168), not the
    // room's Join deadline (server pump.rs:383): every handshake ended in time (F-JOIN-4).
    let acc = room_log.events.iter().find(|e| e.ev == ev).expect(ev);
    assert_eq!(
        (
            acc.f["accepted_count"].clone(),
            acc.f["deadline_closed_count"].clone()
        ),
        (4.into(), 0.into())
    );
    std::fs::remove_dir_all(&dir).expect("removed");
}

#[test]
fn u6c_npt_load_hosts_four_brawlers_over_webtransport_and_ends_at_its_duration() {
    hosts_four("wt", 4);
}

#[test]
fn u6c_npt_load_hosts_four_brawlers_over_websocket_and_ends_at_the_rooms_close() {
    hosts_four("ws", 30);
}

/// C1 (c)'s probe mode: no token read (the token file is absent), `probe_peer` to the room's UDP port for the duration,
/// one `SUMMARY` line, `DONE`, exit 0, the `probe_peer` event in its log, each of its names with a SERIES.md row; bad
/// settings and an existing log exit 2.
#[test]
fn u6c_npt_load_probe_mode_sends_for_its_duration_and_bad_settings_exit_2() {
    let dir = folder("probe");
    let rs = std::net::UdpSocket::bind("127.0.0.1:0").expect("RS's stand-in");
    rs.set_read_timeout(Some(Duration::from_secs(2)))
        .expect("a timeout");
    let ports = (rs.local_addr().expect("bound").port(), 1);
    let ls = load_settings(&dir, "probe", "wt", 1, 1, ports, &"0".repeat(64));
    let out = Command::new(env!("CARGO_BIN_EXE_npt-load"))
        .arg(&ls)
        .output()
        .expect("npt-load runs");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(
        lines.len() == 2
            && lines[0].starts_with("SUMMARY probe_peer_sent_count=")
            && lines[1] == "DONE",
        "{stdout}"
    );
    let mut buf = [0u8; 64];
    assert_eq!(rs.recv(&mut buf).expect("an up datagram"), 40);
    let (load, _) = log(&dir.join("load.jsonl"));
    let ev = load
        .events
        .iter()
        .find(|e| e.ev == "probe_peer")
        .expect("probe_peer");
    assert!(
        ev.f["probe_peer_sent_count"]
            .as_u64()
            .is_some_and(|n| n >= 5_000),
        "{ev:?}"
    );
    // NC-U6C-8: every name LG's probe log writes has a SERIES.md row.
    let series = include_str!("../../../marks/SERIES.md");
    for n in ev.f.keys() {
        assert!(series.contains(&format!("`{n}`")), "no row for {n}");
    }
    for n in load.header.params.keys() {
        let row = format!("`header.params.{n}`");
        assert!(series.contains(&row), "no row for {row}");
    }
    let again = Command::new(env!("CARGO_BIN_EXE_npt-load"))
        .arg(&ls)
        .output()
        .expect("runs");
    assert_eq!(again.status.code(), Some(2), "an existing log");
    std::fs::write(&ls, b"{}").expect("written");
    let bad = Command::new(env!("CARGO_BIN_EXE_npt-load"))
        .arg(&ls)
        .output()
        .expect("runs");
    assert_eq!(bad.status.code(), Some(2), "bad settings");
    let none = Command::new(env!("CARGO_BIN_EXE_npt-load"))
        .output()
        .expect("runs");
    assert_eq!(none.status.code(), Some(2), "no argument");
    std::fs::remove_dir_all(&dir).expect("removed");
}

/// A2 (u6b-report.md:97-98, :156), taken at U6c's close by the main session, never judged here, on each transport
/// (FP-5): 48 clients at P2 on loopback against one room with the split's mark 10 s after OPEN, for NPT_A2_SECS (default
/// 60) after it; prints the room's `ws_split` (WebSocket only: pooled lateness both ways, the mark on the room's clock:
/// F-3), its largest `rss_bytes` (the cap's memory at N = 48; NR-U6B-5's recount of the pool's heap reads it), and LG's
/// inputs a client a second. Run each by its full name, after `cargo build --locked --release -p npt-server`:
///   cargo test --locked --release -p npt-bots --test load a2_48_websocket_clients_at_p2_against_one_room -- --ignored --nocapture --exact
///   cargo test --locked --release -p npt-bots --test load a2_48_webtransport_clients_at_p2_against_one_room -- --ignored --nocapture --exact
#[test]
#[ignore = "A2's container reading: 48 WebSocket clients for 70 s and more; run by name at U6c's close"]
fn a2_48_websocket_clients_at_p2_against_one_room() {
    a2("ws");
}

/// A2 over WebTransport (FP-5): the same reading with `wt` (the run's headers and calibration run 48 WT clients).
#[test]
#[ignore = "A2's container reading: 48 WebTransport clients for 70 s and more; run by name at U6c's close"]
fn a2_48_webtransport_clients_at_p2_against_one_room() {
    a2("wt");
}

/// A2's body on `transport`.
fn a2(transport: &str) {
    let secs: u64 = std::env::var("NPT_A2_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(60);
    let dir = folder(&format!("a2-{transport}"));
    let ports = (free(true), free(false));
    let extra = [
        ("ws_split_mark_s", 10.into()),
        ("transport", transport.into()),
        ("warmup_ticks", 300.into()),
    ];
    let mut rs = room(&room_settings(&dir, ports, 48, (secs + 10) * 1_000, &extra));
    let ls = load_settings(
        &dir,
        "run",
        transport,
        48,
        secs + 40,
        ports,
        &hash_hex(&dir),
    );
    let out = Command::new(env!("CARGO_BIN_EXE_npt-load"))
        .arg(&ls)
        .output()
        .expect("npt-load runs");
    drop(rs.stdin.take());
    let code = rs.wait().expect("the room ends").code();
    let mut err = String::new();
    let _ = rs.stderr.take().expect("piped").read_to_string(&mut err);
    let (room_log, _) = log(&dir.join("room.jsonl"));
    let split = room_log.events.iter().find(|e| e.ev == "ws_split");
    let rss = room_log
        .seconds
        .iter()
        .filter_map(|r| r.v.get("rss_bytes"))
        .max();
    let (load, _) = log(&dir.join("load.jsonl"));
    let inputs = load
        .seconds
        .iter()
        .filter(|r| r.ph == Phase::Measured)
        .filter_map(|r| r.s.get("inputs_count"))
        .flatten();
    println!(
        "A2 {transport}: npt-load exit {:?}, stdout {:?}; room exit {code:?}, stderr {err:?}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout)
    );
    println!("A2 {transport}: room ws_split {split:?}");
    println!(
        "A2 {transport}: room rss_bytes max {rss:?}; LG inputs a client a second min {:?} max {:?}",
        inputs.clone().min(),
        inputs.max()
    );
}
