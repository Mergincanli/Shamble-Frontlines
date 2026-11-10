# Checkpoint: foundation-restructure (foundation step 2.1; now building 2.1a)
- **Goal:** 2.1a, the legacy gate and tools learn both layouts (root and `legacy/`) without moving anything; then 2.1b,
  the move. Plan: `.checkpoints/foundation-restructure/plan.md` (revision 3; the advisor agent's APPROVE WITH CONDITIONS
  of revision 2, 8bbebfa, notes `advisor-plan-rev2.md`). The planning task's history: this file at d389d91.
- **His words:** the foundation decision and task (`mustafa-2026-11-10.md`) and his answers of 2026-11-10
  (`mustafa-2026-11-10-answers.md`): FREEZE-FDN (a), SPLIT (a), SCOPE (a) (ADR 0002 part 1 accepted, recorded in 2.1b's
  docs pass), "Build 2.1a now: the dry run, the gate edits (each asks me) and the mutation proof"; merge order #46,
  STATUS-DIET and LANE-RULES, 2.1a, 2.1b, each on his go; 2.1a's PR stays a draft until its turn; both accounts count
  the same.
- **Branch:** `claude/advisor-fable-4hxq72` (this cloud session may push only there), standing in for
  `cs/foundation-restructure`; Mustafa's personal account. **Base / OLD:** main 44446a5 (unchanged on 2026-11-10 10:15).
- **Last update:** 2026-11-10 11:48 Amsterdam

## Steps done
0. Planning (d389d91): plan revisions 0-3, the inventory workflow, the advisor agent's plan review, his answers.
1. Build step 0 (this commit): checkpoint moved from `foundation` to `foundation-restructure`; his answers saved;
   the open PRs re-read on 2026-11-10: #46 at d7526f5 (guard, its tests, advisor.md, CLAUDE.md, docs), #40 at b7a1a32
   (REPO_MAP, STATUS, docs/design, spikes); STATUS-DIET and LANE-RULES have no PR yet. 2.1a's files (tools/gate.mjs,
   tools/smoke.mjs, tools/village.py, tools/README.md) are in none of them.
   - The toolkit: `npm ci --prefix tools` done. Playwright 1.56.1 wants Chromium build 1194; the container has 1243 at
     /opt/pw-browsers and `npx` is denied, so the smoke runs with `PLAYWRIGHT_BROWSERS_PATH=<repo>/.gate/pw-browsers`,
     a folder of links naming 1243's executables as 1194 (smoke on main: PASS, index 8/8, editor 5/5). Caveat for the
     report: locally a newer Chromium than pinned; CI runs the pinned one.

2. Build step 1, the dry run (`.gate/fdn-dry`, worktree of 44446a5): `git mv` of the five files → five `R100`; today's
   gate: exit 2, "gate: shamble-core.js is missing from the working tree" (gate.mjs:98), as the plan expected.
3. Build step 2, the tools (dd12aeb): built and proven first in a plain scratch folder `.gate/fdn-scratch` (not a
   worktree; `protectedRel` false there), then written with one ask each for `tools/gate.mjs` and `tools/smoke.mjs`
   (`cp` from scratch, `cmp` equal), plus `tools/village.py` and `tools/README.md`. Full gate with smoke PASS on the root
   layout (base 44446a5). Folder rule table `rows-test.mjs`: 8 rows + 2 smoke-CLI rows ALL PASS. Without Playwright
   (`--omit=optional`): `--no-smoke` PASS. Base checks (`e3-bases.md`): the edited gate reads root, CS-1
   (`shamble-frontlines-repo/`) and legacy/ bases with a root or a legacy/ head, all PASS; OLD's gate DIES on a legacy/
   base (the revert case the split prevents); both copies → exit 2.
   - Departure from plan 1.3 (to state in the plan and the PR body): the folder marker is the three `shamble-*.js` files,
     any of them, not "all five files": any overlap → exit 2, and a future v2 `index.html` at the root never makes the
     root look like the legacy build. New names: smoke.mjs `findBuild` (export), `LEGACY_JS`; gate.mjs `BUILD`,
     `BUILD_PREFIX`, `BASE_PREFIXES`, `basePrefix`; `baseFile(b, f)` now takes the base object.

## Current step
4. Build step 3, E3: `e3-run.mjs` over `e3-mutations.json` (31 entries: P1-P4, M1-M22, C1-C3, PC1, PC2, drafted by an
   Opus agent, notes `agent-mutations.md`) in `.gate/fdn-old` (44446a5), `.gate/fdn-new` (dd12aeb) and
   `.gate/fdn-newleg` (9af08f7: dd12aeb + a local commit of the move, never pushed); results to `e3-results.json`;
   running in the background since 11:05 Amsterdam (log `.gate/e3-run1.log`). PC1 judged on check 3 only (the Phase 0
   gate had checks 1-4; refreshRect is still called at boot, so 5a/5b fail on every gate).

   - Pushed (dd12aeb inside 9ae4037); draft PR #47 opened 2026-11-10 (stays a draft until its turn); CI run 42801926823
     (push, head 9ae4037) in progress at 11:47 Amsterdam.
   - E3 so far: P1 PASS (old, new, newleg all fail [3, 5a, 5b]; the report's [3] included).

## Next steps
5. When E3 ends: results into PR #47's body and the checkpoint; CI run ids; report to Mustafa: PR number, the proof,
   his asks.
6. At 2.1a's turn (after #46, STATUS-DIET, LANE-RULES): merge main in; step 4 (his decision recorded, in
   `docs/DECISIONS.md` if STATUS-DIET split STATUS); the plan updated for the marker departure; reviews
   (security-auditor, test-engineer, infra-engineer), the verifier, the report; worktrees removed (links first, then
   `git worktree remove`, never `--force`); checkpoint deleted in the last commit; PR ready; his go.

## Open findings
- (none yet)

## Key numbers
- (none yet)

## Decisions taken
- FREEZE-FDN (a), SPLIT (a), SCOPE (a) · Mustafa · 2026-11-10 (word for word in `mustafa-2026-11-10-answers.md`).
- Build 2.1a in this chat (no CLAUDE.md rule calls for a fresh session: plan approved for 2.1a's build, his decisions
  in, this branch is the session's own) · the session · 2026-11-10.

## Running work
- E3 run: background task b93x1ze3s, `node .checkpoints/foundation-restructure/e3-run.mjs` (resumable: skips entries
  already in `e3-results.json`), started 11:05 Amsterdam.
- Saver: background task bod2js7dn, started 10:44 Amsterdam, stops by 12:34; check at each update.
