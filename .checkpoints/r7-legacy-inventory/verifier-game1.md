verifier-game1: R7 legacy inventory spot-check, group game1 (GW, GA, GC): M rows + GW4 GW21 GA1 GA7 GC18 GC17
GW3: checked game:45-68, :3243, :2959, AUDIT:23 (B2 1.1 ms), :65 (CS1 pre-warm, builds 0), STATUS:68 (11 wall-clock reads). All confirmed.
GW4: game:30, core:167 (SUN_R/SUN_SD/_sunDir), call in loop :3053, TECHNICAL:19. Confirmed.
GW6: game:78-85,98,533 (barracks walls addBox false),665 filter o.crate; report editor-undo-ground:15-16 and STATUS:91 confirmed. But state M: cited report lines are a fix description, no measurement -> should, set C.
GW7: game:87-96 (8 verts, 36 idx=12 tris, stores x,z,w,d,h0,h1,rot), team-prep:72-79 (4 m ramp 20.3/8.7/0.6 -> 18.6/17.4/12.9), :130-133 AD-1, EDITOR_V1_INV:189 O5. Confirmed (LOOP_V2 §9 checked below).
GW8: game:575-608 (Math.random :602,:606), AUDIT:112 CS12 numbers, :42, :118, EDITOR_V1_INV:180 N3 numbers. Confirmed.
GW11: game:552-573, 623-647, 3189; editor-undo-ground:18, :51 PERF-2; team-prep:46, :69-70 B6; EDITOR_V1_INV:180. Confirmed.
GW12: game:648-682 (version :661, name :662, harv/tech/silos :676-679), editor :846-849,:994,:1100,:1111; dedupe report:66 'Gate: PASS, all 8 checks; symbols +2'; STATUS:53,:92-93,:98. Confirmed.
GW13: game:2526 (h.boxes||[h.box] alloc confirmed; Cabana houses core:890-893: brickFarm x=+-46 rot 0, manor rot 0 has boxes via _bkWorldBoxes), :551, callers :2780/:2933 (0.7), :2623-2624 (0.7+cs*0.5), editor:671; AUDIT:25,:110,:113; poplar-wedge:47; TECHNICAL:124 all confirmed. FINDING should: '0/11 rotated houses' is the audit's map; today's Cabana has 8/11 houses at +-90deg (core:890-893) - misleading.
GW16: game:1440-1462 confirmed; team-prep:55 (42.0->0.07 m), AUDIT:12, STATUS:118 (561 B), three-teams-core:31, TECHNICAL:92 confirmed. FINDING blocking (wrong line): houses loop is :1458 not ':1457 builds none' (:1457 is const rampHi=n).
GW17: game:2549-2602, call :2788, editor:226; AUDIT:25 (17 %, 13 %, ~8 MB/s), :111 CS11 20,000/20,000, comment :2561 ~0.9 MB/s, :42, :118, STATUS:98. Houses/covers/trees/props absent from loops :2588-2599 confirmed. Confirmed.
GW18: game:3055,3059-3098 (hash :3067, scale :3095), TECHNICAL:281,:53 (1.56), AUDIT:112 trees -75 % confirmed. nit: poplar stalls are poplar-wedge:25 (row cites :24 = tree-face stalls only).
GW19: game:3099-3188 (8 mats :3114-3121, +8 per-kind, stylizeOne :3186 and :604), AUDIT:112 props -82 % confirmed.
GW21: grep WORLD_FWD|WORLD_RIGHT over shamble-*.js, pages, tools -> only game:2603; :2748-2752 axes from camYaw. Confirmed.
GA1: game:132-193 (hold 153-163, reload 172-177, actions 179-185, aim 187-191), springStep 197, pools 231-240, own smoothing 279/370, no 'new' in 271-444 (grep), ANIM.aim only adsK read (game:290). Confirmed.
GA2: game:207-223, ARM_L1/L2 209/211, 9 call sites 358,415,416,427,432,435,472,486,488 all pass L1,L2; inverts 221 + 354,359,387,422,428,433 in updateSoldierAnim (also 469,473 in animDeath, not claimed); ANIM_PLAN §5 0.58/0.59, §6 stretch 0.98, hands 0.0 cm. Confirmed.
GA4: game:276-312 ranges ok, ANIM 134-146, walkRef 6.3 = T.moveSpeed (core:67), 6.3*1.4286=9.0; ANIM_PLAN §6 hip yaw jump 0. Confirmed.
GA5: game:313-344 (tuck :320 baseY+0.18, pitch 0.25), grounded from 2782/2936; ANIM 143-144; ANIM_PLAN §6 drift 0.0/0.14/5.8 cm, boots 0 cm; §8 item 5. Confirmed.
GA6: game:345-360, floor :350, ANIM 140/145; ANIM_PLAN §6 0.98, 0 cm. Confirmed.
GA7: game:361-366, ANIM 147-148; st.t=_animClock=performance.now()*0.001 (:2674) -> wall clock, no per-soldier phase in breath; ANIM_PLAN §8 item 4. Confirmed.
GLOBAL (should): many M rows cite only ANIMATION_PLAN.md §6 or AUDIT_BUGS_PERF.md, neither in docs/reports/ nor a tools/ harness, but inventory header line 31 defines [M] that way. Header def should widen (or rows go C).
GA8: game:367-418 sub-ranges ok, supT only 252/412 (grep incl tools), core:785-791 five guns, core:696-697 muzzle child of wpn, getMuzzle 1142 (sight line),1406,1472 (+playerFireOrigin),1733 (arc),1760; st.lean=ps.bayonet 2782; ROADMAP:44,:70; LOOP_V2 §5 tier 1 no bayonet. Confirmed.
GA9: game:378-381, 403-407, ANIM 172-177, _aKeys 238, core:786 reload; ANIM_PLAN §6 0.0 cm, §7 bots no reloads (rel=-1 at 2936), §8 item 1. Confirmed.
GA11: game:419-435, 377, 3192-3194, core makeSword 1435/makeAxe 1448/makeShield 1460, comment :684, swing 2740-2741, swingPose 1657, meleeHitCheck 2742 (arc+range, :1670). ANIM_PLAN §6 2.7 cm, 0.993; §7. Confirmed.
GA14: game:457-475, ANIM 180, reset 275, callers 1532/1825/2683/2938 (+editor:386 Anim Lab not listed, not claimed exhaustive); ANIM_PLAN §3 root never tilts [verified]; three-teams-core:72 1.4-1.8x; STATUS:84 risk 2. Confirmed.
GA15: game:204, 271, 2782, 2936, 369/373-375/417, pitchWeapon 1428-1432 calls 2711/2903/2921/2925, header 224-230, editor 323/385/388/672 confirmed; AUDIT:10 0.18 ms/7, P1, CS11 6.3->~4.9 MB/s, -1.5 MB/s held; STATUS:84 7.2-8.2; arithmetic 26 us,1.2/2.4 ms ok. FINDING should: second idle-decay writer rotation.x*=0.9 at game:2916 (objective role) not listed (row lists only 2928).
GC1: game:27, 514-519, core:103; steps-1-6:87 (7.45 ns, 0 B, TurboFan), :46 (54/29 -> 7/1), three-teams-core:37 h6-h13 PASS; STATUS:68 teamOf allow-list. Confirmed.
GC2: game:714-735, 762-779, 794, callers 646/1839/3225; steps-1-6:31 (12 slots), :88 parked-slot review fix; three-teams-core:48 (0.80-1.00), :49 (2.55/2.54/2.47). Confirmed.
GC3: game:32, 1109-1111, 1119-1134 (400 m, sync :1125 first in frame), 1135, 1136-1150 (bow :1138), call :2707, inputs 3284-3285/3299, mesh 3195-3204 (0xffd23f); AUDIT:12, CS2 :71-76. Confirmed.
GC5: game:1413-1421, 1442, 1465-1497, 2722-2730; team-prep:57 (8.1-17 us, 18,105 B -> 5.0-8.6 us, 648-750 B), STATUS:90 PERF-1 112 B/5.4 KB, AUDIT:14 fuzz 0. Confirmed.
GC6: game:1415, 1423-1427, setEnemyType :820, 1428-1433 callers, 1683-1692; core:785/792; AUDIT:30 P6, :15 soak 33.1->34.2 MB +63. Confirmed.
GC7: game:783, 1504-1526 (credit :1523), 1550-1556, 1559-1563 (no gameOver guard), core:85/:88, jug flat at :1312; three-teams-core:39 (69/207 events, 26 bldg, 1 truck), :37 h6-h13; STATUS:68. Confirmed.
GC8: game:1529-1548 (:1539, :1536 onKill, :1546), :1820, 2745-2746 (4/s), 2096-2102 Prog+saveProg, :2963; AUDIT:24 B3 4.2 ms, :107 CS7 40 nodes, :81 Brave 2026-09-26. Confirmed.
GC10: game:28, 1156-1184, unitOwned reset :1846, core:126-140 (Officer tier 2 cost 1250); AUDIT:14-15. Confirmed.
GC13: game:1696-1757 (jav :1697, AIM_ARC_N 48/300, h=0.045 :1734, statics :1742), tick 2857 g=20 / 2858 y 0.26 no wall test, keys 3266/3273/3284-3285, 1469/1406/1760/1800; team-prep:126 NC-7 77.7 KB, AUDIT:109 CS9. Confirmed.
GC14: game:783, 1719/1727/1759, 1652, 1758-1782 (1.5 s, R, R+4, R+3, 0.5x owner), 1409 (1.4 s), 2856-2859 (1.8), 2881/2883 (1.5), 2456 (50*priceMult, grenadeMax), 1846 reset, 2543 blastReaches over losMeshes 552; core:83; AUDIT:28 P4; steps-1-6:126-127 K2 80 %; STATUS:27. Confirmed.
GC17: game:1186-1190, 1297-1373 (5.5/3.2, 130 dmg/130 m/80 ms/3.4 s, +0.6 at 1304, 7 m behind 1369, arena clamp only 1332), 2791, 3267, 1509; no health writes for jugs, not in syncSimWorld, resetRound no jug clear; core:141-144; LOOP_V2 §6 :82, tech table :59; GAME_DESIGN:138. Confirmed.
GC18: game:1821-1824 (no callers; grep shamble-*, pages, tools), :510 findB, :783/:2675 meleeCd, :1697, thrown :1170/:1467/:1687/:1727, TEAM_HEX :710, axe :3193/:1666/:1685/:1689 (WEAPONS only katana), makeBolt :1790-1791 armed=true :1799, comment :1352, lastAttacker(Id) writes only, roundAssists :2477/:2489 ('—' for bots), JUG :1188; TECHNICAL:74,:280; AUDIT:109. FINDING nit: roundAssists and meleeCd also appear at :3379 (CS2 load-time let reassignment, +1 then restored) -> 'appears only at' slightly wrong.
GW13 finding detail: AUDIT:110 says 'The shipped map has none (0/24 obstacles, 0/11 houses)' meaning 45-degree pieces; row drops the qualifier.
Find strings for replace ops verified to occur exactly once (python count) in after-GW/GA/GC.json.
SUMMARY: 34 rows checked; 28 confirmed in full; findings: GW16 blocking (line :1457 -> :1458), GW13 should (0/11 houses qualifier), GW6 should (M without measurement -> C), GA15 should (writer 2916 missing), GA2 should (header [M] definition vs AUDIT/ANIMATION_PLAN-only rows), GW18 nit (:24 -> :24-25), GC18 nit (:3379).
DONE
