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

## Current step
Part a's one review round on b22c30cd: test-engineer and rust-reviewer, both at once, at xhigh; their notes in
`.checkpoints/netcode-judge/test-engineer-a.md` and `rust-reviewer-a.md`.

## Next steps
1. One fix pass on the round's findings (a Low never triggers a push on its own); the size count again.
2. On the final head: `cargo test -p npt-judge --locked`, clippy, fmt, then `node spikes/netcode-proof/check.mjs`
   whole; GitHub's gate and toolchain results on that full SHA (pushed alone, no skip marker); the advisor's read.
3. The report to Mustafa with the commit id (plan-a.md section 6, conditions 10 to 12, say what it must hold); the
   board thread's ready line.

## Open findings
- none yet

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
- The review round: test-engineer and rust-reviewer on b22c30cd, launched 2026-11-24 about 20:25 UTC (2 agents).
- The background saver (`checkpoint.mjs watch netcode-judge`, every 10 minutes for 110) since about 20:30 UTC, for
  the reviewers' notes; it stops by 22:20 UTC and is checked at each card update.
- GitHub on b22c30cd: the gate workflow green (run 45480243854); the toolchain workflow running (run 45480243852).
- `.checkpoints/netcode-judge/moved-table.mjs` was run once with `--write` at the base's marks.json (it refuses any
  other file): 26 marks whole, 21 parts, 2 checks, 1 bite, -56's kept check, -7's and -56's notes.
