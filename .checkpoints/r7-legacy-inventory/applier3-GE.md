applier3-GE: fixes3 applier for area GE (R7 legacy harvest)
15 findings in final-review-by-area.json GE; all verified against shamble-game.js / shamble-core.js / LOOP_V2.md / reports.
- verifier-game2#0 GE12.where accepted: :1375 is in useAbility (:1374), :1466 in fireOnce (:1465). fix_op used.
- verifier-game2#6 GE14.what accepted: shop rows :1903/:1918 tier+HP, :1931 ammo, :1943 held count; own op (folds the warning/banner into one parenthesis).
- verifier-game2#7 GE17.evidence accepted: core WEAPONS 108-122 (]; at 122). fix_op used.
- engine-architect#6 GE5.v2_note accepted: :2434 tech cut; LOOP_V2 §3 keeps only pricing, §4 uplinks. Own append (no double space).
- engine-architect#7 GE8.v2_note accepted: LOOP_V2 §4 step 4 silent on contests; :951-956/:1000-1003 decay + reset. Own replace of the "keep contested rule" sentence (and trimmed the duplicate hold-E sentence) instead of a contradicting append.
- game-designer#2 GE4.v2_note accepted: :701 era 1, :1178/:1358 tier gates, no era check in useAbility, report:44, ECO :694-699, TBOX :706, techPerHeadshot 2 explains 3-16. Own append, labelled as estimate.
- game-designer#4 GE5.reason accepted: fix_op used.
- game-designer#8 GE15.v2_note accepted: :1957/:2099-2100/:2106 literals; sum xpForLevel(1..99)=2,175,855 (node); LEGACY_TUNING.md:22. Own append (path fixed, ROMAN listed, XP sources :2102/:3347).
- game-designer#10 GE13.what accepted: :2456 cost 50. fix_op used.
- coverage-closer#0 GE6.where accepted (:821 _hvDir, :848-851). fix_op used.
- coverage-closer#1 GE10.where accepted (:974 siloRateEl, :990). fix_op used.
- coverage-closer#7 GE17.where accepted (:1863 pips). fix_op used.
- coverage-closer#8 GE15.where accepted (:1955 _progSaveTO). fix_op used.
- coverage-closer#9 GE16.where accepted (:2124 ccIconEl...). fix_op used.
- coverage-closer#10 GE4.where accepted (:2418, :2419). fix_op used.
apply-ops.py GE applies cleanly (29 ops, 21 fields, 11 rows).
DONE
