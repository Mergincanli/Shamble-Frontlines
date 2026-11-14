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
        // PM-0001-1's one case is a whole run of its own, so that mark has no base folder.
        if sub == "base" && !dir.join(sub).exists() {
            continue;
        }
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

/// One part's run: its outcome and its first check's value.
fn part_run<'v>(v: &'v Verdict, part: &str) -> &'v npt_judge::eval::RunOut {
    let p = v
        .parts
        .iter()
        .find(|p| p.name == part)
        .unwrap_or_else(|| panic!("{part}"));
    assert_eq!(p.runs.len(), 1, "{part}: {:?}", p.runs);
    &p.runs[0]
}

fn reported<'v>(v: &'v Verdict, name: &str) -> Vec<&'v str> {
    v.report
        .iter()
        .filter(|r| r.name.starts_with(name))
        .map(|r| r.value.as_str())
        .collect()
}

/// u4-build-plan.md §4.6's six decoded-hash cases, on PM-0001-17's L1 P2 part (the stress scene's load as its
/// precondition): equal; differing; a lost group (not compared); a refused group (the sentinel); a client hash at a
/// tick the server never logged; 0 compared.
#[test]
fn pm_0001_17_equal_hashes_pass_and_the_lost_group_is_not_compared() {
    for (case, compared, incomplete) in [("base", "6", "0"), ("lost-group", "5", "1")] {
        let v = verdict("PM-0001-17", case);
        assert_eq!(
            v.verdict,
            VerdictKind::NotRun,
            "{case}: the other parts wait: {:?}",
            v.why
        );
        let run = part_run(&v, "L1 P2");
        assert!(run.valid, "{case}: {:?}", run.invalid_why);
        assert_eq!(run.checks[0].value.as_deref(), Some("0"), "{case}");
        assert_eq!(
            reported(&v, "decoded snapshots compared"),
            [compared],
            "{case}"
        );
        assert_eq!(
            reported(&v, "client-ticks left incomplete"),
            [incomplete],
            "{case}"
        );
    }
}

#[test]
fn pm_0001_17_fails_on_a_difference_the_sentinel_an_orphan_and_nothing_compared() {
    for (case, expected) in [
        (
            "differ",
            "rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
        ),
        (
            "refused-group",
            "(0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
        ),
        (
            "orphan",
            "(0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
        ),
        (
            "none-compared",
            "no client logged a hash (0 compared); an empty comparison fails",
        ),
        // The advisor's FX-12: pairing is by the room's slot, so two clients' entries swapped fail all six.
        (
            "permuted-slots",
            "(0 mismatches; on L2 under the Arm rule) = 6, needs == 0",
        ),
    ] {
        let v = verdict("PM-0001-17", case);
        assert_eq!(v.verdict, VerdictKind::Fail, "{case}: {:?}", v.why);
        assert!(says(&v, "L1 P2: run p2-1: "), "{case}: {:?}", v.why);
        assert!(says(&v, expected), "{case}: {:?}", v.why);
    }
}

/// One part's verdict and reasons.
fn part_of<'v>(v: &'v Verdict, part: &str) -> &'v npt_judge::eval::PartOut {
    v.parts
        .iter()
        .find(|p| p.name == part)
        .unwrap_or_else(|| panic!("{part}"))
}

/// A report's values in one part.
fn reported_in<'v>(v: &'v Verdict, part: &str, name: &str) -> Vec<&'v str> {
    v.report
        .iter()
        .filter(|r| r.part == part && r.name.starts_with(name))
        .map(|r| r.value.as_str())
        .collect()
}

/// The Arm rule on the real marks.json's L2 P2 part (the advisor's FX-5 on U4j; the test-engineer's F12, C1, C2 and
/// O): each hash name from its own process (`server_hash` from `room`, `decoded_hash` from `client` or `bots`,
/// `replay_hash` from `replay` on x86_64), the rule only for an LG client on aarch64 with the room on x86_64, a
/// client hash the server never logged counted on every machine, and a missing x86-64 re-decode entry a gap. Every
/// case's outcome is collected, so one run shows them all.
#[test]
fn pm_0001_17_the_arm_rule_holds_on_lg_only() {
    let mut wrong = Vec::new();
    // Cross-arch: the LG hash differs and the x86-64 re-decode equals the server's: valid, 0, aarch64-only 1.
    let v = verdict("PM-0001-17", "lg-cross-arch");
    let p = part_of(&v, "L2 P2");
    let shown = (
        p.verdict,
        p.runs.first().map(|r| r.valid),
        p.runs
            .first()
            .and_then(|r| r.checks.first())
            .and_then(|c| c.value.clone()),
        reported_in(&v, "L2 P2", "aarch64-only"),
    );
    if shown
        != (
            VerdictKind::Pass,
            Some(true),
            Some("0".to_owned()),
            vec!["1"],
        )
    {
        wrong.push(format!("lg-cross-arch: {shown:?} {:?}", p.why));
    }
    for (case, part, kind, expected) in [
        (
            "lg-replay-differs",
            "L2 P2",
            VerdictKind::Fail,
            "L2 P2: run l2-p2-1: 100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
        ),
        (
            "lg-replay-in-client-log",
            "L2 P2",
            VerdictKind::Invalid,
            "hs.replay_hash in the bots log on LG1 (aarch64): only process replay on x86_64 writes it",
        ),
        (
            "lg-replay-on-aarch64",
            "L2 P2",
            VerdictKind::Invalid,
            "hs.replay_hash in the replay log on C (aarch64): only process replay on x86_64 writes it",
        ),
        (
            "l1-aarch64-claim",
            "L1 P2",
            VerdictKind::Fail,
            "L1 P2: run p2-1: 100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
        ),
        (
            "lg-orphan",
            "L2 P2",
            VerdictKind::Fail,
            "L2 P2: run l2-p2-1: 100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
        ),
        (
            "lg-replay-missing-entry",
            "L2 P2",
            VerdictKind::Invalid,
            "slot 1's LG aarch64 hash at tick 2 differs from the server's, and the x86-64 re-decode has no entry there",
        ),
        // FX-10 (f): an LG mismatch waiting for its re-decode never hides an orphan's count (NOT-RUN before).
        (
            "lg-orphan-no-replay",
            "L2 P2",
            VerdictKind::Fail,
            "L2 P2: run l2-p2-1: 100 % of decoded snapshots rebuild the server's quantised state (0 mismatches; on L2 under the Arm rule) = 1, needs == 0",
        ),
    ] {
        let v = verdict("PM-0001-17", case);
        let p = part_of(&v, part);
        if p.verdict != kind || !p.why.iter().any(|w| w.contains(expected)) {
            wrong.push(format!(
                "{case}: {part} is {:?}, wanted {kind:?} with {expected:?}: {:?}",
                p.verdict, p.why
            ));
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// README §1's `null` rule, checked (the advisor's FX-10 on U4j; the test-engineer's probe S and R2): every `hs` and
/// `tick.s.snap_incomplete` list holds the run's `clients` entries; a (slot, tick) is resolved by a client hash or by
/// `snap_incomplete` 1, never both; every tick inside a slot's resolved span at which the room logged that slot's
/// hash is resolved, and the span covers the room's measured ticks for the slot. Each case is INVALID.
#[test]
fn pm_0001_17_every_server_hash_is_resolved_on_the_client_side() {
    let mut wrong = Vec::new();
    for (case, expected) in [
        (
            "sparse",
            "slot 0: the client side resolves ticks 1 to 1 only, not the room's measured ticks 1 to 3",
        ),
        (
            "sampled",
            "slot 0, tick 2: the room logged a hash and no client log resolves it",
        ),
        (
            "slot-unlogged",
            "slot 1: the room logged its hash at measured ticks 1 to 3, and no client log resolves any tick of it",
        ),
        (
            "hashed-and-incomplete",
            "slot 1, tick 2: a client hash and tick.s.snap_incomplete 1 at once",
        ),
        (
            "short-lists",
            "hs.server_hash lists hold 2 entries, not the run's 48 clients",
        ),
    ] {
        let v = verdict("PM-0001-17", case);
        let p = part_of(&v, "L1 P2");
        if p.verdict != VerdictKind::Invalid || !p.why.iter().any(|w| w.contains(expected)) {
            wrong.push(format!(
                "{case}: L1 P2 is {:?}, wanted Invalid with {expected:?}: {:?}",
                p.verdict, p.why
            ));
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// C4's cases (the advisor's U2 ruling): a measured run built with the bites, a measured run naming a bite, and a bite
/// run from a measured build are INVALID; the fourth, a twin from the wrong build, is PM-0001-21's `twin-bites-build`.
#[test]
fn pm_0001_4_c4_the_build_must_match_the_bite() {
    for (case, expected) in [
        (
            "measured-bites-build",
            "run s30-1: build: a measured run names no bite, but a log of it comes from a bites build",
        ),
        (
            "measured-names-bite",
            "run s30-2: build: a measured run names the bite PM-0001-4/B-S",
        ),
        (
            "bite-measured-build",
            "bite PM-0001-4/B-S: its run is invalid",
        ),
    ] {
        let v = verdict("PM-0001-4", case);
        assert_eq!(v.verdict, VerdictKind::Invalid, "{case}: {:?}", v.why);
        assert!(says(&v, expected), "{case}: {:?}", v.why);
    }
    let v = verdict("PM-0001-4", "bite-measured-build");
    assert!(
        v.bites[0].runs[0]
            .invalid_why
            .iter()
            .any(|w| w.contains("none of its logs comes from a bites build")),
        "{:?}",
        v.bites[0].runs
    );
}

/// twin_hash_mismatch_count on PM-0001-21's forged part: equal hashes pass, a differing tick fails, a twin from a
/// bites build (C4's fourth case) or no twin named is INVALID, a twin not in the set yet is NOT-RUN.
#[test]
fn pm_0001_21_the_forged_run_against_its_valid_parts_twin() {
    let v = verdict("PM-0001-21", "base");
    assert_eq!(v.verdict, VerdictKind::NotRun, "{:?}", v.why);
    let run = part_run(&v, "forged, option A");
    assert!(run.valid, "{:?}", run.invalid_why);
    let twin = run
        .checks
        .iter()
        .find(|c| c.name.starts_with("forged: 0 effect"))
        .expect("the twin check");
    assert_eq!(twin.value.as_deref(), Some("0"));
    for (case, kind, expected) in [
        (
            "twin-differs",
            VerdictKind::Fail,
            "forged: 0 effect beyond the valid part (state hash equals the valid-parts twin's on every tick) = 1, needs == 0",
        ),
        (
            "twin-bites-build",
            VerdictKind::Invalid,
            "its twin forged-a-1-twin is invalid: build: a twin run names no bite, but a log of it comes from a bites build",
        ),
        (
            "twin-unnamed",
            VerdictKind::Invalid,
            "params.twin names no twin run",
        ),
        (
            "twin-missing",
            VerdictKind::NotRun,
            "its twin forged-a-1-twin is not in set u4f-21 yet",
        ),
    ] {
        let v = verdict("PM-0001-21", case);
        assert_eq!(v.verdict, kind, "{case}: {:?}", v.why);
        assert!(says(&v, expected), "{case}: {:?}", v.why);
    }
}

/// The advisor's FX-4 on U4j (the test-engineer's probe D): two forged measured runs that name each other, with equal
/// state hashes, never pass -21's "0 effect beyond the valid part": the twin must be a run of role twin, else a gap.
#[test]
fn pm_0001_21_the_twin_must_be_role_twin() {
    let v = verdict("PM-0001-21", "twin-is-measured");
    assert_eq!(v.verdict, VerdictKind::Invalid, "{:?}", v.why);
    for (run, twin) in [
        ("forged-a-1", "forged-a-1-twin"),
        ("forged-a-1-twin", "forged-a-1"),
    ] {
        assert!(
            says(
                &v,
                &format!(
                    "run {run}: a gap in the record: run {run}: {twin} has role measured, not twin"
                )
            ),
            "{:?}",
            v.why
        );
    }
}

/// The advisor's FX-14 (TE-F13): PM-0001-1's ML part, a laptop's log naming its machine in full with no address
/// (only a VM gives its day address), passes its machine_named figure.
#[test]
fn pm_0001_1_a_laptop_needs_no_address() {
    let v = verdict("PM-0001-1", "laptop");
    let p = part_of(&v, "ML (the named laptop)");
    assert_eq!(p.verdict, VerdictKind::Pass, "{:?}", p.why);
    let run = part_run(&v, "ML (the named laptop)");
    assert!(run.valid, "{:?}", run.invalid_why);
    let named = run
        .checks
        .iter()
        .find(|c| c.name.starts_with("every machine named"))
        .expect("the machine_named check");
    assert_eq!(named.value.as_deref(), Some("1"), "{named:?}");
}

/// machine_named on PM-0006-11: a fully named RS passes and the no-SMT-map bite fails it; a header with no kernel
/// fails.
#[test]
fn pm_0006_11_every_machine_named_and_its_bite_shown() {
    let v = verdict("PM-0006-11", "base");
    assert_eq!(v.verdict, VerdictKind::Pass, "{:?}", v.why);
    assert_eq!(
        v.bites[0].status,
        BiteStatus::Shown,
        "{:?}",
        v.bites[0].runs
    );
    let v = verdict("PM-0006-11", "unnamed");
    assert_eq!(v.verdict, VerdictKind::Fail, "{:?}", v.why);
    assert!(
        says(&v, "machine_named") || says(&v, "RS named"),
        "{:?}",
        v.why
    );
}

#[test]
fn the_fixtures_read_through_the_command_line_folder_reader_too() {
    // read_logs walks a folder the way the binary does; every base folder reads clean, and so do the Arm rule's L2
    // cases (whole runs of their own).
    for (mark, case) in [
        ("pm-0001-4", "base"),
        ("pm-0001-17", "base"),
        ("pm-0001-17", "lg-cross-arch"),
        ("pm-0001-17", "lg-replay-differs"),
        ("pm-0001-17", "lg-replay-in-client-log"),
        ("pm-0001-17", "lg-replay-on-aarch64"),
        ("pm-0001-17", "lg-orphan"),
        ("pm-0001-17", "lg-replay-missing-entry"),
        ("pm-0001-17", "lg-orphan-no-replay"),
        ("pm-0001-21", "base"),
        ("pm-0001-30", "base"),
        ("pm-0006-11", "base"),
        ("pm-0006-14", "base"),
    ] {
        let logs = read_logs(&root().join("fixtures").join(mark).join(case)).expect("reads");
        assert!(!logs.is_empty());
        assert!(
            logs.iter().all(|l| l.footer.is_some()),
            "every fixture log is complete"
        );
    }
}
