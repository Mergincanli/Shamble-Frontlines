# Checkpoint: kit-v3-1
- **Goal:** WRITE-GUARD, ROUTINE-EFFORT and no stalls in routines (the Advisor's verdicts of 2026-10-25 and 2026-10-27)
  · plan: `.checkpoints/kit-v3-1/plan.md` APPROVE (round 2, advisor-verdict-2.md).
- **Branch:** cs/kit-v3-1 (BRANCH-2 (a)), worked in its own worktree
  (/tmp/claude-0/-home-user-Shamble-Frontlines/dbb5e494-b49f-5970-85a2-528a1e15b24b/scratchpad/wt-kit-v3-1) · **Base:**
  main 4c24d07.
- **Last update:** 2026-10-27 16:55 Amsterdam

## Steps done
1. Audit, workflow write-guard-audit (run wf_e24159e7-346, cut off once by the spend limit and resumed): 6 agents.
   Traces: night-shift path 56 commands, PR Advisor path 43, past runs 18 decisions, permission docs 8 answers;
   proposal (proposal.md, proposer.md); security-auditor attack: FAIL, 2 critical, 4 high, 4 medium, 2 low
   (security-auditor.md; all of it in audit-result.json). The docs agent (claude-code-guide) has no Write tool, so
   its answers are only in audit-result.json (traces[3]).
2. Plan written (plan.md), adopting the attack's fixes; sent to the advisor (xhigh) for its plan review.
3. Advisor plan review round 1: REVISE (advisor-verdict-1.md, verbatim): R1–R6, decisions taken (ROLE-SIGNAL (a),
   SETUP, PR-CODE (a) enforced, MERGE-ROUTE (a), AGENT-MEMORY (a), DAY-COST (a) except R5, TEST-RUN (a),
   ROUTINE-CODE-EXEC no), conditions C1–C8; escalations MANAGED and RESIDUAL (conditional). Plan revision 2 applies
   them all (plan.md).

4. Advisor re-review, round 2: APPROVE with two conditions (two part-B rows: the trigger fires on a ready event; a real
   PR run starts with HEAD = origin/main) and three nits; all applied to plan.md (advisor-verdict-2.md).

5. Build step 1 (3f43a7a): `.claude/hooks/routine-guard.mjs` (the rules), `.claude/hooks/guard.mjs` (a small
   launcher: rules that fail to load or throw → a routine is refused, a day session goes on, plan R3c),
   `.claude/hooks/routine-guard.test.mjs` (239 tests, 26 mutants all caught, the table must pass unmutated first).
   `git config` in routines reads only user.name and user.email (plan M4). Gate PASS (with main's tools/node_modules
   linked in for the run, then removed). Not registered yet.

5b. (0e93ac4) Routine Bash/gh redesigned to **deny-by-default** (advisor verdict-3 APPROVE, inside the plan; security
   review F1/F3/F6). Allow list of programs; bare names only; TZ/LC_ALL/LANG/NO_COLOR only on a command; read operands
   and `<` redirects credential-checked; read-tool write/exec options refused; file tools' targets inside project or
   scratchpad; gh read-only; unterminated here-doc refused (F2); writePath/Read reordered + realpathed (F4/F5).
   300 tests pass, 37 mutants caught, a condition-6 message test. Day sessions unchanged. F7 a documented residual.
   Verdict: `.checkpoints/kit-v3-1/advisor-verdict-3.md`. Plan L2 updated (condition 8).

5c. (8f96128) Step 1e: globs + option abbreviations closed (security review 4th pass I1-I3). 329 tests, 44 mutants.
6. (769fcaf) Step 2: the static settings layer — ask emptied, allows added, denies extended, guard registered on the
   three events (onFailure block, ${CLAUDE_PROJECT_DIR} path), syntax-gate path fix, effortLevel high. settings.test.mjs
   82 tests pass. Gate PASS.

## SCOPE RESET (Advisor + Mustafa, 2026-10-27, authoritative — supersedes the allow-list design above)
The security-review plugin kept finding option-surface gaps because the guard allow-listed shell file tools. The Advisor
reset the scope: **finite, fixed, test-what's-allowed, no more hardening rounds.** Done in ee2a384:
- Routine shell = git, gh, `npm ci` (exact), node/python on main's tracked scripts, date (clock). Everything else denied.
- Reading/searching via the Read/Grep/Glob tools (credentials refused); editing via Edit/Write (protected refused).
- Routines never read the web (WebFetch/WebSearch denied). Research is daytime-only (night shift stops doing R1-R6 web work).
- Pushes to main / force / edits to .claude/ or CLAUDE.md denied; merges fail closed; nothing ask-first. PR-HOME kept.
- Removed ~60 lines (ROLE_READ/ROLE_WRITE and all file-tool helpers, the web allow-list). Test suite: 190 (was 340+),
  40 mutants, gate green. Day sessions unchanged.
- **Residual risk (for the report):** a routine could still misuse an allowed program within its scope (e.g. a clever
  git invocation), or read a project file that holds a secret the project shouldn't track. The platform's walls carry
  the rest: isolated cloud machine, branch-scoped pushes, protected main, reviewed PRs. No further hardening rounds.
- Model: kit-v3-1 (security-heavy) finishes on Opus 4.8; game/engine work moves to a new Opus 5.5 session.

## Decisions taken by Mustafa/Advisor (2026-10-27, this session's chat)
- **#27 MERGED** (merge commit 07ab240); R7 is on main; R1-R6 unblocked. Unsubscribed from #27.
- R7-MARKS (a); MANAGED (b) → RESIDUAL (guard lives in the repo, acceptable; Mustafa isn't a plan admin).
- PR Advisor routine: switch off until the test run passes. **The API refused me (http_api-created routine); Mustafa
  must do it** at https://claude.ai/code/routines/trig_01YPyWAsQmsaQoydhMGG3CkJ.

## Current step
7. Build step 3 (in progress): NIGHT_SHIFT.md role check (check 0) + 08:10 window + guard/no-prompts note DONE. Still to
   do: NIGHT_SHIFT saver-stop via TaskStop, mark-ready via MCP; advisor.md (PR step 0 role check; the new gate bar); CHECKPOINTS.md
   (saver stop via TaskStop); CLAUDE.md (web-fetch sentence, playwright cli.js line); START_HERE/tools README (playwright line).
   advisor.md (PR step 0 role check; the new gate bar), NIGHT_SHIFT.md (start check 0;
   00:10->08:10; stop saver via TaskStop not pkill; mark-ready via MCP not `gh pr ready`), CHECKPOINTS.md (TaskStop),
   CLAUDE.md (web-fetch sentence), and the proposal's other procedure changes (playwright cli.js, no git -C, named
   branch on push, explicit git add paths, git rm for checkpoint deletion, MCP-only GitHub writes, agent memory local).
   Then step 4 (builder.md, effortLevel already set, memory local, Models-and-effort sentence) and step 5 (checkpoint.mjs
   percent-escape fix + test, text fixes).
   Note (verdict-3 condition 6): routine gh is read-only, so the night marks its PR ready via the GitHub MCP
   update_pull_request(draft:false), not `gh pr ready`; the saver stops via the TaskStop tool, not pkill.

## Next steps
1. Build in this worktree, in the plan's order: settings.json, the procedure texts, builder.md, checkpoint.mjs's
   percent-escape fix, the text fixes; gate after every step.
2. Reviews on the final diff: security-auditor, test-engineer, infra-engineer; TEST-RUN per the decision; report;
   delete this checkpoint; PR ready; advisor review; Mustafa's go.

## Open findings
- The stuck PR Advisor run session_01FPXGo8GioEMFrCm8FhivMU now shows ABANDONED (checked 2026-10-27 13:20).
- PR-TRIGGER (filters: author Mergincanli, not draft) closes the attack's critical 1 for outsiders: Mustafa's UI.
- Build finding, for the advisor (scope question): the plan's routine Bash rules name the programs routines may not
  run (a deny list). A deny list can't name every program, so a program it doesn't name runs. Proposal: for routines
  only, an allow list of programs called by name (git, gh, node and python under the existing interpreter rules, npm in
  its exact form, read-only text tools, and the file tools with their target checks), Bash writes kept inside the
  project and the scratchpad, and environment settings on a command limited to a short list. Day sessions unchanged.
  Not built: it widens the approved plan, so it waits for the advisor's answer.
- Design addition beyond the plan's letter, for the final review: the launcher (guard.mjs) split from the rules.
- **Automated security review of 3f43a7a (security-guidance plugin), 7 findings on routine-guard.mjs, 2026-10-27 17:05.**
  It converges with the scope question above (sent to the advisor as verdict-3). Disposition:
  - F1 HIGH program-name deny-list evasion (unknown program → null): **fixed** in 0e93ac4 — routine Bash is a program
    allow list (git, gh, node, python, npm, named read/file tools); everything else refused. Advisor verdict-3 APPROVE.
  - F3 HIGH guard self-modification (WRITERS can't name every writer): **fixed** — file tools check every path they
    write against protectedRel; non-listed writers (ln, dd, chmod, sed, patch, install, shred) are off the list.
  - F6 MEDIUM gh deny-by-default (an unlisted gh subcommand → null): **fixed** — routine gh allows only api (read) and
    pr/issue view|list|status|diff|checks.
  - F2 HIGH heredoc parser differential: **fixed** — an unterminated here-doc is refused for a role; `<<-` strips tabs,
    `<<` does not; a quoted delimiter is matched literally.
  - F4 MEDIUM fail-open ordering: **fixed** — writePath and writeTarget check protected/scope before the scratchpad.
  - F5 MEDIUM credential-read bypass: **fixed** — the Read path is realpathed before matching; read operands and `<`
    redirects are credential-checked; bare env/printenv refused.
  - **Second review pass of 0e93ac4 (4 findings), 2026-10-27, all fixed in step 1c:**
    - G1 HIGH fd's -x/-X/--exec: fixed by dropping fd from the read list.
    - G2 HIGH credential path not normalized: fixed — readOperandDeny now realpaths the operand (resolved against the
      command's folder), credential-checks it, and confines reads to the project, the scratchpad and /opt/pw-browsers;
      option-attached file values (-fFILE, --file=FILE) are checked too.
    - G3 HIGH cp/mv/rm directory clobber (rm -r .): fixed — recursive cp/mv/rm refused; writeTarget refuses a target
      that is the project root or an ancestor of .github/.git/.claude/tools (coversProtected).
    - G4 MEDIUM sort/xxd/tree write options: fixed by dropping sort, xxd, tree (and jq, od, etc.) from the read list;
      the list is now a small, obviously-safe core plus find (the one kept tool with options, a closed set refused).
  - **Third review pass of 1dafe0f (4 findings), 2026-10-27, all fixed in step 1d (dca6a1a):**
    - H1 HIGH `--` end-of-options / bundled values skipped: fixed — the read token scan checks every operand after `--`
      and extracts bundled/`=`-attached file values.
    - H2 HIGH bare-word credential not checked (`cat .netrc`): fixed — readOperandDeny credential-checks every operand,
      bare names included, and treats the whole `.git/` directory as sensitive.
    - H3 MEDIUM `--recursive`/`--archive` abbreviations: fixed — abbrevOf catches `cp --recur`.
    - H4 MEDIUM cp/mv/rm directory clobber: fixed robustly — bash mv is off (git mv / Edit), bash rm is allowed only on
      untracked paths (tracked go through git rm), recursive cp/rm refused, coversProtected backstop kept.
    322 tests, 41 mutants. Probed: credentials/outside-project/recursive/mv/tracked-rm deny; legit reads + untracked rm pass.
  - **Fourth review pass of dca6a1a (3 findings), 2026-10-27, all fixed in step 1e (8f96128):**
    - I1 HIGH cp/rm long-option abbreviation (`cp --recur`) + globs in write paths: fixed — long options off cp/rm
      (short flags only), and a glob in any write path refused.
    - I2 HIGH glob in a read path expands past the guard: fixed — globs refused in read paths (find keeps -name
      patterns via globOk).
    - I3 MEDIUM grep pattern-option by abbreviation (`--regex=`): fixed — abbreviations detected, and the grep pattern
      is always credential-checked (scope check skipped). 329 tests, 44 mutants.
  - **Fifth review pass of 769fcaf/dca6a1a (7 findings across guard + settings), 2026-10-27, all fixed in step 1f (3ada4b5):**
    - J1-J3 (glob/brace/~ expansion differentials): fixed with the right abstraction — parseShell records whether a
      word's glob/~ was quoted; a routine command with an unquoted `* ? ~` is refused at the command level (the shell
      would expand it past the guard). Quoted globs are literals and pass (`find -name "*.md"`, `grep "a*b"`). Brace/
      bracket in a file path refused in readOperandDeny/writeTarget. This subsumes the per-operand glob checks.
    - J4 (cp -a bundled): fixed — cp's recursive check is `/^-[^-]*[rRa]/`.
    - K1 (merge fail-open): fixed — merge_pull_request off the allow list, and mergeCheck fails closed for every
      session (a check that can't complete refuses).
    - K2 (npm audit): narrowed to two read-only forms + deny `npm audit *fix*`.
    - K3 (node .gate/*): narrowed to .gate/*.mjs and .gate/*.js.
    341 guard + 82 settings tests, 46 mutants. The suite caught a real bug (the recursive check had lost its cp/rm
    guard and would have denied `touch -r`).
  - **Guard status:** substantially complete after 5 automated passes. The final security-auditor review (step 5) is
    the human gate; the automated passes are converging (HIGH->HIGH->HIGH->MED->MED).
  - F7 MEDIUM comment-ownership (shared Mergincanli identity: can't tell a routine's comment from Mustafa's by the
    API): a genuine residual. The API shows the same author for both. Mitigation already in place: night edits only
    #16 and its own PRs, PR Advisor only its review comments on PRs, never #16. Note it in the report as a known
    residual; a stronger check (a marker line in the routine's own comments) is a later kit item, not this one.

## Key numbers
- Ask rules on main today: 16, every one a stall in a routine. Allow rules proposed: 21; deny: 22.

## Decisions taken
- BRANCH-2 (a) and SPEND-1 (b), the Advisor, 2026-10-27 (in the session's chat): push cs/kit-v3-1; kit-v3-1 lands
  first if it collides with cs/v2-research; review rounds stop once findings are only nits.

## Running work
- Advisor plan review (a background agent in the implementer session), notes in the scratchpad kit-v3-1/.
