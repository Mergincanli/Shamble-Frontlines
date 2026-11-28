# Step S's review round (4c-1, the page's scene): every finding and what is done with it

_The main session's tally (2026-11-28). One round, six reviewers, three at a time, then one fix pass (the review
budget). Reviewed: `git diff dae6d107 c1de41e5 -- packages crates content` (bc7720f3 + c1de41e5). Each review file is
`review-s-<agent>.md` beside this one._

## graphics-engineer: REVISE (one Medium)
| Id | Severity | Finding | Disposition |
|---|---|---|---|
| GE-F1 | Medium | The overview's search for a camera distance never ends when the window's aspect is 0 or NaN (`scene.ts:286` doubles `far` to Infinity); on the largest map the loader allows it never ends at aspect 0.001 either (a doubling passes the probe camera's far of 1e6 m). `boot.ts:113` passes `window.innerWidth` through `three-picture.ts:100-102`, and every `setContent` (`:77`) reaches the loop: a window 0 wide freezes the page. | **Fix:** cap the doubling; leave the camera as it is when the aspect is not a positive finite number; tests at aspect 0, NaN and the largest map at 0.001, each with its plant |
| GE-F2 | Low | `near` stays 0.1 while `far` follows the distance: on the largest map a 24-bit depth step is 0.3 to 1.4 m, so the grid shows through blocks under about half a metre (the test scene's step: 8 mm). | Follow-up (STATUS), for the first large map |
| GE-F3 | Low | One mesh per solid: 579 draw calls at the loader's caps, 2.9 times the world's share of 202 in ADR 0003 (:358); with a shadow pass past the Low cap of 800. Merging would break the inventory's rule of one buffer per object. | Follow-up (STATUS), with the look's change-set |
| GE-F4 | Low | The first frame after `setContent` builds two new WebGPU pipelines (one per new depth offset): a one-time hitch at match start; `compileAsync` before the swap would avoid it. Not measured (no WebGPU adapter here). | Follow-up (STATUS) |
| GE-F5 | Low | The enclosed-volume test cannot see faces whose plane passes through the origin, and every floor of its test map lies on such a plane: a wrong floor that keeps the vertex count passes (checked). | **Fix**, with G-3: the raised map (bounds above 0) moves every floor off the origin's planes |
| G-3 | Low | (engine-architect's block 5; concurred) `scene.test.ts:149` checks y = 0, so a scene drawing its ground at 0 instead of `bounds.min[1]` would pass. | **Fix:** one test on a map raised above 0 (with GE-F5) |
Checked correct by it: the f32 corners as the simulation's `as f32`, the ramps' corners in all four directions, faces
outward, the normals, the depth offsets on both backends, the grid, the old floor freed, 32 draws / 267 triangles /
162 lines. Unverified: the WebGPU counts (no adapter).
