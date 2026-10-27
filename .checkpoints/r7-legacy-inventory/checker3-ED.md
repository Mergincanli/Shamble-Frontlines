task: R7 checker3 for area ED (fixes3 check)
apply-ops.py ED: rc 0, 26 ops (12 fixes2 + 14 fixes3), 21 fields of 14 rows. Every fixes3 op maps to one finding; no other field touched.
verifier-edpg#2 accepted OK: ED2 evidence; hotfix report :29 H1..:34 H6, :47-56 rotation/Shift+wheel checks (grep -n).
verifier-edpg#3 accepted OK: ED15 evidence; diff of <body> sections = mEditor :1046, script tag :1259, bootEditor() :1262.
coverage-closer#12 accepted OK: ED16 where; :59 _pv1/_pv2/_pvAim, used :663-673 in edPreviewUpdate (:662), _pv1 at :180 in edMarqueePick (:178).
coverage-closer#13 accepted OK: ED2 where; :77,:78,:83,:84 names verified; ring :115-121 funcs; :122 _edStartResize, :123 _edStartRotate; mirror :162-167, _mirrorDesc :165 negates ax or az, rot, swaps team and faction.
coverage-closer#14 accepted OK: ED5 where; clUndoPush :298.
coverage-closer#15 accepted OK: ED8 where; :476 six vars, :491 measReset, :495, :503 ovHostileSpawn callers :510 :524, :530, :531, :532 ovMirror, :542 end.
coverage-closer#16 accepted OK: ED4 where; :646-653 verified; 10 of 11 setFromCamera use edCam(), the other is the playtest (:663) where edCam returns camera.
coverage-closer#17 accepted OK: ED9 where; _edRulesCtl :769 filled :798 read :799; _rulesFmt :787.
coverage-closer#18 accepted OK: ED12 where; :863, :864 verified.
coverage-closer#19 accepted OK: ED1 where; :856 sets _edLibMode, :864 decl, :971 reads only _edLibCollapsed, :1209 reads both (inside bootEditor 1022-1252).
DONE
