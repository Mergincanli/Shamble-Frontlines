# Advisor: BRAWL-1 builder's points (rulings)

Task: rule on brawl1-points.md points 1 (TICKS-NAME), 2 (PROOF-E), 3 (r10_carry_82 boundary), and the "no option
asked" items. Snapshot: wip/brawl1.patch sha256 ee2973aa...; read copy .claude/worktrees/brawl1 (352a245 + uncommitted).
Runs only through u4r-run.sh, logs under .gate/u4r/brawl1-advisor/.

## Plan
1. Verify snapshot sha256 and that the read copy's diff equals the patch.
2. Read my plan review (advisor-brawl1-plan-full.md, conditions 1-18), RR4-L1 (advisor-u4r-r2-full.md:92-97),
   carry 39 and carry 18 (u4s-carries.md), check.mjs:755-806.
3. Point 1: read hash.rs:239, 338-342, 1651-1668; p1-a-check.log; decide the name/route.
4. Point 2: read condition 6 (ii), the logs p1-before-a/b, p1-after-e/e2, normaliser (brawl1-build.md:52), pump.rs.
   Diff them myself.
5. Point 3: hash.rs:1554-1577 vs base; facts-full.md:113-116.
6. Also-recorded items: selector reruns, p1-a-check -> check2, size (numstat), pattern (iii).
7. Further findings; StructuredOutput.

## Progress (a line per point)
