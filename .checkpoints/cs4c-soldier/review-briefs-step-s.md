# Step S's review: the six briefs, ready to send (written 2026-11-28, before his SIZE-4C answer)

_The main session's notes. Sent as they stand, with the head filled in, the moment his SIZE-4C answer starts step S's
reviews: under (b) or (c) at once on 4c-1's head; under (a) on 4c's final diff, rewritten for it. One round, three at a
time (the advisor agent's item 4): graphics-engineer, perf-auditor and engine-architect first, then security-auditor,
test-engineer and art-director. Each writes `.checkpoints/cs4c-soldier/review-s-<agent>.md` (its plan first, a notes
line after each finding, the verdict, then DONE) and returns at most 12 lines._

## Common to all six
- **What is reviewed:** `git diff dae6d107 <head> -- packages crates content` (23 files, 1,288 lines added and 133
  removed at bc7720f3 + c1de41e5; the count's 1,288 written lines). The change: the page's scene, drawn from the map
  read field by field (4b's old step 12; his STEP12-GO (a), `docs/DECISIONS.md:183`), by the plan sheet's section D
  (`.checkpoints/cs4c-soldier/reread-sheet.md`) under the advisor agent's conditions 1 to 9
  (`advisor-reread-4c.md`, its verdict).
- **The files:** `packages/client/src/picture.ts` (the interface: `PictureContent`, `SceneObject`, `setContent`,
  `inventory`); `render/scene.ts` (new: the empty floor moved from the deleted `empty-scene.ts`; the map's floor, solids
  built from corners as f32 with flat outward faces, greys by height, the ground kinds' greys, the grid, disposal, the
  overview camera); `render/inventory.ts` (new: read back from the drawn buffers, development only); `render/
  three-picture.ts` (`setContent`, `inventory`, the frame on resize); `app/content.ts` (`readMap` with closed key sets,
  `readContent` by kind, format and version, `PageContent(deliver, read)`); `app/boot.ts` (the hand-off, the readout's
  data, `window.__shambleScene`); `overlay/report.ts` (the content and scene rows); `sim-worker/src/local-map.ts`
  (`LOCAL_MAP`); `scripts/build-checks.mjs` (two development markers); the tests (content, content-map, content-ties,
  scene, report, content-schema) and the browser specs (skeleton, content, render-counts.json, report.ts).
- **Evidence so far:** the gate PASS with smoke; the full local checks PASS (the browser run 29 of 29 in 217 s); CI green
  on bc7720f3 and on 8bee8365 (gate, smoke, toolchain); the plants run 28 of 28 caught with 3 of 3 controls held
  (`plants-4c.txt`); the render counts 32 / 267 / 162 / 32 / 3 / 6, the same with the map drawn in before and after the
  first frame (`counts-order-4c.mjs`); allocation 85 bytes a frame against main's 100 (the sampling profiler's noise);
  the before and after pictures `.gate/scene-before-4c.png`, `.gate/scene-after-4c.png` (not committed).
- **Known Low, for the round to address or defer:** G-3 (engine-architect's read, block 5): no test tells the bounds'
  bottom from y 0; `packages/client/src/render/scene.test.ts:149` checks the literal 0, while the scene draws the
  ground at `bounds.min[1]` (every map so far starts at 0). The fix is one test on a map raised above 0.
- **Rules:** read-only; write only your review file. Severity on the one scale (Critical, High, Medium, Low), each
  finding with file:line and a failure case. Shell: git and gh always start their own command, with no other program in
  front of them; every file a command writes spelled out with its full path; `git checkout` and `git restore` are
  refused (read with `git show <ref>:<path>`).

## The six, each on its own part
1. **graphics-engineer** (the page's drawing code): `scene.ts`'s geometry (the f32 corners, the same bits as the
   simulation's; face orientation by the centroid test; per-face normals; the enclosed-volume test), the materials and
   the WebGPU and WebGL 2 paths (one renderer, the node materials), the grid, disposal on a new map (`disposeFloor`), the
   overview's fit by halving, the render counts and whether they stay bounded for a map of 16 spawns, 64 regions and the
   most shapes the loader allows.
2. **perf-auditor:** nothing per frame allocates (the inventory runs only on call, development only); the scene is
   built once per delivered map; the draw calls (32 for the test scene) against the GPU budget; the frame on resize; the
   counts' order (the map before or after the first frame).
3. **engine-architect** (the boundary): the interface file (`picture.ts`) and what crosses it; `LOCAL_MAP` in the
   Worker's own module (the page takes in nothing of `recording.ts`); `readMap` against the Rust reader
   (`crates/content/src/map.rs`), field by field, with the ties test; the development cut by `__SHAMBLE_DEVTOOLS__`
   (the production page holds `inventory(){return null}` and neither marker); 4B-TYPES-READER, 4B-MEMBER-KINDS,
   4B-BOOT-CALLS.
4. **security-auditor** (content text shown on the page): every map string that reaches the DOM goes through
   `textContent` (the readout's content and scene rows, the ids); the closed key sets and the refusal of unknown kinds,
   formats and versions; nothing read through a prototype (`__proto__`, `constructor` keys); the two fixed sentences
   (`NOT_THE_MAP`, `INVENTORY_UNKNOWN`); `window.__shambleScene` only in the development build.
5. **test-engineer** (is it proven): each test's planted defect (`plants-4c.txt`: 28 caught, the one miss explained and
   its replacement caught by the volume check); the browser specs E1 to E3 (the drawn scene equals the built map; the
   refused bundles leave the empty floor and its counts; a broken Worker leaves the content "none"); what is not
   proven (the greys are checked "never darker", not strictly lighter: the yard's 0.55 m and 0.56 m steps share a grey).
6. **art-director, on a narrow brief** (the advisor agent's item 4): only whether the greybox reads (heights told apart
   from the overview, sides against tops, the street shown or not by STREET-GREY, the readout's rows readable) and his
   Brave checklist (the sheet's section G, step S's six steps). No judgement against ART_DIRECTION (the old game's) or
   LOOK_V2: the greybox decides nothing of the look.
