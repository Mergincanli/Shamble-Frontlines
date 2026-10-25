# Checkpoint: r7-legacy-inventory
- **Goal:** R7 legacy harvest: docs/LEGACY_INVENTORY.md (every legacy system, where, evidence, recommended mark),
  docs/LEGACY_TUNING.md + docs/legacy-tuning.json (tuning numbers as data), tools/tuning-dump.js. Plan and verdict:
  `.checkpoints/r7-legacy-inventory/plan.md` (APPROVE with 11 conditions).
- **Branch:** claude/bold-faraday-i49gug (draft PR #27), restarted from main f2ccc13 after kit-v3-0 merged (PR #26) ·
  **Base:** main f2ccc13 (the engine files are unchanged since 48ffb51, so every file:line cited at 48ffb51 still holds).
- **Last update:** 2026-10-25 17:05 Amsterdam

## Steps done
1. Plan approved with 11 conditions (plan.md, plan-verdict.json). Night queue #16 item 1 made a plain bullet "in
   progress as R7".
2. Workflow r7-read-verify (run wf_3552ad56-12f): 15 readers, 15 skeptics, 1 coverage critic, all done. 240 rows; the
   skeptics confirmed 97, corrected 143, refuted 0. Inputs collected in rows-and-checks.json and coverage.json · f930865.
3. tools/tuning-dump.js; two runs per map byte-identical (Cabana 12044cb2…, DM_Tri 1850983…) · 45a8a7b (old id;
   cherry-picked onto the push branch).
4. Value check (advisor condition 2): 17 of 17 dumped values equal their literal (tuning-value-check.mjs,
   evidence-tuning-value-check.txt) · 486f425.
5. docs/legacy-tuning.json (60 tables) and docs/LEGACY_TUNING.md (723 lines), built by build-tuning.py · 1eabd14.

## Current step
6. Workflow r7-edit-verify: per area an editor applies the skeptic's corrections, the missing systems and the coverage
   suggestions (final-<area>.json, editor-<area>.md), then a second skeptic checks the new and widened rows
   (skeptic2-<area>.md).

## Next steps
1. Count what came back (15 areas; a null → run that area alone). Save the result as edit-verify-result.json.
2. Apply the second skeptics' corrections (fixes2.json), then build docs/LEGACY_INVENTORY.md with
   build-inventory.py (header, tally by mark and area, method counts, per-area tables, not verified).
3. Add LEGACY_INVENTORY, LEGACY_TUNING, legacy-tuning.json and tools/tuning-dump.js to docs/REPO_MAP.md; run
   repo-map-check and the gate (symbols +0 −0).
4. Verifier spot-check of key claims; the report docs/reports/2026-10-25-r7-legacy-inventory.md (fold in this
   checkpoint, the plan verdict, the R7-PUSH note, the STATUS/ROADMAP deferral: PR #25 edits both).
5. Stop the saver; delete .checkpoints/r7-legacy-inventory* in the last commit; mark PR #27 ready; advisor review;
   edit #16 item 1 to point to the PR.

## Open findings
- Coverage (critic): core 138/138, game 584/714, editor 204/353 top-level declarations named by some row; the editor
  pass adds the critic's suggested rows (structures and barracks is the biggest gap).

## Key numbers
- First pass: 240 rows; marks redesign 85, port 76, rewrite in Rust 44, drop 35.
- Skeptics: 97 confirmed, 143 corrected, 0 refuted; 68 missing systems reported; 15 coverage suggestions.

## Decisions taken
- Areas: CD core data · CL core look · CC core characters and guns · CM core maps and buildings · CS core SIM ·
  GW game world · GA game animation · GC game combat · GN game bots and nav · GE game economy and objectives ·
  GP game presentation and dispatch · ED editor · PG pages · TL tools · KT kit.
- R7-PUSH was not raised: kit-v3-0 merged (PR #26) and the R7 commits were pushed to the session branch on 2026-10-25
  at 12:20 UTC; until then they were local only, as plan condition 5 recorded.
- Side work, not R7 (Mustafa asked, 2026-10-25 17:10): the Night shift routine trig_019LtRqmAdzddyk6K3e3kDJz wakes
  the dedicated session session_01PgUuEsdxACFrGDZL6SfcqD (repo attached, Opus 5.5, auto) at
  `CRON_TZ=Europe/Amsterdam 10 0-7 * * *`; setup check passed; the old repo-less trig_01Rp6QUaUTjj943CibAD5W6p is
  deleted. Open: NS-1 (add an 08:10 finish-only wake-up?) and NS-2 (delete the three old disabled routines?).

## Running work
- Workflow r7-edit-verify · run wf_5e9f70f7-c89 · task wkfrj4u3x · script as launched:
  `.checkpoints/r7-legacy-inventory/r7-edit-verify-run1.js` (no args) · notes in .checkpoints/r7-legacy-inventory/ (editor-*, final-*.json, skeptic2-*) · launched 2026-10-25
  16:58 Amsterdam · running.
- Background saver: `checkpoint.mjs watch r7-legacy-inventory` (task baz4w07oe), started 16:57, stops 18:47.
