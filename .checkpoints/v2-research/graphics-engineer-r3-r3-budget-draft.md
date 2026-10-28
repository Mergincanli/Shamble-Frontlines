# R3-BUDGET draft: budgets for the minimum and recommended hardware

Question 5 of R3. Author: graphics-engineer (Opus 5.5, effort high), 2026-10-28. Source summaries with exact quotes:
`.checkpoints/v2-research/graphics-engineer-r3.md` (S45–S47; S12, S14 and earlier).

**Everything in this section is proposed.** The targets come only from VISION_V2 §3, §4 and §6: "a laptop with
integrated graphics runs the largest mode at 30+ fps on Low" and "a mid-range graphics card runs it at 60 fps on High";
the goal mode is "12v12v12 or 24v24" (36 or 48 players; bots fill empty slots), and CLAUDE.md says to design for 48+; §6.9 asks for "a few very large
units and their effects without dropping frames". The reference machines and every number below are proposals for the
ADR draft (`docs/adr/0003`), to be confirmed by Mustafa.

## Findings

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

**R3-BUDGET-F6.** `renderer.info` reports draw calls and triangles per frame and GPU memory by kind (textures,
attributes, render targets, programs, total), so every budget below except GPU time can be read from the renderer
itself (R3-MEASURE-F3).
· Source: https://threejs.org/docs/pages/Info.html · official · read 2026-10-28 · confirmed in the source.

**R3-BUDGET-F7.** No allow-listed source gives GPU throughput for integrated or mid-range GPUs under WebGPU or WebGL 2,
or a hardware share among players (the Steam Hardware Survey and vendor spec pages are off the allow list).
· Source: the sources read for R3 · confirmed by absence.

### Legacy facts used
- **AUDIT_BUGS_PERF §1** (2026-09-26): `update()` 0.49 ms per frame CPU; draw calls including the shadow pass: village
  303, manor 238, centre 170, base 140; at most 12k triangles; P5: "Draw calls ~300/frame in the village (driver
  overhead on weak machines)", rated medium. **CL9**: village 202 after merging (STATUS health), triangles up to +12 %
  "still under 18k".
- **TL15** (reports/2026-10-16-three-teams-core.md:48–50): 48 soldiers' sim step 0.80–1.00 ms mean (p95 0.97–1.54);
  draw calls 1,480 at 12 and 5,843 at 48 with every soldier on screen; 16 shader programs.
- **GA15**: soldier animation 26 to 51 µs each (1.2 to 2.4 ms at 48 by arithmetic).
- **CD15**: pixel ratio capped at 2 with MSAA on; the row asks a spike to measure ratio 1 and 2 on integrated graphics.
  **CD17**: one 2048 × 2048 sun shadow map over a 100 m frustum.
- **STATUS health**: JS heap 42.7 MB at boot (editor rebuild row, 2026-10-14).
- **Phase-0 report** (docs/reports/2026-10-13-phase-0.md:9): Mustafa's PC has an RTX 5080, a high-end card, above the
  "mid-range" class of VISION_V2 §4.
- **R1 wave 1**: R1-TICK's spike scene (48 soldiers with full AI, 4 big vehicles, 200 live projectiles) and its client
  mark "the page's frame time p95 with the sim in the worker is no worse than with no sim at all by more than 1.0 ms"
  (R1-TICK-REC5's and R1-BND's marks); R1 asks for the reference hardware "to be named with R3, R5 and R6".

## Options

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

## Recommendations

**R3-BUDGET-REC1. Reference machines (proposed, M-A with M-B as the fallback): the minimum is one named laptop with
integrated graphics of Intel Gen12 (Iris Xe) class or the AMD integrated equivalent, a 1920 × 1080 screen, Windows 11,
run on Chrome and Firefox; the recommended is one named desktop with a mid-range discrete card (one or two generations
old), at 1920 × 1080 and 2560 × 1440. Mustafa's RTX 5080 is a headroom and Brave check, never the recommended
reference.**
Reason: VISION_V2 §4 names classes, not models, and no allow-listed source ranks GPUs (F7), so the models have to be
picked and named in the ADR; the only GPU class an official source ties to WebGPU support is Intel Gen12+ (F4); the
RTX 5080 is far above mid-range (legacy phase-0 report); Windows plus Chrome and Firefox covers both WebGPU and, on
Firefox's fallback paths, the hardest WebGL 2 case (R3-GL2-REC2). R1 asks for the same machines (R1-TICK's P2).
Rests on: R3-BUDGET-F4, F7; R3-GL2-F7, F8; legacy phase-0 report.
Serves: VISION_V2 §4.
Spike must prove (proposed): the two machines are named in the ADR draft with their GPU, CPU, RAM, screen and
`devicePixelRatio`, and every R3 pass mark is reported on both.

**R3-BUDGET-REC2. Budgets (proposed, option B-2), measured in the worst case of the largest mode: 48 soldiers on screen
in one fight, 4 large units, 200 live projectiles and their effects (R1-TICK's scene), on the reference machines.**

| Budget (all passes of one frame) | Minimum laptop, Low, 30+ fps | Recommended card, High, 60 fps |
|---|---|---|
| Frame time, p95 | ≤ 33.3 ms | ≤ 16.7 ms |
| Frame time, median | ≤ 28 ms | ≤ 14 ms |
| Main-thread CPU per frame (render submission, interpolation, animation, UI) | ≤ 16 ms | ≤ 8 ms (inside F1's 10 ms) |
| of which: character animation (R3-CHAR-REC2) | ≤ 1.0 ms | ≤ 2.0 ms (more soldiers on the full rig) |
| of which: sim cost seen by the page (R1-TICK, R1-BND) | ≤ 1.0 ms | ≤ 1.0 ms |
| GPU time, total | ≤ 25 ms | ≤ 12 ms |
| of which: shadow pass | ≤ 3 ms (one map, 1024² or off) | ≤ 2 ms |
| of which: main scene pass | ≤ 16 ms | ≤ 6 ms |
| of which: post-processing (whatever the look brief keeps) | ≤ 4 ms | ≤ 3 ms |
| of which: effects, sprites, HUD in the 3D view | ≤ 2 ms | ≤ 1 ms |
| Draw calls (`renderer.info.render.drawCalls`) | ≤ 800 | ≤ 1,500 |
| Triangles (`renderer.info.render.triangles`) | ≤ 500k | ≤ 1.5M |
| GPU memory (`renderer.info.memory.total`) | ≤ 512 MB | ≤ 1 GB |
| Render resolution | `devicePixelRatio` capped at 1, scene pass resolution scale 0.75–1 | ratio capped at 2 |
| JS heap after 10 minutes | ≤ 150 MB, no growth over the last 5 minutes | same |

Reason: frame times follow VISION_V2 §4 directly; the median marks leave room for spikes. The CPU split keeps the
page inside web.dev's 10 ms at 60 Hz (F1) and gives R1's sim mark its 1.0 ms. Draw calls are set well below legacy's
5,843 at 48 (TL15) and a few times above its village 202–303, which legacy already judged a medium risk on weak
machines (AUDIT P5). The resolution cap follows F3 and CD15 (ratio 2 shades four times the pixels). Memory follows the
render-target arithmetic (F5): at 1080p a half-float colour target is 16.6 MB, so a post chain with MSAA, two MRT outputs and bloom can reach
about 100 MB or more (an estimate, not a measurement). The triangle and memory figures are the loosest: no source gives them (F7), so the spike either confirms or
moves them.
Rests on: R3-BUDGET-F1, F2, F3, F5, F6, F7; R3-LOOK-F14; legacy AUDIT_BUGS_PERF §1, CL9, TL15, GA15, CD15, CD17; wave 1
R1-TICK's scene and client mark.
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

## Open questions

- **R3-BUDGET-OQ1. Which models.** No allow-listed source ranks GPUs or gives the players' hardware mix (F7). Decided
  by: Mustafa and the Advisor (what can be bought or borrowed), recorded in the ADR draft; R6's telemetry after launch.
- **R3-BUDGET-OQ2. High-refresh screens.** At 120 or 144 Hz (F2) the render loop runs more often than 60 fps; whether
  High caps the frame rate at 60 or runs free is open. Decided by: the ADR (a cap saves power on laptops).
- **R3-BUDGET-OQ3. HiDPI laptops on Low.** Many integrated-graphics laptops ship with ratios above 1 (F3); a cap at 1
  softens text in the 3D view. Decided by: the spike's GPU time at ratio 1 and 2 (CD15's request) and Mustafa's look.
- **R3-BUDGET-OQ4. Largest N.** VISION_V2 §3 starts small ("two teams with 2–8 players, and three teams with 6–18 players") and names 12v12v12 or 24v24 as the goal (36 or 48 players; CLAUDE.md: design for 48+); R1-TICK's
  OQ7 decides the N of the offline scale test. The draw-call and triangle rows scale with N.
- **R3-BUDGET-OQ5. Triangle and memory rows.** They have no source (F7). Decided by: the spike's measurement and the
  character rework's real models (R3-CHAR-OQ3).

### Recommendations that hinge on a source not read
| Recommendation | Source | Reason not read |
|---|---|---|
| R3-BUDGET-REC1 | Steam Hardware & Software Survey (store.steampowered.com) | outside the allow list |
| R3-BUDGET-REC1 | Intel, AMD and NVIDIA product specification pages | outside the allow list |

## Sources
Read (all 2026-10-28): S45 web.dev "Rendering performance" (official); S46 MDN requestAnimationFrame (official); S47 MDN
devicePixelRatio (official). Reused: S6 Info, S12 GPU.json. Repo: docs/reports/2026-10-13-phase-0.md,
docs/AUDIT_BUGS_PERF.md §1, docs/reports/2026-10-16-three-teams-core.md.
Not read: the two rows of the table above.

## Cross-topic notes
- R1: REC2's scene is R1-TICK's spike scene, so the client and the sim are measured on the same load; REC1's machines
  are the ones R1 asks for.
- R6: the hardware mix and backend share of real players come from telemetry (OQ1).
