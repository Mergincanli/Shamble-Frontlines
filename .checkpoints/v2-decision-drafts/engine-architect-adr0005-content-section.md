## Content data (engine-architect)

*Written by engine-architect for ADR 0005, 2026-10-31; the frame places it verbatim. Short citations: r1, r2, r3, r4,
r5 and r6 are `docs/research/r1-simulation-core.md`, `r2-netcode.md`, `r3-rendering.md`, `r4-physics-navigation.md`,
`r5-toolchain-quality.md` and `r6-infrastructure.md`; lines are at 9e7db28. "The ability section" and "the GPU
section" are the two other sections of this record (game-designer; graphics-engineer).*

This section decides how every definition the game runs on is written, checked, versioned, reloaded and shipped:
units, vehicles, weapons, abilities, effects, upgrade tracks, modes, maps, rule tables, the feel kit and the asset
list. It designs no unit, weapon, ability, rule or look: those are Mustafa's, and the ability section owns which kinds
of definitions exist and how they link.

### Context (this section)

**What it serves.**
- VISION_V2 §6.1 (modes as data), §6.3 ("Units, vehicles, weapons and abilities are built from data, so combos and
  counters can emerge and tuning needs no code changes"), §6.5 (economy and objectives "All server-side and
  tunable"), §6.6 (every match records the numbers that matter), §6.7 ("per-faction units, uniforms and variants, as
  data"), §6.8 (moments the server recognizes).
- LOOP_V2 line 8 ("Keep them all in one data table so they can be tuned without code changes") and §7 (the feel kit
  "kept as data", "tuned live in a small Feel Lab", `[DECIDED]`).
- EDITOR_V2 §2 rules 1, 2 and 5 (the document is the truth, versioned JSON with stable ids; every edit a command with
  do and undo; a document layer that knows nothing about THREE, tested in Node), §3 (the inspector, objectives as data,
  the validation panel) and §4 (1,000 random commands undone and redone return a byte-identical document; every v1
  map migrates; exact round trips).
- UPGRADE-SCOPE (Mustafa, 2026-10-31): a track per unit type and per weapon type, as data (the ability section's AB-4
  and AB-8 carry it).

**Settled inputs.** The stack (one Rust simulation, Wasm in the browser and native on the server); SLICE-ORDER,
condition 1 (from slice 1 a client plus a local server, talking through the network's own messages); TICK-30;
MODES-3 (a); SPIKE-SCOPE (a); BASELINE-WHEN (b); UPGRADE-SCOPE; LOOK_V2 (authored glTF/GLB models are allowed in v2);
[the foundation](../research/GLOSSARY.md#the-foundation), which includes "TypeScript types generated from the Rust
code" (GLOSSARY §8, brief §2).

**The findings it turns on.** Every finding of R1's data unit was confirmed by its verifier (r1, Verifier spot-check
§6, lines 3501–3538), so the claims below are verified at the research level.
- serde's `deny_unknown_fields` makes a load fail on any unknown field, where JSON otherwise ignores it (R1-DATA-F1,
  lines 1402–1406); it cannot be combined with `flatten` (F2, lines 1407–1409); `try_from` checks a value as it loads
  (F3, lines 1410–1413); `default` and `alias` allow additive changes with no migration (F4, lines 1414–1419);
  untagged enums give poor errors (F5, lines 1420–1423).
- serde_json's `float_roundtrip` makes f64 → JSON → f64 exact "at an approximately 2x performance cost for parsing
  floats" (F6, lines 1426–1432); its errors carry a line and a column (F7, lines 1433–1437); serde_path_to_error
  reports the field path (F8, lines 1438–1441); a default recursion limit exists, its value unstated (F9, lines
  1442–1447); a file can be read as an untyped tree and migrated before the typed load (F10, lines 1448–1452).
- schemars derives JSON Schema 2020-12 from the Rust types and follows their serde attributes (F16, lines
  1489–1493), and reads garde's ranges and the doc comments into the schema (F17, lines 1494–1501); garde validates
  with field paths, and its page says nothing about Wasm (F18, lines 1502–1509).
- RON is "not designed to be a fully self-describing format" and supports `flatten` only in part (F12, lines
  1459–1467); TOML has no null (F13, lines 1468–1475); serde's YAML crate is archived (F14, lines 1476–1479).
- The Vite dev server can push custom events to the page's modules; its pages do not cover workers (F22, lines
  1526–1531).
- `JSON.parse` makes `"__proto__"` an ordinary key and `Object.assign` calls setters (F23, lines 1534–1543); OWASP:
  use `Map` or `Set` for untrusted keys (F24, lines 1544–1547) and "Validate on the server even when the browser
  checks the same fields" (F25, lines 1548–1554).
- A hash in a static file's URL plus `immutable` is the documented caching pattern (F27, lines 1561–1565); browsers
  compute SHA-256 in workers, in secure contexts only (F29, lines 1571–1575).
- No shipped-game source was read for this unit (Valve's console-variable pages answered HTTP 403): every
  recommendation rests on official documentation and the legacy facts (r1, lines 1577–1579).

**What it takes from and gives to the other drafts.**
- *Gives* (decision map, part d): typed definitions read by id (I-1); content's TypeScript types and schema (I-5's
  content part); the content bundle and its hash (I-6); the mode file (I-7's file); the map document's schema rules
  (I-9); with the GPU section, the material-slot class list (I-8).
- *Takes:* ADR 0004 runs the definitions inside [the sim step](../research/GLOSSARY.md#sim-step-time) and converts
  authored seconds into its integer time unit (its part 4); ADR 0002 sets the crate and package list, with `content` as the sim's only dependency (R5-LAYOUT-REC2),
  the `packages/types` package (R5-LAYOUT-REC3), the no-panic lints for the loader (R5-RUSTLINT-REC3), the golden
  fixtures (R5-TEST-REC3), the Vite plugin's place, and the editor's build and its double-click question (critic rank
  25); ADR 0001's `Join` and `JoinAccepted` messages carry the content hash, its `HostCommand` carries a private room's
  overrides, and a room is a mode id plus allowed overrides (R2-ROOM-REC7); ADR 0006 serves the bundle from
  [the client host](../research/GLOSSARY.md#the-client-host) and stores the match record with the content hash and the
  mode id; ADR 0003 owns the quality-tier rows and draws what the feel kit describes.
- *Inside this record:* the ability section decides which kinds of definitions exist and how they link (its AB-4);
  this section decides the format they are written in, the loader, the schema, versions, files, reload and delivery.

**What the legacy game proves** (r1, Findings §6, "Legacy facts used", lines 1581–1624; `docs/LEGACY_INVENTORY.md`).
- The starting values already exist as JSON with provenance: `docs/legacy-tuning.json` has 60 tables, each with its
  `where` (file:line) and a group (gameplay, simulation, progression, look or state) (lines 1583–1586).
- Dead tuning looks live: keys that no code reads sit in CD4, CD6, CD7, CD10, CD12 and CD14 (lines 1594–1597).
- Unchecked merges: `rulesMerge` and `envMerge` take any key, `__proto__` included (GW2, GE1; the STATUS item
  SEC-MDL-2, `docs/STATUS.md` line 257), and the editor's `RULES_SCHEMA` limits bind only its sliders, never a load
  (lines 1603–1607).
- Versions and fallbacks: the v1 map save writes `version:1` that nothing reads, rounds positions to 0.1 m and has no
  ids (GW12, lines 1610–1612); a missing objective falls back silently to DM_Cabana's (GE11, CM1, lines 1613–1615); a
  failed save falls back to memory without a word, and there is no import (ED13, lines 1616–1618); a missing gun key
  falls back to the AMK (CC9, `docs/LEGACY_INVENTORY.md` line 138).
- The weapons table mixes what the simulation decides with what the client only shows; CD12 asks to split them (lines
  1600–1602).

### Decision (this section's parts)

Content is JSON, one definition per file, loaded and validated by one Rust content crate that runs on the server, in
the client's Worker, in the editor and in the tools, with the same verdict everywhere. Its JSON Schema and its
TypeScript types are generated from the same Rust types. Every file carries a format name and a version; migrations
run on the untyped tree. The validated set ships as one hashed bundle, and every match names and records its hash.
Production pins one content set per match. In development every save is validated at once; client-only data applies
live, and simulation numbers apply from the next match (proposed; Mustafa and the Advisor decide, OQ-0005-CD4). The
map document is content data under EDITOR_V2 §2's rules.

- **CD-1. JSON is the one content format, written by one canonical writer.**
  - What: every content and tuning file is JSON: maps, modes, factions, units, vehicles, weapons, ability data,
    upgrade tracks, rule tables, the feel kit, the quality tiers and the asset list (the asset pipeline section, AP-2).
    One canonical writer: fixed key order, shortest round-trip number text, no lossy rounding. serde_json is built
    with `float_roundtrip`. Content integers stay within the range a double holds exactly; ids are strings.
  - Why: the editor already writes JSON (EDITOR_V2 §2 rule 1), the browser parses it natively, the legacy values are
    JSON, JSON Schema works on both sides, and exact floats keep every side on the same numbers (r1, lines
    2488–2490).
  - Research: R1-DATA-REC1 (r1, Recommendations §6, lines 2484–2498), resting on F6, F11–F16, F19, F26. "Shortest
    round-trip number text" from both writers has no finding; the first spike mark tests it (lines 2491–2492).
  - Need: VISION_V2 §6.1, §6.3, §6.5, §6.7; EDITOR_V2 §2 rule 1 and §4.
- **CD-2. One Rust content crate loads and validates, with the same code everywhere.**
  - What: R1-DATA-REC2's stages, in order: a byte-size cap → JSON to an untyped tree with the default depth limit →
    the format and version read, then migrated (CD-4) → the typed load with `deny_unknown_fields`, checked newtypes
    and internally tagged kinds (never `untagged`; never `flatten` where unknown keys must fail) → garde rules →
    whole-set checks (every reference resolves; every objective a mode needs is in its map; no fallback map, weapon or
    model) → a frozen content set with its hash. Errors name the file, with line and column for bad syntax and the
    field path for wrong data. Two additions: every collection carries a maximum length in its schema, so no file or
    payload sizes an allocation past a reviewed bound (R1-RR-OQ2); and the loader returns errors and never panics,
    held by the no-implicit-panic lints ADR 0002 sets on `content` (R5-RUSTLINT-REC3) and by fuzzing. The crate
    depends on nothing above it, and the sim depends on it alone (ADR 0002, R5-LAYOUT-REC2). It runs on the server
    (native), in the client's Worker (Wasm), in the editor (Wasm) and in the tools.
  - Why: one verdict on every side, and each legacy failure becomes a stage: unknown keys, no ranges at load, silent
    fallbacks (GW2, GE1, GE11, CM1; r1, lines 2507–2508). The length rules close the gap the rust-reviewer found: R1
    caps bytes, "but not array lengths" (r1, lines 2933–2935). This also disposes of SEC-MDL-2: content is parsed into
    typed Rust structs, and the fuzz corpus carries `__proto__` keys (decision map, part j, J2).
  - The bounds' values: no research number exists, so each bound is written beside its type and reviewed like code;
    the fuzz mark plants a collection one element over its bound. Two facts the spike records: the default depth
    limit's value (R1-DATA-OQ4) and whether garde builds and runs on wasm32 (R1-DATA-OQ5); if it does not, checked
    newtypes and hand-written checks replace it and schemars attributes carry the ranges (r1, lines 2801–2803).
  - Research: R1-DATA-REC2 (r1, lines 2500–2516); R1-RR-OQ2 (lines 2933–2935); the loader's side of R1-RR-OQ3
    (lines 2936–2940; the Worker's restart after a trap is ADR 0004's); R5-RUSTLINT-REC3 (r5, Recommendations, lines
    1340–1352, ADR 0002's).
  - Need: VISION_V2 §6.1, §6.3, §6.5, §6.7.
- **CD-3. The schema and content's TypeScript types come from the same Rust types.**
  - What: schemars writes JSON Schema 2020-12 for every content type at build time, with ranges and descriptions from
    the garde rules and the doc comments. The editor's inspector and the Feel Lab (CD-14) build their fields from it:
    minimum and maximum become slider limits, the description becomes the tooltip, default and deprecated are shown.
    The authoritative verdict always comes from the Rust validator (CD-2); Ajv is optional, for instant per-field hints
    only. Content's TypeScript types come from ts-rs on the same Rust types, the generator R1-BND-REC4 already names
    for "content-schema types", not from the JSON Schema. Both outputs are committed in `packages/types`, regenerated
    by the gate and diffed (ADR 0002, R5-LAYOUT-REC3).
  - Why: one source of truth for types, ranges and help text, with client-side checks only as feedback (F25). ts-rs
    has no runtime part and its default `serde-compat` feature parses serde's attributes (R1-BND-F9, r1, lines
    941–946), as schemars follows them (F16), so content needs no second type generator. A tool that turns JSON
    Schema into TypeScript was not researched.
  - Research: R1-DATA-REC3 (r1, lines 2518–2530); R1-BND-REC4 (r1, lines 2297–2308); R5-LAYOUT-REC3 (r5, lines
    1194–1205). It answers the consistency critic's rank 31 (R1-XT-OQ13, r1, lines 3088–3091; R5-XT-OQ9, r5, lines
    2012–2015) and R1-XU-OQ8 (r1, lines 2886–2890).
  - Need: VISION_V2 §6.3, §6.7; EDITOR_V2 §3 (the inspector, the validation panel); GLOSSARY §8's the foundation
    (types generated from the Rust code).
- **CD-4. Every file carries a format name and a version; migrations run on the untyped tree.**
  - What: additive changes use `default` and `alias` with no version bump; a breaking change bumps the version and adds
    one migration step N → N+1 on the untyped tree; a file newer than the loader knows is refused, never guessed. The
    legacy v1 map (GW12) is the first migration's input, and the duplicate-tree dedupe (ED13) is one of its steps.
    Each migration has golden fixtures (R5-TEST-REC3, ADR 0002's).
  - Why: EDITOR_V2 §4 asks for the migration of every v1 map and exact round trips, and the legacy `version:1` was
    never read (r1, line 2536).
  - Research: R1-DATA-REC4 (r1, lines 2532–2540), on F4, F10, F11.
  - Need: EDITOR_V2 §4; VISION_V2 §6.1, §6.7 (content keeps loading as formats change).
- **CD-5. One definition per file, one folder per kind.**
  - What: each kind's table (the ability section's AB-4: one table per kind, linked by validated ids) is the set of
    files in that kind's folder; each file holds one definition with its string id; the loader assembles every table
    and refuses a duplicate id (a whole-set check; "duplicate ids" is in REC2's fuzz corpus, r1, line 2514). A rule
    table (the economy's or the objectives' numbers) is one definition. A map is one document per file (EDITOR_V2 §2
    rule 1). LOOP_V2's "one data table" is met as one content set: every number lives once in the set, none in code.
  - Why: R1-DATA-REC1 already says "one definition per file" (r1, line 2485), and an error that names its file then
    names its definition (CD-2). R1 left open whether a kind's table is one file or many (R1-XU-OQ9).
  - Research: R1-DATA-REC1; R1-ABL-REC5 (r1, lines 2448–2459); answers R1-XU-OQ9 (r1, lines 2891–2894).
  - Need: VISION_V2 §6.3, §6.7; LOOP_V2 line 8.
- **CD-6. What the content set holds, and what the simulation may read.**
  - What, *simulation content* (the server and the client's predicting sim both read it): the ability section's kinds
    (AB-4: the tag registry, attribute sets, effects, abilities, weapons with their sim fields, units, vehicles,
    factions, upgrade tracks, and the drop and pickup values); modes, each one definition with R2-ROOM-REC7's fields
    (team count, team size, faction per team or "chosen by host", squad size, rules table, maps, bot policy, match
    length) and the overrides it allows (I-7), its range checks (a team count other than 2 or 3, a size of 0) being
    checks in this crate; maps (CD-11); rule tables for the economy, the objectives and the tech tiers (VISION_V2
    §6.5); movement data, which the navigation bake reads too (R4-FLOORS-REC3); and moment rules (CD-12).
  - What, *client content* (presentation only; the sim never reads it): the feel kit per weapon and the quality-tier
    table (CD-14), the asset list and the material-slot classes (the asset pipeline section, AP-2 and AP-3), and the
    look values ADR 0003 tunes in its look proof test (CD-10).
  - The split follows R1-DATA-REC8's simulation and client groups and CD12's split of sim fields from feel fields (r1,
    lines 1600–1602). The same crate validates both, for one verdict; the sim's step takes the simulation part by
    type, so a presentation value cannot reach an outcome.
  - Time: durations are authored in seconds and stored as authored; ADR 0004 converts them into its integer time unit
    when the sim takes a content set, reporting each rounding (R1-TICK-REC1, r1, lines 2012–2013; ADR 0004, part 4).
    The tick rate is never content: it is a build-time constant, so no reload can change it. This answers R1-XU-OQ10
    (r1, lines 2895–2898), as ADR 0004's part 4 expects.
  - Weapons and units: a weapon definition is a weapon type and names its upgrade track, so progress belongs to the
    type, not to the gun; a unit definition names its class's own weapons (the ability section, AB-4; UPGRADE-SCOPE).
    This section adds no field to either; it holds them in the same format, files and checks as every other kind.
  - Top speed: each unit definition carries one top speed per unit type (metres per second), a validated field that
    ADR 0001 (velocity bounds, the interest margin) and ADR 0004 (movement) read, in place of the three figures R2's
    units derived from the legacy tables: about 9.0, about 10.2 and at most about 13.3 m/s (r2, Open questions,
    R2-XU-OQ14, lines 3230–3241). Its starting values come from the legacy import (CD-10). How it stays true when
    effects or upgrade picks add speed is OQ-0005-CD8.
  - Research: R1-DATA-REC8 (r1, lines 2596–2606); R1-DATA-REC5 (client-only feel data, lines 2549–2551);
    R1-TICK-REC1 (lines 2009–2028); R4-FLOORS-REC3 (r4, lines 941–952); R2-ROOM-REC7 (r2, lines 2857–2867, ADR
    0001's); answers R2-XU-OQ14.
  - Need: VISION_V2 §6.1, §6.3, §6.4, §6.5, §6.7; the feel kit: LOOP_V2 §7; the tracks: UPGRADE-SCOPE.
- **CD-7. Reload: production pins one content set per match; development checks every save at once.**
  - What: the development server watches the content folder (notify with a debouncer); a change re-validates the
    whole set; an invalid set leaves the running one in place and reports its errors (file, with line and column or
    field path) to the log and to connected development tools; a valid set is ready at once. In the browser, a small
    Vite plugin pushes changed files to the page, which hands them to the Worker for the same validation (ADR 0002
    places the plugin). Client content applies live, since presentation decides nothing: this is the Feel Lab's live
    tuning (LOOP_V2 §7). Simulation content applies from the next match: a valid set becomes the set the next match
    pins, on the development server and in local play alike (proposed; R1-DATA-REC5's swap of simulation numbers
    inside a running match is option (b) of OQ-0005-CD4, for Mustafa and the Advisor). Production pins one content set
    per match; a change takes effect at the next match.
  - Under this proposal nothing in a running match's simulation changes, so three questions wait on OQ-0005-CD4: what
    a reload may change mid-match (R1-DATA-OQ7), whether movement tuning re-bakes navigation on the spot (R4-FLOORS-OQ4;
    until then a stale bake is reported, R4-FLOORS-REC3), and whether ADR 0001's development message `ContentReload`
    is needed.
  - Why: LOOP_V2's tuning loop without a half-written file or a mid-match change breaking a match (r1, lines
    2552–2553). R1's verifier found REC5's trace partial: swapping simulation numbers inside a running match is "a
    development convenience no §6 item names" (r1, lines 2555–2558). Plan rule 4 lets a draft propose only what a need
    names, so the smaller option is proposed and the larger one is listed with its cost.
  - Research: R1-DATA-REC5 (r1, lines 2542–2563), changed as above; R1-DATA-OQ12 (lines 2824–2828), OQ7 (lines
    2807–2810), OQ8 (Vite into a worker, lines 2811–2812, the boundary spike's).
  - Need: LOOP_V2 line 8 and §7 only (VISION_V2 §6.3 asks for tuning without code changes, not for live reload).
- **CD-8. Content reaches the browser as one hashed, cached bundle; every match names and records its hash.**
  - What: the build writes the validated content set as one bundle whose file name holds its hash, served as immutable
    from the client host (ADR 0006). At join the room names the hash (ADR 0001's `Join` and `JoinAccepted`); the
    Worker checks its bundle against it before the simulation starts, and a mismatch is refused (ADR 0001's
    `JoinRejected`, after which the client reloads the bundle). SHA-256 in the browser needs a secure context (F29), so
    plain-HTTP LAN development needs another path (r1, lines 2567–2569). Content is not embedded in the Wasm binary, so
    tuning ships without a code release. Small, per-match data (a private room's overrides) comes over the game
    connection. Every match record stores the content hash and the mode id (ADR 0006's record), so pick and win rates
    are grouped by tuning version; the hash must resolve to an archived content set (the mark below), and where shipped
    bundles are kept is ADR 0006's storage (I-6). The bundle, like every build file, stays under the client host's
    25 MiB per-file limit (r6, Findings §1, R6-HOST-F3, lines 72–79; R6-HOST-REC1's mark, lines 639–644, ADR 0006's).
  - Why: "client and server must run the same numbers for prediction; telemetry must know which numbers a match used"
    (r1, line 2573).
  - Research: R1-DATA-REC6 (r1, lines 2565–2580), on F27, F28, F29; the Rust SHA-256 crate is R1-DATA-OQ9 (lines
    2813–2814, settled by the spike).
  - Need: VISION_V2 §6.1, §6.6, §6.7.
- **CD-9. The server validates everything a client or the editor sends, through the same pipeline.**
  - What: a private room's overrides name tunables from an allow list, with ranges from the schema, never free paths
    into the content (ADR 0001's `HostCommand` carries them; what the list holds is a design question, R1-DATA-OQ10,
    for Mustafa with the game-designer). On the TypeScript side, parsed JSON is never merged into plain objects with
    `Object.assign` or spread; untrusted keys go into a `Map` or straight to the Wasm validator. Changed from the
    research: R1-DATA-REC7's path for a map uploaded to a private match is not built, since no VISION_V2 §6 item asks
    for one; the rule keeps it possible, because any later payload goes through CD-2 and the editor's map checks on
    the server.
  - Why: client checks are bypassable (F25); the legacy merge is how `__proto__` got in (F23, GW2) (r1, line 2587).
  - The allow list waits on [the security baseline](../research/GLOSSARY.md#the-security-baseline), listed open in
    ADR 0006 with interim positions (BASELINE-WHEN (b)).
  - Research: R1-DATA-REC7 (r1, lines 2582–2594), changed as above.
  - Need: VISION_V2 §6.1 (rooms and private matches), §6.5 ("All server-side").
- **CD-10. `docs/legacy-tuning.json` is imported once, with every key accounted for.**
  - What: a one-off import in phase 1 maps the gameplay and simulation groups to simulation content and the
    progression group to client content, drops the state group (NAV, AI_STATE), and turns `boot_changes` into map
    data. Each imported value keeps its legacy `where` (file:line); each dead key (CD4, CD6, CD7, CD10, CD12, CD14) is
    dropped with a reason or kept by decision. These are starting values, not decisions. Changed for LOOK_V2: the
    research's "leaves the look group pending the look brief" (r1, lines 2597–2598) is superseded, since the look
    brief exists (LOOK_V2, 2026-10-31); the corrected text reads "lists the look group value by value in the import
    report, each kept or dropped in ADR 0003's look proof test; none becomes v2 content before that test", because
    visible values are tuning for that test (plan rule 6).
  - Why: LOOP_V2's one data table, "filled from the measured legacy values without carrying dead tuning forward" (r1,
    line 2601).
  - Research: R1-DATA-REC8 (r1, lines 2596–2606), changed as above; the imported tables' golden fixtures are
    R5-TEST-REC3's (r5, lines 1431–1453, ADR 0002's).
  - Need: VISION_V2 §6.3, §6.5, §6.7.
- **CD-11. The map document is content data (EDITOR_V2 §2).**
  - What: a map is one versioned JSON document (CD-1, CD-4) in which every entity has a stable id (rule 1). The
    content crate validates it on every side; the editor's document layer (TypeScript, knowing nothing about THREE,
    tested in Node: rule 5) uses the generated types (CD-3) and calls the crate through Wasm for the verdict. Every
    edit is a command with do and undo over the document (rule 2), and the canonical writer (CD-1) makes EDITOR_V2 §4's
    "byte-identical document" after 1,000 undone and redone commands a testable mark. The document holds what the
    other drafts read from it (I-9): static colliders whose stable ids become the sim's static entity ids (R4-PHYS-REC2,
    R1-ECS-REC3); the baked navigation with the hash of the movement data it was baked from (R4-NAV-REC1,
    R4-FLOORS-REC3); floors, ceilings, enclosure areas and portals (R4-FLOORS-REC2, R2-INT-REC2); structure states
    (R4-OBST-REC2; the third damage stage is ADR 0004's question); objectives as data (EDITOR_V2 §3); the map's bounds
    (CD21's note); a per-map lighting mood (LOOK_V2 §6.5, its values ADR 0003's); building specs (ED10's note); and
    references to assets by id (AP-2). Every v1 map migrates (CD-4).
  - Not here: the schema's fields are EDITOR_V2's own architecture plan (EDITOR_V2 §6, step 2). Rules 3 and 4 (the
    scene mirrors the document one entity at a time; unmerged objects in edit mode) are the editor's scene, need only
    in EDITOR_V2, listed here for that plan; ADR 0003 shares only the material cache (rule 3). The editor's build and
    entry and the double-click rule (EDITOR_V2 §5; critic rank 25) are ADR 0002's. No draft designs the editor
    (decision map, part j, J3).
  - Why: EDITOR_V2 §2 rule 1 already chose versioned JSON with stable ids, and the sim, the bake, the renderer and the
    editor all read one document (I-9).
  - Research: R1-DATA-REC1–REC4; R1-ECS-REC3 (r1, lines 1973–1990); R4-PHYS-REC2 (r4, lines 721–745), R4-NAV-REC1
    (r4, lines 802–843), R4-FLOORS-REC2 (r4, lines 925–939), R4-FLOORS-REC3 (r4, lines 941–952), all ADR 0004's.
  - Need: EDITOR_V2 §2 rules 1, 2 and 5, and §4; VISION_V2 §6.1 and §6.2 (floors and interiors read from map data).
- **CD-12. Moment rules and telemetry ids are content.**
  - What: the definition ids (AB-4) key the telemetry, and every match record carries the content hash and the mode id
    (CD-8; the record's shape is ADR 0006's, R6-TEL-REC1). The rules by which ADR 0004's moment phase recognizes a
    moment are a simulation-content kind (CD-6), validated and hashed like the rest. Their format has no research and
    is OQ-0005-CD6; their thresholds are a design question (R6-TEL-OQ3, Mustafa with the game-designer).
  - Why: §6.6 tunes on evidence, so a rate must name the numbers it was measured under; §6.8's moments are recognized
    by the server, and rules held as data are tuned like every other number (§6.3).
  - Research: R1-DATA-REC6 (r1, lines 2571–2572); R6-TEL-REC1 (r6, lines 784–811, ADR 0006's); decision map, part h,
    gap G11.
  - Need: VISION_V2 §6.6, §6.8.
- **CD-13. Faction content (VISION_V2 §6.7).**
  - What: a faction is a definition (AB-4) that names its units, vehicles and colours. The three T1 classes are written
    once and shared by every faction (LOOP_V2 §5); a faction differs through its own definitions and its uniforms. Its
    soldiers are dressed by (faction, class) as data (CC6's note, `docs/LEGACY_INVENTORY.md` line 136), its uniform
    models are assets named by id (AP-2), and its colours reach the assets' faction colour slots (AP-3; the GPU
    section's part 2). A faction variant is a full definition of its own that references shared definitions by id
    (proposed: no `extends` and no merge, so the merge rules SEC-MDL-2 came from do not return); whether variants may
    override a base definition is OQ-0005-CD5, for the faction design session. Team colours never change (VISION_V2
    §5); their values are ADR 0003's look tuning.
  - Why: §6.7 asks for per-faction units, uniforms and variants as data; E1's ids let a variant reuse shared parts
    without copying them (the ability section, AB-4).
  - Research: R1-DATA-OQ6 (r1, lines 2804–2806); R1-ABL-REC5 (option E1); R3-CHAR-REC1's per-instance colour serves
    §6.7 (r3, lines 1042–1043, ADR 0003's).
  - Need: VISION_V2 §6.7; LOOK_V2 §2 (uniform details as the seed of faction identity).
- **CD-14. The feel kit, the Feel Lab's data and the quality tiers are client content.**
  - What: the feel kit is a client-only definition per weapon, keyed by the weapon's id (AB-3), holding LOOP_V2 §7's
    list (kick, muzzle flash, tracer, impact, hit marker, sound, target flinch, kill confirm) and the weapon's cosmetic
    sockets (CC9's note asks to merge the socket table with the feel kit; sockets never decide a shot). The Feel Lab
    tunes it live through the schema-driven fields (CD-3) and the live client reload (CD-7); the tool itself (where it
    runs, what it shows) has no research and is OQ-0005-CD7, its screen ADR 0003's. The quality-tier table of
    R3-BUDGET-REC3 is client content in the same format; its rows are ADR 0003's.
  - Why: LOOP_V2 §7 keeps the feel kit as data and tunes it live; CD12's note splits feel fields from sim fields (r1,
    lines 1600–1602).
  - Research: R1-DATA-REC5 ("Client-only feel data (the Feel Lab, LOOP_V2 §7) reloads freely", r1, lines 2549–2550);
    R1-DATA-OQ12 (line 2826); R3-BUDGET-REC3 (r3, lines 1166–1177, ADR 0003's); decision map, part h, gap G15.
  - Need: LOOP_V2 §7 only (the feel kit and the Feel Lab); the quality tiers: VISION_V2 §4 only (ADR 0003's).

### Options considered (this section)
Every option below comes from R1's Options section for the data unit (r1, Options §6, lines 1834–1882), or from the
open question that asked it. Columns follow template 0000. Content feeds
[the authority](../research/GLOSSARY.md#the-authority), so no option here is presentation only.

**Question 1: the file format** (r1, Options §6, Q1, lines 1836–1843)
| Option | Complexity | Cost (money, time, usage) | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort from the current code |
|---|---|---|---|---|---|---|
| **A. JSON for everything, one canonical writer** (proposed, CD-1) | Lowest: serde_json in Rust, `JSON.parse` in the browser and the editor, no second parser | exact floats cost about 2x float parsing (F6) | parse cost measured in the spike (OQ-0005-CD2) | unknown keys fail (F1); prototype keys handled (CD-9) | no comments in files (OQ-0005-CD1) | `docs/legacy-tuning.json` and the editor's maps are JSON already |
| B. RON for hand-written content, JSON for editor maps | two formats, two writers; the TypeScript editor has no first-class RON parser (F12) | not measured | not measured | partial `tag` and `flatten` support and no round-trip guarantee clash with migrations on a tree (F12) | comments and trailing commas for hand edits (F12) | new |
| C. TOML for flat tuning tables, JSON for the rest | two formats | not measured | not measured | no null (F13) | comments (F13) | arrays of tables are awkward for nested definitions (line 1841) |
| D. Text for authoring, compiled to a binary runtime format | a build step and a second schema | not researched (R1-DATA-OQ2) | possibly faster to load; not measured | not researched | none | only worth it if JSON loads too slowly (OQ-0005-CD2) |

YAML is not an option: the serde YAML crate is archived (F14).
- **A.** Pros: one parser and one writer on every side; the schema tooling exists on both sides (F16, F19). Cons: no
  comments; exact floats cost parse time. **B.** Pros: comments and native enums for hand-written Rust-style content.
  Cons: two formats, and the editor would parse RON through the Wasm loader. **C.** Pros: comments for flat tables.
  Cons: poor for units, abilities and maps. **D.** Pros: maybe faster loads. Cons: a second format to keep in step,
  unread sources.

**Question 2: typed loading and validation** (Q2, lines 1845–1855)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **A. One Rust content crate, typed all the way** (proposed, CD-2) | Medium: more Rust types; `flatten` unavailable where unknown keys must fail (F2) | one crate on four sides | one validation pass per load | one verdict on server, Worker, editor and tools; server-side by construction (F25) | none | new; legacy enforces ranges only in the editor's sliders (GW2) |
| B. Hand-written JSON Schema as the source, validated by Ajv and a Rust schema validator | two validators of one schema | the Rust types still matched by hand | not measured | two validators "can disagree" (line 1852) | none | new |
| C. Validate in the editor only | Low | low | – | ruled out: client-side checks are bypassable (F25) | none | the legacy shape (GW2) |

- **A.** Pros: one verdict everywhere. Cons: more Rust types. **B.** Pros: a schema-first workflow. Cons: two verdicts.
  **C.** Ruled out by the research.

**Question 3: versioning and migration** (Q3, lines 1857–1863)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **A. A version per file, migrations on the untyped tree** (proposed, CD-4) | Low: step functions N → N+1 on a tree (F10) | one step per breaking change, with fixtures | migrations run at load only | a future version is refused, never guessed | none | the v1 map is its first input (GW12) |
| B. Versioned Rust types side by side | Medium: V1, V2 … structs converted with `From` | types multiply with each change | not measured | type-safe | none | heavy for many small files |
| C. No versions, best effort | Low | none now | – | – | none | the legacy path; ruled out by EDITOR_V2 §4 |

- **A.** Pros: clear, testable with fixtures. Cons: migrations are untyped code. **B.** Pros: type-safe. Cons: many
  types. **C.** Ruled out (EDITOR_V2 §4 asks that every v1 map migrate).

**Question 4: hot reload, and how a running match takes a changed number** (Q4, lines 1865–1874; R1-DATA-OQ12, lines
2824–2828)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **A1. Development only; simulation numbers at the next match, client data live** (proposed, CD-7: R1-DATA-OQ12's own question, "Is a swap at the next match enough for development?") | Low: the file watcher, the whole-set check, the Vite push to the page | none at run time | nothing changes inside a match | production pins one set per match | the Feel Lab stays live; a simulation-number change needs a new match | new |
| A2. Development only; swapped between ticks, server first, clients at the same tick (R1-DATA-REC5 as written) | Medium: a reload message (ADR 0001's `ContentReload`), the "applies from tick T" rule, the rules of what may change (R1-DATA-OQ7) | REC5's marks 2 and 3 | must not cause extra mispredictions (REC5 mark 2) | production pins one set per match | simulation numbers change mid-fight in development | new |
| B. Live tuning in production too | Medium | not researched | – | matches whose rules change mid-way make telemetry (§6.6) and fairness harder; shipped practice not read (lines 1871–1872) | – | new |
| C. No hot reload; restart on any change | Lowest | none | – | – | works against LOOP_V2's tuning loop and the decided Feel Lab (lines 1873–1874) | none |

- **A1.** Pros: no reload message, no mid-match rules, no re-bake question; the Feel Lab stays live. Cons: a
  simulation-number change is seen only in a new match. **A2.** Pros: the fastest tuning loop. Cons: a message, two
  marks and rules no §6 item needs (r1, lines 2555–2558). **B.** Cons: fairness and telemetry (OQ-0005-CD3). **C.**
  Cons: against LOOP_V2 §7.

**Question 5: how content reaches the browser** (Q5, lines 1876–1882)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| A. Embedded in the Wasm binary (F28) | Low | every tuning change is a rebuild and a bigger download | – | always consistent with the code | none | none |
| **B. A content bundle with its hash in the URL, cached as immutable, checked against the hash the server names at join** (proposed, CD-8) | Low | one file per content version | cached across visits (F27) | verifiable at join (F29) | none | new |
| C. Sent by the server over the game connection at join | Low | bandwidth at every join, nothing cached | – | always matches the server | none | new |

- **B.** Pros: cacheable, independent of code releases, verifiable. Cons: a secure context for the hash (F29). **A.**
  Cons: tuning needs a release. **C.** Pros: suits small things, so CD-8 keeps it for a private room's overrides.
  Cons: costs every join.

**Question 6: where content's TypeScript types come from** (R1-XU-OQ8, lines 2886–2890; critic rank 31)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **a. ts-rs on the Rust types** (proposed, CD-3; R1-BND-REC4's option D3, r1, line 1768) | Low: the generator the protocol types already use | none at run time (no runtime part, R1-BND-F9) | – | the Rust validator stays the verdict | none | new |
| b. Generated from the JSON Schema | not researched (no generator read) | – | – | – | – | – |

**Question 7: how definitions sit in files** (R1-XU-OQ9, lines 2891–2894)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **a. One definition per file, one folder per kind** (proposed, CD-5; R1-DATA-REC1) | Low | many small files | the whole set is validated either way (CD-2) | duplicate ids refused at load | none | the import splits the legacy tables |
| b. One file per kind, holding the whole table | Low | few large files | the same | the same | none | the legacy tables' shape (CD12–CD14) and `legacy-tuning.json`'s |

- **a.** Pros: an error names the definition's own file; a change to one definition touches one file. Cons: a kind's
  numbers are spread over many files. **b.** Pros: one place to scan a kind's numbers, closest to the wording "one data
  table". Cons: against REC1's "one definition per file".

### Trade-off analysis (this section)
- **Why JSON and one crate:** together they are the fewest formats and validators that give one verdict on every side
  (server, Worker, editor and tools), keep the editor's existing choice (EDITOR_V2 §2 rule 1), read the legacy tables
  as they are, and make one Rust type the source of the schema, the TypeScript types and the check. Formats B and C add
  a second writer; validators B and C add a second verdict or none.
- **The strongest argument against:** no shipped-game source backs this unit (r1, lines 1577–1579), so it rests on
  documentation and legacy lessons. JSON has no comments (OQ-0005-CD1). Exact floats double float-parse time (F6).
  serde_json, garde and regex enlarge the Worker's Wasm, which has no budget yet (R1-RR-OQ5, r1, lines 2946–2950,
  ADR 0002's), and garde on wasm32 is unread (R1-DATA-OQ5). The content and ability spike measures load time and size
  before acceptance (OQ-0005-CD2), and option D stays the way out.
- **Reload:** the proposal gives up mid-match swaps of simulation numbers (REC5's marks 2 and 3) for a smaller
  engine: no reload message, no rules for what may change mid-match, no on-the-spot re-bake. The cost is tuning speed:
  a damage change is seen in the next match, not the current one. The Feel Lab, which LOOP_V2 §7 decided, stays live.
- **Files per definition against one file per kind:** per definition follows REC1 and names the definition in every
  error; one file per kind reads closest to LOOP_V2's wording. Either way the content set is one table in LOOP_V2's
  sense: one validated set with one hash, every number in it once.
- **Copies against overrides for faction variants:** copies by reference keep every definition readable alone and
  bring back no merge rules; overrides would save writing but reintroduce the merge that let `__proto__` in. The
  faction design session decides (OQ-0005-CD5).

### Open questions (this section)
- **OQ-0005-CD1. Do hand-edited content files need comments?**
  - Why it matters: a tuning number without its reason is hard to change safely, and JSON has none.
  - Options: (a) no comments; the reasons live in the schema's descriptions and the inspector's tooltips (F15, F17);
    (b) JSONC or JSON5 for hand-edited files (not researched).
  - Decider: Mustafa, after a short trial in the content and ability spike. What decides it: how much tuning he edits
    as raw files rather than in the editor and the Feel Lab. Research ids: R1-DATA-OQ1 (r1, lines 2788–2790).
- **OQ-0005-CD2. What load time and Wasm size may content cost, and is JSON fast and small enough?**
  - Why it matters: the research sets no number for either ("against a budget the ADR sets (no number is sourced
    yet)", r1, lines 2497–2498), and the Worker's Wasm has no size budget (R1-RR-OQ5).
  - Options: (a) JSON at run time, with budgets set from the spike's measurement (proposed, as ADR 0002 sets the Wasm
    size budget from its measurement); (b) a binary runtime format compiled from the same JSON (postcard or bincode,
    not read), only if (a) misses its budgets.
  - Decider: the content and ability spike's measurement, then the Advisor sets the budgets. What decides it: the
    measured native and Wasm load times of a content set the size of the imported legacy tables plus two maps and two
    modes, and the size serde_json and garde add to the Wasm module. Research ids: R1-DATA-OQ2 (lines 2791–2794).
- **OQ-0005-CD3. May production ever change numbers in the middle of a match?**
  - Why it matters: live changes help balance, but a match whose rules change mid-way is harder to compare and to keep
    fair (§6.6; r1, lines 1871–1872).
  - Options: (a) never: one content set per match (proposed, CD-7); (b) allowed for named tunables (the research's option B,
    r1, lines 1871–1872).
  - Decider: Mustafa. What decides it: a readable shipped-game source first (Valve's console-variable pages answered
    HTTP 403; SRC-ACCESS (a)), then his call. ADR 0001 answers the protocol side of critic rank 13. Research ids:
    R1-DATA-OQ3 (lines 2795–2798).
- **OQ-0005-CD4. In development, do simulation numbers apply at the next match, or inside a running match?**
  - Why it matters: tuning speed against an engine feature no §6 item names (r1, lines 2555–2558).
  - Options: (a) at the next match (proposed, CD-7): nothing in a running match changes, no reload message; (b)
    swapped between two ticks on the server and on every client at the same tick (R1-DATA-REC5 as written), which
    needs ADR 0001's `ContentReload` message, REC5's marks 2 and 3 (r1, lines 2560–2562), and answers to what a reload
    may change: numbers in existing definitions, yes; new or removed definitions, map geometry and navigation data
    at the next match (R1-DATA-OQ7; R4-PHYS-REC2 keeps the static world for the match, r4, lines 733–734); movement
    tuning re-baked on the spot or at the next match (R4-FLOORS-OQ4); and, when a maximum such as max health
    changes, current values clamped or scaled (the game-designer's).
  - Decider: Mustafa and the Advisor. What decides it: whether Mustafa tunes simulation numbers inside a fight or
    between matches, weighed against the cost of (b). Research ids: R1-DATA-OQ12 (lines 2824–2828), R1-DATA-OQ7 (lines
    2807–2810), R4-FLOORS-OQ4 (r4, lines 1119–1120).
- **OQ-0005-CD5. Faction variants: full definitions that reference shared ones, or overrides of a base?**
  - Why it matters: VISION_V2 §6.7 wants variants as data; overrides bring back merge rules (the SEC-MDL-2 lesson).
  - Options: (a) a variant is a full definition referencing shared definitions by id (proposed, CD-13); (b) a variant
    overrides a base definition (`extends`), with merge rules the loader enforces.
  - Decider: the faction design session (VISION_V2 §7, faction asymmetry; Mustafa). What decides it: how much
    factions differ. Research ids: R1-DATA-OQ6 (lines 2804–2806).
- **OQ-0005-CD6. How are moment rules written?**
  - Why it matters: §6.8 asks the server to recognize moments; ADR 0004's phase reads the rules, and no research
    covers a format (decision map, part h, gap G11).
  - Options (not researched, so none is proposed): (a) a closed set of recognizers in Rust with their numbers in data,
    as the ability system does for operations (AB-1); (b) conditions over the event stream written in data.
  - Decider: the moment design session (Mustafa with the game-designer, with R6-TEL-OQ3's thresholds), then ADR 0004.
    What decides it: the moments Mustafa names (VISION_V2 §6.8: "a last-second steal or a comeback"). Research ids:
    none (R6-TEL-OQ3 is a design question).
- **OQ-0005-CD7. What is the Feel Lab tool?**
  - Why it matters: LOOP_V2 §7 decided a small Feel Lab for live tuning; the research covers only its data
    (R1-DATA-OQ12, r1, line 2826; decision map, part h, gap G15).
  - Options (not researched): (a) a development page of the client; (b) a panel in the editor.
  - Decider: a later design session with Mustafa, before the feel pass (LOOP_V2 §11 leaves the feel pass out of the
    greybox v0); ADR 0003 for its screen. What decides it: where Mustafa tunes feel. Research ids: none.
- **OQ-0005-CD8. How does a unit's top speed stay true when effects or upgrade picks add speed?**
  - Why it matters: ADR 0001's velocity bounds and interest margin assume no soldier moves faster than its unit's top
    speed (CD-6); abilities (the Banzai speed buff, the ability section's AB-6) and upgrade picks can add speed.
  - Options (not researched): (a) the loader checks each unit's movement numbers and every speed modifier its own
    abilities and tracks can grant against its top speed (a whole-set check, CD-2); (b) the sim caps a soldier's speed
    at its unit's top speed (ADR 0004); (c) both.
  - Decider: ADR 0004 with the game-designer, before track content with speed picks exists. What decides it: whether
    a cap is acceptable as a rule (Mustafa's) or the check can cover every stacking policy (AB-1). Research ids:
    R2-XU-OQ14 (r2, lines 3230–3241).

### What a spike must prove (this section's part)
All in the content and ability spike (a working name; `docs/SPIKE_PLAN.md` names and places the spikes), unless an
item says otherwise. The marks are the research's, restated where this section changes the recommendation; a mark
the research lacks is marked proposed, with its reason. The test-engineer writes the final pass marks.

1. **CD-1** (R1-DATA-REC1 mark 1, r1, lines 2494–2496): every double in `docs/legacy-tuning.json`, plus a generated
   corpus of 10,000 random doubles, parses to the same 64-bit pattern in native Rust, in Rust on Wasm and in
   `JSON.parse`, and is written back as the same text: 100 % equal.
2. **CD-2** (R1-DATA-REC2 marks 1 and 2, lines 2513–2516): a fuzz corpus of 10,000 mutated content files (unknown
   keys, `__proto__` keys, wrong types, out-of-range numbers, 10,000-deep nesting, 50 MB files, duplicate ids,
   dangling references) gives 0 crashes and 0 accepted invalid files natively and on Wasm, with the same verdict and
   error path on both for 100 % of files; every error names the file and either line and column or a field path.
   Proposed addition (reason: R1-RR-OQ2): the corpus includes, for every bounded collection, a file one element over
   its bound, refused 100 % of the time. Zero panics is also R1-RR-OQ3's mark for the loader. The spike records the
   default depth limit (R1-DATA-OQ4) and runs the same tests with garde on wasm32 (R1-DATA-OQ5).
3. **CD-3** (R1-DATA-REC3's mark, lines 2527–2529; R1-BND-REC4 marks 1 and 3, lines 2304–2306; R5-LAYOUT-REC3's
   mark, r5, lines 1203–1204): for 100 % of fields with a garde range, the generated schema carries the same minimum
   and maximum; a value just outside a range is refused by the Rust validator and flagged by the schema-driven field;
   the schema check fails on a deliberately changed type; regenerating the types gives zero diff; `tsc --strict`
   passes on them; renaming a field without regenerating fails the gate. Proposed (reason: ts-rs and schemars must
   read serde's attributes alike, F16, R1-BND-F9): every file of the legacy import, typed against the generated
   TypeScript types, passes the Rust validator. These gate checks also belong to the foundation (types generated from
   the Rust code), ADR 0002's milestone.
4. **CD-4** (R1-DATA-REC4's mark, lines 2539–2540): Cabana, DM_Tri and every saved map Mustafa supplies migrate and
   validate (100 %); migrate → save → load → save is byte-identical; a file with a future version is refused with a
   clear message.
5. **CD-7** (R1-DATA-REC5 marks 1 and 4, lines 2559–2563, restated for the proposal): a saved change to a weapon's
   damage is validated on the development server within 1 s of the save, and the next match runs on it; a malformed
   save leaves the running match on its numbers and shows the error within 1 s. Proposed (reason: REC5 mark 1's 1 s,
   applied to the client path the Feel Lab uses): a saved feel-kit change shows on the page within 1 s. REC5's marks 2
   and 3 apply only if OQ-0005-CD4 picks option (b). How files reach the Worker under Vite is settled in the Rust ↔
   TypeScript boundary spike (R1-DATA-OQ8).
6. **CD-8** (R1-DATA-REC6's mark, lines 2576–2579): a client with a stale or altered bundle is refused or re-fetched
   before the match starts (100 % of 3 tampering cases: a stale file, one changed byte, a missing file); a repeat visit
   loads the bundle from the browser cache, with 0 bytes over the network for it; 100 % of the spike's match records
   carry a hash that resolves to an archived content set. The Rust SHA-256 crate gives the same hash as WebCrypto on
   every spike bundle (R1-DATA-OQ9, r1, lines 2813–2814).
7. **CD-9** (R1-DATA-REC7's mark, lines 2591–2594, restated without the map-upload path): the CD-2 corpus sent through
   the private-room override path gives 0 accepted invalid payloads and 0 server crashes; a payload with a
   `__proto__` key is refused; the editor's document layer passes a test that `Object.prototype` and every content
   object's prototype are unchanged after loading such a payload.
8. **CD-10** (R1-DATA-REC8's mark, lines 2605–2606): 100 % of the non-look, non-state values in
   `docs/legacy-tuning.json` appear in the import report as mapped (with their v2 path) or dropped (with a reason), and
   the imported set passes CD-2. Proposed (reason: CD-10's change for LOOK_V2): 100 % of the look group's values
   appear in the report, each marked for ADR 0003's look proof test.
9. **OQ-0005-CD2's numbers** (R1-DATA-REC1 mark 2, lines 2496–2498): loading a content set the size of the imported
   legacy tables plus two maps and two modes, on the native server, takes a time the spike reports. Proposed (reason:
   the client loads the same set in its Worker, and R1-RR-OQ5 asks for the size): the same load timed in Wasm in the
   Worker, and the size serde_json and garde add to the Wasm module recorded; the budgets are set from these numbers.
10. **Mode files** (R2-ROOM-REC7's mark, r2, lines 2864–2867, ADR 0001's): stated there; this crate supplies the load
    and the rejections.

No item here needs Mustafa's judgement before acceptance; the Feel Lab's feel is his to judge in playtests later.

## The asset pipeline (engine-architect)

*Written by engine-architect for ADR 0005, 2026-10-31. This section and the GPU section ("The asset pipeline: GPU
rules", graphics-engineer) together make the record's asset part: the GPU section decides what an imported model
carries and what the GPU budget allows it; this one decides how assets sit in the content model and the build.*

### Context (this section)

**What it serves.**
- LOOK_V2 §8: "v2 may use authored 3D models" (`[DECIDED]`); the flow (`[DEFAULT]`): a concept sketch, a 3D model in
  Spline, a glTF/GLB export, "an automated step that compresses it, builds LODs and renames materials into the faction
  colour slots", and "the engine applies the comic look"; budgets per unit, "triangles, materials and textures", and
  "The game never runs Spline's own runtime" (`[DEFAULT]`); the tree style (`[OPEN]`).
- The LOOK_V2 items the models carry: iconic silhouettes (§1, §5), uniform details as faction identity (§2), the
  architecture direction (§2), slimmer, serious proportions and real weapon silhouettes (§3).
- VISION_V2 §6.7 (uniforms and variants as data), §6.9 (a few
  [very large units](../research/GLOSSARY.md#big-vehicle-and-very-large-unit) without dropping frames), §4 (the
  minimum and recommended machines).

**Settled inputs.** LOOK_V2 (authored glTF/GLB models allowed; brief §1). ASSET-RESEARCH (a), the advisor's ruling of
2026-10-31 (plan, "Rulings"; decision map, part j, J4): this part takes LOOK_V2 §8's flow as given and proposes only
what R1-DATA, R3-CHAR and R3-BUDGET support; compression formats, the LOD tool, export checks, the material-slot
renaming step and the per-unit budget numbers are open questions marked "not researched", grouped under one research
unit, the asset-pipeline research (graphics-engineer, verified like R3, in its own daytime session), which
`docs/SPIKE_PLAN.md` places before the asset spike. SPIKE-SCOPE (a).

**The findings it turns on.**
- The research has no finding on the pipeline itself: no hit for KTX2, meshopt, Draco, Spline, gltf-transform or Basis
  Universal, and one glTF mention, three.js's example that loads a glTF skinned mesh (r3, Findings Q4, R3-CHAR-F8,
  lines 476–480) (plan, Risks, item 3; decision map, part h, gap G1, as corrected in part j, J4).
- What the research does support: content as validated JSON whose every reference resolves at load (R1-DATA-REC1,
  REC2; the content data section, CD-1, CD-2); static files with a hash in the URL and `immutable` (R1-DATA-F27, r1,
  lines 1561–1565); the client host's limit of 25 MiB per file (r6, Findings §1, R6-HOST-F3, lines 72–79); budgets as
  reviewed data in the repository (r5, Recommendations, R5-GATE-REC4, lines 1734–1744); static colliders "built from
  the map document, never from render objects" (r4, Recommendations, R4-PHYS-REC2, lines 723–724); a very large unit's
  parts take "each part's transform from the sim's vehicle-part records" (r3, R3-CHAR-REC4, lines 1077–1078); and the
  frame totals and per-soldier draws the GPU section derives its caps from (R3-BUDGET-REC2, R3-CHAR-REC1).

**What it takes from and gives to the other drafts.**
- *Inside this record:* the GPU section's parts 1–9 and its open questions OQ-0005-AS1 to AS6 (compression, the LOD
  tool, export checks, slot renaming, the per-unit budgets beyond its two caps, the soldier's vertex and bone counts);
  the content data section's loader, files, reload and bundle (CD-1 to CD-8).
- *Gives:* to ADR 0003, the asset list and the material-slot classes (interface I-8); ADR 0003 loads the models,
  gives each slot its class's toon material, switches LOD levels and colours faction slots per instance (its parts 5
  and 6). ADR 0003 also proposes that soldiers, weapons and very large units become authored models (its inventory
  rows CL10, CC1–CC5, CC6, CC8, GC16).
- *Takes:* the budget file and the gate (ADR 0002, R5-GATE-REC4, interface I-14); the dependency checks and the
  player build's manifest walk (ADR 0002, R5-LAYOUT-REC2); the client host that serves the files (ADR 0006); the
  vehicle-part records and the static world from the map document (ADR 0004).

**What the legacy game proves.**
- No external assets: every asset is built in code (CLAUDE.md's hard constraint), from character recipes (CL10),
  wardrobe parts (CC1), the building kit (CM9, CM10), procedural trees (GW18), farmyard props (GW19) and canvas
  textures (CL13, CM5). LOOK_V2 §8: "The legacy rule 'no external assets' ends with the legacy engine."
- Render objects decided outcomes: blasts raycast render meshes (GW15) and collider sizes depend on render maths (GW9)
  (r4, lines 730–731).
- Unchecked links between data and builders: a missing gun key falls back silently to the AMK (CC9,
  `docs/LEGACY_INVENTORY.md` line 138); builders and the look pipeline share an unchecked `userData` tag contract
  (CL14, line 123).

### Decision (this section's parts)

The flow stays LOOK_V2 §8's. An asset is presentation only: it never decides a collision, a hit or a shot.
Definitions name assets by id, and one asset list in the content set holds, for each asset, its file, its hash, its
material slots with their classes and its counts; the loader refuses a reference that does not resolve. Asset files
are served like the content bundle: hashed names, immutable, each under the host's file limit. Per-unit budgets are
reviewed data, and the gate checks every listed asset against them. The automated step is a tool outside the player
build. Which asset families go through the flow, beyond those ADR 0003 makes authored, is open; the content model
carries a procedural kit spec and an authored asset side by side until it is answered.

- **AP-1. An asset decides nothing.**
  - What: no collider, hit shape, shot origin, navigation data or vehicle-part layout is ever read from a model.
    Static colliders come from the map document (CD-11; R4-PHYS-REC2); a soldier's hit shape is simulation data (CL11,
    "rewrite in Rust"); shots leave from the sim's fixed points, and a weapon model's sockets are cosmetic (CC9's note);
    a very large unit's parts take their transforms from the sim's vehicle-part records, which the asset's part names
    match (R3-CHAR-REC4; the GPU section's part 3). The server loads no model, and no crate that reads glTF enters the
    `sim` or `server` dependency trees (a gate check, proposed below).
  - Why: legacy's render-derived colliders and render-mesh blast rays are failures R4 names (GW9, GW15); an asset
    changes whenever its art changes, and a change of art must never change an outcome (the starting principles:
    "Presentation never decides a hit, a death or a purchase", `.claude/agents/engine-architect.md` lines 25–26).
  - Research: R4-PHYS-REC2 (r4, lines 721–745); R3-CHAR-REC4 (r3, lines 1077–1093); legacy CL11, CC9.
  - Need: VISION_V2 §6.2 (one 3D simulation world) and §6.4 (multi-part vehicles); CLAUDE.md's "Animation is
    cosmetic … Nothing animated decides a hit" carries into v2 unchanged.
- **AP-2. Definitions name assets by id; the asset list is content.**
  - What: a unit, vehicle, weapon, faction uniform, building part or prop definition names its model by an asset id.
    One asset list in the content set (client content, CD-6) holds for each asset: its id; its file name with its
    hash; its material slots, each with its class (AP-3); its LOD levels, if it has any (the GPU section's part 5);
    and its counts of triangles, materials and textures, which the budget check reads (AP-5). The loader's whole-set
    checks (CD-2) refuse a reference to an asset the list does not hold, and a slot whose class is not in the class
    list, naming the file and the field; nothing falls back to a default model. Who writes the list (the automated
    step or a person), and whether the gate recounts a file or trusts its entry, is OQ-0005-AP1.
  - Why: the same verdict for assets as for every other reference; CC9's silent fallback and CL14's unchecked tags
    are the legacy failures, and R1-DATA-REC2's cross-reference stage and R1-ABL-REC5's validated ids already remove
    them for every other kind.
  - Research: R1-DATA-REC1, REC2 (r1, lines 2484–2516); R1-ABL-REC5 (r1, lines 2448–2459).
  - Need: VISION_V2 §6.7 (units, uniforms and variants as data); LOOK_V2 §8.
- **AP-3. The material-slot classes are one content table.**
  - What: the classes the GPU section's part 2 names (for example environment, unit, faction colour, emissive), and
    the slot names that map to each, are one client-content table (CD-6), validated like all content and read by ADR
    0003 (interface I-8). Adding a class is a content change reviewed together with the ADR 0003 material it needs. A
    faction's colours reach its faction colour slots from the faction definition (CD-13). The table says which slots
    exist; how a class looks is ADR 0003's and Mustafa's (LOOK_V2).
  - Why: the GPU section makes the list "this ADR's content data"; one table keeps the asset side and the renderer on
    the same names.
  - Research: R1-DATA-REC1, REC2; the GPU section's part 2 (R3-CHAR-REC3, r3, lines 1065–1075).
  - Need: VISION_V2 §6.7; LOOK_V2 §2 (uniform details), §6.1 (the value hierarchy by material class), §6.4 (the glow
    language's emissive slots).
- **AP-4. Asset files are served like the content bundle.**
  - What: each processed asset file is served from the client host with its hash in its name, as immutable, like the
    content bundle (CD-8); each file stays under the host's 25 MiB per-file limit. Because the asset list sits in the
    content set, the content hash names the asset versions a build uses.
  - Why: "A modern best practice for static resources is to include version/hashes in their URLs, while never
    modifying the resources" (F27), so a returning player fetches only files whose hash changed (the mark below).
  - Research: R1-DATA-F27 (r1, lines 1561–1565); R1-DATA-REC6 (lines 2565–2580); R6-HOST-F3 (r6, lines 72–79) and
    R6-HOST-REC1's mark (lines 639–644, ADR 0006's). Download size and compression are the GPU section's OQ-0005-AS1.
  - Need: LOOK_V2 §8 (authored models reach the browser); VISION_V2 §4 (desktop browsers first).
- **AP-5. Per-unit budgets are reviewed data, and the gate checks every listed asset against them.**
  - What: the budgets per unit LOOK_V2 §8 asks for sit in the repository's budget file (ADR 0002, R5-GATE-REC4;
    interface I-14): today the GPU section's two derived caps (a soldier as drawn: at most 4 materials per pass, from
    ADR 0003's 4 draw calls per soldier per pass, r3, lines 1044–1045; a very large unit: at most 25 parts × materials
    per pass, 200 draw calls ÷ 4 units ÷ 2 passes, r3, lines 1088–1090), the rest once OQ-0005-AS5 sets them. The gate compares each
    asset's counts in the asset list with its budget and fails on any excess; raising a budget is a reviewed change
    with its reason.
  - Why: R5-GATE-REC4's rule ("raising a budget is a reviewed change with its reason"); the frame totals every asset
    shares stay ADR 0003's (R3-BUDGET-REC2).
  - Research: R5-GATE-REC4 (r5, lines 1734–1744); R3-BUDGET-REC2 (r3, lines 1118–1137, ADR 0003's); the GPU section's
    part 4.
  - Need: LOOK_V2 §8 (budgets per unit); VISION_V2 §4, §6.9.
- **AP-6. The automated step is a tool, outside the player build.**
  - What: LOOK_V2 §8's step 4 runs as a tool ("tools may depend on everything", `.claude/agents/engine-architect.md`
    lines 33–34), never in the player build and never at run time; the game loads only its output, glTF/GLB files,
    and never Spline's runtime (the GPU section's part 6). Its package is added when the asset spike starts, with its
    need named (R5-LAYOUT-REC2: "No other crate or package until a §6 item asks for it"; need: LOOK_V2 §8 only). What
    it does (compression, LOD levels, slot renaming, export checks) is the GPU section's OQ-0005-AS1 to AS4; where it
    runs, what it writes and where source and processed files live is OQ-0005-AP1. Steps 1 and 2 (the sketch and the
    model in Spline) are authoring outside the code.
  - Why: dependencies point one way, and the player page carries only what it runs (R5-LAYOUT-REC2's manifest walk
    checks it).
  - Research: R5-LAYOUT-REC2 (r5, lines 1170–1193, ADR 0002's).
  - Need: LOOK_V2 §8 only.
- **AP-7. A procedural kit spec and an authored asset sit side by side.**
  - What: until OQ-0005-AP2 is answered, the content model carries either form for buildings, props and the ground: a
    kit spec in the map document (CD-11; ED10's note: the spec "must live in the v2 document entity") drawn by a ported
    builder, or an asset id (AP-2) drawn from an authored model. Soldiers, weapons and very large units are authored, as
    ADR 0003 proposes. Trees wait for the tree style (LOOK_V2 §8 `[OPEN]`; the GPU section's part 9).
  - Why: every v1 map must load through a migration (EDITOR_V2 §4), and its buildings are kit specs (CM7's note:
    "Migrate houses[].bt entries to kit specs"); LOOK_V2 §8 allows authored models without requiring them.
  - Research: none beyond the legacy rows (CM7, CM9, CM10, ED10); R1-DATA-REC4 for the migration.
  - Need: EDITOR_V2 §4; LOOK_V2 §8.

### Options considered (this section)
The research's options for models (R3, Options Q4: the soldier, very large unit and LOD options) are the GPU
section's. For this section's questions the research gives R1's general options, applied to assets; the rest are
marked "not researched".

**How content refers to assets**
| Option | Complexity | Cost (money, time, usage) | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort from the current code |
|---|---|---|---|---|---|---|
| **a. By id, through an asset list checked at load** (proposed, AP-2; R1-DATA-REC2's cross-reference stage) | Low | one list entry per asset | one check per load | presentation only (AP-1); no fallback model | a missing model fails at load, not on screen | new; replaces CC9's fallback |
| b. By file path in each definition, checked only when the page loads the file | Low | none | – | presentation only | a missing model shows up on screen, or as a fallback (CC9) | the legacy shape |

- **a.** Pros: one verdict for every reference; the budget check has its counts. Cons: a list to keep in step with
  the files (OQ-0005-AP1). **b.** Pros: nothing to maintain. Cons: legacy's silent failure.

**How assets reach the browser** (R1's delivery options, Options §6, Q5, r1, lines 1876–1882, applied to assets)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **A. Separate files with their hash in the name, immutable** (proposed, AP-4; F27) | Low | one file per asset version | a changed asset re-downloads alone | presentation only | none | new |
| B. Inside the content bundle | Low | every art change changes the one bundle | the 25 MiB per-file limit (R6-HOST-F3) binds bundle and art together | presentation only | none | new |
| C. Embedded in the client build | Low | every art change is a code release | – | presentation only | none | new |

- **A.** Pros: cached per file; the bundle stays small. Cons: more files per page load; the host allows 20,000 files
  per site on the Free plan (R6-HOST-F3) and no asset count is known yet. **B.** Cons: art and tuning versions tied.
  **C.** Cons: art tied to code releases.

**Which asset families go through the flow** (OQ-0005-AP2): all authored; units, vehicles and weapons authored with
buildings and props from the ported kit; or family by family after the look proof test. None is researched, so none is
proposed; the decider is Mustafa in the look proof test.

### Trade-off analysis (this section)
- **Why ids, a list and a budget check:** assets get the same verdict as every other reference (CD-2), the gate has
  the counts LOOK_V2 §8's budgets need, and nothing about an asset can change an outcome (AP-1). This is the least the
  research supports that still makes an authored model safe to add.
- **The strongest argument against:** the research says nothing about Spline's export or the step that would write
  the list (gap G1), so the list's producer, whether its counts can be trusted, and whether slot names survive export
  are all open (OQ-0005-AP1; the GPU section's AS3 and AS4); the asset spike may move fields in the list. AP-1 and
  AP-4 stand whatever it finds.
- **Side by side (AP-7):** carrying both forms costs a second drawing path for buildings and props until OQ-0005-AP2
  is answered; dropping the kit now would break the v1 migration that EDITOR_V2 §4 asks for.

### Open questions (this section)
All three are **not researched**. OQ-0005-AP1 belongs to the asset-pipeline research with the GPU section's
OQ-0005-AS1 to AS5; OQ-0005-AP2 and AP3 are Mustafa's.

- **OQ-0005-AP1. Where does the automated step run, what does it write, and where do the files live?**
  - Why it matters: the asset list (AP-2) and the budget check (AP-5) need a producer they can trust, and source and
    processed files need a home.
  - Options (not researched): (a) a tool the artist runs when importing a model, which commits the processed file and
    its list entry; (b) a CI job that processes committed exports on every change (ADR 0002 would place it); (c) a
    build step that processes exports at build time and commits nothing processed. With each: whether the gate recounts
    a file or trusts its entry; where Spline source files and raw exports are kept.
  - Decider: the asset-pipeline research, then the asset spike. What decides it: what the research finds Spline's
    export and the tools for step 4 can do. Research ids: none.
- **OQ-0005-AP2. Which asset families become authored models, and which stay procedural builders?**
  - Why it matters: LOOK_V2 §8 allows authored models without requiring them; legacy builds buildings, props, the
    ground and their textures in code (CM5–CM10, CM12, GW19, CL13, CD20, CD21), and the v1 maps' buildings are kit
    specs that must migrate (EDITOR_V2 §4). ADR 0003 already proposes authored soldiers, weapons and very large units.
  - Options (not researched): (a) every family authored; (b) buildings and props from the ported kit, everything else
    authored; (c) family by family, after the look proof test.
  - Decider: Mustafa, in the look proof test (how each looks, beside legacy screenshots, LOOK_V2 §9), with the
    asset-pipeline research's cost and tooling. Trees wait for the tree style (LOOK_V2 §8 `[OPEN]`). Research ids: none.
- **OQ-0005-AP3. May an AI-generated draft model ever ship?**
  - Why it matters: LOOK_V2 §8's step 2 names "AI-generated for drafts, or a clean model built by Claude", and nothing
    in the pipeline tells a draft from a clean model.
  - Options (not researched): (a) drafts never ship: the asset list marks drafts and a release build refuses them; (b)
    a draft may ship once Mustafa approves it; (c) no mark: every asset is judged in Brave like any other visual
    change.
  - Decider: Mustafa. What decides it: what he meant by "for drafts". Research ids: none.

### What a spike must prove (this section's part)
- **In ADR 0003's look proof test** (plain glTF/GLB exports, no compression or LOD step: ASSET-RESEARCH (a)): every
  test model is named by id in the test content and listed in the asset list; a dangling asset id, and a slot whose
  class is not in the class table, each fail at load with the file and field named (proposed; reason: R1-DATA-REC2's
  error mark, r1, lines 2515–2516, applied to assets). Mustafa judges each family's look there (OQ-0005-AP2).
- **In the asset spike**, after the asset-pipeline research: the tool processes the test models and writes their list
  entries (OQ-0005-AP1's answer); every asset file is served immutable with its hash in its name, and a repeat visit
  fetches 0 bytes of it (proposed; reason: R1-DATA-REC6's and R6-HOST-REC1's bundle mark, r1, lines 2577–2578, r6,
  lines 641–642, applied to assets); every file stays under 25 MiB (R6-HOST-REC1's mark, lines 642–643); a planted
  over-budget asset fails the gate, and a budget raised without a change to the budget file fails too (R5-GATE-REC4's
  mark, r5, lines 1743–1744, applied to asset budgets); a planted dependency on a glTF reader in `sim` fails the gate
  (proposed; reason: AP-1, checked like R5-LAYOUT-REC2's planted violations, r5, lines 1189–1192).

### Appendix A rows (for the frame)
The 26 ids the decision map gives ADR 0005 that are neither the ability section's (7) nor the GPU section's (1).

| id | disposition | where or why |
|---|---|---|
| R1-DATA-REC1 | adopted | CD-1; spike marks 1 and 9 |
| R1-DATA-REC2 | changed: adds a maximum length on every collection (R1-RR-OQ2) and the no-panic loader (R5-RUSTLINT-REC3, R1-RR-OQ3), and refuses a fallback model too; the stages as written | CD-2; spike mark 2 |
| R1-DATA-REC3 | adopted | CD-3 (content's TypeScript types come from ts-rs beside it, answering critic rank 31); spike mark 3 |
| R1-DATA-REC4 | adopted | CD-4; spike mark 4 |
| R1-DATA-REC5 | changed: simulation numbers apply at the next match in development (proposed; the mid-match swap is OQ-0005-CD4's option (b)), client content applies live, production pins one set per match as written; reason: its own partial trace (r1, lines 2555–2558) and plan rule 4 | CD-7; spike mark 5 |
| R1-DATA-REC6 | adopted | CD-8; spike mark 6 |
| R1-DATA-REC7 | changed: the path for a map uploaded to a private match is not built (no VISION_V2 §6 item asks for it); the rule holds for every payload a client or the editor sends | CD-9; spike mark 7 |
| R1-DATA-REC8 | changed: the look group, "pending the look brief" (r1, lines 2597–2598), is listed value by value for ADR 0003's look proof test, since LOOK_V2 exists | CD-10; spike mark 8 |
| R1-DATA-OQ1 | open: OQ-0005-CD1 | comments in content files (Mustafa, after a trial in the spike) |
| R1-DATA-OQ2 | spike: the content and ability spike measures load time and Wasm size; the budgets and the binary-format fallback are OQ-0005-CD2 | CD-2; spike mark 9 |
| R1-DATA-OQ3 | open: OQ-0005-CD3 | production mid-match changes (Mustafa); proposed: never |
| R1-DATA-OQ4 | spike: the content and ability spike's fuzz run records the default depth limit | CD-2; spike mark 2 |
| R1-DATA-OQ5 | spike: the content and ability spike builds and tests the crate on wasm32; the fallback is stated in CD-2 | CD-2; spike mark 2 |
| R1-DATA-OQ6 | open: OQ-0005-CD5 | proposed: full definitions that reference shared ones (CD-13); the faction design session decides |
| R1-DATA-OQ7 | open: OQ-0005-CD4 | under the proposal nothing changes mid-match; the list of what may change is option (b)'s |
| R1-DATA-OQ8 | spike: the Rust ↔ TypeScript boundary spike (how files reach the Worker under Vite) | CD-7 |
| R1-DATA-OQ9 | spike: the content and ability spike checks the Rust SHA-256 crate against WebCrypto | CD-8; spike mark 6 |
| R1-DATA-OQ12 | open: OQ-0005-CD4 | Mustafa and the Advisor; proposed: the next match |
| R1-XU-OQ8 | answered: ts-rs on the same Rust types, beside the JSON Schema in `packages/types` | CD-3 |
| R1-XU-OQ9 | answered: one definition per file, one folder per kind | CD-5 |
| R1-XU-OQ10 | answered: the tick rate is a build-time constant and never changes on a reload (ADR 0004, part 4) | CD-6, CD-7 |
| R1-RR-OQ2 | answered: a maximum length on every collection in the schema, refused at load; the network side's bounded queues are ADR 0001's | CD-2; spike mark 2 |
| R1-XT-OQ13 | answered: as R1-XU-OQ8 (critic rank 31) | CD-3 |
| R5-XT-OQ9 | answered: as R1-XU-OQ8 (critic rank 31) | CD-3 |
| R2-XU-OQ14 | answered: one top speed per unit type in the unit definition, read by ADR 0001 and ADR 0004; keeping it true under speed effects is OQ-0005-CD8 | CD-6 |
| R4-FLOORS-OQ4 | open: OQ-0005-CD4 | under the proposal movement tuning waits for the next match and a stale bake is reported (R4-FLOORS-REC3) |

Counts: adopted 4 · changed 4 · answered 7 · open 6 · spike 5 (26 rows).

### For the frame
- **The decision in lines.** *Content data:* content is JSON, one definition per file, loaded and validated by one
  Rust crate with the same verdict on server, Worker, editor and tools; schema and TypeScript types come from the same
  Rust types; every file is versioned and migrated; one hashed bundle per content set, named at join and recorded per
  match; production pins one set per match, and development validates every save at once, applying client data live
  and simulation numbers at the next match (proposed). *Asset pipeline:* LOOK_V2 §8's flow as given; an asset decides
  nothing; definitions name assets by id through a checked asset list; asset files are hashed and immutable; per-unit
  budgets are reviewed data the gate checks; the automated step is a tool; the rest waits for the asset-pipeline
  research.
- **Parts to point to:** CD-1 to CD-14; AP-1 to AP-7. **Open questions:** OQ-0005-CD1 (comments: Mustafa), CD2 (load
  and size budgets: the spike, then the Advisor), CD3 (production mid-match changes: Mustafa), CD4 (development reload
  of simulation numbers: Mustafa and the Advisor), CD5 (faction variants: the faction design session), CD6 (moment
  rule format: a design session, then ADR 0004), CD7 (the Feel Lab tool: a design session), CD8 (top speed kept true:
  ADR 0004 with the game-designer); OQ-0005-AP1 (the step's place and output: the asset-pipeline research), AP2
  (authored or procedural per family: Mustafa in the look proof test), AP3 (drafts shipping: Mustafa).
- **For `docs/adr/README.md`'s list of features whose need is outside VISION_V2 §6:** development reload, live for
  client content and at the next match for simulation content (CD-7: LOOP_V2 line 8 and §7; R1-DATA-REC5's §6.3 trace
  is partial); the feel kit and the Feel Lab's data (CD-14: LOOP_V2 §7); the map document's rules (CD-11: EDITOR_V2
  §2); the asset list, the hashed asset files, the per-unit budget check and the step's tool (AP-2 to AP-6: LOOK_V2
  §8); the kit spec beside the authored asset (AP-7: EDITOR_V2 §4, LOOK_V2 §8); the quality-tier table's format
  (CD-14: VISION_V2 §4 only, ADR 0003's rows). The upgrade tracks' files are the ability section's kinds (need:
  UPGRADE-SCOPE).
- **Consequences.** *Easier:* tuning with no code change; one verdict everywhere, so a file the editor accepts is a
  file the server accepts; editor and Feel Lab fields from the schema; telemetry grouped by content hash; private-room
  overrides that cannot reach outside an allow list; an authored model that cannot change an outcome and cannot ship
  over budget. *Harder:* a Rust type and a reviewed length bound per collection; JSON without comments; a simulation
  number tested in a new match, not the current one; two generators that must agree; an asset list to keep in step
  with the files; two drawing paths for buildings and props until OQ-0005-AP2 is answered. *Revisit when:* the
  spike's load time or Wasm size misses its budget (OQ-0005-CD2, option D); garde fails on wasm32; Mustafa and the
  Advisor pick the mid-match swap (OQ-0005-CD4); the faction design session (OQ-0005-CD5); a need for map upload
  appears (CD-9); a shipped-game source on mid-match tuning becomes readable (OQ-0005-CD3); the asset-pipeline
  research lands (OQ-0005-AP1, AS1 to AS5); Mustafa answers the tree style.
- **Hard constraints in CLAUDE.md that change:** "No external assets (procedural textures, synthesized audio)" stops
  applying to v2 code under LOOK_V2 §8 ("The legacy rule 'no external assets' ends with the legacy engine"); authored
  glTF/GLB models enter only through AP-2 to AP-6, and synthesized audio is ADR 0003's matter. "Animation is cosmetic
  … Nothing animated decides a hit" carries into v2 unchanged (AP-1). "Persistence only through KVStore" is untouched:
  content is not persistence. Spikes are exempt (SPIKE-SCOPE (a)). Nothing in CLAUDE.md or `.claude/` changes in this
  PR.
- **Legacy inventory rows** (the rows ADR 0003 leaves to this record: the building kit, the kit and ground textures
  and builders, the tree and prop builders, and the other asset rows; the inventory file is not edited, the change
  waits for Mustafa's acceptance; "pending" is the row's *Look brief pending* note):

  | Row id (`LEGACY_INVENTORY.md` line) | Mark now | Proposed mark | Why |
  |---|---|---|---|
  | CM1, CM2, CM4 (149, 150, 152) | redesign (pending) | unchanged: the maps migrate into the map document (CD-4, CD-11); pending becomes "asset families pending (OQ-0005-AP2)" | LOOK_V2 now sets the look; whether the maps' buildings are kit specs or authored assets is open |
  | CM7 (155) | port to TypeScript (pending) | port to TypeScript as kit presets, except the thatched roof, which goes in its current form; pending becomes "OQ-0005-AP2" | LOOK_V2 §3 `[DECIDED]`: "The thatched roof goes in its current form"; §2 keeps the half-timbered house with the brick-tile roof |
  | CM5, CM6, CM8, CM9, CM10, CM12, CL13, CD20, CD21, GW19 (153, 154, 156, 157, 158, 160, 122, 101, 102, 207) | port to TypeScript (pending) | unchanged (the seeded procedural technique, its data in the map document); pending becomes "OQ-0005-AP2: authored through the flow or kept procedural, family by family" | LOOK_V2 §8 allows authored models without requiring them; §2 keeps the architecture direction and the props "as a base to build on" |
  | GW18 (206) | port to TypeScript (pending) | unchanged; pending becomes "the tree style, LOOK_V2 §8 `[OPEN]`" | the tree style is Mustafa's open item |
  | GW20 (208) | redesign (pending) | unchanged (LOOP_V2 §2, §8); pending becomes "OQ-0005-AP2" for the structures' builders | the structures' look follows the families' answer |
  | ED9 (344) | redesign (pending) | unchanged; pending becomes "the mood values are ADR 0003's look-test tuning (LOOK_V2 §6.5)" | the mood lives in the map document (CD-11); LOOK_V2 §6.5 `[DEFAULT]` sets moods per map |
  | ED10 (345) | redesign (pending) | unchanged; the spec lives in the document entity (CD-11); pending becomes "OQ-0005-AP2" | ED10's own note; LOOK_V2 §8 |

  The content data parts change no mark; the marks they rely on stand: CD4, CD6, CD7, CD10, CD12, CD14 (their dead keys
  go through the import, CD-10); GW2, GE1 (rules as validated data); GW12, GE11, CM1, ED13 (CD-4, CD-11); CD2 (KVStore
  stays a client port; content does not pass through it); CC9 (port to TypeScript; its sockets join the feel kit,
  CD-14); TL11 (the legacy oracle the import's goldens use). Not listed here: ED5 and ED7 (the Character Lab and the
  Anim Lab), which follow ADR 0003's CL10 and GA rows; the frame or ADR 0003 places them.
- **Migration (proposed places in SLICE-ORDER; [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test)
  runs first and needs nothing from this record):**
  1. *The foundation:* the `content` crate's place in the workspace and the generated schema and types in
     `packages/types`, diffed by the gate (CD-3; ADR 0002's milestone); the dependency check that keeps glTF readers
     out of `sim` and `server` (AP-1).
  2. *The content and ability spike,* after the sim core spike and before slice 1 (decision map, part g): spike marks
     1–9 of the content data section.
  3. *Slice 1, move & shoot:* the crate with CD-1 to CD-8 on weapons and movement (the import's weapon and movement
     tables, CD-10); the bundle and its hash in local play. *Done when:* spike marks 1, 2, 4 and 6 pass on the real
     crate in CI.
  4. *Slice 2, bots and three teams, played locally:* units, factions and modes as files (CD-6, CD-13); Cabana and
     DM_Tri migrated (CD-4, CD-11).
  5. *Slice 3, online:* the hash at join with ADR 0001 (CD-8); the server's validation of private-room overrides
     (CD-9).
  6. *Slice 4, the loop:* the rule tables (CD-6); moment rules once OQ-0005-CD6 is answered (CD-12); the feel kit and
     the Feel Lab for the feel pass (CD-14); the upgrade tracks' files.
  7. *Slice 5, Octopon-3 and Editor v2:* the map document under EDITOR_V2 §2 (CD-11), with EDITOR_V2's own
     architecture plan.
  8. *Slice 6, scale to 48:* the load-time and size budgets re-measured on the full content (OQ-0005-CD2).
  - *Assets:* the look proof test uses plain exports; the asset-pipeline research, then the asset spike, come before
    the first authored model enters the game (AP-1 to AP-6).
- **Test plan:** the spike marks become CI checks: the fuzz corpus on native and Wasm (CD-2); the float round trip
  (CD-1); the schema and types diff (CD-3); migration goldens, with insta snapshots of validation errors
  (R5-TEST-REC3; CD-4); the import report (CD-10); the bundle tampering cases (CD-8); the asset list's reference
  checks, the per-unit budget check and the glTF dependency check (AP-1, AP-2, AP-5).
- **Rollback:** if JSON misses its budgets, the build compiles the same JSON into a binary runtime format (option D)
  and JSON stays the authored form; if garde fails on wasm32, checked newtypes and hand-written checks replace it
  (r1, lines 2801–2803); if next-match reload proves too slow for tuning, OQ-0005-CD4's option (b) is REC5 as written,
  and ADR 0001's catalogue already holds its message; if the asset list does not fit what the asset-pipeline research
  finds, its fields change and AP-1, AP-4 and AP-5 stand.
- **Seams the other drafts carry:** ADR 0004: converting authored seconds (part 4), the moment phase (CD-12), the
  answer to OQ-0005-CD8, and the static world and bake read from the map document; ADR 0001: the content hash in
  `Join` and `JoinAccepted`, the overrides in `HostCommand`, `ContentReload` only if OQ-0005-CD4 picks (b), the mode
  file's use in rooms; ADR 0002: `content` in the crate list, `packages/types`, the loader's lints, the goldens, the
  Vite plugin, the budget file and its gate check, the manifest walk; ADR 0006: serving the bundle and the asset files
  immutable, keeping shipped bundles so a recorded hash resolves, the record's content hash and mode id, and the allow
  list under the security baseline; ADR 0003: the slot classes and the asset list (I-8), the quality-tier rows, the
  Feel Lab's screen, the look values from the import report.
- **Ids:** the map gave this unit 26 ids (above), and all belong here. R3-CHAR-OQ3, which the map also gives ADR 0005,
  is the GPU section's (its OQ-0005-AS6).
