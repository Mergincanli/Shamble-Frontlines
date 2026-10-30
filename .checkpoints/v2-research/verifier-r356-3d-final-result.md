# Step 3d, R3+R5+R6: the final check's report (verbatim)

Workflow run wf_ca6f48ca-efd (prompt `prompts/verifier-r356-3d-final.txt`), launched 2026-10-30 20:47 UTC, returned about 20:52 UTC. Notes: `verifier-r356-3d-final.md`.

VERDICT: PASS WITH NOTES. One small item is still open (R6-TEL-REC1 under MODES-3 reading (b)). Everything else that remains is nits.

**1. Fix items: closed.**
- R3: the term note is in word for word at r3:1028. R3-LOOK-REC3's row (r3:1019–1027) is unchanged. All of the advisor's p99 marks are in word for word: r3:1125–1132, 1046–1047, 1062, 1088 and 1267.
- R3-AUDIO-REC1 (r3:1243) also got p99. This came from the session's follow-up message `r3-3d-fixes-2.txt`, not from the advisor's list. It is logged and it holds, because the glossary rule already read that mark as p99.
- The 48s inside compounds: r3:885 and r3:1250 were renamed. r3:1398 keeps the inventory's own words (LEGACY_INVENTORY.md:316), with the term beside them.
- R5: all items applied. Keeping r5:1426 ("the 48-player scale") holds, because it means the 24v24 design point, not the largest mode.
- R6: all items applied. Keeping r6:212 and r6:256 holds, because they repeat the 48 soldiers that the legacy measurement (TL15) itself used.

**2. Recommendation changes: one open.**
- At r6:803/806, the largest-mode run must give "a record is at most 1 MB uncompressed (A-10)". But A-10 (r6:602) is now scoped to 24v24. R2's event rate is per player (r2:1760). At 54 players that gives 11.25 events/s × 1,200 s × 80 B = 1.08 MB, which is over 1 MB. So the mark does not read the same under reading (b).
- Fix: scope the 1 MB mark to the 24v24 run, or scale it with the player count.
- All other changes hold under both readings.

**3. No new claim, number or source: closed.** The TL;DR at r6:46 claims only what R6-HOST-F5 says (r6:87–88).

**4. Glossary hits: 24 judged, 24 right, 0 wrong.** My own run of the script produced the same 24 hits as the session's list. "Retire (exact)" is 0 in R3, R5 and R6.

**5. GLOSSARY.md:154–156: closed.** The line matches R3's marks.

**6. Labels: unchanged.**
- R3: "spot-checked (45 of 77 key claims)"
- R5: "spot-checked (61 of 115 key claims)"
- R6: "spot-checked (21 of 42 key claims)"

**Nits:**
- r3:1243: "adds less than 0.3 ms (p99)" is a difference stated at p99, which is unclear because p99s do not add.
- GLOSSARY.md:156 credits all of R3's p99 marks to the advisor's ruling, but R3-AUDIO-REC1's came from the session's follow-up.
- R6's part-7 log (r6:1200–1212) gives no change type per line; the types are only in the owner's reply.
- r5:1426 would read better as "the design point".

**Claims checked at a source:** none. No check needed a fetch.

**Host audit:** WebFetch 0 and curl 0, both requested and final.

Notes (they end with DONE): /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r356-3d-final.md
Script output: /tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/final/
