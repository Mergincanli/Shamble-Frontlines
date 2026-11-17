Task: asset-export-rules-update (graphics-engineer, effort max): bring asset-export-rules.draft.md in line with the
advisor's plan review (plan.md revision 2, points 1-22) and engine-architect's joint ruling (joint-list.md).

## Plan (blocks, in order)
- B1 read: my draft asset-export-rules.draft.md (349 lines) - map its sections, rules, marks, citations.
- B2 read: my notes graphics-engineer-export-draft.md (sources already summarized; the three ways out of the draw cap).
- B3 read: joint-list.md (engine-architect's ruling, 98 lines).
- B4 read: plan.md from "Revision 2" to the end (points 1-22).
- B5 read: the Art lead's correction of 2026-11-15 18:40 (gun poses): rule and socket names only, no pose numbers.
- B6 read: citation targets: the plan report's appendix codes A1-A10 (if the file exists), ADR 0005 (PM-0005-36, the
  fifth class name, the four-draws cap) and ADR 0003 (the fifth class name), with line numbers.
- B7 write: draft, joints and root section (ruling's list, root not a joint, own-list sentence removed).
- B8 write: draft, sockets section (five names, per-gun lists, axes in glTF and Blender, hand meets grip/support,
  his sentences quoted whole).
- B9 write: draft, marking_ rule, proposals-not-decided pass, colour-area/joint/socket names are cs0's.
- B10 write: draft, marks (4, 6, 7, 9 defects; new mark 10; what proves each: script file vs real Blender export).
- B11 write: draft, open points (draw cap: three ways out; fifth class name: both lines).
- B12 write: draft, citations pass (no .checkpoints/ path; appendix codes; plan by Part).
- B13 check: under 400 lines, 120 columns, no .checkpoints/ citations, every finding keeps its source.
- B14 write: "Points for engine-architect" in these notes.
- B15 DONE, answer.

## Notes
- Memory checked: no agent memory folder exists (neither .claude/agent-memory* nor /root/.claude/agent-memory).
- B1 done (draft map, 349 lines): header 1-33 (abbrev cite plan:, go:, S:, G:, A: as .checkpoints paths - must go);
  part 1 scope 35-41; part 2 drafts fix 43-62; part 3 findings AX-F1..F18 (3.1 spec 66-86, 3.2 exporter 88-123 with
  settings table 92-106, 3.3 loader 125-153); part 4 rules AX-R1..R16 (4.1 file 157-175 with the export call 164-172;
  4.2 scale/axes/origin 177-186; 4.3 names 188-217: R7 names, R8 prefixes, R9 Conscript areas, R10 14 bones + "list is
  the model's" + root = top bone, R11 sockets, R12 marking_; 4.4 caps 219-235: R13 caps, R14 one mesh, R15 weights, R16
  stays out); part 5 checks 237-271 (5.1 table of nine marks 245-255; 5.2 points 1-3 against the plan 257-271);
  part 6 AS4 273-298 (counts table 283-288, ways (a)(b)(c) 290-295); part 7 open 300-329; part 8 sources 331-349.
  Next: B2.
- B2 done (old notes): the three ways out (draw cap) as I named them: (a) merge each gun's meshes at load -> 5 draws,
  still above 4; (b) at load add the gun's geometry to the body's unit_dark geometry weighted 1.0 to hand_r -> 4 draws,
  one merged geometry per body+gun pair, shared, swapped on gun change (memory and swap cost unmeasured); (c) count the
  cap in distinct materials -> changes ADR 0003's cap (r3:1044-1045) and 0005:1779-1780 + PM-0005-36 (Mustafa's go).
  Old go-file quotes (go.md lines 43-51) are now appendix A3 citations. Rifle: "2 meshes ... 1 material, four sockets"
  (go:48). Spec/loader lines re-read 2026-11-17 recorded there (do not reread). Next: B3.
- B3 done (joint-list.md, 98 lines; becomes docs/research/soldier-joint-list.md, cite as "joint list §N"):
  §1 table lines 9-21: 20 joints: hips (no joint parent); spine<hips; chest<spine; head<chest; upper_arm_l/r<chest;
  lower_arm_l/r<upper_arm; hand_l/r<lower_arm; upper_leg_l/r<hips; lower_leg_l/r<upper_leg; foot_l/r<lower_leg;
  coat_front_l, coat_front_r, coat_back_l, coat_back_r < hips. Line 28 rules: lower case/digits/underscores; _l/_r the
  soldier's own sides; joint named after the segment it moves, pivot (Blender bone head) at its upper end; skin order
  free (engine maps names once at load); lengths/rest frames read from the bind pose.
  §2 lines 30-35: root is NOT a skin joint: the one node above skeleton and mesh (game moves/turns/scales it); hips is
  the skeleton's top joint. glTF: skin.joints = exactly the 20, any order; hips only joint with non-joint parent; mesh
  node and every non-joint node above hips identity; no node named root required. Engine: root is client-owned, file's
  top nodes hang under it; stand-in builder makes the same node. Blender: armature object is the one root (brief's "one
  empty" becomes it), at origin, transforms applied, top bone hips, no root bone; joined mesh its child; "Its name is
  free: it only names the skin (AR:223-224)"; an extra empty above = one identity node, passes.
  §3 lines 37-61: 17 parts -> joints (do not copy; cite). §4 lines 63-72: sockets: five names; each gun's entry lists its
  sockets (rifle 4, SMG 5); every held gun lists grip, support, muzzle; glTF: socket local +Z = direction, +Y = its up,
  +X completes right-handed; grip/support/muzzle/sight rotation identity; SMG eject +Z = (-0.940, 0.342, 0), +Y =
  (0.342, 0.940, 0) [computed]. Blender: direction = local -Y, up = local +Z; eject -Y along (-0.940, 0, 0.342), +Z along
  (0.342, 0, 0.940); rotations inferred -> real export of an empty turned 5 deg must read back within 0.5 deg. Hands:
  right hand_r on socket_grip, left hand_l on socket_support; fist meets socket: `palm` metres beyond the wrist on the
  lower arm's line, one number per soldier in the model's entry, check vs centre of hand_r vertices within 1 cm
  [proposal]. §5 lines 74-81: origin/facing check, 5 lines (upright; feet's vertex centre ahead of foot joint; _l x>0,
  _r x<0; lowest vertex y=0 within 1 mm [proposal], feet mid-point x=0 within 1 cm [proposal], z=0 between heel and toe;
  one root: hips only top joint, non-joint nodes identity, rest = bind within 0.5 deg and 1 mm [proposal]); planted:
  turned 180, mirrored, Z-up, origin moved 1 m [proposal]. §6 open, §7 cs10 (cite, do not copy).
  Arithmetic checked: eject frame orthonormal; Y x Z = (0,0,1) so +X = toward the muzzle; Blender mapping (bx,by,bz) ->
  glTF (bx,bz,-by) gives the ruling's Blender vectors. Correct.
- FINDING (candidate point for engine-architect, P1): §2 "Its name is free: it only names the skin" - by the exporter's
  source an object's node takes blender_object.name (nodes.py:486-488, AX-F8) and the "Remove Armature Object" option
  (export_armature_object_remove, default False, __init__.py:834-841, old notes) implies the armature object is written
  as a node by default; so its name also names a node in the file and in three.js's scene. A modeller following "one
  root" may call it `root`, giving a second `root` under the engine's. Not a list change; proposal: no file node named
  `root`, or the engine never finds `root` by name. Source text of the option not re-read (only its lines + default).
- No other conflict found with the exporter/loader: neutral bone = extra joint with non-joint parent (mark 3 + mark 10
  line 5 catch it); rest = bind holds by export_rest_position_armature=True (rst:1159-1160); identity non-joint nodes
  avoid three.js bindMatrix questions. Next: B4.
- B4 done (plan.md rev 2, points 1-22 + lines 1-110 for repointing): point 6 mark 7 says "all five named nodes" but
  the brief + ruling §4 + point 19 say the gun's entry lists its sockets (grip, support, muzzle required): follow the
  brief. Point 7: rifle proves marks 1, 2, 5, 7, 8 only; 3, 4, 6 (+10 per brief) need a real Blender export. Point 12:
  stop line for the research file 300 lines; point 21 estimates "about 380"; my brief says under 400 (follow the brief;
  report the count). Point 13: marks join ADR 0005's pass-mark table with the asset spike's group (PM-0005-47 onward,
  at change-set M). Point 14: no "no plain tubes" as decided (draft has none). Point 17: from the correction only the
  rule ("The right hand is on socket_grip and the left hand on socket_support. The butt and the muzzle follow from the
  gun's file.") and the five names, as the Art lead's proposal agreeing with his go. Point 19: [proposal] numbers
  become marks' numbers via test-engineer. Point 20: the ruling's file will cite my rules BY ID: keep ids stable, give
  the main session an old-line -> new-id map.
  Old cs0 plan cites to drop: plan:21 (marks join ADR 0005 at change-set M), plan:34-57 (names), plan:59-72 (marks).
  Report = since-the-plan + build plan verbatim (report-build.mjs), so cite build plan Parts: Part 11 (rules a model
  file must follow, BP 914-1046; rule 21 joints BP:996-998), Part 12 (first Blender model, asset spike marks, BP
  1047-1088), Part 1 (cut into change-sets; change-set M, 10). Appendix codes A1-A10 confirmed (since-the-plan.md:14;
  A7 = sheet notes). Appendix not built yet: cite by code + the item's own line or quoted words.
- B5 done (correction, A10): line 20 "Everything here is the Art lead's proposal."; THE RULE line 23 (take only the two
  sentences named in point 17; 0.62 and all pose numbers stay out); line 43 socket names "socket_grip, socket_support,
  socket_muzzle and socket_sight, as in the rifle file, plus socket_eject."; line 47 "the muzzle socket is where the
  flash and the tracer start. The simulation's shot origin is its own data and is not read from the model." Next: B6.
- B6 done (citation targets, lines re-grepped 2026-11-17 on this branch): 0003:247-248 "Each material class
  (environment, structures, units, faction colours, visual effects)"; 0005:1761-1762 "Every slot's name maps to one
  class from a small fixed list (for example environment, structure, unit, faction colour, emissive;"; 0005:1579 AP-3
  the same example list; 0005:1771 one skinned mesh; 0005:1778-1781 soldier as drawn 4 materials per pass, derived from
  C-B "one per material per soldier" + ADR 0003's 4 draw calls per pass; 0005:2311 PM-0005-36 counts draws (shown minus
  hidden), bites on 5 slots (10 > 8); 0003:1200 PM-0003-25 "at most 4 draw calls per soldier per pass (before the ink)",
  bites "a soldier split into 5 meshes" (way (c) changes it too); 0005:1552 sockets cosmetic; PM-0005-46 last (2323) ->
  new marks PM-0005-47+. BP (build plan, carried verbatim in docs/reports/2026-11-17-build-plan-slice-1.md): Part 1
  table row M at BP:108; Part 11 (BP:914-1046): rules 1-24 incl. rule 21's 16 joints, "The proof by a real export is
  not part of change-set 0", "Until then the list is a draft"; Part 12 (BP:1047-1088): change-set M "The first model",
  "Its first step is the proof of Part 11's rules on the real file, with the checks change-set 0 wrote"; PM-0005-36 "in
  part" at M. Appendix: not built (docs/reports has no 2026-11-17 files); cite A-codes by section heading: A3 sections
  "CHANGE 3" (go file 40-52) and numbered answers 1-12 (MODEL-PUBLIC = item 9); A4 headings WHAT IT IS 4, AXES AND SIZE
  10, MATERIALS 20, PARTS 29, MARKINGS 55, MUST READ 60, SEND BACK 63; A6 CONVENTIONS 10, SOCKETS 35, HOW THE CONSCRIPT
  HOLDS IT 42 (replaced by A10); A5 A1 4 (replaced by A10), A2 12, A3 15; A10 THE RULE 22, TWO LINES 42, ONE WORDING 46.
  Quotes: A3 CHANGE 3 intro "Everything in them is the Art lead's proposal unless a line says it is mine."; A9 item 3
  "Like the other three it is the Art lead's proposal." and "For change-set 0 take only its rule and the socket names.
  The pose numbers are for the body step (change-set 10)."; go:49 and go:50 exact (50 continues after "data of its own"
  with ": docs/adr/0005-content-abilities-assets.md says no shot origin is ever read from a model." - quote whole).
  A8 (20:20): nothing for this draft. quotes-check.mjs flattens white space, so wrapped quotes pass; it only knows his
  messages and the Art texts (spec/ADR quotes need --source or the verifier).
- Format decision: every docs/research file keeps table rows over 120 columns (r3: 177 such rows); the old draft did
  too. Prose at most 120, tables stay tables; say so in the answer.
- ID plan (the ruling cites GD lines that the main session repoints to ids): keep AX-F11, AX-R10, AX-R11, AX-R14,
  AX-R15 stable; hands go inside AX-R11; draw cap becomes open point O1 with ways (a), (b), (c); fifth class O2.
  Ruling's GD cites: 26 & 58 GD:228-231 -> AX-R15; 27 GD:321 -> part "Not read" (uniform buffer); 32 GD:201-206 ->
  removed text (old root reading; flag to main session), GD:249 -> mark 3 row, GD:316-317 -> "Not read"; 35 & 38
  GD:224-227 -> AX-R14; 38 GD:201-206 (vertex groups) -> AX-R10; 58 & 70 GD:290-295 -> O1 (b); 64 GD:207-213 -> AX-R11,
  GD:264-267 -> AX-R11 (per-gun list); 66 GD:119-121 -> AX-F11.
- Quote checks before writing: A5 A2 line 13 "off at game size and on in close views" exact; A5 A3 line 17 "The rifle
  is a model of its own."; A6 CONVENTIONS line 14 "all transforms applied"; A6 SOCKETS line 40 "pointing right and 20
  degrees up"; A4 IF THIS IS BUILT IN BLENDER LATER line 73 "one empty at the origin", "one object per part"; A4
  MATERIALS line 27: some colour values are Mustafa's (so write "the briefs' sizes, colours, poses and part lists are
  never decided here", not "colours are the Art lead's"). 0003:247-248 is NOT marked "for example" (old draft said both
  lists are examples: fix in O2). PM-0003-25 (0003:1200) does not say whether its soldier holds a gun.
- FINDING (mark 6 design): an unapplied 0.01 scale on a box modelled at 100x leaves the loaded world size right, so L's
  bounding box alone can miss it; J must read POSITION min/max and require identity rotation and scale on every node.
- FINDING (mark 4 design): with AX-R2's call a five-joint vertex is cut to four by the exporter; if it renormalises
  (unread, AX-F10) the file passes and the fifth influence is lost silently; JOINTS_1 shows only with Include All Bone
  Influences on. Say so.
- Writing B7-B12 as one full rewrite of the draft (part 3 findings kept verbatim).
- B7-B12 done: draft rewritten in full (header/TL;DR/abbreviations with JL, BP, A3-A10; part 3 kept verbatim; AX-R10
  = JL's 20-joint table + root (JL §2 wording) + joint rules; AX-R11 = names (go:51 quote), per-gun lists, axes in glTF
  and Blender, hands (go:49 whole + A10 rule + palm [proposal]), muzzle (go:50 whole); AX-R12 marking_ engine rule only;
  AX-R13 materials cap, draws -> O1; 5.1 ten marks with "Proven on" column; 5.2 what files prove + mark 4/6/7 failure
  notes; part 7 O1 (three ways: costs, proof) and O2 (both lines); part 8 open table, not read, real export).
  Next: B13 checks.
- Spec re-read 2026-11-17 (raw.githubusercontent.com, scratchpad, grep only): 205402 bytes, sha256 prefix
  093b740752a69612e3f3 = my earlier 2026-11-17 copy; every cited line unchanged; spec:1689 "Accessors for the `POSITION`
  semantic **MUST** have their bounds, i.e., `min` and `max` properties, defined." (AX-F6 quote holds).
- FINDING (for engine-architect, confirms JL §2's inferred claim): spec:2118 "The `skeleton` property (if present)
  identifies the node that is the skeleton root of the skin. That node **MUST** be the root of all nodes listed in the
  `joints` array or their common ancestor." So a written skin.skeleton names hips or a node above it: confirmed.
  Side note spec:2231: a skinned primitive's winding order follows the determinant of the skeleton node's (or else the
  instantiating node's) global transform: one more reason for identity non-joint nodes; a mirrored file flips it.
- B13 first check: 433 lines (over 400), two prose lines at 121. Trim plan: TL;DR to 5 one-line items; fold 5.1/5.2 and
  O1/O2 subheadings into bold paragraphs; drop sources 3 (uncited) and 9 (only AX-F12's docs-page clause), renumber
  (rst 3, src 4, loader 5, PB 6, SK 7, examples 8 and 9, repo 10); tighten scope, abbreviations, part 2, R5, R9, R13,
  axes, hands, O1, part 8. Rewriting the file once more.
- COORDINATOR CHANGE 1 (Mustafa 21:33): the correction file now holds the Art lead's text alone (3,244 bytes, 29 lines);
  A10 = that copy; cite A10 by its own lines: the rule line 5, the socket names line 25, the muzzle wording line 29;
  cite nothing from the Advisor's check that stood above the line (my draft cites none of it: verify).
- COORDINATOR CHANGE 2 (advisor finding 20, Medium): mark 8 counts what PM-0005-36 counts, draw calls per pass
  (0005:1777-1781; row ~2311), not materials; the shared `unit_dark` keeps four materials, not four draws (say so in
  AX-R13); body + rifle (2 meshes) = 6 fails; the three ways out stay the open point O1 for Mustafa with change-set M.
- COORDINATOR CHANGE 3 (advisor Low): no table of the 20 joints in my draft (replaces the launch message): name the
  count and point to JL §1 (docs/research/soldier-joint-list.md).
- COORDINATOR CHANGE 4 (advisor Low): mark 10 and AX-R5 must not say "under `root`"; the advisor's ruling for the first
  real Blender export: skin.joints exactly the 20 names (no armature joint, no neutral bone, no `_leaf`); hips the only
  joint without a joint parent; mesh node and every node above hips identity; hung under a client-made root on both
  backends, a scale of 0.5 or 1.2 gives that multiple of the file's height within 1 mm, mesh and joints moving
  together. If otherwise: the 20 names stand; only marks 3 and 10 and the Blender recipe change. Write into the marks.
  Size: under 400 lines.
- B7-B12 redone (rewrite 2, with the coordinator's four changes and the trims): no joint table (AX-R10 names the 20,
  points to JL §1); mark 8 counts draws as PM-0005-36 does, run by L + renderer.info, proven on the rifle + the
  code-built stand-in body (fails today, 6/pass); AX-R13 says unit_dark keeps four materials, not four draws; mark 3 =
  exactly the 20 (no armature joint, neutral bone, _leaf); mark 10 lines 1-5 (JL §5) + line 6 (client-made root,
  scale 0.5/1.2, height within 1 mm, mesh and joints together), planted adds "a mesh node scaled 0.01 [proposal]";
  advisor's ruling sentence after the marks table; A10 cited by lines 5, 25, 29; sources renumbered 1-10 (3 and 9
  dropped). Next: B13 checks.
- B13 done (checks on the final draft, 395 lines): prose lines all <= 120 columns (two fixed; 18 table rows exceed, as
  in every docs/research file); 0 en/em dashes; 0 `.checkpoints` citations; 0 "under `root`"; 0 old cite forms (go:,
  plan:, S:, G:, A:); 0 old joint names; 34 ids defined (AX-F1..F18, AX-R1..R16), all references resolve; quote marks
  balanced; quotes-check.mjs: every quote of Mustafa and the Art lead found byte for byte (white space aside); with the
  ADRs, SPIKE_PLAN, EDITOR_V2, r3, the build plan, the spec copy and the first-reading notes as sources: 63 spans, 2 not
  found, both loader error strings ("Unsupported glTF-Binary header.", "Unknown extension": loader:1941-1948 and 547-549,
  recorded in my 2026-11-17 notes). Label names in "How to read" set in italics (they are not quotes). Part 8 row on
  [proposal] numbers fixed (mark 10, palm, O1 (b); marks 7 and 8 carry none).
- B14 done: points for engine-architect written below. Next: DONE, answer.

## Points for engine-architect (for the advisor agent to rule on)
- P1 (Low; against the exporter's source). JL §2: the armature object's name "is free: it only names the skin". The
  exporter names every object's node after the object (src blender/exp/nodes.py:486-488, AX-F8), and its option to
  remove the armature object is off by default (src __init__.py:834-841), so by default the armature object is most
  likely written as a node of its own, named after it [inferred; the option's description not re-read; the first real
  export settles it, as JL §2 says]. Its name then also names a node in the file and in three.js's scene. A modeller
  following the brief's "one root" may call it `root`, putting a second node named `root` under the client-made one:
  harmless for the checks (they find non-joint nodes by structure), confusing for any lookup by name. Proposal, no list
  change: AX-R7's characters apply to it, and either no node in the file is named `root`, or the engine never finds
  `root` by name.
- P2 (Low; stale citation). JL §2 cites "spec:2242 via AR:183-186" for "a skinned mesh ignores its node's transform";
  on the specification as re-read 2026-11-17 (205,402 bytes) that sentence is at spec:2238 (lines after 2180 moved by
  4 since 2026-11-14): repoint to AX-F5 or spec:2238.
- P3 (confirmation, no disagreement). JL §2's inferred "a written skin.skeleton names hips or a node above it" holds:
  spec:2118 "That node **MUST** be the root of all nodes listed in the `joints` array or their common ancestor."
- No other conflict with the exporter or the loader: the neutral bone is an extra joint with a non-joint parent (marks
  3 and 10 catch it); rest = bind holds under export_rest_position_armature=True (rst:1159-1160); JL §4's socket axes
  match AX-F11 for locations, the rotation step inferred in both (mark 7's turned socket on a real export proves it).

## For the main session: repointing JL's citations of the draft (point 20 of the plan's revision 2)
JL line -> new target: 26, 58 GD:228-231 -> AX-R15; 27 GD:321 -> part 8 "Not read" (three.js; now 20 joints = 1,280
bytes); 32 GD:201-206 -> removed text (the old AX-R10 reading of `root` as the top bone): cite "the draft before its
update of 2026-11-17" or drop; 32 GD:249 -> mark 3 (part 5); 32, 92 GD:316-317 -> part 8 "Not read" (the exporter);
35, 38 GD:224-227 -> AX-R14; 38 GD:201-206 (vertex groups) -> AX-R10; 58, 70 GD:290-295 -> O1, way (b); 64 GD:207-213
-> AX-R11; 64 GD:264-267 -> AX-R11 and mark 7; 66 GD:119-121 -> AX-F11. JL's AR: cites (the first reading) map to:
AR:160-161 -> AX-F1; AR:183-186 -> AX-F5 (spec:2238); AR:197 -> AX-F11; AR:223-224 -> AX-F8; AR:231-235 -> AX-F10;
AR:272-278 -> AX-F15.

## Answer (from these notes)
- Draft 395 lines (prose <= 120; 18 table rows wider, as in every research file). Rules AX-R1..R9, R14..R16: content
  kept, citations moved to BP Parts and A3..A10 (A10 by lines 5, 25, 29); R5 names the armature object as top object.
- R10: 20 joints by count + pointer to JL §1 (no table); root = node above skeleton and mesh, JL §2's words; own-list
  sentence gone. R11: five names (A3; A10:25), per-gun lists, axes glTF + Blender, hands (go:49 whole, A10:5, palm
  [proposal] check without a mark), muzzle (go:50 whole, A10:29). R12: marking_ engine rule only; "off at game size" =
  Art lead's proposal, Mustafa's call; faction change open. R13: draws per pass as PM-0005-36; unit_dark keeps four
  materials, not four draws; body + rifle = 6, fails; O1.
- Marks: 1, 2, 5 kept (5 adds a script-planted animations entry); 3 exactly JL's 20 (no armature joint, neutral bone,
  _leaf); 4 five-joint vertex; 6 asymmetric box, two defects, J must catch the unapplied scale; 7 entry's sockets, 1 mm,
  0.5 deg, rename + 5 deg turn; 8 draws as PM-0005-36 with the renderer, rifle + code-built stand-in (fails today, 6);
  9 names/counts/materials/box within 1 mm, defect a loader option in one run; 10 new: JL §5 + advisor's scale line +
  ruling and fallback. Proven on: rifle 1, 2, 5, 7, 8 (+9 unskinned); real export 3, 4, 6, 10, 7's Blender half.
- O1 three ways (costs, proof), O2 both lines (0003's not "for example"): decided nowhere.
- Points P1 (armature object's name also names a node), P2 (spec:2242 -> 2238), P3 (spec:2118 confirms JL).
- Not settled: influence cut/renormalising; armature object form; neutral bone's name; Blender version vs 5.4.12;
  mark 7's up axis; the palm check's home; mark 8 now repeats PM-0005-36's count (test-engineer may merge them).
  Size: under 400 per the brief (plan rev 2 point 12 had 300 for this file; point 21 estimated about 380).
DONE

## Pass 3 (coordinator, after engine-architect's final ruling): M1 to M5
Plan (blocks): C1 read soldier-joint-list.draft.md, only what M1-M5 need (§2 root and its ruling on P1, §4 socket axes
and required sockets, §5 mark 10, §6 the coat joints); C2 edit the draft: M1 (no file node named `root`; marks 3/10
fail it; engine never looks root up by name; armature object's name follows AX-R7, else free), M2 (AX-R10 allows the
coat change at change-set 10's pictures, at the cost of a re-rig and re-export), M3 + M4 (part 8: mark 7's up axis and
the palm check await test-engineer's ruling; mark 7's numbers unchanged), M5 (mark 7 checks that a held gun's entry
lists grip, support and muzzle); C3 checks (<= 400 lines, 120 columns, quotes, ids); C4 notes + DONE + answer (<= 10
lines). P2 and P3 are taken in the ruling: no draft change (AX-F5 already cites spec:2238).
- C1 done (soldier-joint-list.draft.md, 223 lines): §2 lines 81, 87-93: "No node in the file may be named `root`";
  armature object's name "follows AER AX-R7 and is never `root`"; ruling on P1: "the name belongs to the client's node
  alone. No node in a file may be named `root` (the check fails such a file), and the engine never looks a soldier's
  root up by name: it holds the node it made." §4 line 128: "Every held gun lists grip, support and muzzle"; §4 fixes +Y
  (lines 130-132). §6 lines 195-197: coat motion is Mustafa's call at change-set 10's pictures; "a model exported before
  change-set 10's pictures would have to be rigged and exported again." P2/P3 taken (spec:2238 line 80; spec:2118
  line 77). Note: the ruling cites appendix items by their own lines (A3:46, A4:73, A10:5); my draft cites A3-A9 by
  section heading, A10 by line: consistent enough, flag in the answer, not changed (not asked; no room).
  Line plan: R10 +1, root bullet +1, part 8 rows +1, "How marks can fail" -1, "What a file can prove" -1, not-read
  first bullet -1 -> about 395.
- C2 done: M1 root bullet: no node in any model file named `root` (the client's alone; mark 3 fails such a file);
  armature object's name free within AX-R7; client never looks root up by name; mark 3 adds the check (every
  nodes[].name) and the defect "an armature object named `root`", proven on any file. M2 AX-R10: list changes only by a
  new ruling; foreseen: coat joints go if Mustafa drops the coat's legacy motion at change-set 10's pictures, and a model
  exported before then is rigged and exported again (JL §6). M3 + M4: part 8 rows "Gap in mark 7" (+Z only, +Y fixed by
  AX-R11, rolled socket passes, numbers stay) and "Gap: no mark carries AX-R11's palm check", both test-engineer's
  ruling, pending; the mark 7 sentence left part 5. M5 mark 7: "a held gun's entry lists grip, support and muzzle",
  defect "an entry without `socket_support`". Paid for by tightening "What a file can prove", mark 4's note, the first
  not-read bullet.
- C3 done: 395 lines; prose <= 120; quotes 63 checked, 2 not found (the loader strings, as before); 34 ids resolve.
DONE (pass 3)
