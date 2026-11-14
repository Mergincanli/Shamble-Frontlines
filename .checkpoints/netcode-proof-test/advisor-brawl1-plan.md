# Advisor: plan review of BRAWL-1's lane (npt-lab, PR #40)

## My plan
1. Read brawl1-plan.md (sections 1 to 13, Q1 to Q8).
2. Read my rulings: advisor-u4r-sets-r2-full.md (BRAWL-1, Fix (a)-(d), next steps 4-6); advisor-u4r-r2-full.md (item 18,
   N2-1, TE-R2-1, TE-R2-2, RR4-L1, RR4-L2); advisor-u4r-r1-full.md (EA-U4R-1, EA-U4R-2, RR3-L2 (a)); U4r build ruling 1
   point 7 (R16); FP-7 item 1 / carry 39; advisor-u4r-f3-plan-full.md (start figures).
3. Read brawl1-facts-full.md.
4. Check against code: Fix (a) source (client decoded snapshot vs room mirror); leash geometry; constants not
   rate-derived; Fix (b) figure; Fix (c) evidence + sim-only seam; runner recordings re-pin; det-u4.mjs 240 s; N2-1
   plant; TE-R2-2/RR4-L2 order; RR4-L1 place; EA-U4R-1/-2 form; order of work; disk plan; expected lines; reviewers;
   size.
5. Kept-logs draft (Q7).
6. Verdict.

## Checks (a line each as I go)
- C1 read: plan (13 sections, Q1-Q8); sets-r2 (BRAWL-1 Fix a-d, steps 4-6); r2 (item 18, N2-1, TE-R2-1/2, RR4-L1/2);
  r1 (EA-U4R-1/-2, RR3-L2 (a)); build1 point 7 (R16 is a brief facts row: "npt-client exposes no decoded record";
  point 7 routes U6 to "an npt-client read item (R16) or U4g's Interp"; no rule bars a pub item, but r1 said
  engine-architect rejoins when a pub item / dependency / seam change comes); facts-full (A1-A9, B).
- C1a build1 line 14: "A SHA-256 failure there means that unit re-pins, never a retry" (det-u4 at every merge touching
  the scenario path) vs INF-U4R-1 "a re-pin without a ruling is not allowed: a mismatch comes to me whole". Both stand:
  the unit re-pins, but on a ruling. Q3 asks for that ruling up front.
