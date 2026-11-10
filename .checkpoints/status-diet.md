# Checkpoint: status-diet
- **Goal:** Mustafa's STATUS-DIET (STATUS <= 120 lines; DECISIONS.md his decisions word for word, newest first;
  STATUS_ARCHIVE.md the rest; moved by a line-range script; decision-id counts before/after; REPO_MAP and CLAUDE.md's
  read list) with LANE-RULES (lanes, ACCOUNTS, the Models line and its addition, the LOOK_V2 read-list line); record
  #46's merge (main 2d22b69). His words: main STATUS:674-761 and docs/reports/2026-11-10-kit-v3-5.md Appendix A.
  Plan: `.checkpoints/status-diet/plan-rev1.md` (next).
- **Branch:** `claude/busy-turing-48e4i9` (stands in for `cs/status-diet`) · **Base:** main 2d22b69
- **Last update:** 2026-11-10 21:25 Amsterdam

## Steps done
1. Facts gathered · 9e33794 · `.checkpoints/status-diet/facts.md`: line ranges and destinations, live PR and
   routine states, kit references, the LANE-RULES edit shape.
2. Plan revision 1 · 2f9d467 · `.checkpoints/status-diet/plan-rev1.md`.
3. The advisor's plan review · (this commit) · APPROVE with conditions 1-12, rulings Q1-Q5
   (`advisor-plan-rev1-verdict.md`; its notes `advisor-plan-rev1.md`, DONE); no new revision needed.
4. Draft PR #49 opened (subscribed); templates, split.mjs, check.mjs, followups.tsv, kit-build.py, e3.mjs · 8edf607,
   dffbbad (autosave).
5. The pure move · 8ead8f1 · check.mjs ALL PASS (E1, E2, Q2; STATUS 103 lines); gate PASS +0 -0.
6. #46 recorded (the archive's kit-v3-5 row, ROADMAP) · 1b030e3 · check.mjs --rev 8ead8f1 ALL PASS; gate PASS.

## Current step
Docs pointers (REPO_MAP, START_HERE:20, TECHNICAL:240); the kit files are built in .gate/status-diet/kit/ (E3 PASS).
Waiting in parallel: the advisor's ruling on his #46 go words (STATUS lists the security follow-ups already; DECISIONS
has the go entry, copied from 2d22b69's message).

## Next steps
1. Docs pointers commit; repo-map check.
2. Kit writes, one ask each (Write of CLAUDE.md, then docs-keeper.md, from .gate/status-diet/kit/); E3 on the real
   CLAUDE.md; E5 rule-diff table; kit tests on main and head (cond. 10); gate.
3. Review round: security-auditor + verifier (Opus 5.5 high), notes in this folder; saver on.
4. Fix pass; report docs/reports/2026-11-10-status-diet.md (cond. 2, 9, 12); delete the checkpoint; PR ready.

## Open findings
- Night shift cron: daily 06:00 Amsterdam from 2026-11-07, back to hourly 00:10-06:10 since 2026-11-10 18:00 UTC (read 20:10 UTC); recorded in STATUS.
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
