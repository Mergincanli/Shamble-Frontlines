# Advisor: U4f builder brief rev 3.2 check (notes)

Started 2026-11-17 (after D2 18:54 UTC). Effort xhigh. Instructions read from origin/main:.claude/agents/advisor.md.
No cut run found (no notes file existed).

## Plan
1. Read the brief rev 3.2 whole (u4f-brief.md, 1,734 lines): sections 0-12, change lists (R11 block, R1-R10, C1-C28),
   questions U4F-Q1..Q18 (+ any added), D-items, section 12.
2. Read the plan's §5 U4f (u4-build-plan.md:805-816) and U4-BRIEFS (:1054-1056), plus the parts the brief cites.
3. Read u4s-carries.md "## U4f (clock and commands)" and the carries named.
4. Read my earlier brief checks (advisor-u4r-brief.md, advisor-u4e-brief.md) for form; my rulings that bear on U4f
   (grep advisor-*.md / -full.json for U4f, ClockSync, lead bound, B3, KL-U4F, KEPT-LOGS, SIZE-CHECK, command forms).
5. Read the round 1 and round 2 findings (TE3, SA3, TE31, SA31) and the reviser's answers (r3-notes, "Revision for
   round 2"); check each answer in the brief text.
6. Read D4 outputs (d4-output.md, d4b-notes.md, d4b-r31-full.json, d4-compare.md) for Q6.
7. Check the code at the head (spikes/netcode-proof) for the cites the brief leans on (ClockSync, Pong, commands,
   lead bound, controller).
8. Run the read-only anchor scripts (stamps_anchors.py, r31_check.py) for his words' anchors.
9. Rule on Q1-Q18 (+added); bars for Mustafa (5.6 table); build rules; D-items; missing items.
10. Write ruling file advisor-u4f-brief.md; hand back.

## Blocks
- B1 done: brief rev 3.2 read whole (1,734 lines; sections 0-12, NA table). Counts at 3.2: +109/+124, 64 neg,
  18 questions, 54 recheck rows, planned size 4,850, disk 3,607-3,727 MiB vs 1,372 above floor. Next: plan §5 U4f.
- (resumed after a cut, same day) B1 re-read into context whole (1,735 lines incl. NA table). Continuing at B2.
