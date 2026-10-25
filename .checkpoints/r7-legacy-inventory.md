# Checkpoint: r7-legacy-inventory
- **Goal:** R7 legacy harvest: docs/LEGACY_INVENTORY.md (every legacy system, where, evidence, recommended mark),
  docs/LEGACY_TUNING.md + docs/legacy-tuning.json (tuning numbers as data), tools/tuning-dump.js. Plan and verdict:
  `.checkpoints/r7-legacy-inventory/plan.md` (APPROVE with 11 conditions).
- **Branch:** claude/bold-faraday-i49gug (draft PR #27), restarted from main f2ccc13 after kit-v3-0 merged (PR #26) ·
  **Base:** main f2ccc13 (the engine files are unchanged since 48ffb51, so every file:line cited at 48ffb51 still holds).
- **Last update:** 2026-10-25 21:44 Amsterdam

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
6. Workflow r7-edit-verify (run wf_5e9f70f7-c89): 28 agents, 0 failed. 15/15 areas back; 249 final rows (9 added);
   73 suggestions not taken, each with a reason; second skeptics on every added or widened row: 13 confirmed,
   41 corrected, 0 refuted. Result: edit-verify-result.json · 70e31f4.

## Current step
7. Workflow r7-apply-fixes2: per area an applier turns the second skeptic's corrections into edit operations
   (fixes2-ops-<area>.json), apply-ops.py applies them exactly (after-<area>.json), and a checker compares every change
   with its correction and the code (applier-<area>.md, checker-<area>.md).

## Next steps
1. Count what came back (13 areas: CL CC CM CS GW GA GC GN GE GP ED PG KT; a null → run that area alone); apply the
   checkers' fix ops; re-run apply-ops.py for every area; save the result as apply-result.json.
2. Run coverage-final.cjs, then build-inventory.py (needs inventory-meta.json: missing 68, added, widened, rejected 73,
   second checks 13 confirmed / 41 corrected / 0 refuted) → docs/LEGACY_INVENTORY.md; read it whole.
3. Add docs/LEGACY_INVENTORY.md to docs/REPO_MAP.md; repo-map-check; gate (symbols +0 −0).
4. Verifier spot-check of key claims (a workflow); the report docs/reports/2026-10-25-r7-legacy-inventory.md (fold in
   this checkpoint, the plan verdict, the R7-PUSH note, the STATUS/ROADMAP deferral: PR #25 edits both).
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
- Workflow r7-apply-fixes2 · run wf_df83f4d1-1f2 · script as launched:
  `.checkpoints/r7-legacy-inventory/r7-apply-fixes2-run1.js` (no args) · launched 2026-10-25 17:44 Amsterdam (task
  w4744pjn9) · cut off by the org's monthly spend limit (reset 20:40 UTC): 16 of 26 agents done (13 appliers, checkers
  CL CC CS; CL found one problem in CL14.where, fix op in apply-result-run1.json); failed: checkers CM GA GN GC GE GP ED
  PG KT GW · resumed 2026-10-25 21:43 Amsterdam with resumeFromRunId (task w97w68lbm) · running.
- Done: r7-edit-verify (wf_5e9f70f7-c89), r7-read-verify (wf_3552ad56-12f).
- Background saver: restarted 21:44 Amsterdam (the first one stopped at 18:47 when its time was up).
