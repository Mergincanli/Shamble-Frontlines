Task: verifier step 3d final check of R3, R5, R6 (prompts/verifier-r356-3d-final.txt), fix-round diff f67245e..d54f3c0
Inputs read: prompt, 3 fix prompts + r3-3d-fixes-2.txt (session extension: AUDIO-REC1), advisor-3d-r3-verdict.md, first-pass report, owner replies. Working tree = d54f3c0 for the 4 files (git diff empty).
C1 R3: item 5 term note word for word at r3:1028, after "Spike must prove"; R3-LOOK-REC3 row (r3:1019-1027) unchanged (diff = one added line). Item 6: ", p99" on all 8 rows r3:1125-1132; CHAR-REC1 r3:1046-1047; CHAR-REC2 r3:1062; CHAR-REC4 r3:1088; AUDIO-REC3 r3:1267: all word for word. Extra: AUDIO-REC1 r3:1243 (0.5 ms (p99), 0.3 ms (p99)) by session's fixes-2 under the same rule, logged; holds (rule already read it p99). Item 3: r3:885, r3:1249-1250 renamed; r3:1398 keeps GP6's words (LEGACY_INVENTORY.md:316 "counts concurrent voices in a 48-bot fight"), term beside: holds. CLOSED.
C1 R5: items 1 (r5:402), 2 (r5:1477), 3 (r5:1561, r5:2087) applied; item 4: r5:58, 1429, 1487 renamed; r5:1426 "§3 (the 48-player scale)" kept as design point: holds (VISION_V2 §3 has no 48; the 48 is the 24v24 goal = design point, not the largest mode). Nit: "the design point" wording would mark it. CLOSED.
C1 R6: items 1 (r6:46), 2 (r6:710-712), 3 (r6:803), 4 (r6:666-667), 5 (r6:772, r6:634-635) applied; item 6: 48/417/508/602 get 24v24 (all R2's 24v24 row, r6:446 3.16 GB), 746 renamed; 212/256 kept (TL15's own measured 48 soldiers, stated beside): holds. Part-7 log at r6:1200-1212 has no type per line (types are in the owner's reply): nit. CLOSED.
C2: R3 p99 marks: not mode-dependent; GPU sums Low 3+16+4+2=25, High 2+6+3+1=12 = totals (planning check). Nit: AUDIO-REC1 "adds less than 0.3 ms (p99)" is a delta at p99 (p99s do not add, GLOSSARY Time statistics): ambiguous. R5 1429/1487: 48/54 from GLOSSARY largest mode; R1-ECS-REC2 mark 1 (r1:1966-1967) names no size, no conflict. R6 COST-REC1 (b) list 54,48,36 = 18v18v18, 24v24, 12v12v12: holds. HOST-REC2, OBS-REC2: hold under both. TEL-REC1 r6:803/806: OPEN (small): under (b) the 54-player run must meet "a record is at most 1 MB uncompressed (A-10)", but A-10 (r6:602) is now scoped to 24v24 and is ~0.96 MB + timeline at 10 events/s; R2's rate is per player (r2:1760 "5 B per player per second (at 48: 10 events/s ...)"), so at 54: 11.25/s x 1200 s x 80 B = 1.08 MB > 1 MB. Mark does not read the same under both readings. Fix: scope the 1 MB mark to the 24v24 run, or scale it with N (proposed).
C3: TL;DR r6:46 "on Workers static assets, static traffic costs nothing (R6-HOST-F5)" = F5 r6:87-88 "free, unlimited requests and no egress or bandwidth charge"; claims nothing beyond F5. No new source in any file; new numbers are 48/54 (GLOSSARY) and the (b) list (arithmetic of GLOSSARY start/goal modes). CLOSED. No fetch needed.
C4: own run glossary_check.py --skip-records: exact 0 in R3, R5, R6; hits on changed lines = 24, identical to session's TSV. Judgments (all right):
r3:885 48 right (beside "the largest mode's N (48 or 54, MODES-3)")
r3:1028 48 right (the term note itself, marking words)
r3:1132 effects right (budget row name, not a count; GLOSSARY Active effect definition cites this phrase as R3's visual effects)
r3:1250 48 right (marking words)
r3:1398 48 right ("48 or 54" beside "the largest mode's N"; GP6's "48-bot" is the inventory's words)
r5:1429 48 right (linked term beside)
r5:1477 "24v24 = 48 and 12v12v12 = 36" right ("the test-mode list" beside; names items 1 and 4, not a replacement list)
r5:1487 48 right (linked term beside)
r6:48 48 right ("24v24" beside; R2's 24v24 row, not the largest mode)
r6:48 the goal right (goal modes' cost scenario)
r6:417 48 right (24v24 beside)
r6:508 48 right (24v24 beside)
r6:602 48 right (24v24 beside)
r6:666 players right (N, not a team size)
r6:666 48 right ("the largest mode", reading (a) named)
r6:710 players right (N per mode, not team size)
r6:710 48 right (goal modes and reading (a) named)
r6:710 the goal right ("the goal modes")
r6:711 48 right (reading (b) named)
r6:712 48 right (24v24 beside)
r6:746 players right (N)
r6:746 48 right ("the largest mode's N", reading (a))
r6:803 players right (N)
r6:803 48 right ("the largest mode", reading (a))
C5: GLOSSARY.md:154-156 lists BUDGET-REC2 rows, CHAR-REC1/2/4, AUDIO-REC1, AUDIO-REC3 at p99: matches r3. Nit: "by the advisor's ruling" while AUDIO-REC1 came from the session's extension (fixes-2). CLOSED.
C6: no R3/R5/R6 key-claim id added to a Rests on list (R6-HOST-REC1 adds R5 ids only); no claim newly checked. Labels unchanged: R3 45 of 77, R5 61 of 115, R6 21 of 42.
Host audit: no fetch, no curl (0 requested, 0 final).
DONE
