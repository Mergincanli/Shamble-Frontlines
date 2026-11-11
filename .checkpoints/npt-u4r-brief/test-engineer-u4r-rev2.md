Task: npt-u4r-brief step 3 -- write rev 2 of the U4r builder brief (one revision pass for netcode-architect's check N1-N16). Notes only.

## Plan (blocks, in order)
B1. Read the check whole (netcode-architect-u4r-check.md) + its notes.
B2. Read rev 1 (u4r-brief-draft.md) whole + drafter notes (test-engineer-u4r.md).
B3. Read the approved plan (night-2026-11-10.md at 2dc1f31, "Rules for every item", "Item 4").
B4. Verify each finding's cite at df8975b (.gate/npt-spike), decide accept/reject; notes line per finding.
B5. rev2-parts/00-changes.md (change list).
B6. rev2-parts/01-head.md (title + intro) and 02-s1-2.md (citations + sections 1-2).
B7. rev2-parts/03-s3-6.md: sections 3-6.
B8. rev2-parts/04-s7.md: section 7.
B9. rev2-parts/05-s8.md: section 8.
B10. rev2-parts/06-s9.md: section 9 (recheck table).
B11. rev2-parts/07-s10-11.md: sections 10-11.
B12. Assemble rev 2 into u4r-brief-draft.md in one Write, order 01, 00, 02, 03, 04, 05, 06, 07; recount; DONE.

## Notes
- B1 done (check read whole: 16 findings; scope -30 pre-check, O1, RTT argument, -18 fresh-delta).
- B2 done: rev 1 read whole (785 lines; sections 1-11; R1-R32; Q1-Q12; run items N1-N9). Drafter notes read.
- X1: rename rev 1's run items N1-N9 to D1-D9 (+ new D10-D12); line 259's "arch N2" is carry 20's words: kept.
- B3 done: plan item 4 = draft, check, test-engineer revises once (rev 2 + change list). No build/run/web.
- B4 verified at df8975b (all 16 ACCEPTED, 0 rejected):
  - N1: l1/src/lib.rs:456-458, :784-785; plan :755-757.
  - N2 + X3: P0 ack sample applies at its packet's send instant (l1 :515, :418-420, :602, :686, :527, :582, :661, :714); link :79-80. Recommend (b') at U4r.
  - N3: plan :755-758; §4.9 :341-344. Receipt instant; U4h carry premises; Loopback class.
  - N4: advisor-u4.md:346; marks.json:6389, :6344, :6353; third reading not offered (l1 :115-117).
  - N5: room/server lib.rs:1-5 docs only. R33 + D10.
  - N6: §4.15 :463 header has no start tick -> recording run's tick_start (R30). X2 admit tick per leg (link :154-163, :187; l1 :30-31, :727-728). Test r6.
  - N7: plan :590; §4.6 :300-302; marks.json:6361. X4 room-log World hash (plan :634).
  - N8: plan :759; marks.json:6377; accepted-command-set example dropped.
  - N9: §4.7 :321-326; marks.json:4269, :4298, :4307; catalogue :90, :150. R34 + D11.
  - N10: §4.7 :321-324. N11: plan :753-755. N12 (a)-(d). N13 + X5. N14 + X6. N15. N16.
- Q12 costs: (i) 6 runs 1,800 s; 9 runs 2,700 s. (ii) 10 runs + 2 rec 3,600 s; 12 runs + 3 rec 4,500 s. Q10 (a') +2 runs (600 s).
- B4 done. B5 done (00-changes.md: N1-N16 accepted, X1-X7).
- B6-B11 done (parts 01, 02, 03, 04, 05, 06, 07 in rev2-parts/).
- B12 done: u4r-brief-draft.md rewritten as rev 2 (about 1,010 lines) from the parts, with the two at-assembly fixes (section 2 intro sentence; X7 line). Grep check: no stray run-item "N<k>" left (all "check Nk", carry 20's "arch N2", or the rename note).
- Recount (grep on the draft): 7.1 table 12 rows (11 judged parts: -17 x4, -57 x4, -58 x2, -18 x1; plus the -30 pre-check row, not judged); carries quoted 4 (40; 6, 14, 20); recheck rows 34 (R1-R34; new R33, R34; R2, R4, R9-R12, R15, R17, R20, R26, R29, R30 changed); advisor questions 12 (U4R-Q1..Q12), all open, 0 answered (Q10, Q12 rewritten; Q2, Q3, Q6 touched); needs a run (day) 12 (D1-D12; new D10, D11, D12).
- Not settled (needs <base>, day): R2/D2 (rtt_ns before first sample after U4c); R4/D12 (server-end first-sample instant at P0; whether Q10 (a') can fire); R15 (where pump reads rtt_ns; session kind for Loopback); R17 (option B lookahead; late counting); R30 (World hash name; recording start tick); R33/D10 (RTT readers, replace/clip paths); R34/D11 (client Resync; event cap).
DONE
