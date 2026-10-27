# Advisor plan review of kit-v3-1, round 1 (2026-10-27): REVISE
General agent reading main's advisor.md (this session started before that file last changed). Verbatim, as returned.

**Verdict: REVISE.** The design answers most of the security attack. It leaves the two worst paths open without saying
so, it counts on a GitHub backstop that doesn't hold, and two of its rules would block work it needs. Every fix is a
change to the plan text, not a new direction. Once they are in, a short re-review of the changed parts should be enough
to approve.

### The attack's critical and high findings against the plan
1. (critical) The PR's own rules govern its review. For outsiders this is answered: the author filter, P-1 as backup,
   and the managed-settings question. It is not answered for our own sessions. Every session posts as Mergincanli, so a
   PR opened by a session that was tricked still passes the author filter. Its review run then starts under that PR's
   own `.claude/` (the guard included), `CLAUDE.md` and `.mcp.json`. The guard's rule "a PR that touches protected
   paths is never merged" lives in a file that PR can change. The attack asked for this leftover to be written into the
   threat model. The plan doesn't, and its risk 3 only covers the case without the author filter. Fix: R1.
2. (critical) Programs a session runs hold its GitHub access. The PR Advisor side is answered: it runs no PR code and
   relies on CI. The night side is missing. The night shift runs programs it wrote (logic harnesses, `preview.js` with
   `eval` or `--pre`) with Mustafa's GitHub access, and no hook sees what they do. The plan dropped the attack's
   decision on whether routines run code they didn't get from main (ROUTINE-CODE-EXEC), and its `preview.js` limits
   (finding 8). Fix: R2.
3. (high) Git accepts abbreviated options. Answered: exact command forms, a narrower fetch, and a parser in the guard.
   The parser's scope is unclear. Fix: R3.
4. (high) The guard can fail open. Answered: absolute path, `onFailure: "block"`, the ConfigChange block, and deny plus
   interrupt. The guard script's sha256 pin was dropped without a word. Fix: R6.
5. (high) Every session posts as Mergincanli. Answered: comment limits per role, and the "links an advisor APPROVE"
   rule for the queue. For the PR Advisor this is only as strong as R1.
6. (high) The merge check proves nothing about approval. Answered. Its scope needs one change. Fix: R3.

### Scope: one change-set, no split
- The urgent stall is cheaper to stop in Mustafa's settings than in a PR: he stops the stuck run and turns the PR
  Advisor off until the test run passes.
- A first PR that only removes stalls would be worse: it would bring the PR Advisor back with merge rights while it
  still runs under the PR head's rules, so it would have to wait for the guard anyway.
- A first PR would unlock nothing at night: DOMAINS-ACCEPT ties night research to the guard, and the queue was empty on
  both nights since.
- Two rules PRs cost Mustafa two reconfirmations. One PR costs one.
- Size is handled by the build order in C6.

### Decisions I take (technical choices inside the vision; Mustafa reconfirms them with his go on this rules PR)
- ROLE-SIGNAL: (a), routine-only environments with `SHAMBLE_ROLE`; a role check at the start of each routine (C1).
- SETUP: the PR Advisor installs nothing (it runs no code); the night shift runs only the exact
  `npm ci --prefix tools --ignore-scripts --no-audit --no-fund`, from main's lockfile, guarded; no `gl` and no
  `npm rebuild gl` in routines; installing from the setup script is optional hardening for later.
- PR-CODE: (a), and the guard enforces it. The PR Advisor runs no interpreter. CI's `gate` and `smoke` on the head
  count only when the PR touches neither `.github/` nor the gate's files.
- MERGE-ROUTE: (a). Merges go only through the MCP merge tool with `expectedHeadSha`; `gh pr merge` is denied.
- AGENT-MEMORY: (a), `memory: local`.
- DAY-COST: (a), except R5.
- TEST-RUN: (a), required, extended per C4.
- ROUTINE-CODE-EXEC: no, for both routines in phase 0 (R2). Revisit with the OS sandbox in phase 1.

### REVISE: the exact changes
- R1. Threat model, and a new outside layer (PR-HOME). (a) Write the leftover from finding 1 into the threat model:
  one tricked session can get a PR that changes the rules reviewed, and merged, under rules that PR wrote. (b) Correct
  the ruleset line: ruleset 25069670 requires a PR and blocks force pushes and deletion, but it needs 0 approvals, and
  its `gate`/`smoke` checks come from the PR's own `gate.yml`, so a PR that edits that file can turn them green. It is
  no backstop against a bad merge. (c) Add PR-HOME to L0: the PR Advisor environment's setup script, which lives
  outside the repository, puts the checkout on `origin/main` before Claude Code starts. Main's settings, hooks, agents
  and CLAUDE.md then govern every review, and the PR is read only through git and the GitHub tools. The setup-script
  doc page doesn't say whether the script runs after the clone, so the test run must prove it. If it fails and managed
  settings aren't available, RESIDUAL goes to Mustafa. P-1 stays as the backup.
- R2. Programs routines run. For both roles, the guard lets an interpreter run only a tracked file whose working copy
  equals `origin/main` (the gate, `checkpoint.mjs`, `repo-map-check.mjs`, the kit's tests). Denied for routines in
  phase 0: `tools/preview.js`, `tools/pageshot.js`, `node .gate/*`, `node -e`, and `--pre`, `eval` or `evalshot`
  snippets. The PR Advisor runs no interpreter at all. npm is refused when any `.npmrc` exists in the tree, in `tools/`
  or in home.
- R3. Guard scope, rule by rule. (a) The git and gh argument checks bind every session, not only routines: a long
  option that isn't spelled in full is denied; bundled short flags that include a dangerous one are denied; leading
  global options are parsed. (b) Split the merge check. Every session: `expectedHeadSha` present, same repository,
  `gate` and `smoke` success on that SHA. PR Advisor only: refuse when the diff touches a protected path. (c) On an
  internal error: deny when a role is set; allow when none is.
- R4. Add `.mcp.json` to the protected-path list and the merge-refusal list.
- R5. Move `git merge` and `mcp__github__update_pull_request_branch` from the universal denies to the role guard,
  denied in both routines (cs/v2-research must take kit-v3-1 from main once it lands).
- R6. A findings ledger: each of the 12 findings and each "must change" bullet maps to adopted (where) or deferred
  (why), including the guard script's sha256 pin, `update_pull_request` base changes, `gh pr ready` own PR only,
  `git config` exact read keys, and the OS sandbox (deferred to phase 1, per R2).

### Conditions for the build
1. A role check first thing in each routine (NIGHT_SHIFT.md start check 0; advisor.md PR-review step 0): if
   `SHAMBLE_ROLE` isn't the expected value, stop and report.
2. advisor.md rewrites "Never: approve without running the gate yourself" to the new bar: the gate and smoke green on
   the approved SHA, run by the advisor in a day session, or by CI when the PR touches neither `.github/` nor the
   gate's files. A PR that changes `shamble-*.js` or the pages, or needs a sim, render or measurement, is needs-mustafa
   ("verify in a day session"), unless the test run shows the PR Advisor can read the CI gate log (the symbol diff).
3. More evidence: (a) a settings test (ask list empty; every needed command form matches an allow rule and no deny;
   every never is denied by a static rule or the guard), labelled as an emulation of the documented matching; (b)
   option cases from the container's real git, plus a sample re-run on throwaway repos; (c) rows for the builder's
   worktree paths (`.claude/worktrees/<x>/docs/…` allowed; `CLAUDE.md`, `.claude/**`, `.github/**` inside it denied;
   `..` and symlink escapes denied); (d) error-mode rows (R3c); (e) the mutation runs.
4. The test run as a table, expected against observed: U-1, U-3 to U-6, PR-HOME, the role variable reaching hooks and
   subagents, one builder spawn in a worktree, a connector call, a protected-path write and a repeated-block case (each
   ending in deny plus interrupt, never a wait, and where that stop is reported), the session's effort, a read-only
   check whether a child process holds the GitHub identity, whether the PR Advisor can read the CI gate log, and the
   role-missing stop. Part A in the night environment, in a day session on cs/kit-v3-1, before the PR is marked ready,
   results in the report. Part B: one PR Advisor run on a docs-only test PR, after merge and before the PR Advisor
   reviews anything else.
5. Order of switching on: neither routine does real work until its part of the test run passes. The PR Advisor stays
   off; the night routine moves to its new bound session only after part A. DOMAINS-ACCEPT takes effect once the PR is
   merged and part A has passed. STATUS records this.
6. Build order, one commit, gate run and checkpoint save per step: the guard and its tests; the settings and the
   settings test; the procedures (advisor.md, NIGHT_SHIFT.md, CHECKPOINTS.md, CLAUDE.md, and every doc instruction that
   would now hit a deny); the builder agent, `effortLevel`, memory local and the CLAUDE.md "Models and effort" sentence
   on routines; the percent-escape secret fix and its test. Reviewers once on the final diff; rounds stop at nits.
7. The builder agent's file lists its tools, works only in its worktree, and never commits, pushes or posts.
8. The routines' named web lists (repositories, crates) start from the sources already cited in `docs/research/`;
   additions are a daytime kit change.

### Day-session cost
Acceptable apart from R5. Replacements: `git -C` → run git from the folder; `git checkout` / `git restore` → `git
switch`, `git show <ref>:<path>`, `git reset -q HEAD -- <path>`; `npx` → the playwright `cli.js` line; `gh pr merge` →
the MCP merge; `npm run` / `npm test` → none exist, revisit in phase 1. Day sessions lose the prompt on `npm install`
(the classifier decides); acceptable, because a new dependency changes `tools/package.json`, a gate file every review
escalates. Deny messages should say what to run instead.

### Decisions needed (Mustafa)
- MANAGED: does the Claude plan include server-managed settings? The only place rules can live that a PR can't change.
  (a) yes: the guard's registration and the universal denies also go there; (b) no: rely on PR-HOME. Recommendation:
  check the plan page; on a personal plan the answer is (b).
- RESIDUAL (only if the test run shows PR-HOME doesn't work and MANAGED is no): should the PR Advisor keep merging the
  PRs it approves? (a) keep auto-merge for "ready" PRs and record the risk; (b) review and label only, Mustafa says
  "merge #n" for every PR. Recommendation: (b), until a layer the PR can't change exists.

### For Mustafa
1. Stop the stuck PR Advisor run (session_01FPXGo8GioEMFrCm8FhivMU). [Since then: the run shows ABANDONED.]
2. Turn the PR Advisor routine off until kit-v3-1's test run passes.
3. Add two filters to the PR Advisor routine: Author is Mergincanli, and Is draft is false.
4. When the implementer sends the steps: create the two routine-only environments (`SHAMBLE_ROLE=night-shift`,
   `SHAMBLE_ROLE=pr-advisor`; the PR Advisor one also gets the PR-HOME setup script), and move the routines there.
5. Answer MANAGED.
6. Later, once the advisor has approved kit-v3-1's head, reconfirm its merge by saying "merge #<n>" in this session.
