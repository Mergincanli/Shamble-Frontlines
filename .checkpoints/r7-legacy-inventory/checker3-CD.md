checker3-CD (R7 legacy harvest, checker for area CD, fixes3)
apply-ops.py CD: 7 ops, 6 fields, 5 rows; no other field touched.
engine-architect#11 accepted: OK. CD5 look_brief_pending true; v2_note sentence matches GP4 (after-GP GP4 true, same wording). core:68 camHeight/camBack/fov, core:161 PerspectiveCamera hold.
graphics-engineer#5 accepted: OK, same op as above.
netcode-architect#1 accepted: OK. grep weaponPitchMax = core:88 + game:1431 only; game:1428 function pitchWeapon, :1431 clamp, :1432 wpn.rotation.x; game:1465 fireOnce, :1473 aimDir from aimPoint, no clamp.
engine-architect#13 accepted: OK. core:104 roles; game:770 en.role=TUNE.ai.roles[ts%len] (per team slot); LOOP_V2.md:132/135 §10 role mix; GN9 v2_note cites §10.
game-designer#0 accepted: OK. game:701 era:1; :1178 unit tier>era; :1358 vehicle tier>era; :2439 era++; core:136 officer tier 2, core:142 juggernaut tier 2; LOOP_V2.md:53-60 Tech 2 = Officer/Javelin/Juggernaut.
game-designer#5 accepted: OK. Recomputed: bot DPS 32/5.19=6.17, 30/5.25=5.71, 6/4.8=1.25; pause 4.5+rand*0.6 (core:79, game:2913); flat enemyDmg game:1627 in enemyFire (:1611); lunge 48, <3.2, cd 2.5 game:2911 (core:133-135 lunge:48); AMK 34/3.5=9.7; Bagpipe 176/3.45=51; Shabzon 288/4.42=65, falloffStart 15; TTKs 0.86, 10.5, 0.85, 3.45, 1.30 all check. tools/: only tuning-dump.js names UNIT_TYPES (a dump, no matchup).
graphics-engineer#6 accepted: OK. core:147 antialias:true, core:156 min(devicePixelRatio,2), VISION_V2.md:52 minimum target.
No problems found.
DONE
