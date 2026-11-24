# Checkpoint: netcode-judge
- **Goal:** the judge piece of the netcode proof test (PR #40's work), taken off the netcode chat's hands by Mustafa's
  start block of 2026-11-24 in the Netcode Builder 2 chat. Two parts, in order: **part a, MOVED** (the judge and
  marks.json report a moved pass mark, or a moved part of one, as MOVED and never as PASS; its text is his MARK-NOTES
  answer), then **part b, U6j** (the judge learns 19 figures and three waiting judge changes). Plans:
  `.checkpoints/netcode-judge/plan-a.md` and `plan-b.md`.
- **Branch:** `claude/practical-ride-igo41n` (this cloud session's own branch, the stand-in for `cs/netcode-judge`), in
  the worktree `.gate/netcode-judge`. **Base:** the netcode branch `claude/netcode-proof-test-plan-hcky5g` at
  e2c0e896dedd5c09c2536e2fbe9323353756fe99, pushed unchanged as this branch's first push (2026-11-24 19:5x UTC).
- **Last update:** 2026-11-24 about 23:35 UTC, both parts done at 72651a3c and reported.

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

## Both parts, for whoever merges them (the netcode chat, in one merge, only on Mustafa's word, after his main Advisor checked the final commit)
- **The commit:** 72651a3c6326546b423c4773ac5a92902a37b590 (commits after it on this branch touch only
  `.checkpoints/netcode-judge*`). From the netcode head e2c0e896 it changes only `spikes/netcode-proof/crates/judge/`
  and `spikes/netcode-proof/marks/` (24 files, 4,712 added, 208 deleted); no manifest or lockfile.
- **To repeat the checks, from `spikes/netcode-proof/`:** `cargo test -p npt-judge --locked --no-fail-fast` (132 pass:
  19 + 4 + 83 + 26); `cargo clippy -p npt-judge --all-targets --locked -- -D warnings`; `cargo fmt --check`;
  `cargo run -q --locked -p npt-judge -- --marks marks/marks.json --check-marks` (81 marks); from the repository's
  root `node spikes/netcode-proof/check.mjs` (11 of 11 steps).
- **What changes for callers:** a new verdict word MOVED (a mark's and a part's), a bite status `moved`, optional
  `moved` fields in the `--out` JSON; the three `outside` marks read MOVED. `--only` judges the named marks against
  the whole marks file (JDG-ONLY-1 is fixed: the interim rule of naming PM-0001-2 beside a mark is no longer needed).
  The summary line escapes control characters. Exit codes and the `npt-judge: ` error lines are as before.
- **marks.json:** sha256 c3396db225b394695b6cf41460723e1d98b96f42e1d42d412976f4ca702f1b7a. An older judge refuses
  it (unknown field `moved`): judge and marks.json merge together. `marks/README.md` 2.6 is the contract.
- **SERIES.md, for the merge with U6c's 51 rows:** 3 new rows (`event.admit` with `event.session_end`;
  `event.room_end`'s slot), the heading "Series read only by figures (21)" (was 19), the 19 figures' rows and
  `decoded_hash_mismatch_count`'s coverage clause reworded, `tick.s.relevant_count` and `event.iface.tx_bytes` changed.
- **Mustafa's message of 2026-11-24** is saved word for word in `.checkpoints/netcode-judge/mustafa-2026-11-24.md`;
  his decisions belong in `docs/DECISIONS.md` (outside this chat's fence: the merging chat or docs-keeper carries it).
- **The burst setting (NPT_KC_BURSTS):** the netcode chat builds it after R1's run and before R1's report (his item 2).
- **Before the merge:** the re-judge of the -47 set on real data (under "Open findings"); his answer to SYNC-ENTITIES
  is needed before R2's -10 runs are judged, not for the merge.

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

16. Part b, block 4 (C-U6-11) · d8d4ff338fd3a0121a5262a192a4ef747c2fb9de (pushed) · figures.rs: `session_ticks`,
    `Session`, `sessions`, `coverage` per session (cut at the room log's `admit` records; a session before the slot's
    last may leave a tail only up to its recorded `session_end`, read from the room's or the lab's log; a slot with no
    admit keeps its single span; no measured room hash before a slot's first admit). Engine tests
    `c_u6_11_coverage_runs_per_session_cut_at_the_admits` and
    `c_u6_11_a_mid_run_join_and_the_slots_with_no_admit_record`; six plants, each failing one and taken back. Every
    PM-0001-17 fixture reads as before. README section 1, SERIES.md's coverage clause and one new SERIES.md row
    (`event.admit`, `event.session_end`; the "read only by figures" count is now 20). 1,021 added so far.
    (Its last-session rule was replaced by the advisor agent's ruling in the fix pass: step 23.)

17. Part b, block 5 · 8735bc5ff66cb517088e2a62c27add169373afd6 (pushed) · `down_client_mean_max_bits`,
    `up_client_mean_max_bits` (`slot_mean_max`), `down_mean_ipv6_bits`, `bulk_control_p99_rise_ns`,
    `one_shot_dropped_count`; helpers `room_log`, `only_from`, `room_seconds`, `room_lists`, `logs_of`,
    `second_total`, and `built(..)` for the table. Four engine tests (`u6j_...`); eight plants, each failing one and
    taken back. SERIES.md's four rows. 110 judge tests pass (17 + 4 + 64 + 25). 1,450 added, 105 deleted so far.

18. Part b, block 6 · 46581724e8cf60eadad58a2a5b4f8079f1025976 · `rss_slope_bytes_per_10min`, `growth_count`
    (`queue_growth_count`, `session_memory_growth_count`; windows by time); 3 tests, 8 plants.
19. Part b, block 7 · 3c42524f85aca1ea672975b2a1a87f5ff34cd633 · `sync_unit`, `sync_cost_ratio`; `Parts` now maps a
    part to (its min, its valid runs); **found:** comparing two such ratios overflowed i128, so `Ratio`'s `Ord` falls
    back to `cmp_wide` (Euclid's steps), with its own test; 2 tests, 7 plants.
20. Part b, block 8 · a6d640646bc4e0b525f55a69143f85774ccb2aab · `R2Counts` (the formula as the netcode-architect
    ruled its inputs: `netcode-architect-r2-inputs.md`; the soldier term is on E - V, golden E is 62), the two formula
    figures with their detail text, `counter_growth`, the three egress figures; 2 tests, 8 plants.
21. Part b, block 9 · 9380e202948ab4317d91c953340736fa1a264480 (`crash_close_max_ns` with the exact failure line,
    `logging_cost_ratio`, the Rule::LoggingOff fix; 3 tests, 8 plants) and
    a3476e6c87701b00f70e811f9e7d3549649d382b (`kill_p99_ratio` last, the texts; 1 test, 4 plants). **All 19 figures
    built.** 122 judge tests pass (18 + 4 + 75 + 25); fmt, clippy and check.mjs's seven quick steps pass. From
    0e0f6547: 2,915 added, 166 deleted, 22 files, only `crates/judge/` and `marks/` (marks.json untouched).

22. Part b's one review round on a3476e6c · both REVISE on one High (`rust-reviewer-b.md`, `test-engineer-b.md`):
    `sync_cost_ratio` stopped the whole judge with an i128 overflow at four or more valid reference runs.
    test-engineer: 4 Medium (M1 to M3 unpinned logic, with probes; M4 a ruling), 9 Low; rust-reviewer: 11 Low.
    `check.mjs` whole on a3476e6c: 11 of 11 (844 and 941 tests).
23. The advisor agent's rulings for the fix pass (2026-11-24): the High by option (A): the mean stays over the part's
    `min` runs, more is a gap, an overflow is a gap, never a stop; C-U6-11's last session MAY leave a tail up to its
    recorded `session_end`; the bottom of -10's unit cost is Mustafa's named decision SYNC-ENTITIES (the architect's
    meaning stands meanwhile).
24. The fix pass · 7c3855c755076de3728bf522abad7fa2eedd77f8 (pushed) · the High, the Mediums' tests (all ten of the
    test-engineer's surviving plants now fail a test), the Lows taken (see the commit message). 130 judge tests
    (19 + 4 + 81 + 26). From 0e0f6547: 3,329 added, 167 deleted. Not taken, follow-ups (Low): rust-reviewer's L7
    (a `Lack::Fail` reads "= ?" in the precondition and setup lines: not reachable today), L9 (visibility), L6's
    enum and `only_from`'s scan cost; test-engineer's L9 (wording of condition 5's bracket).
25. Mustafa's message of 2026-11-24 (saved word for word by script: `mustafa-2026-11-24.md`, `save-message.mjs`) ·
    67892d3966905ec3d6c38a64de92b18e93d43d8a (pushed) · his item 1 (the three outside marks' notes say "Always
    MOVED here.") and item 3 (the two rough-laptop parts move by his decision; their `by` names his message) by
    `moved-table.mjs`: five lines of marks.json, sha256 now
    c3396db225b394695b6cf41460723e1d98b96f42e1d42d412976f4ca702f1b7a. His item 2: the netcode chat builds the burst
    setting (NPT_KC_BURSTS) after R1's run and before R1's report. His item 5: the netcode chat may take both parts
    in one merge, only on his word, after his main Advisor checked the final commit.

26. The reviewers' re-check of the fix pass (on what changed only) · both APPROVE WITH FINDINGS, the High closed;
    the rust-reviewer confirmed no other figure can overflow the same way; the test-engineer raised two Mediums.
27. After the re-check · 72651a3c6326546b423c4773ac5a92902a37b590 (pushed) · both Mediums: the later-session hole
    test (`c_u6_11_a_hole_inside_a_later_session_is_a_gap`), and by the advisor agent's ruling a `session_end` at or
    after the room log's last tick record gives no tail (`c_u6_11_a_session_end_at_the_stop_gives_no_tail`; two
    plants). 132 judge tests (19 + 4 + 83 + 26). From 0e0f6547: 3,393 added, 172 deleted (the stop is 4,900).

28. The final head's checks · 72651a3c6326546b423c4773ac5a92902a37b590 · the advisor agent: APPROVE for this head
    (2026-11-24 about 23:20 UTC; all 11 conditions and its later rulings held; the fence held; the small commit
    67892d39 as Mustafa asked). `node spikes/netcode-proof/check.mjs` whole in this container (ended about 23:30
    UTC): all 11 steps pass; test: measured build 854 passed, bites build 951 passed, 0 failed, 0 skipped (part a's
    head: 820 and 917; +34 each); marks: 81 read, rows unchanged; self 78 of 78; redact 11 of 11, 0 findings. The
    judge's own commands on the same tree: 132 tests (19 + 4 + 83 + 26), clippy and fmt exit 0, --check-marks 81.
    GitHub on that SHA: the gate workflow (jobs gate and smoke) success, run 45508565066; the toolchain workflow
    success, run 45508565097. Part b's commit id reported to Mustafa.

## Current step
**Both parts are done and reported: the commit is 72651a3c6326546b423c4773ac5a92902a37b590.** Waiting on Mustafa:
his word for the merge (the netcode chat merges, after his main Advisor's check), SYNC-ENTITIES and MAIN-MODEL.
Nothing is running but the background saver.
**For the report (collected while building):**
- Condition 4, readings still open (built on the written reading): -12 and -13 "per client" = the largest client
  mean; -15's slope method and "flat" rule; -24's halves; -27's "within 5 %" = not worse by more than 5 %;
  PM-0006-19's pooled comparison; -10's mean of the 3 runs and what its "relevant entities" count (SYNC-ENTITIES;
  the test-engineer's numbers: 1.25 against 0.80).
- Condition 7: C4-KILL (the killed-room record check) is not built: the netcode chat named no kill record. Today any
  log that says `killed` is exempt from `complete`. Home: before R5's -27 and PM-0006-22 runs. README section 4 says so.
- Condition 9: the Rule::LoggingOff fix is outside the U6j list. Also outside it: `Ratio`'s exact order at any size.
- Condition 11: no writer logs 16 of the input series; every figure is proven on made-up logs only. The netcode chat
  says (board) its later units write `admit`, `session_end` and `room_end` on L2 per SERIES.md's rows, after R1.
- The ordering miss of part a (condition 1): entry 64 and JDG-ONLY-1 landed first in part b, not before part a.
- My own readings: `logging_cost_ratio` takes exactly 3 logging-off runs; a figure that reads the room needs one
  room log; -10's reference parts are found by their names `N=24` and `N=36`.
- Writer-side needs now in SERIES.md (posted on the board): `room_end` carries `slot`; on L2 `admit` and
  `session_end` in the room's log; the two `iface` records carry `ns`; `relevant_count`'s meaning; the kill at the
  end of the measured window.
- One review round as he asked, plus a re-check of the one High's fix by the two reviewers (both had said REVISE).
- Two edges of the tail rule left open (the advisor: for U6d's writer brief and the readings change before R5): a
  writer that ends sessions one tick before the room's last tick record; the tail has no length limit (a principled
  fix: `session_end` carries a reason, and only a scripted leave gets the tail).

## Next steps
1. Only on Mustafa's word: an answer to SYNC-ENTITIES may change `tick.s.relevant_count`'s row or add a count (a
   small change-set of its own); MAIN-MODEL; the one-way delay comparison on a real network (NPT-L2-ONEWAY), not
   started.
2. The netcode chat merges this branch into its own (both parts in one merge), on his word.

## Open findings
- Follow-ups, Low, none needed before the merge (the advisor agent's read): part a's rust-reviewer L4 and L6 and
  test-engineer L7; part b's rust-reviewer L6 (`only_from`'s kind as an enum; its scan cost), L7 (a failure with no
  number reads "= ?" in the precondition and setup lines: not reachable today), L9 (visibility), two Info notes;
  test-engineer's L9 (wording).
- Not verified here: the re-plan's real-data line (re-judging `results/u4f/b57221ed139d/pm-0001-47/`): the raw logs
  are not in the repository. For the netcode chat before the merge, from `spikes/netcode-proof/`:
  `cargo run -q --locked -p npt-judge -- --marks marks/marks.json --logs <the -47 set's logs> --only PM-0001-47`,
  expected MOVED.
- With Mustafa: SYNC-ENTITIES (what -10's "relevant entities" count; the advisor agent recommends soldiers plus
  hulls, as built) and MAIN-MODEL (this chat runs Fable 5.2 by his start block; CLAUDE.md says Opus 5.5).

## Key numbers
- The baseline, `node spikes/netcode-proof/check.mjs` on the base e2c0e896 in this container: 11 of 11 steps;
  807 tests on the measured build, 904 on the bites build. Part a's head 0e0f6547: 820 and 917; 98 judge tests.
  Part b's built head a3476e6c: 844 and 941; 122 judge tests. The final head 72651a3c: 132 judge tests.
- marks.json sha256: c1935a5b… at the base; 86705adc… at part a's head; c3396db225b394695b6cf41460723e1d98b96f42e1d42d412976f4ca702f1b7a now.
- Planted defects: part a's two, part b's per block (in each commit message), the fix pass's, and the reviewers' own
  (`te-plant.mjs`, `te-plant-b.mjs`, `te-plant-b2.mjs`); every one ran once and was taken back.

## Decisions taken
- Mustafa, in the chat, 2026-11-24: his four first answers (checks in the container; 19 figures; the first push as
  the netcode head; the main folder parked); and his message saved in `netcode-judge/mustafa-2026-11-24.md`
  (OUTSIDE-NOTES (a); BURST-RIDER (a); the rough-laptop parts move by his decision; one small commit read by the
  advisor agent; both parts in one merge on his word after his main Advisor's check).

## Running work
- Nothing but the background saver (`checkpoint.mjs watch`).
