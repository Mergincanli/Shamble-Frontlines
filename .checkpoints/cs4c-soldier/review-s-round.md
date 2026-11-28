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
## perf-auditor: REVISE (only for P-1)
| Id | Severity | Finding | Disposition |
|---|---|---|---|
| P-1 | Medium | = GE-F1 and EA-1, measured on the built page: in Node `overview(bounds, 0)` was killed by a 20 s timeout; in Chromium, innerWidth forced to 0 with a resize gave no answer in 15 s, innerWidth 1 answered. Reached from resize (`three-picture.ts:100` clamps only the height) and from `setContent` after an earlier 0-wide resize. Whether Brave ever reports innerWidth 0: unshown. | **Fix** (with GE-F1): `Math.max(1, width)` at the resize as well as the cap and the guard in `overview`; a test that `overview(bounds, 0)` gives finite numbers or none |
| P-2 | Low | One object per shape and per region: the loader's largest map (512 shapes, 64 regions) gives 578 objects, 109 materials, about 579 draw calls (inferred from the test scene's 31 objects and 32 draws, not measured); ADR 0003's draft leaves about 216 under the Low cap of 800 once 48 soldiers are drawn; no such map exists yet. | Follow-up (STATUS), with GE-F3: merge the floor by material before maps grow |
Measured by it: each drawn object costs about 85 to 140 B a frame inside three.js; the whole page +4.1 KB a frame with
the test scene (base 41.7 KB, head 45.4 to 46.3 KB, 3 runs each), all in three's per-object render code, none in the
change's own code; the hand-off unchanged at about 93 B a frame; the camera fit about 22 µs a resize; each floor swap
frees 578 of 578 geometries and 109 of 109 materials; the counts 32 / 267 / 162 / 32 / 3 / 6 in both orders (its own
re-run); the WebAssembly file unchanged at 257,421 B; the development bundle +7,344 B raw, +2,649 B gzip. Not verified:
WebGPU counts, real-GPU frame times, the base production bundle's size, the one-time hitch when the map arrives.

## security-auditor: APPROVE (no security finding open)
| Id | Severity | Finding | Disposition |
|---|---|---|---|
| S-1 | Medium | = GE-F1 (not security; its probe copies the fit's maths: never ends at aspect 0 or NaN); a page opened at width 0 freezes its tab at the Worker's `ready`. | **Fix** (with GE-F1): clamp the width, the fit refuses a bad aspect and caps its steps, a test at aspect 0 |
| S-2 | Low | No test feeds the map reader an own `__proto__` key (`content.test.ts:251-269`); a later copy through `Object.assign` before the key check would let an extra key through unseen. | **Fix:** a test with an own `__proto__` key (as `JSON.parse` makes one), refused by the closed key set, with its plant |
| S-note | Low (docs) | The pointer at `docs/LOOP_V2.md:16` sits mid-sentence: the line holds the end of the centre-field sentence ("centre.") and then two more sentences, the second running on to line 17, so the line's end cuts "Tech lifts the / whole team" (the advisor agent's F-S0-2: no sentence cut). | **Fix:** `place-4c.mjs fix-16` (tested on a scratch copy; `loop-check-4c.mjs` now holds the pointer right after "centre."), run by docs-keeper in its docs pass (his item 2: docs-keeper places his text) |
Checked correct by it: map text reaches the page only through `textContent` (overlay.ts:36, boot.ts:36); every map
object has exactly its listed keys, read through `Object.hasOwn` (content.ts:86, 118, 155-203); unknown folders,
formats and versions refused (content.ts:210-238), each refusal a fixed sentence quoting nothing it read;
`window.__shambleScene` and the inventory in the development build only (boot.ts:124, three-picture.ts:79-81), the
production scan failing on either marker; the docs after the code head carry no secret, key, board address or
personal detail beyond his own messages copied whole.

Checked correct by engine-architect: the boundary as designed; `readMap` member for member against map.rs; `LOCAL_MAP` without
recording.ts; the production page's `inventory(){return null}` and no development marker; 4B-TYPES-READER,
4B-MEMBER-KINDS and 4B-BOOT-CALLS met; nothing per frame; nothing no VISION §6 item needs. Not run: a browser.
