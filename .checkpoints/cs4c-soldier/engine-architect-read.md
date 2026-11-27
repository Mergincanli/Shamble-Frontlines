# engine-architect: 4c design read against main dae6d107 (re-read for the sheet)

Started 2026-11-27 (clock date of the session). Read only; this file is the only one I write.

## Plan
1. Read the plan's section 6 (6.1 to 6.3), 9, 12, 13, 16, 20; my design's blocks 2 to 4 and 4.1 to 4.10; 4b's report;
   STATUS_ARCHIVE 2278 to 2300; the advisor's B3 Q2 ruling at 59f6cc7; his question file.
2. Read today's page code: content.ts, boot.ts, picture.ts, render/*, overlay/report.ts, generated content types,
   e2e content.spec.ts, skeleton.spec.ts, render-counts.json, build-checks.mjs, test-scene.json, map.rs, collision.rs.
3. Block 1: the scene step in full (save at once).
4. Block 2: SIZE-4C option (c), the seam.
5. Block 3: steps 3 (layout parts), 4, 5 against today's code; TUNE-HOME and RESUME-FLASH by option.
6. Block 4: 4B-WORLD-ROOM.
7. DONE line, then the hand-back summary.

## Notes (one line per finding)
- N1. picture.ts:44-64: `Picture` has start, loop, draw, counters, resize, stop; no setContent. three-picture.ts:112 builds the empty scene at construction; draw (:128-131) renders it. empty-scene.ts:14 GROUND_M 30 m (the map's bounds are x -32..80, z -24..24: 112 x 48 m), ground colour 0x6b7480 (a blue grey, not neutral), camera fixed at (0,14,16) fov 50 (:46-48).
- N2. content.ts:62-82 readBundle gives path -> text (own fields, Map); :123-156 PageContent reads once on ready and keeps nothing; :103-108 takeWorkerMessage; boot.ts:217-256 wires it. No reader of the generated content types anywhere in packages/client (grep): 4B-TYPES-READER stands.
- N3. The bundle's path rule is `<folder>/<id>.json` (bundle.rs:9-10, :110-116), so the local map's file is `maps/${LOCAL_MAP}.json`; LOCAL_MAP is in sim-worker/src/recording.ts:12 (tied to sim-wasm/src/lib.rs:25 by a boundary test) but not exported by sim-worker/src/index.ts:4-7.
- N4. Format constants: map.rs:24-25 shamble-map 1; movement.rs:13-14 shamble-movement 1; bundle.rs:26-27, :31 FOLDERS maps, movement. content-ties.test.mjs:8-15 ties the bundle's two by regex on the Rust source: the same pattern extends to the map's and the movement's.
- N5. The ramp's wedge (collision.rs:152-200; the library twin test :886-918 for +x): the four floor corners at min[1] plus the two corners at max[1] on the side `rises_toward` names. For +x: (x0,y0,z0) (x1,y0,z0) (x1,y1,z0) (x0,y0,z1) (x1,y0,z1) (x1,y1,z1).
- N6. collision.rs:103-108 `narrow`: corners are f64 -> f32 with -0 made +0. A Float32Array of the same doubles gives the same bits (round to nearest even both sides), so the page's drawn corners can be asserted equal to Math.fround(map value): the scene's corners are the simulation's corners bit for bit.
- N7. three/webgpu 0.187 imports in plain Node (measured here: BoxGeometry, a dispose listener, MeshStandardNodeMaterial colour 808080 all work, no renderer): scene.ts can be unit-tested in vitest without a browser. No client unit test imports three today (grep).
- N8. Precedent for a production interface method read only by development code: `counters()` (picture.ts:59, three-picture.ts:132-142, read only at boot.ts:281 inside the dev block). Precedent for a statically imported module cut by the define: `FrameTimes` (boot.ts:158, :258); DEV_MARKERS (build-checks.mjs:202) has no marker for it, so only the five listed strings are proven cut.
- N9. render-counts.json: WebGL 2 fallback 3 draw calls, 3 triangles, 62 lines (31 + 31 grid lines of the 30 m ground), 3 geometries, 3 textures, 6 programs; WebGPU null. skeleton.spec.ts:45 compares counters on a running page; content.spec.ts:198-200 polls the same baseline for the refused-bundle cases ("the scene stays empty").
- N10. The test scene's tops: 0.55, 0.56, 1, 1.5, 2.5, 2.6, 2.9, 3.05, 3.1, 4.3, 4.7 m; six regions tile the whole bounds at y 0 (street and grass); GREY-COLOURS (a) (DECISIONS.md:310-318) keeps the default of plan section 13 item 5: LOOP_V2.md:158 in neutral greys. Two ground kinds at one height get one grey by that rule: the street would not show. A look point, not mine: named for his check.
