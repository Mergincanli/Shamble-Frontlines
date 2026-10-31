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

