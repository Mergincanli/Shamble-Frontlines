# Claims ledger: ADR 0005, content data and asset pipeline sections (engine-architect)

Notes file beside `engine-architect-adr0005-content-section.md`, not part of the draft. One row per claim or number in
the two sections, in the order they appear. Lines are at 9e7db28 (r1 = `docs/research/r1-simulation-core.md`, r2 =
`r2-netcode.md`, r3 = `r3-rendering.md`, r4 = `r4-physics-navigation.md`, r5 = `r5-toolchain-quality.md`, r6 =
`r6-infrastructure.md`). "map" = `engine-architect-decision-map.md`; "0001 draft", "0003 draft", "0004 draft", "0006
draft" = the other drafts' notes files in this folder; "AB section" = `game-designer-adr0005-abilities-section.md`;
"GPU section" = `graphics-engineer-adr0005-assets-section.md`.

Tiers: `research verified` (an R1 or R2 recommendation claim, or a finding its topic's verifier table lists as checked:
every R1-DATA-F1–F29 is checked, r1 lines 3501–3538; R1-BND-F9 at r1 line 3409; R6-HOST-F3 per
`inputs/findings-checked.tsv`) · `research, source to check` · `arithmetic` · `settled` (a STATUS code, GLOSSARY §8,
the plan's rulings, or a seam or interface the decision map assigns) · `design doc` (VISION_V2, LOOP_V2, EDITOR_V2,
LOOK_V2; CLAUDE.md and the agent file's starting principles are marked `settled`) · `proposed mark`.

## Content data section

| # | the claim, as written (short) | section | citation (file, id, lines) | tier |
|---|---|---|---|---|
| 1 | VISION_V2 §6.1 modes as data | Context | VISION_V2.md:64 | design doc |
| 2 | §6.3 quote "Units, vehicles, weapons and abilities are built from data, so combos and counters can emerge and tuning needs no code changes" | Context | VISION_V2.md:69–70 | design doc |
| 3 | §6.5 "All server-side and tunable" | Context | VISION_V2.md:74 | design doc |
| 4 | §6.6 every match records the numbers that matter | Context | VISION_V2.md:75–76 | design doc |
| 5 | §6.7 quote "per-faction units, uniforms and variants, as data" | Context | VISION_V2.md:77 | design doc |
| 6 | §6.8 moments the server recognizes | Context | VISION_V2.md:78–79 | design doc |
| 7 | LOOP_V2 line 8 quote "Keep them all in one data table …" | Context | LOOP_V2.md:8 | design doc |
| 8 | LOOP_V2 §7 feel kit "kept as data", "tuned live in a small Feel Lab", `[DECIDED]` | Context | LOOP_V2.md:90–91 | design doc |
| 9 | EDITOR_V2 §2 rules 1, 2, 5 as summarized | Context | EDITOR_V2.md:16–26 | design doc |
| 10 | EDITOR_V2 §3 inspector, objectives as data, validation panel | Context | EDITOR_V2.md:30–40 | design doc |
| 11 | EDITOR_V2 §4: 1,000 random commands undone and redone give a byte-identical document; every v1 map migrates; exact round trips | Context | EDITOR_V2.md:45–48 | design doc |
| 12 | UPGRADE-SCOPE: a track per unit type and per weapon type, as data | Context | inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md; plan.md:68–75 | settled |
| 13 | Settled inputs: the stack, SLICE-ORDER condition 1, TICK-30, MODES-3 (a), SPIKE-SCOPE (a), BASELINE-WHEN (b), UPGRADE-SCOPE, LOOK_V2 | Context | STATUS.md:94–118, 187–188; plan.md:66–78 | settled |
| 14 | The foundation includes "TypeScript types generated from the Rust code" | Context | GLOSSARY §8 text (map, J1, lines 1068–1084); inputs/DRAFTS_BRIEF.md §2 | settled |
| 15 | Every R1 data-unit finding confirmed by its verifier | Context | r1, Verifier spot-check §6, lines 3501–3538 | research verified |
| 16 | `deny_unknown_fields` fails on unknown fields; JSON otherwise ignores them | Context | r1, R1-DATA-F1, 1402–1406 | research verified |
| 17 | cannot be combined with `flatten` | Context | r1, F2, 1407–1409 | research verified |
| 18 | `try_from` checks a value as it loads | Context | r1, F3, 1410–1413 | research verified |
| 19 | `default` and `alias` allow additive changes with no migration | Context | r1, F4, 1414–1419 | research verified |
| 20 | untagged enums give poor errors | Context | r1, F5, 1420–1423 | research verified |
| 21 | `float_roundtrip` exact "at an approximately 2x performance cost for parsing floats" | Context | r1, F6, 1426–1432 | research verified |
| 22 | serde_json errors carry line and column | Context | r1, F7, 1433–1437 | research verified |
| 23 | serde_path_to_error reports the field path | Context | r1, F8, 1438–1441 | research verified |
| 24 | a default recursion limit exists, value unstated | Context | r1, F9, 1442–1447 | research verified |
| 25 | a file can be read as an untyped tree and migrated before the typed load | Context | r1, F10, 1448–1452 (migration use inferred in F10) | research verified |
| 26 | schemars derives JSON Schema 2020-12 and follows serde attributes | Context | r1, F16, 1489–1493 | research verified |
| 27 | schemars reads garde ranges and doc comments | Context | r1, F17, 1494–1501 | research verified |
| 28 | garde validates with field paths; its page says nothing about Wasm | Context | r1, F18, 1502–1509 | research verified |
| 29 | RON "not designed to be a fully self-describing format"; `flatten` only in part | Context | r1, F12, 1459–1467 | research verified |
| 30 | TOML has no null | Context | r1, F13, 1468–1475 ("no null" inferred in F13, verifier confirmed) | research verified |
| 31 | serde's YAML crate archived | Context | r1, F14, 1476–1479 | research verified |
| 32 | Vite dev server pushes custom events to page modules; its pages do not cover workers | Context | r1, F22, 1526–1531 | research verified |
| 33 | `JSON.parse` makes `"__proto__"` an ordinary key; `Object.assign` calls setters | Context | r1, F23, 1534–1543 | research verified |
| 34 | OWASP: Map or Set for untrusted keys | Context | r1, F24, 1544–1547 | research verified |
| 35 | OWASP quote "Validate on the server even when the browser checks the same fields" | Context | r1, F25, 1548–1554 | research verified |
| 36 | hash in URL plus `immutable` is the documented caching pattern | Context | r1, F27, 1561–1565 | research verified |
| 37 | SHA-256 in workers, secure contexts only | Context | r1, F29, 1571–1575 | research verified |
| 38 | no shipped-game source read for this unit; Valve's pages answered HTTP 403 | Context; Trade-off | r1, lines 1577–1579 | research, source to check |
| 39 | Gives I-1, I-5 (content part), I-6, I-7 (file), I-9, I-8 (with the GPU section) | Context | map, part d, lines 802–810 | settled |
| 40 | ADR 0004 runs definitions and converts authored seconds into its integer time unit (its part 4) | Context; CD-6 | map I-1 (line 802); 0004 draft, lines 161–171 | settled |
| 41 | `content` is the sim's only dependency (R5-LAYOUT-REC2) | Context; CD-2 | r5, R5-LAYOUT-REC2, lines 1179–1181 | research, source to check |
| 42 | `packages/types` holds the generated types (R5-LAYOUT-REC3) | Context; CD-3 | r5, R5-LAYOUT-REC3, lines 1194–1198 | research, source to check |
| 43 | no-panic lints for the loader (R5-RUSTLINT-REC3) | Context; CD-2 | r5, lines 1340–1352 | research, source to check |
| 44 | golden fixtures (R5-TEST-REC3) | Context; CD-4; CD-10 | r5, lines 1431–1453 | research, source to check |
| 45 | the editor's build and double-click question is ADR 0002's (critic rank 25) | Context; CD-11 | map, part b XT table, rank 25 (line 687); J3 (lines 1161–1164) | settled |
| 46 | ADR 0001's `Join` and `JoinAccepted` carry the content hash | Context; CD-8 | 0001 draft, message catalogue, lines 481–482; map I-4 (line 805) | settled |
| 47 | ADR 0001's `HostCommand` carries a private room's overrides | Context; CD-9 | 0001 draft, line 486 | settled |
| 48 | a room is a mode id plus allowed overrides (R2-ROOM-REC7) | Context | r2, R2-ROOM-REC7, lines 2857–2867 | research verified |
| 49 | ADR 0006 serves the bundle from the client host and stores the record with content hash and mode id | Context; CD-8 | map I-6, I-10 (lines 807, 811); 0006 draft, lines 85, 128, 340 | settled |
| 50 | ADR 0003 owns the quality-tier rows and draws what the feel kit describes | Context; CD-14 | map part b (R3-BUDGET-REC3 owner 0003, line 464); 0003 draft, line 329 | settled |
| 51 | legacy-tuning.json: 60 tables, each with `where` and a group (gameplay, simulation, progression, look, state) | Context | r1, legacy facts, lines 1583–1586; verifier row "Legacy facts", line 3534 | research verified |
| 52 | dead keys in CD4, CD6, CD7, CD10, CD12, CD14 | Context; CD-10 | r1, lines 1594–1597 | research verified |
| 53 | `rulesMerge` and `envMerge` take any key, `__proto__` included (GW2, GE1); `RULES_SCHEMA` limits bind only sliders | Context | r1, lines 1603–1607; STATUS.md:257 (SEC-MDL-2) | research verified |
| 54 | v1 save writes `version:1` nothing reads, rounds to 0.1 m, no ids (GW12) | Context | r1, lines 1610–1612 | research verified |
| 55 | missing objective falls back to DM_Cabana's (GE11, CM1) | Context | r1, lines 1613–1615 | research verified |
| 56 | failed save falls back to memory silently; no import (ED13) | Context | r1, lines 1616–1618 | research verified |
| 57 | missing gun key falls back to the AMK (CC9) | Context | docs/LEGACY_INVENTORY.md:138 | research, source to check |
| 58 | CD12 asks to split simulation fields from client feel fields | Context; CD-6; CD-14 | r1, lines 1600–1602 | research verified |
| 59 | JSON for every content file; canonical writer (fixed key order, shortest round-trip text, no lossy rounding); `float_roundtrip`; integers within the double range; string ids | CD-1 | r1, R1-DATA-REC1, lines 2484–2487 | research verified |
| 60 | reasons: editor writes JSON, browser native, legacy JSON, schema both sides, exact floats | CD-1 | r1, lines 2488–2490 | research verified |
| 61 | "shortest round-trip number text" has no finding; mark 1 tests it | CD-1 | r1, lines 2491–2492 | research verified |
| 62 | CD-1 needs: §6.1, §6.3, §6.5, §6.7; EDITOR_V2 §2 rule 1, §4 | CD-1 | r1, REC1 Serves, line 2493; EDITOR_V2.md:16–17, 45–48 | research verified |
| 63 | loader stages: byte cap → tree with default depth limit → version and migrate → typed load (deny_unknown_fields, newtypes, tagged kinds, no untagged, no flatten) → garde → whole-set checks → frozen set with hash | CD-2 | r1, R1-DATA-REC2, lines 2500–2504 | research verified |
| 64 | errors name file, line and column, or field path | CD-2 | r1, lines 2504–2505 | research verified |
| 65 | "no fallback weapon or model" added to the whole-set checks | CD-2 | decision (CC9, line 138; AP-2); REC2 names "no fallback map", r1 line 2504 | research, source to check |
| 66 | a maximum length on every collection (R1-RR-OQ2) | CD-2 | r1, R1-RR-OQ2, lines 2933–2935 (rust-reviewer's proposal, not re-checked by the verifier, r1 lines 2917–2918) | research, source to check |
| 67 | loader returns errors, never panics (R1-RR-OQ3; R5-RUSTLINT-REC3) | CD-2 | r1, lines 2936–2940; r5, lines 1340–1352 | research, source to check |
| 68 | crate depends on nothing above it; runs on server, Worker, editor (Wasm), tools | CD-2 | r1, R1-DATA-REC2, lines 2505–2506 | research verified |
| 69 | each legacy failure is a stage (GW2, GE1, GE11, CM1) | CD-2 | r1, lines 2507–2508 | research verified |
| 70 | quote "but not array lengths" | CD-2 | r1, R1-RR-OQ2, line 2934 | research, source to check |
| 71 | SEC-MDL-2 disposed here: typed structs; fuzz corpus carries `__proto__` keys | CD-2 | map, J2 (line 1128); r1, REC2 mark 1, lines 2513–2514 | settled |
| 72 | no research number for the length bounds | CD-2 | r1, lines 2933–2935 (no number given) | research, source to check |
| 73 | depth limit value unknown (OQ4); garde on wasm32 unknown (OQ5); fallback: newtypes, hand checks, schemars attributes | CD-2 | r1, F9, F18; R1-DATA-OQ4, OQ5, lines 2799–2803 | research verified |
| 74 | schemars writes 2020-12 at build time; garde ranges and doc comments; inspector fields (min/max sliders, description tooltip, default and deprecated shown); Rust verdict authoritative; Ajv optional | CD-3 | r1, R1-DATA-REC3, lines 2518–2523 | research verified |
| 75 | CI fails when committed schema differs from generated | CD-3 | r1, line 2523 | research verified |
| 76 | R1-BND-REC4 names ts-rs for "content-schema types" | CD-3 | r1, R1-BND-REC4, lines 2297–2300 | research verified |
| 77 | outputs committed in `packages/types`, regenerated and diffed by the gate | CD-3 | r5, R5-LAYOUT-REC3, lines 1194–1198 | research, source to check |
| 78 | ts-rs has no runtime part; default `serde-compat` parses serde attributes | CD-3 | r1, R1-BND-F9, lines 941–946; verifier row line 3409 | research verified |
| 79 | a tool turning JSON Schema into TypeScript was not researched | CD-3; Options Q6 | absence in r1–r6 (no such finding; R1-XU-OQ8 names only the two sources, r1 lines 2886–2890) | research, source to check |
| 80 | CD-3 answers rank 31 (R1-XT-OQ13, R5-XT-OQ9) and R1-XU-OQ8 | CD-3 | r1, lines 3088–3091; r5, lines 2012–2015; r1, lines 2886–2890; map part b (owner 0005, lines 328, 692) | settled |
| 81 | additive changes via default/alias; breaking bump + one step on the tree; future version refused; v1 map first input; ED13 dedupe a step; golden fixtures | CD-4 | r1, R1-DATA-REC4, lines 2532–2535 | research verified |
| 82 | legacy `version:1` never read | CD-4 | r1, line 2536 | research verified |
| 83 | one table per kind, linked by validated ids (AB-4, E1) | CD-5 | r1, R1-ABL-REC5, lines 2448–2459; AB section, AB-4 | research verified |
| 84 | "duplicate ids" is in REC2's fuzz corpus | CD-5 | r1, line 2514 | research verified |
| 85 | REC1 says "one definition per file" | CD-5 | r1, line 2485 | research verified |
| 86 | R1 left open one file or many per kind | CD-5 | r1, R1-XU-OQ9, lines 2891–2894 | research, source to check |
| 87 | AB-4's kinds (tag registry, attribute sets, effects, abilities, weapons, units, vehicles, factions, upgrade tracks, drop and pickup values) | CD-6 | AB section, AB-4 (lines 135–145); r1, R1-ABL-REC5 | research verified |
| 88 | mode fields (team count, size, faction per team or "chosen by host", squad size, rules table, maps, bot policy, match length); checks: team count other than 2 or 3, size 0 | CD-6 | r2, R2-ROOM-REC7, lines 2857–2860 | research verified |
| 89 | movement data read by the bake too | CD-6 | r4, R4-FLOORS-REC3, lines 941–945 | research, source to check |
| 90 | quality-tier table (R3-BUDGET-REC3) is client content | CD-6; CD-14 | r3, lines 1166–1170 | research, source to check |
| 91 | split follows R1-DATA-REC8's simulation and client groups | CD-6 | r1, lines 2596–2598 | research verified |
| 92 | durations authored in seconds, converted to ticks when loaded | CD-6 | r1, R1-TICK-REC1, lines 2012–2013 | research verified |
| 93 | ADR 0004 converts and reports each rounding | CD-6 | 0004 draft, part 4, lines 167–170 | settled |
| 94 | the tick is a build-time constant, never a server option, never changed by a reload | CD-6; Appendix A | r1, R1-TICK-REC1, lines 2011, 2015–2016; 0004 draft, part 4, lines 162–165 | research verified |
| 95 | R1-XU-OQ10 asks whether the tick constant may change on reload | CD-6 | r1, lines 2895–2898 | research, source to check |
| 96 | weapon definition names its track; unit names its class's own weapons | CD-6 | AB section, AB-4 (lines 141–145); UPGRADE-SCOPE (plan.md:70–73) | settled |
| 97 | top speed read by ADR 0001 (velocity bounds, interest margin) and ADR 0004 (movement) | CD-6; OQ-CD8 | map I-1 (line 802); r2, R2-XU-OQ14, lines 3231–3238 (the figures feed the lead, motion margin, velocity bounds); 0001 draft line 287; 0004 draft line 831 | settled |
| 98 | three figures: about 9.0, about 10.2, at most about 13.3 m/s | CD-6 | r2, R2-XU-OQ14, lines 3231–3236 (the R2 fix pass: the verifier found the numbers match the legacy docs, lines 3239–3240) | research, source to check |
| 99 | development server: notify with debouncer; whole-set re-validation; invalid keeps old set and reports to log and dev tools | CD-7 | r1, R1-DATA-REC5, lines 2542–2546 | research verified |
| 100 | small Vite plugin pushes changed files to the page, which hands them to the Worker | CD-7 | r1, lines 2546–2547 | research verified |
| 101 | client-only feel data reloads freely since presentation decides nothing | CD-7; CD-14 | r1, lines 2549–2550 | research verified |
| 102 | production: one content set per match; changes at the next match | CD-7 | r1, lines 2550–2551 | research verified |
| 103 | "swap at the next match" is R1-DATA-OQ12's own option | CD-7; Options Q4 | r1, R1-DATA-OQ12, lines 2824–2828 | research, source to check |
| 104 | stale bake reported (R4-FLOORS-REC3) | CD-7 | r4, lines 943–945, 950–951 | research, source to check |
| 105 | ADR 0001's `ContentReload` exists only if ADR 0005 keeps development reload over the game connection | CD-7 | 0001 draft, line 488 | settled |
| 106 | REC5's reason: no half-written file or mid-match change breaks a match | CD-7 | r1, lines 2552–2553 | research verified |
| 107 | quote "a development convenience no §6 item names" (verifier's partial trace) | CD-7; OQ-CD4; Appendix A | r1, lines 2555–2558 | research verified |
| 108 | plan rule 4: propose only what a need names | CD-7 | plan.md:103–108 | settled |
| 109 | OQ8 (Vite into a worker) is the boundary spike's | CD-7; Appendix A | map, part g, line 942; r1, R1-DATA-OQ8, lines 2811–2812 | settled |
| 110 | §6.3 asks for tuning without code changes, not live reload | CD-7 | r1, lines 2555–2557; VISION_V2.md:69–70 | research verified |
| 111 | bundle file name holds its hash; served immutable | CD-8 | r1, R1-DATA-REC6, lines 2566–2567 | research verified |
| 112 | room names the hash at join; Worker checks before the sim starts | CD-8 | r1, lines 2567–2569 | research verified |
| 113 | mismatch → `JoinRejected`, client reloads the bundle | CD-8 | 0001 draft, failure table, line 554 | settled |
| 114 | SHA-256 needs a secure context; plain-HTTP LAN development needs another path | CD-8 | r1, lines 2567–2569 | research verified |
| 115 | per-match data over the game connection; not embedded in Wasm; tuning without a code release | CD-8 | r1, lines 2569–2571 | research verified |
| 116 | every match record stores content hash and mode id; rates grouped by tuning version | CD-8; CD-12 | r1, lines 2571–2572 | research verified |
| 117 | where shipped bundles are kept is ADR 0006's (I-6) | CD-8 | map I-6, line 807 | settled |
| 118 | 25 MiB per-file limit on the client host | CD-8; AP-4 | r6, R6-HOST-F3, lines 72–79 | research verified |
| 119 | R6-HOST-REC1's mark (bundle immutable, 0 bytes on repeat, largest file under 25 MiB) | CD-8; AP-4 | r6, lines 639–644 | research, source to check |
| 120 | quote "client and server must run the same numbers for prediction; telemetry must know which numbers a match used" | CD-8 | r1, line 2573 | research verified |
| 121 | Rust SHA-256 crate not read (R1-DATA-OQ9) | CD-8 | r1, lines 2813–2814 | research, source to check |
| 122 | allow list of tunables with ranges from the schema, never free paths | CD-9 | r1, R1-DATA-REC7, lines 2583–2584 | research verified |
| 123 | allow-list contents are a design question (R1-DATA-OQ10, Mustafa with the game-designer) | CD-9 | r1, lines 2815–2816; map part b, line 318 (DESIGN) | settled |
| 124 | never `Object.assign` or spread on parsed JSON; `Map` or the Wasm validator | CD-9 | r1, lines 2585–2586 | research verified |
| 125 | REC7 includes a map-upload path for private matches | CD-9 | r1, lines 2584–2586 | research verified |
| 126 | no VISION_V2 §6 item asks for map upload | CD-9 | VISION_V2.md:62–81 | design doc |
| 127 | REC7's reason: client checks bypassable; legacy merge let `__proto__` in | CD-9 | r1, line 2587 | research verified |
| 128 | the allow list waits on the security baseline (BASELINE-WHEN (b)) | CD-9 | STATUS.md:112–113; map f2, line 906 | settled |
| 129 | import mapping: gameplay and simulation → simulation content; progression → client; state dropped; `boot_changes` → map data; `where` kept; dead keys dropped or kept by decision; starting values | CD-10 | r1, R1-DATA-REC8, lines 2596–2600 | research verified |
| 130 | superseded words "leaves the look group pending the look brief" | CD-10; Appendix A | r1, lines 2597–2598 | research verified |
| 131 | LOOK_V2 exists (2026-10-31) | CD-10 | docs/LOOK_V2.md:1–5 | design doc |
| 132 | visible values are tuning for the look proof test (plan rule 6) | CD-10 | plan.md:114–122 | settled |
| 133 | quote "filled from the measured legacy values without carrying dead tuning forward" | CD-10 | r1, line 2601 | research verified |
| 134 | map: one versioned JSON document, stable ids (rule 1); commands with do and undo (rule 2); document layer without THREE, tested in Node (rule 5) | CD-11 | EDITOR_V2.md:16–26 | design doc |
| 135 | editor document layer calls the crate through Wasm | CD-11 | r1, REC2, lines 2505–2506 ("the editor (through Wasm)") | research verified |
| 136 | static colliders from the map document with the sim's entity ids | CD-11; AP-1 | r4, R4-PHYS-REC2, lines 723–725; r1, R1-ECS-REC3, lines 1973–1975 | research, source to check |
| 137 | baked navigation stored in the map document with the movement-data hash | CD-11 | r4, R4-NAV-REC1, lines 802–803; R4-FLOORS-REC3, lines 943–945 | research, source to check |
| 138 | floors, ceilings, enclosure areas and portals | CD-11 | r4, R4-FLOORS-REC2, lines 925–929 | research, source to check |
| 139 | structure states; the third damage stage is ADR 0004's question | CD-11 | r4, R4-OBST-REC2, lines 1043–1061; map part h, G4, line 982 | research, source to check |
| 140 | objectives as data | CD-11 | EDITOR_V2.md:37–38 | design doc |
| 141 | the map's bounds (CD21's note: the map document defines the bounds) | CD-11 | docs/LEGACY_INVENTORY.md:102 | research, source to check |
| 142 | per-map lighting mood | CD-11 | LOOK_V2.md:55–58 | design doc |
| 143 | building specs in the document (ED10's note) | CD-11; AP-7 | docs/LEGACY_INVENTORY.md:345 | research, source to check |
| 144 | schema fields are EDITOR_V2 §6 step 2's architecture plan | CD-11 | EDITOR_V2.md:58 | design doc |
| 145 | rules 3 and 4 are the editor's scene, listed; ADR 0003 shares only the material cache; no draft designs the editor | CD-11 | map, J3, lines 1156–1160 | settled |
| 146 | the double-click rule (EDITOR_V2 §5) | CD-11 | EDITOR_V2.md:51 | design doc |
| 147 | definition ids key the telemetry; the record's shape is ADR 0006's (R6-TEL-REC1) | CD-12 | r6, R6-TEL-REC1, lines 784–811; AB section, AB-7 | research, source to check |
| 148 | no research on a moment-rule format (gap G11) | CD-12; OQ-CD6 | map part h, G11, line 989 | settled |
| 149 | moment thresholds are a design question (R6-TEL-OQ3) | CD-12 | map part b, line 665 (R6-TEL-OQ3 DESIGN) | settled |
| 150 | §6.6: "Snowball and counters get tuned on evidence" | CD-12 | VISION_V2.md:76 | design doc |
| 151 | the three T1 classes are shared by every faction | CD-13 | LOOP_V2.md:67 (`[DEFAULT]`) | design doc |
| 152 | soldiers dressed by (faction, class) as data (CC6's note) | CD-13 | docs/LEGACY_INVENTORY.md:136 | research, source to check |
| 153 | overrides bring back merge rules (R1-DATA-OQ6) | CD-13; OQ-CD5 | r1, lines 2804–2806 | research, source to check |
| 154 | team colours never change | CD-13 | VISION_V2.md:60 | design doc |
| 155 | R3-CHAR-REC1's per-instance colour serves §6.7 | CD-13 | r3, lines 1042–1043 | research, source to check |
| 156 | LOOK_V2 §2 uniform details as the seed of faction identity | CD-13 | LOOK_V2.md:18–19 | design doc |
| 157 | feel kit list: kick, muzzle flash, tracer, impact, hit marker, sound, target flinch, kill confirm | CD-14 | LOOP_V2.md:90–91 | design doc |
| 158 | CC9's note: merge the socket table with the feel kit; sockets cosmetic | CD-14; AP-1 | docs/LEGACY_INVENTORY.md:138 | research, source to check |
| 159 | no research on the Feel Lab tool (gap G15) | CD-14; OQ-CD7 | map part h, G15, line 993 | settled |
| 160 | R1-DATA-OQ12 notes the Feel Lab covers client feel data | CD-14; OQ-CD7 | r1, line 2826 | research, source to check |
| 161 | quality tiers: VISION_V2 §4 only | CD-14 | r3, R3-BUDGET-REC3 Serves, line 1174 | research, source to check |
| 162 | Options Q1 table: JSON lowest complexity, no second parser; 2x float parse (F6); RON two writers, no first-class TS parser, partial tag/flatten, no round-trip guarantee (F12); TOML no null (F13), arrays of tables awkward (line 1841); binary format not researched (OQ2); YAML archived (F14) | Options Q1 | r1, Options §6, Q1, lines 1836–1843 | research verified |
| 163 | the editor would parse RON through the Wasm loader | Options Q1 | r1, line 1840 | research verified |
| 164 | Options Q2 table: A one crate (flatten unavailable, F2); B "two validators of one schema can disagree"; C ruled out (F25) | Options Q2 | r1, lines 1845–1855 | research verified |
| 165 | Options Q3 table: A version + tree migrations; B versioned types multiply; C ruled out by EDITOR_V2 §4 | Options Q3 | r1, lines 1857–1863 | research verified |
| 166 | Options Q4 table: A (REC5 as written), B production live tuning (telemetry and fairness, lines 1871–1872), C no hot reload (against LOOP_V2 and the Feel Lab, lines 1873–1874) | Options Q4 | r1, lines 1865–1874 | research verified |
| 167 | Options Q5 table: embedded (F28), bundle (F27, F29), sent at join | Options Q5 | r1, lines 1876–1882 | research verified |
| 168 | ts-rs is option D3 of R1's boundary options | Options Q6 | r1, line 1768 | research verified |
| 169 | Options Q7: (b) is the legacy tables' shape (CD12–CD14) | Options Q7 | r1, legacy facts, lines 1587–1597 | research verified |
| 170 | serde_json, garde and regex enlarge the Worker's Wasm; no size budget (R1-RR-OQ5) | Trade-off; OQ-CD2 | r1, lines 2946–2950 | research, source to check |
| 171 | exact floats double float-parse time | Trade-off | r1, F6, 1426–1432 | research verified |
| 172 | OQ-CD1 options: schema descriptions and tooltips (F15, F17); JSONC or JSON5 not researched; decided by Mustafa's editing habit and a spike trial | OQ-CD1 | r1, R1-DATA-OQ1, lines 2788–2790 | research, source to check |
| 173 | quote "against a budget the ADR sets (no number is sourced yet)" | OQ-CD2 | r1, REC1 mark 2, lines 2497–2498 | research verified |
| 174 | postcard and bincode not read | OQ-CD2 | r1, R1-DATA-OQ2, lines 2792–2793 | research, source to check |
| 175 | ADR 0002 sets the Wasm size budget from the measurement (N50) | OQ-CD2 | map c4, N50, line 786 | settled |
| 176 | a mid-match rule change makes telemetry and fairness harder | OQ-CD3 | r1, lines 1871–1872 | research verified |
| 177 | Valve's console-variable pages answered HTTP 403; SRC-ACCESS (a) | OQ-CD3 | r1, R1-DATA-OQ3, lines 2795–2798; STATUS.md:167–171 | settled |
| 178 | ADR 0001 answers the protocol side of critic rank 13 | OQ-CD3 | 0001 draft, Appendix A, line 1095 | settled |
| 179 | option (b) needs REC5's marks 2 and 3 | OQ-CD4 | r1, lines 2560–2562 | research verified |
| 180 | what a reload may change: numbers yes; new or removed definitions, map geometry and navigation at the next match; clamp or scale is the game-designer's | OQ-CD4 | r1, R1-DATA-OQ7, lines 2807–2810 | research, source to check |
| 181 | R4-PHYS-REC2: the static world never changes mid-match, so "map geometry probably needs a match restart" holds | OQ-CD4 | r4, lines 733–734 | research, source to check |
| 182 | R4-FLOORS-OQ4: re-bake on the spot or at the next match | OQ-CD4 | r4, lines 1119–1120 | research, source to check |
| 183 | OQ-CD4 deciders: Mustafa and the Advisor | OQ-CD4 | r1, R1-DATA-OQ12, lines 2827–2828; map part b, line 320 | settled |
| 184 | OQ-CD5 decider: the faction design session (VISION_V2 §7) | OQ-CD5 | r1, lines 2805–2806; VISION_V2.md:86 | research, source to check |
| 185 | quote "a last-second steal or a comeback" | OQ-CD6 | VISION_V2.md:78 | design doc |
| 186 | LOOP_V2 §11 leaves the feel pass out of the greybox v0 | OQ-CD7 | LOOP_V2.md:156–157 | design doc |
| 187 | the Banzai speed buff adds speed (legacy 1.3× speed) | OQ-CD8 | r1, legacy facts table, line 1373; AB section, AB-6 | research verified |
| 188 | ADR 0001's velocity bounds and interest margin read the top speed | OQ-CD8 | r2, lines 3231–3238; 0001 draft, line 287 | research, source to check |
| 189 | Spike mark 1: every double + 10,000 random doubles same 64-bit pattern native, Wasm, `JSON.parse`; same text back; 100 % | Spike 1 | r1, REC1 mark 1, lines 2494–2496 | research verified |
| 190 | Spike mark 2: 10,000 mutated files (list), 0 crashes, 0 accepted invalid, same verdict and path on both, 100 %; errors name file and line/column or path | Spike 2 | r1, REC2 marks, lines 2513–2516 | research verified |
| 191 | Spike mark 2 addition: one element over each bound, refused 100 % | Spike 2 | proposed; reason R1-RR-OQ2 (r1, 2933–2935) | proposed mark |
| 192 | zero panics is R1-RR-OQ3's mark for the loader | Spike 2 | r1, lines 2937–2939 | research, source to check |
| 193 | Spike mark 3: schema min/max for 100 % of ranged fields; value outside refused and flagged; schema check fails on changed type | Spike 3 | r1, REC3 mark, lines 2527–2529 | research verified |
| 194 | Spike mark 3: zero diff on regeneration; `tsc --strict` passes | Spike 3 | r1, R1-BND-REC4 marks 1 and 3, lines 2304–2306 | research verified |
| 195 | Spike mark 3: rename without regenerating fails the gate | Spike 3 | r5, R5-LAYOUT-REC3 mark, lines 1203–1204 | research, source to check |
| 196 | Spike mark 3 addition: legacy-import files typed with the generated TS pass the Rust validator | Spike 3 | proposed; reason F16 (r1 1489–1493), R1-BND-F9 (r1 941–946) | proposed mark |
| 197 | Spike mark 4: Cabana, DM_Tri and Mustafa's maps migrate (100 %); byte-identical round trip; future version refused | Spike 4 | r1, REC4 mark, lines 2539–2540 | research verified |
| 198 | Spike mark 5: weapon damage change validated within 1 s; next match uses it; malformed save shows error within 1 s | Spike 5 | r1, REC5 marks 1 and 4, lines 2559–2563 (restated: mark 1's "applies" becomes "validated, next match") | research verified |
| 199 | Spike mark 5 addition: a feel-kit change shows on the page within 1 s | Spike 5 | proposed; reason REC5 mark 1's 1 s (r1, line 2559) on the client path | proposed mark |
| 200 | Spike mark 6: 3 tampering cases 100 %; repeat visit 0 bytes; 100 % of records resolve to an archived set | Spike 6 | r1, REC6 mark, lines 2576–2579 | research verified |
| 201 | Spike mark 6: Rust SHA-256 equals WebCrypto on every spike bundle | Spike 6 | r1, R1-DATA-OQ9, lines 2813–2814 | research, source to check |
| 202 | Spike mark 7: corpus through the override path, 0 accepted, 0 crashes; `__proto__` refused; prototypes unchanged | Spike 7 | r1, REC7 mark, lines 2591–2594 (restated without map upload) | research verified |
| 203 | Spike mark 8: 100 % of non-look, non-state values mapped or dropped; imported set passes | Spike 8 | r1, REC8 mark, lines 2605–2606 | research verified |
| 204 | Spike mark 8 addition: 100 % of look values in the report, marked for the look proof test | Spike 8 | proposed; reason CD-10's change | proposed mark |
| 205 | Spike mark 9: load time of the legacy tables plus two maps and two modes on the native server, reported | Spike 9 | r1, REC1 mark 2, lines 2496–2498 | research verified |
| 206 | Spike mark 9 addition: the same load in Wasm in the Worker, and serde_json and garde's size recorded | Spike 9 | proposed; reason R1-RR-OQ5 (r1 2946–2950) | proposed mark |
| 207 | Spike mark 10: mode files (R2-ROOM-REC7's mark) are ADR 0001's | Spike 10 | r2, lines 2864–2867; map part b (R2-ROOM-REC7 owner 0001) | research verified |
| 208 | the content and ability spike is a working name; SPIKE_PLAN names and places spikes | Spike intro | plan.md:113 (rule 5); map part g, line 944 | settled |

## Asset pipeline section

| # | the claim, as written (short) | section | citation (file, id, lines) | tier |
|---|---|---|---|---|
| 209 | LOOK_V2 §8 quotes: "v2 may use authored 3D models" `[DECIDED]`; the flow `[DEFAULT]`; "an automated step that compresses it, builds LODs and renames materials into the faction colour slots"; "the engine applies the comic look"; "triangles, materials and textures"; "The game never runs Spline's own runtime"; tree style `[OPEN]` | Context | LOOK_V2.md:66–74 | design doc |
| 210 | LOOK_V2 items the models carry: §1, §5 silhouettes; §2 uniform details, architecture; §3 proportions, weapon silhouettes | Context | LOOK_V2.md:10, 18–22, 28–30, 44–47 | design doc |
| 211 | VISION_V2 §6.7, §6.9, §4 | Context | VISION_V2.md:77, 80, 52–54 | design doc |
| 212 | ASSET-RESEARCH (a): only what R1-DATA, R3-CHAR, R3-BUDGET support; the five items "not researched", one research unit (graphics-engineer, verified like R3, own daytime session), before the asset spike | Context | plan.md:184–191, 297–298; map J4, lines 1179–1185 | settled |
| 213 | no hit for KTX2, meshopt, Draco, Spline, gltf-transform or Basis Universal; one glTF mention | Context | plan.md:285–289 (Risks 3); map J4 correction, lines 1184–1185 | settled |
| 214 | R3-CHAR-F8: three.js's example loads a glTF skinned mesh | Context | r3, R3-CHAR-F8, lines 476–480 (not in R3's checked set) | research, source to check |
| 215 | static files with hash in URL plus `immutable` (F27) | Context; AP-4 | r1, lines 1561–1565 | research verified |
| 216 | client host limit 25 MiB per file | Context; AP-4 | r6, R6-HOST-F3, lines 72–79 | research verified |
| 217 | budgets as reviewed data in the repository | Context; AP-5 | r5, R5-GATE-REC4, lines 1734–1737 | research, source to check |
| 218 | quote "built from the map document, never from render objects" | Context; AP-1 | r4, R4-PHYS-REC2, lines 723–724 | research, source to check |
| 219 | quote "each part's transform from the sim's vehicle-part records" | Context; AP-1 | r3, R3-CHAR-REC4, lines 1077–1078 | research, source to check |
| 220 | the GPU section derives its caps from R3-BUDGET-REC2 and R3-CHAR-REC1 | Context | GPU section, part 4 (lines 41–50) | settled |
| 221 | the GPU section's parts 1–9 and OQ-0005-AS1 to AS6 (compression, LOD tool, export checks, renaming, per-unit budgets, soldier counts) | Context; AP-2; AP-6 | GPU section, lines 17–68, 106–138 | settled |
| 222 | ADR 0003 loads models, gives each slot its class's toon material, switches LOD, colours faction slots per instance (parts 5, 6) | Context | 0003 draft, lines 167, 259–274 | settled |
| 223 | ADR 0003 proposes authored soldiers, weapons and very large units (rows CL10, CC1–CC5, CC6, CC8, GC16) | Context; AP-7; OQ-AP2 | 0003 draft, lines 690–691, 699 | settled |
| 224 | takes: budget file (I-14), dependency checks and manifest walk (ADR 0002), the host (ADR 0006), vehicle-part records and static world (ADR 0004) | Context | map part d, I-9, I-14 (lines 810, 815); seams S5, S9 (lines 207, 211) | settled |
| 225 | legacy: no external assets (CLAUDE.md hard constraint) | Context | CLAUDE.md, "Hard constraints" | settled |
| 226 | legacy builds assets in code: CL10, CC1, CM9, CM10, GW18, GW19, CL13, CM5 | Context | docs/LEGACY_INVENTORY.md:119, 131, 157, 158, 206, 207, 122, 153 | research, source to check |
| 227 | quote "The legacy rule 'no external assets' ends with the legacy engine." | Context; For the frame | LOOK_V2.md:66 | design doc |
| 228 | blasts raycast render meshes (GW15); collider sizes from render maths (GW9) | Context; AP-1 | r4, lines 730–731 | research, source to check |
| 229 | CL14: unchecked `userData` tag contract | Context; AP-2 | docs/LEGACY_INVENTORY.md:123 | research, source to check |
| 230 | CL11: hit shape "rewrite in Rust" | AP-1 | docs/LEGACY_INVENTORY.md:120 | research, source to check |
| 231 | CC9's note: sockets cosmetic, shots from fixed points | AP-1 | docs/LEGACY_INVENTORY.md:138 | research, source to check |
| 232 | the GPU section's part 3: part names match the sim's vehicle-part records | AP-1 | GPU section, lines 35–40 | settled |
| 233 | quote "Presentation never decides a hit, a death or a purchase" | AP-1 | .claude/agents/engine-architect.md:25–26 | settled |
| 234 | CLAUDE.md "Animation is cosmetic … Nothing animated decides a hit" | AP-1; For the frame | CLAUDE.md, "Hard constraints" | settled |
| 235 | the GPU section's part 5: LOD levels optional data | AP-2 | GPU section, lines 51–57 | settled |
| 236 | R1-DATA-REC2's cross-reference stage; R1-ABL-REC5's validated ids | AP-2 | r1, lines 2503–2504; 2448–2459 | research verified |
| 237 | the GPU section's part 2: classes "for example environment, unit, faction colour, emissive"; the list is this ADR's content data | AP-3 | GPU section, lines 27–30 | settled |
| 238 | R3-CHAR-REC3 (per-instance colour) | AP-3 | r3, lines 1065–1075 | research, source to check |
| 239 | LOOK_V2 §6.1 value hierarchy; §6.4 glow language | AP-3 | LOOK_V2.md:50–51, 54 | design doc |
| 240 | quote "A modern best practice for static resources is to include version/hashes in their URLs, while never modifying the resources" | AP-4 | r1, F27, lines 1563–1564 | research verified |
| 241 | the GPU section's two caps: a soldier as drawn at most 4 materials per pass; a very large unit at most 25 parts × materials per pass | AP-5 | GPU section, part 4 (lines 42–47): 4 draws per soldier per pass (r3, R3-CHAR-REC1, lines 1044–1045); 200 draws ÷ 4 units ÷ 2 passes = 25 (r3, R3-CHAR-REC4, lines 1088–1090) | arithmetic |
| 242 | quote "raising a budget is a reviewed change with its reason" | AP-5 | r5, R5-GATE-REC4, line 1736 | research, source to check |
| 243 | the frame totals stay ADR 0003's (R3-BUDGET-REC2) | AP-5 | r3, lines 1118–1137 | research, source to check |
| 244 | quote "tools may depend on everything" | AP-6 | .claude/agents/engine-architect.md:33–34 | settled |
| 245 | quote "No other crate or package until a §6 item asks for it" | AP-6 | r5, R5-LAYOUT-REC2, line 1178 | research, source to check |
| 246 | the manifest walk checks what the player page loads | AP-6 | r5, lines 1182–1183 | research, source to check |
| 247 | ED10's note: the spec "must live in the v2 document entity" | AP-7 | docs/LEGACY_INVENTORY.md:345 | research, source to check |
| 248 | CM7's note: "Migrate houses[].bt entries to kit specs" | AP-7 | docs/LEGACY_INVENTORY.md:155 | research, source to check |
| 249 | every v1 map loads through a migration | AP-7; Trade-off | EDITOR_V2.md:48 | design doc |
| 250 | the GPU section's part 9: trees wait | AP-7 | GPU section, lines 67–68 | settled |
| 251 | Options: delivery options are R1's Q5 applied to assets | Options | r1, lines 1876–1882 | research verified |
| 252 | 20,000 files per site on the Free plan | Options | r6, R6-HOST-F3, line 72 | research verified |
| 253 | no asset count is known yet | Options | absence in r1–r6 and the GPU section (OQ-0005-AS5) | research, source to check |
| 254 | legacy builds buildings, props, ground and textures in code (CM5–CM10, CM12, GW19, CL13, CD20, CD21) | OQ-AP2 | docs/LEGACY_INVENTORY.md:153–158, 160, 207, 122, 101, 102 | research, source to check |
| 255 | quote "AI-generated for drafts, or a clean model built by Claude" | OQ-AP3 | LOOK_V2.md:69 | design doc |
| 256 | look proof test: dangling id and unknown class fail at load naming file and field | Spike (assets) | proposed; reason REC2's error mark, r1 lines 2515–2516 | proposed mark |
| 257 | asset spike: assets immutable, repeat visit 0 bytes | Spike (assets) | proposed; reason r1 lines 2577–2578 and r6 lines 641–642, applied to assets | proposed mark |
| 258 | asset spike: every file under 25 MiB | Spike (assets) | r6, R6-HOST-REC1 mark, lines 642–643 | research, source to check |
| 259 | asset spike: planted over-budget asset fails; a raise without the file fails | Spike (assets) | proposed; R5-GATE-REC4 mark, r5 lines 1743–1744, applied to asset budgets | proposed mark |
| 260 | asset spike: planted glTF dependency in `sim` fails the gate | Spike (assets); AP-1 | proposed; reason AP-1, method r5 lines 1189–1192 | proposed mark |

## Appendix A and "For the frame"

| # | the claim, as written (short) | section | citation (file, id, lines) | tier |
|---|---|---|---|---|
| 261 | 26 ids are this unit's: owner 0005, neither the ability section's (7) nor the GPU section's (1) | Appendix A | engine-architect-decision-map-ids.tsv (34 rows with owner 0005); AB section Appendix A (lines 577–586); GPU section Appendix A (line 154) | settled |
| 262 | counts: adopted 4, changed 4, answered 7, open 6, spike 5 (26) | Appendix A | the table's rows | arithmetic (4 + 4 + 7 + 6 + 5 = 26) |
| 263 | features for README's list (need outside §6) | For the frame | map J3, lines 1152–1154 | settled |
| 264 | CLAUDE.md "No external assets (procedural textures, synthesized audio)" stops applying to v2 code under LOOK_V2 §8 | For the frame | CLAUDE.md, "Hard constraints"; LOOK_V2.md:66; plan.md:139–141 (rule 11) | settled |
| 265 | "Persistence only through KVStore" untouched | For the frame | CLAUDE.md, "Hard constraints" | settled |
| 266 | ADR 0003 leaves the building kit, kit and ground textures and builders, tree and prop builders and other asset rows to ADR 0005 | For the frame | 0003 draft, lines 675–677 | settled |
| 267 | inventory rows and their marks now: CM1 149, CM2 150, CM4 152 redesign (pending); CM7 155, CM5 153, CM6 154, CM8 156, CM9 157, CM10 158, CM12 160, CL13 122, CD20 101, CD21 102, GW18 206, GW19 207 port to TypeScript (pending); GW20 208, ED9 344, ED10 345 redesign (pending) | For the frame | docs/LEGACY_INVENTORY.md at those lines (Mark column) | research, source to check |
| 268 | quote "The thatched roof goes in its current form" | For the frame | LOOK_V2.md:31 | design doc |
| 269 | LOOK_V2 §2 keeps the half-timbered house with the brick-tile roof | For the frame | LOOK_V2.md:21–22 | design doc |
| 270 | LOOK_V2 §2 props "as a base to build on" | For the frame | LOOK_V2.md:17 | design doc |
| 271 | LOOK_V2 §6.5 moods per map (`[DEFAULT]`) | For the frame | LOOK_V2.md:49, 55–58 | design doc |
| 272 | the content data parts change no mark; rows relied on: CD4, CD6, CD7, CD10, CD12, CD14, GW2, GE1, GW12, GE11, CM1, ED13, CD2, CC9, TL11 | For the frame | docs/LEGACY_INVENTORY.md:85, 87, 88, 91, 93, 95, 190, 287, 200, 297, 149, 348, 83, 138, 396 | research, source to check |
| 273 | ED5, ED7 follow ADR 0003's CL10 and GA rows | For the frame | 0003 draft, lines 690, 693 (ED5, ED7 unlisted there) | settled |
| 274 | the netcode proof test runs first (SLICE-ORDER condition 3) | For the frame | STATUS.md:101; GLOSSARY §8 (map J1, lines 1086–1101) | settled |
| 275 | slices 1–6 order | For the frame | STATUS.md:14–15, 94–95; inputs/DRAFTS_BRIEF.md §1 | settled |
| 276 | the content and ability spike: after the sim core spike, before slice 1 | For the frame | map part g, line 944 | settled |
| 277 | insta snapshots of validation errors (R5-TEST-REC3) | For the frame | r5, lines 1442–1443 | research, source to check |
| 278 | garde fallback (newtypes, hand checks) | For the frame | r1, lines 2801–2803 | research, source to check |
| 279 | ADR 0001's catalogue holds `ContentReload` | For the frame | 0001 draft, line 488 | settled |
| 280 | seams the other drafts carry (S5, S9, S10; I-1 to I-9, I-14) | For the frame | map, seams lines 207, 211, 212; part d lines 802–815 | settled |
| 281 | R3-CHAR-OQ3 is the GPU section's (OQ-0005-AS6) | For the frame | GPU section, line 154 | settled |

Counts by tier (281 rows, counted by script from the tier column): research verified 106 · research, source to check
69 · arithmetic 2 · settled 59 · design doc 36 · proposed mark 9.
