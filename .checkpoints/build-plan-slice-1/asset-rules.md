# Asset rules for Slice 1: first reading (graphics-engineer, 2026-11-14)

The rules a model file must follow so that the v2 engine loads it and colours it per faction. This is a first reading
for the build plan, not the formal research unit ("the asset-pipeline research", its export and slot-naming part,
OQ-0005-AS3 and AS4), which runs after Mustafa's go on the plan and is verified like R3. Nothing here is a decision.
Where the drafts leave a choice open, it says so and names who decides: Mustafa and the Advisor for anything about the
look; the research unit and its verifier for the facts; the engine (the main session, engine-architect reviewing) for
its own conventions, as a proposal.

Abbreviations: **0003** = `docs/adr/0003-rendering.md`; **0005** = `docs/adr/0005-content-abilities-assets.md`;
**0006** = `docs/adr/0006-infrastructure-operations.md`; **r3** = `docs/research/r3-rendering.md`; **SP** =
`docs/SPIKE_PLAN.md`. Repository line numbers are at today's main. **glTF spec** = the glTF 2.0 specification source,
https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/Specification.adoc (line numbers of that file as
fetched 2026-11-14). **BIO-rst** = Blender's glTF exporter documentation,
https://github.com/KhronosGroup/glTF-Blender-IO/blob/main/docs/blender_docs/scene_gltf2.rst (fetched 2026-11-14).
**BIO-src** = the exporter's source, https://github.com/KhronosGroup/glTF-Blender-IO/tree/main/addons/io_scene_gltf2
(branch main: add-on version 5.4.12, for Blender 5.3.0 and newer, `__init__.py` lines 15-21). **GLTFLoader.js** =
https://github.com/mrdoob/three.js/blob/dev/examples/jsm/loaders/GLTFLoader.js (dev, REVISION '188dev'; every line
cited also exists in r186, line given where it differs).

---

## 1. What the drafts already fix (word for word)

**The format and the flow (LOOK_V2 §8, as the drafts take it).**
- `docs/LOOK_V2.md:66`: "`[DECIDED]` **v2 may use authored 3D models.** The legacy rule "no external assets" ends with
  the legacy engine."
- `docs/LOOK_V2.md:67-72` (`[DEFAULT]`): "1. a concept sketch; 2. a 3D model in Spline: AI-generated for drafts, or a
  clean model built by Claude; 3. a glTF/GLB export; 4. an automated step that compresses it, builds LODs and renames
  materials into the faction colour slots; 5. the engine applies the comic look."
- `docs/LOOK_V2.md:73`: "`[DEFAULT]` **Budgets per unit:** triangles, materials and textures. The game never runs
  Spline's own runtime."
- The route for the models is Blender: SP:537-539, Mustafa 2026-11-03 (LOOK-TEST-INPUTS): "yes, the asset research's
  export and slot-naming part runs before the look proof test. Route: Blender (Spline's skins and named slots are
  unproven)." SP:540-542, Mustafa 2026-11-03 (LOOK_V2-§8): "yes, keep it as it is, with Blender for the test models
  only. When the asset research's export part reports, §8 says "Spline or Blender"." (His addition of 2026-11-14 keeps
  the route and has Claude make the models through the Blender MCP, exported as GLB.)

**The loader, and no Spline runtime.** 0005:1803-1804, GPU rule 6: "**Only glTF/GLB, never Spline's runtime.** The
game loads models as glTF/GLB files through three.js's loader on both backends; it never loads Spline's own runtime
(LOOK_V2 §8)." Also AP-6, 0005:1616-1617: "the game loads only its output, glTF/GLB files, and never Spline's
runtime". Interface I-8, `docs/adr/README.md:350`: "Assets → renderer | glTF/GLB with named material slots, LOD
levels, per-unit budgets; no baked look | 0005 | 0003".

**An imported model carries no look.** 0005:1748-1755, GPU rule 1: "**An imported model carries no look; the engine
applies it.** A model brings geometry, its skin (joints and weights) where it moves, UVs and textures where it has
them, and named material slots. It brings no baked lighting and no ink. At load the engine gives each slot a shared
node toon material of that slot's material class (ADR 0003, part 5.a), one material per slot content and never one
per mesh. A slot's content is its class (ramp, colour gain, ink weight), its faction for faction colour slots, and its
own base and emissive colour, textures, side and alpha mode, [...] so an asset keeps its own colours and textures
inside a class, and how colour is authored is OQ-0005-AS4's, not a rule." Same in 0003:175-188 (part 5.a), which ends
"how colour is authored (material colour, vertex colour or an atlas) is ADR 0005's OQ-0005-AS4, not a rule here."

**Slots name a material class.** 0005:1761-1764, GPU rule 2: "**Material slots name a material class.** Every slot's
name maps to one class from a small fixed list (for example environment, structure, unit, faction colour, emissive;
the structure class gives buildings their heavier ink, LOOK_V2 §2, ADR 0003 parts 5.b and 5.h); the list and the names
are this ADR's content data (interface I-8 in `docs/adr/README.md`), consumed by ADR 0003." 0005:1768-1769: "Emissive
slots write the emissive output that drives the glow, so only they glow".

**The slot-class list: an example, not a decision.** Both drafts write it with "for example" (0005:1762 above, and
AP-3 at 0005:1579-1580: "the classes the GPU section's part 2 names (for example environment, structure, unit, faction
colour, emissive)"). ADR 0003's own list differs: 0003:247-248, part 5.h, "Each material class (environment,
structures, units, faction colours, visual effects) carries a colour gain and an ink weight held as tuning data"
("visual effects" where ADR 0005 says "emissive"). No draft gives a slot name.

**Faction colour slots (AP-3).** 0005:1578-1585: "the classes [...] and the slot names that map to each, are one
client-content table (CD-6), validated like all content and read by ADR 0003 (interface I-8). Adding a class is a
content change reviewed together with the ADR 0003 material it needs. A faction's colours reach its faction colour
slots from the faction's client definition (CD-13; OQ-0005-1, answered by the Advisor, 2026-11-03). The table says
which slots exist; how a class looks is ADR 0003's and Mustafa's (LOOK_V2)." And GPU rule 2, 0005:1764-1767: "Faction
colour slots take the faction's colour at load, as one shared material per slot and faction, or as per-instance colour
where a mesh is instanced (`instanceColor` on `InstancedMesh` [...]; `BatchedMesh` also carries per-instance colour
[...])". ADR 0003 part 6, 0003:315-317: "Faction colour comes from the faction colour slots (ADR 0005, I-8): one shared
material per slot content, the faction included (5.a), or per-instance colour where meshes are instanced". The choice
follows the skinning option, 0005:1852-1853: "C-B uses shared materials per faction; an instanced candidate that wins
its mark uses per-instance colour."

**Soldiers are skinned; very large units are part groups.** 0005:1771-1776, GPU rule 3, whole: "3. **Soldiers are
skinned; very large units are part groups.** A soldier model is one skinned mesh with skin indices and weights as
attributes ("requires skin indices and weights as buffer attributes in its geometry": r3, Findings Q4, R3-CHAR-F1,
lines 436–439) and any weights the artist paints (option C-B, R3-CHAR-REC1, lines 1032–1048). A very large unit's
parts are separate meshes under one group, each part named to match the sim's vehicle-part records (option V-A,
R3-CHAR-REC4, lines 1077–1092; the records are ADR 0004's). *Need:* VISION_V2 §6.4, §6.9; LOOK_V2 §3, §5."
ADR 0003 part 6, 0003:302-305: "One skinned mesh per soldier (option C-B) on both backends, with any skin weights,
since v2 soldiers are authored models [...]. Instanced per-instance skinning (C-E on both backends, C-D as the WebGPU
variant) is measured against it and adopted only if it cuts total frame time by at least 20 % on both reference
machines". AP-1, 0005:1550-1554: "no collider, hit shape, shot origin, navigation data or vehicle-part layout is ever
read from a model. [...] a weapon model's sockets are cosmetic (CC9's note); a very large unit's parts take their
transforms from the sim's vehicle-part records, which the asset's part names match".

**The two derived per-unit caps (GPU rule 4), with their arithmetic.** 0005:1777-1786:
- "*A soldier as drawn* (body, headgear and held weapon) uses at most 4 materials per pass: option C-B draws "one per
  material per soldier" (r3, Options Q4, line 825) and ADR 0003 caps a soldier at 4 draw calls per pass before the ink
  (R3-CHAR-REC1, lines 1044–1045). Each material carries one class ramp (part 1), so at most 4 surface kinds (skin,
  cloth, faction colour, metal, emissive) shade differently on one soldier [...]; the look proof test's soldier is built
  at this cap."
- "*A very large unit* draws at most 25 parts × materials per pass: ADR 0003 allows the four very large units 200 draw
  calls over the shadow and the main pass (R3-CHAR-REC4, lines 1088–1090), and 200 ÷ 4 units ÷ 2 passes = 25. R3's own
  test load, 20 parts each with one material, fits (4 × 20 × 2 = 160, lines 1089–1090)."
- The triangle figure is a ceiling, not a budget, 0005:1787-1792: "At Low, 500k ÷ (48 soldiers × 2 passes) ≈ 5.2k
  triangles per soldier as drawn, if soldiers had the whole row before the world and the very large units take their
  share [...]: a planning input for OQ-0005-AS5 and AS6." (It assumes the triangle row counts the shadow pass:
  "assumed: how three.js counts the shadow pass's triangles is not researched", 0005:1788-1789.)
- Frame totals, 0005:1793-1794: "at Low, ≤ 800 draw calls (without a per-object outline pass; about 1,100 with one, r3
  line 1133), ≤ 500k triangles, ≤ 512 MB of GPU memory; at High, ≤ 1,500, ≤ 1.5M, ≤ 1 GB."

**The 4-materials-per-pass cap is stated in a draft**: GPU rule 4 above, and AP-5 at 0005:1601-1604 puts it in the
budget file: "today the GPU section's two derived caps (a soldier as drawn: at most 4 materials per pass, from ADR
0003's 4 draw calls per soldier per pass, r3, lines 1044–1045; a very large unit: at most 25 parts × materials per
pass, 200 draw calls ÷ 4 units ÷ 2 passes, r3, lines 1088–1090), the rest once OQ-0005-AS5 sets them". Its pass mark
bites on a fifth slot: PM-0005-36 (0005:2311), "Bites: a soldier with 5 material slots (10 draws over 2 passes, above
8)".

**The per-unit budgets left open.** OQ-0005-AS5, 0005:1890-1895: "**What are the per-unit budgets beyond the two
derived caps?** Why it matters: LOOK_V2 §8 asks for triangles, materials and textures per unit; only the soldier's 4
materials per pass and a very large unit's 25 draws per pass follow from the research (part 4). [...] Decider: the
asset-pipeline research, then the render budget bench with real models." And OQ-0005-AS6, 0005:1896-1900: "**How many
vertices and bones does the authored soldier have?** Why it matters: the triangle row and the bone upload in the
animation row (≤ 1.0 ms at Low) depend on them. [...] Decider: the character model's author (LOOK_V2 §3, §8), then the
render budget bench."

**LOD levels.** 0005:1796-1801, GPU rule 5: "**LOD levels are optional data; switching is the engine's.** A model may
carry mesh LOD levels. [...] Animation detail is never in the asset: the engine's budget LOD decides it (R3-CHAR-REC2).
How LOD levels are built is open (OQ-0005-AS2)." For the test models there is no LOD step: SP:390-391, "plain glTF/GLB
exports with named material slots and no compression or LOD step"; 0005:1715, "(plain glTF/GLB exports, no compression
or LOD step: ASSET-RESEARCH (a))"; 0005:1813, "the test decides nothing about compression or LODs".

**The asset list, by id, and its refusals (AP-2).** 0005:1566-1572: "One asset list in the content set (client
content, CD-6) holds for each asset: its id; its file name with its hash; its material slots, each with its class
(AP-3); its LOD levels, if it has any (the GPU section's part 5); and its counts of triangles, materials and textures,
which the budget check reads (AP-5). The loader's whole-set checks (CD-2) refuse a reference to an asset the list does
not hold, a slot whose class is not in the class list, and an id of a presented kind with no client entry (OQ-0005-1's
rule), naming the file and the field; nothing falls back to a default model."

**The look proof test's models, and Mustafa's bar.** 0003:474-479, part 11: "a few plain glTF/GLB exports (one soldier
in LOOK_V2 §3's proportions with its faction colour slots, one weapon, LOOK_V2 §2's two houses, one very large unit of
at least 20 parts, a few props)". SP:390-396 gives the same list. Mustafa's bar, SP:619-622 (OQ-SPIKE-1, answered
2026-11-03): "yes, the asset research's export and slot-naming part runs before the look proof test. Route: Blender
(Spline's skins and named slots are unproven). I make the test models with AI help in Blender; blockouts with a skin,
named colour slots and LOOK_V2's proportions are enough." Quoted again at 0003:482-484 and 0005:1813-1815.

**The scale: not in the decision drafts yet.** No ADR (0001-0006) states it; `docs/STATUS.md:76` lists it "For the
decision drafts' next revision: [...] the scale, "1 unit = 1 metre"". It is a `[DECIDED]` principle of the editor
spec, written by Mustafa and the Advisor, `docs/EDITOR_V2.md:63-65`: "all the basic primitives [...], with one
consistent scale across the engine. Principles: 1 unit = 1 metre; textures tile in world space (they repeat per metre
and never stretch)". `docs/research/GLOSSARY.md` has no entry for it.

**Not fixed by any draft:** the slot names; the model's origin; its facing direction; the world axes; the joint list
(names, hierarchy, bind pose); weapon socket names; whether headgear is part of the body model or its own asset; the
very large unit's part names (they follow ADR 0004's vehicle-part records, not yet written).

---

## 2. What official documentation says, per export setting

Sureness: **sure** = read verbatim in the official text or source today; **inferred** = follows from text read, not
stated; **[unsure]** and **[from memory, unverified]** as marked.

**The format itself (glTF spec).**
- Axes: "glTF uses a right-handed coordinate system." (spec:730) "glTF defines +Y as up; the front side of a glTF
  asset faces +Z, the left side of a glTF asset faces +X." (spec:732). Sure.
- Units: "The units for all linear distances are meters." (spec:737); "After applying the node's global transform,
  mesh vertex position values are meters." (spec:2180). Sure.
- Names: "Any top-level glTF object **MAY** have a `name` string property for this purpose. These property values are
  not guaranteed to be unique as they are intended to contain values created when the asset was authored." (spec:725);
  the schema adds "This is not necessarily unique"
  (https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/schema/glTFChildOfRootProperty.schema.json). Sure.
- Extras: "Application-specific data." and "`extras` **SHOULD** be a JSON object rather than a primitive value for best
  portability." (https://github.com/KhronosGroup/glTF/blob/main/specification/2.0/schema/extras.schema.json). Sure.
- Materials per primitive: "A _mesh_ is defined as an array of _primitives_." (spec:1578); a primitive holds
  attributes, topology, connectivity "and an optional material" (spec:1580); "If the `material` property is not
  defined, then a default material is used." (spec:1584); note: "Splitting one mesh into several primitives can be
  useful [...] to assign different materials to different parts of the mesh." (spec:1589). Sure: one material per
  primitive is the format's shape.
- Skin influences: "Each attribute pair provides up to four joint indices and their weights per vertex." (spec:2009);
  more than four use `JOINTS_1`/`WEIGHTS_1` "and so on" (spec:2014, implementation note); "Client implementations
  **MAY** support only a single set of up to four weights and joints, however not supporting all weight and joint sets
  present in the file may have an impact on the asset's appearance." (spec:2074); clients "**SHOULD** support at least
  [...] `JOINTS_0` with `WEIGHTS_0`" (spec:1685). Sure: **the core specification sets no 4-influence limit**; it lets a
  client read only 4, and three.js reads only 4 (below).
- Weights: unorm weights "**MUST** be exactly 1.0" in sum after dequantization (spec:2044); float weights "**SHOULD**
  be as close as reasonably possible to 1.0" (spec:2064). Sure.
- Skins: "Each skin contains a **REQUIRED** `joints` property [...] and an **OPTIONAL** `inverseBindMatrices`"
  (spec:2084); "The joint hierarchy used for controlling skinned mesh pose is simply the node hierarchy" (spec:2129);
  "A skinned mesh primitive does not use the global transform of the node that instantiates the mesh." (spec:2242).
  Sure.
- GLB: "a container format, _Binary glTF_ [...] enables a glTF asset, including JSON, buffers, and images, to be stored
  in a single binary blob." (spec:3681); extension `.glb` (spec:3688); media type `model/gltf-binary` (spec:3690); the
  file "may be at most 2^32^-4 bytes" (spec:994). Sure.

**Blender's exporter: each option (BIO-rst for the meaning, BIO-src `__init__.py` for the default and the script
name).**

| Setting (UI name; script name) | What the documentation says | Default (source) | Sure? |
|---|---|---|---|
| Format (`export_format`) | "glTF Binary (`.glb`): This produces a single `.glb` file with all mesh data, image textures, and related information packed into a single binary file." (BIO-rst:881-885) | `'GLB'`, the first item, `default=0` (`__init__.py`:335-342, 147-151: "Most efficient and portable") | sure |
| +Y Up (`export_yup`) | "Export using glTF convention, +Y up." (BIO-rst:1037-1038); the swizzle is `Vector((loc[0], loc[2], -loc[1]))` (BIO-src `blender/exp/nodes.py`:611-616), so Blender +Z becomes glTF +Y and Blender −Y becomes glTF +Z, the glTF front | True (728-732) | sure |
| Apply Modifiers (`export_apply`) | "Export objects using the evaluated mesh, meaning the resulting mesh after all Modifiers have been calculated." (BIO-rst:1061-1063); source: "Apply modifiers (excluding Armatures) to mesh objects - WARNING: prevents exporting shape keys" | False (734-739) | sure |
| UVs (`export_texcoords`) | "Export UVs (texture coordinates) with meshes." (BIO-rst:1064-1065) | True (456-460) | sure |
| Normals (`export_normals`) | "Export vertex normals with meshes." (BIO-rst:1066-1067) | True (462-466) | sure |
| Tangents (`export_tangents`) | "Export vertex tangents with meshes." (BIO-rst:1068-1069) | False (548-552) | sure |
| Use Vertex Color (`export_vertex_color`) | Material: "Export vertex color when used in material node tree as Base Color multiplier." ... None: "Do not export vertex color." (BIO-rst:1084-1095) | `'MATERIAL'` (585-598) | sure |
| Materials (`export_materials`) | Export: "Export full materials, including all textures and shaders from node tree." Placeholder: "Export only the material placeholder, without any texture or shader. Primitives are not merged, so material slot information is kept." No Export: "Does not export materials. Primitives are merged, losing material slot information." (BIO-rst:1105-1114) | `'EXPORT'` (554-570) | sure |
| Images (`export_image_format`) | "If None is chosen, materials are exported without textures." (BIO-rst:1115-1119); glTF needs PNG or JPEG images (BIO-rst:116-119) | `'AUTO'` (361-378) | sure |
| Draco (`export_draco_mesh_compression_enable`) | "Compress meshes using Google Draco." (BIO-rst:1193-1196) | False (494-498) | sure |
| Meshopt (`export_meshopt_compression_enable`) | "Compress using Meshoptimizer." (BIO-rst:1212-1218) | False (474-478) | sure |
| Skinning (`export_skins`) | "Export skinning data" (BIO-rst:1173) | True (975-979) | sure |
| Bone Influences (`export_influence_nb`) | "How many joint vertex influences will be exported. Models may appear incorrectly in many viewers with value different to 4 or 8." (BIO-rst:1175-1177) | 4, min 1 (981-986) | sure |
| Include All Bone Influences (`export_all_influences`) | "Export all joint vertex influences. Models may appear incorrectly in many viewers." (BIO-rst:1179-1180) | False (988-992) | sure |
| Use Rest Position Armature (`export_rest_position_armature`) | "Export Armatures using rest position as joint rest pose. When Off, the current frame pose is used as rest pose." (BIO-rst:1159-1160) | True (957-964) | sure |
| Export Deformation Bones Only (`export_def_bones`) | "Export Deformation bones only, not other bones." (BIO-rst:1161-1163) | False (816-820) | sure |
| Animations (`export_animations`, `export_animation_mode`) | an animation changes "the transforms of objects or pose bones, or the values of shape keys" (BIO-rst:732); in the default Actions mode "Actions which are **not** associated with an object in one of these ways are **not exported**" (BIO-rst:792-795) | True; `'ACTIONS'` (747-751, 789-805) | sure |
| Shape Keys (`export_morph`) | "Export shape keys (morph targets)." (BIO-rst:1138) | True (994-998) | sure |
| Custom Properties (`export_extras`) | "Custom properties are always imported, and will be exported from most objects if the Include → Custom Properties option is selected before export. These are stored in the `extras` field on the corresponding object in the glTF file." (BIO-rst:721-723) | False (722-726) | sure |
| Cameras, Punctual Lights (`export_cameras`, `export_lights`) | "Export cameras." / "Export directional, point, and spot lights." (BIO-rst:1028-1031) | False, False (650-654, 1027-1032) | sure |
| Operator | `bl_idname = 'export_scene.gltf'` (`__init__.py`:1967), so a script calls `bpy.ops.export_scene.gltf(...)` | – | sure |

**How names are written (BIO-src, read in the source; the documentation page does not say).**
- Object → glTF node name: `new_name = blender_object.name` (`blender/exp/nodes.py`:486-488). Sure.
- Mesh data → glTF mesh name: `return blender_data.name` (`blender/exp/obj_data.py`:144-149). Sure.
- Material → glTF material name: `return blender_material.name` (`blender/exp/material/material_utils.py`:24-25);
  material extras only with Custom Properties on (18-21). Sure.
- Bone → joint node name: `name=blender_bone.name` (`blender/exp/joints.py`:102-103; leaf bones get "_leaf"). Skin
  name = the armature object's name (`blender/exp/skins.py`:153-154). Sure.

**A mesh with several material slots (BIO-src).** `blender/exp/primitive_extract.py`:428-444: with Materials "No
Export" all triangles go into one primitive ("Only for None. For placeholder and export, keep primitives"); otherwise
"Bucket by material index": one primitive per material slot that triangles use (a slot no triangle uses makes none).
Sure. With Export or Placeholder, each colour area becomes its own primitive, which three.js draws as its own mesh.

**Skin weights in the exporter (BIO-src `primitive_extract.py`).** Vertex groups map to joints by name (1650-1651;
a group with no bone of that name is skipped, 1668-1669); weights at or below 0.0001 are dropped (1648, 1662); a
vertex's weights are sorted largest first (1671); **a vertex weighted to no bone is assigned to a "neutral bone" the
exporter adds at the armature root** (1672-1675). Sure. Where the exporter cuts to "Bone Influences" and whether it
renormalises afterwards: not found in the files read [unsure].

**Geometry in the exporter (BIO-rst:46-53).** "quads and n-gons are automatically converted to triangles when
exporting to glTF. Discontinuous UVs and flat-shaded edges may result in moderately higher vertex counts in glTF
compared to Blender, as such vertices are separated for export. Likewise, curves and other non-mesh data are not
preserved, and must be converted to meshes prior to export." Sure.

**Materials in the exporter.** "The exporter supports Metal/Rough PBR (core glTF) and Shadeless
(`KHR_materials_unlit`) materials." (BIO-rst:112); base colour from the Principled BSDF's Base Color input, or its
default colour when unconnected (BIO-rst:125-127); "If any component of emissiveFactor is > 1.0,
`KHR_materials_emissive_strength` extension will be used." (BIO-rst:264); Backface Culling's inverse is glTF's
`DoubleSided` (BIO-rst:497-505); the alpha mode is read from the Alpha socket (BIO-rst:511-513). Sure.

**three.js's GLTFLoader.**
- Docs page https://threejs.org/docs/pages/GLTFLoader.html (read through the fetch tool's summary): the result holds
  `scene` ("The default scene."), `scenes`, `animations` ("An array of animation clips."), `cameras`, `asset`,
  `parser`, `userData`; `setDRACOLoader` is "Required for decoding assets compressed with the
  `KHR_draco_mesh_compression` extension.", `setMeshoptDecoder` "Required for decoding assets compressed with the
  `EXT_meshopt_compression` extension.", `setKTX2Loader` "Required for loading KTX2 compressed textures." The page
  says nothing about WebGPURenderer, names, extras, skinning or multi-material meshes. Sure for what it lists.
- Attributes (GLTFLoader.js:2330-2341; r186 2286-2287 for the skin pair): `COLOR_0: 'color'`, `WEIGHTS_0:
  'skinWeight'`, `JOINTS_0: 'skinIndex'`; any other attribute becomes `gltfAttributeName.toLowerCase()` (4952; r186
  4871), so `JOINTS_1`/`WEIGHTS_1` arrive as unused `joints_1`/`weights_1`. Sure for the code; that three.js skins
  with at most 4 influences is inferred, and confirmed on the node side by the skinning node, which reads
  `skinIndex.x..w` and `skinWeight.x..w` (https://github.com/mrdoob/three.js/blob/dev/src/nodes/accessors/Skinning.js,
  102-117).
- Skinned mesh (3992-4011; r186 3928): a primitive becomes `new SkinnedMesh( geometry, material )` when its mesh is
  skinned and has both skin attributes, otherwise `new Mesh` with the warning "Missing skinIndex or skinWeight
  attributes. Skinning disabled."; then `mesh.normalizeSkinWeights()` "normalize skin weights to fix malformed
  assets". Joint nodes become `new Bone()` (4479-4481); `loadSkin` builds `new Skeleton( bones, boneInverses )`
  (4133-4192); every SkinnedMesh under the skinned node is bound to that one skeleton, `mesh.bind( skeleton,
  _identityMatrix )` (4374-4378; r186 4297). Sure.
- Several materials (4041-4080; r186 3989): each primitive's mesh is named `parser.createUniqueName( meshDef.name ||
  ( 'mesh_' + meshIndex ) )`; one primitive returns that mesh; several return `new Group()` holding one mesh per
  primitive. So a Blender mesh with four material slots arrives as a Group of four meshes (for a soldier, four
  SkinnedMeshes sharing one Skeleton), named for example `soldier_body`, `soldier_body_1`, `soldier_body_2`,
  `soldier_body_3`. Sure.
- Names: `createUniqueName` runs `PropertyBinding.sanitizeNodeName` and adds `_1`, `_2` on a clash (3838-3852; r186
  3759); `sanitizeNodeName` "Replaces spaces with underscores and removes unsupported characters from node names", the
  removed characters being `[ ] . : /` (https://github.com/mrdoob/three.js/blob/dev/src/animation/PropertyBinding.js,
  3-4 and 178-188). A node keeps its original name in `userData.name` (4507-4510; r186 4428). A material keeps its
  name unchanged: `if ( materialDef.name ) material.name = materialDef.name;` (3817; r186 3736). So "hand.L" becomes
  the bone "handL" and "Bone.001" becomes "Bone001", while a material "faction_primary.001" stays as written. Sure for
  the code; the examples are inferred.
- Extras: `Object.assign( object.userData, gltfDef.extras )` for object extras, a warning for any other type
  (2413-2423; r186 2366), on nodes, meshes, materials and geometries. Sure.
- Fallbacks to watch: a primitive with no material gets a shared default `MeshStandardMaterial` (2370-2386); a geometry
  without normals gets a cloned material with `flatShading = true`, one with `COLOR_0` gets `vertexColors = true`
  (3579-3640); vertex colours under a non-linear working colour space only log a warning (4973-4975). Sure.
- WebGPU: the loader has no WebGPU-specific code (no match for "webgpu" or "NodeMaterial"); it builds
  `MeshStandardMaterial`/`MeshPhysicalMaterial`, which the engine replaces anyway (GPU rule 1). Official examples load
  glTF under `WebGPURenderer`: `examples/webgpu_loader_gltf.html` (`new GLTFLoader()`, `new THREE.WebGPURenderer`) and
  a skinned GLB in `examples/webgpu_skinning.html` (`models/gltf/Michelle.glb`, `AnimationMixer`). Sure; the WebGL 2
  fallback (`forceWebGL`) with a skinned GLB is not shown in what I read.
- Bones on the node path (dev): "Skeletons that fit within the uniform buffer limit use a uniform buffer, larger
  skeletons fall back to a bone texture." (Skinning.js:50-51; the size test is bones × 16 × 4 bytes against
  `builder.getUniformBufferLimit()`, 61-65). So no hard bone cap on dev; this answers part of R3-CHAR-OQ1 (r3:1344-1346)
  for dev only. The limit's value per backend was not read [unsure]; r186's file was not checked.

---

## 3. A draft of the short list (for the Art project's Blender brief)

Legend for the brackets (strip them when pasting if you like): draft = an ADR or the spike plan, file:line; docs = an
official page or source (section 2 has the URLs); proposal = no draft fixes it, and the bracket names who confirms.

1. Export with Blender's built-in glTF 2.0 exporter as "glTF Binary (.glb)": one .glb file per model. [draft: 0005:1803-1804; docs: BIO-rst:881-885]
2. Mesh options: "+Y Up" on, "Apply Modifiers" on, "UVs" on, "Normals" on, "Tangents" off. [docs: BIO-rst:1037-1038, 1061-1069; proposal (Apply Modifiers on, Tangents off): to be confirmed by the asset-pipeline research]
3. Material options: "Materials: Export" (never "No Export", which merges all colour areas into one), "Images: None", "Use Vertex Color: None". [docs: BIO-rst:1084-1119; BIO-src primitive_extract.py:428-444; proposal (Images, Vertex Color): to be confirmed by the asset-pipeline research, OQ-0005-AS4]
4. Skin options: "Skinning" on, "Bone Influences" 4, "Include All Bone Influences" off, "Use Rest Position Armature" on. [docs: BIO-rst:1159-1180; three.js reads only 4 influences: GLTFLoader.js:2339-2340, 4952]
5. Leave out what the engine does not use: "Animations" off, "Shape Keys" off, "Custom Properties" off, cameras and lights off. [proposal, to be confirmed by the engine: soldiers are animated in code, 0003:306, 0003:797]
6. The same in a Blender MCP script: bpy.ops.export_scene.gltf(export_format='GLB', export_yup=True, export_apply=True, export_texcoords=True, export_normals=True, export_tangents=False, export_materials='EXPORT', export_image_format='NONE', export_vertex_color='NONE', export_skins=True, export_influence_nb=4, export_all_influences=False, export_rest_position_armature=True, export_animations=False, export_morph=False, export_extras=False, export_cameras=False, export_lights=False, export_draco_mesh_compression_enable=False, export_meshopt_compression_enable=False). [docs: BIO-src __init__.py (names, for add-on 5.4.12); proposal as lines 2 to 5]
7. Scale: 1 Blender unit = 1 metre; build at real-world size. [design: EDITOR_V2.md:64 `[DECIDED]`; docs: glTF spec:737; the ADRs do not state it yet: STATUS.md:76]
8. Axes: the model stands up along Blender's Z axis and faces Blender's −Y; the exporter turns that into glTF's +Y up and +Z front. [docs: glTF spec:730-732; BIO-src nodes.py:611-616; proposal for the facing, to be confirmed by the engine]
9. Origin: on the ground under the model's centre (for a soldier, between the feet), with rotation and scale applied ("Apply > All Transforms") before rigging and export. [proposal, to be confirmed by the engine; docs: glTF spec:2242, a skinned mesh ignores its own object transform]
10. Names: letters, digits and underscores only: no spaces, dots, colons, slashes or brackets, and no ".001" endings. [docs: three.js PropertyBinding.js:178-188 strips . : / [ ] and turns spaces into _; proposal, to be confirmed by the engine]
11. One colour area is one material; the material's name is the slot name, and the export keeps it unchanged. [draft: 0005:1748-1750, 0005:1761-1764; docs: BIO-src material_utils.py:24-25, GLTFLoader.js:3817]
12. Slot names begin with their class: environment_, structure_, unit_, faction_ or emissive_ (for example unit_skin, unit_cloth, unit_metal, faction_primary). [proposal, to be confirmed by the engine's slot-class table, 0005:1578-1585; the drafts give the classes only "for example", 0005:1762]
13. Faction areas (faction_…) take each faction's colour at load, so their colour in Blender is only a placeholder. [draft: 0005:1764-1765, 0005:1582-1583]
14. Only emissive_ areas glow; give each its glow colour as the material's Emission. [draft: 0005:1768-1769; docs: BIO-rst:250-264]
15. A soldier is one mesh object for the whole body, parented to one armature; the held weapon is its own .glb with the same rules. [draft: 0005:1771-1774, 0003:794; proposal for the object names, to be confirmed by the engine]
16. A very large unit is one mesh object per part under one parent, each named after the simulation's part list (that list is not written yet). [draft: 0005:1774-1775]
17. Materials cap: the soldier as drawn (body, headgear and held weapon together) has at most 4 materials, so at most 4 colour areas. [draft: 0005:1778-1783, 0005:1601-1604]
18. Very large unit cap: parts × materials at most 25 (for example 20 parts with one material each). [draft: 0005:1784-1786]
19. Triangles: no fixed cap yet; keep the soldier as drawn near or under about 5,200 triangles (the drafts' planning ceiling; a quad counts as 2). [draft: 0005:1787-1792, a planning input, not a budget; docs: BIO-rst:48-49; proposal: OQ-0005-AS5 and AS6 set the real number]
20. Skin: yes, Slice 1's soldier is skinned: one skinned mesh, with any weights you paint. [draft: 0003:1397, 0005:1771-1774, 0003:302-303]
21. Every vertex weighted to at least one bone and to at most 4, its weights adding up to 1. [docs: glTF spec:2009, 2044, 2064, 2074; BIO-src primitive_extract.py:1672-1675, an unweighted vertex gets a stray extra joint]
22. Bones: exactly the names, parenting and rest pose of the engine's joint list (it comes before the stand-in is built), with no extra helper or IK bones in the file. [proposal, to be confirmed by the engine: no draft gives a joint list; docs: BIO-rst:1161-1163]
23. No baked lighting, no outlines or ink, no toon or custom shading: the engine applies the look. [draft: 0005:1748-1750; LOOK_V2.md:72]
24. No textures, vertex colours, compression (Draco, Meshopt) or LOD levels for now. [draft: SP:390-391, 0005:1715, 0005:1813; proposal for textures and vertex colours, to be confirmed by the asset-pipeline research, OQ-0005-AS4]
25. No model file goes into the repository until Mustafa answers whether models may be public (OQ-0006-18). [Mustafa, 2026-11-14, item 13; 0006:1105-1106]

---

## 4. Still open

Who answers the formal unit, and when: graphics-engineer, verified like R3, in its own daytime session (SP:535-536;
0005:1499-1500); its export and slot-naming part (AS3, AS4) runs before the look proof test (SP:537-539) and, by
Mustafa's addition of 2026-11-14, before the Art project's Blender brief is final. Where it sits after his go on the
plan is the build plan's call; SP:543-545's earlier recommendation was "right after the netcode proof test".

| Point | Id | Who decides | What the research unit must still do |
|---|---|---|---|
| Which checks an export passes before it enters the game | OQ-0005-AS3 | the research unit (0005:1874-1877) | Propose and plant-test the checks. Candidates from this reading: the file is GLB version 2; no `extensionsRequired` (no decoder is registered); every primitive has a named material whose name is in the slot table (no default material); `NORMAL` present; no `JOINTS_1`; no neutral or unlisted joint; joint names equal the engine's list; no images, animations or morph targets while the brief forbids them; a known-size test object measures its size in metres along +Y; counts within the caps; file under 25 MiB. Candidate tool: the Khronos glTF Validator [from memory, unverified: not read]. |
| How materials become slot classes, and how colour is carried | OQ-0005-AS4 | the research unit (0005:1881-1889); the class table itself is content data the engine proposes (AP-3); how a class looks is Mustafa's | Confirm by round trip that material names survive (the source says they do: section 2); weigh a manual naming convention against an automated renaming step; count materials for each colour route (material colour, vertex colours, an atlas) against the 4-material cap. |
| How many faction colour areas a soldier has, against the 4-material cap | (look) with OQ-0005-AS4 | Mustafa and the Advisor (look); the research gives the counts | Legacy has five palette roles (BaseGear, FactionPrimary, HighlightTrim, CoatAccent, PadColor: `docs/LEGACY_INVENTORY.md:134`, CC3) and ADR 0003 ports "the slot, role and palette schema" as the faction colour slots (0003:793), while the cap allows 4 materials for body, headgear and weapon together, so at most 4 of rule 4's surface kinds "(skin, cloth, faction colour, metal, emissive)" (0005:1780-1782). Arithmetic, not a look choice: with one material per colour area, if skin, cloth and metal each take one, one faction area is left; any area beyond four needs another colour route (a mask in vertex colours or a texture), which no source read here researches. |
| The per-unit triangle and texture budgets | OQ-0005-AS5 | the research unit, then the render budget bench (0005:1893-1894) | Split the frame totals across units, world and effects; until then 5.2k is a planning ceiling only. |
| The soldier's vertex and bone counts | OQ-0005-AS6 | the model's author, then the render budget bench (0005:1899-1900) | Measure on the first real model; note that hard edges and UV seams raise the vertex count (BIO-rst:50-51). |
| Compression formats; the LOD tool and levels | OQ-0005-AS1, AS2 | the research unit, then the asset spike | Not needed for Slice 1 (plain GLB, no LOD step). |
| Where the automated step runs, who writes the asset list, where files live | OQ-0005-AP1 | the research unit, then the asset spike (0005:1690-1693) | For the first model: whether a person writes its list entry and whether the gate recounts the file (PM-0005-38). |
| Which asset families are authored | OQ-0005-AP2 | Mustafa, in the look proof test | – |
| May an AI-generated draft model ship | OQ-0005-AP3 | Mustafa (0005:1706-1712) | Matters now that Claude makes the models through the Blender MCP. |
| May a committed model be public | OQ-0006-18 | Mustafa, before the first model is committed (0006:1105-1106; his item 13 of 2026-11-14) | – |
| The joint list (names, hierarchy, bind pose T or A), the origin, the facing, weapon socket names, whether headgear is its own asset | engine conventions, no id | the engine proposes (main session; engine-architect reviews); the Art brief takes them | Must exist before the code-built stand-in, so the swap changes no code (section 5). The legacy rig is the starting point: two-bone arms and legs, spine, chest and head, hand targets on the gun's grip and rail, four coat panels, and no ankle joint (`docs/LEGACY_INVENTORY.md` GA2, GA3, GA7, GA8, GA13). |
| The very large unit's part names | ADR 0004's vehicle-part records | ADR 0004 / the engine | Not needed for Slice 1. |
| "1 unit = 1 metre" in the decision drafts | STATUS.md:76 | the drafts' next revision | Already `[DECIDED]` in EDITOR_V2.md:64. |
| Hard or smooth edges, and the ink | OQ-0003-2 and the ink family | Mustafa, in the look proof test | Under the proposed screen-space ink, normal edges draw the inner lines (0003:190-191), so hard edges in the mesh may draw lines where the surface direction jumps [inferred from R3-LOOK-F7, not researched]; under the hull family, hard corners leave gaps unless the engine computes smoothed outline normals (0003:601; CL7 ports only if he picks the hull, 0003:788). |
| How WebGPURenderer skins, and its bone limit | R3-CHAR-OQ1 | a source reading before the spike (r3:1344-1346) | Partly answered on dev (section 2); read `getUniformBufferLimit()` per backend and the pinned release's file. |
| Mustafa's Blender version | – | the research unit | The option names and defaults above are the main branch's (add-on 5.4.12, Blender 5.3+); check them against the add-on in his Blender. |
| Blender's unit scale on export | – | the research unit | Whether a scene Unit Scale other than 1.0 changes the exported size [from memory, unverified]; the round trip settles it. |
| The round trip itself | AS3, AS4 | the research unit | Export a 1 m test cube and a skinned test figure (four material slots, one of them a faction slot, a short joint chain) through the real exporter, in Mustafa's Blender through the MCP or in a headless Blender (`blender --background --python`, [from memory, unverified]), and load both through the real loader on both backends (WebGPU and `forceWebGL`): names kept, one mesh per slot, the skin bound, at most 4 influences, 1.0 m along +Y, the front along +Z. Headless WebGPU timings are not hardware results; this checks structure only. |

**Tensions for the build plan (not mine to settle).**
- ADR 0003's migration plan puts "ADR 0005's asset spike before the first authored model enters the game" in slice 5
  (0003:1407-1408; SP:560 "Place: before the first authored model enters the game"), while Mustafa's addition of
  2026-11-14 has the first model replace the stand-in "when it is ready". His item 12 asks the plan to treat the spike's
  marks as tests and to say which of PM-0005-38 to 44 are measured at that change-set and which later.
- SP:406-408 places the look proof test "before slice 1's renderer"; his decision of 2026-11-14 changes that order
  (his addition lists the words it changes).

---

## 5. Does Slice 1's soldier need a skin

**What the drafts say: yes.**
- 0003:1397-1399, migration step 5: "**Slice 1, move & shoot:** one skinned soldier, the toon materials and ink kept in
  step 2, the feel kit's visual effects and voices for the slice's weapons, split feedback. *Milestone:* programs
  constant over a match, zero per-frame allocation in the render path, Mustafa's Brave check."
- 0005:1771-1774, GPU rule 3: "A soldier model is one skinned mesh with skin indices and weights as attributes [...] and
  any weights the artist paints".
- 0003:302-303, part 6: "One skinned mesh per soldier (option C-B) on both backends, with any skin weights".
- R3-CHAR-REC1 (r3:1032-1035) allows a part-built soldier on the same path: "one skinned mesh per soldier (C-B) as the
  baseline that runs on both backends (rigid weights, one bone per part, while the part-based model stays; any skin
  weights once the character model exists, CHAR-OQ3)".

**What "one skinned mesh" means once loaded (inferred from GLTFLoader.js, section 2).** One Blender mesh object with
four material slots becomes one glTF mesh with four primitives, which three.js returns as a Group of four
SkinnedMeshes bound to one shared Skeleton (4041-4080, 4374-4378). That is exactly C-B's "one draw per material per
soldier" (0003:641), and it is the structure the stand-in should copy.

**What the code-built stand-in must be so the swap changes no engine code (proposal, for the engine to confirm).**
1. The same object structure as the loader's output: a root holding one SkinnedMesh per material slot, all bound to
   one Skeleton of Bone objects; geometry built from parts and rigid-weighted, each part 100 % to one joint
   (R3-CHAR-REC1's "rigid weights, one bone per part"), with at most 4 influences per vertex.
2. The same joint list: names (already in their sanitised form: letters, digits, underscores), hierarchy and bind pose.
   The ported animation (GA1 to GA15 "port to TypeScript", 0003:797; the rest pose "folded into the animation port on
   the authored, skinned soldier", 0003:795) finds its bones by these names in both cases.
3. The same slot names and classes, so slot-to-class mapping, the shared toon materials and faction colouring run on
   the stand-in unchanged; at most 4 slots for the soldier as drawn, the weapon included.
4. The same scale, axes and origin (1 unit = 1 m, +Y up, front along +Z, origin on the ground between the feet), and
   the same attachment path for the weapon and headgear.
5. Reached through the same asset id. Two ways, for the build plan to weigh: (a) the asset list entry names a code
   builder for now (as AP-7's kit spec sits beside an authored asset, 0005:1626-1630) and a file later: the GLB loading
   path is then first exercised at the swap; (b) a tool writes the stand-in to a .glb (three.js's GLTFExporter, [from
   memory, unverified: not read]) and the game loads it through the real loader from Slice 1: the swap is then a new
   file and list entry only, but that .glb is a model file, which Mustafa's item 13 holds back until OQ-0006-18 is
   answered, unless the build generates it without committing it.

**What stays open.** The joint list and bind pose (engine); route (a) or (b) (build plan); weapon sockets and whether
headgear is its own asset (engine; AP-2 gives each "faction uniform" its own asset id, 0005:1562-1564); the soldier's
vertex and bone counts (AS6); whether the WebGL 2 fallback skins a GLB soldier identically (C-B is "checked on both
backends" by the spike, 0003:652-653).

---

## 6. The first-model change-set: engine work beyond the look change-set

Work that loading one GLB soldier needs beyond the toon materials, ink and stand-in the look change-set builds (as the
build plan defines that change-set). Draft lines in brackets; "proposed" where no draft fixes it.

1. **The loader.** three.js's GLTFLoader on both backends [0005:1803-1804], with no Draco, Meshopt or KTX2 decoder
   registered, since the files are plain GLB [SP:390-391; 0005:1715]; refuse a file that declares a required extension
   (proposed, an AS3 check).
2. **The asset list entry by id.** id, hashed file name, slots with their classes, no LOD levels, and counts of
   triangles, materials and textures [AP-2, 0005:1566-1569]; the soldier's client definition names the asset id, never
   a simulation definition [0005:1563-1565].
3. **Load-time refusals.** A dangling asset id, a slot whose class is not in the class table, and a presented id with no
   client entry each fail at load naming the file and the field; no fallback model [0005:1569-1572; spike part
   0005:1714-1718; PM-0005-33, "0 stand-in models drawn" in the look test's content]. Proposed additions: a joint list
   that differs from the engine's, a primitive with no material, a missing `NORMAL`.
4. **Slot name to material class.** The AP-3 table read at load [0005:1578-1585]; every loaded material replaced by the
   shared node toon material of its slot content, cached by content and compiled in the warm-up [GPU rules 1 and 7,
   0005:1748-1755, 1805-1809; 0003:146-149]; PM-0005-34 counts "0 meshes of a listed asset drawn with a material other
   than the shared material of its slot's content". The loader's own materials and textures are never drawn, so they
   are disposed after the swap, while the shared materials are never disposed while shared [0005:1808] (proposed:
   disposal on teardown is a review point).
5. **Faction colour.** One shared material per slot and faction under C-B, the colour taken from the faction's client
   definition [0005:1764-1765, 1582-1583, 1849-1853]; per-instance colour only if an instanced path wins its mark
   [0003:304-305]; PM-0005-42 checks the colour on both backends within 2/255.
6. **The skin bound to the engine's animation.** The loaded Skeleton's bones mapped by name to the engine's joint list,
   driven by the ported procedural animation under the budget LOD [0003:306-308, 0003:797]; bounds a fixed padded
   sphere, never recomputed per frame [0003:308]; each soldier its own bones over shared geometry and shared materials
   (proposed; the cloning method, for example three.js's SkeletonUtils, is [from memory, unverified]); zero per-frame
   allocation in the render path [0003:1398-1399].
7. **The caps measured on the real model.** Draw calls per soldier at most 4 per pass [0005:1778-1783; PM-0005-36];
   its triangle count against the 5.2k planning ceiling [0005:1787-1792].
8. **Serving the file.** Hashed name, immutable, under 25 MiB [AP-4, 0005:1592-1594; PM-0005-39]; at which change-set
   this is measured is the plan's call under Mustafa's item 12.
9. **Before any model file is committed:** Mustafa's answer to OQ-0006-18 [his item 13, 2026-11-14; 0006:1105-1106].

The asset spike's marks the plan must place for item 12 (0005:2313-2320): PM-0005-38 the tool processes the models and
writes entries that an independent recount matches; -39 files served hashed, immutable, under 25 MiB; -40 the gate
holds the per-unit budgets; -41 no glTF reader in `sim`, `server` or the player build; -42 every slot classed and every
faction slot coloured on both backends; -43 the stress scene within ADR 0003's frame totals with real models; -44 LOD
levels keep the silhouette (only with its condition).

---

## Not read (and what each was needed for)

- The Blender manual's own page (docs.blender.org): not on the allow-list. The repository copy of the documentation
  (BIO-rst) was read instead; whether it matches the manual page for Mustafa's Blender version is [unsure].
- The rendered glTF specification at registry.khronos.org: not on the allow-list; its source (Specification.adoc) was
  read instead.
- Where the Blender exporter cuts influences to "Bone Influences" and whether it renormalises: not found in the source
  files read (for the influences rule).
- Blender's scene Unit Scale and its effect on export: not read (for the scale rule).
- Blender's Front view direction and its "_L"/"_R" mirror-name handling: from memory (for the axes and names rules).
- `getUniformBufferLimit()` per backend and r186's skinning node file: not read (for the bone limit, R3-CHAR-OQ1).
- three.js GLTFExporter and SkeletonUtils: not read (for stand-in route (b) and per-soldier skeleton cloning).
- The Khronos glTF Validator: not read (as a candidate tool for the AS3 checks).
- A skinned GLB under `WebGPURenderer` with `forceWebGL`: no example read shows it (for the fallback).
- three.js's SkinnedMesh docs page: not re-read today; cited through R3-CHAR-F1 (r3:436-439).
