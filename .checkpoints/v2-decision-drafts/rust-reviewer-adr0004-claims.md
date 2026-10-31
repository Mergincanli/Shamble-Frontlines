# Claims ledger: ADR-0004 (the simulation core)

Author: rust-reviewer, 2026-10-31. A notes file beside the draft (`rust-reviewer-adr0004-draft.md`), not part of it.
One row per claim or number in the draft. Lines are at commit 9e7db28 (the research files do not change in this PR;
checked with `sed -n`). Tiers: `research verified` (an R1 or R2 recommendation claim, or a finding its topic's verifier
table lists as checked: `inputs/findings-checked.tsv`) · `research, source to check` (any other research text: R3–R6
recommendations and findings outside their checked set, R1 and R2 open questions and RR risks, which their verifiers
did not check) · `arithmetic` (shown) · `settled` (a STATUS code or GLOSSARY §8) · `design doc` (VISION_V2, LOOP_V2,
EDITOR_V2, LOOK_V2) · `proposed mark` (with its reason). Where one claim rests on several tiers, the weakest is given
and the others named. "r1" = `docs/research/r1-simulation-core.md`, "r2" = `r2-netcode.md`, "r4" =
`r4-physics-navigation.md`, "r5" = `r5-toolchain-quality.md`, "r6" = `r6-infrastructure.md`, "GL" = `GLOSSARY.md`.

## Summary paragraph and Context

| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 1 | One Rust simulation runs natively on the server and as Wasm in the browser's Worker | Summary; Context | STATUS "Decisions pending" (the stack; SLICE-ORDER condition 1) | settled |
| 2 | State as plain typed tables in one struct, no ECS crate | Summary | r1, R1-ECS-REC1, 1891–1950 | research verified |
| 3 | Same seed, data and inputs give the same state hash after every tick on every run of a build; Wasm–native identity a tested target | Summary | r1, R1-DET-REC1, 2129–2145 | research verified |
| 4 | parry3d plus our own kinematic movement, no physics engine | Summary | r4, R4-PHYS-REC1, 672–719 | research, source to check (rests on R4-PHYS-F1–F4, F19, F21–F24, checked; F20 corrected at R4's fix pass) |
| 5 | Navigation baked into the map document and queried by the sim | Summary | r4, R4-NAV-REC1, 802–843 | research, source to check |
| 6 | One height-aware sight query serves bots, blasts, melee and the aim point | Summary | r4, R4-LOS-REC1, 958–988; R4-LOS-REC3, 1008–1018 | research, source to check |
| 7 | VISION_V2 §6 asks for a true 3D world, data-driven abilities, big vehicles, a server-side economy, telemetry and moments | Summary; Context | VISION_V2 §6.2–§6.6, §6.8 | design doc |
| 8 | SLICE-ORDER: movement and shooting input-driven and replayable from day one | Summary; Context | STATUS "Decisions pending", SLICE-ORDER condition 2 | settled |
| 9 | §6.1 modes as data; §6.2's list; §6.3; §6.4 seats `[OPEN]`; §6.5; §6.6; §6.8 kill-cams, replays, highlights `[LATER]`; §6.9 a few very large units | Context | VISION_V2 §6.1–§6.9 | design doc |
| 10 | §6.9: the sim's share is that very large units are in the stress scene | Context | GL §3, the stress scene, 397–437 (4 very large units) | research verified (GL definition from R1-TICK-REC2, R3-CHAR-REC4) |
| 11 | VISION_V2 §3 "Bots fill empty slots"; §4 fair up to about 100 ms of ping; minimum laptop runs the largest mode at 30+ fps | Context | VISION_V2 §3, §4 | design doc |
| 12 | LOOP_V2 §2, §3–§4, §5–§6, §7 (active reload, split feedback), §8 ("Nothing carries into the next match"), §9 (plateau, ramps, drop-downs, 6 m gaps, reachability), §10 (bots' tasks; replan storm first) | Context | LOOP_V2 §2–§10 | design doc |
| 13 | EDITOR_V2 §3: the validation panel (bot reachability, 6 m gaps); Test plays with bots at once | Context | EDITOR_V2 §3 | design doc |
| 14 | LOOK_V2 §3 "Buildings sized for play: interiors, upper floors, verticality"; §4 ruins, collapsed roofs, planks; damage stages `[DEFAULT]`; roofs fade `[DEFAULT]` | Context | LOOK_V2 §3, §4 | design doc |
| 15 | SLICE-ORDER's three conditions (client plus local server; input-driven and replayable; the netcode proof test before the foundation) | Context | STATUS "Decisions pending", SLICE-ORDER; GL §8 (the foundation, the netcode proof test) | settled |
| 16 | TICK-30: 30 Hz baseline; 60 Hz only with the 60 Hz margin of at least 25 %, owned and measured by ADR 0001 | Context | STATUS "Decisions pending", TICK-30; GL §8, the 60 Hz margin | settled |
| 17 | MODES-3 (a): largest mode 24v24, N = 48 | Context | STATUS, MODES-3 (a) | settled |
| 18 | SEC-SPREAD (a): spread known only to the server ("server-only spread") | Context | STATUS, SEC-SPREAD (a) | settled |
| 19 | ABL-PREDICT open until a Feel Lab test | Context | STATUS, ABL-PREDICT | settled |
| 20 | UPGRADE-SCOPE's content (tracks per unit and weapon type, pick one of three, owned by the player, survive death and switching, reset every match, class's own weapon, drops as world pickups, progress on the weapon type, PlayerState not Pawn; three items open) | Context | UPGRADE-SCOPE, Mustafa 2026-10-31 (inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md; STATUS once docs-keeper records it) | settled |
| 21 | SPIKE-SCOPE (a): spikes throwaway, exempt from legacy constraints; R7-MARKS (a): marks are working recommendations | Context | STATUS, SPIKE-SCOPE (a), R7-MARKS (a) | settled |
| 22 | Neither ECS crate orders its queries or clones its world; bevy's Entity must not be lined up between instances | Context, finding 1 | r1, R1-ECS-REC1, 1891–1950 | research verified |
| 23 | std transcendental functions vary by platform and differ between Wasm and Linux; NaN bits not deterministic; relaxed SIMD hardware-dependent; every std `HashMap` instance its own seed; `libm` one implementation | Context, finding 2 | r1, R1-DET-REC2, 2146–2169; R1-DET-REC5, 2208–2221 | research verified |
| 24 | With server authority a mismatch costs a correction; lockstep would make it a desync | Context, finding 3 | r1, R1-DET-REC1, 2129–2145 | research verified |
| 25 | No VISION_V2 §6 item needs a rigid-body dynamics solver | Context, finding 4 | r4, Options, the needs list, 589–604 | research, source to check (the author's reading, labelled inferred there) |
| 26 | Rapier's controller is only ray and shape casts around a caller's move | Context, finding 4 | r4, R4-PHYS-F1, 56–60 | research verified (checked: confirmed) |
| 27 | The browser target has no C/C++ toolchain, so Detour cannot simply be linked into the sim's Wasm build | Context, finding 5 | r4, R4-NAV-F8, 303–311 | research verified (checked: confirmed; the Detour consequence inferred there) |
| 28 | No Rust navigation crate read promises determinism | Context, finding 5 | r4, R4-NAV-F16, 362–365 | research, source to check (an absence; not checked) |
| 29 | The Wasm target builds with `-Cpanic=abort`, so one panic ends the client sim's instance | Context, finding 6 | r1, R1-BND-F19, 1019–1028; R1-RR-OQ3, 2936–2940 | research verified (F19 checked: confirmed) |
| 30 | Takes the tick rate, prediction scope, messages, room lifecycle (0001), definitions (0005), Worker, block, panic strategy, profiles, lints (0002), reference server (0006, R6-HOST-REC2), client machines (0003, R3-BUDGET-REC1) | Context | decision map a (0004 "Takes"), seams S1, S2, S5 (notes) | settled (the drafts' division, plan rule and decision map) |
| 31 | Gives the tick mechanism, the sim step share and parts, the client's sim share (ADR 0001 adds the share into the server tick budget), ids, the state hash and suite, events and moments, the aim ray query, map-document parts, recorded matches, the player record | Context | decision map seam S1, interfaces I-9, I-10, I-16, I-17, I-19, I-20, I-21; J5 (notes) | settled (the drafts' division) |
| 32 | Legacy declared a fixed step that nothing uses; the loop runs on variable frame time | Context, legacy | LEGACY_INVENTORY CS1 (line 169) | research verified (R7 inventory row, [C]; R1-TICK-REC1 rests on it) |
| 33 | At 20 Hz the legacy jump lost 11 % of its apex | Context, legacy | r1, Options 4a, line 1653 (GW14) | research, source to check (an options-table legacy fact) |
| 34 | The reach formula overstates the integrated jump by about 9 cm at 60 Hz | Context, legacy | r4, R4-FLOORS-REC1, 903–923 (line 913; GW14) | research, source to check |
| 35 | Reset-not-carry loses up to 28 % of the Shabzon's rate at 30 Hz | Context, legacy | r1, R1-ABL-REC3, 2402–2419 (line 2411; GC5) | research verified |
| 36 | The game reads the wall clock in 11 places | Context, legacy | STATUS "Open bugs and risks" (line 273) | settled (a STATUS record; the inventory's code read) |
| 37 | Gameplay shares `Math.random` with effects and audio (GP14, GN8) | Context, legacy | r1, R1-DET-REC4, 2184–2207 | research verified |
| 38 | Shots, sight and blasts use different worlds (CS9, GN7, GW15); statics have no ids (CS7); bots neither hit nor avoid the Juggernaut (GC17) | Context, legacy | r4, R4-PHYS-REC2, 721–745; R4-OBST-REC1, 1022–1041 | research, source to check (legacy rows, [C]/[M] in the inventory) |
| 39 | The 2 m grid closes gaps of 3 to 5.4 m; 27.5 % of objective routes cross a roof bots cannot climb | Context, legacy | r4, legacy facts (R4-NAV), 391–393 (GN11) | research, source to check |
| 40 | The route budget resets per frame and denies 52–57 % of routes at 48 soldiers | Context, legacy | r4, legacy facts (R4-NAV), 397–398 (GN13) | research, source to check |
| 41 | About 2.5 scans per bot per second at 48 soldiers against 4 at 12 | Context, legacy | r4, R4-LOS-REC2, 990–1006 (GN4) | research, source to check |
| 42 | The legacy JS step took 0.80–1.00 ms mean at 48 soldiers, no physics or 3D world; a reference, not a prediction | Context, legacy | r1, R1-ECS-REC1 mark 1, 1930–1931 (GC2); R1-TICK-OQ5, 2658–2660 | research verified (REC1 claim); OQ5 research, source to check |
| 43 | STATUS's ADR-001 inputs: one hit world (K1), phased `update()` (K4), boot-time `Math.random`, UUIDs and the FX cull on the shared generator, `addTech`'s HUD writes, a sight budget sized for 12 | Context, legacy | STATUS "Decisions pending", ADR-001 entry (line 199) | settled (a STATUS record) |
| 44 | Parts 1, 8, 10, 13, 17 and 19 dispose of them; the team allow-list is ADR 0001's | Context | decision map J2, ADR-001 row (notes) | settled (the drafts' division) |

## Decision, parts 1 to 8

| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 45 | One crate runs natively in each room process and as Wasm in the Worker, for prediction and the local server | Decision intro | STATUS, the stack and SLICE-ORDER condition 1; r2, R2-ROOM-REC2 (line 2762: one room per process) | settled (R2-ROOM-REC2 research verified) |
| 46 | One state struct: fixed-slot soldier table sized from the mode; generational pools for big vehicles and parts, live projectiles, pickups, active effects; per-team tables; static colliders with ids; RNG state; tick counter; event list | Part 1 | r1, Options ECS table, option C row "Shape", 1630–1644; R1-ECS-REC1, 1891–1950 | research verified (pickups and player records added per UPGRADE-SCOPE: settled) |
| 47 | Active effects as rows in an ordered pool on the target (owner id, definition id, timers, stacks) | Part 1 | r1, Options ECS table, "Data-driven abilities" row, option C (1630–1644); R1-ABL-OQ10, 2783–2784 | research, source to check (an options-table cell; inferred there) |
| 48 | Phases plain functions in one fixed order, starting from the legacy order (GP14) | Part 1 | r1, R1-ECS-REC1, 1891–1950; R1-DET-REC5, 2208–2221 | research verified |
| 49 | No globals or statics in the sim crate (nothing outside the struct) | Part 1 | r1, Options ECS table, option C "Whole-state snapshot" cell (inferred: "a code rule"), 1630–1644 | research, source to check |
| 50 | One match on one thread; parallelism only if the spike shows the share cannot hold at N = 48 | Part 1 | r1, R1-ECS-OQ4, 2622–2626; r2, R2-ROOM-REC2, line 2762 | research, source to check (OQ text); the condition is a proposed spike outcome |
| 51 | Why: kinds few and known; ECS crates leave order unspecified, cannot clone their world; plain struct gives cheap snapshots and order | Part 1 | r1, R1-ECS-REC1, 1891–1950 | research verified |
| 52 | Need for part 1: VISION_V2 §6.1–§6.5, §6.8 | Part 1 | r1, R1-ECS-REC1 "Serves", 1891–1950 | research verified |
| 53 | Id = (kind, index, generation), sim-owned, every entity incl. static colliders; used on server, predictor, snapshots, events, block, TS client; no library id on the wire | Part 2 | r1, R1-ECS-REC3, 1973–1991 | research verified |
| 54 | A soldier's slot index is its sim index (GC2) | Part 2 | LEGACY_INVENTORY GC2 v2 note ("Keep slot id equal to sim id"), line 241 | research verified (R7 row, [M]) |
| 55 | Wire widths of ids and the tick are ADR 0001's (R1-XT-OQ11) | Part 2 | r1, R1-XT-OQ11, 3079–3083 | research, source to check |
| 56 | Why: bevy's Entity not lined up across instances; generational keys reject stale handles; legacy cannot name the static a shot hit (CS7) | Part 2 | r1, R1-ECS-REC3, 1973–1991 | research verified |
| 57 | Hand-written `clone_from` into a preallocated copy; 0 heap allocations after warm-up; the derived one would not reuse buffers (R1-ECS-OQ2, settled at the fix step) | Part 3 | r1, R1-ECS-REC1 mark 2, 1932–1937; R1-ECS-OQ2, 2614–2618; R1-ECS-F35 (line 274, checked: confirmed) | research verified |
| 58 | Static and movers' indexes rebuilt after a restore, never stored in a snapshot | Part 3 | r4, R4-PHYS-REC2, 721–745 | research, source to check |
| 59 | With no Rapier sets, R1-ECS-REC4 and REC1 mark 2's separate Rapier count fall away | Part 3 | r4, wave-1 items R4 changes, 1216–1237 (first bullet); R4-PHYS-REC1, 672–719 | research, source to check |
| 60 | Whole-world replay kept only if ADR 0001's scope needs it; dropped under R2-SYNC-REC3's scope (own movement, own weapon state, own vehicle) | Part 3 | r1, R1-ECS-REC1 mark 3, 1938–1941; R1-XT-OQ7, 3057–3062; r2, R2-SYNC-REC3, 2068–2105 | research verified (REC1, R2-SYNC-REC3); the conditional drop follows R1-XT-OQ7's own "ADR-0004 keeps or drops" |
| 61 | Why: the client replays its own commands from the server's state on every snapshot | Part 3 | r2, R2-SYNC-REC3, 2068–2105 | research verified |
| 62 | Need for part 3: VISION_V2 §4 only, and SLICE-ORDER condition 2 | Part 3 | r1, R1-ECS-REC1 "Serves" (§4), 1891–1950; STATUS SLICE-ORDER | research verified; settled |
| 63 | `u32` tick counter the only clock; sim time = tick × `dt`; one build-time `dt` on server and client; no time argument, no clock read | Part 4 | r1, R1-TICK-REC1, 2009–2027 | research verified |
| 64 | Tick rate ADR 0001's under TICK-30; never a server option; never changes on a content reload | Part 4 | STATUS TICK-30; r1, R1-TICK-REC1 (Valve removed `-tickrate`, F13), 2009–2027; R1-XU-OQ10, 2895–2898 | settled; research verified |
| 65 | A `u32` counter wraps after 2^32 ÷ 30 s, about 4.5 years, at 30 Hz | Part 4 | 4,294,967,296 ÷ 30 = 143,165,577 s = 1,657 days = 4.54 years | arithmetic |
| 66 | A match lasts about 20 minutes | Part 4 | LOOP_V2 §8 ("the timer (about 20 min)") | design doc |
| 67 | Counters and timers use explicit wrapping or checked operations | Part 4 | r1, R1-DET-REC2, 2146–2169 | research verified |
| 68 | Durations authored in seconds, converted at load to integer sub-tick units with the tick an exact whole number; rounding reported; remainders carried | Part 4 | r1, R1-ABL-REC3, 2402–2419; R1-TICK-REC1, 2009–2027; LOOP_V2 (one data table, line 8) | research verified |
| 69 | The two are one representation (R1-XU-OQ7) | Part 4 | r1, R1-XU-OQ7, 2880–2885 | proposed mark (this record's answer: the integer unit of ABL-REC3 carries TICK-REC1's remainders exactly) |
| 70 | Units per tick set in the sim core spike from the rounding report (no research number) | Part 4 | none | proposed mark (no number in the research, so a spike decides it) |
| 71 | Effects on tick boundaries, fixed order: entity index, start tick, per-match sequence number | Part 4 | r1, R1-ABL-REC3, 2402–2419 | research verified |
| 72 | Slow systems every k ticks from a cadence in seconds, staggered by slot (bot scans, sight budgets, replans, economy trickle, telemetry sampling) | Part 4 | r1, R1-TICK-REC3, 2066–2076 | research verified |
| 73 | Legacy fire intervals 0.072 s and 0.06 s are longer than a 30 Hz tick, 0.0333 s | Part 4 | r1, R1-ABL-OQ7, 2776–2778 (0.033 s there); 1 ÷ 30 = 0.0333 s | research, source to check; arithmetic |
| 74 | The rounding report flags any future interval under one tick | Part 4 | r1, R1-ABL-REC3 ("reports the rounding of each value"), 2402–2419 | proposed mark (this record's answer to R1-ABL-OQ7) |
| 75 | Variable steps change outcomes (GW14, GC15); reset-not-carry loses up to 28 % (GC5) | Part 4 | r1, R1-TICK-REC1, 2009–2027; R1-ABL-REC3, 2402–2419 | research verified |
| 76 | Server loop wakes on a timer, takes due ticks from a monotonic clock | Part 5 | r1, R1-TICK-REC4, 2078–2092 | research verified |
| 77 | Catch-up bounded at 0.25 s of game time = 7 ticks at 30 Hz; the research's 15 is the 60 Hz form | Part 5 | r1, R1-TICK-REC4, 2078–2092; ⌊0.25 × 30⌋ = 7 | arithmetic; proposed mark (the largest whole number within the research's bound) |
| 78 | A longer stall drops time and records it; overruns and drops are monitoring counters (ADR 0006) | Part 5 | r1, R1-TICK-REC4, 2078–2092 | research verified |
| 79 | Snapshots go out after the tick (ADR 0001) | Part 5 | r1, R1-TICK-REC4, 2078–2092 | research verified |
| 80 | Worker owns the tick: `performance.now()` accumulator woken by `setTimeout` (W2); `Atomics.wait` (W3) measured alternative | Part 5 | r1, R1-TICK-REC5, 2094–2116 | research verified |
| 81 | W3 needs shared memory, so isolation, ADR 0002's call (R1-XU-OQ4) | Part 5 | r1, R1-XU-OQ4, 2861–2865; Options 4d W3, 1673–1682 | research, source to check |
| 82 | rAF is the render clock only | Part 5 | r1, R1-TICK-REC5, 2094–2116 | research verified |
| 83 | After a long gap: local play pauses; online resync via ADR 0001's path; catch-up per frame bounded at 7 ticks | Part 5 | r1, R1-TICK-REC5 ("offline it pauses"; "catch-up per frame bounded as in REC4"), 2094–2116; ⌊0.25 × 30⌋ = 7 | research verified; arithmetic; "local play" for "offline" per SLICE-ORDER (settled) |
| 84 | Pause, shop and hit-stop never stop the shared tick online; hit-stop is presentation time | Part 5 | r1, R1-TICK-REC5, 2094–2116 | research verified |
| 85 | The 11 wall-clock reads stay in the frozen legacy game | Part 5 | STATUS line 273; CLAUDE.md (legacy frozen) | settled |
| 86 | Hidden-tab timer rate measured in the boundary spike, 60 s and more than 5 minutes (stated once) | Part 5 | r1, R1-TICK-OQ2, 2645–2647; R1-BND-OQ2, 2724–2727; R1-XU-OQ12, 2903–2912 | research, source to check |
| 87 | Why: Tokio's timers are millisecond-granular; rAF follows the display and stops in background tabs | Part 5 | r1, R1-TICK-REC4, 2078–2092; R1-TICK-REC5, 2094–2116 | research verified |
| 88 | Need for part 5: §6.5, §6.9 (render loop never blocked), §4 | Part 5 | r1, R1-TICK-REC4 and REC5 "Serves", 2078–2116 | research verified |
| 89 | p99 sim step time ≤ the sim step share, 25 % of the tick period: 8.3 ms at 30 Hz (4.2 ms in the 60 Hz margin run), one vCPU of the reference server, the stress scene at N = 48 | Part 6 | r1, R1-TICK-REC2 P1, 2044–2053; GL, 189–213 | research verified |
| 90 | Parts: movement and physics ≤ 1 ms native mean, p99 reported | Part 6 | r4, R4-PHYS-REC1 mark 2, 700–703; GL line 55 | research, source to check |
| 91 | Avoidance ≤ 0.2 ms | Part 6 | r4, R4-NAV-REC3, 867–886; GL line 56 | research, source to check |
| 92 | Abilities, effects and weapons ≤ 1 ms | Part 6 | r1, R1-ABL-REC6, 2465–2480; GL line 57 | research verified |
| 93 | Navigation ≤ 5 % of the tick period (1.67 ms) | Part 6 | r4, R4-NAV-REC2, 845–865; GL line 58; 0.05 × 33.33 = 1.67 | research, source to check; arithmetic |
| 94 | Visibility pass ≤ 5 % (1.67 ms); its id ADR 0001's, its number this record's | Part 6 | r2, R2-INT-REC3, 2467–2483; GL line 59; decision map LE-M (notes) | research verified; settled (the division) |
| 95 | Ceilings sum to 5.53 of 8.33 ms, a planning check, not a bound (p99s do not add) | Part 6 | GL line 60 and Time statistics, 141–166; 1.00 + 0.20 + 1.00 + 1.67 + 1.67 = 5.53 | arithmetic (GL's figure) |
| 96 | The rest of the sim takes 2.80 ms (8.33 − 5.53) | Part 6 | 8.33 − 5.53 = 2.80 | arithmetic |
| 97 | At 60 Hz the parts sum to 3.87 of 4.17 ms | Part 6 | GL line 60 | arithmetic (GL's figure) |
| 98 | ADR 0001 adds this share to the server-only share in the server tick budget | Part 6 | GL, the server tick budget, 167–188; decision map seam S1 (notes) | research verified (GL resolution 1); settled (the division) |
| 99 | Client's sim share: p99 ≤ 40 % of the tick period, 13.3 ms at 30 Hz (6.7 ms at 60 Hz), Wasm in the Worker on the minimum laptop | Part 6 | r1, R1-TICK-REC2 P2, 2054–2057; GL, 214–224 | research verified |
| 100 | Wasm at most 2 × native | Part 6 | r1, R1-ECS-REC1 mark 1, 1923–1931 | research verified |
| 101 | It applies wherever the browser runs the whole sim: the local server of slices 1 and 2 | Part 6 | STATUS SLICE-ORDER (slice 2 "played locally"; condition 1) | settled |
| 102 | N for that mark set by R1-TICK-OQ7, a design question | Part 6 | r1, R1-TICK-OQ7, 2667–2670; id table: owner DESIGN | research, source to check |
| 103 | The page's cost of the sim per frame is ADR 0003's row | Part 6 | r3, R3-BUDGET-REC2 (r3:1127); decision map N12 (notes) | research, source to check |
| 104 | The step makes no heap allocation in steady state | Part 6 | r1, R1-ABL-REC6 (for its part), 2465–2480 | proposed mark (extends REC6's rule to the whole step; reason: a 33 ms loop must not stall on the allocator) |
| 105 | Need for part 6: VISION_V2 §3 and §4 only (a budget) | Part 6 | r1, R1-ABL-REC6 "Serves" ("no §6 item, since it is a budget"), 2465–2480 | research verified |
| 106 | Same-build reproducibility required; Wasm–native identity a target tested on every change; netcode never depends on it; no lockstep | Part 7 | r1, R1-DET-REC1, 2129–2145 | research verified |
| 107 | No fixed point | Part 7 | r1, R1-DET-REC7, 2242–2248 | research verified |
| 108 | f32 throughout, up to a map 2 km across | Part 7 | r4, R4-PHYS-REC5, 786–798 | research, source to check (F26 checked: confirmed) |
| 109 | Allowed ops list; transcendental maths only via pinned `libm`; no std sin/cos/atan2/powf/exp, no `algebraic_*`, fast-math, `relaxed-simd`; `simd128` off until the suite passes | Part 7 | r1, R1-DET-REC2, 2146–2169 | research verified |
| 110 | NaN never enters state; finite checks at entry; debug and test builds assert finite state each tick; no sim value in TypeScript | Part 7 | r1, R1-DET-REC2, 2146–2169 | research verified |
| 111 | Rust never fuses on its own; wide's `mul_add` fuses only where the target has FMA | Part 7 | r1, R1-DET-REC2 (F11), 2146–2169; R1-RR-OQ1, 2923–2932 | research verified (the Rust part); research, source to check (the wide part, RR text not verifier-checked) |
| 112 | Server builds for baseline x86-64, no `+fma` or `-C target-cpu=native`; glam's `scalar-math`; native AArch64 in the hash test before ARM hosts; flags ADR 0002's | Part 7 | r1, R1-RR-OQ1 (the reviewer's proposal), 2923–2932 | research, source to check (adopted here as the decision) |
| 113 | parry3d's own `enhanced-determinism` flag on both targets (critic rank 3) | Part 7 | r4, R4-PHYS-REC1, 672–719; r1, R1-DET-F23 (line 740, checked: confirmed); R1-XT-OQ2, 3022–3029 | research verified (F23) |
| 114 | Attribute maths in f32 under these rules with a fixed, documented aggregation order per attribute | Part 7 | r1, R1-ABL-OQ5, 2770–2771; R1-ABL-REC1 (aggregation order fixed and documented), 2342–2378 | research verified (ABL-REC1); the f32 answer is this record's |
| 115 | An exactly representable `dt` does not matter for identity: one constant, same bits on every target; timers count integer units | Part 7 | r1, R1-XU-OQ11, 2899–2902; R1-DET-REC2's reason (basic operations exactly specified), 2146–2169 | proposed mark (this record's inference on REC2's reason; the suite tests it) |
| 116 | Whether 64 Hz is worth it is R1-TICK-OQ1, ADR 0001's | Part 7 | r1, R1-TICK-OQ1, 2640–2643; id table owner 0001 | research, source to check |
| 117 | Browser Wasm float profile matters only if NaN enters state; the suite's browser runs check it | Part 7 | r1, R1-DET-OQ4, 2689–2690 | research, source to check |
| 118 | Why: basic ops, `sqrt`, `mul_add` exactly specified; std transcendental not; NaN bits and relaxed SIMD differ; a divergence costs a correction | Part 7 | r1, R1-DET-REC2, 2146–2169; R1-DET-REC1, 2129–2145 | research verified |
| 119 | Need for part 7: §6.2, §6.3, §6.4; §4 | Part 7 | r1, R1-DET-REC2 "Serves", 2146–2169 | research verified |
| 120 | `ChaCha8Rng` pinned, seeded by the authority per match, one stream per purpose | Part 8 | r1, R1-DET-REC4, 2184–2207 | research verified |
| 121 | Raw output mapped by the sim's own sampling functions with pinned test vectors; rand's distributions carry no stability statement; uniform floats "may equal `high`" | Part 8 | r1, R1-DET-OQ5, 2691–2696 (rust-reviewer risk 9) | research, source to check |
| 122 | The streams' state is part of the authority's state and its hash | Part 8 | r1, R1-DET-REC4, 2184–2207; GL state hash, 546–559 | research verified |
| 123 | A stream is authority-only unless ADR 0001's prediction scope makes the predicting sim draw from it | Part 8 | GL, shared stream and authority-only stream, 525–545 | proposed mark (this record's rule for assigning the GL kinds) |
| 124 | Today's scope (R2-SYNC-REC3) draws from no sim stream | Part 8 | r2, R2-SYNC-REC3, 2068–2105 (own movement, own weapon state, own vehicle, cosmetic shot feedback from a client-side cosmetic spread) | research verified (the scope); "draws from none" is this record's reading |
| 125 | Streams: match setup, weapon spread (SEC-SPREAD (a)), bot decisions, ability procs, upgrade offers | Part 8 | r1, R1-DET-REC4 (the first four as examples), 2184–2207; STATUS SEC-SPREAD (a); UPGRADE-SCOPE (offers) | research verified; settled |
| 126 | Ability procs shared only if ABL-PREDICT says yes and an own ability draws a proc | Part 8 | STATUS ABL-PREDICT; GL, the prediction scope, 501–524 | settled (the condition); proposed (the rule of row 123) |
| 127 | Offers authority-only for SEC-SPREAD's reason: a held seed lets a cheat compute the stream before the server shows it | Part 8 | GL, shared stream and authority-only stream, 525–545 (finding H2's reasoning); r2, R2-SYNC-REC3, 2068–2105 | research verified (the reasoning for spread); proposed (its use for offers) |
| 128 | An offer is never predicted; it arrives as an owner-only message (ADR 0001) | Part 8 | decision map J5 (0001: tracks and offers to their owner) (notes); R2-SYNC-REC3's scope | settled (the division); research verified (the scope) |
| 129 | Presentation randomness on the page's own generator, no sim stream; tracers from a cosmetic spread the hit corrects | Part 8 | r1, R1-DET-REC4, 2184–2207; STATUS SEC-SPREAD (a) | research verified; settled |
| 130 | Animation never moves a launch point or a hitbox | Part 8 | r1, R1-DET-REC4, 2184–2207 | research verified |
| 131 | In local play the Worker is the authority and holds the authority-only streams | Part 8 | GL, the authority, 485–500; GL stream entry, 525–545 | research verified (GL from R1-BND-REC6, R1-DET-REC4) |
| 132 | A Pcg32 fallback needs a source on PCG stream independence first | Part 8 | r1, R1-DET-OQ11, 2713–2717 | research, source to check |
| 133 | Why: `StdRng` non-portable; ChaCha8 portable, reference-tested, streams and a settable position; shared `Math.random` made same-seed matches differ | Part 8 | r1, R1-DET-REC4, 2184–2207 | research verified |
| 134 | Need for part 8: §3, §6.3, §6.5, §6.6; UPGRADE-SCOPE | Part 8 | r1, R1-DET-REC4 "Serves", 2184–2207; UPGRADE-SCOPE | research verified; settled |

