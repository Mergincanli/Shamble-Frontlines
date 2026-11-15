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
- Snapshot: patch sha256 ee2973aa... = `git diff` of the read copy (both plain and --binary), HEAD 352a245. numstat
  10 files, 875+/123- (998).
- Point 2 checked: my own normalised diffs (brawl1-build.md:52's normaliser). A vs B: lines 46-47 only (wallclock L1
  hash 9375924675163200265/821df9d0 vs 1388086141469307493/134378fd). A vs e2: those two lines (L1 hash
  5648772452115156074/4e64789d) + line 20 (52->53 filtered out) + line 66 (58->59). Index 3, loopback hash
  6328158668484839173, compared 120, mismatching 117 equal in all four. pump.rs:228-246: aim[0] ^= 1|(ns&0xfe) from
  SystemTime (bites only) -> L1 hash is wall-clock dependent by design. +1 test = logs.rs rr3_l2_a (patch line 884), the
  only new #[test] in crates/lab. All four logs exit 0. Ruling: ACCEPT; condition 6 (ii) as written was unmeetable at
  <base> (my wording): record as ADV-BP-3 in the report. No diff change.
- Point 3 checked: base test `at = 799_693 / 12` (66,641) ok-assert now refused by cap (hash.rs:338 after bytes/12 at
  :325). Built: replace bytes 799693->360000 (at 30,000; 30,001 "cannot fit"); a silent no-match of replace would
  make at+1 = 30,001 read OK and fail expect_err, so the replace is self-checking. Cap boundary itself in r10_fp3
  (:1655-1668: 36,001 refused exact text; synthetic 36,000 reads). Ruling: ACCEPT. Facts gap (facts-full:113-116 named
  only r10_fp3) caught by p1-a-check.log:47. No diff change.
