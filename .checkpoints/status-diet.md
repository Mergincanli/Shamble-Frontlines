# Checkpoint: status-diet
- **Goal:** Mustafa's STATUS-DIET (STATUS <= 120 lines; DECISIONS.md his decisions word for word, newest first;
  STATUS_ARCHIVE.md the rest; moved by a line-range script; decision-id counts before/after; REPO_MAP and CLAUDE.md's
  read list) with LANE-RULES (lanes, ACCOUNTS, the Models line and its addition, the LOOK_V2 read-list line); record
  #46's merge (main 2d22b69). His words: main STATUS:674-761 and docs/reports/2026-11-10-kit-v3-5.md Appendix A.
  Plan: `.checkpoints/status-diet/plan-rev1.md` (next).
- **Branch:** `claude/busy-turing-48e4i9` (stands in for `cs/status-diet`) · **Base:** main 2d22b69
- **Last update:** 2026-11-10 21:07 Amsterdam

## Steps done
1. Facts gathered · 9e33794 · `.checkpoints/status-diet/facts.md`: line ranges and destinations, live PR and
   routine states, kit references, the LANE-RULES edit shape.
2. Plan revision 1 · 2f9d467 · `.checkpoints/status-diet/plan-rev1.md`.
3. The advisor's plan review · (this commit) · APPROVE with conditions 1-12, rulings Q1-Q5
   (`advisor-plan-rev1-verdict.md`; its notes `advisor-plan-rev1.md`, DONE); no new revision needed.

## Current step
Build step 1: templates + split.mjs + check.mjs in the notes folder, dry run into .gate/status-diet/out (no docs/
edit yet). Waiting in parallel: the advisor's ruling on his #46 go words (2d22b69's message: "The open security
follow-ups stay in STATUS until the foundation's gate step").

## Next steps
1. Dry run; then the pure-move commit (docs/STATUS.md, DECISIONS.md, STATUS_ARCHIVE.md; E1 on it alone, cond. 11).
2. The record-#46 commit (archive's kit-v3-5 row as a diff; STATUS; ROADMAP); his #46 go in DECISIONS.md (copied
   from the commit message by script) once the advisor rules.
3. Docs: REPO_MAP, START_HERE:20, TECHNICAL:240. 4. Kit writes, one ask each: CLAUDE.md (cond. 3, 6, 7, 8),
   docs-keeper.md (cond. 4). 5. Gate, E1-E6, kit tests on main and head (cond. 10). 6. PR (draft), review round.

## Open findings
- Night shift cron is daily 06:00 Amsterdam since 2026-11-07, STATUS says hourly: record the read state.
- PR #40's branch holds his 2026-11-06 answers not on main: tell its lane (report), never push to it.

## Key numbers
- STATUS at 2d22b69: 1233 lines, 177445 bytes.

## Decisions taken
- Split first from 2d22b69 (pure move, provable), then record #46 in a separate small commit · the session · 11-10.

## Running work
- advisor plan review of rev 1 · background agent · notes `.checkpoints/status-diet/advisor-plan-rev1.md` · launched
  17:33 Amsterdam; cut off by the usage limit (spend limit, reset 20:00 UTC); resumed 21:02 Amsterdam from its notes
  (no block done yet) · running.
- saver: background task b162m354f · output /tmp/claude-0/-home-user-Shamble-Frontlines/7c0a3d46-1eae-5891-9697-3fff27d4d074/tasks/b162m354f.output ·
  started 21:02 · stops by 22:52 (110 minutes on) · last check 21:02: started (the first saver, bn2k4nf3e, stopped
  with "time is up" at 19:12, its last line "nothing to save")
