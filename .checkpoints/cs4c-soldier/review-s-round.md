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

## engine-architect: APPROVE (the same Medium as GE-F1, found on its own)
| Id | Severity | Finding | Disposition |
|---|---|---|---|
| EA-1 | Medium | = GE-F1, measured on its own: at aspect 0 and 0.0001 the fit never returned (killed after 10 to 30 s); at 16:9, 1:1, 9:16, 0.2 and 0.05 on the widest map the loader allows it returns in 2 ms. "It belongs in this round's fix pass, not a STATUS follow-up: once the scene's head is frozen for Sunday, scene changes go into the soldier's pull request." | **Fix** (with GE-F1) |
| EA-2 | Low | = G-3, confirmed: `scene.test.ts:21` builds its map at y 0 and `:149` checks the literal 0, while `scene.ts:197` draws at `bounds.min[1]`; a planted `floorY = 0` passes. | **Fix** (with GE-F5) |
| EA-3 | Low | `readMap`'s three word lists (`content.ts:182`, `:188`, `:196`) are not tied to the generated types: if map.rs gains a word, readMap still compiles, then refuses every map using it with NOT_THE_MAP. | **Fix:** tie the three lists to the Rust source in `content-ties.test.mjs`, with a plant |
| EA-4 | Low | The inventory's kind list is untyped (`inventory.ts:13`, casts at `:31`, `:32`), so a new kind can cross the interface outside `SceneObject.kind` (`picture.ts:26`). | **Fix:** type the list as `SceneObject["kind"]` |
| EA-5 | Low | When the page itself refuses the content, the readout says "not read, the match stopped first" (`boot.ts:166`, `report.ts:79`), which contradicts the NOT_THE_MAP notice. | **Fix** if the wording change is small; else follow-up |
| EA-6 | Low | `scene.ts:185` and `scene.test.ts:135` still call STREET-GREY (b) a default; he answered (b) on 2026-11-28. | **Fix:** the comments |
Checked correct by it: the boundary as designed; `readMap` member for member against map.rs; `LOCAL_MAP` without
recording.ts; the production page's `inventory(){return null}` and no development marker; 4B-TYPES-READER,
4B-MEMBER-KINDS and 4B-BOOT-CALLS met; nothing per frame; nothing no VISION §6 item needs. Not run: a browser.
