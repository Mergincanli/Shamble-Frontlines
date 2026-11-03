# ADR-0004: The simulation core

**Status:** Proposed
**Date:** 2026-10-31
**Author:** rust-reviewer (the "Pass marks" table: test-engineer, plan step 4) ·
**Reviewed by:** (filled in at review) ·
**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted

This record decides how the game's rules run. There is one Rust simulation: the same code runs natively on the match
server and as WebAssembly in the browser's Web Worker. Its state is plain typed tables in one struct (no ECS crate),
every entity has one id, and time is a whole number of ticks of one constant. Runs are reproducible: the same seed, data
and inputs give the same [state hash](../research/GLOSSARY.md#state-hash) after every tick on every run of a build, and
the same hash on Wasm and native is a target that a test checks on every change. Physics is collision queries plus our
own kinematic movement (parry3d, no physics engine); bot navigation is baked into the map document and queried by the
sim; one height-aware sight query serves bots, blasts, melee and the aim point. The record also says where the player's
upgrade tracks live (a player record apart from the soldier), how dropped weapons exist in the world, and which random
stream draws upgrade offers. It decides no game rule. Why: VISION_V2 §6 asks for a true 3D world, data-driven abilities,
[big vehicles](../research/GLOSSARY.md#big-vehicle-and-very-large-unit), a server-side economy, telemetry and moments,
and SLICE-ORDER asks that movement and shooting be input-driven and replayable from day one.

## Context

**What the decision must serve.**
- VISION_V2 §6.1 (modes as data: the slot tables are sized from the mode), §6.2 (a true 3D simulation: floors and
  interiors, ledges, line of sight across heights, bots between levels, aiming at height), §6.3 (abilities and effects
  from data, executed by the sim), §6.4 (big, multi-part vehicles; seats `[OPEN]`), §6.5 (economy and objectives,
  server-side and tunable), §6.6 (balance telemetry), §6.8 (moments; kill-cams, replays and highlights `[LATER]`) and
  §6.9 (a few [very large units](../research/GLOSSARY.md#big-vehicle-and-very-large-unit): the sim's share is that they are in
  [the stress scene](../research/GLOSSARY.md#the-stress-scene); drawing them is ADR 0003's).
- VISION_V2 §3 ("Bots fill empty slots") and §4 (matches feel fair up to about 100 ms of ping; the minimum laptop runs
  [the largest mode](../research/GLOSSARY.md#the-largest-mode-and-its-n) at 30+ fps).
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
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) of at least 25 %, which ADR 0001 owns and measures
  twice: early, in wave 1's 60 Hz run after the sim core spike (SPIKE-ORDER (b)), and finally at slice 6 with the real
  sim (TICK-60-SETUP, the Advisor, 2026-11-03: "(a), as an early signal only. The final 60 Hz call is measured again
  with the real sim at slice 6."). The full rule is ADR 0001's part 2, with Mustafa's addition (TICK-60-HEADROOM,
  2026-11-03; PM-0001-63): "60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz".
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
- Takes: the tick rate and the 60 Hz margin (ADR 0001); [the prediction scope](../research/GLOSSARY.md#the-prediction-scope), the
  messages and the room lifecycle
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
- Bot navigation does not scale: the 2 m grid can close gaps of 3 to 5.4 m, depending on grid alignment (STATUS,
  "Open bugs and risks"), and sends 27.5 % of objective routes across a roof bots cannot climb (GN11;
  r4-physics-navigation.md, legacy facts, lines 391–393), the route budget resets per
  frame and denies 52–57 % of routes at 48 soldiers (GN13; lines 397–398), and the sight budget gives about 2.5 scans
  per bot per second at 48 soldiers against 4 at 12 (GN4; R4-LOS-REC2, lines 990–1006).
- The legacy JavaScript step took 0.80–1.00 ms mean at 48 soldiers, without physics or a 3D world (GC2;
  r1-simulation-core.md, R1-ECS-REC1 mark 1, lines 1930–1931): a reference, not a prediction (R1-TICK-OQ5, lines
  2658–2660).
- The ADR-001 inputs STATUS logged ("Decisions pending", ADR-001) land here: one hit world for every shooter (K1), a phased
  `update()` (K4), boot-time `Math.random` draws, three.js UUIDs and the camera-dependent effects cull on the shared
  generator, `addTech`'s HUD writes as an event, and a sight budget sized for 12 soldiers. Parts 1, 8, 10, 13, 17 and
  19 of the decision dispose of them; the team allow-list on the server is ADR 0001's, with team filtering in the sim.

## Decision

One Rust simulation crate holds the game's rules and runs them one tick at a time: natively in each room process on the
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
   with their ids; the random streams' state; the tick counter; the tick's event list. Every pool has a fixed capacity,
   set at match start from the mode and the content set and allocated then, so the step never grows one (part 6) and
   nothing a client sends sizes it: each pool's worst case follows from the content's numbers (fire intervals and
   projectile lifetimes, pellets per shot, effect durations and stack limits, drops per death and pickup lifetimes)
   and from the game-designer's caps where they exist (R2-BUD-OQ5), and ADR 0005's whole-set check reports it. A full
   pool applies one fixed, documented overflow rule, identical on every target, and counts every overflow in a
   per-pool counter; which rule each kind gets (refuse the new row, or end the oldest early) decides who wins a
   pile-up, so it is OQ-0004-11's, and until it is answered the spikes refuse the new row as a test fixture, not a
   rule. Phases are plain functions called
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
   (R1-XT-OQ7 is ADR 0001's). The sim crate therefore has two entry points over the same phase functions. The
   authority step runs every phase in the fixed order (each room on the server; the Worker's local server). The
   prediction step advances one slot under ADR 0001's prediction scope (its part 4: own movement, own weapon state
   with reload and active reload, the vehicle the player pilots, and the cosmetic shot events these make): it runs
   only those phases, for that slot and its vehicle, against the static world and the movers as last decoded from the
   server's snapshot, which it reads and never advances; it runs no bots, navigation, economy, objectives, moments or
   other soldiers' weapons, and it draws from no random stream (part 8), so a predicting instance holds no
   authority-only stream's state. The Worker's client endpoint calls it for each replayed and each new own command; a
   phase joins it only when ADR 0001's scope changes (ABL-PREDICT may add the player's own abilities).
   - Why: the client replays its own commands from the server's state on every snapshot, the suite must show that a
     restored run continues byte for byte, and both the replay's agreement with the authority and part 8's stream rule
     rest on one named prediction step.
   - Ids: R1-ECS-REC1 marks 2–4, R1-ECS-REC4, R1-ECS-OQ2, R1-ECS-OQ5 (r1-simulation-core.md, lines 1932–1945,
     1993–2005, 2614–2618, 2627–2628); R4-PHYS-REC2 (r4-physics-navigation.md, lines 721–745); cites R2-SYNC-REC3
     (r2-netcode.md, Recommendations, lines 2068–2105; ADR 0001's) for the prediction step.
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
   mark (W3 needs shared memory, so cross-origin isolation: ADR 0002's call, R1-XU-OQ4). What sets when the next tick
   is due depends on the mode: in local play, the plain accumulator; online, ADR 0001's rule for keeping commands just
   in time (OQ-0001-6), which runs the Worker's tick ahead of the server's on the client endpoint's server-time
   estimate (ADR 0001 part 5). The Worker's step call (ADR 0002 part 5) therefore also returns when its next tick is
   due, and the Worker's host sets its timer from that, never from a count of its own; the sim's own step still takes
   no time (part 4). `requestAnimationFrame` is the
   page's render clock only. After a long gap (a hidden tab, a sleeping laptop) the browser does not simulate the gap:
   in local play the match pauses; online the Worker resyncs from the server's state through ADR 0001's path, with
   catch-up per wake of the Worker bounded at 7 ticks (the same rule, proposed; R1-TICK-REC5 says "per frame", and the
   Worker has no frames). Pause, shop and hit-stop never stop the shared tick
   online; hit-stop is presentation time. The loops read the clock; the step never does (the 11 wall-clock reads STATUS
   lists stay in the frozen legacy game). Whether Worker timers keep their rate in a hidden tab is measured in
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
   [the tick period](../research/GLOSSARY.md#tick-period): 8.33 ms at 30 Hz (4.17 ms in the 60 Hz margin run only). Its
   parts, each reported on its own: movement and physics at most 1 ms native mean, its p99 reported; avoidance at most
   0.2 ms; abilities, effects and weapons at most 1 ms; navigation at most 5 % of the tick period (1.67 ms); the shared
   visibility pass at most 5 % (1.67 ms; R2-INT-REC3's id is ADR 0001's, its number is this record's). Their ceilings
   sum to 5.53 of 8.33 ms, a planning check, not a bound ([time statistics](../research/GLOSSARY.md#time-statistics):
   p99s do not add); the rest of the sim (projectiles, economy, objectives, bot decisions, moments, events) takes what is
   left, 2.80 ms (8.33 − 5.53, arithmetic). At 60 Hz the same parts sum to 3.87 of 4.17 ms. ADR 0001 adds this share to
   the server-only share inside the server tick budget (seam S1). In the browser, the client's sim share is a p99 sim
   step time in Wasm in the Worker on the minimum laptop of at most 40 % of the tick period: 13.33 ms at 30 Hz (6.67 ms
   at 60 Hz), with Wasm at most 2 × native. It applies wherever the browser runs the whole sim: the local server of
   slices 1 and 2 (SLICE-ORDER), at an N that R1-TICK-OQ7 sets (a design question for Mustafa and the Advisor, in
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
   lockstep, no fixed point. Inside the sim: f32 for every real-valued quantity (up to a map 2 km across,
   R4-PHYS-REC5), while time, counts and money are integers (parts 4 and 17). The float allow-list: `+ − × ÷` and
   negation; the comparisons and `total_cmp`; `sqrt`, `floor`, `ceil`, `round`, `trunc`, `fract` and explicit
   `mul_add`, which std specifies exactly (R1-DET-F10, r1-simulation-core.md, Findings, lines 655–660); `abs` (std
   1.99.1's `f32` page: "This function always returns the precise result") and `copysign` (proposed: a sign-bit
   operation; std warns only of a NaN sign, which this part's no-NaN rule excludes); `as` casts (R1-DET-F15, lines
   684–692); and the exact non-arithmetic methods the rules themselves use, such as `is_finite`, `is_nan`, `to_bits` and
   `from_bits`. Every other std `f32` and `f64` method is banned, and ADR 0002's lint is this list's complement. Among
   them are `powi`, `hypot`, `cbrt`, `to_degrees` and `to_radians`, which std 1.99.1 marks non-deterministic as it does
   the transcendental functions (R1-DET-F9, lines 648–654; rust-reviewer read the note on all five at the step-10
   review), and `min`, `max` and `clamp`, which may return either zero for equal signed zeros, while `clamp` panics on
   bad bounds (R1-DET-F14, lines 679–683). In their place the sim has three helpers of its own, written as plain
   comparisons (`if a < b { a } else { b }` and the like): on equal inputs, signed zeros included, each returns one
   defined argument, the same on every target, and none panics (a clamp whose bounds are the wrong way round returns a
   bound); a degree value becomes radians by one multiply with a pinned constant. `rem_euclid`, `div_euclid` and the
   float `%` operator are not on the list. std's pages promise both methods "the rounded infinite-precision result"
   (1.99.1's, read by the verifier on 2026-11-02, and 1.97.0's), yet 1.97.0's and the pinned 1.99.1's `div_euclid` give
   3, not 2, for (3 + 2^-22) ÷ (1 + 2^-23) in `f32`, as do their `f64` analogues (verifiers fixcheck-d3a and skeptic-6,
   and for 1.99.1 fixcheck-d3e, 2026-11-03). A shared error passes a same-bits check, all determinism needs; correct
   rounding is a separate claim, so both may join only once fixed inputs, those cases and their sign mirrors included,
   equal a rounded exact reference on every target (PM-0004-22, extended); no source read states `%`'s precision (the
   Rust Reference defines it only by truncating division and the dividend's sign), and review, not the lint, catches
   the operator. Transcendental maths only through the pure-Rust `libm` crate, pinned; no std `sin`, `cos`, `atan2`,
   `powf` or `exp`, no `algebraic_*`, no fast-math, no `relaxed-simd`; Wasm `simd128` off until the suite passes
   with it on. NaN never enters sim state: every input and data value is checked finite where it enters, and debug
   and test builds assert finite state each tick. No sim value is computed in TypeScript. Fused multiply-add across
   targets: Rust never fuses on its own, so the risk is a dependency's SIMD path (wide's `mul_add` fuses only where
   the target has FMA); the server therefore builds for baseline x86-64, never with `+fma` or `-C target-cpu=native`,
   glam's `scalar-math` is on if the sim uses glam itself, and native AArch64 joins the native-against-Wasm hash
   test before ADR 0006 may pick ARM hosts (the build flags are ADR 0002's). An ARM load generator, such as the netcode
   proof test's (ADR 0001's LG), is not an ARM host in this sense, since it serves no player and runs no room (the
   advisor's reading, 2026-11-03), and under ADR 0001's rule for the rows that compare an LG client's result with the
   server's, no verdict of those rows rests on native AArch64 agreeing with x86-64 or Wasm. parry3d's own
   `enhanced-determinism` flag is on for both targets (critic rank 3). Attribute maths (ADR 0005's data model) runs in
   f32 under these rules with a fixed, documented aggregation order per attribute (R1-ABL-OQ5). An exactly representable
   `dt` does not matter for identity: `dt` is one constant with the same bits on every target, and timers count integer
   units (part 4); whether 64 Hz is worth it is R1-TICK-OQ1, ADR 0001's (R1-XU-OQ11). Whether browsers run a different
   Wasm float profile matters only if NaN can enter state, which these rules forbid; the suite's browser runs check it
   (R1-DET-OQ4).
   - Why: basic operations, `sqrt` and `mul_add` are exactly specified on every target while std's transcendental
     functions are not, and neither are `powi`, `hypot`, `cbrt`, `to_degrees` and `to_radians`; std's `min`, `max` and
     `clamp` leave signed zero open, which an allow-list and three helpers close where a deny-list of transcendental
     names would not; NaN bits and relaxed SIMD differ by engine; with server authority a divergence costs a
     correction, not a desync.
   - Ids: R1-DET-REC1, R1-DET-REC2, R1-DET-REC3, R1-DET-REC7, R1-RR-OQ1, R1-ABL-OQ5, R1-XU-OQ11, R1-DET-OQ3,
     R1-DET-OQ4 (r1-simulation-core.md, lines 2129–2183, 2242–2248, 2923–2932, 2770–2771, 2899–2902, 2686–2690);
     R4-PHYS-REC5 (r4-physics-navigation.md, lines 786–798).
   - Need: VISION_V2 §6.2, §6.3, §6.4; §4 (corrections stay rare at about 100 ms of ping).
8. **Random streams.** The sim draws only from `rand_chacha`'s `ChaCha8Rng`, pinned, seeded by the authority per match,
   one stream per purpose. Each match's seed comes from the operating system's secure random source on the server
   (in local play, the Worker's `crypto.getRandomValues`): the host draws it and passes it in at match start, so the sim
   reads no random source itself. The seed is never sent to a client, never logged and never written into ADR 0006's
   match record; outside the live state only the recorded match (I-16) keeps it, and no recording reaches a client
   while its match runs (proposed: the sim crate, and with it the generator, ships in every client's Wasm, so a seed a
   client could derive, from the match id, the room or the start time, would let it compute the authority-only
   streams: the spread SEC-SPREAD (a) keeps on the server, and the offers). The sim maps raw generator output with its
   own few sampling functions and pinned test
   vectors, never rand's distribution functions (R1-DET-OQ5: their outputs carry no stability statement across
   versions, and uniform floats "may equal `high`"). The streams' state (seed, stream, position) is part of the
   authority's state and of the state hash. The kind of each stream
   ([shared stream and authority-only stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream)): a
   stream is authority-only unless ADR 0001's prediction scope makes the client's predicting sim draw from it. Today's
   scope (R2-SYNC-REC3) draws from none, so every stream starts authority-only: match setup, weapon spread (SEC-SPREAD
   (a)), bot decisions, ability procs (shared only if ABL-PREDICT says yes and an own ability draws a proc), and the
   upgrade offers (part 18). Offers are authority-only for SEC-SPREAD's reason: a seed the client holds lets a
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
   R1-RR-OQ6), with every integer, every length and every index at a fixed width and little-endian, and no `usize`
   in the serialized or hashed bytes (proposed: `usize` is 32 bits on wasm32 and 64 bits on x86-64, so a length
   written as `usize`, or a hasher fed through `std::hash::Hasher`, whose derived `Hash` writes slice lengths with
   `write_usize`, would differ by target at tick 0; PM-0004-21 checks it). It hashes those bytes as one byte string
   with a fixed, unseeded hash function: the state hash (which function is picked in the sim
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
    - Why: C++ Detour cannot simply be linked into the sim's Wasm build (`wasm32-unknown-unknown` has no C/C++
      toolchain, R4-NAV-F8, and R4-NAV-REC1 reads linking it as not a supported path), no Rust navigation crate read
      promises determinism (R4-NAV-F16), baking once makes generation's floats shared data, and Quake III shipped bots
      on a precomputed area graph.
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
    research bakes two states, the second of which may be a ruin's shape (R4-OBST-REC2), so the third stage is an
    extension the sim core spike
    must prove (no research finding covers it). What moves a building from one stage to the next is a game rule for
    Mustafa (OQ-0004-4); the stages and their assets are content (ADR 0005); drawing them is ADR 0003's. Roofs that fade
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
    mix; whether bots also take world pickups, which UPGRADE-SCOPE added after that list, is OQ-0004-12); no research
    covers a bot AI architecture, so its structure is OQ-0004-1. Bots act on the present while humans
    see about 100 ms of the past (R2-SYNC-OQ13): bot reaction is tuning data the game-designer sets, starting from
    legacy's 0.25 s scan cadence (GN4), and the sim core spike reports bot reaction times at N = 48. A field of view for
    bots is a design question (R4-LOS-OQ3).
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
    for ABL-PREDICT; the sim code is the same either way. Definitions are read by id (I-1). In development they change
    at a match boundary under ADR 0005's CD-7 (proposed); if OQ-0005-CD4 chooses its option (b), a reload swaps them
    between ticks, never inside one (R1-DATA-REC5).
    - Why: the closed set of operations lets combos come from data while the sim stays reviewable and deterministic.
    - Ids: R1-ABL-REC6 (part 6); cites R1-ABL-REC1, R1-ABL-REC2 (r1-simulation-core.md, lines 2342–2400).
    - Need: VISION_V2 §6.3, §6.4; active reload: LOOP_V2 §7.
17. **Spawning, economy and objectives as sim phases.** Money and harvester deliveries, team-funded rebuilds, uplinks
    (cache, carry, place, countdown, steal, rotation), tech tiers, power-plant pricing and the points race run as sim
    phases in the fixed order at the cadences of part 4, with their numbers in ADR 0005's tables and their rules
    exactly as LOOP_V2 §3, §4 and §8 state them. Money is an integer, never a float (proposed: whole dollars, as every
    LOOP_V2 §3 and §5 amount is, so a split gives the same shares on every target), and it lives on the player record
    (part 18), since a death keeps it (LOOP_V2 §5); a rebuild's equal shares (LOOP_V2 §3) are integer divisions whose
    rounding and remainder rule are data in ADR 0005's tables, applied in a fixed slot order so every target gives the
    same shares. A steal is holding E for a few seconds (LOOP_V2 §4 step 4), one channel for bots and humans alike,
    since bots send the same commands (part 15); GE8's presence count (every team's live members in the uplink's
    circle) is kept as a primitive the steal may read. LOOP_V2 does not say whether a defender inside the circle pauses
    or blocks a steal, or what happens when two enemy teams steal one uplink at once, so the presence rule is a data
    switch whose setting is OQ-0004-8's; whether a carried cache's position is public is OQ-0004-9's (its visibility
    class then ADR 0001's). Spawning runs as LOOP_V2 §2 and §5 state it: everyone spawns inside their own barracks, in
    front of the buy terminal; spawn protection lasts 3 s (`[DEFAULT]`; an effect in ADR 0005's data); a death
    respawns the slot as the Recruit (`[DEFAULT]`); a purchase is a command the authority checks (ADR 0001). "Enemies
    cannot enter a barracks interior" (`[DEFAULT]`) is a team mask on a collider's movement flag (part 13): colliders
    across a barracks' openings block every soldier but its own team's, and the bake gives the nodes behind them that
    team's flag (part 11), so no other team's route runs through them. Team filtering (who a shot or blast can hurt,
    GC1) is sim code. What each team may see of another team's state is ADR 0001's visibility classes. A tier reached
    or money paid is an event (part 19), never a HUD write from the sim (legacy's `addTech`).
    - Why: LOOP_V2's economy is server-side and tunable, and its outcomes must replay like everything else; integer
      money keeps a split identical on every target; one team mask that movement and the bake both read keeps bots and
      humans under one rule.
    - Ids: none of its own (no research finding on economy phases or spawning); cites R1-TICK-REC3 (cadences).
    - Need: VISION_V2 §6.5; spawning, spawn protection, the Recruit respawn and the barracks' team mask: LOOP_V2 §2
      and §5 only.
18. **The player record, weapon drops and pickups (UPGRADE-SCOPE).** Each player slot has a player record, the
    PlayerState, in a table beside the soldier tables and apart from the soldier entity, the Pawn; it is keyed by slot,
    and whether a player who rejoins after ADR 0001's grace window gets it back is OQ-0004-10 (the key follows the
    answer). For the match it holds the player's money (part 17) and one track per unit type and one per weapon type
    (level, picks taken, and its open offers: a bounded queue whose length, like the offer's size, is data, so no
    number is hard-coded; when offers appear, how they wait and what a full queue does are OQ-0005-AB8's), keyed by the
    type ids ADR 0005 defines; what a pick grants is applied through the ability system. The record survives the
    soldier's death and every unit switch: a new soldier entity reads its slot's record. A weapon type's track stays in
    the record while the player holds no gun of that type and applies again with the next gun of that type. The record's
    size is bounded by the content's type counts and allocated at match start; nothing a client sends sizes it. At match
    start the sim builds every slot's record afresh (every track empty, money at LOOP_V2 §3's starting $300), with the
    soldier tables, so nothing carries over from a previous
    match (UPGRADE-SCOPE; LOOP_V2 §8); ADR 0001's room lifecycle calls this phase. A class's own weapon comes from the
    unit definition (ADR 0005). When a soldier dies, the weapons the drop rule names (OQ-0005-AB9) become pickup entities
    in the pickup pool, each with an id (part 2), the weapon type id, a position settled on the ground with the movement
    query, and a lifetime in ticks from ADR
    0005's data. A pickup claim is an input command (ADR 0001's field) handled in the phase's fixed order, so two claims
    on one pickup in one tick always resolve the same way on every machine. When a track levels, the authority draws an
    offer (UPGRADE-SCOPE's one of three, its size from data) from the authority-only offers stream (part 8) into that
    track's queue; the server refuses a pick it did not offer and a pickup it did not place (ADR 0001). Picks, offers
    shown and weapons looted are tick-stamped events (part 19), from which time per weapon type follows, and ADR 0006's
    record needs them with their ticks. A full pickup pool follows part 1's overflow rule (OQ-0004-11). The rules stay
    open: carry slots, ammo for looted weapons and how fast tracks level (ADR 0005's open questions); which deaths drop
    which weapons, how long a drop lies, who may pick it up and which order breaks a tie (OQ-0005-AB9); what bots and
    slot takeovers do with records (OQ-0005-AB10); and whether a late rejoiner gets their record back (OQ-0004-10).
    - Why: Mustafa put the tracks on the player, not the unit; a record apart from the soldier is the only place that
      outlives a death and a switch, and pickups as ordinary entities get ids, snapshots and hashes for free.
    - Ids: none (no research finding on upgrades).
    - Need: UPGRADE-SCOPE (a settled input, listed in `docs/adr/README.md`).
19. **Events, moments and telemetry.** Each tick the sim appends tick-stamped events to the tick's event list: hits,
    deaths and kills (killer and victim slot and unit id, the source weapon or ability id, assisting slots), blasts
    (position and strength, for the concussion shake and the fragments ADR 0003 draws), cosmetic shot events,
    purchases, uplink cache events, harvester deliveries and kills, structure kills and stage changes, tech tiers,
    upgrade picks (slot, track, level, choice) and pickups (slot, weapon type). Every event carries its tick, so the
    match record can place each pick and pickup on the timeline beside the kills, not only count them: OQ-0005-AB3's
    two measurements (the time to kill between a fully levelled and a fresh player, and how long the way back takes)
    need each player's levels at each kill (VISION_V2 §6.6: "Snowball and counters get tuned on evidence"). The event
    list is part of the state and its hash; ADR 0002 places it in the output block,
    ADR 0001 delivers it online with its visibility classes, the room writes the match record at Ended (ADR 0001), and
    ADR 0006 owns the record's shape. A moment phase runs after the outcome phases: it reads the tick's events and the
    state and emits moment events by rules held as data (ADR 0005's format; their thresholds are a design question,
    R6-TEL-OQ3), so the moments and the telemetry read the same stream. ADR 0006's record samples the timeline every 10 s
    of game time (R6-TEL-REC1); the sim supplies the samples at that cadence (part 4).
    No research covers a moment mechanism, so this part stays at VISION_V2 §6.8's level.
    - Why: every §6.6 number and §6.8 moment is a function of the events and the timeline (R6-TEL-REC1).
    - Ids: none of its own; cites R6-TEL-REC1 (r6-infrastructure.md, lines 784–811) and R1-TICK-REC3.
    - Need: VISION_V2 §6.6, §6.8.
20. **No panic from input or content, and recovery from a trap.** No panic is reachable from network input or content
    data: the sim takes only commands that ADR 0001's decoders validated and content that ADR 0005's loader validated
    (its loader returns errors and is fuzzed with zero panics); inside the sim, values a client or content can set never
    reach an `unwrap` or an unchecked index, and std's `clamp`, which panics on bad bounds (R1-DET-F14), is never
    called: part 7's clamp helper returns a bound instead. On the server a panic in the tick ends that room's match and
    its process (each room is its own
    process, R2-ROOM-REC2). The room keeps `panic = "unwind"`, so that a panic in one connection's work closes only that
    connection (R2-ROOM-REC2's fix pass); but an uncaught panic ends only its own thread (R2-ROOM-F8), so a tick thread
    left to die would leave a room that still holds its connections and sends nothing. The room therefore catches a
    panic at the top of its tick loop (`catch_unwind`, R2-ROOM-F8) and treats it as fatal: it reports it (ADR 0006),
    tells its clients the match ended where it can (ADR 0001's room lifecycle), and exits non-zero, so the supervisor
    records the exit and the VM's other rooms run on. The tick thread and the network runtime share only bounded
    channels, never a lock the tick unwraps (a lock poisoned by a panicking connection would turn that panic into the
    room's). Whether a room may instead resume from a snapshot was the Advisor's question in ADR 0001 (OQ-0001-18); the
    Advisor answered it on 2026-11-03: "end the match, as proposed, and keep the crash and match records", so a room
    never resumes. In the browser `-Cpanic=abort` ends the sim's instance, and the trap alone carries no panic message,
    only `unreachable` and frames; so the Wasm build installs a small panic hook of its own (`std::panic::set_hook`, no
    extra crate) that writes the message and location into a fixed buffer in linear memory before the abort, and the
    Worker's host reads it after the trap and attaches it to the report (proposed: a native scratch build on rustc
    1.97.0 ran such a hook under `panic = "abort"` and kept the message; the Wasm build is S20's to check). Online,
    after a trap, the Worker's host starts a fresh instance and rejoins through ADR 0001's resync path, and the trap is
    reported (R6-OBS-REC3, ADR 0006); the panic strategy is ADR 0002's. In local play the trapped instance was the
    authority; recovery there is not researched (OQ-0004-7).
    - Why: a panic on the server is a denial of service for a whole room, and in the browser it ends the match for that
      player; a room that outlives its tick, or a trap without its message, hides the fault.
    - Ids: R1-RR-OQ3, R1-XT-OQ12, R2-XT-OQ12, R6-XT-OQ6 (r1-simulation-core.md, lines 2936–2940, 3084–3087;
      r2-netcode.md, lines 3395–3398; r6-infrastructure.md, lines 1021–1024); cites R2-ROOM-REC2 and R2-ROOM-F8
      (r2-netcode.md, lines 2762–2768, 1459–1467).
    - Need: VISION_V2 §4 only (play in four browsers; matches that feel fair).
21. **Map validation with the sim's own integrator.** The editor's reach value is the sim's movement run at the shipped
    `dt`, never a closed-form formula; the reachability check (every spawn to every goal) and the report of gaps under
    6 m run inside the sim crate, from Node for the editor and from CI, with identical results on native and Wasm.
    - Why: integration error depends on the step, and legacy's formula overstates the real reach (GW14).
    - Ids: R1-TICK-REC6 (r1-simulation-core.md, lines 2118–2125); R4-NAV-REC1 marks 3 and 4 (r4-physics-navigation.md,
      lines 835–837).
    - Need: VISION_V2 §6.2; EDITOR_V2 §3 (the validation panel) and LOOP_V2 §9 (6 m gaps, reachability).

## Options considered

Every option below comes from the research or a settled input, cited per question (the Options sections for Q1–Q8,
GLOSSARY §4 for Q9, UPGRADE-SCOPE for Q10, R1-RR-OQ3 for Q11). Each table carries template 0000's
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

| Dimension | Everything every tick | Cadences counted in ticks, staggered by slot (proposed) |
|---|---|---|
| Complexity | Lowest | Low: one cadence per system, stated in game time |
| Cost | None | None |
| Scalability and performance | AI, sight and route costs scale with the tick rate | The sim step time grows only with what must be fine-grained |
| Security and anti-cheat | Neutral | Neutral |
| Effect on the look and feel | None | None if cadences are in game time (bots behave the same at any rate) |
| Effort from the current code | n/a | Legacy already does this in seconds (GN4) |
| Pros / cons | Simple / expensive at N = 48 | Affordable / cadences must be stated in seconds |

| Dimension | Timer period as the clock | Monotonic accumulator, bounded catch-up (proposed) | Integer-millisecond period |
|---|---|---|---|
| Complexity | Low | Low, plus an overrun policy | Low |
| Cost | None | None | None |
| Scalability and performance | Millisecond timers cannot hit the tick period exactly; `Burst` catches up without a bound | Average rate exact whatever the timer granularity | 30 Hz becomes 1000/33 ≈ 30.3 Hz (the research shows the 60 Hz case: 62.5 Hz) |
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
  portable, reference-tested, 2^64 streams per seed and a settable position); `rand_pcg`'s Pcg32 (portable, "a good
  choice on both 32-bit and 64-bit CPUs", but stream parameters "may be closely correlated", R1-DET-OQ11); legacy
  `_mulberry32` (tiny, quality not
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
  stage is untested). The bake-time mark and the comparison of generators decide it.
- **An authority-only stream for offers.** Nothing predicts an offer, so keeping the stream on the authority costs
  nothing and leaks nothing. The strongest argument against would be a later wish to show offers before the server
  confirms them; that would be a prediction-scope change (ADR 0001), with this stream's kind revisited then.
- **A player record apart from the soldier.** Settled by UPGRADE-SCOPE; the engineering point is that the record is
  bounded by content, allocated at match start and reset in one phase, so it adds nothing to the sim step time and no
  client-sized allocation.
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
  yes (an ability-proc stream may become shared); the tick moves to 60 Hz at slice 6's call (ADR 0001 part 2: the 60 Hz
  margin, and the goal budget holding at the rooms per VM measured at 60 Hz, PM-0001-63; re-convert durations, re-bake
  links, re-run the parity marks).
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
| GC14 (frag grenades and blast damage) | rewrite in Rust, look brief pending | rewrite in Rust; look brief pending lifted | LOOK_V2 §3 and §7 bring fragmentation back; the sim emits the blast event (position, strength) that ADR 0003 draws; whether fragments deal damage per fragment is a design question for Mustafa |
| GC11, GC12, GC13, GC15 (class abilities, melee, the charged throw and arc preview, bolts) | rewrite in Rust, look brief pending | rewrite in Rust; look brief pending lifted | LOOK_V2 exists; what a player sees of them is ADR 0003's presentation from the sim's events, and the sim half is unchanged (ADR 0003 may list the same rows for their presentation half) |
| GN2 (bot roster record and unit assignment, line 267) | redesign, look brief pending | redesign; look brief pending stays, for the head tint only | The roster becomes ADR 0001's slot table (human, bot or empty) over the fixed-slot soldier table, and the record splits: its sim state (health, timers, AI state, target, unit) lives in the sim's tables at the bot's slot index (parts 1 and 2), its view state (`group`, `_aimT`, `_fOn`) on the page; a bot gets its unit as a human does, by buying a T1 unit or respawning as the Recruit (LOOP_V2 §5, §10), since it sends the same commands (part 15), and a random pick draws from part 8's bot decisions stream, never `Math.random`; the sim half shows a player nothing; the head tint (the unit's accent) is the presentation half, ADR 0003's or ADR 0005's |
| GE4 (team tech meter and eras, line 290) | redesign, look brief pending | redesign; look brief pending stays, for the era HUD and toasts only | LOOP_V2 §4's team tech and tiers run as a sim phase on the per-team tables (parts 1 and 17), their numbers in ADR 0005's tables and the minute-5 floor counted in ticks (part 4); a tier reached is an event (part 19), never a HUD write from the sim (part 17), which ends the debt the row cites (`applyDamage` running `addTech`'s HUD writes); what a team sees of another team's tech is ADR 0001's visibility classes; the x0.5 power-plant tech cut is not in LOOP_V2, so part 17 does not build it, and whether a dead plant slows tech stays Mustafa's question (rows GE4, GE5); the era HUD and toasts are the presentation half, ADR 0003's (part 10) |
| GE9 (tech boxes, line 295) | redesign, look brief pending | redesign; look brief pending lifted | LOOP_V2 §4 `[DEFAULT]` replaces tech boxes with uplinks, a sim phase (part 17: cache, carry, place, countdown, steal by holding E (LOOP_V2 §4 step 4), with GE8's presence count kept as a primitive whose part in a steal is OQ-0004-8, rotation) whose cache events part 19 lists; the two lessons the row names are met: if the next site is drawn at random, the draw comes from a seeded sim stream, never `Math.random` (part 8), and "can't shoot while carrying", and whatever the steal channel blocks, binds bots as it binds humans, since bots send the same commands (part 15); the note is lifted because the box's mesh, world bar and HUD go with the box while that `[DEFAULT]` stands, not because LOOK_V2 sets them; the uplink's world marker (GE7, line 293) and its HUD are the presentation half, ADR 0005's and ADR 0003's |
| GE10 (silos, line 296) | redesign, look brief pending | redesign; look brief pending lifted | LOOP_V2 §4 `[DEFAULT]` removes silos, so no silo rule reaches the sim; the row keeps the presence count (GE8, a primitive part 17's steal may read, OQ-0004-8) and hands the "income never zero" role to the safe near-base field (LOOP_V2 §3), a money rule of part 17; the note is lifted because the pads and the HUD's silo dots and rate go with the silos while that `[DEFAULT]` stands, not because LOOK_V2 sets them |

## Open questions

- **OQ-0004-1. How is bot behaviour structured?**
  - Why it matters: LOOP_V2 §10 calls the bots' role mix "the biggest prototype risk", and the stress scene needs
    "bots with full AI".
  - Options: not researched (no finding on a bot AI architecture); the research gives the parts it
    runs on (cadences, the shared sight pass, the navigation queue, the slot model).
  - Decider: a bot research unit, or the slice 2 plan, with the game-designer; Mustafa for the behaviour set.
  - What decides it: LOOP_V2 §10's behaviours kept "simple and observable", run in the slice 2 local server.
  - Research ids: none (a research gap); it cites R2-ROOM-REC6, R1-TICK-REC3, R4-LOS-REC2, R4-NAV-REC2.
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
  - Research ids: none (a research gap); it cites R4-OBST-REC2.
- **OQ-0004-5. Which weapons drop, and who may take them?** Moved: one question, in ADR 0005, OQ-0005-AB9
  (UPGRADE-SCOPE's data model is ADR 0005's). This record keeps only the mechanism: two claims on one pickup in one
  tick resolve in the phase's fixed, deterministic order (part 18); which order is fair, which deaths drop which
  weapons, how long a drop lies and who may take it (the killer's team, the dead soldier's enemies, or anyone) are
  OQ-0005-AB9's.
- **OQ-0004-6. What do bots and slot takeovers do with player records?** Moved: one question, in ADR 0005,
  OQ-0005-AB10. How a bot picks, if bots pick, is bot behaviour (part 15).
- **OQ-0004-7. How does local play recover when its own authority traps?**
  - Why it matters: in local play the Worker is the authority, so part 20's online recovery (resync from the server)
    has no server to resync from.
  - Options: end the local match with an error report; restore from a snapshot kept outside the trapped instance (not
    researched); replay the recorded inputs from match start, which SLICE-ORDER condition 2 makes possible (not
    researched).
  - Decider: the boundary spike with ADR 0002's client + local server skeleton.
  - What decides it: whether the skeleton keeps a copy of state or inputs outside the sim's instance.
  - Research ids: R1-RR-OQ3, partly (r1-simulation-core.md, lines 2936–2940).
- **OQ-0004-8. How does an uplink steal meet defenders, and two enemy teams?**
  - Why it matters: LOOP_V2 §4 step 4 (`[DECIDED]`) says only that enemies steal an uplink by holding E for a few
    seconds. What defenders standing on it do, and who wins when two enemy teams try at once (three-team modes only),
    decide how safe a lead is (VISION_V2 §2, pillars 1 and 2). Legacy's contest rules belong to the tech boxes and silos
    that LOOP_V2 §4 replaces (`docs/LEGACY_INVENTORY.md`, row GE8).
  - Options: (a) holding E only, presence ignored (LOOP_V2's words as written); (b) a defender inside the circle pauses
    the steal; (c) a defender blocks it and its progress decays (legacy's tech-box rule, GE8). For two enemy teams: the
    first to finish takes the uplink, or each pauses the other. Part 17 builds the presence rule as a data switch, so
    each option is data, not code.
  - Decider: Mustafa, with the game-designer.
  - What decides it: the uplinks change-set (LOOP_V2 §11, step 5), from sims of (a) and (b), 3 runs each in the
    three-team mode, reading steals and late steals among LOOP_V2 §11's objective counters.
  - Research ids: none (a design question).
- **OQ-0004-9. Is a carried tech cache's position public?**
  - Why it matters: LOOP_V2 §4 step 1 tells everyone where a cache spawns, not whether its carrier stays visible. A
    carrier hidden like any soldier ends the chase that makes steals possible; a public carrier, slow and unable to
    shoot (step 2), may rarely reach an uplink.
  - Options: (a) public while carried; (b) public at spawn and at placement only.
  - Decider: Mustafa.
  - What decides it: a design session, or the uplinks change-set's sims (cashouts and steals per match), with the carry
    speed as tuning in ADR 0005's tables; the answer sets the carried cache's visibility class in ADR 0001's state
    table.
  - Research ids: none (a design question); either answer is one of R2-INT-REC5's classes (r2-netcode.md, line 2502;
    ADR 0001's).
- **OQ-0004-10. Does a player who rejoins after the grace window get their player record back?**
  - Why it matters: UPGRADE-SCOPE says the tracks are owned by the player, but part 18 keys the record by slot. A
    player who returns after ADR 0001's reconnect grace window joins as a new player, with no claim on the old slot's
    record, which the bot that took the slot over now holds, its tracks and its money (R2-ROOM-REC4: "after the window
    the slot follows the bot-fill rule").
  - Options: (a) the record stays with the slot (part 18 as written); (b) the room keeps a departed player's record by
    player id until the match ends and restores it on rejoin, the slot's bot starting a record of its own; its cost is
    one record per departed player, in a table whose capacity is data set at match start, so no client can grow it
    (part 18's rule).
  - Decider: Mustafa, with OQ-0005-AB10 (what a human gets on taking a bot's slot).
  - What decides it: a design session with OQ-0005-AB10; the record's key (slot or player id) follows the answer.
  - Research ids: none (a design question); it cites R2-ROOM-REC4 (r2-netcode.md, lines 2812–2815; ADR 0001's).
- **OQ-0004-11. When a pool is full, which row gives way?**
  - Why it matters: a step with no heap allocation in steady state (part 6) needs a fixed capacity per pool (part 1).
    When a burst fills one (live projectiles, active effects, weapon pickups), its rule decides who wins a pile-up:
    whose grenade, whose slow, whose dropped gun gives way. The rule must be the same on every target.
  - Options: (a) the new row is refused and counted (nothing that already exists changes); (b) the oldest row, by
    start tick and then sequence number (part 4), ends early. Each kind may take its own: active effects beside ADR
    0005 AB-1's stacking policy, pickups beside OQ-0005-AB9's lifetime.
  - Decider: the game-designer proposes per kind; Mustafa confirms where a player could notice.
  - What decides it: each pool's worst case from the content set (ADR 0005's whole-set check) and the game-designer's
    caps (R2-BUD-OQ5), after which the stress scene's counts gain pickups; until it is answered the spikes use (a) as
    a test fixture, not a rule.
  - Research ids: none (no research sizes a pool); it cites R1-ABL-REC6 (r1-simulation-core.md, lines 2465–2480) and
    R2-BUD-OQ5 (r2-netcode.md, lines 2982–2985).
- **OQ-0004-12. Do bots take world pickups?**
  - Why it matters: bots fill most slots in the start modes (VISION_V2 §3), and LOOP_V2 §10's behaviour list was
    written before UPGRADE-SCOPE added loot; looting bots change what the drop rules do to balance.
  - Options: (a) no, bots keep their class weapons; (b) yes, by the human rules.
  - Decider: Mustafa.
  - What decides it: the slice 2 plan's behaviour set (OQ-0004-1), revisited with OQ-0005-AB9 and AB10.
  - Research ids: none (a design question).

Design questions this record cites and does not own (in `docs/adr/README.md`'s register): who blocks whom (R4-PHYS-OQ1,
R4-NAV-OQ5, R4-OBST-OQ1, R2-XT-OQ13, R4-XT-OQ6; R2-SYNC-OQ5's measurement is ADR 0001's); what a player may see
(R2-INT-OQ1, R4-LOS-OQ2); soldiers and vehicles as blockers of sight and shots, a bot field of view, partial blockers
(R4-LOS-OQ1, OQ3, OQ4); stances, vehicle-driving bots and fall damage (R4-FLOORS-OQ1–OQ3); harvester lanes
(R4-OBST-OQ2); player building (R4-OBST-OQ3); offline play at the largest mode (R1-TICK-OQ7, R1-BND-OQ1, critic rank
15); moment rules and thresholds (R6-TEL-OQ3); the caps on vehicles and projectiles that replace the stress scene's
counts (R2-BUD-OQ5). Also asked by ADR 0003 and listed once in `docs/adr/README.md`'s "Decisions the drafts ask for":
whether grenade fragments deal damage per fragment (LOOK_V2 §3 and §7 bring fragmentation back as a visible effect,
which ADR 0003 draws from the blast event; Mustafa with the game-designer; no research id).

## What a spike must prove

Most claims are for one throwaway spike, called here **the sim core spike** (working name; `docs/SPIKE_PLAN.md` gives
the final names): the sim on one vCPU of the reference server and in Wasm on the minimum laptop, in the stress scene at
N = 48 (three teams, 16v16v16: 15 big vehicles, 4 of them very large units, 300 live projectiles, up to 8 active
effects per soldier, one DM_Tri-sized 3D greybox; GLOSSARY.md, §3, lines 397–437), at 30 Hz, with the 60 Hz form run
where wave 1's 60 Hz margin run needs it (ADR 0001 owns that run and how the sim's load reaches it: OQ-0001-2 (a), the
sim core spike's sim with S1's stand-ins). That run is "an early signal only" (TICK-60-SETUP, the Advisor, 2026-11-03);
the final 60 Hz call is measured again with the real sim at slice 6 (migration phase 8). Three claims belong to other
spikes, named in the table. Every mark below is the research's (cited; all proposed there),
restated for 30 Hz where the research wrote the 60 Hz form, or marked "proposed" with its reason. No claim here needs a
person's judgement; the netcode proof test's "look correct" is ADR 0001's.

| # | Claim | Part | Candidate marks (source) | Spike |
|---|---|---|---|---|
| S1 | The sim fits the sim step share at N = 48 | 6 | p99 sim step time ≤ 8.33 ms at 30 Hz (≤ 4.17 ms for the 60 Hz margin run); mean, p50, p95, p99 and max reported per N at N = 12, 24, 36, 48 and the N = 64 [headroom](../research/GLOSSARY.md#the-design-point-and-headroom) run; 36,000 ticks per run (20 minutes of game time at 30 Hz, 10 at 60 Hz: arithmetic), 3 runs; each part of the share reported on its own against its ceiling (R1-TICK-REC2 P1, r1-simulation-core.md lines 2044–2053; R1-ECS-REC1 mark 1, lines 1923–1931; GLOSSARY.md lines 53–69); the sim core spike builds no ability and effect step (S15 follows it) and no economy, objectives, moments or events (slice 4), so its run stands in for each: 384 rows in the effect pool (so the snapshot, the hash and memory carry them) with the effect step's 1 ms ceiling burned per tick, and the 2.80 ms remainder topped up per tick after the built projectiles and bot decisions, each stand-in named in the run's record and scaled by N / 48 at the other scale steps (proposed: a pass with the stand-ins still holds once the real parts meet their ceilings; a run that fails only through them is reported so, and the share's verdict then waits for S15's step); the built parts are judged against their own ceilings either way; S1 on the real crate is migration phase 8's; the 60 Hz margin run's load is ADR 0001's: in wave 1's run, an early signal only, it is this sim with these stand-ins (OQ-0001-2 (a), TICK-60-SETUP, the Advisor, 2026-11-03), so at 60 Hz the same two stand-ins are 1 ms and 0.30 ms (4.17 − 3.87, arithmetic); at slice 6 the run is on the real crate (migration phase 8) | the sim core spike |
| S2 | The browser runs the whole sim within the client's sim share | 6 | p99 sim step time ≤ 13.33 ms at 30 Hz in Wasm in a dedicated Worker on the minimum laptop while the page holds 30+ fps on Low for the whole run; Wasm ≤ 2 × native (R1-TICK-REC2 P2, lines 2054–2057; R1-ECS-REC1 mark 1); N for the pass from R1-TICK-OQ7 (a design question), the scale steps reported meanwhile; the wave-1 run lacks the same parts as S1's, and the client's sim share has no per-part split to burn, so it reports the built sim's time and its pass waits for S15's step as well | the sim core spike's sim in the boundary spike's Worker |
| S3 | Cadences keep the sim step time flat from tick to tick | 4 | p99 sim step time ≤ 2 × mean at 30 Hz; sight checks per game second per bot at 60 Hz within 1 % of the 30 Hz count, the 60 Hz form only in the 60 Hz margin's test (R1-TICK-REC3, lines 2074–2076) | the sim core spike |
| S4 | One time representation holds every rate | 4 | with held fire for 60 s of game time, every legacy `fireInterval` and `burstInterval` gives a shot count within 1 shot of 60 / interval at 30 Hz, and the same count within 1 shot at 60 Hz, the 60 Hz form only in the 60 Hz margin's test (R1-TICK-REC1, lines 2023–2024), which also meets R1-ABL-REC3's 1 % for the Shabzon (1 shot in 833 is 0.12 %: arithmetic); CD13's Warrior-kills-Conscript time of 0.86 s within one tick (R1-ABL-REC3, lines 2416–2419); a CI check finds no clock read in the step (no `std::time`, no host time import in the Wasm module: R1-TICK-REC1, line 2027); the units-per-tick figure chosen from the rounding report (proposed: the research gives none) | the sim core spike |
| S5 | Snapshot and restore are cheap and exact | 3 | 0 heap allocations per snapshot after warm-up (a counting global allocator); ≤ 0.2 ms native and ≤ 0.5 ms Wasm at p99 in the stress scene (R1-ECS-REC1 mark 2, lines 1932–1937); a run snapshotted, restored and continued gives the same per-tick state hash as an uninterrupted run over a 10-minute bot match (18,000 ticks at 30 Hz: arithmetic), and serialize → deserialize → serialize is byte-identical (mark 4, lines 1942–1945); whole-world restore plus k = ⌈0.15 s × 30⌉ = 5 ticks of replay within 8 ms p99 in Wasm on the minimum laptop only if ADR 0001's prediction scope keeps it (mark 3, lines 1938–1941) | the sim core spike |
| S6 | Ids are stable and never stale | 2 | ids survive snapshot and restore unchanged; a despawned id never resolves after its slot is reused (a property test over random spawn and despawn sequences, 10,000 cases); a server snapshot applied on the client maps every id with no lookup table beyond the pools (R1-ECS-REC3, lines 1985–1989) | the sim core spike |
| S7 | Same build and cross-target identity | 7, 9 | the same seed and input log, run twice on the same build, give the same state hash at every tick for 36,000 ticks (R1-TICK-REC1, lines 2025–2026); bots-only 24v24 (N = 48) and 12v12v12 (N = 36), 10 minutes each (18,000 ticks at 30 Hz), 3 seeds each: 0 mismatching per-tick state hashes between two native runs in separate processes, and 0 against the Wasm runs (R1-DET-REC1, lines 2139–2144; the largest mode is the 24v24 goal under MODES-3 (a), so no third match); the same with the two native processes' std `HashMap` seeds different (R1-ECS-REC2 mark 1, lines 1966–1971); with parry3d's `enhanced-determinism` on, the stress scene gives 0 mismatches between native and Wasm over 10 minutes, and parry3d's cost with the flag on and off is reported with method (R1-DET-REC3, lines 2178–2182, applied to parry3d); at least 10,000 fixed inputs per `libm` function the sim uses give bit-identical outputs on native x86-64, native AArch64 if ADR 0006 picks it, and Wasm in each browser engine; a build check fails on a std transcendental call, `algebraic_*` or `relaxed-simd`; `libm` against std cost reported (R1-DET-REC2, lines 2165–2168); the suite runs these matches with 0 mismatches on every target, catches each injected divergence at the first tick it changes the state, and reports its hashing cost per tick (R1-DET-REC6, lines 2238–2240); the same suite with `simd128` on decides R1-DET-OQ2; `libm`'s accuracy for the aim and sight functions checked against reference values (R1-DET-OQ8, lines 2704–2705) and Wasm's `sqrt` against IEEE in the unit vectors (R1-DET-OQ10, lines 2710–2712) | the sim core spike |
| S8 | Streams replay exactly and stay where they belong | 8 | the first 10,000 outputs of each stream are identical on native and Wasm; a stream saved mid-match and restored continues identically; muting sound, moving the camera or changing the display frame rate changes no state hash in a recorded match (R1-DET-REC4, lines 2205–2207); the client's predicting state holds 0 bytes of any authority-only stream's state (proposed: SEC-SPREAD (a) and part 8 make this the stream rule's test); ChaCha8's cost per tick reported, which decides whether R1-DET-OQ11 comes back (lines 2713–2717) | the sim core spike |
| S9 | parry3d plus our own movement is enough | 10 | the six marks of R4-PHYS-REC1 (r4-physics-navigation.md, lines 692–717): movement parity (a 0.55 m step climbed and 0.56 m stopped; the jump apex within 1 mm of the integrated value; 0 tunnelling through a 0.2 m wall in 10,000 random moves at about 13.3 m/s, which is 0.44 m per tick at 30 Hz); movement plus all physics queries ≤ 1 ms native mean with p99 reported, Wasm ≤ 2 × native; 0 per-tick hash mismatches native against Wasm and a byte-identical static BVH; the sim .wasm size with parry3d, with rapier3d and with neither, after wasm-opt and gzip (also R1-ECS-REC1 mark 5, lines 1946–1949, against a bevy_ecs 0.20 stub, and a one-line incremental native rebuild ≤ 10 s); Rapier's controller measured once as the fallback; 0 `Err(Unsupported)` shape pairs. Also R4-PHYS-REC2 (lines 739–743: 20,000-ray parity with the legacy SIM, 0 of 1,200 bot shots more than 1 cm above a ramp, byte-identical queries after a restore, static BVH build ≤ 50 ms native), REC3 (lines 758–760: 0 tunnelling at the fastest projectile speed at 30 Hz (the 60 Hz form only in the 60 Hz margin's test), grenades bounce back in 1,000 of 1,000 throws, identical landing points), REC4 (lines 779–784: 0 ticks with a soldier or wall more than 1 cm inside a vehicle hull over 10 minutes with 48 soldiers) and REC5 (lines 796–798: the marks hold at the far corner of the largest planned map; distance per second within 0.1 %) | the sim core spike |
| S10 | Baked navigation works and is affordable | 11 | R4-NAV-REC1's six marks (lines 830–841: bake ≤ 1 s native and ≤ 3 s in a browser Worker for each generator; ≥ 99.9 % coverage and 0 unstandable nodes; a 6 m gap stays a route; reachability in the sim crate identical on native and Wasm; 10,000 path queries byte-identical; the generator comparison); R4-NAV-REC2 (lines 859–865: navigation ≤ 5 % of the tick period at p99, 1.67 ms at 30 Hz, at N = 48 and the N = 64 headroom run with a carried uplink cache; no bot waits more than 1 s for a route; ≤ 2 replans per bot per second at p99; a stall rate no worse than legacy's 7.73 %); R4-NAV-REC3 (lines 880–884: 48 bots in a 6 m choke for 10 minutes with 0 deadlocks over 3 s, identical hashes, avoidance ≤ 0.2 ms p99, 0 avoidance pairs across floors); R4-NAV-REC4 (lines 896–897: a Worker build without bots skips the navigation data, its size reported; with bots, local play at the largest mode meets S2) | the sim core spike |
| S11 | Floors and links are correct by construction | 12 | R4-FLOORS-REC1 (lines 918–923: 0 generated jump links above the integrated reach; each kept link succeeds in 1,000 of 1,000 traversals at 30 Hz (the 60 Hz form only in the 60 Hz margin's test); 0 % of routes need an unreachable surface; drop-downs taken when shorter in ≥ 95 % of 200 trials); R4-FLOORS-REC2 (lines 936–939: separate layer and area ids per floor, no walking through ceilings in 100 of 100 trials, the barracks roof not walkable); R4-FLOORS-REC3 (lines 951–952: a changed `jumpV` without a re-bake is reported stale, 1 of 1) | the sim core spike |
| S12 | One sight query serves every user | 13 | R4-LOS-REC1 (lines 975–986: 20,000-pair parity with legacy `hasLOS`; 0 of 10,000 pairs see through floors; 0 of 10,000 ramp disagreements; the cost of one segment query and of the full pass at 24v24 and 16v16v16 within its 5 % share, 1.67 ms at 30 Hz; identical hashes); R4-LOS-REC2 (lines 1002–1006: every bot scans at least 4 times per second within the same share); R4-LOS-REC3 (lines 1015–1016: a downward cursor ray onto a ramp lands within 1 cm in 1,000 of 1,000 samples; R2-SYNC-REC8 mark 1 is ADR 0001's) | the sim core spike |
| S13 | Changing structures and damage stages flip in one tick | 14 | R4-OBST-REC1 (lines 1037–1041: over 10 minutes at 30 Hz (the 60 Hz form only in the 60 Hz margin's test): 0 bots stuck over 3 s against a vehicle, ≤ 2 replans per bot per second at p99, identical hashes); R4-OBST-REC2 (lines 1058–1061: route, sight and portal change in the same tick with 0 ticks of disagreement over 1,000 destroy and rebuild cycles; 1,000 traversals after a flip; a flip ≤ 0.1 ms native at p99); the same marks with three stages per structure on the two-floor test building, including a plank link that exists only in the ruin stage, and the bake with every stage still within S10's bake-time mark (proposed: LOOK_V2 §4 names three stages and the research tested two) | the sim core spike |
| S14 | Bots run the stress scene | 15 | the spike's "bots with full AI" is a stated behaviour set (proposed, since no research defines it: the legacy GN rows' combat behaviour ported, meaning target scans, combat movement, gunfire and route following, plus R4-NAV-REC2's carried uplink cache as a moving shared goal); bot reaction times at N = 48 reported (R2-SYNC-OQ13, r2-netcode.md lines 2952–2954; no mark) | the sim core spike |
| S15 | The ability, effect and weapon step fits its part | 16 | ≤ 1 ms p99 per tick in the stress scene with 384 active effects, on one vCPU of the reference server and on the minimum laptop in Wasm, 0 heap allocations in steady state (R1-ABL-REC6, r1-simulation-core.md lines 2465–2480); a 120 s scripted scenario with every test ability, 3 seeds, gives byte-identical logs on native and Wasm (R1-ABL-REC3, lines 2416–2419) | the content and ability spike (after the sim core spike) |
| S16 | The player record behaves as UPGRADE-SCOPE says | 18 | no research mark; proposed (UPGRADE-SCOPE's sentences as checks): over scripted deaths and unit switches, 0 track entries lost or changed; a weapon type's track resumes when the next gun of that type is picked up; every record's tracks are empty and its money at the starting amount at the start of every match; pickups keep their ids through snapshot and restore; two claims on one pickup in one tick resolve the same way on native and Wasm (the state hash). test-engineer sets the numbers | the content and ability spike |
| S17 | The server loop keeps time | 5 | on the reference server (Linux), tick start jitter p99 ≤ 2 ms over 36,000 ticks (20 minutes at 30 Hz); after an injected 200 ms stall, back on schedule within 1 s and never more than 7 ticks in one wake (proposed restatement of the research's 15 at 60 Hz); overrun and dropped-time counters in the metrics (R1-TICK-REC4, r1-simulation-core.md lines 2090–2092) | the netcode proof test (ADR 0001) |
| S18 | The Worker paces the tick in every browser | 5 | on Chrome, Edge, Firefox and Safari: foreground tick start jitter p99 ≤ 4 ms over 10 minutes, read online against the schedule the server-time estimate sets (part 5); per-frame displacement of a constant-speed unit with a standard deviation ≤ 5 % of its mean on 60, 120 and 144 Hz displays; after 60 s hidden, local play ran no tick and resumes where it stopped, and online the client is back in step within 0.5 s and never runs more than 7 ticks in one wake of the Worker (proposed restatement of 15 at 60 Hz; the research's "per frame") (R1-TICK-REC5, lines 2109–2114); the same after more than 5 minutes hidden (R1-BND-OQ2, lines 2724–2727) | the boundary spike (ADR 0002) |
| S19 | The editor's reach is the sim's reach | 21 | the editor's reach value equals the measured jump apex plus step-up within 1 mm at the shipped tick for `T.jumpV` 11, `T.gravity` 24, `T.stepUp` 0.55 (R1-TICK-REC6, lines 2124–2125) | the sim core spike |
| S20 | No panic from input or content; recovery online | 20 | zero panics over the fuzzed content corpus (ADR 0005's mark, R1-RR-OQ3) and over fuzzed commands fed to the sim (proposed: part 20's rule needs its own test, the decoders being ADR 0001's); a trap injected into the client's Worker ends in a fresh instance back in step through ADR 0001's resync mark, and the trap reaches error reporting with the panic message and location from part 20's hook (R6-OBS-REC3; the hook proposed in part 20) | the sim core spike with the boundary spike |
| S21 | Throws land the same on Wasm and native | 7, 10 | 10,000 random throws: the trajectory function gives bit-identical points on Wasm and native (R2-SYNC-REC6 mark 1, r2-netcode.md line 2218; the id is ADR 0001's, and the shared trajectory function runs in the sim crate) | the sim core spike |
| S22 | One feature set builds the sim on every target | 7 | the resolved feature list of every dependency of `sim` (`cargo tree -e features`) is identical for the `server` build and the `sim-wasm` build, and a planted extra feature on a shared dependency in `server` fails the check (R5-LAYOUT-REC4, r5-toolchain-quality.md lines 1216–1218; the id is ADR 0002's, and the foundation re-proves it as a gate check) | the sim core spike |

### Pass marks
Written by test-engineer (plan step 4). One row per mark, grouped by the spike that runs it: the sim core spike
(PM-0004-1 to PM-0004-61, then PM-0004-73 and PM-0004-74, added at the fix round), the content and ability spike
(PM-0004-62 to PM-0004-66, then PM-0004-75, added at the final round), the netcode proof test (PM-0004-67), the
boundary spike (PM-0004-68 to PM-0004-70), and
the 60 Hz set (PM-0004-71, PM-0004-72), which runs only with the 60 Hz margin's test. The second column names the
claim (S1–S22 above) and the decision part. Every number is
a research mark (cited), arithmetic on research numbers (shown), a settled input, or marked proposed with its reason.
Line numbers are `r1-simulation-core.md`'s at 9e7db28 unless another file is named (r2 `r2-netcode.md`, r3
`r3-rendering.md`, r4 `r4-physics-navigation.md`, r5 `r5-toolchain-quality.md`, r6 `r6-infrastructure.md`, GL
`GLOSSARY.md`).

**How to read the table**
- **Machines.** **RS**, [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of
  [the reference server](../research/GLOSSARY.md#the-reference-server): a Hetzner CCX23 (4 dedicated vCPU, 16 GB,
  x86-64 AMD), Linux, one EU location (R6-HOST-REC2, r6 lines 645–650; GL lines 225–230; a working pick while
  R6-HOST-OQ1 is open), with no other room on that vCPU (GL lines 244–250); the bench process is pinned to one vCPU and
  the VM is otherwise idle (proposed: a neighbour on the same VM moves a p99; the pinning tool is the harness's to
  document). **CX33**, the shared-vCPU CX33, measured beside RS where a row says so (R6-HOST-REC2): reported, never
  gated. **ML**, the minimum laptop, and **RD**, the recommended desktop: ADR 0003's machines (R3-BUDGET-REC1, r3 lines
  1096–1103; named and recorded by PM-0003-1). Each ML run records the browser and its version, `hardwareConcurrency`
  and the core count (R1-BND-OQ3, lines 2728–2731), mains power and the operating system's power mode (proposed, as in
  ADR 0003). On ML the Wasm marks run in Chrome and Firefox on Windows 11, R3-BUDGET-REC1's two browsers there
  (proposed: the sim's speed differs by engine), and Edge is reported. **MAC**, SPIKE_PLAN's macOS machine: the stable
  Safari on the day of the run, its version recorded (WebGPU from Safari 26.0, R3-WGPU-F19, r3 lines 171–174;
  WebTransport from 26.4, R2-TRN-F7, r2 lines 959–975), for the Safari runs (real Safari through its WebDriver:
  R5-TEST-REC2, r5 lines 1415–1418); its model recorded; never a budget
  result. **CI**, the gate v2's runners (ADR 0002): counts, hashes and checks only, never a time (R5-TEST-REC1's
  benchmark row, r5 line 1400). **A64**, a native AArch64 machine, only before ADR 0006 may pick ARM hosts (part 7;
  R1-RR-OQ1, lines 2923–2932). No mark here is judged on Mustafa's machine.
- **Scenes.** **SS**, [the stress scene](../research/GLOSSARY.md#the-stress-scene) (GL lines 397–420): N = 48 bots in
  three teams (16v16v16, MODES-3 (a)) in one combat cluster; 15
  [big vehicles](../research/GLOSSARY.md#big-vehicle-and-very-large-unit), harvesters included, 4 of them very large
  units of at least 20 parts; 300 [live projectiles](../research/GLOSSARY.md#live-projectile); up to 8
  [active effects](../research/GLOSSARY.md#active-effect-and-visual-effect) per soldier (384); one DM_Tri-sized 3D
  greybox. Its scale steps are N = 12, 24, 36, 48 and the N = 64
  [headroom](../research/GLOSSARY.md#the-design-point-and-headroom) run (labelled headroom), with 15 big vehicles and 300
  live projectiles at every step and 8 active effects per soldier (proposed reading: GL §3 counts only the active
  effects per soldier). The spike drives SS from a recording (the seed, the map and mode data, the content data's hash
  and the per-tick inputs: part 9), so every run of one recording does the same work. So that the load cannot thin
  (proposed: a lighter scene than the one named would pass a budget falsely), a dead soldier respawns inside the
  cluster on the next tick, live projectiles and active effects are topped up to their counts at the start of every
  tick, and PM-0004-2 checks the counts. **O3**, the Octopon-3 greybox (LOOP_V2 §9: plateau, ramps, drop-downs, 6 m
  gaps). **TB**, a two-floor test building with stairs, a window and an open stairwell (R4-FLOORS-REC2 (a), r4 lines
  936–937; R4-LOS-REC1 mark 2, r4 lines 978–979), with three damage stages for PM-0004-57. **LB**, a test block with
  ledges at 2.5, 2.9, 3.0 and 3.1 m and platforms at 1.2, 2.5 and 4.0 m with ramps (R4-FLOORS-REC1, r4 lines 918–920;
  R4-PHYS-REC1 mark 1, r4 line 693). **BR**, box-only scenes for the legacy parity fuzzes (legacy CS6's method).
  **M10**, a bots-only match of 10 minutes: 18,000 ticks at 30 Hz (600 × 30, arithmetic).
- **T, a timed run set** (R1-TICK-REC2 P1, lines 2044–2053): 3 runs of one recording, each 900 warm-up ticks (30 s of
  game time; proposed: caches fill and first-time allocations happen early, and ADR 0003's harness also drops a
  warm-up) then 36,000 measured ticks (20 minutes of game time at 30 Hz: 36,000 ÷ 30 = 1,200 s). Each run is labelled
  with the build's commit, the machine, the operating system and kernel, the compiler version and the build flags.
  Every one of the 3 runs must meet the mark (proposed: a mark met only on an average of runs hides a failing run; ADR
  0003 reads R3-BUDGET-REC2 the same way).
- **Statistics.** Percentiles by nearest rank over a run's measured ticks (⌈q × n⌉): of 36,000 ticks, p99 is the
  35,640th smallest value, p95 the 34,200th, the median (p50) the 18,000th; of 18,000 ticks, the 17,820th, 17,100th
  and 9,000th (arithmetic; proposed: one definition, as ADR 0003's). A time mark that names no statistic is a p99
  ([time statistics](../research/GLOSSARY.md#time-statistics)); p99s are never added, so the sim step time is timed
  around the whole step, never summed from its phases. A one-off event (a bake, a BVH build, a rebuild) has a plain
  limit that every occurrence meets (GL lines 146–147). Count and hash marks are exact.
- **V, the validity of a timed run set:** its 3 run medians lie within 3 % of each other (R3-MEASURE-REC1's mark, r3
  lines 1192–1193, applied to every timed set as ADR 0003 does: proposed). An invalid set is neither a pass nor a fail:
  its cause is found and written down before it is measured again.
- **Timers.** The tick bench (ADR 0002 part 8: a mode of the `devtools` replay runner; mean, p50, p95, p99 and max per
  phase and in total; 3 runs per build) reads the host's monotonic clock outside the sim, which reads none (part 4;
  PM-0004-10). Phase times come from a bench entry that calls the step's phases one by one, the same functions in the
  same order; the step's total is timed around the ordinary step call, in its own runs (proposed: a timer import inside
  the module would fail PM-0004-10's check, and the total must be measured directly). The bench reports its timer's
  resolution as the smallest non-zero difference of two reads (proposed). In a Worker, `performance.now()` has 5 µs
  resolution with [the isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag) on and 100 µs
  without (R3-MEASURE-F7, r3 lines 641–643): a Wasm mark under 1 ms runs on a build with the flag on, or passes only
  with 0.1 ms to spare (proposed: one timer step, as ADR 0003).
- **Native and Wasm.** "Wasm ≤ 2 × native" is read on one machine: the native bench build and the Wasm build in a
  dedicated Worker, both on ML under the same operating system, p99 against p99 per run pair (proposed: across RS and
  ML the ratio would measure the two CPUs, not Wasm). Server budgets run natively on RS; client budgets run in Wasm on
  ML. The marks gate at 8.33, 13.33 and 1.67 ms: 25 %, 40 % and 5 % of the tick period, 33.33 ms (GL lines 82–83;
  arithmetic). Every mark states a share at two decimals, the form of the 60 Hz margin's ruling (8.33, 6.25); prose
  and the GLOSSARY's sums may round to one.
- **Tick.** Every mark runs at 30 Hz (TICK-30). The research's 60 Hz forms (R1-TICK-REC2 P1 and P2 at 60 Hz, and the
  "at 30 and 60 Hz" halves of S3, S4, S9, S11 and S13) form the 60 Hz set (PM-0004-71, PM-0004-72), which runs only
  with [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin)'s test, as TICK-60-HEADROOM rules (plan; GL §8;
  confirmed by Mustafa, 2026-11-03): the sim's share there is read from the same run, never summed from parts, in each
  of ADR 0001's two 60 Hz runs: wave 1's early signal on the sim core spike's sim with S1's stand-ins (OQ-0001-2 (a),
  answered by the Advisor, 2026-11-03: "as an early signal only"), which decides nothing, and the run at slice 6 on the
  real crate, which decides (the advisor's reading, 2026-11-03). No 60 Hz result is needed for the 30 Hz pass.
- **Network.** A mark that needs a network names an R2 profile (round trip, jitter, loss: P0 0 ms; P1 50 ms, ±10 ms,
  1 %; P2 100 ms, ±20 ms, 2 %; P3 150 ms, ±30 ms, 5 %; P4 200 ms, ±40 ms, 5 %; B1 = P2 plus one 1 s burst of 100 %
  loss: r2 lines 1987–1990), injected by ADR 0002's network simulator (R5-TEST-REC4, r5 lines 1454–1460). P2 is
  VISION_V2 §4's "about 100 ms of ping" (decision map J6 (3); OQ-0001-3, answered by the Advisor, 2026-11-03).
- **Bites.** Each automated mark names a planted defect that must make it fail on the spike's own build before its
  pass counts, because "a check without a failing case can't be shown to work" (R5-GATE-REC3, r5 lines 1727–1728);
  the planted defects are proposed unless cited. **Rule A**, for an allocation mark: one planted 64-byte heap
  allocation per tick in the path under test must be counted (proposed: a counter proves "zero" only if it can see
  one). **Rule B**, for a time mark: a loop of integer work, calibrated outside the sim to last the mark's measured
  margin plus 1 ms and reading no clock, added to the phase under test on every tick (once, for a one-off event), must
  fail it (adding a constant to every tick raises every percentile by that constant).
- **Tests that join the gate** (the determinism set, unit vectors, property tests, lints) give 20 identical verdicts
  in 20 runs on one commit (R5-TEST-REC6, r5 lines 1503–1504), with one fixed `PROPTEST_RNG_SEED` in the gate and fresh
  seeds nightly (r5 lines 1492–1494). A failing run is a defect to find, never retried into green.

| mark | what it proves (decision part) | number and statistic | hardware | method and tool | runs and noise | basis |
|---|---|---|---|---|---|---|
| **The sim core spike** | | | | | | |
| PM-0004-1 | The machines exist, and every result names its machine (a precondition of every spike here) | RS, CX33 and MAC recorded (model, CPU as the operating system reports it, cores, RAM, operating system); ML and RD as PM-0003-1 names them; the session's build machine for PM-0004-33 | RS, CX33, ML, RD, MAC | a results-log header per machine, written before the first run; every later log row names its machine and build | checked before the first run of each spike that uses the machine | R6-HOST-REC2 (r6 lines 645–650); R3-BUDGET-REC1's mark (r3 lines 1114–1116); R1-TICK-OQ6 (lines 2662–2666) |
| PM-0004-2 | The stress scene carries its whole load, so no budget passes on a lighter scene (S1, S14; parts 6, 15) | on every measured tick: N soldiers alive in the one cluster (48, or the scale step), 15 big vehicles with the 4 very large units of at least 20 parts, 300 live projectiles, 8 active effects per soldier (384 at N = 48; in the sim core spike these are S1's 384 stand-in rows in the effect pool, which count as the stress scene's effects, with the effect step's 1 ms ceiling burned and the 2.80 ms remainder topped up on every tick, scaled by N / 48 at the other scale steps, and at 60 Hz 1 ms and 0.30 ms; the run's record names each stand-in and its per-tick log shows each ran); every bot logs at least one target scan, one shot and one route followed in every minute of game time, and the carried uplink cache moves (S14's bot set: legacy GN rows' target scans, combat movement, gunfire and route following, plus the cache; the per-minute floor proposed: an idle bot lightens the scene) | the counts are machine-free: read on every T run, on RS and ML | the bench's per-tick count log and the bots' action log | every T run; a run that fails it is invalid, not failed. Bites: a build where dead soldiers stay dead fails it within its first minute; a run with the effect step's stand-in burn switched off fails it on its first tick | GL §3 (lines 397–420); the draft's S14 (proposed there); R4-NAV-REC2 (r4 lines 859–861: the carried cache); R1-ECS-REC1 mark 1 (lines 1923–1926) |
| PM-0004-3 | The sim fits the sim step share at the largest mode (S1; part 6) | p99 [sim step time](../research/GLOSSARY.md#sim-step-time) ≤ 8.33 ms (25 % of 33.33 ms) at N = 48; mean, p50, p95, p99 and max reported, each part in its own row (PM-0004-31, -45, -46, -52, -62) and the rest of the sim reported beside them | RS | T on SS at N = 48, parry3d's `enhanced-determinism` on (part 10); the tick bench times the whole step | T, V; rule B (in the movement phase) | R1-TICK-REC2 P1 (lines 2044–2050); R1-ECS-REC1 mark 1 (lines 1923–1929); GL lines 53–60, 189–196 |
| PM-0004-4 | How the cost grows with N, and on the cheaper host (S1; part 1's one thread per match; decides R1-ECS-OQ4 and R1-TICK-OQ5) | reported, no pass number: mean, p50, p95, p99 and max at N = 12, 24, 36, 48 and the N = 64 headroom run, each part beside the total; the same at N = 48 on CX33; legacy's JavaScript step (0.80–1.00 ms mean at 48 soldiers) beside them as a reference, not a prediction. Passes when every cell holds a number. Parallelism inside a match is weighed only if PM-0004-3 fails | RS; CX33 | T on SS at each N | T, V per N | R1-TICK-REC2 P1 (lines 2044–2050); R1-ECS-REC1 mark 1 (lines 1930–1931); R1-ECS-OQ4 (lines 2622–2626); R1-TICK-OQ5 (lines 2658–2660); R6-HOST-REC2 (CX33 beside) |
| PM-0004-5 | Cadences keep the step time flat from tick to tick (S3; part 4) | p99 sim step time ≤ 2 × the mean sim step time at N = 48, both from the same run | RS | PM-0004-3's runs | T, V. Bites: the stagger by slot removed, so every bot's scan and replan land on one tick in 30 (above the 1 % of ticks the p99 reads) | R1-TICK-REC3 (lines 2074–2076) |
| PM-0004-6 | The step allocates nothing in steady state, even past a pool's capacity (parts 1, 6) | 0 heap allocations in total over the measured ticks of a run at N = 48; and in a burst of 30 ticks (one second) that asks the projectile and effect pools for twice their capacity (at least twice the stress scene's counts, 600 live projectiles and 768 active effects: arithmetic; proposed, so every overflow path runs): 0 heap allocations, per-tick state hashes equal on native and Wasm, and each pool's overflow counter equal on both targets and equal to the rows its rule turned away (part 1's test fixture, refusing the new row, until OQ-0004-11 is answered) | RS (native); ML: Chrome (Wasm); CI (the burst's hashes and counters: native, Wasm in Node) | a counting global allocator in the bench build, read before and after the measured ticks and around the burst; the burst scripted at a fixed tick of SS's recording | 1 run per target (the count repeats: PM-0004-18). Rule A; bites: a pool that grows its buffer when full (a `Vec::push` past the capacity) must be counted in the burst | the draft's part 6 (proposed there from R1-ABL-REC6's rule, lines 2471–2472); part 1 (capacities and the overflow counter); the burst proposed (the Rust review's finding 4) |
| PM-0004-7 | Wasm costs at most twice native (S1, S2; part 6) | p99 sim step time in Wasm in a dedicated Worker ≤ 2 × the native p99 at N = 48, on the same machine | ML: the native bench build, and Wasm in Chrome and in Firefox, under one operating system (recorded) | T on SS with each build; the ratio per run pair (run i against run i) | T, V on both sides; all 3 pairs ≤ 2. Bites: rule B on the Wasm build only, sized to push the ratio over 2 | R1-ECS-REC1 mark 1 (line 1929); the one-machine reading proposed ("Native and Wasm" above) |
| PM-0004-8 | The browser runs the whole sim within [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share) (S2; part 6) | p99 sim step time ≤ 13.33 ms (40 % of 33.33 ms) in Wasm in a dedicated Worker at the N that R1-TICK-OQ7 sets, while the page meets ADR 0003's minimum-laptop row over the whole run (frame time median ≤ 28 ms and p95 ≤ 33.3 ms, mode U: the reading of "holds 30+ fps on Low", since R1-TICK-REC2 P2 leaves the render budget to R3; it is the drafts' one reading of P2's "30+ fps", so ADR 0002's PM-0002-28 cites this row and sets none of its own, and the presented-frame count on ML at Low is ADR 0003's PM-0003-44, reported there, which gates on RD at High (OQ-0003-17, answered by the Advisor, confirmed by Mustafa, 2026-11-03)); until R1-TICK-OQ7 is answered: reported at N = 12, 24, 36 and 48, no pass | ML: Chrome and Firefox on Windows 11 (Edge reported; Safari on MAC reported, no budget row) | T on SS in the boundary spike's Worker with the sim core spike's sim; the page draws the sim's output at Low (ADR 0003's renderer if it exists by then, else the boundary spike's page: recorded which); step times from the Worker's timer around the step, isolation flag on; frame times from the page's harness, mode U (ADR 0003: vsync off) | T, V; rule B | R1-TICK-REC2 P2 (lines 2054–2057); R1-TICK-OQ7 (lines 2667–2670); R3-BUDGET-REC2's frame rows (r3 lines 1123–1124); GL lines 214–218 |
| PM-0004-9 | One time representation keeps every legacy rate at the baseline (S4; part 4) | with held fire for 60 s of game time at 30 Hz, each of the 13 legacy intervals (12 `fireInterval`s and the one `burstInterval` in `docs/legacy-tuning.json`, each run as a repeating timer: proposed reading for the burst) gives a shot count within 1 shot of 60 ÷ interval (the Shabzon's 0.072 s: 833.3 shots, so within 1 shot is within 0.12 %, inside R1-ABL-REC3's 1 %: arithmetic); CD13's Warrior-kills-Conscript time of 0.86 s reproduced within one tick (33.33 ms) | CI (counts) | a scripted fire test per interval on the sim crate; the kill time from the sim's events in a scripted duel | 1 run (deterministic). Bites: legacy's reset-not-carry timer (GC5: 10.0 instead of 13.9 shots per second at 30 Hz) | R1-TICK-REC1 (lines 2022–2024); R1-ABL-REC3 (lines 2411–2412, 2416–2418); LEGACY_INVENTORY GC5 (line 244), CD13 (line 94) |
| PM-0004-10 | The step reads no clock (S4; part 4) | 0 uses of `std::time` in the sim crate; 0 host time imports in the built sim `.wasm` module | CI | a source scan of the sim crate; the module's import list read with a Wasm parser (the tool is ADR 0002's: proposed) | every gate run (G2). Bites: `Instant::now()` in a tick (R5-GATE-REC3's G2 mutation, r5 line 1714) and a JavaScript `Date.now` import; control: `Instant::now()` in the server crate passes (the same row) | R1-TICK-REC1 (line 2027); R5-GATE-REC3 (r5 lines 1706–1714) |
| PM-0004-11 | Durations convert to one integer unit, and the units per tick are chosen from data (S4; part 4; R1-ABL-OQ7) | the loader's rounding report lists every duration in `docs/legacy-tuning.json` with its rounding; the chosen units-per-tick figure is written into this record beside the report (no number: the research gives none); 0 legacy intervals under one tick at 30 Hz flagged (the smallest is 0.06 s against 0.0333 s) | CI | the content loader run over the legacy tuning data | 1 run. Bites: a planted 0.02 s interval is flagged for the game-designer | the draft's part 4 and S4 (proposed there); R1-ABL-REC3 (lines 2403–2404); R1-ABL-OQ7 (lines 2776–2778) |
| PM-0004-12 | Snapshots are cheap and allocate nothing (S5; part 3) | 0 heap allocations per snapshot after warm-up; one snapshot (the hand-written `clone_from` into the preallocated copy) p99 ≤ 0.2 ms native and ≤ 0.5 ms in Wasm | RS (native); ML: Chrome and Firefox (Wasm, isolation flag on) | one snapshot per tick through a T run on SS at N = 48; the counting allocator around each snapshot; each snapshot timed alone | T, V; rule A (a derived `clone_from`, which does not reuse buffers, R1-ECS-OQ2, must be counted); rule B | R1-ECS-REC1 mark 2 (lines 1932–1937); R1-ECS-OQ2 (lines 2614–2618) |
| PM-0004-13 | Restore and continue is exact, ids and streams included (S5, S6, S8; parts 2, 3, 8) | over M10 (18,000 ticks): a snapshot taken at every 30th tick, restored 5 ticks later and those 5 ticks re-run from the recorded inputs (proposed: 600 restores, each on a different state, as a predicting client re-simulates); the per-tick [state hash](../research/GLOSSARY.md#state-hash) equals the uninterrupted run's on 18,000 of 18,000 ticks; serialize → deserialize → serialize byte-identical at every 300th tick (60 checks, proposed) | CI (native, and Wasm in Node); RS | the replay runner (ADR 0002) with a restore mode; the comparer reports the first mismatch with a field-level diff | 1 run per target (deterministic). Bites: one stream's position left out of the hand-written `clone_from` fails at the first restore after that stream draws | R1-ECS-REC1 mark 4 (lines 1942–1945); R1-ECS-REC2 mark 3 (line 1971); R1-ECS-REC3 mark 1 (line 1986); R1-DET-REC4 (line 2206) |
| PM-0004-14 | Whole-world restore and replay, only if ADR 0001's prediction scope needs it (S5; part 3) | restore plus k = ⌈0.15 s × 30⌉ = 5 ticks of re-simulation, p99 ≤ 8 ms in Wasm | ML: Chrome and Firefox | T on SS at N = 48, one restore and replay per tick | not run while ADR 0001's prediction scope is R2-SYNC-REC3's (own movement, own weapon state, own vehicle) | R1-ECS-REC1 mark 3 (lines 1938–1941); the draft's part 3 |
| PM-0004-15 | A stale id never resolves (S6; part 2) | 0 failures in 10,000 cases: random spawn and despawn sequences of up to 1,000 operations on a pool of 64 slots (proposed: a pool smaller than the sequence forces reuse), after which every despawned id resolves to nothing | CI | a proptest property on the pools | gate (fixed seed) and nightly (fresh seeds); 20 identical verdicts. Bites: a pool that does not raise the generation on reuse | R1-ECS-REC3 mark 2 (lines 1987–1988); R5-TEST-REC6 (r5 lines 1492–1494) |
| PM-0004-16 | The client maps the server's ids with no table (S6; part 2) | every per-tick server state of an M10 run, applied to a client instance, resolves every id to the same kind, slot and generation: 0 misses or mismatches; the client's apply path holds no id-keyed map (a reviewer's check, recorded) | CI (native server, Wasm client in Node) | the replay runner's server states fed to a client-mode instance; ids compared per tick | 1 run. Bites: a client that numbers slots in arrival order | R1-ECS-REC3 mark 3 (line 1989) |
| PM-0004-17 | Nothing lives outside the state struct (part 1) | two instances in one process, stepped in turn with different seeds, give per-tick hashes equal to each one's solo run: 0 mismatches over M10 each | CI (native; Wasm in Node) | the replay runner with two instances | 1 run. Bites: a `static` counter read by a phase | proposed: part 1's "no globals or statics in the sim crate" has no research mark, and ADR 0002's local match in two instances (OQ-0002-3 (b)) rests on it |
| PM-0004-18 | The same build reproduces itself (S7; parts 7, 9) | the same seed and input log in two native processes give the same state hash on 36,000 of 36,000 ticks; each process logs its std `HashMap` seed and the two differ (proposed: with equal seeds a hidden map's order could not show) | RS; CI | SS's recording at N = 48 through the replay runner, built as the server builds the sim (R5-TEST-REC2, r5 line 1413), twice | 1 pair. Bites: PM-0004-21's injections | R1-TICK-REC1 (lines 2025–2026); R1-ECS-REC2 mark 1 (lines 1967–1968) |
| PM-0004-19 | Wasm and native agree tick by tick (S7; part 7) | 0 mismatching per-tick state hashes over bots-only 24v24 (N = 48) and 12v12v12 (N = 36) matches of 18,000 ticks each, 3 seeds each: between two native processes, and between native and Wasm in Node, in Chromium, Firefox and WebKit under Playwright, and in real Safari | CI (native, Node, the three engines); MAC (Safari); ML once (Chrome and Firefox on Windows; proposed: the players' own platform, which CI's Linux runners are not) | the determinism suite (part 9; ADR 0002's tools, R5-TEST-REC2) on the six recordings | 1 run per target and recording; nightly in CI after the spike. Bites: PM-0004-21's injections | R1-DET-REC1 (lines 2139–2144); R1-DET-REC6 (lines 2222–2229); R5-TEST-REC2 (r5 lines 1412–1418) |
| PM-0004-20 | parry3d's determinism flag holds across targets, and its cost is known (S7; parts 7, 10; decides R1-DET-OQ1) | with `enhanced-determinism` on: 0 mismatches between native and Wasm over SS for 18,000 ticks; reported: parry3d's query time per tick with the flag on and off, native and in a browser, mean and p99 | CI (hashes); RS (native); ML: Chrome (Wasm) | SS at N = 48; the hashes as PM-0004-19; the times from the phase entry | 1 run for the hashes; T, V for the times | R1-DET-REC3 (lines 2178–2182, applied to parry3d by the draft's part 10); R1-DET-OQ1 (line 2680) |
| PM-0004-21 | The suite catches a divergence at the tick it happens; the hash function is fixed (S7; part 9) | each injected divergence (one std `sin` call, one std `HashMap` iteration, one presentation random draw inside the sim) reported at its arming tick T with a field-level diff: 3 of 3; reported: the hash function picked, and its cost per tick (mean, p99); and neither the hash nor the serializer depends on the target's word size: the picked function gives the same digest for a committed byte string on native x86-64 and on wasm32, and one committed state serializes to the same bytes on both (proposed: `usize` is 32 bits on wasm32 and 64 bits on x86-64, so a length or index written as `usize`, or a hasher fed through `write_usize`, would make PM-0004-19 fail on its first tick for no game reason) | CI (native ×2, Node, the three engines); RS (the cost) | each injection armed at a known tick T with a value that differs between the two runs (proposed: an injection that changes no state proves nothing): the `sin` input taken from PM-0004-22's table where std's `sin` differs between native and Wasm (if none differs, the injection is recorded as not catchable on those targets and a 1-ulp planted change stands in); the `HashMap` holding at least 32 keys whose order decides an outcome, with its two orders logged | 1 run per injection; the digest and serialization vectors join the unit vectors (20 identical verdicts). Control: a presentation draw outside the sim passes (R5-GATE-REC3's G10 row, r5 line 1721). Bites (the word size): a planted `usize` field in the serialized state must change the bytes between the two targets | R1-DET-REC6 (lines 2222–2240); R5-TEST-REC2's mark (r5 lines 1427–1428); R5-GATE-REC3 (r5 line 1721) |
| PM-0004-22 | The maths library is one implementation everywhere (S7; part 7; decides R1-DET-OQ8 and R1-DET-OQ10) | for each `libm` function the sim uses, and for `sqrt`: at least 10,000 fixed inputs (a committed table of edge cases and seeded values: proposed) give bit-identical outputs (`to_bits`) on native x86-64 and in Wasm in Node and in each browser engine; `sqrt` also equals a correctly rounded reference; if part 7's `rem_euclid` or `div_euclid` is to join the allow-list (neither is on it, so this row's pass today does not wait on them), the method gets the same check, at least 10,000 fixed inputs with (3 + 2^-22, 1 + 2^-23) in `f32`, (3 + 2^-51, 1 + 2^-52) in `f64` and their sign mirrors among them, and its outputs also equal by value (`==`, which leaves a zero's sign to `to_bits`) a correctly rounded Euclidean reference, the exact quotient and remainder by integer or rational arithmetic rounded once to the type: for (a, b) and its mirrors (−a, b), (a, −b) and (−a, −b), the quotients 2, −3, −2 and 3 and the remainders 1, 2^-23, 1 and 2^-23 in `f32` and 1, 2^-52, 1 and 2^-52 in `f64` (verifiers fixcheck-d3a and skeptic-6 and the Rust and test reviews, 2026-11-03); part 7's `min`, `max` and `clamp` helpers give the same bits on every target for equal signed zeros, (+0, −0) and (−0, +0), and a clamp with its bounds the wrong way round returns a bound with no panic (part 7: one defined argument on equal inputs); reported: each function's largest error in units in the last place against a higher-precision reference (R1-DET-OQ8's "enough" read as PM-0004-30's 1 mm apex and PM-0004-54's 1 cm cursor marks passing with `libm`: proposed), and `libm`'s cost against std's, native and in Wasm | CI; MAC (Safari); RS and ML (the cost) | unit vectors in `cargo test` and wasm-bindgen-test (R5-TEST-REC1) | 20 identical verdicts. Bites: one output moved by one unit in the last place on one target; and, for the Euclidean clause when it runs, errors every target shares: a `div_euclid` planted as rustc 1.97.0's steps (`trunc(a / b)`, corrected by one only when `a % b < 0`: skeptic-6's disassembly, 2026-11-03), which gives 3 for (3 + 2^-22, 1 + 2^-23) where the exact quotient is 2, and one `rem_euclid` output moved by one unit in the last place on every target alike | R1-DET-REC2 (lines 2165–2168); R1-DET-OQ8 (lines 2704–2705); R1-DET-OQ10 (lines 2710–2712) |
| PM-0004-23 | The float rules and [the hash-map ban](../research/GLOSSARY.md#the-hash-map-ban-in-the-sim-crate) are enforced by the build (S7; parts 7, 9) | the build fails on any std `f32` or `f64` method outside part 7's allow-list (`powi`, `hypot`, `cbrt`, `to_degrees`, `to_radians`, `min`, `max` and `clamp` named), as well as on each of: a std transcendental call, an `algebraic_*` method, the `relaxed-simd` feature, a std `HashMap` or `HashSet` in the sim crate, a server build with `+fma` or `-C target-cpu=native`; 0 such uses in the spike's own build | CI | the gate v2's G2 lints (ADR 0002's `disallowed-methods`, the complement of part 7's allow-list) and its build-flag check (ADR 0002) | every gate run. Bites: each forbidden item planted once (R5-GATE-REC3's G2 row: a `HashMap` field in `sim`, `x.sin()`), plus `deg.to_radians()` and `a.min(b)` in `sim`, each failing G2, as in PM-0002-41; controls: a reviewed `#[allow]` for a lookup-only map outside the state passes, and so do part 7's three helpers and one allowed method (`x.abs()`, `x.fract()`) in `sim` | R1-DET-REC2 (lines 2146–2150, 2166–2168); R1-DET-F9 (lines 648–654) and R1-DET-F14 (lines 679–683), as part 7 cites them; R1-ECS-REC2 mark 2 (lines 1969–1970); R1-RR-OQ1 (lines 2929–2931); R5-GATE-REC3 (r5 line 1714); GL lines 729–734 |
| PM-0004-24 | NaN never enters the state (part 7) | 0 non-finite values in the state over every run of PM-0004-13 and PM-0004-18 to -21, read by the replay runner itself on the release builds those runs use (it already reads every float by `to_bits` for the state hash, part 9, so it flags any non-finite value with no sim feature, which PM-0004-74 would show as a difference), with the finite-state assertion on in debug and test builds; every non-finite value in PM-0004-60's fuzzed commands rejected where it enters | CI | the assertion each tick; NaN and infinities in the fuzzed commands' float fields | as the runs it rides on. Bites: a planted `0.0 / 0.0` in one phase trips the assertion at its tick | R1-DET-REC2 (lines 2150–2152); the draft's part 7 |
| PM-0004-25 | Whether Wasm `simd128` may be turned on (S7; part 7; decides R1-DET-OQ2) | PM-0004-19's suite with `simd128` on: 0 mismatches lets it be turned on; any mismatch keeps it off. A decision row | CI; MAC | as PM-0004-19 | 1 run | R1-DET-OQ2 (lines 2682–2685) |
| PM-0004-26 | Native AArch64 agrees, before ARM hosts are picked (S7; part 7) | PM-0004-19 and PM-0004-22 with native AArch64 added: 0 mismatches | A64 | as PM-0004-19 and -22 | only before ADR 0006 may pick ARM hosts | R1-DET-REC2 (line 2166); R1-RR-OQ1 (lines 2929–2931) |
| PM-0004-27 | The random streams replay exactly (S8; part 8; decides whether R1-DET-OQ11 returns) | the first 10,000 outputs of each stream, and of the sim's own sampling functions on them, equal pinned test vectors on every target of PM-0004-19; reported: ChaCha8's cost per tick (mean, p99) at N = 48 | CI; MAC; RS and ML (the cost) | unit vectors; the cost from the phase entry | 20 identical verdicts; T, V for the cost. Bites: a stream seeded from the clock | R1-DET-REC4 (line 2205); R1-DET-OQ5 (lines 2691–2696); R1-DET-OQ11 (lines 2713–2717) |
| PM-0004-28 | Presentation never reaches the sim (S8; part 8) | in one recorded match, each pair gives identical per-tick state hashes: sound on and muted; the camera still and moved; the display at 60 Hz and at 144 Hz (or the highest rate on hand, recorded) | RD: Chrome, on PM-0004-69's displays | the boundary spike's page with the sim core spike's sim replaying one recording; the hash stream read from the Worker | 1 run per pair. Bites: a sim draw from the page's presentation generator (R5-GATE-REC3's G10 mutation, r5 line 1721) | R1-DET-REC4 (lines 2206–2207) |
| PM-0004-29 | [Authority-only streams](../research/GLOSSARY.md#shared-stream-and-authority-only-stream) stay off the client (S8; part 8) | 0 occurrences of any authority-only stream's seed or state bytes in the predicting client's serialized state, after every tick of M10 | CI (Wasm in Node) | the prediction step (part 3's second entry point, which draws from no stream) on a client-mode instance, as ADR 0001's prediction scope uses it; each tick's serialization searched for every authority-only stream's seed and state | 1 run. Bites: the spread stream copied into the client's state; a seed derived from the room id instead of drawn by the host from the operating system's random source (part 8) must be found, since the search also derives a seed from every identifier the client receives (room id, match id) by the build's own derivation and compares it with each authority-only seed | the draft's S8 (proposed there: SEC-SPREAD (a)); GL lines 525–532 |
| PM-0004-30 | Our own movement does what the movement data says (S9; part 10) | on LB at 30 Hz: a soldier steps up 0.55 m and is stopped by 0.56 m; the jump apex equals the integrated value (PM-0004-59's) within 1 mm; 0 tunnellings through a 0.2 m wall in 10,000 random moves at 13.27 m/s (legacy's fastest soldier: 6.3 × 1.62 × 1.3, arithmetic; 0.44 m per tick at 30 Hz) | CI | scripted moves on the sim crate | 1 run (deterministic). Bites: a move applied as a position step without a sweep tunnels | R4-PHYS-REC1 mark 1 (r4 lines 693–699) |
| PM-0004-31 | Movement and physics fit their part (S9; parts 6, 10) | movement plus all physics queries: mean ≤ 1 ms native, its p99 reported, apart from AI and abilities; Wasm ≤ 2 × native on one machine (PM-0004-7's method) | RS; ML | the phase entry on SS at N = 48 | T, V; rule B | R4-PHYS-REC1 mark 2 (r4 lines 700–703); GL line 55 |
| PM-0004-32 | One static collision world, identical everywhere (S9; parts 3, 10) | the static BVH built from one map document has byte-identical nodes on native and Wasm; its build ≤ 50 ms native at match start (a plain limit); a restore followed by the index rebuild gives byte-identical results for 10,000 random rays | RS (build time); CI (bytes, rays) | the BVH dumped node by node on each target; the build timed 10 times (proposed) | every build within the limit; 1 run for the bytes. Bites: colliders inserted in another order on the Wasm build | R4-PHYS-REC1 mark 3 (r4 lines 704–705); R4-PHYS-REC2 (c), (d) (r4 lines 741–743) |
| PM-0004-33 | Size and build speed (S9; parts 1, 10; records R1-ECS-OQ1) | recorded after wasm-opt and gzip: the sim `.wasm` with parry3d, with rapier3d and with neither, each smaller than a stub of the same tick on bevy_ecs 0.20 with default features off; a one-line change to a phase rebuilds natively in ≤ 10 s (a plain limit) | CI (sizes); the session's build machine (the research's "dev machine": recorded) | one build each; the rebuild timed 5 times (proposed) | each of the 5 rebuilds within 10 s | R1-ECS-REC1 mark 5 (lines 1946–1948); R4-PHYS-REC1 mark 4 (r4 lines 706–707); R1-ECS-OQ1 (line 2612) |
| PM-0004-34 | Rapier's controller is measured once, as the fallback (S9; part 10; decides R4-PHYS-OQ2) | option B with autostep on: PM-0004-30's parity marks, PM-0004-31's cost and PM-0004-33's size, reported; option C stays unless B wins on parity and on cost. A decision row | RS; CI | as PM-0004-30, -31 and -33 | T, V for the cost | R4-PHYS-REC1 mark 5 (r4 lines 708–711); R4-PHYS-OQ2 (r4 line 1085) |
| PM-0004-35 | parry3d supports every shape pair the sim needs (S9; part 10; checks OQ-0004-3's answer) | 0 `Err(Unsupported)` results over PM-0004-30's and -31's runs; the pairs listed (the soldier's capsule or cylinder and the grenade's ball against oriented boxes, ramp wedges, other convex shapes and the ground shape; vehicle hull compounds against the same and against capsules) | CI | a counter on every query's result | 1 run. Bites: an unsupported pair placed in the scene is counted | R4-PHYS-REC1 mark 6 (r4 lines 712–717); R4-PHYS-OQ4 (r4 line 1090) |
| PM-0004-36 | One query set keeps legacy's answers where they are kept, and fixes the ramp (S9; part 10) | 20,000 random rays on BR give the same hit or miss and the same box as legacy's SIM, distances within the fixture's stated tolerance; 0 of 1,200 bot shots stop more than 1 cm above a ramp's slope | CI | the CS6 golden fixture (R5-TEST-REC3: captured once from legacy, committed); bot shots on O3's ramps | 1 run. Bites: a golden value moved beyond its tolerance (R5-TEST-REC3's mark) | R4-PHYS-REC2 (a), (b) (r4 lines 739–741); R5-TEST-REC3 (r5 lines 1431–1437, 1451–1452) |
| PM-0004-37 | Projectiles are swept, grenades bounce, landings agree (S9; part 10) | at 30 Hz: 0 tunnellings through a 0.2 m wall in 10,000 shots at the fastest projectile speed in the data; a grenade thrown at a wall bounces back in 1,000 of 1,000 throws; a fixed throw's landing point bit-identical on native and Wasm | CI | scripted shots and throws on the sim crate | 1 run. Bites: a projectile tested only at its new position, with no segment | R4-PHYS-REC3 (r4 lines 758–760) |
| PM-0004-38 | Big vehicles never end a tick inside a soldier or a wall (S9; part 10; answers R1-TICK-OQ9) | 0 ticks ending with a soldier or a wall more than 1 cm inside a vehicle hull, over 18,000 ticks (10 minutes at 30 Hz) of a vehicle driven into walls, ramps and a crowd of 48 soldiers, turning on the spot and while moving; reported: the turning method picked (a nonlinear cast, or an overlap test before the linear sweep) and its cost | CI (the count); RS (the cost) | a scripted drive; penetration depth checked after every tick with a contact query | 1 run. Bites: the turn applied without its check | R4-PHYS-REC4 (r4 lines 779–784); R1-TICK-OQ9 (line 2675) |
| PM-0004-39 | f32 holds at the far edge (S9; part 7's 2 km bound) | with the greybox at the far corner of a 2 km map (part 7's bound; proposed reading of "the largest planned map", whose size LOOP_V2 §9 leaves open; where the map format puts the origin is recorded), PM-0004-30's parity marks and PM-0004-37's tunnelling mark still hold, and a scripted soldier walking a fixed 10-minute route covers the same distance per second as at the origin within 0.1 % | CI | the same scripts with the offset | 1 run | R4-PHYS-REC5 (r4 lines 796–798) |
| PM-0004-40 | The navigation bake is quick enough for the editor's Test (S10; part 11; with PM-0004-41 to -44, decides R4-NAV-OQ1 and OQ2) | each generator (rerecast's polygon mesh; the layered grid from the sim's own queries) bakes O3 in ≤ 1 s native and ≤ 3 s in a browser Worker (plain limits) | RS (native); ML: Chrome (Worker) | 5 bakes per generator and target (proposed) | each of the 5 within its limit. Bites: rule B, once, inside the bake | R4-NAV-REC1 mark 1 (r4 lines 831–832); R4-NAV-OQ1 (r4 line 1099) |
| PM-0004-41 | The baked nodes cover where soldiers stand (S10; part 11) | of 10,000 points where a scripted soldier stood, ≥ 99.9 % lie on a node; of 10,000 random points on nodes, 0 are places a 0.7 m soldier cannot stand (checked with the sim's movement) | CI | each generator's bake of O3 | 1 run per generator. Bites: an agent radius rounded below 0.7 m gives unstandable nodes | R4-NAV-REC1 mark 2 (r4 lines 833–834) |
| PM-0004-42 | Gaps and reachability come from the sim (S10; parts 11, 21) | a 6 m gap between two blocks stays a route; the bake reports every gap under 6 m; the reachability check (every spawn to every goal, a flood fill as the ground truth) runs inside the sim crate from Node and from CI, with identical results on native and Wasm | CI | O3 with a planted 5.5 m gap (proposed) | 1 run. Bites: the 5.5 m gap must be reported | R4-NAV-REC1 marks 3, 4 (r4 lines 835–837) |
| PM-0004-43 | Paths and the baked data are identical everywhere (S10; part 11) | 10,000 random path queries give byte-identical paths on native and Wasm; the baked data hashes the same wherever it is loaded | CI | each generator | 1 run. Bites: path ties broken by a std `HashMap`'s order | R4-NAV-REC1 mark 5 (r4 lines 838–839) |
| PM-0004-44 | Which generator (S10; part 11) | reported for both generators: node count, data size in the map document, mean and p99 query cost, path length against the straight-line optimum; the pick by PM-0004-40 to -43 first, then these numbers. A decision row | RS; CI | as PM-0004-40 to -43 | T, V for the query costs | R4-NAV-REC1 mark 6 (r4 lines 840–841) |
| PM-0004-45 | Navigation fits its part and ends the replan storm (S10; parts 6, 11; decides R4-NAV-OQ3) | navigation (queries plus route following) p99 ≤ 1.67 ms (5 % of 33.33 ms) at N = 48 and in the N = 64 headroom run, with an uplink cache carried around the map; 0 bots waiting more than 1 s for a route; replans per bot in each whole second of game time at most 2 at p99 over all bot-seconds (proposed reading of "per bot per second at p99"); the headless stall rate (alive and moved under 0.3 m in 3 s: legacy TL13's test) at most 7.73 % | RS | T on SS at each N; the phase entry for the time; the bots' route log for waits, replans and stalls | T, V; the stall rate in each of the 3 runs (TL13's 3 runs per build). Bites: rule B; a route budget that resets per call (legacy GN13's storm) fails the replan mark | R4-NAV-REC2 (r4 lines 859–865); GL line 58; LEGACY_INVENTORY TL13 (line 398), GN13 (line 278) |
| PM-0004-46 | Local avoidance is cheap, deadlock-free and floor-aware (S10; parts 6, 11) | 48 bots crossing a 6 m choke point for 18,000 ticks: 0 deadlocks longer than 3 s (a bot with a route that moves under 0.3 m in 3 s: TL13's stall test, proposed as the deadlock test), per-tick hashes identical on native and Wasm; the avoidance step p99 ≤ 0.2 ms native at N = 48 in SS (the share's scene, GL line 56), and reported in the choke; on TB, 0 avoidance pairs join bots on different floors | RS (time); CI (counts, hashes) | a scripted choke on O3; the avoidance step timed by the phase entry | T, V for the time; 1 run for the counts. Bites: rule B; neighbours taken across floors | R4-NAV-REC3 (r4 lines 880–884) |
| PM-0004-47 | Navigation data goes only where bots run (S10; part 11) | the Worker build without bots skips the navigation data (its size, and the bytes saved, reported); with bots, PM-0004-8 holds | CI (sizes) | both builds | 1 build each | R4-NAV-REC4 (r4 lines 896–897) |
| PM-0004-48 | Links are what the movement can do (S11; part 12) | on O3 and LB at 30 Hz: 0 generated jump links above the integrated reach measured at 30 Hz (PM-0004-59; the research's "2.9 m kept, 3.0 and 3.1 m rejected" is its 60 Hz form); each kept link succeeds in 1,000 of 1,000 bot traversals; from every spawn to every goal, routes exist and 0 % need a surface bots cannot reach; a bot on high ground whose goal is below takes the drop-down whenever it is the shorter route in ≥ 95 % of 200 trials | CI | the bake and scripted bot traversals | 1 run (deterministic). Bites: links generated from legacy's closed-form reach (GW14: about 9 cm over the integrated jump at 60 Hz) fail the first mark | R4-FLOORS-REC1 (r4 lines 918–923); LEGACY_INVENTORY GW14 (line 202) |
| PM-0004-49 | Floors are separate layers with ceilings (S11; part 12) | on TB: each floor's nodes carry different layer and area ids, and the only edges between floors run along the stairs; a bot on the ground floor reaches a goal directly above in 100 of 100 trials, never by walking into the ceiling; under the barracks roof the nodes are walkable and the roof is not, unless a link reaches it | CI | TB and the greybox barracks | 1 run. Bites: a bake without the floor-to-ceiling height test | R4-FLOORS-REC2 (r4 lines 936–939) |
| PM-0004-50 | A bake made for old movement data is caught (S11; part 12) | after `jumpV` changes in the tuning data without a re-bake, the sim crate's stale check, called from Node (the editor's path) and from CI, reports the navigation stale: 1 of 1; after the re-bake the hash matches | CI | the check on O3 | 1 run (the changed value is its own bite) | R4-FLOORS-REC3 (r4 lines 951–952) |
| PM-0004-51 | One sight query, right across floors and ramps (S12; part 13) | 20,000 random eye-target pairs on BR give the same visible or blocked answer as legacy `hasLOS`, margin 0.5 m; between TB's floors, 0 of 10,000 random pairs see through the floor or the ceiling, and pairs with a clear segment through the stairwell or window are seen in 100 %; on O3, 0 of 10,000 pairs blocked by a ramp's bounding box but clear of its slope | CI | the legacy golden (R5-TEST-REC3) and seeded pairs | 1 run. Bites: sight tested against ramps' bounding boxes (legacy GN7's) | R4-LOS-REC1 marks 1–3 (r4 lines 976–981) |
| PM-0004-52 | The shared visibility pass fits its part (S12; parts 6, 13; decides R4-LOS-OQ5) | the full pass p99 ≤ 1.67 ms at 24v24 and at 16v16v16 (the test-mode list), three sample points per soldier, everyone in one fight; the pass's results hash identically on native and Wasm each tick; reported: mean and p99 of one segment query, native and in a browser Worker | RS; ML: Chrome (one query in the Worker); CI (hashes) | T on SS (16v16v16) and on a 24v24 form of it; the phase entry | T, V; rule B | R4-LOS-REC1 marks 4, 5 (r4 lines 982–986); GL line 59; R4-LOS-OQ5 (r4 line 1133) |
| PM-0004-53 | Bots look often enough at 48 (S12; part 13) | at 24v24 and at 16v16v16 in one fight on the greybox: every bot scans at least 4 times in every whole second of game time after warm-up (the minimum over bot-seconds: proposed reading of "every bot scans at least 4 times per second"), within PM-0004-52's share; reported: the share of pairs culled before a ray and the rays per tick | RS | PM-0004-52's runs; the bots' scan log | T. Bites: legacy's per-second budget sized for 12 soldiers (GN4: about 2.5 scans per bot per second at 48) | R4-LOS-REC2 (r4 lines 1002–1006); LEGACY_INVENTORY GN4 (line 269) |
| PM-0004-54 | The point the player clicks is the point the server tests (S12; part 13) | a downward cursor ray onto a ramp lands within 1 cm of the slope in 1,000 of 1,000 samples (R2-SYNC-REC8 mark 1 is ADR 0001's) | CI | the ray query on O3's ramps | 1 run. Bites: ramps queried as boxes (legacy GC3: 1.25 m above the slope) | R4-LOS-REC3 (r4 lines 1015–1016) |
| PM-0004-55 | Big vehicles are obstacles without a replan storm (S13; part 14; decides R4-OBST-OQ4) | SS for 18,000 ticks at 30 Hz with big vehicles driving through bots' routes: 0 bots stuck longer than 3 s against a vehicle (TL13's 0.3 m in 3 s); replans per bot per second at most 2 at p99 (PM-0004-45's reading); per-tick hashes identical on native and Wasm; reported: the vehicles' marking and avoidance work as a share of the tick, and the hull footprint form picked (a polygon, or circles) | RS; CI | SS with scripted vehicle routes | 1 run for the counts; T for the reported time. Bites: vehicles left out of the avoidance step | R4-OBST-REC1 (r4 lines 1037–1041); R4-OBST-OQ4 (r4 line 1145) |
| PM-0004-56 | A structure's change flips route, sight and portal in one tick (S13; part 14) | over 1,000 destroy-and-rebuild cycles of a test wall: 0 ticks where route, sight and the portal flag (the area connectivity ADR 0001's re-flood reads) disagree; every route found after a flip walked by the sim's movement without failure in 1,000 traversals; a flip p99 ≤ 0.1 ms native over the 2,000 flips | RS (time); CI (counts) | a scripted wall on O3 | the 2,000 flips once; rule B | R4-OBST-REC2 (r4 lines 1058–1061) |
| PM-0004-57 | Three damage stages work as two do (S13; part 14; LOOK_V2 §4) | PM-0004-56's marks with three stages per structure on TB (intact, damaged, ruin), including a plank link to the upper floor that exists only in the ruin stage; the bake with every stage still within PM-0004-40's limits | RS; ML; CI | TB's three baked states | as PM-0004-56 and -40 | the draft's S13 (proposed there: LOOK_V2 §4 names three stages, the research tested two) |
| PM-0004-58 | Bot reaction at the largest mode (S14; part 15) | reported, no pass number: per bot, the ticks from a target first visible in the shared pass to the bot's first shot command; median, p95 and max at N = 48 | RS (counted in ticks, so machine-free) | PM-0004-3's runs; the bots' event log | 1 run | R2-SYNC-OQ13 (r2 lines 2952–2954) |
| PM-0004-59 | The editor's reach is the sim's reach (S19; part 21) | the reach the sim crate returns to the editor equals the measured jump apex plus step-up within 1 mm at 30 Hz, for `T.jumpV` 11, `T.gravity` 24, `T.stepUp` 0.55 | CI | the validation function called from Node; a scripted jump in the sim | 1 run. Bites: legacy's closed-form formula (GW14) | R1-TICK-REC6 (lines 2124–2125); LEGACY_INVENTORY GW14 (line 202) |
| PM-0004-60 | No panic from what a client can send (S20; part 20) | 0 panics over 10 million fuzzed commands fed to a running sim (proposed count: ADR 0001's per-message-type decoder fuzz, R2-TRN-REC4 mark 4), every field over its full decoded range, NaN, infinities and claims on ids that do not exist included | CI (nightly, Linux) | cargo-fuzz on a target that feeds decoded command values into a running sim (R5-TEST-REC4's fuzz targets), in a small scene: a 1v1 on LB (proposed: coverage comes from the command values, not from the crowd; at the stress scene 10 million ticks would take about 23 hours, 10,000,000 × 8.33 ms = 83,300 s, arithmetic); the corpus is kept between nights, so the 10 million accumulate within a nightly budget of 60 minutes (proposed), the running total recorded. Sequences of decoded control messages into the `room` crate (a pick on a closing offer, a host kicking itself, settings floods, join, leave and reconnect races) are fuzzed beside it once `room` exists, in ADR 0001's pass marks (beside PM-0001-34), since a panic there ends the match too (part 20) | nightly. Bites: a planted `.unwrap()` on a client-sent pickup id is found within the run | the draft's S20 (proposed there); R2-TRN-REC4 mark 4 (r2 line 2696); R1-RR-OQ3 (lines 2936–2940) |
| PM-0004-61 | A trap online ends in a fresh instance back in step (S20; part 20) | after a trap injected into the client's Worker: a fresh instance runs and holds the server's state (its [client-held state's hash](../research/GLOSSARY.md#state-hash) equal to the server's for that tick), the resync itself within ADR 0001's join mark (a complete world within 300 ms at P2); reported: the time from the trap to a running instance; the trap reaches error reporting (Sentry with Rust function names if ADR 0006's EU organisation exists by then, else the page's own error log), carrying the panic message the hook wrote into its fixed buffer (part 20), which the Worker's host reads after the trap: 20 of 20 messages arrive whole, each naming the planted panic's file and line | ML: Chrome, Edge, Firefox; MAC: Safari | the boundary spike's Worker against the netcode proof test's server at P2 (ADR 0002's simulator); 20 traps per browser at random ticks (proposed) | 20 of 20 per browser. Bites: a host that does not restart the instance; a host that reports the trap without reading the hook's buffer (the message must be missing from the report, so the check can fail) | R1-RR-OQ3 (lines 2936–2940); R2-BUD-REC3 (r2 line 2359: ADR 0001's join mark); R6-OBS-REC3 (r6 lines 765–766); GL lines 546–550 |
| PM-0004-73 | Throws land the same on Wasm and native (S21; parts 7, 10; the id is ADR 0001's) | 10,000 random throws: the trajectory function gives bit-identical points on Wasm and native, 10,000 of 10,000 (every point of every throw equal bit for bit) | CI (native; Wasm in Node and in Chromium, Firefox and WebKit under Playwright) | the shared trajectory function in the sim crate, fed 10,000 throws (start position, velocity, fuse) drawn from one fixed seed; each throw's point at every tick of its flight logged as raw bits on each target and compared by script (proposed: a compared hash alone would not show where two flights part) | 1 run per target (the function is deterministic). Bites: one std `sin` call in the trajectory function at an input where std's `sin` differs between native and Wasm (PM-0004-22's table; if none differs, a 1-ulp planted change stands in, as in PM-0004-21) | R2-SYNC-REC6 mark 1 (r2 line 2218); the draft's S21 |
| PM-0004-74 | One feature set builds the sim on every target (S22; part 7; the id is ADR 0002's) | the resolved feature list of every dependency of `sim` (`cargo tree -e features`) identical for every build that runs the sim (`server`, `devtools`, `navbake` and `sim-wasm`), each resolved by the exact command CI and the release use: 0 differences | CI | `cargo tree -e features` run for `sim` in each build's target and features; the two lists sorted and compared by script | 1 run; the foundation re-proves it as a gate check (ADR 0002). Bites: a planted extra feature on a shared dependency in `server` makes the check fail (R5-LAYOUT-REC4's own); control: a dev-only feature on `devtools` that touches no `sim` dependency passes | R5-LAYOUT-REC4 (r5 lines 1216–1218); the draft's S22 |
| **The content and ability spike** | | | | | | |
| PM-0004-62 | The ability, effect and weapon step fits its part (S15; parts 6, 16) | p99 ≤ 1 ms per tick with 384 active effects (8 per soldier at N = 48) in SS; 0 heap allocations in steady state | RS (native); ML: Chrome and Firefox (Wasm, isolation flag on) | T on SS with the spike's data-driven abilities; the phase entry; the counting allocator | T, V; rules A and B | R1-ABL-REC6 (lines 2465–2472); GL line 57 |
| PM-0004-63 | Abilities replay byte for byte (S15; parts 7, 16) | a 120 s scripted scenario (3,600 ticks at 30 Hz) using every test ability, run with 3 seeds, gives byte-identical logs of damage, effect start and end, and cooldown start and end on native and Wasm | CI | the replay runner natively, in Node and in the three engines (proposed: PM-0004-19's targets) | 1 run per seed and target. Bites: active effects applied in a std `HashMap`'s order | R1-ABL-REC3 (lines 2416–2419) |
| PM-0004-64 | Tracks survive deaths and switches (S16; part 18) | 0 track entries lost or changed: over a scripted 18,000-tick match in which each of the 48 slots dies at least 10 times and switches unit at least 10 times (proposed), every record compared field by field before and after each death and switch; a weapon type's track equal to before when the next gun of that type is picked up; a property test of random deaths, switches, pickups and picks: 0 failures in 10,000 cases (proposed, as PM-0004-15) | CI | the scripted match; proptest | 1 run; the property test 20 identical verdicts. Bites: a track kept on the soldier entity (the Pawn) instead of the record | UPGRADE-SCOPE (settled), its sentences as checks (the draft's S16; the numbers proposed here) |
| PM-0004-65 | Every match starts with empty records, sized by content (S16; part 18) | 100 of 100 consecutive matches in one process start with every slot's record at its start, field by field: empty tracks and money at its starting amount, $300 (part 18, S16) (proposed: a leak between matches shows only when one process runs several); 0 heap allocations in the record table after match start over M10 with random picks and pickup claims | CI | the match-start phase called 100 times; the counting allocator | 1 run. Bites: a record table reused without its reset; money carried from the last match instead of reset to $300; rule A | UPGRADE-SCOPE; LOOP_V2 §8 ("Nothing carries into the next match"); the draft's part 18 |
| PM-0004-66 | Pickups are ordinary entities, and ties resolve the same everywhere (S16; parts 2, 18) | pickups keep their ids through PM-0004-13's restores (0 changes); two claims on one pickup in one tick resolve to the same claimant on native and Wasm in 1,000 of 1,000 scripted ties (proposed count), with equal state hashes | CI | scripted drops and claims; the replay runner on both targets | 1 run. Bites: ties broken by iterating a std `HashMap` of claims | UPGRADE-SCOPE; the draft's S16 |
| PM-0004-75 | Each pool's overflow rule holds, per kind, as OQ-0004-11 answers it (parts 1, 16, 18; ADR 0005 AB-1's proposal) | only once Mustafa confirms OQ-0004-11; for the proposal as ADR 0005 AB-1 writes it: an effect the content bounds on its target (its own picks, its own ability's and weapon's effects, spawn protection) is never refused or ended at a full pool, 0 in 1,000 scripted pile-ups (proposed count, as PM-0004-66); when the share of effects others apply is full, the oldest such row on the target, by start tick and then sequence number (part 4), ends early, in 1,000 of 1,000 pile-ups on native and Wasm alike; the projectile pool's overflow counter stays 0 at the content's worst case at N = 48 (for legacy's Molotov, ⌊1.4 ÷ 1.05⌋ + 1 = 2 per thrower, 96 at N = 48: AB-1's arithmetic); a drop into a full pickup pool ends the pickup that has lain longest | CI | scripted pile-ups (48 sources on one target; every thrower throwing at its fire interval; deaths into a full pickup pool) on the replay runner, natively and in Node; equal per-tick state hashes and overflow counters on both targets | 1 run per kind (deterministic). Bites: a pool that refuses a content-bounded effect; a full effect share that ends the newest row instead of the oldest; a projectile pool sized from the stress scene's 300 instead of the content's worst case, which must count overflows | OQ-0004-11; ADR 0005 AB-1 ("When a pool is full"); part 1; R1-ABL-REC6 (lines 2471–2472: no heap allocation in steady state) |
| **The netcode proof test** | | | | | | |
| PM-0004-67 | The server loop keeps time (S17; part 5) | at 30 Hz: tick start jitter (actual minus scheduled start) p99 ≤ 2 ms over 36,000 ticks; after an injected 200 ms stall, back on schedule within 1 s (no ticks owed, and every later start within the jitter mark: proposed definition) and never more than 7 ticks in one wake (⌊0.25 s × 30⌋ = 7: the draft's restatement of the research's 15 at 60 Hz); after an injected 1 s stall, at most 7 ticks in that wake and the dropped time in the dropped-time counter (proposed: the drop rule's own check); overrun and dropped-time counters in the metrics output | RS (Linux) | the netcode proof test's room under its 30 Hz baseline load (ADR 0001); a planted sleep on the tick thread at a fixed tick; start times from the loop's monotonic clock | 3 runs of 36,000 ticks; each meets the mark. Bites: a loop that counts ticks from its timer period instead of elapsed time | R1-TICK-REC4 (lines 2082–2092); the draft's part 5 |
| **The boundary spike** | | | | | | |
| PM-0004-68 | The Worker paces the tick in every browser (S18; part 5) | foreground Worker tick start jitter p99 ≤ 4 ms over 18,000 ticks (10 minutes at 30 Hz), read against the Worker's own schedule: in local play the accumulator's, online the schedule the step returns from the client endpoint's server-time estimate (part 5, S18), with the estimate's changes over the run recorded | ML: Chrome, Edge, Firefox; MAC: Safari | the boundary spike's Worker (W2: `setTimeout` waking a `performance.now()` accumulator) with the sim core spike's sim at N = 48; start times logged in the Worker; online, against the netcode proof test's server at P2 (ADR 0002's simulator); the isolation flag as shipped (recorded); if W2 fails, W3 is measured on an isolated build (ADR 0002, OQ-0002-4) | 3 runs per browser; each meets the mark. Bites: ticks counted per timer callback instead of from elapsed time; online, a Worker that keeps its local schedule and ignores the server-time estimate must fail the jitter read against that schedule under P2 | R1-TICK-REC5 (lines 2096–2097, 2109–2110) |
| PM-0004-69 | Motion is smooth on every display (S18; parts 5, 6) | a unit moving at constant speed: the standard deviation of its per-frame on-screen displacement ≤ 5 % of its mean, on 60, 120 and 144 Hz displays | RD: Chrome and Firefox on the displays on hand (each recorded); ML: Chrome at its own rate; MAC: Safari | the boundary spike's page drawing between the two newest ticks; per-frame positions from the page; mode P (presented: what a player sees) | 3 runs of 60 s per display and browser (proposed). Bites: the newest tick drawn without interpolation | R1-TICK-REC5 (lines 2111–2112) |
| PM-0004-70 | A hidden tab is a pause in local play and a resync online (S18; part 5; decides R1-TICK-OQ2 and R1-BND-OQ2) | after 60 s hidden and after 6 minutes hidden ("more than 5 minutes": proposed): in local play no tick ran while hidden (the tick counter equal before and after) and play resumes where it stopped; online, within 0.5 s of the tab's return the client has applied the server's state and its commands are accepted again (proposed definition of "back in step"), never running more than 7 ticks in one frame | ML: Chrome, Edge, Firefox; MAC: Safari | the boundary spike's page; online against the netcode proof test's server at P2 (proposed: R1-TICK-REC5 names no profile, and P2 is VISION_V2 §4's) | 5 hides per duration and browser (proposed); each meets the mark. Bites: a Worker that simulates the whole hidden gap on the tab's return (it fails the equal tick counter in local play and the 7-tick cap online) | R1-TICK-REC5 (lines 2113–2114); R1-BND-OQ2 (lines 2724–2727) |
| **The 60 Hz set** (only with the 60 Hz margin's test) | | | | | | |
| PM-0004-71 | The sim step share at 60 Hz, for the 60 Hz margin only (S1, S2; part 6; TICK-60-HEADROOM) | inside each of ADR 0001's 60 Hz runs (its setup and R2-BUD-REC6's 30-minute soak): p99 sim step time ≤ 4.17 ms (25 % of 16.67 ms), max ≤ 16.67 ms, zero overruns; navigation and the visibility pass p99 ≤ 0.83 ms each (5 % of 16.67 ms); the client's sim share at 60 Hz, p99 ≤ 6.67 ms in Wasm (40 % of 16.67 ms) | RS (that run's tick thread); ML (the client share) | the sim's phase times read from that same run (the sim inside the proof-test server: in wave 1 the sim core spike's sim with S1's stand-ins, OQ-0001-2 (a), answered; at slice 6 the real crate), never added to a separately measured server-only share; the client share as PM-0004-8 on a 60 Hz build | with each of ADR 0001's 60 Hz runs; it decides only at slice 6 (the key) | R1-TICK-REC2 P1, P2 (lines 2047–2049, 2054–2056); GL lines 53–60; GL §8 (the 60 Hz margin); ADR 0001's spike item 3 |
| PM-0004-72 | The time mechanism and the movement checks hold at 60 Hz (S3, S4, S9, S11, S13; parts 4, 10, 12, 14) | with PM-0004-71's 60 Hz build (durations re-converted, links re-baked): every legacy interval gives the same shot count at 30 and 60 Hz within 1 shot; sight checks per game second per bot equal within 1 % at 30 and 60 Hz; PM-0004-37's tunnelling, PM-0004-38, PM-0004-48 (in the research's form: 2.9 m kept, 3.0 and 3.1 m rejected if the measured reach stays near legacy's 2.98 m) and PM-0004-55, each at 60 Hz | CI; RS | the same scripts at 60 Hz | with each of ADR 0001's 60 Hz runs, deciding only at slice 6 (the key); each as its 30 Hz row | R1-TICK-REC1 (lines 2023–2024); R1-TICK-REC3 (lines 2075–2076); R4-PHYS-REC3, REC4 (r4 lines 758–759, 779–784); R4-FLOORS-REC1 (r4 lines 919–921); R4-OBST-REC1 (r4 lines 1037–1038) |

**When a mark fails.** A spike passes when every mark that applies passes: a conditional mark (PM-0004-14, -26, -71,
-72, and PM-0004-75 until OQ-0004-11 is answered) only when its condition holds; a "reported" mark (PM-0004-4, -44,
-58) when every cell holds a number; a decision
mark (PM-0004-25, -34, -44) when the pick is written down with its numbers; PM-0004-8's pass waits for R1-TICK-OQ7's N.
A failing run is never re-run until green: its data, build commit, machine and target are kept, and its cause is found
and written down (R5-TEST-REC6, r5 lines 1490–1498). A failed mark goes back to this draft: rust-reviewer revises the
decision part the row names by the draft's Rollback (option B if our own movement fails PM-0004-30, -31 or -34; our own
query or option B for a pair PM-0004-35 finds missing; the legacy grid, ported and layered, if neither generator passes
PM-0004-40 to -43; R1-TICK-REC2's "back to the drafts", with parallelism inside a match as the candidate, if PM-0004-3
fails; same-build reproducibility alone, with more corrections, if PM-0004-19 fails and cannot be fixed; two stages if
PM-0004-57 fails), and the revised part returns to the advisor and to Mustafa and the Advisor with the measured
numbers. A mark run in another draft's spike (PM-0004-8, -28, -61, -67 to -71) comes back to this draft, not to the
host spike's. A mark's number changes only in this draft, with its reason, never inside a spike (budgets are data, and
raising one is a reviewed change: R5-GATE-REC4, r5 lines 1734–1738). A run set that fails V, or a run that fails
PM-0004-2, is neither a pass nor a fail.

**What has no measurable mark here** (one line each)
- Part 4's tick rate itself: ADR 0001's (TICK-30 and the 60 Hz margin); this record's 60 Hz forms wait in the 60 Hz set.
- Part 5's recovery in local play after a trap: OQ-0004-7, decided with ADR 0002's skeleton; no mark until then.
- Part 15's bot structure (OQ-0004-1): no mark; the spike's bot set is a load definition (PM-0004-2), and reaction is
  reported (PM-0004-58).
- Part 17 (economy and objectives): no spike mark; they arrive in slice 4, where a recorded full match replaying with
  identical per-tick hashes on every target is their check, with the test plan's planted divergence that changes only
  one economy value, caught at its tick, and migration phase 6's scripted cases for LOOP_V2 §2 to §5's rules.
- Part 18's open rules (which deaths drop which weapons, who may pick up, ties, takeovers: OQ-0005-AB9 and AB10; carry
  slots, looted ammo, levelling speed: ADR 0005): no mark until decided; PM-0004-64 to -66 test the record, not the
  rules.
- Part 19 (events, moments, telemetry): no spike mark; the event list is in the state hash, so PM-0004-18 and -19 cover
  its order, the test plan's planted divergence that changes only one event is caught at its tick, and the moment
  mechanism has no research behind it.
- S20's content half (zero panics over fuzzed content, R1-RR-OQ3) is ADR 0005's mark; the determinism suite's CI time
  (under 3 minutes, R5-TEST-REC2) is ADR 0002's.
- Only PM-0004-61 and PM-0004-70 name a network profile; PM-0004-67 runs under the netcode proof test's own load
  (ADR 0001's), and every other mark runs the sim with no network.

**No Brave checklist.** No claim of this record is judged by a person (the draft's spike section); Mustafa's checklist
for the netcode proof test (prediction, reconciliation and lag-compensated hits at P2, beside the confirmed-hit share,
correction sizes and prediction error) is ADR 0001's, and the boundary spike's is ADR 0002's. PM-0004-69 measures what
a player would see as smoothness, so it needs no judgement here.


## Migration plan

In the slice order (SLICE-ORDER), with the netcode proof test before the foundation and the foundation (GLOSSARY §8)
before slice 1. The legacy game stays frozen and playable throughout; v2 work is additive.

| Phase | Work | Verifiable milestone |
|---|---|---|
| 0 | The drafts signed off by Mustafa as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)) | the sign-off line of this record |
| 1 | Spikes, throwaway in `spikes/` (SPIKE-SCOPE (a)): those that carry this record's marks, in `docs/SPIKE_PLAN.md`'s order, which Mustafa accepted (SPIKE-ORDER (b), 2026-11-03: "in SPIKE_PLAN's order, but wave 1 runs one heavy spike at a time"). In wave 1, before the foundation: the netcode proof test first; then the sim core spike; then the 60 Hz margin run, with the sim core spike's sim and S1's stand-ins (OQ-0001-2 (a)), an early signal only (TICK-60-SETUP, the Advisor, 2026-11-03); then the boundary spike. In wave 3, after the foundation (phase 2) and before slice 1 (phase 3): the content and ability spike | S1–S14 and S19–S22 recorded in this record; S17 from the netcode proof test; S18 from the boundary spike; S15 and S16 from the content and ability spike (wave 3); option C or B, the navigation generator, the units per tick and the hash function chosen from the numbers |
| 2 | The foundation (ADR 0002 builds it): the sim crate in the workspace with the hash-map ban and the float rules as lints, a replay runner, and the first recording in CI | an empty scene runs through the full stack; its per-tick hashes match natively twice and in Wasm |
| 3 | Slice 1, move & shoot: soldier table and ids, the tick and its loops, movement on parry3d against a greybox map document, hitscan and weapons from data in integer time, the local server in the Worker; every match input-driven and recorded | S4, S5, S6 and S9 hold on the real crate; the determinism suite is green on slice 1 recordings on every target |
| 4 | Slice 2, bots and three teams played locally: the bake and the navigation runtime, floors and links, the sight query and pass, avoidance, bot behaviour (after OQ-0004-1), cadences; the Worker runs the whole local match | S2, S3, S10–S12 and S14 hold at the slice's N; reachability on the greybox runs in CI |
| 5 | Slice 3, online: the native room loop, authority-only streams kept off the client, the restore that ADR 0001's prediction scope uses, recovery after a trap | S17 and S8's last mark on the real crate; S20's online recovery; the netcode proof test's marks rerun on the real sim |
| 6 | Slice 4, the loop: economy and objective phases, the ability system's execution, player records, pickups and offers (after ADR 0005's upgrade questions, OQ-0005-AB9 and AB10 among them), events and moments, structure states and damage stages (after OQ-0004-4) | S13, S15 and S16 on the real crate; a recorded full match replays with identical per-tick hashes on every target; scripted cases for the rules LOOP_V2 §2 to §5 state, held as v2's own snapshots (ADR 0002): the $300 start, a delivery paying every player on the team, a rebuild's integer split and its remainder, power-plant pricing, the minute-5 Tech 1 floor (9,000 ticks at 30 Hz: 5 × 60 × 30, arithmetic), a steal by holding E under each setting of OQ-0004-8's switch, 3 s of spawn protection (90 ticks at 30 Hz: 3 × 30, arithmetic), an enemy stopped at a barracks opening, a death respawning as the Recruit with money kept; GE8's presence count against its legacy golden (ADR 0002) |
| 7 | Slice 5, Octopon-3 and Editor v2: the bake in the editor's Test and on save; the validation panel's reach, reachability and 6 m gap checks from the sim crate; tiles only if S10's bake time fails | S19 and S10's gap and reachability marks; the panel's results equal CI's on native and Wasm |
| 8 | Slice 6, scale to 48: the stress scene on the real sim at N = 12, 24, 36, 48 and the N = 64 headroom run; the 60 Hz margin run on the real sim, which makes the final 60 Hz call (TICK-60-SETUP, the Advisor, 2026-11-03: "The final 60 Hz call is measured again with the real sim at slice 6."), run whatever wave 1's early signal showed (the advisor's reading, 2026-11-03; ADR 0001 owns the run and the call) | S1 on the real crate: p99 sim step time ≤ 8.33 ms at N = 48, 3 runs per build; PM-0004-71 and PM-0004-72 in the 60 Hz margin run, where they decide (the pass marks' key) |

## Test plan

- **The determinism suite** (part 9) on every sim change, with ADR 0002's tools (R5-TEST-REC2): recorded matches run
  natively twice in separate processes, as Wasm in Node.js, and as Wasm in Chromium, Firefox and WebKit, with real
  Safari through its WebDriver; a small gate set on every change and a nightly 10-minute bot match; the first mismatch
  reported with a field-level diff; the injected-divergence tests, among them one that changes only one event (a kill
  credited to the wrong slot) and one that changes only one economy value (one player's money, by $1), each caught at
  the tick it happens (proposed: they show that the state hash covers the event list and the economy, parts 17 and
  19).
- **Unit vectors:** every `libm` function the sim uses (at least 10,000 inputs each), every random stream (its first
  10,000 outputs) and the sim's own sampling functions, on the same targets.
- **Property tests:** ids over random spawn and despawn sequences (10,000 cases); snapshot fidelity (restore and
  continue against an uninterrupted run); the time representation (random durations converted and accumulated keep the
  fire-rate mark); player records over random deaths, switches and pickups.
- **Port-parity fuzzes against the legacy SIM:** 20,000 random rays (R4-PHYS-REC2) and 20,000 sight pairs
  (R4-LOS-REC1).
- **Fuzzing for panics:** commands into the sim, and content into ADR 0005's loader, with zero panics as the mark.
- **Benchmarks:** sim step time per part, on one vCPU of the reference server and in Wasm on the minimum laptop; mean,
  p50, p95, p99 and max; 3 runs per build, with the method stated (the bench tool is ADR 0002's); a counting allocator
  for heap allocations per tick.
- **Soak:** the stress scene for 36,000 ticks (20 minutes at 30 Hz), the sim step share and memory recorded.
- **Navigation checks in CI on every map change:** bake time, coverage, reachability from every spawn to every goal,
  gaps under 6 m, a stale-bake check.
- **Network simulation** (latency, jitter, loss) belongs to ADR 0001's tests; the sim meets it in the trap-recovery and
  prediction-restore tests, run under ADR 0001's network profiles.
- **Playtests:** slice 2's local matches with bots, with LOOP_V2 §11's evidence (objective counters, at least 3 runs per
  build).

## Rollback

- **Before acceptance** (the spike is throwaway, so nothing to undo in the game): a failed claim sends this record back
  to Mustafa and the Advisor with its numbers, and the measured alternative takes its place:
  - our own movement fails parity or cost: option B, Rapier's controller over the same static world, measured in the
    same spike;
  - parry3d lacks a needed shape pair: the pair is listed, and its query is written by us (option D for that pair) or
    option B is taken;
  - neither navigation generator passes: the legacy grid, ported and layered (option D of Q5), as the generator, with
    the same runtime;
  - the sim step share fails at N = 48: R1-TICK-REC2's last branch, "back to the drafts", with parallelism inside a
    match (R1-ECS-OQ4) as the measured candidate;
  - cross-target identity fails and cannot be fixed: same-build reproducibility stays required (level B), the client
    is corrected more often, and fixed point comes back only under R1-DET-REC7's revisit rule;
  - the third damage stage fails S13: two baked states, as the research tested, and the third waits for Mustafa
    (OQ-0004-4).
- **After acceptance:** each change lands as its own change-set (CLAUDE.md), so backing out is a revert; the physics
  code behind one module, the navigation runtime behind its queries and the stream kinds in one table keep each swap
  local; recorded matches and their per-tick hashes show whether a rollback restored the earlier behaviour.

## Appendix A: research dispositions

One row per research id this record owns (82 ids, the owner list of the drafts' id table).

| id | disposition | where or why |
|---|---|---|
| R1-ECS-REC1 | changed: marks restated at 30 Hz (TICK-30); its Rapier reasons and mark 5 replaced by R4-PHYS-REC1 (mark 4: size with parry3d, rapier3d or neither); mark 3 kept only if ADR 0001's prediction scope needs whole-world replay | parts 1 and 3; S1, S5, S9 |
| R1-ECS-REC2 | adopted | parts 1 and 9; S7 |
| R1-ECS-REC3 | adopted | part 2; S6 |
| R1-ECS-REC4 | rejected: under R4-PHYS-REC1 no Rapier sets are in the state; it returns only if option B wins the fallback measurement | parts 3 and 10 |
| R1-TICK-REC1 | changed: one time representation with R1-ABL-REC3 (integer sub-tick units, remainders carried); its 36,000 ticks read as 20 minutes of game time at 30 Hz | part 4; S4, S7 |
| R1-TICK-REC3 | adopted | part 4; S3 |
| R1-TICK-REC4 | changed: catch-up bound 0.25 s of game time, 7 ticks at 30 Hz (proposed; 15 is the 60 Hz form); 36,000 ticks are 20 minutes at 30 Hz | part 5; S17 |
| R1-TICK-REC5 | changed: at most 7 ticks per frame at 30 Hz (proposed; 15 is the 60 Hz form); the page reads ADR 0002's block, not shared memory (R1-XU-OQ3 is ADR 0002's); "offline" reads "local play" (SLICE-ORDER) | part 5; S18 |
| R1-TICK-REC6 | adopted | part 21; S19 |
| R1-DET-REC1 | changed: the third match drops, since the largest mode is the 24v24 goal (MODES-3 (a)); 10 minutes are 18,000 ticks at 30 Hz | part 7; S7 |
| R1-DET-REC2 | changed: f32 fixed (R4-PHYS-REC5); the FMA build rules of R1-RR-OQ1 added | part 7; S7 |
| R1-DET-REC3 | changed: the flag is parry3d's `enhanced-determinism`, not Rapier's (R4-PHYS-REC1; critic rank 3) | parts 7 and 10; S7 |
| R1-DET-REC4 | changed: every stream is authority-only unless ADR 0001's prediction scope draws from it; the offers stream is authority-only; raw output mapped by the sim's own functions (R1-DET-OQ5) | part 8; S8 |
| R1-DET-REC5 | adopted | part 9 |
| R1-DET-REC6 | adopted | part 9; S7 (its tools are ADR 0002's) |
| R1-DET-REC7 | adopted | part 7 |
| R1-ABL-REC3 | adopted (one representation with R1-TICK-REC1) | parts 4 and 16; S4, S15 |
| R1-ABL-REC6 | adopted | parts 6 and 16; S15 |
| R1-ECS-OQ1 | spike: the sim core spike records the size (S9); the budget is ADR 0002's (R1-RR-OQ5) | part 1 |
| R1-ECS-OQ2 | answered: settled at the R1 fix step; `clone_from` written by hand | part 3 |
| R1-ECS-OQ4 | spike: the sim core spike (S1); one thread per match unless S1 fails at N = 48 | part 1 |
| R1-ECS-OQ5 | answered: moot with no Rapier state; R1-ECS-REC4's test runs only if option B wins | parts 3 and 10 |
| R1-ECS-OQ6 | answered: the decision does not wait for the unread Overwatch talk; OQ-0004-2's revisit trigger carries the risk it guards | part 1 |
| R1-ECS-OQ7 | open: OQ-0004-2 | part 1 |
| R1-TICK-OQ2 | spike: the boundary spike (S18), stated once with R1-BND-OQ2 | part 5 |
| R1-TICK-OQ5 | spike: the sim core spike (S1, S2); the [server tick time](../research/GLOSSARY.md#server-tick-time) is the netcode proof test's (ADR 0001) | part 6 |
| R1-TICK-OQ9 | answered: no substeps for kinematic, swept vehicles at 30 Hz (R4-PHYS-REC4) | part 10 |
| R1-DET-OQ1 | spike: the sim core spike (S7: parry3d's flag cost) | part 7 |
| R1-DET-OQ2 | spike: the sim core spike (S7: the suite with `simd128` on); off until then | part 7 |
| R1-DET-OQ3 | answered: f32, up to a map 2 km across (R4-PHYS-REC5) | part 7 |
| R1-DET-OQ4 | answered: it matters only if NaN enters state, which part 7 forbids; the suite's browser runs check it | part 7 |
| R1-DET-OQ5 | answered: raw ChaCha8 output mapped by the sim's own sampling functions, with pinned test vectors | part 8 |
| R1-DET-OQ8 | spike: the sim core spike (S7: reference values for the sim's `libm` functions) | part 7 |
| R1-DET-OQ10 | spike: the sim core spike (S7: Wasm's `sqrt` in the unit vectors) | part 7 |
| R1-DET-OQ11 | spike: the sim core spike (S8: ChaCha8's cost); the question returns only if the cost asks for a fallback | part 8 |
| R1-BND-OQ2 | spike: the boundary spike (S18), stated once with R1-TICK-OQ2 | part 5 |
| R1-ABL-OQ5 | answered: f32 under part 7's rules, with a fixed aggregation order per attribute | parts 7 and 16 |
| R1-ABL-OQ7 | answered: under TICK-30 no legacy fire interval is shorter than a tick; the loader's rounding report flags any future one | part 4 |
| R1-ABL-OQ10 | answered: active effects are rows in an ordered pool on their target | part 1 |
| R1-XU-OQ1 | answered: settled by the glossary (one stress scene; one server tick budget with its shares) | part 6 |
| R1-XU-OQ5 | answered: settled at the R1 fix step (the std `HashMap` seed is per instance) | part 9 |
| R1-XU-OQ6 | answered: settled at the R1 fix step (both articles are tier "other"); nothing here depends on it | part 9 |
| R1-XU-OQ7 | answered: one representation, one mark | part 4; S4 |
| R1-XU-OQ11 | answered: no; `dt` is one constant with the same bits on every target, and timers count integer units; 64 Hz is R1-TICK-OQ1, ADR 0001's | part 7 |
| R1-XU-OQ12 | answered: each question stated once: hidden tabs here (S18), the reference hardware with ADR 0003 and ADR 0006, offline play a design question | part 5 |
| R1-RR-OQ1 | answered: baseline x86-64, no `+fma` or `-C target-cpu=native`, glam's `scalar-math`, native AArch64 in the hash test before ARM hosts; S7 proves it | part 7 |
| R1-RR-OQ3 | answered: no panic reachable from input or content; online, the Worker restarts and resyncs; local play is OQ-0004-7 | part 20; S20 |
| R1-XT-OQ2 | answered: the list of R1 items that assumed Rapier, and what each becomes | part 10 |
| R1-XT-OQ12 | answered: this record owns the restart; ADR 0001 the resync path; ADR 0002 the panic strategy | part 20 |
| R2-SYNC-OQ13 | answered: bot reaction is tuning data the game-designer sets; S14 reports it at N = 48 | part 15 |
| R2-XT-OQ12 | answered: as R1-XT-OQ12 | part 20 |
| R4-PHYS-REC1 | adopted | part 10; S9 |
| R4-PHYS-REC2 | adopted | parts 3 and 10; S9 |
| R4-PHYS-REC3 | adopted | part 10; S9 |
| R4-PHYS-REC4 | adopted (N = 48, MODES-3 (a)) | part 10; S9 |
| R4-PHYS-REC5 | adopted | part 7; S9 |
| R4-NAV-REC1 | adopted | parts 11 and 21; S10 |
| R4-NAV-REC2 | adopted | part 11; S10 |
| R4-NAV-REC3 | adopted (N = 48, MODES-3 (a)) | part 11; S10 |
| R4-NAV-REC4 | changed: SLICE-ORDER makes local play with bots a slice 2 need, so the browser's local-server build carries navigation from slice 2 | part 11; S10 |
| R4-FLOORS-REC1 | adopted | part 12; S11 |
| R4-FLOORS-REC2 | adopted | parts 12 and 14; S11 |
| R4-FLOORS-REC3 | adopted | part 12; S11 |
| R4-LOS-REC1 | adopted | part 13; S12 |
| R4-LOS-REC2 | adopted | part 13; S12 |
| R4-LOS-REC3 | adopted | part 13; S12 |
| R4-OBST-REC1 | adopted | part 14; S13 |
| R4-OBST-REC2 | changed: from two baked states to one per damage stage the map declares (three for LOOK_V2 §4), each structure's stages baked over its own nodes; the third stage is a proposed mark | part 14; S13 |
| R4-OBST-REC3 | adopted | part 14 |
| R4-PHYS-OQ2 | spike: the sim core spike (S9: R4-PHYS-REC1 marks 2, 4 and 5) | part 10 |
| R4-PHYS-OQ3 | spike: the sim core spike (S9 mark 3; S7's suite) | part 10 |
| R4-PHYS-OQ4 | open: OQ-0004-3 | part 10 |
| R4-PHYS-OQ5 | answered: no dynamics until a VISION_V2 §6 item asks; the physics code sits behind one sim module | part 10 |
| R4-NAV-OQ1 | spike: the sim core spike (S10: R4-NAV-REC1 marks 1–6) | part 11 |
| R4-NAV-OQ2 | spike: the sim core spike (S10: rerecast built for the browser Worker and compared) | part 11 |
| R4-NAV-OQ3 | spike: the sim core spike (S10: R4-NAV-REC2's marks at N = 48 and the N = 64 headroom run) | part 11 |
| R4-NAV-OQ4 | answered: polyanya is not used at run time; rust-reviewer audits its maps before any later adoption | part 11 |
| R4-LOS-OQ5 | spike: the sim core spike (S12: R4-LOS-REC1 mark 4) | part 13 |
| R4-OBST-OQ4 | spike: the sim core spike (S10 and S13: a moving hull as an obstacle polygon or as circles) | part 11 |
| R4-XT-OQ2 | answered: as R1-XT-OQ2 | part 10 |
| R5-XT-OQ1 | answered: as R1-XT-OQ2 | part 10 |
| R6-XT-OQ6 | answered: as R1-XT-OQ12 | part 20 |

## Sources

Research files at commit 9e7db28 (unchanged in this PR), and the sections read for this record:
- `docs/research/r1-simulation-core.md`: header and TL;DR (lines 1–67); Findings R1-BND-F19 (lines 1019–1028) and
  R1-DET-F14 (lines 679–683); Options, units `ecs`, `tick`, `determinism` (lines 1626–1730); Recommendations, units
  `ecs`, `tick`, `determinism` (lines 1884–2248) and `abilities` (lines 2340–2480); Open questions, units `ecs`, `tick`,
  `determinism` (lines 2608–2717), R1-BND-OQ2 (lines 2724–2727), R1-ABL-OQ5 to OQ10 (lines 2770–2784), the conflicts
  between units (lines 2830–2912), the rust-reviewer risks (lines 2914–2961), the cross-topic questions (lines
  3011–3112).
- `docs/research/r4-physics-navigation.md`: header and TL;DR (lines 1–50); Findings, Rapier and parry3d (lines 53–233),
  navigation (lines 296–408), obstacles (lines 536–584); Options (lines 585–662); Recommendations (lines 663–1076); Open
  questions, the shared question and the cross-topic questions (lines 1078–1214); wave-1 items R4 changes (lines
  1216–1237); the verifier's spot-check summary (lines 1292–1310).
- `docs/research/r2-netcode.md`: R2-SYNC-REC3 (lines 2068–2105), R2-INT-REC2 and REC3 (lines 2445–2483),
  R2-ROOM-REC2 (line 2762), R2-ROOM-REC6 (lines 2842–2856), R2-SYNC-OQ13 (lines 2952–2954), R2-XT-OQ12 (lines
  3395–3398).
- `docs/research/r5-toolchain-quality.md`: R5-LAYOUT-REC2 (lines 1170–1193), R5-LAYOUT-REC4 (lines 1206–1218),
  R5-RUSTLINT-REC2 (lines 1321–1339), R5-TEST-REC2 (lines 1412–1430), R5-XT-OQ1 (lines 1963–1970).
- `docs/research/r6-infrastructure.md`: R6-OBS-REC3 (lines 754–768), R6-TEL-REC1 (lines 784–811), R6-XT-OQ6 (lines
  1021–1024).
- `docs/research/GLOSSARY.md`: §1 tick budgets (lines 46–264), §2 the largest mode and headroom (lines 335–374), §3 the
  stress scene and its counts (lines 395–482), §4 the authority, the prediction scope, the stream kinds and the state
  hash (lines 483–559), §7 the hash-map ban (lines 727–742), and §8 as the session appends it (the foundation, the
  netcode proof test, the 60 Hz margin).
- `docs/research/README.md` (the topic statuses).

Design documents and records: `docs/VISION_V2.md` (§3, §4, §6), `docs/LOOP_V2.md` (§2–§11), `docs/EDITOR_V2.md` (§2,
§3, §5), `docs/LOOK_V2.md` (§3, §4, §8), `docs/STATUS.md` ("Decisions pending"; "Open bugs and risks"),
`docs/LEGACY_INVENTORY.md` (rows CS1–CS11, GW9, GW13–GW17, GW20, GC1–GC5, GC7, GC11–GC17, GN4–GN14, GE8, GP9, GP13,
GP14, CM11, CM13, CD4–CD14, TL13–TL15), `docs/adr/0000-template.md`.
