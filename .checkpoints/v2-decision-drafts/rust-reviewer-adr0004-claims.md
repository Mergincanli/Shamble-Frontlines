# Claims ledger: ADR-0004 (the simulation core)

Author: rust-reviewer, 2026-10-31. A notes file beside the draft (`rust-reviewer-adr0004-draft.md`), not part of it.
One row per claim or number in the draft. Lines are at commit 9e7db28 (the research files do not change in this PR;
checked with `sed -n`). Tiers: `research verified` (an R1 or R2 recommendation claim, or a finding its topic's verifier
table lists as checked: `inputs/findings-checked.tsv`) · `research, source to check` (any other research text: R3–R6
recommendations and findings outside their checked set, R1 and R2 open questions and RR risks, which their verifiers
did not check) · `arithmetic` (shown) · `settled` (a STATUS code or GLOSSARY §8) · `design doc` (VISION_V2, LOOP_V2,
EDITOR_V2, LOOK_V2) · `proposed mark` (with its reason). Where one claim rests on several tiers, the weakest is given
and the others named. `settled` also marks the drafts' division of scope (plan revision 3 and the decision map,
approved by the advisor), cited with "(notes)". Row 360 points back to the rows of the part each disposition names. "r1" = `docs/research/r1-simulation-core.md`, "r2" = `r2-netcode.md`, "r4" =
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

## Decision, parts 9 to 21

| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 135 | State in dense arrays or slot maps keyed by stable ids, iterated in id order; phases in a fixed order | Part 9 | r1, R1-DET-REC5, 2208–2221 | research verified |
| 136 | The hash-map ban: no std `HashMap`/`HashSet`; `BTreeMap` or index-ordered `Vec`; one escape, a reviewed `#[allow(clippy::disallowed_types)]` with a reason for a lookup-only map outside the serialized state; the lint is ADR 0002's (R5-RUSTLINT-REC2) | Part 9 | GL §7, 729–742; r1, R1-DET-REC5, 2208–2221; r5, R5-RUSTLINT-REC2, 1321–1339 | research verified (GL from R1-ECS-REC2, R1-DET-REC5) |
| 137 | Canonical serialization each tick: fixed field order, floats by `to_bits`, streams' state, tick counter; a fixed, unseeded hash | Part 9 | r1, R1-DET-REC6, 2222–2241 | research verified |
| 138 | Field by field, never raw struct bytes: padding bytes are uninitialised | Part 9 | r1, R1-RR-OQ6, 2951–2954 | research, source to check |
| 139 | The hash function is picked in the sim core spike; the research names none | Part 9 | r1, R1-DET-REC6 ("a fixed, unseeded hash function"), 2222–2241; R1-DET-OQ9 (production comparison is ADR 0001's), 2706–2709 | proposed mark (no named function in the research) |
| 140 | The suite: recorded match = seed, map and mode data, content hash, per-tick inputs; native twice in separate processes, Wasm in Node.js, Wasm in Chrome and Edge, Firefox, Safari; first mismatch with field-level diff; `libm` and stream unit vectors; injected divergences caught at the first tick | Part 9 | r1, R1-DET-REC6, 2222–2241 | research verified |
| 141 | Tools and CI jobs are ADR 0002's (R5-TEST-REC2) | Part 9 | r5, R5-TEST-REC2, 1412–1430; id table (R5-TEST-REC2 owner 0002) | research, source to check |
| 142 | Recorded matches keep §6.8's `[LATER]` kill-cams, replays and highlights possible | Part 9 | VISION_V2 §6.8; r1, R1-TICK-REC1 "Serves" (§6.8, replays not designed out), 2009–2027 | design doc; research verified |
| 143 | Why: no source gives a verification method; every `HashMap` instance its own seed; engines differ | Part 9 | r1, R1-DET-REC6 reason, 2222–2241 | research verified |
| 144 | R1-XU-OQ5 settled per instance; R1-XU-OQ6 settled tier "other" | Part 9 | r1, R1-XU-OQ5, 2866–2871; R1-XU-OQ6, 2872–2879 | research verified (settled by the R1 verifier at the fix step) |
| 145 | Need for part 9: §6.6, §6.8; §3 (the scale test at 48) | Part 9 | r1, R1-DET-REC6 "Serves", 2222–2241 | research verified |
| 146 | No physics engine: parry3d pinned, `enhanced-determinism` on; our own kinematic slide-and-step move (option C) | Part 10 | r4, R4-PHYS-REC1, 672–719 | research, source to check (F1–F4, F19, F21–F24 checked) |
| 147 | Rapier's controller (option B) measured once as the fallback; picked only if it wins on parity and cost | Part 10 | r4, R4-PHYS-REC1 mark 5, 708–711 ("The ADR picks C unless B wins on marks 1 and 2") | research, source to check |
| 148 | One static world from the map document, immutable, ids; BVH built once in id order; destructible state in the sim's tables filters queries; movers' index rebuilt each tick; one set of query functions for shots, sight, blasts, melee, aim, arc preview | Part 10 | r4, R4-PHYS-REC2, 721–745 | research, source to check (F12, F21 checked) |
| 149 | Projectiles per-tick swept queries, no physics bodies, no CCD; grenades reflect at the normal; knockback a velocity change | Part 10 | r4, R4-PHYS-REC3, 747–760 | research, source to check |
| 150 | Big vehicles kinematic compound bodies (hull, turret, legs, each with hit volume and alive flag), swept against statics and soldiers | Part 10 | r4, R4-PHYS-REC4, 762–784 | research, source to check |
| 151 | At 30 Hz a vehicle at 6 m/s moves 0.2 m per tick, so no substeps (R1-TICK-OQ9) | Part 10 | r4, R4-PHYS-REC4, 762–784 (`JUG.maxSpeed` 6); 6 ÷ 30 = 0.2 m | research, source to check; arithmetic |
| 152 | A turning hull: nonlinear cast or overlap test before the sweep, picked by the spike | Part 10 | r4, R4-PHYS-REC4, 762–784 | research, source to check |
| 153 | Vehicle against soldier and soldiers blocking each other are design questions | Part 10 | r4, R4-PHYS-OQ1, 1082–1084; R4-XT-OQ6, 1201–1205; id table (owner DESIGN) | research, source to check |
| 154 | Physics behind one sim module; a later need for dynamics swaps one module; none in §6 today | Part 10 | r4, R4-PHYS-OQ5, 1093–1095; Options needs list, 589–604 | research, source to check |
| 155 | The R1 items that assumed Rapier and what each becomes (DET-REC3 flag to parry3d; ECS-REC4, ECS-OQ5 only under B; ECS-REC1 reasons and mark 5 to R4-PHYS-REC1 mark 4; TICK-REC2's Rapier reason gone; RR-OQ1's hazard tested on parry3d's BVH; DET-OQ1, OQ2 asked of parry3d) | Part 10 | r1, R1-XT-OQ2, 3022–3029; r4, wave-1 items, 1216–1237; R4-PHYS-OQ3, 1087–1089 | research, source to check (the list is this record's answer to critic rank 3) |
| 156 | Why: no §6 item needs a solver; Quake III's shared movement is kinematic trace code; owning the move keeps reach the sim's integrator | Part 10 | r4, R4-PHYS-REC1 reason, 672–719 (F23, F24 checked: confirmed) | research verified (F23, F24) |
| 157 | Need for part 10: §6.2, §6.3, §6.4 | Part 10 | r4, R4-PHYS-REC1–REC3 "Serves" | research, source to check |
| 158 | Sim-owned navigation runtime; data baked by a tool, stored in the map document, read-only in a match: convex nodes with heights, portals, typed links, area ids and flags, integer costs; queries: nearest node, A*, string-pulled path, link traversal | Part 11 | r4, R4-NAV-REC1, 802–843 | research, source to check |
| 159 | The bake runs in Test, on save and in CI, from the sim's collision world, agent numbers from the movement data, rounding reported | Part 11 | r4, R4-NAV-REC1, 802–843 | research, source to check |
| 160 | Where the bake code lives is ADR 0002's (R4-XT-OQ5) | Part 11 | r4, R4-XT-OQ5, 1195–1200 | research, source to check |
| 161 | The spike compares rerecast's mesh and a layered grid from the sim's own collision | Part 11 | r4, R4-NAV-REC1, 802–843; R4-NAV-OQ1, line 1099 | research, source to check |
| 162 | Path queue with a fixed per-tick budget in node expansions, rotating fair order, hysteresis; per-goal distance tables for shared goals | Part 11 | r4, R4-NAV-REC2, 845–865 | research, source to check (F4, F19 checked) |
| 163 | Avoidance a step in movement, neighbours on the same floor layer in id order, result through a player's movement; ORCA via dodgy_2d after an audit; separation fallback | Part 11 | r4, R4-NAV-REC3, 867–886 | research, source to check (R4-OBST-F3 checked) |
| 164 | Navigation runs where bots run: server, and the browser's local server, a slice 2 need under SLICE-ORDER; never in prediction; a bot-free Worker build skips the data; new bake = new match | Part 11 | r4, R4-NAV-REC4, 888–899; STATUS SLICE-ORDER | research, source to check; settled |
| 165 | polyanya not used at run time, so no audit unless a later change brings it in | Part 11 | r4, R4-NAV-OQ4, 1106–1107; option C, 619–631 | research, source to check (this record's answer) |
| 166 | Why: Detour cannot link into the browser build; no nav crate promises determinism; baking once shares generation's floats; Quake III's precomputed area graph | Part 11 | r4, R4-NAV-REC1 reason, 802–843; R4-NAV-F8 (checked), F16 (not checked), F18, F19 (checked) | research, source to check (F16) |
| 167 | Need for part 11: §6.2, §3; LOOP_V2 §9, §10; EDITOR_V2 §3 | Part 11 | r4, R4-NAV-REC1 "Serves" and "Rests on", 802–843 | research, source to check; design doc |
| 168 | Typed links (walk, jump-up, one-way drop-down, gap jump) kept only if the sim's movement completes them at the shipped tick; editor overrides; type list is data; each link's stored fields; a bot issues a player's inputs | Part 12 | r4, R4-FLOORS-REC1, 903–923 | research, source to check (R4-FLOORS-F2–F4, F6, F7 checked; F5's count corrected) |
| 169 | Planks and ladders as later link types need no format change | Part 12 | r4, R4-FLOORS-REC1 ("open to later ones (ladder, jump pad) without a format change"), 903–923; LOOK_V2 §4 (planks) | research, source to check (planks are this record's application) |
| 170 | Floors separate layers with ceilings from day one; walkable only if the soldier fits; node layer and enclosure area id shared with R2-INT-REC2 | Part 12 | r4, R4-FLOORS-REC2, 925–939; r2, R2-INT-REC2, 2445–2465 | research, source to check; research verified (R2-INT-REC2) |
| 171 | The bake reads movement data from the sim's tuning table; the map records a hash; a mismatch marks navigation stale | Part 12 | r4, R4-FLOORS-REC3, 941–954 | research, source to check |
| 172 | Why: hand links can promise impossible jumps (GW14); 2D grid cannot stack floors (GN10, GN11); humans take over bot slots | Part 12 | r4, R4-FLOORS-REC1–REC3 reasons, 903–954 | research, source to check |
| 173 | One height-aware sight query for bots, blasts, melee, shield check, visibility pass: eye point from stance data to sample points (three per soldier, per part for a vehicle), any-hit, statics and flagged movers; per-collider blocking flags | Part 13 | r4, R4-LOS-REC1, 958–988 | research, source to check (R4-LOS-F1–F4 checked) |
| 174 | Who blocks sight and shots (R4-LOS-OQ1) and partial blockers (R4-LOS-OQ4) are design questions answered as data | Part 13 | r4, R4-LOS-OQ1, 1124–1126; R4-LOS-OQ4, 1131–1132 | research, source to check |
| 175 | Cull order (distance and team, connectivity, field of view, segments); rays per tick with carried remainders and a rotating start | Part 13 | r4, R4-LOS-REC2, 990–1006 | research, source to check |
| 176 | Cursor ray and server shot use the same static world and functions; the ray query is this record's, the command field ADR 0001's, the crossing ADR 0002's (critic rank 2) | Part 13 | r4, R4-LOS-REC3, 1008–1018; R4-XT-OQ1, 1168–1174; decision map I-19 (notes) | research, source to check; settled (the division) |
| 177 | Why: three legacy worlds (CS9, GN7, GW15); a plain segment test is height-aware with 3D points, slopes and separate floors, as Quake III's bots show | Part 13 | r4, R4-LOS-REC1 reason, 958–988 | research, source to check (F1 checked) |
| 178 | Moving vehicles never carved: avoidance obstacles by hull footprint; slow or stopped vehicles mark nodes with a temporary cost refreshed past a threshold; only crossing routes replan | Part 14 | r4, R4-OBST-REC1, 1022–1041 | research, source to check (R4-OBST-F1–F3 checked) |
| 179 | A changing structure baked in each state; a change flips node, link and portal flags in its tick, with colliders' state and ADR 0001's re-flood; no navigation generated in a match | Part 14 | r4, R4-OBST-REC2, 1043–1061 | research, source to check (R4-OBST-F4 checked) |
| 180 | LOOK_V2 §4's damage stages: intact, damaged, ruin with interior exposed; debris and planks become routes | Part 14 | LOOK_V2 §4 | design doc |
| 181 | One baked state per stage the map declares (three for LOOK_V2 §4), each with surfaces, links, sight flags, ceilings and portals; each structure baked over its own nodes, never in combinations | Part 14 | r4, R4-OBST-REC2 (two states and a ruin's shape), 1043–1061 | proposed mark (extends REC2 to LOOK_V2 §4's three stages; reason: LOOK_V2 §4 `[DEFAULT]`; the research tested two) |
| 182 | The research bakes two states and a ruin's shape | Part 14 | r4, R4-OBST-REC2, 1043–1061 | research, source to check |
| 183 | Stage triggers are a rule for Mustafa; stages and assets are content (ADR 0005); drawing is ADR 0003's | Part 14 | plan rule 6; decision map e2 (notes) | settled (the drafts' rules and division) |
| 184 | Roofs that fade are ADR 0003's; the sim gives the local player's floor layer and area id in the block | Part 14 | LOOK_V2 §4; r4, R4-FLOORS-REC2 (area ids), 925–939 | design doc; research, source to check |
| 185 | What a player may see inside is R2-INT-OQ1, a design question; the cutaway rule is ADR 0003's (critic rank 39) | Part 14 | r2, R2-INT-OQ1, 3005–3009; id table (DESIGN; rank 39 owner 0003) | research, source to check |
| 186 | Tiles only if a full bake is too slow for Test; no re-bake in a match; player building not designed (R4-OBST-OQ3) | Part 14 | r4, R4-OBST-REC3, 1063–1076; R4-OBST-OQ3, 1142–1144 | research, source to check |
| 187 | Why: carving a mover means rebuilding tiles every tick on both targets on the same tick; flag flips are identical and avoid the replan storm | Part 14 | r4, R4-OBST-REC1 reason, 1022–1041; Options Q8 table, 651–662 | research, source to check |
| 188 | Need for part 14: §6.2, §6.4, §6.5; damage stages LOOK_V2 §4 only | Part 14 | r4, R4-OBST-REC1, REC2 "Serves"; LOOK_V2 §4 | research, source to check; design doc |
| 189 | Bots run where the authority runs, fill slots and send the same commands as humans | Part 15 | r2, R2-ROOM-REC6, 2842–2856; GL the authority, 485–500 | research verified |
| 190 | Bot behaviour is a sim phase at its cadences reading the sight pass, navigation and state, writing commands | Part 15 | r1, R1-TICK-REC3, 2066–2076; r2, R2-INT-REC3 (bots read the pass), 2467–2483 | research verified |
| 191 | The behaviour set is LOOP_V2 §10's | Part 15 | LOOP_V2 §10 | design doc |
| 192 | No research covers a bot AI architecture | Part 15 | decision map h, G10 (0 hits; notes); r4 and r1 have no such finding | research, source to check (an absence, the map's grep) |
| 193 | Humans see about 100 ms of the past; bots the present (R2-SYNC-OQ13) | Part 15 | r2, R2-SYNC-OQ13, 2952–2954 | research, source to check |
| 194 | Legacy scans every 0.25 s (GN4) | Part 15 | r2, R2-SYNC-OQ13, 2952–2954 ("GN4 scans every 0.25 s") | research, source to check |
| 195 | Bot reaction is tuning data the game-designer sets; the spike reports reaction times at N = 48 | Part 15 | r2, R2-SYNC-OQ13 ("What decides it: the game-designer's bot reaction tuning … checked in the spike"), 2952–2954 | research, source to check (this record's answer) |
| 196 | A bot field of view is a design question (R4-LOS-OQ3) | Part 15 | r4, R4-LOS-OQ3, 1129–1130; id table (DESIGN) | research, source to check |
| 197 | ADR 0005 defines the data model: a reduced GAS subset over a closed set of operations; weapons beside it sharing one damage pipeline | Part 16 | r1, R1-ABL-REC1, 2342–2378; R1-ABL-REC2, 2380–2400 | research verified |
| 198 | The weapon state machine (timer, ammo, burst count, charge, reload, active-reload window) is per-soldier sim state | Part 16 | r1, R1-ABL-REC2, 2380–2400; LOOP_V2 §7 | research verified; design doc |
| 199 | The ability, effect and weapon step ≤ 1 ms p99 in the stress scene with up to 8 active effects per soldier, 384 at N = 48, heap-free | Part 16 | r1, R1-ABL-REC6, 2465–2480; GL stress scene, 397–437 (N × 8) | research verified; arithmetic (48 × 8 = 384) |
| 200 | Own-ability prediction waits for ABL-PREDICT; sim code the same either way | Part 16 | STATUS ABL-PREDICT; r1, R1-ABL-REC4, 2421–2446 | settled; research verified |
| 201 | Definitions read by id; a development reload swaps them between ticks (R1-DATA-REC5, ADR 0005's) | Part 16 | r1, R1-DATA-REC5, 2542–2563; id table note ("swap between ticks (0004)") | research verified |
| 202 | Why: a closed set of operations keeps combos in data while the sim stays reviewable and deterministic | Part 16 | r1, R1-ABL-REC1 reason, 2342–2378 | research verified |
| 203 | Economy and objectives (money, deliveries, rebuilds, uplinks, tiers, power-plant pricing, points race) as sim phases with numbers in ADR 0005's tables and rules as LOOP_V2 §3, §4, §8 state | Part 17 | LOOP_V2 §3, §4, §8; VISION_V2 §6.5 | design doc |
| 204 | Team filtering (GC1) and presence counting (GE8) are sim code | Part 17 | LEGACY_INVENTORY GC1 (line 240), GE8 (line 294), both "rewrite in Rust" | research verified (R7 rows) |
| 205 | Visibility classes are ADR 0001's | Part 17 | r2, R2-INT-REC5 (by id; owner 0001 in the id table) | settled (the division) |
| 206 | A tier or payment is an event, never a HUD write (legacy `addTech`) | Part 17 | STATUS ADR-001 entry (line 199: "should become an event") | settled (a STATUS record) |
| 207 | Need for part 17: §6.5 | Part 17 | VISION_V2 §6.5 | design doc |
| 208 | A player record per slot (PlayerState), beside the soldier tables, apart from the soldier (Pawn), holding one track per unit type and per weapon type (level, picks, pending offer) keyed by ADR 0005's type ids | Part 18 | UPGRADE-SCOPE; decision map J5 (notes) | settled |
| 209 | The record survives death and unit switches; a weapon type's track waits while no gun of that type is held | Part 18 | UPGRADE-SCOPE | settled |
| 210 | The record's size is bounded by content type counts, allocated at match start; nothing a client sends sizes it | Part 18 | r1, R1-RR-OQ2 (bounded allocation as a risk), 2933–2935 | proposed mark (this record's design; reason: client-sized allocation is a denial-of-service risk) |
| 211 | At match start every record is built empty; nothing carries over; ADR 0001's room lifecycle calls the phase | Part 18 | UPGRADE-SCOPE; LOOP_V2 §8 ("Nothing carries into the next match"); decision map J5 (notes) | settled; design doc |
| 212 | A class's own weapon comes from the unit definition (ADR 0005) | Part 18 | UPGRADE-SCOPE; decision map J5 row 1 (notes) | settled |
| 213 | A dead soldier's weapon becomes a pickup entity: id, weapon type id, position settled with the movement query, lifetime in ticks from ADR 0005's data | Part 18 | UPGRADE-SCOPE ("dropped weapons are world pickups"); decision map J5 (notes) | settled (the need); proposed mark (the entity's fields) |
| 214 | A pickup claim is an input command handled in fixed order, so ties resolve the same everywhere | Part 18 | r1, R1-DET-REC5 (fixed order), 2208–2221 | research verified (the ordering rule); the claim-as-command is this record's |
| 215 | Offers drawn on the authority from the authority-only offers stream when a track levels; the server refuses unoffered picks and unplaced pickups (ADR 0001) | Part 18 | part 8 (rows 123, 127); decision map J5 row 4 (notes) | proposed mark; settled (the division) |
| 216 | Picks, offers shown, weapons looted, time per weapon type are telemetry events (ADR 0006's fields) | Part 18 | decision map J5 row 5 (notes) | settled (the division) |
| 217 | Carry slots, looted ammo and levelling speed are ADR 0005's open questions | Part 18 | UPGRADE-SCOPE; plan rev 3, ADR 0005 paragraph | settled |
| 218 | Why: a record apart from the soldier is the only place that outlives death and switch; pickups as entities get ids, snapshots and hashes | Part 18 | UPGRADE-SCOPE; parts 1–3 | settled; proposed (the reasoning) |
| 219 | Need for part 18: UPGRADE-SCOPE | Part 18 | UPGRADE-SCOPE; decision map J3 (notes) | settled |
| 220 | Tick-stamped events: hits, deaths and kills with killer and victim slot and unit id, source weapon or ability id, assisting slots; purchases; uplink events; harvester deliveries and kills; structure kills; tech tiers | Part 19 | r6, R6-TEL-REC1, 784–811 | research, source to check |
| 221 | Blasts with position and strength, for the concussion shake and the fragments; cosmetic shot events | Part 19 | LOOK_V2 §2, §3, §7; decision map I-20 (notes) | design doc; settled (the division) |
| 222 | The event list is part of the state and its hash; ADR 0002 places it in the block; ADR 0001 delivers it; the room writes the record at Ended; ADR 0006 owns the shape | Part 19 | r1, R1-ECS-REC1 / options C (the tick's event list in the state), 1630–1644; decision map S6, I-10 (notes) | research verified; settled |
| 223 | A moment phase after the outcome phases reads events and state and emits moments by rules held as data; thresholds a design question (R6-TEL-OQ3); moments and telemetry read one stream | Part 19 | r6, R6-TEL-REC1 ("so the moment rules and the telemetry read the same stream"), 784–811; R6-TEL-OQ3, 915–916 | research, source to check; the phase placement is proposed (no research on the mechanism) |
| 224 | Timeline sampled every 10 s of game time | Part 19 | r6, R6-TEL-REC1, 784–811 | research, source to check |
| 225 | No research covers a moment mechanism | Part 19 | decision map h, G11 (notes) | research, source to check (an absence) |
| 226 | Need for part 19: §6.6, §6.8 | Part 19 | r6, R6-TEL-REC1 "Serves" | research, source to check |
| 227 | The sim takes only validated commands and content; the loader returns errors and is fuzzed with zero panics | Part 20 | r1, R1-RR-OQ3 (the reviewer's proposal), 2936–2940; r5, R5-LAYOUT-REC2 (decoders that never panic), 1170–1193 | research, source to check |
| 228 | No `unwrap`, unchecked index or unchecked `clamp` on client- or content-set values; `f32::clamp` panics on bad bounds | Part 20 | r1, R1-DET-F14, 679–683 (checked: confirmed) | research verified (F14); proposed mark (the rule; reason: a panic is a denial of service) |
| 229 | On the server a panic stops that room's match (each room its own process) | Part 20 | r2, R2-ROOM-REC2, line 2762; R2-ROOM-F8 (line 1459, checked: an uncaught panic ends only its own thread) | research verified |
| 230 | In the browser `-Cpanic=abort` ends the sim's instance | Part 20 | r1, R1-BND-F19, 1019–1028 (checked) | research verified |
| 231 | Online, after a trap, a fresh instance rejoins through ADR 0001's resync; the trap is reported (R6-OBS-REC3) | Part 20 | r1, R1-RR-OQ3, 2936–2940; R1-XT-OQ12, 3084–3087; r6, R6-OBS-REC3, 754–768 | research, source to check |
| 232 | In local play the trapped instance was the authority; recovery not researched | Part 20 | GL the authority, 485–500; no finding | research verified (GL); the gap is an absence |
| 233 | Need for part 20: VISION_V2 §4 only | Part 20 | VISION_V2 §4 | design doc |
| 234 | Editor reach = the sim's movement at the shipped `dt`; reachability and the sub-6 m gap report run in the sim crate, from Node and CI, identical on native and Wasm | Part 21 | r1, R1-TICK-REC6, 2118–2125; r4, R4-NAV-REC1 marks 3–4, 835–837 | research verified (REC6); research, source to check (R4) |
| 235 | Why: integration error depends on the step; legacy's formula overstates reach (GW14) | Part 21 | r1, R1-TICK-REC6, 2118–2125 | research verified |
| 236 | Need for part 21: §6.2; EDITOR_V2 §3; LOOP_V2 §9 | Part 21 | r1, R1-TICK-REC6 "Serves"; EDITOR_V2 §3; LOOP_V2 §9 | research verified; design doc |

## Options considered

Cells copied or condensed from the cited research table are `research, source to check` (R4 tables; R1 options cells
are unit text outside the recommendations). Cells the research table has no row for (template 0000's "Cost",
"Security and anti-cheat", "Effect on the look and feel" where the table is silent) are the author's assessment from the
same table's cells, tier `proposed mark` with that reason, so the verifier can weigh them.

| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 237 | Q1 table: shape, order, snapshot, ids, size, upkeep and fit of bevy_ecs 0.20, hecs 0.11.2 and plain data | Options Q1 | r1, Options ECS table, 1630–1644 | research, source to check |
| 238 | Q1: a migration roughly each 0.x release (five releases in 18 months, inferred) | Options Q1 | r1, Options ECS table, "Upkeep and risk" row (F1), 1630–1644 | research, source to check (inferred there) |
| 239 | Q1: bevy's multi-threaded executor is the one thing C lacks (R1-ECS-OQ4); sizes unmeasured (R1-ECS-OQ1) | Options Q1 | r1, R1-ECS-OQ4, 2622–2626; R1-ECS-OQ1, 2612–2613 | research, source to check |
| 240 | Q1: legacy's flat pooled SIM records (CS8) are C's starting layout | Options Q1 | LEGACY_INVENTORY CS8 v2 note (line 176) | research verified (R7 row) |
| 241 | Q1 cells "Cost: no money", "Security: neutral", "Effect on look and feel: none directly" | Options Q1 | none in the table | proposed mark (template dimensions; the author's assessment) |
| 242 | Q2: the tick rate is settled by TICK-30; the 20/60/64/66 Hz rows and variable commands not weighed | Options Q2 | STATUS TICK-30; r1, Options 4a, 1648–1659 | settled |
| 243 | Q2 slow systems: everything every tick scales AI, sight and route cost with the rate; cadences keep it to what must be fine-grained; legacy does it in seconds (GN4) | Options Q2 | r1, Options 4b, 1660–1665 | research, source to check |
| 244 | Q2 server loop: millisecond timers miss the period; `Burst` catches up unbounded; the accumulator gives an exact average rate; integer-ms period: 62.5 Hz at 60 Hz | Options Q2 | r1, Options 4c, 1666–1672 | research, source to check |
| 245 | Integer-ms period at 30 Hz gives 1000/33 ≈ 30.3 Hz | Options Q2 | ⌊1000/30⌋ = 33 ms; 1000/33 = 30.3 Hz | arithmetic |
| 246 | Q2 client loop W1–W4 cells (rAF rate 60–144 Hz and stops in background tabs; W2 independent, late timers, hidden-tab unknown; W3 precise, needs shared memory and isolation, inputs through shared memory; W4 as W1) | Options Q2 | r1, Options 4d, 1673–1682 | research, source to check |
| 247 | W3's isolation narrows embeds (R1-BND-OQ5, if adopted) | Options Q2 | r1, R1-XT-OQ17 (R1-BND-OQ5's embeds under isolation), 3105–3108 | research, source to check |
| 248 | Q2 cells "Cost: none", "Effect on look and feel" judgements, "Effort: small" | Options Q2 | none in the tables | proposed mark (template dimensions; the author's assessment) |
| 249 | Q3: option E (lockstep-grade) settled by the stack's server authority | Options Q3 | r1, Options Q1 row E ("not the brief's model (server authority)"), 1685–1694; STATUS (the stack) | settled |
| 250 | Q3 table A–D cells (complexity, cost unknown for `libm` and the flag, risk, fit, effort; D excludes the collision library and brings no trig) | Options Q3 | r1, Options Q1 table, 1685–1694; R1-DET-OQ1, 2680–2681 | research, source to check |
| 251 | Q3 transcendental sub-choices (std varies by platform and Rust version, different code on Wasm and Linux; `libm` same code, no accuracy bound; nalgebra ComplexField with `libm_force`; own approximations) | Options Q3 | r1, Options Q2 table, 1695–1703 | research, source to check |
| 252 | parry3d now uses glam (R4-PHYS-F22) | Options Q3 | r4, R4-PHYS-F22, 196–202 (checked: confirmed) | research verified |
| 253 | RNG sub-choices: StdRng non-portable; ChaCha8 portable, reference-tested, 2^64 streams per seed, settable position; Pcg32 portable, stream parameters "may be closely correlated"; `_mulberry32` quality not researched | Options Q3 | r1, Options Q4 table, 1713–1721 | research, source to check |
| 254 | Any generator whose seed a client holds lets it predict draws | Options Q3 | GL, shared stream and authority-only stream, 525–545 (H2's reasoning) | research verified (GL definition from R1-DET-REC4, R2-SYNC-REC3) |
| 255 | Iteration sub-choices: std `HashMap` arbitrary and seeded per instance; fixed hasher repeatable for one build, not shown identical on wasm32 and 64-bit, loses HashDoS resistance; `BTreeMap` sorted, slower lookups unmeasured; dense arrays by id | Options Q3 | r1, Options Q5 table, 1722–1730 | research, source to check |
| 256 | Q4 table A–D (complexity, snapshot, determinism, size and CPU, churn, effort, fit) | Options Q4 | r4, Options PHYS table, 606–617 | research, source to check |
| 257 | 12 Rapier releases in about a year with format breaks | Options Q4 | r4, R4-PHYS-F15, 146–151 (checked: confirmed) | research verified |
| 258 | Autostep "very computationally expensive" | Options Q4 | r4, R4-PHYS-F2, 61–71 (checked: confirmed) | research verified |
| 259 | Legacy GW13 and GW14 are C's reference; D ports CS4, CS5 | Options Q4 | r4, Options PHYS table "What it is", 606–617; R4-PHYS-REC1 "Rests on", 672–719 | research, source to check |
| 260 | Q4 "Effect on look and feel" for C: movement feel ours to tune, as Quake III's is | Options Q4 | r4, R4-PHYS-F23, F24 (checked) | proposed mark (the author's assessment from F23, F24) |
| 261 | Q5 table A–D (Detour blocked for the browser build; pure-Rust crates young, no determinism promise; C ours; D many nodes, 8,000-node cap fails, 2 m cells close gaps) | Options Q5 | r4, Options NAV table, 619–631 | research, source to check |
| 262 | Q6 table A–C (hand links can promise impossible jumps; generated links measured; no links loses LOOP_V2 §9's drop-downs) | Options Q6 | r4, Options FLOORS table, 632–641 | research, source to check |
| 263 | Q6: the reachability check (TL14) moves into the sim | Options Q6 | r4, legacy facts (R4-NAV), TL14, lines 401–402 | research, source to check |
| 264 | Q7 table A–C (cost bounded by pairs × points; movers' index rebuilt each tick anyway; precomputed visibility coarse, wrong for moving cover) | Options Q7 | r4, Options LOS table, 642–650 | research, source to check |
| 265 | Q7: A shares the pass interest uses (ADR 0001) | Options Q7 | r2, R2-INT-REC3, 2467–2483 | research verified |
| 266 | Q8 table A–C (carving rebuilds each tick, nondeterministic risk, replans; avoidance only leaves walls closed; flags plus avoidance identical, crossed routes only) | Options Q8 | r4, Options OBST table, 651–662 | research, source to check |
| 267 | Q9: the two stream kinds are GL §4's; SEC-SPREAD weighed them for spread; no research weighs them for offers | Options Q9 | GL, 525–545; STATUS SEC-SPREAD (a) | research verified (GL); the absence is this record's statement |
| 268 | Q9 cells: a shared stream lets a seed-holding client compute its (and others') next offers; authority-only keeps the seed and leaves the client hash; an offer arrives with the server's message | Options Q9 | GL, 525–545; GL state hash, 546–559 | proposed mark (applies GL's reasoning to offers) |
| 269 | Q10: tracks on the soldier entity settled out by UPGRADE-SCOPE | Options Q10 | UPGRADE-SCOPE | settled |
| 270 | Q11: R1 plans no recovery; the reviewer proposes a Worker restart from the server's snapshot | Options Q11 | r1, R1-RR-OQ3, 2936–2940 | research, source to check |
| 271 | Q11 cells (a short freeze then back in step; the server stays the authority; effort in the Worker host) | Options Q11 | none in the research | proposed mark (template dimensions; the author's assessment) |

## Trade-off analysis and Consequences

| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 272 | Plain data wins on order, one-call snapshot and sim-owned ids; against: we write pools, ids, change detection; kinds may multiply (R1-ECS-OQ7) | Trade-off | r1, Options ECS table, 1630–1644; R1-ECS-OQ7, 2633–2636 | research, source to check |
| 273 | `libm` and parry3d flag costs unmeasured; a dependency's SIMD path can differ (R1-RR-OQ1, R4-PHYS-OQ3); worst case more corrections | Trade-off | r1, R1-DET-OQ1, 2680–2681; R1-RR-OQ1, 2923–2932; r4, R4-PHYS-OQ3, 1087–1089; r1, R1-DET-REC1, 2129–2145 | research, source to check |
| 274 | parry3d returns an error for an unsupported pair (R4-PHYS-F20) | Trade-off | r4, R4-PHYS-F20, 177–187 (verifier: wrong as first written, corrected to this text at R4's fix pass) | research, source to check |
| 275 | Neither navigation generator measured on Octopon-3's blocks and plateaus; every damage stage adds to the bake | Trade-off | r4, R4-NAV-REC1 ("not settled by any source (F12, F16), so it is measured"), 802–843 | research, source to check |
| 276 | A later wish to show offers before confirmation would be a prediction-scope change | Trade-off | GL the prediction scope, 501–524 | proposed mark (the author's reasoning) |
| 277 | Unlocking 60 Hz is a rebuild plus a re-bake (links measured at the shipped tick) | Trade-off; Consequences | r4, R4-FLOORS-REC1 (links checked "at the shipped tick"), 903–923; r1, R1-TICK-REC1, 2009–2027 | research, source to check |
| 278 | Easier / harder lists (replayable matches, prediction with the server's code, one hit world, editor checks share sim code, flips in one tick, tracks by construction; we own pools, movement, runtime, bake; review load; dependency audits; no research on bot architecture or moments) | Consequences | parts 1–21 and their citations; decision map h (G10, G11) (notes) | proposed mark (consequences drawn from the decision) |
| 279 | Revisit triggers (R1-ECS-OQ7, R1-ECS-OQ4, 2 km map, dynamics, R1-DET-REC7's rule, R1-DET-OQ11, R4-OBST-REC3, R4-OBST-OQ3, ABL-PREDICT yes, 60 Hz unlocked) | Consequences | r1, 2633–2636, 2622–2626, 2242–2248, 2713–2717; r4, 786–798, 1093–1095, 1063–1076, 1142–1144; STATUS ABL-PREDICT, TICK-30 | research, source to check; settled |
| 280 | Hard constraints: the classic-scripts and double-click constraint does not apply to the sim (a Rust crate), under SPIKE-SCOPE (a), after acceptance | Consequences | CLAUDE.md "Hard constraints"; STATUS SPIKE-SCOPE (a); plan rule 11 | settled |
| 281 | "Animation is cosmetic … Nothing animated decides a hit" carries into v2 unchanged (part 8) | Consequences | CLAUDE.md "Hard constraints"; r1, R1-DET-REC4, 2184–2207 | settled; research verified |
| 282 | "No allocation per frame in hot paths" becomes part 6's heap-free step; other hard constraints are not the sim's; "no external assets" ends with LOOK_V2 §8 | Consequences | CLAUDE.md "Code rules", "Hard constraints"; LOOK_V2 §8 | settled; design doc |
| 283 | Nothing in CLAUDE.md or `.claude/` changes in this PR; the inventory file is not edited | Consequences | plan rev 3, "Not touched"; rule 11 | settled |
| 284 | Confirmed "rewrite in Rust" rows: CS2, CS4–CS9, GW9, GW13–GW15, GC1, GC2, GC5, GC7, GC17, GN4–GN6, GN12, GN13, GP14, GE8, CM11, CM13, CD4, CD6, CD8, CD10–CD14 | Consequences | LEGACY_INVENTORY lines 85–95, 159, 161, 170–177, 197, 201–203, 240–246, 256, 269–271, 277–278, 294, 324 | research verified (R7 marks as read) |
| 285 | GN10: rewrite in Rust → redesign (typed links replace climb steering) | Consequences | LEGACY_INVENTORY GN10 (line 275); r4, R4-FLOORS-REC1, 903–923 | proposed mark (a mark change for Mustafa) |
| 286 | GN11: rewrite in Rust → redesign (baked navigation; the grid method a candidate generator) | Consequences | LEGACY_INVENTORY GN11 (line 276); r4, R4-NAV-REC1, 802–843 | proposed mark |
| 287 | GC4: rewrite in Rust → the state machine stays; loadout redesign (UPGRADE-SCOPE) | Consequences | LEGACY_INVENTORY GC4 (line 243); UPGRADE-SCOPE | proposed mark |
| 288 | GC14: look brief pending lifted; LOOK_V2 §3, §7 bring fragmentation back; per-fragment damage a design question | Consequences | LEGACY_INVENTORY GC14 (line 253); LOOK_V2 §3, §7 | proposed mark; design doc |
| 289 | GC11, GC12, GC13, GC15: look brief pending lifted; sim half unchanged | Consequences | LEGACY_INVENTORY lines 250–252, 254; LOOK_V2 (exists, 2026-10-31) | proposed mark |

## Open questions

| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 290 | OQ-0004-1: LOOP_V2 §10 calls the role mix "the biggest prototype risk"; no research on bot architecture; the parts it runs on | OQ-0004-1 | LOOP_V2 §10; decision map h, G10 (notes); cites R2-ROOM-REC6, R1-TICK-REC3, R4-LOS-REC2, R4-NAV-REC2 | design doc; research, source to check (the absence) |
| 291 | OQ-0004-2: about a dozen kinds; deployables, zones, mines would favour an ECS; trigger "a new kind cannot be a row" | OQ-0004-2 | r1, R1-ECS-OQ7, 2633–2636 | research, source to check |
| 292 | OQ-0004-3: boxes and wedges, a heightfield or a triangle mesh, all parry shapes; terrain sculpting `[LATER]` | OQ-0004-3 | r4, R4-PHYS-OQ4, 1090–1092; EDITOR_V2 §3 | research, source to check; design doc |
| 293 | OQ-0004-4: legacy buildings already have health (GW20); the sim carries any trigger as data | OQ-0004-4 | LEGACY_INVENTORY GW20 (line 208) | research verified (R7 row) |
| 294 | OQ-0004-5: UPGRADE-SCOPE says only "Other weapons come only from killed enemies' drops; dropped weapons are world pickups"; the listed rule options | OQ-0004-5 | UPGRADE-SCOPE (verbatim) | settled (the quote); the options are questions, not claims |
| 295 | OQ-0004-6: bots fill slots and humans take over bot slots | OQ-0004-6 | r2, R2-ROOM-REC6, 2842–2856 | research verified |
| 296 | OQ-0004-7: snapshot-outside-the-instance and input-replay options not researched; input replay possible under SLICE-ORDER condition 2 | OQ-0004-7 | r1, R1-RR-OQ3, 2936–2940; STATUS SLICE-ORDER | research, source to check; settled |
| 297 | The DESIGN-owned questions listed after OQ-0004-7 (who blocks whom; what a player may see; blockers, field of view, partial blockers; stances, vehicle bots, fall damage; harvester lanes; player building; offline play; moment thresholds; the caps) | Open questions | id table (owner DESIGN): R4-PHYS-OQ1, R4-NAV-OQ5, R4-OBST-OQ1, R2-XT-OQ13, R4-XT-OQ6, R2-INT-OQ1, R4-LOS-OQ1–OQ4, R4-FLOORS-OQ1–OQ3, R4-OBST-OQ2, OQ3, R1-TICK-OQ7, R1-BND-OQ1, R6-TEL-OQ3, R2-BUD-OQ5; R2-SYNC-OQ5 owner 0001 | settled (the id table's owners) |

## What a spike must prove (S1–S20)

The Pass marks table under it is the test-engineer's (plan step 4); its rows join this ledger then.

| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 298 | The stress scene at N = 48: 16v16v16, 15 big vehicles (4 very large units), 300 live projectiles, up to 8 active effects per soldier, one DM_Tri-sized greybox | Spike intro | GL §3, 397–437; MODES-3 (a) | research verified (GL); settled |
| 299 | ADR 0001 owns the 60 Hz margin run and how the sim's load reaches it (TICK-60-SETUP) | Spike intro | GL §8, the 60 Hz margin; decision map J6 (2) (notes) | settled |
| 300 | No claim here needs a person's judgement; "look correct" is ADR 0001's | Spike intro | GL §8, the netcode proof test; plan rule 12 | settled |
| 301 | S1: p99 ≤ 8.3 ms at 30 Hz (≤ 4.2 ms at 60 Hz); mean, p50, p95, p99, max per N; N = 12, 24, 36, 48 and the N = 64 headroom run; 3 runs; parts reported | S1 | r1, R1-TICK-REC2 P1, 2044–2053; R1-ECS-REC1 mark 1, 1923–1931; GL 53–69 | research verified |
| 302 | 36,000 ticks per run = 20 minutes at 30 Hz, 10 at 60 Hz | S1, S17 | 36,000 ÷ 30 = 1,200 s; 36,000 ÷ 60 = 600 s | arithmetic |
| 303 | S2: p99 ≤ 13.3 ms at 30 Hz in a dedicated Worker on the minimum laptop, page 30+ fps on Low; Wasm ≤ 2 × native; N from R1-TICK-OQ7 | S2 | r1, R1-TICK-REC2 P2, 2054–2057; R1-ECS-REC1 mark 1, 1923–1931; R1-TICK-OQ7, 2667–2670 | research verified |
| 304 | S3: p99 ≤ 2 × mean; sight checks per game second per bot within 1 % at 30 and 60 Hz | S3 | r1, R1-TICK-REC3, 2074–2076 | research verified |
| 305 | S4: every legacy interval within 1 shot at 30 and 60 Hz and within 1 shot of 60 / interval over 60 s | S4 | r1, R1-TICK-REC1, 2023–2024 | research verified |
| 306 | It meets R1-ABL-REC3's 1 % for the Shabzon: 1 shot in 833 is 0.12 % | S4 | 60 ÷ 0.072 = 833.3 shots; 1 ÷ 833 = 0.12 % | arithmetic |
| 307 | CD13's Warrior-kills-Conscript 0.86 s within one tick | S4 | r1, R1-ABL-REC3, 2416–2419 | research verified |
| 308 | A CI check finds no clock read in the step (no `std::time`, no host time import) | S4 | r1, R1-TICK-REC1, line 2027 | research verified |
| 309 | The units-per-tick figure chosen from the rounding report | S4 | none in the research | proposed mark (reason: part 4 needs a number the research does not give) |
| 310 | S5: 0 heap allocations per snapshot after warm-up (counting allocator); ≤ 0.2 ms native, ≤ 0.5 ms Wasm p99 | S5 | r1, R1-ECS-REC1 mark 2, 1932–1937 | research verified |
| 311 | S5: snapshot–restore–continue equals an uninterrupted 10-minute bot match by per-tick hash; serialize round trip byte-identical | S5 | r1, R1-ECS-REC1 mark 4, 1942–1945 | research verified |
| 312 | 10 minutes = 18,000 ticks at 30 Hz | S5, S7 | 600 s × 30 = 18,000 | arithmetic |
| 313 | Whole-world replay k = ⌈0.15 s × 30⌉ = 5 ticks within 8 ms p99, only if ADR 0001 keeps it | S5 | r1, R1-ECS-REC1 mark 3, 1938–1941; 0.15 × 30 = 4.5, rounded up to 5 | research verified; arithmetic |
| 314 | S6: ids survive restore; despawned id never resolves (property test, 10,000 cases); client maps ids with no extra table | S6 | r1, R1-ECS-REC3, 1985–1989 | research verified |
| 315 | S7: same seed and input log twice on one build, same hash every tick for 36,000 ticks | S7 | r1, R1-TICK-REC1, 2025–2026 | research verified |
| 316 | S7: bots-only 24v24 (48) and 12v12v12 (36), 10 minutes, 3 seeds, 0 mismatches native–native and native–Wasm; no third match under MODES-3 (a) | S7 | r1, R1-DET-REC1, 2139–2144; STATUS MODES-3 (a) | research verified; settled |
| 317 | S7: different std `HashMap` seeds in the two processes, same hashes | S7 | r1, R1-ECS-REC2 mark 1, 1966–1971 | research verified |
| 318 | S7: parry3d's flag on, stress scene, 0 mismatches over 10 minutes; flag cost on and off reported | S7 | r1, R1-DET-REC3, 2178–2182 (applied to parry3d per R4-PHYS-REC1) | research verified (as written for Rapier); the transfer to parry3d is this record's change |
| 319 | S7: ≥ 10,000 inputs per `libm` function bit-identical on x86-64, AArch64 if picked, Wasm per browser engine; a build check on std transcendentals, `algebraic_*`, `relaxed-simd`; cost reported | S7 | r1, R1-DET-REC2, 2165–2168 | research verified |
| 320 | S7: the suite with 0 mismatches, catching each injected divergence at its first tick, hashing cost reported | S7 | r1, R1-DET-REC6, 2238–2240 | research verified |
| 321 | S7: `simd128` on decides R1-DET-OQ2; `libm` accuracy against reference values (R1-DET-OQ8); Wasm `sqrt` against IEEE (R1-DET-OQ10) | S7 | r1, R1-DET-OQ2, 2682–2685; R1-DET-OQ8, 2704–2705; R1-DET-OQ10, 2710–2712 | research, source to check |
| 322 | S8: first 10,000 outputs per stream identical; saved and restored stream continues identically; muting sound, moving the camera or changing frame rate changes no hash | S8 | r1, R1-DET-REC4, 2205–2207 | research verified |
| 323 | S8: 0 bytes of authority-only stream state in the client's predicting state | S8 | GL 525–545; STATUS SEC-SPREAD (a) | proposed mark (reason: the test of part 8's rule) |
| 324 | S8: ChaCha8's cost per tick reported, deciding whether R1-DET-OQ11 returns | S8 | r1, R1-DET-OQ11, 2713–2717 | research, source to check |
| 325 | S9: R4-PHYS-REC1's six marks (0.55 m climbed, 0.56 m stopped; apex within 1 mm; 0 tunnelling through 0.2 m in 10,000 moves at about 13.3 m/s; ≤ 1 ms native mean, Wasm ≤ 2×; 0 mismatches and byte-identical BVH; sizes; fallback; 0 unsupported pairs) | S9 | r4, R4-PHYS-REC1, 692–717 | research, source to check |
| 326 | 13.3 m/s is 0.44 m per tick at 30 Hz | S9 | r4, R4-PHYS-REC1 mark 1, 693–699 ("0.44 m at 30 Hz"); 13.3 ÷ 30 = 0.44 | research, source to check; arithmetic |
| 327 | S9: R1-ECS-REC1 mark 5 (smaller than a bevy_ecs 0.20 stub; one-line incremental rebuild ≤ 10 s) | S9 | r1, R1-ECS-REC1 mark 5, 1946–1949 | research verified |
| 328 | S9: R4-PHYS-REC2 (20,000 rays; 0 of 1,200 shots over 1 cm above a ramp; byte-identical after restore; BVH build ≤ 50 ms) | S9 | r4, R4-PHYS-REC2, 739–743 | research, source to check |
| 329 | S9: R4-PHYS-REC3 (0 tunnelling at 30 and 60 Hz; 1,000 of 1,000 bounces; identical landings) | S9 | r4, R4-PHYS-REC3, 758–760 | research, source to check |
| 330 | S9: R4-PHYS-REC4 (0 ticks with penetration over 1 cm, 10 minutes, 48 soldiers) | S9 | r4, R4-PHYS-REC4, 779–784 | research, source to check |
| 331 | S9: R4-PHYS-REC5 (marks hold at the far corner; distance within 0.1 %) | S9 | r4, R4-PHYS-REC5, 796–798 | research, source to check |
| 332 | S10: R4-NAV-REC1's six marks (bake ≤ 1 s native, ≤ 3 s Worker; ≥ 99.9 % coverage, 0 unstandable; 6 m gap; reachability identical; 10,000 identical paths; comparison) | S10 | r4, R4-NAV-REC1, 830–841 | research, source to check |
| 333 | S10: R4-NAV-REC2 (≤ 5 % of the tick period, 1.67 ms at 30 Hz, at N = 48 and the N = 64 headroom run; no wait over 1 s; ≤ 2 replans per bot per second p99; stall ≤ 7.73 %) | S10 | r4, R4-NAV-REC2, 859–865; 0.05 × 33.33 = 1.67 | research, source to check; arithmetic |
| 334 | S10: R4-NAV-REC3 (48 bots, 6 m choke, 10 minutes, 0 deadlocks over 3 s; identical hashes; ≤ 0.2 ms p99; 0 cross-floor pairs) and REC4 (bot-free build skips the data; with bots meets S2) | S10 | r4, R4-NAV-REC3, 880–884; R4-NAV-REC4, 896–897 | research, source to check |
| 335 | S11: FLOORS-REC1 (0 links above reach; 1,000 of 1,000 traversals at 30 and 60 Hz; 0 % unreachable surfaces; drop-downs ≥ 95 % of 200), REC2 (separate ids; 100 of 100; roof not walkable), REC3 (stale 1 of 1) | S11 | r4, R4-FLOORS-REC1, 918–923; REC2, 936–939; REC3, 951–952 | research, source to check |
| 336 | S12: LOS-REC1 (20,000 pairs; 0 of 10,000 through floors; 0 of 10,000 ramp errors; cost within 5 %, 1.67 ms at 30 Hz; identical hashes), REC2 (≥ 4 scans per second), REC3 (within 1 cm in 1,000 of 1,000; R2-SYNC-REC8 mark 1 ADR 0001's) | S12 | r4, R4-LOS-REC1, 975–986; REC2, 1002–1006; REC3, 1015–1016 | research, source to check |
| 337 | S13: OBST-REC1 (0 stuck over 3 s; ≤ 2 replans p99; identical hashes) and REC2 (same tick, 0 ticks of disagreement over 1,000 cycles; 1,000 traversals; flip ≤ 0.1 ms p99) | S13 | r4, R4-OBST-REC1, 1037–1041; REC2, 1058–1061 | research, source to check |
| 338 | S13: the same marks with three stages, a plank link only in the ruin stage, bake still within S10's bake time | S13 | LOOK_V2 §4; r4, R4-OBST-REC2, 1043–1061 | proposed mark (reason: LOOK_V2 §4 names three stages, the research tested two) |
| 339 | S14: the spike's bot set (legacy GN combat behaviour ported plus a carried cache as a moving goal) | S14 | LEGACY_INVENTORY GN4–GN6, GN13 (lines 269–271, 278); r4, R4-NAV-REC2, 859–865 | proposed mark (reason: "bots with full AI" has no research definition) |
| 340 | S14: bot reaction times at N = 48 reported (no mark) | S14 | r2, R2-SYNC-OQ13, 2952–2954 | research, source to check |
| 341 | S15: ≤ 1 ms p99 with 384 active effects, reference server and laptop, 0 heap allocations; 120 s scenario, 3 seeds, identical logs | S15 | r1, R1-ABL-REC6, 2465–2480; R1-ABL-REC3, 2416–2419; 48 × 8 = 384 | research verified; arithmetic |
| 342 | S16: UPGRADE-SCOPE's sentences as checks (0 entries lost over deaths and switches; track resumes with the next gun; empty records at match start; pickup ids through restore; identical tie resolution) | S16 | UPGRADE-SCOPE | proposed mark (reason: no research mark; test-engineer sets the numbers) |
| 343 | S17: jitter p99 ≤ 2 ms over 36,000 ticks on the reference server (Linux); back on schedule within 1 s after a 200 ms stall; counters in the metrics | S17 | r1, R1-TICK-REC4, 2090–2092 | research verified |
| 344 | S17, S18: never more than 7 ticks in one wake or frame (the research's 15 at 60 Hz) | S17, S18 | r1, R1-TICK-REC4, 2090–2092; R1-TICK-REC5, 2109–2114; ⌊0.25 × 30⌋ = 7 | proposed mark (restatement for 30 Hz); arithmetic |
| 345 | S18: four browsers; foreground jitter p99 ≤ 4 ms over 10 minutes; displacement SD ≤ 5 % of mean on 60, 120, 144 Hz; after 60 s hidden, no tick ran locally; online back in step within 0.5 s | S18 | r1, R1-TICK-REC5, 2109–2114 | research verified |
| 346 | S18: the same after more than 5 minutes hidden | S18 | r1, R1-BND-OQ2, 2724–2727 | research, source to check |
| 347 | S19: reach within 1 mm for `T.jumpV` 11, `T.gravity` 24, `T.stepUp` 0.55 | S19 | r1, R1-TICK-REC6, 2124–2125 | research verified |
| 348 | S20: zero panics over the fuzzed content corpus (ADR 0005's mark, R1-RR-OQ3) | S20 | r1, R1-RR-OQ3, 2936–2940 | research, source to check |
| 349 | S20: zero panics over fuzzed commands fed to the sim | S20 | none in the research | proposed mark (reason: part 20's rule needs its own test) |
| 350 | S20: an injected trap ends in a fresh instance back in step via ADR 0001's resync, reported (R6-OBS-REC3) | S20 | r6, R6-OBS-REC3, 754–768; r1, R1-XT-OQ12, 3084–3087 | research, source to check |
| 351 | Each claim's spike: the sim core spike; S17 the netcode proof test; S18 the boundary spike; S15, S16 the content and ability spike | Spike table | decision map g (spike candidates and what they subsume) (notes) | settled (the drafts' working names; SPIKE_PLAN names them) |

## Migration plan, Test plan, Rollback, Appendix A

| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 352 | Phases in the slice order, the netcode proof test before the foundation, the foundation before slice 1 | Migration | STATUS SLICE-ORDER; GL §8 (the foundation) | settled |
| 353 | Phase 1: the sim core spike in parallel with the netcode proof test, then the content and ability spike before slice 1 | Migration | decision map g, "The proposed order" (notes); SPIKE_PLAN confirms | settled (proposed order, for SPIKE_PLAN and the Advisor) |
| 354 | Phase 2: the foundation's empty scene through the full stack, hashes matching | Migration | GL §8, the foundation ("an empty scene runs through the full stack") | settled; the hash milestone is proposed |
| 355 | Phases 3–8 milestones (S-marks on the real crate per slice; S1 ≤ 8.3 ms at N = 48, 3 runs) | Migration | the S rows above | proposed mark (milestones built from the spike claims) |
| 356 | Test plan items: suite on every change with ADR 0002's tools (gate set and nightly 10-minute match); unit vectors; property tests (10,000 cases); 20,000-ray and 20,000-pair parity fuzzes; fuzzing for panics; benchmarks (3 runs, method); soak 36,000 ticks; navigation checks in CI; network simulation is ADR 0001's; playtests with LOOP_V2 §11's evidence | Test plan | r5, R5-TEST-REC2, 1412–1430; r1, R1-DET-REC6, 2222–2241; R1-ECS-REC3, 1973–1991; r4, R4-PHYS-REC2, 739–743; R4-LOS-REC1, 975–986; LOOP_V2 §11 | research verified (R1); research, source to check (R4, R5); design doc |
| 357 | Rollback options: B for movement; own query for a missing pair; legacy grid generator; "back to the drafts" with R1-ECS-OQ4; level B and R1-DET-REC7's rule; two stages | Rollback | r4, Options PHYS 606–617, NAV 619–631; r1, R1-TICK-REC2 rule, 2061–2062; R1-ECS-OQ4, 2622–2626; R1-DET-REC7, 2242–2248; r4, R4-OBST-REC2, 1043–1061 | research, source to check; research verified (R1) |
| 358 | After acceptance each change is its own change-set, so a revert backs it out | Rollback | CLAUDE.md, the change-set protocol | settled |
| 359 | Appendix A: 82 ids, one row each (adopted 25, changed 10, rejected 1, answered 27, spike 17, open 2) | Appendix A | engine-architect-decision-map-ids.tsv (owner 0004) | arithmetic (counted by script) |
| 360 | Each "changed" and "answered" disposition's reason | Appendix A | as rows 46–236 for the part named in each row | as the cited row |

