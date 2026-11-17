# Asset export rules: the export and slot-naming part of the asset research

- **Topic:** the export and slot-naming part of the asset-pipeline research: OQ-0005-AS3, AS4
  (`docs/adr/0005-content-abilities-assets.md`:1874-1899; `docs/SPIKE_PLAN.md`:568-586: "graphics-engineer, verified
  like R3").
- **Author agent:** graphics-engineer (Opus 5.5). **Date:** 2026-11-17; sources read 2026-11-14, the glTF specification
  and three.js's loader again 2026-11-17; updated that day for JL and the advisor agent's review of change-set 0's plan.
- **Status:** draft; spot-checked by the verifier (35 of 36 key claims). No export has gone through these rules yet.
- **The questions:** "Which checks does an export pass before it enters the game?" (OQ-0005-AS3, 0005:1874); "How are a
  model's materials renamed into the slot classes and faction colour slots?" (OQ-0005-AS4, 0005:1886).

**TL;DR.**
1. Names in lower case, digits and underscores pass Blender's exporter and three.js's loader unchanged (part 3).
2. The rules (part 4) fix the export, axes, names, the 20 joints (`root` is a node above them) and the sockets.
3. Ten marks (PM-0005-47 to -56) answer AS3 on paper; the rifle file can prove five, a real Blender export the rest.
4. Open for Mustafa (part 7): with a gun from its own file a soldier exceeds 4 draws per pass; the fifth class's name.
5. Nothing here was proven by an export; part 8 says what the first real export must prove.

**How to read this file.** *Confirmed*: the source's own words or code, cited by line; *inferred*: my reading or
arithmetic; *[from memory, unverified]*: no source read. Numbers once marked [proposal] are kept as test-engineer ruled
(ADR 0005, PM-0005-47 to -56). Rules are marked *from a draft* (an ADR, SPIKE_PLAN or EDITOR_V2 line), *from a source*
(specification, exporter, loader), *change-set 0's naming* (Mustafa: "The brief's names for colour areas and bones are
working names; the names of your change-set 0 win.", A3) or *the engine's convention (proposed)*. The briefs' sizes,
colours, poses and part lists are never decided here ("Everything in them is the Art lead's proposal unless a line says
it is mine.", A3), nor is the look (Mustafa's and the Advisor's). Rules and marks stand for the README's recommendations
(`docs/research/README.md`:36-48), options are in parts 6 and 7, and the verifier adds its spot-check table.

**Abbreviations.** Repository lines are this branch's on 2026-11-17. **0003**, **0005**: `docs/adr/0003-rendering.md`
and `docs/adr/0005-content-abilities-assets.md`; **SP**: `docs/SPIKE_PLAN.md`; **r3**: `docs/research/r3-rendering.md`;
**JL**: engine-architect's joint ruling, `docs/research/soldier-joint-list.md`, by section; **BP**: the build plan as
carried in `docs/reports/2026-11-17-build-plan-slice-1.md`, by Part; **A3** to **A10**: items of its appendix,
`docs/reports/2026-11-17-build-plan-slice-1-appendix.md`, by the item's section heading (A10 by its own line): A3
Mustafa's go of 2026-11-17, A4 the Art lead's soldier brief, A5 its addition, A6 the SMG brief, A9 Mustafa's message of
2026-11-17, 20:58, A10 the Art lead's correction. **spec**, **rst**, **src**, **loader**, **PB**, **SK**: sources 1 and
3 to 7 (part 9).

## 1. Scope
Covered: the export, scale, axes and origin, names, joints and sockets, caps, the skin and what stays out (part 4); the
checks a file passes (AS3, part 5); materials as slot classes, and colour (AS4, part 6). Not covered (SP:568-573):
compression (OQ-0005-AS1), LOD (AS2), budgets beyond the two derived caps (AS5), vertex and bone counts (AS6), the
automated step and the asset list (OQ-0005-AP1), serving files (AP-4), very large units' part names (ADR 0004), model
files in the repository (MODEL-PUBLIC, A3, answer 9), the joints' reasons, parts and open points (JL §1, §3, §6, §7).

## 2. What the decision drafts already fix
- Only glTF/GLB, through three.js's loader on both backends (0005:1803-1804); the models' route is Blender (SP:576-581);
  test models are plain exports with no compression or LOD step (SP:416-417; 0005:1715).
- A model carries no look: geometry, its skin, UVs and textures where it has them, and named material slots, but no
  baked lighting or ink; at load each slot gets a shared toon material of its class (0005:1748-1755).
- Each slot's name maps to one class "from a small fixed list (for example environment, structure, unit, faction colour,
  emissive" (0005:1761-1762), a content table (AP-3, 0005:1578-1585); ADR 0003 names the fifth differently (O2).
  Faction slots take the faction's colour at load (0005:1764-1765); only emissive slots glow (0005:1768-1769).
- "A soldier model is one skinned mesh" (0005:1771). A soldier as drawn (body, headgear and held weapon) uses at most 4
  materials per pass, a cap derived from ADR 0003's 4 draw calls per pass (0005:1777-1781), and PM-0005-36 counts draws
  (0005:2334): mark 8, O1. About 5.2k triangles per soldier as drawn is a planning ceiling, not a budget
  (0005:1787-1792).
- No collider, hit shape or shot origin comes from a model; "a weapon model's sockets are cosmetic" (0005:1550-1553).
- 1 unit = 1 metre is `[DECIDED]` in `docs/EDITOR_V2.md`:63-65, not yet in an ADR (`docs/STATUS.md`:76); Mustafa,
  2026-11-17: "Units stay as decided in docs/EDITOR_V2.md, "1 unit = 1 metre"." (A3, CHANGE 3).
- No draft fixes slot names, origin, facing, joints or sockets: BP Part 11 proposed them (rules 9 to 12 and 21);
  change-set 0 fixes the names (A3, quoted above), and JL rules on the joints.

## 3. What the sources say

### 3.1 The glTF 2.0 specification (lines as read 2026-11-17)
- **AX-F1. Axes and units.** "glTF uses a right-handed coordinate system." (spec:730) "glTF defines +Y as up; the front
  side of a glTF asset faces +Z, the left side of a glTF asset faces +X." (spec:732) "The units for all linear distances
  are meters." (spec:737); positions are metres "After applying the node's global transform" (spec:2180) · confirmed.
- **AX-F2. Names.** Any top-level object "**MAY** have a `name`", and names "are not guaranteed to be unique"
  (spec:725); extras are "Application-specific data." (source 2) · confirmed. Our names need a check (inferred).
- **AX-F3. One material per primitive.** "A _mesh_ is defined as an array of _primitives_." (spec:1578), each with one
  optional `material`; without it "a default material" is used (spec:1584) · confirmed.
- **AX-F4. Influences and weights.** "Each attribute pair provides up to four joint indices and their weights per
  vertex." (spec:2009); more go into `JOINTS_1`/`WEIGHTS_1` (spec:2014); clients "**MAY** support only a single set of
  up to four weights and joints" (spec:2074); normalised integer weights **MUST** sum to exactly 1.0, float ones
  **SHOULD** come close (spec:2044, 2064) · confirmed. The limit of four is the client's, not the format's.
- **AX-F5. Skins.** A skin lists its joints by node index (spec:2084); the joint hierarchy "is simply the node
  hierarchy" (spec:2129); "A skinned mesh primitive does not use the global transform of the node that instantiates the
  mesh." (spec:2238) · confirmed. A scale on the mesh's node does nothing; it goes on or above the joints (inferred).
- **AX-F6. GLB.** One blob holds "JSON, buffers, and images" (spec:3677), extension `.glb` (spec:3684). It starts with
  magic `0x46546C67` ("glTF") and a version, "GLB version 2" (spec:3703-3714), then the total length (spec:3721-3728);
  chunk types are JSON `0x4E4F534A` and BIN `0x004E4942` (spec:3764-3765), and the one JSON chunk "**MUST** be the first
  chunk" (spec:3773). `asset` "**MUST** contain a `version`" (spec:624); a needed extension "**MUST** be listed in the
  top-level `extensionsRequired` array" (spec:3644); `POSITION` accessors "**MUST** have their bounds" (spec:1689) ·
  confirmed 2026-11-17. A script can read every name, count and size from the first chunk (inferred).

### 3.2 Blender's exporter (add-on 5.4.12 on main, for Blender 5.3.0 and newer: src `__init__.py`:15-21)
- **AX-F7. The settings.** The operator is `export_scene.gltf` (src `__init__.py`:1967). "Rule" is AX-R2's choice; after
  each semicolon come the `__init__.py` lines that hold the default · confirmed for this version.

| Setting (UI name; script name) | Default | Rule | Documentation; default |
|---|---|---|---|
| glTF Binary (.glb) (`export_format`) | `'GLB'` | `'GLB'` | rst:881-885; 335-342 |
| Selected Objects (`use_selection`) | False | True | not in rst; 656-660 |
| +Y Up (`export_yup`) | True | True | rst:1037-1038; 728-732 |
| Apply Modifiers (`export_apply`) | False | True | rst:1061-1063; 734-739 ("excluding Armatures") |
| UVs, Normals, Tangents (`export_texcoords`, `export_normals`, `export_tangents`) | True, True, False | the same | rst:1064-1069; 456-466, 548-552 |
| Materials (`export_materials`) | `'EXPORT'` | `'EXPORT'` | rst:1105-1114; 554-570 |
| Images (`export_image_format`) | `'AUTO'` | `'NONE'` | rst:1115-1119; 361-378 |
| Use Vertex Color (`export_vertex_color`) | `'MATERIAL'` | `'NONE'` | rst:1084-1095; 585-598 |
| Skinning, Bone Influences, Include All Bone Influences (`export_skins`, `export_influence_nb`, `export_all_influences`) | True, 4, False | the same | rst:1173-1180; 975-992 |
| Use Rest Position Armature, Add Leaf Bones (`export_rest_position_armature`, `export_leaf_bone`) | True, False | the same | rst:1159-1160; 957-964, 843-850 |
| Animations, Shape Keys (`export_animations`, `export_morph`) | True, True | False, False | rst:732, 792-795, 1138; 747-751, 994-998 |
| Custom Properties, Cameras, Punctual Lights, Draco, Meshopt (`export_extras`, `export_cameras`, `export_lights`, `export_draco_mesh_compression_enable`, `export_meshopt_compression_enable`) | all False | the same | rst:721-723, 1028-1031, 1193-1218; 722-726, 650-654, 1027-1032, 494-498, 474-478 |

- **AX-F8. Names are written as they are.** Object to node: `blender_object.name` (src `blender/exp/nodes.py`:486-488);
  mesh data to mesh (`obj_data.py`:144-149); material to material (`material/material_utils.py`:24-25); bone to joint
  node (`joints.py`:102-103; leaf bones get "_leaf"); the armature object names the skin (`skins.py`:153-154) ·
  confirmed in the source (the documentation page does not say it).
- **AX-F9. One primitive per used material.** With Materials "Export" or "Placeholder", triangles are bucketed by
  material index, one primitive per slot in use; "No Export" merges them (src `primitive_extract.py`:428-444;
  rst:1105-1114) · confirmed.
- **AX-F10. Weights and the neutral bone.** Vertex groups map to joints by name, and a group with no such bone is
  skipped (`primitive_extract.py`:1650-1651, 1668-1669); weights at or below 0.0001 are dropped (1648, 1662); a vertex
  with no bone weight goes to a "neutral bone" added at the armature root (1672-1675) · confirmed. Where the cut to
  "Bone Influences" happens, and any renormalising after it: not found [unsure].
- **AX-F11. Axes, geometry, materials.** With +Y Up, a Blender location (x, y, z) becomes (x, z, -y) (src
  `nodes.py`:611-616), so Blender's Z up becomes +Y and Blender's -Y becomes +Z, the glTF front (confirmed for
  locations; rotations inferred). Quads and n-gons become triangles, and UV seams and flat-shaded edges split vertices
  (rst:46-53); base colour comes from the Principled BSDF (rst:125-127); emission above 1.0 adds
  `KHR_materials_emissive_strength` (rst:264) · confirmed.

### 3.3 three.js's GLTFLoader (dev branch; the lines cited were the same on 2026-11-14 and 2026-11-17)
- **AX-F12. Header, version, extensions.** A wrong magic ("Unsupported glTF-Binary header."), a GLB version below 2
  (loader:1941-1948), a missing JSON chunk (1983) and an `asset.version` below 2 (481-483) each fail. A required
  extension the loader does not know only logs "Unknown extension", and loading goes on (547-549). Draco throws without
  a DRACOLoader (533-534, 2000-2006); Meshopt and KTX2 throw without a decoder only when required (1528-1532,
  1671-1677) · confirmed 2026-11-17. Refusing a required extension is the engine's own check (inferred).
- **AX-F13. Skins.** `JOINTS_0` and `WEIGHTS_0` become `skinIndex` and `skinWeight` (loader:2339-2340); other attributes
  are only lower-cased (4952), so `JOINTS_1` arrives unused, and the skinning node reads four (SK:102-117). A skinned
  primitive with both attributes becomes a `SkinnedMesh`, otherwise a `Mesh` with "Skinning disabled." (3992-4004);
  weights are normalised at load "to fix malformed assets" (4009); joints become `Bone`s (2836, 4479-4481); a missing
  joint warns (4186); a node's SkinnedMeshes bind one Skeleton (4374-4378) · confirmed (the limit of four inferred).
- **AX-F14. Several materials.** One primitive gives one mesh; several give a `Group` with one mesh per primitive, named
  from the mesh's name plus `_1`, `_2` (loader:4041-4080) · confirmed. A four-slot body arrives as four SkinnedMeshes
  sharing one Skeleton: C-B's "one per material per soldier" (r3:825).
- **AX-F15. Names.** Node, bone and mesh names pass `sanitizeNodeName`, which turns spaces into underscores and removes
  `[ ] . : /` (loader:3838-3852; PB:3-4, 178-188), with `_1` on a clash; the original stays in `userData.name`
  (loader:4509-4510); material names are kept as written (3817) · confirmed. So `hand.L` loads as `handL`, while a
  material `unit_dark.001` stays `unit_dark.001` (inferred).
- **AX-F16. Empty nodes, extras, fallbacks.** A node with no mesh, camera or light becomes an `Object3D` (loader:4493),
  so a socket loads as a named Object3D; object extras go to `userData` (2413-2423). A primitive with no material gets a
  shared default `MeshStandardMaterial` (2370-2386); missing normals give a flat-shaded clone; `COLOR_0` turns on vertex
  colours (3579-3640) · confirmed. Each fallback hides a defect on screen, so the checks read the file.
- **AX-F17. Backends.** The loader has no WebGPU-specific code; official examples load glTF and a skinned GLB under
  `WebGPURenderer` (sources 8, 9) · confirmed. A skinned GLB with `forceWebGL`: no example I read shows it.
- **AX-F18. Draw calls.** A separate mesh draws on its own: C-B merges a soldier's parts "into one geometry per
  material" (r3:825); legacy's merging cut the village from 398 to 202 draw calls (r3:303); `InstancedMesh` "will help
  you to reduce the number of draw calls" (r3:452-455) · inferred from these; the renderer's draw loop was not read.

## 4. The rules

### 4.1 The file and the export
- **AX-R1.** One model, one `.glb` file, written by Blender's built-in glTF 2.0 exporter [from a draft: 0005:1803-1804,
  SP:576-578; from a source: rst:881-885]. Whether a script-written gun file may enter the game is asked of Mustafa in
  the build plan's Part 15, a file of its own (GUN-FILES); until he answers, the rifle is used only to try the checks.
- **AX-R2.** Export with AX-F7's "Rule" column: in a Blender script (the MCP route), one call, after selecting every
  object of the model: BP Part 11's rule 7 plus `filepath` and `use_selection` (what `use_selection` takes along was not
  read; `filepath` is [from memory, unverified]). [the engine's convention (proposed); option names: src `__init__.py`]

```python
bpy.ops.export_scene.gltf(filepath=PATH, export_format='GLB', use_selection=True, export_yup=True,
    export_apply=True, export_texcoords=True, export_normals=True, export_tangents=False,
    export_materials='EXPORT', export_image_format='NONE', export_vertex_color='NONE', export_skins=True,
    export_influence_nb=4, export_all_influences=False, export_rest_position_armature=True,
    export_leaf_bone=False, export_animations=False, export_morph=False, export_extras=False,
    export_cameras=False, export_lights=False, export_draco_mesh_compression_enable=False,
    export_meshopt_compression_enable=False)
```

- **AX-R3.** Before export, apply rotation and scale on every object; Apply Modifiers then bakes the other modifiers,
  never the Armature (AX-F7). [the engine's convention (proposed), as the SMG brief asks ("all transforms applied", A6,
  CONVENTIONS); why a skin needs it: AX-F5]

### 4.2 Scale, axes and origin
- **AX-R4.** One unit is one metre; build at real size. The soldier's 1.80 m is a placeholder: "hang the soldier on one
  root so it scales with one number." (Mustafa, A3, CHANGE 3); that root is the engine's node above the file (AX-R10).
  [from a draft: `docs/EDITOR_V2.md`:63-65; from a source: spec:737]
- **AX-R5.** A soldier: +Y up, front +Z, his left +X (in Blender: Z up, front -Y, left +X); his origin on the ground
  between his feet; in Blender the armature object, at the origin, is his one top object and the joined mesh its child
  (JL §2); mark 10 checks it. [axes: spec:730-732, src `nodes.py`:611-616, as A4 has them (AXES AND SIZE); the origin
  and the top object: JL §2, §5]
- **AX-R6.** A gun: +Y up, +Z to the muzzle, +X to the gun's left, origin at `socket_grip` (in Blender: Z up, muzzle
  -Y). [change-set 0's convention, as the SMG brief has it (A6, CONVENTIONS); the rifle file matches ("+Y up, +Z to the
  muzzle", A3); +X left agrees with spec:732]

### 4.3 Names, joints and sockets
- **AX-R7.** Every name in a file (objects, meshes, materials, bones, sockets) uses lower-case letters, digits and
  underscores only: no spaces, dots, colons, slashes or brackets, so no Blender ".001" ending (Blender adds one to a
  duplicated name [from memory, unverified]). [change-set 0's naming, after BP Part 11, rule 11; from a source: AX-F15]
- **AX-R8.** A material's name is its slot name, and its prefix is its class: `faction_` (takes the faction's colour;
  the faction's data holds one colour per such area), `unit_`, `emissive_`, `structure_`, `environment_`. [change-set
  0's naming, after BP Part 11, rule 12; the drafts give the classes differently, and ADR 0005 only as examples: O2]
- **AX-R9.** The Conscript's four areas are `faction_coat`, `faction_top`, `faction_trim` and `unit_dark`, for the
  soldier brief's Coat, Top, Trim and Dark (A4, MATERIALS: the first three vary by faction). "No skin colour on this
  first stand-in: hands and face take the Dark material, so four materials hold, the rifle included." (Mustafa, A3,
  CHANGE 3). A gun's one material is `unit_dark` too (A6, CONVENTIONS; A5, A3). [change-set 0's naming]
- **AX-R10. The soldier's joints** are engine-architect's 20, named with their parents in JL §1, the list's only home;
  the stand-in, the Blender model and the file's skin carry exactly those names. The list changes only by a new ruling,
  and one change is foreseen: if Mustafa drops the coat's legacy motion at change-set 10's pictures, the four coat
  joints go, and a model exported before then is rigged and exported again (JL §6). Names follow AX-R7, `_l` and `_r`
  being the soldier's own sides; a joint's pivot (in Blender, the bone's head) is at the upper end of the segment it
  moves; the skin's order is free; the engine reads lengths and rest frames from the bind pose; vertex groups carry the
  joint names exactly (AX-F10). [JL §1, §2, §6; groups by name: from a source]
- **`root` is not a skin joint (AX-R10; JL §2).** It is the one node above both the skeleton and the mesh, which the
  game moves, turns and scales with one number; `hips` is the skeleton's top joint. In the file: `skin.joints` holds
  exactly the 20 names, in any order; `hips` is the only joint whose parent is not a joint; the mesh node and every
  non-joint node above `hips` (the armature's node, if the exporter writes one) carry an identity transform; and no
  node in any model file is named `root`, which belongs to the client's node alone (mark 3 fails such a file). In
  Blender: the armature object is the one root (the brief's "one empty at the origin", A4), at the origin, transforms
  applied, top bone `hips`, no root bone, its name free within AX-R7; the joined mesh is its child (AX-R14). In the
  engine: the client makes one `root` per soldier, hangs the file's top nodes under it, never looks it up by name.
- **AX-R11. A gun's sockets** are empty nodes (Blender empties under the gun; each loads as a named `Object3D`,
  AX-F16): `socket_grip`, `socket_support`, `socket_muzzle`, `socket_sight`, `socket_eject`. Mustafa: "Its sockets are
  named as in the rifle file: socket_grip, socket_support, socket_muzzle, socket_sight, and the new socket_eject." (A3,
  CHANGE 3); the Art lead's correction lists the same (A10:25). Each gun's entry lists the sockets its file has, and
  mark 7 asks for those: the rifle file has "four sockets" (A3), the SMG five (A6, SOCKETS); every held gun lists grip,
  support and muzzle. [names: Mustafa's, change-set 0's; the lists, the axes and the hands: JL §4]
- **Socket axes (AX-R11).** In glTF, in the gun's frame (AX-R6), a socket's local +Z is the direction it points, its
  local +Y its up at right angles to +Z, and +X completes a right-handed frame. Grip, support, muzzle and sight point
  along the bore with the gun's up: rotation identity, only their positions differ. `socket_eject` points where the
  case flies, as its gun's entry gives it: the SMG brief's "right and 20 degrees up" (A6, SOCKETS; the Art lead's
  proposal) gives +Z = (-0.940, 0.342, 0) and +Y = (0.342, 0.940, 0). In Blender (Z up, muzzle -Y) a socket's direction
  is its local -Y and its up its local +Z: the four keep identity rotation, and the SMG's eject empty is turned until
  its -Y points along (-0.940, 0, 0.342) and its +Z along (0.342, 0, 0.940). The exporter's axis change is confirmed
  for locations and inferred for rotations (AX-F11): a real export of a turned empty must read back its whole frame,
  forward and up, within 0.5 degrees (mark 7) before Blender files rely on it.
- **The hands on the gun (AX-R11).** Mustafa, 2026-11-17: "The poses. Where the addition's hand points and a gun's
  sockets disagree, the sockets win: the right hand's point and the gun's direction set the pose; the left hand, the
  butt and the muzzle follow from the gun model's sockets, as the SMG brief already does." (A3, CHANGE 3). The Art
  lead's correction proposes the same: "The right hand is on socket_grip and the left hand on socket_support. The butt
  and the muzzle follow from the gun's file." (A10:5). So the right arm's chain (`upper_arm_r`, `lower_arm_r`, `hand_r`)
  reaches `socket_grip` and the left one `socket_support`. The fist, not the wrist, meets the socket: the hand's point
  lies `palm` metres beyond the hand joint on the lower arm's line, one number per soldier in the model's entry, not a
  bone; mark 10's line 7 checks it for both hands within 1 cm, once the entry holds it (ruled by test-engineer,
  PM-0005-56). No pose number is here: "For change-set 0 take only its rule and the socket names. The pose numbers are
  for the body step (change-set 10)." (Mustafa, A9).
- **The muzzle (AX-R11).** "The muzzle is where the flash and the tracer start. In the simulation a shot's start is
  data of its own: docs/adr/0005-content-abilities-assets.md says no shot origin is ever read from a model." (Mustafa,
  A3, CHANGE 3); the correction's wording agrees (A10:29); "a weapon model's sockets are cosmetic" (0005:1552).
- **AX-R12.** A node whose name starts with `marking_` is an optional part the game can show or hide, and it uses one
  of the model's own materials: that is the engine's whole rule. That a marking is "off at game size and on in close
  views" is the Art lead's proposal (A5, A2); the call is Mustafa's. How a marking changes with the faction stays open
  (part 8). A shown marking draws as a mesh of its own (AX-F14, AX-F18; inferred); how it follows its joint was not
  read. [change-set 0's naming]

### 4.4 Caps, the skin, and what stays out
- **AX-R13.** A soldier as drawn, his gun included, stays within 4 draw calls per pass, as PM-0005-36 counts them
  (0005:1777-1781, 2334), and mark 8 counts the same. The shared `unit_dark` name keeps four materials, not four draws:
  a gun loaded from its own file draws on its own whatever its material is named (AX-F14, AX-F18), so the Conscript
  with the rifle's "2 meshes" (A3) makes 6 per pass and fails. How to meet the cap is open (O1). A very large unit:
  parts × materials at most 25 per pass (0005:1784-1786). Triangles: no cap; about 5.2k per soldier as drawn is a
  planning ceiling (0005:1787-1792), and the rifle alone has 2,596 (A3). [from a draft]
- **AX-R14.** A soldier is one mesh object: the parts are joined before export and parented to the armature, each
  part's vertices weighted 1.0 to the joint JL §3 gives it (a rigid skin) until someone paints weights. The brief's
  "one object per part" (A4, IF THIS IS BUILT IN BLENDER LATER) is the modelling form; exported as it is, each part
  would be a node and a mesh of its own. [from a draft: 0005:1771, r3:825; joining: the engine's convention (proposed)]
- **AX-R15.** Every vertex is weighted to one to four joints, summing to 1; an unweighted vertex makes the exporter add
  a neutral bone, an extra joint (mark 3). The armature's rest position is the bind pose (rst:1159-1160); the model's
  author chooses that pose. A gun has no armature and no skin. [from a source: AX-F4, AX-F10, AX-F13; the gun: the
  engine's convention (proposed), after A5, A3: "The rifle is a model of its own."]
- **AX-R16.** No images or textures, vertex colours, animations, shape keys, cameras, lights, custom properties,
  compression or LOD levels; no baked lighting, ink or toon shading; no collider, hit shape, shot origin or navigation
  data. [from a draft: 0005:1748-1750, SP:416-417, 5:1715, 0005:1550-1553; no textures is also A4, DO NOT; no
  animations: the engine's convention (proposed), since soldiers are animated in code (0003:306, 3:797)]

## 5. The checks a file passes (OQ-0005-AS3)
Each of the ten marks has a defect of its own that a test plants and the check must catch. The main session carries
them into ADR 0005's pass-mark table as PM-0005-47 to PM-0005-56 (each row names its id), placed at change-set M, whose
first step is "the proof of Part 11's rules on the real file, with the checks change-set 0 wrote" (BP Part 12); until
a real export exists they are written, not run (BP Part 11). **J** is a script over the GLB (header and first chunk,
AX-F6; the binary chunk for vertices and weights); **L** is the game's loader in a test on both backends (AX-F12 to
AX-F17). CI's headless Chromium probably gets no WebGPU adapter (ADR 0002 part 9), so its two runs are WebGL 2, and the
WebGPU halves of marks 8 and 9 run on the minimum laptop (ML, 0003:1083), as PM-0005-36 and PM-0003-46 do. The Khronos
glTF Validator might do part of J; not read. Each number's final form is in the row of its PM id.

| # | Mark | Planted defect | Run by | Proven on |
|---|---|---|---|---|
| 1 (PM-0005-47) | GLB, glTF 2, no required extension | an unknown name in `extensionsRequired` (the loader only warns); `KHR_draco_mesh_compression` there; GLB version 1 in the header; a wrong magic; the BIN chunk first | J: magic, version 2, JSON chunk first, `asset.version`, no `extensionsRequired` (spec:624, 3644, 3703-3714, 3773). L throws for Draco without a decoder but only warns on an unknown name (AX-F12) | the rifle file, and every file of the first real export |
| 2 (PM-0005-48) | every primitive has a material named in the slot table; every node's name follows AX-R7 | a primitive with no material; a material `unit_dark.001`; a node named `Cube.001`; a slot `paint_coat` in entry and file | J: `primitives[].material` set, `materials[].name` in the table, every `nodes[].name`. L keeps material names (loader:3817) but draws the default material silently (2370-2386) and rewrites a dotted node name (AX-F15) | the rifle file, and every file of the first real export |
| 3 (PM-0005-49) | one skin, whose `joints` are the joints JL §1 lists (20 today), each once: no armature joint, no neutral bone, no `_leaf`; in any model file, no node is named `root` (AX-R10) | one unweighted vertex, which makes the exporter add its neutral bone (AX-F10); one joint removed, its vertices given to its parent (19 joints); a second armature (two skins); an armature object named `root` | J: `skins[].joints` mapped to `nodes[].name`, and every `nodes[].name`; L: bones by `userData.name` (loader:4509) | a real Blender export (the skinned figure); the `root` line on every model file, the rifle included |
| 4 (PM-0005-50) | one set of joints and weights; every vertex's weights sum to within 0.001 of 1 for float weights, exactly 1 for normalised integers (AX-F4) | a vertex weighted to five joints, exported with Include All Bone Influences on (`JOINTS_1` appears); one vertex's weights scaled to sum 0.9 (by script) | J: no `JOINTS_1` or `WEIGHTS_1`; sums from the binary chunk. L alone cannot: it normalises weights at load (loader:4009) | a real Blender export (the skinned figure) |
| 5 (PM-0005-51) | normals; no image, animation, morph target, `COLOR_0`, camera or light | the exporter's defaults on a rig with an action (Animations and Shape Keys are on by default); by script, one copy per listed key, and a primitive without `NORMAL` | J: `NORMAL` in every primitive; no `images`, `animations`, `cameras`, `targets`, `COLOR_0` or `KHR_lights_punctual` | the rifle file, and every file of the first real export |
| 6 (PM-0005-52) | scale and axes: an asymmetric test box, 1.0 m wide, 2.0 m high, 0.5 m deep, with a small named marker standing out of its front face, off-centre toward +X, after export and load measures 2.0 m along +Y, 1.0 m along X and 0.5 m along Z, each within 1 mm; the marker's centre at a larger Z than every vertex of the box, and at X > 0 | "+Y Up" off; a scale of 0.01 left unapplied; the box turned 180 degrees about Y; the box mirrored in X | J: `POSITION` min and max (spec:1689), and every node's rotation and scale identity: the unapplied scale shows there, since the node's scale can bring the world size back to 1 m (inferred); L: the bounding box and the marker's position | a real Blender export (the test box) |
| 7 (PM-0005-53) | sockets: a held gun's entry lists grip, support and muzzle; every socket the entry lists exists; `socket_grip` within 1 mm of the origin; `socket_muzzle` at a positive Z and within 1 mm of the gun's centre plane (X = 0); each socket's frame (forward +Z and up +Y) within 0.5 degrees of AX-R11's | one socket renamed; the muzzle socket turned 5 degrees; `socket_grip` rolled 5 degrees about its forward axis; an entry without `socket_support`; `socket_grip` moved 2 mm; `socket_muzzle` moved 2 mm off X = 0 | J: the entry's list, `nodes[].name`, `translation`, `rotation`; L: each socket by name, world position and frame (loader:4493) | the rifle file, for its sockets as a script wrote them; AX-R11's Blender half needs a real export of a turned empty |
| 8 (PM-0005-54) | PM-0005-36's own number, run early, with none of its own: the soldier as drawn, gun included, at most 4 draw calls per pass (shown minus hidden, at most 4 × the passes that draw him); the very large unit stays PM-0005-36's. Known before the run: today's rifle beside the body gives 6 per pass, 12 over two passes: a fail, written as one and never waived (O1) | a body with a fifth material slot (10 draws over 2 passes, above 8: PM-0005-36's own bite) | L with the renderer: `renderer.info` draw calls with the soldier shown minus hidden; control: the body without a gun, 4 per pass; J cannot count draws (it lists each file's meshes and materials) | the body change-set M draws (the code-built stand-in at its first step, then the Blender model) with the rifle file; one run shared with PM-0005-36's at M |
| 9 (PM-0005-55) | the same result on WebGPU and with `forceWebGL`: the same node names, the same count of meshes and materials, the same material on each mesh and the same bounding box within 1 mm; 0 console and device errors; for the skinned figure under a root scaled 1.2, with `lower_arm_r` turned 90 degrees, an unlit pixel probe finds the hand where the turn and the scale put it, on both runs | a loader option that differs between the two runs (for example a plugin registered in one run only); the figure drawn with its skin off in one run; `hips` hung beside the root in one run | L run twice in a browser; CI's headless Chromium probably gives WebGL 2 for both runs and cannot pass it: the WebGPU run is ML's; headless runs give structure and counts, never frame rates as hardware results | any file: the rifle for the unskinned path, the skinned figure for a skinned GLB with `forceWebGL` (AX-F17) |
| 10 (PM-0005-56) | the soldier's origin and facing, in the bind pose (JL §5): (1) `head` above `chest` above `hips` above both feet; (2) each foot's weighted vertices centred ahead of its joint (larger Z); (3) every `_l` joint at X > 0, every `_r` joint at X < 0; (4) the lowest vertex at Y = 0 within 1 mm, the foot joints' mid-point at X = 0 within 1 cm, Z = 0 between the least and greatest Z of the feet's weighted vertices; (5) `hips` the only joint without a joint parent, the mesh node and every node above `hips` identity, each joint's rest transform its bind pose within 0.5 degrees and 1 mm; (6) hung under a client-made root scaled 0.5, then 1.2, in the loaded scene graph: every joint's and the mesh node's world position that multiple of the unscaled one within 0.5 mm, and the mesh node's world scale that number; (7) once the soldier's entry holds `palm`: for each hand, the point `palm` beyond the hand joint along its lower arm within 1 cm of the centre of that hand's weighted vertices | turned 180 degrees; upside down (turned 180 degrees about z); mirrored; Z-up; origin moved 1 m; a mesh node scaled 0.01; exported in a pose (one joint 10 degrees off its rest); a joint other than `hips` left without a joint parent; `hips` hung beside the root; the entry's `palm` 2 cm longer | J: lines 1 to 5 and 7, from the node transforms, the inverse bind matrices, the binary chunk and the entry; L: line 6 (the scene graph, the renderer's input on both backends), and the front along +Z after load | a real Blender export (the skinned figure, made with mark 6's box) |

- **The advisor agent's ruling on the first real export** (its re-read of change-set 0's plan): it must show
  `skin.joints` holding exactly the 20 names (mark 3), `hips` as the only joint without a joint parent, the mesh node
  and every node above `hips` with identity transforms, and line 6's scaling (mark 10). If it shows otherwise, the 20
  names stand, and only marks 3 and 10 and the Blender recipe (AX-R2, AX-R5, AX-R10's Blender lines) change.
  test-engineer adds: once these are pass marks, a change that lets through a file they refuse is a loosening and needs
  Mustafa's word, so the Blender recipe changes first.
- **What a file can prove.** The rifle file `AMK9_v4_stage1.glb`, "a rifle prototype written by a script" (A3), never
  went through Blender's exporter: it can pass or fail marks 1, 2, 5, 7 and 8 (8 beside the body), mark 3's `root` line
  and mark 9's unskinned path, but cannot show what AX-R2's call writes, so marks 1, 2, 5 and mark 3's `root` line run
  again on every real export. Marks 3, 4, 6 and 10, mark 7's Blender half and mark 9's skinned path need a real Blender
  export (the test box; a small skinned figure with the 20 joints). The rifle, unseen here (A3, A10 describe it), "stays
  with me until you ask for it" (A3): change-set M's plan asks.
- **How mark 4 can miss.** AX-R2's call cuts a five-joint vertex to four weights, and whether the exporter renormalises
  them was not found (AX-F10): if it does, the file passes, the fifth influence lost unseen; `JOINTS_1` shows only with
  Include All Bone Influences on. Mark 7 checks each socket's whole frame; mark 10's line 7 carries `palm` (PM-0005-56).

## 6. Materials to slot classes, and colour (OQ-0005-AS4)
- **The convention for the first models:** one plain material per colour area, named by its class prefix (AX-R8, AX-R9),
  with no renaming step. The names survive by the sources (AX-F8, AX-F9, AX-F15): that answers, not yet by an export,
  what the draft marks "not researched": "whether the glTF export keeps material names" (0005:1895-1896). The engine
  finds a slot by its material's name, never a mesh's, since mesh names get `_1`, `_2` (AX-F14; inferred). Whether
  LOOK_V2 §8's automated renaming step comes later is OQ-0005-AP1's. A `faction_` material's colour in Blender is a
  placeholder; the faction's data gives one colour per such area at load (0005:1764-1765, 1582-1583).
- **More than four areas:** a vertex-colour mask (rst:1084-1099; AX-F16) or a texture (rst:1115-1119), not researched.

## 7. Open points for Mustafa (decided nowhere here)
**O1. The draw cap when a gun comes from its own file.** ADR 0005's cap on a soldier as drawn (body, headgear and held
weapon) reads "at most 4 materials per pass", derived from C-B's "one per material per soldier" and ADR 0003's cap of 4
draw calls per soldier per pass (0005:1777-1781; r3:1044-1045); PM-0005-36 counts draws, the soldier shown minus hidden,
at most 4 × the passes that draw it (0005:2334), and so does mark 8. A gun loaded from its own file is a mesh of its own
and draws on its own, whatever its material is named (AX-F14, AX-F18; inferred, the renderer's draw loop not read):

| What is drawn | Distinct materials | Draw calls per pass |
|---|---|---|
| the body, four areas (AX-F9, AX-F14) | 4 | 4 |
| with the rifle: `unit_dark`, "2 meshes" (A3) | 4 | 6 (12 over the main and shadow passes, above PM-0005-36's 8) |
| with the SMG: `unit_dark` (A6, CONVENTIONS), its mesh count not given | 4 | 4 + its meshes |
| each shown `marking_` node (AX-R12) | 4 | + 1 |

Three ways out, none researched further; each that changes an ADR line needs Mustafa's go, and the question goes to
him with change-set M's plan.
- **(a) Merge each gun's meshes into one at load.** Costs: one merge per gun type, shared by every soldier who carries
  it (not measured). Gets: 5 draws per pass with any gun, still above 4, so it helps only beside (b) or (c). Needs
  proven: the sockets survive the merge (separate nodes, AX-F16; inferred) and PM-0005-36's count on both backends.
- **(b) At load, add the gun's geometry to the body's `unit_dark` geometry, weighted 1.0 to `hand_r`.** Gets: 4 draws
  per pass with any gun. Costs: one merged geometry per body and gun pair, shared by every soldier with that pair
  (memory not measured); a geometry swap when a soldier changes guns (time not measured); the gun then moves only with
  `hand_r`, so the body step sets the hand's turn from the gun's direction (JL §4), and the sockets ride on `hand_r`.
  Needs proven: memory per pair and the swap's frame time on the minimum machine; one draw per material on both
  backends (PM-0005-36); flash and tracer still starting at `socket_muzzle`, within 1 mm (test-engineer's ruling).
- **(c) Count the cap in distinct materials and allow more draws per soldier.** Costs: decision-doc lines change, ADR
  0005's derived cap (0005:1777-1781), PM-0005-36 (0005:2334), ADR 0003's cap of 4 draw calls per soldier per pass
  (r3:1044-1045; PM-0003-25, 0003:1200, which does not say whether its soldier holds a gun) and mark 8; with the rifle,
  6 draws per soldier per pass, 288 per pass for 48 soldiers instead of 192 (arithmetic). Needs proven: the render
  budget bench (BP Part 3, section 3.7: PM-0003-18 to -47) holds at that count on the minimum machine.
- Either way, a per-object hull outline would double each count (r3:1021).

**O2. The fifth class's name.** ADR 0005 lists the classes "for example environment, structure, unit, faction colour,
emissive" (0005:1761-1762; the same list in AP-3, 0005:1579); ADR 0003 lists "(environment, structures, units, faction
colours, visual effects)" (0003:247-248), not as an example. The prefix `emissive_` (AX-R8; BP Part 11, rule 12)
follows ADR 0005. Nothing here decides which name holds; the class table (AP-3, 0005:1578-1585) is where it would be
settled, and an ADR line that changes needs Mustafa's go.

## 8. What stays open

| Open point | Who decides |
|---|---|
| O1 and O2 (part 7) | Mustafa, with change-set M's plan; the engine proposes (main session; engine-architect reviews) |
| Whether a marking is off at game size (A5, A2); how a marking changes with the faction; names after `marking_` | Mustafa decides; the Art lead and the engine propose |
| Which sockets each gun has, and its eject direction | the gun's entry, from its author (the Art lead), with the engine |
| Where the joints sit, weights beyond the rigid baseline, whether the coat keeps its four joints | JL §6 |
| The bind pose; the soldier's vertex and bone counts (AS6) | the model's author |
| How the exporter writes the armature object; the neutral bone's name; the influence cut; Mustafa's Blender version against add-on 5.4.12 | the first real export (the advisor agent's ruling, part 5) |
| AS1, AS2, AS5, AP1 | the rest of the asset research (SP:568-586) |

**Not read (and what for).**
- Blender's manual and the rendered specification, not on the allow-list: the exporter's repository documentation and
  the specification's source were read instead (lines after 2180 moved by 4 between my two reads of it).
- The exporter: its influence cut and any renormalising (mark 4); the neutral bone's name (mark 3); how it writes the
  armature object (a non-joint node is likely: objects become nodes, AX-F8, and the option that removes the armature
  object is off by default, src `__init__.py`:834-841; inferred; JL §2); how an object parented to a bone is written
  (AX-R12); what `use_selection` takes along; `filepath` (AX-R2); Blender's Unit Scale (mark 6) and ".001" (AX-R7).
- three.js: the renderer's draw loop (AX-F18, O1; cited through r3); `normalizeSkinWeights` on a vertex with no weight
  (mark 4); the uniform buffer limit per backend (20 joints need 20 × 64 = 1,280 bytes on SK:61-65's path; the limit
  was not read). The Khronos glTF Validator (a candidate for J).

**What the first real export must prove.** Export with AX-R2's call, from Mustafa's Blender through the MCP or a
headless Blender (`blender --background --python` [from memory, unverified]): mark 6's test box; a small skinned figure
with the Conscript's four slots and the 20 joints (marks 3, 4, 10); a gun with a turned empty (mark 7's Blender half).
Load each on WebGPU and with `forceWebGL` (mark 9), check names, one mesh per slot, one bound Skeleton, no `JOINTS_1`,
the box's sizes and front, the soldier's origin, facing and scaling, each socket's place and whole frame and, once the
entry holds it, `palm`; then plant each mark's defect and see it fail. Headless runs check structure and counts only,
and probably never get WebGPU (ADR 0002 part 9). **Nothing in this file was proven by an export.**

## 9. Sources
All official, none from shipped games; each URL once, with the date read.
1. https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/Specification.adoc · 2026-11-14, 2026-11-17.
2. https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/schema/extras.schema.json · 2026-11-14.
3. https://github.com/KhronosGroup/glTF-Blender-IO/blob/main/docs/blender_docs/scene_gltf2.rst · 2026-11-14.
4. https://github.com/KhronosGroup/glTF-Blender-IO/tree/main/addons/io_scene_gltf2 · 2026-11-14: `__init__.py` and, in
   `blender/exp/`, `nodes.py`, `obj_data.py`, `material/material_utils.py`, `joints.py`, `skins.py`,
   `primitive_extract.py`.
5. https://github.com/mrdoob/three.js/blob/dev/examples/jsm/loaders/GLTFLoader.js · 2026-11-14 (188dev), 2026-11-17.
6. https://github.com/mrdoob/three.js/blob/dev/src/animation/PropertyBinding.js · 2026-11-14.
7. https://github.com/mrdoob/three.js/blob/dev/src/nodes/accessors/Skinning.js · 2026-11-14.
8. https://github.com/mrdoob/three.js/blob/dev/examples/webgpu_loader_gltf.html · 2026-11-14.
9. https://github.com/mrdoob/three.js/blob/dev/examples/webgpu_skinning.html · 2026-11-14.
10. Repository files, this branch, 2026-11-17: ADR 0003 and 0005, SPIKE_PLAN, EDITOR_V2, STATUS, r3, JL, the build plan
    (BP) and the appendix's items A3 to A10, read as saved for the report before it was built.
