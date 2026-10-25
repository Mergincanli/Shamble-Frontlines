# Task: R7 legacy harvest, editor of area CM (final-CM.json)
Started 2026-10-25. coverage.json suggested_rows: none name CM (core 138/138 covered).
Skeptic missing_systems: 5 (M1 glue, M2 Cabana fallback, M3 env block, M4 crate warm, M5 misplaced comments).
## Progress
- Corrections verified in code: game:2526 resolveObstacles (covers boxes, pos.y>b.max.y-0.2 skip, localColliders for rotated houses), game:1458-1459, game:1690 shield vis, core:118 katana only, game:1685 isAxe=w.id==='axe', makeShield(0x38e1b0) at game:3194 / editor:373.
- M1 glue: REJECT, owned by GW8 (577-599), GW9 (589-593), GW11 (buildMap 624-647), GW12 (serializeMap 660-682).
- M2 DM_Cabana fallback: WIDEN CM1 (game:704, 707, 744, 750, 973, 3189, editor:1101; grep verified). GE11 already covers placeObjectives fallback logic.
- M3 env block: WIDEN CM2 (core:855 env fog 260/760 inside parser; read at game:644 / CD19) and CM4 Where (core:983).
- M4 crate warm: WIDEN CM1 (core:863 (r+c)&1; DM_Tri 961-966; game:626 crateMatWarm/crateMat; game:665 serialized; GW6 owns materials).
- M5 comments: REJECT as row (comments, no code: core:813 orphan IK header, no IK code in core; core:985-986 describes game:555-573). KIT SEAM (core:1197-1199) noted in CM5 Where.
- coverage suggestions naming CM: none.
- Rows CM1-CM14: corrections applied per rows-and-checks (CM3, CM4, CM5, CM7, CM11, CM13, CM14); writing final-CM.json next.
- final-CM.json written (14 rows, CM1-CM14; changed for missing systems: CM1, CM2, CM4). Builder: scratchpad build_cm.py.
DONE
