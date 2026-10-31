# verifier-3d-critic: completeness critic of step 3d (plan-amendments-2.md), all six topics, read-only (task: prompts/verifier-3d-critic.txt)

Started 2026-10-31. Head under test: 4cf8633 (diff base 7c6e0d7).

## Check 1 (approved ids and shared questions at 4cf8633): DONE, result none
- My own script (scratchpad/critic/chk_after.py, chk_oq.py) on 4cf8633: 143 change ids parsed (same set as apply-3d-all.tsv); 6 (C17.b1-b3, C23.b1-b3) absent, their befores present (r1:2958, r6:683-688): right.
- 13 "NOT found verbatim" all explained: C1.3 (r14 fix item 1, r1:2050), C1.9/C11.9/C11.10/C17.3/C17.5/C9.4/C9.8 (glossary renames, verifier-r2-3d.md:12, r14 and r356 reports), C1.10 (C11.8 + R2 fix item 2, r2:2411-2414), C1.10b ("at p99" r2:2415), C10.17 (R5 fix item 2), C23.2 (OBJECT-1 nit), C11.12 (session-script parse artefact: inline "core"->"vCPU" greedy match; applied at r2:1725 and r2:3436).
- No replaced "before" text survives anywhere at head.
- 72 XT-OQ texts and 18 shared-question copies: all word for word at head (links stripped); one subsection each per file; R1 18, R2 15, R3 8, R4 8, R5 14, R6 9 headings. Rank 41: r5:1629 + part-7 r5:2256.
- Verdict-revised texts (C1.5, C1.9, C1.10, C1.10b, C1.21, C19.3, C19.8, C21.1, C21.2, C21.4, C21.8, C29.2, rank 9) are folded in resolutions.md.
## Check 2 (condition-4 recommendation changes no verifier checked): DONE, result none
- R1 items (C1.1, C1.5, C5.1-4, C6.1-2, C9.1, C9.3, C9.7, C29.1-3) and R4 (C1.15-18, C9.18-20): verifier-r14-3d-result.md check 2, K1-K11. R2 (C1.8-C1.11 incl. C1.10b, C9.9; INT-REC1/2/4 p99): verifier-r2-3d-result.md check 2 + table recompute; C1.8 also K3 (r14). C1.8, C1.11, C9.9 only under R2's blanket "each holds" (no line of its own); I re-read them at head: C1.11 5 % = 10 % x 50 % (0.83/1.67 ms) ok; C9.9 1 MB vs r2:2139-2140 "about 0.1 MB at 60 Hz for 60 movers", 54+15 = 69 movers still far under: holds; C1.8 r2:2292-2296 holds.
- R3 C9.14, R5 C19.1/C21.1/C22.1/C22.3, R6 C1.21/C19.5/C19.6/C21.4/C21.8/C22.5: verifier-r356-3d.md "20:55 Check 2" lines. R3 p99 marks: r356 final check item 1.
- R5/R6 time marks with no statistic (r5:1263, 1281, 1320, 1429, 1473, 1545, 1691, 1732; r6:669, 838) are single-run limits (GLOSSARY "Time statistics" one-off clause, 96a4c29): the rule changes none of them.
## Check 5 (labels): DONE, result none
- Headers at 4cf8633: r1:10 and r2:8 "verified (every recommendation claim)"; r3:9 "spot-checked (45 of 77 key claims)"; r4:7 "(50 of 52)"; r5:12 "(61 of 115)"; r6:8 "(21 of 42)": all as the r14, r2, r356 reports and their final checks say.
- Note (not a mismatch): the R2 step-3d verifier confirmed R6-HOST-F23's "c6a.large (1 core, 2 vCPU, 4 GB)" at aws.amazon.com (verifier-r2-3d-result.md claims table); r6:1150-1151 still lists F23 among the 21 "not checked at their source". Only part of F23 (not its "bandwidth free" half) was confirmed, so 21 of 42 can stand; no report takes it up.
## Check 4 (first real uses not linked): DONE (scripts scratchpad/critic/first.py, allu.py)
Clear (searchable phrase; first real use unlinked):
- R1 stress scene: r1:1915 (ECS-REC1 Rests on, added by r14 fix 3aad9d8) unlinked; first link r1:1923.
- R3 largest mode's N: r3:885 (options V-A, renamed by fix 1b69364) unlinked; links r3:1028 (N), r3:1035.
- R5 stress scene: only use r5:1920 (MODES-3) unlinked; R5 has 0 links to #the-stress-scene (R6's copy r6:948 links it).
- R6 Player count (N): r6:807 (TEL-REC1 "1 MB x N / 48", added by final fix 1bfd235) unlinked; R6 has 0 links to #player-count-n.
Pattern-marked (script cannot search "AvB" or "... players/soldiers in a pass mark or estimate"; nobody read for them):
- Team size (AvB): R5 0 links, first use r5:1477 (TEST-REC5 "24v24 = 48 and 12v12v12 = 36"); R2 first r2:456 (GC2 soldier slots) / r2:867 (INT-F20 "24v24 gives 24 x 24"), link only r2:1384; R4 r4:529 (wave-1 "576 at 24v24"), link r4:592; R6 r6:417 (G3 "per 24v24 (48-player) match-hour"), link r6:441.
- Player count: R2 r2:858 ("With 48 players spread evenly", estimate), link r2:1800; R3 r3:515 / r3:592 (estimate; R1's spike scene "48 soldiers"), link r3:1028.
Borderline (glossary's own Used today counts them; arguably another sense): r1:295 (GC2 slots; link r1:316); "mean" of non-time legacy figures r4:403, r5:531, r6:383 (links r4:700, r5:1400, r6:416); r5:1857 "the start budget's line" (link r5:1919); R4 start/goal only in MODES-3 r4:1153-1154, 0 links in R4.
Acceptable: r2:1725 section link #1-tick-budgets; r4:531 link to the shares entry; flagged false positives all other senses (r1:209, 293, 518; r2:56, 404, 451, 1528; r3:202, 676; r4:165, 347, 727; r5:1170; r6:565).
## Check 3 (diff changes no report covers): DONE (scratchpad/critic/wdiff.py, resid.py, gfd.py, inapp.py)
- Word diff 7c6e0d7..4cf8633 of all six topics, residue after removing approved/XT/shared texts read by hand: every residue chunk is an approved change, a rename/link, a pointer, a header/total line, a part-7 line or a fix-round item a final check closed. Totals checked: R2 91 = 72 + 19, R1 99, R4 32.
- GLOSSARY.md field diff outside "Used today": only OBJECT-1 (advisor's exact words), the 96a4c29 nits (R14 final), header/status lines (accurate). No new number.
- Not covered by any report (all after the last check of their file):
  - 1bfd235 r6:806-807 R6-TEL-REC1 record mark (recommendation change): "a 24v24 record is at most 1 MB (A-10), and a record of the largest mode at most 1 MB x N / 48 (1.125 MB at N = 54)". My check: holds (A-10 r6:602 12,000 x 80 B = 0.96 MB at 48; 54/48 = 1.125; per-player event rate r2:1760). Its N is unlinked (check 4).
  - 902c249 r3:1243 AUDIO-REC1 "raises that p99 by less than 0.3 ms" + part-7 line: nit, holds.
  - 8260592 r5:1426 "§3 (the design point (48 players))" + part-7 line: nit, holds (GLOSSARY:351 design point = VISION_V2 §3's 24v24 goal; no quote misattributed).
  - 7bb3d2e UNREAD_SOURCES.md: the 2 rows R14 final flagged now "no tie found" (right); 3 ties added (sohu, juejin -> R2-SYNC-REC1; gdcvault 1024041 -> + R2-SYNC-REC3) as R14 final suggested; 23 note citations moved "at a3962ae" -> "at d3958f3": 7 sampled, identical lines at both SHAs.
- Cross-topic effect nobody checked: R2 fix 2382ea2 turned R2-ROOM-REC2 marks 1-3 into the largest mode's N (r2:2774-2778); R6 A-2 r6:516 still "8 rooms of 48 players per 4-vCPU VM (R2-ROOM-REC2's proposal)", A-3 r6:522 "4 rooms of 48"; r356 judged r6:516 right as "unchanged by resolution 10" before that fix. Nit (A-2 is the goal's 24v24 room).
## Check 6 (report findings > nit not listed for an owner): DONE
- R356 skeptic's script gap (compound 48s) went to R3, R5, R6 owners only. R2's seven compound places at head were never judged by anyone: r2:1583, 1785, 2304, 2332, 2990, 3183, 3493. I judge all 7 right (legacy figure, design-point estimate, or XU-OQ8's pre-2026-10-30 record). R1, R4: none.
- verifier-r2-3d-final-result.md "Note, not a fix item": under MODES-3 (b) no R2 bandwidth mark covers 54 players (formula ~164-171 kbps vs the 160 kbps mean at the design point, r2:2339). No file carries it (R2-BUD-OQ6 r2:2985-2987 only says 54 is inside the 64-player row); same shape as R3-BUDGET-REC2's hull-outline Low cap (about 1,100 at 48; 1,130 at 54, r3:1144-1148; r356 nit). Borderline: say it in the report or BUD-OQ6, or leave to the synthesis.
- Everything else in the 3 reports, 3 skeptics and 3 final checks is in a fix message (r2-3d-fixes*, r14-3d-fixes, r3-3d-*, r5-3d-*, r6-3d-*, engine-architect-glossary-*) or is a nit.
DONE
