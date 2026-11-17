# ADR 0005 pass marks PM-0005-47 to PM-0005-56: the export checks (draft rows)

- **Task:** adr0005-export-marks, change-set 0 ("Model rules") of the build plan; docs only.
- **Author:** test-engineer (Opus 5.5), 2026-11-17. ADR 0005's pass marks are test-engineer's ("pass marks:
  test-engineer"); the main session pastes these rows into `docs/adr/0005-content-abilities-assets.md`.
- **Read for this file** (lines as on 2026-11-17): the research draft
  `.checkpoints/cs0-model-rules/asset-export-rules.draft.md` (read whole at about 21:02; its 21:10 update re-read where
  it changed: lines 12–20, 197–236, 266–298, 329–334, 348–379; part 5 is now lines 266–298); engine-architect's ruling
  `.checkpoints/cs0-model-rules/joint-list.md` (§1 to §6; §4 is lines 63–72, §5 lines 74–81); ADR 0005 lines
  2180–2440 (the pass-mark section starts at line 2202, not 2192); the build plan
  `docs/reports/2026-11-17-build-plan-slice-1.md` Part 7 (lines 918–980) and Part 12 (lines 1193–1233);
  `docs/DECISIONS.md` lines 453–473 (Mustafa's entry of 2026-11-14, 13:58:05 UTC); ADR 0002 lines 613–626 (WebGPU in
  CI); ADR 0003 PM-0003-25, -36, -46 (lines 1200, 1211, 1220); `docs/LEGACY_INVENTORY.md` line 87 (CD5: the camera).
- **Mustafa's rule these rows keep** (DECISIONS.md line 473, inside his decision of 2026-11-14): "No pass mark and no
  open question of theirs is dropped or loosened: each is measured on the real game, at the first build that can show
  it, and a draft is accepted only when its marks are met. If a mark fails, the code that rests on it is redone."
  No existing row (PM-0005-1 to -46) changes; no number here is looser than the research draft's.
- **Abbreviations in the rows:** AER `docs/research/asset-export-rules.md` (the research draft once it lands), by part,
  mark, finding (AX-F) and rule (AX-R); JL `docs/research/soldier-joint-list.md`, by section. Both are proposed for the
  section's source sentence (section 3 (a) below).

## 1. The rows (paste after PM-0005-44's row, ADR 0005 line 2330, before the Feel Lab header, line 2331)

The first two lines are the table's own header, for reading only: do not paste them.

| mark | what it proves (decision part) | number and statistic | hardware | method and tool | runs and noise | basis |
|---|---|---|---|---|---|---|
| **The export checks (OQ-0005-AS3: the asset research's export and slot-naming part), written 2026-11-17 by the build plan's change-set 0, first run at its change-set M on the first real files (the rifle file and the first Blender export)** | | | | | | |
| PM-0005-47 | The file is a plain GLB that needs nothing the loader may skip (OQ-0005-AS3; AX-R1, AX-R2, AX-R16) | magic `glTF`, GLB version 2, the JSON chunk first, an `asset.version` of major version 2, no `extensionsRequired` | CI | J (the header and the JSON chunk) on the rifle file and on every file of the first real export (the rifle cannot show what AX-R2's call writes); the loader is not the check: it only warns on an unknown required extension (AX-F12) | 1 run per file. Bites: an unknown name in `extensionsRequired`; `KHR_draco_mesh_compression` there; GLB version 1 in the header | AER part 5, mark 1; AX-F6 (spec:624, 3644, 3703–3714, 3773); AX-F12 |
| PM-0005-48 | Every primitive draws in a slot the engine knows, and every node keeps its name through the loader (OQ-0005-AS3; AX-R7, AX-R8, AX-R9) | every primitive's `material` set; every material's name a slot of the model's entry, its prefix one of AX-R8's classes (O2 may rename the fifth), with no `.001` ending; every node's name of lower-case letters, digits and underscores only | CI | J (the JSON chunk) on the rifle file and on every file of the first real export. The loader keeps material names but draws a primitive without a material in a default material, silently (AX-F16), and turns `hand.L` into `handL` (AX-F15): only J sees either | 1 run per file. Bites: a primitive with no material; a material `unit_dark.001`; a node named `Cube.001` | AER part 5, mark 2; AX-F8, AX-F9, AX-F15, AX-F16; AP-3 (0005:1578–1585) |
| PM-0005-49 | The skin holds exactly the soldier's joint list, and `root` stays the client's (OQ-0005-AS3; AX-R10, AX-R15) | one skin; its `joints` name exactly the joints JL §1 lists (20 today), each once: no armature joint, no neutral bone, no `_leaf` joint; the loader finds each of them by its name; in any model file, no node named `root` | CI | J (`skins[].joints` mapped to `nodes[].name`) and L (the bones by `userData.name`, AX-F15) on the skinned figure of the first real export; the `root` line (every `nodes[].name`) on every model file, the rifle file included, which has no skin and meets only that line | 1 run per file. Bites: one unweighted vertex (the exporter adds its neutral bone: AX-F10); one joint removed, its vertices given to its parent (19 joints); a second armature (two skins); an armature object named `root` | AER part 5, mark 3; JL §1, §2; AX-F10, AX-F13 |
| PM-0005-50 | One set of at most four influences per vertex, summing to 1 (OQ-0005-AS3; AX-R15) | no `JOINTS_1` or `WEIGHTS_1` in any primitive; every vertex's `WEIGHTS_0` within 0.001 of 1 for float weights, exactly 1 for normalised integers (AX-F4) (proposed: a sum below 1 is the file's only sign of an influence cut at export, and a lost weight under 0.001 moves a vertex under 1.5 mm when a joint 1 m away turns 90 degrees; four float32 weights round to about 10⁻⁷) | CI | J on the skinned figure of the first real export: the attributes from the JSON chunk, the sums from the binary chunk. The loader cannot: it renormalises weights at load (AX-F13) | 1 run. Bites: a vertex weighted to five joints, exported with Include All Bone Influences on (`JOINTS_1` appears); one vertex's weights scaled to sum 0.9 (by script). Blind spot: a fifth influence that AX-R2's call cuts and renormalises leaves no trace in the file (AER part 5) | AER part 5, mark 4; AX-F4, AX-F10, AX-F13 |
| PM-0005-51 | The file carries geometry, slots and its skin, nothing else (OQ-0005-AS3; AX-R2, AX-R16) | `NORMAL` in every primitive; no `images`, `animations`, `cameras`, morph `targets`, `COLOR_0` or `KHR_lights_punctual` | CI | J (the JSON chunk) on the rifle file and on every file of the first real export | 1 run per file. Bites: the exporter's defaults on a rig with an action (Animations and Shape Keys are on by default, AX-F7); by script, an `animations` entry, a primitive without `NORMAL`, a `COLOR_0` attribute | AER part 5, mark 5; AX-F7, AX-F16 |
| PM-0005-52 | Scale and axes survive export and load (OQ-0005-AS3; AX-R3, AX-R4, AX-R5) | the test box (1.0 m wide, 2.0 m high, 0.5 m deep, with a small named marker standing out of its front face, off-centre toward +x) measures 2.0 m along y, 1.0 m along x and 0.5 m along z, each within 1 mm; the marker's centre at a larger z than every vertex of the box, and at x > 0; every node's rotation and scale identity (proposed: authored sizes are exact and float32 rounding at 2 m is under 1 µm; 1 mm is 0.06 px at ADR 0003's test view, where 1 px covers about 1.7 cm: height 40 and boom 56.6 put the camera 69.3 m away, 15° field of view, 1,080 px high) | CI | J on the test box of the first real export: `POSITION` min and max (spec:1689) and every node's transform; L: the loaded box's bounds and the marker's position. An unapplied scale can bring the world size back to 1 m, so only J's node check sees it | 1 run. Bites: "+Y Up" off; a scale of 0.01 left unapplied; the box turned 180 degrees about y; the box mirrored in x | AER part 5, mark 6; AX-F1, AX-F5, AX-F11; CD5 (`LEGACY_INVENTORY.md` line 87) |
| PM-0005-53 | A gun's sockets sit and point as AX-R11 gives them (OQ-0005-AS3; AX-R6, AX-R11) | a held gun's entry lists `socket_grip`, `socket_support` and `socket_muzzle`, and every socket the entry lists exists by name; `socket_grip` within 1 mm of the file's origin; `socket_muzzle` at z > 0 and within 1 mm of x = 0; each socket's frame within 0.5 degrees of AX-R11's (the angle of the rotation between them, so a roll about its forward axis counts): identity for grip, support, muzzle and sight, the entry's direction and up for `socket_eject` (proposed: 1 mm as PM-0005-52; the planted turns are 5 degrees, and the briefs' vectors, written to 3 decimals, are off by under 0.01 degrees) | CI | J (the entry's list; each socket's position and axes in the file's frame, node transforms composed) and L (each socket by name after load, AX-F16) on the rifle file as its script wrote it; AX-R11's Blender half needs a real export of a gun with one empty turned, as the SMG's eject is (the exporter's turn of rotations is inferred: AX-F11) | 1 run per gun file. Bites: one socket renamed; an entry without `socket_support`; `socket_muzzle` turned 5 degrees; `socket_grip` rolled 5 degrees about its forward axis (forward unchanged) | AER part 5, mark 7; JL §4; AX-F11, AX-F16 |
| PM-0005-54 | A soldier with a gun from its own file stays within the derived draw cap (OQ-0005-AS3; AX-R13; GPU section part 4): PM-0005-36's soldier count, run on change-set M's soldier | PM-0005-36's number, unchanged and not a second number: (draw calls with the soldier shown − hidden), gun included, at most 4 × the passes that draw him, on every frame; the very large unit stays PM-0005-36's. Known before the run: the rifle file as reported ("2 meshes", AER part 7) beside a four-slot body gives 6 per pass, 12 over the main and shadow passes, above 8: a fail until open point O1 (AER part 7) is settled | CI (a count, on the backend each run records); ML: Chrome on WebGPU and with `forceWebGL` (as PM-0005-36) | PM-0005-36's method, the body being the one change-set M draws (the code-built stand-in, one skinned mesh per colour area, at M's first step; the Blender model after) with the rifle file in his hand; J lists each file's meshes and materials beside the count (it cannot count draws) | 1 run per backend, shared with PM-0005-36's soldier half at M (BP Part 12). Control: the body without a gun, 4 per pass. Bites: PM-0005-36's (a body with a fifth material slot: 10 draws over 2 passes, above 8) | AER part 5, mark 8, and part 7, O1; PM-0005-36 (R3-CHAR-REC1, r3 lines 1044–1045) |
| PM-0005-55 | A file loads to the same scene, and its skin draws, on both backends (OQ-0005-AS3; AX-F17) | between a run whose recorded backend is WebGPU and a run with `forceWebGL`: the same node names, the same count of meshes and materials, the same slot's material on each mesh and the same bounds within 1 mm; 0 console and device errors (PM-0003-2's hook); for the skinned figure, with `lower_arm_r` turned 90 degrees, an unlit pixel probe finds the hand's slot colour where the turn puts the hand, and not at its bind position, on both runs (proposed: names and counts come from the loader, which has no backend code, AX-F17, so only drawing shows that the skin runs on each backend) | ML: Chrome on WebGPU and with `forceWebGL`. CI's headless Chromium probably gets no WebGPU adapter (ADR 0002 part 9): there both runs are WebGL 2, so CI runs the comparison and its bites but cannot pass this mark (the WebGPU route: PM-0003-46) | L twice in one browser, the scene graph walked and compared; the unlit debug view (BP Part 12) at a close camera; on the rifle file (the unskinned path) and the skinned figure of the first real export | 1 run per backend and file. Bites: a loader plugin registered in one run only; the figure drawn with its skin off in one run (the probe must find the hand at its bind position) | AER part 5, mark 9; AX-F17 (sources 8, 9); ADR 0002 part 9; PM-0003-46 |
| PM-0005-56 | The soldier's origin, facing, joints and hands in the bind pose (OQ-0005-AS3; AX-R5, AX-R10) | in the file's bind pose (JL §5): (1) `head` above `chest` above `hips` above both foot joints (y); (2) each foot's weighted vertices centred ahead of its joint (larger z); (3) every `_l` joint at x > 0, every `_r` joint at x < 0; (4) the lowest vertex at y = 0 within 1 mm, the foot joints' mid-point at x = 0 within 1 cm, z = 0 between the least and greatest z of the feet's weighted vertices; (5) `hips` the only joint without a joint parent, the mesh node and every node above `hips` identity, each joint's rest transform its bind pose (from the inverse bind matrices) within 0.5 degrees and 1 mm; (6) hung under a client-made root scaled 0.5, then 1.2: every joint's and the mesh node's world position that multiple of the unscaled one within 1 mm, and the mesh node's world scale that number; with the root unturned, line 2 holds after load; (7) once the soldier's entry holds `palm` (its field name is the entry schema's, JL §6): for each hand, the point `palm` beyond the hand joint along its lower arm within 1 cm of the centre of that hand's weighted vertices (proposed: 1 mm and 0.5 degrees as PM-0005-52 and -53; 1 cm for joints placed by hand and for a centre of vertices, 0.6 px at ADR 0003's test view; 0.5 and 1.2: one scale below 1, one above) | CI | J on the skinned figure of the first real export (lines 1 to 5 and 7: node transforms, inverse bind matrices, the binary chunk, and for line 7 the entry); L for line 6 (the loaded scene graph: its transforms are the renderer's input, the same for both backends) | 1 run. Bites: the figure turned 180 degrees; mirrored; Z-up; origin moved 1 m; the mesh node scaled 0.01; exported in a pose (one joint 10 degrees off its rest); a joint other than `hips` left without a joint parent; `hips` hung beside the root, not under it; the entry's `palm` 2 cm longer | AER part 5, mark 10; JL §2, §4, §5 |

## 2. My rulings on what the drafts left to me (point 3), with reasons

1. **The [proposal] tolerances: all kept.** 1 mm for every length and position; 0.5 degrees for every angle; 1 cm for
   the foot joints' mid-point and for `palm`; the root scales 0.5 and 1.2 within 1 mm; and O1's way (b), "flash and
   tracer still starting at `socket_muzzle`, within 1 mm" (AER line 334; not one of the ten marks), also 1 mm.
   Arithmetic: ADR 0003's test view (CD5: camHeight 40, camBack 56.6, `LEGACY_INVENTORY.md` line 87; 15° field of
   view) puts the soldier 69.3 m from the camera, so on a 1,080-px-high view 1 px covers 1.69 cm (0.84 cm at the
   close view, half the distance). 1 mm is 0.06 px there; 1 cm is 0.6 px (1.2 px at the close view). Float32 rounding
   at 2 m is 0.24 µm. The SMG eject's written vector (-0.940, 0.342, 0) is 19.993 degrees, off by 0.007, so a
   tolerance far below 0.5 degrees would fail the briefs' own numbers; the planted turns (5 degrees) sit 10 times
   above it. 1 cm, not 1 mm, where joints are placed by hand (the Art lead's, JL §6) and where a centre of vertices
   moves with how densely a hand is modelled. 0.5 and 1.2: one shrink, one growth, neither 1.
2. **The socket mark checks the up axis: yes.** AX-R11 defines each socket's up; a roll about the forward axis is a
   plain authoring slip in Blender and costs nothing more to read (the same rotation). PM-0005-53 checks the whole
   frame (the angle of the rotation between the socket's frame and AX-R11's), with a bite of its own: `socket_grip`
   rolled 5 degrees, forward unchanged, which a forward-only check passes.
3. **`palm`: PM-0005-56 carries it, as its line 7, for both hands, within 1 cm.** It reads the same data as line 2
   (the centre of a joint's weighted vertices, from the binary chunk) and needs a skinned file, so it belongs with the
   soldier's bind-pose mark. It applies from the first build whose soldier entry holds the `palm` number: the field's
   name is the entry schema's (JL §6), never this mark's. Both hands, because `palm` is one number per soldier for
   both hands (JL §4) and the left fist meets `socket_support` with it. Bite: the entry's `palm` 2 cm longer.
4. **The draw count: a row of its own that sets no number.** PM-0005-54 keeps the group whole (one row per research
   mark), names the input only the export route has (a gun from its own file, AX-R13), and runs PM-0005-36's soldier
   number on change-set M's soldier, sharing PM-0005-36's run at M (BP Part 12: "PM-0005-35 and -36 in part"); the
   very large unit stays PM-0005-36's. So O1's answer changes one number, in one row. **How a mark known to fail is
   written without loosening anything:** its number is the existing one, unchanged; the predicted fail is written in
   the row (6 per pass, 12 over 2 passes, above 8) as a fail, with no exemption, waiver or "until O1" tolerance; the
   control (the body without a gun, 4 per pass) shows the counter can pass, and PM-0005-36's bite shows it can fail;
   the fail is recorded at every run. Only three things change the result: the code (O1's ways (a) and (b)), the
   files, or Mustafa's answer to O1 written into the draft with its reason (way (c)).
5. **What the script-written rifle file proves, and what needs a real Blender export.** The rifle file can pass or fail
   PM-0005-47, -48, -51, -53 (its sockets as written), -54 (beside the body), -49's `root` line and -55's unskinned
   path: it proves each of those checks passes a valid file and, with its planted copies, bites. It cannot show that
   AX-R2's call writes a passing file, so -47, -48, -51 and -49's `root` line run again on every file of the first real
   export. A real export is needed for the rest of PM-0005-49, for -50, -52 and -56, for AX-R11's Blender half of -53
   (a turned empty) and for -55's skinned path.

**Added beyond the research draft's marks** (each closes a check line that had no failing case, or a rule that had no
check; none loosens a number): -47 a header bite; -48 AX-R7 on every node name (joints, sockets and `marking_` nodes
are found by node name, and the loader rewrites dotted names: AX-F15); -49 one skin, a missing-joint bite (the neutral
bone only tests the extra side of "exactly") and a second-armature bite, with the joint count written as JL §1's (20
today), so a ruled change of the list (JL §6: the coat joints) needs no new number; -50 the sum's tolerance and a sum
bite; -51 `COLOR_0` (AX-R2 turns off a default that writes it) and a `NORMAL` bite; -52 the marker off-centre (x > 0)
with turned and mirrored bites (sizes alone pass both); -53 the up axis; -54 the control; -55 errors and a drawn skin
(the structure compared is the same on both backends by construction, so without drawing the mark could not fail for
the reason it exists); -56 line 7 and bites for lines 5 and 6 (no research bite reached "rest equals bind", "one top
joint" or line 6).

**Left out on purpose:** a check of `asset.generator` (AX-R1's "written by Blender's exporter") would fail the
script-written rifle file, so it is a decision (section 5, item 3); `extras` (harmless `userData`, AX-F16, and whether
the exporter writes any by default was not read); mesh, skin and scene names (the engine finds nothing by them, AER
part 6, and Blender's scene "Scene" would fail a check that only a rename fixes, so such a mark would likely need
loosening).

## 3. Replacement text for ADR 0005's pass-mark section

**(a) The group sentence** (replaces lines 2203–2211, from "Written by test-engineer" to "`docs/LEGACY_INVENTORY.md`)."):

> Written by test-engineer (plan step 4). One row per mark, grouped by the spike or build step that runs it: one
> precondition (PM-0005-1); the content and ability spike, content data (PM-0005-2 to PM-0005-17, Mustafa's content
> trial inside PM-0005-17) and the ability and effect system (PM-0005-18 to PM-0005-32); ADR 0003's look proof test,
> this record's asset marks (PM-0005-33 to PM-0005-37); the asset spike (PM-0005-38 to PM-0005-44); the export checks
> of the asset research (PM-0005-47 to PM-0005-56, OQ-0005-AS3: written 2026-11-17 by the build plan's change-set 0,
> first run at its change-set M); the Feel Lab test for ABL-PREDICT (PM-0005-45, PM-0005-46). The second column names
> the decision part and, where the draft numbers them, the spike item ("content item n", "ability item n", u1–u6), or,
> for the export checks, the asset research's rules (AX-R1 to AX-R16). Every number is a research mark (cited),
> arithmetic on research numbers (shown), a settled input, or marked proposed with its reason. Line numbers are
> `r1-simulation-core.md`'s at 9e7db28 unless another file is named (r2 `r2-netcode.md`, r3 `r3-rendering.md`, r5
> `r5-toolchain-quality.md`, r6 `r6-infrastructure.md`, GL `GLOSSARY.md`, INV `docs/LEGACY_INVENTORY.md`); AER is
> `asset-export-rules.md`, cited by part, mark, finding (AX-F) and rule (AX-R), and JL is `soldier-joint-list.md`,
> cited by section.

**(b) A bullet for "How to read the table"** (insert after "Bites.", line 2270):

> - **Export checks** (PM-0005-47 to -56; AER part 5). **J** is the export check script: it reads a GLB file's header,
>   JSON chunk and binary chunk (AX-F6), never the loader's output. **L** is the game's glTF loader in a Playwright
>   test that walks the loaded scene graph; it waits on the loader's promise and counts frames, never a timer. **The
>   rifle file** is `AMK9_v4_stage1.glb`, written by a script, not by Blender's exporter (AER part 5); **the first real
>   export** is the test box, the small skinned figure (the Conscript's four slots, JL §1's joints) and a gun with a
>   turned empty, written by AX-R2's call (AER part 8). Each bite is a copy of the passing file with that one defect
>   (made by the export setting the row names, else by a script edit of the file) and must fail naming its rule; the
>   passing file is the control. Until the gate v2 runs them, CI in these rows means the build chat's container and its
>   headless Chromium (build plan Part 7).

**(c) "When a mark fails"**, three changes:

- Insert after "a checklist mark (PM-0005-17, -37, -46) when every step has a verdict." (line 2337):

> An export check (PM-0005-47 to -56) is written before a file exists to run it on: until it has run on the file its
> row names it is neither passed nor failed, and nothing that rests on it counts it as met. A mark that needs a real
> Blender export (PM-0005-49, -50, -52 and -56, the Blender half of -53 and the skinned half of -55) is never met on the
> script-written rifle file, and PM-0005-56's line 7 applies from the first build whose soldier entry holds the `palm`
> number. PM-0005-54 sets no number of its own: it runs PM-0005-36's, and its fail on the rifle file as reported (6 draws
> per pass: AER part 7, O1) is recorded at every run, never waived, until the code or the files change, or Mustafa
> answers O1 and the draft's number changes with its reason.

- In "A failed mark goes back to the draft, to the author of the part the row names (engine-architect for CD and AP
  parts; game-designer for AB parts and u1–u6; graphics-engineer for the GPU section's parts)" (lines 2338–2340),
  extend the bracket to end: "...; graphics-engineer for the GPU section's parts and the export rules (AER),
  engine-architect for the joints (JL), and the model's author when a file breaks a rule that stands)".
- After "...never inside a spike (budgets are data, and raising one is a reviewed change: R5-GATE-REC4, r5 lines
  1734–1738)." (line 2352), add:

> A change to an export check that lets through a file it refused, after the first real export as before it, is a
> loosening, and is Mustafa's (his decision of 2026-11-14); a change to the Blender recipe (AX-R2, AX-R5, AX-R10) that
> makes the export pass the mark as written is not.

**(d) "What has no measurable mark here"**: the bullet at lines 2380–2382 becomes untrue for AS3 once these rows land.
Replace it with:

> - The asset-pipeline research's own marks (OQ-0005-AS1, AS2, AS4 and AS5: compression, the LOD tool, slot renaming,
>   budgets beyond the two caps) are set by that research and appended to the asset spike before it starts; none is set
>   here. Its export checks (OQ-0005-AS3) are PM-0005-47 to PM-0005-56 (AER part 5).

## 4. For graphics-engineer's file (`asset-export-rules.draft.md`; do not edit it here)

Line numbers are the draft's at its 21:10 save; each item quotes the text, so it can be found if lines move again.

1. Line 15 (TL;DR 3): "the rifle file can test five, a Blender export the rest" becomes: the rifle file can prove five
   checks (marks 1, 2, 5, 7, and 8 beside the body), mark 3's `root` name and mark 9's unskinned path; a Blender export
   proves the rest, and marks 1, 2 and 5 again.
2. Lines 225–226: "a real export of a turned empty must read back within 0.5 degrees (mark 7)": add "its whole frame,
   forward and up".
3. Lines 234–235: "checking it against the centre of the vertices weighted to `hand_r`, within 1 cm [proposal], is a
   check no mark carries yet" becomes: mark 10's line 7 checks it for both hands within 1 cm, once the soldier's entry
   holds `palm` (PM-0005-56).
4. Line 268: "as PM-0005-47 onward" becomes "as PM-0005-47 to PM-0005-56".
5. Line 271 (L): add that CI's headless Chromium probably gets no WebGPU adapter (ADR 0002 part 9), so its two runs are
   WebGL 2 and the WebGPU halves of marks 8 and 9 run on ML (Chrome on WebGPU), as PM-0005-36 and PM-0003-46.
6. Line 272: "Numbers marked [proposal] go to test-engineer's review." becomes: test-engineer kept every [proposal]
   number (ADR 0005, PM-0005-47 to -56); drop the [proposal] tags in row 285 (mark 10) and at lines 234 and 334, and say
   in line 20's legend that they are ruled.
7. Line 276 (mark 1): planted defect, add "GLB version 1 in the header"; proven on, add "and every file of the first
   real export".
8. Line 277 (mark 2): mark, add "every node's name follows AX-R7"; planted, add "a node named `Cube.001`"; proven on,
   add "and every file of the first real export".
9. Line 278 (mark 3): mark, add "one skin", and write "JL §1's 20 names" as "the joints JL §1 lists (20 today)";
   planted, add "one joint removed, its vertices given to its parent (19 joints); a second armature (two skins)".
10. Line 279 (mark 4): mark, "every vertex's weights sum to 1" becomes "within 0.001 of 1 for float weights, exactly 1
    for normalised integers"; planted, add "one vertex's weights scaled to sum 0.9 (by script)".
11. Line 280 (mark 5): add `COLOR_0` to the list; planted, add "by script, a primitive without `NORMAL`, a `COLOR_0`
    attribute"; proven on, add "and every file of the first real export".
12. Line 281 (mark 6): "with a small named marker standing out of its front face ... the marker's centre has the
    greatest Z" becomes "... off-centre toward +X; the marker's centre at a larger Z than every vertex of the box, and
    at X > 0"; planted, add "the box turned 180 degrees about Y; the box mirrored in X".
13. Line 282 (mark 7): "each socket's forward axis (+Z) as AX-R11 gives it, within 0.5 degrees" becomes "each
    socket's frame (forward +Z and up +Y) within 0.5 degrees of AX-R11's"; planted, add "`socket_grip` rolled 5
    degrees about its forward axis".
14. Line 283 (mark 8): the mark is PM-0005-36's own number, run early (PM-0005-54 sets none of its own), and the very
    large unit stays PM-0005-36's; run by, add "control: the body without a gun, 4 per pass"; proven on, "the body
    change-set M draws (the code-built stand-in at its first step, then the Blender model) with the rifle file; one run
    shared with PM-0005-36's at M".
15. Line 284 (mark 9): mark, add "0 console and device errors; for the skinned figure, with `lower_arm_r` turned 90
    degrees, an unlit pixel probe finds the hand where the turn puts it, on both runs"; planted, add "the figure drawn
    with its skin off in one run"; run by, add "CI's headless Chromium gives WebGL 2 for both runs and cannot pass it;
    the WebGPU run is ML's".
16. Line 285 (mark 10): drop the [proposal] tags; line (4) "Z = 0 between heel and toe" becomes "z = 0 between the
    least and greatest Z of the feet's weighted vertices"; line (6) "on both backends ... gives that multiple of the
    file's height within 1 mm, mesh and joints moving together" becomes "in the loaded scene graph: every joint's and
    the mesh node's world position that multiple of the unscaled one within 1 mm, and the mesh node's world scale
    that number" (the scene graph's transforms are the renderer's input, the same for both backends); add line (7), the `palm` check (item 3 above); planted, add "exported
    in a pose (one joint 10 degrees off its rest); a joint other than `hips` left without a joint parent; `hips` hung
    beside the root; the entry's `palm` 2 cm longer"; run by, "L: line 6 (the scene graph)".
17. Lines 291–295 ("What a file can prove"): add that marks 1, 2 and 5, and mark 3's `root` name, also run on every real
    export, since the rifle cannot show what AX-R2's call writes.
18. Line 298: "Mark 7's gap and the `palm` check: part 8." becomes "Mark 7 checks each socket's whole frame, and mark
    10's line 7 carries the `palm` check (test-engineer's ruling: PM-0005-53, -56)."
19. Line 334 (O1, way (b)): "within 1 mm [proposal]" becomes "within 1 mm" (kept by test-engineer, as every length).
20. Lines 355–356 (part 8): the two "Gap" rows are ruled: mark 7 checks the up axis (PM-0005-53), mark 10 carries
    `palm` (PM-0005-56, line 7), every [proposal] number is kept. Remove both rows, or mark them "ruled in ADR 0005,
    PM-0005-47 to -56".
21. Lines 374–379: add the up axis and, once the entry holds it, the `palm` check to what the first real export must
    prove; after "Headless runs check structure and counts only", add "and probably never get WebGPU (ADR 0002 part 9)".

**Also, for engine-architect's ruling** (`joint-list.md` at 20:21; the same sections in `soldier-joint-list.draft.md`):
§4 line 69, "the check compares it with the centre of the vertices weighted to `hand_r`, within 1 cm ... [proposal]"
becomes both hands, ruled (PM-0005-56, line 7); §5 lines 79–81, drop the [proposal] tags and add the bites "the mesh
node scaled 0.01; exported in a pose; a joint other than `hips` left without a joint parent"; §6 line 91, the
tolerances are ruled, and the `palm` field's name stays the entry schema's.

## 5. For Mustafa or the advisor (not mine to decide)

1. **O1-AT-M: when O1 is asked.** Question: does Mustafa answer open point O1 (a gun from its own file breaks the
   4-draws-per-pass cap) before change-set M builds? Why it matters: PM-0005-54 and PM-0005-36 fail at M's first step
   with today's rifle file (6 per pass), and his rule says "If a mark fails, the code that rests on it is redone".
   Options: (a) ask O1 in M's plan, before M builds; (b) build M and let the fail stand in its report until he answers.
   Recommendation: (a), as AER part 7 already routes it ("the question goes to him with change-set M's plan"), so M
   builds the way he picks.
2. **MARKS-AFTER-EXPORT: the advisor agent's ruling and the no-loosening rule.** AER lines 287–290 say that if the first
   real export shows otherwise, "only marks 3 and 10 and the Blender recipe (AX-R2, AX-R5, AX-R10's Blender lines)
   change". Once marks 3 and 10 are PM-0005-49 and -56, a change that lets through a file they refuse is a loosening,
   which is Mustafa's. Options: (a) the recipe changes first (for example the exporter option that leaves out the
   armature object, which AER part 8 names but did not read), and a mark changes only with his word and the export's
   evidence; (b) the advisor's ruling covers changing the marks too. Recommendation: (a); the advisor confirms what its
   ruling meant.
3. **RIFLE-ROUTE: the rifle file and AX-R1.** AX-R1 says every model is written by Blender's exporter; the rifle file
   is written by a script (AER part 5). No mark checks the file's writer, and I added none, since such a check would
   refuse the rifle. Options: (a) the rifle enters at M as it is, as a test file, and AX-R1 holds for every model after
   it; (b) the rifle is exported again through Blender before it enters. Recommendation: (a) for Slice 1's test builds,
   said in M's plan; Mustafa's file, so his word or the advisor's.

## 6. What I could not settle

- Whether any CI engine gets WebGPU (ADR 0002 part 9; PM-0003-46's route) and whether ML exists at change-set M: the
  WebGPU halves of PM-0005-54 and -55 wait for one of them.
- The exporter's unread behaviour (how it writes the armature object, the neutral bone's name, the influence cut and
  any renormalising: AER part 8). The first real export may make PM-0005-49 or -56 fail as written; then section 5,
  item 2 applies.
- Whether three.js counts draws the same way on both backends: PM-0005-36's own question, not reopened here.
- The `palm` field's name: the soldier entry's schema, not written yet.
- I did not re-read the research's sources (the glTF specification, the exporter, the loader); the rows cite AER for
  them, and the verifier spot-checks AER.
