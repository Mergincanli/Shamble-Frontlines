# Pass marks for ADR 0003 (rendering and presentation), test-engineer, 2026-10-31

Plan step 4. Two parts: the `### Pass marks` subsection, which the session swaps in verbatim for the draft's
placeholder (`### Pass marks` and its one line, `graphics-engineer-adr0003-draft.md` lines 884–885), and the spike
entries for `docs/SPIKE_PLAN.md`, which stay in this notes file for the synthesis (plan step 6). The claims ledger for
every row is `test-engineer-passmarks-0003-claims.md`.

---8<--- the subsection starts on the next line ---

### Pass marks
Written by test-engineer (plan step 4). One row per mark: the look proof test's first (PM-0003-1 to PM-0003-17), then
the render budget bench's (PM-0003-18 to PM-0003-42), then the check at each later three.js upgrade (PM-0003-43). Every
number is a research mark (cited), arithmetic on research numbers (shown), or marked proposed with its reason. Line
numbers are r3-rendering.md's at 9e7db28 unless another file is named.

**How to read the table**
- **Machines** (R3-BUDGET-REC1, r3-rendering.md, Recommendations Q5, lines 1096–1102; the models are OQ-0003-1):
  **ML**, the minimum laptop: integrated graphics of Intel Gen12 (Iris Xe) class or the AMD equivalent, a laptop CPU of
  at least 4 cores and 8 threads (proposed in the research, no source), 1920 × 1080, Windows 11; its browsers on
  WebGPU and with the WebGL 2 backend forced (`forceWebGL`). **ML-Linux**, the same laptop booted into Linux: Firefox
  there is the real WebGL 2 fallback; Chrome runs WebGPU there only on an Intel Gen12+ GPU (R3-BUDGET-F4, lines
  557–560). **RD**, the recommended desktop: a mid-range discrete card one or two generations old, its CPU named, at
  1920 × 1080 and 2560 × 1440. **MAC**, a macOS machine with Safari 26 for the marks that name Safari; the draft
  names none, so its model is recorded with each result, and it carries no budget row. **MP**, Mustafa's PC (RTX
  5080, Windows) in Brave: only what he judges, and the record of Brave's backend (R3-WGPU-OQ1, lines 1286–1289);
  never a budget result. **CI**, the gate v2's headless runner (W-A): counts only, never a time (R3-MEASURE-REC3, lines
  1206–1209).
- **Scenes.** **BS**, the benchmark scene (part 8; R3-MEASURE-REC2, lines 1195–1197): a recorded match replayed at the
  stress scene's fight on screen (N = 48 soldiers in three teams of 16, the 4 very large units of at least 20 parts
  each, 300 live projectiles with their visual effects), with a scripted camera, started by a URL flag. Until ADR
  0004's sim core spike gives a recorded match, BS is a synthetic recording at the same counts (the draft's own
  fallback, the bench's "Needs"). **LS**, the look-test scene ("one look-test scene", R3-LOOK-REC1, line 991): the look
  proof test's scripted scene with its authored test assets (part 11) at the game camera, at BS's counts wherever a
  mark asks for the fight on screen. It runs before any recorded match exists, so it is a synthetic recording (one
  script, one fixed seed).
- **H**, a harness run set (part 8; R3-MEASURE-REC1, lines 1181–1185): 3 runs of 600 measured frames after 300
  warm-up frames, each run labelled as R3-MEASURE-REC1 asks (backend, adapter, browser version, `devicePixelRatio`,
  resolution, GPU times quantized or not) plus the build's commit, the display's refresh rate, mode U or P, the tier,
  and mains power with the operating system's power mode (proposed: a laptop's power state moves its numbers). Every
  one of the 3 runs must meet the mark ("holds … for 3 runs", R3-BUDGET-REC2, lines 1162–1163).
- **Statistics.** Percentiles by nearest rank over a run's 600 frames: p99 is the 594th smallest value, p95 the 570th,
  the median the 300th (proposed: one definition, so two people get the same number). A time mark that names no
  statistic is a p99 ([time statistics](../research/GLOSSARY.md#time-statistics)); p99s are never added, so every
  total is measured directly. A count mark holds on every frame, so its statistic is the run's max (proposed reading:
  counts are exact, and the replay repeats them, PM-0003-35).
- **V**, the validity of a timed run set: its 3 run medians lie within 3 % of each other (R3-MEASURE-REC1's mark,
  lines 1192–1193, applied to every timed set: proposed). An invalid set is neither a pass nor a fail: its cause is
  found and written down before it is measured again.
- **U and P.** **U**, uncapped: every frame-time and CPU-time mark that sets a budget or compares two builds or two
  backends runs with the browser's frame-rate limit and vsync off (proposed). Why: on a 60 Hz display a presented frame
  lasts a whole number of 16.7 ms refreshes, so the recommended column's median of at most 14 ms could never pass,
  the minimum column's median of at most 28 ms would demand half the frames at 60 fps, and two builds that both reach
  60 fps would compare as equal (PM-0003-4, PM-0003-37 could not fail). The switch for each browser is not researched:
  the harness's preparation reads each browser's documentation first; a browser without one runs presented with its
  refresh rate recorded, and a mark below one refresh period is reported "not provable on this browser". **P**,
  presented: normal vsync at the display's refresh rate, only where the mark is about what a player sees (sound onset,
  frame pacing).
- **Timers.** Frame and CPU times from `performance.now()`. A mark under 1 ms runs on a profiling build with
  [the isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag) on, whose timers have 5 µs
  resolution instead of 100 µs (R3-MEASURE-F7, lines 641–646; R3-MEASURE-OQ4, lines 1389–1391); without it, such a mark
  passes only with 0.1 ms to spare (proposed: one timer step). GPU times come from timestamp queries (T-A, then T-B);
  per-pass times on the WebGPU backend run with Chrome's WebGPU Developer Features flag on, so they are not quantized
  to 100 µs (R3-MEASURE-F2, lines 610–615; R3-MEASURE-REC4, lines 1217–1219). Where a browser has no GPU timer, the
  harness reports "GPU time not available" and the GPU rows are not checked on that browser (R3-GL2-REC3, lines
  979–980).
- **Tick.** The Worker's sim (a stand-in until the real one) runs at 30 Hz (TICK-30), so the page receives 30 blocks
  per second. No mark here is set at 60 Hz: [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) is a server
  mark, ADR 0001's.
- **Bites.** Each automated mark names a planted defect that must make it fail on the spike's own build before its
  pass counts, because "a check without a failing case can't be shown to work" (R5-GATE-REC3, r5-toolchain-quality.md
  lines 1727–1728). The planted defects are proposed. **Rule B**, for a budget mark: a per-frame busy-wait (or, for a
  GPU row, an extra full-screen pass in the pass under test) sized to the mark's measured margin plus 1 ms must fail
  it.

| mark | what it proves (decision part) | number and statistic | hardware | method and tool | runs and noise | basis |
|---|---|---|---|---|---|---|
| **The look proof test** | | | | | | |
| PM-0003-1 | The machines exist, and every mark that names both is reported on both (part 7; a precondition of both spikes) | ML and RD named in this ADR with GPU, CPU, RAM, screen, `devicePixelRatio` and `hardwareConcurrency`; MAC's model recorded; every mark naming ML and RD has results from both | ML, RD, MAC | a results-log header per machine, written before the first run; every later log row names its machine | checked before the look proof test's first run and again at the bench | R3-BUDGET-REC1's mark (lines 1114–1116); R1-BND-OQ3 (r1-simulation-core.md, lines 2728–2731) |
| PM-0003-2 | One build runs on both backends in every target browser (part 1) | 0 console errors in every start | ML: Chrome, Edge and Firefox (Windows), each on WebGPU and with `forceWebGL`; MAC: Safari 26, both backends | LS played through its whole script; the page's own error hook writes every console error and page error to the results log (no browser driver needed) | the 10 starts of PM-0003-5 per browser and backend; 0 in each. Bites: a `console.error` 2 s after load (R5-GATE-REC3's G12 mutation, r5-toolchain-quality.md line 1723) | R3-WGPU-REC1's mark 1 (lines 910–912) |
| PM-0003-3 | The fallback shows the same look (parts 1, 5; decides R3-GL2-OQ3) | at each of 3 fixed frames, at most 1 % of pixels differ by more than 8/255 in any channel between the two backends (a tolerance the research marks proposed); effects that exist on one backend only are off | ML: Chrome and Firefox, WebGPU against `forceWebGL`; MAC: Safari 26, the same pair | LS's three fixed frames read from the canvas on each backend, same machine and browser; a pixel-diff script counts the pixels with any channel difference above 8 | 1 capture per frame and backend. Control: the same backend captured twice gives 0 differing pixels (else the scene does not repeat and the set is invalid). Bites: the ink switched off on the WebGL 2 backend only | R3-WGPU-REC1's mark 2 (lines 912–913); R3-GL2-REC1's mark (lines 965–967) |
| PM-0003-4 | WebGPU is not slower than the fallback on our scene (part 1; decides R3-WGPU-OQ3) | median frame time on the WebGPU backend at most 1.1 × the WebGL 2 backend's, same machine, browser and tier | ML at Low: Chrome, Firefox; RD at High: Chrome | H on LS at the fight on screen, mode U, once per backend; the ratio taken per run pair (run i against run i) | H and V on both sides; all 3 pairs at most 1.1. Bites: a busy-wait of 15 % of the median per frame on the WebGPU path only | R3-WGPU-REC1's mark 3 (lines 913–915) |
| PM-0003-5 | The backend is named correctly everywhere (part 4; decides R3-WGPU-OQ1) | correct in 10 of 10 starts per browser and backend; on ML-Linux, Firefox's overlay names "WebGL 2 fallback"; Brave's backend on MP recorded | ML: Chrome, Edge, Firefox; ML-Linux: Firefox, Chrome; MAC: Safari 26; MP: Brave | each start, the overlay's and the log's string compared with what it should be: "WebGPU" when `navigator.gpu` returns an adapter and WebGL 2 is not forced, else "WebGL 2 fallback"; every test screenshot prints the string (R5-GATE-REC1's G12, r5-toolchain-quality.md line 1670) | 10 starts per browser and backend (forced and not). Bites: a hard-coded "WebGPU" label fails on ML-Linux's Firefox | R3-WGPU-REC4's mark (lines 950–951); R3-BUDGET-REC1's mark (lines 1115–1116); R3-WGPU-OQ1 (lines 1286–1289) |
| PM-0003-6 | GPU timing on the fallback never fakes a number (part 4) | Chrome's WebGL 2 backend: a GPU time above 0 in 600 of 600 frame records; Firefox's and Safari's WebGL 2 backends: "GPU time not available" in 600 of 600, never 0 | ML: Chrome and Firefox with `forceWebGL`; ML-Linux: Firefox; MAC: Safari 26 with `forceWebGL` | one H run on LS per browser; the frame records read from the results log | 1 run per browser (a count over every frame). Bites: a harness that writes 0 when the timer is missing fails on Firefox | R3-GL2-REC3's mark (lines 984–985) |
| PM-0003-7 | The match compiles nothing after the loading screen (part 3) | `renderer.info.memory.programs` equal 5 s after deploy and at the end of a scripted 5-minute match | CI (a count); ML: Chrome on WebGPU and with `forceWebGL`, Firefox on WebGPU | the 5-minute script shows every unit type, visual effect, damage stage and roof fade at least once (part 3's warm-up list); the counter read at both points | 3 runs; equal in each. Bites: the roof fade's blend variant left out of the warm-up | R3-WGPU-REC3's mark 1 (lines 937–938); R3-MEASURE-F3 (lines 617–622) |
| PM-0003-8 | No first-sight stutter (part 3) | no frame above 1.5 × the run's median frame time in the 1 s after the first appearance of each unit type, visual effect, damage stage and roof fade (the 1 s window proposed: pipelines are created asynchronously, R3-WGPU-F22, lines 188–192, so a hitch can land a few frames late) | ML at Low: Chrome on WebGPU and with `forceWebGL`, Firefox on WebGPU | PM-0003-7's script, mode U; frame times from the harness; each first appearance logged with its frame number | 3 runs, each from a fresh browser profile (proposed: a browser's disk cache of compiled shaders could hide a missing warm-up variant, so the mark could not fail; not researched); 0 frames over in each run. Bites: as PM-0003-7, and a material created at a unit type's first sight | R3-WGPU-REC3's mark 2 (lines 938–939); the damage stages and the roof fade are the draft's additions (part 3) |
| PM-0003-9 | Each look technique's cost, alone (part 5; decides R3-LOOK-OQ1 and R3-LOOK-OQ4) | reported, no pass number: per technique and variant, GPU ms (median and p95), extra draw calls, extra render-target memory (`renderer.info.memory`, with `texturesSize`); passes when every cell holds a number or "not available" | ML at Low and RD at High, Chrome on WebGPU and with `forceWebGL`, 1920 × 1080 | LS with only that technique on, against LS with every technique off; the techniques of part 5: toon ramps (5.a), both ink families (5.b), AO (5.c), glow (5.d), the shadow map (5.f), the roof fade (5.i), visual effects (5.k), halftone and line boil (5.l), MSAA (5.n) | H per technique and variant, V | R3-LOOK-REC1's mark (lines 998–1000); R3-LOOK-OQ4 (lines 1330–1331) |
| PM-0003-10 | No effect re-renders the scene unplanned (part 5) | with every kept effect on, `drawCalls` at most 1.1 × (the draw calls with every effect off + the planned extras: one per outlined object for a hull, one per drawn object for an AO pre-pass), on every frame | CI (a count); ML, both backends | LS, the same frames with effects off and on; `renderer.info.render.drawCalls` reset once per frame | 1 run per configuration (counts repeat, PM-0003-35). Bites: an effect that renders the scene a second time | R3-LOOK-REC2's mark (lines 1014–1016) |
| PM-0003-11 | Every effect's tier switch saves real time (parts 5, 7) | each effect's off setting and its lowered setting each cut that effect's own GPU pass time by at least 40 % (p99: the research names no statistic, so GLOSSARY §1's rule applies; the median is reported) | ML at Low, Chrome on WebGPU and with `forceWebGL` | H on LS per effect and setting; the effect's own pass time from per-pass timestamps | H, V. Bites: a "lowered" setting that changes no resolution scale | R3-LOOK-REC2's mark (lines 1016–1017) |
| PM-0003-12 | Both ink families are measured, and each fits its draw cap at Low (part 5.b; part 7) | reported: GPU and frame time (median, p95) at 12, 24 and 48 soldiers on screen for each family; gated: at 48 soldiers, `drawCalls` at most 800 with the screen-space ink and at most 1,100 with the hull (the research's "about 1,100", read as 1,100), on every frame | ML at Low: Chrome on WebGPU; ML-Linux: Firefox (the count's worst case, R3-GL2-REC2) | LS at 12, 24 and 48 soldiers on screen, each family alone, mode U | H, V for the times; every frame for the counts. Bites: a fifth draw per soldier per pass (5 × 48 × 2 = 480 instead of 384: 882 draws with the screen-space ink; 1,202 with the hull, which outlines 48 more objects) | R3-LOOK-REC3's mark (lines 1026–1027); R3-BUDGET-REC2 (lines 1133, 1142–1148): 4 × 48 × 2 + 202 + 200 = 786; 786 + 192 + 80 = 1,058 |
| PM-0003-13 | The kept look fits the frame (parts 5, 7; R3-GL2-REC1's "meets the minimum budget" for the frame rows) | ML at Low: frame time median at most 28 ms and p95 at most 33.3 ms; RD at High: median at most 14 ms and p95 at most 16.7 ms | ML: Chrome on WebGPU and with `forceWebGL`, Firefox on WebGPU; RD: Chrome on WebGPU at 1920 × 1080 and 2560 × 1440 | H on LS at the fight on screen with the techniques Mustafa keeps (PM-0003-17), mode U | H, V; rule B | R3-BUDGET-REC2's rows (lines 1123–1124); R3-GL2-REC1's mark (lines 965–967); VISION_V2 §4: 1000 / 30 = 33.3 ms, 1000 / 60 = 16.7 ms |
| PM-0003-14 | What a pixel ratio of 1 or 2 costs on Low (part 7; feeds OQ-0003-12) | reported, no pass number: GPU time (median, p95) at ratio 1 and at ratio 2 | ML at Low, Chrome on WebGPU | H on LS with the renderer's pixel ratio set to 1, then 2 | H, V | R3-BUDGET-OQ3 (lines 1368–1369; CD15's request, line 587) |
| PM-0003-15 | The lush patch and the props fit the counts (parts 5.h, 6; OQ-0003-9) | over the lush patch (fields, logs, trees with variety) at the game camera: `drawCalls` at most 800 and triangles at most 500k, on every frame | CI (counts); ML at Low | LS's lush patch under its scripted camera; `renderer.info` per frame | 1 run. Bites: a planted unmerged copy of the prop field | R3-BUDGET-REC2's rows (lines 1133–1134); the draft's OQ-0003-9 ("a lush test patch within the draw-call and triangle rows") |
| PM-0003-16 | The look proof test's records are complete (parts 1, 4, 5; decides R3-WGPU-OQ2, R3-MEASURE-OQ1, R3-MEASURE-OQ3) | recorded: `renderer.hasFeature` for timestamp queries on Firefox and Safari; `renderer.info.memory` and the adapter's `memoryHeaps` (Chrome's developer flag on) on ML; Brave's backend on MP (PM-0003-5) | ML, MAC, MP | read once per browser and written to the results log | passes when complete | R3-WGPU-OQ2 (lines 1290–1293); R3-MEASURE-OQ1 (lines 1382–1383); R3-MEASURE-OQ3 (lines 1386–1388) |
| PM-0003-17 | Mustafa's verdict on the look (part 5; part 11; LOOK_V2 §9) | every LOOK_V2 item in the look checklist below judged "achieved", and a pick recorded for each call he makes; 0 items left "not yet" | MP, Brave on Windows | the look checklist below, each view beside a legacy screenshot at the same framing | one session; an item marked "not yet" goes back to the draft and is shown again | LOOK_V2 §9; plan rule 12 |
| **The render budget bench** | | | | | | |
| PM-0003-18 | The frame-time rows hold (part 7) | ML at Low: frame time median at most 28 ms, p95 at most 33.3 ms; RD at High: median at most 14 ms, p95 at most 16.7 ms | ML: Chrome and Firefox, each on WebGPU and with `forceWebGL`; RD: Chrome on WebGPU at 1920 × 1080 and 2560 × 1440 | H on BS with the kept techniques, mode U | H, V; rule B | R3-BUDGET-REC2's rows (lines 1123–1124) and its mark (lines 1162–1164) |
| PM-0003-19 | The main-thread CPU rows hold (parts 6, 7) | CPU per frame p99 at most 16 ms (ML, Low) and 8 ms (RD, High); of which character animation (posing and bone upload, budget LOD on) p99 at most 1.0 ms (ML) and 2.0 ms (RD) | as PM-0003-18 | H on BS; per frame, `performance.now()` from the animation-loop callback's start to the end of its work, plus that frame's block message handler; animation timed around posing and bone upload, on a build with the isolation flag on | H, V; rule B | R3-BUDGET-REC2's rows (lines 1125–1126); R3-CHAR-REC2's mark (lines 1061–1062) |
| PM-0003-20 | The page's cost of the sim (part 7, N12; part 10) | p99 at most 1.0 ms per frame on both machines. **Which statistic gates (decision map LE-B): both, each in its own spike:** this p99 here, and R1-BND-REC1's mark 1 (the page's p95 frame time with the sim in the Worker no worse than with no sim by more than 1.0 ms, at N = 12, 24, 36, 48 on ML) in ADR 0002's boundary spike. They measure different things: this row times the page's own sim work; R1's difference also catches what timing the page's code cannot see (garbage collection, the Worker sharing the laptop's cores). Passing one does not imply the other | ML at Low and RD at High: Chrome on WebGPU; ML: Firefox on WebGPU | H on BS with the boundary spike's Worker and block (its sim at 30 Hz); per frame, `performance.now()` around the page's block intake (message handler, typed-array views, returning old blocks to the pool) and the command post; isolation flag on | H, V; rule B | R3-BUDGET-REC2's row (line 1127); R1-BND-REC1's mark 1 (r1-simulation-core.md, lines 2259–2261); GLOSSARY §1, the client's sim share (lines 214–218) |
| PM-0003-21 | The GPU-time rows hold (part 7; with OQ-0003-4, decides R3-BUDGET-OQ6's shadow row) | p99, ML at Low / RD at High: total at most 25 / 12 ms; shadow pass 3 / 2 ms; main scene pass 16 / 6 ms; post-processing (ink, AO, glow, comic touches) 4 / 3 ms; visual effects, sprites and HUD in the 3D view 2 / 1 ms; each row measured directly, never as a sum | ML: Chrome on WebGPU and with `forceWebGL`; RD: Chrome on WebGPU at both resolutions; Firefox only where PM-0003-16 found timestamp queries | H on BS; the total and each pass from timestamp queries, developer flag on (WebGPU backend) | H, V; rule B (an extra full-screen pass) | R3-BUDGET-REC2's rows (lines 1128–1132); R3-BUDGET-OQ6 (lines 1375–1378) |
| PM-0003-22 | The count rows hold (part 7; with the first authored models, decides R3-BUDGET-OQ5) | per frame, the run's max: draw calls at most 800 at Low (at most 1,100 if Mustafa keeps the hull at Low), at most 1,500 at High; triangles at most 500k / 1.5M; GPU memory (`renderer.info.memory.total`) at most 512 MB / 1 GB | CI (counts); ML and RD in every browser and backend of PM-0003-18 | `renderer.info` read on every frame of BS, reset once per frame | 1 run per configuration (counts repeat, PM-0003-35). Bites: PM-0003-12's fifth draw per soldier per pass (882 draws at Low) | R3-BUDGET-REC2's rows (lines 1133–1135); R3-BUDGET-OQ5 (lines 1373–1374) |
| PM-0003-23 | The tiers are set as the table says (part 7) | Low: pixel ratio at most 1, scene resolution scale 0.75 to 1, one shadow map at 1024² or none; High: pixel ratio at most 2 | ML, RD | read from the renderer (its pixel ratio, the scene pass's resolution scale, the shadow map's size) at the start and end of every BS run | every run of PM-0003-18. Bites: a tier file that gives Low a ratio of 2 | R3-BUDGET-REC2's rows (lines 1129, 1136); R3-BUDGET-REC3 (lines 1166–1169) |
| PM-0003-24 | The page does not leak (part 7) | JS heap at most 150 MB after 10 minutes; "no growth over the last 5 minutes" read as: the heap after a forced garbage collection at minute 10 at most 0.5 MB above the one at minute 5 (proposed: R5-TEST-REC5's server slope of 1 MB per 10 minutes, r5-toolchain-quality.md lines 1487–1488, over 5 minutes) | ML and RD, Chrome on WebGPU; CI | BS looped for 10 minutes; garbage collection forced and heap use read through the Chrome DevTools Protocol at minutes 5 and 10 (proposed tool; not researched) | 3 runs; each passes. Bites: one array kept per frame (as R5-TEST-REC5's planted leak) | R3-BUDGET-REC2's row (line 1137) |
| PM-0003-25 | One skinned mesh per soldier works on both backends (part 6; decides R3-CHAR-OQ1) | at most 4 draw calls per soldier per pass (before the ink) with 48 soldiers on screen, on every frame | CI (a count); ML: Chrome on WebGPU and with `forceWebGL` | BS: draw calls with the soldiers drawn minus with them hidden, per frame, divided by 48 × the passes that draw them (shadow and main) | 1 run per backend. Bites: a soldier split into 5 meshes | R3-CHAR-REC1's mark (lines 1044–1045) |
| PM-0003-26 | The full rig's cost without the budget LOD (part 6) | posing and bone upload with every soldier on the full rig: p99 at most 2.0 ms per frame | ML: Chrome on WebGPU and with `forceWebGL` | H on BS with the budget LOD off; `performance.now()` around posing and bone upload; isolation flag on | H, V; rule B | R3-CHAR-REC1's mark (lines 1045–1047) |
| PM-0003-27 | Instanced skinning only for a clear win (part 6; decides R3-CHAR-OQ2) | C-D or C-E is adopted only if its total frame time is at least 20 % lower than C-B's on both machines (p99: the research names no statistic, GLOSSARY §1; median and p95 reported) | ML at Low and RD at High: Chrome on WebGPU (C-D, C-E) and with `forceWebGL` (C-E) | H on BS for C-B and each candidate, mode U; the cut taken per run pair | H, V; all 3 pairs at least 20 %. An adoption rule: C-B stands otherwise | R3-CHAR-REC1's mark (lines 1047–1048) |
| PM-0003-28 | Fixed bounds never hide a visible soldier (part 6) | identical pixels on 600 of 600 frames with frustum culling off and on | CI; ML: Chrome on WebGPU | BS rendered twice, culling off then on; frames compared pixel for pixel | 1 run each. Control: two runs with culling on give identical pixels. Bites: a padded sphere smaller than a soldier's pose | R3-CHAR-REC2's mark (lines 1062–1063) |
| PM-0003-29 | Per-instance data makes no garbage, and health bars take few draws (parts 6, 10) | 0 bytes allocated per frame in the character update path over 60 s; health bars for 48 soldiers in at most 2 draw calls | ML: Chrome on WebGPU; CI | heap sampling by legacy's method (TL15: an inspector sampling profile that includes objects collected by minor and major collections, `tools/perfbots.js` lines 9–10), allocations attributed to the character update functions; bar draws counted as bars on minus bars off | 3 runs of 60 s (proposed: R3-BUDGET-REC2's run count); 0 in each. Bites: one planted 64-byte allocation per frame in that path must be reported (proposed: a sampling profile proves "zero" only if it can see a small leak) | R3-CHAR-REC3's mark (lines 1074–1075) |
| PM-0003-30 | The spectacle budget (parts 6, 7; VISION_V2 §6.9) | the 4 very large units add at most 2.0 ms GPU time (p99) and at most 200 draw calls against BS without them, before any ink or AO pre-pass; render bundles stay only if they cut the WebGPU backend's frame time (p99) by more than V's 3 % (proposed: a smaller cut is noise) | RD at High, Chrome on WebGPU | H on BS with the units, then with them hidden; the difference of p99s per run pair; draw calls per frame | H, V; all 3 pairs within 2.0 ms; draws within 200 on every frame. Bites: each unit drawn with 30 parts (4 × 30 × 2 = 240 draws) | R3-CHAR-REC4's mark (lines 1088–1092): 4 × 20 × 2 = 160 draws, plus margin |
| PM-0003-31 | Props: BatchedMesh against InstancedMesh (part 6; decides R3-CHAR-OQ4) | reported, no pass number: CPU and GPU time (median, p95) for the same prop field | ML: Chrome on WebGPU and with `forceWebGL`, Firefox with `forceWebGL` (no multi-draw) | H on LS's lush patch, once per variant | H, V | R3-CHAR-OQ4 (lines 1352–1354); R3-CHAR-REC4 (lines 1083–1084: BatchedMesh is judged on CPU time) |
| PM-0003-32 | The fallback's worst case (part 1) | `drawCalls` within the Low cap (800, or 1,100 with a hull at Low) on every frame, and median frame time at most 33.3 ms | ML-Linux: Firefox (its real WebGL 2 fallback, no `WEBGL_multi_draw`) | H on BS; mode U if Firefox has a switch, else P with the refresh rate recorded | H, V; rule B | R3-GL2-REC2's mark (lines 975–977) |
| PM-0003-33 | The quality tiers bite (part 7) | switching High to Low cuts GPU frame time by at least 50 % (p99: no statistic named, GLOSSARY §1), with `programs` unchanged across the switch | ML: Chrome on WebGPU and with `forceWebGL` | H on BS at High, then at Low after a live switch | H, V. Bites: a Low tier whose table equals High's | R3-BUDGET-REC3's mark (lines 1176–1177) |
| PM-0003-34 | The harness tells the truth (part 8) | when GPU-bound: the GPU total within 2 ms of the frame time minus the main-thread CPU work (median); two back-to-back runs of one build: medians within 3 % | RD: Chrome on WebGPU, developer flag on | H on BS made GPU-bound by raising its resolution scale (proposed: until the median GPU time is at least twice the median CPU work), mode U | 2 back-to-back runs (the mark's own count) and one H set | R3-MEASURE-REC1's mark (lines 1191–1193) |
| PM-0003-35 | The benchmark scene repeats exactly (part 8) | identical `drawCalls` and `triangles` on 600 of 600 frames; identical screenshots at 3 fixed frames | ML and RD: Chrome on WebGPU; CI | BS run twice on one machine; counts compared per frame, screenshots pixel for pixel | 2 runs. Bites: a presentation random draw seeded from the clock (proposed) | R3-MEASURE-REC2's mark (lines 1203–1204) |
| PM-0003-36 | CI catches render regressions, never with a time (part 8; the gate v2's check G12) | the CI job fails on a planted +10 % in draw calls and on a planted +10 % in GPU memory in BS (1 of 1 each); a control under the limit (+5 %, proposed) and R5-GATE-REC3's G12 control pass; the job runs in under 5 minutes; it also fails on a console error, on triangles or programs beyond the committed baseline file, and on any per-frame allocation in the render path | CI (headless; counts only) | BS in headless Chromium against the committed baseline file (R5-GATE-REC4); proven by `gate --prove`'s mutations and controls (R5-GATE-REC3) | every `--prove` run; 1 of 1 per mutation | R3-MEASURE-REC3's mark (lines 1214–1215); R5-GATE-REC1's G12 (r5-toolchain-quality.md line 1670); R5-GATE-REC3 (r5-toolchain-quality.md, lines 1706–1732; G12's row, line 1723); R5-GATE-REC4 (r5-toolchain-quality.md, lines 1734–1744) |
| PM-0003-37 | The overlay costs nothing while closed (part 8) | median frame time with the overlay closed within 1 % of a build without it; open, within 3 %; and, closed, 0 overlay calls and 0 bytes allocated by the overlay per frame (counted: the draft's "zero cost while closed") | ML at Low and RD at High: Chrome on WebGPU | BS, mode U; the two builds' runs interleaved (A, B, A, B …), 10 pairs (proposed: 1 % is below the 3 % by which two back-to-back runs may differ, PM-0003-34, so 3 runs cannot resolve it); the mark reads the median of the 10 paired differences | 10 interleaved pairs per comparison. Bites: an overlay that keeps sampling while closed | R3-MEASURE-REC4's mark (lines 1224–1225) |
| PM-0003-38 | Equal-power voices hold their cost (part 9; decides R3-AUDIO-OQ1) | with 32 concurrent equal-power voices: no dropout in a 5-minute capture, the AudioContext "running" throughout; main-thread audio work under 0.5 ms (p99) per frame; HRTF on 4 priority voices raises that p99 by under 0.3 ms | ML: Chrome and Firefox | BS's fight held at 32 voices; the output recorded and heard through by a listener (the person running the bench); `AudioContext.state` read every frame; audio work timed with `performance.now()`, isolation flag on | 3 runs of 5 minutes (proposed: R3-BUDGET-REC2's run count). Bites: a planted 1 ms busy-wait in the voice update | R3-AUDIO-REC1's mark (lines 1241–1243) |
| PM-0003-39 | The voice cap and its priorities (part 9; decides R3-AUDIO-OQ2) | live source nodes at most the cap (32 at Low, 48 at High) on every frame; shots at the player dropped: 0 in 3 runs; new `AudioBuffer`s after warm-up: 0; reported: how many voices the fight asks for before the cap | ML at Low and RD at High: Chrome | BS's fight; counters around node and buffer creation, and the voice manager's drop log | 3 runs (the mark's count). Bites: a priority table that ranks shots at the player last | R3-AUDIO-REC2's mark (lines 1254–1256) and caps (lines 1245–1247) |
| PM-0003-40 | Occlusion rays, only if O-B is ever proposed (part 9) | at most 1 ray per voice per 100 ms; all rays at most 0.2 ms (p99) per frame at the voice cap; 4 of 5 testers tell an occluded source from an open one in a blind test | ML at Low | H on BS with O-B on; a blind listening test with 5 testers | not run while part 9 proposes zones or none | R3-AUDIO-REC3's mark (lines 1267–1269) |
| PM-0003-41 | Sound in step with the picture (part 9) | sound onset after the drawn event at most one rendered frame plus the output latency, p99 over 100 shots (the 99th by nearest rank); 0 autoplay warnings in the console on all four browsers | ML: Chrome, Edge, Firefox; MAC: Safari 26; mode P | 100 shots in BS; per shot, the time the muzzle flash is drawn against the time the sound leaves the output, mapped with the AudioContext's output timestamp and output latency (proposed tools; not researched) | 1 run of 100 shots per browser (the mark's count). Bites: sounds started from the next block's events (one tick late) | R3-AUDIO-REC4's mark (lines 1279–1280) |
| PM-0003-42 | Frame pacing and power for the frame-rate cap (part 7; feeds OQ-0003-10) | reported, no pass number: per option (a 60 fps cap; follow the display; a cap per tier), R1-TICK-REC5's smoothness (the standard deviation of a steady unit's per-frame displacement as a share of its mean) on 60, 120 and 144 Hz displays; on ML, battery drain over 10 minutes per option (proposed method) | RD on the displays available (each recorded); ML on battery | BS, mode P | 1 run per option and display | R3-BUDGET-OQ2 (lines 1366–1367); R1-TICK-REC5's mark (r1-simulation-core.md, lines 2111–2112) |
| **Later, at each three.js upgrade** | | | | | | |
| PM-0003-43 | An upgrade changes no cost without a reason (part 2) | p95 GPU time and p95 frame time in BS change by at most 5 % from one pinned release to the next, unless the upgrade's report explains the cause | ML at Low and RD at High: Chrome on WebGPU | H on BS before and after the step, mode U; the change read between the medians of the 3 runs' p95s (proposed) | H, V on both sides. Bites: a planted 6 % slowdown (a busy-wait) | R3-WGPU-REC2's mark (lines 925–926) |

**When a mark fails.** A spike passes when every mark that applies passes: a conditional mark (PM-0003-40) only when
its condition holds; a "reported" mark (PM-0003-9, -14, -16, -31, -42) when its record is complete. A failing run is
never re-run until green: its data, build commit, machine, browser and backend are kept, and its cause is found and
written down (R5-TEST-REC6, r5-toolchain-quality.md lines 1490–1498). A failed mark goes back to the draft:
graphics-engineer revises the decision part the row names, by the draft's own Rollback and "Revisit when" routes (for
example a technique moved to High only in the tier data, the other ink family, the AO pre-pass on High only, a
tighter budget LOD), and the revised part returns to the advisor for review and to Mustafa and the Advisor with the
measured numbers. A mark's number changes only in the draft, with its reason, never inside a spike (budgets are data,
and raising one is a reviewed change: R5-GATE-REC4, r5-toolchain-quality.md lines 1734–1744). A run set that fails V is neither a pass nor a
fail. An item Mustafa marks "not yet" goes back to the draft in the same way, and the look proof test shows it again.

**What has no measurable mark here** (one line each)
- Part 2's pin policy has no spike mark; PM-0003-43 runs at each later upgrade.
- Part 5.e (the colour output), 5.g (lighting and moods), 5.h (the value hierarchy), 5.l (halftone and line boil) and
  5.m (the concussion) have no number beyond their cost in PM-0003-9: Mustafa judges them (the look checklist).
- Part 8's client field data has no spike mark: it is switched on in slice 6 (the draft's migration step 10).
- Part 9's listener placement waits for the camera framing (OQ-0003-13), then Mustafa's listening check.
- Part 10's order of steps, the cursor ray on the drawn state and split feedback have no spike mark: they are proven at
  slice milestones; part 10's measurable parts are PM-0003-20, -29 and -36.
- Part 11 (the test assets) has no mark of its own: they are counted by PM-0003-15 and PM-0003-22 and judged in
  PM-0003-17.
- No mark here runs under a network profile: neither spike has a network in it. The remote-entity pop-in check of
  slice 3 is ADR 0001's mark (R2-INT-REC1 (a), r2-netcode.md lines 2437–2439) under ADR 0001's profiles, and the
  netcode proof test's checklist is ADR 0001's.

#### The look checklist (Mustafa, in Brave on his Windows PC)
Beside it, the measurable marks: each technique's cost (PM-0003-9), the two ink families (PM-0003-12), the frame time
with the kept look on both reference machines (PM-0003-13) and the pixel-ratio costs (PM-0003-14). His PC is far above
the recommended class, so what he sees is the look, never the budget. Each step: what to look at, beside a legacy
screenshot at the same framing (taken by the session with legacy's pageshot, headed), and the expected result in
LOOK_V2's words. He records "achieved" or "not yet" (with a note), or his pick where a step offers one.

1. Open the look proof test's link in Brave and wait for the loading screen to end. Expected: the scene appears and the
   performance overlay names the backend ("WebGPU" or "WebGL 2 fallback") and the graphics adapter. Record: which
   backend Brave runs (R3-WGPU-OQ1).
2. The whole frame at the game camera, beside legacy (LOOK_V2 §1). Expected: "XIII meets Red Alert 3, lit like
   Planetary Annihilation": cel-shaded comic rendering with bold ink; a clean, well-lit scene with emissive accents.
3. The whole frame again, for the audience (§3). Expected: it does not look like a children's game.
4. The two ink families side by side, beside legacy's ink (§2, §6.2; OQ-0003-2): A, screen-space edges (proposed); B,
   the hull. Expected: thick black outlines, especially strong on buildings; thick ink on silhouettes, thinner lines
   inside. Pick: A or B, and whether the buildings' ink is heavy enough.
5. The same frame with the colours as authored (legacy's rule) and with the tone-mapped output, beside legacy (§2;
   OQ-0003-3). Expected: vibrant, beautiful colours. Pick: as authored, or tone-mapped.
6. Shadows beside legacy's (§2). Expected: soft shadows.
7. The frame at the game camera with ink and ambient occlusion on, then off (§2). Expected: geometry that reads
   clearly when they are on.
8. A soldier, a vehicle and a very large unit at the game camera (§1, §5). Expected: iconic silhouettes you could draw
   from memory, each readable by its outline alone.
9. One unit at the closest and at the farthest camera framing (§5). Expected: simple and readable from a distance, real
   depth up close; never "a block on wheels with a tube".
10. The new soldier beside legacy's soldier (§3). Expected: slimmer, serious proportions (XIII, Valiant Hearts); no
    chunky body, no round or goofy head.
11. A conscript in each faction's colours, beside legacy's conscript (§2). Expected: the round shoulder pieces, the
    centre stripe and the long trench-coat jacket; the factions told apart by colour.
12. Weapons and headgear at the game camera (§3, §6.3). Expected: real weapon silhouettes, no plain tubes or
    cylinders; weapons and headgear read first from above.
13. A soldier strafing on the full rig, then on the cheaper rig of the budget LOD (§2). Expected: the body reacts to
    strafing on the full rig; the cheaper rig's difference does not distract at the distance it is used.
14. Both houses beside legacy's (§2, §3). Expected: the half-timbered house with the brick-tile roof and the plastered
    house with the silo tower; no thatched roof anywhere.
15. The lush patch: fields, logs and trees (§1, §2, §4). Expected: lush; logs and trees with variety. The trees are
    labelled placeholders: the tree style is `[OPEN]` and not judged here.
16. Ground and environment against units, faction colours and visual effects (§6.1). Expected: the environment sits one
    step calmer; units, faction colours and effects are the brightest things on screen.
17. The glow (§1, §6.4). Expected: objectives, faction tech and muzzle flashes glow; nothing else does.
18. Sunlit and shadowed sides of the same buildings (§6.5; OQ-0003-5). Expected: warm sun with cool shadows. Pick: if
    two lighting options are shown, which one.
19. The same scene at day and at dusk (§6.5). Expected: a different mood per map.
20. The two-floor building: walk in, go upstairs, look down (§3, §6.5). Expected: interiors and the upper floor read
    from above; soft ambient occlusion in the corners (with it on and off: this decides whether AO without the
    pre-pass is good enough, R3-LOOK-OQ3).
21. Walk into a building and out again (§4; OQ-0003-6). Expected: the roof fades while the player is inside and
    returns outside; no ink drawn on the faded parts. Record: his verdict on how the fade looks.
22. The test building through its three stages (§4). Expected: intact, then damaged, then a ruin with the interior
    exposed; no hitch when a stage changes (measured: PM-0003-8).
23. The ruin entered by a plank to its upper floor (§4). Expected: a ruined building you can enter, a collapsed roof,
    a plank that reads as a route up; a corner worth holding.
24. A grenade blast near the player, beside legacy's (§3, §7). Expected: fragments flying, not just smoke and fire.
    Whether fragments deal damage is not judged here (a design question).
25. Grenade blasts at three distances (§2, §7). Expected: a screen shake that tilts away from the blast and grows
    stronger closer in, as legacy's feels; the aim does not move.
26. The Imperial Warrior's SMG firing at a target, beside legacy's (§7; LOOP_V2 §7). Expected: punchy: kick, muzzle
    flash, tracer, impact, hit marker, sound, target flinch and kill confirm; the shot feels instant and the hit
    feedback comes when the hit is confirmed.
27. One big moment with the halftone flash and ink line boil (§7; OQ-0003-8). Expected: comic impact touches, used
    sparingly. Pick: keep or drop each (its cost is in PM-0003-9).
28. The same frame at pixel ratio 1 and 2, with PM-0003-14's GPU times beside it (OQ-0003-12). Pick: ratio 1 or 2 on
    Low.
29. The legacy touches LOOK_V2 does not name, each shown beside legacy (OQ-0003-15): comic words, the kill vignette,
    the toy-soldier death fall, the skin rim. Pick: keep, change or drop each.
30. Nothing to judge for LOOK_V2 §8: the test assets are plain glTF/GLB exports given the comic look by the engine
    (steps 2 to 29 show it); compression, LODs and slot renaming are not tested here (ASSET-RESEARCH (a)); the
    per-unit budgets are counted in PM-0003-15 and PM-0003-22.

#### The bench checklist (Mustafa, in Brave on his Windows PC)
Beside it: PM-0003-42 (frame pacing and power per option), PM-0003-38 and PM-0003-39 (voices).

1. Play the benchmark scene for one minute with each frame-rate option: a 60 fps cap, following the display, a cap per
   tier (OQ-0003-10). Expected: he can say which feels smoothest on his display. Record: his pick, and his display's
   refresh rate.
2. Listen through the benchmark fight. Expected: his own shots and shots at him are always heard; no crackle, gap or
   dropout. Record: achieved or not yet.

---8<--- the subsection ends on the line above ---

### Spike entries (for docs/SPIKE_PLAN.md)
Names from the decision map's part g. Sizes: S about a day, M 2 to 4 days, L a week or more of session work.

**The look proof test** (LOOK_V2 §9)
- **What it proves:** how each LOOK_V2 `[DECIDED]` item looks, judged by Mustafa in Brave beside legacy screenshots at
  the same framing; each look technique's GPU time, draw calls and memory alone, on both reference machines and both
  backends; both ink families at 12, 24 and 48 soldiers on the minimum laptop at Low; one build on both backends in
  the four browsers with the same look; no first-sight shader stutter; the backend reported (and Brave's on his PC);
  the kept look's frame time on both machines.
- **Its marks:** PM-0003-1 to PM-0003-17 (the look checklist inside PM-0003-17).
- **Dependencies:** the drafts approved; both reference machines named and on hand (OQ-0003-1), plus a macOS machine
  with Safari 26 (the draft names none); a few authored test assets as plain glTF/GLB exports (part 11: one soldier
  with faction colour slots, one weapon, one two-floor building with its damage stages, one very large unit of at
  least 20 parts, a few props), with no compression or LOD step (ASSET-RESEARCH (a)); legacy screenshots at the same
  framing; Mustafa's session in Brave; the measurement harness of part 8, built here first; a synthetic recording for
  the look-test scene (no recorded match exists yet). It shares nothing with the netcode proof test, so it runs in
  parallel with it.
- **Size:** L. Two machines and a Mac, five browser setups on two backends, two ink families (the screen-space one is
  our own TSL node), the authored test assets, and Mustafa's session.
- **Research spikes it subsumes:** R3-LOOK-REC1, REC2, REC3; R3-WGPU-REC1, REC3, REC4; R3-GL2-REC1, REC3;
  R3-BUDGET-REC1 (the machines named). **Decides:** R3-WGPU-OQ1, OQ2, OQ3; R3-GL2-OQ3; R3-LOOK-OQ1, OQ3, OQ4;
  R3-MEASURE-OQ1, OQ3; R3-GL2-OQ1 and R3-LOOK-OQ6 after their source readings (no mark); it gives Mustafa what he needs
  for OQ-0003-2, -3, -5, -6, -8, -12 (R3-BUDGET-OQ3) and -15.

**The render budget bench** (may merge into the look proof test)
- **What it proves:** every row of part 7's budget table in the benchmark scene with the techniques the look proof test
  kept, on both machines: characters at N = 48 on screen, the four very large units, the Firefox WebGL 2 worst case,
  the quality tiers, the harness and the CI split, audio in the benchmark fight; later, each three.js upgrade.
- **Its marks:** PM-0003-18 to PM-0003-42 (PM-0003-40 only if occlusion rays are ever proposed), PM-0003-1 checked
  again; PM-0003-43 at each later upgrade. R1-BND-REC1's mark 1 (the second statistic of PM-0003-20) is the boundary
  spike's.
- **Dependencies:** the look proof test's kept techniques and Mustafa's picks (the ink family sets the Low draw cap);
  ADR 0002's boundary spike (the Worker and its block; PM-0003-20 runs on them); a recorded match for the benchmark
  scene from ADR 0004's sim core spike, or a synthetic recording; the stress scene's counts (SPIKE_PLAN; GLOSSARY §3);
  the gate v2's G12 job for PM-0003-36 (the bench comes after the foundation in part g's order).
- **Size:** L. The skinned crowd at 48 with its budget LOD, two instanced candidates (C-E is our own TSL code), the
  voice manager, the CI job and its planted defects, every row on two machines, two browsers and two backends.
- **Research spikes it subsumes:** R3-CHAR-REC1, REC2, REC3, REC4; R3-BUDGET-REC2, REC3; R3-GL2-REC2;
  R3-MEASURE-REC1, REC2, REC3, REC4 (the draft places REC1 and REC4 here; the map's part g places them in the look
  proof test); R3-AUDIO-REC1, REC2, REC3, REC4; R3-WGPU-REC2 (later). **Decides:** R3-CHAR-OQ1, OQ2, OQ4 (OQ6 only if
  the draw-call cap is still missed); R3-BUDGET-OQ5, and R3-BUDGET-OQ6 with OQ-0003-4; R3-AUDIO-OQ1, OQ2 (OQ6 only if
  the built-in panners miss PM-0003-38); it gives Mustafa the numbers for OQ-0003-10 (R3-BUDGET-OQ2).
