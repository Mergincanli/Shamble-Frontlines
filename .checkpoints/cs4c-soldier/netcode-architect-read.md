Task: netcode-architect's design read for change-set 4c ("The first soldier"), against main dae6d107 (worktree
`.gate/cs4b-2`, branch cs/4c-soldier), 2026-11-28. Input to the main session's re-read sheet (plan.md section 3 item 1).
Read only; this file is my only write. My design: `.checkpoints/cs4-walk/netcode-architect-cs4.md` (blocks 1, 3-5).

## Plan (blocks, in order)
0. Read today's code: protocol (messages.rs, codec.rs), client-endpoint (post.rs, endpoint.rs, layout.rs, block.rs),
   room/lib.rs, sim (lib.rs, world.rs, local_match.rs, collision.rs's StaticWorld and cast_ray), sim-wasm/lib.rs,
   devtools (recording.rs, replay, make_recordings, recordings/ with first-loop and first-loop.content.json),
   sim-worker (worker.ts, host.ts, messages.ts), client app/worker-link.ts, e2e replay/determinism.spec.ts; 4b's report;
   STATUS_ARCHIVE :2278-2300. A notes line per finding.
1. Step 2, the command path: symbols to add / change / remove (file:line), tests with planted defects, count
   recounted, what changed since my design.
2. Step 3, the snapshot's soldier and the block (timing side): header count, the due time per block, three newest
   blocks (DEPART-4) with the test that fails with two, 3B-POOL-16. Symbols, tests, count.
3. Step 6, the movement and lane recordings: what each records, on which frozen content; first-loop made anew; the
   three plants on movement; PM-0004-17, -19, -20, -21, -24 with test and plant; 4B-QUERY-RECORDING (native, Node,
   Chromium); 4B-RAY-WASM. Symbols, tests, count.
4. Steps 6b and 6c: paced replay in the real Worker (PM-0004-28); 2,000,000 hostile movement commands (PM-0004-60);
   far-corner map (PM-0004-39, the 13,150 rung). Symbols, tests, count.
5. RESUME-FLASH by option (a)-(d): what the code does, and where.
6. Order: can steps 2 and 3 be built before step 1's movement if MOVE-RULES / NO-SOURCE come late, without building
   twice? Facts only; the advisor agent rules.
For each step: which of his open questions (MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME, SIZE-4C) changes what,
by option. Then DONE.

## Notes log (one line per finding)
- N1 (base): HEAD deb337ad = main dae6d107 + notes. My design is `.checkpoints/cs4-walk/netcode-architect-cs4.md` (2026-11-25, against 311c0484); every symbol below is re-read on today's code.
- N2 (post): `client_endpoint::post::Post{number, buttons, presses}` only, `from_words(number, held, counts)` (post.rs:20-48): no move, no ray yet. `Sim::input` takes six numbers (sim-wasm/src/lib.rs:166-169); the host keeps six (host.ts:57-64, :81, :103-118); the page posts six (worker-link.ts:346); the guard checks six (messages.ts:140-151).
- N3 (endpoint): `build_input` writes move 0, aim {0,0,0}, view {newest snapshot tick, 0} (endpoint.rs:251-263); `Sent{sequence, tick, view_tick, post}` (:83-88). The endpoint holds no static world.
- N4 (world placement, 4B-WORLD-ROOM): the one `StaticWorld` is owned by `LocalMatch` (local_match.rs:48-50, :63, :96-100), "no step queries it yet"; `Room::new(setup, content_hash)` takes no map (room/lib.rs:105-118); `World` holds tick, player, seed only (world.rs:112-117). `StaticWorld` derives Clone (collision.rs:94).
- N5 (cast): `StaticWorld::cast_ray(origin: V3, direction: V3, limit: f32) -> Option<RayHit{shape, t}>` (collision.rs:300-345): no hit for any number beyond RAY_REACH 2^20 or NaN, zero direction, negative/NaN limit; limit capped at RAY_REACH; ties to the lowest index; allocates nothing. The point is ours to form (origin + direction * t).
- N6 (legacy aim, corrects my design 3.7): the old game casts with far = 400 m and falls back to the GROUND plane (y = 0), keeping the last ground point when the ray never crosses it (legacy/shamble-game.js:1120-1126, `raycaster.far=400`, `intersectPlane(groundPlane)`, `aimPoint.copy(aimGround)`); my design said 500 m and the plane at fire height. Plan section 10 already says "the cursor ray's first hit, else the ground". Facing turns only beyond 2 cm horizontally (:2708-2709).
- N7 (bounds): the loader holds a map's bounds inside the aim's reach, on the centimetre grid (content/src/map.rs:9-14, :316-335; AIM_REACH_CM 32,767 both ways, AIM_HEIGHT_MAX_CM 65,535; Mustafa 2026-11-26 "until the aim codec widens, the loader refuses a playable map larger than its reach"). So the room's aim check is integer compares against the bounds in cm, and no loaded map can hold a far corner at 1 km: PM-0004-39 can only be a sim-level map built in code (map.rs:15-16 says such a map "is no playable map").
- N8 (float scan reach): the rule scan reads `crates/sim/src` and `crates/content/src` only (sim/tests/rules_scan.rs:1-7, :472). Float arithmetic written in `client-endpoint` (the ground crossing, the clamp, the rounding to cm) would sit outside it, though it decides a replayed, hashed value. Finding for step 2: that arithmetic lives in `sim` (beside `cast_ray`), the endpoint only calls it.
- N9 (replay refusals): the native runner refuses a recording whose move, ray or view words are non-zero (devtools/src/replay.rs:79-100); the TS reader likewise (packages/sim-worker/src/recording.ts:141-143); the replay calls `input` with six numbers (replay.rs:150, recording.ts:160, e2e/tests/replay/determinism.spec.ts:26-27, :84). The format (version 1, 68 bytes a step) already holds move, ray and view (recording.rs:1-11, :71-90): no format change in 4c.
- N10 (frozen content): first-loop.content.json (8,375 B) holds maps/test-scene.json and movement/soldier.json at movement version 1 with 15 fields (content/src/movement.rs:13-35); the reader refuses unknown and missing keys and there is no default (4B-FIRST-DEFAULT). Any field the movement kind gains (NO-SOURCE (a) or (b)) makes the frozen first-loop content unloadable, so first-loop must be frozen anew (`make_recordings --freeze first-loop`) in the same commit.
