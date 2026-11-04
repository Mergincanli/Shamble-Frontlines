# Fixer notes: B3 fix round (worktree head bfaf142, not committed)

## Plan
Apply the 17 upheld findings (skeptic refuted=false), each with its own fix text:
1. GD-1: LOOP_V2 infiltrator open list (+ Shinobi stealth reveal, sabotage); same two items in STATUS's LOOP_V2 [OPEN]
   line (merged with F3/CC-3).
2. F1: README R0 "bears on" list: drop OQ-0001-9, R2-BUD-OQ5, R4-OBST-OQ1, OQ-0005-AB15 (main session's choice of
   four; the skeptic wanted OQ-0001-9 kept: recorded in the report); re-read the remaining 17 ids against his words.
3. F2: STATUS "faction asymmetry is answered in part" -> "bears on" wording.
4. F3 + CC-3 (+ GD-1): STATUS LOOP_V2 [OPEN] items line completed.
5. F4: untagged pointers after STATUS's four superseded lines (ammo, pencil, faction asymmetry, UPGRADE-SCOPE quote).
6. F5 + CC-7: STATUS:3 comma.
7. F6: README R0 paragraph: AB1 exception.
8. CC-1: LOOP_V2 §12 "Outside this file" bullet gains ADR 0004 part 17 (502, 518) and ADR 0005 (797, 991, 994).
9. CC-4: STATUS queue item, ROADMAP queue item, STATUS B3 row: B3's VISION_V2 hand-over to B2.
10. CC-5: README:124/125 qualify "B3"; :119 qualify "B3-Q1".
11. CC-6: queue/docs-pass-items.md marks "taken by B3 (PR #36)".
12. CC-8: STATUS:132, STATUS:84, ROADMAP:91 tag wording.
13. F7: the report docs/reports/2026-11-04-b3-design-walk.md with the appendix; STATUS B3 rows point at it.
Then checks: E1 against the appendix, gate --no-smoke, repo-map-check.

## Log
- F5 + CC-7 done: STATUS.md:3, ", " after "10:21 UTC)".
- GD-1 (LOOP_V2 part) done: LOOP_V2.md:104-105, the "Also open" list gains the Shinobi's stealth reveal and sabotage.
- CC-1 done: LOOP_V2.md:222-226 (the "Outside this file" bullet) names ADR 0004:502 (part 17), ADR 0004:518 (part 18, not part 17 as the finding said), ADR 0005:796-797, 991 and 994 (991 added per the skeptic).
- F2 done: STATUS.md:149-153, "faction asymmetry is answered in part" -> "faction asymmetry: his walk and B3-Q1 bear on it (OQ-0005-CD5); his unit list decides the rest"; the sub-bullet no longer says "pointers only".
- F4 done: untagged pointers "→ 2026-11-04: see "Taken by Mustafa, 2026-11-04, B3 on his list" above." after STATUS.md ammo (216), pencil (226), "Still open: faction asymmetry" (229) and after the UPGRADE-SCOPE blockquote (275, outside his quoted words).
- CC-8 (STATUS part) done: STATUS.md:131-134 names [OPEN] for the war and the one Advisor [DEFAULT] reading; the entry now points at the report file.
- F3 + CC-3 + GD-1 (STATUS part) done: STATUS.md:429-433, the LOOP_V2 [OPEN] items line gains the Shinobi's stealth reveal and sabotage, AB14's coverage and the ability-press question, and the war around the matches.
- CC-4 done: STATUS.md:27-29 (queue) and ROADMAP.md:71 (queue) carry B3's VISION_V2 hand-over (lines 36-37, 59, 86) to B2; STATUS.md:86 (B3 row) and ROADMAP.md:91 (B3 entry) say the same.
- CC-8 (STATUS:86, ROADMAP:91) done: the tag wording names [OPEN] for the war and the Advisor [DEFAULT] reading in §13.
- Bookkeeping: STATUS.md:86 B3 row (progress column now names 6a09137, the review round, the fix commit and the report file), STATUS.md:133-136 entry and ROADMAP.md:91 point at docs/reports/2026-11-04-b3-design-walk.md.
- F6 done: docs/adr/README.md:50-52, "each answered at most in part, except OQ-0005-AB1, whose last half is now confirmed (the advisor's reading, 2026-11-04)".
- F1 done: docs/adr/README.md:53-55, the "bears on" list drops OQ-0001-9, R2-BUD-OQ5, R4-OBST-OQ1 and AB15 (the main session's four), then on re-reading the 17 left also OQ-0004-11, R2-INT-OQ3 and R2-ROOM-OQ3 (not plainly touched); 14 ids stay. Reasons in the report.
- CC-5 done: docs/adr/README.md:124 and :125 ("The design-walk change-set (B3 on Mustafa's list, PR #36) routed its question on raw power / how the cards fill here."), :119 ("Mustafa's answer B3-Q1 (PR #36, the design walk)").
- CC-6 done: queue/docs-pass-items.md "For the NEXT docs pass": items 1, 2, 3, 6 and item 5's STATUS/ROADMAP part marked "Taken by B3 (PR #36)"; item 5's other places, item 4 (done earlier), item 7 (no change) and the routine proofs left as they are.
- F7 done: docs/reports/2026-11-04-b3-design-walk.md written (body lines 1-81, appendix 82-151: his walk, B3-Q1..Q3 as sent, his answers, each block identical to its source by diff).
- Checks: E1 rerun on the appendix (scratchpad/e1-appendix.mjs): 43 pieces (33 sentences, 10 answer clauses), each once in the appendix, 0 failing. Builder's b3-check.mjs: PASS, 0 failing. gate --no-smoke: GATE PASS, +0 -0. repo-map-check: OK (113 entries, 100 paths).
- Note: the finding counts are 18 total, 15 upheld (13 distinct), 3 refuted; the task text's "17" overcounts (its own list has 14 ids plus F7).
- Final line numbers: LOOP_V2 104-105 (GD-1), 219-223 (CC-1); STATUS 3 (F5/CC-7), 27-29 (CC-4), 86 (CC-4, CC-8, report), 133-136 (CC-8, report), 153-157 (F2), 216/226/229/275 (F4), 431-435 (F3/CC-3/GD-1); ROADMAP 71 (CC-4), 91 (CC-4, CC-8, report); README 50-55 (F6, F1), 119/124/125 (CC-5).
