# editor-GC: R7 legacy harvest, editor of area GC
started; inputs loaded (18 reader rows, 18 checks, 3 missing, 3 coverage suggestions naming GC)
- GC1: kept (skeptic confirmed).
- GC2: skeptic where applied (buildTeamIndex :717-735; sed 735-736).
- GC3: updateAim :1119-1134, updateAimVisuals :1136-1150 per code (sed 1133-1136, 1150-1152); coverage WIDEN: :1109-1111 aim ray state, :1135 scratch (sed verified).
- GC4: skeptic where + v2_note applied (:2688 reserve check, :2735-2736 dev ammo).
- GC5: coverage WIDEN: :1413 muzzleW/aimDir, :1442 _hpV/_hnV/_dmgPt (grep verified).
- GC6: skeptic where + evidence applied (AUDIT :30 P6; :1683-1692).
- GC7: skeptic where applied (:1506-1526); coverage suggestion 2 partly: :783 kills (credited :1523).
- GC8: skeptic where + evidence applied; missing #3 (shield flash :1539 set, :2745-2746 decay) merged here (cosmetic feedback from damage event).
- GC9: skeptic what + evidence applied (x-only offset :1827; STATUS :88).
- GC10: kept.
- GC11, GC12: skeptic corrections applied (shield :1514-1515, F :3265; katana values core:118); look flag true.
- GC13: skeptic v2_note applied; coverage WIDEN :1706 scratch.
- GC14: look flag true; coverage suggestion 2 partly: grenadeCount :783 (:1719, :1727, :1759).
- GC15: look flag true. GC16: state C; coverage WIDEN :1191 scratch. GC17: what corrected (7 m behind, :1369).
- GC18: skeptic where applied; missing #2 merged (ps.lastAttackerId :1517, ps.lastAttacker :685/:1827/:1843, roundAssists :783/:1846 read :2477/:2489).
- GC19: NEW from missing #1, player health regen (:2699-2700, :1517, core:77; grep: no bot regen).
- Rejected: coverage suggestion 2's roundDeaths, gameOver, shopOpen (GE12, the named first target; roundDeaths written by onDeath :2104, progression); bot en.lastAttackerId (GN's dead-code row covers :1519/:1842/:3213); GP4/camShake parts of suggestions 1 and 3 (other areas).
- final-GC.json written (19 rows).
DONE
