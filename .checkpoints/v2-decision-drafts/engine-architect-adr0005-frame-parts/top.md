# ADR-0005: Content data, abilities and assets

**Status:** Proposed
**Date:** 2026-10-31
**Author:** engine-architect (frame, content data, the asset pipeline) · game-designer (the ability and effect system) · graphics-engineer (the asset pipeline's GPU rules) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no

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
one Appendix A. Where two sections disagree, the frame says so (OQ-0005-1, OQ-0005-2) and each author fixes its own
text in the fix round.

### Where this record sits among the drafts
It defines the content every other draft reads (decision map, part d): typed definitions read by id (I-1; ADR 0004
executes them), content's schema and TypeScript types (I-5's content part, in ADR 0002's `packages/types`), the
content bundle and its hash (I-6; ADR 0006 serves it, ADR 0001 names it at join), the mode file (I-7; the room is ADR
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
   length bounds on every collection and no panics (CD-2); the schema and content's TypeScript types from the same
   Rust types (CD-3); versions and migrations (CD-4); one definition per file, one folder per kind (CD-5). *Need:*
   VISION_V2 §6.1, §6.3, §6.5, §6.7.
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
   cost mark: VISION_V2 §3 and §4 only.
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

