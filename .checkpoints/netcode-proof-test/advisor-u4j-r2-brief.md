# The advisor's brief for U4j's round-2 ruling (given 2026-11-11 about 17:55 UTC, as a general-purpose agent, opus, xhigh)

Kept so a restarted advisor can take the same brief: if advisor-u4j-r2.md is incomplete and no notice came back, start
the advisor again with this text; it continues the file after its last complete block.

You are the Advisor for SHAMBLE FRONTLINES. First read your role and rules and follow them: run `cd
/home/user/Shamble-Frontlines && git show origin/main:.claude/agents/advisor.md` (main's copy wins over any other copy).

**Task:** rule on U4j's round 2 (the netcode proof test, PR #40, spike branch claude/netcode-proof-test-plan-hcky5g,
notes folder /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test, below N), and say whether U4j merges into the
spike branch now, and on what conditions.

**Facts:**
- U4j is the judge and marks change (npt-judge, npt-log, marks/). Round 1 (three REVISE; the security-auditor's F1 High,
  the FAR BETTER cap hole) led to your ruling N/advisor-u4j.md (FX-1 to FX-14, FU-1 to FU-6, R3's readings (a) to (f),
  R5). The fixer applied all 14 fixes in /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50
  (uncommitted there; saved on the branch as N/wip/u4j.patch at 4ad80a9, 64 files on base 411eeee). Its own account:
  N/u4j-build.md, "Fix pass, round 1" (claims, not evidence).
- Round 2 (netcode-architect, security-auditor, test-engineer) started from a review packet cut by script
  (N/u4j-round2-packet.md; N/review_packet.py with N/u4j-round2-packet.json). All three returned APPROVE WITH FINDINGS:
  each finds F1 closed (the test-engineer by runs and your three plants), no new Critical or High. The test-engineer's
  counted check.mjs run passed all 10 steps (measured 317 = 286 + 31, bites 340 = 309 + 31, self 48), and all 66 changed
  and new files match the start hashes.
- Their notes, with every finding in full: N/u4j-round2-netcode.md, N/u4j-round2-security.md, N/u4j-round2-test.md
  (runs in N/u4j-round2-test/).
- The review budget (CLAUDE.md): round 2 was the last ordinary round; a round 3 only for an open Critical or High;
  anything below becomes a follow-up or a carry; a Low never triggers a push. Mustafa, 2026-11-11 (verbatim in
  N/history.md): "U4j's High: carry on as the review budget says. If it is still open after round 3, stop and bring it
  to me."

**Rule on each, with reasons, and with exact words wherever text changes:**
1. **The merge:** does U4j merge into the spike branch now (the main session snapshots the worktree, applies the patch
   on the branch head, runs check.mjs counted and the gate, and commits with wip/u4j.patch removed)? Any condition
   before or inside the merge commit?
2. **Round 2's three Mediums**, each routed as a carry (give its words and its section in N/u4s-carries.md), a
   follow-up with an owner, or a fix inside the merge commit (only where the review budget allows one):
   - NC-U4J-R2-N1: the writer rule for a tick written lost while L1 reorders datagrams (the reviewer offers one
     sentence for U4e's and U4r's carries, and folds N6's two missing writer rules into it);
   - TE2-1: no committed test pins the LG part of the Arm rule's key (FX-5 (b)); the reviewer proposes one engine case
     and a README row's words;
   - TE2-5: outside U4j's code, an npt-net test failed once on the bites build in the fixer's check.mjs run 2 and was
     retried until green, with no test name and no cause; check.mjs's summary does not print the failing test's name.
3. **Round 2's Lows** (NC-U4J-R2-N2 to N6; SEC-R2-N1, SEC-R2-N2; TE2-2, TE2-3, TE2-4, TE2-6): route each in one line.
4. **The fixer's points** you were to rule on: SERIES.md's twin_hash row still says "any role" (TE2-6 too: give the
   row's words); the fixer's stricter reading of FX-3 (c), that two practice runs in force make a bite run INVALID
   (TE2-3); FX-10's malformed entries and its order of outcomes (TE2-4); FX-9's plant T also raising FX-1's new guard
   to 1.12.
5. **At the merge:** confirm what of R3's readings (a) to (f) and R5's orphan reading goes into the notes, and that PR
   #40's report names FU-1 to FU-5 as open security findings (they keep PR #40 needs-mustafa).
6. **The test-engineer's two process slips** (plants placed by whole-file Write, each restore hash-checked; three notes
   lines appended with python): does either weaken the evidence?

**How:** write your ruling to N/advisor-u4j-r2.md: your plan first, then each block as you finish it (save as you go;
if the file exists from a cut run, continue after its last complete block). Write files with the Write and Edit tools
only; run git only as `cd <path> && git ...` (never `git -C` or `git -c`). Edit nothing else, and commit nothing.
Build and run nothing unless a ruling needs a fact only a run can settle; then use `bash
/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4j-fix/run.sh <command...>` (it waits for other cargo
runs and uses the shared target), and take pass or fail from the exit code and the test runner's own summary line.
Severity on CLAUDE.md's one scale. End the file with "Decisions needed (Mustafa)" (only what you may not decide
yourself) and "For Mustafa".

**Return** a short summary, at most 15 lines: the merge verdict and its conditions, each Medium's route, and anything
for Mustafa.
