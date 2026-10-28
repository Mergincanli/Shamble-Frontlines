# R3-CHAR draft: characters (skinning, instancing, LOD)

Question 4 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S38–S44; S25, S26 of question 2; S18 of question 1).

## Findings

**R3-CHAR-F1.** A `SkinnedMesh` is "A mesh that has a [Skeleton] that can then be used to animate the vertices of the
geometry with skinning/skeleton animation." and "requires skin indices and weights as buffer attributes in its
geometry"; `DetachedBindMode` "is useful when sharing a skeleton across multiple skinned meshes."
· Source: https://threejs.org/docs/pages/SkinnedMesh.html · official · read 2026-10-28 · confirmed in the source.

**R3-CHAR-F2.** A skinned mesh's bounds do not follow its animation: the bounding sphere is "computed by the engine
once when it is needed, e.g., for ray casting and view frustum culling. If the skinned mesh is animated, the bounding
sphere should be recomputed per frame in order to reflect the current animation state."
· Source: https://threejs.org/docs/pages/SkinnedMesh.html (computeBoundingSphere) · official · read 2026-10-28 ·
confirmed in the source.

**R3-CHAR-F3.** Bone data reaches the vertex shader as a texture: `boneTexture` is "A texture holding the bone data for
use in the vertex shader.", built by `computeBoneTexture()`.
· Source: https://threejs.org/docs/pages/Skeleton.html · official · read 2026-10-28 · confirmed in the source (that
WebGPURenderer's skinning node uses the same path is not confirmed: its source page returned 404).

**R3-CHAR-F4.** `InstancedMesh` is for "a large number of objects with the same geometry and material(s) but with
different world transformations" and "will help you to reduce the number of draw calls"; it carries per-instance colour
(`instanceColor`) and per-instance morph weights (`morphTexture`); its page says nothing about skinning.
· Source: https://threejs.org/docs/pages/InstancedMesh.html · official · read 2026-10-28 · confirmed in the source.

**R3-CHAR-F5.** An instanced mesh is culled as one object: its bounding sphere is computed when needed "for ray casting
or view frustum culling", and "You may need to recompute the bounding sphere if an instance is transformed via
InstancedMesh#setMatrixAt."
· Source: https://threejs.org/docs/pages/InstancedMesh.html (computeBoundingSphere) · official · read 2026-10-28 ·
confirmed in the source ("culled as one object" is inferred from the one bounding sphere).

**R3-CHAR-F6.** `BatchedMesh` renders "a large number of objects with the same material but with different geometries
or world transformations", culls each object (`perObjectFrustumCulled`, default true), sorts them (`sortObjects`,
default true), has per-instance colour, and must reserve its vertex and index space up front (`setGeometryAt` "Throws
an error if there is not enough space reserved for geometry."); its page says nothing about skinning.
· Source: https://threejs.org/docs/pages/BatchedMesh.html · official · read 2026-10-28 · confirmed in the source.

**R3-CHAR-F7.** On the WebGPU backend r186 issues one `drawIndexed` per BatchedMesh entry (`passEncoderGPU.drawIndexed(
counts[ i ], 1, starts[ i ] / bytesPerElement, 0, i );`) and counts each in `renderer.info`; there is no multi-draw
indirect. So BatchedMesh saves state changes and per-object CPU work, not GPU draw commands, and `info.render.drawCalls`
does not fall with it.
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/renderers/webgpu/WebGPUBackend.js (`_draw`) ·
official (source) · read 2026-10-28 · confirmed in the source for the quote; the consequence is inferred.

**R3-CHAR-F8.** three.js's example of skinned instancing with one shared pose marks a glTF mesh as instanced by hand
(`child.isInstancedMesh = true;`, `const instanceCount = 30;`) and drives it with one AnimationMixer, so every instance
shows the same pose.
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/examples/webgpu_skinning_instancing.html · official
(example) · read 2026-10-28 · confirmed in the source for the quotes; "same pose" is inferred (one mixer, no offsets).

**R3-CHAR-F9.** three.js's example of skinned instancing with a pose per instance ("Per-instance poses are computed once
and reused by every render pass.") sets each instance's clip time on the CPU (`mixer.setTime( elapsed + timeOffsets[ i
] )`), copies its bone matrices into one storage buffer, skins every instance in a compute pass (`renderer.compute(
computeSkinning )`), and refuses to run without WebGPU ('No WebGPU support').
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/examples/webgpu_skinning_instancing_individual.html ·
official (example) · read 2026-10-28 · confirmed in the source for the quotes; the storage-buffer layout is from the
fetch tool's reading of the code (inferred).

**R3-CHAR-F10.** `LOD` is "a basic Level of Detail (LOD) mechanism" that switches objects "at the distances specified",
with `hysteresis` as a "Threshold used to avoid flickering at LOD boundaries, as a fraction of distance."; its page says
nothing about instanced meshes.
· Source: https://threejs.org/docs/pages/LOD.html · official · read 2026-10-28 · confirmed in the source.

**R3-CHAR-F11.** three.js r186's example list has no LOD or impostor example under WebGPU (it has `webgpu_skinning`,
`webgpu_skinning_instancing`, `webgpu_skinning_instancing_individual`, `webgpu_instance_mesh`, `webgpu_mesh_batch`,
`webgpu_instancing_morph`).
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/examples/files.json · official · read 2026-10-28 ·
inferred (names taken from the fetch tool's filtered list).

**R3-CHAR-F12.** Render bundles help static draws only on WebGPU: a `BundleGroup`'s contents are "considered as one
render bundle", `static` by default, and there is no gain on WebGL (R3-WGPU-F16, R3-GL2-F9).
· Source: https://threejs.org/docs/pages/BundleGroup.html · official · read 2026-10-28 · confirmed in the source.

**R3-CHAR-F13.** With a top-down camera at a fixed height, soldiers on screen sit at similar distances from the camera
(legacy CAM_STYLES: camHeight 15.5 to 52, FOV 12 to 52 degrees), so distance-based LOD (F10) separates near and far
soldiers less than in a third-person game; LOD by screen count, importance (own squad, targets) or update rate fits that
camera better.
· Source: docs/LEGACY_TUNING.md, CAM_STYLES; F10 · inferred (geometry of the camera; the v2 camera is look brief
pending).

### Legacy facts used
- **CL10** (character recipes): soldiers are data recipes of primitive parts with joints and material roles; the game
  uses one body (CHAR_MALE). Tells v2: rigid parts can become one mesh with one bone per part (rigid skinning).
- **GA15** (animation driver): CPU per soldier about 26 µs (runtime) to 51 µs (headless); 48 soldiers ≈ 1.2 to 2.4 ms
  per frame; "Nothing culls or simplifies animation for distant or off-screen soldiers"; IK stays CPU work; "rigid
  skinning of the parts would cut the per-part draw calls and Object3D matrix updates".
- **CL6, STATUS risk 3, TL15**: about 120 draw calls per on-screen soldier; 1,480 draw calls at 12 and 5,843 at 48 with
  every soldier on screen; 1,145 at 48 on a ring mostly off screen.
- **GN1**: one bar geometry and material per bot, a `traverse` that rewrites every mesh's emissive on hit flash; the row
  recommends an instanced bar and a per-instance flash.
- **GC16** (Juggernaut): a 5.6 m multi-part walker from boxes and cylinders, one new material per part.
- **CL9**: static merging cut village draw calls 398 to 202; soldiers are not merged.
- **R1-BND-REC2** (wave 1): the page receives one flat block per tick with sections per entity kind (soldiers, vehicle
  parts); **R1-TICK-REC5**: the main thread interpolates between two ticks every frame; **R2-INT-REC1**: the server's
  relevance region bounds which entities a client knows. Together they bound how many characters the renderer can ever
  be asked to draw and give it typed arrays to write instance data from.

## Options

| Option | What it is | Draw calls (per soldier, before shadows and outlines) | CPU per soldier | Backends | Risk | Fit with VISION_V2 §3, §6.4, §6.9 |
|---|---|---|---|---|---|---|
| C-A. Rigid parts as separate meshes (legacy) | One mesh per part, posed by Object3D transforms | one per part (legacy: about 120 per soldier with hulls, CL6) | matrix update per part (GA15) | both | The measured legacy cost at 48 (TL15) | Poor at 48+ |
| C-B. One skinned mesh per soldier, rigid weights (one bone per part) | Parts merged into one geometry per material; joints become bones; procedural IK writes bone matrices | one per material per soldier | IK and pose as today, plus a bone upload | both (bone texture, F3) | Per-frame bounds (F2) | Good up to tens of soldiers |
| C-C. Instanced skinning, one shared pose (F8) | One instanced skinned draw for a crowd | one per material for all | one mixer | both (inferred) | Every soldier in the same pose: unusable for aim and gait | Distant crowds only |
| C-D. Instanced skinning, pose per instance by compute (F9) | Bone matrices per instance in a storage buffer, a compute skinning pass | one per material for all | pose per soldier on CPU (F9), upload once | WebGPU only (F9; R3-GL2-F2, F3) | Needs a WebGL 2 path of its own | Strong on WebGPU; fallback gap |
| C-E. Instanced skinning by bone texture (custom TSL) | Per-instance bone matrices in a data texture read in the vertex stage | one per material for all | pose per soldier, one texture upload | both (inferred) | Not a shipped three.js node (F11); own code | Strong on both if it works |
| C-F. Baked vertex animation textures for far or idle soldiers | Pre-baked poses sampled in the vertex stage | one per material for all | none per soldier | both (inferred) | Loses procedural aim and IK; bake tooling | Only as a far LOD |
| C-G. Impostors (billboards) | Pre-rendered sprites for the farthest tier | one for all | none | both | Weak with a near-constant top-down distance (F13) | Weak fit |
| L-1. Distance LOD (`LOD`, F10) | Swap meshes by distance | – | per object | both | Little gain under a top-down camera (F13) | Partial |
| L-2. Budget LOD | Cap fully animated soldiers per frame by importance (own squad, nearest, shooting); others update at a lower rate or use a cheaper rig | – | bounded | both | Visible pops if the budget is low | Good fit for 48+ (GA15) |

Large units (VISION_V2 §6.4, §6.9): a few multi-part vehicles and walkers. Options: **V-A** parts as separate meshes in
a group (few units, so few draws; parts match the sim's vehicle-part records in R1-BND-REC2); **V-B** one rigid-skinned
mesh per unit (as C-B). Either fits "a few"; V-A keeps parts independently destructible and pickable, V-B halves draws.

## Recommendations

**R3-CHAR-REC1. Soldiers: one rigid-skinned mesh per soldier (C-B) as the baseline that runs on both backends; measure
instanced per-instance skinning (C-E on both backends, C-D as the WebGPU variant) against it in the spike, and adopt it
only if it wins at 48 soldiers on screen.**
Reason: separate parts are the measured legacy problem (CL6, TL15) and the inventory already points to rigid skinning
(GA15); C-B uses only documented three.js features on both backends (F1, F3). Per-instance compute skinning is WebGPU
only in three's example (F9), and the fallback lacks real compute (R3-GL2-F2), so it cannot be the only path.
Rests on: R3-CHAR-F1, F2, F3, F8, F9; R3-GL2-F2, F3; legacy CL6, CL10, GA15, TL15.
Serves: VISION_V2 §3 (48+ players, bots fill slots), §4, §6.7 (faction uniforms and variants as data: per-instance
colour, F4, F6).
Spike must prove (proposed): with 48 soldiers on screen in the benchmark scene, C-B uses at most 4 draw calls per
soldier per pass (before outlines), and the CPU cost of posing plus upload stays within 2.0 ms per frame on the
minimum machine; C-D or C-E is adopted only if it cuts total frame time by at least 20 % against C-B on both reference
machines.

**R3-CHAR-REC2. A budget LOD for animation (L-2), not only distance LOD: a fixed number of soldiers get the full
procedural rig each frame by importance, the rest update at a lower rate or with a cheaper rig; bounds set per soldier
to a fixed padded sphere, not recomputed per frame.**
Reason: legacy animates every soldier fully every frame (GA15: 1.2 to 2.4 ms at 48 by arithmetic, and nothing is
culled); distance LOD helps less under a near-constant top-down distance (F13, F10); per-frame bound recomputation is
a CPU cost the docs ask for on animated skinned meshes (F2), which a padded fixed sphere avoids.
Rests on: R3-CHAR-F2, F10, F13; legacy GA15.
Serves: VISION_V2 §3, §4 (30+ fps on the minimum laptop in the largest mode).
Spike must prove (proposed): at 48 soldiers on screen, animation CPU time per frame at most 1.0 ms on the minimum
machine at Low, with no soldier inside its padded sphere ever culled while visible (checked by rendering with culling
off and on over 600 frames: identical pixels).

**R3-CHAR-REC3. Soldiers' per-instance data (transform, faction colour, hit flash, health bar) is written from the
per-tick block into preallocated typed arrays, never by traversing scene objects; health bars and flashes are
instanced or per-instance attributes.**
Reason: R1-BND-REC2 delivers flat typed sections per entity kind and R1-TICK-REC5 interpolates on the main thread every
frame; instanced and batched meshes take per-instance colour (F4, F6); legacy's per-bot bars and traverse-based flash
(GN1) are the allocation and draw-call pattern to avoid.
Rests on: R3-CHAR-F4, F6; legacy GN1; wave 1 R1-BND-REC2, R1-TICK-REC5 (cited, not restated as findings).
Serves: VISION_V2 §3, §6.7, §6.9.
Spike must prove (proposed): zero bytes allocated per frame in the character update path over 60 s (heap sampling, as
legacy's method in TL15), and health bars for 48 soldiers in at most 2 draw calls.

**R3-CHAR-REC4. Large units: parts as separate meshes under one group (V-A), each part's transform from the sim's
vehicle-part records; static world geometry in bundles on WebGPU and merged by material content on both backends.**
Reason: "a few" large units add few draws (VISION_V2 §6.9); parts map one-to-one onto R1-BND-REC2's vehicle-part
section; static merging is legacy's measured win (CL9) and bundles help static draws on WebGPU only (F12). BatchedMesh
does not lower GPU draw commands on WebGPU (F7), so it is judged on CPU time, not on `drawCalls`.
Rests on: R3-CHAR-F7, F12; legacy CL9, GC16; wave 1 R1-BND-REC2.
Serves: VISION_V2 §6.4 (big, multi-part vehicles), §6.9.
Spike must prove (proposed): three large units (each at least 20 parts) on screen with 48 soldiers add at most 1.5 ms
GPU time and at most 150 draw calls against the scene without them, on the recommended machine at High.

## Open questions

- **R3-CHAR-OQ1. How WebGPURenderer skins.** The skinning node's source and doc pages returned 404 at the paths tried;
  whether it reads bones from a texture (F3) or a uniform or storage buffer, and its bone limit, are unconfirmed.
  Decided by: reading the node source at its current path (a source-reading task) before the spike.
- **R3-CHAR-OQ2. A bone-texture instanced path on WebGL 2 (C-E).** No three.js node ships it (F11). Decided by: the
  spike's comparison in REC1.
- **R3-CHAR-OQ3. The character model itself.** LOOP_V2 §11 puts the character rework outside v0 (CL10); vertex and
  bone counts per soldier are unknown until it exists. Decided by: the character rework, then R3-BUDGET's triangle
  budget.
- **R3-CHAR-OQ4. BatchedMesh against InstancedMesh for varied props and uniforms.** F7 means BatchedMesh's gain on
  WebGPU is CPU-side only, and on Firefox's fallback it loses multi-draw (R3-GL2-F7, F8). Decided by: CPU and GPU time
  in the benchmark scene.
- **R3-CHAR-OQ5. Camera framing.** F13 assumes a top-down camera like legacy's; the v2 framing is look brief pending
  (R2-INT-REC1 holds it as data). Decided by: the look brief, then the LOD tiers.

### Recommendations that hinge on a source not read
| Recommendation | Source | Reason not read |
|---|---|---|
| R3-CHAR-REC1 | three.js SkinningNode source and docs (r186) | HTTP 404 at the paths tried (OQ1) |

## Sources
Read (all 2026-10-28, all official three.js): S38 SkinnedMesh docs; S39 examples/files.json r186; S40
webgpu_skinning_instancing.html r186; S41 webgpu_skinning_instancing_individual.html r186; S42 InstancedMesh docs; S43
LOD docs; S44 Skeleton docs. Reused: S18 BundleGroup, S25 BatchedMesh, S26 WebGPUBackend.js.
Not read: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/nodes/accessors/SkinningNode.js (HTTP 404);
https://threejs.org/docs/pages/SkinningNode.html (HTTP 404).

## Cross-topic notes
- R1 (boundary): REC3 and REC4 assume R1-BND-REC2's per-kind sections carry aim yaw and pitch, ADS, reload, throw and
  draw state per soldier (GA15 lists them); R1-XU-OQ3 (shared memory or a transferred buffer) does not change the
  renderer side: it reads typed arrays either way.
- R2: R2-INT-REC1's relevance region caps the soldiers the client can draw; R2-SYNC-REC4's interpolation delay sets
  how old the drawn pose is (presentation only).
