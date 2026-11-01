import sys
DOC='/home/user/Shamble-Frontlines/docs/adr/0005-content-abilities-assets.md'
OUT='/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/engine-architect-0005-fixes-2.md'
FILE='docs/adr/0005-content-abilities-assets.md'
text=open(DOC).read()
lines=text.split('\n')
fixes=[]
def span(fid,a1,a2):
    idx=[i for i,l in enumerate(lines) if a1 in l]
    if len(idx)!=1: sys.exit(f'{fid}: start anchor found {len(idx)} times: {a1}')
    i=idx[0]
    j=i
    while a2 not in lines[j]:
        j+=1
        if j-i>30: sys.exit(f'{fid}: end anchor not found: {a2}')
    return '\n'.join(lines[i:j+1])
def fix(fid,a1,a2,after,expect=None):
    before=span(fid,a1,a2)
    if expect and expect not in before: sys.exit(f'{fid}: expected text missing')
    n=text.count(before)
    if n!=1: sys.exit(f'{fid}: BEFORE occurs {n} times')
    fixes.append((fid,before,after))
def rep(fid,a1,a2,old,new):
    before=span(fid,a1,a2)
    if before.count(old)!=1: sys.exit(f'{fid}: old not once')
    fix(fid,a1,a2,before.replace(old,new))
# F14 author line
rep('F14 (author line)','**Author:** engine-architect','**Author:** engine-architect',' · **Reviewed by:**',' (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:**')

# F111/F112 frame: how this record is put together
fix('F111, F112 (frame, how this record is put together)','one Appendix A. Where two sections disagree','text in the fix round.',
"""one Appendix A. Where two sections disagreed, the frame asks the question (OQ-0005-1, OQ-0005-2); after the fix
round no section answers either, and each points to it.""",expect='each author fixes its own')

# C53
fix('C53','- Unchecked merges: `rulesMerge`','(lines 1603–1607).',
"""- Unchecked merges: `rulesMerge` and `envMerge` take any key, `__proto__` included (GW2, GE1; STATUS, SEC-MDL-2), and
  the editor's `RULES_SCHEMA` limits bind only its sliders, never a load (lines 1603–1607).""",expect='line 257')

# F112 CD-6
fix('F112 (CD-6)','  - Weapons and units: a weapon definition is a weapon type','This section adds no field to either',
"""  - Weapons and units: a weapon definition names its upgrade track, and progress belongs to the weapon type, not to
    the gun (UPGRADE-SCOPE); whether each weapon definition is its own type or several definitions share one track is
    OQ-0005-AB4, and this section takes no side (OQ-0005-2). A unit definition names its class's own weapons (the
    ability section, AB-4). This section adds no field to either; it holds them in the same format, files and checks as
    every other kind.""",expect='a weapon definition is a weapon type')

# F111 CD-13
fix('F111 (CD-13)','  - What: a faction is a definition (AB-4) that names its units',"their values are ADR 0003's look tuning.",
"""  - What: a faction is a definition (AB-4) that names its units and vehicles. The three T1 classes are written once
    and shared by every faction (LOOP_V2 §5); a faction differs through its own definitions and its uniforms. Its
    soldiers are dressed by (faction, class) as data (CC6's note, `docs/LEGACY_INVENTORY.md` line 136), its uniform
    models are assets named by id (AP-2), and its colours reach the assets' faction colour slots (AP-3; the GPU
    section's part 2). Where its colours and model ids sit is OQ-0005-1: proposed, pending it, in a client definition
    keyed by the faction's id, as the feel kit is keyed by the weapon's (CD-14), so the faction's simulation definition
    holds no presentation field. A faction variant is a full definition of its own that references shared definitions
    by id (proposed: no `extends` and no merge, so the merge rules SEC-MDL-2 came from do not return); whether variants
    may override a base definition is OQ-0005-CD5, for the faction design session. Team colours never change
    (VISION_V2 §5); their values are ADR 0003's look tuning.""",expect='names its units, vehicles and colours')

# F11 content trade-off
fix('F11 (Content data, trade-off)',"  serde_json, garde and regex enlarge the Worker",'and option D stays the way out.',
"""  serde_json, garde and regex enlarge the Worker's Wasm, which has no size limit yet (R1-RR-OQ5, r1, lines 2946–2950;
  the one `sim-wasm` budget is ADR 0002's, part 4), and garde on wasm32 is unread (R1-DATA-OQ5). The content and
  ability spike measures load time and the content crate's share of the size before acceptance (OQ-0005-CD2), and
  option D stays the way out.""",expect='which has no budget yet')

# F11 OQ-CD2
fix('F11 (OQ-0005-CD2)','- **OQ-0005-CD2. What load time','Research ids: R1-DATA-OQ2 (lines 2791–2794).',
"""- **OQ-0005-CD2. What load time may content cost, how much of the Wasm file is content's, and is JSON fast and small
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
    size serde_json and garde add to the Wasm module. Research ids: R1-DATA-OQ2 (lines 2791–2794).""",expect='then the Advisor sets the budgets')

# F11 spike item 9
fix('F11 (Content data, spike item 9)',"9. **OQ-0005-CD2's numbers**",'the budgets are set from these numbers.',
"""9. **OQ-0005-CD2's numbers** (R1-DATA-REC1 mark 2, lines 2496–2498): loading a content set the size of the imported
   legacy tables plus two maps and two modes, on the native server, takes a time the spike reports. Proposed (reason:
   the client loads the same set in its Worker, and R1-RR-OQ5 asks for the size): the same load timed in Wasm in the
   Worker, and the size serde_json and garde add to the Wasm module recorded; the load-time budget is set from these
   numbers, and the size is content's share in ADR 0002's one `sim-wasm` budget (part 4).""",expect='the budgets are set from these numbers')

# F111 AP-2
fix('F111 (AP-2)','prop definition names its model by an asset id.','is OQ-0005-AP1.',
"""  - What: each unit, vehicle, weapon, faction uniform, building part and prop names its model by an asset id. Where
    that id sits is OQ-0005-1: proposed, pending it, in a client definition keyed by the simulation definition's id,
    as the feel kit is (CD-14), so no simulation definition holds an asset id (CD-6); the map document holds its own
    references either way (CD-11). One asset list in the content set (client content, CD-6) holds for each asset: its
    id; its file name with its hash; its material slots, each with its class (AP-3); its LOD levels, if it has any (the
    GPU section's part 5); and its counts of triangles, materials and textures, which the budget check reads (AP-5).
    The loader's whole-set checks (CD-2) refuse a reference to an asset the list does not hold, and a slot whose class
    is not in the class list, naming the file and the field; nothing falls back to a default model. Who writes the
    list (the automated step or a person), and whether the gate recounts a file or trusts its entry, is OQ-0005-AP1.""",
    expect='prop definition names its model by an asset id')

# F111 AP-3
fix('F111 (AP-3)','Adding a class is a content change reviewed together',"how a class looks is ADR 0003's",
"""    0003 (interface I-8). Adding a class is a content change reviewed together with the ADR 0003 material it needs. A
    faction's colours reach its faction colour slots from the faction's data (CD-13; where those colours sit is
    OQ-0005-1). The table says which slots exist; how a class looks is ADR 0003's and Mustafa's (LOOK_V2).""",
    expect='from the faction definition (CD-13)')

# F11 frame Options F1 cons
fix('F11 (frame, Question F1)','- **A.** Pros: one verdict and one hash for everything','Wasm size is unbudgeted until the spike measures it',
"""- **A.** Pros: one verdict and one hash for everything a match runs on. Cons: one crate grows with every kind, and its
  Wasm size is recorded, not gated, until the spikes have measured it (ADR 0002 part 4; OQ-0005-CD2). **B.** Pros:
  smaller pieces. Cons: no check sees""",expect='unbudgeted until the spike measures it')

# F111 frame Question F2 rows
def repn(fid,a,b,pairs):
    before=span(fid,a,b); after=before
    for o,n in pairs:
        if after.count(o)!=1: sys.exit(f'{fid}: old not once: {o[:40]}')
        after=after.replace(o,n)
    fix(fid,a,b,after)
repn('F111 (frame, Question F2, the table)','(how AP-2 and CD-13 read today)',"| b. A client definition keyed by the simulation definition",[
    ('(how AP-2 and CD-13 read today)','(how AP-2 and CD-13 read before the fix round)'),
    ('| b. A client definition keyed by the simulation definition\'s id, as the feel kit is (AB-3, CD-14) |',
     '| **b. A client definition keyed by the simulation definition\'s id, as the feel kit is (AB-3, CD-14)** (proposed, pending OQ-0005-1) |')])

# F111 frame F2 bullet
fix('F111 (frame, Question F2, the note)','The map document holds both','each author fixes its own text in the fix round.',
"""- Both options apply R1-DATA-REC8's simulation and client groups (r1, lines 2596–2598). The map document holds both
  sides in one document either way (EDITOR_V2 §2, rule 1). Proposed: (b) for definitions, because the feel kit already
  takes it and a weapon's presentation (feel kit, sockets, model) then sits in one place. The question stays open for
  the Advisor as OQ-0005-1; AP-2, AP-3 and CD-13 state (b) as the proposal pending it and decide nothing.""",
    expect='The frame leans to (b)')

# F111 frame trade-off
fix('F111 (frame, Trade-off analysis)','- **Why one model:** a unit names its weapons','- **Why one model:**',
"""- **Why one model:** a unit names its weapons, abilities and track, and its model is named under the unit's id (AP-2;
  where that reference sits is OQ-0005-1); a faction names its units""",expect='and its model (AP-2)')

# F11 frame trade-off
fix('F11 (frame, Trade-off analysis)','before any size budget exists','proven or refuted, and each',
"""  match records (OQ-0005-3), and the Worker's Wasm carries the whole loader before its one size budget has a first
  limit (R1-RR-OQ5; ADR 0002 part 4; OQ-0005-CD2). The content and ability spike and the asset spike are where this is
  proven or refuted, and each""",expect='before any size budget exists')

# F11 frame revisit when
fix('F11 (frame, Consequences, revisit when)','- **Revisit when:** the content and ability spike','- **Revisit when:**',
"""- **Revisit when:** content's load time misses its budget, or the Wasm file the limit ADR 0002 sets (OQ-0005-CD2, option""",
    expect="load time or Wasm size misses its budget")

# F11 OQ table row CD2, F2 rows AB9 AB10
fix('F11 (frame, open-question list, CD2)','| OQ-0005-CD2 |','| OQ-0005-CD2 |',
"""| OQ-0005-CD2 | What load time may content cost, how much of the Wasm file is content's, and is JSON fast and small enough? | load time: the content and ability spike measures, the Advisor sets the budget; size: ADR 0002 part 4's one `sim-wasm` budget, which takes content's share as an input | Content data |""",
    expect='the Advisor sets the budgets')
fix('F2 (frame, open-question list, AB9 and AB10)','| OQ-0005-AB9 |','| OQ-0005-AB10 |',
"""| OQ-0005-AB9 | Which drop and pickup rules does UPGRADE-SCOPE leave unstated? (the one home of the drop and pickup questions; ADR 0004's OQ-0004-5 keeps only the tie mechanism) | Mustafa, with the game-designer | The ability and effect system |
| OQ-0005-AB10 | Do bots have tracks and make picks, and what does a human get when taking a bot's slot? (the one home of these questions: ADR 0004's OQ-0004-6 moved here) | Mustafa | The ability and effect system |""",
    expect='ADR 0004 asks the same as OQ-0004-5')
fix('F111, F112 (frame, open-question list, OQ-0005-1 and -2)','| OQ-0005-1 |','| OQ-0005-2 |',
"""| OQ-0005-1 | Do presentation references sit on the simulation definitions or in client definitions keyed by the same id? (proposed: client definitions) | the Advisor, at the stop review | this frame |
| OQ-0005-2 | Is a weapon definition the same thing as a weapon type? | Mustafa (through OQ-0005-AB4); open for the Advisor until then | this frame |""",
    expect="on the fix round's text")

# F111 OQ-0005-1 full text
fix('F111 (frame, OQ-0005-1)','  - The disagreement: AP-2 has','lines 2596–2598) and the legacy fact CD12',
"""  - The question: AP-2 has each unit, vehicle, weapon, faction uniform, building part and prop name its model by an
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
    lines 1600–1602).""",expect='The frame\'s lean')

# F112 OQ-0005-2 full text
fix('F112 (frame, OQ-0005-2)','  - The disagreement: CD-6 says','so that it leaves AB4 open.',
"""  - The question: UPGRADE-SCOPE ties progress to "the weapon type", and OQ-0005-AB4 leaves open which definitions
    share a track; its option (b) lets several definitions share one (the Luger in two classes; the variants of one
    weapon). Before the fix round CD-6 read "a weapon definition is a weapon type", which matched only AB4's option (a)
    or (c); CD-6 now says only that a weapon definition names its upgrade track, and points to OQ-0005-AB4.
  - Why it matters: ADR 0004 part 18 keys the player record's tracks by "the type ids ADR 0005 defines"; if one type
    spans several definitions, the key is the track's id, not the weapon's.
  - Options: the answer follows OQ-0005-AB4. Under its (a) or (c), a definition is a type; under (b), a track's id is
    the type.
  - Decider: Mustafa answers OQ-0005-AB4; the question stays open for the Advisor until then. Research ids: none
    (UPGRADE-SCOPE).""",expect='CD-6\'s wording reads as')

# F1, F2 consistency paragraph
fix('F2 (and F1; frame, the paragraph for the consistency pass)','For the consistency pass (plan step 8), three points','does not name it yet.',
"""The consistency pass (plan step 8) settled the three points where this record and another draft differed: ADR 0004
part 16 now changes definitions at a match boundary under CD-7 and swaps them between ticks only if OQ-0005-CD4
chooses its option (b), and ADR 0001's `ContentReload` message carries the same condition; OQ-0005-AB9 and AB10 are
the one home of the drop, pickup and bot-record questions, which ADR 0004 cites (its OQ-0004-5 keeps only the tie
mechanism, and OQ-0004-6 moved here); and OQ-0005-AB5 now says why ADR 0006 does not cite it: ADR 0006 builds no account store, so the unlock
catalogue's weapon question is asked only in this record.""",expect='three points where this record')

# F11 migration, rollback, appendix
fix('F11 (frame, Migration plan, slice 6)','the load-time and size budgets measured again','the load-time and size budgets measured again',
"""   7); content's load time and its share of the Wasm size measured again on the full content (OQ-0005-CD2; content
   item 9; the size limit is ADR 0002's, part 4).""",expect='the load-time and size budgets measured again')
fix('F11 (frame, Rollback)','- JSON misses its budgets:','the authored form (OQ-0005-CD2).',
"""- JSON misses its load-time budget, or content's share takes the Wasm file past the limit ADR 0002 sets: the build
  compiles the same JSON into a binary runtime format (option D), and JSON stays the authored form (OQ-0005-CD2).""",
    expect='JSON misses its budgets')
fix('F11 (Appendix A, R1-DATA-OQ2)','| R1-DATA-OQ2 |','| R1-DATA-OQ2 |',
"""| R1-DATA-OQ2 | spike: the content and ability spike measures load time and content's share of the Wasm size; the load-time budget and the binary-format fallback are OQ-0005-CD2, the size limit ADR 0002's (part 4) | CD-2; spike mark 9 |""",
    expect='the budgets and the binary-format fallback')

out=['# engine-architect: ADR 0005 second fix round (frame, "Content data", "The asset pipeline")','',
'Fixes against `docs/adr/0005-content-abilities-assets.md` as it stands at 03645ce; every BEFORE was copied from',
'the file by script and occurs exactly once. The ability section (game-designer) and the GPU section (graphics-engineer)',
'are not touched, nor the "Pass marks" section (its MAC key line, consistency F4, is left to test-engineer, as the',
'session asked). Blocks touch disjoint line ranges, so they apply in any order.','']
for fid,before,after in fixes:
    out+=[f'### FIX {fid}',f'FILE: {FILE}','BEFORE:',before,'AFTER:',after,'END','']
out+=['### NOTE X2',
'The misquote is in the ability section (game-designer\'s Trade-off analysis, doc line 1079: "no §6 item asks for a',
'runtime"), not in the frame or in my sections (grep: the phrase occurs only at doc lines 992, 1074 and 1079, all in',
'that section). game-designer owns it; the verifier\'s fix is to quote r1-simulation-core.md line 1798 (Options §5,',
'Question A, option A3) as written: "Serves §6.3 but adds a runtime no §6 item asks for".','END','']
open(OUT,'w').write('\n'.join(out))
print(len(fixes),'fixes written')
# overlap check
ranges=[]
