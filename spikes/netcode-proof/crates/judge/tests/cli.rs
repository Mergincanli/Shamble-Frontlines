//! npt-judge's command line, run as a binary on temporary folders: the verdict lines, the JSON it writes, and its exit
//! codes (0 judged, 1 a malformed input, 2 bad usage).

use npt_log::{Build, Ended, Header, Machine, Phase, SCHEMA, TickRecord, Writer};
use std::path::{Path, PathBuf};
use std::process::Command;

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("npt-judge-cli-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("logs")).expect("temp dir");
    dir
}

fn write_marks(dir: &Path) -> PathBuf {
    let marks = serde_json::json!({
        "schema": 1, "drafts_commit": "a".repeat(40),
        "marks": [
            {"id": "PM-0001-4", "title": "tick budget", "verdict": "pass",
             "draft": {"file": "docs/adr/0001-multiplayer-netcode.md", "row_sha256": "0".repeat(64)},
             "parts": [{"name": "S30", "select": {"layer": "L1"}, "min": 1}],
             "checks": [{"name": "tick p99", "series": "tick.v.work_ns", "stat": "p99", "op": "<=", "value": 100}]},
            {"id": "PM-0001-39", "title": "60 Hz", "verdict": "outside", "notes": "the 60 Hz run",
             "draft": {"file": "docs/adr/0001-multiplayer-netcode.md", "row_sha256": "0".repeat(64)}}
        ]
    });
    let path = dir.join("marks.json");
    std::fs::write(&path, marks.to_string()).expect("marks");
    path
}

fn write_log(path: &Path, worst: i64) {
    let header = Header {
        schema: SCHEMA,
        run: "c-1".to_owned(),
        set: "c".to_owned(),
        role: "measured".to_owned(),
        marks: vec!["PM-0001-4".to_owned()],
        machine: Machine {
            role: "C".to_owned(),
            name: "C".to_owned(),
            plan: "container".to_owned(),
            arch: "x86_64".to_owned(),
            cpu: "test".to_owned(),
            vcpus: 4,
            mem_mb: 8192,
            location: "here".to_owned(),
            kernel: "6".to_owned(),
            address: String::new(),
            threads_per_core: 1,
            smt: vec![],
            clock_res_ns: 1,
        },
        build: Build {
            commit: "0".repeat(40),
            binary_sha256: "0".repeat(64),
            features: vec![],
            profile: "release".to_owned(),
        },
        bite: None,
        layer: "L1".to_owned(),
        profile: "P2".to_owned(),
        seed: 1,
        tick_hz: 30,
        clients: 48,
        scene: "SS".to_owned(),
        process: "lab".to_owned(),
        slot: 0,
        date: "2026-11-06".to_owned(),
        params: Default::default(),
    };
    let file = std::fs::File::create(path).expect("log file");
    let mut w = Writer::new(file, &header).expect("header");
    for (t, v) in [(1, 10), (2, worst)] {
        let mut r = TickRecord::new(t, Phase::Measured);
        r.v.insert("work_ns".to_owned(), v);
        w.tick(&r).expect("tick");
    }
    w.finish(Ended::Clean).expect("footer");
}

fn judge(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_npt-judge"))
        .args(args)
        .output()
        .expect("runs");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn judges_a_folder_and_writes_the_verdicts() {
    let dir = temp_dir("judge");
    let marks = write_marks(&dir);
    write_log(&dir.join("logs").join("room.jsonl"), 90);
    let out = dir.join("verdicts.json");
    let (code, stdout, stderr) = judge(&[
        "--marks",
        marks.to_str().unwrap(),
        "--logs",
        dir.join("logs").to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "{stderr}");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 2, "{stdout}");
    assert!(lines[0].starts_with("PM-0001-4    PASS"), "{stdout}");
    assert!(
        lines[1].starts_with("PM-0001-39   NOT-RUN   outside this change-set"),
        "{stdout}"
    );
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).unwrap()).unwrap();
    assert_eq!(json[0]["verdict"], "PASS");
    assert_eq!(json[0]["parts"][0]["runs"][0]["checks"][0]["value"], "90");
    let (code, stdout, _) = judge(&[
        "--marks",
        marks.to_str().unwrap(),
        "--logs",
        dir.join("logs").to_str().unwrap(),
        "--only",
        "PM-0001-39",
    ]);
    assert_eq!(code, 0);
    assert_eq!(stdout.lines().count(), 1, "--only keeps one mark");
    write_log(&dir.join("logs").join("room.jsonl"), 101);
    let (code, stdout, _) = judge(&[
        "--marks",
        marks.to_str().unwrap(),
        "--logs",
        dir.join("logs").to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "a FAIL is a verdict, not an error");
    assert!(stdout.starts_with("PM-0001-4    FAIL"), "{stdout}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn malformed_inputs_exit_1_and_bad_usage_exits_2() {
    let dir = temp_dir("errors");
    let marks = write_marks(&dir);
    let (code, stdout, _) = judge(&["--marks", marks.to_str().unwrap(), "--check-marks"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("2 marks read and checked"), "{stdout}");
    let bad = dir.join("bad.json");
    std::fs::write(&bad, r#"{"schema":2,"drafts_commit":"x","marks":[]}"#).unwrap();
    let (code, _, stderr) = judge(&["--marks", bad.to_str().unwrap(), "--check-marks"]);
    assert_eq!(code, 1);
    assert!(
        stderr.contains("schema 2") && stderr.contains("drafts_commit"),
        "{stderr}"
    );
    std::fs::write(
        dir.join("logs").join("broken.jsonl"),
        "{\"k\":\"tick\",\"t\":1}\n",
    )
    .unwrap();
    let (code, _, stderr) = judge(&[
        "--marks",
        marks.to_str().unwrap(),
        "--logs",
        dir.join("logs").to_str().unwrap(),
    ]);
    assert_eq!(code, 1, "a malformed log is never skipped");
    assert!(stderr.contains("broken.jsonl: line 1"), "{stderr}");
    assert_eq!(judge(&[]).0, 2);
    assert_eq!(
        judge(&["--marks", marks.to_str().unwrap()]).0,
        2,
        "neither --logs nor --check-marks"
    );
    assert_eq!(
        judge(&[
            "--marks",
            marks.to_str().unwrap(),
            "--check-marks",
            "--what"
        ])
        .0,
        2
    );
    let (code, _, stderr) = judge(&[
        "--marks",
        marks.to_str().unwrap(),
        "--logs",
        ".",
        "--only",
        "PM-9",
    ]);
    assert_eq!(code, 2, "{stderr}");
    let _ = std::fs::remove_dir_all(&dir);
}
