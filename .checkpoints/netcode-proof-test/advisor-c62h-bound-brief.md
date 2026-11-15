# Advisor: plan review of the C62H-BOUND build brief (c62h-bound-brief.md)

Seat: advisor (main's .claude/agents/advisor.md, read with `git show origin/main:.claude/agents/advisor.md`, 27881 bytes).
Read only: this notes file is the only file I change.
Repo HEAD at start: 77d3ad7 (checkpoint autosave); origin/main 53272a1.

## Plan
1. Read the brief (540 lines) and its notes.
2. Read his words: history.md 2113-2129 (13:35 rule), 2131-2146 (C62H-BOUND (a) + conditions), 2163-2175 (C62H-BOUND-YES).
3. Read my earlier reads: advisor-c62h-bound-full.json (C1-C11, Q1-Q5), advisor-c62h-bound-r2-full.json (te_rulings,
   replacements, planned_size, paste_blocks, other_changes 6), c62h-bound-test-full.json (TE-CB-1..10), the design json.
4. Frozen texts: c62h-bound-texts-v3.md, c62h-bound-freeze.txt; recompute sha256 of each block; paste sha256.
5. Card: NOW block, KEPT-LOGS lines, C62H-BOUND blocks, MH-L1 note. advisor-merged-head-full.json for head.
6. Code: spikes/netcode-proof (l1, lab, judge, marks, ops/runs/c-u4r-runs.mjs, check.mjs); runners u4r-run.sh, main-run.sh sha256.
7. Earlier briefs for form: u4r-brief.md, brawl1-brief*.md.
8. Cross-check: every C5-C11 item, te_rulings, other_changes 6, his YES words (freeze, P0 re-judge, size stop, stop rules) carried,
   nothing beyond; frozen texts by script + sha256; order build -> review round -> merge into branch -> P2 then P3 -> re-judge -> report;
   builder decides nothing.
9. Findings with brief line + source line; verdict; structured output.

## Progress
- 1-5 read. Hashes recomputed: texts-v3 3f21c9b7, freeze 9cb8be17, paste cfac7502, u4r-run.sh e9437ae2, main-run.sh
  8df2dee3, peak.py dfacdcc0, c-u4r-runs.mjs 99dd070e, marks.json 9fb5be57, SERIES.md 3411eee7, README.md b49f3441:
  all equal the brief's. Brief sha256 ba6bdeb0 (540 lines) = its notes.

## Findings (a line per finding)
