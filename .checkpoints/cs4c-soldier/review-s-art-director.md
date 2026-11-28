# Step S, art-director review (narrow brief: does the greybox read; his Brave checklist)

_Reviewed: `git diff dae6d107 c1de41e5 -- packages crates content`. Narrow brief (advisor-reread-4c.md item 4): heights
told apart from the overview, sides against tops, the street shown by STREET-GREY (b), the readout's new rows readable,
and the Brave checklist (reread-sheet.md section G, step S). No judgement against ART_DIRECTION.md or LOOK_V2.md._

## Plan
1. Read `packages/client/src/render/scene.ts` at c1de41e5: `greyOf`, `groundGrey`, the constants, the lighting; compute
   every block's grey and every ground grey for the test scene; check (b): two neutral greys, both darker than every block.
2. Read the map (`content/maps/test-scene.json` or wherever) to list heights; check the order lightness follows height.
3. Look at `.gate/scene-after-4c.png` (and make my own with shot-4c.mjs if needed); judge heights, sides vs tops,
   street vs grass from the overview. Software renderer (WebGL 2 fallback): Brave is final.
4. Read `overlay/report.ts` rows; check they read in the picture and match checklist step 4's expected text.
5. Check the six Brave steps against what the code draws; amend if any expected result is wrong.

## Notes (one line per finding as I go)
