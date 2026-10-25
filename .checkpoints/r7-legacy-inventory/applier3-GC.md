applier3-GC: R7 fixes3 applier for area GC (final review findings)
Ops in fixes3-ops-GC.json (14 ops); apply-ops.py GC applies cleanly (45 ops, 32 fields, 14 rows).
- verifier-game1#6 GC18.evidence: accepted, fix_op as written. sed 3379p: CS2 let reassignment adds 1 then restores roundAssists; meleeCd 13 times.
- netcode-architect#0 GC3.v2_note: accepted, own op. Probe (preview eval, DM_Cabana default): SIM down-ray stops 1.20/2.50/4.00 on platforms, 2.50 on ramp (rampHeightAt 1.25). Ground plane at core:1507. Widened 'throws' to throwVelocity users incl. the bow (fireBolt 1800 uses throwVelocity -> aimGround 1711).
- netcode-architect#2 GC3.v2_note: accepted, fix_op. 1473 aimDir=aimPoint-_fireO; 1126-1129.
- coverage-closer#2 GC3.where: accepted, own wording (no nested parens). 1110 declares _v/_p1/_p2; uses at 1406,1429,1617,2501,2543 checked.
- netcode-architect#3 GC5.v2_note: accepted, fix_op plus frame arithmetic (ceil(0.072/dt)=3,5,11 frames). 1471, 2690, 2913, 1597 checked.
- coverage-closer#4 GC5.where: accepted, fix_op (range :1618-1620). 1414, 1419, 1618-1620 checked.
- netcode-architect#4 GC7.evidence: accepted, own op citing FX 1029-1088 (10 Math.random) and sound 2240. report :39 and :77 checked.
- netcode-architect#5 GC17.v2_note: accepted, edited: dropped 'the one wall-clock read in the game' (soldier rig breath also runs on performance.now, 2674 -> 363, and grenades launch from the rig muzzle), bob range 1289-1292. Lines 1207,1232,1290-1294,1301,1303,1340-1341,1343,2781 checked.
- netcode-architect#15 GC12.v2_note: accepted, 'render group's facing' instead of 'mesh's'. 1666, 1671, 2739 checked.
- coverage-closer#5 GC12.where: accepted, fix_op. 1669 _melTo, 1670 meleeHitCheck.
- coverage-closer#6 GC13.where: accepted, fix_op. 1702-1705 names checked.
- netcode-architect#16 GC15.v2_note: accepted, fix_op. 2865-2869, 2873, 2881 checked.
- game-designer#3 GC2.v2_note: accepted, own wording (LOOP_V2 §2 [DEFAULT] team count+size as data; VISION_V2 §3 :43,:48, §6 item 1). Probe: DM_Cabana active allies+soviets, empire false; tri all three. 764-765 checked.
- coverage-closer#3 GC10.where: accepted, fix_op. 1168 _devWheelT, 1169 devCycleGun, 3294 wheel listener.
DONE
