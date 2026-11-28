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
- Evidence: my own picture of the head's development page (built from the worktree at e1c90eec, code equal to
  c1de41e5), `.gate/art-director-s-head.png`, 1280 x 720 at pixel ratio 2, WebGL 2 fallback on SwiftShader (software),
  errors 0, counts 32 / 267 / 162 / 32 / 3 / 6. Its scene area is pixel-identical to `.gate/scene-after-4c.png`.
  Crops: `.gate/art-director-s-houses.png`, `.gate/art-director-s-yard.png`. Brave is final.
- Measured (median sRGB red channel, 13 x 9 px boxes; green/blue within +1/+4, the cool hemisphere light): grass 80,
  street 67, lane 66; steps 0.55 and 0.56 tops 109 and 109; ramp slopes 113 / 114; platform tops 1.0 m 119, 1.5 m 131;
  ledge tops 2.90 / 3.05 / 3.10 m 164 / 168 / 168; roofs (near slope) house A 4.7 m 200, house B 4.3 m 192; front walls
  house A 101, ledge 106. Grid lines 44 over everything.
- N1 (street, STREET-GREY (b)): met. Street 67 against grass 80 (13 levels), both below the darkest block top (109);
  the two street bands (the corner street and the yard's lane) show in the picture as darker strips. Faint, not lost.
- N2 (heights): four bands read from the overview (low steps about 110, platforms 119 to 131, ledges about 166, roofs
  192 to 200). Within a band nothing separates: 0.55 vs 0.56 identical, the three ledges within 4 levels, the two
  platforms 12 apart. As the constants give (greyOf, scene.ts BLOCK_LOW..BLOCK_HIGH over 0.55..4.7 m); not a defect,
  but checklist step 2's expected result promises more than shows.
- N3 (sides vs tops): every block's top is lighter than its own visible sides (roof 200 vs wall 101; ledge 164 vs 106).
  A tall block's front wall (101 to 106) is as dark as the lowest step's top (109): brightness tells height on tops only.
- N4 (gable roofs): the roofs read as one bright slab per house; the far slope faces away from the camera (roof pitch
  about 44 deg against the camera's 41 deg elevation, VIEW_DIRECTION (0,14,16)), so no ridge line shows; the gable end
  shows only as the slanted face at each house's right end. Step 2's "the roofs' ridges lightest" cannot be seen.
- N5 (other windows): pictures at pixel ratio 1, `.gate/art-director-s-1920x1080.png`, `.gate/art-director-s-1280x540.png`,
  `.gate/art-director-s-800x900.png` (WebGL 2 fallback, software; errors 0). The street holds its step at all three
  (mean with grid lines: grass 76 / street 63 at 1920 x 1080). At 1280 x 540 (wide, short) the readout panel (bottom at
  213 css px) covers the map's far grass strip (far edge at about 170 px); no block is hidden. At 800 x 900 the panel
  runs past the window's right edge (the backend line is cut); the scene's rows stay whole.
- N6 (readout rows): "content: maps/test-scene.json shamble-map 1 · movement/soldier.json shamble-movement 1" and
  "scene: test-scene · 17 boxes · 6 ramps · 6 regions" read cleanly (light monospace on near-black), wording as the sheet's
  step 4 expects (`overlay/report.ts:120-123`); the counts match the map (17 boxes, 6 ramps, 6 regions).

## Verdict: APPROVE

The greybox reads for what step S must show: four height bands told apart from the overview, sides darker than tops on
every block, the street in its own grey under STREET-GREY (b), the two new rows readable. Nothing in the code needs a
change for this brief. Two Mediums: his checklist's wording (AD-1, fix it before he gets the checklist) and the panel over
the scene on a short window (AD-4, a checklist line now); the rest are Low. Neither blocks the head.

Every picture cited below is the WebGL 2 fallback on the container's software renderer (SwiftShader), not WebGPU and not a
real GPU; Brave is final.

| Id | Severity | Finding | Where | Suggested fix |
|---|---|---|---|---|
| AD-1 | Medium | His checklist step 2 expects what the scene does not show: "the roofs' ridges lightest" (no ridge line is visible: the far roof slope faces away from the camera, N4); "the 0.55 m step the darkest block" (the 0.56 m step has the same grey, 109 = 109); "each block's grey lighter the higher its top" (true only between bands: the three ledges within 4 levels, the two steps equal). He would mark a correct scene as wrong, or confirm what he cannot see. | `reread-sheet.md` section G, step S, item 2; pictures `.gate/art-director-s-yard.png`, `.gate/art-director-s-houses.png` | Use step 2 of the checklist below in the report's "For Mustafa" (and the sheet). No code change. |
| AD-2 | Low | The street shows, but faintly: 13 levels of 255 between street (67) and grass (80) on the software renderer; it meets (b) as he worded it. Whether it is strong enough is his eye's call in Brave; the lever, if he asks, is `PATCH_GREYS` (`scene.ts:22`), still inside (b), and by his 6d it goes into the soldier's pull request. | `scene.ts:21-22`; `.gate/art-director-s-head.png` (the band from the houses to the middle; the yard's lane, bottom right) | Say in step 3 where the two bands are and that the step is small. No code change now. |
| AD-3 | Low | Brightness gives height on tops only: a tall block's front wall (101 to 106) is as dark as the lowest step's top (109). Inherent to a lit greybox; it could make him read a wall as a low block. | `scene.ts:64-66` (the sun and hemisphere light); `.gate/art-director-s-yard.png` (ledge fronts against the step tops) | Say it in step 2. Nothing to build: a look matter for LOOK_V2's change-set. |
| AD-4 | Medium | On a short window the readout panel (translucent near-black, bottom at 213 css px whatever the height) lies over the map and darkens what is under it, which falsifies the brightness-for-height read: at 1280 x 720 nothing is covered; at 1280 x 540 the roofs sit exactly at the panel's edge; at 1280 x 480 the roofs, the two steps and the ledge tops are under it and the roofs read dark. At 800 wide the backend line runs off the right edge. Workaround: a window at least about 600 px tall (maximised on his screen is fine). | `.gate/art-director-s-1280x540.png`, `.gate/art-director-s-1280x480.png`, `.gate/art-director-s-800x900.png` | Now: step 1 of the checklist below says it. Later, the main session's choice (fix pass or STATUS follow-up; after Sunday's freeze, by his 6d, into the soldier's pull request): fit the overview to the part of the window below the panel. |

Not verified: anything on WebGPU or a real GPU (every picture here is the WebGL 2 fallback on SwiftShader); the
greys on his screen; Firefox. Brave is final.

## His check of step S, as I would give it to him (about 6 minutes)
1. Open the page as the zip's README says (`node serve.mjs` in its folder, then the address it prints), in a normal
   Brave window. **Expected:** the whole map in one view from above at an angle (the street corner on the left, the
   training yard on the right); the readout's first line names the backend ("backend: WebGPU (...)" on most Windows
   machines; "WebGL 2 fallback, because ..." is also fine, just tell me which); "errors: 0". Keep the window maximised (at least about
   600 px tall): on a shorter window the black panel lies over the houses and the yard's back row and darkens them.
2. Look at the blocks. **Expected:** two houses on the left with the narrow alley between them, each a bright slab (the near side of its roof) over a
   darker front wall, with the slanted end of the gable on its right; the ridge itself does not show from this angle.
   A low cover wall on the grass just past the street, below them. In the yard: two small dark squares (the 0.55 m and 0.56 m steps, the
   same grey), three ledges just in front of them (the same light grey), a thin wall like a line, two platforms
   with their ramps (the ramp a little darker than its platform), six thin posts along the lane at the bottom right.
   Height reads in steps of brightness: steps darkest, then platforms, then ledges, then the roofs lightest; blocks
   within about 0.2 m of each other look the same. On every block the top is lighter than its sides; a tall block's
   wall can look as dark as a low step's top (only the tops carry height).
3. Look at the ground. **Expected (his STREET-GREY (b)):** two strips a little darker than the grass around them: the
   street from the left edge to the middle, between the houses and the cover wall, and the yard's lane at the bottom
   right, under the posts. The difference is small but should be plain; if you want it stronger or weaker, say so (it
   goes into the soldier's pull request).
4. Read the readout's two new lines. **Expected:** "content: maps/test-scene.json shamble-map 1 ·
   movement/soldier.json shamble-movement 1" and "scene: test-scene · 17 boxes · 6 ramps · 6 regions".
5. Press "Copy report" and paste it into the chat. **Expected:** the same rows (`content.*`, `scene.*`), "errors: 0",
   the render counts.
6. Open the same page once in Firefox. **Expected:** the same scene, greys and rows, "errors: 0"; tell me its backend
   line. If Brave and Firefox both say WebGPU, open the page once more in Brave with `?backend=webgl` added to the
   address (the first line then reads "backend: WebGL 2 fallback, asked for by ?backend=webgl ...", checked here) and
   check the greys look the same (the pictures here could only show WebGL 2).

Changes to his checklist against the sheet's section G: steps 1, 2, 3 and 6 reworded (step 1: keep the window maximised;
step 2: what shows and what does not, the alley kept; step 3: where the two street strips are; step 6: the backend line
and the optional `?backend=webgl` load); about 6 minutes instead of 5.

## Calibration notes (no art-director memory folder exists in this repository; kept here for the main session)
- The software render (SwiftShader, WebGL 2 fallback) of this scene is pixel-stable across runs: two runs 12 hours apart
  gave an identical scene area at 1280 x 720, pixel ratio 2.
- Pixel ratio 1 and 2 give the same street/grass step (about 13 levels); the 1 m grid does not drown it at 800 to 1920 wide.
- The readout panel's bottom sits at 213 css px at every window size; below about 600 px of window height it covers
  blocks of the test scene.
- Lit greys come out 2 to 4 levels bluer than authored (the hemisphere light's sky 0xdfe8f0); below what an eye sees.

DONE
