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
- **Last update:** 2026-11-11 10:50 Amsterdam

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
It is #47's turn (Mustafa, 2026-11-11: "order: #49, then #47, then 2.1b, then #48"). This chat stopped at his step 1
(context 748k of 1,000k, over half full) with this checkpoint and a resume card. Nothing done yet for his steps 2-4.
Main is bb2d817: #46 (2d22b69, the guard fix with the PR Advisor's allow-list) and #49 (bb2d817, STATUS split into
STATUS, DECISIONS and STATUS_ARCHIVE; LANE-RULES; CLAUDE.md "Lanes and accounts") merged since 44446a5; neither touches
`tools/` or `.github/`, and no file is changed on both sides (`comm` of the two diffs: empty), so the merge is clean
and OLD's gate files still equal 44446a5's.

## Next steps (his steps 2-4, in a fresh chat on this account and this branch)
1. Bring main in without switching the main folder (CLAUDE.md Never): `git fetch origin main`, then `git merge origin/main`
   (it is not `--ff-only`, so the guard asks him; or do it in a worktree and push the merge). Re-read CLAUDE.md (new
   "Lanes and accounts") and `docs/STATUS.md`, and how `docs/DECISIONS.md` and `docs/STATUS_ARCHIVE.md` are laid out
   (and docs-keeper's agent file, which follows the split). Gate on the merged head: `PLAYWRIGHT_BROWSERS_PATH=$PWD/.gate/pw-browsers node tools/gate.mjs`
   (rebuild the browser alias first if the container is new: `e3-summary.md`).
2. The docs pass, by docs-keeper, in the new layout, text moved by script, never retyped (his rule):
   - `docs/STATUS.md` "In flight": one row for #47 (2.1a), from the evidence below.
   - `docs/DECISIONS.md`: his words of 2026-11-10 and 2026-11-11, word for word from `mustafa-2026-11-10.md` and
     `mustafa-2026-11-10-answers.md` (the foundation decision; FREEZE-FDN (a), SPLIT (a), SCOPE (a); the build go;
     the merge order; both accounts count the same; today's order); check DECISIONS.md's own entry format first.
   - The overtaken lines get dated pointers to that entry (plan step 4): ADR 0002 migration step 1 (0002:1422-1425),
     SLICE-ORDER condition 3 (now in DECISIONS or the archive: re-find "still runs before the foundation"), SPIKE_PLAN
     :86-91, :115 and :429-432. Decision docs: his go.
   - The deferred kit-v2-1 gate-globals Low (was STATUS:1009; re-find it in the archive): closed by dd12aeb.
   - The rest (2.1a's done record) in `docs/STATUS_ARCHIVE.md` as docs-keeper's split rule says; ROADMAP's entry.
   - `docs/REPO_MAP.md` needs no 2.1a change (no new top-level path); `node .claude/scripts/repo-map-check.mjs`.
3. The review round on the final diff (3 agents at a time; Opus, effort high): security-auditor (tier 2: gate files),
   test-engineer (E3, the rows, the bases), infra-engineer (CI and the gate on old bases); then the verifier until PASS;
   one fix pass at most (the review budget). The advisor agent checks plan conditions C1-C8 at the PR head.
4. The report `docs/reports/2026-11-1x-foundation-2-1a.md` (`/advisor-report`): fold this checkpoint in (steps with
   SHAs, E3 31/31, the rows, the bases, CI runs 42801926823 and 42802055698, the Chromium-1243 caveat, the marker
   departure, his two approved gate prompts); remove the worktrees (`.gate/fdn-*`: delete each `tools/node_modules` link
   first, then `git worktree remove`, never `--force`); delete `.checkpoints/foundation-restructure*` in the last commit
   (no `[skip ci]`), but first copy `plan.md` and `prompt-2.2.md` somewhere 2.1b can read them: 2.1b reads the plan
   at this branch's last checkpoint SHA (record it in the report). Mark #47 ready; stop for his go (needs-mustafa: gate
   files; no Brave check for 2.1a).

## Open findings
- (none yet)

## Key numbers
- E3: 31 of 31 PASS (4 Phase 0 mutations, 22 check-5 mutations, 3 + 2 controls) × 3 gates = 93 full gate runs.
- Folder-rule table: 8 + 2 rows PASS. Base checks: 7 edited-gate runs PASS over root, CS-1 and legacy/ bases; OLD dies on a legacy/ base.

## Decisions taken
- FREEZE-FDN (a), SPLIT (a), SCOPE (a) · Mustafa · 2026-11-10 (word for word in `mustafa-2026-11-10-answers.md`).
- Build 2.1a in this chat (no CLAUDE.md rule calls for a fresh session: plan approved for 2.1a's build, his decisions
  in, this branch is the session's own) · the session · 2026-11-10.
- He approved both gate prompts (gate.mjs, smoke.mjs); PR #47 is not watched (he says when its turn comes); 2.2 is
  planned in a fresh session on this account, from a one-paragraph prompt given in this chat · Mustafa · 2026-11-10.

## Running work
- E3 run: done (31 of 31). Worktrees `.gate/fdn-*` kept until 2.1a's turn (the verifier may re-run E3); remove then.
- Saver: background task bod2js7dn, 10:44-12:34 Amsterdam, ended by time (last save d403bd3); not running.
