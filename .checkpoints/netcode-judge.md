# Checkpoint: netcode-judge
- **Goal:** the judge piece of the netcode proof test (PR #40's work), taken off the netcode chat's hands by Mustafa's
  start block of 2026-11-24 in the Netcode Builder 2 chat. Two parts, in order: **part a, MOVED** (the judge and
  marks.json report a moved pass mark, or a moved part of one, as MOVED and never as PASS; its text is his MARK-NOTES
  answer), then **part b, U6j** (the judge learns 19 figures and three waiting judge changes). Plan:
  `.checkpoints/netcode-judge/plan-a.md` (not written yet).
- **Branch:** `claude/practical-ride-igo41n` (this cloud session's own branch, the stand-in for `cs/netcode-judge`), in
  the worktree `.gate/netcode-judge`. **Base:** the netcode branch `claude/netcode-proof-test-plan-hcky5g` at
  e2c0e896dedd5c09c2536e2fbe9323353756fe99, pushed unchanged as this branch's first push (2026-11-24 19:5x UTC).
- **Last update:** 2026-11-24 20:55 Amsterdam (19:55 UTC).

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

## Current step
Part a is reported (waiting on Mustafa: OUTSIDE-NOTES, BURST-RIDER, MAIN-MODEL). **Part b, block 1's second half:**
u4s-carries.md entry 64 (a) to (f), each test shown failing under its plant and passing restored (figures-b.md C1
gives each item's test, wording, line numbers and plant): (a) the Arm test's role C and ML cases, and the
fixtures/README.md row's wording; (b) the far-better test's exact reason and the pinned probe; (c) the bite-run
practice test's case (5); (d) a coverage gap and a waiting Arm mismatch in one run reads INVALID; (e) two wordings
(eval.rs's Arm note, practice.rs's module doc); (f) `summary_line` escapes control characters, with its test. Then
commit, push, the size count from 0e0f6547, the card, and block 2 (the plumbing: `Inputs`, `Lack::Fail`, the detail
text; plan-b.md section 2 and its condition 2). Part b's board thread is open (state doing).

## Next steps
1. If Mustafa answers OUTSIDE-NOTES (a): the three notes' sentence by `moved-table.mjs` (it rebuilds from the base),
   one small commit, the checks again on the new head, the new commit id to him.
2. Part b (U6j; the stop is 4,900 added lines). Read so far: u6-brief.md sections 1, 2, 3, 6, 7, 8; figures.rs whole;
   SERIES.md's "Figures" table (:258-299) and "Series read only by figures" (:402-424); which marks name the
   figures (jq over marks.json). Its pieces:
   - **J0, found while reading, not in the hand-off's detail:** two standing items for "the judge's next change" that
     part a did not do. (1) u4s-carries.md entry 64 (:455-480): six follow-ups (a) to (f) from the advisor's U4j
     round-2 ruling, "before any other edit to crates/judge/src, and at the latest before U6 builds -19's figure",
     each shown failing under its plant; (a) is Medium (the Arm test's role C and ML cases), (f) changes
     `summary_line` (control characters escaped). (2) JDG-ONLY-1, Medium (history.md:4609-4614): `--only` narrows the
     marks before a precondition is looked up (main.rs:94; eval.rs `Rule::Pre`), so a mark with `pre:PM-0001-2`
     judged alone reads INVALID; home "npt-judge's next change, before PR #40 is marked ready at the latest". Both go
     first in part b; the advisor rules whether part a's head stands as it is (my lean: yes).
   - **J1, the three waiting judge changes** (plan.md:1254's row): C-U6-9, `twin_hash_mismatch_count` gains -30's
     form (each other's measured runs differing in `params.transport`), with fixtures and SERIES.md's row naming both
     forms; C-U6-10, PM-0006-20's logging-off twin against C4's role list and `twin_of`'s same-bite rule
     (`alloc_twin_diff_count` settles it); C-U6-11, -19's join hash under the coverage rule: how a client admitted
     mid-run starts its span (a marks/README.md rule), and whether ML's `browser` process joins the `decoded_hash`
     writers (-19's ML part is moved now).
   - **J2, 19 figures** (figures.rs's `planned(.., "U6", ..)` entries less `rtt_agreement_p99_ns`, which moved with
     PM-0001-33): sync_unit_cost, sync_cost_ratio (-10); down_client_mean_max_bits, down_mean_ipv6_bits,
     r2_down_formula_bits (-12); up_client_mean_max_bits (-13); rss_slope_bytes_per_10min, queue_growth_count (-15);
     session_memory_growth_count (-24); kill_p99_ratio (-27); bulk_control_p99_rise_ns, one_shot_dropped_count (-38);
     crash_close_max_ns (-59); egress_interface_bytes, egress_metric_vs_interface_ratio, r2_egress_formula_bytes,
     egress_vs_r2_ratio (PM-0006-17); logging_cost_ratio (PM-0006-19); alloc_twin_diff_count (PM-0006-20).
   Then `plan-b.md`, the advisor agent's review, the build in blocks (each compiling with its tests before the next).

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
- Part b's spec sheet: DONE (`.checkpoints/netcode-judge/figures-b.md`, 358 lines, test-engineer, cited). Part b's
  plan: `.checkpoints/netcode-judge/plan-b.md`, with the advisor agent for its plan review since 2026-11-24 about
  21:22 UTC (six rulings asked; no part b edit before its APPROVE). On its return: its verdict and conditions into
  plan-b.md's end, then block 1 (J0: entry 64's six items and JDG-ONLY-1). Two questions for the netcode chat are on
  the board (the kill record's name and fields; `room_end`'s `slot` and the room log's `admit` and `session_end`).
- The review round: test-engineer and rust-reviewer on b22c30cd, 2026-11-24 about 20:25 to 20:40 UTC: DONE.
- The two defects that survived the test-engineer's planting are now caught, each run once and taken back: a moved
  check kept on a bite run that matches no part fails `a_moved_check_is_skipped_wherever_checks_are_chosen`; a bite
  run judged with its moved part's checks too fails `a_bite_run_on_moved_parts_only_is_left_out`.
- The background saver (`checkpoint.mjs watch netcode-judge`, every 10 minutes for 110) since about 20:30 UTC, for
  the reviewers' notes; it stops by 22:20 UTC and is checked at each card update.
- GitHub on b22c30cd: the gate workflow green (run 45480243854); the toolchain workflow running (run 45480243852).
- `.checkpoints/netcode-judge/moved-table.mjs` was run once with `--write` at the base's marks.json (it refuses any
  other file): 26 marks whole, 21 parts, 2 checks, 1 bite, -56's kept check, -7's and -56's notes.
