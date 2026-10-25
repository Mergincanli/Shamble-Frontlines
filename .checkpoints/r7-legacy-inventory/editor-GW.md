Task: R7 legacy harvest, editor of area GW (apply skeptic corrections, missing systems, coverage suggestions; write final-GW.json)
Inputs read: rows-and-checks.json GW (19 rows, 19 checks, 14 missing, 16 not_covered); coverage.json suggested_rows naming GW: (a) NEW GE (or GW) structures and barracks game:491-550, (b) FIX CL12 (not a GW row; GW3 already right).
Corrections (skeptic) to apply: GW5 what; GW9 where (core:1430 _bkWorldBoxes verified); GW10 what+where (:722, :806-809 verified); GW11 evidence (:69-70); GW12 where (editor:846/848/849/1100/1111 serializeMap, :994 dedupeMapDef verified); GW13 what+v2_note; GW14 v2_note; GW15 where (+:1781 verified); GW16 what. Confirmed unchanged: GW1-4, 6-8, 17-19.
Missing-system decisions:
- env model: reject, CD19/CD20 own core:169-223; game calls :644/:680 already in GW11/GW12.
- arena bounds: reject, source CD4/CD21 (core:225), player clamp GP3 (:2779), Juggernaut clamp GC17 tickPilot (:1317-1348 incl :1332); GW5 v2_note already says bounds -> map data + sim.
- map defs: reject, CM1-CM3.
- buildTeamIndex/placeObjectives/activateRoster: reject, GC2 + GE11.
- terminal + mapSpawn: folded into new GW20 (with coverage suggestion a).
- mergeStaticGroup: reject, CL9.
- stale orchard comment :609: reject, a comment not a system; orchard tools ED1 editor:18-45.
- player construction :683, :3192-3194: reject, CC6 (caller :683), GA11/CC12/CM14 (:3192-3194).
- WORLD_FWD/WORLD_RIGHT :2603: real, dead (grep: declared only, never read in any .js/.html) -> new GW21 drop.
- _sd/_so/_st: reject, GP10.
- ADS meshes :3195-3204: reject for GW, belongs to GC3 (cites :32 only; its editor should add :3195-3204).
- unitOwned: reject, GC10/GE13.
- character kit/ANIM: reject, CC6, GA1, GA2/CC11, GA15, GA16.
- slot table :3205-3218: reject, GN2.
Coverage: (a) structures game:491-550 -> new GW20 (GE reader not_covered defers base buildings to the world area; no other row owns makeBuilding/makeBarracks). (b) FIX CL12 -> not GW; GW3 lines :45/:46 verified correct.
GW1-GW4, GW6-GW8, GW17-GW19: kept as read (skeptic confirmed).
GW5, GW9, GW10, GW11, GW12, GW13, GW14, GW15, GW16: skeptic corrections applied verbatim (lines re-verified by grep/sed).
GW20: new, structures/barracks/terminal/mapSpawn, redesign (LOOP_V2 §2, §8), LBP true; lines 491-550, 521, 628, 636-642, 562-563, 572, 663-664, 674-675, 2793, 2973, 2978, core:86, core:864-867, editor:599-606, :621 verified.
GW21: new, dead WORLD_FWD/WORLD_RIGHT :2603, drop; grep finds only the declaration.
final-GW.json written (21 rows).
DONE
