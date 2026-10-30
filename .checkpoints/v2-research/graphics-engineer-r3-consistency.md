Task: step 3c owner apply (R3) — apply approved consistency resolutions, glossary terms, shared/cross-topic OQs to docs/research/r3-rendering.md
Restarted 2026-10-30 ~20:10 UTC after the spend limit: r3-rendering.md unchanged (git diff empty), nothing applied yet.

## Plan (R3 items, as approved)
- Res 9: C9.12 (CHAR-REC4 spike), C9.13 (BUDGET-REC2 arithmetic), C9.14 (BUDGET-REC2 heading; 200 -> 300 projectiles: recommendation), C9.15 (MEASURE-REC2 heading), C9.16 (MEASURE-REC2 reason).
- Res 10: C10.12 (BUDGET-OQ4 end), C10.13 insert MODES-3.
- Res 1, 5, 6, 11, 17, 19, 21, 22, 23, 29: no R3 change.
- Shared subsection before "### Pointers to other topics"; XT subsection right after it: R3-XT-OQ1..OQ8 = ranks 4, 16, 25, 26, 33, 35, 37, 39.
- Glossary: exact hit only line 1110 (removed by C9.14). Links at first uses; judge in-context hits.
- Part 7 = "## Verifier spot-check"; add "### Consistency resolutions (2026-10-30)" after "### Second pass (2026-10-29)".

## Log (type)
- C9.12 applied, R3-CHAR-REC4 spike: the stress scene's four very large units with its soldiers. nit (not in the advisor's recommendation list)
- C9.13 applied, R3-BUDGET-REC2 arithmetic: N = 48 qualifier, N = 54 sum 834 over 800 (R3-BUDGET-OQ4). number (approved arithmetic, no new source)
- C9.14 applied, R3-BUDGET-REC2 heading: stress scene's fight on screen, N soldiers, 4 very large units, 300 live projectiles (was 200). recommendation
- C9.15 applied, R3-MEASURE-REC2 heading: the stress scene. nit
- C9.16 applied, R3-MEASURE-REC2 reason: the stress scene gives the load. nit
- C10.12 applied, R3-BUDGET-OQ4 end: largest mode's N 48 or 54 by MODES-3. nit
- Checked: each "before" matched exactly once.
## Glossary step
- Retire (exact): the only hit ("the worst case of the largest mode", R3-BUDGET-REC2 heading) went with C9.14; script: R3 exact 0.
- Player count (in context "characters" for soldiers): legacy-facts R1-BND/R1-TICK/R2-INT line (Q4) "how many characters" -> "how many soldiers". nit
- Largest mode (in context bare "48" for the largest mode): R3-CHAR-REC1 heading and mark, R3-CHAR-REC2 mark, R3-CHAR-REC3 mark -> "the largest mode's N soldiers (48, or 54 under MODES-3 (b))". recommendation (value changes under MODES-3 (b))
- Very large unit (in context "large units" = the stress scene's): R3-BUDGET-REC2 arithmetic, the hull-outline and GTAO sentences ("the very large units' parts"). nit
- Stress scene (in context "R1-TICK's spike scene" as the scene a mark is set on): pointer Q5 to R1 -> "the stress scene, which R1-TICK's spike also runs". nit
- R1 wave-1 legacy fact (R1's old scene counts and its quote "to be named with R3, R5 and R6"): words kept, new terms added after it (stress scene with 15 big vehicles, 300 live projectiles; the reference server and R3-BUDGET-REC1's laptop, resolutions 9 and 11). nit
- Kept (judged): legacy measurements at 48 (TL15, GA15, CL6, STATUS risk 3); R3-LOOK-REC3's "48 soldiers" and "12, 24 and 48" (a look row: word for word, condition 5); "players" as people (hardware mix, fallback players, Serves 48+); "large units" for the category (CHAR-REC4 heading and reason, options, audio, Serves §6.4/§6.9 quotes); "effects" (look rows, §6.9 quotes, the GPU row is a time row, not a count); "always" (options rows, unrelated); "50 %" (GPU frame time named); "Largest N" heading (does not fix 48); Rests-on "R1-TICK's scene"/"R1-TICK's spike scene" (names R1's own scene).
- Links at first uses: time statistics (TL15 "mean"; line 201's "mean" is a verb), start/goal modes, team size, design point (Q5 intro), stress scene, big vehicles, live projectiles, reference server (R1 wave-1 fact), largest mode (CHAR-REC1 heading), very large units (CHAR-REC4 spike), N and visual effects (BUDGET-REC2 heading), client's sim share (BUDGET-REC2 row), preview (MEASURE-REC2), isolation flag (MEASURE-OQ4); approved "(GLOSSARY" markers converted to links. nit
- C10.13 inserted MODES-3 in new "### Shared open questions (the same words in every topic)" before "### Pointers to other topics" (links on its GLOSSARY markers; word-for-word check passed). nit
- R3-XT-OQ1..OQ8 inserted (ranks 4, 16, 25, 26, 33, 35, 37, 39; critic's words, word-for-word check passed) in "### Cross-topic open questions (consistency critic, 2026-10-29)" right after it. nit (8 entries)
- Pointers added (line "Cross-topic open questions: ..." before each named rec's spike mark): R3-WGPU-REC3 (XT-OQ8), R3-WGPU-REC4 (XT-OQ3), R3-CHAR-REC3 (XT-OQ2), R3-CHAR-REC4 (XT-OQ2), R3-AUDIO-REC3 (XT-OQ6). XT-OQ1, OQ7 name no R3 id; XT-OQ4, OQ5 name only R3 open questions; MODES-3 names R3-BUDGET-OQ4, which C10.12 already points to MODES-3. nit (5 lines)
- Header: GLOSSARY.md linked in the new line "Glossary and cross-topic resolutions applied 2026-10-30 (see GLOSSARY.md and part 7)"; totals 38 -> 47 OQs (38 own, 1 shared, 8 cross-topic); status label unchanged. nit (2)
- Part 7 ("## Verifier spot-check"): "### Consistency resolutions (2026-10-30)" added after "### Second pass (2026-10-29)". nit
- Link fix: big vehicles and live projectiles linked at their first use (R1 wave-1 fact, R1's old counts; words unchanged). nit (part of the link pass)
- Checks: glossary_check.py --skip-records: R3 exact 0, in context 77 (all judged above), header links GLOSSARY.md: True; first uses unlinked only Preview at "preview.js" (the legacy tool's name) and Time statistics at "can mean" (a verb): both false positives, linked at their real first uses. Shared and XT texts match the approved files word for word (links removed, whitespace normalized). repo-map-check OK. C17.b*/C23.b* not R3.

## Totals
recommendation 4 (C9.14; R3-CHAR-REC1, R3-CHAR-REC2, R3-CHAR-REC3: the largest mode's N, 48 or 54); number 1 (C9.13, approved arithmetic); source 0; nit 27 (C9.12, C9.15, C9.16, C10.12; MODES-3 + 8 XT entries; 5 pointers; 5 glossary wording changes; link pass; header 2; part 7). OBJECT: none.
DONE
## Step 3d fix round (2026-10-30, coordinator message; advisor-3d-r3-verdict.md items 5, 6; script's compound-48 hits)
