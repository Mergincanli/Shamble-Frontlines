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
- N9 (replay refusals): the native runner refuses a recording whose move, ray or view words are non-zero (devtools/src/replay.rs:81-84 move, :85-91 aim, :92-98 view time; line numbers rechecked with grep -n); the TS reader likewise (packages/sim-worker/src/recording.ts:141-143); the replay calls `input` with six numbers (replay.rs:150, recording.ts:160, e2e/tests/replay/determinism.spec.ts:26-27, :84). The format (version 1, 68 bytes a step) already holds move, ray and view (recording.rs:1-11, :71-90): no format change in 4c.
- N10 (frozen content): first-loop.content.json (8,375 B) holds maps/test-scene.json and movement/soldier.json at movement version 1 with 15 fields (content/src/movement.rs:13-35); the reader refuses unknown and missing keys and there is no default (4B-FIRST-DEFAULT). Any field the movement kind gains (NO-SOURCE (a) or (b)) makes the frozen first-loop content unloadable, so first-loop must be frozen anew (`make_recordings --freeze first-loop`) in the same commit.
- N11 (manifests): room and client-endpoint depend on protocol and sim only (their Cargo.toml); sim on content and parry3d; sim-wasm on all five. 4B-WORLD-ROOM wants `content` kept out of room's and endpoint's manifests, so the bounds and the world reach them as sim types (`StaticWorld`), never as `MapDocument`. Their tests still need a world: either a dev-dependency on content (a manifest change, the dependency hold) or a world made through sim (which already depends on content). Engine-architect's call; step 2 needs it settled.
- N12 (constructors to change): `Room::new(setup, content_hash)` (room/lib.rs:105; callers room/lib.rs:314, :655, room/tests/any_bytes.rs:117, local_match.rs:66); `Endpoint::new(content_hash, first_sequence, pad)` (endpoint.rs:117; callers endpoint.rs:400, :620, client-endpoint/tests/any_bytes.rs:131, local_match.rs:67); `LocalMatch::new(.., world)` (local_match.rs:58; callers sim-wasm lib.rs:122, local_match.rs:213, :399, :413, sim-wasm/tests/step_alloc.rs:86); `LocalMatch::static_world()` read at sim-wasm lib.rs:406.
- N13 (initial aim): an endpoint that starts its aim at (0, 0, 0) cm would be refused by the room on a map whose bounds exclude the origin (the loader allows any bounds inside the reach, map.rs:316-335): the first aim must be clamped into the bounds. A planted unclamped start on such a map turns refused_aim above 0.
- N14 (CI browsers): Firefox already runs `tests/replay/**` (playwright.config.ts:38-40, his FIREFOX-REPLAY), so every new `.rec` is replayed natively, in Node, in Chrome and in Firefox by today's specs, which iterate the folder (determinism.spec.ts:69; replay.test.mjs:19): his FEATURE-COMPARE condition (DECISIONS.md:536, "native, Chrome and Firefox are compared on the real thing") needs no new spec, only a recording whose steps query the world. A paced replay that draws the page cannot run in CI's Firefox (FF-DRAW): Chromium only.
- N15 (CI time): the Chromium run uses 3.6 to 3.9 of its 5 minutes (4B-CHROMIUM-TIME); a paced replay is real time (lane-walk 1,200 ticks = 40 s a run; four runs = 160 s). It cannot fit before the CI Builder's split or new limit; a 300-tick slice four times is about 40 s and still near the limit.
- N16 (CI cargo): CI runs `cargo test --workspace` twice in debug and `cargo test --release` on six crates (toolchain.yml:158-162): a 2,000,000-command run belongs in the release line, with a short debug count.
- N17 (PM-0004-60's own count): the mark is 10 million fuzzed commands, nightly, through cargo-fuzz, every field over its range, NaN and infinities, and "claims on ids that do not exist" (ADR 0004:1181). The plan's 2,000,000 in CI is a partial reading not in DEPART-4's list; Walk's commands carry no float and no id (a slot is the connection's), so NaN can enter only through the post's ray, which the endpoint checks.
- N18 (pacer): the due time of the tick a wake runs is `#nextDueMs` before the step (pacing.ts:101-104); `afterTick(now)` gets only the start (:104), and the block message carries startAt and doneAt only (host.ts:184-194; messages.ts:56). The stamp is a pacer signature change, not new state.
- N19 (blend allowance, corrects my design 3.6): with the allowance at the 90th percentile of lateness, about 10 % of ticks hold for a few ms; by my estimate (2 frames in 9.6 affected at 144 Hz, about 29 % error each) std/mean comes to about 5.8 %, over the mark. The allowance must sit at the top of the window (the largest of the last 64 arrivals, or the 99th percentile of the last 300), held within [0, P/2]. The test pins whichever is built.
- N20 (two-block plants): with the render time t - P - d and only two blocks held, the needed pair is gone whenever the newest arrived early in its window (a window of d minus lateness each tick); the other two-block form (fraction paced from the newest's arrival) moves with every lateness difference. The test that must fail with two plants both forms, so it is not won against a straw man.
- N21 (SUPERSEDED by "N21 CORRECTED" below: its rule is the one rejected there as A) (pause in the blend): after a rebase the due times of two consecutive ticks lie the hidden time apart. Blending by due time slides or jumps; taking each held block's time from its tick against the newest's due (time(k) = due(newest) - (tick(newest) - tick(k)) x P) holds the paused picture until the render time reaches the new tick and then blends one tick: no slide across the pause and no jump, which also removes the 0.2 to 0.44 m jump my design gave RESUME-FLASH (a).
- N22 (pins step 2 breaks): `sim-wasm.test.mjs:82, :97-98` (input calls of six), `:253` (the glue text "input(post, held, p0, p1, p2, p3)" reaching one export; the prototype list `:246` gains no name); `host.test.ts:28-31, :98-99, :292` (six numbers), `:303` (one input per tick stays); `step_alloc.rs:89-100` (a `Post` literal of three fields); `world.rs:301` (22 and 54, only if the record grows).
- N23 (hide and the move): the host lets go of the keys at hide by `#held = 0` (host.ts:121-122). Once the post carries a move made from those keys, the host must zero the move there too, or the first tick after the return walks on with the old move until the page's first post.
- N24 (record width): if the World's record (world.rs:115, the whole `PlayerCommand`) takes the new move and aim, `SHARED_STATE_BYTES` goes from 22 to 30 and every first-loop hash moves in step 2 (inputs unchanged); if the record stays buttons and presses (what the echo and the press base need) and the move and aim are consumed by the soldier, step 2 moves no hash.

## Block 1. Step 2, the command path, against today's code
Plan row: "385 / 530". What it carries, page to room: the post gains the move and the cursor's ray; one input call of
13 plain numbers; the endpoint casts the ray against its own copy of the world; the room refuses a move longer than 1
and an aim outside the map; one jump per command (3B-ONE-PER-TICK).

### 1.1 Symbols to add, change and remove (file:line on dae6d107)
**Rust, `sim`**
- ADD `crates/sim/src/aim.rs` (new; `pub mod aim;` in `lib.rs:18-23`): `pub struct AimCm { x: i16, y: u16, z: i16 }`
  (the wire's integers; the sim turns them into metres itself); `pub const AIM_REACH_M: f32 = 400.0` (the old game's
  `raycaster.far=400`, shamble-game.js:1120: a code number of the old game, like its 2 cm facing rule, not a value of
  his Export); `pub fn aim_point(world: &StaticWorld, origin: V3, direction: V3, last: AimCm) -> (AimCm, AimFrom)` with
  `enum AimFrom { Hit, Ground, Kept }`: the first hit within 400 m (`cast_ray`), else the crossing of the ground plane
  y = 0 ahead of the origin, else `last`; then rounded to whole centimetres (`round`, on part 7's list) and clamped
  into the bounds in whole centimetres (integer compares). It lives in `sim`, not in the endpoint, so the rule scan
  (which reads only `sim` and `content`, N8) holds its float arithmetic: it decides a value that is replayed and,
  once the soldier faces it, hashed. Corrects my design 3.7 (fire-height plane, 500 m) to the old game's rule (N6).
- CHANGE `collision.rs:94-99`: `StaticWorld` gains `bounds_cm: [[i32; 3]; 2]`, taken in `build` (`:212`) from
  `map.bounds` (on the centimetre grid for every loaded map, map.rs:9-14); a document built in code whose bounds are
  off that grid is refused (`WorldError` gains `Bounds`, with its `message`). ADD `pub const fn bounds_cm(&self)`
  and `pub fn holds_cm(&self, aim: AimCm) -> bool`. Through this the room and the endpoint get the bounds as a `sim`
  type and never name `content` (4B-WORLD-ROOM, N11).
- CHANGE `world.rs:33-51`: `PlayerCommand` gains `move_x: i8, move_y: i8, aim: AimCm`; `NONE` gains zeros; the doc
  (":33-34, ...join when a system reads them") says the move and the aim are read by the movement and the facing, and
  the view time joins with lag compensation (change-set 8). **The record (`world.rs:115`) is a choice with a hash
  consequence (N24):** (i) recommended, the record stays buttons and presses (a `PlayerRecord`, what the echo
  `room/lib.rs:286-296` and the jump's press base need) and the soldier consumes move and aim: step 2 moves no hash;
  (ii) the record is the whole command: `SHARED_STATE_BYTES` 22 -> 30 (`world.rs:18`, the known answers `:301`
  and the second implementation's script), first-loop's `.hashes` and `.vector.txt` re-made in step 2 with its
  inputs byte-identical. Whatever the World keeps is hashed (ADR 0004 part 9); (i) keeps less for change-set 7's reset
  to carry. rust-reviewer's and engine-architect's call; I state only the hash side.

**Rust, `client-endpoint`**
- CHANGE `post.rs:17-24`: `Post` gains `move_x: i8, move_y: i8, ray: Option<Ray>`; ADD `pub struct Ray { origin: V3,
  direction: V3 }`; `NONE` (`:28-32`) gains zeros and `None`.
- REMOVE `post.rs:34-48` `from_words`; ADD `from_numbers(number, held, counts: [u32; 4], movement: u32, ray: [f32; 6])
  -> Self`, total (any 13 numbers): the move from the low 16 bits (x bits 0-7, y bits 8-15, as `i8`: the recording's
  own packing, recording.rs:81-82), then `clamp_move`; the ray `None` unless all six numbers are finite (PM-0004-24's
  "checked where it enters").
- ADD `post.rs` `pub fn clamp_move(x: i8, y: i8) -> (i8, i8)`: integers only; while x^2 + y^2 > 127^2 the longer
  component steps toward 0, **and on a tie both step** (the page rounds to nearest, so a keyboard diagonal arrives as
  (90, 90): 127 x 0.7071 = 89.8; 90^2 + 90^2 = 16,200 > 16,129, and both stepping gives (89, 89) with the direction
  kept), so (-128, 0) becomes (-127, 0) and the eight keyboard directions at 0 yaw ((+-127, 0),
  (0, +-127), (+-89, +-89)) stay as they are. (My design had it; 3b did not build it: post.rs holds none.)
- CHANGE `endpoint.rs:81-88`: `Sent` gains `aim: AimCm`, the aim cast when the command was built, so every repeat
  carries the aim it first carried. CHANGE `:90-110`: `Endpoint` gains `world: StaticWorld` (its own copy) and
  `aim: AimCm` (the last aim; at the start (0, 0, 0) clamped into the bounds, N13). CHANGE `:117` `new(content_hash,
  first_sequence, pad_bytes, world: StaticWorld)`. CHANGE `:226-263` `build_input`: the newest command's aim is
  `aim_point(&self.world, ray, self.aim)` when the post has a ray, else `self.aim`; the command takes `move_x/move_y`
  from the post and `aim` from its `Sent`; `view_time` stays `{view_tick, 0}` (change-set 8, the plan's choice 2).
  CHANGE `:56-79` `Readout` gains `aims_kept` (commands whose aim was kept: no usable ray, or none that met the world
  or the ground): read by tests; whether the block shows it is engine-architect's layout.
**Rust, `room`**
- CHANGE `lib.rs:45-68` `RoomCounters` gains `refused_move`, `refused_aim`; `refused()` (`:73-79`) adds both.
- CHANGE `lib.rs:86-100, :105` `Room` holds the authority's `StaticWorld` (4B-WORLD-ROOM: the movement queries it here
  from step 1 on; step 2 needs only its bounds); `new(setup, content_hash, world)`. ADD an accessor for it (replaces
  `LocalMatch::static_world`, N12).
- CHANGE `lib.rs:203-231` `on_datagram`: per command, before the queue (where `refused_ahead` is judged): a move with
  x^2 + y^2 > 127^2 (saturating operations: the crate denies plain arithmetic, `:8-16`) is dropped and counted
  `refused_move`; an aim the world's bounds do not hold, `refused_aim`. CHANGE `:216-223`: the `PlayerCommand` takes
  `move_x, move_y, aim`. CHANGE the doc `:176-182`: move and aim are checked; still not checked, each with what brings
  it: the aim's reach against a weapon (change-set 8: "clamped, not refused", ADR 0001 :234-236), the view time (lag
  compensation), the rest as written.
**Rust, `sim-wasm` and `devtools`**
- CHANGE `sim-wasm/src/lib.rs:161-169` `input`: six numbers -> 13 (`post, held, p0, p1, p2, p3, movement: u32, ox, oy,
  oz, dx, dy, dz: f32`), still one call per tick, plain numbers (WB-HEALTH, `:5-7`); its doc says what each is.
  CHANGE `:116-122` `create`: the map's world is built twice from the one document (the room's and the endpoint's:
  what an online client does with its own copy; risk 6 measures the two), or built once and cloned (`StaticWorld`
  derives Clone, collision.rs:94): engine-architect's call.
- CHANGE `local_match.rs:35-51, :58-84`: the `world` field leaves `LocalMatch` (to the room and the endpoint); REMOVE
  `static_world()` (`:96-100`), its one reader `sim-wasm lib.rs:406` reads the room's.
- REMOVE `devtools/src/replay.rs:81-91` the two refusals "move: the input call does not take it yet" (`:81-84`) and
  "aim: ..." (`:85-91`); KEEP `:76-80` (the map) and `:92-98` (view time); CHANGE the refusal test `:312-350` (its move
  and aim cases, `:337-341`, become recordings that replay). CHANGE `:150` the input call: 13 numbers, the ray's floats from the recorded bits
  (`f32::from_bits`).
**TypeScript**
- CHANGE `packages/sim-worker/src/messages.ts:25-42`: the input post gains `move` (one number: the packed word, at
  most 0xFFFF) and `ox, oy, oz, dx, dy, dz`; the doc's last sentence (`:30-31`) says the view time joins in change-set
  8. CHANGE `isPageToWorker` `:140-151`: `isCount(move) && move <= 0xffff`, and `Number.isFinite` for each ray number.
- CHANGE `host.ts:9-15` (`HostSim.input`, 13 parameters), `:57-64` (seven more fields of the newest post), `:81` (the
  call passes 13), `:103-118` (taken with the post, none rolled back by a stale one), `:121-122` (hide: `#move = 0`
  beside `#held = 0`, N23).
- CHANGE `packages/client/src/app/worker-link.ts:332-348` `frame(...)`: posts the move and the ray it is given (the
  page makes them in step 4: the move from the keys and the camera's yaw, the ray from the camera's stable base;
  engine-architect's).
- CHANGE `packages/sim-worker/src/recording.ts:42-46` (`Replayable.input`, 13), REMOVE `:141-142` (move and aim
  refusals), KEEP `:143`, CHANGE `:159-161` (`replayStep` passes the move and the six floats from their bits, through
  one shared `Uint32Array`/`Float32Array` pair: no allocation per step); `packages/e2e/tests/replay/determinism.spec.ts:
  18-31, :84` (the worker's call and the step's tuple).
- **Flag (two designs differ):** engine-architect's design 3.6 posts `move_x, move_z` as two numbers and makes the
  call 14 (six `u32`, eight `f32`); the plan and the recording's format say 13 with the move packed in one word. The
  main session settles it in the sheet; I recommend 13 (the format holds one word, the clamp is integer, and the page
  then sends what the room judges).

### 1.2 Tests, each with its planted defect (bottom-up lines; Rust then TypeScript)
| # | Where | What it holds | Planted defect that must turn it red |
|---|---|---|---|
| T2-1 | `post.rs` | `clamp_move` over all 65,536 pairs: result within 127, unchanged if inside, each part only toward 0, sign kept; the eight directions unchanged; the ties pinned: (90, 90) -> (89, 89), (-90, 90) -> (-89, 89) (25) | `<` for `<=` in the length test, so (127, 0) moves |
| T2-2 | `post.rs` | `from_numbers` takes any numbers; a NaN, an infinity in any one of the six ray numbers gives no ray; spare move and held bits ignored (30) | the finite check on five of the six numbers |
| T2-3 | `sim/src/aim.rs` | a ray onto the step's top gives its point in cm; one that misses gives the ground crossing; one to the sky, one parallel to the ground, keep the last; a hit at 401 m is no hit; a point beyond the bounds is clamped to the bound's cm; the rounding of a half centimetre pinned (70) | the fallback plane at fire height (my old design); the reach 500 m; no clamp |
| T2-4 | `collision.rs` | `bounds_cm` exact at the reach's edges (+-32,767; 0 to 65,535); `holds_cm` on each face true, 1 cm past false; bounds off the grid refused by name (30) | the bounds narrowed through `f32` before the cm (an off-by-one at a face) |
| T2-5 | `endpoint.rs` | PM-0002-14's native half: each command's aim is the cast of the post handed in for that tick (posts alternate rays onto two shapes); repeats carry their own first aim; the first aim lies inside bounds that exclude the origin; a post without a usable ray keeps the aim and counts `aims_kept`; a (-128, -128) post sends a move the room takes (90) | the previous post's ray cast; repeats cast again from the newest post; the first aim (0, 0, 0) unclamped |
| T2-6 | `room/lib.rs` | moves (127, 0), (0, -127), (89, 89), (-89, -89) taken; (90, 90), (-128, 0), (127, 127) refused and counted; an aim on each of the six faces taken, 1 cm past each refused; a refused command never runs, and its sequence counts as missing when the next one runs (70) | `<` for `<=`; the aim held to the `i16` range instead of the bounds; `refused_aim` left out of `refused()` |
| T2-7 | `room/tests/any_bytes.rs:139-143` | the generator's move and aim over their full ranges (the aim is `{1, 2, 3}` today); its identities still hold (15) | as T2-6's third plant: the identity breaks |
| T2-8 | `client-endpoint/tests/any_bytes.rs` | random posts with hostile rays (NaN, infinities, beyond 2^20, zero direction): no panic, every aim sent inside the bounds (25) | the clamp skipped on the ground fallback |
| T2-9 | `sim-wasm/src/lib.rs` tests | the 13 numbers reach the endpoint and the room: a ray onto a shape is the aim of the room's next decoded command; `:240-259`'s echo test on 13 numbers (30) | the input call dropping the ray |
| T2-10 | `sim-wasm/tests/step_alloc.rs` | rays every tick (hits, sky, NaN, beyond the bounds): 0 allocations over 18,000 ticks (15) | a `Vec` made in `aim_point` |
| J1-J5 | the movement's tests (step 1's code) | 3B-ONE-PER-TICK as my design 4.1: 255 presses in one grounded command make one jump; a press in the air nothing, and none kept for the landing; Space held jumps at each landing; two commands in a tick, each +1, one jump; a tap shorter than a tick one jump (80) | a jump per press; presses kept for the landing; the held bit read alone |
| T2-11 | `messages.test.ts` | posts with the seven numbers taken; each ray number NaN, Infinity or a string refused; a move above 0xFFFF or negative refused (25) | `Number.isFinite` left off one number |
| T2-12 | `host.test.ts` | the newest post's 13 numbers reach `input`; a stale post rolls none back; hide zeroes held and move (30) | the move kept at hide: the first tick after the return carries it |
| T2-13 | `worker-link.test.ts` | `frame` posts the move and ray it is given (15) | the ray's direction posted as its origin |
| T2-14 | `recording.test.ts`, `replay.test.mjs` | a recording with a move and a ray now replays; a view time is still refused by name; the TypeScript and the native refusal lists name the same fields (20) | one list keeping the move refusal |
| T2-15 | `sim-wasm.test.mjs:82, :97-98, :253` | the glue's 13-parameter `input` reaches one export; any 13 numbers taken, NaN rays without a trap (10) | a second export call in the glue |
| T2-16 | a Chromium spec beside `presses.spec.ts` (PM-0002-14's page half) | 10,000 frames with the mouse moving: the match never fails (a post without finite ray numbers ends it at the guard), the page's posts reach 10,000, and the aim the block's own section shows follows the cursor (step 3's section) (50) | a separate `{type: "aim"}` message (the guard refuses it: the match fails) |

### 1.3 The count, recounted (bottom-up, unmultiplied)
- Code: `sim` aim.rs 45, bounds 20, `PlayerCommand` and record 15; `post.rs` 45; `endpoint.rs` 35; `room` 30;
  `sim-wasm` 15, `local_match` 15; `replay.rs` 10; the jump rule 10 (built in step 1's movement): Rust 240. TS:
  messages.ts 15, host.ts 25, worker-link.ts 15, recording.ts 15, the e2e worker 5: 75. **Code 315 (+-30 %).**
- Tests: Rust T2-1 to T2-10 400, J1-J5 80: 480. TS T2-11 to T2-16 150. **Tests 630.**
- **945 against the plan's 915 (385 / 530).** Less code than my design (the page's move from the keys and the ray
  from the camera are step 4's, engine-architect's), more tests (the aim function's own and PM-0002-14's page half).
  If T2-16 moves to step 7 (the browser tests), step 2 is 315 / 580. The J row of my design (15 / 110) carried
  RESUME-FLASH (a)'s guards; they are built in the blend (step 3) and the camera (step 4), and counted there.

### 1.4 What changed since my design (2026-11-25, on 311c0484)
1. 15 numbers -> 13: the view time leaves the call until change-set 8 (the plan's choice 2). So **PM-0002-14 is met in
   its ray half only** ("10,000 of 10,000 posts carry the cursor ray and the view time"): a reading DEPART-4 does not
   list yet; the report must say "half".
2. The static world exists (4b-2) and is held by `LocalMatch`; the endpoint's copy and the room's are 4c's
   (4B-WORLD-ROOM). `cast_ray` exists, with RAY_REACH and its NaN guard (RR-1), and returns `t`, not a point.
3. The loader holds every map inside the aim's reach on the cm grid (N7): the room's aim check is integer compares
   against the bounds, and the client's clamp is integer after the rounding.
4. The old game's aim falls back to the ground plane at 400 m (N6), not the fire-height plane at 500 m.
5. The float scan reads `sim` and `content` only (N8): the aim's arithmetic moves into `sim`.
6. The host must zero the move at hide (N23): new, because the move did not exist in 3b.
7. The first aim must start inside the bounds (N13): new, because 4b allowed bounds that exclude the origin.
8. The replay readers refuse move and aim today (N9): step 2 removes those two refusals, natively and in TypeScript.

### 1.5 His open questions, by option, for step 2
- **MOVE-RULES:** nothing, under (a), (b) or (c): no row touches the move vector, the ray, the room's two checks or
  the jump's one-per-command rule. Row 8 under (a) reads the same bounds for the map's edge (rust-reviewer's); under
  (c) the old edge (the ground less 1 m) does not touch the aim check.
- **NO-SOURCE:** nothing. The aim's 400 m reach is a code number of the old game, like the 2 cm facing rule the plan
  keeps as code (plan section 10); if the main session counts it as a value, it joins NO-SOURCE's list.
- **RESUME-FLASH:** (a), (b), (d): the host lets go of held and move at hide (3b's rule). (c): the host no longer lets
  go at hide (`host.ts:121-122` loses both zeroings) and T2-12's case reverses.
- **TUNE-HOME:** nothing: no tuned value in the command path.
- **SIZE-4C:** (a) and (b): step 2 sits in the soldier's pull request. (c): its Rust half, the host, the guard and
  the recording readers are page-free and go in the first half (unit, Node and Chromium replay tests need no drawn
  page); T2-16 (PM-0002-14's page half) needs the page's post of the ray, so it goes in the second half with step 4.
- N21 CORRECTED (worked by hand, the frame on which the first block after a pause arrives): tick n drawn whole before the return; resume at R, so the next due is R + P and the block arrives at R + P + late; the render time at that frame is r = R + late - d. (A) Every held block re-timed from the newest's due by ticks: n sits at R and n-1 at R - P, so r falls in (n-1, n) at fraction (P + late - d) / P < 1: **a step backwards** of up to d / P (half a tick at most). Rejected. (B) The pair chosen by the stamped due times, the fraction measured back from the newer block's own due, clamped: f = clamp(1 - (due(n+1) - r) / P, 0, 1) = clamp((late - d) / P) = 0, so tick n exactly, the same as the frame before; it starts to blend when r passes R and reaches n+1 one period later. Normal ticks (due times P apart) give the usual fraction. A gap below P cannot occur (a rebase sets the next due one period after a time at or past the old due). Rule (B) is the design; its test asserts the drawn position never goes back.
- N19 ADDED: the 5.8 % is my arithmetic, not a measurement; his lateness is two numbers (median 1.967 ms, p99 14.07 ms) from one Brave run (docs/reports/2026-11-25-cs3b-browser-check.md). The test's arrival distribution is an assumption, named in the test.
- N25 (versions): `Object::header` takes any version from 1 to the newest (content/src/read.rs:272-282), but each kind has one field list (movement.rs:17-35, :58-80), and there is no default (4B-FIRST-DEFAULT). So a field the movement kind gains refuses every older frozen movement file unless the reader keeps the old list and a value for the new field, which is a default and goes to the advisor agent first. A new kind (step 5's camera folder) adds a folder to `FOLDERS` (bundle.rs:31) and no rule asks a bundle to hold every kind, so a frozen bundle without a camera file still loads: the camera forces no fresh freeze.
- N26 (DEPART-4): his answer (a), "all fourteen stand, with two notes" (DECISIONS.md:699), covers the three newest blocks and the blend time on the block's message. None of MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME, SIZE-4C has an answer in the register (grep: none).
- N27 (PM-0004-20's flag-off half): the flag is pinned in the root manifest's text (devtools/tests/dependency_rules.rs:124-125, his FEATURE-COMPARE (a)) and in the build's feature lists (devtools/vectors/sim-feature-lists.json:52, :125, :198): no build in the repository can have it off. The timing with the flag off is not runnable in 4c's build; it could only be a throwaway reading on a copy under `.gate/`, or recorded as not run. The hash half (0 mismatches native against WebAssembly) is met on the `walk` recording, not on SS's 18,000 ticks at N = 48 (Slice 2's stress scene): a reading to state.
- N28 (test scene, from content/maps/test-scene.json): bounds x -32..80, y 0..20, z -24..24; eight spawns (four in the street at x -20 and -22, four in the yard at x 25 to 27); no ground shape (the floor is the bounds' bottom: rust-reviewer's F11 too); 23 shapes: two houses (boxes to 2.6 m, roof ramps to 4.7 and 4.3 m), the 1.0 m cover wall, the 1.5 m alley between the houses, steps 0.55 and 0.56, ledges 2.90, 3.05, 3.10, a 0.2 m thin wall (x 30 to 30.2), a gentle ramp (1 m over 4 m, 14 degrees) and a steep one (1.5 m over 3 m, 26.6 degrees) meeting platforms at a shared face, six lane posts every 10 m at z 20 to 20.2 from x 25 to 75.2 (the lane spawn at z 18, 1.6 m from the posts' faces).
- N29 (one slot): a local match has one slot until change-set 5, so no recording can hold two soldiers: the soft push (START-RULES, built and proven with two soldiers in 4c, plan 6.1) runs in no recording. Its identity across targets rests on sim-level tests unless they also run in WebAssembly through the test module (as `static_world` does). Flag for rust-reviewer's marks.
- N30 (overlap): rust-reviewer's read (F11) also needs the bounds in the movement (the floor and the four sides as solid; MOVE-RULES row 8): one home for the bounds, on `StaticWorld`, in metres for the movement and in cm for the aim, built once.

## Block 2. Step 3, the snapshot's soldier and the block (engine-architect owns the layout; I own the wire and the timing)
Plan row: "320 / 340": the header's soldier count, 48 soldier rows of 40 bytes, the owner's section; each block
stamped with the time its tick was due; the page blends between ticks; the Worker's pool grows (3B-POOL-16).

### 2.1 Symbols (file:line on dae6d107)
**The wire: the snapshot's own soldier (mine; ADR 0001 part 4 :203-209, "sent unquantised")**
- ADD `protocol/src/messages.rs` `pub struct OwnSoldier`: every field of the soldier's movement state as step 1 makes
  it (the rule of my design 3.4, so change-set 7's reset starts from exactly the server's state): the generation
  (`u16`), the team (`u8`), the flags (`u8`: grounded, sprinting, the sprint lock; any other bit a `Flags` error),
  position and velocity (3 + 3 `u32`, the `f32`s' bits, so `protocol` stays integer-only), the facing as step 1 holds
  it, stamina and the time since the sprint ended (counts of the time unit). **No `placement` counter** (the plan's
  choice 1: a new generation at a row means place). The exact list is step 1's state, field for field (2.4).
- CHANGE `messages.rs:168-187` `Snapshot` gains `soldier: Option<OwnSoldier>`; `OwnState` (`:156-166`, the echo) stays.
- CHANGE `codec.rs:7-18` (the table), `:34-37` ADD `FLAG_SOLDIER: u8 = 16` (bit 3 `FLAG_OWN` stays as built),
  `:43` `MAX_SNAPSHOT_BYTES` grows by the record (33 + about 47), `:326-360` `encode_snapshot`, `:364-405`
  `decode_snapshot`: a float whose bits are not finite refuses the whole snapshot (`DecodeError` gains `NotFinite`: a
  hostile room must not put NaN into the client); an unknown soldier flag bit is a `Flags` error.
- CHANGE `protocol/tests/decoder_inputs.rs`: the generator gains the soldier part; 10 million per type kept.
- The TypeScript form of `Snapshot` (`ts_export`, `SHAMBLE_WRITE_TYPES`) is regenerated.
- CHANGE `room/src/lib.rs:286-300` `step`: the snapshot's `soldier` from the world's soldier in the player's slot
  (step 1's accessor; rust-reviewer's F12 lists this site).
- CHANGE `client-endpoint/src/endpoint.rs:291-339` `on_datagram`: takes the decoded soldier into what the block is
  written from (a refused snapshot keeps the last row; `refused` counts it as today); `:175-178` `write_block` passes
  it and the join's slot and team to the writer (engine-architect's rows and owner section).
**The block (engine-architect's: listed so the sheet has the whole row, not designed here)**
- `layout.rs:6-33` `ElementType` gains `F32` (and `U16` if the row takes one); `:46-55` `Section` gains `rows`; `:111-
  119` `end_of` becomes rows times stride; `:84-103` `BLOCK_LAYOUT`: the header's `soldier_count`, the soldier section
  (48 rows of 40 bytes: the row is slot i), the owner's section (the local soldier's id, team, the aim the endpoint
  sent); `block.rs:22-79` the writer; `layout_ts.rs`; `packages/types` regenerated; `worker-link.ts:5-16` offsets.
  **The header's `soldier_count`** is written by the block writer from what the endpoint holds: 0 until the first
  snapshot that carries the soldier (so before the join is answered and before the first snapshot), then 1 in 4c (one
  slot, and the snapshot carries only the owner's record); from change-set 5, the slots the snapshot's records fill.
  The yaw and pitch for the row: computed from the soldier's facing and aim where engine-architect puts it (Rust with
  the pinned libm's arc tangent, which joins the float vectors, plan 5.3; or the page's `Math.atan2`, allowed because
  they are no simulation values). In the client endpoint, a direct `libm` line would be a manifest change.
**The timing (mine)**
- CHANGE `packages/sim-worker/src/pacing.ts:96-107` `wake(afterTick: (startedAtMs: number, dueMs: number) => void)`:
  the due time of the tick about to run (`#nextDueMs`, read before `step` at `:103`) is handed to `afterTick` (N18).
- CHANGE `host.ts:184-194` `#afterTick(startedAtMs, dueMs)`: the block message gains `dueAt: origin + dueMs`, on the
  shared clock beside `startAt` and `doneAt`. This is the DEPART-4 entry he accepted ("the time the page blends by
  travels on the block's message, not as a field inside the block", DECISIONS.md:699).
- CHANGE `host.ts:33-37` `POOL_SIZE` 4 -> 16 (3B-POOL-16, STATUS_ARCHIVE.md:1711: "the Worker's buffer pool from 4 to
  16"), its comment: three blocks held by the page, one on its way, one being filled, and returns that come in lumps
  (perf-auditor's round 2: lumps of 12 to 17 at about 6 frames a second under key input). 16 x 1,988 = 31,808 bytes.
- CHANGE `messages.ts:52-56` the block message gains `readonly dueAt: number`; `isWorkerToPage` (`:180-182`) takes it
  only when finite and `dueAt <= startAt <= doneAt` (true by construction: the pacer runs a tick only once its due
  time has passed, `pacing.ts:101`, and adding the same origin keeps the order).
- CHANGE `packages/client/src/app/worker-link.ts:62-63, :99-105, :277-320`: the link keeps the **three** newest blocks,
  each with its `dueAt` and its arrival time on the shared clock, and sends the fourth-newest back with the next frame
  (today two, `:287-292`); `#takeBlock(buffer, startAt, doneAt, dueAt)`; it keeps a ring of (arrival - due) for the
  allowance.
- ADD `packages/client/src/app/blend.ts` (lane 1): the render time r = frame time - P - d, with P = 1000 / tick rate
  and d the allowance: **it rises at once to the largest (arrival - due) of the last 64 blocks and falls gradually
  (a slew of about 1 ms per second), held within [0, P / 2]; the estimator is pinned by B1** (corrects my design's
  90th percentile, N19, and a fall at once: when the window's largest ages out, d would drop by up to 8 ms in one
  frame, a forward step of more than one 144 Hz frame's motion every couple of seconds, which by the same arithmetic
  sits near the 5 % mark, not clear of it; a rise is continuous, because it happens only when a block arrives later
  than d, while the page holds the newest, and the new d puts the render time exactly at that held block); the pair: the first held block whose stamped due lies after r, and the one before
  it; **the fraction measured back from the newer block's own due: f = clamp(1 - (due(newer) - r) / P, 0, 1)** (N21's
  rule B: after a pause or a dropped gap it holds the paused tick and blends one period, never back, never a slide
  over the hidden time); no held block after r: the newest whole (never extrapolate). Placing (never sliding) a row
  whose id changed between the pair is the picture's, by the ids in the two blocks (plan 6.2).
- `packages/client/src/picture.ts:18-24` and `PictureFrame` (lane 2's interface file; engine-architect's): the frame
  carries the pair and the fraction; the sentence "it comes as a field of the block" becomes "on the block's message"
  (DEPART-4). The view time the post reports from change-set 8 is (the pair's older tick, the fraction).

### 2.2 Tests, each with its planted defect
| # | Where | What it holds | Planted defect that must turn it red |
|---|---|---|---|
| S1 | `protocol` codec tests | the soldier record round-trips with every flag combination; NaN, an infinity or a signalling NaN in any float refuses the snapshot; an unknown flag bit refuses it; the longest snapshot is `MAX_SNAPSHOT_BYTES` (40) | the finite check on all floats but one |
| S2 | `decoder_inputs.rs` | the generator's soldier part; 10 million per type, no panic, a decode re-encodes to the same bytes (15) | a soldier part that encodes `-0.0` and decodes `0.0` (two wire forms) |
| S3 | `endpoint.rs` tests | row 0 of the block is the decoded soldier; a refused snapshot leaves the row as it was and counts `refused`; the header's count is 0 before the first snapshot that carries the soldier and 1 after it (30) | the row written before the finite check |
| S4 | a native test in `sim-wasm` (PM-0001-17's first piece, my design 3.4) | over a replay of `walk`, the endpoint's decoded soldier serialised through the state's visitor equals the room's soldier's bytes, every tick (30) | the endpoint narrowing a float through `f64` and back with a rounding step |
| B1 | `blend.test.ts` (PM-0002-26's smoothness clause, the blend alone) | frames at exactly 60, 120 and 144 Hz over 18,000 ticks; blocks due every P; arrivals = due + lateness drawn from a seeded distribution **assumed** through his two numbers (median 1.967 ms, p99 14.07 ms, one Brave run) plus the report's copy and delivery; a mover at 6.3 m/s: std / mean of the drawn step per frame at most 5 % with three blocks, at each rate; the fraction always in [0, 1], r never past the newest's due (70) | (P1) the same rule with only the two newest held; (P2) two held, the fraction paced from the newest's arrival (N20: the test is not won against one weak form); (P3) the newest drawn without blending; (P4) the fraction not clamped. P1 to P3 must exceed 5 % at 144 Hz; P4 must break the [0, 1] check. The 90th-percentile allowance is printed as a reading, not asserted (my 5.8 % is arithmetic) |
| B2 | `blend.test.ts` | a pause of 60 s and one of 6 minutes between ticks 100 and 101 (due times by the rebase), a dropped gap of 9 ticks, and a block later than d: on a mover going one way the drawn position never goes back; the frames after the return draw tick 100 exactly until r passes due(101) - P; no frame steps more than its own motion plus one late block's catch-up; and the allowance falling (an outlier of 14 ms leaving the window of 64): no frame steps more than its own motion (60) | rule A of N21 (every block re-timed from the newest by ticks): a step back; blending by (r - due(older)) / (due(newer) - due(older)): a jump of nearly a tick at the return; the allowance falling at once: a forward step of up to 8 ms of motion in one frame |
| B3 | `host.test.ts`, `pacing.test.ts` | each block's `dueAt` is its own tick's due time on the shared clock (the first after a rebase is the rebase's time plus P; in a catch-up wake each tick its own, not the wake's start); `dueAt <= startAt <= doneAt` (25) | the start stamped as the due |
| B4 | `messages.test.ts` | a block message without `dueAt`, with a non-finite one, or with `dueAt > startAt`, refused (15) | the order check left out |
| B5 | `worker-link.test.ts` (replaces `:67-73`, "keeps the two newest blocks") | the three newest kept with their due times; the fourth sent back with the next frame's post; none sent back twice (20) | two kept |
| B6 | `block-pool.test.ts`, `host.test.ts` | the pool keeps 16 and lets the 17th go; a scripted page returning buffers in lumps of 12 at 6 frames a second: 0 misses (20) | the old size 4 (misses above 0) |

### 2.3 The count (bottom-up, unmultiplied)
- Mine: code: `protocol` 60, `room` 10, `endpoint` 25 (Rust 95); `pacing.ts` 5, `host.ts` 10, `messages.ts` 10,
  `worker-link.ts` 35, `blend.ts` 60 (TS 120): **215**. Tests: S1 to S4 115; B1 to B6 210: **325**.
- engine-architect's layout and writer (layout.rs, block.rs, layout_ts, types, picture.ts): by its own design about
  105 code and 25 tests in this row (its block 3); its read gives today's figure.
- **About 320 / 340 together, the plan's row.** Not counted: the regenerated TypeScript.

### 2.4 What changed since my design
1. The row is engine-architect's 36 bytes plus the vertical velocity and `ready` (40), and there is no placement
   counter (the plan's choices 1, 6.2): my `placement` field and flag bit leave the snapshot and the block.
2. The allowance moves from the 90th percentile of 64 to a rule that rises at once to the largest of 64 and falls
   gradually, its estimator pinned by B1 (N19; the 5 % figures are arithmetic, not measurements), and the fraction is
   measured back from the newer block's due (N21 rule B), which removes the jump my design gave the first block
   after a pause.
3. The pool goes to 16 (3B-POOL-16's own number), not the 6 of my design.
4. DEPART-4 (a) accepted three blocks and the due time on the message (N26): no question left there.
5. `PictureFrame` is still two blocks and a time (picture.ts:24-30): its change is engine-architect's, lane 2's file.

### 2.5 His open questions, by option, for step 3
- **MOVE-RULES:** the block's row: nothing under any option (its fields are lane 2's list and the plan's 6.2). The
  snapshot's `OwnSoldier`: its field list is step 1's soldier state under whatever option he picks; a row whose rule
  keeps state between ticks (for example a snap to a slope that remembers the ground under him, row 4 under (a))
  adds a field, and (c) may keep fewer. So the record's list is fixed only when step 1's state is.
- **NO-SOURCE:** nothing (it brings numbers, not state).
- **RESUME-FLASH:** (a) and (b): the first picture after a pause is rule B's (2.1, B2): the paused tick, then one
  period's blend; (b)'s dark key diagram is the page's overlay (block 5), not the blend. (c): nothing here. (d):
  nothing here (the soldier stops in the simulation; the blend draws what arrives).
- **TUNE-HOME:** nothing.
- **SIZE-4C:** (a) and (b): all in the soldier's pull request. (c): the wire's soldier record, the room's fill, the
  endpoint's decode and the block's Rust layout and writer are page-free (native tests; the page keeps reading its
  named offsets, and its length check uses the ready message's `blockBytes`, worker-link.ts:283) and can go in the
  first half; the stamp, the three blocks, `blend.ts`, the pool and B1 to B6 are Worker and page code and stay with
  the page in the second half, with the smoothness reading on his displays.

## Block 3. Step 6, the movement recording and the lane recording
Plan row: "320 / 230": the movement recording (about 2,400 ticks on the street corner and the yard) and the lane
recording (the 50 m lane, for the smoothness readings), every mark that rides them, the three planted divergences
run again on movement, the first recording re-made.

### 3.1 What each records, and on which frozen content
Each recording is a script's output (scripts.rs:1-4), replayed through the players' entry points, on a frozen copy of
the content beside it (`<name>.content.json`, written only by `make_recordings --freeze <name>`, files.rs:93-110,
bin/make_recordings.rs:4-6), so a value he tunes later never moves an old recording's hashes (plan 6.2).
- **`first-loop`, made anew (1,800 steps).** Its inputs stay byte for byte (`looped`, scripts.rs:78-141; `--check`
  shows the steps unchanged): move 0 and ray 0 in every step, since it was made before both existed. After step 1 the
  soldier stands at the local match's spawn, jumps (Space held at steps 600 to 699: a jump at each landing; taps at
  uneven gaps, two in one tick and bursts of six: one jump per command on a recording, 3B-ONE-PER-TICK), and holds
  Shift with no move (the stamina rule); no wall is met. Its aim: the zero ray has no direction, so the endpoint keeps
  its first aim (step 2's clamped start) and the soldier turns toward it once. **Its hashes move from tick 1 in the
  commit of step 1** (the soldier table enters the state), and in step 2's commit already if the record takes the
  move and aim (N24, choice ii): the recordings test (`tests/recordings.rs:29`) fails on any change of the state's
  bytes otherwise (rust-reviewer's F12 says the same). **Its frozen content is frozen anew** in the commit where the
  movement kind's (or the map kind's) field list changes (N25), which changes its header's content hash and nothing
  of its steps.
- **`walk`, new (about 2,400 steps)**, seed `TESTSEEDwalk`, on `walk.content.json` (today's content frozen in step 6,
  after the movement kind's fields are final). From the spawn, through the street corner and the yard
  (content/maps/test-scene.json, N28): the eight directions and moves at other angles (the camera's yaw is already in
  the recorded move word), and one (-128, -128) the endpoint clamps; sprint until the stamina locks; the 1.5 m alley;
  into a house wall (the slide); the cover wall (1.0 m) and a roof edge (2.6 m) by a jump (row 5); the 0.55 m step
  gained and the 0.56 m stopped; the ledges 2.90 and 3.05 gained and 3.10 not (the step-up in the air at the arc's
  top); the 0.2 m thin wall at a sprint (tunnelling); the gentle (14 degrees) and steep (26.6 degrees) ramps up and
  down, across the seam with their platforms; jumps by tap, by hold, by one command whose count moves by 255, and a
  press in the air; a ray every step onto the ground, walls, box tops, ramp faces, the sky and parallel to the
  ground, and a few hostile posts (a NaN, an infinity, a number beyond 2^20, a zero direction: the aim kept); repeated
  and skipped post numbers; jittered host times. No sights stretch (4d makes `walk` anew with them, plan 7, step 1).
- **`lane-walk`, new (about 1,200 steps)**, on `lane-walk.content.json` (frozen with `walk`): from the spawn to the
  lane (z 18, beside the posts at z 20), then walking at constant speed up and down the 50 m lane (x 25 to 75), one
  straight move at a time, the aim held on the ground ahead: the paced replay's smoothness readings (6b; 4d's frame
  cap readings).
- Not in any recording: two soldiers (one slot until change-set 5): the soft push's cross-target identity needs its
  own run in WebAssembly (N29, rust-reviewer's marks).

### 3.2 Symbols
- ADD `crates/devtools/src/scripts.rs`: `walk(content) -> Recording`, `lane_walk(content) -> Recording`, and `roam(name,
  steps, salt)` (a procedural movement match for the two-process test's 36,000 ticks, never committed); `RECORDINGS`
  (`:154`) from 1 entry to 3: `("walk", "test-scene", walk)`, `("lane-walk", "test-scene", lane_walk)`; each gets its
  `.vector.txt` at `VECTOR_TICK` 900 (`:157`). The rays are written as `f32` bits from points the script aims at
  (inputs, not simulation values, so the script's arithmetic is free).
- CHANGE `devtools/src/replay.rs:181-201` `impl StateSink for Lines`: gains `f32` (hex bits and decimal) once step 1
  gives `StateSink` its float method (rust-reviewer's F12). ADD a finite read in the runner: after every step it
  visits the state and fails naming the tick and the field of the first value that is not finite (PM-0004-24's "read
  by the replay runner itself on the release builds"); `replay()` and the two-process children report it.
- CHANGE `devtools/tests/two_instances.rs:15-52`: the two matches stepped in turn are `walk` and `first-loop`.
- CHANGE `devtools/tests/two_processes.rs:29-55`: each child replays every recording (automatic through
  `RECORDINGS`) and `roam` for 36,000 ticks in place of the loop.
- CHANGE `packages/sim-worker/boundary/replay.test.mjs:76-100` (W2): the interleaved pair is `walk` and `first-loop`.
  `replay.test.mjs:19` and `determinism.spec.ts:69` already iterate every `.rec`: `walk` and `lane-walk` are replayed
  in Node, in Chrome and in Firefox with no new spec (N14).
- ADD (a test) `devtools/tests/walk_reaches.rs`: replays `walk` natively and reads the state at the end of each
  stretch: on the 0.55 m step's top; short of the 0.56 m step; on the 3.05 m ledge; not on the 3.10 m one; never past
  the thin wall; the aim on a box top at its height in cm; and so on. So the recording provably queries the world.
- **4B-RAY-WASM** (mine, round 2 of 4b-2): CHANGE `devtools/src/world_proofs.rs` `made()` (`:437-485`): three `path`
  lines, one committed ray of each of RR-1's three paths (a number just beyond `RAY_REACH`: no hit; a limit beyond it,
  cut to it; a ramp whose test would overflow), each with its numbers by bits and its answer; `vectors/static-world.txt`
  made anew; the line counts in `static-world.test.mjs:20-21` (23 -> 26) and `tests/rules/static-world.spec.ts`. The
  whole text is already compared natively, in Node (both profiles) and in Chromium and Firefox (the rules folder runs
  in Firefox), so no new path is needed. Which three rays, and 4B-RAY-COUNTS, are rust-reviewer's (it wrote RR-1;
  the checkpoint card gives it this item too: overlap flagged).
- The plants (notes, as plants-4b2): the three injections armed at T = 600 on `walk`, the field diff through the
  `replay` bin's `state` natively and `replay_state_at` (devtools/src/wasm.rs:57-67) in WebAssembly.

### 3.3 The marks, each with its test and its planted defect
| Mark | Test, where it runs | Planted defect |
|---|---|---|
| PM-0004-17 (nothing outside the state) | `two_instances.rs` (natively, both columns) and W2 (Node, one module instance, shared column): `walk` and `first-loop` stepped in turn, each equal to its own file. Their seeds differ, and so do their inputs, which is what moves the shared column until change-set 8 draws a number (plan section 9's reading) | a `static` counter bumped per movement step and added into a soldier's float: the interleaved run differs from the solo one |
| PM-0004-19 (native and WebAssembly agree tick by tick) | `recordings.rs` (every recording, both columns), `two_processes.rs` (two children whose std hash-map seeds differ), W1 `replay.test.mjs` (Node, the players' file), C1 `determinism.spec.ts` (Chrome and Firefox, the players' file): on `first-loop`, `walk`, `lane-walk` | the three injections below; one hex digit of one hashes line; one byte of a `.rec` |
| PM-0004-20 (the library's flag across targets) | its hash half: `walk`, whose steps query the world, through the same runs: 0 mismatches natively against Node, Chrome and Firefox. Reading: over `walk`'s ticks with one soldier, not SS's 18,000 at N = 48 (Slice 2's stress scene). Its flag-off timing is not runnable (N27): said in the report | the bite of PM-0004-21 (i) |
| PM-0004-21 (caught at its tick, with the field) | three injections at T = 600 on movement fields of `walk`: (i) std `sin(K)` added to a soldier's position, K where native and WebAssembly differ (else a 1-ulp change on WebAssembly stands in, and the report says so): native against Node differ at T, and the diff names the field; (ii) a std `HashMap` of 32 keys folded into a soldier field in its iteration order: the two children differ at T; (iii) a draw from outside inside the simulation (a JavaScript `Math.random` import in WebAssembly, a per-process `RandomState` natively): two runs differ at T. Control: a draw outside the simulation (the page's generator) moves no hash. Word size: each recording's vector at tick 900, soldier bytes included, natively and in Node | each injection is the plant: 3 of 3 reported at T with the field; a planted `usize` field differs at tick 1 |
| PM-0004-24 (NaN never enters) | the runner's finite read after every step of N1 to N3 (CI's release line runs them too); the simulation's finite assertion each tick in debug and test builds (step 1's); the ray checked where it enters (step 2's T2-2); 6c's hostile runs | `0.0 / 0.0` in the movement phase at tick T: the debug assertion trips at T; the release runner names T and the field |
| ADR 0001 :1785 (build plan :485) and the recordings of :484 | `two_processes.rs` on `walk`; `walk` and `lane-walk` are 4c's recordings, 4d makes them anew | as PM-0004-19 |
| **4B-QUERY-RECORDING**, his FEATURE-COMPARE condition (DECISIONS.md:536) | `walk`'s steps run the movement's sweeps in the room's step and the endpoint's cast on every ray: compared natively, in Node, in Chrome and in Firefox; `walk_reaches.rs` shows the stretches happen | `walk`'s moves zeroed in its script: `walk_reaches.rs` red (the recording would replay equal everywhere and test nothing) |
| **4B-RAY-WASM** | the three `path` lines in the static world's file, compared on every target | one line's answer bit changed: every target names the line (the check's existing bite) |

### 3.4 The count (mine; bottom-up, unmultiplied)
- Code: `walk` 120, `lane_walk` 30, `roam` 30, the list 5 (185); `Lines` `f32` 10 and the finite read 25 (35); the
  three path lines 25: **245**.
- Tests: the scripts' own tests (each stretch present in the steps) 60; `walk_reaches.rs` 50; `two_instances.rs` 15;
  `two_processes.rs` 10; W2 10; the finite read's own test 20; the line counts 5: **170**.
- **415 against the plan's 550 (320 / 230)**; the rest of that row is the other designs' marks riding the recordings
  (rust-reviewer's PM-0004-30, -35, -59 on movement). Data, not counted: `walk.rec` about 163 KB and its hashes about
  92 KB; `lane-walk.rec` about 82 KB and 46 KB; two frozen bundles of about 8.4 KB; three vectors.

### 3.5 What changed since my design
1. Frozen content is one bundle file per recording (`<name>.content.json`), not a `recordings/content/<set>/` folder.
2. `first-loop` moves in step 1's commit (and maybe step 2's), not only in step 6; it is frozen anew whenever a
   content kind's field list changes (N25); `walk` and `lane-walk` are frozen once the movement's fields are final.
3. `walk` has no sights stretch (4d's).
4. Firefox already replays every recording (N14); PM-0004-20's flag-off half cannot be built (N27).
5. 4B-RAY-WASM goes into the static world's committed file, which every target compares already.
6. The two soldiers' push is in no recording (N29).

### 3.6 His open questions, by option, for step 6
- **MOVE-RULES:** the scripts' inputs are the same under every option; what `walk_reaches.rs` must find differs. (a):
  the cover wall and a roof edge gained by a jump (row 5), staying on the ramps going down (row 4), the slide with the
  speed lost (row 3), the round foot past an edge (row 2), the 0.4 m body through the alley (row 1). (c): the old
  rules (walls and houses endless: no roof, no cover top; small hops down a ramp; pushed out with the speed kept; the
  0.7 m body with 0.1 m spare in the alley). (b): row by row. The hashes are whatever the built rules give.
- **NO-SOURCE:** (a) or (b): the movement kind gains fields, so `first-loop` is frozen anew and `walk` and `lane-walk`
  are frozen after them; under (a) the values arrive with his new Export (a change under `content/source/`, his go);
  (c): fewer new fields, perhaps none. The steepest slope decides whether the 26.6-degree ramp is walked up, so one of
  `walk_reaches.rs`'s findings follows that number.
- **RESUME-FLASH:** (a), (b), (c): nothing (a pause is a rebase, which no recording holds: the simulation sees no
  pause). (d): block 5.
- **TUNE-HOME:** nothing (the recordings run on frozen content; the camera's file is no input of the simulation).
- **SIZE-4C:** (c): step 6 is page-free and goes in the first half with step 1 and step 2's cast, since his condition
  puts the recording that covers the queries in the change-set that first calls them; `lane-walk` is made there and
  used by 6b in the second half.

## Block 4. Steps 6b and 6c
Plan rows: 6b "130 / 110" (the paced replay in the real Worker, development builds only, PM-0004-28); 6c "30 / 180"
(2,000,000 hostile movement commands, PM-0004-60; the yard at the far corner of a 2 km map, PM-0004-39).

### 4.1 Step 6b, the paced replay in the real Worker (PM-0004-28: nothing of the picture reaches the simulation)
**Symbols**
- CHANGE `packages/sim-worker/src/messages.ts:18-46`: `PageToWorker` gains `{ type: "replay"; recording: ArrayBuffer;
  bundle: ArrayBuffer; hashes: ArrayBuffer }` (transferred), `WorkerToPage` gains `{ type: "replay-done"; steps:
  number; firstDifference: number | null; ignoredPosts: number }`; `isPageToWorker` (`:135-159`) takes `replay` in a
  development build only (behind the build's development define), `isWorkerToPage` (`:175-191`) takes `replay-done`.
- ADD `packages/sim-worker/src/replay-host.ts` (development only): it ends the live match (one match per Worker) and
  makes `new Sim(recording's test seed, MODE_LOCAL, 0, frozen bundle)` (the TypeScript reader refuses any seed but a
  test seed, recording.ts:79, so no real seed can be replayed: ADR 0004 part 8); then, through the real `Pacer`, per
  due tick: `input` with the recording's 13 numbers, `step` with **the recorded host time, never the live clock** (so
  a later step that reads its time, as the link simulator's will, stays replayable), `shared_hash()` against the
  hashes file (`readRecording`, `readHashes`, `firstDifference` of recording.ts reused). The pacer keeps the Worker's
  own schedule (an adapter returns the live due times; the simulation's own schedule runs on the recorded times).
  Every block still goes to the page, so the picture, the camera and the frame rate are live while the hashes are
  compared. Page posts during the replay give back their buffers; their numbers are ignored and counted.
- CHANGE `packages/sim-worker/src/worker.ts:47-66`: the `replay` message's dispatch (development only).
- ADD in `packages/client/src/app/` and the development readout (`overlay/`): a "replay" button that fetches a
  committed recording, its frozen content and its hashes (served to the development build from
  `crates/devtools/recordings/`), sends `replay`, and shows "replay hashes equal" or the first differing tick in the
  readout and in "Copy report" (the reading for his screens: plan 17, the replay button on each screen he has, its
  refresh rate noted).
- CHANGE `packages/client/scripts/build-checks.mjs:202` `DEV_MARKERS` gains the replay's own text, so a production
  build that carries it is refused.
**Tests**
| # | Where | What it holds | Planted defect |
|---|---|---|---|
| R1 | `replay-host.test.ts` (a stand-in `Sim` whose hash depends on the ray, a made-up clock) | the recorded numbers reach `input`, the recorded host time reaches `step` whatever the clock says, posts ignored and counted, the first differing step named (40) | the replay taking the newest post's (live) ray |
| R2 | `messages.test.ts` | `replay` taken in development, refused in production; `replay-done`'s shape (10) | the production guard taking `replay` |
| R3 | `packages/e2e/tests/paced/paced.spec.ts`, a folder of its own: the firefox project takes `replay/**/*.spec.ts` (playwright.config.ts:40), and CI's Firefox cannot draw the page (FF-DRAW); the plan's 6b row names `packages/e2e/tests/replay/`, which the sheet corrects (or names the config change) | a 300-tick slice of `lane-walk` at live pace, three times: the camera still; the camera orbiting by script; frames thinned to about 30 a second: each equal to its file, ignored posts above 0 (60) | R1's plant (the orbiting run differs). Control: a draw from the page's own generator changes nothing |
| R4 | `build-checks` tests | the replay's marker in the production build is refused (5) | the marker left off the list |
**Count:** code 130 (the replay host 60, messages 15, worker 10, the button and its fetch 35, the marker and the adapter
10) / tests 115: **245 against the plan's 240.**
**Facts for the sheet:** CI's Chromium run uses 3.6 to 3.9 of its 5 minutes (4B-CHROMIUM-TIME); R3's three 10-second
runs and the page's start add about 40 s: R3 needs the CI Builder's split or a new limit first (a CI file, never
ours). The whole `lane-walk` at live pace (40 s a run) is his check's button, not CI's. PM-0004-28's remaining pairs:
60 and 144 Hz on his displays (his check); sound on and muted (change-set 11).

### 4.2 Step 6c, part 1: 2,000,000 hostile movement commands (PM-0004-60)
**Tests (all new files; release-line counts in CI's `cargo test --release`, toolchain.yml:162; a short count in debug)**
| # | Where | What it holds | Planted defect |
|---|---|---|---|
| H1 | `crates/room/tests/hostile_moves.rs` | a running `Room` on the test scene with its soldier, fed decoded `Input`s whose commands run every field over its whole range (ticks near and far, sequences in order, repeated and out of order, every move in i8 x i8, every aim, every held bit, press counts jumping by up to 255): 2,000,000 commands in release, 20,000 in debug; no panic; each refusal count equal to the test's own count of the commands that broke that rule; every float of the state finite each tick; one tick per step (70) | the move normalised with no zero-length check (a NaN: the finite read red); a client-sent number used as an index (a panic); `refused_move` not counted for (-128, -128) |
| H2 | `crates/sim/tests/hostile_commands.rs` | the world's step fed `PlayerCommand`s directly, with no room before it (moves longer than 1, any aim, any presses): 2,000,000 in release, 20,000 in debug; no panic, a finite state, the soldier inside the bounds the map's edge rule keeps him in (50) | H1's first plant |
| H3 | a `sim-wasm` test | 200,000 hostile posts through `Sim::input`'s 13 numbers (NaN, infinities, numbers beyond 2^20, zero directions, any move word) into a running local match: no panic; the room refuses 0 (the endpoint turns any post into a command the room takes); every aim inside the bounds (30) | the endpoint's clamp skipped on the ground fallback |
**Reading to state (N17):** 2,000,000 of the mark's 10 million, in CI's release line, not a nightly cargo-fuzz run (no
nightly job exists; that is a CI file). Walk's commands carry no float and no id: the mark's "NaN, infinities" reach
only the post (H3), and "claims on ids that do not exist" have no field to ride. **Not in DEPART-4's fourteen (N26):
the sheet must list it for his word.** Time, to measure at the first build: about 2 to 10 microseconds a command in
release with the movement, so about 4 to 20 s per run; the toolchain job runs 8:48 to 13:02 of its 25 minutes.

### 4.3 Step 6c, part 2: the yard at the far corner of a 2 km map (PM-0004-39)
- ADD `crates/sim/tests/far_corner.rs`: the yard's shapes (from `content/maps/test-scene.json`) moved to the far
  corner of a 2 km map **built in code**, which never passes the loader (map.rs:15-16: "is no playable map"; the
  loader keeps every playable map within 327.67 m of the origin, N7). Offset: the corner at 2,000 m on x and z (the
  map's origin at a corner: f32's step there is 2^-13 m, about 0.12 mm; the format itself puts the origin nowhere,
  and that is recorded). There, PM-0004-30's parity marks (0.55 gained, 0.56 stopped, 3.05 gained, 3.10 not, the
  arc's top) and PM-0004-37's tunnelling mark (the thin wall at a sprint) hold, and a scripted soldier walking a fixed
  10-minute route (18,000 ticks) covers the same distance per second as at the origin within 0.1 %. Through the
  simulation alone: the command's aim cannot name a point there (`i16` centimetres), so the aim is 0, which the
  movement does not read.
- Planted defect: the movement's skin set below f32's step at 2 km (0.05 mm): the marks hold at the origin and fail at
  the corner.
- **The ladder (plan section 6):** at 13,150 written lines this test moves to 4d whole; the movement it proves stays
  in 4c, the one move of a proof the card itself names (plan section 0, build plan lines 355-356); he is told first.
  The parity marks themselves are rust-reviewer's (its design block 5); this is their offset run.
**Count for 6c:** code 45 (the generators and helpers 30, the offset map and the route 15) / tests 220 (H1 70, H2 50,
H3 30, the far corner 70): **265 against the plan's 210.**

### 4.4 His open questions, by option, for 6b and 6c
- **MOVE-RULES:** 6b nothing. 6c: H2's "inside the bounds" follows row 8 ((a): the map's own bounds, the speed into
  them dropped; (c): the ground less 1 m, the speed kept); the far corner's marks follow the rows as `walk_reaches.rs`
  does (block 3.6).
- **NO-SOURCE:** nothing beyond the numbers the movement already takes.
- **RESUME-FLASH:** (a), (b), (c): nothing. (d): its new input (block 5) joins H1 to H3's ranges.
- **TUNE-HOME:** nothing: 6b moves the camera by script under any option, and PM-0004-28's point is that the camera's
  values never reach the hashes.
- **SIZE-4C:** (c): 6c is page-free and goes in the first half with the movement and the room's checks it proves
  (PM-0004-60 is on the never-moved list); 6b is Worker and page code and goes in the second half with the page, with
  `lane-walk` made in the first.

## Block 5. RESUME-FLASH by option: what the code does, and where
**Today (3b), the same under every option:** on `visibilitychange` the page lets go of its keys and tells the Worker
(`packages/client/src/app/boot.ts:106-108`); the host sets `#held = 0` and pauses the pacer (`host.ts:120-124`), and on
the return resumes it with a rebase, so the first tick is due one period later (`pacing.ts:127-135`); no tick runs
while hidden. The link records the pause and clears its own copy of the held keys (`worker-link.ts:350-363`). The key
diagram follows the newest echo every frame (`boot.ts:164`, `overlay/keys.ts:62-69`), so it shows the server's last
echo until the first block after the return arrives (about P + d, 33 to 50 ms): the lit box he saw in 3b.

- **(a) Leave the rule.** What the code does with a soldier: the host also zeroes the move at the hide (`host.ts:121-
  122`, N23; block 1); the blend's rule B (`blend.ts`, step 3; N21, B2) draws the paused tick exactly on the first
  frames after the return and then blends one period into the first new tick: no slide over the hidden time, no
  jump, never back; the first command after the return holds no key and no move, so the soldier eases to a stop (with
  his stop time of 0.164 s to 10 %, an exponential easing stops a walk in about 6.3 x 0.164 / ln 10 = 0.45 m and a
  sprint in about 0.68 m; rust-reviewer confirms the easing's form); the follow camera's smoothing takes no catch-up
  step from the long gap between two frames (its time step held to a cap in `camera/follow.ts`, step 4,
  engine-architect's; a unit test whose plant is the uncapped step). The key diagram is unchanged: the 3b flash
  stays. Where: steps 1, 3 and 4, and step 7's browser test (a 5-second hide with W held in CI: 0 ticks while hidden,
  the first frames at the paused position, the soldier stopped within about 15 ticks with no input; the 1-minute and
  6-minute hides from a script of their own, as in 3b, and in his checklist). Lines: about 5 code and 40 tests beyond
  what steps 1 and 3 build anyway.
- **(b) As (a), and the development key diagram goes dark from the hide** until the first block of a tick that ran
  after the return. Where: `worker-link.ts:350-363` (at the hide the diagram's source reads nothing held; at the
  return the newest tick seen is kept) and `:277-320` (a block of that tick or older, one that was on its way at the
  hide, does not light it again), and `boot.ts:164` or a `dark` state in `overlay/keys.ts`. Test: hide with W held, a
  block still on its way with W held arrives, return: dark until the next tick's block; its plant: dark only at the
  hide, so the block in flight lights it again. About 10 code and 20 test lines; development builds only (production
  has no key diagram; 3B-CSS-PROD covers its CSS). The soldier is as in (a).
- **(c) Keys held at the hide count as held until his first input.** Where: `host.ts:121-122` stops letting go (both
  zeroings leave), `worker-link.ts:356` (`#postedHeld.fill(0)`, its model of the Worker letting go) leaves, and two
  tests reverse (3b's own in `host.test.ts`, and block 1's T2-12). The page still lets go of its keys at the hide
  (`boot.ts:107`), so its first post after the return holds nothing; the first tick after the return uses the post
  kept from before the hide only when no frame has posted before it (the tick is due a period after the return, so
  usually a post comes first): then the soldier walks on for a tick, and a key let go while away counts as held for
  that tick. It undoes 3b's rule that a hidden tab lets every key go (NA-N2). About 5 lines.
- **(d) The soldier stops dead at the return.** The simulation then takes an input made from the tab's visibility,
  which is presentation; two ways to carry it, both against "nothing of the picture reaches the simulation":
  (d1) a reserved button bit the host sets on the first post after a resume: the 13 numbers and the recording's format
  stay (the held word has 16 bits, recording.rs:77-78), but the movement gains a rule that zeroes the velocity on that
  bit, the room passes it like any button (its counter, its echo), any client can send it at any time, `walk` gains a
  stretch with it, and H1 to H3 run it over its range (about 25 code, 40 tests); (d2) a new call or a fourteenth
  number: a new name in the `Sim.prototype` pin (sim-wasm.test.mjs:246), against DEPART-3's "one new read-only
  number", and a field the format lacks, so version 2 and every recording made anew in both readers. Not
  recommended, in either form.
- **Recommendations stand as the question file has them:** the main session's (b); mine (a), "(b) if he also wants
  the 3b flash gone". Nothing here decides it.

## Block 6. The order: can steps 2 and 3 come before step 1's movement if MOVE-RULES and NO-SOURCE come late?
Facts only; the advisor agent rules on any reorder.
| Piece | Needs step 1 to exist | Needs which of his answers | Touched again later |
|---|---|---|---|
| 2a `clamp_move`, `Post::from_numbers`, `Ray` | no | none | no |
| 2b `sim::aim::aim_point`; the bounds on `StaticWorld` | no (`cast_ray` exists since 4b-2) | none (NO-SOURCE only if the 400 m reach is counted as a value) | the movement reads the same bounds (rust-reviewer's F11): an addition, not a rebuild |
| 2c the endpoint's own world, its cast, `Sent.aim`, the clamped first aim | no | none | no |
| 2d the room's two checks and counters | no | none | no |
| 2e the world into the room and the endpoint (4B-WORLD-ROOM): `Room::new`, `Endpoint::new`, `LocalMatch::new` | no | none | **yes**: step 1 adds the movement's rules and the spawn to the same constructors and their callers (N12; rust-reviewer's F12): a second, small edit of the same sites |
| 2f the 13-number input, the host, the guard, the replay readers (Rust, TypeScript, the e2e worker) | no | RESUME-FLASH (c) changes the host's two hide lines | no |
| 2g `PlayerCommand` gains move and aim | no | none | record (i): no hash moves before step 1; record (ii): first-loop's hashes and vector made anew in step 2 and again in step 1 (one generator command each time) |
| 2h the jump's rule, J1 to J5 (3B-ONE-PER-TICK) | **yes** (the jump is step 1's) | none of the five | it is built with the jump |
| 2i T2-16, PM-0002-14's page half | needs step 3's owner section and step 4's page | SIZE-4C (c): the second half | no |
| 3a `OwnSoldier` on the wire: the flag bit, the finite refusal, the decoder's generator | no | none | **its field list, yes**: it is step 1's soldier state, and MOVE-RULES can add or remove a field (block 2.5) |
| 3b the room's snapshot fill; S4 (PM-0001-17's first piece) | **yes** | none directly | no |
| 3c the endpoint's decode into the row, the header's count, the owner's section | the writer and its tests no; a soldier to fill it in play, yes | none | no |
| 3d the block's layout (engine-architect's) | no | none (lane 2's list and the plan's 6.2) | no |
| 3e the due-time stamp, three blocks, `blend.ts` (B1, B2), the pool of 16 | no | none (rule B serves RESUME-FLASH (a) and (b); (c) and (d) leave the blend as it is) | no |
| 6 `walk`, `lane-walk`, `walk_reaches.rs`, the plants on movement fields | **yes** | MOVE-RULES (what `walk_reaches.rs` must find), NO-SOURCE (the fields that are frozen) | first-loop made anew at each change of the state's bytes |
| 4B-RAY-WASM's three lines | no | none | no |
| 6b the paced replay | its host and guard no (they run on `first-loop`); its real run needs `lane-walk` | none | no |
| 6c H1, H2 / H3 / the far corner | H1, H2 yes / H3 no (step 2 only) / yes | the far corner's marks follow MOVE-RULES | no |
**What these facts give:**
- Of steps 2 and 3, everything but 2h, 3b and a soldier for 3c's row needs neither step 1 nor any of the five answers,
  except RESUME-FLASH (c) on two host lines. Built before step 1, nothing is built twice in substance; three things are
  touched twice: the constructors (2e, a few lines per site), the snapshot's soldier field list (3a), and, under
  record choice (ii) only, first-loop's hashes (2g, a generator command, no hand work).
- What waits for MOVE-RULES and NO-SOURCE: step 1 itself, 3a's field list, step 6's recordings and their frozen
  content, `walk_reaches.rs`, the far corner. Neither answer touches anything else in steps 2 and 3.
- In the meantime no recording holds a move or a ray (first-loop has zeros): the endpoint's cast is proven by unit
  tests and H3 until `walk` exists. His FEATURE-COMPARE condition binds the change-set, not the step order: `walk`
  must still land in the pull request that brings step 1 and the cast.
- Under SIZE-4C (c), 2a to 2g, 3a, 3c's Rust and 3d belong to the first half (page-free), 3e and 2i to the second. Doing
  second-half work while the first half waits for his go meets the single-branch limit the question file states
  ("while one waits for him, this chat can only write notes, because it has a single branch"): it can be built, not
  pushed.

## Block 7. Three written answers the advisor agent's verdict asks for (advisor-reread-4c.md items 6 and 11)
Asked 2026-11-28 on head 3ab59bd7: (1) the record rule, (2) one home for the bounds, (3) the zero ray. Read only;
this block is my only write. rust-reviewer answers (1) from its side at the same time; I did not wait for it.

### 7.0 Plan
1. Re-read the ruling (advisor-reread-4c.md :30-31, :40), my block 1.1 (:73-83, :84-102), block 6 (:579), N24, N30,
   rust-reviewer-read 1.1 (:71-73) and 1.6 (:240-247), world.rs (:33-51, :112-117, :143-191), room/lib.rs (:262-300),
   map.rs (:9-14, :33-52, :244-250, :316-335, :645-646), collision.rs (:94-108, :212-250), endpoint.rs (:56-88),
   worker.ts (:53-54). A notes line per finding.
2. Answer 1: yes or no; a command-less tick under (i); the bytes; the restore test and the room pin with their plants;
   first-loop's hashes in step 2, with the conditions.
3. Answer 2: yes or no; the derivation; the readers; the tie test and its plants.
4. Answer 3: yes or no; the definition; the layer; the two tests and their plants; where the count lives.
5. What this supersedes in blocks 1, 3 and "For the sheet", by line; the count's change. Then DONE.

### 7.1 Notes (one line per finding)
- B7-N1 (the record today): `World.player` is the whole `PlayerCommand` (buttons, presses), hashed as bytes 4 to 21 of the shared state (world.rs:18, :115, :185-187); `authority_step` keeps it when a tick has no command (:143-151). The room hands `&[]` to the world on a tick whose queue gives nothing due (room/lib.rs:262-282) and runs a late command in a later tick (`late`, :273-275); the echo reads the record (:286-296). So a starved tick reaches the world as a command-less tick, and nothing outside the world repeats a command today.
- B7-N2 (my (i) left a gap): block 1.1 (:75-81) said the soldier "consumes" move and aim and never said what a tick with no command does; N24 (:50) the same. With nothing kept, a command-less tick would stop him, a rule change nobody chose. The ruling closes it; my answer below keeps the held move and aim in the soldier's row.
- B7-N3 (the row's fields): rust-reviewer-read 1.1 (:71-73) gives the Soldier `aim: V3` and `facing: [f32; 2]` and no held move; its 1.6 (:246-247) has `visit_shared` write "the move and aim of the record", which contradicts its F17 (:29) taking (i). The move is the one new field.
- B7-N4 (no signed method): `StateSink` has `u16`, `u32`, `bytes`, `secret` (world.rs:57-67) and no `i8`; the path form `i8::to_le_bytes(x)` is what the rule scan needs (world.rs:91-92). rust-reviewer adds `f32` in step 1 (F12), which the `aim: V3` field uses.
- B7-N5 (no reader of the bytes): `write_state` is `#[cfg(test)]` (world.rs:188-195) and nothing reads canonical bytes back. A restore test needs a test-only reader; change-set 7's reset is the real one and is not built here.
- B7-N6 (first-loop's aim): first-loop runs on the test scene (N10) whose bounds hold the origin (x -32..80, y 0..20, z -24..24; N28), so the clamped start aim is (0, 0, 0) cm, exactly what today's endpoint writes (endpoint.rs:251-263). Its rays are all-zero bits (N9).
- B7-N7 (the grid trap): `on_grid` rounds `value * 100` and keeps it only if it divides back to the same double (map.rs:244-250); 0.55 x 100 is not 55 to the last bit (map.rs:645-646), so a plain `ceil` on a lower face at 0.55 gives 56. `on_grid` is private to content today (`fn`, no `pub`).
- B7-N8 (the far corner): block 4.3's map is built in code from 0 to 2,000 m on x and z: on the grid, but past the aim's reach (327.67 m, map.rs:33-40). My block 1.1's "off the grid refused" would not refuse it; a reach check in `build` would. `build` checks shape corners for finiteness and thickness (collision.rs:227-232), the bounds not at all.
- B7-N9 (the guard's layer): a page message the guard refuses becomes a Worker error that ends the match (worker.ts:53-54). So a non-finite ray from the real page never reaches `Sim::input`; the endpoint's count serves the callers without the guard: the replays (replay.rs:150, recording.ts:160, determinism.spec.ts:26-27), H3, and the endpoint's own tests.
- B7-N10 (the counts' home): `Readout` (endpoint.rs:56-79) is the endpoint's counts for the block and the tests, "Each count wraps"; nothing in it is simulation state.

### 7.2 Answer 1, the record rule: YES, (i) with the held move and aim in the soldier's hashed row
The invariant holds as ruled: every value a later tick reads is in the hashed state. Under (i) the record stays the
player's buttons and presses (bytes 4 to 21 of the shared state, unchanged; the echo and the press base read it); the
move and the aim are the soldier's, kept in its row.
- **A tick with commands** (k of them for the slot, oldest first, after the room's checks): the newest one's buttons
  and presses become the record, as today; its move and aim are written into the soldier's row before the movement
  runs. The movement then runs **once** for the tick (3B-ONE-PER-TICK): the press base is the record before the tick
  against the newest's counters.
- **A tick with no command for the slot** (the room hands the world `&[]`, room/lib.rs:282): the record holds (held
  buttons stay held, the counters do not move, so no new press: no jump; Space held still jumps at a landing, J3);
  **the row's held move and aim hold**. The movement runs once from the held move (he walks on with the same wish), and
  the facing turns toward the held aim from his new position. A command-less tick is the tick of a command repeated
  with no new press. Nothing outside the world repeats a command: the room keeps passing `&[]` (B7-N1).
- **At spawn** the held move is (0, 0) and the held aim is whatever FACING-SPAWN picks (item 10 (5), open), written
  into the same field. The first command overwrites both.
- Failure mode, said so it is not decided silently: a client whose commands stop walks on at the held move until the
  next command or the room's end of the connection, as a held button does today. A starvation limit that stops him is
  a rule for change-set 7 or for the connection's timeout, not made here.

**The bytes** (rust-reviewer's Soldier, its read 1.1 :71-73, owns the row's order; these two follow `facing`):
- `held_move: [i8; 2]`, new: move_x then move_y as the room took them (length within 127), **2 bytes**, written as
  `sink.bytes::<2>("soldier.held_move", &[i8::to_le_bytes(move_x)[0], i8::to_le_bytes(move_y)[0]])`: the two's
  complement bits, the path form the rule scan needs (world.rs:91-92), no new sink method (B7-N4).
- `aim: V3`, the field the ruling names (rust-reviewer's): written at take as `f32::from(x_cm) / 100.0` per axis
  (`i16` and `u16` to `f32` are exact, one correctly rounded division, the same bits on both targets; never a minus
  zero, since 0 converts to +0.0), **12 bytes** through `StateSink::f32` (bits, little-endian, rust-reviewer's F12).
  So 14 bytes of each row hold them. **Flagged option, not my answer:** hold the command's integers instead (`AimCm`,
  x `i16`, y `u16`, z `i16`: 6 bytes, so 8 bytes in all) and divide where the facing reads them: one canonical form, no
  float to restore, exactly what the room judged; rust-reviewer's call, either width meets the invariant.
- The record never takes move or aim: `visit_shared` writes the tick, the record (22 bytes as today), then the soldier
  rows. That settles rust-reviewer-read 1.6 (:246-247) against its F17 (:29) in (i)'s favour, if it agrees.

**The restore test** (step 1, in world.rs's test module beside `write_state`, which is `#[cfg(test)]`, :188-195):
- A test-only reader `read_shared(bytes, constants) -> World` that takes the shared bytes field by field in
  `visit_shared`'s order; the match constants (rules, static world, spawns) and the seed come from the same build and
  setup, never from the bytes. It is a test's tool, not change-set 7's reset (B7-N5).
- It plants: on the test scene, one soldier; ticks 1 to 30 with move (127, 0) and an aim on a box top; then **ticks 31
  to 60 with no command** (`authority_step(&[])`). Copy A runs straight; copy B is rebuilt from A's shared bytes after
  tick 30 and runs the same command-less ticks.
- It must catch: A's and B's shared bytes equal at every tick 31 to 60; the soldier's position at tick 60 advanced by
  30 ticks of the held move (he did not stop); his facing at tick 60 the turn toward the held aim from where he stands
  (not the tick-30 facing); no jump. Plants, each red: (P1) `held_move` left out of `visit_shared` (`held_move: _`):
  B stops at tick 31, bytes differ; (P2) the held aim left out: B's facing differs from tick 31; (P3) a command-less tick
  that zeroes the move (the narrow record with nothing kept): A stops, the "advanced by 30 ticks" check is red.
- **The room's pin** (room tests): a room on the test scene, commands 1 to 30 with move (127, 0), then the datagrams
  for ticks 31 to 33 dropped so the queue gives nothing due at ticks 31 to 33, then the repeat brings them late. At
  ticks 31 to 33 the world is handed **no** command (a test-only count of the slice length at room/lib.rs:282, beside
  `commands_run`) and the soldier walks on; when the late commands run, `late` counts them and the newest's move is
  taken. Plant: the room re-feeds `last_run`'s command on a starved tick (the slice length becomes 1: red). Without
  this pin, P3 together with that re-feed would look right in play while the held move lived outside the state, and the
  world-only test would stay green on a correct world.
- Count: the reader 35, the world test 35, the room pin 30 (tests, all step 1's); the held move in the row 10 (code).

**First-loop's hashes in step 2: NO, they do not move**, on four conditions, each written down:
1. Step 1's room already hands each command's move and aim to the soldier (rust-reviewer's 1.6 puts the
   `PlayerCommand` fields in step 1; my block 1.1 had them in step 2: step 1 it is). If step 1's soldier kept a spawn
   aim until step 2, step 2 would move the hashes and would re-make first-loop in its own commit.
2. The all-zero ray is no ray and keeps the aim (answer 3).
3. The test scene's bounds hold the origin, so the clamped start aim is (0, 0, 0) cm, the same aim today's endpoint
   writes (B7-N6); and first-loop's move word is 0, which `clamp_move` leaves (0, 0).
4. The proof: `crates/devtools/tests/recordings.rs` green in step 2's commit with no recording file re-made: `:24`
   (every committed file is what its script makes, the hashes included; `make_recordings --check` the same) and `:29`
   (every recording replays to its hashes).
So the commands the room runs in first-loop are byte-identical before and after step 2. In the other order (step 2
before step 1, item 7's conditions) no soldier exists, nothing reads move or aim, and nothing moves (block 6 row 2g
stands). First-loop moves in step 1's commit, which re-makes it anyway (F12).
## For the sheet: overlaps and readings not yet listed (flagged, not resolved)
1. The post's shape: 13 numbers with the move packed in one word (the plan, the recording's format) against
   engine-architect's design 3.6 (14: `move_x, move_z` as two numbers). I recommend 13 (block 1.1).
2. The bounds on `StaticWorld`: the aim's cm (mine) and the movement's metres for the floor and the four sides
   (rust-reviewer's F11): one home, built once (N30).
3. 4B-RAY-WASM: the checkpoint card puts it in rust-reviewer's read too; mine gives where it runs (the static world's
   file), rust-reviewer picks the three rays and owns 4B-RAY-COUNTS.
4. How the room's and the endpoint's tests get a world without naming `content` (N11): a dev-dependency (a manifest
   change, the dependency hold) or a world made through `sim`. Engine-architect's call; step 2 needs it.
5. Readings not in DEPART-4's fourteen, for his word: PM-0004-60 at 2,000,000 commands in CI's release line, not 10
   million nightly through cargo-fuzz, with no id and no float in Walk's commands (N17, block 4.2); PM-0002-14 met in
   its ray half only until change-set 8 brings the view time (block 1.4); PM-0004-20's hash half on `walk` (one
   soldier) and its flag-off timing not runnable (N27).
6. The record's width (N24): whether the World keeps the command's move and aim decides whether step 2 moves any
   hash. rust-reviewer's and engine-architect's choice; I recommend the narrow record.
7. The soft push is in no recording (one slot): its cross-target identity needs its own WebAssembly run (N29).
8. CI: 6b's paced replay needs the CI Builder's answer to 4B-CHROMIUM-TIME first (N15); the 2,000,000-command runs go
   in the release line, which has room (N16).
9. J1 to J5 (the jump's rule, 10 code / 80 tests) are counted in my step 2 (block 1.3), though their code is built
   in step 1's movement: if rust-reviewer's step 1 count holds them too, count them once.
10. Corrections to my own design of 2026-11-25, all applied above: the aim falls back to the ground plane at 400 m (N6);
   the blend's allowance rises at once to the largest of 64 and falls gradually, not the 90th percentile, the
   estimator pinned by B1 (N19); the fraction is measured back from the
   newer block's due (N21, rule B); the pool is 16; there is no placement counter; first-loop moves with step 1.

## Counts, one line per block (bottom-up, unmultiplied; plus or minus 30 %)
- Block 1, step 2: 315 code / 630 tests = 945 (plan 385 / 530 = 915).
- Block 2, step 3: mine 215 / 325 = 540; with engine-architect's layout about 320 / 340 (plan 320 / 340).
- Block 3, step 6: mine 245 / 170 = 415 (plan 320 / 230 = 550, the rest being the other designs' marks).
- Block 4, step 6b: 130 / 115 = 245 (plan 130 / 110); step 6c: 45 / 220 = 265 (plan 30 / 180).
- Block 5, RESUME-FLASH: (a) about 5 / 40 beyond steps 1 and 3; (b) 10 / 20 more; (c) about 5 changed, two tests
  reversed; (d1) about 25 / 40; (d2) a format version and every recording anew.

DONE
