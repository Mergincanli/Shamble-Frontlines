# verifier-build: review-tracks applied change-set (tier 2, kit)

Branch claude/confident-gates-tbnq38, HEAD cd1a300b (= origin), kit head dcd8f563; dcd8f563..HEAD touches only
.checkpoints/review-tracks.md. Merge-base with origin/main b9aad019; origin/main b8f02b01 (PR #78) touched no kit file
since (only .github/workflows/toolchain.yml and docs).

## Plan
1. gate --no-smoke (full if Chromium present)
2. five kit suites on branch + same on main worktree (.gate/verify-rt-main)
3. fix-words20 999 (+ cmp copy track.mjs vs real), fix-mutate, track.mjs --files x2
4. cmp 8 kit files vs .gate/review-tracks/build/, diff --stat list
5. grep pushed .checkpoints notes for exploit detail words

## Results (saved as I go)
### 1. Gate
- `node tools/gate.mjs --no-smoke`: exit 0, GATE PASS; 1-4 and 5b-5d PASS (symbols core 138/game 714/editor 353, +0 -0;
  player build 51 refs, 51 guarded, 0 flagged); 5a skipped.
- `node tools/gate.mjs` (full): 5a FAIL from environment only: installed Chromium is 1243 under /opt/pw-browsers, the
  pinned Playwright wants headless_shell-1194 ("Executable doesn't exist"). Not installed by me. Every other check PASS.
  Smoke not verified here; CI's smoke job is the check. (legacy/ untouched by this change-set.)
### 2. Kit suites (`node --test <file>`, real folder vs worktree .gate/verify-rt-main at origin/main b8f02b01)
| suite | head | main |
|---|---|---|
| routine-guard.test.mjs | 1581/1581, 0 fail | 1551/1551 |
| settings.test.mjs | 127/127 | 127/127 |
| checkpoint.test.mjs | 33/33 | 33/33 |
| kept-logs.test.mjs | 9 pass, 1 skipped (#10 needs KEPT_LOGS_DRY_RUN), 0 fail | same |
| track.test.mjs | 85/85 | (new) |
- routine-guard test names: every main name still present on head; head adds 30 (diff of sorted ok-lines: 30 `>`, 0 `<`).
### 3. Evidence spot-checks
- cmp .claude/scripts/track.mjs vs .gate/review-tracks/build/.claude/scripts/track.mjs: identical (fix-words20 imports the copy).
- `node .checkpoints/review-tracks/fix-words20.mjs 999` (reads origin/main b8f02b01): 165 entries: ok 47, no stated
  bytes and sha256 65, no saved file named 52, quote does not match 1. Differs from the claim (44 ok of 162) only by
  the 3 entries PR #78 added to main after the claim (all 3 ok). Same script pointed at the merge-base b9aad019
  (scratch copy verifier-words-base.mjs, one sed on the ref): 162 entries, ok 44 / 65 / 52 / 1: claim CONFIRMED exactly.
- `node .claude/scripts/track.mjs --files docs/STATUS.md`: exit 0, track light ("light (fixed list)"), picks
  [test-engineer] (no OWNER prefix matches docs/, so the default; the rule string adds "none at risk tier 0"), goFloors [],
  provisional note present. As claimed (light).
- `--files .claude/x.md`: exit 0, track full ("rule files (risk tier 2)"), picks [advisor, security-auditor], goFloors
  [".claude/x.md: rule files, secrets or infrastructure (risk tier 2)"]. As claimed.
### 4. Applied = reviewed copy
- cmp of the 7 kit files vs .gate/review-tracks/build/<same path>, and CLAUDE.md vs build/CLAUDE.md.new: all 8 identical.
- `git diff --name-only origin/main...HEAD -- .claude CLAUDE.md`: exactly those 8; whole-branch diff outside
  .checkpoints/ is the same 8. dcd8f563 vs working tree on .claude/CLAUDE.md: no diff; no uncommitted kit change.
- Every other file in the build copy equals the branch, except .claude/agent-memory-local/security-auditor/MEMORY.md
  (gitignored, .gitignore:10; not part of the change).
### 5. Pushed notes (git grep on HEAD -- .checkpoints; repo visibility: public)
- No secrets seen. Hits by word (locations only, no detail repeated):
  - "mcp__gh__": 0. "--repo": 0. "tag named": 0.
  - "-R": security-auditor-plan.md:82 (a source citation of a CLI's flags).
  - "graft": review-tracks.md:143,145; advisor-plan.md:291; fix-mutate.mjs:50; security-auditor-plan.md:131. All are
    about the fix inside the new track.mjs (not on main): no open-on-main gap.
  - wider sweep: "mcp__" security-auditor-plan.md:136,139; "three-part" :134; FLAG-POSITION :90; fail-open :10,:85;
    GIT_* env names review-tracks.md:128,142, advisor-plan.md:291, fix-mutate.mjs:41,50, te-fx-cwd.mjs:13 (fixes in the
    new script).
- Judgment: security-auditor-plan.md:8-11, :82 with :90, and :134-139 go beyond bare class words for items the notes
  themselves mark pre-existing on main (they name a shape or a code location, not only a class). Each is recorded as
  ending at a permission prompt (day sessions), none a silent route. Reported to the caller as a note, not repeated here.
### Extra
- `node .claude/scripts/repo-map-check.mjs`: exit 1, "missing from the map" track.mjs and track.test.mjs. Known and
  assigned to docs-keeper (card step 34); not in CI or the gate (only docs-keeper.md:29 runs it). Must land before ready.
- guard.mjs diff: a day session's PreToolUse now prints permissionDecision 'allow' when decide() returns allow only
  (MERGE-PROMPT (d)); routines unchanged (role set: no allow printed).
- `node .checkpoints/review-tracks/fix-mutate.mjs` (exit 0, on the build copy = real file by cmp): 62 mutants, 58
  killed, 4 survived, 0 missing; survivors: diff algorithm not pinned; graft file honoured; raw and numstat consistency
  not checked; twins list without --full-tree. Claim CONFIRMED exactly.
- Second runs: track 85/85, settings 127/127, checkpoint 33/33, kept-logs 9 pass/0 fail (stable).
- Launcher allow path is tested: routine-guard.test.mjs:3993-4005 (day allow printed; routines and ConfigChange never;
  a mutant dropping `!role` is caught).
