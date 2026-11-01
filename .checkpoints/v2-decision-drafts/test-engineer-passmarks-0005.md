# Pass marks for ADR 0005 (content data, abilities and assets), test-engineer, 2026-11-01

Plan step 4. Two parts: the `### Pass marks` subsection, which the session swaps in verbatim for the draft's
placeholder (`### Pass marks` and its one line, `engine-architect-adr0005-draft.md` lines 1915–1916), and the spike
entries for `docs/SPIKE_PLAN.md`, which stay in this notes file for the synthesis (plan step 6). The claims ledger for
every row is `test-engineer-passmarks-0005-claims.md`. The measurement rules follow the pass marks of ADR 0003 and ADR
0004 (`test-engineer-passmarks-0003.md`, `-0004.md`: nearest-rank percentiles, the 3 % validity check, a planted defect
per mark, timer resolution, every run meeting the mark), so the tables read alike.

---8<--- the subsection starts on the next line ---

### Pass marks
Written by test-engineer (plan step 4). One row per mark, grouped by the spike that runs it: one precondition
(PM-0005-1); the content and ability spike, content data (PM-0005-2 to PM-0005-17, Mustafa's content trial inside
PM-0005-17) and the ability and effect system (PM-0005-18 to PM-0005-32); ADR 0003's look proof test, this record's
asset marks (PM-0005-33 to PM-0005-37); the asset spike (PM-0005-38 to PM-0005-44); the Feel Lab test for
ABL-PREDICT (PM-0005-45, PM-0005-46). The second column names the decision part and, where the draft numbers them, the
spike item ("content item n", "ability item n", u1–u6). Every number is a research mark (cited), arithmetic on research
numbers (shown), a settled input, or marked proposed with its reason. Line numbers are `r1-simulation-core.md`'s at
9e7db28 unless another file is named (r2 `r2-netcode.md`, r3 `r3-rendering.md`, r5 `r5-toolchain-quality.md`, r6
`r6-infrastructure.md`, GL `GLOSSARY.md`, INV `docs/LEGACY_INVENTORY.md`).

**How to read the table**
- **Machines** (as ADR 0003's and ADR 0004's pass marks name them). **RS**,
  [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of
  [the reference server](../research/GLOSSARY.md#the-reference-server) (a Hetzner CCX23, Linux, one EU location; GL
  lines 225–230), the bench process pinned to it, the VM otherwise idle. **ML**, the minimum laptop, and **RD**, the
  recommended desktop (R3-BUDGET-REC1, r3 lines 1096–1103; named by PM-0003-1); ML's Wasm marks run in Chrome and
  Firefox on Windows 11, with mains power and the power mode recorded. **MAC**, a macOS machine with Safari 26, for
  the marks that name Safari (real Safari through its WebDriver: R5-TEST-REC1's Safari row, r5 line 1395). **MP**,
  Mustafa's PC (Windows) in Brave: only what he judges, never a budget result. **CI**, the gate v2's runners: counts,
  verdicts, hashes and byte comparisons, never a time (R5-TEST-REC1's benchmark row, r5 line 1400); a browser engine in
  CI means Playwright's Chromium, Firefox or WebKit, headless.
- **Wasm targets.** "Wasm" without more means both: the content crate compiled to `wasm32` and run in Node (CI), and
  the same module in a dedicated Worker in Chromium, Firefox and WebKit under Playwright (CI). A mark that names
  `JSON.parse` runs it in Node, in those three engines and in real Safari on MAC (proposed: each engine parses JSON
  with its own code, and the editor runs in all four browsers, VISION_V2 §4).
- **Scenes.** **SS**, [the stress scene](../research/GLOSSARY.md#the-stress-scene) (GL lines 397–418): N = 48 bots in
  three teams of 16 (MODES-3 (a)), 15 [big vehicles](../research/GLOSSARY.md#big-vehicle-and-very-large-unit) of which
  4 are very large units, 300 [live projectiles](../research/GLOSSARY.md#live-projectile), up to 8
  [active effects](../research/GLOSSARY.md#active-effect-and-visual-effect) per soldier (384), driven from a recording,
  with the counts topped up every tick (ADR 0004's reading, PM-0004-2). **LS**, ADR 0003's look-test scene at the game
  camera; **BS**, ADR 0003's benchmark scene (PM-0003-18's setup). **The test content**: the legacy weapons and
  abilities R1-ABL-REC1 mark 1 lists (AMK, Bagpipe, Shabzon, Retro Lancer, Beam Katana, Torque Bow, frag grenade,
  Molotov, Riot Shield, Banzai, Bayonet, the Juggernaut cannon; lines 2365–2368), imported from
  `docs/legacy-tuning.json` by the import of PM-0005-14, plus test-only definitions (the combination of PM-0005-19, the
  knockback weapon of PM-0005-25 and the test tracks of PM-0005-26 to PM-0005-31), which are fixtures, never game
  content. **C**, the labelled fuzz corpus of PM-0005-4, shared by PM-0005-13 and PM-0005-23.
- **T, a timed run set** (ADR 0004's, from R1-TICK-REC2 P1, lines 2044–2053): 3 runs of one recording, each 900
  warm-up ticks then 36,000 measured ticks at 30 Hz, every run labelled with its commit, machine, compiler and build
  flags; every one of the 3 runs must meet the mark. **V**: the 3 run medians lie within 3 % of each other
  (R3-MEASURE-REC1's mark, r3 lines 1192–1193, applied to every timed set: proposed, as in ADR 0003 and ADR 0004); a set
  that fails V is neither a pass nor a fail.
- **Statistics.** Percentiles by nearest rank (⌈q × n⌉; of 36,000 ticks, p99 is the 35,640th smallest value). A time
  mark that names no statistic is a p99 ([time statistics](../research/GLOSSARY.md#time-statistics), GL lines 141–147);
  p99s are never added. A per-event deadline (a save validated within 1 s) is a plain limit that every event meets (GL
  lines 146–147), so its statistic is the max over the events. Count, verdict, hash and byte marks are exact.
- **Timers.** Tick times as ADR 0004 measures them (its tick bench, ADR 0002 part 8, R1-XT-OQ15, lines 3097–3100). A
  Wasm mark under 1 ms runs on a build with
  [the isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag) on (5 µs timers instead of
  100 µs: R3-MEASURE-F7, r3 lines 641–643). Wall-clock latencies of the development loop (PM-0005-10) are read on one
  machine's clock: the file's modification time against the log line's time stamp (proposed: two clocks would add
  their offset to a 1 s mark).
- **Tick.** Every mark runs at 30 Hz (TICK-30). No mark here is set at 60 Hz; AB-10's 60 Hz arithmetic is a note, and
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) is ADR 0001's, measured in one run.
- **Network.** Only PM-0005-11 and the Feel Lab test run under a network: R2's profiles (round trip, jitter, loss: P0
  0 ms; P1 50 ms, ±10 ms, 1 %; P2 100 ms, ±20 ms, 2 %; P3 150 ms, ±30 ms, 5 %; P4 200 ms, ±40 ms, 5 %; B1 = P2 plus one
  1 s burst of 100 % loss: r2 lines 1987–1990), injected by ADR 0002's network simulator (R5-TEST-REC4, r5 lines
  1454–1460: L1 in one process, L2 under real browsers). P2 is VISION_V2 §4's "about 100 ms of ping" (decision map
  J6 (3); OQ-0001-3). Every other mark runs in one process or on localhost (P0).
- **Labels and controls.** A mark that counts refusals also counts acceptances of valid input (a control), because a
  check that refuses everything passes "0 accepted invalid" (proposed). Every file of C carries the verdict its
  generator expects (valid, or invalid with its defect class and the path it changed), so "accepted invalid" and
  "error names the right field" can be counted by script.
- **Bites.** Each automated mark names a planted defect that must make it fail on the spike's own build before its
  pass counts, because "a check without a failing case can't be shown to work" (R5-GATE-REC3, r5 lines 1727–1728); the
  planted defects are proposed unless cited. **Rule A**, for an allocation mark: one planted 64-byte heap allocation
  per tick in the path under test must be counted (ADR 0004's rule A).
- **Tests that join the gate** give 20 identical verdicts in 20 runs on one commit (R5-TEST-REC6, r5 lines 1503–1504),
  with one fixed `PROPTEST_RNG_SEED` and a fixed generator seed for C in the gate, fresh seeds nightly (r5 lines
  1492–1494); a failing case found nightly is committed as a regression fixture with its fix. A failing run is a
  defect to find, never retried into green.
- **Shared runs** (the frame leaves it to the test-engineer): the two fuzz marks share one corpus and one loader run
  (content item 2 and ability item 3: PM-0005-4 and PM-0005-23, the latter adding a sim run of the accepted
  definitions); the next-match change shares one run (content item 5 and ability item 1's last clause: PM-0005-10 and
  PM-0005-21, the saved change being the Shabzon's damage 8 → 9).

| mark | what it proves (decision part) | number and statistic | hardware | method and tool | runs and noise | basis |
|---|---|---|---|---|---|---|
| **Precondition** | | | | | | |
| PM-0005-1 | The machines and targets exist, and every result names them (a precondition of every spike here) | RS, ML, RD and MAC recorded as PM-0003-1 and PM-0004-1 name them; the CI engine versions (Node, Chromium, Firefox, WebKit) and the Rust toolchain recorded with every CI result | RS, ML, RD, MAC, CI | a results-log header per machine and per CI image, written before the first run; every later row names its machine, browser or engine, and commit | checked before each spike's first run | R3-BUDGET-REC1's mark (r3 lines 1114–1116); R6-HOST-REC2 (r6 lines 645–650); ADR 0004's PM-0004-1 |
| **The content and ability spike: content data** | | | | | | |
| PM-0005-2 | Every side reads the same numbers (CD-1; content item 1) | 100 % of doubles parse to the same 64-bit pattern in native Rust, in Rust on Wasm and in `JSON.parse` | CI (native, Wasm, `JSON.parse` in Node and the three engines); MAC: Safari 26 (`JSON.parse`) | the corpus: every number in `docs/legacy-tuning.json` (2,038, counted by script at 9e7db28) plus 10,000 generated doubles, 5,000 from uniform random 64-bit patterns with NaN and the infinities left out (JSON cannot carry them) and 5,000 decimals of up to 17 significant digits inside the legacy file's range (proposed: the first half reaches subnormals and extreme exponents, the second the values content holds), plus −0 and the largest and smallest finite doubles; each side writes each value's bits as 16 hex digits; a script compares | 1 run per target (exact). Bites: a parser that rounds through `f32` on one side | R1-DATA-REC1 mark 1 (lines 2494–2496); CD-1 |
| PM-0005-3 | One canonical writer, the same text everywhere (CD-1; content item 1) | 100 % of the PM-0005-2 corpus written back as the same text as its input (the research's mark); the canonical writer's output byte-identical on native and Wasm, and idempotent (write, parse, write gives the same bytes and the same bits); an integer above 2^53 (9,007,199,254,740,993) in a content file refused at load with the file and field named | CI | the writer of CD-1 run over the corpus on every target; byte comparison by script. Recorded: every number whose canonical text differs from `JSON.stringify`'s text for the same double (not researched: the two writers' number formats may differ for integral doubles and exponent forms, which would fail the research's mark as written; if any TypeScript code writes content, it must give the canonical text or write through the Wasm writer) | 1 run per target. Bites: a writer that prints 15 significant digits; a planted 2^53 + 1 in a weapon file | R1-DATA-REC1 mark 1 (lines 2494–2496) and "no finding" (lines 2491–2492); CD-1 ("Content integers stay within the range a double holds exactly") |
| PM-0005-4 | One crate gives one verdict everywhere (CD-2; content item 2; shared with ability item 3) | over C: 0 crashes (no panic natively, no trap in Wasm), 0 accepted invalid files, 100 % of valid control files accepted, and for 100 % of files the same verdict and the same error record (stage, file, line and column or field path, error kind) natively and in Wasm; every error names the file and either line and column (syntax) or a field path (data), and the path named is the path the generator changed in 100 % of data errors; the 50 MB files refused at the byte-size stage and the 10,000-deep files at the depth stage. Recorded: the default depth limit's value (R1-DATA-OQ4) and whether garde builds and runs on wasm32 (R1-DATA-OQ5; if not, CD-2's fallback is built and this row runs on it) | CI | C: 10,000 files from a seeded generator (regenerated byte for byte from its seed: proposed, so a failure reproduces) mutating valid seeds of every content kind (the imported set of PM-0005-14, the two maps of PM-0005-9, two modes, test track definitions); every defect class of the research at least 500 files (proposed), except the 50 MB class, 10 files generated in the run and not committed (proposed: their size adds time, not cases); 1,000 unmutated or validly mutated controls (proposed); a trap in Wasm counts as a crash and the instance is recreated | 1 run per target; joins the gate (20 identical verdicts). Bites: a planted `serde(flatten)` that lets an unknown key through; a planted fallback weapon on a dangling id | R1-DATA-REC2 marks 1–2 (lines 2513–2516); R1-DATA-OQ4 (lines 2799–2800), OQ5 (lines 2801–2803); CD-2 |
| PM-0005-5 | No file sizes an allocation past a reviewed bound (CD-2; content item 2) | for every bounded collection, a file one element over its bound refused (100 %) and a file at exactly its bound accepted (100 %, a control: proposed); 0 collections in the content types without a maximum length | CI | the bound of every collection read from the generated schema (`maxItems`, `maxProperties`) by script; a file per bound added to C; the schema scan lists any collection with no bound | 1 run. Bites: a `Vec` field added without a bound | R1-RR-OQ2 (lines 2933–2935); CD-2 (proposed there: "one element over its bound") |
| PM-0005-6 | The loader never panics (CD-2; content item 2) | the panic lints at deny in `content`'s loader: a planted `.unwrap()` and a planted `buf[i]` fail clippy, the same in a `#[test]` pass; cargo-fuzz on the loader: 0 panics and 0 allocations above the fuzz run's allocation limit over 10 million inputs (proposed count: ADR 0001's per-message-type decoder fuzz, R2-TRN-REC4 mark 4) | CI; CI nightly, Linux (fuzz) | the gate's G2 step on the planted lines; a cargo-fuzz target that feeds bytes to the loader's entry; the allocation limit set to the largest reviewed bound times its element size (proposed; the fuzzer's option is not researched) | the lint bite 1 of 1; the fuzz nightly, fresh seeds. Bites: as listed; a planted `.unwrap()` on a parsed id is found within the fuzz run | R5-RUSTLINT-REC3's mark (r5 lines 1351–1352); R5-TEST-REC1's fuzz row (r5 line 1399); R1-RR-OQ3 (lines 2936–2940); R2-TRN-REC4 mark 4 (r2 line 2696) |
| PM-0005-7 | The schema and the validator agree on every range (CD-3; content item 3) | for 100 % of fields with a garde range, the generated schema carries the same minimum and maximum (and the same inclusive or exclusive form); for each such field, a value just outside the range is refused by the Rust validator and flagged by the schema-driven field, and the value at the bound is accepted by both (a control: proposed) | CI | a script pairs each garde rule with its schema keyword; "just outside" is the bound ± 1 for integers and the next representable double beyond the bound for floats (proposed: an off-by-one between inclusive and exclusive forms shows only there); "flagged by the schema-driven field" is a Vitest test of the field's limit logic built from the generated schema (the editor's inspector comes later) | 1 run. Bites: a deliberately changed type (the research's mark); a garde range edited without regenerating | R1-DATA-REC3's mark (lines 2527–2529); CD-3 |
| PM-0005-8 | The generated schema and types never drift (CD-3; content item 3) | regenerating the schema and content's ts-rs types gives 0 changed bytes; `tsc --strict` passes on them; renaming a field without regenerating fails the gate; a hand edit to a generated file fails the gate; 100 % of the legacy import's files, typed against the generated types, pass the Rust validator (the draft's proposed addition) | CI | the gate's G4 step (it shares ADR 0002's PM-0002-15; this row adds the content schema and content's types); the import's files checked by `tsc` against the types and then loaded by the crate | 1 of 1 each. Control: a rename with regeneration passes | R1-DATA-REC3 (lines 2527–2529); R1-BND-REC4 marks 1, 3 (lines 2304–2306); R5-LAYOUT-REC3's mark (r5 lines 1203–1204); CD-3 (proposed there: F16, R1-BND-F9) |
| PM-0005-9 | Every map migrates and round-trips exactly (CD-4; content item 4) | Cabana, DM_Tri and every saved map Mustafa supplies migrate and validate (100 %); migrate → save → load → save byte-identical (100 %); a property test of random valid documents gives the same, 0 failures in 10,000 cases (proposed: two maps exercise few field shapes); a file with a future version refused with a message naming the file, its version and the newest version the loader knows (proposed reading of "a clear message"); each migration step's golden fixture and each validation error's insta snapshot pass | CI | the migration chain on the v1 files; proptest; insta snapshots of the errors, never rewritten in CI | 1 run; the property test 20 identical verdicts. Bites: a migration step changed by one field fails its golden; a future-version file guessed instead of refused | R1-DATA-REC4's mark (lines 2539–2540); R5-TEST-REC3 (r5 lines 1431–1453: goldens, insta for validation errors); CD-4 |
| PM-0005-10 | Development reload: checked at once, simulation numbers at the next match, client content live (CD-7; content item 5; shared with PM-0005-21) | each of 20 saves of a weapon's damage change (the Shabzon's 8 → 9) validated on the development server within 1 s of the save (a plain limit, max over the 20), and the next match runs on it; the running match's per-tick state hashes after the save equal a control run with no save (proposed: CD-7's "nothing in a running match's simulation changes"); each of 20 malformed saves leaves the running match on its numbers and shows the error (file, with line and column or field path) within 1 s; each of 20 saved feel-kit changes shows on the page within 1 s, in the running match | ML: the development server native; the page in Chrome, Edge and Firefox | the development server's watcher (notify with a debouncer); a script saves the files on a fixed schedule; validation time from the file's modification time to the server's "valid" or "error" log line on the same clock; the page's arrival time stamped on the same machine (the file's route to the Worker is the boundary spike's, PM-0002-31) | 20 saves of each kind per browser. Bites: a watcher that validates only on the next match start (fails the 1 s); a reload that swaps the numbers inside the running match (fails the hash comparison) | R1-DATA-REC5 marks 1, 4 (lines 2559–2563), restated for CD-7; CD-7 (proposed there: the feel-kit 1 s) |
| PM-0005-11 | Only if OQ-0005-CD4 picks option (b): the mid-match swap stays in step (CD-7) | the client's Wasm sim applies the change at the same tick as the server, with 0 mispredictions beyond the change itself, under P2; 50 random valid tuning edits in a 5-minute bot match give 0 crashes and 0 desyncs (per-tick hashes of the server and a sampled client) | RS (server); ML: Chrome (client); L1 for the edit storm | ADR 0002's simulator at P2; the edits from a seeded script | 3 seeds for the edit storm. Bites: the client swapping one tick late | R1-DATA-REC5 marks 2, 3 (lines 2559–2562); OQ-0005-CD4 |
| PM-0005-12 | One hashed bundle, checked at join and recorded per match (CD-8; content item 6) | 100 % of 3 tampering cases (a stale file, one changed byte, a missing file) refused or re-fetched before the match starts, with 0 matches started on a bundle whose hash differs from the one the room names; a repeat visit loads the bundle with 0 bytes over the network (Resource Timing `transferSize` 0); 100 % of the spike's match records carry a hash that resolves to an archived content set (the archived file re-hashes to it) and the mode id; the Rust SHA-256 crate gives the same hash as WebCrypto on 100 % of the spike's bundles | CI (cases, records, hashes); ML: Chrome (the repeat visit) | a localhost server with immutable headers (the real host is ADR 0006's PM-0006-4); the stand-in join names the hash; WebCrypto in a secure context (localhost); every tampering case 10 times (proposed) | 1 run; the repeat visit 3 times from a fresh profile (ADR 0006's method). Bites: a Worker whose check always passes; the bundle served `no-cache` | R1-DATA-REC6's mark (lines 2576–2579); R1-DATA-OQ9 (lines 2813–2814); CD-8 |
| PM-0005-13 | The server checks every payload a client or the editor sends (CD-9; content item 7) | C sent through the private-room override path: 0 accepted invalid payloads, 0 server crashes; 100 % of valid overrides from the spike's test allow list accepted (a control: proposed); an override naming a tunable outside the list refused (100 %); a payload with a `__proto__` key refused; the editor's document layer passes a test that `Object.prototype` and every content object's prototype are unchanged after loading such a payload | CI (the server path natively; the document layer in Node under Vitest) | the override path called as the room will call it (ADR 0001's `HostCommand`); a test allow list, not the game's (its content is R1-DATA-OQ10, a design question) | 1 run. Bites: an override merged with `Object.assign` in the document layer; the allow-list check removed | R1-DATA-REC7's mark (lines 2591–2594), without the map-upload path (CD-9); R1-DATA-OQ10 (lines 2815–2816) |
| PM-0005-14 | The legacy tuning is imported once, every value accounted for (CD-10; content item 8) | 100 % of the non-look, non-state values appear in the import report as mapped (with their v2 path and their legacy `where`) or dropped (with a reason), counted per leaf (proposed reading of "value": a table mapped as a whole could hide an unmapped key; 725 leaves in the gameplay, simulation and progression groups at 9e7db28, plus every `boot_changes` entry as map data); 100 % of the look group's leaves (2,011) listed, each marked for ADR 0003's look proof test; each dead key (CD4, CD6, CD7, CD10, CD12, CD14) dropped with a reason or kept by decision; the imported set passes PM-0005-4's checks; every kept value equal to legacy's, exactly (bits) | CI | the import tool over `docs/legacy-tuning.json`; a script walks the file's leaves and matches each to a report row; the kept values compared as golden fixtures (R5-TEST-REC3) | 1 run. Bites: a key planted in a copy of the file (a new leaf under `T`) must show as unaccounted; a kept value moved by one unit in the last place fails its golden | R1-DATA-REC8's mark (lines 2605–2606); CD-10 (proposed there: the look group); R5-TEST-REC3 (r5 line 1438: "exact for every value v2 keeps") |
| PM-0005-15 | Presentation cannot reach an outcome (CD-6; AP-1) | changing every client-content value (the feel kit, the asset list, the material-slot classes, the look values) to other valid values leaves the per-tick state hashes of a 120 s scripted scenario identical, on native and Wasm (proposed: CD-6's "a presentation value cannot reach an outcome" has no mark of its own) | CI | the scenario of PM-0004-63 run twice, the second time with a seeded random valid change to every client-content file | 3 seeds. Bites: a planted read of a feel-kit value in the sim (a weapon's recoil feeding its spread) | CD-6, AP-1 (the draft's rules; the mark proposed here) |
| PM-0005-16 | What content costs to load, and its Wasm size (OQ-0005-CD2; content item 9) | reported, no pass number (the Advisor sets the budgets from it): the load time of a set the size of the imported legacy tables plus two maps (Cabana, DM_Tri) and two modes (24v24, 16v16v16: proposed, from the test-mode list), median and max over 20 cold loads, natively and in the Worker; the bytes serde_json and garde add to the Worker's Wasm module, raw and gzip | RS (native); ML: Chrome and Firefox (Wasm in the Worker) | each load in a fresh process or Worker, timed from the bytes in memory to the frozen set; the size as the module with the content crate minus a build with a stub loader, under the boundary spike's build settings (R1-RR-OQ5 asks for those settings' effect: ADR 0002's) | 20 loads per target. Passes when every cell holds a number | R1-DATA-REC1 mark 2 (lines 2496–2498); R1-DATA-OQ2 (lines 2791–2794); R1-RR-OQ5 (lines 2946–2950); CD-2 (proposed there: the Wasm half) |
| PM-0005-17 | Mustafa's content trial (OQ-0005-CD1) | the content checklist below done; his answer to OQ-0005-CD1 recorded; 0 steps left "not yet" | MP: the development server on his PC, the page in Brave | the content checklist (below) | one session; a "not yet" goes back to the draft | R1-DATA-OQ1 (lines 2788–2790: "a short trial in the spike"); plan rule 12 |
| **The content and ability spike: abilities, effects and upgrades** | | | | | | |
| PM-0005-18 | The test content runs with no ability-specific Rust code (AB-1; ability item 1) | 0 matches for the test content's ids and legacy names in the sim crate's non-test code; every test content item, fired or pressed in a scripted scenario, produces its expected event log; with every test id replaced by a random string, the scenario's event log is byte-identical after mapping the ids back (proposed: a search for names misses code keyed by a constant spelled otherwise); reported: the list of operations with the test items that use each, for the rust-reviewer | CI | `rg` over the sim crate excluding tests; the scenario in the replay runner; the id rename by script over the test content | 1 run. Bites: a planted `if id == "molotov"`, and a planted constant `FIREBOMB` keyed on the Molotov's handle (only the rename test sees it) | R1-ABL-REC1 mark 1 (lines 2365–2368); AB-1 |
| PM-0005-19 | A new combination needs data only (AB-1, AB-7; ability item 1) | a test-only definition putting the Riot Shield's block on a vehicle and the Banzai speed buff on another class loads and behaves as its parts: the vehicle takes 30 % of an SMG hit inside the block's arc and 100 % behind it; the class moves at 1.3 × its speed for 10 s, then at 1 ×; the change set that adds it touches only content files (0 Rust changes) | CI | the combination's files; `git diff --name-only` of its commit; a scripted fixture (hits from inside and outside the arc, cos 0.30; speed sampled every tick) | 1 run. Bites: a block keyed to the Peacekeeper's unit id | R1-ABL-REC1 mark 2 (lines 2369–2371); legacy numbers: block 0.7, arc cos 0.30, ×1.3 for 10 s (r1 line 1373; INV CD8, line 89) |
| PM-0005-20 | The two §6.4 primitives work (AB-1, AB-6; ability item 1) | an effect removed when its source dies, on the tick the death is processed, in 1,000 of 1,000 scripted deaths; a trigger on the last hit, and on the last periodic tick, moves the victim's item (a granted ability) to the source in 1,000 of 1,000 scripted kills, with two kills of one victim in one tick resolved by the stable order the same way on native and Wasm (proposed counts and tie case) | CI | a fixture with test-only definitions (the item and the leech rules stay undesigned, `[LATER]`) | 1 run per target. Bites: removal one tick late; a periodic-tick kill not firing the trigger | R1-ABL-REC1 mark 3 (lines 2372–2376); R1-ABL-REC3's stable order (lines 2406–2409) |
| PM-0005-21 | One number changed in data changes the next match (AB-1; ability item 1, last clause; shares PM-0005-10's run) | after the Shabzon's damage is saved as 9: 0 Rust changes; in the running match a hit deals 8; in the next match a hit deals 9, and a Conscript (100 HP) dies on the 12th body hit instead of the 13th (9 × 12 = 108 ≥ 100; 8 × 12 = 96 < 100) | CI (the counts); ML (the run of PM-0005-10) | PM-0005-10's run; the hit counts from the sim's events | as PM-0005-10. Bites: as PM-0005-10 | R1-ABL-REC1 mark 4 (lines 2377–2378); legacy Shabzon damage 8, Conscript HP 100 (`docs/legacy-tuning.json`, `WEAPONS`, `UNIT_TYPES`) |
| PM-0005-22 | Weapons as data equal the legacy tables (AB-2, AB-3; ability item 2) | for the seven legacy weapons and the Juggernaut cannon: damage per body part (head 2.6, body 1.0, legs 0.65; Bagpipe head 1.2), the Shabzon's falloff at 10, 15, 25, 35 and 50 m against legacy's `falloffMult`, pellet count, magazine, reload and burst: integer results exactly, floating-point results within 1e-6 relative; timing fields compared as stored values (the loader's converted duration in seconds within the rounding it reports, PM-0004-11) and as behaviour (each shot within one tick, 33.3 ms, of the table's time) (proposed reading: on a 30 Hz tick no shot can land on a 0.06 s instant, so 1e-6 applies to stored values only); a bot and a human firing one definition from the same inputs give identical results (whether v2 keeps one model is Mustafa's, R1-ABL-OQ6) | CI | golden fixtures captured from the legacy oracle (`tools/preview.js legacy eval`), each with the legacy commit, the inventory row and the inputs; v2's tests read them | 1 run; joins the gate. Bites: a v2 value moved beyond its tolerance (R5-TEST-REC3's mark); the legacy reset-not-carry timer (GC5) | R1-ABL-REC2's mark (lines 2396–2400); R5-TEST-REC3 (r5 lines 1431–1453); INV CS7, CD8 (lines 172, 89) |
| PM-0005-23 | One table per kind, every link checked, fuzzed definitions never panic the sim (AB-4; ability item 3; shares C) | the legacy `WEAPONS`, `UNIT_TYPES`, `VEHICLE_TYPES` and ability fields convert with every legacy number mapped once or dropped with a reason (0 unmapped, 0 mapped twice: proposed reading, as CD-10 drops bot-only fields and colours); an unknown id, an unknown tag and an out-of-range value each fail at load with the file and field named, for every kind, track definitions included (a track naming an unknown effect fails); 0 panics when every definition of C that the loader accepts runs in a scripted 300-tick scenario (10 s at 30 Hz: proposed; the research's "never panic the simulation" is about running them, not loading them) | CI; CI nightly (the sim runs on fresh seeds) | the conversion report by script; C's accepted definitions each given to a unit or weapon in the scenario | 1 run (C's 10,000 files); nightly fresh seeds. Bites: a sim that indexes an array with a pellet count of 0; a planted dangling ability id | R1-ABL-REC5's mark (lines 2460–2463); AB-4 (proposed there: track definitions) |
| PM-0005-24 | Tech tiers gate abilities and purchases (AB-5; ability item 4) | an ability press before the team's Tech 1, and a T2 purchase before Tech 2, refused by the authority in 1,000 of 1,000 scripted attempts each; the same presses and purchases after the tier accepted in 1,000 of 1,000 (a control: proposed) | CI | scripted team state at Tech 0, 1 and 2; the authority's verdict log | 1 run. Bites: an ability with no tier tag | AB-5 (proposed there; LOOP_V2 §4 lines 50–52, `[DECIDED]`; INV GE4, line 290); the count 1,000 from R1-ABL-REC4 (line 2444) |
| PM-0005-25 | The counters reproduce from data alone (AB-6, AB-7; ability item 5) | from the imported tables: the Warrior kills a Conscript in 0.86 s within one tick (13 SMG body hits, 12 × 0.072 = 0.864 s); the Peacekeeper kills a Warrior in 0.85 s within one tick when 11 of its 16 pellets land over two shots (11 × 11 = 121 ≥ 120 HP), and 10 of 16 leave it alive (a control: proposed); a Peacekeeper behind the Shield takes exactly 63 SMG body hits (62 leave 1.2 HP) and 19 without it; the knockback operation changes a target's velocity by the test weapon's value within 1e-6 relative, with 0 matches for the test weapon's id in the sim's non-test code | CI | scripted duels on the converted data (the pellet hits scripted, since spread draws from an [authority-only stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream)); the times from the sim's events; the knockback weapon is test-only (the Javelin's numbers are Mustafa's) | 1 run (deterministic). Bites: the Shield block applied to blasts only | AB-6, AB-7 (proposed there); R1-ABL-REC3's mark (lines 2416–2418: the 0.86 s, ADR 0004's, PM-0004-9); INV CD13 (line 94); arithmetic in AB-7 |
| PM-0005-26 | u1: no track-specific Rust code (AB-8; ability item 6) | 0 matches for the test tracks' ids in the sim crate's non-test code, and PM-0005-18's rename test passes with the test tracks renamed | CI | as PM-0005-18 | 1 run. Bites: a planted branch on a test track's id | AB-8 (proposed there; UPGRADE-SCOPE) |
| PM-0005-27 | u2: picks survive death (AB-8; ability item 6) | with picks on unit type U and weapon type W: the unit dies, respawns as a Recruit and buys U again; the new unit and its W carry exactly the same pick effects (effect lists compared field by field) in 1,000 of 1,000 scripted cycles | CI | scripted cycles over test tracks; it shares ADR 0004's PM-0004-64 runs (that row checks the player record; this one the effects on the new unit) | 1 run. Bites: picks applied to the soldier entity instead of from the record | AB-8 (proposed there); UPGRADE-SCOPE |
| PM-0005-28 | u3: tracks survive switching (AB-8) | U, then V, then U again: U's track unchanged field by field, V's its own, in 1,000 of 1,000 scripted cycles (proposed count, as u2) | CI | as PM-0005-27 | 1 run. Bites: one track shared by every unit type | AB-8 (proposed there); UPGRADE-SCOPE |
| PM-0005-29 | u4: a lost gun's track waits for the next gun of its type (AB-8) | the player drops W and takes another W from the world: the player's W picks apply to it; a second player who takes the first gun gets their own W picks and none of the first player's; 1,000 of 1,000 scripted exchanges (proposed count) | CI | scripted drops and claims (ADR 0004's pickup entities, PM-0004-66) | 1 run. Bites: picks stored on the gun | AB-8 (proposed there); UPGRADE-SCOPE |
| PM-0005-30 | u5: every track is empty at a match's first tick (AB-8) | 100 of 100 consecutive matches in one process start with every track empty and no pick effect on any unit | CI | shares ADR 0004's PM-0004-65 runs (that row checks the record; this one the effects) | 1 run. Bites: a pick effect left on a pooled unit | AB-8 (proposed there); UPGRADE-SCOPE; LOOP_V2 §8 |
| PM-0005-31 | u6: only offered picks count, and offers repeat (AB-8) | every offer holds the data's offer size (3) of distinct candidates whose requirements hold; a pick that was not offered, or for a closed offer, refused in 1,000 of 1,000 forged picks; 1,000 of 1,000 offered picks accepted (a control: proposed); the same seed and inputs give byte-identical offer logs on two runs, and on native and Wasm only if ADR 0004 puts the offers in a stream the client holds | CI | forged pick commands; the offer log written by the authority | 2 runs (the log); 1 run (the counts). Bites: an authority that accepts any candidate of the track; offers drawn from a clock seed | AB-8 (proposed there); UPGRADE-SCOPE; SEC-SPREAD (a)'s reasoning |
| PM-0005-32 | How many long-lived active effects the step can carry (AB-10; ability item 7) | reported, no pass number: the ability, effect and weapon step's p99 per tick in SS at 8, 16 and 32 active effects per soldier (384, 768 and 1,536 at N = 48), and the bracket in which it reaches 1 ms (where the crossing lies between two points, points 4 effects per soldier apart are added: proposed); the extra effects are permanent attribute modifiers on the soldier's unit and weapon, the shape AB-8 gives picks (proposed); the effect count per soldier checked equal to the target on every tick | RS (native); ML: Chrome and Firefox (Wasm, isolation flag on) | T per point; the 8-per-soldier point is ADR 0004's PM-0004-62 run (its 1 ms p99 and 0 allocations are that row's pass marks) | T, V per point. Passes when every cell holds a number | AB-10 (proposed there); R1-ABL-REC6 (lines 2465–2472); GL lines 397–418 (8 per soldier) |
| **ADR 0003's look proof test: this record's asset marks** | | | | | | |
| PM-0005-33 | Every test model is listed, and a broken link fails at load (AP-2, AP-3) | 100 % of the test models named by id in the test content and listed in the asset list; a dangling asset id and a slot whose class is not in the class table each fail at load with the file and field named (1 of 1 each); 0 stand-in models drawn | CI | the content crate's whole-set check on the look test's content | 1 run. Bites: the two planted defects | AP-2 (proposed there: R1-DATA-REC2's error mark, lines 2515–2516) |
| PM-0005-34 | The models load on both backends and carry no look of their own (GPU section parts 1, 6) | 0 console errors in every start; 0 meshes of a listed asset drawn with a material other than its slot class's shared material (identity against the material cache); 0 model requests other than listed glTF/GLB files, and 0 requests outside the client's host | ML: Chrome, Edge, Firefox, each on WebGPU and with `forceWebGL`; MAC: Safari 26, both backends | PM-0003-2's starts; after load, a script walks the scene graph; the page's network log | PM-0003-2's 10 starts per browser and backend. Bites: one slot left with its glTF material | the GPU section's spike part (R3-WGPU-REC1 mark 1, r3 lines 910–912); R5-GATE-REC3's G12 mutation (r5 line 1723) |
| PM-0005-35 | Materials are shared by content (GPU section parts 1, 2, 7) | the material objects on listed assets' meshes number at most the distinct (slot class, faction) pairs those meshes show, on every frame | CI (a count); ML: Chrome on WebGPU and with `forceWebGL` | LS with every test model in each faction's colours; distinct material objects counted by identity, pairs counted from the slots' classes and factions | 1 run per backend (counts repeat, PM-0003-35). Bites: a loader that clones the class material per mesh (legacy's per-mesh conversion, CL4) | the GPU section's spike part (proposed there; R3-LOOK-F3, r3 lines 331–334) |
| PM-0005-36 | The two derived per-unit caps hold (GPU section part 4; AP-5) | the test soldier as drawn: (draw calls with it shown − hidden) at most 4 × the passes that draw it; the test very large unit: at most 25 × those passes; on every frame | CI (a count); ML: Chrome on WebGPU and with `forceWebGL` | LS with one soldier, then one very large unit, shown and hidden; shadow and main pass (PM-0003-25's method, here for one unit) | 1 run per backend. Bites: a soldier with 5 material slots (10 draws over 2 passes, above 8); a very large unit of 30 single-material parts (60, above 50) | the GPU section's part 4: 4 per pass (R3-CHAR-REC1, r3 lines 1044–1045); 200 ÷ 4 ÷ 2 = 25 (R3-CHAR-REC4, r3 lines 1088–1090) |
| PM-0005-37 | Mustafa's verdict on each asset family (OQ-0005-AP2; AP-7) | every step of the families checklist below judged, with a pick or "not yet" per family | MP: Brave on Windows | the families checklist (below), each view beside a legacy screenshot at the same framing | one session; a "not yet" goes back to the draft | OQ-0005-AP2; LOOK_V2 §9 (lines 76–78); plan rule 12 |
| **The asset spike (after the asset-pipeline research)** | | | | | | |
| PM-0005-38 | The tool processes the test models and writes their list entries (AP-2, AP-6; OQ-0005-AP1's answer) | 100 % of the test models processed; every entry holds its id, its hashed file name, its slots with their classes and its counts; 100 % of the entries' triangle, material and texture counts equal a recount of the output file by an independent reader (proposed: OQ-0005-AP1 asks whether the gate recounts or trusts the entry) | CI | the tool on the test models; the recount script | 1 run. Bites: an entry with its triangle count lowered by 1 | the asset pipeline section's spike part; OQ-0005-AP1 |
| PM-0005-39 | Asset files are served like the bundle (AP-4) | every asset file's name holds the hash of its bytes (recomputed); its `Cache-Control` contains `immutable`; a repeat visit fetches 0 bytes of it (`transferSize` 0); the largest asset file under 25 MiB (26,214,400 bytes) | the client host's preview (ADR 0006's hosting setup); ML: Chrome | ADR 0006's PM-0006-4 and PM-0006-5 methods, applied to every asset file | 3 repeats from a fresh profile. Bites: an asset served `no-cache`; a planted 26 MiB file is flagged | the asset pipeline section's spike part (proposed there: R1-DATA-REC6, lines 2577–2578; R6-HOST-REC1's mark, r6 lines 641–643) |
| PM-0005-40 | The gate holds the per-unit budgets (AP-5) | a planted over-budget asset fails the gate's asset budget check; a budget raised without a change to the budget file fails; controls pass: an asset exactly at its budget, and a raise written in the budget file with its reason | CI | `gate --prove`'s mutations and controls | every `--prove` run; 1 of 1 each | the asset pipeline section's spike part (R5-GATE-REC4's mark, r5 lines 1743–1744); R5-GATE-REC3 (r5 lines 1706–1732) |
| PM-0005-41 | No asset reader reaches the sim, the server or the player build (AP-1, AP-6) | a planted glTF reader dependency in `sim`, and the same in `server`, each fails the dependency check; a planted import of the asset tool's package from `client` fails the player-build manifest walk (proposed: AP-6's "never in the player build"); control: the tool depending on a glTF reader passes | CI | `gate --prove` (the dependency rules check and the manifest walk, ADR 0002's) | 1 of 1 each. The draft's migration puts the dependency check in the foundation; the bite runs here once the tool's package exists | AP-1, AP-6 (proposed there); R5-LAYOUT-REC2's mark (r5 lines 1189–1192) |
| PM-0005-42 | Every slot is classed and every faction slot coloured, on both backends (GPU section parts 2, 6) | 0 slots without a class; every faction colour slot's colour equal to its faction definition's colour value, for every faction and test model; 0 console errors | ML: Chrome, Edge, Firefox on both backends; MAC: Safari 26 | the research's processed test models in LS; colours read back from the materials and compared with the faction data | 10 starts per browser and backend (PM-0003-2's count). Bites: a faction slot given another faction's colour | the GPU section's spike part |
| PM-0005-43 | The stress scene's fight on screen fits ADR 0003's frame totals with real models (GPU section part 4) | PM-0003-18 (frame time), PM-0003-21 (GPU time) and PM-0003-22 (draw calls, triangles, GPU memory) each pass with BS built from the real models | ML at Low; RD at High (as those rows) | those rows' methods on BS with the real models | as those rows (H, V, rule B) | the GPU section's spike part; R3-BUDGET-REC2's rows (r3 lines 1121–1137) |
| PM-0005-44 | Only if the research proposes LOD levels and the framing spreads distances (part 5): levels keep the silhouette | 0 level switches while the camera holds inside a switch's hysteresis band for 600 frames (proposed: flicker shows as switches); Mustafa's verdict in the LOD checklist below | ML: Chrome on WebGPU; MP: Brave | a scripted camera dwell at each switch distance; the LOD checklist | 1 dwell per switch; one session | the GPU section's part 5 and spike part (R3-CHAR-F10, r3 lines 490–493); OQ-0005-AS2 |
| **The Feel Lab test (ABL-PREDICT), once abilities are playable** | | | | | | |
| PM-0005-45 | What prediction changes for the Molotov and the Shield, measured beside Mustafa's verdict (AB-9; ability item 8) | reported, no pass number, per ability and per variant (predicted, server-confirmed) at P2: the time from the input to the player's own visible response (median and p95 over 200 presses); the share of shown activations the server confirms; rolled-back activations per 100 presses; the largest position correction a rolled-back Shield slow causes. The pass marks of R1-ABL-REC4 (same-tick response, undo within one round trip plus one tick, one cue per activation over 1,000 presses, no damage shown before the server's event) are ADR 0001's and run here if the predicted variant is built | ML: Chrome; the server on RS | L2 at P2; presses scripted through the test-hook API; times from the page's input event to the first frame showing the response | 200 presses per ability and variant (proposed); 3 runs (proposed) | R1-ABL-REC4 (lines 2441–2445, ADR 0001's); R2-SYNC-OQ10 (r2 lines 2936–2939); ABL-PREDICT (lines 2967–2980) |
| PM-0005-46 | Mustafa's answer to ABL-PREDICT (AB-9) | every step of the Feel Lab checklist below judged, and his answer recorded | MP: Brave on Windows, against the same server at P2 | the Feel Lab checklist (below) | one session | ABL-PREDICT (STATUS, "Waiting"; r1 lines 2967–2980); plan rule 12 |

**When a mark fails.** A spike passes when every mark that applies passes: a conditional mark (PM-0005-11, -44) only
when its condition holds; a "reported" mark (PM-0005-16, -32, -45) when every cell holds a number; a checklist mark
(PM-0005-17, -37, -46) when every step has a verdict. A failing run is never re-run until green: its data, seed, build
commit, machine and target are kept, and its cause is found and written down (R5-TEST-REC6, r5 lines 1490–1498). A
failed mark goes back to the draft, to the author of the part the row names (engine-architect for CD and AP parts;
game-designer for AB parts and u1–u6; graphics-engineer for the GPU section's parts), who revises it by the draft's
Rollback: the binary runtime format if load time or size misses its budget (option D, OQ-0005-CD2); checked newtypes
if garde fails on wasm32 (CD-2); OQ-0005-CD4's option (b) if reload at the next match proves too slow; a new reviewed
operation for one mechanic that fails PM-0005-18 to -21, option A4 with E1's tables if many fail (OQ-0005-AB13); track
operations of their own if u1 fails (PM-0005-26); new asset-list fields if the research needs them; the ported
procedural kit for a family that fails the look proof test (AP-7). The revised part returns to the advisor and to
Mustafa and the Advisor with the measured numbers. A mark of another draft that runs in these spikes (ADR 0004's
PM-0004-62 to -66; ADR 0003's PM-0003-2, -18, -21, -22, -25; ADR 0001's R1-ABL-REC4 marks) goes back to its own
draft. A mark's number changes only in the draft, with its reason, never inside a spike (budgets are data, and raising
one is a reviewed change: R5-GATE-REC4, r5 lines 1734–1738). A run set that fails V is neither a pass nor a fail. A
step Mustafa marks "not yet" goes back to the draft in the same way and is shown again.

**What has no measurable mark here** (one line each)
- CD-5 (one definition per file, one folder per kind) holds by review; its one testable rule, a duplicate id refused,
  is a defect class of C (PM-0005-4).
- CD-6's time conversion is ADR 0004's (PM-0004-11); the top speed per unit type has only its load checks until
  OQ-0005-CD8 is answered.
- CD-11 (the map document): EDITOR_V2 §4's bars (1,000 random commands undone and redone give a byte-identical
  document) are slice 5's "Done when", not a spike mark; the v1 maps' migration is PM-0005-9.
- CD-12 (moment rules): no mark until OQ-0005-CD6 gives them a format; the match record's hash and mode id are
  PM-0005-12.
- CD-13 (factions): no mark until the faction design session answers OQ-0005-CD5; a variant's dangling reference is a
  defect class of C.
- CD-14: the feel kit's live reload is PM-0005-10; the Feel Lab tool (OQ-0005-CD7) and the quality-tier rows (ADR
  0003's PM-0003-23, -33) have no mark here.
- Content item 10 (the mode files' rejections) is R2-ROOM-REC7's mark (r2 lines 2864–2867), ADR 0001's, a slice 2 and
  slice 3 milestone; this crate supplies the load and the rejections (C covers mode files).
- How content files reach the Worker under Vite (R1-DATA-OQ8) is ADR 0002's PM-0002-31, in the boundary spike.
- AB-2 (one damage pipeline) has no row of its own: each stage is reproduced by PM-0005-19, -22 and -25.
- AB-8's messages, visibility and telemetry fields are ADR 0001's and ADR 0006's; its open rules (carry slots, looted
  ammo, levelling speed: OQ-0005-AB1 to AB3) get no mark until Mustafa decides them; u1 to u6 test the data, not the
  rules.
- AB-9's own-weapon cues played once on a replay are ADR 0001's mark in the netcode proof test (R1-ABL-OQ3; R1-ABL-REC4
  mark, lines 2441–2445).
- AB-10's 1 ms p99 and zero allocations at 8 active effects per soldier are ADR 0004's PM-0004-62; the replay of every
  test ability on native and Wasm is PM-0004-63.
- AP-7 waits for OQ-0005-AP2 (PM-0005-37); GPU part 9 (trees) waits for the tree style, LOOK_V2 §8 `[OPEN]`.
- The asset-pipeline research's own marks (OQ-0005-AS1 to AS5: compression, the LOD tool, export checks, slot
  renaming, budgets beyond the two caps) are set by that research and appended to the asset spike before it starts;
  none is set here.
- GPU part 7's warm-up (no shader compiled after the loading screen) is ADR 0003's PM-0003-7 and PM-0003-8, run with
  the test models in the look proof test.
- No mark is set at 60 Hz (TICK-30); the 60 Hz margin is ADR 0001's.

#### The content checklist (Mustafa, on his Windows PC, the page in Brave; OQ-0005-CD1)
Beside it: PM-0005-10 (the 1 s marks), PM-0005-4 (the error records) and PM-0005-21 (the next-match change). The
session gives him the steps to start the development server. He records "achieved" or "not yet" (with a note) for each
step, and his answer in step 6.
1. Open the Shabzon's weapon file in a text editor, change `damage` from 8 to 9 and save. Expected: within about a
   second the development server says the content set is valid; the running match still deals 8 per hit.
2. Start the next match and shoot a Conscript with the Shabzon, body hits only. Expected: the Conscript falls on the
   12th hit (it took 13 before the change).
3. Change `damage` to the word `nine` and save. Expected: within about a second an error names the file and the field
   (`damage`), and the running match keeps its numbers. Record: whether the message tells him what to fix.
4. Delete a closing brace and save. Expected: the error names the file with the line and column of the break.
5. In the Imperial Warrior's SMG feel-kit file, change one value he can see (the muzzle flash's size), save and fire.
   Expected: the change shows within about a second, in the running match.
6. OQ-0005-CD1. Record: do hand-edited content files need comments (yes or no), and in which file he missed them.

#### The families checklist (Mustafa, in Brave on his Windows PC, in the look proof test; OQ-0005-AP2)
Beside it: PM-0005-33 to PM-0005-36 and ADR 0003's look checklist, which judges the look itself. A family can only be
picked where the look proof test shows both forms (see the spike entries); otherwise the step records "not shown".
1. The test building as an authored model and as the ported kit, at the game camera, beside legacy's house (LOOK_V2
   §2, §3). Expected: the half-timbered house with the brick-tile roof reads in both. Pick: authored, kit, or not yet.
2. The props (barrels, carts, hay bales, logs) as authored models and as the ported kit, beside legacy (LOOK_V2 §2).
   Pick: authored, kit, or not yet.
3. The ground and its textures as an authored asset (if shown) and as the ported procedural textures. Pick: authored,
   kit, or not yet.
4. Trees: not judged; the tree style is LOOK_V2 §8's `[OPEN]` item. Expected: trees shown are labelled placeholders.
5. Every test model in each faction's colours. Expected: no missing piece and no grey stand-in anywhere (a missing
   model fails at load, PM-0005-33); the factions told apart by their colour slots.

#### The LOD checklist (Mustafa, in Brave, in the asset spike; only with PM-0005-44's condition)
1. Each test model with LOD levels at the closest and the farthest camera framing (LOOK_V2 §5). Expected: simple and
   readable from a distance, real depth up close; the silhouette kept at the farthest framing.
2. A slow zoom through each switch distance and back. Expected: no visible pop or flicker at a switch.

#### The Feel Lab checklist (Mustafa, in Brave on his Windows PC; ABL-PREDICT)
Beside it: PM-0005-45's numbers for each ability and variant. The server injects P2 (100 ms round trip, ±20 ms, 2 %
loss); his own round trip to the server is recorded and added. The two variants are shown as A and B, their order
shuffled by the session and their labels hidden until he has answered (proposed: the label must not decide the feel).
1. Throw the Molotov ten times with A, then with B. Expected: he can say which one feels right for a throw.
2. Raise and lower the Riot Shield ten times with A, then with B, while moving. Expected: he can say which one feels
   right; whether the slow when the Shield goes up feels late in either.
3. Before his team reaches Tech 1, press each ability with A and with B. Expected: the press is refused in both; in
   the predicted variant a short start of the ability may show and then undo. Record: whether that undo is acceptable.
4. Record: his answer to ABL-PREDICT (does the client predict the player's own abilities), and whether it differs per
   ability.

---8<--- the subsection ends on the line above ---

### Spike entries (for docs/SPIKE_PLAN.md)
Names from the decision map's part g. Sizes: S about a day, M 2 to 4 days, L a week or more of session work.

**The content and ability spike** (ADR 0005; ADR 0004 has a share)
- **What it proves:** one Rust content crate gives one verdict on native and Wasm: exact floats and one canonical
  writer, a labelled fuzz corpus with 0 crashes and 0 accepted invalid files, bounded collections, no panics, a schema
  and types that never drift from the Rust types, exact migrations of Cabana and DM_Tri, development reload within 1 s
  with simulation numbers at the next match, one hashed bundle checked at join, server-side checks of overrides, the
  legacy tuning imported with every value accounted for, presentation that cannot reach an outcome, and load time and
  Wasm size reported for the budgets; the legacy weapons and the three T1 abilities run from data with no
  ability-specific Rust code, a new combination by data alone, the two §6.4 primitives, weapons equal to the legacy
  tables, tech gates, the counters from data alone, and Mustafa's upgrade tracks (u1 to u6) on test tracks, with the
  step's cost curve at 8, 16 and 32 active effects per soldier.
- **Its marks:** PM-0005-1 to PM-0005-32 (PM-0005-11 only if OQ-0005-CD4 picks option (b); Mustafa's content trial in
  PM-0005-17); ADR 0004's PM-0004-62 to PM-0004-66 run in the same spike.
- **Dependencies:** the drafts approved; the sim core spike's throwaway sim (the step, the stress scene, the replay
  runner and tick bench); `docs/legacy-tuning.json` and the legacy oracle (`tools/preview.js`) for the goldens;
  Cabana, DM_Tri and any saved maps Mustafa supplies; LOOP_V2 §5's T1 abilities; the reference server and the minimum
  laptop (PM-0005-16, -32); a macOS machine with Safari 26 (PM-0005-2); the boundary spike's Worker and its build
  settings (PM-0005-16; PM-0002-31 for files reaching the Worker); ADR 0002's network simulator only for PM-0005-11;
  Mustafa's session for the content trial. The decision map places it after the foundation and before slice 1.
- **Size:** L. A content crate with every check and both targets, a labelled corpus generator, an import tool, a
  reload loop on three browsers, the ability model with 12 test items and fixtures, golden capture from the legacy
  oracle, six upgrade cases and a three-point cost curve on two machines.
- **Research spikes it subsumes:** R1-DATA-REC1 to REC8 (REC5's marks 2 and 3 only under OQ-0005-CD4 (b); REC7
  without the map-upload path); R1-ABL-REC1, REC2, REC5; R1-ABL-REC6 and R1-ABL-REC3's replay half through ADR 0004's
  rows; R5-TEST-REC3's goldens for the tuning tables and locational damage. **Decides:** R1-DATA-OQ2 (the numbers;
  the Advisor sets the budgets), OQ4, OQ5, OQ9; R1-RR-OQ2 (the bounds), OQ3 (the loader's half); it gives Mustafa
  what he needs for OQ-0005-CD1, and gives the Advisor OQ-0005-AB13's trigger (PM-0005-18 to -21). The map's part g
  also lists R1-ABL-OQ7 and R1-XU-OQ7 here; ADR 0004's PM-0004-9 and PM-0004-11 run their marks in the sim core spike.

**ADR 0003's look proof test** (ADR 0005's share)
- **What ADR 0005's marks prove there:** every test model listed and every broken link refused at load; plain glTF/GLB
  exports load on both backends in the four browsers, drawn with the engine's shared materials per slot class and no
  look of their own; materials shared by (slot class, faction); the two derived per-unit caps; Mustafa's pick per
  asset family.
- **Its marks:** PM-0005-33 to PM-0005-37.
- **Dependencies:** ADR 0003's look proof test (its scene, machines and Mustafa's session); the content crate's
  whole-set check (from the content and ability spike, or a minimal loader if the look proof test runs first: the
  decision map runs it in parallel with the netcode proof test, before the content and ability spike); the test models
  as plain glTF/GLB exports with named material slots (ASSET-RESEARCH (a)); for OQ-0005-AP2, the ported kit builders
  for buildings, props and the ground beside the authored models (ADR 0003's spike entry lists only authored test
  assets, so a family cannot be picked unless this is added).
- **Size:** S inside ADR 0003's L; M if the kit builders are ported for the families' comparison.
- **Research spikes it subsumes:** none in R1–R6 (the asset marks are this draft's proposals on R1-DATA-REC2's error
  mark, R3-LOOK-F3, R3-CHAR-REC1 and R3-CHAR-REC4). It gives Mustafa what he needs for OQ-0005-AP2.

**The asset spike** (later; after the asset-pipeline research)
- **What it proves:** the automated step processes the test models and writes their list entries; asset files served
  hashed and immutable, under 25 MiB; the gate holds the per-unit budgets and keeps asset readers out of `sim`,
  `server` and the player build; every slot classed and every faction slot coloured on both backends; the stress
  scene's fight on screen from real models within ADR 0003's frame totals; LOD levels that keep the silhouette, if any;
  and the research's own marks for OQ-0005-AS1 to AS5.
- **Its marks:** PM-0005-38 to PM-0005-44 (PM-0005-44 only with its condition), plus the asset-pipeline research's
  marks, appended before the spike starts.
- **Dependencies:** the asset-pipeline research, verified; the look proof test's results and Mustafa's family picks
  (OQ-0005-AP2); ADR 0003's render budget bench and its benchmark scene (PM-0005-43); ADR 0006's preview hosting
  (PM-0005-39); the gate v2's `--prove` (the foundation); authored models for every family that goes through the flow.
  It comes before the first authored model enters the game.
- **Size:** M, to be sized again once the research names the tools.
- **Research spikes it subsumes:** none in R1–R6 (decision map part h); the asset-pipeline research's, once written.
  **Decides:** proves the answers to OQ-0005-AP1 and AS1 to AS5.

**The Feel Lab test** (ABL-PREDICT; a later test the spike plan places, decision map part g's later milestones)
- **What it proves:** what predicting the player's own abilities changes for the Molotov and the Shield at P2, in
  numbers (response time, confirmed share, rollbacks, corrections) and in Mustafa's verdict, blind between the two
  variants.
- **Its marks:** PM-0005-45, PM-0005-46; ADR 0001's R1-ABL-REC4 ability marks run here if the predicted variant is
  built.
- **Dependencies:** abilities playable in the engine (slice 4: tech tiers and the three T1 abilities); both variants
  switchable in one build; ADR 0002's network simulator (L2) at P2; the server on the reference server; Mustafa's
  session in Brave.
- **Size:** M (two variants of ability prediction, the measurements and Mustafa's session).
- **Research spikes it subsumes:** R1-ABL-REC4's ability half (ADR 0001's mark); R1-ABL-OQ3's ability-cue half.
  **Decides:** gives Mustafa what he needs for ABL-PREDICT (R2-SYNC-OQ10, R1-ABL-OQ4).

**In other spikes** (marks owned elsewhere, listed so the spike plan places them): how content files reach the Worker
under Vite, ADR 0002's PM-0002-31 in the boundary spike (R1-DATA-OQ8); own weapon cues played once on a replay, ADR
0001's in the netcode proof test (R1-ABL-OQ3); the mode files' rejections, R2-ROOM-REC7's mark, ADR 0001's, at slices 2
and 3.
