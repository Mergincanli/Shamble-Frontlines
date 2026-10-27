# Plan: kit-v3-1 (WRITE-GUARD, ROUTINE-EFFORT, no stalls in routines)

Approved work (issue #16 comment 6396928581, the Advisor, 2026-10-25): ROUTINE-EFFORT (b) extended (code changes at
night run through a high-effort agent; the night's main session only orchestrates; an Opus builder agent with edit
rights; a default effort of high if settings support one); WRITE-GUARD (a) with the secret-pattern fix (use deny where
routines can't answer a prompt; kit changes daytime only); DOMAINS-ACCEPT (a), the risk recorded, effective once
WRITE-GUARD is merged. The Advisor's lesson (2026-10-27): every command the night shift and the PR Advisor need is
allowed, everything they must never do is denied, nothing they hit is ask-first; audit both paths. NS-1 (b): runs to
08:10. Branch `cs/kit-v3-1` (BRANCH-2 (a)), in its own worktree. Daytime only. A rules change: escalation category 3,
so it merges on Mustafa's go after the advisor approves its head. No game code (symbols +0 −0).

## Evidence already gathered (the audit, `.checkpoints/kit-v3-1/` once the branch exists)
- Four traces: night-shift path (56 commands), PR Advisor path (43), past runs (18 permission decisions mined), the
  permission docs (8 answers with quotes).
- A proposal (allow, deny, the 16 ask rules, a role guard, 13 procedure changes, 12 open items).
- A security-auditor attack on it: **FAIL**, 2 critical, 4 high, 4 medium, 2 low. This plan adopts its fixes:
  1. (critical) A PR-event run starts from the PR head, so the PR's own `.claude/`, hooks, `CLAUDE.md` and
     `.mcp.json` are live before the advisor reads anything.
  2. (critical) Code the session runs (`tools/preview.js` runs the tree's game code; a PR's own `gate.mjs`) has the
     session's GitHub access, and no permission rule sees it.
  3. (high) Git accepts abbreviated long options and bundled short ones: on throwaway repos, abbreviated forms of branch
     delete, amend, hard reset and `--upload-pack` got past the deny rules. Allowed rules skip the classifier.
  4. (high) The guard can fail open (relative hook path, no `onFailure: block`, `disableAllHooks`).
  5. (high) Every session posts as Mergincanli, so a hijacked PR Advisor could write to the night queue.
  6. (high) A merge check on "a 40-hex `expectedHeadSha`" proves nothing about approval.

## Threat model (short)
Routines run unattended with Mustafa's GitHub identity. Outsiders control PR content (the head tree, `.claude/`
included), PR text, comments on the public issue #16 and fetched pages. Static rules match spellings, not intent; they
stop accidents and the common forms, not a determined bypass. The real backstops are: no outsider PR ever starts a
session (L0), no PR code runs where merge rights are (L3), the guard parses commands (L2), and the GitHub ruleset on main
(pull requests only, `gate` and `smoke` required, no force push, no deletion).

## Design
**L0, outside the repository (Mustafa's settings; the plan lists them, the session can't set them):**
- PR Advisor trigger filters: author is Mergincanli, and not a draft. Fork and outsider PRs then never start a session.
- Two routine-only cloud environments with one variable each, `SHAMBLE_ROLE=night-shift` / `pr-advisor`
  (ROLE-SIGNAL); the routines move to them; the night routine gets a new bound session.
- Managed settings, if Mustafa's plan offers them: the only layer a repository can't override (MANAGED).
- An environment setup script that installs the toolkit from main's lockfile, outside the repository (SETUP), so no
  routine runs `npm` at all. Until then, one exact `npm ci` form from main's lockfile, guarded (L2).
**L1, static settings (all sessions), `.claude/settings.json`:** the proposal's allows, narrowed per the attack: no
`git grep *` or `git ls-remote *` wildcards (exact forms; the Grep tool for searching), `git fetch` narrowed to the
forms the paths use, no `node .gate/*`, no `ln -s` rule; its denies (history rewrites, branch delete and move, leading
`git -*` options, `gh pr merge`, `gh api` writes, `npx`, `npm run`/`test`, shell wrappers, MCP file writes), plus Read
denies for `.git/config`, `~/.gitconfig` and `/proc/**`. **The ask list ends empty.** `effortLevel: "high"`.
**L2, the role guard, `.claude/hooks/routine-guard.mjs`** (PreToolUse on every tool, plus PermissionRequest and
ConfigChange), registered as `node "${CLAUDE_PROJECT_DIR}/.claude/hooks/routine-guard.mjs"` with `onFailure: "block"`;
the syntax-gate hook gets the same path fix. With a role set it fails closed. It:
- parses argv for git, gh and npm, treating any unambiguous abbreviation or bundled short flag of a dangerous option as
  that option;
- holds per-role allowlists for the GitHub MCP tools (any `mcp__github__*` not listed is denied): the PR Advisor
  comments and labels on PRs only, and edits only its own review comment; the night shift comments only on #16 and on
  its own PRs;
- refuses a merge when there is no `expectedHeadSha`, when the head repository isn't the base repository, when the
  `gate` and `smoke` check runs on that SHA aren't success, or when the diff touches `.claude/`, `CLAUDE.md`,
  `.github/` or the gate's files (Mustafa merges those through a day session);
- denies the night shift any merge, and the PR Advisor any commit, push, checkpoint save or PR edit;
- denies routines Edit and Write on protected paths (`CLAUDE.md`, `.claude/**`, `.github/**`, the gate's files, any
  `.npmrc`) and Bash writes to them (best effort; the commit and push checks back it up);
- limits routine WebFetch to the allow-listed hosts, and on `github.com`, `raw.githubusercontent.com`, `docs.rs` and
  `crates.io` to this repository and named upstream repositories and crates;
- denies routines `AskUserQuestion`, plan mode, routine and session tools, and connector tools whose org policy is ask;
- PermissionRequest: with a role set, every prompt becomes deny plus interrupt, with a line for "Decisions needed";
- ConfigChange: with a role set, blocks changes to project and local settings.
**L3, procedures (`advisor.md`, `NIGHT_SHIFT.md`, `CHECKPOINTS.md`, `CLAUDE.md`):**
- **The PR Advisor never runs PR-tree code.** It relies on the CI `gate` and `smoke` results on the head. A PR that
  needs a sim, a render or a measurement is needs-mustafa ("verify in a day session"). The proposal's P-1
  (`git switch --detach origin/main`) stays as a guard against honest mistakes, written down as no boundary.
- The night shift runs a queue item or a handoff only if it links an advisor APPROVE naming a full SHA it can check.
- The rest of the proposal's procedure changes: playwright without `npx`, no `git -C`, name the branch on every push,
  explicit `git add` paths, `git rm` for checkpoint deletion, the saver stop forms, MCP-only GitHub writes, labels
  exist, agent memory `local` (AGENT-MEMORY), and the text fixes (NIGHT_SHIFT 00:10 to 08:10, "no approval prompts
  tonight", the web-fetch sentence in CLAUDE.md).
**L4, ROUTINE-EFFORT:** a `builder` agent (`.claude/agents/builder.md`, model opus, effort high, with Edit and Write) that
does every code and docs change at night in an isolated worktree; `NIGHT_SHIFT.md`: the night's main session only
orchestrates (plans, starts agents, commits, posts); `effortLevel: "high"` (L1).
**L5, the rest:** the secret-pattern fix in `.claude/scripts/checkpoint.mjs` (percent-escapes such as `%73k-` before
the token patterns) with a test; the DOMAINS-ACCEPT risk written into `CLAUDE.md`'s web-domains rule.

## Files
`.claude/settings.json` · `.claude/hooks/routine-guard.mjs` (new) and `routine-guard.test.mjs` (new) ·
`.claude/hooks/syntax-gate.mjs` registration · `.claude/agents/builder.md` (new) · the 11 agents with `memory:
project` · `.claude/agents/advisor.md` · `.claude/NIGHT_SHIFT.md` · `.claude/CHECKPOINTS.md` · `CLAUDE.md` ·
`.claude/scripts/checkpoint.mjs` and its test · `tools/README.md` and `docs/START_HERE.md` (the `npx` lines; not gate
files) · `docs/REPO_MAP.md` · the report. Not touched: the gate's own files (the PR Advisor stops running them on PR
trees instead of changing them).

## Evidence (kit change type)
- The guard's test table: role × tool × input → expected decision. Every needed command of the four traces passes;
  every "never" is denied, including the abbreviated git forms the attack found. Mutation runs: each guard rule removed
  makes its own test fail (as kit-v3-0 did for checkpoint.mjs).
- `checkpoint.mjs` tests, with the percent-escape case.
- TEST-RUN before the first unattended night: in a day session in the new night environment, the test table live,
  one example of each needed command, one protected-path write, one connector call and one repeated-block case (each
  must end in deny plus interrupt, never a wait); then one PR Advisor run on a docs-only test PR.
- Gate PASS (symbols +0 −0), repo map check.

## Reviewers
security-auditor (the final diff, adversarial, the same threat model) · test-engineer (is it proven) · infra-engineer
(environments, setup script) · advisor (this plan; the PR). Review rounds stop once findings are only nits.

## Risks
1. Rules that bind day sessions too (DAY-COST): no `git -C`, `git checkout`, `git restore`, local branch deletion,
   `git merge`, `rebase`, `--amend`, `gh pr merge`, `npx`, `npm run`/`test`, `bash -c`. Each has a replacement.
2. Behaviour the docs leave open (the audit's U-1, U-3 to U-6): whether settings reload mid-session, case-sensitive
   matching, git writes going to the classifier anyway, auto mode dropping interpreter rules, hooks in routines. The
   TEST-RUN settles them before anything relies on them.
3. Without L0, a PR still governs its own review run. The trigger filter is the cheap, immediate fix.
4. `gl`'s install script downloads a prebuilt binary that the lockfile doesn't pin (U-11); the setup script, plus a
   hash check of the binary, contains it.
5. A larger kit change-set: the guard is new code; its test table and the security review carry the proof.

## Decisions needed (through the advisor; the settings ones are Mustafa's)
- **PR-TRIGGER:** set the PR Advisor's filters now (author Mergincanli, not draft). Cheap, closes critical 1 for
  outsiders. Mustafa's UI.
- **ROLE-SIGNAL:** (a) routine-only environments with `SHAMBLE_ROLE` (recommended); (b) a role file the routine prompt
  arms; (c) no guard.
- **MANAGED:** does Mustafa's plan offer managed settings? If yes, the guard's registration and the denies go there.
- **SETUP:** an environment setup script that installs the toolkit from main's lockfile (recommended), or the guarded
  exact `npm ci`.
- **PR-CODE:** the PR Advisor never runs PR-tree code; sims, renders and measurements go to needs-mustafa (recommended),
  or it runs them and accepts critical 2.
- **MERGE-ROUTE:** the MCP merge with `expectedHeadSha` only (recommended).
- **AGENT-MEMORY:** `memory: local` for the 11 agents (recommended).
- **DAY-COST:** accept the universal denies for day sessions (recommended).
- **TEST-RUN:** prove it in a day session before the first unattended night (recommended).

## For Mustafa
Nothing in Brave. The settings in L0, once decided.
