# R4-OBST draft: obstacles placed mid-match (engine-architect, 2026-10-28)

Scope (task rule 11): the need is VISION_V2 §6.4, big vehicles as moving obstacles, plus structures that change state
(destroyed, rebuilt). Player building is only "don't design it out": nothing here designs a building system.

All sources read 2026-10-28. Tier: official / shipped game / other.

## Findings
- **R4-OBST-F1.** Detour's tile cache takes temporary obstacles as cylinders, axis-aligned boxes or boxes rotated about
  the vertical axis, and applies them by rebuilding the tiles they touch. Quotes: "DT_OBSTACLE_CYLINDER,";
  "DT_OBSTACLE_BOX, // AABB"; "DT_OBSTACLE_ORIENTED_BOX // OBB"; "// Box obstacle: can be rotated in Y."; `update`:
  "Updates the tile cache by rebuilding tiles touched by unfinished obstacle requests." Source:
  https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/DetourTileCache/Include/DetourTileCache.h
  (main). Tier: official. Sure: confirmed in the source.
- **R4-OBST-F2.** The tile cache works through a bounded queue and may need several updates to catch up. Quotes:
  "static const int MAX_REQUESTS = 64;"; "static const int MAX_UPDATE = 64;"; `upToDate` "Whether the tile cache is fully
  up to date with obstacle requests and tile rebuilds."; obstacle states "DT_OBSTACLE_PROCESSING," "DT_OBSTACLE_PROCESSED,"
  "DT_OBSTACLE_REMOVING". Source: the same header. Tier: official. Sure: confirmed in the source; that a moving
  obstacle would keep re-queuing rebuilds is inferred.
- **R4-OBST-F3.** dodgy_2d computes avoiding velocities with ORCA against both agents and static obstacle polygons.
  Quotes: "An implementation of ORCA, a local collision avoidance algorithm for 2D."; "essentially a port of RVO2 to
  Rust,"; `Agent` "A single agent in the simulation."; `Obstacle` "A single obstacle in the simulation." Source:
  https://docs.rs/dodgy_2d/latest/dodgy_2d/ (dodgy_2d 0.5.5; docs page dated 2026-10-08). Tier: official. Sure:
  confirmed in the source; the inputs of `compute_avoiding_velocity` (neighbours, obstacles, preferred velocity, maximum
  speed, time step, time horizons) are from the fetch summary of the example (inferred); it works in f32 and makes no
  determinism statement (inferred).
- **R4-OBST-F4.** Quake III can mark a link as temporarily impossible instead of rebuilding its area graph. Quote:
  "#define TRAVEL_INVALID 1 //temporary not possible". Source:
  https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/aasfile.h. Tier: shipped game. Sure:
  confirmed in the source; how and when the game sets it is not in the file read (inferred use).
- (Used, read under other questions: R4-NAV-F5 (Detour's poly flags and area costs filter queries), R4-NAV-F10
  (oxidized_navigation regenerates tiles asynchronously on collider changes), R4-NAV-F11 (polyanya layers that can be
  enabled or disabled), R4-NAV-F13 (landmass: characters are "only as obstacles to be avoided by agents"), R4-NAV-F17
  (Detour's crowd moves its own agents).)

## Legacy facts used
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
  Rests on: F1, F2, F3; R4-NAV-F10, R4-NAV-F13, R4-NAV-REC1, R4-NAV-REC2, R4-NAV-REC3; legacy GC17, GN13.
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

## Sources read
- https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/DetourTileCache/Include/DetourTileCache.h · main · official
- https://docs.rs/dodgy_2d/latest/dodgy_2d/ · dodgy_2d 0.5.5 (2026-10-08) · official
- https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/aasfile.h · GPL release · shipped game (read under FLOORS)
- (also used: the NAV sources for R4-NAV-F3, F5, F10, F11, F13, F17)

## Sources not read
- none specific to this question.

## Cross-topic notes
- R2: REC2's flag flips and R2-INT-REC2's portal re-flood are one event; vehicles as cover in rewind (R2-SYNC-OQ8).
- R1: R1-DATA-OQ7 answered for navigation (flags only mid-match).
- EDITOR_V2: tile re-bake on edit (REC3).
- game-designer: OQ1, OQ2.
