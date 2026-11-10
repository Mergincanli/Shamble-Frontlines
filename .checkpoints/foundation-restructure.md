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
- **Last update:** 2026-11-10 13:50 Amsterdam

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

4. Build step 3, E3 (`e3-summary.md`, `e3-results.json`): 31 of 31 PASS. For every mutation OLD's gate (44446a5), the
   edited gate at the root (dd12aeb) and the edited gate on `legacy/` (9af08f7) fail exactly the same checks, never none;
   every control exits 0 on all three (PC1 on check 3). P1-P4 fail on the Phase 0 report's checks for its reasons.
   M19/M20 split the report's "5b + 5d" pair (5b and 5d one each, the same on all three): a note. Worktrees restored
   clean. CI run 42801926823 on 9ae4037: gate and smoke success (pinned Chromium).

5. PR #47's body updated with E3; M5 covers the outside-folder check on all three gates; the stricter `legacy/` rule
   proven (a repo-root read from `legacy/` fails 5a); CI also ran on the PR (run 42802055698, success); rebuild commands
   in `e3-summary.md`.

## Current step
Waiting for 2.1a's turn (after #46, STATUS-DIET, LANE-RULES), PR #47 a draft. Nothing runs.

## Next steps
6. At 2.1a's turn (after #46, STATUS-DIET, LANE-RULES): merge main in; step 4 (his decision recorded, in
   `docs/DECISIONS.md` if STATUS-DIET split STATUS); the plan updated for the marker departure; reviews
   (security-auditor, test-engineer, infra-engineer), the verifier, the report; worktrees removed (links first, then
   `git worktree remove`, never `--force`); checkpoint deleted in the last commit; PR ready; his go.

## Open findings
- (none yet)

## Key numbers
- E3: 31 of 31 PASS (4 Phase 0 mutations, 22 check-5 mutations, 3 + 2 controls) × 3 gates = 93 full gate runs.
- Folder-rule table: 8 + 2 rows PASS. Base checks: 7 edited-gate runs PASS over root, CS-1 and legacy/ bases; OLD dies on a legacy/ base.

## Decisions taken
- FREEZE-FDN (a), SPLIT (a), SCOPE (a) · Mustafa · 2026-11-10 (word for word in `mustafa-2026-11-10-answers.md`).
- Build 2.1a in this chat (no CLAUDE.md rule calls for a fresh session: plan approved for 2.1a's build, his decisions
  in, this branch is the session's own) · the session · 2026-11-10.

## Running work
- E3 run: done (31 of 31). Worktrees `.gate/fdn-*` kept until 2.1a's turn (the verifier may re-run E3); remove then.
- Saver: background task bod2js7dn, 10:44-12:34 Amsterdam, ended by time (last save d403bd3); not running.
