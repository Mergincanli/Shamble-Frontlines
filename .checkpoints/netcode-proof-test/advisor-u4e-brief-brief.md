# The advisor's brief for its check of the U4e builder brief (given 2026-11-11 about 20:30 UTC, general-purpose, opus, xhigh)

Kept so a restarted advisor can take the same brief: if advisor-u4e-brief.md is incomplete and no notice came back, start
the advisor again with this text; it continues the file after its last complete block.

You are the Advisor for SHAMBLE FRONTLINES. First read your role and rules and follow them: run `cd
/home/user/Shamble-Frontlines && git show origin/main:.claude/agents/advisor.md` (main's copy wins over any other copy).

**Task:** check the U4e builder brief before U4e's build (the netcode proof test, PR #40, spike branch
claude/netcode-proof-test-plan-hcky5g; notes folder /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test, below
N). U4 wave 2's sub-unit U4e is the room and the codec path (N/u4-build-plan.md §5 U4e). Every wave-2 brief goes to you
before its build (the U4 rules). Say whether the brief is ready, and rule on its open questions.

**Facts:**
- The brief: N/u4e-brief.md, rev 3.2. Its history: rev 2 (the night shift's draft); rev 3 (the netcode-architect, with
  your W1-U4E answers; its notes N/u4e-brief-r3-notes.md); the test-engineer's check of rev 3 (N/u4e-brief-r3-check.md:
  FAIL, three High (F1 to F3), five Medium, five Low); rev 3.1, the netcode-architect's revision (all thirteen fixed,
  none rejected; a new question U4E-Q14); two folds by the main session, each described under the brief's title: before
  the check (your U4d close rulings, N/advisor-u4d-close.md) and at U4j's merge (rev 3.2, by N/u4e_fold2.py: section 3.1
  re-copied from N/u4s-carries.md §U4e with carry 61, rows R27, R28 and R38 rechecked, the lines that called U4j
  unmerged updated). The drafter's notes and the checker's notes are their own accounts: readable, not evidence.
- U4j (the judge and marks) merged at 4d2b0ef and d50396a, under your round-2 ruling N/advisor-u4j-r2.md (its B2.1 made
  carry 61 for U4e). At d50396a: check.mjs 10 steps, measured 396, bites 443, self 52 of 52. Wave 1 merged before it:
  U4a 952f734, U4b 411eeee, U4c f3f7c2f, U4d 39f4290. U4d-F (the estimate scene fix, carry 52's) has not built yet;
  the brief handles that case (carry 56). `<base>` stays a placeholder: the main session fills it at launch.
- Sources: N/u4-build-plan.md (§5 U4e, §4.1 to §4.15, the Lanes, §6, §7); your rulings N/advisor-u4.md,
  N/advisor-u4-wave1.md (W1-U4E :437-498, W1-S1, W1-N2, W1-TARGET :558-587), N/advisor-u4d-close.md, N/advisor-u4j.md,
  N/advisor-u4j-r2.md; N/u4s-carries.md §U4e (carries 21-27, 39, 50, 51, 53-57, 61); the message catalogue
  docs/design/netcode-proof-messages.md; the merged code at the branch head (spikes/netcode-proof/crates/{sim, proto,
  server, room, client, log, judge}, marks/); .claude/rules/netcode.md.

**Rule on each, with reasons, and with exact words wherever text changes:**
1. **Ready or not:** APPROVE (the build may start once `<base>` is filled) or REVISE (with each change's exact words).
   Check at least: every test, negative case, bite and carry of §5 U4e is in the brief and testable as worded; the
   check's three High findings (F1 to F3) are fixed as rev 3.1 claims; rev 3.2's fold is right (carry 61's note, R27,
   R28, R38, section 0's note on the plan's line numbers); the build rules carry W1-TARGET's rule; nothing in the brief
   goes beyond the plan.
2. **The brief's open questions** (its section 8): U4E-Q12, U4E-Q13 and U4E-Q14, each answered.
3. **One routing question from the main session:** U4c's netcode round-1 finding F2 (Medium, open: npt-lab records only
   the SHA-256 of the whole fate log, so no npt-lab run shows L-c8's hash half; u4c-review-netcode.md:53-54). L-c8's
   judged re-run on U4j is done (N/u4c-lc8-rerun.md, commit 9cd7c39): its hash half holds through U4c's scratch harness
   (equal whole-log hashes). The main session proposes a carry for U4d2 (crates/lab is U4d2's alone in wave 2:
   u4-build-plan.md :842, :862). Rule on the route, and give the carry's words if it is one.

**How:** write your ruling to N/advisor-u4e-brief.md: your plan first, then each block as you finish it (save as you go;
if the file exists from a cut run, continue after its last complete block). Write files with the Write and Edit tools
only, and rewrite your file whole with Write if you have no Edit tool; run git only as `cd <path> && git ...` (never
`git -C` or `git -c`). Edit nothing else, and commit nothing. Build and run nothing: the main session runs the bites
build's tests at this time under the shared target's exclusive lock. Severity on CLAUDE.md's one scale. End the file
with "Decisions needed (Mustafa)" (only what you may not decide yourself) and "For Mustafa".

**Return** a short summary, at most 15 lines: the verdict, each question's answer, F2's route, and anything for Mustafa.
