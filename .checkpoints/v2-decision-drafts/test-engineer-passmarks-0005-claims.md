# Claims ledger: ADR-0005's pass marks (test-engineer), 2026-11-01

A notes file beside `test-engineer-passmarks-0005.md`, in the plan's ledger shape (rule 9). It joins ADR 0005's four
ledgers (`engine-architect-adr0005-frame-claims.md`, `engine-architect-adr0005-content-claims.md`,
`game-designer-adr0005-abilities-claims.md`, `graphics-engineer-adr0005-assets-claims.md`) [C15]; rows are numbered
TE1, TE2 … so they never clash with those ledgers' numbers. One row per claim or number in the pass-mark subsection
and the spike entries. Lines are at 9e7db28 (the research files, `docs/legacy-tuning.json`, `docs/LEGACY_INVENTORY.md`,
LOOP_V2, VISION_V2 and EDITOR_V2 are unchanged at the head: `git diff --quiet 9e7db28 HEAD --` on them exits 0,
checked 2026-11-01). Short names: r1 = `docs/research/r1-simulation-core.md`, r2 = `r2-netcode.md`, r3 =
`r3-rendering.md`, r5 = `r5-toolchain-quality.md`, r6 = `r6-infrastructure.md`, GL = `GLOSSARY.md`, INV =
`docs/LEGACY_INVENTORY.md`, LT = `docs/legacy-tuning.json`, draft = `engine-architect-adr0005-draft.md` (its sections
named CD, AB, AP, GPU), PM3 = `test-engineer-passmarks-0003.md`, PM4 = `-0004.md`, PM2 = `-0002.md`, PM6 = `-0006.md`,
map = `engine-architect-decision-map.md`, plan = `plan.md`.

Tiers, aligned with the earlier pass-mark ledgers: `research verified` = an R1 or R2 recommendation's text, its
proposed pass marks included · `research, source to check` = R3–R6 recommendations (their proposed marks included), any
finding outside a verifier's checked set, an open question or a legacy fact · `arithmetic` (shown; a count of a
committed file is arithmetic done by script, its command named) · `settled` (a STATUS code, GLOSSARY, the plan, the
map's assignment, or another draft's pass-mark row read as written) · `design doc` (VISION_V2, LOOP_V2, LOOK_V2,
EDITOR_V2) · `proposed (the draft's)` (a mark ADR 0005's draft proposes, with its reason there) · `proposed
(test-engineer)` (a method choice, reading, count, control or planted defect of this table, with its reason). Every pass
mark in R1–R6 is itself "proposed" in the research; the first tier names where the number comes from. Where a row rests
on several tiers, all are named, weakest last.

Mark codes: PRE the "How to read the table" bullets · PM-n the table row · FAIL the failure rule · NOM the "no
measurable mark" lines · CC the content checklist · FC the families checklist · LC the LOD checklist · FL the Feel Lab
checklist · SE the spike entries.

## How to read the table

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE1 | RS: one vCPU of the reference server, a Hetzner CCX23, Linux, one EU location | PRE | GL 225–230 (from R6-HOST-REC2, r6 645–650); GL 244–250 | settled (GLOSSARY); research, source to check (R6's proposal) |
| TE2 | Bench process pinned to the vCPU, VM otherwise idle | PRE | PM4 PRE (proposed there: a neighbour moves a p99) | settled (PM4) |
| TE3 | ML and RD are R3-BUDGET-REC1's machines, named by PM-0003-1 | PRE | r3 R3-BUDGET-REC1, 1096–1103; PM3 PM-0003-1 | research, source to check (R3's proposal); settled |
| TE4 | ML's Wasm marks in Chrome and Firefox on Windows 11; mains power and power mode recorded | PRE | PM4 PRE (R3-BUDGET-REC1's two browsers there) | settled (PM4); proposed (test-engineer, as PM3 and PM4) |
| TE5 | MAC with Safari 26; real Safari through its WebDriver | PRE | r5 R5-TEST-REC1 Safari row, 1394 | research, source to check |
| TE6 | MP: Mustafa's PC in Brave, only what he judges, never a budget result | PRE | brief (hardware rule); plan rule 12; PM3 PRE | settled |
| TE7 | CI: counts, verdicts, hashes and byte comparisons, never a time | PRE | r5 R5-TEST-REC1 benchmark row, 1400 | research, source to check |
| TE8 | "Wasm" = the crate in Node and in a dedicated Worker in Chromium, Firefox and WebKit under Playwright | PRE | r5 R5-TEST-REC1 table, 1389–1393 (Rust in Wasm in Node; Playwright's three engines) | proposed (test-engineer: the client runs the crate in a Worker; Node alone does not show an engine difference) |
| TE9 | `JSON.parse` also in real Safari | PRE | r5 1394 | proposed (test-engineer: each engine parses JSON with its own code; the editor runs in all four browsers, VISION_V2 §4) |
| TE10 | SS: N = 48 in three teams of 16; 15 big vehicles, 4 very large units; 300 live projectiles; up to 8 active effects per soldier (384) | PRE | GL 397–418; MODES-3 (a) | settled (GLOSSARY, STATUS) |
| TE11 | 48 × 8 = 384 | PRE | GL 404 | arithmetic |
| TE12 | SS driven from a recording, counts topped up every tick | PRE | PM4 PRE, PM-0004-2 | settled (PM4) |
| TE13 | LS and BS are ADR 0003's scenes | PRE | PM3 PRE | settled (PM3) |
| TE14 | The test content list (12 items) | PRE | r1 R1-ABL-REC1 mark 1, 2365–2368 | research verified |
| TE15 | Test-only definitions are fixtures, never game content | PRE | draft AB item 6 ("test-only tracks that are not game content"); the advisor's condition 11 (no design) | settled |
| TE16 | T: 3 runs, 900 warm-up then 36,000 measured ticks at 30 Hz; every run meets the mark | PRE | PM4 PRE (from r1 R1-TICK-REC2 P1, 2044–2053) | settled (PM4); research verified |
| TE17 | V: 3 run medians within 3 % | PRE | r3 R3-MEASURE-REC1, 1192–1193; PM3, PM4 | research, source to check; proposed (test-engineer, as PM3/PM4: applied to every timed set) |
| TE18 | Nearest-rank percentiles; p99 of 36,000 = 35,640th | PRE | PM4 PRE; ⌈0.99 × 36,000⌉ = 35,640 | arithmetic; settled (PM4) |
| TE19 | A time mark with no statistic is a p99; p99s never added; a one-off deadline is a plain limit | PRE | GL 141–147 | settled (GLOSSARY) |
| TE20 | A per-event deadline's statistic is the max over the events | PRE | GL 146–147 applied per event | proposed (test-engineer: every event must meet a plain limit) |
| TE21 | Tick bench of ADR 0002 part 8; R1-XT-OQ15 decides the method | PRE | r1 R1-XT-OQ15, 3097–3100; PM4 PRE | research, source to check; settled |
| TE22 | 5 µs timers with the isolation flag, 100 µs without | PRE | r3 R3-MEASURE-F7, 641–643 | research, source to check |
| TE23 | Development-loop latencies read on one machine's clock | PRE | none | proposed (test-engineer: two clocks add their offset to a 1 s mark) |
| TE24 | Every mark at 30 Hz; none at 60 Hz; the 60 Hz margin is ADR 0001's, measured in one run | PRE, NOM | TICK-30; plan TICK-60-HEADROOM (83–91); map J1 | settled |
| TE25 | Profiles P0–P4 and B1 with their round trip, jitter and loss | PRE | r2 1987–1990 | research verified (R2's profiles) |
| TE26 | Injected by ADR 0002's simulator, L1 and L2 | PRE | r5 R5-TEST-REC4, 1454–1460 | research, source to check |
| TE27 | P2 is VISION_V2 §4's "about 100 ms of ping" | PRE | map J6 (3); OQ-0001-3; r2 1990–1991 | settled (the map's reading, for the Advisor to confirm) |
| TE28 | A refusal count needs an acceptance control; every corpus file carries its expected verdict and changed path | PRE | none | proposed (test-engineer: a check that refuses everything passes "0 accepted invalid") |
| TE29 | Every automated mark has a planted defect; "a check without a failing case can't be shown to work" | PRE | r5 R5-GATE-REC3, 1727–1728 | research, source to check |
| TE30 | Rule A: one planted 64-byte allocation per tick must be counted | PRE | PM4 rule A | settled (PM4); proposed (test-engineer) |
| TE31 | Gate tests: 20 identical verdicts in 20 runs; one fixed `PROPTEST_RNG_SEED`; fresh seeds nightly; failures committed as regression fixtures | PRE | r5 R5-TEST-REC6, 1492–1494, 1503–1504 | research, source to check |
| TE32 | A fixed generator seed for C in the gate | PRE | r5 1492–1494 applied to the corpus | proposed (test-engineer: a red result must reproduce) |
| TE33 | The two fuzz marks share one corpus and one loader run; the next-match change shares one run (Shabzon 8 → 9) | PRE | draft frame "What a spike must prove" 1 (lines 1896–1900: "as the test-engineer decides") | settled (the frame leaves it here); proposed (test-engineer: the choice) |

## The table

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE34 | Machines and targets recorded before each spike's first run; CI engine and toolchain versions with every CI result | PM-1 | r3 R3-BUDGET-REC1 mark, 1114–1116; r6 R6-HOST-REC2, 645–650; PM4 PM-0004-1 | research, source to check; proposed (test-engineer: the CI versions) |
| TE35 | 100 % of doubles parse to the same 64-bit pattern natively, in Wasm and in `JSON.parse` | PM-2 | r1 R1-DATA-REC1 mark 1, 2494–2496 | research verified |
| TE36 | LT holds 2,038 numbers (1,888 in `tables`, 150 in `boot_changes`) | PM-2 | LT at 9e7db28, counted by a Node script walking every value | arithmetic (script count) |
| TE37 | 10,000 generated doubles (the research's count) | PM-2 | r1 2494–2495 | research verified |
| TE38 | Split 5,000 random bit patterns (no NaN, no infinities) and 5,000 decimals of up to 17 significant digits in LT's range; plus −0 and the extreme finite doubles | PM-2 | none (JSON carries no NaN or infinity) | proposed (test-engineer: the first half reaches subnormals and exponents, the second the values content holds) |
| TE39 | Bits written as 16 hex digits and compared by script | PM-2 | none | proposed (test-engineer: method) |
| TE40 | Bite: a parser rounding through `f32` | PM-2 | none | proposed (test-engineer) |
| TE41 | 100 % written back as the same text | PM-3 | r1 R1-DATA-REC1 mark 1, 2495–2496 | research verified |
| TE42 | "Shortest round-trip number text" has no finding; mark (1) tests it | PM-3 | r1 2491–2492 | research verified |
| TE43 | Canonical output byte-identical on native and Wasm, and idempotent | PM-3 | CD-1 ("One canonical writer") | proposed (test-engineer: what one writer on two targets must give) |
| TE44 | The two writers' number formats may differ for integral doubles and exponent forms, which would fail the research's mark as written | PM-3 | none in R1–R6 | proposed (test-engineer: knowledge of serde_json/ryu and ECMAScript number formatting, not researched; the spike's first run settles it) |
| TE45 | If TypeScript writes content, it gives the canonical text or writes through the Wasm writer | PM-3 | CD-1 (one canonical writer) | proposed (test-engineer: a reading of CD-1, recorded, not decided) |
| TE46 | An integer above 2^53 (9,007,199,254,740,993 = 2^53 + 1) refused at load | PM-3 | CD-1 ("Content integers stay within the range a double holds exactly"; draft line 262) | arithmetic (2^53 + 1); proposed (test-engineer: the rule's bite) |
| TE47 | Bite: a writer printing 15 significant digits | PM-3 | none | proposed (test-engineer) |
| TE48 | 0 crashes, 0 accepted invalid files, same verdict and error path on native and Wasm for 100 %; every error names the file and line and column or a field path | PM-4 | r1 R1-DATA-REC2 marks 1–2, 2513–2516 | research verified |
| TE49 | The research's defect classes: unknown keys, `__proto__` keys, wrong types, out-of-range numbers, 10,000-deep nesting, 50 MB files, duplicate ids, dangling references | PM-4 | r1 2513–2515 | research verified |
| TE50 | 10,000 files | PM-4 | r1 2513 | research verified |
| TE51 | 100 % of valid controls accepted; 1,000 controls | PM-4 | none | proposed (test-engineer: TE28's reason) |
| TE52 | The path named equals the path the generator changed in 100 % of data errors | PM-4 | r1 2516 (a field path) | proposed (test-engineer: naming a path is not naming the right one) |
| TE53 | Error record compared as (stage, file, line and column or field path, kind) | PM-4 | r1 2515 ("the same verdict and error path") | proposed (test-engineer: the record's fields) |
| TE54 | 50 MB files refused at the byte-size stage, 10,000-deep at the depth stage | PM-4 | CD-2's stage order (draft lines 270–274; R1-DATA-REC2, r1 2500–2503) | proposed (test-engineer: a refusal must come from the stage meant for it) |
| TE55 | The default depth limit's value recorded | PM-4 | r1 R1-DATA-OQ4, 2799–2800 | research, source to check |
| TE56 | Whether garde builds and runs on wasm32 recorded; CD-2's fallback otherwise | PM-4 | r1 R1-DATA-OQ5, 2801–2803; CD-2 | research, source to check; settled (the draft) |
| TE57 | Seeded generator, regenerated byte for byte | PM-4 | r5 R5-TEST-REC6 (a red run reproduces), 1490–1494 | proposed (test-engineer) |
| TE58 | Seeds of every content kind (imported set, two maps, two modes, test tracks) | PM-4 | draft AB item 3 (track definitions included, proposed there) | proposed (the draft's, for tracks); proposed (test-engineer, the rest) |
| TE59 | At least 500 files per defect class; the 50 MB class 10 files generated in the run | PM-4 | none | proposed (test-engineer: size adds time, not cases) |
| TE60 | A trap in Wasm counts as a crash; the instance is recreated | PM-4 | r1 R1-RR-OQ3, 2936–2938 (wasm32 panics abort) | research, source to check; proposed (test-engineer: method) |
| TE61 | Joins the gate (20 identical verdicts) | PM-4 | draft Test plan ("the fuzz runs on native and Wasm"); r5 1503–1504 | settled (the draft); research, source to check |
| TE62 | Bites: `serde(flatten)` letting an unknown key through; a fallback weapon on a dangling id | PM-4 | CD-2 ("never `flatten` where unknown keys must fail"; "no fallback map, weapon or model") | proposed (test-engineer) |
| TE63 | Every bounded collection: one element over refused 100 % | PM-5 | CD-2 (proposed there; reason R1-RR-OQ2, r1 2933–2935) | proposed (the draft's) |
| TE64 | At exactly the bound accepted; 0 collections without a maximum length | PM-5 | CD-2 ("every collection carries a maximum length") | proposed (test-engineer: control; the rule checked mechanically) |
| TE65 | Bounds read from the schema's `maxItems` and `maxProperties` | PM-5 | CD-3 (schema from the Rust types) | proposed (test-engineer: method) |
| TE66 | Planted `.unwrap()` and `buf[i]` fail clippy; the same in a test passes | PM-6 | r5 R5-RUSTLINT-REC3 mark, 1351–1352 | research, source to check |
| TE67 | cargo-fuzz on the loader: 0 panics, 0 over-limit allocations; nightly, Linux | PM-6 | r5 R5-TEST-REC1 fuzz row, 1399; R1-RR-OQ3, r1 2936–2940 | research, source to check |
| TE68 | 10 million inputs | PM-6 | r2 R2-TRN-REC4 mark 4, 2696 (per message type); PM4 PM-0004-60 | research verified (the count's source); proposed (test-engineer: applied to the loader) |
| TE69 | Allocation limit = the largest reviewed bound × its element size; the fuzzer's option not researched | PM-6 | none | proposed (test-engineer) |
| TE70 | Schema carries the same minimum and maximum for 100 % of ranged fields; a value just outside refused by Rust and flagged by the schema field | PM-7 | r1 R1-DATA-REC3 mark, 2527–2529 | research verified |
| TE71 | The same inclusive or exclusive form; the value at the bound accepted by both | PM-7 | none | proposed (test-engineer: an off-by-one shows only at the bound) |
| TE72 | "Just outside" = bound ± 1 for integers, next representable double for floats | PM-7 | none | proposed (test-engineer) |
| TE73 | "Flagged" tested in Vitest on the field's limit logic | PM-7 | CD-3 (the editor's fields built from the schema); r5 R5-TEST-REC1 TypeScript unit row | proposed (test-engineer: the inspector comes later) |
| TE74 | Bite: a deliberately changed type | PM-7 | r1 2529 | research verified |
| TE75 | Zero diff on regeneration; `tsc --strict` passes | PM-8 | r1 R1-BND-REC4 marks 1, 3, 2304–2306 | research verified |
| TE76 | Rename without regeneration fails; hand edit fails | PM-8 | r5 R5-LAYOUT-REC3 mark, 1203–1204 | research, source to check |
| TE77 | Shares PM-0002-15 (the gate's G4) | PM-8 | PM2 PM-0002-15; r5 R5-GATE-REC1 G4 (1662) | settled (PM2) |
| TE78 | Legacy import files typed against the generated types pass the Rust validator | PM-8 | CD-3 (proposed there; reason F16, R1-BND-F9, r1 941–946) | proposed (the draft's) |
| TE79 | Cabana, DM_Tri and Mustafa's maps migrate and validate 100 %; byte-identical round trip; future version refused | PM-9 | r1 R1-DATA-REC4 mark, 2539–2540 | research verified |
| TE80 | Property test of random valid documents, 10,000 cases | PM-9 | none | proposed (test-engineer: two maps exercise few field shapes) |
| TE81 | "A clear message" = names the file, its version and the newest version known | PM-9 | none | proposed (test-engineer: a reading) |
| TE82 | Migration goldens and insta snapshots of validation errors, never rewritten in CI | PM-9 | r5 R5-TEST-REC3, 1431–1453 (insta for content-validation errors) | research, source to check |
| TE83 | Saved damage change validated within 1 s; next match runs on it; malformed save leaves the match and shows the error within 1 s | PM-10 | r1 R1-DATA-REC5 marks 1, 4, 2559–2563, restated by CD-7 | research verified; proposed (the draft's restatement) |
| TE84 | A feel-kit change shows on the page within 1 s | PM-10 | CD-7 / content item 5 (proposed there: REC5 mark 1's 1 s on the client path) | proposed (the draft's) |
| TE85 | 20 saves of each kind | PM-10 | PM2 PM-0002-31 (20 saves) | proposed (test-engineer: as ADR 0002's row) |
| TE86 | The running match's per-tick hashes after the save equal a no-save control | PM-10 | CD-7 ("nothing in a running match's simulation changes") | proposed (test-engineer: the claim's check) |
| TE87 | ML for the development loop; Chrome, Edge, Firefox | PM-10 | PM2 PM-0002-31 (ML: Chrome, Edge, Firefox) | proposed (test-engineer: the slowest named machine; a 1 s met there holds on faster ones) |
| TE88 | Watcher: notify with a debouncer | PM-10 | CD-7; r1 R1-DATA-REC5, 2542–2544 | research verified |
| TE89 | Bites: validation only at next match start; a mid-match swap | PM-10 | none | proposed (test-engineer) |
| TE90 | Only under OQ-0005-CD4 (b): same tick as the server, 0 extra mispredictions under P2; 50 edits in a 5-minute bot match, 0 crashes, 0 desyncs | PM-11 | r1 R1-DATA-REC5 marks 2, 3, 2559–2562; OQ-0005-CD4 | research verified |
| TE91 | "100 ms injected latency with jitter" read as P2 | PM-11 | r1 2561; map J6 (3) | settled (the map's reading) |
| TE92 | 3 seeds for the edit storm; bite: the client one tick late | PM-11 | none | proposed (test-engineer) |
| TE93 | 3 tampering cases (stale, one changed byte, missing) 100 %; repeat visit 0 bytes; 100 % of records resolve to an archived set | PM-12 | r1 R1-DATA-REC6 mark, 2576–2579 | research verified |
| TE94 | 0 matches started on a mismatched bundle; the archive re-hashes; the record carries the mode id | PM-12 | CD-8 ("Every match record stores the content hash and the mode id") | proposed (test-engineer: the checks behind "refused" and "resolves") |
| TE95 | Rust SHA-256 equals WebCrypto on every spike bundle | PM-12 | r1 R1-DATA-OQ9, 2813–2814 | research, source to check |
| TE96 | `transferSize` 0; 3 repeats from a fresh profile | PM-12 | PM6 PM-0006-4 | settled (PM6) |
| TE97 | WebCrypto needs a secure context; localhost used | PM-12 | r1 R1-DATA-REC6, 2567–2569 (F29) | research verified |
| TE98 | Each tampering case 10 times | PM-12 | none | proposed (test-engineer) |
| TE99 | C through the override path: 0 accepted invalid, 0 crashes; `__proto__` refused; prototypes unchanged | PM-13 | r1 R1-DATA-REC7 mark, 2591–2594 (without map upload, CD-9) | research verified; proposed (the draft's restatement) |
| TE100 | Valid allow-list overrides accepted 100 %; outside the list refused | PM-13 | CD-9 (an allow list) | proposed (test-engineer: control and the list's bite) |
| TE101 | A test allow list; its content is a design question | PM-13 | r1 R1-DATA-OQ10, 2815–2816 | research, source to check |
| TE102 | The override path called as ADR 0001's `HostCommand` carries it | PM-13 | CD-9 | settled (the draft) |
| TE103 | Bites: `Object.assign` merge; the allow-list check removed | PM-13 | CD-9 (no `Object.assign` or spread) | proposed (test-engineer) |
| TE104 | 100 % of non-look, non-state values mapped (v2 path) or dropped (reason); imported set passes REC2 | PM-14 | r1 R1-DATA-REC8 mark, 2605–2606 | research verified |
| TE105 | "Value" read per leaf | PM-14 | none | proposed (test-engineer: a table mapped whole could hide an unmapped key) |
| TE106 | 725 leaves in gameplay (493), simulation (20) and progression (212); 2,011 in look | PM-14 | LT at 9e7db28, counted by a Node script over each table's `values` by its `group` | arithmetic (script count: 493 + 20 + 212 = 725) |
| TE107 | `boot_changes` entries become map data | PM-14 | CD-10; r1 R1-DATA-REC8, 2598–2599 | research verified |
| TE108 | Look group's values listed, each marked for the look proof test | PM-14 | CD-10 / content item 8 (proposed there) | proposed (the draft's) |
| TE109 | Each mapped row keeps its legacy `where` | PM-14 | CD-10; r1 2599–2600 | research verified |
| TE110 | Dead keys CD4, CD6, CD7, CD10, CD12, CD14 dropped with a reason or kept by decision | PM-14 | r1 2599–2600; CD-10 | research verified |
| TE111 | Every kept value exact (bits) | PM-14 | r5 R5-TEST-REC3, 1438 ("exact for every value v2 keeps") | research, source to check |
| TE112 | Bites: a planted key in a copy of LT; a kept value moved by one unit in the last place | PM-14 | r5 R5-TEST-REC3 mark ("each golden fails when the v2 value is moved") | proposed (test-engineer) |
| TE113 | Changing every client-content value leaves the per-tick state hashes identical on native and Wasm | PM-15 | CD-6 ("a presentation value cannot reach an outcome"); AP-1 | proposed (test-engineer: the claim had no mark) |
| TE114 | The scenario of PM-0004-63 (120 s), 3 seeds | PM-15 | PM4 PM-0004-63; r1 R1-ABL-REC3, 2418–2419 | settled (PM4) |
| TE115 | Bite: a feel-kit value read in the sim | PM-15 | none | proposed (test-engineer) |
| TE116 | Load time of the legacy tables plus two maps and two modes natively: reported | PM-16 | r1 R1-DATA-REC1 mark 2, 2496–2498 | research verified |
| TE117 | The same load in Wasm in the Worker; serde_json and garde's size recorded | PM-16 | content item 9 (proposed there; reason R1-RR-OQ5, r1 2946–2950) | proposed (the draft's) |
| TE118 | The budgets set from these numbers (the Advisor) | PM-16 | OQ-0005-CD2; r1 R1-DATA-OQ2, 2791–2794; r5 R5-GATE-REC4 ("Starting values come from the first spike's measurements", 1737–1738) | settled (the draft); research, source to check |
| TE119 | Maps Cabana and DM_Tri; modes 24v24 and 16v16v16 | PM-16 | GL test-mode list, 375–394 | proposed (test-engineer: from the test-mode list) |
| TE120 | Median and max over 20 cold loads | PM-16 | none | proposed (test-engineer) |
| TE121 | Size as module with the crate minus a stub-loader build, under the boundary spike's settings, raw and gzip | PM-16 | r5 R5-GATE-REC1 G7 (Wasm bytes raw and gzip, 1665); R1-RR-OQ5 | research, source to check; proposed (test-engineer: method) |
| TE122 | RS native; ML Chrome and Firefox Wasm | PM-16 | content item 9 ("on the native server"; "in Wasm in the Worker") | proposed (the draft's); settled (machines) |
| TE123 | Mustafa's content trial answers OQ-0005-CD1 | PM-17 | r1 R1-DATA-OQ1, 2788–2790 ("a short trial in the spike"); OQ-0005-CD1 | research, source to check; settled (the draft) |
| TE124 | 0 matches for the test ids in the sim's non-test code; the test content runs | PM-18 | r1 R1-ABL-REC1 mark 1, 2365–2368 | research verified |
| TE125 | Id-rename test: byte-identical event log after mapping ids back | PM-18 | none | proposed (test-engineer: a name search misses code keyed by a constant spelled otherwise) |
| TE126 | The operation list reported for the rust-reviewer | PM-18 | AB-1 ("every new operation is a reviewed Rust change") | proposed (test-engineer) |
| TE127 | Bites: `if id == "molotov"`; a constant `FIREBOMB` | PM-18 | none | proposed (test-engineer) |
| TE128 | Shield block on a vehicle and the Banzai buff on another class behave as their parts, 0 Rust changes | PM-19 | r1 R1-ABL-REC1 mark 2, 2369–2371 | research verified |
| TE129 | 30 % inside the arc (1 − 0.7), 100 % behind; arc cos 0.30; ×1.3 for 10 s | PM-19 | r1 1373; INV CD8, line 89; arithmetic 1 − 0.7 = 0.3 | research, source to check (legacy facts); arithmetic |
| TE130 | 0 Rust changes read from `git diff --name-only` | PM-19 | none | proposed (test-engineer: method) |
| TE131 | Effect removed when its source dies; last-hit or tick trigger moves the item | PM-20 | r1 R1-ABL-REC1 mark 3, 2372–2376 | research verified |
| TE132 | On the tick the death is processed; 1,000 deaths and 1,000 kills; the periodic-tick case; two kills in one tick by the stable order on native and Wasm | PM-20 | r1 R1-ABL-REC3, 2406–2409 (fixed order) | proposed (test-engineer: counts, timing and tie case) |
| TE133 | One number changed in data changes the next match with zero Rust changes | PM-21 | r1 R1-ABL-REC1 mark 4, 2377–2378 | research verified |
| TE134 | Shabzon damage 8, Conscript HP 100 | PM-21 | LT `WEAPONS` (shamble-core.js:108) and `UNIT_TYPES` (shamble-core.js:126) | research, source to check (legacy data) |
| TE135 | 9 × 12 = 108 ≥ 100; 8 × 12 = 96 < 100: the 12th hit instead of the 13th | PM-21, CC 2 | TE134 | arithmetic |
| TE136 | Per-shot results equal the legacy tables: head 2.6, body 1.0, legs 0.65, Bagpipe head 1.2; Shabzon falloff at 10, 15, 25, 35, 50 m; pellets, magazine, reload, burst; integers exactly, floats within 1e-6 relative; one weapon model for bots and humans | PM-22 | r1 R1-ABL-REC2 mark, 2396–2400 | research verified |
| TE137 | Timing compared as stored values (within the loader's reported rounding) and as behaviour (within one tick) | PM-22 | PM4 PM-0004-11 (rounding report); r1 R1-ABL-REC3, 2403–2405 | proposed (test-engineer: on a 30 Hz tick 1e-6 can apply to stored values only) |
| TE138 | One tick at 30 Hz = 33.3 ms | PM-22, PM-25 | 1000 ÷ 30 = 33.3 | arithmetic |
| TE139 | Whether v2 keeps one weapon model is Mustafa's | PM-22 | r1 R1-ABL-OQ6, 2772–2775; AB-3 | research, source to check |
| TE140 | Goldens captured from the legacy oracle with commit, row and inputs | PM-22 | r5 R5-TEST-REC3, 1431–1436 | research, source to check |
| TE141 | Bites: a value moved beyond tolerance; the reset-not-carry timer | PM-22 | r5 R5-TEST-REC3 mark; INV GC5, line 244 | research, source to check |
| TE142 | Legacy fields convert with every number present once; unknown id, tag or out-of-range value fails at load; 10,000 fuzzed definitions never panic the simulation | PM-23 | r1 R1-ABL-REC5 mark, 2460–2463 | research verified |
| TE143 | "Present once" read as mapped once or dropped with a reason | PM-23 | CD-10; INV CD13 line 94 ("Avoid: bot-only fields … a colour (accent) in sim data") | proposed (test-engineer: a reading reconciling REC5 with CD-10) |
| TE144 | Track definitions included; a track naming an unknown effect fails | PM-23 | AB item 3 (proposed there: AB-4 adds the kind) | proposed (the draft's) |
| TE145 | Accepted definitions each run 300 ticks (10 s at 30 Hz) | PM-23 | 10 × 30 = 300 | arithmetic; proposed (test-engineer: "never panic the simulation" is about running them) |
| TE146 | Bites: a pellet count of 0 indexing an array; a dangling ability id | PM-23 | none | proposed (test-engineer) |
| TE147 | Press before Tech 1 and T2 purchase before Tech 2 refused in 1,000 of 1,000 | PM-24 | AB item 4 (proposed there: LOOP_V2 §4, 50–52; INV GE4, 290); count from r1 2444 | proposed (the draft's) |
| TE148 | Accepted after the tier, 1,000 of 1,000 | PM-24 | none | proposed (test-engineer: control) |
| TE149 | Warrior kills Conscript in 0.86 s within one tick | PM-25 | r1 R1-ABL-REC3 mark, 2416–2418 (ADR 0004's, PM-0004-9); INV CD13, 94 | research verified |
| TE150 | 13 SMG body hits; 12 × 0.072 = 0.864 s | PM-25 | LT Shabzon damage 8, interval 0.072; Conscript 100 HP; ⌈100 ÷ 8⌉ = 13 | arithmetic |
| TE151 | Peacekeeper kills Warrior in 0.85 s when 11 of 16 pellets land | PM-25 | AB item 5 (proposed there); INV CD13, 94 | proposed (the draft's); research, source to check |
| TE152 | 11 × 11 = 121 ≥ 120 HP; two shots, 8 pellets each; 10 of 16 (110) leave it alive | PM-25 | LT Bagpipe damage 11, pellets 8, interval 0.85; Warrior HP 120 | arithmetic; proposed (test-engineer: the control) |
| TE153 | 63 SMG body hits behind the Shield; 62 leave 1.2 HP; 19 without | PM-25 | AB-7 arithmetic (8 × 0.3 = 2.4; ⌈150 ÷ 2.4⌉ = 63; 150 − 62 × 2.4 = 1.2; ⌈150 ÷ 8⌉ = 19) | arithmetic; proposed (the draft's golden) |
| TE154 | Knockback changes velocity by the test weapon's value within 1e-6 relative; 0 matches for its id | PM-25 | AB item 5 ("no Javelin-specific code"); 1e-6 from r1 2399 | proposed (the draft's); proposed (test-engineer: the tolerance's reuse) |
| TE155 | Pellet hits scripted since spread draws from an authority-only stream | PM-25 | SEC-SPREAD (a); GL shared stream and authority-only stream | settled |
| TE156 | u1 to u6 as written in the draft | PM-26 to PM-31 | draft AB item 6 (lines 1260–1271; no research mark; UPGRADE-SCOPE) | proposed (the draft's); settled (UPGRADE-SCOPE) |
| TE157 | u1 adds the rename test | PM-26 | TE125 | proposed (test-engineer) |
| TE158 | u2: 1,000 cycles; effect lists compared field by field; shares PM-0004-64 | PM-27 | draft u2 (1,000: R1-ABL-REC4's count, r1 2444); PM4 PM-0004-64 | proposed (the draft's); settled (PM4) |
| TE159 | u3: 1,000 cycles | PM-28 | none | proposed (test-engineer: as u2) |
| TE160 | u4: 1,000 exchanges; ADR 0004's pickups (PM-0004-66) | PM-29 | PM4 PM-0004-66 | proposed (test-engineer: count); settled |
| TE161 | u5: 100 consecutive matches; shares PM-0004-65 | PM-30 | PM4 PM-0004-65; LOOP_V2 §8 | settled (PM4); design doc |
| TE162 | u6: offers of the data's size (3), distinct, requirements met; 1,000 forged refused; 1,000 offered accepted; byte-identical logs on two runs; native and Wasm only if the client holds the stream | PM-31 | draft u6; UPGRADE-SCOPE ("pick one of three"); AB-8 offer row | proposed (the draft's); proposed (test-engineer: distinctness, requirements and the control) |
| TE163 | Bites for u1–u6 (a branch on a track id; picks on the soldier; one shared track; picks on the gun; a pick effect on a pooled unit; any candidate accepted; a clock seed) | PM-26 to PM-31 | none | proposed (test-engineer) |
| TE164 | Step p99 at 8, 16, 32 active effects per soldier (384, 768, 1,536) reported; the 1 ms bracket | PM-32 | AB-10 (proposed there); 48 × 16 = 768, 48 × 32 = 1,536 | proposed (the draft's); arithmetic |
| TE165 | Points 4 effects per soldier apart added inside the bracket | PM-32 | none | proposed (test-engineer) |
| TE166 | The extra effects are permanent attribute modifiers on unit and weapon | PM-32 | AB-8 ("a pick is an effect or a granted ability") | proposed (test-engineer: the curve must measure what picks add) |
| TE167 | The 8-per-soldier point is PM-0004-62 (1 ms p99, 0 allocations) | PM-32, NOM | PM4 PM-0004-62; r1 R1-ABL-REC6, 2465–2472 | settled (PM4); research verified |
| TE168 | Every test model named and listed; dangling id and unknown slot class fail at load with file and field | PM-33 | AP section's spike part (proposed there: R1-DATA-REC2's error mark, r1 2515–2516) | proposed (the draft's) |
| TE169 | 0 stand-in models drawn | PM-33 | AP-2 ("nothing falls back to a default model") | proposed (test-engineer) |
| TE170 | Models load on both backends with 0 console errors, drawn with the engine's materials, no own material left | PM-34 | GPU spike part (draft 1650–1654; R3-WGPU-REC1 mark 1, r3 910–912) | proposed (the draft's); research, source to check |
| TE171 | Material identity checked against the cache | PM-34 | GPU part 7 (shared, cached) | proposed (test-engineer: method) |
| TE172 | 0 model requests other than listed glTF/GLB; 0 requests outside the client's host | PM-34 | GPU part 6 (only glTF/GLB); r5 R5-GATE-REC3 G12 mutation (outbound request), 1723 | proposed (test-engineer); research, source to check |
| TE173 | PM-0003-2's 10 starts per browser and backend | PM-34, PM-42 | PM3 PM-0003-2, -5 | settled (PM3) |
| TE174 | Material objects at most the distinct (slot class, faction) pairs | PM-35 | GPU spike part (proposed there; R3-LOOK-F3, r3 331–334) | proposed (the draft's) |
| TE175 | Counted by identity on listed assets' meshes; every model in each faction | PM-35 | none | proposed (test-engineer: method) |
| TE176 | Bite: per-mesh material clone (CL4) | PM-35 | INV CL4, line 113 | research, source to check |
| TE177 | Soldier at most 4 per pass | PM-36 | GPU part 4 (R3-CHAR-REC1, r3 1044–1045) | research, source to check; arithmetic |
| TE178 | Very large unit at most 25 per pass: 200 ÷ 4 ÷ 2 = 25 | PM-36 | GPU part 4 (R3-CHAR-REC4, r3 1088–1090) | arithmetic |
| TE179 | Method (shown − hidden) ÷ passes, PM-0003-25's | PM-36 | PM3 PM-0003-25 | settled (PM3) |
| TE180 | Bites: 5 slots → 5 × 2 = 10 > 8; 30 parts → 30 × 2 = 60 > 50 | PM-36 | 4 × 2 = 8; 25 × 2 = 50 | arithmetic; proposed (test-engineer) |
| TE181 | Mustafa judges each family in the look proof test, beside legacy screenshots | PM-37, FC | OQ-0005-AP2; LOOK_V2 §9 (76–78); plan rule 12 | design doc; settled |
| TE182 | The tool processes the test models and writes their entries | PM-38 | AP section's spike part; OQ-0005-AP1 | proposed (the draft's) |
| TE183 | Entry counts equal an independent recount | PM-38 | OQ-0005-AP1 (recount or trust) | proposed (test-engineer) |
| TE184 | Hash in name, immutable, repeat visit 0 bytes | PM-39 | AP spike part (proposed there: r1 2577–2578; r6 641–642) | proposed (the draft's) |
| TE185 | Under 25 MiB = 26,214,400 bytes | PM-39 | r6 R6-HOST-REC1 mark, 642–643; 25 × 1,048,576 | research, source to check; arithmetic |
| TE186 | PM-0006-4 and -5 methods | PM-39 | PM6 | settled (PM6) |
| TE187 | Over-budget asset fails; raise without the file fails | PM-40 | r5 R5-GATE-REC4 mark, 1743–1744 (applied to assets in the AP spike part) | research, source to check; proposed (the draft's) |
| TE188 | Controls: an asset at its budget; a raise written with its reason | PM-40 | r5 R5-GATE-REC3 (a control per check), 1707–1709 | research, source to check; proposed (test-engineer) |
| TE189 | Planted glTF reader in `sim` fails | PM-41 | AP spike part (proposed there: AP-1; r5 1189–1192) | proposed (the draft's) |
| TE190 | The same in `server`; the tool's package imported from `client` fails the manifest walk; control: the tool depending on a glTF reader | PM-41 | AP-1 ("`sim` or `server` dependency trees"); AP-6 ("never in the player build"); r5 R5-LAYOUT-REC2 (manifest walk), 1181–1182 | proposed (test-engineer) |
| TE191 | The draft's migration places the dependency check in the foundation | PM-41 | draft Migration plan item 2 (lines 1925–1927) | settled (the draft's text) |
| TE192 | Every slot classed, every faction slot coloured, loader on both backends | PM-42 | GPU spike part (draft 1655–1658) | proposed (the draft's) |
| TE193 | Colours read back equal the faction data | PM-42 | CD-13; GPU part 2 | proposed (test-engineer: method) |
| TE194 | The fight on screen from real models within ADR 0003's frame totals: PM-0003-18, -21, -22 | PM-43 | GPU spike part; r3 R3-BUDGET-REC2, 1121–1137 | proposed (the draft's); research, source to check |
| TE195 | LOD only where the framing spreads distances; switching with hysteresis | PM-44 | GPU part 5 (R3-CHAR-REC2, r3 1050–1063; R3-CHAR-F10, 490–493) | research, source to check |
| TE196 | 0 switches in a 600-frame dwell inside a hysteresis band | PM-44 | none | proposed (test-engineer: flicker shows as switches) |
| TE197 | Mustafa judges the silhouette | PM-44, LC | GPU spike part ("judged by Mustafa"); OQ-0005-AS2 | proposed (the draft's) |
| TE198 | Feel Lab: predicted against server-confirmed Molotov and Shield; Mustafa judges | PM-45, PM-46 | AB item 8; ABL-PREDICT (r1 2967–2980; STATUS "Waiting"); R2-SYNC-OQ10 (r2 2936–2939) | settled (STATUS); research verified |
| TE199 | Reported per ability and variant at P2: response time median and p95 over 200 presses; confirmed share; rollbacks per 100; largest correction | PM-45 | plan rule 12 (the netcode proof test's measures, applied to the feel check) | proposed (test-engineer: counts and statistics) |
| TE200 | R1-ABL-REC4's marks (same tick; undo within one round trip plus one tick; one cue over 1,000 presses; no damage before the server) are ADR 0001's | PM-45, NOM | r1 R1-ABL-REC4 mark, 2441–2445; map part b (owner 0001) | research verified; settled |
| TE201 | 3 runs; L2; server on RS | PM-45 | r5 R5-TEST-REC4 (L2 for real browsers), 1457–1459 | proposed (test-engineer) |
| TE202 | Mustafa's answer to ABL-PREDICT recorded | PM-46 | STATUS ABL-PREDICT ("Mustafa, after a Feel Lab test") | settled |

## The failure rule and the "no mark" lines

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE203 | Conditional, reported and checklist marks pass when their condition holds or their record is complete | FAIL | PM3, PM4 failure rules | settled (PM3, PM4) |
| TE204 | A failing run is never re-run until green; its data kept and cause written down | FAIL | r5 R5-TEST-REC6, 1490–1498 | research, source to check |
| TE205 | A failed mark goes to the author of the part; the draft's Rollback routes (option D; checked newtypes; OQ-0005-CD4 (b); a new operation or A4; track operations; asset-list fields; the procedural kit) | FAIL | draft Rollback (lines 1965–1980); draft header (authors) | settled (the draft's text) |
| TE206 | Other drafts' marks go back to their own drafts | FAIL | PM4 failure rule | settled (PM4) |
| TE207 | A mark's number changes only in the draft; raising a budget is a reviewed change | FAIL | r5 R5-GATE-REC4, 1734–1738 | research, source to check |
| TE208 | CD-5 holds by review; duplicate ids are a defect class of C | NOM | CD-5; r1 2514 | settled (the draft); research verified |
| TE209 | Time conversion is PM-0004-11; top speed has only load checks until OQ-0005-CD8 | NOM | PM4 PM-0004-11; OQ-0005-CD8 | settled |
| TE210 | EDITOR_V2 §4's 1,000 commands are slice 5's "Done when" | NOM | draft Migration item 8 (lines 1941–1943); EDITOR_V2 §4 | settled (the draft); design doc |
| TE211 | CD-12 no mark until OQ-0005-CD6; CD-13 none until OQ-0005-CD5 | NOM | draft open questions table | settled (the draft) |
| TE212 | Quality-tier rows are PM-0003-23 and -33 | NOM | PM3 | settled (PM3) |
| TE213 | Content item 10 is R2-ROOM-REC7's mark, ADR 0001's, slices 2 and 3 | NOM, SE | r2 R2-ROOM-REC7 mark, 2864–2867; PM 0001's "no mark" line (test-engineer-passmarks-0001.md line 196) | research verified; settled |
| TE214 | R1-DATA-OQ8 is PM-0002-31 | NOM, SE | PM2 PM-0002-31; r1 2811–2812 | settled (PM2) |
| TE215 | AB-8's messages and telemetry are ADR 0001's and 0006's; its open rules have no mark | NOM | map J5; OQ-0005-AB1 to AB3 | settled |
| TE216 | Own weapon cues once on replay: ADR 0001's in the netcode proof test | NOM, SE | r1 R1-ABL-OQ3, 2763–2765; draft Appendix A row R1-ABL-OQ3 | research, source to check; settled |
| TE217 | Replay of every test ability on native and Wasm is PM-0004-63 | NOM | PM4 PM-0004-63 | settled (PM4) |
| TE218 | The asset-pipeline research's marks are set by that research | NOM, SE | ASSET-RESEARCH (a); GPU spike part ("its marks come from that research") | settled |
| TE219 | GPU part 7's warm-up is PM-0003-7, -8 | NOM | PM3 | settled (PM3) |

## The checklists

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE220 | Content trial steps 1–6 (8 → 9 within about a second; next match 12 hits; a word for a number names the field; a broken brace names line and column; a feel-kit change live; his OQ-0005-CD1 answer) | CC | CD-2, CD-7; PM-0005-4, -10, -21; OQ-0005-CD1 | settled (the draft's claims); proposed (test-engineer: the steps) |
| TE221 | The development server started on his PC with the session's steps | CC | none | proposed (test-engineer) |
| TE222 | A family can be picked only where both forms are shown | FC | OQ-0005-AP2; AP-7 (both forms side by side) | proposed (test-engineer: a reading) |
| TE223 | Building: the half-timbered house with the brick-tile roof | FC 1 | LOOK_V2 §2, §3 | design doc |
| TE224 | Props (barrels, carts, hay bales, logs) | FC 2 | LT `BK.PROPS` in `boot_changes` (barrels, cart, haybales, logpile …); LOOK_V2 §2 (line 17: "logs and trees with variety") | design doc; research, source to check (legacy data) |
| TE225 | Trees not judged: LOOK_V2 §8 `[OPEN]` | FC 4 | LOOK_V2 §8 (line 74) | design doc |
| TE226 | No missing piece or grey stand-in (missing models fail at load) | FC 5 | AP-2; PM-0005-33 | settled (the draft) |
| TE227 | LOD: readable from a distance, depth up close; no pop or flicker | LC | LOOK_V2 §5; GPU part 5 | design doc |
| TE228 | Feel Lab: P2 injected; his own round trip recorded and added | FL | r2 1987–1990 | research verified; proposed (test-engineer) |
| TE229 | Variants shown as A and B, shuffled, labels hidden until he answers | FL | none | proposed (test-engineer: the label must not decide the feel) |
| TE230 | Ten throws and ten raises per variant; presses before Tech 1 refused, a predicted start may undo | FL 1–3 | R1-ABL-REC4 (a rejected press is undone, r1 2442–2444); LOOP_V2 §4 | research verified; proposed (test-engineer: counts) |
| TE231 | His ABL-PREDICT answer, and whether it differs per ability | FL 4 | STATUS ABL-PREDICT | settled |

## The spike entries

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE232 | Spike names and their owners | SE | map part g, rows 944, 945, 948; later milestones (lines 955–960) | settled (the map's assignment) |
| TE233 | The content and ability spike after the foundation and before slice 1 | SE | map part g order (lines 962–965) | settled (the map's proposal) |
| TE234 | It depends on the sim core spike's throwaway sim, LT and LOOP_V2 §5's T1 abilities | SE | map part g row 944 | settled |
| TE235 | Its subsumed ids: R1-DATA-REC1–REC8; R1-ABL-REC1, REC2, REC5; REC6 and REC3's replay half via ADR 0004; R5-TEST-REC3 | SE | map part g row 944; draft Appendix A; PM4 spike entry | settled |
| TE236 | Decides R1-DATA-OQ2, OQ4, OQ5, OQ9; R1-RR-OQ2, OQ3 (loader half) | SE | map part g row 944; draft Appendix A | settled |
| TE237 | R1-ABL-OQ7 and R1-XU-OQ7 listed here by the map; ADR 0004's PM-0004-9 and -11 run their marks | SE | map row 944; PM4 PM-0004-9, -11 | settled |
| TE238 | Sizes: content and ability spike L; look proof test share S (M with ported kit builders); asset spike M; Feel Lab test M | SE | none | proposed (test-engineer: the work listed in each entry) |
| TE239 | ADR 0003's spike entry lists only authored test assets | SE | PM3 spike entries (look proof test dependencies) | settled (PM3) |
| TE240 | The look proof test runs in parallel with the netcode proof test, before the content and ability spike | SE | map part g order (lines 962–965) | settled (the map's proposal) |
| TE241 | The asset spike before the first authored model enters the game; none of R1–R6 | SE | map part g row 948; part h; ASSET-RESEARCH (a) | settled |
| TE242 | The Feel Lab test needs abilities in the engine (slice 4) | SE | map J6 (8); LE-N; draft Migration item 7 | settled |

## Counts
- Rows: 242 (TE1–TE242), counted by script on this file.
- By first tier (script on this file): research verified 36 · research, source to check 36 · arithmetic 13 · settled 56 ·
  design doc 5 · proposed (the draft's, or another draft's) 24 · proposed (test-engineer) 72.
- Line citations re-read against the files on 2026-11-01 (r1, r2, r5, r6, GL, INV, LOOK_V2, LT, the draft, the other
  pass-mark files); eight corrected in this ledger and two in the subsection before this count.
