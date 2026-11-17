# Asset export rules: the export and slot-naming part of the asset research

- **Topic:** the export and slot-naming part of the asset-pipeline research, OQ-0005-AS3 and OQ-0005-AS4
  (`docs/adr/0005-content-abilities-assets.md`:1874-1889; `docs/SPIKE_PLAN.md`:529-545, "graphics-engineer,
  verified like R3").
- **Author agent:** graphics-engineer (Opus 5.5).
- **Date:** 2026-11-17. Sources read 2026-11-14; the glTF specification and three.js's loader read again 2026-11-17.
- **Status:** draft, not verified. No export has gone through these rules yet.
- **The questions (quoted):** "Which checks does an export pass before it enters the game?" (OQ-0005-AS3,
  0005:1874); "How are a model's materials renamed into the slot classes and faction colour slots?" (OQ-0005-AS4,
  0005:1881).

**TL;DR.**
1. Blender's exporter writes material and bone names as they are and makes one primitive per used material; three.js
   keeps material names and changes node names only for spaces and `[ ] . : /`, so lower-case names survive both.
2. The rules (part 4) fix the file and its one export call, scale, axes, origin, names, the skin and what stays out.
3. Nine checks with a planted defect each answer AS3 on paper; three.js alone does not refuse an unknown extension.
4. Three points speak against the plan's proposals (part 5.2): the draw cap, the rifle's sockets, the fifth class.
5. Nothing here was proven by an export; the round trip in part 7.3 must prove it.

**How to read this file.**
- Certainty: "confirmed" means the source's own words or code, cited by line; "inferred" means my reading or
  arithmetic; "[from memory, unverified]" means no source was read for it.
- Each rule is marked "from a draft" (an ADR, SPIKE_PLAN, EDITOR_V2 or LOOK_V2 line), "from a source" (the glTF
  specification, the exporter, the loader) or "the engine's convention (proposed)" (no draft or source fixes it; the
  main session proposes it and engine-architect reviews it).
- Names and file conventions are the engine's. Shapes, colours, proportions and line widths are the look, Mustafa's
  and the Advisor's: none is a rule here, and the briefs' numbers stay the Art lead's proposals.
- Where this file leaves the README's shape (`docs/research/README.md`:36-48): it is a short topic, so the findings
  are grouped by source (part 3), the rules and checks stand where the README has recommendations (parts 4 and 5),
  options appear only where a question has them (part 6), and the verifier adds its table when it checks the file.

**Abbreviations.** Repository lines are this branch's on 2026-11-17. **0003**, **0005**: `docs/adr/0003-rendering.md`,
`docs/adr/0005-content-abilities-assets.md`; **SP**: `docs/SPIKE_PLAN.md`; **r3**: `docs/research/r3-rendering.md`.
**plan**: the change-set's plan; **go**: Mustafa's message of 2026-11-17; **S**, **G**, **A**: the Art lead's soldier
brief (2026-11-14), SMG brief (2026-11-15) and addition (2026-11-15). Those five are cited at their checkpoint copies
(`.checkpoints/cs0-model-rules/plan.md`, `.checkpoints/build-plan-slice-1/mustafa-2026-11-17-go.md` and `inputs/`);
when this file moves into `docs/`, those citations move to the report appendix's lines. **spec**: the glTF 2.0
specification source (source 1), lines as read 2026-11-17. **rst**: the exporter's documentation (source 4). **src**:
the exporter's source (source 5). **loader**: `GLTFLoader.js` (source 6). **PB**, **SK**: `PropertyBinding.js`,
`Skinning.js` (sources 7, 8).

## 1. Scope

Covered: the export settings and the one-call form for a Blender script; scale, axes and origin for a soldier and a
gun; names (slot classes, the Conscript's four colour areas, his joints, gun sockets, optional parts); the caps as they
bear on one file; the skin; what stays out of a file; the checks a file passes (AS3); how materials become slot
classes and how colour is carried (AS4).

Not covered, the rest of the asset research (SP:529-534): compression (OQ-0005-AS1), LOD tools and levels (AS2),
per-unit budgets beyond the two derived caps (AS5), the soldier's vertex and bone counts (AS6), where the automated
step runs and who writes the asset list (OQ-0005-AP1), serving files (AP-4), the very large units' part names (ADR
0004's records), and whether model files enter the repository (his answer MODEL-PUBLIC of 2026-11-17, go:24).

## 2. What the decision drafts already fix

- Only glTF/GLB, through three.js's loader, on both backends; never Spline's runtime (0005:1803-1804).
- A model carries no look: geometry, its skin, UVs and textures where it has them, and named material slots; no baked
  lighting and no ink; at load each slot gets a shared toon material of its class (0005:1748-1755).
- Every slot's name maps to one class "from a small fixed list (for example environment, structure, unit, faction
  colour, emissive" (0005:1761-1762); the list and the names are content data (AP-3, 0005:1578-1585). ADR 0003's own
  list reads "(environment, structures, units, faction colours, visual effects)" (0003:247-248).
- Faction colour slots take the faction's colour at load, one shared material per slot and faction (0005:1764-1765);
  only emissive slots glow (0005:1768-1769).
- "A soldier model is one skinned mesh" (0005:1771-1772); a very large unit is a group of named parts (0005:1774-1775).
- The caps: "*A soldier as drawn* (body, headgear and held weapon) uses at most 4 materials per pass", derived from
  C-B's "one per material per soldier" and ADR 0003's cap of "4 draw calls per pass before the ink" (0005:1778-1781).
  Its mark PM-0005-36 counts draw calls with the soldier shown minus hidden, and bites on "a soldier with 5 material
  slots (10 draws over 2 passes, above 8)" (0005:2311). A very large unit: 25 parts × materials per pass
  (0005:1784-1786). Triangles: about 5.2k per soldier as drawn is a planning ceiling, not a budget (0005:1787-1792).
- The test models are plain exports with no compression or LOD step (SP:390-391; 0005:1715).
- No collider, hit shape or shot origin is read from a model, and "a weapon model's sockets are cosmetic"
  (0005:1550-1553).
- 1 unit = 1 metre is `[DECIDED]` in `docs/EDITOR_V2.md`:63-65 and not yet in an ADR (`docs/STATUS.md`:76). Mustafa,
  2026-11-17: "Units stay as decided in docs/EDITOR_V2.md, "1 unit = 1 metre"." (go:45).
- The route for the models is Blender (SP:537-542).
- No draft fixes the slot names, the origin, the facing, the joint list or the socket names. The plan proposes them
  (plan:34-57), and Mustafa, 2026-11-17: "The brief's names for colour areas and bones are working names; the names of
  your change-set 0 win." (go:46).

## 3. What the sources say

### 3.1 The glTF 2.0 specification

**AX-F1. Axes.** "glTF uses a right-handed coordinate system." (spec:730) "glTF defines +Y as up; the front side of a
glTF asset faces +Z, the left side of a glTF asset faces +X." (spec:732) · confirmed.

**AX-F2. Units.** "The units for all linear distances are meters." (spec:737); positions are metres "After applying
the node's global transform" (spec:2180) · confirmed.

**AX-F3. Names and extras.** Any top-level object "**MAY** have a `name`", and names "are not guaranteed to be unique"
(spec:725); extras are "Application-specific data." (source 2) · confirmed. The format enforces no naming rule of its
own, so ours must be checked (inferred).

**AX-F4. One material per primitive.** "A _mesh_ is defined as an array of _primitives_." (spec:1578); each primitive
has one optional `material`, and "If the `material` property is not defined, then a default material" is used
(spec:1584) · confirmed.

**AX-F5. Joint influences.** "Each attribute pair provides up to four joint indices and their weights per vertex."
(spec:2009); more go into `JOINTS_1`/`WEIGHTS_1` (spec:2014); clients "**MAY** support only a single set of up to four
weights and joints" (spec:2074) · confirmed. The format allows more than four; the limit of four is the client's.

**AX-F6. Weights.** Unsigned normalised weights **MUST** sum to exactly 1.0 (spec:2044); float weights **SHOULD** be as
close as reasonably possible to 1.0 (spec:2064) · confirmed.

**AX-F7. Skins.** A skin lists its joints by node index (spec:2084); the joint hierarchy "is simply the node
hierarchy" (spec:2129); "A skinned mesh primitive does not use the global transform of the node that instantiates the
mesh." (spec:2238) · confirmed. A scale put on the mesh's own node therefore does nothing; it belongs on the joints or
above them (inferred).

**AX-F8. GLB.** One binary blob holds "JSON, buffers, and images" (spec:3677), extension `.glb` (spec:3684). The file
starts with magic `0x46546C67` ("glTF") and a version, "GLB version 2" (spec:3703-3714), then the total length
(spec:3721-3728); chunk types are JSON `0x4E4F534A` and BIN `0x004E4942` (spec:3764-3765), and the JSON chunk is the
only one of its type and "**MUST** be the first chunk" (spec:3773). The JSON's `asset` "**MUST** contain a `version`"
(spec:624); every extension needed to load it "**MUST** be listed in the top-level `extensionsRequired` array"
(spec:3644); `POSITION` accessors "**MUST** have their bounds", `min` and `max` (spec:1689) · confirmed 2026-11-17. A
script can therefore read every name, count and size from the first chunk without decoding the binary one (inferred).

### 3.2 Blender's exporter (add-on 5.4.12 on its main branch, for Blender 5.3.0 and newer: src `__init__.py`:15-21)

**AX-F9. The settings.** The operator is `export_scene.gltf` (src `__init__.py`:1967). The "Rule" column is AX-R2's
choice; the line numbers after each semicolon are `__init__.py`'s, where the defaults are · confirmed for this version.

| Setting (UI name; script name) | Default | Rule | Documentation; default line |
|---|---|---|---|
| glTF Binary (.glb) (`export_format`) | `'GLB'` | `'GLB'` | rst:881-885; 335-342 |
| Selected Objects (`use_selection`) | False | True | not in rst; 656-660 |
| +Y Up (`export_yup`) | True | True | rst:1037-1038; 728-732 |
| Apply Modifiers (`export_apply`) | False | True | rst:1061-1063; 734-739 ("excluding Armatures") |
| UVs, Normals (`export_texcoords`, `export_normals`) | True, True | True, True | rst:1064-1067; 456-466 |
| Tangents (`export_tangents`) | False | False | rst:1068-1069; 548-552 |
| Materials (`export_materials`) | `'EXPORT'` | `'EXPORT'` | rst:1105-1114; 554-570 |
| Images (`export_image_format`) | `'AUTO'` | `'NONE'` | rst:1115-1119; 361-378 |
| Use Vertex Color (`export_vertex_color`) | `'MATERIAL'` | `'NONE'` | rst:1084-1095; 585-598 |
| Skinning (`export_skins`) | True | True | rst:1173; 975-979 |
| Bone Influences (`export_influence_nb`) | 4 | 4 | rst:1175-1177; 981-986 |
| Include All Bone Influences (`export_all_influences`) | False | False | rst:1179-1180; 988-992 |
| Use Rest Position Armature (`export_rest_position_armature`) | True | True | rst:1159-1160; 957-964 |
| Add Leaf Bones (`export_leaf_bone`) | False | False | not in rst; 843-850 |
| Animations (`export_animations`) | True | False | rst:732, 792-795; 747-751 |
| Shape Keys (`export_morph`) | True | False | rst:1138; 994-998 |
| Custom Properties (`export_extras`) | False | False | rst:721-723; 722-726 |
| Cameras, Punctual Lights (`export_cameras`, `export_lights`) | False, False | False, False | rst:1028-1031; 650-654, 1027-1032 |
| Draco, Meshopt (`export_draco_mesh_compression_enable`, `export_meshopt_compression_enable`) | False, False | False, False | rst:1193-1218; 494-498, 474-478 |

**AX-F10. Names are written as they are.** Object to node: `blender_object.name` (src `blender/exp/nodes.py`:486-488);
mesh data to mesh (`obj_data.py`:144-149); material to material (`material/material_utils.py`:24-25); bone to joint
node (`joints.py`:102-103, leaf bones get "_leaf"); the armature object names the skin (`skins.py`:153-154) ·
confirmed in the source; the documentation page does not say it.

**AX-F11. One primitive per used material.** With Materials "Export" or "Placeholder", triangles are bucketed by
material index, one primitive per slot that triangles use; "No Export" merges them into one (src
`primitive_extract.py`:428-444; rst:1105-1114) · confirmed.

**AX-F12. Weights and the neutral bone.** Vertex groups map to joints by name, and a group with no bone of that name is
skipped (`primitive_extract.py`:1650-1651, 1668-1669); weights at or below 0.0001 are dropped (1648, 1662); a vertex
with no bone weight goes to a "neutral bone" the exporter adds at the armature root (1672-1675) · confirmed. Where the
cut to "Bone Influences" happens, and whether weights are renormalised after it: not found [unsure].

**AX-F13. Axes.** With +Y Up, a Blender location (x, y, z) becomes (x, z, -y) (src `nodes.py`:611-616): Blender's Z up
becomes glTF +Y, and Blender's -Y becomes glTF +Z, the glTF front · confirmed for locations; rotations follow
(inferred, not read).

**AX-F14. Geometry and materials.** Quads and n-gons become triangles; UV seams and flat-shaded edges split vertices;
curves must be converted to meshes first (rst:46-53). The base colour comes from the Principled BSDF's Base Color
(rst:125-127); an emissive factor above 1.0 adds `KHR_materials_emissive_strength` (rst:264) · confirmed.

### 3.3 three.js's GLTFLoader (dev branch; the lines cited were the same on 2026-11-14 and 2026-11-17)

**AX-F15. Header and version.** A binary file not starting with "glTF" fails with "Unsupported glTF-Binary header.", a
GLB version below 2 with "Legacy binary file detected." (loader:1941-1948), a file with no JSON chunk with "JSON content
not found." (1983), and an `asset.version` below 2 with "Unsupported asset" (481-483) · confirmed 2026-11-17.

**AX-F16. Extensions.** A required extension the loader does not know only logs "Unknown extension", and loading goes
on (loader:547-549). A file that uses Draco throws without a DRACOLoader (533-534, 2000-2006); Meshopt and KTX2 throw
without a decoder only when required, and fall back otherwise (1528-1532, 1671-1677) · confirmed 2026-11-17. The docs
page calls each decoder "Required" for its extension (source 9). Refusing a required extension is the engine's own
check (inferred).

**AX-F17. Skin attributes.** `JOINTS_0` and `WEIGHTS_0` become `skinIndex` and `skinWeight` (loader:2339-2340); any
other attribute name is only lower-cased (4952), so `JOINTS_1` arrives unused; the skinning node reads four (SK:102-117)
· confirmed in the code; that three.js skins with at most four influences is inferred from it.

**AX-F18. Skinned meshes.** A primitive of a skinned mesh with both attributes becomes a `SkinnedMesh`, otherwise a
`Mesh` with "Skinning disabled." (loader:3992-4004); weights are normalised at load "to fix malformed assets" (4009);
joint nodes become `Bone`s (2836, 4479-4481); a missing joint logs a warning (4186); all SkinnedMeshes of the node bind
one Skeleton (4374-4378) · confirmed.

**AX-F19. Several materials.** One primitive gives one mesh; several give a `Group` holding one mesh per primitive,
named from the mesh's name with `_1`, `_2` added (loader:4041-4080) · confirmed. A body with four slots arrives as four
SkinnedMeshes sharing one Skeleton: C-B's "one per material per soldier" (r3:825).

**AX-F20. Names.** Node, bone and mesh names pass `sanitizeNodeName`, which turns spaces into underscores and removes
`[ ] . : /` (loader:3838-3852; PB:3-4, 178-188), with `_1` added on a clash; the original stays in `userData.name`
(loader:4509-4510). Material names are kept as written (loader:3817) · confirmed. So `hand.L` arrives as `handL`, while
a material `unit_dark.001` stays `unit_dark.001` (inferred).

**AX-F21. Empty nodes, extras, fallbacks.** A node with no mesh, camera or light becomes an `Object3D` (loader:4493), so
a socket arrives as a named Object3D; object extras go to `userData` (2413-2423). A primitive with no material gets a
shared default `MeshStandardMaterial` (2370-2386); missing normals give a flat-shaded clone; `COLOR_0` turns on vertex
colours (3579-3640) · confirmed. Each fallback hides a defect on screen, so the checks look for it in the file.

**AX-F22. Backends.** The loader has no WebGPU-specific code; official examples load glTF and a skinned GLB under
`WebGPURenderer` (sources 10, 11) · confirmed. A skinned GLB with `forceWebGL`: no example I read shows it.

**AX-F23. Draw calls.** A separate mesh draws on its own: C-B merges a soldier's parts "into one geometry per material"
(r3:825), legacy's merging cut the village from 398 to 202 draw calls (r3:303), and `InstancedMesh` "will help you to
reduce the number of draw calls" (r3:452-455) · inferred from these; the renderer's own draw loop was not read.

## 4. The rules

### 4.1 The file and the export
**AX-R1.** One model, one `.glb` file, written by Blender's built-in glTF 2.0 exporter. [from a draft: 0005:1803-1804,
SP:537-539; from a source: rst:881-885]

**AX-R2.** Export with AX-F9's "Rule" column. In a Blender script (the MCP route) that is one call. [the engine's
convention (proposed); the option names from a source, src `__init__.py`; `filepath`: from memory, unverified]

```python
bpy.ops.export_scene.gltf(filepath=PATH, export_format='GLB', use_selection=True, export_yup=True,
    export_apply=True, export_texcoords=True, export_normals=True, export_tangents=False,
    export_materials='EXPORT', export_image_format='NONE', export_vertex_color='NONE', export_skins=True,
    export_influence_nb=4, export_all_influences=False, export_rest_position_armature=True,
    export_leaf_bone=False, export_animations=False, export_morph=False, export_extras=False,
    export_cameras=False, export_lights=False, export_draco_mesh_compression_enable=False,
    export_meshopt_compression_enable=False)
```

Select every object of the one model first (mesh, armature, sockets): what `use_selection` takes along was not read.

**AX-R3.** Before export, apply rotation and scale on every object of the model; Apply Modifiers then bakes the other
modifiers, never the Armature (AX-F9). [the engine's convention (proposed), as G:14 asks; AX-F7 is why a skin needs it]

### 4.2 Scale, axes and origin
**AX-R4.** One unit is one metre; build at real size. The soldier's 1.80 m is a placeholder: "hang the soldier on one
root so it scales with one number." (Mustafa, 2026-11-17, go:44). [from a draft: `docs/EDITOR_V2.md`:63-65; from a
source: spec:737]

**AX-R5.** A soldier: +Y up, front +Z, his left +X; in Blender, Z up, front -Y, his left +X. His origin is on the
ground between his feet, and every node of his hangs under `root`. [axes from a source: spec:730-732, src
`nodes.py`:611-616, and the briefs agree, S:11 and S:73; the origin: the engine's convention (proposed), plan:51-52]

**AX-R6.** A gun: +Y up, +Z to the muzzle, +X to the gun's left, origin at `socket_grip`; in Blender, Z up, muzzle -Y.
[the engine's convention (proposed), plan:52-53, as G:11 and G:14 write it; the rifle file matches (go:48); +X left
agrees with spec:732]

### 4.3 Names
**AX-R7.** Every name in a file (objects, meshes, materials, bones, sockets) uses lower-case letters, digits and
underscores only: no spaces, dots, colons, slashes or brackets, so no Blender ".001" ending. [the engine's convention
(proposed), plan:37; from a source: AX-F20; that Blender adds ".001" to a duplicated name: from memory, unverified]

**AX-R8.** A material's name is its slot name, and its prefix is its class: `faction_` (takes the faction's colour;
the faction's data holds one colour per such area), `unit_`, `emissive_`, `structure_`, `environment_`. [the engine's
convention (proposed), plan:37-41; the drafts give the classes only "for example" (0005:1761-1762) and differ on the
fifth one (point 3)]

**AX-R9.** The Conscript's four areas are `faction_coat`, `faction_top`, `faction_trim` and `unit_dark`, for the
brief's Coat, Top, Trim and Dark (S:20-25), whose table varies the first three by faction and keeps Dark the same. "No
skin colour on this first stand-in: hands and face take the Dark material, so four materials hold, the rifle
included." (Mustafa, 2026-11-17, go:43). A gun's one material is `unit_dark` too (G:12, A:18). [the engine's
convention (proposed), plan:37-39]

**AX-R10.** The Conscript's 14 bones: `root`, `hips`, `chest`, `head`; `upper_arm_l`, `forearm_l`, `hand_l` and the
same three with `_r`; `leg_l`, `foot_l`, `leg_r`, `foot_r`. Parents: root > hips > chest > head; chest > upper arm >
forearm > hand; hips > leg > foot; the leg is one bone from hip to ankle (S:33). The list belongs to the model's
entry; a later model brings its own. `root` is a bone, the armature's top one, not the armature object and not an
empty (the brief's "one empty at the origin", S:73, becomes this bone). Vertex groups carry the bone names exactly.
[the engine's convention (proposed), plan:42-46, after the brief's bone column (S:29-50); groups map to bones by
name: from a source, AX-F12]

**AX-R11.** A gun's sockets are empty nodes (Blender empties parented to the gun) named `socket_grip`,
`socket_support`, `socket_muzzle`, `socket_sight` and `socket_eject` (Mustafa's names, go:51); `socket_grip` is the
gun's origin. The right hand holds `socket_grip` and the left goes to `socket_support`; where a pose disagrees, "the
sockets win" (Mustafa, 2026-11-17, go:49). The muzzle socket is where flash and tracer start; the simulation never
reads it (0005:1550-1553; go:50). [empties: G:35; a socket loads as a named Object3D: AX-F21; the rest: the engine's
convention (proposed), plan:47-50] Open: which sockets each gun has (point 2), and which local axis is a socket's
direction (G:40: the eject socket points "right and 20 degrees up"). Proposed: the gun's entry lists its sockets, and
a socket points along its local +Z.

**AX-R12.** A node whose name starts with `marking_` is a part the game may hide, using one of the model's own
materials. A marking differs per faction in shape and in the area whose colour it takes (S:57), so each faction's
marking is its own node (inferred). A shown marking is a mesh of its own and adds draw calls (AX-F23); how it follows
its bone was not read. [the engine's convention (proposed), plan:54-55; "off at game size and on in close views", A:13]

### 4.4 The caps
**AX-R13.** A soldier as drawn, his gun included, uses at most 4 materials per pass, and the draft's mark counts draw
calls (0005:1778-1781, 2311): a gun from its own file adds draw calls whatever its material is called (point 1). A
very large unit: parts × materials at most 25 per pass (0005:1784-1786). Triangles: no cap yet; about 5.2k per
soldier as drawn is a planning ceiling (0005:1787-1792), and the rifle alone has 2,596 (go:48). [from a draft]

### 4.5 The skin
**AX-R14.** A soldier is one mesh object: the parts are joined before export, the mesh is parented to one armature,
and each part's vertices are weighted 1.0 to the bone the brief names for it (a rigid skin) until someone paints
weights. The brief's "one object per part" (S:73) is the modelling form: exported as it is, each part would be a node
and a mesh of its own. [from a draft: 0005:1771-1772, r3:825; joining: the engine's convention (proposed)]

**AX-R15.** Every vertex is weighted to one to four bones, the weights summing to 1; an unweighted vertex makes the
exporter add a neutral bone. [from a source: spec:2009, 2044, 2064, 2074; AX-F12; AX-F17]

**AX-R16.** The armature's rest position is the bind pose (Use Rest Position Armature on, rst:1159-1160); which pose
that is belongs to the model's author. A gun has no armature and no skin. [the rest position: from a source; the gun:
the engine's convention (proposed), after A:17, "The rifle is a model of its own."]

### 4.6 What stays out of a file
**AX-R17.** No images or textures, vertex colours, animations, shape keys, cameras, lights, custom properties,
compression or LOD levels; no baked lighting, ink or toon shading; no collider, hit shape, shot origin or navigation
data. [from a draft: 0005:1748-1750, SP:390-391, 0005:1715, 0005:1550-1553; "No textures" is also the brief's (S:69);
no animations: the engine's convention (proposed), since soldiers are animated in code (0003:306, 0003:797)]

## 5. The checks a file passes (OQ-0005-AS3)

### 5.1 Nine marks
The plan's nine marks (plan:59-70), each with a defect a test plants and must see fail. At change-set M they join ADR
0005 as PM-0005-47 onward (plan:21); until a real export exists they are written and not run (plan:71). Two kinds of
small code run them: **J**, a script over the GLB that reads the header and the first chunk (AX-F8) and, for weights
only, the binary chunk; **L**, the game's loader in a test on both backends (AX-F15 to AX-F22). The Khronos glTF
Validator might do part of J; it was not read.

| # | Mark (plan:62-70) | Planted defect | Run by | Not confirmed |
|---|---|---|---|---|
| 1 | GLB, glTF 2, no required extension | `extensionsRequired: ["KHR_draco_mesh_compression"]`; an unknown name there | J: magic, version 2, JSON chunk first, `asset.version`, no `extensionsRequired` (spec:624, 3644, 3703-3714, 3773). L throws for Draco without a decoder but only warns on an unknown name (AX-F16) | none |
| 2 | every primitive has a material named in the slot table | a primitive with no material; a material `unit_dark.001` | J: `primitives[].material` set, `materials[].name` in the table. L keeps the names (loader:3817) but draws the default material silently (2370-2386) | whether Blender ever writes a primitive with no material |
| 3 | bone names equal the model's list; no extra joint | one unweighted vertex (the neutral bone); a leaf bone `_leaf` | J: `skins[0].joints` mapped to `nodes[].name`; L: bones by `userData.name` (loader:4509) | the neutral bone's name; whether the armature object becomes a joint |
| 4 | one joint and weight set; every vertex weighted | Include All Bone Influences on and a vertex with five weights (`JOINTS_1` appears) | J: no `JOINTS_1` or `WEIGHTS_1`; weights from the binary chunk sum to 1. L alone cannot: it normalises weights at load (loader:4009) | the exporter's cut and renormalising (AX-F12) |
| 5 | normals; no image, animation, morph target, camera or light | the defaults on a rig with an action (Animations and Shape Keys are on by default) | J: `NORMAL` in every primitive; no `images`, `animations`, `cameras`, `targets` or `KHR_lights_punctual` | none |
| 6 | a 1 m test cube is 1.0 m along +Y and faces +Z | the cube built at 100 units; +Y Up off | J: `POSITION` min and max (spec:1689), node transforms identity; L: the bounding box | a cube has no front: the test object needs a marker on its +Z side; whether Blender's Unit Scale changes the export |
| 7 | a gun's sockets by name; `socket_grip` at the origin, `socket_muzzle` on +Z | `socket_muzzle.001`; a gun exported muzzle to -Z | L: each socket by name and position (loader:4493); J: `nodes[].name`, `translation` | the rifle has four sockets (point 2); the socket axis (AX-R11) |
| 8 | counts inside the two derived caps | a fifth material; a soldier with gun above 4 draws per pass | J counts distinct materials; draws need the renderer: PM-0005-36's shown minus hidden (0005:2311) | what the mark counts (point 1) |
| 9 | the same result on WebGPU and with `forceWebGL` | the plan names none; PM-0005-42's "a backend path that drops the slot's colour" fits (0005:2318) | L run twice in a browser; headless runs give structure and counts, never frame rates as hardware results | a skinned GLB with `forceWebGL` (AX-F22) |

The rifle file itself was not seen: its names, extensions and origin are unknown until change-set M asks for it
(go:48).

### 5.2 Where a source or a draft speaks against the plan's proposals
**Point 1, the draw cap.** The plan (plan:38-39) and the briefs (G:12, A:18) keep the soldier with his gun at 4
materials by naming the gun's material `unit_dark`, and Mustafa's words agree for materials (go:43). The draft,
though, derives the cap from 4 draw calls per pass, and PM-0005-36 counts draw calls (0005:1778-1781, 2311). A gun
from its own file is its own mesh, whatever its material is called (AX-F19, AX-F23), and the rifle has "2 meshes"
(go:48): soldier 4 plus rifle 2 is 6 draws per pass, 12 over the main and shadow passes, above PM-0005-36's 8. Mark 8
(plan:69) counts materials and would pass what PM-0005-36 fails. Ways out: part 6.2.

**Point 2, five sockets.** Mark 7 asks for "a gun's five sockets" (plan:68), and the plan offers the rifle as the
first file (plan:71-72). The rifle has "four sockets" (go:48): "grip, support, muzzle and sight" (S:52); `socket_eject`
is the SMG's new one (go:51). As written, the first file fails mark 7. Proposed: mark 7 checks the sockets the gun's
entry lists, or the rifle gains an eject socket (its author decides).

**Point 3, the fifth class.** The plan's prefixes include `emissive_` (plan:40-41), from ADR 0005's example list
(0005:1761-1762); ADR 0003's list has "visual effects" in that place (0003:247-248). Both lists are examples, so the
prefixes stay a proposal, and the class table (content data, AP-3, 0005:1578-1585) should settle which list holds.

Smaller points, not against a source: mark 6's cube has no front; mark 4's "per vertex" is a primitive's attribute set
in the file (spec:2009); the plan should say that `root` is a bone (AX-R10).

## 6. Materials to slot classes, and colour (OQ-0005-AS4)

### 6.1 The convention for the first models
- One plain material per colour area, named by its class prefix (AX-R8, AX-R9), with no renaming step. The names
  survive by the sources: the exporter writes material names as they are (AX-F10) and makes one primitive per used
  material (AX-F11), and the loader keeps the name (AX-F20). That answers, by the source and not yet by an export, the
  point the draft marks "not researched": "whether the glTF export keeps material names" (0005:1885-1886).
- The engine finds a slot by its material's name, never by a mesh's name, since mesh names get `_1`, `_2` (AX-F19)
  (inferred). LOOK_V2 §8's automated renaming step is not needed for models named this way (inferred); whether it
  exists later is OQ-0005-AP1's.
- A `faction_` material's colour in Blender is a placeholder; at load the faction's data gives it (0005:1764-1765,
  1582-1583).

### 6.2 The counts against the cap: the Conscript with his gun

| What is drawn | Distinct materials | Draw calls per pass |
|---|---|---|
| the body, four areas (AX-F11, AX-F19) | 4 | 4 |
| with the rifle: `unit_dark`, "2 meshes" (go:48) | 4 | 6 |
| with the SMG: `unit_dark` (G:12), its mesh count not given | 4 | 4 + its meshes |
| each shown `marking_` node (AX-R12) | 4 | + 1 |

The draft allows 4 per pass, 8 over the main and shadow passes (0005:1778-1781, 2311). Ways to 4 draws per pass,
none researched (the engine proposes; an ADR line that changes needs Mustafa's go):
- (a) merge each gun's meshes into one at load: 5 draws, still above 4;
- (b) at load, add the gun's geometry to the body's `unit_dark` geometry, weighted 1.0 to `hand_r`: 4 draws; one
  merged geometry per body and gun pair, shared by every soldier holding that gun and swapped when he changes guns;
  its memory and swap cost are not measured;
- (c) count the cap in distinct materials and allow more draws per soldier: this changes ADR 0003's cap of 4 draw
  calls per soldier per pass (r3:1044-1045; 0005:1779-1780) and PM-0005-36.
A per-object hull outline, if the look takes one, doubles each count (r3:1021; inferred).

### 6.3 More than four areas: routes left open
- A vertex-colour mask: colour carried per vertex inside one material per class; the exporter can write vertex
  colours (rst:1084-1099) and the loader turns them on (AX-F21). How ADR 0003's toon material reads a mask: not
  researched.
- One texture: needs images exported (rst:1115-1119) and UVs; the same open question, and faction colour through a
  texture besides.
Neither is needed for the Conscript of the briefs: four areas, four materials.

## 7. What stays open

### 7.1 Open points

| Point | Who decides |
|---|---|
| What mark 8 and PM-0005-36 count, and route (a), (b) or (c) of 6.2 (point 1) | the engine proposes (main session; engine-architect reviews); Mustafa for any ADR line |
| Which sockets each gun has; an eject socket on the rifle (point 2) | the gun's author (the Art lead), with the engine |
| The fifth class (point 3); the class table itself (AP-3) | the engine, reviewed with ADR 0003's material |
| A socket's direction axis; `marking_` names after the prefix | the engine |
| The bind pose; the soldier's vertex and bone counts (AS6) | the model's author |
| Mustafa's Blender version against add-on 5.4.12 | this research, before the round trip |
| The not-read points of 7.2 | the round trip (7.3) |
| AS1, AS2, AS5, AP1 | the rest of the asset research (SP:529-545) |

### 7.2 Not read (and what for)
- Blender's manual (docs.blender.org): not on the allow-list; the exporter's repository documentation was read
  instead (the settings).
- The rendered glTF specification (registry.khronos.org): not on the allow-list; its source on main was read. It
  changed between my two reads (205,509 bytes on 2026-11-14, 205,402 on 2026-11-17; lines after 2180 moved by 4, and
  the GLB size header's wording changed), so a pinned release would be steadier.
- The exporter: where it cuts influences and whether it renormalises; the neutral bone's name; whether the armature
  object becomes a node or a joint; how an object parented to a bone is written; what `use_selection` takes along;
  `filepath` (AX-R2, AX-R12, AX-R15; marks 3 and 4).
- Blender: Unit Scale's effect on export; the ".001" ending; whether its mirror tools read `_l` and `_r` [from memory:
  they do; unverified] (AX-R7, AX-R10; mark 6).
- three.js: the renderer's draw loop (AX-F23, cited through r3); `normalizeSkinWeights` on a vertex with no weight
  (mark 4); the uniform buffer limit per backend (14 bones need 14 × 64 = 896 bytes on SK:61-65's path; the limit's
  value was not read).
- The Khronos glTF Validator (a candidate for J).

### 7.3 What a real export must still prove (the round trip)
Export three test files with AX-R2's call, from Mustafa's Blender through the MCP or from a headless Blender [from
memory, unverified: `blender --background --python`]: a 1 m cube with a marker on its +Z side; a skinned test figure
with the Conscript's four slots and 14 bones; a gun with its sockets. Load each through the loader on WebGPU and with
`forceWebGL`, and check: names as written, one mesh per slot, the skin bound to one Skeleton, no `JOINTS_1`, 1.0 m
along +Y, the front and the muzzle on +Z; then plant each mark's defect and see it fail. Headless runs check structure
and counts only. **Nothing in this file was proven by an export.**

## 8. Sources

All official; 0 from shipped games. Each URL once, with the date read.
1. glTF 2.0 specification, source: https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/Specification.adoc
   · 2026-11-14 and 2026-11-17 (raw file).
2. https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/schema/extras.schema.json · 2026-11-14.
3. https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/schema/glTFChildOfRootProperty.schema.json ·
   2026-11-14 (a name "is not necessarily unique").
4. Blender's exporter, documentation:
   https://github.com/KhronosGroup/glTF-Blender-IO/blob/main/docs/blender_docs/scene_gltf2.rst · 2026-11-14.
5. Blender's exporter, source: https://github.com/KhronosGroup/glTF-Blender-IO/tree/main/addons/io_scene_gltf2
   (`__init__.py`; `blender/exp/`: `nodes.py`, `obj_data.py`, `material/material_utils.py`, `joints.py`, `skins.py`,
   `primitive_extract.py`) · 2026-11-14.
6. https://github.com/mrdoob/three.js/blob/dev/examples/jsm/loaders/GLTFLoader.js · 2026-11-14 (REVISION 188dev) and
   2026-11-17 (raw file).
7. https://github.com/mrdoob/three.js/blob/dev/src/animation/PropertyBinding.js · 2026-11-14.
8. https://github.com/mrdoob/three.js/blob/dev/src/nodes/accessors/Skinning.js · 2026-11-14.
9. https://threejs.org/docs/pages/GLTFLoader.html · 2026-11-14, through the fetch tool's summary.
10. https://github.com/mrdoob/three.js/blob/dev/examples/webgpu_loader_gltf.html · 2026-11-14.
11. https://github.com/mrdoob/three.js/blob/dev/examples/webgpu_skinning.html · 2026-11-14.

Repository, this branch, 2026-11-17: ADR 0003 and 0005, SPIKE_PLAN, EDITOR_V2, STATUS, `docs/research/r3-rendering.md`,
and the five checkpoint files named under "Abbreviations".
