# 4c, "The first soldier": the sheet for the advisor agent's re-read, revision 1 (2026-11-28, main dae6d107)

_Written by the main session (Builder V6, lane 1) from the approved walk plan (`.checkpoints/cs4-walk/plan.md`,
revision 4: section 6 and the sections it names), three reads against today's code (`engine-architect-read.md`,
`netcode-architect-read.md`, `rust-reviewer-read.md` in this folder) and his words. The walk plan's section 3, item 1:
"Each later part starts only after the advisor agent has read this plan again against that day's main ... That
re-read carries the part's exact symbols to add and to remove, taken from the designs' tables as they stand that
day." **No file outside `.checkpoints/` is edited before the advisor agent's verdict.**_

**Status of this revision: complete for the advisor agent's review** (2026-11-28, 00:59 Amsterdam, the commit's clock
time of 78fd901e), from four
finished reads: engine-architect, netcode-architect and rust-reviewer (DONE), and game-designer's on BODY-SHAPE and the
push's numbers, still running: its result comes as an addendum, and the movement step is held for BODY-SHAPE in any
case.

## A. Since the plan was approved (2026-11-25)
- **Merged:** 4a (pull request #70, main 2483e9a), 4b-1 (#75, b9aad01), 4b-2 (#79, dae6d107, at head b7145273 on his go
  of 2026-11-27 22:50:52 UTC). Written lines against the designs' count: 4a 3,300 (2.4 times), 4b-1 7,164 (2.17),
  4b-2 8,408 (2.7).
- **His answers since then that reach 4c** (each word for word in `docs/DECISIONS.md`):
  - STEP12-GO (a): 4b's step 12, the page's scene with the map's reading field by field, is 4c's first building step.
  - GREY-COLOURS (a): the scene's colours are LOOP_V2 section 9's default, "plain blocks, with height read through
    brightness (higher is lighter)", in neutral greys; the stand-in is one mid grey with a darker gun.
  - STEP-60 (a) and DIFF-LIST (a): the movement values are his Export at 30 steps, already in
    `content/movement/soldier.json`.
  - FEATURE-COMPARE (a), condition 4: "the recorded match exercises the collision queries from the first change-set
    that calls them, so native, Chrome and Firefox are compared on the real thing and not only on 10,000 rays": 4c is
    that change-set (follow-up 4B-QUERY-RECORDING).
  - DEPART-4 (a) with its two notes; EARLY-LOOK-ORDER (b); SIZE-4B's point 2 as a standing habit: before step 1 of
    each part, the card names what leaves first if the count runs high, and he hears in one sentence where the count
    stands at two thirds of the stop line.
- **Asked on 2026-11-28, open** (`question-2026-11-28-before-4c.md`): MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME
  (the plan's four before 4c), SIZE-4C, BOARD-WORDS, PASTE-BREAKS, DESIGN-CLASH.
- **4b-2's follow-ups placed in 4c** (`docs/STATUS_ARCHIVE.md` lines 2278 to 2300): 4B-TYPES-READER, 4B-MEMBER-KINDS
  and 4B-BOOT-CALLS at step S; 4B-QUERY-RECORDING, 4B-RAY-COUNTS and 4B-RAY-WASM at steps 1 and 6; 4B-WORLD-ROOM at
  steps 1 and 2; 4B-LIMIT-IMPORT at step 5; 4B-CHROMIUM-TIME is a risk (section G). The advisor agent's acceptance of
  security-auditor's two developer-machine risks goes into STATUS as a dated note at 4c's docs pass.

## B. The steps, in order, and what each waits for
| Step | What | Waits for (his open answers) | Designs' count (code / tests) |
|---|---|---|---|
| 0 | The first docs commit: his words of 2026-11-27 into the register; LOOP_V2's section 14 and its pointers | the advisor agent's APPROVE; PASTE-BREAKS (section 14), BOARD-WORDS (his message of 23:59), DESIGN-CLASH (two lines) | docs only |
| S | The page's scene, with the map's reading field by field (4b's old step 12) | none of the movement questions (GREY-COLOURS is answered; STREET-GREY is a default he can veto); SIZE-4C sets its stop line and when its reviews run | 362 / 378 (engine-architect's recount; the first sheet said about 300 / 330) |
| 1 | The simulation's movement | MOVE-RULES (rows 1 to 5 and 8), NO-SOURCE | 1,240 / 1,090 |
| 2 | The command path | pending netcode-architect's read | 385 / 530 |
| 3 | The snapshot's soldier and the block | pending the reads | 320 / 340 |
| 4 | The page: the stand-in, the follow camera, input, the aim line, the stamina bar | RESUME-FLASH | 800 / 540 |
| 5 | The camera panel and the camera's content file (the early look after it) | TUNE-HOME | 360 / 300 |
| 6 | The movement and lane recordings, and every mark that rides them | step 1 | 320 / 230 |
| 6b | The paced replay in the real Worker | step 6 | 130 / 110 |
| 6c | The hostile commands; the far-corner map | step 2 | 30 / 180 |
| 7 | The page's browser tests | RESUME-FLASH (the hidden-tab test) | 0 / 410 |
The counts are the plan's; the reads recount them against today's code (section E).

## C. Step 0, the first docs commit (exact)
After the advisor agent's APPROVE. It waits for PASTE-BREAKS, BOARD-WORDS and DESIGN-CLASH, since nothing in step S
needs it; if his answers come after step S's code, step 0 still lands before any other docs commit of 4c.
- **`docs/DECISIONS.md`**, newest first, by a script (`decisions-4c.mjs` in this folder: 4b-2's `decisions-4b2.mjs`,
  readable at 59f6cc7, with these entries and the same `write` and `check`), from the saved files, never typed:
  1. `cs4-walk/mustafa-2026-11-27-go-merge-79.md`: his go for the merge of 4b-2 at b7145273 ("you have my go").
  2. `cs4-walk/mustafa-2026-11-27-may-merge.md`: his word that the chat may merge it ("You can merge it").
  3. `cs4-walk/mustafa-2026-11-27-design-notes.md`: his design notes on resource points and repair bots, whole.
  4. His message of 23:59 Amsterdam (MERGE-ROUTE (a), how 4c starts), only as BOARD-WORDS says.
  STEP12-GO is already there (`docs/DECISIONS.md:183`): the script refuses a message that is in the file.
- **`docs/LOOP_V2.md`** (his items 2 and 3, placed by docs-keeper, which writes no design of its own):
  - Section 14 after section 13: the saved message's lines 61 to 128 (from "## 14. Resource points and repair bots"
    to the line before "9. "), cut by `chat-words.mjs part` and never typed. By PASTE-BREAKS (a): each line "- " joined
    to the line after it and each line "  " to the line after it, nothing else; a check proves that the placed text's
    words and signs are the message's, in order.
  - The dated pointer "(2026-11-27: resource points replace open fields, see §14)" at the lines of sections 1, 3, 9
    and 11 that speak of the harvest field in the centre or the safe field near each base: line 15 (section 1), 32
    and 34 (section 3), 150, 154 and 156 (section 9), 181 (section 11). The old text stays.
  - The header's "Updated" line (line 5) gains one sentence that names section 14.
  - By DESIGN-CLASH: `docs/EDITOR_V2.md:45` and `docs/LOOP_V2.md:52` as he answers.
- **`docs/adr/README.md`** (the register): one dated line that names LOOP_V2's section 14.
- **Evidence:** `decisions-4c.mjs check` (every quoted line byte for byte, each saved file against its own size and
  sha256); the section-14 check; `git diff --stat` touching only those three files (four with EDITOR_V2); the gate.

## D. Step S, the page's scene (exact): engine-architect's block 1 (`engine-architect-read.md`, lines 32 to 298)
The read's block 1 holds every ruling with today's file:line; this section is what the build carries out.
- **The interface file** (`picture.ts`, engine-architect reviews every change): `PictureContent { readonly map:
  MapDocument }` and `Picture.setContent(content)`, called once per match from the content's hand-off on the Worker's
  `ready`; the picture builds the map's floor, swaps it for what it drew before and keeps no reference to the
  document. The camera joins at step 4 as one more field of `PictureContent`, with its reader, so the method does
  not change again. `Picture.inventory(): readonly SceneObject[] | null` (`SceneObject { id; kind: "ground" | "box" |
  "ramp" | "region"; points; grey }`), `null` in a production build; its sentence says it is lane 1's test hook,
  kept answering by whoever owns `render/` while the test "the scene equals the map" stands, and retired with the
  greybox by the change-set that replaces the look.
- **The map's reading** (`app/content.ts`, the advisor agent's conditions of 2026-11-27, `advisor-cutC.md` B3 Q2 at
  59f6cc7): `readMap(tree: unknown): Outcome<MapDocument>`, closed key sets for every object kind, members by kind,
  own properties only, no cast and no spread; `readContent(kept): Outcome<ReadContent>` (`readBundle` unchanged, then
  each file's own format and version against its folder's kind, then `readMap` on `maps/${LOCAL_MAP}.json`, with
  `LOCAL_MAP` re-exported by `@shamble/sim-worker`'s index); `ReadContent { picture; files }`, `ContentFile { path;
  format; version }`. The page checks the shape the generated type names, never the loader's ranges (a second copy
  would drift). A failure is a fixed sentence (`NOT_A_BUNDLE`, and a new `NOT_THE_MAP`), never a key, a value or a
  parse error's text; nothing of a failed reading is kept; the match ends through `link.end` as today, the empty
  floor stays.
- **The hand-off:** `new PageContent(deliver, read = readContent)`; `afterMessage` reads once on `ready` and calls
  `deliver` once; the copy is let go as today. boot.ts gives `deliver = (read) => picture.setContent(read.picture)`
  (and, in the development block, keeps `read.files` and the scene's counts, taken once at delivery).
- **What is drawn** (`render/scene.ts`, replacing `empty-scene.ts`, whose code moves unchanged as the empty floor):
  our own geometry from the corners, never a transformed box, so each drawn corner is `Math.fround` of the map's
  number, the simulation's corners bit for bit (`narrow`, collision.rs:103-108); a box's 8 corners and 12
  triangles; a ramp's 6 corners and 8 triangles, the four floor corners and the two top corners on the side
  `rises_toward` names (as `enter_ramp`, collision.rs:152-200); every triangle faces outward; regions as quads at the
  bounds' floor; the map's ground and a grid of one line per metre over the bounds (113 and 49 lines on the test
  scene); polygon offset for depth as today; spawns not drawn. Greys (GREY-COLOURS (a)): one neutral grey per solid by
  its top, rising from the map's lowest top to its tallest between two named placeholder constants; the ground kinds
  darker than every solid. **The camera:** an overview that frames the whole map from today's side and angle, at any
  aspect, replaced by the follow camera at step 4.
- **The development inventory:** `render/inventory.ts` reads each drawn object's own geometry buffer, name and
  material colour (never a list the builder wrote beside the meshes); `window.__shambleScene` installed in boot's
  development block; two new `DEV_MARKERS` (`"__shambleScene"`, `"scene-inventory"`), so the production scan refuses a
  production build that holds either and a development build that lacks either.
- **The readout:** `ReportData.content` (`"holding"`, `"none"`, or the files with their formats and versions) and
  `ReportData.scene` (the map's id and its counts of boxes, ramps and regions), counted once at delivery and never in
  a frame; two readout lines and their rows in "Copy report"; text through `textContent` as today.
- **`render-counts.json`:** the empty floor's counts stay (the refused bundles' baseline); the test scene's counts
  join beside them, measured in the container on the built development page and never computed into the file
  (expected by arithmetic, the measurement winning: 32 draw calls, 267 triangles, 162 lines, 32 geometries); WebGPU
  stays null until a Brave report.
- **Disposal and allocation:** the old floor's geometries and materials disposed once on the swap (materials made
  per build, never shared across builds); nothing new per frame; allocation.spec.ts read on base and head.
- **Follow-ups met here:** 4B-TYPES-READER (`readMap` is the generated map types' first reader; `Movement` keeps no
  page reader yet, said in the report), 4B-MEMBER-KINDS (each member's kind compared with the schema's, directly),
  4B-BOOT-CALLS (the scene appears only through boot's real calls; a broken Worker leaves the readout's content
  "none"). 4B-CHROMIUM-TIME: the step adds no page load.

**The exact symbols** (the read's table 1.10):
| | Symbols |
|---|---|
| Add | picture.ts: `PictureContent`, `Picture.setContent`, `SceneObject`, `Picture.inventory`. render/scene.ts (new): `buildEmptyScene` (moved; returns `{ scene, camera, floor }`), `buildMapFloor(map)`, `solidPoints(shape)`, `greyOf(top, lowest, tallest)`, `groundGrey(kind)`, `disposeFloor(floor)`, `overview(bounds, aspect)`. render/inventory.ts (new): `readInventory(floor)`, `INVENTORY_TAG`. app/content.ts: `MAP_FORMAT`, `MAP_VERSION`, `MOVEMENT_FORMAT`, `MOVEMENT_VERSION`, `CONTENT_KINDS`, `NOT_THE_MAP`, `ContentFile`, `ReadContent`, `readMap`, `readContent`, private `closed`, `corner`, `word`, `text`. sim-worker/src/index.ts: `LOCAL_MAP` re-exported. overlay/report.ts: `ContentRows`, `ReportData.content`, `ReportData.scene`. boot.ts: `window.__shambleScene`. build-checks.mjs: two `DEV_MARKERS`. render/scene.test.ts (new). |
| Change | three-picture.ts: `createThreePicture` (the floor, `setContent`, `inventory`, the overview in `setContent` and `resize`). content.ts: `PageContent`'s constructor and `afterMessage`, its comment. boot.ts: the hand-off, `data()`, the second hook. report.ts: `readoutLines`, `formatReport`. e2e: skeleton.spec.ts (the counts' read, the broken-Worker tests), content.spec.ts (the refused bundles' counts), report.ts's `Report` type, render-counts.json's shape. content.test.ts, content-ties.test.mjs, content-schema.test.mjs, report.test.ts, build-checks.test.mjs. |
| Remove | render/empty-scene.ts (its code moves into scene.ts unchanged). `readBundle` as `PageContent`'s default reader (it stays, called by `readContent`). |

**The tests, each with its planted defect** (the read's table 1.11; the plants script runs every one): T1 to T8 in
content.test.ts (the reading equals the committed map; closed key sets; members by kind, its plant the cast
`tree as MapDocument`; `rises_toward` only on a ramp; nothing read through a prototype; every refusal exactly one
of the two fixed sentences, none echoing `<img`; `readContent`'s folders, formats, versions and the local map by
path; `deliver` once on `ready`, never on `error`); T9 the constants tied to the Rust files; T10 the member kinds;
S1 to S7 in scene.test.ts (the corners as `Math.fround` with outward faces for a box and a ramp toward each of the
four sides; each shape and region once, no spawn; the greys' order and neutrality; the grid over the bounds; the
swap's disposals; the overview at three aspects; the inventory reads the drawn buffers); R1 the readout's rows; E1 to
E5 in the browser and the build (the inventory equals the built bundle's map and the counts equal the test scene's on
both backends; the seven hostile bundles leave the empty floor and its counts; a broken Worker's content reads
"none"; the production build holds neither marker; allocation on base and head).

**Its size:** about 362 code and 378 tests by the designs' count (about 740, against the 630 this sheet first said),
so about 1,600 to 2,000 written lines at the measured rates: above the 1,420 the step was to be held to until his
SIZE-4C (section H asks the advisor agent about that hold). What could leave first, said to him before it moves: the
overview camera (only under SIZE-4C (a), where the follow camera frames the scene before 4c merges), then the per-file
headers of the readout (the rows would name paths alone).

**A look point the read raises, his to veto (STREET-GREY):** the street and the grass both lie at height 0, so by
"higher is lighter" alone they take one grey and the street does not show, while the plan's section 2 says he sees "a
street". Default: (b) two neutral greys, one per ground kind, both darker than every block; (a) one grey for both;
(c) the regions as outlines in the grid's grey. Asked of him with the next update; the build takes (b) until he says
otherwise, and test S3's assertion on the two kinds follows his letter.

## E. Steps 1 to 7 against today's code
### E1. Step 1, the simulation's movement: rust-reviewer's read (`rust-reviewer-read.md`, blocks 1 to 4, DONE)
The read's block 1 holds every symbol with today's file:line (ids.rs, soldiers.rs, rules.rs, vec.rs and fmath.rs,
collision.rs's sweeps, movement.rs, world.rs and every caller, the allocation test, exp and its vectors); block 2 the
ray counts, 4B-RAY-WASM and the simulation side of 4B-QUERY-RECORDING; block 3 what MOVE-RULES and NO-SOURCE change,
by option; block 4 its risks. What the sheet takes from it:
- **BODY-SHAPE (High; a finding against plan 6.1), for game-designer's read and then for him:** the plan's capsule
  fails PM-0004-30 (the 0.55 m step gained, the 0.56 m step stopped) under every step rule its prototype tried (9 runs
  per step height each: landing on an edge allowed, the 0.56 m step gained; landing on a face only, the 0.55 m step
  missed 6 of 9; stepping from the last face stood on, the 0.56 m step ended perched on its edge 9 of 9). A
  flat-footed vertical cylinder gains 0.55 m and stops at 0.56 m, 9 of 9 each, but stands 0.4 m past an edge and
  floats 0.10 to 0.20 m over the yard's ramps unless the foot has a smaller radius of its own: that changes what
  MOVE-RULES row 2 told him. Untried for every body: ramps, all vertical motion (the in-air step-up with 3.05 m gained
  and 3.10 m refused, landing, the snap, ceilings) and the push. **The movement step waits for BODY-SHAPE.**
  game-designer reads the sandbox's own step rule now (`game-designer-read.md`).
- **SWEEP-ROUTE (a departure from the build plan's Walk card, "our own movement on parry3d's queries"):** recommended
  our own sweeps on today's corners, as the ray already is, with the library keeping its tree and serving as the
  tests' oracle: ties by index, no allocation by construction (the library's cast runs a contact query when a sweep
  starts within 1e-4 of contact, which allocates when the shapes truly overlap), no tolerance-decided ties, no `sqrtf`
  linked. It rewords PM-0004-20 (the tree only) and PM-0004-35 (0 unsupported pairs by construction, with every form
  pair still run once through the library's dispatcher in a test). Measured on boxes only so far. **For the advisor
  agent to rule (section H): accept it as a departure recorded for his acceptance, or keep the library's queries.**
- **The pins move in step 1, not step 6:** world.rs's known answers and the committed first recording change the
  moment the state holds a soldier, so the step that changes the state re-makes them as its own named sub-step (the
  advisor agent's ruling, plan 6.2; netcode-architect agrees), or CI is red from step 1 to step 6. The plan's step 6
  line is corrected by this.
- **libm joins as a direct dependency of `sim`** (exp, once per match, for the two easing shares): `libm = "=0.2.16"`
  in the root's workspace dependencies and `crates/sim/Cargo.toml`, a lockfile edge, the scan's one-file exception,
  one entry in `maths-allowed.mjs` with its vectors (PM-0004-22). **This reopens the dependency hold:** the batch goes
  to security-auditor by its digests before it is committed, pushed or built (his DEP-HOLD). The other way, our own
  exp, departs from ADR 0004 part 7: not recommended.
- **`Rules::take` refuses what the content's ranges let through:** a jump reach below the step height (the take-off
  speed would be the root of a negative); a sprint or refill time that rounds to 0 units. The plan's "3.0700 m" is
  out of date: with his reach of 3.071 m the tick-sampled top is 3.0702 m and the true top 3.0710 m; every jump test
  compares the true top to `jump_reach_m` as read, never to a literal.
- **Open points with an owner (engine-architect, the map kind):** FACING-SPAWN (a spawn faces any degree, which
  needs a sine and a cosine once per spawn: (a) libm `sin` and `cos`, two more reviewed entries; (b) quarter turns
  only; (c) the endpoint draws the spawn's degrees until the first aim); GROUND (no document says the bounds' floor is
  solid ground, and the test scene has no ground shape). The slope's unit: a grade in percent needs no maths routine;
  degrees would need one more (recommended: the grade).
- **NO-SOURCE (b)'s fields** (block 3's table): `body_height_m`, `steepest_slope_pct`, `foot_radius_m` (only under
  BODY-SHAPE's cylinder; not on his question's list), and the push's four: `push_sideways_pct`,
  `push_undo_at_most_pct`, `push_most_m_per_s`, `push_height_gap_m`; their starting values are the sandbox's code
  constants, which game-designer reads with file:line. The Testlines table gains rows with a `field` and no `export`,
  which `testlines.mjs --check` must take, with a planted row that claims an export it does not have.
- **Buildable before his answers, without building anything twice** (block 3): the soldier table and ids with
  PM-0004-15; the vector, the helpers and the scan; `Rules::take` on today's fifteen fields; the jump with its exact
  arc, true top and PM-0004-59, and landing on the bounds' floor; stamina and sprint, wish, speed, easing, facing; the
  sweeps' common part (the grown-box ray, the touching rule, ties, the bounds, the walk, the counts, the ray kinds);
  the world's command and state, its callers, the known answers and the first recording re-made; exp once the
  dependency hold is answered. Waits for MOVE-RULES rows 2 to 5 and BODY-SHAPE: the edge and ramp pieces, the slide's
  speed rule, the step-up and its in-air form, the snap, the walkable test. Waits for NO-SOURCE: the slope, the
  height, the foot, the push's four guards.
- **Its size:** 1,300 / 1,260 by the designs' count on the route of our own sweeps (the plan: 1,240 / 1,090); with the
  ray counts, the sweep proof and NO-SOURCE's fields about 1,430 / 1,370.

### E3 to E5, the page side: engine-architect's blocks 3 and 4 (`engine-architect-read.md`, lines 395 to 565, DONE)
The read's block 3 is a delta against its design 4.1 to 4.10 with today's file:line; what the sheet takes from it:
- **Step 3's layout (crates/client-endpoint):** `ElementType::F32`; `Section.rows` (with `end_of` and the hash's
  stream); sections `own` (1 row of 20 bytes) and `soldiers` (48 rows of 40); `header.soldier_count` (the header 8 to
  12 bytes, `tick` and `layout_hash` unmoved); the soldier row: `id` (slot and generation), position, velocity, yaw,
  pitch, flags (occupied, alive, grounded, sprinting; aiming joins in 4d), team, `ready`, one reserved byte; the
  block 1,988 bytes. **New since the plan:** the stamina bar has no field anywhere, so the owner's section takes
  `stamina u8` (a share of a full bar) and `own_flags u8` (bit 0: sprint locked) in its spare space, the size staying
  20; the lock cannot be worked out from the stamina (run dry, it stays locked until the bar is back at its share),
  so it is a hashed state bit in the simulation and a wire bit, **raised for rust-reviewer and netcode-architect**.
- **Step 3's page part:** `PictureFrame` becomes `{ older, newer, blend, timeMs }` with the blocks as the `DataView`
  the link already makes once per block (a view made per frame would fail allocation.spec.ts's limit); `WorkerLink`
  keeps a ring of three `{ buffer, view, dueAt }` (DEPART-4's three newest blocks); `app/blend.ts`; `dueAt` from the
  pacer's own due time (`afterTick(now, dueMs)`), posted with each block; the Worker's pool from 4 to 16 (3B-POOL-16).
- **Step 4:** picture.ts gains `cursorRay`, `cameraYaw`, `plainLines` (sized for its first user, the aim line) and the
  start option `panelHost`, and `PictureContent.camera`; `soldier/stand-in.ts` and `rows.ts`; `camera/follow.ts`
  from the camera file's values (his Export's framing replaces the old game's: DIFF-LIST); `input/move.ts` (the move
  as one packed word, as the 13-number call has it); `input/pointer.ts` (no mouse button is a game button in 4c: the
  right button is 4d's, so 3B-BLUR-TEST holds W in 4c and gains the button in 4d); `hud/stamina.ts` (a plain
  production meter, hidden when full and not locked); the readout's rows; `DEV_MARKERS` + `"aim-line"`; the
  production scan reads `.css` too and the development rules move to their own file (3B-CSS-PROD). **The camera's
  content kind moves from step 5 to step 4,** where its first reader (the follow camera) is: `crates/content/src/
  camera.rs` (format `shamble-camera` 1, his Export's 21 camera values by their own names and units), the camera file
  written by the Testlines tool from his Export, the bundle's folders, the schema writer, the generated `Camera` type,
  `readCamera` on the page. The scene's overview camera leaves at step 4.
- **Step 5:** the panel (backquote, sliders whose limits come from the generated camera schema through one product
  function, `numberLimits(schema)` moved into `@shamble/types`: 4B-LIMIT-IMPORT); the five old presets as a `presets`
  list in the camera kind, written by script from `docs/legacy-tuning.json` (each sets the 6 values the old preset
  set, never 15 invented ones); "copy values" by one browser-safe copy of the content's canonical rule, which
  `canonical.mjs` then imports. What TUNE-HOME and RESUME-FLASH change, option by option: the read's 3.6 (TUNE-HOME
  (a) has "copy values" print his Export's names instead of the file's bytes; RESUME-FLASH (b) adds a `dark()` call on
  the key diagram when the tab hides).
- **NO-SOURCE's one page consequence:** the stand-in's height and the body's are one number; under (a) and (b) it is a
  movement field the page reads (`readMovement`, the generated `Movement`'s first page reader), so no 1.80 m constant
  sits in the page.
- **4B-WORLD-ROOM (block 4):** the room owns the authority's static world (and the match's `Rules` and spawns), taken
  in `Room::new` and borrowed by `World::authority_step` each tick; never inside `World` (it is not state) and never
  beside the room in `LocalMatch` (the room is also the native server's match). The constructor that loads the
  content builds it, so no `[dependencies]` line changes; **`content` joins the room's `[dev-dependencies]`** for its
  tests, a lockfile change under the dependency hold (security-auditor first); a layering test in devtools
  (`tests/layering.rs`) keeps `content` out of the room's and the client endpoint's manifests and sources, with two
  planted defects. With step 1, the first step that queries the world inside the room.
- **The page side's open points for others:** the zero-ray meaning (netcode-architect: whether an all-zero ray means
  "no ray this post"; needed only under SIZE-4C (c)); step 3's Rust lines split between the snapshot and the block
  (netcode-architect); the sprint-lock bit and the stamina's wire (rust-reviewer and netcode-architect); under (c),
  whether PM-0004-28 ("before 4c merges") may be met in 4c-3 (the advisor agent).

### E2, E3's timing, E6, E6b, E6c: netcode-architect's read (`netcode-architect-read.md`, blocks 1 to 6, DONE)
The read holds every symbol with today's file:line and each test with its planted defect; what the sheet takes:
- **Step 2, the command path** (block 1; 315 / 630 against the plan's 385 / 530): the aim's maths in `sim::aim` (so
  the simulation's float scan covers it); the old game's rule corrected: the aim falls back to the ground plane
  beyond a 400 m reach (not the design's 500 m and fire-height plane); the map's bounds added to `StaticWorld` in
  whole centimetres; the host zeroes the move when the tab hides; the first aim clamped into the bounds; the replay
  readers' refusals of a move and an aim go (replay.rs:81-91, recording.ts:141-142); `clamp_move` steps both parts on
  a tie. **The post: 13 numbers with the move packed in one word** (the plan's 6.2 (2) and the recording's format;
  engine-architect's design 3.6 had 14, and its read now follows the 13).
- **Step 3's wire and timing** (block 2; about the plan's 320 / 340 with engine-architect's layout): the block
  message carries `dueAt` (a pacer signature change); the page keeps three blocks; the pool goes to 16; the blend
  picks its pair by the stamped due times and measures the fraction back from the newer block's due (worked by hand
  for the first frame after a pause: no jump, no step back); the allowance rises at once and falls gradually, its
  estimator pinned by its test B1.
- **Step 6, the recordings** (block 3; 245 / 170): `first-loop` moves in step 1's commit (as rust-reviewer found) and
  is frozen anew whenever the movement kind gains a field; new `walk` (2,400 ticks) and `lane-walk` (1,200), with
  `walk_reaches.rs` proving every query kind is hit; his FEATURE-COMPARE condition needs no new browser spec (Firefox
  already runs `tests/replay/**`); 4B-RAY-WASM as three lines in `static-world.txt` (rust-reviewer picks the three
  rays and owns 4B-RAY-COUNTS).
- **Steps 6b and 6c** (block 4; 6b 130 / 115, 6c 45 / 220): the paced spec needs a folder of its own,
  `tests/paced/`, because the Firefox project runs everything under `replay/` (**the plan's 6b row is corrected**);
  PM-0004-39 can only use a map built in code, since the loader refuses bounds beyond 327.67 m.
- **RESUME-FLASH by option** (block 5): (a) about 5 / 40 beyond steps 1 and 3; (b) 10 / 20 more; (c) the host's two
  hide lines removed and two tests reversed; (d1) a reserved button bit, about 25 / 40; (d2) a new call, so a new
  recording format and every recording made anew.
- **The order if his movement answers come late** (block 6, facts; the advisor agent rules): of steps 2 and 3 all
  but three pieces need neither step 1 nor any of his answers (the jump rule J1 to J5, the room's snapshot fill with
  S4, and a soldier to fill the block's row); built before step 1, nothing is built twice in substance, and three
  things are touched twice (the constructors, the snapshot's soldier field list, and first-loop's hashes, the last
  only if the world keeps the command's move and aim). His FEATURE-COMPARE condition binds the pull request, not the
  order: `walk` lands in the pull request that brings step 1 and the cast.
- **Its calls for the sheet** (its closing list): the record's width (it recommends that the world does not keep the
  command's move and aim, so step 2 moves no hash; rust-reviewer's read has the shared state write them: **a clash
  for the advisor agent**); one home for the bounds (the aim's centimetres and the movement's metres, built once);
  the room's and endpoint's tests get a world through `content` as a dev-dependency (engine-architect's block 4); the
  jump's rule J1 to J5 counted once (it sits in both reads); the soft push is in no recording (one slot), so its
  sameness across targets needs a WebAssembly run of its own; 6b's spec waits for the CI Builder's answer to
  4B-CHROMIUM-TIME; **three readings not in DEPART-4's fourteen, for his word**: PM-0004-60 at 2,000,000 commands in
  CI's release line (not 10 million nightly through a fuzzer); PM-0002-14 met in its ray half only until change-set 8
  brings the view time; PM-0004-20's hash half on `walk` (one soldier), its flag-off timing not runnable.

## F. Sizes, the ladder and the stop lines
- By the designs' count: steps 1 to 7, 7,315; step S about 740 (engine-architect's recount; 630 before); together
  about 8,055. At the measured rates (2.17 to 2.7): about 17,500 to 21,750 written lines. The plan's stop line, 16,450,
  was set before step S moved in: the line with it inside is his (SIZE-4C). His options and engine-architect's seam
  for (c) (block 2 of its read: 4c-2, the soldier's simulation with the command path and the recordings, nothing new on
  screen, about 3,960 to 4,395 by the designs' count; 4c-3, he plays, about 2,920 to 3,355) are in the question file;
  the main session now recommends (b), since (c) delays the first build he can play by 4c-2's reviews and his go.
- **What leaves first if the count runs high** (his SIZE-4B point 2): the plan's rung, the far-corner map (PM-0004-39)
  to 4d at 13,150 written lines, told to him before it moves. Nothing else moves; at a stop line the chat stops and he
  decides. Under SIZE-4C (b) or (c), each pull request gets its own rungs, written here when he has answered.
- Until he answers SIZE-4C, step S is held to about 1,420 written lines (the number told to him: its planned size
  times 1.25 on the first sheet's 630), and its reviews wait, since under (a) they run on 4c's final diff. On the
  recount (740) the plan's rule gives about 1,665, and the step is expected at about 1,600 to 2,000: section H asks
  the advisor agent whether the hold, which is this chat's own limit and not his line, stays at 1,420 or follows the
  plan's rule on the recount.
- The count is taken at each step's end by 4b-2's method (`count-4c.mjs` in this folder, from `count-4b2.mjs` at
  59f6cc7): `git diff --numstat dae6d107 -- crates packages`, added lines; the vectors' data and what `make_recordings`
  writes left out; everything else in, the generated TypeScript included. The advisor agent's count ruling of
  2026-11-27 held the generated TypeScript counted "to the end of 4b-2": whether it stays counted in 4c is its to rule
  (section H).

## G. Evidence, reviewers, his checks, risks
- **Step S:** a visual change. Before and after pictures of the real page, made by `shot-4c.mjs` (this folder) on
  the built development page in Chromium at 1280 x 720 and a pixel ratio of 2. **The before is taken** (2026-11-28,
  00:36 Amsterdam): main's page, built from this branch at 8df0a4e (main's code), WebGL 2 fallback on the
  container's software renderer, 3 draw calls, 3 triangles, 62 lines, 3 geometries, 3 textures, 6 programs (the
  committed baseline exactly), errors 0; the picture is `.gate/scene-before-4c.png` (not committed: a binary).
  Then the render counts after, with the same method; the browser tests; the gate. Reviewers on its diff, as 4b's
  plan named them for this step: graphics-engineer and perf-auditor; with engine-architect (the interface file),
  security-auditor (content text shown on the page) and test-engineer. art-director's lens is the old game's locked
  look and the war room's UI: whether it reads a greybox is the advisor agent's call. His Brave check: the street
  corner and the training yard in greys.
- **Steps 1 to 7:** pending the reads.
- **Risks:** CI's Chromium run uses 3.6 to 3.9 of its 5 minutes (4B-CHROMIUM-TIME): 4c's browser tests may pass it,
  and the fix is a CI edit, the CI Builder's, with his yes first. Others pending the reads.

## H. What I ask of the advisor agent
1. **Step 0 and step S: APPROVE or REVISE now.** Neither waits for a movement answer. Step 0 is made after your verdict
   and waits for PASTE-BREAKS, BOARD-WORDS and DESIGN-CLASH; step S is built after your verdict, and its reviews wait for
   his SIZE-4C answer (under (a) they run on 4c's final diff).
2. **Step S's hold:** this chat told him it holds step S to about 1,420 written lines until he answers SIZE-4C (the
   plan's rule on the first sheet's 630). engine-architect's recount (740) gives about 1,665 by the same rule, and the
   step is expected at about 1,600 to 2,000. The hold is this chat's own limit, not his line: does it stay at 1,420,
   or follow the plan's rule on the recount? Either way the count is said to him at each step's end.
3. **The count rule in 4c:** your count ruling of 2026-11-27 held the generated TypeScript counted "to the end of
   4b-2". Counted in 4c too, or not?
4. **Step S's reviewers:** graphics-engineer, perf-auditor, engine-architect, security-auditor, test-engineer, as 4b's
   plan named them for this step; whether art-director reads a greybox.
5. **SWEEP-ROUTE** (E1): accept our own sweeps as a departure recorded for his acceptance (DEPART list), or keep the
   library's queries as the Walk card under his go words it ("our own movement on parry3d's queries")? Or is it his
   to answer now?
6. **The record's width** (E2's calls): the world keeps the command's move and aim in the shared state
   (rust-reviewer's read) or not (netcode-architect's, so step 2 moves no hash)? And one home for the bounds
   (centimetres for the aim, metres for the movement).
7. **The order while his movement answers are out** (netcode-architect's block 6; rust-reviewer's block 3's list):
   may the pieces that need no answer be built before step 1's movement (steps 2 and 3 but for J1 to J5, the room's
   snapshot fill and a soldier for the row; step 1's table, rules, jump, stamina, easing, facing and the sweeps' common
   part), or does the plan's order stand?
8. **The dependency holds 4c reopens:** libm as a direct dependency of `sim` (E1) and `content` in the room's
   dev-dependencies (E3 to E5): one batch for security-auditor before either is committed, pushed or built?
9. **Readings and departures for his word or for dated pointers** (E1 to E6): our own sweeps (item 5); the three
   readings netcode-architect lists (PM-0004-60 at 2,000,000 in CI's release line; PM-0002-14 in its ray half;
   PM-0004-20's hash half on `walk`); the camera kind at step 4, not step 5; the stamina and the sprint lock in the
   owner's section; the plan's step 6 corrections (first-loop re-made in step 1; the paced spec in `tests/paced/`).
   Which go to him now, and which wait for the drafts' next revision as dated pointers?
10. **BODY-SHAPE** (E1; game-designer's read is running and its result comes as an addendum): the movement step is
    held until it is settled with him. Anything else you need before you would approve steps 1 to 7 once his answers
    are in?
11. **Under SIZE-4C (c) only:** may PM-0004-28 ("before 4c merges") be met in 4c-3 (engine-architect's seam)?
