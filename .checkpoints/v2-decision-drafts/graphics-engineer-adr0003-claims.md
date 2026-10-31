# Claims ledger: ADR-0003 (rendering and presentation), graphics-engineer, 2026-10-31

A notes file beside the draft (`graphics-engineer-adr0003-draft.md`), not part of it. One row per claim or number in
the draft. Lines are at 9e7db28. File short names: r1 = `docs/research/r1-simulation-core.md`, r2 = `r2-netcode.md`,
r3 = `r3-rendering.md`, r4 = `r4-physics-navigation.md`, r6 = `r6-infrastructure.md`, INV = `docs/LEGACY_INVENTORY.md`.

Tiers (the brief's): `research verified` = an R1 or R2 recommendation claim, or an R3 finding the R3 verifier's table
lists as checked (r3:1548–1593, the four extras at r3:1541–1542 and R3-BUDGET-F6 in the second pass at r3:1627–1628:
R3-WGPU-F1, F3–F11, F14–F18, F22; R3-GL2-F1, F2, F4, F5, F7–F9, F12, F13; R3-LOOK-F5, F9, F10, F13; R3-CHAR-F1, F2,
F4, F7, F9, F12; R3-BUDGET-F1, F4, F6; R3-MEASURE-F1–F4, F7, F8; R3-AUDIO-F2, F3, F5, F8, F10) · `research, source
to check` = every other finding (R3 findings outside that set, the legacy facts the topics cite from the inventory, R4
and R6 recommendations, R2 and R1 open-question statements, and "not researched" claims, checked by grep of
`docs/research/r*.md`) · `arithmetic` (shown) · `settled` (a STATUS code or GLOSSARY §8) · `design doc` (VISION_V2,
LOOP_V2, EDITOR_V2, LOOK_V2) · `proposed mark` (R3's recommendations propose every pass mark and budget: r3:46 "Every
pass mark and every reference machine is proposed", r3:534–538; the reason is the recommendation's own, cited).

Section codes: H header · C Context · D1–D11 Decision parts (D5a–D5n its sub-parts) · T the trace table · O Options ·
TA Trade-off analysis · CQ Consequences · INV the inventory table · OQ Open questions · S What a spike must prove ·
M Migration plan · TP Test plan · R Rollback · A Appendix A.

## Context and header

| # | the claim, as written (short) | section | citation (file, id, lines) | tier |
|---|---|---|---|---|
| 1 | Minimum: laptop with integrated graphics runs the largest mode at 30+ fps on Low; recommended: mid-range card 60 fps on High; four desktop browsers | C | VISION_V2 §4 | design doc |
| 2 | §6.9: "the renderer handles a few very large units and their effects without dropping frames" | C | VISION_V2 §6.9 | design doc |
| 3 | §6.2, §6.4, §6.7 needs as listed; §6.8 kill-cams, replays, highlights are [LATER] | C | VISION_V2 §6.2, §6.4, §6.7, §6.8 | design doc |
| 4 | Feel kit list (kick, muzzle flash, tracer, impact, hit marker, sound, target flinch, kill confirm); split feedback quote; targets flinch | C | LOOP_V2 §7 | design doc |
| 5 | HUD and scoreboard for three teams | C | LOOP_V2 §11 (item 1) | design doc |
| 6 | LOOK_V2 §9 asks this record to show each [DECIDED] item, look proof test judged by Mustafa in the browser side by side with legacy | C | LOOK_V2 §9 | design doc |
| 7 | [DEFAULT] items incl. §6 additions built as defaults; tree style §8 [OPEN] | C | LOOK_V2 header tags, §6, §8 | design doc |
| 8 | EDITOR_V2 §2 rule 3 quote "Shared materials and textures are cached and reused, never rebuilt per edit" | C | EDITOR_V2 §2 | design doc |
| 9 | The stack: TypeScript client on the latest three.js with WebGPU renderer and WebGL 2 fallback | C | V2_PHASE0_BRIEF §0 line 8; brief §1 | settled |
| 10 | N = 48, the largest mode 24v24 | C | MODES-3 (a) (STATUS:104–105) | settled |
| 11 | Where an R3 mark gives 48 and 54, 48 holds | C | carry-forward 3 (STATUS:133–134) | settled |
| 12 | SLICE-ORDER: sim in a Web Worker behind the network's messages from slice 1; netcode proof test before the foundation | C | SLICE-ORDER (STATUS:94–101); GLOSSARY §8 (decision map J1) | settled |
| 13 | SEC-SPREAD (a): spread server-only; tracers use a similar-looking spread the hit corrects | C, D5k | STATUS:108–109 | settled |
| 14 | TICK-30: 30 Hz baseline | C | STATUS:118–120 | settled |
| 15 | The client's sim share at 30 Hz is ADR 0004's number | C | decision map N11 (owner 0004) | settled |
| 16 | LOOK_V2 is the look where [DECIDED]; authored glTF/GLB allowed | C | brief §1; LOOK_V2 §8 | settled |
| 17 | UPGRADE-SCOPE: tracks live on the player; the page shows owner's offers and tracks (I-21) | C, D10 | inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md; decision map J5, I-21 | settled |
| 18 | SPIKE-SCOPE (a): spikes throwaway in spikes/, exempt from legacy constraints | C, S | STATUS:187–188 | settled |
| 19 | R7-MARKS (a): marks are working recommendations; inventory changes after acceptance | C, INV | STATUS:159–160; plan revision 3 "Not touched" | settled |
| 20 | TSL "capable of encoding code into different outputs such as `WGSL`/`GLSL`" | C, D1 | r3 R3-WGPU-F11, 127–131 | research verified |
| 21 | Custom GLSL, onBeforeCompile and EffectComposer "not supported in WebGPURenderer" | C, D1 | r3 R3-WGPU-F6, 97–102 | research verified |
| 22 | Legacy skin rim (CL3) and hull shader (CL6) do not carry over | C | r3 legacy facts Q1, 199–200 (CL3, CL6) | research, source to check |
| 23 | Fallback compute = transform feedback, no indirect dispatch | C, D1 | r3 R3-GL2-F2, 219–223 | research verified |
| 24 | Fallback storage buffers packed into textures, no atomics | C, D1 | r3 R3-GL2-F3, 225–232 (texture packing and no atomics inferred there) | research, source to check |
| 25 | Render bundles bring no gain on the fallback | C, D1 | r3 R3-GL2-F9, 262–265 | research verified |
| 26 | Who falls back: Firefox Linux and Intel Macs, Chrome Linux without Intel Gen12+, Safari before 26 | C, OQ11 | r3 R3-GL2-F12, 279–283 | research verified |
| 27 | Hull adds one draw per outlined object; screen-space one full-screen pass whatever the object count | C, D5b, TA | r3 R3-LOOK-F8, 366–370 | research, source to check |
| 28 | No source read gives GPU ms for any look technique on any hardware | C | r3 R3-LOOK-F15, 411–413 | research, source to check |
| 29 | Per-instance compute skinning is WebGPU only in three.js's example | C | r3 R3-CHAR-F9, 482–488 | research verified |
| 30 | Headless Chrome quotes "Software only, hardware acceleration unavailable." and "Hardware accelerated but at reduced performance." | C | r3 R3-MEASURE-F8, 648–653 | research verified |
| 31 | Legacy CD15: one r128 WebGLRenderer, MSAA, ratio capped at 2, PCFSoftShadowMap, LinearEncoding ("authored hex = on-screen colour") | C | r3 legacy facts Q1, 195–198 (CD15) | research, source to check |
| 32 | r186 removed PCFSoftShadowMap; "Use PCFShadowMap which is now soft as well." | C, D5f | r3 R3-WGPU-F3, 78–83 | research verified |
| 33 | About 120 draws per on-screen soldier; 1,480 with 12 on screen, 5,843 with 48 | C, TA | r3 legacy facts Q4, 518–519 (CL6, TL15) | research, source to check |
| 34 | Static merging cut the village from 398 to 202 draws | C | r3 legacy facts Q4, 523 (CL9) | research, source to check |
| 35 | Soldier animation 26 to 51 µs each, 1.2 to 2.4 ms at 48; "Nothing culls or simplifies animation…" | C | r3 legacy facts Q4, 515–517 (GA15) | research, source to check |
| 36 | GW17 fades structures between camera and player and hides their ink | C | INV GW17, line 205 | research, source to check |
| 37 | Toggling material.transparent can mean a new pipeline on first use | C, OQ6 | r3 legacy facts Q1, 201–202 (GW17) | research, source to check |
| 38 | Audio synthesized; "there is no voice limit and no occlusion" | C | r3 legacy facts Q7, 760–765 (GP6) | research, source to check |
| 39 | Halftone is a CSS multiply layer, "not a render pass", off by default | C, D5l, INV | INV CD16, line 97 | research, source to check |
| 40 | Takes/gives lists (block I-2, command I-3, aim point I-19, snapshot fields and interpolation buffer 0001, parts/floors/states/events 0004, recordings I-17, assets/slots/LODs/budgets/mood 0005 I-8 I-9, field data storage 0006 I-18; gives I-15, N41–N48, N12, I-18, I-19) | C | decision map parts a (ADR 0003), c4, d | settled (the map's assignment; plan step 2) |

## Decision

| # | the claim, as written (short) | section | citation (file, id, lines) | tier |
|---|---|---|---|---|
| 41 | One WebGPURenderer, every material and post effect in TSL; WebGL 2 backend fallback | D1 | r3 R3-WGPU-REC1, 902–915 | proposed mark (R3's recommendation; reason F11, F6) |
| 42 | Same features on both backends (F-A); WebGPU-only extras only with named fallback and measured gain | D1 | r3 R3-GL2-REC1, 955–967 | proposed mark (R3's recommendation) |
| 43 | No draw-call budget depends on BatchedMesh or multi-draw; Firefox WebGL 2 is the worst case | D1 | r3 R3-GL2-REC2, 969–977 | proposed mark (R3's recommendation) |
| 44 | GPU timers only in Chrome and Edge on the fallback | D1, D4 | r3 R3-GL2-F5, 240–245 | research verified |
| 45 | No multi-draw on Firefox: each BatchedMesh entry is its own draw | D1 | r3 R3-GL2-F7, F8, 252–260 | research verified |
| 46 | Some depth and MSAA paths need extensions on the fallback | D1 | r3 R3-GL2-F10, 267–271 | research, source to check |
| 47 | Pin the newest release when the look proof test starts; r186 when researched | D2 | r3 R3-WGPU-REC2, 917–926; R3-WGPU-F1, 67–71 | research verified (F1); proposed mark (the policy) |
| 48 | API broke in r183, r185, r186 within a year | D2, O | r3 R3-WGPU-F3, 78–83 | research verified |
| 49 | Old ADR-003 entry ("stay on r128 vs upgrade", STATUS:202) disposed: the stack settles it | D2 | STATUS:202; decision map J2 (ADR-003 row) | settled |
| 50 | Warm-up: await init, compileAsync on every variant; the match creates no new programs | D3 | r3 R3-WGPU-REC3, 928–939 | proposed mark (R3's recommendation) |
| 51 | WebGPU creates pipelines asynchronously | D3 | r3 R3-WGPU-F22, 188–192 | research verified |
| 52 | "shader compilation stutter" (three.js's words) | D3 | r3 R3-WGPU-F10, 121–125 | research verified |
| 53 | Legacy compiles on first sight: 16 programs against 9 | D3 | r3 legacy facts Q1, 203–205 (TL15) | research, source to check |
| 54 | Backend logged, overlay, screenshots print it; secure context | D4 | r3 R3-WGPU-REC4, 941–951 | proposed mark (R3's recommendation) |
| 55 | Renderer picks its backend itself | D4 | r3 R3-WGPU-F5, 91–95 | research verified |
| 56 | WebGPU needs a secure context | D4 | r3 R3-WGPU-F20, 177–180 | research, source to check |
| 57 | Fallback GPU time: timer extension where present, "not available", never 0 | D4 | r3 R3-GL2-REC3, 979–985 | proposed mark (R3's recommendation) |
| 58 | One shared scene pass with MRT; effects reading buffers never re-render; planned extras counted; tier switch per effect | D5 | r3 R3-LOOK-REC2, 1002–1017 | proposed mark (R3's recommendation) |
| 59 | Each technique measured alone, per pass and target, both machines and backends | D5 | r3 R3-LOOK-REC1, 991–1000 | proposed mark (R3's recommendation, changed to serve LOOK_V2) |
| 60 | MeshToonNodeMaterial is three.js's node toon material | D5a | r3 R3-LOOK-F1, 318–321 | research, source to check |
| 61 | Ramp is one texture lookup in lighting, no extra pass, both backends; each ramp a distinct material | D5a | r3 R3-LOOK-F2, F3, 323–334 | research, source to check |
| 62 | Toon model reads only the ramp's red channel (`return vec3( gradientMap.r );`) | D5a, OQ5 | r3 R3-LOOK-F2, 323–329 | research, source to check |
| 63 | Custom TSL lighting is the options table's second toon variant | D5a, OQ5 | r3 Options Q3, 809 | research, source to check |
| 64 | Slot names are ADR 0005's (I-8) | D5a | decision map I-8, seam S9 | settled (the map's assignment) |
| 65 | Depth-and-normal edges are a custom TSL node over MRT (no shipped node) | D5b | r3 R3-LOOK-F7, 360–364 | research, source to check |
| 66 | Depth edges for silhouettes, normal edges for inner lines (inferred) | D5b | r3 R3-LOOK-F7 (inferred in the draft); decision map h G8 (inner lines: 0 findings) | research, source to check |
| 67 | A per-class weight in screen space is not researched | D5b, OQ2 | grep of docs/research for a per-class or per-material line weight: none (decision map h, G8) | research, source to check |
| 68 | Screen-space keeps the Low cap at 800 at N = 48; the hull needs about 1,100 | D5b, TA | r3 R3-BUDGET-REC2, 1142–1148 | arithmetic (rows 103–106) |
| 69 | three.js's toonOutlinePass is an inverted hull, only toon materials | D5b, O | r3 R3-LOOK-F4, 337–341 | research, source to check |
| 70 | Hull: back-face copy pushed out in clip space, default thickness 0.003, no per-vertex width | D5b, O | r3 R3-LOOK-F5, 343–350 | research verified |
| 71 | Legacy hull in world units (0.09, characters 0.8x) with ground clamp | D5b, O | INV CL6, line 115; r3 legacy facts Q3, 421–423 | research, source to check |
| 72 | Both families measured at 12, 24, 48 soldiers on the minimum laptop at Low | D5b, S | r3 R3-LOOK-REC3, 1019–1028 | proposed mark (R3's recommendation) |
| 73 | Screen-space: needs masking for the roof fade; threshold tuning; thin lines alias | D5b, O | r3 Options Q3, 810–813 | research, source to check |
| 74 | Picking the hull: Low cap about 1,100 or hull High-only | D5b, R | r3 R3-BUDGET-REC2, 1146–1148 | proposed mark (R3's recommendation) |
| 75 | GTAO is three.js's node; documented pre-pass for normals and depth | D5c, O | r3 R3-LOOK-F9, 373–377 | research verified |
| 76 | "a value of `0.5` should be sufficient for most scenes"; noise needs TRAA ("ghosting and temporal instabilities") or DenoiseNode | D5c, O | r3 R3-LOOK-F10, 379–384 | research verified |
| 77 | Documented setup draws the scene twice | D5c, O, TA | r3 R3-LOOK-F11, 386–389 | research, source to check |
| 78 | TAA ghosting would show on fast-moving soldiers | D5c | inference in the draft from R3-LOOK-F10 (marked "inferred") | research, source to check |
| 79 | Whether single-pass AO is good enough is open | D5c | r3 R3-LOOK-OQ3, 1328–1329 | research, source to check |
| 80 | Selective bloom via the emissive property and MRT (quote) | D5d, O | r3 R3-LOOK-F12, 392–395 | research, source to check |
| 81 | Full-image bloom "Washes bright UI-like colours" | D5d, O | r3 Options Q3, 817 | research, source to check |
| 82 | Bloom: twelve full-screen passes at falling resolution | D5d, O | r3 R3-LOOK-F13, 397–402 | research verified |
| 83 | One full-screen half-float RGBA target is 16.6 MB at 1920 × 1080 | D5d, O | r3 R3-LOOK-F14, 405–409 (1920 × 1080 × 8 B = 16,588,800 B) | arithmetic |
| 84 | RenderPipeline tone-maps and converts colour space by default; `outputColorTransform` switch | D5e, OQ3 | r3 R3-WGPU-F12, 133–137 | research, source to check |
| 85 | Research has no options section on the output transform | D5e, O | r3 Options, 774–894 (no colour-output table); R3-WGPU-OQ4, 1296–1298 | research, source to check |
| 86 | The internal buffer defaults to half float | D5e | r3 R3-WGPU-F14, 144–148 | research verified |
| 87 | Shadow row: Low one map, 1024² or off, ≤ 3 ms; High ≤ 2 ms | D5f, D7 | r3 R3-BUDGET-REC2, 1129 | proposed mark (R3's) |
| 88 | Legacy texel-snapped, player-following sun frustum | D5f, INV | INV GW4, line 192 | research, source to check |
| 89 | Cascades not researched | D5f, OQ4 | r3 R3-BUDGET-OQ6, 1375–1378 | research, source to check |
| 90 | Lighting model for warm sun / cool shadows not researched | D5g, OQ5 | decision map h, G7 (grep of r*.md) | research, source to check |
| 91 | Mood as map data (I-9) | D5g | decision map I-9 | settled (the map's assignment) |
| 92 | Legacy nature gain on environment materials, 1.56 | D5h | INV CL4, line 113 | research, source to check |
| 93 | Roof fade: port GW17 as a fade of flagged tall statics | D5i | INV GW17 note, line 205 | research, source to check |
| 94 | Floor layers and area ids shared with interest (which enclosure) | D5i, T | r4 R4-FLOORS-REC2, 925–939 | research, source to check |
| 95 | "The client's cutaway (GW17) follows the same rule, so what is not sent is also what the screen would not show" | D5i, A | r2 R2-INT-REC2, 2445–2466 (2450–2451) | research verified |
| 96 | What a player may see is R2-INT-OQ1, a design question | D5i | r2 R2-INT-OQ1, 3005–3009; decision map b (owner DESIGN) | research, source to check |
| 97 | The player's own area is connected to itself, so LOOK_V2 §4's case is inside the rule | D5i | reading of R2-INT-REC2 (an enemy in an area is relevant if the viewer's area is connected to it), r2:2448–2450 | research verified |
| 98 | Fade technique not researched | D5i, OQ6 | decision map h, G3 | research, source to check |
| 99 | R4-OBST-REC2 bakes two states; a third stage is 0004's and 0005's | D5j, T | r4 R4-OBST-REC2, 1043–1061; decision map h, G4 | research, source to check |
| 100 | Planks and debris as routes are typed links (R4-FLOORS-REC1) | D5j, T | r4 R4-FLOORS-REC1, 903–924 | research, source to check |
| 101 | Visual effects pooled per kind with caps (legacy GP1); caps as tier data | D5k | INV GP1, line 311; r3 R3-BUDGET-REC3, 1166–1177 | research, source to check (GP1); proposed mark (REC3) |
| 102 | Visual-effects row ≤ 2 ms Low, ≤ 1 ms High | D5k, D7 | r3 R3-BUDGET-REC2, 1132 | proposed mark (R3's) |
| 103 | Particle technique not researched | D5k, OQ7 | decision map h, G5 (particles appear only as option F-B's example, r3:797) | research, source to check |
| 104 | Whether a fragment deals damage is a design question | D5k, T | plan rule 6; decision map e2 (§3 fragmentation row) | settled (plan rule 6, the advisor's condition 9) |
| 105 | Line boil technique not researched | D5l, OQ8 | decision map h, G2 (0 hits) | research, source to check |
| 106 | Legacy spring camera kick and concussion (CD9, GP4) | D5m, INV | INV CD9 line 90; GP4 line 314 | research, source to check |
| 107 | "Presentation (effects, sound, camera shake, animation variety) uses its own client-side generator" | D5m, D10 | r1 R1-DET-REC4, 2184–2207 (2192–2193) | research verified |
| 108 | MSAA "4 samples are used by default" | D5n, O | r3 R3-WGPU-F15, 150–152 | research verified |
| 109 | MSAA multiplies multisampled MRT attachments' memory by four | D5n | r3 R3-LOOK-F14, 405–409 | research, source to check |
| 110 | Post-process AA not researched as an option | D5n | r3 R3-LOOK-OQ6, 1335–1340 | research, source to check |
| 111 | One skinned mesh per soldier (C-B), any skin weights; instanced only with ≥ 20 % total frame time cut on both machines | D6 | r3 R3-CHAR-REC1, 1032–1048 | proposed mark (R3's) |
| 112 | The character model is ADR 0005's (R3-CHAR-OQ3) | D6 | r3 R3-CHAR-OQ3, 1349–1351; decision map b (owner 0005) | settled (the map's assignment) |
| 113 | Budget LOD (L-2); padded sphere bounds | D6 | r3 R3-CHAR-REC2, 1050–1063 | proposed mark (R3's) |
| 114 | three.js LOD switches at distances with hysteresis | D6, T | r3 R3-CHAR-F10, 490–493 | research, source to check |
| 115 | Camera framing is a design question LOOK_V2 does not settle | D6, OQ13 | r2 R2-INT-OQ2, 3010–3013; r3 R3-CHAR-OQ5, 1355–1356; decision map b (owner DESIGN); LOOK_V2 (no camera item) | research, source to check |
| 116 | Per-instance data from the block into preallocated arrays; bars and flashes instanced | D6 | r3 R3-CHAR-REC3, 1065–1075 | proposed mark (R3's) |
| 117 | instanceColor per instance on InstancedMesh and BatchedMesh | D6, T | r3 R3-CHAR-F4, F6, 452–467 | research verified (F4); research, source to check (F6) |
| 118 | Very large units as part groups (V-A); static merging; bundles only with a gain | D6 | r3 R3-CHAR-REC4, 1077–1092 | proposed mark (R3's) |
| 119 | Minimum machine class: Intel Gen12 (Iris Xe) or AMD equivalent, 1920 × 1080, Windows 11, Chrome and Firefox WebGPU, WebGL 2 forced, plus a Linux run | D7 | r3 R3-BUDGET-REC1, 1096–1116 | proposed mark (R3's) |
| 120 | CPU floor of 4 cores and 8 threads (proposed, no source) | D7 | r3 R3-BUDGET-REC1, 1096–1098, 1109–1110 ("a proposal with no source") | proposed mark (R3's, no source) |
| 121 | Recommended: named desktop, mid-range discrete card one or two generations old, 1080p and 1440p | D7 | r3 R3-BUDGET-REC1, 1100–1102 | proposed mark (R3's) |
| 122 | RTX 5080 far above mid-range; a check above the class and the Brave check | D7 | r3 legacy facts Q5, 590–591; R3-BUDGET-REC1, 1101–1102 | research, source to check |
| 123 | Budget table, every row (frame time p95 ≤ 33.3 / ≤ 16.7 ms; median ≤ 28 / ≤ 14; CPU ≤ 16 / ≤ 8; animation ≤ 1.0 / ≤ 2.0; sim ≤ 1.0 / ≤ 1.0; GPU ≤ 25 / ≤ 12; shadow ≤ 3 / ≤ 2; main ≤ 16 / ≤ 6; post ≤ 4 / ≤ 3; visual effects ≤ 2 / ≤ 1; draws ≤ 800 (≤ 1,100 with hull) / ≤ 1,500; triangles ≤ 500k / ≤ 1.5M; memory ≤ 512 MB / ≤ 1 GB; ratio 1 and scale 0.75–1 / ratio 2; heap ≤ 150 MB) | D7 | r3 R3-BUDGET-REC2, 1121–1137 | proposed mark (R3's) |
| 124 | p95 ≤ 33.3 ms is 30 fps; p95 ≤ 16.7 ms is 60 fps | D7 | 1000 / 30 = 33.3 ms; 1000 / 60 = 16.7 ms (VISION_V2 §4) | arithmetic |
| 125 | Post-processing row covers ink, AO, glow, comic touches | D7 | the draft's part 5 (what LOOK_V2 keeps) on R3-BUDGET-REC2's post row, 1131 | proposed mark (R3's row; the draft names its contents) |
| 126 | A time with no statistic is a p99 | D7 | GLOSSARY §1 time statistics, 141–166 | settled (GLOSSARY, approved terms) |
| 127 | 4 × 48 = 192 per pass | D7 | r3 R3-BUDGET-REC2, 1142–1144 | arithmetic (4 × 48 = 192) |
| 128 | Shadow + main = 384 | D7 | same | arithmetic (192 × 2 = 384) |
| 129 | Merged world about 202 | D7 | r3 1144 (CL9) | research, source to check |
| 130 | Four very large units up to 200; sum about 786, inside 800 | D7 | r3 1144 | arithmetic (384 + 202 + 200 = 786) |
| 131 | Hull adds 192 + about 80 = about 1,060 | D7 | r3 1145–1147 | arithmetic (786 + 192 + 80 = 1,058, "about 1,060") |
| 132 | The N = 54 sum (about 834) no longer applies (supersedes r3:1144–1145) | D7, A | MODES-3 (a); r3 1144–1145 | settled |
| 133 | Triangle and memory rows are the loosest: no source | D7 | r3 R3-BUDGET-F7, 575–577; R3-BUDGET-OQ5, 1373–1374 | research, source to check |
| 134 | Four very large units add ≤ 2.0 ms (p99) GPU and ≤ 200 draws, recommended, High, before ink or pre-pass | D7, S | r3 R3-CHAR-REC4, 1088–1092 | proposed mark (R3's) |
| 135 | 4 units × 20 parts × 2 passes = 160 draws | D7 | r3 1089–1090 | arithmetic (4 × 20 × 2 = 160) |
| 136 | Tier table contents; Low default for integrated adapter or WebGL 2; B-3 a measured option | D7 | r3 R3-BUDGET-REC3, 1166–1177 | proposed mark (R3's) |
| 137 | Tier table's file format is ADR 0005's content data | D7 | decision map a (ADR 0005 item 1: "quality tiers … as data") | settled (the map's assignment) |
| 138 | Harness: CPU and GPU time, info counters, median/p95/p99/max, 3 × 600 after 300, labels | D8 | r3 R3-MEASURE-REC1, 1181–1193 | proposed mark (R3's) |
| 139 | Benchmark scene: recorded match replayed from inputs, scripted camera, URL flag, previews | D8 | r3 R3-MEASURE-REC2, 1195–1204 | proposed mark (R3's) |
| 140 | Identical results rest on R1-DET-REC1 and REC6 | D8 | r1 R1-DET-REC1, 2129–2145; R1-DET-REC6, 2222–2241 | research verified |
| 141 | Fallback: a scripted bot match with a fixed seed, less exact | D8, A | r3 Pointers Q6, 1504–1505 | research, source to check |
| 142 | CI gates only hardware-independent numbers; no headless frame rate as a hardware result | D8 | r3 R3-MEASURE-REC3, 1206–1215 | proposed mark (R3's) |
| 143 | Overlay ported with its contents; Chrome's developer flag for developers only | D8 | r3 R3-MEASURE-REC4, 1217–1225; INV GP5 line 315 | proposed mark (R3's); research, source to check (GP5) |
| 144 | Field data per match: backend, adapter info, tier, ratio, resolution, frame-time median, p95, frames over budget | D8 | r3 Pointers Q2, 1477–1478; R3-GL2-OQ2, 1308–1311; R3-BUDGET-OQ1, 1364–1365; R3-MEASURE-REC1 (statistics) | research, source to check |
| 145 | ADR 0006 stores it under R6-OBS-REC4's data rules | D8 | r6 R6-OBS-REC4, 769–780; decision map I-18 | research, source to check |
| 146 | Listener at the player, oriented with the camera; PannerNode equalpower; distance model per weapon; HRTF measured for a few | D9 | r3 R3-AUDIO-REC1, 1229–1243 | proposed mark (R3's) |
| 147 | Voice cap 32 Low, 48 High (proposed start); priorities; 55 ms throttle; pooled nodes; pre-built buffers | D9 | r3 R3-AUDIO-REC2, 1245–1256 | proposed mark (R3's) |
| 148 | Occlusion O-C or none; O-B only if players cannot tell | D9 | r3 R3-AUDIO-REC3, 1258–1269 | proposed mark (R3's) |
| 149 | Legacy keeps a client copy of static geometry for explosions (GW15) | D9 | r3 legacy facts Q7, 769–770 | research, source to check |
| 150 | The page makes zero calls into Wasm | D9, A | r1 R1-BND-REC1 mark 2, 2261–2262 | research verified |
| 151 | Sounds from the event section and interpolated positions; AudioContext resumes on first input | D9 | r3 R3-AUDIO-REC4, 1271–1280 | proposed mark (R3's) |
| 152 | setAnimationLoop is the render loop | D10 | r3 R3-WGPU-F9, 115–119 | research verified |
| 153 | The page keeps the latest two blocks | D10 | r1 R1-BND-REC2, 2265–2285 | research verified |
| 154 | The main thread interpolates between the two newest ticks; hit-stop presentation only | D10 | r1 R1-TICK-REC5, 2094–2116 | research verified |
| 155 | Remote entities' buffer location is 0001's (R3-XT-OQ1) | D10 | r3 R3-XT-OQ1, 1424–1430; decision map b (owner 0001) | settled (the map's assignment) |
| 156 | HUD in the DOM from legacy PG and GP rows; textContent for dynamic text | D10 | CLAUDE.md code rule (textContent); decision map h, G16 | settled (project rule; the map's gap) |
| 157 | Cursor ray hits the drawn state; command field 0001, ray query 0004 | D10 | decision map I-19 | settled (the map's assignment) |
| 158 | OffscreenCanvas not researched | D10, A | grep of r3 for OffscreenCanvas: only R3-XT-OQ7 (1457); r1 1739 ("not assessed here") | research, source to check |
| 159 | "it reads typed arrays either way" | D10, A | r3 Pointers Q4, 1489–1491 | research, source to check |
| 160 | Nothing allocates per frame (gated) | D10 | r3 R3-CHAR-REC3 mark, 1074–1075; R3-MEASURE-REC3, 1206–1208 | proposed mark (R3's) |
| 161 | The look proof test needs a few plain glTF/GLB authored models; decides nothing on compression or LODs | D11 | ASSET-RESEARCH (a) (advisor-verdicts.md §1, condition 4); decision map J4 | settled (the advisor's ruling) |
| 162 | LOOK_V2 §3's proportions and weapons cannot be judged on legacy's primitive parts | D11 | INV CL10 line 119 (primitive part recipes); LOOK_V2 §3 | research, source to check |
| 163 | The only glTF evidence: three.js's example loads a glTF skinned mesh into WebGPU | D11, T | r3 R3-CHAR-F8, 476–480; advisor-verdicts.md §1 condition 4 (one glTF mention) | research, source to check |

## Trace table, Options, Trade-off analysis

| # | the claim, as written (short) | section | citation (file, id, lines) | tier |
|---|---|---|---|---|
| 164 | Each LOOK_V2 item and its tag, as quoted in the table's first column | T | LOOK_V2 §1–§8 | design doc |
| 165 | Models are not researched (asset pipeline) | T | decision map h, G1; J4 | research, source to check |
| 166 | Vegetation not researched (0 hits for foliage) | T, OQ9 | decision map h, G9 | research, source to check |
| 167 | BatchedMesh against InstancedMesh is R3-CHAR-OQ4 | T, OQ9 | r3 R3-CHAR-OQ4, 1352–1354 | research, source to check |
| 168 | Uniform details: faction variants are ADR 0005's (R1-DATA-OQ6) | T | decision map e2 (§2 uniform details row) | settled (the map's assignment) |
| 169 | Legacy CC9 (gun kit sockets) ports | T | INV CC9, line 138 | research, source to check |
| 170 | Building assets, thatched roof, weapons: content only (ADR 0005) | T | decision map e2 (§2 architecture, §3 rows) | settled (the map's assignment) |
| 171 | The per-row "Owner if not 0003" column | T | decision map e2, seams S5, S9 | settled (the map's assignment) |
| 172 | The tree style stays [OPEN]; the test's trees are labelled placeholders | T, OQ9 | LOOK_V2 §8 [OPEN]; plan rule 6 | design doc |
| 173 | Options and their cells, per question, as the research's Options tables give them (Q1 A, B, C; V1, V2; F-A to F-D; toon two variants; outline four; AO three; bloom two; C-A to C-G, L-1, L-2, V-A, V-B; M-A to M-D, B-1 to B-3; T-A to T-C, W-A to W-D; P-A to P-D, V-A to V-C, O-A to O-C) | O | r3 Options, 778–894 | research, source to check |
| 174 | Option C (WebGLRenderer only) is settled by the stack | O | V2_PHASE0_BRIEF §0 line 8; brief §1 | settled |
| 175 | WebGPURenderer is "experimental" | O, TA | r3 R3-WGPU-F4, 85–89 | research verified |
| 176 | "a better performance with WebGLRenderer" possible | O, TA | r3 R3-WGPU-F7, 104–107 | research verified |
| 177 | WebGLRenderer "the recommended choice for pure WebGL 2 applications"; no big new features | O | r3 R3-WGPU-F8, 109–113 | research verified |
| 178 | Serves all four browsers (Chrome 113/144, Firefox 141/145/147, Safari 26) | O | r3 R3-WGPU-F17–F19, 159–175 | research verified (F17, F18); research, source to check (F19) |
| 179 | three.js advises upgrading "whenever possible" | O | r3 R3-WGPU-F4, 85–89 | research verified |
| 180 | Pinned version is a lockfile entry; upgrade check a CI job | O | r3 Pointers Q1, 1470–1471 | research, source to check |
| 181 | Legacy uses nothing WebGL 2 lacks (CD15) | O | r3 legacy facts Q2, 300–302 | research, source to check |
| 182 | Compatibility mode is opt-in, platforms still being added | O | r3 R3-GL2-F13, 285–291 | research verified |
| 183 | Compatibility mode in three.js unknown | O | r3 R3-GL2-OQ1, 1305–1307 | research, source to check |
| 184 | "WebGPU required" not researched | O, OQ11 | r3 R3-GL2-OQ5, 1317–1320 | research, source to check |
| 185 | F-C punishes some recommended-class machines | O | r3 Options Q2, 798 | research, source to check |
| 186 | Legacy ramps carry over as single-channel textures (CL2) | O | r3 legacy facts Q3, 418–419 | research, source to check |
| 187 | Outline-normal algorithm (CL7) ports | O | INV CL7, line 116 | research, source to check |
| 188 | GTAO documented passes 800 and 1,100 at Low | O | r3 R3-BUDGET-REC2, 1148–1151 | proposed mark (R3's arithmetic) |
| 189 | How WebGPURenderer skins is unconfirmed | O | r3 R3-CHAR-OQ1, 1344–1346 | research, source to check |
| 190 | Juggernaut multi-part build (GC16) is the precedent for V-A | O | INV GC16, line 255; r3 legacy facts Q4, 522 | research, source to check |
| 191 | Chrome quantizes timestamps to 100 µs | O | r3 R3-MEASURE-F2, 610–615 | research verified |
| 192 | Timers coarsened on purpose | O | r3 R3-MEASURE-F1, 604–608 | research verified |
| 193 | WebGL 2 timer queries Chrome and Edge only | O | r3 R3-MEASURE-F6, 636–639 | research, source to check |
| 194 | performance.now() 100 µs resolution, fine for frames | O | r3 R3-MEASURE-F7, 641–646 | research verified |
| 195 | GPU runners paid per minute | O | r3 R3-MEASURE-F9, 655–658 | research, source to check |
| 196 | Self-hosted runners are ADR 0006's (R3-MEASURE-OQ2) | O | decision map b (owner 0006) | settled (the map's assignment) |
| 197 | StereoPannerNode lowest CPU | O | r3 R3-AUDIO-F7, 730–734 | research, source to check |
| 198 | Equal power "simple and efficient" | O, TA | r3 R3-AUDIO-F2, 696–700 | research verified |
| 199 | HRTF cost unmeasured; no voice cost in any source | O, TA | r3 R3-AUDIO-F9, 743–747 | research, source to check |
| 200 | three.js PositionalAudio defaults to HRTF | O | r3 R3-AUDIO-F5, 717–722 | research verified |
| 201 | A playing voice costs its nodes whatever its gain (inferred) | TA | r3 R3-AUDIO-F4, 711–715 | research, source to check |
| 202 | Trade-off reasons and "strongest argument against" lines (one per decision) | TA | rows 20–32, 41–160 (each restates a cited row) | as the rows they restate |

## Consequences, inventory, open questions

| # | the claim, as written (short) | section | citation (file, id, lines) | tier |
|---|---|---|---|---|
| 203 | Budget rows as data in the repository (I-14) | CQ | decision map I-14; R5-GATE-REC4 (owner 0002) | settled (the map's assignment) |
| 204 | 4 draws per soldier per pass against about 120 | CQ | r3 R3-CHAR-REC1 mark, 1044–1045; legacy CL6 518 | proposed mark (4); research, source to check (120) |
| 205 | Revisit at a 5 % pass-cost change on upgrade | CQ | r3 R3-WGPU-REC2 mark, 925–926 | proposed mark (R3's) |
| 206 | Nothing in CLAUDE.md or .claude/ changes in this PR | CQ | plan revision 3, rule 11 | settled (plan) |
| 207 | Legacy constraints quoted (r128 UMD, no addons, no post-processing; LinearEncoding; no external assets; stylizeScene; animation cosmetic) | CQ | CLAUDE.md "Hard constraints" | settled (project rules) |
| 208 | Under SPIKE-SCOPE (a) legacy constraints apply to legacy/ only once the drafts are approved | CQ | STATUS:187–188 | settled |
| 209 | "No external assets" ends with the legacy engine | CQ | LOOK_V2 §8 | design doc |
| 210 | "animation never moves a launch point or a hitbox" | CQ | r1 R1-DET-REC4, 2193–2194 | research verified |
| 211 | Every "mark now" in the inventory table, with its line | INV | INV lines 86, 90, 96–98, 100, 110–123, 131–141, 191, 192, 205, 217–231, 245, 247, 255, 266, 273, 311, 314–315, 317, 319, 320, 322, 360–378 | research, source to check |
| 212 | 118 rows carry "look brief pending" (the inventory's tally) | INV (intro, via R7-MARKS) | INV line 22 (header tally) | research, source to check |
| 213 | Building-kit, texture and asset rows are ADR 0005's | INV | decision map f3, LE-H | settled (the map's assignment) |
| 214 | Each proposed inventory mark and its reason | INV | the draft's parts named in each row; LOOK_V2 § named | design doc (LOOK_V2) and the draft's proposals |
| 215 | CL5's drop, CL11's Rust mark, GP6's port are unchanged | INV (not listed) | INV lines 114, 120, 316 | research, source to check |
| 216 | OQ-0003-1 to -15: each "why it matters" line restates a row above (machines: 119–121; ink: 65–74; colour: 84–86; shadows: 87–89; lighting: 62, 63, 90; roof fade: 37, 93–98; visual effects: 101–103; line boil: 105; vegetation: 166, 167, 172; frame cap: row 217; fallback: 26, 184; ratio: row 218; listener: row 219; Web Audio spec: 199; legacy touches: row 220) | OQ | as those rows | as those rows |
| 217 | The render clock follows the display (60, 75, 120, 144 Hz); a cap saves power on laptops | OQ10 | r3 R3-BUDGET-F2, 545–549; R3-BUDGET-OQ2, 1366–1367 | research, source to check |
| 218 | Ratio 2 shades four times the pixels | OQ12 | r3 R3-BUDGET-F3, 551–555 (2² = 4) | arithmetic |
| 219 | A listener on a high camera hears all sources at nearly the same distance | OQ13 | r3 R3-AUDIO-F11, 753–757 | research, source to check |
| 220 | Comic words (CL12, GW3), kill vignette (GP9), toy-soldier death fall (GA14), skin rim (CL3) are on screen in legacy and not named in LOOK_V2 | OQ15 | INV lines 121, 191, 319, 230, 112; LOOK_V2 (no such item) | research, source to check |
| 221 | LOOK_V2 §3: nothing may look like a children's game | OQ15 | LOOK_V2 §3 | design doc |
| 222 | Decider "the verifier (plan step 7, VERIFY-FETCH (a))" | OQ14 | plan revision 3, step 7; STATUS:150–151 | settled |

## Spikes, migration, test plan, rollback, appendix

| # | the claim, as written (short) | section | citation (file, id, lines) | tier |
|---|---|---|---|---|
| 223 | "Every pass mark and every reference machine is proposed" | S | r3 header, 46 | research, source to check |
| 224 | Brave's backend on Mustafa's machine is unknown | S | r3 R3-WGPU-OQ1, 1286–1289 | research, source to check |
| 225 | Per technique: GPU ms median and p95 over 600 frames at 1080p, both machines; extra draws; extra memory | S | r3 R3-LOOK-REC1 mark, 998–1000 | proposed mark (R3's) |
| 226 | Both ink families: GPU and CPU frame time at 12, 24, 48 soldiers, minimum laptop, Low | S | r3 R3-LOOK-REC3 mark, 1026–1027 | proposed mark (R3's) |
| 227 | Draw calls within 1.1 × (effects off + planned extras); each effect's settings cut its GPU time ≥ 40 % | S | r3 R3-LOOK-REC2 mark, 1014–1017 | proposed mark (R3's) |
| 228 | Both backends in Chrome, Edge, Firefox (Windows), Safari 26; zero console errors; ≤ 1 % of pixels > 8/255; WebGPU CPU frame time ≤ 1.1 × WebGL 2's | S | r3 R3-WGPU-REC1 marks, 910–915 | proposed mark (R3's; tolerance marked proposed there) |
| 229 | The fallback shows the same features and meets the minimum budget on the minimum machine | S | r3 R3-GL2-REC1 mark, 965–967 | proposed mark (R3's) |
| 230 | Programs equal at 5 s and at the end of a 5-minute match; no frame above 1.5 × median at first appearance | S | r3 R3-WGPU-REC3 marks, 937–939 | proposed mark (R3's; damage stage and roof fade added as appearances to check, from part 3) |
| 231 | Backend named correctly in 10 of 10 starts | S | r3 R3-WGPU-REC4 mark, 950–951 | proposed mark (R3's) |
| 232 | GPU time on Chrome WebGL 2; "not available", never 0, on Firefox and Safari | S | r3 R3-GL2-REC3 mark, 984–985 | proposed mark (R3's) |
| 233 | Every budget row, 3 × 600 after 300, Chrome both machines, Firefox minimum, WebGPU and WebGL 2 on minimum | S | r3 R3-BUDGET-REC2 mark, 1162–1164 | proposed mark (R3's) |
| 234 | ≤ 4 draws per soldier per pass; ≤ 2.0 ms full rig; ≤ 1.0 ms with budget LOD; ≥ 20 % for instancing | S | r3 R3-CHAR-REC1 mark, 1044–1048 | proposed mark (R3's) |
| 235 | No visible soldier culled: 600 frames, culling off and on, identical pixels | S | r3 R3-CHAR-REC2 mark, 1061–1063 | proposed mark (R3's) |
| 236 | Zero bytes per frame over 60 s; health bars for 48 in ≤ 2 draws | S | r3 R3-CHAR-REC3 mark, 1074–1075 | proposed mark (R3's) |
| 237 | Firefox WebGL 2: draws within Low budget, median within 33.3 ms on the minimum machine's Linux run | S | r3 R3-GL2-REC2 mark, 975–977 | proposed mark (R3's) |
| 238 | High → Low cuts GPU frame time ≥ 50 %, no compile during the switch | S | r3 R3-BUDGET-REC3 mark, 1176–1177 | proposed mark (R3's) |
| 239 | GPU total within 2 ms of CPU minus CPU work (median, GPU-bound); back-to-back medians within 3 % | S | r3 R3-MEASURE-REC1 mark, 1191–1193 | proposed mark (R3's) |
| 240 | Identical drawCalls and triangles every frame; identical screenshots at three frames | S | r3 R3-MEASURE-REC2 mark, 1203–1204 | proposed mark (R3's) |
| 241 | CI fails on +10 % draws or +10 % memory (1 of 1), under 5 minutes | S | r3 R3-MEASURE-REC3 mark, 1214–1215 | proposed mark (R3's) |
| 242 | Overlay within 1 % closed, 3 % open | S | r3 R3-MEASURE-REC4 mark, 1224–1225 | proposed mark (R3's) |
| 243 | Page's cost of the sim: p99 ≤ 1.0 ms per frame (budget row) | S | r3 R3-BUDGET-REC2, 1127 | proposed mark (R3's) |
| 244 | Boundary spike's p95 frame-time difference ≤ 1.0 ms at N = 12, 24, 36, 48 | S | r1 R1-BND-REC1 mark 1, 2259–2261 | research verified |
| 245 | Both statistics reported; test-engineer picks which gates (LE-B) | S | decision map f3, LE-B | settled (the map's assignment) |
| 246 | 32 voices, 5-minute capture no dropout, < 0.5 ms p99 audio per frame; HRTF 4 voices +< 0.3 ms | S | r3 R3-AUDIO-REC1 mark, 1241–1243 | proposed mark (R3's) |
| 247 | Live nodes ≤ cap; shots at the player never dropped (0 in 3 runs); no AudioBuffer after warm-up | S | r3 R3-AUDIO-REC2 mark, 1254–1256 | proposed mark (R3's) |
| 248 | If O-B: ≤ 1 ray per voice per 100 ms, ≤ 0.2 ms p99 for all rays; 4 of 5 testers | S | r3 R3-AUDIO-REC3 mark, 1267–1269 | proposed mark (R3's) |
| 249 | Onset within one rendered frame plus output latency over 100 shots; zero autoplay warnings | S | r3 R3-AUDIO-REC4 mark, 1279–1280 | proposed mark (R3's) |
| 250 | Machines named with GPU, CPU, RAM, screen, ratio; Linux run names WebGL 2 in Firefox | S | r3 R3-BUDGET-REC1 mark, 1114–1116 | proposed mark (R3's) |
| 251 | Upgrade step changes p95 GPU and CPU frame time ≤ 5 % without an explained cause | S | r3 R3-WGPU-REC2 mark, 925–926 | proposed mark (R3's) |
| 252 | The bench needs the boundary spike's block, a recorded match, the stress scene's counts | S | decision map g (render budget bench row) | settled (the map's assignment) |
| 253 | The stress scene: N soldiers, 4 very large units, 300 live projectiles | D7, S | GLOSSARY §3 the stress scene, 397–437 | settled (GLOSSARY, approved terms) |
| 254 | Netcode proof test runs first, before the foundation | M | SLICE-ORDER condition 3 (STATUS:101) | settled |
| 255 | Legacy stays playable in legacy/ (ADR 0002) | M | decision map a (ADR 0002 item 1) | settled (the map's assignment) |
| 256 | The foundation ends when an empty scene runs through the full stack, CI green, preview link | M | GLOSSARY §8 the foundation (decision map J1) | settled |
| 257 | Slice order 1–6 as listed | M | SLICE-ORDER (brief §1; STATUS:94–96 for 1–4) | settled |
| 258 | ADR 0001's pop-in mark: 0 rendered frames with an entity in view whose state is missing | M | r2 R2-INT-REC1 (a), 2437–2439 | research verified |
| 259 | Asset spike before the first authored model enters the game | M | decision map g (asset spike row); ASSET-RESEARCH (a) | settled |
| 260 | P2 = 100 ms round trip, ±20 ms jitter, 2 % loss | TP | r2 R2-XU-OQ12, 3220–3225 | research, source to check |
| 261 | JS heap ≤ 150 MB after 10 minutes, no growth over last 5 | TP | r3 R3-BUDGET-REC2, 1137 | proposed mark (R3's) |
| 262 | Rollback options (forceWebGL; hull; pre-pass High only; budget LOD then instanced; previous pin; equal power; tier change) | R | rows 41–151 (the options already cited) | as those rows |
| 263 | Appendix A dispositions, 71 rows, counts adopted 23, changed 2, answered 15, open 12, spike 19 | A | engine-architect-decision-map-ids.tsv (owner 0003: 71 rows); the dispositions restate D and OQ rows | arithmetic (23 + 2 + 15 + 12 + 19 = 71) |
| 264 | R3-LOOK-REC1's old words "before the look brief closes" (superseded by LOOK_V2) | A | r3 991 | research, source to check |
| 265 | MODES-3 (a) replaces "48 or 54" at r3:1035, 1044, 1061, 1075 | A | r3 those lines; STATUS:104–105 | settled |

## Counts
- Rows: 265 (counted by script on this file; a row with two tiers is counted under its first).
- By tier: research verified 44 · research, source to check 90 · arithmetic 10 · settled 44 · design doc 13 ·
  proposed mark 61 · rows that only restate other rows 3 (202, 216, 262).
- Pass-mark rows from the test-engineer join this ledger in plan step 4.
