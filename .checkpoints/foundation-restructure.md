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
- **Last update:** 2026-11-10 10:20 Amsterdam

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

## Current step
2. Build step 1, the dry run (plan 1.6).

## Next steps
3. Step 2: the tools commit (plan 1.3): gate.mjs (`BUILD`, `BASE_PREFIXES`, `basePrefix`, both copies → exit 2, the
   prefix on the base line and in `--json`, :333 wording), smoke.mjs (default root = `BUILD`'s rule), village.py
   (default), tools/README.md:35 wording. Each gate-file edit asks Mustafa. Gate with smoke PASS on the root layout.
4. Step 3: E3 (26 mutations + controls, OLD's gate vs the edited gate, failing check ids equal, not empty); the edited
   gate on the dry run's `legacy/` layout vs OLD; both copies → exit 2; bases OLD, 8e58937, own head.
5. Push; draft PR (stays a draft); report to Mustafa: PR number, the proof, his asks.
6. At 2.1a's turn (after #46, STATUS-DIET, LANE-RULES): merge main in; step 4 (his decision recorded, in
   `docs/DECISIONS.md` if STATUS-DIET split STATUS); reviews (security-auditor, test-engineer, infra-engineer), the
   verifier, the report; checkpoint deleted in the last commit; PR ready; his go.

## Open findings
- (none yet)

## Key numbers
- (none yet)

## Decisions taken
- FREEZE-FDN (a), SPLIT (a), SCOPE (a) · Mustafa · 2026-11-10 (word for word in `mustafa-2026-11-10-answers.md`).
- Build 2.1a in this chat (no CLAUDE.md rule calls for a fresh session: plan approved for 2.1a's build, his decisions
  in, this branch is the session's own) · the session · 2026-11-10.

## Running work
- (none)
