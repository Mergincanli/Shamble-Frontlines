# The advisor's check of the U4d2 builder brief (rev 1.1), 2026-11-12

Seat: a day session's advisor (workflow agent, opus), on main's `.claude/agents/advisor.md` (read with `git show
origin/main:.claude/agents/advisor.md`). Read only: no build, no run, no commit. Branch
`claude/netcode-proof-test-plan-hcky5g` at f94b078 (the main folder's head). The brief: u4d2-brief.md rev 1.1, sha256
e6bcefc841dd963dd4ce9c4bd72b3dc62efa5bf11aea2eb4aa06c3c51b9268fa (929 lines), as the task names it.

## Plan
1. Read the brief whole (N/u4d2-brief.md rev 1.1), the check (N/u4d2-brief-check.md) and the drafter's notes
   (N/u4d2-brief-notes.md, "Revision for the check").
2. Read the plan's §5 U4d2 (:666-702) and the parts of the plan, plan.md and the ADRs the brief leans on.
3. Read my conditions on U4d2 (advisor-u4.md 2, 7, 8, 10 (b)), W1-TARGET and W1-PUB's U4d part, the U4d-F ruling's T4,
   j-r2's B2.3 and B6, e-ruling's F2-ROUTE and L1-L4, e-r1's R6 and R9 and T2/T6.
4. Read carries "## U4d2 or the main session" (29, 41, 42, 52, 65, 66) and the recordings section (67, 71, 72).
5. Coverage table: every item of §5 U4d2 and of my conditions against the brief, testable as worded.
6. The check's findings TB-1 to TB-20 against rev 1.1 (TB-1 to TB-3 first; TB-7 and TB-11 with Q12).
7. Spot-check the brief's code claims at the head (crates sim, lab, l1, tcpinfo; check.mjs, check.test.mjs; Cargo.toml).
8. Rule U4D2-Q1 to Q12, with exact words.
9. The placeholders P1 to P8, 7.2's launch steps, `<base>`, the runner, U4e merging first; the lanes beside U4e.
10. The main session's points (task item 6).
11. Verdict, conditions, Decisions needed, For Mustafa.

## Blocks (written as each finishes)
