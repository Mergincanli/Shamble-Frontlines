# Checkpoint: r7-legacy-inventory
- **Goal:** R7 legacy harvest: docs/LEGACY_INVENTORY.md (every legacy system, where, evidence, recommended mark),
  docs/LEGACY_TUNING.md + docs/legacy-tuning.json (tuning numbers as data), tools/tuning-dump.js. Plan and verdict:
  `.checkpoints/r7-legacy-inventory/plan.md` (APPROVE with 11 conditions).
- **Branch:** local `r7-legacy-inventory` in a worktree (not pushed until kit-v3-0 merges; then onto
  claude/bold-faraday-i49gug restarted from main) · **Base:** main 48ffb51
- **Last update:** 2026-10-25 00:40 Amsterdam

## Steps done
1. Plan approved with conditions (verdict in plan.md and plan-verdict.json). Night queue #16 item 1 turned into a
   plain bullet "in progress as R7" (2026-10-25 00:3x).

## Current step
2. Workflow "r7-read-verify": 15 area readers, each followed by a skeptic, then a coverage critic. Notes:
   `.checkpoints/r7-legacy-inventory/reader-<area>.md`, `skeptic-<area>.md`, `critic-coverage.md`.

## Next steps
3. Write tools/tuning-dump.js (defaults via --pre, post-boot diffs on Cabana and tri), its value check, two runs.
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
- (filled in at launch)
