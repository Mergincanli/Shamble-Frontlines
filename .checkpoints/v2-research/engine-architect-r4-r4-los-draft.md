# R4-LOS draft: line of sight across heights (engine-architect, 2026-10-28)

All sources read 2026-10-28. Tier: official / shipped game / other.

## Findings
- **R4-LOS-F1.** Quake III's bots judge whether they see an enemy by tracing from their eye to three points on the
  enemy's box (its centre, its bottom and its top), after a field-of-view check, and return a visibility fraction.
  Quotes: "returns visibility in the range [0, 1] taking fog and water surfaces into account"; "for (i = 0; i < 3; i++)
  {"; "if (i == 0) middle[2] += entinfo.mins[2];"; "else if (i == 1) middle[2] += entinfo.maxs[2] - entinfo.mins[2];";
  "BotAI_Trace(&trace, start, NULL, NULL, end, passent, contents_mask);"; "if (!InFieldOfVision(viewangles, fov,
  entangles)) return 0;". Source:
  https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/ai_dmq3.c (GPL release; the first
  100,000 of 158,046 characters read), `BotEntityVisible`. Tier: shipped game. Sure: confirmed in the source.
- **R4-LOS-F2.** Before any trace, Quake III's enemy search drops clients beyond a distance that depends on the bot's
  alertness; a potentially-visible-set test is present only as a commented-out line. Quotes: "if (squaredist >
  Square(900.0 + alertness * 4000.0)) continue;"; "vis = BotEntityVisible(bs->entitynum, bs->eye, bs->viewangles, f,
  i);"; "//if (!AAS_inPVS(eye, middle)) continue;". Source: the same file, `BotFindEnemy` and `BotEntityVisible`. Tier:
  shipped game. Sure: confirmed in the source; the field-of-view value (360 when hurt or shot at, otherwise
  distance-dependent) is from the fetch summary (inferred).
- **R4-LOS-F3.** parry3d's BVH traversal is under the caller's control, so a sight test can stop at the first blocker
  instead of looking for the nearest one. Quotes: `TraversalAction` "Controls the execution flow of `Bvh::traverse`.";
  `traverse` "Traverses the BVH in depth-first order with full control over traversal."; `cast_ray` "delegates the
  ray-casting task to an external function." Sources: https://docs.rs/parry3d/latest/parry3d/partitioning/index.html
  and .../struct.Bvh.html (parry3d 0.31.1). Tier: official. Sure: confirmed in the source; the early exit is inferred
  from "full control".
- **R4-LOS-F4.** In Rapier's queries a "solid" ray that starts inside a shape hits it at distance 0, while a non-solid
  one treats shapes as hollow. Quotes: "if `solid` is `true` then the hit point will be the ray origin itself (`toi =
  0.0`)"; "If `solid` is `false` then the shape will be assumed to have an empty interior". Source:
  https://rapier.rs/docs/user_guides/rust/scene_queries (Rust 0.36). Tier: official. Sure: confirmed in the source.
  (Relevant to legacy CS4's inside-start miss: the start rule must be chosen, not inherited.)
- **R4-LOS-F5.** No source read in this run gives the cost of one ray or segment query through a BVH, in Rust or in
  Wasm. Sure: confirmed (an absence; R4-PHYS-F13 and F27 say the same for Rapier; R2-INT-OQ5 asks for it).
- (Used, read under other questions: R4-PHYS-F10, F12 (queries and filters), F21 (a BVH built once for a static
  scene); R4-FLOORS-F1 (stacked floors kept apart).)

## Legacy facts used
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

## Options

| | A. Rays against statics only (legacy) | B. Rays against statics and selected movers | C. Precomputed visibility (PVS-like) only |
|---|---|---|---|
| What | Segment queries through the static BVH | As A, plus vehicle hulls (and soldiers, if the rule says) from the per-tick movers index | Visibility between map cells baked offline; no rays at runtime |
| Height | Yes, if eyes and targets are 3D points and ramps are slopes | Yes | Coarse: per cell, not per point |
| Cost | Bounded by pairs × points (R2-INT-F20) | Plus the movers index (rebuilt each tick anyway, R4-PHYS-REC2) | Cheapest at runtime; bake cost; wrong for moving cover |
| Fit | §6.2; vehicles never give cover | §6.2 and §6.4 (big vehicles as cover, R2-SYNC-OQ8) | Not enough alone: open-ground walls and moving vehicles; Q3 itself commented its PVS test out of bot sight (F2) |

## Recommendations

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
  Rests on: F1, F3, F4; R4-PHYS-F12, R4-PHYS-REC2, R4-FLOORS-REC2; legacy CS4, CS9, GN4, GN7, GW15, GW16, GC12.
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

## Open questions
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

## Sources read
- https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/ai_dmq3.c · GPL release (two reads:
  `BotEntityVisible`, `BotFindEnemy`) · shipped game
- https://docs.rs/parry3d/latest/parry3d/partitioning/index.html · parry3d 0.31.1 · official (read under PHYS)
- https://docs.rs/parry3d/latest/parry3d/partitioning/struct.Bvh.html · parry3d 0.31.1 · official (read under PHYS)
- https://rapier.rs/docs/user_guides/rust/scene_queries · Rust 0.36 · official (read under PHYS)

## Sources not read
- none specific to this question (the cost question has no source read: F5).

## Cross-topic notes
- R2: REC1 mark 4 answers R2-INT-OQ5; REC3 is the server query R2-SYNC-REC8 asks for; area ids shared with
  R2-INT-REC2 (R4-FLOORS-REC2).
- R1: blasts (NC-7, GC14) and melee (GC12) use REC1's query; the shield's frontal arc reads sim facing (R1 cross-topic
  note), not a render object.
- game-designer: OQ1, OQ3, OQ4.
