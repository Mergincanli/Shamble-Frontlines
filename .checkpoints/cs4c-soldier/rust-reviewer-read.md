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
- [B6-10, corrections after a check] (1) The landing fix was first run only in the high drop; now switched on over every case (FIX_LAND): no line changes outside the high drop, for 3a, 3b, 2w. (2) Push-out against sweep and 3a against 3b were first compared as printed text: now by a digest of the final bits: 3a = 3b in every push case; push-out = sweep by bits in 13 of 16 (not: the mate 0.15 m clear, 3 mm short by the skin and the slide; into a wall glancing and three standing close, by the sweep dropping pushes under 1e-7 m), the same printed numbers in 15 of 16. Two cases added for the sweep's slab and corner paths (mate 0.15 m clear; near the corner); the corner path's execution is not confirmed by a counter. (3) The record row lacked the bodies' carry-over (pull, stepped): added. (4) VISION_V2 section 6 item 2 and LOOK_V2:32 (floors, interiors, upper floors) make the height-field rule a Walk-only rule: the recommendation moves from C3 to C2.

### What ran
`rr-body-proto-6.rs` beside this file (rustc 1.99.1 -O, edition 2024, f32; sha256 26a2c9c9...87f1ed; binary under
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
| Walking off a top (no jump) and caught in the air by a higher top 2 m away | off 2.9 toward 3.05: caught at 9.576 and 13.27 m/s; off 3.05 toward 3.10: caught at all 3 speeds; off the 0.55 step toward the **0.56 step: caught at all 3 speeds** (the 0.56 step, refused from the ground, is gained this way) | same | same | same | the reading gives the in-air step for the jump's arc top (movement.ts:123-131); whether an unjumped fall also steps up is NOT in it: the run assumes it does (test feet = the tick's start). These first counted as flickers; the flicker test was narrowed to "back on the same height" |
| High drop, made map, 10-18 m onto a 2.0 m top (21) | **4 miss** (end on the ground beside it); 0 with the crossing fix (B6-4) | same | same | same | which feet surfaceAt uses: not in the reading |
| Snap down the ramps at 3 speeds (6) | 6; allowance 0.290-0.496 m against drops 0.053-0.221 m a tick, smallest margin 0.094 m (B6-7) | 6 | 6 (snap = step-up 0.55) | 6 | |
| Snap down the 0.55 step | a fall (0.55 > 0.2), about 6 airborne ticks | same | snapped (0.55 <= 0.55) | same | |
| Easing from rest at the 0.55 / 0.56 steps, walk on or let go at the step (48) | 48 | 48 | 48 | 48 | k 0.37375 a tick, up = stop, air 100 %, rest under 1e-4 m/s |
| Easing, ledge jumps from rest (24) | 24 (3.05 caught, 3.10 not) | 24 | 24 | 24 | |
| Block 5's ledge jumps with the inside counter (36) | **28**: at 13.27 m/s, 3 ticks inside the 2.9 m box, 1 inside the 3.05 m box, then on top; 36 with "toward" judged from the tick's start (B6-5) | same | same | same | "while moving towards it": no formula in the reading |
| Ceilings (made map): cost only, the reading has no head rule | C1 (none): jump under a 2.0 m beam, head through it, put out 1.40 m sideways in one tick; jumping at it mantles onto it; at 13.27 m/s 3 ticks inside it. C2 (head stop): feet stop at 0.20 m, falls back | same | same | same | the floating test (bottom above the head) is mine |
| Walk under a 2.0 m beam / into a 1.7 m beam / up a step under a 2.2 m beam | under / stopped 0.40 m out / on the step 0.40 m short of the beam | same | same | same | |
| Push, 16 cases (head-on 6.3 and 13.27, glancing, one still, both still, one spot, into a wall: touching, 0.15 m clear, near its corner, glancing; across the 0.55 step both ways, the 1.0 m gap, three meeting, three still, three into a wall) x push-out and sweep x every slot order | never stopped on open ground, every pair ends 0.80 m apart or past, none inside, no fall from the push; MAX_STEP never bit (largest 0.147 m), KEEP_GOING 5 times; push-out and sweep print the same numbers (mm) in 15 of 16 and end bit-equal in 13 (the sweep's 1 mm skin; its dropping of pushes under 1e-7 m); the mate 0.15 m clear runs the sweep's slab path and ends 3 mm short of the push-out's spot; whether the corner path ran is not confirmed; slot orders bit-identical in 15 of 16 (not: two on one spot, by construction) (B6-8, B6-10) | = (3a), final bits equal by digest | = (2w) | differs from (3a) only on the step | velocity = displacement / dt; turn side x (-n.z, n.x); standing not re-run after the push; one-spot tie n = +x from the lower slot |
Not run: the sandbox itself; its set-back along the slope; its headroom test (no formula); the far corner (block 1.11:
step 6c's); a push on a ramp; four or more soldiers (sums of three terms or more are where f32 order shows, block 1.9).

### What the runs change
- (3a) and (3b) pass every case here as block 5 built them, except two model holes every body shares: (i) the
  in-air step-up's "toward" (B6-5: inside the ledge for 1 to 3 ticks at sprint) and (ii) a landing on a top crossed
  inside one tick (B6-4: falls above about 5.4 m). Both fixes are a condition each and need no number and no rule of
  his. Each was switched on over every case of blocks 5 and 6 for 3a, 3b and 2w (FIX_TOWARD, FIX_LAND) and diffed:
  the landing fix changes no line outside the high drop; the toward fix changes only the 3.05 m ledge at 13.27 m/s
  (caught one tick earlier, tick 12) and the ledge counter (36 of 36). They belong in step 1 under any body, with a
  test each (plants: "toward" from the tick's end; landing without the crossing).
- Point for game-designer (touches PM-0004-30's "0.56 stopped"): an unjumped fall from the 0.55 m step onto the
  0.56 m step 2 m away gains it, at every speed, because the in-air step-up runs on a fall too in this model. The
  mark reads the 0.56 step from the ground (still refused, 9 of 9); whether the sandbox steps up on a fall from rest
  is not in the reading.
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
- **The honest count for step 1** (analysis, not run: the head stop ran, the stood-on slope and the map check did
  not), against block 5's 170 / 200 (2.5D) and block 1.4's 260 / 260 (3D), each with the
  two fixes of B6-4 and B6-5 (+10 / +25), which every route needs:
  - if maps may hold space under a shape: 2.5D about **205 / 265** (+ the head test 20 / 30, the stood-on slope 5 / 10);
  - if a map rule says there is no space under any shape: 2.5D about **180 / 225** in `sim`, plus the map check in
    `content` (each shape above y 0 has its whole footprint on tops at exactly its bottom, a covering check over the
    corners' grid; about 40 / 40, with a planted bridge refused); the test scene passes it (the roofs sit on their
    houses, footprints equal);
  - 3D (1.4): 260 / 260 plus the same two fixes in their 3D form and the head's stop (a vertical sweep that 1.4 has).
  "Rests on another shape" is not enough as the rule: a beam on two posts rests on them and still has space under its
  middle. The rule that makes 2.5D need no bottom handling is "no space under any shape" (the map is a height field).
- **Which count is honest: the first.** VISION_V2.md section 6 item 2 asks "a true 3D simulation: floors and building
  interiors, ledges" (:90-91), and LOOK_V2.md:32 is decided: "Buildings sized for play: interiors, upper floors,
  verticality" (EDITOR_V2.md:54 has enterable structures as LATER). So maps will hold space under shapes; a
  height-field rule could only be a rule for Walk, lifted when the first floor comes. Step 1 should be counted at
  about 205 / 265 for 2.5D (still under 3D's 260 / 260; exact for boxes and ramps with their bottoms carried), or
  3D's, and the 2.5D form stays exact only while every shape is axis-aligned.
- **Who decides:** whether Walk's maps may have space under a shape is Mustafa's (his vision already says floors
  will come); the rule's wording and its check are engine-architect's (the map kind's owner). It is the ceiling's
  question (below).

### 3. The record rule (advisor-reread-4c.md:31): YES
F17 stands and 1.6's "and the move and aim of the record" is withdrawn (it described option (ii)); B6-R above has the
reasons and the two tests. Under (i) the Soldier row holds `held_move: [i8; 2]` (new) and the aim (1.1's `aim: V3`;
holding it as received, `aim_cm: AimCm`, is my optional proposal, not needed by the ruling), and, because the same
invariant reaches the bodies' own carry-over, the body's state a later tick reads: under (3a) the pull's block
(`pull: u16`, a shape index or none) and under (3b) the stepped block (`stepped: u16`, same), none under (2); under
easing the velocity is already in the row (1.1). `visit_shared` writes: tick; the record's buttons and presses; then
the 48 rows in slot order with every field, these included. The restore test's plant gains: the pull's block kept
outside the hashed row (red: the restored world stops pulling).

### New question for Mustafa (CEILING), tied to MOVE-RULES row 8 (my block 3: (a) "the head stops at a ceiling", (c) "no ceiling test")
His sandbox by its reading has no head rule; the test scene has nothing above a soldier's head; the map kind allows it.
(C1) no head rule, as the sandbox: free, but on a map with a beam a jump under it goes through and is put 1.4 m aside
in one tick. (C2) the head stops at a shape's bottom and he falls back: about 20 / 30 lines. (C3) maps never have space
under a shape: no ceiling exists, 2.5D stays smallest (about 180 / 225 in `sim` plus 40 / 40 in `content`), but it
contradicts VISION_V2 section 6 item 2 and LOOK_V2:32's upper floors, so it could hold only until the first floor.
Under C1, by the beam run's rule, a jump under an upper floor would put him out sideways from under it (not run on a floor). Recommended: (C2), since floors are in
his vision; if he answers MOVE-RULES row 8 (a), that is C2.

DONE

## Block 7. engine-architect's RR1 to RR5 (engine-architect-read.md 5.7, :729-745), FACING-SPAWN (a), and the CEILING correction (2026-11-28)
### Plan
1. RR1: the scan's name lists and path rule (crates/sim/tests/rules_scan.rs) against libm's own spellings; what it must hold when exp and sincos join; security-auditor's planned bites (security-auditor-deps-4c.md:102-105).
2. RR2 to RR4: libm 0.2.16's sincos read and built offline (rustc on the cargo cache's source, under .gate/rr7/), a scratch program (rr-trig-proto.rs) for K's bits, quarter turns, f64-then-narrow against sincosf against a correctly rounded reference (Python decimal), and what sincos links.
3. RR5: the floor plane and the ray (collision.rs's cast_ray, world proofs).
4. FACING-SPAWN (a) yes/no and the batch's root comment.
5. The correction to block 6 (CEILING is MOVE-RULES row 8: game-designer-cs4.md:29, F18).
6. DONE.

### Notes
- [B7-C, correction to block 6] Block 6's "his sandbox by its reading has no head rule" is WRONG: game-designer's walk-check read has the sandbox's head rule, "head bumps a floating block (:132-138)" (cs4-walk/game-designer-cs4.md:29, F18, movement.ts:132-138), and MOVE-RULES row 8 already lists "the head bumps a block above" (question-2026-11-28-before-4c.md:28-31); F19 (:30) adds "no headroom = wall" for the step. So CEILING is not a new question: it is row 8's, and under row 8 (a) the head bumps. My C2 ("the head stops at the bottom and he falls back": feet set to bottom - 1.8 m, upward speed to 0) is consistent with "bumps" but F18's one line leaves unknown: what the speed becomes (0, or reversed), whether the feet are put back under the bottom, which circle and which height test the bump uses (the body's 0.4 m and the 1.8 m height, or another), and the floating test's formula. Also F19's "no headroom = wall" differs from my run "up the step under the 2.2 m beam": there the step itself would be the wall (he stays on the ground), where my model lets him onto the step and stops him at the beam. C1 (no rule) is not the sandbox's and falls; C3's VISION point stands. Block 6's 2.5D answer and counts do not change (the head test was already counted in them).
- [B7-1, RR1] Confirmed by reading the scan. Its float list `STD` (rules_scan.rs:52-62) holds std's spellings: exp, exp2, exp_m1, sin, cos, sin_cos, atan2, powf. A path call is refused when its last name is on that list (:385-387). So today `libm::exp(`, `libm::exp2(`, `libm::sin(`, `libm::cos(` and `libm::powf(` are refused, while `libm::expm1(`, `libm::expf(`, `libm::exp2f(`, `libm::sincos(`, `libm::sincosf(`, `libm::sinf(`, `libm::cosf(` and `libm::atan2f(` pass. A second gap: a grouped import with a bare call (`use libm::{exp};` then `exp(x)`) passes even for std's names, because the import's `exp` follows `{`, not `::`, and a bare call is neither a method nor a path; so does `use libm::*;`. Security-auditor's planned bite "`libm::expm1` still refused" (security-auditor-deps-4c.md:102-105) would come out red, not refused, against the scan as it stands; its answer needs that line corrected. Severity Medium, agreed.
- [B7-2, RR2] libm 0.2.16 built offline from the cargo cache (rustc 1.99.1, the build script's default cfgs `arch_enabled` and `optimizations_enabled` passed by hand; .gate/rr7/). Scratch files: rr-trig-proto.rs and rr-trig-ref.py (Python decimal at 60 digits). Route A, whole quarter turns out, then `libm::sincos` in f64 on r x K, then `as f32`: 0 of 90,000 remainders (0 to 89.999 by 0.001) off the correctly rounded f32 of sin and cos. Route C, `sincosf` on the f32 angle: 49,125 of 90,000 off, up to 19,437 ulp near 90 degrees (the f32 angle's error over a small cosine). K = `core::f64::consts::PI / 180.0` has bits 3f91df46a2529d39, the same as `f64::to_radians`'s constant (measured). Largest |x^2 + z^2 - 1| over 360,000 facings: 1.19e-7 (one f32 epsilon).
- [B7-3, RR3] Quarter turns, measured. Taken out first: 90 gives (1, 0), 180 gives (0, -1), 270 gives (-1, 0). Not taken out (route B): 90 gives (1, 6.12e-17) and 180 gives (1.22e-16, -1); route C at 90 gives (1, -4.37e-8). The remainder deg - 90q is exact and lies in [0, 90) for all 370,000 inputs tried (270,000 on the 0.001 grid, 100,000 random doubles in [90, 360)): Sterbenz holds. One trap, found by measuring: the turn written with plain negation (`-x`) gives -0.0 (90 became (1, -0)). Signed zero is a different bit pattern in the hashed state, so the turn must write `0.0 - v`. Likewise a map's facing of -0.0 passes the range [0, 360) and would carry -0.0 into the state: canonicalise r with `r + 0.0`.
- [B7-4, RR4] Built for wasm32 at opt-level 3 (no LTO; scratch rr-trig-wasm.rs) and read with the build check's own `wasmFunctionNames` and `libmProblems` (scratch rr-trig-names.mjs). The file printed four libm names: `libm::math::sincos::sincos`, `libm::math::rem_pio2::rem_pio2`, `libm::math::rem_pio2_large::rem_pio2_large` and `libm::math::exp::exp`. k_sin, k_cos, scalbn and floor were inlined. No fused or banned names appeared.
- [B7-5, RR5] `cast_ray` returns a shape (collision.rs:290-297: "the first shape a ray meets"; ties by index) and today's committed proof counts its hits per seeded map (crates/devtools/vectors/static-world.txt:2-4, "rays 403"...). Making it stop on the floor would move every downward kind's met count and digest, and its hit would have no shape index. So: the floor is the plane at `bounds_m`'s lower y beside the tree, not a 513th solid (agreed: the 512 cap and the tree's committed bytes stay); the movement's standing search starts at it (block 6's prototype: `surf` starts at the floor, 0 there). `cast_ray` stays shape-only; the floor crossing is one function beside it in collision.rs, used by the aim's fallback (NA1) and later by 4d's shots, with its own proof lines; static-world.txt does not move.

### Block 7 answers
- **RR1: yes, Medium, and a name list is not enough.** Extending the scan's list with libm's spellings still misses a grouped import followed by a bare call (B7-1). The rule should be the collision library's rule, applied to `libm`: the crate named in one file only (the maths home in `sim`, e.g. `fmath.rs`); never imported (no `use libm`, no `as`, no `pub use`, no `extern crate`), only full-path calls `libm::<name>(`; in that file an allowance per (function, routine) pair: the easing-share setup with `exp`, the spawn-facing function with `sincos`; every other `libm::` path refused, so `expm1`, `expf`, `sincosf` and `atan2f` are refused by construction, not by a list. Bites (each must be red): `libm::expm1(` and `libm::sincos(` in another function of that file, `libm::sincosf(`, `use libm::{exp};` with a bare `exp(`, `use libm::*;`, libm named in a second file. Controls: the two allowed sites pass. About 25 / 40 lines in rules_scan.rs, reusing the LIBRARY rule's code (:95-96, :326-348, :418-431). This must be in place in the commit where exp first joins; sincos joins the same allowance list. Security-auditor's item 3 (its "libm::expm1 still refused" line) goes back to it with this.
- **RR2: `libm::sincos` in f64 on r x K, then `as f32`.** One routine, measured correctly rounded on every remainder tried (B7-2). Not `sincosf`: off by up to 19,437 ulp. Not `sin` plus `cos`: the same kernels, two names, no gain. Pin K by a test that asserts `K.to_bits() == 0x3f91df46a2529d39` (it equals `f64::to_radians`'s constant). The step from degrees to the f32 pair is one IEEE multiply, one routine and one narrowing, the same on both targets (libm is pure Rust on both: N31's read, and F1 for exp).
- **RR3: exact, quarter turns taken out first.** Three compares give q; the turn is a swap and a sign written `0.0 - v`; the remainder is canonicalised with `+ 0.0` (B7-3). Tests: 0, 90, 180 and 270 by bits, with no -0.0; 45, 120, 240 and the largest double under 360 against the reference; a plant of `-v` for `0.0 - v` must turn red at 90.
- **RR4: the reviewed list gains what the players' build prints, expected four entries.** They are `libm::math::sincos`, `libm::math::rem_pio2` and `libm::math::rem_pio2_large`, plus `libm::math::exp` from the easing (B7-4). The real build's names are the truth: inlining under its profile may print fewer or more; an entry never covers a sibling (rem_pio2 does not cover rem_pio2_large). Each entry's reason: pure Rust, no fused multiply-add, run once per spawn point at match start. Only r < 45 degrees takes sincos's direct path (`ix <= 0x3fe921fb`); r in [45, 90) runs rem_pio2's medium path, plain arithmetic; rem_pio2_large is linked but never reached (it serves |x| >= about 2^20 x pi/2). Vectors (PM-0004-22): inputs on the remainder's domain [0, 90) degrees: the 90,000 points of the 0.001 grid, at least 10,000 random doubles, and the edges (0, -0.0, 45, the double just below 45, the double just below 90, tiny r under the 2^-27 x sqrt(2) cut where sincos returns (x, 1)). Also the full facings 0, 90, 180 and 270 and the largest double under 360 through the quarter turns. Outputs are compared bit for bit natively, in Node and in each browser engine, and against the correctly rounded reference (0 off natively here).
- **RR5:** B7-5. T1 and its plant in step 1, agreed.
- **FACING-SPAWN (a): YES from the maths side**, on RR1 to RR4's conditions. It needs no manifest line (libm is already the batch's dependency, and sincos needs no feature). It **does change the batch's root comment.** The comment names only "the exponential, once per match", and says "each routine it brings is on the built-file check's reviewed list", which with sincos means four entries, not one. Suggested words: "the exponential for the movement's easing shares and the sine and cosine (one routine, `sincos`) for the spawns' facings, each worked out once per match (change-set 4c)". The comment's digest moves, so the reworded batch goes to security-auditor once, with its F1 and F2 conditions standing and its item 3 corrected by RR1.
- **CEILING, corrected (B7-C).** It is not a new question but MOVE-RULES row 8's, and the sandbox has a head bump (F18). C2 is consistent with "bumps"; what F18's line does not tell: the speed after the bump, whether the feet are put back, which circle and height the test uses, and the headroom rule's formula (F19). Block 6's 2.5D answer and its counts stand.
Not run: the players' real build (its profile and wasm-opt may inline differently), Node and the browsers on the vectors (native only), folding r >= 45 to keep sincos on its direct path (not needed: 0 off without it).

DONE

## Block 8. netcode-architect's NA2, "no aim yet" (its read, block 8.3, :871-917) (2026-11-28)
### Plan
1. Read 8.3 and 7.2, endpoint.rs:258 (today's (0, 0, 0) aim), the replay's aim handling. 2. Yes or no; the row,
`visit_shared` and the facing rule. 3. Tests from my side. 4. first-loop. 5. DONE.

### Notes
- [B8-1] Today the endpoint writes `AimPoint { x: 0, y: 0, z: 0 }` into every command (client-endpoint/src/endpoint.rs:258). The recordings hold the page's ray inputs, all zero today: replay.rs:90 and :337 refuse a non-zero ray ("aim: the input call does not take it yet"). So the hashed state is what moves, not the recorded inputs: whatever aim value step 1 puts into the soldier's row enters every tick's hash from step 1 on.
- [B8-2] NONE = (-32,768, 0, 0) cannot be confused with a point the simulation makes or holds. Every aim the room passes lies inside the bounds, and the bounds lie inside +-32,767 cm (map.rs:33-37; aim_reach.rs per B8-N4). In the simulation the compare is three integer equalities, with no float and no rounding, so it gives the same answer on both targets.

### Answer: YES
- **The row** (1.1, as amended by B6-R) holds `aim_cm: AimCm`: x i16, y u16, z i16, the wire's own widths. This is now **required**: a `V3` aim would need a flag of its own to say "none". It is set to `AimCm::NONE` at spawn and overwritten by each real aim in the room's oldest-first fold; NONE leaves it as it is, and so does a tick with no command. Beside it in the row: `facing: [f32; 2]` (FACING-SPAWN (a)'s direction from the spawn, B7-3: quarter turns exact, no -0.0), `held_move: [i8; 2]`, and the body's carry (`pull`/`stepped`, B6-R).
- **`visit_shared`** writes the 6 bytes of `aim_cm` in its slot of the row: x as its two's-complement bits through `u16`, y, then z. The sink already has `u16`, so it needs no new method for this. A restore from those bytes gives NONE back exactly.
- **The facing rule**, one compare first: if `aim_cm == AimCm::NONE` (all three fields), the facing is left untouched: not recomputed, not normalised again, its bits unchanged. Otherwise the aim's metres (each field divided by 100 as f32, after the compare, so NONE is never converted) turn the facing toward it when it lies more than 2 cm away horizontally. The simulation compares the whole value; the room's check means any other x of -32,768 never reaches it.
- **Tests from my side,** besides netcode-architect's N-1 to N-3, which I take as written:
  - S-1: a spawn facing 90 is (1, 0) by bits, and stays bit-identical through 30 ticks with no command, then 30 ticks of NONE while walking. Plant: the facing recomputed or normalised again on a NONE tick (bits drift).
  - S-2: after a real aim A, NONE ticks keep turning him toward A as he walks past it (the held aim is kept). Plant: NONE resetting the row's aim, or the facing, to the spawn's.
  - S-3: the restore test of B6-R, run with NONE in the row.
  - N-2's plant (NONE read as the point (-327.68, 0, 0)) also covers converting before the compare.
- **first-loop:** YES, it moves in step 1. Step 1 already re-makes it, because the row joins the hashed state (B8-1). It moves **none in step 2 only if** NONE lands whole in step 1: `AimCm::NONE` in the row from spawn, the room's fold with its NONE case, the facing's keep, and endpoint.rs:258 writing NONE in place of (0, 0, 0). If step 1 wrote (0, 0, 0) and step 2 switched to NONE, every hash from the first tick would move again in step 2. Agreed with netcode-architect's condition 1.

DONE

## Block 9. Body (3a) rebuilt to the sandbox's rules as game-designer read them (its block 6: N16 to N24, G1 to G11), every case of blocks 5 and 6 again (2026-11-28)
### Plan
1. rr-body-proto-7.rs: block 6's case code kept, its tick and wall pass replaced by the sandbox's order and rules as game-designer-read.md block 6 states them (N17 to N22, G1 to G10): arc with end-of-tick test feet (G3); the head bump (N18, G7); move; walls with headroom, "no step from below", the slope-head rule and the set-back along a ramp (N19, N20, G4, G5), G6's guard as a switch; the step-up with the "already over it" clause (G2); the sandbox's fixed pull on top of the walk (G1); standing on the 0.4 m circle while pulling; the snap from the tick's whole displacement and the steepest ramp under the foot (G8); the push with each mover's velocity (G10). Body (3a) only.
2. Every case of blocks 5 and 6 with the same pass rule, plus a made map for the set-back (a freestanding ramp met beyond its high end, a fast fall onto a slope, a climb cap) and G5's beam-over-the-step layout.
3. What each change does to block 1's design and count. 4. DONE.

### Notes
- [B9-1] What ran: rr-body-proto-7.rs (block 6's case code; its tick and wall pass replaced), rustc 1.99.1 -O, f32, body (3a); binary under .gate/rr7/. Outputs: rr-body-proto-7-b5.txt (block 5's cases), -b6.txt (block 6's), -b9.txt and -b9-guard.txt (this block's own cases, without and with G6's guard). The sandbox's order as game-designer reads it: jump; the arc with end-of-tick test feet (G3); the head bump (N18); the move; the walls (headroom, no step from below, slope head, set-back: N19, N20, G4, G5); the step-up with "or already over it" (G2); the fixed pull on top of the walk (G1); standing on 0.4 m while it runs; the snap from the tick's whole displacement and the steepest ramp under the foot (G8); the push on each mover's velocity (G10).
- [B9-2] Where the reading stops, my assumptions, each a switch or said here: (i) the slope-head rule's "the slope's rise across the body" taken as radius x grade (env HEAD_WIDTH=1: the whole width, 2R x grade); (ii) a top at or below the feet is never a wall (without it, env HEADROOM_ON_OWN_TOP=1, headroom turns the top he stands on into a wall and, by a beam 1 m into a 0.55 m step, he flickers on and off it 6 to 12 times in 40 ticks); (iii) the set-back does not touch the velocity; (iv) of several step-up targets the highest wins; (v) the landing test is block 5's (vy <= 0 and feet at or under the top).
- [B9-3] Block 5's cases: 69 of 69 PASS (as before). Steps: every start now ends at one spot (31.125 / 31.142 / 31.192 at the three speeds): the fixed pull places him 0.135 m short of the face whatever the phase, then he walks on. Seams (G4) on the yard ramps: up at most 0.0175 m (gentle) and 0.035 (steep) at 6.3 and 9.576 m/s; at 13.27 m/s 0.0625 and 0.125 under assumption (i), 0.0175 and 0.035 under the width reading; down at most 0.035 and 0.071. Block 5 had up to 0.18. The 3.05 ledge at 13.27 is caught at tick 12 (block 5: 13). Roof edges gained 18 of 18, with seam floats 0.08 to 0.24 m (the slope-head rule excludes ramps).
- [B9-4] Block 6's cases: landings 240 of 240 (deepest below a top at landing 0.03 m, block 6 0.37); snap down the ramps 6 of 6; easing 72 of 72; off a ramp's side 24 of 24; the ledge inside counter 0 of 36 with the sandbox's "already over it" (G2: B6-5's hole is not in the sandbox; my fix is withdrawn); ceilings: a jump under the 2.0 m beam stops at feet 0.20 m and falls back; a running jump at it mantles onto it (2.30 m), never through it. The push: the same outcomes as block 6 on each mover's velocity; slot orders bit-identical in 15 of 16 (not: one spot); push-out and sweep bit-equal in 13 of 16. Ramps from the side: **264 of 306**. All 42 fails are at 13.27 m/s on the high side: one tick's move (0.442 m) is more than the 0.4 m radius, so the centre lands inside the slope's width, and the set-back along the slope (N20, as read) moves him down the slope and onto it at 0.55 m where he should stand stopped. None at 6.3 or 9.576 (his walk and sprint).
- [B9-5] This block's own cases. G1: the pull carries him 0.008 to 0.205 m beyond his walk over 0 to 3 ticks (measured over 9 runs; game-designer's arithmetic 0.055 to 0.265). With no wall test after the pull, a 0.2 m deep step against a 3 m wall: carried up to 0.051 m into the wall for 3 ticks in 7 of 9 runs, then pushed back where his foot misses the step, ending on the ground. G6: met beyond a freestanding ramp's high end (made map, 2.5 m over 9 m), the literal set-back carries him 7.26 to 7.38 m through the ramp in one tick at every speed; with the guard he stops 0.40 m out at 6.3 and 9.576, but at 13.27 he is still carried 6.98 m (the same one-tick jump into the footprint as B9-4). A fall from 8 to 16 m onto that slope: 0 of 21 missed (the set-back catches it). A 300 % ramp: the rise is capped at 0.55 m a tick. G5: a 0.55 m step with a beam (bottom 2.2 m) over its edge is a wall (stopped 0.40 m out); a floating plank (0.3 to 0.5 m) is a wall from the ground, not a step. G3: walking off the 0.55 step toward the 0.56 step, caught in 3 of 30 phases at 6.3, 30 of 30 at 9.576 and 13.27; 3.05 toward 3.10 the same; 2.9 toward 3.05: 0, 30 and 30. Block 6's row "caught at all 3 speeds" is overturned at walk. High drop onto a 2.0 m top: 4 of 21 miss (G9, the sandbox's hole too); the crossing fix works only in the vertical phase, before the walls (0 of 21); placed in standing, as block 6 had it, the walls have already pushed him out (4 of 21).

### What each change does to block 1's design and to its count (route Ours in the 2.5D form; estimates, plus or minus 30 %)
Block 1.4 and 1.5 were written before BODY-SHAPE; with (3a) built to the sandbox, step 1's movement is:
| Change | Block 1's design becomes | Code / tests |
|---|---|---|
| The tick's order | 1.5's horizontal-then-vertical is reversed to the sandbox's: arc and head bump, then the move, the walls, the step-up, the pull, standing, the snap (N24 (1)) | 0 / 0 |
| G1 the pull | the remaining pull is a fixed vector set at the step-up, added on top of the walk after the walls, dropped on an opposing velocity; he stands on 0.4 m while it runs. **The Soldier row's carry becomes `pull: [f32; 2]`** (the vector left, zero for none), not B6-R's `pull: u16` shape index, so block 8's row changes by 4 bytes | +5 / +15 over block 5's pull |
| G1 and the wall behind | the sandbox's pull has no wall test (inside a wall up to 5 cm for 3 ticks, B9-5); recommended: clip the pull by the swept circle (not felt; the sandbox's own next wall pass undoes it), as game-designer confirms not a rule of his | +5 / +5 |
| G2 "toward" | the sandbox's clause "or already over it"; B6-5's "from the tick's start" withdrawn | -2 / -5 |
| G3 test feet | end-of-tick feet on every falling tick but the arc's top | 0 / +10 (the phase test) |
| G4 slope head | the rule with its 1 cm and 5 cm tolerances; the window's formula (B9-2 (i)) asked of game-designer, since it decides the seam rise at 13.27 m/s | +20 / +25 |
| G5 headroom, no step from below | both, plus the "own top is never a wall" clause (B9-2 (ii)), confirmed by game-designer | +15 / +25 |
| G6 the set-back | along the slope with the guard, **and the wall pass sweeps the body circle** (block 5's "the game's version sweeps"), so no tick jumps the centre into a footprint: that closes the 42 side cases, the guard's leak and the beam's 1 tick inside at 13.27, and changes nothing at 9.576 or below | +25 / +35 (the sweep is in block 5's 170) |
| The head bump | block 6's head test, as the sandbox's (1 mm, tick's-start position, upward speed to 0) | in block 6's 20 / 30 |
| G8, G9, G10 | the snap's slope from any ramp under the foot (replaces block 6's stood-on slope); the landing crossing in the vertical phase (not a rule of his, G9); the push on velocities | 0 / 0; 5 / 15 already counted; 0 / 0 |
| Rules (1.2) | NO-SOURCE (b)'s rows: foot 0.15, the pull's 0.5 a tick, 0.03 and 0.9, snap 0.2, the 5 mm, the slope head's 1 cm and 5 cm, the head's 1 mm | in I2's rows |
**Count:** the 2.5D sweeps and rules go from block 6's about 205 / 265 to about **275 / 365**; step 1 for (3a) from about
1,255 / 1,315 (block 5's 1,220 / 1,250 plus block 6's bottoms) to about **1,325 / 1,415**.

### For game-designer (no rule of his changes; each is a reading the sandbox's code can settle)
1. The slope-head window (B9-2 (i)): the body's radius or its width; it decides the seam rise at 13.27 m/s (0.125 against 0.035 m).
2. Whether a top at or below his feet can be a wall by headroom (B9-2 (ii)).
3. The pull's missing wall test (B9-5): keep it, or clip the pull (recommended).
4. The 13.27 m/s cases: the sandbox's set-back by its reading shunts him onto a ramp met from its high side when one tick's move is over 0.4 m (above 12 m/s; his sprint is 9.576). The sweep closes it.
Not run: the sandbox itself; Node or browsers; the push on a ramp; four or more soldiers; easing in the push.

DONE

## Block 10. game-designer's block 7 (N25 to N27) applied to body (3a); the swept wall pass and the pull's clip measured; HEADROOM-FEET both ways (2026-11-28)
### Plan
1. rr-body-proto-7.rs gains: the slope-head window with the step-up (yard.ts:265) and the :281 test in place of my face test; the set-back's slide (movement.ts:147-157); standing with no test on upward speed (movement.ts:195); a swept wall pass for the walk (switch WALK_SWEEP); the pull clipped by the same sweep (switch PULL_CLIP); every tick's position bits written to stderr, so that two runs compare tick by tick.
2. Every block 5, 6 and 9 case, sweep off and on, clip off and on: bit-equal or not, and where not, at 6.3 and 9.576 m/s.
3. HEADROOM-FEET both ways on the window wall (3.0 m wall, 0.2 m thick, sill 0.9 m, opening 1.2 m) and a crate under an upper floor.
4. What changes in block 1 and its count. 5. DONE.

### Notes
- [B10-1] rr-body-proto-7.rs (sha256 07868554...d5cab420) gains block 10's forms as its defaults: the slope-head window feet - 0.05 .. feet + 0.55 + 0.4 x grade + 0.005 (yard.ts:265) with the :281 test ("slope height at the box's nearest point >= box top - 0.55") in place of my face test; the set-back's slide (his speed up the slope removed, movement.ts:147-157); standing with no test on upward speed (movement.ts:195). OLD_HEAD, OLD_SETBACK and OLD_LANDING give block 9's forms back: with all three, blocks 5, 6 and 9 reproduce block 9's outputs byte for byte. Outputs: rr-body-proto-8-b5.txt, -b6.txt, -b9.txt (push-out), -swept.txt (sweep and clip on), -b9-sandbox-headroom.txt.
- [B10-2] With the 0.55 in the window, the seam rise at the x 45 seams is at most 0.0175 m (gentle) and 0.035 m (steep) at every speed, 13.27 m/s included: game-designer's prediction holds (block 9 had 0.0625 / 0.125 at 13.27). Block 5's cases: 69 of 69. Block 6's: the same tallies as block 9 (ramps from the side 264 of 306 under push-out, every fail at 13.27 m/s). The set-back's slide and the landing with no speed test change no printed line of blocks 5, 6 or 9.
- [B10-3] The swept wall pass for the walk (WALK_SWEEP), measured tick by tick on bits (each tick's x, z and feet written to stderr; rr-trace-compare.py; traces under .gate/rr7/). Its form: a swept circle against the shapes that are walls at the tick's start, no skin, a face contact put exactly on the grown face (so push-out's own bits), one slide; a ramp met within its width is left to the set-back unless he comes from beyond its high end. Two earlier forms were wrong and are fixed: (i) a start exactly on the grown face read as no contact (the slab test needed "a >= t0"); (ii) a ramp swept at its low end blocked the house roofs (a raised ramp is a wall from below) and lost the roof edge; now only its sides and an exposed high end are swept. Result at 6.3 m/s: block 5's 35 runs and 12 slow runs bit-equal; block 6, 4 of 341 runs differ, all in one push case (the walker going round the 3.10 m ledge's corner while pushing his mate: push-out moves him radially, the sweep slides; up to 0.063 m); block 9, 1 of 101 (the G6 high-end case, where the sweep stops him 0.40 m out instead of the 7.4 m set-back: intended). At 9.576 m/s: block 5, 6 of 32 runs differ (the six roof jumps, up to 0.51 m in the path, one tick earlier at the run's end; same outcome: edge gained, ridge reached); block 6, 1 of 255 (a landing that grazes a corner, 4 micrometres); block 9, 3 of 101 (G6, and the pull-into-wall layout by 0.2 m in timing). At 13.27 m/s it changes what it was built to change: ramps from the side 306 of 306 (push-out 264), the G6 case stopped 0.40 m out at every speed with or without the guard, the beam's tick inside gone; the 10,000 thin-wall starts differ by at most 1 micrometre. So "no change at 6.3 and 9.576" is NOT exact: bit-equal on face contacts and in the open, different at corners (radial against slide) and in the roof jumps at sprint.
- [B10-4] The pull's clip (PULL_CLIP, the pull swept with the same sweep): bit-equal to no clip on every block 5 and block 6 run at 6.3 and 9.576 m/s (and at 1 and 2 m/s); 2 of 259 block 6 runs differ at 13.27 m/s (up to 0.024 m, where a wall is within the pull's reach); in block 9 only the pull-into-wall layout differs (9 runs), where the clip takes the body's 3 ticks inside the wall to 0. game-designer's condition (bit-equal where no wall is in reach) holds on every case here.
- [B10-5] HEADROOM-FEET both ways (made map: his window wall, 3.0 m high, 0.2 m thick, sill 0.9 m, opening 1.2 m, the wall over the window 2.1 to 3.0 m; and a crate 1.5 m high under a floor from 3.0 to 3.2 m). (a), the clause: he walks the wall's whole top over the window (x 51 to 58, feet 3.0 throughout), and walks the floor over the crate; dropped onto the wall's top over the window he stands on it. (b), the sandbox's own test (HEADROOM_ON_OWN_TOP=1): stopped 0.40 m before the window stretch on the wall's top (x 53.6) and stopped 0.40 m before the crate's footprint on the floor (x 63.6, an invisible wall), at 6.3 and 9.576 m/s alike; dropped onto the stretch over the window, he is put off the wall and lands on the ground (feet 0). On the test scene's cases (b) changes only the block 6 beam over the 0.55 step (the flicker), nothing else.

### Block 10's answers
- **The slope-head window:** with the 0.55 (yard.ts:265) and the :281 test, measured: the seam rise is at most 0.0175 / 0.035 m at every speed, as predicted.
- **The swept wall pass, measured:** at his walk and sprint it is bit-equal to the push-out in the open and on every face contact, and differs at corners (up to 0.063 m over a run, the walker rounding a corner while pushing) and in the six roof jumps at sprint (path up to 0.51 m, the outcome the same). Above 12 m/s it closes the three holes (sides, the high end, the beam). It is his DEPART-4C (a), but the report must not say "no change at 6.3 and 9.576": it says "bit-equal except at corners and on the roof jumps at sprint", with these numbers.
- **The pull's clip:** bit-equal wherever no wall is in reach, on every case: game-designer's condition holds.
- **HEADROOM-FEET:** built as a switch, both ways measured (B10-5); (a) is the default.
- **What changes in block 1 (the 2.5D route Ours) and its count:**
  - the wall pass is a sweep with block 10's form (face contacts on the grown face, no skin, one slide, ramps swept only at their sides and an exposed high end): +10 / +20 over block 9 (the start-on-face case and the ramp rule, each with a test and a plant: the slab without "a >= t0", and a raised ramp swept at its low end, both red on the roof case);
  - the pull is swept by the same function: 0 code beyond block 9's clip / +10 (the bit-equal test on the yard's steps, the clip's wall case as its plant);
  - the slope-head window with the step-up and the :281 test: 0 / +5 (the 13.27 m/s seam case);
  - the set-back's slide and the landing with no speed test: +2 / +10;
  - HEADROOM-FEET as a content-free switch in `Rules` (a bool, default the clause): +5 / +20 (the window wall and the floor over a crate, both ways);
  - total: the 2.5D movement goes from block 9's about 275 / 365 to about **292 / 430**; step 1 for (3a) from about 1,325 / 1,415 to about **1,342 / 1,480** (estimates, plus or minus 30 %).
Not run: the sandbox itself; Node or browsers; the clip and the sweep in the push pass together with a pull (the push has its own sweep, block 6); four or more soldiers.

DONE

## Block 11. Step 1, consolidated: every symbol, rule, test and count, against the branch head (2026-11-28)
### Plan
1. Gather: my blocks 1 and 5 to 10; netcode-architect-read blocks 7 and 8 (record (i), the bounds' one home, "no aim yet"); engine-architect-read blocks 5 and 6 (GROUND, FACING-SPAWN (a), the world with no aim box); the sheet's E1, the record rule, the batch's commit order, I2 and I4b; dependency-check.md.
2. Read today's code at the branch head (2fe2c9c6: main dae6d107 plus 4c-1, which touches no Rust): crates/sim/src (lib 48, world 477, collision 954, vec 24, time 17, rng 317, state_hash 78 lines), content's movement.rs, the callers.
3. Write one block: the tick's order; the row and `visit_shared`; the sweep and every rule; `Rules::take`; exp and sincos with the batch; symbols by file; tests with their plants; pins re-made; count; what it supersedes. 4. DONE.

### Notes
- [B11-1] The branch head's Rust is main's: `git log` shows 4c-1's commits are page and notes only, so every file:line below is dae6d107's. Today's facts used: `PlayerCommand { buttons: u16, presses: [u8; 16] }` (world.rs:40-43); `SHARED_STATE_BYTES = 4 + 2 + 16` (:18); `StateSink` has `u16`, `u32`, `bytes`, `authority` and `secret`, with no float method (:57-66); `World { tick, player, seed }` (:112-117); `World::new` is `const fn` (:122); `authority_step` (:149); `visit_shared` (:177); the known answers at :401-415 (0x9c1e_17d4_0d7a_14dd and six more); `StaticWorld` (collision.rs:95), `narrow` (:103), `enter_box` (:114), `enter_ramp` (:152), `build` (:212), `cast_ray` (:300); content's `FIELDS: [Field; 15]` (movement.rs:17-33) and `Movement` (:37-54); recordings `crates/devtools/recordings/first-loop.{content.json,hashes,rec,vector.txt}` and `manifest.txt`.

### 11.1 The tick (`World::authority_step(&mut self, rules: &Rules, statics: &StaticWorld, counts: &mut QueryCounts, commands: &[PlayerCommand])`, every borrow for the call only, lent by the `Room` (engine-architect block 7, confirmed in block 14); in this order; slot order throughout, no hash map, no clock)
1. **The command fold** (room side, then world): the slot's commands for the tick are applied oldest first. The newest one's buttons and presses become the record, a `PlayerRecord { buttons, presses }` (engine-architect B1; today world.rs:150-152 copies the whole command). Each command's move goes into the row's `held_move`. Each command with a real aim sets the row's `aim_cm`; `AimCm::NONE` leaves it. A tick with no command keeps all three (world.rs:143-145's rule, extended to the row). A slot with no soldier: the record is taken, no row is touched (netcode-architect 9.2 condition 2).
0. **At the match's start** (`World::new`, netcode-architect 9.2, B4 (a)): slot `PLAYER_SLOT`'s soldier at the spawn table's first point (ids in byte order, block 15), facing by `facing_of`, stamina full, `held_move` (0, 0), `aim_cm` NONE, **grounded set** (block 15: the map's `at` is "the point on the ground between the feet", map.rs:92; tick 1's standing corrects a mismatch: a top within the snap is stood on, a larger drop starts a fall from rest).
   Known edge (G9, pinned in test 23): grounded is set at spawn and the jump (2b) comes before standing (2h), so a soldier spawned above the ground who presses jump on tick 1 jumps from the air. No recording meets it: tick 1 is command-less (B9-N2), and map.rs:92 says a spawn stands on the ground.
2. **Per soldier, in slot order:**
   a. Wish: `held_move` over `MOVE_REACH` (127): integer length at most 127, refused beyond it by the room (`refused_move`, `sim::move_in_reach`; netcode-architect 9.3); a world fed a longer pair directly (a test, a future caller) caps the wish at length 1 (one compare, then the pair over its own length), so no path moves him faster than his speed (block 15); sprint and stamina in integers (block 1.2); the speed = walk or sprint.
   b. Jump: at most one per tick when the presses counter passed the record's base and he is grounded (3B-ONE-PER-TICK, J1 to J5).
   c. **The vertical phase first** (the sandbox's order, movement.ts:104-139): the exact arc; the test feet are the end-of-tick feet on every falling tick, and the arc's top on the tick that holds it (G3). The head bump comes next: at the tick's start position, the lowest underside over the 0.4 m circle, no lower than the head at the tick's start less 1 mm, caps the feet at underside - body height. His upward speed goes to 0; a downward one is kept (N18). The bounds' top counts as an underside. The landing crossing follows: a top under the foot crossed inside this falling tick lands him on it, in this phase, before the walls (G9; a fix, not a rule of his).
   d. **The horizontal move:** the easing toward the wish (`ease_up` or `ease_stop` a tick, times `air_share` while airborne; rest under 1e-4 m/s sets 0, a stated tolerance the sandbox lacks), then move = velocity x dt.
   e. **The one sweep** (11.3) of the move against what is a wall at the tick's start, with one slide; then the set-back pass for ramps whose width holds his centre (11.3); the bounds' four sides are walls, and the speed into them is dropped (MOVE-RULES row 8).
   f. **The step-up** (11.3) from the feet after the arc; on a hit: feet = top, upward speed 0, grounded, and the pull fixed.
   g. **The pull** (G1): the fixed vector left, half a tick (all of it under 0.03 m), moved by the same sweep (so clipped by walls); dropped when his velocity points against it.
   h. **Standing:** on the 0.4 m circle while the pull runs, else on the 0.15 m foot: the highest top within feet + 0.55 + 0.005. Landing needs no test on upward speed (movement.ts:195). **The snap:** a drop of at most 0.2 + (the tick's whole displacement + 0.15) x the steepest ramp under the foot circle at any height (G8); else a fall from rest.
   i. **The facing:** if `aim_cm == AimCm::NONE`, unchanged bit for bit; else toward the aim's metres (cm / 100) when more than 2 cm away horizontally.
3. **The push** (game-designer-read block 3, on each mover's velocity, G10): pairs i < j in slot order; skipped when the feet are 1.0 m or more apart or the pair is not within 0.8 m; head-on turn above 0.2 m/s of relative speed; share 1 - exp(-8/30); sums applied after the pairs; keep-going above 0.1 m/s; cap 0.25 m a tick; the pushed move goes through the same sweep. Two soldiers on one spot: n = +x from the lower soldier id (the sandbox's per-pair angle is trigonometry). Standing is not run again.
4. The tick counter, as today (world.rs:153-156).

### 11.2 The Soldier row, the record and `visit_shared`
- **Row** (soldiers.rs, `Soldier`, 60 bytes as hashed, little-endian, in this order):
  | Field | Type | Bytes | Note |
  |---|---|---|---|
  | `generation` | u16 | 2 | stale ids (PM-0004-15) |
  | `flags` | u8 | 1 | bit 0 occupied, 1 alive, 2 grounded, 3 sprinting, 4 sprint locked |
  | `team` | u8 | 1 | |
  | `position` | V3 (f32 x 3) | 12 | the feet, as content's spawn `at` (map.rs:92) |
  | `velocity` | V3 (f32 x 3) | 12 | horizontal (eased) and vertical |
  | `stamina` | u64 | 8 | integer units (block 1.2) |
  | `facing` | [f32; 2] | 8 | FACING-SPAWN (a)'s direction from spawn; quarter turns exact, no -0.0 (B7-3) |
  | `held_move` | [i8; 2] | 2 | netcode-architect 7.2 |
  | `aim_cm` | AimCm (i16, u16, i16) | 6 | `AimCm::NONE` = (-32,768, 0, 0) from spawn (block 8) |
  | `pull` | [f32; 2] | 8 | the fixed pull's remainder; (0, 0) none (block 9, replaces B6-R's `pull: u16`) |
  Not in step 1's row: block 1.1's `since_sprint` (the sprint-to-fire delay, 4d) and `aim: V3` (replaced by `aim_cm`).
- **The record** stays `PlayerCommand`'s buttons and presses for the one player (world.rs:115; record (i)).
- **`visit_shared` writes:** tick (u32); the record's buttons (u16) and presses (16 bytes), as today; then the 48 rows in slot order, every field above, floats by their bits (`f32::to_bits`, LE), i8 and i16 through their two's-complement bits. `SHARED_STATE_BYTES` = 22 + 48 x 60 = 2,902. `StateSink` gains `fn f32(&mut self, name, value: f32)`; u64 goes through `bytes::<8>`. `visit` names every field in its pattern, as today (:166-170).

### 11.3 The one sweep and every rule (the 2.5D form; block 10's measured form)
- **`sweep_circle`** (new `body.rs`; our own, no library: SWEEP-ROUTE): a 0.4 m circle swept by (dx, dz) against the footprints of the shapes that are walls at the tick's start, found through the tree's candidates for the swept box (allocation-free, the tree's inline stack). Contacts: touching within 0.1 mm and moving along or away is no hit; a start exactly on the grown face is a contact (the slab test's "a >= t0"); a face contact puts the contact axis exactly on the grown face (push-out's own bits); a corner uses the radial normal; no skin; one slide; the speed into the wall is removed. Ramps: a ramp whose width holds his centre is swept only when he comes from beyond its exposed high end; otherwise it is left to the set-back. Its sides are swept like any wall. Ties go to the lower shape index.
- **Is a wall** (for him, at test feet f; N19, N20, G5): skip a shape whose bottom is at or above f + 1.8 - 1e-6; under HEADROOM-FEET (a) only, skip a top at or below f (a flag: `HeadroomFeet::SandboxTest`, his sandbox's rule, is the default by the advisor agent's Q2 until he answers; `OwnTopNeverWall` only through a test-only constructor; block 13); not a wall if the height nearest his centre <= f + 0.55 + 0.005 with headroom (no floating underside strictly inside top .. top + 1.8 over the 0.4 m circle less 1e-6) and its bottom <= f + 1e-6; nor at a slope's head.
- **The slope-head rule** (G4, yard.ts:264-281 as game-designer reads it): a resting box, level within 1 cm with the top of a ramp under the 0.4 m circle whose height near him lies in f - 0.05 .. f + 0.55 + 0.4 x grade + 0.005, and where the ramp's height at the box's nearest point is >= box top - 0.55, is neither a wall nor a step target.
- **The set-back** (N20, G6): a ramp that is a wall with his centre within its width and over the slope: set back along the slope to where its height is f + 0.55 (from the ramp's own low height; no 5 mm), only if he is further up; his speed up the slope is removed (movement.ts:147-157). Beyond the high end, the sweep has already stopped him (G6's guard is the sweep's ramp rule).
- **The step-up** (N17, G2): the 0.4 m circle overlaps; the top is above the feet after the arc (+1e-6) and <= test feet + 0.55 + 0.005; headroom; bottom <= feet + 1e-6; not at a slope's head; his velocity points toward the shape's nearest point **or his centre is already over its footprint**; the highest such top wins. On a hit the pull is fixed: toward the shape's nearest point, length distance - 0.9 x 0.15.
- **The floor** (GROUND, engine-architect 5.2): the plane at `bounds_m`'s lower y beside the tree, not a 513th solid; the standing search starts at it.

### 11.4 `Rules::take(&content::movement::Movement) -> Result<Rules, RulesError>` (rules.rs)
- **From his Export** (today's 15 fields, movement.rs:17-33), each derived once in f64 and then narrowed: walk; sprint = walk x multiplier; ads (4d; carried); `ease_up` and `ease_stop` a tick = 1 - exp(-ln 10 x dt / T) (fmath); air share; radius; step-up; g dt and g dt^2 / 2; take-off = sqrt(2 g (reach - step-up)); the stamina's integers; push strength, giving the share 1 - exp(-strength x dt) (fmath); push reach = 2 x push radius.
- **NO-SOURCE (b) rows, each a new content field marked "not from his Export"** (the movement kind's FIELDS grows from 15 to 28, each with its version rule, 4B-FIRST-DEFAULT; the Testlines table takes rows with a `field` and no `export`):
  - `body_height_m` 1.8 (his Export's measured `soldier_height_m`, so its row says "his Export, measured block");
  - `foot_radius_m` 0.15;
  - `pull_share_per_step` 0.5 (a share a step at his 30 steps, G11, not a time);
  - `pull_done_under_m` 0.03;
  - `pull_to_foot_pct` 90;
  - `step_allowance_m` 0.005;
  - `snap_down_m` 0.2;
  - `push_height_gap_m` 1.0;
  - `push_sideways_share` 0.7;
  - `push_keep_going_share` 0.5;
  - `push_most_m_per_step` 0.25;
  - `push_head_on_gate_m_per_s` 0.2;
  - `push_keep_going_gate_m_per_s` 0.1.
  - The tolerances stay code constants, not content: the slope head's 1 cm and 5 cm, the head's 1 mm, the 1e-6 shrinks, the 0.1 mm touch.
- **Refusals** (`RulesError`, each with `message()`):
  - a reach below the step-up (a negative under the root);
  - a sprint or refill time that rounds to 0 units;
  - a body height under 2 x the radius;
  - a foot radius over the body radius;
  - a pull share outside (0, 1];
  - a step allowance or snap outside [0, step-up].
- **`HEADROOM_FEET`**: a `rules.rs` const, `HeadroomFeet::SandboxTest` (b) by default (the advisor agent's Q2); `Rules` carries it; `Rules::with_headroom_feet` (`#[cfg(test)]`) runs `OwnTopNeverWall` (a). It flips only on his word (question 13).

### 11.5 exp and sincos, and the dependency batch (one commit)
- `crates/sim/src/fmath.rs`, the one file that names `libm`, by full path only:
  - `share_per_tick(t_s: f64, dt_s: f64) -> f32` uses `libm::exp`, called by `Rules::take` three times per match.
  - `facing_of(deg: f64) -> [f32; 2]` uses `libm::sincos` once per spawn point at match start: whole quarter turns taken out by three compares, r x K with K = PI / 180 (bits 3f91df46a2529d39, pinned by a test), narrowed with `as f32`, the turn written `0.0 - v`, and r canonicalised with `+ 0.0`.
- **The commit that first calls them also holds:**
  - `deps-4c-v2.diff` applied (its five digests, dependency-check.md);
  - F1, the tightened `crates/devtools/tests/dependency_rules.rs`;
  - F2, `crates/devtools/tests/layering.rs`;
  - the scan's stricter libm rule in `crates/sim/tests/rules_scan.rs` (B7-1: named in fmath.rs only, never imported, two allowances, (share_per_tick, exp) and (facing_of, sincos), every other libm path refused);
  - `packages/client/scripts/maths-allowed.mjs` entries for what the players' real build prints (expected `libm::math::exp`, `libm::math::sincos`, `libm::math::rem_pio2`, `libm::math::rem_pio2_large`), with :9-10 rewritten;
  - the exp and sincos vectors (`crates/devtools/vectors/`);
  - world.rs's known answers and first-loop re-made (11.8).

### 11.6 Symbols to add, change and remove, by file
- **sim, new files:**
  - `ids.rs`: `SoldierId { slot: u16, generation: u16 }`, `to_bits`, `from_bits`, `NONE`.
  - `soldiers.rs`: `MAX_SOLDIERS = 48`, `Soldier` (11.2), its flag constants, `Soldiers { slots, count }`, `spawn`, `despawn`, `resolve`, `resolve_mut`, `iter`; `SpawnPoint { at: V3, facing: [f32; 2] }` and `spawn_points(&MapDocument) -> Vec<SpawnPoint>` (ranked by id, `at` by `narrow`, `facing` by `fmath::facing_of`; built once per match).
  - `rules.rs`: `Rules`, `Rules::take`, `RulesError` and its `message`, `HeadroomFeet`, `HEADROOM_FEET`.
  - `fmath.rs`: `share_per_tick`, `facing_of`, `DEG_TO_RAD`, plus 3A-SCAN-HELPERS' float helpers (block 1.3).
  - `aim.rs`: `AimCm`, `AimCm::NONE`.
  - `body.rs`: `sweep_circle`, `is_wall`, `headroom`, `at_slope_head`, `set_back`, `step_up_target`, `standing_height`, `steepest_under_foot`, `QueryCounts`.
  - `movement.rs`: `step_soldier` and its phases (11.1 2a to 2i), `push_pass`.
- **sim, changed:**
  - `vec.rs`: V3's `add`, `sub`, `scale`, `dot`, `length_sq`.
  - `collision.rs`: `StaticWorld` gains `bounds_m`, `bounds_cm`, `floor_y()`, `candidates(min, max, visit)`, `holds_cm`, `clamp_cm`; `build` derives both bounds from the f64 face (netcode-architect 7.3); `WorldError::Bounds`; `narrow` (:103) becomes `pub(crate)`.
  - `world.rs`: ADD `MOVE_REACH: i32 = 127` and `const fn move_in_reach(x: i8, y: i8) -> bool` (integer squares in `i32`), and `PLAYER_SLOT: u16 = 0`, `PLAYER_TEAM: u8 = 0` (read by `World::new` and by the room's join answer in place of its literals, room/src/lib.rs:161-162); `PlayerCommand` gains `move_x`, `move_y: i8` and `aim: AimCm` (its `NONE` takes `AimCm::NONE`); ADD `PlayerRecord { buttons: u16, presses: [u8; BUTTONS] }` with `NONE`, and `World.player: PlayerRecord`, `World::player() -> &PlayerRecord`; `World` gains `soldiers` only (`World { tick, player, soldiers, seed }`; block 14 strikes "and the match's constants (`rules`, `statics`, `spawns`), which are not state"); `World::new(setup: &MatchSetup, rules: &Rules, spawns: &[SpawnPoint])`, no longer `const`, making slot 0's soldier at the lowest-id spawn (B4 (a), its owners' call); `authority_step` (11.1); `visit` and `visit_shared` (11.2); `SHARED_STATE_BYTES` and `STATE_BYTES`; `StateSink::f32` with its implementors `Canonical` (:89) and the test sinks (:313, :360).
  - `lib.rs`: the new modules; the re-exports gain `PlayerRecord` and `SpawnPoint`.
  - `body.rs`: `QueryCounts` is a run's tally held by the `Room` and lent `&mut` to the step; block 2's accessor on the world is struck (engine-architect B2).
- **content:**
  - `movement.rs`: FIELDS 15 → 28 and `Movement` (11.4), with the schema, generated types and test text.
  - `map.rs`: `on_grid` made `pub`; GROUND's words (engine-architect 5.2); the aim's "floor" words.
  - `content/movement/soldier.json`: the 13 new values.
  - `content/source/testlines-table.json` rows with `packages/client/scripts/testlines.mjs --check`.
- **Callers:**
  - `room/src/lib.rs`: `Room` gains `statics: StaticWorld` (C4, read by `holds_cm`), then `rules: Rules` and `query_counts: QueryCounts` (C5, read by the step) beside `world` (:87); **it keeps no spawn table** (block 16, G1: nothing reads it after `World::new`); `Room::new(setup, content_hash, rules, statics, spawns: &[SpawnPoint])` (:105), total, borrowing the table (and in C4 `&Rules`) for `World::new`; ADD `Room::statics()` and `Room::query_counts()`; :282 lends the four; :286-296 reads the `PlayerRecord`; the fill (:216-223) takes the decoded command's move and aim; `on_datagram` refuses, per command after the lead check, a move beyond `move_in_reach` (`refused_move`) and an aim that is not NONE and that `statics.holds_cm` does not hold (`refused_aim`), both in `RoomCounters` and `refused()` (:73-79); the doc (:176-182); `const _: () = assert!(...)` tying `AimPoint::NONE` to `AimCm::NONE` beside :28; the join answer reads `PLAYER_SLOT` and `PLAYER_TEAM`; :219 and `queue.rs:137` (literals); its tests and `tests/any_bytes.rs:117` build the three once from the test bundle (the batch's dev-dependency) and clone them per room.
  - `room/tests/any_bytes.rs` (the generator: honest moves inside 127 through `sim::move_in_reach`, honest aims inside the bounds or NONE; three lies: a move beyond 127, an aim 1 cm past a face, (-32,768, 1, 0); `counted` gains `refused_move` and `refused_aim`).
  - `protocol/src/messages.rs:117`: "a room refuses a longer one" joins the move's sentence.
  - sim-wasm: `lib.rs`'s `local_world` (:83-86) becomes `local_constants(set) -> Result<(Rules, StaticWorld, Vec<SpawnPoint>), CreateError>` (`Sim::create` drops the table after `LocalMatch::new` borrows it) (the map by `LOCAL_MAP`; the movement as the bundle's one movement document, B5 (c)); `CreateError` gains `Rules(RulesError)` and `Soldier { found }`; its test :401-410 reads `local().room().statics()`; `local_match.rs`: `LocalMatch::new(setup, first_sequence, pad_bytes, link, rules, statics, spawns: &[SpawnPoint])`, loses `world` and its doc (:48-50) and `static_world()` (:96-100); its tests (:203-215) build the three; `tests/step_alloc.rs:81-86`.
  - `protocol/src/messages.rs`: `AimPoint::NONE` and the aim doc's sentence.
  - `client-endpoint/src/endpoint.rs:258`: writes NONE.
  - devtools: `replay.rs:181` (`Lines` gains `f32`); `tests/hash_cost.rs:40-46` builds the three and passes them; `tests/layering.rs` (the batch's F2) reads non-test tokens only (rules_scan.rs:695-718's marking reused).
  - `sim/tests/state_hash_alloc.rs:70-94` builds the three and passes them.
  - client-endpoint: nothing beyond `endpoint.rs:258` in step 1.
- **Removed:** `LocalMatch.world` and `LocalMatch::static_world()`; `local_world`; `World.player: PlayerCommand`; `World::new`'s `const`; block 2's query-count accessor on the world; block 7.3's first aim (never built); netcode-architect's "off the grid is refused".
- **New tests:**
  - `crates/sim/tests/step_alloc.rs`;
  - `crates/sim/tests/movement_yard.rs` (the yard's cases);
  - the restore test in world.rs's test module;
  - the oracle in collision.rs's test module (it alone may name parry3d).

### 11.7 Tests, each with its planted defect (red when the plant is in)
| # | Test | Plant |
|---|---|---|
| 1 | ids: 10,000 seeded cases of up to 1,000 spawns and despawns on a 64-slot table (PM-0004-15) | spawn without raising the generation; despawn leaving `occupied` |
| 2 | `Rules::take`: each field reaches its own rule; each refusal by name; the shares against a decimal reference within 1 ulp | `ease_stop` from the speed-up time; one refusal removed |
| 3 | `facing_of`: 0, 90, 180, 270 by bits with no -0.0; 45, 120, 240 and the largest double under 360 against the reference; K's bits | `-v` for `0.0 - v`; no quarter turns (90 gives 6.1e-17) |
| 4 | vectors (PM-0004-22): exp and sincos, 10,000+ inputs each, bit-equal natively, in Node and in each browser engine, and correctly rounded (sincos's f32 0 off over 90,000, B7-2) | one output moved one ulp on one target |
| 5 | scan: the libm rule's bites (`libm::expm1(` and `libm::sincos(` in another function, `libm::sincosf(`, `use libm::{exp};` with a bare `exp(`, `use libm::*;`, libm in a second file) and its two controls | the rule switched to a name list (the grouped import passes) |
| 6 | the yard: 0.55 gained and 0.56 stopped, 9 runs each (PM-0004-30); 3.05 caught and 3.10 refused (PM-0004-59, the true top against `jump_reach_m` as read); slow walks; stop at touch; both ramps up and down; walk-offs; thin wall at 13.27 m/s, 0 of 10,000 past (block 5's cases, 69) | the 5 mm allowance dropped (0.555 line); the old order of the two sums (180 mm, DEPART-4) |
| 7 | the sweep, start on a face (made map: the 3.10 m ledge's x 40 face alone): from push-out's own spot (40 - 0.4), walking into it 30 ticks at 6.3, 9.576 and 13.27 m/s, the circle is 0 m into the wall right after the sweep, before any push-out, and ends on push-out's spot to the bit (first-sweep bug 1; block 12) | the slab test without "a >= t0" and a 1e-6 touch: 0.21 / 0.32 / 0.40 m into the wall right after the sweep |
| 8 | the sweep, a raised ramp, off the roofs (made map: a 2.6 m block x 70..76, z -10..-4, with a 25 % ramp on it, x 70..76, z -10..-6, 2.6 to 3.6 m, its low end at the block's edge): a running jump from 0.5, 1.0 and 1.5 m out gains the edge and walks up the ramp, at 6.3, 9.576 and 13.27 m/s, 9 of 9 (first-sweep bug 2; block 12) | the ramp swept at its low end: the edge not gained, 0 of 9 |
| 9 | ramps from the side, 306 runs, every speed (block 6 A); off a ramp's side, 24 | push-out with no sweep (42 fail at 13.27) |
| 10 | G6: beyond an exposed high end he stops 0.40 m out at every speed; a fall from 8 to 16 m onto a slope lands on it (21 of 21); a 300 % ramp rises at most 0.55 m a tick | the set-back with no guard (7.4 m through) |
| 11 | the pull: the carry at the 0.55 step bit-equal to the unclipped pull (the yard's steps, every speed); before a wall 0.2 m beyond a step, 0 ticks inside | the pull moved with no sweep (3 ticks inside, 5 cm) |
| 12 | G2: the 36 ledge jumps spend 0 ticks inside | "toward" without "or already over it" (1 to 3 ticks inside at 13.27) |
| 13 | G3: off the 0.55 step toward the 0.56 step, caught in 3 of 30 phases at 6.3, 30 of 30 at 9.576 | test feet at the tick's start (caught at walk too) |
| 14 | G4: the x 45 seams rise at most 0.0175 / 0.035 m at every speed | the window without the 0.55 (0.125 at 13.27) |
| 15 | G5 and HEADROOM-FEET, each layout under both modes (block 13): his window wall (3.0 m, sill 0.9, opening 1.2) walked along its top: (b) stopped 0.40 m short of the window stretch (x 53.6), (a) walked whole (x 58); dropped onto that stretch: (b) put off the wall to the ground, (a) stands; the floor over a 1.5 m crate: (b) stopped 0.40 m short (x 63.6), (a) walked; a step with a beam over its edge: a wall in both; a floating plank: a wall from the ground in both; a beam 1 m into a 0.55 m step: on the step 0.40 m short of the beam, 1 step, 0 falls, in both (with the sweep; the 6 to 12 falls in 40 ticks of B9-2 come only from a push-out wall pass, pinned as the sandbox's push-out behaviour by the prototype's model, not the game's); the walk and lane-walk routes' hashes equal under both modes (the test scene holds no shape where (b) can bite) | each mode run with the other's rule (red on the window wall and the crate); the wall pass as a push-out under (b) (the flicker returns) |
| 16 | the head bump: a jump under a 2.0 m beam stops the feet at 0.20 m, upward speed 0, then a fall | the bump at the tick's end position |
| 17 | landings: 240 runs, no bounce, no fall after landing; the high drop onto a 2.0 m top lands 21 of 21 | the crossing in standing, after the walls (4 of 21 miss) |
| 18 | the snap: down both ramps at three speeds, 0 airborne ticks; the 0.55 step a fall (0.55 > 0.2) | the snap's slope from the centre's ramp only |
| 19 | easing from rest at the steps and ledges (72 runs) | `air_share` applied on the ground |
| 20 | the push: 16 cases (block 6 F) on velocities; slot orders bit-identical bar the one-spot case, its tie by id | the push applied in place in pair order (a slot swap changes the bits) |
| 21 | the oracle (DEPART-4C (a) (1); F3): our circle's sweep against parry3d's cast of the 1.8 m Cylinder, horizontal moves at seeded feet heights. Compared: a box's four vertical faces and four vertical edges; a ramp's two side faces, its exposed high-end face and the vertical edges they share; hit times within 1e-5 and **no hit against no hit** (a miss where the library hits fails). Left out, with why: the ramp's low end and its slope face (our sweep never meets them: a ramp met within its width is the set-back's and the step-up's, 11.3), every top and bottom face (vertical motion is the arc's, not the sweep's), and any contact where our wall test and the face's height at the library's contact point fall on opposite sides of feet + 0.555 (a step for us; counted, not compared). Seeded heights include each shape's bottom and top at plus and minus 1e-4 (both hit or both miss); exactly equal heights counted, not compared | the ramp's side taken at its high end's height; a miss reported as no contact where the library hits |
| 22 | the record and NONE: netcode-architect's restore (P1 to P3) as T-F, with `read_shared(bytes, setup) -> World` and the step lent the same three, the room's pin, N-1 to N-3; my S-1 to S-3 (block 8); one room test on a world wholly past the reach (engine-architect 6.2) | each of theirs as written; NONE turning the facing (S-1); a `clamp_cm` that unwraps an empty box |
| 23 | GROUND T1: on a map whose bounds start at y 2, a soldier spawned 1 m up lands at exactly 2; and the known edge (G9, pinned): the same soldier pressing jump on tick 1 jumps from the air (grounded is set at spawn and the jump, 2b, comes before standing, 2h) | the floor at y 0; the edge changed silently (the tick-1 jump not taken, or taken from the floor) |
| 24 | no allocation: a counting allocator around `authority_step`, 1,800 ticks of 12 scripted soldiers (walk, sprint, jump, step, push, starts inside shapes) | 64 bytes allocated per tick; a sweep collecting into a `Vec` |
| 25 | `visit_shared` names every field (the pattern), 2,902 shared bytes, the row's order | a row field left out (`held_move: _`) |
| 26 | J1 to J5, the jump's rule (3B-ONE-PER-TICK; netcode-architect-read.md:162, moved here from its step 2 by the advisor agent's Q4 condition 5): 255 presses in one grounded command make one jump; a press in the air makes none and none is kept for the landing; Space held jumps again at each landing; two commands in one tick, each +1, one jump; a tap shorter than a tick one jump | a jump per press; presses kept for the landing; the held bit read alone |
| 27 | T-A, the state rule (rules_scan.rs's tokenizer): every field of `struct World` bound by name in exactly one of `visit` and `visit_shared`, no `_` in `visit_shared`'s pattern but `seed: _` | `counts: QueryCounts` in `World` written `counts: _`; `player: PlayerCommand` with `move_x: _, move_y: _, aim: _`; a field bound in both. Control: the built `World` |
| 28 | T-B, one bundle, two builds: two worlds from two loads and two builds of the test bundle, 300 scripted commands: per-tick hashes, `tree_bytes()`, `Rules` and spawn tables equal; from C5 it also asserts the route reaches the 0.55 m box's top (G4) | a `static` counter in `spawn_points` moving each later build's spawn 1 mm |
| 29 | T-C, HEADROOM-FEET on a map with no floating shape (the test bundle's one-box map): equal hashes under both modes at every tick | the mode written into the row at spawn (a `flags` bit) |
| 30 | T-D, the bundle decides (devtools replay tests through `Sim::create`): bundle Y (X with `jump_reach_m` changed) equals X's hashes until the first jump's tick and differs from it on | `Sim::create` taking its movement from `TEST_TEXT` instead of the bundle |
| 31 | T-E, built once: sim-wasm's `step_alloc.rs` around `LocalMatch::step` | `self.statics.clone()` in `Room::step` every tick |
| 32 | T-G, layering, with security-auditor's narrow marking (its third answer; step1-build-order.md C3), one home `crates/sim/tests/support/marking.rs` (not a test target), used by layering.rs and by rules_scan.rs's content panic scan (F5, block 15): test-only is a file starting `#![cfg(test)]` or a top-level `#[cfg(test)] [#[..]] mod NAME {..}`; a mark counts only in item position; a `#[cfg(test)]` on a fn, use or impl marks nothing (a false red on the safe side); raw names stay distinct from keywords; a depth-0 closer the item did not open ends the mark; both crates' `src/` walked recursively. Controls, passing: the real tree; a top-level `#[cfg(test)] mod tests` naming content; a `#![cfg(test)]` file; `content_hash` | each red: T-G's line in a copy of a room file and of an endpoint file; `cfg(any(test, ..))`, `cfg(not(test))`, `cfg_attr(test, ..)`; `extern crate content as c;`, `use ::content as c;`, `use r#content::..`; the four leaks (a `#[cfg(test)] r#type` field before a `content` field, a `#[cfg(test)] r#fn` parameter, a cfg on a const generic, a macro dropping the attribute); a top-level `#[cfg(test)] use` or `fn`; `'{'`, `"{"`, `r#"{"#`, `/* { */` in a test module before T-G's line; `#[path]`, `include!`, `include_bytes!`; the manifest's `c = { package = "content", .. }`, `[dependencies.content]`, a target table; sim's `pub use content::map::MapDocument`; and for F5, `fn f(#[cfg(test)] r#fn: u8) -> u8 { x.unwrap() }` in a copy of a content file (the panic scan red) |
| 33 | B5 (c): a bundle with no movement document and one with two are refused by `Sim::create` with `CreateError::Soldier { found }` (0 and 2); the three bundles of today (test, repository, frozen first-loop) each play their one | the first of several taken silently |
| 34 | W-1 (netcode-architect 9.2): a fresh `World::new` on the test bundle: row 0 at the first spawn by bits, facing (1, 0) by bits, stamina full, `held_move` (0, 0), `aim_cm` NONE, grounded; rows 1 to 47 zero bytes; the tick-0 known answer re-made | `aim_cm` (0, 0, 0) at spawn; stamina 0; the soldier in slot 1; grounded clear at spawn (a jump pressed at tick 1 is dropped: red on a tick-1 jump test) |
| 35 | W-2: a code-built map with spawns "c", "a", "b": the soldier stands on "a"; and "street_10" against "street_2": "street_10" first (byte order, the stated rule, block 15) | the table in file order; a numeric-aware order |
| 36 | W-3: a room stepped 10 ticks before any join (he exists and stands), the join's slot is the occupied row's, then 30 `Input` datagrams with move (127, 0) and NONE move that row as the movement's own test gives | the fill with move (0, 0); the soldier made at the join's tick; the join answer's slot 1 |
| 37 | W-4: a code-built map with no spawn: no soldier; 30 commands with a move and a real aim: the record takes buttons and presses, all 48 rows zero bytes, no panic | the fold writing slot 0's row with no soldier |
| 38 | T2-6 (moved from step 2): moves (127, 0), (0, -127), (89, 89), (-89, -89) taken; (90, 90), (-128, 0), (127, 127) refused and counted; an aim on each face taken, 1 cm past refused; a refused command never runs and its sequence counts as missing; and a world fed (-128, -128) directly moves at most his speed (block 15) | `<` for `<=`; the aim held to the `i16` range; `refused_aim` left out of `refused()`; no cap in the wish |
| 39 | T2-7 (moved from step 2): any_bytes with honest moves inside 127, its three new lies, the counters' identity | an honest move drawn over the whole `i8` range |
| 40 | Q-1 (netcode-architect 9.5): two worlds, T-B's 300 commands, one lent fresh `QueryCounts` each tick, the other counts from u32::MAX - 5: shared hashes equal every tick, no panic | a branch on a count in the step |

### 11.8 The pins, by commit (block 16, G5: the build order C1 to C5)
- C2 re-freezes the first recording's bundle and re-makes the recordings: its evidence is the `.hashes` files byte-identical, with only the headers and the manifest moving.
- C4 (the soldier enters the hashed state) and C5 (he moves) each re-make the pins below. In the text that follows, "the batch's commit" is superseded. What each re-makes:
- world.rs's known answers (:401-415), by the second implementation (a script) writing them again;
- the committed first recording: its frozen bundle first re-frozen (`make_recordings --freeze first-loop`, once content's 13 new values have landed, so the header's hash names the new bundle and the replay builds `Rules` from it; engine-architect block 7 item 4), then the plain run re-making `crates/devtools/recordings/first-loop.*` and `manifest.txt`, with `crates/devtools/tests/recordings.rs` (:24, :29) green on the new files.
- world.rs's known answers now also rest on the test bundle (its spawn and its stamina): a change to the test bundle or a derivation re-makes them in that commit.

CI is green at that commit (the advisor agent's item 7, condition 4). Step 2 then moves no first-loop hash, since NONE lands whole here (block 8).

### 11.9 The count (written lines, code / tests, estimates, plus or minus 30 %)
| Piece | Code / tests |
|---|---|
| ids, soldiers (the row of 11.2) | 165 / 150 |
| rules, with NO-SOURCE's derivations, the refusals and HEADROOM-FEET | 155 / 140 |
| vec, fmath (exp and sincos), the scan's libm rule | 110 / 165 |
| body.rs and movement.rs: the sweep and every rule of 11.3, wish, easing, stamina, jump, facing, the push | 720 / 850 |
| world and the callers | 170 / 110 |
| allocation test | 0 / 90 |
| exp and sincos vectors, maths-allowed entries | 100 / 60 |
| the record, NONE, the bounds' one home (netcode-architect 7 and 8, engine-architect 5 and 6) | 30 / 205 |
| content's 13 fields, the table rows, `--check` | 60 / 60 |
| **Step 1** | **about 1,633 / 2,300 (3,933 lines; block 15: netcode-architect's block 9 +25 / +135 (the room's half and W-1 to W-4, T2-6, T2-7 moved from step 2 to here), the wish's cap +3 / +5, grounded at spawn +0 / +10, the shared narrow marking with its plants +45 / +70, the spawn order's test +0 / +5; block 14: the placement +40 / +130; block 12's two made maps +10 tests; block 13: J1 to J5 +10 / +80, test 15 both modes +10, test 21's faces +15)** |
The movement row (720 / 850) is block 1's sweeps plus movement (690 / 680) re-made as block 10's 2.5D form: blocks 5, 6, 9 and 10. It agrees with the sheet's "about 3,350 in all" (I4b). Not in it: block 2's ray counts and sweep proof (90 / 70, step 1 or 6, the sheet's call).

### 11.10 What this supersedes
- **In block 1:**
  - 1.0's BODY-SHAPE (decided (3a)) and SWEEP-ROUTE (decided: ours, DEPART-4C (a)); route Library is dropped.
  - 1.4 whole: 3D sweeps of a cylinder or capsule become 11.3's 2.5D swept circle.
  - 1.5:
    - the up/across/down step-up becomes 11.3's step-up;
    - the snap by the step-up becomes 0.2 + the slope term;
    - the steepest grade goes: none, row 9 open;
    - the horizontal-then-vertical order becomes vertical first;
    - the in-air step from the tick's start becomes end-of-tick feet;
    - the head stop is kept as the bump.
  - 1.1's `aim: V3` and `since_sprint`.
  - 1.6's "move and aim of the record".
  - 1.8's exp-only list (sincos joins).
  - 1.10's counts.
- **In blocks 6 to 9:**
  - B6-5's "toward from the tick's start" (G2);
  - block 6's C1 and C3;
  - B6-R's `pull: u16`;
  - block 9's push-out wall pass, unswept pull and slope-head window without the 0.55.
- **Still standing:** blocks 1.2 (except its sincos-free maths), 1.3, 1.7, 1.9, 1.11, block 2, block 3's NO-SOURCE table (now 11.4's rows), block 4's risks 4 to 13.

DONE

## Block 12. game-designer's block 8 (N28 to N32): tests off the roofs, the ridge seam, a corner corrected (2026-11-28)
### Plan
1. Re-place 11.7's test 8 (and any other row that needs a roof) on a made gentle raised ramp; show each plant still red there, test 7's included, with the sweep run alone (no push-out after it).
2. The ridge seam (N32): what reading a slope's height past a seam would take, run once as a switch, its cost, now or later.
3. Correct block 10's corner and say what my 0.063 m measures.
4. 11.9's count if it moves. 5. DONE.

### Notes
- [B12-1] Block 10's corner, corrected: the four differing walk runs round the **3.05 m** ledge's corner (x 38, z -4; the 3.05 m ledge spans x 35..38). Block 10 said "the 3.10 m ledge's corner", which is wrong. The paths part at tick 8 at (38.030, -4.399), 0.40 m from that corner. My 0.063 m is the largest per-axis difference (x, z or feet) over the run's ticks. Measured the other way, as the largest horizontal (x, z) distance between the two runs at one tick, it is 0.069 m, game-designer's figure.
- [B12-2] Tests 7 and 8 were re-placed on made maps and run (rr-body-proto-7.rs, group "b12"; the sweep and clip on; plants PLANT_SLAB and PLANT_LOWEND):
  - Test 7, start on a face: the 3.10 m ledge's x 40 face alone, 30 ticks walking into it from push-out's own spot. Right after the sweep, before any push-out, the circle is 0 m into the wall at all three speeds, ending at x 39.599998, push-out's spot to the bit. With the first sweep's slab (no "a >= t0", 1e-6 touch) it is 0.210 / 0.319 / 0.400 m in: red.
  - Test 8, a raised ramp: a 2.6 m block with a 25 % ramp on it (2.6 to 3.6 m, its low end at the block's edge). A running jump from 0.5, 1.0 and 1.5 m out gains the edge and walks up the ramp in 9 of 9 runs, 0 ticks inside. With the ramp swept at its low end: 0 of 9 gained: red.
  - The two rows of 11.7 are rewritten so. No other row needs a roof: test 6 is the yard, 9 the yard's ramps, 10 and 11 made maps, 14 the x 45 seams, 15 to 23 the yard, made maps or the street.
- [B12-3] The ridge (N32), run as a switch (RIDGE_FIX): a ramp's high end counts as exposed only if no other shape meets that edge along the ramp's width with its top within 1 cm of the ramp's top; a high end that another shape continues is left to the set-back and push-out, as the sandbox reads it.
  - With it, block 5's six roof runs at 9.576 m/s are bit-equal to the push-out (were 6 differing, up to 0.51 m). Blocks 5 and 6 are otherwise unchanged against block 10's sweep: ramps from the side 306 of 306, the G6 freestanding ramp still stopped 0.40 m out at every speed.
  - Where the hold can happen at all: it needs a start more than 0.555 / grade from the seam and a move that reaches within 0.4 m. That is a grade above about 0.555 / (0.4 + move): about 91 % at walk and 77 % at sprint, roof grades only. A gentle made ridge never shows it, so its test needs a steep made ridge, which row 9's undecided slope limit could make a wall.
  - Cost: one bool per ramp worked out once in `StaticWorld::build` (`high_end_open`), about 15 code / 20 tests (a made steep ridge, bit-equal to the push-out; plant: the bool forced true, the one-tick hold back).

### Block 12's answers
1. **Tests off the roofs:** test 8 moves to a made 25 % raised ramp (low end 2.6 m) and test 7 to a lone wall face (B12-2); both plants are red there, run. No other row of 11.7 stands on a roof.
2. **The ridge:** reading a slope's height past a seam it can cross takes one bool per ramp, "high end open", worked out once at build, and the sweep leaves a high end that another shape continues to the set-back and push-out. It is measured: the roof runs become bit-equal to his sandbox's push-out, and nothing else moves. It costs about 15 / 20. **Recommended: a follow-up** (STATUS), not now. It bites only on ramp-to-ramp seams steeper than about 77 to 91 %, the roofs' grades today, which MOVE-RULES row 9 keeps open; a slope limit of his could make those ramps walls and the fix moot; and its test would need a steep made ridge whose meaning row 9 decides. It is taken when row 9 is answered, or when the first ramp-to-ramp seam under 77 % appears in a map.
3. **The corner, corrected:** the 3.05 m ledge's corner (x 38, z -4), not the 3.10 m one. 0.063 m is my largest per-axis difference; the largest horizontal distance at one tick is 0.069 m, as game-designer measured (B12-1).
4. **The count:** 11.9 moves to about 1,510 / 1,840 (+10 tests for the two made maps); with the ridge fix, if taken later, +15 / +20.

DONE

## Block 13. The advisor agent's Q2, Q4 and F3 (advisor-steps1to7.md): HEADROOM-FEET (b) as the default, every case re-run; test 15, test 21, J1 to J5 (2026-11-28)
### Plan
1. Re-run blocks 5, 6, 9 (with 10's cases) and 12 on the design's form (sweep and clip on), under (b) and under (a), printed outputs and per-tick bit traces; list every differing case.
2. Rewrite 11.7's numbers that move under (b); test 15 by mode and layout; 11.6's switch.
3. Test 21's faces; J1 to J5 as a row; the recordings under both modes; 11.9. 4. DONE.

### Notes
- [B13-1] Every group was re-run on the design's form (WALK_SWEEP and PULL_CLIP on) under (b) (HEADROOM_ON_OWN_TOP=1) and under (a), printed outputs and per-tick bit traces compared (out-*.txt and trace-*-a/-b.txt under .gate/rr7/; rr-trace-compare.py).
  - Block 5: 10,111 runs, 0 differ by bits, the six roof runs included.
  - Block 6: 1,021 runs, 0 differ.
  - Block 12: 12 runs, 0 differ.
  - Block 9 with block 10's cases: 5 of 329 runs differ, all block 10's H5 layouts. The window wall's top at 6.3 and 9.576: (b) stopped at x 53.6, (a) to x 58. The floor over the crate at both speeds: (b) at x 63.6, (a) to 68. Dropped onto the stretch over the window: (b) put off the wall to feet 0 at z -0.4, (a) stands at 3.0.
  - Nothing else differs, so tests 6 to 14 and 16 to 20 keep their numbers.
- [B13-2] A correction to block 9's B9-2 and to the advisor agent's Q2 condition 3. The beam 1 m into a 0.55 m step flickers 6, 8 and 12 times under (b) only with a push-out wall pass. Under the design's sweep, (b) gives what (a) gives: on the step 0.40 m short of the beam, 1 step, 0 falls, at all three speeds. The reason: the swept circle stops at the beam's face exactly 0.40 m away, so it never overlaps the beam, and headroom needs an overlap. So question 13's sentence "under (b) he flickers off and on 6 to 12 times" holds for his sandbox's own push-out by the prototype's model, not for the game under (b).
- [B13-3] Scan of content/maps/test-scene.json's 23 shapes for every pair where (b) can bite: a floating shape whose underside lies strictly inside the 1.8 m above another shape's top, within 0.8 m of it in plan. None found (the roofs' undersides are exactly the houses' tops, 2.6 m, not inside). So on the test scene the two modes are identical by construction: the walk and lane-walk routes give the same hashes either way, and a flip to (a) moves no committed pin; only test 15's made layouts change.

### Block 13's answers
1. **Under (b), as the default, re-run:** only block 10's three made HEADROOM-FEET layouts differ (5 runs, B13-1); blocks 5, 6 and 12 are bit-identical, the roofs included. 11.7's tests 6 to 14 and 16 to 20 keep their numbers. Test 15 is rewritten by mode, and 11.3, 11.4 and 11.6 now default to `SandboxTest`, with `OwnTopNeverWall` only through `Rules::with_headroom_feet` (`#[cfg(test)]`).
2. **Test 15** (11.7, rewritten), mode by mode and layout by layout:
   - the window wall's top and the floor over a crate: (b) stopped 0.40 m short, (a) walked whole;
   - dropped onto the window stretch: (b) put off the wall, (a) stands;
   - a step with a beam over its edge, and a floating plank: a wall in both;
   - a beam 1 m into a 0.55 m step: no flicker in **both** under the sweep (B13-2). The 6 to 12 falls are the push-out's, pinned by a plant (the wall pass as a push-out under (b)), not by the game's expected numbers.
   - Plants: each mode run with the other's rule.
3. **Test 21** (F3, rewritten):
   - Compared: the box's four faces and four vertical edges; the ramp's two sides, its exposed high-end face and their shared vertical edges; hit times, and a miss against a miss.
   - Left out, with the reason: the ramp's low end and its slope face (the set-back's and the step-up's, never met by our sweep); every top and bottom face (the arc's); contacts that are a step for us; exactly equal heights. Those are counted, not compared.
   - Seeded heights include each bottom and top at plus and minus 1e-4.
4. **J1 to J5** are row 26 of 11.7, +10 / +80 in 11.9; step 2's count drops them.
5. **The recordings:** walk and lane-walk run on the test scene, which holds no case where (b) can bite (B13-3), so their hashes are identical under both modes. A flip to (a) moves no committed pin, only test 15's expected numbers; test 15 checks the two routes' hashes under both modes, so a later map that breaks this shows at once.
6. **The count:** 11.9 moves to about **1,520 / 1,945** (3,465 lines): J1 to J5 +10 / +80, test 15's both modes +10, test 21's faces +15. 11.6's placement of `rules`, `statics` and `spawns` is untouched (block 14).

DONE

## Block 14. engine-architect's block 7 (WORLD-ROOM, the advisor agent's F1): confirmation (2026-11-28)
### Plan
1. Read block 7 (Part A, B1 to B5, items 1 to 6); check its citations against today's code. 2. Confirm or dispute Part A, the 17 changes and B5. 3. Rewrite 11.1, 11.6, 11.7, 11.8 and 11.9 to the placement. 4. DONE.

### Notes
- [B14-1] Citations checked against dae6d107:
  - room/src/lib.rs:85-99: `Room { world, content_hash, joined_at, queue, input, last_run, newest_received, newest_sent, counters }`. :282 is `self.world.authority_step(run.get(..ran).unwrap_or(&[]))`; :286-296 reads `player.buttons` and `player.presses` into `OwnState`.
  - sim-wasm/src/lib.rs:25: `LOCAL_MAP = "test-scene"`, with its doc "A mode that names its map comes with a later change-set" (:23-24). :83-86: `local_world`, the one map lookup and `StaticWorld::build`.
  - local_match.rs:48-50 holds `world: StaticWorld` ("No step queries it yet"); :96-100 `static_world()`.
  - collision.rs:101-108: `narrow`, private today.
  - content's bundle.rs:53 and :65: `movements: Vec<Movement>` and `movements()`, so (c)'s "the one movement document" reads a list that exists.
  - All as block 7 says.

### Answers
1. **Part A: YES, A1 to A5.**
   - A constant inside `World` would either be hashed with content the digest already names or ride every copy unhashed. Lending it per call keeps `World` free of a lifetime, and the room's disjoint field borrows (`&self.rules`, `&self.statics`, `&mut self.world`) need no unsafe and no `RefCell`.
   - The plain form (A5) is enough: three fields, three arguments, and `LocalMatch::new` at 7 arguments sits at clippy's `too_many_arguments` limit (it fires above 7), not over it.
   - **The 17 changes: YES to each, none disputed:** (1) to (3), (4) the step's signature, (5) `SpawnPoint` and `spawn_points` (the `Vec` is built once in `Sim::create`, not in the step, so 11.7's test 24 stays clean), (6) `narrow` as `pub(crate)`, (7) `QueryCounts` in the room, (8) the re-exports, (9) to (12) the room, sim-wasm and the three test callers, (13) and (14) nothing more, (15) layering.rs on non-test tokens, (16) the rows, (17) the superseded lines (1.6's "into the room's `World`" and block 2's accessor here; netcode-architect's `read_shared(bytes, setup)` is its own).
   - **One addition from item 4, taken into 11.8:** the frozen first-loop bundle is re-frozen (`make_recordings --freeze first-loop`) once content's 13 new values land, before the plain run, in the same commit.
2. **B5 (c): YES.**
   - The match plays the bundle's one movement document and refuses none or several with `CreateError::Soldier { found }`.
   - It works on all three bundles of today with no edit to the 49 "test-soldier" lines, no change to the recording's format, and no batch file; it follows `LOCAL_MAP`'s own rule (a mode that names its soldier comes with the second movement document).
   - Test: 11.7 row 33, with its plant.
3. **11.1, 11.6 and 11.8 are rewritten** in place:
   - the step's signature;
   - `PlayerRecord` (it changes no byte `visit_shared` writes: the record's buttons and presses as before, so 11.2 stands);
   - `SpawnPoint` and `spawn_points`;
   - the room's four fields and two accessors;
   - `local_constants` and `CreateError`'s two variants;
   - `LocalMatch::new`;
   - the removals;
   - the re-freeze.
   - Superseded: 11.6's earlier "the match's constants ... which are not state" in `World`, block 1.6's "into the room's `World`", block 2's accessor on the world.
4. **11.7:**
   - T-A, T-B, T-C, T-D, T-E and T-G are new rows 27 to 32, with their plants; B5 (c) is row 33.
   - T-F is my row 22 (netcode-architect's restore), now with `read_shared(bytes, setup)` and the step lent the same three.
   - My row 24 (allocation around `authority_step`) stays beside T-E: they see different layers.
   - Row 26 (J1 to J5) is back in its place.
5. **11.9:** about **1,560 / 2,075 (3,635 lines)**, with the placement's +40 / +130.

DONE

## Block 15. netcode-architect's block 9, security-auditor's third answer and F5, game-designer's block 9 (2026-11-28)
### Plan
1. Read netcode-architect-read block 9 (9.2 to 9.5), security-auditor-deps-4c-v2.md's third answer and F5 (:180-192), step1-build-order.md C3, game-designer-read block 9 (N37). 2. Fold them into 11.1, 11.6, 11.7 and 11.9; settle grounded at spawn, F5 and the spawn order. 3. DONE.

### Notes
- [B15-1] Checked against today's code:
  - room/src/lib.rs:161-162 names slot 0 and team 0 by literal;
  - the fill at :216-223 takes buttons and presses only;
  - protocol's `Command` already carries the move and the aim, and the codec writes and reads them (codec.rs:207-234);
  - map.rs:92 says a spawn's `at` is "the point on the ground between the feet";
  - rules_scan.rs:699 `in_tests` is the marking the content panic scan uses at :767;
  - content's only `#[cfg(test)]` items are top-level mods (lib.rs:33-37, `mod schema;` and `mod ts_export;` among them), so the narrow marking marks content's tree as today's does;
  - schema.rs and ts_export.rs do not start with `#![cfg(test)]`, so both markings read them as game code, today and after.

### Answers
- **(a) netcode-architect's block 9: taken whole, nothing disputed.**
  - B4 (a): yes, with its three conditions. `PLAYER_SLOT` and `PLAYER_TEAM` live in sim's world.rs and are read by `World::new` and the join answer. The fold on a slot with no soldier takes the record and touches no row (11.1 item 1). Grounded at spawn: below.
  - 9.3, step 1 carries the wire's move and aim through the room with both checks. Taken into 11.6: `MOVE_REACH`, `move_in_reach`, the fill, `refused_move` and `refused_aim` with `refused()`, the doc, the NONE tie, messages.rs:117's sentence and any_bytes. 11.1 2a now says "refused beyond it by the room".
  - My call on a world fed directly: **the wish caps a longer pair at length 1**. It is one compare and one division by the pair's own length, deterministic on both targets, so no caller of `authority_step` can move him faster than his speed (row 38's last case and plant).
  - Q-1 is taken (row 40); the step only writes `QueryCounts`, with wrapping additions.
  - W-1 to W-4, T2-6 and T2-7 are rows 34 to 39.
- **(b) Grounded at spawn: set.**
  - The map's contract is that `at` stands on the ground (map.rs:92). `World::new` is lent no static world (engine-architect block 7), and tick 1's standing corrects any mismatch: a top within the snap is stood on, a larger drop starts a fall from rest. So no state ever holds a floating grounded soldier past tick 1.
  - Clear would cost a jump pressed on tick 1 (dropped: jumps start only from the ground) and give nothing. On the floor both give the same tick-1 state, since a clear spawn falls 13 mm and lands within the tick.
  - Test: W-1's plant (grounded clear at spawn; a tick-1 jump then fails). A spawn above the ground is a map's error that no check of ours refuses today; not in 4c.
- **(c) Row 32** is rewritten to security-auditor's narrow marking with its full lists of plants and controls (C3).
- **(d) F5: the same commit, one home.**
  - The narrow marking is written once in `crates/sim/tests/support/marking.rs`, a support file and not a test target. layering.rs (devtools, which depends on sim) and rules_scan.rs's content panic scan include it, the panic scan in place of `in_tests` for content (rules_scan.rs:767).
  - Content's tree holds only top-level `#[cfg(test)]` mods, so nothing turns red today.
  - Cost: about +5 code (the include and the switch) / +10 tests (F5's plant, `fn f(#[cfg(test)] r#fn: u8) -> u8 { x.unwrap() }`, red, and today's tree as its control), on top of the marking's own +40 / +60, counted once.
  - Not a STATUS follow-up: one parser in two homes would drift, and it closes both findings at once.
- **(e) The spawn order: byte order of the id, as the shapes are ranked (collision.rs:13-14).** It is stated as the rule and pinned by W-2: "c", "a", "b" stands on "a", and "street_10" comes before "street_2". It is a build default until the barracks (change-set 5; game-designer N37: not his). A numeric-aware order would add a second ordering rule beside the shapes' for a choice that rule then retires. The early look's Brave step says where he starts and why (game-designer N37).
- **11.9:** about **1,633 / 2,300 (3,933 lines)**. Over netcode-architect's 1,585 / 2,210 it adds the wish cap +3 / +5, grounded at spawn +0 / +10, the shared narrow marking +45 / +70 and the order test +0 / +5.

DONE

## Block 16. The advisor agent's step 1 delta (advisor-delta-step1.md): G1 and the build order C1 to C5 (2026-11-28)
### Plan
1. Read advisor-delta-step1.md and step1-build-order.md C1 to C5. 2. G1 yes or no. 3. Fold G2 to G10 into 11.1, 11.6, 11.7 and 11.8. 4. One table, symbol by symbol and row by row, per commit. 5. DONE.

### Notes
- [B16-1] G1: nothing in 4c reads the spawn table after `World::new`. No respawn exists before change-set 5 (netcode-architect 9.2), no accessor is planned (engine-architect block 7 item 3), and the step never reads a spawn. So a kept `Vec<SpawnPoint>` is a dead field: rustc's "field is never read", red under `-D warnings` (local-checks.sh:14).
- [B16-2] G3 reaches one more field than the advisor agent listed: `Room.rules`. In C4 the soldier stands and the step reads no rule (the fold reads none), and `World::new` borrows `Rules` only while it builds. So in C4 `Room::new` borrows `&Rules`, and the `Room` owns `rules: Rules` only from C5, when `authority_step` first reads it; `query_counts` enters with it. `Room.statics` enters in C4, read by `holds_cm` in `on_datagram`. The cost is one signature change between C4 and C5 (`Room::new` and `LocalMatch::new` take `rules` by reference, then by value), with the same callers touched twice.
- [B16-3] Dead code and `pub`: sim's modules are `pub` (lib.rs:18-23), so a `pub` item in them draws no dead-code lint. C1's V3 helpers, `resolve_mut`, the `Soldier` fields, and C3's `Rules::take` and `spawn_points` are therefore `pub` and may wait for their first non-test reader. A `pub(crate)` item enters with its first reader: `narrow`'s `pub(crate)` lands in C3 with `spawn_points`.
- [B16-4] G8: `crates/devtools/tests/layering.rs` takes the shared marking as `#[path = "../../sim/tests/support/marking.rs"] mod marking;` (the path is relative to the including file's directory, crates/devtools/tests/). `crates/sim/tests/rules_scan.rs` takes it as `#[path = "support/marking.rs"] mod marking;`. `tests/support/` is no test target (Cargo makes targets only of files directly in tests/). layering.rs's own `#[path]` plant walks only room's and the endpoint's `src/`, so these two test-side includes do not trip it.

### Answer
- **G1: YES.** `Room::new(.., spawns: &[SpawnPoint])` and `LocalMatch::new(.., spawns: &[SpawnPoint])` borrow the table for `World::new` and keep nothing; `Sim::create` drops it after; no `#[allow(dead_code)]`. 11.6 is rewritten so (and `rules` is borrowed in C4, owned from C5: B16-2).
- **G2, G3, G10, G5, G4, G8, G9:** folded. 11.6's room and sim-wasm lines are rewritten; 11.8 follows the order; row 23 pins G9's edge and 11.1 states it; row 28 asserts the box's top from C5; the include is B16-4.
- **Per commit** (each passes local-checks.sh whole, clippy `-D warnings` with no new `allow`; pins re-made in C4 and C5):
| Commit | 11.6's symbols | 11.7's rows |
|---|---|---|
| C1 | sim `ids.rs` (`SoldierId`, `to_bits`, `from_bits`, `NONE`); `soldiers.rs` (`MAX_SOLDIERS`, `Soldier` with `pub` fields, its flags, `Soldiers`, `spawn`, `despawn`, `resolve`, `resolve_mut`, `iter`); `aim.rs` (`AimCm`, `NONE`); `vec.rs`'s V3 helpers (`pub`); `lib.rs`'s modules | 1 |
| C2 | content `movement.rs` (FIELDS 15 → 28, `Movement`, the schema, the generated types, the test text); `content/movement/soldier.json` (13 values); `content/source/testlines-table.json`'s rows and `testlines.mjs --check`; the re-freeze and the recordings re-made with `.hashes` byte-identical | content's own field and version tests; the table's planted row |
| C3 | the batch (`deps-4c-v2.diff`, digests checked); `fmath.rs` (`share_per_tick`, `facing_of`, `DEG_TO_RAD`, the float helpers); `rules.rs` (`Rules`, `Rules::take`, `RulesError`, `HeadroomFeet`, `HEADROOM_FEET` = `SandboxTest`, `with_headroom_feet` under `cfg(test)`); `soldiers.rs`'s `SpawnPoint` and `spawn_points` (sim's tests their only callers); collision.rs's `narrow` as `pub(crate)`; F1's `dependency_rules.rs`; `tests/support/marking.rs` with layering.rs and rules_scan.rs's content panic scan switched to it (F5); the libm scan rule; the exp and sincos vectors | 2, 3, 4, 5, 32; 35's order half |
| C4 | world.rs: `PLAYER_SLOT`, `PLAYER_TEAM`, `MOVE_REACH`, `move_in_reach`, `PlayerRecord` and `NONE`, `World { tick, player, soldiers, seed }`, `World::new(setup, &Rules, &[SpawnPoint])`, `PlayerCommand`'s move and aim, the fold with NONE and the empty-slot rule, `visit` and `visit_shared` (2,902 bytes), `StateSink::f32` with `Canonical`, devtools `Lines` and the test sinks; collision.rs `bounds_cm`, `holds_cm`, `build`'s cm derivation, `WorldError::Bounds`; content map.rs `on_grid` `pub` and GROUND's words; room: `statics` field, `Room::new(.., &Rules, statics, &[SpawnPoint])`, `Room::statics()`, the fill, `refused_move`, `refused_aim`, `refused()`, the doc, the NONE tie, the join reading `PLAYER_SLOT` and `PLAYER_TEAM`, queue.rs's literal, any_bytes; protocol `AimPoint::NONE` and messages.rs:117's sentence; endpoint.rs:258's NONE; sim-wasm `local_constants`, `CreateError::Rules` and `::Soldier`, `LocalMatch::new(.., &Rules, statics, &[SpawnPoint])` without `world` and `static_world()`; the test callers; `maths-allowed.mjs`'s exact entries (the players' build first prints exp and sincos here); lib.rs's re-exports; pins re-made | 22 (T-F's restore, N-1, the room's pin; S-1 to S-3's spawn halves), 25, 27, 28 (hashes), 31, 33, 34, 35's stand half, 36's stand-and-join half, 37, 38 bar its direct-feed case, 39 |
| C5 | collision.rs `bounds_m`, `floor_y()`, `candidates`; `body.rs` (the sweep and every rule of 11.3, `QueryCounts`); `movement.rs` (`step_soldier` 2a to 2i with the wish's cap, `push_pass`); `authority_step(&mut self, &Rules, &StaticWorld, &mut QueryCounts, commands)` in 11.1's order; room: `rules: Rules` owned and `query_counts`, `Room::new` taking `Rules` by value, `Room::query_counts()`, :282 lending the three; `LocalMatch::new` taking `Rules` by value; pins re-made | 6 to 21, 23, 24, 26, 28's box top, 29, 30, 36's moving half, 38's direct-feed case, 40; 22's facing halves (S-1 to S-3 under the movement) |

DONE

## Block 17. C3's exp and sincos: the vectors' form (Q1), the rate signature (Q2), ads (Q3), the shares' 1 ulp domain (Q4), the vector files (Q5) (2026-11-28)
### Plan
1. Measure with rr7's two libm 0.2.16 rlibs (scratch under .gate/rr17/): `libm::exp` and `libm::sincos` f64 bits over the proposed inputs natively and in wasm32 under Node; Python decimal for the correctly rounded f64 (count off, largest ulp) and for Q4's shares (easing T in (0, 5], push strength in [0, 100] swept toward 0).
2. Q5's file form from that count: make_f32_vectors.py's pins (computed or recorded); edges from libm's own branch constants; the four call sites and wasm-rules.spec.ts:11's list.
3. Q1: F1 and the `pub mod`; the cost (rem_pio2_large on a raw sincos); the scan rule's wording, bites, controls; the gap outside sim's src; one scratch build of the players' twins (release and dev) from a git-archive copy with the batch and the drafts in Q1's form, `twinProblems` plus a positive control; clippy once on that copy.
4. Q2 (strength 0), Q3 (what the multiplier multiplies), row 2's plant on distinct values.
5. Answers, count delta, DONE.

### Notes
- [B17-1, measured] Scratch .gate/rr17/ (make_inputs.py, dump.rs, dumpwasm.rs, runwasm.mjs, ref.py, twin.py), rr7's libm 0.2.16 rlibs, rustc 1.99.1, Node 22. Inputs as share_per_tick and facing_of form them: exp 33,648 (his two values; easing T = k/1000 on (0, 5]; push s = k/100 on [0, 100]; push 2^(-e/4) down to 2^-1074; easing T from 5 down by 2^(-1/4); 10,000 seeded; exp.rs's branch edges 0x3e300000, 0x3fd62e42/43, 0x3ff0a2b2, 0x4086232b, -708.396..., -745.133..., +-0, -inf, each with its neighbours), sincos 100,017 (the 0.001 grid on [0, 90) x K, 10,000 seeded facings through the quarter turns, sincos.rs's and rem_pio2.rs's edges 0x3e46a09e, 0x3fe921fb/fc, 0x3ff921fb (medium path) and its neighbour, 45 x K, the rests of the doubles just under 90, 180, 270 and 360, the smallest subnormal and normal). Native against wasm32 in Node, the caller at -O and at opt-level 0: **0 of 33,648 exp and 0 of 200,034 sin/cos outputs differ (cmp, byte-equal files).**
- [B17-2, measured] Against the correctly rounded f64 (Python decimal, 60 digits): exp 1,788 of 33,648 off, largest 0.8148 ulp (x = -3.081); sin 3,064 and cos 3,093 of 100,017 off, largest 0.7539 and 0.7178 ulp. Narrowed to f32: 0 of 200,034 off the correctly rounded f32 of the double's sine and cosine (the route's reference, the coordinator's point). So libm is not correctly rounded in f64 (its own doc: "less than 1 ulp"): an expected file cannot be the reference's bits as f32-expected.txt's are, and per-case pins would be about 7,900 lines.
- [B17-3, measured] A Python twin of libm 0.2.16's exp (exp.rs:84-155, with math.ldexp for scalbn) and sincos (sincos.rs, k_sin.rs, k_cos.rs, rem_pio2.rs's fast and medium paths; rem_pio2_large not twinned, out of the domain) in CPython floats (each + - * / rounded once, no contraction): **133,665 of 133,665 inputs bit-equal to libm native** (twin.py, 0.2 s). A standard-library generator can therefore derive the expected bits itself, and `--check` can rebuild both files byte for byte.
- [B17-4, Q4 measured] Shares (1.0 - libm::exp(x)) as f32 against the correctly rounded f32 of 1 - e^x: easing T = k/1000 (5,000) 0 off, largest 0.4998 ulp; easing T from 5 down to 1e-300 (4,300) 0 off; push s = k/100 on [0, 100] (10,001) 0 off, largest 0.4999 ulp; his values: ease 3ebf5c1e (0.1011 ulp), push 3e6fb07b (0.3069 ulp), both the reference's bits. Toward zero (push 2^(-e/4)): 467 cases at or over 1 ulp, the largest |x| among them 2.953e-10, a strength of 8.86e-9 per second; above that edge 19,409 cases, 10 off, largest 0.8633 ulp. Strength 0 gives x = -0.0, exp 1, share exactly 0. Cause: under |x| = 2^-28 libm returns 1 + x (exp.rs:145-148), so 1 - exp(-x) is x on a 2^-53 grid.
- [B17-5, measured] Players' twins, built offline with build.mjs's own cargo line (`cargo build --locked --release|dev --target wasm32-unknown-unknown -p sim-wasm`) from a copy of the worktree's C3 work in progress (uncommitted, 2026-11-28 16:2x) with fmath.rs put into Q1's form (exp_of, sincos_of; share_per_tick and facing_of naming no libm), read by build-checks.mjs's own `twinProblems` and MATHS_ALLOWED (scratch twins.mjs): release 400 functions, 0 libm names, 0 problems; dev 2,874 functions, 11 names that end in libm's crate (core generics instantiated for it, no libm path), 0 problems. HEAD 14538bd7 (git archive) gives the same: 400 / 0 and 2,874 / 11, 0 problems. **In C3 the players' file links no libm routine; the maths list needs no entry.** Positive control (an exported sim-wasm function calling share_per_tick and facing_of): release 4 problems (`libm::math::exp::exp`, `sincos::sincos`, `rem_pio2::rem_pio2`, `rem_pio2_large::rem_pio2_large`), **dev 29 problems** (also k_sin, k_cos, scalbn, generic::scalbn, floor, generic::floor (+ floor_status), get_high_word, rem_pio2::medium, rem_pio2_large::floor, support::int_traits, support::float_traits, support::env).
- [B17-6, C4 flag, Medium] So C4's "expected four entries" is the release twin only: build.mjs:189-199 reads both twins, and the dev twin needs about 14 entries by the list's prefix rule (build-checks.mjs:777-778): libm::math::exp, ::sincos, ::rem_pio2 (covers ::medium), ::rem_pio2_large (covers ::floor), ::k_sin, ::k_cos, ::scalbn, ::generic::scalbn, ::floor, ::generic::floor, ::get_high_word, ::support::int_traits, ::support::float_traits, ::support::env; each with its reason. Not C3's.
- [B17-7, measured] Clippy (`cargo clippy --workspace --all-targets --locked -- -D warnings`) on that copy: exit 0, no warning. `cargo fmt --all -- --check` on it: not clean in the work in progress (rules.rs 23 hunks, devtools/tests/dependency_rules.rs 4, soldiers.rs 1): a `cargo fmt` before the commit, nothing else.
- [B17-8] RR4 corrected: for x = r x K in (pi/4, pi/2) sincos takes rem_pio2's |x| <= 3pi/4 fast path (rem_pio2.rs:93-99), not the medium one; medium only when the high word is 0x3ff921fb (rem_pio2.rs:89-91), i.e. r above about 89.9999991 degrees, which the 0.001 grid never reaches. The edges reach it (0x3ff921fb00000000, (just under 90) x K, and the rests of the doubles just under 90, 180, 270 and 360), and the latter run medium's second and third rounds. rem_pio2_large is reached only by |x| >= 2^20 pi/2: never by facing_of on [0, 360), but by a raw sincos on any double, and by facing_of on a degree far outside [0, 360) (content refuses those, map.rs:66).
- [B17-9] The coordinator's reference point: confirmed. libm's own output at rest(just under 360) gives x = a688d313, z = 3f800000 (my dump). The vectors' reference is sin and cos of the double the routine receives (the route's), and that is what B17-2's "0 of 200,034 narrowed off" measures; B7-2's "0 off" was against the true angle on the 0.001 grid, where both agree. RR3 stands; the vectors' edges already hold that case.
- [B17-10] Row 2's planted defect ("ease_stop from the speed-up time") cannot bite on either table today: soldier.json:23,30 has 0.164 and 0.164, movement.rs:163 TEST_TEXT has 0.25 and 0.25. Row 2 needs a table with the two times apart (e.g. TEST_TEXT with one changed in the test's own copy), or the plant survives.
- [B17-11] Q3's fact: testlines-table.json:109-113 says ads_speed_multiplier is "walking speed while aiming" (old game T.adsMoveMult); the old game sets the speed factor to adsMoveMult in place of the sprint factor (shamble-game.js:2774): walk x multiplier, as sprint = walk x its multiplier.

### Answers
**Q1 (the vectors' form): YES, on one condition: the raw-name rule below goes into the scan in C3.** No batch change: devtools already depends on sim (crates/devtools/Cargo.toml `sim = { path = "../sim" }`), `fmath` is `pub mod` (lib.rs WIP), and devtools names `sim::fmath::exp_of`, never `libm`, so F1 ("only sim names libm") holds. The vectors then compare libm's own f64 bits, which PM-0004-22 asks ("for each libm function the sim uses ... bit-identical outputs (to_bits)"). The f32 route would compare narrowed results only. That asks less (B17-2: 6,157 of sincos's f64 outputs are off the correctly rounded value and 0 are after narrowing, so a one-target difference in the low bits could hide under the narrowing), so it would be a departure.
- Symbols (fmath.rs):
  - `pub fn exp_of(x: f64) -> f64 { libm::exp(x) }`, doc: "libm 0.2.16's exponential, its own result; called by `share_per_tick` and, outside the simulation, by the vectors (PM-0004-22) only".
  - `pub fn sincos_of(x: f64) -> (f64, f64) { libm::sincos(x) }`, the same doc with `facing_of`.
  - `share_per_tick` calls `exp_of`; `facing_of` calls `sincos_of`.
  - `facing_of`'s doc names its domain [0, 360), as map.rs:66 guarantees: outside it, the rest leaves [0, 90) and could reach rem_pio2_large (B17-8).
- The scan (rules_scan.rs), with its WIP names:
  - `MATHS_CALLS` becomes `[("exp_of", "exp"), ("sincos_of", "sincos")]`.
  - A new `RAW: [(&str, &str); 2] = [("exp_of", "share_per_tick"), ("sincos_of", "facing_of")]` with the wording: "libm's two routines as the simulation reaches them, by their own names: each name may stand only in `fmath.rs`, and there only in its own `fn` and in the body of its one caller. A name counts wherever it stands as a whole token (a call, a path, `use`, `as`, `pub use`, a function pointer, a macro's argument), test code included, so no file but `fmath.rs` reaches them and no other function there does. Outside `src/` the vectors call them (crates/devtools), and nothing else may; `layering.rs` holds that for the crates that build the simulation's callers."
- Bites (each red):
  - `exp_of(` in `facing_of`;
  - `sincos_of(` in `share_per_tick`;
  - `crate::fmath::exp_of(1.0)` in world.rs;
  - `use crate::fmath::exp_of as e;` in rules.rs;
  - `use crate::fmath::{sincos_of};` in soldiers.rs;
  - `pub use fmath::exp_of;` in lib.rs;
  - `let f: fn(f64) -> f64 = crate::fmath::exp_of;` in collision.rs;
  - `use crate::fmath::*;` plus a bare `sincos_of(x)` in world.rs;
  - `exp_of` in a `#[cfg(test)] mod tests` of fmath.rs;
  - `libm::exp(` moved back into `share_per_tick` (now outside `MATHS_CALLS`);
  - the old B7-1 bites (`libm::expm1(`, `libm::sincosf(`, `use libm::{exp};` with a bare `exp(`, `use libm::*;`, libm in a second file).
- Controls (passing):
  - the real tree;
  - each raw function's own definition and its one caller's call;
  - the names in a comment and in a string;
  - `exp_offset` and `my_exp_of` (whole-token compare).
- Outside sim's src (C4, Medium): from C4 the maths list holds exp and sincos, so the built-file check no longer stops a call from sim-wasm, room or client-endpoint. In C4, `layering.rs` refuses the tokens `exp_of` and `sincos_of` in those three crates' `src/` (it already walks room's and the endpoint's; add sim-wasm). Its plant: one call in room's src.
- Optional, Low (the "once per match" reason up one level): `share_per_tick` only in `Rules::take`, and `facing_of` only in `spawn_points`, test-only code (marking.rs) exempt.
- The players' file in C3 (B17-5, measured): release 0 libm names and dev 0 libm paths, 0 problems, the same as HEAD. The positive control is red. So the maths list stays at its 2 entries in C3.
- `maths-allowed.mjs:9` ("The simulation itself calls no maths routine") reads false at source level from C3. Reword it in C3 (Low) to: "the simulation's two (fmath.rs, exp and sincos) are not linked into the players' file until its constructor calls them (change-set 4c's C4)". C4's entries: about 14, not 4 (B17-6).

**Q2 (rate signature): YES.** `push_strength_per_s` is a rate, closed at 0 (movement.rs:35). A time signature has no finite T for strength 0, while the rate form gives x = -0.0, exp 1, share exactly 0 (B17-4). The easing's rate LN_10 / T stays finite for every T in (0, 5] that is not subnormal; a subnormal T gives inf, then x = -inf, exp 0, share 1, no NaN (measured, the "T from 5 down" sweep). The argument's rounding order, (LN_10 / T) x dt rather than LN_10 x dt / T, is a choice. Row 2's reference uses the code's own double x, as the route's reference does for sincos.

**Q3 (ads as a speed): YES.** The table's row says "walking speed while aiming" (testlines-table.json:109-113), and the old game puts adsMoveMult in place of the sprint factor (shamble-game.js:2774). So `ads = (walk x ads_speed_multiplier) as f32` in f64, as sprint is. It is a `pub` field with no reader until 4d, and it draws no lint (B16-3). 4d's rules (ADS while sprinting, the old game's banzai and shield factors) stay 4d's. Row 2 checks its bits.

**Q4 (the shares' 1 ulp domain), measured (B17-4).** On the code's own x, against the correctly rounded f32 of 1 - e^x (decimal, 60 digits):
- the easing over T in (0, 5] (x <= -LN_10 / 150): 0 off over 9,300 cases;
- the push over strengths 0 and [0.01, 100] (10,001 cases): 0 off, largest 0.4999 ulp;
- his values: 3ebf5c1e (ease) and 3e6fb07b (push), each the reference's bits;
- toward zero, the push stays under 1 ulp for strengths above 8.86e-9 per second (|x| > 2.953e-10): above that edge 10 off and largest 0.8633 ulp over 19,409 cases; at or below it, 467 cases at 1 ulp or more. The cause is exp.rs:145-148 (1 + x under 2^-28).
- Row 2's words: "within 1 ulp of f32 for every easing time in (0, 5] and for a push strength of 0 or above 1e-8 per second; correctly rounded (0 off) on his values and on both grids". Below 1e-8 the share is still the same bits on every target, only less exact: no refusal is needed.

**Q5 (the vector files).** Generator `crates/devtools/vectors/make_libm_vectors.py`: Python standard library only (struct, random, math, decimal, hashlib, pathlib), run with `-I -B`, with `--check` (byte for byte) and `--full <routine>`.
- It holds a twin of libm 0.2.16's exp and sincos: exp.rs, sincos.rs, k_sin.rs and k_cos.rs, and rem_pio2.rs's fast and medium paths. It is twin.py, measured bit-equal on all 133,665 inputs (B17-3). So the expected bits are derived, not recorded. A twin miss fails natively on the first run.
- Inputs: `libm-inputs.bin`, f64 LE, 1,069,320 bytes.
  - exp, 33,648: his two values; easing T = k/1000; push s = k/100; push 2^(-e/4) to 2^-1074; T from 5 down; 10,000 seeded; exp.rs's branch edges 0x3e300000, 0x3fd62e42/43, 0x3ff0a2b2, 0x4086232b, -708.396..., -745.133..., +-0, -inf, each with its neighbours.
  - then sincos, 100,017: the 0.001 grid x K; 10,000 seeded facings through the quarter turns; the edges 0x3e46a09e, 0x3fe921fb/fc, 0x3ff921fb00000000 and its neighbour, 45 x K, (just under 45) x K, (just under 90) x K, the rests of the doubles just under 90, 180, 270 and 360, the smallest subnormal and normal.
  - Add 1,000 seeded rests in (89.9999991, 90) for medium's rounds (B17-8).
  - No NaN.
- Expected file: `libm-expected.txt`.
  - Its first line: the inputs' byte count and sha256, as f32-expected.txt:1.
  - One line per routine with its case count and the FNV-1a 64 of all outputs' LE bits in order: `exp 33648 <digest>`, `sincos 100017 <digest>` (sin and cos interleaved, as the routine returns them).
  - `report` lines, recomputed by `--check` against decimal at 60 digits: exp 1,788 off the correctly rounded f64, largest 0.8148 ulp; sin 3,064, 0.7539; cos 3,093, 0.7178; narrowed to f32, 0 of 200,034 off the route's reference (the double's sin and cos). PM-0004-22's "largest error in ulp" is reported, not pinned.
  - No pin lines: native = wasm measured (0 of 233,682 outputs differ, B17-1).
- Rust: `crates/devtools/src/rules/libm_ops.rs` (`EXP_CASES`, `SINCOS_CASES`, `check(inputs, expected) -> Result<(), String>`, through `sim::fmath::exp_of` / `sincos_of` and `sim::state_hash::fnv1a_64`; a NaN input or a wrong length is refused). The other entries:
  - `wasm.rs`: `#[wasm_bindgen] pub fn libm_vectors(inputs: &[u8], expected: &str) -> String`;
  - native: `crates/devtools/tests/libm_vectors.rs`;
  - Node: `rules.test.mjs`, beside :16-31, with its changed-input control;
  - browsers: `wasm-rules.spec.ts:11`'s VECTORS gains the two files, and the outcome gains `libm: ""` and `libmChanged`;
  - `local-checks.sh`'s generator line gains `make_libm_vectors.py --check`, and CI's line the same.
- Planted defect (PM-0004-22's bite): in libm_ops.rs, `if cfg!(target_arch = "wasm32") && case == 0 { bits ^= 1 }`. Node and each browser go red, naming `exp`, and the native test stays green. That shows the per-target comparison catches a one-target 1-ulp move. Second plant: the twin's exp loses its `1.0 + x` branch, and `--check` goes red.

**Count delta (code / tests, against 11.9's "exp and sincos vectors, maths-allowed entries 100 / 60"):**
- fmath +12 / 0;
- scan's raw rule 0 / +45;
- libm_ops.rs and wasm.rs +75 / 0;
- native, Node and browser tests 0 / +55;
- generator 0 / +230;
- maths-allowed moves to C4 at about 14 entries (+45 there);
- C4's layering.rs addition 0 / +10.
- C3 net: about -13 / +270 against that row, plus two data files (1,069,320 bytes of inputs; about 8 lines of expected).

**Anything that blocks C3's commit:**
1. Row 2's ease_stop plant cannot bite on either table (B17-10, High if left: a surviving plant). The test needs the two times apart.
2. `cargo fmt` (B17-7).
3. The raw-name rule must be in the scan with Q1 (or keep libm in share_per_tick and facing_of, and the vectors become a departure).
4. Row 2's 1 ulp words need Q4's domain.

Clippy -D warnings is clean (B17-7). Not run:
- browsers (Node only);
- the real build.mjs (its cargo line was run by hand, without the path remap flag, which does not change names);
- libm's cost against std's (PM-0004-22's "reported" cost).

DONE
