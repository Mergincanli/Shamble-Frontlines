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
