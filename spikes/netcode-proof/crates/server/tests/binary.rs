//! npt-server's binary from `CARGO_BIN_EXE_npt-server` (u6a-brief.md Block 3, section 3): the start to `OPEN` within the
//! 5 s real-clock cap (the time printed: a container reading, no 2 s bar here), exit 0 when stdin closes, no token in
//! the log, stdout or stderr, and the exit codes; in bites builds the two process-level bites, each failing its mark's
//! check by name on the log it writes. The binary binds no socket (SEC-U6AD-4), so no client joins here.

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
        "io_threads": 1, "duration_ms": 0, "warmup_ticks": 0, "first_cpu_model": ""
    });
    v["log"] = dir.join("room.jsonl").to_str().expect("utf-8").into();
    v["token_file"] = dir.join("tokens").to_str().expect("utf-8").into();
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

/// One start: spawn, the time to its first stdout line (the test's stand-in for npt-super's timer, which npt-server's
/// tests cannot run: npt-super's own tests run it against fake children), a few ticks, stdin closed, the exit.
/// Returns (the first line, spawn to that line in ns, the exit code, the rest of stdout, stderr).
#[allow(
    clippy::disallowed_types,
    clippy::disallowed_methods,
    reason = "the test times spawn to OPEN on the real clock (u6a-brief.md section 3); the room reads its own clock in \
              one item"
)]
fn start(settings: &Path) -> (String, u64, Option<i32>, String, String) {
    let t0 = std::time::Instant::now();
    let mut child = spawn(settings);
    let mut out = BufReader::new(child.stdout.take().expect("piped"));
    // The first line on its own thread, awaited at most 5 s (TR-U6A-7): a room that never opens is killed and the test
    // fails at the cap, never hangs.
    let (tx, rx) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        let mut first = String::new();
        let read = out.read_line(&mut first);
        let open_ns = u64::try_from(t0.elapsed().as_nanos()).unwrap_or(u64::MAX);
        let _ = tx.send((read.map(|_| first), open_ns));
        out
    });
    let (first, open_ns) = match rx.recv_timeout(Duration::from_secs(5)) {
        Ok((first, open_ns)) => (first.expect("stdout reads"), open_ns),
        Err(e) => {
            let _ = child.kill();
            let _ = child.wait();
            panic!("no first line within the 5 s cap ({e:?}): the room was killed");
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
    let (first, open_ns, code, rest, err) = start(&settings(&dir, &[]));
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
        let (first, open_ns, code, _, err) = start(&settings(
            &dir,
            &[("bite", "PM-0001-16/double-load".into())],
        ));
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
        let (first, _, code, _, err) =
            start(&settings(&dir, &[("bite", "PM-0006-11/no-smt-map".into())]));
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
