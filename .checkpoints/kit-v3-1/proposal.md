# kit-v3-1 proposal: permission lists for the two routines (WRITE-GUARD, revised)

Base: `.claude/settings.json` on origin/main f2ccc13, re-checked on 4c24d07 (#25 merged since; that diff touches only
docs/ROADMAP.md and docs/STATUS.md, so every settings.json line number below still holds). Goal (the Advisor's lesson, 2026-10-25): every command the night
shift and the PR Advisor need is **allowed**, everything they must never do is **denied**, and nothing they hit is
**ask-first**. Nothing here is applied: no repository edit, no commit, no routine or session change.

## 0. In short
- **The stuck prompt.** The PR Advisor run on PR #26 (`session_01FPXGo8GioEMFrCm8FhivMU`) has waited since
  11:02:43Z on one Bash approval: `cd <its scratchpad>/pr26/tools && npm ci --silent 2>&1 | tail -3; ls node_modules | head; ls /opt/pw-browsers`.
  The `npm ci` part hit the ask rule `Bash(npm ci *)` (settings.json:119); the `ls /opt/pw-browsers` part reads
  outside the working folders (the harness offered `Read(//opt/pw-browsers/**)`). It had to install because a fresh
  worktree has no `tools/node_modules`, and neither did the run's own checkout (E5). #26 has since merged, so the
  answer no longer matters for #26.
- **One settings file can't do this job alone.** It serves three kinds of sessions: the night shift (must never
  merge), the PR Advisor (must merge, never commit), and Mustafa's day sessions (edit `.claude/` in kit change-sets,
  merge on his go). A deny in that file binds all three. So the proposal has three layers:
  1. **Static allow** for every command form the routines really type (section 2).
  2. **Static deny** only for what CLAUDE.md forbids every session, plus commands nobody needs in phase 0 (section 3).
  3. **A role guard** for the routine-only "nevers": a `PreToolUse` hook that knows which routine it runs in from an
     environment variable set on a routine-only cloud environment, plus a `PermissionRequest` hook that turns any
     prompt left over into a denial for routines (section 5).
- **All 16 ask rules go** (section 4). The `ask` list ends empty.
- **Procedure changes** make the needed commands safe to allow as typed: the PR Advisor reviews from main's rules and
  installs from main's lockfile only, and `npx` is replaced so it can be denied (section 6).

Trace ids: N1–N56 = night-shift trace entries in order; A1–A43 = PR Advisor trace; P1–P18 = past-runs trace;
D1–D8 = permission-docs trace; E1–E11 = new evidence from this step (section 9).

## 1. How the rules behave (what the design relies on)
- Deny, then ask, then allow; a deny in any scope wins; an ask prompts even when a narrower allow matches (D1, D7).
- Every part of a compound command (`&&`, `;`, `|`) must match an allow rule or be read-only; deny and ask hit any part.
- In a cloud routine an ask rule **waits for a human** (D7: routines doc); cloud sessions ignore `defaultMode`
  `dontAsk` (E1), so ask rules can't be turned into denials by mode.
- Cloud sessions run file edits without asking in every mode (E1), so only a deny or a hook stops an edit.
- A trailing ` *` also matches the bare command (D1). Wrappers `timeout`, `time`, `nice`, `nohup`, `stdbuf`,
  `command` are stripped before matching; `xvfb-run`, `npx` and `env` are not (N19).
- Static deny rules can't stop a determined bypass (`python -c 'subprocess…'`); they stop the common and accidental
  forms. The classifier and the GitHub ruleset on main (required gate + smoke, PR rule, no bypass; A26) stay behind them.

## 2. Allow rules (static, all sessions)
Keep lines 5–10, 12–40 and the WebSearch / 62 WebFetch domains (41–103) as they are. Remove line 11
(`npx --no -- playwright install chromium`, replaced, see P-5). Add:

| Rule | Serves |
|---|---|
| `Bash(npm ci --prefix tools --ignore-scripts --no-audit --no-fund)` | N13, A7, P1, D1: the install the stuck run needed; one exact form, run from the main checkout only (P-3) |
| `Bash(npm rebuild gl --prefix tools)` | E6: `preview.js:8` requires `gl`, whose binary comes from its install script; runs that one script only |
| `Bash(node tools/node_modules/playwright/cli.js install chromium)` | N14, A10: replaces `npx --no -- …` so `npx` can be denied (P-5) |
| `Bash(ln -s ../../../tools/node_modules .gate/*/tools/node_modules)` | A9, N24: one install shared by every worktree under `.gate/` (relative link, path-independent) |
| `Read(//opt/pw-browsers/**)` | N15, A8, D2, P1: the second half of the stuck prompt |
| `Bash(xvfb-run -a node tools/preview.js *)` · `Bash(xvfb-run -a node --expose-gc tools/preview.js *)` · `Bash(xvfb-run -a node tools/pageshot.js *)` | N19, A12, A15: the Linux forms CLAUDE.md:164 and tools/README.md:65 prescribe |
| `Bash(node --expose-gc tools/preview.js *)` | N21, A14: perfbots (CLAUDE.md Commands) |
| `Bash(node tools/smoke.mjs)` · `Bash(node tools/smoke.mjs *)` | N22 |
| `Bash(node .gate/*)` | N23, A17: logic harnesses (CLAUDE.md:66). Adds no power: `preview.js eval` already runs any file |
| `Bash(node --version)` · `Bash(npm --version)` | A6, P6 |
| `Bash(git rev-parse *)` · `Bash(git ls-tree *)` · `Bash(git cat-file *)` · `Bash(git for-each-ref *)` · `Bash(git grep *)` · `Bash(git remote -v)` · `Bash(git remote show origin)` · `Bash(sha256sum *)` | N5, A2, A18, P2, P5, P7, P8: read-only forms that sent half the past calls to the classifier |
| `Bash(git ls-remote *)` | N6, P8: the handoff check (NIGHT_SHIFT.md:16) |
| `Bash(git rm -r .checkpoints/*)` · `Bash(git rm .checkpoints/*)` | N12: delete the checkpoint in the last commit (CHECKPOINTS.md:132); `rm -rf` stays denied |
| `Bash(mkdir -p .checkpoints/*)` · `Bash(mkdir -p .gate/*)` | N28, A21: notes and scratch |
| `Bash(pkill -TERM -f checkpoint.mjs)` | N11: stop the saver so its last save runs (CHECKPOINTS.md:92-93) |
| `Bash(gh pr ready *)` · `Bash(gh pr diff *)` | N32, A27 |
| `Bash(gh api repos/Mergincanli/Shamble-Frontlines/*)` · `Bash(gh api --hostname github.com repos/Mergincanli/Shamble-Frontlines/*)` | A3: the reads the trigger context tells the session to make; write flags denied (section 3) |
| `Bash(npm audit --prefix tools*)` | N34: security-auditor (`npm audit fix` denied) |
| `mcp__github__issue_read` · `mcp__github__pull_request_read` · `mcp__github__list_pull_requests` · `mcp__github__get_label` · `mcp__github__add_issue_comment` · `mcp__github__update_issue_comment` · `mcp__github__issue_write` · `mcp__github__create_pull_request` · `mcp__github__update_pull_request` · `mcp__github__merge_pull_request` | N31, N32, A4, A24–A26, P11–P15: accepted today by a cloud config rule, not by our file; listing them keeps them prompt-free if that default changes. The merge tool is guarded by the hook (section 5) |

Not given an allow rule, on purpose:
- **Agent / Workflow spawns** (N33, A22): Agent allow rules are dropped in auto mode; the classifier reviews spawns.
- **Plain edits** (N29): already prompt-free in cloud sessions (E1); the role guard fences them.
- **`TZ=Europe/Amsterdam date`** (N1, P10): read-only, already passes.
- **Profiler flags other than `--expose-gc`** (`--allow-natives-syntax`, `--trace-deopt`, inspector profiles; N34):
  rare. They stay with the classifier (accepted, not a stall unless it blocks three in a row).

## 3. Deny rules (static, all sessions)
Keep every current deny (124–159). Add the rules below. "Checked against" lists the needed commands each rule could
touch; section 7 repeats the check the other way round.

| Rule(s) | Stops | Checked against |
|---|---|---|
| `Bash(git rebase *)` · `Bash(git commit *--amend*)` | N47, A38: history rewrite (CLAUDE.md:212); were ask 113, 115 | N8 `git commit -m …` (no `--amend`) |
| `Bash(git switch -f*)` · `Bash(git switch *--force*)` · `Bash(git switch *--discard-changes*)` · `Bash(git switch -C *)` | N47, N49, A39; were ask 106–109 (`*--force*` also covers `--force-create`) | N7 `git switch -c …`, `--detach`, `--track`: pass, **if matching is case-sensitive** (U-3) |
| `Bash(git branch -d*)` · `Bash(git branch -D*)` · `Bash(git branch --delete*)` · `Bash(git branch * -d*)` · `Bash(git branch * -D*)` · `Bash(git branch * --delete*)` · `Bash(git branch -f*)` · `Bash(git branch --force*)` · `Bash(git branch * -f*)` · `Bash(git branch * --force*)` · `Bash(git branch -M*)` · `Bash(git branch -C*)` | N45, N47, A37: branch deletion and moves. Today `-d` and `-f` are ALLOWED by `Bash(git branch *)` (line 23) and `-D` is ask (110–111) | N4 `git branch --show-current`, `-a`, `-r`, `-vv`, `--format=…`: pass |
| `Bash(git worktree remove *--force*)` · `Bash(git worktree remove -f*)` · `Bash(git worktree remove * -f*)` | N49: forced removal deletes untracked work | N25, A29: clean `git worktree remove <path>` stays allowed by line 27 |
| `Bash(git reset --soft*)` · `--mixed*` · `--keep*` · `--merge*` · `Bash(git reset *HEAD~*)` · `Bash(git reset *HEAD^*)` · `Bash(git reset *@{*)` · `Bash(git reset *ORIG_HEAD*)` · `Bash(git reset *origin/*)` | N47, A38 | E7: `git reset -q HEAD -- .checkpoints` (checkpoint.mjs:174 advice) passes |
| `Bash(git filter-repo*)` · `Bash(git update-ref *)` · `Bash(git replace *)` · `Bash(git reflog expire*)` · `Bash(git reflog delete*)` | N47, A38 | checkpoint.mjs runs `update-ref` as a child process (N10), which rules never see |
| `Bash(git checkout *)` · `Bash(git restore *)` · `Bash(git stash drop*)` · `Bash(git stash clear*)` | N49: discarding work | no needed form: branches via `git switch` (N7), files via `git show <ref>:<path>` |
| `Bash(git merge *)` | N37, A30; was ask 114 | N26 `git merge-base …`: passes (the rule needs a space after `merge`) |
| `Bash(git push)` · `Bash(git push * HEAD)` · `Bash(git push * :*)` | N39 (bare push on a local main), N45, A37 (`:branch` deletes) | N9 `git push -u origin cs/<slug>`, `git push origin cs/<slug>`: pass (P-7 names the branch every time) |
| `Bash(git -*)` | N39, A20, A32, A33: every leading git option (`-C`, `-c`, `--git-dir`, `--work-tree`, `--no-pager`, …) slips past every git deny | A20 `git -C <wt> status`: replaced by P-6. No other needed git command starts with `-` |
| `Bash(git ls-remote *--upload-pack*)` · `Bash(git ls-remote *-u *)` · `Bash(git ls-remote *ext::*)` · `Bash(git grep *-O*)` · `Bash(git grep *--open-files-in-pager*)` | the command-running options of the two new read allows | N6 `git ls-remote --heads origin cs/<slug>`: passes |
| `Bash(git config *extraheader*)` · `Bash(git config --list*)` · `Bash(git config -l*)` · `Bash(printenv*)` · `Bash(env *)` · `Bash(gh auth *)` · `Read(~/.git-credentials)` · `Read(~/.config/gh/**)` · `Read(~/.netrc)` · `Read(~/.npmrc)` | N51, A41: secrets by Bash (only the Read tool is covered today, 155–159). `env *` also closes the `env git push -f` wrapper bypass | none; npm and gh read their own config as child processes |
| `Bash(gh pr merge *)` · `mcp__github__enable_pr_auto_merge` | N36, A28, A30, A31: one merge route only (MCP with `expectedHeadSha`, E3); auto-merge skips the expected-head check (advisor.md:120-121); was ask 116 | A26 uses `mcp__github__merge_pull_request` |
| `Bash(gh api *-X*)` · `Bash(gh api *--method*)` · `Bash(gh api *-f *)` · `Bash(gh api *-F *)` · `Bash(gh api *--field*)` · `Bash(gh api *--raw-field*)` · `Bash(gh api *--input*)` | A3, A30, A37: `gh api` writes (PUT …/merge, DELETE …/refs) | A3 reads: pass, also case-insensitively (`Shamble-Frontlines` has `-F` with no space after it) |
| `Bash(gh secret *)` · `Bash(gh variable *)` · `Bash(gh repo edit*)` · `mcp__github__create_repository` · `mcp__github__fork_repository` | A43, advisor.md:39-43 (CI and security settings are Mustafa's) | none |
| `mcp__github__push_files` · `mcp__github__create_or_update_file` · `mcp__github__delete_file` · `mcp__github__update_pull_request_branch` | N39, A34: commits through the API (also a push-to-main route) and changing an approved head | none: commits go through git |
| `Bash(npx *)` · `Bash(npm exec *)` | N52, A40: `npx <pkg>` without `--no` downloads and runs code; a static rule can't tell the forms apart, so all of `npx` goes once P-5 lands | N14, A10 move to `node tools/node_modules/playwright/cli.js install chromium` |
| `Bash(npm run *)` · `Bash(npm test*)` · `Bash(npm start*)` · `Bash(npm audit fix*)` · `Bash(npm audit * fix*)` | N53, A40: `tools/package.json` has no scripts, so nothing needs them in phase 0; were ask 120–121. Revisit when phase 1 brings a package with scripts | `npm audit --prefix tools`: passes (the rule needs a space before `fix`) |
| `Bash(sh *)` · `Bash(bash *)` · `Bash(eval *)` · `Bash(node)` · `Bash(node -)` · `Bash(python)` · `Bash(python3)` | N52: `curl … \| sh` style pipes and `bash -c` wrappers around denied commands | no needed command starts a shell; `node -e`, `node .gate/…` and `python3 tools/village.py` pass |

## 4. The 16 ask rules
None stays. Every one of them is a Bash command a routine can type, so each is a stall today (D7).

| Ask rule (line) | Becomes | Why |
|---|---|---|
| `git switch -f *` (106) | deny `git switch -f*` | N49, A39 |
| `git switch --force *` (107) | deny `git switch *--force*` | N49, A39 |
| `git switch --discard-changes *` (108) | deny `git switch *--discard-changes*` | N49, A39 |
| `git switch -C *` (109) | deny `git switch -C *` (U-3) | N47 |
| `git branch -D *` (110) | deny (`-D*`, `-d*`, `--delete*` and the `* -d*` forms) | N45, A37 |
| `git branch --delete --force *` (111) | deny `git branch --delete*` | N45 |
| `git worktree remove *` (112) | removed: clean removal allowed by line 27; forced forms denied | N25, A29 (the live #25 parent-session prompt) |
| `git commit --amend *` (113) | deny `git commit *--amend*` (also catches a bare `--amend`) | N47, A38 |
| `git merge *` (114) | deny | N37, A30 |
| `git rebase *` (115) | deny | N47, A38 |
| `gh pr merge *` (116) | deny; the one merge route is the MCP tool with `expectedHeadSha` | N36, A28, D7 |
| `npm install *` (117) | role-deny (hook), section 5 | N52, A40; day sessions may need it for a Mustafa-approved dependency |
| `npm i *` (118) | role-deny (hook) | same |
| `npm ci *` (119) | allow the one exact form (section 2) + hook lockfile check | N13, A7, P1, D1: **the stuck prompt** |
| `npm run *` (120) | deny | N53 |
| `npm test *` (121) | deny | N53 |

If the role guard is not adopted (decision ROLE-SIGNAL (c)), `npm install` / `npm i` become static denies instead.

## 5. The role guard (routine-only "nevers") and the prompt backstop
### Which routine am I? (decision ROLE-SIGNAL)
Recommended: two routine-only cloud environments, each with one environment variable, `SHAMBLE_ROLE=night-shift` or
`SHAMBLE_ROLE=pr-advisor`. Environments carry their own variables and a new session picks them up (E2). The model
can't change the environment of the Claude Code process, a PR can't set it, and day sessions keep the current
environment (`env_018T14C2tnr6oAhPw1LRLadK`) without it. Catch: `update_trigger` can't change a routine's environment
or bound session (E10), so both routines move to the new environments through claude.ai (or are recreated), and
the night routine gets a new bound session.
Fallback: the routine prompt arms a role file first (`node .claude/scripts/role.mjs night-shift` writes the gitignored
`.gate/role`); once the file exists the hook refuses to change or remove it. That needs only a prompt edit, but the
arming itself is an instruction.

### `.claude/hooks/routine-guard.mjs` (PreToolUse, matcher `*`)
Exit 2 with a one-line reason = deny (stops the call before permission rules). With a role set, any internal error
also denies (fail closed); with no role it allows (fail open).
- **All sessions:** `mcp__github__merge_pull_request` without a 40-hex `expectedHeadSha` → deny (CLAUDE.md:93,
  advisor.md:120-121; E3).
- **Both routines:**
  - `AskUserQuestion`, `EnterPlanMode` → deny ("no one is watching: write it under Decisions needed").
  - `mcp__Claude_Docs__*` → deny (P18 note: the org policy is ask, so a call would wait).
  - The claude-code-remote tools that change routines or sessions (`create/update/delete/fire_trigger`,
    `send_later`, `create_session`, `send_message`, `archive/unarchive/interrupt_session`, `set_session_*`,
    `add_repo`, `watch_url`) → deny (A43).
  - `WebFetch` to a host that is not one of settings.json's `WebFetch(domain:…)` entries → deny (N54; CLAUDE.md:197-198).
  - Bash `curl`, `wget`, `npm install|i|add|update|uninstall|link`, `yarn`, `pnpm`, `pip|pip3 install`,
    `python* -m pip install`, `cargo install` → deny (N52, A40).
  - Bash `npm ci`: deny unless it is the exact allowed form, with no `cd` in the command, and
    `git diff --quiet origin/main -- tools/package.json tools/package-lock.json` passes in the session folder (A40).
  - Edit / Write / NotebookEdit on a protected path → deny. Protected: `CLAUDE.md`, `.claude/**` (except
    `.claude/agent-memory-local/**`, see AGENT-MEMORY), `.github/**`, `tools/gate.mjs`, `tools/smoke.mjs`,
    `tools/battery.js`, `tools/gate-globals.json`, `tools/package.json`, `tools/package-lock.json`, any `.npmrc`
    (N41–N43, A36; advisor.md:39-42; NIGHT_SHIFT.md:44). This replaces the Edit denies D3–D5 suggest, which would also
    block every kit change-set in day sessions.
  - Bash that writes a protected path (a `>`/`>>` target, `tee`, `sed -i`, `cp`/`mv`/`ln`/`rm`/`git rm`/`git mv` with it
    as an argument) → deny (best effort; the commit and push checks below are the real backstop).
- **Night shift only:**
  - `mcp__github__merge_pull_request` → deny (N36; NIGHT_SHIFT.md:43).
  - `git fetch` of a `pull/` ref or of any URL other than `origin` → deny (N56; NIGHT_SHIFT.md:16-17).
  - `git commit`: deny if `git diff --cached --name-only` lists a protected path (N41–N43).
  - `git push` and `checkpoint.mjs save|watch`: deny unless the pushed branch is the current branch, and deny if
    `git diff --name-only origin/main...HEAD` lists a protected path (N55 in part; N41–N43).
- **PR Advisor only:**
  - Bash `git add`, `git commit`, `git push`, `node .claude/scripts/checkpoint.mjs …`, `gh pr create|ready|edit` →
    deny (A34, A35; advisor.md:130-131,139).
  - `mcp__github__create_pull_request`, `update_pull_request`, `create_branch` → deny.
  - Edit / Write outside `.checkpoints/**`, `.gate/**` and `.claude/agent-memory-local/**` → deny (A36).
- **Tests** (`node --test .claude/hooks/routine-guard.test.mjs`): a table of role × tool × input with the expected
  decision. It must contain every needed command in section 7 (all pass) and every "never" row in N36–N56 and
  A30–A43 (all denied).

### `PermissionRequest` backstop (same file, second entry)
With a role set, answer every permission request with `decision.behavior = "deny"` and a message ("routine: this
would have waited for a human; record it under Decisions needed"). Without a role, return nothing (normal prompt).
This turns every prompt nobody predicted into a refusal: the classifier's pause after 3 blocks in a row or 20 in all,
first reads outside the working folders, MCP connector approvals (D8). Untested in routines (U-6).

## 6. Procedure changes
- **P-1 PR Advisor reviews from main's rules** (advisor.md step 1): first command
  `git fetch -q origin main && git switch --detach origin/main` in the main checkout. Settings are reloaded when
  they change on disk, so main's permissions, hooks and agents take over from the PR head's (U-1). If
  `git diff --quiet origin/main <headSHA> -- .claude CLAUDE.md .github` fails, the run runs no PR code (no npm, no
  gate, no preview). It labels the PR needs-mustafa and asks for an advisor review from a day session, because its
  own session started under that PR's rules (A42). If the PR touches the gate's own files (tools/gate.mjs, smoke.mjs,
  battery.js, gate-globals.json, package*.json, .npmrc), the gate the run executes is the PR's own. Its verdict is
  then at most needs-mustafa (advisor.md:39-42, escalation category 3).
- **P-2 Review worktrees in `.gate/`**: `git worktree add -q .gate/pr<n> <headSHA>`, not the scratchpad. `.gate/`
  is gitignored and inside the working folder, and every allow pattern above uses that path.
- **P-3 Install from main's lockfile only, once per container**, from the main checkout while it is at origin/main
  (PR Advisor after P-1; night shift before its first `git switch -c`):
  `npm ci --prefix tools --ignore-scripts --no-audit --no-fund` then `npm rebuild gl --prefix tools`. Link it into
  each worktree: `ln -s ../../../tools/node_modules .gate/<wt>/tools/node_modules`. Never `npm ci` inside a PR
  worktree. Why not plain `--ignore-scripts` alone: `gl` (preview.js, sim.js) needs its install script, and it is the
  only package with one (E6). If a PR changes the toolkit's dependencies, its gate runs against main's install, and
  any failure goes on Mustafa's list as "can't verify headless".
- **P-4 Skip `ls /opt/pw-browsers`.** Run the gate. Only if 5a reports a missing browser, run P-5's command.
- **P-5 Playwright without npx**: `node tools/node_modules/playwright/cli.js install chromium`. Update CLAUDE.md:159,
  docs/START_HERE.md:109 and tools/README.md:11,53 (CI keeps `npx --no`; settings don't govern CI).
- **P-6 No `git -C`, no `cd <worktree> && git …`**: run git from the main checkout and name the commit
  (`git diff origin/main...<sha>`, `git ls-tree -r --name-only <sha> -- .checkpoints`, `git log <sha>`). Run tools
  as `cd .gate/pr<n> && node tools/gate.mjs origin/main` (the gate checks the tree it lives in, gate.mjs:32-33).
  Run sims with main's tools on either tree: `xvfb-run -a node tools/preview.js .gate/pr<n> eval tools/sim.js`
  (head) and `xvfb-run -a node tools/preview.js . eval tools/sim.js` (base). This also satisfies advisor.md:82.
- **P-7 Name the branch on every push**: `git push -u origin cs/<slug>`, then `git push origin cs/<slug>`; never a bare
  `git push` or `HEAD`.
- **P-8 Night shift stages explicit paths** (`git add <paths>`), never `-A` or `.`.
- **P-9 No cleanup in routines** (the container is thrown away). render-ab names base worktrees `.gate/base-<sha7>`
  and reuses one only if its SHA matches (render-ab SKILL.md:8), so nothing needs `worktree remove --force` or `rm -rf`.
- **P-10 Checkpoint deletion** uses `git rm -r .checkpoints/<slug>.md .checkpoints/<slug>/` (CHECKPOINTS.md:132).
  **Saver stop** uses TaskStop or `pkill -TERM -f checkpoint.mjs` (CHECKPOINTS.md:92-93).
- **P-11 GitHub writes go through MCP only.** The PR Advisor merges with `mcp__github__merge_pull_request`
  (`expectedHeadSha` = the approved SHA). Reads use MCP, or `gh api` without quotes around the path. The three labels
  already exist (E4): advisor.md:110 changes from "create these labels" to "they exist; report one that is missing".
- **P-12 Agent memory**: switch the 11 `memory: project` agents to `memory: local` (gitignored
  `.claude/agent-memory-local/`), so no session's memory writes touch tracked `.claude/` files (N30; decision
  AGENT-MEMORY). No memory file is tracked on main today (E9).
- **P-13 Text fixes in the same change-set**: NIGHT_SHIFT.md:3 says "00:10 to 07:40", but runs now go to 08:10
  (NS-1 (b): the 08:10 run only finishes). NIGHT_SHIFT.md:43 "there are no approval prompts tonight" becomes
  true. CLAUDE.md:197-198 "any other fetch asks first (and is refused at night)" becomes "day sessions: the
  classifier or a prompt; routines: refused by the role guard".

## 7. Every needed command against the deny list
All pass: N1–N35 and A1–A29 in the forms of sections 2 and 6. The cases worth naming:
- `git switch -c cs/<slug> origin/main` vs `git switch -C *`: passes only with case-sensitive matching (U-3).
- `git merge-base HEAD main` vs `git merge *`: passes.
- `git push -u origin cs/<slug>` vs `git push * main`, `* HEAD`, `* :*`, `*+*`, `* -d *`: passes (a slug ending in
  `-main` has no space before `main`).
- `git branch --show-current` / `-a` / `-r` / `--format=…` vs the branch denies: passes.
- `git reset -q HEAD -- .checkpoints` (E7): passes.
- `gh api --hostname github.com repos/Mergincanli/Shamble-Frontlines/pulls/<n>` vs the `gh api` write denies: passes.
- `npm audit --prefix tools` vs `npm audit * fix*`: passes.
- `node -e …`, `node .gate/x.mjs`, `node --check f`, `python3 tools/village.py` vs `Bash(node)`, `Bash(node -)`,
  `Bash(python3)` (exact): pass.
- Lost on purpose, with a replacement: `git -C <wt> status` (P-6), `npx --no -- playwright install chromium` (P-5),
  `git checkout <branch>` (`git switch`), `rm -rf .gate/base` (P-9), `gh pr merge` (P-11).
- Hook vs needed: `node .claude/scripts/checkpoint.mjs save` (night) passes the push check when the branch is
  current and touches no protected path. `git show origin/main:.claude/NIGHT_SHIFT.md` is a read, not a write.
  Notes under `.checkpoints/` and `.gate/` pass for the PR Advisor.

## 8. Unresolved
- **U-1** Does `git switch --detach origin/main` really swap the live settings, hooks and agents (P-1)? It relies on
  "settings reload on change". Untested; until tested, a PR's own `.claude/` governs its review run.
- **U-2 Security finding for security-auditor and Mustafa.** A PR-triggered run starts from the PR head, so that
  head's `.claude/settings.json`, hooks (any SessionStart hook runs before the prompt is read) and agents are live
  (A42; get_session source `refs/pull/26/head`). PR code that the gate, smoke and preview run is a child process, so
  no permission rule sees it, and it runs with the container's GitHub access. Main is protected by the ruleset; PR
  branches are not. Open: does the routine fire on fork PRs, and can its trigger be limited to same-repo PRs?
  Server-managed settings would be the only layer a PR can't change (needs a Team or Enterprise plan; not confirmed for
  routines).
- **U-3** Are rule patterns case-sensitive? If not, `git switch -C *` blocks the needed `git switch -c`. Drop that one
  rule then: its damage stays local, since publishing it needs a force push, which is denied.
- **U-4** Every `git fetch` (13 of 13 calls in 4 sessions, P4) went to the classifier although `Bash(git fetch *)`
  is allowed, while `cd <repo> && git show …` was decided by the allow rules (E11). Likely cause: git commands that
  write `.git` (a protected path) go to the classifier anyway. If so, fetch, add, commit, switch, push and worktree
  stay classifier-reviewed in auto mode whatever we allow. Not a stall unless it blocks (3 in a row or 20 in all),
  and the backstop covers that.
- **U-5** Auto mode may drop `Bash(node .gate/*)`, `Bash(npm ci …)` and `Bash(npm rebuild …)` as interpreter or
  package-manager rules (the two docs pages disagree). They then go to the classifier, which allows lockfile installs
  by default.
- **U-6** Untested in a routine: do the environment's variables reach hook processes (E2 says sessions get them)?
  Does a PreToolUse exit 2 stop MCP tools whose org policy is ask, and `AskUserQuestion`? Does a PermissionRequest
  hook run in routines, and is its deny final?
- **U-7** No night run has ever reached a write step (P-runs: 50 calls, all read-only), and only one PR Advisor run
  exists. The write paths are a static audit, not observed behaviour.
- **U-8** Instruction-only even after this change: pushing a non-fix after a PR is marked ready (N55); merging only
  on Mustafa's go for the approved head (A30; the hook enforces the SHA, not the go); skipping fork PRs (A42); Agent
  spawns (classifier). A determined bypass through an interpreter (`python -c`) is stopped only by the classifier and
  the GitHub ruleset.
- **U-9** The universal denies also bind day sessions: no `git -C`, `git checkout`, `git restore`, local branch
  deletion, `git merge`, `rebase`, `--amend`, `gh pr merge`, `gh auth`, `npx`, `npm run`/`test`, `bash -c`. Each has a
  replacement or is Mustafa's to do by hand (decision DAY-COST).
- **U-10** The stuck run stays REQUIRES_ACTION until someone answers or stops it. The docs give no timeout.
- **U-11** `npm rebuild gl` (P-3) runs `gl`'s install script. `prebuild-install` downloads a prebuilt binary from
  GitHub releases, and the lockfile's integrity hash does not cover that download (if it fails, `node-gyp` builds
  from source, which needs a compiler and Python; untested here). The exposure is the same as today's `npm ci`, and
  main's reviewed lockfile pins the version, but it is not zero. The alternative is no `gl` in routines, which means
  no headless sims or renders at night or in PR reviews. This needs the security-auditor's view.
- **U-12** Wildcard allows also match extra arguments: `git rm -r .checkpoints/*` matches
  `git rm -r .checkpoints/x CLAUDE.md`, and `ln -s … .gate/*/tools/node_modules` is the same kind of rule. For
  routines, the role guard's Bash protected-path check covers the protected files. For day sessions, nothing does,
  but git keeps the history.

## 9. New evidence from this step
- E1: read_documentation `session.permissions`: cloud sessions ignore `defaultMode` `bypassPermissions` and `dontAsk`
  in a settings file, and run file edits without asking in every mode.
- E2: read_documentation `environment.secrets`: an environment holds its own variables; a new session picks them up.
- E3: `mcp__github__merge_pull_request` takes `expectedHeadSha` (tool schema).
- E4: labels `ready-to-merge`, `needs-mustafa`, `changes-requested` exist (get_label, colour `ededed`).
- E5: the PR Advisor's own checkout had no `tools/node_modules`: step 7 (`ls tools/node_modules … && echo has_nm;
  node --version`) printed only `v22.22.0`.
- E6: `tools/preview.js:8` requires `gl`; CI installs with `--ignore-scripts` (gate.yml:34,65) and runs only gate and
  smoke, which don't need `gl`; `tools/package.json` has no scripts. In the lockfile only `gl` 8.1.6 and `fsevents`
  have install scripts (`hasInstallScript`); `fsevents` is macOS-only and is not installed on Linux, so `gl` is the
  only one that runs here. `gl`'s script uses `prebuild-install`, with `node-gyp` as its fallback.
- E7: checkpoint.mjs:174 advises `git reset -q HEAD -- .checkpoints`.
- E8: `npx` appears in CLAUDE.md:159, docs/START_HERE.md:109, tools/README.md:11,53.
- E9: no `.claude/agent-memory/` file is tracked on main.
- E10: `update_trigger` takes name, cron, enabled, model, prompt and run_once_at only: no environment and no bound
  session.
- E11: ns3 14:51:03Z `cd /home/user/Shamble-Frontlines && git show origin/main:.claude/NIGHT_SHIFT.md` → subcommandResults (allow
  rules), while every compound with `git fetch` → classifier.

## Decisions needed
- **ROLE-SIGNAL** · How does the guard know a session is a routine? · Without it, the routine-only "nevers" (night
  merges, edits to `.claude/` and CLAUDE.md, PR Advisor commits) stay instruction-only. · (a) two routine-only cloud
  environments with `SHAMBLE_ROLE`, routines moved there (Mustafa's settings; the night routine gets a new bound
  session); (b) the routine prompt arms a role file first thing (prompt edit only); (c) no role guard: only the
  static lists. · Recommend (a), with (b) if (a) can't be set up.
- **MERGE-ROUTE** · Which tool merges? · One route lets the hook enforce the approved SHA. · (a) MCP
  `merge_pull_request` with `expectedHeadSha`, `gh pr merge` denied; (b) `gh pr merge --match-head-commit`, MCP merge
  denied; (c) both. · Recommend (a).
- **AGENT-MEMORY** · Agent memory: project (tracked `.claude/agent-memory/`) or local (gitignored)? · Project memory
  writes into `.claude/`, which the night shift must never change. · (a) `memory: local` for the 11 agents; (b) keep
  project memory, exempt it in the hook, never commit it at night. · Recommend (a): cloud containers are fresh per
  session, and no memory file is tracked today.
- **DAY-COST** · Accept that the universal denies (U-9) also bind day sessions? · A deny never prompts, so a day
  session that needs one of them must use the replacement or ask Mustafa to run it. · (a) accept as listed; (b) move
  some to the role guard. · Recommend (a); revisit `npm run`/`test` and `npm install` when phase 1 adds packages.
- **PR-HEAD-SETTINGS** · How far do we trust the PR head's own `.claude/` during its review (U-2)? · A PR can loosen
  the rules of its own review, and PR code runs with the container's GitHub access. · (a) P-1 plus same-repo-only
  triggers plus a security-auditor review; (b) also server-managed settings, if the plan allows it. · Recommend (a)
  now, (b) checked.
- **TEST-RUN** · Prove it before the first night? · No routine has ever run a write step (U-7). · (a) in a day
  session in the new night environment, run the hook test table and one example of each needed command; then force
  one PR Advisor run on a docs-only test PR; (b) let the first real night be the test. · Recommend (a).

## For Mustafa
1. The PR Advisor run for #26 (`session_01FPXGo8GioEMFrCm8FhivMU`) still waits on the `npm ci` approval described in
   section 0. #26 is merged, so don't approve it: stop the run from the routine's page on claude.ai/code/routines.
2. Decide ROLE-SIGNAL when the Advisor brings the kit-v3-1 plan. If (a): create the two environments with the one
   variable each, and move the two routines to them.
