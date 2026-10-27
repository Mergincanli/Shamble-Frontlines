# Advisor report: r7-legacy-inventory (2026-10-27, branch claude/bold-faraday-i49gug, base main f2ccc13)
This cloud session can push only `claude/bold-faraday-i49gug`; it stands in for `cs/r7-legacy-inventory`.

## Goal
R7 of the v2 phase 0 brief (§4): one document that tells the v2 team what the legacy game has, where, how well it is
proven, and what to do with each piece, plus every tuning number as data, so R1–R6 and v2's content tables start
from the real game, not from memory.

## Outcome
DONE. 247 systems with a recommended mark, 60 tuning tables dumped from the live build, docs and one read-only tool
only (symbols +0 −0). Review rounds stopped after the final fix pass, at the Advisor's word (2026-10-27).

## Plan and approval
Plan: `.checkpoints/r7-legacy-inventory/plan.md` (at a715be2; deleted in this PR's last commit, the history keeps it).
Advisor verdict 2026-10-24: APPROVE with 11 conditions. How each was met:
1. **Dump timing:** `tools/tuning-dump.js` runs as `--pre` (defaults, before `bootGame`) and as `eval` (only the values
   boot changed: Cabana 18, DM_Tri 41); functions and THREE objects follow a stated rule and are listed.
2. **Tool evidence:** 17 dumped values equal their literal at file:line (at least 10 asked); two runs per map are
   byte-identical (sha256 Cabana 12044cb2…, DM_Tri 1850983…); the tool imports nothing and writes no file;
   `node --check` passes. Deviation: the value check is a Node script reading the dump and `git show 48ffb51:<file>`,
   because an eval snippet can't read the source files; the values themselves come from the live build.
3. **LOOP_V2 marks:** the game-designer's review found the marks follow the rule (eras and income, silos and tech
   boxes, win conditions, spawning and buying, progression are redesign with the section cited); its 11 findings
   (9 should, 2 nits) applied.
4. **The look:** 118 rows carry "look brief pending". The graphics-engineer confirmed that the look-pipeline rows
   (CL1–4, CL6–8, CL14, CD15–17, CD19) say "redesign (look brief pending)" with no further recommendation, and added the
   one missing flag (CD5, the camera framing).
5. **Push risk:** local only until kit-v3-0 merged; pushed on 2026-10-25 at 12:20 UTC. R7-PUSH was not needed.
6. **STATUS and ROADMAP:** not edited (open PR #25 edited both; it merged on 2026-10-27). The next docs pass records R7.
7. **Checkpoint:** kept and pushed after every step; folded in here; deleted in the last commit.
8. **Kit rows:** describe the kit at 48ffb51, each changed row says "Changed by kit-v3-0"; routines rows list only
   Shamble Frontlines routines. No Unit Forge text anywhere (grep: 0 hits).
9. **Night queue:** #16 item 1 points to PR #27 as a plain bullet, so the night shift skips it.
10. **Models:** about 126 agents, all Opus; effort high, xhigh for the engine and netcode reviews.
11. **Merging kit-v3-0:** merged on Mustafa's go after the advisor's APPROVE of its head (PR #26, f2ccc13).

## What changed
- `docs/LEGACY_INVENTORY.md` (new): 247 rows in 15 areas; header with marks, tally, states, method, coverage.
- `docs/LEGACY_TUNING.md` (new) and `docs/legacy-tuning.json` (new): 60 tables, defaults and boot changes per map.
- `tools/tuning-dump.js` (new): read-only dump through `tools/preview.js`.
- `docs/REPO_MAP.md`: the four new files. This report.

## Evidence
- **Gate:** PASS on checks 1–4 and 5a–5d against main 4c24d07; symbols +0 −0; smoke index 8/8, editor 5/5, 0 errors.
- **Repo map check:** 93 of 93 paths.
- **Rows:** 240 read → 249 after the edit pass (9 added, 45 widened) → 247 (2 duplicates merged).
- **Marks:** rewrite in Rust 45 · port to TypeScript 74 · redesign 96 · drop 32. States: [C] 141, [M] 106, [D] 0.
- **Coverage** (acorn, every top-level declaration): core 138/138 named; game 625 named + 89 in a cited range = 714/714;
  editor 293 + 60 = 353/353. Range hits are mechanical, and the page says so.
- **Verification passes** (details in the page header):
  - skeptics on every row: 97 confirmed, 143 corrected, 0 refuted;
  - second skeptics on every added or widened row: 13 confirmed, 41 corrected;
  - a checker per area on every applied fix: 6 more fixes;
  - final review: 115 findings → 110 fixed, 1 rejected, 4 handled by the session; checkers then found 13 more, all fixed.
- **Spend-limit cut-offs:** four, all on the org's spend limit: the read workflow twice on 2026-10-25 (at about 01:00
  Amsterdam, and the coverage critic before the 15:20 UTC reset), the first apply workflow (reset 20:40 UTC), and the
  final fix pass (reset 2026-10-26 01:40 UTC). Each run resumed from its cache and notes; nothing was lost or redone.

## Reviews (final review, once, on the assembled rows)
- **Verifiers** (5 groups; every [M] row's measurement re-read at its source, plus 28 sampled code-read rows): PASS
  WITH NOTES. 9 blocking findings: 5 fixed as stated; the other 4 asked to renumber kit rows to today's lines and were
  kept at 48ffb51 under condition 8, with "at 48ffb51" added. The worst real one: CL8 credited a draw-call and memory
  fix to `stylizeOne`, but it came from the `clearMap` fix.
- **engine-architect** (107 rows): REVISE (small). Found a bug in the session's coverage script, which skipped bare
  `:NNN` ranges; fixed.
- **netcode-architect** (70 rows): 2 blocking, both fixed:
  - CD8: `weaponPitchMax` is not an aim limit; it only tilts the held gun model;
  - GC3: ledges are aimed at their height, not at 0.
- **graphics-engineer** (138 rows): REVISE, nothing blocking. Look flags complete; CD5 (camera framing) is now look
  brief pending.
- **game-designer** (45 rows): APPROVE with fixes. The marks follow condition 3.
- **Coverage closer:** 20 fixes named all 92 unplaced declarations; no new rows were needed.

## Decisions and deviations
- **Kit line numbers stay at 48ffb51** (condition 8). The verifier flagged four kit rows (KT2, KT7, KT9, TL10) for
  renumbering to today's lines; the appliers added "at 48ffb51" wording instead. **Doc citations are to f2ccc13**; PR #25 later moved
  STATUS lines by up to 2 and ROADMAP lines by up to 3 (the page header says so). Rows whose fact #25 changed (main's
  branch protection: KT3, KT8, KT10, TL1) were corrected.
- **Duplicates merged:** CM14 into CC12 (melee props), and CC7 into CL5 (applyVillain). CL5's two readers disagreed
  (port vs drop); it now recommends drop, since no game code calls it after D6, and records both views.
- **Session edits:** KT14 records the night-shift routine of 2026-10-25 (hourly 00:10–08:10).
- **The tuning tables are mapped to their source lines by script** (`build-tuning.py`), not by an agent as the plan
  said: a regex on each declaration is exact.

## Risks
1. R1–R6 will use the marks before Mustafa confirms them: the research must treat them as recommendations.
2. Some `ANIMATION_PLAN.md` §6 numbers come from headless snippets that are not in `tools/`, so they can't be re-run as is.
3. Line numbers go stale once phase 1 moves the legacy files to `legacy/` (the paths change; the lines don't).

## Proposed next step
R1–R6 research on `cs/v2-research` (plan approved with 14 conditions) once this merges; kit-v3-1 on `cs/kit-v3-1`.

## Decisions needed
- **R7-MARKS · Confirm the recommended marks?** Every row's last column is Claude's recommendation. Why it matters:
  R1–R6 and the ADR drafts will lean on them (45 rows say rewrite in Rust, 96 redesign). Options: (a) take them as
  working recommendations now, and change any row when its ADR draft comes up; (b) review all 247 first. I
  recommend (a): the drafts are where each mark gets decided.

## For Mustafa
1. After the advisor approves this PR's final head: say "merge #27". Nothing to check in Brave (docs and a read-only tool).
