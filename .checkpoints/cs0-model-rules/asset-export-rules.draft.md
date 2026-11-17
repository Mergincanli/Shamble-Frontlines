# Asset export rules: the export and slot-naming part of the asset research

- **Topic:** the export and slot-naming part of the asset-pipeline research: OQ-0005-AS3, AS4
  (`docs/adr/0005-content-abilities-assets.md`:1874-1889; `docs/SPIKE_PLAN.md`:529-545: "graphics-engineer, verified
  like R3").
- **Author agent:** graphics-engineer (Opus 5.5). **Date:** 2026-11-17; sources read 2026-11-14, and the glTF
  specification and three.js's loader read again 2026-11-17.
- **Status:** draft, not verified. No export has gone through these rules yet.
- **The questions:** "Which checks does an export pass before it enters the game?" (OQ-0005-AS3, 0005:1874); "How are a
  model's materials renamed into the slot classes and faction colour slots?" (OQ-0005-AS4, 0005:1881).

**TL;DR.**
1. Blender's exporter writes material and bone names as they are and makes one primitive per used material; three.js
   keeps material names and changes node names only for spaces and `[ ] . : /`, so lower-case names survive both.
2. The rules (part 4) fix the file and its one export call, scale, axes, origin, names, the skin and what stays out.
3. Nine checks with a planted defect each answer AS3 on paper; three.js alone does not refuse an unknown extension.
4. Three points speak against the plan's proposals (part 5.2): the draw cap, the rifle's sockets, the fifth class.
5. Nothing here was proven by an export; the round trip in part 7 must prove it.

**How to read this file.** "Confirmed": the source's own words or code, cited by line; "inferred": my reading or
arithmetic; "[from memory, unverified]": no source read. Rules are marked "from a draft" (an ADR, SPIKE_PLAN, EDITOR_V2
or LOOK_V2 line), "from a source" (specification, exporter, loader) or "the engine's convention (proposed)" (the main
session proposes, engine-architect reviews). Names and file conventions are the engine's; shapes, colours, proportions
and line widths are the look (Mustafa's and the Advisor's), and none is a rule here. Against the README's shape
(`docs/research/README.md`:36-48), findings are grouped by source, rules and checks stand for recommendations, options
appear only in part 6, and the verifier adds its spot-check table.

**Abbreviations.** Repository lines are this branch's on 2026-11-17. **0003**, **0005**: `docs/adr/0003-rendering.md`
and `docs/adr/0005-content-abilities-assets.md`; **SP**: `docs/SPIKE_PLAN.md`; **r3**: `docs/research/r3-rendering.md`;
**plan**: `.checkpoints/cs0-model-rules/plan.md`; **go**: Mustafa's message of 2026-11-17 (`mustafa-2026-11-17-go.md`)
and **S**, **G**, **A**: the Art lead's soldier brief (2026-11-14), SMG brief and addition (2026-11-15) (`inputs/`),
both in `.checkpoints/build-plan-slice-1/`; in `docs/`, these five citations move to the report appendix's lines.
**spec**, **rst**, **src**, **loader**, **PB**, **SK**: sources 1 and 4 to 8 (part 8).

## 1. Scope
Covered: the export settings and call, scale, axes and origin, names, caps, the skin and what stays out (part 4); the
checks a file passes (AS3, part 5); how materials become slot classes and carry colour (AS4, part 6). Not covered, the
rest of the asset research (SP:529-534): compression (OQ-0005-AS1), LOD tools and levels (AS2), per-unit budgets beyond
the two derived caps (AS5), vertex and bone counts (AS6), where the automated step runs and who writes the asset list
(OQ-0005-AP1), serving files (AP-4), very large units' part names (ADR 0004's records), and whether model files enter
the repository (his answer MODEL-PUBLIC of 2026-11-17, go:24).

## 2. What the decision drafts already fix
- Only glTF/GLB, through three.js's loader on both backends (0005:1803-1804); the models' route is Blender (SP:537-542);
  test models are plain exports with no compression or LOD step (SP:390-391; 0005:1715).
- A model carries no look: geometry, its skin, UVs and textures where it has them, and named material slots, but no
  baked lighting or ink; at load each slot gets a shared toon material of its class (0005:1748-1755).
- Each slot's name maps to one class "from a small fixed list (for example environment, structure, unit, faction colour,
  emissive" (0005:1761-1762), a content table (AP-3, 0005:1578-1585); ADR 0003 lists "(environment, structures, units,
  faction colours, visual effects)" (0003:247-248). Faction slots take the faction's colour at load (0005:1764-1765);
  only emissive slots glow (0005:1768-1769).
- "A soldier model is one skinned mesh" (0005:1771-1772). "*A soldier as drawn* (body, headgear and held weapon) uses at
  most 4 materials per pass", derived from C-B's "one per material per soldier" and a cap of "4 draw calls per pass
  before the ink" (0005:1778-1781); PM-0005-36 counts draw calls with the soldier shown minus hidden and bites on "a
  soldier with 5 material slots (10 draws over 2 passes, above 8)" (0005:2311). About 5.2k triangles per soldier as
  drawn is a planning ceiling, not a budget (0005:1787-1792).
- No collider, hit shape or shot origin comes from a model; "a weapon model's sockets are cosmetic" (0005:1550-1553).
- 1 unit = 1 metre is `[DECIDED]` in `docs/EDITOR_V2.md`:63-65, not yet in an ADR (`docs/STATUS.md`:76); Mustafa,
  2026-11-17: "Units stay as decided in docs/EDITOR_V2.md, "1 unit = 1 metre"." (go:45).
- No draft fixes slot names, origin, facing, joints or sockets; the plan proposes them (plan:34-57), and Mustafa,
  2026-11-17: "The brief's names for colour areas and bones are working names; the names of your change-set 0 win."
  (go:46).

## 3. What the sources say

### 3.1 The glTF 2.0 specification
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
| Custom Properties, Cameras, Punctual Lights (`export_extras`, `export_cameras`, `export_lights`) | False, False, False | the same | rst:721-723, 1028-1031; 722-726, 650-654, 1027-1032 |
| Draco, Meshopt (`export_draco_mesh_compression_enable`, `export_meshopt_compression_enable`) | False, False | the same | rst:1193-1218; 494-498, 474-478 |

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
  1671-1677); the docs page calls each decoder "Required" (source 9) · confirmed 2026-11-17. Refusing a required
  extension is the engine's own check (inferred).
- **AX-F13. Skins.** `JOINTS_0` and `WEIGHTS_0` become `skinIndex` and `skinWeight` (loader:2339-2340); other attributes
  are only lower-cased (4952), so `JOINTS_1` arrives unused, and the skinning node reads four (SK:102-117). A skinned
  primitive with both attributes becomes a `SkinnedMesh`, otherwise a `Mesh` with "Skinning disabled." (3992-4004);
  weights are normalised at load "to fix malformed assets" (4009); joints become `Bone`s (2836, 4479-4481); a missing
  joint warns (4186); all SkinnedMeshes of the node bind one Skeleton (4374-4378) · confirmed (the limit of four
  inferred).
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
  `WebGPURenderer` (sources 10, 11) · confirmed. A skinned GLB with `forceWebGL`: no example I read shows it.
- **AX-F18. Draw calls.** A separate mesh draws on its own: C-B merges a soldier's parts "into one geometry per
  material" (r3:825); legacy's merging cut the village from 398 to 202 draw calls (r3:303); `InstancedMesh` "will help
  you to reduce the number of draw calls" (r3:452-455) · inferred from these; the renderer's draw loop was not read.

## 4. The rules

### 4.1 The file and the export
- **AX-R1.** One model, one `.glb` file, written by Blender's built-in glTF 2.0 exporter. [from a draft: 0005:1803-1804,
  SP:537-539; from a source: rst:881-885]
- **AX-R2.** Export with AX-F7's "Rule" column: in a Blender script (the MCP route), one call, after selecting every
  object of the model (what `use_selection` takes along was not read; the `filepath` parameter is
  [from memory, unverified]). [the engine's convention (proposed); option names from a source, src `__init__.py`]

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
  never the Armature (AX-F7). [the engine's convention (proposed), as G:14 asks; why a skin needs it: AX-F5]

### 4.2 Scale, axes and origin
- **AX-R4.** One unit is one metre; build at real size. The soldier's 1.80 m is a placeholder:
  "hang the soldier on one root so it scales with one number." (Mustafa, 2026-11-17, go:44). [from a draft:
  `docs/EDITOR_V2.md`:63-65; from a source: spec:737]
- **AX-R5.** A soldier: +Y up, front +Z, his left +X (in Blender: Z up, front -Y, left +X); his origin on the ground
  between his feet; every node of his under `root`. [axes from a source: spec:730-732, src `nodes.py`:611-616, and the
  briefs agree, S:11 and S:73; the origin: the engine's convention (proposed), plan:51-52]
- **AX-R6.** A gun: +Y up, +Z to the muzzle, +X to the gun's left, origin at `socket_grip` (in Blender: Z up, muzzle
  -Y). [the engine's convention (proposed), plan:52-53, as G:11 and G:14 write it; the rifle matches (go:48); +X left
  agrees with spec:732]

### 4.3 Names
- **AX-R7.** Every name in a file (objects, meshes, materials, bones, sockets) uses lower-case letters, digits and
  underscores only: no spaces, dots, colons, slashes or brackets, so no Blender ".001" ending (Blender adds one to a
  duplicated name [from memory, unverified]). [the engine's convention (proposed), plan:37; from a source: AX-F15]
- **AX-R8.** A material's name is its slot name, and its prefix is its class: `faction_` (takes the faction's colour;
  the faction's data holds one colour per such area), `unit_`, `emissive_`, `structure_`, `environment_`. [the engine's
  convention (proposed), plan:37-41; the drafts give the classes only "for example" (0005:1761-1762) and differ on the
  fifth (part 5.2, point 3)]
- **AX-R9.** The Conscript's four areas are `faction_coat`, `faction_top`, `faction_trim` and `unit_dark`, for the
  brief's Coat, Top, Trim and Dark (S:20-25), whose table varies the first three by faction and keeps Dark the same.
  "No skin colour on this first stand-in: hands and face take the Dark material, so four materials hold, the rifle included."
  (Mustafa, 2026-11-17, go:43). A gun's one material is `unit_dark` too (G:12, A:18). [the engine's convention
  (proposed), plan:37-39]
- **AX-R10.** The Conscript's 14 bones: `root`, `hips`, `chest`, `head`; `upper_arm_l`, `forearm_l`, `hand_l` and the
  same three with `_r`; `leg_l`, `foot_l`, `leg_r`, `foot_r`. Parents: root > hips > chest > head; chest > upper arm >
  forearm > hand; hips > leg > foot; the leg is one bone from hip to ankle (S:33). The list is the model's, not the
  engine's. `root` is the armature's top bone, not the armature object and not an empty (S:73's "one empty at the
  origin" becomes this bone); vertex groups carry the bone names exactly. [the engine's convention (proposed),
  plan:42-46, after the brief's bone column (S:29-50); groups map to bones by name: from a source, AX-F10]
- **AX-R11.** A gun's sockets are empty nodes (Blender empties under the gun) named `socket_grip`, `socket_support`,
  `socket_muzzle`, `socket_sight` and `socket_eject` (names: go:51); `socket_grip` is the origin. The right hand holds
  `socket_grip`, the left goes to `socket_support`, and where a pose disagrees, "the sockets win" (Mustafa, 2026-11-17,
  go:49). Flash and tracer start at `socket_muzzle`; the simulation never reads it (0005:1550-1553; go:50). [empties:
  G:35; a socket loads as a named Object3D: AX-F16; the rest: the engine's convention (proposed), plan:47-50] Open:
  which sockets a gun has (point 2) and which local axis is a socket's direction (G:40's eject socket points "right and
  20 degrees up"); proposed: the gun's entry lists them, and a socket points along its local +Z.
- **AX-R12.** A node whose name starts with `marking_` is a part the game may hide, using one of the model's own
  materials; since a marking differs per faction in shape and colour area (S:57), each faction's marking is its own node
  (inferred). A shown marking adds draw calls (AX-F18); how it follows its bone was not read. [the engine's convention
  (proposed), plan:54-55; "off at game size and on in close views", A:13]

### 4.4 Caps, the skin, and what stays out
- **AX-R13.** A soldier as drawn, his gun included, uses at most 4 materials per pass, and the draft's mark counts draw
  calls (0005:1778-1781, 2311): a gun from its own file adds draws whatever its material is called (point 1). A very
  large unit: parts × materials at most 25 per pass (0005:1784-1786). Triangles: no cap; about 5.2k per soldier as drawn
  is a planning ceiling (0005:1787-1792), and the rifle alone has 2,596 (go:48). [from a draft]
- **AX-R14.** A soldier is one mesh object: the parts are joined before export and parented to one armature, each part's
  vertices weighted 1.0 to the bone the brief names for it (a rigid skin) until someone paints weights. The brief's "one
  object per part" (S:73) is the modelling form; exported as it is, each part would be a node and a mesh of its own.
  [from a draft: 0005:1771-1772, r3:825; joining: the engine's convention (proposed)]
- **AX-R15.** Every vertex is weighted to one to four bones, summing to 1; an unweighted vertex makes the exporter add a
  neutral bone. The armature's rest position is the bind pose (rst:1159-1160); the model's author chooses that pose. A
  gun has no armature and no skin. [from a source: AX-F4, AX-F10, AX-F13; the gun: the engine's convention (proposed),
  after A:17, "The rifle is a model of its own."]
- **AX-R16.** No images or textures, vertex colours, animations, shape keys, cameras, lights, custom properties,
  compression or LOD levels; no baked lighting, ink or toon shading; no collider, hit shape, shot origin or navigation
  data. [from a draft: 0005:1748-1750, SP:390-391, 0005:1715, 0005:1550-1553; "No textures" is also S:69; no animations:
  the engine's convention (proposed), since soldiers are animated in code (0003:306, 0003:797)]

## 5. The checks a file passes (OQ-0005-AS3)

### 5.1 Nine marks
The plan's nine marks (plan:59-70), each with a defect a test plants and must see fail; they join ADR 0005 as PM-0005-47
onward at change-set M (plan:21) and stay written, not run, until a real export exists (plan:71). **J** is a script over
the GLB (the header and first chunk, AX-F6; the binary chunk only for weights); **L** is the game's loader in a test on
both backends (AX-F12 to AX-F17). The Khronos glTF Validator might do part of J; not read.

| # | Mark (plan:62-70) | Planted defect | Run by | Not confirmed |
|---|---|---|---|---|
| 1 | GLB, glTF 2, no required extension | `extensionsRequired: ["KHR_draco_mesh_compression"]`; an unknown name there | J: magic, version 2, JSON chunk first, `asset.version`, no `extensionsRequired` (spec:624, 3644, 3703-3714, 3773). L throws for Draco without a decoder but only warns on an unknown name (AX-F12) | none |
| 2 | every primitive has a material named in the slot table | a primitive with no material; a material `unit_dark.001` | J: `primitives[].material` set, `materials[].name` in the table. L keeps the names (loader:3817) but draws the default material silently (2370-2386) | whether Blender ever writes a primitive with no material |
| 3 | bone names equal the model's list; no extra joint | one unweighted vertex (the neutral bone); a leaf bone `_leaf` | J: `skins[0].joints` mapped to `nodes[].name`; L: bones by `userData.name` (loader:4509) | the neutral bone's name; whether the armature object becomes a joint |
| 4 | one joint and weight set; every vertex weighted | Include All Bone Influences on and a vertex with five weights (`JOINTS_1` appears) | J: no `JOINTS_1` or `WEIGHTS_1`; weights from the binary chunk sum to 1. L alone cannot: it normalises weights at load (loader:4009) | the exporter's cut and renormalising (AX-F10) |
| 5 | normals; no image, animation, morph target, camera or light | the defaults on a rig with an action (Animations and Shape Keys are on by default) | J: `NORMAL` in every primitive; no `images`, `animations`, `cameras`, `targets` or `KHR_lights_punctual` | none |
| 6 | a 1 m test cube is 1.0 m along +Y and faces +Z | the cube built at 100 units; +Y Up off | J: `POSITION` min and max (spec:1689), node transforms identity; L: the bounding box | a cube has no front: the test object needs a marker on its +Z side; whether Blender's Unit Scale changes the export |
| 7 | a gun's sockets by name; `socket_grip` at the origin, `socket_muzzle` on +Z | `socket_muzzle.001`; a gun exported muzzle to -Z | L: each socket by name and position (loader:4493); J: `nodes[].name`, `translation` | the rifle has four sockets (point 2), and its names, extensions and origin were not seen (go:48); the socket axis (AX-R11) |
| 8 | counts inside the two derived caps | a fifth material; a soldier with gun above 4 draws per pass | J counts distinct materials; draws need the renderer: PM-0005-36's shown minus hidden (0005:2311) | what the mark counts (point 1) |
| 9 | the same result on WebGPU and with `forceWebGL` | the plan names none; PM-0005-42's "a backend path that drops the slot's colour" fits (0005:2318) | L run twice in a browser; headless runs give structure and counts, never frame rates as hardware results | a skinned GLB with `forceWebGL` (AX-F17) |

### 5.2 Where a source or a draft speaks against the plan's proposals
- **Point 1, the draw cap.** The plan (plan:38-39) and the briefs (G:12, A:18) keep the soldier with his gun at 4
  materials by naming the gun's material `unit_dark`, and Mustafa's words agree for materials (go:43). But the draft
  derives the cap from 4 draw calls per pass, and PM-0005-36 counts draws (0005:1778-1781, 2311). A gun from its own
  file is its own mesh whatever its material is called (AX-F14, AX-F18), and the rifle has "2 meshes" (go:48): 4 plus 2
  is 6 draws per pass, 12 over the main and shadow passes, above PM-0005-36's 8. Mark 8 (plan:69) counts materials and
  would pass what PM-0005-36 fails. Ways out: part 6.
- **Point 2, five sockets.** Mark 7 asks for "a gun's five sockets" (plan:68), and the plan offers the rifle as the
  first file (plan:71-72). The rifle has "four sockets" (go:48), "grip, support, muzzle and sight" (S:52), and
  `socket_eject` is the SMG's new one (go:51), so the first file fails mark 7 as written. Proposed: mark 7 checks the
  sockets the gun's entry lists, or the rifle gains an eject socket (its author decides).
- **Point 3, the fifth class.** The plan's prefixes include `emissive_` (plan:40-41), from ADR 0005's example list
  (0005:1761-1762); ADR 0003's list has "visual effects" in that place (0003:247-248). Both lists are examples, so the
  prefixes stay a proposal, and the class table (AP-3, 0005:1578-1585) should settle which list holds.
- Smaller points, not against a source: mark 6's cube has no front (row 6); the plan should say `root` is a bone.

## 6. Materials to slot classes, and colour (OQ-0005-AS4)
- **The convention for the first models:** one plain material per colour area, named by its class prefix (AX-R8, AX-R9),
  with no renaming step. The names survive by the sources (AX-F8, AX-F9, AX-F15): that answers, not yet by an export,
  what the draft marks "not researched": "whether the glTF export keeps material names" (0005:1885-1886). The engine
  finds a slot by its material's name, never a mesh's, since mesh names get `_1`, `_2` (AX-F14; inferred); LOOK_V2 §8's
  renaming step is then not needed (inferred; whether it comes later is OQ-0005-AP1's). A `faction_` material's colour
  in Blender is a placeholder; the faction's data gives it at load (0005:1764-1765, 1582-1583).
- **The counts for the Conscript with his gun** (the draft allows 4 per pass, 8 over the main and shadow passes:
  0005:1778-1781, 2311):

| What is drawn | Distinct materials | Draw calls per pass |
|---|---|---|
| the body, four areas (AX-F9, AX-F14) | 4 | 4 |
| with the rifle: `unit_dark`, "2 meshes" (go:48) | 4 | 6 |
| with the SMG: `unit_dark` (G:12), its mesh count not given | 4 | 4 + its meshes |
| each shown `marking_` node (AX-R12) | 4 | + 1 |

- **Ways to 4 draws per pass**, none researched (the engine proposes; an ADR line that changes needs Mustafa's go): (a)
  merge each gun's meshes at load: 5 draws, still above 4; (b) at load, add the gun's geometry to the body's `unit_dark`
  geometry, weighted 1.0 to `hand_r`: 4 draws, one merged geometry per body and gun pair, shared by every soldier with
  that gun and swapped when he changes guns (memory and swap cost not measured); (c) count the cap in distinct materials
  and allow more draws per soldier, changing ADR 0003's cap of 4 draw calls per soldier per pass (r3:1044-1045;
  0005:1779-1780) and PM-0005-36. A per-object hull outline would double each count (r3:1021).
- **More than four areas, routes left open:** a vertex-colour mask inside one material per class (the exporter writes
  vertex colours, rst:1084-1099; the loader turns them on, AX-F16), or one texture (images exported, rst:1115-1119). How
  ADR 0003's toon material reads either: not researched. The Conscript of the briefs needs neither.

## 7. What stays open

| Open point | Who decides |
|---|---|
| What mark 8 and PM-0005-36 count; route (a), (b) or (c) of part 6 (point 1) | the engine proposes (main session; engine-architect reviews); Mustafa for any ADR line |
| Which sockets each gun has; an eject socket on the rifle (point 2) | the gun's author (the Art lead), with the engine |
| The fifth class (point 3) and the class table (AP-3); a socket's axis; `marking_` names after the prefix | the engine (the class table reviewed with ADR 0003's material) |
| The bind pose; the soldier's vertex and bone counts (AS6) | the model's author |
| Mustafa's Blender version against add-on 5.4.12; the not-read points below | this research, by the round trip |
| AS1, AS2, AS5, AP1 | the rest of the asset research (SP:529-545) |

**Not read (and what for).**
- Blender's manual (docs.blender.org) and the rendered specification (registry.khronos.org), not on the allow-list: the
  exporter's repository documentation and the specification's source were read instead. That source changed between my
  reads (205,509 bytes on 2026-11-14, 205,402 on 2026-11-17; lines after 2180 moved by 4; the GLB size header reworded):
  a pinned release would be steadier.
- The exporter: its influence cut and any renormalising; the neutral bone's name; whether the armature object becomes a
  node or a joint; how an object parented to a bone is written; what `use_selection` takes along; `filepath` (AX-R2,
  AX-R12, AX-R15; marks 3, 4). Blender: Unit Scale's effect on export; the ".001" ending; whether its mirror tools read
  `_l` and `_r` (they do [from memory, unverified]) (AX-R7, AX-R10; mark 6).
- three.js: the renderer's draw loop (AX-F18, cited through r3); `normalizeSkinWeights` on a vertex with no weight (mark
  4); the uniform buffer limit per backend (14 bones need 14 × 64 = 896 bytes on SK:61-65's path; the limit was not
  read). The Khronos glTF Validator (a candidate for J).

**What a real export must still prove (the round trip).** Export, with AX-R2's call from Mustafa's Blender through the
MCP or a headless Blender (`blender --background --python` [from memory, unverified]), a 1 m cube marked on its +Z side,
a skinned figure with the Conscript's four slots and 14 bones, and a gun with its sockets. Load each on WebGPU and with
`forceWebGL`; check names as written, one mesh per slot, one bound Skeleton, no `JOINTS_1`, 1.0 m along +Y, front and
muzzle on +Z; then plant each mark's defect and see it fail. Headless runs check structure and counts only. **Nothing in
this file was proven by an export.**

## 8. Sources
All official, none from shipped games; each URL once, with the date read.
1. https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/Specification.adoc · 2026-11-14, 2026-11-17.
2. https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/schema/extras.schema.json · 2026-11-14.
3. https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/schema/glTFChildOfRootProperty.schema.json ·
   2026-11-14 (a name "is not necessarily unique").
4. https://github.com/KhronosGroup/glTF-Blender-IO/blob/main/docs/blender_docs/scene_gltf2.rst · 2026-11-14.
5. https://github.com/KhronosGroup/glTF-Blender-IO/tree/main/addons/io_scene_gltf2 · 2026-11-14: `__init__.py` and, in
   `blender/exp/`, `nodes.py`, `obj_data.py`, `material/material_utils.py`, `joints.py`, `skins.py`,
   `primitive_extract.py`.
6. https://github.com/mrdoob/three.js/blob/dev/examples/jsm/loaders/GLTFLoader.js · 2026-11-14 (188dev), 2026-11-17.
7. https://github.com/mrdoob/three.js/blob/dev/src/animation/PropertyBinding.js · 2026-11-14.
8. https://github.com/mrdoob/three.js/blob/dev/src/nodes/accessors/Skinning.js · 2026-11-14.
9. https://threejs.org/docs/pages/GLTFLoader.html · 2026-11-14, through the fetch tool's summary.
10. https://github.com/mrdoob/three.js/blob/dev/examples/webgpu_loader_gltf.html · 2026-11-14.
11. https://github.com/mrdoob/three.js/blob/dev/examples/webgpu_skinning.html · 2026-11-14.
12. Repository files, this branch, 2026-11-17: ADR 0003 and 0005, SPIKE_PLAN, EDITOR_V2, STATUS, r3, and the five
    checkpoint files
under "Abbreviations".
