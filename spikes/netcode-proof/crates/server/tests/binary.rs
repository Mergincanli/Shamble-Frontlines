//! npt-server's binary from `CARGO_BIN_EXE_npt-server` (u6a-brief.md Block 3, section 3): the start to `OPEN` within the
//! 5 s real-clock cap (the time printed: a container reading, no 2 s bar here), exit 0 when stdin closes, no token in
//! the log, stdout or stderr, and the exit codes; in bites builds the two process-level bites, each failing its mark's
//! check by name on the log it writes. U6b: both listeners accept at `OPEN`, and the hash file pins the certificate.

use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use npt_log::{Ended, Log, Machine, read_log};

/// A folder in the system's temporary folder, new for this test.
fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("npt-u6a-bin-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the test's folder");
    dir
}

/// The settings file in `dir`: two slots of SS, the log and token file beside it, `extra` fields added.
fn settings(dir: &Path, extra: &[(&str, serde_json::Value)]) -> PathBuf {
    let mut v = serde_json::json!({
        "run": "C-u6a-bin-1", "set": "C-u6a-bin", "role": "measured", "marks": ["PM-0001-16", "PM-0006-11"],
        "date": "2026-11-22", "commit": "0123456789abcdef0123456789abcdef01234567",
        "layer": "L2", "profile": "P0", "seed": 7, "scene": "SS", "clients": 2, "cmd_timing": "B",
        "tick_hz": 30, "machine_role": "C", "machine_name": "C", "machine_plan": "container",
        "machine_location": "here", "machine_address": "", "tick_vcpu": null, "io_vcpus": [],
        "io_threads": 1, "duration_ms": 0, "warmup_ticks": 0, "first_cpu_model": "",
        "transport": "both", "wt_bind": "127.0.0.1:0", "ws_bind": "127.0.0.1:0", "ws_cert_chain": "",
        "ws_cert_key": "", "ws_split_mark_s": null
    });
    v["log"] = dir.join("room.jsonl").to_str().expect("utf-8").into();
    v["token_file"] = dir.join("tokens").to_str().expect("utf-8").into();
    v["hash_path"] = dir.join("hash.json").to_str().expect("utf-8").into();
    for (k, x) in extra {
        v[*k] = x.clone();
    }
    let path = dir.join("settings.json");
    std::fs::write(&path, serde_json::to_vec(&v).expect("serializes")).expect("written");
    path
}

fn spawn(settings: &Path) -> Child {
    Command::new(env!("CARGO_BIN_EXE_npt-server"))
        .args(["--settings", settings.to_str().expect("utf-8")])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary starts")
}

/// The first line's real-clock cap for a start's own work (u6a-brief.md section 3's 5 s cap; TR-U6A-7).
const OPEN_CAP: Duration = Duration::from_secs(5);

/// One start: spawn, the time to its first stdout line (the test's stand-in for npt-super's timer, which npt-server's
/// tests cannot run: npt-super's own tests run it against fake children), a few ticks, stdin closed, the exit.
/// Returns (the first line, spawn to that line in ns, the exit code, the rest of stdout, stderr).
#[allow(
    clippy::disallowed_types,
    clippy::disallowed_methods,
    reason = "the test times spawn to OPEN on the real clock (u6a-brief.md section 3); the room reads its own clock in \
              one item"
)]
fn start(settings: &Path, cap: Duration) -> (String, u64, Option<i32>, String, String) {
    let t0 = std::time::Instant::now();
    let mut child = spawn(settings);
    let mut out = BufReader::new(child.stdout.take().expect("piped"));
    // The first line on its own thread, awaited at most `cap` (TR-U6A-7): a room that never opens is killed and the
    // test fails at the cap, never hangs.
    let (tx, rx) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        let mut first = String::new();
        let read = out.read_line(&mut first);
        let open_ns = u64::try_from(t0.elapsed().as_nanos()).unwrap_or(u64::MAX);
        let _ = tx.send((read.map(|_| first), open_ns));
        out
    });
    let (first, open_ns) = match rx.recv_timeout(cap) {
        Ok((first, open_ns)) => (first.expect("stdout reads"), open_ns),
        Err(e) => {
            let _ = child.kill();
            let _ = child.wait();
            panic!("no first line within the {cap:?} cap ({e:?}): the room was killed");
        }
    };
    let mut out = reader.join().expect("the reader");
    std::thread::sleep(Duration::from_millis(300));
    drop(child.stdin.take());
    let status = child.wait().expect("the binary ends");
    let mut rest = String::new();
    out.read_to_string(&mut rest).expect("stdout reads");
    let mut err = String::new();
    child
        .stderr
        .take()
        .expect("piped")
        .read_to_string(&mut err)
        .expect("stderr reads");
    (first, open_ns, status.code(), rest, err)
}

fn log_of(dir: &Path) -> (Log, String) {
    let text = std::fs::read_to_string(dir.join("room.jsonl")).expect("the room's log");
    (read_log(text.as_bytes()).expect("the log reads back"), text)
}

/// The judge's `machine_named` (judge/src/figures.rs:742-764), as this test reads a header: every text field filled,
/// every count above 0, an SMT map with every list filled, and a VM's day address.
fn machine_named(m: &Machine) -> bool {
    let texts = [
        &m.role,
        &m.name,
        &m.plan,
        &m.arch,
        &m.cpu,
        &m.location,
        &m.kernel,
    ];
    texts.iter().all(|t| !t.is_empty())
        && m.vcpus > 0
        && m.mem_mb > 0
        && m.threads_per_core > 0
        && m.clock_res_ns > 0
        && !m.smt.is_empty()
        && m.smt.iter().all(|s| !s.is_empty())
        && (!matches!(m.role.as_str(), "RS" | "LG" | "MV") || !m.address.is_empty())
}

#[test]
fn s8a_the_binary_opens_within_5_s_exits_0_when_stdin_closes_and_no_token_leaves_the_token_file() {
    let dir = folder("open");
    let (first, open_ns, code, rest, err) = start(&settings(&dir, &[]), OPEN_CAP);
    println!(
        "spawn to OPEN: {:.1} ms (container reading, not judged; the 5 s cap only)",
        open_ns as f64 / 1e6
    );
    assert_eq!(first, "OPEN\n", "stderr: {err}");
    assert!(
        open_ns < 5_000_000_000,
        "OPEN within the 5 s real-clock cap"
    );
    assert_eq!(code, Some(0), "stderr: {err}");
    assert_eq!(rest, "", "one line on stdout");
    let (log, text) = log_of(&dir);
    assert_eq!(
        (
            log.header.process.as_str(),
            log.header.clients,
            log.header.scene.as_str()
        ),
        ("room", 2, "SS")
    );
    assert!(
        machine_named(&log.header.machine),
        "the container's header names its machine"
    );
    assert!(!log.ticks.is_empty());
    assert_eq!(log.footer.map(|f| f.ended), Some(Ended::Clean));
    assert!(
        log.events
            .iter()
            .any(|e| e.ev == "cpu_read" && e.f["changed"] == 0),
        "the first start's cpu_read"
    );
    let tokens = std::fs::read_to_string(dir.join("tokens")).expect("the token file");
    let tokens: Vec<&str> = tokens
        .lines()
        .filter_map(|l| l.split_once(' ').map(|x| x.1))
        .collect();
    assert_eq!(tokens.len(), 2);
    for t in &tokens {
        let raw = t.trim_start_matches("nptj_");
        for (what, s) in [("log", &text), ("stdout", &rest), ("stderr", &err)] {
            assert!(!s.contains(raw), "a token in the {what}");
        }
    }
    std::fs::remove_dir_all(&dir).expect("removed");
}

#[test]
fn s8a_the_binary_exits_2_for_bad_settings_or_an_existing_token_file_or_log() {
    let usage = Command::new(env!("CARGO_BIN_EXE_npt-server"))
        .output()
        .expect("runs");
    assert_eq!(usage.status.code(), Some(2), "no settings");
    let dir = folder("codes");
    let unknown = settings(&dir, &[("extra", 1.into())]);
    let out = Command::new(env!("CARGO_BIN_EXE_npt-server"))
        .args(["--settings", unknown.to_str().expect("utf-8")])
        .output()
        .expect("runs");
    assert_eq!(out.status.code(), Some(2), "an unknown field");
    assert!(
        !dir.join("tokens").exists(),
        "nothing minted for bad settings"
    );
    let good = settings(&dir, &[]);
    for existing in ["tokens", "room.jsonl"] {
        std::fs::write(dir.join(existing), b"there").expect("written");
        let out = Command::new(env!("CARGO_BIN_EXE_npt-server"))
            .args(["--settings", good.to_str().expect("utf-8")])
            .stdin(Stdio::null())
            .output()
            .expect("runs");
        assert_eq!(out.status.code(), Some(2), "an existing {existing}");
        assert_eq!(
            std::fs::read(dir.join(existing)).expect("read"),
            b"there",
            "left as it was"
        );
        let _ = std::fs::remove_file(dir.join("tokens"));
        let _ = std::fs::remove_file(dir.join("room.jsonl"));
        let _ = std::fs::remove_file(dir.join("hash.json"));
    }
    std::fs::remove_dir_all(&dir).expect("removed");
}

/// A port free on 127.0.0.1 for `udp` or TCP now (bound and released: the room binds it next).
fn free_port(udp: bool) -> u16 {
    if udp {
        std::net::UdpSocket::bind("127.0.0.1:0")
            .and_then(|s| s.local_addr())
            .expect("a UDP port")
            .port()
    } else {
        std::net::TcpListener::bind("127.0.0.1:0")
            .and_then(|s| s.local_addr())
            .expect("a TCP port")
            .port()
    }
}

/// The hash file's 32 bytes.
fn hash_of(path: &Path) -> [u8; 32] {
    let v: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).expect("the hash file")).expect("JSON");
    let hex = v["value_hex"].as_str().expect("value_hex");
    let mut h = [0u8; 32];
    for (i, b) in h.iter_mut().enumerate() {
        *b = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).expect("hex");
    }
    h
}

/// The room's clock stand-in for the clients' stamps (no assertion reads it).
#[allow(
    clippy::disallowed_types,
    clippy::disallowed_methods,
    reason = "the test's clients stamp on the real clock; no time is asserted"
)]
fn test_clock() -> u64 {
    static T0: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    T0.get_or_init(std::time::Instant::now).elapsed().as_nanos() as u64
}

/// Block 6 (P14's and P15's tests; advisor-u6a-brief.md condition 3): `OPEN` comes once both listeners accept: right
/// after it, with no retry, a WebTransport client pinned by the hash file's value and a WebSocket client both open (the
/// hash file names the served certificate; another hash is refused). The hash file's folder holds it alone (no key on
/// disk); the header carries U6b's params; the run's end writes one event per transport, and (NR-U6B-1) a WebSocket
/// client still connected when stdin closes gets its one `ws_summary`, with `ws_split` after it; and a hash path that
/// exists is refused (exit 2).
#[test]
fn u6b_open_comes_once_both_listeners_accept_and_the_hash_file_pins_the_served_certificate() {
    use npt_link::{Link, LinkState};
    let dir = folder("u6b-open");
    let hash_dir = dir.join("hash");
    std::fs::create_dir_all(&hash_dir).expect("a folder");
    let (wt_port, ws_port) = (free_port(true), free_port(false));
    let hash_path = hash_dir.join("room.json");
    let s = settings(
        &dir,
        &[
            ("wt_bind", format!("127.0.0.1:{wt_port}").into()),
            ("ws_bind", format!("127.0.0.1:{ws_port}").into()),
            ("hash_path", hash_path.to_str().expect("utf-8").into()),
            ("profile", "P1".into()),
        ],
    );
    let mut child = spawn(&s);
    let mut out = BufReader::new(child.stdout.take().expect("piped"));
    let mut first = String::new();
    out.read_line(&mut first).expect("stdout reads");
    assert_eq!(first, "OPEN\n");
    let hash = hash_of(&hash_path);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("a runtime");
    let _g = rt.enter();
    let wt_addr = std::net::SocketAddr::from(([127, 0, 0, 1], wt_port));
    let ws_addr = std::net::SocketAddr::from(([127, 0, 0, 1], ws_port));
    let mut wt = npt_net::WtClient::connect(wt_addr, hash, test_clock).expect("a client");
    let mut ws =
        npt_net::WsClient::connect(ws_addr, "localhost", hash, test_clock).expect("a client");
    let mut wrong = hash;
    wrong[31] ^= 0x80;
    let mut bad = npt_net::WtClient::connect(wt_addr, wrong, test_clock).expect("a client");
    let mut states = [LinkState::Connecting; 3];
    for _ in 0..4_000 {
        for (st, l) in states
            .iter_mut()
            .zip([&mut wt as &mut dyn Link, &mut ws, &mut bad])
        {
            l.poll_recv(0);
            *st = l.state();
        }
        if !states.contains(&LinkState::Connecting) {
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(
        states,
        [LinkState::Open, LinkState::Open, LinkState::Closed],
        "both open at OPEN, no retry; another hash refused"
    );
    // NR-U6B-1: the WebSocket client stays connected while the room ends.
    drop((wt, bad));
    drop(child.stdin.take());
    let status = child.wait().expect("the binary ends");
    drop(ws);
    // SEC-U6B-5 (P15's and P15b's test), after the run: the run folder holds the settings, the tokens, the log and the
    // hash file alone, none with a PKCS#8 key's start (version 0, then the EC AlgorithmIdentifier): the key never on
    // disk.
    let pkcs8 = [
        2, 1, 0, 0x30, 0x13, 6, 7, 0x2a, 0x86, 0x48, 0xce, 0x3d, 2, 1,
    ];
    let has_key = |b: &[u8]| b.windows(pkcs8.len()).any(|w| w == pkcs8);
    let probe = rcgen::KeyPair::generate().expect("a key").serialize_der();
    assert!(has_key(&probe), "the scan sees a P-256 key");
    let mut files = Vec::new();
    let listed = |d: &Path| std::fs::read_dir(d).expect("listed");
    for e in listed(&dir).chain(listed(&hash_dir)) {
        let p = e.expect("an entry").path();
        if p.is_file() {
            assert!(!has_key(&std::fs::read(&p).expect("read")), "{p:?}");
            files.push(p.strip_prefix(&dir).expect("in the run folder").to_owned());
        }
    }
    files.sort();
    let expected = ["hash/room.json", "room.jsonl", "settings.json", "tokens"].map(PathBuf::from);
    assert_eq!(files, expected, "the key never on disk");
    let mut err = String::new();
    child
        .stderr
        .take()
        .expect("piped")
        .read_to_string(&mut err)
        .expect("stderr");
    assert_eq!(status.code(), Some(0), "stderr: {err}");
    let (log, _) = log_of(&dir);
    let p = &log.header.params;
    assert_eq!(p["cert_validity_ns"], 13 * 86_400 * 1_000_000_000u64);
    assert_eq!(
        (p["transport"].as_str(), p["cc"].as_str()),
        (Some("both"), Some("cubic"))
    );
    assert_eq!(p["wt_crate"], "web-transport-quinn 0.13.6");
    assert_eq!(p["conn_cap_count"], 10);
    assert_eq!(p["ws_wrapper"], 1, "P1: the wrapper on");
    assert!(p.contains_key("l2_timer_slack_ns") && p.contains_key("ws_split_fixed_bytes"));
    for ev in ["wt_accept", "ws_accept", "ws_split"] {
        assert!(log.events.iter().any(|e| e.ev == ev), "{ev}");
    }
    // T-7 (P14b's test): both listeners served before OPEN, on the room's one clock.
    let open = log.events.iter().find(|e| e.ev == "open").expect("open");
    let ns = |k: &str| open.f[k].as_u64().expect(k);
    assert!(ns("wt_serving_ns") <= ns("open_ns") && ns("ws_serving_ns") <= ns("open_ns"));
    let at = |ev: &str| log.events.iter().position(|e| e.ev == ev);
    assert_eq!(
        log.events.iter().filter(|e| e.ev == "ws_summary").count(),
        1,
        "the WebSocket client connected at the end has its ws_summary"
    );
    assert!(at("ws_summary") < at("ws_split"), "ws_split after it");
    let split = &log.events[at("ws_split").expect("ws_split")];
    assert_eq!(
        split.f["ws_split_mark_room_ns"], 0,
        "no mark set: NR-U6B-6's row 0"
    );
    let acc = |ev: &str| {
        log.events.iter().find(|e| e.ev == ev).map(|e| {
            (
                e.f["accepted_count"].clone(),
                e.f["handshake_failed_count"].clone(),
            )
        })
    };
    assert_eq!(
        acc("wt_accept"),
        Some((1.into(), 1.into())),
        "one session, one pin refusal"
    );
    assert_eq!(acc("ws_accept").map(|a| a.0), Some(1.into()));
    let again = Command::new(env!("CARGO_BIN_EXE_npt-server"))
        .args(["--settings", s.to_str().expect("utf-8")])
        .stdin(Stdio::null())
        .output()
        .expect("runs");
    // The token file and log exist too: remove them so the hash file alone is the cause.
    assert_eq!(again.status.code(), Some(2));
    let _ = std::fs::remove_file(dir.join("tokens"));
    let _ = std::fs::remove_file(dir.join("room.jsonl"));
    let again = Command::new(env!("CARGO_BIN_EXE_npt-server"))
        .args(["--settings", s.to_str().expect("utf-8")])
        .stdin(Stdio::null())
        .output()
        .expect("runs");
    assert_eq!(again.status.code(), Some(2), "an existing hash path");
    assert!(String::from_utf8_lossy(&again.stderr).contains("hash file"));
    std::fs::remove_dir_all(&dir).expect("removed");
}

/// U6c, C1 (b): probe mode prints `OPEN` once P0's socket is bound, binds no TCP port, mints no token file and writes
/// no hash file; with no peer, stdin's close ends it at once and non-zero, and no leg's log is left (none finished).
#[test]
fn u6c_probe_mode_opens_on_p0s_bind_and_ends_non_zero_when_stdin_closes() {
    let dir = folder("u6c-probe");
    let (wt, ws) = (free_port(true), free_port(false));
    let s = settings(
        &dir,
        &[
            ("mode", "probe".into()),
            ("probe_peer_ipv4", "127.0.0.9".into()),
            ("wt_bind", format!("127.0.0.1:{wt}").into()),
            ("ws_bind", format!("127.0.0.1:{ws}").into()),
        ],
    );
    let mut child = spawn(&s);
    let out = child.stdout.take().expect("piped");
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut first = String::new();
        let _ = BufReader::new(out).read_line(&mut first);
        let _ = tx.send(first);
    });
    let first = rx.recv_timeout(OPEN_CAP).unwrap_or_else(|e| {
        let _ = child.kill();
        panic!("no OPEN within the cap: {e:?}")
    });
    assert_eq!(first, "OPEN\n");
    assert!(
        std::net::TcpStream::connect(("127.0.0.1", ws)).is_err(),
        "no TCP port bound"
    );
    drop(child.stdin.take());
    let status = child.wait().expect("ends");
    let mut err = String::new();
    child
        .stderr
        .take()
        .expect("piped")
        .read_to_string(&mut err)
        .expect("stderr");
    assert_eq!(status.code(), Some(1), "stderr: {err}");
    assert!(err.contains("stdin closed"), "{err}");
    for f in ["tokens", "hash.json", "room-P0.jsonl"] {
        assert!(!dir.join(f).exists(), "{f}");
    }
    std::fs::remove_dir_all(&dir).expect("removed");
}

/// The process-level bites (bites builds only), each failing its mark's check by name on the log it writes.
#[cfg(feature = "bites")]
mod bites {
    use super::*;

    /// PM-0001-16/double-load: the bitten start reads its settings twice more, each behind 1 s, so its OPEN comes
    /// 2 s or more after the spawn, and its header names the bite in a bites build. -16's check on npt-super's own log
    /// (process `super`, the run's bite in its header) is npt-super's test
    /// `h_the_log_header_carries_the_runs_bite_scene_and_tick_rate` (TR-U6A-1).
    #[test]
    fn pm_0001_16_double_load_fails_every_start_under_2_s() {
        let dir = folder("double-load");
        let s = settings(&dir, &[("bite", "PM-0001-16/double-load".into())]);
        // The bite's two 1 s sleeps (server/src/main.rs:505-506) come on top of the start's own 5 s (TR2-U6A-4).
        let (first, open_ns, code, _, err) = start(&s, OPEN_CAP + Duration::from_secs(2));
        assert_eq!(first, "OPEN\n", "stderr: {err}");
        assert_eq!(code, Some(0));
        let h = log_of(&dir).0.header;
        assert_eq!(h.bite.as_deref(), Some("PM-0001-16/double-load"));
        assert_eq!(h.build.features, ["bites"]);
        println!("double-load: spawn to OPEN {:.1} ms", open_ns as f64 / 1e6);
        assert!(
            open_ns >= 2_000_000_000,
            "the bitten start opens 2 s or more after its spawn: {open_ns} ns"
        );
        std::fs::remove_dir_all(&dir).expect("removed");
    }

    /// PM-0006-11/no-smt-map: the bitten room's header has no SMT map, so `machine_named` is 0 and PM-0006-11's check
    /// "RS named: ... threads per core and the SMT sibling map" (machine_named == 1) fails; the clean start's names it.
    #[test]
    fn pm_0006_11_no_smt_map_fails_the_machine_named_check() {
        let dir = folder("no-smt-map");
        let s = settings(&dir, &[("bite", "PM-0006-11/no-smt-map".into())]);
        let (first, _, code, _, err) = start(&s, OPEN_CAP);
        assert_eq!(first, "OPEN\n", "stderr: {err}");
        assert_eq!(code, Some(0));
        let log = log_of(&dir).0;
        assert!(log.header.machine.smt.is_empty(), "no SMT map");
        assert!(
            !machine_named(&log.header.machine),
            "PM-0006-11's check 'RS named: ... and the SMT sibling map' fails: machine_named 0"
        );
        std::fs::remove_dir_all(&dir).expect("removed");
    }
}
