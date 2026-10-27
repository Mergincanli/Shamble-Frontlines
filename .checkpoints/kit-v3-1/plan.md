# Plan: kit-v3-1 (WRITE-GUARD, ROUTINE-EFFORT, no stalls in routines), revision 2

Approved work (issue #16 comment 6396928581, the Advisor, 2026-10-25): ROUTINE-EFFORT (b) extended; WRITE-GUARD (a)
with the secret-pattern fix (deny where routines can't answer; kit changes daytime only); DOMAINS-ACCEPT (a), the risk
recorded, effective once WRITE-GUARD is in force. The Advisor's lesson (2026-10-27): every command the night shift and
the PR Advisor need is allowed, everything they must never do is denied, nothing they hit is ask-first. NS-1 (b): runs
to 08:10. BRANCH-2 (a): `cs/kit-v3-1`, in its own worktree; it lands first if it collides with cs/v2-research.
SPEND-1 (b). Plan review round 2: APPROVE with two conditions (`advisor-verdict-2.md`), applied below. Daytime only. A rules change (escalation category 3): it merges on Mustafa's go after the advisor approves
its head. No game code (symbols +0 −0). Plan review round 1: REVISE (`advisor-verdict-1.md`); this revision applies
R1–R6, the decisions the advisor took and conditions C1–C8.

## Evidence already gathered (this folder)
Four traces (night path 56 commands, PR Advisor path 43, past runs 18 decisions, permission docs 8 answers), the
proposal, and the security-auditor's attack on it: FAIL, 2 critical, 4 high, 4 medium, 2 low (`audit-result.json`,
`security-auditor.md`). The ledger below maps every finding.

## Threat model
- Routines run unattended with Mustafa's GitHub identity (every session posts as Mergincanli). Outsiders control PR
  content (the head tree, `.claude/` and `.mcp.json` included), PR text, comments on the public issue #16 and fetched
  pages.
- **A PR's own rules govern its review run** unless something outside the repository changes that: a PR-event run
  starts from the PR head. The author filter keeps outsiders out, but **one of our own sessions, tricked by a page or a
  comment, can open a PR that changes the rules (the guard included), and its review run would then start under the rules
  that PR wrote.** PR-HOME (L0) closes this; if it doesn't work and managed settings aren't available, RESIDUAL goes to
  Mustafa.
- **Programs a session runs hold its GitHub access**, and no permission rule or hook sees what they do. In phase 0
  routines run no program that didn't come from main (R2).
- Static rules match spellings, not intent: they stop accidents and the common forms.
- **The GitHub ruleset on main (25069670) is no backstop against a bad merge**: it requires a pull request and blocks
  force pushes and deletion, but it needs 0 approvals, and its `gate` and `smoke` checks come from the PR's own
  `.github/workflows/gate.yml`, so a PR that edits that file can turn them green.

## Design
**L0, outside the repository (Mustafa's settings; the session can't set them):**
- PR Advisor trigger filters: author is Mergincanli, and not a draft (now).
- ROLE-SIGNAL (advisor: (a)): two routine-only cloud environments, `SHAMBLE_ROLE=night-shift` / `pr-advisor`; the
  routines move there (the night routine gets a new bound session) only as C5 allows.
- **PR-HOME:** the PR Advisor environment's setup script puts the checkout on `origin/main` before Claude Code starts,
  so main's settings, hooks, agents and CLAUDE.md govern every review; the PR is read only through git and the GitHub
  tools. The TEST-RUN proves the script runs after the clone. P-1 (`git switch --detach origin/main`) stays as backup.
- MANAGED: server-managed settings, if Mustafa's plan has them (his question).
**L1, static settings (all sessions), `.claude/settings.json`:** the proposal's allows narrowed per the attack (exact
forms; no `git grep *` or `git ls-remote *` wildcards; `git fetch` narrowed; no `node .gate/*`, no `ln -s`, no `npm
rebuild gl`); its universal denies **except `git merge` and `mcp__github__update_pull_request_branch`**, which move to
the guard (R5); Read denies for `.git/config`, `~/.gitconfig`, `/proc/**`. **The ask list ends empty.** `effortLevel:
"high"`. Deny messages say what to run instead.
**L2, the guard, `.claude/hooks/routine-guard.mjs`:** PreToolUse on every tool, plus PermissionRequest and ConfigChange,
registered as `node "${CLAUDE_PROJECT_DIR}/.claude/hooks/routine-guard.mjs"` with `onFailure: "block"` (the syntax-gate
hook gets the same path fix). Rules, with their scope:
- **Every session:** the git and gh argument checks (R3a): a long option not spelled in full is denied; bundled short
  flags that include a dangerous one are denied; leading global options are parsed. The merge check (R3b):
  `expectedHeadSha` present, head repository = base repository, `gate` and `smoke` success on that SHA. On an internal
  error (R3c): allow when no role is set, so a guard bug never locks a day session out of fixing it.
- **Both routines** (role set; internal error → deny): no `git merge`, no `update_pull_request_branch` (R5);
  interpreters run only a tracked file whose working copy equals `origin/main` (the gate, `checkpoint.mjs`,
  `repo-map-check.mjs`, the kit's tests); denied: `tools/preview.js`, `tools/pageshot.js`, `node .gate/*`, `node -e`, and
  `--pre`, `eval`, `evalshot` snippets (R2, ROUTINE-CODE-EXEC: no in phase 0); npm refused when any `.npmrc` exists in the
  tree, in `tools/` or in home; per-role allowlists for the GitHub MCP tools (any `mcp__github__*` not listed is denied;
  `update_pull_request` may not change a base); WebFetch only to allow-listed hosts, with `github.com`,
  `raw.githubusercontent.com`, `docs.rs` and `crates.io` limited to this repository and named repositories and crates
  that start from the sources cited in `docs/research/` (C8); Edit, Write and Bash writes on protected paths denied
  (`CLAUDE.md`, `.claude/**`, `.github/**`, `.mcp.json`, the gate's files, any `.npmrc`; R4); no `AskUserQuestion`, plan
  mode, routine or session tools, or connector tools whose org policy is ask; PermissionRequest → deny plus interrupt,
  with a line for "Decisions needed"; ConfigChange → block project and local settings changes.
- **Night shift only:** no merge; commits and pushes refused if they touch a protected path; push only the current
  branch; `npm` only as the exact `npm ci --prefix tools --ignore-scripts --no-audit --no-fund` from main's lockfile
  (SETUP); `gh pr ready` only for its own PR; comments only on #16 and its own PRs.
- **PR Advisor only:** no interpreter at all, no install (PR-CODE (a), SETUP); no commit, push, checkpoint save or PR
  edit; comments and labels on PRs only, editing only its own review comment; merges refused when the diff touches a
  protected path (R3b, R4).
**L3, procedures:** `advisor.md` PR-review step 0 and NIGHT_SHIFT.md start check 0: if `SHAMBLE_ROLE` isn't the
expected value, stop and report (C1). `advisor.md`'s "Never: approve without running the gate yourself" becomes: the
gate and smoke green on the approved SHA, run by the advisor in a day session, or by CI when the PR touches neither
`.github/` nor the gate's files; a PR that changes `shamble-*.js` or the pages, or needs a sim, render or measurement,
is needs-mustafa ("verify in a day session") unless the TEST-RUN shows the PR Advisor can read the CI gate log (C2).
The night shift runs a queue item or a handoff only if it links an advisor APPROVE naming a full SHA. The proposal's
other procedure changes: playwright without `npx`, no `git -C`, the branch named on every push, explicit `git add`
paths, `git rm` for checkpoint deletion, the saver stop forms, MCP-only GitHub writes, the labels exist, agent memory
`local`, and the text fixes (NIGHT_SHIFT 00:10 to 08:10, "no approval prompts tonight", CLAUDE.md's web-fetch sentence).
**L4, ROUTINE-EFFORT:** `.claude/agents/builder.md` (Opus, effort high; it lists its tools, works only in its worktree,
never commits, pushes or posts; C7) does every change at night; the night's main session only orchestrates;
`effortLevel: "high"`; CLAUDE.md "Models and effort" says routines now start at high through the project settings.
**L5:** the percent-escape secret fix in `checkpoint.mjs` with a test; the DOMAINS-ACCEPT risk written into CLAUDE.md's
web-domains rule.

## Findings ledger (the attack's 12, and its "must change" bullets)
| Finding | Where it lands |
|---|---|
| C1 PR head governs its review | L0 trigger filters + PR-HOME + MANAGED question; threat model; RESIDUAL if PR-HOME fails; P-1 as backup |
| C2 programs hold GitHub access | R2: routines run only main's tracked files; PR Advisor no interpreter; ROUTINE-CODE-EXEC no; OS sandbox deferred to phase 1 |
| H1 git abbreviations | L1 exact forms, no grep/ls-remote wildcards, narrow fetch; L2 parser for every session (R3a); C3b real-git cases |
| H2 guard fails open | `${CLAUDE_PROJECT_DIR}` path, `onFailure: block`, ConfigChange block, deny plus interrupt. **sha256 pin: deferred**: with the hook registered in the repository, a program that could rewrite the guard already holds the GitHub access, so the pin buys little; it moves with the registration if MANAGED is (a) |
| H3 the Mergincanli channel | per-role comment allowlists; queue items need an advisor APPROVE naming a SHA; as strong as C1's fix for the PR Advisor |
| H4 merge check | R3b: every session SHA + same repo + checks; PR Advisor protected paths; `gh pr merge` denied (MERGE-ROUTE (a)) |
| M1 npm supply chain | night: exact `npm ci`, main's lockfile, `.npmrc` refusal; PR Advisor installs nothing; no `gl` in routines; setup-script install later |
| M2 wildcard allows | dropped: `node .gate/*`, `ln -s`, `git grep *`, `git ls-remote *`; preview.js denied to routines (R2) |
| M3 WebFetch on open hosts | routine host + path limits (C8) |
| M4 secret reads | Read denies for `.git/config`, `~/.gitconfig`, `/proc/**`; `git config` only with exact read keys |
| L1 other allows | `gh pr ready` own PR only; gh api write flags parsed by the guard; `pkill` form replaced by TaskStop |
| L2 prompts left | PermissionRequest deny plus interrupt; TEST-RUN rows for connector, protected-path write, repeated blocks |

## Files
`.claude/settings.json` · `.claude/hooks/routine-guard.mjs` (new), `routine-guard.test.mjs` (new),
`settings.test.mjs` (new) · `syntax-gate.mjs` registration · `.claude/agents/builder.md` (new) · the 11 agents with
`memory: project` · `advisor.md` · `NIGHT_SHIFT.md` · `CHECKPOINTS.md` · `CLAUDE.md` · `checkpoint.mjs` and its test ·
`tools/README.md`, `docs/START_HERE.md` (the `npx` lines; not gate files) · `docs/REPO_MAP.md` · STATUS and ROADMAP
(docs-keeper: the v2 pivot, kit-v3-0, R7, kit-v3-1, and C5's switch-on rule) · the report. The gate's own files stay
untouched.

## Build order (C6: one commit, gate run and checkpoint save per step)
1. The guard and its tests. 2. The settings and the settings test. 3. The procedures, and every doc instruction that
would now hit a deny. 4. The builder agent, `effortLevel`, memory local, the "Models and effort" sentence. 5. The
percent-escape fix and its test. Then reviews once on the final diff, the TEST-RUN part A, the report, and the PR.

## Evidence
- The guard's test table (role × tool × input → decision): every needed command of the four traces passes; every never
  is denied, including the abbreviated git forms; mutation runs (each rule removed makes its own test fail).
- The settings test (C3a): the ask list is empty; every needed command form matches an allow rule and no deny rule;
  every never is denied by a static rule or by the guard. Labelled an emulation of the documented matching.
- Real-git option cases (C3b): the option table comes from the container's real git, and a sample of its cases is run
  on throwaway repositories (no network), as the auditor did. The builder worktree rows (C3c), the error-mode rows (C3d).
- `checkpoint.mjs` tests with the percent-escape case.
- **TEST-RUN (C4), a table of expected against observed.** Part A, in a day session in the night environment on
  cs/kit-v3-1, before the PR is marked ready: U-1, U-3 to U-6; PR-HOME (in the PR Advisor environment); the role
  variable reaching hooks and subagents; one builder spawn in a worktree; a connector call, a protected-path write and a
  repeated-block case, each ending in deny plus interrupt (never a wait), with where the stop is reported: the
  session's final message and the run's status comment if it can still post, else the fallback (night shift: the next
  run's "Cut off" note, NIGHT_SHIFT rule 4; PR Advisor: a ready PR with no review comment is the signal), written into
  the procedures before switching on; the session's effort; a read-only check whether a child process holds the
  GitHub identity; whether the PR Advisor can read the CI gate log; the role-missing stop. Results go in the report.
  Part B, after merge: one PR Advisor run on a docs-only test PR, before it reviews anything else, with two more rows
  (round-2 conditions): **the trigger fires on the test PR's ready event** (it did not fire when PR #27 was marked
  ready; if it doesn't, the infra-engineer finds out why before switching on), and **the run's first command shows HEAD =
  `origin/main`**, not the test PR's head (a day session in that environment starts on main anyway, so only a real PR
  run proves PR-HOME; if it shows the PR head, RESIDUAL goes to Mustafa before the PR Advisor is switched on).
- Gate PASS (symbols +0 −0), repo map check.

## Switching on (C5)
Neither routine does real work until its part of the TEST-RUN passes. The PR Advisor stays off (Mustafa turns it off
now); the night routine moves to its new bound session only after part A. DOMAINS-ACCEPT takes effect once this PR is
merged and part A has passed. STATUS records this.

## Reviewers
security-auditor (required; the final diff, the parser included, with the R1 and R2 leftovers) · test-engineer · 
infra-engineer (environments, setup scripts, PR-HOME) · advisor (the plan; the PR). Rounds stop at nits.

## Risks
1. PR-HOME may not work (the setup script may run before the clone): then RESIDUAL.
2. Behaviour the docs leave open (U-1, U-3 to U-6): the TEST-RUN settles it before anything relies on it.
3. Day-session cost (DAY-COST (a)): no `git -C`, `git checkout`, `git restore`, local branch deletion, `rebase`,
   `--amend`, `gh pr merge`, `npx`, `npm run`/`test`, `bash -c`; each has a replacement; `git merge` stays for day
   sessions (R5).
4. The guard is new code that every tool call passes through: its test table, mutation runs, the settings test, the
   TEST-RUN and the security review carry the proof; R3c keeps a broken guard from locking out a day session.
5. Routines lose headless sims and renders in phase 0 (R2): acceptable, as nothing at night needs them now; revisit with
   the OS sandbox in phase 1.

## Decisions needed (Mustafa)
- **MANAGED:** does his Claude plan include server-managed settings? Why it matters: it is the only place rules can
  live that a pull request can't change. (a) yes: the guard's registration and the universal denies go there too;
  (b) no: rely on PR-HOME. Check the plan page; on a personal plan the answer is (b).
- **RESIDUAL** (only if PR-HOME fails and MANAGED is (b)): should the PR Advisor keep merging the PRs it approves? Why it
  matters: it would review each PR under that PR's own rules, so one session tricked by a web page could open a PR that
  changes the rules and get it merged with no human. (a) keep merging "ready" PRs, risk recorded; (b) review and label
  only, and Mustafa says "merge #n" for every PR. Recommended (b), until a layer the PR can't change exists.

## For Mustafa
1. Turn the PR Advisor routine off until the TEST-RUN passes (the session can do it on his word).
2. Add its two filters: author is Mergincanli, and not a draft.
3. When the session sends the steps: create the two routine-only environments and the PR-HOME setup script.
4. Answer MANAGED.
5. Later: reconfirm the merge of this PR ("merge #<n>") once the advisor has approved its head.
