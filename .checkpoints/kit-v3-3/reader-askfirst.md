Task: A3 (kit-v3-3) reader for Mustafa's item 1 "ask before kit and gate edits in day sessions" + item 4 (the worktree rule) + the `node .checkpoints/*` allow-list entry (plan stage, notes only).

## Plan (blocks, in order)
B1. Read routine-guard.mjs protectedRel(): list the kit and gate paths.
B2. Read guard.mjs (day-session guard), settings.json (hooks, permissions allow/ask/deny, allowed WebFetch domains), settings.test.mjs (what it asserts).
B3. Official docs: hooks (PreToolUse permissionDecision "ask"), permissions (ask rules, modes), settings; when settings/hooks load (start vs file change); quote with URLs.
B4. How "ask" behaves per mode (default, acceptEdits, plan, auto, bypassPermissions); cloud auto mode; Mustafa's PC.
B5. Edge cases: subagents/workflow agents, routines, commit/merge.
B6. Allow-list entry: STATUS kit-v3-2 item 5; allow list; recommendation.
B7. Item 4: every checkout/switch instruction in CLAUDE.md, CHECKPOINTS.md, changeset skill, advisor.md, NIGHT_SHIFT.md; wording, place, guard enforcement, tests.
B8. Write the answer: options, design, tests, questions.

## Notes
(first start: no block was finished; widened 2026-11-04 to item 4, plan rewritten)

### B1 done: kit and gate paths (routine-guard.mjs protectedRel, lines 21-22 and 27-38)
- Protected (routines may not write): any file named CLAUDE.md, .npmrc or .mcp.json at any depth (l.30); everything
  under .github/ and .git/ (l.31); everything under .claude/ (l.32-35: settings.json, hooks/, scripts/, agents/,
  skills/, rules/, CHECKPOINTS.md, NIGHT_SHIFT.md, security-patterns.json, claude-security-guidance.md) EXCEPT
  .claude/agent-memory-local/ (l.33); under .claude/worktrees/<name>/ the same rules apply to the path inside the
  worktree (l.34); GATE_FILES (l.21-22): tools/gate.mjs, tools/smoke.mjs, tools/battery.js, tools/gate-globals.json,
  tools/package.json, tools/package-lock.json.
- Routine uses: writePath() l.287-299 (Edit/Write/NotebookEdit/MultiEdit), writeTarget() l.368-381 (redirects),
  git commit (night-shift) l.532-538, git rm/mv l.540-545, protectedInBranch() for night pushes l.554-560.
- Finding: day sessions (no role) get from decide() only Bash git/gh option checks and mcp checks (l.270-272:
  `if (!role) return null` before file tools). bash() for no role only unwraps and checks git and gh (l.409-436);
  redirects and other programs are not looked at for day sessions.
- Finding: decide() returns only null or {deny, interrupt} (l.9-11, l.251); guard.mjs print() only emits deny
  (guard.mjs l.23-34). An "ask" needs a new return shape and a new print branch.
- Next: B2.

### B2 done: today's day guard, settings.json, settings.test.mjs
- guard.mjs (44 lines): print() emits only deny (l.23-34: PreToolUse permissionDecision "deny"; PermissionRequest
  behavior "deny"; ConfigChange decision "block"); a day-session exception is swallowed (l.41-42: only routines are
  refused on error, R3c). Comment l.8-12: "A session keeps the hook settings it loaded while a branch switch changes
  the files on disk" (the GUARD-FILE lesson, the freeze).
- settings.json: hooks PreToolUse matcher "*" (l.265-274), PermissionRequest "*" (l.275-284), ConfigChange (l.285-293),
  all the same node -e bootstrap with onFailure "block"; PostToolUse Edit|Write runs syntax-gate.mjs (l.294-301).
  permissions.allow l.5-150; permissions.ask is [] (l.151); deny l.152-262.
- Already denied statically (deny list): `git checkout *` l.195, `git restore *` l.196, `git reset --hard[ *]` l.178-179,
  `git switch -f*`, `--force`, `--discard-changes`, `-C` l.199-202. Already allowed: `git switch *` l.25,
  `git worktree *` l.28. No rule names `gh pr checkout` (so it asks in default mode; neither allowed nor denied).
- In the guard, for EVERY session (role or not): GIT_NEVER (routine-guard.mjs l.202-211) refuses git checkout
  ("use git switch, or git show <ref>:<path>") and git restore; git reset only unstages (l.502-508:
  `git reset -q HEAD -- <path>`); switch refuses --force/--discard-changes/--force-create and -f/-C (l.190, l.198).
  So "git checkout <ref>", "git restore" and "git reset --hard <ref>" are already refused in day sessions; the open
  holes for item 4 are `git switch <branch>`, `git switch --detach <sha>`, `gh pr checkout <n>` (gh() l.562-590 has no
  rule for it for day sessions), `git merge`/`git pull` into the main folder (changes files on disk; a merge of main
  is the normal flow), and `git worktree` (allowed; good).
- Finding: "plain file restores stay possible" is already false for git restore/checkout -- the rules' substitute is
  `git show <ref>:<path> > file` (GIT_NEVER l.203-204). Item 4 must not change that.
- WebFetch domains allowed (settings.json l.88-149): threejs.org, github.com, raw.githubusercontent.com,
  docs.github.com, github.blog, www.w3.org, w3c.github.io, gpuweb.github.io, webaudio.github.io,
  developer.mozilla.org, hacks.mozilla.org, developer.chrome.com, web.dev, webkit.org, developer.apple.com,
  caniuse.com, webassembly.org, webassembly.github.io, doc.rust-lang.org, rust-lang.github.io, blog.rust-lang.org,
  rustwasm.github.io, docs.rs, crates.io, rustsec.org, embarkstudios.github.io, rapier.rs, dimforge.com,
  bevyengine.org, tokio.rs, datatracker.ietf.org, www.rfc-editor.org, www.ietf.org, gafferongames.com,
  developer.valvesoftware.com, www.gdcvault.com, gdcvault.com, www.typescriptlang.org, vite.dev, vitejs.dev,
  vitest.dev, playwright.dev, nodejs.org, docs.npmjs.com, eslint.org, typescript-eslint.io, developers.cloudflare.com,
  docs.hetzner.com, www.hetzner.com, fly.io, vercel.com, docs.netlify.com, docs.aws.amazon.com, aws.amazon.com,
  cloud.google.com, learn.microsoft.com, docs.sentry.io, grafana.com, opentelemetry.io, prometheus.io, owasp.org,
  cheatsheetseries.owasp.org. Plus WebSearch (l.87).
- Finding: NEITHER code.claude.com NOR docs.anthropic.com is on the list: the Claude Code docs can't be fetched without
  a prompt. Reachable official sources: github.com/anthropics/claude-code and raw.githubusercontent.com (its
  CHANGELOG.md), and WebSearch snippets. Gap to record (a follow-up: add code.claude.com/docs.anthropic.com to the
  WebFetch allow list, its own decision).
- settings.test.mjs asserts: ask list empty (l.56, "every ask would stall a routine"); effortLevel high (l.57); guard on
  3 events, node -e bootstrap, onFailure block, identical handlers (l.59-73, l.121-125); syntax-gate on PostToolUse
  (l.74-77); NEEDED routine commands allowed and not denied (l.35-45, 79-82); NEVER commands statically denied
  (l.47-54, 83-85; includes 'git checkout main', 'git restore x', 'git reset --hard'); GUARD-FILE bootstrap cases
  (l.87-198); real guard: day `git checkout main` -> deny (l.154-156), day `git status` -> no output (l.163-165).
- Consequence: option (a) (permissions.ask rules) breaks the l.56 test by design; the test's reason is routines.
- Next: B3 (docs).

### B3 done: official docs (method and findings)
- Method limit: code.claude.com is not on the WebFetch allow list (B2), so no doc page was fetched. Sources used:
  WebSearch restricted to code.claude.com (its answers are search summaries of the pages, NOT verbatim quotes; the
  verifier or a session that may fetch code.claude.com must check them); the cloud environment's own documentation
  tool (read_documentation, topic session.permissions: verbatim below); the CHANGELOG at
  https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md (allowed domain; 1.15 MB, only the first
  100000 characters read, quotes verbatim as returned). Local Claude Code: 2.1.318 (`claude --version`).
- Hooks, "ask" (https://code.claude.com/docs/en/hooks, summary): PreToolUse permissionDecision "ask" shows the normal
  permission prompt and permissionDecisionReason is shown to the user in it; with several hooks the most restrictive
  answer wins, order deny > defer > ask > allow; ask/deny RULES still apply whatever a hook returns ("a matching ask
  rule still prompts even when the hook returned allow"); in a -p run where nobody can answer, the prompt is not
  shown and Claude reads the reason in the tool result (the call does not run).
- CHANGELOG verbatim: 2.1.304 "only the permission dialog or a host's callback can approve the ask"; 2.1.314
  "such hooks can block or ask, but not approve or rewrite those commands"; 2.1.314 "a hook's allow of an MCP tool
  call now counts as no answer, so the classifier or a permission prompt decides"; 2.1.317 "Fixed edits to settings
  files made in the first second or so after launch not taking effect" (settings files ARE re-read after launch);
  2.1.317 "Added a notice when auto mode's check has no answer for a subagent's tool call".
- Permissions (https://code.claude.com/docs/en/permissions and /settings-reference, summary): permissions.ask = tool
  uses that still prompt even in acceptEdits or bypassPermissions; deny holds in every mode including
  bypassPermissions; allow rules have no effect in bypassPermissions; in dontAsk a would-prompt call is denied.
  Built-in protected paths (.git, .claude, .vscode, shell config...) are never auto-approved except in
  bypassPermissions; in auto mode protected-path writes go to the classifier even when an allow rule matches.
  NOTE: built-in protection covers .claude/ and .git/, not CLAUDE.md at the root, tools/gate.mjs etc. (unconfirmed;
  the full list was not in the results).
- Settings reload (https://code.claude.com/docs/en/settings and /hooks-guide, summary): Claude Code watches settings
  files and reloads them, so edits to permissions and hooks apply without a restart; some keys are read only at start;
  the hooks guide says a missed edit needs a restart (file watcher). ConfigChange (https://code.claude.com/docs/en/hooks)
  fires when a configuration file changes during a session; matcher source project_settings = .claude/settings.json;
  it can block the change (exit 2 or JSON decision), not policy_settings. Our guard blocks it only for routines
  (routine-guard.mjs l.257-260).
- Consequence for item 4 (the switch): a `git switch` in the main folder rewrites .claude/settings.json on disk; Claude
  Code then (per the docs) reloads it for the running session -- in a day session nothing blocks the ConfigChange, so
  the other branch's permissions AND hook list take effect live. Separately, hook COMMANDS read their script from disk
  on every call (`${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs`), so even without a reload the other branch's
  guard.mjs/routine-guard.mjs code runs at once. Both are what Mustafa describes. (guard.mjs l.8-9 records that the
  session kept the hook settings it loaded during the freeze: a reload is not guaranteed either; both paths are bad.)
- Next: B4.

### B4 done: how an "ask" behaves per permission mode
Sources: B3's (search summaries of code.claude.com/docs/en/permissions, /permission-modes, /settings-reference,
/hooks; not verbatim) + the environment doc (verbatim) + prior repo report docs/reports/2026-10-24-kit-v3-0.md:178
("explicit "ask" rules prompt in every permission mode except dontAsk, which denies, and Edit rules also cover Write").
| mode | permissions.ask rule | hook permissionDecision "ask" |
| default | prompt to the person | prompt (reason shown) |
| acceptEdits | still prompts (settings reference: ask = "still prompt even in acceptEdits or bypassPermissions") | prompt |
| plan | no edits are made in plan; a shell call that matches asks -> prompt | prompt |
| auto | content-scoped ask rules are checked before the classifier and force a prompt; the classifier can't approve | normally a prompt (docs mention exceptions the search excerpt cut off; a changelog fix "hook ask floors the decision at a prompt" for unsandboxed Bash, unpinned) |
| bypassPermissions | ask rules still prompt (settings reference, summary) | unknown (not found) ; not offered in cloud sessions |
| dontAsk / -p / no one can answer | denied | denied; Claude reads the reason in the tool result |
- Cloud sessions (read_documentation session.permissions, verbatim): "Cloud sessions run file edits without asking in
  every mode; Auto also runs most other tool calls without asking" and "Bypass permissions is not offered in cloud
  sessions" and "Cloud sessions also ignore defaultMode "bypassPermissions" and "dontAsk" in a settings file".
  UNKNOWN: whether an explicit permissions.ask rule on Edit still prompts in a cloud session ("file edits without
  asking in every mode"), and whether a hook "ask" does. Must be proven by a live probe at build time (below).
- This cloud day session: auto mode (checkpoint; reports kit-v1-1:28, kit-v1-2:25). Who answers: Mustafa in the
  claude.ai/code web UI or the app; if he is away the call waits (an ask is not decided by the classifier).
- Mustafa's PC sessions: what is known = he runs Claude Code on Windows with /permissions available
  (kit-v1-2 report:25) and gave plans there (kit-v3-1 report:221, "Co-Authored-By: Claude Opus 4.8"); its permission
  mode is NOT recorded anywhere in the repo (grep for auto mode|acceptEdits|bypassPermissions|defaultMode, no hit
  about his PC). Since 2.1.283 auto is the built-in starting mode for interactive terminal and VS Code sessions
  (permission-modes, summary). If he ever runs bypassPermissions there, ask rules still prompt but a hook "ask" is
  unconfirmed. Question for Mustafa: which mode his PC sessions use.
- FINDING (big): this day session's project folder /home/user/Shamble-Frontlines is checked out at e4d8b22
  (`git worktree list`), which predates kit-v3-1: `.claude/hooks/` there holds only syntax-gate.mjs (no guard.mjs),
  and `git diff --stat e4d8b22 origin/main -- .claude/settings.json .claude/hooks` = 5 files, +1530 -21. A day
  session's settings and hooks come from its main folder's checkout, NOT from the worktree it builds in. So any day
  guard A3 adds protects only sessions whose main folder carries it; this session would not have it. (Whether this
  session loaded e4d8b22's settings at its start is not checked; the folder is at e4d8b22 now.)
- FINDING: work happens in worktrees under the scratchpad, outside CLAUDE_PROJECT_DIR. routine-guard's projectRel()
  returns null for them (l.48-53), so protectedRel() never sees a kit file there; a settings ask rule written as a
  project-relative path (`Edit(.claude/**)`) would not match either. The day check must find the file's own git
  worktree (`git rev-parse --show-toplevel` from its folder, same `--git-common-dir` as the project) and apply
  protectedRel() to the path relative to THAT worktree's top. Side finding for security-auditor: routines' writePath()
  allows any write inside the scratchpad (l.296), so a routine can write a kit file in a scratchpad worktree; it is
  caught later only at night-shift commit (l.532-538) and push (protectedInBranch l.554-560). Not weakened by A3.
- Next: B5.

### B5 done: edge cases
- Subagents and workflow agents: PreToolUse fires the same configured hooks inside a subagent, and the input carries
  agent_id and agent_type (https://code.claude.com/docs/en/hooks, search summary: "agent_id and agent_type tell your
  script which agent a hook fired in, such as a subagent, an in-process teammate"; absent on the main thread;
  agent_type also present with --agent). Background subagent prompts surface in the main session naming the agent
  (https://code.claude.com/docs/en/sub-agents, summary); where no prompt can be shown the call is denied (hooks,
  summary); CHANGELOG 2.1.317 verbatim "Added a notice when auto mode's check has no answer for a subagent's tool call".
  Not documented: what a workflow agent does while its prompt waits unanswered (likely the run stalls until Mustafa
  answers). Today none of the guard reads agent_id (grep agent_id|agent_type .claude/hooks/*.mjs: no hit).
- Rules on agents: CLAUDE.md:211 "They review and research; the main session builds", CLAUDE.md:216 "Agents never edit
  source, except build work the main session assigns them in an isolated worktree". So a subagent editing a kit or
  gate file is either a rule break or assigned worktree build work. Options: (i) the same "ask" for subagents (the
  prompt reaches the main session; an unwatched workflow stalls); (ii) deny when agent_id is set and a kit file is the
  target, reason "kit and gate files are edited by the main session, which asks Mustafa" (no stall, no lost prompt;
  costs: a kit build can't be delegated to a worktree agent). Recommendation: (ii); kit change-sets are small and
  built by the main session anyway (kit-v3-1/-2 were).
- Isolated worktree agents write under .claude/worktrees/<name>/...; protectedRel() already maps them (l.34), and the
  scratchpad worktree case needs the git-top mapping of B4.
- Routines: decide() handles role first for every tool; the new ask code must sit in the `!role` branch only (after
  l.272 `if (!role) return null` becomes the day path) so every routine deny is unchanged, and an "ask" can never be
  returned when SHAMBLE_ROLE is set (a routine has nobody to ask: PermissionRequest is denied with interrupt,
  l.261-264). Test: for each role, the same kit-write inputs still give deny (not ask) with today's reasons.
  Also: the ask list stays [] in settings.json (settings.test.mjs:56) if mechanism (b) is chosen; mechanism (a)
  would put ask rules into the shared settings.json the routines also load: a routine would then hit
  PermissionRequest -> deny+interrupt (stops the run) instead of today's PreToolUse deny with a reason (the run goes
  on). That changes routine behaviour (WRITE-GUARD's own caveat, STATUS:427-428), so (a) is worse for routines.
- Commit and merge: committing or merging kit files does not edit them and is not asked (Mustafa: "kit and gate
  edits"); git commit, git push of a branch, a PR merge, `git merge origin/main` stay as today. git commands that
  rewrite working files from elsewhere: checkout/restore are already refused (GIT_NEVER l.202-205); `git switch`,
  `git merge`, `git pull`, `git stash pop/apply`, `git cherry-pick`, `git am`, `git apply` write files from another
  commit or patch: the switch belongs to item 4; `git apply`/`git am` of a patch onto kit files is an edit by another
  name (ask when the patch touches a kit path is too deep to parse; recommend: ask on `git apply`/`git am` always in
  the main folder's repo? -> keep it out of scope, list as a known limit).
- Shell writes the day guard can see (parseShell gives redirects for every session, l.75, l.78): redirect targets
  (`>`, `>>`, `&>`), `tee [-a] FILE`, `sed -i`/`--in-place` FILE, `perl -i`, `cp SRC DST`, `mv SRC DST` (both ends
  for mv), `rm`, `ln`, `touch`, `truncate`, `install`, `dd of=FILE`, `chmod`/`chown`. Not visible: interpreters with
  inline code (`node -e`, `python -c`, a here-document into node/python) and scripts that write files: heuristic
  option = ask when the inline code text names a protected path segment (`.claude/`, `CLAUDE.md`, `.github/`,
  `tools/gate.mjs`...); a script file run by path is not inspected (documented limit). Variables/globs in a target
  (`$X`, `*`) -> ask (can't resolve).
- Next: B6.

### B6 done: the `node .checkpoints/*` allow-list entry
- Source: STATUS:517-519 (kit-v3-2 risk 5: "scripts in the notes folder ask before they run in day sessions, and
  routines refuse them: an allow-list entry for `node .checkpoints/*` is a possible later item, with
  security-auditor"); CHECKPOINTS.md:93-97 (scratch = "text and small scripts only ... a short harness"; the folder is
  pushed to the public repo).
- Today: no allow rule names .checkpoints scripts (settings.json allow l.5-150; only `node .checkpoints/`-free
  entries; `node .gate/*.mjs` and `node .gate/*.js` ARE allowed, l.49-50). Routines: interpreter() runs only tracked
  files identical to origin/main and refuses .gate/ (routine-guard.mjs l.457-472), so a notes-folder script is refused
  (it is never on main); a settings allow can't change that (the hook deny comes first). So the entry affects day
  sessions only.
- Who gains: only sessions in default/acceptEdits mode (Mustafa's PC, mode unknown, B4). In this cloud auto session
  the classifier already decides most shell calls without a prompt (read_documentation: "Auto also runs most other
  tool calls without asking").
- Risks: the scripts are agent-written and unreviewed; agents read web pages (data that may try to steer them); an
  allow removes the only human look before such code runs on the PC. The pattern is wide: under the repo's own
  emulation of Bash-rule matching (settings.test.mjs:23 globToRe), `node .checkpoints/*` also matches
  `node .checkpoints/../tools/x.js` (checked: `node -e` with that regex -> true), i.e. any file reachable by `..`.
- Inconsistency found: `node .gate/*.mjs` / `.gate/*.js` (l.49-50) already allow agent-written scripts with the same
  `..` reach; .gate/ is git-ignored scratch (not pushed) but equally unreviewed. For security-auditor to weigh.
- Recommendation: keep asking (no allow entry). If Mustafa wants fewer prompts: a guard-side allow is not possible
  (the guard never allows, routine-guard.mjs l.9-11), so the only form is a settings allow, narrowed by the day guard
  denying `..` and absolute paths in the script argument; still unreviewed code. Second-best, not recommended.
- Next: B7 (item 4).

### B7 done: item 4, the worktree rule
- When settings and hooks load: docs (B3: settings are watched and reloaded mid-session; ConfigChange fires and can
  block). The repo's own incident agrees: kit-v3-1 report docs/reports/2026-10-27-kit-v3-1.md:97-102 "A day session did
  a trial merge of this PR in its main folder, which loaded this PR's .claude/settings.json" / "Switching back removed
  guard.mjs, but the session kept the hook settings it had loaded" / "The ConfigChange hook blocked the reload of
  main's settings as well, so every tool stayed blocked". So a switch (or merge) in the main folder does two things:
  the other ref's settings.json is reloaded live (unless ConfigChange blocks it), and every hook command reads its
  script from disk at each call (`${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs`, settings.json l.270), so the other
  ref's hook code runs at once. Worktrees elsewhere don't touch CLAUDE_PROJECT_DIR, so they change neither.
- FINDING: this session's main folder is at e4d8b22 (pre-kit-v3-1): its settings.json has no guard.mjs at all
  (`grep -c guard.mjs` = 0) and an older non-empty ask list (l.105-...: git switch -f, branch -D, worktree remove...).
  If loaded from there, this day session runs with no guard. The worktree rule keeps a main folder still, so it also
  keeps it stale: the rule needs a sanctioned way to bring the main folder up to main (a fast-forward to origin/main:
  main's settings are the reviewed ones). Question for Mustafa (MAIN-FOLDER-UPDATE).
- Where the rules tell a session to check out or switch (grep checkout|switch|worktree|detach|reset --hard):
  1. .claude/skills/changeset/SKILL.md:11 "From an up-to-date `main`, create `cs/<short-slug>`" (a switch -c in the
     main folder; from main = main's rules: fine, keep, but say "from origin/main").
  2. .claude/NIGHT_SHIFT.md:25-26 handoff "Check out that branch and resume from the checkpoint" (a routine switches
     its main folder to a cs/ branch from origin; ConfigChange is blocked for routines (routine-guard.mjs l.257-260) so
     settings stay, BUT hook scripts are read from disk: a cs/ branch that changes .claude/hooks would run its own guard
     code in the night routine. Only checked: the branch exists on origin and is not a fork/pull ref. HOLE: nothing
     checks the branch leaves protected files alone before the switch (protectedInBranch runs only at push, l.554).
  3. .claude/agents/advisor.md:72-73 PR Advisor routine: `git switch --detach origin/main` (onto main's rules; the
     guard allows exactly this form for pr-advisor, routine-guard.mjs l.531). Keep.
  4. .claude/CHECKPOINTS.md §8 (l.186-192) "Check the branch matches it" -- says nothing about how the session gets
     onto the branch (implicitly a switch in the main folder). Needs the worktree wording.
  5. CLAUDE.md:2-3 protocol "on its own branch cs/<slug>" (l.~57), :63 and :211-216 (agents in isolated worktrees);
     no rule says how a day session checks out a PR or an old commit. advisor.md:91-94 day-session PR review "run the
     full gate ... reproduce the headline claims" -- on the PR head, place unstated: must be a worktree.
  6. Already right: render-ab SKILL.md:8 `git worktree add .gate/base <baseRef>`; gate SKILL.md:12 battery "on a
     worktree of the base ref"; verifier.md:20 "on a base worktree".
- Proposed wording (one rule, in CLAUDE.md "Never" + a line in "The change-set protocol" step 1; copied in
  CHECKPOINTS.md §8 step 2, the changeset skill step 1, advisor.md (day PR review, step 2) and NIGHT_SHIFT.md step 4):
  "Never move a session's main folder (the folder the session started in) to another branch, a pull request's head or
  an old commit: no git switch, checkout, merge or pull of another ref, gh pr checkout, or reset there. Work on another
  ref in its own worktree: `git worktree add .gate/<slug> <ref>` (a PR's head: `git fetch origin pull/<n>/head` then
  a worktree on FETCH_HEAD's SHA). Why: the main folder's `.claude/settings.json` and hooks govern the running session;
  moving it loads that ref's settings and runs its hook scripts at once (the freeze of 2026-11-03), and would let a
  PR's settings act on the session. Two moves stay: a new branch cut from origin/main (`git switch -c cs/<slug>
  origin/main`, main's own rules), and bringing the main folder's branch up to origin/main (`git merge --ff-only
  origin/main`)." (The second sentence only if Mustafa says yes to MAIN-FOLDER-UPDATE.)
  CHECKPOINTS.md §8 step 2 addition: "Resume in a worktree of the task's branch (`git worktree add
  .gate/<slug>-wt <branch>`) unless the main folder is already on it." NIGHT_SHIFT step 4: "Before resuming on that
  branch, check that it changes no rules, CI or gate file (`git diff --name-only origin/main...origin/<branch>`); if
  it does, stop the item under Decisions needed" (the guard should enforce it, below).
  Note: routines' scratch worktrees: `.gate/` is fine for day; night shift may only write inside the project or the
  scratchpad (writeTarget l.368-381), so `.gate/<slug>` works for routines too.
- Should the day guard enforce it? Options: (i) words only (today: nothing stops `git switch <branch>`, allowed by
  settings.json l.25; `gh pr checkout` has no rule); (ii) the day guard ASKS, with a reason naming `git worktree add`,
  for these in the main folder (the command's folder resolves, via `git rev-parse --show-toplevel`, to the realpath of
  CLAUDE_PROJECT_DIR): `git switch <ref>` / `git switch --detach <ref>` where ref is not origin/main or main, `git
  switch -c <new> <start>` with a start other than origin/main/main, `gh pr checkout`, `git merge <ref>` / `git pull`
  of anything but origin/main (the freeze was a trial merge), `git cherry-pick`, `git am`, `git bisect`, `git
  read-tree -u`, `git checkout-index`; and DENY them for a subagent (agent_id set: agents never move the main
  folder); (iii) refuse outright. Already refused for every session: git checkout, git restore, git reset --hard/any
  ref (GIT_NEVER l.202-205, reset rule l.502-508, deny list l.178-196). Recommendation (ii): it keeps a door for a rare
  legitimate move that Mustafa approves, matches the day guard's R3c philosophy (a day session can always be fixed),
  and in auto mode a hook "ask" goes to the person, not the classifier (B4). Question MAIN-SWITCH for Mustafa: ask (ii)
  or refuse (iii).
- Routines: night-shift handoff switch (NIGHT_SHIFT step 4) -> the guard (role night-shift) refuses `git switch
  <branch>` when `git diff --name-only origin/main...<branch>` names a protectedRel path (reuse protectedInBranch's
  logic on the target ref). Only adds a refusal; the PR Advisor's `--detach origin/main` stays the only switch it has.
- Test cases (routine-guard.test.mjs, new describe "main-folder moves (item 4)", fake io with projectDir=/p, worktree
  /w with the same common dir, git(['rev-parse','--show-toplevel']) answering per cwd):
  T1 day, cwd /p, `git switch cs/other` -> ask, reason matches /worktree add/;
  T2 day, cwd /p, `git switch --detach 4def8d2` -> ask; T3 day, cwd /p, `gh pr checkout 38` -> ask;
  T4 day, cwd /p, `git merge cs/other` -> ask; T5 day, cwd /p, `git merge origin/main` -> null;
  T6 day, cwd /p, `git switch -c cs/new origin/main` -> null; T7 day, cwd /p, `git switch -c cs/new` -> null;
  T8 day, cwd /w (a worktree), `git switch cs/other` -> null; T9 day, `git worktree add .gate/x origin/pr-head-sha` -> null;
  T10 day, `cd /p && git switch cs/other` from cwd /w -> ask (cd tracked); T11 day, `git pull` in /p -> ask;
  T12 subagent (agent_id set), cwd /p, `git switch cs/other` -> deny; T13 day, `git checkout main` -> deny (unchanged);
  T14 day, `git show origin/main:CLAUDE.md > /w/x.md` -> null (a plain file restore stays possible);
  T15 night-shift, `git switch cs/x` where the fake diff names .claude/hooks/guard.mjs -> deny; T16 night-shift,
  `git switch cs/x` with a diff of only shamble-game.js -> null; T17 pr-advisor `git switch --detach origin/main` ->
  null (unchanged); T18 every role: no decision is ever {ask} (property: decide() with role set never returns ask).
  settings.test.mjs: T19 the real guard through the bootstrap: day `git switch cs/x` in the repo root -> stdout
  permissionDecision "ask" with a reason; T20 `git worktree add` -> no output.
- Next: B8 (the answer).

### B8 done: the answer (design, tests, questions)
Item 1 options: (a) permissions.ask rules for Edit/Write on kit paths in settings.json -- breaks settings.test.mjs:56
(ask must be [] for routines), routines then reach PermissionRequest -> deny+interrupt (stops the run, a change to
routine behaviour), the rules are project-relative so scratchpad worktrees are missed, shell writes not covered.
(b) the day guard returns "ask" from PreToolUse: covers file tools and visible shell writes, any worktree of this
repo, only when SHAMBLE_ROLE is unset; ask list stays []. (c) both. Recommend (b).
Design (b):
- routine-guard.mjs: new `export function kitPath(p, cwd, io)` -> kit-relative path or null: projectRel()+protectedRel()
  (l.27-53), else the path's own git worktree (`rev-parse --show-toplevel` and `--git-common-dir` from its nearest
  existing folder; same common dir as io.projectDir) + protectedRel(relative to that top). New `const ask = (reason) =>
  ({ ask: reason })` beside deny (l.251). decide(): replace l.272 `if (!role) return null;` with
  `if (!role) return dayFiles(tool, ti, cwd, io, input);` (Edit/Write/NotebookEdit/MultiEdit -> kitPath -> ask, or
  deny when input.agent_id is set). bash(): for !role, collect write targets (redirects c.redirects, tee, sed -i /
  --in-place, perl -i, cp/mv (both ends for mv), rm, ln, touch, truncate, install, dd of=, chmod/chown; inline code of
  node -e/-p, python -c naming a kit path segment) -> ask; plus item 4's main-folder moves -> ask; keep scanning after
  an ask so a later deny in the same line wins (deny > ask). Routines: every role path unchanged (role code runs
  before; an ask is never returned when a role is set).
- guard.mjs print(): a third shape: `{ hookSpecificOutput: { hookEventName: 'PreToolUse', permissionDecision: 'ask',
  permissionDecisionReason } }`, only for PreToolUse and only when no role (PermissionRequest/ConfigChange never ask).
- settings.json: no change for item 1 (ask stays []); item 4: none needed (the guard asks); CHECKPOINT-NODE: none
  (keep asking).
- Docs: CLAUDE.md "Never" + protocol step 1 (worktree rule), "Hard constraints"/"Agents" line on kit edits asking;
  CHECKPOINTS.md §8; changeset SKILL.md:11; advisor.md:91 (day review in a worktree); NIGHT_SHIFT.md:25-26 (handoff
  checks the branch's protected files; resume in place is OK only for a branch that changes none); STATUS risks.
- Limits to state: a guard error in a day session = no decision (guard.mjs l.41-42, R3c) so no ask; scripts run by path
  and git apply/am/stash pop are not inspected; this session's main folder (e4d8b22) has no guard, so the new day ask
  protects only sessions whose main folder carries it.
Tests (routine-guard.test.mjs, new group "day ask-first (item 1)"; fake io projectDir /p, worktree /w same common dir):
A1 Edit /p/.claude/hooks/guard.mjs ask; A2 Write /p/CLAUDE.md ask; A3 Edit /p/tools/gate.mjs ask; A4 Edit
/p/docs/STATUS.md null; A5 /p/.claude/agent-memory-local/x null; A6 /p/.checkpoints/x.md null; A7 /w/.claude/settings.json
ask; A8 /w/shamble-game.js null; A9 /p/.claude/worktrees/a1/CLAUDE.md ask; A10 symlink in /p to .claude ask; A11 other
repo's .claude/settings.json null; A12 `echo x > .claude/settings.json` ask; A13 `sed -i s/a/b/ CLAUDE.md` ask; A14
`cp /tmp/x tools/gate.mjs` ask; A15 `mv .github/workflows/gate.yml /tmp/` ask; A16 `tee -a .claude/rules/x.md` ask; A17
`node -e "require('fs').writeFileSync('.claude/x','')"` ask; A18 `cat CLAUDE.md` null; A19 `git show
origin/main:CLAUDE.md > CLAUDE.md` ask; A20 redirect to `$F` ask; A21 `echo x > .claude/a && git push -f origin cs/x`
deny; A22 subagent (agent_id) Edit kit path deny; A23 every role, A1-A3 -> today's deny reasons; A24 PermissionRequest
day null; A25 MultiEdit/NotebookEdit kit path ask; A26 decide() with a role never returns ask (property over all
cases). Item 4: T1-T20 (B7). settings.test.mjs: S1 ask still []; S2 real guard via bootstrap, day Edit
.claude/settings.json -> permissionDecision "ask" with a reason; S3 same with SHAMBLE_ROLE=night-shift -> deny.
Live probes (only in a session whose main folder has the change): P1 cloud auto: Edit .claude/rules/probe.md -> the
prompt reaches Mustafa (he answers No); P2 hook ask while a settings allow matches (`git switch *`, l.25) -> still a
prompt; P3 a workflow subagent -> deny; P4 his PC session in its usual mode.
Questions: ASK-MECH (b rec); AGENT-KIT (deny rec); MAIN-SWITCH (ask rec / refuse); MAIN-FOLDER-UPDATE (fast-forward
to origin/main allowed, rec yes); CHECKPOINT-NODE (keep asking rec); PC-MODE (which mode, info); DOCS-DOMAIN
(code.claude.com on the WebFetch list, rec yes, its own small item with security-auditor).

DONE
