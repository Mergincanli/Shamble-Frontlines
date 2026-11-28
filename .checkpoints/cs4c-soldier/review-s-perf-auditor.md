# perf-auditor: step S review (the page's scene), code head c1de41e5 on main dae6d107

## Plan
1. Per-frame path: does anything new run or allocate in `draw()` / the loop? (read three-picture.ts, boot.ts, report.ts)
2. Scene built once per delivered map (read content.ts's `PageContent`); old floor freed (geometries and materials counted before/after a swap, Node harness on the real scene.ts).
3. Draw calls: the test scene's 32 rechecked by arithmetic against the map; the worst map the loader allows (512 shapes, 64 regions, 655 m bounds) built by a harness, objects counted, against ADR 0003's draft merged-world row (about 202).
4. The frame on resize: `overview()` cost and allocation per call (Node harness, many calls); edge cases (aspect 0) for termination.
5. The counts' order script: read; re-run on the existing build.
6. Allocation per frame: the allocation spec re-run 3 times on base and head; the whole page's sampling profile by function.

## Notes (one line per finding, as found)
- packages under c1de41e5 = worktree HEAD 28690f1c (git diff --stat empty for packages crates content). c1de41e5 touches only scene.test.ts (1 file, +11); the build .gate/dist-development-4c (00:32) holds bc7720f3's page code; .gate/dist-before-4c is main's development build.
- test-scene: 23 shapes (6 ramps), 6 regions, bounds 112 x 48 m. 23 + 6 + ground + grid = 31 objects; the counts say 32 draws: the empty floor's 2 objects also count 3, so +1 is a constant of the renderer (17*12 + 6*8 + 6*2 + 2 + 1 = 267 triangles; lines 113 + 49 = 162). The committed counts match the map exactly.
- Per frame: `draw()` is unchanged in code (`renderer.render`), but the same call now walks 31 objects instead of 2 (see the allocation rows). The inventory runs only in setContent (once, development) and `__shambleScene` on call; the readout's new rows run every READOUT_EVERY_MS, not per frame.
- Built once per map: `PageContent.afterMessage` (content.ts:301-314) sets `#kept = null` before `#deliver`, and every later message returns at line 302: one delivery, so one `setContent`, per match (by reading).
- Harness `.checkpoints/cs4c-soldier/perf-s-harness.mjs` (the real scene.ts in Node 22.23, no renderer): test-scene 31 objects (30 meshes, 1 grid), 14 materials, 266 triangles, 162 line segments.
- Worst map the loader allows (512 shapes, 64 regions, bounds +-327.67 m, many distinct tops): 578 objects, 109 materials, 5,762 triangles, 1,310 grid segments; build 10 to 22 ms (7 runs, median 17.6 ms, Node). Draw calls about 579, inferred from the test scene's 31 objects -> 32 draws (no renderer in the harness). Finding P-2.
- disposeFloor on the worst map: 578 of 578 geometries and 109 of 109 materials disposed (dispose events counted).
- overview(): 20.5 to 22.9 us a call, roughly 3 to 5 KB allocated a call (heap growth over 2,000 calls before gc, approximate): per resize event only.
- overview(bounds, 0) in Node: never returns (killed by `timeout 20`, exit 124): `while (!fits(far)) far *= 2` (scene.ts:286) has no bound. (NaN also loops, but the page cannot pass NaN: the height is clamped.) Finding P-1.
- Browser check (`perf-s-zero-width.mjs`, .gate/dist-development-4c, container Chromium, WebGL 2 fallback): innerWidth forced to 0 through a getter, then a resize event: the page never answers again (page.evaluate TIMEOUT after 15 s); control with innerWidth 1: answers (65 frames). Playwright's setViewportSize width 0 left innerWidth at 640, so a truly 0-wide window was not produced: the reach in Brave is not shown.
- counts-order-4c.mjs re-run by me on the same build: late map (ready at 4404 ms, after the first frame at 4110 ms) and late frames (ready at 111 ms, first frame at 10099 ms): both 32/267/162/32/3/6, errors 0. geometries 32 = 31 + 1, so the empty floor's 2 were freed on the GPU side too. programs stays 6 on the WebGL 2 fallback.
- Allocation spec (allocation.spec.ts) x3 each, container Chromium, software WebGL: hand-off code base 87/94/92 B a frame, head 91/98/91: unchanged. Whole page: base 41,783 / 41,676 / 41,765 B a frame (86-87 frames in 8 s), head 46,307 / 45,841 / 45,395 (85-86 frames): +4.1 KB a frame (+9.8 %), spread about 0.1 KB (base) and 0.9 KB (head): a real rise, outside the noise.
- Sampling profile by function (`perf-s-alloc-top.mjs`, CDP HeapProfiler, 512 B interval, minor and major GC included, 8 s): base 41,652 B a frame, head 44,074 (+2.4 KB); the top allocators are three's per-object render path in both (base: multiplyMatrices 7.5 KB, invert 5.3 KB, applyMatrix4 4.5 KB; head: _bindUniforms 9.7 KB, invert 6.3 KB, needsGeometryUpdate 2.6 KB), none in the change's own code. About 85 to 140 B per drawn object per frame on this software path; part of P-2.
- Size: development main chunk 832,897 -> 840,241 B raw (+7,344), 223,837 -> 226,486 B gzip -9 (+2,649). Production head 832,767 raw / 224,572 gzip / 182,282 brotli (build-production-4c.log). Wasm unchanged: 257,421 B, the same content hash.

## Verdict: REVISE (scoped to P-1 only; P-2 is a follow-up)

### P-1, Medium: a 0-wide window hangs the page for good (new in this diff)
- Where: `packages/client/src/render/scene.ts:286` (`while (!fits(far)) far *= 2;`, no bound) reached from `packages/client/src/render/three-picture.ts:100-102` (`camera.aspect = width / Math.max(1, height)` clamps the height only, then `frame()`) and from `setContent` (three-picture.ts:77) after an earlier 0-wide resize. Main's `resize` (dae6d107 three-picture.ts:68-73) never called `overview`, so this is new.
- Failure case: `window.innerWidth` = 0 -> aspect 0 -> every projected corner is non-finite -> `fits` is never true -> the main thread spins forever (the frame loop, the readout and input all stop; only closing the tab ends it).
- Numbers and method: Node harness on the real scene.ts: `overview(test-scene bounds, 0)` killed by a 20 s timeout. Built page in Chromium: innerWidth forced to 0 by a getter plus a resize event: no answer in 15 s; innerWidth 1: answers. A truly 0-wide Brave window was not produced.
- Fix (one line, either or both): clamp the width as the height is, `Math.max(1, width) / Math.max(1, height)` at three-picture.ts:100; and bound the doubling at scene.ts:286 (for example at most 64 doublings, then keep the last distance). A unit test: `overview(bounds, 0)` returns finite numbers.

### P-2, Low (STATUS follow-up; overlaps graphics-engineer's part): one draw per shape does not scale to the largest map the loader allows
- Where: `packages/client/src/render/scene.ts:211-218` (one Mesh per region and per shape, one material per grey).
- Numbers: the test scene 32 draws (measured, both orders). The worst map the loader allows (crates/content/src/map.rs:30-31: 512 shapes, 64 regions) gives 578 objects and 109 materials, so about 579 draws (inferred from 31 -> 32). The draft ADR 0003 (not yet accepted) gives the merged world about 202 draws (docs/adr/0003-rendering.md:358, 1186: 4 x 48 x 2 + 202 + 200 = 786 under the Low cap of 800): the world's headroom under 800 with 48 soldiers is 216, so such a map would break the draft Low row once soldiers arrive. Each drawn object also allocates about 85 to 140 B a frame in three's render path on this software renderer (whole page +2.4 to +4.1 KB a frame for +29 objects), so a 578-object map would add roughly 50 to 80 KB a frame there.
- Not a defect today: no map exists beyond the test scene. Follow-up: merge the static floor by material (ADR 0003 part 6) before maps grow past about 200 objects, keeping the inventory's per-object read (for example from merged groups' ranges).

### Table
| metric | before (dae6d107) | after (c1de41e5) | method | noise |
|---|---|---|---|---|
| draw calls, test scene | 3 (empty floor) | 32 | renderer.info, counts-order-4c.mjs, both orders, Chromium WebGL 2 fallback | counts repeat (2 of 2 identical) |
| triangles / lines / geometries / programs | 3 / 62 / 3 / 6 | 267 / 162 / 32 / 6 | same | same |
| draw calls, worst loader map | - | about 579 (inferred) | harness object count + the measured +1 | none (a count) |
| hand-off allocation, B a frame | 87 / 94 / 92 | 91 / 98 / 91 | allocation.spec.ts x3 | about +-4 B |
| whole page allocation, B a frame | 41,676 to 41,783 | 45,395 to 46,307 | allocation.spec.ts x3 (sampling profiler) | 0.1 KB base, 0.9 KB head |
| whole page, by function, B a frame | 41,652 | 44,074 | perf-s-alloc-top.mjs x1 each | one run each |
| floor build, worst map | - | 10 to 22 ms (median 17.6) | Node harness, 7 runs | wide (JIT warm-up) |
| overview() per resize | - | 21 to 23 us, about 3 to 5 KB | Node harness, 2,000 calls | approximate |
| development main chunk | 832,897 B raw / 223,837 gzip | 840,241 / 226,486 | wc, gzip -9 | exact |
| Wasm | 257,421 B | 257,421 B | file size, same hash | exact |

### Brave-side confirmation needed
1. Does Brave on Windows ever report `innerWidth` 0 (window minimized, a background or restored tab, DevTools docked and dragged to the full width)? P-1's reach.
2. The test scene's frame time and draw calls on the real GPU from the DEV readout (WebGPU), so the WebGPU counts row gets its numbers (`render-counts.json` holds null for WebGPU).
3. Whether the whole page's per-frame allocation rise (+2.4 to +4.1 KB a frame on the software renderer) holds on WebGPU at 60 fps (Chrome's Memory panel, an allocation sampling profile of 10 s).

### Not verified
- The base production build's size (only main's development build was at hand).
- The one-off frame when the map arrives (the build plus the first render of 30 new objects: the hitch length).
- The WebGPU backend's counts and programs (the container has no WebGPU adapter).
- Real-GPU frame times (headless software WebGL only).

### Baselines (2026-11-28, c1de41e5 page code, container Chromium software WebGL 2): test scene 32 draws / 267 triangles / 162 lines / 32 geometries / 3 textures / 6 programs; whole page about 45.8 KB a frame (3 runs) against main's 41.7; hand-off code about 93 B a frame; development main chunk 840,241 B raw, 226,486 B gzip; production main chunk 832,767 / 224,572 / 182,282 (raw, gzip, brotli); Wasm 257,421 B. No agent-memory folder exists for perf-auditor, so these are recorded here only.

Scratch files: `.checkpoints/cs4c-soldier/perf-s-harness.mjs`, `perf-s-zero-width.mjs`, `perf-s-alloc-top.mjs`; outputs under `.gate/perf-s-pw-out/` and a stray `.gate/perf-s-zero-width-head.txt` (a copy of 40 lines of the script, safe to delete).

DONE
