# engine-architect: ADR 0005 second fix round (frame, "Content data", "The asset pipeline")

Fixes against `docs/adr/0005-content-abilities-assets.md` as it stands at 03645ce; every BEFORE was copied from
the file by script and occurs exactly once. The ability section (game-designer) and the GPU section (graphics-engineer)
are not touched, nor the "Pass marks" section (its MAC key line, consistency F4, is left to test-engineer, as the
session asked). Blocks touch disjoint line ranges, so they apply in any order.

### FIX F14 (author line)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
**Author:** engine-architect (frame, content data, the asset pipeline) · game-designer (the ability and effect system) · graphics-engineer (the asset pipeline's GPU rules) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** engine-architect (frame, content data, the asset pipeline) · game-designer (the ability and effect system) · graphics-engineer (the asset pipeline's GPU rules) (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
END

### FIX F111, F112 (frame, how this record is put together)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
one Appendix A. Where two sections disagree, the frame says so (OQ-0005-1, OQ-0005-2) and each author fixes its own
text in the fix round.
AFTER:
one Appendix A. Where two sections disagreed, the frame asks the question (OQ-0005-1, OQ-0005-2); after the fix
round no section answers either, and each points to it.
END

### FIX C53
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- Unchecked merges: `rulesMerge` and `envMerge` take any key, `__proto__` included (GW2, GE1; the STATUS item
  SEC-MDL-2, `docs/STATUS.md` line 257), and the editor's `RULES_SCHEMA` limits bind only its sliders, never a load
  (lines 1603–1607).
AFTER:
- Unchecked merges: `rulesMerge` and `envMerge` take any key, `__proto__` included (GW2, GE1; STATUS, SEC-MDL-2), and
  the editor's `RULES_SCHEMA` limits bind only its sliders, never a load (lines 1603–1607).
END

### FIX F112 (CD-6)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Weapons and units: a weapon definition is a weapon type and names its upgrade track, so progress belongs to the
    type, not to the gun; a unit definition names its class's own weapons (the ability section, AB-4; UPGRADE-SCOPE).
    This section adds no field to either; it holds them in the same format, files and checks as every other kind.
AFTER:
  - Weapons and units: a weapon definition names its upgrade track, and progress belongs to the weapon type, not to
    the gun (UPGRADE-SCOPE); whether each weapon definition is its own type or several definitions share one track is
    OQ-0005-AB4, and this section takes no side (OQ-0005-2). A unit definition names its class's own weapons (the
    ability section, AB-4). This section adds no field to either; it holds them in the same format, files and checks as
    every other kind.
END

### FIX F111 (CD-13)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - What: a faction is a definition (AB-4) that names its units, vehicles and colours. The three T1 classes are written
    once and shared by every faction (LOOP_V2 §5); a faction differs through its own definitions and its uniforms. Its
    soldiers are dressed by (faction, class) as data (CC6's note, `docs/LEGACY_INVENTORY.md` line 136), its uniform
    models are assets named by id (AP-2), and its colours reach the assets' faction colour slots (AP-3; the GPU
    section's part 2). A faction variant is a full definition of its own that references shared definitions by id
    (proposed: no `extends` and no merge, so the merge rules SEC-MDL-2 came from do not return); whether variants may
    override a base definition is OQ-0005-CD5, for the faction design session. Team colours never change (VISION_V2
    §5); their values are ADR 0003's look tuning.
AFTER:
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
END

### FIX F11 (Content data, trade-off)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  serde_json, garde and regex enlarge the Worker's Wasm, which has no budget yet (R1-RR-OQ5, r1, lines 2946–2950,
  ADR 0002's), and garde on wasm32 is unread (R1-DATA-OQ5). The content and ability spike measures load time and size
  before acceptance (OQ-0005-CD2), and option D stays the way out.
AFTER:
  serde_json, garde and regex enlarge the Worker's Wasm, which has no size limit yet (R1-RR-OQ5, r1, lines 2946–2950;
  the one `sim-wasm` budget is ADR 0002's, part 4), and garde on wasm32 is unread (R1-DATA-OQ5). The content and
  ability spike measures load time and the content crate's share of the size before acceptance (OQ-0005-CD2), and
  option D stays the way out.
END

### FIX F11 (OQ-0005-CD2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **OQ-0005-CD2. What load time and Wasm size may content cost, and is JSON fast and small enough?**
  - Why it matters: the research sets no number for either ("against a budget the ADR sets (no number is sourced
    yet)", r1, lines 2497–2498), and the Worker's Wasm has no size budget (R1-RR-OQ5).
  - Options: (a) JSON at run time, with budgets set from the spike's measurement (proposed, as ADR 0002 sets the Wasm
    size budget from its measurement); (b) a binary runtime format compiled from the same JSON (postcard or bincode,
    not read), only if (a) misses its budgets.
  - Decider: the content and ability spike's measurement, then the Advisor sets the budgets. What decides it: the
    measured native and Wasm load times of a content set the size of the imported legacy tables plus two maps and two
    modes, and the size serde_json and garde add to the Wasm module. Research ids: R1-DATA-OQ2 (lines 2791–2794).
AFTER:
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
END

### FIX F11 (Content data, spike item 9)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
9. **OQ-0005-CD2's numbers** (R1-DATA-REC1 mark 2, lines 2496–2498): loading a content set the size of the imported
   legacy tables plus two maps and two modes, on the native server, takes a time the spike reports. Proposed (reason:
   the client loads the same set in its Worker, and R1-RR-OQ5 asks for the size): the same load timed in Wasm in the
   Worker, and the size serde_json and garde add to the Wasm module recorded; the budgets are set from these numbers.
AFTER:
9. **OQ-0005-CD2's numbers** (R1-DATA-REC1 mark 2, lines 2496–2498): loading a content set the size of the imported
   legacy tables plus two maps and two modes, on the native server, takes a time the spike reports. Proposed (reason:
   the client loads the same set in its Worker, and R1-RR-OQ5 asks for the size): the same load timed in Wasm in the
   Worker, and the size serde_json and garde add to the Wasm module recorded; the load-time budget is set from these
   numbers, and the size is content's share in ADR 0002's one `sim-wasm` budget (part 4).
END

### FIX F111 (AP-2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - What: a unit, vehicle, weapon, faction uniform, building part or prop definition names its model by an asset id.
    One asset list in the content set (client content, CD-6) holds for each asset: its id; its file name with its
    hash; its material slots, each with its class (AP-3); its LOD levels, if it has any (the GPU section's part 5);
    and its counts of triangles, materials and textures, which the budget check reads (AP-5). The loader's whole-set
    checks (CD-2) refuse a reference to an asset the list does not hold, and a slot whose class is not in the class
    list, naming the file and the field; nothing falls back to a default model. Who writes the list (the automated
    step or a person), and whether the gate recounts a file or trusts its entry, is OQ-0005-AP1.
AFTER:
  - What: each unit, vehicle, weapon, faction uniform, building part and prop names its model by an asset id. Where
    that id sits is OQ-0005-1: proposed, pending it, in a client definition keyed by the simulation definition's id,
    as the feel kit is (CD-14), so no simulation definition holds an asset id (CD-6); the map document holds its own
    references either way (CD-11). One asset list in the content set (client content, CD-6) holds for each asset: its
    id; its file name with its hash; its material slots, each with its class (AP-3); its LOD levels, if it has any (the
    GPU section's part 5); and its counts of triangles, materials and textures, which the budget check reads (AP-5).
    The loader's whole-set checks (CD-2) refuse a reference to an asset the list does not hold, and a slot whose class
    is not in the class list, naming the file and the field; nothing falls back to a default model. Who writes the
    list (the automated step or a person), and whether the gate recounts a file or trusts its entry, is OQ-0005-AP1.
END

### FIX F111 (AP-3)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    0003 (interface I-8). Adding a class is a content change reviewed together with the ADR 0003 material it needs. A
    faction's colours reach its faction colour slots from the faction definition (CD-13). The table says which slots
    exist; how a class looks is ADR 0003's and Mustafa's (LOOK_V2).
AFTER:
    0003 (interface I-8). Adding a class is a content change reviewed together with the ADR 0003 material it needs. A
    faction's colours reach its faction colour slots from the faction's data (CD-13; where those colours sit is
    OQ-0005-1). The table says which slots exist; how a class looks is ADR 0003's and Mustafa's (LOOK_V2).
END

### FIX F11 (frame, Question F1)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **A.** Pros: one verdict and one hash for everything a match runs on. Cons: one crate grows with every kind, and its
  Wasm size is unbudgeted until the spike measures it (OQ-0005-CD2). **B.** Pros: smaller pieces. Cons: no check sees
AFTER:
- **A.** Pros: one verdict and one hash for everything a match runs on. Cons: one crate grows with every kind, and its
  Wasm size is recorded, not gated, until the spikes have measured it (ADR 0002 part 4; OQ-0005-CD2). **B.** Pros:
  smaller pieces. Cons: no check sees
END

### FIX F111 (frame, Question F2, the table)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| a. A presentation section in the simulation definition's own file, typed apart from the simulation part (how AP-2 and CD-13 read today) | Low: one file per unit | none | – | presentation only, if the sim's types leave the section out | a unit's numbers and its model in one file | close to legacy's mixed tables; CD13's note says "Avoid: bot-only fields mixed into the player class, and a colour (accent) in sim data" (`docs/LEGACY_INVENTORY.md` line 94) |
| b. A client definition keyed by the simulation definition's id, as the feel kit is (AB-3, CD-14) | Low: one more file for each definition with a presentation side | files kept in step | – | presentation only; no simulation type holds a presentation field, so CD-6's split holds by kind | an art or feel change touches only client files | new; the feel kit already works this way |
AFTER:
| a. A presentation section in the simulation definition's own file, typed apart from the simulation part (how AP-2 and CD-13 read before the fix round) | Low: one file per unit | none | – | presentation only, if the sim's types leave the section out | a unit's numbers and its model in one file | close to legacy's mixed tables; CD13's note says "Avoid: bot-only fields mixed into the player class, and a colour (accent) in sim data" (`docs/LEGACY_INVENTORY.md` line 94) |
| **b. A client definition keyed by the simulation definition's id, as the feel kit is (AB-3, CD-14)** (proposed, pending OQ-0005-1) | Low: one more file for each definition with a presentation side | files kept in step | – | presentation only; no simulation type holds a presentation field, so CD-6's split holds by kind | an art or feel change touches only client files | new; the feel kit already works this way |
END

### FIX F111 (frame, Question F2, the note)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- Both options apply R1-DATA-REC8's simulation and client groups (r1, lines 2596–2598). The map document holds both
  sides in one document either way (EDITOR_V2 §2, rule 1). The frame leans to (b) for definitions, because the feel
  kit already takes it and a weapon's presentation (feel kit, sockets, model) then sits in one place. The sections
  disagree today, so this is OQ-0005-1, and each author fixes its own text in the fix round.
AFTER:
- Both options apply R1-DATA-REC8's simulation and client groups (r1, lines 2596–2598). The map document holds both
  sides in one document either way (EDITOR_V2 §2, rule 1). Proposed: (b) for definitions, because the feel kit already
  takes it and a weapon's presentation (feel kit, sockets, model) then sits in one place. The question stays open for
  the Advisor as OQ-0005-1; AP-2, AP-3 and CD-13 state (b) as the proposal pending it and decide nothing.
END

### FIX F111 (frame, Trade-off analysis)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **Why one model:** a unit names its weapons, abilities and track, and its model (AP-2); a faction names its units
AFTER:
- **Why one model:** a unit names its weapons, abilities and track, and its model is named under the unit's id (AP-2;
  where that reference sits is OQ-0005-1); a faction names its units
END

### FIX F11 (frame, Trade-off analysis)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  match records (OQ-0005-3), and the Worker's Wasm carries the whole loader before any size budget exists (R1-RR-OQ5;
  OQ-0005-CD2). The content and ability spike and the asset spike are where this is proven or refuted, and each
AFTER:
  match records (OQ-0005-3), and the Worker's Wasm carries the whole loader before its one size budget has a first
  limit (R1-RR-OQ5; ADR 0002 part 4; OQ-0005-CD2). The content and ability spike and the asset spike are where this is
  proven or refuted, and each
END

### FIX F11 (frame, Consequences, revisit when)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **Revisit when:** the content and ability spike's load time or Wasm size misses its budget (OQ-0005-CD2, option
AFTER:
- **Revisit when:** content's load time misses its budget, or the Wasm file the limit ADR 0002 sets (OQ-0005-CD2, option
END

### FIX F11 (frame, open-question list, CD2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| OQ-0005-CD2 | What load time and Wasm size may content cost, and is JSON fast and small enough? | the content and ability spike measures; the Advisor sets the budgets | Content data |
AFTER:
| OQ-0005-CD2 | What load time may content cost, how much of the Wasm file is content's, and is JSON fast and small enough? | load time: the content and ability spike measures, the Advisor sets the budget; size: ADR 0002 part 4's one `sim-wasm` budget, which takes content's share as an input | Content data |
END

### FIX F2 (frame, open-question list, AB9 and AB10)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| OQ-0005-AB9 | Which drop and pickup rules does UPGRADE-SCOPE leave unstated? (ADR 0004 asks the same as OQ-0004-5) | Mustafa | The ability and effect system |
| OQ-0005-AB10 | Do bots have tracks and make picks, and what does a human get when taking a bot's slot? (ADR 0004 asks the same as OQ-0004-6) | Mustafa | The ability and effect system |
AFTER:
| OQ-0005-AB9 | Which drop and pickup rules does UPGRADE-SCOPE leave unstated? (the one home of the drop and pickup questions; ADR 0004's OQ-0004-5 keeps only the tie mechanism) | Mustafa, with the game-designer | The ability and effect system |
| OQ-0005-AB10 | Do bots have tracks and make picks, and what does a human get when taking a bot's slot? (the one home of these questions: ADR 0004's OQ-0004-6 moved here) | Mustafa | The ability and effect system |
END

### FIX F111, F112 (frame, open-question list, OQ-0005-1 and -2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| OQ-0005-1 | Do presentation references sit on the simulation definitions or in client definitions keyed by the same id? | the Advisor, at the stop review, on the fix round's text | this frame |
| OQ-0005-2 | Is a weapon definition the same thing as a weapon type? | Mustafa (through OQ-0005-AB4), then the fix round | this frame |
AFTER:
| OQ-0005-1 | Do presentation references sit on the simulation definitions or in client definitions keyed by the same id? (proposed: client definitions) | the Advisor, at the stop review | this frame |
| OQ-0005-2 | Is a weapon definition the same thing as a weapon type? | Mustafa (through OQ-0005-AB4); open for the Advisor until then | this frame |
END

### FIX F111 (frame, OQ-0005-1)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
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
AFTER:
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
END

### FIX F112 (frame, OQ-0005-2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
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
AFTER:
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
END

### FIX F2 (and F1; frame, the paragraph for the consistency pass)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
For the consistency pass (plan step 8), three points where this record and another draft differ; the frame changes
neither text: ADR 0004 part 16 says "a development reload swaps them between ticks, never inside one (R1-DATA-REC5,
ADR 0005's)", while CD-7 proposes simulation numbers at the next match and keeps the swap as OQ-0005-CD4's option (b);
OQ-0004-5 and OQ-0004-6 ask Mustafa what OQ-0005-AB9 and AB10 ask, so each is one question for him; OQ-0005-AB5 says
"ADR 0006 cites this", and ADR 0006 part 12 does not name it yet.
AFTER:
The consistency pass (plan step 8) settled the three points where this record and another draft differed: ADR 0004
part 16 now changes definitions at a match boundary under CD-7 and swaps them between ticks only if OQ-0005-CD4
chooses its option (b), and ADR 0001's `ContentReload` message carries the same condition; OQ-0005-AB9 and AB10 are
the one home of the drop, pickup and bot-record questions, which ADR 0004 cites (its OQ-0004-5 keeps only the tie
mechanism, and OQ-0004-6 moved here); and OQ-0005-AB5 now says why ADR 0006 does not cite it: ADR 0006 builds no account store, so the unlock
catalogue's weapon question is asked only in this record.
END

### FIX F11 (frame, Migration plan, slice 6)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   7); the load-time and size budgets measured again on the full content (OQ-0005-CD2; content item 9).
AFTER:
   7); content's load time and its share of the Wasm size measured again on the full content (OQ-0005-CD2; content
   item 9; the size limit is ADR 0002's, part 4).
END

### FIX F11 (frame, Rollback)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- JSON misses its budgets: the build compiles the same JSON into a binary runtime format (option D), and JSON stays
  the authored form (OQ-0005-CD2).
AFTER:
- JSON misses its load-time budget, or content's share takes the Wasm file past the limit ADR 0002 sets: the build
  compiles the same JSON into a binary runtime format (option D), and JSON stays the authored form (OQ-0005-CD2).
END

### FIX F11 (Appendix A, R1-DATA-OQ2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| R1-DATA-OQ2 | spike: the content and ability spike measures load time and Wasm size; the budgets and the binary-format fallback are OQ-0005-CD2 | CD-2; spike mark 9 |
AFTER:
| R1-DATA-OQ2 | spike: the content and ability spike measures load time and content's share of the Wasm size; the load-time budget and the binary-format fallback are OQ-0005-CD2, the size limit ADR 0002's (part 4) | CD-2; spike mark 9 |
END

### NOTE X2
The misquote is in the ability section (game-designer's Trade-off analysis, doc line 1079: "no §6 item asks for a
runtime"), not in the frame or in my sections (grep: the phrase occurs only at doc lines 992, 1074 and 1079, all in
that section). game-designer owns it; the verifier's fix is to quote r1-simulation-core.md line 1798 (Options §5,
Question A, option A3) as written: "Serves §6.3 but adds a runtime no §6 item asks for".
END
