# Claims ledger: ADR-0003's pass marks (test-engineer), 2026-10-31

A notes file beside `test-engineer-passmarks-0003.md`, in the plan's ledger shape (rule 9). It joins
`graphics-engineer-adr0003-claims.md` [C15]; rows are numbered TE1, TE2 … so they never clash with that ledger's
1–265. One row per claim or number in the pass-mark subsection and the spike entries. Lines are at 9e7db28 (the
research files are unchanged at the head: `git diff --quiet 9e7db28 HEAD -- docs/research/` exits 0). Short names:
r1 = `docs/research/r1-simulation-core.md`, r2 = `r2-netcode.md`, r3 = `r3-rendering.md`, r5 =
`r5-toolchain-quality.md`, GL = `docs/research/GLOSSARY.md`, INV = `docs/LEGACY_INVENTORY.md`, draft =
`graphics-engineer-adr0003-draft.md`, map = `engine-architect-decision-map.md`.

Tiers (the plan's): `research verified` = an R1 or R2 recommendation claim, or an R3 finding in the R3 verifier's
checked set (as listed at the head of the graphics-engineer ledger) · `research, source to check` = any other finding,
open question or legacy fact · `arithmetic` (shown) · `settled` (a STATUS code, GLOSSARY, the plan or the map's
assignment) · `design doc` (VISION_V2, LOOP_V2, LOOK_V2) · `proposed` (R3's or R5's proposed mark, "proposed (R3's)";
or the test-engineer's own, "proposed (test-engineer)", with its reason).

Mark codes: PRE the "How to read the table" bullets · PM-n the table row · FAIL the failure rule · NOM the "no
measurable mark" lines · LC the look checklist (step n) · BC the bench checklist · SE the spike entries.

## How to read the table

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE1 | ML: Intel Gen12 (Iris Xe) class or AMD equivalent integrated graphics, 1920 × 1080, Windows 11, browsers on WebGPU and with `forceWebGL` | PRE | r3 R3-BUDGET-REC1, 1096–1100 | proposed (R3's) |
| TE2 | ML's CPU at least 4 cores and 8 threads, "proposed in the research, no source" | PRE | r3 R3-BUDGET-REC1, 1097–1098, 1109–1110 | proposed (R3's, no source) |
| TE3 | ML-Linux: Firefox is the real fallback; Chrome runs WebGPU on Linux only on Intel Gen12+ | PRE | r3 R3-BUDGET-REC1, 1099–1100; R3-BUDGET-F4, 557–560 | proposed (R3's); research verified (F4) |
| TE4 | RD: mid-range discrete card one or two generations old, CPU named, 1920 × 1080 and 2560 × 1440 | PRE | r3 R3-BUDGET-REC1, 1100–1101 | proposed (R3's) |
| TE5 | MAC: Safari marks need a macOS machine; the draft names none; carries no budget row | PRE, SE | draft (no macOS machine in part 7 or the spikes' Needs, lines 280–287, 835–836); map part g, netcode proof test row ("Safari needs a macOS machine", line 941) | proposed (test-engineer: R3-WGPU-REC1 mark 1 names Safari 26) |
| TE6 | MP: Mustafa's RTX 5080 in Brave; only what he judges and Brave's backend; never a budget result | PRE | r3 R3-WGPU-OQ1, 1286–1289; R3-BUDGET-REC1, 1101–1102; legacy facts Q5, 590–591 | research, source to check |
| TE7 | CI: counts only, never a time | PRE | r3 R3-MEASURE-REC3, 1206–1209 | proposed (R3's) |
| TE8 | BS: a recorded match at the stress scene's fight on screen, scripted camera, URL flag | PRE | r3 R3-MEASURE-REC2, 1195–1197 | proposed (R3's) |
| TE9 | BS counts: N = 48 in three teams of 16; 4 very large units of at least 20 parts; 300 live projectiles with visual effects | PRE | GL §3 the stress scene, 397–420; very large unit, 438–442; MODES-3 (a) | settled (GLOSSARY) |
| TE10 | BS is a synthetic recording until ADR 0004's sim core spike gives one | PRE, SE | draft, render budget bench Needs, lines 880–881 | proposed (the draft's) |
| TE11 | LS: "one look-test scene" | PRE | r3 R3-LOOK-REC1, 991 | proposed (R3's) |
| TE12 | LS is a synthetic recording (no recorded match exists before the look proof test) | PRE, SE | map part g: the look proof test runs in parallel with the netcode proof test, before the sim core spike's results (line 945, order lines 962–965) | proposed (test-engineer: the only recording available at its place) |
| TE13 | H: 3 runs of 600 measured frames after 300 warm-up frames, with R3's labels | PRE | r3 R3-MEASURE-REC1, 1183–1185 | proposed (R3's) |
| TE14 | Extra labels: build commit, refresh rate, mode U or P, tier, mains power and power mode | PRE | none | proposed (test-engineer: a laptop's power state and the display's refresh move the numbers) |
| TE15 | Every one of the 3 runs must meet the mark | PRE | r3 R3-BUDGET-REC2 mark, 1162–1163 ("holds … for 3 runs") | proposed (test-engineer's reading of R3's mark) |
| TE16 | p99 = 594th of 600, p95 = 570th, median = 300th (nearest rank) | PRE | ceil(0.99 × 600) = 594; ceil(0.95 × 600) = 570; ceil(0.5 × 600) = 300 | arithmetic; the nearest-rank choice proposed (test-engineer: one definition so results repeat) |
| TE17 | A time mark naming no statistic is a p99; p99s are never added; totals measured directly | PRE | GL §1 time statistics, 141–147 | settled (GLOSSARY) |
| TE18 | A count mark holds on every frame (statistic: the run's max) | PRE | r3 R3-MEASURE-REC2 mark, 1203–1204 (counts repeat per frame) | proposed (test-engineer's reading) |
| TE19 | V: the 3 run medians within 3 % of each other | PRE | r3 R3-MEASURE-REC1 mark, 1192–1193 | proposed (R3's mark, applied by test-engineer to every timed set) |
| TE20 | An invalid set is neither pass nor fail; cause found before re-measuring | PRE, FAIL | r5 R5-TEST-REC6, 1490–1498 (a failure is a defect, reproduced before it is fixed, never retried into green) | proposed (R5's rule, applied to measurement) |
| TE21 | The render clock follows the display's refresh rate | PRE | r3 R3-BUDGET-F2, 545–549 | research, source to check |
| TE22 | A 60 Hz refresh period is 16.7 ms; presented frames last whole periods; median ≤ 14 ms cannot pass at 60 Hz; median ≤ 28 ms then needs half the frames in one period | PRE | 1000 / 60 = 16.7 ms; 14 < 16.7; 28 < 2 × 16.7 = 33.3, so the 300th frame must take one period | arithmetic (on R3-BUDGET-REC2's rows, 1123–1124, and F2) |
| TE23 | Mode U (frame-rate limit and vsync off) for budget and comparison marks; the switch per browser is not researched | PRE | none (grep of docs/research for an uncapped or vsync-off switch: none) | proposed (test-engineer: TE22; without it PM-0003-4 and PM-0003-37 could not fail) |
| TE24 | `performance.now()`: 5 µs when cross-origin isolated, 100 µs otherwise | PRE | r3 R3-MEASURE-F7, 641–646 | research verified |
| TE25 | The isolation flag is on for profiling builds | PRE | GL §5 preview isolation and the isolation flag, 636–642; r3 R3-MEASURE-OQ4, 1389–1391 | settled (GLOSSARY) |
| TE26 | Without isolation, a sub-millisecond mark passes only with 0.1 ms to spare | PRE | one timer step of 100 µs (TE24) | proposed (test-engineer) |
| TE27 | Chrome quantizes timestamp queries to 100 µs unless the WebGPU Developer Features flag is on | PRE | r3 R3-MEASURE-F2, 610–615 | research verified |
| TE28 | Developers profile per pass with that flag on the reference machines | PRE | r3 R3-MEASURE-REC4, 1217–1219 | proposed (R3's) |
| TE29 | No GPU timer: "GPU time not available"; no mark rests on a missing timer | PRE | r3 R3-GL2-REC3, 979–980 | proposed (R3's) |
| TE30 | The Worker's sim runs at 30 Hz, so 30 blocks per second | PRE | TICK-30 (STATUS:118–120); r1 R1-BND-REC2, 2265–2271 (one block per tick) | settled; research verified |
| TE31 | No mark here is set at 60 Hz; the 60 Hz margin is ADR 0001's server mark | PRE | GL §8 the 60 Hz margin (map J1, lines 1103–1116); plan, TICK-60-HEADROOM | settled |
| TE32 | "a check without a failing case can't be shown to work" | PRE | r5 R5-GATE-REC3, 1727–1728 | research, source to check |
| TE33 | Planted defects (every "Bites" entry) and rule B (busy-wait or extra full-screen pass of margin + 1 ms) | PRE, PM | none | proposed (test-engineer: proves each check can fail) |

## The look proof test's marks

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE34 | Machines named with GPU, CPU, RAM, screen, `devicePixelRatio`; every mark reported on both | PM-1 | r3 R3-BUDGET-REC1 mark, 1114–1116 | proposed (R3's) |
| TE35 | `hardwareConcurrency` recorded | PM-1 | r1 R1-BND-OQ3, 2728–2731 | research, source to check |
| TE36 | 0 console errors on WebGPU and `forceWebGL` in Chrome, Edge, Firefox (Windows) and Safari 26 | PM-2 | r3 R3-WGPU-REC1 mark 1, 910–912 | proposed (R3's) |
| TE37 | 0 errors checked in the 10 starts of PM-0003-5 | PM-2 | none | proposed (test-engineer: reuses PM-5's starts) |
| TE38 | Bite: a console error 2 s after load | PM-2 | r5 R5-GATE-REC3, G12 row, 1723 | proposed (R5's mutation) |
| TE39 | At most 1 % of pixels differ by more than 8/255 per channel; one-backend effects off | PM-3 | r3 R3-WGPU-REC1 mark 2, 912–913 ("proposed tolerance") | proposed (R3's) |
| TE40 | 3 fixed frames | PM-3 | r3 R3-MEASURE-REC2 mark, 1203–1204 | proposed (R3's) |
| TE41 | The fallback shows the same features (screenshot comparison) | PM-3 | r3 R3-GL2-REC1 mark, 965–967 | proposed (R3's) |
| TE42 | Decides R3-GL2-OQ3 | PM-3, SE | draft Appendix A, R3-GL2-OQ3 row (line 998) | settled (the draft's disposition) |
| TE43 | Control: one backend captured twice gives 0 differing pixels | PM-3 | none | proposed (test-engineer) |
| TE44 | WebGPU median frame time at most 1.1 × WebGL 2's, same machine | PM-4 | r3 R3-WGPU-REC1 mark 3, 913–915 | proposed (R3's) |
| TE45 | Decides R3-WGPU-OQ3 | PM-4, SE | r3 R3-WGPU-OQ3, 1294–1295; draft Appendix A (line 993) | research, source to check |
| TE46 | Bite: a 15 % busy-wait on the WebGPU path only | PM-4 | 15 % > the mark's 10 % margin (1.1 ×) | proposed (test-engineer) |
| TE47 | Backend named correctly in 10 of 10 starts per browser of F17–F19 (checked against `navigator.gpu`) | PM-5 | r3 R3-WGPU-REC4 mark, 950–951 | proposed (R3's) |
| TE48 | On ML-Linux, Firefox's overlay names the WebGL 2 backend | PM-5 | r3 R3-BUDGET-REC1 mark, 1115–1116 | proposed (R3's) |
| TE49 | The backend string on every test screenshot | PM-5 | r5 R5-GATE-REC1, G12, 1670; draft part 4 | proposed (R5's); the draft's decision |
| TE50 | Brave's backend recorded on MP | PM-5, PM-16, LC1 | r3 R3-WGPU-OQ1, 1286–1289 | research, source to check |
| TE51 | Chrome WebGL 2 reports GPU time; Firefox and Safari WebGL 2 report "GPU time not available", never 0 | PM-6 | r3 R3-GL2-REC3 mark, 984–985 | proposed (R3's) |
| TE52 | 600 of 600 frame records | PM-6 | H's 600 frames (r3 1183–1185) | proposed (R3's count) |
| TE53 | `programs` equal 5 s after deploy and at the end of a scripted 5-minute match | PM-7 | r3 R3-WGPU-REC3 mark 1, 937–938 | proposed (R3's) |
| TE54 | `renderer.info.memory.programs` counts active programs | PM-7 | r3 R3-MEASURE-F3, 617–622 | research verified |
| TE55 | The script shows damage stages and the roof fade as well as unit types and effects | PM-7, PM-8 | draft part 3 (lines 143–150); graphics-engineer ledger row 230 | proposed (the draft's) |
| TE56 | 3 runs | PM-7, PM-8 | R3-BUDGET-REC2's run count (r3 1162–1163) | proposed (test-engineer) |
| TE57 | No frame above 1.5 × the run's median at a first appearance | PM-8 | r3 R3-WGPU-REC3 mark 2, 938–939 | proposed (R3's) |
| TE58 | The 1 s window after a first appearance | PM-8 | r3 R3-WGPU-F22, 188–192 (pipelines created asynchronously) | proposed (test-engineer); research verified (F22) |
| TE59 | Fresh browser profile per run: a disk cache of compiled shaders could hide a missing variant | PM-8 | none (not researched) | proposed (test-engineer) |
| TE60 | Per technique and variant: GPU ms median and p95 over 600 frames at 1080p on both machines; extra draws; extra render-target memory | PM-9 | r3 R3-LOOK-REC1 mark, 998–1000 | proposed (R3's) |
| TE61 | `texturesSize` decides bloom target sizes | PM-9 | r3 R3-LOOK-OQ4, 1330–1331 | research, source to check |
| TE62 | The technique list (5.a, 5.b both families, 5.c, 5.d, 5.f, 5.i, 5.k, 5.l, 5.n) | PM-9 | draft part 5 (lines 159–258) | proposed (the draft's) |
| TE63 | Draws with every kept effect on at most 1.1 × (effects off + planned extras: one per outlined object, one per drawn object for a pre-pass) | PM-10 | r3 R3-LOOK-REC2 mark, 1014–1016 | proposed (R3's) |
| TE64 | Each effect's off and lowered settings cut its own GPU time by at least 40 % | PM-11 | r3 R3-LOOK-REC2 mark, 1016–1017 | proposed (R3's) |
| TE65 | Its statistic is a p99 (none named) | PM-11, PM-27, PM-33 | GL §1, 141–142 | settled (GLOSSARY) |
| TE66 | Both families: GPU and CPU frame time at 12, 24 and 48 soldiers on the minimum machine at Low | PM-12 | r3 R3-LOOK-REC3 mark, 1026–1027 | proposed (R3's) |
| TE67 | Low draw cap 800 with the screen-space ink; about 1,100 with a hull | PM-12, PM-22, PM-32 | r3 R3-BUDGET-REC2, 1133, 1146–1148 | proposed (R3's) |
| TE68 | "about 1,100" read as 1,100 for a pass mark | PM-12, PM-22, PM-32 | r3 1148 ("about 1,100 (proposed)") | proposed (test-engineer's reading) |
| TE69 | 4 × 48 × 2 = 384; 384 + 202 + 200 = 786; 786 + 192 + 80 = 1,058 | PM-12 | r3 R3-BUDGET-REC2 arithmetic, 1142–1147 (202 from CL9, 1144) | arithmetic |
| TE70 | Bite: 5 × 48 × 2 = 480 (+96): 786 + 96 = 882 > 800; with the hull 1,058 + 96 + 48 = 1,202 > 1,100 | PM-12 | on TE69 | arithmetic |
| TE71 | Firefox's WebGL 2 path is the draw count's worst case | PM-12, PM-32 | r3 R3-GL2-REC2, 969–972 | proposed (R3's) |
| TE72 | ML Low: median at most 28 ms, p95 at most 33.3 ms; RD High: median at most 14 ms, p95 at most 16.7 ms | PM-13, PM-18 | r3 R3-BUDGET-REC2 rows, 1123–1124 | proposed (R3's) |
| TE73 | 1000 / 30 = 33.3 ms; 1000 / 60 = 16.7 ms; 30+ fps and 60 fps | PM-13 | VISION_V2 §4 | arithmetic; design doc |
| TE74 | The WebGL 2 backend meets the minimum budget on the minimum machine | PM-13 | r3 R3-GL2-REC1 mark, 965–967 | proposed (R3's) |
| TE75 | RD at 1920 × 1080 and 2560 × 1440 both gate | PM-13, PM-18, PM-21 | r3 R3-BUDGET-REC1, 1100–1101 (both resolutions named for the recommended machine; the budget table names none) | proposed (test-engineer's reading; for the author's domain review) |
| TE76 | GPU time at pixel ratio 1 and 2 on the minimum machine (CD15's request) | PM-14 | r3 R3-BUDGET-OQ3, 1368–1369; legacy facts Q5, 587 (CD15) | research, source to check |
| TE77 | Lush patch within draw calls at most 800 and triangles at most 500k | PM-15 | r3 R3-BUDGET-REC2 rows, 1133–1134; draft OQ-0003-9 (lines 758–764: "a lush test patch within the draw-call and triangle rows") | proposed (R3's rows; the draft's test) |
| TE78 | `hasFeature` for timestamp queries on Firefox and Safari | PM-16 | r3 R3-WGPU-OQ2, 1290–1293; R3-MEASURE-OQ1, 1382–1383 | research, source to check |
| TE79 | `renderer.info.memory` and `memoryHeaps` (with Chrome's developer flag) on the minimum machine | PM-16 | r3 R3-MEASURE-OQ3, 1386–1388; R3-MEASURE-F2, 610–615 | research, source to check; research verified (F2) |
| TE80 | Mustafa judges every LOOK_V2 item in Brave beside legacy screenshots | PM-17, LC | LOOK_V2 §9; plan rule 12 [C10] | design doc; settled (plan) |

## The render budget bench's marks

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE81 | Chrome on both machines, Firefox on the minimum machine, WebGPU, and the WebGL 2 backend on the minimum machine | PM-18 | r3 R3-BUDGET-REC2 mark, 1162–1164 | proposed (R3's) |
| TE82 | "also on the WebGL 2 backend" read for both browsers on ML | PM-18 | r3 1163–1164 | proposed (test-engineer's reading) |
| TE83 | CPU per frame p99 at most 16 / 8 ms; animation p99 at most 1.0 / 2.0 ms | PM-19 | r3 R3-BUDGET-REC2 rows, 1125–1126 | proposed (R3's) |
| TE84 | Animation at most 1.0 ms p99 at Low with the budget LOD on the minimum machine | PM-19 | r3 R3-CHAR-REC2 mark, 1061–1062 | proposed (R3's) |
| TE85 | Page's cost of the sim p99 at most 1.0 ms per frame on both machines | PM-20 | r3 R3-BUDGET-REC2 row, 1127; GL §1 the client's sim share, 214–218 | proposed (R3's); settled (GLOSSARY) |
| TE86 | R1-BND-REC1 mark 1: p95 frame time with the sim in the Worker no worse than with no sim by more than 1.0 ms at N = 12, 24, 36, 48 on the minimum machine | PM-20 | r1 R1-BND-REC1 mark 1, 2259–2261 | research verified |
| TE87 | LE-B: the test-engineer states both statistics or picks one, with the reason | PM-20 | map f3, LE-B (line 915); draft lines 862–865 | settled (the map's assignment) |
| TE88 | Both gate, each in its own spike; they measure different things | PM-20 | reason: the R3 row times the page's own work; R1's difference includes garbage collection and core sharing (r1 2259–2261; R1-BND-OQ3, 2728–2731: page contention) | proposed (test-engineer) |
| TE89 | Map N12 cites R1-BND-REC1 mark 1 at r1:2258–2260; the mark is at 2259–2261 (2258 is its "Spike must prove" line) | (report) | r1 2258–2261, read 2026-10-31 | research, source to check (a citation slip for the critic) |
| TE90 | GPU p99: total 25 / 12 ms; shadow 3 / 2; main 16 / 6; post 4 / 3; visual effects, sprites, HUD 2 / 1 | PM-21 | r3 R3-BUDGET-REC2 rows, 1128–1132 | proposed (R3's) |
| TE91 | Post-processing covers ink, AO, glow and comic touches | PM-21 | draft part 7's table (line 305); graphics-engineer ledger row 125 | proposed (the draft's) |
| TE92 | Shadow row with OQ-0003-4 decides R3-BUDGET-OQ6 | PM-21, SE | r3 R3-BUDGET-OQ6, 1375–1378; draft Appendix A (line 1016) | research, source to check |
| TE93 | Draw calls 800 (1,100 with a hull at Low) / 1,500; triangles 500k / 1.5M; GPU memory 512 MB / 1 GB | PM-22 | r3 R3-BUDGET-REC2 rows, 1133–1135 | proposed (R3's) |
| TE94 | With the first authored models, decides R3-BUDGET-OQ5 | PM-22, SE | r3 R3-BUDGET-OQ5, 1373–1374; draft Appendix A (line 1015) | research, source to check |
| TE95 | Low: ratio at most 1, scene scale 0.75–1, one shadow map at 1024² or none; High: ratio at most 2 | PM-23 | r3 R3-BUDGET-REC2 rows, 1129, 1136 | proposed (R3's) |
| TE96 | The tier table holds ratio cap, scale and shadow size | PM-23 | r3 R3-BUDGET-REC3, 1166–1169 | proposed (R3's) |
| TE97 | JS heap at most 150 MB after 10 minutes, no growth over the last 5 | PM-24 | r3 R3-BUDGET-REC2 row, 1137 | proposed (R3's) |
| TE98 | "No growth" as at most 0.5 MB from minute 5 to minute 10 | PM-24 | r5 R5-TEST-REC5 mark, 1487–1488 (1 MB per 10 minutes); 1 MB × 5 / 10 = 0.5 MB | arithmetic on R5's proposed slope; its use for the client proposed (test-engineer) |
| TE99 | Heap read through the Chrome DevTools Protocol after a forced collection | PM-24 | none (not researched) | proposed (test-engineer) |
| TE100 | Bite: a planted leak, one array kept per frame | PM-24 | r5 R5-TEST-REC5 mark, 1488–1489 ("one buffer kept per snapshot") | proposed (R5's form) |
| TE101 | At most 4 draw calls per soldier per pass before outlines, 48 on screen, both backends | PM-25 | r3 R3-CHAR-REC1 mark, 1044–1045 | proposed (R3's) |
| TE102 | Divisor 48 × 2 passes (shadow and main) | PM-25 | r3 1142–1143 (192 per pass, two passes) | arithmetic |
| TE103 | Decides R3-CHAR-OQ1 (C-B checked on both backends) | PM-25, SE | r3 R3-CHAR-OQ1, 1344–1346; draft Appendix A (line 1007) | research, source to check |
| TE104 | Full rig: posing and bone upload p99 at most 2.0 ms on the minimum machine | PM-26 | r3 R3-CHAR-REC1 mark, 1045–1047 | proposed (R3's) |
| TE105 | C-D or C-E only with at least 20 % less total frame time on both machines | PM-27 | r3 R3-CHAR-REC1 mark, 1047–1048 | proposed (R3's) |
| TE106 | Decides R3-CHAR-OQ2 | PM-27, SE | r3 R3-CHAR-OQ2, 1347–1348 | research, source to check |
| TE107 | Identical pixels over 600 frames with culling off and on | PM-28 | r3 R3-CHAR-REC2 mark, 1062–1063 | proposed (R3's) |
| TE108 | 0 bytes per frame in the character update path over 60 s; health bars for 48 in at most 2 draws | PM-29 | r3 R3-CHAR-REC3 mark, 1074–1075 | proposed (R3's) |
| TE109 | Legacy's allocation method: an inspector sampling profile with objects collected by minor and major GC | PM-29 | INV TL15 (line 400); `tools/perfbots.js` lines 9–10; reports/2026-10-16-three-teams-core.md:54–55 (`--heap-prof` about 20× too low) | research, source to check |
| TE110 | 3 runs of 60 s; bite: one planted 64-byte allocation per frame must be reported | PM-29 | none | proposed (test-engineer: a sampling profile proves zero only if it sees a small leak) |
| TE111 | 4 very large units add at most 2.0 ms GPU (p99) and at most 200 draws, recommended machine, High, before ink or AO pre-pass | PM-30 | r3 R3-CHAR-REC4 mark, 1088–1092 | proposed (R3's) |
| TE112 | 4 × 20 × 2 = 160 draws, plus margin; bite 4 × 30 × 2 = 240 | PM-30 | r3 1089–1090 | arithmetic |
| TE113 | Render bundles stay only if they cut the WebGPU backend's frame time; a cut must exceed V's 3 % | PM-30 | r3 R3-CHAR-REC4, 1092; TE19 | proposed (R3's rule); the 3 % threshold proposed (test-engineer) |
| TE114 | BatchedMesh against InstancedMesh: CPU and GPU time in the benchmark scene | PM-31 | r3 R3-CHAR-OQ4, 1352–1354 | research, source to check |
| TE115 | BatchedMesh is judged on CPU time, not `drawCalls` | PM-31 | r3 R3-CHAR-REC4, 1083–1084 | proposed (R3's reason) |
| TE116 | Firefox WebGL 2: draws within the Low budget, median frame time at most 33.3 ms on the minimum machine's Linux run | PM-32 | r3 R3-GL2-REC2 mark, 975–977 | proposed (R3's) |
| TE117 | High to Low cuts GPU frame time by at least 50 %, no compile during the switch | PM-33 | r3 R3-BUDGET-REC3 mark, 1176–1177 | proposed (R3's) |
| TE118 | GPU total within 2 ms of frame time minus CPU work (median) when GPU-bound; back-to-back medians within 3 %; on the recommended machine | PM-34 | r3 R3-MEASURE-REC1 mark, 1191–1193 | proposed (R3's) |
| TE119 | GPU-bound when median GPU time is at least twice median CPU work | PM-34 | none | proposed (test-engineer) |
| TE120 | Identical `drawCalls` and `triangles` every frame; identical screenshots at three fixed frames | PM-35 | r3 R3-MEASURE-REC2 mark, 1203–1204 | proposed (R3's) |
| TE121 | CI fails on +10 % draw calls or +10 % GPU memory (1 of 1), under 5 minutes | PM-36 | r3 R3-MEASURE-REC3 mark, 1214–1215 | proposed (R3's) |
| TE122 | CI gates console errors, draw calls, triangles, programs, GPU memory and per-frame allocation in the render path | PM-36 | r3 R3-MEASURE-REC3, 1206–1208 | proposed (R3's) |
| TE123 | G12 runs R3-MEASURE-REC3's counts against a committed baseline | PM-36 | r5 R5-GATE-REC1, G12, 1670 | proposed (R5's) |
| TE124 | `--prove`: one mutation and one control per check; G12's control an endless CSS animation | PM-36 | r5 R5-GATE-REC3, 1706–1709, 1723 | proposed (R5's) |
| TE125 | Count baselines and their 10 % limits are committed data; raising a budget is a reviewed change | PM-36, FAIL | r5 R5-GATE-REC4, 1734–1738 | proposed (R5's) |
| TE126 | Control: a +5 % change passes | PM-36 | none | proposed (test-engineer: a control under the 10 % limit) |
| TE127 | Overlay closed within 1 % of a build without it; open within 3 % (median frame time) | PM-37 | r3 R3-MEASURE-REC4 mark, 1224–1225 | proposed (R3's) |
| TE128 | 1 % is below the 3 % two back-to-back runs may differ by, so 3 runs cannot resolve it | PM-37 | r3 1192–1193 against 1224–1225 | arithmetic (1 % < 3 %) |
| TE129 | 10 interleaved pairs; the median of the paired differences | PM-37 | none | proposed (test-engineer) |
| TE130 | Zero overlay work and allocation while closed (counted) | PM-37 | r3 R3-MEASURE-REC4, 1218 ("zero cost while closed"); draft part 8 (line 345) | proposed (R3's words, counted by test-engineer) |
| TE131 | 32 equal-power voices, 5-minute capture, no dropout (a listener and the "running" state), audio work under 0.5 ms p99 on the minimum machine; HRTF on 4 voices raises p99 by under 0.3 ms | PM-38 | r3 R3-AUDIO-REC1 mark, 1241–1243 | proposed (R3's) |
| TE132 | 3 runs of 5 minutes; bite: a 1 ms busy-wait in the voice update | PM-38 | none | proposed (test-engineer) |
| TE133 | Decides R3-AUDIO-OQ1 | PM-38, SE | r3 R3-AUDIO-OQ1, 1395–1396 | research, source to check |
| TE134 | Caps of 32 on Low and 48 on High (proposed start) | PM-39 | r3 R3-AUDIO-REC2, 1245–1247 | proposed (R3's) |
| TE135 | Live nodes never exceed the cap; shots at the player 0 dropped in 3 runs; no new AudioBuffer after warm-up | PM-39 | r3 R3-AUDIO-REC2 mark, 1254–1256 | proposed (R3's) |
| TE136 | Report the voices the fight asks for (a count in a fight at the largest mode's N) | PM-39 | r3 R3-AUDIO-OQ2, 1397–1398 | research, source to check |
| TE137 | O-B only: at most 1 ray per voice per 100 ms, at most 0.2 ms p99 per frame for all rays at the cap; 4 of 5 testers in a blind test | PM-40 | r3 R3-AUDIO-REC3 mark, 1267–1269 | proposed (R3's) |
| TE138 | Part 9 proposes zones or none, so PM-40 is conditional | PM-40 | draft part 9 (lines 364–365) | proposed (the draft's) |
| TE139 | Onset at most one rendered frame plus output latency over 100 shots; 0 autoplay warnings on all four browsers | PM-41 | r3 R3-AUDIO-REC4 mark, 1279–1280 | proposed (R3's) |
| TE140 | p99 of 100 shots is the 99th by nearest rank | PM-41 | ceil(0.99 × 100) = 99 | arithmetic |
| TE141 | Onset measured with the AudioContext's output timestamp and output latency | PM-41 | none (not researched) | proposed (test-engineer) |
| TE142 | Bite: sounds started one tick late (33.3 ms at 30 Hz) exceed one frame plus latency | PM-41 | 1000 / 30 = 33.3 ms > 16.7 ms (one frame at 60 Hz) | arithmetic; the bite proposed |
| TE143 | Cap options: a 60 fps cap, follow the display, a cap per tier | PM-42, BC1 | draft OQ-0003-10 (lines 765–770); r3 R3-BUDGET-OQ2, 1366–1367 | proposed (the draft's options); research, source to check |
| TE144 | Smoothness: per-frame displacement's standard deviation as a share of its mean on 60, 120 and 144 Hz | PM-42 | r1 R1-TICK-REC5 mark, 2111–2112 | research verified |
| TE145 | Battery drain over 10 minutes per option | PM-42 | none | proposed (test-engineer: "power on the minimum laptop" has no method in the draft or research) |
| TE146 | An upgrade changes p95 GPU and CPU frame time by at most 5 % without an explained cause | PM-43 | r3 R3-WGPU-REC2 mark, 925–926 | proposed (R3's) |
| TE147 | The change read between the medians of the 3 runs' p95s; bite: a planted 6 % slowdown | PM-43 | none | proposed (test-engineer) |

## Failure rule, no-mark lines, checklists, spike entries

| # | the claim or number (short) | where | citation (file, id, lines) | tier |
|---|---|---|---|---|
| TE148 | A failing run is never re-run until green; data kept; cause found | FAIL | r5 R5-TEST-REC6, 1490–1498 | proposed (R5's) |
| TE149 | A failed mark goes back to the draft by its Rollback and "Revisit when" routes, then to the advisor and to Mustafa and the Advisor | FAIL | draft Rollback (lines 940–952), Consequences "Revisit when" (lines 654–658); plan step 2 and rule 12 | settled (plan); the draft's routes |
| TE150 | Client field data is switched on in slice 6 | NOM | draft Migration plan step 10 (lines 916–917) | proposed (the draft's) |
| TE151 | Listener placement waits for the camera framing, then Mustafa's listening check | NOM | r3 R3-AUDIO-OQ3, 1399–1400; draft OQ-0003-13 | research, source to check |
| TE152 | The pop-in check of slice 3 is ADR 0001's mark: 0 rendered frames with an entity in view whose state is missing | NOM | r2 R2-INT-REC1 (a), 2437–2439 | research verified |
| TE153 | Neither spike runs under a network profile | NOM | draft "What a spike must prove" (lines 799–882: no profile named); Test plan's network line (lines 934–935) belongs to slice 3 | proposed (test-engineer's reading of the draft) |
| TE154 | Mustafa's PC is far above the recommended class, so it judges the look, never the budget | LC intro | r3 legacy facts Q5, 590–591; R3-BUDGET-REC1, 1101–1102 | research, source to check |
| TE155 | Legacy screenshots by legacy's pageshot, headed | LC intro | INV TL12 (line 397) | research, source to check |
| TE156 | Each checklist step's expected result quotes LOOK_V2 (§1 to §8) | LC 2–30 | LOOK_V2 §1–§8 | design doc |
| TE157 | Each pick names the draft's open question (OQ-0003-2, -3, -5, -6, -8, -12, -15) | LC 4, 5, 18, 21, 27, 28, 29 | draft "What a spike must prove", lines 807–812; Open questions, lines 714–797 | proposed (the draft's) |
| TE158 | Step 20 decides whether AO without the pre-pass is good enough | LC 20 | r3 R3-LOOK-OQ3, 1328–1329 | research, source to check |
| TE159 | Step 24: whether a fragment deals damage is a design question | LC 24 | plan rule 6; draft part 5.k | settled (plan) |
| TE160 | Step 26: the Imperial Warrior's SMG; the feel kit list; the shot instant, hit feedback on confirmation | LC 26 | draft trace table §7 row (line 437); LOOP_V2 §7 | design doc |
| TE161 | Step 30: §8's assets are plain glTF/GLB exports; no compression or LOD step | LC 30, SE | ASSET-RESEARCH (a) (advisor-verdicts.md §1, condition 4; map J4) | settled |
| TE162 | Spike names, what each proves, its owner and its subsumed ids | SE | map part g, lines 945–946 | settled (the map's assignment) |
| TE163 | The look proof test runs in parallel with the netcode proof test; the bench after the foundation, before slice 1 | SE | map part g order, lines 962–965 | settled (the map's proposal, for SPIKE_PLAN) |
| TE164 | The look proof test's test assets (one soldier, one weapon, a two-floor building with its damage stages, one very large unit of at least 20 parts, props) | SE | draft part 11 (lines 389–396) | proposed (the draft's) |
| TE165 | The draft places R3-MEASURE-REC1 and REC4 in the bench; the map places them in the look proof test | SE | draft lines 856–861; map line 945 | settled (both texts, read 2026-10-31) |
| TE166 | Decides lists per spike (R3-WGPU-OQ1–OQ3, R3-GL2-OQ1, OQ3, R3-LOOK-OQ1, OQ3, OQ4, OQ6, R3-MEASURE-OQ1, OQ3; R3-CHAR-OQ1, OQ2, OQ4, OQ6, R3-BUDGET-OQ5, OQ6, R3-AUDIO-OQ1, OQ2, OQ6) | SE | draft Appendix A (lines 991–1024); map part g (lines 945–946) | settled (the draft's dispositions) |
| TE167 | Sizes: both L (a week or more) | SE | none | proposed (test-engineer: the work listed in each entry) |

## Counts
- Rows: 167 (TE1–TE167), counted by script on this file.
- By tier (first tier of each row; script on this file): research verified 6 · research, source to check 23 · arithmetic 11 · settled 15 · design doc 3 · proposed 109.
- Proposed by test-engineer (method choices, readings and planted defects, each with its reason): 34 rows.
