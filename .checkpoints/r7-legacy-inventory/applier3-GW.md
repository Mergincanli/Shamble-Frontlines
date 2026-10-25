applier3-GW: R7 fixes3 applier for area GW (final review findings)
- verifier-game1#0 GW16 evidence: accepted; game:1457 is `const rampHi=n;`, houses loop at :1458 (sed -n). Op: ":1457" -> "houses loop at :1458".
- engine-architect#0 GW16 evidence: accepted (same fix as verifier-game1#0, one op).
- verifier-game1#1 GW13 evidence: accepted; AUDIT:110 "0/24 obstacles, 0/11 houses" = no 45° pieces; core:890-893 has 8/11 houses at ±1.5708; game:2526 oriented branch on |th|>=1e-3. fix_op used.
- verifier-game1#2 GW6 state: accepted with a better op: the report does measure (editor-undo-ground.md:24-25 harness FAIL->PASS, MeshStandard->MeshToon+ink), so state stays M and the measurement is appended to evidence.
- verifier-game1#5 GW18 evidence: accepted; bot-poplar-wedge.md:24 tree-face 39->0, :25 poplar 4->0. fix_op used.
- engine-architect#2 GW15 v2_note: accepted; game:552 no alive filter, rebuildLOS callers 584/598/643 + editor only; :2944 sink 0.7/s to 0.15h; :2543 intersectObjects; three 0.128.0 raycast matrixWorld 8290-8305, render updateMatrixWorld 17867; core:1531 skips alive===false. Op appended (reworded "Also from the code", cites :1561 death).
- engine-architect#4 GW13 v2_note: accepted; game:2526 o.mesh.rotation.y / position, :85 Box3.setFromObject, callers 2780/2933. fix_op used.
- netcode-architect#9 GW14 v2_note: accepted; :2537 jump, :2538 gravity, EJUMP_REACH :1575 used :1602 (enemyClimbSteer) and :2932; semi-implicit Euler apex 2.250/2.430/2.483 at 20/60/144 Hz vs 2.521 formula (python check). Op appended (enemyClimbSteer :1602).
- netcode-architect#11 GW14 v2_note: accepted; :2780 passes true, :2933 omits, editor :671 true. Appended to v2_note and clarified "what".
- netcode-architect#10 GW16 v2_note: accepted with corrections: _bkWorldBoxes is core:1430-1432 (1429 is a comment), and the last sentence fixed: Cabana has 8 houses at ±90° (exact AABB), none off-grid.
- graphics-engineer#7 GW17 v2_note: accepted; :2574-2575 transparent=true, :2582 reset; replaced the dither/stencil sentence. fix_op used.
- graphics-engineer#8 GW17 what: accepted; :2578 ov=rec.op>0.9, :2580 outlines visible=ov. fix_op used.
- coverage-closer#11 GW17 where: accepted; :2556-2557 verified. fix_op used.
- game-designer#6 GW2 v2_note: accepted; GE1 cites the same :40-44; GW2 has content GE1 lacks (§8, _rulesReady), so no remove; cross-reference appended.
apply-ops.py GW: 16 ops applied to 16 fields of 9 rows, clean.
DONE
