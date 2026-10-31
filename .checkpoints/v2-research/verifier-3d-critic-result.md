# Step 3d: the completeness critic's report (verbatim)

Workflow run wf_8b0673a0-f49 (prompt `prompts/verifier-3d-critic.txt`, xhigh), launched 2026-10-31 10:32 UTC, returned about 10:52 UTC. Notes: `verifier-3d-critic.md`.

**Step 3d completeness critic (all six topics, head 4cf8633, base 7c6e0d7)**

Nothing was fetched and nothing was committed. My notes end with DONE: `/home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-3d-critic.md`. My scripts are in `/tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/critic/`.

**Check 1: approved ids and shared open questions missing from their file**
None.
- I parsed all 143 change ids myself. The six that wait for BASELINE-WHEN (C17.b1–b3, C23.b1–b3) are absent, and the texts they would replace are still there (r1:2958, r6:683-688).
- The 13 ids the session's script marks "NOT found verbatim" all have a logged cause:
  - glossary renames: C1.9, C9.4, C9.8, C11.9, C11.10, C17.3, C17.5;
  - fix rounds: C1.3, C1.10, C10.17, plus C1.10b's added "at p99";
  - the approved OBJECT-1 nit: C23.2;
  - C11.12 is an artefact of the session's script (its pattern grabs the wrong quote); the change is applied at r2:1725 and r2:3436.
- No "before" text that was meant to be replaced survives anywhere.
- All 72 cross-topic questions (R<n>-XT-OQ<k>) and all 18 copies of the shared decision questions match their approved texts word for word. The rank-41 fix is at r5:1629.

**Check 2: recommendation changes on the verdict's condition-4 list that no verifier checked**
None.
- R1 and R4 items are covered by the R1+R4 report's check 2 and its claims K1–K11.
- R2 items are covered by the R2 report's check 2 and its recomputed table. C1.8, C1.11 and C9.9 only fall under its blanket "each holds", so I re-read them; all three hold. For C9.9, the 1 MB history cap is far above r2:2139-2140's "about 0.1 MB … for 60 movers", even at 54 + 15 = 69 movers.
- R3, R5 and R6 items are covered in the R3+R5+R6 verifier's notes ("20:55 Check 2") and its final check.

**Check 3: changes in the diffs that no verifier report covers**
All of these came after the last check of their file.
1. **r6:806-807 (commit 1bfd235), a recommendation change:** R6-TEL-REC1 now reads "a 24v24 record is at most 1 MB (A-10), and a record of the largest mode at most 1 MB × N / 48". Nobody verified it. My check says it holds:
   - A-10 (r6:602) gives 12,000 events × 80 B ≈ 0.96 MB at 48 players;
   - 54/48 = 1.125;
   - the event rate is per player (r2:1760).
2. **r3:1243 (902c249), a nit:** R3-AUDIO-REC1 now says the HRTF voices "raise that p99 by less than 0.3 ms". It holds.
3. **r5:1426 (8260592), a nit:** now reads "§3 (the design point (48 players))". It holds: GLOSSARY:351 defines the design point as VISION_V2 §3's 24v24 goal, and nothing is misquoted.
4. **UNREAD_SOURCES.md (7bb3d2e):**
   - The 2 rows the R1+R4 final check flagged now say "no tie found", which is right.
   - 3 ties were added as that check suggested.
   - 23 notes citations moved from a3962ae to d3958f3. I sampled 7: the cited lines are identical at both commits.
5. **Cross-topic effect nobody checked (a nit):** R2's fix round 2382ea2 changed R2-ROOM-REC2's marks to the largest mode's N (r2:2774-2778).
   - R6's A-2 (r6:516) still says "8 rooms of 48 players per 4-vCPU VM (R2-ROOM-REC2's proposal)", and A-3 (r6:522) says "4 rooms of 48".
   - The R3+R5+R6 verifier judged r6:516 right as "unchanged by resolution 10", before that fix.
   - The 48 itself can stand, because A-2 is the goal's 24v24 room; only the attribution to R2-ROOM-REC2 is now loose.

Everything else in the residue of my word diff is accounted for: approved texts, renames and links, pointer lines, headers and totals (R1 99, R2 91, R4 32), part-7 lines, and items a final check closed. GLOSSARY.md outside "Used today" changed only by OBJECT-1 (the advisor's exact words), the 96a4c29 nits and the header lines.

**Check 4: first real uses not linked to GLOSSARY.md**
Clear cases (a searchable phrase, unlinked at its first real use):
1. **R1, the stress scene:** r1:1915 (R1-ECS-REC1's "Rests on" line, added by fix round 3aad9d8) is unlinked; the first link is at r1:1923.
2. **R3, the largest mode's N:** r3:885 (options row V-A, from fix round 1b69364) is unlinked; the links come at r3:1028 and r3:1035.
3. **R5, the stress scene:** its only use, r5:1920 inside MODES-3, is unlinked. R5 has no link to #the-stress-scene; R6's copy of the same question links it at r6:948.
4. **R6, Player count (N):** r6:807 (from 1bfd235) is unlinked, and R6 has no link to #player-count-n anywhere.

Pattern-marked uses. The search script cannot find "AvB" mode names or "… players / soldiers" in a pass mark or estimate, and no checker read for them:
5. **R5, team size:** R5 has no link to the entry at all. Its first use is r5:1477 ("24v24 = 48 and 12v12v12 = 36").
6. **R2, team size:** first used at r2:456 (legacy "12 fixed soldier slots") and r2:867 ("24v24 gives 24 × 24"); linked only at r2:1384.
7. **R4, team size:** r4:529 ("576 at 24v24") comes before the link at r4:592.
8. **R6, team size:** r6:417 ("per 24v24 (48-player) match-hour") comes before the link at r6:441.
9. **R2, player count:** r2:858 ("With 48 players spread evenly") comes before the first link at r2:1800.
10. **R3, player count:** r3:515 and r3:592 come before the first link at r3:1028.

Borderline cases. The glossary's own "Used today" lists count these, but they arguably use the word in another sense:
- r1:295, GC2's legacy soldier slots (linked at r1:316);
- "mean" of figures that are not times: r4:403, r5:531, r6:383;
- r5:1857, "the start budget's line";
- R4's start and goal modes appear only inside MODES-3 (r4:1153-1154), with no link in R4.

**Check 5: header labels that differ from the verifier's label**
None. The headers at r1:10, r2:8, r3:9, r4:7, r5:12 and r6:8 carry the labels the reports give.
- Note: the R2 verifier confirmed part of R6-HOST-F23 ("c6a.large (1 core, 2 vCPU, 4 GB)") at its source. r6:1150-1151 still lists F23 among R6's unchecked claims. Only the half about cores was confirmed, so "21 of 42" can stand.

**Check 6: findings bigger than a nit that are not yet listed for an owner**
1. **R2's compound 48s were never judged.** The R3+R5+R6 skeptic found that the script missed numbers like "48-player". That finding was sent to the R3, R5 and R6 owners only. R2's 7 such places were never judged by a verifier or an owner: r2:1583, 1785, 2304, 2332, 2990, 3183, 3493. I judged all 7 right (a legacy figure, a design-point estimate, or R2-XU-OQ8's record of the old figures), so nothing needs fixing. R1 and R4 have none.
2. **Borderline: R2's bandwidth at 54 players.** The R2 final check notes ("not a fix item") that under MODES-3 (b) no R2 bandwidth mark covers 54 players: R2's formula gives about 164–171 kbps against the 160 kbps mean, which is set at the design point (r2:2339). No file says this; R2-BUD-OQ6 (r2:2985-2987) only says 54 is inside the estimate's 64-player row. R3's hull-outline Low cap has the same shape (about 1,100 at 48, about 1,130 at 54; r3:1144-1148, logged as a nit). The session should state it in the report or in BUD-OQ6, or leave it to the synthesis.
- Every other finding in the three reports, the three skeptics and the three final checks is either in a fix message or a nit.

**Counts**
- Check 1: 0
- Check 2: 0
- Check 3: 5 (1 recommendation change unchecked but it holds; 2 nits that hold; 1 UNREAD batch that holds; 1 stale cross-reference, a nit)
- Check 4: 10 (4 clear, 6 pattern-marked), plus 6 borderline
- Check 5: 0 (1 note)
- Check 6: 2 (1 coverage gap, closed by my reading; 1 borderline)
