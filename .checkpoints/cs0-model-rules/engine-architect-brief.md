# Brief: Rule on the soldier joint list

_The main session's own prompt of 2026-11-17T19:45:32.235Z (agent type engine-architect, effort max), copied by script from the session's transcript so a restarted agent gets the same task. A first run was cut off at about 19:55 UTC on 2026-11-17._

---
Task: joint-list (a design ruling for change-set 0 of the build plan: docs only; you write one file in the notes folder and nothing else).

## The question
Change-set 0 fixes the names a model file must use, among them the soldier's joint list. The advisor agent's plan review found the first proposal unsound (High): 13 joints with one-piece legs, no spine and no coat joints, taken from the Art lead's blockout brief, cannot be shown to carry change-set 10's animation port. Its words: ADR 0003 line 797 ports legacy animation rows GA1 to GA15 and change-set 10 uses legacy's numbers as goldens; those rows need a knee (GA2, GA6 leg IK and "boots below ground 0 cm", GA14 knees buckle), thigh-driven coat panels (GA13), and GA7 counter-twists the spine and chest; the brief leaves the rig to us ("the rig's rules are the build plan's"); the build plan ties the code-built stand-in and the Blender model to ONE engine joint list, so that the model replaces the stand-in without engine changes; a per-model list would be a feature built for later, and a kneeless rig would settle how the walk looks before Mustafa sees it. Its fix: engine-architect, with graphics-engineer's needs in view, rules on the list against those rows and ADR 0003 lines 794-797, keeps one engine list for Slice 1, and maps the brief's rigid parts onto it (boot and trouser follow the lower leg).
Rule on it. You design the engine's naming and structure; you decide nothing about how the soldier looks or moves (shapes, proportions, poses and the feel of the walk are Mustafa's, the Art lead's and change-set 10's).

## Read
- docs/LEGACY_INVENTORY.md: the rows GA1 to GA15 (the legacy animation) and CC9, CC10 (sockets, the rest pose). Grep for "| GA" and "| CC9" / "| CC10".
- docs/adr/0003-rendering.md lines 301-323 (part 6: characters) and 790-799 (what ports).
- docs/adr/0005-content-abilities-assets.md lines 1549-1555 (AP-1: no shot origin from a model) and 1771-1776 (GPU rule 3: soldiers are skinned).
- The Art lead's briefs (working names; proposals): /home/user/Shamble-Frontlines/.checkpoints/build-plan-slice-1/inputs/ (the Conscript's first blockout: its part list ends each line with the bone it follows; its addition: three poses; the SMG brief: sockets and poses).
- Mustafa's words of 2026-11-17, CHANGE 3: /home/user/Shamble-Frontlines/.checkpoints/build-plan-slice-1/mustafa-2026-11-17-go.md (the stand-in hangs on one root so it scales with one number; "the names of your change-set 0 win"; sockets named socket_grip, socket_support, socket_muzzle, socket_sight, socket_eject; "Where the addition's hand points and a gun's sockets disagree, the sockets win: ...").
- The change-set's plan, the section "The names this change-set fixes": /home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/plan.md; the advisor agent's findings 1, 2, 3 and 16 in /home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/advisor-plan-r1.md.
- The loader facts you may rely on (already read from three.js's source by graphics-engineer): /home/user/Shamble-Frontlines/.checkpoints/build-plan-slice-1/asset-rules.md section 2 (a glTF skin's joints become Bone objects; node names lose dots and spaces; one SkinnedMesh per material primitive on one shared Skeleton; at most 4 influences).

## Write: /home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/joint-list.md (at most 130 lines)
1. The ruling: one engine joint list for Slice 1's soldier: each joint's name (lower case, digits, underscores; `_l` and `_r` for sides), its parent, and one line on why it is there, citing the legacy row or draft line that needs it. Say for each joint whether Slice 1 needs it or which named change-set does; leave out any joint nothing in Slice 1 needs (no joint "for later"). Say plainly whether the coat needs joints of its own (GA13's panels) or follows the hips and thighs, and whether a spine joint between hips and chest is needed (GA7).
2. `root`: is it a skin joint or the parent node above the skeleton that the game scales? What that means in glTF (the skin's joints, the skeleton root, the node the meshes hang on) and in Blender (the armature object, a root bone). One answer, with its reason.
3. The mapping of the brief's rigid parts (its 17 numbered parts) onto the joints, as a table: part, joint. Where a part spans a joint the blockout does not have (the one-piece leg), say how the stand-in splits it.
4. Sockets on a gun: each socket's local axes, so that a direction survives export (for example the eject direction "right and 20 degrees up"): one rule, in glTF axes and in Blender axes. How the hand meets `socket_grip` (legacy's rule for where the fist sits beyond the wrist, if GA or CC rows give one) and which joint each hand rule uses.
5. The soldier's origin and facing as a check a file can pass or fail.
6. What you leave open, each with who decides (for example anything that would fix how the walk looks; joint limits; twist joints).
7. What changes in change-set 10's size or risk because of this list, in two lines.

## Rules
- Cite file:line for every reason. Mark an inference as "[inferred]". No look decisions. No edits outside your notes folder. Do not open another branch or a folder named after a spike. No builds, no web.
- Your answer to me: at most 20 lines: the joint list as one line per joint (name, parent), the root answer, the socket axes rule, and what you leave open.

## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in /home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/engine-architect-joint-list.md (make the folder; you may always write there). Look there first: if the first line names another task, move the file into /home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/old/ and start fresh; if the notes end with DONE, write your answer from them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never reread, recheck or redo them (a later block may still read a file it needs). Start a new file with one line naming your task, then your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group run, a section written). Write each finished block to its file at once. Add a notes line for each finding as you find it (what, where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done, written over, never appended. Scratch files go in /home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/: text and small scripts only; builds and binaries go under .gate/cs0-model-rules/. This folder is pushed to a public repository: never put a secret, token, environment value, request header, cookie or raw payload in it, whether in a note, a script or a script's output; the saver's secret check only knows a few token formats and cannot read archives or binaries. End with DONE, then give your answer from your notes alone, opening no other file. Never commit or push.

---
## A later message from the main session (2026-11-17T19:46:24.809Z), part of the task
Three points from graphics-engineer's finished research draft (/home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/asset-export-rules.draft.md) to weigh in your ruling; they are its readings, not decisions.
1. `root`: it says root must be a bone, not the armature object or an empty (from the Blender exporter's behaviour: see the draft's findings on the armature and on names). Your section 2 should agree with it or say why not.
2. Sockets: it proposes a socket's direction along its local +Z, and that each gun's entry lists its own sockets (the rifle file has four; `socket_eject` is new with the SMG), so a check asks for the sockets the entry lists, not always five.
3. The soldier brief builds "one object per part"; for a skinned file these must be joined into one skinned mesh before export (one mesh object, one armature). Your mapping table in section 3 should read as the stand-in's weights, not as separate objects.
Nothing else changes in your task.
