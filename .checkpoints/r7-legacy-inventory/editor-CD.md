editor-CD: R7 legacy harvest, editor of area CD (shamble-core.js:1-231)
Inputs: rows-and-checks.json CD (21 rows, 21 checks, 3 missing, 7 not_covered); coverage.json suggested_rows: none name CD (core 138/138 covered).
Verified by grep at HEAD f930865 (engine files identical to 48ffb51):
- CD1 confirmed, unchanged.
- CD2 skeptic where+evidence applied; callers game:1952-1954 ProgStore, game:2336-2338 TutStore, editor:444, :695-699 MapStore, :704-708 PrefabStore (grep KVStore); TECHNICAL.md:206 26-assertion harness (sed).
- CD3 confirmed, unchanged.
- CD4 where+v2_note applied; T.arena writes only core:225 and game:77; setMapSize only called at game:3336 (dvMap slider).
- CD5 where applied; applyCamStyle callers game:3332, :3334; sliders :3326-3328, dvAdsCam :3333; :3329-3331 are halftone.
- CD6 evidence+v2_note applied; eliteHealth only at game:820 en.elite branch; bots spawn elite:false at game:3212.
- CD7, CD8, CD9 confirmed, unchanged.
- CD10 confirmed; evidence widened with skeptic's code-checked side note (.short has 0 reads; grep '\.short\b' over 5 files finds none).
- CD11 where applied; grep TUNE. 22 lines / 27 occurrences.
- CD12 what+evidence applied; falloffStart only at core:114.
- CD13, CD14, CD16, CD19 confirmed, unchanged.
- CD15 mark->redesign, reason, v2_note applied (look-pipeline rule).
- CD17 v2_note applied (drop invented 'roughly doubles').
- CD18 where applied; noRay( core 5 (def :168 + :395, :1445, :1457, :1470), game 45, editor 11; raycast=function at core :168, :230, :231.
- CD20 where applied; generateGrassTexture call at core:206 (regen branch), :226.
- CD21 v2_note applied.
Missing: (1) 'no uncovered system' -> nothing to add; (2) V8 let-tracking shim game:3374-3379 -> already GP16 (also cited CL1, GE17), rejected; (3) VISION_V2 not in 48ffb51 -> not a system; it is in main now (4afb714 is ancestor of HEAD, after 48ffb51), rejected.
Coverage: no suggested_rows name CD (core 138/138 covered). Changed for reasons 2-3: none.
final-CD.json written (21 rows)
DONE
