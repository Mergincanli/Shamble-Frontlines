# Step S review: graphics-engineer (4c-1, the page's scene), code head c1de41e5 on main dae6d107

## Plan
1. Read `render/scene.ts`, `render/three-picture.ts`, `render/inventory.ts`, `picture.ts` at c1de41e5, and the deleted
   `empty-scene.ts`.
2. Geometry: f32 corners against the sim's, the ramp's corner indices and faces for all four `rises_toward`, winding by
   the centroid test, per-face normals, the floor quads' winding.
3. Materials and the two backends: node materials, polygon offset (WebGL state vs WebGPU pipeline depth bias), program
   and pipeline counts, compile on the first frame after `setContent`.
4. Grid, depth precision at the largest map the loader allows, the overview's fit by halving (termination, edge
   aspects).
5. Disposal on a new map (`disposeFloor`) and on `stop`.
6. Bounds of the counts at the loader's caps (`crates/content/src/map.rs:27-31`: 16 spawns, 512 shapes, 64 regions;
   bounds up to 655.34 m a side).
7. Small Node scripts under `.gate/` only where a claim needs a number.

## Notes (one line per finding, as found)
- N1 (geometry, no finding): corners go into `Float32BufferAttribute` untransformed, so each is the f64 rounded to f32 by
  round-to-nearest-even, the same as the sim's `value as f32` (`crates/sim/src/collision.rs:103-108`; its minus-zero
  rule cannot change a drawn position). Ramp corner indices checked by hand for all four `rises_toward`
  (`scene.ts` `solidFaces`): a, b under tops 4, 5 and c, d beside them are right in every case; each face list is a
  proper cycle. The centroid test is valid because a box and a wedge are convex. The floor quads wind to +y (front
  from above). The slope is the sim's plane (`collision.rs:149-177`).
- N2 (materials and backends, no finding): node materials only; one material per grey does not multiply the programs,
  by measurement: the test scene's 23 solids take about ten greys and the WebGL 2 count stays 6 (render-counts.json);
  WebGPU unverified (no adapter in the container). Polygon offset reaches WebGPU as pipeline depth bias (`.../webgpu/utils/WebGPUPipelineUtils.js:238-242`) and is in
  the pipeline key (`.../webgpu/WebGPUBackend.js:2500`): three offsets, three pipelines; on WebGL 2 it is plain state.
  Ground pushed 2, patches 1, grid 0: the order is right on both. Test-scene counts add up: 1 ground + 6 patches + 23
  solids + 1 grid + the output quad = 32 draws; 17x12 + 6x8 + 12 + 2 + 1 = 267 triangles; 113 + 49 = 162 lines.
- F1 (Medium): `overview` never returns when the aspect is 0 or NaN, or so small that the fit needs a camera past its own
  far of 1e6 m. Measured with a copy on three's core camera, a capped loop (`.gate/ge-review/overview-aspect.mjs`):
  aspect 0 and NaN hang on both the test-size and the largest map (`scene.ts:286` doubles `far` to Infinity and never
  fits); at aspect 0.001 the largest map (655 m) also hangs, because a doubling overshoots past the probe camera's far
  of 1e6 m and no larger distance passes `projected.z < 1` again. The fix is the cap, not a larger far. Failure case: `boot.ts:113` passes `window.innerWidth`; a
  width of 0 (a page in a zero-size frame, or a window that reports 0 while minimised: not seen in Brave, unverified)
  makes `resize` (`three-picture.ts:100-102`) and every later `setContent` (`:77`) freeze the main thread for good.
  Fix: cap the doubling (64 steps) and leave the camera as it was when the aspect is not a positive finite number;
  one test at aspect 0.
- F2 (Low): depth precision on large maps. `near` stays 0.1 (`scene.ts:68`) while `far` follows the distance
  (`three-picture.ts:51`). The overview of the loader's largest map stands 750 m off at 16:9 and 1,010 m at 1:1; a
  24-bit depth step there is 0.34 to 0.61 m at the middle, up to about 1.4 m at the far corner (same script). Failure
  case: grid lines and patch edges bleed through blocks lower than about half a metre (the 0.55 m steps on a
  600 m map), and two touching solids flicker. The test scene's step is 8 mm: nothing shows today. Fix when a large map
  is drawn: near scaled to the distance (it must stay under the nearest corner), or the reversed depth buffer.
- F3 (Low, follow-up): draw calls at the loader's caps (`crates/content/src/map.rs:27-31`): one mesh per solid and
  patch gives 512 + 64 + 1 + 1 + 1 = 579 draws, at most 6,274 triangles, 1,310 grid lines, 579 geometries and 109
  materials (105 block greys, 0x70 to 0xd8, plus 3 ground greys and the grid), programs unchanged. Bounded, and under
  ADR 0003's Low cap of 800 (`docs/adr/0003-rendering.md:351`) on its own, but 2.9 times the "merged world about 202"
  that cap sets aside for the world (`:358`), and a shadow pass doubles it. Failure case: a full-size authored map
  with the soldiers and a shadow pass passes 800 at Low. Merging (one mesh per grey, or vertex colour) breaks the
  inventory's "own geometry buffer per object" (`picture.ts:19-31`), so the follow-up must change both: record it for
  the change-set that brings shadows or the real look.
- F4 (Low): the first frame after `setContent` creates two new WebGPU pipelines (polygon offset 0 and 2; the empty
  floor only had 1), synchronously inside `render` (`three-picture.ts:70-78`, then `draw`): a one-time hitch at match
  start. `await renderer.compileAsync(next, camera)` before the swap would move it off the frame. Not measured: the
  container has no WebGPU adapter. On WebGL 2 the program count is 6 before and after; whether programs are freed with
  the empty floor and rebuilt is unverified.
- F5 (Low): the enclosed-volume test (`scene.test.ts`, the second test) sums `a.(b x c)/6`, and a triangle whose plane
  passes through the origin adds 0 to it. In its made map every solid has min y 0 and the ramps min z 0, so the floor
  faces and the z = 0 faces of three ramps are not checked by the volume at all. Checked: a box with min at the origin
  gives the same sum with its floor right, with its floor as one triangle drawn twice, and with no floor
  (`node -e` script, this session). Failure case: a wrong floor triangulation that keeps the vertex count (36 or 24)
  and winds outward passes every assertion. Fix with G-3's raised map: one map whose solids have no face on a
  coordinate plane.
- N3 (disposal, no finding): `disposeFloor` frees each geometry and material of a flat group once; the new floor is
  built before the old is freed, so a throw in the build keeps the old one; the counts show the empty floor freed
  (geometries 32, not 34). `stop` is unchanged from main (renderer.dispose only).
- N4 (G-3, concur Low): the loader allows `bounds.min[1] > 0` (`map.rs:325`, lowest 0); the scene draws ground and grid
  at `bounds.min[1]`, consistent, but `scene.test.ts:149` checks the literal 0. One test on a raised map, as briefed.
- Not verified: the WebGPU path's counts (render-counts.json holds null for WebGPU), the pipeline hitch's size, and
  whether Brave on Windows ever reports an inner width of 0.

## Verdict: REVISE
One Medium to fix in the fix pass, the rest Low.
1. F1 Medium: `scene.ts:286`, the overview's doubling loop never ends at aspect 0, NaN, or a tiny aspect on a large
   map; `boot.ts:113` -> `three-picture.ts:100-102` and `:77` reach it with a zero window width and freeze the page.
   Cap the loop, skip framing on a non-positive or non-finite aspect, add a test at aspect 0.
2. F2 Low: `scene.ts:68` with `three-picture.ts:51`, near 0.1 against a far of up to about 4 km: 0.3 to 1.4 m depth
   steps on the largest map; grid bleeds through low blocks. Follow-up for the first large map.
3. F3 Low: `scene.ts:215-217`, one mesh per solid: 579 draws at the loader's caps (2.9 times ADR 0003's world share of
   202, `docs/adr/0003-rendering.md:358`); a full-size map with soldiers and a shadow pass passes the Low cap of 800.
   Merging needs the inventory contract changed (`picture.ts:19-31`). Follow-up.
4. F4 Low: two new WebGPU pipelines compile on the first frame after `setContent` (`three-picture.ts:70-78`): a
   one-time hitch at match start; `compileAsync` before the swap. Unmeasured; follow-up or defer.
5. F5 Low: `scene.test.ts`'s volume check cannot see faces on a coordinate plane (every floor in its made map): a
   wrong floor triangulation with the right count passes. Fix with G-3's raised map.
6. G-3 Low (concur): `scene.test.ts:149` checks y 0; a scene that drew the ground at 0 instead of `bounds.min[1]`
   would pass every test and leave a raised map's solids floating above the drawn ground.
Geometry, f32 corners, face orientation, normals, the two backends' polygon offset, the grid and disposal: correct.

DONE
