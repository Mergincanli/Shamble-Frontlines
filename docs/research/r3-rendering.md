# R3: Rendering and presentation

- **Topic:** R3, rendering and presentation (`docs/V2_PHASE0_BRIEF.md` §4, row R3).
- **Author agent:** graphics-engineer (Opus 5.5, effort high), one owner agent for the whole topic (wave 2 of
  `.checkpoints/v2-research/plan.md`). Notes with every source summary and its exact quotes:
  `.checkpoints/v2-research/graphics-engineer-r3.md`; one draft per question:
  `.checkpoints/v2-research/graphics-engineer-r3-r3-<id>-draft.md`.
- **Date:** 2026-10-28 (every source below was read that day).
- **Status:** spot-checked (45 of 77 key claims). The verifier's table, its critical read, the owner's fixes and the
  second pass on them (2026-10-29: closed, nits only) are in part 7 ("Verifier spot-check").
- Glossary and cross-topic resolutions applied 2026-10-30 (see [GLOSSARY.md](GLOSSARY.md) and part 7).
- **The brief's questions for this topic (quoted, `docs/V2_PHASE0_BRIEF.md` §4, row R3):** "three.js WebGPURenderer with
  TSL; the WebGL 2 fallback; candidate techniques for a comic look (toon ramps, screen-space outlines, ambient
  occlusion, emissive bloom); characters (skinning, instancing, LOD); budgets for the minimum and recommended hardware;
  how we measure it; spatial audio with Web Audio"

| # | Brief question | Ids | Draft (`.checkpoints/v2-research/`) |
|---|---|---|---|
| 1 | three.js WebGPURenderer with TSL | R3-WGPU-F1–F22, REC1–REC4, OQ1–OQ5 | `graphics-engineer-r3-r3-wgpu-draft.md` |
| 2 | the WebGL 2 fallback | R3-GL2-F1–F14, REC1–REC3, OQ1–OQ5 | `graphics-engineer-r3-r3-gl2-draft.md` |
| 3 | candidate techniques for a comic look (toon ramps, screen-space outlines, ambient occlusion, emissive bloom) | R3-LOOK-F1–F15, REC1–REC3, OQ1–OQ6 | `graphics-engineer-r3-r3-look-draft.md` |
| 4 | characters (skinning, instancing, LOD) | R3-CHAR-F1–F13, REC1–REC4, OQ1–OQ6 | `graphics-engineer-r3-r3-char-draft.md` |
| 5 | budgets for the minimum and recommended hardware | R3-BUDGET-F1–F7, REC1–REC3, OQ1–OQ6 | `graphics-engineer-r3-r3-budget-draft.md` |
| 6 | how we measure it | R3-MEASURE-F1–F10, REC1–REC4, OQ1–OQ4 | `graphics-engineer-r3-r3-measure-draft.md` |
| 7 | spatial audio with Web Audio | R3-AUDIO-F1–F11, REC1–REC4, OQ1–OQ6 | `graphics-engineer-r3-r3-audio-draft.md` |

**Totals:** 92 findings, 25 recommendations, 47 open questions (38 own, 1 shared, 8 cross-topic); 62 sources read (61 official, 0 shipped game, 1 other),
22 sources not read.

**How to read this file.**
- Certainty marks: "confirmed in the source" means the source's own words, quoted in the finding, say it (the fetch
  tool returns at most about 125 characters verbatim, so long passages are quoted as fragments); "inferred" means my
  reading, my arithmetic, or a point seen only in the fetch tool's summary; "confirmed by absence" means the sources read
  were checked for the point and do not contain it.
- Tiers: official (specifications, browser and vendor documentation, MDN and its compatibility data, three.js's own
  docs, wiki, examples and source code), shipped game (none could be read: see part 8), other (caniuse.com).
- Legacy ids (CD15, CL6, GA15, …) are rows of `docs/LEGACY_INVENTORY.md`; `CAM_STYLES`, `TUNE.fx`, `JUG` are tables of
  `docs/LEGACY_TUNING.md`. The inventory's marks are working recommendations (the Advisor's R7-MARKS (a)); rows marked
  "look brief pending" stay pending. `docs/ART_DIRECTION.md` is cited only as the legacy look ("Living Comic", locked for
  the legacy build).
- Wave 1 ids (R1-TICK-REC5, R1-TICK-OQ6, R1-BND-REC2, R1-BND-REC5, R1-XU-OQ3, R1-DET-REC1, R1-DET-REC6, R2-SYNC-REC4,
  R2-INT-REC1) are cited, not restated as findings of this file.
- **The look is not decided.** Question 3 gives each technique's capability and cost only, each marked "look brief
  pending", and picks no technique or set of techniques. Recommendations here choose a renderer path, a measurement
  method or a budget, never a look.
- Every pass mark and every reference machine is proposed. No spike was run, no benchmark, no engine code; nothing here
  is decided.
- Inside a question's section, a bare id (F12, REC3) means that question's id.

## TL;DR

- three.js r186's WebGPURenderer ("experimental") falls back to WebGL 2 by itself; TSL builds WGSL and GLSL from one source.
- WebGPU: Chrome, Edge, Safari 26, Firefox on Windows and Apple-silicon Macs; Firefox on Linux and Intel Macs falls back.
- The fallback emulates compute and storage and has no GPU timer in Firefox or Safari; the four look techniques run on both.
- Recommended: one TSL renderer, a pinned release, precompiled pipelines, one skinned mesh per soldier, a budget LOD.
- Also: proposed Low and High budgets, frame times only from real hardware, an equal-power, voice-capped audio mixer.

## Findings

One finding per claim, numbered per question. Each question ends with the legacy facts it uses.

### Question 1: three.js WebGPURenderer with TSL (R3-WGPU)

Question 1 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S1–S18).

**R3-WGPU-F1.** The current three.js release is r186; its release page dates it "24 Sep" (no year shown); the
three releases before it are r185 (01 Jul), r184 (16 Apr) and r183 (20 Feb).
· Source: https://github.com/mrdoob/three.js/releases and https://github.com/mrdoob/three.js/releases/tag/r186 ·
official · read 2026-10-28 · confirmed in the source for the tag and day ("r186: 24 Sep 14:35 (latest)"); the year
(2026) is inferred from the release order and the page footer.

**R3-WGPU-F2.** r186 is still changing WebGPURenderer's core: it adds "Introduce compileComputeAsync()." and "Add
DirectRenderPipeline.", and fixes "Fix timestamp query limits." and "Fix draw offsets of `BatchedMesh`.".
· Source: https://github.com/mrdoob/three.js/releases/tag/r186 (WebGPURenderer section) · official · read 2026-10-28 ·
confirmed in the source (the quotes above).

**R3-WGPU-F3.** The API moves from release to release: r183 renamed the post-processing class ("`PostProcessing` has
been renamed to `RenderPipeline`."), r185 changed blending ("`WebGPURenderer` has changed how premultiplied alpha is
implemented"), and r186 removed a shadow type ("`PCFSoftShadowMap` with `WebGPURenderer` has been removed" … "Use
`PCFShadowMap` which is now soft as well.").
· Source: https://github.com/mrdoob/three.js/wiki/Migration-Guide (sections r182 → r183, r184 → r185, r185 → r186) ·
official · read 2026-10-28 · confirmed in the source.

**R3-WGPU-F4.** three.js calls WebGPURenderer experimental: "The renderer itself is still in an experimental state
although its maturity level has been greatly improved in the last years.", and advises "it's recommended to upgrade to
the latest version whenever possible."
· Source: https://threejs.org/manual/pages/webgpurenderer.html ("Migration") · official · read 2026-10-28 · confirmed in
the source.

**R3-WGPU-F5.** WebGPURenderer picks its backend itself: "WebGPURenderer uses a WebGPU backend by default and a WebGL 2
backend as a fallback."; the `forceWebGL` option forces WebGL 2 "for testing purposes".
· Source: https://threejs.org/manual/pages/webgpurenderer.html ("Usage"); https://threejs.org/docs/pages/WebGPURenderer.html
(forceWebGL: "the renderer uses a WebGL 2 backend no matter if WebGPU is supported or not.") · official · read
2026-10-28 · confirmed in the source.

**R3-WGPU-F6.** Custom GLSL does not carry over: "Custom materials based on ShaderMaterial, RawShaderMaterial and
modifications of built-in materials via onBeforeCompile() are not supported in WebGPURenderer." … "This part of your
application must be ported to node materials and TSL."; likewise "EffectComposer with its effect passes are not
supported because WebGPURenderer comes with a new, more modern post-processing stack."
· Source: https://threejs.org/manual/pages/webgpurenderer.html ("Migration") · official · read 2026-10-28 · confirmed in
the source.

**R3-WGPU-F7.** three.js does not promise that WebGPURenderer is faster: "depending on your application and scene
setup, you will encounter missing features or a better performance with WebGLRenderer."
· Source: https://threejs.org/manual/pages/webgpurenderer.html ("Migration") · official · read 2026-10-28 · confirmed in
the source.

**R3-WGPU-F8.** WebGLRenderer stays maintained but frozen in features: "WebGLRenderer is still maintained and the
recommended choice for pure WebGL 2 applications." and "there are no plans to add larger new features to the renderer
since the project's focus is now on WebGPURenderer."
· Source: https://threejs.org/manual/pages/webgpurenderer.html ("State of WebGLRenderer") · official · read 2026-10-28
· confirmed in the source.

**R3-WGPU-F9.** Start-up is asynchronous: "WebGPU is initialized in an asynchronous fashion."; an app that drives its
own loop with `requestAnimationFrame` must `await renderer.init();`, and `setAnimationLoop()` is advised "for best
compatibility".
· Source: https://threejs.org/manual/pages/webgpurenderer.html ("Usage"); https://threejs.org/docs/pages/Renderer.html
(setAnimationLoop) · official · read 2026-10-28 · confirmed in the source.

**R3-WGPU-F10.** `compileAsync()` "Compiles all materials in the given scene. This can be useful to avoid a phenomenon
which is called "shader compilation stutter", which occurs when rendering an object with a new shader for the first
time."
· Source: https://threejs.org/docs/pages/Renderer.html (compileAsync) · official · read 2026-10-28 · confirmed in the
source.

**R3-WGPU-F11.** TSL is "a Node-based shader abstraction, written in JavaScript", "capable of encoding code into
different outputs such as `WGSL`/`GLSL`", through "`WGSLNodeBuilder` aimed at WebGPU and `GLSLNodeBuilder` aimed at
WebGL2." So one TSL material serves both backends.
· Source: https://github.com/mrdoob/three.js/wiki/Three.js-Shading-Language ("Learning TSL", "Example", "Architecture") ·
official · read 2026-10-28 · confirmed in the source (the last sentence is inferred from the quotes).

**R3-WGPU-F12.** Post-processing is a node graph: `RenderPipeline` "can only be used with `WebGPURenderer`"; a
`pass( scene, camera )` node feeds `outputNode`; by default the pipeline applies tone mapping and the colour-space
conversion at its end (`outputColorTransform`), which "must be disabled for effects that expect to be executed after
tone mapping and color space conversion. A typical example is FXAA which requires sRGB input."
· Source: https://threejs.org/docs/pages/RenderPipeline.html · official · read 2026-10-28 · confirmed in the source.

**R3-WGPU-F13.** A scene pass "can provide multiple outputs via MRT for further processing" (`setMRT`,
`getTextureNode`, `getViewZNode`, `getLinearDepthNode`), and `setResolutionScale` renders a pass at "a factor that is
multiplied with the renderer's width and height".
· Source: https://threejs.org/docs/pages/PassNode.html · official · read 2026-10-28 · confirmed in the source.

**R3-WGPU-F14.** The renderer's internal output buffer defaults to half float: "The default `HalfFloatType` is
recommend for best quality. To save memory and bandwidth, `UnsignedByteType` might be used. This will reduce rendering
quality though."
· Source: https://threejs.org/docs/pages/WebGPURenderer.html (outputBufferType) · official · read 2026-10-28 ·
confirmed in the source.

**R3-WGPU-F15.** `antialias` enables MSAA; "When `antialias` is `true`, `4` samples are used by default."
· Source: https://threejs.org/docs/pages/WebGPURenderer.html (antialias, samples) · official · read 2026-10-28 ·
confirmed in the source.

**R3-WGPU-F16.** `BundleGroup` exposes WebGPU render bundles; "This module is only fully supported by `WebGPURenderer`
with a WebGPU backend." and "With a WebGL backend, the group can technically be rendered but without any performance
improvements."
· Source: https://threejs.org/docs/pages/BundleGroup.html · official · read 2026-10-28 · confirmed in the source.

**R3-WGPU-F17.** Chrome: WebGPU from 113 "Supported on ChromeOS, macOS, and Windows."; from 144 "Supported on ChromeOS,
macOS, Windows, and Linux (Intel Gen12+ GPUs only)."; Chrome Android from 121; Edge mirrors Chrome.
· Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/GPU.json (`GPU` support entries; main
branch, read 2026-10-28) · official (MDN's compatibility data) · read 2026-10-28 · confirmed in the source.

**R3-WGPU-F18.** Firefox: "Supports Windows since Firefox 141." (released July 22, 2025), "Supports macOS Tahoe on
Apple silicon since Firefox 145.", "Supports older macOS versions on Apple silicon since Firefox 147." (released January
13, 2026), "Does not support macOS on Intel CPUs.", "Does not support Linux."; Firefox for Android: not supported.
· Source: same GPU.json; https://developer.mozilla.org/en-US/docs/Mozilla/Firefox/Releases/141 ("The WebGPU API is now
fully supported on Windows, in all contexts except for service workers."); .../Releases/147 · official · read 2026-10-28
· confirmed in the source.

**R3-WGPU-F19.** Safari: WebGPU "is now shipping in Safari 26.0 for macOS, iOS, iPadOS, and visionOS." (post dated
September 15, 2025); Safari 26.0 runs on macOS Tahoe, Sequoia and Sonoma; WebKit says "WebGPU supersedes WebGL on macOS,
iOS, iPadOS, and visionOS and is preferred for new sites and web apps."
· Source: https://webkit.org/blog/17333/webkit-features-in-safari-26-0/ (WebGPU section); GPU.json (safari "26") ·
official · read 2026-10-28 · confirmed in the source.

**R3-WGPU-F20.** WebGPU is not Baseline ("This feature is not Baseline because it does not work in some of the most
widely-used browsers.") and needs a secure context ("The whole API is available only in a secure context.").
· Source: https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API (page last modified Sep 2, 2026) · official ·
read 2026-10-28 · confirmed in the source.

**R3-WGPU-F21.** caniuse.com marks Safari desktop 26.0–27.1 TP as "Partial support" and every Firefox version through
161 as "Disabled by default", which disagrees with MDN's data for Firefox on Windows (F18); its fetched text shows no
footnotes. Global usage with support: 87.82 %.
· Source: https://caniuse.com/webgpu (no update date; usage data September 2026) · other (aggregator) · read
2026-10-28 · inferred (the table came through the fetch tool's summary; footnotes not visible).

**R3-WGPU-F22.** The WebGPU specification is a W3C Candidate Recommendation Draft dated 15 September 2026. "Object
creation operations in WebGPU don’t return promises, but nonetheless are internally asynchronous."; the async forms
"createComputePipelineAsync() and createRenderPipelineAsync() return a Promise which resolves to a pipeline object when
creation of the pipeline has completed."
· Source: https://www.w3.org/TR/webgpu/ (status; §3.2.1; §10) · official · read 2026-10-28 · confirmed in the source.

#### Legacy facts used
- **CD15** (renderer setup): legacy runs one r128 WebGLRenderer with MSAA, pixel ratio capped at 2, PCFSoftShadowMap and
  LinearEncoding output ("authored hex = on-screen colour"). Tells v2: the soft shadow type it names is gone from
  WebGPURenderer (F3), and RenderPipeline applies tone mapping and a colour-space conversion by default (F12), so the
  colour-output rule is a look-brief decision with a technical switch (`outputColorTransform`).
- **CL3** (skin rim by `onBeforeCompile` string injection) and **CL6** (outline hull as a GLSL `ShaderMaterial`): both
  techniques are unsupported on WebGPURenderer (F6); any equivalent must be written in TSL. The inventory says so too.
- **GW17** (cutaway toggles `material.transparent` per mesh): on WebGPU, render pipelines are created objects (F22), so
  a new blend state can mean a new pipeline on first use; the row asks a spike to measure the hitch.
- **TL15** (perfbots): shader programs with every soldier on screen 16 vs 9, "each compiles once on first sight"
  (reports/2026-10-16-three-teams-core.md:50). Tells v2: first-sight compiles already happen in legacy; F10's
  `compileAsync` is the tool against them.
- **TL11 / TL17**: legacy tools pin three 0.128.0 and render headless through headless-gl (WebGL 1), so nothing in the
  toolkit exercises WebGPU today.

### Question 2: the WebGL 2 fallback (R3-GL2)

Question 2 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S19–S29; S3–S5, S12, S16, S18 from question 1).

**R3-GL2-F1.** The fallback is a separate backend inside WebGPURenderer: "A backend implementation targeting WebGL 2."
It is used when the browser has no WebGPU, or when `forceWebGL` is set (R3-WGPU-F5).
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/renderers/webgl-fallback/WebGLBackend.js (class
doc comment) · official (three.js source, r186) · read 2026-10-28 · confirmed in the source.

**R3-GL2-F2.** On WebGL 2, TSL compute runs as a vertex pass with transform feedback and rasterization off
(`gl.beginTransformFeedback( gl.POINTS );`, `state.enable( gl.RASTERIZER_DISCARD );`), and indirect or array dispatch
counts are refused: "WebGLBackend.compute(): The count parameter must be a single number, not
IndirectStorageBufferAttribute".
· Source: WebGLBackend.js r186, `compute()` · official (source) · read 2026-10-28 · confirmed in the source.

**R3-GL2-F3.** On WebGL 2, storage buffers are emulated: the GLSL builder "Setups the Pixel Buffer Object (PBO) for
the given storage", packing the data into a texture read back by texel fetch, and reports no native storage buffer
support; it has no atomics and no workgroup barrier, and it errors on subgroup nodes ("GLSLNodeBuilder: WebGLBackend
does not support the subgroupSize node").
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/renderers/webgl-fallback/nodes/GLSLNodeBuilder.js
(`setupPBO`, `supports`, `getSubgroupSize`) · official (source) · read 2026-10-28 · confirmed in the source for the two
quotes; the texture packing, `storageBuffer: false` and the absence of atomics and barriers are inferred (seen in the
fetch tool's summary of the file).

**R3-GL2-F4.** GPU timing on the fallback uses `EXT_disjoint_timer_query_webgl2` (`this.disjoint =
this.extensions.get( 'EXT_disjoint_timer_query_webgl2' );`) and is skipped when the extension is missing (`if ( !
this.disjoint || ! this.trackTimestamp ) return;`).
· Source: WebGLBackend.js r186, `init()`, `initTimestampQuery()` · official (source) · read 2026-10-28 · confirmed in the
source.

**R3-GL2-F5.** `EXT_disjoint_timer_query_webgl2` exists in Chrome desktop from 70 and Edge from 80; Firefox, Safari and
Chrome for Android have none ("version_added": false). (Its WebGL 1 sibling was "Removed due to the [GLitch exploit]"
in Chrome 65 and Firefox 59.)
· Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/EXT_disjoint_timer_query_webgl2.json and
.../EXT_disjoint_timer_query.json (main, read 2026-10-28) · official (MDN compatibility data) · read 2026-10-28 ·
confirmed in the source.

**R3-GL2-F6.** The extension's purpose: "provides a way to measure the duration of a set of GL commands, without
stalling the rendering pipeline."; "EXT_disjoint_timer_query_webgl2 is available in WebGL 2 contexts."
· Source: https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query (last modified Aug 29, 2024) ·
official · read 2026-10-28 · confirmed in the source.

**R3-GL2-F7.** BatchedMesh on the fallback depends on `WEBGL_multi_draw`: without it the backend draws each batch
entry in its own loop iteration (`if ( this.hasFeature( 'WEBGL_multi_draw' ) === false ) {`).
· Source: WebGLBackend.js r186, `_draw()` · official (source) · read 2026-10-28 · confirmed in the source (the quote);
"one draw per entry" is inferred from the fetch tool's description of the loop.

**R3-GL2-F8.** `WEBGL_multi_draw` exists in Chrome from 86 (Edge and Chrome Android mirror) and Safari from 15;
Firefox: "version_added": false.
· Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WEBGL_multi_draw.json · official · read
2026-10-28 · confirmed in the source.

**R3-GL2-F9.** Render bundles bring nothing on the fallback: "With a WebGL backend, the group can technically be rendered
but without any performance improvements."
· Source: https://threejs.org/docs/pages/BundleGroup.html · official · read 2026-10-28 · confirmed in the source (also
R3-WGPU-F16).

**R3-GL2-F10.** Some depth and MSAA paths need extensions on the fallback: "Shared depth texture is not supported with
MSAA when "WEBGL_multisampled_render_to_texture" is unsupported.", and "Unable to use reversed depth buffer due to
missing EXT_clip_control extension."
· Source: WebGLBackend.js r186, `_setFramebuffer()`, `init()` · official (source) · read 2026-10-28 · confirmed in the
source.

**R3-GL2-F11.** WebGL 2 itself is everywhere the four target browsers run: the compatibility data's `version_added`
values are chrome "56" (Edge mirrors Chrome), firefox "51" and safari "15".
· Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebGL2RenderingContext.json · official ·
read 2026-10-28 · confirmed in the source for the quoted `version_added` values (the source is a data file, so the
values are its only words); "everywhere the four browsers run" is inferred from them.

**R3-GL2-F12.** Who lands on the fallback, by MDN's data (R3-WGPU-F17–F19): Firefox on Linux and on Intel Macs
("Does not support Linux.", "Does not support macOS on Intel CPUs."), Chrome on Linux without an Intel Gen12+ GPU,
Safari before 26, and any machine whose browser offers no WebGPU adapter.
· Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/GPU.json · official · read 2026-10-28 ·
confirmed in the source for the quoted notes; the list as a whole is my reading (inferred).

**R3-GL2-F13.** Chrome says core WebGPU needs "modern, explicit graphics APIs", and that "on Windows, 31% of Chrome
users don't have Direct3D 11.1 or higher."; its "compatibility mode" ("an opt-in feature", via `featureLevel:
"compatibility"` in `requestAdapter()`) runs WebGPU on "older graphics APIs like OpenGL ES 3.1", first on Android, with
"ChromeOS with OpenGL ES 3.1 and Windows with Direct3D 11" being explored (February 2026).
· Source: https://developer.chrome.com/blog/new-in-webgpu-139 (July 30, 2025) and
https://developer.chrome.com/blog/new-in-webgpu-146 (February 25, 2026) · official · read 2026-10-28 · confirmed in the
source.

**R3-GL2-F14.** The fallback's GPU timing quality differs from WebGPU's in kind: WebGPU timestamps are quantized to
100 µs in Chrome by default (R3-MEASURE), while the WebGL 2 query measures "without stalling" but only in Chrome and Edge
(F5, F6).
· Source: F5, F6; https://developer.chrome.com/docs/web-platform/webgpu/developer-features · official · read
2026-10-28 · inferred (a comparison of two confirmed findings).

#### Legacy facts used
- **CD15, TL11, TL17**: legacy is WebGL only (r128 WebGLRenderer; headless-gl WebGL 1 in the tools). Every legacy
  technique (MSAA, one PCF shadow map, inverted hulls, sprites, pooled meshes) uses only what WebGL 2 has, so nothing
  legacy does needs WebGPU-only features.
- **CL9**: legacy's draw-call saving is CPU-side merging (village 398 to 202 draw calls, AUDIT_BUGS_PERF CS12); it works
  identically on any backend, unlike BatchedMesh on Firefox's fallback (F7, F8).
- **GP5**: legacy's perf overlay reads `renderer.info` (draw calls, triangles, programs); the WebGL 2 backend fills the
  same `renderer.info` (R3-WGPU, Info page), so the overlay ports to both backends.

### Question 3: candidate techniques for a comic look (R3-LOOK)

Question 3 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S30–S37; S4, S13–S15 of question 1).

**Every technique here is "look brief pending".** This section gives capability, cost and failure modes only. It picks
no technique and no set of techniques; Mustafa and the Advisor decide the look in the look brief.
`docs/ART_DIRECTION.md` is cited only as the legacy look ("Living Comic", locked for the legacy build).

#### Toon ramps
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

#### Outlines
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

#### Ambient occlusion
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

#### Emissive bloom
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

#### Shared costs
**R3-LOOK-F14.** Memory per full-screen half-float RGBA target is width × height × 8 bytes: 16.6 MB at 1920 × 1080,
29.5 MB at 2560 × 1440 (the renderer's own output buffer is half float by default, R3-WGPU-F14); each MRT output
(normals, emissive) adds one such attachment, and 4 × MSAA (R3-WGPU-F15) multiplies the multisampled attachments by
four. *Look brief pending.*
· Source: R3-WGPU-F14, F15 · inferred (arithmetic; actual formats per attachment unmeasured).

**R3-LOOK-F15.** No allow-listed source read here gives GPU milliseconds for any of these techniques on any hardware.
· Source: S30–S37 · confirmed by absence in the sources read (the Activision GTAO paper and the shipped-game talks
were not read: part "Sources").

#### Legacy facts used
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

### Question 4: characters (skinning, instancing, LOD) (R3-CHAR)

Question 4 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S38–S44; S25, S26 of question 2; S18 of question 1).

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

#### Legacy facts used
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
  relevance region bounds which entities a client knows. Together they bound how many soldiers the renderer can ever
  be asked to draw and give it typed arrays to write instance data from.

### Question 5: budgets for the minimum and recommended hardware (R3-BUDGET)

Question 5 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S45–S47; S12, S14 and earlier).

**Everything in this section is proposed.** The targets come only from VISION_V2 §3, §4 and §6: "a laptop with
integrated graphics runs the largest mode at 30+ fps on Low" and "a mid-range graphics card runs it at 60 fps on High";
the [goal mode](GLOSSARY.md#the-start-modes-and-the-goal-modes) is "12v12v12 or 24v24" (36 or 48 players; bots fill empty [slots](GLOSSARY.md#team-size-and-how-modes-are-written)), and CLAUDE.md says to design for [48+](GLOSSARY.md#the-design-point-and-headroom); §6.9 asks for "a few very large
units and their effects without dropping frames". The reference machines and every number below are proposals for the
ADR draft (`docs/adr/0003`), to be confirmed by Mustafa.

**R3-BUDGET-F1.** At 60 Hz a frame lasts 16.66 ms, and the browser needs part of it: "the browser has its own overhead
for each frame", so "all of your work needs to be completed inside 10 milliseconds."
· Source: https://web.dev/articles/rendering-performance (last updated 2023-12-13) · official (Google's web.dev) · read
2026-10-28 · confirmed in the source.

**R3-BUDGET-F2.** The render clock follows the display: "The frequency of calls to the callback function will generally
match the display refresh rate. The most common refresh rate is 60hz, … though 75hz, 120hz, and 144hz are also widely
used."; calls "are paused in most browsers when running in background tabs".
· Source: https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame (last modified Aug 21, 2026) ·
official · read 2026-10-28 · confirmed in the source.

**R3-BUDGET-F3.** HiDPI screens multiply the pixels to shade: `devicePixelRatio` is "the ratio of the resolution in
physical pixels to the resolution in CSS pixels"; "a value of 2 is expected for HiDPI/Retina displays" (four times the
pixels of a ratio of 1).
· Source: https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio (last modified Aug 21, 2026) ·
official · read 2026-10-28 · confirmed in the source (the factor of four is arithmetic).

**R3-BUDGET-F4.** The only GPU class an official source names for WebGPU support is Intel's: Chrome 144 supports WebGPU
on Linux with "Intel Gen12+ GPUs only"; on Windows and macOS Chrome names no GPU class.
· Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/GPU.json · official · read 2026-10-28 ·
confirmed in the source.

**R3-BUDGET-F5.** Memory per full-screen half-float colour target is 16.6 MB at 1920 × 1080 (8 bytes per pixel), the
renderer's output buffer is half float by default, and 4 × MSAA multiplies multisampled attachments by four
(R3-LOOK-F14, R3-WGPU-F14, F15).
· Source: R3-LOOK-F14 · inferred (arithmetic).

**R3-BUDGET-F6.** `renderer.info` reports draw calls ("The number of draw calls of the current frame.") and triangles
per frame, memory sizes in bytes (`texturesSize`, `attributesSize`, `total`: "The total memory size in bytes.") and
counts of programs ("The number of active programs.") and render targets ("The number of active renderTargets."), so
every budget below except GPU time and frame time can be read from the renderer itself (R3-MEASURE-F3).
· Source: https://threejs.org/docs/pages/Info.html · official · read 2026-10-28 · confirmed in the source for the
quotes; "every budget below except GPU time and frame time" is inferred (matching the counters to the rows of
R3-BUDGET-REC2's table).

**R3-BUDGET-F7.** No allow-listed source gives GPU throughput for integrated or mid-range GPUs under WebGPU or WebGL 2,
or a hardware share among players (the Steam Hardware Survey and vendor spec pages are off the allow list).
· Source: the sources read for R3 · confirmed by absence.

#### Legacy facts used
- **AUDIT_BUGS_PERF §1** (2026-09-26): `update()` 0.49 ms per frame CPU; draw calls including the shadow pass: village
  303, manor 238, centre 170, base 140; at most 12k triangles; P5: "Draw calls ~300/frame in the village (driver
  overhead on weak machines)", rated medium. **CL9**: village 202 after merging (STATUS health), triangles up to +12 %
  "still under 18k".
- **TL15** (reports/2026-10-16-three-teams-core.md:48–50): 48 soldiers' sim step 0.80–1.00 ms [mean](GLOSSARY.md#time-statistics) (p95 0.97–1.54);
  draw calls 1,480 at 12 and 5,843 at 48 with every soldier on screen; 16 shader programs.
- **GA15**: soldier animation 26 to 51 µs each (1.2 to 2.4 ms at 48 by arithmetic).
- **CD15**: pixel ratio capped at 2 with MSAA on; the row asks a spike to measure ratio 1 and 2 on integrated graphics.
  **CD17**: one 2048 × 2048 sun shadow map over a 100 m frustum.
- **STATUS health**: JS heap 42.7 MB at boot (editor rebuild row, 2026-10-14).
- **Phase-0 report** (docs/reports/2026-10-13-phase-0.md:9): Mustafa's PC has an RTX 5080, a high-end card, above the
  "mid-range" class of VISION_V2 §4.
- **R1 wave 1**: R1-TICK's spike scene (48 soldiers with full AI, 4 [big vehicles](GLOSSARY.md#big-vehicle-and-very-large-unit), 200 [live projectiles](GLOSSARY.md#live-projectile)) and its client
  mark "the page's frame time p95 with the sim in the worker is no worse than with no sim at all by more than 1.0 ms"
  (R1-TICK-REC5's and R1-BND's marks); R1 asks for the reference hardware "to be named with R3, R5 and R6". Since
  2026-10-30 (resolutions 9 and 11) R1-TICK-REC2 P1 and R3-BUDGET-REC2 share [the stress scene](GLOSSARY.md#the-stress-scene)
  (15 big vehicles, 300 live projectiles), and R1 names
  [the reference server](GLOSSARY.md#the-reference-server) and R3-BUDGET-REC1's minimum laptop as its reference hardware.

### Question 6: how we measure it (R3-MEASURE)

Question 6 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S48–S52; S5, S6, S16, S17, S19, S23, S26 from earlier questions).

**R3-MEASURE-F1.** WebGPU's optional `timestamp-query` feature "provides high precision timing of GPU operations", but
"To mitigate security and privacy concerns, the timing query values are aligned to a lower precision"; and
"cross-origin isolation (provided by COOP/COEP) does not provide isolation of device/queue-timeline timers."
· Source: https://www.w3.org/TR/webgpu/ §2.1.7.2 (W3C Candidate Recommendation Draft, 15 September 2026) · official ·
read 2026-10-28 · confirmed in the source.

**R3-MEASURE-F2.** In Chrome, "timestamp queries are quantized with a resolution of 100 microseconds"; "This
quantization is automatically disabled when the "WebGPU Developer Features" flag is enabled.", a flag Chrome says is for
development only ("Do not use these features in production."); the same flag adds adapter details (driver, backend,
type, `memoryHeaps`).
· Source: https://developer.chrome.com/docs/web-platform/webgpu/developer-features (2025-06-03) · official · read
2026-10-28 · confirmed in the source.

**R3-MEASURE-F3.** `renderer.info` gives per frame `render.drawCalls` ("The number of draw calls of the current
frame."), `render.triangles`, `render.timestamp` ("The timestamp of the frame."), `compute.timestamp`, and memory by
kind: byte sizes (`texturesSize`, `attributesSize`, `total`: "The total memory size in bytes.") and counts
(`programs`: "The number of active programs."; `renderTargets`: "The number of active renderTargets."); an app with its own loop sets `autoReset` to false and "must then call `renderer.info.reset()` once per frame
manually."
· Source: https://threejs.org/docs/pages/Info.html · official · read 2026-10-28 · confirmed in the source.

**R3-MEASURE-F4.** GPU timing in three.js is opt-in per backend: the WebGPU backend's option is documented as "Whether to
track timestamps with a Timestamp Query API or not." and is switched off unless the device has the feature
(`this.trackTimestamp = this.trackTimestamp && this.hasFeature( GPUFeatureName.TimestampQuery );`); results are read with
`async resolveTimestampsAsync( type = 'render' )`; renderer options pass through to the backend (`new BackendClass(
parameters )`); query pools hold 2048 queries ("// TODO: Variable maxQueries?").
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/renderers/webgpu/WebGPUBackend.js,
.../src/renderers/common/Renderer.js, .../src/renderers/webgpu/WebGPURenderer.js · official (source) · read 2026-10-28 ·
confirmed in the source.

**R3-MEASURE-F5.** r186 fixed "Fix timestamp query limits.", so this path is still moving.
· Source: https://github.com/mrdoob/three.js/releases/tag/r186 · official · read 2026-10-28 · confirmed in the source.

**R3-MEASURE-F6.** On the WebGL 2 backend three.js times with `EXT_disjoint_timer_query_webgl2` when present (R3-GL2-F4),
which only Chrome (70+) and Edge (80+) desktop ship (R3-GL2-F5).
· Source: R3-GL2-F4, F5 · official · read 2026-10-28 · inferred (a combination of two findings confirmed in their
sources; the quotes are in R3-GL2-F4 and F5).

**R3-MEASURE-F7.** CPU timers are coarsened: `performance.now()` resolution is "5 microseconds" in cross-origin isolated
contexts and "100 microseconds" in non-isolated ones; the `requestAnimationFrame` timestamp has "a minimal precision of
1 millisecond".
· Source: https://developer.mozilla.org/en-US/docs/Web/API/Performance/now ("Security requirements", last modified Aug
25, 2025); https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame · official · read 2026-10-28 ·
confirmed in the source.

**R3-MEASURE-F8.** Headless Chrome is not a hardware measurement by default: it reports "Software only, hardware
acceleration unavailable." and a "SwiftShader Device (Subzero)" GPU; even with GPU flags (`--enable-unsafe-webgpu`,
`--use-angle=vulkan`, `--enable-features=Vulkan`) and the right drivers, Chrome reported "Hardware accelerated but at
reduced performance."; `chrome://gpu` shows which case applies.
· Source: https://developer.chrome.com/blog/supercharge-web-ai-testing (last updated 2024-01-16) · official · read
2026-10-28 · confirmed in the source.

**R3-MEASURE-F9.** GitHub offers "GPU-powered runners" among its paid larger runners, "billed at the per-minute rate";
the page names no GPU or OS.
· Source: https://docs.github.com/en/actions/concepts/runners/larger-runners · official · read 2026-10-28 · confirmed in
the source.

**R3-MEASURE-F10.** `compileAsync()` exists to avoid "shader compilation stutter" (R3-WGPU-F10), and
`renderer.info.memory.programs` counts active programs (F3): together they make first-sight compiles visible in a
counter.
· Source: https://threejs.org/docs/pages/Renderer.html; https://threejs.org/docs/pages/Info.html · official · read
2026-10-28 · inferred (combination of two confirmed findings).

#### Legacy facts used
- **GP5** (perf overlay): samples raw frame deltas every frame, shows FPS, average and max frame time, `renderer.info`
  draw calls, triangles, geometries, textures, programs and JS heap four times a second, idle while closed. The row
  says "port it to the WebGPU renderer's info".
- **TL15** (perfbots): warm-up then measured steps (600 + 1,200), 3 reps, mean, p50, p95, max; a clustered worst case for
  draw calls; the method lesson that `node --heap-prof` under-counted allocation about 20 × and inspector sampling
  including collected garbage is the right method (reports/2026-10-16-three-teams-core.md:54–56). The row asks v2 to
  split it into a Rust sim bench and a client render bench.
- **TL9** (smoke): headless Chromium on SwiftShader; checks real behaviour, prints the renderer string; its timings do
  not count.
- **TL11** (preview.js): headless-gl WebGL 1; "Headless CPU timings only compare builds on the same machine" (TL15).
- **TL12** (pageshot): `--headed` uses the real GPU; "SwiftShader frame times and fonts do not count; Brave is the final
  check"; Mustafa's headed renders on his Windows PC were still pending (STATUS.md:7–8).
- **AUDIT_BUGS_PERF §1**: legacy's measurement set (runtime profile, sampling heap profiler including garbage, V8
  optimization status, kill-frame timing, soak, draw calls at heavy views) is the template for v2's client checks.

### Question 7: spatial audio with Web Audio (R3-AUDIO)

Question 7 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S53–S62).

The sound design itself (what each weapon sounds like) is LOOP_V2 §7's per-weapon feel kit and is not R3's to choose.
This section covers the spatial machinery, its cost and its limits.

**R3-AUDIO-F1.** A `PannerNode` "represents the location, direction, and behavior of an audio source signal in a
simulated physical space" and "uses right-hand Cartesian coordinates"; positions are relative to the context's one
`AudioListener`, and position and orientation are `AudioParam`s (`positionX` and so on).
· Source: https://developer.mozilla.org/en-US/docs/Web/API/PannerNode (last modified Sep 28, 2026) · official · read
2026-10-28 · confirmed in the source.

**R3-AUDIO-F2.** Two panning models: "`equalpower`: Represents the equal-power panning algorithm, generally regarded as
simple and efficient. `equalpower` is the default value." and "`HRTF`: Renders a stereo output of higher quality than
`equalpower` — it uses a convolution with measured impulse responses from human subjects."
· Source: https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel (Apr 7, 2023) · official · read
2026-10-28 · confirmed in the source.

**R3-AUDIO-F3.** Three distance models, `inverse` the default: linear `1 - rolloffFactor * (distance - refDistance) /
(maxDistance - refDistance)`; inverse `refDistance / (refDistance + rolloffFactor * (Math.max(distance, refDistance) -
refDistance))`; exponential `pow((Math.max(distance, refDistance) / refDistance, -rolloffFactor)` (as MDN prints it, with an
unbalanced parenthesis). Past `maxDistance` "the volume is not reduced any further"; three.js notes that `maxDistance` "is
used only by the `linear` distance model."
· Source: https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel (Apr 7, 2023);
https://developer.mozilla.org/en-US/docs/Web/API/PannerNode; https://threejs.org/docs/pages/PositionalAudio.html
(setMaxDistance) · official · read 2026-10-28 · confirmed in the source.

**R3-AUDIO-F4.** Neither inverse nor exponential ever reaches zero gain, so distance alone never silences a distant
source; only linear reaches zero (at `maxDistance` with `rolloffFactor` 1). Whatever its gain, even zero, a playing
source keeps its nodes until the game stops it, so no distance curve bounds the number of voices.
· Source: F3 · inferred (the first sentence from the formulas; the second is my reading of how audio nodes run, not
stated in a source read, and the spike's voice counter checks it: R3-AUDIO-OQ2).

**R3-AUDIO-F5.** three.js's `PositionalAudio` creates a PannerNode with `this.panner.panningModel = 'HRTF';`, so it
uses the costlier model by default (against Web Audio's own default, F2); every `updateMatrixWorld` while playing ramps
the panner to the object's position (`panner.positionX.linearRampToValueAtTime( _position.x, endTime );`, with `endTime
= this.context.currentTime + this.listener.timeDelta`), without allocating.
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/audio/PositionalAudio.js · official (source) · read
2026-10-28 · confirmed in the source for the quotes; "without allocating" is inferred (the fetch tool's reading).

**R3-AUDIO-F6.** three.js's `AudioListener` ("a virtual listener of the all positional and non-positional audio
effects") ramps the context's listener to its own world position the same way and owns a master gain
(`this.gain = this.context.createGain();`); three's example adds it to the camera.
· Source: https://raw.githubusercontent.com/mrdoob/three.js/r186/src/audio/AudioListener.js;
https://threejs.org/docs/pages/PositionalAudio.html (example) · official · read 2026-10-28 · confirmed in the source.

**R3-AUDIO-F7.** The simpler alternative, `StereoPannerNode`, is "designed to deal with the common use case of creating
simple left and right stereo panning effects" and is "much simpler to use, but obviously nowhere near as versatile.";
the panner "does some very involved math under the hood".
· Source: https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API/Web_audio_spatialization_basics (Aug 27,
2026) · official · read 2026-10-28 · confirmed in the source.

**R3-AUDIO-F8.** Occlusion has no node of its own; the building block is a `BiquadFilterNode` ("a simple low-order
filter" with "exactly one input and one output"), whose `lowpass` type is a "Standard second-order resonant lowpass
filter with 12dB/octave rolloff. Frequencies below the cutoff pass through; frequencies above it are attenuated."
· Source: https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode (Jun 23, 2025) · official · read
2026-10-28 · confirmed in the source (that occlusion is approximated with it is inferred; no source read describes
occlusion).

**R3-AUDIO-F9.** No source read gives the CPU cost of a voice, of HRTF against equal power, or a voice limit; the Web
Audio specification's PannerNode and spatialization sections could not be reached (the fetched text of the Editor's
Draft of 27 October 2026 stops before them).
· Source: https://webaudio.github.io/web-audio-api/ (Editor's Draft, 27 October 2026; read in part) and the MDN pages
above · confirmed by absence.

**R3-AUDIO-F10.** Starting sound "outside the context of handling a user input event is subject to autoplay rules."
· Source: https://developer.mozilla.org/en-US/docs/Web/Media/Guides/Autoplay (Sep 10, 2026) · official · read
2026-10-28 · confirmed in the source.

**R3-AUDIO-F11.** With a top-down camera 15.5 to 52 m above the ground (legacy CAM_STYLES), a listener on the camera
hears every on-screen source at nearly the same distance, so distance attenuation barely separates near from far; a
listener at the player's position (oriented with the camera, so left on screen is left in the ears) keeps distance
meaningful. Legacy already measures gain from the player and pans by the camera's right axis (GP6).
· Source: docs/LEGACY_TUNING.md CAM_STYLES; legacy GP6; F3 · inferred (geometry; the v2 camera is look brief pending).

#### Legacy facts used
- **GP6** (synthesized audio): every sound is generated at runtime from oscillators and noise buffers (no files);
  enemy shots and booms use inverse-distance gain and a `StereoPannerNode` against the camera's right axis; the
  AudioContext starts on the first pointer or key press; most shots build a fresh AudioBuffer in a JS loop (only the AMK
  and war-room sounds cache theirs); "there is no voice limit and no occlusion"; the only limits are a 55 ms impact
  throttle and the 90 m bot-shot sound cull. The row asks R3 for the distance model, the occlusion question and a spike
  that "counts concurrent voices in a 48-bot fight".
- **GN8 / TUNE.fx**: `botSfxR` 90 m and `botCullR` 50 m (LEGACY_TUNING); shots at or hitting the player are never culled.
- **GC16**: the Juggernaut's footfalls play a thunk with camera shake: large units have their own sounds (VISION_V2 §6.9
  "and their effects").
- **GW15**: legacy keeps an explosion line-of-sight list (`rebuildLOS`, `losMeshes`); occlusion could reuse a client
  copy of the static geometry the same way.
- **R1-TICK-REC5 / R1-BND-REC2** (wave 1): the sim runs in a Worker and the page receives events per tick (hits,
  deaths, …); sounds are triggered on the page from those events (inferred; R1's event section).

## Options

Trade-offs per question (complexity, cost, performance, risk, effort, fit with VISION_V2 §6). Look options are all *look brief pending*.

### Question 1: three.js WebGPURenderer with TSL (R3-WGPU)

| Option | What it is | Complexity and effort | Performance | Risk | Fit with VISION_V2 §4, §6.9 |
|---|---|---|---|---|---|
| A. WebGPURenderer with its built-in WebGL 2 backend (the brief's decision, `docs/V2_PHASE0_BRIEF.md` §0) | One renderer object, every material and pass in TSL (F11), the backend chosen at start (F5) | One shader language; TSL is new to the team; the API moves each release (F3) | WebGPU-only gains (render bundles, F16; compute) on WebGPU; three.js says some scenes run better on WebGLRenderer (F7) | "experimental" (F4); fallback differences (R3-GL2) | Serves all four browsers (F17–F19) through one code path |
| B. WebGPURenderer on WebGPU, plus a separate WebGLRenderer for the fallback | Two renderers, two material sets (TSL plus GLSL), since WebGLRenderer has no node materials ("investigating", S3) | Doubles material, post and outline work; two sets of visual tests | Fallback players get the mature renderer (F8) | Look drift between the two paths | Fits §4 but costs every look feature twice |
| C. WebGLRenderer only (WebGL 2) | The mature path three.js recommends "for pure WebGL 2 applications" (F8) | Lowest risk today | No compute, no bundles; no new features planned (F8) | Contradicts the brief's decision | Reference point only |

Version policy (applies to A and B):
- **V1 pin and step:** pin one exact release; upgrade one release at a time with the migration guide (F3) as the
  checklist and the benchmark scene (R3-MEASURE) re-run before and after.
- **V2 track latest:** follow every release as three.js advises (F4). Cheaper per step, but an API break (F3) can land
  in the middle of feature work.

### Question 2: the WebGL 2 fallback (R3-GL2)

| Option | What it is | Effort | What a fallback player gets | Risk | Fit with VISION_V2 §4, §6.9 |
|---|---|---|---|---|---|
| F-A. Same features on both backends ("lowest common denominator") | Use no compute, no storage writes, no bundles; everything WebGL 2 can do | Lowest; one look, one test matrix | The same game | Forgoes GPU particles or culling by compute on WebGPU | Keeps one look everywhere; §6.9 effects must fit CPU or vertex work |
| F-B. Tiered: WebGPU-only extras behind `hasFeature` / backend checks, with a defined fallback for each | e.g. compute particles on WebGPU, CPU or fewer particles on WebGL 2 | Two code paths per extra effect; tests on both | The same rules, fewer or simpler effects | Look drift between backends; every extra needs a fallback test | Fits §4 if the Low tier is the WebGL 2 tier |
| F-C. Fallback as a separate minimum tier only | WebGL 2 always runs the Low settings | Small | Low settings even on a strong machine | Strong Linux or Firefox-on-Mac machines get Low | Simple, but punishes some recommended-class machines |
| F-D. Also try WebGPU compatibility mode before WebGL 2 | `featureLevel: "compatibility"` adapter where core WebGPU is missing (F13) | Needs three.js support (unknown) | WebGPU on some older devices | Opt-in, platforms still being added (F13) | Could widen WebGPU reach on the minimum laptop |

### Question 3: candidate techniques for a comic look (R3-LOOK)

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

### Question 4: characters (skinning, instancing, LOD) (R3-CHAR)

| Option | What it is | Draw calls (per soldier, before shadows and outlines) | CPU per soldier | Backends | Risk | Fit with VISION_V2 §3, §6.4, §6.9 |
|---|---|---|---|---|---|---|
| C-A. Rigid parts as separate meshes (legacy) | One mesh per part, posed by Object3D transforms | one per part (legacy: about 120 per soldier with hulls, CL6) | matrix update per part (GA15) | both | The measured legacy cost at 48 (TL15) | Poor at 48+ |
| C-B. One skinned mesh per soldier (rigid weights, one bone per part, for a legacy-style part model; any skin weights once the character model exists, CHAR-OQ3) | Parts merged into one geometry per material; joints become bones; procedural IK writes bone matrices | one per material per soldier | IK and pose as today, plus a bone upload | both (bone texture, F3) | Per-frame bounds (F2) | Good up to tens of soldiers |
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

### Question 5: budgets for the minimum and recommended hardware (R3-BUDGET)

Reference machines:
| Option | Minimum | Recommended | Trade-off |
|---|---|---|---|
| M-A. Named models bought or borrowed for the project | one specific integrated-graphics laptop | one specific mid-range desktop card | Repeatable numbers; costs money and someone to run it |
| M-B. Classes only | "integrated graphics, 1080p" | "mid-range card" | Free; numbers not comparable between runs |
| M-C. Mustafa's PC as the recommended machine | – | RTX 5080 | Free and always there; far above mid-range, so it hides problems |
| M-D. A cloud GPU instance | – | a rented GPU | Repeatable; headless or virtual display, not a laptop; R6's cost |

Budget shape:
| Option | What is budgeted | Trade-off |
|---|---|---|
| B-1. Frame time only | p95 frame time per target | Simple; says nothing about where time goes |
| B-2. Frame time plus per-pass GPU, CPU, draw calls, triangles, memory | the table below | Every regression has an owner; needs timestamp queries (R3-MEASURE) |
| B-3. Adaptive (dynamic resolution) on top of B-2 | the renderer lowers its resolution scale to hold the frame time | Holds the target on weak machines; image softens under load; one more control loop |

### Question 6: how we measure it (R3-MEASURE)

GPU timing:
| Option | Where it works | Precision | Cost | Risk |
|---|---|---|---|---|
| T-A. WebGPU timestamp queries via three.js (`trackTimestamp`, `resolveTimestampsAsync`) | WebGPU backends whose device has the feature (F4) | 100 µs in Chrome by default (F2); finer with the developer flag | queries per pass; async readback | Path still being fixed (F5); feature may be missing on some browsers (R3-WGPU-OQ2) |
| T-B. WebGL 2 timer queries (`EXT_disjoint_timer_query_webgl2`) | Chrome and Edge only (F6) | not stated in the sources read | per pass | Missing in Firefox and Safari |
| T-C. CPU frame time only | everywhere | 100 µs (F7) | none | Cannot split GPU from CPU or by pass |

Where measurements run:
| Option | What it can prove | What it cannot |
|---|---|---|
| W-A. CI, headless (SwiftShader, or forced WebGL 2) | Zero errors; counts that do not depend on hardware: draw calls, triangles, programs, memory sizes, per-frame allocation | Any frame time (F8) |
| W-B. CI on a GPU runner (F9) | Repeatable GPU numbers on one cloud GPU | A laptop's integrated graphics; costs per minute |
| W-C. The reference machines, run by hand or as self-hosted runners | The VISION_V2 §4 targets | Runs only when the machines are on |
| W-D. Mustafa's Brave run | His hardware and browser (final visual gate) | Not the recommended class (RTX 5080, R3-BUDGET) |

### Question 7: spatial audio with Web Audio (R3-AUDIO)

Spatialization:
| Option | What it is | CPU | Quality | Fit |
|---|---|---|---|---|
| P-A. `StereoPannerNode` plus own gain (legacy) | left-right pan and a computed distance gain | lowest (F7) | no front-back or height cues | Fits a top-down view where on-screen left and right matter most |
| P-B. `PannerNode` with `equalpower` | 3D position against a listener, built-in distance model (F1–F3) | low ("simple and efficient", F2) | left-right with distance; no binaural cues | One listener, data-driven distance curves |
| P-C. `PannerNode` with `HRTF` (three.js's default, F5) | convolution with measured responses (F2) | highest; unmeasured (F9) | best on headphones | Only for few, important sources |
| P-D. Mixed: HRTF for a few priority voices, equal power for the rest | P-C + P-B by priority | bounded | good where it matters | Needs a priority system anyway (V-B) |

Voice management:
| Option | What it is | Trade-off |
|---|---|---|
| V-A. No limit (legacy) | every event plays | Unbounded nodes in a fight at the largest mode's N (48 or 54, MODES-3; GP6) |
| V-B. Voice cap with priorities | a fixed number of voices; priority by kind (own weapon, shots at the player, large units, nearby, far), distance and age; the lowest is stolen or never started; per-kind caps and throttles | Bounded cost; needs tuning; quiet sources vanish |
| V-C. Distance cull only (legacy's 90 m) | sources beyond a radius never start | Simple, but a cluster within the radius is still unbounded |

Occlusion:
| Option | What it is | Cost | Risk |
|---|---|---|---|
| O-A. None (legacy) | – | none | Sounds through walls read as in the open |
| O-B. Ray test from listener to source against static geometry, lowpass plus gain (F8) | one ray per playing voice, at a low rate | one filter per voice; rays on the CPU | Thin walls; floors need height-aware rays (VISION_V2 §6.2) |
| O-C. Room or zone based | "inside building X" flags from the map data; muffle between zones | cheap | Coarse |

## Recommendations

Each gives the reason, "Rests on:" every finding it uses, "Serves:" the VISION_V2 item(s), and proposed pass marks for a spike. None chooses a look.

### Question 1: three.js WebGPURenderer with TSL (R3-WGPU)

**R3-WGPU-REC1. Option A: one WebGPURenderer, every material and post effect in TSL, the WebGL 2 backend as the
fallback; no ShaderMaterial, onBeforeCompile or EffectComposer anywhere.**
Reason: TSL builds both WGSL and GLSL from one source (F11), so the fallback costs no second shader set; the GLSL routes
legacy used are unsupported on WebGPURenderer (F6, legacy CL3, CL6); option B doubles every look feature. The costs
are named, not hidden: "experimental" (F4) and possibly slower than WebGLRenderer for some scenes (F7), which the spike
measures.
Rests on: R3-WGPU-F4, F5, F6, F7, F8, F11, F12, F17, F18, F19; legacy CL3, CL6.
Serves: VISION_V2 §4 (four desktop browsers), §6.9 (spectacle budget).
Spike must prove (proposed): (1) one benchmark scene (R3-MEASURE-REC2) renders on the WebGPU backend in Chrome, Edge,
Firefox on Windows and Safari 26, and on the WebGL 2 backend (`forceWebGL`) in the same browsers, with zero console
errors; (2) the two backends' screenshots of the same frame differ in at most 1 % of pixels by more than 8/255 per
channel (proposed tolerance; post effects that exist on one backend only are switched off for this test); (3) the
WebGPU backend's median CPU frame time is no worse than 1.1 × the WebGL 2 backend's on the same machine (F7 is the
reason to check).

**R3-WGPU-REC2. Pin one exact three.js release (r186 today, or the newest at the spike's start) and step one release at
a time, with the migration guide as the checklist and the benchmark scene re-run (version policy V1).**
Reason: the API renamed post-processing (r183), changed premultiplied alpha (r185) and removed a shadow type (r186)
within a year (F3); r186 still fixes timestamp queries and BatchedMesh offsets (F2). Tracking every release (F4's
advice) is still the direction, just never mid-feature.
Rests on: R3-WGPU-F1, F2, F3, F4.
Serves: VISION_V2 §4 (no regression slips into the 30 and 60 fps targets unmeasured). No VISION_V2 §6 item: it serves
the §4 targets only.
Spike must prove (proposed): an upgrade from one pinned release to the next changes the benchmark scene's p95 GPU and
CPU frame time by no more than 5 % without an explained cause (R3-MEASURE).

**R3-WGPU-REC3. Build every render pipeline behind the loading screen: `await renderer.init()`, then `compileAsync()` on
a warm-up scene that holds every material variant a match can show (each faction, each effect, each blend state); the
match itself creates no new shader programs.**
Reason: WebGPU creates pipelines asynchronously (F22) and three.js names the first-sight hitch "shader compilation
stutter" (F10); legacy already compiles on first sight (TL15: 16 programs vs 9); the cutaway's blend-state toggle
(GW17) is one such variant. Init itself is asynchronous (F9).
Rests on: R3-WGPU-F9, F10, F22; R3-MEASURE-F3 (mark 1's counter); legacy TL15, GW17.
Serves: VISION_V2 §6.9 (large units and their effects "without dropping frames"), §4.
Cross-topic open questions: R3-XT-OQ8 (Open questions, "Cross-topic open questions").
Spike must prove (proposed): (1) `renderer.info.memory.programs` (R3-MEASURE-F3) is the same 5 s after deploy and at
the end of a scripted 5-minute match; (2) the first appearance of each unit type and effect causes no frame above
1.5 × the median frame time of that run.

**R3-WGPU-REC4. Detect and report the backend: the client logs which backend runs (WebGPU or "WebGL 2 fallback") with
the adapter's info, shows it in the performance overlay, and every test screenshot prints it; the game is served from
a secure context (HTTPS, or localhost in development).**
Reason: the backend is chosen inside the renderer (F5), WebGPU's reach differs by browser and OS (F17–F19, F21), and
the API exists only in secure contexts (F20). Legacy's pageshot already prints the WebGL renderer string on every shot
(TL12), and the inventory asks v2 to print the WebGPU adapter or "WebGL 2 fallback".
Rests on: R3-WGPU-F5, F17, F18, F19, F20, F21; legacy TL12.
Serves: VISION_V2 §4. No VISION_V2 §6 item: the backend logging and the overlay line serve the §4 targets only.
Cross-topic open questions: R3-XT-OQ3 (Open questions, "Cross-topic open questions").
Spike must prove (proposed): on each browser of F17–F19 the overlay and the test log name the backend correctly
(checked against `navigator.gpu` availability) in 10 of 10 starts.

### Question 2: the WebGL 2 fallback (R3-GL2)

**R3-GL2-REC1. Option F-A for the first slice, F-B only by measured need: every technique the engine ships in the first
spikes must exist on the WebGL 2 backend; a WebGPU-only extra (compute, storage writes, bundles) enters only with a
named fallback and a measured gain.**
Reason: compute on the fallback is a transform-feedback emulation without indirect dispatch (F2), storage is texture
emulation without atomics (F3), and bundles give no gain there (F9). Whole platform groups land there (F12: Firefox on
Linux and on Intel Macs, Chrome on Linux without an Intel Gen12+ GPU); their share of players is unknown (OQ2). F13's
31 % is about Direct3D 11.1 among Chrome users on Windows, which is what compatibility mode targets, not a share of
fallback users; it shows only that hardware below core WebGPU's needs is common. Legacy's look uses nothing WebGL 2 lacks (CD15), so F-A loses nothing legacy had.
Rests on: R3-GL2-F2, F3, F9, F12, F13; legacy CD15.
Serves: VISION_V2 §4 (four browsers; the minimum laptop), §6.9.
Spike must prove (proposed): the benchmark scene (R3-MEASURE-REC2) shows the same features on both backends
(screenshot comparison as in R3-WGPU-REC1 mark 2) and the WebGL 2 backend meets the minimum budget (R3-BUDGET-REC2) on
the minimum reference machine.

**R3-GL2-REC2. Do not let the draw-call budget depend on BatchedMesh or multi-draw on the fallback: test Firefox's WebGL 2
path (no `WEBGL_multi_draw`) as the worst case, and count draw calls there.**
Reason: without `WEBGL_multi_draw` the backend draws each batch entry separately (F7), Firefox has no such extension
(F8), and Firefox on Linux and Intel Macs has no WebGPU (F12), so that combination is a real worst case.
Rests on: R3-GL2-F7, F8, F12.
Serves: VISION_V2 §4 (Firefox among the four browsers), §6.9.
Spike must prove (proposed): on Firefox with the WebGL 2 backend, the benchmark scene's `renderer.info.render.drawCalls`
stays within the Low budget (R3-BUDGET-REC2) and its median frame time within 33.3 ms on the minimum reference
machine (if that machine runs Linux or an Intel Mac; otherwise on the closest available).

**R3-GL2-REC3. GPU timing on the fallback is best effort: use `EXT_disjoint_timer_query_webgl2` where it exists (Chrome,
Edge) and CPU frame time elsewhere; never gate CI or a pass mark on a GPU timer the browser does not offer.**
Reason: only Chrome and Edge ship the extension (F5); three.js skips timing without it (F4).
Rests on: R3-GL2-F4, F5, F6.
Serves: VISION_V2 §4 (measuring the targets on every browser). No VISION_V2 §6 item: it serves the §4 targets only.
Spike must prove (proposed): the measurement harness (R3-MEASURE-REC1) reports GPU time on Chrome's WebGL 2 backend and
reports "GPU time not available" (never 0) on Firefox and Safari's WebGL 2 backends.

### Question 3: candidate techniques for a comic look (R3-LOOK)

These recommend a measurement and a render path, never a look.

**R3-LOOK-REC1. Measure every candidate technique alone, in one look-test scene, before the look brief closes: GPU time
per pass and memory per target, on both reference machines and both backends, reported as a table for Mustafa and the
Advisor. The spike reports; it does not pick.**
Reason: no source read gives GPU times (F15); each family scales with something different (F8, F11, F13), so the
trade-offs depend on our scene (48+ soldiers, a few very large units).
Rests on: R3-LOOK-F8, F11, F13, F14, F15; R3-MEASURE-F3 (the mark's counters).
Serves: VISION_V2 §4 (both targets), §6.9 (spectacle budget).
Spike must prove (proposed): a table with, per technique and variant, GPU ms (median and p95 over 600 frames, R3-MEASURE)
at 1080p on the minimum and recommended reference machines, extra draw calls, and extra render-target memory from
`renderer.info.memory` (R3-MEASURE-F3), each measured with only that technique on.

**R3-LOOK-REC2. Build any post chain the brief picks on one shared scene pass with MRT (colour plus only the outputs
the chosen effects read: normals, emissive), so effects that only read buffers never re-render the scene; a variant
that does draw geometry again (the hull outline's second draw of each toon object, F5; GTAO's documented pre-pass, F9,
F11) is counted as a planned extra, not ruled out; every effect gets a quality-tier switch (off, or a lower
`resolutionScale`) from the start.**
Reason: MRT gives one geometry pass multiple outputs (R3-WGPU-F13), and bloom and depth-normal edges read MRT outputs
(F7, F12), so they need no re-render. The hull outline and GTAO as documented do draw geometry again (F5, F11), which
hurts most where legacy already measures thousands of draws (TL15), so their extra draws must show in the count, not
hide in it. Whether to keep them is the brief's call: R3-LOOK-REC3 measures both outline families, R3-LOOK-OQ3 asks
about AO without the pre-pass. Resolution scaling is the documented cost knob (F10; R3-WGPU-F13).
Rests on: R3-LOOK-F5, F7, F9, F10, F11, F12, F14; R3-WGPU-F13; R3-MEASURE-F3 (the mark's counter); legacy TL15.
Serves: VISION_V2 §4 (Low and High), §6.9.
Spike must prove (proposed): with every effect the brief keeps turned on, `renderer.info.render.drawCalls` stays
within 1.1 × the sum of the draw calls with all effects off and the planned extras (one per outlined object for a
hull outline, one per drawn object for a GTAO pre-pass), so no unplanned re-render slips in; and each effect's off and
lowered settings each cut its own GPU time by at least 40 %.

**R3-LOOK-REC3. If the brief keeps outlines, the spike measures both families at 48 soldiers on screen (hull and
screen-space), because the hull's cost follows draw calls and the screen-space cost follows pixels.**
Reason: the hull doubles draws per outlined object (F5, F8), the cost legacy measured at about 120 draw calls per
soldier (CL6); a screen-space pass is fixed per pixel (F6, F8). Which one looks right is the brief's call; which one
fits the budget is a measurement.
Rests on: R3-LOOK-F5, F6, F8; legacy CL6, TL15.
Serves: VISION_V2 §3 (48+ players), §4, §6.9.
Spike must prove (proposed): for each family, GPU and CPU frame time at 12, 24 and 48 soldiers on screen on the minimum
machine at Low; the result goes to the look brief.
Term note (outside the look row, which keeps its words under condition 5): its "48" means the largest mode's [N](GLOSSARY.md#player-count-n) (48 or 54, MODES-3).

### Question 4: characters (skinning, instancing, LOD) (R3-CHAR)

**R3-CHAR-REC1. Soldiers: one skinned mesh per soldier (C-B) as the baseline that runs on both backends (rigid weights,
one bone per part, while the part-based model stays; any skin weights once the character model exists, CHAR-OQ3);
measure instanced per-instance skinning (C-E on both backends, C-D as the WebGPU variant) against it in the spike, and
adopt it only if it wins at [the largest mode](GLOSSARY.md#the-largest-mode-and-its-n)'s N soldiers on screen (48, or 54 under MODES-3 (b)).**
Reason: separate parts are the measured legacy problem (CL6, TL15) and the inventory already points to rigid skinning
for today's part-based soldiers (GA15); C-B uses documented three.js skinning (F1; F3, the Skeleton page's bone
texture), but how WebGPURenderer skins is unconfirmed (CHAR-OQ1), so the spike's mark checks C-B on both backends.
Per-instance compute skinning is WebGPU only in three's example (F9), and the fallback lacks real compute
(R3-GL2-F2), so it cannot be the only path.
Rests on: R3-CHAR-F1, F2, F3, F4, F6, F8, F9; R3-GL2-F2, F3; legacy CL6, CL10, GA15, TL15.
Serves: VISION_V2 §3 (48+ players, bots fill slots), §4, §6.7 (faction uniforms and variants as data: per-instance
colour, F4, F6).
Spike must prove (proposed): with the largest mode's N soldiers on screen (48 or 54, MODES-3) in the benchmark scene, C-B renders on both backends with at
most 4 draw calls per soldier per pass (before outlines); the CPU cost of posing plus bone upload with every soldier on
the full rig (no budget LOD) stays within 2.0 ms (p99) per frame on the minimum machine, and with R3-CHAR-REC2's budget LOD
at Low within that tier's 1.0 ms (p99) (R3-BUDGET-REC2); C-D or C-E is adopted only if it cuts total frame time by at least 20 % against C-B on both reference
machines.

**R3-CHAR-REC2. A budget LOD for animation (L-2) as the base, under any camera: a fixed number of soldiers get the full
procedural rig each frame by importance, the rest update at a lower rate or with a cheaper rig; distance LOD (L-1) on
top only if the look brief's camera spreads soldiers over a wide range of distances (CHAR-OQ5); bounds set per soldier
to a fixed padded sphere, not recomputed per frame.**
Reason: legacy animates every soldier fully every frame (GA15: 1.2 to 2.4 ms at 48 by arithmetic, and nothing is
culled), and a budget LOD bounds that cost whatever the camera; distance LOD helps less under a legacy-like,
near-constant top-down distance (F13, F10), but the v2 camera is look brief pending (CHAR-OQ5), so L-1 stays
conditional; per-frame bound recomputation is a CPU cost the docs ask for on animated skinned meshes (F2), which a
padded fixed sphere avoids.
Rests on: R3-CHAR-F2, F10, F13; legacy GA15.
Serves: VISION_V2 §3, §4 (30+ fps on the minimum laptop in the largest mode).
Spike must prove (proposed): at the largest mode's N soldiers on screen (48 or 54, MODES-3), animation CPU time per frame (posing plus bone upload, as in
R3-CHAR-REC1) at most 1.0 ms (p99) on the minimum machine at Low, with no soldier inside its padded sphere ever culled while visible (checked by rendering with culling
off and on over 600 frames: identical pixels).

**R3-CHAR-REC3. Soldiers' per-instance data (transform, faction colour, hit flash, health bar) is written from the
per-tick block into preallocated typed arrays, never by traversing scene objects; health bars and flashes are
instanced or per-instance attributes.**
Reason: R1-BND-REC2 delivers flat typed sections per entity kind and R1-TICK-REC5 interpolates on the main thread every
frame; instanced and batched meshes take per-instance colour (F4, F6); legacy's per-bot bars and traverse-based flash
(GN1) are the allocation and draw-call pattern to avoid.
Rests on: R3-CHAR-F4, F6; legacy GN1; wave 1 R1-BND-REC2, R1-TICK-REC5 (cited, not restated as findings).
Serves: VISION_V2 §3, §6.7, §6.9.
Cross-topic open questions: R3-XT-OQ2 (Open questions, "Cross-topic open questions").
Spike must prove (proposed): zero bytes allocated per frame in the character update path over 60 s (heap sampling, as
legacy's method in TL15), and health bars for the largest mode's N soldiers (48 or 54, MODES-3) in at most 2 draw calls.

**R3-CHAR-REC4. Large units: parts as separate meshes under one group (V-A), each part's transform from the sim's
vehicle-part records; static world geometry merged by material content on both backends, and put in render bundles on
WebGPU only if the spike measures a gain (R3-GL2-REC1's rule for WebGPU-only extras).**
Reason: "a few" large units add few draws (VISION_V2 §6.9); parts map one-to-one onto R1-BND-REC2's vehicle-part
section; static merging is legacy's measured win (CL9) and works on both backends. Bundles help static draws on WebGPU
only and give nothing on WebGL (F12), so they are a WebGPU-only extra: under R3-GL2-REC1 they enter only with a named
fallback (merging) and a measured gain, which no source read gives. BatchedMesh does not lower GPU draw commands on
WebGPU (F7), so it is judged on CPU time, not on `drawCalls`.
Rests on: R3-CHAR-F7, F12; R3-GL2-REC1 (cited); legacy CL9, GC16; wave 1 R1-BND-REC2.
Serves: VISION_V2 §6.4 (big, multi-part vehicles), §6.9.
Cross-topic open questions: R3-XT-OQ2 (Open questions, "Cross-topic open questions").
Spike must prove (proposed): the stress scene's four [very large units](GLOSSARY.md#big-vehicle-and-very-large-unit) ([GLOSSARY](GLOSSARY.md#the-stress-scene); each at least 20 parts) on screen with its soldiers add at most 2.0 ms (p99) GPU time and at most 200 draw calls against the scene without
them, before any outline or ambient-occlusion pre-pass, on the recommended machine at High (4 units × 20 parts × 2
passes, shadow and main, is 160 draws, plus margin; a hull outline adds about 80 more for the same parts,
R3-BUDGET-REC2, second pass 2026-10-29);
render bundles stay only if they cut the WebGPU backend's CPU frame time in the benchmark scene.

### Question 5: budgets for the minimum and recommended hardware (R3-BUDGET)

**R3-BUDGET-REC1. Reference machines (proposed, M-A with M-B as the fallback): the minimum is one named laptop with
integrated graphics of Intel Gen12 (Iris Xe) class or the AMD integrated equivalent, a laptop CPU of at least 4 cores
and 8 threads from the same generation, a 1920 × 1080 screen, Windows 11, run on Chrome and Firefox (both on WebGPU
there) and with the WebGL 2 backend forced (`forceWebGL`), plus a run of the same laptop under Linux for the real
fallback (Firefox there has no WebGPU); the recommended is one named desktop with a mid-range discrete card (one or two
generations old) and its CPU named, at 1920 × 1080 and 2560 × 1440. Mustafa's RTX 5080 is a headroom and Brave check,
never the recommended reference.**
Reason: VISION_V2 §4 names classes, not models, and no allow-listed source ranks GPUs or CPUs (F7), so the models have
to be picked and named in the ADR. On Windows both Chrome and Firefox run WebGPU (R3-WGPU-F17, F18), so a Windows
machine reaches the WebGL 2 backend only by forcing it; the players who really fall back are on Firefox on Linux and on
Intel Macs (R3-GL2-F12). Intel Gen12+ is the GPU class Chrome needs for WebGPU on Linux (F4: it says nothing about
Windows), so an Intel Gen12 laptop booted into Linux covers both Chrome's WebGPU path there and Firefox's real
fallback, the hardest WebGL 2 case (no `WEBGL_multi_draw`, R3-GL2-REC2); an AMD laptop would fall back in Chrome on
Linux as well. R1-TICK-OQ6 asks R3 for a named minimum laptop CPU; the 4-core, 8-thread floor is a proposal with no
source (the sim's Worker and the page's main thread each want a core). The RTX 5080 is far above mid-range (legacy
phase-0 report). R1 asks for the same machines (R1-TICK's P2).
Rests on: R3-BUDGET-F4, F7; R3-WGPU-F17, F18; R3-GL2-F7, F8, F12; legacy phase-0 report; wave 1 R1-TICK-OQ6 (cited).
Serves: VISION_V2 §4. No VISION_V2 §6 item: it serves the §4 targets only.
Spike must prove (proposed): the two machines are named in the ADR draft with their GPU, CPU, RAM, screen and
`devicePixelRatio`, and every R3 pass mark is reported on both; on the minimum laptop's Linux run the overlay
(R3-WGPU-REC4) names the WebGL 2 backend in Firefox.

**R3-BUDGET-REC2. Budgets (proposed, option B-2), measured in the stress scene's fight on screen ([GLOSSARY](GLOSSARY.md#the-stress-scene)): the largest mode's [N](GLOSSARY.md#player-count-n) soldiers
in one fight, its 4 very large units and its 300 live projectiles with their [visual effects](GLOSSARY.md#active-effect-and-visual-effect), on the reference machines.**

| Budget (all passes of one frame) | Minimum laptop, Low, 30+ fps | Recommended card, High, 60 fps |
|---|---|---|
| Frame time, p95 | ≤ 33.3 ms | ≤ 16.7 ms |
| Frame time, median | ≤ 28 ms | ≤ 14 ms |
| Main-thread CPU per frame (render submission, interpolation, animation, UI), p99 | ≤ 16 ms | ≤ 8 ms (inside F1's 10 ms) |
| of which: character animation (posing plus bone upload, with R3-CHAR-REC2's budget LOD), p99 | ≤ 1.0 ms | ≤ 2.0 ms (more soldiers on the full rig) |
| of which: sim cost seen by the page ([the client's sim share](GLOSSARY.md#the-clients-sim-share); R1-TICK, R1-BND), p99 | ≤ 1.0 ms | ≤ 1.0 ms |
| GPU time, total, p99 | ≤ 25 ms | ≤ 12 ms |
| of which: shadow pass, p99 | ≤ 3 ms (one map, 1024² or off) | ≤ 2 ms |
| of which: main scene pass, p99 | ≤ 16 ms | ≤ 6 ms |
| of which: post-processing (whatever the look brief keeps), p99 | ≤ 4 ms | ≤ 3 ms |
| of which: effects, sprites, HUD in the 3D view, p99 | ≤ 2 ms | ≤ 1 ms |
| Draw calls (`renderer.info.render.drawCalls`) | ≤ 800 without a per-object outline pass at Low; about ≤ 1,100 with one (see the arithmetic below) | ≤ 1,500 |
| Triangles (`renderer.info.render.triangles`) | ≤ 500k | ≤ 1.5M |
| GPU memory (`renderer.info.memory.total`) | ≤ 512 MB | ≤ 1 GB |
| Render resolution | `devicePixelRatio` capped at 1, scene pass resolution scale 0.75–1 | ratio capped at 2 |
| JS heap after 10 minutes | ≤ 150 MB, no growth over the last 5 minutes | same |

Reason: frame times follow VISION_V2 §4 directly; the median marks leave room for spikes. The CPU split keeps the
page inside web.dev's 10 ms at 60 Hz (F1) and gives R1's sim mark its 1.0 ms. Draw calls are set well below legacy's
5,843 at 48 (TL15) and a few times above its village 202–303, which legacy already judged a medium risk on weak
machines (AUDIT P5). The arithmetic behind the Low cap (proposed figures): soldiers at R3-CHAR-REC1's cap of 4 draws
per soldier per pass give 192 per pass at 48; a shadow and a main pass make 384; the merged world about 202 (CL9; the
unmerged village was 303, AUDIT §1); four very large units up to 200 (R3-CHAR-REC4): about 786, inside 800, at N = 48 (at N = 54, MODES-3 (b),
the soldiers alone take 54 × 4 × 2 = 432 draws and the sum is about 834, over 800: R3-BUDGET-OQ4). A hull outline
pass adds one draw per outlined object (R3-LOOK-F5): 192 more for the soldiers and about 80 for the very large units' parts,
about 1,060 in all. So the 800 cap holds only without a per-object outline pass at Low; if the look brief keeps a hull
outline at Low, the Low cap becomes about 1,100 (proposed), or the hull is a High-only setting. A GTAO pre-pass
(R3-LOOK-REC2's other planned extra) adds about one draw per drawn object as well: about 192 for the soldiers and 80
for the very large units' parts, plus the world's main-pass share, so it passes both 800 and 1,100 at Low; whether Low keeps
it is the same trade (second pass, 2026-10-29). Both counts are conservative, since the world's 202 already includes
the world's shadow pass and its own hulls (CS12 gives 398 as the count just before the merge). That trade is the brief's, measured by
R3-LOOK-REC3; R3 does not choose it. The CPU-side animation row (1.0 ms) applies with the budget
LOD; R3-CHAR-REC1's 2.0 ms mark is the cost with every soldier on the full rig, which is why the budget LOD exists. The
resolution cap follows F3 and CD15 (ratio 2 shades four times the pixels). Memory follows the
render-target arithmetic (F5): at 1080p a half-float colour target is 16.6 MB, so a post chain with MSAA, two MRT outputs and bloom can reach
about 100 MB or more (an estimate, not a measurement). The triangle and memory figures are the loosest: no source gives them (F7), so the spike either confirms or
moves them.
Rests on: R3-BUDGET-F1, F2, F3, F5, F6, F7; R3-LOOK-F5, F14; R3-CHAR-REC1, R3-CHAR-REC4 (cited for the arithmetic);
legacy AUDIT_BUGS_PERF §1, CL9, TL15, GA15, CD15, CD17; wave 1 R1-TICK's scene and client mark.
Serves: VISION_V2 §3 (the largest mode), §4 (both targets), §6.4 (large units), §6.9 (the spectacle budget).
Spike must prove (proposed): every row of the table holds on its machine in the benchmark scene (R3-MEASURE-REC2) for 3
runs of 600 measured frames after 300 warm-up frames, on Chrome on both machines and on Firefox on the minimum machine,
on the WebGPU backend and, on the minimum machine, also on the WebGL 2 backend.

**R3-BUDGET-REC3. Quality tiers are data: one table holds, per tier (Low, Medium, High), the pixel-ratio cap, the scene
resolution scale, MSAA samples, shadow map size and distance, each post effect's on/off and resolution scale, the
animation budget (R3-CHAR-REC2) and the effect caps (FX pools); Low is the default when the adapter is integrated or
the backend is WebGL 2. Dynamic resolution (B-3) is a measured option, not the start.**
Reason: the budgets differ by tier (REC2) and every look technique has its own cost knob (R3-LOOK-F10, F13;
R3-WGPU-F13); legacy holds its effect caps and culls as tuning data already (GP1: scuffs capped at 40; GN8: effects
culled beyond 50 m, sound beyond 90 m). Which effects exist at all is the look brief's call; the tier table only says
how much each may cost.
Rests on: R3-BUDGET-F3; R3-LOOK-F10, F13; R3-WGPU-F13, F15; legacy GP1, GN8, CD15.
Serves: VISION_V2 §4 (Low and High), §6.9.
Spike must prove (proposed): switching from High to Low on the minimum machine cuts GPU frame time by at least 50 % in
the benchmark scene, with no shader compile during the switch after warm-up (R3-WGPU-REC3).

### Question 6: how we measure it (R3-MEASURE)

**R3-MEASURE-REC1. One client measurement harness: per frame it records CPU frame time from `performance.now()`, GPU
time from timestamp queries where the backend has them (T-A, then T-B, else "not available"), and the `renderer.info`
counters (reset manually once per frame); it reports median, p95, p99, max and the count of frames over budget for 3 runs
of 600 frames after 300 warm-up frames, each report labelled with backend, adapter, browser version, `devicePixelRatio`,
resolution and whether GPU times are quantized.**
Reason: three.js exposes GPU timing and counters (F3, F4) but GPU timing is missing on some backends and browsers (F4,
F6) and quantized in Chrome (F2); CPU timers are fine for frame times (F7: 100 µs against a 16.7 ms frame). The
percentiles and warm-up repeat legacy's perfbots method (TL15).
Rests on: R3-MEASURE-F2, F3, F4, F6, F7; legacy TL15, GP5.
Serves: VISION_V2 §4 (the 30 and 60 fps targets need a method), §6.9.
Spike must prove (proposed): on the recommended machine, the harness's GPU total per frame agrees with the CPU frame
time minus CPU work within 2 ms median when the scene is GPU-bound; two back-to-back runs of the same build give medians
within 3 % of each other.

**R3-MEASURE-REC2. A fixed benchmark scene: a recorded match replayed from its inputs (R1's identical-results work: R1-DET-REC1, R1-DET-REC6) at the
stress scene ([GLOSSARY](GLOSSARY.md#the-stress-scene); R3-BUDGET-REC2's fight on screen), with a scripted
camera path, started from a URL flag on any build, including [preview links](GLOSSARY.md#preview) (R5).**
Reason: numbers compare only if every run draws the same frames; legacy's clustered worst case (TL15) and the stress scene
([GLOSSARY](GLOSSARY.md#the-stress-scene)) give the load; preview links make it runnable on any machine with a browser.
Rests on: R3-MEASURE-F3; legacy TL15; R3-BUDGET-REC2 (cited); wave 1 R1-TICK's spike scene, R1-DET-REC1 and
R1-DET-REC6 (cited).
Serves: VISION_V2 §3, §4, §6.9.
Spike must prove (proposed): two runs on the same machine produce identical `renderer.info.render.drawCalls` and
`triangles` on every frame, and identical screenshots at three fixed frames.

**R3-MEASURE-REC3. Split by where it runs: CI (W-A) gates only hardware-independent numbers (zero console errors, draw
calls, triangles, programs, GPU memory totals, zero per-frame allocation in the render path); frame times and GPU times
come only from the reference machines (W-C), recorded per run in a results log; a headless frame rate is never reported
as a hardware result.**
Reason: headless Chrome renders on SwiftShader by default and is "at reduced performance" even with a GPU (F8); GPU
runners are a paid cloud GPU, not a laptop (F9); legacy already treats SwiftShader timings as not counting (TL9, TL12).
Rests on: R3-MEASURE-F3, F8, F9; legacy TL9, TL11, TL12.
Serves: VISION_V2 §4. No VISION_V2 §6 item: it serves the §4 targets only.
Spike must prove (proposed): the CI job fails when a change adds 10 % or more draw calls or 10 % or more GPU memory in
the benchmark scene (a planted regression is caught in 1 of 1 tries), and runs in under 5 minutes.

**R3-MEASURE-REC4. Port legacy's perf overlay to v2 (GP5): backend and adapter, frame time median and p95, draw calls,
triangles, programs, GPU memory, GPU time when available, zero cost while closed; developers profile per pass with
Chrome's developer flag on the reference machines (F2), never asking players to set it.**
Reason: GP5's technique already works and the inventory marks it to keep; per-pass GPU times need finer than 100 µs
(F2); program counts expose compile stalls (F10).
Rests on: R3-MEASURE-F2, F3, F10; legacy GP5.
Serves: VISION_V2 §4, §6.9.
Spike must prove (proposed): with the overlay closed, the benchmark scene's median frame time is within 1 % of a build
without the overlay; with it open, within 3 %.

### Question 7: spatial audio with Web Audio (R3-AUDIO)

**R3-AUDIO-REC1. One `AudioListener` at the local player's position, oriented with the camera, as long as the camera is
legacy-like (top-down and high; the v2 camera is look brief pending, and AUDIO-OQ3 re-decides the placement with it);
sources as `PannerNode`s
with `equalpower` by default (P-B), the distance model and its numbers as per-weapon data (LOOP_V2 §7 feel kit); HRTF
only as a measured option for a few priority voices (P-D).**
Reason: a panner against one listener replaces legacy's split (gain from the player, pan from the camera, GP6) with one
model (F1, F3); equal power is the efficient model (F2), while three's `PositionalAudio` silently picks HRTF (F5) and
no source gives HRTF's cost (F9); a listener at the camera flattens distance under a top-down view (F11), which is the
only reason for the player position, so the placement holds only for such a camera (AUDIO-OQ3).
Rests on: R3-AUDIO-F1, F2, F3, F5, F6, F9, F11; legacy GP6.
Serves: the brief's R3 row ("spatial audio with Web Audio", `docs/V2_PHASE0_BRIEF.md` §4); VISION_V2 §3 (the largest
mode, 12v12v12 or 24v24), §6.9 (large units and their effects). No VISION_V2 §6 item names audio.
Spike must prove (proposed): with 32 concurrent equal-power voices, a 5-minute recorded capture has no dropout (checked by
a listener and by the AudioContext staying in the "running" state) and main-thread audio work stays under
0.5 ms (p99) per frame on the minimum machine; HRTF for 4 priority voices raises that p99 by less than 0.3 ms.

**R3-AUDIO-REC2. A voice manager with a cap and priorities (V-B) plus distance culling: proposed start of 32 voices on
Low and 48 on High, priorities as data (own weapon and shots at the player first, then large units, then by distance),
per-kind throttles as legacy's 55 ms impact throttle, pooled nodes and pre-built noise buffers.**
Reason: a playing source costs its nodes whatever its gain until the game stops it, and inverse and exponential gains
never reach zero anyway (F4), so no distance curve bounds the voice count: without a cap every shot of a fight at the largest
mode's N (48 or 54, MODES-3) inside the cull radius keeps a voice; legacy has no cap (GP6) and only a 90 m cull (GN8); it also builds a buffer per shot (GP6). Proposed
numbers are starting points; no source gives a voice budget (F9).
Rests on: R3-AUDIO-F3, F4, F9; legacy GP6, GN8.
Serves: the brief's R3 row (spatial audio); VISION_V2 §3, §6.9. No VISION_V2 §6 item names audio.
Spike must prove (proposed): in the benchmark fight (R3-MEASURE-REC2), the count of live source nodes never exceeds the
cap, shots at the player are never dropped (0 of all such events in 3 runs), and audio code allocates no new
AudioBuffer after warm-up.

**R3-AUDIO-REC3. Occlusion starts as option O-C (zones from map data) or none; O-B (rays plus a lowpass, F8) only if the
spike shows players cannot tell sounds behind walls, and then at a low ray rate per voice, height-aware.**
Reason: no source read describes occlusion or its cost (F8, F9); legacy has none (GP6); the inventory asks "whether
occlusion is worth its cost". Rays against static geometry are cheap per voice but need floors and heights, the
simulation's data for VISION_V2 §6.2, which R4's line-of-sight work owns; the rays would reuse it.
Rests on: R3-AUDIO-F8, F9; legacy GP6, GW15.
Serves: the brief's R3 row (spatial audio: occlusion). No VISION_V2 §6 item names audio; §6.2 (floors and line of sight)
is the simulation data the rays would reuse, not a need this recommendation serves.
Cross-topic open questions: R3-XT-OQ6 (Open questions, "Cross-topic open questions").
Spike must prove (proposed): with O-B on, at most 1 ray per voice per 100 ms and at most 0.2 ms (p99) per frame for all rays at
the voice cap on the minimum machine; a blind listening test where 4 of 5 testers tell an occluded source from an open
one.

**R3-AUDIO-REC4. Sounds are triggered on the page from the per-tick event section (R1-BND-REC2) and from interpolated
positions (R1-TICK-REC5), never from the sim directly; the AudioContext starts suspended and resumes on the first user
input (F10), as legacy does.**
Reason: the sim runs in a Worker (R1-TICK-REC5), Web Audio plays on the page's context, and autoplay rules gate the
start (F10); legacy's first-input start (GP6) already works.
Rests on: R3-AUDIO-F10; legacy GP6; wave 1 R1-BND-REC2, R1-TICK-REC5 (cited).
Serves: the brief's R3 row (spatial audio); VISION_V2 §4 (all four browsers), §6.9 (large units and their effects heard
in step with what is drawn). No VISION_V2 §6 item names audio.
Spike must prove (proposed): sound onset trails the drawn event by at most one rendered frame plus the output
latency, measured on 100 shots; zero autoplay warnings in the console on all four browsers.

## Open questions

### Question 1: three.js WebGPURenderer with TSL (R3-WGPU)

- **R3-WGPU-OQ1. Brave on Windows.** Mustafa plays in Brave (CLAUDE.md), and his screenshots are the final visual gate.
  No allow-listed source says whether Brave enables WebGPU by default (MDN's data covers Chrome, Edge, Firefox, Safari,
  Opera). Decided by: a check on his machine during the spike (`navigator.gpu`, adapter info, which backend the overlay
  names).
- **R3-WGPU-OQ2. Firefox and Safari "partial".** MDN marks Firefox "partial_implementation" (F18) and caniuse marks
  Safari desktop "Partial support" (F21) without saying what is missing in the text the fetcher saw. Decided by: the
  spike's benchmark scene on both browsers, with `renderer.hasFeature` checks for the features R3 relies on
  (timestamp queries, R3-MEASURE).
- **R3-WGPU-OQ3. WebGPURenderer against WebGLRenderer on our scene.** F7 allows that WebGLRenderer can be faster.
  Decided by: R3-WGPU-REC1 mark (3).
- **R3-WGPU-OQ4. Colour output.** RenderPipeline tone-maps and converts colour space by default (F12); the internal
  buffer is half float (F14). Legacy's "authored hex = on-screen colour" rule (CD15) is the locked legacy look only.
  Which output transform v2 uses is for the look brief; technically both are reachable (`outputColorTransform`).
- **R3-WGPU-OQ5. Which release to pin.** r186 is current (F1); the last four releases came 55, 76 and 85 days apart
  (F1's dates, my arithmetic: about every two to three months).
  Decided by: the release current when the spike starts.

### Question 2: the WebGL 2 fallback (R3-GL2)

- **R3-GL2-OQ1. Does three.js r186 request a WebGPU compatibility-mode adapter, or can it?** The fetched sources say
  nothing about `featureLevel` in three.js. Decided by: reading three's WebGPUBackend device request (a source-reading
  task, not a spike) and a spike on a machine without core WebGPU.
- **R3-GL2-OQ2. How many target players fall back?** F13's 31 % is about Direct3D 11.1 among Chrome users on Windows,
  not about WebGPU adapters; F21 of R3-WGPU (caniuse, 87.82 %) is an aggregate. No source gives the share of
  integrated-graphics laptops without a WebGPU adapter. Decided by: client telemetry of the backend after launch (R6's
  telemetry), and the spike's minimum machine.
- **R3-GL2-OQ3. Visual differences between backends.** No fetched source lists where the WebGL 2 backend renders
  differently (blending, precision); the maintainer's forum reply that suggests it is off the allow list. Decided by:
  R3-WGPU-REC1 mark 2.
- **R3-GL2-OQ4. Transform-feedback compute cost.** Whether TSL compute on the fallback is fast enough for any effect is
  unmeasured. Decided by: a spike only if REC1's F-B route is taken.
- **R3-GL2-OQ5. "WebGPU required, no fallback" as a reference point.** The options table weighs four ways to keep a
  fallback and no row for dropping it; dropping it would shut out the platform groups of F12, whose share is unknown
  (OQ2). Not researched in this pass (a verifier finding, C7). Decided by: the rendering ADR draft (`docs/adr/0003`)
  and Mustafa, with OQ2's share once telemetry exists.

### Question 3: candidate techniques for a comic look (R3-LOOK)

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
- **R3-LOOK-OQ6. Anti-aliasing.** MSAA is the renderer's built-in option (R3-WGPU-F15) and multiplies the multisampled
  MRT attachments' memory (F14); post-process anti-aliasing (FXAA, which needs sRGB input, R3-WGPU-F12; SMAA, named
  here as a candidate only: no finding or source read covers it; TRAA,
  F10, with its ghosting) is the alternative, and screen-space outlines alias (options table). These were not
  researched as options in this pass (a verifier finding, C7). Decided by: a reading of three's anti-aliasing nodes,
  then the look-test scene (REC1), only for the techniques the brief keeps.

### Question 4: characters (skinning, instancing, LOD) (R3-CHAR)

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
- **R3-CHAR-OQ6. GPU-driven culling and indirect draws.** Not researched in this pass (a verifier finding, C7). On the
  fallback, indirect dispatch counts are refused (R3-GL2-F2), so either would likely be a WebGPU-only extra under
  R3-GL2-REC1 (inferred: F2 covers compute dispatch counts only, not indirect draws). Decided by: a source reading and a spike, only if the draw-call budget (R3-BUDGET-REC2) is still missed
  after REC1 to REC4.

### Question 5: budgets for the minimum and recommended hardware (R3-BUDGET)

- **R3-BUDGET-OQ1. Which models.** No allow-listed source ranks GPUs or gives the players' hardware mix (F7). Decided
  by: Mustafa and the Advisor (what can be bought or borrowed), recorded in the ADR draft; R6's telemetry after launch.
- **R3-BUDGET-OQ2. High-refresh screens.** At 120 or 144 Hz (F2) the render loop runs more often than 60 fps; whether
  High caps the frame rate at 60 or runs free is open. Decided by: the ADR (a cap saves power on laptops).
- **R3-BUDGET-OQ3. HiDPI laptops on Low.** Many integrated-graphics laptops ship with ratios above 1 (F3); a cap at 1
  softens text in the 3D view. Decided by: the spike's GPU time at ratio 1 and 2 (CD15's request) and Mustafa's look.
- **R3-BUDGET-OQ4. Largest N.** VISION_V2 §3 starts small ("two teams with 2–8 players, and three teams with 6–18 players") and names 12v12v12 or 24v24 as the goal (36 or 48 players; CLAUDE.md: design for 48+); R1-TICK's
  OQ7 decides the N of the offline scale test. The draw-call and triangle rows scale with N. The largest mode's
  N ([GLOSSARY](GLOSSARY.md#the-largest-mode-and-its-n)) is 48 or 54 by the answer to MODES-3 (below).
- **R3-BUDGET-OQ5. Triangle and memory rows.** They have no source (F7). Decided by: the spike's measurement and the
  character rework's real models (R3-CHAR-OQ3).
- **R3-BUDGET-OQ6. Shadow technique.** REC2 budgets a shadow pass ("one map, 1024² or off" on Low), but one shadow map
  against cascades was not researched as an option in this pass (a verifier finding, C7); legacy uses one 2048² map
  over a 100 m frustum (CD17), and r186 removed `PCFSoftShadowMap` from WebGPURenderer (R3-WGPU-F3). Decided by: a
  reading of three's WebGPU shadow options, then the shadow-pass row of REC2 in the spike.

### Question 6: how we measure it (R3-MEASURE)

- **R3-MEASURE-OQ1. Timestamp-query support in Firefox and Safari.** No source read says whether their WebGPU exposes
  `timestamp-query`. Decided by: `renderer.hasFeature` on both in the spike.
- **R3-MEASURE-OQ2. Self-hosted reference machines in CI.** Whether the reference machines become self-hosted runners
  (R5, R6) or are run by hand is a cost and security question. Decided by: R5 and R6.
- **R3-MEASURE-OQ3. GPU memory on integrated graphics.** WebGPU gives no memory budget; `renderer.info.memory` counts
  what three.js allocated (F3), and the adapter's `memoryHeaps` appear only with Chrome's developer flag (F2). Decided
  by: the spike reads both on the minimum machine.
- **R3-MEASURE-OQ4. Cross-origin isolation for profiling.** Isolation gives 5 µs CPU timers (F7) but not finer GPU timers
  (F1); R1-BND-REC5 allows it in development and previews (the [isolation flag](GLOSSARY.md#preview-isolation-and-the-isolation-flag)). Decided by: the R1-BND-REC5 route already recommended; no
  new need from R3.

### Question 7: spatial audio with Web Audio (R3-AUDIO)

- **R3-AUDIO-OQ1. HRTF cost.** No allow-listed source gives it (F9); the Chromium and W3C mailing-list discussions found
  by search are off the allow list. Decided by: REC1's spike mark.
- **R3-AUDIO-OQ2. Voice counts in a real fight.** The inventory asks for a count of concurrent voices in a 48-bot fight
  (GP6; for v2, a fight at the largest mode's N, 48 or 54, MODES-3); no measurement exists. Decided by: the spike's counter under REC2.
- **R3-AUDIO-OQ3. Listener placement.** F11 assumes a legacy-like top-down camera; the camera is look brief pending.
  Decided by: the look brief's camera, then a listening check by Mustafa.
- **R3-AUDIO-OQ4. Spec details.** The specification's PannerNode and spatialization sections were not reached (F9);
  MDN's pages stood in. Decided by: a verifier read of https://www.w3.org/TR/webaudio-1.1/ in parts.
- **R3-AUDIO-OQ5. Occlusion ownership.** Rays across floors depend on R4's line-of-sight data. Decided by: R4 and the
  synthesis.
- **R3-AUDIO-OQ6. An AudioWorklet mixer or spatializer of our own.** Not researched in this pass (a verifier finding,
  C7); it would replace built-in panners (F1, F2) with our own code. Decided by: REC1's spike, only if the built-in
  panners miss its CPU mark.

### Shared open questions (the same words in every topic)

- **MODES-3 (Mustafa). How are VISION_V2 §3's starting sizes read?** VISION_V2 §3 says "two teams with 2–8 players,
  and three teams with 6–18 players" to start, and "The goal: 12v12v12 or 24v24". (a) Totals: 1v1 to 4v4, and 2v2v2 to
  6v6v6; the largest mode is then the goal 24v24, N = 48. (b) Per team: 2v2 to 8v8, and 6v6v6 to 18v18v18; the largest
  mode is then 18v18v18, N = 54. The answer sets the largest mode's N
  ([GLOSSARY](GLOSSARY.md#the-largest-mode-and-its-n)), which [the stress scene](GLOSSARY.md#the-stress-scene), the
  offline scale test (R1-TICK-OQ7), the soak runs (R5-TEST-REC5) and the cost rows (R6-COST) use, and items 3 and 4 of
  the test-mode list ([GLOSSARY](GLOSSARY.md#the-test-mode-list)). Until it is answered, a mark that needs the largest
  mode's N gives both values. Also asked as R2-BUD-OQ6, R2-XU-OQ13 and R3-BUDGET-OQ4. Decided by: Mustafa.

### Cross-topic open questions (consistency critic, 2026-10-29)

- **R3-XT-OQ1. Where does the remote-entity interpolation buffer live: in the worker or on the page?** A two-block
  buffer on the page cannot hold R2's 100 to 167 ms delay. So either the worker (Rust) runs R2's adaptive snapshot
  buffer and publishes already-delayed remote poses, or the page (TypeScript) keeps a snapshot buffer, as R5's test
  layer assumes. No topic places R2's interpolation and clock estimate on one side of R1's boundary. Side A:
  R1-TICK-REC5; R1-BND-REC2; R1-BND-REC6. Side B: R2-SYNC-REC4; R5-TEST-REC1. Existing questions: none. Decided by:
  ADR-0001 (interpolation design) with ADR-0004 (boundary: block contents and the number of blocks kept). (Consistency
  critic, 2026-10-29, rank 4.)
- **R3-XT-OQ2. Do the snapshot and the output block carry remote soldiers' animation state?** Animation of remote
  soldiers online needs those states in R2's snapshot, then in R1's block. R2's bit budget names none of them and R1
  leaves the layout open, so R3's character path and R2's bandwidth figure may not fit together. Side A: R1-BND-REC2.
  Side B: none by id. Also named: R3-CHAR-REC3, R3-CHAR-REC4, R1-BND-OQ6, R2 worked estimate. Existing questions:
  R1-BND-OQ6 (block size and layout), partly. Decided by: ADR-0001 (snapshot fields) with ADR-0004 (block layout) and
  ADR-0003 (what animation reads). (Consistency critic, 2026-10-29, rank 16.)
- **R3-XT-OQ3. Does the editor still open by double-click with a Wasm validator?** R1 hands this question to R5, and
  R5's editor build does not address the double-click rule EDITOR_V2 keeps. Side A: R1-DATA-OQ11. Side B: missing
  answer (R5-LAYOUT-REC5, R3-WGPU-REC4 named). Existing questions: R1-DATA-OQ11. Decided by: ADR-0002 (editor build),
  or Mustafa if EDITOR_V2 §5's double-click rule must change. (Consistency critic, 2026-10-29, rank 25.)
- **R3-XT-OQ4. Who creates the client's field data (backend, hardware, frame time)?** Each topic hands client-side
  field data (backend share, hardware mix, frame time) to the other. No recommendation creates it, yet R3's
  fallback-share and hardware questions depend on it. Side A: R3-GL2-OQ2. Side B: R6-TEL-REC1; R6-OBS-OQ5. Also named:
  R3-BUDGET-OQ1. Existing questions: R3-GL2-OQ2, R3-BUDGET-OQ1, R6-OBS-OQ5. Decided by: The synthesis: ADR-0003 (what
  is measured) and ADR-0006 (where it is stored, under R6-OBS-REC4's data rules). (Consistency critic, 2026-10-29,
  rank 26.)
- **R3-XT-OQ5. Who sets the CI runner costs (GPU, macOS, reference machines, cache)?** Real-Safari runs, WebGPU
  coverage in CI and reference-machine runs all depend on costs no topic sets. Side A: R5-CI-OQ5; R5-TEST-OQ4;
  R3-MEASURE-OQ2. Side B: missing answer. Also named: R5-XQ-OQ1, R6-COST-REC1. Existing questions: R5-CI-OQ5,
  R5-TEST-OQ4, R5-XQ-OQ1, R3-MEASURE-OQ2. Decided by: ADR-0006 (costs) with ADR-0002 (CI); Mustafa for any paid plan.
  (Consistency critic, 2026-10-29, rank 33.)
- **R3-XT-OQ6. Who owns audio occlusion rays, if option O-B is chosen?** Page-side audio rays would need the sim's
  query across a boundary that allows no page calls. This only matters if option O-B is adopted. Side A:
  R3-AUDIO-REC3; R3-AUDIO-OQ5. Side B: missing answer (R4-LOS-REC1 named). Also named: R1-BND-REC1. Existing
  questions: R3-AUDIO-OQ5. Decided by: ADR-0003 with ADR-0004, only if R3-AUDIO-REC3's spike asks for O-B.
  (Consistency critic, 2026-10-29, rank 35.)
- **R3-XT-OQ7. Does R3 need OffscreenCanvas, or a per-entity or per-field block layout?** R1 asks R3 for two inputs to
  the block layout, and R3 gives neither. Side A: R1-BND-OQ6. Side B: missing answer. Also named: R1 pointer to R3.
  Existing questions: R1-BND-OQ6. Decided by: ADR-0003 with ADR-0004 (block layout), measured in the R1-BND-REC2
  spike. (Consistency critic, 2026-10-29, rank 37.)
- **R3-XT-OQ8. Does the client's cutaway follow the server's visibility rule?** This is a pointer R3 does not take up.
  It stays look brief pending and depends on R2-INT-OQ1. Side A: R2-INT-REC2. Side B: R3-WGPU-REC3. Also named:
  R2-INT-OQ1. Existing questions: R2-INT-OQ1. Decided by: The look brief and Mustafa (R2-INT-OQ1), then ADR-0003.
  (Consistency critic, 2026-10-29, rank 39.)

### Pointers to other topics

#### Question 1 (R3-WGPU)

- R5 (toolchain): the TypeScript client imports three's WebGPU build; R3-WGPU-REC2's pinned version is a lockfile
  entry, and the upgrade check is a CI job (R3-MEASURE).
- R1-TICK-REC5: the main thread renders every frame; `setAnimationLoop` (F9) is the render clock, consistent with
  R1-TICK-REC5's "`requestAnimationFrame` is the render clock only".

#### Question 2 (R3-GL2)

- R6 (telemetry): the backend in use and the adapter info are worth one field in the client's session report
  (OQ2).
- R5 (CI): headless runs can force the WebGL 2 backend; neither backend's frame times count from headless (R3-MEASURE).

#### Question 3 (R3-LOOK)

- The look brief (Mustafa and the Advisor) receives R3-LOOK-REC1's table; R3 picks nothing.
- CLAUDE.md's legacy constraint "no post-processing (under review)" applies to legacy only; v2's post chain waits for
  the ADR on rendering (`docs/adr/0003`).

#### Question 4 (R3-CHAR)

- R1 (boundary): REC3 and REC4 assume R1-BND-REC2's per-kind sections carry aim yaw and pitch, ADS, reload, throw and
  draw state per soldier (GA15 lists them); R1-XU-OQ3 (shared memory or a transferred buffer) does not change the
  renderer side: it reads typed arrays either way.
- R2: R2-INT-REC1's relevance region caps the soldiers the client can draw; R2-SYNC-REC4's interpolation delay sets
  how old the drawn pose is (presentation only).

#### Question 5 (R3-BUDGET)

- R1: REC2's scene is [the stress scene](GLOSSARY.md#the-stress-scene), which R1-TICK's spike also runs, so the client and the sim are measured on the same load; REC1's machines
  are the ones R1 asks for.
- R6: the hardware mix and backend share of real players come from telemetry (OQ1).

#### Question 6 (R3-MEASURE)

- R5: REC3 is a gate v2 check (counts, not timings); REC2's URL flag runs on preview links.
- R1: REC2 relies on R1's identical-results replay (R1-DET-REC1, R1-DET-REC6); if replay is not identical on the client, the benchmark falls back
  to a scripted bot match with a fixed seed (less exact).
- R6: results logs could live with other telemetry.

#### Question 7 (R3-AUDIO)

- R4: occlusion rays (REC3) want the same static geometry and floor data as line of sight.
- LOOP_V2 §7: distance curves and voice priorities sit in the per-weapon feel-kit data.

### Recommendations that hinge on a source not read

| Recommendation | Source | Reason not read |
|---|---|---|
| R3-WGPU-REC1 (mark 3) | discourse.threejs.org thread 87805 (a maintainer on forceWebGL against WebGLRenderer; a search lead) | outside the allow list |
| R3-WGPU-REC4 | Brave's own WebGPU documentation (brave.com) | outside the allow list (OQ1) |
| R3-GL2-REC1 | Khronos WebGL 2.0 specification (registry.khronos.org; that WebGL 2 has no compute stage) | outside the allow list (the three.js source, F2, shows the emulation instead) |
| R3-GL2-REC3 | Khronos `EXT_disjoint_timer_query_webgl2` registry page | outside the allow list (MDN read instead, F6) |
| R3-LOOK-REC1 | Activision, "Practical Real-Time Strategies for Accurate Indirect Occlusion" (GTAO; activision.com) | outside the allow list |
| R3-LOOK-REC3 | Guilty Gear Xrd's art-style talk (GDC 2015, GDC Vault) | video only (GDC Vault redirects outside the allow list) |
| R3-LOOK-REC3 | Valve, "Illustrative Rendering in Team Fortress 2" (NPAR 2007; valvesoftware.com) | outside the allow list |
| R3-CHAR-REC1 | three.js SkinningNode source and docs (r186) | HTTP 404 at the paths tried (OQ1) |
| R3-BUDGET-REC1 | Steam Hardware & Software Survey (store.steampowered.com) | outside the allow list |
| R3-BUDGET-REC1 | Intel, AMD and NVIDIA product specification pages | outside the allow list |
| R3-MEASURE-REC3 | GitHub's runner hardware table ("supported runners and hardware resources") | linked page not fetched; GPU details not needed for the recommendation |
| R3-AUDIO-REC1 | Web Audio API specification, PannerNode and §6 spatialization (webaudio.github.io, w3.org/TR/webaudio-1.1) | fetched text truncated before those sections (MDN read instead) |
| R3-AUDIO-REC1 | W3C public-audio list, 2014 (HRTF cost; lists.w3.org) | outside the allow list |
| R3-AUDIO-REC1 | Chromium code review 652073003 (equal-power default; codereview.chromium.org) | outside the allow list |

## Verifier spot-check

One verifier pass ran on 2026-10-28 (wave 2 of `.checkpoints/v2-research/plan.md`), and the owner's fix pass
(graphics-engineer, the same day) applied it. Verdict: PASS WITH NOTES. The verifier checked 44 key claims at their
sources (WebFetch, allow-listed domains, 39 fetches) and found none wrong or unsupported; its critical read found four
problems in the recommendations that were more than nits (L1, L2, C1, C2). Its report, verbatim:
`.checkpoints/v2-research/verifier-r3-result.md`; its notes with every quote in full:
`.checkpoints/v2-research/verifier-r3.md`; the fix log: `.checkpoints/v2-research/graphics-engineer-r3-fix.md`.

**Counts.** 44 checked · 44 confirmed · 0 wrong · 0 unsupported. Four claims outside the key list were also confirmed
(R3-WGPU-F14, R3-WGPU-F16, R3-GL2-F1, R3-MEASURE-F1). Final state after the fix pass: 43 no change needed · 1 fixed
(R3-MEASURE-F3) · 0 re-sourced · 0 now an open question. No recommendation rests on a claim whose verdict is wrong or
unsupported; every one of the 25 has at least one checked basis claim. M = 77 distinct R3 finding ids in the 25 "Rests
on:" lists (legacy and wave-1 ids not counted; unchanged by the fix), N = 44 checked (45 after the second pass, which
confirmed R3-BUDGET-F6's new quotes).

| Claim id | Claim | Source | Verdict | Location and exact words (as the verifier gave them) | Final state after the fix pass |
|---|---|---|---|---|---|
| R3-WGPU-F1 | r186 dated 24 Sep; then r185 01 Jul, r184 16 Apr, r183 20 Feb | github.com/mrdoob/three.js/releases | confirmed | "24 Sep 14:35" (Latest); "01 Jul 23:22"; "16 Apr 16:49"; "20 Feb 15:31" | no change needed (R3-WGPU-OQ5's release gaps fixed from these dates, C9) |
| R3-WGPU-F3 | API breaks in r183, r185, r186 | three.js wiki Migration Guide, sections 182→183, 184→185, 185→186 | confirmed | "`PostProcessing` has been renamed to `RenderPipeline`."; "…has changed how premultiplied alpha is implemented"; "`PCFSoftShadowMap` with `WebGPURenderer` has been removed" | no change needed |
| R3-WGPU-F4 | three.js calls the renderer experimental | manual webgpurenderer, Migration | confirmed | "still in an experimental state"; "recommended to upgrade to the latest version whenever possible" | no change needed |
| R3-WGPU-F5 | WebGL 2 is the built-in fallback | manual Usage; WebGPURenderer docs; WebGPURenderer.js | confirmed | "uses a WebGPU backend by default and a WebGL 2 backend as a fallback."; "uses a WebGL 2 backend no matter if WebGPU is supported or not."; "const backend = new BackendClass( parameters );" | no change needed |
| R3-WGPU-F6 | custom GLSL and EffectComposer unsupported | manual Migration | confirmed | "…are not supported in WebGPURenderer. This part of your application must be ported to node materials and TSL." | no change needed |
| R3-WGPU-F7 | no promise that WebGPURenderer is faster | manual Migration | confirmed | "you will encounter missing features or a better performance with WebGLRenderer." | no change needed |
| R3-WGPU-F8 | WebGLRenderer kept but gets no big features | manual "State of WebGLRenderer" | confirmed | "the recommended choice for pure WebGL 2 applications"; "no plans to add larger new features" | no change needed |
| R3-WGPU-F9 | start-up is asynchronous | manual Usage; Renderer docs | confirmed (the `await` line not rechecked) | "WebGPU is initialized in an asynchronous fashion."; "…for best compatibility." | no change needed |
| R3-WGPU-F10 | compileAsync prevents compile stutter | Renderer docs, compileAsync | confirmed | "…a phenomenon which is called "shader compilation stutter"…" | no change needed |
| R3-WGPU-F11 | one TSL source builds both WGSL and GLSL | TSL wiki: Learning TSL, Example, Architecture | confirmed | "capable of encoding code into different outputs such as `WGSL`/`GLSL`"; "`WGSLNodeBuilder` aimed at WebGPU and `GLSLNodeBuilder` aimed at WebGL2." | no change needed |
| R3-WGPU-F15 | MSAA uses 4 samples by default | WebGPURenderer docs, samples | confirmed | "When `antialias` is `true`, `4` samples are used by default." | no change needed |
| R3-WGPU-F17 | Chrome 113 / 144 (Linux: Intel Gen12+), Android 121 | MDN GPU.json, chrome | confirmed | "144" "…and Linux (Intel Gen12+ GPUs only)."; "113" "Supported on ChromeOS, macOS, and Windows."; chrome_android "121" | no change needed (now also in R3-BUDGET-REC1's fixed reason, C1) |
| R3-WGPU-F18 | Firefox 141 / 145 / 147; no Intel Mac, no Linux | MDN GPU.json, firefox | confirmed | "Supports Windows since Firefox 141."; "…since Firefox 145."; "…since Firefox 147."; "Does not support macOS on Intel CPUs."; "Does not support Linux." | no change needed (now also in R3-BUDGET-REC1's fixed reason, C1) |
| R3-WGPU-F22 | spec draft of 15 Sep 2026; pipeline creation is async | w3.org/TR/webgpu: status, §3.2.1, §10 | confirmed | "W3C Candidate Recommendation Draft, 15 September 2026"; "…internally asynchronous."; "return a Promise which resolves to a pipeline object when creation of the pipeline has completed." | no change needed |
| R3-GL2-F2 | fallback compute is transform feedback; indirect counts refused | WebGLBackend.js r186, compute() | confirmed | "gl.beginTransformFeedback( gl.POINTS );"; "state.enable( gl.RASTERIZER_DISCARD );"; "…not IndirectStorageBufferAttribute" | no change needed |
| R3-GL2-F4 | fallback GPU timing is skipped without the extension | WebGLBackend.js, init() and initTimestampQuery() | confirmed | "this.disjoint = this.extensions.get( 'EXT_disjoint_timer_query_webgl2' );"; "if ( ! this.disjoint \|\| ! this.trackTimestamp ) return;" | no change needed |
| R3-GL2-F5 | timer extension: Chrome 70, Edge 80, not Firefox or Safari | MDN compat data JSON | confirmed | chrome "70"; edge "80"; firefox, safari, chrome_android "version_added": false | no change needed |
| R3-GL2-F7 | without multi-draw, one draw per batch entry | WebGLBackend.js, _draw | confirmed | "if ( this.hasFeature( 'WEBGL_multi_draw' ) === false ) {" | no change needed |
| R3-GL2-F8 | multi-draw: Chrome 86, Safari 15, not Firefox | MDN compat data JSON | confirmed | "86"; "15"; firefox "version_added": false | no change needed |
| R3-GL2-F9 | render bundles bring no gain on WebGL | BundleGroup docs | confirmed | "…without any performance improvements." | no change needed |
| R3-GL2-F12 | who lands on the fallback | MDN GPU.json | confirmed (the list is the owner's reading, labelled as such) | "Does not support Linux."; "Does not support macOS on Intel CPUs." | no change needed (now the basis of R3-GL2-REC1's and R3-BUDGET-REC1's fixed reasons, C1, C4) |
| R3-GL2-F13 | 31 % lack Direct3D 11.1; compatibility mode is opt-in | Chrome blog posts for WebGPU 139 and 146 | confirmed | "on Windows, 31% of Chrome users don't have Direct3D 11.1 or higher."; "a new opt-in feature called compatibility mode." | no change needed; fixed: R3-GL2-REC1 no longer reads it as a share of fallback users (C4) |
| R3-LOOK-F5 | inverted-hull outline details | ToonOutlinePassNode.js | confirmed | "thickness = 0.003"; "material.side = BackSide;"; "// TODO: support outline thickness ratio for each vertex"; two renderObject calls | no change needed; R3-LOOK-REC2 and R3-BUDGET-REC2 now count its extra draws (L1, L2) |
| R3-LOOK-F9 | GTAO with a normals-and-depth pre-pass | GTAONode docs | confirmed | "…Ground Truth Ambient Occlusion (GTAO) to a scene."; "// pre-pass for normals and depth"; "normals can be automatically reconstructed from depth values" | no change needed; R3-LOOK-REC2 counts the pre-pass as a planned extra (L1) |
| R3-LOOK-F10 | GTAO cost settings | GTAONode docs, properties | confirmed | "Changing this member triggers a shader recompilation."; "a value of `0.5` should be sufficient for most scenes." "Default is `1`." | no change needed |
| R3-LOOK-F13 | bloom: 5 mips, 12 passes | BloomNode.js | confirmed | "this._nMips = 5;"; "[ 6, 10, 14, 18, 22 ]"; "{ depthBuffer: false, type: HalfFloatType }"; the loop quoted: 1 + 2×5 + 1 passes | no change needed |
| R3-CHAR-F1 | skinned mesh needs skin indices and weights | SkinnedMesh docs | confirmed | "requires skin indices and weights as buffer attributes"; "useful when sharing a skeleton across multiple skinned meshes." | no change needed; R3-CHAR-REC1's wording fixed (L3, C11) |
| R3-CHAR-F2 | skinned bounds must be recomputed per frame | SkinnedMesh docs, computeBoundingSphere | confirmed | "…should be recomputed per frame in order to reflect the current animation state." | no change needed |
| R3-CHAR-F4 | InstancedMesh cuts draw calls; no skinning on its page | InstancedMesh docs | confirmed | "will help you to reduce the number of draw calls"; "Represents the color of all instances."; no skinning text on the page | no change needed |
| R3-CHAR-F7 | WebGPU backend: one draw per BatchedMesh entry | WebGPUBackend.js, _draw | confirmed | "passEncoderGPU.drawIndexed( counts[ i ], 1, starts[ i ] / bytesPerElement, 0, i );"; "info.update( object, counts[ i ], 1 );" | no change needed |
| R3-CHAR-F9 | pose-per-instance skinning example is WebGPU only | webgpu_skinning_instancing_individual.html | confirmed | "Per-instance poses are computed once and reused by every render pass."; "mixer.setTime( elapsed + timeOffsets[ i ] );"; "throw new Error( 'No WebGPU support' );" | no change needed |
| R3-CHAR-F12 | a BundleGroup is one bundle, static by default | BundleGroup docs | confirmed | "considered as one render bundle"; static "Default is `true`." | no change needed; R3-CHAR-REC4's bundles now need a measured gain (C3) |
| R3-BUDGET-F1 | 16.66 ms frame, 10 ms of work | web.dev rendering-performance | confirmed | "the browser has 16.66 milliseconds to produce each frame."; "all of your work needs to be completed inside 10 milliseconds" | no change needed |
| R3-BUDGET-F4 | Intel Gen12+ is the only GPU class named | MDN GPU.json | confirmed | "(Intel Gen12+ GPUs only)" | no change needed; R3-BUDGET-REC1 now uses it for Linux only (C1) |
| R3-MEASURE-F2 | Chrome rounds GPU timestamps to 100 µs | Chrome developer-features page | confirmed | "…quantized with a resolution of 100 microseconds"; "Do not use these features in production." | no change needed |
| R3-MEASURE-F3 | renderer.info counters and manual reset | three.js Info docs | confirmed (nit: programs and render targets are counts, not byte sizes) | "The number of draw calls of the current frame."; "…call `renderer.info.reset()` once per frame manually." | fixed: programs and render targets listed as counts, with the docs' words ("The number of active programs."; "The number of active renderTargets.") |
| R3-MEASURE-F4 | GPU timing is opt-in; 2048-query pool | WebGPUBackend.js, Renderer.js, WebGPURenderer.js | confirmed (the option's doc string was not seen in these files) | "this.trackTimestamp = this.trackTimestamp && this.hasFeature( GPUFeatureName.TimestampQuery );"; "async resolveTimestampsAsync( type = 'render' ) {"; "…( this.device, type, 2048 );" | no change needed (the doc-string quote stays as read by the owner; not rechecked) |
| R3-MEASURE-F7 | performance.now() is 5 µs isolated, 100 µs otherwise | MDN Performance.now, "Security requirements" | confirmed | "Resolution in isolated contexts: 5 microseconds"; "Resolution in non-isolated contexts: 100 microseconds" | no change needed |
| R3-MEASURE-F8 | headless Chrome renders in software | Chrome blog supercharge-web-ai-testing | confirmed | "Software only, hardware acceleration unavailable."; "SwiftShader Device (Subzero)"; after the drivers were installed: "Hardware accelerated but at reduced performance." | no change needed |
| R3-AUDIO-F2 | equal power is the default panning model | MDN PannerNode.panningModel | confirmed | "generally regarded as simple and efficient. `equalpower` is the default value." | no change needed |
| R3-AUDIO-F3 | distance formulas; inverse is the default | MDN PannerNode.distanceModel | confirmed (the maxDistance parts not rechecked) | inverse "refDistance / (refDistance + rolloffFactor * (Math.max(distance, refDistance) - refDistance))"; "`inverse` is the default" | no change needed |
| R3-AUDIO-F5 | three.js PositionalAudio uses HRTF and does not allocate | PositionalAudio.js | confirmed | "this.panner.panningModel = 'HRTF';"; "const endTime = this.context.currentTime + this.listener.timeDelta;"; module-level "const _position = /*@__PURE__*/ new Vector3();" | no change needed |
| R3-AUDIO-F8 | lowpass filter, 12 dB per octave | MDN BiquadFilterNode | confirmed | "always has exactly one input and one output."; "Standard second-order resonant lowpass filter with 12dB/octave rolloff." | no change needed |
| R3-AUDIO-F10 | sound before user input meets autoplay rules | MDN Autoplay guide | confirmed | "…outside the context of handling a user input event is subject to autoplay rules." | no change needed |

**The critical read and what the fix pass did** (the verifier's ids; L = look choices, C = other findings):

| Id | The verifier's finding (short) | What the fix pass did |
|---|---|---|
| L1 | R3-LOOK-REC2's "no effect re-renders the scene" and its 1.1 × mark rule out the hull outline and GTAO's documented pre-pass, which the file keeps open | Fixed: REC2 now keeps both open; their extra draws are "planned extras", and the mark is 1.1 × (effects-off draws plus the planned extras); Rests on adds R3-LOOK-F5 |
| L2 | R3-BUDGET-REC2's Low cap of 800 draw calls excludes hull outlines on Low by arithmetic, unsaid | Fixed: the arithmetic is shown (about 786 without a per-object outline pass, about 1,060 with one); the Low row reads "≤ 800 without a per-object outline pass at Low; about ≤ 1,100 with one" (proposed); the trade is left to the look brief |
| L3 | "rigid-skinned soldiers" ties the baseline to legacy's part-based characters | Fixed: the TL;DR, R3-CHAR-REC1 and option C-B say "one skinned mesh per soldier", rigid weights only while the part-based model stays (CHAR-OQ3) |
| L4 | R3-AUDIO-REC1 and R3-CHAR-REC2 assume a legacy-like top-down camera without saying so | Fixed: AUDIO-REC1's listener placement holds only for such a camera (AUDIO-OQ3); CHAR-REC2's budget LOD holds under any camera, distance LOD is conditional on the brief's camera (CHAR-OQ5) |
| L5 | BUDGET-REC2's post slot and Low pixel-ratio cap are cost limits, not look picks | No change (note only; BUDGET-OQ3 already leaves the ratio cap to Mustafa) |
| C1 | R3-BUDGET-REC1 does not follow: Intel Gen12+ applies to Chrome on Linux only; a Windows machine reaches WebGL 2 only by forcing it; no CPU class | Fixed: the reason is rewritten on R3-WGPU-F17, F18 and R3-GL2-F12; the minimum laptop adds a Linux run for the real fallback and a proposed CPU floor (4 cores, 8 threads, no source) for R1-TICK-OQ6 |
| C2 | R3-CHAR-REC1's 2.0 ms and R3-CHAR-REC2's and R3-BUDGET-REC2's 1.0 ms contradict | Fixed: 2.0 ms is the cost with every soldier on the full rig (no budget LOD), 1.0 ms the Low budget with the budget LOD; all three say which |
| C3 | R3-CHAR-REC4 adopts render bundles without a measured gain, against R3-GL2-REC1 | Fixed: bundles only with a measured gain; merging is the named fallback; the mark checks the gain |
| C4 | R3-GL2-REC1 reads GL2-F13's 31 % as a share of fallback users | Fixed: the reason now rests on F12's platform groups, says their share is unknown (OQ2), and reads F13 as a Direct3D 11.1 statistic only |
| C5 | Missing "Rests on:" entries | Fixed: R3-MEASURE-F3 added to R3-WGPU-REC3 and R3-LOOK-REC1 (and R3-LOOK-REC2); R3-MEASURE-REC2 cites R1-DET-REC1 and R1-DET-REC6 |
| C6 | Recommendations that trace to no VISION_V2 §6 item without saying so; AUDIO-REC1's §3 and AUDIO-REC3's §6.2 | Fixed: R3-WGPU-REC2, R3-WGPU-REC4, R3-GL2-REC3, R3-BUDGET-REC1 and R3-MEASURE-REC3 say "No VISION_V2 §6 item"; the four audio recommendations serve the brief's R3 row and say no §6 item names audio; AUDIO-REC1's §3 now reads "the largest mode"; AUDIO-REC3 names §6.2 as reused data, not a need it serves |
| C7 | Missing options: anti-aliasing, shadow technique, "WebGPU required" row, AudioWorklet, GPU-driven culling | Applied as open questions, not options (R3-GL2-OQ5, R3-LOOK-OQ6, R3-CHAR-OQ6, R3-BUDGET-OQ6, R3-AUDIO-OQ6): the fix rule allows no new claims, and an option row needs sourced findings; each says what would decide it. Partly applied, with that reason |
| C8 | Findings marked "confirmed" without a quote: GL2-F11, BUDGET-F6, MEASURE-F6 | Fixed: GL2-F11 quotes the data file's `version_added` values; BUDGET-F6 quotes the Info docs; MEASURE-F6 is now "inferred" from R3-GL2-F4 and F5 |
| C9 | Number nits: WGPU-OQ5's "two to five months"; counts listed as byte sizes; three against four large units; AUDIO-F4's reason for a cap | Fixed: OQ5 gives the gaps (55, 76, 85 days); MEASURE-F3 and BUDGET-F6 list programs and render targets as counts; CHAR-REC4 uses four large units (mark 200 draws and 2.0 ms, proposed, arithmetic shown); AUDIO-F4 and AUDIO-REC2 say a voice costs its nodes whatever its gain (inferred), so the cap, not the curve, bounds voices |
| C10 | No silent contradiction of R1 or R2 | No change needed |
| C11 | R3-CHAR-REC1's "documented on both backends" leans on the Skeleton page; WebGPURenderer's skinning is unconfirmed (CHAR-OQ1) | Fixed: REC1 says so and its mark checks C-B on both backends |

**Not verified** (the verifier's list): claims off the check list (including R3-WGPU-F2, F12, F13, F19, F20, F21), the
Firefox release dates in R3-WGPU-F18, the `maxDistance` parts of R3-AUDIO-F3, the `trackTimestamp` option's doc
string, and any GPU timing (no source gives one, R3-LOOK-F15). The fix pass fetched no new page and re-sourced no claim.

### Second pass (2026-10-29)

The spot-check found more than nits, so the verifier checked what the fix changed (`git diff 605c09d`; report:
`.checkpoints/v2-research/verifier-r3-second-pass-result.md`, notes `verifier-r3-second-pass.md`). Verdict: PASS WITH
NOTES.
- L1, L2, C1 and C2 closed; C3 to C11 closed or, for C7, turned into open questions with a sound reason; L5 left as a
  note. No fix chooses a look or a set of look techniques.
- Claims and numbers checked: 14 confirmed (11 at a source, among them R3-BUDGET-F6's quotes, 3 arithmetic), 0 wrong,
  2 unsupported (both in new open questions: SMAA in R3-LOOK-OQ6 had no source; R3-CHAR-OQ6's WebGPU-only point was an
  unmarked inference).
- Two minor gaps of the L2 kind, each one sentence with no new source, which the verifier said the session could word
  without another pass: (A) R3-BUDGET-REC2's Low draw-call cap now says a GTAO pre-pass passes both caps; (B)
  R3-CHAR-REC4's mark now says it counts before any outline or pre-pass. Applied by the session on 2026-10-29 in the
  verifier's figures, with the nits: R3-LOOK-OQ6 marks SMAA as a candidate with no source; R3-CHAR-OQ6 marks its point
  inferred; the world's 202 is noted as including its own hulls; M is "unchanged by the fix".
- Label: "spot-checked (45 of 77 key claims)". Review stops here: what remains is nits.

### Consistency resolutions (2026-10-30)

Step 3c of `.checkpoints/v2-research/plan-amendments-2.md`, applied by the owner (graphics-engineer) to this file only,
as the advisor approved them on 2026-10-30 (`.checkpoints/v2-research/advisor-resolutions-verdict.md`; the texts as
folded into `resolutions.md` and `cross-topic-oqs.md`). Nothing was fetched; no claim or source was added; the only
new numbers are the approved arithmetic of C9.13. Notes: `.checkpoints/v2-research/graphics-engineer-r3-consistency.md`.
- **Resolutions applied** (by the critic's rank and change id; each "before" text matched the file exactly once):
  - rank 9 (the stress scene): C9.12 (R3-CHAR-REC4's spike: the stress scene's four very large units with its
    soldiers), C9.13 (R3-BUDGET-REC2's arithmetic: about 786 draws at N = 48; at N = 54, MODES-3 (b), about 834, over
    800, R3-BUDGET-OQ4), C9.14 (R3-BUDGET-REC2's heading: the stress scene's fight on screen, with 300 live
    projectiles instead of 200; a recommendation change), C9.15 and C9.16 (R3-MEASURE-REC2's heading and reason);
  - rank 10 (MODES-3, escalated; interim wording): C10.12 (R3-BUDGET-OQ4: the largest mode's N is 48 or 54), C10.13
    (MODES-3 inserted);
  - ranks 1, 5, 6, 11, 17, 19, 21, 22, 23 and 29 change nothing in R3. The look rows (post-processing, outline,
    style) are unchanged.
- **Glossary terms** ([GLOSSARY.md](GLOSSARY.md); linked from the header and from the first use of each term this
  file uses): the one "Retire (exact)" phrase ("the worst case of the largest mode") went with C9.14. "Retire (in
  context)" words replaced where they carried the term's meaning: "characters" for soldiers in the R3-CHAR legacy
  facts (soldiers); a bare 48 for the largest mode in R3-CHAR-REC1 (heading and mark), R3-CHAR-REC2's mark and
  R3-CHAR-REC3's mark (the largest mode's N, 48 or 54 under MODES-3 (b); a recommendation change under reading (b));
  "large units" for the stress scene's in R3-BUDGET-REC2's arithmetic (very large units); "R1-TICK's spike scene" in
  the pointer to R1 for question 5 (the stress scene). The R1 wave-1 fact in question 5's legacy facts keeps R1's
  old words and now names the stress scene and the reference server after them. The other in-context hits stay:
  legacy measurements at 48, R3-LOOK-REC3's soldier counts (a look row), "players" as people, "large units" and
  "effects" for the category or in VISION_V2's words, the Rests-on lines that name R1-TICK's own scene, and unrelated
  words ("always", the 50 % of GPU frame time).
- **Shared open questions added** (part "Open questions"): MODES-3 under "Shared open questions (the same words in
  every topic)"; R3-XT-OQ1 to R3-XT-OQ8 (the critic's ranks 4, 16, 25, 26, 33, 35, 37 and 39) under "Cross-topic
  open questions (consistency critic, 2026-10-29)", with a pointer from each R3 recommendation they name
  (R3-WGPU-REC3, R3-WGPU-REC4, R3-CHAR-REC3, R3-CHAR-REC4, R3-AUDIO-REC3).
- **Header:** the open-question total becomes 47 (9 added); the status label is unchanged (the step-3d verifier
  recounts it).
- **OBJECT:** none.

Step 3d fixes (2026-10-30):
- R3-LOOK-REC3: the advisor's term note added after its spike mark, outside the look row (its "48" is the largest
  mode's N, 48 or 54); the row's words unchanged (a nit).
- R3-BUDGET-REC2: ", p99" added to the main-thread CPU, character animation, sim cost and the five GPU rows (time
  statistics: a time mark with no statistic is a p99; a recommendation change).
- R3-CHAR-REC1: 2.0 ms (p99) and 1.0 ms (p99) (a recommendation change).
- R3-CHAR-REC2: 1.0 ms (p99) (a recommendation change).
- R3-CHAR-REC4: 2.0 ms (p99) GPU time (a recommendation change).
- R3-AUDIO-REC3: 0.2 ms (p99) per frame (a recommendation change).
- R3-AUDIO-REC1: p99 written out, by the advisor's item-6 ruling (the same rule; found by the owner); the HRTF
  addition reads as a rise of that p99 by less than 0.3 ms, since p99s do not add (a nit from the final check).
- Options for question 7, row V-A: "a 48-player fight" becomes a fight at the largest mode's N (48 or 54, MODES-3)
  (a nit).
- R3-AUDIO-REC2's reason: "a 48-player fight" becomes a fight at the largest mode's N (48 or 54, MODES-3) (a nit).
- R3-AUDIO-OQ2: the inventory's "48-bot fight" (GP6's words) stays; the largest mode's N is named beside it (a nit).

## Sources

### Sources read

All read on 2026-10-28. "S" numbers are the entries of the notes file (`.checkpoints/v2-research/graphics-engineer-r3.md`),
where each has its exact quotes. Questions: the question(s) whose findings cite it.

#### Specifications

| S | Source | Title | Version or page date | Tier | Questions |
|---|---|---|---|---|---|
| 16 | https://www.w3.org/TR/webgpu/ | WebGPU | W3C Candidate Recommendation Draft, 15 September 2026 (read in part: §2.1.7.2, §3.2.1, §10) | official | 1, 6 |
| 53 | https://webaudio.github.io/web-audio-api/ | Web Audio API (Editor's Draft) | 27 October 2026 (read in part: status, §1.11; the PannerNode and spatialization sections were not reached) | official | 7 |

#### Browser documentation and compatibility data

| S | Source | Title | Version or page date | Tier | Questions |
|---|---|---|---|---|---|
| 9 | https://developer.mozilla.org/en-US/docs/Mozilla/Firefox/Releases/141 | Firefox 141 release notes for developers | Firefox 141, released July 22, 2025 | official | 1 |
| 10 | https://developer.mozilla.org/en-US/docs/Mozilla/Firefox/Releases/147 | Firefox 147 release notes for developers | Firefox 147, released January 13, 2026 | official | 1 |
| 11 | https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API | WebGPU API (MDN) | last modified Sep 2, 2026 | official | 1 |
| 12 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/GPU.json | MDN browser-compat-data, `GPU` | main branch as of 2026-10-28 | official | 1, 2, 5 |
| 13 | https://webkit.org/blog/17333/webkit-features-in-safari-26-0/ | WebKit Features in Safari 26.0 | September 15, 2025 | official | 1 |
| 17 | https://developer.chrome.com/docs/web-platform/webgpu/developer-features | WebGPU developer features | 2025-06-03 | official | 1, 2, 6 |
| 21 | https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query | EXT_disjoint_timer_query (MDN) | last modified Aug 29, 2024 | official | 2 |
| 22 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/EXT_disjoint_timer_query.json | MDN browser-compat-data, `EXT_disjoint_timer_query` | main, 2026-10-28 | official | 2 |
| 23 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/EXT_disjoint_timer_query_webgl2.json | MDN browser-compat-data, `EXT_disjoint_timer_query_webgl2` | main, 2026-10-28 | official | 2, 6 |
| 24 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WEBGL_multi_draw.json | MDN browser-compat-data, `WEBGL_multi_draw` | main, 2026-10-28 | official | 2 |
| 27 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebGL2RenderingContext.json | MDN browser-compat-data, `WebGL2RenderingContext` | main, 2026-10-28 | official | 2 |
| 28 | https://developer.chrome.com/blog/new-in-webgpu-146 | What's New in WebGPU (Chrome 146) | February 25, 2026 | official | 2 |
| 29 | https://developer.chrome.com/blog/new-in-webgpu-139 | What's New in WebGPU (Chrome 139) | July 30, 2025 | official | 2 |
| 45 | https://web.dev/articles/rendering-performance | Rendering performance | last updated 2023-12-13 | official | 5 |
| 46 | https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame | Window: requestAnimationFrame() | last modified Aug 21, 2026 | official | 5, 6 |
| 47 | https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio | Window: devicePixelRatio | last modified Aug 21, 2026 | official | 5 |
| 50 | https://developer.mozilla.org/en-US/docs/Web/API/Performance/now | Performance: now() | last modified Aug 25, 2025 | official | 6 |
| 51 | https://developer.chrome.com/blog/supercharge-web-ai-testing | Supercharge Web AI model testing: WebGPU, WebGL, and Headless Chrome | last updated 2024-01-16 | official | 6 |
| 52 | https://docs.github.com/en/actions/concepts/runners/larger-runners | Larger runners (GitHub Actions) | no date shown | official | 6 |
| 54 | https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel | PannerNode: distanceModel | last modified Apr 7, 2023 | official | 7 |
| 55 | https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel | PannerNode: panningModel | last modified Apr 7, 2023 | official | 7 |
| 56 | https://developer.mozilla.org/en-US/docs/Web/API/PannerNode | PannerNode | last modified Sep 28, 2026 | official | 7 |
| 60 | https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API/Web_audio_spatialization_basics | Web audio spatialization basics | last modified Aug 27, 2026 | official | 7 |
| 61 | https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode | BiquadFilterNode | last modified Jun 23, 2025 | official | 7 |
| 62 | https://developer.mozilla.org/en-US/docs/Web/Media/Guides/Autoplay | Autoplay guide for media and Web Audio APIs | last modified Sep 10, 2026 | official | 7 |

#### three.js (docs, manual, wiki, releases, examples and source; release r186)

| S | Source | Title | Version or page date | Tier | Questions |
|---|---|---|---|---|---|
| 1 | https://github.com/mrdoob/three.js/releases ; https://github.com/mrdoob/three.js/releases/tag/r186 | three.js releases; r186 | r186, "24 Sep" (year not shown; 2026 inferred) | official | 1, 6 |
| 2 | https://github.com/mrdoob/three.js/wiki/Migration-Guide | Migration Guide | sections r182 → r186 | official | 1 |
| 3 | https://threejs.org/manual/pages/webgpurenderer.html | Manual: WebGPURenderer | no date (site at r186) | official | 1 |
| 4 | https://threejs.org/docs/pages/WebGPURenderer.html | Docs: WebGPURenderer | r186 site | official | 1 |
| 5 | https://threejs.org/docs/pages/Renderer.html | Docs: Renderer | r186 site | official | 1, 6 |
| 6 | https://threejs.org/docs/pages/Info.html | Docs: Info | r186 site | official | 5, 6 |
| 7 | https://github.com/mrdoob/three.js/wiki/Three.js-Shading-Language | TSL specification (wiki) | no date | official | 1 |
| 14 | https://threejs.org/docs/pages/RenderPipeline.html | Docs: RenderPipeline | r186 site | official | 1 |
| 15 | https://threejs.org/docs/pages/PassNode.html | Docs: PassNode | r186 site | official | 1, 3 |
| 18 | https://threejs.org/docs/pages/BundleGroup.html | Docs: BundleGroup | r186 site | official | 1, 2, 4 |
| 19 | https://raw.githubusercontent.com/mrdoob/three.js/r186/src/renderers/webgl-fallback/WebGLBackend.js | WebGLBackend.js | r186 | official | 2, 6 |
| 20 | https://raw.githubusercontent.com/mrdoob/three.js/r186/src/renderers/webgl-fallback/nodes/GLSLNodeBuilder.js | GLSLNodeBuilder.js | r186 | official | 2 |
| 25 | https://threejs.org/docs/pages/BatchedMesh.html | Docs: BatchedMesh | r186 site | official | 4 |
| 26 | https://raw.githubusercontent.com/mrdoob/three.js/r186/src/renderers/webgpu/WebGPUBackend.js | WebGPUBackend.js | r186 | official | 4, 6 |
| 30 | https://threejs.org/docs/pages/MeshToonNodeMaterial.html | Docs: MeshToonNodeMaterial | r186 site | official | 3 |
| 31 | https://raw.githubusercontent.com/mrdoob/three.js/r186/src/nodes/functions/ToonLightingModel.js | ToonLightingModel.js | r186 | official | 3 |
| 32 | https://threejs.org/docs/pages/ToonOutlinePassNode.html | Docs: ToonOutlinePassNode | r186 site | official | 3 |
| 33 | https://raw.githubusercontent.com/mrdoob/three.js/r186/src/nodes/display/ToonOutlinePassNode.js | ToonOutlinePassNode.js | r186 | official | 3 |
| 34 | https://threejs.org/docs/pages/SobelOperatorNode.html ; https://raw.githubusercontent.com/mrdoob/three.js/r186/examples/jsm/tsl/display/SobelOperatorNode.js | Docs and source: SobelOperatorNode | r186 | official | 3 |
| 35 | https://threejs.org/docs/pages/GTAONode.html | Docs: GTAONode | r186 site | official | 3 |
| 36 | https://threejs.org/docs/pages/BloomNode.html | Docs: BloomNode | r186 site | official | 3 |
| 37 | https://raw.githubusercontent.com/mrdoob/three.js/r186/examples/jsm/tsl/display/BloomNode.js | BloomNode.js | r186 | official | 3 |
| 38 | https://threejs.org/docs/pages/SkinnedMesh.html | Docs: SkinnedMesh | r186 site | official | 4 |
| 39 | https://raw.githubusercontent.com/mrdoob/three.js/r186/examples/files.json | Examples list | r186 | official | 4 |
| 40 | https://raw.githubusercontent.com/mrdoob/three.js/r186/examples/webgpu_skinning_instancing.html | Example: webgpu_skinning_instancing | r186 | official | 4 |
| 41 | https://raw.githubusercontent.com/mrdoob/three.js/r186/examples/webgpu_skinning_instancing_individual.html | Example: webgpu_skinning_instancing_individual | r186 | official | 4 |
| 42 | https://threejs.org/docs/pages/InstancedMesh.html | Docs: InstancedMesh | r186 site | official | 4 |
| 43 | https://threejs.org/docs/pages/LOD.html | Docs: LOD | r186 site | official | 4 |
| 44 | https://threejs.org/docs/pages/Skeleton.html | Docs: Skeleton | r186 site | official | 4 |
| 48 | https://raw.githubusercontent.com/mrdoob/three.js/r186/src/renderers/common/Renderer.js | Renderer.js | r186 | official | 6 |
| 49 | https://raw.githubusercontent.com/mrdoob/three.js/r186/src/renderers/webgpu/WebGPURenderer.js | WebGPURenderer.js | r186 | official | 6 |
| 57 | https://threejs.org/docs/pages/PositionalAudio.html | Docs: PositionalAudio | r186 site | official | 7 |
| 58 | https://raw.githubusercontent.com/mrdoob/three.js/r186/src/audio/PositionalAudio.js | PositionalAudio.js | r186 | official | 7 |
| 59 | https://raw.githubusercontent.com/mrdoob/three.js/r186/src/audio/AudioListener.js | AudioListener.js | r186 | official | 7 |

#### Other

| S | Source | Title | Version or page date | Tier | Questions |
|---|---|---|---|---|---|
| 8 | https://caniuse.com/webgpu | Can I use: WebGPU | no update date; usage data September 2026 | other (aggregator) | 1 |

**Counts by tier (sources read):** official 61 · shipped game 0 · other 1 · total 62.

Repository documents used as legacy facts (not counted as sources): `docs/LEGACY_INVENTORY.md` (rows CD15, CD17, CL1–CL14,
GW4, GW15, GW17, GA15, GC16, GN1, GN8, GP1, GP5, GP6, TL9, TL11, TL12, TL15, TL17), `docs/LEGACY_TUNING.md` (CAM_STYLES,
TUNE.fx, JUG), `docs/AUDIT_BUGS_PERF.md` §1, `docs/STATUS.md`, `docs/reports/2026-10-16-three-teams-core.md`,
`docs/reports/2026-10-13-phase-0.md`, `docs/research/r1-simulation-core.md`, `docs/research/r2-netcode.md`.

### Sources not read

| # | Source | Reason |
|---|---|---|
| 1 | https://threejs.org/manual/en/webgpurenderer.html | HTTP 404 (the /manual/pages/ copy was read, S3) |
| 2 | https://discourse.threejs.org/t/webgpurenderer-forcewebgl-true-vs-webglrenderer/87805 | outside the allow list |
| 3 | https://www.firefox.com/firefox/147.0/releasenotes/ | outside the allow list (MDN's release notes read instead, S10) |
| 4 | utsubo.com WebGPU migration and performance articles (search leads) | outside the allow list |
| 5 | buildmvpfast.com WebGPU migration article (search lead) | outside the allow list |
| 6 | registry.khronos.org, WebGL 2.0 specification | outside the allow list |
| 7 | registry.khronos.org, `EXT_disjoint_timer_query_webgl2` extension page | outside the allow list (MDN read instead, S21) |
| 8 | webgpufundamentals.org, "WebGPU Compatibility Mode" (search lead) | outside the allow list |
| 9 | activision.com, "Practical Real-Time Strategies for Accurate Indirect Occlusion" (GTAO) | outside the allow list |
| 10 | Valve, "Illustrative Rendering in Team Fortress 2" (NPAR 2007, valvesoftware.com) | outside the allow list |
| 11 | Arc System Works, Guilty Gear Xrd art-style talk (GDC 2015, GDC Vault) | video only (GDC Vault redirects outside the allow list) |
| 12 | https://raw.githubusercontent.com/mrdoob/three.js/r186/src/nodes/accessors/SkinningNode.js | HTTP 404 (path not found) |
| 13 | https://threejs.org/docs/pages/SkinningNode.html | HTTP 404 |
| 14 | Steam Hardware & Software Survey (store.steampowered.com) | outside the allow list |
| 15 | Intel, AMD and NVIDIA product specification pages | outside the allow list |
| 16 | GitHub "supported runners and hardware resources" (linked from S52) | not fetched: not needed for the recommendation |
| 17 | Web Audio API specification, PannerNode and §6 spatialization (webaudio.github.io; https://www.w3.org/TR/webaudio-1.1/) | fetched text truncated before those sections (MDN read instead) |
| 18 | lists.w3.org public-audio archive, July–September 2014 (HRTF cost; search lead) | outside the allow list |
| 19 | codereview.chromium.org, issue 652073003 (equal-power default; search lead) | outside the allow list |
| 20 | abratabia.com, spatial audio guide (search lead) | outside the allow list |
| 21 | brave.com, Brave's WebGPU support | outside the allow list |
| 22 | https://developer.chrome.com/blog/new-in-webgpu-121 (search lead) | not fetched: the developer-features page (S17) covered the point |

Also read only in part, and listed as read above: S16 (the WebGPU specification: the timestamp precision value in
"current queue timestamp" lies beyond the fetched text) and S53 (the Web Audio Editor's Draft).
