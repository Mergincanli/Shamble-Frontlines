# R3-LOOK draft: candidate techniques for a comic look

Question 3 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S30–S37; S4, S13–S15 of question 1).

**Every technique here is "look brief pending".** This section gives capability, cost and failure modes only. It picks
no technique and no set of techniques; Mustafa and the Advisor decide the look in the look brief.
`docs/ART_DIRECTION.md` is cited only as the legacy look ("Living Comic", locked for the legacy build).

## Findings

### Toon ramps
**R3-LOOK-F1.** three.js ships a node toon material: "Node material version of MeshToonMaterial.", whose lighting model
is `ToonLightingModel`. *Look brief pending.*
· Source: https://threejs.org/docs/pages/MeshToonNodeMaterial.html · official · read 2026-10-28 · confirmed in the
source.

**R3-LOOK-F2.** The toon model quantizes diffuse light, "reduced to a small number of discrete shades to create a
comic-like, flat look.", by sampling the gradient map at `vec2( dotNL.mul( 0.5 ).add( 0.5 ), 0.0 )` and using only its
red channel (`return vec3( gradientMap.r );`); without a map it draws two bands with a derivative-smoothed edge (`mix(
vec3( 0.7 ), vec3( 1.0 ), smoothstep( … ) )`). Indirect light is multiplied by ambient occlusion
(`reflectedLight.indirectDiffuse.mulAssign( ambientOcclusion );`). *Look brief pending.*
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/nodes/functions/ToonLightingModel.js · official
(source) · read 2026-10-28 · confirmed in the source.

**R3-LOOK-F3.** A ramp costs no extra pass: it is one texture lookup inside the material's lighting (F2); because it is
plain TSL, it builds for both backends (R3-WGPU-F11). Each distinct ramp is a distinct material, so ramps per material
kind multiply materials and break batching unless materials are shared by content (legacy CL4). *Look brief pending.*
· Source: F2; R3-WGPU-F11; legacy CL2, CL4 · inferred (from the source code and the legacy rows; no GPU time measured).

### Outlines
**R3-LOOK-F4.** three.js's toon outline is an inverted hull drawn as a pass: "Represents a render pass for producing a
toon outline effect on compatible objects. Only 3D objects with materials of type `MeshToonMaterial` and
`MeshToonNodeMaterial` will receive the outline." *Look brief pending.*
· Source: https://threejs.org/docs/pages/ToonOutlinePassNode.html · official · read 2026-10-28 · confirmed in the
source.

**R3-LOOK-F5.** It draws each toon object twice, once with a cached back-face outline material (`material.side =
BackSide;`, one per toon material in `this._materialCache = new WeakMap();`) and once normally; the hull is pushed out
by `norm.mul( this.thicknessNode ).mul( pos.w )`, in clip space, with defaults `color = new Color( 0, 0, 0 ), thickness
= 0.003, alpha = 1`, and no per-vertex width yet (`// TODO: support outline thickness ratio for each vertex`).
*Look brief pending.*
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/nodes/display/ToonOutlinePassNode.js · official
(source) · read 2026-10-28 · confirmed in the source for the quotes; "drawn twice" and "constant width on screen rather
than in world units" are inferred (the fetch tool's reading of `updateBefore` and of the clip-space maths).

**R3-LOOK-F6.** three.js's screen-space edge node is a Sobel filter: "Post processing node for detecting edges with a
sobel filter. A sobel filter should be applied after tone mapping and output color space conversion."; it takes nine
samples ("fetch the 3x3 neighbourhood of a fragment") of one input's luminance and returns the gradient magnitude
(`valueGx.mul( valueGx ).add( valueGy.mul( valueGy ) ).sqrt()`). *Look brief pending.*
· Source: https://threejs.org/docs/pages/SobelOperatorNode.html and
https://raw.githubusercontent.com/mrdoob/three.js/r186/examples/jsm/tsl/display/SobelOperatorNode.js · official · read
2026-10-28 · confirmed in the source.

**R3-LOOK-F7.** Edges from depth and normals instead of colour need those buffers: a scene pass can emit extra outputs
"via MRT" and offers `getViewZNode` and `getLinearDepthNode` (R3-WGPU-F13), so a depth-and-normal edge filter is a
custom TSL node over the same scene pass, not a shipped three.js node. *Look brief pending.*
· Source: https://threejs.org/docs/pages/PassNode.html; F6 · inferred (no fetched three.js node does depth and normal
edges).

**R3-LOOK-F8.** The two outline families scale with different things: the hull adds one draw per outlined object and
its vertex work (F5; legacy CL6 measured the same doubling), while a screen-space filter costs one full-screen pass of
nine samples per pixel whatever the object count (F6), plus a normal attachment if normals are used (F7).
*Look brief pending.*
· Source: F5, F6, F7; legacy CL6 · inferred (no GPU time measured; a spike measures both).

### Ambient occlusion
**R3-LOOK-F9.** three.js ships GTAO: "Post processing node for applying Ground Truth Ambient Occlusion (GTAO) to a
scene."; its documented setup renders a "pre-pass for normals and depth" with MRT and then the scene pass again, feeding
AO into lighting (`builtinAOContext( aoPassOutput.sample( screenUV ).r )`); normals "can be automatically reconstructed
from depth values in the shader" when MRT is not available. *Look brief pending.*
· Source: https://threejs.org/docs/pages/GTAONode.html · official · read 2026-10-28 · confirmed in the source.

**R3-LOOK-F10.** GTAO's cost knobs: samples ("A higher value results in better quality but also in a more expensive
runtime behavior."; "Changing this member triggers a shader recompilation."), and `resolutionScale` (default 1; "a
value of `0.5` should be sufficient for most scenes."). Its noise needs `TRAANode`, which "introduces typical TAA
artifacts like ghosting and temporal instabilities", or "a manual denoise via `DenoiseNode`". *Look brief pending.*
· Source: https://threejs.org/docs/pages/GTAONode.html (properties) · official · read 2026-10-28 · confirmed in the
source.

**R3-LOOK-F11.** In that documented setup the scene's geometry is drawn twice per frame (pre-pass, then scene pass), so
its draw calls double, on top of the AO pass and a denoise or TAA pass. *Look brief pending.*
· Source: F9 · inferred (from the documented example; a single-pass variant applying AO after lighting is possible in
principle, unmeasured).

### Emissive bloom
**R3-LOOK-F12.** three.js's bloom node can be selective: "For a selective bloom, use the `emissive` material property
to control which objects should contribute to bloom or not. This can be achieved via MRT."; parameters strength (default
1), radius (default 0, "Must be in the range `[0,1]`") and threshold (default 0). *Look brief pending.*
· Source: https://threejs.org/docs/pages/BloomNode.html · official · read 2026-10-28 · confirmed in the source.

**R3-LOOK-F13.** The bloom node uses five blur mips (`this._nMips = 5;`), one bright-pass target plus a horizontal and a
vertical blur target per mip, each `{ depthBuffer: false, type: HalfFloatType }`, kernel sizes `[ 6, 10, 14, 18, 22 ]`,
and a weighted composite: twelve full-screen passes per frame at falling resolution. *Look brief pending.*
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/examples/jsm/tsl/display/BloomNode.js · official
(source) · read 2026-10-28 · confirmed in the source for the quotes; the pass count of twelve is inferred (the fetch
tool's count of `updateBefore`).

### Shared costs
**R3-LOOK-F14.** Memory per full-screen half-float RGBA target is width × height × 8 bytes: 16.6 MB at 1920 × 1080,
29.5 MB at 2560 × 1440 (the renderer's own output buffer is half float by default, R3-WGPU-F14); each MRT output
(normals, emissive) adds one such attachment, and 4 × MSAA (R3-WGPU-F15) multiplies the multisampled attachments by
four. *Look brief pending.*
· Source: R3-WGPU-F14, F15 · inferred (arithmetic; actual formats per attachment unmeasured).

**R3-LOOK-F15.** No allow-listed source read here gives GPU milliseconds for any of these techniques on any hardware.
· Source: S30–S37 · confirmed by absence in the sources read (the Activision GTAO paper and the shipped-game talks
were not read: part "Sources").

### Legacy facts used
- **ART_DIRECTION.md** (cited only as the legacy look): legacy's "Living Comic" uses cel ramps, ink outlines and a skin
  rim; nothing here proposes it for v2.
- **CL2**: legacy ramps are 1-pixel `RGBFormat` DataTextures (2 or 4 bands) per material kind; three's node toon model
  reads only the red channel (F2), so the ramp data carries over as single-channel textures if the brief keeps ramps.
- **CL3**: the skin rim was GLSL injection; any rim in v2 is a TSL node (R3-WGPU-F6).
- **CL6, CL7**: legacy's hull is in world units (0.09, characters 0.8×) with a ground clamp and mitred normals;
  three's `toonOutlinePass` scales in clip space and has no per-vertex width (F5). Legacy's measured cost: about 120
  draw calls per on-screen soldier (STATUS risk 3), 5,843 with 48 soldiers on screen (TL15).
- **CL9**: static merging kept hulls at one per merged mesh; a hull's cost falls with merging, a screen-space filter's
  does not depend on it.
- **CD15**: legacy has MSAA on, no post-processing (CLAUDE.md hard constraint "no post-processing (under review)"), and
  LinearEncoding with no tone mapping; RenderPipeline tone-maps by default (R3-WGPU-F12).
- **GW17**: the cutaway hides the ink of faded structures; a screen-space outline would need the same mask as an input
  (inferred).

## Options

All options are *look brief pending*. "Both backends" means the technique is plain TSL and builds for WebGPU and WebGL 2
(R3-WGPU-F11); none of them needs compute.

| Technique | Variant | Extra passes | Scales with | Extra memory (1080p) | Both backends | How it fails |
|---|---|---|---|---|---|---|
| Toon ramp | Node toon material with a gradient map (F1, F2) | none | materials | a few bytes per ramp | yes | Banding shimmer without smoothing (three smooths the default 2-band case only, F2); one material per ramp breaks sharing (F3) |
| Toon ramp | Custom TSL lighting (own bands, per-kind ramps, rim) | none | materials | none | yes | Own code to maintain; same material-count risk |
| Outline | Inverted hull, `toonOutlinePass` (F4, F5) | none (an extra draw per object) | outlined objects and vertices | none | yes | Doubles draw calls; screen-space width; only toon materials; gaps at hard corners without smoothed normals (legacy CL7) |
| Outline | Inverted hull, own TSL (world-unit width, ground clamp as in CL6) | none (an extra draw per object) | outlined objects | none | yes | Same draw-call doubling; own code |
| Outline | Screen-space Sobel on colour (F6) | 1 full-screen pass, 9 samples per pixel | pixels | one colour target if not already present | yes | Picks up texture detail; misses edges between same-coloured surfaces |
| Outline | Screen-space depth and normal edges (custom TSL over MRT, F7) | 1 full-screen pass | pixels | + one normal attachment (about 16.6 MB at RGBA16F, F14) | yes | Threshold tuning; thin lines alias; needs masking for cutaway fades (GW17) |
| Ambient occlusion | GTAO as documented (F9–F11) | depth-normal pre-pass (scene drawn twice) + AO + denoise or TAA | pixels and (pre-pass) draw calls | AO target, normal target, history if TAA | yes | Noise or TAA ghosting (F10); halos at depth gaps; recompiles when samples change (F10) |
| Ambient occlusion | GTAO at half resolution, normals from depth (F9, F10) | AO + denoise | pixels / 4 | AO target at quarter area | yes | Softer, may flicker on thin geometry |
| Ambient occlusion | Baked or vertex AO in builders (no post) | none | none per frame | per-vertex attribute | yes | Static only: no contact shadow from moving units |
| Bloom | Full-image bloom (F12) | 12 passes at falling resolution (F13) | pixels | 11 half-float targets (sizes per mip not read) | yes | Washes bright UI-like colours; needs a threshold |
| Bloom | Selective emissive bloom via MRT (F12) | same 12 + emissive output | pixels | + one emissive attachment | yes | Every emissive material must write the emissive output correctly |

## Recommendations

These recommend a measurement and a render path, never a look.

**R3-LOOK-REC1. Measure every candidate technique alone, in one look-test scene, before the look brief closes: GPU time
per pass and memory per target, on both reference machines and both backends, reported as a table for Mustafa and the
Advisor. The spike reports; it does not pick.**
Reason: no source read gives GPU times (F15); each family scales with something different (F8, F11, F13), so the
trade-offs depend on our scene (48+ soldiers, a few very large units).
Rests on: R3-LOOK-F8, F11, F13, F14, F15.
Serves: VISION_V2 §4 (both targets), §6.9 (spectacle budget).
Spike must prove (proposed): a table with, per technique and variant, GPU ms (median and p95 over 600 frames, R3-MEASURE)
at 1080p on the minimum and recommended reference machines, extra draw calls, and extra render-target memory from
`renderer.info.memory` (R3-MEASURE-F3), each measured with only that technique on.

**R3-LOOK-REC2. Build any post chain the brief picks on one scene pass with MRT (colour plus only the outputs the chosen
effects read: normals, emissive), so no effect re-renders the scene; every effect gets a quality-tier switch (off, or a
lower `resolutionScale`) from the start.**
Reason: MRT gives one geometry pass multiple outputs (R3-WGPU-F13), bloom and depth-normal edges read MRT outputs (F7,
F12), and GTAO's documented pre-pass doubles draw calls (F11), which hurts most where legacy already measures thousands
of draws (TL15). Resolution scaling is the documented cost knob (F10; R3-WGPU-F13).
Rests on: R3-LOOK-F7, F9, F10, F11, F12, F14; R3-WGPU-F13; legacy TL15.
Serves: VISION_V2 §4 (Low and High), §6.9.
Spike must prove (proposed): with every effect the brief keeps turned on, `renderer.info.render.drawCalls` stays
within 1.1 × the draw calls with all effects off (no scene re-render), and each effect's off and lowered settings each
cut its own GPU time by at least 40 %.

**R3-LOOK-REC3. If the brief keeps outlines, the spike measures both families at 48 soldiers on screen (hull and
screen-space), because the hull's cost follows draw calls and the screen-space cost follows pixels.**
Reason: the hull doubles draws per outlined object (F5, F8), the cost legacy measured at about 120 draw calls per
soldier (CL6); a screen-space pass is fixed per pixel (F6, F8). Which one looks right is the brief's call; which one
fits the budget is a measurement.
Rests on: R3-LOOK-F5, F6, F8; legacy CL6, TL15.
Serves: VISION_V2 §3 (48+ players), §4, §6.9.
Spike must prove (proposed): for each family, GPU and CPU frame time at 12, 24 and 48 soldiers on screen on the minimum
machine at Low; the result goes to the look brief.

## Open questions

- **R3-LOOK-OQ1. GPU cost of each technique on our hardware.** No source read gives numbers (F15). Decided by:
  R3-LOOK-REC1.
- **R3-LOOK-OQ2. Outline width model.** Legacy inks in world units (CL1, CL6); three's hull scales in clip space (F5); a
  screen-space filter is in pixels. Which one the look wants is the brief's call; the engine can build any of them.
- **R3-LOOK-OQ3. AO without the pre-pass.** Whether GTAO applied after lighting from a single MRT pass is good enough is
  not in the docs (F11). Decided by: the look-test scene, only if the brief keeps AO.
- **R3-LOOK-OQ4. Bloom target sizes.** The source read gives the number and type of bloom targets but not their sizes
  per mip (F13). Decided by: `renderer.info.memory.texturesSize` in the spike.
- **R3-LOOK-OQ5. Shipped-game practice.** Comic-look talks (for example Arc System Works' Guilty Gear Xrd, GDC 2015) and
  Valve's "Illustrative Rendering in Team Fortress 2" were not readable here. Decided by: the Advisor's search for an
  allow-listed copy, if the look brief wants them.

### Recommendations that hinge on a source not read
| Recommendation | Source | Reason not read |
|---|---|---|
| R3-LOOK-REC1 | Activision, "Practical Real-Time Strategies for Accurate Indirect Occlusion" (GTAO; activision.com) | outside the allow list |
| R3-LOOK-REC3 | Guilty Gear Xrd's art-style talk (GDC 2015, GDC Vault) | video only (GDC Vault redirects outside the allow list) |
| R3-LOOK-REC3 | Valve, "Illustrative Rendering in Team Fortress 2" (NPAR 2007; valvesoftware.com) | outside the allow list |

## Sources
Read (all 2026-10-28): S30 MeshToonNodeMaterial docs; S31 ToonLightingModel.js r186; S32 ToonOutlinePassNode docs; S33
ToonOutlinePassNode.js r186; S34 SobelOperatorNode docs and source r186; S35 GTAONode docs; S36 BloomNode docs; S37
BloomNode.js r186. All official (three.js). Reused: S4, S15.
Not read: the three sources in the table above.

## Cross-topic notes
- The look brief (Mustafa and the Advisor) receives R3-LOOK-REC1's table; R3 picks nothing.
- CLAUDE.md's legacy constraint "no post-processing (under review)" applies to legacy only; v2's post chain waits for
  the ADR on rendering (`docs/adr/0003`).
