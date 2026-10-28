# R3-WGPU draft: three.js WebGPURenderer with TSL

Question 1 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S1–S18).

## Findings

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

### Legacy facts used
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

## Options

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

## Recommendations

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
Serves: VISION_V2 §4 (no regression slips into the 30 and 60 fps targets unmeasured).
Spike must prove (proposed): an upgrade from one pinned release to the next changes the benchmark scene's p95 GPU and
CPU frame time by no more than 5 % without an explained cause (R3-MEASURE).

**R3-WGPU-REC3. Build every render pipeline behind the loading screen: `await renderer.init()`, then `compileAsync()` on
a warm-up scene that holds every material variant a match can show (each faction, each effect, each blend state); the
match itself creates no new shader programs.**
Reason: WebGPU creates pipelines asynchronously (F22) and three.js names the first-sight hitch "shader compilation
stutter" (F10); legacy already compiles on first sight (TL15: 16 programs vs 9); the cutaway's blend-state toggle
(GW17) is one such variant. Init itself is asynchronous (F9).
Rests on: R3-WGPU-F9, F10, F22; legacy TL15, GW17.
Serves: VISION_V2 §6.9 (large units and their effects "without dropping frames"), §4.
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
Serves: VISION_V2 §4.
Spike must prove (proposed): on each browser of F17–F19 the overlay and the test log name the backend correctly
(checked against `navigator.gpu` availability) in 10 of 10 starts.

## Open questions

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
- **R3-WGPU-OQ5. Which release to pin.** r186 is current (F1); releases arrive about every two to five months (F1).
  Decided by: the release current when the spike starts.

### Recommendations that hinge on a source not read
| Recommendation | Source | Reason not read |
|---|---|---|
| R3-WGPU-REC1 (mark 3) | discourse.threejs.org thread 87805 (a maintainer on forceWebGL against WebGLRenderer; a search lead) | outside the allow list |
| R3-WGPU-REC4 | Brave's own WebGPU documentation (brave.com) | outside the allow list (OQ1) |

## Sources
Read (all 2026-10-28): S1 releases + r186 tag (official); S2 Migration Guide wiki (official); S3 manual WebGPURenderer
(official); S4 docs WebGPURenderer (official); S5 docs Renderer (official); S7 TSL wiki (official); S8 caniuse webgpu
(other); S9 MDN Firefox 141 (official); S10 MDN Firefox 147 (official); S11 MDN WebGPU API (official); S12 MDN BCD
GPU.json (official); S13 WebKit Safari 26.0 post (official); S14 docs RenderPipeline (official); S15 docs PassNode
(official); S16 W3C WebGPU CRD (official); S18 docs BundleGroup (official).
Not read: https://threejs.org/manual/en/webgpurenderer.html (HTTP 404; the /manual/pages/ copy was read);
discourse.threejs.org/t/webgpurenderer-forcewebgl-true-vs-webglrenderer/87805 (outside the allow list);
www.firefox.com/firefox/147.0/releasenotes/ (outside the allow list; MDN's page read instead); utsubo.com and
buildmvpfast.com articles (search leads, outside the allow list).

## Cross-topic notes
- R5 (toolchain): the TypeScript client imports three's WebGPU build; R3-WGPU-REC2's pinned version is a lockfile
  entry, and the upgrade check is a CI job (R3-MEASURE).
- R1-TICK-REC5: the main thread renders every frame; `setAnimationLoop` (F9) is the render clock, consistent with
  R1-TICK-REC5's "`requestAnimationFrame` is the render clock only".
