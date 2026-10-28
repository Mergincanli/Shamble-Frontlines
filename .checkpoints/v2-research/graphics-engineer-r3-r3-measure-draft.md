# R3-MEASURE draft: how we measure it

Question 6 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S48–S52; S5, S6, S16, S17, S19, S23, S26 from earlier questions).

## Findings

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
kind with byte sizes (`texturesSize`, `attributesSize`, `programs`, `renderTargets`, `total`: "The total memory size in
bytes."); an app with its own loop sets `autoReset` to false and "must then call `renderer.info.reset()` once per frame
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
· Source: R3-GL2-F4, F5 · official · read 2026-10-28 · confirmed in the sources.

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

### Legacy facts used
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

## Options

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

## Recommendations

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

**R3-MEASURE-REC2. A fixed benchmark scene: a recorded match replayed from its inputs (R1's identical-results work) at the
largest mode's worst case (R3-BUDGET-REC2: 48 soldiers in one fight, 4 large units, 200 projectiles), with a scripted
camera path, started from a URL flag on any build, including preview links (R5).**
Reason: numbers compare only if every run draws the same frames; legacy's clustered worst case (TL15) and R1-TICK's scale
scene give the load; preview links make it runnable on any machine with a browser.
Rests on: R3-MEASURE-F3; legacy TL15; R3-BUDGET-REC2 (cited); wave 1 R1-TICK's spike scene (cited).
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
Serves: VISION_V2 §4.
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

## Open questions

- **R3-MEASURE-OQ1. Timestamp-query support in Firefox and Safari.** No source read says whether their WebGPU exposes
  `timestamp-query`. Decided by: `renderer.hasFeature` on both in the spike.
- **R3-MEASURE-OQ2. Self-hosted reference machines in CI.** Whether the reference machines become self-hosted runners
  (R5, R6) or are run by hand is a cost and security question. Decided by: R5 and R6.
- **R3-MEASURE-OQ3. GPU memory on integrated graphics.** WebGPU gives no memory budget; `renderer.info.memory` counts
  what three.js allocated (F3), and the adapter's `memoryHeaps` appear only with Chrome's developer flag (F2). Decided
  by: the spike reads both on the minimum machine.
- **R3-MEASURE-OQ4. Cross-origin isolation for profiling.** Isolation gives 5 µs CPU timers (F7) but not finer GPU timers
  (F1); R1-BND-REC5 allows it in development and previews. Decided by: the R1-BND-REC5 route already recommended; no
  new need from R3.

### Recommendations that hinge on a source not read
| Recommendation | Source | Reason not read |
|---|---|---|
| R3-MEASURE-REC3 | GitHub's runner hardware table ("supported runners and hardware resources") | linked page not fetched; GPU details not needed for the recommendation |

## Sources
Read (all 2026-10-28): S48 Renderer.js r186 (official, source); S49 WebGPURenderer.js r186 (official, source); S50 MDN
performance.now (official); S51 Chrome "Supercharge Web AI model testing" (official); S52 GitHub larger runners
(official). Reused: S1, S5, S6, S16, S17, S19, S23, S26, S46.
Not read: the table row above.

## Cross-topic notes
- R5: REC3 is a gate v2 check (counts, not timings); REC2's URL flag runs on preview links.
- R1: REC2 relies on R1's identical-results replay; if replay is not identical on the client, the benchmark falls back
  to a scripted bot match with a fixed seed (less exact).
- R6: results logs could live with other telemetry.
