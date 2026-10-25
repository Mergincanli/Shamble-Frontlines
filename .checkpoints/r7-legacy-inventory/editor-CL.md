R7 editor of area CL (legacy harvest) — notes
Task: apply skeptic corrections to CL1-CL13, check 2 missing_systems and coverage "FIX CL12", write final-CL.json.
- Verified repo engine files unchanged vs 48ffb51 (git diff empty); spot-checked cited lines (core 233-462, 628-629, 1482; game 45-68, 1388, 1394, 1473, 1536, 1768, 2959, 3243, 3363-3379): all match the corrected rows.
- CL1,CL2,CL13: state D->C per skeptic (CL2 v2_note replaced). CL3 where (:283). CL4 where (:278-286, callers :404/:415 only). CL5 evidence+v2_note (game:3212 elite:false). CL6 where (:295-303)+evidence (core:387). CL7 where (:344, :333-347, :393). CL8 where (25 call sites on 21 lines). CL9, CL11 confirmed. CL10 what/where/state/v2_note. CL12 where+evidence (game:45/:46 etc.; this also satisfies coverage FIX CL12).
- missing[0] userData tag contract: real, no row -> new CL14. Skeptic lines partly off (:274 is `const head`, noOutline is :276; part read at :392 not :391; isOutline set :395 not :394; _toon set :384,:415; noMerge read :364 not :363; nature also :230, :1417, game:3070). Lines re-grepped.
- missing[1] halftone/_groundGrad/litCap/comicPost: rejected, covered elsewhere: CD16 (halftone core:148-155), CD20 (_groundGrad :185), CD19 (applyEnv :203-223 holds the litCap write :209-210, not CD20), GP9 (comicPost game:2353; skeptic said GP6, which is WebAudio SFX).
- CL14 added (userData tag contract), state C, mark redesign (look brief pending), look_brief_pending true; every line re-grepped (core :221 :230 :271 :276 :284 :364 :366 :384 :386 :392 :395 :401 :404 :405 :413 :415 :416 :482 :646 :686 :687 :689 :716 :753 :754 :755 :1414 :1417; game :245 :577 :583 :598 :1200 :2573 :3070; editor :814 :877 :882).
- Coverage "FIX CL12": already applied through the skeptic's CL12 where correction (game:45 GUN_WORDS, :46 KILL_WORDS); no further change.
- final-CL.json written (14 rows). changed: CL14.
DONE
