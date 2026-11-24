# Checkpoint: netcode-judge
- **Goal:** the judge piece of the netcode proof test (PR #40's work), taken off the netcode chat's hands by Mustafa's
  start block of 2026-11-24 in the Netcode Builder 2 chat. Two parts, in order: **part a, MOVED** (the judge and
  marks.json report a moved pass mark, or a moved part of one, as MOVED and never as PASS; its text is his MARK-NOTES
  answer), then **part b, U6j** (the judge learns 19 figures and three waiting judge changes). Plans:
  `.checkpoints/netcode-judge/plan-a.md` and `plan-b.md`.
- **Branch:** `claude/practical-ride-igo41n` (this cloud session's own branch, the stand-in for `cs/netcode-judge`), in
  the worktree `.gate/netcode-judge`. **Base:** the netcode branch `claude/netcode-proof-test-plan-hcky5g` at
  e2c0e896dedd5c09c2536e2fbe9323353756fe99, pushed unchanged as this branch's first push (2026-11-24 19:5x UTC).
- **Last update:** 2026-11-24, block 3 of part b complete (d98b0024).

## Rules in force (my summary; his words are in the Netcode Builder 2 chat, 2026-11-24, never retyped here)
- **Fence:** edits only in `spikes/netcode-proof/crates/judge/`, `spikes/netcode-proof/marks/` and these notes. In
  `marks/SERIES.md` only the rows my figures need. No pass mark and no number in marks.json changes, beyond what his
  MARK-NOTES answer allows (replan.md:106-111 on the netcode branch). No manifest or lockfile change: a needed
  dependency is a stop. Nothing under `.github/`, `.claude/`, `CLAUDE.md` or `tools/`.
- **Git:** never push to the netcode branch or any branch but this one; no pull request (the netcode chat merges this
  branch on his word); never forced. The main folder stays parked on the local name `park/main-folder` at main 1ec6f6c.
- **How:** read, a short plan, the advisor agent's APPROVE, then build. Every computation gets pass, fail and gap tests
  on made-up logs. One review round: test-engineer and rust-reviewer read every judge line. Agents on Opus 5.5 at the
  kit's effort (xhigh for netcode), at most 2 at a time. One resume card, no copies of rulings.
- **Checks (his answer, 2026-11-24):** the judge's tests, its lints and `spikes/netcode-proof/check.mjs` run in this
  container on each part's final head, reported with the full commit id and the exact commands; GitHub's two checks
  only have to stay green; no workflow file. The test-engineer reviewer runs the tests itself.
- **Size stops:** part a at 1,440 added lines; part b at 4,900. At a stop: cut, and three lines to him.
- **Not started, only on his word:** the one-way delay comparison on a real network (NPT-L2-ONEWAY).
- **Part b's count (his answer):** 19 figures (replan-units.md:63); the figure of the moved mark PM-0001-33 moves.
- The board add-on is active.

## Steps done
1. The worktree and the first push · e2c0e896 (the netcode head, unchanged) · done.

2. The reading and part a's plan · `.checkpoints/netcode-judge/plan-a.md` · baseline: the judge's 85 tests pass at
   the base (`cargo test -p npt-judge --locked`, 14 + 2 + 49 + 20).

3. The advisor agent's plan review · APPROVE with 12 conditions (plan-a.md, section 6) · two questions for Mustafa
   that do not stop the build: OUTSIDE-NOTES (the three outside marks' notes sentence) and BURST-RIDER (a test
   setting in crates/net that an earlier ruling put in MOVED's brief; outside this fence).

4. Part a's build · b22c30cd7fa35e02d4bbcfb64f2db4c50cb0488f (pushed, no skip marker) · 9 files, 1,232 added and 36
   deleted against e2c0e896 (the stop is 1,440); 98 judge tests pass (85 before; 3 changed, 13 new); fmt, clippy with
   warnings as errors and check.mjs's nine quick steps pass; marks.json's sha256 is now 7f148c3e….

5. Part a's one review round on b22c30cd · both APPROVE WITH FINDINGS, no Critical or High · test-engineer: 4 Medium,
   7 Low (`test-engineer-a.md`); rust-reviewer: 7 Low (`rust-reviewer-a.md`). The advisor agent's ruling for the fix
   pass: PM-0001-3's bite `not-net-of-base` is labelled moved with its part; it and the two rough twins carry a `by`
   that says it is the advisor agent's reading, not a line of the re-plan's table.
6. The fix pass · 0e0f654776fc0e10e153875d59b33869e6179a88 (pushed alone, no skip marker) · every Medium and the cheap
   Lows taken; 1,321 added and 38 deleted against e2c0e896 (the stop is 1,440); 98 judge tests pass, fmt and clippy
   clean; marks.json's sha256 is 86705adc5f120747e97ecf5ef6eef7250983a838fe031772161215144edcfee4 (before:
   c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9); labels: 26 marks whole, 21 parts, 2 checks, 2
   bites; the test-engineer's own diff script on base and head: "rest equal after strip and undo: true".

7. The advisor agent's read of the final head · APPROVE for 0e0f654776fc0e10e153875d59b33869e6179a88 (2026-11-24,
   about 20:47 UTC) · all 12 conditions and both fix-pass rulings hold; it checked the fence, the judge's tests in its
   own build, marks.json by its own script, and GitHub's gate, smoke and toolchain (all success on 0e0f6547). Two
   things before the commit id goes to Mustafa: check.mjs whole passes on this head, and the report names the main
   session's model (it is Fable 5.2, by his start block; CLAUDE.md's Models line says Opus 5.5: his to settle).

8. The final head's checks · 0e0f654776fc0e10e153875d59b33869e6179a88 · `node spikes/netcode-proof/check.mjs` whole
   in this container (2026-11-24, ended 20:56 UTC): all 11 steps pass; test: measured build 820 passed, bites build 917
   passed, 0 failed, 0 skipped (the base: 807 and 904; +13 each); marks: 81 read, rows unchanged; self 78 of 78;
   redact 11 of 11, 0 findings. GitHub on that SHA: the gate workflow's jobs gate and smoke success (run 45483689493),
   the toolchain workflow success (run 45483689440). Part a's commit id reported to Mustafa.

## Part a, for whoever merges it (the netcode chat, on Mustafa's word only)
- **The commit:** 0e0f654776fc0e10e153875d59b33869e6179a88 (commits after it on this branch touch only
  `.checkpoints/netcode-judge*`). It changes only `spikes/netcode-proof/crates/judge/` and `spikes/netcode-proof/marks/`
  (9 files); no manifest or lockfile; `marks/SERIES.md` untouched.
- **To repeat the checks, from `spikes/netcode-proof/`:** `cargo test -p npt-judge --locked --no-fail-fast` (98 pass:
  17 + 3 + 55 + 23); `cargo clippy -p npt-judge --all-targets --locked -- -D warnings`; `cargo fmt --check`;
  `cargo run -q --locked -p npt-judge -- --marks marks/marks.json --check-marks` (81 marks); from the repository's
  root `node spikes/netcode-proof/check.mjs` (11 of 11 steps).
- **What changes for callers:** a new verdict word MOVED (a mark's and a part's), a bite status `moved`, and optional
  `moved` fields in the `--out` JSON (`moved` on a verdict, a part and a bite; `moved_checks` on a verdict). Exit
  codes and the `npt-judge: ` error lines are as before. The three `outside` marks read MOVED, not NOT-RUN.
- **marks.json:** sha256 86705adc5f120747e97ecf5ef6eef7250983a838fe031772161215144edcfee4. An older judge refuses
  it (unknown field `moved`): judge and marks.json merge together. `marks/README.md` 2.6 is the contract.
- **Before the merge:** the real-data line under "Open findings"; Mustafa's answer to OUTSIDE-NOTES (on his (a), one
  more small commit lands here first).

9. Part b's plan and its review · `plan-b.md` on `figures-b.md` (the spec sheet) · the advisor agent: APPROVE with 11
   conditions (plan-b.md section 5); part a's head stands; the board has the two questions for the netcode chat.

10. Part b, block 1's first half: JDG-ONLY-1 · ab1074e367ae6928c1774bceb3c9285732dfa45a (pushed) · `judge_only`
    judges the named marks against the whole file; main.rs no longer narrows it; the cli test
    `only_keeps_a_marks_precondition_in_reach` (real marks.json, PM-0001-4's base fixture) fails with the old
    narrowing planted back and passes restored; 99 judge tests pass; part b so far 32 added lines (from 0e0f6547).

11. Part b, block 1: entry 64 (a), (e), (f) · cb34ba5e8527579dfca3ed6879a2d7e317266538 (pushed) · (a) the Arm
    test's role C and ML cases and the fixtures README row (planted: the LG key dropped in figures.rs, the Arm test
    fails); (e) the two wordings; (f) `summary_line` escapes control characters, test
    `the_summary_line_escapes_control_characters` (planted: the escape off, it fails; npt-log's writer refuses a
    control character in a run id, so the test sets the reason by hand); 100 judge tests pass; part b so far 82 added.

12. Part b, block 1 complete (JDG-ONLY-1 and entry 64 (a) to (f)) · 0d31500e00230f062b6381bb7f4127d867034c23
    (pushed; before it 46c79d4b (d) and 0534800c (b)) · each item's test failed under its plant and passed restored:
    (b) the pin off fails `u4j_ws_far_better_past_the_cap_is_invalid` at its pin assertion, the band rule off fails
    it at the pinned probe; (c) the "practice runs … in force" refusal off fails
    `u4j_ws_a_bite_run_takes_its_counted_runs_practice_run` at case (5); (d) the gap and waiting checks swapped fails
    the Arm test. 100 judge tests pass, fmt and clippy clean; part b so far 145 added, 10 deleted (from 0e0f6547; the
    stop is 4,900).

13. Part b, block 2 (the plumbing) · 24c3ba21fb060da40695b4e44e1711f297b546d2 (pushed) · as plan-b.md's "Block 2's
    design": `figures::Inputs { run, set, parts }`, `Parts`, `Lack::Fail` and `Missing::Fail`, `Figure.detail` with
    `figures::detail`, `Ctx.parts`, the first pass in `judge_mark` (only for a mark that names a figure; a run is
    valid when its validity holds with nothing undecided and its part's setup checks pass), `fail_lines`' reason
    line, the report's detail text. No test file changed: all 100 judge tests pass as before; fmt and clippy clean.
    Part b so far 287 added, 34 deleted (from 0e0f6547; the stop is 4,900). Still owed by later blocks: the exact-line
    test for `Lack::Fail` (condition 10, with `crash_close_max_ns`), the detail text's test (with the R2 formula),
    `Inputs.parts`' tests (with `sync_cost_ratio` and `logging_cost_ratio`).

14. Part b, block 3's code · e01e681560cbcd30d6ee93c5267eddb9969bf494 (pushed) · in figures.rs:
    `other_transport_twin` (C-U6-9: called by `twin_hash_mismatch_count` when the twin is not role `twin`; both
    measured, the twin's `params.twin` names the run back, `params.transport` wt and ws one each, else a gap whose
    text keeps "has role …, not twin"); `twin_of` passes `every_run_rules(twin, roles)` with `logging-off` added
    only when the run is role `bite` and the twin role `logging-off` (C-U6-10; eval.rs: `always_rules` =
    `every_run_rules(run, &["bite", "twin"])`); `alloc_counts` and `alloc_twin_diff_count` (room logs, measured
    ticks, gaps for no room log, a tick without the count, two logs at one tick; empty when neither run has a
    measured tick), its table entry built (unit "U6j"). The 100 existing tests pass.

15. Part b, block 3 complete · d98b00244fc3e268fa3f573418a2bb4fd62ad411 (pushed; before it bc1efd7c, the engine
    tests) · engine.rs: `alloc_twin_diff_count_compares_the_two_runs_tick_by_tick` and
    `a_bite_runs_logging_off_twin_may_name_its_bite`; fixtures.rs with the real marks.json and ten made-up logs under
    `crates/judge/fixtures/pm-0001-30/` (written by `gen-30-fixtures.mjs`):
    `pm_0001_30_each_run_is_compared_with_its_other_transport_twin` and
    `pm_0001_30_a_measured_twin_must_be_the_other_transport_and_name_the_run_back`. Four plants, each failing its
    test and taken back: one-sided ticks dropped; the logging-off exemption widened; the transport check off; the
    name-back check off. SERIES.md's two rows, README section 4's build rule and the fixtures README say it. 104
    judge tests pass (17 + 4 + 58 + 25); fmt, clippy and check.mjs's seven quick steps pass. Part b so far 713 added,
    48 deleted (from 0e0f6547; the stop is 4,900).

## Current step
Part a is reported (waiting on Mustafa: OUTSIDE-NOTES, BURST-RIDER, MAIN-MODEL). **Part b, block 4 (C-U6-11):** the
coverage rule's spans per session, cut at the room log's `admit` events (figures-b.md B3; plan-b.md condition 5: a
slot with no `admit` keeps today's single span; a run with no `session_end` keeps today's verdict). Every PM-0001-17
fixture keeps its verdict. README section 1's warm-up sentence and SERIES.md's coverage clause are reworded. The two
writer-side needs go on the board first (condition 8). Then blocks 5 to 9 as plan-b.md section 2 and the verdict's
last paragraph order them; before block 8, one netcode-architect agent rules R2-INPUTS (condition 3).

## Next steps
1. If Mustafa answers OUTSIDE-NOTES (a): the three notes' sentence by `moved-table.mjs` (it rebuilds from the base),
   one small commit, the checks again on the new head, the new commit id to him.
2. Part b's blocks 4 to 9 (plan-b.md section 2 and its 11 conditions; figures-b.md is the spec sheet), then one
   review round (test-engineer and rust-reviewer, xhigh), one fix pass, the final head's checks, the advisor agent's
   read, and part b's commit id with the report to Mustafa.

## Open findings
- Left as follow-ups from part a's round (Low, no fix now): rust-reviewer's L4 (a kept bite whose only runs were left
  out reads "no run yet", though runs exist: it needs a field on `BiteOut`) and L6 (a kept mark reading MOVED lists
  its moved parts' lines after the summary line, but not its moved checks' or bites': the summary line and the JSON
  carry them); test-engineer's L7 (the 12 real kept marks with a moved piece are proven never-PASS by the rule's
  order and the two-mark enumeration, not by passing logs for each of the 12: said so in the report).
- Not verified here: the re-plan's real-data line (re-judging `results/u4f/b57221ed139d/pm-0001-47/`): the raw logs
  are not in the repository. For the netcode chat before it merges part a, from `spikes/netcode-proof/`:
  `cargo run -q --locked -p npt-judge -- --marks marks/marks.json --logs <the -47 set's logs> --only PM-0001-47`,
  expected MOVED.
- Not carried by part a: the NPT_KC_BURSTS test rider (crates/net/src/tcp_model/tests.rs, outside this fence; an
  earlier ruling put it in "MOVED's brief": advisor-u8-brief.md:106-113). BURST-RIDER is with Mustafa.
- With Mustafa: OUTSIDE-NOTES (the three outside marks' notes still end "Always NOT-RUN here." while they read MOVED).

## Key numbers
- The judge today: src 5,238 lines (eval.rs 1,784; marks.rs 1,263; figures.rs 1,020; practice.rs 797), tests 3,618,
  marks.json 12,630 lines, 81 marks (wc -l at e2c0e896).
- The baseline, `node spikes/netcode-proof/check.mjs` on the base's files in this container (2026-11-24, 975 s): all
  11 steps pass; test: measured build 807 passed, bites build 904 passed, 0 failed, 0 skipped; self 78 of 78; redact
  11 of 11, 359 items, 0 findings; marks: 81 read, rows unchanged.
- Part a's two planted defects, each run once and taken back (`cargo test -p npt-judge --locked --no-fail-fast`):
  (A) a moved mark judged anyway fails `a_mark_moved_whole_is_never_judged`,
  `moved_marks_and_parts_show_in_the_summary_and_the_json` and `the_real_marks_file_labels_what_the_trim_moved`;
  (B) MOVED ranked above FAIL fails `a_moved_part_is_never_judged_and_the_kept_parts_still_decide`,
  `a_moved_check_is_skipped_wherever_checks_are_chosen`, `a_moved_bite_is_never_evaluated`,
  `a_bite_run_on_moved_parts_only_is_left_out`, `a_moved_bite_hides_no_kept_failure_or_invalid_run` and
  `a_label_anywhere_on_a_passing_mark_never_reads_pass`.
- Existing assertions changed by part a (old, new): cli.rs, the outside mark's line "PM-0001-39   NOT-RUN   outside
  this change-set" is now "PM-0001-39   MOVED     outside this change-set"; engine.rs `person_and_outside_marks`, the
  outside mark's verdict NOT-RUN is now MOVED; fixtures.rs, `pm_0006_14_counts_a_padded_session_and_waits_for_the_
  other_n` (NOT-RUN, "N=36: 0 of 1 valid runs", "N=18: 0 of 1 valid runs") is now `pm_0006_14_counts_a_padded_
  session_and_its_other_n_moved` (MOVED, both parts named with "slice 6, on the real sim", N=48's count reported).

## Decisions taken
- His four answers of 2026-11-24 (checks in the container; 19 figures; the first push as the netcode head; the main
  folder parked) · Mustafa · in the chat.

## Running work
- Nothing in the background but the saver (`checkpoint.mjs watch netcode-judge`), which stops by about 22:20 UTC.
- Two questions for the netcode chat are on the board, unanswered so far (the kill record's name and fields;
  `room_end`'s `slot` and the room log's `admit` and `session_end`).
