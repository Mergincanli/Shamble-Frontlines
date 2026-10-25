checker-ED: R7 legacy harvest, checker for area ED fixes2 ops
- apply-ops ED ran rc=0, 11 ops / 11 fields / 7 rows; only those 11 fields differ from final-ED.json.
- All 7 corrections' new text found verbatim in after-ED.json (ED13 note moved after O4 clause, ED16 where :158/:222 qualified as shamble-editor.js: both acceptable).
- Open: ED2 v2_note still says "The N-S/E-W mirror" (not in correction). Next: code spot-checks.
- Code at 48ffb51 unchanged (git diff empty). Spot-checks all hold: game.js 187-191, 271, 290, 479 (no game callers), 623-625, 660-662, 3044-3047, 3255 (KeyG, KeyM), 3293; editor 158, 162-168, 222, 369-370, 873, 903-905, 1068/1071, 1095-1096, 1111, 1249-1252; describe/spawn callers 166,169-171,214,219-220,640-642,710,952,961,1004; ea1ebb8 adds curMap; 67cf55b had name:'DM_Romania' (game.js:636 then).
- ED2 done: what applied exactly. PROBLEM: v2_note "The N-S/E-W mirror" uses the symmetry overlay's (ED8, editor :551) label for the X/Z mirror tool; fix op: replace with "The two-axis mirror (Off / X / Z)".
- ED3, ED6, ED7, ED13, ED14, ED16 done: applied exactly, code holds, no problem.
DONE
