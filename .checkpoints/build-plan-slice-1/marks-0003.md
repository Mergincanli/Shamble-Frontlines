# ADR 0003 pass marks: digest for the build plan (foundation 2.4, 2.5 and Slice 1)

digest-0003, graphics-engineer, 2026-11-14. A reading aid, not a decision: numbers are copied from
docs/adr/0003-rendering.md (cited as 0003:NNNN); "[unsure]" marks my reading where the draft does not say. Machine keys
as the draft gives them (0003:1082-1100): ML the minimum laptop (Iris Xe class, 1920 x 1080, Windows 11), ML-Linux the
same laptop on Linux, RD the recommended desktop (1920 x 1080 and 2560 x 1440), MAC the Mac for Safari, MP Mustafa's PC
in Brave (judges only, never a measured result), CI the headless runner (counts only, never a time). Until Mustafa names
ML, his laptop on "Power saving" is a stand-in "for rough checks only"; a rough check is reported, never judged against a
pass mark, and nothing stands in for RD or MAC (SPIKE_PLAN 65-69; 0003:1096-1099). Scenes: BS the benchmark scene (N = 48
soldiers in three teams of 16, 4 very large units of at least 20 parts each, 300 live projectiles with their visual
effects; 0003:1101-1108), LS the look-test scene (0003:1109-1112). H = 3 runs of 600 measured frames after 300 warm-up
frames (tail form: 3,000 frames, for PM-0003-11, -19, -20, -21, -26, -27, -30, -33, -40; 0003:1118-1127); V = run medians
within 3 % (0003:1141-1145); S = the soaked set on ML after at least 15 minutes, which gates PM-0003-18 to -21
(0003:1128-1134); U uncapped, P presented (0003:1146-1154); "rule B" = a planted busy-wait of margin + 1 ms must fail
the mark (0003:1168-1170). Slices: 2.5 the client + local server skeleton (empty scene from one block per tick, backend
named); Slice 1 move and shoot (one code-built stand-in soldier, small greybox test scene, a weapon, projectiles, hit and
death events, camera; marks at N = 12); Slice 2 bots, three teams, first map; Slice 3 online; Slice 4 the loop
(abilities, VFX-heavy); Slice 5 Octopon-3, the editor; Slice 6 scale to 48, the stress scene with the four very large
units (task brief, from ADR 0002's migration plan).

## 1. The pass marks, one row each

| id | test | what is measured or judged (draft's numbers) | where it runs, who judges | what must exist to measure it | first real-game build that can show it | code-built stand-in + greybox enough? | does code rest on it? | conditional / reported-only? | source |
|---|---|---|---|---|---|---|---|---|---|
| PM-0003-1 | look proof test (precondition); checked again at the bench | ML and RD named with GPU, CPU, RAM, screen, `devicePixelRatio` and `hardwareConcurrency`; MAC's model recorded; every mark naming ML and RD has results from both | ML, RD, MAC; a results-log header per machine, written before the first run | Mustafa names the machines (OQ-0003-1, open in part: "named by Mustafa before the look proof test") | 2.5 (before the first timed run; no build needed for the header) | n/a (no models) | low: gates every timed mark, not code itself | no | 0003:1175 |
| PM-0003-2 | look proof test | 0 console errors and 0 device errors (`uncapturederror`, device `lost`, `webglcontextlost`) in every start | ML: Chrome, Edge, Firefox (Windows), each on WebGPU and `forceWebGL`; MAC: Safari, both backends; scripts | the page's own error hook; a scripted play-through; 10 starts per browser and backend (PM-0003-5) | 2.5 (empty scene); again at Slice 1 with the kept look on | yes | medium: errors are mostly local fixes; a backend-only failure is not | no | 0003:1176 |
| PM-0003-3 | look proof test | at each of 3 fixed frames, at most 1 % of pixels differ by more than 8/255 in any channel between the two backends, every kept effect on | ML: Chrome and Firefox, WebGPU against `forceWebGL`; MAC: Safari, the same pair; pixel-diff script | the kept techniques (toon, ink, sun shadow, glow, AO); 3 repeatable fixed frames (control: same backend twice gives 0 differing pixels) | Slice 1 (stand-in + greybox with the kept look); re-run as each technique enters | yes for the techniques; house and very-large-unit frames later | high: F-A; a WebGL 2 mismatch redoes that TSL node | no | 0003:1177 |
| PM-0003-4 | look proof test | median CPU frame time on the WebGPU backend at most 1.1 x the WebGL 2 backend's, same machine, browser and tier; all 3 run pairs | ML at Low: Chrome, Firefox; RD at High: Chrome; scripts, mode U | harness CPU side (H, V); LS "at the fight on screen", i.e. BS's counts | Slice 1 reading at N = 12 (reported only); as written Slice 6 (Slice 2: 48 soldiers without the very large units) | a reading yes; the mark's fight needs very-large-unit content | medium: one TSL source; a fail moves the default backend | no | 0003:1178 |
| PM-0003-5 | look proof test | backend named correctly in 10 of 10 starts per browser and backend; on ML-Linux, Firefox's overlay names "WebGL 2 fallback"; Brave's backend on MP recorded | ML: Chrome, Edge, Firefox; ML-Linux: Firefox, Chrome; MAC: Safari; MP: Brave; scripts compare strings | the overlay's and log's backend string; every test screenshot prints it | 2.5 (its own milestone: the empty scene names its backend) | n/a | low: a label | no | 0003:1179 |
| PM-0003-6 | look proof test | every frame record: timed, disjoint, or "GPU time not available"; Chrome's WebGL 2: at least 99 % of 600 timed, each above 0, tied to its frame; Firefox's, Safari's WebGL 2: "not available" in 600 of 600 | ML: Chrome and Firefox with `forceWebGL`; ML-Linux: Firefox; MAC: Safari with `forceWebGL`; scripts | harness GPU side: GPU timestamp queries (`EXT_disjoint_timer_query_webgl2`), per-record status and frame-number tie | 2.5 if the harness's GPU side lands there, else Slice 1 | yes (no content needed) | medium: every GPU-time mark trusts these records | no | 0003:1180 |
| PM-0003-7 | look proof test | `renderer.info.memory.programs` and the pipeline count (`createRenderPipeline`, `createRenderPipelineAsync`, `linkProgram`) equal 5 s after deploy and at the end of a scripted 5-minute match; 3 runs | CI (a count); ML: Chrome on WebGPU and `forceWebGL`, Firefox on WebGPU | the warm-up (`await renderer.init()`, `compileAsync()`, part 3); a 5-minute script showing every unit type, visual effect, damage stage and roof fade; a pipeline-call spy | Slice 1 (its milestone: "programs constant over a match"); grows each slice; complete at Slice 5 (damage stages, roof fade) | yes | high: warm-up and material-per-slot-content design (parts 3, 5.a) | no | 0003:1181 |
| PM-0003-8 | look proof test | no frame above 1.5 x the run's median frame time in the 1 s after the first appearance of each unit type, visual effect, damage stage and roof fade; 0 in each of 3 runs, fresh profiles | ML at Low: Chrome on WebGPU and `forceWebGL`, Firefox on WebGPU; scripts, mode U | harness frame times; a first-appearance log; the warm-up; ML named | Slice 1 (soldier, weapon effects, tracers, hits); complete at Slice 5 | yes | high: the loading-screen warm-up design | no | 0003:1182 |
| PM-0003-9 | look proof test | per technique and variant: GPU ms (median, p95), extra draw calls, extra render-target memory (`texturesSize`), each attachment single-sampled and at 4x MSAA; halftone overlay as frame time; each cell a number or "not available" | ML at Low and RD at High, Chrome on WebGPU and `forceWebGL`, 1920 x 1080; scripts | each technique switchable alone: 5.a toon, both 5.b ink families, 5.c AO, 5.d glow, 5.f shadow map, 5.i roof fade, 5.k effects, 5.l halftone and line boil, 5.n MSAA; per-pass GPU timestamp queries; H, V | Slice 1 for all but 5.i (roof fade: Slice 5); full-screen passes (ink, AO, bloom) cost by resolution, so early numbers are honest [inferred]; main-pass costs grow with content | yes, except 5.i (needs buildings) | high: feeds the ink pick, the look floor (OQ-0003-16), Low's tier data | reported-only | 0003:1183 |
| PM-0003-10 | look proof test | with every kept effect on, `drawCalls` at most 1.1 x (draws with every effect off + planned extras: one per outlined object for a hull, one per drawn object for an AO pre-pass), every frame | CI (a count); ML, both backends | the shared MRT scene pass (part 5); per-effect on/off switches | Slice 1 | yes | high: validates the shared-pass design every effect builds on | no | 0003:1184 |
| PM-0003-11 | look proof test | each effect's off setting and its lowered setting each cut that effect's own GPU pass time by at least 40 % (p99; the median reported) | ML at Low, Chrome on WebGPU and `forceWebGL`; scripts | per-effect tier switches (there "from the start", part 5); per-pass timestamps (developer flag); H tail form | Slice 1 | yes | medium: tier switches are built in from the start | no | 0003:1185 |
| PM-0003-12 | look proof test | reported: GPU and frame time (median, p95) at 12, 24 and 48 soldiers per ink family; gated: at 48 soldiers `drawCalls` at most 800 (screen-space ink), at most 1,100 (hull), whole frame incl. post passes | ML at Low: Chrome on WebGPU; ML-Linux: Firefox; scripts, mode U | both ink families built; 12, 24, 48 soldiers on screen; for the gated count the whole frame (world, very large units, health bars, effects, projectiles: 799 to 817 with the screen-space ink, 0003:366) | times: Slice 1 at 12 (24 and 48 as copies of the stand-in [unsure]); gated whole-frame count: Slice 6 | times yes if the stand-in is built to the soldier budget (at most 4 draws per pass); the gate needs the map and very large units | high: the ink family drives the post chain and the Low cap | count gated; times reported | 0003:1186 |
| PM-0003-13 | look proof test | ML at Low: frame time median at most 28 ms and p95 at most 33.3 ms; RD at High: median at most 14 ms and p95 at most 16.7 ms; kept techniques, fight on screen | ML: Chrome on WebGPU and `forceWebGL`, Firefox on WebGPU; RD: Chrome on WebGPU at 1920 x 1080 and 2560 x 1440; scripts, mode U | ML and RD named; harness; the techniques Mustafa keeps (PM-0003-17); the fight on screen at BS's counts | Slice 1 reading (N = 12, small scene; reported); as written Slice 6 | a reading yes; the mark needs the full fight | high: if the kept look misses Low, techniques and tiers are redone | no (rule B) | 0003:1187 |
| PM-0003-14 | look proof test | GPU time (median, p95) at pixel ratio 1 and at ratio 2 | ML at Low, Chrome on WebGPU; feeds Mustafa's OQ-0003-12 | harness GPU side; the renderer's pixel-ratio setting | Slice 1 | yes | low: one tier value | reported-only | 0003:1188 |
| PM-0003-15 | look proof test | over the lush patch (fields, logs, trees with variety) at the game camera: `drawCalls` at most 800 and triangles at most 500k, every frame | CI (counts); ML at Low | a lush patch: props, logs, legacy's tree builders ported as placeholders (GW18); instancing or merging; scripted camera | Slice 5 ("the lush countryside", 0003:1408) [unsure: Slice 2's first map may already hold a patch] | yes (code-built props and ported builders; no authored model) | medium: the prop instancing or merging path | no | 0003:1189 |
| PM-0003-16 | look proof test | recorded: `renderer.hasFeature` for timestamp queries on Firefox, Safari; `info.memory` and `memoryHeaps` on ML; `info.memory.total` with MSAA against 5.n's 199.2 MB at 1920 x 1080; the planted device error's display; Brave's backend, MP's dpr and resolution | ML, MAC, MP; read once per browser into the results log | the MRT pass with MSAA switchable; PM-0003-2's planted device error | 2.5 for most records; the MSAA reading at Slice 1 (the MRT pass) | yes | low: records; the MSAA memory may change Low's samples | reported-only | 0003:1190 |
| PM-0003-17 | look proof test | every LOOK_V2 item in the look checklist judged "achieved", and a pick recorded for each call he makes; 0 items left "not yet" | MP, Brave on Windows; Mustafa judges, legacy in a second Brave window at the same framing | the look checklist's content: soldier, weapon, vehicle, very large unit, both houses, the two-floor building and its stages, the lush patch, a grenade, a big moment (section 2) | in pieces, section 2: Slice 1 (steps 1-7, 9, 13's strafing, 16-18 in part, 26, 28, 31 in part); Slice 2 (11, 13's crowd); Slice 4 (17's objectives); Slice 5 (14-15, 19-23); Slice 6 (8's very large unit, 32) | partly: steps 10-12 need authored models by 0003:479-481 unless Mustafa accepts the stand-in [unsure] | high: the ink family, colour output and lighting model rest on it | no | 0003:1191 |
| PM-0003-18 | render budget bench | on BS: ML at Low median at most 28 ms, p95 at most 33.3 ms; RD at High median at most 14 ms, p95 at most 16.7 ms | ML: Chrome and Firefox, each on WebGPU and `forceWebGL`; RD: Chrome on WebGPU at 1920 x 1080 and 2560 x 1440; scripts, mode U | BS; harness; ML's 15-minute soak; the Worker's load recorded; ML and RD named | Slice 6 (Slice 2 gives an N = 48 reading without very large units, reported) | object counts yes; soldier and very-large-unit costs need budget-sized models [unsure] | high: the whole renderer's frame budget | S on ML gates; cold set reported | 0003:1193 |
| PM-0003-19 | render budget bench | CPU per frame p99 at most 16 ms (ML, Low) and 8 ms (RD, High); of which character animation (posing, bone upload, budget LOD on) p99 at most 1.0 ms (ML) and 2.0 ms (RD) | as PM-0003-18; scripts | BS; a build with the isolation flag; H tail form; the budget LOD (Slice 2) | animation share Slice 2 (N = 48, budget LOD); whole row Slice 6; Slice 1 reading at N = 12 | yes if the stand-in's rig has the real bone count [unsure] | high: per-frame flow and animation design | S on ML gates; cold set reported | 0003:1194 |
| PM-0003-20 | render budget bench | the page's sim work p99 at most 1.0 ms per frame on both machines; R1-BND-REC1 mark 1 (p95 frame-time difference at most 1.0 ms at N = 12, 24, 36, 48 on ML) also gates | ML at Low and RD at High: Chrome on WebGPU; ML: Firefox on WebGPU; scripts | the real Worker and block at 30 Hz; isolation flag; H tail form; S on ML | 2.5 (one block per tick: trivial load); Slice 1 at N = 12; Slice 2 up to N = 48 | yes | high: block intake and the Worker boundary (ADR 0002) | S on ML gates; cold set reported | 0003:1195 |
| PM-0003-21 | render budget bench | p99, ML Low / RD High: total at most 25 / 12 ms; shadow pass 3 / 2 ms; main scene pass 16 / 6 ms; post-processing (ink, AO, glow, comic touches) 4 / 3 ms; effects, sprites, HUD in 3D 2 / 1 ms; each measured directly | ML: Chrome on WebGPU and `forceWebGL`; RD: Chrome on WebGPU, both resolutions; Firefox only where PM-0003-16 found timestamp queries | per-pass GPU timestamp queries, developer flag on; BS; S on ML | the post row at Slice 1 (full-screen cost scales with resolution [inferred]); every row on BS at Slice 6 | post row yes; scene and shadow rows need full content | high: the post row decides ink, AO and glow at Low | S on ML gates; cold reported; with OQ-0003-4 decides R3-BUDGET-OQ6 | 0003:1196 |
| PM-0003-22 | render budget bench | per frame, the run's max: draw calls at most 800 at Low (at most 1,100 if Mustafa keeps the hull at Low), at most 1,500 at High; triangles at most 500k / 1.5M; GPU memory at most 512 MB / 1 GB | CI (counts); ML and RD in every browser and backend of PM-0003-18 | BS; `renderer.info` read every frame; with the first authored models it decides R3-BUDGET-OQ5 | CI baseline from 2.5; render-target memory honest at Slice 1; full rows Slice 6 | draw counts yes with budget-sized stand-ins; triangles and memory need authored models (R3-BUDGET-OQ5) | high: the soldier draw scheme and the Low cap | no | 0003:1197 |
| PM-0003-23 | render budget bench | Low: pixel ratio at most 1, scene resolution scale 0.75 to 1, one 1024² shadow map, every technique carrying a LOOK_V2 `[DECIDED]` item on; High: pixel ratio at most 2 | ML, RD; read at the start and end of every BS run | the tier table as data (part 7) | Slice 1 | yes | medium: the tiers-as-data shape | no | 0003:1198 |
| PM-0003-24 | render budget bench | JS heap at most 150 MB after 10 minutes; the heap after a forced garbage collection at minute 10 at most 0.5 MB above the one at minute 5 | ML and RD, Chrome on WebGPU; CI | a 10-minute loop; garbage collection and heap read through the Chrome DevTools Protocol | 2.5 (the method), Slice 1 (a match loop with pools); BS at Slice 6 | yes | medium: pools and allocation discipline | no | 0003:1199 |
| PM-0003-25 | render budget bench | at most 4 draw calls per soldier per pass (before the ink) with 48 soldiers on screen, every frame | CI (a count); ML: Chrome on WebGPU and `forceWebGL` | one skinned mesh per soldier (C-B), at ADR 0005's cap of 4 materials per pass; 48 soldiers | Slice 1 for the per-soldier count (48 copies [unsure]); Slice 2 at N = 48 | yes if the stand-in is one skinned mesh at the material cap | high: the C-B draw scheme (decides R3-CHAR-OQ1) | no | 0003:1200 |
| PM-0003-26 | render budget bench | posing and bone upload with every soldier on the full rig: p99 at most 2.0 ms per frame | ML: Chrome on WebGPU and `forceWebGL` | BS with the budget LOD off; 48 skinned soldiers; isolation flag; H tail form | Slice 2 (N = 48); Slice 1 reading at N = 12 | only if the stand-in's rig matches the real bone count [unsure] | high: whether the budget LOD is needed | no | 0003:1201 |
| PM-0003-27 | render budget bench | C-D or C-E adopted only if its total frame time is at least 20 % lower than C-B's on both machines (p99; median and p95 reported); all 3 run pairs | ML at Low and RD at High: Chrome on WebGPU (C-D, C-E) and `forceWebGL` (C-E) | C-D and C-E built beside C-B; BS | Slice 2 (N = 48) | counts yes; costs need a budget-sized soldier [unsure] | medium: C-B stands unless a clear win | adoption rule (C-B stands otherwise) | 0003:1202 |
| PM-0003-28 | render budget bench | identical pixels on 600 of 600 frames with frustum culling off and on | CI; ML: Chrome on WebGPU | fixed padded-sphere bounds; BS rendered twice (a repeatable replay; control: culling on twice) | Slice 1 if a local match replays identically [unsure: client replay is ADR 0004's] | yes | low: one bounds value | no | 0003:1203 |
| PM-0003-29 | render budget bench | 0 bytes allocated per frame in the character update path over 60 s; health bars for 48 soldiers in at most 2 draw calls | ML: Chrome on WebGPU; CI | heap sampling by legacy's TL15 method; preallocated per-instance arrays; instanced health bars | allocation: Slice 1 (its milestone); 48 bars: Slice 2 ("instanced health bars", 0003:1400-1401) | yes | high: the per-instance data path from the blocks | no | 0003:1204 |
| PM-0003-30 | render budget bench | the 4 very large units add at most 2.0 ms GPU time (p99) and at most 200 draw calls against BS without them, before any ink or AO pre-pass; render bundles stay only if they cut CPU frame time by more than V's 3 % | RD at High, Chrome on WebGPU | 4 very large units of at least 20 parts (V-A), their effects; BS | Slice 6 (the stress scene) | draws: a code-built blockout of at least 20 parts could; GPU time needs budget-sized parts [unsure] | medium: V-A, parts as meshes | no | 0003:1205 |
| PM-0003-31 | render budget bench | CPU and GPU time (median, p95) for the same prop field, `BatchedMesh` against `InstancedMesh` | ML: Chrome on WebGPU and `forceWebGL`, Firefox with `forceWebGL` (no multi-draw) | a prop field (LS's lush patch); both variants | Slice 5 (lush) [unsure: Slice 2's map] | yes (code-built props) | medium: the prop path (decides R3-CHAR-OQ4) | reported-only | 0003:1206 |
| PM-0003-32 | render budget bench | `drawCalls` within the Low cap (800, or 1,100 with a hull at Low) on every frame, and median frame time at most 33.3 ms | ML-Linux: Firefox (the real WebGL 2 fallback, no `WEBGL_multi_draw`) | BS; mode U if Firefox has a switch, else P | Slice 6; Slice 1 reading | a reading yes | high: the fallback's worst case sets the Low cap | no (rule B) | 0003:1207 |
| PM-0003-33 | render budget bench | switching High to Low cuts GPU frame time by at least 50 % (p99), with `programs` and the pipeline count unchanged across the switch | ML: Chrome on WebGPU and `forceWebGL` | every tier's variants in the warm-up; a live tier switch; BS; H tail form | Slice 1 (the no-compile half; the 50 % cut as a reading); BS at Slice 6 | yes | medium: tier variants in the warm-up | no | 0003:1208 |
| PM-0003-34 | render budget bench | when GPU-bound: the GPU total within 2 ms of the frame time minus the main-thread CPU work (median); two back-to-back runs of one build: medians within 3 % | RD: Chrome on WebGPU, developer flag on | harness CPU and GPU sides (GPU timestamp queries); a scene made GPU-bound by raising its resolution scale | Slice 1 (the post chain can make the frame GPU-bound) | yes | medium: every timed mark trusts the harness | no | 0003:1209 |
| PM-0003-35 | render budget bench | identical `drawCalls` and `triangles` on 600 of 600 frames; identical screenshots at 3 fixed frames | ML and RD: Chrome on WebGPU; CI | a replayed recording (ADR 0004's replay, or a fixed-seed bot match); presentation randomness seeded | Slice 1 if a local match replays [unsure]; Slice 2 (fixed-seed bot match) | yes | high: every count mark and the CI baseline rest on it | no | 0003:1210 |
| PM-0003-36 | render budget bench | CI fails on a planted +10 % in draw calls and +10 % in GPU memory (1 of 1 each); a +5 % control passes; runs under 5 minutes; also fails on errors, triangles, programs, pipelines beyond baseline, per-frame render-path allocation | CI (headless; counts only) | the gate v2's CI job; a committed baseline file; `gate --prove` mutations; heap sampling | 2.5 (0003:1391-1392: "CI gates the render counts"); grows each slice | yes | medium: process; catches regressions early | no | 0003:1211 |
| PM-0003-37 | render budget bench | median frame time with the overlay closed within 1 % of a build without it; open, within 3 %; closed, 0 overlay calls and 0 bytes allocated by the overlay per frame | ML at Low and RD at High: Chrome on WebGPU; 10 interleaved pairs | the overlay (legacy GP5 ported); two builds | 2.5 (the counted part); Slice 1 (the timed part) | yes | low: the overlay only | no | 0003:1212 |
| PM-0003-38 | render budget bench | with 32 concurrent equal-power voices: no dropout in a 5-minute capture, the AudioContext "running"; main-thread audio under 0.5 ms (p99) per frame; HRTF on 4 voices raises that p99 by under 0.3 ms | ML: Chrome and Firefox; a listener (the person running the bench) and scripts | the voice manager; sources enough for 32 voices; isolation flag | Slice 2 (many shooters) [unsure: Slice 1 if a scripted 32-voice load counts] | yes | medium: voice design (decides R3-AUDIO-OQ1) | no | 0003:1213 |
| PM-0003-39 | render budget bench | live source nodes at most the cap (32 at Low, 48 at High) every frame; shots at the player dropped: 0 in 3 runs; new `AudioBuffer`s after warm-up: 0 | ML at Low and RD at High: Chrome | the voice manager with priorities as data; node and buffer counters; the drop log | Slice 1 (buffers and priorities with one weapon); the cap under load at Slice 2 | yes | medium: the voice manager (decides R3-AUDIO-OQ2) | gated; voices asked before the cap reported | 0003:1214 |
| PM-0003-40 | render budget bench, only if O-B is proposed | at most 1 ray per voice per 100 ms; all rays at most 0.2 ms (p99) per frame at the voice cap; 4 of 5 testers tell an occluded source from an open one in a blind test | ML at Low; 5 testers | O-B occlusion rays; walls | none while part 9 proposes zones or none; if ever, Slice 5 (buildings) | needs buildings | low | conditional | 0003:1215 |
| PM-0003-41 | render budget bench | sound onset after the drawn event at most one rendered frame plus the output latency, p99 over 100 shots; 0 autoplay warnings in the console on all four browsers | ML: Chrome, Edge, Firefox; MAC: Safari; mode P | a weapon, its muzzle flash, sound started from the tick's events; the AudioContext's output timestamp | Slice 1 | yes | medium: sounds from the block's events (part 10, step 6) | no | 0003:1216 |
| PM-0003-42 | render budget bench | per option (a 60 fps cap; follow the display; a cap per tier): smoothness (standard deviation of a steady unit's per-frame displacement over its mean) on 60, 120 and 144 Hz; ML battery drain over 10 minutes | RD on the displays available; ML on battery; beside the bench checklist's step 1 | a moving unit interpolated between ticks; the frame-cap options | Slice 1 (smoothness); battery drain honest on a heavier scene later | yes | low: a setting (feeds OQ-0003-10) | reported-only | 0003:1217 |
| PM-0003-43 | later, at each three.js upgrade | p95 GPU time and p95 frame time in BS change by at most 5 % from one pinned release to the next, unless the upgrade's report explains the cause | ML at Low and RD at High: Chrome on WebGPU | BS before and after the step; H, V | the first upgrade after the pin (on whatever benchmark scene exists then; BS from Slice 6) | yes | low | later | 0003:1223 |
| PM-0003-44 | render budget bench | 600 presented frames with the four very large units and their effects: frames over 1.5 refresh periods (25 ms at 60 Hz) at most 6 of 600 per run; ML at Low reported against 33.3 ms (over 50 ms) | RD at High, Chrome on WebGPU, a 60 Hz display; ML at Low, Chrome on WebGPU (reported) | the 4 very large units and their effects; mode P; animation-frame timestamps | Slice 6 | very-large-unit blockouts [unsure] | medium: very-large-unit drawing and effects | gated on RD; reported on ML | 0003:1218 |
| PM-0003-45 | render budget bench | after each of 3 match cycles torn down: `total`, `texturesSize`, `attributesSize`, `programs`, `renderTargets` equal to the first cycle's; heap within 0.5 MB; 0 shared materials disposed while a scene uses them | CI (counts); ML: Chrome on WebGPU and `forceWebGL` | a match teardown that keeps the renderer; a second scene with other factions; a spy on `dispose()` | Slice 1 (match end and restart with one faction [unsure: a second scene]); the faction swap at Slice 2 | yes | high: material ownership and teardown | no | 0003:1219 |
| PM-0003-46 | render budget bench | every CI run records each engine's backend; if one gets WebGPU, PM-0003-36's checks also run on it; else each render-path change-set carries a 60 s WebGPU smoke run on ML | CI; otherwise ML (Chrome on WebGPU) | the CI job; the smoke script; ML on hand for every render-path change | 2.5 (the draft starts it "from the render budget bench on", which no longer runs first) | yes | low: process; needs ML per render change if CI has no WebGPU | no | 0003:1220 |
| PM-0003-47 | render budget bench | reported: the HUD's DOM mutations per frame (median, max); gated: 0 HUD mutations in a scripted 5 s stretch with the match timer hidden and no change of score, ammo, health or kill feed | CI (a count) | the HUD; a `MutationObserver` on the HUD's root | Slice 1 (ammo, health, hit marker) | yes | medium: the HUD's write-on-change setters | gated and reported | 0003:1221 |

## 2. The look checklist and the bench checklist

What Mustafa judges in Brave on his Windows PC (MP). Every view at the test view (legacy's framing: height 40, boom
56.6, field of view 15°, yaw 45°, 0003:1113-1117; retaken at the v2 framing once R2-INT-OQ2 is answered), High unless
the step says Low, beside legacy opened in a second Brave window (0003:1255-1267). He records "achieved" or "not yet",
or his pick. The last bracket on each line is my reading of the first real-game build that can show the step
(not the draft's).

### The look checklist (inside PM-0003-17; 0003:1255-1359)
1. Open the link, loading screen ends; the overlay names the backend and adapter; record Brave's backend (R3-WGPU-OQ1)
   (0003:1269-1271). [2.5]
2. The whole frame at the game camera beside legacy (§1): cel-shaded comic rendering with bold ink, clean, well-lit,
   emissive accents (0003:1272-1273). [Slice 1 on the stand-in and greybox; the full frame by Slice 5/6]
3. The whole frame for the audience (§3): not a children's game (0003:1274). [Slice 1, partial]
4. The two ink families side by side beside legacy's ink (§2, §6.2; OQ-0003-2): A screen-space, B hull; thick outlines,
   strongest on buildings, thinner inside; look at the contact line with the ground. Pick: A or B, and whether the
   buildings' ink and contact line are heavy enough (0003:1275-1279). [Slice 1 for the soldier and greybox blocks;
   real buildings Slice 5]
5. Colours as authored against tone-mapped output (§2; OQ-0003-3): vibrant, beautiful colours. Pick (0003:1280-1281).
   [Slice 1]
6. Shadows beside legacy's (§2): soft shadows (0003:1282). [Slice 1]
7. Ink and AO on, then off (§2): geometry reads clearly when on (0003:1283-1284). [Slice 1]
8. A soldier, a vehicle and a very large unit at the game camera (§1, §5): iconic silhouettes, readable by outline
   alone (0003:1285-1286). [soldier Slice 1; vehicle Slice 4 (harvesters, 0003:1405) [unsure]; very large unit Slice 6]
9. One unit at the test view and at half its distance (§5): readable far, real depth close; never "a block on wheels
   with a tube" (0003:1287-1290). [Slice 1 on the stand-in; mesh LOD levels come with ADR 0005's pipeline]
10. The new soldier beside legacy's, at ADR 0005's cap of 4 materials per pass (§3): slimmer, serious proportions;
    no chunky body, no round or goofy head (0003:1291-1293). [Slice 1 on the stand-in, if Mustafa accepts it: part 11
    says authored models are needed for §3's proportions, 0003:479-481 [unsure]]
11. A conscript in each faction's colours beside legacy's (§2): round shoulder pieces, centre stripe, long trench-coat
    jacket; factions told apart by colour (0003:1294-1295). [Slice 2 (three teams); same stand-in caveat]
12. Weapons and headgear at the game camera (§3, §6.3): real weapon silhouettes, read first from above
    (0003:1296-1297). [Slice 1; same caveat]
13. Strafing on the full rig, then the fight at Low's animation budget (budget LOD by importance) (§2): the body
    reacts to strafing; the cheaper rig does not distract (0003:1298-1301). [strafing Slice 1; the crowd Slice 2]
14. Both houses beside legacy's (§2, §3): the half-timbered house with the brick-tile roof and the plastered house with
    the silo tower; no thatched roof (0003:1302-1303). [Slice 5 [unsure: Slice 2's first map if kit builders are
    ported]]
15. The lush patch (§1, §2, §4): lush; logs and trees with variety; trees are ported placeholders, tree style not judged
    (0003:1304-1306). [Slice 5 [unsure: Slice 2]]
16. Ground and environment against units, faction colours and effects (§6.1): environment one step calmer
    (0003:1307-1308). [Slice 1 partial; with real environment Slice 2/5]
17. The glow (§1, §6.4): objectives, faction tech and muzzle flashes glow; nothing else (0003:1309). [muzzle flashes
    Slice 1; objectives and faction tech Slice 4]
18. Sunlit and shadowed sides of the same buildings (§6.5; OQ-0003-5): warm sun, cool shadows. Pick if two lighting
    options (0003:1310-1311). [Slice 1 on greybox blocks; houses Slice 5]
19. The same scene at day and at dusk (§6.5): a different mood per map (0003:1312). [Slice 5 ("moods per map",
    0003:1408) [unsure: Slice 1 if its test scene carries two light sets]]
20. The two-floor building: walk in, upstairs, look down (§3, §6.5): interiors and upper floor read; soft AO in corners,
    on and off (decides R3-LOOK-OQ3) (0003:1313-1315). [Slice 5]
21. Walk into a building and out (§4; OQ-0003-6): the roof fades inside, returns outside; no ink on faded parts;
    record his verdict (0003:1316-1317). [Slice 5]
22. The test building through its three stages (§4): intact, damaged, ruin with interior exposed; no hitch
    (PM-0003-8) (0003:1318-1319). [Slice 5]
23. The ruin entered by a plank to its upper floor (§4): enterable ruin, collapsed roof, a plank that reads as a route,
    a corner worth holding (0003:1320-1321). [Slice 5]
24. A grenade blast near the player beside legacy's (§3, §7): fragments flying, not just smoke and fire; damage per
    fragment not judged (0003:1322-1324). [the first build with a grenade [unsure which]]
25. Grenade blasts at three distances (§2, §7): the shake tilts away from the blast and grows closer in, as legacy's;
    the aim does not move (0003:1325-1326). [as step 24]
26. The Imperial Warrior's SMG firing at a target beside legacy's (§7; LOOP_V2 §7): kick, muzzle flash, tracer, impact,
    hit marker, sound, target flinch, kill confirm; the timing is judged at slice 1's Brave check, not here
    (0003:1327-1331). [Slice 1 [unsure: whether Slice 1's weapon is that SMG]]
27. One big moment with the halftone flash and line boil, the flash over the HUD (§7; OQ-0003-8): used sparingly;
    record whether the dotted HUD reads; pick keep or drop each; OQ-0003-18 (reduced motion) for steps 25 and 27
    (0003:1332-1335). [the first build with a big-moment event [unsure: Slice 1 (a kill) or Slice 4]]
28. At Low, pixel ratio 1 and 2 with PM-0003-14's GPU times (OQ-0003-12); if MP's `devicePixelRatio` is below 1.5,
    shown on ML's panel or on MAC. Pick: 1 or 2 on Low (0003:1336-1339). [Slice 1]
29. The legacy touches LOOK_V2 does not name (OQ-0003-15): comic words, the kill vignette, the toy-soldier death fall,
    the skin rim, the camera-player cutaway, the ground grid, the red arena border, the world text labels. Pick keep,
    change or drop each (0003:1340-1343). [Slice 1 for most; the cutaway needs walls (greybox Slice 1, buildings
    Slice 5)]
30. Nothing to judge for LOOK_V2 §8 (0003:1344-1346). [n/a]
31. The Low frame (look floor; OQ-0003-16): Low's starting row (pixel ratio 1, scene scale 0.75, one 1024² sun map)
    beside High and legacy, for steps 2, 4, 6, 7 and 20; every `[DECIDED]` item still reads at Low. Pick per item:
    Low may only lower it or may switch it off, with PM-0003-9's costs; whether AO stays on at Low (0003:1347-1354).
    [Slice 1 for steps 2, 4, 6, 7; step 20's corners Slice 5; the costs need ML named]
32. The busy fight: 48 soldiers in three teams, the 4 very large units, 300 projectiles, at High, day and dusk
    (§5, §6.1, §6.4, §6.5): units, faction colours and effects brightest; notes on his soldier, the factions, a
    grenade, glow hiding units; OQ-0003-19 (factions by colour alone) (0003:1355-1359). [Slice 6]

### The bench checklist (beside PM-0003-38, -39 and -42; 0003:1361-1368)
1. Play the benchmark scene one minute per frame-rate option (a 60 fps cap, follow the display, a cap per tier;
   OQ-0003-10): which feels smoothest on his display; record his pick and refresh rate (0003:1364-1366). [as written,
   BS: Slice 6; any moving scene from Slice 1 can give an early feel [unsure]]
2. Listen through the benchmark fight: his own shots and shots at him always heard; no crackle, gap or dropout;
   achieved or not yet (0003:1367-1368). [Slice 2 (many shooters); BS Slice 6]

## 3. Open questions these two tests decide or feed

Format: id (source line): the question · who decides · what result answers it · the first real-game build that could
give it (my reading).

### ADR 0003's own (OQ-0003-n)
- OQ-0003-1 (0003:814): which laptop, desktop and Mac · Mustafa and the Advisor (answered in part, 2026-11-03: classes
  named, "All borrowed"; the actual machines still open, "named by Mustafa before the look proof test") · each named
  with GPU, CPU, RAM, screen, `devicePixelRatio`, `hardwareConcurrency` (PM-0003-1) · no build needed; it gates every
  timed mark from 2.5 on.
- OQ-0003-2 (0003:830): ink width, inner lines, weight on buildings (and so the ink family) · Mustafa, in the look
  proof test, with its measured cost · look checklist step 4 beside PM-0003-12's costs · Slice 1 (soldier and greybox
  blocks; the buildings' weight again at Slice 5).
- OQ-0003-3 (0003:841): colours as authored or tone-mapped · Mustafa · look checklist step 5 · Slice 1.
- OQ-0003-4 (0003:847): one sun shadow map enough, or cascades · the look proof test (softness, Mustafa) and the bench
  (shadow row, PM-0003-21), after a reading of three.js's WebGPU shadow options · step 6 and the shadow-pass p99 (3 ms
  Low, 2 ms High) · softness Slice 1; the row on BS Slice 6.
- OQ-0003-5 (0003:853): how warm sun with cool shadows and moods are lit (node toon material with coloured lights, or a
  custom TSL lighting model) · Mustafa, with each option's GPU time · steps 18 and 19 · warm and cool at Slice 1 on
  greybox; dusk at Slice 5 [unsure].
- OQ-0003-6 (0003:859): how the roof fade renders and how ink is masked · the look proof test (no frame above 1.5 x
  the median when a roof fades, ink hidden, and Mustafa's look) · step 21 with PM-0003-8 · Slice 5.
- OQ-0003-7 (0003:866): which technique draws fragments, flashes, tracers, impacts, debris · the look proof test (look)
  and the bench (cost: the effects row, at most 2 ms Low, 1 ms High, in the stress fight) · steps 24, 26 and
  PM-0003-21's effects row · Slice 1 for look and an early cost; the row at Slice 6.
- OQ-0003-8 (0003:872): how line boil is drawn (no options researched yet; a short reading first) · Mustafa · step 27
  with its cost against the post row · the first build with a big-moment event [unsure].
- OQ-0003-9 (0003:877): vegetation technique and tree style · Mustafa (the style, a later design session); the bench
  (the technique) · PM-0003-15, PM-0003-31 · Slice 5 [unsure: Slice 2].
- OQ-0003-10 (0003:884): frame rate capped or following a 120/144 Hz screen · Mustafa (feel), on the bench's numbers ·
  PM-0003-42 and bench checklist step 1 · smoothness from Slice 1; battery drain on a heavier scene later.
- OQ-0003-11 (0003:890): share of players on the WebGL 2 fallback · Mustafa, on client field data after launch · the
  backend each match reports · not these tests; field data is switched on at Slice 6 (0003:1245, 1411).
- OQ-0003-12 (0003:897): pixel ratio capped at 1 on Low · Mustafa · PM-0003-14 and step 28 · Slice 1.
- OQ-0003-13 (0003:902): audio listener placement · the camera framing first (R2-INT-OQ2), then Mustafa's listening
  check · not decided by these tests (0003:1246) · after the framing is answered.
- OQ-0003-14 (0003:907): answered by the verifier (2026-11-01); part 9 unchanged.
- OQ-0003-15 (0003:913): legacy touches LOOK_V2 does not name · Mustafa · step 29 · Slice 1 for most; the cutaway with
  walls.
- OQ-0003-16 (0003:920): the look floor, which items Low may switch off · Mustafa · step 31 with PM-0003-9's costs on
  ML · Slice 1 (the Low frame of the stand-in scene; the costs need ML named); step 20's corners at Slice 5.
- OQ-0003-17 (0003:936): answered (Advisor, confirmed by Mustafa, 2026-11-03): at most 6 of 600 presented frames,
  RD at High; it is PM-0003-44 · Slice 6.
- OQ-0003-18 (0003:950): reduced motion for the shake, flashes and line boil · Mustafa · step 27 (with step 25) · the
  first build with both a concussion and a big moment [unsure].
- OQ-0003-19 (0003:959): factions told apart by colour alone · Mustafa · step 32 (and step 11) · a three-faction view
  at Slice 2; step 32 as written at Slice 6.
- OQ-0003-20 (0003:965): answered (Advisor, 2026-11-03): "judge after 15 minutes of play"; the soaked set S gates
  PM-0003-18 to -21 on ML · applies from the first build those rows are measured on.

### The R3 ids (SPIKE_PLAN 411-414 for the look proof test, 522-527 for the bench; subjects from ADR 0003's Appendix A, 0003:1489-1522)
Who decides: the test's own result, read into the draft by graphics-engineer (0003:1229-1233); Mustafa where it is a
look call.
- R3-WGPU-OQ1 (0003:1489): Brave's backend on MP recorded · PM-0003-5, -16, step 1 · 2.5.
- R3-WGPU-OQ2 (0003:1490): `renderer.hasFeature` checks on Firefox and Safari · PM-0003-16 · 2.5.
- R3-WGPU-OQ3 (0003:1491): WebGPU's CPU cost against WebGL 2 (R3-WGPU-REC1 mark 3) · PM-0003-4 · a reading at Slice 1;
  as written Slice 6.
- R3-GL2-OQ3 (0003:1496): the fallback shows the same look (REC1 mark 2) · PM-0003-3 · Slice 1.
- R3-LOOK-OQ1 (0003:1499): the technique cost table · PM-0003-9 · Slice 1 (the roof fade at Slice 5).
- R3-LOOK-OQ3 (0003:1501): is single-pass AO good enough · Mustafa at step 20 · Slice 5 (greybox corners at Slice 1 as
  an early look).
- R3-LOOK-OQ4 (0003:1502): `renderer.info.memory.texturesSize` · PM-0003-9 · Slice 1.
- R3-MEASURE-OQ1 (0003:1515): `renderer.hasFeature` on Firefox and Safari · PM-0003-16 · 2.5.
- R3-MEASURE-OQ3 (0003:1516): `renderer.info.memory` and `memoryHeaps` on ML · PM-0003-16 · 2.5 (ML named).
- R3-GL2-OQ1 (0003:1494; 0003:577): WebGPU compatibility mode before WebGL 2 · "after a reading of three.js's WebGPU
  device request" · a source reading first; then any build on ML [unsure].
- R3-LOOK-OQ6 (0003:1504; 0003:299-300): post-process anti-aliasing · "after a reading of three.js's anti-aliasing
  nodes" · Slice 1 (the MRT pass and MSAA exist) [unsure who picks: a look call would be Mustafa's].
- R3-CHAR-OQ1 (0003:1505): one skinned mesh per soldier on both backends (after a reading of the skinning node's
  source) · PM-0003-25 · Slice 1 (the per-soldier count); Slice 2 at N = 48.
- R3-CHAR-OQ2 (0003:1506): C-E against C-B · PM-0003-27 · Slice 2.
- R3-CHAR-OQ4 (0003:1507): `BatchedMesh` against `InstancedMesh` · PM-0003-31 · Slice 5 [unsure: Slice 2].
- R3-CHAR-OQ6 (0003:1508): only if the draw-call budget is still missed after part 6 · PM-0003-22, -32 · Slice 6.
- R3-BUDGET-OQ5 (0003:1513): the triangle and memory rows, "with the first authored models" · PM-0003-22 · the first
  build with authored models (after ADR 0005's asset spike; Slice 5 by 0003:1407-1408), so not Slice 1's stand-in.
- R3-BUDGET-OQ6 (0003:1514): the shadow row, with OQ-0003-4 · PM-0003-21 · Slice 6.
- R3-AUDIO-OQ1 (0003:1517): the HRTF mark · PM-0003-38 · Slice 2 [unsure].
- R3-AUDIO-OQ2 (0003:1518): the voice counter in the benchmark fight · PM-0003-39 · Slice 2.
- R3-AUDIO-OQ6 (0003:1522): only if the built-in panners miss PM-0003-38's CPU mark · Slice 2.
- The bench also gives Mustafa the numbers for OQ-0003-10 (SPIKE_PLAN 525).

## 4. Step 2.5 and Slice 1 milestones (ADR 0003's migration plan, word for word)

The foundation's skeleton step (0003:1390-1393):
> 3. **The foundation** ([GLOSSARY §8](../research/GLOSSARY.md#the-foundation); ADR 0002). This record's share: the
>    renderer starts with `await renderer.init()`, reports its backend in the overlay and in every test screenshot, and
>    CI gates the render counts (W-A). *Milestone:* an empty scene runs through the full stack, CI green, and the
>    preview link names its backend.

Points at: `await renderer.init()` → part 3's warm-up, the base of PM-0003-7 and PM-0003-8; "reports its backend in the
overlay and in every test screenshot" → PM-0003-5 (and PM-0003-16's Brave record, PM-0003-46's overlay naming
"WebGPU"); "CI gates the render counts (W-A)" → PM-0003-36 (with PM-0003-2's error hook and PM-0003-46's backend record;
PM-0003-22's counts against a baseline); milestone → PM-0003-5, PM-0003-36, PM-0003-46.

Slice 1's renderer (0003:1397-1399):
> 5. **Slice 1, move & shoot:** one skinned soldier, the toon materials and ink kept in step 2, the feel kit's visual
>    effects and voices for the slice's weapons, split feedback. *Milestone:* programs constant over a match, zero
>    per-frame allocation in the render path, Mustafa's Brave check.

Points at: "one skinned soldier" → PM-0003-25 (C-B), PM-0003-26, PM-0003-28, PM-0003-29; "the toon materials and ink
kept in step 2" → PM-0003-17 (steps 2, 4, 5, 31; OQ-0003-2), PM-0003-3, PM-0003-9, PM-0003-10, PM-0003-12; "the feel
kit's visual effects and voices" → PM-0003-9 (5.k), PM-0003-41, PM-0003-39, PM-0003-38, look checklist step 26; "split
feedback" → no mark (part 10's split feedback is "proven at slice milestones", 0003:1247-1248); milestone "programs
constant over a match" → PM-0003-7 (and PM-0003-8); "zero per-frame allocation in the render path" → PM-0003-29,
PM-0003-36; "Mustafa's Brave check" → the slice-1 steps of PM-0003-17 and step 26's timing (0003:1329-1331).

Two neighbouring lines the build plan meets (for context; the 2026-11-14 decision overrides their order):
> 4. **The render budget bench**, after the look proof test and ADR 0002's boundary spike, before slice 1's renderer
>    work. *Milestone:* every row of the budget table on both machines, or a recorded change of tier for each technique
>    that misses, inside the look floor (part 7; OQ-0003-16). (0003:1394-1396)

> 10. **Slice 6, scale to 48:** the render budget bench re-run on the real game in the stress scene's fight on screen;
>     client field data switched on. *Milestone:* every budget row holds on both machines. (0003:1410-1411)

Step 5's "kept in step 2" assumes the look proof test already picked the ink family and the kept techniques; under the
2026-11-14 decision those picks come from the real game, so Slice 1 is where OQ-0003-2 and the first look of
OQ-0003-16 would be judged [my reading, not the draft's]. Test plan, 0003:1432: "Playtests: Mustafa's Brave check at
every slice that changes what a player sees or hears."

## 5. What Slice 1's renderer rests on

The marks or decisions that would force the most rework in Slice 1's rendering if they came out badly late, ranked
(my reading of the draft; I decide none of them). Each: why it bites, the smallest real-game build that measures it
honestly, and the machine it needs.

1. **The ink family and its weights** (OQ-0003-2; look checklist step 4; PM-0003-12). The hull and the screen-space
   pass differ in what the shared pass must output (normals, a per-pixel class value), in the post chain, in roof-fade
   masking, and in the Low draw cap (800 against about 1,100: 0003:198-199, 210-211). Smallest build: Slice 1, both
   families on the stand-in and greybox blocks, costs at 12 soldiers (24 and 48 as copies [unsure]). Machines: MP for
   Mustafa's pick; ML at Low for the costs; ML-Linux Firefox for the count.
2. **Frame time on the minimum laptop at Low with the look floor on** (PM-0003-13, PM-0003-21's post row at most 4 ms,
   PM-0003-9; OQ-0003-16). The look floor (toon, ink, one 1024² sun map, emissive colour) may not leave Low without
   Mustafa's answer (0003:1233-1236), so a miss means techniques or tiers redone. Smallest build: Slice 1 gives honest
   per-technique and post-row costs at 1920 x 1080 (full-screen passes cost by pixels [inferred]); the whole frame
   needs Slice 6. Machine: ML, named (the stand-in laptop gives rough checks only, never a pass).
3. **The same look on WebGPU and the WebGL 2 fallback** (PM-0003-2, PM-0003-3; R3-GL2-OQ3). Under F-A no effect may
   exist on one backend only (0003:125-137); a TSL node that differs on WebGL 2 is rewritten. Smallest build: Slice 1
   with the kept look. Machines: ML (Chrome, Firefox; ML-Linux Firefox) and MAC for Safari (nothing stands in for MAC).
4. **No first-sight shader stutter** (PM-0003-7, PM-0003-8, PM-0003-33's no-compile half). The warm-up scene behind
   the loading screen and the one-material-per-slot-content rule (parts 3, 5.a) are costly to retrofit. Smallest
   build: Slice 1 (counts in CI; hitches on ML at Low, fresh browser profiles). Machines: CI and ML.
5. **The soldier's draw and skinning scheme** (PM-0003-25, PM-0003-29, later PM-0003-26; R3-CHAR-OQ1). One skinned
   mesh per soldier at most 4 draws per pass is the base of the Low cap arithmetic (0003:357-358), and per-instance data
   from the blocks into preallocated arrays is the update path. Smallest build: Slice 1 for the per-soldier draw count
   and zero allocation, provided the stand-in is one skinned mesh at ADR 0005's 4-material cap; the posing cost at
   N = 48 waits for Slice 2. Machines: CI (counts), ML.
6. **The shared MRT scene pass and its memory** (PM-0003-10; PM-0003-16's MSAA reading; PM-0003-22's memory row).
   Every effect reads the one pass; 4x MSAA on its attachments is 199.2 MB at 1920 x 1080 by arithmetic against
   512 MB at Low (0003:292-298), and whether `info.memory.total` even counts it is unresearched. Smallest build:
   Slice 1. Machines: CI and ML.
7. **The lighting model and the colour output** (OQ-0003-5, OQ-0003-3; steps 5, 18). If the node toon material with
   coloured lights cannot reach "warm sun with cool shadows", a custom TSL lighting model replaces it (0003:186-188),
   touching every material. Smallest build: Slice 1 (greybox blocks with a sun and fill). Machines: MP for Mustafa's
   look; each option's GPU time on ML.
8. **Measurement that tells the truth and a scene that repeats** (PM-0003-6, PM-0003-34, PM-0003-35). Every timed mark
   and every CI baseline rests on them, and the replay and seeded presentation randomness are design choices.
   Smallest build: Slice 1 for the harness (a GPU-bound frame); the repeatable scene at Slice 1 if a local match
   replays identically [unsure], else Slice 2's fixed-seed bot match. Machines: RD (PM-0003-34), CI, ML and RD
   (PM-0003-35).

Just outside the eight: the teardown rule (PM-0003-45: nothing outlives its match; shared materials never disposed while
used), cheap at Slice 1 and costly to add later.

## 6. LOOK_V2 items by build

One line per `[DECIDED]` and `[DEFAULT]` item, in the order of ADR 0003's trace table (0003:504-541, which splits
LOOK_V2 §2's "colour and light" into three rows). Format: item · ADR 0003 part 5 technique · can Slice 1 (one stand-in
soldier, a small greybox test scene, one weapon, tracers, hit effects) show it, or which later content it needs (my
reading).

- §1 `[DECIDED]` cel-shaded comic rendering with bold ink · toon materials with shared ramps per class (5.a); the ink
  pass (5.b) (0003:504) · Slice 1: yes.
- §1 `[DECIDED]` iconic, RA-style unit silhouettes · authored models drawn whole; silhouette ink from depth edges (5.b);
  LOD levels keep the outline (part 6) (0003:505) · Slice 1: the soldier only, on the stand-in [unsure, 0003:479-481];
  vehicle Slice 4, very large unit Slice 6, authored models after ADR 0005's asset spike.
- §1 `[DECIDED]` clean, well-lit scenes with emissive accents · lighting from map data (5.g), soft shadows (5.f),
  selective emissive glow (5.d) (0003:506) · Slice 1: yes (muzzle flashes as the emissive accent).
- §1 `[DECIDED]` lush environments, made for an adult audience · props and vegetation instanced or merged (parts 6, 7);
  environment one step calmer (5.h) (0003:507) · later: the lush patch (Slice 5 [unsure: Slice 2's first map]).
- §2 `[DECIDED]` vibrant, beautiful colours · the output transform switch (5.e), half-float buffer (0003:508) ·
  Slice 1: yes.
- §2 `[DECIDED]` soft shadows · one sun `PCFShadowMap`, texel-snapped, following the player (5.f) (0003:509) · Slice 1:
  yes.
- §2 `[DECIDED]` geometry that reads clearly · ink (5.b), toon bands (5.a), AO in corners (5.c), value hierarchy (5.h)
  (0003:510) · Slice 1: on greybox geometry; buildings and interiors later (Slice 5).
- §2 `[DECIDED]` thick black outlines, especially strong on buildings · ink weight per material class, structures
  heaviest (5.b, 5.h) (0003:511) · Slice 1: soldier and greybox blocks as the structure class [unsure]; the two houses
  at Slice 5.
- §2 `[DECIDED]` props: logs and trees with variety · instanced or batched props, per-instance transform and colour;
  static merging by material (part 6) (0003:512) · later: Slice 2's map or Slice 5 [unsure]; code-built builders suffice.
- §2 `[DECIDED]` uniform details as faction identity · authored models with faction colour slots; one shared material
  per slot and faction, or per-instance colour (part 6) (0003:513) · Slice 1: one faction on the stand-in built from
  the Art project's sheets [unsure, 0003:479-481]; three factions at Slice 2.
- §2 `[DECIDED]` small movement touches (body reacting to strafing) · procedural animation (legacy GA rows), full rig
  for important soldiers under the budget LOD (part 6) (0003:514) · Slice 1: yes for strafing; the budget LOD crowd at
  Slice 2.
- §2 `[DECIDED]` architecture: half-timbered house with brick-tile roof; plastered house with silo tower · building
  assets under the toon materials and building ink (5.a, 5.b) (0003:515) · later: Slice 5 [unsure: Slice 2 with ported
  kit builders].
- §2 `[DECIDED]` grenade concussion · camera effect from the blast event, legacy's spring kick ported, page-side
  randomness (5.m) (0003:516) · needs a grenade: the first build with one [unsure: not in Slice 1 as the brief lists it].
- §3 `[DECIDED]` the audience is adults · no engine feature: the whole look (0003:517) · Slice 1: an early overall
  judgement; complete with the full content.
- §3 `[DECIDED]` slimmer, serious character proportions · one skinned mesh per soldier, any skin weights (part 6)
  (0003:518) · Slice 1 on the stand-in only if Mustafa accepts a code-built soldier for it (0003:479-481 says authored
  models are needed) [unsure].
- §3 `[DECIDED]` weapons with real silhouettes · authored weapon models on the hand sockets (legacy CC9 data)
  (0003:519) · Slice 1 on a code-built weapon, same caveat [unsure].
- §3 `[DECIDED]` the thatched roof goes · content only: the building kit drops it (0003:520) · with the first buildings
  (Slice 2 or 5).
- §3 `[DECIDED]` buildings sized for play: interiors, upper floors, verticality · AO in corners (5.c), the roof fade
  (5.i), floors from the sim (0003:521) · later: Slice 5.
- §3 `[DECIDED]` fragmentation comes back · pooled fragments as visual effects from the blast event (5.k) (0003:522) ·
  needs a grenade [unsure which build]; the pooled-effect path itself is Slice 1's (hit effects).
- §4 `[DECIDED]` lush countryside plus contested urban centres (enterable ruins, collapsed roofs, planks, corners) · the
  same pipeline; ruins as damage-stage meshes (5.j); planks as the sim's typed links (0003:523) · later: Slice 5.
- §4 `[DEFAULT]` damage stages: intact, damaged, ruin; debris and planks as routes · a mesh per stage swapped on the
  structure-state event, every stage in the warm-up; debris as effects (5.j, part 3) (0003:524) · later: Slice 5.
- §4 `[DEFAULT]` roofs fade when the player is inside · the cutaway over the player's enclosure, ink masked,
  precompiled blend variant (5.i) (0003:525) · later: Slice 5.
- §5 `[DECIDED]` iconic silhouettes you could draw from memory · as §1's silhouettes (0003:526) · as §1: soldier at
  Slice 1, the rest later.
- §5 `[DECIDED]` Planetary Annihilation's principle: readable far, depth up close · mesh LOD levels with hysteresis
  where the framing spreads distances; the budget LOD for animation (part 6) (0003:527) · Slice 1: one unit at two
  views on the stand-in; mesh LOD levels with ADR 0005's pipeline.
- §6.1 `[DEFAULT]` value hierarchy: environment one step calmer · a colour gain per material class (5.h) (0003:528) ·
  Slice 1: greybox ground against soldier and effects; the real environment at Slice 2/5.
- §6.2 `[DEFAULT]` line weight: thick on silhouettes, thinner inside · depth edges heavier than normal edges (5.b)
  (0003:529) · Slice 1: yes.
- §6.3 `[DEFAULT]` weapons and headgear first · per-unit budgets that favour them; no renderer feature (0003:530) ·
  Slice 1 on the stand-in [unsure, as §3].
- §6.4 `[DEFAULT]` glow language: objectives, faction tech, muzzle flashes · emissive slots into the emissive MRT
  output and selective bloom (5.d) (0003:531) · Slice 1: muzzle flashes; objectives and faction tech at Slice 4.
- §6.5 `[DEFAULT]` warm sun with cool shadows · coloured sun and fill lights per map (5.g) (0003:532) · Slice 1: yes on
  greybox.
- §6.5 `[DEFAULT]` soft ambient occlusion in corners, interiors read from above · GTAO at half resolution on the shared
  pass, denoised (5.c) (0003:533) · Slice 1: greybox corners; interiors at Slice 5.
- §6.5 `[DEFAULT]` different moods per map, for example dusk · the map document's lighting mood, applied at load (5.g)
  (0003:534) · Slice 5 by 0003:1408 [unsure: Slice 1 if its test scene carries two light sets].
- §7 `[DECIDED]` punchy weapons through VFX, sound and feedback · visual effects (5.k), voices (part 9), HUD feedback
  (part 10), hit feedback on confirmation (0003:535) · Slice 1: yes (look checklist step 26; timing at slice 1's Brave
  check).
- §7 `[DECIDED]` grenades: fragmentation first, with the concussion · as §3's fragmentation and §2's concussion (5.k,
  5.m) (0003:536) · needs a grenade [unsure which build].
- §7 `[DEFAULT]` comic impact touches for big moments: halftone flashes, ink line boil · halftone as legacy's DOM
  overlay (CD16); line boil (5.l, technique not researched: OQ-0003-8) (0003:537) · needs a big-moment event [unsure:
  Slice 1 (a kill) or Slice 4].
- §8 `[DECIDED]` v2 may use authored 3D models · glTF/GLB loaded and given the comic look per material slot (5.a)
  (0003:538) · not Slice 1 (the stand-in is code-built); after ADR 0005's asset spike, "before the first authored model
  enters the game" (0003:1407-1408).
- §8 `[DEFAULT]` the flow: sketch, Spline, glTF/GLB, automated step, the engine applies the look · step 5 is part 5.a;
  steps 3-4 are ADR 0005's (0003:539) · Slice 1 exercises step 5 only if the stand-in carries named material slots
  (0003:175-177) [unsure].
- §8 `[DEFAULT]` budgets per unit; never Spline's runtime · per-unit budgets from this record's totals; no Spline
  runtime (0003:540) · Slice 1: the stand-in counted against at most 4 draws per pass; the rest with authored models.
- §8 `[OPEN]` the tree style · nothing decided; placeholders are legacy's ported tree builders (0003:541) · not judged
  (Mustafa's, OQ-0003-9).

## 7. Count

Ids expected: 47 (PM-0003-1 to PM-0003-47). Rows written: 47, one per id, none merged (PM-0003-43 sits in the draft
after PM-0003-47, 0003:1223, and is placed in number order here). Ids not found: none. Section 6: 38 lines (25
`[DECIDED]`, 12 `[DEFAULT]`, 1 `[OPEN]`), as the trace table's rows 0003:504-541.

Points the build plan may want to weigh (readings, not decisions):
- ADR 0003 part 11 says "Authored models are needed because LOOK_V2 §3's proportions and weapon silhouettes cannot be
  judged on legacy's primitive parts" (0003:479-481), while the 2026-11-14 decision makes Slice 1's soldier a stand-in
  built in code from the Art project's sheets. Look checklist steps 10-12 (and the §3 items) are judgeable at Slice 1
  only if Mustafa accepts that stand-in for them; R3-BUDGET-OQ5 (triangle and memory rows) waits for authored models by
  its own words (0003:1513).
- Migration step 4 places the bench "before slice 1's renderer work" (0003:1394) and step 5 takes "the toon materials
  and ink kept in step 2" (0003:1397); both assume the spikes ran first. PM-0003-46's route starts "from the render
  budget bench on" (0003:1220), and PM-0003-20 pairs with R1-BND-REC1 mark 1 from ADR 0002's boundary spike
  (0003:1195): under the 2026-11-14 decision each needs a real-game start point.
- No timed mark can pass before Mustafa names ML, RD and MAC (OQ-0003-1): the stand-in laptop gives rough checks only,
  "never judged against a pass mark", and "nothing stands in for RD or MAC" (0003:1096-1099).
- Most bench rows (PM-0003-4, -13, -18, -21 in full, -22 in full, -30, -32, -44) are written for BS's full fight
  (48 soldiers, the 4 very large units, 300 projectiles), so they gate first at Slice 6; Slice 1 gives readings only.
  PM-0003-12's Low cap of 800 is tight even then: the whole frame is 799 to 817 by the draft's own sum (0003:366-370).
- PM-0003-28 and PM-0003-35 need a scene that replays identically on the client (ADR 0004's question; fallback: a
  fixed-seed bot match, 0003:409-410), which Slice 1 may not have.
- No mark is impossible on the real game; PM-0003-40 runs only if O-B occlusion rays are ever proposed, and PM-0003-27
  and -31 need alternative variants (C-D/C-E; `BatchedMesh`) built beside the chosen path for the comparison.
- SPIKE_PLAN and the draft agree on the mark ranges (SPIKE_PLAN 382, 511-514 against 0003:1075-1077) and on the calls
  left to Mustafa; I found no contradiction between them.
