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
- N11. three.webgpu.js:6990-6994 (three 0.187, read in node_modules): `info.memory.geometries` goes up when the renderer first meets a geometry and down on its `dispose`. So a running page's geometries count proves the empty floor was disposed when the map's replaced it: a missed disposal leaves it 2 higher than the committed baseline.
- N12. `picture.stop()` has no caller anywhere (grep packages/client/src): disposal on stop is no live path; the swap of the empty floor for the map's in the first setContent is.
- N13. worker-link.ts:293 makes one DataView per block it takes (counted in the allocation spec's per-block bytes, allocation.spec.ts:18-26); the spec's per-frame limit sits below one planted 64-byte typed array per frame (allocation.spec.ts:5). A picture that makes a view per frame over the two blocks would fail it (step 4's rows; block 3 of this read).
- N14. The stamina bar (plan section 2 and step 4) has no field in the block: plan 6.2's soldier row and owner's section and my design 3.3 and 3.4 carry none; the wire carries it (netcode-architect-cs4.md:376). Block 3 of this read.

## Block 1. The scene step (4b's old step 12), in full (DONE)
Against main dae6d107. The sheet row it carries out: 4b's sheet at 59f6cc7, row 12 (quoted in the brief). The advisor
agent's conditions on the map's reading: `advisor-cutC.md` B3 Q2 (59f6cc7), lines 31 to 38.

### 1.1 `Picture.setContent` and `PictureContent` (picture.ts, the interface file)
- **Ruling:** `setContent(content: PictureContent): void` with `interface PictureContent { readonly map: MapDocument }`
  (`MapDocument` from `@shamble/types`, type import only). Called once per match, from the content's hand-off on the
  Worker's `ready` (1.3); the picture builds the map's floor from it, swaps it for what it drew before and keeps no
  reference to the document afterwards (the spawns are read, not drawn: 1.2).
- **How the camera joins without a second change of the method:** `PictureContent` is a record, so step 4 adds one
  field, `readonly camera: <the camera kind's generated type>`, with its reader (the follow camera) in the same step
  (the camera kind moves from step 5 to step 4: block 3, C6, a departure from the approved step rows).
  The method, its one caller and the hand-off stay as they are; only `readContent` reads one more file. That is the
  cheap route. Not taken: an optional `camera?` now (no type exists before the camera kind, and a field with no
  reader is what B3 Q2 condition 2 refuses), or `setContent(files)` with the picture reading the bundle itself (the
  reading would sit in `render/`, which passes to lane 2 in change-set 5, Part 15.2 :112-115, and couple the
  picture to the bundle's format).
- **The development inventory (1.4)** adds `Picture.inventory(): readonly SceneObject[] | null` and the record
  `SceneObject { id; kind: "ground" | "box" | "ramp" | "region"; points: [x, y, z][] (distinct, sorted); grey: number
  (the material's authored colour, 0xRRGGBB) }`; its sentence says a production build returns `null`.
  **Who keeps it, and until when** (advisor-tool point 3, taken): the method puts the greybox's shape (points, grey)
  into the interface file, and `render/` passes to lane 2 at change-set 5's merge (Part 15.2 :112-115). Its sentence
  says so in words: "lane 1's test hook for 'the scene equals the map'; whoever owns `render/` keeps it answering
  while that test stands; the greybox's `grey` is retired with the greybox, by the change-set that replaces it (lane
  2's look, change-set 6), which then changes this record and its test in the same pull request". So it is no
  hidden obligation: lane 2 reads it in the file it takes over.
- picture.ts:18-24's sentence about the blend ("it comes as a field of the block with the first thing that moves") is
  not touched here: it changes in step 3 (block 3 of this read), where the blend arrives.

### 1.2 The map's reading, field by field (content.ts; B3 Q2 conditions 1 and 2)
- **Where:** `packages/client/src/app/content.ts` (my design 4.5's place; the sheet's row 12). Not in `render/`
  (1.1) and not in `@shamble/types` (that package holds generated types and the layout check only:
  types/package.json:5).
- **Functions** (each returns `Outcome<T>`, content.ts:19; nothing cast, nothing spread, own properties only through
  `own`, content.ts:53-55):
  - `readMap(tree: unknown): Outcome<MapDocument>`: the top object's keys are exactly `format`, `version`, `id`,
    `bounds`, `spawns`, `shapes`, `regions`; `format === MAP_FORMAT`, `version === MAP_VERSION`; `bounds` exactly
    `min`, `max`, each three finite numbers; each spawn exactly `id` (string), `at` (three), `facing_deg` (finite);
    each shape exactly `id`, `min`, `max`, `type`, and `rises_toward` when and only when `type` is `"ramp"`, its word
    one of `"+x" "-x" "+z" "-z"`; each region exactly `id`, `kind` (`"street"` or `"grass"`), `min`, `max` (two each).
    Built by small private helpers: `closed(object, keys)` (own keys equal the list), `corner(value, n)` (an array of
    exactly n finite numbers: `JSON.parse` turns 1e400 into Infinity), `word(value, words)`, `text(value)`.
  - `readContent(kept: Uint8Array): Outcome<ReadContent>`: `readBundle` (content.ts:62-82, unchanged), then for each
    file its folder's kind from `CONTENT_KINDS` (`maps` -> `shamble-map`, newest 1; `movement` -> `shamble-movement`,
    newest 1; an unknown folder is a fault), its own `format` equal to the kind's and an integer `version` from 1 to
    the newest; then `readMap` on the file at `maps/${LOCAL_MAP}.json` (the bundle's path rule is `<folder>/<id>.json`,
    bundle.rs:9-10 and :110-116, so the path is the id; `LOCAL_MAP` exported by `@shamble/sim-worker`'s index, one
    line, sim-worker/src/index.ts:4-7, from recording.ts:12 where a boundary test already ties it to
    sim-wasm/src/lib.rs:25). Returns `ReadContent { picture: PictureContent; files: readonly ContentFile[] }` with
    `ContentFile { path; format; version }` for the readout (1.5). Each file is parsed once; the map's tree goes to
    `readMap`, not its text again.
- **What the page does not check:** ranges, grids, thickness, ids' grammar, "inside the bounds". Those are the
  loader's (map.rs:8-17, :303-411), already passed when `ready` came; a second copy in TypeScript would drift. The
  page reads the shape the generated type names, and a mismatch is a build fault.
- **A failure in fixed words:** two sentences, both constants beside `NOT_A_BUNDLE` (content.ts:29): `NOT_A_BUNDLE`
  for the envelope and the file headers, and a new `NOT_THE_MAP` ("the map the game's loader took does not read as a
  map on the page: the page and the Wasm are from different builds"). Never a key, a value or a parse error's text
  (advisor-cutC Q1 condition 5); nothing of a failed reading is kept; the match ends through `link.end` as today
  (content.ts:103-108); the scene stays the empty floor.
- **The constants and their ties:** `MAP_FORMAT`, `MAP_VERSION`, `MOVEMENT_FORMAT`, `MOVEMENT_VERSION` and the kinds'
  folders, each tied to map.rs:24-25, movement.rs:13-14 and bundle.rs:31 by content-ties.test.mjs's pattern
  (:8-15).

### 1.3 What `PageContent` keeps after `ready`, and how boot.ts hands it to the picture
- **Keeps nothing new.** On the first message after which the link is ready, `afterMessage` reads (content.ts:150-154
  as today), and on success calls a `deliver(read: ReadContent)` given to its constructor, once; then the copy is let
  go, as today. Constructor: `new PageContent(deliver, read = readContent)` (today `(read = readBundle)`,
  content.ts:128). Not taken: a `#content` field that boot polls each frame (a per-frame check and a field held for a
  reader that could be called directly).
- **boot.ts:** `new PageContent((read) => { picture.setContent(read.picture); if (__SHAMBLE_DEVTOOLS__) files =
  read.files; })` at boot.ts:217; nothing else in the order changes (picture start, fetch, Worker, `ready`,
  `setContent`, frames). `setContent` runs inside the message handler, once, between two frames.
- `takeWorkerMessage` and `takeWorkerError` (content.ts:103-115) keep their signatures.

### 1.4 The development hook that lists the drawn objects (the test "the scene equals the map")
- **The data comes from the scene, not from the map:** `render/inventory.ts` (new, development only) walks the floor
  group and reads each object's own geometry buffer (distinct points), its name (the shape's or region's id), and
  its material's colour. A test that reads a list the builder wrote beside the meshes would test the list.
- **How it reaches the test with only `createPicture` importable from `render/`** (picture.ts:66-70): through the
  interface, `Picture.inventory()` (1.1). three-picture.ts implements it as `__SHAMBLE_DEVTOOLS__ ?
  readInventory(floor) : null` with `readInventory` imported statically: the precedent is `FrameTimes` (boot.ts:158
  and :258), cut by the literal `false` and tree-shaking. boot.ts's existing development block (boot.ts:270-326)
  installs `window.__shambleScene = () => picture.inventory()` beside `__shambleReport` (boot.ts:315). The app
  imports nothing new from `render/`.
- **No development path in the production file, proven:** `DEV_MARKERS` (build-checks.mjs:202) gains
  `"__shambleScene"` (boot's block) and the fixed sentence `readInventory` throws when the floor holds an object
  it does not know (`INVENTORY_UNKNOWN`, "the scene inventory met an object it does not know": a live string of
  inventory.ts's own code, so the development build keeps it, where an unreferenced tag constant would be dropped
  there too and the marker check would fail; advisor-tool point 5, taken), so the production scan (build-checks.mjs:242-245) refuses a production build holding either and a development
  build missing either; build-checks.test.mjs:243's fixture string gains both. The production file keeps only
  `inventory(){return null}`. (Seen while reading: `FrameTimes` has no marker today, so its cut is not proven; a Low
  for the sheet, not this step's.)

### 1.5 The readout's content rows (overlay/report.ts)
- `ReportData` (report.ts:6-53) gains `content: ContentRows` and `scene: { map: string; boxes: number; ramps: number;
  regions: number } | null` (the scene's counts, taken once at delivery: this section's last bullet). `ContentRows` is `"holding"` (the copy is held,
  no answer yet), `"none"` (the match ended before the loader's answer: the copy was let go unread) or the list of
  `ContentFile` (read).
- `readoutLines` (report.ts:85-99) gains two lines: `content: maps/test-scene.json shamble-map 1 · movement/soldier.json
  shamble-movement 1` (or `content: waiting for the loader` / `content: not read, the match stopped first`), and
  `scene: test-scene · 17 boxes · 6 ramps · 6 regions` (or `scene: empty`). `formatReport` (report.ts:102-158) gains
  `content.<path>: <format> <version>` per file (or `content: holding|none`) and `scene.map`, `scene.boxes`,
  `scene.ramps`, `scene.regions`. Text through `textContent` as today (overlay.ts:3, :36). Paths are the loader's own
  (`<folder>/<id>.json`, the id grammar), never free text.
- boot.ts's `data()` (boot.ts:276-312) fills them from `content.holding` (content.ts:133-135, today read by tests
  only) and two values the development hand-off sets **once, at delivery** (advisor-tool point 2, taken): the
  delivered `files`, and the scene row, counted from one `picture.inventory()` call right after `setContent`, with
  its `map` from the delivered document's `id` (a `SceneObject` carries no map id). `data()` never calls
  `inventory()`: the readout runs inside the frame callback (boot.ts:318-325 under :328-333), and a walk of the floor
  there would allocate in a frame. `__shambleScene` calls it only when a test asks.

### 1.6 `render-counts.json`
- **The empty scene's counts stay** as they are (WebGL 2 fallback 3, 3, 62, 3, 3, 6; WebGPU null): the empty floor is
  today's 30 m ground and grid, moved, not changed, and it is what a refused bundle leaves on screen.
- **The file's shape:** `{ "note", "empty": { "WebGL 2 fallback": {...}, "WebGPU": null }, "test-scene": { "WebGL 2
  fallback": <measured>, "WebGPU": null } }`. skeleton.spec.ts:45 compares a running page with `["test-scene"]`;
  content.spec.ts:198-200 (the refused bundles) with `["empty"]`; both read sites change (skeleton.spec.ts:6,
  content.spec.ts:121).
- **Method and evidence:** measured, never computed into the file: the build step runs skeleton.spec.ts in the
  container on the built development page (its line 16 prints the report with the counters), copies the WebGL 2
  fallback numbers in, then runs the spec three times green; the note names the date, the commit and "the container's
  Chromium, WebGL 2 fallback". WebGPU stays null (no adapter in the container, as the note says today).
- **What to expect** (arithmetic on the design, not a measurement; the measurement wins): each drawn object one draw
  call and one geometry, the map's ground and grid replacing the empty floor's two: draw calls 3 - 2 + 2 + 23 + 6 = 32;
  triangles 3 - 2 + 2 + 17 x 12 + 6 x 8 + 6 x 2 = 267; lines 113 + 49 = 162 (one per metre over x -32..80 and z
  -24..24); geometries 3 - 2 + 2 + 29 = 32; textures and programs as measured (whether the materials share programs is
  three's node cache's call).

### 1.7 What is drawn, and how boxes, ramps and regions take their greys
- **Our own geometry from the corners, for every solid:** a box is its 8 corners and 12 triangles, a ramp its 6
  corners and 8 triangles (the library twin's faces, collision.rs:908-917), written straight from the document's
  numbers into a `Float32Array` with no transform on the mesh. So every drawn corner is `Math.fround` of the map's
  number: the same bits as the simulation's `Solid` corners (`narrow`, collision.rs:103-108; both round to nearest
  even). Not `BoxGeometry` plus a translate, whose centre and half-size arithmetic can move a corner's last bit.
- **The ramp's six points, matching `enter_ramp`** (collision.rs:152-200, the slope through the floor edge on the side
  it rises from and the top edge on the side it rises toward): the four corners at `min[1]`, and the two corners at
  `max[1]` on the side `rises_toward` names (+x: x = max[0]; -x: x = min[0]; +z: z = max[2]; -z: z = min[2]). For +x
  these are exactly the twin test's points (collision.rs:900-907).
- **Regions:** a quad of their four corners at `bounds.min[1]`; **the ground:** a quad over the bounds' x and z at
  `bounds.min[1]`; **the grid:** one line per metre over the bounds (design 4.2: "the ground and the 1 m grid kept
  (they judge sizes)"). Depth order by polygon offset, as empty-scene.ts:28-33 does today: the ground pushed back
  most, the regions less, the lines on top; no corner moved. Spawns are not drawn (nothing in the scene step needs
  them; the soldier appears in step 4).
- **Greys** (GREY-COLOURS (a), DECISIONS.md:310-318: plan section 13 item 5's default, LOOP_V2.md:158 "plain blocks,
  with height read through brightness (higher is lighter)", neutral greys): one neutral grey (R = G = B) per solid by
  its top, `max[1]` (a box's and a ramp's alike), rising linearly from the map's lowest top to its tallest between two
  end greys that are named placeholder constants in scene.ts (empty-scene.ts:10-13's rule: "a plain placeholder that
  decides nothing"). Lit by cs2's sun and sky (empty-scene.ts:43-45) on node materials (ADR 0003 part 1), so a side
  face reads darker than a top: the tests check the authored colours, and his eyes check the screen.
- **Look points this does not decide** (his and the art-director's; named for the sheet, not chosen here): (1) the
  street and the grass are both at height 0, so by the height rule alone they take one grey and the street does not
  show, while plan section 2 says he sees "a street". This is a look choice, so it is **his question, not built in**
  (advisor-tool point 4, taken), STREET-GREY: (a) one grey for both ground kinds, darker than every block, so the
  street does not show apart until the look (change-set 6); (b) two neutral greys, one per kind, both darker than
  every block (the street shows); (c) the regions drawn as outlines in the grid's grey, no fill. A default he can
  veto, as GREY-COLOURS was: recommended (b), since plan section 2 promised him "a street" and it stays inside
  "neutral greys"; until he answers the build takes (b), and S3's assertion on the two kinds follows his letter.
  (2) The empty floor's ground is a blue grey (0x6b7480, empty-scene.ts:24), not
  neutral: the map's ground takes a neutral grey; the empty floor keeps its own (its counts and its look are the
  refused-bundle baseline).
- **The camera in the scene step:** the empty scene's camera (empty-scene.ts:47-49: fixed at (0, 14, 16), 50
  degrees, looking at the origin) shows the street corner only; the yard lies 25 to 80 m along x. So `setContent`
  also frames the map: an overview from the same side and angle as today's camera, far enough that the bounds' four
  floor corners lie in view at the canvas's aspect (recomputed in `resize`, three-picture.ts:143-148), outside the
  bounds, so no grid line runs behind the camera (empty-scene.ts:3-6's software-renderer note). It is replaced by the
  follow camera in step 4. Under SIZE-4C (a) it can be left out (1.13).

### 1.8 Disposal, and no allocation per frame
- **Disposal on the next `setContent`, live from the first call:** the picture holds one floor group; today's ground
  and grid become the empty floor (built at construction, as now, three-picture.ts:112); `setContent` builds the
  map's floor, removes the old one, and `disposeFloor` disposes each of its geometries and materials once. Materials
  are made per build (a small cache by grey inside one build), never module-level, so no disposal can reach another
  floor's ("never dispose pooled/shared materials"). `stop()` is unchanged: it has no caller (N12).
- **No allocation per frame:** `draw` (three-picture.ts:128-131) is unchanged; the floor is built once; the inventory
  runs only when a test asks, and once at delivery for the scene row (1.5): `data()` and the readout's rewrite
  (boot.ts:318-325) never call it. Evidence: allocation.spec.ts on base and head, its per-frame bytes
  unchanged within its stated noise (allocation.spec.ts:18-26).

### 1.9 The follow-ups placed
- **4B-TYPES-READER** (STATUS_ARCHIVE.md:2280-2281): met here. `readMap` is the first reader of `MapDocument`,
  `Bounds`, `Spawn`, `Shape`, `Toward`, `Region` and `Ground` (packages/types/src/index.ts:16-23), typed as returning
  `MapDocument` with no cast, so tsc holds the reader's output to the generated type. In the scene step `Movement` keeps no page
  reader (only its file's header is read, for the readout); its first page reader is step 4's `readMovement`, for
  the stand-in's height, if NO-SOURCE puts the height in the movement kind (block 3.6: raised, not ruled). Step 4's
  row in the plan (800 / 540) does not count it: about 20 / 30 more.
- **4B-MEMBER-KINDS** (STATUS_ARCHIVE.md:2292-2293): met here, twice. (1) The chain: types to reader by tsc, reader
  to loader by `readMap` on the committed `content/maps/test-scene.json` (and the built bundle in Chromium), loader
  to schema by the Rust test (schema.rs). (2) Directly: content-schema.test.mjs's compare (types:144-179) gains each
  member's kind (`number` against `type: number`, `string` against the id rule or `type: string`, a word union
  against its `enum`, a tuple against `prefixItems` of numbers).
- **4B-BOOT-CALLS** (STATUS_ARCHIVE.md:2294-2295): met by the browser specs that already load the page. The message
  half: the scene appears only through boot's `takeWorkerMessage` -> `afterMessage` -> `deliver` -> `setContent`, so
  skeleton.spec.ts's "the scene equals the map" fails if boot calls `link.receive` alone. The error half: the
  broken-Worker specs (skeleton.spec.ts:94-111, which already wait for `__shambleReport`, :107) assert the readout's
  `content` is `"none"`; if boot's error listener called `link.workerError` without `takeWorkerError`, the copy is
  still held and it reads `"holding"`.
- **4B-CHROMIUM-TIME** (STATUS_ARCHIVE.md:2286-2287; CI's Chromium run at 3.6 to 3.9 of 5 minutes): the scene step
  adds **no page load**. Its proofs sit in Node (content.test.ts, scene.test.ts with three's objects, which import in
  plain Node: N7) and in page loads that already happen (skeleton.spec.ts's two backends, the seven hostile bundles,
  the two broken Workers). The CI file is the CI Builder's; the risk is named for step 7's long specs (block 3).

### 1.10 The exact symbols
| | Symbols |
|---|---|
| **Add** | picture.ts: `PictureContent`, `Picture.setContent`, `SceneObject`, `Picture.inventory`. render/scene.ts (new): `buildEmptyScene` (moved; now returns `{ scene, camera, floor }`), `buildMapFloor(map)`, `solidPoints(shape)` (the 8 or 6 corners), `greyOf(top, lowest, tallest)`, `groundGrey(kind)`, `disposeFloor(floor)`, `overview(bounds, aspect)`. render/inventory.ts (new): `readInventory(floor)`, `INVENTORY_UNKNOWN` (its thrown sentence, the marker). app/content.ts: `MAP_FORMAT`, `MAP_VERSION`, `MOVEMENT_FORMAT`, `MOVEMENT_VERSION`, `CONTENT_KINDS`, `NOT_THE_MAP`, `ContentFile`, `ReadContent`, `readMap`, `readContent`, private `closed`, `corner`, `word`, `text`. sim-worker/src/index.ts: `LOCAL_MAP` re-exported. overlay/report.ts: `ContentRows`, `ReportData.content`, `ReportData.scene`. boot.ts: `window.__shambleScene` (development block). build-checks.mjs: two `DEV_MARKERS`. render/scene.test.ts (new). |
| **Change** | three-picture.ts: `createThreePicture` (the floor, `setContent`, `inventory`, the overview in `setContent` and `resize`; `declare const __SHAMBLE_DEVTOOLS__`). content.ts: `PageContent` constructor `(deliver, read = readContent)`, `afterMessage` delivers once, its comment (content.ts:117-122) rewritten. boot.ts:217 (the hand-off), :276-312 (`data()`), :315 (the second hook). report.ts: `readoutLines`, `formatReport`. e2e: skeleton.spec.ts:6, :45 and the broken-Worker tests; content.spec.ts:121, :198-200; report.ts's `Report` type; render-counts.json's shape. content.test.ts, content-ties.test.mjs, content-schema.test.mjs, report.test.ts, build-checks.test.mjs:243. docs/REPO_MAP.md:128 (docs-keeper). |
| **Remove** | render/empty-scene.ts (its content moves into scene.ts unchanged). `readBundle` as `PageContent`'s default reader (it stays, called by `readContent`). |

### 1.11 Each test, with its planted defect (each plant must turn the test red; run by the plants script as in 4b)
| # | Test | Where | Planted defect |
|---|---|---|---|
| T1 | `readMap` on the committed `content/maps/test-scene.json` equals the parsed tree without `format` and `version`, key for key | content.test.ts | a reader that returns `regions: []` |
| T2 | one extra key in each object kind (top, bounds, spawn, box, ramp, region) gives `NOT_THE_MAP` | content.test.ts | no key-set check |
| T3 | each member of the wrong kind (a string for a number, a number for an id, an unknown word for `type`, `rises_toward`, `kind`; a corner of two for three; 1e400) gives `NOT_THE_MAP` | content.test.ts | `return { ok: true, value: tree as MapDocument }` (B3 Q2 condition 1) |
| T4 | a box with `rises_toward`, a ramp without it: refused | content.test.ts | `rises_toward` read when present, for any type |
| T5 | with `Object.prototype.regions` set for the test (restored in `finally`), a map without its own `regions` is refused | content.test.ts | a reader using `tree.regions` |
| T6 | every refusal is exactly `NOT_THE_MAP` or `NOT_A_BUNDLE`; inputs carry `<img` in keys and values and no refusal contains it | content.test.ts | a refusal that names the field it met |
| T7 | `readContent`: no `maps/test-scene.json`, an unknown folder, a file whose format is not its folder's, version 2: each refused; a bundle with `maps/aaa.json` before `maps/test-scene.json` gives test-scene's map | content.test.ts | the first map file of the bundle taken |
| T8 | `PageContent`: `deliver` once on `ready`, never on `error`, never on a reading fault; the copy let go each time | content.test.ts | the copy kept after the read (a second message delivers again) |
| T9 | the page's format and version constants and folders equal map.rs, movement.rs, bundle.rs | content-ties.test.mjs | `MAP_FORMAT = "shamble-maps"` |
| T10 | each member's kind in the generated types equals the schema's (4B-MEMBER-KINDS) | content-schema.test.mjs | the compare run on a copy of Spawn's declaration with `facing_deg: string` must fail |
| S1 | each solid's distinct points equal `Math.fround` of its corners: 8 for a box; for a ramp toward each of the four sides (a made map, as collision.rs's `yard()`, :395-421), the four floor corners and the two top corners on that side; **and every triangle faces outward** (its normal by the right-hand rule points away from the solid's centre), for the box and for each of the four sides (the -x and -z wedges are mirrors of the twin's +x: a mirrored face list keeps the points and turns the faces inside out; advisor-tool point 1, taken) | scene.test.ts | the -x ramp built with the +x rule; the -z wedge's face list mirrored |
| S2 | each shape and region drawn once, by id; the floor holds nothing else but the ground and the grid; no spawn drawn | scene.test.ts | the last shape skipped |
| S3 | greys: every colour neutral; a higher top never darker; the tallest the lightest; both ground kinds darker than every solid (hard: GREY-COLOURS (a)); whether the two kinds differ follows his STREET-GREY letter (1.7) | scene.test.ts | the mapping inverted; cs2's blue grey kept for the map's ground |
| S4 | the grid: one line per metre over the bounds (113 + 49 on test-scene) | scene.test.ts | the empty floor's 30 m grid kept |
| S5 | the swap: each geometry and material of the old floor disposed once (dispose events counted); two builds share no geometry and no material | scene.test.ts | materials not disposed; a module-level material cache |
| S6 | the overview: the bounds' four floor corners project inside the view at 16:9, 4:3 and 1:1 | scene.test.ts | today's fixed camera |
| S7 | the inventory reads the drawn buffers: a vertex moved after the build is reported moved | scene.test.ts | an inventory that lists the map's corners |
| R1 | the content rows: each file with its format and version; `holding` and `none` in words; the scene row's counts | report.test.ts | the version printed from the other file |
| E1 | both backends (skeleton.spec.ts's first test): `__shambleScene()` equals the built bundle's test-scene map (ids, kinds, `Math.fround` corners, greys in height order), and the counters equal `["test-scene"]` | skeleton.spec.ts | boot calling `link.receive` alone (no scene: 4B-BOOT-CALLS); the swap without `disposeFloor` (geometries 2 high) |
| E2 | the seven hostile bundles: the inventory is the empty floor alone, the counters equal `["empty"]` | content.spec.ts | `setContent` called before `ready` |
| E3 | a Worker that cannot load or throws: the readout's content is `none` | skeleton.spec.ts | boot's error listener calling `link.workerError` alone (4B-BOOT-CALLS) |
| E4 | the production build holds neither new marker (`__shambleScene`, `INVENTORY_UNKNOWN`'s sentence), the development build both | build-checks (every build) | `readInventory` called outside the define |
| E5 | allocation.spec.ts on base and head: per-frame bytes unchanged within its noise | allocation.spec.ts | (a reading on both heads, as risk 5 of the plan says) |

### 1.12 The designs' count, recounted (written new lines; plus or minus 30 %)
| Part | Code | Tests |
|---|---|---|
| picture.ts (two records, two methods, their sentences) | 25 | 0 |
| render/scene.ts (map floor, corners, greys, grid, disposal, overview) | 100 | 0 |
| render/inventory.ts | 40 | 0 |
| render/three-picture.ts | 30 | 0 |
| app/content.ts (readMap and helpers, readContent, kinds and constants, the hand-off) | 125 | 0 |
| app/boot.ts | 15 | 0 |
| overlay/report.ts | 25 | 25 |
| sim-worker index, build-checks markers | 2 | 3 |
| content.test.ts (T1 to T8) | 0 | 150 |
| content-ties, content-schema (T9, T10) | 0 | 35 |
| render/scene.test.ts (S1 to S7, with the outward faces) | 0 | 130 |
| e2e (E1 to E3, folded into existing specs; render-counts.json is data) | 0 | 35 |
| **Total** | **362** | **378** |
Plus about 35 lines moved unchanged from empty-scene.ts into scene.ts. **About 740 against the sheet's about 630**
(230 / 260, plus 60 to 80 and as many in tests): about 15 % higher, inside the designs' 30 %. What made it grow: the
per-file headers for the readout (about 20 / 20), the inventory as a module of its own with its marker and S7 (about
40 / 25), the overview camera (about 15 / 15), and 4B-MEMBER-KINDS' direct compare (25 tests). At the measured rates
(2.17 to 2.7 written per counted line) that is about **1,600 to 2,000 written lines**: likely above the 1,420 the
scene is held to until SIZE-4C is answered. What could leave without a proof leaving its code (the main session's
choice, said to him before it moves): the overview camera under SIZE-4C (a) only; the per-file headers (the rows
then name paths alone and the formats from the folder table).

### 1.13 His open questions, by option, for the scene step
- **MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME:** none changes the scene step under any option. (Row 5 of
  MOVE-RULES changes what can be stood on, not what is drawn: the drawn heights are the map's real ones either way.
  TUNE-HOME acts on the camera's content, which joins at step 4: block 3, C6.)
- **SIZE-4C:** (a) the scene's code merges with the soldier's, reviewed on 4c's final diff; the overview camera can be
  left out (the follow camera frames the scene before 4c merges) and he sees the scene first in the early look.
  (b) and (c): the scene step is 4c-1 on its own, with its own reviewers (engine-architect for picture.ts,
  graphics-engineer for the drawing, security-auditor for a new reader of content on the page, test-engineer,
  perf-auditor) and his short Brave check: both areas in the overview, the greys rising with height, whether the
  street shows (look point 1), "errors: 0", the content and scene rows, "Copy report". The overview camera is then
  needed, since nothing else shows the yard.
- B1 DONE. Next: B2, SIZE-4C option (c)'s seam.
- N15. host.ts:81 calls `sim.input` with six numbers inside the Pacer's step; recording.ts:6 `STEP_BYTES = 68` and :44, :160 replay six numbers: both TypeScript sites change the moment `Sim::input` grows, whichever half carries it.
- N16. netcode-architect-cs4.md:453-456: the endpoint's aim cast falls back to the plane at the soldier's height "from the newest snapshot + 1.9 m": the cast (step 2) reads the snapshot's own soldier (step 3's protocol part), so a recording made before that part exists would change when it arrives.
- N17. netcode-architect-cs4.md:545 PM-0002-14 has a native half (the endpoint's cast against the aim sent) and a Chromium half (10,000 of 10,000 posts carry six finite ray numbers; no other message kind): its code runs from the page's post to the endpoint.
- N18. netcode-architect-cs4.md:520-531: RESUME-FLASH (a)'s two guards and its browser test are the page's (no extrapolation across a pause; no camera catch-up step), about 5 / 40 inside step 2's J row (plan step 2 = C 370/420 + J 15/110 = 385/530).

## Block 2. SIZE-4C option (c): the seam inside steps 1 to 7 (DONE)
**Answer: yes, one seam is clean, with three conditions.** It runs between the soldier's simulation with its whole
command path, and the soldier on the page. Counts are the plan's step rows (plan.md:234-245) split by file, not
re-estimated; where no design counts a file group apart it is said.

### 2.1 The seam
| Half | Steps, split by file | What it holds |
|---|---|---|
| **4c-2, "the soldier's simulation"** | step 1 whole; step 2 but for RESUME-FLASH's page guards and test (N18); **step 3's snapshot part** (`crates/protocol/src/`: the own soldier record and its codec; the room's encoder; the endpoint's decoder and the parity test); step 6 whole; step 6c whole | the soldier table, the rules taken from content, our own movement on the collision world, the post carrying the move and the ray, the 13-number input call, the endpoint's cast and the room's checks, the jump rule, the snapshot's own soldier, the movement and lane recordings with every mark that rides them, the first recording re-made, the hostile commands, the far-corner map; **and every new field of the simulation's state and its wire that steps 3 to 5 need**: the sprint-lock bit, the stamina's fields in the snapshot's own soldier, and the body-height field if NO-SOURCE puts it in the movement kind (block 3.2 and 3.6), so 4c-3 changes no hashed state |
| **4c-3, "he plays"** | step 3's block part (`client-endpoint/src/layout.rs`, `block.rs`, `layout_ts.rs`, the generated layout) and its page part (`app/blend.ts`, the Worker's `dueAt` in host.ts and pacing.ts, the pool, the three newest blocks); RESUME-FLASH's guards and test; steps 4, 5, 6b, 7 whole | the block's soldier rows and owner's section, the blend, the stand-in, the follow camera, the keys and the mouse, the aim line, the stamina bar, the readout's rows, the camera panel and its content file, the early-look zip, the paced replay, the page's browser tests, his main check |

### 2.2 The three conditions (each checked against today's code)
1. **The page posts "no move, no ray" in 4c-2.** `Sim::input` grows to 13 numbers in 4c-2 (the recordings drive the
   movement through it: N15), so host.ts:81 and recording.ts must change there, and main must run after 4c-2's
   merge. To keep PM-0002-14 whole in one half (N17), the post's shape changes in 4c-2 too
   (`PageToWorker.input` and `isPageToWorker`, messages.ts:33-42 and :135, the host's copy at host.ts:103-112),
   and `WorkerLink.frame` (worker-link.ts:332) posts a zero move and an all-zero ray, the values the page also posts
   in 4c-3 before the first mouse move. 4c-3 then replaces only those constants with `input/move.ts` and the cursor
   ray. **netcode-architect must agree** that an all-zero direction means "no ray this post" (keep the last aim, not
   counted), while a non-finite number stays "refused and counted" (its design counts a zero direction as refused:
   netcode-architect-cs4.md:306-308). If it does not, every post of 4c-2's live page counts as a refused ray: main
   still runs, but the counter is wrong in play, and the seam is not clean.
2. **Step 3's snapshot part goes with the cast** (N16): the movement recording, PM-0004-54's rays and the re-made
   first-loop need the Rust sides of steps 2 and 3's snapshot, and nothing of the page; the block's part has no
   reader before the page draws a soldier, so it stays in 4c-3. **Step 3's Rust lines (215 code / 220 tests,
   netcode-architect-cs4.md:563) are counted by no design apart between the snapshot and the block**:
   netcode-architect gives the split; until then the halves carry it as a range.
3. **The block changes only in 4c-3.** 4c-2 leaves layout.rs, the layout hash and `BLOCK_BYTES` as they are
   (layout.ts:5, :40), so the page's link, its checks and the allocation spec's per-block bytes change once, with
   their reader.

### 2.3 Each half's count (designs' count: code / tests)
- **4c-2:** step 1 1,240 / 1,090; step 2 380 / 490 (385 / 530 less RESUME-FLASH's 5 / 40); step 6 320 / 230; step 6c
  30 / 180; = **1,970 / 1,990 = 3,960**, plus step 3's snapshot share (0 to 215 / 0 to 220): **3,960 to 4,395**.
- **4c-3:** step 3 320 / 340 less that share; RESUME-FLASH 5 / 40; step 4 800 / 540; step 5 360 / 300; step 6b 130
  / 110; step 7 0 / 410; = **1,615 / 1,740 = 3,355**, less the share: **2,920 to 3,355**. (3,960 + 3,355 = 7,315, the
  plan's 4c.)
- At the measured rates (2.17 to 2.7 written per counted line): 4c-2 about 8,600 to 11,900 written; 4c-3 about 6,300
  to 9,100; the scene (4c-1, block 1.12) about 1,600 to 2,000. Each half is near 4b-2's 8,408 (two review rounds).
  By the plan's own rule (planned = 1.8 times, stop = planned times 1.25): 4c-2 planned 7,130 to 7,910, stop 8,910 to
  9,890; 4c-3 planned 5,260 to 6,040, stop 6,570 to 7,550. Every measured rate so far is above 1.8, so these stop
  lines would be reached; the lines are his (SIZE-4C).

### 2.4 The marks in each half (plan section 9's 4c row, one by one)
- **4c-2:** ADR 0001's replay line (build plan line 485) and the recordings of line 484; PM-0004-19, -17, -20, -21
  (three of three on movement fields), -24 on the movement recording's four runs (natively in one process and two,
  Node, Chromium); PM-0004-6, -15, -22 (the two new maths routines), -30, -35, -39 (the ladder's rung moves it to 4d,
  as now), -54, -59, -60; START-RULES' two rules; **PM-0002-14 whole** (its native half on the endpoint's cast, its
  Chromium half on the post's shape: it proves the aim rides the one post, and its plant, a separate aim message,
  bites on 4c-2's page; the ray's values from the camera are proven by 4c-3's cursor tests); 3B-ONE-PER-TICK;
  3A-SCAN-HELPERS; 4B-QUERY-RECORDING (the first query of the static world from the step is recorded here);
  4B-RAY-WASM and 4B-RAY-COUNTS (with the casts' first caller, the endpoint); 4B-WORLD-ROOM (block 4 of this read).
- **4c-3:** PM-0004-28 (the paced replay in the real Worker, camera still and camera moved: plan step 6b; it proves
  the page's code, which merges here, on the recordings 4c-2 made); PM-0002-26's smoothness (the blend's own test;
  the reading on his displays); PM-0002-34's walking steps (his checklist, the tab hidden for 1 and 6 minutes); lane
  2's rows of plan section 12; NA-N3, H2, 3B-POOL-16, 3B-CSS-PROD, 3B-BLUR-TEST, 3B-RESUME-FLASH; 4B-LIMIT-IMPORT;
  DEPART-4's three newest blocks.
- **Nothing in 4c-3 changes the simulation** (the block and the snapshot's bytes are not hashed: the shared state is,
  netcode-architect-cs4.md:396-399), so 4c-2's recordings stand through 4c-3. If 4c-3's review changed the input's
  meaning, the recordings would be made anew there, as its own named step (plan 6.2's ruling).
- **The never-moved list** (plan.md:249-251): the movement recording's four runs, the stale-id run, the hostile
  commands and the parity marks in 4c-2; the camera panel in 4c-3, with steps 4 and 5 in one half, so the panel is in
  the first build he plays. **PM-0004-28 is a reading for the advisor agent to confirm, not mine to rule:** section 9
  lists it "on the movement recording, before 4c merges", and the paced replay is on the never-moved list from the
  advisor agent's High condition 1 (plan.md:824). Under the seam it stays inside 4c and is met before 4c's last part
  merges, with the page's code it proves; whether "before 4c merges" may mean "before 4c-3 merges" is the advisor
  agent's (and, if it says the words are his, Mustafa's) call. If not, the seam is not clean.

### 2.5 What he sees or plays after each
- **After 4c-1 (the scene):** the grey street corner and training yard in an overview, the content and scene rows; a
  short Brave check (block 1.13).
- **After 4c-2:** **nothing new on screen.** The soldier lives in the simulation and its recordings, unseen; the
  page and its readout read as after 4c-1. So 4c-2 costs one go of his with no Brave check, and, with one branch,
  **the early look moves later by 4c-2's review rounds, its verifier and the wait for his go**: under (b) the same
  work comes before the early look with no such stop. That is the price of (c), for the sheet to put to him.
- **After 4c-3:** he plays: walk, sprint, jump, the two steps, the ledges, aim, the follow camera and its panel; the
  early look after step 5, his main check at its end.
- Not proposed: the other order (the page with a soldier standing at his spawn first, the movement after). A
  standing soldier needs the soldier table, the facing rule and the block anyway; PM-0002-26's reading and
  PM-0004-28 then need movement from the second half, and the panel is judged on a still figure.

### 2.6 Risks the seam adds
- **4B-CHROMIUM-TIME** (CI's Chromium run at 3.6 to 3.9 of 5 minutes): 4c-2 already adds Chromium time (the
  movement and lane recordings replayed in Chromium; PM-0002-14's 10,000-post run), so the split or new limit the
  CI Builder owns is needed before 4c-2's specs land, not only before step 7's.
- The zero-ray meaning (condition 1) is netcode-architect's to rule; a "no" makes the seam unclean.
- B2 DONE. Next: B3, steps 3, 4 and 5 against today's code.
- N19. pacing.ts:101-104: inside `wake`, `#nextDueMs` is the due time of the tick about to run, read before `step`; `afterTick(now)` passes only the start. So `dueAt` is one argument more on `afterTick` and one field on host.ts:191's post; nothing new is computed.
- N20. Today's layout (layout.rs:85-104; layout.ts:5-40): header stride 8 (tick, layout_hash), echo 20, link 16, 44 bytes; no F32, no `rows`. Plan 6.2: header 12, echo 20, link 16, own 20, soldiers 48 x 40 = 1,988. Design 3.3 had stride 36 (1,796): the plan added `vel_y` and `ready`.
- N21. The Export's camera is two groups, 5 + 16 = 21 values (content/source/testlines-export.json: `camera`, `camera_framing`); legacy CAM_STYLES (legacy/shamble-game.js:2195-2201) sets 6 of them per preset (height, back, fov, fovADS, yaw, adsCam) and is already extracted into docs/legacy-tuning.json (grep: 1 hit).
- N22. canonical.mjs's rule (JSON.stringify of the sorted value, two spaces, a newline; a minus zero refused) is the content's canonical form, but canonical.mjs imports node:fs through testlines.mjs (:15): it cannot be bundled for the page, so "copy values" needs a browser-safe copy of the rule; one copy only, the script importing it.
- N23. The movement kind has no body height (Movement.ts: body_radius_m only); the stand-in's 1.80 m and the capsule's height are "one number scales both" (plan 6.1), and NO-SOURCE decides where that number lives.
- N24. styles.css (71 lines) holds the development-only #overlay and #key-diagram rules (:29-71) beside the page's own; build-checks.mjs:232 scans `.js` files only (3B-CSS-PROD).

## Block 3. Steps 3 (the block's layout parts), 4 and 5 against today's code (DONE)
A delta against my design 4.1 to 4.10 and 3.1 to 3.7 (engine-architect-cs4.md:484-750); what is unchanged is not
repeated. "Today" is main dae6d107 with 4c-1's scene in.

### 3.1 What changed since the design, and what it changes
| # | Since the design | Consequence for steps 3 to 5 |
|---|---|---|
| C1 | `setContent` and `PictureContent { map }` arrive in the scene step (block 1), not at step 4 with the camera | step 4 adds `PictureContent.camera` (one field, block 1.1); design 4.1 item 2 is split in two |
| C2 | Plan 6.2: soldier rows of **40 bytes** (`vel_y`, `ready` added), block **1,988 bytes** (design 3.3 and 3.5: 36 and 1,796) | the row table below; the pool holds 16 x 1,988 = 31,808 bytes (design 3.7: 28,736) |
| C3 | DEPART-4: the page keeps the **three** newest blocks (netcode-architect-cs4.md:437-447), so the pair drawn is not always (previous, newest) | `PictureFrame`'s two fields are renamed for the pair the app picks (3.3) |
| C4 | N13: a view made per frame over two blocks fails allocation.spec.ts's per-frame limit | the blocks reach the picture as the `DataView` the link already makes once per block (3.3) |
| C5 | N14 and game-designer's stamina bar (plan section 2, step 4): no field carries the stamina | the owner's section gains two bytes in its reserved space (3.2); the lock is a state bit for rust-reviewer and netcode-architect |
| C6 | The camera's content file is placed in step 5, but its first reader, the follow camera, is step 4's | the camera kind moves to step 4 (3.5); the panel stays step 5. Both are in 4c-3, so the seam is untouched. **A departure from the approved step rows (plan.md:239-240), for the advisor agent's re-read**; the lines move between the two rows, the sum does not change |
| C7 | 4c-1 already puts `__SHAMBLE_DEVTOOLS__` inside render/ (block 1.4) | design 4.1 item 6's `devtools` start option is dropped; `panelHost` stays |
| C8 | His Export's framing replaces legacy's (DEPART-4; DIFF-LIST: height 24.5, boom 34.7, field of view 13, turn 64 degrees, and 16 framing values) | design 4.4's "legacy's framing, height 40, boom 56.6, fov 15, yaw 45" is replaced by the camera file's values |
| C9 | The input call takes 13 numbers with the move packed in one word (plan 6.2 (2); netcode-architect-cs4.md:299-316, which had 15 with the view time) | `input/move.ts` returns the packed word, not two floats (design 3.6 said floats quantised by the endpoint) |

### 3.2 Step 3's layout parts: the exact symbols (crates/client-endpoint)
- **Add:** `ElementType::F32` (layout.rs:8-36); `Section.rows: u32` (layout.rs:46-56), with `end_of` (:115-119) as
  `start + rows * stride` and `rows` in the hash's stream (:211-240) and in `layout_ts`'s output; sections `own`
  (rows 1, stride 20) and `soldiers` (rows 48, stride 40); `header.soldier_count u32 @8` (header stride 8 -> 12). The
  `tick` and `layout_hash` offsets do not move (layout.rs:76-78); `echo` moves from 8 to 12 by the generated
  offsets, which every reader takes from `BLOCK_LAYOUT`.
- **The soldier row (40 bytes):** `id u32 @0` (slot bits 0-15, generation 16-31), `pos_x/y/z f32 @4, 8, 12`,
  `vel_x/y/z f32 @16, 20, 24`, `yaw f32 @28`, `pitch f32 @32`, `flags u8 @36` (bit 0 occupied, 1 alive, 2 grounded, 3
  sprinting; bit 4 aiming joins in 4d), `team u8 @37`, `ready u8 @38`, one reserved byte @39 written 0.
- **The owner's section (20 bytes):** `soldier u32 @0` (0xFFFF_FFFF before the join is answered), `team u8 @4`, **new
  (C5): `stamina u8 @5`** (the share of a full bar, 0 to 255) **and `own_flags u8 @6`** (bit 0: sprint locked), one
  reserved byte @7, `aim_x/y/z f32 @8, 12, 16`. The size stays 20, the block 1,988.
- **The lock cannot be derived from the stamina** (game-designer-cs4.md:22, F11: run dry, sprint stays locked until
  the bar is back at its share, and while locked it refills with Shift held): between empty and the return share one
  bar value is locked on the way up and free on the way down. So it is movement state: a bit in the soldier's state,
  hashed, and in the snapshot's own soldier (netcode-architect-cs4.md:362-378 lists flags bits 0 to 2, :371, and `stamina`, :376,
  no lock). **Raised for rust-reviewer (the state bit) and netcode-architect (the wire bit and the share's source:
  the endpoint knows the stamina's time units from the snapshot, not the full bar's size), not designed here.**
- **Change:** `block.rs`'s writer (the rows by slot, the free rows zero, padding by the writer alone); `layout_ts.rs`
  (prints `rows`); the generated `packages/types/src/generated/block/layout.ts`; the layout tests (layout.rs:253-458).
- **Remove:** nothing; the "a section with more rows brings its count ... when it enters" comment (layout.rs:112-114)
  is done.

### 3.3 Step 3's page parts: the blend, the three blocks, the views
- **`PictureFrame`** (picture.ts:25-31) becomes `{ older: DataView | null; newer: DataView | null; blend: number;
  timeMs }`: the pair the app picked among the three newest and the share between them, in [0, 1]. Blocks are
  handed as the **`DataView` the link made when the block arrived** (worker-link.ts:293 makes exactly one today), so
  the picture reads rows with no view of its own: zero new allocation per frame and none extra per block (N13).
  Its sentence (picture.ts:18-24) is rewritten: "the Worker stamps the time each block's tick was due; the app picks
  the pair and the share (`blend.ts`); the picture only blends; keep neither a view nor its buffer past the call".
  The proof is allocation.spec.ts on the walking page (per frame and per block, against today's limits, never
  loosened by guess: plan risk 5).
- **`WorkerLink`** (worker-link.ts:62-63 `#newest`, `#previous`) keeps a ring of three `{ buffer, view, dueAt }`;
  `#takeBlock` (:276) gains `dueAt` and reads `own.stamina`, `own_flags`, `own.soldier` once per block into numbers
  (the stamina bar's source: no per-frame read of the block by the app). `newest` and `previous` getters give way to
  `pair(renderTime)` through `blend.ts`.
- **`app/blend.ts`** (new, as design 3.4 and netcode-architect-cs4.md:426-447): a pure function, state in numbers.
  **`dueAt`**: `Pacer.wake`'s `afterTick(now)` (pacing.ts:96, :104) becomes `afterTick(now, dueMs)` (N19); host.ts:191
  posts it; `WorkerToPage.block` (messages.ts:56) and `isWorkerToPage` (:182) take it. **`POOL_SIZE`** 4 -> 16
  (host.ts:37).

### 3.4 Step 4: the exact symbols, against design 4.1 to 4.9
| Part | Add / change (today's file:line) | Since the design |
|---|---|---|
| picture.ts | add `cursorRay(x, y, out: Float32Array): boolean`, `cameraYaw(): number`, `plainLines(points, count)`; `start`'s options gain `panelHost: HTMLElement \| null` (its use in step 5); `PictureContent.camera` | C1, C7; `plainLines` sized for its first user, the aim line (two points), its capacity a constant cs8 raises with its tracers, not 256 now |
| render/three-picture.ts | the soldier rows and the follow camera called from `draw`; the overview camera of 4c-1 removed | 4c-1's overview has no reader once the follow camera exists |
| soldier/stand-in.ts, rows.ts (new) | as design 4.3: one root, 48 roots made at start, rows `0 .. soldier_count`, place-not-slide on an id change, the alive bit lying down from a crafted block; one mid grey, a darker gun (GREY-COLOURS (a)) | reads `DataView`s (C4); the height: NO-SOURCE below |
| camera/follow.ts (new) | the follow camera from `PictureContent.camera`'s values; dt clamped across a long frame gap (RESUME-FLASH (a)'s second guard) | C6, C8 |
| input/move.ts (new) | held W, A, S, D and `cameraYaw()` -> the packed move word | C9 |
| input/pointer.ts (new) | the cursor's canvas position (`pointermove` on the canvas, `contextmenu` stopped there); let go on blur and on hide | no mouse button is a game button in 4c (the right button is 4d's): 3B-BLUR-TEST holds W in 4c and gains the button in 4d |
| app/worker-link.ts | `frame(keys, move, ray)` posts the real move and ray (4c-2's constants replaced: block 2.2) | the post's shape is 4c-2's |
| app/boot.ts | each frame: `move.ts`, `picture.cursorRay`, `link.frame`, `link.pair` -> `picture.draw`; the aim line (development) | |
| hud/stamina.ts (new; production) | a plain `<meter>` (or `<progress>`) whose `value` is set as a number only when the share changes, hidden at full and not locked (F11's rule); its look is the placeholder's | new since the design (C5); a number, not a style string, so no allocation per change |
| overlay/report.ts | rows: position, speed (from `vel`), flags, stamina and lock, aim point, blend share, `soldier_count`, block bytes (1,988), rays refused | 4c-1's content and scene rows stand |
| build-checks.mjs | `DEV_MARKERS` + `"aim-line"`; the production scan reads `.css` too (3B-CSS-PROD, build-checks.mjs:232) | 4c-1's two markers stand |
| styles.css | the development rules (:29-71) move to `overlay/dev.css`, imported only by the development modules; the stamina bar's plain rule added | 3B-CSS-PROD (N24) |

### 3.5 Step 5: the camera panel, and the camera's content file (moved to step 4: C6)
- **The camera kind (step 4, with its first reader):** `crates/content/src/camera.rs` (format `shamble-camera`,
  version 1, the 21 values by his Export's own names and units, as the movement table does: plan 5.1),
  `content/camera/<id>.json` written by the testlines tool from his Export, `bundle.rs`'s `FOLDERS` (:31) gains
  `camera`, `schema.rs`'s `schema_files` (:322, `[_; 2]` -> 3), the generated `Camera` type, `CONTENT_KINDS` on the
  page (block 1.2), `readCamera` beside `readMap`, the content-ties and member-kinds tests extended.
- **The panel (step 5):** `camera/panel.ts`, `settings.ts` as design 4.4: backquote opens it (the key handled in
  boot's development block, which shows or hides `panelHost`: no picture method for it); sliders whose limits come
  from the generated camera schema through **one product function** (4B-LIMIT-IMPORT): `content-schema.test.mjs`'s
  `limits` (:21-25) moves to `@shamble/types` as `numberLimits(schema)`, and the test imports it from there; the
  schema reaches the panel as a JSON import in its development chunk (`DEV_MARKERS` + `"camera-panel"`).
- **The five presets:** legacy CAM_STYLES sets 6 of the 21 values per preset (N21); a preset sets those values and
  leaves the others; its five rows are written by script from docs/legacy-tuning.json, never typed. Where they live
  follows TUNE-HOME: under (b) and (c), a `presets` list in the camera kind (data, held to the same ranges by the
  loader); under (a) the camera file must equal his Export, which has no presets, so they are the panel's own
  development-only table (`camera/presets.ts`, written by the same script). Not taken in any option: five whole
  camera files (each would have to invent 15 values a preset never set).
- **"Copy values":** the camera file's canonical bytes, by the content's own rule (N22): a browser-safe
  `canonicalText` in product code (development chunk), which canonical.mjs then imports, so the rule has one copy.

### 3.6 What his open answers change, by option
- **TUNE-HOME** (needed before step 5; it acts on step 4's camera file too):
  - (a) the sandbox is home: `testlines.mjs` `write` and `--check` gain the camera group (the camera file written
    from his Export and held equal to it, like soldier.json); "copy values" prints the Export's group and value
    names for him to set in the sandbox, not the file's bytes, so `canonicalText` in product code is not needed.
  - (b) one home per group: the camera file is written from his Export once, then `--check` leaves the camera
    group out and says so; "copy values" gives the file's canonical bytes (`canonicalText` in product code, one
    copy); the presets live in the camera file.
  - (c) everything in the game: as (b) for the camera; and `--check` leaves the movement group out too, so
    soldier.json is tuned by hand in the content from then on (no in-game movement panel exists in 4c).
- **RESUME-FLASH** (needed before step 4):
  - (a): `blend.ts` holds the newest block across a pause (the gap snaps the share: no slide from old stamps) and
    `follow.ts` clamps dt across a long frame gap; one browser test (hide with W held, return: the first frames draw
    the paused position, 0 ticks while hidden, he stops within 15 ticks with no input). About 5 / 40 (N18).
  - (b): (a), plus `KeyDiagram.dark()` (overlay/keys.ts:38-71) called from boot's `visibilitychange` (boot.ts:262-266)
    when the tab hides; one more test. About 10 lines more.
  - (c): host.ts:122's `this.#held = 0` and boot.ts:263's `keys.clear` are removed at the hide, undoing 3b's rule;
    the key diagram unchanged. (d): a "resume" input into the simulation, recorded: a new input field; not
    recommended by anyone.
- **NO-SOURCE** (step 1's, with one page consequence): the stand-in's height and the capsule's are one number (N23).
  Under (a) and (b) it is a field of the movement kind (from his new Export, or a row "not from your Export"), and
  the page reads it (`readMovement`, the first reader of the generated `Movement`, the rest of 4B-TYPES-READER), so
  `PictureContent` gains it in step 4; under (c) the height seems not to be among the dropped rules, so as (b) (an
  inference from the question's text, not ruled). Until the field exists, the stand-in cannot take its height from
  content, and a 1.80 m constant in the page would be a second home. **Raised for rust-reviewer** (whose read holds
  the movement kind's fields), not ruled here: the page reading the height crosses its read.
- **MOVE-RULES:** no symbol of steps 3 to 5 changes by its rows (rows 6 and 7 are 4d's; the others act inside the
  simulation).
- B3 DONE. Next: B4, 4B-WORLD-ROOM.
- N25. Today's owners: `LocalMatch` holds `world: StaticWorld` (local_match.rs:48-50, built by sim-wasm's `local_world`, lib.rs:83-86, handed in at :121, read through `static_world()`, :96-100, by one test, lib.rs:399-410); `Room` holds `World` (room/src/lib.rs:87-88, :105-108) and steps it with `authority_step(commands)` (:282; world.rs:149); `MatchSetup` is the seed and the start tick only (world.rs:26-31). Constructor callers: Room::new 3 (room lib tests 2, room/tests/any_bytes.rs 1), LocalMatch::new 7 (sim-wasm lib 1, local_match tests 5, tests/step_alloc.rs 1).
- N26. rules_scan.rs reads `crates/sim/src` (and `crates/content/src`): nothing holds the room's or the client endpoint's sources or manifests (rules_scan.rs:6-16, :471-499). Both manifests today: protocol and sim only.

## Block 4. 4B-WORLD-ROOM: where the authority's world lives (DONE)
- **Ruling: the room owns it.** `Room` gains `statics: StaticWorld` (and, by the same rule, the match's `Rules` and
  its spawn points, in whatever sim types rust-reviewer's step 1 gives them), taken by value in `Room::new(setup,
  content_hash, statics, ...)`; `World::authority_step` takes `&StaticWorld` (and `&Rules`) as arguments, borrowed
  from the room each tick (room/src/lib.rs:282). The movement functions already take `&StaticWorld`, never `&World`
  (design 2.1), so change-set 7's prediction passes the endpoint's own copy the same way. [REFINED by block 7: the
  types, signatures and the query counts' place are block 7's items 2 and 3 and B2.]
- **Not in `World`:** the static world is never state (collision.rs:1-3; design 2.4: not hashed, rebuilt after a
  restore). Inside `World` it would ride every copy of the state (change-set 7's resets) and sit among the fields
  `write_state` hashes. **Not in `LocalMatch`:** the room is also the native server's match; a world held beside it
  in local play would have to be held beside it again on the server.
- **Who builds it decides the manifests** (both crates already depend on `sim`, N26): the constructor that loads
  the content builds sim's types and hands them in: `Sim::create` (sim-wasm/src/lib.rs:105-128) today, the native
  server's match start later. So **no `[dependencies]` line changes**: room and client-endpoint receive
  `sim::collision::StaticWorld` through the `sim` they name; only sim-wasm (lib.rs:83) and devtools name `content`.
  The client endpoint's own copy for the aim cast (step 2) comes the same way: one build, cloned (`StaticWorld` is
  `Clone`, collision.rs:94-95), one copy for the room and one for the endpoint (plan risk 6: measured).
- **The one manifest consequence:** room's own tests (room/src/lib.rs tests, room/tests/any_bytes.rs) must now build
  a world, which needs a `MapDocument`, a content type: `content` joins **room's `[dev-dependencies]`** (test-only,
  as devtools' protocol line is), which changes Cargo.lock's room entry, so the dependency hold applies
  (`.claude/CHECKPOINTS.md` section 3, DEP-HOLD: security-auditor on that exact content, `dependency-check.md` before
  any push). The client endpoint's tests the same, once its endpoint takes a world (step 2).
- **A rule, since none exists (N26):** a test in devtools (tools may depend on everything), `tests/layering.rs`:
  the `[dependencies]` tables of crates/room/Cargo.toml and crates/client-endpoint/Cargo.toml name no `content`, and
  no file under their `src/` names `content` [SUPERSEDED by block 7, item 5 point 15: no non-test token of their
  `src/` names it] (rules_scan.rs's tokenizer, :127-252, reused, not copied: moved to a
  shared place if needed); planted: a `content = { path = "../content" }` line in a copy of the manifest text, and a
  `use content::map::MapDocument;` in a copy of a room source, each must fail. It also holds what `sim` hands on of
  `content`: today one constant, `pub use content::time::UNITS_PER_SECOND` (sim/src/time.rs:8), and no document
  type or reader (grep of `pub use` in crates/sim/src), so a room cannot read a map through `sim` either; a `pub use
  content::map::MapDocument` added to sim must fail it. About 25 / 25 lines. It keeps one
  reader of content in the authority's path: the constructor, through sim's `Rules::take` and `StaticWorld::build`.
- **What changes:** `Room` (a field, `new`, the step's call); `World::authority_step` (an argument); `LocalMatch`
  loses `world` and `static_world()` (local_match.rs:48-50, :63, :73, :96-100), and `new` hands the world on;
  `Sim::create` (lib.rs:121); the tests at lib.rs:399-410 (reads `local().room().statics()`), local_match.rs:203-213,
  tests/step_alloc.rs:84-86, room's two test sites. 4B-QUERY-RECORDING's first recorded query is then the room's.
  [REFINED by block 7, item 5: the list by number; the test reads `local().room().statics()`.]
- **When:** with the first step that queries it, step 1's movement inside the room's step: 4c-2 under the seam of
  block 2.
- B4 DONE.

### Plan (block 5: the advisor agent's item 10, points 4 and 5; advisor-reread-4c.md:39; sheet lines 228-232)
Started 2026-11-28 on branch cs/4c-soldier at 3ab59bd7. Read only; this file is the only one I write.
1. Read map.rs (the module doc, :33-66, `check` :316-405), schema.rs's bounds words, aim_reach.rs, step S's
   scene.ts (:190-239) and scene.test.ts, the test scene, StaticWorld::build, rust-reviewer-read (F5, F11, F16, 1.4,
   1.6 to 1.8), netcode-architect-read 1.1 and 2.1, the dependency batch (deps-4c.diff, dependency-check.md,
   security-auditor-deps-4c.md), ADR 0004 part 7 and PM-0004-22, maths-allowed.mjs and BANNED_MATHS, EDITOR_V2
   section 3, MOVE-RULES row 8, the old game's ground, libm 0.2.16's sine and cosine (the cargo cache, read only).
2. GROUND: reconcile the three statements (and any other that names a floor), say where a finding lies and its
   severity, then the rule, where it is written, its tests with their plants, and the count.
3. FACING-SPAWN: (a), (b), (c) each with its count and what it touches; (b) as a question for Mustafa; the
   recommendation; questions to rust-reviewer and netcode-architect.
4. A notes line after each finding; the block; B5 DONE; "Not verified" extended.

- N27. "The floor" in today's code: map.rs:39 (an aim point's height "in centimetres above the floor at 0"), schema.rs:146-147 ("y from the floor at 0 up to `AIM_HEIGHT_MAX_CM`"), devtools tests/aim_reach.rs:45 ("below the floor"): each calls y 0 the floor. map.rs:325 and :330 refuse `bounds.min[1]` only below 0, so the bounds' bottom may be any whole centimetre from 0 up. No committed or generated map starts above 0: test-scene.json bounds min [-32, 0, -24]; map.rs TEST_TEXT (:412) and the tests' TEXT (:420) min y 0; content_proofs.rs:393-395 always 0.
- N28. Step S (bc7720f3) draws the ground, every region and the grid at `bounds.min[1]` (scene.ts:197, :210-211, :231-235) and frames the overview at it (:263-265). scene.test.ts:21 uses the test scene's bounds (min y 0) and the grid test asserts the literal 0 (:149): no test tells "the bounds' bottom" from "y 0", so a planted `floorY = 0` would pass them all (read, not run).
- N29. netcode-architect-read.md:62-63: the aim falls back to "the crossing of the ground plane y = 0 ahead of the origin", then clamps into the bounds. The old game's own ground is that plane: `groundPlane` at 0 (shamble-game.js:1110) and `SIM.groundY: 0`, which stops the simulation's ray beside the statics (shamble-core.js:1507, :1516). On a map whose bounds start at h > 0 the fallback crosses y 0 below the drawn floor, h / tan(pitch) farther along than the floor point the cursor shows, and the clamp then lifts only its y: a wrong aim point on such a map. No such map exists.
- N30. The collision world has no floor today: `StaticWorld::build` reads `map.shapes` only (collision.rs:212-254); rust-reviewer-read F11 and 1.4 item 2 hold the bounds as solid in the sweep beside the tree. MOVE-RULES row 8 (question-2026-11-28-before-4c.md:30-31) names the sides only ("the map's edge is the map's own bounds"); no line of LOOP_V2 or VISION_V2 names where a fall ends (grep "fall", "pit": LOOP_V2.md:157 "Drop-downs from the high ground" only).
- N31. libm 0.2.16 in the cargo cache (read, not built): `pub fn sincos(x: f64) -> (f64, f64)` (src/math/sincos.rs:19); sin.rs, cos.rs, sincos.rs, k_sin.rs, k_cos.rs, rem_pio2.rs hold no `select_implementation`, `fma` or `mul_add`; rem_pio2.rs:56 holds the one x87 guard (32-bit x86 without SSE2, no target of ours). For a tiny argument it returns `(x, 1.0)` (sincos.rs:27-38), so 0 degrees gives (0, 1) exactly.
- N32. Quarter turns are not exact through radians (measured here with Python 3's `math` on glibc, not with the libm crate): with K = pi / 180 (f64 bits 3f91df46a2529d39), cos(90 K) = 6.123233995736766e-17, sin(180 K) = 1.2246467991473532e-16, cos(270 K) = -1.8369701987210297e-16, each still non-zero in f32. So a spawn facing 90 becomes (1, 6.1e-17), not (1, 0), unless whole quarter turns are taken out first: deg - 90 q is exact for q = 1, 2, 3 (deg lies within a factor 2 of 90 q: Sterbenz's lemma), and a quarter turn of a direction is a swap and a sign.
- N33. The batch's root comment (deps-4c.diff, the Cargo.toml hunk) names one routine: "the exponential, once per match, for the movement's easing shares (change-set 4c)". dependency-check.md: "If any of these five files differs when the batch is applied (main moved them, or the comments are reworded), the digests below no longer match and the batch goes to security-auditor again." security-auditor-deps-4c.md item 3: one scan allowance per named call site (file and function), bites for the same call elsewhere.
- N34. EDITOR_V2.md:37-38 ([DEFAULT]): rotation snaps "to the grid and to 15° / 45° / 90°"; :43-44 ([DEFAULT]) and :70-76 ([DECIDED], his B.1 of 2026-11-03): three-team mirroring, "the others follow at 120°". A spawn facing 90 mirrored at 120° faces 210 and 330: no quarter turn. EDITOR_V2 says nothing else on how a spawn faces. The test scene's 8 spawns all face 90.
- N35. Pointer only, not 4c's: the same 120° mirror cannot keep a box axis-aligned (map.rs:4-5, "all axis-aligned: no angle enters the static world"); the map kind and the editor's mirroring meet at editor slice 6 (EDITOR_V2.md:108).
- N36. ADR 0004:275 "a degree value becomes radians by one multiply with a pinned constant"; :283 "Transcendental maths only through the pure-Rust `libm` crate, pinned; no std `sin`, `cos` ..."; :286 "No sim value is computed in TypeScript." map.rs:63-64 asks for no sine or cosine "of the standard library": libm's is the ADR's own route.
- N37. The source scan is keyed to std's 79 method names (rules_scan.rs:52-62) and the fused ones (:102-110): a path call is refused only when its last name is one of std's (:386-387). std's is `sin_cos`; libm's `sincos`, `sincosf`, `sinf`, `cosf`, `expf`, `atan2f` are not in the list, nor `expm1` (std's spelling is `exp_m1`, :56), so `libm::sincos(`, `libm::expf(` and `libm::expm1(` pass the scan today (read, not run). The built-file check (build-checks.mjs:776-789) lists any libm routine it sees against the reviewed list, but an inlined routine prints no name (security-auditor-deps-4c.md item 3).
- N38. What `sincos` brings (cargo cache, read): k_sin, k_cos, rem_pio2 (sincos.rs:13); rem_pio2 uses rem_pio2_large (rem_pio2.rs:13, called :185 for a large argument only), which uses scalbn (rem_pio2_large.rs:14) and its own `extern "C" fn floor` over libm's floor (:229-232); libm's floor has an arch path only under `intrinsics_enabled` on wasm32 (floor.rs:15-20), set only when `unstable-intrinsics` is on and `force-soft-floats` off (configure.rs:85-89); the batch keeps both off. How many of these the built file prints is not known until it is built.
- N39. His words on spawns, facing and ground (grep of DECISIONS.md, LOOP_V2.md, main's EDITOR_V2 in editor-v2-main.md, which differs from the worktree's in no line): LOOP_V2.md:24 ([DECIDED]) "Everyone spawns inside their own barracks, in front of the buy terminal"; :153 ([DEFAULT]) "Rotational symmetry at 120°, with each base on its own lobe"; nothing on a facing's unit or steps, nothing on the ground under the bounds. DECISIONS.md's "ground" lines (:406, :437) are about what stays visible.
- N40. My own row (engine-architect-cs4.md:512, carried into block 3.2 above): `yaw` f32, radians, "the convention (0 toward +z, positive toward +x, or legacy's) is stated in the layout's doc and pinned by a test"; map.rs:58-62 shows the two are one (legacy turns by `atan2(dir.x, dir.z)`). Who computes it is step 3's open route: the endpoint with libm's arc tangent (my design :404-405) or the page's `Math.atan2` (rust-reviewer-read 1.8, :293-295).
- N41. The held aim at spawn is open and tied to this point: netcode-architect-read.md:648-649 ("the held aim is whatever FACING-SPAWN picks"); its 7.3 (2) gives `Endpoint::new` a first aim of `clamp_cm` of (0, 0, 0), and its zero-ray rule keeps the endpoint's last aim until a ray arrives. So, before any ray, commands carry the map's origin as the aim: on the test scene the yard's spawns (x 25 to 27, facing 90, +x) would turn toward -x on the first command, whatever FACING-SPAWN gives (inferred from the read, not run).

## Block 5. GROUND and FACING-SPAWN (the advisor agent's item 10, points 4 and 5) (DONE)
Against cs/4c-soldier at 3ab59bd7 (main dae6d107 with step S). Counts are my estimates of written new lines, code /
tests, plus or minus 30 %.

### 5.1 GROUND: the statements, reconciled
| Statement | What it says | Bounds from y 0 | Bounds from y h > 0 |
|---|---|---|---|
| map.rs:39; schema.rs:146-147; aim_reach.rs:45 | an aim's height counts "above the floor at 0" | y 0 is the bounds' bottom | calls y 0 "the floor", h below the bounds' bottom, where nothing may lie |
| map.rs:325, :330 | `bounds.min[1]` any whole centimetre from 0 up | | such a map loads |
| step S: scene.ts:197, :210-211, :231-235, :263-265 | the ground, the regions and the grid at `bounds.min[1]` | drawn at 0 | drawn at h |
| netcode-architect-read.md:62-63 (a design, not code) | the aim falls back to the plane y = 0 | at the floor | h below the drawn floor |
- They agree for every map that exists (N27: all start at 0) and part for a map whose bounds start above 0. What an
  aim's height counts from is y 0 and stays y 0 (the wire's unsigned 16 bits, aim_reach.rs); the word "floor" there is
  the defect: it takes the bottom to be at 0, which the check does not make so.
- **Findings, on the one scale:**
  - **G-1 (Low, against the map kind, mine):** map.rs:39, schema.rs:146-147 and aim_reach.rs:45 call y 0 "the floor"
    while map.rs:325 lets the floor stand above it. Fix: the words of 5.2.
  - **G-2 (Medium if built as written, against netcode-architect's aim design; its owner answers, NA1):** on a map
    whose bounds start at h the fallback crosses y 0 below the drawn floor, h / tan(pitch) farther along than the floor
    point the cursor shows, and the clamp lifts only its y back. Small reach (no map starts above 0 today); High once
    one does. Fix: the crossing at the floor, the lower y of `bounds_m` (netcode-architect-read 7.3's one home).
  - **G-3 (Low, against step S's evidence, not its code):** step S draws at the bounds' bottom, which 5.2 makes right,
    but no test tells the bounds' bottom from y 0 (scene.test.ts:21 at min y 0; :149 asserts the literal 0), so a
    planted `floorY = 0` passes them all. Fix: T2 of 5.2. A Low: it rides the movement step's commit (4c-2) and
    triggers no push of its own (the review budget; F-ORD-1).
  - Nothing against step S's code.
- **The other fix, not recommended:** refuse every `bounds.min[1]` but 0, so the floor is y 0 and every statement is
  true as written. Cost: a refusal kind and its sentence, the schema's lower y from [0, 655.35] to exactly 0,
  content_proofs' refusal class gains a case, the editor writes 0: about 15 / 25. It narrows what a map file may say
  (no committed file is refused: N27), so by the rule the brief applies to FACING-SPAWN (b) it would be his to decide.
  Not needed: under 5.2 every part reads one face, the bounds' lower y, so a raised map is right everywhere and the
  format stays as it is.

### 5.2 GROUND: the rule
- **Ruling (owner of the map kind): the bounds' bottom is solid ground**, flat and walkable, under the whole of the
  bounds' x and z: a soldier with no shape under him stands on it; a fall ends on it, landing as on any walkable top.
  The aim's crossing of it is NA1's question; whether the ray cast (`cast_ray`, 4d's shots) stops on it is RR5's, not
  ruled or counted here. Regions are drawn on it (map.rs:383 already: "A region lies on the ground"). The four sides are MOVE-RULES row 8's
  (his question: "the map's edge is the map's own bounds"); the top is the movement's (rust-reviewer-read F11: a
  ceiling); neither is ruled here. No word of his changes: none of his rules names the bottom (N30, N39), and the old
  game's ray and aim already end on a ground plane at 0 (shamble-core.js:1507, :1516; shamble-game.js:1110).
- **Not taken: a ground shape in every map** (rust-reviewer-read 1.4 item 2, option (b)). A shape lies inside the
  bounds and is at least 1 cm thick (map.rs:11-12), so its top stands at least 1 cm above the drawn ground and step S
  would draw it as a block over the ground and every region, hiding them; a fall still needs an end below it; and it
  takes one of the 512 shapes and a test in every ray.
- **Where it is written**, in the commit that first makes the movement read it (step 1, in 4c-2):
  1. map.rs's module doc (:3-6), one sentence: "`bounds` is the playable box; its bottom is solid ground under the
     whole box: a soldier with nothing under him stands on it, and a fall ends on it." (2 lines)
  2. map.rs:39 becomes "in centimetres above y 0 (the aim's zero; the bounds' bottom, the ground, lies at or above
     it)"; schema.rs:146-147 and aim_reach.rs:45 take the same words; schema.rs:56's bounds description gains "its
     bottom is the ground", which moves one line of the committed schema (its tests compare it). (about 5 lines)
  3. The check: no change. It is a meaning, not a range; the bottom stays any whole centimetre from 0 up.
  4. One home: the floor is the lower y of `bounds_m`, derived once in `StaticWorld::build` (netcode-architect-read
     7.3); the sweeps and the aim read it there; the page reads `bounds.min[1]` from the map (step S); the two are tied
     as N6 shows for the corners (Math.fround of the map's number equals the simulation's f32).
- **Tests, each with its plant (each plant must turn its test red):**
  - T1 (sim, step 1): on a made map whose bounds start at y 2, with no shape under the spawn, a soldier spawned 1 m
    above the floor falls and lands at exactly y 2 (grounded, vertical speed 0); on the test scene at 0 (step 1's own
    landing test). Plant: the sweep's floor at y 0 (the raised map's soldier falls on through 2 m).
  - T2 (page, scene.test.ts): with the same raised bounds, the ground's, every region's and every grid line's y equal
    2. Plant: `floorY = 0` at scene.ts:197.
  - T3 (aim, netcode-architect's if NA1 is yes): on the raised map a cursor ray that misses every shape ends on the
    floor, at the x and z where the ray meets y 2. Plant: the plane at y 0.
- **Count:** about 7 lines of words (map.rs, schema.rs, aim_reach.rs) and 2 of code (the aim's crossing reads the
  floor; the sweep's floor plane is already in step 1's count, rust-reviewer-read 1.4 item 2) / about 42 of tests
  (T1 20, T2 10, T3 12).

### 5.3 FACING-SPAWN: the three options
Today `facing_deg` is any degree in [0, 360) (map.rs:58-66); the soldier keeps his facing as a direction
(rust-reviewer-read 1.1, `facing: [f32; 2]`; 1.7); the test scene's eight spawns all face 90 (N34).

| | (a) libm's sine and cosine | (b) quarter turns only | (c) degrees until the first aim |
|---|---|---|---|
| Where it runs | once per spawn point when a match starts, in the constructor that takes the match's spawns beside `Rules::take` (at most 16, map.rs:27); never in the step | the same place, a four-row table | in the state, every tick until the first aim farther than 2 cm |
| Maths | one routine, `libm::sincos` (N31); degrees to radians by one multiply with a pinned constant (ADR 0004:275; K's f64 bits 3f91df46a2529d39) | none | none in the simulation; the endpoint's yaw from degrees is degrees x K, one multiply (the two conventions are one: N40) |
| Quarter turns exact | only if whole quarter turns are taken out first (N32; RR3); else 90 gives (1, 6.1e-17) | yes | yes |
| What a map file may say | unchanged | narrowed: every degree but four refused (no committed file is: N34) | unchanged |
| The guards | the scan names `sincos` and `sincosf` with a bite (they pass it today: N37), then one allowance at the one call site; maths-allowed.mjs gains `libm::math::sincos` and whatever the built file prints of N38's candidates (one entry or more: rust-reviewer's count); PM-0004-22's vectors (at least 10,000 inputs, natively, in Node and in each browser engine) | none | none |
| The manifest batch | no manifest line; the root comment names only the exponential (N33), so it must change, its digest moves, and the batch goes to security-auditor again (5.6) | unchanged | unchanged |
| State, wire, block, recordings | one direction from the first tick: rust-reviewer's Soldier, netcode-architect's OwnSoldier (its 2.1: "the facing as step 1 holds it") and my row unchanged | as (a) | two forms of one field (5.4) |
| Count (estimate) | about 25 (the routine with its quarter turns 15, the scan 5, the comment 2, the vectors' generator 3 beyond its exp lines), plus the list's entries / about 45 (quarter turns exact, 45 and 120 by bits against a reference, the scan's bite and control, the vectors' run) | about 30 / 30 (a refusal kind and its sentence, the schema's four values, content_proofs' generator and a refusal case, the table) | about 35 / 60 |
| Whose | rust-reviewer (the maths); security-auditor (the batch, again) | **his decision** (5.5) | rust-reviewer (the state); netcode-architect (the wire, the recordings) |

### 5.4 (c) in full: the row, the state, a recording
- **The row (block 3.2):** `yaw f32 @28` and `pitch f32 @32` keep their place and meaning. The endpoint writes the
  yaw from two sources: degrees x K while the state holds degrees, the arc tangent of the direction after. That fixes
  step 3's open route at the endpoint: the page's `Math.atan2` route reads a direction, which the degrees form does not
  have without a sine and a cosine, unless a flag bit (bit 5) says which meaning the field holds, which puts two
  meanings of one field across the boundary.
- **The state:** yes, the hashed state holds the degrees until the first aim farther than 2 cm: `facing` becomes a
  two-form field (a tag byte, then 4 bytes of degrees or 8 of direction), written by `visit_shared` with its tag. A
  soldier with no commands (a bot's slot, an idle player) keeps the degrees form for as long as none arrives.
- **A recording or a restore:** each tick's digest holds the tag and the degrees until the first aim; a replay agrees
  (it is deterministic), but every reader of the field takes both forms: the block writer; netcode-architect's
  OwnSoldier (a tag on the wire, the codec refusing an unknown one); change-set 7's reset from the snapshot; devtools'
  `Lines` (replay.rs:181). A restore must give the tag back exactly, or the digest after it differs.
- **Later:** any rule of the simulation that reads the facing as a direction (none in 4c) meets the degrees form and
  needs a sine and a cosine inside the step, which 1.7 keeps out, or converts at its first read: (c) moves the routine
  later and into the step; it does not remove it.

### 5.5 What goes to Mustafa
- Under (a), the recommendation: **nothing.** No word of his changes; the map kind keeps any degree, as today.
- Only if a reviewer wants (b), ready to ask:
  **FACING-QUARTER.** *Question:* should a spawn on a map face only the four main directions (0, 90, 180 or 270
  degrees) instead of any direction? *Why it matters:* any direction needs one reviewed sine-and-cosine routine, run
  once per spawn point when a match starts; four directions need none, but the map format then refuses every other
  direction. *Options:* (a) any direction, as today, and the routine joins with its proofs; (b) the four main
  directions only: the map reader refuses the others; no map changes today (all eight spawns of the test scene face
  90), but the editor's rotate snapping to 15° and 45° (a default of his editor brief) could not be used for spawns,
  and on a three-team map turned at 120° (the default layout of his loop brief, section 9) at most one team's spawns
  could face a main direction (engine-architect's reading of the two briefs, not a word of his on spawns).
  *Recommended:* (a).
- Pointer only (N35): the same 120° turn cannot keep a box axis-aligned; the map kind and the editor's mirroring meet
  at editor slice 6, not in 4c. The 120° point still supports the case against (b), but does not decide it: the kind
  cannot follow that turn today either.

### 5.6 Recommendation
- **(a): `libm::sincos`, once per spawn point in the match-start constructor, exact on quarter turns.** It is ADR
  0004 part 7's own route (N36) and the one map.rs:63-64 already promises; it touches no state, wire, block or
  recording; its cost is guard work already set up for the exponential (the scan's allowance, the reviewed list,
  PM-0004-22's vectors); the format stays open to any facing. (c) spreads two forms of one field over the state, the
  wire, the block and every recording, for a facing his first cursor aim replaces; (b) is the least code but narrows
  his format, and is his to decide.
- **The batch: yes, (a) needs the root comment changed, and so a new answer.** It names "the exponential, once per
  match" (N33); with the sine and cosine it would be wrong by omission. Recommended: reword the patch now, before it is
  applied, to name both (for example "the exponential for the movement's easing shares, and the sine and cosine (one
  routine) for the spawns' facings, each once per match"), and send the reworded batch to security-auditor once, its
  F1 and F2 conditions standing. The patch, its digests and dependency-check.md are the main session's files; none of
  the other four files changes.
- **The held aim at spawn** (N41): recommended the spawn point in whole centimetres until a command brings an aim:
  rounding a millimetre spawn moves it at most 0.71 cm, under the 2 cm rule, so the spawn's facing holds with no
  maths. The endpoint's first aim, the map's origin, is NA2.

### 5.7 Questions for rust-reviewer (the maths), answered in its own file
- RR1. The scan refuses std's names only (N37): `libm::sincos(` passes it today, and so do `libm::expf(`, `sinf`,
  `cosf`, `atan2f` and `libm::expm1(` (std's spelling is `exp_m1`). Should the refused names gain libm's own (at least
  `sincos` and `sincosf`, and `expf`, `exp2f` and `expm1` for the exponential) before any allowance? It reaches the
  exponential's planned guard too: security-auditor-deps-4c.md item 3 lists `libm::expm1` among the bites "still
  refused", which would not bite as the scan stands, so it touches an answer already given. My severity: Medium (the
  guard does not hold for libm's own spellings; the built-file check covers a routine only when it is not inlined).
- RR2. One routine, `libm::sincos` in f64 and then `as f32`, rather than `sin` and `cos`, or `sincosf`; and K's f64
  bits pinned by a test?
- RR3. Exact quarter turns by taking them out first (three compares give q; deg - 90 q is exact by Sterbenz's lemma;
  the turn is a swap and a sign), or accept (1, 6.1e-17) at 90 and test by value?
- RR4. Which of N38's candidates the built file prints, and so the reviewed list's entries (the count is yours), and
  the vectors' domain (the remainder's [0, 90) x K and its edges).
- RR5. The floor as the plane at `bounds_m`'s lower y beside the tree, not a 513th solid (the 512 cap and the tree's
  committed bytes would move); T1 and its plant in step 1. And does `cast_ray` stop on it (the old game's ray does,
  shamble-core.js:1507), now or with 4d's shots, and which world proofs would that move?

### 5.8 Questions for netcode-architect (the row, the state, the recordings), answered in its own file
- NA1. The aim's ground crossing (netcode-architect-read.md:62-63) at the floor, `bounds_m`'s lower y, instead of the
  plane y = 0 (G-2), with T3 and its plant?
- NA2. The held aim at spawn: the spawn point in whole centimetres? And `Endpoint::new`'s first aim, `clamp_cm` of
  (0, 0, 0) (your 7.3 (2)), kept by the zero-ray rule until a ray arrives, turns a new soldier toward the map's origin
  on his first command under every option (N41): accept that as the rule and say so, or keep the soldier's held aim
  until the endpoint has a ray (and how, on the wire)?
- NA3. Under (a) the facing is one direction from the first tick: do your OwnSoldier (2.1) and the recordings need no
  change? Under (c) a tag joins the wire, the codec's checks and every recording's state (5.4): any reason to prefer
  it?
- B5 DONE.

### Plan (block 6: netcode-architect's block 8.5 flag and block 7.3's constructor question)
Asked 2026-11-28 on 3ab59bd7 by the main session. Read only; this file is the only one I write.
1. Read netcode-architect-read block 8 (:830-953) and 7.3 (:714-760); recheck my block 3.2 (the owner's section, the
   row), my design's pitch (engine-architect-cs4.md:513), room/endpoint constructors, sim-wasm's `local_world`.
2. Answer yes or no, each with its reason and count: what the owner's section and the row's pitch show for "no aim
   yet"; whether `Room::new` and `Endpoint::new` refuse a world with no aim box.
3. A notes line per finding; the block; B6 DONE; "Not verified" extended.

- N42. netcode-architect-read 8.3 (:871-917): `AimCm::NONE` = (-32,768, 0, 0) cm, "no aim yet"; the row holds it from the spawn; the facing keeps the spawn's while it is NONE; NONE never passes `clamp_cm`; `Endpoint::new` takes no aim and the endpoint holds no aim of its own (item 4); the snapshot's own soldier carries the row's `aim_cm` (B8-N6, :849). 8.5 (:943-945) asks me what the owner's section and the row's pitch show for NONE, and proposes "no aim point drawn, and a pitch of 0".
- N43. My block 3.2: the owner's section holds `aim_x/y/z f32 @8, 12, 16`, which netcode-architect's 2.1 (:240) calls "the aim the endpoint sent"; `own_flags u8 @6` uses bit 0 (sprint locked) only. The row's `pitch` is "radians, positive up, from the aim point and the eye height" (engine-architect-cs4.md:513). With the endpoint holding no aim (8.3 item 4), "the aim the endpoint sent" exists per post only, and is NONE on every post with no new ray, so it would flicker between a point and nothing.
- N44. 7.3 (:725-735): an aim box empty on some axis exists only for a map built in code; every loaded map lies inside the reach (map.rs:319-339; his words at map.rs:13-14), so its box is its bounds. sim-wasm's `local_world` builds from the loaded map (sim-wasm/src/lib.rs:83-86); `Room::new` returns `Self` (room/src/lib.rs:105), `Endpoint::new` an `Option` (client-endpoint/src/endpoint.rs:117). map.rs:16-17: "A map built in code, for a test of the simulation alone, never passes the loader and is no playable map."

## Block 6. "No aim yet" in the block, and a world with no aim box (DONE)
Counts are my estimates of written new lines, code / tests, plus or minus 30 %.

### 6.1 What the block shows for "no aim yet": YES to its proposal, with one bit and one change of source
- **The owner's section's aim is the soldier's held aim**, as the newest snapshot's own soldier carries it
  (`aim_cm`, B8-N6), turned into metres in the endpoint (each axis cm / 100 as f32, exact, as its 7.2 has it). It is no
  longer "the aim the endpoint sent" (N43): the endpoint holds none, and the held aim is what the facing follows, so
  the page shows what the authority took. A change of meaning in my layout, for netcode-architect's note; no wire change
  (`aim_cm` already rides the own soldier).
- **New: `own_flags` bit 1, "aim held"**, set when that held aim is a real point. While it is NONE, and before the
  first snapshot that carries the soldier, bit 1 is clear and `aim_x/y/z` are written 0 by the writer. NONE never
  crosses the boundary as a number: not its metres (-327.68, 0, 0), not a NaN. One meaning per field (the argument of
  5.4), and the page needs no copy of the wire's sentinel. The section stays 20 bytes, the block 1,988.
- **The row's pitch is 0 while the held aim is NONE** (level); its yaw comes from the facing (the spawn's, under
  FACING-SPAWN (a)); with a real held aim, the pitch is computed from it and the eye height, as laid out. Yes, as
  proposed.
- **The page:** the development aim point and line are drawn only while bit 1 is set; the readout's aim row reads
  "none yet" while it is clear.
- **Tests, each with its plant:** the block writer (client-endpoint), before the first snapshot and with NONE held:
  bit 1 clear, aim 0, the row's pitch 0; with a real held aim: bit 1 set, the metres equal cm / 100 bit for bit.
  Plants, each red: NONE's metres written (aim_x -327.68); the pitch computed from NONE's point (not 0). The page
  (vitest): bit 1 clear draws no aim line and the readout says "none yet". Plant: the line drawn from (0, 0, 0).
- **Count:** about 8 code (the layout's bit and its doc 2, the writer 3, the page's two reads 3) / about 20 tests,
  in step 3's block part and step 4.

### 6.2 A world with no aim box: NO constructor change (its option (B))
- **Reason:** by block 4 the room and the endpoint take their world from the constructor that loads the content, and
  the loader refuses a playable map beyond the reach (his words, map.rs:13-14), so every loaded map's box is its
  bounds (N44). A world with no box is built in code, for a test of the simulation alone. A refusal in `Room::new`
  would be an error path no product caller can take, and four callers would change for it.
- **No check in `create` either**, if 7.3's "the box is checked once where a match's world is built" means a new
  refusal there: it could not fire on any loaded map, and it would be a second home for the loader's rule. The loader's
  check stays the one home; 7.3's tie test (d) already shows a loaded map's box equals its bounds.
- **It fails safe:** under block 8, on such a world every real aim is refused by `holds_cm` and counted, NONE is taken,
  and the soldier keeps his facing.
- **Test:** one room test on a code-built world wholly past the reach: every real aim refused and counted, NONE
  commands taken, no panic. Plant: a `clamp_cm` that unwraps the empty box (it panics: red). **Count:** 0 code /
  about 10 tests.
- B6 DONE.

## Not verified
- Nothing was built or run for this read but one import of three/webgpu in plain Node (N7). Every count is the
  designs' estimate (plus or minus 30 %).
- The scene's render counts in block 1.6 are arithmetic on the design; the measurement goes into the file.
- That the define and tree-shaking cut `inventory.ts` from the production build (the `FrameTimes` precedent) is
  unproven until a build runs the marker scan.
- The `DataView` hand-off's claim of no allocation per frame and none extra per block (block 3.3) is unmeasured until
  allocation.spec.ts runs on the walking page.
- netcode-architect's and rust-reviewer's parallel reads were not seen; the zero-ray meaning, step 3's Rust split,
  the sprint-lock bit, the stamina's wire and share, and the body-height field are theirs.
- three's dispose bookkeeping (N11) was read in the built file, not run.
- Block 5: libm 0.2.16's sine and cosine were read in the cargo cache, not built or run: which of its routines the
  built file prints (N38) and whether its `sincos` gives N32's values are unmeasured. N32's values are glibc's,
  through Python 3 on this machine, not the libm crate's.
- Block 5: the scan's gap (N37) and every plant of 5.2 are read from the source, not run; T1 to T3 and the counts are
  estimates. netcode-architect's block 7 was read as its file stood during this block.
- Block 6: read only; the bit, the pitch rule and the room test are design, not built or run.

DONE

### Plan (block 7: the advisor agent's F1, WORLD-ROOM agreed in writing before step 1's code)
Asked 2026-11-28 by the main session, on cs/4c-soldier at 695b45b9 (the Rust is main's dae6d107: 4c's commits since
are page and notes only). Read only; this file is the only one I write.
1. Read the clash: rust-reviewer-read 1.6 (:239-269) and 11.6 (:892-924), my block 4 (:529-565), the sheet's E
   (:313-319), the advisor agent's F1 and Q4 (advisor-steps1to7.md:25-31, :136-158), netcode-architect-read's record
   and restore lines (:72-84, :674-684).
2. Read today's code: sim world.rs, lib.rs, collision.rs (StaticWorld, build, narrow), room lib.rs, sim-wasm lib.rs
   and local_match.rs, client-endpoint endpoint.rs (struct, new), devtools replay.rs, the recordings' manifest and
   frozen bundle, content bundle.rs and map.rs (spawns), the manifests, the batch (deps-4c-v2.diff), ADR 0004 parts 1,
   3, 9, 10 and 17.
3. Answer the six items of the brief with file:line; Q4's conditions; a notes line per finding; the block; B7 DONE.

- N45. Today's `World` is `{ tick, player: PlayerCommand, seed }` (world.rs:112-117), "the whole state of one match"
  (:111); both visitors name every field in an exhaustive pattern (:166-170, :180-184), so a field added to `World`
  compiles only if handed over or written `_` in plain sight. The room owns the `World` (room/src/lib.rs:87-88) and
  steps it at :282; `LocalMatch` owns the `StaticWorld` beside the room (local_match.rs:48-50, :63, :73), read only by
  `static_world()` (:96-100) and one test (sim-wasm/src/lib.rs:401-410).
- N46. ADR 0004 part 1 (:131-137) lists "the static colliders with their ids" among the state struct's tables; part 3
  (:163-165) and part 10 (:365-367) say the static collision index is "rebuilt after a restore, never stored in a
  snapshot", the static world "immutable for the match", and "a destructible piece's state lives in the sim's tables".
  collision.rs:1-3 says the same ("never changes after, and is never part of a snapshot"). Read together: the
  immutable static world is not state; a collider's changing state (none before destructibles) is a table in the state.
  Part 1's "nothing lives outside the struct (no globals or statics in the sim crate)" is about Rust `static`s, whose
  test is PM-0004-17 (ADR 0004:1138, "Bites: a `static` counter read by a phase"; devtools/tests/two_instances.rs).
- N47. netcode-architect-read.md:83-84: "Whatever the World keeps is hashed (ADR 0004 part 9)". Part 9 (:342-344): the
  sim writes "a canonical serialization of its whole state". A constant held in `World` but left out of `visit` would
  break that sentence; a constant written into it would hash content the digest already names (N50).
- N48. The record: `authority_step` stores the newest command whole (`self.player = *newest`, world.rs:150-152). Once
  `PlayerCommand` gains `move_x`, `move_y` and `aim` (rust-reviewer-read 11.6, :904), a `player: PlayerCommand` field
  keeps the newest move and aim in `World` while `visit_shared` writes only buttons and presses (11.2, :842-843):
  unhashed state. netcode-architect named the narrow type, `PlayerRecord` (netcode-architect-read.md:78); block 11.6
  has no such symbol (grep of rust-reviewer-read.md: no `PlayerRecord`). The record's readers outside world.rs: the
  snapshot echo (room/src/lib.rs:286-296), room tests :461, :496, sim-wasm local_match.rs:267-268, :336, :370.
- N49. `QueryCounts` (rust-reviewer-read 11.6, :899; block 2, :381-384) is "outside the state ... in the scratch the
  step already owns; never hashed ... read through a `#[doc(hidden)]` accessor on the world": a second non-state field
  in `World`. The room already holds its run's tallies beside the world (`counters: RoomCounters`,
  room/src/lib.rs:45-68, :99, :133-135: "the counters are what a test, and later a metric, reads").
- N50. How content is named today. The loader takes a bundle only byte for byte in canonical form (bundle.rs:3-11,
  `document` :88-108), so one content set has one byte string and two different sets two different sha256 digests.
  A recording's header names its content by sha256, and the replay refuses any other bundle before the match is made
  (replay.rs:37-54), then makes the match through the players' own entry point on that bundle (`Sim::create`,
  replay.rs:119-123); the manifest pins the frozen bundle's digest (recordings/manifest.txt:1, first-loop.content.json
  b9901bc9...). ADR 0004 part 9 (:350-351): a recorded match is "the seed, the map and mode data, the content data's
  hash and the per-tick inputs". In live play the join's hash is zeros on both sides until change-set 8 (CD-8)
  (local_match.rs:20-22, :66-67; room/src/lib.rs:149); the bundle crosses once, into `Sim::create` (sim-wasm/src/
  lib.rs:96-99, :116).
- N51. The build half of "the same bundle gives the same constants": `StaticWorld::build` ranks shapes by id, so two
  documents with the same shapes give the same world byte for byte (collision.rs:13-14, :212-254); its trees and rays
  are committed and compared natively, in Node and in the browsers (devtools/tests/static_world.rs:1-5, PM-0004-32).
  `Rules::take` and the spawns' facings have no such file yet: block 11.7's tests 2 to 4 (:930-933) pin them.
- N52. Which movement document a match takes: none today (sim-wasm reads the map only, lib.rs:83-86, :116-117). The
  repository's content and the frozen first-loop bundle hold `movement/soldier.json` (content_files.rs:363-366; grep of
  first-loop.content.json's paths); the test bundle holds `movement/test-soldier.json` (bundle.rs:260-265), with
  "test-soldier" in 49 lines of 7 files. bundle.rs:307: "which files a match needs is the match's to say, not the
  loader's"; the map is named by `LOCAL_MAP` (sim-wasm/src/lib.rs:23-25). No block names the movement's id.
- N53. Who uses the spawns, and when: no block says when the one player's soldier is made or at which spawn
  (grep of the four reads for "spawn(", "join", "spawn 0": only `Soldiers::spawn`, rust-reviewer-read.md:74).
  11.6 has `World::new` take the constants (:904); netcode-architect's routes start "from the spawn" (singular,
  netcode-architect-read.md:351-362). The test scene's eight spawns in file order begin with `street_1` at
  (-20, 0, -0.9), which is also the lowest id (content/maps/test-scene.json). ADR 0004 part 17 (:489, :501-503) makes
  spawning a sim phase (barracks, respawn), which is change-set 5's and the loop's, not 4c's.
- N54. The batch already gives room and client-endpoint `content` as a dev-dependency for their tests (deps-4c-v2.diff
  :59-82), reviewed with five digests (dependency-check.md:30-34); their comments say "the room's code ... never names
  `content` (devtools' tests/layering.rs holds that)". The room's unit tests live in src/lib.rs (:304-669), so a
  layering rule over every token of src/ would fail on them; rules_scan.rs already marks test-only tokens
  (:695-718: a `#![cfg(test)]` file, or the item after `#[cfg(test)]`).
- N55. Every caller that builds or steps a `World` or a `Room` (grep over crates/): `World::new` at room/src/lib.rs:108,
  sim/tests/state_hash_alloc.rs:70, devtools/tests/hash_cost.rs:40, world.rs tests (:241); `authority_step` at
  room/src/lib.rs:282, state_hash_alloc.rs:94, hash_cost.rs:46, 23 lines of world.rs; `Room::new` at local_match.rs:66,
  room/src/lib.rs:314, :655, room/tests/any_bytes.rs:117; `LocalMatch::new` at sim-wasm/src/lib.rs:122,
  local_match.rs:213, :399, :413, sim-wasm/tests/step_alloc.rs:86. `Endpoint::new` (endpoint.rs:117) is not touched by
  step 1 (11.6 changes only :258); step 2 gives it its own static world (netcode-architect-read.md:99-101).
- N56. netcode-architect's restore test reads `read_shared(bytes, constants) -> World`, "the match constants (rules,
  static world, spawns) and the seed come from the same build and setup, never from the bytes"
  (netcode-architect-read.md:675-677). With the constants outside `World`, the reader takes the setup only.
- N57. `make_recordings --freeze <name>` is "the one writer of a recording's frozen content" (devtools/src/bin/
  make_recordings.rs:4-6). 11.8 (rust-reviewer-read.md:955-958) says first-loop is re-made "through its script"; the
  sheet (:341) and netcode-architect-read.md:434 say first-loop is frozen anew whenever the movement kind gains a
  field. Today's frozen soldier.json has the 15 fields; step 1's has 28 (11.4), so the re-freeze is part of 11.8.
- N58. clippy runs at `all = warn` (Cargo.toml `[workspace.lints.clippy]`), whose `too_many_arguments` fires above 7
  (clippy's default); today's `LocalMatch::new` has 5 (local_match.rs:58-64).

## Block 7. WORLD-ROOM, one written answer: the match's constants, the state, and their tests (the advisor agent's F1) (DONE)
Against cs/4c-soldier at 695b45b9 (main's Rust, dae6d107). Counts are my estimates of written new lines, code / tests,
plus or minus 30 %. Part A is the ruling rust-reviewer confirms or refuses; Part B holds what it surfaced, each with
its owner, so a disagreement there cannot hold step 1's code on Part A.

### Part A. The ruling (the default: my block 4 and the sheet's E, :313-319), for rust-reviewer's yes or no
- **A1. `World` is the state and nothing else.** The match's `Rules`, its `StaticWorld` and its spawn points are
  constants of the match: built once from the content when the match starts, never changed, never copied with the
  state, restored or hashed. They live in the `Room`, which owns the `World` beside them (room/src/lib.rs:87-88), and
  are lent to the `World` per call. Not in `World`: ADR 0004 part 9 hashes "a canonical serialization of its whole
  state" (:342-344; netcode-architect's "whatever the World keeps is hashed", its read :83-84), and part 3 copies the
  state for re-simulation (:161-165), so a constant inside would either be hashed with content the digest already names
  or ride every copy unhashed beside the state (N47). Not in `LocalMatch`: the room is also the native server's match
  (block 4).
- **A2. Read against ADR 0004 part 1** ("the static colliders with their ids" among the state's tables, :135): the
  immutable static world is no state (parts 3 and 10, :163-165, :365-367; collision.rs:1-3); a collider's changing
  state (a destructible's, none before then) will be a table in the state. Part 1's "nothing lives outside the struct"
  bans Rust `static`s (PM-0004-17, :1138, two_instances.rs), which the room's owned values are not (N46).
- **A3. Per call, never kept:** the room owns both the `World` and the constants, so a reference kept inside `World`
  would point into its own owner (self-referential: not expressible in safe Rust without a lifetime on `World` that
  the room could not satisfy). `World` stays without a lifetime parameter, and the borrow ends with each call. Room's
  `step` lends disjoint fields of itself (`&self.rules`, `&self.statics`, `&mut self.world`), which compiles as is.
- **A4. Who builds them, once per match:** the constructor that loads the content, `Sim::create`
  (sim-wasm/src/lib.rs:105-126, today's `local_world` :83-86), through sim's own builders:
  `StaticWorld::build(&MapDocument)` (collision.rs:212, exists), `Rules::take(&Movement)` (11.4) and
  `spawn_points(&MapDocument)` (B3 below). On the server later, the native match start does the same. The room and the
  client endpoint receive sim types only, so no `[dependencies]` line changes (block 4; N54).
- **A5. The plain form is the default:** three fields of `Room` and three arguments, as the sheet has it. Considered and
  not taken: one `MatchConstants { rules, statics, spawns }` value (Room::new 3 arguments, LocalMatch::new 5, about +15
  code); it is a symbol beyond the sheet, so it comes only as a plan delta if rust-reviewer asks for it.

### Part B. What the ruling surfaced, each ruled or given a default with its owner
- **B1. The record is a `PlayerRecord`, not a `PlayerCommand` (ruled: item 1's "nothing else"; N48).** A
  `player: PlayerCommand` field would keep the newest command's move and aim in `World` unhashed once `PlayerCommand`
  gains them. `PlayerRecord { buttons: u16, presses: [u8; BUTTONS] }` with `NONE` (netcode-architect's name, its read
  :78); the fold copies the newest command's buttons and presses into it (today's world.rs:150-152). Its readers move
  with it: room/src/lib.rs:286-296, :461, :496; sim-wasm local_match.rs:267-268, :336, :370. About +12 code / 0 tests
  (this block's T-A covers it). Owner of the confirmation: rust-reviewer and netcode-architect (record (i)).
- **B2. The query counts leave `World` too (ruled: item 1; N49).** `QueryCounts` is a run's tally, not state: the room
  holds `query_counts: QueryCounts` beside `counters` (room/src/lib.rs:99) and lends it `&mut` to the step; read
  through `Room::query_counts()`. A restored or copied `World` (change-set 7) then carries no tally to double or reset.
  Supersedes block 2's "in the scratch the step already owns ... a `#[doc(hidden)]` accessor on the world"
  (rust-reviewer-read.md:381-385). The step's other scratch (the push sums, 11.1 item 3) is a local array inside the
  step, no field. About +6 code.
- **B3. The spawn points' type (a gap in 11.6, which names `spawns` with no type).** `SpawnPoint { at: V3, facing:
  [f32; 2] }` in soldiers.rs, and `spawn_points(map: &MapDocument) -> Vec<SpawnPoint>` (at most 16, map.rs:27; built
  once, so the `Vec` is no step allocation): ranked by id, as the shapes are (collision.rs:13-14), so the table does
  not depend on the file's list order; `at` narrowed by the corners' own rule (collision.rs:101-108's `narrow`, made
  `pub(crate)`: no minus zero); `facing` by `fmath::facing_of` (11.5; the scan's allowance stays (facing_of, sincos)).
  About +20 code.
- **B4. When the one soldier is made, and at which spawn (default given; owners netcode-architect for the when,
  rust-reviewer for `Soldiers::spawn`; game-designer to say it is no rule of his; N53).**
  - (a) default: `World::new` makes slot 0's soldier at the table's first point (the lowest id: `street_1`,
    (-20, 0, -0.9), facing 90, on the test scene; the same point the file lists first), stamina full from `Rules`,
    `aim_cm` NONE; an empty table (a map built in code only: map.rs:29, :341-349) makes none. A stand-in until LOOP_V2
    section 2's barracks and ADR 0004 part 17's spawning phase (:489, :501-503), which are change-set 5's and the loop's.
  - (b): the soldier is made inside the step at the tick the join is taken (spawning as a phase, part 17); the step then
    borrows the spawns too and takes a join signal that netcode-architect defines. More code now (about +25), and
    nothing in 4c needs a soldier absent before the join.
- **B5. Which movement document the match takes (it blocks step 1's code whatever Part A says; N52).** Constraints:
  works on the test bundle (`test-soldier`), the frozen first-loop bundle and the repository's (`soldier`); no change
  to the recording's header or format; no batch file touched.
  - (a) `LOCAL_SOLDIER = "soldier"` beside `LOCAL_MAP` (sim-wasm/src/lib.rs:23-25), `CreateError::SoldierMissing`:
    the test bundle fails it, so content's movement `TEST_TEXT` and the test bundle take the id `soldier` and the lines
    that name "test-soldier" with it (up to 49 lines in 7 files, packages/types/src/index.test.ts among them). About
    +8 code, up to 49 lines edited.
  - (c) **default:** the local match plays the bundle's one movement document, refusing a bundle with none or with
    several (`CreateError::Soldier { found }`, its message in plain words); all three bundles hold exactly one today
    (bundle.rs:260-265; content_files.rs:363-366; first-loop.content.json's paths), and so does every other bundle
    written in code that reaches `Sim::create` or `Replay::new` (grep of `bundle::write(`: sim-wasm/src/lib.rs:357, :388,
    :419 and devtools replay.rs:367, :394, one movement each, each refused before the movement check by the content,
    the map or the digest; content_files.rs:110 writes the repository's content); and bundle.rs:307 already says
    "which files a match needs is the match's to say". A mode that names its soldier comes with the change-set that
    brings a second movement document, as `LOCAL_MAP`'s doc says of maps (:23-24). About +10 code / +10 tests.
  - Not (b) `LOCAL_SOLDIER = "test-soldier"` (the repository's and frozen bundles fail), nor (d) an id in the
    recording's header (a format change).
  - Owner: rust-reviewer confirms in its table (it lands in sim-wasm's constructor, the boundary I hold).

### Item 1. What `World` holds after step 1
`World { tick: u32, player: PlayerRecord, soldiers: Soldiers, seed: [u8; SEED_BYTES] }`, and nothing else.
- **Shared, hashed by both hashes:** the tick, the record (buttons and presses), the 48 rows in slot order
  (`visit_shared`, 2,902 bytes: 11.2).
- **The authority's part:** the seed (world.rs:116, :173), hashed by `state_hash` only, never by `shared_hash`
  (world.rs:208-224). The brief's "the soldier table, the record, the tick, and nothing else" is the shared part; the
  seed stays where it is (not a constant of the content: the setup's, and the random streams' positions join it).
- **Not in `World`:** `Rules`, `StaticWorld`, the spawn points (Part A), the query counts (B2), the newest command's
  move and aim (B1; they live in the row as `held_move` and `aim_cm`, 11.2).

### Item 2. Where the three live, their types, who builds them
| Constant | Type | Held by | Built by, once per match |
|---|---|---|---|
| rules | `sim::rules::Rules` (11.4) | `Room.rules` | `Rules::take(movement)` in `Sim::create`, movement per B5 |
| statics | `sim::collision::StaticWorld` (collision.rs:94-99, `Clone`) | `Room.statics`; from step 2 the endpoint's own copy, `statics.clone()` made in `LocalMatch::new` (block 4; netcode-architect-read.md:99-101) | `StaticWorld::build(map)` in `Sim::create`, map `LOCAL_MAP` |
| spawns | `Vec<sim::soldiers::SpawnPoint>` (B3) | `Room.spawns` | `spawn_points(map)` in `Sim::create` |
- `LocalMatch` holds none of them: its `world` field and `static_world()` go (local_match.rs:48-50, :96-100), as 11.6 has.
- The client endpoint holds none in step 1 (endpoint.rs:90-110 unchanged; 11.6's :258 only); its copy of the static
  world is step 2's, its `Rules` copy change-set 7's prediction.

### Item 3. The signatures
- `World::new(setup: &MatchSetup, rules: &Rules, spawns: &[SpawnPoint]) -> World` (no longer `const`; borrows both for
  the call; B4 (a)). Under B4 (b): `World::new(setup: &MatchSetup) -> World`, as today.
- `World::authority_step(&mut self, rules: &Rules, statics: &StaticWorld, counts: &mut QueryCounts,
  commands: &[PlayerCommand])`: every borrow lasts the call only (A3). Under B4 (b) it also takes
  `spawns: &[SpawnPoint]` and the join signal; under either, the spawns are never kept.
- `Room::new(setup: &MatchSetup, content_hash: [u8; 32], rules: Rules, statics: StaticWorld, spawns: Vec<SpawnPoint>)
  -> Room`: owns them; total, as today (block 6.2: no refusal of a world with no aim box). Added: `Room::statics(&self)
  -> &StaticWorld` (replaces `LocalMatch::static_world`, one test reader) and `Room::query_counts(&self) ->
  &QueryCounts`. No accessor for `rules` or `spawns` (no reader).
- `LocalMatch::new(setup, first_sequence, pad_bytes, link, rules: Rules, statics: StaticWorld, spawns:
  Vec<SpawnPoint>) -> Option<Self>`: 7 arguments, at clippy's limit and not over it (N58); step 2 adds none (the
  endpoint's copy is cloned inside).
- `Sim::create(seed, mode, pad_bytes, bundle)` and the bindings' `Sim::new` (sim-wasm/src/lib.rs:105-110, :145): no
  signature change (WB-HEALTH, :5-7). The body: load, the map (`MapMissing`), the movement (B5), `StaticWorld::build`
  (`World`), `Rules::take` (new `CreateError::Rules(RulesError)`), `spawn_points`, `LocalMatch::new` (`PadBytes`), in
  that order. `local_world` becomes `local_constants(set) -> Result<(Rules, StaticWorld, Vec<SpawnPoint>),
  CreateError>`.
- `Endpoint::new(content_hash, first_sequence, pad_bytes)` (endpoint.rs:117): unchanged in step 1.

### Item 4. What the hash sees, what the digest names, and the replay
- **The hash sees none of the three** and cannot: `visit` and `visit_shared` are methods of `World` with no argument
  that reaches the room, and `World` holds no constant (item 1). The state still holds values a constant set at spawn
  (the spawn's `at` and `facing`, a full stamina): those are state, hashed as the row.
- **What the digest names:** the content's bytes, and through them the content. The loader takes a bundle only byte for
  byte in canonical form (bundle.rs:3-11, :88-108), so one content set has one byte string and one sha256 (N50).
- **Where 4c checks it:** a recording's header (its content hash) and the recordings' manifest (manifest.txt:1). The
  replay refuses any bundle whose sha256 is not the header's, before the match is made (replay.rs:37-54), then makes the
  match through `Sim::create` on that bundle (replay.rs:119-123), which builds the three anew; nothing of them is read
  from the `.rec` or `.hashes` files. ADR 0004 part 9's recorded match is exactly "the seed, the map and mode data, the
  content data's hash and the per-tick inputs" (:350-351). **Not checked in 4c:** the live join, whose content hash is
  zeros on both sides until change-set 8 (local_match.rs:20-22, :66-67; room/src/lib.rs:149); in local play one bundle
  crosses once, into `Sim::create` (sim-wasm/src/lib.rs:96-99), so the room and step 2's endpoint copy come from it.
- **What the digest does not name: the code that derives the constants.** Held by: the static world's committed trees
  and rays, natively, in Node and in the browsers (static_world.rs:1-5, PM-0004-32); 11.7's tests 2 to 4 (`Rules::take`
  field by field and the shares within 1 ulp, `facing_of` by bits, the exp and sincos vectors on every target); and the
  recordings, which move at the first tick a changed derivation acts. A rule that no recording exercises is held by
  tests 2 to 4 alone.
- **The re-freeze:** first-loop's frozen bundle holds today's 15-field soldier.json (N57). 11.8 must name
  `make_recordings --freeze first-loop` after content's 13 new values land, then the plain run, in the same commit, so
  the header's hash names the new bundle and the replay can build `Rules` from it.

### Item 5. Changes to rust-reviewer's block 11.6, by number, for block 14 to confirm
1. world.rs, `World`: gains `soldiers` only. Strike "and the match's constants (`rules`, `statics`, `spawns`), which
   are not state" (rust-reviewer-read.md:904).
2. world.rs, ADD `PlayerRecord { buttons, presses }` and `PlayerRecord::NONE`; `World.player: PlayerRecord`;
   `World::player() -> &PlayerRecord` (B1).
3. world.rs, CHANGE `World::new(setup, rules, spawns)` (B4 (a)).
4. world.rs, CHANGE `authority_step(&mut self, rules, statics, counts, commands)` (item 3).
5. soldiers.rs, ADD `SpawnPoint` and `spawn_points` (B3).
6. collision.rs, CHANGE `narrow` (:103) to `pub(crate)` (B3).
7. body.rs, `QueryCounts` stays; REMOVE block 2's accessor on the world (B2).
8. lib.rs, the re-exports gain `PlayerRecord` and `SpawnPoint` (beside :26-28).
9. room/src/lib.rs: `Room` gains `rules`, `statics`, `spawns`, `query_counts`; `Room::new` per item 3; ADD `statics()`
   and `query_counts()`; :282 lends the four; :286-296 reads the `PlayerRecord`; its tests build the three once from
   the test bundle (the batch's dev-dependency) and clone them per room; room/tests/any_bytes.rs:117 the same, once per
   run, cloned per case.
10. sim-wasm/src/lib.rs: `local_world` becomes `local_constants`; `CreateError` gains `Rules(RulesError)` and B5's
    variant; the test :401-410 reads `local().room().statics()`.
11. sim-wasm/src/local_match.rs: `LocalMatch::new` per item 3; `world` and `static_world()` removed (as 11.6); its tests
    (:203-215) build the three.
12. sim-wasm/tests/step_alloc.rs:81-86, sim/tests/state_hash_alloc.rs:70-94, devtools/tests/hash_cost.rs:40-46: build
    the three and pass them.
13. client-endpoint: nothing beyond :258 in step 1.
14. content: nothing for B5 (c); under (a), `TEST_TEXT`'s id and its lines.
15. devtools/tests/layering.rs (the batch's F2): its "no file under src/ names `content`" reads non-test tokens only
    (rules_scan.rs:695-718's test-only marking, reused), since room's unit tests in src/lib.rs (:304-669) build the
    constants from content through the batch's dev-dependency; the same line inside `#[cfg(test)] mod tests` is its
    control (passes), outside it its plant (red). Supersedes my block 4's wording (:550-554).
16. 11.7 gains rows T-A to T-F below; 11.9 about +40 code / +130 tests for B1 to B3, B5 (c) and the tests (rust-reviewer
    recounts).
17. Lines outside 11.6 that restate the old placement, superseded: rust-reviewer-read 1.6 (:244-246, :251-254: "into the
    room's `World`"), its block 2 (:381-385), netcode-architect-read :675-677 (`read_shared(bytes, constants)` becomes
    `read_shared(bytes, setup)`).

### Item 6. The tests that hold the placement, each with a plant written against the code as built
| # | Test | Plant (red, deterministically) |
|---|---|---|
| T-A | **The state rule** (sim/tests/rules_scan.rs, its tokenizer): in world.rs, every field of `struct World` is bound by name in exactly one of `visit` and `visit_shared`, and `visit_shared`'s pattern holds no `_` but `seed: _` (nested patterns included). It is the one test that sees a non-state field parked in `World`, which every hash, names, restore and two-builds test passes | (1) `counts: QueryCounts` added to `World`, written `counts: _` in both patterns (the defect F1 is about); (2) `player: PlayerCommand` with `move_x: _, move_y: _, aim: _` in `visit_shared`; (3) a field bound in both visitors. Control: the built `World` passes |
| T-B | **One bundle, two builds** (sim/tests, at the `World` level, where step 1's `PlayerCommand` carries the move whatever the wire does before step 2): two worlds, each from its own `content::load` and its own three builds of the test bundle, the same 300 scripted commands (move, the sprint button, jump presses, onto the 0.55 m box of map.rs:412): per-tick shared and whole hashes equal; the two `tree_bytes()` equal, the two `Rules` equal, the two spawn tables equal | a `static` counter in `spawn_points` that moves each later build's `at` x by 1 mm per build made before it (PM-0004-17's bite; it bites on the test map's one spawn): the second world's soldier starts 1 mm off, the hashes differ at tick 0 |
| T-C | **A rule's value changes, the bundle is the same** (world.rs's test module, since the mode's constructor is `#[cfg(test)]`): the test bundle, `Rules` under both HEADROOM-FEET modes (11.4 as block 13 has it), T-B's 300 commands on the test bundle's one-box map (map.rs:412: one 0.55 m box on the floor, no floating shape, so by block 13's own criterion, B13-3, the modes cannot differ there; my reading, not run): equal hashes at every tick | the mode written into the row at spawn (a `flags` bit set from `Rules`): the hashes differ at tick 0 |
| T-D | **The bundle decides** (devtools replay.rs tests, through `Sim::create`): a short recording made on bundle X replays to its hashes; the same inputs on bundle Y (X with `jump_reach_m` changed, a rule a button reaches: until step 2 the replay refuses any move, replay.rs:81-84, and `Sim::input` takes six numbers, sim-wasm/src/lib.rs:166) with the header naming Y's digest equal X's hashes until the first jump's tick and differ from it on | `Sim::create` taking its movement from content's built-in `TEST_TEXT` instead of the bundle: Y replays equal to X |
| T-E | **Built once:** sim-wasm/tests/step_alloc.rs (the counting allocator around `LocalMatch::step`, the whole exchange the room's step included, step_alloc.rs:1-4) gains a plant; 11.7's test 24 wraps `authority_step` below the room and would not see it | `self.statics.clone()` in `Room::step` every tick (the room holds no `MapDocument` to build from): allocations counted every tick |
| T-F | **The state alone restores:** netcode-architect's restore test (11.7 test 22) with `read_shared(bytes, setup) -> World`, the step lent the same three | its plants as written (P1 to P3); the reader takes no constant, so B is made from the bytes and the setup alone (T-A is the structural guard) |
| T-G | layering.rs's control (item 5, point 15) | `use content::map::MapDocument;` in a copy of room's non-test source |
About 0 code / 130 tests (T-A 40, T-B 35, T-C 20, T-D 30, T-E 5); T-F and T-G are counted in their rows already.

### Q4 (advisor-steps1to7.md:136-158): does this answer touch its conditions?
- **Condition 2 (the batch lands whole):** the five batch files stay byte for byte (no manifest line changes: the
  dev-dependencies are the batch's own, deps-4c-v2.diff:59-82). One rewording: layering.rs's non-test reading
  (item 5, point 15) is F2's test, so security-auditor sees it when it reads the batch's commit (F9). The first callers
  of exp and sincos are `Rules::take` and `spawn_points`, both in sim; the allowances stay in fmath.rs.
- **Condition 3 (the record rule and NONE land whole in step 1):** B1's `PlayerRecord` is part of the record rule, so
  it lands in step 1 too.
- **Condition 4 (every commit that changes the hashed state re-makes the pins):** holds, and reaches one more case: the
  known answers (world.rs:401-415) now depend on the constants (the spawn row: the test bundle's spawn `a` at
  (0, 0, -5) facing 90, map.rs:412; its stamina from movement `TEST_TEXT`, movement.rs:107, which gains 13 fields in
  step 1). world.rs's tests build on `content::bundle::test_bundle()` (sim depends on content), so a change to the test
  bundle or to a derivation moves them, and that commit re-makes them by the second implementation.
- **Conditions 1, 5, 6, 7, 8, 9:** not touched.

### Not verified (block 7)
- Read only: nothing built or run. "Compiles as is" (A3, disjoint field borrows) and "7 arguments pass clippy" (N58)
  are from the language's and clippy's documented rules, not from a build.
- T-A's scan is design: whether the tokenizer's patterns read nested `_` bindings as planned is unbuilt.
- The 49 "test-soldier" lines (B5 (a)) are a grep count; which of them must change was not judged line by line.
- B4 and B5 are defaults for their owners; game-designer has not said B4 (a) is no rule of his.
- Whether step 1's room carries the wire's move into `PlayerCommand` (11.1 item 1 says the fold takes each command's
  move; netcode-architect's read lists the room's move check under step 2): not settled here, so T-B drives the
  `World` and T-D uses a jump, which reach the step without it.
- The TypeScript tests and e2e specs that create a `Sim` were not grepped for their bundles; they load the built
  content.json of `content/`, which holds one movement (content_files.rs:363-366), as far as read.
- B7 DONE.

DONE
