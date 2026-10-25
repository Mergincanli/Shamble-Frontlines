R7 applier3 for area ED (fixes3-ops-ED.json)
verifier-edpg#2 (ED2 evidence): accepted. Hotfix report :29-34 lists H3 (live Rotation/X/Z drag) and H6 (Shift+wheel both ways); checks :47-56. Line refs corrected from the finding's :48-54.
verifier-edpg#3 (ED15 evidence): accepted. diff of <body> sections: +mEditor editor.html:1046, +script tag :1259, +bootEditor(); :1262.
coverage-closer#12 (ED16 where): accepted. shamble-editor.js:59 _pv1/_pv2/_pvAim; used :663-673 and :180.
coverage-closer#13 (ED2 where): accepted, written as inline replaces (not append) and added :84 _UP/_edGZC/_edPlane and :123 _edStartRotate. Lines :77,:78,:83,:115-123,:162-167 verified.
coverage-closer#14 (ED5 where): accepted. :298 clUndoPush; callers :308, :405.
coverage-closer#15 (ED8 where): accepted, inline replaces. :476, :491, :495, :503 (callers :510,:524), :530, :531 verified.
coverage-closer#16 (ED4 where): accepted, reworded per line. :646-653 verified.
coverage-closer#17 (ED9 where): accepted. :769 _edRulesCtl filled :798 read :799; :787 _rulesFmt.
coverage-closer#18 (ED12 where): accepted. :863, :864; _edInspHidden read :964, :971, toggled :972.
coverage-closer#19 (ED1 where): accepted with correction: edApplyGridCols :971 reads only _edLibCollapsed (not _edLibMode); rail toggle :1209 reads both.
Applied cleanly: 26 ops (fixes2+fixes3) on 14 rows.
DONE
