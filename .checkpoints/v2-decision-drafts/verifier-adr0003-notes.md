Task: verifier for ADR 0003 (rendering and presentation), v2-decision-drafts plan step 7 (brief prompts/verifier-adr0003.txt).
- 09:08 Amsterdam: started fresh (no notes existed). Next: read ADR 0003, ledgers, plan rules, advisor conditions, findings-checked.tsv.
- 09:15: SOURCE CHECKS done (21 curl fetches 09:09:45-09:09:52, all HTTP 200; audit in .gate/verifier-adr0003/fetch-audit.tsv; pages in .gate/verifier-adr0003/fetch/). Verdicts (quotes found by script q.py in scratch):
  WGPU-F12 confirmed (RenderPipeline docs: "This is enabled by default but it must be disabled for effects that expect to be executed after tone mapping and color space conversion. A typical example is FXAA which requires sRGB input."; "can only be used with WebGPURenderer").
  WGPU-F20 confirmed (MDN WebGPU_API: "This feature is not Baseline..."; Security requirements "The whole API is available only in a secure context").
  GL2-F3 confirmed incl. inferred parts (GLSLNodeBuilder.js r186 l.105 `storageBuffer: false`; setupPBO l.385ff builds `new DataTexture( attribute.array, width, height...)`; subgroupSize error quote; no atomic/barrier functions in file - absence).
  GL2-F10 confirmed (WebGLBackend.js r186 l.2272 shared depth/MSAA warning; EXT_clip_control warning).
  LOOK-F1 confirmed (MeshToonNodeMaterial docs "Node material version of MeshToonMaterial"; setupLightingModel(): ToonLightingModel).
  LOOK-F2 confirmed (ToonLightingModel.js r186: `return vec3( gradientMap.r );`, `vec2( dotNL.mul( 0.5 ).add( 0.5 ), 0.0 )`, "comic-like, flat look", AO mulAssign).
  LOOK-F4 confirmed (ToonOutlinePassNode docs "Only 3D objects with materials of type MeshToonMaterial and MeshToonNodeMaterial will receive the outline.").
  LOOK-F5 (verified set; re-read for F8): updateBefore l.85-112 calls renderer.renderObject twice (outline material l.100, default l.108) -> "one extra draw per outlined object" holds.
  LOOK-F6 confirmed (SobelOperatorNode.js r186: doc line, "fetch the 3x3 neighbourhood", magnitude). LOOK-F8 (inferred from F5,F6,F7) holds on these sources.
  LOOK-F7 confirmed (PassNode docs "can provide multiple outputs via MRT"; getViewZNode, getLinearDepthNode); "not a shipped node" is inference (no edge node besides Sobel seen).
  LOOK-F9/F11: GTAONode docs code example has `const prePass = pass( scene, camera )` then `// scene pass const scenePass = pass( scene, camera )` -> scene drawn twice: F11 confirmed as inference from the documented example.
  LOOK-F12 confirmed (BloomNode docs: "For a selective bloom, use the emissive material property ... This can be achieved via MRT."; radius "Must be in the range [0,1]").
  CHAR-F6 confirmed (BatchedMesh docs: quote, perObjectFrustumCulled default true, sortObjects, setColorAt (per-instance colour), setGeometryAt throws; no "skin" on page).
  CHAR-F8 confirmed (webgpu_skinning_instancing.html r186: GLTFLoader loads models/gltf/Michelle.glb l.84, AnimationMixer l.88, instanceCount = 30 l.93, child.isInstancedMesh = true l.113, THREE.WebGPURenderer l.138).
  CHAR-F10 confirmed (LOD docs: "basic Level of Detail (LOD) mechanism", "at the distances specified", hysteresis quote).
  BUDGET-F2 confirmed (MDN rAF: "will generally match the display refresh rate ... 75hz, 120hz, and 144hz are also widely used").
  BUDGET-F3 confirmed (MDN devicePixelRatio: ratio quote; "a value of 2 is expected for HiDPI/Retina displays").
  MEASURE-F9 confirmed (GitHub docs larger runners: "GPU-powered runners"; "billed at the per-minute rate").
  AUDIO-F7 confirmed (MDN spatialization basics: StereoPannerNode quote, "nowhere near as versatile", "does some very involved math under the hood"). NB: "lowest CPU" in ADR is an inference from "much simpler" - check wording.
  WGPU-F19 confirmed (WebKit blog 17333: "is now shipping in Safari 26.0 for macOS, iOS, iPadOS, and visionOS."; supersedes quote; Sonoma). Post date string not found in stripped text (minor).
  WGPU-F14/F15 confirmed (WebGPURenderer docs: outputBufferType HalfFloatType quote; "4 samples are used by default").
  Next: read the ADR body section by section, check claims vs research lines, write verdicts file.
- 09:2x: ADR lines 1-397 read (header, Context, Decision 1-11). Checked vs r3 Options 774-818, RECs 900-1280, legacy facts 193-205, 513-524, 760-770, INV 97/205, STATUS 94-202, brief line 8: all confirmed so far (rows 1-40 context, 41-163 decision) except notes:
  N1 (D5b, row 73): "can mark folds inside a shape" attributed to r3 Options Q3 lines 810-813: those lines do not say it (inference from F7, which the ADR elsewhere marks inferred / inner lines not researched) -> unsupported at cited place; fix: add "(inferred)".
  N2 (D7 draw cap): R3's 786 counts no full-screen post passes (bloom 12 passes F13, edge pass, AO, denoise); margin to 800 is ~14; whether renderer.info counts quad passes is not researched -> note only (R3 calls its counts conservative, r3:1151-1152).
  RTX 5080: ADR avoids R3's "headroom" word (r3:1101) - good (rule 5).
Next: r3 OQs 1282-1430, Pointers 1460-1510, then ADR 398-611 (trace table, Options).
