# ADR-0005: Content data, abilities and assets

**Status:** Proposed
**Date:** 2026-10-31
**Author:** engine-architect (frame, content data, the asset pipeline) · game-designer (the ability and effect system) · graphics-engineer (the asset pipeline's GPU rules) · test-engineer (the "Pass marks" table, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no

This record decides how everything the game runs on is written down as data and checked: units, vehicles, weapons,
abilities, effects, Mustafa's in-match upgrade tracks, modes, maps, rule tables, the feel kit and the list of 3D
models. All of it is plain JSON files that one Rust program, the content crate, reads and checks in the same way on
the server, in the browser and in the editor; a file with a mistake is refused with the file and the field named, and
nothing is quietly replaced by a default. Abilities and effects are built from a short, fixed list of operations
written in Rust and combined by data (a cut-down version of Unreal's Gameplay Ability System); weapons are data too, and
every hit goes through one damage pipeline, so a combo or a counter can be tried by changing data. The upgrade tracks ride on the same
system: a track is data, a pick is an effect or a granted ability, and the player's progress lives on the player's
record, not on the soldier. Authored 3D models (LOOK_V2 §8) enter as listed content: a model decides nothing in the
simulation, carries no look of its own and must fit per-unit budgets the gate checks; how models are compressed, given
levels of detail and renamed is not researched yet and waits for its own research. The record designs no unit, weapon,
ability, upgrade, rule or look: those stay Mustafa's, and every question his rules leave open is listed for him with
its options and no pick. Four sections by three authors carry the detail; this frame joins them.

## Context

### Requirements
- **VISION_V2 §6** (lines 62–81), the items this record serves and the part that serves each:
  - §6.3, "A data-driven ability and effect system, like UE's Gameplay Ability System. Units, vehicles, weapons and
    abilities are built from data, so combos and counters can emerge and tuning needs no code changes": the ability
    section (AB-1 to AB-7) on the content data section's format and loader (CD-1 to CD-6).
  - §6.1, modes as data, rooms and private matches: the mode file (CD-6) and the private-room overrides (CD-9).
  - §6.2, a true 3D simulation: the map document holds what the sim, the bake and the renderer read (CD-11), and no
    model decides a collision (AP-1).
  - §6.4, big, multi-part vehicles with their own weapons and abilities: the vehicle kind and the leech and marking
    primitives (AB-4, AB-6); parts drawn from the sim's part records (the GPU section's part 3; AP-1). Seats are
    `[OPEN]`: the vehicle kind leaves their count open (AB-4).
  - §6.5, economy and objectives, "All server-side and tunable": the rule tables (CD-6) and the tech tiers as tag
    gates (AB-5).
  - §6.6, balance telemetry: definition ids key the match record, and every match records its content hash and mode
    id (CD-8, CD-12, AB-7).
  - §6.7, per-faction units, uniforms and variants as data: CD-13, AB-4, AP-2, AP-3 and the GPU section's part 2.
  - §6.8, moments the server recognizes: moment rules as content (CD-12). Kill-cams, replays and highlights are
    `[LATER]`: matches recorded with their content hash keep them possible.
  - §6.9, a few [very large units](../research/GLOSSARY.md#big-vehicle-and-very-large-unit) without dropping frames:
    the per-unit caps (the GPU section's part 4; AP-5).
  - §6.10, accounts, is `[LATER]`: the upgrade tracks reset every match and need no persistence (AB-8), and the
    question of how account unlocks reach a match is listed (OQ-0005-AB5).
- **VISION_V2 §2** (lines 25–40): pillar 3, "every strong thing has an answer", and pillar 4, "Every choice costs
  something"; pillars 1 and 2 pull against each other and their mechanics are `[OPEN]`. **§3** (lines 42–49):
  two-team and three-team battles are both tested, and modes are data. **§4** (lines 51–55): four desktop browsers,
  the minimum laptop at Low and the recommended machine at High.
- **LOOP_V2:** "Every number here is a starting value. Keep them all in one data table so they can be tuned without
  code changes" (line 8); the abilities, tiers and counters of §4 to §6; the feel kit, active reload and the Feel Lab
  of §7; the three unlock gates of §8.
- **EDITOR_V2 §2** rules 1, 2 and 5 (the document is the truth; every edit is a command; a document layer that knows
  nothing about THREE), with rules 3 and 4 listed for the editor's own plan; **§3** (objectives as data, the
  validation panel); **§4** (every v1 map migrates; 1,000 commands undone and redone return a byte-identical
  document).
- **LOOK_V2 §8** (lines 65–74): authored 3D models `[DECIDED]`; the five-step flow and the budgets per unit
  `[DEFAULT]`; "The game never runs Spline's own runtime"; the tree style `[OPEN]`. The items a model carries:
  silhouettes (§1, §5), uniform details (§2), proportions and weapon silhouettes (§3), emissive accents (§6.4).

### Settled inputs (never reopened here)
- **The stack:** one Rust simulation, native on the server and Wasm in the browser's Worker (CLAUDE.md, "The v2
  rebuild"; brief §1).
- **UPGRADE-SCOPE** (Mustafa, 2026-10-31; docs-keeper records it in STATUS with the day's decisions; his words are
  quoted in full in the ability section's Context): every unit and every weapon has its own roguelike upgrade track
  (pick one of three per level), owned by the player; tracks survive death and switching and reset at the start of
  every match; a class always comes with its own weapon; other weapons come only from killed enemies' drops, as world
  pickups; progress belongs to the weapon type; the tracks live on the PlayerState, not the Pawn. Still open, for
  Mustafa: carry slots, ammo for looted weapons and how fast tracks level.
- **SLICE-ORDER** and its three conditions, **TICK-30** (a 30 Hz server tick at the baseline), **MODES-3 (a)**
  ([the largest mode](../research/GLOSSARY.md#the-largest-mode-and-its-n) is 24v24, N = 48), **SEC-SPREAD (a)**
  (weapon spread in an [authority-only stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream)),
  **ABL-PREDICT** (open until the Feel Lab test), **BASELINE-WHEN (b)**
  ([the security baseline](../research/GLOSSARY.md#the-security-baseline) listed open in ADR 0006),
  **SPIKE-SCOPE (a)** and **R7-MARKS (a)** (all in STATUS, "Decisions pending").
- **LOOK_V2** (Mustafa's look brief, 2026-10-31; brief §1): authored glTF/GLB models are allowed in v2.
- **ASSET-RESEARCH (a)** (the advisor's ruling, 2026-10-31): the asset part takes LOOK_V2 §8's flow as given and
  proposes only what R1-DATA, R3-CHAR and R3-BUDGET support; compression formats, the LOD tool, export checks, the
  material-slot renaming step and the per-unit budget numbers are open questions marked "not researched", grouped
  under one research unit, the asset-pipeline research (graphics-engineer, verified like R3, in its own daytime
  session), which `docs/SPIKE_PLAN.md` places before the asset spike.
- **GLOSSARY §8:** [the foundation](../research/GLOSSARY.md#the-foundation) includes "TypeScript types generated from
  the Rust code"; [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test) runs before the foundation
  (SLICE-ORDER, condition 3); it is throwaway and needs none of this record's machinery.
- **No design here** (the advisor's condition 11, 2026-10-31: escalation category 2): this record designs no upgrade,
  ability, unit or rule. It shows that the engine can carry Mustafa's answers and lists what his rules leave open.

### How this record is put together
Four sections by three authors, placed below verbatim (decision map, part j, J4; the advisor's condition 11):
"Content data" (engine-architect), "The ability and effect system" (game-designer), "The asset pipeline"
(engine-architect) and "The asset pipeline: GPU rules" (graphics-engineer). Each section keeps its own context,
options, trade-offs, open questions and spike part. This frame adds the joined decision, the options that cross
sections, the consequences, every open question in one list, the joined spike, migration, test and rollback plans, and
one Appendix A. Where two sections disagreed, the frame asks the question (OQ-0005-1, OQ-0005-2); after the fix
round no section answers either, and each points to it.

### Where this record sits among the drafts
It defines the content every other draft reads (decision map, part d): typed definitions read by id (I-1; ADR 0004
executes them and sizes its pools from the whole-set check's report of each pool's worst case, its part 1), content's
schema and TypeScript types (I-5's content part, in ADR 0002's `packages/types`), the content bundle and its hash (I-6;
ADR 0006 serves it and archives every content set a match record names, its part 12; ADR 0001 names it at join), the
mode file (I-7; the room is ADR
0001's), the asset rules and the material-slot classes (I-8; ADR 0003 draws them), the map document's schema rules
(I-9) and the upgrade data model (I-21; the player record is ADR 0004 part 18, the messages and visibility ADR 0001
part 12, the telemetry fields ADR 0006 part 12, the offer on the page ADR 0003 part 10). It takes the budget file and
its gate check (I-14, ADR 0002), the events the feel kit answers (I-20, ADR 0004) and the protocol's hash, override and
development messages (I-4, ADR 0001). Its seams are the decision map's S5 (content, execution and drawing), S6
(moments and telemetry), S9 (the look and the asset rules) and S10 (the editor).

## Decision

All game content is data in one model. Content is JSON, one definition per file, read and checked by one Rust content
crate that gives the same verdict on the server, in the client's Worker, in the editor and in the tools; its JSON
Schema and its TypeScript types come from the same Rust types; every file carries a version; the checked set ships as
one hashed bundle that every match names at join and records. Abilities, effects and weapons are kinds in that model: a
reduced subset of Unreal's Gameplay Ability System (GAS), written as data over a closed set of Rust operations, with
weapons keeping their own bookkeeping on one damage pipeline and the team's tech tiers as tag gates. Mustafa's upgrade
tracks are one more kind: a pick is an effect or a granted ability, and the player's progress lives on the player's
record (ADR 0004), not on the soldier. Authored models enter through LOOK_V2 §8's flow as listed content: a model
decides no outcome, carries no look of its own, names its material slots by class, stays within per-unit caps the gate
checks and is served hashed and immutable; what the research does not cover waits for the asset-pipeline research. In
development every save is checked at once; client content applies live and simulation numbers from the next match
(proposed; OQ-0005-CD4).

The parts, each pointing to the section part that decides it:
1. **Format, loader and checks:** JSON with one canonical writer (CD-1); one content crate with a whole-set check,
   length bounds on every collection, a report of each sim pool's worst case per mode, and no panics (CD-2); the
   schema and content's TypeScript types from the same Rust types, the generators' derives kept out of the shipped
   builds (CD-3); versions and migrations (CD-4); one definition per file, one folder per kind (CD-5). *Need:* VISION_V2
   §6.1, §6.3, §6.5, §6.7; the pools' report: ADR 0004 parts 1 and 6.
2. **What the content set holds:** simulation content and client content, split so that a presentation value cannot
   reach an outcome, with authored seconds converted by ADR 0004 and one top speed per unit type (CD-6); moment rules
   and telemetry ids (CD-12); factions and their variants (CD-13); the feel kit, the Feel Lab's data and the
   quality-tier table (CD-14). *Need:* VISION_V2 §6.1, §6.3 to §6.8; the feel kit: LOOP_V2 §7 only; the quality-tier
   table: VISION_V2 §4 only.
3. **Reload, delivery and trust:** development reload (CD-7, proposed: simulation numbers at the next match); one
   hashed bundle named at join and recorded per match (CD-8); the server checks every payload through the same
   pipeline (CD-9); the legacy tuning imported once, every key accounted for (CD-10). *Need:* VISION_V2 §6.1, §6.3,
   §6.5, §6.6, §6.7; reload: LOOP_V2 line 8 and §7 only.
4. **The map document** as content under EDITOR_V2 §2 (CD-11). *Need:* EDITOR_V2 §2 and §4; VISION_V2 §6.1, §6.2.
5. **The ability and effect model:** the reduced GAS subset (AB-1); one damage pipeline (AB-2); weapons' own
   bookkeeping (AB-3); one table per kind, linked by validated ids (AB-4); tech tiers as tag gates (AB-5); LOOP_V2's
   abilities written in the model (AB-6); combos and counters as data (AB-7); ABL-PREDICT left open, both answers
   carried (AB-9); the step's cost mark and a proposed curve (AB-10). *Need:* VISION_V2 §6.3, §6.4, §6.5, §6.7; the
   cost mark and the full-pool rules: VISION_V2 §3 and §4 only; AB-2's immunity rule for spawn protection: LOOP_V2 §2
   only.
6. **The upgrade tracks:** the track kind and the drop and pickup values (AB-4); each clause of UPGRADE-SCOPE and who
   carries it (AB-8); weapon and unit definitions name their tracks (AB-4, CD-6). The state, the messages, the
   telemetry fields and the page are ADR 0004 part 18, ADR 0001 part 12, ADR 0006 part 12 and ADR 0003 part 10.
   *Need:* UPGRADE-SCOPE.
7. **The asset pipeline, content side:** an asset decides nothing (AP-1); definitions name assets by id through a
   checked asset list (AP-2); the material-slot classes are one table (AP-3); asset files are hashed and immutable
   (AP-4); per-unit budgets are reviewed data the gate checks (AP-5); the automated step is a tool outside the player
   build (AP-6); a procedural kit spec and an authored asset sit side by side until OQ-0005-AP2 is answered (AP-7).
   *Need:* as each part names it: LOOK_V2 §8; VISION_V2 §4, §6.2, §6.4, §6.7, §6.9; EDITOR_V2 §4.
8. **The asset pipeline, GPU side:** a model carries no look (the GPU section's part 1); slots name a material class
   (part 2); soldiers are skinned and very large units are part groups (part 3); two derived per-unit caps (part 4);
   LOD levels are optional data (part 5); only glTF/GLB (part 6); materials shared, cached and compiled up front (part
   7); the look proof test's plain exports (part 8); trees wait (part 9). *Need:* as each part names it: LOOK_V2 §2,
   §3, §5, §6.4, §8; VISION_V2 §4, §6.4, §6.7, §6.9; EDITOR_V2 §2.
9. **Its spikes:** the content and ability spike; the asset marks inside ADR 0003's look proof test; the asset spike
   after the asset-pipeline research; the Feel Lab test for ABL-PREDICT once abilities are playable ("What a spike
   must prove", below).

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
- Unchecked merges: `rulesMerge` and `envMerge` take any key, `__proto__` included (GW2, GE1; STATUS, SEC-MDL-2), and
  the editor's `RULES_SCHEMA` limits bind only its sliders, never a load (lines 1603–1607).
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
  - The pools' worst case (for ADR 0004 part 1, which gives every sim pool a fixed capacity at match start from the
    mode and the content set, so that the step never grows one and nothing a client sends sizes it): the whole-set
    check reports, for each mode in the set, the worst case of each pool (live projectiles, active effects, weapon
    pickups, big vehicles and their parts), from the content's numbers (fire intervals and projectile lifetimes,
    pellets per shot, effect durations and stack limits, drops per death and pickup lifetimes), the mode's team count
    and team size, and the game-designer's caps where they exist; the report travels with the frozen set. For active
    effects it reports per target, in the two shares AB-1 proposes ("When a pool is full"): the reserved worst case of
    the effects the content bounds on that target (the pick effects of the tracks that can apply to it at once, one
    per track level; its unit's own ability and weapon effects; spawn protection), which every target's rows reserve;
    and, for the effects other soldiers, vehicles and structures apply, whose count play sets, the missing cap. Where
    the content alone sets no bound (those effects, weapon pickups, big vehicles), the report names the pool and the
    missing cap instead of a number, and nothing fills it with a default: "Caps on vehicles and live projectiles per
    team are not in VISION_V2 or LOOP_V2", and until the game-designer sets them every topic's stress marks use the
    stress scene's 15 big vehicles and 300 live projectiles (R2-BUD-OQ5, r2-netcode.md, Open questions, Brief
    question 2, lines 2982–2985); once those caps are set, a set whose worst case exceeds one fails the check (AB-1).
    No research sizes a pool, so each pool's arithmetic is written beside its kind and reviewed like the bounds above
    (proposed); which row gives way when a pool is full is ADR 0004's OQ-0004-11, and AB-1 holds the game-designer's
    proposal per kind for it.
  - Research: R1-DATA-REC2 (r1, lines 2500–2516); R1-RR-OQ2 (lines 2933–2935); the loader's side of R1-RR-OQ3
    (lines 2936–2940; the Worker's restart after a trap is ADR 0004's); R5-RUSTLINT-REC3 (r5, Recommendations, lines
    1340–1352, ADR 0002's); for the pools, R1-ABL-REC6's rule that the step "allocates nothing on the heap in steady
    state" (r1, Recommendations §5, lines 2471–2472; ADR 0004 part 6's) and R2-BUD-OQ5.
  - Need: VISION_V2 §6.1, §6.3, §6.5, §6.7; the pools' report: ADR 0004 parts 1 and 6.
- **CD-3. The schema and content's TypeScript types come from the same Rust types.**
  - What: schemars writes JSON Schema 2020-12 for every content type when the gate regenerates the generated files
    (on `cargo test` or by a tool, never from a shipped build), with ranges and descriptions from the garde rules and
    the doc comments. The editor's inspector and the Feel Lab (CD-14) build their fields from it: minimum and maximum
    become slider limits, the description becomes the tooltip, default and deprecated are shown. The authoritative
    verdict always comes from the Rust validator (CD-2); Ajv is optional, for instant per-field hints only. Content's
    TypeScript types come from ts-rs on the same Rust types, the generator R1-BND-REC4 already names for
    "content-schema types", not from the JSON Schema. Both outputs are committed in `packages/types`, regenerated by
    the gate and diffed (ADR 0002, R5-LAYOUT-REC3). Neither generator's derives reach a shipped build (`server`,
    `sim-wasm`): schemars and ts-rs are dev-dependencies, their derives behind `cfg_attr(test, …)` or behind a feature
    that only dev-dependency entries turn on, as ADR 0002 part 5 sets ("Dev-dependencies do not activate features
    unless building a Cargo target that needs them": R5-LAYOUT-F4, r5 lines 83–91), so the `content` crate the shipped
    builds compare and measure is the shipped one (proposed by rust-reviewer at the step-10 review).
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
  - Weapons and units: a weapon definition names its upgrade track, and progress belongs to the weapon type, not to
    the gun (UPGRADE-SCOPE); whether each weapon definition is its own type or several definitions share one track is
    OQ-0005-AB4, and this section takes no side (OQ-0005-2). A unit definition names its class's own weapons (the
    ability section, AB-4). This section adds no field to either; it holds them in the same format, files and checks as
    every other kind.
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
    are grouped by tuning version; the hash must resolve to an archived content set (the mark below): ADR 0006 part 12
    archives every content set a match record names, by its content hash, in the match records' bucket, uploaded by
    the supervisor beside the first record that names it and kept as long as any record does (proposed there; I-6).
    No research finding stores the sets: R6-TEL-REC2 stores the records (r6-infrastructure.md, Recommendations §4,
    lines 812–827), while R1-DATA-REC6's mark asks that every record's hash resolve to an archived set (r1, lines
    2576–2579). The bundle, like every build file, stays under the client host's
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
  - What: a faction is a definition (AB-4) that names its units and vehicles. The three T1 classes are written once
    and shared by every faction (LOOP_V2 §5); a faction differs through its own definitions and its uniforms. Its
    soldiers are dressed by (faction, class) as data (CC6's note, `docs/LEGACY_INVENTORY.md` line 136), its uniform
    models are assets named by id (AP-2), and its colours reach the assets' faction colour slots (AP-3; the GPU
    section's part 2). Where its colours and model ids sit is OQ-0005-1: proposed, pending it, in a client definition
    keyed by the faction's id, as the feel kit is keyed by the weapon's (CD-14), so the faction's simulation definition
    holds no presentation field. A faction variant is a full definition of its own that references shared definitions
    by id (proposed: no `extends` and no merge, so the merge rules SEC-MDL-2 came from do not return); whether variants
    may override a base definition is OQ-0005-CD5, for the faction design session. Team colours never change
    (VISION_V2 §5); their values are ADR 0003's look tuning.
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
  serde_json, garde and regex enlarge the Worker's Wasm, which has no size limit yet (R1-RR-OQ5, r1, lines 2946–2950;
  the one `sim-wasm` budget is ADR 0002's, part 4), and garde on wasm32 is unread (R1-DATA-OQ5). The content and
  ability spike measures load time and the content crate's share of the size before acceptance (OQ-0005-CD2), and
  option D stays the way out.
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
- **OQ-0005-CD2. What load time may content cost, how much of the Wasm file is content's, and is JSON fast and small
  enough?**
  - Why it matters: the research sets no number for either ("against a budget the ADR sets (no number is sourced
    yet)", r1, lines 2497–2498), and the Worker's Wasm file (`sim-wasm`, which holds the `content` crate) has one size
    budget, ADR 0002's (part 4; R1-RR-OQ5), whose first limit waits for this spike's measurement of content's share.
  - Options: (a) JSON at run time, with a load-time budget set from the spike's measurement and content's size share
    an input to ADR 0002's one `sim-wasm` budget (proposed); (b) a binary runtime format compiled from the same JSON
    (postcard or bincode, not read), only if (a) misses its load-time budget or the size limit ADR 0002 sets.
  - Decider: for load time, the content and ability spike's measurement, then the Advisor sets the budget; for size,
    ADR 0002 part 4: the Advisor sets the first `sim-wasm` limit from the boundary and sim core spikes' measurements and
    this spike's content share, and until then the size is recorded, not gated. What decides it: the measured native
    and Wasm load times of a content set the size of the imported legacy tables plus two maps and two modes, and the
    size serde_json and garde add to the Wasm module. Research ids: R1-DATA-OQ2 (lines 2791–2794).
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
   default depth limit (R1-DATA-OQ4) and runs the same tests with garde on wasm32 (R1-DATA-OQ5). Proposed (reason:
   ADR 0004 part 1 sizes its pools from this report): for each test mode, every pool's reported worst case, and for
   active effects the reserved share per target, equals a count by hand from the test content's numbers, and every
   pool the content leaves without a bound is named as such.
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
   Worker, and the size serde_json and garde add to the Wasm module recorded; the load-time budget is set from these
   numbers, and the size is content's share in ADR 0002's one `sim-wasm` budget (part 4).
10. **Mode files** (R2-ROOM-REC7's mark, r2, lines 2864–2867, ADR 0001's): stated there; this crate supplies the load
    and the rejections.

No item here needs Mustafa's judgement before acceptance; the Feel Lab's feel is his to judge in playtests later.

## The ability and effect system (game-designer)

This section decides how abilities, effects and weapons are written as data, and what the engine runs for them: a
reduced version of Unreal's Gameplay Ability System (GAS), written as data over a closed set of operations in Rust.
Weapons keep their own bookkeeping, and every hit goes through one damage pipeline. The section shows how LOOP_V2's
abilities, the team's tech tiers and the counters ride on that model. It also shows how the model carries Mustafa's
in-match upgrade tracks (UPGRADE-SCOPE) without designing any of them. It designs no upgrade, ability, unit or rule:
those are Mustafa's. Every question his rules leave open is listed for him, with options and no pick.

### Context (this section)

Short citations: r1, r2 and r4 are `docs/research/r1-simulation-core.md`, `r2-netcode.md` and
`r4-physics-navigation.md`; r1:1373–1376 are r1's Findings §5, "Legacy facts used (unit `abilities`)"; lines are at
9e7db28.

**What it serves.**
- VISION_V2 §6.3: "A data-driven ability and effect system, like UE's Gameplay Ability System. Units, vehicles, weapons
  and abilities are built from data, so combos and counters can emerge and tuning needs no code changes."
- VISION_V2 §6.4 (vehicles with their own weapons and abilities: "leech, marking"; seats `[OPEN]`), §6.5 (economy and
  objectives server-side and tunable: the tech gates), §6.6 (balance telemetry: pick and win rates), §6.7 (faction
  content as data).
- VISION_V2 §2, pillar 3 ("every strong thing has an answer") and pillar 4 ("Every choice costs something"). Pillars 1
  and 2 pull against each other on purpose, and their mechanics are `[OPEN]` (VISION_V2 §2, §7).
- LOOP_V2 §4 (class abilities locked until Tech 1, with a minute-5 floor, `[DECIDED]`; the tier ladder `[DEFAULT]`),
  §5 (the T1 classes and their abilities `[DECIDED]`; the counter triangle `[DEFAULT]`; the Javelin `[DECIDED]`, its
  knockback `[DEFAULT]`; the Spy `[DECIDED]` `[LATER]`, its cost and rules `[OPEN]`), §6 (the Juggernaut `[DECIDED]`;
  the Hammer's leech `[DECIDED]` `[LATER]`), §7 (the feel kit, split feedback, flinch and knockback, active reload, the
  AMK's rhythm: all `[DECIDED]`), §8 (the three unlock gates `[DECIDED]`; the catalogue `[LATER]`), and its header: every
  number is a starting value, kept in one data table (LOOP_V2 line 8).

**Settled inputs.**
- UPGRADE-SCOPE (Mustafa, 2026-10-31), his words:
  > - Every unit and every weapon has its own roguelike upgrade track (pick one of three per level), owned by the player.
  > - Tracks survive death and switching (switch back and the track is still there). They reset at the start of every match.
  > - A class always comes with its own weapon: no weapon choice when buying.
  > - Other weapons come only from killed enemies' drops; dropped weapons are world pickups.
  > - Progress belongs to the weapon type, not the physical gun: when a gun is lost, its track waits for the next gun of that type.
  > - In Unreal terms, the tracks live on the PlayerState, not the Pawn.
  >
  > Still open; list them as open questions, don't decide them: carry slots, ammo for looted weapons, and how fast tracks level (including catch-up when switching).
- ABL-PREDICT: open, Mustafa decides after the Feel Lab test. SEC-SPREAD (a): weapon spread stays in an
  [authority-only stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream). MODES-3 (a):
  [the largest mode](../research/GLOSSARY.md#the-largest-mode-and-its-n) is 24v24, N = 48. TICK-30: a 30 Hz server
  tick is the baseline. SLICE-ORDER, condition 2: movement and shooting are input-driven and replayable from day one.
  SPIKE-SCOPE (a): spikes are throwaway.

**The findings it turns on.**
- R1 recommends a reduced GAS subset as data over a closed set of Rust operations (option A2), weapons with their own
  bookkeeping on a shared damage pipeline (option B1), and one table per kind linked by validated ids (option E1)
  (r1-simulation-core.md, Recommendations §5, R1-ABL-REC1, REC2, REC5, lines 2342–2463).
- Every GAS fact R1 uses comes from one community write-up (tranek's GASDocumentation, current with UE 5.3); Epic's
  own documentation and the engine source were not read (r1-simulation-core.md, Findings §5, method note, lines
  1185–1189; R1-ABL-OQ1, lines 2756–2758).
- Weapons: Fortnite "does its own bookkeeping for weapon firing rates"; with cooldowns as effects that the server
  overwrites, Paragon's "Higher latency connections would have a lower ROF with basic attacks" (r1-simulation-core.md,
  Findings §5, R1-ABL-F17, lines 1286–1295). Quake III's weapon timer is an integer counter that carries its overshoot
  into the next shot (R1-ABL-F22, lines 1322–1330).
- Upgrade tracks: the research has no finding on them (no topic file mentions a roguelike track, an offer of three or
  in-match upgrades; its "playerstate" hits are Quake III's player state, r1:1320, r2:199–237). What this section says
  about tracks rests on UPGRADE-SCOPE and on R1-ABL-REC1's vocabulary only. Anything more is an open question or a
  proposed spike mark (plan rule 2).

**What it takes from and gives to the other drafts.**
- ADR 0004 executes it: the closed set of operations and the damage pipeline inside
  [the sim step](../research/GLOSSARY.md#sim-step-time); the one time representation (integer time with carried
  remainders, R1-ABL-REC3); attribute maths (R1-ABL-OQ5); effects as rows or as entities (R1-ABL-OQ10); the 1 ms
  [p99](../research/GLOSSARY.md#time-statistics) part of
  [the sim step share](../research/GLOSSARY.md#the-sim-step-share-and-the-server-only-share) (R1-ABL-REC6). ADR 0004
  also decides the player record that holds the tracks, the random stream that draws the offers of three, and weapon
  drops as world pickup entities.
- ADR 0001 decides [the prediction scope](../research/GLOSSARY.md#the-prediction-scope) (R1-ABL-REC4, ABL-PREDICT),
  who sees which tags (R1-ABL-OQ8), and [the authority](../research/GLOSSARY.md#the-authority)'s checks, the messages
  and the visibility classes for offers, picks, drops and pickups.
- ADR 0006 decides the match record's fields for picks (it owns R6-TEL-REC1). ADR 0003 draws the cues and the HUD,
  offers included.
- This record's content data section decides the file format, the schema, validation, hot reload (R1-DATA) and the
  feel kit's format.

**What the legacy game proves** (LEGACY_INVENTORY.md, by row).
- Weapons and classes are already data: seven weapons in one table, with special-case fields for falloff, burst, melee
  and charge (CD12, line 93); four classes with HP 100 / 150 / 120 / 110 and their abilities (CD13, line 94); the
  Juggernaut (CD14, line 95). Their faults are what v2 fixes: bot and colour fields mixed into unit data (CD13), the
  body-part multipliers in two places (CS3, line 171), a missing gun key that silently falls back to the AMK (CC9,
  line 138).
- One damage path with a frontal shield block that skips blasts (CD8, line 89; GC7, line 246); four class abilities
  in code, none of which checks a tech tier (GC11, line 250; GE4, line 290); a purchase status enum (GE13, line 299);
  fire cooldowns that are reset, not carried (GC5, line 244).
- No live in-match weapon progression exists to port: legacy's gun-game ladder (`RIFLE_TIERS`) is dead code, marked
  drop (GE17, line 303).

### Decision (this section's parts)

Abilities, effects and weapons are data over a closed set of Rust operations: a reduced GAS subset. Weapons keep their
own bookkeeping and send every hit through one damage pipeline. Definitions sit in one table per kind, linked by
validated ids, with an upgrade-track kind added for UPGRADE-SCOPE. The team's tech tier gates abilities and purchases as
tags. Mustafa's upgrade tracks ride on the same model: a track is data; the player's progress lives on the player's
record (ADR 0004), not on the soldier; a pick becomes effects or granted abilities on the player's current unit and
weapon.

- **AB-1. The model: a reduced GAS subset, as data, over a closed set of Rust operations (option A2).**
  - What: attributes with a base value and a current value; effects that are instant, timed or permanent, optionally
    periodic; counted hierarchical tags; abilities gated by required and blocked tags, with a cost and a cooldown;
    effects that grant tags or abilities for their lifetime; cues as presentation events only. Content chooses and
    parameterizes operations. It cannot run arbitrary code: every new operation is a reviewed Rust change. The
    aggregation order is fixed and documented per attribute (GAS's default sums its multipliers, a rule designers must
    know). Stacking is an explicit policy per effect, from a short list: no stacking, refresh the duration, add a stack
    up to a limit, strongest only; what a full pool does to a new row sits beside it (the next item). Left out: what
    exists for Unreal's object and replication model (instancing policies, replication modes, ability tasks as a
    framework), and anything no legacy or LOOP_V2 test case needs.
  - When a pool is full (OQ-0004-11: the game-designer proposes per kind, and Mustafa confirms where a player could
    notice; ADR 0004 part 1 fixes each pool's capacity at match start and counts every overflow; CD-2's whole-set
    check reports each pool's worst case, or names the missing cap). Proposed, per kind:
    - Active effects the content bounds on their target (its own picks, its own abilities' and weapon's effects,
      spawn protection): the whole-set check reserves their worst case in every target's rows, so they never give way.
      Reason: their count follows from the content alone, and a pick or a pressed ability that silently fails to apply
      takes away what the player earned (pillar 2).
    - Active effects that other soldiers, vehicles or structures apply (one row per application under "no stacking", so
      play sets their count, not the content): they share the rest of the target's rows, up to a cap in data set once
      AB-10's curve shows what the step can carry; when that share is full, the oldest such row on the target, by start
      tick and then sequence number (ADR 0004 part 4), ends early (OQ-0004-11's option (b)). Reason: refusing the new
      row would make a saturated target immune to every further slow or mark, so cheap buffs stacked by teammates could
      shut out the enemy's answers (pillar 3); ending the oldest keeps every new application landing, at the cost of an
      older effect ending early, which the overflow counter shows to tuning.
    - Live projectiles: the pool holds the content's worst case at the mode's N, so honest play cannot fill it. Per
      shooter that is at most ⌊lifetime ÷ fire interval⌋ + 1 live shots of each projectile weapon it can carry at once
      (carry slots are OQ-0005-AB1's), times the weapon's pellets (at least 1); legacy's Molotov (fuse 1.4 s, r1:1373;
      fire interval 1.05 s and `pellets` 0, LEGACY_TUNING.md, `WEAPONS`, lines 167–203) gives (⌊1.4 ÷ 1.05⌋ + 1) × 1 = 2
      per thrower, and 2 × 48 = 96 at N = 48 (arithmetic). Refusing the new row (option (a)), counted, stays as the
      safety net, and a non-zero count is a content or capacity bug, not a rule a player meets. Reason: a shot that
      never leaves the gun and a projectile that vanishes in flight would both be seen, so the pool is sized never to
      need either; R2-BUD-OQ5's caps, once set, bound the content (a set whose worst case exceeds them fails the check),
      not the play.
    - Weapon pickups: the pickup that has lain longest ends early (option (b)), whatever lifetime OQ-0005-AB9 sets.
      Reason: UPGRADE-SCOPE makes drops the only source of other weapons, so the drop from the fight just won matters
      more than one nobody has taken, and refusing it would leave a kill with no drop.
    - Research: none (no research sizes a pool); R2-BUD-OQ5 (r2-netcode.md, Open questions, Brief question 2, lines
      2982–2985) for the caps; R1-ABL-REC6's "allocates nothing on the heap in steady state" (r1, Recommendations §5,
      lines 2471–2472). Need: VISION_V2 §3 and §4 (a budget, not a feature, as AB-10).
  - Why: §6.3 asks for GAS-like data, and the subset covers every legacy ability and weapon and the primitives §6.4
    names, with no generic-engine feature.
  - Research: R1-ABL-REC1 (r1-simulation-core.md, Recommendations §5, lines 2342–2363), resting on R1-ABL-F1–F11, F16,
    F19.
  - Need: VISION_V2 §6.3, §6.4, §6.5, §6.7.
- **AB-2. One damage pipeline: fixed Rust stages, parameterized by data.**
  - What: the stages are the locational multiplier, range falloff, the frontal-arc block, a target-tag multiplier (for
    counters such as "strong against vehicles") and the owner and team rules. Each hit becomes an instant damage effect
    that passes through them. Beside the owner and team rules sits one rule the research does not list, for spawn
    protection (LOOP_V2 §2 `[DEFAULT]`; AB-6): a hit on a target that holds the immunity tag the pipeline's data names
    deals no damage, a blast included (unlike the frontal-arc block, it reads no arc). It is one reviewed Rust addition,
    like the knockback operation (AB-6). Proposed as a tag, not as a damage scale of 0 on an attribute, so that no
    modifier order (AB-1's aggregation) can undo it, and with the tag named once in the pipeline's data rather than as
    a target-tag multiplier in every weapon (two homes, CS3).
  - Why: the outcome logic GAS gives to execution code becomes fixed, reviewed stages, and each number has one home
    (CS3 shows the cost of two homes).
  - Research: R1-ABL-REC1 (lines 2350–2352), R1-ABL-REC2 (lines 2384–2385); R1-ABL-F9; the immunity rule: none (no
    research finding on spawn protection).
  - Need: VISION_V2 §6.3 ("combos and counters can emerge"); the immunity rule: LOOP_V2 §2 only.
- **AB-3. Weapons keep their own bookkeeping and share the damage pipeline (option B1).**
  - What: a weapon definition holds the sim fields only (fire interval, fire mode, magazine, reserve, pellets, the
    spread fields, range, falloff, damage, reload, charge curve, projectile kind). The weapon's state machine is
    per-soldier sim state: timer, ammo, burst count, charge, reload and the active-reload window. The feel kit is a
    separate client definition keyed by the same id. Abilities may grant or swap weapons (the Banzai blade equips the
    Katana; the Molotov is a thrown weapon) and apply effects to weapon attributes (active reload's short damage
    boost). One weapon model for bots and humans fits this shape; whether v2 keeps it is Mustafa's (R1-ABL-OQ6, a
    design question).
  - Why: with cooldowns as effects that the server overwrites, high-latency players fire more slowly (F17); a shipped
    shooter runs an integer weapon timer inside the player code both sides run (F21, F22).
  - Research: R1-ABL-REC2 (lines 2380–2395).
  - Need: VISION_V2 §6.3 (weapons built from data), §6.4 (vehicle weapons on mounts); active reload itself: LOOP_V2 §7
    only (ADR 0004 executes it).
- **AB-4. One table per kind, linked by validated ids (option E1), with two kinds added for UPGRADE-SCOPE.**
  - What: the kinds are the tag registry; an attribute set per unit kind; effect; ability; weapon (sim); feel kit
    (client only); unit (a class: attributes, weapons, abilities, tier, cost, tags); vehicle (parts, each with HP and
    tags; weapon mounts as fixed points; seats, their count left open by VISION_V2 §6.4 `[OPEN]`); faction. String ids
    resolve to dense handles at load; every reference, tag and number range is checked; a failure names the file and
    the field; nothing defaults silently.
  - Added for UPGRADE-SCOPE: an **upgrade track** kind (what a track can offer at each level, as references to effects
    and abilities), with every unit and weapon definition naming its track; and the **drop and pickup values** that
    ADR 0004's pickup entities read (their values are Mustafa's, OQ-0005-AB9). The unit kind names the class's own
    weapon or weapons, and a purchase gives exactly those ("A class always comes with its own weapon: no weapon choice
    when buying").
  - Not here: the file format, the schema tooling, how definitions sit in files and hot reload are the content data
    section's (R1-DATA; R1-XU-OQ9).
  - Why: the legacy tables already have this shape, and their faults are mixing, duplication and silent fallbacks
    (CD13, CS3, CC9).
  - Research: R1-ABL-REC5 (lines 2448–2459).
  - Need: VISION_V2 §6.3, §6.4, §6.6 (ids key the telemetry), §6.7; the track kind and the drop and pickup values: need
    UPGRADE-SCOPE.
- **AB-5. Tech tiers gate abilities and purchases as tags.**
  - What: the team's tier is team state the sim keeps (ADR 0004). An ability requires its tier's tag, and a unit or
    vehicle is buyable at its tier (its tier and cost are unit and vehicle data, AB-4). The T1 class abilities require
    Tech 1 (LOOP_V2 §4, `[DECIDED]`); T2 and T3 units and vehicles require their tier (the `[DEFAULT]` ladder; LOOP_V2
    §8: "the team's tech tier still decides when a unit or vehicle becomes buyable"). The tier thresholds and the
    minute-5 floor are tuning numbers in data, run by the economy and objective phases (ADR 0004). The authority refuses
    a press or a purchase before its tier; whether the client predicts the press is ABL-PREDICT.
  - What may be bought is data, and ADR 0001's `Buy{unitId}` (part 12) is checked against it: the faction definition
    lists the units and vehicles its team can buy (AB-4's faction kind; R1-ABL-REC5: "which units and vehicles"), the
    mode's rules table may change that per mode (R2-ROOM-REC7), and each unit and vehicle definition holds its tier and
    its price ($100 for each T1 class, LOOP_V2 §5 `[DECIDED]`). The rise in a team's prices after it loses a power plant
    (LOOP_V2 §3 `[DEFAULT]`, run by ADR 0004 part 17) and the buy terminal's reach are numbers in the same tables. The
    Recruit is free and comes with every death (LOOP_V2 §5 `[DEFAULT]`), so its price in data is 0. A purchase gives the
    unit's own weapons and nothing else (AB-8).
  - Why: R1-ABL-REC1 names "team tech tiers" among the tag gates, and legacy checks no tier (GE4, GC11).
  - Research: R1-ABL-REC1 (lines 2343–2345); R1-ABL-REC5 (lines 2448–2452) for the faction's list; R2-ROOM-REC7
    (r2-netcode.md, Recommendations, lines 2857–2867) for the mode's rules table.
  - Need: VISION_V2 §6.5; the faction's list: §6.7.
- **AB-6. LOOP_V2's abilities, written in the model.** Each row uses AB-1 to AB-5; a row that needs a new operation
  says so. The legacy numbers are starting values (LEGACY_TUNING.md; LOOP_V2 line 8), never decisions.

  | Item (LOOP_V2 §, tag) | Written as | Starting values (legacy) | Stays Mustafa's |
  |---|---|---|---|
  | Molotov (§5 `[DECIDED]`; locked until Tech 1) | the Conscript's ability: a thrown weapon (AB-3) whose projectile is swept per tick (R4-PHYS-REC3, ADR 0004) and ends in an area blast through the pipeline (radius, falloff, owner and team rules; a blast skips the frontal block, as in legacy) | fuse 1.4 s, building ×1.8; the shared blast is 140 damage in 6.5 m with linear falloff, owner ×0.5 (r1:1373, r1:1375; GC14); no fire area (GC11) | whether it leaves a burning area: legacy has none and LOOP_V2 is silent; an area that lingers would be a new operation |
  | Riot Shield (§5 `[DECIDED]`; Tech 1) | a toggled ability that applies a permanent effect while it is on: a granted tag and a movement modifier; the pipeline's frontal-arc stage reads the tag | block 70 % within arc cos 0.30, move ×0.72 (LEGACY_TUNING.md, `T`, lines 132–135) | how the block meets a one-hit kill (OQ-0005-AB11) |
  | Banzai blade (§5 `[DECIDED]`: "one-hit kill while active"; Tech 1) | a timed ability: a timed effect that grants the melee weapon and a speed modifier, with a cooldown; the kill is the weapon's damage through the pipeline | Katana for 10 s, 45 s cooldown, speed ×1.3 (r1:1373); Katana damage 9999, arc 120°, impact 0.13 s, swing 0.4 s, range 3.6 m (LEGACY_TUNING.md, `WEAPONS`, lines 167–203) | its answer (OQ-0005-AB11); whether melee is judged at the attacker's view time is ADR 0001's (R2-SYNC-OQ9) |
  | Active reload (§7 `[DECIDED]`: "tap R at the right moment for a faster reload and a short damage boost") | the reload state machine's window (AB-3); a tap inside it shortens the reload and applies a timed effect on the weapon's damage attribute; the authority judges the tap from the command's tick (SLICE-ORDER, condition 2), so near the window's edge the 33.3 ms tick (1000 ÷ 30) decides while the page draws the window every frame, and a tap the bar shows inside can be judged outside: whether the tap carries its moment inside a tick is asked beside shots (OQ-0001-7); the timing is own weapon state, inside the prediction scope; the boost is damage, which only the authority applies | none in legacy | the window, the faster reload and the boost's size and length (R1-ABL-OQ6) |
  | The AMK's rhythm (§7 `[DECIDED]`: "a small magazine, the active reload, or both") | the magazine is a weapon field and active reload is the row above, so each of the three fixes is a data change | magazine 1, reload 3.5 s, interval 0.2 s (`WEAPONS`) | which fix |
  | Javelin (§5 `[DECIDED]`: "strong against vehicles and aircraft, weak against infantry"; knockback `[DEFAULT]`; §7 `[DECIDED]`: "Targets react: flinch and knockback") | a projectile weapon; target-tag multipliers in the pipeline (above 1 against vehicle tags, below 1 against infantry); knockback as one new operation, a velocity change on the target, which the research reads as "a velocity change on the kinematic soldier" (R4-PHYS-REC3, ADR 0004); the flinch is presentation (the feel kit) | none (the Javelin is new; LOOP_V2 §11 leaves it out of the greybox v0) | the numbers |
  | Spawn protection (§2 `[DEFAULT]`: "3 s of spawn protection"; ADR 0004 part 17 applies it at every spawn) | a timed effect on the new soldier that grants the immunity tag AB-2's immunity rule reads; that rule is one new pipeline rule (a reviewed Rust change, like the knockback); the effect's length is data, and its rows are among those the content bounds (AB-1, full pools) | 3 s (LOOP_V2 §2), 90 ticks at 30 Hz (3 × 30, arithmetic); none in legacy, whose respawn waits 2.0 s (LEGACY_INVENTORY.md, CD4, line 85) | what it covers beyond damage, and what ends it early (OQ-0005-AB14) |
  | The Hammer's leech (§6 `[DECIDED]` `[LATER]`) | not built; kept possible by two primitives VISION_V2 §6.4 needs anyway: an effect removed when its source dies, and a trigger on the last hit or tick that moves the victim's item to the source (R1-ABL-REC1 mark 3, lines 2372–2376) | none | the item and the leech rules |
  | Marking (VISION_V2 §6.4) | an effect that grants a tag (the research's reading, R1-ABL-REC1, lines 2359–2361); who sees the tag is ADR 0001's interest management (R1-ABL-OQ8) | none | what marking does (R1-ABL-OQ6) |
  | The Spy's disguise and hack (§5 `[DECIDED]` `[LATER]`; cost, what breaks the disguise, hack length, the amount stolen and the warnings `[OPEN]`) | not built; kept possible: the disguise as a tag-granting effect whose shown identity differs per team (an enemy client receives the disguise as the identity it sees: R2-INT-REC5, ADR 0001); the hack as a timed ability with cancel tags; the cost as an instant effect on a named attribute, so the player's money and the team's tech both stay possible | none | all of it (R2-INT-OQ8, a design question) |
  | Legacy Bayonet, Torque Bow, health regen (not in LOOP_V2) | the Bayonet and the Torque Bow are test cases only (R1-ABL-REC1 mark 1); regen would be a periodic effect, "a candidate periodic effect" in the research (r1:1380) | Bayonet 1.3 s charge, kills within 2.4 m, 10 s cooldown (r1:1373); Torque Bow fuse 0.55 s (r1:1376); regen after 3.5 s at 16 HP/s (LEGACY_TUNING.md, `T`, lines 126–127) | whether each stays (R1-ABL-OQ6) |

  - Research: R1-ABL-REC1, REC2; R4-PHYS-REC3 (r4-physics-navigation.md, Recommendations, lines 747–751).
  - Need: VISION_V2 §6.3, §6.4; the rows' rules: LOOP_V2 §2 (spawn protection) and §4–§7.
- **AB-7. Combos and counters as data (pillar 3).** Any effect, tag or weapon can be put on any unit or vehicle by data
  alone (R1-ABL-REC1 mark 2, lines 2369–2371). The ids of AB-4 key the telemetry: every kill names its source weapon or
  ability id (R6-TEL-REC1, ADR 0006's), so counters get tuned on evidence (VISION_V2 §6.6). The counters LOOP_V2 states,
  and how the data carries them:

  | Strong thing | Its answer (LOOP_V2) | Carried by | Legacy numbers (starting values) |
  |---|---|---|---|
  | Imperial Warrior (SMG, strong up close) | the Peacekeeper (the triangle, §5 `[DEFAULT]`) | weapon numbers; the Riot Shield's frontal-arc stage | the Peacekeeper kills a Warrior in 0.85 s only if 11 of its 16 pellets land, else 3.45 s, against the Warrior's 1.30 s, so that side "leans on the Riot Shield", locked until Tech 1 (CD13, line 94); with the Shield up, an SMG hit deals 30 %: 63 hits to kill instead of 19 (arithmetic below) |
  | The Banzai blade (Tech 1) | not stated | the frontal-arc stage and target-tag rules can carry any answer | a blocked hit keeps 30 %, so the blade's 9999 still kills through the Shield: 9999 × 0.3 = 2999.7, above 150 HP (shamble-game.js:1514–1515; OQ-0005-AB11) |
  | Conscript (AMK) | the Warrior | weapon numbers | the Warrior kills a Conscript in 0.86 s, 10.5 s the other way (CD13; R1's verifier row R1-ABL-N6, r1:3487) |
  | Peacekeeper (shotgun, Shield) | the Conscript | weapon range; a blast skips the frontal block | the Conscript wins only beyond the Bagpipe's 34 m (CD13); the Molotov's blast is not blocked by the Shield (GC7: the block is "not to blasts") |
  | The Juggernaut and other vehicles | the Javelin, "strong against vehicles and aircraft" (§5 `[DECIDED]`) | a target-tag multiplier (R1-ABL-REC1, line 2351) | none: legacy never reduces the Juggernaut's 1500 HP (GC17, line 256) |
  | The Javelin | infantry ("weak against infantry", §5) | a target-tag multiplier below 1 | none |
  | The Hammer's leech, the Spy (`[LATER]`) | not designed | primitives kept possible (AB-6) | none |
  | A player with many picks (UPGRADE-SCOPE) | not designed: track content and levelling are Mustafa's (OQ-0005-AB3, AB7) | picks are effects, so any answer is data too | none |

  - The arithmetic (legacy table values: SMG damage 8, Shield block 0.7, Peacekeeper HP 150, Katana damage 9999):
    a blocked SMG hit deals 8 × (1 − 0.7) = 2.4, so ceil(150 ÷ 2.4) = ceil(62.5) = 63 hits; unblocked, ceil(150 ÷ 8) =
    ceil(18.75) = 19 hits; a blocked blade hit deals 9999 × (1 − 0.7) = 2999.7. These are sums on starting values, not
    balance evidence: CD13 itself asks for sims of each matchup (3 or more runs) before the values are kept.
  - Research: R1-ABL-REC1 (lines 2350–2352, 2369–2371), R1-ABL-REC5 (line 2459).
  - Need: VISION_V2 §6.3, §6.6; pillar 3 (VISION_V2 §2).
- **AB-8. Mustafa's upgrade tracks ride on the model (UPGRADE-SCOPE).** Each clause of his words, and who carries it.
  The rules, the track content and every number are his; this section adds only the track kind (AB-4) and its load
  checks, and no new operation: a pick is an effect or a granted ability (AB-1).

  | Mustafa's words | Data (this section) | State and execution (ADR 0004 decides) | Authority, messages, visibility (ADR 0001 decides) |
  |---|---|---|---|
  | "Every unit and every weapon has its own roguelike upgrade track (pick one of three per level), owned by the player." | a track per unit type and per weapon type (AB-4), listing the candidates it can offer at each level; a candidate is an effect or a granted ability, with optional requirements (tags, the track's level); the offer size is a data value, 3, so no number is hard-coded | the player's record holds, per track, its level, its picks and its open offers, in a bounded queue whose length is data (ADR 0004 part 18; when offers appear, how they wait and what a full queue does are OQ-0005-AB8's) | the pick command, and the visibility class of offers and track state (R2-INT-REC5's classes: public, team, owner, spatial) |
  | "Tracks survive death and switching (switch back and the track is still there)." | nothing changes in the definitions | the record, not the soldier, holds every track; on each spawn (as a Recruit, or after a purchase) the picks of that unit type's track apply to the new unit | respawns and purchases are commands the authority checks |
  | "They reset at the start of every match." | nothing is saved: tracks need no persistence (accounts are `[LATER]`, VISION_V2 §6.10) | the record starts empty at every match start (with ADR 0001's room lifecycle) | none |
  | "A class always comes with its own weapon: no weapon choice when buying." | the unit definition names its weapon or weapons (AB-4) | a purchase gives exactly those | the purchase names a unit, never a weapon |
  | "Other weapons come only from killed enemies' drops; dropped weapons are world pickups." | the drop and pickup values (AB-4; OQ-0005-AB9) | a drop is a world pickup entity | the check of who may take what; a pickup the authority did not place is refused; the drops' visibility class |
  | "Progress belongs to the weapon type, not the physical gun: when a gun is lost, its track waits for the next gun of that type." | the weapon definition names its track (AB-4) | the picks of a weapon type's track apply to whichever gun of that type the player holds and leave the gun when the player loses it; a world pickup carries none of its last holder's picks; the track keeps its state while no gun of that type is held | none |
  | "In Unreal terms, the tracks live on the PlayerState, not the Pawn." | none | the record is per player slot, beside the soldier tables; whether a player who rejoins after the grace window gets it back, and so whether it is keyed by slot or by player, is OQ-0004-10 | none |
  | "pick one of three" (the offer) | the candidates a track offers at a level, and their requirements | the authority draws the offer (its size from data, 3) from ADR 0004's authority-only offers stream (part 8): a [shared stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream) would let the owner's client foresee its next offers, the reasoning that kept weapon spread in an authority-only stream, SEC-SPREAD (a) | a pick that was not offered, or for an offer that is no longer open, is refused |

  - Telemetry: which pick and track fields the match record holds is ADR 0006's (its record shape, R6-TEL-REC1); they
    are keyed by AB-4's ids.
  - If a pick changes something inside the prediction scope (own movement, own weapon state), the client's predicting
    sim needs its owner's track state; whether and how it gets it is ADR 0001's.
  - Research: none on tracks; R1-ABL-REC1 and REC5 give the vocabulary.
  - Need: UPGRADE-SCOPE (a settled input outside VISION_V2 §6, LOOP_V2, EDITOR_V2 and LOOK_V2), on §6.3's system.
- **AB-9. ABL-PREDICT stays open; the model carries both answers.** If yes, the client predicts its own ability
  activation, cost, cooldown, granted tags and movement changes, with the command's sequence number as the prediction
  key and cues keyed so that a replayed activation plays once (R1-ABL-REC4, lines 2421–2433, ADR 0001's). If no,
  ability presses wait for the server's confirmation. Nothing in AB-1 to AB-8 depends on the answer: the definitions
  are the same either way; only the prediction scope changes (ADR 0001). Need: VISION_V2 §6.3.
- **AB-10. What the ability, effect and weapon step may cost per tick (R1-ABL-REC6, owned by ADR 0004), and one
  proposed addition.**
  - The mark as the research sets it: the ability, effect and weapon step in
    [the stress scene](../research/GLOSSARY.md#the-stress-scene) (N = 48 soldiers, each with up to 8
    [active effects](../research/GLOSSARY.md#active-effect-and-visual-effect), so 48 × 8 = 384) takes at most 1 ms
    per tick at p99, on [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of
    [the reference server](../research/GLOSSARY.md#the-reference-server) and on the minimum laptop in Wasm in a
    Worker, and allocates nothing on the heap in steady state (r1-simulation-core.md, Recommendations §5,
    R1-ABL-REC6, lines 2465–2472; GLOSSARY.md, §3 The stress scene, lines 397–418).
  - At the 30 Hz baseline, 1 ms is 12 % of the sim step share (1 ÷ 8.33 = 0.12); at 60 Hz it would be 24 % of 4.17 ms
    (1 ÷ 4.17 = 0.24). The 1 ms itself is the same at both rates.
  - Proposed addition: picks are long-lived active effects, and how many a soldier holds is not designed yet
    (OQ-0005-AB3). So the spike runs the step at 8, 16 and 32 active effects per soldier (384, 768 and 1,536 at
    N = 48) and reports the count at which the step reaches 1 ms p99. Reason: the stress scene's 8 per soldier
    predates UPGRADE-SCOPE, and the curve tells Mustafa how much room track content has before a number is designed.
  - Need: VISION_V2 §3 and §4 only (a budget, not a feature: R1-ABL-REC6, line 2478).

### Options considered (this section)
Questions C (the time base for durations) and D (what the client predicts) are other drafts': C is ADR 0004's
(R1-ABL-REC3), D is ADR 0001's (R1-ABL-REC4; ABL-PREDICT open). Who owns the tracks and how long they live is settled
by UPGRADE-SCOPE. The research has no option for how tracks are carried, so AB-8 only shows the fit; a separate upgrade
system beside the ability system is not researched and is not proposed.

**Question A: how abilities and effects are expressed** (r1-simulation-core.md, Options §5, Question A, lines 1792–1800).

Option A1: the full GAS model.
| Dimension | Assessment |
|---|---|
| Complexity | High: GAS's write-up needs 11 concept sections (F1–F20), several only for Unreal's object and replication model |
| Cost (money, time, usage) | the highest effort of the five |
| Scalability / performance | unknown (not measured) |
| Security & anti-cheat | execution code is not predicted anyway (F9); otherwise not researched |
| Effect on the art direction / feel | not researched |
| Effort from the current code | legacy has none of it (CD12, CD13, GC11 are tables plus code) |

**Pros:** covers everything GAS does. **Cons:** builds generic-engine features "for later" (CLAUDE.md: REVISE);
over-serves §6.3.

Option A2: a reduced GAS subset as data, over a closed set of Rust operations (proposed: AB-1).
| Dimension | Assessment |
|---|---|
| Complexity | Medium: one generic interpreter over plain data; every new operation a reviewed Rust change |
| Cost (money, time, usage) | medium |
| Scalability / performance | bounded by data size; no script VM; testable for zero allocation; held to 1 ms p99 (AB-10) |
| Security & anti-cheat | content picks and parameterizes operations and cannot add code paths; definitions are validated at load (AB-4) |
| Effect on the art direction / feel | a number change needs no code (R1-ABL-REC1 mark 4) |
| Effort from the current code | the legacy tables convert (R1-ABL-REC5's mark); each operation traces to a legacy or LOOP_V2 test case |

**Pros:** serves §6.3, §6.4, §6.5 and §6.7 directly. **Cons:** a needed mechanic may lack an operation; it then waits
for a Rust change, which is visible and reviewed (the Javelin's knockback is one, AB-6).

Option A3: an embedded scripting language for abilities.
| Dimension | Assessment |
|---|---|
| Complexity | High: a second language and runtime in both the Wasm and the native build |
| Cost (money, time, usage) | not researched |
| Scalability / performance | not researched |
| Security & anti-cheat | the determinism and Wasm support of a script runtime were not researched (R1-ABL-OQ9, line 2781; the options table at line 1798 points to "OQ10", which is the effects-as-entities question) |
| Effect on the art direction / feel | not researched |
| Effort from the current code | no legacy precedent (legacy abilities are code, GC11) |

**Pros:** serves §6.3; Overwatch's Statescript is a shipped example (the talk was not read: R1-ABL-OQ2). **Cons:** adds a
runtime no §6 item asks for.

Option A4: hard-coded Rust per ability, numbers in tables.
| Dimension | Assessment |
|---|---|
| Complexity | Low at first; grows with every ability |
| Cost (money, time, usage) | low at first |
| Scalability / performance | the best of the five |
| Security & anti-cheat | not researched |
| Effect on the art direction / feel | combos and counters need code per pairing, so trying a combo waits for a programmer |
| Effort from the current code | the legacy shape (CD12, CD13, GC11): the least change |

**Pros:** simple and fast. **Cons:** meets "tuning needs no code changes" but not "built from data" (§6.3).

Option A5: adopt Bevy crates (leafwing_abilities, bevy_gauge, bevy_alchemy).
| Dimension | Assessment |
|---|---|
| Complexity | Medium, plus glue |
| Cost (money, time, usage) | not measured |
| Scalability / performance | not measured |
| Security & anti-cheat | not researched |
| Effect on the art direction / feel | leafwing's cooldowns tick by a `Duration` built from `f32` seconds (F23), not on integer ticks (ADR 0004) |
| Effort from the current code | each crate is Bevy-bound and covers one slice (F23–F25); ties this choice to the ECS choice |

**Pros:** existing code. **Cons:** none documents fixed ticks, determinism or prediction (inferred, line 1800).

**Question B: weapons on the effect system or beside it** (Options §5, Question B, lines 1802–1808).

Option B1: weapons keep their own bookkeeping (proposed: AB-3).
| Dimension | Assessment |
|---|---|
| Complexity | one more mechanism to learn |
| Cost (money, time, usage) | low |
| Scalability / performance | keeps the per-shot cost low |
| Security & anti-cheat | the authority decides ammo, cooldowns and reloads (GC4's mark: "the server must decide them") |
| Effect on the art direction / feel | avoids Paragon's lower rate of fire at high latency (F17) |
| Effort from the current code | legacy's player weapon state is already a state machine: switching, swap and fire cooldowns, timed reloads, spread (GC4, line 243) |

**Pros:** Fortnite's and Quake III's shape (F17, F22). **Cons:** a second mechanism beside effects.

Option B2: weapons as abilities, with a cooldown effect per shot.
| Dimension | Assessment |
|---|---|
| Complexity | one mechanism for everything |
| Cost (money, time, usage) | not researched |
| Scalability / performance | an effect instance per shot at up to 13.9 shots per second per soldier (the cost is inferred, not measured) |
| Security & anti-cheat | not researched |
| Effect on the art direction / feel | the predicted-cooldown problem F17 points at: high-latency players fire more slowly |
| Effort from the current code | not researched |

**Pros:** one mechanism. **Cons:** the latency failure Epic's own games show (F17).

**Question E: the shape of the definitions** (Options §5, Question E, lines 1826–1832).

Option E1: one table per kind, linked by validated ids (proposed: AB-4).
| Dimension | Assessment |
|---|---|
| Complexity | low: string ids resolved to dense integer handles at load |
| Cost (money, time, usage) | low |
| Scalability / performance | integer handles at run time |
| Security & anti-cheat | an unknown id is a load error; no silent fallback (CC9) |
| Effect on the art direction / feel | none |
| Effort from the current code | matches the legacy tables (CD12–CD14) |

**Pros:** shared content is written once (the three T1 classes serve every faction, LOOP_V2 §5). **Cons:** harder to
read than E2, which the research calls "simpler to read" (line 1830).

Option E2: nested, self-contained definitions (each unit embeds its weapons and abilities).
| Dimension | Assessment |
|---|---|
| Complexity | simpler to read |
| Cost (money, time, usage) | shared content is duplicated (the three T1 classes are shared by every faction, LOOP_V2 §5) |
| Scalability / performance | not researched |
| Security & anti-cheat | not researched |
| Effect on the art direction / feel | none |
| Effort from the current code | blocks reuse across factions (§6.7) |

**Pros:** one file tells the whole unit. **Cons:** duplication, and no reuse across factions.

### Trade-off analysis (this section)
- **Why A2, B1 and E1:** together they are the smallest model that writes every legacy weapon and ability and every
  LOOP_V2 ability as data (AB-6) and still keeps counters and combos in data (AB-7). A4 is cheaper today but turns every
  combo into code; A1 and A3 add machinery no §6 item asks for; A5 ties abilities to Bevy and to float time. B1 keeps
  the latency problem Paragon had out of the weapons (F17). E1 keeps one definition, and so one track id, for a class
  every faction shares (LOOP_V2 §5).
- **The strongest argument against:** every GAS fact behind AB-1 comes from one community write-up (R1-ABL-OQ1), and
  the closest shipped team-shooter precedent, Overwatch's scripted abilities, was not read (R1-ABL-OQ2). So the case
  against scripting rests on cost and on the research's own judgement that A3 "adds a runtime no §6 item asks for"
  (r1-simulation-core.md, Options §5, Question A, line 1798), not on proof that a closed set is enough for a live game.
  And a closed set means each new mechanic Mustafa designs that lacks an operation waits for a reviewed
  Rust change. The content and ability spike's marks 1 and 2 test exactly this; OQ-0005-AB13 names the trigger.
- **Pillars 1 and 2, both sides:** the model takes neither side. Tracks that survive death let a player's power grow
  through a match (pillar 2, power that feels earned); how fast tracks level, and whether a switched-to track catches
  up, decide whether a trailing player keeps a real way back (pillar 1). No track numbers exist, so neither side can be
  measured yet; OQ-0005-AB3 names the two measurements.
- **Two teams and three teams:** everything here is data, so a mode can carry its own values (a room is a mode id plus
  allowed overrides: R2-ROOM-REC7, r2-netcode.md, Recommendations, lines 2857–2867, ADR 0001's). Balance is checked
  per mode, separately (VISION_V2 §3: "Two-team and three-team battles are both tested").

### Open questions (this section)
The first three are the ones Mustafa named; OQ-0005-AB4 to AB11 and AB15 are what his rules and LOOP_V2 leave open
beside each other, and OQ-0005-AB14 is what LOOP_V2 §2's spawn protection leaves open. None of AB1 to AB11, AB14 or
AB15 has a pick here: they are Mustafa's (for the upgrade questions, his own words: "list them as open questions, don't
decide them"), and the data model carries every option listed, except where an option says it needs a reviewed Rust
addition. Which row gives way when a pool is full is ADR 0004's OQ-0004-11, with the game-designer's proposal per kind
in AB-1 for Mustafa to confirm.

- **OQ-0005-AB1. Carry slots: how many weapons can a player carry besides the class weapon, and what happens to the
  one in hand when they take a pickup?**
  - Why it matters: it decides whether loot replaces or adds, and how many weapon tracks act at once (each held gun's
    picks apply, AB-8).
  - Options: (a) one weapon: a pickup replaces the class weapon, which drops as a pickup; simplest to read and balance,
    but the class's identity in the triangle (LOOP_V2 §5) can be lost mid-life, and one weapon track acts at a time.
    (b) The class weapon plus one looted slot; keeps the class, adds a choice; two weapon tracks act, and the switch
    time is a tuning number. (c) The class weapon plus N looted slots, N as data; the most loot freedom; more tracks act
    at once (more active effects per soldier, AB-10); harder to read from the top-down camera. In every option, a class
    that already comes with two weapons (the Javelin's rockets and Luger sidearm, LOOP_V2 §5) counts both as its own.
  - Decider: Mustafa. What decides it: a design session, then playtests with bots (sims, 3 runs per build).
  - Research ids: none (UPGRADE-SCOPE; the research has no finding on loot).
- **OQ-0005-AB2. Ammo for looted weapons: what does a looted gun come with, and can it be refilled?**
  - Why it matters: it decides how long a looted gun matters against the class's own weapon. LOOP_V2 does not say how
    class weapons refill either; in legacy, normal guns have unlimited reserve by a quirk (GC4, line 243).
  - Options: (a) what its last holder had left, magazine and reserve, never refilled; loot is a short burst, a gun can
    drop empty, and the victim's state matters. (b) A fixed share of a full load (as data), never refilled; predictable
    value, less "what did the victim have". (c) Refilled the same way as class weapons; loot can replace the class for a
    whole life, the strongest pull away from class identity.
  - Decider: Mustafa. What decides it: a design session, together with how class weapons refill. Research ids: none.
- **OQ-0005-AB3. How fast tracks level: what earns a level, how fast, and does a track the player switches to catch
  up?**
  - Why it matters: it sets the snowball inside a match (VISION_V2 §2, pillars 1 and 2, `[OPEN]`; DESIGN_NOTES open
    question 2, line 52: "Snowball control within a match, given that Momentum survives death") and the cost of
    switching.
  - Options for what earns a level: (a) the current unit's and the current weapon's tracks earn from what the player
    does with them (kills, damage, objectives); rewards mastery, makes switching costly. (b) The player earns one pool
    and puts it into any track; switching costs nothing, and a player's power becomes one number. (c) Time in the match,
    the same for everyone; no snowball from skill, the least "earned". (d) Team events (tier-ups, cashouts); ties
    personal power to objective play, and a team shut out of uplinks falls behind twice.
  - Options for catch-up on switching: (i) none: a new track starts at level 0; switching costs the most. (ii) A new
    track starts at a level derived from the player's other tracks (for example a share of the highest). (iii) Lower
    tracks earn faster (a multiplier as data). (ii) and (iii) keep counter-picking (pillar 3) cheap, which the triangle
    needs, and weaken "power that feels earned".
  - Both sides measured, once numbers exist: sims (3 runs per build, two-team and three-team modes separately) and
    match telemetry measure (1) earned power: the time to kill between a fully levelled and a fresh player, both ways,
    per triangle matchup; (2) the way back: how long a fresh player or a trailing team takes to close that gap
    (catch-up speed). Nothing can be computed now: no track content exists.
  - Decider: Mustafa. What decides it: the snowball design session VISION_V2 §2 and §7 call for, then the measurements
    above. Research ids: none (the match record that carries the measurements is ADR 0006's, R6-TEL-REC1).
- **OQ-0005-AB4. What is one track: which definitions share one?**
  - Why it matters: UPGRADE-SCOPE ties progress to "the weapon type". LOOP_V2 §5 gives the same Luger to the Recruit
    and to the Javelin as its sidearm; abilities hand out weapons (the Banzai blade's Katana, the Molotov as a thrown
    weapon, AB-3); vehicles carry mounted weapons (the Juggernaut's cannon, CD14, GC17); LOOP_V2 §8 plans "sidegrade
    variants"; and a vehicle with several seats (`[OPEN]`, VISION_V2 §6.4) would need a rule for whose vehicle track
    applies.
  - Options: (a) every unit, weapon and vehicle definition has its own track; most tracks, and the Luger's progress is
    split by class. (b) Definitions may share a track (the Luger in two classes; the variants of one weapon); progress
    follows the gun the player sees, and needs a rule for which definitions share. (c) Weapons an ability grants for a
    few seconds, and vehicles, have no track; fewer tracks, but against a literal reading of "every weapon". The data
    carries all three: each definition names its track, or none (AB-4).
  - Decider: Mustafa. What decides it: a design session, before track content is written. Research ids: R2-SYNC-OQ7
    (seats, a design question).
- **OQ-0005-AB5. WEAPON-UNLOCKS: how does an account-unlocked weapon reach a match, when buying offers no weapon choice
  and other weapons come only from drops?**
  - Why it matters: LOOP_V2 §8's third gate ("In a match you buy it each life with match money, at a low price") and
    VISION_V2 §1's "Finally unlocked that weapon. Can't wait to use it against them." sit beside UPGRADE-SCOPE's "no
    weapon choice when buying" and "Other weapons come only from killed enemies' drops".
  - Options: (a) a weapon unlock is a class variant that comes with that weapon, bought each life (LOOP_V2 §8:
    "sidegrade variants"); keeps both rules, and the catalogue grows in classes. (b) An unlocked weapon reaches a match
    only as loot; keeps buying choice-free, but needs drops beyond the victim's own weapons, which UPGRADE-SCOPE does not
    describe, and an unlocker may never hold it. (c) Weapons leave the unlock catalogue, and VISION_V2 §1's line is read
    as unlocking a class; narrows pillar 7. (d) An account loadout sets a class's weapon before the match, not when
    buying; keeps the buy step choice-free, but adds a weapon choice UPGRADE-SCOPE's words do not cover.
  - Nothing is built now: the catalogue is `[LATER]` (LOOP_V2 §8) and accounts are `[LATER]` (VISION_V2 §6.10). The
    data model rules none out (a unit definition names its weapons, AB-4). ADR 0006 builds no account store (accounts
    are `[LATER]` there, its legacy row GE15 included), so the unlock catalogue's weapon question is asked only here.
  - Decider: Mustafa, with the Advisor (LOOP_V2 §8 is theirs). What decides it: the unlock catalogue's design session,
    before any weapon unlock is built. Research ids: none.
- **OQ-0005-AB6. How do the player's tracks sit beside the team's tech tiers?**
  - Why it matters: LOOP_V2 §12 and DESIGN_NOTES (lines 19–20) call the team's tiers the answer to in-match
    progression ("Momentum"); UPGRADE-SCOPE adds per-player tracks beside them.
  - Options: (a) independent: two progressions for the player to read. (b) Tiers gate track levels or candidates (a
    candidate can require a tier tag, AB-5); the tier stays the big beat and tracks grow inside it; a shut-out team's
    players level within lower tiers. (c) Tier-ups feed track levels; objective play feeds personal power (pillar 4),
    and a team shut out of uplinks falls behind twice (pillar 1).
  - Decider: Mustafa. What decides it: a design session, with OQ-0005-AB3. Research ids: none.
- **OQ-0005-AB7. May a pick add raw power (more damage, more health), or only change how a unit or weapon plays?**
  - Why it matters: Mustafa decided that tiers are "sophistication, not damage" (DESIGN_NOTES, lines 27–28) and that
    unlocks "give breadth, never raw power" (LOOP_V2 §8); the Advisor's suggestion, not decided, was to bias any deck
    "toward sidegrades over raw power" (DESIGN_NOTES, lines 40–41). None of these lines names in-match picks.
  - Options: (a) sidegrades only; protects the triangle and new players (LOOP_V2 §8: every counter role from level 1),
    with a weaker sense of earned power. (b) Raw power in small steps, with a cap per track as data (a stacking limit,
    AB-1); a strong pillar 2, a risk to pillar 1 and to counters. (c) Mixed, chosen per track; flexible, the hardest to
    balance.
  - Decider: Mustafa. What decides it: a design session, then the two measurements of OQ-0005-AB3; who may see a
    player's picks is OQ-0005-AB15, decided with this question, since raw power that nobody can see has no answer.
    Research ids: none.
- **OQ-0005-AB8. The offer: when does it appear, and when can the player answer it?**
  - Why it matters: a live fight has no pause. The open offers are state on the player's record (ADR 0004 part 18: a
    bounded queue per track, its length and the offer's size both data), the answer is a command the authority checks
    (ADR 0001), and ADR 0003 draws it.
  - Options: (a) at once, in play, answerable any time (unanswered offers wait, in a queue whose length is data); the
    fastest power, a distraction mid-fight. (b) Only at the buy terminal, where everyone spawns (LOOP_V2 §2); ties picks
    to a visit to the barracks. (c) While dead or respawning; gives the respawn wait a use, and picks lag behind levels.
    With each: are the candidates always different, and what happens when a track has fewer left than the offer's size
    (3). Also with each, what a full queue does when the track levels again: (i) the level is kept and its offer drawn
    once a pick frees a place, so nothing earned is lost and the queue's length only limits what is shown; (ii) the
    oldest open offer gives way to the new one, so the player loses a choice they earned (pillar 2); (iii) the track
    stops earning until a pick, a push to pick and lost progress for a player who cannot stop. Under (b) and (c) the
    queue fills fastest, since offers wait for a visit or a death.
  - Decider: Mustafa. What decides it: a design session, then the Feel Lab or playtests. Research ids: none.
- **OQ-0005-AB9. Which drop and pickup rules does UPGRADE-SCOPE leave unstated?**
  - Why it matters: ADR 0004 builds pickups as sim entities that read these values from data; until they exist, the
    spike uses placeholders. This is the one place the question is asked: ADR 0004's OQ-0004-5 points here and keeps
    only the mechanism (two claims on one pickup in one tick resolve in its phase's fixed, deterministic order).
  - Options for which deaths drop: (i) any death; (ii) only a death to an enemy (his words name "killed enemies' drops"
    only). Options for what a dead soldier drops: (a) its class weapon; (b) the gun in its hands; (c) every weapon it
    carried. Options for who may take a drop: the killer's team; any enemy of the dead soldier (in three-team modes two
    teams, not one); or anyone. Also Mustafa's, each a value in data: how long a pickup stays; whether a gun a player
    swaps out becomes a pickup that anyone can take; which of two claims on one pickup in the same tick wins (ADR 0004
    keeps the order deterministic whichever is chosen); and which act a press means where a pickup lies within reach of
    an uplink or a cache, since a steal is holding E (LOOP_V2 §4 step 4), picking up and placing a cache (steps 2 and 3)
    name no key yet, and a pickup is asked for through the same button edges: the pickup wins, the steal or the cache's
    act wins, or a tap takes the pickup and a hold steals. A full pickup pool follows AB-1's proposal for OQ-0004-11.
  - Decider: Mustafa, with the game-designer. What decides it: a design session, with OQ-0005-AB1; the press precedence
    in the uplinks change-set's playtests, where fights cluster at uplinks and drops lie there. Research ids: none;
    whether a pickup is predicted is ADR 0001's (Quake III predicts item pickups: R2-SYNC-F17, r2-netcode.md, Findings,
    lines 230–236).
- **OQ-0005-AB10. Do bots have tracks and make picks, and what does a human get when taking a bot's slot?**
  - Why it matters: bots fill empty slots (LOOP_V2 §2 `[DECIDED]`; VISION_V2 §3), and a joining human takes a bot's
    slot (R2-ROOM-REC6, r2-netcode.md, Recommendations, lines 2842–2846, ADR 0001's); tracks are owned by the player.
  - Options: (a) bots level and pick like humans, and a human taking the slot inherits its tracks; seamless for the
    team, but the joiner holds picks they did not choose. (b) Bots level and pick, and a human taking the slot starts
    empty; fair to the joiner, and the team loses the bot's picks at once. (c) Bots have no tracks; simplest, and teams
    with more bots are weaker, which matters in three-team modes with uneven bot counts. How a bot picks is bot
    behaviour (ADR 0004).
  - Asked beside it in ADR 0004, both Mustafa's: whether a player who rejoins after ADR 0001's grace window gets their
    record back, tracks and money included, or the slot's bot keeps it (OQ-0004-10, decided with this question; the
    record's key, slot or player, follows the answer); and whether bots take world pickups at all (OQ-0004-12), which
    changes what OQ-0005-AB9's drop rules do to balance when most slots are bots.
  - Decider: Mustafa. What decides it: a design session, with OQ-0004-10 and OQ-0004-12, then sims with mixed humans
    and bots (3 runs per build, two-team and three-team modes separately). Research ids: none.
- **OQ-0005-AB11. Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong
  enough before Tech 1?**
  - Why it matters: pillar 3 ("every strong thing has an answer") and the triangle (LOOP_V2 §5, `[DEFAULT]`: the
    Peacekeeper beats the Warrior). On legacy numbers, before Tech 1 the Peacekeeper wins only if 11 of 16 pellets land
    (CD13); at Tech 1 the Shield cuts SMG hits to 30 % (63 hits instead of 19), but a blocked blade hit still deals
    2999.7 against 150 HP (AB-7).
  - Options: (a) the frontal block treats the blade like any hit (legacy: the blade kills through it). (b) The Shield
    stops a one-hit kill from the front (a target-tag rule in the pipeline). (c) The Peacekeeper's side is tuned by
    numbers alone (pellets, range, HP), the Shield aside. The data carries all three.
  - Decider: Mustafa. What decides it: sims of each matchup at Tech 0 and at Tech 1 (3 or more runs, as CD13 asks),
    then the Feel Lab. Research ids: R1-ABL-OQ6 (a design question); R2-SYNC-OQ9 (melee timing, ADR 0001's).
- **OQ-0005-AB12. Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation
  first?**
  - Why it matters: every GAS concept AB-1 borrows comes from one community write-up (R1-ABL-OQ1, lines 2756–2758).
  - Options: (a) accept it: the subset is defined by our own test cases (legacy and LOOP_V2) and proven by the content
    and ability spike, so no part rests on GAS internals beyond the concept list. (b) Allow Epic's documentation domain
    for one verifier pass before acceptance (SRC-ACCESS (a): official-docs domains such as Epic's can be added in the
    next kit PR).
  - Proposed: (a), with (b) as a cheap check if the kit adds the domain. Decider: the Advisor. What decides it: whether
    the next kit PR adds Epic's domain. Research ids: R1-ABL-OQ1.
- **OQ-0005-AB13. When does scripting (option A3) come back?**
  - Why it matters: A3 is set aside on cost and need; the shipped precedent (Overwatch's "Networking Scripted Weapons
    and Abilities", GDC 2017) was not read (R1-ABL-OQ2, lines 2759–2762).
  - Options: (a) only if the content and ability spike's mark 1 or 2 fails (a test ability needs ability-specific Rust
    code, or a combination needs a Rust change), or if new operations turn out to be needed often; (b) read the talk
    first, when a copy turns up (SRC-ACCESS (a)).
  - Proposed: (a), and (b) whenever a copy appears. Decider: the Advisor (the source). What decides it: the content and
    ability spike's marks 1 and 2 (the trigger). Research ids: R1-ABL-OQ2, R1-ABL-OQ9.
- **OQ-0005-AB14. What does spawn protection cover, and what ends it early?**
  - Why it matters: LOOP_V2 §2 (`[DEFAULT]`) gives "3 s of spawn protection" and no more. AB-2's immunity rule stops
    damage for the effect's whole length; whether it also stops other effects, and whether firing ends it, decides
    whether a fresh spawn is merely safe or can fight with no risk. On legacy numbers a Warrior empties its 36-round SMG
    magazine in 35 × 0.072 = 2.52 s, inside the 3 s, for up to 36 × 8 = 288 damage, nearly two Peacekeepers' worth
    (2 × 150 = 300) (LEGACY_TUNING.md, `WEAPONS` and `UNIT_TYPES`, lines 167–230; arithmetic), and nothing can answer
    it while it lasts (pillar 3). Barracks interiors are closed to enemies (LOOP_V2 §2), so this plays out at the
    openings and against blasts thrown in.
  - Options for what it covers: (a) damage only (AB-2's rule as written); (b) damage and every effect another team
    applies (slows, marks, knockback), which needs one more reviewed Rust check: an effect's application refused on a
    target holding the tag. Options for what ends it early: (i) nothing, the full 3 s; (ii) the player's first shot or
    ability press, which needs a trigger that removes the effect on the soldier's own fire, of the kind R1-ABL-REC1's
    examples name ("on kill by source, run effect E", line 2347); (iii) leaving the barracks interior, read from the
    interior's flag (ADR 0004 parts 11 and 17). The length stays data with every option.
  - Decider: Mustafa (the 3 s is the Advisor's `[DEFAULT]`; what it covers and what ends it are new rules). What
    decides it: a design session, then sims (3 runs per build, two-team and three-team modes separately) counting
    deaths before a soldier leaves its barracks and the damage protected soldiers deal, then playtests. Research ids:
    none (no research finding on spawn protection).
- **OQ-0005-AB15. Who sees a player's upgrade picks?** (GD-PICK-VIS)
  - Why it matters: ADR 0001 part 12 sends track state and picks to their owner only, as its starting class. Teammates
    plan around each other's power (pillar 4: "Teams must split up and talk"); winners should "feel strong and can see
    it" (pillar 2); and enemies need to read a strong player to answer it (pillar 3, "every strong thing has an
    answer").
  - Options: (a) the owner only (ADR 0001 today); (b) the team too; (c) everyone, through the scoreboard or a visible
    cue on the soldier. Each is one of R2-INT-REC5's visibility classes (r2-netcode.md, Recommendations, line 2502;
    ADR 0001's), so the answer changes a class in ADR 0001's state table, not the data model; a visible cue in (c) is
    also ADR 0003's to draw and LOOK_V2's to style.
  - Decider: Mustafa. What decides it: a design session, with OQ-0005-AB7 (if picks may add raw power, the answer
    pillar 3 needs is easier to give when enemies can see them), then playtests. Research ids: none.

### What a spike must prove (this section's part)
All in the content and ability spike (a working name: `docs/SPIKE_PLAN.md` names the spikes and places them). The
test-engineer writes the final pass marks; the candidates below are the research's, or marked proposed with a reason.

1. **AB-1, AB-2 (R1-ABL-REC1's marks, lines 2364–2378; the first mark's proof proposed):** the test content (AMK,
   Bagpipe, Shabzon, Retro Lancer, Beam Katana, Torque Bow, frag grenade, Molotov, Riot Shield, Banzai, Bayonet, the
   Juggernaut cannon) runs with no ability-specific Rust code. The proof is behaviour, not a search: a test-only
   combination (the Shield's block on a vehicle, the Banzai speed buff on another class) behaves as its parts, with
   zero Rust changes, and a scripted scenario over the test content gives the same event log (ids mapped back) with
   every test id renamed and the definitions reordered in the data alone (proposed: a search for the ids, the
   research's measure, misses code keyed by an enum variant or by a numeric handle per ability; it stays as a smell
   check, zero matches for those ids in the sim crate's non-test code). The two §6.4 primitives work in a fixture; one
   number changed in data changes the next match with zero Rust changes.
2. **AB-3 (R1-ABL-REC2's mark, lines 2396–2400):** for each of the seven legacy weapons and the Juggernaut cannon,
   per-shot results equal the legacy tables (body-part multipliers; Shabzon falloff at 10, 15, 25, 35 and 50 m; pellets,
   magazine, reload and burst timing), integers exactly and floats within 1e-6 relative (proposed in the research); one
   weapon model for bots and humans.
3. **AB-4 (R1-ABL-REC5's mark, lines 2460–2463):** the legacy `WEAPONS`, `UNIT_TYPES`, `VEHICLE_TYPES` and ability
   fields convert with every legacy number present once; an unknown id, tag or out-of-range value fails at load with
   the file and field named; 10,000 fuzzed definitions never panic the simulation. Proposed: the fuzzing and the load
   checks include track definitions (a track naming an unknown effect fails at load), because AB-4 adds the kind.
4. **AB-5 (proposed):** an ability press before the team's Tech 1, and a T2 purchase before Tech 2, are refused by the
   authority in 100 % of 1,000 scripted attempts, and accepted after the tier. Reason: LOOP_V2 §4 `[DECIDED]`, and legacy
   checks no tier (GE4); 1,000 is the count R1-ABL-REC4's cue mark uses (line 2444).
5. **AB-6, AB-7 (proposed golden cases):** from data alone, the CD13 matchups reproduce: the Warrior kills a Conscript
   in 0.86 s within one tick (R1-ABL-REC3's mark, lines 2416–2418, ADR 0004's); the Peacekeeper kills a Warrior in
   0.85 s when 11 of 16 pellets land; a Peacekeeper behind the Shield takes 63 SMG body hits. The triangle's third leg
   (the Conscript beats the Peacekeeper, AB-7): a Molotov blast deals a Peacekeeper the same damage with the Shield up
   as with it down (a blast skips the frontal block, GC7), and at 35 m a Bagpipe pellet deals 0 while an AMK body hit
   deals 34 (ranges 34 m and 300 m, AMK damage 34: LEGACY_TUNING.md, `WEAPONS`, lines 173 and 181). The knockback
   operation moves a target with no Javelin-specific code. AB-2's immunity rule: a soldier inside its spawn protection
   takes 0 damage from a body hit and from a blast, and full damage once the effect's 90 ticks end (3 × 30 at 30 Hz),
   with no protection-specific code; this holds under every option of OQ-0005-AB14. Reason: these are the numbers the
   counter check uses (AB-7), and spawn protection (LOOP_V2 §2) is the one pipeline rule the research does not list.
6. **AB-8 (proposed; the research has no mark; reason: UPGRADE-SCOPE), with test-only tracks that are not game
   content, and 1,000 as the count R1-ABL-REC4 uses:**
   - u1: no track-specific Rust code, proven as mark 1 is: u2 to u6 pass on test-only tracks written in data alone,
     and give the same logs with the test track ids renamed and the definitions reordered; zero matches for those ids
     in the sim crate's non-test code is the smell check;
   - u2, death: with picks on unit type U and weapon type W, the unit dies, respawns as a Recruit and buys U again; the
     new unit and its W carry exactly the same pick effects (compared effect lists) in 100 % of 1,000 scripted cycles;
   - u3, switching: U, then V, then U again: U's track is unchanged, V's is its own;
   - u4, a lost gun: the player holds a looted weapon of type X, loses it on death (dropped by the fixture's drop rule,
     a test rule while OQ-0005-AB9 is open), and takes another X the fixture places, and the player's X picks apply to
     it; a second player who takes the first gun gets their own X picks and none of the first player's;
   - u5, reset: at the first tick of a new match, every track is empty;
   - u6, offers: the authority draws an offer of the data's size (3); a pick that was not offered, or for a closed
     offer, is refused in 100 % of 1,000 forged picks; the same seed and the same inputs give byte-identical offer logs
     on two runs, and on native and Wasm: the offers stream is authority-only (ADR 0004 part 8), but in local play the
     authority is the local server in the browser's Web Worker (ADR 0002, Decision).
7. **AB-10:** R1-ABL-REC6's mark (ADR 0004's), measured with the method ADR 0002 sets (R1-XT-OQ15); plus the proposed
   curve at 8, 16 and 32 active effects per soldier (AB-10).
8. **AB-9 (ABL-PREDICT):** no spike before acceptance. The Feel Lab test, once abilities are playable (the spike plan
   places it), compares predicted and server-confirmed Molotov and Shield; Mustafa judges the feel, and the
   test-engineer adds his checklist. Ability feel and, later, upgrade picks are Mustafa's to judge in playtests.

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
  - What: each unit, vehicle, weapon, faction uniform, building part and prop names its model by an asset id. Where
    that id sits is OQ-0005-1: proposed, pending it, in a client definition keyed by the simulation definition's id,
    as the feel kit is (CD-14), so no simulation definition holds an asset id (CD-6); the map document holds its own
    references either way (CD-11). One asset list in the content set (client content, CD-6) holds for each asset: its
    id; its file name with its hash; its material slots, each with its class (AP-3); its LOD levels, if it has any (the
    GPU section's part 5); and its counts of triangles, materials and textures, which the budget check reads (AP-5).
    The loader's whole-set checks (CD-2) refuse a reference to an asset the list does not hold, and a slot whose class
    is not in the class list, naming the file and the field; nothing falls back to a default model. Who writes the
    list (the automated step or a person), and whether the gate recounts a file or trusts its entry, is OQ-0005-AP1.
  - Why: the same verdict for assets as for every other reference; CC9's silent fallback and CL14's unchecked tags
    are the legacy failures, and R1-DATA-REC2's cross-reference stage and R1-ABL-REC5's validated ids already remove
    them for every other kind.
  - Research: R1-DATA-REC1, REC2 (r1, lines 2484–2516); R1-ABL-REC5 (r1, lines 2448–2459).
  - Need: VISION_V2 §6.7 (units, uniforms and variants as data); LOOK_V2 §8.
- **AP-3. The material-slot classes are one content table.**
  - What: the classes the GPU section's part 2 names (for example environment, structure, unit, faction colour,
    emissive), and
    the slot names that map to each, are one client-content table (CD-6), validated like all content and read by ADR
    0003 (interface I-8). Adding a class is a content change reviewed together with the ADR 0003 material it needs. A
    faction's colours reach its faction colour slots from the faction's data (CD-13; where those colours sit is
    OQ-0005-1). The table says which slots exist; how a class looks is ADR 0003's and Mustafa's (LOOK_V2).
  - Why: the GPU section makes the list "this ADR's content data"; one table keeps the asset side and the renderer on
    the same names.
  - Research: R1-DATA-REC1, REC2; the GPU section's part 2 (R3-CHAR-REC3, r3, lines 1065–1075).
  - Need: VISION_V2 §6.7; LOOK_V2 §2 (uniform details; the ink "especially strong on buildings", which the structure
    class carries), §6.1 (the value hierarchy by material class), §6.4 (the glow language's emissive slots).
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
    per pass, 200 draw calls ÷ 4 units ÷ 2 passes, r3, lines 1088–1090), the rest once OQ-0005-AS5 sets them; the GPU
    section's triangle ceiling (about 5.2k triangles per soldier as drawn at Low, its part 4) is a planning input to
    OQ-0005-AS5 and AS6, not a budget the gate checks. The gate compares each
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
    processed files need a home. The repository is public (r5-toolchain-quality.md, Findings, Question 6, legacy
    facts, lines 913–914), so a model file committed to it can be read by anyone once it is pushed (ADR 0006 part 2):
    where the files live also decides who can see unreleased art.
  - Options (not researched): (a) a tool the artist runs when importing a model, which commits the processed file and
    its list entry; (b) a CI job that processes committed exports on every change (ADR 0002 would place it); (c) a
    build step that processes exports at build time and commits nothing processed. With each: whether the gate recounts
    a file or trusts its entry; where Spline source files and raw exports are kept; and whether the files sit in the
    public repository or in a private store the build reads with a secret (ADR 0006's OQ-0006-18 (b), not researched).
  - Decider: the asset-pipeline research, then the asset spike; the files' visibility follows Mustafa's answer to ADR
    0006's OQ-0006-18 (must unreleased art stay unseen?), due before the first authored model is committed, which may
    be one of the look proof test's models (the GPU section's part 8). What decides it: what the research finds
    Spline's export and the tools for step 4 can do, and that answer. Research ids: none.
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

## The asset pipeline: GPU rules (graphics-engineer)

*Written by graphics-engineer for ADR 0005, 2026-10-31; engine-architect places it verbatim. Lines are at 9e7db28;
r3 = `docs/research/r3-rendering.md`. Rendering itself (how the look is drawn) is ADR 0003's; this section says only
what an imported model must carry and what the GPU budget allows it.*

LOOK_V2 §8 gives the flow as a default: a concept sketch, a model in Spline, a glTF/GLB export, "an automated step
that compresses it, builds LODs and renames materials into the faction colour slots", and "the engine applies the
comic look"; budgets per unit (triangles, materials, textures); never Spline's own runtime. This section takes that
flow as given and proposes only what the research supports: R3-CHAR (skinning, instancing, LOD) and R3-BUDGET (the
frame totals). The research has no findings on compression, LOD generation, export checks or slot renaming: its one
glTF mention is three.js's own example, which loads a glTF skinned mesh into the WebGPU renderer (r3, Findings Q4,
R3-CHAR-F8, lines 476–480). Everything else is an open question marked "not researched", grouped under one research
unit, the **asset-pipeline research** (graphics-engineer, verified like R3, in its own daytime session), which
`docs/SPIKE_PLAN.md` places before the asset spike (ASSET-RESEARCH (a), the advisor's ruling of 2026-10-31).

### What this section decides
1. **An imported model carries no look; the engine applies it.** A model brings geometry, its skin (joints and
   weights) where it moves, UVs and textures where it has them, and named material slots. It brings no baked lighting
   and no ink. At load the engine gives each slot a shared node toon material of that slot's material class (ADR
   0003, part 5.a), one material per slot content and never one per mesh. A slot's content is its class (ramp,
   colour gain, ink weight), its faction for faction colour slots, and its own base and emissive colour, textures,
   side and alpha mode, as legacy keys its shared materials by content (CL4, `docs/LEGACY_INVENTORY.md` line 113;
   `_matSig`, `shamble-core.js` lines 356–359); so an asset keeps its own colours and textures inside a class, and
   how colour is authored is OQ-0005-AS4's, not a rule. *Why:* each distinct ramp is a distinct material, so
   materials must be shared by content or batching breaks (r3, Findings Q3, R3-LOOK-F3, lines 331–334);
   legacy converted per mesh and got one material per mesh, which defeated its first merge attempt (CL4,
   `docs/LEGACY_INVENTORY.md` line 113); and a model's own shader code could not run anyway, since custom GLSL and
   `onBeforeCompile` are "not supported in WebGPURenderer" (r3, Findings Q1, R3-WGPU-F6, lines 97–102). *Need:*
   LOOK_V2 §8 (step 5) only.
2. **Material slots name a material class.** Every slot's name maps to one class from a small fixed list (for
   example environment, structure, unit, faction colour, emissive; the structure class gives buildings their heavier
   ink, LOOK_V2 §2, ADR 0003 parts 5.b and 5.h); the list and the names are this ADR's content data (interface
   I-8 in `docs/adr/README.md`), consumed by ADR 0003. Faction colour slots take the faction's colour at load, as one shared material per
   slot and faction, or as per-instance colour where a mesh is instanced (`instanceColor` on
   `InstancedMesh`: r3, Findings Q4, R3-CHAR-F4, lines 452–455; `BatchedMesh` also carries per-instance colour:
   R3-CHAR-F6, lines 463–467; R3-CHAR-REC3, Recommendations Q4, lines
   1065–1075). Emissive slots write the emissive output that drives the glow, so only they glow (r3, Findings Q3,
   R3-LOOK-F12, lines 392–395; LOOK_V2 §6.4). *Need:* VISION_V2 §6.7 (faction uniforms and variants as data);
   LOOK_V2 §2 (uniform details), §6.4.
3. **Soldiers are skinned; very large units are part groups.** A soldier model is one skinned mesh with skin
   indices and weights as attributes ("requires skin indices and weights as buffer attributes in its geometry": r3,
   Findings Q4, R3-CHAR-F1, lines 436–439) and any weights the artist paints (option C-B, R3-CHAR-REC1, lines
   1032–1048). A very large unit's parts are separate meshes under one group, each part named to match the sim's
   vehicle-part records (option V-A, R3-CHAR-REC4, lines 1077–1092; the records are ADR 0004's). *Need:* VISION_V2
   §6.4, §6.9; LOOK_V2 §3, §5.
4. **The derived per-unit caps.** Two caps follow from ADR 0003's totals by arithmetic; the rest are open (OQ-0005-AS5):
   - *A soldier as drawn* (body, headgear and held weapon) uses at most 4 materials per pass: option C-B draws "one
     per material per soldier" (r3, Options Q4, line 825) and ADR 0003 caps a soldier at 4 draw calls per pass
     before the ink (R3-CHAR-REC1, lines 1044–1045). Each material carries one class ramp (part 1), so at most 4
     surface kinds (skin, cloth, faction colour, metal, emissive) shade differently on one soldier, where legacy used
     separate skin, cloth, metal and lacquer ramps (CL2, `docs/LEGACY_INVENTORY.md` line 111); the look proof test's
     soldier is built at this cap.
   - *A very large unit* draws at most 25 parts × materials per pass: ADR 0003 allows the four very large units 200
     draw calls over the shadow and the main pass (R3-CHAR-REC4, lines 1088–1090), and 200 ÷ 4 units ÷ 2 passes = 25.
     R3's own test load, 20 parts each with one material, fits (4 × 20 × 2 = 160, lines 1089–1090).
   - *A triangle ceiling, not a budget:* ADR 0003's harness resets `renderer.info` once per frame (its part 8), so the
     triangle row counts every pass of the frame, the shadow pass included, as the draw-call arithmetic does (assumed:
     how three.js counts the shadow pass's triangles is not researched). At Low, 500k ÷ (48 soldiers × 2 passes) ≈
     5.2k triangles per soldier as drawn, if soldiers had the whole row before the world and the very large units
     take their share (arithmetic on R3-BUDGET-REC2's triangle row, r3 line 1134): a planning input for OQ-0005-AS5
     and AS6.
   - The frame totals every asset shares stay ADR 0003's (r3, R3-BUDGET-REC2, lines 1121–1137): at Low, ≤ 800 draw
     calls (without a per-object outline pass; about 1,100 with one, r3 line 1133), ≤ 500k triangles, ≤ 512 MB of GPU memory; at High, ≤ 1,500, ≤ 1.5M, ≤ 1 GB.
   *Need:* VISION_V2 §4, §6.9; LOOK_V2 §8 (budgets per unit).
5. **LOD levels are optional data; switching is the engine's.** A model may carry mesh LOD levels. The engine
   switches them through three.js's `LOD`, which switches "at the distances specified" with a `hysteresis` "to avoid
   flickering" (r3, Findings Q4, R3-CHAR-F10, lines 490–493), only where the camera's framing spreads units over a
   wide range of distances (R3-CHAR-REC2, lines 1050–1063); the framing is a design question LOOK_V2 does not settle
   (r3, Open questions Q4, R3-CHAR-OQ5, lines 1355–1356). Animation detail is never in the asset: the engine's budget
   LOD decides it (R3-CHAR-REC2). How LOD levels are built is open (OQ-0005-AS2). *Need:* LOOK_V2 §5 (readable from
   a distance, depth up close).
6. **Only glTF/GLB, never Spline's runtime.** The game loads models as glTF/GLB files through three.js's loader on
   both backends; it never loads Spline's own runtime (LOOK_V2 §8). *Need:* LOOK_V2 §8 only.
7. **Materials and textures are shared, cached and compiled up front.** One material per slot content (part 1: the
   class, the faction for faction slots, and the slot's own colours, textures, side and alpha mode) is created once,
   cached by that content and reused, "never rebuilt per edit" (EDITOR_V2 §2, rule 3), and never disposed while
   shared; every one is compiled in ADR 0003's warm-up, so a unit's first appearance creates no shader program (r3,
   Recommendations Q1, R3-WGPU-REC3, lines 928–939). *Need:* VISION_V2 §6.9; EDITOR_V2 §2.
8. **The look proof test's models.** ADR 0003's look proof test needs a few authored models (a soldier, a weapon,
   LOOK_V2 §2's two houses, one of them with an interior and its damage stages, a very large unit, props; ADR 0003
   part 11 lists them and says where the second house comes from if only one is authored). They are plain glTF/GLB
   exports; the test decides nothing about compression or LODs (ASSET-RESEARCH (a)).
9. **Trees wait.** The tree style is LOOK_V2 §8's `[OPEN]` item; tree assets wait for Mustafa's answer (ADR 0003,
   OQ-0003-9). Nothing here decides it.

### Options
"Presentation only" means the option changes nothing the authority decides. Options come from R3's Options section
and the legacy facts it cites; an option the research lacks is marked "not researched".

**How an imported model gets the look**
| Option | Complexity | Cost (money, time, usage) | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort from the current code |
|---|---|---|---|---|---|---|
| **One shared toon material per slot content, applied at load** (proposed, part 1) | Low | one material per distinct slot content (class, faction, the slot's own colours and textures) | materials stay as few as the assets' distinct contents, so sharing and merging work (R3-LOOK-F3) | presentation only | one look on every asset, set in one place; each asset keeps its own colours | new; legacy's per-mesh conversion is the lesson (CL4) |
| Convert each mesh's material at load (legacy's `toonify`) | Low | one material per mesh | defeats batching and merging (CL4) | presentation only | the same look | ports, then fails the draw-call budget |

- A look baked into each asset by the modelling tool: settled by LOOK_V2 §8 (step 5, "the engine applies the comic
  look"); not weighed.
- **Shared per slot content.** Pros: materials bounded by the assets' distinct contents; one place to tune each
  class's look. Cons: every slot must be named into a class (OQ-0005-AS4); an asset with many distinct colours brings
  as many materials, and ADR 0003's static merge groups only equal contents. **Per mesh.** Pros: no naming rule.
  Cons: one material per mesh, legacy's measured problem.

**How a very large unit is built** (r3, Options Q4, lines 834–836)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **V-A. Parts as separate meshes in one group** (proposed, part 3) | Low | more draws than V-B | "a few" units absorb it; parts stay destructible and pickable | presentation only | parts move as the sim's parts | the Juggernaut is already multi-part (GC16, `docs/LEGACY_INVENTORY.md` line 255) |
| V-B. One rigid-skinned mesh per unit | Medium | halves draws | – | presentation only | parts cannot break off on their own | new |

- **V-A.** Pros: maps one-to-one onto the sim's vehicle-part records. Cons: more draws. **V-B.** Pros: fewer draws.
  Cons: loses per-part destruction and picking.

**How faction colour reaches a soldier**
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **One shared material per slot and faction** (with option C-B, proposed) | Low | one material per faction per slot | one draw per material per soldier (Options Q4, line 825) | presentation only | colour per faction, uniform across a team | new |
| Per-instance colour (`instanceColor` on `InstancedMesh`, R3-CHAR-F4; per-instance colour on `BatchedMesh`, R3-CHAR-F6) | Low | none per faction | only with instanced meshes (C-D, C-E: measured against C-B in ADR 0003) | presentation only | the same | new |

- The choice follows ADR 0003's skinning option: C-B uses shared materials per faction; an instanced candidate that
  wins its mark uses per-instance colour.

**LOD** (r3, Options Q4, lines 831–832): L-1, distance LOD from the asset's levels, adopted only where the framing
spreads distances; L-2, the engine's budget LOD for animation, always (ADR 0003, part 6). Compression, the LOD tool,
export checks and the slot-renaming step have no options in the research: they are the open questions below.

### Open questions
All six are **not researched**. AS1 to AS5 form one research unit, the asset-pipeline research (graphics-engineer,
verified like R3, its own daytime session), which `docs/SPIKE_PLAN.md` places before the asset spike; the asset spike
then proves its answers.

- **OQ-0005-AS1. Which compression formats for textures and meshes?**
  Why it matters: download size and GPU memory against ADR 0003's ≤ 512 MB at Low (r3, R3-BUDGET-REC2, line 1135).
  Options: none researched (no finding in R1–R6). Decider: the asset-pipeline research, then the asset spike. What
  decides it: support on both backends and in the four browsers, decode cost, memory. Research ids: none.
- **OQ-0005-AS2. Which tool builds the LOD levels, and how many levels?**
  Why it matters: LOOK_V2 §5's "readable from a distance, with real depth up close"; part 5 switches levels the asset
  carries. Options: none researched (only animation LOD is researched, R3-CHAR-REC2). Decider: the asset-pipeline
  research, then the asset spike and Mustafa (silhouettes kept). What decides it: the levels keep the silhouette at
  the farthest framing, within the triangle row. Research ids: none.
- **OQ-0005-AS3. Which checks does an export pass before it enters the game?**
  Why it matters: a model with unnamed slots, the wrong scale or a broken skin fails silently on screen, as legacy's
  unchecked tags did (CL14, `docs/LEGACY_INVENTORY.md` line 123). Options: none researched. Decider: the
  asset-pipeline research. What decides it: the failures the asset spike can plant and catch. Research ids: none.
- **OQ-0005-AS4. How are a model's materials renamed into the slot classes and faction colour slots?**
  Why it matters: part 1 and part 2 depend on every slot carrying a class; and since part 1 makes one material per
  slot content, how an asset carries its colours sets how many materials it brings. Options: none researched (a
  manual convention in the modelling tool, or LOOK_V2 §8's automated step; for colour, each slot's material colour,
  vertex colours, or one texture atlas). Decider: the asset-pipeline research. What decides it: whether Spline's
  glTF export keeps material names (not researched), and each colour route's material count against part 4's caps.
  Research ids: none.
- **OQ-0005-AS5. What are the per-unit budgets beyond the two derived caps?**
  Why it matters: LOOK_V2 §8 asks for triangles, materials and textures per unit; only the soldier's 4 materials per
  pass and a very large unit's 25 draws per pass follow from the research (part 4). Options: split ADR 0003's frame
  totals across units, the world and visual effects (not researched). Decider: the asset-pipeline research, then the
  render budget bench with real models. What decides it: the totals still hold in the stress scene's fight on screen
  with the real models. Research ids: none (R3-BUDGET-OQ5, the triangle and memory rows' looseness, is ADR 0003's).
- **OQ-0005-AS6. How many vertices and bones does the authored soldier have?**
  Why it matters: the triangle row and the bone upload in the animation row (≤ 1.0 ms at Low) depend on them.
  Options: none until the model exists ("vertex and bone counts per soldier are unknown until it exists", r3, Open
  questions Q4, R3-CHAR-OQ3, lines 1349–1351). Decider: the character model's author (LOOK_V2 §3, §8), then the render
  budget bench. What decides it: the bench's character marks with the real model. Research ids: R3-CHAR-OQ3.

### What a spike must prove (this section's part)
- **In ADR 0003's look proof test** (plain glTF/GLB exports, no compression or LODs): every test model loads on both
  backends with zero console errors and is drawn with the engine's toon materials per slot, with no material of its
  own left in use; the scene holds no more material objects than the distinct slot contents it shows (part 1;
  proposed mark; reason: sharing by content, R3-LOOK-F3), and two meshes with the same slot content share one
  material object; the test soldier as drawn meets 4 draw calls per pass and
  the test very large unit 25 (part 4's arithmetic, on R3-CHAR-REC1 and R3-CHAR-REC4).
- **In the asset spike**, after the asset-pipeline research: the pipeline's answers to OQ-0005-AS1 to AS5 (its marks
  come from that research and are not set here); the loader on both backends; every slot classed and every faction
  slot coloured; LOD levels that keep the silhouette, judged by Mustafa; and the stress scene's fight on screen built
  from real models within ADR 0003's frame totals.

## Options considered
Each section above keeps its own options: the content data section's Questions 1 to 7, the ability section's
Questions A, B and E, the asset pipeline section's three questions and the GPU section's four. The frame weighs only
the choices that cross sections. Content feeds [the authority](../research/GLOSSARY.md#the-authority), so only an
option that changes nothing the authority decides is marked presentation only.

**Question F1: one content model for gameplay data, abilities and assets, or one per family** (r1-simulation-core.md,
Options §6, Q2, lines 1845–1855; Options §5, Question E, lines 1826–1832; the asset pipeline section's "How content
refers to assets")
| Option | Complexity | Cost (money, time, usage) | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort from the current code |
|---|---|---|---|---|---|---|
| **A. One model: every kind in one content crate, one id space, one whole-set check** (proposed: CD-2, CD-6, AB-4, AP-2, AP-3) | Medium: one crate holds every kind | one loader on four sides | one check per load; the Worker's Wasm carries the whole loader, with no size budget yet (R1-RR-OQ5, r1, lines 2946–2950) | a reference from a unit to an ability, an effect, a track or a model resolves at load, on the server too (R1-DATA-F25) | a missing model or ability fails at load, not on screen | new; replaces CC9's and CL14's unchecked links |
| B. A pipeline per family: ability data in its own loader, models checked by the page when it loads a file | Medium: two or three loaders | each family's own tooling | each loader smaller | a reference across families (a unit's ability, a unit's model) is checked twice or by no one; R1's own warning: "two validators of one schema can disagree" (r1, line 1852) | a missing model shows on screen or falls back (CC9) | the legacy shape |

- **A.** Pros: one verdict and one hash for everything a match runs on. Cons: one crate grows with every kind, and its
  Wasm size is recorded, not gated, until the Advisor sets the budget's first limit from the spikes' measurements
  (ADR 0002 part 4; OQ-0005-CD2). **B.** Pros:
  smaller pieces. Cons: no check sees
  the links between families, which is where legacy failed silently (CC9, CL14). The split per family is not
  researched beyond R1's option B.

**Question F2: where presentation data sits beside a simulation definition** (asset ids, faction colours; R1's legacy
fact CD12: "separate files, or separate sections, for what the simulation decides and what the client only shows",
r1-simulation-core.md, Findings §6, lines 1600–1602)
| Option | Complexity | Cost (money, time, usage) | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort from the current code |
|---|---|---|---|---|---|---|
| a. A presentation section in the simulation definition's own file, typed apart from the simulation part (how AP-2 and CD-13 read before the fix round) | Low: one file per unit | none | – | presentation only, if the sim's types leave the section out | a unit's numbers and its model in one file | close to legacy's mixed tables; CD13's note says "Avoid: bot-only fields mixed into the player class, and a colour (accent) in sim data" (`docs/LEGACY_INVENTORY.md` line 94) |
| **b. A client definition keyed by the simulation definition's id, as the feel kit is (AB-3, CD-14)** (proposed, pending OQ-0005-1) | Low: one more file for each definition with a presentation side | files kept in step | – | presentation only; no simulation type holds a presentation field, so CD-6's split holds by kind | an art or feel change touches only client files | new; the feel kit already works this way |

- Both options apply R1-DATA-REC8's simulation and client groups (r1, lines 2596–2598). The map document holds both
  sides in one document either way (EDITOR_V2 §2, rule 1). Proposed: (b) for definitions, because the feel kit already
  takes it and a weapon's presentation (feel kit, sockets, model) then sits in one place. The question stays open for
  the Advisor as OQ-0005-1; AP-2, AP-3 and CD-13 state (b) as the proposal pending it and decide nothing.

**Question F3: what the content hash covers** (r1-simulation-core.md, Recommendations §6, R1-DATA-REC6, lines
2565–2575)
| Option | Complexity | Cost (money, time, usage) | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort from the current code |
|---|---|---|---|---|---|---|
| **a. One hash over the whole content set: simulation content, client content and the asset list** (proposed, as CD-8 and AP-4 write it; R1-DATA-REC6) | Low | one hash; an art-only change makes a new hash, so clients fetch the new bundle before they join and the match record starts a new tuning group | – | the client and the server agree on every byte, art included | none | new |
| b. A hash of the simulation part for the join check and the match record, beside the bundle's own hash for caching | not researched | – | – | – | – | – |

- **a.** Pros: one number names everything a match used. Cons: pick and win rates split at changes that cannot change
  an outcome (CD-8 and CD-12 group them by the hash). **b.** Not researched, so it cannot be proposed (plan rule 3); it
  is listed as OQ-0005-3.

## Trade-off analysis
- **Why one model:** a unit names its weapons, abilities and track, and its model is named under the unit's id (AP-2;
  where that reference sits is OQ-0005-1); a faction names its units
  and vehicles; an ability names its effects and tags; a track names its candidates. One content crate checks every one
  of those links at load, on the server, in the Worker and in the editor alike (CD-2, AB-4, AP-2), so the silent
  fallbacks and unchecked links legacy shipped (CC9, GE11, CL14) cannot return, and one hash names the numbers every match used (CD-8). The ability model, the upgrade tracks and the asset
  list add kinds to that model rather than machinery beside it: the tracks need no new operation (AB-8), and the asset
  part proposes only what R1-DATA, R3-CHAR and R3-BUDGET support (ASSET-RESEARCH (a)).
- **The strongest argument against:** the joined decision rests on documentation, not on shipped games. No
  shipped-game source was read for the data unit (r1-simulation-core.md, lines 1577–1579); every GAS fact comes from
  one community write-up (R1-ABL-OQ1, r1, lines 2756–2758); and the research has nothing at all on the asset step (the
  asset sections' gap). One crate and one hash also tie three families together: an art change changes the hash every
  match records (OQ-0005-3), and the Worker's Wasm carries the whole loader before its one size budget has a first
  limit (R1-RR-OQ5; ADR 0002 part 4; OQ-0005-CD2). The content and ability spike and the asset spike are where this is
  proven or refuted, and each
  section names its way out (Rollback, below).
- **Tracks on the ability system:** riding on the same model costs long-lived active effects per soldier, which the
  ability, effect and weapon step must absorb; the ability section proposes measuring the curve before any track
  content is designed (AB-10). A separate upgrade system beside the ability system is not researched and is not
  proposed (the ability section's Options).
- **Two teams and three teams:** everything here is data, so each mode can carry its own values (a room is a mode id
  plus allowed overrides, R2-ROOM-REC7, ADR 0001's), and balance is checked per mode (VISION_V2 §3).

## Consequences
- **Easier:** tuning with no code change, and combos and counters tried by data alone (AB-7); one verdict everywhere,
  so a file the editor accepts is a file the server accepts (CD-2); editor and Feel Lab fields built from the schema
  (CD-3); telemetry keyed by definition ids and grouped by content hash (AB-7, CD-8); content work inside the closed set
  of operations without a programmer (AB-1); private-room overrides that cannot reach outside an allow list (CD-9); an
  authored model that cannot change an outcome and cannot ship over its per-unit budget (AP-1, AP-5); one material per
  slot content (its class, its faction for faction slots, and its own colours and textures), never one per mesh, so
  batching survives authored art (the GPU section's parts 1 and 7).
- **Harder:** every mechanic outside the closed set is a reviewed Rust change, knockback the first (AB-6); designers
  must know the aggregation order and the stacking policies (AB-1); upgrade tracks add long-lived active effects per
  soldier, which the 1 ms p99 mark of the ability, effect and weapon step must absorb (AB-10); a Rust type and a
  reviewed length bound for every collection, and each sim pool's worst-case arithmetic beside its kind (CD-2); JSON
  without comments (OQ-0005-CD1); a simulation number tested in a new match, not the current one (CD-7); two
  generators that must agree (CD-3); an asset list kept in step with the files (AP-2); every material slot named into
  a class, and each distinct slot content costing a material of its own (OQ-0005-AS4; the GPU section's part 1); two
  drawing paths for buildings and props until
  OQ-0005-AP2 is answered (AP-7).
- **Revisit when:** content's load time misses its budget, or content's share takes the Wasm file past the limit ADR
  0002 sets (OQ-0005-CD2, option
  D), or garde fails on wasm32 (CD-2); the ability marks 1 or 2 fail (OQ-0005-AB13); track content exists (the stress scene's active effects
  per soldier restated with ADR 0004, AB-10); ABL-PREDICT is answered; Mustafa and the Advisor pick the mid-match swap
  (OQ-0005-CD4); the faction design session meets (OQ-0005-CD5); a shipped-game source on mid-match tuning becomes
  readable (OQ-0005-CD3); a need for map upload appears (CD-9); the Spy or the Hammer's leech leaves `[LATER]`, or
  seats are decided; the asset-pipeline research lands (OQ-0005-AP1, AS1 to AS5); the authored soldier exists
  (OQ-0005-AS6); Mustafa answers the tree style.
- **Hard constraints in CLAUDE.md that change** (for v2 code only, once Mustafa accepts the drafts; spikes are exempt
  now under SPIKE-SCOPE (a); the legacy constraints keep applying to `legacy/`; nothing in CLAUDE.md or `.claude/`
  changes in this pull request):
  - "No external assets (procedural textures, synthesized audio)": stops applying to v2 code under LOOK_V2 §8 ("The
    legacy rule 'no external assets' ends with the legacy engine"). Authored glTF/GLB models enter only through AP-2
    to AP-6 and the GPU section's parts 1 to 7; synthesized audio is ADR 0003's matter.
  - "Animation is cosmetic … Nothing animated decides a hit": carries into v2, and AP-1 applies it to every asset: no
    collider, hit shape, shot origin or part layout is read from a model, and cues stay presentation only (AB-1).
  - "Persistence only through `KVStore`": untouched by this record. Content is not persistence, and the upgrade tracks
    reset every match, so they need none (AB-8); ADR 0001 and ADR 0006 say what changes for the guest secret and the
    server-side stores.
  - The rest are other drafts': classic scripts, the double-click build and layering (ADR 0002); three.js r128,
    `LinearEncoding` output and `stylizeScene()` (ADR 0003).
- **Legacy inventory rows** (R7-MARKS (a); the inventory file is not edited, the change waits for Mustafa's
  acceptance; "pending" is the row's *Look brief pending* note). These are the rows ADR 0003 leaves to this record
  (the building kit, the kit and ground textures and builders, the tree and prop builders and the other asset rows),
  as engine-architect, the author of the content data and asset pipeline sections, marks them, plus ED5, ED7, GW6,
  GW7, GW8, GW10, GW11 and GE7, which no draft had placed:

  | Row id (`LEGACY_INVENTORY.md` line) | Mark now | Proposed mark | Why |
  |---|---|---|---|
  | CM1, CM2, CM4 (149, 150, 152) | redesign (pending) | unchanged: the maps migrate into the map document (CD-4, CD-11); pending becomes "asset families pending (OQ-0005-AP2)" | LOOK_V2 now sets the look; whether the maps' buildings are kit specs or authored assets is open |
  | CM7 (155) | port to TypeScript (pending) | port to TypeScript as kit presets, except the thatched roof, which goes in its current form; pending becomes "OQ-0005-AP2" | LOOK_V2 §3 `[DECIDED]`: "The thatched roof goes in its current form"; §2 keeps the half-timbered house with the brick-tile roof |
  | CM5, CM6, CM8, CM9, CM10, CM12, CL13, CD20, CD21, GW19 (153, 154, 156, 157, 158, 160, 122, 101, 102, 207) | port to TypeScript (pending) | unchanged (the seeded procedural technique, its data in the map document); pending becomes "OQ-0005-AP2: authored through the flow or kept procedural, family by family" | LOOK_V2 §8 allows authored models without requiring them; §2 keeps the architecture direction and the props "as a base to build on" |
  | GW18 (206) | port to TypeScript (pending) | unchanged; pending becomes "the tree style, LOOK_V2 §8 `[OPEN]`" | the tree style is Mustafa's open item |
  | GW20 (208) | redesign (pending) | unchanged (LOOP_V2 §2, §8); pending becomes "OQ-0005-AP2" for the structures' builders | the structures' look follows the families' answer |
  | ED9 (344) | redesign (pending) | unchanged; pending becomes "the mood values are ADR 0003's look-test tuning (LOOK_V2 §6.5)" | the mood lives in the map document (CD-11); LOOK_V2 §6.5 `[DEFAULT]` sets moods per map |
  | ED10 (345) | redesign (pending) | unchanged; the spec lives in the document entity (CD-11); pending becomes "OQ-0005-AP2" | ED10's own note; LOOK_V2 §8 |
  | ED5 (340) | port to TypeScript (pending) | unchanged here; pending becomes "OQ-0005-4: what the lab works on once soldiers are authored models" | the lab drives the procedural character builder, which ADR 0003 proposes to replace with authored, skinned soldiers (its rows CL10, CC1–CC5); EDITOR_V2 keeps the lab (lines 4–5) |
  | ED7 (342) | port to TypeScript (pending) | unchanged here; pending becomes "OQ-0005-4" | EDITOR_V2 keeps the lab (lines 4–5); it tunes the cosmetic animation that ADR 0003's GA1–GA15 rows port, and its own note asks for "a data file the lab writes and the client loads", which client content (CD-6) can hold |
  | GW6, GW8 (194, 196) | redesign (pending) | unchanged: the registries and the placement glue split into the map document (CD-11), the sim's colliders, never read from a model (AP-1), and a per-entity scene on the page (EDITOR_V2 §2 rules 1, 3 and 4); pending becomes "OQ-0005-AP2" for the crates, walls, roads, houses, covers and props, and "the tree style, LOOK_V2 §8 `[OPEN]`" for the trees | LOOK_V2 §8 allows authored models without requiring them; §2 keeps the props ("logs and trees with variety, as a base to build on") and the two houses as `[DECIDED]` directions (LOOK_V2 lines 17, 21–22), and §4 asks for buildings you can enter, but no section says whether any of these pieces is authored or procedural; the tree style is Mustafa's open item |
  | GW7 (195) | port to TypeScript (pending) | unchanged: the platform and wedge builders take the map document's parameters (w, d, h0, h1, rot; CD-11), and the slope that movement, shots and sight use stays the sim's (GW14's row); pending becomes "OQ-0005-AP2 (the ground family)", and until it is answered the greybox reads height through brightness (LOOP_V2 §9) | LOOK_V2 §3 asks for verticality and sets no look for ramps or platforms |
  | GW10 (198) | redesign (pending) | unchanged: spawns become team objectives in the map document (CD-11; EDITOR_V2 §3, "objectives as data"), and spawning inside the barracks is the sim's rule (LOOP_V2 §2; ADR 0004); the pending note is lifted, since the markers are drawn only on the editor's layer, whose scene EDITOR_V2's own architecture plan builds (CD-11) | no LOOK_V2 item names editor-only markers, and ADR 0003 draws nothing of this row |
  | GW11 (199) | redesign (pending) | unchanged (EDITOR_V2 §1–2: per-entity commands instead of a whole-map rebuild); for the environment the pending note is lifted: the mood is map data (CD-11), lit by ADR 0003 (its part 5.g), and its values are tuning in ADR 0003's look proof test; for the terminal mesh, pending becomes "OQ-0005-AP2" | LOOK_V2 §6.5 `[DEFAULT]` sets "different moods per map"; the structures' look follows the families' answer |
  | GE7 (293) | port to TypeScript (pending) | unchanged (the technique carries over to uplinks and harvesters; the silos and the tech box go with LOOP_V2 §4); the pending note is lifted for the glow, since objectives are emissive through emissive slots (AP-3; the GPU section's part 2), and stays for the shapes as "OQ-0005-AP2"; their colours are ADR 0003's tuning | LOOK_V2 §6.4 `[DEFAULT]`: "objectives, faction tech and muzzle flashes are emissive"; §5's iconic silhouettes come with the families' answer |

  The rows these parts rely on keep their marks: CD4, CD6, CD7, CD10, CD12 and CD14, whose dead keys go through the
  import (CD-10); CD8, CD13, GC4, GC5, GC7, GC11, GC14, GC15 and GC17 (rewrite in Rust: the ability model carries
  them); GW2 and GE1 (rules as checked data); GW12, GE11, CM1 and ED13 (CD-4, CD-11); CD2 (KVStore stays a client
  port; content does not pass through it); TL11 (the legacy oracle the import's goldens use); CS3 (drop); CC9 (port to
  TypeScript; its sockets join the feel kit, CD-14); GE13 (redesign); GE17 (drop: its dead gun-game ladder is no
  upgrade track and is not ported). The GPU section's lessons CL4, CL14 and GC16 are ADR 0003's rows, and so are the
  "look brief pending" notes on GC11, GC14, GC15, CC9 and CC12, which concern presentation.

## Open questions
Every open question of the four sections, one line each, with its decider and the section that holds its full text
(the question, why it matters, the options and the research ids), then the frame's own four. Mustafa's questions carry
no pick; where a section proposes an answer, the line says so.

| OQ | The question, in one line | Decider | Where |
|---|---|---|---|
| OQ-0005-CD1 | Do hand-edited content files need comments? | Mustafa, after a trial in the content and ability spike | Content data |
| OQ-0005-CD2 | What load time may content cost, how much of the Wasm file is content's, and is JSON fast and small enough? | load time: the content and ability spike measures, the Advisor sets the budget; size: ADR 0002 part 4's one `sim-wasm` budget, which takes content's share as an input | Content data |
| OQ-0005-CD3 | May production ever change numbers in the middle of a match? (proposed: never) | Mustafa | Content data |
| OQ-0005-CD4 | In development, do simulation numbers apply at the next match or inside a running match? (proposed: the next match) | Mustafa and the Advisor | Content data |
| OQ-0005-CD5 | Faction variants: full definitions that reference shared ones, or overrides of a base? (proposed: full definitions) | the faction design session (Mustafa) | Content data |
| OQ-0005-CD6 | How are moment rules written? | the moment design session, then ADR 0004 | Content data |
| OQ-0005-CD7 | What is the Feel Lab tool? | a later design session with Mustafa; ADR 0003 for its screen | Content data |
| OQ-0005-CD8 | How does a unit's top speed stay true when active effects or upgrade picks add speed? | ADR 0004 with the game-designer | Content data |
| OQ-0005-AB1 | Carry slots: how many weapons besides the class weapon, and what happens to the one in hand at a pickup? | Mustafa | The ability and effect system |
| OQ-0005-AB2 | Ammo for looted weapons: what does a looted gun come with, and can it be refilled? | Mustafa | The ability and effect system |
| OQ-0005-AB3 | How fast tracks level, what earns a level, and does a track the player switches to catch up? | Mustafa | The ability and effect system |
| OQ-0005-AB4 | What is one track: which definitions share one? | Mustafa | The ability and effect system |
| OQ-0005-AB5 | WEAPON-UNLOCKS: how does an account-unlocked weapon reach a match, when buying offers no weapon choice? | Mustafa with the Advisor | The ability and effect system |
| OQ-0005-AB6 | How do the player's tracks sit beside the team's tech tiers? | Mustafa | The ability and effect system |
| OQ-0005-AB7 | May a pick add raw power, or only change how a unit or weapon plays? | Mustafa | The ability and effect system |
| OQ-0005-AB8 | The offer: when does it appear, and when can the player answer it? | Mustafa | The ability and effect system |
| OQ-0005-AB9 | Which drop and pickup rules does UPGRADE-SCOPE leave unstated? (the one home of the drop and pickup questions; ADR 0004's OQ-0004-5 keeps only the tie mechanism) | Mustafa, with the game-designer | The ability and effect system |
| OQ-0005-AB10 | Do bots have tracks and make picks, and what does a human get when taking a bot's slot? (the one home of these questions: ADR 0004's OQ-0004-6 moved here) | Mustafa | The ability and effect system |
| OQ-0005-AB11 | Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1? | Mustafa, after sims of each matchup | The ability and effect system |
| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? (proposed: accept it) | the Advisor | The ability and effect system |
| OQ-0005-AB13 | When does scripting (option A3) come back? (proposed: if the spike's ability mark 1 or 2 fails, or new operations are needed often; the talk is read when a copy appears) | the Advisor | The ability and effect system |
| OQ-0005-AB14 | What does spawn protection cover beyond damage, and what ends it early? | Mustafa | The ability and effect system |
| OQ-0005-AB15 | Who sees a player's upgrade picks? | Mustafa | The ability and effect system |
| OQ-0005-AP1 | Where does the automated step run, what does it write, and where do the files live? | the asset-pipeline research, then the asset spike; the files' visibility follows Mustafa's answer to ADR 0006's OQ-0006-18 | The asset pipeline |
| OQ-0005-AP2 | Which asset families become authored models, and which stay procedural builders? | Mustafa, in the look proof test | The asset pipeline |
| OQ-0005-AP3 | May an AI-generated draft model ever ship? | Mustafa | The asset pipeline |
| OQ-0005-AS1 | Which compression formats for textures and meshes? | the asset-pipeline research, then the asset spike | The asset pipeline: GPU rules |
| OQ-0005-AS2 | Which tool builds the LOD levels, and how many levels? | the asset-pipeline research, then the asset spike and Mustafa | The asset pipeline: GPU rules |
| OQ-0005-AS3 | Which checks does an export pass before it enters the game? | the asset-pipeline research | The asset pipeline: GPU rules |
| OQ-0005-AS4 | How are a model's materials renamed into the slot classes and faction colour slots? | the asset-pipeline research | The asset pipeline: GPU rules |
| OQ-0005-AS5 | What are the per-unit budgets beyond the two derived caps? | the asset-pipeline research, then the render budget bench | The asset pipeline: GPU rules |
| OQ-0005-AS6 | How many vertices and bones does the authored soldier have? | the model's author, then the render budget bench | The asset pipeline: GPU rules |
| OQ-0005-1 | Do presentation references sit on the simulation definitions or in client definitions keyed by the same id? (proposed: client definitions) | the Advisor, at the stop review | this frame |
| OQ-0005-2 | Is a weapon definition the same thing as a weapon type? | Mustafa (through OQ-0005-AB4); open for the Advisor until then | this frame |
| OQ-0005-3 | What does the content hash cover? | the Advisor | this frame |
| OQ-0005-4 | What do the Character Lab and the Anim Lab work on once soldiers are authored models? | Mustafa with the Advisor | this frame |

One candidate for OQ-0005-AS3's list, which the frame adds as an input to that research and decides nothing about: a
very large unit's model names a mesh for every part of its vehicle definition (the GPU section's part 3 asks the names
to match; AB-4 defines the parts), since no load check covers that link today (AP-2 checks asset ids and slot classes
only).

The frame's own questions, in full:

- **OQ-0005-1. Do presentation references sit on the simulation definitions, or in client definitions keyed by the
  same id?**
  - The question: AP-2 has each unit, vehicle, weapon, faction uniform, building part and prop name its model by an
    asset id, and CD-13 gives each faction its colours. CD-6 classes units, vehicles, weapons and factions as
    simulation content, whose step "takes the simulation part by type, so a presentation value cannot reach an
    outcome", and says of the weapon and unit definitions "This section adds no field to either"; the ability section's
    unit kind names no model (AB-4), and its feel kit is "a separate client definition keyed by the same id" (AB-3).
    Before the fix round AP-2 and CD-13 put these references on the simulation definitions; now AP-2, AP-3 and CD-13
    state option (b) as a proposal pending this question and decide nothing.
  - Why it matters: it decides whether CD-6's split holds by kind or only by a rule about fields, and whether a
    weapon's presentation (feel kit, sockets, model) sits in one place or two. The legacy lesson points the same way:
    CD13's note says to avoid "a colour (accent) in sim data" (`docs/LEGACY_INVENTORY.md` line 94).
  - Options: (a) a presentation section in the definition's own file, typed apart from the simulation part; (b) a
    client definition keyed by the simulation definition's id, as the feel kit is (Options considered, Question F2,
    with their trade-offs). The map document keeps both sides in one document either way (EDITOR_V2 §2, rule 1).
  - Proposed: (b) for definitions, for the reasons under Question F2, pending this question.
  - Decider: the Advisor, at the stop review (the game-designer changes AB-4's kinds if the answer needs it). What
    decides it: whether CD-6's split can be checked by type alone. Research ids: none. Both options apply
    R1-DATA-REC8's groups (r1-simulation-core.md, lines 2596–2598) and the legacy fact CD12 (r1-simulation-core.md,
    lines 1600–1602).
- **OQ-0005-2. Is a weapon definition the same thing as a weapon type?**
  - The question: UPGRADE-SCOPE ties progress to "the weapon type", and OQ-0005-AB4 leaves open which definitions
    share a track; its option (b) lets several definitions share one (the Luger in two classes; the variants of one
    weapon). Before the fix round CD-6 read "a weapon definition is a weapon type", which matched only AB4's option (a)
    or (c); CD-6 now says only that a weapon definition names its upgrade track, and points to OQ-0005-AB4.
  - Why it matters: ADR 0004 part 18 keys the player record's tracks by "the type ids ADR 0005 defines"; if one type
    spans several definitions, the key is the track's id, not the weapon's.
  - Options: the answer follows OQ-0005-AB4. Under its (a) or (c), a definition is a type; under (b), a track's id is
    the type.
  - Decider: Mustafa answers OQ-0005-AB4; the question stays open for the Advisor until then. Research ids: none
    (UPGRADE-SCOPE).
- **OQ-0005-3. What does the content hash cover?**
  - Why it matters: one hash covers simulation content, client content and the asset list (CD-6, CD-8, AP-4), and
    pick and win rates are grouped by it ("always grouped by content hash and mode id", because "pick and win rates are
    only comparable within one tuning version": r6-infrastructure.md, Recommendations §4, R6-TEL-REC3, lines 828–836,
    ADR 0006's). A change of art or feel only, which cannot
    change an outcome, then makes a new hash: every client fetches the new bundle before it joins, as with any change,
    and the match record starts a new tuning group although no simulation number changed.
  - Options: (a) one hash over the whole set (proposed, as written; R1-DATA-REC6); (b) a hash of the simulation part
    for the join check and the match record, beside the bundle's own hash for caching (not researched).
  - Decider: the Advisor, with the infra-engineer, whose ADR 0006 owns the analysis (R6-TEL-REC3). What decides it:
    whether the analysis can merge tuning groups whose simulation part is identical, or needs the record to say so.
    Research ids: none (R6-TEL-REC3 is ADR 0006's).
- **OQ-0005-4. What do the Character Lab and the Anim Lab work on once soldiers are authored models?**
  - Why it matters: EDITOR_V2 says "The Character Lab and Anim Lab stay as they are" (lines 4–5). The Character Lab
    drives the procedural character builder (inventory row ED5, line 340), which ADR 0003 proposes to replace with
    authored, skinned soldiers (its rows CL10, CC1–CC5) and AP-7 follows; the Anim Lab (row ED7, line 342) tunes the
    cosmetic animation that ADR 0003's GA1–GA15 rows port.
  - Options (not researched): (a) both labs stay with the legacy game in `legacy/`; (b) the Character Lab is rebuilt
    over the authored soldier and its faction colour slots, and the Anim Lab ports as ED7's note asks; (c) the
    Character Lab's proportion sliders go, since LOOK_V2 §3's proportions are authored, and the Anim Lab ports.
  - Decider: Mustafa with the Advisor (EDITOR_V2 is theirs). What decides it: ADR 0003's look proof test, which shows
    the authored soldier. Research ids: none.

The consistency pass (plan step 8) settled the three points where this record and another draft differed: ADR 0004
part 16 now changes definitions at a match boundary under CD-7 and swaps them between ticks only if OQ-0005-CD4
chooses its option (b), and ADR 0001's `ContentReload` message carries the same condition; OQ-0005-AB9 and AB10 are
the one home of the drop, pickup and bot-record questions, which ADR 0004 cites (its OQ-0004-5 keeps only the tie
mechanism, and OQ-0004-6 moved here); and OQ-0005-AB5 now says why ADR 0006 does not cite it: ADR 0006 builds no account store, so the unlock
catalogue's weapon question is asked only in this record.

## What a spike must prove
Each section's spike part above stands as written. The test-engineer writes the pass marks below (plan step 4);
`docs/SPIKE_PLAN.md` names the spikes and places them. Joined, by spike:
1. **The content and ability spike** (after the sim core spike and before slice 1; decision map, part g): the content
   data section's items 1 to 9 and the ability section's items 1 to 7. Two pairs rest on one mechanism and may share a
   run, as the test-engineer decides: the fuzz runs (content item 2, R1-DATA-REC2; ability item 3, R1-ABL-REC5, with
   track definitions) and the change that takes effect at the next match (content item 5; the last clause of ability
   item 1).
2. **ADR 0003's look proof test** (plain glTF/GLB exports, no compression or LOD step; ASSET-RESEARCH (a)): the GPU
   section's first bullet (the models load on both backends with the engine's materials; no more material objects than
   the distinct slot contents shown; the two derived caps) and the asset pipeline section's first bullet (every model listed
   by id; a dangling id and an unknown slot class fail at load). Mustafa judges each family's look there
   (OQ-0005-AP2), beside legacy screenshots (LOOK_V2 §9).
3. **The asset spike,** after the asset-pipeline research: the GPU section's second bullet and the asset pipeline
   section's second bullet.
4. **In other spikes:** how content files reach the Worker under Vite, in the Rust ↔ TypeScript boundary spike
   (R1-DATA-OQ8; content item 5); own weapon cues played once on a replay, in the netcode proof test (R1-ABL-OQ3; the
   mark is R1-ABL-REC4's, ADR 0001's); the mode files' rejections (content item 10; R2-ROOM-REC7's mark, ADR 0001's);
   the Feel Lab test for ABL-PREDICT once abilities are playable, judged by Mustafa (ability item 8).

The frame adds no mark of its own.

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
  Firefox on Windows 11, with mains power and the power mode recorded. **MAC**, SPIKE_PLAN's macOS machine: the
  stable Safari on the day of the run, its version recorded (WebGPU from Safari 26.0, R3-WGPU-F19, r3 lines 171–174;
  WebTransport from 26.4, R2-TRN-F7, r2 lines 959–975), for the marks that name Safari (real Safari through its
  WebDriver: R5-TEST-REC1's Safari row, r5 line 1394). **MP**,
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
- **Timers.** [Sim step times](../research/GLOSSARY.md#sim-step-time) as ADR 0004 measures them (its tick bench, ADR 0002 part 8, R1-XT-OQ15, lines 3097–3100). A
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
| PM-0005-2 | Every side reads the same numbers (CD-1; content item 1) | 100 % of doubles parse to the same 64-bit pattern in native Rust, in Rust on Wasm and in `JSON.parse` | CI (native, Wasm, `JSON.parse` in Node and the three engines); MAC: Safari (`JSON.parse`) | the corpus: every number in `docs/legacy-tuning.json` (2,038, counted by script at 9e7db28) plus 10,000 generated doubles, 5,000 from uniform random 64-bit patterns with NaN and the infinities left out (JSON cannot carry them) and 5,000 decimals of up to 17 significant digits inside the legacy file's range (proposed: the first half reaches subnormals and extreme exponents, the second the values content holds), plus −0 and the largest and smallest finite doubles; each side writes each value's bits as 16 hex digits; a script compares | 1 run per target (exact). Bites: a parser that rounds through `f32` on one side | R1-DATA-REC1 mark 1 (lines 2494–2496); CD-1 |
| PM-0005-3 | One canonical writer, the same text everywhere (CD-1; content item 1) | 100 % of the PM-0005-2 corpus written back as the same text as its input (the research's mark); the canonical writer's output byte-identical on native and Wasm, and idempotent (write, parse, write gives the same bytes and the same bits); an integer above 2^53 (9,007,199,254,740,993) in a content file refused at load with the file and field named | CI | the writer of CD-1 run over the corpus on every target; byte comparison by script. Recorded: every number whose canonical text differs from `JSON.stringify`'s text for the same double (not researched: the two writers' number formats may differ for integral doubles and exponent forms, which would fail the research's mark as written; if any TypeScript code writes content, it must give the canonical text or write through the Wasm writer) | 1 run per target. Bites: a writer that prints 15 significant digits; a planted 2^53 + 1 in a weapon file | R1-DATA-REC1 mark 1 (lines 2494–2496) and "no finding" (lines 2491–2492); CD-1 ("Content integers stay within the range a double holds exactly") |
| PM-0005-4 | One crate gives one verdict everywhere (CD-2; content item 2; shared with ability item 3) | over C: 0 crashes (no panic natively, no trap in Wasm), 0 accepted invalid files, 100 % of valid control files accepted, and for 100 % of files the same verdict and the same error record (stage, file, line and column or field path, error kind) natively and in Wasm; every error names the file and either line and column (syntax) or a field path (data), and the path named is the path the generator changed in 100 % of data errors; the 50 MB files refused at the byte-size stage and the 10,000-deep files at the depth stage. Recorded: the default depth limit's value (R1-DATA-OQ4) and whether garde builds and runs on wasm32 (R1-DATA-OQ5; if not, CD-2's fallback is built and this row runs on it) | CI | C: 10,000 files from a seeded generator (regenerated byte for byte from its seed: proposed, so a failure reproduces) mutating valid seeds of every content kind (the imported set of PM-0005-14, the two maps of PM-0005-9, two modes, test track definitions); every defect class of the research at least 500 files (proposed), except the 50 MB class, 10 files generated in the run and not committed (proposed: their size adds time, not cases); 1,000 unmutated or validly mutated controls (proposed); a trap in Wasm counts as a crash and the instance is recreated | 1 run per target; joins the gate (20 identical verdicts). Bites: a planted `serde(flatten)` that lets an unknown key through; a planted fallback weapon on a dangling id | R1-DATA-REC2 marks 1–2 (lines 2513–2516); R1-DATA-OQ4 (lines 2799–2800), OQ5 (lines 2801–2803); CD-2 |
| PM-0005-5 | No file sizes an allocation past a reviewed bound (CD-2; content item 2) | for every bounded collection, a file one element over its bound refused (100 %) and a file at exactly its bound accepted (100 %, a control: proposed); 0 collections in the content types without a maximum length; and the whole-set check's pool report (CD-2): for each test mode, every pool's reported worst case, and for active effects the reserved worst case per target (the pick effects of the tracks that can apply at once, one per track level; the unit's own ability and weapon effects; spawn protection), equals a count by hand from the test content's numbers in 100 % of cases, and every pool the content leaves without a bound (effects others apply, weapon pickups, big vehicles) is named with its missing cap, never filled with a default (content item 2) | CI | the bound of every collection read from the generated schema (`maxItems`, `maxProperties`) by script; a file per bound added to C; the schema scan lists any collection with no bound; the hand counts written beside the test content before the run (proposed: a count made after the report could copy it) | 1 run. Bites: a `Vec` field added without a bound; a projectile lifetime doubled in the test content while the report stays the same must fail the comparison; a pool given a default cap where the content sets none | R1-RR-OQ2 (lines 2933–2935); CD-2 (proposed there: "one element over its bound") |
| PM-0005-6 | The loader never panics (CD-2; content item 2) | the panic lints at deny in `content`'s loader: a planted `.unwrap()` and a planted `buf[i]` fail clippy, the same in a `#[test]` pass; cargo-fuzz on the loader: 0 panics and 0 allocations above the fuzz run's allocation limit over 10 million inputs (proposed count: ADR 0001's per-message-type decoder fuzz, R2-TRN-REC4 mark 4) | CI; CI nightly, Linux (fuzz) | the gate's G2 step on the planted lines; a cargo-fuzz target that feeds bytes to the loader's entry; the allocation limit set to the largest reviewed bound times its element size (proposed; the fuzzer's option is not researched) | the lint bite 1 of 1; the fuzz nightly, fresh seeds. Bites: as listed; a planted `.unwrap()` on a parsed id is found within the fuzz run | R5-RUSTLINT-REC3's mark (r5 lines 1351–1352); R5-TEST-REC1's fuzz row (r5 line 1399); R1-RR-OQ3 (lines 2936–2940); R2-TRN-REC4 mark 4 (r2 line 2696) |
| PM-0005-7 | The schema and the validator agree on every range (CD-3; content item 3) | for 100 % of fields with a garde range, the generated schema carries the same minimum and maximum (and the same inclusive or exclusive form); for each such field, a value just outside the range is refused by the Rust validator and flagged by the schema-driven field, and the value at the bound is accepted by both (a control: proposed) | CI | a script pairs each garde rule with its schema keyword; "just outside" is the bound ± 1 for integers and the next representable double beyond the bound for floats (proposed: an off-by-one between inclusive and exclusive forms shows only there); "flagged by the schema-driven field" is a Vitest test of the field's limit logic built from the generated schema (the editor's inspector comes later) | 1 run. Bites: a deliberately changed type (the research's mark); a garde range edited without regenerating | R1-DATA-REC3's mark (lines 2527–2529); CD-3 |
| PM-0005-8 | The generated schema and types never drift (CD-3; content item 3) | regenerating the schema and content's ts-rs types gives 0 changed bytes; `tsc --strict` passes on them; renaming a field without regenerating fails the gate; a hand edit to a generated file fails the gate; 100 % of the legacy import's files, typed against the generated types, pass the Rust validator (the draft's proposed addition) | CI | the gate's G4 step (it shares ADR 0002's PM-0002-15; this row adds the content schema and content's types); the import's files checked by `tsc` against the types and then loaded by the crate | 1 of 1 each. Control: a rename with regeneration passes | R1-DATA-REC3 (lines 2527–2529); R1-BND-REC4 marks 1, 3 (lines 2304–2306); R5-LAYOUT-REC3's mark (r5 lines 1203–1204); CD-3 (proposed there: F16, R1-BND-F9) |
| PM-0005-9 | Every map migrates and round-trips exactly (CD-4; content item 4) | Cabana, DM_Tri and every saved map Mustafa supplies migrate and validate (100 %); migrate → save → load → save byte-identical (100 %); a property test of random valid documents gives the same, 0 failures in 10,000 cases (proposed: two maps exercise few field shapes); a file with a future version refused with a message naming the file, its version and the newest version the loader knows (proposed reading of "a clear message"); each migration step's golden fixture and each validation error's insta snapshot pass | CI | the migration chain on the v1 files; proptest; insta snapshots of the errors, never rewritten in CI | 1 run; the property test 20 identical verdicts. Bites: a migration step changed by one field fails its golden; a future-version file guessed instead of refused | R1-DATA-REC4's mark (lines 2539–2540); R5-TEST-REC3 (r5 lines 1431–1453: goldens, insta for validation errors); CD-4 |
| PM-0005-10 | Development reload: checked at once, simulation numbers at the next match, client content live (CD-7; content item 5; shared with PM-0005-21) | each of 20 saves of a weapon's damage change (the Shabzon's 8 → 9) validated on the development server within 1 s of the save (a plain limit, max over the 20), and the next match runs on it; the running match's per-tick state hashes after the save equal a control run with no save (proposed: CD-7's "nothing in a running match's simulation changes"); each of 20 malformed saves leaves the running match on its numbers and shows the error (file, with line and column or field path) within 1 s; each of 20 saved feel-kit changes shows on the page within 1 s, in the running match; and the same in local play, where slices 1 and 2 have no server crate: each of 20 saves pushed to the Worker (PM-0002-31's route) is validated there within 1 s, the next local match runs on it, and the running local match's per-tick hashes equal a control run with no save (CD-7: "on the development server and in local play alike") | ML: the development server native, and the Worker's local play; the page in Chrome, Edge and Firefox | the development server's watcher (notify with a debouncer); a script saves the files on a fixed schedule; validation time from the file's modification time to the server's "valid" or "error" log line on the same clock; the page's arrival time stamped on the same machine (the file's route to the Worker is the boundary spike's, PM-0002-31) | 20 saves of each kind per browser. Bites: a watcher that validates only on the next match start (fails the 1 s); a Worker that applies the pushed numbers to the running local match (fails the hash comparison); a reload that swaps the numbers inside the running match (fails the hash comparison) | R1-DATA-REC5 marks 1, 4 (lines 2559–2563), restated for CD-7; CD-7 (proposed there: the feel-kit 1 s) |
| PM-0005-11 | Only if OQ-0005-CD4 picks option (b): the mid-match swap stays in step (CD-7) | the client's Wasm sim applies the change at the same tick as the server, with 0 mispredictions beyond the change itself, under P2; 50 random valid tuning edits in a 5-minute bot match give 0 crashes and 0 desyncs (per-tick hashes of the server and a sampled client) | RS (server); ML: Chrome (client); L1 for the edit storm | ADR 0002's simulator at P2; the edits from a seeded script | 3 seeds for the edit storm. Bites: the client swapping one tick late | R1-DATA-REC5 marks 2, 3 (lines 2559–2562); OQ-0005-CD4 |
| PM-0005-12 | One hashed bundle, checked at join and recorded per match (CD-8; content item 6) | 100 % of 3 tampering cases (a stale file, one changed byte, a missing file) refused or re-fetched before the match starts, with 0 matches started on a bundle whose hash differs from the one the room names; a repeat visit loads the bundle with 0 bytes over the network (Resource Timing `transferSize` 0); 100 % of the spike's match records carry a hash that resolves to an archived content set (the archived file re-hashes to it) and the mode id; the Rust SHA-256 crate gives the same hash as WebCrypto on 100 % of the spike's bundles | CI (cases, records, hashes); ML: Chrome (the repeat visit) | a localhost server with immutable headers (the real host is ADR 0006's PM-0006-4); the stand-in join names the hash; WebCrypto in a secure context (localhost); every tampering case 10 times (proposed) | 1 run; the repeat visit 3 times from a fresh profile (ADR 0006's method). Bites: a Worker whose check always passes; the bundle served `no-cache` | R1-DATA-REC6's mark (lines 2576–2579); R1-DATA-OQ9 (lines 2813–2814); CD-8 |
| PM-0005-13 | The server checks every payload a client or the editor sends (CD-9; content item 7) | C sent through the private-room override path: 0 accepted invalid payloads, 0 server crashes; 100 % of valid overrides from the spike's test allow list accepted (a control: proposed); an override naming a tunable outside the list refused (100 %); a payload with a `__proto__` key refused; the editor's document layer passes a test that `Object.prototype` and every content object's prototype are unchanged after loading such a payload | CI (the server path natively; the document layer in Node under Vitest) | the override path called as the room will call it (ADR 0001's `HostCommand`); a test allow list, not the game's (its content is R1-DATA-OQ10, a design question) | 1 run. Bites: an override merged with `Object.assign` in the document layer; the allow-list check removed | R1-DATA-REC7's mark (lines 2591–2594), without the map-upload path (CD-9); R1-DATA-OQ10 (lines 2815–2816) |
| PM-0005-14 | The legacy tuning is imported once, every value accounted for (CD-10; content item 8) | 100 % of the non-look, non-state values appear in the import report as mapped (with their v2 path and their legacy `where`) or dropped (with a reason), counted per leaf (proposed reading of "value": a table mapped as a whole could hide an unmapped key; 725 leaves in the gameplay, simulation and progression groups at 9e7db28, plus every `boot_changes` entry as map data); 100 % of the look group's leaves (2,011) listed, each marked for ADR 0003's look proof test; each dead key (CD4, CD6, CD7, CD10, CD12, CD14) dropped with a reason or kept by decision; the imported set passes PM-0005-4's checks; every kept value equal to legacy's, exactly (bits) | CI | the import tool over `docs/legacy-tuning.json`; a script walks the file's leaves and matches each to a report row; the kept values compared as golden fixtures (R5-TEST-REC3) | 1 run. Bites: a key planted in a copy of the file (a new leaf under `T`) must show as unaccounted; a kept value moved by one unit in the last place fails its golden | R1-DATA-REC8's mark (lines 2605–2606); CD-10 (proposed there: the look group); R5-TEST-REC3 (r5 line 1438: "exact for every value v2 keeps") |
| PM-0005-15 | Presentation cannot reach an outcome (CD-6; AP-1) | changing every client-content value (the feel kit, the asset list, the material-slot classes, the look values) to other valid values leaves the per-tick state hashes of a 120 s scripted scenario identical, on native and Wasm (proposed: CD-6's "a presentation value cannot reach an outcome" has no mark of its own) | CI | the scenario of PM-0004-63 run twice, the second time with a seeded random valid change to every client-content file | 3 seeds. Bites: a planted read of a feel-kit value in the sim (a weapon's recoil feeding its spread) | CD-6, AP-1 (the draft's rules; the mark proposed here) |
| PM-0005-16 | What content costs to load, and its Wasm size (OQ-0005-CD2; content item 9) | reported, no pass number (the Advisor sets the budgets from it): the load time of a set the size of the imported legacy tables plus two maps (Cabana, DM_Tri) and two modes (24v24, 16v16v16: proposed, from the test-mode list), median and max over 20 cold loads, natively and in the Worker; the bytes serde_json and garde add to the Worker's Wasm module, raw and gzip | RS (native); ML: Chrome and Firefox (Wasm in the Worker) | each load in a fresh process or Worker, timed from the bytes in memory to the frozen set; the size as the module with the content crate minus a build with a stub loader, under the boundary spike's build settings (R1-RR-OQ5 asks for those settings' effect: ADR 0002's) | 20 loads per target. Passes when every cell holds a number | R1-DATA-REC1 mark 2 (lines 2496–2498); R1-DATA-OQ2 (lines 2791–2794); R1-RR-OQ5 (lines 2946–2950); CD-2 (proposed there: the Wasm half) |
| PM-0005-17 | Mustafa's content trial (OQ-0005-CD1) | the content checklist below done; his answer to OQ-0005-CD1 recorded; 0 steps left "not yet" | MP: the development server on his PC, the page in Brave | the content checklist (below) | one session; a "not yet" goes back to the draft | R1-DATA-OQ1 (lines 2788–2790: "a short trial in the spike"); plan rule 12 |
| **The content and ability spike: abilities, effects and upgrades** | | | | | | |
| PM-0005-18 | The test content runs with no ability-specific Rust code (AB-1; ability item 1) | 0 matches for the test content's ids and legacy names in the sim crate's non-test code; every test content item, fired or pressed in a scripted scenario, produces its expected event log; with every test id replaced by a random string and the definitions reordered in data, the scenario's event log is byte-identical after mapping the ids back (proposed: a search for names misses code keyed by a constant spelled otherwise); reported: the list of operations with the test items that use each, for the rust-reviewer | CI | `rg` over the sim crate excluding tests; the scenario in the replay runner; the id rename by script over the test content | 1 run. Bites: a planted `if id == "molotov"`, and a planted constant `FIREBOMB` keyed on the Molotov's handle (only the rename and reorder test sees it, whatever order the handles are given in) | R1-ABL-REC1 mark 1 (lines 2365–2368); AB-1 |
| PM-0005-19 | A new combination needs data only (AB-1, AB-7; ability item 1) | a test-only definition putting the Riot Shield's block on a vehicle and the Banzai speed buff on another class loads and behaves as its parts: the vehicle takes 30 % of an SMG hit inside the block's arc and 100 % behind it; the class moves at 1.3 × its speed for 10 s, then at 1 ×; the change set that adds it touches only content files (0 Rust changes) | CI | the combination's files; `git diff --name-only` of its commit; a scripted fixture (hits from inside and outside the arc, cos 0.30; speed sampled every tick) | 1 run. Bites: a block keyed to the Peacekeeper's unit id | R1-ABL-REC1 mark 2 (lines 2369–2371); legacy numbers: block 0.7, arc cos 0.30, ×1.3 for 10 s (r1 line 1373; INV CD8, line 89) |
| PM-0005-20 | The two §6.4 primitives work (AB-1, AB-6; ability item 1) | an effect removed when its source dies, on the tick the death is processed, in 1,000 of 1,000 scripted deaths; a trigger on the last hit, and on the last periodic tick, moves the victim's item (a granted ability) to the source in 1,000 of 1,000 scripted kills, with two kills of one victim in one tick resolved by the stable order the same way on native and Wasm (proposed counts and tie case) | CI | a fixture with test-only definitions (the item and the leech rules stay undesigned, `[LATER]`) | 1 run per target. Bites: removal one tick late; a periodic-tick kill not firing the trigger | R1-ABL-REC1 mark 3 (lines 2372–2376); R1-ABL-REC3's stable order (lines 2406–2409) |
| PM-0005-21 | One number changed in data changes the next match (AB-1; ability item 1, last clause; shares PM-0005-10's run) | after the Shabzon's damage is saved as 9: 0 Rust changes; in the running match a hit deals 8; in the next match a hit deals 9, and a Conscript (100 HP) dies on the 12th body hit instead of the 13th (9 × 12 = 108 ≥ 100; 8 × 12 = 96 < 100) | CI (the counts); ML (the run of PM-0005-10) | PM-0005-10's run; the hit counts from the sim's events | as PM-0005-10. Bites: as PM-0005-10 | R1-ABL-REC1 mark 4 (lines 2377–2378); legacy Shabzon damage 8, Conscript HP 100 (`docs/legacy-tuning.json`, `WEAPONS`, `UNIT_TYPES`) |
| PM-0005-22 | Weapons as data equal the legacy tables (AB-2, AB-3; ability item 2) | for the seven legacy weapons and the Juggernaut cannon: damage per body part (head 2.6, body 1.0, legs 0.65; Bagpipe head 1.2), the Shabzon's falloff at 10, 15, 25, 35 and 50 m against legacy's `falloffMult`, pellet count, magazine, reload and burst: integer results exactly, floating-point results within 1e-6 relative; timing fields compared as stored values (the loader's converted duration in seconds within the rounding it reports, PM-0004-11) and as behaviour (each shot within one tick, 33.33 ms, of the table's time) (proposed reading: on a 30 Hz tick no shot can land on a 0.06 s instant, so 1e-6 applies to stored values only); a bot and a human firing one definition from the same inputs give identical results (whether v2 keeps one model is Mustafa's, R1-ABL-OQ6) | CI | golden fixtures captured from the legacy oracle (`tools/preview.js legacy eval`), each with the legacy commit, the inventory row and the inputs; v2's tests read them | 1 run; joins the gate. Bites: a v2 value moved beyond its tolerance (R5-TEST-REC3's mark); the legacy reset-not-carry timer (GC5) | R1-ABL-REC2's mark (lines 2396–2400); R5-TEST-REC3 (r5 lines 1431–1453); INV CS7, CD8 (lines 175, 89) |
| PM-0005-23 | One table per kind, every link checked, fuzzed definitions never panic the sim (AB-4; ability item 3; shares C) | the legacy `WEAPONS`, `UNIT_TYPES`, `VEHICLE_TYPES` and ability fields convert with every legacy number mapped once or dropped with a reason (0 unmapped, 0 mapped twice: proposed reading, as CD-10 drops bot-only fields and colours); an unknown id, an unknown tag and an out-of-range value each fail at load with the file and field named, for every kind, track definitions included (a track naming an unknown effect fails); 0 panics when every definition of C that the loader accepts runs in a scripted 300-tick scenario (10 s at 30 Hz: proposed; the research's "never panic the simulation" is about running them, not loading them) | CI; CI nightly (the sim runs on fresh seeds) | the conversion report by script; C's accepted definitions each given to a unit or weapon in the scenario | 1 run (C's 10,000 files); nightly fresh seeds. Bites: a sim that indexes an array with a pellet count of 0; a planted dangling ability id | R1-ABL-REC5's mark (lines 2460–2463); AB-4 (proposed there: track definitions) |
| PM-0005-24 | Tech tiers gate abilities and purchases (AB-5; ability item 4) | an ability press before the team's Tech 1, and a T2 purchase before Tech 2, refused by the authority in 1,000 of 1,000 scripted attempts each; the same presses and purchases after the tier accepted in 1,000 of 1,000 (a control: proposed); and the minute-5 floor (LOOP_V2 §4 `[DECIDED]`: "Every team reaches Tech 1 by minute 5 at the latest"): a team with no cashout reaches Tech 1 at the floor's tick, read from data (5 × 60 × 30 = 9,000 ticks at 30 Hz: arithmetic), and its presses are refused at tick 8,999 and accepted from tick 9,000 | CI | scripted team state at Tech 0, 1 and 2; the authority's verdict log | 1 run. Bites: an ability with no tier tag; a floor that is never applied (the team without a cashout stays at Tech 0 past tick 9,000) | AB-5 (proposed there; LOOP_V2 §4 lines 50–52, `[DECIDED]`; INV GE4, line 290); the count 1,000 from R1-ABL-REC4 (line 2444) |
| PM-0005-25 | The counters reproduce from data alone (AB-6, AB-7; ability item 5) | from the imported tables: the Warrior kills a Conscript in 0.86 s within one tick (13 SMG body hits, 12 × 0.072 = 0.864 s); the Peacekeeper kills a Warrior in 0.85 s within one tick when 11 of its 16 pellets land over two shots (11 × 11 = 121 ≥ 120 HP), and 10 of 16 leave it alive (a control: proposed); a Peacekeeper behind the Shield takes exactly 63 SMG body hits (62 leave 1.2 HP) and 19 without it; the triangle's third leg (the Conscript beats the Peacekeeper, AB-7): a Molotov blast deals a Peacekeeper the same damage with the Shield up as with it down (a blast skips the frontal block, GC7), and at 35 m a Bagpipe pellet deals 0 while an AMK body hit deals 34 (ranges 34 m and 300 m, AMK damage 34: LEGACY_TUNING.md, `WEAPONS`, lines 173 and 181, as ability item 5 cites them); AB-2's immunity rule: a soldier inside its spawn protection takes 0 damage from a body hit and from a blast, and full damage once the effect's 90 ticks end (3 × 30 at 30 Hz), under every option of OQ-0005-AB14; the knockback operation changes a target's velocity by the test weapon's value within 1e-6 relative, with 0 matches for the test weapon's id, or the spawn-protection effect's, in the sim's non-test code | CI | scripted duels on the converted data (the pellet hits scripted, since spread draws from an [authority-only stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream)); the times from the sim's events; the knockback weapon is test-only (the Javelin's numbers are Mustafa's) | 1 run (deterministic). Bites: the Shield block applied to blasts only; the Shield block applied to blasts as well (the third leg fails); an immunity rule that skips blasts; the protection's length taken from a constant instead of the effect's data (changing the data to 60 ticks must move the end) | AB-6, AB-7 (proposed there); R1-ABL-REC3's mark (lines 2416–2418: the 0.86 s, ADR 0004's, PM-0004-9); INV CD13 (line 94); arithmetic in AB-7 |
| PM-0005-26 | u1: no track-specific Rust code (AB-8; ability item 6) | 0 matches for the test tracks' ids in the sim crate's non-test code, and PM-0005-18's rename test passes with the test tracks renamed | CI | as PM-0005-18 | 1 run. Bites: a planted branch on a test track's id | AB-8 (proposed there; UPGRADE-SCOPE) |
| PM-0005-27 | u2: picks survive death (AB-8; ability item 6) | with picks on unit type U and weapon type W: the unit dies, respawns as a Recruit and buys U again; the new unit and its W carry exactly the same pick effects (effect lists compared field by field) in 1,000 of 1,000 scripted cycles | CI | scripted cycles over test tracks; it shares ADR 0004's PM-0004-64 runs (that row checks the player record; this one the effects on the new unit) | 1 run. Bites: picks applied to the soldier entity instead of from the record | AB-8 (proposed there); UPGRADE-SCOPE |
| PM-0005-28 | u3: tracks survive switching (AB-8) | U, then V, then U again: U's track unchanged field by field, V's its own, in 1,000 of 1,000 scripted cycles (proposed count, as u2) | CI | as PM-0005-27 | 1 run. Bites: one track shared by every unit type | AB-8 (proposed there); UPGRADE-SCOPE |
| PM-0005-29 | u4: a lost gun's track waits for the next gun of its type (AB-8) | the player holds a looted weapon of type X and loses it on death (dropped by the fixture's drop rule, a test rule while OQ-0005-AB9 is open; no voluntary drop or swap, which OQ-0005-AB1 and AB9 leave open), then takes another X the fixture places: the player's X picks apply to it; a second player who takes the first gun gets their own X picks and none of the first player's; 1,000 of 1,000 scripted cycles (proposed count) | CI | scripted deaths, fixture drops and claims (ADR 0004's pickup entities, PM-0004-66) | 1 run. Bites: picks stored on the gun | AB-8 (proposed there; ability item 6, u4); UPGRADE-SCOPE |
| PM-0005-30 | u5: every track is empty at a match's first tick (AB-8) | 100 of 100 consecutive matches in one process start with every track empty and no pick effect on any unit | CI | shares ADR 0004's PM-0004-65 runs (that row checks the record; this one the effects) | 1 run. Bites: a pick effect left on a pooled unit | AB-8 (proposed there); UPGRADE-SCOPE; LOOP_V2 §8 |
| PM-0005-31 | u6: only offered picks count, and offers repeat (AB-8) | every offer holds the data's offer size (3) of candidates whose requirements hold, distinct or not as the test track's data says (whether candidates are always different, and what a track with fewer left than the offer's size does, are OQ-0005-AB8's: here fixture rules, not game rules), with one test track that has fewer candidates than the offer's size handled as its data says; a pick that was not offered, or for a closed offer, refused in 1,000 of 1,000 forged picks; 1,000 of 1,000 offered picks accepted (a control: proposed); the same seed and inputs give byte-identical offer logs on two runs, and on native and Wasm: the offers stream is authority-only (ADR 0004 part 8), but in local play the authority runs in the browser's Web Worker (ADR 0002) | CI (native; Wasm in Node) | forged pick commands; the offer log written by the authority | 2 runs per target (the log); 1 run (the counts). Bites: an authority that accepts any candidate of the track; offers drawn from a clock seed; an offer size hard-coded as 3 (changing the data to 2 must change every offer) | AB-8 (proposed there; ability item 6, u6); UPGRADE-SCOPE; SEC-SPREAD (a)'s reasoning |
| PM-0005-32 | How many long-lived active effects the step can carry (AB-10; ability item 7) | reported, no pass number: the ability, effect and weapon step's p99 per tick in SS at 8, 16 and 32 active effects per soldier (384, 768 and 1,536 at N = 48), and the bracket in which it reaches 1 ms (where the crossing lies between two points, points 4 active effects per soldier apart are added: proposed); the extra effects are permanent attribute modifiers on the soldier's unit and weapon, the shape AB-8 gives picks (proposed); the effect count per soldier checked equal to the target on every tick | RS (native); ML: Chrome and Firefox (Wasm, isolation flag on) | T per point; the 8-per-soldier point is ADR 0004's PM-0004-62 run (its 1 ms p99 and 0 allocations are that row's pass marks) | T, V per point. Passes when every cell holds a number | AB-10 (proposed there); R1-ABL-REC6 (lines 2465–2472); GL lines 397–418 (8 per soldier) |
| **ADR 0003's look proof test: this record's asset marks** | | | | | | |
| PM-0005-33 | Every test model is listed, and a broken link fails at load (AP-2, AP-3) | 100 % of the test models named by id in the test content and listed in the asset list; a dangling asset id and a slot whose class is not in the class table each fail at load with the file and field named (1 of 1 each); 0 stand-in models drawn | CI | the content crate's whole-set check on the look test's content | 1 run. Bites: the two planted defects | AP-2 (proposed there: R1-DATA-REC2's error mark, lines 2515–2516) |
| PM-0005-34 | The models load on both backends and carry no look of their own (GPU section parts 1, 6) | 0 console errors in every start; 0 meshes of a listed asset drawn with a material other than the shared material of its slot's content (identity against the material cache, which is keyed by slot content: the class, the faction for faction slots, and the slot's own base and emissive colour, textures, side and alpha mode, GPU part 1); 0 model requests other than listed glTF/GLB files and the buffers and images a listed `.gltf` references, and 0 requests outside the client's host | ML: Chrome, Edge, Firefox, each on WebGPU and with `forceWebGL`; MAC: Safari, both backends | PM-0003-2's starts; after load, a script walks the scene graph; the page's network log | PM-0003-2's 10 starts per browser and backend. Bites: one slot left with its glTF material | the GPU section's spike part (R3-WGPU-REC1 mark 1, r3 lines 910–912); R5-GATE-REC3's G12 mutation (r5 line 1723) |
| PM-0005-35 | Materials are shared by content (GPU section parts 1, 2, 7) | the material objects on listed assets' meshes number at most the distinct slot contents those meshes show (GPU part 1's key: the class, the faction for faction slots, and the slot's own base and emissive colour, textures, side and alpha mode), two meshes with the same slot content share one material object, and each mesh's material carries its own slot's base colour and textures (read back and compared with the loaded file), on every frame | CI (a count); ML: Chrome on WebGPU and with `forceWebGL` | LS with every test model in each faction's colours; distinct material objects counted by identity, distinct slot contents counted from each slot's key fields in the loaded files | 1 run per backend (counts repeat, PM-0003-35). Bites: a loader that clones the class material per mesh (legacy's per-mesh conversion, CL4); a loader that keys materials by (slot class, faction) only, which must fail the colour comparison (two slots of one class with different base colours then share one material) | the GPU section's spike part (proposed there; R3-LOOK-F3, r3 lines 331–334); GPU part 1; legacy's content signature (CL4, `LEGACY_INVENTORY.md` line 113) |
| PM-0005-36 | The two derived per-unit caps hold (GPU section part 4; AP-5) | the test soldier as drawn: (draw calls with it shown − hidden) at most 4 × the passes that draw it; the test very large unit: at most 25 × those passes; on every frame | CI (a count); ML: Chrome on WebGPU and with `forceWebGL` | LS with one soldier, then one very large unit, shown and hidden; shadow and main pass (PM-0003-25's method, here for one unit) | 1 run per backend. Bites: a soldier with 5 material slots (10 draws over 2 passes, above 8); a very large unit of 30 single-material parts (60, above 50) | the GPU section's part 4: 4 per pass (R3-CHAR-REC1, r3 lines 1044–1045); 200 ÷ 4 ÷ 2 = 25 (R3-CHAR-REC4, r3 lines 1088–1090) |
| PM-0005-37 | Mustafa's verdict on each asset family (OQ-0005-AP2; AP-7) | every step of the families checklist below judged, with a pick or "not yet" per family | MP: Brave on Windows | the families checklist (below), each view beside a legacy screenshot at the same framing | one session; a "not yet" goes back to the draft | OQ-0005-AP2; LOOK_V2 §9 (lines 76–78); plan rule 12 |
| **The asset spike (after the asset-pipeline research)** | | | | | | |
| PM-0005-38 | The tool processes the test models and writes their list entries (AP-2, AP-6; OQ-0005-AP1's answer) | 100 % of the test models processed; every entry holds its id, its hashed file name, its slots with their classes and its counts; 100 % of the entries' triangle, material and texture counts equal a recount of the output file by an independent reader (proposed: OQ-0005-AP1 asks whether the gate recounts or trusts the entry) | CI | the tool on the test models; the recount script | 1 run. Bites: an entry with its triangle count lowered by 1 | the asset pipeline section's spike part; OQ-0005-AP1 |
| PM-0005-39 | Asset files are served like the bundle (AP-4) | every asset file's name holds the hash of its bytes (recomputed); its `Cache-Control` contains `immutable`; a repeat visit fetches 0 bytes of it (`transferSize` 0); the largest asset file under 25 MiB (26,214,400 bytes) | the client host's preview (ADR 0006's hosting setup); ML: Chrome | ADR 0006's PM-0006-4 and PM-0006-5 methods, applied to every asset file | 3 repeats from a fresh profile. Bites: an asset served `no-cache`; a planted 26 MiB file is flagged | the asset pipeline section's spike part (proposed there: R1-DATA-REC6, lines 2577–2578; R6-HOST-REC1's mark, r6 lines 641–643) |
| PM-0005-40 | The gate holds the per-unit budgets (AP-5) | a planted over-budget asset fails the gate's asset budget check; a budget raised without a change to the budget file fails; controls pass: an asset exactly at its budget, and a raise written in the budget file with its reason | CI | `gate --prove`'s mutations and controls | every `--prove` run; 1 of 1 each | the asset pipeline section's spike part (R5-GATE-REC4's mark, r5 lines 1743–1744); R5-GATE-REC3 (r5 lines 1706–1732) |
| PM-0005-41 | No asset reader reaches the sim, the server or the player build (AP-1, AP-6) | a planted glTF reader dependency in `sim`, and the same in `server`, each fails the dependency check; a planted import of the asset tool's package from `client` fails the player-build manifest walk (proposed: AP-6's "never in the player build"); control: the tool depending on a glTF reader passes | CI | `gate --prove` (the dependency rules check and the manifest walk, ADR 0002's) | 1 of 1 each. The draft's migration puts the dependency check in the foundation; the bite runs here once the tool's package exists | AP-1, AP-6 (proposed there); R5-LAYOUT-REC2's mark (r5 lines 1189–1192) |
| PM-0005-42 | Every slot is classed and every faction slot coloured, on both backends (GPU section parts 2, 6) | 0 slots without a class; every faction colour slot's colour equal to the faction's colour value in its data (CD-13's client definition), for every faction and test model, read from the materials; and drawn on each backend: in an unlit debug view, a pixel probe at a faction slot of each test model returns that colour within 2/255 per channel (proposed tolerance: reading materials proves the data path, which is the same on both backends, not that each backend draws the colour); 0 console errors | ML: Chrome, Edge, Firefox on both backends; MAC: Safari | the research's processed test models in LS; colours read back from the materials and compared with the faction data | 10 starts per browser and backend (PM-0003-2's count). Bites: a faction slot given another faction's colour; a backend path that drops the slot's colour (the probe must see it while the material still reads right) | the GPU section's spike part |
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
1. The test building as an authored model and as the ported kit, at ADR 0003's test view (legacy's framing, CD5; not a
   camera decision, R2-INT-OQ2), beside legacy's house (LOOK_V2
   §2, §3). Expected: the half-timbered house with the brick-tile roof reads in both. Pick: authored, kit, or not yet.
2. The props (barrels, carts, hay bales, logs) as authored models and as the ported kit, beside legacy (LOOK_V2 §2).
   Pick: authored, kit, or not yet.
3. The ground and its textures as an authored asset (if shown) and as the ported procedural textures. Pick: authored,
   kit, or not yet.
4. Trees: no pick; the trees shown are legacy's tree builders ported as labelled placeholders (GW18), judged in ADR
   0003's look checklist (step 15) for variety and lushness only; the tree style is LOOK_V2 §8's `[OPEN]` item.
5. Every test model in each faction's colours. Expected: no missing piece and no grey stand-in anywhere (a missing
   model fails at load, PM-0005-33); the factions told apart by their colour slots.

#### The LOD checklist (Mustafa, in Brave, in the asset spike; only with PM-0005-44's condition)
Every view is at ADR 0003's test view (legacy's framing, CD5: height 40, boom 56.6, field of view 15°, yaw 45°) or a
named variant of it, never a camera decision: the v2 framing is R2-INT-OQ2's, and both steps are taken again at its
own closest and farthest once that question is answered.
1. Each test model with LOD levels at two test views: the test view itself, as the far case, and the same camera at
   half its distance to the player, as the close case (ADR 0003's look checklist, step 9) (LOOK_V2 §5). Expected:
   simple and readable from a distance, real depth up close; the silhouette kept at the far view.
2. The camera moved slowly from the close view out to twice the test view's distance and back (a test path through
   each switch distance, proposed: legacy has no zoom). Expected: no visible pop or flicker at a switch.

#### The Feel Lab checklist (Mustafa, in Brave on his Windows PC; ABL-PREDICT)
Beside it: PM-0005-45's numbers for each ability and variant. The server injects P2 (100 ms round trip, ±20 ms, 2 %
loss); his own round trip to the server is recorded and added. The two variants are shown as A and B, their order
shuffled by the session and their labels hidden until he has answered (proposed: the label must not decide the feel).
1. Throw the Molotov ten times with A, then with B. Expected: he can say which one feels right for a throw.
2. Raise and lower the Riot Shield ten times with A, then with B, while moving. Expected: he can say which one feels
   right; whether the slow when the Shield goes up feels late in either.
3. Before his team reaches Tech 1, press each ability with A and with B. Expected: the press is refused in both, with
   no start shown, since the client knows its team's tier (team state goes to the team, ADR 0001) and the ability's
   required tag (AB-5), so a predicted variant can refuse the press locally. Record: any start that shows, in either
   variant, as a defect of that variant.
4. Record: his answer to ABL-PREDICT (does the client predict the player's own abilities), and whether it differs per
   ability.


## Migration plan
The legacy game stays playable in `legacy/` at every step (ADR 0002); v2 is playable from slice 1. The places in
SLICE-ORDER and the "Done when" lines are proposed (reason: each names the spike items that test what its slice brings,
run again on the real crate; slice 1's list is the content data section author's, with the map migration item moved to
slice 2, where the maps migrate).
1. **The netcode proof test** runs first; it needs none of this record's machinery (the cue-once check it carries for
   R1-ABL-OQ3 is ADR 0001's mark).
2. **The foundation:** the `content` crate's place in the workspace; the generated schema and types in
   `packages/types`, diffed by the gate (CD-3; ADR 0002's milestone); the dependency check that keeps glTF readers out
   of `sim` and `server` (AP-1).
3. **The content and ability spike** (above).
4. **Slice 1, move & shoot:** the crate with CD-1 to CD-8 on weapons and movement (the import's weapon and movement
   tables, CD-10); weapons as data (AB-3, AB-4); the bundle and its hash in local play. *Done when:* content items 1,
   2 and 6 and ability item 2 pass on the real crate in CI.
5. **Slice 2, bots and three teams, played locally:** units, factions and modes as files (CD-6, CD-13); the T1 classes
   as unit definitions, with one weapon model for bots and humans if Mustafa keeps it (R1-ABL-OQ6, a design question);
   Cabana and DM_Tri migrated (CD-4, CD-11). *Done when:* content item 4 and ability item 3 pass on the real crate.
6. **Slice 3, online:** the hash at join with ADR 0001 (CD-8); the server's check of private-room overrides (CD-9).
   *Done when:* content item 7 passes against the real server.
7. **Slice 4, the loop:** the rule tables (CD-6); tech tiers gating abilities and purchases, and the three T1
   abilities (AB-5, AB-6); moment rules once OQ-0005-CD6 is answered (CD-12); the feel kit and the Feel Lab for the feel
   pass (CD-14); the upgrade tracks' kinds and files once OQ-0005-AB1 to AB3 are answered (AB-4, AB-8). *Done when:*
   ability items 4 and 5 pass on the real crate, and item 6 once track content exists.
8. **Slice 5, Octopon-3 and Editor v2:** the map document under EDITOR_V2 §2 (CD-11), with EDITOR_V2's own
   architecture plan. *Done when:* EDITOR_V2 §4's bars on the document (1,000 random commands undone and redone return
   a byte-identical document; every v1 map migrates).
9. **Slice 6, scale to 48:** the ability, effect and weapon step measured with real content (R1-ABL-REC6; ability item
   7); content's load time and its share of the Wasm size measured again on the full content (OQ-0005-CD2; content
   item 9; the size limit is ADR 0002's, part 4).
- **Assets:** the look proof test uses plain glTF/GLB exports (the GPU section's part 8); the asset-pipeline research,
  then the asset spike, come before the first authored model enters the game (AP-1 to AP-6); until OQ-0005-AP2 is
  answered, buildings and props keep the ported kit beside authored assets (AP-7).

## Test plan
- **CI checks from the spike items:** the fuzz runs on native and Wasm, with zero panics (content item 2; ability item
  3, track definitions included); the float round trip (content item 1); the schema and types diff (content item 3);
  migration goldens, with snapshots of the validation errors (R5-TEST-REC3; content item 4); the import report (content
  item 8); the bundle tampering cases (content item 6); the asset list's reference checks, the per-unit budget check
  and the glTF dependency check (AP-1, AP-2, AP-5).
- **Ability goldens:** the legacy-table conversion and the CD13 matchups from data alone (ability items 2, 3 and 5);
  the upgrade cases u1 to u6 with test-only tracks (ability item 6).
- **Identical results:** the determinism log of R1-ABL-REC3 (a 120 s scenario, 3 seeds, native and Wasm) in ADR
  0004's suite; byte-identical offer logs on two runs (ability item 6, u6).
- **Balance:** sims, 3 runs per build, in two-team and three-team modes separately (VISION_V2 §3), and match telemetry
  keyed by definition ids and the content hash (ADR 0006).
- **The look:** the look proof test's asset items (2 under "What a spike must prove"), judged by Mustafa in Brave
  beside legacy screenshots (LOOK_V2 §9).

## Rollback
No spike starts before approval, and spikes are throwaway (SPIKE-SCOPE (a)), so nothing needs backing out before slice
1. After that, by part:
- JSON misses its load-time budget, or content's share takes the Wasm file past the limit ADR 0002 sets: the build
  compiles the same JSON into a binary runtime format (option D), and JSON stays the authored form (OQ-0005-CD2).
- garde fails on wasm32: checked newtypes and hand-written checks replace it, and schemars attributes carry the ranges
  (CD-2).
- Reload at the next match proves too slow for tuning: OQ-0005-CD4's option (b) is R1-DATA-REC5 as written, and ADR
  0001's message catalogue already holds its message.
- The ability items 1 or 2 fail for one mechanic: it becomes a new reviewed operation; if many fail, those mechanics
  fall back to option A4 with E1's tables kept, and OQ-0005-AB13 reopens scripting.
- Picks cannot be written as effects and granted abilities (ability item 6, u1, fails): tracks get their own
  operations, a reviewed Rust change.
- The asset list does not fit what the asset-pipeline research finds: its fields change, and AP-1, AP-4 and AP-5
  stand.
- An authored family fails the look proof test: that family keeps the ported procedural kit (AP-7; OQ-0005-AP2).

## Appendix A: research dispositions
The decision map gives this record 34 ids as owner, and the sections dispose of all of them: 26 in the content data
and asset pipeline sections (engine-architect), 7 in the ability section (game-designer) and 1 in the GPU section
(graphics-engineer). The rows below are the sections' own, verbatim and in that order; the frame adds no row, because
no id is left. Recommendations: adopted · changed (how, why) · rejected (why). Open questions: answered (how) · open
(its OQ) · spike (which). In the first 26 rows, "spike mark n" is item n of the content data section's "What a spike
must prove"; in the ability section's rows, "What a spike must prove" n is that section's item n.

Counts: adopted 5 · changed 6 · answered 8 · open 9 · spike 6 · rejected 0 (34 rows).

| id | disposition | where or why |
|---|---|---|
| R1-DATA-REC1 | adopted | CD-1; spike marks 1 and 9 |
| R1-DATA-REC2 | changed: adds a maximum length on every collection (R1-RR-OQ2) and the no-panic loader (R5-RUSTLINT-REC3, R1-RR-OQ3), refuses a fallback model too, and reports each sim pool's worst case per mode for ADR 0004 part 1; the stages as written | CD-2; spike mark 2 |
| R1-DATA-REC3 | changed: the schema is written when the gate regenerates (on `cargo test` or by a tool), not by the shipped build, and the generators' derives stay out of the shipped builds (ADR 0002 part 5; the rust-reviewer's step-10 review); content's TypeScript types come from ts-rs beside it, answering critic rank 31 | CD-3; spike mark 3 |
| R1-DATA-REC4 | adopted | CD-4; spike mark 4 |
| R1-DATA-REC5 | changed: simulation numbers apply at the next match in development (proposed; the mid-match swap is OQ-0005-CD4's option (b)), client content applies live, production pins one set per match as written; reason: its own partial trace (r1, lines 2555–2558) and plan rule 4 | CD-7; spike mark 5 |
| R1-DATA-REC6 | adopted | CD-8 (the archive its mark needs is ADR 0006 part 12's); spike mark 6 |
| R1-DATA-REC7 | changed: the path for a map uploaded to a private match is not built (no VISION_V2 §6 item asks for it); the rule holds for every payload a client or the editor sends | CD-9; spike mark 7 |
| R1-DATA-REC8 | changed: the look group, "pending the look brief" (r1, lines 2597–2598), is listed value by value for ADR 0003's look proof test, since LOOK_V2 exists | CD-10; spike mark 8 |
| R1-DATA-OQ1 | open: OQ-0005-CD1 | comments in content files (Mustafa, after a trial in the spike) |
| R1-DATA-OQ2 | spike: the content and ability spike measures load time and content's share of the Wasm size; the load-time budget and the binary-format fallback are OQ-0005-CD2, the size limit ADR 0002's (part 4) | CD-2; spike mark 9 |
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
| R1-ABL-REC1 | adopted | AB-1, AB-2, AB-5, AB-6, AB-7; its marks are "What a spike must prove" 1 |
| R1-ABL-REC2 | adopted | AB-3; its mark is "What a spike must prove" 2 |
| R1-ABL-REC5 | changed: for UPGRADE-SCOPE (Mustafa, 2026-10-31, after the research), two kinds are added (the upgrade track; the drop and pickup values) and the unit kind names its class's own weapons; the rest as written | AB-4, AB-8; "What a spike must prove" 3 |
| R1-ABL-OQ1 | open: OQ-0005-AB12 | the Advisor decides the reference |
| R1-ABL-OQ2 | open: OQ-0005-AB13 | the source, or the spike's trigger |
| R1-ABL-OQ3 | spike: the cue-once rule is tested where a replay happens: own weapon cues in [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test) (the prediction scope), own ability cues in the Feel Lab test if ABL-PREDICT says yes; the mark is R1-ABL-REC4's (1,000 scripted presses), ADR 0001's | AB-9 |
| R1-ABL-OQ9 | answered: it matters only if option A3 returns; A3 is not adopted (AB-1), and OQ-0005-AB13 names the trigger | AB-1; Options, Question A |
| R3-CHAR-OQ3 | open: OQ-0005-AS6 | the authored soldier (LOOK_V2 §3, §8) sets the vertex and bone counts; the render budget bench measures them against ADR 0003's rows |

## Sources
Each section cites its own sources inline; lines are at 9e7db28 (the topic files do not change in this pull request).
The frame read, beyond the four sections:
- `docs/VISION_V2.md` §2, §3, §4 and §6 (lines 25–81); `docs/LOOP_V2.md` line 8 and §4 to §8; `docs/EDITOR_V2.md`
  lines 4–5 and §2 to §4; `docs/LOOK_V2.md` §1 to §3, §5, §6.4, §8 and §9 (lines 65–78 for §8 and §9).
- `docs/STATUS.md`, "Decisions pending": SLICE-ORDER, MODES-3, SEC-SPREAD, ABL-PREDICT, BASELINE-WHEN, TICK-30,
  R7-MARKS, SPIKE-SCOPE; UPGRADE-SCOPE in Mustafa's words of 2026-10-31 (STATUS records it in this pull request).
- `docs/research/r1-simulation-core.md`: the data unit's note that no shipped-game source was read (lines 1577–1579);
  the legacy fact CD12 (lines 1600–1602); Options §5, Question E (lines 1826–1832) and Options §6, Q2 (lines
  1845–1855); R1-DATA-REC6 (lines 2565–2575) and R1-DATA-REC8 (lines 2596–2606); R1-ABL-OQ1 (lines 2756–2758);
  R1-RR-OQ5 (lines 2946–2950).
- `docs/research/r6-infrastructure.md`: R6-TEL-REC3 (lines 828–836).
- `docs/research/GLOSSARY.md`: the entries linked above; §8 (the foundation, the netcode proof test), appended in this
  pull request.
- `docs/LEGACY_INVENTORY.md`: the marks (lines 1–40) and rows ED5 and ED7 (lines 340 and 342), with the rows the
  sections cite.
- `docs/adr/0000-template.md`.
- The other decision drafts, by number and part: ADR 0001 parts 11 and 12 and its message catalogue; ADR 0002 parts 3
  and 5; ADR 0003 part 10 and its inventory table; ADR 0004 parts 16 and 18, OQ-0004-5 and OQ-0004-6; ADR 0006 part
  12.
- The research the sections rest on, by topic (each cited inline at its lines): R1, the data and abilities units
  (R1-DATA-REC1 to REC8 and their findings and open questions; R1-ABL-REC1 to REC6, F1, F9, F17, F22, OQ1, OQ2, OQ3,
  OQ5, OQ6, OQ8, OQ9, OQ10),
  R1-BND-REC4, F9, R1-TICK-REC1, R1-ECS-REC3, OQ7, R1-XU-OQ8 to OQ10, R1-RR-OQ2, OQ3, OQ5, R1-XT-OQ13, OQ15; R2-ROOM-REC6,
  REC7, R2-INT-REC2, REC5, OQ8, R2-SYNC-F17, OQ7, OQ9, R2-XU-OQ14; R3-CHAR-REC1 to REC4, F1, F4, F8, F10, OQ3, OQ5,
  R3-BUDGET-REC2, REC3, OQ5, R3-LOOK-F3, F12, R3-WGPU-F6, REC3; R4-PHYS-REC2, REC3, R4-NAV-REC1, R4-FLOORS-REC2, REC3,
  OQ4, R4-OBST-REC2; R5-LAYOUT-REC2, REC3, R5-RUSTLINT-REC3, R5-TEST-REC3, R5-GATE-REC4, R5-XT-OQ9; R6-HOST-F3, REC1,
  R6-TEL-REC1, OQ3.
- The process inputs, which leave the branch with the checkpoint: the plan (revision 3: rules 1 to 12 and its ADR
  0005 paragraph), the advisor's conditions 4, 11 and 13 of 2026-10-31, and the decision map's parts a, d, g and j
  (J3 to J5).
