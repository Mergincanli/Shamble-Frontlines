//! The plan's fixtures (§6.1): fixed results logs, judged with the real `marks/marks.json`, that must come out PASS,
//! FAIL and INVALID, PM-0006-14's short padding among them. `fixtures/README.md` lists the cases.

use npt_judge::eval::{BiteStatus, Verdict};
use npt_judge::{VerdictKind, judge, parse_marks, read_logs};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Judges one case: the mark's base logs, with the case's files replacing those of the same name.
fn verdict(mark: &str, case: &str) -> Verdict {
    let text =
        std::fs::read_to_string(root().join("../../marks/marks.json")).expect("marks/marks.json");
    let mut marks = parse_marks(&text).expect("marks.json compiles");
    marks.marks.retain(|m| m.id == mark);
    assert_eq!(marks.marks.len(), 1, "{mark} is in marks.json");
    let dir = root().join("fixtures").join(mark.to_lowercase());
    let mut files: BTreeMap<String, PathBuf> = BTreeMap::new();
    for sub in ["base", case] {
        for entry in std::fs::read_dir(dir.join(sub)).expect("fixture folder") {
            let path = entry.expect("entry").path();
            files.insert(
                path.file_name().unwrap().to_string_lossy().into_owned(),
                path,
            );
        }
    }
    let mut logs = Vec::new();
    for path in files.values() {
        let file = std::fs::File::open(path).expect("fixture file");
        logs.push(
            npt_log::read_log(std::io::BufReader::new(file))
                .unwrap_or_else(|e| panic!("{}: {e}", path.display())),
        );
    }
    judge(&marks, &logs).remove(0)
}

fn says(v: &Verdict, text: &str) -> bool {
    v.why.iter().any(|w| w.contains(text))
}

#[test]
fn pm_0001_4_passes_on_three_valid_runs_and_a_shown_bite() {
    let v = verdict("PM-0001-4", "base");
    assert_eq!(v.verdict, VerdictKind::Pass, "{:?}", v.why);
    assert_eq!(v.parts[0].runs.len(), 3);
    assert!(v.parts[0].runs.iter().all(|r| r.valid));
    assert_eq!(
        v.bites[0].status,
        BiteStatus::Shown,
        "the B-S bite must fail the mark first"
    );
    let p99: Vec<&str> = v.parts[0]
        .runs
        .iter()
        .map(|r| r.checks[0].value.as_deref().unwrap_or(""))
        .collect();
    assert_eq!(
        p99,
        ["9300000", "16670000", "9400000"],
        "16.67 ms is at most 16.67 ms: the edge passes"
    );
}

#[test]
fn pm_0001_4_fails_when_one_run_is_over_the_mark() {
    let v = verdict("PM-0001-4", "fail");
    assert_eq!(v.verdict, VerdictKind::Fail, "{:?}", v.why);
    assert!(
        says(
            &v,
            "run s30-2: tick p99 at most 16.67 ms = 16680000, needs <= 16670000"
        ),
        "{:?}",
        v.why
    );
}

#[test]
fn pm_0001_4_is_invalid_when_v_s_breaks() {
    let v = verdict("PM-0001-4", "invalid");
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
    assert!(
        says(
            &v,
            "run s30-3: V-S: RS's steal averaged 11.5 permille, over 10 (1 %)"
        ),
        "{:?}",
        v.why
    );
}

#[test]
fn pm_0006_14_counts_a_padded_session_and_waits_for_the_other_n() {
    let v = verdict("PM-0006-14", "base");
    assert_eq!(v.verdict, VerdictKind::NotRun, "{:?}", v.why);
    assert_eq!(v.parts[0].name, "N=48");
    assert_eq!(
        v.parts[0].verdict,
        VerdictKind::Pass,
        "{:?}",
        v.parts[0].why
    );
    assert!(
        says(&v, "N=36: 0 of 1 valid runs") && says(&v, "N=18: 0 of 1 valid runs"),
        "{:?}",
        v.why
    );
}

#[test]
fn pm_0006_14_short_padding_comes_out_invalid() {
    let v = verdict("PM-0006-14", "short");
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
    assert!(
        says(
            &v,
            "run d48-k4: padding: 1 measured ticks fell short of 8330000 ns (first: tick 3 at 8329000 ns)"
        ),
        "{:?}",
        v.why
    );
}

#[test]
fn the_fixtures_read_through_the_command_line_folder_reader_too() {
    // read_logs walks a folder the way the binary does; every base folder reads clean.
    for mark in ["pm-0001-4", "pm-0006-14"] {
        let logs = read_logs(&root().join("fixtures").join(mark).join("base")).expect("reads");
        assert!(!logs.is_empty());
        assert!(
            logs.iter().all(|l| l.footer.is_some()),
            "every fixture log is complete"
        );
    }
}
