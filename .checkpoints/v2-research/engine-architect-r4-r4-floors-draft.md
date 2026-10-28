# R4-FLOORS draft: bots moving between floors (engine-architect, 2026-10-28)

All sources read 2026-10-28. Tier: official / shipped game / other.

## Findings
- **R4-FLOORS-F1.** Recast's heightfield keeps a column of spans per grid cell, each pointing to the next span higher
  up, so surfaces stacked at the same x and z (a floor above a floor, a roof over a room) stay separate; it can also
  split the field into non-overlapping layers. Quotes: `rcSpan::next` "The next span higher up in column";
  `rcHeightfield` "A dynamic heightfield representing obstructed space."; `rcBuildLayerRegions` "Builds region data
  for the heightfield by partitioning the heightfield in non-overlapping layers." Source:
  https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Recast/Include/Recast.h (main). Tier:
  official. Sure: confirmed in the source for the quotes; that this is how stacked floors are kept apart is inferred
  from the column layout.
- **R4-FLOORS-F2.** Detour models any non-walking move as an off-mesh connection: two user-placed endpoints with a
  radius, one-way unless flagged bidirectional, with a user id. Quotes: "An off-mesh connection is a user defined
  traversable connection made up to two vertices."; `pos` "The endpoints of the connection. [(ax, ay, az, bx, by,
  bz)]"; `DT_OFFMESH_CON_BIDIR` "A flag that indicates that an off-mesh connection can be traversed in both directions.
  (Is bidirectional.)"; `userId` "(User assigned when the navigation mesh is built.)" Source:
  https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Detour/Include/DetourNavMesh.h (main). Tier:
  official. Sure: confirmed in the source.
- **R4-FLOORS-F3.** landmass has the same idea as an "animation link". Quote: "A link connecting two edges where an
  agent must perform some action (or animation) to use the link." Source: https://docs.rs/landmass/latest/landmass/
  (landmass 0.9.3). Tier: official. Sure: confirmed in the source.
- **R4-FLOORS-F4.** polyanya adds one-way layers to its overlapping floor layers. Quote: "One-way layers for
  directional movement." (beside "Overlapping navmeshes for 3D navigation (floors, bridges, …).", R4-NAV-F11). Source:
  https://docs.rs/polyanya/latest/polyanya/ (polyanya 0.18.0). Tier: official. Sure: confirmed in the source.
- **R4-FLOORS-F5.** Quake III's bots know 19 typed ways to move between areas, among them walking, crouching, jumping
  onto a barrier, jumping, ladders, walking off a ledge, teleporters, elevators and jump pads. Quotes: "#define
  TRAVEL_WALK 2 //walking"; "TRAVEL_BARRIERJUMP 4 //jumping onto a barrier"; "TRAVEL_JUMP 5 //jumping";
  "TRAVEL_LADDER 6 //climbing a ladder"; "TRAVEL_WALKOFFLEDGE 7 //walking of a ledge"; "TRAVEL_JUMPPAD 18 //jump pad".
  Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/aasfile.h (GPL release).
  Tier: shipped game. Sure: confirmed in the source.
- **R4-FLOORS-F6.** Quake III finds jump links by predicting the player's own movement across the gap, and step and
  barrier links by comparing heights with the movement's maximum step and barrier. Quotes: "creates possible jump
  reachabilities between the areas"; "Between these two points there must be one or more gaps."; "If the gaps exist a
  potential jump is predicted."; "AAS_PredictClientMovement(&move, -1, beststart, PRESENCE_NORMAL, qtrue,"; "if
  (!AAS_HorizontalVelocityForJump(phys_jumpvel, beststart, bestend, &speed))"; "if (ground_bestdist >= 0 &&
  ground_bestdist < aassettings.phys_maxstep)"; "< aassettings.phys_maxbarrier". Source:
  https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/be_aas_reach.c (GPL release; the
  first 100,000 of 152,877 characters read). Tier: shipped game. Sure: confirmed in the source.
- **R4-FLOORS-F7.** Each Quake III link records where the move starts and ends and what kind of travel it is, and each
  area records how a bot can be present in it. Quotes: "vec3_t start; //start point of inter area movement"; "vec3_t end;
  //end point of inter area movement"; "int traveltype; //type of travel required to get to the area"; "int
  presencetype; //how a bot can be present in this area". Source: aasfile.h. Tier: shipped game. Sure: confirmed in the
  source.
- (Used, read under other questions: R4-NAV-F2 for Recast's step, height and slope limits; R4-PHYS-F2 and F24 for
  step-up in movement; R4-NAV-F18 for integer travel times.)

## Legacy facts used
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

## Options

| | A. Links placed by hand | B. Links generated by the bake, by simulating movement, plus hand overrides | C. No links: ramps and stairs only |
|---|---|---|---|
| How | The designer places every jump and drop in the editor (Detour style, F2) | Steps and ramps are walkable surfaces; the bake proposes jump-up, drop-down and gap links where heights allow, and keeps only those the sim's own movement completes (Q3 style, F6); designers add or forbid links | Bots walk ramps; ledges and drop-downs are not used |
| Correctness | A hand link can promise a jump the movement cannot make (legacy's formula vs integrated reach, GW14) | Every link is a measured move at the shipped tick (R1-TICK-REC6) | Always walkable, but routes are long |
| Editor work | High, repeated on every map change | Low; overrides only | None |
| Fit with §6.2 and LOOP_V2 §9 | Yes, at authoring cost | Yes: ledges, drop-downs, interiors | No: drop-downs "for fast rotations" are lost |
| Cost | None at bake | Bake time grows with candidate links (each simulated) | None |

## Recommendations

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
  Rests on: F2, F3, F4, F5, F6, F7; R4-NAV-F18; R1-TICK-REC6; legacy GN10, GW14, TL14.
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
  Rests on: legacy GW14; R1-TICK-REC6; R1-DATA-OQ7 (what a reload may change).
  Serves: VISION_V2 §6.2 and §6.3 (tuning as data without code changes).
  A spike must prove (proposed): changing `jumpV` in the tuning data without re-baking makes CI and the editor's
  validation panel report the map's navigation as stale (1 of 1 cases); after the re-bake the hash matches.

## Open questions
- **R4-FLOORS-OQ1. Crouching and other stances.** Quake III areas record a presence type (F7). Legacy crouch is dead
  code (CS2: stance is always 0). If v2 adds crouch-only passages, nodes need a stance. Decided by game design.
- **R4-FLOORS-OQ2. Vehicles between levels.** Do bots drive vehicles, and can a Juggernaut climb ramps? A vehicle needs
  its own bake (radius, step, no jumps). The format keys bakes by agent class so this is not designed out. Decided by
  game design (LOOP_V2 §10 lists no vehicle-driving bots).
- **R4-FLOORS-OQ3. Fall damage and drop height.** Legacy has no fall damage; if v2 adds it, drop-down links need a
  maximum drop. Decided by game design.
- **R4-FLOORS-OQ4. Movement tuning hot-reloaded mid-match.** REC3 marks the bake stale; whether a development reload
  re-bakes on the spot or waits for the next match is R1-DATA-OQ7's call with REC1's bake time.

## Sources read
- https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Recast/Include/Recast.h · main · official
- https://raw.githubusercontent.com/recastnavigation/recastnavigation/main/Detour/Include/DetourNavMesh.h · main · official
- https://docs.rs/landmass/latest/landmass/ · 0.9.3 · official
- https://docs.rs/polyanya/latest/polyanya/ · 0.18.0 · official
- https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/aasfile.h · GPL release · shipped game
- https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/be_aas_reach.c · GPL release · shipped game
- https://github.com/id-Software/Quake-III-Arena/tree/master/code/botlib · directory listing (to find the files) · shipped game

## Sources not read
- https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/botlib/be_aas.h · HTTP 404 (the file is
  not in that directory; aasfile.h holds the travel types)

## Cross-topic notes
- R2: nodes carry R2-INT-REC2's area ids (REC2); one floor model for navigation, sight and interest.
- R1: REC3 adds a staleness hash between tuning and baked links (R1-DATA-OQ7).
- EDITOR_V2: link overrides and the stale-bake warning in the validation panel.
- game-designer: OQ1 to OQ3.
