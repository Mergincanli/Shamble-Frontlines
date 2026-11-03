# Plan A1: kit-v3-1 update. PR #30 onto main, plus the GUARD-FILE fix (revision 1, 2026-11-03)

**Task (Mustafa, 2026-11-03, item A1):** "Update PR #30 (kit-v3-1) onto main, resolve its conflicts, have it re-reviewed,
then wait for my go." **GUARD-FILE (a), decided:** "fix it inside #30 as part of A1. When guard.mjs is missing, a day
session is let through and only a routine is blocked. Add a test that removes the file. security-auditor reviews that
change, then the advisor re-reviews the whole updated PR." **Working rule:** never switch this session's main folder to
another branch or an old commit; trial merges, other branches and old commits go in a separate folder (git worktree).

## Where the work happens
- PR #30: branch `cs/kit-v3-1`, head 5be7d37, base main. Main is now e4d8b22 (PR #33 merged 2026-11-03 21:57 UTC).
- Working folder: a worktree outside the project, `<scratchpad>/kit-v3-1`, on a local `cs/kit-v3-1` that tracks
  origin. The session's main folder stays on `claude/sleepy-tesla-ofsxsr` at main's e4d8b22 and is never switched, so
  #30's `.claude/settings.json` never loads into this session.
- Pushes go to `cs/kit-v3-1`, PR #30's own branch: the task is to update #30 itself. This session's `claude/` branch
  is not used for A1.
- Checkpoint `.checkpoints/kit-v3-1.md` on `cs/kit-v3-1`; PR #30 goes back to draft while the work runs, and the
  checkpoint is deleted in the last commit before it is marked ready again (CHECKPOINTS.md §9.4).

## What the freeze showed (the defect)
The three guard hooks run `node ${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs` with `"onFailure": "block"`. A session
keeps the hook settings it loaded, while a branch switch changes the files on disk. If `guard.mjs` is gone, `node`
exits 1 ("Cannot find module"). That exit counts as a hook failure, and `onFailure: "block"` turns every failure into a
block. That covers every tool call, and the ConfigChange hook blocks the reload of the new settings too, so the
session stays frozen. The official hooks reference (quoted on 2026-10-27 in commit 2a75345,
`.checkpoints/kit-v3-1/hook-docs.md`) says so: "Can't start: a command hook fails to start, for example because the
script or executable doesn't exist" counts as a failure; "With "block" set, a failure does what exit code 2 does on that
event, except on PermissionRequest, where it denies the request". #30's report already names this as infra-engineer's
"accepted tradeoff" ("the same flag can block a day session if the guard can't start"); GUARD-FILE (a) reverses it.
Reproduced here in a scratch folder: the old command with `guard.mjs` removed exits 1 for a day session.

## Steps
0. **Save this plan** in the checkpoint on `cs/kit-v3-1` and push it; mark PR #30 as a draft.
1. **Merge main.** `git merge origin/main` (e4d8b22) into `cs/kit-v3-1`. Conflicts are in `docs/STATUS.md`,
   `docs/ROADMAP.md` and `docs/REPO_MAP.md` only (trial merge, 2026-11-03). `.claude/`, `CLAUDE.md` and the rest merge
   cleanly, because main has not touched them since #30 branched at 07ab240 (`git log 07ab240..origin/main -- CLAUDE.md
   .claude/` is empty).
   - Resolution rule: in every conflict hunk, take main's side. #30's text in those hunks is its 2026-10-27 dashboard
     (header, phase, "In flight" line, its kit-v3-1 row and ROADMAP entry, REPO_MAP's "Last checked"), which main's
     current text supersedes. The kit-v3-1 entries come back in step 4, written for the final state. Hunks git merged
     without a conflict stay as git merged them (for example REPO_MAP's hook rows and the 07:30 window line).
   - Check: for each of the three files, `git diff origin/main HEAD -- <file>` shows only #30's non-conflicting hunks.
     I also grep the merged ROADMAP for a doubled "Docs: loop v2" entry (#30 moved it into Build history; main
     reorganised Recent work).
   - Commit message: "Merge origin/main (e4d8b22) into cs/kit-v3-1".
   - Then the tests and the gate on the merged tree. The numbers should not change: routine-guard 192/192, settings
     82/82.
   - Then the window grep over the live docs (`00:10–08:30`, `08:30`, `08:20`, `08:10`, `07:45`, `07:40`), because
     main's new documents since 07ab240 may name the old window. Each hit gets a disposition: a dated record of what
     happened stays; a live rule goes to docs-keeper in step 4.
2. **GUARD-FILE fix** (one commit).
   - `.claude/settings.json`: the three guard handlers (PreToolUse, PermissionRequest, ConfigChange) change `args` from
     `["${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs"]` to `["-e", BOOT, "${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs"]`.
     Nothing else changes (`"command": "node"`, `"onFailure": "block"`, matchers). BOOT is one line:
     `import(require('url').pathToFileURL(process.argv[1]).href).catch(function(e){if(process.env.SHAMBLE_ROLE){process.stderr.write('guard.mjs could not load ('+String(e?e.message:e).slice(0,300)+'): a routine fails closed\n');process.exit(1)}process.exit(0)})`
     - If `guard.mjs` loads, it runs exactly as today: it decides, prints any refusal as JSON, and exits 0 itself.
     - If `guard.mjs` is missing, or present but unable to load (a syntax error on some branch), it depends on the
       session. A **day session** (no `SHAMBLE_ROLE`) exits 0 with no output, which means no decision: the call goes on,
       and so does the reload of the new settings. A **routine** (any `SHAMBLE_ROLE`, unknown values included) exits 1, a
       hook failure, which `onFailure: "block"` turns into a block on all three events (on PermissionRequest, a deny).
       That is the same mechanism that froze the last session, now limited to routines.
     - Why the check sits in `settings.json` and not in a file: any file can vanish on a branch switch, but the hook
       settings the session loaded cannot. BOOT contains no `${`, `$`, `"`, `&`, `|`, `<`, `>`, `^` or `%`. So no
       placeholder is substituted into it, and no character in it means anything to a shell, even if a Windows shell
       were involved. The docs say exec form uses no shell on any platform.
     - Scratch run on Node 22.23.4 (no repo file touched). guard.mjs present: a day session's `git checkout main` is
       refused, a routine's WebFetch is refused, and `git status` passes. guard.mjs removed: day session exit 0 with
       empty output on all three events; night-shift and pr-advisor exit 1 on all three. Broken guard.mjs: day 0,
       routine 1.
   - `.claude/hooks/settings.test.mjs`, new tests:
     a. the three guard handlers are identical, so a fix can't land on one event and miss another;
     b. the hook command run as Claude Code runs it (exec form; `${CLAUDE_PROJECT_DIR}` substituted into each `args`
        element as a plain string; no shell), against a copy of `.claude/hooks/` in a temporary project:
        - **guard.mjs removed (the test deletes it):** a day session exits 0 with empty output on PreToolUse,
          PermissionRequest and ConfigChange; night-shift, pr-advisor and an unknown role exit 1 (not 0, not 2) on each;
        - **guard.mjs present but with a syntax error:** day exits 0 with empty output; a routine exits 1;
        - **guard.mjs present:** the real guard still runs through the bootstrap (a day session's `git checkout main`
          gets a deny JSON; a routine's WebFetch gets a deny JSON; a day session's `git status` gets empty output).
     The existing registration test stays as it is (command `node`, `onFailure: "block"`, `${CLAUDE_PROJECT_DIR}` in the
     args).
   - `.claude/hooks/guard.mjs`: its header comment only, saying how settings.json starts it. No code change.
   - **Proof the new test catches the defect:** run the new tests against 5be7d37's `settings.json` (the old args). The
     "removed, day session" cases must FAIL (exit 1); on the fix they pass.
3. **Gate and tests** on the fix commit.
4. **Docs.**
   - **docs-keeper** (docs/ only):
     - STATUS: one kit-v3-1 row. Main's row has the stale scope "write guards for kit and gate files, a builder agent,
       ..."; it gets #30's accurate description. State: updated onto main e4d8b22, GUARD-FILE fixed, re-reviewed.
       Waiting on: Mustafa's go.
     - ROADMAP: #30's "Kit v3.1 — routine guard" Recent-work entry, back at the top, updated.
     - REPO_MAP: "Last checked", plus the `settings.json` and `guard.mjs` rows mention the bootstrap.
     - Any live-rule hit from the window grep.
     - The repo-map check must pass.
   - **Me:** `docs/reports/2026-10-27-kit-v3-1.md` gets an "Update, 2026-11-03" section: the merge, GUARD-FILE (a)
     (it reverses infra-engineer's accepted tradeoff; what remains is `node` itself failing to start), the evidence and
     the reviews.
   - **Out of scope:** STATUS and ROADMAP lines about PR #33. It merged at 21:57 UTC today, and main's dashboard still
     says "not merged". That belongs to a docs-after-33 pass, not to this rules PR (question Q2).
5. **Reviews.**
   - security-auditor on the fix (the `settings.json` hook args, `settings.test.mjs` and the `guard.mjs` comment); every
     finding is addressed or explicitly deferred.
   - Then the advisor re-reviews the whole updated PR, working from main's `advisor.md`. This session's `advisor` agent
     is main's, because the main folder is main. It is told the branch changes `advisor.md` and `CLAUDE.md`, so the
     branch's copies are evidence, not its instructions.
6. **Finish.**
   - Delete the checkpoint in the last commit, which runs CI.
   - Update the PR body with the report; mark the PR ready; keep `needs-mustafa`.
   - CI `gate` and `smoke` green on the final head.
   - Then wait for Mustafa's go.

## Symbols and files
- Game code: none (gate symbols +0 −0).
- Hook code: no new function in `guard.mjs` or `routine-guard.mjs`. `settings.json`: the `args` of three handlers.
  `settings.test.mjs`: one helper (`runGuardHook`) and the tests above.
- Files the fix touches: `.claude/settings.json`, `.claude/hooks/settings.test.mjs`, `.claude/hooks/guard.mjs`
  (comment). Merge: whatever main brings. Docs: STATUS, ROADMAP, REPO_MAP and #30's report.

## Change type and evidence
Kit/rules plus hook logic, so the evidence is:
- Node tests on the real code (above), including the defect shown on the old settings and gone on the fix.
- Gate PASS vs origin/main: checks 1–4 and 5b–5d here. 5a smoke runs here if this container's Playwright can launch;
  otherwise it comes from CI's `smoke` job on the final head (both CI jobs must be green before any merge).
- `node --test` on routine-guard, settings and checkpoint; the repo-map check OK.
- `git diff --name-only origin/main...HEAD` lists only `.claude/*`, `CLAUDE.md` and `docs/*`.
- No `.checkpoints/` in the final head.

## Risks
1. **The routine side rests on the documented `onFailure: "block"`** (Claude Code v2.1.295 or later; already a
   condition in #30's report). On an older version a routine fails open. This fix doesn't change that.
2. **One day-session freeze remains:** if `node` itself can't start (not on PATH). The project requires Node 22.13+.
   Stated, not fixed.
3. **#30's own old commits** (5be7d37 and earlier) keep the unfixed hook, so checking one out in a main folder could
   still freeze. That is the working rule's case; after the merge, main carries the fix.
4. **The merge drops #30's 2026-10-27 dashboard text.** Its kit-v3-1 entries are rewritten in step 4. The check in step
   1 and the reviewers confirm nothing else of #30's docs was lost.
5. **Windows:** exec form passes `args` without a shell on every platform (docs), and BOOT avoids shell-special
   characters anyway. `-e` runs as CommonJS, so `require` is available, and `pathToFileURL` handles drive-letter paths.
   It was tested here on Linux; CI runs Linux as well. Mustafa's Windows run (tools/README, "Check a new machine") would
   show a Windows problem.

## Reviewers, agents, Brave
- Reviewers: security-auditor (the fix), the advisor (the whole PR). No art, perf or netcode reviewer: nothing visual,
  no hot path, no sim.
- Agents: advisor, security-auditor and docs-keeper, each `model: opus`, `effort: high` from its agent file. They run
  one at a time, and none edits the same file as another.
- Brave: nothing for Mustafa to check, because nothing a player sees, hears or feels changes. After the advisor approves
  the final head with gate and smoke green, Mustafa says "merge #30" in a session's chat. The post-merge environment
  setup from #30's report is unchanged.

## Questions for the advisor
- **Q1 BROKEN-GUARD:** Should the decision's "missing" be read as "missing or unable to load"? Recommendation: yes. It
  is the R3c rule `guard.mjs` already applies to a broken rules file, and one `catch` covers both.
- **Q2 STATUS-33:** Should PR #33's stale STATUS and ROADMAP lines be left to a docs-after-33 pass? Recommendation: yes.
  This PR stays a rules PR.
