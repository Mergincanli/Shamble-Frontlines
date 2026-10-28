# R3 verifier spot-check: the report, verbatim

Workflow run wf_1f477dcb-7cb (script `v2-research-r3-check-fix-run1.js`, prompt `prompts/verifier-r3.txt`), returned 2026-10-28 (about 20:45 UTC; copied from the workflow's own result by the session). Notes: `verifier-r3.md`.

VERDICT: PASS WITH NOTES. All 44 claims I checked are confirmed at their sources. The critical read found four problems that are more than nits (L1, L2, C1, C2 below), so the R3 file needs an owner fix. None of them is a source error.

Gate: skipped, as the task says (research check; the main session runs the gate).
Symbol diff: not applicable (research).
Evidence: WebFetch only, all on allow-listed domains, 39 fetches. Exact quotes are in `/home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r3.md`, which ends with the full table and DONE.

**Research spot-check (claim id · claim · source · verdict · location and exact words)**

| Claim | What it says | Source | Verdict | Location and exact words |
|---|---|---|---|---|
| WGPU-F1 | r186 dated 24 Sep; then r185 01 Jul, r184 16 Apr, r183 20 Feb | github.com/mrdoob/three.js/releases | confirmed | "24 Sep 14:35" (Latest); "01 Jul 23:22"; "16 Apr 16:49"; "20 Feb 15:31" |
| WGPU-F3 | API breaks in r183, r185, r186 | three.js wiki Migration Guide, sections 182→183, 184→185, 185→186 | confirmed | "`PostProcessing` has been renamed to `RenderPipeline`."; "…has changed how premultiplied alpha is implemented"; "`PCFSoftShadowMap` with `WebGPURenderer` has been removed" |
| WGPU-F4 | three.js calls the renderer experimental | manual webgpurenderer, Migration | confirmed | "still in an experimental state"; "recommended to upgrade to the latest version whenever possible" |
| WGPU-F5 | WebGL 2 is the built-in fallback | manual Usage; WebGPURenderer docs; WebGPURenderer.js | confirmed | "uses a WebGPU backend by default and a WebGL 2 backend as a fallback."; "uses a WebGL 2 backend no matter if WebGPU is supported or not."; "const backend = new BackendClass( parameters );" |
| WGPU-F6 | custom GLSL and EffectComposer unsupported | manual Migration | confirmed | "…are not supported in WebGPURenderer. This part of your application must be ported to node materials and TSL." |
| WGPU-F7 | no promise that WebGPURenderer is faster | manual Migration | confirmed | "you will encounter missing features or a better performance with WebGLRenderer." |
| WGPU-F8 | WebGLRenderer kept but gets no big features | manual "State of WebGLRenderer" | confirmed | "the recommended choice for pure WebGL 2 applications"; "no plans to add larger new features" |
| WGPU-F9 | start-up is asynchronous | manual Usage; Renderer docs | confirmed (the `await` line not rechecked) | "WebGPU is initialized in an asynchronous fashion."; "…for best compatibility." |
| WGPU-F10 | compileAsync prevents compile stutter | Renderer docs, compileAsync | confirmed | "…a phenomenon which is called "shader compilation stutter"…" |
| WGPU-F11 | one TSL source builds both WGSL and GLSL | TSL wiki: Learning TSL, Example, Architecture | confirmed | "capable of encoding code into different outputs such as `WGSL`/`GLSL`"; "`WGSLNodeBuilder` aimed at WebGPU and `GLSLNodeBuilder` aimed at WebGL2." |
| WGPU-F15 | MSAA uses 4 samples by default | WebGPURenderer docs, samples | confirmed | "When `antialias` is `true`, `4` samples are used by default." |
| WGPU-F17 | Chrome 113 / 144 (Linux: Intel Gen12+), Android 121 | MDN GPU.json, chrome | confirmed | "144" "…and Linux (Intel Gen12+ GPUs only)."; "113" "Supported on ChromeOS, macOS, and Windows."; chrome_android "121" |
| WGPU-F18 | Firefox 141 / 145 / 147; no Intel Mac, no Linux | MDN GPU.json, firefox | confirmed | "Supports Windows since Firefox 141."; "…since Firefox 145."; "…since Firefox 147."; "Does not support macOS on Intel CPUs."; "Does not support Linux." |
| WGPU-F22 | spec draft of 15 Sep 2026; pipeline creation is async | w3.org/TR/webgpu: status, §3.2.1, §10 | confirmed | "W3C Candidate Recommendation Draft, 15 September 2026"; "…internally asynchronous."; "return a Promise which resolves to a pipeline object when creation of the pipeline has completed." |
| GL2-F2 | fallback compute is transform feedback; indirect counts refused | WebGLBackend.js r186, compute() | confirmed | "gl.beginTransformFeedback( gl.POINTS );"; "state.enable( gl.RASTERIZER_DISCARD );"; "…not IndirectStorageBufferAttribute" |
| GL2-F4 | fallback GPU timing is skipped without the extension | WebGLBackend.js, init() and initTimestampQuery() | confirmed | "this.disjoint = this.extensions.get( 'EXT_disjoint_timer_query_webgl2' );"; "if ( ! this.disjoint \|\| ! this.trackTimestamp ) return;" |
| GL2-F5 | timer extension: Chrome 70, Edge 80, not Firefox or Safari | MDN compat data JSON | confirmed | chrome "70"; edge "80"; firefox, safari, chrome_android "version_added": false |
| GL2-F7 | without multi-draw, one draw per batch entry | WebGLBackend.js, _draw | confirmed | "if ( this.hasFeature( 'WEBGL_multi_draw' ) === false ) {" |
| GL2-F8 | multi-draw: Chrome 86, Safari 15, not Firefox | MDN compat data JSON | confirmed | "86"; "15"; firefox "version_added": false |
| GL2-F9 | render bundles bring no gain on WebGL | BundleGroup docs | confirmed | "…without any performance improvements." |
| GL2-F12 | who lands on the fallback | MDN GPU.json | confirmed (the list is the owner's reading, labelled as such) | "Does not support Linux."; "Does not support macOS on Intel CPUs." |
| GL2-F13 | 31 % lack Direct3D 11.1; compatibility mode is opt-in | Chrome blog posts for WebGPU 139 and 146 | confirmed | "on Windows, 31% of Chrome users don't have Direct3D 11.1 or higher."; "a new opt-in feature called compatibility mode." |
| LOOK-F5 | inverted-hull outline details | ToonOutlinePassNode.js | confirmed | "thickness = 0.003"; "material.side = BackSide;"; "// TODO: support outline thickness ratio for each vertex"; two renderObject calls |
| LOOK-F9 | GTAO with a normals-and-depth pre-pass | GTAONode docs | confirmed | "…Ground Truth Ambient Occlusion (GTAO) to a scene."; "// pre-pass for normals and depth"; "normals can be automatically reconstructed from depth values" |
| LOOK-F10 | GTAO cost settings | GTAONode docs, properties | confirmed | "Changing this member triggers a shader recompilation."; "a value of `0.5` should be sufficient for most scenes." "Default is `1`." |
| LOOK-F13 | bloom: 5 mips, 12 passes | BloomNode.js | confirmed | "this._nMips = 5;"; "[ 6, 10, 14, 18, 22 ]"; "{ depthBuffer: false, type: HalfFloatType }"; the loop quoted: 1 + 2×5 + 1 passes |
| CHAR-F1 | skinned mesh needs skin indices and weights | SkinnedMesh docs | confirmed | "requires skin indices and weights as buffer attributes"; "useful when sharing a skeleton across multiple skinned meshes." |
| CHAR-F2 | skinned bounds must be recomputed per frame | SkinnedMesh docs, computeBoundingSphere | confirmed | "…should be recomputed per frame in order to reflect the current animation state." |
| CHAR-F4 | InstancedMesh cuts draw calls; no skinning on its page | InstancedMesh docs | confirmed | "will help you to reduce the number of draw calls"; "Represents the color of all instances."; no skinning text on the page |
| CHAR-F7 | WebGPU backend: one draw per BatchedMesh entry | WebGPUBackend.js, _draw | confirmed | "passEncoderGPU.drawIndexed( counts[ i ], 1, starts[ i ] / bytesPerElement, 0, i );"; "info.update( object, counts[ i ], 1 );" |
| CHAR-F9 | pose-per-instance skinning example is WebGPU only | webgpu_skinning_instancing_individual.html | confirmed | "Per-instance poses are computed once and reused by every render pass."; "mixer.setTime( elapsed + timeOffsets[ i ] );"; "throw new Error( 'No WebGPU support' );" |
| CHAR-F12 | a BundleGroup is one bundle, static by default | BundleGroup docs | confirmed | "considered as one render bundle"; static "Default is `true`." |
| BUDGET-F1 | 16.66 ms frame, 10 ms of work | web.dev rendering-performance | confirmed | "the browser has 16.66 milliseconds to produce each frame."; "all of your work needs to be completed inside 10 milliseconds" |
| BUDGET-F4 | Intel Gen12+ is the only GPU class named | MDN GPU.json | confirmed | "(Intel Gen12+ GPUs only)" |
| MEASURE-F2 | Chrome rounds GPU timestamps to 100 µs | Chrome developer-features page | confirmed | "…quantized with a resolution of 100 microseconds"; "Do not use these features in production." |
| MEASURE-F3 | renderer.info counters and manual reset | three.js Info docs | confirmed (nit: programs and render targets are counts, not byte sizes) | "The number of draw calls of the current frame."; "…call `renderer.info.reset()` once per frame manually." |
| MEASURE-F4 | GPU timing is opt-in; 2048-query pool | WebGPUBackend.js, Renderer.js, WebGPURenderer.js | confirmed (the option's doc string was not seen in these files) | "this.trackTimestamp = this.trackTimestamp && this.hasFeature( GPUFeatureName.TimestampQuery );"; "async resolveTimestampsAsync( type = 'render' ) {"; "…( this.device, type, 2048 );" |
| MEASURE-F7 | performance.now() is 5 µs isolated, 100 µs otherwise | MDN Performance.now, "Security requirements" | confirmed | "Resolution in isolated contexts: 5 microseconds"; "Resolution in non-isolated contexts: 100 microseconds" |
| MEASURE-F8 | headless Chrome renders in software | Chrome blog supercharge-web-ai-testing | confirmed | "Software only, hardware acceleration unavailable."; "SwiftShader Device (Subzero)"; after the drivers were installed: "Hardware accelerated but at reduced performance." |
| AUDIO-F2 | equal power is the default panning model | MDN PannerNode.panningModel | confirmed | "generally regarded as simple and efficient. `equalpower` is the default value." |
| AUDIO-F3 | distance formulas; inverse is the default | MDN PannerNode.distanceModel | confirmed (the maxDistance parts not rechecked) | inverse "refDistance / (refDistance + rolloffFactor * (Math.max(distance, refDistance) - refDistance))"; "`inverse` is the default" |
| AUDIO-F5 | three.js PositionalAudio uses HRTF and does not allocate | PositionalAudio.js | confirmed | "this.panner.panningModel = 'HRTF';"; "const endTime = this.context.currentTime + this.listener.timeDelta;"; module-level "const _position = /*@__PURE__*/ new Vector3();" |
| AUDIO-F8 | lowpass filter, 12 dB per octave | MDN BiquadFilterNode | confirmed | "always has exactly one input and one output."; "Standard second-order resonant lowpass filter with 12dB/octave rolloff." |
| AUDIO-F10 | sound before user input meets autoplay rules | MDN Autoplay guide | confirmed | "…outside the context of handling a user input event is subject to autoplay rules." |

**Counts:** 44 confirmed / 0 wrong / 0 unsupported. Four claims outside the key list were also confirmed: WGPU-F14, WGPU-F16, GL2-F1, MEASURE-F1.

**Wrong or unsupported basis claims per recommendation:** none, for all 25 recommendations. Each has at least one checked basis claim:
- WGPU-REC1: F11, F6, F7
- WGPU-REC2: F1, F3
- WGPU-REC3: F10, F22
- WGPU-REC4: F17, F18
- GL2-REC1: F2, F13
- GL2-REC2: F7, F8
- GL2-REC3: F4, F5
- LOOK-REC1: F13
- LOOK-REC2: F9, F10
- LOOK-REC3: F5
- CHAR-REC1: F9, F1
- CHAR-REC2: F2
- CHAR-REC3: F4
- CHAR-REC4: F7, F12
- BUDGET-REC1: F4
- BUDGET-REC2: F1
- BUDGET-REC3: WGPU-F15, LOOK-F13
- MEASURE-REC1: F2, F7
- MEASURE-REC2: F3
- MEASURE-REC3: F8
- MEASURE-REC4: F2
- AUDIO-REC1: F5, F2
- AUDIO-REC2: F3
- AUDIO-REC3: F8
- AUDIO-REC4: F10

**Critical read, ranked**

Look choices (ranked first):
- **L1. LOOK-REC2 rules out techniques the file keeps open.** Its recommendation ("one scene pass with MRT … so no effect re-renders the scene") and its pass mark ("drawCalls stays within 1.1 × … (no scene re-render)") cannot be met by two variants:
  - the hull outline (`toonOutlinePass` draws each toon object twice, LOOK-F5);
  - GTAO as documented (a scene pre-pass, LOOK-F9/F11).
  So it quietly favours screen-space outlines and AO without the pre-pass, and contradicts LOOK-REC3, which keeps both outline families open. Moderate.
- **L2. BUDGET-REC2's Low draw-call cap (≤ 800) excludes hull outlines on Low by arithmetic, without saying so.** CHAR-REC1 allows ≤ 4 draws per soldier per pass, so 48 soldiers × 4 = 192 per pass. Shadow, main and hull passes make 576. The world (legacy 202–303) and large units (up to +150, CHAR-REC4) bring the total to about 930–1,030. Moderate-minor (the numbers are marked proposed).
- **L3. CHAR-REC1 and the TL;DR say "rigid-skinned soldiers".** That ties the baseline to legacy's part-based characters (CL10), while the character model is still pending (CHAR-OQ3). Option C-B works with any skin weights, so this is a wording fix. Minor.
- **L4. AUDIO-REC1 (listener at the player) and CHAR-REC2 assume a legacy-like top-down camera.** They rest on AUDIO-F11 and CHAR-F13, and the file itself marks the camera "look brief pending". The recommendations are not worded as conditional; only AUDIO-OQ3 and CHAR-OQ5 say so. Minor.
- **L5. BUDGET-REC2's post-processing slot (≤ 4 / 3 ms) and Low pixel-ratio cap of 1 are cost limits, not look picks.** BUDGET-OQ3 already leaves the pixel-ratio cap to Mustafa. Note only.

Other findings:
- **C1. BUDGET-REC1 does not follow from its findings.** Moderate.
  - Intel Gen12+ (BUDGET-F4) applies only to Chrome on Linux, not to a Windows laptop.
  - Firefox on Windows has WebGPU since 141 (WGPU-F18). A Windows reference machine therefore reaches Firefox's WebGL 2 path only with `forceWebGL`. Real fallback users are Firefox on Linux and on Intel Macs (GL2-F12).
  - It names no laptop CPU class, which R1-TICK-OQ6 asks for (its pass mark does ask the ADR draft to list the CPU).
- **C2. Two pass marks contradict each other.** CHAR-REC1 allows "posing plus upload … within 2.0 ms per frame on the minimum machine". CHAR-REC2 and BUDGET-REC2's row allow "character animation ≤ 1.0 ms" on the minimum machine at Low. Minor-moderate.
- **C3. CHAR-REC4 adopts render bundles on WebGPU without a measured gain.** GL2-REC1 says a WebGPU-only extra ("compute, storage writes, bundles") enters "only with a named fallback and a measured gain". The fallback is named (merging), the gain is not measured. Minor.
- **C4. GL2-REC1 overreads GL2-F13.** "Not rare edge cases (F12, F13)" uses a statistic about Direct3D 11.1, which is what compatibility mode targets, as if it were a share of fallback users. The file's own GL2-OQ2 says that share is unknown. The conclusion still follows from F2, F3 and F9. Minor.
- **C5. Gaps in "Rests on:" lists.**
  - WGPU-REC3 mark (1) and LOOK-REC1's mark use MEASURE-F3, which is not listed.
  - MEASURE-REC2 cites "R1's identical-results work" without an id (R1-DET-REC1 / REC6).
  - Minor.
- **C6. Some recommendations trace to no VISION_V2 §6 item and do not say so.**
  - WGPU-REC2, WGPU-REC4 (backend logging and overlay), GL2-REC3, BUDGET-REC1 and MEASURE-REC3 serve §4 only, without the explicit "no §6 item" note R1 uses.
  - AUDIO-REC1's "§3 (hearing where a fight is)" is not in §3.
  - AUDIO-REC3's "§6.2" is the simulation's floors and line of sight, not audio. Spatial audio traces to the brief's R3 row, not to §6.
  - Minor.
- **C7. Missing options a reader would expect.**
  - Anti-aliasing: MSAA against FXAA, SMAA or TRAA, given the MSAA × MRT memory (LOOK-F14) and the aliasing of screen-space outlines.
  - Shadow technique (one map against cascades), although BUDGET-REC2 budgets a shadow pass.
  - A "WebGPU required, no fallback" reference row in question 2.
  - AudioWorklet (our own mixer or spatializer) in question 7.
  - GPU-driven culling or indirect draws in question 4.
  - Minor.
- **C8. Findings marked "confirmed" without a quote:** GL2-F11 (version numbers only), BUDGET-F6, and MEASURE-F6 (derived from GL2-F4 and F5 but marked "confirmed in the sources"). Nits.
- **C9. Number nits.**
  - WGPU-OQ5 says releases arrive "every two to five months (F1)", but F1's dates give gaps of about 1.9, 2.5 and 2.8 months; "five" is unsupported.
  - MEASURE-F3 and BUDGET-F6 put programs and render targets among the byte sizes; the docs give them as counts.
  - CHAR-REC4's mark uses 3 large units; BUDGET-REC2 and R1-TICK use 4.
  - AUDIO-F4 → AUDIO-REC2: a node at zero gain still costs a voice, so "never reaches zero" is not why a cap is needed (the conclusion stands).
- **C10. No silent contradiction of R1 or R2.** The cited ids are consistent with R1 and R2: R1-TICK-REC5, R1-BND-REC2, R1-BND-REC5, R1-XU-OQ3, R1-TICK P2/OQ6/OQ7, R2-INT-REC1, R2-SYNC-REC4.
- **C11. No recommendation's "Rests on:" list names a source the file lists as not read.** One small overreach: CHAR-REC1's "documented … on both backends (F1, F3)" leans on the Skeleton page, but how WebGPURenderer skins is unconfirmed (CHAR-OQ1). Minor.

**Label**
- M = 77: the distinct R3 finding ids named in the 25 "Rests on:" lists (legacy and wave-1 ids not counted).
- N = 44 checked.
- New status label: "spot-checked (44 of 77 key claims)".

**More than nits?** Yes. No claim is wrong at its source, but L1, L2, C1 and C2 are real reasoning and consistency problems in the recommendations and pass marks. Under the plan that means a second pass, at least of the owner's fixes for L1, L2, C1 and C2.

**Not verified**
- Claims not on the check list, including WGPU-F2, F12, F13, F19, F20 and F21.
- The Firefox release dates in WGPU-F18.
- The maxDistance parts of AUDIO-F3.
- The `trackTimestamp` option's doc string (not in the two backend files I fetched).
- Nothing about GPU timings could be checked: no source gives them, as the file itself says.

Notes: `/home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r3.md`. Not committed.
