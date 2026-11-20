# Advisor: U4f-F1's close, -20's L1 re-run held to the expected figures (2026-11-20)

Seat: the advisor agent, day session, read-only (main's advisor.md, read with `git show origin/main:.claude/agents/advisor.md`).
Branch claude/netcode-proof-test-plan-hcky5g at e51d1ef; nothing switched, nothing committed.
Why: my head check's condition 3 (advisor-u4f-f1-head.md:109-113) and my pass check's condition 3
(advisor-u4f-f1-pass.md:192-197): hold -20's L1 re-run to my expected figures; any other difference stops for me; read
FP-5 (i)-(v) in the -20 L1 record and F1's report. Plus condition 10 of my plan review (advisor-u4f-f1-plan.md:222-225).

## Plan
1. The tree: `git rev-parse 96de6ac:spikes/netcode-proof 1faf894:spikes/netcode-proof` (both d4f6cd41...); 46ca143
   touches only the results folder; the run header names HEAD 96de6ac and that tree.
2. The committed results (46ca143): runs.txt against 350677f's, line by line, wall and binary sha256s aside; verdict.json
   and verdict-bites.json leaf by leaf (jdiff.py, or my own python), every changed leaf named.
3. The expected list, item by item: summary lines; per-tick `h` (compare_20.py on both raw-log folders, re-run by me in
   the background, or spot-checked); lost 0, doubled 0; presses B 432,000 x3, A 432,003 / 432,003 / 432,009; -2 SS PASS
   6/6; -20 A and B PASS 3/3; L2 NOT-RUN; the bite at 0.894544 with lost 3,614 to 0; the reorder row "not available".
4. FP-5 (i)-(v) and ADV-U4FF1H-1's correction, in the -20 L1 record (u4f-report.md item 5) and in F1's report, each
   against its source line in r1 and the pass check.
5. Condition 10: u4f2-brief.md rev 1.2 (base 1faf894, counts 642/739, C-U4F2-9 and -10 as ruled, block 4's T1 line,
   section 6's line), against u4s-carries.md's filed texts and r1 :126-127, :159-165.
6. Findings (ADV-U4FF1C-n), verdict, what I did not check.

## Blocks
