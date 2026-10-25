applier-CM: R7 fixes2 ops for area CM (corrected checks CM1, CM2, CM4)
CM1: look_brief_pending set true; v2_note replace (validation rejects -> should reject, recommendation text); evidence append (Reset tooltip mismatch, verified index.html:1058, editor.html:1059, editor:1101).
CM2: what replace (platforms + two ramps + refinery roof ramp, verified core:883); evidence replace AUDIT figures (kept the words "draw calls" in the new text so the numbers stay labelled; AUDIT:16 heading says draw calls main + shadow); evidence replace adding AD-1 player-shot citations (verified docs/reports/2026-10-15-team-prep-fixes.md:130/:150, STATUS.md:90).
CM4: look_brief_pending set true; where append header comment core:953-956 (verified).
apply-ops.py CM: 8 ops applied cleanly -> after-CM.json
DONE
