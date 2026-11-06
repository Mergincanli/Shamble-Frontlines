Task: R-PR for the netcode proof test: place the advisor's readings of 2026-11-06 (exact words) and Mustafa's two answers into the merged drafts, in worktree .gate/npt-readings (branch claude/npt-readings, main af8393e). Never commit or push.

## Plan (blocks, in order)
1. Read source: advisor-verdict.md "Exact words for R-PR" (the words to place).
2. Survey anchors: grep the four drafts + README register for every anchor; record line numbers and the 2026-11-03 inline style.
3. ADR 0006 edits: OQ-0006-16 (R-1/R-18), OQ-0006-19 (Mustafa), PM-0006-11/-14/-26 (R-12), bite rule (R-12 general), sessions rule (R-26), ROUTE if natural.
4. ADR 0001 edits: part 13 (R-5), PM-0001-3 (R-6/R-9), -28 (R-7), OQ-0001-9 cover (R-10), -18/-36/-55 (R-12), bite rule (R-12), -59 (R-16), -23/-24 (R-17), L2 WS proxy (R-25), checklist (R-28).
5. ADR 0004 edits: PM-0004-67 (R-12, R-15).
6. SPIKE_PLAN edits: runner list (R-13), item 1 Place (R-14), FIXED-IP 64-66 and 173, ROUTE if natural.
7. README register: OQ-0006-16, OQ-0006-19, OQ-0001-9 cover half.
8. Verify: git diff --stat, table rows single-line with same pipe counts, line lengths.
9. Answer: placements with file:line, Problems.

Edit method: no Edit tool in this session; exact-string replacement via small python scripts in this folder (rpr-*.py), each asserting the old string occurs exactly once.

## Notes
- block 1 done: source read (advisor-verdict.md, 15 bullets incl. FIXED-IP). Next: block 2 survey.
- ADDED by coordinator (mid-task): MAC-NONE placement in SPIKE_PLAN Machines paragraph (~44-57), after MACHINES-TIMING words: Mustafa, 2026-11-06: "No Mac available for now (the loan Macs are out of stock). Nothing in this test waits on one: the timed Safari rows wait until a Mac is available, at the latest for the look proof test." then: So if GitHub's macOS runner fails condition (i), (ii) or (iii), every Safari row of the netcode proof test waits for that Mac too, instead of a Mac being borrowed before the test (the advisor's reading of it, as the plan applies it, 2026-11-06). Replaces nothing. -> becomes block 6b.
- survey findings so far (block 2 in progress):
  - style: readings inline as prose, no quote marks, tag before final period. Answered OQ style: first sub-bullet "  - Answered by X (date): ...".
  - ADR0006 OQ-0006-16 at 1038-1050 (no Answered line yet); OQ-0006-19 at 1087-1098.
  - ADR0001 part 13 at 567-581; key "Profiles" bullet 1436-1450 (ends "...names no rate)."); R-25 anchor = 1444-1445 "(and a delaying TCP proxy for WebSocket) ... ADR 0002's simulator)."
  - ADR0001 rows: PM-0001-3 @1513, -18 @1531, -23 @1536, -24 @1537, -59 @1540, -28 @1543, -36 @1551, -55 @1577. Bites go in "runs and noise" cell as ". Bites: ..." (style of -23/-24).
  - ADR0001 general Bites para 1499-1504; ADR0006 Bites para 1319-1321.
  - OQ-0001-9 at 1159-1164 (one-paragraph style with " · *Options:*").
  - DECISION: PM-0001-3: "100,000" -> "1,000,000 packets per profile and direction on both layers, so PM-0001-3 and PM-0002-49 read one count (tag)", probe+delay sentences appended at end of same (pass-mark) cell. Method cell keeps older ambiguous "half of the profile's RTT minus" phrase -> Problems note.
  - DECISION: R-12 provenance sentence "Test-engineer's digest F proposals, each written into its own row." is a placement instruction; not written as prose -> Problems note. General paras get "Memory and queue marks bite over a full S30 (tag)."
  - NOTE: ADR0001:427 says "OQ-0006-16 ... until it is answered" -> stale after R-1; outside scope -> Problems note.
  - R-5 placed twice (part 13 + key), as task names both.
- survey continued:
  - ADR0006 rows: PM-0006-11 @1349 (runs cell "every session"), -14 @1352 (runs cell ends "(it feeds PM-0006-18)"), -26 @1364 (runs cell "...0 dropped in each"). ADR0006 sessions rule end @1316 "...r6 line 669)." ADR0006 bite rule ends @1321. ADR0006 has no S30 term (ADR0001's) -> Problems note.
  - ADR0004 PM-0004-67 @1193: runs cell "3 runs of 36,000 ticks; each meets the mark. Bites: a loop that counts ticks from its timer period instead of elapsed time". R-15 after "each meets the mark."; R-12 sentence after existing bite.
  - ADR0001 checklist: R-28 after "(PM-0001-3)." @1641; earlier clause "The profile and the cap change from the overlay" stays -> conflicts in its cap half -> Problems note.
  - SPIKE_PLAN: runner reading ends @58 "(the advisor's reading, 2026-11-03)." -> R-13 then MAC-NONE there (conditions (i)-(iii) defined just before). FIXED-IP + ROUTE at end of Machines para @66. FIXED-IP 2nd copy in Depends-on list after "(the advisor's reading, 2026-11-03);" @176 (lowercase "with" to fit the semicolon list -> report). R-14 after Place bullet end @182.
  - R-17 full words into both -23 and -24 method cells. R-16 at end of -59 method cell. R-7 at end of -28 hardware cell.
- block 2 done: survey complete. Next: block 3-7 edits via script rpr-edit.py (all edits in one idempotent script run against clean files).
- blocks 3-7 done: rpr-edit.py run OK (rebuilds from af8393e; all asserts passed); diff stat 5 files, +85 -27. Next: block 8 verify diff.
- block 8 done: rpr-verify.py: every sentence of every reading found verbatim (whitespace-collapsed), except R-12's "PM-00xx-nn:" labels (written as "Bites:" in each row) and its digest F sentence (not placed); table line counts and pipe counts unchanged; 0 added prose lines over 120 chars. Tags 2026-11-06: 0001 x16, 0004 x2, 0006 x8, SPIKE_PLAN x9, README x5.
- Placements (final lines):
  R-1/R-18: 0006:1040-1048 (new first sub-bullet of OQ-0006-16, "Answered by infra-engineer and netcode-architect, confirmed by the Advisor (2026-11-06):"); README:154 (question cell).
  OQ-0006-19: 0006:1097 (new first sub-bullet, + "(DENSITY-FIGURE)"); README:157 (id cell "OQ-0006-19 (DENSITY-FIGURE)", answer in question cell).
  R-5: 0001:579-584 (part 13, after "The simulator that runs the profiles is ADR 0002's."); 0001:1470-1474 (key, Profiles, after "...names no rate).").
  R-6/R-9: 0001:1538 PM-0001-3 pass-mark cell ("over 1,000,000 packets per profile and direction on both layers, so ... read one count (tag),"; probe+delay sentences at the cell's end).
  R-7: 0001:1568 PM-0001-28 hardware cell end.
  R-10: 0001:1166-1173 OQ-0001-9 after its description, before "· *Why it matters:*" (lead-in: "The cover half is answered by the Advisor's rule (2026-11-06), which Mustafa may overrule at the close."); README:79 question cell.
  R-12: 0001:1556 (-18), 1576 (-36), 1602 (-55); 0006:1363 (-11), 1366 (-14), 1378 (-26); 0004:1193 (-67, after its existing bite); general: 0001:1528-1529, 0006:1334-1335.
  R-13: SPIKE_PLAN:57-61 after the 2026-11-03 runner reading. MAC-NONE: SPIKE_PLAN:61-65 right after R-13, before "Until they are named".
  R-14: SPIKE_PLAN:197-200 end of item 1 Place. R-15: 0004:1193 runs cell after "each meets the mark.". R-16: 0001:1565 method cell end. R-17: 0001:1561 and 1562 method cells (full words in both).
  R-25: 0001:1458-1465 key Profiles after "ADR 0002's simulator).". R-26: 0006:1326-1329 after times-of-day sentence (bracket B bullet). R-28: 0001:1667-1669 after "(PM-0001-3).".
  FIXED-IP: SPIKE_PLAN:74-76 (end of Machines para) and SPIKE_PLAN:187-188 (Depends-on list, lowercase "with"). ROUTE: SPIKE_PLAN:76-78 (last sentence of Machines para).
- Problems (for the answer): P1 digest F sentence not placed; P2 PM-0001-3 method cell keeps ambiguous "half of the profile's RTT minus..." and runs cell "every L2 session"; P3 R-28 leaves "The profile and the cap change from the overlay" (0001:1665); P4 OQ-0006-16 "Answered" though 443 half open + 0001:427 "until it is answered" stale; P5 S30 undefined in ADR 0006; P6 FIXED-IP lowercase in list; P7 MAC-NONE supersedes SPIKE_PLAN:171-172 and 0001:1410 summary, untouched; P8 PM-0006-14 row/0006:1211 still say "interim", OQ-0006-19 "What decides it" unchanged; P9 DENSITY-FIGURE code added (droppable); P10 R-5 placed twice, R-17 in both rows; P11 register rows quote full words (long).
DONE
