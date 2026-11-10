# kit-v3-5 facts (F1, GUARD-RENAME (c)): facts for the plan, gathered 2026-11-09 (read-only)

## Plan (blocks, in order)
- B1 merge check end to end (routine-guard.mjs)
- B2 protectedRel and every caller
- B3 tests (routine-guard.test.mjs, settings.test.mjs)
- B4 GitHub API docs (List pull request files, PR object changed_files)
- B5 Claude Code docs: instruction files, AGENTS.md, subagent memory local
- B6 other paths not covered by protectedRel
- B7 rules text depending on the gap (advisor.md, STATUS, CLAUDE.md)
- B8 PR Advisor routine prompt (STATUS)
- B9 Not found / not checked; Tensions

## Working log

## 1. The merge check, end to end (B1)
- Seat: `decide()` reads `io.env.SHAMBLE_ROLE` (routine-guard.mjs:371-372); `ROLES` = night-shift, pr-advisor (:25);
  empty = day session; any other value = 'unknown', refused with interrupt (:382). Header comment :7 says the role is
  "set only by the routine-only cloud environments".
- Hook wiring: settings.json:266-300 runs guard.mjs on every tool (`matcher: "*"`), fail-closed for routines when the
  module can't load or ends without a decision (the inline -e at :272/:282/:291). guard.mjs:42-43 imports decide().
- Recognized merge routes:
  - GitHub MCP `merge_pull_request` → `mergeCheck(ti, role, io)` for EVERY session, wrapped in try/catch that turns
    any exception into a deny ("merge fails closed for every session (K1)") (:1394-1395).
  - MCP `enable_pr_auto_merge` → always denied: "auto-merge skips the expected-head check" (:1393). (No check of
    `disable_pr_auto_merge`; it falls to the generic write path.)
  - `gh pr merge` → always denied: "merge with the GitHub MCP merge tool and the approved SHA as expectedHeadSha" (:1340).
  - `gh api` with any write (`-f/-F/--field/--raw-field/--input`) or non-GET method → denied for every session:
    "gh api is read-only here" (:1341-1357), so `gh api -X PUT .../pulls/N/merge` is refused. Routines are also limited
    to `repos/Mergincanli/Shamble-Frontlines/` or `rate_limit` (:1356).
  - Routines: `gh` beyond read forms is deny-by-default (:1359-1362); `git merge` denied in routines (:1029).
- mergeCheck (:1444-1467), in order:
  1. `expectedHeadSha` must be 40 lowercase hex (:1445-1446).
  2. routines: owner/repo must be REPO (:1448); night-shift: always denied, "the night shift never merges" (:1449).
  3. `GET repos/<repo>/pulls/<n>` via `ghJson` (= `gh api <path>`, :1404-1408); `pr.head.sha !== sha` → deny (:1451);
     fork check: head.repo.full_name vs base.repo.full_name (:1452).
  4. Check runs: `GET repos/<repo>/commits/<sha>/check-runs?per_page=100` (single page, NOT paginated) (:1453); for
     `gate` and `smoke`: none → deny; `rs.some((r) => r.status !== 'completed')` → deny "still running" (:1457), i.e. ANY
     run of that name not completed (a stale `in_progress` or `queued` re-run) blocks, even if another run of the same
     name succeeded; then every run's conclusion must be success/skipped/neutral and at least one success (:1458).
  5. Only when `role === 'pr-advisor'` (:1460): file list via
     `io.gh(['api', '--paginate', 'repos/<repo>/pulls/<n>/files?per_page=100', '--jq', '.[].filename'])` (:1461);
     non-zero exit → throw → deny (:1462); `hit = lines.find(protectedRel)` (:1463) → deny "the PR changes <hit>; rules,
     CI and gate changes merge from a day session on Mustafa's go" (:1464).
     Fields read: `filename` ONLY. `status` and `previous_filename` are not read, so a rename's OLD path (the protected
     one) is never checked. No count check against the 3000 cap and no comparison with `pr.changed_files` (pr object is
     fetched at :1450 but `changed_files` is unused).
  6. return null (no decision; normal permission flow) (:1466).
- Day sessions: steps 1, 3, 4 apply (header :5-6: "the merge check (expectedHeadSha, same repository, gate and smoke
  green on that SHA)"); NOT the repo check (:1448 is `role &&`) and NOT the file list (:1460).
- Contrast: the guard's own git-based lists use `--no-renames` (DIFF_NAMES :1129) "renames off (a kit file moved away
  still names its old path)" (:1125-1128) — used by night commit (:1088-1100), night push (protectedInBranch
  :1139-1145), night switch (switchTargetCheck :1188), day main-folder moves (mainFolderMove :1298/:1306/:1319). The
  merge check is the one list that does not have this property.
- B1 done; next B2

## 2. protectedRel (:32-43) and every caller (B2)
- What it protects (:32-43): basename `CLAUDE.md`, `.npmrc` or `.mcp.json` at any depth (:35); first segment `.github`
  or `.git` (:36); first segment `.claude` → true, EXCEPT `.claude/agent-memory-local/**` → false (:38) and
  `.claude/worktrees/<a>/<rest>` → protectedRel(<rest>) (:39; `.claude/worktrees` and `.claude/worktrees/<a>` alone
  → false); else GATE_FILES exact (:26-27: tools/gate.mjs, smoke.mjs, battery.js, gate-globals.json, package.json,
  package-lock.json). Not covered: `CLAUDE.local.md`, `AGENTS.md`, any other basename. Comment :31: "agent
  instructions anywhere".
- Case fold: `CANON` (:70-71) lists `agent-memory-local` and `worktrees`, used by `protectedFold` (:72-73) on
  win32/darwin only.
- Why agent-memory-local is exempt (evidence, no comment at :38 itself): it is where `memory: local` agents keep
  memory (advisor.md:304: "`memory: local` also turns on Read, Write and Edit so you can keep your memory in
  `.claude/agent-memory-local/advisor/`"); the PR Advisor's write allowance names it (:417, file tools only); the
  kit-v3-1 report :31 ("`memory: project` → `memory: local` (agent memory is gitignored, never committed or pushed)")
  and :48 (security-auditor L1: the shell-redirect target list for pr-advisor "omits `agent-memory-local`, which is
  written via the Write tool, not a redirect"). `.gitignore` lists `.claude/agent-memory-local/` (and
  `CLAUDE.local.md`, `.claude/settings.local.json`, `.claude/claude-security-guidance.local.md`). `git ls-files
  .claude/agent-memory-local` → empty on this tree.
- Direct callers of protectedRel:
  - :39 recursion (worktrees).
  - :73 `protectedFold` → `kitPath` (:97-108; :104, :107) and `kitDir` (:131).
  - :407 `writePath` (routines' Edit/Write/NotebookEdit/MultiEdit, both roles) — then :411 kitPath, then :417
    pr-advisor allow-list `^(\.checkpoints|\.gate|\.claude\/agent-memory-local)(\/|$)`.
  - :501 `writeTarget` (routines' shell redirects) — then :503 kitPath; :509 pr-advisor allow-list is only
    `.checkpoints|.gate` (no agent memory).
  - :1106 routines' `git rm` / `git mv` args (`sub !== 'add'`: `git add` of a protected path is not refused here).
  - :1120 `kitName` → night `git commit` lists (:1088-1101) and `protectedHit` (:1133-1138) → night push
    `protectedInBranch` (:1139-1145) and day `mainFolderMove` (:1298, :1306, :1319).
  - :1188 night `switchTargetCheck`.
  - :1463 the merge check (pr-advisor only). This is the only use that is "the merge check only".
- kitPath callers (via protectedFold): writePath :411 (routines), writeTarget :503 (routines), day shell write
  targets :862 (with kitDir), `dayEdit` :936 (day file tools: main session ASK, subagent with `agent_id` DENY
  `KIT_AGENT` :741/:937).
- If protectedRel stopped exempting agent-memory-local, behaviour changes at: routines' Write to agent memory
  (:407 denies before :417; the :417 agent-memory allowance becomes dead) — test row :171 'night: agent memory
  (local)' expects allowed; day sessions: a subagent's (e.g. the advisor's, `memory: local`) Write/Edit to its own
  memory would be DENIED by dayEdit (:936-937) and the main session's ASKed — rows :681 'A5s agent memory' and :757
  'A5 Write agent memory' expect null; the night switch check — row :1756-1757 'T16 night, docs and agent memory only:
  goes on' expects null; night commit/push lists and git rm/mv would also count it (gitignored, so only a force-added
  file shows). Mutation row :495 pins the :417 regex text.
- Conclusion (fact, not a decision): a merge-check-only refusal of `.claude/agent-memory-local/**` needs a separate
  predicate at :1463, not a change to protectedRel, if the memory writes are to keep working.
- B2 done; next B3

## 3. The tests (B3)
- Counts confirmed on this tree: `node --test .claude/hooks/routine-guard.test.mjs` → 1034 pass, 0 fail (≈31 s);
  `node --test .claude/hooks/settings.test.mjs` → 127 pass (output in `.gate/kit-v3-5-facts/`).
- How merge-check tests are built: no fixtures, no network. `fakeIo(o)` (routine-guard.test.mjs:35-86) answers `gh`
  from a map `ghAnswers` keyed by the first argument that starts with `repos/` (:78-82: other args such as
  `--paginate`, `--jq` are IGNORED; unknown path → code 1). Keys (:46-53): `repos/<REPO>/pulls/27` (pr object from
  `pr(ref, n)` :45: `number`, `head.sha`, `head.ref`, `head.repo.full_name`, `base.repo.full_name` — NO
  `changed_files`), `.../commits/<headSha>/check-runs?per_page=100` (from `checks` triples [name, status,
  conclusion], :51), `.../pulls/27/files?per_page=100` → `prFiles` (default `'docs/a.md\n'`, :41, :52): the faked
  file list is the `--jq '.[].filename'` OUTPUT (newline names), not JSON. `ghFail: true` makes every gh call fail (:79).
- Helpers: `merge(extra)` (:88) = MCP `merge_pull_request` input {owner, repo, pullNumber: 27, expectedHeadSha: SHA};
  `gw(name, input)` (:89); roles DAY '', NIGHT, PRA (:91). Table `CASES` (:93-355) rows [name, role, input, refused?,
  fakeIo options]; runner :357-362 (`assert.equal(!!d, refused)`).
- Existing merge rows (all of them): :127 'merge, green' (DAY); :128 without a SHA; :129 short SHA; :130 'merge, head
  moved'; :131 fork head; :132 'merge, no smoke'; :133 'merge, gate failed'; :134 'merge, the check errors -> fails
  closed'; :135 auto-merge; :249 'night: the merge tool'; :280 'advisor: merge a green docs PR'; :281 'advisor: merge
  a rules PR' `{ prFiles: 'docs/a.md\n.claude/settings.json\n' }`; plus test :364-367 'a merge whose check cannot
  complete fails closed, even for a day session'.
- NOT covered in the merge rows today: a renamed-away file (status/previous_filename), CLAUDE.md or nested CLAUDE.md
  in a merge, the 3000 cap / changed_files, pagination, a non-completed (in_progress/queued) check run, a
  skipped/neutral run, two runs of one name, CLAUDE.local.md, AGENTS.md, agent memory. No test file mentions
  `CLAUDE.local` or `AGENTS.md` (grep: 0 hits).
- Mutation runs (:476-567): MUTANTS [label, from, to] text-replace one rule in a copy of routine-guard.mjs; each `from`
  must occur exactly once (:550); merge-related mutants :488-494 incl. 'the merge head' (:490), 'the merge checks'
  (:491, the conclusion line), 'merge fails closed' (:492), 'protected files in a merge (PR Advisor)' (:493, text `if
  (hit) return deny(\`routine (pr-advisor): the PR changes`), "the PR Advisor's notes-only writes" (:495, pins the
  :417 regex). Editing those guard lines means editing these mutant strings too. There is no mutant for the "still
  running" line :1457.
- Rename rows exist only for the git-based lists: SD-2 rows (:2042-2066 area), e.g. :2042 'SD-2 night push: a kit file
  renamed away (tools/gate.mjs to docs/gate.mjs, both names listed): deny', :2048 'SD-2 night commit (--cached): a kit
  file renamed away: deny', :2061 'SD-2 day git switch cs/ren in the main folder, a kit file renamed away: ask'; the
  comment :1984-1987 says the fakes answer "the names git prints under these flags (a renamed kit file: both names...)".
- Agent memory rows that pin its exemption: :171, :681, :757, :1756-1757 (see section 2).
- settings.test.mjs (127): the static settings layer (ask list empty, effortLevel, hook registration on three events,
  allow/deny matching emulation, the launcher with guard.mjs missing/broken) (:1-7); merge appears only as a denied
  command string 'gh pr merge 27' (:53). Nothing there tests the merge check or protectedRel.
- B3 done; next B4

## 4. GitHub's API (B4; official docs, fetched 2026-11-09)
- https://docs.github.com/en/rest/pulls/pulls#list-pull-requests-files (`GET /repos/{owner}/{repo}/pulls/{pull_number}/files`):
  "Lists the files in a specified pull request." "Responses include a maximum of 3000 files. The paginated response
  returns 30 files per page by default." `per_page`: "The number of results per page (max 100)." (default 30);
  `page` default 1.
  Per-file (diff entry) fields: `filename` (required string); `status` (required) enum `added`, `removed`, `modified`,
  `renamed`, `copied`, `changed`, `unchanged`; `previous_filename` (optional string, no description on the page);
  `additions`, `deletions`, `changes` (required integers); `patch` (optional).
  The page does not say which statuses carry `previous_filename` (by its name, renamed and copied; not stated).
- https://docs.github.com/en/rest/pulls/pulls#get-a-pull-request: "Lists details of a pull request by providing its
  number." Response "Same response schema as [Create a pull request]", which lists `changed_files` (required integer),
  also `commits`, `additions`, `deletions`. The docs give no sentence relating `changed_files` to the files list or
  to the 3000 cap.
- gh manual https://cli.github.com/manual/gh_api: `--paginate` "Make additional HTTP requests to fetch all pages of
  results"; "Each page is a separate JSON array or object."; `--slurp` wraps pages in an outer array. The page does
  not say whether `--jq` runs per page (the guard's `.[].filename` works either way only if it runs per page; not
  documented — not checked live).
- So with today's check (:1461): a PR of > 3000 files returns at most 3000 names; a protected file beyond that is
  never seen. `pr.changed_files` is already fetched at :1450 (unused).
- B4 done; next B5

## 5. Instruction files Claude Code loads (B5; official docs, fetched 2026-11-09)
Source A: https://code.claude.com/docs/en/memory ("How Claude remembers your project").
- Table: "**Project instructions** | `./CLAUDE.md` or `./.claude/CLAUDE.md`. See [AGENTS.md] for when `./AGENTS.md`
  loads instead of or alongside them"; "**Local instructions** | `./CLAUDE.local.md` | Personal project-specific
  preferences; add to `.gitignore`"; user `~/.claude/CLAUDE.md`; managed policy CLAUDE.md.
- "CLAUDE.md and CLAUDE.local.md files in the directory hierarchy above the working directory are loaded at launch.
  Files in subdirectories load on demand."
- "Claude Code loads `CLAUDE.md` and `CLAUDE.local.md` from your current working directory and every directory above
  it." "Within each directory, `CLAUDE.local.md` is appended after `CLAUDE.md`, so your personal notes are the last
  thing Claude reads at that level."
- "Claude also discovers `CLAUDE.md` and `CLAUDE.local.md` files in subdirectories under your current working
  directory. Instead of loading them at launch, Claude Code loads each one once Claude reads, writes, or edits another
  file in that subdirectory." → a nested `docs/CLAUDE.local.md` would load when a session reads a file in docs/.
- `CLAUDE.local.md`: "create a `CLAUDE.local.md` at the project root. It loads alongside `CLAUDE.md` and is treated
  the same way. Add `CLAUDE.local.md` to your `.gitignore` so it isn't committed." (The docs do not say a tracked
  CLAUDE.local.md is skipped; gitignore does not stop a tracked/force-added file from loading — inference, not stated.)
- Imports: "CLAUDE.md files can import additional files using `@path/to/import` syntax." "Relative paths resolve
  relative to the file containing the import" "Imported files can recursively import other files, with a maximum
  depth of four hops." External imports (outside the working directory) need a one-time approval dialog; imports
  INSIDE the working directory load without one (by contrast; the page states the dialog only for external).
  "Import parsing skips Markdown code spans and fenced code blocks."
- Rules: "All `.md` files [in `.claude/rules/`] are discovered recursively"; rules without `paths` load at launch;
  "The `.claude/rules/` directory supports symlinks"; a symlink target outside the working directory is treated like
  an external import.
- `--add-dir` + `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` loads "`CLAUDE.md`, `.claude/CLAUDE.md`,
  `.claude/rules/*.md`, and `CLAUDE.local.md` from the additional directory".
- AGENTS.md: "Claude Code can read `AGENTS.md` as your project instructions"; "Reading `AGENTS.md` directly requires
  Claude Code v2.1.277 or later." Default ("`claude-md-or-agents-md` ... This is the default"): "By default, Claude
  reads `AGENTS.md` only when you have no `CLAUDE.md` in your working directory or above it." What counts: "a
  `CLAUDE.md`, `.claude/CLAUDE.md`, or `CLAUDE.local.md` in your working directory or any directory above it". When
  none count, it reads "every `AGENTS.md` and `.claude/AGENTS.md` in your working directory and the directories above
  it" and "a subdirectory's `AGENTS.md`, when Claude opens a file there with the Read tool and that subdirectory has
  none of the three `CLAUDE.md` files of its own". "**Not read**: `AGENTS.local.md`, `AGENTS.override.md`, or anything
  under a `.agents/` directory". Value `claude-md-and-agents-md` reads both; it is set via `/config` or
  `pluginConfigs` → `cc-plugin-agents-md@builtin`, which "Claude Code reads ... from `~/.claude/settings.json`, a
  `--settings` file, or managed settings, and ignores it in project and local settings files." AGENTS.md files also
  expand `@path` imports.
  → For THIS repo (root CLAUDE.md tracked) under the default, a root or nested AGENTS.md is NOT read; it would be read
  if the root CLAUDE.md were removed/renamed away, or if a user/managed setting chose `claude-md-and-agents-md`. The
  docs do not say whether the subdirectory rule applies when a root CLAUDE.md exists (it sits under "When none count").
- Auto memory (not subagent memory): `~/.claude/projects/<project>/memory/`, `MEMORY.md` index; "The first 200 lines
  of `MEMORY.md`, or the first 25KB, whichever comes first, are loaded at the start of every conversation."
  `autoMemoryDirectory` "is read from any settings scope: user, project, local, policy, or `--settings`" (project
  settings = `.claude/settings.json`, already protected).
- Memory page line: "Claude treats them as context, not enforced configuration. To block an action regardless of what
  Claude decides, use a PreToolUse hook".
- B5 part A (memory page) done; next B5 part B (sub-agents page)
Source B: https://code.claude.com/docs/en/sub-agents ("Enable persistent memory", "What loads at startup").
- Scopes: "`user` | `~/.claude/agent-memory/<name-of-agent>/`"; "`project` | `.claude/agent-memory/<name-of-agent>/` |
  ... shareable via version control"; "`local` | `.claude/agent-memory-local/<name-of-agent>/` | ... shouldn't be
  checked into version control".
- "When memory is enabled: The subagent's system prompt includes instructions for reading and writing to the memory
  directory. The subagent's system prompt also includes the first 200 lines or 25KB of `MEMORY.md` in the memory
  directory, whichever comes first ... Read, Write, and Edit tools are automatically enabled".
  → for the advisor: `.claude/agent-memory-local/advisor/MEMORY.md` goes into the advisor's SYSTEM PROMPT. A
  force-added copy merged to main would be in every checkout of main (the PR Advisor works on `origin/main`,
  routine-guard.mjs:1068) and so in every advisor run.
- "Subagent memory is part of auto memory: if you turn auto memory off ... the `memory` field has no effect".
- The page has no sentence about `.gitignore` (verified by the fetch).
- Startup: a non-fork subagent's initial context contains "every level of the CLAUDE.md hierarchy the main
  conversation loads, including `~/.claude/CLAUDE.md`, project rules, `CLAUDE.local.md`, managed policy files, and
  any `AGENTS.md` files loaded as project instructions. The built-in Explore and Plan agents skip this."
- Only the first 100000 of 116722 characters of the page were read (the relevant sections were inside).
Source C (git, official): https://git-scm.com/docs/gitignore: "Files already tracked by Git are not affected";
https://git-scm.com/docs/git-add `-f, --force`: "Allow adding otherwise ignored files." → a force-added ignored file
is tracked, shows in a PR's file list, and is checked out with main.
- B5 done; next B6

## 6. Paths that can carry instructions or code into a session, not covered by protectedRel today (B6)
Each shown from the docs or the repo; protectedRel's `.claude` rule tests only the FIRST segment (:37), its basename
rule only `CLAUDE.md`, `.npmrc`, `.mcp.json` (:35).
- `CLAUDE.local.md` at any depth: root loads at launch, nested on demand (memory page, §5). `.gitignore` lists only
  the root form `CLAUDE.local.md` (no leading slash, so git matches it at any depth — gitignore semantics, not
  re-checked here), which a `git add -f` overrides (§5 C).
- `AGENTS.md` at any depth: read when no CLAUDE.md/.claude/CLAUDE.md/CLAUDE.local.md is in the working directory or
  above, or under the `claude-md-and-agents-md` setting (user/managed only), and its `@path` imports expand (§5).
  In this repo it matters chiefly together with a renamed-away root CLAUDE.md (the rename gap, §1).
- `.claude/agent-memory-local/<agent>/MEMORY.md` (and the folder): first 200 lines/25KB go into that agent's system
  prompt (sub-agents page, §5); exempt at :38. Eleven agents set `memory: local` (grep: advisor.md:8,
  art-director.md:7, game-designer.md:7, rust-reviewer.md:7, infra-engineer.md:7, security-auditor.md:7,
  perf-auditor.md:7, engine-architect.md:7, graphics-engineer.md:7, netcode-architect.md:7, test-engineer.md:7).
- NESTED `.claude/` folders (first segment not `.claude`), e.g. `docs/.claude/skills/<n>/SKILL.md`: skills page table
  row "Nested | `<subdir>/.claude/skills/<skill-name>/SKILL.md` | Sessions started in or below `<subdir>` ... a
  session started above it loads the skill once Claude works on files there"
  (https://code.claude.com/docs/en/skills). And nested `.claude/rules/`: memory page "rules that load on demand,
  including path-scoped rules and rules in nested `.claude/rules/` directories". `protectedRel('docs/.claude/rules/x.md')`
  → false by the code at :32-43 (reasoned from the code, not run). A nested `<subdir>/.claude/CLAUDE.md` IS covered (basename).
- Not shown as a gap: `.claude/AGENTS.md`, `.claude/rules/**`, `.claude/skills/**`, `.claude/agent-memory/**`
  (project scope), `.claude/settings*.json` — all first-segment `.claude`, protected. `.mcp.json` anywhere — protected.
  The gate's code: tools/gate.mjs imports only `./battery.js`, `./smoke.mjs` (both GATE_FILES) and node_modules
  (installed from the protected tools/package-lock.json) (gate.mjs:63, :189).
- Not checked: whether nested `<subdir>/.claude/agents/`, `<subdir>/.claude/commands/`, `<subdir>/.claude/settings.json`
  or `<subdir>/.mcp.json` are loaded (the docs fetched do not say; `.mcp.json` is covered anyway).
- B6 done; next B7
- Run on this tree (`node -e` importing protectedRel): false for `docs/.claude/rules/x.md`,
  `docs/.claude/skills/a/SKILL.md`, `CLAUDE.local.md`, `docs/CLAUDE.local.md`, `AGENTS.md`,
  `.claude/agent-memory-local/advisor/MEMORY.md`, `.claude/worktrees/a1`; true for `.claude/AGENTS.md`,
  `.claude/agent-memory/advisor/MEMORY.md`, `docs/CLAUDE.md`. (Replaces "reasoned, not run" above.)
- Also note: a committed `.claude/worktrees/<a>/` file maps to protectedRel(<rest>) (:39), so
  `.claude/worktrees/a1/CLAUDE.local.md` inherits the same gap.

## 7. Rules text that depends on the guard's current gap (B7)
Lines that would change (or should be re-read) once the guard is fixed:
- `.claude/agents/advisor.md:72-77` (tier 2): "...for kit, CI and gate files only from a day session (the guard
  refuses the PR Advisor's merge of those it can see; it does not yet see a renamed-away path, `CLAUDE.local.md` or
  `AGENTS.md`, so read the `--no-renames` list yourself)." → the bracket goes (STATUS:567 says so). It does not name
  agent memory, which is also a gap.
- advisor.md:71 tier 2 covers "anything in `.claude/` (agent memory aside)" — with agent memory refused in the merge
  check, this exemption no longer matches the guard (agent memory then is a merge refusal, but its tier is unstated).
- advisor.md:63-64 (tier 0 "Not tier 0 ... a file renamed or moved from a path outside tier 0, and any `CLAUDE.md`,
  `CLAUDE.local.md` or `AGENTS.md` anywhere (risk tier 2)") — stays (judgment rule), but no mention of agent memory.
- advisor.md:54 and :130 (`git diff --name-only --no-renames origin/main...<headSHA>`) — advisor's own reading; stays.
- advisor.md:304-308 ("in a routine it lets pr-advisor write only to `.checkpoints/`, `.gate/` and agent memory") —
  still true if only the merge check changes; false if protectedRel stops exempting agent memory.
- CLAUDE.md:63-64 ("the guard's protected files: `.claude/` except agent memory, every `CLAUDE.md`, `.github/`,
  `tools/gate.mjs` and the other gate files") — would need `CLAUDE.local.md`, `AGENTS.md` if protectedRel takes them;
  note protectedRel also drives the day ask (kitPath), so a new protected name changes day-session asks too.
- CLAUDE.md:95-99 short risk-tier table: row 0 "docs with no word or decision of Mustafa's; not ADRs, the register,
  design or look docs"; row 2 "CLAUDE.md, `.claude/`, `.github/`, gate files, the guard, ..." — F4 (STATUS:577-580)
  wants row 0 "not CLAUDE.md, CLAUDE.local.md, AGENTS.md or a rename"; the advisor's Low on #44 (comment 6740050583
  finding 2) adds: "F4 should also name agent memory" and "every gap is on the strict side" does not hold for row 0
  read alone.
- docs/STATUS.md:562-568 (F1 text: "reads both names ... (`.filename, (.previous_filename // empty)`), refuses a
  pr-advisor merge when the PR's `changed_files` is 3000 or more ..., and `protectedRel` protects `CLAUDE.local.md`
  and `AGENTS.md` anywhere; with test rows. SA-5 ... rides F1 if Mustafa wants it. Once F1 is on main, advisor.md's
  tier-2 bracket ... goes. Whether A4 waits for F1 is his open GUARD-RENAME") — F1 does not mention agent memory; his
  GUARD-RENAME is now answered (c) (`mustafa-2026-11-09-answers.md`).
- STATUS:577-580 F4; :581-585 F5 (revert check; not tied to the guard gap, "next kit pass, with F4").
- STATUS:44-45 still calls #44 a draft (PR Advisor's Low, comment 6740560231 finding 2); STATUS:3 and :114 still say
  kit-v3-4 "In flight" / waiting on GUARD-RENAME (not guard text, but stale since #44 merged 2026-11-09T18:21:13Z,
  merge commit 44446a5, per `gh api .../pulls/44`).
- Sources for SA-1/SA-5/SA-8 (`git show d96e215:.checkpoints/kit-v3-4/security-auditor.md`): SA-1 :73-87 (Critical;
  rename via `previous_filename` never read; `CLAUDE.local.md` unprotected; sketch `git mv .claude/settings.json
  docs/settings-notes.md` or `docs/CLAUDE.local.md` with `git add -f`; fix `--jq '.[] | .filename,
  (.previous_filename // empty)'`, the 3000 refusal, protect CLAUDE.local.md and AGENTS.md); SA-5 :126-131 (Low,
  refuse pr-advisor merges of `docs/adr/**` and the named design and look docs; "The 'records his words' half stays
  judgment."); SA-8 :144-147 (Low, "refuse a pr-advisor merge when the PR's `changed_files` is 3000 or more").
- The agent-memory finding (comment 6740050583, finding 1, read with `gh api .../issues/comments/6740050583`):
  "Critical, on main and outside this PR ...: the guard's merge check exempts `.claude/agent-memory-local/`
  (routine-guard.mjs:38, used at :1461-1464). That folder is gitignored, but `git add -f` can track a file there, and
  after a merge it would load into an agent's memory. ... F1's plan should refuse it in the merge check." The PR
  Advisor's comment 6740560231 finding 1 adds: main's own step 0 used `git diff --name-only` without `--no-renames`
  (now fixed in text by #44).

## 8. The PR Advisor routine's prompt (B8)
- docs/STATUS.md:797-800, "Its stored prompt, verbatim": "You are the PR Advisor for SHAMBLE FRONTLINES. Read
  .claude/agents/advisor.md from the default branch and follow its PR review procedure exactly, together with
  CLAUDE.md, for every open pull request in this repository, oldest first. Stay on the default branch: read each pull
  request only through git, naming its head SHA, and the GitHub tools." (The search phrase "Its stored prompt" spans a
  line break at :797-798.)
- Nothing in it names the merge check, protectedRel, renames or agent memory: the fix does not touch it (fact). It
  makes the routine load main's CLAUDE.md hierarchy and advisor.md, which is why a merged `CLAUDE.local.md`,
  `AGENTS.md` (if CLAUDE.md is renamed away) or `.claude/agent-memory-local/advisor/MEMORY.md` would reach later runs.
- STATUS:793 says the routine has "no MCP connections"; yet B2 merged with `expectedHeadSha` (STATUS:810-811), so the
  GitHub tools it uses are not counted as connections there (not checked further).
- B7, B8 done; next B9

## 9. Not found / not checked (B9)
- Whether `gh api --paginate --jq` runs the filter per page: the gh manual does not say (§4); not tried live.
- Which file statuses carry `previous_filename`: the GitHub page lists it as optional with no description (§4).
- No live call of `pulls/<n>/files` on a real rename PR; no live routine run of any walk-around (all from code + docs).
- Check runs: the merge check reads one page (`per_page=100`, :1453) with GitHub's default `filter=latest`
  (https://docs.github.com/en/rest/checks/runs#list-check-runs-for-a-git-reference: "latest returns the most recent
  check runs"; a limit to "the 1000 most recent check suites"); more than 100 runs on one SHA was not tested.
- Whether a TRACKED `CLAUDE.local.md` loads like an untracked one: the docs say it "loads alongside CLAUDE.md" and say
  nothing about git; that gitignore does not stop it is an inference from git's docs.
- Whether a subdirectory `AGENTS.md` is read when the root CLAUDE.md exists: the rule sits under "When none count";
  the docs do not say it plainly.
- Whether nested `<subdir>/.claude/agents/`, `commands/` or `settings.json` load: not in the pages read. The last
  ~16.7k characters of the sub-agents page and ~19k of the skills page were not read.
- `disable_pr_auto_merge` has no rule of its own (falls to the generic write path); not followed further.
- SA-5: no list of the decision docs exists in code; only advisor.md:60-65 names them.
- Merges outside the guard (GitHub's web UI, another client) are out of the guard's reach; not examined.

## 10. Tensions (B9)
1. protectedRel's comment :31 says "agent instructions anywhere", but the code (:32-43, run in §6) leaves out
   `CLAUDE.local.md`, `AGENTS.md`, nested `<subdir>/.claude/skills|rules/` and `.claude/agent-memory-local/**`.
2. Every git-based list in the guard uses `--no-renames` so that "a kit file moved away still names its old path"
   (:1125-1129). The merge check alone reads only `.filename` (:1461), so a renamed-away rules file passes.
3. The rules call agent memory harmless: CLAUDE.md:63 "`.claude/` except agent memory", advisor.md:71 "(agent memory
   aside)". The kit-v3-1 report :31 says it is "gitignored, never committed or pushed". But git's docs say `-f` adds
   ignored files and tracked files are unaffected by gitignore. The sub-agents docs say `MEMORY.md` goes into the
   agent's system prompt. Hence the advisor's Critical of 2026-11-09.
4. STATUS F1 (:562-568) leaves out agent memory, while Mustafa's GUARD-RENAME (c) says "agent memory included" (and
   the kit-v3-5 checkpoint includes it). F1 also still calls GUARD-RENAME open.
5. To refuse agent memory, the merge check needs its own rule. Changing protectedRel would also refuse the memory
   writes the rules allow (writePath :417; advisor.md:304-308) and flip test rows :171, :681, :757 and :1756-1757 (§2).
6. The PR Advisor's two write paths differ: file tools may write agent memory (:417), shell redirects may not (:509).
   This was known as L1 in kit-v3-1 and left alone.
7. The 3000 cap: the PR object, with `changed_files`, is already fetched (:1450) but not used. GitHub's docs give no
   sentence linking `changed_files` to the files list.
8. The "still running" rule (:1457) refuses a merge when any check run of that name is not `completed`. #44's head
   had a stale `in_progress` gate run (both review comments). By the time I read it, all 4 runs were completed/success.
   No mutant covers :1457.
9. AGENTS.md: SA-1 says it "loads when no CLAUDE.md is above it", and the docs agree for the default setting. In this
   repo it is a danger only together with the rename gap (root CLAUDE.md renamed away) or a user/managed setting.
10. `.gitignore`'s `CLAUDE.local.md` line covers every depth (git rule, not re-checked), while protectedRel covers none.
11. Stale docs: STATUS:3, :44-45 and :114 still show #44 as in flight. GitHub says it merged at 2026-11-09T18:21:13Z
    (44446a5).
12. STATUS:793 says the routine has "no MCP connections", but it merges with the GitHub MCP merge tool (STATUS:810-811).
- B9 done.
DONE
