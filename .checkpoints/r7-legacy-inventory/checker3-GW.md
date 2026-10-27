checker3-GW: R7 checker for area GW (fixes3 round). Started.
Applier run rc=0, 13 fixes3 ops; 13 findings to check.
- verifier-game1#0 + engine-architect#0 (GW16 evidence :1457->:1458): accepted, fixed. game:1457 const rampHi=n; :1458 houses loop. OK.
- verifier-game1#1 (GW13 evidence CS10 paraphrase): accepted, fixed as proposed; core:890-893 = 11 houses, 8 at +-1.5708; houses loop in game:2526 takes oriented branch. OK.
- netcode-architect#10 (GW16 v2_note): PROBLEM. Lines hold (core:1430-1433 _bkWorldBoxes, game:1458, :2526, :578-579, ed:151, core:890-893), but text says houses are mirrored through _bkWorldBoxes; only kit houses (game:580-581) and the manor (core:1180-1185) are. Cabana's 8 rotated houses (timber/thatch/brickFarm) take box=Box3.setFromObject(body) at core:1044/1099/1144 (game:631 legacy makers). Fix op: replace parenthetical.
  fix op: GW16 v2_note replace "houses are mirrored as world AABBs (_bkWorldBoxes, shamble-core.js:1430-1432, applies ... at :1458)" -> kit houses and manor via _bkWorldBoxes core:1430-1433; timber/thatch/brickFarm via Box3.setFromObject(body) core:1044, :1099, :1144. find occurs once (checked).
- engine-architect#4 (GW13 v2_note): accepted, fixed. game:2526 o.mesh.rotation.y/o.mesh.position, :85 Box3.setFromObject, :2780/:2933 callers. OK.
- verifier-game1#2 (GW6 state): applier kept M and appended the report's measurement instead; right call. editor-undo-ground.md:21 base ef019e0->a4d7289, :24 FAIL->PASS, :25 MeshStandard no ink -> MeshToon + ink. OK.
- verifier-game1#5 (GW18 evidence): fixed; bot-poplar-wedge.md:24 39->0, :25 4->0, 240 pairs seeded 90 s. OK.
- engine-architect#2 (GW15 v2_note): fixed; game:552 no alive filter, rebuildLOS callers 584/598/643 + editor only, :1561 death site, :2944 sink 0.7/0.85, three.js 8290-8305 raycast matrixWorld, 17867 updateMatrixWorld, :1450 alive, core:1531 alive===false skip (rayStatics :1529). OK.
- netcode-architect#9 (GW14 v2_note): fixed; integrated jumpV 11, g 24 (core:82): 2.25/2.43/2.483, formula 2.521, +0.55 = 2.98 vs 3.07; :1575, :2932, :1602 hold. OK.
- netcode-architect#11 (GW14 what + v2_note): fixed; :2780 true, :2933 omitted. OK.
- graphics-engineer#7 (GW17 v2_note): fixed exactly as proposed. OK.
- graphics-engineer#8 (GW17 what): fixed; :2578 ov=rec.op>0.9, outlines visible at :2581. OK.
- coverage-closer#11 (GW17 where): fixed; :2556, :2557 hold. OK.
- game-designer#6 (GW2 v2_note): fixed; GE1 where matches game:40-44, GE1 covers editor Rules panel. OK.
- No other fields changed by fixes3 (GW20 changes are fixes2 ops).
DONE
