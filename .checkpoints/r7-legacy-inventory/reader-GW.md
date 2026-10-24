Task: R7 legacy harvest, reader GW (game world, map building, movement physics)
Checks: HEAD 698fbad = 48ffb51 + .checkpoints only. docs/VISION_V2.md absent at this checkout (used task text's v2 line). Lines verified by grep: rulesMerge 41, applyRules 43, makeComicTex 48, grid 69, rebuildBorder 72, setMapSize 77, addBox 85, makePlatform 89, rampGeometry 91, makeRamp 96, rampHeightAt 97, addPath 98, makeLabel 102, rebuildLOS 552, _sharedMapMats 554, _disposeObj3D 555, clearMap 559, placeLegacyHouse 577, spawnKitBuilding 578, _coverWorldBoxes 589, addCover 594, _propWorldBox 601, addProp 602, _treeWorldBox 605, addTree 606, makeEnemySpawn 613, curMap 623, buildMap 624, dedupeMapDef 652, serializeMap 660, syncSimWorld 1443, resolveObstacles 2526, surfaceHeightAt 2528, tickVertical 2532, blastReaches 2543, OCC_FADE 2555, _occTest 2559, updateOcclusion 2583, occRestoreAll 2602, bootGame 3055, BK.TREES 3059, buildTree 3064, BK.PROPS 3100, buildProp 3110, boot map pick 3189, _rulesReady=true 3236. T.gravity 24 / jumpV 11 / stepUp 0.55 at core:82.
Finding: updateOcclusion (2588-2599) fades obstacles, buildings, platforms, ramps, barracks only; houses/covers/trees/props never fade (code read).
Finding: tree collider colR scales with seeded Math.sin hash (3095-3097) -> collider size depends on JS float hash.
Finding: _propWorldBox uses Box3.setFromObject -> rotated props get loose world AABB.
Finding: blastReaches (2543) raycasts THREE meshes in losMeshes (rebuildLOS 552: obstacles, buildings, house bodies, los covers) -> trees, props, platforms, ramps never block blasts, though trees/props with los block shots in SIM statics (syncSimWorld 1459-1460).
GW1 manifest + PLAYER_FACTION/PLAYER_TEAM game:1-27 | redesign (LOOP_V2 §2) | LBP F | C
GW2 rules RULES_DEFAULT 40, rulesMerge 41, applyRules 43, currentRulesJSON 44, _rulesReady 42/3236, buildMap 645 | redesign LOOP_V2 §3/§4/§8 | LBP F | C; SEC-MDL-2 STATUS:68
GW3 comic words GUN_WORDS 45..comicKill 68, pool 3243, anim 2959 | port TS | LBP T | M (AUDIT B2 1.1 ms -> CS1 prewarm, 0 builds)
GW4 updateSunFollow 30 (called loop 3053) | port TS | LBP T | C; TECHNICAL:19 2048 map
GW5 grid 69, border 71-76, setMapSize 77 (dev slider 3336), makeLabel 102-108 | port TS | LBP T | C
GW6 registries 78-85, 98 (obstacles/paths/covers/trees/props, crateMat, addBox, addPath) | redesign EDITOR_V2 §2 | LBP T | C; editor-undo-ground stylizeOne fix
GW7 platforms/ramps 86-97 (makePlatform, rampGeometry, makeRamp) | port TS (builders) | LBP T | M team-prep-fixes ramp table
GW8 placement glue 577-608 (placeLegacyHouse, spawnKitBuilding, addCover, addProp, addTree) | redesign EDITOR_V2 §2 r3-4 | LBP T | M CS12 meshes 1611->370
GW9 collider derivation _coverWorldBoxes 589, _propWorldBox 601, _treeWorldBox 605 (+ core _bkWorldBoxes, CM) | rewrite Rust | LBP F | C
GW10 makeEnemySpawn 610-622 | redesign LOOP_V2 §2 | LBP T | C
GW11 buildMap 624-647 / clearMap 559-573 / _disposeObj3D 555 / _sharedMapMats 554 / curMap 623 / boot pick 3189 | redesign EDITOR_V2 §1-2 | LBP T | M editor-undo-ground, team-prep-fixes B6
GW12 serializeMap 660-682, dedupeMapDef 652-659 | redesign EDITOR_V2 §2 r1, §4 migration | LBP F | M map-dedupe-on-load
GW13 resolveObstacles 2526 (+551 comment) | rewrite Rust | LBP F | M AUDIT P1 7.5 %, CS10, CS13, bot-poplar-wedge R1
GW14 surfaceHeightAt 2528, rampHeightAt 97, tickVertical 2532-2542 | rewrite Rust | LBP F | C (T core:82)
GW15 blast LOS rebuildLOS/losMeshes 552, blastReaches 2543 | rewrite Rust | LBP F | C
GW16 syncSimWorld 1443-1461 | drop | LBP F | M team-prep-fixes 42.0->0.07 m, STATUS:118 561 B (stale, fixed three-teams-core:31)
GW17 occlusion 2549-2602 | port TS | LBP T | M AUDIT P1 17 %/13 %, CS11 20,000/20,000
GW18 BK.TREES/buildTree 3059-3098 (inside bootGame 3055) | port TS | LBP T | M CS12 trees -75 %
GW19 BK.PROPS/buildProp 3099-3188 | port TS | LBP T | M CS12 props -82 %
DONE
