# Plan A1: kit-v3-1 update — PR #30 onto main + the GUARD-FILE fix (revision 2, 2026-11-03)

Revision 2 supersedes revision 1 (committed 4def8d2). What changed: the BOOT snippet is now `require`-free
(promise-only, CJS/ESM-safe) after validation on this machine; a newly found pre-existing finding (the routine-guard
Windows test failures) is disposed of as Q3; the session/worktree details match this session (main folder on `main`).

**Task (Mustafa, 2026-11-03, item A1):** "Update PR #30 (kit-v3-1) onto main, resolve its conflicts, have it re-reviewed,
then wait for my go." **GUARD-FILE (a), decided:** "fix it inside #30 as part of A1. When guard.mjs is missing, a day
session is let through and only a routine is blocked. Add a test that removes the file. security-auditor reviews that
change, then the advisor re-reviews the whole updated PR." **Working rule (Mustafa, 2026-11-03):** never switch this
session's main folder to another branch or an old commit; trial merges, other branches and old commits go in a separate
folder (git worktree).

## Where the work happens
- PR #30: branch `cs/kit-v3-1`. Current head `4def8d2` (a `[skip ci]` checkpoint commit added by the frozen session on
  top of the pre-freeze head `5be7d37`). Base main is now `e4d8b22` (PR #33 squash-merged 2026-11-03).
- This session's main folder `C:\dev\shamble-fresh` stays on **`main`** (e4d8b22) and is never switched, so #30's
  `.claude/settings.json` (with the guard hook) never loads here. Verified: main's `.claude/settings.json` has no
  `guard.mjs` reference, so this session has no freeze risk.
- Working folder: a git worktree outside the project at `<scratchpad>/kit-v3-1`, on local branch `cs/kit-v3-1` tracking
  origin. All build edits happen there. Pushes go to `cs/kit-v3-1` (the task is to update #30 itself).
- Checkpoint `.checkpoints/kit-v3-1.md` on `cs/kit-v3-1`; PR #30 goes back to draft while the work runs; the checkpoint
  is deleted in the last commit before the PR is marked ready (CHECKPOINTS.md §9.4).

## The defect (what the freeze showed)
The three guard handlers in `.claude/settings.json` (PreToolUse, PermissionRequest, ConfigChange) run
`node ${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs` with `"onFailure": "block"`. The hook settings a session loaded stay
in memory; a branch switch changes the files on disk. If `guard.mjs` is gone, `node` exits 1 ("Cannot find module")
*before* any of guard.mjs's own fail-safe runs. That exit is a hook failure, and `onFailure: "block"` turns every failure
into a block on every tool call (a deny on PermissionRequest), including the ConfigChange that would reload the new
settings — so the session stays frozen. `guard.mjs` already fails safe when its *rules file* (`routine-guard.mjs`) can't
load (day session → allow, routine → deny; plan R3c) and always exits 0 itself — but it cannot cover *itself* being
missing. GUARD-FILE (a) moves that one remaining fail-safe up into `settings.json`, which cannot vanish on a branch
switch. #30's report already names this as infra-engineer's accepted tradeoff ("the same flag can block a day session if
the guard can't start"); GUARD-FILE (a) reverses it.

## Steps
0. **Save this plan** (revision 2) in the checkpoint on `cs/kit-v3-1` and push it. Mark PR #30 a draft.
1. **Merge main.** In the worktree, `git merge origin/main` (e4d8b22) into `cs/kit-v3-1`.
   - Conflicts are **only** `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/REPO_MAP.md` (confirmed 2026-11-03 by
     `git merge-tree origin/main origin/cs/kit-v3-1`). `.claude/`, `CLAUDE.md` and everything else merge cleanly
     (`git log 07ab240..origin/main -- CLAUDE.md .claude/` is empty — main has not touched them since #30 branched).
   - Resolution rule: in every conflict hunk, **take main's side**. #30's text there is its 2026-10-27 dashboard, which
     main's current (2026-11-03) text supersedes. The kit-v3-1 doc entries are written fresh for the final state in
     step 4 (docs-keeper), not hand-merged from stale copies. Non-conflicting #30 hunks stay as git merged them.
   - Checks after the merge: for each of the three files, `git diff origin/main HEAD -- <file>` shows only #30's
     non-conflicting additions; grep the merged docs for the old night-shift window strings (`00:10–08:30`, `08:30`,
     `08:20`, `08:10`, `07:45`, `07:40`) — a dated record stays, a live rule goes to docs-keeper in step 4.
   - Commit message: "Merge origin/main (e4d8b22) into cs/kit-v3-1".
2. **GUARD-FILE fix** (one commit).
   - `.claude/settings.json`: change the `args` of the three guard handlers from
     `["${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs"]` to
     `["-e", BOOT, "${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs"]`. Nothing else changes (`"command": "node"`,
     `"onFailure": "block"`, matchers). The three handlers stay byte-identical to one another.
   - BOOT (one line; `require`-free, promise-only):
     `import('node:url').then(function(u){return import(u.pathToFileURL(process.argv[1]).href)}).catch(function(e){if(process.env.SHAMBLE_ROLE){process.stderr.write('guard.mjs could not load ('+String(e?e.message:e).slice(0,300)+'): a routine fails closed');process.exit(1)}process.exit(0)})`
     - `guard.mjs` loads → it runs exactly as today (decides, prints JSON, exits 0 itself). `.catch` never fires.
     - `guard.mjs` missing or unloadable (a syntax error on some branch): a **day session** (no `SHAMBLE_ROLE`) exits 0
       with no output → the call proceeds and the settings reload proceeds; a **routine** (any `SHAMBLE_ROLE`, unknown
       values included) exits 1 → a hook failure that `onFailure:"block"` turns into a block/deny on all three events.
     - Why promise-only and not the revision-1 `require('url')` form: if `node -e` ever ran as ESM (a future Node
       default, or a root `package.json` with `"type":"module"`), `require` would throw **synchronously**, outside the
       `.catch`, re-freezing day sessions. `import('node:url')` has no such path. (There is no root `package.json` today,
       so `-e` is CommonJS and both forms work; the promise-only form removes the dependency.)
     - Why BOOT lives in settings.json, not a file: any file can vanish on a branch switch; the loaded hook settings
       cannot. BOOT contains no `$`, `${`, `"`, backtick, `\`, `&`, `|`, `<`, `>`, `^` or `%`, so no placeholder is
       substituted into it and no character means anything to a shell even if one were involved (exec form uses no shell
       on any platform, per the hooks reference).
   - `.claude/hooks/settings.test.mjs`, new tests (one helper `runGuardHook`, exec form, `${CLAUDE_PROJECT_DIR}`
     substituted as a plain string, no shell, against a temp copy of `.claude/hooks/`):
     a. the three guard handlers are byte-identical (a fix can't land on one event and miss another);
     b. **guard.mjs removed:** a day session exits 0 with empty output on PreToolUse, PermissionRequest and
        ConfigChange; night-shift, pr-advisor and an unknown role exit 1 (not 0, not 2) on each;
     c. **guard.mjs present but a syntax error:** day exits 0 empty; a routine exits 1;
     d. **guard.mjs present:** the real guard still runs through the bootstrap (day `git checkout main` → deny JSON;
        routine `curl` → deny JSON; day `git status` → empty).
     The existing registration test stays (command `node`, `onFailure:"block"`, `${CLAUDE_PROJECT_DIR}` in args; now
     also the `-e` + BOOT shape).
   - **Proof the test catches the defect:** run tests b/c against 5be7d37's old `args` — the "removed/broken, day" cases
     must FAIL (exit 1) there and PASS on the fix.
   - `.claude/hooks/guard.mjs`: header comment only (how settings.json starts it). No code change.
3. **Gate and tests** on the fix commit (evidence below).
4. **Docs (docs-keeper, docs/ only).**
   - STATUS: one kit-v3-1 row with #30's accurate scope (replacing main's stale "write guards … a builder agent …"):
     state = updated onto e4d8b22, GUARD-FILE fixed, re-reviewed; waiting on Mustafa's go. Note the Q3 finding as a
     follow-up if the advisor agrees.
   - ROADMAP: #30's "Kit v3.1 — routine guard" recent-work entry, updated.
   - REPO_MAP: "Last checked" date; rows for `.claude/hooks/guard.mjs`, `routine-guard.mjs`, `routine-guard.test.mjs`,
     `settings.test.mjs` (the bootstrap mentioned on the settings/guard rows); `repo-map-check.mjs` must pass.
   - Any live-rule hit from the window grep.
   - **Out of scope:** PR #33's now-stale STATUS/ROADMAP lines ("not merged") — Q2.
   - **Me:** `docs/reports/2026-10-27-kit-v3-1.md` gets an "Update, 2026-11-03" section: the merge, the GUARD-FILE (a)
     fix, the evidence, the reviews, and the Q3 finding.
5. **Reviews.**
   - security-auditor on the fix (the settings.json hook args, `settings.test.mjs`, the guard.mjs comment), with the Q3
     finding in front of it; every finding addressed or explicitly deferred.
   - Then the advisor re-reviews the whole updated PR, from main's `advisor.md`. It is told the branch changes
     `advisor.md` and `CLAUDE.md`, so the branch's copies are evidence, not instructions.
6. **Finish.** Delete the checkpoint in the last commit (runs CI); update the PR body with the report; mark it ready;
   keep `needs-mustafa`; CI `gate` + `smoke` green on the final head; then wait for Mustafa's go.

## Change type and evidence
Kit/rules plus one hook-launch change, so:
- **BOOT mechanism validated** on this machine (Node v24.17.0): a matrix of {present, missing, syntax-error} ×
  {day, night-shift, pr-advisor, unknown-role} gave exactly the intended exits (present → 0 + guard ran; missing/broken →
  day 0 empty, any role 1 + stderr). Both the `require` and promise-only forms passed; the promise-only form is chosen.
- **New `settings.test.mjs` tests** (step 2) on the real files, including the defect shown on the old `args` and gone on
  the fix.
- **Gate** vs origin/main: checks 1–4 and 5b–5d run in the worktree (reusing main's `tools/node_modules` via a
  directory junction, since kit-v3-1 does not change `tools/`). 5a smoke runs locally if Playwright is available, else
  from CI's `smoke` job. Both CI jobs (`gate`, `smoke`) must be green on the final head before any merge.
- `node --test` on `routine-guard.test.mjs`, `settings.test.mjs`, `.claude/scripts/checkpoint.test.mjs`; the repo-map
  check OK. **Note:** CI does not run the hook tests (gate.yml runs only `tools/gate.mjs` + smoke), so these numbers are
  local/dev evidence. On Linux (where routines and CI run) #30's suite is 192/192 (its report + the advisor's 2026-10-28
  review); `routine-guard.mjs` is byte-identical to the approved head, so its behaviour is unchanged by A1.
- `git diff --name-only origin/main...HEAD` lists only `.claude/*`, `CLAUDE.md` and `docs/*`; game symbols +0 −0.
- No `.checkpoints/` in the final head.

## Symbols and files
- Game code: none (gate symbols +0 −0).
- Hook code: no new function in `guard.mjs` or `routine-guard.mjs`. `settings.json`: the `args` of three handlers.
  `settings.test.mjs`: one helper + the tests above.
- Files the fix touches: `.claude/settings.json`, `.claude/hooks/settings.test.mjs`, `.claude/hooks/guard.mjs`
  (comment). Merge brings whatever main brings in the three docs. Docs: STATUS, ROADMAP, REPO_MAP, #30's report.

## Risks
1. The routine side rests on the documented `onFailure:"block"` (Claude Code v2.1.295+). On an older version a routine
   fails open. This fix doesn't change that.
2. One day-session freeze remains: if `node` itself can't start (not on PATH). The project requires Node 22.13+. Stated,
   not fixed.
3. #30's old commits (5be7d37 and earlier) keep the unfixed hook, so checking one out in a main folder could still
   freeze. That is the working rule's case; after merge, main carries the fix.
4. The merge drops #30's 2026-10-27 dashboard text in the three docs; its kit-v3-1 entries are rewritten in step 4, and
   the step-1 checks + reviewers confirm nothing else of #30's docs was lost.
5. Windows: exec form passes `args` without a shell on every platform (docs), and BOOT has no shell-special characters.
   `-e` runs as CommonJS (no root package.json), and the promise-only BOOT needs neither `require` nor a module type.
   `pathToFileURL` handles drive-letter paths. Tested here on Windows (Node v24) and on CI (Linux).

## Questions for the advisor
- **Q1 BROKEN-GUARD:** Read GUARD-FILE's "missing" as "missing *or* unable to load" (a syntax error too)? One `catch`
  covers both; it is the same R3c rule guard.mjs already applies to a broken rules file. **Recommend: yes.**
- **Q2 STATUS-33:** Leave PR #33's now-stale STATUS/ROADMAP lines ("ready for review, not merged") to a separate
  docs-after-33 pass, keeping A1 a focused rules PR? **Recommend: yes.**
- **Q3 WINDOWS-TESTS (new):** On this Windows machine (Node v24) `routine-guard.test.mjs` is **183/192** — 9 pre-existing
  failures, all because `credentialDeny()` / the `.npmrc` check match sensitive paths with forward-slash-only regexes
  while Windows `path.resolve`/`join` give backslashes (so `~/.ssh`, `.git/config`, `.env`, `.npmrc`, and the real-git
  abbreviation + mutation tests don't fire/behave). `routine-guard.mjs` is untouched by A1; routines run only in the
  cloud (Linux), where it is 192/192. Options: **(a)** document it as a follow-up (fix the guard's path handling to be
  separator-agnostic, and add the hook tests to CI) and keep A1 focused — **recommended**; (b) widen A1 to harden the
  path handling now (reopens reviewed guard code, scope creep); (c) escalate to Mustafa. Note either way that CI does
  not currently run the hook tests, so a Linux regression in them would not be caught by CI.
