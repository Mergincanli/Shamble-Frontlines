# ADR-0004: The simulation core

**Status:** Proposed
**Date:** 2026-10-31
**Author:** rust-reviewer · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no

This record decides how the game's rules run. There is one Rust simulation: the same code runs natively on the match
server and as WebAssembly in the browser's Web Worker. Its state is plain typed tables in one struct (no ECS crate),
every entity has one id, and time is a whole number of ticks of one constant. Runs are reproducible: the same seed, data
and inputs give the same state hash after every tick on every run of a build, and the same hash on Wasm and native is a
target that a test checks on every change. Physics is collision queries plus our own kinematic movement (parry3d, no
physics engine); bot navigation is baked into the map document and queried by the sim; one height-aware sight query
serves bots, blasts, melee and the aim point. The record also says where the player's upgrade tracks live (a player
record apart from the soldier), how dropped weapons exist in the world, and which random stream draws upgrade offers.
It decides no game rule. Why: VISION_V2 §6 asks for a true 3D world, data-driven abilities, big vehicles, a server-side
economy, telemetry and moments, and SLICE-ORDER asks that movement and shooting be input-driven and replayable from day
one.

## Context

**What the decision must serve.**
- VISION_V2 §6.1 (modes as data: the slot tables are sized from the mode), §6.2 (a true 3D simulation: floors and
  interiors, ledges, line of sight across heights, bots between levels, aiming at height), §6.3 (abilities and effects
  from data, executed by the sim), §6.4 (big, multi-part vehicles; seats `[OPEN]`), §6.5 (economy and objectives,
  server-side and tunable), §6.6 (balance telemetry), §6.8 (moments; kill-cams, replays and highlights `[LATER]`) and
  §6.9 (a few very large units: the sim's share is that they are in the stress scene; drawing them is ADR 0003's).
- VISION_V2 §3 ("Bots fill empty slots") and §4 (matches feel fair up to about 100 ms of ping; the minimum laptop runs
  the largest mode at 30+ fps).
- LOOP_V2 §2 (spawn inside the barracks), §3 and §4 (money, harvesters, uplinks, tech tiers), §5 and §6 (units and
  vehicles), §7 (active reload, split feedback), §8 (points race; "Nothing carries into the next match"), §9 (the
  Octopon-3 greybox: plateau, ramps, drop-downs, 6 m gaps, the reachability check) and §10 (bots that fight across three
  teams, buy, escort and hunt harvesters, and carry, defend and steal uplinks; the replan storm fixed first).
- EDITOR_V2 §3 (the validation panel: bot reachability, the 6 m gap rule; Test plays the map with bots at once).
- LOOK_V2 §3 ("Buildings sized for play: interiors, upper floors, verticality") and §4 (ruined buildings you can enter,
  collapsed roofs, planks up to upper floors; damage stages `[DEFAULT]`: intact → damaged → ruin, debris and planks
  become routes; roofs fade `[DEFAULT]`, a render matter).

**Settled inputs** (cited by their STATUS decision code or GLOSSARY §8 entry; never reopened):
- The stack: a Rust simulation core, Wasm in the browser and native on the server (STATUS "Decisions pending"; brief §1).
- SLICE-ORDER and its three conditions (STATUS "Decisions pending"): from slice 1 the game runs as a client plus a local
  server, the sim in a Web Worker behind the network's own messages; movement and shooting are input-driven and
  replayable from day one; [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test) runs before
  [the foundation](../research/GLOSSARY.md#the-foundation).
- TICK-30 (STATUS "Decisions pending"): 30 Hz is the baseline; 60 Hz only with
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) of at least 25 %, which ADR 0001 owns and measures in
  the netcode proof test.
- MODES-3 (a): the largest mode is 24v24, N = 48. SEC-SPREAD (a): weapon spread is known only to the server ("server-only
  spread"). ABL-PREDICT: open until a Feel Lab test.
- UPGRADE-SCOPE (Mustafa, 2026-10-31; recorded in STATUS with the day's decisions): every unit and every weapon type has
  its own upgrade track (pick one of three per level), owned by the player; tracks survive death and switching and reset
  at the start of every match; a class always comes with its own weapon; other weapons come only from killed enemies'
  drops, which are world pickups; progress belongs to the weapon type, not the physical gun; the tracks live on the
  PlayerState, not the Pawn. Carry slots, ammo for looted weapons and how fast tracks level stay open (ADR 0005 lists
  them).
- SPIKE-SCOPE (a): spikes are throwaway and exempt from the legacy constraints. R7-MARKS (a): the inventory's marks are
  working recommendations.

**The findings the decision turns on.**
1. Neither ECS crate orders its queries or clones its world, and bevy's Entity is documented not to line up between
   instances (r1-simulation-core.md, Recommendations, R1-ECS-REC1, lines 1891–1950).
2. std's transcendental functions vary by platform and come from different code on Wasm and Linux; NaN bits are not
   deterministic; relaxed SIMD is hardware-dependent; every std `HashMap` instance takes its own seed; the pure-Rust
   `libm` is one implementation on every target (r1-simulation-core.md, Recommendations, R1-DET-REC2, lines 2146–2169;
   R1-DET-REC5, lines 2208–2221).
3. With server authority a mismatch between client and server costs a correction; lockstep would turn it into a desync
   (r1-simulation-core.md, Recommendations, R1-DET-REC1, lines 2129–2145).
4. No VISION_V2 §6 item needs a rigid-body dynamics solver, and Rapier's character controller is itself only ray and
   shape casts around a move the caller asks for (r4-physics-navigation.md, Options, lines 589–604; Findings,
   R4-PHYS-F1, lines 56–60).
5. Rust's browser target has no C or C++ toolchain, so C++ Detour cannot simply be linked into the sim's Wasm build
   (r4-physics-navigation.md, Findings, R4-NAV-F8, lines 303–311), and no Rust navigation crate read promises
   determinism (R4-NAV-F16, lines 362–365).
6. The Wasm target builds with `-Cpanic=abort`, so one panic ends the client sim's instance (r1-simulation-core.md,
   Findings, R1-BND-F19, lines 1019–1028; Open questions, R1-RR-OQ3, lines 2936–2940).

**What it takes from and gives to the other drafts.**
- Takes: the tick rate and the 60 Hz margin (ADR 0001); the prediction scope, the messages and the room lifecycle
  (ADR 0001); the definitions of units, weapons, abilities, tracks and maps as typed content (ADR 0005, interface I-1);
  the Worker, the output block's layout, the panic strategy, the build profiles and the lints (ADR 0002); the reference
  server (ADR 0006, R6-HOST-REC2); the client reference machines (ADR 0003, R3-BUDGET-REC1).
- Gives: the fixed-tick mechanism, [the sim step share](../research/GLOSSARY.md#the-sim-step-share-and-the-server-only-share)
  and its parts, and [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share) (seam S1, which ADR 0001
  adds into [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget)); entity ids (ADR 0001 sets their
  wire width); [the state hash](../research/GLOSSARY.md#state-hash) and the determinism suite (I-17); the per-tick
  events and moments (I-10, I-20); the ray query behind the aim point (I-19); the sim's parts of the map document:
  colliders, baked navigation, floors and enclosure areas, structure states (I-9, in ADR 0005's schema); recorded
  matches (I-16); the player record that holds the upgrade tracks (I-21).

**What the legacy game proves** (rows of `docs/LEGACY_INVENTORY.md`, as the research cites them):
- A fixed step was declared but never used: the loop runs on a variable frame time (CS1). Variable steps change
  outcomes: at 20 Hz the legacy jump lost 11 % of its apex (GW14; r1-simulation-core.md, Options 4a, line 1653), the
  reach formula overstates the integrated jump by about 9 cm at 60 Hz (GW14; r4-physics-navigation.md, R4-FLOORS-REC1,
  lines 903–923), and resetting fire timers instead of carrying the remainder loses up to 28 % of the Shabzon's rate at
  30 Hz (GC5; r1-simulation-core.md, R1-ABL-REC3, lines 2402–2419).
- Same-seed matches differ run to run: the game reads the wall clock in 11 places (STATUS, "Open bugs and risks",
  WALL-CLOCK), and gameplay shares `Math.random` with effects and audio (GP14, GN8; r1-simulation-core.md,
  R1-DET-REC4, lines 2184–2207).
- Three hit worlds disagree: shots, sight and blasts use different worlds (CS9, GN7, GW15), static colliders have no
  ids (CS7), and bots neither hit nor avoid the Juggernaut (GC17) (r4-physics-navigation.md, R4-PHYS-REC2, lines
  721–745; R4-OBST-REC1, lines 1022–1041).
- Bot navigation does not scale: the 2 m grid closes gaps of 3 to 5.4 m and sends 27.5 % of objective routes across a
  roof bots cannot climb (GN11; r4-physics-navigation.md, legacy facts, lines 391–393), the route budget resets per
  frame and denies 52–57 % of routes at 48 soldiers (GN13; lines 397–398), and the sight budget gives about 2.5 scans
  per bot per second at 48 soldiers against 4 at 12 (GN4; R4-LOS-REC2, lines 990–1006).
- The legacy JavaScript step took 0.80–1.00 ms mean at 48 soldiers, without physics or a 3D world (GC2;
  r1-simulation-core.md, R1-ECS-REC1 mark 1, lines 1930–1931): a reference, not a prediction (R1-TICK-OQ5, lines
  2658–2660).
- The ADR-001 inputs STATUS logged (decision map, J2) land here: one hit world for every shooter (K1), a phased
  `update()` (K4), boot-time `Math.random` draws, three.js UUIDs and the camera-dependent effects cull on the shared
  generator, `addTech`'s HUD writes as an event, and a sight budget sized for 12 soldiers. Parts 1, 8, 13, 17 and 19
  of the decision dispose of them; the team allow-list on the server is ADR 0001's, with team filtering in the sim.

## Decision

One Rust simulation crate holds the game's rules and runs them in whole ticks: natively in each room process on the
server, and as Wasm in the browser's Web Worker for prediction and for the local server of SLICE-ORDER. Its state is
one plain struct of typed tables, its only clock is an integer tick counter, its random numbers come from seeded
streams, and it reads no wall clock, no hash-map order and no platform maths, so a recorded match can be replayed and
compared tick by tick on every target. Collision is parry3d's queries with our own kinematic movement; navigation is
baked data the sim queries; bots are sim code that sends the same inputs as players. Each part below says what, why, its
research ids, and the need it serves.

1. **The sim state is plain data.** One state struct of typed tables: a fixed-slot soldier table sized from the mode's
   team count and team size; generational pools for [big vehicles](../research/GLOSSARY.md#big-vehicle-and-very-large-unit)
   and their parts, [live projectiles](../research/GLOSSARY.md#live-projectile), weapon pickups (part 18) and
   [active effects](../research/GLOSSARY.md#active-effect-and-visual-effect) (as rows in an ordered effect pool on their
   target: owner id, definition id, timers, stacks); per-team tables; the player records (part 18); the static colliders
   with their ids; the random streams' state; the tick counter; the tick's event list. Phases are plain functions called
   in one fixed order, starting from the legacy order (GP14). No ECS crate; nothing lives outside the struct (no globals
   or statics in the sim crate). One match runs on one thread: no parallelism inside a match unless the sim core spike
   shows the sim step share cannot hold at N = 48.
   - Why: the entity kinds are few and known, ECS crates leave iteration order unspecified and cannot clone their world,
     and a plain struct gives cheap snapshots and a defined order.
   - Ids: R1-ECS-REC1, R1-ECS-REC2, R1-ABL-OQ10, R1-ECS-OQ4, R1-ECS-OQ7 (r1-simulation-core.md, Recommendations,
     lines 1891–1971; Open questions, lines 2622–2636, 2783–2784; Options, ECS table, lines 1630–1644).
   - Need: VISION_V2 §6.1, §6.2, §6.3, §6.4, §6.5, §6.8.
2. **One id for every entity.** An id is (kind, index, generation), owned by the sim, for every entity including static
   colliders and pickups; the server, the predicting client, snapshots, events, the output block and the TypeScript
   client all use it; no library-internal id ever reaches the wire. A soldier's slot index is its sim index (GC2). The
   wire width of ids and of the tick is ADR 0001's (R1-XT-OQ11).
   - Why: bevy's Entity must not be lined up between instances; generational keys reject stale handles after a slot is
     reused; legacy cannot name the static a shot hit (CS7).
   - Ids: R1-ECS-REC3 (r1-simulation-core.md, Recommendations, lines 1973–1991).
   - Need: VISION_V2 §6.3, §6.4, §6.6, §6.8.
3. **Snapshot and restore for re-simulation.** The whole state is copied with a hand-written `clone_from` into a
   preallocated copy that reuses its buffers, so a snapshot makes no heap allocation after warm-up (the derived
   `clone_from` would not reuse them: R1-ECS-OQ2, settled at the R1 fix step). The static collision index and the
   movers' index are rebuilt after a restore, never stored in a snapshot (part 10). Restore and replay serve what ADR 0001's
   [prediction scope](../research/GLOSSARY.md#the-prediction-scope) covers, the determinism suite's fidelity test and
   recorded matches. With no Rapier sets in the state (part 10), R1-ECS-REC4's physics snapshot and R1-ECS-REC1 mark 2's
   separate Rapier count fall away. Whole-world restore and replay (R1-ECS-REC1 mark 3) stays a mark only if ADR 0001's
   prediction scope needs it; under R2-SYNC-REC3's scope (own movement, own weapon state, own vehicle) it is dropped
   (R1-XT-OQ7 is ADR 0001's).
   - Why: the client replays its own commands from the server's state on every snapshot, and the suite must show that a
     restored run continues byte for byte.
   - Ids: R1-ECS-REC1 marks 2–4, R1-ECS-REC4, R1-ECS-OQ2, R1-ECS-OQ5 (r1-simulation-core.md, lines 1932–1945,
     1993–2005, 2614–2618, 2627–2628); R4-PHYS-REC2 (r4-physics-navigation.md, lines 721–745).
   - Need: VISION_V2 §4 only (fair up to about 100 ms of ping) and SLICE-ORDER condition 2 (replayable from day one).
4. **One fixed tick, counted in integers, and one time representation.** A `u32` tick counter is the sim's only clock
   (sim time = tick × `dt`); `dt` is one build-time constant, the same on server and client; the step takes no time
   argument and reads no clock. The tick rate is ADR 0001's under TICK-30 (30 Hz; 60 Hz only with the 60 Hz margin), and
   it changes only with a rebuild: it is never a server option and never changes on a content reload (R1-XU-OQ10 is ADR
   0005's; a build-time constant answers it). At 30 Hz a `u32` counter wraps after 2^32 ÷ 30 s, about 4.5 years
   (arithmetic), far past a match of about 20 minutes (LOOP_V2 §8); counters and timers still use explicit wrapping or
   checked operations. Durations are authored in seconds (LOOP_V2's one data table) and converted at load into an integer
   count of sub-tick units in which one tick is an exact whole number, with the rounding of each value reported; timers
   carry their remainder (the next shot is due one interval after the previous scheduled shot, not after the tick it
   landed on). That is R1-TICK-REC1's "ticks with remainders carried" and R1-ABL-REC3's "integer time unit" as one
   representation (R1-XU-OQ7); the number of units per tick is set in the sim core spike from the rounding report (the
   research gives no number). Effects start, tick and expire on tick boundaries, in a fixed order within a tick (entity
   index, then start tick, then a per-match sequence number). Slow systems (bot target scans, sight budgets, route
   replans, the economy trickle, telemetry sampling) run every k ticks, k from a cadence in seconds, staggered by slot.
   Under TICK-30 every legacy fire interval is longer than a tick (0.072 s and 0.06 s against 0.0333 s), so no legacy
   weapon needs two shots in one tick; the rounding report flags any future interval under one tick for the
   game-designer (R1-ABL-OQ7).
   - Why: variable steps change outcomes (GW14, GC15), a reset-not-carry timer loses up to 28 % of a weapon's rate (GC5),
     and a fixed integer step is what replay, prediction and the parity tests need.
   - Ids: R1-TICK-REC1, R1-TICK-REC3, R1-ABL-REC3, R1-XU-OQ7, R1-ABL-OQ7 (r1-simulation-core.md, lines 2009–2027,
     2066–2076, 2402–2419, 2880–2885, 2776–2778).
   - Need: VISION_V2 §6.2, §6.3, §6.5, §6.8; SLICE-ORDER condition 2.
5. **The loops that drive the tick.** On the server, each room's loop wakes on a timer but takes the number of due ticks
   from a monotonic clock; catch-up per wake is bounded at 0.25 s of game time, which at 30 Hz is 7 ticks (proposed:
   the largest whole number of ticks within the bound, ⌊0.25 × 30⌋ = 7; the research's 15 is the 60 Hz form); a longer
   stall drops the time and records it, and every overrun and dropped interval is a counter for monitoring (ADR 0006,
   I-12). Snapshots go out after the tick (ADR 0001). In the browser, the Worker owns the tick: a `performance.now()`
   accumulator woken by `setTimeout` (W2), with `Atomics.wait` (W3) as the measured alternative if W2 fails the jitter
   mark (W3 needs shared memory, so cross-origin isolation: ADR 0002's call, R1-XU-OQ4). `requestAnimationFrame` is the
   page's render clock only. After a long gap (a hidden tab, a sleeping laptop) the browser does not simulate the gap:
   in local play the match pauses; online the Worker resyncs from the server's state through ADR 0001's path, with
   catch-up per frame bounded at 7 ticks (the same rule, proposed). Pause, shop and hit-stop never stop the shared tick
   online; hit-stop is presentation time. The loops read the clock; the step never does (WALL-CLOCK, decision map J2:
   the legacy reads stay in the frozen legacy game). Whether Worker timers keep their rate in a hidden tab is measured in
   the boundary spike, for 60 s and for more than 5 minutes (R1-TICK-OQ2 and R1-BND-OQ2, stated once: R1-XU-OQ12).
   - Why: Tokio's timers are millisecond-granular, so the rate must come from elapsed time; `requestAnimationFrame`
     follows the display and stops in background tabs.
   - Ids: R1-TICK-REC4, R1-TICK-REC5, R1-TICK-OQ2, R1-BND-OQ2, R1-XU-OQ12 (r1-simulation-core.md, lines 2078–2092,
     2094–2116, 2645–2647, 2724–2727, 2903–2912).
   - Need: VISION_V2 §6.5 (the server's economy and objectives keep time), §6.9 (the render loop is never blocked by the
     sim) and §4 (30+ fps on the minimum laptop, four browsers).
6. **The sim step share, its parts, and the client's sim share** (restated at 30 Hz, TICK-30). On
   [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of
   [the reference server](../research/GLOSSARY.md#the-reference-server), in
   [the stress scene](../research/GLOSSARY.md#the-stress-scene) at
   [the largest mode's N](../research/GLOSSARY.md#the-largest-mode-and-its-n) (48), the p99
   [sim step time](../research/GLOSSARY.md#sim-step-time) is at most the sim step share, 25 % of
   [the tick period](../research/GLOSSARY.md#tick-period): 8.3 ms at 30 Hz (4.2 ms in the 60 Hz margin run only). Its
   parts, each reported on its own: movement and physics at most 1 ms native mean, its p99 reported; avoidance at most
   0.2 ms; abilities, effects and weapons at most 1 ms; navigation at most 5 % of the tick period (1.67 ms); the shared
   visibility pass at most 5 % (1.67 ms; R2-INT-REC3's id is ADR 0001's, its number is this record's). Their ceilings
   sum to 5.53 of 8.33 ms, a planning check, not a bound ([time statistics](../research/GLOSSARY.md#time-statistics):
   p99s do not add); the rest of the sim (projectiles, economy, objectives, bot decisions, moments, events) takes what is
   left, 2.80 ms (8.33 − 5.53, arithmetic). At 60 Hz the same parts sum to 3.87 of 4.17 ms. ADR 0001 adds this share to
   the server-only share inside the server tick budget (seam S1). In the browser, the client's sim share is a p99 sim
   step time in Wasm in the Worker on the minimum laptop of at most 40 % of the tick period: 13.3 ms at 30 Hz (6.7 ms at
   60 Hz), with Wasm at most 2 × native. It applies wherever the browser runs the whole sim: the local server of slices 1
   and 2 (SLICE-ORDER), at an N that R1-TICK-OQ7 sets (a design question for Mustafa and the Advisor, in
   `docs/adr/README.md`). The page's own cost of the sim per frame is ADR 0003's row. In steady state the step makes no
   heap allocation (proposed: R1-ABL-REC6 sets this for its own part; the reason is the same for every part, a 33 ms
   loop that must not stall on the allocator).
   - Why: one budget, the one the glossary already split (resolution 1), restated for the 30 Hz baseline.
   - Ids: R1-ABL-REC6 (r1-simulation-core.md, lines 2465–2480); R1-TICK-REC2 P1 and P2, ADR 0001's id with these marks
     this record's (r1-simulation-core.md, lines 2044–2057); R1-XU-OQ1 (lines 2835–2847); GLOSSARY.md, §1, the table at
     lines 53–69 and the entries at lines 189–224.
   - Need: VISION_V2 §3 and §4 only (scale and the minimum laptop): a budget, not a feature.
7. **Identical results: two levels, and the float rules.** Same-build reproducibility is required: the same seed, data
   and input stream give the same state hash after every tick, on every run of the same build. Identity between Wasm and
   native is a target, tested on every change, that the netcode never depends on: the server is
   [the authority](../research/GLOSSARY.md#the-authority), and the client's prediction is corrected when it differs. No
   lockstep, no fixed point. Inside the sim: f32 throughout (up to a map 2 km across, R4-PHYS-REC5); only `+ − × ÷`,
   `sqrt`, `floor`, `ceil`, `round`, `trunc`, explicit `mul_add` and `as` casts from core; transcendental maths only
   through the pure-Rust `libm` crate, pinned; no std `sin`, `cos`, `atan2`, `powf` or `exp`, no `algebraic_*`, no
   fast-math, no `relaxed-simd`; Wasm `simd128` off until the suite passes with it on. NaN never enters sim state: every
   input and data value is checked finite where it enters, and debug and test builds assert finite state each tick. No
   sim value is computed in TypeScript. Fused multiply-add across targets: Rust never fuses on its own, so the risk is a
   dependency's SIMD path (wide's `mul_add` fuses only where the target has FMA); the server therefore builds for
   baseline x86-64, never with `+fma` or `-C target-cpu=native`, glam's `scalar-math` is on if the sim uses glam itself,
   and native AArch64 joins the native-against-Wasm hash test before ADR 0006 may pick ARM hosts (the build flags are
   ADR 0002's). parry3d's own `enhanced-determinism` flag is on for both targets (critic rank 3). Attribute maths
   (ADR 0005's data model) runs in f32 under these rules with a fixed, documented aggregation order per attribute
   (R1-ABL-OQ5). An exactly representable `dt` does not matter for identity: `dt` is one constant with the same bits on
   every target, and timers count integer units (part 4); whether 64 Hz is worth it is R1-TICK-OQ1, ADR 0001's
   (R1-XU-OQ11). Whether browsers run a different Wasm float profile matters only if NaN can enter state, which these
   rules forbid; the suite's browser runs check it (R1-DET-OQ4).
   - Why: basic operations, `sqrt` and `mul_add` are exactly specified on every target while std's transcendental
     functions are not; NaN bits and relaxed SIMD differ by engine; with server authority a divergence costs a
     correction, not a desync.
   - Ids: R1-DET-REC1, R1-DET-REC2, R1-DET-REC3, R1-DET-REC7, R1-RR-OQ1, R1-ABL-OQ5, R1-XU-OQ11, R1-DET-OQ3,
     R1-DET-OQ4 (r1-simulation-core.md, lines 2129–2183, 2242–2248, 2923–2932, 2770–2771, 2899–2902, 2686–2690);
     R4-PHYS-REC5 (r4-physics-navigation.md, lines 786–798).
   - Need: VISION_V2 §6.2, §6.3, §6.4; §4 (corrections stay rare at about 100 ms of ping).
8. **Random streams.** The sim draws only from `rand_chacha`'s `ChaCha8Rng`, pinned, seeded by the authority per match,
   one stream per purpose. The sim maps raw generator output with its own few sampling functions and pinned test
   vectors, never rand's distribution functions (R1-DET-OQ5: their outputs carry no stability statement across
   versions, and uniform floats "may equal `high`"). The streams' state (seed, stream, position) is part of the
   authority's state and of the state hash. The kind of each stream
   ([shared stream and authority-only stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream)): a
   stream is authority-only unless ADR 0001's prediction scope makes the client's predicting sim draw from it. Today's
   scope (R2-SYNC-REC3) draws from none, so every stream starts authority-only: match setup, weapon spread (SEC-SPREAD
   (a)), bot decisions, ability procs (shared only if ABL-PREDICT says yes and an own ability draws a proc), and the
   upgrade offers of three (part 18). Offers are authority-only for SEC-SPREAD's reason: a seed the client holds lets a
   cheat compute what the stream gives before the server shows it, here the player's next offers (and other players'
   offers, if they share a stream); and an offer is never predicted (it arrives as an owner-only message, ADR 0001), so
   keeping its stream on the authority costs nothing. Presentation randomness (visual effects, sound, camera shake,
   animation variety) uses the page's own generator and is no sim stream; tracers use a cosmetic spread that the hit
   event corrects (ADR 0001, ADR 0003). Animation never moves a launch point or a hitbox. In local play the Worker is the
   authority and holds the authority-only streams. If ChaCha8's cost shows in the profile, a fallback needs a source on
   keeping PCG streams independent first (R1-DET-OQ11).
   - Why: `StdRng` is not portable; ChaCha8 is portable, reference-tested and has streams and a settable position; the
     legacy's shared `Math.random` is what made same-seed matches differ (GP14).
   - Ids: R1-DET-REC4, R1-DET-OQ5, R1-DET-OQ11 (r1-simulation-core.md, lines 2184–2207, 2691–2696, 2713–2717);
     SEC-SPREAD (a); R2-SYNC-REC3 (r2-netcode.md, lines 2068–2105).
   - Need: VISION_V2 §3 (bots), §6.3 (ability procs), §6.5, §6.6; UPGRADE-SCOPE (the offers).
9. **Order, the hash-map ban, the state hash and the determinism suite.** State lives in dense arrays or slot maps keyed
   by stable ids and is iterated in id order; phases run in their fixed order. [The hash-map
   ban](../research/GLOSSARY.md#the-hash-map-ban-in-the-sim-crate) holds in the sim crate: no std `HashMap` or `HashSet`;
   `BTreeMap` or an index-ordered `Vec` instead; the one escape is a reviewed `#[allow(clippy::disallowed_types)]` with a
   written reason, for a lookup-only map outside the serialized state (the lint is ADR 0002's, R5-RUSTLINT-REC2). After
   each tick the sim writes a canonical serialization of its whole state (fixed field order, floats by `to_bits`, the
   streams' state, the tick counter), field by field and never as raw struct bytes (padding bytes are uninitialised,
   R1-RR-OQ6), and hashes it with a fixed, unseeded hash function: the state hash (which function is picked in the sim
   core spike; the research names none). The determinism suite runs on every sim change: a recorded match (the seed, the
   map and mode data, the content data's hash and the per-tick inputs) runs natively twice in separate processes, as Wasm
   in Node.js, and as Wasm in Chrome and Edge, Firefox and Safari; it reports the first mismatching tick with a
   field-level diff of the two states; unit vectors for `libm` and the streams run on the same targets; a deliberately
   injected divergence (one std `sin`, one hash-map iteration, one presentation draw inside the sim) must be caught at the
   first tick it changes the state. Its tools and CI jobs are ADR 0002's (R5-TEST-REC2). Recorded matches with per-tick
   hashes keep VISION_V2 §6.8's `[LATER]` kill-cams, replays and highlights possible; nothing is built for them.
   - Why: no source gives a verification method, so the suite defines one; every std `HashMap` instance takes its own
     seed, so two runs can expose a leaking order; engines differ, so the browsers are in the matrix.
   - Ids: R1-DET-REC5, R1-DET-REC6, R1-XU-OQ5 (settled: per instance), R1-XU-OQ6 (settled: tier "other")
     (r1-simulation-core.md, lines 2208–2241, 2866–2879); GLOSSARY.md, §7, lines 729–742.
   - Need: VISION_V2 §6.6 and §6.8 (numbers and moments from runs that can be re-run); §3 (the scale test at 48).
10. **Physics: collision queries and our own kinematic movement.** No physics engine. parry3d (pinned,
    `enhanced-determinism` on) gives shapes, queries and the BVH; soldiers move by our own kinematic slide-and-step move
    (option C); Rapier's character controller (option B) is measured once in the sim core spike as the fallback and is
    picked only if it wins on movement parity and cost. One static collision world comes from the map document, immutable
    for the match, each collider with its sim id; its BVH is built once at match start in id order; a destructible
    piece's state lives in the sim's tables and filters queries; the movers' index is rebuilt from the tables each tick.
    Shots, sight, blasts, melee, the aim point and the grenade-arc preview call the same query functions. Projectiles
    are per-tick swept queries, never physics bodies, with no CCD; grenades bounce by reflecting at the hit normal;
    knockback is a velocity change. Big vehicles are kinematic compound bodies (hull, turret, legs, each with its hit
    volume and alive flag) swept against statics and soldiers each tick; at 30 Hz a vehicle at 6 m/s moves 0.2 m per
    tick, so no substeps (R1-TICK-OQ9); a turning hull uses a nonlinear cast or an overlap test before the sweep, picked
    by the spike. What a vehicle does to a soldier in its path, and whether soldiers block each other, are design
    questions (R4-PHYS-OQ1, R4-XT-OQ6; `docs/adr/README.md`). The physics code sits behind one sim module, so a later
    need for dynamics (none in VISION_V2 §6 today) would swap one module (R4-PHYS-OQ5). The R1 items that assumed Rapier
    (critic rank 3) change as follows: R1-DET-REC3's flag moves to parry3d; R1-ECS-REC4 and R1-ECS-OQ5 apply only if
    option B wins; R1-ECS-REC1's Rapier reasons and its mark 5 become R4-PHYS-REC1 mark 4 (size with parry3d, with
    rapier3d, with neither); R1-TICK-REC2's "Rapier's default step" reason for 60 Hz no longer applies (TICK-30 sets the
    rate, and no physics step runs); R1-RR-OQ1's SIMD and FMA hazard is tested on parry3d's "3D f32 SIMD" BVH
    (R4-PHYS-OQ3); R1-DET-OQ1 and R1-DET-OQ2 ask the same of parry3d's flag and SIMD path.
    - Why: no §6 item needs a dynamics solver, a shipped shooter's shared movement is exactly kinematic trace code, and
      owning the move keeps reach and step-up the sim's own integrator.
    - Ids: R4-PHYS-REC1–REC5, R4-PHYS-OQ2, R4-PHYS-OQ3, R4-PHYS-OQ5 (r4-physics-navigation.md, lines 672–798,
      1085–1095); R4-XT-OQ2, R1-XT-OQ2, R5-XT-OQ1 (r4-physics-navigation.md, lines 1175–1182; r1-simulation-core.md,
      lines 3022–3029; r5-toolchain-quality.md, lines 1963–1970); R1-TICK-OQ9 (r1-simulation-core.md, lines 2675–2676).
    - Need: VISION_V2 §6.2, §6.3 (projectiles and blasts), §6.4.
11. **Navigation: baked data, queried by the sim.** The sim owns the navigation runtime; the navigation data is baked
    by a tool and stored in the map document, read-only during a match: convex nodes with heights, portals, typed links
    (part 12), area ids and flags (part 14), integer traversal costs. The sim crate holds the queries: nearest node, A*
    over nodes, a string-pulled path, link traversal. The bake runs in the editor's Test, on map save and in CI, from the
    same collision world the sim moves in, with its agent numbers taken from the soldier's movement data and their
    rounding reported; where the bake code lives in the crates is ADR 0002's (R4-XT-OQ5). The sim core spike compares two
    generators, rerecast's polygon mesh and a layered grid built from the sim's own collision queries. Path requests go
    through a queue with a fixed per-tick budget counted in node expansions, served in a rotating fair order, with
    hysteresis so a flickering straight line cannot start a replan storm; shared goals (uplink sites, harvest fields,
    base entrances, the carried cache) get per-goal distance tables filled inside the same budget. Local avoidance is a
    step inside the movement phase: a bot's preferred velocity is adjusted against neighbours on the same floor layer,
    taken from the movers' index in id order, and goes through the same movement as a player's input; ORCA through
    dodgy_2d is adopted only after an audit against the float rules, with a plain separation force as the fallback.
    Navigation runs where bots run: on the server, and in the browser's local server, which SLICE-ORDER makes a slice 2
    need ("bots and three teams, played locally"); client prediction never runs it, and a Worker build without bots
    skips the navigation data. Navigation data changes only with the map: a new bake means a new match. polyanya is not
    used at run time, so its hashbrown maps need no audit unless a later change brings it in (R4-NAV-OQ4).
    - Why: C++ Detour cannot be linked into the browser build, no Rust navigation crate promises determinism, baking
      once makes generation's floats shared data, and Quake III shipped bots on a precomputed area graph.
    - Ids: R4-NAV-REC1–REC4, R4-NAV-OQ1–OQ4 (r4-physics-navigation.md, lines 802–899, 1099–1107).
    - Need: VISION_V2 §6.2 (bots that move between levels) and §3 ("Bots fill empty slots"); LOOP_V2 §9 and §10;
      EDITOR_V2 §3 (Test, the validation panel).
12. **Floors and links.** The bake generates typed links (walk, jump-up onto a ledge, one-way drop-down, gap jump) and
    keeps only those the sim's own movement completes at the shipped tick; designers add or forbid links in the editor.
    The type list is data, so later types (a plank, a ladder) need no format change. Each link stores its entry and exit
    points, type, the input a soldier issues, direction and an integer travel time; a bot uses a link by issuing the
    same inputs a player would. Floors are separate layers with ceilings from day one: a surface is walkable only if the
    soldier fits under what is above it, and each node carries its layer and its enclosure area id, the same area ids
    ADR 0001's interest uses (R2-INT-REC2). The bake reads the soldier's movement data from the tuning table the sim
    reads, and the map document records a hash of those values; a mismatch marks the navigation stale in CI and in the
    editor.
    - Why: a hand-placed link can promise a jump the movement cannot make (GW14), a 2D grid cannot hold a floor above a
      floor (GN10, GN11), and humans take over bot slots, so bots and players need one movement rule.
    - Ids: R4-FLOORS-REC1–REC3 (r4-physics-navigation.md, lines 903–954).
    - Need: VISION_V2 §6.2 (floors and interiors, bots between levels); LOOK_V2 §3 (upper floors, verticality).
13. **Sight and aim at height.** One height-aware sight query serves bots, blasts, melee, the shield check and the
    shared visibility pass: a segment from an eye point (stance data) to sample points on the target (three for a
    soldier, one per part for a vehicle), any-hit with early exit, against the static world and, by data flag, movers.
    Each collider carries data flags for what it blocks (movement, shots, sight), so any answer to the design questions
    on who blocks sight and shots (R4-LOS-OQ1) and on partial blockers (R4-LOS-OQ4) is data. Sight is culled before
    casting (distance and team, then area connectivity, then a field of view if bots get one, then segments) and
    budgeted in rays per tick with carried remainders and a rotating start. The client's cursor ray and the server's shot
    use the same static world from the same map document and the same query functions, so the point the player clicked
    is the point the server tests; the ray query is this record's, the command field ADR 0001's and how the result
    crosses the boundary ADR 0002's (interface I-19, critic rank 2).
    - Why: legacy uses three worlds for sight, shots and blasts (CS9, GN7, GW15); with 3D points, slopes and separate
      floors a plain segment test is height-aware, as Quake III's bots show.
    - Ids: R4-LOS-REC1–REC3, R4-LOS-OQ5 (r4-physics-navigation.md, lines 958–1018, 1133).
    - Need: VISION_V2 §6.2 (line of sight across heights, aiming at height).
14. **Obstacles placed mid-match, changing structures, and LOOK_V2 §4's places.** Moving vehicles are never carved into
    the navigation data: they are obstacles in the avoidance step by their hull footprint; a vehicle that stops or moves
    slowly marks the nodes under it with a temporary cost, refreshed only after it moves past a threshold, and only bots
    whose route crosses newly marked nodes replan. A structure that changes state is baked in each of its states; a
    change flips flags on its nodes, links and portals in the tick it happens, together with its colliders' state (which
    the sight and shot queries read) and ADR 0001's portal re-flood, so routes, sight and relevance change in the same
    tick. No navigation is generated during a match. LOOK_V2 §4's damage stages (`[DEFAULT]`: intact, damaged, a ruin
    with the interior exposed; debris and planks become routes) are built on this: one baked state per stage the map
    data declares (three for LOOK_V2 §4), each with its own walkable surfaces, links (planks as typed links to upper
    floors, debris as the ruin's walkable surfaces), sight flags, ceilings (a collapsed roof stops being a ceiling) and
    portals, each structure's stages baked over its own nodes only, never as combinations across structures. The
    research bakes two states and a ruin's shape (R4-OBST-REC2), so the third stage is an extension the sim core spike
    must prove (decision map h, G4). What moves a building from one stage to the next is a game rule for Mustafa
    (OQ-0004-4); the stages and their assets are content (ADR 0005); drawing them is ADR 0003's. Roofs that fade
    (LOOK_V2 §4 `[DEFAULT]`) are drawn by ADR 0003; the sim's part is what the camera needs: the local player's floor
    layer and enclosure area id, written into the output block (ADR 0002's layout). What a player may see inside a
    building is a design question (R2-INT-OQ1), and whether the cutaway follows the server's visibility rule is ADR
    0003's (critic rank 39). The bake is split into tiles only if a full bake is too slow for the editor's Test; the sim
    never re-bakes during a match; player building is not designed (R4-OBST-OQ3, a design question).
    - Why: carving a moving obstacle means rebuilding tiles every tick it moves, on both targets on the same tick; flag
      flips on baked data are identical everywhere and avoid the replan storm.
    - Ids: R4-OBST-REC1–REC3, R4-OBST-OQ4 (r4-physics-navigation.md, lines 1022–1076, 1145–1147).
    - Need: VISION_V2 §6.2 (interiors whose openings change), §6.4 (vehicles), §6.5 (objectives destroyed and rebuilt);
      the damage stages: LOOK_V2 §4 only.
15. **Bots run in the sim.** Bots run wherever the authority runs (each room on the server; the Worker in local play),
    fill empty slots and send the same input commands as humans, so the sim cannot tell them apart (R2-ROOM-REC6, ADR
    0001's). Their behaviour is a sim phase at its own cadences (part 4) that reads the shared sight pass, the navigation
    queries and the game state, and writes commands. The behaviour set is LOOP_V2 §10's (fights across three teams,
    buying T1 units, escorting and hunting harvesters, carrying, defending and stealing uplinks, a simple per-team role
    mix); no research covers a bot AI architecture (decision map h, G10), so its structure is OQ-0004-1. Bots act on the
    present while humans see about 100 ms of the past (R2-SYNC-OQ13): bot reaction is tuning data the game-designer
    sets, starting from legacy's 0.25 s scan cadence (GN4), and the sim core spike reports bot reaction times at N = 48.
    A field of view for bots is a design question (R4-LOS-OQ3).
    - Why: one simulation for humans and bots keeps the slot model simple and the bots fair under the same rules.
    - Ids: R2-SYNC-OQ13 (r2-netcode.md, lines 2952–2954); cites R2-ROOM-REC6 (lines 2842–2856).
    - Need: VISION_V2 §3 ("Bots fill empty slots") and §6.2 (bots between levels); LOOP_V2 §10.
16. **Abilities, effects and weapons, as the sim executes them.** ADR 0005 defines the data model (a reduced GAS subset
    over a closed set of Rust operations, weapons beside it sharing one damage pipeline); this record executes it inside
    the sim step: the operations in their fixed phase order, in integer time (part 4), with attribute maths under part
    7's rules and active effects as rows (part 1). A weapon's state machine (timer, ammo, burst count, charge, reload, the
    active-reload window of LOOP_V2 §7) is per-soldier sim state counted in sim time units; its numbers are tuning. The
    ability, effect and weapon step costs at most 1 ms p99 per tick in the stress scene with up to 8 active effects per
    soldier (384 at N = 48), and allocates nothing in steady state. Whether the client predicts its own abilities waits
    for ABL-PREDICT; the sim code is the same either way. Definitions are read by id (I-1), and a development reload swaps
    them between ticks, never inside one (R1-DATA-REC5, ADR 0005's).
    - Why: the closed set of operations lets combos come from data while the sim stays reviewable and deterministic.
    - Ids: R1-ABL-REC6 (part 6); cites R1-ABL-REC1, R1-ABL-REC2 (r1-simulation-core.md, lines 2342–2400).
    - Need: VISION_V2 §6.3, §6.4; active reload: LOOP_V2 §7.
17. **Economy and objectives as sim phases.** Money and harvester deliveries, team-funded rebuilds, uplinks (cache,
    carry, place, countdown, steal, rotation), tech tiers, power-plant pricing and the points race run as sim phases in
    the fixed order at the cadences of part 4, with their numbers in ADR 0005's tables and their rules exactly as LOOP_V2
    §3, §4 and §8 state them. Team filtering (who a shot or blast can hurt, GC1) and presence counting for steals (GE8)
    are sim code. What each team may see of another team's state is ADR 0001's visibility classes. A tier reached or
    money paid is an event (part 19), never a HUD write from the sim (legacy's `addTech`).
    - Why: LOOP_V2's economy is server-side and tunable, and its outcomes must replay like everything else.
    - Ids: none of its own (decision map h, G12); cites R1-TICK-REC3 (cadences).
    - Need: VISION_V2 §6.5.
18. **The player record, weapon drops and pickups (UPGRADE-SCOPE).** Each player slot has a player record, the
    PlayerState, in a table beside the soldier tables and apart from the soldier entity, the Pawn. For the match it
    holds one track per unit type and one per weapon type (level, picks taken, the pending offer of three), keyed by the
    type ids ADR 0005 defines; what a pick grants is applied through the ability system. The record survives the
    soldier's death and every unit switch: a new soldier entity reads its slot's record. A weapon type's track stays in
    the record while the player holds no gun of that type and applies again with the next gun of that type. The record's
    size is bounded by the content's type counts and allocated at match start; nothing a client sends sizes it. At match
    start the sim builds every slot's record empty, with the soldier tables, so nothing carries over from a previous
    match (UPGRADE-SCOPE; LOOP_V2 §8); ADR 0001's room lifecycle calls this phase. A class's own weapon comes from the
    unit definition (ADR 0005). When a soldier dies, its weapon becomes a pickup entity in the pickup pool: an id (part
    2), the weapon type id, a position settled on the ground with the movement query, and a lifetime in ticks from ADR
    0005's data. A pickup claim is an input command (ADR 0001's field) handled in the phase's fixed order, so two claims
    on one pickup in one tick always resolve the same way on every machine. The offer of three is drawn on the authority
    from the authority-only offers stream (part 8) when a track levels; the server refuses a pick it did not offer and a
    pickup it did not place (ADR 0001). Picks, offers shown, weapons looted and time per weapon type are events for
    telemetry (part 19; ADR 0006's fields). The rules stay open: carry slots, ammo for looted weapons and how fast tracks
    level (ADR 0005's open questions); which deaths drop which weapons, how long a drop lies, who may pick it up and how
    a tie is broken (OQ-0004-5); and what bots and slot takeovers do with records (OQ-0004-6).
    - Why: Mustafa put the tracks on the player, not the unit; a record apart from the soldier is the only place that
      outlives a death and a switch, and pickups as ordinary entities get ids, snapshots and hashes for free.
    - Ids: none in the research (decision map h, G13; J5).
    - Need: UPGRADE-SCOPE (a settled input, listed in `docs/adr/README.md`).
19. **Events, moments and telemetry.** Each tick the sim appends tick-stamped events to the tick's event list: hits,
    deaths and kills (killer and victim slot and unit id, the source weapon or ability id, assisting slots), blasts
    (position and strength, for the concussion shake and the fragments ADR 0003 draws), cosmetic shot events,
    purchases, uplink cache events, harvester deliveries and kills, structure kills and stage changes, tech tiers,
    upgrade picks and pickups. The event list is part of the state and its hash; ADR 0002 places it in the output block,
    ADR 0001 delivers it online with its visibility classes, the room writes the match record at Ended (ADR 0001), and
    ADR 0006 owns the record's shape. A moment phase runs after the outcome phases: it reads the tick's events and the
    state and emits moment events by rules held as data (ADR 0005's format; their thresholds are a design question,
    R6-TEL-OQ3), so the moments and the telemetry read the same stream. The timeline is sampled every 10 s of game time.
    No research covers a moment mechanism (decision map h, G11), so this part stays at VISION_V2 §6.8's level.
    - Why: every §6.6 number and §6.8 moment is a function of the events and the timeline (R6-TEL-REC1).
    - Ids: none of its own; cites R6-TEL-REC1 (r6-infrastructure.md, lines 784–811) and R1-TICK-REC3.
    - Need: VISION_V2 §6.6, §6.8.
20. **No panic from input or content, and recovery from a trap.** No panic is reachable from network input or content
    data: the sim takes only commands that ADR 0001's decoders validated and content that ADR 0005's loader validated
    (its loader returns errors and is fuzzed with zero panics); inside the sim, values a client or content can set never
    reach an `unwrap`, an unchecked index or a `clamp` with unchecked bounds (`f32::clamp` panics on bad bounds,
    R1-DET-F14). On the server a panic stops that room's match (each room is its own process, R2-ROOM-REC2); in the
    browser `-Cpanic=abort` ends the sim's instance. Online, after a trap, the Worker's host starts a fresh instance and rejoins through ADR 0001's resync path,
    and the trap is reported (R6-OBS-REC3, ADR 0006); the panic strategy is ADR 0002's. In local play the trapped
    instance was the authority; recovery there is not researched (OQ-0004-7).
    - Why: a panic on the server is a denial of service for a whole room, and in the browser it ends the match for that
      player.
    - Ids: R1-RR-OQ3, R1-XT-OQ12, R2-XT-OQ12, R6-XT-OQ6 (r1-simulation-core.md, lines 2936–2940, 3084–3087;
      r2-netcode.md, lines 3395–3398; r6-infrastructure.md, lines 1021–1024).
    - Need: VISION_V2 §4 only (play in four browsers; matches that feel fair).
21. **Map validation with the sim's own integrator.** The editor's reach value is the sim's movement run at the shipped
    `dt`, never a closed-form formula; the reachability check (every spawn to every goal) and the report of gaps under
    6 m run inside the sim crate, from Node for the editor and from CI, with identical results on native and Wasm.
    - Why: integration error depends on the step, and legacy's formula overstates the real reach (GW14).
    - Ids: R1-TICK-REC6 (r1-simulation-core.md, lines 2118–2125); R4-NAV-REC1 marks 3 and 4 (r4-physics-navigation.md,
      lines 835–837).
    - Need: VISION_V2 §6.2; EDITOR_V2 §3 (the validation panel) and LOOP_V2 §9 (6 m gaps, reachability).

## Options considered

Every option below comes from a research Options section (cited per question). Each table carries template 0000's
dimensions for every option; "Cost" means money and usage, where the research gives none it says so. Effort and size
judgements in the research are labelled inferred there, and stay so here.

### Q1. ECS or plain data (part 1)
Source: r1-simulation-core.md, Options, ECS or plain data, lines 1630–1644.

| Dimension | A. bevy_ecs 0.20 (standalone) | B. hecs 0.11.2 | C. Custom plain data (proposed) |
|---|---|---|---|
| Complexity | High: schedules, executor, change detection, observers, most of which no §6 item needs | Medium: an archetype world, no scheduler | Medium: our own pools, ids, ordering and snapshot, for about a dozen known kinds |
| Cost (money, time, usage) | No money; a migration roughly each 0.x release (inferred from five releases in 18 months) | No money; a smaller API to track | No money; the most code written up front |
| Scalability and performance | A multi-threaded executor exists (the one thing C lacks, R1-ECS-OQ4); size unmeasured | Small dependency closure; size unmeasured | Fewest dependencies; size unmeasured (all three: R1-ECS-OQ1) |
| Security and anti-cheat | Neutral; deterministic order needs a sort in every outcome system, or a mismatch costs corrections | Neutral; order is ours to add on every outcome loop | Neutral; storage order is iteration order, under the hash-map ban |
| Effect on the look and feel | None directly; snapshot traps (unregistered state desyncs silently) would show as corrections | None directly | None directly |
| Effort from the current code | Lowest to a first tick, highest ongoing for determinism and snapshots | In between | Highest up front; legacy's flat pooled SIM records (CS8) are its starting layout |
| Pros | Ready scheduler and relationships | Lighter than A; a column serializer | Defined order; one `clone_from` snapshot; the sim owns ids |
| Cons | World not Clone; Entity not shareable across instances | World not Clone; ids align only while live sets match | No ready-made change detection; we write pools and ids |

### Q2. The tick (parts 4 and 5)
Sources: r1-simulation-core.md, Options, A fixed tick rate, lines 1648–1682.
- **The tick rate:** settled by TICK-30 (30 Hz baseline; 60 Hz only with the 60 Hz margin, ADR 0001). The research's 20,
  60, 64 and 66 Hz rows and variable-duration commands are not weighed here.
- **Slow systems (4b, lines 1660–1665), the server loop (4c, lines 1666–1672) and the client loop (4d, lines
  1673–1682):**

| Dimension | Everything every tick | Cadences in whole ticks, staggered by slot (proposed) |
|---|---|---|
| Complexity | Lowest | Low: one cadence per system, stated in game time |
| Cost | None | None |
| Scalability and performance | AI, sight and route costs scale with the tick rate | Per-tick cost grows only with what must be fine-grained |
| Security and anti-cheat | Neutral | Neutral |
| Effect on the look and feel | None | None if cadences are in game time (bots behave the same at any rate) |
| Effort from the current code | n/a | Legacy already does this in seconds (GN4) |
| Pros / cons | Simple / expensive at 48 bots | Affordable / cadences must be stated in seconds |

| Dimension | Timer period as the clock | Monotonic accumulator, bounded catch-up (proposed) | Integer-millisecond period |
|---|---|---|---|
| Complexity | Low | Low, plus an overrun policy | Low |
| Cost | None | None | None |
| Scalability and performance | Millisecond timers cannot hit the period exactly; `Burst` catches up without a bound | Average rate exact whatever the timer granularity | 30 Hz becomes 1000/33 ≈ 30.3 Hz (the research shows the 60 Hz case: 62.5 Hz) |
| Security and anti-cheat | Unbounded catch-up after a stall | A stall drops time and is recorded | Neutral |
| Effect on the look and feel | Rate drift | Steady rate | Rate error |
| Effort from the current code | Small | Small | Small |
| Pros / cons | Simplest / inexact, unbounded | Exact, bounded / needs the policy | Integers / wrong rate: avoided |

| Dimension | W1. Page `requestAnimationFrame` drives the Worker | W2. Worker `setTimeout` + `performance.now()` (proposed) | W3. Worker `Atomics.wait` loop (measured alternative) | W4. Worker `requestAnimationFrame` |
|---|---|---|---|---|
| Complexity | Low | Low | Medium: shared memory | Low |
| Cost | None | None | Cross-origin isolation (ADR 0002's call) | None |
| Scalability and performance | Rate follows the display (60–144 Hz) and stops in background tabs; a page hitch stalls the sim | Independent of the page; timers can fire late under load; hidden-tab behaviour unknown | Precise wake, lowest input latency; inputs must come through shared memory | Same variation and background pause as W1 |
| Security and anti-cheat | Neutral | Neutral | Isolation narrows embeds (R1-BND-OQ5, if adopted) | Neutral |
| Effect on the look and feel | Sim tied to frame rate | Smooth if jitter stays within the mark | Smoothest | As W1 |
| Effort from the current code | n/a | Small | Larger | Small |
| Pros / cons | Display-synced / wrong clock | Simple, independent / timer lateness | Precise / needs isolation | Display-synced / pauses |

### Q3. Identical results (parts 7 to 9)
Source: r1-simulation-core.md, Options, Identical results between Wasm and native, lines 1685–1730. Option E,
lockstep-grade (the netcode depends on bit-identity), is settled by the stack (server authority: STATUS "Decisions
pending", brief §1) and is not weighed.

| Dimension | A. None | B. Same build only | C. Cross-target by discipline (proposed) | D. Fixed point |
|---|---|---|---|---|
| Complexity | Lowest | Low | Medium: rules plus a test suite | High: own physics, own trig |
| Cost | None | None | `libm`'s and parry3d's flag cost unmeasured | Unknown |
| Scalability and performance | Best | Near best | Cost unknown (R1-DET-OQ1) | Unknown |
| Security and anti-cheat | Authority still holds | Authority holds | Authority holds; a hash compared with a client leaves authority-only streams out | Authority holds |
| Effect on the look and feel | More prediction corrections; tests cannot be replayed (legacy TL13 again) | Prediction drifts where std maths differ | Corrections rare; one code path for prediction, local play and tests | Bit-exact, but no ready collision library |
| Effort from the current code | n/a | Low | Medium | Large rewrite of every query |
| Pros / cons | Free / irreproducible | Reproducible servers / drift | Reproducible everywhere, divergence caught / rules to keep | Exact / excludes parry3d and Rapier |

Sub-choices (same source):
- **Transcendental functions (Q2, lines 1695–1703):** std methods (fast to write; "varies by platform, Rust version";
  different code on Wasm and Linux); the pure-Rust `libm` crate (proposed: the same code on both targets, no documented
  accuracy bound, speed unmeasured); nalgebra's `ComplexField` with simba's `libm_force` (what Rapier asks for; ties the
  sim to nalgebra's traits, and parry3d now uses glam: R4-PHYS-F22); our own approximations (full control; the
  accuracy proof is ours). No money cost and no security difference in any; the feel differs only through prediction
  corrections.
- **Random numbers (Q4, lines 1713–1721):** `StdRng` (non-portable: excluded); `rand_chacha`'s ChaCha8Rng (proposed:
  portable, reference-tested, 2^64 streams per seed and a settable position); `rand_pcg`'s Pcg32 (portable, fast on 32-
  and 64-bit, but stream parameters "may be closely correlated", R1-DET-OQ11); legacy `_mulberry32` (tiny, quality not
  researched). Security: any generator whose seed a client holds lets it predict draws, which is why the stream kind
  (Q9) matters more than the generator.
- **Iteration order (Q5, lines 1722–1730):** std `HashMap` iterated (arbitrary, seeded per instance: excluded);
  `HashMap` with a fixed hasher (repeatable for one build, not shown identical on wasm32 and 64-bit native, loses HashDoS
  resistance for client-controlled keys); `BTreeMap` (sorted order, slower lookups, unmeasured); dense arrays or slot
  maps by stable id (proposed, with `BTreeMap` where a map is needed).

### Q4. Physics (part 10)
Source: r4-physics-navigation.md, Options, Rapier 3D or custom, lines 606–617.

| Dimension | A. Rapier, full engine | B. Rapier static world plus its character controller (measured fallback) | C. parry3d plus our own kinematic movement (proposed) | D. Fully custom |
|---|---|---|---|---|
| Complexity | High: solver, joints, islands unused by §6 | Medium | Medium | High: shape-against-shape sweeps |
| Cost | None in money; 12 Rapier releases in about a year with format breaks | The controller's feel can change on upgrade | parry moves as fast, but the surface used is small | No upstream churn |
| Scalability and performance | Largest Wasm and CPU (not measured); sets allocate on clone | Rapier linked; stripping undocumented; autostep "very computationally expensive" | Smaller than A or B (inferred, not measured) | Smallest (inferred) |
| Security and anti-cheat | Determinism needs `enhanced-determinism`; solver SIMD path open | Same flag; no solver runs | parry3d's own flag; the BVH build order is ours | Fully ours |
| Effect on the look and feel | Gives dynamics no §6 item needs; Bullet-style cars do not fit a walker and faction tanks | Autostep, snapping and slopes ready made | Movement feel is ours to tune, as Quake III's is | Same as C, more risk |
| Effort from the current code | Low to start, high to keep deterministic and snapshotted | Low for soldiers; vehicles by us | Medium: the slide-and-step move and the vehicle sweep; legacy GW13 and GW14 are the reference | Highest; ports CS4, CS5 |
| Pros / cons | Everything built in / snapshot and determinism cost | Ready controller / Rapier in the build | Small surface, state in our tables / we write the move | Full control / hardest sweeps |

### Q5. Navigation (part 11)
Source: r4-physics-navigation.md, Options, Navigation meshes in Rust, lines 619–631.

| Dimension | A. Recast/Detour (C++) through bindings | B. Pure-Rust crates at run time (rerecast plus polyanya or landmass) | C. A sim-owned runtime on baked data (proposed) | D. Legacy grid, ported and layered |
|---|---|---|---|---|
| Complexity | Low effort, but blocked for the browser build (no C/C++ toolchain) | Low effort, young 0.x dependencies | Medium: queries and the funnel are ours; generation reuses a crate or D | Low; quality limits known |
| Cost | A second build or a C-capable Wasm target, not weighed by R1 | None | None | None |
| Scalability and performance | Good; sliced queries | Any-angle optimal; costs unknown | Depends on the generator; Quake III shows a coarse area graph suffices | Many nodes; the 8,000-node cap fails (GN12) |
| Security and anti-cheat | C++ floats outside the sim's rules | No determinism promise; a randomly seeded hasher to audit | Ours: integer link costs, id order, the float rules; generation's floats are shared data | Ours |
| Effect on the look and feel | Good paths | Good paths | Bots follow what the bake allows; checked by movement | 2 m cells close gaps (GN11) |
| Effort from the current code | n/a for the browser | Low | Medium; GN11's collision-based build and GN12's allocation-free search carry over | Low |
| Pros / cons | Mature / not in the browser sim | Quick / determinism and agent ownership | Deterministic, owned by the sim / we write the runtime | Known / does not scale |

### Q6. Bots between floors (part 12)
Source: r4-physics-navigation.md, Options, Bots moving between floors, lines 632–641.

| Dimension | A. Links placed by hand | B. Links generated by the bake, checked by the sim's movement, plus overrides (proposed) | C. No links: ramps and stairs only |
|---|---|---|---|
| Complexity | Low code, high editor work | Medium | Lowest |
| Cost | Editor time on every map change | Bake time grows with candidate links | None |
| Scalability and performance | Neutral | Bake cost only | Long routes |
| Security and anti-cheat | Neutral | Neutral | Neutral |
| Effect on the look and feel | A hand link can promise a jump the movement cannot make | Every link is a measured move | Drop-downs "for fast rotations" (LOOP_V2 §9) are lost |
| Effort from the current code | Small | Medium (the reachability check, TL14, moves into the sim) | None |
| Pros / cons | Control / error-prone | Correct by construction / bake time | Simple / fails LOOP_V2 §9 |

### Q7. Line of sight across heights (part 13)
Source: r4-physics-navigation.md, Options, Line of sight across heights, lines 642–650.

| Dimension | A. Rays against statics only (legacy) | B. Rays against statics and selected movers (proposed, by data flag) | C. Precomputed visibility only |
|---|---|---|---|
| Complexity | Low | Low, plus the movers' index (rebuilt each tick anyway) | Bake step |
| Cost | None | None | Bake time |
| Scalability and performance | Bounded by pairs × points | Same, plus movers | Cheapest at run time |
| Security and anti-cheat | Shares the pass that interest uses (ADR 0001) | Same | Coarse per cell; wrong for moving cover |
| Effect on the look and feel | Vehicles never give cover | Big vehicles can be cover if the rule says so | Coarse sight |
| Effort from the current code | Port of CS9 | Port plus flags | New |
| Pros / cons | Proven / no moving cover | Any blocking rule as data / slightly more work | Fast / not enough alone |

### Q8. Obstacles placed mid-match (part 14)
Source: r4-physics-navigation.md, Options, Obstacles placed mid-match, lines 651–662.

| Dimension | A. Carve the mesh (tile rebuilds at run time) | B. Avoidance only | C. Flags on baked data, plus avoidance (proposed) |
|---|---|---|---|
| Complexity | Highest | Lowest | Medium |
| Cost | Rebuilds every tick a vehicle moves | None | None |
| Scalability and performance | Replans after every rebuild (the storm risk) | No replans | Only crossed routes replan |
| Security and anti-cheat | The rebuild must be bit-identical on both targets on the same tick | Avoidance order only | Flag flips are identical everywhere |
| Effect on the look and feel | Correct routes | Bots grind against hulls; destroyed walls never open routes | Routes follow wrecks and opened walls |
| Effort from the current code | Highest | Lowest | Medium |
| Pros / cons | General / costly, nondeterministic risk | Cheap / wrong around structures | Deterministic, fits damage stages / bake every state |

### Q9. The random stream for upgrade offers (part 8)
The two stream kinds are GLOSSARY §4's ([shared stream and authority-only
stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream), lines 525–545); SEC-SPREAD weighed them for
weapon spread; no research weighs them for offers.

| Dimension | Shared stream | Authority-only stream (proposed) |
|---|---|---|
| Complexity | Low | Low |
| Cost | None | None |
| Scalability and performance | Neutral | Neutral |
| Security and anti-cheat | A client holding the seed computes its next offers, and other players' if they share the stream, before the server shows them | The seed never leaves the authority; the client-held hash leaves it out |
| Effect on the look and feel | Offers could appear without a round trip, but nothing predicts them | An offer arrives with the server's message (ADR 0001) |
| Effort from the current code | None (no legacy upgrades) | None |
| Pros / cons | Predictable / leaks the future | Safe / one round trip, already the case |

### Q10. Where the upgrade tracks live (part 18)
- On the soldier entity: settled by UPGRADE-SCOPE ("the tracks live on the PlayerState, not the Pawn"). A record apart
  from the soldier is the only option weighed.

### Q11. Recovery from a trap in the client's Wasm sim (part 20)
Source: r1-simulation-core.md, Open questions, R1-RR-OQ3, lines 2936–2940 (R1 plans no recovery; the reviewer proposes a
restart).

| Dimension | No recovery (a reload) | The Worker restarts and resyncs from the server (proposed) |
|---|---|---|
| Complexity | None | Low: a fresh instance plus ADR 0001's resync |
| Cost | None | None |
| Scalability and performance | Neutral | One resync per trap |
| Security and anti-cheat | Neutral | The server stays the authority |
| Effect on the look and feel | The player leaves the match | A short freeze, then back in step |
| Effort from the current code | None | Small, in the Worker host (ADR 0002) |
| Pros / cons | Nothing to build / a crash ends play | Play continues / local play still needs its own answer (OQ-0004-7) |

## Trade-off analysis

- **Plain data over an ECS crate.** It wins on the three things every later part leans on: a defined iteration order,
  a one-call snapshot into a preallocated copy, and ids the sim owns end to end. The strongest argument against: we
  write the pools, the generational ids, the ordering and any change detection ourselves, and if factions and abilities
  multiply the entity kinds (R1-ECS-OQ7), an ECS's machinery would start to pay for itself. OQ-0004-2 carries the
  trigger: revisit when a new kind cannot be a row in an existing pool.
- **Cross-target identity by discipline, with the netcode not depending on it.** It gives one code path for the server,
  the predicting client, local play and every test, and the suite catches a divergence at the tick it happens. Against
  it: `libm`'s cost and parry3d's flag cost are unmeasured, and a dependency's SIMD path can still differ by target
  (R1-RR-OQ1, R4-PHYS-OQ3). Because the server is the authority, the worst case is more prediction corrections, not a
  broken match; fixed point (D) would cost every query and every library and is kept only as R1-DET-REC7's revisit rule.
- **parry3d plus our own movement over Rapier.** No §6 item needs a dynamics solver, so the physics state is our own
  tables, the snapshot has nothing foreign in it, and reach and step-up are exactly the sim's integrator, which the
  bake's link checks and the editor's reach check reuse. Against it: we write the slide-and-step move and the vehicle
  sweep, parry3d must support every shape pair the sim needs (it returns an error for a pair it does not, R4-PHYS-F20),
  and its BVH's SIMD path is not shown identical across targets. Rapier's controller is measured once beside it, so the
  fallback is a number, not a guess.
- **Baked navigation over a runtime library.** It builds for the browser, makes generation's floats shared data and
  lets structure states flip flags instead of rebuilding tiles. Against it: we write the runtime, neither generator is
  measured on Octopon-3's blocks and plateaus yet, and every damage stage multiplies what the bake covers (the third
  stage is untested: G4). The bake-time mark and the comparison of generators decide it.
- **An authority-only stream for offers.** Nothing predicts an offer, so keeping the stream on the authority costs
  nothing and leaks nothing. The strongest argument against would be a later wish to show offers before the server
  confirms them; that would be a prediction-scope change (ADR 0001), with this stream's kind revisited then.
- **A player record apart from the soldier.** Settled by UPGRADE-SCOPE; the engineering point is that the record is
  bounded by content, allocated at match start and reset in one phase, so it adds no per-tick cost and no client-sized
  allocation.
- **Rate-independent mechanism under a 30 Hz baseline.** Cadences in seconds and durations in sub-tick units mean that
  unlocking 60 Hz under the 60 Hz margin is a rebuild plus a re-bake (links are measured at the shipped tick), not a
  redesign.

## Consequences

- **Easier:** every match can be replayed and compared tick by tick, so tests, balance runs and bug reports reproduce;
  prediction uses the server's own code; one hit world ends the legacy disagreements between shots, sight and blasts;
  the editor's checks run the sim's own code; a destroyed wall or a damage stage changes routes, sight and relevance in
  one tick; upgrade tracks outlive deaths and switches by construction.
- **Harder:** we own the pools, ids, the movement and vehicle sweep, the navigation runtime and the bake; the float rules
  and the hash-map ban are a permanent review load on the sim crate, and every new dependency's maths must be audited
  (FMA, SIMD, hidden hash maps); the bake must cover every state of every changing structure; the bot architecture and
  the moment mechanism have no research behind them yet.
- **Revisit when:** a new entity kind cannot be a row in an existing pool (R1-ECS-OQ7); the sim step share fails at
  N = 48 on the reference server (parallelism inside a match, R1-ECS-OQ4); a map exceeds 2 km across (R4-PHYS-REC5); a
  VISION_V2 §6 item asks for dynamics (R4-PHYS-OQ5); the suite finds a divergence that cannot be removed and corrections
  break VISION_V2 §4's fairness (R1-DET-REC7); ChaCha8's cost shows in the profile (R1-DET-OQ11); a full bake is too slow
  for the editor's Test (R4-OBST-REC3); Mustafa adds player building to VISION_V2 §6 (R4-OBST-OQ3); ABL-PREDICT says
  yes (an ability-proc stream may become shared); the 60 Hz margin unlocks 60 Hz (re-convert durations, re-bake links,
  re-run the parity marks).
- **Hard constraints in CLAUDE.md that change** (for v2 code only, under SPIKE-SCOPE (a), once Mustafa accepts the
  drafts; nothing in CLAUDE.md or `.claude/` changes in this PR):
  - "Classic scripts sharing globals in a fixed load order; the game runs by double-clicking index.html (file://)" does
    not apply to the simulation: it is a Rust crate built natively and to Wasm (the build is ADR 0002's).
  - "Animation is cosmetic: shots come from `playerFireOrigin()` / fixed chest points. Nothing animated decides a hit."
    carries into v2 unchanged as a sim rule (part 8: animation never moves a launch point or a hitbox); only the legacy
    function name stays behind.
  - The legacy code rule "No allocation per frame in hot paths" becomes part 6's heap-free step (a code rule, not a
    hard constraint). The other hard constraints (three.js r128, `LinearEncoding`, `KVStore`, `stylizeScene()`, and "no
    external assets", which LOOK_V2 §8 ends) are not the simulation's.
- **Legacy inventory rows** whose mark this decision would change (the file is not edited; R7-MARKS (a), and the change
  waits for Mustafa's acceptance). Rows whose "rewrite in Rust" mark this decision confirms (CS2, CS4–CS9, GW9,
  GW13–GW15, GC1, GC2, GC5, GC7, GC17, GN4–GN6, GN12, GN13, GP14, GE8, CM11, CM13, CD4, CD6, CD8, CD10–CD14) keep it.

| Row | Mark now | Proposed mark | Why |
|---|---|---|---|
| GN10 (bot verticality: climb steering) | rewrite in Rust | redesign | Typed links found by the bake and checked by the sim's movement replace climb steering (part 12, R4-FLOORS-REC1); collision and gravity stay with the movement rows |
| GN11 (nav grid build) | rewrite in Rust | redesign | Navigation is baked into the map document by a tool, not built at match start; the legacy method (a grid from the movement collision) survives as one of the two candidate generators (part 11, R4-NAV-REC1) |
| GC4 (player weapon state: loadout, switching, reload, spread) | rewrite in Rust | rewrite in Rust (the state machine); loadout: redesign | UPGRADE-SCOPE: a class always comes with its own weapon and other weapons come from drops; carry slots are open (part 18) |
| GC14 (frag grenades and blast damage) | rewrite in Rust, look brief pending | rewrite in Rust; look brief pending lifted | LOOK_V2 §3 and §7 bring fragmentation back; the sim emits the blast event (position, strength) that ADR 0003 draws; whether fragments deal damage per fragment is a design question (decision map e2) |
| GC11, GC12, GC13, GC15 (class abilities, melee, the charged throw and arc preview, bolts) | rewrite in Rust, look brief pending | rewrite in Rust; look brief pending lifted | LOOK_V2 exists; what a player sees of them is ADR 0003's presentation from the sim's events, and the sim half is unchanged (ADR 0003 may list the same rows for their presentation half) |

## Open questions

- **OQ-0004-1. How is bot behaviour structured?**
  - Why it matters: LOOP_V2 §10 calls the bots' role mix "the biggest prototype risk", and the stress scene needs
    "bots with full AI".
  - Options: not researched (no finding on a bot AI architecture: decision map h, G10); the research gives the parts it
    runs on (cadences, the shared sight pass, the navigation queue, the slot model).
  - Decider: a bot research unit, or the slice 2 plan, with the game-designer; Mustafa for the behaviour set.
  - What decides it: LOOP_V2 §10's behaviours kept "simple and observable", run in the slice 2 local server.
  - Research ids: none (gap G10); it cites R2-ROOM-REC6, R1-TICK-REC3, R4-LOS-REC2, R4-NAV-REC2.
- **OQ-0004-2. Will the entity kinds stay few?**
  - Why it matters: part 1 rests on about a dozen known kinds; many new kinds (deployables, zones, mines) would favour an
    ECS.
  - Options: a new pool per kind (part 1); hecs (Q1, option B) if kinds multiply.
  - Decider: the game-designer's roster and ability lists, then this record's revisit.
  - What decides it: the trigger "a new kind cannot be expressed as a row in an existing pool".
  - Research ids: R1-ECS-OQ7 (r1-simulation-core.md, lines 2633–2636); R1-ECS-OQ6 (the unread Overwatch talk) leans on
    the same trigger.
- **OQ-0004-3. What shape is the ground?**
  - Why it matters: every shape pair the sim needs must be supported by parry3d (R4-PHYS-REC1 mark 6).
  - Options: boxes and wedges; a heightfield; a triangle mesh (all parry shapes).
  - Decider: ADR 0005's map format with the editor work (EDITOR_V2 §3: plateaus, ramps, cliff edges; terrain sculpting
    is `[LATER]`), checked by the sim core spike.
  - What decides it: the map format, then R4-PHYS-REC1 mark 6 and R4-PHYS-REC2's spike.
  - Research ids: R4-PHYS-OQ4 (r4-physics-navigation.md, lines 1090–1092).
- **OQ-0004-4. What moves a building through its damage stages?**
  - Why it matters: LOOK_V2 §4 sets the stages and says debris and planks become routes, but not what triggers a stage,
    whether a ruin can be rebuilt, or which planks and debris become which routes; those are rules.
  - Options: for Mustafa to name; the sim carries any answer as data (thresholds per stage on structure health, as
    legacy buildings already have health, GW20; or another trigger).
  - Decider: Mustafa, with the game-designer (rule 6: no rule beyond LOOK_V2 is designed here).
  - What decides it: a design session on LOOK_V2 §4's places, before slice 4 brings structure damage.
  - Research ids: none (gap G4); it cites R4-OBST-REC2.
- **OQ-0004-5. Which weapons drop, and who may take them?** (beyond ADR 0005's carry slots, looted ammo and levelling
  speed)
  - Why it matters: part 18 builds pickups as world entities, but UPGRADE-SCOPE says only "Other weapons come only from
    killed enemies' drops; dropped weapons are world pickups".
  - Options, each a rule for Mustafa: which deaths drop a weapon (any death, or only a death to an enemy); which weapons
    drop (the class's own weapon, looted ones, or both); how long a drop lies and how many may lie on the map; who may
    pick it up (the dead soldier's enemies only, or anyone); how two claims in one tick are broken (fixed order, a
    rotating start, nearest first).
  - Decider: Mustafa with the game-designer; ADR 0005 holds the answers as data.
  - What decides it: the in-match upgrade design session.
  - Research ids: none (gap G13); UPGRADE-SCOPE.
- **OQ-0004-6. What do bots and slot takeovers do with player records?**
  - Why it matters: bots fill slots and humans take over bot slots (R2-ROOM-REC6), so a slot's record can change hands
    mid-match.
  - Options: whether bots level tracks and take picks; whether a human taking over a bot's slot gets that slot's record
    or a fresh one.
  - Decider: Mustafa with the game-designer.
  - What decides it: the in-match upgrade design session.
  - Research ids: none; UPGRADE-SCOPE; cites R2-ROOM-REC6 (r2-netcode.md, lines 2842–2856).
- **OQ-0004-7. How does local play recover when its own authority traps?**
  - Why it matters: in local play the Worker is the authority, so part 20's online recovery (resync from the server)
    has no server to resync from.
  - Options: end the local match with an error report; restore from a snapshot kept outside the trapped instance (not
    researched); replay the recorded inputs from match start, which SLICE-ORDER condition 2 makes possible (not
    researched).
  - Decider: the boundary spike with ADR 0002's client + local server skeleton.
  - What decides it: whether the skeleton keeps a copy of state or inputs outside the sim's instance.
  - Research ids: R1-RR-OQ3, partly (r1-simulation-core.md, lines 2936–2940).

Design questions this record cites and does not own (in `docs/adr/README.md`'s register): who blocks whom (R4-PHYS-OQ1,
R4-NAV-OQ5, R4-OBST-OQ1, R2-XT-OQ13, R4-XT-OQ6; R2-SYNC-OQ5's measurement is ADR 0001's); what a player may see (R2-INT-OQ1, R4-LOS-OQ2); soldiers
and vehicles as blockers of sight and shots, a bot field of view, partial blockers (R4-LOS-OQ1, OQ3, OQ4); stances,
vehicle-driving bots and fall damage (R4-FLOORS-OQ1–OQ3); harvester lanes (R4-OBST-OQ2); player building
(R4-OBST-OQ3); offline play at the largest mode (R1-TICK-OQ7, R1-BND-OQ1, critic rank 15); moment rules and thresholds
(R6-TEL-OQ3); the caps on vehicles and projectiles that replace the stress scene's counts (R2-BUD-OQ5).

