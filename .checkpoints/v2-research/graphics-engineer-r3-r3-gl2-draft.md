# R3-GL2 draft: the WebGL 2 fallback

Question 2 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S19–S29; S3–S5, S12, S16, S18 from question 1).

## Findings

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

**R3-GL2-F11.** WebGL 2 itself is everywhere the four target browsers run: Chrome 56, Edge (mirror), Firefox 51,
Safari 15.
· Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebGL2RenderingContext.json · official ·
read 2026-10-28 · confirmed in the source.

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

### Legacy facts used
- **CD15, TL11, TL17**: legacy is WebGL only (r128 WebGLRenderer; headless-gl WebGL 1 in the tools). Every legacy
  technique (MSAA, one PCF shadow map, inverted hulls, sprites, pooled meshes) uses only what WebGL 2 has, so nothing
  legacy does needs WebGPU-only features.
- **CL9**: legacy's draw-call saving is CPU-side merging (village 398 to 202 draw calls, AUDIT_BUGS_PERF CS12); it works
  identically on any backend, unlike BatchedMesh on Firefox's fallback (F7, F8).
- **GP5**: legacy's perf overlay reads `renderer.info` (draw calls, triangles, programs); the WebGL 2 backend fills the
  same `renderer.info` (R3-WGPU, Info page), so the overlay ports to both backends.

## Options

| Option | What it is | Effort | What a fallback player gets | Risk | Fit with VISION_V2 §4, §6.9 |
|---|---|---|---|---|---|
| F-A. Same features on both backends ("lowest common denominator") | Use no compute, no storage writes, no bundles; everything WebGL 2 can do | Lowest; one look, one test matrix | The same game | Forgoes GPU particles or culling by compute on WebGPU | Keeps one look everywhere; §6.9 effects must fit CPU or vertex work |
| F-B. Tiered: WebGPU-only extras behind `hasFeature` / backend checks, with a defined fallback for each | e.g. compute particles on WebGPU, CPU or fewer particles on WebGL 2 | Two code paths per extra effect; tests on both | The same rules, fewer or simpler effects | Look drift between backends; every extra needs a fallback test | Fits §4 if the Low tier is the WebGL 2 tier |
| F-C. Fallback as a separate minimum tier only | WebGL 2 always runs the Low settings | Small | Low settings even on a strong machine | Strong Linux or Firefox-on-Mac machines get Low | Simple, but punishes some recommended-class machines |
| F-D. Also try WebGPU compatibility mode before WebGL 2 | `featureLevel: "compatibility"` adapter where core WebGPU is missing (F13) | Needs three.js support (unknown) | WebGPU on some older devices | Opt-in, platforms still being added (F13) | Could widen WebGPU reach on the minimum laptop |

## Recommendations

**R3-GL2-REC1. Option F-A for the first slice, F-B only by measured need: every technique the engine ships in the first
spikes must exist on the WebGL 2 backend; a WebGPU-only extra (compute, storage writes, bundles) enters only with a
named fallback and a measured gain.**
Reason: compute on the fallback is a transform-feedback emulation without indirect dispatch (F2), storage is texture
emulation without atomics (F3), and bundles give no gain there (F9). The players who land there are not rare edge
cases (F12, F13). Legacy's look uses nothing WebGL 2 lacks (CD15), so F-A loses nothing legacy had.
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
Serves: VISION_V2 §4 (measuring the targets on every browser).
Spike must prove (proposed): the measurement harness (R3-MEASURE-REC1) reports GPU time on Chrome's WebGL 2 backend and
reports "GPU time not available" (never 0) on Firefox and Safari's WebGL 2 backends.

## Open questions

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

### Recommendations that hinge on a source not read
| Recommendation | Source | Reason not read |
|---|---|---|
| R3-GL2-REC1 | Khronos WebGL 2.0 specification (registry.khronos.org; that WebGL 2 has no compute stage) | outside the allow list (the three.js source, F2, shows the emulation instead) |
| R3-GL2-REC3 | Khronos `EXT_disjoint_timer_query_webgl2` registry page | outside the allow list (MDN read instead, F6) |

## Sources
Read (all 2026-10-28): S19 WebGLBackend.js r186 (official, source); S20 GLSLNodeBuilder.js r186 (official, source); S21
MDN EXT_disjoint_timer_query (official); S22 BCD EXT_disjoint_timer_query.json (official); S23 BCD
EXT_disjoint_timer_query_webgl2.json (official); S24 BCD WEBGL_multi_draw.json (official); S27 BCD
WebGL2RenderingContext.json (official); S28 Chrome "New in WebGPU 146" (official); S29 Chrome "New in WebGPU 139"
(official); reused: S12 GPU.json, S17 Chrome developer features, S18 BundleGroup.
Not read: registry.khronos.org (WebGL 2 spec, extension registry: outside the allow list); webgpufundamentals.org
"compatibility mode" lesson (search lead, outside the allow list); discourse.threejs.org thread 87805 (outside the
allow list).

## Cross-topic notes
- R6 (telemetry): the backend in use and the adapter info are worth one field in the client's session report
  (OQ2).
- R5 (CI): headless runs can force the WebGL 2 backend; neither backend's frame times count from headless (R3-MEASURE).
