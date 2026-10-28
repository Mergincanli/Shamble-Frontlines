# R4-PHYS draft: Rapier 3D or custom (engine-architect, 2026-10-28)

All sources read 2026-10-28. Tier: official / shipped game / other.

## Findings

### Rapier's character controller (rapier3d 0.36.1)
- **R4-PHYS-F1.** Rapier's character controller is not a physics body: it is a helper that issues ray casts and shape
  casts to correct a movement the caller asks for. Quote: "a higher-level tool that will emit the proper ray-casts and
  shape-casts to adjust the user-defined trajectory". Source: https://rapier.rs/docs/user_guides/rust/character_controller
  (guide navigation "User Guide: Rust (0.36)"; no page date). Tier: official. Sure: confirmed in the source.
- **R4-PHYS-F2.** It climbs steps and small obstacles (autostep, off by default), snaps to the ground, slides on
  slopes up to a maximum climb angle, and keeps a small offset from surfaces. Quotes: "the **autostep** setting allows
  the character to climb stairs automatically and walk over small obstacles."; "**snap-to-ground** will force the
  character to stick to the ground if the following conditions are met."; field `autostep`: "Should the character
  automatically step over small obstacles? (disabled by default)"; field `max_slope_climb_angle`: "The maximum angle
  (radians) between the floor’s normal and the `up` vector that the character is able to climb."; field `offset`: "A
  small gap to preserve between the character and its surroundings." Sources: the guide page above and
  https://docs.rs/rapier3d/latest/rapier3d/control/struct.KinematicCharacterController.html (rapier3d 0.36.1). Tier:
  official. Sure: confirmed in the source.
- **R4-PHYS-F3.** The controller holds no per-character state, so one instance can move every soldier; the call takes
  the time step, a query pipeline, a shape, a pose and the wanted translation. Quotes: the configuration "does not
  contain any collider-specific or rigid-body-specific information like handles, velocities, positions, etc."; the same
  instance "can be used to control multiple rigid-bodies/colliders if they rely on the same set of parameters.";
  `move_shape(&self, dt: f32, queries: &QueryPipeline<'_>, character_shape: &dyn Shape, character_pos: &Pose,
  desired_translation: Vector, events: impl FnMut(CharacterCollision)) -> EffectiveCharacterMovement`, doc "Computes the
  possible movement for a shape." Sources: both pages above. Tier: official. Sure: confirmed in the source.
- **R4-PHYS-F4.** Gravity and jumping stay the caller's job. Quote: "Since you are responsible for providing the
  movement vector to the character controller at each frame," "it is up to you to emulate gravity by adding a downward
  component to that movement vector." Source: the guide page, section "Gravity". Tier: official. Sure: confirmed in
  the source.
- **R4-PHYS-F5.** The controller does not push dynamic bodies unless asked to apply impulses. Quotes: dynamic bodies
  "will generally not react to (i.e. not be pushed by) the character."; "The character controller can apply these
  forces for you if needed,"; `solve_character_collision_impulses` "will apply impulses to the rigid-bodies". Sources:
  both pages above. Tier: official. Sure: confirmed in the source.
- **R4-PHYS-F6.** Its snap-to-ground behaviour changed in a minor release. Quote (v0.35.0): "The character
  controller's snap-to-ground now triggers on any movement that isn't upwards". Source:
  https://raw.githubusercontent.com/dimforge/rapier/master/CHANGELOG.md (master, head v0.36.1). Tier: official
  (Dimforge repository). Sure: confirmed in the source.

### Rapier's bodies, vehicles and CCD
- **R4-PHYS-F7.** Kinematic bodies are moved by the game and ignore contacts, so nothing stops them at a wall unless the
  game sweeps them. Quotes: `KinematicPositionBased` "Indicates that the body position must not be altered by the
  physics engine."; "kinematic bodies will simply ignore any contact force and go through walls and the ground."
  Source: https://rapier.rs/docs/user_guides/rust/rigid_bodies (Rust 0.36; no date). Tier: official. Sure: confirmed in
  the source.
- **R4-PHYS-F8.** Rapier's vehicle controller is a ray-cast wheeled car ported from Bullet, and the guide itself says
  joint-built cars are hard to control for non-realistic games. Quotes: "Simulating a car with rigid-bodies and joints,
  for example using one rigid-body per wheel attached to the chassis" "by a joint, is possible but can be difficult to
  control for games requiring non-realistic vehicles."; "(the current implementation was ported from the
  `btRaycastVehicle` of Bullet)". Source: the character-controller guide page, section "Vehicle controller". Tier:
  official. Sure: confirmed in the source; that its chassis must be a dynamic body is inferred (fetch summary of the
  "Setup" section).
- **R4-PHYS-F9.** Rapier's CCD exists for dynamic bodies: fast dynamic bodies are swept against fixed colliders
  automatically, and "bullet" CCD against moving bodies costs more and is off by default. Quotes: "Continuous Collision
  Detection (CCD) is used to make sure that fast-moving objects don't miss any contacts" "(a problem usually called
  tunneling)."; "Every fast-moving dynamic rigid-body is swept against the fixed colliders and the soft-bodies of the
  scene."; "A dynamic rigid-body with CCD enabled (aka. a bullet) is swept against the kinematic and dynamic bodies as
  well."; "This is more expensive, therefore it is disabled by default". Source:
  https://rapier.rs/docs/user_guides/rust/rigid_body_ccd (Rust 0.36; no date). Tier: official. Sure: confirmed in the
  source.

### Rapier's scene queries and filters
- **R4-PHYS-F10.** Rapier's queries cover what the sim asks of a collision world: first hit along a ray (with normal),
  every collider a ray passes through, shape sweeps, point projection, and point, shape and box overlap. Quotes: "Casts
  a ray through the world and returns the first collider it hits."; "Returns ALL colliders that a ray passes through
  (not just the first)."; `cast_shape` "Sweeps a shape through the world to find what it would collide with.";
  `intersect_shape` "Retrieve all the colliders intersecting the given shape."; "Get a QueryPipeline from your
  BroadPhaseBvh using as_query_pipeline()." Source:
  https://docs.rs/rapier3d/latest/rapier3d/pipeline/struct.QueryPipeline.html (rapier3d 0.36.1). Tier: official. Sure:
  confirmed in the source.
- **R4-PHYS-F11.** Rapier's queries run on the broad phase's BVH, which the physics step updates, so they see the world
  as of the last step and keep no history. Quotes: "It reuses the acceleration data-structure (BVH) of the
  broad-phase," "which is automatically updated by the physics stepping function."; the BVH "is used in Rapier for both
  scene queries (ray-casting, point projection, etc) and the broad-phase." Sources:
  https://rapier.rs/docs/user_guides/rust/scene_queries (Rust 0.36; no date) and
  https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge (2026-01-09). Tier: official. Sure: confirmed in the
  source; "no history" is inferred (no history API appears on the QueryPipeline page).
- **R4-PHYS-F12.** Queries filter by body type, collision group, one excluded collider and a user closure; there are 32
  collision groups. Quotes: "`exclude_collider` is the handle of one collider the query must ignore."; "`predicate` is
  a user-defined closure to apply any filtering rule."; "Because the `membership` and `filter` bit masks are `u32`
  there is a total of 32 groups." Sources: the scene-queries page and https://rapier.rs/docs/user_guides/rust/colliders
  (Rust 0.36; no date). Tier: official. Sure: confirmed in the source.
- **R4-PHYS-F13.** The guide gives no query costs, only their direction. Quotes: "The more results you get, the more
  computationally expensive the ray-cast will be."; "Nonlinear shape-casting is more expensive than the linear one".
  Source: the scene-queries page. Tier: official. Sure: confirmed in the source (the absence of numbers is inferred).
- **R4-PHYS-F14.** Colliders come in balls, cuboids, capsules, cylinders, cones, compounds, convex meshes, triangle
  meshes, heightfields and polylines. Quote: "Compound shapes, convex meshes, triangle meshes, heightfields, and
  polylines". Source: the colliders page. Tier: official. Sure: confirmed in the source (the simple shapes are named in
  the fetch summary: inferred).

### Rapier's versions, features and focus
- **R4-PHYS-F15.** Rapier shipped 12 releases between 0.30.1 (17 Oct. 2025) and 0.36.1 (4 Oct. 2026), and 0.36.0
  broke loading of older snapshots. Quotes: release headings "v0.36.1 (4 October 2026)" … "v0.30.1 (17 Oct. 2025)"
  (as listed by the fetch); v0.36.0: "Snapshots serialized with previous versions can't be loaded anymore."; v0.35.0-
  beta.0: "The `IslandManager` and `RigidBodyActivation` serialization formats changed." Source: the CHANGELOG above.
  Tier: official. Sure: confirmed in the source.
- **R4-PHYS-F16.** In 0.35, SIMD became always on and the broad phase was reworked for large, mostly static worlds.
  Quotes: "SIMD is now always on, backed by `wide`"; "The broad phase was reworked for large, mostly-static worlds".
  Source: the CHANGELOG. Tier: official. Sure: confirmed in the source. (Context for R1-DET-F22 and R1-DET-OQ2.)
- **R4-PHYS-F17.** rapier3d 0.36.1 builds f32, 3D and std by default; f64 is a separate crate. Its flags include
  `enhanced-determinism`, `serde-serialize`, `parallel`, `simd8` and `fem`, and none is Wasm-specific. Quote (guide):
  `rapier3d-f64` for "high-precision simulation using 64-bits floats."; the feature names come from
  https://docs.rs/crate/rapier3d/latest/features (rapier3d 0.36.1; the page shows no Cargo syntax). Sources: that page
  and https://rapier.rs/docs/user_guides/rust/getting_started (Rust 0.36). Tier: official. Sure: confirmed for the
  crate names and the f64 quote; the flag list is inferred (fetch summary).
- **R4-PHYS-F18.** Dimforge's 2026 work and goals are the solver, multithreading, soft bodies, GPU physics and
  robotics; none of the posts read mentions queries or the character controller. Quotes: "Overall, this makes Rapier up
  to 2x faster than before single-threaded, and up to 4x faster than before multi-threaded"; "measured on my M4 Max
  MacBook Pro with 8 worker threads"; 2026 goal: "Improving Rapier's features and accuracy for robotics." Sources:
  https://dimforge.com/blog/2026/10/25/dimforge-Q3-technical-report (2026-10-25) and
  https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge (2026-01-09). Tier: official. Sure: confirmed in the
  source for the quotes; "none mentions" is inferred (the fetch reported the absence).

### parry3d: Dimforge's collision library on its own (parry3d 0.31.1)
- **R4-PHYS-F19.** parry3d is a geometry library (shapes, queries, spatial partitioning, bounding volumes) with no
  dynamics module; Rapier is built on it. Quotes: "**parry** is a 2 and 3-dimensional geometric library written with
  the rust programming language."; modules `query` "Non-persistent geometric queries.", `shape` "Shapes supported by
  parry.", `partitioning` "Spatial partitioning tools." Source: https://docs.rs/parry3d/latest/parry3d/ (parry3d
  0.31.1). Tier: official. Sure: confirmed in the source; "no dynamics module" is inferred from the module list; that
  Rapier uses it is R1-DET-F22 / F23.
- **R4-PHYS-F20.** parry3d has swept shape tests, overlap and distance tests, and ray casts between any two of its
  shapes. Quotes: `cast_shapes` "Computes when two moving shapes will collide (shape casting / swept collision
  detection)."; `intersection_test` "Tests whether two shapes are intersecting (overlapping)."; `RayCast` "Traits of
  objects which can be transformed and tested for intersection with a ray." Source:
  https://docs.rs/parry3d/latest/parry3d/query/index.html (parry3d 0.31.1). Tier: official. Sure: confirmed in the
  source.
- **R4-PHYS-F21.** parry3d's BVH is built once for static scenes and also supports moving leaves (insert, remove, a
  cheap refit); its ray cast calls a leaf test we supply. Quotes: "For scenes where objects don’t move, build the BVH
  once and query repeatedly:"; "The BVH supports dynamic scenes where objects move or are added/removed:"; `refit` is
  "very fast (just updates AABBs)"; `cast_ray` "Casts a ray on this BVH using the provided leaf ray-cast function.";
  memory "~64 bytes per pair of children (3D f32 SIMD), O(n) total." Source:
  https://docs.rs/parry3d/latest/parry3d/partitioning/struct.Bvh.html (parry3d 0.31.1; the first 100,000 of 117,438
  characters read). Tier: official. Sure: confirmed in the source; the build cost O(n log n) and refit O(n) are from the
  fetch summary (inferred).
- **R4-PHYS-F22.** parry moves as fast as Rapier: Parry 0.26 went with Rapier 0.32 (January 2026), and Rapier 0.36.1
  pins parry3d 0.31.1 (R1-DET-F23), five minor versions in about nine months. Quote: the 2025 review notes "Rapier 0.32
  and Parry 0.26 moved most of the public API from nalgebra to glam" (fetch wording; the exact words were not
  returned). Source: https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge. Tier: official. Sure: inferred.

### A shipped shooter's movement: Quake III Arena (GPL source release)
- **R4-PHYS-F23.** Quake III's player movement is shared code that the server and the client both run. Quotes: "//
  bg_pmove.c -- both games player movement code"; above `Pmove`: "Can be called by either the server or the client".
  Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c (id Software,
  GPL release; copyright 1999-2005). Tier: shipped game. Sure: confirmed in the source.
- **R4-PHYS-F24.** That movement is kinematic: box traces through the world, sliding along up to 5 clip planes in 4
  bumps, a fixed step-up and a walkable-slope threshold; there is no rigid-body solver in it. Quotes: "#define
  MAX_CLIP_PLANES 5"; "numbumps = 4;"; "pm->trace ( &trace, pm->ps->origin, pm->mins, pm->maxs, end,
  pm->ps->clientNum, pm->tracemask);"; "up[2] += STEPSIZE;"; "#define STEPSIZE 18"; "#define MIN_WALK_NORMAL 0.7f //
  can't walk on very steep slopes"; "if ( trace.plane.normal[2] < MIN_WALK_NORMAL ) {". Sources:
  https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_slidemove.c,
  .../code/game/bg_local.h, .../code/game/bg_pmove.c. Tier: shipped game. Sure: confirmed in the source for the lines;
  "no rigid-body solver" is inferred from the files read.

### Other options and facts
- **R4-PHYS-F25.** Avian, the other Rust physics engine, is built for Bevy's ECS. Quotes: "Avian is an ECS-driven 2D
  and 3D physics engine for the Bevy game engine."; "built _for_ Bevy _with_ Bevy,". Source:
  https://docs.rs/avian3d/latest/avian3d/ (avian3d 0.8.0). Tier: official. Sure: confirmed in the source.
- **R4-PHYS-F26.** f32 is IEEE 754-2008 binary32 with 24 significant bits. Quotes: "A 32-bit floating-point type
  (specifically, the “binary32” type defined in IEEE 754-2008)."; `MANTISSA_DIGITS` "Number of significant digits in
  base 2." (24); `EPSILON` 1.19209290e-07. Source: https://doc.rust-lang.org/std/primitive.f32.html (std 1.99.1). Tier:
  official. Sure: confirmed in the source. Arithmetic (inferred): the spacing between f32 values is 2^-16 m (about
  0.015 mm) for coordinates from 128 to 256 m, 2^-15 m (0.03 mm) to 512 m, 2^-14 m (0.06 mm) to 1,024 m and 2^-13 m
  (0.12 mm) to 2,048 m.
- **R4-PHYS-F27.** No source read in this run gives Rapier's or parry's Wasm size, the cost of one character-controller
  call, or the cost of one ray query. The one lead found (a tutorial claiming "~1.1 MB" of Wasm) is outside the allow
  list and was not read. Sure: confirmed (an absence across the sources above).

## Legacy facts used
- **CS2, CS4:** soldiers are hit as a head sphere plus two capsules; the closed-form ray tests are small and
  allocation-free but miss a capsule when the ray starts inside it. v2 keeps per-unit hit shapes as sim data.
- **CS5, CS6, CS9, GN7:** one slab test exists in three copies; rays loop over every static (220 in the test) with no
  broad phase (LOS 2.7-5.7 µs per call in JS after B7); ramps are full-height boxes, so bot fire needs a second world
  without them. v2 needs one static world, one set of queries, real slopes, and a spatial index.
- **CS7, CS8:** shots are a pure, read-only query; every static block has id -1. v2 needs stable ids on statics (also
  R1-ECS-REC3).
- **GW9, GW13:** movement collision is a circle pushed out of oriented boxes, shared by player, bots and the nav grid,
  O(agents × colliders); collider shapes come from render objects (a tree's radius from a `Math.sin` hash).
- **GW14 (tuning `T.gravity` 24, `T.jumpV` 11, `T.stepUp` 0.55):** legacy movement is a ground-height query, a
  step-up guard and a ballistic jump, "with no physics engine"; the jump apex is frame-rate dependent (2.25 / 2.43 /
  2.48 m at 20 / 60 / 144 Hz). Only the player stands on cover tops.
- **GW15, GC14:** blasts raycast render meshes and grenades never collide with walls; v2 needs one sim query for bullets
  and blasts and grenades that bounce off walls.
- **GC15:** bolts use a swept ray per step through `resolveShot`, so fast bolts never tunnel, but they integrate with
  the frame dt.
- **GC17 (tuning `JUG.maxSpeed` 6, `JUG.targetH` 5.6):** the Juggernaut has no obstacle collision (walks through
  walls), is not in the SIM and cannot be hit by bots.
- **Tuning `GROUND_W` 372, `arenaX` 186, `arenaZ` 126:** the legacy world spans a few hundred metres.

## What the simulation asks of physics (from VISION_V2 §6, LOOP_V2 and the legacy rows; my reading)
- Soldiers: kinematic movement with a step-up, slopes (ramps, plateaus), drop-downs, a ballistic jump, push-out from
  statics; one rule for every soldier, since humans take over bot slots (GW14). §6.2.
- Shots: hitscan rays against rewound movers and the present static world (R2-SYNC-REC5); the aim point at height
  (R2-SYNC-REC8). §6.2.
- Slow projectiles: bolts, grenades, Molotovs, Javelin rockets (LOOP_V2 §5) swept per tick; grenades bouncing off walls
  (GC14). §6.3.
- Blasts and melee: radius and arc overlap plus line of sight and height (GW15, GC12). §6.3.
- Big multi-part vehicles: driven bodies with several hit parts (R2-SYNC-REC5: "alive per part") that block soldiers
  and are blocked by walls (GC17). §6.4.
- Knockback (LOOP_V2 §5: Javelin rockets "knock infantry back"): a velocity change on a kinematic soldier.
- No §6 item asks for stacking, joints, ragdolls or rigid-body props in the sim: death falls are presentation (GC8). So
  the sim needs collision geometry and queries, not a dynamics solver. (Inferred from reading VISION_V2 §6 and LOOP_V2;
  a later §6 change could add such a need.)

## Options

| | A. Rapier, full engine | B. Rapier, static world only, plus its character controller | C. parry3d plus our own kinematic movement | D. Fully custom |
|---|---|---|---|---|
| What it is | Rapier sets hold statics, soldiers (kinematic), vehicles (kinematic), maybe grenades (dynamic); `PhysicsPipeline::step` each tick; the controller moves soldiers | Rapier holds only the immutable map colliders; soldiers and vehicles live in our tables and move with Rapier's controller against the statics; movers are tested by our own code | parry3d shapes, queries and BVH; the static BVH built once from the map document; movers in our tables with a per-tick index; a Quake-style slide-and-step move (F23, F24) written by us | Our own shapes (legacy CS4, CS5 ported, plus wedges), our own grid or BVH, our own sweeps |
| Complexity | High: a full engine, most of it (solver, joints, islands, soft bodies) unused by §6 | Medium | Medium | High: shape-vs-shape sweeps (capsule against oriented box, wedge, mesh) are the hard part |
| Snapshot | Rapier's sets are part of every snapshot (R1-ECS-REC4) and allocate on clone (R1-ECS-REC1 mark 2, R1-ECS-F35); 0.36.0 broke old snapshots (F15) | Statics never change after load, so they stay out of the snapshot (alive flags live in our tables and filter queries through the predicate, F12) | Nothing outside our tables: static index immutable, dynamic index rebuilt each tick | As C |
| Determinism | `enhanced-determinism` (R1-DET-REC3); the solver's SIMD path is open (R1-DET-OQ2, F16) | Same flag; no solver runs, so its SIMD lanes only matter if queries use them | parry3d's own flag forces libm and scalar maths (R1-DET-F23); the BVH build order is ours | Fully ours (R1-DET-REC2's rules) |
| Wasm size and CPU | Largest (not measured: F27) | Rapier crate linked; how much unused code is stripped is not documented (inferred) | Smaller than A or B (inferred, not measured: F27) | Smallest (inferred) |
| Version churn | 12 releases in about a year, with API and format breaks (F15, F6) | The controller's feel can change on upgrade (F6) | parry moves as fast (F22), but the surface used is small: shapes, queries, BVH | None upstream |
| Effort | Low to start, high to keep deterministic and snapshotted | Low for soldiers; vehicles and mover-vs-mover by us | Medium: the slide-and-step move (Quake III keeps it in two files, F23, F24) and the vehicle sweep | Highest |
| Fit with §6 | Gives dynamics no §6 item needs; vehicles as Bullet-style cars do not fit a walker and faction tanks (F8; LOOP_V2 §6) | §6.2 well (autostep, slopes, F2); §6.4 by our code | §6.2 and §6.4, all by our code on proven queries | Same fit as C, more risk |

## Recommendations

- **R4-PHYS-REC1. No physics engine: use parry3d (pinned, `enhanced-determinism` on) for shapes, queries and the BVH,
  and write the movement ourselves as a kinematic slide-and-step move (option C). Measure Rapier's character
  controller on the same static world in the spike (option B) as the fallback.**
  The reason: no §6 item needs a dynamics solver (the needs list above), and Rapier's controller is itself only rays and
  shape casts around a caller-supplied move (F1, F3, F4); a shipped shooter's shared client-server movement is exactly
  that kind of kinematic trace code (F23, F24). With option C the physics state is our own tables, so R1-ECS-REC4's
  "Rapier's sets in every snapshot" and R1-ECS-REC1 mark 2's separate Rapier allocation count fall away, and Rapier's
  snapshot-format breaks (F15) cannot reach us. Owning the move also keeps reach and step-up exactly the sim's own
  integrator (R1-TICK-REC6, GW14), where an upstream controller changed its snap rule in a minor release (F6). parry3d
  gives the hard part, swept tests between any two shapes and a BVH (F19, F20, F21), and has its own determinism flag
  (R1-DET-F23). Avian is Bevy-bound (F25), which R1-ECS-REC1's no-ECS layout rules out. Rapier stays the measured
  fallback because its controller already does autostep, snapping and slopes (F2).
  Rests on: F1, F2, F3, F4, F6, F15, F19, F20, F21, F22, F23, F24, F25, F27; R1-ECS-REC1, R1-ECS-REC4, R1-TICK-REC6,
  R1-DET-F23; legacy GW13, GW14.
  Serves: VISION_V2 §6.2 (a true 3D simulation: floors, ledges, aiming at height) and §6.4 (big vehicles).
  A spike must prove (proposed pass marks; reference hardware as proposed in R1-TICK-REC2):
  1. **Movement parity.** On a greybox with platforms at 1.2, 2.5 and 4.0 m (legacy GC3's probe heights) and ramps, at
     the shipped tick: a soldier steps up 0.55 m and is stopped by 0.56 m; the jump apex equals R1-TICK-REC6's
     integrated value within 1 mm; 0 tunnelling through a 0.2 m wall in 10,000 random moves at sprint speed (legacy
     `moveSpeed` 6.3 × `sprintMult` 1.4286, about 9.0 m/s).
  2. **Cost.** At R1-ECS-REC1 mark 1's stress load, movement plus all physics queries take at most half of that mark's
     mean tick (≤ 1 ms native mean), reported separately from AI and abilities; Wasm in a Worker ≤ 2× native.
  3. **Determinism.** 0 per-tick hash mismatches between native and Wasm over R1-DET-REC1's runs, with parry3d's flag
     on; the static BVH built from the same map document has byte-identical nodes on both targets.
  4. **Size.** The sim .wasm with parry3d, with rapier3d (option B) and with neither, after wasm-opt and gzip, is
     recorded (extends R1-ECS-REC1 mark 5).
  5. **The fallback, measured once.** The same 48 soldiers moved by Rapier's controller over the same static world
     (option B): cost, parity under mark 1, and size. The ADR picks C unless B wins on marks 1 and 2.

- **R4-PHYS-REC2. One static collision world from the map document, immutable for the match, with stable ids; one set
  of queries for every user.**
  Static colliders (oriented boxes about Y, wedges for ramps, capsules or cylinders for trunks, and whatever the map
  format adds for plateaus) are built from the map document, never from render objects, each with the sim's entity id
  (R1-ECS-REC3). Their BVH is built once at match start in id order and never changes; a destructible piece's alive flag
  lives in the sim's tables, and queries skip dead pieces. Movers (soldiers, vehicle parts, projectiles) live in the
  sim's tables; their index is rebuilt from the tables each tick, so neither index is in the snapshot, and both are
  rebuilt identically after a restore. Shots, sight, blasts, melee checks, aim and the grenade-arc preview call the same
  query functions (one ray-versus-box, not three copies).
  The reason: legacy has three copies of the slab test and two static worlds for shots and sight (CS5, GN7), blasts
  raycast render meshes (GW15), statics have no ids (CS7), and collider sizes depend on render maths (GW9). parry3d's
  BVH suits a build-once static scene (F21). R1's cross-topic note asks R4 whether the spatial index lives inside the
  snapshot or is rebuilt after a restore: rebuilt, from immutable map data and the tables. R1-DATA-OQ7's "map geometry
  probably needs a match restart" holds: the static world never changes mid-match except through alive flags.
  Rests on: F21; R1-ECS-REC3, R1-ECS-F22, R1-DATA-OQ7; legacy CS5, CS6, CS7, CS8, GN7, GW9, GW15.
  Serves: VISION_V2 §6.2 (line of sight across heights, ledges, interiors) and §6.3 (blasts and abilities query one
  world).
  A spike must prove (proposed): (a) a port-parity fuzz of 20,000 random rays (legacy CS6's method) gives the same
  hits on boxes as the legacy SIM; (b) 0 of 1,200 bot shots stop more than 1 cm above a ramp's slope (legacy GN7's
  test, which used 30 cm); (c) a restore followed by the index rebuild gives byte-identical query results for 10,000
  random rays; (d) the static BVH build for the greybox takes ≤ 50 ms native at match start (proposed; legacy's nav
  grid build took 42.8 ms on Cabana, GN11).

- **R4-PHYS-REC3. Projectiles as per-tick swept queries, never as physics bodies; no CCD.**
  Hitscan is a ray query (R2-SYNC-REC5). Each slow projectile (bolt, grenade, Molotov, rocket) moves by the sim's
  integrator on the fixed tick, then tests the segment from its last position: a ray for point-like ones, a shape cast
  of its radius for grenades, which bounce off walls and floors by reflecting the velocity at the hit normal. Knockback
  is a velocity change on the kinematic soldier.
  The reason: Rapier's CCD only concerns dynamic bodies (F9), which REC1 does not use; legacy's swept ray per step
  already keeps bolts from tunnelling (GC15) but integrates with the frame dt; grenades that ignore walls (GC14) are a
  known gap; parry's swept tests give the time of impact (F20).
  Rests on: F9, F20; legacy GC14, GC15; R2-SYNC-REC5; R1-TICK-REC1.
  Serves: VISION_V2 §6.3 (weapons and abilities as data, whose projectiles share one flight model) and §6.2.
  A spike must prove (proposed): 0 tunnelling through a 0.2 m wall in 10,000 shots at the fastest projectile speed in
  the data, at 30 and 60 Hz; a grenade thrown at a wall bounces back in 100 % of 1,000 throws; the landing point of a
  fixed throw is identical on native and Wasm (state hash).

- **R4-PHYS-REC4. Big vehicles as kinematic compound bodies, moved by a swept test against statics and soldiers; no
  substeps at 30 Hz.**
  A vehicle is a pose plus parts (hull, turret, legs) in the sim's tables, each part a parry shape with its own hit
  volume and alive flag. Each tick it sweeps its hull from the old to the new pose against the static world and stops
  or slides like a soldier; what happens to a soldier in its path is a game rule (OQ1). Mount points for weapons are sim
  data (GC17's lesson).
  The reason: kinematic bodies go through walls unless the game sweeps them (F7); Rapier's wheeled vehicle is a
  Bullet-style dynamic car (F8), while LOOP_V2 §6's vehicles are a walker and faction tanks. Answering R1-TICK-OQ9: at
  `JUG.maxSpeed` 6 m/s a vehicle moves 0.2 m per tick at 30 Hz (arithmetic), well under a soldier's 0.7 m radius, and
  the move is swept anyway, so no physics substeps are needed for kinematic vehicles.
  Rests on: F7, F8, F20; legacy GC17, GW13 (radius 0.7); R1-TICK-OQ9.
  Serves: VISION_V2 §6.4 (big, multi-part vehicles).
  A spike must prove (proposed): a vehicle driven into walls, ramps and a crowd of 48 soldiers for 10 minutes ends 0
  ticks with any soldier or wall inside its hull (penetration > 1 cm); the stress load for R1-DET-REC3 and R1-ECS-REC1
  mark 1 keeps 8 big vehicles (R1-ECS-REC1's number), harvesters included (LOOP_V2 §3; how many per team is the
  game-designer's), at 30 and 60 Hz.

- **R4-PHYS-REC5. f32 for the whole sim, revisited only if a map exceeds 2 km across.**
  The reason: R1-DET-OQ3 leaves f32 or f64 to R4 with the map size. Legacy spans 372 m (`GROUND_W`); LOOP_V2 §9 starts
  Octopon-3 near Cabana's footprint. Within ±1,024 m of the origin f32 positions are spaced at most 0.06 mm apart
  (F26), far below the 1 mm marks above; rapier3d and parry3d default to f32 (F17), and Rapier's controller takes f32
  (F3).
  Rests on: F3, F17, F26; legacy `GROUND_W`, `arenaX`, `arenaZ`; R1-DET-OQ3.
  Serves: VISION_V2 §6.2.
  A spike must prove (proposed): a 10-minute bot match placed with its centre at the far corner of the largest planned
  map gives the same kill and objective events as the same match at the origin, and the movement-parity marks of REC1
  hold there.

## Open questions
- **R4-PHYS-OQ1. Who blocks whom.** Do soldiers block each other (legacy: no soldier-soldier push-out, GW13)? Does a
  vehicle push, stop or crush a soldier in its path, and do teammates pass through each other? What decides it: the
  game-designer with Mustafa, then REC4's spike.
- **R4-PHYS-OQ2. Our own move or Rapier's controller.** The sources give no cost or Wasm size for either (F27). What
  decides it: REC1 marks 2, 4 and 5.
- **R4-PHYS-OQ3. parry3d's BVH and SIMD across targets.** parry's BVH node is described as "3D f32 SIMD" (F21); whether
  its build and traversal give bit-identical results on wasm32 and x86-64 with the flag on is not stated (as R1-DET-OQ2
  for Rapier). What decides it: REC1 mark 3 and R1-DET-REC6's suite.
- **R4-PHYS-OQ4. The shape of the ground.** Plateaus, cliffs and drop-downs (EDITOR_V2 §3) as boxes and wedges, a
  heightfield, or a triangle mesh (all parry shapes, F14)? Terrain sculpting is [LATER] in EDITOR_V2. What decides it:
  the map format (editor work) and REC2's spike.
- **R4-PHYS-OQ5. If dynamics is ever needed.** A future §6 item (physics props or debris that decide outcomes) would
  bring Rapier back; the physics code stays behind one sim module so the swap is contained. What decides it: Mustafa,
  if such a feature is asked for.

## Wave-1 items this changes (said openly, not silently)
- R1-ECS-REC4 and R1-ECS-REC1 mark 2 assumed Rapier's sets in the snapshot; under REC1 there are none (only if the
  ADR keeps C).
- R1-DET-REC3's flag moves to parry3d's `enhanced-determinism` (R1-DET-F23); its spike scene and the vehicle count
  (REC4) stand.
- R1-TICK-OQ9 answered: no substeps for kinematic vehicles (REC4).
- R1-DET-OQ3 answered with a bound: f32, up to a 2 km map (REC5).

## Sources read
- https://rapier.rs/docs/user_guides/rust/character_controller · Rapier user guide, character controller · Rust 0.36, no date · official
- https://docs.rs/rapier3d/latest/rapier3d/control/struct.KinematicCharacterController.html · rapier3d 0.36.1 · official
- https://docs.rs/rapier3d/latest/rapier3d/pipeline/struct.QueryPipeline.html · rapier3d 0.36.1 · official
- https://rapier.rs/docs/user_guides/rust/scene_queries · Rust 0.36, no date · official
- https://rapier.rs/docs/user_guides/rust/rigid_body_ccd · Rust 0.36, no date · official
- https://rapier.rs/docs/user_guides/rust/rigid_bodies · Rust 0.36, no date · official
- https://rapier.rs/docs/user_guides/rust/colliders · Rust 0.36, no date · official
- https://rapier.rs/docs/user_guides/rust/getting_started · Rust 0.36, no date · official
- https://raw.githubusercontent.com/dimforge/rapier/master/CHANGELOG.md · master, head v0.36.1 (2026-10-04) · official
- https://docs.rs/crate/rapier3d/latest/features · rapier3d 0.36.1 · official
- https://dimforge.com/blog · index · official
- https://dimforge.com/blog/2026/10/25/dimforge-Q3-technical-report · 2026-10-25 · official
- https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge · 2026-01-09 · official
- https://docs.rs/parry3d/latest/parry3d/ · parry3d 0.31.1 · official
- https://docs.rs/parry3d/latest/parry3d/partitioning/index.html · parry3d 0.31.1 · official
- https://docs.rs/parry3d/latest/parry3d/partitioning/struct.Bvh.html · parry3d 0.31.1 · official
- https://docs.rs/parry3d/latest/parry3d/query/index.html · parry3d 0.31.1 · official
- https://docs.rs/avian3d/latest/avian3d/ · avian3d 0.8.0 · official
- https://doc.rust-lang.org/std/primitive.f32.html · std 1.99.1 · official
- https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_slidemove.c · GPL release · shipped game
- https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_local.h · GPL release · shipped game
- https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c · GPL release · shipped game

## Sources not read
- https://learnwithhasan.com/js-libraries/rapier/ (search lead: a Wasm size claim) · outside the allow list
- https://app.unpkg.com/@dimforge/rapier3d@0.17.3/files/README.md (search lead) · outside the allow list
- https://classic.yarnpkg.com/en/package/@dimforge/rapier3d (search lead) · outside the allow list
- https://git.nea.moe/github/rapier.git/log/CHANGELOG.md (search lead, a mirror) · outside the allow list; the
  official CHANGELOG was read instead
- https://www.abratabia.com/game-physics/rapier.php (search lead) · outside the allow list

## Cross-topic notes
- R1: REC1 changes R1-ECS-REC4, R1-ECS-REC1 mark 2 and R1-DET-REC3's crate (listed above); R1-TICK-OQ9 and
  R1-DET-OQ3 answered; the spatial index is rebuilt, not snapshotted.
- R2: the rewound hitscan tests movers from R2-SYNC-REC5's history arrays with our own capsule maths (CS4) and the
  static world through the static BVH (F11: Rapier's queries keep no history either way).
- R5: a counting test for the static BVH build and a parity fuzz (REC2 (a)); the native-versus-Wasm hash suite covers
  parry3d.
- game-designer: OQ1 (blocking rules).
