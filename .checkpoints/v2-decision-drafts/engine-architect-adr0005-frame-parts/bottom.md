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
  Wasm size is unbudgeted until the spike measures it (OQ-0005-CD2). **B.** Pros: smaller pieces. Cons: no check sees
  the links between families, which is where legacy failed silently (CC9, CL14). The split per family is not
  researched beyond R1's option B.

**Question F2: where presentation data sits beside a simulation definition** (asset ids, faction colours; R1's legacy
fact CD12: "separate files, or separate sections, for what the simulation decides and what the client only shows",
r1-simulation-core.md, Findings §6, lines 1600–1602)
| Option | Complexity | Cost (money, time, usage) | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort from the current code |
|---|---|---|---|---|---|---|
| a. A presentation section in the simulation definition's own file, typed apart from the simulation part (how AP-2 and CD-13 read today) | Low: one file per unit | none | – | presentation only, if the sim's types leave the section out | a unit's numbers and its model in one file | close to legacy's mixed tables; CD13's note says "Avoid: bot-only fields mixed into the player class, and a colour (accent) in sim data" (`docs/LEGACY_INVENTORY.md` line 94) |
| b. A client definition keyed by the simulation definition's id, as the feel kit is (AB-3, CD-14) | Low: one more file for each definition with a presentation side | files kept in step | – | presentation only; no simulation type holds a presentation field, so CD-6's split holds by kind | an art or feel change touches only client files | new; the feel kit already works this way |

- Both options apply R1-DATA-REC8's simulation and client groups (r1, lines 2596–2598). The map document holds both
  sides in one document either way (EDITOR_V2 §2, rule 1). The frame leans to (b) for definitions, because the feel
  kit already takes it and a weapon's presentation (feel kit, sockets, model) then sits in one place. The sections
  disagree today, so this is OQ-0005-1, and each author fixes its own text in the fix round.

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
- **Why one model:** a unit names its weapons, abilities and track, and its model (AP-2); a faction names its units
  and vehicles; an ability names its effects and tags; a track names its candidates. One content crate checks every one
  of those links at load, on the server, in the Worker and in the editor alike (CD-2, AB-4, AP-2), so the silent
  fallbacks and unchecked links legacy shipped (CC9, GE11, CL14) cannot return, and one hash names the numbers every match used (CD-8). The ability model, the upgrade tracks and the asset
  list add kinds to that model rather than machinery beside it: the tracks need no new operation (AB-8), and the asset
  part proposes only what R1-DATA, R3-CHAR and R3-BUDGET support (ASSET-RESEARCH (a)).
- **The strongest argument against:** the joined decision rests on documentation, not on shipped games. No
  shipped-game source was read for the data unit (r1-simulation-core.md, lines 1577–1579); every GAS fact comes from
  one community write-up (R1-ABL-OQ1, r1, lines 2756–2758); and the research has nothing at all on the asset step (the
  asset sections' gap). One crate and one hash also tie three families together: an art change changes the hash every
  match records (OQ-0005-3), and the Worker's Wasm carries the whole loader before any size budget exists (R1-RR-OQ5;
  OQ-0005-CD2). The content and ability spike and the asset spike are where this is proven or refuted, and each
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
  slot class, so batching survives authored art (the GPU section's parts 1 and 7).
- **Harder:** every mechanic outside the closed set is a reviewed Rust change, knockback the first (AB-6); designers
  must know the aggregation order and the stacking policies (AB-1); upgrade tracks add long-lived active effects per
  soldier, which the 1 ms p99 mark of the ability, effect and weapon step must absorb (AB-10); a Rust type and a
  reviewed length bound for every collection (CD-2); JSON without comments (OQ-0005-CD1); a simulation number tested in
  a new match, not the current one (CD-7); two generators that must agree (CD-3); an asset list kept in step with the
  files (AP-2); every material slot named into a class (OQ-0005-AS4); two drawing paths for buildings and props until
  OQ-0005-AP2 is answered (AP-7).
- **Revisit when:** the content and ability spike's load time or Wasm size misses its budget (OQ-0005-CD2, option
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
  as engine-architect, the author of the content data and asset pipeline sections, marks them, plus ED5 and ED7,
  which no draft had placed:

  | Row id (`LEGACY_INVENTORY.md` line) | Mark now | Proposed mark | Why |
  |---|---|---|---|
{{INV_ROWS}}
  | ED5 (340) | port to TypeScript (pending) | unchanged here; pending becomes "OQ-0005-4: what the lab works on once soldiers are authored models" | the lab drives the procedural character builder, which ADR 0003 proposes to replace with authored, skinned soldiers (its rows CL10, CC1–CC5); EDITOR_V2 keeps the lab (lines 4–5) |
  | ED7 (342) | port to TypeScript (pending) | unchanged here; pending becomes "OQ-0005-4" | EDITOR_V2 keeps the lab (lines 4–5); it tunes the cosmetic animation that ADR 0003's GA1–GA15 rows port, and its own note asks for "a data file the lab writes and the client loads", which client content (CD-6) can hold |

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
| OQ-0005-CD2 | What load time and Wasm size may content cost, and is JSON fast and small enough? | the content and ability spike measures; the Advisor sets the budgets | Content data |
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
| OQ-0005-AB9 | Which drop and pickup rules does UPGRADE-SCOPE leave unstated? (ADR 0004 asks the same as OQ-0004-5) | Mustafa | The ability and effect system |
| OQ-0005-AB10 | Do bots have tracks and make picks, and what does a human get when taking a bot's slot? (ADR 0004 asks the same as OQ-0004-6) | Mustafa | The ability and effect system |
| OQ-0005-AB11 | Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1? | Mustafa, after sims of each matchup | The ability and effect system |
| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? (proposed: accept it) | the Advisor | The ability and effect system |
| OQ-0005-AB13 | When does scripting (option A3) come back? (proposed: if the spike's ability mark 1 or 2 fails, or new operations are needed often; the talk is read when a copy appears) | the Advisor | The ability and effect system |
| OQ-0005-AP1 | Where does the automated step run, what does it write, and where do the files live? | the asset-pipeline research, then the asset spike | The asset pipeline |
| OQ-0005-AP2 | Which asset families become authored models, and which stay procedural builders? | Mustafa, in the look proof test | The asset pipeline |
| OQ-0005-AP3 | May an AI-generated draft model ever ship? | Mustafa | The asset pipeline |
| OQ-0005-AS1 | Which compression formats for textures and meshes? | the asset-pipeline research, then the asset spike | The asset pipeline: GPU rules |
| OQ-0005-AS2 | Which tool builds the LOD levels, and how many levels? | the asset-pipeline research, then the asset spike and Mustafa | The asset pipeline: GPU rules |
| OQ-0005-AS3 | Which checks does an export pass before it enters the game? | the asset-pipeline research | The asset pipeline: GPU rules |
| OQ-0005-AS4 | How are a model's materials renamed into the slot classes and faction colour slots? | the asset-pipeline research | The asset pipeline: GPU rules |
| OQ-0005-AS5 | What are the per-unit budgets beyond the two derived caps? | the asset-pipeline research, then the render budget bench | The asset pipeline: GPU rules |
| OQ-0005-AS6 | How many vertices and bones does the authored soldier have? | the model's author, then the render budget bench | The asset pipeline: GPU rules |
| OQ-0005-1 | Do presentation references sit on the simulation definitions or in client definitions keyed by the same id? | the Advisor, at the stop review, on the fix round's text | this frame |
| OQ-0005-2 | Is a weapon definition the same thing as a weapon type? | Mustafa (through OQ-0005-AB4), then the fix round | this frame |
| OQ-0005-3 | What does the content hash cover? | the Advisor | this frame |
| OQ-0005-4 | What do the Character Lab and the Anim Lab work on once soldiers are authored models? | Mustafa with the Advisor | this frame |

One candidate for OQ-0005-AS3's list, which the frame adds as an input to that research and decides nothing about: a
very large unit's model names a mesh for every part of its vehicle definition (the GPU section's part 3 asks the names
to match; AB-4 defines the parts), since no load check covers that link today (AP-2 checks asset ids and slot classes
only).

The frame's own questions, in full:

- **OQ-0005-1. Do presentation references sit on the simulation definitions, or in client definitions keyed by the
  same id?**
  - The disagreement: AP-2 has "a unit, vehicle, weapon, faction uniform, building part or prop definition names its
    model by an asset id", and CD-13 has a faction definition name "its units, vehicles and colours". CD-6 classes
    units, vehicles, weapons and factions as simulation content, whose step "takes the simulation part by type, so a
    presentation value cannot reach an outcome", and says of the weapon and unit definitions "This section adds no
    field to either"; the ability section's unit kind names no model (AB-4), and its feel kit is "a separate client
    definition keyed by the same id" (AB-3).
  - Why it matters: it decides whether CD-6's split holds by kind or only by a rule about fields, and whether a
    weapon's presentation (feel kit, sockets, model) sits in one place or two. The legacy lesson points the same way:
    CD13's note says to avoid "a colour (accent) in sim data" (`docs/LEGACY_INVENTORY.md` line 94).
  - Options: (a) a presentation section in the definition's own file, typed apart from the simulation part; (b) a
    client definition keyed by the simulation definition's id, as the feel kit is (Options considered, Question F2,
    with their trade-offs). The map document keeps both sides in one document either way (EDITOR_V2 §2, rule 1).
  - The frame's lean: (b) for definitions, for the reasons under Question F2.
  - Decider: the Advisor, at the stop review, on the text the authors write in the fix round (engine-architect for
    AP-2, AP-3, CD-6 and CD-13; the game-designer if AB-4's kinds change). What decides it: whether CD-6's split can
    be checked by type alone. Research ids: none. Both options apply R1-DATA-REC8's groups (r1-simulation-core.md,
    lines 2596–2598) and the legacy fact CD12 (r1-simulation-core.md, lines 1600–1602).
- **OQ-0005-2. Is a weapon definition the same thing as a weapon type?**
  - The disagreement: CD-6 says "a weapon definition is a weapon type and names its upgrade track, so progress belongs
    to the type, not to the gun". OQ-0005-AB4 leaves open which definitions share a track, and its option (b) lets
    several definitions share one (the Luger in two classes; the variants of one weapon); CD-6's wording reads as
    AB4's option (a) or (c).
  - Why it matters: UPGRADE-SCOPE ties progress to "the weapon type", and ADR 0004 part 18 keys the player record's
    tracks by "the type ids ADR 0005 defines"; if one type spans several definitions, the key is the track's id, not
    the weapon's.
  - Options: the answer follows OQ-0005-AB4. Under its (a) or (c), a definition is a type, as CD-6 says; under (b), a
    track's id is the type, and CD-6 says only that a weapon definition names its track.
  - Decider: Mustafa answers OQ-0005-AB4; the content data section's author aligns CD-6's wording in the fix round,
    so that it leaves AB4 open. Research ids: none (UPGRADE-SCOPE).
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

For the consistency pass (plan step 8), three points where this record and another draft differ; the frame changes
neither text: ADR 0004 part 16 says "a development reload swaps them between ticks, never inside one (R1-DATA-REC5,
ADR 0005's)", while CD-7 proposes simulation numbers at the next match and keeps the swap as OQ-0005-CD4's option (b);
OQ-0004-5 and OQ-0004-6 ask Mustafa what OQ-0005-AB9 and AB10 ask, so each is one question for him; OQ-0005-AB5 says
"ADR 0006 cites this", and ADR 0006 part 12 does not name it yet.

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
   (slot class, faction) pairs; the two derived caps) and the asset pipeline section's first bullet (every model listed
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
Written by test-engineer (plan step 4).

## Migration plan
The legacy game stays playable in `legacy/` at every step (ADR 0002); v2 is playable from slice 1. The places in
SLICE-ORDER and the "Done when" lines are proposed (reason: each names the spike items that test what its slice brings,
run again on the real crate; slice 1's list is the content data section author's, with the map migration item moved to
slice 2, where the maps migrate).
1. **The netcode proof test** runs first and needs nothing from this record.
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
   7); the load-time and size budgets measured again on the full content (OQ-0005-CD2; content item 9).
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
- JSON misses its budgets: the build compiles the same JSON into a binary runtime format (option D), and JSON stays
  the authored form (OQ-0005-CD2).
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

Counts: adopted 6 · changed 5 · answered 8 · open 9 · spike 6 · rejected 0 (34 rows).

| id | disposition | where or why |
|---|---|---|
{{APPENDIX_ROWS}}

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
