//! The plan's fixtures (§6.1): fixed results logs, judged with the real `marks/marks.json`, that must come out PASS,
//! FAIL and INVALID, PM-0006-14's short padding among them. `fixtures/README.md` lists the cases.

use npt_judge::eval::{BiteStatus, Verdict};
use npt_judge::marks::{Kind, Mark};
use npt_judge::{VerdictKind, judge, parse_marks, read_logs};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn marks_text() -> String {
    std::fs::read_to_string(root().join("../../marks/marks.json")).expect("marks/marks.json")
}

/// Judges one case: the mark's base logs, with the case's files replacing those of the same name.
fn verdict(mark: &str, case: &str) -> Verdict {
    verdict_with(&marks_text(), mark, case).expect("marks.json compiles")
}

/// The same with this marks text (the real file, or the real file with a label added); the loader's refusal as `Err`.
fn verdict_with(text: &str, mark: &str, case: &str) -> Result<Verdict, Vec<String>> {
    // The whole file: a mark's preconditions (pre:PM-0001-2) are other marks of it.
    let marks = parse_marks(text)?;
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
    Ok(judge(&marks, &logs).remove(index))
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

/// PM-0006-14 after the trim: its N = 36 and N = 18 parts moved to slice 6, so the padded N = 48 session reads MOVED
/// (never REPORTED, and no longer NOT-RUN waiting for the other two counts).
#[test]
fn pm_0006_14_counts_a_padded_session_and_its_other_n_moved() {
    let v = verdict("PM-0006-14", "base");
    assert_eq!(v.verdict, VerdictKind::Moved, "{:?}", v.why);
    assert_eq!(v.parts[0].name, "N=48");
    assert_eq!(
        v.parts[0].verdict,
        VerdictKind::Pass,
        "{:?}",
        v.parts[0].why
    );
    assert!(
        says(
            &v,
            "moved, never judged here: part \"N=36\" to slice 6, on the real sim; part \"N=18\" to slice 6, on the real sim"
        ),
        "{:?}",
        v.why
    );
    assert!(
        v.parts[1..]
            .iter()
            .all(|p| p.verdict == VerdictKind::Moved && p.runs.is_empty()),
        "{:?}",
        v.parts
    );
    assert!(!v.report.is_empty(), "the N = 48 count is still reported");
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
/// hash is resolved, and the span covers the room's measured ticks for the slot. Each case is INVALID (both at once:
/// the next test).
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

/// C62H-BOUND (B-2): slot 1's tick 2 is filed lost while the room sent it a snapshot and the link lost none of it, so
/// `snap_lost_unexplained_count` reads 1 and the part FAILs (INVALID before), the coverage gap listed beside it.
#[test]
fn pm_0001_17_hashed_and_incomplete_fails_with_its_coverage_gap_beside() {
    let v = verdict("PM-0001-17", "hashed-and-incomplete");
    let p = part_of(&v, "L1 P2");
    assert_eq!(p.verdict, VerdictKind::Fail, "{:?}", p.why);
    let why = "(0 unexplained) = 1, needs == 0";
    assert!(p.why.iter().any(|w| w.contains(why)), "{:?}", p.why);
    let gap = "slot 1, tick 2: a client hash and tick.s.snap_incomplete 1 at once";
    let gaps = &part_run(&v, "L1 P2").gaps;
    assert!(gaps.iter().any(|g| g.contains(gap)), "{gaps:?}");
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

// ---------------------------------------------------------------------------------------------------------------
// MOVED on the real marks.json (README §2.6; the trim of 2026-11-21 and its re-plan).

/// What the trim moved, read from the real file, so a label lost or added by mistake fails here by name: the marks
/// moved whole, the three outside from the start, and the kept marks with a moved part, check or bite.
#[test]
fn the_real_marks_file_labels_what_the_trim_moved() {
    let marks = parse_marks(&marks_text()).expect("marks.json compiles");
    // Ids in the file's order, without their "PM-000" start, joined by spaces.
    let ids = |keep: &dyn Fn(&Mark) -> bool| -> String {
        let kept = marks.marks.iter().filter(|m| keep(m));
        let short: Vec<&str> = kept.map(|m| m.id.trim_start_matches("PM-000")).collect();
        short.join(" ")
    };
    let whole = ids(&|m| m.moved.is_some());
    let outside = ids(&|m| m.kind == Kind::Outside);
    let in_part = ids(&|m| m.moved.is_none() && m.any_moved());
    assert_eq!(
        whole,
        "1-25 1-32 1-33 1-34 1-35 1-41 1-42 1-60 1-43 1-44 1-45 1-46 1-61 1-47 1-48 1-49 1-50 1-51 1-52 \
         1-62 1-53 1-54 1-55 6-15 6-26 6-27"
    );
    assert_eq!(outside, "1-39 1-40 1-63");
    assert_eq!(
        in_part,
        "1-1 1-3 1-14 1-58 1-19 1-28 1-29 1-31 1-36 1-56 6-14 6-25"
    );
    let count = |of: &dyn Fn(&Mark) -> usize| marks.marks.iter().map(of).sum::<usize>();
    let parts = count(&|m| m.parts.iter().filter(|p| p.moved.is_some()).count());
    let checks = count(&|m| m.checks.iter().filter(|c| c.moved.is_some()).count());
    let bites = count(&|m| m.bites.iter().filter(|b| b.moved.is_some()).count());
    assert_eq!((parts, checks, bites), (21, 2, 2));
    // With no log at all, the 29 read MOVED and every other mark NOT-RUN: a kept mark with a moved piece still waits
    // for its kept parts, so MOVED never stands in for a run that is missing.
    for v in judge(&marks, &[]) {
        let id = v.mark.trim_start_matches("PM-000");
        let left = whole.split(' ').chain(outside.split(' ')).any(|x| x == id);
        let want = if left {
            VerdictKind::Moved
        } else {
            VerdictKind::NotRun
        };
        assert_eq!(v.verdict, want, "{}: {:?}", v.mark, v.why);
    }
}

fn label() -> serde_json::Value {
    serde_json::json!({"to": "a later test", "why": "a test's label", "by": "tests/fixtures.rs"})
}

/// The real marks.json with one mark's entry edited.
fn edited(mark: &str, edit: &dyn Fn(&mut serde_json::Value)) -> String {
    let mut file: serde_json::Value = serde_json::from_str(&marks_text()).expect("marks.json");
    let entry = file["marks"]
        .as_array_mut()
        .expect("marks")
        .iter_mut()
        .find(|m| m["id"] == mark)
        .unwrap_or_else(|| panic!("{mark} is in marks.json"));
    edit(entry);
    file.to_string()
}

fn count_of(mark: &str, list: &str) -> usize {
    let file: serde_json::Value = serde_json::from_str(&marks_text()).expect("marks.json");
    let entry = file["marks"].as_array().expect("marks").iter();
    entry
        .filter(|m| m["id"] == mark)
        .map(|m| m[list].as_array().map_or(0, Vec::len))
        .sum()
}

/// The property (the re-plan's): a label added anywhere on a mark that passes today never reads PASS. On the two
/// committed cases that read PASS with the real file, each position in turn: the mark, each part, each check, each
/// bite. It reads MOVED; or FAIL where the labelled check was the one the mark's bite fails, so the kept bite now
/// passes every kept check. A position the loader refuses (the mark's only part) is counted, never judged.
#[test]
fn a_label_anywhere_on_a_passing_mark_never_reads_pass() {
    for (mark, case) in [("PM-0001-4", "base"), ("PM-0006-11", "base")] {
        assert_eq!(verdict(mark, case).verdict, VerdictKind::Pass, "{mark}");
        let mut positions: Vec<Option<(&str, usize)>> = vec![None];
        for list in ["parts", "checks", "bites"] {
            positions.extend((0..count_of(mark, list)).map(|i| Some((list, i))));
        }
        let (mut moved, mut missed, mut refused) = (0, 0, 0);
        for at in &positions {
            let text = edited(mark, &|m| match at {
                None => m["moved"] = label(),
                Some((list, i)) => m[*list][*i]["moved"] = label(),
            });
            match verdict_with(&text, mark, case) {
                Ok(v) if v.verdict == VerdictKind::Moved => moved += 1,
                Ok(v) => {
                    let bite_missed = v.verdict == VerdictKind::Fail
                        && v.why.iter().all(|w| w.contains("did not fail"))
                        && matches!(at, Some(("checks", _)));
                    assert!(bite_missed, "{mark} {at:?}: {} {:?}", v.verdict, v.why);
                    missed += 1;
                }
                Err(errors) => {
                    assert!(
                        matches!(at, Some(("parts", _)))
                            && errors.iter().any(|e| e.contains("every part is moved")),
                        "{mark} {at:?}: {errors:?}"
                    );
                    refused += 1;
                }
            }
        }
        assert!(
            moved >= 3 && missed == 1 && refused == 1,
            "{mark}: {moved}, {missed}, {refused}"
        );
    }
}

/// The other half of the order: a kept FAIL or INVALID still shows. On committed cases that read FAIL or INVALID from
/// a kept part, a label on each bite in turn leaves the verdict as it was.
#[test]
fn a_moved_bite_hides_no_kept_failure_or_invalid_run() {
    for (mark, case, kind) in [
        ("PM-0001-4", "fail", VerdictKind::Fail),
        ("PM-0001-4", "fail-with-gap", VerdictKind::Fail),
        ("PM-0001-4", "invalid", VerdictKind::Invalid),
        ("PM-0001-4", "load-short", VerdictKind::Invalid),
        ("PM-0006-11", "unnamed", VerdictKind::Fail),
        ("PM-0001-17", "differ", VerdictKind::Fail),
        ("PM-0001-17", "sparse", VerdictKind::Invalid),
    ] {
        assert_eq!(verdict(mark, case).verdict, kind, "{mark} {case}");
        let bites = count_of(mark, "bites");
        assert!(bites > 0, "{mark} has a bite to label");
        for i in 0..bites {
            let text = edited(mark, &|m| m["bites"][i]["moved"] = label());
            let v = verdict_with(&text, mark, case).expect("a moved bite compiles");
            assert_eq!(v.verdict, kind, "{mark} {case}, bite {i}: {:?}", v.why);
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
// PM-0001-30's twin form (C-U6-9): each measured run against its other-transport twin, on the real marks.json.

const C_PART: &str = "C: loopback, 3 recordings x 2 transports";

/// The container's six runs (3 recordings x wt and ws), each naming its other-transport run: equal hashes pass the
/// C part. The mark still waits for RS's six runs and its wallclock bite.
#[test]
fn pm_0001_30_each_run_is_compared_with_its_other_transport_twin() {
    let v = verdict("PM-0001-30", "base");
    let part = part_of(&v, C_PART);
    assert_eq!(part.verdict, VerdictKind::Pass, "{:?}", part.why);
    assert_eq!(part.runs.len(), 6);
    assert!(
        part.runs
            .iter()
            .all(|r| r.valid && r.checks[0].value.as_deref() == Some("0"))
    );
    assert_eq!(v.verdict, VerdictKind::NotRun, "{:?}", v.why);
    // One transport's tick 2 differs: both runs of that recording fail, each against the other.
    let v = verdict("PM-0001-30", "twin-differs");
    assert_eq!(v.verdict, VerdictKind::Fail, "{:?}", v.why);
    let needs = "identical server state hashes on every tick (each run against its other-transport twin) = 1, needs == 0";
    for run in ["c-rec2-ws", "c-rec2-wt"] {
        assert!(
            says(&v, &format!("{C_PART}: run {run}: {needs}")),
            "{:?}",
            v.why
        );
    }
    // The twin is not in the set yet: NOT-RUN, never a guess.
    let v = verdict("PM-0001-30", "twin-missing");
    let part = part_of(&v, C_PART);
    assert_eq!(part.verdict, VerdictKind::NotRun, "{:?}", part.why);
    assert!(
        part.why
            .iter()
            .any(|w| w.contains("its twin c-rec1-ws is not in set c-both yet")),
        "{:?}",
        part.why
    );
}

/// What the other-transport form refuses: two measured runs on one transport, and a twin that does not name the run
/// back. Each is a gap, so the run is INVALID (as PM-0001-21's measured twin still is: it carries no transport).
#[test]
fn pm_0001_30_a_measured_twin_must_be_the_other_transport_and_name_the_run_back() {
    for (case, run, why) in [
        (
            "same-transport",
            "c-rec3-wt",
            "their params.transport are not wt and ws, one each",
        ),
        ("not-mutual", "c-rec1-wt", "it does not name this run back"),
    ] {
        let v = verdict("PM-0001-30", case);
        let part = part_of(&v, C_PART);
        assert_eq!(part.verdict, VerdictKind::Invalid, "{case}: {:?}", part.why);
        let line = format!("run {run}: a gap in the record: run {run}: ");
        assert!(
            part.why.iter().any(|w| w.contains(&line)
                && w.contains("has role measured, not twin, and")
                && w.contains(why)),
            "{case}: {:?}",
            part.why
        );
    }
}
