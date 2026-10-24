R7 legacy inventory — reader CM (core maps and building kit, shamble-core.js 846-1473)
## Read pass done (core 814-1473). Facts:
- cabanaRepublicMap core:814 (ASCII, just before scope); GRID_SIZE 850; parseGridToMapDef 851-881; DM_Cabana 882; platforms/ramps IIFE 883
- village pass houses 889, trees 895, props 915, covers 929 (mirrored, validated tools/village.py)
- DM_Romania 937-952; DM_Tri 957-984 (3 teams allies/soviets/empire, enemySpawns, harvLanes)
- orphan C1 comment 985-986 (clearMap dispose; code lives elsewhere)
- genTimberTex 987, gableRoofGeo 1001, genTileTex 1009, makeTimberHouse 1023, genPlasterTex 1049, genThatchTex 1056, makeThatchCottage 1071, genBrickFlintTex 1104, makeBrickFlintFarmhouse 1123, makeManorHouse 1150 (localColliders + _bkWorldBoxes)
- BK section 1190; _mulberry32 1201; _hipRoofGeo 1204; _halfHipRoofGeo 1213; _gambrelRoofGeo 1233; _roofFor 1253; skins _bkBrickTex 1265 _bkSlateTex 1271 _bkStoneTex 1279 _bkHedgeTex 1286 _bkDirtTex 1291
- BK 1296 (PARTS 1299, DIMS 1307, PRESETS normandy 1309, NAMED 1315); BK.tex 1321 (cache, repeats); BK.roll 1328; BK.build 1337-1395 (footprints U/L/T/longbar 1347-1362, chimney, door 1371 r=0.7 char, dormers 1379); BK.COVER 1397 (los flag); BK.buildCover 1403; _bkWorldBoxes 1430
- 1434-1472 MELEE_IDLE, makeSword, makeAxe, makeShield (melee weapon models) — in range but not maps
- Legacy makers use Math.random in textures (not seeded) and new materials per house (not shared)
## Checks done
- HEAD 698fbad = 48ffb51 + .checkpoints only. VISION_V2.md absent at 48ffb51; read from /home/user/Shamble-Frontlines/docs/VISION_V2.md (§3 modes as data, §6 true 3D sim, interiors).
- Cabana eval (node, core 814-883): grid 41x29 cells x 10 m; 14 crates, 83 path tiles, 8 buildings, 2 barracks, 2 silos, 3 tech, spawn (-98.9,89.91), resupply (-104.82,95.29), field (0,0), 3 platforms, 3 ramps. Village: 11 houses (1 manor, 2 thatch, 4 timber, 4 brickFarm), 70 trees, 36 props, 10 covers.
- DM_Romania: no code path builds it (game:99 comment, editor placeholder text only); boot game:3189 picks DM_Cabana or DM_Tri (?map=tri). team ids friend/hostile mapped game:516.
- Consumers: buildMap houses game:631 (bt->maker, h.spec->spawnKitBuilding); serialize game:669; spawnKitBuilding game:578; addCover game:594 (_coverWorldBoxes 589); rebuildLOS game:552 (covers los only); syncSimWorld houses boxes game:1458, covers 1459; editor _bkSpec editor:801, BK.roll editor:819.
- Legacy makers build new canvas textures per house (genXTex direct, not BK.tex): 10 non-manor Cabana houses x2 = 20 textures; materials new per house.
- Colliders: legacy = Box3 of body only (walls; roof/chimney not solid); manor 2 local boxes; kit one box per block to wallH; door cosmetic (collider solid).
## Evidence
- AUDIT §1 line16 draw calls village 303 / manor 238; P5; CS12 meshes 1,611->370 (houses -60%, covers -79%), village 398->202, manor 284->162; STATUS health village draw calls 202
- CS10: shipped map 0/24 obstacles, 0/11 houses at odd angles
- editor-undo-ground report: manor framing differs run to run (unseeded house textures; house-textures-seeded queued, STATUS:30); PERF-2 _disposeObj3D disposes shared BK._tex (STATUS risks)
- bot-poplar-wedge: Romania 5,150/5,150 pairs, Cabana 7,589/7,590; navBuild 42.8 vs 43.5 ms Cabana, 15.5/15.0 Romania
- three-teams-core: DM_Tri 120deg symmetry within 0.007 m; reach.js DM_Tri 180 routes 100 %; Cabana objective climbs 72.5 %, siege 75 % (R19); DM_Tri 3x300 s 10 deliveries/team, 0 errors; perfbots 48 soldiers 0.80-1.00 ms
- STATUS risk 3a-4: DM_Tri Soviet/Empire barracks doors face south; risk 3a-6 truck lanes map data
- ART_DIRECTION:127 / TECHNICAL:250 brick-flint candy stripes, one preset normandy; TECHNICAL:72 says DM_Romania is the map data (drift)
- EDITOR_V1_INVENTORY J3/J4 keep (pickers from BK.PARTS/DIMS), J7 copies lose spec (NEW-3)
## Rows planned: CM1 grid parser, CM2 Cabana content (platforms/ramps+village), CM3 DM_Romania drop, CM4 DM_Tri, CM5 textures+BK.tex, CM6 roof geos, CM7 three legacy makers (drop), CM8 manor, CM9 BK schema+roll, CM10 BK.build, CM11 BK.COVER rules (Rust), CM12 buildCover, CM13 _bkWorldBoxes, CM14 melee props
## Rows (final marks)
CM1 grid map+parser core:814,850-882 | C | redesign (EDITOR_V2 §2.1 JSON doc; LOOP_V2 §4 uplinks replace O/X) | look false
CM2 Cabana content 883-934 | M | redesign (LOOP_V2 §9 Octopon-3; migrate via EDITOR_V2 §4) | look true
CM3 DM_Romania 937-952 | M | drop (no code path builds it) | false
CM4 DM_Tri 957-984 | M | redesign (LOOP_V2 §9 Octopon-3, §4 uplinks) | false
CM5 canvas skins 987..1294 + BK.tex 1321 | M | port TS | true
CM6 roof geos 1001,1204,1213,1233,1253 | C | port TS | true
CM7 three legacy makers 1023,1071,1123 | M | drop (dup of BK.NAMED) | true
CM8 manor 1150-1188 | M | port TS (as kit part) | true
CM9 BK schema+roll 1201,1296-1335 | C | port TS | true
CM10 BK.build 1337-1395 | C | port TS | true
CM11 BK.COVER 1397-1401 | C | rewrite Rust | false
CM12 BK.buildCover 1403-1427 | M | port TS | true
CM13 _bkWorldBoxes 1430-1433 | C | rewrite Rust | false
CM14 melee props 1434-1472 | C | port TS | true
not_covered: C1 comment 985-986 (orphan); game-side glue (spawnKitBuilding, addCover, BK.TREES/PROPS in game) = game areas; buildMap/clearMap game
DONE
