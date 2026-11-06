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
    // The whole file: a mark's preconditions (pre:PM-0001-2) are other marks of it.
    let marks = parse_marks(&text).expect("marks.json compiles");
    let index = marks
        .marks
        .iter()
        .position(|m| m.id == mark)
        .unwrap_or_else(|| panic!("{mark} is in marks.json"));
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
    judge(&marks, &logs).remove(index)
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
fn pm_0001_4_is_invalid_when_a_run_did_not_carry_the_stress_scenes_load() {
    let v = verdict("PM-0001-4", "load-short");
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
    assert!(
        says(
            &v,
            "run s30-1: pre:PM-0001-2: 48 soldiers on every measured tick = 47, needs >= 48"
        ),
        "{:?}",
        v.why
    );
}

#[test]
fn pm_0001_4_fails_with_its_gap_listed_beside_the_failure() {
    let v = verdict("PM-0001-4", "fail-with-gap");
    assert_eq!(
        v.verdict,
        VerdictKind::Fail,
        "a gap never hides a failure: {:?}",
        v.why
    );
    assert!(
        says(
            &v,
            "run s30-2: tick p99 at most 16.67 ms = 16680000, needs <= 16670000"
        ),
        "{:?}",
        v.why
    );
    assert!(
        says(&v, "run s30-2: also a gap in the record") && says(&v, "tick.v.overrun"),
        "{:?}",
        v.why
    );
}

#[test]
fn pm_0001_30s_validity_bite_is_shown_by_a_late_command() {
    let v = verdict("PM-0001-30", "base");
    let bite = v
        .bites
        .iter()
        .find(|b| b.id == "PM-0001-30/late-command")
        .expect("the late-command bite");
    assert_eq!(bite.status, BiteStatus::Shown, "{:?}", bite.runs);
    assert!(
        bite.runs[0]
            .invalid_why
            .iter()
            .any(|w| w.contains("late-zero: 1 commands ran late"))
    );
}

#[test]
fn pm_0001_30_fails_when_the_validity_check_lets_a_late_command_through() {
    let v = verdict("PM-0001-30", "missed");
    assert_eq!(v.verdict, VerdictKind::Fail, "{:?}", v.why);
    assert!(
        says(&v, "bite PM-0001-30/late-command did not fail"),
        "{:?}",
        v.why
    );
}

#[test]
fn the_fixtures_read_through_the_command_line_folder_reader_too() {
    // read_logs walks a folder the way the binary does; every base folder reads clean.
    for mark in ["pm-0001-4", "pm-0001-30", "pm-0006-14"] {
        let logs = read_logs(&root().join("fixtures").join(mark).join("base")).expect("reads");
        assert!(!logs.is_empty());
        assert!(
            logs.iter().all(|l| l.footer.is_some()),
            "every fixture log is complete"
        );
    }
}
