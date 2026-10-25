R7 applier ED (fixes2-ops-ED.json) — notes
Corrected checks: ED2 ED3 ED6 ED7 ED13 ED14 ED16
ED2: what replace (mirror Off/X/Z). v2_note still says "N-S/E-W mirror"; correction said all other fields stand, so left untouched (flag for review).
ED3: what replace (full caller list).
ED6: v2_note replace (adsK still read at game.js:290), evidence append.
ED7: v2_note replace (hold sliders edit one style+stance).
ED13: evidence replace; stale-name note placed after O4 so the sentence reads well (O2 and O4 text kept).
ED14: where, evidence, v2_note replaces (stray placeCursor comment); used plain quotes for find.
ED16: what replace; where done as replace on "bootEditor :1095 edSnapBtn click" (avoid " ; "), :158 and :222 qualified as shamble-editor.js since they follow shamble-game.js refs. Code checked: :873, :903-905, :1096, game.js:3255 KeyG, :3293.
11 ops, apply-ops.py clean -> after-ED.json
DONE
