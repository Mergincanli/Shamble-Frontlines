# Task: R7 legacy harvest, editor of area CS (final-CS.json)
Started 2026-10-25.
coverage.json suggested_rows: none name CS (core 138/138 covered). Skeptic missing_systems: 3 (M1 fence boundary, M2 hit-result kind/id contract, M3 adjacent ray code in game).
Line numbers verified by grep at HEAD (engine files identical to 48ffb51): FIXED_DT 1479, BTN 1480, makeInputCommand 1481, BODY 1482, PART 1483, raySphere 1484, rayCapsule 1485, rayAabb 1486-1490, resolveShot 1493 (comment 1491-1492), SIM 1516, rayStatic 1519, rayStatics 1529, hasLOS 1544, fence 1474-1478, bridge comment 1512-1515.
- CS1: confirmed, kept as read.
- CS2: skeptic correction applied (what/where/evidence/v2_note; _BODY_HIT core:462 duplicate).
- CS3: correction applied (bots flat damage game:1627; LOOP_V2.md:8).
- CS4: correction applied (where: parallel-axis fallback, callers 1498/1500/1502; evidence: node check).
- CS5: correction applied (rayAabb 1486-1490, updateArcPreview); M3 cross-ref added: third slab copy _occTest game:2559-2570 (owned GW17). CHANGED.
- CS6: correction applied (NaN rule, B1+B7, map size [OPEN] LOOP_V2.md:130).
- CS7: correction applied (1493-1510, caller functions); M2 widened: result record + kind/id contract (core:1494,1507,1509; game:1446-1448,1450,1451,1453; consumers 1128-1133,1312-1313,1493-1495,2870-2871). CHANGED.
- CS8: correction applied (sync exceptions game:1444, 2862, 778, editor:564).
- CS9: confirmed, kept.
- CS10: correction applied (core 323, 876; editor 9 Math.hypot).
- CS11: NEW from M1: SIM fence boundary core:1474-1478, header core:19-20, MODULE_MAP.md:120-123; outside deps CHAR_DEFAULT core:1482->421, hyp3 core:744; hasLOS reads global SIM. Mark drop (replaced by Rust crate boundary, V2_PHASE0_BRIEF.md:121/125).
- M3 rejected as separate row: _botFireWorld owned by GN7 (+GW16), arc preview by GC13, occlusion slab by GW17.
- final-CS.json written (11 rows CS1-CS11; changed for missing systems: CS5, CS7, CS11). Fixed CS5 inline slab range to core:1535-1538 by grep. Builder: scratchpad build_cs.py.
DONE
- CS11 fix: hyp3 use at core:1508 (grep), dropped unverifiable 'no lift-out attempted' sentence. DONE
