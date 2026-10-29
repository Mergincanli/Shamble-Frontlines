# R4 verifier spot-check: the report, verbatim

Workflow run wf_d3f13cf7-77e (script `v2-research-r4-check-fix-run1.js`, prompt `prompts/verifier-r4.txt`), returned 2026-10-28 (about 20:46 UTC; copied from the workflow's own result by the session). Notes: `verifier-r4.md`.

VERDICT: PASS WITH NOTES. I checked 49 of the 51 key claims: 46 are confirmed, 2 are wrong and 1 is unsupported. No recommendation is overturned, but five points need content edits, so the findings are slightly more than nits.

This was a research check, so I skipped the gate and the symbol checks as instructed. I used WebFetch only, on allow-listed domains. My notes, with the full table and DONE at the end, are in /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r4.md.

**Spot-check table** (id · claim · source · verdict · location and exact words)
| id | claim | source | verdict | location, words |
|---|---|---|---|---|
| PHYS-F24 | Quake III movement is kinematic: 5 planes, 4 bumps, STEPSIZE 18, MIN_WALK_NORMAL 0.7 | bg_local.h, bg_slidemove.c | confirmed | "#define MAX_CLIP_PLANES 5"; "numbumps = 4;"; "#define STEPSIZE 18"; "#define MIN_WALK_NORMAL 0.7f // can't walk on very steep slopes" |
| PHYS-F23 | Quake III movement is shared by server and client | bg_pmove.c | confirmed | "// bg_pmove.c -- both games player movement code"; "Can be called by either the server or the client" |
| PHYS-F15 | 12 releases from 0.30.1 to 0.36.1; 0.36.0 breaks snapshots | Rapier CHANGELOG | confirmed (the 12 counts both ends and one beta) | "## v0.36.1 (4 October 2026)" … "## v0.30.1 (17 Oct. 2025)"; "Snapshots serialized with previous versions can't be loaded anymore." |
| PHYS-F6 | snap-to-ground changed in 0.35.0 | CHANGELOG, v0.35.0, "Fixed" | confirmed | "The character controller's snap-to-ground now triggers on any movement that isn't upwards," |
| PHYS-F1 | the character controller is only ray and shape casts | rapier.rs character_controller | confirmed | "is a higher-level tool that will emit the proper ray-casts and shape-casts to adjust the user-defined trajectory" |
| PHYS-F3 | the controller holds no per-character state; move_shape takes dt: f32 | guide, "Setup and usage"; docs.rs 0.36.1 | confirmed | "It does not contain any collider-specific or rigid-body-specific information like handles, velocities, positions, etc."; "pub fn move_shape( &self, dt: f32, …" |
| PHYS-F2 | autostep is off by default; snap, slope and offset | docs.rs fields, guide | confirmed (one caveat left out, see critical read 10) | "Should the character automatically step over small obstacles? (disabled by default)" |
| PHYS-F4 | gravity is the caller's job | guide, Gravity | confirmed | "it is up to you to emulate gravity by adding a downward component to that movement vector." |
| PHYS-F8 | the vehicle controller is a port of Bullet's btRaycastVehicle | guide, Vehicle controller | confirmed | "can be difficult to control for games requiring non-realistic vehicles."; "ported from the btRaycastVehicle of Bullet"; "The chassis is created like any other dynamic rigid-body," |
| PHYS-F7 | kinematic bodies go through walls | rapier.rs rigid_bodies, "Rigid-body type" | confirmed | "kinematic bodies will simply ignore any contact force and go through walls and the ground." |
| PHYS-F9 | CCD is only for dynamic bodies; "bullet" CCD is off by default | rapier.rs rigid_body_ccd | confirmed | "Every fast-moving dynamic rigid-body is swept against the fixed colliders and the soft-bodies of the scene."; "This is more expensive, therefore it is disabled by default" |
| PHYS-F21 | the BVH is built once for static scenes; about 64 bytes per node pair | docs.rs parry3d Bvh | confirmed | "For scenes where objects don’t move, build the BVH once and query repeatedly:"; "~64 bytes per pair of children (3D f32 SIMD), O(n) total"; "O(n log n) where n is the number of leaves" |
| PHYS-F20 | swept, overlap and ray tests between "any two" of parry's shapes | docs.rs parry3d query, fn.cast_shapes | **wrong** (overstated) | "-> Result<Option<ShapeCastHit>, Unsupported>"; "`Err(Unsupported)` - This shape pair is not supported" |
| PHYS-F26 | f32 is binary32 with 24 significant bits; EPSILON | doc.rust-lang.org f32 (std 1.99.1) | confirmed (the spacing arithmetic also rechecked) | "the “binary32” type defined in IEEE 754-2008"; "MANTISSA_DIGITS: u32 = 24"; "EPSILON: f32 = 1.19209290e-07_f32" |
| PHYS-F17 | rapier3d defaults to f32, 3D and std; listed flags; none for Wasm | docs.rs rapier3d features 0.36.1 | confirmed | default "`dim3`, `f32`, `std`"; "`enhanced-determinism`", "`serde-serialize`", "`parallel`", "`simd8`", "`fem`" |
| PHYS-F22 | Parry 0.26 shipped with Rapier 0.32 (January 2026) | dimforge.com 2025 review, CHANGELOG | confirmed (the file says inferred; it can be upgraded) | "Starting with rapier 0.32 and parry 0.26, most of their API and internals using the nalgebra linear algebra library"; "## v0.32.0 (09 Jan. 2026)" |
| PHYS-F12 | query filters; 32 collision groups | rapier.rs scene_queries, colliders | confirmed | "is the handle of one collider the query must ignore."; "there is a total of 32 groups." |
| PHYS-F19 | parry is geometry only, with no dynamics module | docs.rs parry3d root | confirmed | "parry is a 2 and 3-dimensional geometric library written with the rust programming language."; the module list has no dynamics module |
| PHYS-F25 | Avian is built for Bevy | docs.rs avian3d 0.8.0 | confirmed | "Avian is an ECS-driven 2D and 3D physics engine for the Bevy game engine." |
| NAV-F8 | Rust's browser target has no C/C++ toolchain | rustc book, wasm32-unknown-unknown, Requirements | confirmed | "There is no C/C++ toolchain for this target."; "While interop is theoretically possible it’s recommended to instead use one of:" |
| NAV-F4 | Detour's path query can be sliced; at most 65535 search nodes | DetourNavMeshQuery.h | confirmed | "maxIter The maximum number of iterations to perform."; "maxNodes Maximum number of search nodes. [Limits: 0 < value <= 65535]" |
| NAV-F19 | Quake III caches distances per goal, refreshes every 15 s, accepts a longer route | be_aas_route.c | confirmed | "#define CACHE_REFRESHTIME 15.0f //15 seconds refresh time"; "//NOTE: there might be a shorter route via another cluster!!! but we don't care" |
| NAV-F17 | DetourCrowd moves its own agents; 6 neighbours | DetourCrowd.h | confirmed | "static const int DT_CROWDAGENT_MAX_NEIGHBOURS = 6;"; "Updates the steering and positions of all agents." |
| NAV-F18 | precomputed area graph with integer travel times | aasfile.h | confirmed | "unsigned short int traveltime;//travel time of the inter area movement" |
| NAV-F15 | foldhash seeds at random, and hashbrown uses it by default | docs.rs foldhash 0.2.0, hashbrown 0.17.1 | **unsupported** (the hashbrown-default part) | foldhash: "always generates a random per-hasher seed"; the hashbrown page only shows "foldhash ^0.2.0 _normal_ _optional_" and never names a default hasher |
| NAV-F2 | Recast's four agent numbers | Recast.h rcConfig | confirmed (the units are voxels) | "Minimum floor to 'ceiling' height that will still allow the floor area to" / "be considered walkable. [Limit: >= 3] [Units: vx]" |
| NAV-F11 | polyanya has floor layers and Wasm demos | docs.rs polyanya 0.18.0 | confirmed | "Overlapping navmeshes for 3D navigation (floors, bridges, …)."; "WASM demos made with Bevy are available here." |
| NAV-F12 | polyanya states performance without numbers | same page | confirmed | "This crate seems to generate a few more nodes, but tends to be faster than the cpp implementation." |
| NAV-F13 | landmass manages its own agents | docs.rs landmass 0.9.3 | confirmed | "While agents are "managed" by the archipelago, characters are only as obstacles to be avoided by agents." |
| NAV-F9 | rerecast ports Recast's generation; determinism through libm is on its roadmap | docs.rs rerecast 0.5.0, GitHub README | confirmed | "Rust port of Recast, the industry-standard navigation mesh generator"; "cross-platform determinism (use `libm` feature)" |
| NAV-F1 | Recast's build steps | recastnavigation README | confirmed | "First Recast rasterizes the input triangle meshes into voxels." |
| NAV-F3 | Detour's mesh is tiles of convex polygons | DetourNavMesh.h | confirmed | "A navigation mesh based on tiles of convex polygons." |
| NAV-F5 | polygon flags and per-area costs | DetourNavMeshQuery.h, DetourNavMesh.h | confirmed | "Cost per area type."; "static const int DT_MAX_AREAS = 64;" |
| NAV-F10 | oxidized_navigation builds tiles asynchronously | docs.rs oxidized_navigation 0.12.0 | confirmed | "and asynchronously generates tiles of navigation meshes based on NavMeshSettings" |
| FLOORS-F5 | Quake III knows "19 typed ways to move" | aasfile.h | **wrong** (the count; a nit) | "#define TRAVEL_INVALID 1 //temporary not possible" … "#define TRAVEL_FUNCBOB 19 //func bob": 18 ways to move plus an invalid marker |
| FLOORS-F6 | jump links are found by predicting the player's movement | be_aas_reach.c | confirmed | "// If the gaps exist a potential jump is predicted."; "AAS_PredictClientMovement(&move, -1, beststart, PRESENCE_NORMAL, qtrue," |
| FLOORS-F7 | each link stores start, end and type; each area a presence type | aasfile.h | confirmed | "vec3_t start; //start point of inter area movement"; "int presencetype; //how a bot can be present in this area" |
| FLOORS-F1 | span columns per cell; non-overlapping layers | Recast.h | confirmed | "The next span higher up in column."; "partitioning the heightfield in non-overlapping layers." |
| FLOORS-F2 | Detour's off-mesh connections | DetourNavMesh.h | confirmed | "An off-mesh connection is a user defined traversable connection made up to two vertices." |
| FLOORS-F3 | landmass "animation link" | docs.rs landmass | confirmed | "A link connecting two edges where an agent must perform some action (or animation) to use the link." |
| FLOORS-F4 | polyanya one-way layers | docs.rs polyanya | confirmed | "One-way layers for directional movement." |
| LOS-F1 | bots trace from the eye to the centre, bottom and top of the target; field-of-view check; result in [0, 1] | ai_dmq3.c BotEntityVisible | confirmed | "for (i = 0; i < 3; i++) {"; "if (!InFieldOfVision(viewangles, fov, entangles)) return 0;"; "if (bestvis >= 0.95) return bestvis;" |
| LOS-F2 | distance cull depends on alertness; the visibility-set test is commented out | ai_dmq3.c | confirmed | "if (squaredist > Square(900.0 + alertness * 4000.0)) continue;"; "//if (!AAS_inPVS(eye, middle)) continue;" |
| LOS-F3 | the caller controls BVH traversal | docs.rs parry3d partitioning, Bvh | confirmed | "Traverses the BVH in depth-first order with full control over traversal." |
| LOS-F4 | a "solid" ray that starts inside a shape hits at distance 0 | rapier.rs scene_queries | confirmed | "if solid is true then the hit point will be the ray origin itself" |
| OBST-F1 | tile-cache obstacle shapes; rebuilds the tiles they touch | DetourTileCache.h | confirmed | "DT_OBSTACLE_ORIENTED_BOX // OBB"; "Updates the tile cache by rebuilding tiles touched by unfinished obstacle requests." |
| OBST-F2 | bounded queues of 64 | DetourTileCache.h | confirmed | "static const int MAX_REQUESTS = 64;"; "static const int MAX_UPDATE = 64;" |
| OBST-F3 | dodgy_2d is ORCA, a port of RVO2 | docs.rs dodgy_2d 0.5.5 | confirmed | "An implementation of ORCA, a local collision avoidance algorithm for 2D."; "This crate is essentially a port of RVO2 to Rust." |
| OBST-F4 | Quake III can mark a link "temporary not possible" | aasfile.h | confirmed | "#define TRAVEL_INVALID 1 //temporary not possible" |

I also checked the basis of the two recommendations that cite no R4 finding, against the legacy inventory. These are not counted in N.
- **FLOORS-REC3 (GW14)** is confirmed. `docs/LEGACY_INVENTORY.md:202` says "only the player can stand on cover tops. v2 needs one rule for every soldier, since humans take over bot slots."
- **LOS-REC3 (GC3)** is confirmed. `docs/LEGACY_INVENTORY.md:242` says "at 2.50 m on a ramp whose slope there is 1.25 m".

**Counts:** confirmed 46, wrong 2, unsupported 1.

**Wrong or unsupported basis claims, per recommendation**
- PHYS-REC1: PHYS-F20 (wrong). PHYS-F27 is an absence and was not checked.
- PHYS-REC3: PHYS-F20 (wrong).
- PHYS-REC4: PHYS-F20 (wrong).
- NAV-REC1: NAV-F15 (unsupported). NAV-F16 is an absence and was not checked.
- FLOORS-REC1: FLOORS-F5 (wrong, a nit).
- The other 13 recommendations: none.

**Critical read, ranked**
1. **PHYS-REC1's main reason overstates parry.** It says parry gives "swept tests between any two shapes". The docs say some pairs return "`Err(Unsupported)` - This shape pair is not supported". The fix is to reword, and to add a spike item proving that every pair v2 needs is supported: capsule or cylinder against oriented box, wedge, convex shape, and heightfield or triangle mesh (PHYS-OQ4).
2. **PHYS-REC1 mark 1 tests tunnelling at the wrong speed.** It uses `moveSpeed` 6.3 × `sprintMult` 1.4286 ≈ 9.0 m/s. GP3 (`LEGACY_INVENTORY.md:313`) says `T.sprintMult` is "never read: sprint uses the per-unit value". The per-unit values reach 1.62 (`shamble-core.js:128-137`), and banzai or the bayonet charge multiply by a further 1.3 (`shamble-game.js:2756`, `:2774`). The fastest soldier moves at about 13.3 m/s, which is 0.22 m per tick at 60 Hz and 0.44 m at 30 Hz, both more than the 0.2 m test wall. The mark should use the fastest speed in the data, as REC3's mark already does.
3. **Six recommendations cite §6.1 for something it does not say.** NAV-REC1 to REC4, LOS-REC2 and OBST-REC1 cite "§6.1 (bots fill empty slots…)". §6.1 is "Modes as data". "Bots fill empty slots." is in VISION_V2 §3 (`VISION_V2.md:48`). R1 had this same mistake and corrected it in its second fix round. NAV-REC4 serves only §6.1; it should cite §3 and add §6.2 ("bots that move between levels").
4. **OBST-REC3 (tiled bake with partial re-bakes) risks being a "for later" feature.** NAV-REC1 mark 1 already requires a full bake in ≤ 1 s native and ≤ 3 s in a Worker. If that passes, the tiles only keep player building open. Tiling should be conditional on mark 1 failing or on a measured need for faster editor tests. It does not design a building system, which is compliant.
5. **NAV option A is missing an option.** The rustc page behind NAV-F8 names two targets that do link C/C++: `wasm32-unknown-emscripten` and `wasm32-wasip1`. Building the sim for one of them to keep Detour is not weighed. The TL;DR's "cannot join the browser sim" is stronger than the source ("interop is theoretically possible"). The option is ruled out only by R1-DET-F6's choice of target, and the file should say so.
6. **Three absences are labelled "confirmed" with no quote.** These are PHYS-F27, NAV-F16 and LOS-F5. This breaks the file's own rule, and R1 treated absences as not confirmable. They should read "inferred". PHYS-F27 and NAV-F16 are key claims.
7. **PHYS-REC4 does not cover turning.** It says a vehicle sweeps its hull "from the old to the new pose", but `cast_shapes` is a linear sweep (it takes velocities). F13 notes that nonlinear casts cost more. A long hull that turns needs a nonlinear cast or rotate-then-sweep. The spike's 0-penetration mark would catch it, but the reasoning should say so.
8. **NAV-REC3's avoidance is 2D.** ORCA through dodgy_2d works on one plane, so neighbours must be filtered by floor (FLOORS-REC2). Otherwise bots on different floors avoid each other.
9. **Some "Rests on" lists miss claims they use.**
   - PHYS-REC2 mark (d) uses GN11's 42.8 ms, and "queries skip dead pieces" uses F12's filter; neither is listed.
   - FLOORS-REC2 does not list LOOP_V2 §2 and §9.
   - PHYS-REC3 does not list LOOP_V2 §5 (knockback).
   - OBST-REC2 does not list LOOP_V2 §3 (rebuilds).
10. **PHYS-F2 leaves out a caveat.** The same autostep doc says "Note that autostepping is currently a very computationally expensive feature, so it is disabled by default." That matters for PHYS-REC1's fallback reasoning and its mark 5.
11. **The vehicle count is not reconciled.** R4 sets 8 big vehicles, but NAV-REC2 uses R1-TICK-REC2's P1 scene, which has "plus 4 big vehicles". The file should say whether P1 moves to 8.
12. **Nits.**
    - Recast's height, climb and radius are in voxels ("[Units: vx]"), so setting them from the soldier's movement data needs a note on rounding to the cell size.
    - "Big vehicles as moving obstacles" is the plan's wording, not §6.4's ("big and multi-part, with their own weapons and abilities"); the trace holds as an inference.
    - PHYS-F15's 12 includes both ends and one beta.
    - FLOORS-F5's count should be "18 ways to move plus TRAVEL_INVALID".

Every pass mark is covered by the file's blanket "proposed" statement, and none is unsourced. I found no look choice and no building design. Beyond item 11, nothing silently contradicts R1 or R2. The changes to wave-1 items are listed openly, and R2-INT-F20's pair counts and R2-INT-REC3's 10 % budget match `r2-netcode.md`.

**Label**
M = 51 distinct R4 findings across all "Rests on" lists. Following the R3 convention, legacy and wave-1 ids are not counted.
N = 49. PHYS-F27 and NAV-F16 were not checked, because they are absences no quote can confirm.
Label: "spot-checked (49 of 51 key claims)".

**Only nits?** No, narrowly. Items 1 to 5 need content edits by the owner: a key basis claim is wrong, a pass-mark speed is wrong, the §6 traces are wrong, the tiling scope needs a condition, and an option is missing. None of them reverses a recommendation. A narrow recheck of the fixed items is enough; a full second pass is not needed.

**Not verified**
- PHYS-F27 and NAV-F16 (absences).
- That rapier 0.36.1 pins parry3d 0.31.1; this rests on R1-DET-F23.
- The "trace.plane.normal[2] < MIN_WALK_NORMAL" line, which is not in bg_slidemove.c and is presumably in bg_pmove.c.
- The parts of Bvh.html, be_aas_reach.c and ai_dmq3.c past 100,000 characters.
- The field-of-view value in LOS-F2, which stays inferred.

**Decisions needed:** none.

**For Mustafa:** nothing.
