# Step 3c, R3: graphics-engineer's report (background agent launched 18:49 UTC, cut off by the spend limit about 18:52, resumed 20:11 UTC, returned about 20:16 UTC) (verbatim)

R3 step 3c is finished, and I raised no OBJECT. Every approved "before" text matched the file exactly once. The glossary check now finds zero "Retire (exact)" hits in R3, and its header links GLOSSARY.md. I did not commit anything.

**Changes by type:**
- **Recommendation (4):**
  - C9.14: R3-BUDGET-REC2's heading is now the stress scene's fight on screen, with 300 live projectiles instead of 200.
  - A glossary replacement: the bare "48" that meant the largest mode, in R3-CHAR-REC1 (heading and mark), R3-CHAR-REC2's mark and R3-CHAR-REC3's mark, now reads "the largest mode's N (48 or 54, MODES-3)". The value changes only under MODES-3 reading (b), and the step-3d verifier should judge it.
- **Number (1):** C9.13, the approved arithmetic: 786 draws at N = 48, and 834 at N = 54, which is over the 800 cap.
- **Source:** 0.
- **Nit (27):** C9.12, C9.15, C9.16 and C10.12; C10.13 (MODES-3) plus 8 cross-topic entries; 5 recommendation pointers; 5 glossary wording changes; the link pass; 2 header changes; part 7.

**Resolutions applied:** rank 9 (C9.12–C9.16) and rank 10 (C10.12, C10.13). No other rank touches R3.

**Glossary terms applied:** the stress scene, big vehicles and very large units, live projectiles, visual effects, player count N, the largest mode, the goal modes, team size, the design point, time statistics, the client's sim share, preview, the isolation flag, the reference server and the test-mode list.

**Shared open questions added:**
- MODES-3, under "Shared open questions (the same words in every topic)".
- R3-XT-OQ1–OQ8 (ranks 4, 16, 25, 26, 33, 35, 37, 39), under "Cross-topic open questions (consistency critic, 2026-10-29)".
- Pointers from R3-WGPU-REC3, R3-WGPU-REC4, R3-CHAR-REC3, R3-CHAR-REC4 and R3-AUDIO-REC3.

**Not changed:** R3-LOOK-REC3's soldier counts stay word for word because it is a look row.

Files are in /home/user/Shamble-Frontlines:
- docs/research/r3-rendering.md
- .checkpoints/v2-research/graphics-engineer-r3-consistency.md (my notes, ending in DONE)

## Addendum: the step-3d fix round (reply to `prompts/r3-3d-fixes.txt`, about 20:45 UTC, verbatim)

All three step-3d items are applied to /home/user/Shamble-Frontlines/docs/research/r3-rendering.md, with no OBJECT and nothing committed. The glossary check still finds 0 retired phrases in R3.

1. **R3-LOOK-REC3:** your term note is added word for word after its "Spike must prove"; the row itself is unchanged. Nit.
2. **p99 marks** (5 recommendation changes):
   - R3-BUDGET-REC2: ", p99" after all 8 row names (CPU, animation, sim cost and the five GPU rows).
   - R3-CHAR-REC1: "2.0 ms (p99)" and "1.0 ms (p99)".
   - R3-CHAR-REC2: "1.0 ms (p99)".
   - R3-CHAR-REC4: "2.0 ms (p99) GPU time".
   - R3-AUDIO-REC3: "0.2 ms (p99) per frame".
3. **Compound 48s** (3 nits):
   - r3:885 (row V-A) and r3:1248 (R3-AUDIO-REC2's reason) meant the largest fight, so I renamed them to a fight at the largest mode's N (48 or 54, MODES-3).
   - r3:1396 (R3-AUDIO-OQ2) is the inventory's own wording (LEGACY_INVENTORY.md:316), so "48-bot fight" stays and the term is added beside it.

Part 7 has a new "Step 3d fixes" list with one line per change.

R3-AUDIO-REC1's spike mark ("under 0.5 ms", "less than 0.3 ms") also names no statistic. It is not on your list, so I left it. Your call whether it gets "(p99)" too.

Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/graphics-engineer-r3-consistency.md

## Addendum: R3-AUDIO-REC1 (reply to `prompts/r3-3d-fixes-2.txt`, about 20:46 UTC, verbatim)

Done (two recommendation changes, part-7 line added, not committed). R3-AUDIO-REC1's mark now reads: "main-thread audio work stays under 0.5 ms (p99) per frame on the minimum machine; HRTF for 4 priority voices adds less than 0.3 ms (p99)."
