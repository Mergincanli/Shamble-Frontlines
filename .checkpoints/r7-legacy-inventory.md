# Checkpoint: r7-legacy-inventory
- **Goal:** R7 legacy harvest: docs/LEGACY_INVENTORY.md (every legacy system, where, evidence, recommended mark),
  docs/LEGACY_TUNING.md + docs/legacy-tuning.json (tuning numbers as data), tools/tuning-dump.js. Plan and verdict:
  `.checkpoints/r7-legacy-inventory/plan.md` (APPROVE with 11 conditions).
- **Branch:** claude/bold-faraday-i49gug (draft PR #27), restarted from main f2ccc13 after kit-v3-0 merged (PR #26) ·
  **Base:** main f2ccc13 (the engine files are unchanged since 48ffb51, so every file:line cited at 48ffb51 still holds).
- **Last update:** 2026-10-25 21:51 Amsterdam

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

7. Workflow r7-apply-fixes2 (run wf_df83f4d1-1f2, resumed once after the spend limit): 26 agents, 13/13 areas;
   41 rows corrected by script-applied ops; checkers found 6 more problems (CL14, GC3, GC13, ED2, PG3, KT20), all
   verified by grep and applied. Session edit: KT14 records the new night-shift routine. Results apply-result.json.
8. docs/LEGACY_INVENTORY.md built (249 rows: Rust 45, TypeScript 76, redesign 96, drop 32; look brief pending 119;
   [C] 143, [M] 106); coverage after all passes: core 138/138, game 586 named + 92 in range / 714 (36 neither),
   editor 233 + 64 / 353 (56 neither); repo map 93/93; gate PASS, symbols +0 -0 · 0c6e2b9.
## Current step
9. Final review and verification workflow (r7-final-review): verifier spot-checks of every [M] claim and sampled
   rows; marks lenses (engine-architect, netcode-architect, graphics-engineer, game-designer); a coverage closer for
   the 92 unplaced declarations. Each returns fix ops; the session applies them with apply-ops.py.

## Next steps
1. Count what came back; verify and apply the fix ops (fixes3-ops-<area>.json, then re-run apply-ops on them); rebuild
   the page; gate.
2. The report docs/reports/2026-10-25-r7-legacy-inventory.md (fold in this checkpoint, the plan verdict and how each
   condition was met, the R7-PUSH note, the STATUS/ROADMAP deferral: PR #25 edits both).
3. Stop the saver; delete .checkpoints/r7-legacy-inventory* in the last commit; mark PR #27 ready; advisor review.

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
- Workflow r7-final-review · run wf_4e8e0798-666 · task ws35vt5h9 · script as launched:
  `.checkpoints/r7-legacy-inventory/r7-final-review-run1.js` (no args; sample in final-review-sample.json) · 10 agents
  (5 verifiers, engine-architect, netcode-architect, graphics-engineer, game-designer, coverage closer) · notes
  verifier-<group>.md, <agent>-review.md, coverage-closer.md · launched 2026-10-25 21:53 Amsterdam · running.
- Done: r7-apply-fixes2 (wf_df83f4d1-1f2), r7-edit-verify (wf_5e9f70f7-c89), r7-read-verify (wf_3552ad56-12f).
- Background saver: task bstocjk8x, started 21:42 Amsterdam, stops 23:32.
