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
