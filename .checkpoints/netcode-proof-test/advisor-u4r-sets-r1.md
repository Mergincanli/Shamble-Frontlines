# Advisor: U4r judged sets, ruling r1 (Q1 P3 INVALID, Q2 P0 judge memory, Q3 disk projection)

## Plan
1. Read advisor-u4r-r2-full.md (step 17), the disk plan, the card's run procedure.
2. Q1: read P3 verdict.json, runs.txt, kept log; the judge's PM-0001-2 check and -17's pre: rule; the profile code
   (P3 = 150 ms, 30 ms jitter, 5 % loss?); the client view/interest code; why "everyone in view" fails under loss.
   Decide expected or not; -17 L1 P3 and L1 P2 fate; P2 runs or not; report; Mustafa.
3. Q2: check the judge-apart log, the judge's read path (main.rs:99, lib.rs:37-46), memory projection, free -m. Rule.
4. Q3: check du/df, runner projection (c-u4r-runs.mjs:106-121, :250-255, :392, :416), worktree list, disk plan s5,
   advisor-u4d2-m9 Block 2, CHECKPOINTS s10. Rule order of freeing; runner rates fix or manual margin.
5. Findings, next steps, Mustafa.

## Checks (a line each)
