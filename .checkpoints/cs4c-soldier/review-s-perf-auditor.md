# perf-auditor: step S review (the page's scene), code head c1de41e5 on main dae6d107

## Plan
1. Per-frame path: does anything new run or allocate in `draw()` / the loop? (read three-picture.ts, boot.ts, report.ts)
2. Scene built once per delivered map; old floor freed (geometries and materials counted before/after a swap, Node harness on the real scene.ts).
3. Draw calls: the test scene's 32 rechecked by arithmetic against the map; the worst map the loader allows (512 shapes, 64 regions, 655 m bounds) built by a harness, meshes counted, against ADR 0003's merged-world row (~202).
4. The frame on resize: `overview()` cost and allocation per call (Node harness, many calls); edge cases (aspect 0, huge bounds) for termination.
5. The counts' order script: read; re-run on the existing build.
6. Allocation per frame: the hand-off spec's logs read; the draw path checked unchanged.

## Notes (one line per finding, as found)
- packages under c1de41e5 = worktree HEAD 28690f1c (git diff --stat empty for packages crates content).
- test-scene: 23 shapes (6 ramps), 6 regions, bounds 112 x 48 m. 23 + 6 + ground + grid = 31 objects; counts say 32 draws: the empty floor's 2 objects also count 3, so +1 is a constant of the renderer (one triangle: 17*12 + 6*8 + 6*2 + 2 + 1 = 267 triangles; lines 113 + 49 = 162). The committed counts match the map exactly.
- Per frame: `draw()` is unchanged (`renderer.render`); inventory runs only in setContent (once, dev) and `__shambleScene` on call; the readout's new rows run every READOUT_EVERY_MS, not per frame.
- Harness `.checkpoints/cs4c-soldier/perf-s-harness.mjs` (real scene.ts in Node 22.23, no renderer): test-scene 31 objects (30 meshes, 1 grid), 14 materials, 266 triangles, 162 line segments: matches the committed 32/267/162 (+1 renderer constant).
- Worst map the loader allows (512 shapes, 64 regions, bounds +-327.67 m, many distinct tops): 578 objects -> about 579 draw calls, 109 materials, 5,762 triangles, 1,310 grid segments; build 10 to 22 ms (7 runs, median 17.6 ms, Node). Built once per map: fine. Draw calls: 579 against ADR 0003 part 7's merged-world share (about 202, docs/adr/0003-rendering.md:358) and the Low cap 800 with 384 soldier draws: 579 + 384 = 963 > 800. Finding P-2.
- disposeFloor on the worst map: 578 of 578 geometries and 109 of 109 materials disposed (dispose events counted). A swap frees all.
- overview(): 20.5 to 22.9 us a call, about 2.8 to 4.6 KB allocated a call (heap growth over 2,000 calls, before gc): per resize event only, not per frame: fine.
- overview(bounds, 0) and overview(bounds, NaN) in Node: never return (killed by `timeout 20`, exit 124): `while (!fits(far)) far *= 2` (scene.ts:286) has no bound. Reached from resize when `window.innerWidth` is 0 (three-picture.ts:100 guards the height only). Finding P-1.
- c1de41e5 touches only scene.test.ts (1 file, +11); the build in .gate/dist-development-4c (00:32) holds bc7720f3's page code.
- Browser check (`perf-s-zero-width.mjs`, the built development page .gate/dist-development-4c, container Chromium, WebGL 2 fallback): innerWidth read as 0, then a resize event: the page never answers again (page.evaluate TIMEOUT after 15 s); control with innerWidth 1: answers (65 frames). Playwright's setViewportSize width 0 left innerWidth at 640, so a truly 0-wide window was not produced here: the reach in Brave is not shown. P-1 = Medium.
- counts-order-4c.mjs re-run by me on the same build: late map (ready at 4404 ms, after the first frame at 4110 ms) and late frames (ready at 111 ms, first frame at 10099 ms): both 32/267/162/32/3/6, errors 0. programs stays 6 (as the empty floor): the 14 greys share the pipelines. geometries 32 = 31 + 1, so the empty floor's 2 were freed on the GPU side too.
- Allocation per frame (the hand-off spec, logs .gate/allocation-base-4c.log and allocation-head-4c.log, not re-run by me): 100 -> 85 B a frame; the whole page 3,098,472 -> 3,139,732 B sampled in about 8.1 s (+1.3 %): within the sampling profiler's noise; draw() unchanged, so no new per-frame work exists to allocate.
- Size: development main chunk 832,897 -> 840,241 B raw (+7,344), 223,837 -> 226,486 B gzip -9 (+2,649) (dist-before-4c vs dist-development-4c, my gzip). Production head 832,767 raw / 224,572 gzip / 182,282 brotli (build-production-4c.log); a base production build was not measured by me. Wasm unchanged: 257,421 B, the same content hash in both builds.
