# Brief: update the model-file rules draft (graphics-engineer, effort max)

Task name: asset-export-rules-update. You wrote `.checkpoints/cs0-model-rules/asset-export-rules.draft.md` (349 lines:
18 findings, 16 rules, nine marks) on 2026-11-17. The advisor agent's review of change-set 0's plan and
engine-architect's ruling on the joints came after it. Bring the draft in line with both. You edit that one draft
file and your notes; nothing else. You never decide the look.

## Read first
1. Your draft and your notes (`graphics-engineer-export-draft.md`) in `.checkpoints/cs0-model-rules/`.
2. engine-architect's ruling: `.checkpoints/cs0-model-rules/joint-list.md` (the engine's joint list for Slice 1's
   soldier, whether `root` is a joint, the sockets' axes, how the hand meets the gun, the mapping of the brief's rigid
   parts). The draft takes the ruling's list and wording as they stand; where you find the ruling wrong against the
   exporter's or the loader's documented behaviour, do not change it: write the point, with its source, in a section
   "Points for engine-architect" at the end of your notes and in your answer.
3. The plan's revision 2: `.checkpoints/cs0-model-rules/plan.md` from "Revision 2" to the end (points 1 to 18).
4. The Art lead's correction of 2026-11-15, 18:40:
   `.checkpoints/build-plan-slice-1/inputs/ART_BRIEF_CORRECTION_2026-11-15_1840_gun_poses.md`. Mustafa's words on it
   (2026-11-17, 20:58): "For change-set 0 take only its rule and the socket names. The pose numbers are for the body
   step (change-set 10)." No pose number enters the draft.
5. Open nothing under /root/.claude/uploads/.

## What changes in the draft
1. **Joints and `root`:** the ruling's joint list, parents and wording replace the draft's list (old rule AX-R10 and
   what rests on it); the sentence that lets each model bring its own list goes.
2. **Sockets:** each socket's local axes as the ruling states them, in glTF's and in Blender's axes; each gun's entry
   lists its own sockets (the rifle file has four; `socket_eject` is new with the SMG); the five names are
   `socket_grip`, `socket_support`, `socket_muzzle`, `socket_sight`, `socket_eject`; how the hand meets
   `socket_grip` and the left hand `socket_support`, from the ruling and the correction's rule.
3. **His sentence on sockets, quoted whole** wherever the draft states that rule: "The poses. Where the addition's
   hand points and a gun's sockets disagree, the sockets win: the right hand's point and the gun's direction set the
   pose; the left hand, the butt and the muzzle follow from the gun model's sockets, as the SMG brief already does."
   And: "The muzzle is where the flash and the tracer start. In the simulation a shot's start is data of its own".
4. **`marking_` parts:** the engine's rule is only that a node whose name starts with `marking_` is an optional part
   the game can show or hide and that it uses one of the model's own materials. That a marking is off at game size
   is the Art lead's proposal; the call is Mustafa's. How a marking changes with the faction stays open.
5. **The marks**, each with one planted defect of its own that the check must catch:
   - mark 4's defect: a vertex with five influences (mark 3 keeps the exporter's neutral bone);
   - mark 6 (scale and axes): an asymmetric test box, 1.0 m wide, 2.0 m high, 0.5 m deep, with a small named marker
     on its front face; after export and load it measures 2.0 m along +Y, 1.0 m along X and 0.5 m along Z, each
     within 1 mm, and the marker's centre has the greatest Z. Defects: "+Y Up" off; a scale of 0.01 left unapplied;
   - mark 7 (sockets): every socket the gun's entry lists exists; `socket_grip` lies within 1 mm of the origin;
     `socket_muzzle` has a positive Z and lies within 1 mm of the gun's centre plane; each socket's forward axis
     points as the ruling says, within 0.5 degrees. Defects: one socket renamed; the muzzle socket turned 5 degrees;
   - mark 9 (both backends): on WebGPU and with `forceWebGL` the loaded model has the same node names, the same
     count of meshes and materials, the same material on each mesh and the same bounding box within 1 mm. Defect: a
     loader option that differs between the two runs;
   - mark 10 (new): the soldier's origin and facing (origin between the feet under `root`, front along +Z after
     load), placed with mark 6's export.
   Say per mark what proves it: the rifle file `AMK9_v4_stage1.glb` was "written by a script", so it can pass or
   fail marks 1, 2, 5, 7 and 8 only; marks 3, 4, 6 and 10 need a real Blender export (the test box and a small
   skinned figure).
6. **The draw cap, as an open point, not a rule:** ADR 0005 caps a soldier as drawn at four draw calls per pass
   (PM-0005-36 counts draws); a gun loaded from its own file draws on its own whatever its material is named. State
   the three ways out you named, each with what it costs and what it needs proven, and decide nothing: it goes to
   Mustafa as a question with change-set M's plan.
7. **The fifth class name** differs between ADR 0005 and ADR 0003: state both with their lines; decide nothing.
8. **Citations:** the file will live at `docs/research/asset-export-rules.md`. Cite Mustafa's messages and the Art
   lead's texts by the appendix of the plan's report, `docs/reports/2026-11-17-build-plan-slice-1-appendix.md`: A1
   his task of 2026-11-14, A2 his addition, A3 his go of 2026-11-17, A4 the soldier brief, A5 the addition of
   2026-11-15, A6 the SMG brief, A8 and A9 his two later messages, A10 the correction. Cite the plan as
   `docs/reports/2026-11-17-build-plan-slice-1.md` with its Part. No citation of a `.checkpoints/` path may remain.
9. **What stays the Art lead's proposal** is never written as decided: sizes, colours, poses, part lists. The names
   of colour areas, joints and sockets are change-set 0's (his words: "The brief's names for colour areas and bones
   are working names; the names of your change-set 0 win.").
10. Keep it under 400 lines, 120 columns, plain words; every finding keeps its source (the exporter's manual, the
    glTF specification, three.js's loader documentation or source at the pinned version).

## Your answer to me (at most 25 lines)
What changed, rule by rule and mark by mark, in one line each; the points for engine-architect, if any; what a real
export must still prove; what you could not settle.

## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
/home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/graphics-engineer-export-update.md (you may always write
there). Look there first: if the first line names another task, move the file into
/home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/old/ and start fresh; if the notes end with DONE, write
your answer from them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never
reread, recheck or redo them (a later block may still read a file it needs). Start a new file with one line naming
your task, then your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group
run, a section written). Write each finished block to its file at once. Add a notes line for each finding as you find
it (what, where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done,
written over, never appended. Scratch files go in /home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/: text
and small scripts only; builds and binaries go under .gate/cs0-model-rules/. This folder is pushed to a public
repository: never put a secret, token, environment value, request header, cookie or raw payload in it, whether in a
note, a script or a script's output; the saver's secret check only knows a few token formats and cannot read archives
or binaries. End with DONE, then give your answer from your notes alone, opening no other file. Never commit or push.
