checker-CM: R7 checker for area CM (fixes2)
Ran apply-ops.py CM: 8 ops, 7 fields, 3 rows; row-by-row diff of final vs after shows only those 7 fields changed, 14 rows, same order.
CM1: lbp true, v2_note replace verbatim, evidence append verbatim. Checked index.html:1058, editor.html:1059 (tooltip 'Clear the map back to empty'), shamble-editor.js:1101 (buildMap(DM_Cabana)), EDITOR_V2 §3 (:28-42 validation panel, objectives as data, no missing-objective check). OK.
CM2: what replace verbatim; core:883 ramps h1 2.5 and 4.0, roof ramp via _roofH (refinery 4 lowest). evidence: replace keeps 'draw calls' beyond correction wording (accurate: AUDIT_BUGS_PERF.md:16 'Draw calls (main + shadow) at heavy views'); judged acceptable, keeps the unit from the original text. :112 CS12 draw calls; team-prep-fixes.md:130 AD-1, :150 player shots and LOS still see ramp box; STATUS.md:90 same. OK.
CM4: lbp true, where append verbatim; core:953-956 holds 120 deg, 92 m from (0,-23), even-integer barracks rot 0 for nav grid, 6 m gaps checked by 3a layout script. OK.
Result: ok true, no problems.
DONE
