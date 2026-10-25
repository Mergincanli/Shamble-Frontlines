# Checkpoint: r7-legacy-inventory
- **Goal:** R7 legacy harvest: docs/LEGACY_INVENTORY.md (every legacy system, where, evidence, recommended mark),
  docs/LEGACY_TUNING.md + docs/legacy-tuning.json (tuning numbers as data), tools/tuning-dump.js. Plan and verdict:
  `.checkpoints/r7-legacy-inventory/plan.md` (APPROVE with 11 conditions).
- **Branch:** claude/bold-faraday-i49gug, restarted from main f2ccc13 after kit-v3-0 merged (PR #26); the local commits
  were cherry-picked onto it and pushed 2026-10-25 12:20 (4ab639a…6d278c4) · **Base:** main f2ccc13 (engine files
  unchanged since 48ffb51, so every file:line cited at 48ffb51 still holds). The running workflow still writes its notes
  into the old worktree; they are copied over when it ends.
- **Last update:** 2026-10-25 12:25 Amsterdam

## Steps done
1. Plan approved with conditions (verdict in plan.md and plan-verdict.json). Night queue #16 item 1 turned into a
   plain bullet "in progress as R7" (about 2026-10-25 00:00).

## Current step
2. Workflow "r7-read-verify": 15 area readers, each followed by a skeptic, then a coverage critic. Notes:
   `.checkpoints/r7-legacy-inventory/reader-<area>.md`, `skeptic-<area>.md`, `critic-coverage.md`.

## Steps done (continued)
3. tools/tuning-dump.js written; two runs per map byte-identical (Cabana 12044cb2…, DM_Tri 1850983…) · 45a8a7b.
   Defaults are equal on both maps; boot changes: Cabana 18 (NAV grid, BK.PROPS/TREES), DM_Tri 41 (also SILO, TBOX,
   HARV points). headless-gl built with `npm rebuild gl` in tools/.

## Next steps
4. Value check (advisor condition 2): 17 dumped values equal their literal at file:line, 0 fail
   (`tuning-value-check.mjs`, `evidence-tuning-value-check.txt`). Deviation: a plain Node script reading the dump
   JSON and `git show 48ffb51:<file>`, not an eval snippet, because an eval snippet cannot read the source files; the
   dump itself is the live value from preview.js.
4. Assemble LEGACY_INVENTORY.md and LEGACY_TUNING.md; tally; fix what the skeptics and the critic confirmed.
5. Gate (symbols +0 -0), verifier spot-check, report; once kit-v3-0 merges: restart the push branch from main,
   bring these commits over, push, PR (draft → ready), edit #16 item 1 to point to the PR.

## Open findings
- none yet

## Key numbers
- none yet

## Decisions taken
- Areas: CD core data · CL core look · CC core characters and guns · CM core maps and buildings · CS core SIM ·
  GW game world · GA game animation · GC game combat · GN game bots and nav · GE game economy and objectives ·
  GP game presentation and dispatch · ED editor · PG pages · TL tools · KT kit.

## Running work
- Workflow r7-read-verify · run wf_3552ad56-12f · task w2s0uav6m · script as launched:
  `.checkpoints/r7-legacy-inventory/r7-read-verify-run1.js` (no args) · notes in `.checkpoints/r7-legacy-inventory/` ·
  launched about 2026-10-25 00:00 Amsterdam · cut off at about 01:00 by the org's monthly spend limit
  (11 of 27 agents done: 11 readers; failed: readers ED, PG, TL, KT, all 15 skeptics, the coverage critic) · resumed
  2026-10-25 11:25 with resumeFromRunId (task wllinx1zp) · running
