# stop-engine-architect-a notes

- 13:05 started; files created. Next: read inputs (answers, verdict r4, map, plan).
- 13:08 read inputs (answers, verdict r4, plan r4, map). Next: read my places in 0002, 0005, GLOSSARY section 8.
- 13:20 read all my places in 0002 (47-48, 229-234, 510-515, 540-543, 710-713, 746, 894-898, 1053-1070, 1073-1076, 1373-1378, 1505, 1508), 0005 (header, 60-94, CD-2, CD-6, CD-13, CD-14, AP-2, AP-3, AP-1 OQ, trade-offs, frame table, OQ-0005-1, revisit, rollback, appendix) and GLOSSARY 782-797. Plan: H23 NOTE (ruling) + NOTE H23-2 for test-engineer (0002:1332-1333); FIX H24 (S23 covered by H24); next: grep approval wording then write blocks starting with 0002.
- 13:35 block plan (by file). 0002: NOTE H23 (ruling: necessary condition, may stay), NOTE H23-2 (test-engineer 0002:1332-1333 "measured in one run of the netcode proof test"), FIX N8 511-515, O15 line 5 (3-line header), O18 710-711, O18-2 1073-1076, O18-3 1375-1378 (step 0 + step 1), O18-4 1121 (item 7 stand-in load), NOTE O18-5 (test-engineer: PM-0002-28 / BX-P stand-in clauses), M22 539-543, E1 1066, E2 232-234, E3 897-898, E4 1505, E4-2 1508, E5 746. 0005: O15-2 line 5, NOTE O22, L7 76-80 (+LOOK-TEST-INPUTS settled input), L7-2 1454-1459, L7-3 2388-2390, L9 1641-1647, C1 2041-2058, C2 2027, C2-2 1989-1991, C3 93-94, C4 276-277, C4-2 307-308 (+client-entries sub-bullet with the kinds lists), C4-3 spike item 2 (700-701), C5 491-494, C6 1519-1527, C6-2 1537-1539, C7 1878, C7-2 1880-1883, C7-3 1897-1898, A4 2014, A4-2 2465, B3 2416-2417 (drop many-fail/A4), B4 1936-1939, NOTE B4-2 (test-engineer 2263), B5 2015, B5-2 2466, B5-3 2468, NOTE X5. GLOSSARY: FIX H24 787-790, FIX S23 783-786, NOTE M24. Presented kinds: unit, vehicle, weapon, faction; none: tags, attribute sets, effects, abilities, upgrade tracks, drop/pickup values, modes, maps (own refs), rule tables, movement data, moment rules; building parts/props are map entities (CD-11, AP-7). game-designer confirms ability kinds (say in result, not doc).
- 13:40 NOTE H23, NOTE H23-2, FIX H24 (GLOSSARY 787-790), FIX S23 (783-786) written. Next: 0002 blocks N8, O15, O18*, M22, E1-E5.
- 13:55 0002 blocks done: N8, O15, O18, O18-2, O18-3, O18-4, NOTE O18-5, M22, E1, E3, E2 (redone 231-234), E4, E4-2, E5. Next: dry run 0002 in scratch, then 0005 blocks.
- 14:05 0005 blocks O15-2, NOTE O22, L7, L7-2, L7-3, L9 written. Next: C1-C7, A4, B3-B5, X5, M24.
- 14:15 C1, C1-2, C3, C4, C4-2 (kinds lists), C4-3, C5 written. Next: C2, C2-2, C6, C6-2, C7, C7-2, C7-3, A4, A4-2, B3, B4, NOTE B4-2, B5, B5-2, B5-3, NOTE X5, NOTE M24.
- 14:25 C2, C2-2, C6, C6-2, C7, C7-2, C7-3, A4, A4-2, B3, B4, NOTE B4-2, B5, B5-2, B5-3 written. Next: NOTE X5, NOTE M24, then all checks.
- 14:30 NOTE X5, NOTE M24 written. All map rows have blocks. Next: checks (dry runs on all three docs, glossary, tables, line lengths, write guard prefix), final read.
- 14:45 reworked C1-2 (Proposed line -> Answered + Decided) and C7-2 (Proposed -> Decided) per rule 2; NOTE X5 made precise. Next: re-run all checks on a fresh scratch tree.
- 14:55 checks on a fresh scratch tree (stop-engine-architect-a-scratch/tree): dry runs 41/41 ok on 0002, 0005, GLOSSARY
  (dryrun.out); glossary exact 0 in all nine (in-context 604 -> 605: "effects" as a kind name in C4-2's list, as CD-6
  uses it; glossary-applied.out); tables 0 glued, 0 cell mismatches, PM rows 340 (tables-applied.out); new lines 139,
  0 over 120; GLOSSARY base 9e7db28 a byte prefix and lines 1-743 identical; dispositions 521/521
  (dispositions-applied.out). 41 FIX, 7 NOTE (H23, H23-2, O18-5, O22, B4-2, X5, M24). For others: test-engineer
  (H23-2 0002:1332-1333; O18-5 PM-0002-28/BX-P; B4-2 0005:2263), game-designer confirms the ability kinds listed in C4-2.
DONE
