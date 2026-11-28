Task: rust-reviewer's design read for 4c step 1 (the simulation's soldier and movement) against main dae6d107, for the re-read sheet (plan.md section 3 item 1). Read only; no source edits; no commit.

## Plan (blocks, in order)
0. Read today's code: crates/sim/src (lib, collision, vec, world, time, rng, state_hash), crates/sim-wasm local_match.rs, crates/content/src (movement.rs, map.rs, time.rs), content/movement/soldier.json, content/source/testlines-table.json, build-checks.mjs (math routine list, FMA/SIMD rules), rules_scan.rs, parry3d 0.31.1 query API in ~/.cargo/registry (std off), game-designer-cs4.md, rules-cs4.md, STATUS_ARCHIVE 2278-2300, 4b-2 report.
1. Step 1's pieces against today's code: symbols add/change/remove (file:line), tests with planted defects, marks, counts recounted.
2. 4B-RAY-COUNTS and the simulation side of 4B-QUERY-RECORDING.
3. MOVE-RULES rows 1-5, 8 and NO-SOURCE (b) fields by option; what can be built before his answers.
4. Risks, ranked.

## Notes lines
- [read] branch cs/4c-soldier at 24929c43 (main dae6d107 + notes). No agent memory dir exists. Read: sim lib.rs (48), vec.rs (24: V3 with new/to_array only), time.rs (17), world.rs (477: World{tick, player: PlayerCommand{buttons,presses}, seed}; StateSink has u16/u32/bytes/authority/secret, no f32), collision.rs (954: StaticWorld{solids, tree: Bvh, depth}, Solid{min,max: [f32;3], form}, enter_box :114, enter_ramp :152, cast_ray :300, RAY_REACH :32, MAX_TREE_DEPTH 31 :38; boxes and four-way ramps only, axis-aligned), sim-wasm local_match.rs (LocalMatch owns StaticWorld :50, no step queries it), room lib.rs (Room owns World :88), content movement.rs (15 fields :18-34), time.rs (units_from_seconds :26), map.rs (spawn facing_deg any [0,360) :66), soldier.json, testlines-table.json, build-checks.mjs (FUSED_STEPS :696, BANNED_MATHS :717 incl. exp/expf/atan2/sin/cos, libmProblems :765), maths-allowed.mjs (2 entries: C::round, C::__multi3), rules_scan.rs (NAMED :114 incl. exp, atan2, sin, cos, min, max, clamp; LIBRARY parry3d only in collision.rs :95-96; content lint guard :820-840), devtools world_proofs.rs (ray classes :97-150, line writer :436-470), static-world.txt.
- [F1] libm 0.2.16 exp/expf have no arch path on x86-64 or wasm32 (only x86 without SSE2: exp.rs:86-90, expf.rs:35-39; arch/mod.rs:10-36 dispatches only sqrt, fma on sse2): pure Rust on both targets, no fma inside (grep 0 hits in exp.rs, expf.rs, atan*.rs). Same for atan2.
- [F2] A sim call of libm::exp needs three changes beyond the code: `libm` as a direct dependency of sim (manifest + lockfile edge: DEP-HOLD batch), the scan (NAMED holds "exp": `libm::exp(` is a path call and is refused, rules_scan.rs:114-116 with the path rule in the header :19-24), and an entry `libm::math::exp` in maths-allowed.mjs with vectors.
- [F3] parry's typed shape cast (shape_cast_support_map_support_map.rs:36-47) calls the contact query whenever TOI < 1e-4 with stop_at_penetration false, i.e. every sweep that starts resting at the skin (most grounded ticks); that query runs EPA::new() (Vecs + BinaryHeap, contact_support_map_support_map.rs:64-67) whenever the core shapes overlap: allocation in the step on any overlap. Ties between two shapes at one TOI are decided by GJK tolerance, the problem collision.rs:7-12 already names for rays.
- [F4] Today's world is axis-aligned boxes and four-way ramps only (collision.rs:76-83, enter_ramp :152). A vertical capsule (segment half h, radius r) against a box is the ray of its segment centre against (box grown by h in y) rounded by r: three slab boxes (today's enter_box on grown corners) plus twelve edge capsules; against a ramp the same over its extruded profile (a quadrilateral prism). All + - * / sqrt and comparisons: no library call, no allocation, ties by index as cast_ray.
- [F5] Spawn facing (map.rs:66, any degree in [0, 360); test-scene all 90) needs a sine and cosine once per spawn if the state keeps facing as a vector: a third and fourth routine, not in the plan's "two that join". Not addressed in any design (grep facing_deg: only engine-architect :199).
- [F6] Content ranges let jump_reach_m < step_up_m (movement.rs:25,27): take-off speed sqrt(2 g (reach - step)) of a negative is NaN. Rules::take needs a cross-field refusal. Also durations that round to 0 units (Range::above(0.0, ..) admits 1e-9 s; time.rs:33 rounds to 0) for sprint and refill times.
- [F7] Export reach 3.071 (soldier.json): take-off 11.000364 m/s; apex at 13.75 ticks; tick-sampled apex 2.52017 m, true 2.52100 m. Sampled reach 3.07017 sits 0.83 mm under content's 3.071 (PM-0004-59's 1 mm holds by 0.17 mm); the shortfall ranges 0 to g dt^2/8 = 3.33 mm with the tuning. The in-air step-up must test the tick's true top y + vy^2/(2g) when vy crosses zero inside the tick.
- [F8] Test-scene slopes as grades: yard_ramp_gentle 25 % (14.0 deg), yard_ramp_steep 50 % (26.6 deg), roofs 95 % (house a, 43.7 deg) and 89 % (house b, 41.8 deg). Quake's 0.7 normal limit (45.6 deg, 102 %) makes the roofs walkable: the steepest-slope number decides whether he walks up a roof after gaining its 2.6 m edge.
- [F9] Easing with his values: k = 1 - exp(-ln10 dt / 0.164) = 0.37374 a tick; up and stop equal, air 100 %: MOVE-RULES row 8 (a) and (c) give the same bits today (k x 1.0 exact), so building (a) wastes nothing.
- [F10] MOVE-RULES row 5 (c) (walls and houses endlessly high) needs a field the map kind does not have (Shape is id, min, max, form only: map.rs:104-118): a content-kind change, not a sim one.
- [F9 corrected] k = 0.37375 a tick (1 - exp(-ln10 x (1/30) / 0.164) = 0.373750; the old game's 1 - exp(-14/30) = 0.372911).
- [F11] The test scene has no ground shape (23 shapes, none under the street or the yard): the floor is the bounds' bottom (y = 0; bounds [-32,0,-24]..[80,20,24]). The sweeps must hold the bounds as solid (floor walkable; the four sides by MOVE-RULES row 8; the top at 20 m). Shapes confirmed: yard_step_055 0.55, yard_step_056 0.56, yard_ledge_290/305/310 at 2.90/3.05/3.10, yard_thin_wall 0.2 m thick (x 30..30.2), the alley between the houses 1.5 m (x -5.9..-4.4), street_cover_wall 1.0 m, house tops 2.6 m with roof ramps to 4.7 and 4.3 m, ramps meeting platforms at a shared face (x 45, y 1.0 and 1.5): a seam a sweep must cross.
- [F12] Callers outside sim that step 1 changes: room/src/lib.rs:108 (World::new), :219 and queue.rs:137 (PlayerCommand literals), :282 (authority_step), :286-296 (player() into the snapshot's OwnState), :314 and :651 and tests/any_bytes.rs:117 (MatchSetup literals); sim-wasm/src/lib.rs:118, local_match.rs:50/63/98/212/394/407, tests/step_alloc.rs:76; sim/tests/state_hash_alloc.rs:70/94; devtools/src/replay.rs:181 (impl StateSink: a new float method breaks it), devtools/tests/hash_cost.rs:40. world.rs:401-415's known answers (from a script) and the committed first recording (crates/devtools/recordings/first-loop.*, manifest.txt) pin today's state bytes: both change in step 1, not in step 6.
- [F13] The scan refuses `libm::exp(`: rules_scan.rs:386 `on_a_path = before == "::" && !NOT_FLOATS.contains(&before_2)`; no bite or control names libm except libm::fma (:587). Confirmed F2.
- [F14, prototype, my scratch rr-sweep-proto.rs, rustc 1.99.1 -O, f32] Our own sweeps against boxes, 2 mm skin, up/across/down step-up, 4 slides; 0.55/0.56/0.30/0.05 m steps at 6.3, 9.576 and 13.27 m/s from three starts each (36 runs): (1) without the rule "touching and moving along or away is no hit" nothing moves at all (resting on the floor at the skin is a hit at t = 0 every sweep); (2) a CAPSULE with edges counted by their normal perches on the step's rounded edge in the down phase and stalls (0.55 m: foot 0.448 to 0.543, x stuck); (3) with edges counted as walls the capsule climbs in two stages, perch then step from the perch: the 0.56 m step is GAINED in all 9 runs (foot 0.5620): PM-0004-30 fails; (4) a vertical CYLINDER (flat foot, same radius and 1.80 m): 0.55 gained (foot 0.5520) and 0.56 stopped at x 29.5980 (= 30 - 0.4 - 0.002) in all 9 runs each; (5) a tie of two boxes sharing the plane x = 30: the lower index wins in both list orders; (6) 10,000 seeded moves at 13.27 m/s into the 0.2 m wall: 0 past its mid-plane (both bodies). Outputs: rr-sweep-proto-capsule.txt, rr-sweep-proto-cylinder.txt beside this file.
- [F15, prototype round 5] Two more capsule rules over the same 9 runs per height: the down phase landing on a face only: 0.56 stopped, 0.55 NOT gained at 6.3 and 9.576 m/s (6 of 9; x stays 29.5980), gained at 13.27 only; (B) the step raised from the last face's height: 0.55 gained 9 of 9, 0.56 perched on its edge (foot 0.5540, x 29.9202) 9 of 9. Files: rr-sweep-proto-capsule-face.txt, rr-sweep-proto-capsule-base.txt. The prototype has no vertical motion, no ramps, no push.
- [F16] GROUND is written nowhere: map.rs:3-5 ("bounds is the playable box"; regions "only drawn"); engine-architect's design and the 4b-2 report have no line on it; the page's floor (engine-architect-read N10) and netcode's aim fallback to y = 0 (netcode-architect-read 1.1) assume it.
- [F17, sibling reads] netcode-architect-read.md 1.1: PlayerCommand gains move_x, move_y: i8 and aim: AimCm (i16, u16, i16) in a new sim/aim.rs, the record stays buttons and presses (option (i)), StaticWorld gains bounds_cm: I take its widths (withdraw my i32). Its :357 takes StateSink's f32 (my F12): agreed. Its :403 and design 1.4 line 155 re-make first-loop in step 1's commit: agreed. engine-architect-read N23 counts the body height under NO-SOURCE and writes "the capsule's height": BODY-SHAPE would change that word only.

## Block 1. Step 1's pieces against today's code (DONE)
Counts are written lines, bottom-up, not multiplied (code / tests). "Ours" and "library" are the two sweep routes of
1.4; the body is the flat-footed cylinder of 1.4 unless the sheet keeps the plan's capsule (then 1.4's fix (B) is
added). Names of new types are proposals; the module split is engine-architect's (its block 2.1).

### 1.0 Two choices this read puts on the sheet (not decided here)
- **BODY-SHAPE (a finding against plan 6.1, and against my own design's block 3 item 7).** Plan 6.1 says "a vertical
  capsule". My prototype (F14) shows a capsule with the plan's up/across/down step-up gains the 0.56 m step by
  perching on the step's rounded edge and stepping again from the perch (9 of 9 runs, foot 0.5620): PM-0004-30 fails.
  The query route does not matter here: this is the shape's geometry. Which acceptance rule gives which result, each
  over the same 9 runs per step height (F15; files rr-sweep-proto-*.txt): capsule, the step's down phase may land on
  an edge: 0.55 gained, **0.56 gained** (perch, then a second step from the perch); capsule, the down phase lands on a
  face only (the rule my 1.5 item 7 first wrote): 0.56 stopped, but **0.55 not gained at 6.3 and 9.576 m/s** (6 of 9;
  only 13.27 m/s carries the centre past the edge in one tick); (B) capsule, the step raised from the height of the
  last face stood on: 0.55 gained 9 of 9, the 0.56 step **perched on its edge** (feet at 0.554 m, centre 8 cm short of
  the face) in 9 of 9: not on top, not on the ground; (A) vertical cylinder, flat foot: 0.55 gained and 0.56 stopped
  on the ground, 9 of 9 each. Options: (A); (B) with a rule for the perch (it slides off, or the perch counts as
  stopped by the mark's reading); (C) the capsule with face-only landing and an across phase pushed one radius past
  the contact (moves farther than the velocity; untried). **(A) changes what MOVE-RULES row 2 told him** ("holds a
  little past an edge (the body's round foot rests on it)"): with (A) he stands a full 0.4 m past an edge, and floats
  over a ramp at his centre by radius x grade (0.10 m on the gentle ramp, 0.20 m on the steep one) unless the foot gets
  a smaller radius of its own, a number NO-SOURCE does not list. So BODY-SHAPE is a point for game-designer and for
  him, not an engineering pick. This read's engineering view: (A) is the only one that met both steps; ramps,
  vertical motion and the push are untried for all of them (risk 2).
- **SWEEP-ROUTE.** (Ours) our own sweeps on today's corners, as the ray is (collision.rs:7-12 gives the reasons, and
  they hold for sweeps: F3); the library keeps the tree (build, walk, depth) and serves as the tests' oracle. (Library)
  parry3d's typed cast (`query::details::cast_shapes_support_map_support_map`, shape_cast_support_map_support_map.rs:10)
  of a `Cylinder` or `Capsule` against a `Cuboid` and a `ConvexPolyhedron` wedge. The Walk card under his go says "our
  own movement on parry3d's queries", and PM-0004-20 and -35 are worded for the library's queries: route Ours is a
  departure for the advisor agent (DEPART list), with these readings: the library's 33 packages give the binned tree
  and its walk in the players' file and the oracle in the tests; PM-0004-35's "0 Err(Unsupported)" holds by
  construction (an exhaustive match on `Form`, no dispatcher in the game; the test runs every form pair once through
  the library's dispatcher as today's ramp test does, collision.rs:889); PM-0004-20 covers the tree only.
  Recommended: Ours (ties by index, no allocation by construction, no 1e-4 rule, no GJK tolerance, no libm sqrtf
  linked); measured only on boxes so far (F14).

### 1.1 The soldier table and ids that go stale (PM-0004-15)
- Add `crates/sim/src/ids.rs`: `SoldierId { slot: u16, generation: u16 }` with `to_bits() -> u32` / `from_bits(u32)`
  (named as integers: the scan's float rule does not see u32 methods, rules_scan.rs:72-74), `NONE`.
- Add `crates/sim/src/soldiers.rs`: `pub const MAX_SOLDIERS: usize = 48` (the block's 48 rows, plan 6.2);
  `Soldier { generation, occupied, alive, team: u8, position: V3 (the feet, as content's Spawn.at, map.rs:94),
  velocity: V3, grounded, sprinting, stamina: u64, locked, since_sprint: u32 (units, saturating), facing: [f32; 2],
  aim: V3 }`; `Soldiers { slots: [Soldier; MAX_SOLDIERS], count: u16 }` (a fixed array: no allocation ever);
  `spawn(slot, at, facing) -> Option<SoldierId>` (raises the generation with `wrapping_add`), `despawn(id) -> bool`,
  `resolve(id) -> Option<&Soldier>`, `resolve_mut`, `iter()` in slot order. All indexing by `get`/`get_mut`: a slot
  from anywhere never panics.
- Tests: 10,000 cases of up to 1,000 spawn and despawn operations; the row names a 64-slot pool (ADR 0004:1136) and
  the table holds 48: the test runs on a table built with 64 slots (the capacity a const parameter), or the sheet
  states 48 as a reading of the row (DEPART). Drawn from `ChaCha8` (rng.rs, the
  plan's DEPART-4 reading). Plants: a spawn that does not raise the generation (the stale id resolves: red); a
  `despawn` that leaves `occupied` set (red). Also: the generation's wrap after 65,536 reuses of one slot is stated
  and tested (an id that old resolves again: ABA by design, 54 hours of a respawn every 3 s).
- Mark: PM-0004-15. Count: 150 / 140 (engine-architect had 150 / 120).

### 1.2 The rules taken from content once per match
- Add `crates/sim/src/rules.rs`: `Rules` (all f32 unless said): `walk`, `sprint` (= walk x sprint_multiplier),
  `ads` (4d), `ease_up`, `ease_stop` (shares a tick), `air_share` (air_control_pct / 100), `radius`, `half_height`,
  `step_up`, `g_dt`, `half_g_dt2`, `take_off` (= sqrt(2 g (reach - step_up)) in f64, then narrowed), `inv_two_g`
  (for the arc's true top, 1.5), stamina in integers (`stamina_full: u64` = sprint units x refill units, `drain:
  u64` = UNITS_PER_TICK x refill units, `refill: u64` = UNITS_PER_TICK x sprint units, `returns_at: u64` =
  ceil(full x pct / 100) in f64, exact below 2^53), `push_share`, `push_reach` (2 x push_radius); `Rules::take(&
  content::movement::Movement) -> Result<Rules, RulesError>` and `RulesError::message()`; each derived number in f64
  from the content's f64 (movement.rs:39-56), then narrowed once.
- Refusals that the content's own ranges let through (F6): `jump_reach_m < step_up_m` (the take-off speed would be
  the square root of a negative); a sprint or refill time that rounds to 0 units (`Range::above(0.0, ..)` takes
  1e-9 s, time.rs:33 rounds it to 0); under BODY-SHAPE and NO-SOURCE, a body height under twice the radius.
- The easing share: k = 1 - exp(-ln10 x dt / T), with `core::f64::consts::LN_10` (a constant, no call) and one
  exponential per time (1.8). Speed-up and stop at 0.164 s give 0.373750 (F9).
- Plan 6.2's choices, confirmed from this side: **(5)** the Export's names and units, easing as two times: confirm
  (the sim works both shares out once; the air share multiplies the one in use; with today's values row 8 (a) and
  (c) give the same bits, F9). **(6)** the reach stored and the take-off worked out: confirm, with the refusal above.
  Correction for plan 6.1: "the exact arc gives 3.0700 m" was for a take-off of 11 m/s; with his reach as content
  (3.071) the tick-sampled top gives 3.0702 m and the true top 3.0710 m (F7). Every jump test compares to
  `jump_reach_m` as read, never to a literal 3.07 (the true top sits exactly 1.0 mm from 3.07).
- Tests: every field reaches its own rule (one value moved at a time changes exactly the numbers that use it); the
  refusals each by name; the shares against a Python `decimal` reference within 1 ulp of f32. Plant: ease_stop
  taken from the speed-up time (the "moved one at a time" test is red).
- Count: 130 / 110.

### 1.3 Our vector and the three float helpers (3A-SCAN-HELPERS)
- Today `V3` has `new` and `to_array` only (vec.rs:12-23). Add `add`, `sub`, `scale`, `dot`, `length_squared`,
  `length` (f32 `sqrt`: the instruction f32.sqrt in WebAssembly, exact, on part 7's list), `flat` (y = 0),
  `is_finite_all`; no `normalize` that can divide by zero (a `try_unit` returning `Option`).
- Add `crates/sim/src/fmath.rs`: `min_f32`, `max_f32`, `clamp_f32` (plain comparisons; on equal signed zeros the
  first argument, written and tested; `clamp_f32(x, lo, hi)` with lo > hi returns lo, never panics: ADR 0004
  :271-275). Their names hold no banned word, so the scan passes them as written (rules_scan.rs:19-24, and its
  control `min(a, b)` at :622 shows a bare name passes).
- The scan (rules_scan.rs): 3A-SCAN-HELPERS' two false refusals (archive :1572-1575) do not arise with these names;
  what the scan gains is (i) a bite: a helper whose body calls std `min` (refused, as today), and (ii) the one
  exception the libm route of 1.8 needs: `libm::exp` named in one file only (`rules.rs`), on the pattern of the
  library's one-file rule (`LIBRARY`/`LIBRARY_FILE`, :95-96), with bites (`libm::exp` in movement.rs refused;
  `libm::sin` in rules.rs refused) and a control. And a guard like the content crate's (:820-840): the new modules
  deny `clippy::indexing_slicing`, `unwrap_used`, `expect_used`, `panic`, `arithmetic_side_effects` outside tests
  (a panic from a command is a server's denial of service); world.rs keeps its one documented `expect` (:156) under
  an `allow` with a reason.
- Tests: the helpers on (+0, -0), (-0, +0), NaN never reaches them (asserted at entry), a reversed clamp; vectors run
  natively and in WebAssembly (PM-0004-22). Plant: a helper built on std `min` (the scan is red; the signed-zero
  vector is red on the first argument rule).
- Count: 70 / 80 (vec 30 + fmath 40; scan 40 of the tests).

### 1.4 The sweeps on the collision world
What a sweep needs beyond today's `cast_ray` (collision.rs:300):
1. The body's shape against each form, as the ray of its centre against the form grown by the body. Cylinder (A)
   against a box: the box grown by the half height in y and by the radius in x, the same in z (today's `enter_box`
   on grown corners), and four vertical cylinders at its vertical edges (a quadratic, `sqrt`): F14's prototype,
   about 40 lines. Against a ramp: the wedge grown by the half height (a quadrilateral prism along the axis across the
   rise), whose slope plane moves out by radius x the horizontal part of its normal, its vertical edges as vertical
   cylinders, its two inclined side edges as slanted cylinders with horizontal round sections (still a quadratic),
   its horizontal edges as flat stadium slabs: about 80 lines, untried. (Capsule: the same with sphere-swept edges,
   F4.)
2. The floor and the edge (F11): the bounds as solid, held in the sweep beside the tree (floor walkable; sides by
   MOVE-RULES row 8; top a ceiling). **GROUND, an open point with an owner (engine-architect, the map kind):** no
   document says the bounds' bottom is ground. The map kind calls the bounds "the playable box" and the regions
   "patches of ground that are only drawn" (map.rs:3-5); the page draws a floor at y 0 (engine-architect-read N10)
   and netcode-architect's aim falls back to the plane y = 0 (netcode-architect-read 1.1); the test scene has no
   shape under the soldier. Options: (a) the map kind says the bounds' bottom is solid ground (a doc line and a test);
   (b) maps hold a ground shape (the test scene gains one; 512 shapes still the cap).
3. "Touching and moving along or away is no hit" (dot(normal, move) >= 0): without it nothing moves (F14 (1)).
3b. "A box whose top is no higher than the foot plus the skin is floor to a horizontal sweep": both prototype bodies
   ran with it (without it the round foot rides every edge). A per-box top test cannot work for a ramp (its top is
   high where the slope under the foot is low): the ramp's rule, and its seams with a platform at x 45, are open.
3c. The pieces are computed relative to the body's centre (corners minus centre first), so the numbers stay small at a
   2 km map's corner (PM-0004-39: f32's step there is 0.12 mm against the 2 mm skin; netcode-architect-read 4.3's
   plant is the skin below that step).
4. The closest hit over every shape whose grown box the move enters, ties to the lower index, exactly as `cast_ray`
   (collision.rs:329-333); the walk prunes a node whose box grown by the body is entered later than the best hit.
5. A skin of 2 mm (a code constant); a move that starts inside a shape (a soldier pushed or spawned into one) is
   pushed out by our own closest point (for a box a clamp; for a ramp the prism's planes), never by the library's EPA
   (it allocates: F3, contact_support_map_support_map.rs:64-67).
6. The normal, the feature (face or not) and the shape's index in the hit.
- Symbols, route Ours: in collision.rs add `pub struct Body { radius: f32, half_height: f32 }`, `pub struct SweepHit
  { shape: u32 (u32::MAX for the bounds), t: f32, normal: V3, face: bool }`, `pub fn StaticWorld::sweep(&self, body:
  &Body, centre: V3, motion: V3, counts: &mut QueryCounts) -> Option<SweepHit>`, `pub fn StaticWorld::push_out(&self,
  body, centre) -> V3`, private `enter_grown_box`, `enter_vertical_cylinder`, `enter_slanted_cylinder`,
  `enter_grown_ramp`; `StaticWorld` gains `bounds: Solid` (build takes it from `map.bounds`, today unused there:
  collision.rs:212-254); `pub struct QueryCounts` (1.9; Block 2). The library stays in this one file (the scan,
  rules_scan.rs:95-96). Library route: the same public symbols, built on `Cylinder::new`, `Cuboid::new`,
  `ConvexPolyhedron::from_convex_mesh` (one per solid, built in `build`, stored in a `Vec` there), `Pose`,
  `ShapeCastOptions { target_distance: SKIN, stop_at_penetration: false, .. }`; ties need every candidate cast (no
  early exit), and an `OutOfIterations` status taken as a hit.
- Tests: route Ours against the library's cast on the same pairs within 0.1 mm (the pattern of collision.rs:886-953);
  the prototype's cases as unit tests (0.55 / 0.56 at three speeds and three starts; the tie in both list orders; the
  touching rule); a seam: ramp to platform (x 45, F11) crossed at sprint with no stop; a move that starts inside;
  the bounds. Plants: no touching rule (everything stops: red); edges of a capsule counted by their normal (the 0.56
  step is gained: red, the prototype's case (3)); a tie to the higher index (red in one order); a sweep that collects
  its candidates into a `Vec` (PM-0004-6 red).
- Marks: PM-0004-30's sweep half, -35 (reading above), -20 (reading above), -6's sweep half; -32 unchanged (the tree).
- Count, route Ours: 260 / 260; route Library: 140 / 220 (and 1.8's sqrtf entry, and 1.9's tie cost).

### 1.5 The movement (`crates/sim/src/movement.rs`), one command at a time, in slot order
Each command that runs is one movement step of one tick (netcode-architect 3.3: once per command, none in a tick
without one). In order, all f32, + - x / sqrt and comparisons:
1. Wish: (move_x, move_z) from the command's integers (1.6), `as f32`; zero stays zero, else divided by its length
   (plan 6.1: always length 1).
2. Sprint and stamina (the old game's rule, game-designer F11): wants = sprint button held and a wish (and, in 4d,
   not aiming); while wanting and not locked: stamina -= drain (saturating), sprinting; at 0: locked. Otherwise:
   stamina += refill up to full; locked ends at `returns_at`. In the air and against a wall too. Integers only;
   `since_sprint` counts units from the end of a sprint (saturating), hashed (plan 10).
3. Speed: walk or sprint (ads in 4d). Velocity, horizontal: v += (wish x speed - v) x share, share = ease_up with a
   wish, ease_stop without, times air_share in the air (row 8). A speed under 1e-4 m/s is set to exactly 0 (a code
   constant: the state settles at rest and no subnormal number is carried tick after tick).
4. Jump: requested = jump held or a press edge since the last applied command: the simulation's own
   `new.wrapping_sub(old)` on the record's counter (sim depends on content and the collision library only, so not
   `protocol::wrap::press_edges`, wrap.rs:32-34, which is the same one line); if grounded: vy = take_off, grounded = false; every other press of that command is dropped.
5. Vertical, the exact arc: dy = vy x dt - half_g_dt2; vy -= g_dt (two roundings each, never fused: MUL-ADD). The
   tick's true top, when vy > 0 and vy - g_dt < 0: y + vy x vy x inv_two_g (F7); it, not the sampled height, is
   what the step-up in the air reaches.
6. Horizontal move with up to four slides: at each hit, move to it, remove the rest's part along the normal; a hit
   that is no walkable face (a wall, an edge) is flattened to the horizontal first.
7. The step-up (up by step_up + skin, across, down), on the ground and in the air (plan 6.2 (4): confirmed, with the
   true top of item 5 as its base in the air and BODY-SHAPE's (A) or (B): the plain capsule fails it, F14 (3)).
   Accepted if the down phase lands on a walkable face and got further than the plain slide (with body (A); with a
   capsule this rule misses the 0.55 step at walk and sprint, F15, and the edge-landing rule gains the 0.56, F14).
8. Vertical sweep by dy: falling onto a walkable face lands (vy = 0, grounded); rising into a ceiling stops vy (row 8).
9. Snap down (row 4 (a)): grounded before, not jumping, no ground found in dy: sweep down by step_up; a walkable face
   snaps; else airborne from rest (the old game's walk-off).
10. Walkable: a face whose normal's horizontal part is at most grade x its vertical part, squared: (nx^2 + nz^2) <=
    s^2 x ny^2 with s the steepest grade (NO-SOURCE; no trigonometry, no root).
11. The map's edge (row 8): the bounds as walls in the sweep (the speed into them dropped), or (c) a clamp 1 m inside
    with the speed kept.
12. Facing: d = aim.xz - position.xz; if d.x^2 + d.z^2 > 0.0004 (the old game's 2 cm, code constant): facing =
    d / sqrt(..); else kept.
After every soldier moved (authority only): the soft push (START-RULES). Pairs i < j in slot order, both occupied and
alive: horizontal distance under push_reach, vertical spans overlapping (NO-SOURCE's height gap); overlap share x
push_share, half to each, along the line between them (equal centres: a fixed direction by slot order); sums per
soldier into a fixed array of MAX_SOLDIERS (no allocation), from the moved positions, then applied in slot order
through a horizontal sweep (never into a wall). The four guards (sideways turn, at most half a walker's own step, a
cap, the height gap) are NO-SOURCE's numbers; their formulas are the sandbox's (push.ts:10-17 per game-designer
F20), which this read could not open: the main session supplies those lines (the branch is not in this clone).
Debug and test builds assert every float of the state finite after the tick (PM-0004-24).
- Symbols: `pub fn move_soldier(rules: &Rules, world: &StaticWorld, soldier: &mut Soldier, command: &PlayerCommand,
  last: &PlayerCommand, counts: &mut QueryCounts)` (no `&World`: cs7's prediction calls the same function,
  engine-architect 2.1); `pub fn push_apart(rules, world, soldiers: &mut Soldiers, counts)`; `pub fn
  jump_reach(rules: &Rules) -> f32` (PM-0004-59: steps the same arc, returns the true top + step_up).
- Tests (sim, deterministic) with their plants: yard 0.55 stepped / 0.56 stopped at walk, sprint and 13.27 m/s (plant:
  the step's down phase that accepts an edge: red); ledges 3.05 gained / 3.10 refused in the air and 2.90 gained
  (plant: the step-up tested against the sampled height while the content reach is tuned so the sampled top falls
  3 mm short: red); 10,000 seeded moves at 13.27 m/s through the 0.2 m wall, 0 past its mid-plane (plant: a move as
  a position step without a sweep); the gentle and steep ramps up and down with no hop (plant: no snap: a hop
  measured); the alley (1.5 m) passed with radius 0.4; stamina: 7 s of sprint from full, 5 s refill, sprint back at
  30 % (plant: refill while locked off); J1 to J5 of netcode 4.1 (3B-ONE-PER-TICK; plant: a jump per press); the push:
  two soldiers walking into each other are never stopped and end apart, none ends in a wall, the same result with
  the slots swapped in the list (plant: a push that blocks; a push applied in place in pair order, which differs when
  the slots swap); the facing 2 cm rule; `jump_reach` = a scripted jump's true top + step_up within 1 micrometre and
  = `jump_reach_m` within 1 mm (plant: the old game's order of the two sums, 180 mm off: plan 9's DEPART-4 reading).
- Marks: PM-0004-30, -59, -6 (1.7), -24, START-RULES' two rules, 3B-ONE-PER-TICK's movement half.
- Count: 430 / 420 (engine-architect 380 / 350; my design 570 / 600 counted the marks glue and the parry adapter).

### 1.6 The world, the command and their callers
- `world.rs`: `PlayerCommand` (:40-43) gains `move_x: i8, move_y: i8, aim: AimCm` as netcode-architect-read.md 1.1
  has it (`AimCm { x: i16, y: u16, z: i16 }` in a new `aim.rs`, the wire's widths, with the record kept as buttons and
  presses, its option (i)); agreed, and my earlier i32 withdrawn: the far-corner test feeds aim 0, which the movement
  does not read (netcode-architect-read 4.3; only the facing reads it). No float enters from a command, so PM-0004-24's "refused where it
  enters" holds by type, and PM-0004-60's fuzz covers integer ranges. `World` (:112-117) gains `soldiers: Soldiers`,
  and, as match constants outside the state, `rules: Rules`, `statics: StaticWorld`, `spawns`; `World::new` (:122)
  takes them (it can no longer be `const fn`); `authority_step` (:149) runs each slot's commands then the push;
  `visit_shared` (:177) writes the soldier rows and the move and aim of the record; `SHARED_STATE_BYTES` (:18) becomes
  a function of the slot count, or the fixed 48 rows. `StateSink` (:57-67) gains `f32` (bits, little-endian):
  implementors to change: `Canonical` (world.rs:89), devtools `Lines` (replay.rs:181), the two test sinks
  (world.rs:314-371).
- Callers to change (F12): room/src/lib.rs:105-108 (`Room::new` takes the match's constants; 4B-WORLD-ROOM: the static
  world moves from `LocalMatch.world`, local_match.rs:50, into the room's `World`), :219 and queue.rs:137
  (PlayerCommand literals), :282 (authority_step), :286-296 (player() into OwnState); sim-wasm lib.rs:83-122 (the
  constructor builds the constants), local_match.rs:58-64 and :98 (`static_world()` reads the room's now),
  tests/step_alloc.rs:76; sim/tests/state_hash_alloc.rs:70-94; devtools/tests/hash_cost.rs:40; room tests :314,
  :651, tests/any_bytes.rs:117. Callers of the two constructors that change (grep `Room::new(` and `LocalMatch::new(`
  over crates/): room/src/lib.rs:314 and :655, room/tests/any_bytes.rs:117, sim-wasm/src/local_match.rs:66 (the one
  product call of `Room::new`), :213, :399, :413, sim-wasm/src/lib.rs:122, sim-wasm/tests/step_alloc.rs:86; the grep
  finds no call in devtools. Remove: `LocalMatch.world` and its doc
  (local_match.rs:48-50).
- The pins that move in step 1, not in step 6 (F12): world.rs:401-415's known answers (a second implementation, a
  script, writes them again) and the committed first recording (crates/devtools/recordings/first-loop.*, its
  manifest): the moment the state holds a soldier their hashes change, so the step that changes the state re-makes
  them, as its own named sub-step (the advisor agent's ruling, plan 6.2), or CI is red from step 1 to step 6. This
  agrees with netcode-architect's own rule ("in the commit that changes the simulation's behaviour or its state's
  layout on purpose, every recording is re-made", netcode-architect-cs4.md 1.4, line 155) and its read
  (netcode-architect-read.md:403: "first-loop moves in step 1's commit"); the conflict is with plan section 6's step 6
  line, which the sheet corrects.
- Count: 110 / 70 (sim) plus about 60 / 40 in room, sim-wasm and devtools.

### 1.7 No trigonometry in the step; nothing allocates (PM-0004-6)
- Trigonometry: none in items 1 to 12 or the push (the walkable test squared, the facing a vector, F8's grades). The
  scan refuses std's; the built-file check refuses any C-named one (BANNED_MATHS, build-checks.mjs:717) and lists any
  libm one against the reviewed list.
- Allocation: the soldier table and the push sums are fixed arrays; the sweep tests leaves inside the tree's walk,
  which keeps its stack inline up to 32 (bvh_traverse.rs:119-122; MAX_TREE_DEPTH 31 holds it, collision.rs:38); route
  Ours has no library contact query, so no EPA. Proof: a counting allocator around `authority_step` in
  `crates/sim/tests/step_alloc.rs` (new, on the pattern of state_hash_alloc.rs) over 1,800 ticks of 12 scripted
  soldiers walking, sprinting, jumping, stepping, pushing, with planted starts inside shapes; and sim-wasm's existing
  tests/step_alloc.rs:65 on the moving local match. Plants: a 64-byte allocation per tick in the step; a sweep that
  collects into a `Vec`; (library route) a cast on overlapping shapes.
- Count: 0 / 90.

### 1.8 The maths routines that join, and the built-file check's reviewed list
- **exp, once per match** (the two easing shares; and the push share if the sandbox's push is exponential). libm
  0.2.16's `exp` is pure Rust on x86-64 and wasm32, with no fused multiply-add (F1). Joining it costs: `libm =
  "=0.2.16"` as a direct dependency of `sim` (root `[workspace.dependencies]`, crates/sim/Cargo.toml; a lockfile edge:
  the DEP-HOLD batch, security-auditor's answer first; libm is already audited in 4b); the scan's one-file exception
  (1.3; F13); an entry `libm::math::exp` in packages/client/scripts/maths-allowed.mjs with its reason; vectors.
  Alternative with no dependency change: our own exp from + - x / (about 25 lines, deterministic by construction),
  against ADR 0004 part 7's "transcendental maths only via pinned libm" (engine-architect N45): a departure. Named
  as a choice; recommended: libm, as the plan has it.
- **The arc tangent is not step 1's**: the simulation keeps facing as a vector (netcode-architect 3.3); yaw and pitch
  for the block are the client endpoint's (step 3; or the page's `Math.atan2`, netcode 3.5's cheaper option, not a
  simulation value). The reviewed list gains `libm::math::atan2f` only if step 3 takes the endpoint route.
- **FACING-SPAWN (F5), an open point with an owner (engine-architect, the map kind):** a spawn faces `facing_deg`,
  any degree (map.rs:66). Options: (a) libm `sin` and `cos` once per spawn point at match start (two more routines,
  two more entries and vectors); (b) the map kind takes quarter turns only and the simulation a four-row lookup (a
  content change to 4b's kind); (c) the soldier keeps the spawn's degrees until his first aim and the endpoint draws
  it (two representations of one field). Every spawn in today's scene faces 90.
- **sqrt**: route Ours calls f32 `sqrt` only, which is the instruction (no routine, no entry). Route Library links
  glam's length, libm `sqrtf` (soft, exact): one more entry.
- Vectors (PM-0004-22): `crates/devtools/vectors/make_f32_vectors.py` (or a sibling) gains exp: 10,000 f64 inputs over
  the domain the shares use (-ln10 x dt / T for T in (0, 5], and the push's) plus edges (-745, -0.0, the smallest
  normal), outputs bit-identical natively, in Node and in Chromium through the test-only module
  (crates/devtools/src/wasm.rs), and within 1 ulp of Python `decimal`'s correctly rounded exp; plant: one output moved
  one ulp on one target. The helpers' signed-zero rows join the same files.
- Count: 90 / 50.

### 1.9 The same results natively and in WebAssembly
What holds it: IEEE + - x / and sqrt (correctly rounded on both, never contracted by Rust; MUL-ADD's ban by the scan
and the built-file check); libm's exp pure Rust; stamina in integers; ties by index in every query; slot order for
the movement and the push, sums applied after (order-free); no hash map; no clock; the derived constants worked out
once, in f64, narrowed once. What proves it: the movement recording on four runs (PM-0004-19, -20, -21; step 6); the
vectors (1.8); and a committed sweep proof beside static-world.txt (Block 2). Count: in Block 2.

### 1.10 Step 1 recounted
| Piece | engine-architect-cs4.md 2.10 | This read, route Ours | Route Library |
|---|---|---|---|
| ids, soldiers | 150 / 120 | 150 / 140 | same |
| rules (and time, which 4b built) | 120 / 80 | 130 / 110 | same |
| vec, fmath, scan | 40 / 60 (fmath only) | 70 / 80 | same |
| sweeps | 260 / 150 (collide.rs) | 260 / 260 | 140 / 220 |
| movement and push | 380 / 350 | 430 / 420 | 400 / 420 |
| world and callers | 120 / 60 | 170 / 110 | same |
| allocation test | 0 / 80 | 0 / 90 | 0 / 100 |
| exp and vectors, list | (in its 3A rows) | 90 / 50 | 100 / 60 |
| **Step 1** | plan: **1,240 / 1,090** | **1,300 / 1,260** | **1,160 / 1,240** |
Plus, outside sim, NO-SOURCE (b)'s fields in the content crate and the Testlines table (Block 3): about 40 / 40.
Plus Block 2's ray counts and sweep proof: about 90 / 70 (in devtools; step 1 or step 6, the sheet's call).
**With both: about 1,430 / 1,370 (route Ours) or 1,290 / 1,350 (Library), against the plan's 1,240 / 1,090.**

### 1.11 The three marks that ride on step 1 but are tested in later steps
- **PM-0004-54** (1,000 downward cursor rays onto the yard's ramps within 1 cm of the slope; plant: ramps queried as
  boxes, the old game's GC3): its test is step 2's, on netcode-architect's `aim_point` (aim.rs) over today's
  `cast_ray` and `enter_ramp` (collision.rs:152-204, 300-338). Step 1 provides nothing new for it: the ray exists and
  is exact on ramps (the library-twin test, collision.rs:886-953, within 0.1 mm).
- **PM-0004-39** (the yard at a 2 km map's far corner; plant: the skin below f32's step there): its test is step 6c's
  (netcode-architect-read 4.3, `crates/sim/tests/far_corner.rs`, a map built in code, aim 0). Step 1 provides the
  sweeps computed relative to the body (1.4 item 3c), a skin of 2 mm (16 f32 steps at 2 km), and no constant that
  assumes coordinates near the origin. On the plan's ladder (moves to 4d at 13,150 lines).
- **PM-0004-60** (2,000,000 hostile movement commands, no panic; plant: a client-sent number used as an index): its
  tests are step 6c's (netcode-architect-read H1 to H3). Step 1 provides: a command of integers only (no float to be
  NaN), the deny lints on the new modules (1.3), `get` indexing everywhere, saturating and wrapping integer
  operations only where written, the wish's zero-length check, and the finite assertion each tick.

## Block 2. 4B-RAY-COUNTS, 4B-RAY-WASM, and the simulation side of 4B-QUERY-RECORDING (DONE)
**Today.** `crates/devtools/src/world_proofs.rs` casts 2,000 rays into each of 12 seeded maps; `ray()` (:97-150)
picks one of five kinds by `rng.between(0, 5)`: 0 straight down, 1 along a horizontal axis, 2 a slanted integer
direction (its parts drawn from -3..3 and -3..0, so a part, or all three, can be zero), 3 steeply down with a
tenth-step tilt, 4 straight down the plane of a shape's own x face. The committed line (static-world.txt) holds only
the total met (`rays 403`, :463-469) and one digest: a change that starved a kind, or made every ray of a kind miss,
moves the digest once and is then committed as the new truth. That is TE-4's gap.

**4B-RAY-COUNTS: the design.**
- `ray()` returns its kind beside the ray (`enum RayKind { Down, Axis, Slanted, Steep, FacePlane, Camera }`).
- `made()` writes, per map, the cast and met count of each kind, and five counts of what the answers were: met a ramp,
  met at t = 0 (from inside), met with a zero part in the direction (TE-4's -0.0 case included), a tie (two shapes at
  one t: counted by a second, plain search of every shape, already present in `judge_taken`'s style, :232-258), met
  at the limit. Format version 2 of static-world.txt (its first line, :437-441), made again.
- A test in crates/devtools/tests/static_world.rs holds a minimum per kind and per answer count, per seeded map with
  at least 17 shapes and over the file. The numbers are set at about half of what the first run with the kinds gives
  (not measured in this read; the test names the run they came from), so a seed change passes and a starved kind
  fails. Ties: `judge_taken` already casts one ray per shape into every hostile map and holds the tie rule there (the
  lowest index of those as high, :264-268; the row map `hostile(6)`, :297-306, is built for it); what is missing is a
  committed count of those ties, which the hostile line (:423-431) gains. Plant: `rng.between(0, 4)` in `ray()` (kind 4 never drawn): red on its minimum; a cast that misses every
  ray along an axis (`direction[axis] == 0.0` returning None): red.
- **The casts' first caller** is the client endpoint's aim (step 2, netcode-architect 3.7: from the camera, down onto
  the scene, first hit within 500 m). Its rays are a kind of their own: origin 10 to 40 m above the ground, pitched 35
  to 75 degrees down (drawn as integer rise over run, no angle), any heading: kind `Camera`, with the same minimums.
  Step 2's tests then use the committed kind instead of hand-made rays.
- **4B-RAY-WASM** (in the same file): one committed ray of each of RR-1's three paths (beyond the reach, a limit cut to
  it, a ramp whose numbers overflow) in the list that runs in WebAssembly through the test-only module
  (crates/devtools/src/wasm.rs), so the three paths are compared natively and in Node and Chromium, not natively only.
- Count: 45 / 35.

**4B-QUERY-RECORDING, the simulation side.** The first queries the match's step makes are the movement's sweeps
(step 1); the aim's ray runs in the endpoint (step 2) on recorded inputs. So:
1. **The state's hash carries the sweeps' results** (positions and velocities by their bits, 1.6): a sweep that
   differs between targets shows at its tick in the recording's per-tick hash and in the field-level diff (PM-0004-21).
2. **Query counts, outside the state.** `QueryCounts` (collision.rs, 1.4) counts, per kind: sweeps that met a box face,
   a vertical edge, a ramp's slope, a ramp's edge, the bounds, nothing; touching-and-leaving (no hit); starts inside
   (pushed out); ties; step-ups tried, taken, refused; snaps; landings; pushes; rays. Wrapping u32 additions in the
   scratch the step already owns; never hashed, never in the block, no allocation; read through a `#[doc(hidden)]`
   accessor on the world and on the test-only module.
3. **The movement recording (step 6) commits its counts** in its manifest and the replay compares them natively, in
   Node and in Chromium (equal counts: the same paths ran on every target, a second signal beside the hash), with a
   minimum per kind (a route edit that stops exercising the 0.56 refusal or the ramp's edge fails). Its scripted route
   must hold each kind: the box face slide (thin wall at sprint), a vertical edge (the posts), the 0.55 step taken
   and the 0.56 refused, both ramps up and down with the snap, both ramp-to-platform seams, the 3.05 ledge gained and
   the 3.10 refused in the air, the alley, the bounds' edge, a landing, and two soldiers pushing beside a wall.
4. **A committed sweep proof** beside static-world.txt (world_proofs.rs gains `sweeps`): 2,000 seeded sweeps per
   seeded map, by kind (horizontal, down, up, slanted, from inside, touching), each answer's shape, t and normal by
   their bits into one digest per kind, with the per-kind minimums above; native lines committed, the test-only
   module's lines compared in Node and Chromium. This is PM-0004-32's query half for the sweeps, independent of the
   recording, so a sweep difference is found without replaying a match. Plant: on the WebAssembly
   build only (cfg-gated, the 4b pattern for the tree), a tie that goes to the higher index, and the skin one ulp
   wider: each red in Node. (A reversed candidate order would not bite: the answer is order-free by design; the same
   proof shows that as a control.)
- Count: 45 / 35 (counts 20 / 10, proof 25 / 25); the recording's part is netcode-architect's step 6.

## Block 3. What MOVE-RULES and NO-SOURCE change in step 1's code, by option (DONE)
Options as the question file has them: (a) every row as written; (b) the rows he names; (c) the old game's rule.
Under (b) each row takes its (a) or (c) cell below. Line counts are changes to Block 1's route-Ours counts.

| Row | (a): what the code does | (c): what the code does instead | Numbers it needs |
|---|---|---|---|
| 1 body size | radius = `body_radius_m` (0.4, his Export); height for headroom (NO-SOURCE) | radius 0.7 (the old game's code, game:2780): a value from outside his Export, so a NO-SOURCE-like row; no height (walls endless, row 5) | (a) radius from content, height a new field; (c) 0.7 a new field |
| 2 steps and edges | the body touches the step and rises (BODY-SHAPE (A): the cylinder's down phase lands on the step's top when the body overlaps it); stands past an edge by the foot's radius | rises only when the centre crosses the edge: the ground is a ray straight down from the centre (today's `cast_ray`), a step is taken when the ground under the centre rises by at most step_up; drops off as the centre passes | (a) a foot radius if smaller than the body (not in NO-SOURCE's list: add it, or take the body radius); (c) none |
| 3 walls | slide, the speed into the wall removed; round corners (the cylinder's side) | pushed out along the nearest side of the box grown by the radius, speed kept, square corners: the centre's ray against the grown box (today's `enter_box` on grown corners, collision.rs:114), no edge cylinders, the velocity untouched | none either way |
| 4 slopes and stairs down | snap down by step_up (no new number) | no snap: falls in small hops (the old game, R9) | (a) the steepest walkable grade (NO-SOURCE) |
| 5 real heights | every shape is solid at its own height (the code above) | walls and houses endlessly high, platforms, ramps and cover standable: the map kind has no field that says which shape is which (map.rs:104-118, F10): a content-kind change, engine-architect's, plus a flag in `Solid` | (c) a field per shape |
| 8 listed only | two shares and the air share (F9: same bits as (c) today); the head stops at a ceiling (vertical sweep); the bounds are walls, speed into them dropped | one share, full steering; no ceiling test; clamp 1 m inside the bounds, speed kept (game:2779) | none |
Under (b), rows 2, 3 and 4 share one body and one sweep model (the body's foot, the wall's corners and the slope's
snap all come from it): a mix such as row 3 (a) with row 2 (c) is not two independent cells but a third model to
design and test; rows 1, 5 and 8 mix freely.
Code size by option: all (a) is Block 1. All (c): the sweeps shrink to the grown-box ray plus the down ray (about
-170 / -150), the movement loses slides, snap and ceilings (about -60 / -60), row 5 (c) adds the content field and its
reader (about +30 / +30 in `content`, engine-architect's). Rows 6 and 7 are 4d's and change nothing in step 1.

**NO-SOURCE under option (b): the fields, as content rows marked "not from your Export".** Names in the movement kind's
style (crates/content/src/movement.rs:18-34), units per second where the sandbox counts per step (it runs at 60
steps a second, STEP-60), bounds wide enough to keep a typing mistake out and no tighter. No starting values here:
they are the sandbox's code constants, which this read cannot open (the branch is not in this clone); the main session
supplies them with their file and line.
| Field | Unit | Bounds | Needed under |
|---|---|---|---|
| `body_height_m` | m | [1.0, 3.0], and at least 2 x `body_radius_m` (refused by `Rules::take`) | row 1 (a) |
| `steepest_slope_pct` | % (rise per 100 of run: the yard's ramps are 25 and 50, the roofs 89 and 95, F8) | (0, 1000] | rows 2 to 4 (a) |
| `foot_radius_m` (if the foot is not the body: BODY-SHAPE (A)) | m | (0, `body_radius_m`] | row 2 (a); not on the question's list |
| `push_sideways_pct` (how far a head-on push turns sideways) | % | [0, 100] | the push (START-RULES) |
| `push_undo_at_most_pct` (a push never undoes more than this share of a walker's own step) | % | [0, 100] | the push |
| `push_most_m_per_s` (the most a push may move him) | m/s | (0, 20] | the push |
| `push_height_gap_m` (above this the two do not push) | m | [0, 3] | the push |
The slope in degrees instead of a grade would need a tangent or cosine once per match: a third maths routine on the
reviewed list with vectors (owner: the sheet; recommended: the grade, which needs none). What the fields change
beyond `sim`: `FIELDS` grows from 15 (movement.rs:18) and `Movement` with it, so every movement file, the test text
(:107), the schema and the generated types change; `content/source/testlines-table.json` gains rows with a `field`
and no `export`, a row shape today's table does not have (its rows hold `export` with `field`, or `export` with
`must_be`), so packages/client/scripts/testlines.mjs's `--check` must take it, with a planted row that claims an
`export` it does not have. Under NO-SOURCE (a) the same fields come from a new Export (a `content/source/` change,
risk tier 1 with his go) and the table rows carry their export names: the simulation's code is the same either way.

**What can be built before his answers without building anything twice:**
1. The soldier table and ids (1.1), with PM-0004-15.
2. The vector and the helpers, and the scan's changes (1.3).
3. `Rules::take` for the fifteen fields that exist, with its refusals (1.2); the new fields join when he answers.
4. The vertical part: the jump rule (J1 to J5), the exact arc, the true top, `jump_reach` and PM-0004-59, and landing
   on the bounds' floor (no shape needed: F11).
5. Stamina and the sprint rule (the old game and the sandbox agree, game-designer block 3), wish, speed, easing (row
   8's two shares cost nothing to keep or drop, F9), facing.
6. The sweeps' common part under every row: the grown-box ray (it is row 3 (c)'s whole wall test and part of (a)'s),
   the touching rule, ties, the bounds, the candidates' walk, the counts, and Block 2's ray kinds.
7. The world's command and state changes, the callers, the known answers and the first recording re-made (1.6).
8. exp's vectors and the reviewed list's entry (1.8), once the DEP-HOLD batch for `libm` is answered.
Waits for MOVE-RULES rows 2 to 5 and BODY-SHAPE: the cylinder's edge pieces and the ramp's pieces, the slide's speed
rule, the step-up and its in-air form, the snap, the walkable test. Waits for NO-SOURCE: the slope's number, the
height, the foot, the push's four guards (the push's core, pairs and sums swept against the world, can be built on
the two numbers that exist, with the guards off).

## Block 4. Risks, ranked (DONE)
1. High: the plan's capsule fails PM-0004-30 under every step rule tried (edge landing: 0.56 gained 9 of 9; face landing: 0.55 missed 6 of 9; base memory: 0.56 perched on its edge 9 of 9); BODY-SHAPE must be settled, with game-designer and him, before the movement step.
2. High: the prototype ran horizontal moves against boxes only: ramps (the grown prism, the slanted side edges, the floor rule for ramps, the seams at x 45), every vertical motion (the in-air step-up with 3.05 gained and 3.10 refused, landing, the snap, ceilings) and the push are untried, for every body and both routes.
3. Medium: the cylinder floats over a ramp by radius x grade (0.10 and 0.20 m at the yard's ramps), which he will see; a separate foot radius is a number no question asks for yet.
4. Medium: route Ours is a departure from the card's "on parry3d's queries" and rewords PM-0004-20 and -35; the advisor agent must accept it or the library route runs, with its EPA allocation on overlap and GJK-tolerance ties (F3).
5. Medium: the first recording and world.rs's known answers change the moment the state holds a soldier; if their re-making waits for step 6, CI is red from step 1 on.
6. Medium: libm as a direct dependency of sim reopens the dependency hold and needs the scan's one-file exception; our own exp instead departs from ADR 0004 part 7.
7. Medium: spawn facing in any degree needs a sine and cosine (FACING-SPAWN), outside "the two routines that join".
8. Medium: content ranges admit a reach below the step-up and durations that round to 0 units; `Rules::take` must refuse them or a NaN or a stuck bar enters the match.
9. Medium: the push's formulas and four limits are the sandbox's, unread here; a per-step share taken at 30 Hz from a 60 Hz sandbox halves or doubles the feel.
10. Low: PM-0004-59 with his 3.071 m holds by 0.17 mm when measured on sampled heights; the tests must measure the true top and compare to `jump_reach_m`.
11. Low: the steepest-slope number decides whether he walks up the roofs (89 % and 95 %) after gaining their 2.6 m edge (row 5).
12. Low: the u16 generation wraps after 65,536 reuses of one slot (an old id resolves again): stated, tested, accepted.
13. Low: without the 1e-4 m/s rest rule a stopping soldier's speed never reaches 0: v - v x 0.37375 stops changing once v x k rounds to nothing, at the smallest subnormals (about 190 ticks in), so the state never settles and x86 runs those ticks slowly (arithmetic, not run).

## Block 5. The sandbox's own body, run (the coordinator's follow-up, 2026-11-28) (DONE)
**What ran.** `rr-body-proto.rs` beside this file (rustc 1.99.1 -O, f32; outputs `rr-body-proto-3a.txt`, `-3b.txt`,
`-2.txt`, `-3x.txt`; sha256 of the source 8ba37c11...4b14e). It is the sandbox's body as game-designer-read.md block 1
reads it (be608cad), in the sandbox's own 2.5D form: a 0.4 m body circle for walls and the step trigger (a top above
the feet and at most step-up + 5 mm above the test feet, while moving toward it: the rise is instant), a 0.15 m foot
circle for standing at the height nearest the centre, the snap down 0.2 m + (move + 0.15) x slope, else a fall from
rest; the exact arc with the tick's true top as the test feet in the air (block 1, 1.5 item 5); the test scene's 17
shapes of the yard and the street. Bodies: **(3a)** foot plus the pull (0.5 of what is left a tick, all of it under
0.03 m, to 0.9 x foot); **(3b)** foot plus the full body on the step just taken until the foot reaches it; **(2)** my
cylinder in the same 2.5D form (stands on the highest point under its 0.4 m disc, snap = step-up, no 5 mm); and, to
check the model against game-designer's N5, **(3x)** the foot with neither bridge. Runs: three speeds (6.3, 9.576 and
13.27 m/s) x three starts, as before; plus slow walks (1 and 2 m/s, the easing's first ticks) and walks that stop the
tick the step fires. Pass needs the outcome and no fall without a jump ("unjumped falls", the flicker).

| Case (runs) | (3a) | (3b) | (2) | (3x), check only |
|---|---|---|---|---|
| 0.55 m step gained (9) | 9 pass, 0 falls | 9, 0 | 9, 0 | 1 pass: 8 runs fall once for 1 tick after the rise |
| 0.56 m step stopped on the ground (9) | 9 (x 33.6000 = face - 0.4) | 9 | 9 | 9 |
| slow walk on, 1 and 2 m/s (6) | 6, 0 falls | 6, 0 | 6, 0 | 0: 3 to 8 falls a run |
| stop the tick the step fires (9) | 9: stands on the top, centre pulled to 0.135 m short of the face | 9: stands on the top, centre 0.21 to 0.40 m short | 9: same as (3b) | 0: drops back to the ground |
| 3.05 m ledge caught by a jump (9) | 9, top at tick 12 or 13, 0 falls | 9 | 9 | 6: 3 runs fall once after catching |
| 3.10 m ledge refused (9) | 9 | 9 | 9 | 9 |
| ramps 25 % and 50 %, up and down (18) | 18, 0 airborne ticks | 18 | 18 | 18 |
| float on the open slope (centre over the ramp, a body radius from its ends) | 0.000 m | 0.000 | 0.100 (25 %) and 0.200 (50 %) | 0.000 |
| float at a slope's seam (the body touches the platform or the ridge and rises at once) | up to 0.18 m (ramps), 0.31 (roofs), for 1 to 2 ticks | the same | 0.10 / 0.20 / 0.36 / 0.38 throughout | smaller (it falls back) |
| roofs 95 % and 89 % after the 2.6 m edge (9 each) | edge gained and the ridge stood on in 18 of 18 | 18 | 18; on house b at 13.27 m/s all 3 runs fall once (1 tick) at the roof | 18; 6 runs fall once |
| walking off an edge | falls once the foot leaves it: centre 0.21 to 0.33 m past (0.15 plus up to one tick's move); falls from 0.55 and 1.0 m | the same | centre 0.52 to 0.65 m past (0.4 plus a tick); snaps down the 0.55 m step (snap = step-up), falls from 1.0 m | as (3a) |
| thin wall 0.2 m at 13.27 m/s (10,000) | 0 past its mid-plane | 0 | 0 | 0 |
Reading: (3x) reproduces game-designer's N5 (the flicker without a bridge), so the model behaves as its reading says;
(3a) and (3b) pass every run of every case with no fall; (2) passes every pass/fail case, and flickers once (1 tick)
on house b's roof in the 3 runs at 13.27 m/s. (3a) and (3b) give the same numbers in every walking run at a steady
speed (the walk alone carries the centre within 0.135 m of the face in one tick at these speeds, so the pull ends
without moving him); they differ only when he stops at the step: (3a) is pulled onto it, (3b) stays where he stopped,
standing on the step with his centre up to 0.4 m short of it. The roofs are walked to the ridge by every body because
the sandbox has no slope limit (game-designer N3): that is a NO-SOURCE and MOVE-RULES row 5 matter, not a body one.
**Not run, said plainly:** the sandbox itself (this is a re-implementation of a reading); easing (steady speeds, with
the slow runs standing in for its first ticks); ramps approached from their sides, and the sandbox's "set back along
the slope" (yard.ts:318-334); ceilings; the push; two soldiers; the far corner. The wall rule is a push-out after the
move with no sweep: 0 tunnellings at 13.27 m/s through 0.2 m holds by 6 cm of margin (0.44 m a tick against the
0.5 m from the nearest legal start to the mid-plane); a faster move or a thinner wall would pass through, so the
game's version sweeps the body circle against the footprints (a 2D form of 1.4's grown box and vertical cylinders).

**2. The pull at 30 against 60 steps a second.** The sandbox's pull is per step (half of what is left each step,
movement.ts:61-63), not per second, so its time depends on the rate: about 0.17 s at 30 steps, 0.08 s at 60.
His Export says 30 (game-designer N2: `step_rate_hz` 30, the sandbox's start rate 30) and his STEP-60 (a) is "I play
the sandbox at 30 steps and export again", so at our 30 ticks the three numbers stand as they are: nothing to work
out again. To keep it from depending on the rate in content: the share as a half time, `pull_half_time_s` = 1/30 s,
which the time unit holds exactly (100 units); the share a tick is 1 - 0.5^(units a tick / half-time units): exactly
0.5 at 30 Hz with no routine (a whole power); at 60 Hz 1 - 0.5^0.5 = 0.2929, through the easing's exp once per match.
The 0.03 m stop and the 0.9 x foot target are lengths and do not depend on the rate. Of the other numbers: the 0.2 m
snap and the 5 mm allowance are lengths (rate-free); the snap's slope term uses the tick's move, which is physical.

**3. What each body changes in step 1's counts (code / tests, against Block 1's route Ours 1,300 / 1,260) and in
NO-SOURCE.**
- The form: all three bodies run here in the sandbox's 2.5D form, which is exact for the map kind's only forms (boxes
  and four-way ramps, all axis-aligned). Built so, the sweeps become a swept body circle against footprints plus
  heights from footprints and the tree's candidates: about 170 / 200 in place of 260 / 260 (no edge cylinders in
  3D, no grown prisms). The 3D sweeps of 1.4 stay the route if a later form (a rotated box, a mesh) needs them.
- **(3a):** movement +30 / +50 (the pull, the stop-at-touch and slow tests, the flicker count), the step-up simpler
  (-20): step 1 about 1,220 / 1,250. NO-SOURCE: `body_height_m`; `foot_radius_m` (m, (0, body_radius_m]; 0.15);
  `pull_half_time_s` (s, [1/3,000, 1]; 1/30); `pull_done_under_m` (m, [0, 0.5]; 0.03); `pull_to_foot_pct` (%,
  [0, 100]; 90); `step_allowance_m` (m, [0, 0.05]; 0.005, or 0 by game-designer's "no allowance"); `snap_down_m` (m,
  [0, step_up_m]; 0.2). The steepest slope: none (the sandbox has none), so that row leaves NO-SOURCE under (3a).
- **(3b):** movement +15 / +40 (the stepped block stood on with the full body): about 1,205 / 1,240. NO-SOURCE: as
  (3a) without the pull's three numbers.
- **(2):** in 2.5D about 1,190 / 1,210; in Block 1's 3D sweeps 1,300 / 1,260. NO-SOURCE: `body_height_m`, and
  `snap_down_m` if he takes the sandbox's 0.2 m (my design snapped by the step-up, which drops him off the 0.55 m step
  at once, game-designer N8); the steepest slope only if a limit is wanted (the sandbox has none).
- Every body keeps 1.2, 1.3, 1.6 to 1.9 and Block 2 as they are; the in-air step-up and the true top (1.5 items 5
  and 7) hold for each (the ledges above).
BODY-SHAPE stays his; game-designer's condition for (3a) (every run passes, vertical motion, ledges and ramps
included) is met in this prototype, with the "not run" list above as its limit.

DONE

## Block 6. The bodies on the cases block 5 did not run (the advisor agent's item 10 point 2; 2026-11-28)
### Plan
1. New prototype `rr-body-proto-6.rs` (block 5's source kept as it is), same 2.5D model and same three bodies (3a),
   (3b), (2), plus: horizontal velocity with the easing (k 0.37375 a tick, up = stop, air 100 %) as an option; the
   soft push by game-designer-read block 3 (both wall rules: the sandbox's push-out, and a swept circle with one slide
   for the plan's sweep); made-map shapes (a beam over flat ground, a high drop onto a top); per-run counters for
   flickers (an unjumped fall that comes back up), bounces, landing tick and the per-tick drop.
2. Cases: ramps from their sides (gentle x 41..45 z 4..8, steep x 42..45 z 10..14, met along z at several x, both
   directions); ceilings on a made map (shown as cost only if the reading has no ceiling rule); landings off the
   2.9 / 3.05 m ledges and the platforms onto the ground and onto the 0.55 m step; the snap's numbers down the step and
   the ramps at three speeds; the push (head-on, glancing, one still, both still, into a wall, across the 0.55 step and
   at the 1.0 m gap, three at once, slot order swapped); easing's first ticks at the 0.55 / 0.56 steps.
3. The 2.5D question against map.rs:9-13 and :317 on (no resting rule).
4. The record rule (advisor-reread-4c.md:31) against F17 and 1.6.
5. Table, then DONE.

### Notes
- [B6-R, the record rule, written first] YES to the advisor agent's item 6 ruling (advisor-reread-4c.md:31). F17 (:29, option (i): the record stays buttons and presses, the soldier consumes move and aim) STANDS; 1.6's words "and the move and aim of the record" (:246-247) are WITHDRAWN: under (i) the record holds no move and no aim, so that phrase described option (ii). Why (i): world.rs:114 already says "the soldier table takes its place", so per-soldier held input belongs in the soldier row that scales to 48, and the record keeps what the echo and the jump's press base need. Under (i) the Soldier row (1.1) gains `held_move: [i8; 2]` (the newest command's move_x, move_y, kept unchanged by a tick with no command, as world.rs:143-145 keeps buttons) and holds the aim as received, `aim_cm: AimCm` (i16, u16, i16), in place of 1.1's `aim: V3` (6 bytes, exact; the metres are derived each tick by one IEEE division, the same on both targets, and are not state); `facing: [f32; 2]` stays. `visit_shared` writes: tick; the record's buttons and presses (as today, world.rs:177-189); then each of the 48 rows in slot order, every field including held_move and aim_cm. Tests: a command-less tick after a moving command keeps walking and turning (plant: held_move zeroed on a command-less tick: red); a world restored from the hashed shared bytes at tick n runs tick n+1 to the same bytes as the original (plant: held_move kept in an unhashed scratch: red).
- [B6-1] rr-body-proto-6.rs (rustc 1.99.1 -O, edition 2024, f32): with every new option off, its "b5" mode reproduces block 5's four outputs byte for byte (diff against rr-body-proto-3a/3b/2/3x.txt: identical), so block 5's table stands behind block 6. Body "2w" (body 2 with its wall height taken as its step height, the disc's highest point) on block 5's cases: identical except the house-b roof at 13.27 m/s, where block 5's one-tick fall (3 runs) is gone (0 falls).
- [B6-2] Body 2 as block 5 built it FAILS ramps met from the side in its band (gentle x 42.9 to 43.1, steep x 42.8 to 43.0: the height under the centre at most 0.55, under the disc's uphill point above it): neither the wall test (height_near) nor the step (height_max) fires, he walks into the ramp's solid at ground level, 3 to 6 ticks inside it, 108 of 306 runs. One line fixes it (2w: the wall test uses the same disc height): 306 of 306. (3a) and (3b): 306 of 306. A model defect, not a body property: the step test and the wall test must read one height.
- [B6-3] Landings, 240 runs per body (5 tops x 8 directions x walk-off/jump x 3 speeds, then 15 still ticks): 240 of 240 for 3a, 3b, 2, 2w: no bounce, no fall after landing, no flicker, never inside. Falls caught by the in-air step-up are outcomes of the reading's rule, not flickers (walking off the 2.9 ledge toward x 35 lands on the 3.05 ledge at 9.576 and 13.27 m/s; off the 0.55 step toward x 34 lands on the 0.56 step at every speed: the 0.56 step, refused from the ground, is gained by walking off the 0.55 one 2 m away). Largest drop in one airborne tick in the test scene 0.5533 m (onto the ground); deepest moved feet below a non-ground top in a landing tick 0.37 m (3a/3b), 0.43 (2): under the 0.55 m the landing test reaches, so the test scene never misses.
- [B6-4] High drop (made map, a fall from 10 to 18 m onto a 2.0 m top, 21 runs): 4 of 21 miss the top under the reading's landing (the feet end more than 0.55 m inside the top, the standing search no longer sees it, the push-out puts him on the ground beside the block), every body. The candidate fix (a top crossed inside the tick counts: the search reaches the start-of-tick feet) 0 of 21. Any fall above about 5.4 m onto a non-ground top can miss (0.55 = v dt + g dt^2 / 2 at v = 16.1 m/s): the reading does not say which feet the sandbox's surfaceAt uses, so this is my model's hole until the sandbox's code says otherwise; the fix is a few lines and needs no number.
- [B6-5] New counter on block 5's own ledge jumps (a body inside a solid): at 13.27 m/s, in all 4 runs per ledge, the body spends 3 ticks (2.9 m) and 1 tick (3.05 m) INSIDE the ledge's box while rising, then ends on top: block 5's PASS hid it (its pass rule had no inside test). Cause, the model's, every body: from the wall's push-out (centre at R = 0.4 m) a 0.442 m move puts the centre inside the footprint, so "moving toward it" (nearest point minus centre) reads 0 and the step-up does not fire, while the wall test lets a top within a step through. Same on the made map's 2.0 m beam at 13.27 (3 ticks inside, then over it). Candidate fix, one condition: judge "toward" from where the tick started: 0 inside in 36 of 36 ledge runs and on the beam; block 5's cases unchanged except the 3.05 ledge at 13.27 caught one tick earlier (tick 12, not 13); block 6's tallies unchanged. Whether the sandbox has this hole is not in the reading (its "while moving towards it", yard.ts:252, gives no formula).
- [B6-6] Ramps from the side, 306 runs per body (4 approaches x 8 or 9 x positions x 3 speeds x 3 starts): 3a, 3b and 2w 306 of 306; on the slope where the height at contact is within the step (3a/3b: under the centre, gentle up to x 43.1, steep up to 43.0; 2w: under the disc, gentle up to 42.0, steep up to 42.6), else stopped at the side 0.40 m out. The wall rule is block 5's push-out, a stand-in: the sandbox's "set back along the slope" (yard.ts:318-334) is NOT run, the reading gives no formula. Off a ramp's side (24 runs per body): falls once the foot leaves (3a/3b: centre 0.21 to 0.31 m past), snaps from 0.15 m and less; body 2 snaps up to 0.55 m; no flicker, none inside.
- [B6-7] Snap numbers (3a/3b, the reading's 0.2 + (move + 0.15) x slope): flat 0.2; 25 %: 0.290 / 0.317 / 0.348 at 6.3 / 9.576 / 13.27 m/s against a drop of 0.053 / 0.080 / 0.111 a tick; 50 %: 0.380 / 0.435 / 0.496 against 0.105 / 0.160 / 0.221; smallest margin measured down the ramps (seams included) 0.094 m (steep, 9.576): 6 of 6 down runs per body with 0 airborne ticks. Down the 0.55 step 0.55 > 0.2: a fall (3a/3b); body 2's snap is the step-up 0.55: snapped. Easing (k 0.37375 a tick, up = stop, air 100 %): 48 of 48 step runs from rest 0.05 to 1.2 m out, walking on or letting go the tick the step fires, and 24 of 24 ledge jumps from rest, every body.
- [B6-8] The push (14 cases x 2 wall rules x every slot order, per body; velocities = this tick's displacement / dt after the walls, the reading not saying which velocity; perpendicular side x (-n.z, n.x), the reading not saying which; standing not run again after the push, the reading not saying): no walker is ever stopped by the push on open ground (0 stalled ticks), every pair ends 0.80 m apart or walked past, none ever inside a solid, never a fall caused by the push; head-on at 6.3 they overlap to 0.236 m between centres for 4 ticks and pass with 0.119 m sideways each (13.27: 0.305 m, 2 ticks); one standing: displaced 0.35 m along and 0.19 m aside; both standing 0.3 m apart: 0.80 m in 31 ticks; into the 3.10 m ledge's face: the walker slides round his mate and both end against the wall side by side 0.80 m apart; at the 1.0 m height gap: 0 pushes (HEIGHT_GAP is ">= 1.0"); across the 0.55 step: pushed, neither ends inside it. MAX_STEP (0.25) never bit (largest push 0.147 m, three meeting); KEEP_GOING bit 5 times (three meeting). Push-out and sweep end bit-identical in every case (each pushed body began touching the wall, and both rules keep the tangent part; they differ only by the 1 mm skin when a push starts clear of a wall). Slot orders: bit-identical in 13 of 14 cases; the exception is two bodies on one spot, where the tie rule (n = +x from the lower slot; the sandbox's is a per-pair angle by trigonometry, push.ts:51-55) picks by slot by construction: the game's tie rule must use the soldier id and the slot-swap test must swap ids with slots or leave coincidence out. 3a = 3b in every push run; body 2 differs only on the step (it snaps off the far side, then the 0.56 step stops it).
- [B6-9] Ceilings (made map; the reading has no head rule, only "with headroom" and "not a floating block" for the step and the wall, game-designer-read block 1, yard.ts:243-245, :309; block 5's floating test, bottom above the head, is MY formula): walking under a 2.0 m beam: passes (bottom above the 1.8 m head); into a 1.7 m beam: stopped 0.40 m out; up the 0.55 step under a 2.2 m beam: stands on the step 0.40 m short of the beam. Jumping standing under the 2.0 m beam with no head rule (C1): the head goes through the beam (feet to 2.52 m, head 4.32 m), then the wall test puts him out sideways 1.40 m in one tick (x 56.0 to 54.6); jumping while walking at it: mantles onto the beam's top (feet 2.30). With a head stop (C2, 7 prototype lines): feet stop at 0.20 m (head at the bottom, 2.0), he falls back where he stood. All three bodies alike.

### What ran
`rr-body-proto-6.rs` beside this file (rustc 1.99.1 -O, edition 2024, f32; sha256 451aed00...282648; binary under
`.gate/rr6/`); outputs `rr-body-proto-6-3a.txt`, `-3b.txt`, `-2.txt` (body 2 as block 5 built it) and `-2w.txt` (body 2
with one line changed, B6-2). Its "b5" mode reproduces block 5's four outputs byte for byte (B6-1). Same pass rule as
block 5 (the outcome, and no fall without a jump), plus two counters block 5 lacked: ticks with the body inside a
solid, and flickers (an unjumped fall that comes back to the height it fell from). Every rule is game-designer-read's
reading (block 1 the body, block 3 the push); where the reading stops, the row says which stand-in ran.

### The table (runs per body; "pass" = outcome and no unjumped fall, no flicker, never inside, unless the row says)
| Case | (3a) | (3b) | (2) as block 5 | (2w) | Stand-in where the reading stops |
|---|---|---|---|---|---|
| Ramps from their sides, both ramps, both directions, 8 or 9 x, 3 speeds, 3 starts (306) | 306 | 306 | **198**: 108 walk into the ramp at ground level, 3 to 6 ticks inside (B6-2) | 306 | wall = block 5's push-out; the sandbox's set-back along the slope NOT run (no formula in the reading) |
| ... where on, where stopped | on the slope up to gentle x 43.1 / steep 43.0, at the slope's height under the centre; else stopped 0.40 m out | same | band gentle 42.9-43.1, steep 42.8-43.0 fails | on up to gentle 42.0 / steep 42.6 (disc height), else stopped | |
| Off a ramp's side (24) | 24: falls once the foot leaves (centre 0.21-0.31 m past), snaps from 0.15 m | 24, same | 24: snaps up to 0.55 m | 24 | |
| Landings: 5 tops (2.9, 3.05, platforms 1.0 and 1.5, step 0.55) x 8 directions x walk-off/jump x 3 speeds, then 15 still ticks (240) | 240: no bounce, no fall after landing | 240 | 240 | 240 | landing tick = the tick the feet reach a top under the foot within 0.55 m |
| ... air time off the 2.9 m ledge at 6.3 m/s | walk-off 14 airborne ticks (exact 14.75), jump 33 (exact 33.91) | same | same | same | |
| ... largest drop in one tick / deepest below a non-ground top at landing | 0.553 / 0.370 m (< 0.55: the test scene never misses) | same | 0.553 / 0.430 | same | |
| High drop, made map, 10-18 m onto a 2.0 m top (21) | **4 miss** (end on the ground beside it); 0 with the crossing fix (B6-4) | same | same | same | which feet surfaceAt uses: not in the reading |
| Snap down the ramps at 3 speeds (6) | 6; allowance 0.290-0.496 m against drops 0.053-0.221 m a tick, smallest margin 0.094 m (B6-7) | 6 | 6 (snap = step-up 0.55) | 6 | |
| Snap down the 0.55 step | a fall (0.55 > 0.2), about 6 airborne ticks | same | snapped (0.55 <= 0.55) | same | |
| Easing from rest at the 0.55 / 0.56 steps, walk on or let go at the step (48) | 48 | 48 | 48 | 48 | k 0.37375 a tick, up = stop, air 100 %, rest under 1e-4 m/s |
| Easing, ledge jumps from rest (24) | 24 (3.05 caught, 3.10 not) | 24 | 24 | 24 | |
| Block 5's ledge jumps with the inside counter (36) | **28**: at 13.27 m/s, 3 ticks inside the 2.9 m box, 1 inside the 3.05 m box, then on top; 36 with "toward" judged from the tick's start (B6-5) | same | same | same | "while moving towards it": no formula in the reading |
| Ceilings (made map): cost only, the reading has no head rule | C1 (none): jump under a 2.0 m beam, head through it, put out 1.40 m sideways in one tick; jumping at it mantles onto it; at 13.27 m/s 3 ticks inside it. C2 (head stop): feet stop at 0.20 m, falls back | same | same | same | the floating test (bottom above the head) is mine |
| Walk under a 2.0 m beam / into a 1.7 m beam / up a step under a 2.2 m beam | under / stopped 0.40 m out / on the step 0.40 m short of the beam | same | same | same | |
| Push, 14 cases (head-on 6.3 and 13.27, glancing, one still, both still, one spot, into a wall and glancing, across the 0.55 step both ways, the 1.0 m gap, three meeting, three still, three into a wall) x push-out and sweep x every slot order | never stopped on open ground, every pair ends 0.80 m apart or past, none inside, no fall from the push; MAX_STEP never bit (largest 0.147 m), KEEP_GOING 5 times; push-out = sweep bit for bit in all 14; slot orders identical in 13 (not: two on one spot, by construction) (B6-8) | = (3a) bit for bit | = (2w) | differs from (3a) only on the step | velocity = displacement / dt; turn side x (-n.z, n.x); standing not re-run after the push; one-spot tie n = +x from the lower slot |
Not run: the sandbox itself; its set-back along the slope; its headroom test (no formula); the far corner (block 1.11:
step 6c's); a push on a ramp; four or more soldiers (sums of three terms or more are where f32 order shows, block 1.9).

### What the runs change
- (3a) and (3b) pass every case here as block 5 built them, except two model holes every body shares: (i) the
  in-air step-up's "toward" (B6-5: inside the ledge for 1 to 3 ticks at sprint) and (ii) a landing on a top crossed
  inside one tick (B6-4: falls above about 5.4 m). Both fixes are a condition each, need no number and no rule of his,
  and leave block 5's cases unchanged but for the 3.05 m ledge caught one tick earlier at 13.27 m/s. They belong in
  step 1 under any body, with a test each (plants: "toward" from the tick's end; landing without the crossing).
- (2) needs one more line (2w: the wall test reads the step's own height, the disc's highest point), or it walks
  through the yard's ramps from their sides. With it, (2) passes every case; it also removes block 5's one-tick fall on
  house b's roof at 13.27 m/s.
- So no answer of his to BODY-SHAPE among (3a), (3b), (2) forces a re-run of these cases.

### 2. The 2.5D question (shapes whose bottom is above the ground)
- **The static test is exact.** An upright cylinder against an axis-aligned box or ramp overlaps exactly when its disc
  overlaps the footprint and the two height ranges overlap (for a ramp, the top is the plane's highest point over the
  disc's part of the footprint). That holds for a shape that starts anywhere: nothing in it assumes a shape rests on
  the ground. What is not exact is the motion (horizontal, then vertical, with a push-out, not a 3D sweep), and that
  is the same for a floating shape as for a resting one.
- **But every test must carry the bottom, and block 5's 170 / 200 counted a world where no test needed it.** With a
  shape above the ground: walking under it is right if the walls and the step skip a shape whose bottom is above the
  head (block 5 has it: `floating`, my formula); standing under it is right (its top is out of the step's reach);
  jumping into it has no rule at all (C1 above: through it and out sideways, a 1.4 m jump in one tick), so it needs a
  head test (C2, 7 prototype lines) or a map without such space; the slope under the foot reads any ramp whose
  footprint holds the centre, ignoring heights (rr-body-proto-6.rs `slope_under`), so a ramp overhead changes the snap
  of a soldier under it: it must read the shape the foot stands on.
- **The honest count for step 1**, against block 5's 170 / 200 (2.5D) and block 1.4's 260 / 260 (3D), each with the
  two fixes of B6-4 and B6-5 (+10 / +25), which every route needs:
  - if maps may hold space under a shape: 2.5D about **205 / 265** (+ the head test 20 / 30, the stood-on slope 5 / 10);
  - if a map rule says there is no space under any shape: 2.5D about **180 / 225** in `sim`, plus the map check in
    `content` (each shape above y 0 has its whole footprint on tops at exactly its bottom, a covering check over the
    corners' grid; about 40 / 40, with a planted bridge refused); the test scene passes it (the roofs sit on their
    houses, footprints equal);
  - 3D (1.4): 260 / 260 plus the same two fixes in their 3D form and the head's stop (a vertical sweep that 1.4 has).
  "Rests on another shape" is not enough as the rule: a beam on two posts rests on them and still has space under its
  middle. The rule that makes 2.5D need no bottom handling is "no space under any shape" (the map is a height field).
- **Who decides:** whether maps may have space under a shape (a bridge, a balcony, a tunnel, a gate's lintel) is a
  design question for Mustafa (what his maps and editor may build; map.rs:9-13 and :317 on allow it today, and
  nothing in the reading or the test scene uses it); the rule's wording and its check are engine-architect's (the map
  kind's owner), once he answers. It is the same question as the ceiling's (below).

### 3. The record rule (advisor-reread-4c.md:31): YES
F17 stands and 1.6's "and the move and aim of the record" is withdrawn (it described option (ii)); B6-R above has the
reasons, the Soldier row (`held_move: [i8; 2]`, `aim_cm: AimCm` in place of 1.1's `aim: V3`, `facing` kept) and what
`visit_shared` writes under (i) (tick; the record's buttons and presses; then the 48 rows in slot order with every
field), with the two tests and their plants.

### New question for Mustafa (CEILING), tied to MOVE-RULES row 8 (my block 3: (a) "the head stops at a ceiling", (c) "no ceiling test")
His sandbox by its reading has no head rule; the test scene has nothing above a soldier's head; the map kind allows it.
(C1) no head rule, as the sandbox: free, but on a map with a beam a jump under it goes through and is put 1.4 m aside
in one tick. (C2) the head stops at a shape's bottom and he falls back: about 20 / 30 lines. (C3) maps never have space
under a shape: no ceiling exists, 2.5D stays smallest (about 180 / 225 in `sim` plus 40 / 40 in `content`), but no
bridges or overhangs until he lifts it (then C2 joins). Recommended: (C3) now, since no map he has uses it; C2 when he
wants one. If he answers row 8 (a), that is C2.

DONE
