applier for area GC (R7 legacy harvest, fixes2-ops-GC.json)
8 corrected checks (GC3, GC7, GC8, GC13, GC14, GC16, GC18, GC19); 29 ops in fixes2-ops-GC.json; apply-ops.py GC applies cleanly (22 fields of 8 rows -> after-GC.json).
Where-appends starting with "; " were written as replace ops on a unique tail of the field, so no " ;" gap appears.
GC3: what (last sentence replaced), where (mesh build appended), v2_note (ground-plane sentence replaced). Lines :1124, :1138 checked.
GC7: what (3 replaces: DEV god mode, multipliers/flat hits, damageBuilding inline/no gameOver guard), v2_note (kill money/tech/points sentence).
GC8: what (replace + append), where (append), v2_note (append), reason (set). Rule 2: correction cited onKill :1535 and playerDie :1547; code has them at :1536 and :1546 (grep -n), so used those.
GC13: v2_note (insert after playerFireOrigin (1800); replace 0.045 s step sentence), where (keys appended). T_CHARGE.nade.grav 20 confirmed at :1698.
GC14: what (first and last sentence replaced), where (append), v2_note (append). Rule 2: correction cited Molotov fuse :1410; code has fuse:1.4 at :1409, so used :1409.
GC16: where (JUG replace + callers append), what (cannon recoil), v2_note (typeof guards list). JUG reads confirmed by grep.
GC18: what (2 replaces), where (append). Bot '—' at :2477 confirmed.
GC19: what (max health), where (core.js:77 passage), reason (set). maxHealth 100/150/120/110 at core :128-137 confirmed.
DONE
