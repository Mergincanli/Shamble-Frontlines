Task: npt-u4r-brief step 3 -- write rev 2 of the U4r builder brief (one revision pass for netcode-architect's check N1-N16). Notes only.

## Plan (blocks, in order)
B1. Read the check whole (netcode-architect-u4r-check.md) + its notes.
B2. Read rev 1 (u4r-brief-draft.md) whole + drafter notes (test-engineer-u4r.md).
B3. Read the approved plan (night-2026-11-10.md at 2dc1f31, "Rules for every item", "Item 4").
B4. Verify each finding's cite at df8975b (.gate/npt-spike), decide accept/reject; notes line per finding.
B5. Write rev2-parts/00-changes.md (change list).
B6+. Write each revised section to rev2-parts/<n>.md.
Bn. Assemble rev 2 into u4r-brief-draft.md in one Write; recount; DONE.

## Notes
- B1 done (check read whole: 16 findings; scope -30 pre-check, O1, RTT argument, -18 fresh-delta).
- B2 rev 1 read whole (785 lines; sections 1-11; R1-R32; Q1-Q12; run items N1-N9). Drafter notes still to skim.
- Extra defect seen: rev 1's §11 run items are named N1-N9, colliding with the check's finding ids N1-N16. Plan: rename the run items D1-Dn in rev 2 (list as extra change X1), update every cite (R2, R10-R12, R20, R29, U4R-Q3, rule 4, 7.3, 8.3).
