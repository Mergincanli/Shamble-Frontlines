checker3-GC: R7 final-review fix pass, checker for area GC
Ran apply-ops.py GC: 45 ops, 32 fields, 14 rows, no error. 14 fixes3 ops = 14 findings, all accepted.
verifier-game1#6 accepted OK: GC18.evidence fix_op verbatim; :3379 has the roundAssists +1/restore and 13 meleeCd; AUDIT_BUGS_PERF.md:70-74 names it CS2.
netcode-architect#0 accepted OK: GC3.v2_note; core:1507 groundY ground plane (SIM.groundY 0, core:1516); 1128 aimGround only on kind none; statics include buildings/platforms/ramps (game:1449-1457); DM_Cabana platforms h 1.2/2.5/4.0, ramp h1 2.5 (core:883) match probe; throwVelocity heading from aimGround 1711, callers 1406/1733/1760/1800; cannon 1304.
netcode-architect#2 accepted OK: GC3.v2_note; 1473 aimDir=aimPoint-_fireO; 1126-1129 aimPoint from SIM hit.
coverage-closer#2 accepted OK: GC3.where; 1110 declares _v/_p1/_p2; 1406 throwMolotov(1405), 1429 pitchWeapon(1428), 1617 enemyFire(1611), 2501 spawnDamageNumber(2500), 2543 blastReaches.
netcode-architect#3 accepted OK: GC5.v2_note; 1471 reset, 2690 decrement, 2913 bots, 1597 carry; 3/5/11 frames -> 10.0/12.0/13.09 per s.
coverage-closer#4 accepted OK: GC5.where; 1414 _sRight/_sUpv/_sDir, used 1618-1620; 1419 _fireO.
netcode-architect#4 accepted OK: GC7.evidence; three-teams-core.md:39 'Seeded random, effects stubbed', :77 camera-dependent random; 1029-1088 FX Math.random; 2240 sfxStep Math.random.
netcode-architect#5 accepted OK: GC17.v2_note; 1301,1232,1207,1289-1294,1303,1340-1341,2781,1343,1290 all hold; dropping 'one wall-clock read' is right (2674 _animClock, 957, 2947 also read performance.now).
netcode-architect#15 accepted OK: GC12.v2_note; 1666, 2739, 1671; player is the buildSoldier group (683), so 'render group' is accurate.
coverage-closer#5 accepted OK: GC12.where; 1669 _melTo, 1670 function.
coverage-closer#6 accepted OK: GC13.where; 1702-1705 names hold.
netcode-architect#16 accepted OK: GC15.v2_note; 2865-2869, 2873, 2881 hold.
game-designer#3 accepted OK: GC2.v2_note; LOOP_V2 §2 [DECIDED]/[DEFAULT], VISION_V2 §3 and §6 item 1 hold; 764-765 hq rule and soviets fallback; DM_Cabana is default (3189).
coverage-closer#3 accepted OK: GC10.where; 1168, 1169, 3294 hold.
No problems found.
DONE
