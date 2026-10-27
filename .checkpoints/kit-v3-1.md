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

## Current step
6. Build step 2: settings.json (allows, denies, the empty ask list, `effortLevel`, the hook registration) and its test.
   Step-3 note: drop `gh pr ready` from the night procedure (routine gh is read-only now) — mark ready via the GitHub
   MCP `update_pull_request` (draft:false); replace `pkill -f checkpoint.mjs` with the TaskStop tool in NIGHT_SHIFT.md
   and CHECKPOINTS.md (verdict-3 condition 6).

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
