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
