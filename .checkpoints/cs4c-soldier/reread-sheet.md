# 4c, "The first soldier": the sheet for the advisor agent's re-read, revision 1 (2026-11-28, main dae6d107)

_Written by the main session (Builder V6, lane 1) from the approved walk plan (`.checkpoints/cs4-walk/plan.md`,
revision 4: section 6 and the sections it names), three reads against today's code (`engine-architect-read.md`,
`netcode-architect-read.md`, `rust-reviewer-read.md` in this folder) and his words. The walk plan's section 3, item 1:
"Each later part starts only after the advisor agent has read this plan again against that day's main ... That
re-read carries the part's exact symbols to add and to remove, taken from the designs' tables as they stand that
day." **No file outside `.checkpoints/` is edited before the advisor agent's verdict.**_

**Revision 2: the advisor agent's verdict applied** (`advisor-reread-4c.md`, on revision 1 at 78fd901e): **step 0
and step S APPROVED, with conditions 1 to 9; steps 1 to 7 not yet** (its item 10, thirteen points, listed in section
H). Each condition is applied where it lands, marked "(condition n)". Revision 1 was written from four reads
(engine-architect, netcode-architect and rust-reviewer, then game-designer's on BODY-SHAPE and the push, since DONE).

**Revision 3 (2026-11-28, from 02:14 Amsterdam by the clock): the advisor agent's item 10, as far as it goes without
his answers.**
Step S is built (bc7720f3, c1de41e5; 1,288 written lines by the count; CI green on 8bee8365). The dependency batch is
answered (security-auditor APPROVE, `dependency-check.md`). New here: the record rule applied with its confirmations
outstanding (end of section E), the recount of steps 1 to 7 with each option's stop line (section F), each step's
evidence, reviewers and his checklist (section G), and where each of item 10's thirteen points stands (section H).
Running: rust-reviewer's block 6 (the bodies on the cases block 5 left out; the 2.5D question), engine-architect's
block 5 (GROUND, FACING-SPAWN), netcode-architect's block 7 (the record rule, the bounds' home, the zero ray).
**Revision 3.1 (by 02:37 Amsterdam, from the clock):** those three DONE and folded in (the record rule confirmed by both, section E;
the recount with their numbers, F2; the table of item 10, H). Running: rust-reviewer's block 7 (engine-architect's
RR1 to RR5, FACING-SPAWN from the maths side, a correction on the head bump), netcode-architect's block 8 (NA1 to NA3,
G-2), game-designer's new block (an unjumped fall onto the 0.56 m step; the head bump from its notes).

**The branch rule (condition 2; F-ORD-1):** pull request #79's head was this same branch, so until he answers SIZE-4C
only step 0 and step S go on it: anything of steps 1 to 7 committed before his answer would make (b) and (c)
impossible to cut without rewriting history. Meanwhile notes and prototypes only. Told to him as a consequence of his
choice: "(a) releases the rest at once". Under (b) or (c), once 4c-1's head is under review even a notes push moves it,
so notes would then stay local, which his 25-minute save rule does not allow for long: his to weigh in SIZE-4C.

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
- **The advisor agent's conditions on step 0 (condition 7):**
  - **A byte check against main (F-S0-5):** a script shows that the head's LOOP_V2, with section 14, the pointers and
    the header's sentence taken out, equals main's byte for byte; that each pointer is exactly his string "(2026-11-27:
    resource points replace open fields, see §14)" at exactly the listed lines; and EDITOR_V2 likewise under
    DESIGN-CLASH. (`git diff --stat` shows files, not that nothing else moved: his item 3, "Keep the old text for the
    history ... Change nothing else in the design docs".)
  - **Each pointer at the end of the sentence or bullet that names the field (F-S0-2),** so no sentence is cut: section
    1's sentence ends on line 16, section 3's bullets on 33 and 35, section 9's first bullet on 151.
  - **The as-pasted file tied to the saved message's digest (F-S0-3):** `section14-4c.mjs` checks
    `section14-as-pasted.md` against sha256 2bd82216... (3,063 bytes) before it mends or checks.
  - **The method for BOARD-WORDS (b), written before it runs (F-S0-1):** `decisions-4c.mjs` quotes whole messages only; under
    (b) the saved file is the message less its item 3, with a header line saying item 3 is left out and why; the header
    gives the shown text's digest, which the check verifies; and the whole message's digest is recorded beside it, so
    that it can be checked against the transcript. Written into the script before it is run. **Written and tested
    (revision 3, 2026-11-28):** `chat-words.mjs` gained the mode `less` (the message as `words` finds it, the one line
    that starts with a given start left out, here "3. "; the header gives the left-out line's number and why, and the
    shown text's bytes and sha256); `decisions-4c.mjs`'s `saved` reads that header, checks the shown text against its
    own digest and the line count, and the entry's bracket says "without its line 5 of 8, left out because it names
    his board (the whole message is in the chat's transcript, by this id)". **Changed after the commit security review
    of 2026-11-28 (Medium):** the whole message's bytes and sha256 are written into no file of the repository, since
    a digest and a length of the whole beside the published shown text would let a reader confirm a guess of the
    left-out line; `less` prints them for the chat only, and the chat gives them to him (the check against the
    transcript is `less` run again into a scratch file). This departs from F-S0-1's "the whole message's digest
    recorded" only in where it is recorded: in the chat, not the repository (for the advisor agent at its next
    review). The notes commit bfe447a3 before this change named the whole message's length and the first eight
    characters of its digest in this paragraph; nothing of the message's text is in the repository. Tested in the
    scratchpad only: `words` still makes the saved design notes byte for byte (7,797 bytes); `less` shows his message
    of 23:59 at 1,284 bytes with items 1, 2, 4, 5 and 6, no "board" in the shown text, and neither the whole message's
    digest nor its byte count in the saved file; a scratch copy of the register took all four entries newest first,
    `check` passed on each, and a planted one-byte change in the quoted text failed it.
- **Its landing (condition 8):** step 0 waits for PASTE-BREAKS, BOARD-WORDS and DESIGN-CLASH, lands before any other docs
  commit of 4c, and the pull request that carries it is not marked ready before it (its evidence: F-S0-5's check and
  `decisions-4c.mjs check`). **Risk tier 1, needs-mustafa (condition 9):** LOOP_V2 and the register are decision docs,
  and the register holds his words.

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
  development block; two new `DEV_MARKERS`: `"__shambleScene"` and the fixed sentence `readInventory` throws when the
  floor holds an object it does not know (`INVENTORY_UNKNOWN`, "the scene inventory met an object it does not know": a
  live string of the module's own code, so the development build keeps it, where an unreferenced tag constant would be
  dropped there too and the both-ways marker check would fail) (condition 1); so the production scan refuses a
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
| Add | picture.ts: `PictureContent`, `Picture.setContent`, `SceneObject`, `Picture.inventory`. render/scene.ts (new): `buildEmptyScene` (moved; returns `{ scene, camera, floor }`), `buildMapFloor(map)`, `solidPoints(shape)`, `greyOf(top, lowest, tallest)`, `groundGrey(kind)`, `disposeFloor(floor)`, `overview(bounds, aspect)`. render/inventory.ts (new): `readInventory(floor)`, `INVENTORY_UNKNOWN` (its thrown sentence, the marker: condition 1). app/content.ts: `MAP_FORMAT`, `MAP_VERSION`, `MOVEMENT_FORMAT`, `MOVEMENT_VERSION`, `CONTENT_KINDS`, `NOT_THE_MAP`, `ContentFile`, `ReadContent`, `readMap`, `readContent`, private `closed`, `corner`, `word`, `text`. sim-worker/src/index.ts: `LOCAL_MAP` re-exported. overlay/report.ts: `ContentRows`, `ReportData.content`, `ReportData.scene`. boot.ts: `window.__shambleScene`. build-checks.mjs: two `DEV_MARKERS`. render/scene.test.ts (new). |
| Change | three-picture.ts: `createThreePicture` (the floor, `setContent`, `inventory`, the overview in `setContent` and `resize`; `declare const __SHAMBLE_DEVTOOLS__`). docs/REPO_MAP.md (docs-keeper, at the docs pass). content.ts: `PageContent`'s constructor and `afterMessage`, its comment. boot.ts: the hand-off, `data()`, the second hook. report.ts: `readoutLines`, `formatReport`. e2e: skeleton.spec.ts (the counts' read, the broken-Worker tests), content.spec.ts (the refused bundles' counts), report.ts's `Report` type, render-counts.json's shape. content.test.ts, content-ties.test.mjs, content-schema.test.mjs, report.test.ts, build-checks.test.mjs. |
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
so about 1,600 to 2,000 written lines at the measured rates. **The hold follows the recount (condition 4):** about
1,665 written lines (740 x 1.8 x 1.25), said to him in one sentence before the count passes 1,420; the two-thirds
sentence at about 1,110; at 1,665 the step stops; his SIZE-4C line wins when he gives one. What could leave first,
said to him before it moves: the overview camera (only under SIZE-4C (a), where the follow camera frames the scene
before 4c merges), then the per-file headers of the readout (the rows would name paths alone).

**Build by symbol (F-S-3, a Low):** the read's line numbers for boot.ts, three-picture.ts and content.spec.ts do not
exist at dae6d107 (those files have 178, 80 and 101 lines); every claim holds by symbol (`PageContent` at boot.ts:61,
the development block :114-159, `__shambleReport` :159, `visibilitychange` :106-107; `buildEmptyScene` at
three-picture.ts:37, `draw` :53, `counters` :57, `resize` :68; content.spec.ts's baseline :10, its poll :87-88). The
report cites the right lines.

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
  whole centimetres; the host zeroes the move when the tab hides; the first aim clamped into the bounds (withdrawn by
  netcode-architect's block 8: there is no first aim, the soldier holds "no aim yet" from spawn, section E); the replay
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

### The record rule: the advisor agent's ruling, applied (revision 3); confirmations outstanding
The clash above (whether the world keeps the command's move and aim) is ruled by the advisor agent's item 6
(`advisor-reread-4c.md` line 31), and the sheet takes it as the design for steps 1 and 2:
- **The invariant:** every value a later tick reads is in the hashed shared state (ADR 0004 part 9: "a canonical
  serialization of its whole state"), and a tick with no command keeps the held move and aim as it keeps held buttons
  today (`crates/sim/src/world.rs:143-145`: "With none the record holds, so a held button stays held").
- **The design:** netcode-architect's record (i), the record kept as buttons and presses, with the soldier's hashed row
  holding the held move as well as the aim point (rust-reviewer's Soldier already has the aim point, its read 1.1). A
  command-less tick moves the soldier by the held move toward the held aim. Not the narrow record with nothing kept: a
  command-less tick would then stop the soldier, a rule change for change-set 7 that is not made silently.
- **Its test,** in step 2: a planted command-less tick (the soldier keeps moving and turning as with the held command),
  and a restore from the hashed bytes alone that must carry on identically; each with its planted defect (the held move
  dropped from the row; the row restored without it).
- **One home for the bounds:** `StaticWorld::build` derives both from the map's bounds once (metres for the sweeps,
  whole centimetres rounded inward for the aim's clamp), and a test ties the two.
- **Confirmed by both (2026-11-28):** netcode-architect YES (its block 7): a command-less tick keeps the record's
  buttons and presses (no new press, no jump) and the soldier's `held_move: [i8; 2]`, and he walks on toward the held
  aim; the aim held as the command's integers, `aim_cm: AimCm` (6 bytes), so 8 bytes of each row hold move and aim; the
  restore test plants 30 command-less ticks and rebuilds a world from the shared bytes, and must catch the held move or
  aim left out of the hashed bytes and a command-less tick that zeroes the move; a room test pins that a tick with no
  command due hands the world none; step 2 moves no first-loop hash on four named conditions (its block 7; block 8
  replaces them: step 2 moves none only if "no aim yet" lands whole in step 1, and the condition that the bounds
  contain the map's origin is gone, since there is no (0, 0, 0) first aim). rust-
  reviewer YES (its block 6, section 3): F17 stands and 1.6's "move and aim of the record" is withdrawn; the row also
  holds the body's own carry-over that the next tick reads (`pull: u16` under BODY-SHAPE (3a), `stepped: u16` under
  (3b), none under (2)); `visit_shared` writes the tick, the record's buttons and presses, then the 48 rows in slot
  order. The bounds' one home: netcode-architect YES, with the loader's own grid rule (content's `on_grid`, made
  public) and its earlier "off the grid is refused" withdrawn; a tie test with four planted defects. The zero ray
  (under SIZE-4C (c)): netcode-architect YES (all-zero judged by value; a non-finite number refused and counted in
  `rays_refused`), tests Z1 and Z2. Their counts: step 2 to about 340 / 675; step 1 about +10 / +100 (block 7's;
  block 8 makes them step 2 about 335 / 690 and step 1 about +25 / +145, as F2 counts them).

### The held aim at spawn, "no aim yet" (revision 3.2; netcode-architect's block 8, with rust-reviewer's yes in its block 8 and engine-architect's in its block 6)
- **The rule:** the one aim value no map can hold, (-32,768, 0, 0) in whole centimetres, means "no aim yet". The
  soldier's hashed row holds it from spawn (`aim_cm: AimCm`, now required); his facing stays the spawn's direction
  (FACING-SPAWN (a)) until a real aim arrives; the room applies a tick's commands oldest first, and the value never
  goes through the bounds' clamp. `Endpoint::new` has no first aim and the endpoint holds no aim (block 7's (0, 0, 0)
  first aim is withdrawn, and with it the condition that the bounds contain the map's origin).
- **The facing rule:** one integer compare first; on NONE the facing is left bit for bit; otherwise the fields are
  turned into metres after the compare, and the 2 cm rule applies. Both targets agree, since the compare is integers.
- **On the page:** `own_flags` bit 1 "aim held", clear while NONE and before the first snapshot that carries the
  soldier, the aim then written as 0, so NONE never crosses to the page as a number; the owner's section shows the
  held aim from the newest snapshot; the row's pitch is 0 and the yaw the spawn facing while NONE; no aim line; the
  readout says "none yet". The section stays 20 bytes, the block 1,988. It changes the meaning netcode-architect's
  design gives the owner's aim (its 2.1), with no wire change: noted for step 2's review.
- **Tests:** netcode-architect's N-1 to N-3 and rust-reviewer's S-1 to S-3, each with its planted defect.
- **Where it lands:** whole in step 1 (the row's NONE from spawn, the room's fold, the facing's keep, endpoint.rs:258
  writing NONE in place of (0, 0, 0)): then step 2 moves no first-loop hash; otherwise every hash moves again in step 2.
- **A world with no aim area** (engine-architect, block 6): no constructor change; one room test with a planted panic.

### Step 1's commit order for the dependency batch (revision 3.2; security-auditor's second answer, the advisor tool's point)
The batch's commit must also hold the scan's stricter libm rule, the maths list's entries for the libm paths the
players' real build prints, the exp and sincos vectors, F1's tightened test and `layering.rs`
(`dependency-check.md`). The list's entries and the vectors exist only once `sim` calls both routines, so the batch
cannot land ahead of that code: it lands in the commit that first calls exp (the easing shares at match setup) and
sincos (the spawns' facings), with their vectors and the scan rule, and that same commit re-makes world.rs's known
answers and first-loop if it changes the hashed state (the advisor agent's item 7, condition 4: CI green at each
commit). Nothing that holds the batch is built before it (DEP-HOLD).

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
  writes left out; everything else in, the generated TypeScript included. **The advisor agent's ruling for 4c (its
  item 3):** the generated TypeScript stays counted in all of 4c (the measured rates the stop lines use were taken with
  it in); each part is counted from the main commit it starts on (4c-1 from dae6d107; under (b) or (c) the next part
  from the main that holds 4c-1's merge); the about 35 lines moved from empty-scene.ts count as added, and the report
  says so.

### F2. The recount of steps 1 to 7, and each option's stop line (revision 3; the advisor agent's item 10, point 12)
**Step S as built:** 1,288 written lines (`count-4c.mjs` on the branch at 3ab59bd7 against dae6d107: bc7720f3's 1,277
and c1de41e5's 11), about 1.74 times its recount of 740: under the 1,420 he hears about first and the 1,665 hold.

**Steps 1 to 7 by the designs' count (code / tests), as the reads recount them against today's code:**
| Step | The plan | The reads | Where |
|---|---|---|---|
| 1 | 1,240 / 1,090 = 2,330 | body (3a) on our own sweeps: about **3,100** in the 2.5D form (1,445 / 1,655), about **3,200** with the 3D sweeps (1,515 / 1,685) (revision 3.2: netcode-architect's block 8 raised its step 1 part from +10 / +100 to +25 / +145) | rust-reviewer-read 1.10 (1,300 / 1,260, body (2) in 3D); E1 above (+130 / +110: the ray counts, the sweep proof, NO-SOURCE's fields); block 5 item 3 ((3a): +30 / +50 and -20); **block 6 section 2: the honest 2.5D sweeps 205 / 265 (shapes may have space under them; with the two model fixes, the head test and the stood-on slope), not block 5's 170 / 200; the 3D sweeps 260 / 260 plus the same fixes and the head's stop, about 275 / 295 (my estimate from its words)**; netcode-architect's block 7 (+10 / +100: the record rule's row, the bounds' home); engine-architect's block 5 (GROUND about 9 / 42; FACING-SPAWN (a) about 25 / 45) |
| 2 | 385 / 530 = 915 | 335 / 700 = **1,035** | netcode-architect-read, its counts, block 1, raised by its blocks 7 and 8 (the record rule's tests, the bounds' tie test, Z1 and Z2, "no aim yet" N-1 to N-3: 335 / 690) and engine-architect's block 6 (the room test for a world with no aim area, 0 / 10) |
| 3 | 320 / 340 = 660 | **about 675** (with engine-architect's layout, and half of its block 6's 8 / 20 for "aim held") | netcode-architect-read block 2; engine-architect-read block 6 |
| 4 | 800 / 540 = 1,340 | **about 1,355** (split by file, not counted again; the camera kind moves in from step 5 with its lines; the other half of "aim held") | engine-architect-read blocks 3 and 6 |
| 5 | 360 / 300 = 660 | **660** less the camera kind's lines (the sum of 4 and 5 unchanged) | engine-architect-read block 3 |
| 6 | 320 / 230 = 550 | **550** (netcode-architect's own part 245 / 170; the rest the other designs' marks) | netcode-architect-read block 3 |
| 6b | 130 / 110 = 240 | 130 / 115 = **245** | netcode-architect-read block 4 |
| 6c | 30 / 180 = 210 | 45 / 220 = **265** | netcode-architect-read block 4 |
| 7 | 0 / 410 = 410 | **410** (no read counted it again) | the plan |
| **All** | **7,315** | **about 8,295 to 8,395** (13 to 15 % above the plan; revision 3.2) | |
Not in these numbers: (3a)'s five more NO-SOURCE fields than body (2) (about 50 to 75 lines); RESUME-FLASH beyond (a)
((b) about +30; (c) about 5 changed and two tests reversed; (d1) about +65; (d2) a new recording format and every
recording made anew); whatever game-designer's answer on an unjumped fall adds. The two sweep forms are both carried
(revision 3.1: the 2.5D form's honest count, rust-reviewer's block 6, narrows the gap to about 100 lines).

**The plan's rule** (planned size times 1.8, then the stop line at 1.25 times that, so 2.25 times the designs'
count: the plan's 16,450 is 7,315 x 2.25) **on the recount:** steps 1 to 7 alone, a stop line of about 18,660 to
18,890, two thirds about 12,440 to 12,590. At the measured rates (2.17 to 2.7 written lines a counted line, 4a to
4b-2) steps 1 to 7 come to about 18,000 to 22,670 written lines: likely to reach that line near the end.

**By his SIZE-4C option** (the line is his in each; these are the plan's rule's numbers for him to take or change):
| Option | Pull requests and their designs' count | Stop line by the rule | Two thirds | The rung (what leaves first) |
|---|---|---|---|---|
| (a) one | all of 4c: the scene (740) and steps 1 to 7 (8,295 to 8,395) | about 20,330 to 20,550 (the question said about 17,870 on the old counts) | about 13,550 to 13,700 | the plan's: the far-corner map (PM-0004-39) to 4d at 13,150 written lines, told to him before it moves |
| (b) two | 4c-1: step 0 and the scene (1,288 written now); 4c-2: steps 1 to 7 (8,295 to 8,395) | 4c-1: the hold, 1,665; 4c-2: the plan's 16,450 (on the old count) or about 18,660 to 18,890 (on the recount), his to say which | 4c-2: about 10,970 on 16,450, or 12,440 to 12,590 | 4c-2: the plan's 13,150, the far-corner map (the plan's planned size, 1.8 times 7,315); the planned size on the recount would be about 14,930 to 15,110, his to move to or keep |
| (c) three | 4c-1 as (b); 4c-2: steps 1, 2 (less RESUME-FLASH's part), 6, 6c and step 3's snapshot share, 4,905 to 5,440; 4c-3: step 3's rest, RESUME-FLASH, steps 4, 5, 6b, 7, 2,955 to 3,390 | 4c-2 about 11,040 to 12,240; 4c-3 about 6,650 to 7,630 | 4c-2 about 7,360 to 8,160; 4c-3 about 4,430 to 5,090 | 4c-2 holds the far-corner map (step 6c): its rung inside 4c-2 is his to set, since the plan's 13,150 lies above 4c-2's whole line |
Under every option nothing else moves: at a stop line the chat stops and he decides; the plan's never-moved list stands
(the movement recording on its four runs with PM-0004-17, -19, -21 and -24, the paced replay PM-0004-28, the stale-id
run PM-0004-15, the hostile commands PM-0004-60, the parity marks PM-0004-30 and -59, the camera panel). The card
takes the chosen option's line, its two-thirds sentence and its rung before step 1 (SIZE-4B point 2).

## G. Evidence, reviewers, his checks, risks
- **Step S:** a visual change. Before and after pictures of the real page, made by `shot-4c.mjs` (this folder) on
  the built development page in Chromium at 1280 x 720 and a pixel ratio of 2. **The before is taken** (2026-11-28,
  00:36 Amsterdam): main's page, built from this branch at 8df0a4e (main's code), WebGL 2 fallback on the
  container's software renderer, 3 draw calls, 3 triangles, 62 lines, 3 geometries, 3 textures, 6 programs (the
  committed baseline exactly), errors 0; the picture is `.gate/scene-before-4c.png` (not committed: a binary).
  Then the render counts after, with the same method; the browser tests; the gate. **Reviewers on its diff (condition
  5): six**, three at a time, one round: graphics-engineer and perf-auditor (as 4b's plan named them for this step),
  engine-architect (the interface file), security-auditor (content text shown on the page), test-engineer, and
  art-director on a narrow brief: whether the greybox reads (heights told apart from the overview, sides against tops,
  the street shown or not by STREET-GREY, the readout's rows readable) and its Brave checklist; no judgement against
  ART_DIRECTION (the old game's) or LOOK_V2 (the greybox decides nothing).
- **His check of step S (condition 6), in Brave and once in Firefox, about 5 minutes, on the zip's development build,
  each step with its expected result:**
  1. Open the page as the zip's README says (`node serve.mjs` in its folder, then the address it prints). **Expected:**
     the grey street corner and training yard in one overview from above at an angle; the readout's first line names
     the backend; "errors: 0".
  2. Look at the blocks. **Expected:** the two houses with their gable roofs, the alley, the cover wall; the yard's two
     low steps, its ledges, the platforms with their ramps, the thin wall and the 50 m lane; each block's grey lighter
     the higher its top (the roofs' ridges lightest, the 0.55 m step the darkest block), sides darker than tops.
  3. Look at the ground. **Expected, by his STREET-GREY letter:** (b) the street in a grey of its own, darker than every
     block, beside the grass; (a) one grey for street and grass; (c) the patches as outlines.
  4. Read the readout's two new lines. **Expected:** "content: maps/test-scene.json shamble-map 1 ·
     movement/soldier.json shamble-movement 1" and "scene: test-scene · 17 boxes · 6 ramps · 6 regions".
  5. Press "Copy report" and paste it into the chat. **Expected:** the same rows (`content.*`, `scene.*`), "errors: 0",
     the render counts.
  6. Open the same page once in Firefox. **Expected:** the same scene and rows, "errors: 0".
- **Steps 1 to 7 (revision 3; the advisor agent's item 10, point 13).** Each step's evidence, by the plan's section 17
  (logic by tests on the real code, each with its planted defect, run by a plants script; netcode by deterministic
  tests; performance by numbers with their method; what a player sees by screenshots that name their backend, then his
  eyes). Reviews run on the part's final diff, not per step (the review budget); "read by" names who covers that
  step's part of the diff.
  - **Step 1, the movement:** the body's cases as tests (blocks 5 and 6 of rust-reviewer's read become the table);
    PM-0004-30 (the 0.55 m step gained, the 0.56 m step stopped); PM-0004-59 and every jump test against
    `jump_reach_m` as read, never a literal; PM-0004-15 (stale ids, a 64-slot table through the capacity parameter);
    PM-0004-6 (nothing allocates in the step); `Rules::take`'s refusals (a jump reach below the step height; a sprint
    or refill time that rounds to 0 units), each tested; PM-0004-22 for exp (at least 10,000 inputs, bit-identical
    natively and in WebAssembly in Node and each browser engine, the largest error in ulp, libm's cost; one native run
    over all 2^32 inputs); the source scan's one named allowance with its control and bites, and the maths list's
    exact paths (security-auditor's item 3); the dependency batch in one commit with F1's test and F2's layering test;
    world.rs's known answers and first-loop re-made in the commit that changes the state (CI green at each commit).
    Read by rust-reviewer, netcode-architect (the hashed state), game-designer (the rules against his words),
    test-engineer.
  - **Step 2, the command path:** deterministic tests: the post's 13 numbers with the move in one word; the endpoint's
    cast on its own copy of the world; the room's refusals counted (a move longer than 1, an aim outside the map); one
    jump per command (3B-ONE-PER-TICK; J1 to J5 counted once); the record rule's command-less tick and restore; the
    bounds' one home with its tie test; the host zeroing the move when the tab hides; "no aim yet" (N-1 to N-3 and
    S-1 to S-3; there is no first aim to clamp); under (c) the zero ray, both ways. Read by netcode-architect,
    security-auditor (message handling, and a client that sends the "no aim yet" value), rust-reviewer.
  - **Step 3, the snapshot's soldier and the block:** the layout's sizes pinned (header 12 bytes, the owner's section
    20, 48 rows of 40: 1,988 bytes) and `layout_hash` re-pinned; the blend's tests (B1, the allowance's estimator; the
    first frame after a pause: no jump, no step back); the ring of three blocks with `dueAt`; the pool of 16
    (3B-POOL-16); allocation.spec's limit holding. Read by netcode-architect (the wire, the timing), engine-architect
    (the layout across the boundary), perf-auditor.
  - **Step 4, the page:** before and after pictures of the real page naming their backend; the render counts; the
    allocation measured again; the aim line in the development build only (`DEV_MARKERS` gains `"aim-line"`; the
    production scan reads `.css`: 3B-CSS-PROD); 3B-BLUR-TEST; the stamina bar (hidden when full and not locked); the
    camera's content kind (`shamble-camera` 1, his Export's 21 values) with its schema, generated type and
    `readCamera`. Read by engine-architect, graphics-engineer (the drawing code), art-director on a narrow brief (the
    stand-in, the stamina bar and the aim line readable; no judgement of the look: for the advisor agent to confirm,
    as for step S), test-engineer, perf-auditor.
  - **Step 5, the camera panel:** its limits from the generated schema through `numberLimits` in `@shamble/types`
    (4B-LIMIT-IMPORT); the five old presets written by script from `docs/legacy-tuning.json` (each sets the six values
    the old preset set); "copy values" through one browser-safe copy of the content's canonical rule; the development
    code out of the production build (the scan). The early-look zip after this step. Read by engine-architect,
    security-auditor (the panel's text, the clipboard), art-director (narrow), test-engineer.
  - **Step 6, the recordings:** `walk` (2,400 ticks) and `lane-walk` (1,200) on four runs (native, Node's WebAssembly,
    Chromium, Firefox) with PM-0004-17, -19, -21 and -24; the three planted divergences run again on movement;
    `walk_reaches.rs` proving every query kind is hit (4B-QUERY-RECORDING; FEATURE-COMPARE condition 4: `walk` lands
    in the pull request that brings step 1 and the cast); 4B-RAY-WASM's three lines in `static-world.txt`
    (4B-RAY-COUNTS). Read by netcode-architect, test-engineer, rust-reviewer.
  - **Step 6b, the paced replay:** in development builds, a recording at live pace in the real Worker, the camera still
    and moved, its hashes the recording's own (PM-0004-28), in `tests/paced/`; it waits for the CI Builder's answer on
    4B-CHROMIUM-TIME. Read by netcode-architect, test-engineer, perf-auditor.
  - **Step 6c:** 2,000,000 hostile movement commands with no panic (PM-0004-60, by his DEPART-4C reading); the yard
    at the far corner of a 2 km map built in code (PM-0004-39, the rung). Read by security-auditor, rust-reviewer,
    test-engineer.
  - **Step 7, the page's browser tests:** walk, jump, the two steps, aim on a ramp, the camera follows, keys let go on
    blur (3B-BLUR-TEST), the hidden tab with a soldier, allocation measured again. Read by test-engineer,
    perf-auditor.
  - **The part's reviewers** (the plan's section 17; at most three at a time, two rounds at most, the second on what
    changed): rust-reviewer, netcode-architect, engine-architect; then graphics-engineer, test-engineer,
    security-auditor; then game-designer, perf-auditor; art-director on the narrow brief if the advisor agent confirms
    it. The verifier until PASS, the advisor agent on every pushed head, docs-keeper.
- **His main check of steps 1 to 7 (the plan's section 17: about 35 minutes, Brave then Firefox), a draft;** the
  numbered list with every expected result is written with the part's zip. Where a result hangs on an answer of his,
  the answer is named:
  1. Open the page as the zip's README says, without moving the mouse into the game yet. **Expected:** the scene, the
     stand-in at a spawn of the test scene facing +x (every spawn there faces 90 degrees), the follow camera at his
     Export's framing, no aim line and the readout's aim "none yet" ("no aim yet", section E), "errors: 0"; then the
     mouse into the game: the aim line appears and he turns toward it.
  2. Walk, then sprint until the stamina locks. **Expected:** the walk eases in and out; the sprint is faster, the bar
     shows and drains, and at empty the sprint stops and stays locked until the bar is back at its share.
  3. Jump, and hold the jump key. **Expected:** one jump per press, its top at his reach (3.071 m); held, by MOVE-RULES.
  4. The two steps, then the three ledges. **Expected:** he walks up the 0.55 m step and is stopped by the 0.56 m one;
     a jump catches the 2.9 m and 3.05 m ledges and not the 3.10 m one.
  5. The cover wall and a roof edge (only if he takes MOVE-RULES row 5); a ramp up and down; the thin wall at a
     sprint. **Expected:** by row 5; no hop on the ramps (the snap); never through the thin wall.
  6. Aim on the ground, on a ledge and on a ramp. **Expected:** the aim line ends where the cursor points; he faces it.
  7. The camera panel (backquote): its five presets, "copy values". **Expected:** each preset sets its six values;
     "copy values" gives what TUNE-HOME says.
  8. The tab hidden for 1 minute, and once for 6 minutes, while walking (PM-0002-34); then a covered window.
     **Expected:** by RESUME-FLASH; no jump of the soldier or the camera on return.
  9. The replay button on each screen he has, its refresh rate noted. **Expected:** it says the hashes are the
     recording's (PM-0004-28).
  10. Two minutes of walking the lane. **Expected:** the smoothness numbers in the readout.
  11. "Copy report", pasted into the chat; then steps 1 to 6 once in Firefox. **Expected:** "errors: 0" in both.
- **Risks:** CI's Chromium run uses 3.6 to 3.9 of its 5 minutes (4B-CHROMIUM-TIME): 4c's browser tests may pass it,
  and the fix is a CI edit, the CI Builder's, with his yes first. Others pending the reads.

## H. The advisor agent's verdict on revision 1, and what is left before steps 1 to 7
**Verdict** (`advisor-reread-4c.md`, its section "Verdict"): step 0 and step S APPROVED with conditions 1 to 9 (each
applied above where it lands); steps 1 to 7 not yet. Its rulings on items 2 to 11: item 2, the hold follows the recount
(about 1,665; section D); item 3, the generated TypeScript stays counted (section F); item 4, six reviewers with
art-director's narrow brief (section G); item 5, SWEEP-ROUTE is his, put to him as one question with two readings of
marks (DEPART-4C: our own sweeps with PM-0004-20's and PM-0004-35's new wording, measured on boxes only, the library
then giving the players' file its tree only; PM-0004-60 at 2,000,000 commands in CI's release line against the mark's
10 million nightly through a fuzzer, unless the 10 million is placed at a named change-set; PM-0004-20's hash half on
`walk`, one soldier, against the mark's 48 over 18,000 ticks, with the 48-soldier run and the flag-off timing placed at
the first build that can show them); item 6, every value a later tick reads is in the hashed state, and a tick with no
command keeps the held move and aim as it keeps held buttons today (world.rs:143-145): recommended netcode-architect's
record (i) with the held move in the soldier's hashed row, one home for the bounds derived once from the map's bounds
(metres for the sweeps, whole centimetres rounded inward for the aim) with a test tying them; item 7, pieces before step
1's movement are allowed under (a), or under (b) and (c) once 4c-1 has merged, on six conditions; item 8, one whole
dependency batch (libm in the workspace and `sim`; `content` in the room's dev-dependencies, and the client endpoint's
if its tests need a world; the lockfile by resolve-only commands; `cargo tree -e features -i libm` before and after),
kept as a patch in the notes and not applied, so step S's pushes are never held; item 9, DEPART-4C to him now, the rest
recorded by the advisor agent as dated pointers (PM-0002-14's view-time half at change-set 8; the camera kind at step
4; the stamina share and the sprint lock in the owner's section; first-loop re-made in step 1; the paced spec in
`tests/paced/`; PM-0004-15 on a 64-slot table through the capacity parameter); item 11, under (c) PM-0004-28 is met in
4c-3, with netcode-architect's written yes on the zero ray.

**Before it approves steps 1 to 7 (its item 10):** (1) BODY-SHAPE settled with him on the corrected rows (sent:
question file at 5597b00d); (2) a prototype of the chosen body on both ramps and the seam at x 45, the in-air step-up
(3.05 m gained, 3.10 m not), landing, the snap, a ceiling and the push, with numbers, before the movement code
(rust-reviewer's re-run, running); (3) SWEEP-ROUTE answered; (4) GROUND ruled by engine-architect (recommended: the
bounds' bottom is solid ground, a line in map.rs and a test); (5) FACING-SPAWN picked with its reviewers; (6) the slope:
a grade in percent, and whether a limit exists at all is his (MOVE-RULES row 9); (7) item 6 applied; (8) NO-SOURCE's
rows in the Testlines table with `--check` taking rows with no export, and its plant; any new movement field's version
rule; (9) `Rules::take`'s refusals, and jump tests against `jump_reach_m` as read; (10) the dependency hold answered;
(11) the CI Builder's request for 4B-CHROMIUM-TIME sent now, through him; (12) the recount of steps 1 to 7 with the
part's stop line, its two-thirds sentence and its rungs on the card before step 1; (13) section G's line for steps 1 to
7 replaced by each step's evidence, reviewers and his checklist.

**Where the thirteen points stand (revision 3, 2026-11-28):**
| Point | State |
|---|---|
| (1) BODY-SHAPE with him on the corrected rows | asked (the question file, 5597b00d and 04646ca3); his |
| (2) the body prototype with numbers | blocks 5 and 6 of rust-reviewer's read DONE: (3a), (3b) and (2) (the last with a one-line wall fix) pass the steps, ledges, ramps up, down and from the side, seams, roofs, edges, the thin wall, landings (240 each), the snap (6 each, margin 0.094 m), easing from rest (72 each) and the push's 16 cases; two model holes shared by every body, each one condition and, by rust-reviewer's claim, no rule of his (game-designer's yes at the step's review); not run: the sandbox itself, its set-back along the slope (the wall rule's push-out stood in for it) and headroom formulas, the far corner, a push on a ramp, four or more soldiers; not confirmed: that the sweep's corner path ran; the ceiling ran for its cost only. Open: FALL-STEP, his (the question file's question 11, from game-designer's block 5: whether an unjumped fall steps up onto a slightly higher top; every pass so far assumed yes, so (b) would mean the runs again); the head bump is the sandbox's own rule (MOVE-RULES row 8; game-designer-cs4.md:29), its details not in the notes (rust-reviewer's block 7, B7-C) |
| (3) SWEEP-ROUTE | asked (DEPART-4C); his |
| (4) GROUND | ruled by engine-architect (block 5): the bounds' bottom is solid ground; its words in step 1's commit, tests T1 to T3; three findings (G-1 Low and G-3 Low on wording and step S's test; G-2 Medium if built as written, the aim's fallback plane, netcode-architect's NA1, running) |
| (5) FACING-SPAWN | **picked with its reviewers: (a)**, one `libm::sincos` per spawn at the match's start, in f64 then narrowed, quarter turns exact, no -0.0 (engine-architect's block 5; rust-reviewer's block 7, RR1 to RR5; netcode-architect's block 8, NA3: nothing on the wire or in the recordings changes); nothing to him; the batch reworded for it (`deps-4c-v2.diff`), security-auditor answering on it; the held aim at spawn by "no aim yet" (section E) |
| (6) the slope | his (MOVE-RULES row 9; the unit recommended a grade in percent) |
| (7) item 6 applied | applied in section E ("The record rule") and confirmed by rust-reviewer and netcode-architect |
| (8) NO-SOURCE's rows in the Testlines table, `--check` and its plant, the version rule of new fields | waits for NO-SOURCE and BODY-SHAPE (which fields); designed in E1 |
| (9) `Rules::take`'s refusals; jump tests against `jump_reach_m` | designed (E1; G step 1); built in step 1 |
| (10) the dependency hold | answered: APPROVE (`security-auditor-deps-4c.md`, `dependency-check.md`), F1 and F2 on the commit that applies it |
| (11) the CI Builder's request for 4B-CHROMIUM-TIME | asked of him in the chat (it goes through him); his |
| (12) the recount, the stop line, two thirds, the rungs | section F2, for each SIZE-4C option; the card takes his option's numbers before step 1 |
| (13) section G for steps 1 to 7 | section G, with his checklist as a draft |
The advisor agent is asked to approve steps 1 to 7 only once his answers on (1), (3) and (6) are in, with blocks 5 to 7
folded in.

**Revision 1's asks, as they were put (kept for the record):**
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

## I. Revision 4 (2026-11-28, from 13:00 Amsterdam by the clock): his answers of 12:10 applied to steps 1 to 7
His message is saved by script (`.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md`, 4,516 bytes, sha256
3b14fb63...) and is in the register (step 0). Where this section and the text above differ, this section holds.

### I1. The shape of 4c from here (his SIZE-4C (b) and his item 6)
- **4c-1 is step 0 and step S**, finished now: the six reviews, one fix pass, the verifier, the docs pass, the report,
  the pull request's title and text in the notes, the notes out of the head, a green CI run, the advisor agent on that
  head; then he is told the final head with its full id and the Brave steps (his item 6a).
- **4c-2 is steps 1 to 7, built on the same branch on top of 4c-1's final head** (his item 6b). On Sunday his Advisor
  points a branch of its own at exactly that head and opens 4c-1's pull request from it, so this branch's pushes never
  move it (6c). A change he wants in the scene after his Brave check goes into 4c-2 (6d). When 4c-1's merge is on main,
  this branch takes main in once (6d).
- **4c-2's count** starts from 4c-1's final head (the same lines as from the main that will hold 4c-1's merge, since
  4c-2 sits on top of it), by `count-4c.mjs`'s method, the generated TypeScript counted (the advisor agent's item 3).
- **4c-2's stop line: 18,700 written lines** (his word). The two-thirds sentence at about 12,470. "If the count
  reaches it before the early look is ready, do not stop: tell me in two lines in your next report and build on to
  the early look. Past the line, nothing after the early look starts without my word." So the early look (after step
  5) is never cut by the line; the steps after it (6, 6b, 6c, 7) wait for his word if the line is passed.
- **The rung (what leaves first), for the advisor agent to rule:** the plan's ladder moves the far-corner map
  (PM-0004-39, step 6c) to 4d at 13,150 written lines, the plan's planned size for a stop line of 16,450. His new line
  is 18,700; the plan's rule would put the rung at its planned size, 18,700 / 1.25 = about 14,960. Either way he is
  told before it moves, and nothing else moves.

### I2. The movement (step 1) under his answers
- **MOVE-RULES (a), as starting rules,** each his to change after he has played. Rows 6 and 7 (Q aims too; the
  sights camera) are 4d's and confirmed again there. **Row 9 is open, not decided:** the soldier walks any slope a
  tick's step allows, as his sandbox does, and **nothing is built that depends on roofs being walkable or not** (no
  test, recording route, mark or readout asserts that a roof is or is not walked; the yard's and street's tests stay
  off the roofs; the walk recording's route keeps off them). He decides a slope limit after he has walked the roofs in
  the early look.
- **BODY-SHAPE (3a):** his sandbox's body: the 0.4 m body for walls and the step trigger, the 0.15 m foot for standing,
  the pull, the 0.2 m snap down with its slope term (rust-reviewer's blocks 5 and 6; the 2.5D form honestly counted at
  about 205 / 265, the head test and the stood-on slope included). The two model holes are fixed in step 1, one
  condition each (the in-air step's "toward" from the tick's start; a top crossed inside one tick counts as landed),
  with a test and a plant each, and game-designer's yes at the review that neither is a rule of his.
- **NO-SOURCE (b):** the game starts from his sandbox's numbers, each a row of the content marked "not from his
  Export": the foot radius 0.15 m; the pull's half time 1/30 s, its done-under 0.03 m and its 90 % of the foot; the
  step allowance 5 mm (**his answer takes the sandbox's number**, where this sheet had recommended none: the 0.55 m step
  is still gained and the 0.56 m step still stopped, so PM-0004-30 holds either way); the snap 0.2 m; the push's height
  gap 1.0 m, its sideways share 0.7, its keep-going share 0.5, its most 0.25 m a tick, and its two speed gates 0.2 and
  0.1 m/s. Each number's sandbox file:line comes from the Testlines read (I4). The Testlines table gains rows with a
  `field` and no `export`, which `testlines.mjs --check` takes, with a planted row that claims an export it does not
  have; each new field of the movement kind states its version rule (4B-FIRST-DEFAULT). A later Export of his that
  carries one of them replaces it the same way (his words: "I may make them sliders in Testlines later, and a later
  Export then replaces them the same way").
- **FALL-STEP (c), by reading:** the game does what his sandbox does; if the sandbox's code does not settle it, (a)
  (an unjumped fall steps up onto a slightly higher top, as the old game does). The Testlines read (I4) settles it
  before step 1's movement is written.
- **The head bump** (MOVE-RULES row 8) and **the ramp met from the side** (the sandbox's set-back along the slope):
  the Testlines read settles both, numbers and behaviour only.
- **DEPART-4C (a), on his terms:** (1) **SWEEP-ROUTE:** our own sweeps; **the collision library stays the check on
  them for every shape the game uses, the ramp included, before the movement step counts as done** (a test that runs
  every form pair through the library and compares, so the ramp's sweep is no longer untried); the report shows the
  old and the new wording of PM-0004-20 and PM-0004-35 side by side, **and the new wording asks no less than the old
  about what the players' file does.** (2) **PM-0004-60:** 2,000,000 hostile commands per CI run now; **the mark is
  not met by that: it stays open in STATUS until the nightly job runs its 10 million, named with the change-set that
  brings it.** (3) **PM-0004-20's 48-soldier run is not met by the one-soldier recording: it stays open in STATUS,
  named with the first build that can show it.**
- Settled without him and confirmed by the reviewers (section E): the floor (the bounds' bottom is solid ground), the
  spawn facing (FACING-SPAWN (a), `libm::sincos`, in the batch `deps-4c-v2.diff` answered by security-auditor), the
  record rule (the held move and aim in the hashed row), "no aim yet" (his "fine"; he looks at it in the early look).

### I3. The page (steps 3 to 5) under his answers
- **RESUME-FLASH (b):** as (a) (the match as it stood; a tick later the keys count as let go, and the soldier slows to
  a stop), and the development key diagram goes dark the moment the tab hides (netcode-architect's block 5: about 10 /
  20 more; one test with its plant).
- **TUNE-HOME (b):** movement, jump, stamina, pushing and the aiming rule stay in his sandbox (he exports, the tool
  rewrites the content, the check holds them equal); the camera's values start from his Export once and then live in
  the game's camera file, tuned in the panel on the real picture, with "copy values" giving the file's bytes to paste
  into the chat (engine-architect's read 3.6: (b) as designed).
- **STREET-GREY (b)** stands as built (he looks at it at the scene's Brave check).

### I4. The Testlines read (his TESTLINES yes)
"Yes, you may read my Testlines repository by its public address, read only. Copy no code from it: numbers and
behaviour only. Do not attach it to this chat as a second repository." game-designer reads it for: FALL-STEP; the
head bump (what it does to the upward speed and the position, which circle and height it tests, the headroom rule);
the ramp met from the side (the set-back along the slope); every NO-SOURCE number with its file:line at the commit his
Export names, and whether the branch moved since 703fd922. Nothing of its code enters this repository: the read's
notes describe rules in plain words with file:line.

### I5. CI's Chromium time (his CHROME-TIME)
The CI Builder is parked until Tuesday 1 December, 20:00, and raises the Chromium run's limit then. Until then no test
is cut or thinned to fit the 5 minutes; a run stopped at the limit is re-run once (`gh api -X POST
repos/mergincanli/shamble-frontlines/actions/runs/<run>/rerun-failed-jobs`); stopped again, he is told in two lines and
the build goes on. A head that goes to the advisor agent needs a green run.

### I4b. The Testlines read's result (game-designer's block 6, 2026-11-28; read at be608cad, the branch unmoved)
- **FALL-STEP is (a), by the sandbox's code:** a fall without a jump steps up onto a top no more than 0.555 m above
  his feet. The sandbox tests the feet at the end of a falling tick (movement.ts:131), so walking off the 0.55 m step
  he reaches the 0.56 m step 2 m away only about 1 time in 10 at 6.3 m/s, and every time at 9.576 and 13.27 m/s
  (rust-reviewer's block 6 row 597, "caught at all 3 speeds", is overturned). PM-0004-30 holds as worded.
- **The head bump is rust-reviewer's C2:** while airborne, the lowest underside over his 0.4 m circle at the tick's
  start caps his feet at that underside less 1.8 m; the upward speed goes to 0, a downward one is kept; a top with
  less than 1.8 m of room above it is a wall.
- **A ramp met from its side:** with his centre outside the slope's width, he is pushed straight out where the
  slope's height at the side point nearest his centre is above feet + 0.555, and steps onto it below that; with his
  centre within the width, he is set back along the slope to the height feet + 0.55 (yard.ts:318-334).
- **The numbers:** none differs from I2's list. The pull's 0.5 is a share of what is left per tick, not a time
  (movement.ts:62; so "a half time of 1/30 s" is a rewording, G11); the 5 mm allowance is not used in the set-back
  (yard.ts:325). Tolerances, not tuning: the slope-head rule's 1 cm and 5 cm (yard.ts:280, :265), the jump's 0.01 m
  minimum (movement.ts:105, a guard), the 1 mm head allowance and the 1e-6 shrinks.
- **Where the prototype is not the sandbox** (its findings; each for step 1's design, nothing in step S): **G1 (High)**
  the sandbox fixes the pull when he steps up and adds it on top of his walk from that tick (each step-up carries him
  0.055 to 0.265 m further, movement.ts:169-192), the prototype recomputed it each tick; **G2 (Medium)** the sandbox's
  own "moving toward it" clause (a centre already over the footprint counts, yard.ts:251-252) makes rust-reviewer's
  B6-5 fix unneeded: build the sandbox's clause; **G3 (Medium)** end-of-tick feet on falling ticks; **G4 (Medium)** the
  slope-head rule (a rise of at most 0.04 or 0.08 m at the x 45 seams, not the prototype's 0.18 m float); **G5
  (Medium)** headroom and no step onto a block whose bottom is above his feet; **G6 (Medium)** the set-back, copied as
  written, would carry him through a ramp met from beyond its high end (no such ramp in the sandbox or the test scene):
  recommended, set back only while his centre is over the slope (a guard for a case the sandbox never meets: for the
  advisor agent to rule whether that is his); Lows G7 to G11 (the head bump's tolerances, the snap's slope term, the
  landing fix, which game-designer says is no rule of his, the push using each soldier's own velocity, the pull's
  wording), the push's height test (feet less than 1.0 m apart) and the 1e-4 m/s rest speed.
- **Next:** rust-reviewer's block 9 re-runs the prototype for body (3a) on G1 to G6 and the head bump, before the
  advisor agent is asked for steps 1 to 7.
- **rust-reviewer's block 9 (DONE, `rr-body-proto-7.rs`):** block 5's cases 69 of 69 pass (the x 45 seams rise at
  most 0.035 m at walk and sprint; at 13.27 m/s, above his 9.576 m/s sprint, 0.125 m or 0.035 m by the slope-head
  window's reading); block 6's: landings 240 of 240, the snap 6 of 6, easing 72 of 72, off a ramp's side 24 of 24; the
  sandbox's "already over it" clause leaves 0 ticks inside a block in 36 of 36 (its "from the tick's start" fix
  withdrawn); the head bump puts the feet 0.20 m under a 2.0 m beam, then he falls back; G3 measured: the 0.56 m step
  caught in 3 of 30 phases at 6.3 m/s, 30 of 30 faster; G5 holds. Not passing, all at 13.27 m/s: ramps met from the
  side 264 of 306, and G6's guard still carries him 6.98 m; the pull (G1) puts him up to 5 cm inside a wall for 3 ticks
  in 7 of 9 runs on a made map. **Step 1's design changes:** the tick in the sandbox's order (vertical first, then the
  move); the Soldier row's pull as the remaining vector `pull: [f32; 2]`; the wall pass sweeps the body circle (it
  closes every 13.27 m/s case and changes nothing at his speeds) and clips the pull; the slope-head rule, headroom, "no
  step from below" and the guarded set-back added. Count: the 2.5D movement about 275 / 365 (from 205 / 265), step 1
  for (3a) about +70 / +100. Open for game-designer (its block 7, asked): the slope-head window; whether a top at or
  below the feet can be a wall by headroom; whether clipping the pull, the swept wall pass and G6's guard are rules
  of his.
- **game-designer's block 7 (DONE):** the slope-head window is his radius (0.4 x grade, yard.ts:264), but its top also
  adds the step-up, feet + 0.55 + 0.4 x grade + 0.005 (:265), which the prototype leaves out (Medium: narrower than
  the sandbox; a re-run to confirm its predicted 0.0175 / 0.035 m seam rise at 13.27 m/s); the last slope test is
  "slope height at the box's nearest point >= box top - 0.55" (:281, Low). **A top at or below his feet can be a wall
  by the sandbox's headroom test** (it ignores the feet: :185-194, :306-309), and he can meet it at his Export's sizes
  (a 3.0 m wall with a window: stopped 0.4 m before the stretch over it): rust-reviewer's "never a wall at or below his
  feet" is kept as recommended but is a difference from his sandbox, so it is his: **HEADROOM-FEET**, the question
  file's question 13 (his item 18). Not rules of his: the pull's clip (on condition the carry stays bit-equal wherever
  no wall is in reach), the swept wall pass (his DEPART-4C (a); "no change at his speeds" not yet measured for the
  walk), G6's guard. Two of block 9's assumptions differ from the code (Low): the set-back removes his speed up the
  slope (movement.ts:147-157); standing has no test on upward speed (movement.ts:195). **Next for step 1:** rust-
  reviewer's block 10 (the 0.55 in the slope-head window, the swept wall pass measured at 6.3 and 9.576 m/s, the two
  assumptions corrected), then the advisor agent on steps 1 to 7 with HEADROOM-FEET open.
- **rust-reviewer's block 10 (DONE; `rr-body-proto-7.rs`, outputs `rr-body-proto-8-*.txt`, bit traces under
  `.gate/rr7/`):** with the 0.55 in the slope-head window and the :281 test, the x 45 seams rise at most 0.0175 /
  0.035 m at every speed, 13.27 m/s included (game-designer's prediction holds); block 5 still 69 of 69; the set-back's
  speed removal and standing with no speed test change no printed result; with the old switches it reproduces block
  9 byte for byte. **The swept wall pass is not "no change at his speeds"** (compared tick by tick on bits): bit-equal
  in the open and on every face contact; at 6.3 m/s 4 of 341 block 6 runs differ, all one push case rounding the 3.10 m
  ledge's corner, by up to 0.063 m; at 9.576 m/s the six roof jumps differ in path by up to 0.51 m with the same outcome,
  and one landing grazing a corner by 4 micrometres. **The report says so, with these numbers** (his DEPART-4C (a)
  took our own sweeps; this is how they differ from the sandbox's push-out at corners). At 13.27 m/s the sweep closes
  all three holes (ramps from the side 306 of 306, against 264 with the push-out; the high-end case stops 0.40 m out
  with or without G6's guard; the tick inside the beam gone); two bugs of its first sweep found and fixed (a start
  exactly on a face read as no contact; a raised ramp swept at its low end lost the roof's edges), each to get a test
  that plants it. The pull's clip is bit-equal to no clip on every block 5 and 6 run at 1, 2, 6.3 and 9.576 m/s (only
  where a wall is in reach does it differ: game-designer's condition holds). HEADROOM-FEET measured both ways: (a) he
  walks the 3.0 m window wall's whole top and an upper floor over a 1.5 m crate; (b) he is stopped 0.40 m before the
  window stretch and before the crate, and dropped onto the stretch is put off the wall; on the test scene (b) changes
  only the beam-over-step flicker. **Step 1's design:** the wall pass and the pull share one sweep (contacts placed
  exactly on the face, no skin, one slide; ramps swept only at their sides and an exposed high end); the window with the
  0.55; the set-back's speed removal; landing with no speed test; HEADROOM-FEET a flag in the movement rules. **Count:**
  the movement about 292 / 430 (from 205 / 265 in F2), so step 1 about +87 / +165 over F2's 2.5D figure, about 3,350 in
  all; steps 1 to 7 about 8,550 by the designs' count (F2: 8,295 to 8,395); at the measured rates about 18,550 to 23,100
  written lines against his stop line of 18,700, and the early look (steps 1 to 5, about 7,080) at about 15,360 to
  19,110: his rule for the line before the early look applies ("do not stop: tell me in two lines in your next report
  and build on to the early look").

### I6. What the advisor agent is asked, for steps 1 to 7 (after the Testlines read is folded in)
Its item 10's thirteen points stand as section H's table, with his answers on (1) BODY-SHAPE, (3) SWEEP-ROUTE and
(6) the slope now in; the asks: APPROVE or REVISE steps 1 to 7 as revised here; the rung (I1); and the order inside
4c-2 (its item 7's six conditions, now that SIZE-4C is answered and 4c-2 sits on top of 4c-1's final head).
