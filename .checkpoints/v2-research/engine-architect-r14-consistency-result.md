# Step 3c, R1 and R4: engine-architect's report (background agent launched 18:39 UTC, cut off by the spend limit about 18:52, resumed 20:11 UTC, returned about 20:23 UTC) (verbatim)

Step 3c is finished for R1 and R4, with no OBJECT. Nothing was committed or fetched, and GLOSSARY.md was not touched.

**Changes by type:** recommendation 25 (R1 16, R4 9); number 0; source 0; nits for everything else.

**Resolutions applied** (each "before" quote matched exactly once):
- R1: C1.1–C1.6; C5.1–C5.7; C6.1–C6.4; C9.1–C9.8; C10.1–C10.3; C11.1–C11.7; C17.1–C17.2; C29.1–C29.3. Resolution 22 needed only the glossary link on R1-BND-REC5's G3.
- R4: C1.15–C1.18; C9.17–C9.22; C10.14–C10.16; C11.13.
- Not applied: C17.b1, which waits for BASELINE-WHEN.

**Glossary:**
- Header and first-use links added in both files. Exact retired phrases: R1 13 → 0, R4 5 → 0.
- Renames: sim step time, the client's sim share, the server tick budget, the tick period, the prediction scope, the security baseline.
- 4 recommendation changes come from definitions: R1-ECS-REC1 marks 2 and 3 and R4-OBST-REC2 now say "at p99"; R4-PHYS-REC1 mark 5 is now the largest mode's N.

**Questions added:**
- R1: ABL-PREDICT, SEC-SPREAD, MODES-3, BASELINE-WHEN, and R1-XT-OQ1–OQ18, with pointers in the 17 recommendations they name.
- R4: MODES-3 and R4-XT-OQ1–OQ8, with pointers in 8 recommendations.
- The subsection names follow your correction.

**Left for step 3d:** C9.4 and C9.8 differ from their approved text only by logged renames. One-off time marks and 7 first hits that mean something else were left unlinked; the notes list them.

Files:
- /home/user/Shamble-Frontlines/docs/research/r1-simulation-core.md
- /home/user/Shamble-Frontlines/docs/research/r4-physics-navigation.md
- /home/user/Shamble-Frontlines/.checkpoints/v2-research/engine-architect-r14-consistency.md

## Addendum: the step-3d fix round (reply to `prompts/r14-3d-fixes.txt`, cut off by the spend limit about 21:04 UTC on 2026-10-30, resumed 2026-10-31 10:14 UTC, returned about 10:16 UTC, verbatim)

All three items are fixed in R1 and R4, and each file's part 7 has its "Step 3d fixes (2026-10-30):" lines. Nothing is committed. The glossary check still finds 0 exact hits in both files.

1. **r1:2049** now reads "The other 25 % of the tick period, inside the server tick budget (… 50 % of the tick period)". Logged as a nit: 4.2 ms at 60 Hz was the intended value.
2. **The five bare 48s** now name the largest mode's N (48, or 54 under MODES-3 (b)), each logged as a recommendation: R1-BND-REC2 mark 4, R1-BND-OQ6, R4-PHYS-REC4, and both R4-NAV-REC3 hits. In R4-NAV-REC3 the avoidance cost now reads ≤ 0.2 ms p99 at N = 48, scaled ×1.125 at N = 54 by GLOSSARY's tick-budget check.
3. **Nits:**
   - Pointer lines added: R1-BND-REC2 to R1-XT-OQ16 and R1-XT-OQ17, R1-DET-REC6 to R1-XT-OQ2, R4-NAV-REC4 to R4-XT-OQ5.
   - R4-NAV-REC2's "Rests on" now names the stress scene.
   - R1-ECS-REC1's "Rests on" now names R2's worked estimate.
   - "The hash-map ban" sits beside the old wording at r1:1633, r1:2404 and r1:3136.

Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/engine-architect-r14-consistency.md
