checker3-GE: R7 GE fixes3 checker. started
apply-ops.py GE: 29 ops, 21 fields, 11 rows, no errors.
verifier-game2#0 accepted, fixed: GE12.where; :1374 useAbility, :1375 guard, :1465 fireOnce, :1466 guard.
verifier-game2#6 accepted, fixed: GE14.what; :1903/:1918 tier+HP, :1931 ammo, :1943 held, :1896 banner.
verifier-game2#7 accepted, fixed: GE17.evidence; core WEAPONS :108-:122.
engine-architect#6 accepted, fixed: GE5.v2_note; :2434 in addTech (:2432), callers :847/:916/:1523; LOOP_V2:37 §3.
engine-architect#7 accepted, fixed: GE8.v2_note replace (no contradiction left); :951-956, :1000-1003, LOOP_V2:44.
game-designer#2 accepted, fixed: GE4.v2_note; :701 era:1, :1178/:1358, :699 60, :1523, report:44 kills 3-8, TBOX :706 8/120/6, LOOP_V2:50-59 §4 (Officer/Juggernaut Tech 2); useAbility has no era check; all WEAPONS era 1.
game-designer#4 accepted, fixed: GE5.reason.
game-designer#8 accepted, fixed: GE15.v2_note; :2099-2100, :1957, :2106, node sum 2175855 /50 = 43517; addXP callers only :2102, :3347; LEGACY_TUNING.md:22; legacy-tuning.json has no xp keys.
game-designer#10 accepted, fixed: GE13.what; :2456.
coverage-closer#0 accepted, fixed: GE6.where; :821, :848-851, :870-871.
coverage-closer#1 accepted, fixed: GE10.where; :974, :990.
coverage-closer#7 accepted, fixed: GE17.where; :1863.
coverage-closer#8 accepted, fixed: GE15.where; :1955.
coverage-closer#9 accepted, fixed: GE16.where; :2124.
coverage-closer#10 accepted, fixed: GE4.where; :2418, :2419, :2420.
No problems found.
DONE
