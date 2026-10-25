Task: R7 skeptic for area CM (core maps and building kit), rows CM1-CM14
Checks: git diff 48ffb51..HEAD on shamble-*.js/html/docs empty. State tags: C code read, M measured, D doc not re-checked (plan.md:18).
Code facts (node eval scratchpad/cm_eval.js of core 814-934,937-984): Cabana 41x29, obst 14, paths 83, bld 8, barr 2, silos 2, tech 3, field (0,0), spawn (-98.9,89.91), resup (-104.82,95.29); houses 11 (1 manor,2 thatch,4 timber,4 brickFarm), trees 70, props 36, covers 10 (4 fence, 6 hedge, 0 drystone); platforms h 1.2/2.5/4; 3 ramps (2 platform + roof of refinery h1=4). Romania 13 crates, 4 bld/team, 2 barracks. Tri HQs 106 m from (0,-23); enemySpawns 92 m (comment core:953 says 92).
Movement: resolveObstacles game:2526 tests rotated houses against localColliders in house frame (OBB-like), unrotated via h.boxes; covers incl. fences (skips if pos.y>max.y-0.2). SIM syncSimWorld houses game:1458, covers los-only game:1459.
DM_Cabana fallback data used by game: 704, 707, 744, 750, 973; editor reset editor:1101.
Reports verified: poplar-wedge :29 (433->0), :33 (5,150; 7,589/7,590), :41 (42.8 vs 43.5; 15.5 vs 15.0). team-prep B6 :69. STATUS :82 3a-1, :85 3a-4, :87 3a-6, :97 PERF-2, :108 R1 0.34 m, :118 561 B, :121 village 202, :30 house-textures-seeded.
VISION_V2.md not at 48ffb51/HEAD of r7 checkout; read from /home/user/Shamble-Frontlines/docs/VISION_V2.md:64-67 (§6.2 true 3D sim: floors, interiors, LOS across heights) - row citations match.
EDITOR_V2 §2 rule1 :16, rule3 :20, rule4 :23, §3 120 mirror :34, [LATER] interiors :42, §4 migration :48. LOOP_V2 §4 :39-60 (uplinks replace silos/tech boxes :47 [DEFAULT]); Riot Shield/Banzai blade :58,:65-66; §9 :115-131 (6 m gap :125, [OPEN] Cabana footprint :131).
CM14: arsenal ids only katana melee (core:118); no 'axe'/'sword' id -> makeAxe never shown; sword/axe/shield only on player (game:3192-3194) + editor puppet (ed:372-373); shield fixed 0x38e1b0, shown when ps.shieldOn (game:1690). Header core:14-15 lists weapon-prop makers with gun kit.
VERDICTS
CM1 confirmed (all counts reproduced; village.py:16; game:516; 813 holds a stray IK comment).
CM2 confirmed (counts, lines, AUDIT:16, CS12 :112, STATUS:121, poplar :33/:41, 3teams :41, AD-1 team-prep :19 + STATUS:90).
CM3 corrected: evidence "(scratch harness)" -> report says harnesses via tools/preview.js eval (poplar :17); 433->0 at :29. Rest ok; no tools ref.
CM4 corrected: perfbots figure is "combat 0.80-1.00 ms mean per update step (p95 0.97-1.54), 3 reps" (3teams :48). 92 m = enemySpawns/comment core:953; HQs at 106 m.
CM5 corrected: genTileTex (1009-1022) uses no Math.random -> 9 of 10 painters unseeded; PERF-2 wording (editor-undo-ground :51-53).
CM6 confirmed.
CM7 corrected: mark drop -> port to TypeScript (live code, 10 of 11 shipped houses; not dead/workaround; NAMED presets lack corner posts, straw rolls, overhang 0.95; consistent with CM8).
CM8 confirmed. CM9 confirmed. CM10 confirmed. CM12 confirmed.
CM11 corrected: Where add resolveObstacles game:2526 (movement collides with every cover incl. fences; skips when mover y > box top - 0.2).
CM13 corrected: movement does not read world boxes for rotated houses (game:2526 uses localColliders in house frame = oriented); CS10 :110 says movement already oriented; SIM houses still AABB game:1458.
CM14 corrected: axe never shown (no 'axe' id in arsenal; only katana melee core:118); props only on player (game:3192-3194) + editor puppet; shield fixed teal 0x38e1b0 shown when ps.shieldOn (game:1690); header core:14-15.
Missing: building-kit game glue row (game:577-599, 631-632, 670); DM_Cabana as objective fallback (game:704,707,744,750,973; ed:1101); def.env fog per map (core:855, 983 -> game:644); crate warm flag (core:863 -> game:626); misplaced comments core:813, core:985-986.
DONE
