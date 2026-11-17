Task: cs0-plan-review (advisor plan review of change-set 0, model-file rules; notes only)

## Plan (blocks, in order)
B1. Read Mustafa's go 2026-11-17 (mustafa-2026-11-17-go.md)
B2. Read Mustafa 2026-11-14 task + addition
B3. Read cs0 plan.md
B4. Read build plan rev 2: Parts 1, 7, 11, 12, 14
B5. Read Art lead briefs (inputs/) + asset-rules.md
B6. Read ADR 0005 (asset rules, PM marks, open questions) + ADR 0003 part 6
B7. Read LOOK_V2 §8 + line 30; SPIKE_PLAN ~537-545
B8. Check DECISIONS.md / STATUS / ROADMAP / copy script; branch state (git)
B9. Findings consolidated + verdict

## B1 done: go 2026-11-17 (key lines)
- CHANGE 1 (l.30): DECISIONS.md by script: 11-14 block w/ decision, 15:03 addition, "this message". STATUS/ROADMAP only where decision changes them. No game code before cs0 on main. 2.3 chat stays stopped (l.31).
- CHANGE 2 (l.37): decision "to record word for word" (second builder chat; changes 11-12 model words for 2nd chat only). Part 15 before cs1, "does not hold change-set 0" (l.38).
- CHANGE 3: names working, "the names of your change-set 0 win" (l.46); sockets socket_grip/_support/_muzzle/_sight/_eject (l.51); sockets win over hand points (l.49); 1 unit = 1 m (l.45); 4 materials incl rifle, hands/face Dark, skinned mesh stays (l.43); 1.80 m placeholder, one root scales (l.44); line widths: LOOK_V2 stands, both switchable at cs6 (l.47); muzzle = flash+tracer, sim shot start own data (l.50); LOOK_V2 l.30 "no plain tubes or cylinders" + rendering draft words "not mine", "Treat them as not decided; the carry ... removes them" (l.52).
- CHANGE 4: VFX; EDITOR_V2 s7 "say which line needs a dated pointer" -> asked of Part 15, not cs0 (l.55).
- Answers that touch docs: 4 MACHINES (classes named: RTX 2070 + i7-9700K desktop; Iris Xe laptop; VISION_V2 line stands); 8 COMPARE (b) = his word for PM-0004-34 wording change; 9 MODEL-PUBLIC (a) = OQ-0006-18 answered; 12 START-RULES (b) = OQ-0002-6 (a) 3.07 m + soft push never block; 3 GATE-FOLLOW (b)(i) sets aside 11-12 words for gate/CI edits; 11 SCOPE-RELOAD.
- Candidate gap: answered OQs (OQ-0006-18, OQ-0002-6, PM-0004-34 wording, push rule) get no dated pointer in their drafts in cs0 plan -> check build plan Part 14/12 for where those land.

## B2 done: 11-14 task + addition
- Task l.13 decision to record word for word; l.31 originally: recorded by 2.3's first docs cs (now changed by CHANGE 1).
- Task l.10: model words (Fable 5.2 main model; every other agent Opus 5.5; reviewers of game/engine code at effort max).
- Addition l.11: Blender sentences replaced; "my words of 2026-11-03 on the route stand unchanged" (l.12). Items 11-13 (model-file rules; change-set M; OQ-0006-18).
- Addition l.11 changed list drops "LOOK-TEST-INPUTS, Route: Blender" from the changed-words list -> pointers on 11-03 LOOK-TEST-INPUTS must NOT say it is changed.

## B3 done: cs0 plan read (108 lines). Files table l.17-30; names l.34-57; marks l.59-72; evidence l.82-91; reviewers l.93-96; tier l.98-101.
- Note: plan l.24 dated pointers on "SLICE-ORDER condition 3; his words of 2026-11-10 and of 2026-11-13" -- omits SPIKE-ORDER (b) / "wave 1 one heavy spike" (11-03) which the decision also changes; l.25 adds SPIKE_PLAN + ADR 0002 pointers. Check DECISIONS.md for SPIKE-ORDER entry.
- Note: plan l.24 omits pointer on 11-12 model words (changed by CHANGE 2 for 2nd chat; and GATE-FOLLOW (i) sets them aside for gate/CI edits). Check DECISIONS.md.
- Next: B4 build plan Parts 1, 7, 11, 12, 14.

## B4 done: build plan rev 2 (Parts 0, 1, 7, 11, 12, 13, 14, Decisions, For Mustafa)
- Part 1 row 0 (l.95): cs0 "about 150 lines of docs", ~6 h, "your go (it answers two open questions in a decision draft)". cs0 plan l.100 now ~500 lines, stop at 1,000. Growth comes from CHANGE 1 (record) + report; plan must say so (size re-based vs approved 150: Part 0 l.66-67 "a plan that leaves it comes back to you").
- Part 0 l.68: cs0 "is a decision doc"; risk tier 1.
- Part 0 l.81-83 models: builder Fable 5.2; other agents Opus 5.5; reviewers/verifiers of game+engine code effort max; at most 3 agents.
- Part 7 l.811-812: CS-A (2.3 chat's docs cs) was to record the 11-14 decision -> now cs0 (CHANGE 1). Report/STATUS should say CS-A no longer carries it (avoid a double record when 2.3 resumes).
- Part 11 l.923-931: cs0 = graphics-engineer closes open points by reading; verifier spot-checks; writes this part's own pass marks placed at M; one docs PR: research file, two OQs "answered as rules with proof still to come" in ADR 0005, LOOK_V2 s8 "Spline or Blender" line (his 11-03 words) -- "If the docs chat ... has already written that line, cs0 leaves it". Decision doc -> his go.
- Part 11 l.937-939: names (rules 12, 21) "proposals of a naming convention, not of the look; they need your or the Advisor's OK".
- Part 11 draft list rule 9: Blender Z up, faces -Y -> glTF +Y up, front +Z. Rule 10 origin ground under centre. Rule 21 old joint list (16, two-bone limbs, spine). Part 11 l.1021: joint seed = legacy animation drives "two-bone arms and legs, spine, chest, head" (LEGACY_INVENTORY GA2, GA7, GA8) -> cs0 one-piece legs + no spine departs from what legacy animation drives: matters for cs10 port (check GA2/GA7/GA8).
- Part 11 open (l.1029-1031): marking is a shape; rule of its own needed (small part per faction, or model per faction). cs0's marking_ prefix answers this partly.
- Part 11 l.1037: "Whether headgear is its own model, and the names of the weapon's grip points" open.
- Part 12 l.1062: M's first step = proof of rules with cs0's checks. l.1059: skin bound to same joints as code-built soldier so cs10's animation drives both.
- Decisions 8 COMPARE l.1228-1229: (b) "changes the wording of one mark (PM-0004-34), so it needs your word" -> his go gives it; no change-set named for the wording edit. Decisions 12 START-RULES l.1262: OQ-0002-6 and R2-SYNC-OQ5 (soldiers meeting). Decision 9 MODEL-PUBLIC: OQ-0006-18.
- For Mustafa l.1271-1272: cs0's PR "also carries this plan to main as a report, with your two messages in its appendix" -> report + appendix in cs0 plan conform (now three messages).
- Next: B5 briefs + asset-rules.md.

## B5a done: the three Art briefs (inputs/)
- Conscript brief l.2: "This brief covers his shapes and colours only. Movement, shooting and the rig's rules are the build plan's." l.29: bone names are "for a rigid skin ... (working bone names)". Bones used: Hips, Chest, Head, Upper arm, Forearm, Hand, Leg ("one piece from hip to ankle", l.33), Foot. l.12 one root scales. l.11 origin ground between feet; l.73 Blender (L,F,U)=(X,-Y,Z), "Parent all parts to one empty at the origin". l.20 "WORKING NAMES; the build plan's names replace them". Dark same hex all factions; Coat/Top/Trim per faction (Soviet+Empire Trim equal).
- Leg geometry: boot 0.10-0.50 (top hidden in coat), coat hem 0.43 -> whole visible leg is below where a knee (~0.5) would be. One-piece leg = the visible boot swings from the hip.
- Addition A2 l.13: marking "its own optional piece ..., off at game size and on in close views" = Art lead's proposal (go l.41: proposal unless marked his). A3 l.16 hat = part of body, follows head bone. A3 l.17 right hand at grip socket, left at support. A3 l.19 cites look brief "no plain tubes or cylinders" (struck by his go l.52).
- Marking colours differ per faction by slot (Soviet hat marking = Top, Allies = Trim, Empire = Coat) and shape differs (star/chevron/flower) -> one `marking_` node with one material cannot serve 3 factions; per-faction rule stays open (Part 11 l.1029-1031).
- SMG brief l.11: conventions = rifle file (+Y up, +Z muzzle, +X gun's left, origin grip socket); l.14 Blender Z up, muzzle -Y. l.35-40 sockets grip/support/muzzle/sight/eject; eject has a DIRECTION ("pointing right and 20 degrees up", l.40) -> socket local-axis convention not fixed in cs0 plan. l.36: grip 0.036 below bore line; muzzle on bore line -> muzzle is NOT on the +Z axis through the origin (y = +0.036).
- FINDING F-marks6: mark 6 "1 m test cube measures 1.0 m along +Y, front faces +Z" cannot fail on +Y-Up-off (a cube is 1 m on every axis; a cube has no front). Needs an asymmetric test object. (plan l.69)
- FINDING F-marks7: mark 7 "socket_muzzle on +Z" fails a correct SMG if read as "on the axis" (muzzle y=+0.036 vs grip); needs exact test (x ~ 0, z > 0, z = max). (plan l.68)
- FINDING F-marks9: mark 9 "the same result on WebGPU and forceWebGL" names no result to compare, no planted defect. (plan l.70)
- FINDING F-marks-file: AMK9 "written by a script" (go l.48) - not shown to be Blender's exporter; as "first file" it cannot prove export-setting marks (1, 3-6). (plan l.61, l.71-72)
- FINDING F-root: "13 joints under one root" with `root` listed among joints: is root a skin joint (bone) or the armature/parent node? Mark 3 (joint list equality) depends on it. (plan l.42-44)
- FINDING F-sockaxes: socket local axes (which axis points "where things leave"; eject direction) not fixed; Blender empty identity -> glTF identity (exporter conjugates), so rule is cheap: +Z = out (Blender -Y), +Y = up (Blender Z). (plan l.47-50)
- FINDING F-hand: no attachment point on the soldier's hand (hand joint = wrist; hand box centre 5.5 cm off). sockets-win needs a hand point meeting socket_grip: engine-architect to decide hand nodes vs offset in model entry. (plan l.48-49)
- FINDING F-marking: `marking_` rule carries "off at game size" = Art lead's look proposal, unattributed; per-faction shape/colour rule must stay open. (plan l.54-55)
- FINDING F-quote: "where a pose's hand points and the sockets disagree, the sockets win (his words)" paraphrase labelled his words; drops operative half ("the right hand's point and the gun's direction set the pose; the left hand, the butt and the muzzle follow from the gun model's sockets"). Docs must quote verbatim. (plan l.49)
- Next: B5b asset-rules.md skim; then B6.

## B5b done: asset-rules.md (first reading)
- s1 l.54-64: ADR 0005 class list "for example environment, structure, unit, faction colour, emissive" (0005:1762); ADR 0003 5.h list differs (visual effects). Plan prefixes match 0005's example. AP-3 (0005:1578-1585): slot names -> class via one client-content table; prefix must agree with table (Low: say table is authority or prefix derives class).
- s1 l.148-150: "Not fixed by any draft: slot names; origin; facing; world axes; joint list; weapon socket names; headgear ..." -> cs0 fixes engine conventions, not drafts' rules: OK as research + proposal for engine.
- s4 l.348: "The legacy rig is the starting point: two-bone arms and legs, spine, chest and head, hand targets on the gun's grip and rail, four coat panels, and no ankle joint (LEGACY_INVENTORY GA2, GA3, GA7, GA8, GA13)". cs0 list drops knee, spine, coat panels; adds ankle (foot).
- s5 l.389-391: stand-in = same joint list; "The ported animation (GA1 to GA15 'port to TypeScript', 0003:797 ...) finds its bones by these names".
- s4 l.355: round trip = 1 m test cube + skinned test figure, real exporter, real loader both backends; "1.0 m along +Y, the front along +Z" (same cube defect as mark 6).
- s2 l.233-234: neutral bone at armature root for unweighted vertex (BIO-src 1672-1675): sure. Influence cut/renormalise: unsure.
- FINDING F-joints: plan l.42-46 one-piece legs, no spine, no coat panels; "the list belongs to the model's entry, not to the engine" changes the approved contract (Part 11 rule 21 "exactly the game's joint list"; Part 12 l.1059 "bound to the same joints ... so the animation of cs10 drives both") and makes per-model skeletons a generic feature. Brief l.2 leaves "the rig's rules" to the build plan. Needs engine-architect ruling vs GA rows + 0003:795-797 BEFORE fixing.
- Next: B6 ADR 0005 (OQ AS3/AS4, PM table end, Proposed status) + ADR 0003 part 6 + LEGACY_INVENTORY GA rows spot check.

## B6 done: ADR 0005 + ADR 0003 part 6 + LEGACY_INVENTORY GA rows (spot-checked myself)
- ADR 0005 l.3 Status Proposed. OQ-0005-AS3 l.1874-1880 (decider: the asset-pipeline research; "What decides it: the failures the asset spike can plant and catch"); AS4 l.1881-1889 (decider: the research; export keeps names "not researched"). Register rows l.2089-2090. Mark groups l.2196-2197: asset spike PM-0005-38..44; Feel Lab PM-0005-45, -46 -> PM-0005-47 is the next free id: OK. Group intro line l.2196-2197 must name the new ids (Low).
- "answered in part" line under each OQ, decider = the research itself, proof at M: right shape for a Proposed draft IF the edit stays out of "What this section decides" (l.1747-1818) and the body; new marks only add acceptance conditions (stricter), allowed. PM table rows need the draft's columns (machine, method, runs, bites, source): plan's 9 marks give none (Low/Medium; research file must).
- ADR 0005 GPU rule 2 l.1761-1764: classes "for example environment, structure, unit, faction colour, emissive" -> prefixes OK; not look (class looks = ADR 0003 + Mustafa).
- ADR 0003 part 6 l.302-305: C-B one skinned mesh per soldier; budget LOD for "the full procedural rig". ADR 0003 l.796: "GA1-GA15 port to TypeScript"; l.792: "CC6, CC8, CC9, ... kit data, sockets, IK and props port".
- LEGACY_INVENTORY (spot-checked l.217-231): GA2 two-bone IK "Used for both hands and both legs" (mark: leg IK stretch 0.98); GA6 "solves each leg onto its foot's ankle point", mark "boots below ground 0 cm, all gaits"; GA7 "Counter-twists spine and chest"; GA8 hand IK onto grip socket + support hand along rail; GA13 four greatcoat skirt panels "rotate with the thigh", v2 note "Keep the follow-the-thigh idea"; GA14 death "Knees buckle into a kneel"; GA3 measures the fist beyond the wrist (so F-hand -> Low: say how grip point found).
- => F-joints raised to HIGH: one-piece legs + no spine + no coat joints cannot carry GA2 (legs), GA6, GA13, GA14 which ADR 0003 l.796 ports and cs10 uses as goldens (build plan l.271-272 "legacy's animation metrics as goldens"); plan's "coat hides the knee" argues for the blockout mesh, not the rig. Brief l.2 leaves the rig to the build plan; approved Part 11 rule 21 had two-bone legs + spine.
- Next: B7 LOOK_V2 s8 + l.30; SPIKE_PLAN 537-545.
