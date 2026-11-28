# engine-architect: review of step S (the page's scene, 4c-1), code head c1de41e5 on main dae6d107

Started 2026-11-28. One round (CLAUDE.md's review budget). Read-only; this file is the only one I write.
Reviewed: `git diff dae6d107 c1de41e5 -- packages crates content` (23 files, +1,288 / -133). Brief:
`review-briefs-step-s.md` "Common to all six" and item 3. His answers (2026-11-28): SIZE-4C (b), STREET-GREY (b).
Against my own design (engine-architect-read.md block 1) and today's code, not the design's wording.

## Plan
1. picture.ts: the interface, what crosses it (`PictureContent`, `SceneObject`, `setContent`, `inventory`), type-only
   imports, no reference kept after `setContent`, who owns what state.
2. `LOCAL_MAP` in sim-worker/src/local-map.ts: what the page imports from `@shamble/sim-worker` (nothing of
   recording.ts), the tie to sim-wasm/src/lib.rs, index.ts's exports.
3. `readMap` and `readContent` (content.ts) against crates/content/src/map.rs (and movement.rs, bundle.rs) field by
   field; the ties test (content-ties.test.mjs) and content-map.test.mjs; content-schema.test.mjs (4B-MEMBER-KINDS).
4. The development cut: three-picture.ts's `inventory`, render/inventory.ts, boot.ts's block, build-checks.mjs's
   markers; whether the production page holds `inventory(){return null}` and neither marker (build it if the toolchain
   is here; else read the checks' code).
5. 4B-TYPES-READER, 4B-MEMBER-KINDS, 4B-BOOT-CALLS: met or not, with the line that meets each.
6. Dependency direction (render/ imports nothing of app/; app/ imports only createPicture from render/), hidden
   coupling (boot's hand-off, the readout's data at delivery), boundary cost (nothing per frame).
7. G-3 (my block 5): confirm or update against the code; address or defer.
8. Verdict, findings ranked, DONE.

## Notes (one line per finding, as I go)
- N1 (plan 1, interface). picture.ts:6 type-only import of `MapDocument`; `PictureContent { map }`, `SceneObject`, `setContent`, `inventory` as designed (block 1.1). three-picture.ts:42-45, :69-77 keeps only the bounds' six numbers (`framed`), no reference to the document; `buildMapFloor` keeps none (ids are copied strings). State owners: the floor and `framed` the picture's; `delivered` boot's (development); `PageContent` keeps nothing after delivery (content.ts:303-313). render/index.ts still exports `createPicture` alone; the app imports nothing new from render/. Holds.
- N2 (plan 2, LOCAL_MAP). sim-worker/src/local-map.ts:7 alone; index.ts:5 re-exports it; recording.ts:6-9 imports and re-exports the same binding; the boundary test (boundary/sim-wasm.test.mjs:18, :194-196) still ties it to sim-wasm/src/lib.rs:25 through that binding. The import graph from index.ts (local-map, messages, panic-message, read-block types, stats) never reaches recording.ts; neither built page holds "SHMBLREC" or "TESTSEED" (grep of .gate/dist-production-4c and dist-development-4c). Holds.
- N3 (plan 3, readMap against map.rs, field by field). Top: format, version, id, bounds, spawns, shapes, regions, closed (map.rs:173-241 reads the same and `finish`es each object). Bounds: min, max, three numbers each, closed. Spawn: id, at (3), facing_deg (number), closed. Shape: id, min, max, type in box|ramp; rises_toward only on a ramp (map.rs:200-209: read only for a ramp, so a box's is an unknown key), closed per form. Region: id, kind in street|grass, min (2), max (2), closed. Array entries must be objects (map.rs:160-168 `Object::of`). Duplicate keys and "-0" never reach the page (json.rs:330, :570-572; :602-603). The ranges, grids, id grammar and "inside" are the loader's, not repeated (content.ts:7-10). Matches, member for member.
- N4 (plan 3, version). readMap asks `version === MAP_VERSION` (content.ts:158) where the loader takes 1..newest (read.rs:273) and readContent too; equal while newest is 1, and content-map.test.mjs would fail on the committed file the day MAP_VERSION moves: caught, not a finding.
- N5 (plan 3, ties). content-ties.test.mjs ties MAP/MOVEMENT FORMAT and VERSION (map.rs:23-24, movement.rs) and FOLDERS' order (bundle.rs:31) to CONTENT_KINDS; content-map.test.mjs reads the committed test-scene.json and asserts readMap gives the file less its header, toStrictEqual, with 8 / 23 / 6. content-schema.test.mjs's new test compares each member's kind, its plant inline (facing_deg as string). 4B-MEMBER-KINDS met.
- N6 (plan 4, the development cut). three-picture.ts:79-81 `__SHAMBLE_DEVTOOLS__ ? readInventory(floor) : null`; boot.ts:64-71 counts once at delivery inside the define; :172 `__shambleScene` inside the development block. The production page built at 00:24 (.gate/dist-production-4c, from the step's WIP, before bc7720f3) holds `inventory(){return null}` and neither marker; the development page holds both. build.mjs:292 runs `bundleProblems` (DEV_MARKERS, build-checks.mjs:205-213) on every build, so CI's builds of bc7720f3 hold the committed code to it. Holds.
- N7 (plan 5). 4B-TYPES-READER: readMap returns `Outcome<MapDocument>` built from typed literals with no cast of the tree; a field added or removed in Rust breaks tsc (missing or excess property). Met for the keys. 4B-BOOT-CALLS: E1 (skeleton.spec.ts, the scene and data.scene through boot's own message path) and E3 (the broken Workers: content "none", where a boot calling `link.workerError` alone would leave "holding"). Met.
- N8 (finding, the overview's fit can loop forever). scene.ts:286 `while (!fits(far)) far *= 2;` has no bound, and `fits` is false at every distance when the aspect is 0 (three-picture.ts:100 sets `camera.aspect = width / Math.max(1, height)`, so a 0 width gives 0; :48 frames with it on every resize once a map is drawn) or when the corners pass the internal camera's far plane (scene.ts:272, 1e6 m) before they fit. Measured here (node 22.23.5 with --experimental-strip-types on the head's scene.ts, `timeout`): aspect 16:9 returns in 2 ms, 0.001 and 0.0003 in 2 ms (435 km away), 0.0001, 0.00005 and 0 never return (killed at 10 s and 30 s, exit 124). So a window whose page width is 0 after the map is framed (or at the first setContent) blocks the page's main thread for good; before this step a 0 aspect only drew nothing. Whether Brave ever reports an innerWidth of 0 (a minimised or background window, a zero-width frame) is not known to me and not tested. Medium (a hang, small reach). Fix: `frame()` skips a non-positive or non-finite aspect, and the doubling is capped (a fixed count), so the loop's end never rests on its input; a test at aspect 0.
- N9 (finding, G-3 confirmed). scene.test.ts:21 builds its map at min y 0 and :149 asserts the literal 0, while scene.ts:197 draws the ground, the regions and the grid at `bounds.min[1]`; a planted `floorY = 0` passes every unit test and E1 (test-scene starts at 0). Low. Rides a fix pass if N8's runs (the same test file), else the soldier's pull request (his answer 6.d puts scene changes there).
- N10 (finding, readMap's word lists are not held to the generated unions). content.ts:182, :188, :196 list the words inline; `word()` returns a subset union, which tsc accepts against a wider generated one. If map.rs grows a word (FORMS, TOWARDS, GROUNDS at map.rs:66-68), tsc fails in scene.ts (PATCH_GREYS is a `Record<Ground, number>`, scene.ts:21; `solidPoints`' switch and form narrowing), but not in readMap, which then refuses every map using the word with NOT_THE_MAP ("from different builds", untrue then) until the committed local map carries it and content-map.test.mjs fails. Low (latent; caught by a test before a player meets it, with a misleading sentence). Fix: tie the three lists to map.rs's constants in content-ties.test.mjs (export them), or make each an exhaustive `Record<Ground, true>`-style table.
- N11 (finding, inventory's kinds are untyped). inventory.ts:13 `KINDS: readonly string[]` and :32 `kind as SceneObject["kind"]`: a kind added to KINDS alone compiles and crosses the interface as a value outside `SceneObject.kind` (picture.ts:26); :31 casts the material (an array material throws a TypeError, not INVENTORY_UNKNOWN). Four copies of the kind list (picture.ts:26, scene.ts:27, inventory.ts:13, e2e report.ts). Low. Fix: `as const satisfies readonly SceneObject["kind"][]` and a guard.
- N12 (finding, the readout's "none" for a page-side refusal). boot.ts:166 and report.ts:79: when the page reads the copy and refuses it (NOT_THE_MAP or NOT_A_BUNDLE after `ready`), `delivered` stays null and `holding` is false, so the readout says "content: not read, the match stopped first" and the report `content: none`, beside a notice and an errors row that say the page refused it. Development readout only; the notice carries the truth. Low. Fix: the sentence for "none" true in both cases (for example "not drawn: the match stopped"), or a third state.
- N8, reach pinned. The same run on the loader's widest bounds ([-327.67, 0, -327.67] to [327.67, 655.35, 327.67], map.rs's aim reach) returns in 2 ms at 16:9 (750 m), 1:1 (1,010 m), 9:16 (1,604 m), 0.2 (4,066 m) and 0.05 (15,522 m). So no ordinary window on an allowed map hangs: the trigger is a degenerate width (0, or about 1:10,000). Medium stands. It lies in graphics-engineer's part too (the fit by halving) and may duplicate its finding; kept because it is measured.
- N13 (finding, wording). scene.ts:185 and scene.test.ts:135 still call STREET-GREY (b) "the default ... until he says otherwise"; he answered (b) on 2026-11-28 (mustafa-2026-11-28-answers.md item 10). Low. Fix: "his STREET-GREY (b), 2026-11-28".
- N14 (a departure, not a finding). E1 as built (skeleton.spec.ts) checks each solid's x extents and top y in Chromium, where my design row E1 named the full `Math.fround` corners. The chain still closes: content-map.test.mjs holds readMap to the committed file (toStrictEqual), scene.test.ts S1 holds every corner of every form from a MapDocument, and the browser runs the same code on the same file.
- N15 (the approval terms). The advisor agent's conditions on step S that touch my part (advisor-reread-4c.md:45-53): 1 met (`INVENTORY_UNKNOWN` and its sentence are the marker, inventory.ts:11, build-checks.mjs:212); 3 met in the code (two greys, S3 asserts they differ), his letter is in. B3 Q2 (advisor-cutC.md at 59f6cc7): condition 1 met (no cast of parsed content to a generated type; `word` and `objects` narrow only after their checks); condition 2 met (every field of `ReadContent` has a reader: `picture` in every build, `files` in the development readout; `spawns` are read because the generated type names them and the content-map test holds the whole file). The rest are process conditions, not code.
- N16 (4B-BOOT-CALLS, measured). plants-4c.txt:25 (E1 CAUGHT: boot taking the Worker's messages through the link alone, no map drawn) and :28 (E3 CAUGHT: boot taking the Worker's error through the link alone, the copy stays held). Met by plant, not only by reading.
- N6, on the head. CI's toolchain job builds the production page with the scan (toolchain.yml:181, build.mjs:292) on every head; CI was green on bc7720f3, and c1de41e5 changes only scene.test.ts after it. The local checks' production page (.gate/dist-production, built 00:47:48, five seconds after bc7720f3, local-checks-4c.log:1512-1522, "ALL LOCAL CHECKS PASSED" :1640) holds `inventory(){return null}` and neither marker; its development page holds both. Holds on the head's code.
- N10, method: the claims that tsc fails in scene.ts when a word list grows are from reading the types, not from a run.

## Verdict: APPROVE

The boundary is as designed and holds against today's code: the interface file carries a type-only `MapDocument` record and a
development-only read-back; the picture keeps six numbers of the map and nothing else; the page reads the map field by field,
member for member as map.rs does, tied to the loader by the ties, map and schema tests; the Worker's package hands the page
`LOCAL_MAP` without recording.ts; the production page holds `inventory(){return null}` and neither marker. 4B-TYPES-READER,
4B-MEMBER-KINDS and 4B-BOOT-CALLS are met (the last by its two plants). Nothing is added that no VISION §6 item needs; no
allocation per frame (the inventory and the fit run at delivery and on resize only).

Findings, ranked (the one scale):
1. **Medium: the overview's fit can loop forever.** scene.ts:286 (`while (!fits(far)) far *= 2;`, no bound), fed by
   three-picture.ts:100 and :48. Failure case: a page whose width is 0 (or about 1:10,000) when the map is framed or on a
   later resize: `overview` never returns and the page's main thread hangs for good (measured: aspect 0 and 0.0001 killed at
   10 to 30 s; ordinary aspects on the widest allowed map return in 2 ms). Fix: `frame()` skips a non-positive or non-finite
   aspect; cap the doubling at a fixed count; one test at aspect 0. **For this round's fix pass**, not a STATUS follow-up:
   once the scene's head is frozen for his Sunday check, scene changes land in the soldier's pull request (his answer 6.d),
   so if it misses the fix pass it is the soldier's pull request's first commit.
2. **Low: G-3, no test tells the bounds' bottom from y 0.** scene.test.ts:21 (map at min y 0), :149 (the literal 0) against
   scene.ts:197 (`floorY = bounds.min[1]`). Failure case: a planted `floorY = 0` passes every unit test and E1. Fix: one test
   on a map raised above 0. Rides finding 1's fix pass (the same file); else the soldier's pull request.
3. **Low: readMap's word lists are not held to the generated unions.** content.ts:182, :188, :196. Failure case: a word added
   to map.rs:66-68 (and its enum) compiles in readMap, which then refuses every map using it with NOT_THE_MAP ("different
   builds", untrue) until the local map carries it and content-map.test.mjs fails. Fix: tie the lists to map.rs's constants
   in content-ties.test.mjs, or make each an exhaustive table typed by the generated union.
4. **Low: the inventory's kinds are untyped.** inventory.ts:13 (`readonly string[]`), :32 (`kind as SceneObject["kind"]`),
   :31 (material cast). Failure case: a kind added to KINDS alone crosses the interface as a value outside
   `SceneObject.kind` (picture.ts:26); an array material throws a TypeError instead of INVENTORY_UNKNOWN. Fix: `as const
   satisfies readonly SceneObject["kind"][]` and a guard.
5. **Low: the readout says "not read" for a page-side refusal.** boot.ts:166, report.ts:79. Failure case: page and Wasm from
   different builds: the notice says NOT_THE_MAP while the readout says "content: not read, the match stopped first" and the
   report `content: none`. Fix: a "none" sentence true in both cases, or a third state.
6. **Low: STREET-GREY still called a default.** scene.ts:185, scene.test.ts:135. He answered (b) on 2026-11-28. Fix: the
   comment names his answer.

Lows 3 to 6 trigger no push of their own (the review budget): they ride the fix pass if one runs, else STATUS follow-ups
or the soldier's pull request.

Not verified: nothing run in a browser by me; Brave's innerWidth on a minimised or background window (finding 1's real
reach in his browser); finding 3's tsc claims (read, not run); the production page built from c1de41e5 itself (CI's build of
bc7720f3 and the local build five seconds after it stand for it; c1de41e5 adds only a test).

DONE
