# Claims ledger: ADR 0005's frame (engine-architect, 2026-10-31/11-01)

Covers only the frame's own text in `engine-architect-adr0005-draft.md` (header to the end of "Decision", and
"Options considered" to "Sources"). The four sections placed verbatim keep their own ledgers:
`engine-architect-adr0005-content-claims.md` (content data and the asset pipeline), `game-designer-adr0005-abilities-claims.md`
and `graphics-engineer-adr0005-assets-claims.md`. The copied Appendix A rows and the seven copied inventory rows are
the content and section authors' claims and sit in those ledgers.

Tiers (plan rule 9): **research verified** (R1 or R2 recommendation claims and the findings their verifiers checked,
r1:3233) · **research, source to check** · **arithmetic** · **settled** · **design doc** · **proposed** (a frame
proposal with its reason). Added for a frame: **restating** (a placed section's text, or another draft's, compared word
for word by the frame's author) · **repo doc** (a repository file such as `docs/LEGACY_INVENTORY.md` or CLAUDE.md) ·
**process** (the plan, the advisor's conditions, the decision map: notes files that leave the branch with the
checkpoint). Line numbers: research files and design docs at 9e7db28 (unchanged at the head, except `docs/LOOK_V2.md`,
added in 6617e25); the drafts at their notes files of 2026-10-31.

| # | Claim (as the frame states it) | Frame place | Citation | Tier | Checked |
|---|---|---|---|---|---|
| 1 | Status Proposed; date 2026-10-31; authors engine-architect (frame, content data, asset pipeline), game-designer (abilities), graphics-engineer (GPU rules) | header | the brief (prompts/engine-architect-adr0005-frame.txt); decision map part j, J4 | process | yes |
| 2 | One JSON model read and checked by one Rust content crate with the same verdict on server, browser and editor | intro paragraph | CD-1, CD-2 | restating | yes |
| 3 | A faulty file is refused with the file and the field named; nothing falls back to a default | intro | CD-2 ("Errors name the file … field path"; "no fallback map, weapon or model"); AB-4 ("nothing defaults silently") | restating | yes |
| 4 | Abilities and effects: a short, fixed list of Rust operations combined by data, a cut-down GAS | intro | AB-1; R1-ABL-REC1 (r1:2342–2363) | restating (research verified underneath) | yes |
| 5 | Weapons are data; every hit goes through one damage pipeline | intro | AB-2, AB-3; R1-ABL-REC2 (r1:2380–2398) | restating | yes |
| 6 | A track is data, a pick is an effect or a granted ability, progress on the player's record, not the soldier | intro | AB-8; UPGRADE-SCOPE ("the tracks live on the PlayerState, not the Pawn") | restating; settled | yes |
| 7 | Authored models decide nothing, carry no look, fit per-unit budgets the gate checks | intro | AP-1; GPU part 1; AP-5 | restating | yes |
| 8 | Compression, levels of detail and renaming are not researched and wait for their own research | intro | ASSET-RESEARCH (a) (advisor condition 4); GPU section intro and OQ-0005-AS1 to AS4 | process; restating | yes |
| 9 | The record designs no unit, weapon, ability, upgrade, rule or look | intro; Settled inputs | the advisor's condition 11 (escalation category 2); plan rule 6 (the look) | process | yes |
| 10 | Four sections by three authors | intro; "How this record is put together" | decision map part j, J4 | process | yes |
| 11 | VISION_V2 §6 is at lines 62–81 | Requirements | `docs/VISION_V2.md:62–81` | design doc | yes |
| 12 | §6.3 quote "A data-driven ability and effect system, like UE's Gameplay Ability System. Units, vehicles, weapons and abilities are built from data, so combos and counters can emerge and tuning needs no code changes" | Requirements | VISION_V2:68–69 (quote joined across the line break) | design doc | yes |
| 13 | §6.3 served by AB-1 to AB-7 on CD-1 to CD-6 | Requirements | the sections' Need lines (AB-1 to AB-7: §6.3; CD-1, CD-2, CD-3, CD-5, CD-6: §6.3) | restating | yes |
| 14 | §6.1 modes, rooms, private matches served by the mode file (CD-6) and the overrides (CD-9) | Requirements | VISION_V2:64; CD-6, CD-9 Need lines | design doc; restating | yes |
| 15 | §6.2 a true 3D simulation; the map document holds what sim, bake and renderer read (CD-11); no model decides a collision (AP-1) | Requirements | VISION_V2:65; CD-11 (Need §6.2), AP-1 (Need §6.2) | restating | yes |
| 16 | §6.4 vehicles: the vehicle kind and the leech and marking primitives (AB-4, AB-6); parts from the sim's part records (GPU part 3; AP-1); seats `[OPEN]`, count left open by AB-4 | Requirements | VISION_V2:71–73; AB-4 ("seats, their count left open by VISION_V2 §6.4 `[OPEN]`"), AB-6 rows "The Hammer's leech", "Marking"; GPU part 3; AP-1 | design doc; restating | yes |
| 17 | §6.5 "All server-side and tunable": rule tables (CD-6), tech tiers as tag gates (AB-5) | Requirements | VISION_V2:74; CD-6; AB-5 | design doc; restating | yes |
| 18 | §6.6: definition ids key the record; every match records content hash and mode id | Requirements | CD-8, CD-12, AB-7 | restating | yes |
| 19 | §6.7 served by CD-13, AB-4, AP-2, AP-3, GPU part 2 | Requirements | the parts' Need lines | restating | yes |
| 20 | §6.8 moment rules as content (CD-12); kill-cams, replays, highlights `[LATER]`, kept possible by recorded matches with their content hash | Requirements | VISION_V2:77–78; CD-12; decision map part j, J3 ("recorded matches keep them possible") | design doc; process | yes |
| 21 | §6.9 a few very large units: the per-unit caps (GPU part 4; AP-5) | Requirements | VISION_V2:79; GPU part 4 Need (§6.9); AP-5 Need (§6.9) | restating | yes |
| 22 | §6.10 accounts `[LATER]`; tracks reset every match and need no persistence (AB-8); the unlock question listed (OQ-0005-AB5) | Requirements | VISION_V2:80; AB-8 ("nothing is saved: tracks need no persistence (accounts are `[LATER]`"); OQ-0005-AB5 | design doc; restating | yes |
| 23 | VISION_V2 §2 at lines 25–40: pillar 3 "every strong thing has an answer", pillar 4 "Every choice costs something", pillars 1 and 2 `[OPEN]` | Requirements | VISION_V2:31–32, 33, 29–30 | design doc | yes |
| 24 | §3 at lines 42–49: two-team and three-team battles both tested; modes are data | Requirements | VISION_V2:45–46, 49 | design doc | yes |
| 25 | §4 at lines 51–55: four desktop browsers; minimum laptop at Low; recommended machine at High | Requirements | VISION_V2:52–54 | design doc | yes |
| 26 | LOOP_V2 line 8 quote "Every number here is a starting value. Keep them all in one data table so they can be tuned without code changes" | Requirements | LOOP_V2:8 | design doc | yes |
| 27 | LOOP_V2 §4 to §6 abilities, tiers, counters; §7 feel kit, active reload, Feel Lab; §8 three unlock gates | Requirements | LOOP_V2 §4 (39–61), §5 (62–80), §6 (81–88), §7 (89–96), §8 (97–114); the ability section's Context | design doc | yes |
| 28 | EDITOR_V2 §2 rules 1, 2, 5 (3 and 4 listed for the editor's own plan); §3 objectives as data, validation panel; §4 every v1 map migrates, 1,000 commands undone and redone give a byte-identical document | Requirements | EDITOR_V2:15–26, 37–40, 47–48; CD-11 ("Not here") | design doc; restating | yes |
| 29 | LOOK_V2 §8 at lines 65–74: models `[DECIDED]`; flow and budgets `[DEFAULT]`; "The game never runs Spline's own runtime"; tree style `[OPEN]` | Requirements | LOOK_V2:66, 67–72, 73, 74 | design doc | yes |
| 30 | Items a model carries: silhouettes (§1, §5), uniform details (§2), proportions and weapon silhouettes (§3), emissive accents (§6.4) | Requirements | the asset pipeline section's Context; the GPU section's part 2 (§6.4) | restating | yes |
| 31 | The stack: one Rust simulation, native on the server and Wasm in the browser's Worker | Settled inputs | CLAUDE.md, "The v2 rebuild" ("a Rust simulation core compiled to WebAssembly for the browser and to native code for the server"); brief §1 | settled | yes |
| 32 | UPGRADE-SCOPE's clauses as summarized (track per unit and weapon type, pick one of three, owned by the player, survives death and switching, resets every match, class comes with its own weapon, other weapons only from killed enemies' drops as world pickups, progress on the weapon type, PlayerState not Pawn; open: carry slots, looted ammo, levelling speed) | Settled inputs | inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md:10–17 | settled | yes |
| 33 | docs-keeper records UPGRADE-SCOPE in STATUS with the day's decisions | Settled inputs | plan.md step 11; advisor condition 12 | process | yes |
| 34 | SLICE-ORDER and its three conditions | Settled inputs | STATUS:94–101 (at 9e7db28) | settled | yes |
| 35 | TICK-30: a 30 Hz server tick at the baseline | Settled inputs | STATUS:118–120 | settled | yes |
| 36 | MODES-3 (a): the largest mode is 24v24, N = 48 | Settled inputs | STATUS:104–105 ("the biggest match is 24v24 = 48") | settled | yes |
| 37 | SEC-SPREAD (a): weapon spread in an authority-only stream | Settled inputs | STATUS:108–109 ("server-only spread"); the term per GLOSSARY:525–545 (retire "server-only random stream", line 538) | settled | yes |
| 38 | ABL-PREDICT open until the Feel Lab test | Settled inputs | STATUS:110, 122–123 | settled | yes |
| 39 | BASELINE-WHEN (b): the security baseline listed open in ADR 0006 | Settled inputs | STATUS:112–113 | settled | yes |
| 40 | SPIKE-SCOPE (a) and R7-MARKS (a) are settled | Settled inputs | STATUS:187–188; 159–160 | settled | yes |
| 41 | LOOK_V2: authored glTF/GLB models allowed in v2 | Settled inputs | LOOK_V2:66 (`[DECIDED]`); plan "Settled inputs" | settled | yes |
| 42 | ASSET-RESEARCH (a) as stated (flow as given; only R1-DATA, R3-CHAR, R3-BUDGET; five items open "not researched"; one research unit by graphics-engineer, verified like R3, own daytime session; SPIKE_PLAN places it before the asset spike) | Settled inputs | advisor-verdicts.md §1, condition 4; plan.md "Rulings" | process (advisor's ruling) | yes |
| 43 | GLOSSARY §8: the foundation includes "TypeScript types generated from the Rust code" | Settled inputs | decision map part j, J1 (the §8 text, from brief §2) | settled | yes |
| 44 | The netcode proof test runs before the foundation (SLICE-ORDER, condition 3) | Settled inputs | STATUS:101 | settled | yes |
| 45 | The netcode proof test is throwaway and needs none of this record's machinery; the cue-once check it carries for R1-ABL-OQ3 is ADR 0001's mark | Settled inputs; Migration 1 | ADR 0001 part 14 (netcode-architect-adr0001-draft.md:400–406: throwaway, in `spikes/`, its research spikes, no content need); the ability section's Appendix row R1-ABL-OQ3; the content section's "For the frame" (migration) | proposed (reading) | yes |
| 46 | Condition 11: escalation category 2; shows the engine can carry Mustafa's answers | Settled inputs | advisor-verdicts.md §1, condition 11 | process | yes |
| 47 | Sections placed verbatim; frame adds the joined parts; disagreements as OQ-0005-1 and -2, fixed by each author in the fix round | How put together | brief; plan.md step 9 ("Each author fixes only its own text") | process | yes |
| 48 | Interfaces: I-1 (ADR 0004 executes), I-5 content part (ADR 0002's `packages/types`), I-6 (ADR 0006 serves, ADR 0001 names at join), I-7 (room ADR 0001), I-8 (ADR 0003 draws), I-9, I-21 | Where it sits | decision map part d (map lines 802–822) | process | yes |
| 49 | I-21's carriers: player record ADR 0004 part 18; messages and visibility ADR 0001 part 12; telemetry fields ADR 0006 part 12; the offer on the page ADR 0003 part 10 | Where it sits; Decision 6 | rust-reviewer-adr0004-draft.md:426–446; netcode-architect-adr0001-draft.md:373–390; infra-engineer-adr0006-draft.md:325–341; graphics-engineer-adr0003-draft.md:373–380 | restating (other drafts) | yes |
| 50 | Takes I-14 (ADR 0002), I-20 (ADR 0004 emits), I-4 (ADR 0001) | Where it sits | decision map part d | process | yes |
| 51 | Seams S5, S6, S9, S10 | Where it sits | decision map part a, "The seams, settled" (map lines 207–212) | process | yes |
| 52 | The joined decision paragraph (JSON, one definition per file, one crate, one verdict, schema and types from Rust types, versions, one hashed bundle named at join and recorded; GAS subset; weapons' bookkeeping; tiers as tag gates; tracks as a kind; models listed, decide nothing, no look, slots by class, caps, hashed immutable files; rest waits for research) | Decision | CD-1 to CD-8; AB-1 to AB-5, AB-8; AP-1 to AP-5; GPU parts 1, 2, 4 | restating | yes |
| 53 | In development every save checked at once; client content live, simulation numbers next match (proposed; OQ-0005-CD4) | Decision | CD-7 | restating | yes |
| 54 | Part 1's pointers and Need (§6.1, §6.3, §6.5, §6.7) | Decision 1 | CD-1 to CD-5 Need lines | restating | yes |
| 55 | Part 2's pointers; Need §6.1, §6.3 to §6.8; feel kit LOOP_V2 §7 only; quality-tier table VISION_V2 §4 only | Decision 2 | CD-6 (§6.1, §6.3, §6.4, §6.5, §6.7), CD-12 (§6.6, §6.8), CD-13 (§6.7), CD-14 Need lines | restating | yes |
| 56 | Part 3's pointers; Need §6.1, §6.3, §6.5, §6.6, §6.7; reload LOOP_V2 line 8 and §7 only | Decision 3 | CD-7 to CD-10 Need lines | restating | yes |
| 57 | Part 4: map document; Need EDITOR_V2 §2, §4; VISION_V2 §6.1, §6.2 | Decision 4 | CD-11 Need line | restating | yes |
| 58 | Part 5's pointers; Need §6.3, §6.4, §6.5, §6.7; the cost mark VISION_V2 §3 and §4 only | Decision 5 | AB-1 to AB-10 Need lines (AB-10: "VISION_V2 §3 and §4 only") | restating | yes |
| 59 | Part 6: track kind, drop and pickup values (AB-4), AB-8, definitions name tracks (AB-4, CD-6); Need UPGRADE-SCOPE | Decision 6 | AB-4, AB-8, CD-6 | restating | yes |
| 60 | Part 7's pointers; Need LOOK_V2 §8; VISION_V2 §4, §6.2, §6.4, §6.7, §6.9; EDITOR_V2 §4 | Decision 7 | AP-1 to AP-7 Need lines | restating | yes |
| 61 | Part 8's pointers; Need LOOK_V2 §2, §3, §5, §6.4, §8; VISION_V2 §4, §6.4, §6.7, §6.9; EDITOR_V2 §2 | Decision 8 | GPU parts 1–9 Need lines | restating | yes |
| 62 | Part 9: the spikes listed | Decision 9 | the four spike parts; decision map part g | restating; process | yes |
| 63 | Sections' option sets: content Questions 1–7; ability A, B, E; asset pipeline three questions; GPU four | Options intro | the sections' Options headings | restating | yes |
| 64 | F1-A: one Rust content crate, typed all the way, one verdict everywhere (R1's option Q2 A) | Options F1 | r1-simulation-core.md:1845–1850; adopted as R1-DATA-REC2 (r1:2500–2516) | research verified | yes |
| 65 | The Worker's Wasm carries the whole loader with no size budget yet | Options F1; Trade-off | R1-RR-OQ5 (r1:2946–2950: "R1 records the size but sets no limit"; "the content crate in the Worker brings serde_json, garde and regex") | research, source to check (a reviewer's record; no outside source) | yes |
| 66 | Server-side validation by construction | Options F1 | R1-DATA-F25 (r1:1548–1554), verified (r1:3501–3538) | research verified | yes |
| 67 | "two validators of one schema can disagree" | Options F1 | r1-simulation-core.md:1852 | research, source to check (options text) | yes |
| 68 | CC9 silent fallback; CL14 unchecked tag contract | Options F1; Trade-off | LEGACY_INVENTORY.md lines 138, 123 | repo doc | yes |
| 69 | The split per family is not researched beyond R1's option B | Options F1 | r1 Options §6 (1834–1882) read: no per-family option | proposed (gap statement) | yes |
| 70 | CD12 legacy fact quote "separate files, or separate sections, for what the simulation decides and what the client only shows" | Options F2; OQ-0005-1 | r1-simulation-core.md:1600–1602 | research (legacy fact) | yes |
| 71 | CD13's note "Avoid: bot-only fields mixed into the player class, and a colour (accent) in sim data" | Options F2; OQ-0005-1 | LEGACY_INVENTORY.md line 94 | repo doc | yes |
| 72 | The feel kit is a separate client definition keyed by the same id | Options F2; OQ-0005-1 | AB-3; CD-14 | restating | yes |
| 73 | Both F2 options apply R1-DATA-REC8's simulation and client groups | Options F2; OQ-0005-1 | r1-simulation-core.md:2596–2598 | research verified | yes |
| 74 | The map document holds both sides in one document (EDITOR_V2 §2 rule 1) | Options F2; OQ-0005-1 | EDITOR_V2:16–17; CD-11 (lighting mood, asset references in the document) | design doc; restating | yes |
| 75 | The frame leans to (b) for definitions | Options F2; OQ-0005-1 | reason: the feel kit already takes it, one place for a weapon's presentation, CD13's note | proposed | yes |
| 76 | F3-(a): one hash over the whole set, as CD-8 and AP-4 write it; R1-DATA-REC6 | Options F3; OQ-0005-3 | r1-simulation-core.md:2565–2575; CD-8; AP-4 ("Because the asset list sits in the content set, the content hash names the asset versions a build uses"); CD-6 (asset list is client content) | research verified; restating | yes |
| 77 | An art-only change makes a new hash; clients fetch the new bundle before joining; the record starts a new tuning group | Options F3; OQ-0005-3 | follows from CD-6 + CD-8 (the Worker checks its bundle against the named hash) + R6-TEL-REC3 (grouped by content hash) | arithmetic (logical consequence) | yes |
| 78 | F3-(b) not researched, so not proposable | Options F3; OQ-0005-3 | plan rule 3; no option in r1 Options §6 Q5 (1876–1882) or R6 | process; proposed | yes |
| 79 | Pick and win rates "always grouped by content hash and mode id"; "pick and win rates are only comparable within one tuning version" | OQ-0005-3 | r6-infrastructure.md:828, 835–836 (R6-TEL-REC3, ADR 0006's) | research, source to check (R6 recommendation text; quotes checked) | yes |
| 80 | Links: a unit names weapons, abilities and track (AB-4) and model (AP-2); a faction names units and vehicles (CD-13); an ability names effects and tags; a track names candidates (AB-8) | Trade-off | AB-1, AB-4, AB-8, AP-2, CD-13 | restating | yes |
| 81 | Legacy's silent fallbacks and unchecked links CC9, GE11, CL14 | Trade-off | LEGACY_INVENTORY.md lines 138, 297, 123; CD-2's "Why" | repo doc | yes |
| 82 | Tracks need no new operation | Trade-off | AB-8 ("and no new operation") | restating | yes |
| 83 | No shipped-game source was read for the data unit | Trade-off | r1-simulation-core.md:1577–1579 | research verified (the research's own method note) | yes |
| 84 | Every GAS fact from one community write-up | Trade-off | R1-ABL-OQ1 (r1:2756–2758); r1:1185–1189 | research verified (method note) | yes |
| 85 | The research has nothing on the asset step | Trade-off | plan.md Risks 3 (0 hits for KTX2, meshopt, Draco, Spline, gltf-transform, Basis; 1 glTF mention); advisor-verdicts §1 "What I checked myself" | process (advisor-checked) | yes |
| 86 | AB-10's curve; a separate upgrade system not researched, not proposed | Trade-off | AB-10; the ability section's Options intro | restating | yes |
| 87 | A room is a mode id plus allowed overrides (R2-ROOM-REC7, ADR 0001's) | Trade-off | r2-netcode.md:2857–2867 | research verified | yes |
| 88 | Balance checked per mode (VISION_V2 §3) | Trade-off | VISION_V2:45–46; the ability section's Trade-off | design doc | yes |
| 89 | Easier list | Consequences | content section "For the frame" Consequences; ability section "For the frame" Consequences; GPU parts 1 and 7 | restating | yes |
| 90 | Harder list, incl. "the 1 ms p99 mark of the ability, effect and weapon step" | Consequences | AB-10; R1-ABL-REC6 (r1:2465–2472) | restating (research verified underneath) | yes |
| 91 | Revisit-when list | Consequences | both "For the frame" lists; AS6 | restating | yes |
| 92 | Hard-constraint quotes "No external assets (procedural textures, synthesized audio)", "Animation is cosmetic … Nothing animated decides a hit", "Persistence only through `KVStore`" | Consequences | CLAUDE.md:48, 50, 49 | repo doc | yes |
| 93 | LOOK_V2 §8 quote "The legacy rule 'no external assets' ends with the legacy engine" | Consequences | LOOK_V2:66 | design doc | yes |
| 94 | Synthesized audio is ADR 0003's matter | Consequences | graphics-engineer-adr0003-draft.md:665 ("synthesized sound recipes still port (part 9)") | restating (other draft) | yes |
| 95 | ADR 0001 and ADR 0006 say what changes for KVStore (guest secret; server-side stores) | Consequences | netcode-architect-adr0001-draft.md:827–828; infra-engineer-adr0006-draft.md:576 | restating (other drafts) | yes |
| 96 | Classic scripts, double-click, layering are ADR 0002's; r128, `LinearEncoding`, `stylizeScene()` ADR 0003's | Consequences | engine-architect-adr0002-draft.md Consequences (the hard-constraint list); graphics-engineer-adr0003-draft.md:662–666 | restating (other drafts) | yes |
| 97 | Spikes exempt under SPIKE-SCOPE (a); nothing in CLAUDE.md or `.claude/` changes in this PR | Consequences | STATUS:187–188; plan.md rule 11 | settled; process | yes |
| 98 | ADR 0003 leaves the CM rows, CL13, CD20, CD21, GW18, GW19 and the other asset rows to this record | Consequences (inventory) | graphics-engineer-adr0003-draft.md "Legacy inventory rows" intro (lines 672–677) | restating (other draft) | yes |
| 99 | The seven copied inventory rows | Consequences (inventory) | engine-architect-adr0005-content-section.md "For the frame" (lines 853–859); its ledger | restating (copied by script) | yes |
| 100 | ED5 (line 340) and ED7 (line 342): port to TypeScript, Look brief pending | Consequences; OQ-0005-4 | LEGACY_INVENTORY.md:340, 342 | repo doc | yes |
| 101 | No draft had placed ED5 and ED7 | Consequences | grep of the five drafts for ED5/ED7: 0 hits (graphics-engineer-adr0003-draft.md, the others); the content section's "For the frame" ("the frame or ADR 0003 places them") | process (checked by grep) | yes |
| 102 | ED5's lab drives the procedural character builder | Consequences; OQ-0005-4 | LEGACY_INVENTORY.md:340 ("the tool drives the real game builder (buildEntity + applyFaction)") | repo doc | yes |
| 103 | ADR 0003 proposes authored, skinned soldiers (CL10, CC1–CC5); GA1–GA15 port | Consequences; OQ-0005-4 | graphics-engineer-adr0003-draft.md inventory rows (CL10, CC1–CC5 redesign; GA1–GA15 port) | restating (other draft) | yes |
| 104 | EDITOR_V2: "The Character Lab and Anim Lab stay as they are" (lines 4–5) | Consequences; OQ-0005-4 | EDITOR_V2:4–5 | design doc | yes |
| 105 | ED7's note: tuning as "a data file the lab writes and the client loads" | Consequences; OQ-0005-4 | LEGACY_INVENTORY.md:342 | repo doc | yes |
| 106 | Rows that keep their marks: CD4, CD6, CD10, CD12, CD14 rewrite in Rust and CD7 redesign (dead keys via CD-10); CD8, CD13, GC4, GC5, GC7, GC11, GC14, GC15, GC17 rewrite in Rust; GW2, GE1, GW12, GE11, CM1, ED13 redesign; CD2 port; TL11 drop; CS3 drop; CC9 port; GE13 redesign; GE17 drop | Consequences | LEGACY_INVENTORY.md rows (marks read by grep, 2026-11-01); dead keys r1:1594–1597 | repo doc | yes |
| 107 | CL4, CL14, GC16 are ADR 0003's rows; the pending notes on GC11, GC14, GC15, CC9, CC12 are ADR 0003's | Consequences | graphics-engineer-adr0003-draft.md inventory table (rows "CL2, CL4, CL8, CL14", "GC16"); the ability section's "For the frame" | restating | yes |
| 108 | The 30 section OQs, one line each, with deciders and proposals | Open questions | the four sections' Open questions (CD1–CD8; AB1–AB13; AP1–AP3; AS1–AS6) | restating | yes |
| 109 | OQ-0004-5 asks what AB9 asks; OQ-0004-6 what AB10 asks | Open questions | rust-reviewer-adr0004-draft.md:773–790 | restating (other draft) | yes |
| 110 | AS3 candidate: a very large unit's model names a mesh per vehicle part; GPU part 3 asks names to match; AB-4 defines parts; AP-2 checks only asset ids and slot classes | Open questions | GPU part 3; AB-4 (vehicle kind: parts); AP-2 | restating; proposed (an input to the research, decides nothing) | yes |
| 111 | OQ-0005-1's quotes: AP-2 "a unit, vehicle, weapon, faction uniform, building part or prop definition names its model by an asset id"; CD-13 "its units, vehicles and colours"; CD-6 "takes the simulation part by type, so a presentation value cannot reach an outcome" and "This section adds no field to either"; AB-3 "a separate client definition keyed by the same id"; AB-4's unit kind names no model | OQ-0005-1 | engine-architect-adr0005-content-section.md (AP-2 line 627; CD-13 line 296; CD-6 lines 181–182, 189); game-designer-adr0005-abilities-section.md (AB-3 lines 125–126; AB-4 line 137) | restating (quotes compared) | yes |
| 112 | OQ-0005-2's quotes: CD-6 "a weapon definition is a weapon type and names its upgrade track, so progress belongs to the type, not to the gun"; AB4 (b) examples (the Luger in two classes; variants of one weapon); ADR 0004 part 18 "the type ids ADR 0005 defines" | OQ-0005-2 | content section line 187–188; abilities section line 441; rust-reviewer-adr0004-draft.md:428–429 | restating (quotes compared) | yes |
| 113 | OQ-0005-3's decider: the Advisor with the infra-engineer (ADR 0006 owns R6-TEL-REC3) | OQ-0005-3 | engine-architect-decision-map-ids.tsv (R6-TEL-REC3 owner 0006) | process | yes |
| 114 | OQ-0005-4's options (a) to (c), not researched; LOOK_V2 §3's proportions | OQ-0005-4 | LOOK_V2 §3 ("slimmer, serious character proportions", per ADR 0003's table); options are the frame's listing | proposed (listing, no pick) | yes |
| 115 | ADR 0004 part 16 quote "a development reload swaps them between ticks, never inside one (R1-DATA-REC5, ADR 0005's)" vs CD-7 | Open questions (consistency) | rust-reviewer-adr0004-draft.md:412–413; CD-7 | restating (other draft) | yes |
| 116 | AB5 says "ADR 0006 cites this"; ADR 0006 part 12 does not name it | Open questions (consistency) | abilities section line 459; grep "unlock" in infra-engineer-adr0006-draft.md: 0 hits | process (checked by grep) | yes |
| 117 | The content and ability spike after the sim core spike and before slice 1 | What a spike must prove 1 | decision map part g, the proposed order (map lines 962–965) | process | yes |
| 118 | Content items 1–9 and ability items 1–7 in that spike; item 10 is ADR 0001's mark | Spike 1, 4 | content section "What a spike must prove" (intro, item 10); ability section (intro: "All in the content and ability spike") | restating | yes |
| 119 | Two pairs share a mechanism: content item 2 (R1-DATA-REC2) with ability item 3 (R1-ABL-REC5); content item 5 with ability item 1's last clause | Spike 1 | content items 2, 5; ability items 1, 3 | restating; proposed (may share a run; the test-engineer decides) | yes |
| 120 | The look proof test's asset items as summarized; Mustafa judges each family there beside legacy screenshots | Spike 2 | GPU spike part, first bullet; asset pipeline spike part, first bullet; OQ-0005-AP2; LOOK_V2 §9 (lines 76–78) | restating; design doc | yes |
| 121 | The asset spike after the asset-pipeline research: both second bullets | Spike 3 | GPU and asset pipeline spike parts | restating | yes |
| 122 | Other spikes: R1-DATA-OQ8 in the boundary spike; R1-ABL-OQ3 in the netcode proof test (R1-ABL-REC4's mark, ADR 0001's); the Feel Lab test for ABL-PREDICT (ability item 8) | Spike 4 | content item 5; the ability section's Appendix row R1-ABL-OQ3; ability item 8 | restating | yes |
| 123 | The "### Pass marks" placeholder exactly as assemble.py expects | Spike | scripts/assemble.py PH regex: 1 hit | process (script-checked) | yes |
| 124 | The legacy game stays playable in `legacy/` (ADR 0002); v2 playable from slice 1 | Migration | engine-architect-adr0002-draft.md:9–10; SLICE-ORDER | restating; settled | yes |
| 125 | Slice items (foundation, spike, slices 1–6, assets) | Migration | content section "For the frame" Migration; ability section "For the frame" Migration | restating | yes |
| 126 | The "Done when" lines; slice 1's list moved content item 4 to slice 2 | Migration | reason in the text (each names the items that test its slice; maps migrate in slice 2, per the same list) | proposed | yes |
| 127 | Slice 5 done when: EDITOR_V2 §4's bars (1,000 random commands byte-identical; every v1 map migrates) | Migration | EDITOR_V2:47–48 | design doc | yes |
| 128 | The asset-pipeline research and the asset spike come before the first authored model enters the game | Migration | decision map part g (line 965); ASSET-RESEARCH (a); content section "For the frame" | process; restating | yes |
| 129 | Test plan items (CI checks, ability goldens, R1-ABL-REC3's log: a 120 s scenario, 3 seeds, native and Wasm; sims 3 runs per build, two- and three-team modes) | Test plan | both "For the frame" test plans; R1-ABL-REC3 mark (r1:2416–2419); CLAUDE.md change-set protocol 4 ("tools/sim.js ×3 runs") | restating; research verified | yes |
| 130 | No spike starts before approval; spikes throwaway | Rollback | template 0000 ("No spike starts before Mustafa and the Advisor have reviewed and approved the draft"); SPIKE-SCOPE (a) | repo doc; settled | yes |
| 131 | Rollback items | Rollback | content section "For the frame" Rollback; ability section "For the frame" Rollback; AP-7 | restating | yes |
| 132 | 34 ids owned by 0005: 26 + 7 + 1 | Appendix A | engine-architect-decision-map-ids.tsv (awk owner 0005: 34); the sections' Appendix rows (26, 7, 1); scripts/dispositions.py on the draft: disposed 34, more than once 0, not where the map puts them 0 | arithmetic; process (script-checked) | yes |
| 133 | Counts: adopted 6 (4 + 2) · changed 5 (4 + 1) · answered 8 (7 + 1) · open 9 (6 + 2 + 1) · spike 6 (5 + 1) · rejected 0; total 34 | Appendix A | the sections' rows, counted | arithmetic | yes |
| 134 | "spike mark n" in the content rows is the content section's item n; the ability rows' "What a spike must prove" n is that section's item n | Appendix A | the rows' wording | restating | yes |
| 135 | Sources list and the line ranges in it | Sources | each range re-opened with sed on 2026-11-01 (r1:1577–1579, 1600–1602, 1826–1832, 1845–1855, 2565–2575, 2596–2606, 2756–2758, 2946–2950; r6:828–836; inventory 340, 342) | process | yes |

Counts by tier (a row may carry two tiers; counted by its first, by script): restating 55 · process 20 · design doc 19 ·
settled 13 · repo doc 9 · research verified 7 · proposed 5 · research, source to check 3 · arithmetic 3 · research
(legacy fact) 1 (135 rows). Script checks on the draft: `scripts/cite_check.py` 0 ids missing; 2 line-range warnings, both inside the
ability section (multi-range citation groups: R1-ABL-OQ1 beside lines 1185–1189; R1-ABL-REC6 beside GLOSSARY lines
397–418), heuristic false positives, the frame's text has none.
