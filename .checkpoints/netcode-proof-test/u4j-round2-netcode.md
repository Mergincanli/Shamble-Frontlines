# U4j round 2: netcode-architect review notes

Reviewer: netcode-architect. Round 2 of at most 2, on what changed since round 1 (.gate/u4j-prefix vs
.claude/worktrees/agent-a3b8dfeb01ed52b50). Never edit a copy; commit nothing; commands only through
.checkpoints/netcode-proof-test/u4j-fix/run.sh; git only as `cd <copy> && git ...`.

## Plan
1. Read the review packet in full (diffs, plan U4j section, design rules, advisor FX-1..FX-14 rulings, round-1
   findings, Mustafa's bars, carries).
2. Find my round-1 findings NC-U4J-1..-14 and how the advisor routed each (fix / follow-up / carry).
3. For each routed-to-fix NC finding: CLOSED or NOT CLOSED with file:line evidence in the fixed copy.
4. README paragraphs: ws-practice rule (FX-3 words) and hs writer rules (FX-12 words) vs advisor R6, R8 (v) and
   Mustafa's two quotes (WS-BAR refinement; NETEM-BAR (a)), word for word where rulings give words.
5. Coverage floor (FX-10) and Arm rule key (FX-5) vs plan §4.6 and ADR 0001 -17 / -19 rows.
6. Can U4r's runner and U4c's L-c8 re-judge meet every writer rule as written (process names, arch, role,
   snap_incomplete lists, clients header).
7. Run the judge's tests through run.sh (exit code + test runner summary line).
8. New defects: severity, file:line, scenario, cheapest proof, before-merge flag.
9. Verdict.

## Progress log
- (start) plan written.
- step 1 partly: packet read (src diff, README/SERIES/marks diff, rulings FX-1..14, FU-1..6, carries, R2, R4, my
  round-1 F1-F14, Mustafa's bars, §4.6, §4.12, carry 54). My routing: 1->FX-1, 2->FX-2, 3->FX-12, 4->FX-4,
  5(i)->FX-10, 5(ii-iv)->FX-12, 6->FX-5(b)+FU-5, 7->FX-7, 8->FX-5(a), 9->FX-6, 10->R2.4 U4r carry, 11->FX-12,
  12->FX-5 fixtures, 13->U6-b carry, 14->FX-11.
