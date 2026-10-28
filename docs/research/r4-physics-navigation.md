# R4: Physics and navigation

- **Topic:** R4, physics and navigation (`docs/V2_PHASE0_BRIEF.md` §4).
- **Author agent:** engine-architect (wave 2 of `.checkpoints/v2-research/plan.md`: one owner agent for the topic,
  covering all of its brief questions).
- **Date:** 2026-10-28 (every source below was read that day).
- **Status:** draft: spot-check pending.
- **The brief's questions for this topic (quoted, `docs/V2_PHASE0_BRIEF.md` §4, row R4):** "Rapier 3D or custom;
  navigation meshes in Rust; bots moving between floors; line of sight across heights; obstacles placed mid-match (for
  later building)"

**How this file is laid out.** Each part of the shape (`docs/research/README.md`) has one section per brief question,
in the brief's order.

| # | Brief question | Ids |
|---|---|---|
| 1 | Rapier 3D or custom | R4-PHYS-F1–F27, REC1–REC5, OQ1–OQ5 |
| 2 | navigation meshes in Rust | R4-NAV-F1–F19, REC1–REC4, OQ1–OQ5 |
| 3 | bots moving between floors | R4-FLOORS-F1–F7, REC1–REC3, OQ1–OQ4 |
| 4 | line of sight across heights | R4-LOS-F1–F5, REC1–REC3, OQ1–OQ5 |
| 5 | obstacles placed mid-match (for later building) | R4-OBST-F1–F4, REC1–REC3, OQ1–OQ4 |

Totals: 62 findings, 18 recommendations, 23 open questions; 46 sources read (36 official, 8 shipped game, 2 other)
and 32 sources not read (part 8).

**Reading rules.**
- Tiers: "official" is a project's own documentation or repository (for a third-party Rust crate, its docs.rs pages);
  "shipped game" is a shipped game's own source code (Quake III Arena's GPL release, read on GitHub); "other" is
  anything else, including two crate READMEs whose statements are about other projects (the rerecast README on who
  uses Recast; the recastnavigation-sys README on upstream's release process).
- "Confirmed in the source" means the finding quotes the source's exact words; "inferred" means the point came from
  the fetch tool's summary or from reasoning on the quotes; "unverified" is not used.
- Every pass mark and reference machine is proposed; the targets come only from VISION_V2 §3, §4 and §6.
- A wave-1 finding or recommendation is cited by its id (R1-…, R2-…), not copied. Legacy facts are cited by their
  `docs/LEGACY_INVENTORY.md` row id or `docs/LEGACY_TUNING.md` table, with the inventory's marks read as working
  recommendations.
- Nothing here decides the look, designs player building, or is engine code.

## TL;DR
- Found: no VISION_V2 §6 item needs rigid-body dynamics; Rapier's controller is only rays and shape casts; C++ Detour
  cannot join the browser sim (no C/C++ toolchain for `wasm32-unknown-unknown`); no Rust nav crate promises determinism.
- Recommended (marks proposed): parry3d plus our own kinematic movement, Rapier's controller the measured fallback; one
  immutable static world from the map document; navigation baked into it and queried by the sim, links found by running
  the sim's movement (as Quake III's bots do); one height-aware sight query; vehicles avoided, never carved.

## Findings

### 1. Rapier 3D or custom (R4-PHYS)

#### Rapier's character controller (rapier3d 0.36.1)
- **R4-PHYS-F1.** Rapier's character controller is not a physics body: it is a helper that issues ray casts and shape
  casts to correct a movement the caller asks for. Quote: "a higher-level tool that will emit the proper ray-casts and
  shape-casts to adjust the user-defined trajectory". Source:
  https://rapier.rs/docs/user_guides/rust/character_controller (guide navigation "User Guide: Rust (0.36)"; no page
  date). Tier: official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-PHYS-F2.** It climbs steps and small obstacles (autostep, off by default), snaps to the ground, slides on
  slopes up to a maximum climb angle, and keeps a small offset from surfaces. Quotes: "the **autostep** setting allows
  the character to climb stairs automatically and walk over small obstacles."; "**snap-to-ground** will force the
  character to stick to the ground if the following conditions are met."; field `autostep`: "Should the character
  automatically step over small obstacles? (disabled by default)"; field `max_slope_climb_angle`: "The maximum angle
  (radians) between the floor’s normal and the `up` vector that the character is able to climb."; field `offset`: "A
  small gap to preserve between the character and its surroundings." Sources: the guide page above and
  https://docs.rs/rapier3d/latest/rapier3d/control/struct.KinematicCharacterController.html (rapier3d 0.36.1). Tier:
  official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-PHYS-F3.** The controller holds no per-character state, so one instance can move every soldier; the call takes
  the time step, a query pipeline, a shape, a pose and the wanted translation. Quotes: the configuration "does not
  contain any collider-specific or rigid-body-specific information like handles, velocities, positions, etc."; the same
  instance "can be used to control multiple rigid-bodies/colliders if they rely on the same set of parameters.";
  `move_shape(&self, dt: f32, queries: &QueryPipeline<'_>, character_shape: &dyn Shape, character_pos: &Pose,
  desired_translation: Vector, events: impl FnMut(CharacterCollision)) -> EffectiveCharacterMovement`, doc "Computes the
  possible movement for a shape." Sources: both pages above. Tier: official. Read 2026-10-28. Sure: confirmed in the
  source.
- **R4-PHYS-F4.** Gravity and jumping stay the caller's job. Quote: "Since you are responsible for providing the
  movement vector to the character controller at each frame," "it is up to you to emulate gravity by adding a downward
  component to that movement vector." Source: the guide page, section "Gravity". Tier: official. Read 2026-10-28. Sure:
  confirmed in the source.
- **R4-PHYS-F5.** The controller does not push dynamic bodies unless asked to apply impulses. Quotes: dynamic bodies
  "will generally not react to (i.e. not be pushed by) the character."; "The character controller can apply these
  forces for you if needed,"; `solve_character_collision_impulses` "will apply impulses to the rigid-bodies". Sources:
  both pages above. Tier: official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-PHYS-F6.** Its snap-to-ground behaviour changed in a minor release. Quote (v0.35.0): "The character
  controller's snap-to-ground now triggers on any movement that isn't upwards". Source:
  https://raw.githubusercontent.com/dimforge/rapier/master/CHANGELOG.md (master, head v0.36.1). Tier: official
  (Dimforge repository). Read 2026-10-28. Sure: confirmed in the source.

#### Rapier's bodies, vehicles and CCD
- **R4-PHYS-F7.** Kinematic bodies are moved by the game and ignore contacts, so nothing stops them at a wall unless the
  game sweeps them. Quotes: `KinematicPositionBased` "Indicates that the body position must not be altered by the
  physics engine."; "kinematic bodies will simply ignore any contact force and go through walls and the ground." Source:
  https://rapier.rs/docs/user_guides/rust/rigid_bodies (Rust 0.36; no date). Tier: official. Read 2026-10-28. Sure:
  confirmed in the source.
- **R4-PHYS-F8.** Rapier's vehicle controller is a ray-cast wheeled car ported from Bullet, and the guide itself says
  joint-built cars are hard to control for non-realistic games. Quotes: "Simulating a car with rigid-bodies and joints,
  for example using one rigid-body per wheel attached to the chassis" "by a joint, is possible but can be difficult to
  control for games requiring non-realistic vehicles."; "(the current implementation was ported from the
  `btRaycastVehicle` of Bullet)". Source: the character-controller guide page, section "Vehicle controller". Tier:
  official. Read 2026-10-28. Sure: confirmed in the source; that its chassis must be a dynamic body is inferred (fetch
  summary of the "Setup" section).
- **R4-PHYS-F9.** Rapier's CCD exists for dynamic bodies: fast dynamic bodies are swept against fixed colliders
  automatically, and "bullet" CCD against moving bodies costs more and is off by default. Quotes: "Continuous Collision
  Detection (CCD) is used to make sure that fast-moving objects don't miss any contacts" "(a problem usually called
  tunneling)."; "Every fast-moving dynamic rigid-body is swept against the fixed colliders and the soft-bodies of the
  scene."; "A dynamic rigid-body with CCD enabled (aka. a bullet) is swept against the kinematic and dynamic bodies as
  well."; "This is more expensive, therefore it is disabled by default". Source:
  https://rapier.rs/docs/user_guides/rust/rigid_body_ccd (Rust 0.36; no date). Tier: official. Read 2026-10-28. Sure:
  confirmed in the source.

#### Rapier's scene queries and filters
- **R4-PHYS-F10.** Rapier's queries cover what the sim asks of a collision world: first hit along a ray (with normal),
  every collider a ray passes through, shape sweeps, point projection, and point, shape and box overlap. Quotes: "Casts
  a ray through the world and returns the first collider it hits."; "Returns ALL colliders that a ray passes through
  (not just the first)."; `cast_shape` "Sweeps a shape through the world to find what it would collide with.";
  `intersect_shape` "Retrieve all the colliders intersecting the given shape."; "Get a QueryPipeline from your
  BroadPhaseBvh using as_query_pipeline()." Source:
  https://docs.rs/rapier3d/latest/rapier3d/pipeline/struct.QueryPipeline.html (rapier3d 0.36.1). Tier: official. Read
  2026-10-28. Sure: confirmed in the source.
- **R4-PHYS-F11.** Rapier's queries run on the broad phase's BVH, which the physics step updates, so they see the world
  as of the last step and keep no history. Quotes: "It reuses the acceleration data-structure (BVH) of the broad-phase,"
  "which is automatically updated by the physics stepping function."; the BVH "is used in Rapier for both scene queries
  (ray-casting, point projection, etc) and the broad-phase." Sources:
  https://rapier.rs/docs/user_guides/rust/scene_queries (Rust 0.36; no date) and
  https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge (2026-01-09). Tier: official. Read 2026-10-28. Sure:
  confirmed in the source; "no history" is inferred (no history API appears on the QueryPipeline page).
- **R4-PHYS-F12.** Queries filter by body type, collision group, one excluded collider and a user closure; there are 32
  collision groups. Quotes: "`exclude_collider` is the handle of one collider the query must ignore."; "`predicate` is
  a user-defined closure to apply any filtering rule."; "Because the `membership` and `filter` bit masks are `u32`
  there is a total of 32 groups." Sources: the scene-queries page and https://rapier.rs/docs/user_guides/rust/colliders
  (Rust 0.36; no date). Tier: official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-PHYS-F13.** The guide gives no query costs, only their direction. Quotes: "The more results you get, the more
  computationally expensive the ray-cast will be."; "Nonlinear shape-casting is more expensive than the linear one".
  Source: the scene-queries page. Tier: official. Read 2026-10-28. Sure: confirmed in the source (the absence of numbers
  is inferred).
- **R4-PHYS-F14.** Colliders come in balls, cuboids, capsules, cylinders, cones, compounds, convex meshes, triangle
  meshes, heightfields and polylines. Quote: "Compound shapes, convex meshes, triangle meshes, heightfields, and
  polylines". Source: the colliders page. Tier: official. Read 2026-10-28. Sure: confirmed in the source (the simple
  shapes are named in the fetch summary: inferred).

#### Rapier's versions, features and focus
- **R4-PHYS-F15.** Rapier shipped 12 releases between 0.30.1 (17 Oct. 2025) and 0.36.1 (4 Oct. 2026), and 0.36.0
  broke loading of older snapshots. Quotes: release headings "v0.36.1 (4 October 2026)" … "v0.30.1 (17 Oct. 2025)"
  (as listed by the fetch); v0.36.0: "Snapshots serialized with previous versions can't be loaded anymore."; v0.35.0-
  beta.0: "The `IslandManager` and `RigidBodyActivation` serialization formats changed." Source: the CHANGELOG above.
  Tier: official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-PHYS-F16.** In 0.35, SIMD became always on and the broad phase was reworked for large, mostly static worlds.
  Quotes: "SIMD is now always on, backed by `wide`"; "The broad phase was reworked for large, mostly-static worlds".
  Source: the CHANGELOG. Tier: official. Read 2026-10-28. Sure: confirmed in the source. (Context for R1-DET-F22 and
  R1-DET-OQ2.)
- **R4-PHYS-F17.** rapier3d 0.36.1 builds f32, 3D and std by default; f64 is a separate crate. Its flags include
  `enhanced-determinism`, `serde-serialize`, `parallel`, `simd8` and `fem`, and none is Wasm-specific. Quote (guide):
  `rapier3d-f64` for "high-precision simulation using 64-bits floats."; the feature names come from
  https://docs.rs/crate/rapier3d/latest/features (rapier3d 0.36.1; the page shows no Cargo syntax). Sources: that page
  and https://rapier.rs/docs/user_guides/rust/getting_started (Rust 0.36). Tier: official. Read 2026-10-28. Sure:
  confirmed for the crate names and the f64 quote; the flag list is inferred (fetch summary).
- **R4-PHYS-F18.** Dimforge's 2026 work and goals are the solver, multithreading, soft bodies, GPU physics and robotics;
  none of the posts read mentions queries or the character controller. Quotes: "Overall, this makes Rapier up to 2x
  faster than before single-threaded, and up to 4x faster than before multi-threaded"; "measured on my M4 Max MacBook
  Pro with 8 worker threads"; 2026 goal: "Improving Rapier's features and accuracy for robotics." Sources:
  https://dimforge.com/blog/2026/10/25/dimforge-Q3-technical-report (2026-10-25) and
  https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge (2026-01-09). Tier: official. Read 2026-10-28. Sure:
  confirmed in the source for the quotes; "none mentions" is inferred (the fetch reported the absence).

#### parry3d: Dimforge's collision library on its own (parry3d 0.31.1)
- **R4-PHYS-F19.** parry3d is a geometry library (shapes, queries, spatial partitioning, bounding volumes) with no
  dynamics module; Rapier is built on it. Quotes: "**parry** is a 2 and 3-dimensional geometric library written with the
  rust programming language."; modules `query` "Non-persistent geometric queries.", `shape` "Shapes supported by
  parry.", `partitioning` "Spatial partitioning tools." Source: https://docs.rs/parry3d/latest/parry3d/ (parry3d
  0.31.1). Tier: official. Read 2026-10-28. Sure: confirmed in the source; "no dynamics module" is inferred from the
  module list; that Rapier uses it is R1-DET-F22 / F23.
- **R4-PHYS-F20.** parry3d has swept shape tests, overlap and distance tests, and ray casts between any two of its
  shapes. Quotes: `cast_shapes` "Computes when two moving shapes will collide (shape casting / swept collision
  detection)."; `intersection_test` "Tests whether two shapes are intersecting (overlapping)."; `RayCast` "Traits of
  objects which can be transformed and tested for intersection with a ray." Source:
  https://docs.rs/parry3d/latest/parry3d/query/index.html (parry3d 0.31.1). Tier: official. Read 2026-10-28. Sure:
  confirmed in the source.
- **R4-PHYS-F21.** parry3d's BVH is built once for static scenes and also supports moving leaves (insert, remove, a
  cheap refit); its ray cast calls a leaf test we supply. Quotes: "For scenes where objects don’t move, build the BVH
  once and query repeatedly:"; "The BVH supports dynamic scenes where objects move or are added/removed:"; `refit` is
  "very fast (just updates AABBs)"; `cast_ray` "Casts a ray on this BVH using the provided leaf ray-cast function.";
  memory "~64 bytes per pair of children (3D f32 SIMD), O(n) total." Source:
  https://docs.rs/parry3d/latest/parry3d/partitioning/struct.Bvh.html (parry3d 0.31.1; the first 100,000 of 117,438
  characters read). Tier: official. Read 2026-10-28. Sure: confirmed in the source; the build cost O(n log n) and refit
  O(n) are from the fetch summary (inferred).
- **R4-PHYS-F22.** parry moves as fast as Rapier: Parry 0.26 went with Rapier 0.32 (January 2026), and Rapier 0.36.1
  pins parry3d 0.31.1 (R1-DET-F23), five minor versions in about nine months. Quote: the 2025 review notes "Rapier 0.32
  and Parry 0.26 moved most of the public API from nalgebra to glam" (fetch wording; the exact words were not returned).
  Source: https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge. Tier: official. Read 2026-10-28. Sure:
  inferred.

#### A shipped shooter's movement: Quake III Arena (GPL source release)
- **R4-PHYS-F23.** Quake III's player movement is shared code that the server and the client both run. Quotes: "//
  bg_pmove.c -- both games player movement code"; above `Pmove`: "Can be called by either the server or the client".
  Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c (id Software,
  GPL release; copyright 1999-2005). Tier: shipped game. Read 2026-10-28. Sure: confirmed in the source.
- **R4-PHYS-F24.** That movement is kinematic: box traces through the world, sliding along up to 5 clip planes in 4
  bumps, a fixed step-up and a walkable-slope threshold; there is no rigid-body solver in it. Quotes: "#define
  MAX_CLIP_PLANES 5"; "numbumps = 4;"; "pm->trace ( &trace, pm->ps->origin, pm->mins, pm->maxs, end, pm->ps->clientNum,
  pm->tracemask);"; "up[2] += STEPSIZE;"; "#define STEPSIZE 18"; "#define MIN_WALK_NORMAL 0.7f // can't walk on very
  steep slopes"; "if ( trace.plane.normal[2] < MIN_WALK_NORMAL ) {". Sources:
  https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_slidemove.c,
  .../code/game/bg_local.h, .../code/game/bg_pmove.c. Tier: shipped game. Read 2026-10-28. Sure: confirmed in the source
  for the lines; "no rigid-body solver" is inferred from the files read.

#### Other options and facts
- **R4-PHYS-F25.** Avian, the other Rust physics engine, is built for Bevy's ECS. Quotes: "Avian is an ECS-driven 2D and
  3D physics engine for the Bevy game engine."; "built _for_ Bevy _with_ Bevy,". Source:
  https://docs.rs/avian3d/latest/avian3d/ (avian3d 0.8.0). Tier: official. Read 2026-10-28. Sure: confirmed in the
  source.
- **R4-PHYS-F26.** f32 is IEEE 754-2008 binary32 with 24 significant bits. Quotes: "A 32-bit floating-point type
  (specifically, the “binary32” type defined in IEEE 754-2008)."; `MANTISSA_DIGITS` "Number of significant digits in
  base 2." (24); `EPSILON` 1.19209290e-07. Source: https://doc.rust-lang.org/std/primitive.f32.html (std 1.99.1). Tier:
  official. Read 2026-10-28. Sure: confirmed in the source. Arithmetic (inferred): the spacing between f32 values is
  2^-16 m (about 0.015 mm) for coordinates from 128 to 256 m, 2^-15 m (0.03 mm) to 512 m, 2^-14 m (0.06 mm) to 1,024 m
  and 2^-13 m (0.12 mm) to 2,048 m.
- **R4-PHYS-F27.** No source read in this run gives Rapier's or parry's Wasm size, the cost of one character-controller
  call, or the cost of one ray query. The one lead found (a tutorial claiming "~1.1 MB" of Wasm) is outside the allow
  list and was not read. Read 2026-10-28. Sure: confirmed (an absence across the sources above).

#### Legacy facts used (R4-PHYS)

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

### 2. Navigation meshes in Rust (R4-NAV)

#### Recast and Detour, the reference library (C++, zlib)
- **R4-NAV-F1.** Recast builds a navigation mesh from triangle meshes in steps: voxelize, remove voxels an agent cannot
  stand in, split the walkable voxels into regions, and re-triangulate the regions into polygons. Quotes: "Recast
  constructs a navmesh through a multi-step mesh rasterization process."; "First Recast rasterizes the input triangle
  meshes into voxels."; "Voxels in areas where agents would not be able to move are filtered and removed."; "The
  navigation polygons are generated by re-triangulating the generated polygonal regions into a navmesh." Source:
  https://github.com/recastnavigation/recastnavigation (README, main branch; no release shown). Tier: official. Read
  2026-10-28. Sure: confirmed in the source.
- **R4-NAV-F2.** Recast's agent is described by four numbers that map onto legacy movement: the steepest walkable slope,
  the floor-to-ceiling height, the highest ledge still walked over, and the radius by which walkable space is shrunk
  away from walls. Quotes: `walkableSlopeAngle` "The maximum slope that is considered walkable."; `walkableHeight`
  "Minimum floor to 'ceiling' height that will still allow the floor area to" "be considered walkable."; `walkableClimb`
  "Maximum ledge height that is considered to still be traversable."; `walkableRadius` "The distance to erode/shrink the
  walkable area of the heightfield away from" "obstructions."; `cs` "The xz-plane cell size to use for fields." Source:
  https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Recast/Include/Recast.h (main). Tier:
  official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-NAV-F3.** Detour is the runtime half: it loads the mesh, a set of tiles of convex polygons, and answers path
  queries. Quotes: Detour "Runtime loading of navmesh data, pathfinding, navmesh queries"; `dtNavMesh` "A navigation
  mesh based on tiles of convex polygons." Sources: the README and
  https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Detour/Include/DetourNavMesh.h. Tier:
  official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-NAV-F4.** Detour's path query is two-stage (a corridor of polygons, then a straight path through it), can be
  sliced over several updates with an iteration cap, and has a bounded node pool. Quotes: `findPath` "Finds a path from
  the start polygon to the end polygon."; `findStraightPath` "Finds the straight path from the start to the end position
  within the polygon corridor."; `updateSlicedFindPath` "Updates an in-progress sliced path query." with `maxIter` "The
  maximum number of iterations to perform."; `init` `maxNodes` "Maximum number of search nodes. [Limits: 0 < value <=
  65535]". Source:
  https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Detour/Include/DetourNavMeshQuery.h (main).
  Tier: official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-NAV-F5.** Detour's queries filter polygons by user flags and weight them by area type. Quotes: `dtQueryFilter`
  "Defines polygon filtering and traversal costs for navigation mesh query operations."; "Cost per area type."; poly
  flags "The user defined polygon flags."; `DT_MAX_AREAS` "The maximum number of user defined area ids." Sources:
  DetourNavMeshQuery.h and DetourNavMesh.h. Tier: official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-NAV-F6.** Recast and Detour are zlib-licensed and have no regular release process. Quotes: "Recast & Detour is
  licensed under the ZLib license."; "Unfortunately, `recastnavigation` does not have a regular release process."
  Sources: the README and https://github.com/andriyDev/recastnavigation-rs-sys (the binding's README). Tier: official
  (license); other (the binding author's statement). Read 2026-10-28. Sure: confirmed in the source.
- **R4-NAV-F7.** The Rust binding to the C++ library compiles it with C toolchain crates. Quote: "Raw FFI bindings for
  recastnavigation."; "Raw Rust bindings for `recastnavigation`, including Recast, Detour, DetourCrowd, and
  DetourTileCache."; build dependencies bindgen ^0.69.1, cc ^1.0.83, cmake ^0.1.50, pkg-config ^0.3.28. Sources:
  https://docs.rs/recastnavigation-sys/latest/recastnavigation_sys/ (recastnavigation-sys 1.0.3, released 2026-06-05)
  and the binding's README. Tier: official. Read 2026-10-28. Sure: confirmed in the source (the build-dependency list is
  the page's).
- **R4-NAV-F8.** Rust's browser target has no C or C++ toolchain, so C++ code such as Detour cannot simply be linked
  into the sim's `wasm32-unknown-unknown` build. Quotes: "This target currently has no equivalent in C/C++. There is no
  C/C++ toolchain for this target."; "While interop is theoretically possible it’s recommended to instead use one of:"
  (Emscripten or WASI targets). Source: https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html
  (rustc book, current). Tier: official. Read 2026-10-28. Sure: confirmed in the source; the consequence for Detour is
  inferred (R1 builds the sim for this target, R1-DET-F6).

#### Pure-Rust crates
- **R4-NAV-F9.** rerecast is a Rust port of Recast's generation steps only; its roadmap names cross-platform determinism
  through libm, and the repository has a comparison test against the C++ original. Quotes: "Rust port of Recast, the
  industry-standard navigation mesh generator"; "This crate contains low-level API."; the README: "Rust port of Recast,
  the industry-standard navigation mesh generator used by Unreal, Unity, Godot"; a "cpp comparison automated test".
  Sources: https://docs.rs/rerecast/latest/rerecast/ (rerecast 0.5.0; optional libm, bevy_reflect ^0.20) and
  https://github.com/janhohenheim/rerecast (README). Tier: official (crate docs); other (README claims). Read
  2026-10-28. Sure: confirmed in the source for the quotes; that it has no Detour, and whether the determinism item is
  done, are inferred (the fetch could not show the roadmap's check marks).
- **R4-NAV-F10.** oxidized_navigation generates tiles at runtime inside Bevy, asynchronously, whenever an affecting
  collider changes. Quotes: "A Nav-Mesh generation plugin for Bevy Engine."; "Takes in colliders that implement the
  OxidizedCollider trait from entities with the NavMeshAffector component and asynchronously generates tiles of
  navigation meshes based on NavMeshSettings."; "whenever the collider or GlobalTransform of any entity with a
  NavMeshAffector is changed." Source: https://docs.rs/oxidized_navigation/latest/oxidized_navigation/
  (oxidized_navigation 0.12.0; depends on bevy ^0.15). Tier: official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-NAV-F11.** polyanya plans any-angle paths on navigation meshes, needs no Bevy, and supports overlapping layers
  for floors with per-layer costs and layers that can be switched off. Quotes: "Implementation of Polyanya in Rust!
  Polyanya is a any-angle path planning algorithm."; "Overlapping navmeshes for 3D navigation (floors, bridges, …).";
  "Conditional layer traversal with the ability to enable or disable layers."; "Layers with different traversal costs
  for more realistic pathfinding."; "WASM demos made with Bevy are available here,". Source:
  https://docs.rs/polyanya/latest/polyanya/ (polyanya 0.18.0; dependencies include hashbrown, glam, spade, rstar;
  optional rerecast). Tier: official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-NAV-F12.** polyanya's only performance statement is a comparison without numbers. Quote: "This crate seems to
  generate a few more nodes, but tends to be faster than the cpp implementation." Source: the polyanya page. Tier:
  official (the author's claim). Read 2026-10-28. Sure: confirmed in the source.
- **R4-NAV-F13.** landmass is an engine-agnostic navigation system on a navmesh the user supplies; it covers path
  finding, path simplification, steering and local avoidance, and it manages its agents itself. Quotes: "A Rust crate to
  provide a navigation system for video game characters to walk around levels."; "This generally involves 4 things:"
  "Path finding (e.g. A-star)" "Path simplification (e.g. SSFA)" "Steering (e.g. boids)" "Local collision avoidance";
  `Character`: "While agents are “managed” by the archipelago," "characters are only as obstacles to be avoided by
  agents." Source: https://docs.rs/landmass/latest/landmass/ (landmass 0.9.3; depends on dodgy_2d ^0.5.4, slotmap,
  kdtree). Tier: official. Read 2026-10-28. Sure: confirmed in the source; that its avoidance is dodgy's ORCA is
  inferred from the dependency.
- **R4-NAV-F14.** The general `pathfinding` crate's A* returns a newly built path vector per call and needs a totally
  ordered cost type, so plain floats do not fit. Quotes: `astar(start: &N, successors: FN, heuristic: FH, success: FS)
  -> Option<(Vec<N>, C)>`; "Compute a shortest path using the A\* search algorithm."; bounds `C: Zero + Ord + Copy`.
  Sources: https://docs.rs/pathfinding/latest/pathfinding/ and .../directed/astar/fn.astar.html (pathfinding 4.16.0).
  Tier: official. Read 2026-10-28. Sure: confirmed in the source for the signature; "allocates per call" is inferred
  from the return type.
- **R4-NAV-F15.** foldhash's random state, which hashbrown can use by default, seeds every hasher at random;
  deterministic hashing needs its fixed state. Quotes: `RandomState` "always generates a random per-hasher seed";
  `FixedState` makes a map "trivially vulnerable to HashDoS attacks." Sources: https://docs.rs/foldhash/latest/foldhash/
  (foldhash 0.2.0) and https://docs.rs/hashbrown/latest/hashbrown/ (hashbrown 0.17.1; foldhash ^0.2.0 is an optional
  dependency). Tier: official. Read 2026-10-28. Sure: confirmed in the source; whether polyanya's maps use it and
  iterate in outcome-deciding code is not known (inferred risk).
- **R4-NAV-F16.** None of the crate pages read (rerecast, oxidized_navigation, polyanya, landmass, pathfinding,
  dodgy_2d) makes a determinism promise, and none gives generation times, query costs or memory for a map of a given
  size. Read 2026-10-28. Sure: confirmed (an absence across these pages; rerecast's roadmap item is a plan, F9).

#### Crowds that own their agents
- **R4-NAV-F17.** Detour's crowd module steers and moves its agents itself, each considering at most 6 neighbours.
  Quotes: `dtCrowd` "Provides local steering behaviors for a group of agents."; `update` "Updates the steering and
  positions of all agents."; "DT_CROWDAGENT_MAX_NEIGHBOURS = 6" "The maximum number of neighbors that a crowd agent can
  take into account" "for steering decisions." Source:
  https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/DetourCrowd/Include/DetourCrowd.h. Tier:
  official. Read 2026-10-28. Sure: confirmed in the source.

#### A shipped game's bot navigation: Quake III Arena's botlib (GPL source release)
- **R4-NAV-F18.** Quake III's bots route through a precomputed area graph whose links carry integer travel times.
  Quotes: "//reachability to another area"; "unsigned short int traveltime;//travel time of the inter area movement";
  "int traveltype; //type of travel required to get to the area". Source:
  https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/aasfile.h. Tier: shipped game. Read
  2026-10-28. Sure: confirmed in the source.
- **R4-NAV-F19.** It caches distances to each goal area (within a cluster, and from every cluster portal) and routes
  hierarchically through portals, accepting a route that is not the shortest. Quotes: "area routing cache:" "stores the
  distances within one cluster to a specific goal area"; "portal routing cache:" "stores the distances of all portals to
  a specific goal area"; "//NOTE: there might be a shorter route via another cluster!!! but we don't care"; "#define
  CACHE_REFRESHTIME 15.0f //15 seconds refresh time". Sources: .../code/botlib/be_aas_route.c and aasfile.h ("int
  cluster; //cluster the area belongs to, if negative it's a portal"). Tier: shipped game. Read 2026-10-28. Sure:
  confirmed in the source.

#### Legacy facts used (R4-NAV)

- **GN11:** a 2 m grid built at deploy from the bots' own collision (42.8 ms on Cabana); buildings are not cells, so
  27.5 % of objective routes cross a roof bots cannot climb (R19); the editor's Play never rebuilds it (N1); odd-angle
  colliders leave up to 0.34 m; 3-5.4 m gaps can close (hence LOOP_V2 §9's 6 m rule).
- **GN12:** eight-neighbour A* on preallocated typed arrays with a search stamp, no allocation, 0.26 ms per search
  (AUDIT CS13), capped at 8,000 expansions, string-pulled into at most 64 waypoints; parity notes (f64 maths stored in
  f32, tie rules).
- **GN13:** replans every 1.5 s, when the goal moves 3 m, or when stuck; the budget of 2 searches resets per
  `update()` call, so searches per second grow with frame rate; the replan storm; route denials 52-57 % at 48 soldiers.
- **GN5, GN9:** strafing and backing off skip navigation; bots pick goals by role (push, objective, hunt), so many bots
  share a few goals.
- **TL14:** the reachability check (every spawn to every goal, flood fill as ground truth, climb sampling) is to become
  "a 3D navigation check inside the Rust sim, callable from the editor and CI".
- **TL13 (via GN5):** headless stall rate, base mean 8.82 % against 7.73 % after the bot-poplar-wedge fix (40 runs).
- Design inputs: LOOP_V2 §9 (6 m gaps, reachability on Octopon-3), §10 (replan storm before uplinks), §11 (scale test
  N = 12 / 24 / 36 / 48); EDITOR_V2 §2 rule 5 (validation testable in Node), §3 (validation panel; Test plays the map
  with bots instantly).

### 3. Bots moving between floors (R4-FLOORS)

- **R4-FLOORS-F1.** Recast's heightfield keeps a column of spans per grid cell, each pointing to the next span higher
  up, so surfaces stacked at the same x and z (a floor above a floor, a roof over a room) stay separate; it can also
  split the field into non-overlapping layers. Quotes: `rcSpan::next` "The next span higher up in column";
  `rcHeightfield` "A dynamic heightfield representing obstructed space."; `rcBuildLayerRegions` "Builds region data for
  the heightfield by partitioning the heightfield in non-overlapping layers." Source:
  https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Recast/Include/Recast.h (main). Tier:
  official. Read 2026-10-28. Sure: confirmed in the source for the quotes; that this is how stacked floors are kept
  apart is inferred from the column layout.
- **R4-FLOORS-F2.** Detour models any non-walking move as an off-mesh connection: two user-placed endpoints with a
  radius, one-way unless flagged bidirectional, with a user id. Quotes: "An off-mesh connection is a user defined
  traversable connection made up to two vertices."; `pos` "The endpoints of the connection. [(ax, ay, az, bx, by,
  bz)]"; `DT_OFFMESH_CON_BIDIR` "A flag that indicates that an off-mesh connection can be traversed in both directions.
  (Is bidirectional.)"; `userId` "(User assigned when the navigation mesh is built.)" Source:
  https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Detour/Include/DetourNavMesh.h (main). Tier:
  official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-FLOORS-F3.** landmass has the same idea as an "animation link". Quote: "A link connecting two edges where an
  agent must perform some action (or animation) to use the link." Source: https://docs.rs/landmass/latest/landmass/
  (landmass 0.9.3). Tier: official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-FLOORS-F4.** polyanya adds one-way layers to its overlapping floor layers. Quote: "One-way layers for directional
  movement." (beside "Overlapping navmeshes for 3D navigation (floors, bridges, …).", R4-NAV-F11). Source:
  https://docs.rs/polyanya/latest/polyanya/ (polyanya 0.18.0). Tier: official. Read 2026-10-28. Sure: confirmed in the
  source.
- **R4-FLOORS-F5.** Quake III's bots know 19 typed ways to move between areas, among them walking, crouching, jumping
  onto a barrier, jumping, ladders, walking off a ledge, teleporters, elevators and jump pads. Quotes: "#define
  TRAVEL_WALK 2 //walking"; "TRAVEL_BARRIERJUMP 4 //jumping onto a barrier"; "TRAVEL_JUMP 5 //jumping";
  "TRAVEL_LADDER 6 //climbing a ladder"; "TRAVEL_WALKOFFLEDGE 7 //walking of a ledge"; "TRAVEL_JUMPPAD 18 //jump pad".
  Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/aasfile.h (GPL release).
  Tier: shipped game. Read 2026-10-28. Sure: confirmed in the source.
- **R4-FLOORS-F6.** Quake III finds jump links by predicting the player's own movement across the gap, and step and
  barrier links by comparing heights with the movement's maximum step and barrier. Quotes: "creates possible jump
  reachabilities between the areas"; "Between these two points there must be one or more gaps."; "If the gaps exist a
  potential jump is predicted."; "AAS_PredictClientMovement(&move, -1, beststart, PRESENCE_NORMAL, qtrue,"; "if
  (!AAS_HorizontalVelocityForJump(phys_jumpvel, beststart, bestend, &speed))"; "if (ground_bestdist >= 0 &&
  ground_bestdist < aassettings.phys_maxstep)"; "< aassettings.phys_maxbarrier". Source:
  https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/be_aas_reach.c (GPL release; the
  first 100,000 of 152,877 characters read). Tier: shipped game. Read 2026-10-28. Sure: confirmed in the source.
- **R4-FLOORS-F7.** Each Quake III link records where the move starts and ends and what kind of travel it is, and each
  area records how a bot can be present in it. Quotes: "vec3_t start; //start point of inter area movement"; "vec3_t
  end; //end point of inter area movement"; "int traveltype; //type of travel required to get to the area"; "int
  presencetype; //how a bot can be present in this area". Source: aasfile.h. Tier: shipped game. Read 2026-10-28. Sure:
  confirmed in the source.
- (Used, read under other questions: R4-NAV-F2 for Recast's step, height and slope limits; R4-PHYS-F2 and F24 for
  step-up in movement; R4-NAV-F18 for integer travel times.)

#### Legacy facts used (R4-FLOORS)

- **GN10:** bots climb only in combat, by walking to the low end of the nearest ramp (chosen by its centre, whether or
  not it leads to the target's ledge), and jump when the ground 1.5 m ahead is within `EJUMP_REACH`; the nav grid has
  no height. Inventory note: "give the nav graph heights and links (ramps, jumps) so A* plans the climb".
- **GW14 (tuning `T.jumpV` 11, `T.gravity` 24, `T.stepUp` 0.55, `EJUMP_REACH` 3.0708):** the real reach at 60 Hz is
  about 2.98 m, below the formula's 3.07 m; only the player can stand on cover tops ("v2 needs one rule for every
  soldier, since humans take over bot slots").
- **GN11 (R19):** buildings are not nav cells; 27.5 % of Cabana's objective routes and 25 % of siege legs cross a roof
  bots cannot climb.
- **TL14:** reach.js samples routes every 0.25 m for surfaces higher than `EJUMP_REACH`; Cabana objective climbs
  72.5 %, DM_Tri 100 %.
- **GW7, GW20:** ramps are data (w, d, h0, h1, rot); the barracks is enterable, with a roof (LOOP_V2 §2 spawns everyone
  inside it).
- Design inputs: VISION_V2 §6.2 ("bots that move between levels"; "floors and building interiors, ledges");
  LOOP_V2 §9 (raised centre plateau with ramps, drop-downs from high ground for fast rotations; enterable structures
  [LATER]); EDITOR_V2 §3 (plateaus, ramps, cliff edges, drop-downs; interiors [LATER]).
- Wave 1: R1-TICK-REC6 (reach computed with the sim's own integrator at the shipped tick).

### 4. Line of sight across heights (R4-LOS)

- **R4-LOS-F1.** Quake III's bots judge whether they see an enemy by tracing from their eye to three points on the
  enemy's box (its centre, its bottom and its top), after a field-of-view check, and return a visibility fraction.
  Quotes: "returns visibility in the range [0, 1] taking fog and water surfaces into account"; "for (i = 0; i < 3; i++)
  {"; "if (i == 0) middle[2] += entinfo.mins[2];"; "else if (i == 1) middle[2] += entinfo.maxs[2] - entinfo.mins[2];";
  "BotAI_Trace(&trace, start, NULL, NULL, end, passent, contents_mask);"; "if (!InFieldOfVision(viewangles, fov,
  entangles)) return 0;". Source:
  https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/ai_dmq3.c (GPL release; the first
  100,000 of 158,046 characters read), `BotEntityVisible`. Tier: shipped game. Read 2026-10-28. Sure: confirmed in the
  source.
- **R4-LOS-F2.** Before any trace, Quake III's enemy search drops clients beyond a distance that depends on the bot's
  alertness; a potentially-visible-set test is present only as a commented-out line. Quotes: "if (squaredist >
  Square(900.0 + alertness * 4000.0)) continue;"; "vis = BotEntityVisible(bs->entitynum, bs->eye, bs->viewangles, f,
  i);"; "//if (!AAS_inPVS(eye, middle)) continue;". Source: the same file, `BotFindEnemy` and `BotEntityVisible`. Tier:
  shipped game. Read 2026-10-28. Sure: confirmed in the source; the field-of-view value (360 when hurt or shot at,
  otherwise distance-dependent) is from the fetch summary (inferred).
- **R4-LOS-F3.** parry3d's BVH traversal is under the caller's control, so a sight test can stop at the first blocker
  instead of looking for the nearest one. Quotes: `TraversalAction` "Controls the execution flow of `Bvh::traverse`.";
  `traverse` "Traverses the BVH in depth-first order with full control over traversal."; `cast_ray` "delegates the
  ray-casting task to an external function." Sources: https://docs.rs/parry3d/latest/parry3d/partitioning/index.html and
  .../struct.Bvh.html (parry3d 0.31.1). Tier: official. Read 2026-10-28. Sure: confirmed in the source; the early exit
  is inferred from "full control".
- **R4-LOS-F4.** In Rapier's queries a "solid" ray that starts inside a shape hits it at distance 0, while a non-solid
  one treats shapes as hollow. Quotes: "if `solid` is `true` then the hit point will be the ray origin itself (`toi =
  0.0`)"; "If `solid` is `false` then the shape will be assumed to have an empty interior". Source:
  https://rapier.rs/docs/user_guides/rust/scene_queries (Rust 0.36). Tier: official. Read 2026-10-28. Sure: confirmed in
  the source. (Relevant to legacy CS4's inside-start miss: the start rule must be chosen, not inherited.)
- **R4-LOS-F5.** No source read in this run gives the cost of one ray or segment query through a BVH, in Rust or in
  Wasm. Read 2026-10-28. Sure: confirmed (an absence; R4-PHYS-F13 and F27 say the same for Rapier; R2-INT-OQ5 asks for
  it).
- (Used, read under other questions: R4-PHYS-F10, F12 (queries and filters), F21 (a BVH built once for a static
  scene); R4-FLOORS-F1 (stacked floors kept apart).)

#### Legacy facts used (R4-LOS)

- **CS9:** `hasLOS` casts a segment against statics only (soldiers never block), stops 0.5 m short of the target, and
  counts the first hit as blocked; 2.7-5.7 µs per call in JS (B7); inventory: "Decide explicitly whether units and
  vehicles block sight"; the 0.5 m margin is a tuning value.
- **CS6:** no broad phase; the parity-fuzz method (20,000 random rays) is the port test.
- **GN4 (tuning `TUNE.ai` retargetS 0.25, losPerS 480, losRecheckS 0.1; `T.enemyDetect` 44):** bots scan every 0.25 s
  for the nearest visible enemy within 44 m and re-check their target every 0.1 s, on a shared budget of 480 sight
  checks per game second against a demand of about 1,100/s at 48; at 48 soldiers each bot scans about 2.5 times per
  second instead of 4; eyes at +1.45 m.
- **GN7, GW7:** ramps are boxes in the hit world, so sight and shots disagree on ramps.
- **GW15, GC14:** blasts use three.js raycasts against render meshes, which miss trees, props, platforms and ramps and
  keep destroyed buildings.
- **GC12:** melee has no line-of-sight or height check (it hits through walls and up onto platforms).
- **GC3:** the cursor ray resolves through the SIM; on a ramp it stops at the box top (2.50 m where the slope is at
  1.25 m).
- **GW16:** covers and props carry separate flags for "solid" and "blocks sight"; non-blocking ones are left out of the
  sight world.
- Wave 1: R2-INT-F20 (enemy-pair counts: 576 at 24v24, 768 at 16v16v16, 432 at 12v12v12, and the legacy-cost
  arithmetic), R2-INT-REC2 (areas and portals, O(1) connectivity), R2-INT-REC3 (one shared visibility pass, ≤ 10 % of
  the tick budget), R2-SYNC-REC5 and REC8 (rewound shots; aim point at height), R2-INT-OQ1 (what a player may see),
  R2-INT-OQ5 (the cost of a BVH ray).

### 5. Obstacles placed mid-match (for later building) (R4-OBST)

Scope (task rule 11): the need is VISION_V2 §6.4, big vehicles as moving obstacles, plus structures that change state
(destroyed, rebuilt). Player building is only "don't design it out": nothing here designs a building system.

- **R4-OBST-F1.** Detour's tile cache takes temporary obstacles as cylinders, axis-aligned boxes or boxes rotated about
  the vertical axis, and applies them by rebuilding the tiles they touch. Quotes: "DT_OBSTACLE_CYLINDER,";
  "DT_OBSTACLE_BOX, // AABB"; "DT_OBSTACLE_ORIENTED_BOX // OBB"; "// Box obstacle: can be rotated in Y."; `update`:
  "Updates the tile cache by rebuilding tiles touched by unfinished obstacle requests." Source:
  https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/DetourTileCache/Include/DetourTileCache.h
  (main). Tier: official. Read 2026-10-28. Sure: confirmed in the source.
- **R4-OBST-F2.** The tile cache works through a bounded queue and may need several updates to catch up. Quotes: "static
  const int MAX_REQUESTS = 64;"; "static const int MAX_UPDATE = 64;"; `upToDate` "Whether the tile cache is fully up to
  date with obstacle requests and tile rebuilds."; obstacle states "DT_OBSTACLE_PROCESSING," "DT_OBSTACLE_PROCESSED,"
  "DT_OBSTACLE_REMOVING". Source: the same header. Tier: official. Read 2026-10-28. Sure: confirmed in the source; that
  a moving obstacle would keep re-queuing rebuilds is inferred.
- **R4-OBST-F3.** dodgy_2d computes avoiding velocities with ORCA against both agents and static obstacle polygons.
  Quotes: "An implementation of ORCA, a local collision avoidance algorithm for 2D."; "essentially a port of RVO2 to
  Rust,"; `Agent` "A single agent in the simulation."; `Obstacle` "A single obstacle in the simulation." Source:
  https://docs.rs/dodgy_2d/latest/dodgy_2d/ (dodgy_2d 0.5.5; docs page dated 2026-10-08). Tier: official. Read
  2026-10-28. Sure: confirmed in the source; the inputs of `compute_avoiding_velocity` (neighbours, obstacles, preferred
  velocity, maximum speed, time step, time horizons) are from the fetch summary of the example (inferred); it works in
  f32 and makes no determinism statement (inferred).
- **R4-OBST-F4.** Quake III can mark a link as temporarily impossible instead of rebuilding its area graph. Quote:
  "#define TRAVEL_INVALID 1 //temporary not possible". Source:
  https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/aasfile.h. Tier: shipped game. Read
  2026-10-28. Sure: confirmed in the source; how and when the game sets it is not in the file read (inferred use).
- (Used, read under other questions: R4-NAV-F5 (Detour's poly flags and area costs filter queries), R4-NAV-F10
  (oxidized_navigation regenerates tiles asynchronously on collider changes), R4-NAV-F11 (polyanya layers that can be
  enabled or disabled), R4-NAV-F13 (landmass: characters are "only as obstacles to be avoided by agents"), R4-NAV-F17
  (Detour's crowd moves its own agents).)

#### Legacy facts used (R4-OBST)

- **GC17 (tuning `JUG.maxSpeed` 6, `JUG_PILOT.moveSpeed` 5.5):** the Juggernaut is deployed 7 m behind the buyer
  mid-match, idles when unpiloted, has no obstacle collision and is not in the hit world, so bots neither hit nor avoid
  it.
- **HARV (tuning `speed` 7, `lanes`):** harvesters drive between field and refinery on lanes; they are in the sight
  world as boxes (GW16), and nothing records bots avoiding them.
- **GN11:** the nav grid is built once at deploy and never updated in a match (and not on the editor's Play, N1).
- **GN13:** bots replan on goal moves, timers and stalls; the replan storm happens when a straight line flickers
  between clear and blocked; LOOP_V2 §10: moving objectives will make bots replan constantly.
- **GW15, GW20:** destroyed buildings stay as ruins that sink to 15 % of their height; bullets ignore them, blasts do
  not.
- Design inputs: VISION_V2 §6.4 (big, multi-part vehicles), LOOP_V2 §3 (harvesters killed and rebuilt), §6 (the
  Juggernaut and faction tanks), §8 (a destroyed HQ is a points swing); R2-INT-REC2 (a destroyed wall opens a portal and
  re-floods connectivity); R2-INT-F4 (Quake III flips area connectivity when a door changes, by re-flooding).

## Options

### 1. Rapier 3D or custom (R4-PHYS)

#### What the simulation asks of physics (from VISION_V2 §6, LOOP_V2 and the legacy rows; the author's reading)

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

#### The options

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

### 2. Navigation meshes in Rust (R4-NAV)

| | A. Recast/Detour (C++) through bindings | B. Pure-Rust crates at runtime (rerecast + polyanya or landmass) | C. A sim-owned navigation runtime on baked data | D. Legacy grid, ported and layered |
|---|---|---|---|---|
| Generation | Recast, mature (F1, F2) | rerecast, 0.5.0 (F9) | A tool step, output stored in the map document; generator chosen by the spike (rerecast's mesh or D's grid) | Our grid from the sim's collision (GN11 method) at match start |
| Runtime queries | Detour (F3-F5) | polyanya (F11) or landmass (F13) | Our own A* over the baked nodes plus string pulling, per-goal distance tables, typed links | Our A* (GN12) on cells |
| Runs in the browser sim | No: no C/C++ toolchain for the target (F8); offline bots would need a second build | Yes (polyanya shows Wasm demos, F11) | Yes | Yes |
| Determinism | C++ floats outside R1-DET-REC2's Rust rules (inferred) | No promise (F16); hashbrown's random seeds are a risk to audit (F15) | Ours: integer link costs (F18), id order, R1-DET-REC2; generation runs once, so its floats never differ between server and client | Ours |
| State ownership | DetourCrowd moves agents itself (F17) | landmass manages agents (F13) | The sim's tables; navigation data is read-only | The sim's tables |
| Path quality and cost | Good; sliced queries (F4) | Any-angle optimal (F11); costs unknown (F12, F16) | Depends on the generator; Q3 shows a coarse area graph is enough for shipped bots (F18, F19) | Many nodes; 8,000 cap failures (GN12); 2 m cells close gaps (GN11) |
| Effort and risk | Low effort, blocked by F8 | Low effort, young 0.x dependencies | Medium: queries and funnel are ours; generation reuses a crate or D | Low; quality limits known |
| Fit with §6.2 | Floors and links (FLOORS) | Floors via layers (F11) | Floors via typed links and layers we define (FLOORS) | Needs layers and links added |

### 3. Bots moving between floors (R4-FLOORS)

| | A. Links placed by hand | B. Links generated by the bake, by simulating movement, plus hand overrides | C. No links: ramps and stairs only |
|---|---|---|---|
| How | The designer places every jump and drop in the editor (Detour style, F2) | Steps and ramps are walkable surfaces; the bake proposes jump-up, drop-down and gap links where heights allow, and keeps only those the sim's own movement completes (Q3 style, F6); designers add or forbid links | Bots walk ramps; ledges and drop-downs are not used |
| Correctness | A hand link can promise a jump the movement cannot make (legacy's formula vs integrated reach, GW14) | Every link is a measured move at the shipped tick (R1-TICK-REC6) | Always walkable, but routes are long |
| Editor work | High, repeated on every map change | Low; overrides only | None |
| Fit with §6.2 and LOOP_V2 §9 | Yes, at authoring cost | Yes: ledges, drop-downs, interiors | No: drop-downs "for fast rotations" are lost |
| Cost | None at bake | Bake time grows with candidate links (each simulated) | None |

### 4. Line of sight across heights (R4-LOS)

| | A. Rays against statics only (legacy) | B. Rays against statics and selected movers | C. Precomputed visibility (PVS-like) only |
|---|---|---|---|
| What | Segment queries through the static BVH | As A, plus vehicle hulls (and soldiers, if the rule says) from the per-tick movers index | Visibility between map cells baked offline; no rays at runtime |
| Height | Yes, if eyes and targets are 3D points and ramps are slopes | Yes | Coarse: per cell, not per point |
| Cost | Bounded by pairs × points (R2-INT-F20) | Plus the movers index (rebuilt each tick anyway, R4-PHYS-REC2) | Cheapest at runtime; bake cost; wrong for moving cover |
| Fit | §6.2; vehicles never give cover | §6.2 and §6.4 (big vehicles as cover, R2-SYNC-OQ8) | Not enough alone: open-ground walls and moving vehicles; Q3 itself commented its PVS test out of bot sight (F2) |

### 5. Obstacles placed mid-match (for later building) (R4-OBST)

| | A. Carve the mesh (tile rebuilds at runtime) | B. Avoidance only | C. Flags on baked data, plus avoidance |
|---|---|---|---|
| Moving vehicles | Rebuild the tiles under a vehicle as it moves (F1, F2): requests every tick it moves | Vehicles are large obstacles in the avoidance step; routes ignore them | As B, and nodes under a parked or slow vehicle get a temporary cost or block flag, refreshed past a movement threshold |
| Structures that change | Rebuild the touched tiles | Not handled (a destroyed wall never opens a route) | Baked in both states; the change flips node, link and portal flags (F4; R4-NAV-F5, F11) |
| Runtime generation in the sim | Yes, on both targets, every match | No | No |
| Determinism | The rebuild must be bit-identical on native and Wasm and land on the same tick; async designs do not (R4-NAV-F10) | Avoidance order and floats only (R4-NAV-REC3) | Flag flips on a tick are trivially identical |
| Replans | Every rebuild can invalidate routes (the storm risk, GN13) | None | Only bots whose route crosses a changed node |
| Effort | Highest | Lowest | Medium |
| Fit with §6.4 | Yes, at a high cost | Bots grind against big hulls without route changes | Yes |

## Recommendations

Every pass mark and every reference machine below is proposed (VISION_V2 §3, §4 and §6 give the only targets).
Reference hardware, where a mark needs it, is the one R1-TICK-REC2 proposes (one core of the R6 server type; the
VISION_V2 §4 minimum laptop for Wasm).

### 1. Rapier 3D or custom (R4-PHYS)

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
  (R1-DET-F23); it changes versions as often as Rapier (F22), but the surface we would use is small (shapes, queries,
  BVH), so an upgrade touches one sim module. Avian is Bevy-bound (F25), which R1-ECS-REC1's no-ECS layout rules out.
  Rapier stays the measured fallback because its controller already does autostep, snapping and slopes (F2).
  Rests on: F1, F2, F3, F4, F6, F15, F19, F20, F21, F22, F23, F24, F25, F27; R1-ECS-REC1 (marks 1, 2 and 5),
  R1-ECS-REC4, R1-TICK-REC2 (reference hardware), R1-TICK-REC6, R1-DET-REC1, R1-DET-F23; legacy GC3, GW13, GW14;
  tuning `T.stepUp`, `moveSpeed`, `sprintMult`.
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
  Rests on: F7, F8, F20; legacy GC17, GW13 (radius 0.7); tuning `JUG.maxSpeed`; R1-TICK-OQ9, R1-DET-REC3,
  R1-ECS-REC1 (mark 1's stress load); LOOP_V2 §3 and §6.
  Serves: VISION_V2 §6.4 (big, multi-part vehicles).
  A spike must prove (proposed): a vehicle driven into walls, ramps and a crowd of 48 soldiers for 10 minutes has 0
  ticks that end with a soldier or a wall inside its hull (penetration > 1 cm); the stress load for R1-DET-REC3 and
  R1-ECS-REC1 mark 1 keeps 8 big vehicles (R1-ECS-REC1's number), harvesters included (LOOP_V2 §3; how many per team
  is the game-designer's), at 30 and 60 Hz.

- **R4-PHYS-REC5. f32 for the whole sim, revisited only if a map exceeds 2 km across.**
  The reason: R1-DET-OQ3 leaves f32 or f64 to R4 with the map size. Legacy spans 372 m (`GROUND_W`); LOOP_V2 §9 starts
  Octopon-3 near Cabana's footprint. Within ±1,024 m of the origin f32 positions are spaced at most 0.06 mm apart
  (F26), far below the 1 mm marks above; rapier3d and parry3d default to f32 (F17), and Rapier's controller takes f32
  (F3). Rounding still accumulates as positions are updated every tick; at 60 Hz a soldier at `moveSpeed` 6.3 m/s moves
  about 0.1 m per tick, so half a spacing (0.03 mm) per tick is a drift of about 0.03 % in distance covered (inferred
  arithmetic), the same on every target, which the spike measures rather than assumes.
  Rests on: F3, F17, F26; legacy `GROUND_W`, `arenaX`, `arenaZ`, `moveSpeed`; R1-DET-OQ3; LOOP_V2 §9 (map size
  [OPEN]); REC1 and REC3 (their marks are reused).
  Serves: VISION_V2 §6.2.
  A spike must prove (proposed): with the greybox placed at the far corner of the largest planned map, REC1's
  movement-parity marks and REC3's tunnelling mark still hold, and a scripted soldier walking a fixed 10-minute route
  there covers the same distance per second as at the origin within 0.1 %.

### 2. Navigation meshes in Rust (R4-NAV)

- **R4-NAV-REC1. The sim owns the navigation runtime; the navigation data is baked by a tool and stored in the map
  document (option C). The spike compares two generators for the bake: rerecast's polygon mesh and a layered grid built
  from the sim's own collision queries.**
  The navigation data is read-only during a match: convex nodes (polygons or cells) with heights, the portals between
  them, typed links (FLOORS), area ids and flags (OBST), and integer traversal costs. The sim crate holds the queries:
  nearest node, A* over nodes, a string-pulled straight path, link traversal. The bake runs in the editor's Test, on map
  save and in CI, from the same collision world the sim moves in (R4-PHYS-REC2), with Recast's four agent numbers set
  from the soldier's movement data (radius 0.7 m, `T.stepUp` 0.55 m, the slope limit and height).
  The reason: the client runs the same sim for offline play, bots included (the starting principles), and C++ Detour
  cannot join the sim's browser build (F8). R1-DET-REC1 (b) asks a bots-only match to hash the same on native and Wasm,
  and no Rust navigation crate promises determinism (F16); polyanya depends on hashbrown, whose optional default
  hasher seeds itself at random (F11, F15), though whether that reaches path results is not known. Baking
  once makes generation's floating-point results shared data rather than something each target recomputes. Crowd
  layers that move agents themselves (F17, F13) would split position ownership away from the sim's movement and
  snapshot (R1-ECS-REC1). Quake III shipped bots on exactly this shape: a precomputed area graph with typed links and
  integer travel times (F18, F19). Legacy proves the rest: build from the movement collision so bots and movement agree
  (GN11), search on preallocated arrays without allocating (GN12). Which generator is better for Octopon-3's blocks,
  ramps and plateaus is not settled by any source (F12, F16), so it is measured.
  Rests on: F1, F2, F8, F9, F11, F12, F13, F15, F16, F17, F18, F19; R1-DET-REC1, R1-DET-REC2, R1-DET-F6, R1-ECS-REC1,
  R1-TICK-REC2 (reference hardware); R4-PHYS-REC2; legacy GN11, GN12, TL14; tuning `T.stepUp`, bot radius 0.7 (GW13);
  LOOP_V2 §9 (6 m gaps, reachability); EDITOR_V2 §3 (Test, validation panel).
  Serves: VISION_V2 §6.2 (bots that move between levels) and §6.1 (bots fill empty slots in every mode).
  A spike must prove (proposed pass marks; the reference hardware of R1-TICK-REC2):
  1. **Bake time.** The Octopon-3 greybox (LOOP_V2 §9 layout) bakes in ≤ 1 s native and ≤ 3 s in a browser Worker,
     for each generator, so the editor's Test stays quick (EDITOR_V2 §3).
  2. **Coverage.** Of 10,000 points where a scripted soldier stood during movement runs, ≥ 99.9 % lie on a node; of
     10,000 random points on nodes, 0 are places a 0.7 m soldier cannot stand (checked with the sim's own movement).
  3. **Gaps.** A 6 m gap between two blocks stays a route; the bake reports every gap under 6 m (LOOP_V2 §9).
  4. **Reachability.** The redesigned check (TL14: every spawn to every goal, flood fill as ground truth) runs inside
     the sim crate, from Node for the editor and from CI, with identical results on native and Wasm.
  5. **Determinism.** 10,000 random path queries give byte-identical paths on native and Wasm; the baked data hashes
     the same wherever it is loaded.
  6. **The comparison.** For both generators: node count, data size in the map document, mean and p99 query cost, and
     path length against the straight-line optimum; the ADR picks by marks 1 to 5 first, then these numbers.

- **R4-NAV-REC2. Path requests go through a queue with a fixed budget per tick, measured in node expansions, served in
  a rotating fair order; bots heading to shared goals read cached per-goal distance tables.**
  Each bot asks for a route when it has none, when its goal moves past a threshold, or when it is stuck, with
  hysteresis so a flickering straight line does not trigger a storm (LOOP_V2 §10). Shared goals (uplink sites, the
  harvest fields, each base's entrances, the carried uplink cache) get a distance table over the nodes, filled
  incrementally inside the same budget and reused by every bot; a bot then follows decreasing distance and smooths with
  the straight-path step. If the node count makes tables too large, nodes are grouped into clusters with portals.
  The reason: legacy's budget counts searches per `update()` call, so searching grows with frame rate (GN13), and the
  replan storm is a known bug; Detour itself caps work per update (F4); Quake III serves every bot from per-goal cached
  distances, hierarchically, and accepts a slightly longer route (F19); many bots share a few goals (GN9).
  Rests on: F4, F19; legacy GN5 (the stall figures), GN9, GN12, GN13, TL13; LOOP_V2 §10; R1-TICK-REC2 (the P1
  scene).
  Serves: VISION_V2 §6.1 (48+ players with bots) and §6.2 (bots between levels).
  A spike must prove (proposed): in R1-TICK-REC2's P1 scene at N = 48 and N = 64 bots with an uplink cache carried
  around the map: navigation (queries plus route following) takes ≤ 10 % of the tick's p99 budget; no bot waits more
  than 1 s for a route (legacy: 52-57 % route denials at 48, GN13); at most 2 replans per bot per second at p99; the
  headless stall rate is no worse than legacy's best measured 7.73 % (TL13 method, 3 runs per build).

- **R4-NAV-REC3. Local avoidance is a step inside the sim's movement phase, never a crowd system that moves agents.**
  Each bot's route gives a preferred velocity; an avoidance step adjusts it against neighbours taken from the movers'
  index in id order, and the result goes through the same movement as a player's input (R4-PHYS-REC1). The first
  candidate is ORCA through dodgy_2d (F13's dependency; R4-OBST-F3: dodgy_2d 0.5.5 is "An implementation of ORCA" and
  "essentially a port of RVO2"), adopted only after an audit against R1-DET-REC2's float rules; a plain separation force
  is the fallback.
  The reason: legacy has no avoidance and its strafing skips navigation (GN5); crowd modules that update positions
  themselves (F17, F13) would split position ownership from the sim; neighbour order must be stable for determinism
  (R1-ECS-REC2).
  Rests on: F13, F17; R1-ECS-REC2, R1-DET-REC2; R4-PHYS-REC1; legacy GN5; R4-OBST-F3; LOOP_V2 §9 (the 6 m gap).
  Serves: VISION_V2 §6.1 and §6.2.
  A spike must prove (proposed): 48 bots crossing in a 6 m choke point (LOOP_V2 §9's gap) for 10 minutes: 0 deadlocks
  longer than 3 s, and per-tick state hashes identical on native and Wasm; the avoidance step costs ≤ 0.2 ms p99 native
  at 48 (proposed).

- **R4-NAV-REC4. Navigation runs where bots run: on the server, and in the browser only for offline or local-bot play.
  Client prediction never runs it. Navigation data changes only with the map: a new bake means a new match.**
  The reason: R1's cross-topic note asks whether navigation runs only on the server. Prediction replays only the local
  player or its vehicle (R1-TICK-REC2 P3), which never needs a route; offline play runs bots in the browser (the starting
  principles), so the code must also build for the browser (F8). R1-DATA-OQ7 asks what may reload mid-match: navigation
  data no (a re-bake needs a restart); flag toggles on baked data yes (OBST).
  Rests on: F8; R1-TICK-REC2, R1-DATA-OQ7.
  Serves: VISION_V2 §6.1.
  A spike must prove (proposed): the client's Worker build without bots skips the navigation data (its size reported);
  with bots, offline play at the largest mode meets R1-TICK-REC2's P2 mark.

### 3. Bots moving between floors (R4-FLOORS)

- **R4-FLOORS-REC1. Typed links generated by the bake and checked by running the sim's own movement, with editor
  overrides (option B).**
  Link types start with what the maps need today: walk (implicit, across shared edges, slopes and steps within limits),
  jump-up onto a ledge, drop-down off a ledge (one-way), and a gap jump; the type list is data, open to later ones
  (ladder, jump pad) without a format change. Each link stores its entry and exit points, its type, the input a soldier
  issues (none, jump), its direction, and an integer travel time measured during the check. A bot uses a link by
  issuing the same inputs a player would: the link never moves the bot itself.
  The reason: VISION_V2 §6.2 asks for bots that move between levels, and LOOP_V2 §9 has drop-downs and ramps. Quake III
  shipped exactly this: typed links between areas, jump links found by predicting the player's own movement, step and
  barrier links from the movement's limits (F5, F6, F7). Legacy shows why a formula is not enough: the bots' reach
  formula overstates the jump by about 9 cm at 60 Hz (GW14), and climb steering picks ramps that may not lead to the
  target (GN10). Detour, landmass and polyanya all represent such moves as links or one-way layers (F2, F3, F4), so the
  data model fits whichever generator R4-NAV-REC1 picks. Simulated links are what R1-TICK-REC6 asks of reach.
  Rests on: F2, F3, F4, F5, F6, F7; R4-NAV-F18; R1-TICK-REC6; legacy GN10, GN11 (R19), GW14, TL14; LOOP_V2 §9.
  Serves: VISION_V2 §6.2 (bots that move between levels; ledges).
  A spike must prove (proposed pass marks): on the Octopon-3 greybox (plateau, ramps, drop-downs) and a test block
  with ledges at 2.5 m, 2.9 m, 3.0 m and 3.1 m: (a) 0 generated jump links above the integrated reach (2.9 m kept, 3.0
  m and 3.1 m rejected at 60 Hz, if the measured reach stays near legacy's 2.98 m); (b) each kept link succeeds in 1,000
  of 1,000 bot traversals in the sim, at 30 and 60 Hz; (c) from every spawn to every goal, routes exist and the share of
  routes needing a surface bots cannot reach is 0 % (legacy Cabana: 27.5 %, R19); (d) on high ground, a bot whose goal is
  below takes a drop-down whenever it is the shorter route (≥ 95 % of 200 trials).

- **R4-FLOORS-REC2. Floors are separate layers with ceilings from day one, even though interiors are [LATER].**
  The bake treats a surface as walkable only if the soldier fits under what is above it (Recast's floor-to-ceiling
  height, R4-NAV-F2), keeps stacked surfaces apart (F1), and gives each node its layer and its enclosure area id, the
  same area ids as R2-INT-REC2's interiors-and-floors map data, so navigation, line of sight and interest agree on which
  floor a soldier is on.
  The reason: the barracks is enterable and roofed in the first greybox (LOOP_V2 §2, GW20), and enterable structures
  come later (LOOP_V2 §9 [LATER]); a 2D grid cannot represent a floor above a floor (GN10, GN11), and fixing that later
  would change the data format. Sharing area ids with R2-INT-REC2 avoids two floor models.
  Rests on: F1; R4-NAV-F2; R2-INT-REC2; legacy GN10, GN11, GW20.
  Serves: VISION_V2 §6.2 (floors and building interiors).
  A spike must prove (proposed): in a two-floor test building with stairs (R2-INT-REC2 (a)'s greybox): (a) nodes on
  each floor carry different layer and area ids and the only edges between floors run along the stairs; (b) a bot on
  the ground floor reaches a goal on the floor directly above in 100 of 100 trials, never by trying to walk through the
  ceiling; (c) the nodes under the barracks roof are walkable and the roof is not, unless a link reaches it.

- **R4-FLOORS-REC3. One movement rule, one source of numbers: the bake reads the soldier's movement data, and baked
  links are tied to it.**
  The bake's agent numbers (radius, step-up, jump, gravity, slope limit, height) come from the same tuning table the
  sim's movement reads, and the map document records a hash of those values with the baked data; when the hash no
  longer matches, the bake is stale.
  The reason: legacy bots and players differ (only the player stands on covers, GW14), which v2 cannot afford because
  humans take over bot slots; tuning is data and can change (R1-DATA-REC5's hot reload), and a link checked against old
  jump values is a lie.
  Rests on: legacy GW14; R1-TICK-REC6, R1-DATA-REC5 (hot reload), R1-DATA-OQ7 (what a reload may change).
  Serves: VISION_V2 §6.2 and §6.3 (tuning as data without code changes).
  A spike must prove (proposed): changing `jumpV` in the tuning data without re-baking makes CI and the editor's
  validation panel report the map's navigation as stale (1 of 1 cases); after the re-bake the hash matches.

### 4. Line of sight across heights (R4-LOS)

- **R4-LOS-REC1. One height-aware sight query in the sim, used by bots, blasts, melee, the shield check and the shared
  visibility pass: a segment from an eye point to sample points on the target, tested with early exit against the
  static world and, by data flag, against movers.**
  Eye height comes from the unit's stance data (legacy +1.45 m); the target gives sample points along its height (three
  for a soldier, as Quake III does: bottom, middle, top; per part for a vehicle). The segment is tested any-hit, stopping
  at the first blocker. Each collider carries data flags for what it blocks (movement, shots, sight), as legacy already
  separates "solid" from "blocks sight" (GW16); movers block sight only where their type's data says so (OQ1). The end
  margin (legacy 0.5 m) is data. Blasts test from the blast centre to the target's points; melee from the attacker's
  chest. A ray that starts inside a shape follows one explicit rule (F4; legacy CS4's inside-start miss).
  The reason: VISION_V2 §6.2 asks for line of sight across heights; with 3D points, slopes for ramps (R4-PHYS-REC2) and
  floors kept apart (R4-FLOORS-REC2), a plain segment test is height-aware. Quake III's shipped bots use exactly this:
  eye to three box points, any clear trace counts (F1). parry's controlled traversal allows the early exit a sight test
  needs (F3). Legacy uses three different worlds for sight, shots and blasts (CS9, GN7, GW15) and none for melee
  (GC12); one query removes those disagreements.
  Rests on: F1, F3, F4; R4-PHYS-F12, R4-PHYS-REC2, R4-FLOORS-REC2; R2-INT-REC3, R2-INT-OQ5; legacy CS4, CS6, CS9,
  GN4, GN7, GW15, GW16, GC12.
  Serves: VISION_V2 §6.2 (line of sight across heights) and §6.3 (blasts and melee abilities decided by one world).
  A spike must prove (proposed pass marks):
  1. **Port parity.** On box-only scenes, 20,000 random eye-target pairs give the same visible or blocked answer as
     legacy `hasLOS` (CS6's fuzz method), with the margin set to 0.5 m.
  2. **Across floors.** Between two stacked floors of a test building, 0 of 10,000 random pairs see through the floor
     or the ceiling; through an open stairwell or window, pairs with a clear segment are seen in 100 %.
  3. **Ramps and ledges.** On the Octopon-3 greybox, 0 of 10,000 pairs blocked by a ramp's bounding box but clear of
     its slope (the legacy disagreement, GN7).
  4. **Cost, answering R2-INT-OQ5.** Mean and p99 cost of one segment query on the greybox, native and in a browser
     Worker, with method; and R2-INT-REC3's full pass (24v24 and 16v16v16, three points) inside its 10 % budget.
  5. **Determinism.** The pass's results hash identically on native and Wasm per tick.

- **R4-LOS-REC2. Cull before casting, and budget sight in rays per tick: distance and team first, then area
  connectivity, then (if bots get one) field of view, then segments.**
  The order follows Quake III (distance, then field of view, then traces, F1, F2) with R2-INT-REC2's O(1) area test in
  front of the rays. Bots read the shared pass's results for acquisition (R2-INT-REC3) and re-check only their current
  target more often (legacy 0.1 s). The budget counts rays per tick with carried remainders and a rotating start, as
  legacy's per-second budget does (GN4).
  The reason: the pair count bounds the work (R2-INT-F20); legacy at 48 soldiers can only afford about 2.5 scans per
  bot per second against 4 at 12 (GN4); culls that cost almost nothing per pair (distance, team, area flood numbers)
  cut the average case, but not the worst case where everyone fights over the centre (R2-INT-F19 makes the same point
  for interest), which is why the marks below test that worst case.
  Rests on: F1, F2; R2-INT-F19, R2-INT-F20, R2-INT-REC2, R2-INT-REC3; legacy GN4.
  Serves: VISION_V2 §6.2 and §6.1 (48+ players with bots).
  A spike must prove (proposed): at 24v24 and 16v16v16 with everyone in one fight on the greybox, every bot scans at
  least 4 times per second (legacy's rate at 12 soldiers, GN4) within R2-INT-REC3's 10 % budget; the share of pairs
  culled before a ray and the rays per tick are reported.

- **R4-LOS-REC3. Aim at height through the same queries on both sides: the client's cursor ray and the server's shot
  use the same static world built from the same map document, and the same functions.**
  The reason: R2-SYNC-REC8 sends a world aim point picked on the client and asks R4 for the server's height-aware query;
  if the client and the server build their worlds from the same document with the same code (R4-PHYS-REC2), the point
  the player clicked is the point the server tests. Legacy's ramp boxes put the aim point 1.25 m above the slope (GC3).
  Rests on: R4-PHYS-REC2; R2-SYNC-REC8; legacy GC3, GN7.
  Serves: VISION_V2 §6.2 (aiming at height).
  A spike must prove (proposed): R2-SYNC-REC8's mark 1; and a downward cursor ray onto a ramp lands within 1 cm of the
  slope in 1,000 of 1,000 samples (legacy: 1.25 m above it).

### 5. Obstacles placed mid-match (for later building) (R4-OBST)

- **R4-OBST-REC1. Moving vehicles are never carved into the navigation data: they are obstacles in the sim's avoidance
  step, by their hull footprint; a vehicle that stops or moves slowly also marks the nodes under it with a temporary
  cost, refreshed only when it has moved past a threshold.**
  A bot whose route crosses newly marked nodes replans through R4-NAV-REC2's queue; other bots do not. Wrecks and idle
  vehicles keep their marks until they move or are removed.
  The reason: carving a moving obstacle means re-queuing tile rebuilds every tick it moves (F1, F2) and running
  generation inside the sim on both targets on the same tick, which an asynchronous rebuild cannot promise (R4-NAV-F10)
  and which undoes the bake-as-data design (R4-NAV-REC1). The libraries read split the problem the same way: crowd
  agents avoid moving characters rather than carving them (R4-NAV-F13; F3's ORCA agents and obstacles). Marking only on
  a threshold, and replanning only crossed routes, avoids the replan storm (GN13; LOOP_V2 §10). Legacy bots ignore the
  Juggernaut entirely (GC17).
  Rests on: F1, F2, F3; R4-NAV-F10, R4-NAV-F13, R4-NAV-REC1, R4-NAV-REC2, R4-NAV-REC3; R4-PHYS-REC4 (the stress
  scene); legacy GC17, GN13; LOOP_V2 §10.
  Serves: VISION_V2 §6.4 (big vehicles as moving obstacles) and §6.1 (bots in every mode).
  A spike must prove (proposed pass marks): R4-PHYS-REC4's stress scene (48 bots, 8 big vehicles including harvesters)
  for 10 minutes at 30 and 60 Hz: 0 bots stuck longer than 3 s against a vehicle; at most 2 replans per bot per second
  at p99 (R4-NAV-REC2's mark), with vehicles driving through the bots' routes; the marking and avoidance work for
  vehicles reported as a share of the tick; per-tick hashes identical on native and Wasm.

- **R4-OBST-REC2. Structures that change state are baked in both states; a change flips flags on the affected nodes,
  links and portals in the tick it happens, together with R2-INT-REC2's portal re-flood. No navigation is generated at
  runtime.**
  A destructible wall, a gate or a building footprint carries its alternate state in the bake (blocked while alive,
  open when destroyed, or a ruin's shape); a rebuild flips it back. The same event updates sight (the alive flag of
  R4-PHYS-REC2) and interest connectivity (R2-INT-REC2), so routes, sight and relevance change in the same tick.
  The reason: Quake III marks links "temporary not possible" (F4) and flips area connectivity by re-flooding when a door
  changes (R2-INT-F4) rather than rebuilding its graph; Detour's query filter and polyanya's switchable layers do the
  same job on a mesh (R4-NAV-F5, R4-NAV-F11). R1-DATA-OQ7 then holds: geometry and navigation data never change
  mid-match, only flags do.
  Rests on: F4; R4-NAV-F5, R4-NAV-F11; R2-INT-F4, R2-INT-REC2; R1-DATA-OQ7; R4-PHYS-REC2; legacy GN11, GW15, GW20.
  Serves: VISION_V2 §6.2 (floors and interiors whose openings change) and §6.5 (objectives destroyed and rebuilt,
  team-funded rebuilds).
  A spike must prove (proposed): destroying a test wall opens its route, its sight lines and its portal in the same
  tick (0 ticks of disagreement over 1,000 destroy and rebuild cycles); every route found after a flip is walked by
  the sim's movement without failure in 1,000 traversals; a flip costs ≤ 0.1 ms native (proposed).

- **R4-OBST-REC3. The bake's output is split into fixed tiles with stable ids, so the editor re-bakes only what an
  edit touches; the sim never re-bakes during a match. Player building is not designed here, and nothing in the format
  rules it out.**
  The reason: EDITOR_V2 §3's Test plays the map with bots at once, so the bake should redo only edited tiles (Detour's
  mesh is tile-based already, R4-NAV-F3; oxidized_navigation regenerates per tile, R4-NAV-F10). If Mustafa ever adds
  player building, re-baking touched tiles in the sim (the tile-cache approach, F1) is the path the tiles keep open;
  its determinism and cost would be that feature's questions (OQ3).
  Rests on: F1; R4-NAV-F3, R4-NAV-F10; EDITOR_V2 §3.
  Serves: EDITOR_V2 §3 (Test; validation panel), through VISION_V2 §6.2 (maps with verticality made in the editor).
  A spike must prove (proposed): moving one block re-bakes only the tiles it overlaps (count reported), and the result
  is byte-identical to a full bake of the edited map.

## Open questions

### 1. Rapier 3D or custom (R4-PHYS)

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

### 2. Navigation meshes in Rust (R4-NAV)

- **R4-NAV-OQ1. Which generator.** rerecast's mesh or our layered grid? Decided by REC1 marks 1 to 6.
- **R4-NAV-OQ2. rerecast's maturity and browser build.** Its C++ comparison test exists (F9) but its results, the
  Recast version it ports and a Wasm build are not stated in the pages read. Decided by the spike (and by reading the
  test, which is code: not run in phase 0).
- **R4-NAV-OQ3. Distance tables or flow fields for crowds.** The shipped sources on flow fields (the Age of Empires IV
  pathing talk, a Game AI Pro chapter) were not read (video only, or outside the allow list). Whether per-goal tables
  (F19) are enough at 48-64 bots, or a finer field is needed, is decided by REC2's spike.
- **R4-NAV-OQ4. If polyanya is used anyway.** Its hashbrown dependency (F15) needs an audit of every map it iterates in
  path-deciding code. Decided by a code review (rust-reviewer) before adoption.
- **R4-NAV-OQ5. Do soldiers block each other?** REC3's marks depend on it (R4-PHYS-OQ1).

### 3. Bots moving between floors (R4-FLOORS)

- **R4-FLOORS-OQ1. Crouching and other stances.** Quake III areas record a presence type (F7). Legacy crouch is dead
  code (CS2: stance is always 0). If v2 adds crouch-only passages, nodes need a stance. Decided by game design.
- **R4-FLOORS-OQ2. Vehicles between levels.** Do bots drive vehicles, and can a Juggernaut climb ramps? A vehicle needs
  its own bake (radius, step, no jumps). The format keys bakes by agent class so this is not designed out. Decided by
  game design (LOOP_V2 §10 lists no vehicle-driving bots).
- **R4-FLOORS-OQ3. Fall damage and drop height.** Legacy has no fall damage; if v2 adds it, drop-down links need a
  maximum drop. Decided by game design.
- **R4-FLOORS-OQ4. Movement tuning hot-reloaded mid-match.** REC3 marks the bake stale; whether a development reload
  re-bakes on the spot or waits for the next match is R1-DATA-OQ7's call with REC1's bake time.

### 4. Line of sight across heights (R4-LOS)

- **R4-LOS-OQ1. Do soldiers and vehicles block sight and shots?** Legacy: soldiers never block (CS9), the Juggernaut is
  not in the hit world (GC17). Big vehicles as cover fits §6.4 and R2-SYNC-OQ8 (rewound or present). REC1 supports any
  answer by data flag. Decided by Mustafa with the game-designer.
- **R4-LOS-OQ2. What a player may see.** Whether the shared pass hides enemies or only serves bots is R2-INT-OQ1 (look
  brief and Mustafa).
- **R4-LOS-OQ3. A field of view for bots.** Quake III's bots have one (F1); legacy bots see all around. Decided by game
  design, after the stall and reaction numbers of REC2's spike.
- **R4-LOS-OQ4. Partial blockers.** Hedges, fences and smoke could block sight but not shots, or give a fraction as
  Quake III's fog does (F1). REC1's per-collider flags cover the yes/no case; fractions are a design question.
- **R4-LOS-OQ5. The cost of a segment query** is not in any source read (F5). Decided by REC1 mark 4.

### 5. Obstacles placed mid-match (for later building) (R4-OBST)

- **R4-OBST-OQ1. What a vehicle does to soldiers in its path** (block, push, crush): R4-PHYS-OQ1. REC1's avoidance
  costs depend on it.
- **R4-OBST-OQ2. Do harvesters follow fixed lanes or navigation?** Legacy uses lanes (`HARV.lanes`). Lanes keep them
  out of the bots' navigation work, but a vehicle on a lane is still an obstacle (REC1). Decided by game design with
  LOOP_V2 §3.
- **R4-OBST-OQ3. Player building, if ever.** Runtime re-bake of touched tiles inside the sim, bit-identical on both
  targets and on the same tick: no source read measures or promises it (R4-NAV-F16). Decided only if Mustafa adds
  building to VISION_V2 §6.
- **R4-OBST-OQ4. ORCA against non-circular hulls.** dodgy_2d takes agents and obstacle polygons (F3); whether a moving
  oriented hull is best an obstacle polygon updated per tick or several circles is not settled by the pages read.
  Decided by REC1's spike.

### Wave-1 items R4 changes or answers (said openly, not silently)

These follow from R4's recommendations; where they change a wave-1 recommendation, the ADR-0004 synthesis decides,
with R4-PHYS-REC1's spike as the evidence.
- R1-ECS-REC4 and R1-ECS-REC1 mark 2 assumed Rapier's sets in the snapshot. Under R4-PHYS-REC1 (option C) there are
  none; if the spike picks option B instead, the static world stays outside the snapshot (R4-PHYS-REC2), and R1's
  marks apply only to what Rapier holds.
- R1-DET-REC3's flag moves to parry3d's own `enhanced-determinism` (R1-DET-F23); its spike scene stands, with the
  vehicle count it asked R4 for: 8 big vehicles, harvesters included (R4-PHYS-REC4).
- R1-TICK-OQ9 answered: no physics substeps for kinematic, swept vehicles at 30 Hz (R4-PHYS-REC4).
- R1-DET-OQ3 answered with a bound: f32, up to a 2 km map (R4-PHYS-REC5).
- R1-DATA-OQ7 answered for navigation: navigation data and geometry never change mid-match; only flags do
  (R4-NAV-REC4, R4-OBST-REC2).
- R1's cross-topic question "whether navigation runs only on the server": no, also in the browser for offline or
  local-bot play, never in prediction (R4-NAV-REC4).
- R1's cross-topic question "the spatial index inside the snapshot or rebuilt after a restore": rebuilt
  (R4-PHYS-REC2).
- R2-INT-OQ5 (the cost of a BVH ray) is measured by R4-LOS-REC1 mark 4; no source read gives it (R4-LOS-F5).

### Pointers to other topics

#### From R4-PHYS

- R1: REC1 changes R1-ECS-REC4, R1-ECS-REC1 mark 2 and R1-DET-REC3's crate (listed above); R1-TICK-OQ9 and
  R1-DET-OQ3 answered; the spatial index is rebuilt, not snapshotted.
- R2: the rewound hitscan tests movers from R2-SYNC-REC5's history arrays with our own capsule maths (CS4) and the
  static world through the static BVH (F11: Rapier's queries keep no history either way).
- R5: a counting test for the static BVH build and a parity fuzz (REC2 (a)); the native-versus-Wasm hash suite covers
  parry3d.
- game-designer: OQ1 (blocking rules).

#### From R4-NAV

- R1: answers "whether navigation runs only on the server" (REC4) and R1-DATA-OQ7's navigation part; R1-DET-REC1 (b)
  covers bots, so navigation is inside the hash test.
- R5: the reachability check in the sim crate, runnable from Node (editor) and CI (REC1 mark 4); the stall-rate
  comparison (TL13 method) in the v2 harness.
- EDITOR_V2: the bake runs on Test and save; the validation panel reads the bake's gap and reachability reports.
- rust-reviewer: OQ4 (hashing in polyanya), REC3's audit of dodgy_2d.
- game-designer: OQ5.

#### From R4-FLOORS

- R2: nodes carry R2-INT-REC2's area ids (REC2); one floor model for navigation, sight and interest.
- R1: REC3 adds a staleness hash between tuning and baked links (R1-DATA-OQ7).
- EDITOR_V2: link overrides and the stale-bake warning in the validation panel.
- game-designer: OQ1 to OQ3.

#### From R4-LOS

- R2: REC1 mark 4 answers R2-INT-OQ5; REC3 is the server query R2-SYNC-REC8 asks for; area ids shared with
  R2-INT-REC2 (R4-FLOORS-REC2).
- R1: blasts (NC-7, GC14) and melee (GC12) use REC1's query; the shield's frontal arc reads sim facing (R1 cross-topic
  note), not a render object.
- game-designer: OQ1, OQ3, OQ4.

#### From R4-OBST

- R2: REC2's flag flips and R2-INT-REC2's portal re-flood are one event; vehicles as cover in rewind (R2-SYNC-OQ8).
- R1: R1-DATA-OQ7 answered for navigation (flags only mid-match).
- EDITOR_V2: tile re-bake on edit (REC3).
- game-designer: OQ1, OQ2.

### Recommendations that hinge on a source not read

| Recommendation | Source | Reason not read |
|---|---|---|
| R4-NAV-REC2 (per-goal distance tables rather than flow fields) | https://gdcvault.com/play/1028036/Pathing-in-Age-of-Empires (also /1027659) | GDC Vault redirects to a maintenance page outside the allow list; the talks are video only. R4-NAV-OQ3 says what decides it. |
| R4-NAV-REC2 | https://www.taylorfrancis.com/books/9780429055096/chapters/10.1201/9780429055096-8 (a Game AI Pro chapter on flow-field tiles, search lead) | outside the allow list |
| R4-NAV-REC1 (Source's navigation meshes for bots as a second shipped precedent) | https://developer.valvesoftware.com/wiki/Navigation_Meshes | the site answers HTTP 403 to the fetcher (known; not retried). REC1 rests on Quake III's botlib instead. |
| R4-PHYS-REC1 (mark 4: Wasm size) | https://learnwithhasan.com/js-libraries/rapier/ (a tutorial claiming "~1.1 MB" of Wasm, search lead) | outside the allow list; REC1 does not use the figure: the size is measured (R4-PHYS-F27) |

## Verifier spot-check

Pending: the wave 2 verifier spot-check.

## Sources

Every source in the read list was read on 2026-10-28 with WebFetch (none with a shell command). Counts: 46 read (36
official, 8 shipped game, 2 other); 32 not read.

### Sources read (46)

#### Official (36)
| # | URL | Title | Version or page date |
|---|---|---|---|
| 1 | https://rapier.rs/docs/user_guides/rust/character_controller | Rapier user guide: character controller (also the vehicle and PID controllers) | guide "Rust (0.36)"; no date |
| 2 | https://docs.rs/rapier3d/latest/rapier3d/control/struct.KinematicCharacterController.html | `KinematicCharacterController` | rapier3d 0.36.1 |
| 3 | https://docs.rs/rapier3d/latest/rapier3d/pipeline/struct.QueryPipeline.html | `QueryPipeline` | rapier3d 0.36.1 |
| 4 | https://rapier.rs/docs/user_guides/rust/scene_queries | Rapier user guide: scene queries | Rust 0.36; no date |
| 5 | https://rapier.rs/docs/user_guides/rust/rigid_body_ccd | Rapier user guide: CCD | Rust 0.36; no date |
| 6 | https://rapier.rs/docs/user_guides/rust/rigid_bodies | Rapier user guide: rigid bodies | Rust 0.36; no date |
| 7 | https://rapier.rs/docs/user_guides/rust/colliders | Rapier user guide: colliders | Rust 0.36; no date |
| 8 | https://rapier.rs/docs/user_guides/rust/getting_started | Rapier user guide: getting started | Rust 0.36; no date |
| 9 | https://raw.githubusercontent.com/dimforge/rapier/master/CHANGELOG.md | Rapier changelog | master; head v0.36.1 (2026-10-04) |
| 10 | https://docs.rs/crate/rapier3d/latest/features | rapier3d feature flags | rapier3d 0.36.1 |
| 11 | https://dimforge.com/blog | Dimforge blog index | as of 2026-10-28 |
| 12 | https://dimforge.com/blog/2026/10/25/dimforge-Q3-technical-report | Dimforge Q3 2026 technical report | 2026-10-25 |
| 13 | https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge | The Rapier physics engine 2025 review and 2026 goals | 2026-01-09 |
| 14 | https://docs.rs/parry3d/latest/parry3d/ | parry3d crate root | parry3d 0.31.1 |
| 15 | https://docs.rs/parry3d/latest/parry3d/partitioning/index.html | parry3d `partitioning` | parry3d 0.31.1 |
| 16 | https://docs.rs/parry3d/latest/parry3d/partitioning/struct.Bvh.html | parry3d `Bvh` (first 100,000 of 117,438 characters) | parry3d 0.31.1 |
| 17 | https://docs.rs/parry3d/latest/parry3d/query/index.html | parry3d `query` | parry3d 0.31.1 |
| 18 | https://docs.rs/avian3d/latest/avian3d/ | avian3d crate root | avian3d 0.8.0 |
| 19 | https://doc.rust-lang.org/std/primitive.f32.html | std `f32` | Rust 1.99.1 (2026-10-13) |
| 20 | https://github.com/recastnavigation/recastnavigation | Recast & Detour README | main; no release shown |
| 21 | https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Recast/Include/Recast.h | Recast.h | main |
| 22 | https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Detour/Include/DetourNavMesh.h | DetourNavMesh.h | main |
| 23 | https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Detour/Include/DetourNavMeshQuery.h | DetourNavMeshQuery.h | main |
| 24 | https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/DetourCrowd/Include/DetourCrowd.h | DetourCrowd.h | main |
| 25 | https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/DetourTileCache/Include/DetourTileCache.h | DetourTileCache.h | main |
| 26 | https://docs.rs/recastnavigation-sys/latest/recastnavigation_sys/ | recastnavigation-sys | 1.0.3 (2026-06-05) |
| 27 | https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html | rustc book: `wasm32-unknown-unknown` | current |
| 28 | https://docs.rs/rerecast/latest/rerecast/ | rerecast | 0.5.0 |
| 29 | https://docs.rs/oxidized_navigation/latest/oxidized_navigation/ | oxidized_navigation | 0.12.0 |
| 30 | https://docs.rs/polyanya/latest/polyanya/ | polyanya | 0.18.0 |
| 31 | https://docs.rs/landmass/latest/landmass/ | landmass | 0.9.3 |
| 32 | https://docs.rs/pathfinding/latest/pathfinding/ | pathfinding | 4.16.0 |
| 33 | https://docs.rs/pathfinding/latest/pathfinding/directed/astar/fn.astar.html | pathfinding `astar` | 4.16.0 |
| 34 | https://docs.rs/hashbrown/latest/hashbrown/ | hashbrown | 0.17.1 (2026-09-24) |
| 35 | https://docs.rs/foldhash/latest/foldhash/ | foldhash | 0.2.0 |
| 36 | https://docs.rs/dodgy_2d/latest/dodgy_2d/ | dodgy_2d | 0.5.5 (2026-10-08) |

#### Shipped game (8): Quake III Arena, id Software, GPL source release (copyright 1999-2005), master branch
| # | URL | Title |
|---|---|---|
| 37 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_slidemove.c | player slide-and-step movement |
| 38 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_local.h | movement constants |
| 39 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c | shared player movement |
| 40 | https://github.com/id-Software/Quake-III-Arena/tree/master/code/botlib | botlib directory listing (to locate the files) |
| 41 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/aasfile.h | AAS file format: travel types, reachabilities, clusters, portals |
| 42 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/be_aas_reach.c | reachability calculations (first 100,000 of 152,877 characters) |
| 43 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/be_aas_route.c | routing caches |
| 44 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/ai_dmq3.c | bot AI: `BotEntityVisible`, `BotFindEnemy` (first 100,000 of 158,046 characters, read twice) |

#### Other (2)
| # | URL | Title |
|---|---|---|
| 45 | https://github.com/andriyDev/recastnavigation-rs-sys | recastnavigation-sys README (the binding author's statements) |
| 46 | https://github.com/janhohenheim/rerecast | rerecast README (the port author's claims) |

### Sources not read (32)
| # | URL | Reason |
|---|---|---|
| 1 | https://learnwithhasan.com/js-libraries/rapier/ | search lead (a Rapier Wasm size claim); outside the allow list |
| 2 | https://app.unpkg.com/@dimforge/rapier3d@0.17.3/files/README.md | search lead; outside the allow list |
| 3 | https://classic.yarnpkg.com/en/package/@dimforge/rapier3d | search lead; outside the allow list |
| 4 | https://classic.yarnpkg.com/en/package/@dimforge/rapier2d | search lead; outside the allow list |
| 5 | https://git.nea.moe/github/rapier.git/log/CHANGELOG.md?showmsg=1 (and the mirror's other pages) | search lead, a mirror of the changelog; outside the allow list (the official changelog was read) |
| 6 | https://www.abratabia.com/game-physics/rapier.php | search lead; outside the allow list |
| 7 | https://warp.dev/blog/reducing-wasm-binary-size | search lead (general Wasm size advice); outside the allow list |
| 8 | https://book.leptos.dev/deployment/binary_size.html | search lead (general Wasm size advice); outside the allow list |
| 9 | https://docs.rs/crate/rapier2d/0.35.0-beta.0 | allowed, not fetched: an older beta of the 2D crate; the current 3D crate's pages were read |
| 10 | https://rapier.rs/docs/ | allowed, not fetched: the docs index; the specific guide pages were read |
| 11 | https://gdcvault.com/play/1028036/Pathing-in-Age-of-Empires | GDC Vault: maintenance redirect outside the allow list; video only |
| 12 | https://gdcvault.com/play/1027659/Pathing-in-Age-of-Empires | as above |
| 13 | https://www.gdcvault.com/play/1027659 | as above |
| 14 | https://www.taylorfrancis.com/books/9780429055096/chapters/10.1201/9780429055096-8 | search lead (Game AI Pro chapter); outside the allow list |
| 15 | https://gamedev.net/forums/topic/701267-flow-field-architecture-advice | search lead; outside the allow list |
| 16 | https://gamedev.net/forums/topic/701267-flow-field-architecture-advice/5401826/ | search lead; outside the allow list |
| 17 | https://www.gamedev.net/forums/topic/563819-whats-starcraft2s-pathfinding-tech/ | search lead; outside the allow list |
| 18 | https://doc.photonengine.com/quantum/current/addons/flow-fields/overview | search lead; outside the allow list |
| 19 | https://doc.photonengine.com/quantum/v2/addons/flow-fields/overview | search lead; outside the allow list |
| 20 | https://doc.photonengine.com/quantum/v3/addons/flow-fields/overview | search lead; outside the allow list |
| 21 | https://www.lib.rs/crates/recastnavigation-sys | search lead; outside the allow list (the docs.rs page was read) |
| 22 | https://lib.rs/crates/divert | search lead; outside the allow list |
| 23 | https://rust-digger.code-maven.com/crates/detour-sys | search lead; outside the allow list |
| 24 | https://rust-digger.code-maven.com/crates/divert | search lead; outside the allow list |
| 25 | https://licenses.dev/cargo/recast-detour-rs/0.2.0 | search lead; outside the allow list |
| 26 | https://crates-io.ganymede-static.progval.net/crates/divert/0.3.0 | search lead; outside the allow list |
| 27 | https://crates-io.ganymede-static.progval.net/crates/divert/0.4.0 | search lead; outside the allow list |
| 28 | https://recast-rs.readthedocs.io/en/stable/print.html | search lead (a Rust port for a WoW emulator); outside the allow list |
| 29 | https://docs.rs/recast-detour-rs | allowed, not fetched: a Detour-only wrapper over recastnavigation-sys (read) and Detour (read) |
| 30 | https://developer.valvesoftware.com/wiki/Navigation_Meshes | the site answers HTTP 403 to the fetcher (known; not retried) |
| 31 | https://docs.rs/hashbrown/latest/hashbrown/hash_map/type.DefaultHashBuilder.html | HTTP 404 (the crate root was read instead) |
| 32 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/be_aas.h | HTTP 404 (not in that directory; aasfile.h holds the travel types) |
