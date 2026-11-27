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
- **How the camera joins without a second change of the method:** `PictureContent` is a record, so step 5 adds one
  field, `readonly camera: <the camera kind's generated type>`, with its reader (the follow camera) in the same step.
  The method, its one caller and the hand-off stay as they are; only `readContent` reads one more file. That is the
  cheap route. Not taken: an optional `camera?` now (no type exists before step 5's camera kind, and a field with no
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
  `"__shambleScene"` (boot's block) and `"scene-inventory"` (a string that only inventory.ts holds, its result's tag),
  so the production scan (build-checks.mjs:242-245) refuses a production build holding either and a development
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
  runs only when a test or the readout asks (four times a second at most, development only: the readout's counts come
  from one call per rewrite, boot.ts:321-324). Evidence: allocation.spec.ts on base and head, its per-frame bytes
  unchanged within its stated noise (allocation.spec.ts:18-26).

### 1.9 The follow-ups placed
- **4B-TYPES-READER** (STATUS_ARCHIVE.md:2280-2281): met here. `readMap` is the first reader of `MapDocument`,
  `Bounds`, `Spawn`, `Shape`, `Toward`, `Region` and `Ground` (packages/types/src/index.ts:16-23), typed as returning
  `MapDocument` with no cast, so tsc holds the reader's output to the generated type. `Movement` keeps no page reader
  (only its file's header is read, for the readout): said in the report, and its first reader is the change-set
  whose page needs a movement number (gap 10, design 3.8).
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
| **Add** | picture.ts: `PictureContent`, `Picture.setContent`, `SceneObject`, `Picture.inventory`. render/scene.ts (new): `buildEmptyScene` (moved; now returns `{ scene, camera, floor }`), `buildMapFloor(map)`, `solidPoints(shape)` (the 8 or 6 corners), `greyOf(top, lowest, tallest)`, `groundGrey(kind)`, `disposeFloor(floor)`, `overview(bounds, aspect)`. render/inventory.ts (new): `readInventory(floor)`, `INVENTORY_TAG = "scene-inventory"`. app/content.ts: `MAP_FORMAT`, `MAP_VERSION`, `MOVEMENT_FORMAT`, `MOVEMENT_VERSION`, `CONTENT_KINDS`, `NOT_THE_MAP`, `ContentFile`, `ReadContent`, `readMap`, `readContent`, private `closed`, `corner`, `word`, `text`. sim-worker/src/index.ts: `LOCAL_MAP` re-exported. overlay/report.ts: `ContentRows`, `ReportData.content`, `ReportData.scene`. boot.ts: `window.__shambleScene` (development block). build-checks.mjs: two `DEV_MARKERS`. render/scene.test.ts (new). |
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
| E4 | the production build holds neither new marker, the development build both | build-checks (every build) | `readInventory` called outside the define |
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
  TUNE-HOME acts on the camera's content, which joins at step 5.)
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
| **4c-2, "the soldier's simulation"** | step 1 whole; step 2 but for RESUME-FLASH's page guards and test (N18); **step 3's snapshot part** (`crates/protocol/src/`: the own soldier record and its codec; the room's encoder; the endpoint's decoder and the parity test); step 6 whole; step 6c whole | the soldier table, the rules taken from content, our own movement on the collision world, the post carrying the move and the ray, the 13-number input call, the endpoint's cast and the room's checks, the jump rule, the snapshot's own soldier, the movement and lane recordings with every mark that rides them, the first recording re-made, the hostile commands, the far-corner map |
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
- **The never-moved list holds** (plan.md:249-251): the movement recording's four runs, the stale-id run, the hostile
  commands and the parity marks in 4c-2; the paced replay and the camera panel in 4c-3; steps 4 and 5 in one half,
  so the panel is in the first build he plays.

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
| C6 | The camera's content file is placed in step 5, but its first reader, the follow camera, is step 4's | the camera kind moves to step 4 (3.5); the panel stays step 5. Both are in 4c-3, so the seam is untouched |
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
- **The five presets:** legacy CAM_STYLES sets 6 of the 21 values per preset (N21). Recommended: a `presets` list in
  the camera kind (data, held to the same ranges by the loader), its five rows written by script from
  docs/legacy-tuning.json, never typed; a preset sets those values and leaves the others. Not taken: five whole
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
  `PictureContent` gains it in step 4; under (c) the height is not among the dropped rules, so as (b). Until the
  field exists, the stand-in cannot take its height from content, and a 1.80 m constant in the page would be a
  second home.
- **MOVE-RULES:** no symbol of steps 3 to 5 changes by its rows (rows 6 and 7 are 4d's; the others act inside the
  simulation).
- B3 DONE. Next: B4, 4B-WORLD-ROOM.
