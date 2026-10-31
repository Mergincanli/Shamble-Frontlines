# ADR-0003: Rendering and presentation

**Status:** Proposed
**Date:** 2026-10-31
**Author:** graphics-engineer · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no

This record decides how SHAMBLE FRONTLINES v2 draws and plays its game in the browser: one three.js WebGPURenderer
with every material and screen effect written once in TSL, which falls back to WebGL 2 by itself; how each item of the
look brief (LOOK_V2) is achieved on screen; how 48 soldiers and a few [very large units](../research/GLOSSARY.md#big-vehicle-and-very-large-unit) fit the frame on a laptop with
integrated graphics; how frame time is measured on real machines; and how sound is placed and capped. It proposes one
technique where LOOK_V2 leaves a technical choice, names every visible difference as Mustafa's call in the look proof
test, and decides nothing LOOK_V2 leaves open (the tree style). Every number is from the research, arithmetic on it,
a settled input, or a mark marked proposed.

How to read the citations: each gives the research file, its section, the id and the lines at commit 9e7db28 (the
topic files do not change in this PR). "Findings Q1" means the Findings section, question 1, of that file; legacy ids
(CD15, CL6, GA15 …) are rows of `docs/LEGACY_INVENTORY.md`. Terms are the
[glossary's](../research/GLOSSARY.md); each is linked at its first use.

## Context

### What the engine must do (the requirements)
- **VISION_V2 §4:** desktop Chrome, Edge, Firefox and Safari first; the minimum is "a laptop with integrated graphics
  runs the largest mode at 30+ fps on Low"; the recommended is "a mid-range graphics card runs it at 60 fps on High"
  ([VISION_V2.md](../VISION_V2.md) §4).
- **VISION_V2 §6.9, the spectacle budget:** "the renderer handles a few very large units and their effects without
  dropping frames". Also served here: §6.2 (aiming at height needs a cursor ray against what is drawn; interiors and
  floors need the roof fade), §6.4 (big, multi-part vehicles drawn from their parts), §6.7 (faction uniforms and
  variants: colour per faction on shared meshes). §6.8's kill-cams, replays and highlights are `[LATER]`: nothing here
  builds them; the recorded matches the benchmark scene replays keep them possible.
- **LOOP_V2 §7:** a per-weapon feel kit as data ("kick, muzzle flash, tracer, impact, hit marker, sound, target flinch,
  kill confirm"), split feedback ("the shot feels instant; hit feedback arrives when the hit is confirmed"), targets
  that flinch ([LOOP_V2.md](../LOOP_V2.md) §7). **LOOP_V2 §11:** the HUD and scoreboard for three teams.
- **LOOK_V2, the whole brief** ([LOOK_V2.md](../LOOK_V2.md)): §9 asks this record to show how each `[DECIDED]` item
  of §1 to §8 is achieved, with a look proof test "judged by Mustafa in the browser, side by side with screenshots of
  the legacy game". The `[DEFAULT]` items (including the Advisor's additions in §6) are built as defaults. The tree
  style (§8) is `[OPEN]` and stays open.
- **EDITOR_V2 §2, rule 3:** "Shared materials and textures are cached and reused, never rebuilt per edit"
  ([EDITOR_V2.md](../EDITOR_V2.md) §2); this record cites only that shared material cache, not the editor's design.

### Settled inputs (not reopened)
- **The stack:** a TypeScript client on the latest three.js with the WebGPU renderer and a WebGL 2 fallback
  (`docs/V2_PHASE0_BRIEF.md` §0, line 8; brief §1). A renderer other than three.js WebGPU is settled by it.
- **[N](../research/GLOSSARY.md#player-count-n) = 48:** [the largest mode](../research/GLOSSARY.md#the-largest-mode-and-its-n) is 24v24 (MODES-3 (a), Mustafa, 2026-10-31; brief §1). Where an R3 mark gives values for
  48 and 54, the 48 value holds (carry-forward 3, STATUS:133–134).
- **SLICE-ORDER** and its conditions (STATUS:94–101; brief §1): from slice 1 the simulation runs in a Web Worker behind
  the network's own messages, so the page only reads what the Worker publishes. The netcode proof test runs before the
  foundation ([GLOSSARY §8: the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test);
  [the foundation](../research/GLOSSARY.md#the-foundation)).
- **SEC-SPREAD (a)** (STATUS:108–109): weapon spread is known only to the server; "tracers use a similar-looking spread
  that the hit corrects", so this draft draws tracers from a cosmetic spread.
- **TICK-30** (STATUS:118–120): a 30 Hz server tick is the baseline. The page interpolates between ticks whatever the
  rate (R1-TICK-REC5, below); [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share) at 30 Hz is ADR 0004's number.
- **LOOK_V2** (Mustafa and the Advisor, 2026-10-31; brief §1): the look is decided where LOOK_V2 says `[DECIDED]`, and
  authored 3D models (glTF/GLB) are allowed in v2 (LOOK_V2 §8).
- **UPGRADE-SCOPE** (Mustafa, 2026-10-31; `inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md`, recorded in STATUS by
  docs-keeper): the upgrade tracks live on the player; the page only shows the owner's offers and tracks (I-21 of the
  decision map; ADR 0005 designs them).
- **SPIKE-SCOPE (a)** (STATUS:187–188): spikes are throwaway in `spikes/`, exempt from the legacy constraints.
- **R7-MARKS (a)** (STATUS:159–160): the inventory's marks are working recommendations; this draft lists the rows whose
  mark it would change (Consequences), and the inventory file changes only after Mustafa accepts the drafts.

### The findings the decision turns on
1. One TSL source builds both WGSL and GLSL: "capable of encoding code into different outputs such as `WGSL`/`GLSL`"
   (r3-rendering.md, Findings Q1, R3-WGPU-F11, lines 127–131). Custom GLSL, `onBeforeCompile` and `EffectComposer` are
   "not supported in WebGPURenderer" (R3-WGPU-F6, lines 97–102), so legacy's skin rim (CL3) and hull shader (CL6) do
   not carry over.
2. The fallback emulates what WebGPU has natively: compute runs as transform feedback with no indirect dispatch
   (R3-GL2-F2, lines 219–223), storage buffers are packed into textures with no atomics (R3-GL2-F3, lines 225–232), and
   render bundles bring no gain (R3-GL2-F9, lines 262–265). Whole platform groups land there: Firefox on Linux and on
   Intel Macs, Chrome on Linux without an Intel Gen12+ GPU, Safari before 26 (R3-GL2-F12, lines 279–283).
3. The two outline families scale with different things: the inverted hull adds one draw per outlined object; a
   screen-space filter costs one full-screen pass whatever the object count (r3-rendering.md, Findings Q3, R3-LOOK-F8,
   lines 366–370). No source read gives GPU milliseconds for any look technique on any hardware (R3-LOOK-F15, lines
   411–413).
4. Per-instance compute skinning is WebGPU only in three.js's own example (r3-rendering.md, Findings Q4, R3-CHAR-F9,
   lines 482–488).
5. Headless Chrome is not a hardware measurement: it reports "Software only, hardware acceleration unavailable." and,
   even with GPU flags, "Hardware accelerated but at reduced performance." (r3-rendering.md, Findings Q6,
   R3-MEASURE-F8, lines 648–653).

### What the legacy game proves
- One r128 WebGLRenderer with MSAA, pixel ratio capped at 2, `PCFSoftShadowMap` and `LinearEncoding` output ("authored
  hex = on-screen colour"): CD15 (r3-rendering.md, Findings Q1, legacy facts, lines 195–198). r186 removed
  `PCFSoftShadowMap` from WebGPURenderer: "Use `PCFShadowMap` which is now soft as well." (R3-WGPU-F3, lines 78–83).
- Draw calls are the measured problem: about 120 per on-screen soldier; 1,480 with 12 soldiers on screen and 5,843 with
  48 (CL6, TL15; r3-rendering.md, Findings Q4, legacy facts, lines 518–519). Static merging cut the village from 398 to
  202 draw calls (CL9; line 523).
- Soldier animation costs about 26 to 51 µs each, 1.2 to 2.4 ms at 48, and "Nothing culls or simplifies animation for
  distant or off-screen soldiers" (GA15; r3-rendering.md, Findings Q4, legacy facts, lines 515–517).
- The cutaway fades structures between camera and player and hides their ink (GW17; `LEGACY_INVENTORY.md` line 205);
  toggling `material.transparent` per mesh can mean a new pipeline on first use (r3-rendering.md, Findings Q1, legacy
  facts, lines 201–202).
- Audio is synthesized; "there is no voice limit and no occlusion" (GP6; r3-rendering.md, Findings Q7, legacy facts,
  lines 760–765).
- The halftone is a CSS multiply layer over the page, "not a render pass", off by default (CD16;
  `LEGACY_INVENTORY.md` line 97).

### What this record takes from and gives to the other drafts
- **Takes:** the per-tick output block, its event section and its layout (ADR 0002; decision map I-2); the command the
  page posts (ADR 0002, I-3) and the aim point in it (ADR 0001, I-19); the snapshot fields of remote soldiers,
  including their animation state, and where the remote-entity interpolation buffer lives (ADR 0001: R3-XT-OQ2,
  R3-XT-OQ1); vehicle-part records, floor layers and enclosures, structure states, and the blast and hit events
  (ADR 0004; I-2, I-9, I-20); the recorded matches the benchmark scene replays (ADR 0004, I-17); asset formats,
  material slot names, faction colour and emissive slots, LOD levels and per-unit budgets, and the map document's
  lighting mood (ADR 0005; I-8, I-9); where client field data is stored (ADR 0006, I-18).
- **Gives:** the client reference machines every client mark uses (I-15); the frame budget rows (shared numbers N41 to
  N48) that ADR 0002 keeps as data (I-14) and ADR 0005 derives per-unit budgets from; the page's cost of the sim per
  frame (N12) that ADR 0002's boundary spike measures; what client field data contains (I-18) for ADR 0006; the
  interpolated, drawn state the cursor ray hits (I-19).

## Decision

v2 draws with one three.js WebGPURenderer, every material and screen effect in TSL, and its own WebGL 2 backend as
the fallback with the same features. The look is built as one shared scene pass with extra outputs (MRT) that the ink,
the ambient occlusion and the glow read, on node toon materials the engine assigns to each asset's material slots.
Soldiers are one skinned mesh each, animated under a budget, with per-instance data written straight from the
Worker's block. Budgets and quality tiers are data, measured only on two named reference machines. Sound is placed
against a listener at the player and capped by a voice manager. Two spikes prove it: the look proof test (Mustafa
judges the look) and the render budget bench (the numbers).

1. **The renderer and its backends.** One `WebGPURenderer`; every material and post effect in TSL; no
   `ShaderMaterial`, `onBeforeCompile` or `EffectComposer` anywhere. Its built-in WebGL 2 backend is the fallback,
   with the same features on both backends (option F-A): a WebGPU-only extra (compute, storage writes, render
   bundles) enters only with a named fallback and a measured gain (F-B by measured need). No draw-call budget depends
   on `BatchedMesh` or multi-draw on the fallback; Firefox's WebGL 2 path is the worst case where draw calls are
   counted. *Why:* one TSL source builds both backends, and legacy's GLSL routes are unsupported (r3-rendering.md, Findings Q1,
   R3-WGPU-F11, lines 127–131; R3-WGPU-F6, lines 97–102).
   *What the fallback loses* (stated, not hidden): native compute, which runs as transform feedback without indirect
   dispatch (R3-GL2-F2, lines 219–223); native storage buffers, which are packed into textures without atomics
   (R3-GL2-F3, lines 225–232); the gain of render bundles (R3-GL2-F9, lines 262–265); GPU timers outside Chrome and
   Edge (R3-GL2-F5, lines 240–245); multi-draw on Firefox, so each `BatchedMesh` entry is its own draw (R3-GL2-F7,
   F8, lines 252–260); and some depth and MSAA paths that need extensions (R3-GL2-F10, lines 267–271). Under F-A a
   fallback player loses none of the look; the engine forgoes WebGPU-only extras. *Ids:* R3-WGPU-REC1
   (r3-rendering.md, Recommendations Q1, lines 902–915), R3-GL2-REC1 (Recommendations Q2, lines 955–967),
   R3-GL2-REC2 (lines 969–977). *Need:* VISION_V2 §6.9; LOOK_V2 §1 (one look on every backend).
2. **The three.js pin and upgrades.** Pin one exact release: the newest when the look proof test starts (r186 when
   researched, R3-WGPU-F1, lines 67–71). Upgrade one release at a time, with the migration guide as the checklist and
   the benchmark scene re-run before and after (version policy V1). *Why:* the API broke in r183, r185 and r186
   within a year (R3-WGPU-F3, lines 78–83). This also disposes of the old ADR-003 entry ("stay on r128 vs upgrade",
   STATUS:202): the stack settles the upgrade, and this part is its policy. *Ids:* R3-WGPU-REC2 (Recommendations Q1,
   lines 917–926); answers R3-WGPU-OQ5. *Need:* VISION_V2 §4 only.
3. **No first-sight stutter.** Behind the loading screen: `await renderer.init()`, then `compileAsync()` on a warm-up
   scene that holds every material variant a match can show: each faction's material slots, each visual effect,
   each blend state (the roof fade's included), each damage-stage mesh (LOOK_V2 §4) and each quality tier's
   variants. The match itself creates no new shader programs. *Why:* WebGPU creates pipelines asynchronously
   (R3-WGPU-F22, lines 188–192), three.js names the hitch "shader compilation stutter" (R3-WGPU-F10, lines 121–125),
   and legacy already compiles on first sight (TL15: 16 programs against 9; r3-rendering.md, Findings Q1, legacy
   facts, lines 203–205). *Ids:* R3-WGPU-REC3 (Recommendations Q1, lines 928–939). *Need:* VISION_V2 §6.9 ("without
   dropping frames").
4. **The backend is reported; GPU timing on the fallback is best effort.** The client logs which backend runs
   ("WebGPU" or "WebGL 2 fallback") with the adapter's info, shows it in the performance overlay, and every test
   screenshot prints it; the game is served from a secure context (HTTPS, or localhost in development). On the
   fallback, GPU time comes from `EXT_disjoint_timer_query_webgl2` where it exists (Chrome, Edge) and is reported as
   "GPU time not available", never 0, elsewhere; no CI gate or pass mark rests on a timer a browser lacks. *Why:* the
   renderer picks its backend itself (R3-WGPU-F5, lines 91–95), WebGPU needs a secure context (R3-WGPU-F20, lines
   177–180), and only Chrome and Edge ship the timer extension (R3-GL2-F5, lines 240–245). *Ids:* R3-WGPU-REC4 (Recommendations Q1,
   lines 941–951), R3-GL2-REC3 (Recommendations Q2, lines 979–985). *Need:* VISION_V2 §4 only.
5. **The look pipeline, against LOOK_V2.** One shared scene pass with MRT: colour plus only the outputs the kept
   effects read (depth and normals for the ink and the ambient occlusion, emissive for the glow). Effects that only
   read buffers never re-render the scene; a second geometry pass is a planned, counted extra; every effect has a
   quality-tier switch (off, or a lower resolution scale) from the start (R3-LOOK-REC2, Recommendations Q3, lines
   1002–1017). Every technique below is measured alone in the look proof test: GPU time per pass and memory per
   target, on both reference machines and both backends (R3-LOOK-REC1, lines 991–1000, now serving LOOK_V2's items).
   The table after this list traces every LOOK_V2 item to its part. *Need:* LOOK_V2 §1–§7 only, and VISION_V2 §6.9
   for the budget.
   - **a. Toon materials, applied by the engine.** At load, each material slot of an asset (slot names are ADR 0005's,
     decision map I-8) gets the shared node toon material of its material class (`MeshToonNodeMaterial`, R3-LOOK-F1,
     lines 318–321) with that class's ramp; one material per slot content, never one per mesh. *Why:* the ramp is one
     texture lookup inside lighting, no extra pass, on both backends; each distinct ramp is a distinct material, so
     materials are shared by content (R3-LOOK-F2, F3, lines 323–334; legacy CL4's lesson). The toon model reads only
     the ramp's red channel (`return vec3( gradientMap.r );`, R3-LOOK-F2), so colour in shade comes from the lights
     (5.g), not the ramp. A custom TSL lighting model (the options table's second toon variant, r3-rendering.md,
     Options Q3, line 809) is the fallback if the look proof test shows an item the node model cannot reach
     (OQ-0003-5). This is LOOK_V2 §8's step 5, "the engine applies the comic look".
   - **b. The ink: screen-space edges, proposed.** Edges from depth and normals, a custom TSL node over the shared
     pass's MRT outputs (R3-LOOK-F7, lines 360–364): depth edges carry the silhouette ink, normal edges the inner
     lines, so LOOK_V2 §6.2's two weights can map onto two edge kinds (inferred from F7; inner lines are not
     researched: OQ-0003-2). The weight per material class (buildings heaviest, LOOK_V2 §2) needs a per-pixel class
     value in an MRT output (not researched: OQ-0003-2). *Why this family, on cost and robustness:* its cost is one
     full-screen pass whatever the number of soldiers (R3-LOOK-F8, lines 366–370), it shares the normal output with
     the ambient occlusion (5.c), and it keeps the Low draw-call cap at 800 at N = 48, where the hull family needs
     about 1,100 (R3-BUDGET-REC2's arithmetic, lines 1142–1148). *The measured alternative:* the inverted hull,
     legacy's family (three.js's `toonOutlinePass`, R3-LOOK-F4, F5, lines 337–350, or our own TSL hull in world
     units with legacy's ground contact line, CL6, `LEGACY_INVENTORY.md` line 115). The look proof test measures both families at 12, 24 and 48
     soldiers on screen on the minimum laptop at Low (R3-LOOK-REC3, lines 1019–1028). *The visible difference is
     Mustafa's call:* the hull draws a line of even width around each object, with legacy's ink contact line where
     objects meet the ground (CL6); the screen-space pass draws a line in pixels from depth and normal changes, can
     mark folds inside a shape, and must be masked where the roof fade hides structures (r3-rendering.md, Options Q3,
     lines 810–813). If he picks the hull, the Low cap becomes about 1,100 (proposed) or the hull a High-only setting
     (R3-BUDGET-REC2, lines 1146–1148). Its known failures: threshold tuning and thin lines that alias (line 813).
   - **c. Ambient occlusion in corners.** GTAO (three.js's node, R3-LOOK-F9, lines 373–377) at half resolution ("a
     value of `0.5` should be sufficient for most scenes", R3-LOOK-F10, lines 379–384), reading depth and normals
     from the shared pass instead of the documented pre-pass, so the scene is not drawn twice (R3-LOOK-F11, lines
     386–389), and denoised with `DenoiseNode` rather than TAA, whose documented "ghosting and temporal
     instabilities" (R3-LOOK-F10) would show on fast-moving soldiers (inferred). Whether single-pass AO is good
     enough is R3-LOOK-OQ3 (lines 1328–1329), which the look proof test decides; the documented pre-pass stays a High-only fallback
     whose extra draws are counted (R3-LOOK-REC2). Low may switch AO off (tier data, part 7).
   - **d. Emissive glow, selective.** Bloom from an emissive MRT output, so only emissive slots glow ("use the
     `emissive` material property to control which objects should contribute to bloom", R3-LOOK-F12, lines 392–395).
     Full-image bloom is not proposed: it makes whatever is bright glow ("Washes bright UI-like colours", Options Q3,
     line 817), against LOOK_V2 §6.4's list and §6.1's value hierarchy. *Cost:* twelve full-screen passes at falling
     resolution (R3-LOOK-F13, lines 397–402) plus one emissive attachment, 16.6 MB at 1920 × 1080 in half float
     (R3-LOOK-F14, lines 405–409).
   - **e. Colour output: shown both ways, not proposed.** The render pipeline tone-maps and converts colour space by
     default, and the switch `outputColorTransform` reaches either output (R3-WGPU-F12, lines 133–137); legacy's rule
     is "authored hex = on-screen colour" with no tone mapping (CD15). The research has no options section on the
     output transform, so this record proposes none: the look proof test shows both outputs beside legacy, and
     Mustafa picks (OQ-0003-3; R3-WGPU-OQ4, lines 1296–1298). The internal buffer stays half float, the renderer's
     default (R3-WGPU-F14, lines 144–148).
   - **f. Soft shadows.** One sun shadow map, as R3-BUDGET-REC2's shadow row budgets it (Low: one map, 1024² or off,
     ≤ 3 ms p99; High: ≤ 2 ms; line 1129), using `PCFShadowMap`, which r186 made soft (R3-WGPU-F3, lines 78–83), with legacy's
     texel-snapped frustum that follows the player (GW4; `LEGACY_INVENTORY.md` line 192). Cascades and other setups
     were not researched (R3-BUDGET-OQ6, lines 1375–1378): OQ-0003-4.
   - **g. Lighting and moods.** The sun, fill and sky light colours and intensities, the fog and the mood come from
     the map document (ADR 0005, I-9) and are applied at load, so a map can be at dusk (LOOK_V2 §6.5). Whether the
     toon material with coloured lights reaches "warm sun with cool shadows", or a custom TSL lighting model is needed,
     was not researched: OQ-0003-5, decided in the look proof test.
   - **h. The value hierarchy.** Each material class (environment, units, faction colours, visual effects) carries a
     colour gain held as tuning data, so the environment can sit one step calmer (LOOK_V2 §6.1); legacy's nature gain
     on environment materials (CL4, 1.56; `LEGACY_INVENTORY.md` line 113) is the precedent. The values are tuning in
     the look proof test, not decisions; nothing beyond the legacy precedent was researched.
   - **i. The roof fade.** Port legacy's cutaway (GW17) as a fade of any structure carrying a fade flag (the
     inventory's note, line 205): over the enclosure the local player is in (ADR 0004's floor layers and area ids,
     r4-physics-navigation.md, Recommendations, R4-FLOORS-REC2, lines 925–939) and between the camera and the player. Its blend state is
     a precompiled variant (part 3), and the ink is masked on faded structures. It follows the server's visibility
     rule: "The client's cutaway (GW17) follows the same rule, so what is not sent is also what the screen would not
     show" (r2-netcode.md, Recommendations, R2-INT-REC2, lines 2445–2466, quote at 2450–2451); what a player may see
     is R2-INT-OQ1 (r2-netcode.md, Open questions, lines 3005–3009), a design question for Mustafa. LOOK_V2 §4's own case, the roof of
     the building the player is in, is inside that rule, since the viewer's own area is connected to itself. The fade's
     technique (a blend-state variant, or a fade computed in the shader) was not researched: OQ-0003-6. *Answers*
     R3-XT-OQ8 and R2-XT-OQ15. *Need:* VISION_V2 §6.2; LOOK_V2 §4.
   - **j. Damage stages and ruins.** A mesh per stage (ADR 0005's assets), swapped when the block's structure section
     reports a new state (ADR 0004's states: R4-OBST-REC2 bakes two, r4-physics-navigation.md, Recommendations, lines 1043–1061; a
     third stage is ADR 0004's and ADR 0005's), every stage mesh compiled in the warm-up (part 3). Debris and dust are
     visual effects (5.k). Planks and debris as routes are ADR 0004's typed links (r4-physics-navigation.md, Recommendations, R4-FLOORS-REC1, lines 903–924); this
     record only draws them. *Need:* VISION_V2 §6.2; LOOK_V2 §4.
   - **k. Visual effects.** Fragments, muzzle flashes, tracers (from the cosmetic spread, SEC-SPREAD (a)), impacts and
     debris are pooled per kind, with caps as tier data (legacy GP1's pools, `LEGACY_INVENTORY.md` line 311;
     R3-BUDGET-REC3, lines 1166–1177), spawned from the block's events and the feel kit's data (ADR 0005; decision map
     I-20), on CPU or vertex work on both backends (F-A); GPU particles come only as an F-B extra with a measured gain.
     They fit R3-BUDGET-REC2's visual-effects row (≤ 2 ms p99 at Low, ≤ 1 ms at High; line 1132). The particle
     technique was not researched: OQ-0003-7. Whether a fragment deals damage is a design question for Mustafa; this
     part only draws fragments. *Need:* LOOK_V2 §3, §7; LOOP_V2 §7.
   - **l. Comic impact touches, sparingly.** Halftone flashes as legacy's DOM overlay (CD16: a CSS multiply layer,
     "not a render pass", `LEGACY_INVENTORY.md` line 97), triggered by big-moment events; ink line boil's technique
     was not researched (OQ-0003-8). Both are built as defaults and costed in the look proof test against the
     post-processing row. *Need:* LOOK_V2 §7 only.
   - **m. The grenade concussion.** Port legacy's spring camera kick and concussion (CD9, GP4;
     `LEGACY_INVENTORY.md` lines 90, 314) as a camera effect on the page, driven by the blast event's position and
     strength (ADR 0004 emits it, I-20): it tilts away from the blast and grows stronger closer in, draws its
     randomness from the page's own generator, never the sim's ("Presentation (effects, sound, camera shake, animation
     variety) uses its own client-side generator", r1-simulation-core.md, Recommendations, R1-DET-REC4, lines 2184–2207, quote at 2192–2193), and never
     moves the aim. *Need:* LOOK_V2 §2 only.
   - **n. Anti-aliasing.** The look proof test starts from the renderer's built-in MSAA ("4 samples are used by
     default", R3-WGPU-F15, lines 150–152), which multiplies the multisampled MRT attachments' memory (R3-LOOK-F14, lines 405–409).
     Post-process anti-aliasing was not researched as an option (R3-LOOK-OQ6, lines 1335–1340); the look proof test
     decides after a reading of three.js's anti-aliasing nodes.
6. **Characters, crowds and very large units.**
   - One skinned mesh per soldier (option C-B) on both backends, with any skin weights, since v2 soldiers are
     authored models (LOOK_V2 §3, §8; the model is ADR 0005's, R3-CHAR-OQ3, r3-rendering.md lines 1349–1351). Instanced per-instance skinning (C-E on
     both backends, C-D as the WebGPU variant) is measured against it and adopted only if it cuts total frame time by
     at least 20 % on both reference machines (R3-CHAR-REC1, Recommendations Q4, lines 1032–1048).
   - A budget LOD for animation (option L-2): a fixed number of soldiers get the full procedural rig each frame, by
     importance (own squad, nearest, shooting); the rest update at a lower rate or with a cheaper rig; bounds are a
     fixed padded sphere, never recomputed per frame (R3-CHAR-REC2, lines 1050–1063). Distance LOD (L-1) and the
     asset's mesh LOD levels (ADR 0005) switch through three.js's `LOD` with its hysteresis (R3-CHAR-F10, lines
     490–493) only if the camera's framing spreads soldiers over a wide range of distances; the framing is a design
     question LOOK_V2 does not settle (r2-netcode.md, Open questions, R2-INT-OQ2, lines 3010–3013;
     r3-rendering.md, Open questions Q4, R3-CHAR-OQ5, lines 1355–1356).
   - Per-instance data (transform, faction colour, hit flash, health bar) is written from the per-tick block into
     preallocated typed arrays, never by traversing scene objects; health bars and flashes are instanced or
     per-instance attributes (R3-CHAR-REC3, lines 1065–1075). Faction colour comes from the faction colour slots
     (ADR 0005, I-8): one shared material per slot and faction, or per-instance colour where meshes are instanced
     (`instanceColor`, R3-CHAR-F4, F6, lines 452–467).
   - Very large units: parts as separate meshes under one group (option V-A), each part's transform from the sim's
     vehicle-part records; the static world merged by material content on both backends; render bundles on WebGPU
     only if they cut CPU frame time (R3-CHAR-REC4, lines 1077–1092).
   - *Ids:* R3-CHAR-REC1–REC4. *Need:* VISION_V2 §6.4, §6.7, §6.9; LOOK_V2 §2, §3, §5.
7. **Client budgets, reference machines and quality tiers.**
   - *Reference machines* (R3-BUDGET-REC1, Recommendations Q5, lines 1096–1116; the models are OQ-0003-1). Minimum:
     one named laptop with Intel Gen12 (Iris Xe) class integrated graphics or the AMD equivalent, a laptop CPU of at
     least 4 cores and 8 threads (proposed, no source), 1920 × 1080, Windows 11, run on Chrome and Firefox (both on
     WebGPU there) and with the WebGL 2 backend forced, plus a Linux run of the same laptop for the real fallback.
     Recommended: one named desktop with a mid-range discrete card one or two generations old, its CPU named, at
     1920 × 1080 and 2560 × 1440. Mustafa's RTX 5080 is a check above the recommended class and the Brave check,
     never the recommended reference. *Need:* VISION_V2 §4 only.
   - *Budgets* (option B-2), measured in [the stress scene's fight on screen](../research/GLOSSARY.md#the-stress-scene):
     the largest mode's N = 48 soldiers in one fight, its 4 very large units and its 300
     [live projectiles](../research/GLOSSARY.md#live-projectile) with their
     [visual effects](../research/GLOSSARY.md#active-effect-and-visual-effect) (R3-BUDGET-REC2, lines 1118–1164;
     every row proposed in the research). A time with no statistic named is a p99
     ([time statistics](../research/GLOSSARY.md#time-statistics)).

     | Budget (all passes of one frame) | Minimum laptop, Low, 30+ fps | Recommended card, High, 60 fps |
     |---|---|---|
     | Frame time, p95 | ≤ 33.3 ms | ≤ 16.7 ms |
     | Frame time, median | ≤ 28 ms | ≤ 14 ms |
     | Main-thread CPU per frame (render submission, interpolation, animation, UI), p99 | ≤ 16 ms | ≤ 8 ms |
     | of which: character animation (posing and bone upload, with the budget LOD), p99 | ≤ 1.0 ms | ≤ 2.0 ms |
     | of which: [the page's cost of the sim](../research/GLOSSARY.md#the-clients-sim-share), p99 | ≤ 1.0 ms | ≤ 1.0 ms |
     | GPU time, total, p99 | ≤ 25 ms | ≤ 12 ms |
     | of which: shadow pass | ≤ 3 ms (one map, 1024² or off) | ≤ 2 ms |
     | of which: main scene pass | ≤ 16 ms | ≤ 6 ms |
     | of which: post-processing (ink, AO, glow, comic touches) | ≤ 4 ms | ≤ 3 ms |
     | of which: visual effects, sprites, HUD in the 3D view | ≤ 2 ms | ≤ 1 ms |
     | Draw calls (`renderer.info.render.drawCalls`) | ≤ 800 with the screen-space ink (part 5.b); about ≤ 1,100 if a hull outline runs at Low | ≤ 1,500 |
     | Triangles | ≤ 500k | ≤ 1.5M |
     | GPU memory (`renderer.info.memory.total`) | ≤ 512 MB | ≤ 1 GB |
     | Render resolution | `devicePixelRatio` capped at 1, scene pass resolution scale 0.75–1 | ratio capped at 2 |
     | JS heap after 10 minutes | ≤ 150 MB, no growth over the last 5 minutes | same |

     The arithmetic behind the Low draw-call cap, at N = 48 (R3-BUDGET-REC2, lines 1142–1152): 4 draws per soldier
     per pass × 48 soldiers = 192 per pass; a shadow and a main pass make 384; the merged world about 202; four very
     large units up to 200; about 786, inside 800. A hull outline adds one draw per outlined object: 192 for the
     soldiers and about 80 for the very large units' parts, about 1,060. The research's N = 54 sum (about 834) no
     longer applies (MODES-3 (a); it supersedes r3-rendering.md lines 1144–1145, "at N = 54, MODES-3 (b), … about
     834, over 800"). The triangle and memory rows are the loosest: no source gives them (R3-BUDGET-F7, lines
     575–577; R3-BUDGET-OQ5, lines 1373–1374). *Need:* VISION_V2 §6.9; §4.
   - *The spectacle budget* (VISION_V2 §6.9): the stress scene's four very large units on screen with its soldiers add
     at most 2.0 ms (p99) GPU time and at most 200 draw calls against the scene without them, before any ink or AO
     pre-pass, on the recommended machine at High (4 units × 20 parts × 2 passes = 160 draws, plus margin;
     R3-CHAR-REC4, lines 1088–1092); their visual effects sit inside the visual-effects row, and their first
     appearance compiles nothing (part 3).
   - *Quality tiers as data* (R3-BUDGET-REC3, lines 1166–1177): one table holds, per tier (Low, Medium, High), the
     pixel-ratio cap, the scene resolution scale, MSAA samples, shadow map size and distance, each post effect's
     on/off and resolution scale, the animation budget and the visual-effect caps. Low is the default when the adapter
     is integrated or the backend is WebGL 2. Dynamic resolution (option B-3) is a measured option, not the start. The
     table's file format is ADR 0005's content data; its rows are this record's. *Need:* VISION_V2 §4 only.
8. **Measurement.**
   - One client harness: per frame, CPU frame time from `performance.now()`, GPU time from timestamp queries where
     the backend has them (else "not available"), and the `renderer.info` counters reset once per frame; it reports
     median, p95, p99, max and frames over budget for 3 runs of 600 frames after 300 warm-up frames, each labelled with
     backend, adapter, browser version, `devicePixelRatio`, resolution and whether GPU times are quantized
     (R3-MEASURE-REC1, Recommendations Q6, lines 1181–1193).
   - A fixed benchmark scene: a recorded match replayed from its inputs (ADR 0004's identical results: r1-simulation-core.md, Recommendations, R1-DET-REC1, lines 2129–2145, and
     R1-DET-REC6, lines 2222–2241) at the stress scene, with a scripted camera path, started by a URL flag on any build, previews included
     (R3-MEASURE-REC2, lines 1195–1204). If replay on the client is not identical, the fallback is a scripted bot
     match with a fixed seed, less exact (r3-rendering.md, Pointers Q6, lines 1504–1505).
   - CI gates only hardware-independent numbers (zero console errors, draw calls, triangles, programs, GPU memory
     totals, zero per-frame allocation in the render path); frame and GPU times come only from the reference machines,
     recorded per run in a results log; a headless frame rate is never reported as a hardware result
     (R3-MEASURE-REC3, lines 1206–1215). *Need:* VISION_V2 §4 only.
   - Legacy's performance overlay ported (GP5, `LEGACY_INVENTORY.md` line 315): backend and adapter, frame time median and p95, draw calls,
     triangles, programs, GPU memory, GPU time when available, zero cost while closed; developers profile per pass
     with Chrome's developer flag on the reference machines, never asking players to set it (R3-MEASURE-REC4, lines
     1217–1225).
   - *Client field data* (answers R3-XT-OQ4, R6-XT-OQ5 and R6-OBS-OQ5): once per match the client reports its backend,
     the adapter's info as the browser gives it, its quality tier, `devicePixelRatio` and resolution, and the
     harness's frame-time median, p95 and frames over budget. ADR 0006 decides where it is stored and for how long,
     under R6-OBS-REC4's data rules (r6-infrastructure.md, Recommendations, R6-OBS-REC4, lines 769–780). *Why:* the fallback share (R3-GL2-OQ2, lines 1308–1311) and
     the players' hardware mix (R3-BUDGET-OQ1, lines 1364–1365) wait on it ("the backend in use and the adapter info are worth one
     field in the client's session report", r3-rendering.md, Pointers Q2, lines 1477–1478). *Need:* VISION_V2 §4 only.
   - *Ids:* R3-MEASURE-REC1–REC4. *Need:* VISION_V2 §4; §6.9 for the benchmark scene and the overlay.
9. **Spatial audio.**
   - One `AudioListener` at the local player's position, oriented with the camera, while the camera is top-down and
     high (R3-AUDIO-OQ3, lines 1399–1400, re-decides it with the camera's framing); sources are `PannerNode`s with `equalpower`, the
     distance model and its numbers per weapon in the feel kit (ADR 0005); HRTF only as a measured option for a few
     priority voices (R3-AUDIO-REC1, Recommendations Q7, lines 1229–1243).
   - A voice manager: a cap of 32 voices on Low and 48 on High (proposed start), priorities as data (own weapon and
     shots at the player first, then large units, then by distance), per-kind throttles such as legacy's 55 ms impact
     throttle, pooled nodes and pre-built noise buffers; legacy's synthesized recipes port (GP6, `LEGACY_INVENTORY.md` line 316) without a new buffer
     per shot (R3-AUDIO-REC2, lines 1245–1256).
   - Occlusion starts as zones from the map data (option O-C) or none; rays (O-B) only if the spike shows players
     cannot tell sounds behind walls (R3-AUDIO-REC3, lines 1258–1269). If O-B is ever chosen, the rays run on the page
     against a client copy of the map's static geometry, as legacy keeps one for explosions (GW15; r3-rendering.md,
     Findings Q7, legacy facts, lines 769–770), with the floor and height data of the map document (ADR 0004, I-9),
     because the page makes no call into the sim (r1-simulation-core.md, Recommendations, R1-BND-REC1 mark 2, lines 2261–2262).
     *Answers* R3-XT-OQ6, R1-XT-OQ14, R4-XT-OQ7 and R3-AUDIO-OQ5.
   - Sounds start on the page from the per-tick event section and interpolated positions, never from the sim; the
     AudioContext starts suspended and resumes on the first input, as legacy does (R3-AUDIO-REC4, lines 1271–1280).
   - *Ids:* R3-AUDIO-REC1–REC4. *Need:* LOOP_V2 §7 (sound in the feel kit) and LOOK_V2 §7 only.
10. **The per-frame flow on the page.** Each frame (`setAnimationLoop`, R3-WGPU-F9, lines 115–119): (1) take the
    newest blocks, keeping two (r1-simulation-core.md, Recommendations, R1-BND-REC2, lines 2265–2285); (2) interpolate between the two
    newest ticks (r1-simulation-core.md, Recommendations, R1-TICK-REC5, lines 2094–2116), remote entities on ADR 0001's delay wherever its buffer lives
    (R3-XT-OQ1, r3-rendering.md lines 1424–1430, owner 0001); (3) write per-instance data into preallocated arrays (part 6); (4) animate within the
    budget LOD; (5) render the shared scene pass and the post chain; (6) start sounds (part 9) and visual effects
    (5.k) from the tick's events, the shot's own feedback at once and hit feedback only on confirmed hits (LOOP_V2 §7);
    (7) update the HUD in the DOM, legacy's PG and GP rows ported, dynamic text through `textContent`, including the
    owner's upgrade offers and tracks from owner-only data in the block (decision map I-21; ADR 0005 designs them).
    Presentation randomness comes from the page's own generator (R1-DET-REC4); hit-stop is presentation time only
    (R1-TICK-REC5); the cursor ray for the aim point hits the interpolated, drawn state (I-19: the command field is
    ADR 0001's, the ray query ADR 0004's). The page keeps rendering (R1-BND-REC1); OffscreenCanvas was not researched
    and is not proposed. The renderer reads typed arrays whatever the block's layout ("it reads typed arrays either
    way", r3-rendering.md, Pointers Q4, lines 1489–1491), so it sets no per-entity or per-field requirement on
    ADR 0002's layout (*answers* R3-XT-OQ7 and R1-XT-OQ16). Nothing in this flow allocates per frame (R3-CHAR-REC3's
    mark; R3-MEASURE-REC3 gates it). *Need:* VISION_V2 §6.2 (aiming at height), §6.9; LOOP_V2 §7, §11; UPGRADE-SCOPE
    (the track display).
11. **The spikes.** The look proof test (LOOK_V2 §9) and the render budget bench, set out under "What a spike must
    prove"; `docs/SPIKE_PLAN.md` gives their final names and places. **The look proof test needs authored models:**
    a few plain glTF/GLB exports (one soldier in LOOK_V2 §3's proportions with its faction colour slots, one weapon,
    one two-floor building with an interior and its damage stages, one very large unit of at least 20 parts, a few
    props), because LOOK_V2 §3's proportions and weapon silhouettes cannot be judged on legacy's primitive parts
    (CL10, `LEGACY_INVENTORY.md` line 119). It decides nothing about compression or LODs (ASSET-RESEARCH (a), the advisor's ruling; decision map J4).
    The only glTF evidence in the research is three.js's own example, which loads a glTF skinned mesh into the
    WebGPU renderer (R3-CHAR-F8, lines 476–480).

### How each LOOK_V2 item is achieved (LOOK_V2 §9)
One row per item of LOOK_V2 §1–§8 tagged `[DECIDED]` or `[DEFAULT]`; the `[OPEN]` tree style is the last row, and
nothing is decided for it. "Part" is this record's Decision part. "Not researched" names the open question (OQ) or the
look proof test's mark that stands in for missing research (plan rule 2). Line numbers are r3-rendering.md's unless
another file is named.

| LOOK_V2 item | Technique or system that delivers it | Research id, or "not researched" | What the look proof test shows | Owner if not 0003 |
|---|---|---|---|---|
| §1 `[DECIDED]` cel-shaded comic rendering with bold ink | node toon materials with shared ramps per material class (part 5.a); the ink pass (5.b) | R3-LOOK-F1–F3 (318–334); R3-LOOK-REC3 (1019–1028) | the toon-shaded, inked frame beside legacy's; both ink families | – |
| §1 `[DECIDED]` iconic, RA-style unit silhouettes | authored models drawn whole; silhouette ink from depth edges (5.b); LOD levels keep the outline shape (part 6) | models not researched (ADR 0005's asset-pipeline research); ink R3-LOOK-REC3 | a soldier, a vehicle and a very large unit read by silhouette at the game camera | 0005 (models) |
| §1 `[DECIDED]` clean, well-lit scenes with emissive accents | lighting from map data (5.g), soft shadows (5.f), selective emissive glow on the shared pass (5.d) | R3-LOOK-F12, F13 (392–402), R3-LOOK-REC2 (1002–1017); lighting model not researched: OQ-0003-5 | the lit scene with glow only on emissive slots | 0005 (emissive slots) |
| §1 `[DECIDED]` lush environments, made for an adult audience | props and vegetation instanced or merged inside the budget rows (parts 6, 7); environment one step calmer (5.h) | R3-CHAR-REC4 (1077–1092), R3-CHAR-OQ4 (1352–1354); vegetation not researched: OQ-0003-9 | a lush patch (fields, logs, trees) at the budget | 0005 (assets) |
| §2 `[DECIDED]` vibrant, beautiful colours | the output transform switch (5.e), half-float buffer | R3-WGPU-F12 (133–137), F14 (144–148), R3-WGPU-OQ4 (1296–1298): OQ-0003-3 | the same frame with both outputs beside legacy | – |
| §2 `[DECIDED]` soft shadows | one sun map with `PCFShadowMap`, texel-snapped and following the player (5.f) | R3-WGPU-F3 (78–83); R3-BUDGET-REC2's shadow row (1129); beyond one map not researched: OQ-0003-4 | shadow softness beside legacy; the shadow pass's GPU time | – |
| §2 `[DECIDED]` geometry that reads clearly | ink (5.b), toon bands (5.a), AO in corners (5.c), value hierarchy (5.h) | R3-LOOK-REC2, REC3 | the frame read at the game camera, ink and AO on and off | – |
| §2 `[DECIDED]` thick black outlines, especially strong on buildings | ink weight per material class, buildings heaviest (5.b) | R3-LOOK-REC3, R3-LOOK-OQ2 (1326–1327); a per-class weight in screen space not researched: OQ-0003-2 | buildings' ink beside legacy's, in both families | 0005 (material classes in the slots) |
| §2 `[DECIDED]` props: logs and trees with variety | instanced or batched props with per-instance transform and colour; static merging by material (part 6) | R3-CHAR-F4, F6 (452–467), R3-CHAR-REC4; `BatchedMesh` or `InstancedMesh` is R3-CHAR-OQ4 (the render budget bench) | a prop field with variety within the draw-call row | 0005 (prop assets) |
| §2 `[DECIDED]` uniform details as faction identity | authored models with faction colour slots; one shared material per slot and faction, or per-instance colour (part 6) | R3-CHAR-REC1 (1032–1048), REC3 (1065–1075) | a conscript in each faction's colours beside legacy's | 0005 (models, slots, faction variants) |
| §2 `[DECIDED]` small movement touches (the body reacting to strafing) | procedural animation on the page (legacy GA rows ported), the full rig for important soldiers under the budget LOD (part 6) | R3-CHAR-REC2 (1050–1063); legacy GA15 | strafing on the full rig; what the cheaper rig drops | – |
| §2 `[DECIDED]` architecture: half-timbered house with brick-tile roof; plastered house with silo tower | building assets under the toon materials and the building ink weight (5.a, 5.b) | models not researched (ADR 0005) | both houses beside legacy's | 0005 (building assets) |
| §2 `[DECIDED]` grenade concussion: shake tilting away from the blast, stronger closer in | camera effect on the page from the blast event, legacy's spring kick ported, presentation randomness (5.m) | R1-DET-REC4 (r1-simulation-core.md 2184–2207); legacy CD9, GP4; nothing beyond the legacy port researched | Mustafa feels it beside legacy | 0004 (the blast event) |
| §3 `[DECIDED]` the audience is adults, not a children's game | no engine feature: the whole look (proportions, materials, ink) | none needed | Mustafa's overall judgement of the frame | 0005 (assets) |
| §3 `[DECIDED]` slimmer, serious character proportions | one skinned mesh per soldier with any skin weights (part 6) | R3-CHAR-REC1; R3-CHAR-F1 (436–439), F8 (476–480) | the new soldier skinned and animated on both backends | 0005 (the model, R3-CHAR-OQ3) |
| §3 `[DECIDED]` weapons with real silhouettes | authored weapon models on the hand sockets (legacy CC9's data ported) | models not researched (ADR 0005) | weapons read at the game camera | 0005 (weapon models) |
| §3 `[DECIDED]` the thatched roof goes | content only: the building kit drops it | none needed | absent from the test buildings | 0005 (building kit) |
| §3 `[DECIDED]` buildings sized for play: interiors, upper floors, verticality | interiors drawn with AO in corners (5.c), the roof fade (5.i), floors from the sim | R3-LOOK-OQ3 (1328–1329); R4-FLOORS-REC2 (r4-physics-navigation.md 925–939); R2-INT-REC2 (r2-netcode.md 2445–2466) | a two-floor building, entered, its roof fading | 0004 (floors), 0005 (building assets) |
| §3 `[DECIDED]` fragmentation comes back | pooled fragments as visual effects from the blast event, on both backends (5.k) | R3-GL2-REC1 (955–967); R3-BUDGET-REC2's visual-effects row (1132); the particle technique not researched: OQ-0003-7 | a grenade blast with flying fragments beside legacy's smoke and fire | 0004 (the blast event); damage per fragment is a design question |
| §4 `[DECIDED]` lush countryside plus contested urban centres (enterable ruins, collapsed roofs, planks to upper floors, corners) | the same pipeline; ruins as damage-stage meshes (5.j); planks as the sim's typed links | R4-FLOORS-REC1 (r4-physics-navigation.md 903–924), R4-OBST-REC2 (1043–1061) | a ruin entered by a plank | 0004 (routes, states), 0005 (assets) |
| §4 `[DEFAULT]` damage stages: intact → damaged → ruin; debris and planks become routes | a mesh per stage swapped on the structure-state event, every stage compiled in the warm-up; debris as visual effects (5.j, part 3) | R3-WGPU-REC3 (928–939); R4-OBST-REC2 bakes two states: a third is ADR 0004's question (decision map part h, G4) | a building through three stages with no frame above the first-sight mark | 0004 (states), 0005 (stage assets) |
| §4 `[DEFAULT]` roofs fade when the player is inside | the cutaway over the player's enclosure, ink masked, precompiled blend variant (5.i) | legacy GW17; R2-INT-REC2; R3-WGPU-REC3; the fade technique not researched: OQ-0003-6 | walking in and out; ink hidden on faded parts | 0004 (which enclosure), 0001 (visibility rule) |
| §5 `[DECIDED]` iconic silhouettes you could draw from memory | as §1's silhouettes | models not researched (ADR 0005) | as §1 | 0005 (models) |
| §5 `[DECIDED]` Planetary Annihilation's principle: readable from a distance, depth up close | the asset's mesh LOD levels switched with hysteresis where the framing spreads distances; the budget LOD for animation (part 6) | R3-CHAR-REC2, R3-CHAR-F10 (490–493), F13 (505–510); mesh LOD generation not researched (ADR 0005) | a unit at the closest and the farthest camera framing | 0005 (LOD levels) |
| §6.1 `[DEFAULT]` value hierarchy: the environment one step calmer | a colour gain per material class, tuned in the test (5.h) | not researched: tuning in the look proof test (legacy CL4's nature gain is the precedent) | environment one step calmer than units, faction colours and visual effects | 0005 (material classes) |
| §6.2 `[DEFAULT]` line weight: thick on silhouettes, thinner inside | depth edges heavier than normal edges (5.b) | R3-LOOK-F7 (360–364); inner lines not researched: OQ-0003-2 | both weights side by side | – |
| §6.3 `[DEFAULT]` weapons and headgear first | per-unit budgets that favour them; no renderer feature | none (ADR 0005's budgets) | weapons and headgear read at the camera | 0005 (budgets) |
| §6.4 `[DEFAULT]` a glow language: objectives, faction tech, muzzle flashes emissive | emissive slots into the emissive MRT output and selective bloom (5.d) | R3-LOOK-F12, F13; R3-LOOK-REC2 | those three glow, nothing else | 0005 (emissive slots) |
| §6.5 `[DEFAULT]` warm sun with cool shadows | coloured sun and fill lights per map (5.g) | not researched: OQ-0003-5 | warm lit sides, cool shadow sides | 0005 (map lighting data) |
| §6.5 `[DEFAULT]` soft ambient occlusion in corners, so interiors read from above | GTAO at half resolution on the shared pass, denoised (5.c) | R3-LOOK-F9–F11 (373–389); R3-LOOK-OQ3 | interiors with AO on and off | – |
| §6.5 `[DEFAULT]` different moods per map, for example dusk | the map document's lighting mood, applied at load (5.g) | not researched: OQ-0003-5 | the same scene at day and at dusk | 0005 (map document) |
| §7 `[DECIDED]` punchy weapons through VFX, sound and feedback (the feel kit) | visual effects (5.k), voices (part 9) and HUD feedback (part 10) from the feel kit's data and the block's events; hit feedback on confirmation | R3-AUDIO-REC1–REC4 (1229–1280); R3-BUDGET-REC2's visual-effects row; the particle technique: OQ-0003-7 | the Imperial Warrior's SMG firing beside legacy's | 0005 (feel kit data), 0001 (confirmation), 0004 (events) |
| §7 `[DECIDED]` grenades: fragmentation first, with the concussion screen effect | as §3's fragmentation and §2's concussion (5.k, 5.m) | as those rows | as those rows | 0004 (the blast event) |
| §7 `[DEFAULT]` comic impact touches for big moments, sparingly: halftone flashes, ink line boil | halftone: legacy's DOM overlay ported (CD16); line boil (5.l) | halftone: legacy CD16 only; line boil not researched: OQ-0003-8 | both on one big moment, with their cost | 0004 (moment events) |
| §8 `[DECIDED]` v2 may use authored 3D models | glTF/GLB loaded and given the comic look per material slot (5.a) | R3-CHAR-F8 (476–480) only | the test's authored assets | 0005 (the pipeline) |
| §8 `[DEFAULT]` the flow: sketch → Spline model → glTF/GLB → automated compress, LODs, slot renaming → the engine applies the look | step 5 is part 5.a; steps 3–4 are ADR 0005's | R3-CHAR-F8 only; steps 3–4 not researched (ADR 0005's asset-pipeline research) | plain glTF/GLB exports, with no compression or LOD step | 0005 (steps 1–4) |
| §8 `[DEFAULT]` budgets per unit; never Spline's runtime | per-unit budgets derived from this record's totals; no Spline runtime is loaded | R3-BUDGET-REC2 (1118–1164), R3-CHAR-REC1 | test assets counted against the budget rows | 0005 (per-unit budgets) |
| §8 `[OPEN]` the tree style | nothing decided; the test's trees are labelled placeholders | – | – | Mustafa (OQ-0003-9 lists it) |

## Options considered

Each table carries the template's dimensions for every option the research's Options sections give (r3-rendering.md,
Options, lines 774–894). "Presentation only" in the security column means the option draws what the client already
holds and changes nothing the authority decides. An option that contradicts a settled input is one line, not weighed.

### Q1. Which renderer (R3-WGPU; Options Q1, lines 778–790)
| Option | Complexity | Cost (money, time, usage) | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort from the current code |
|---|---|---|---|---|---|---|
| **A. WebGPURenderer with its built-in WebGL 2 backend** (proposed) | Medium: one renderer, TSL new to the team | one shader set; upgrade work each release (R3-WGPU-F3) | WebGPU-only gains on WebGPU; "a better performance with WebGLRenderer" possible for some scenes (R3-WGPU-F7, lines 104–107) | presentation only | one look on every backend | all new TSL; legacy's GLSL does not carry over (R3-WGPU-F6) |
| B. WebGPURenderer on WebGPU plus a separate WebGLRenderer for the fallback | High: two renderers, two material sets | every look feature twice, two sets of visual tests | fallback players get the mature renderer (R3-WGPU-F8, lines 109–113) | presentation only | look drift between the two paths | TSL and GLSL both written new |
| C. WebGLRenderer only (WebGL 2) | settled by the stack (`docs/V2_PHASE0_BRIEF.md` §0, line 8; brief §1) | – | – | – | – | – |

- **A.** Pros: one source for both backends (R3-WGPU-F11); serves all four browsers through one path (R3-WGPU-F17–F19,
  lines 159–175). Cons: "experimental" (R3-WGPU-F4, lines 85–89); may be slower than WebGLRenderer on some scenes (F7).
- **B.** Pros: the fallback runs on the renderer three.js calls "the recommended choice for pure WebGL 2 applications"
  (F8). Cons: doubles material, post and outline work and the tests; the two paths drift apart.

### Q2. How the pinned release moves (R3-WGPU; Options Q1, lines 786–790)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **V1. Pin and step** (proposed) | Low | one release at a time, with the migration guide and a benchmark re-run | every regression is measured before it lands | presentation only (one lockfile entry) | none unmeasured | a CI job and a checklist (r3-rendering.md, Pointers Q1, lines 1470–1471) |
| V2. Track latest | Low | cheaper per step | an API break (R3-WGPU-F3) can land mid-feature | presentation only | breaks can surface unplanned | the same tooling |

- **V1.** Pros: no break lands in the middle of feature work. Cons: lags three.js's advice to "upgrade to the latest
  version whenever possible" (R3-WGPU-F4).
- **V2.** Pros: follows that advice. Cons: r183, r185 and r186 each broke an API within a year (F3).

### Q3. What the WebGL 2 fallback gets (R3-GL2; Options Q2, lines 792–799)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **F-A. Same features on both backends** (proposed) | Low: one look, one test matrix | lowest | forgoes GPU particles and compute culling on WebGPU; §6.9's visual effects must fit CPU or vertex work | presentation only | one look everywhere | legacy uses nothing WebGL 2 lacks (CD15; r3-rendering.md, Findings Q2, legacy facts, lines 300–302) |
| F-B. Tiered: WebGPU-only extras with a defined fallback each | Medium | two code paths per extra, tests on both | more visual effects on WebGPU | presentation only | fewer or simpler effects on the fallback; drift | each extra needs its fallback built |
| F-C. The fallback always runs Low | Low | small | strong Linux or Firefox-on-Mac machines get Low | presentation only | Low settings even on strong machines | small |
| F-D. Try WebGPU compatibility mode before WebGL 2 | unknown: needs three.js support (R3-GL2-OQ1, lines 1305–1307) | unknown | could widen WebGPU's reach on older devices; opt-in, platforms still being added (R3-GL2-F13, lines 285–291) | presentation only | none | a source reading first |

- "WebGPU required, no fallback" was not researched as an option (R3-GL2-OQ5, lines 1317–1320); it stays Mustafa's
  question (OQ-0003-11).
- **F-A.** Pros: the fallback's emulations (R3-GL2-F2, F3) never matter; nothing legacy had is lost. Cons: WebGPU's
  compute stays unused until a measured need.
- **F-B.** Pros: richer visual effects where WebGPU runs. Cons: two paths per extra; look drift. Kept for measured
  needs only (part 1).
- **F-C.** Pros: simple. Cons: punishes some recommended-class machines (Options Q2, line 798).
- **F-D.** Pros: could keep more of the minimum class on WebGPU. Cons: unknown three.js support; opt-in and partial.

### Q4. How the toon shading is built (R3-LOOK; Options Q3, lines 806–809)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **Node toon material with a gradient map** (proposed, 5.a) | Low: shipped by three.js (R3-LOOK-F1) | a few bytes per ramp | no extra pass; scales with materials (R3-LOOK-F3) | presentation only | banding shimmer without smoothing (only the default two bands are smoothed, F2); one material per ramp breaks sharing (F3) | legacy ramps carry over as single-channel textures (CL2; Findings Q3, legacy facts, lines 418–419) |
| Custom TSL lighting (own bands, per-kind ramps, rim) | Medium: own code | none in memory | no extra pass; same material-count risk | presentation only | free to shape bands and shade colour | new code; legacy's rim injection does not carry over (CL3) |

- **Node toon material.** Pros: maintained by three.js, both backends. Cons: the ramp is single-channel, so colour in
  shade comes from the lights (F2). **Custom lighting.** Pros: any band shape and shade colour. Cons: our own code to
  keep working across releases. Kept as the fallback if the look proof test needs it (OQ-0003-5).

### Q5. How the ink is drawn (R3-LOOK; Options Q3, lines 810–813)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| Inverted hull, three.js `toonOutlinePass` (F4, F5) | Low: shipped | one extra draw per outlined object | scales with outlined objects and vertices; Low cap about 1,100 draws (R3-BUDGET-REC2) | presentation only | width constant on screen; only toon materials; gaps at hard corners without smoothed normals (CL7) | new; legacy's GLSL hull does not carry over |
| Inverted hull, own TSL (world-unit width, ground clamp as CL6) | Medium: own code | the same extra draw per object | as above | presentation only | legacy's look: even world-unit width with the ink contact line | the outline-normal algorithm ports (CL7, `LEGACY_INVENTORY.md` line 116); the shader is new |
| Screen-space Sobel on colour (F6) | Low: shipped node | one full-screen pass, 9 samples per pixel | scales with pixels only | presentation only | picks up texture detail; misses edges between same-coloured surfaces | new |
| **Screen-space depth and normal edges** (proposed, 5.b; F7) | Medium: a custom TSL node | one full-screen pass plus one normal attachment, about 16.6 MB at 1080p (F14) | scales with pixels only; Low stays at 800 draws | presentation only | lines in pixels; can mark folds inside a shape; threshold tuning; thin lines alias; needs masking for the roof fade | new |

- **The hulls.** Pros: legacy's proven look; a per-object weight for buildings is a plain thickness value (F5).
  Cons: doubles the draws of outlined objects, the measured legacy problem (CL6, TL15).
- **Sobel on colour.** Pros: shipped, cheap. Cons: draws texture detail as ink and misses same-coloured edges.
- **Depth and normal edges.** Pros: fixed cost per pixel whatever N; shares the normal buffer with AO. Cons: custom
  code; aliasing; a per-class weight and inner lines are not researched (OQ-0003-2).

### Q6. How ambient occlusion is made (R3-LOOK; Options Q3, lines 814–816)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| GTAO as documented (F9–F11) | Medium | depth-normal pre-pass (the scene drawn twice) plus AO plus denoise or TAA; AO, normal and history targets | draws double; passes 800 and 1,100 at Low (R3-BUDGET-REC2, lines 1148–1151) | presentation only | noise or TAA ghosting; halos at depth gaps; recompiles when samples change (F10) | new |
| **GTAO at half resolution, no pre-pass** (proposed, 5.c) | Medium | AO plus denoise; AO target at a quarter of the area | scales with pixels / 4 | presentation only | softer; may flicker on thin geometry | new |
| Baked or vertex AO, no post-processing | Low | a per-vertex attribute; nothing per frame | none per frame | presentation only | static only: no contact shadow from moving units | new for v2's assets |

- **GTAO documented.** Pros: three.js's documented setup. Cons: doubles draws. **Half resolution.** Pros: no second
  geometry pass; "a value of `0.5` should be sufficient for most scenes" (F10). Cons: whether it is good enough is
  R3-LOOK-OQ3. **Baked.** Pros: free per frame. Cons: nothing that moves gets AO.

### Q7. How the glow is made (R3-LOOK; Options Q3, lines 817–818)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| Full-image bloom (F12) | Low | 12 passes at falling resolution (F13); 11 half-float targets | scales with pixels | presentation only | washes bright UI-like colours; needs a threshold | new (legacy has no post-processing, CD15) |
| **Selective emissive bloom via MRT** (proposed, 5.d) | Low | the same 12 passes plus one emissive attachment | scales with pixels | presentation only | only emissive slots glow; each emissive material must write the output correctly | new |

- **Full-image.** Pros: no extra output. Cons: anything bright glows, against LOOK_V2 §6.1 and §6.4. **Selective.**
  Pros: exactly LOOK_V2 §6.4's glow language. Cons: every emissive material must write the emissive output.

The colour output (5.e) and the shadow setup beyond one map (5.f) have no options section in the research; they are
open questions (OQ-0003-3, OQ-0003-4), not proposals.

### Q8. How soldiers and very large units are drawn (R3-CHAR; Options Q4, lines 820–836)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| C-A. Rigid parts as separate meshes (legacy) | Low | about 120 draws per soldier with hulls (CL6) | poor at 48+: 5,843 draws at 48 (TL15) | presentation only (animation is cosmetic) | legacy's primitive look | ports as is, then fails the budget |
| **C-B. One skinned mesh per soldier** (proposed) | Medium | one draw per material per soldier; a bone upload | good up to tens of soldiers; per-frame bounds avoided by a padded sphere | presentation only | any skin weights for authored models | new; joints become bones |
| C-C. Instanced skinning, one shared pose (F8) | Low | one draw per material for all | every soldier in the same pose | presentation only | unusable for aim and gait | three.js example |
| C-D. Instanced skinning, pose per instance by compute (F9) | High | one draw per material for all | strong on WebGPU; WebGPU only | presentation only | full poses | needs its own WebGL 2 path |
| C-E. Instanced skinning by bone texture (custom TSL) | High | one draw per material for all | strong on both if it works | presentation only | full poses | not a shipped node (F11); own code |
| C-F. Baked vertex animation for far or idle soldiers | Medium | bake tooling | none per soldier | presentation only | loses procedural aim and IK | new tooling |
| C-G. Impostors | Medium | sprite baking | one draw for all | presentation only | weak under a near-constant top-down distance (F13) | new |
| **L-2. Budget LOD for animation** (proposed) | Medium | – | bounded CPU whatever N | presentation only | visible pops if the budget is low | the GA rows port; the budget is new |
| L-1. Distance LOD (F10) | Low | – | little gain under a top-down camera (F13) | presentation only | partial | three.js's `LOD` |
| **V-A. Very large units as part groups** (proposed) | Low | few units, so few draws | parts stay independently destructible and pickable | presentation only | parts match the sim's records | the Juggernaut's multi-part build (GC16) is the precedent |
| V-B. One rigid-skinned mesh per large unit | Medium | halves draws | – | presentation only | – | new |

- **C-B.** Pros: documented skinning; both backends. Cons: how WebGPURenderer skins is unconfirmed (R3-CHAR-OQ1,
  lines 1344–1346), so the spike checks both backends. **C-D / C-E.** Pros: one draw for a crowd. Cons: C-D lacks a
  fallback path (R3-GL2-F2); C-E is our own code. Measured against C-B. **C-A, C-C, C-F, C-G.** Cons as in the table;
  C-F and C-G only as far tiers.
- **L-2.** Pros: bounds animation cost under any camera. Cons: pops if the budget is low. **L-1.** Pros: shipped.
  Cons: little gain while soldiers sit at similar camera distances; conditional on the framing.
- **V-A.** Pros: parts map one-to-one onto the block's vehicle-part section. Cons: more draws than V-B, which "a few"
  units absorb (VISION_V2 §6.9). **V-B.** Pros: halves draws. Cons: loses per-part destruction and picking.

### Q9. Which machines, and what is budgeted (R3-BUDGET; Options Q5, lines 838–853)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **M-A. Named models bought or borrowed** (proposed, with M-B as the fallback) | Low | money, and someone to run them | repeatable numbers | none | – | none in code |
| M-B. Classes only | Low | free | numbers not comparable between runs | none | – | none |
| M-C. Mustafa's PC as the recommended machine | Low | free | far above mid-range, so it hides problems | none | Brave is the final visual gate anyway | none |
| M-D. A cloud GPU instance | Medium | rented by the minute | repeatable; not a laptop | runner access is ADR 0006's | – | CI setup |
| B-1. Frame time only | Low | – | says nothing about where time goes | none | – | – |
| **B-2. Frame time plus per-pass GPU, CPU, draw calls, triangles, memory** (proposed) | Medium | needs timestamp queries | every regression has an owner | none | – | the harness (part 8) |
| B-3. Adaptive resolution on top of B-2 | Medium | one more control loop | holds the target on weak machines | none | the image softens under load | new |

- **M-A** pros: repeatable; cons: costs money (a decision for Mustafa, OQ-0003-1). **M-B** pros: free; cons: not
  comparable. **M-C** pros: always there; cons: hides problems. **M-D** pros: repeatable; cons: not the minimum class.
- **B-2** pros: each pass has a budget; cons: needs GPU timers, missing on some browsers (part 4). **B-1** cons: blind
  to the cause. **B-3** pros: holds frame time; cons: softens the image; kept as a measured option.

### Q10. How and where it is measured (R3-MEASURE; Options Q6, lines 855–870)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **T-A. WebGPU timestamp queries** (proposed, first) | Medium | queries per pass, async readback | 100 µs in Chrome by default (R3-MEASURE-F2, lines 610–615) | timers are coarsened on purpose (R3-MEASURE-F1, lines 604–608) | – | three.js's `trackTimestamp` |
| **T-B. WebGL 2 timer queries** (proposed, second) | Low | per pass | Chrome and Edge only (R3-MEASURE-F6, lines 636–639) | – | – | three.js does it when present |
| **T-C. CPU frame time** (proposed, always) | Low | none | 100 µs resolution, fine for 16.7 ms frames (R3-MEASURE-F7, lines 641–646) | – | – | `performance.now()` |
| **W-A. CI headless** (proposed, counts only) | Low | CI minutes | proves counts, never a frame time (R3-MEASURE-F8) | – | – | the gate v2 (ADR 0002) |
| W-B. CI on a GPU runner | Medium | paid per minute (R3-MEASURE-F9, lines 655–658) | one cloud GPU, not a laptop | ADR 0006's runner policy | – | CI setup |
| **W-C. The reference machines** (proposed, all times) | Medium | the machines' time | the VISION_V2 §4 targets | self-hosted runners would be ADR 0006's (R3-MEASURE-OQ2) | – | a results log |
| **W-D. Mustafa's Brave run** (proposed, the final visual gate) | Low | his time | his hardware, not the recommended class | – | the final visual gate | – |

- **T-A/T-B/T-C** together: GPU time where it exists, CPU time everywhere, "not available" otherwise. **W-A** pros:
  free and every change; cons: no timing. **W-B** pros: repeatable GPU; cons: paid and not a laptop. **W-C** pros: the
  real targets; cons: runs only when the machines are on. **W-D** pros: the real player view; cons: one machine.

### Q11. How sound is placed, capped and occluded (R3-AUDIO; Options Q7, lines 872–894)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| P-A. `StereoPannerNode` plus own gain (legacy) | Low | lowest CPU (R3-AUDIO-F7, lines 730–734) | – | presentation only | no front-back or height cues | ports as is (GP6) |
| **P-B. `PannerNode` with `equalpower`** (proposed) | Low | "simple and efficient" (R3-AUDIO-F2, lines 696–700) | one listener, data-driven curves | presentation only | left-right with distance | new routing; recipes port |
| P-C. `PannerNode` with HRTF | Low | highest, unmeasured (R3-AUDIO-F9, lines 743–747) | – | presentation only | best on headphones | three.js's default (R3-AUDIO-F5, lines 717–722) |
| P-D. HRTF for a few priority voices, equal power for the rest | Medium | bounded | needs the priority system anyway | presentation only | good where it matters | – |
| V-A. No voice limit (legacy) | Low | unbounded nodes in a fight at N = 48 | – | presentation only | – | legacy |
| **V-B. Voice cap with priorities** (proposed) | Medium | bounded | tuning needed | presentation only | quiet sources vanish | new |
| V-C. Distance cull only (legacy's 90 m) | Low | a cluster inside the radius is unbounded | – | presentation only | – | legacy GN8 |
| O-A. No occlusion (legacy) | Low | none | – | presentation only | sounds through walls read as in the open | legacy |
| O-B. Rays to static geometry, lowpass plus gain (F8) | Medium | one ray per voice at a low rate; one filter per voice | – | presentation only: rays against geometry the client already holds | thin walls; floors need height-aware rays | new |
| **O-C. Zones from map data** (proposed, or none) | Low | cheap | – | presentation only | coarse | map data (ADR 0005) |

- **P-B** pros: one model replaces legacy's split of gain and pan; cons: no binaural cues. **P-C** cons: unmeasured
  cost; kept as **P-D** for a few voices, measured. **P-A** cons: no height cue for §6.2's floors.
- **V-B** pros: bounded cost; cons: tuning. **V-A** and **V-C** cons: unbounded in a fight.
- **O-C** pros: cheap; cons: coarse. **O-B** pros: real occlusion; cons: rays and filters per voice, height-aware data.
  **O-A** cons: sounds behind walls read as open.

## Trade-off analysis

- **One TSL renderer with its own fallback (A, F-A)** wins because the look is written once and runs everywhere the
  four browsers run, and the fallback's emulations never matter while no feature needs compute (R3-GL2-F2, F3). *The
  strongest argument against:* three.js calls WebGPURenderer experimental and admits WebGLRenderer can be faster on
  some scenes (R3-WGPU-F4, F7). The look proof test measures exactly that (R3-WGPU-REC1 mark 3), and the fallback
  path is the same TSL code, so a failure on the WebGPU backend still leaves a working game on the WebGL 2 backend.
- **Screen-space ink over the hull** wins on cost at N = 48: its pass is fixed per pixel, it shares the normal output
  with the AO, and it keeps the Low cap at 800 draws, where the hull adds a draw per outlined object (R3-LOOK-F8;
  R3-BUDGET-REC2, lines 1142–1148), the cost legacy measured at 5,843 draws with 48 soldiers on screen (TL15). *The
  strongest argument against:* LOOK_V2 §2 keeps legacy's ink, and legacy's ink is a hull; the hull is shipped code
  with a known look and a trivial per-object weight for buildings, while the screen-space pass is our own TSL node with
  thresholds to tune, aliasing, a mask for the roof fade, and a per-class weight that is not researched. That is why
  both families are measured, and why the look call is Mustafa's: if he prefers the hull, the record keeps it at the
  measured cost (about 1,100 draws at Low, or High only).
- **Half-resolution GTAO on the shared pass** wins because the documented pre-pass doubles every draw (R3-LOOK-F11).
  *Against:* nothing documents that single-pass AO looks good enough (R3-LOOK-OQ3); the look proof test decides, and
  the pre-pass stays a High-only fallback.
- **Selective glow** wins because LOOK_V2 §6.4 names what glows; full-image bloom cannot tell an objective from a
  bright wall. *Against:* every emissive material must write the emissive output correctly, a content rule for
  ADR 0005's slots.
- **One skinned mesh per soldier with a budget LOD (C-B, L-2)** wins because it runs on both backends with documented
  skinning and bounds animation cost whatever the camera (R3-CHAR-REC1, REC2). *Against:* instanced skinning could
  draw a crowd in one call per material; it is measured against C-B and adopted only with at least 20 % less frame
  time on both machines.
- **Named machines and B-2 budgets** win because VISION_V2 §4 names classes only and no allow-listed source ranks
  hardware (R3-BUDGET-F7), so only named machines make numbers comparable. *Against:* they cost money and someone's
  time to run (OQ-0003-1).
- **Equal-power panners with a voice cap** win because a playing voice costs its nodes whatever its gain, so only a
  cap bounds the cost (R3-AUDIO-F4, inferred), and equal power is the efficient model (R3-AUDIO-F2). *Against:* no
  source gives any voice cost (R3-AUDIO-F9), so 32 and 48 are starting points the render budget bench confirms.

## Consequences

- **Easier:** one shader language for both backends; every look technique has a tier switch from the start, so Low
  is a data change; the budget rows are data in the repository (ADR 0002's I-14), so a regression shows in CI as a
  count; per-unit asset budgets derive from one table (ADR 0005); legacy's draw-call problem is designed out (4 draws
  per soldier per pass against about 120).
- **Harder:** TSL is new to the team and three.js's API moves each release (R3-WGPU-F3); the screen-space ink and the
  instanced-skinning candidates are our own TSL code; the fallback rule (F-A) keeps GPU particles and compute culling
  out until a measured need; frame times need two physical machines and someone to run them; Mustafa must judge the
  look in a dedicated session before any art production.
- **Revisit when:** the look proof test fails a frame-time mark on the minimum laptop (drop a technique to High only,
  or change family); a three.js upgrade changes a pass's cost by more than R3-WGPU-REC2's 5 %; client field data shows
  the fallback share (R3-GL2-OQ2), which may reopen dropping the fallback (OQ-0003-11) or allow F-B extras; the camera
  framing is decided (R2-INT-OQ2), which re-decides distance LOD and the listener's place; Mustafa picks the hull
  family.
- **Hard constraints in CLAUDE.md that change:** none in this PR: nothing in `CLAUDE.md` or `.claude/` changes. Once
  Mustafa accepts the drafts, under SPIKE-SCOPE (a) the legacy constraints apply to `legacy/` only, so for v2 code:
  "Three.js r128 UMD (no CapsuleGeometry). No addons, no post-processing (under review)." stops applying (this record
  adopts the latest pinned three.js with its node post-processing, parts 1, 2 and 5); "`LinearEncoding` output:
  authored hex = on-screen colour" stops applying as a constraint and becomes OQ-0003-3, Mustafa's call in the look
  proof test; "No external assets (procedural textures, synthesized audio)" ends with the legacy engine (LOOK_V2 §8),
  so v2 loads authored glTF/GLB models (part 11; ADR 0005), while synthesized sound recipes still port (part 9);
  "`stylizeScene()` runs once; runtime builders call `stylizeOne()` themselves" is replaced by the engine applying the
  look to each asset's material slots at load (part 5.a). "Animation is cosmetic: shots come from `playerFireOrigin()`
  / fixed chest points. Nothing animated decides a hit." is not changed: v2 keeps it (ADR 0004's R1-DET-REC4:
  "animation never moves a launch point or a hitbox", r1-simulation-core.md lines 2193–2194). The code rules on
  per-frame allocation, pooled visual effects and disposal carry over as v2 rules (parts 5.k, 6, 10).

### Legacy inventory rows
The rows whose mark this decision would change, including the "look brief pending" rows LOOK_V2 now unblocks
(R7-MARKS (a)). The inventory file is not edited: the change waits for Mustafa's acceptance. Rows with the same
change share a line. "Pending" is the row's *Look brief pending* note. The building-kit rows (CM), the kit and ground
textures and builders (CL13, CD20, CD21), the tree and prop builders (GW18, GW19) and the other asset rows are
ADR 0005's.

| Row id (`LEGACY_INVENTORY.md` line) | Mark now | Proposed mark | Why |
|---|---|---|---|
| CD15 (96) | redesign (look brief pending) | redesign: the v2 renderer (part 1); colour output OQ-0003-3; shadows part 5.f | LOOK_V2 §2 keeps vibrant colour and soft shadows; the r128 renderer is replaced |
| CD16 (97) | redesign (look brief pending) | port to TypeScript: the DOM halftone overlay, triggered sparingly on big moments (part 5.l) | LOOK_V2 §7 `[DEFAULT]` names halftone flashes |
| CD17, CD19 (98, 100) | redesign (look brief pending) | redesign: lights, fog and mood as map data (part 5.g); one sun shadow map (5.f) | LOOK_V2 §2, §6.5 |
| CD5 (86) | port to TypeScript (look brief pending) | unchanged; pending becomes "camera framing pending (R2-INT-OQ2)" | LOOK_V2 does not set the camera |
| CD9, GP4 (90, 314) | port to TypeScript (pending) | port to TypeScript: the spring kick and concussion from the blast event (part 5.m); pending removed | LOOK_V2 §2 keeps the concussion feel |
| CL1, CL6, CL7 (110, 115, 116) | redesign (look brief pending) | redesign: the ink pass in TSL (part 5.b); the hull family is measured as the alternative, and CL7's outline-normal algorithm ports only if Mustafa picks the hull | LOOK_V2 §2, §6.2 keep thick ink; the GLSL hull does not carry over (R3-WGPU-F6) |
| CL2, CL4, CL8, CL14 (111, 113, 117, 123) | redesign (look brief pending) | redesign: shared node toon materials per material class, applied by the engine to each asset's slots at load (part 5.a); no runtime conversion of authored materials | LOOK_V2 §1, §8 step 5; CL4's one-material-per-mesh lesson |
| CL3 (112) | redesign (look brief pending) | redesign in TSL only if kept: LOOK_V2 names no skin rim, so whether one stays is Mustafa's call in the look proof test | the rim was GLSL injection (R3-WGPU-F6) |
| CL9 (118) | port to TypeScript (pending) | port to TypeScript: static merging by material content on both backends (part 6); pending removed | the appearance is unaffected (pixel-identical) |
| CL10, CC1–CC5 (119, 131–135) | port to TypeScript (pending) | redesign: soldiers become authored, skinned models (part 6; ADR 0005's pipeline); the slot, role and palette schema ports as the faction colour slots | LOOK_V2 §3 (slimmer, serious proportions), §8 |
| CC6, CC8, CC9, CC11, CC12, GC6 (136–138, 140, 141, 245) | port to TypeScript (pending) | port to TypeScript: kit data, sockets, IK and props port; the gun and prop shapes become authored models (ADR 0005); pending removed | LOOK_V2 §3 (real weapon silhouettes) |
| CL12, GW3 (121, 191) | port to TypeScript (pending) | unchanged; pending becomes "Mustafa's call: LOOK_V2 §7 names halftone flashes and line boil, not comic words" | LOOK_V2 does not mention onomatopoeia |
| GA1–GA15 (217–231) | port to TypeScript (pending) | port to TypeScript; pending removed; GA15 adds the budget LOD (part 6) | LOOK_V2 §2 keeps small movement touches |
| GA14 (230) | port to TypeScript (pending) | port to TypeScript; pending becomes "Mustafa's call: whether the toy-soldier death fall fits LOOK_V2 §3's adult audience" | LOOK_V2 §3 |
| GW4 (192) | port to TypeScript (pending) | port to TypeScript: the texel-snapped sun frustum (part 5.f); pending removed | LOOK_V2 §2 soft shadows |
| GW17 (205) | port to TypeScript (pending) | port to TypeScript: the roof fade by flag over the player's enclosure, precompiled blend variant, ink masked (part 5.i); pending removed | LOOK_V2 §4 `[DEFAULT]` roofs fade |
| GP1, GN8, GC8 (311, 273, 247) | port to TypeScript (pending) | port to TypeScript: pools per kind with caps as tier data, spawned from events (part 5.k); pending removed | LOOK_V2 §3, §7 |
| GN1 (266) | port to TypeScript (pending) | port to TypeScript: instanced health bars and a per-instance flash (part 6); pending removed | R3-CHAR-REC3 |
| GC16 (255) | port to TypeScript (pending) | redesign: very large units as authored multi-part models in one group (part 6, option V-A) | LOOK_V2 §5 (never "a block on wheels with a tube") |
| GP5 (315) | redesign (pending) | redesign: the perf overlay ported with backend, GPU time and programs (part 8); pending removed | R3-MEASURE-REC4 |
| GP7, GP10, GP12, PG2–PG8, PG10–PG12, PG14–PG16, PG20 (317, 320, 322, 360–366, 368–370, 372–374, 378) | port to TypeScript (pending) | unchanged; pending stays for the HUD's layout, which LOOK_V2 does not set | LOOK_V2 covers the 3D look, not the HUD |
| GP9 (319) | port to TypeScript (pending) | unchanged; pending becomes "Mustafa's call: LOOK_V2 does not mention the kill vignette" | presentation-time hit-stop stays (R1-TICK-REC5) |

## Open questions

- **OQ-0003-1. Which laptop and which desktop are the two reference machines?**
  Why it matters: every client mark, and R1's client marks, are measured on them (decision map I-15).
  Options: M-A, named models bought or borrowed, in the classes of part 7; M-B, classes only, as the fallback
  (r3-rendering.md, Options Q5, lines 841–846). Decider: Mustafa and the Advisor (money and availability).
  What decides it: what can be bought or borrowed; each is recorded with GPU, CPU, RAM, screen, `devicePixelRatio`
  (R3-BUDGET-REC1's mark, lines 1114–1116) and `hardwareConcurrency` (R1-BND-OQ3, r1-simulation-core.md lines
  2728–2731). Research ids: R3-BUDGET-OQ1 (lines 1364–1365), R1-TICK-OQ6 (r1-simulation-core.md lines 2662–2665),
  R1-BND-OQ3.
- **OQ-0003-2. The ink's width, its inner lines and its weight on buildings.**
  Why it matters: LOOK_V2 §2 asks for ink "especially strong on buildings" and §6.2 for thinner lines inside.
  Options: width in world units (legacy CL6), constant on screen (three.js's hull, R3-LOOK-F5), or in pixels
  (screen-space); inner lines from normal edges (screen-space) or none beyond the silhouette (hull); the building
  weight as a per-pixel class value in an MRT output (screen-space, not researched) or a per-object thickness (hull).
  Decider: Mustafa, in the look proof test, with its measured cost; the family in part 5.b follows his pick.
  What decides it: side-by-side frames with legacy at the game camera. Research ids: R3-LOOK-OQ2 (lines 1326–1327);
  the decision map's gap G8.
- **OQ-0003-3. Which colour output: authored colour as shown, or a tone-mapped output?**
  Why it matters: LOOK_V2 §2 keeps "vibrant, beautiful colours"; legacy's rule was "authored hex = on-screen colour".
  Options: no tone mapping, colours as authored (legacy's rule, CD15); the render pipeline's default tone mapping and
  colour-space conversion (R3-WGPU-F12, lines 133–137); both are one switch (`outputColorTransform`). Decider:
  Mustafa, in the look proof test, side by side with legacy. What decides it: his judgement; how bright glow values
  look without tone mapping is a test mark. Research ids: R3-WGPU-OQ4 (lines 1296–1298).
- **OQ-0003-4. Is one sun shadow map enough, or are cascades or another setup needed?**
  Why it matters: LOOK_V2 §2 keeps soft shadows, inside the shadow row (≤ 3 ms at Low, ≤ 2 ms at High).
  Options: one map following the player, as budgeted (part 5.f); cascades or other WebGPU shadow setups (not
  researched). Decider: the look proof test (softness, Mustafa) and the render budget bench (the shadow row).
  What decides it: a reading of three.js's WebGPU shadow options first, then both spikes. Research ids:
  R3-BUDGET-OQ6 (lines 1375–1378); the decision map's gap G7.
- **OQ-0003-5. How are "warm sun with cool shadows" and moods per map lit?**
  Why it matters: LOOK_V2 §6.5; the toon material's ramp is single-channel, so shade colour comes from the lights
  (R3-LOOK-F2, lines 323–329). Options: the node toon material with coloured sun and fill lights from the map's mood
  data (part 5.g); a custom TSL lighting model (Options Q3, line 809). Not researched. Decider: Mustafa, in the look
  proof test, with each option's GPU time. What decides it: the same scene at day and at dusk, beside legacy.
  Research ids: none (the decision map's gap G7).
- **OQ-0003-6. How does the roof fade render, and how is the ink masked on it?**
  Why it matters: LOOK_V2 §4's roofs fade (`[DEFAULT]`); a blend-state change can create a pipeline on first use
  (r3-rendering.md, Findings Q1, legacy facts, lines 201–202). Options: a transparent blend variant compiled in the
  warm-up (part 3); a fade computed in the shader (the inventory's GW17 note, `LEGACY_INVENTORY.md` line 205). Not
  researched. Decider: the look proof test. What decides it: no frame above 1.5 × the median when a roof fades
  (R3-WGPU-REC3's mark, lines 937–939), ink hidden on faded parts, and Mustafa's look. Research ids: none of R3's
  (the decision map's gap G3).
- **OQ-0003-7. Which technique draws fragments, muzzle flashes, tracers, impacts and debris?**
  Why it matters: LOOK_V2 §3 brings fragmentation back and §7 asks for punchy weapons, inside the visual-effects row
  (≤ 2 ms at Low, ≤ 1 ms at High). Options: pooled meshes and sprites updated on the CPU (legacy GP1, F-A);
  instanced sprites or meshes moved in the vertex stage (F-A); GPU compute particles on WebGPU only (F-B, with a
  fallback). Not researched. Decider: the look proof test (look) and the render budget bench (cost). What decides
  it: the visual-effects row in the stress scene's fight on screen. Research ids: none (the decision map's gap G5).
- **OQ-0003-8. How is ink line boil drawn?**
  Why it matters: LOOK_V2 §7's comic touches (`[DEFAULT]`). Options: none researched; a short reading of
  techniques proposes them before the look proof test. Decider: Mustafa, in the look proof test. What decides it:
  one big moment with line boil on, and its cost against the post-processing row. Research ids: none (the
  decision map's gap G2).
- **OQ-0003-9. How are vegetation and the lush countryside drawn, and in which tree style?**
  Why it matters: LOOK_V2 §1, §2 (logs and trees with variety) and §4 (fields, forests, villages); the tree style is
  LOOK_V2 §8's `[OPEN]` item. Options: instanced or batched props with per-instance variety (R3-CHAR-F4, F6), merged
  static vegetation by material (R3-CHAR-REC4); foliage itself was not researched. The tree style has no options
  here: it is Mustafa's. Decider: Mustafa (the style, a later design session); the render budget bench (the
  technique). What decides it: a lush test patch within the draw-call and triangle rows. Research ids: none disposed
  here (R3-CHAR-OQ4 is the render budget bench's; the decision map's gap G9).
- **OQ-0003-10. Does the frame rate follow a 120 or 144 Hz screen, or is it capped?**
  Why it matters: the render loop follows the display (R3-BUDGET-F2, lines 545–549); "a cap saves power on laptops"
  (R3-BUDGET-OQ2, lines 1366–1367). Options: cap at 60 fps on High; follow the display; a cap per tier as data.
  Decider: Mustafa (feel), on the render budget bench's numbers. What decides it: smoothness on 60, 120 and 144 Hz
  displays with each option (the boundary spike's smoothness mark, R1-TICK-REC5, r1-simulation-core.md lines
  2094–2116) and power on the minimum laptop. Research ids: R3-BUDGET-OQ2.
- **OQ-0003-11. Should v2 one day drop the WebGL 2 fallback, and how many players use it?**
  Why it matters: the fallback doubles the test matrix; dropping it shuts out Firefox on Linux and Intel Macs, Chrome
  on Linux without Intel Gen12+, and Safari before 26 (R3-GL2-F12, lines 279–283). Options: keep it with the same
  features (part 1); "WebGPU required" (not researched). Decider: Mustafa. What decides it: the fallback share from
  client field data after launch (part 8). Research ids: R3-GL2-OQ5 (lines 1317–1320), R3-GL2-OQ2 (lines
  1308–1311).
- **OQ-0003-12. Is the pixel ratio capped at 1 on Low for HiDPI laptops?**
  Why it matters: ratio 2 shades four times the pixels (R3-BUDGET-F3, lines 551–555), while a cap at 1 softens text
  in the 3D view. Options: cap at 1 on Low (the budget table); cap at 2. Decider: Mustafa, in the look proof test.
  What decides it: GPU time at ratio 1 and 2 on the minimum laptop (CD15's request) and his look. Research ids:
  R3-BUDGET-OQ3 (lines 1368–1369).
- **OQ-0003-13. Where does the audio listener sit under the v2 camera?**
  Why it matters: a listener on a high camera hears every source at nearly the same distance (R3-AUDIO-F11, lines
  753–757). Options: at the player, oriented with the camera (part 9); at the camera. Decider: the camera framing
  first (R2-INT-OQ2, a design question), then Mustafa's listening check. What decides it: the framing's height and
  angle. Research ids: R3-AUDIO-OQ3 (lines 1399–1400).
- **OQ-0003-14. Do the Web Audio specification's PannerNode and spatialization sections change part 9?**
  Why it matters: MDN stood in for the specification, whose fetched text stopped before those sections
  (R3-AUDIO-F9, lines 743–747). Options: a read of the specification's sections; keep MDN's pages. Decider: the
  verifier (plan step 7, VERIFY-FETCH (a)) or the render budget bench's preparation. What decides it: whether the
  specification's text differs from MDN on the panning and distance models. Research ids: R3-AUDIO-OQ4 (lines
  1401–1402).
- **OQ-0003-15. The legacy look touches LOOK_V2 does not name: keep, change or drop each?**
  Why it matters: they are on screen today, and LOOK_V2 §3 asks that nothing look like a children's game. The items:
  comic words (CL12, GW3), the kill vignette (GP9), the toy-soldier death fall (GA14) and the skin rim (CL3).
  Options per item: keep, change or drop. Decider: Mustafa, in the look proof test. What decides it: his judgement,
  side by side with legacy. Research ids: none (legacy rows only).

## What a spike must prove

Two spikes, throwaway and in `spikes/` (SPIKE-SCOPE (a)); none starts before Mustafa and the Advisor approve the
drafts. `docs/SPIKE_PLAN.md` gives their final names, order and sizes. Every candidate mark below is proposed in the
research (r3-rendering.md, line 46: "Every pass mark and every reference machine is proposed"), restated at N = 48,
and measured with the harness of part 8 on the two reference machines, never headless.

### The look proof test (LOOK_V2 §9; parts 1, 3, 4, 5 and 11)
- **Mustafa judges the look.** In Brave on his Windows PC, each LOOK_V2 item of the trace table above is shown side
  by side with a legacy screenshot at the same framing; the test-engineer's Brave checklist names each item with its
  expected result. Mustafa also makes the calls this record leaves to him: the ink family and its weights
  (OQ-0003-2), the colour output (OQ-0003-3), the lighting (OQ-0003-5), the roof fade's look (OQ-0003-6), line boil
  (OQ-0003-8), the pixel-ratio cap (OQ-0003-12) and the legacy touches (OQ-0003-15). The test also records which
  backend Brave runs on his machine (R3-WGPU-OQ1, lines 1286–1289).
- **Each technique alone (part 5):** a table with GPU ms (median and p95 over 600 frames) at 1920 × 1080 on the
  minimum and the recommended machine, extra draw calls and extra render-target memory, each measured with only that
  technique on (R3-LOOK-REC1, lines 998–1000).
- **Both ink families (5.b):** GPU and CPU frame time at 12, 24 and 48 soldiers on screen on the minimum laptop at Low
  (R3-LOOK-REC3, lines 1026–1027).
- **No unplanned re-render (5):** with every kept effect on, `renderer.info.render.drawCalls` stays within 1.1 × the
  draws with all effects off plus the planned extras; each effect's off and lowered settings each cut its own GPU time
  by at least 40 % (R3-LOOK-REC2, lines 1014–1017).
- **Both backends (1):** the benchmark scene renders on the WebGPU backend in Chrome, Edge, Firefox on Windows and
  Safari 26, and on the WebGL 2 backend (`forceWebGL`) in the same browsers, with zero console errors; the two
  backends' screenshots of the same frame differ in at most 1 % of pixels by more than 8/255 per channel (a proposed
  tolerance; effects on one backend only are off for this test); the WebGPU backend's median CPU frame time is no
  worse than 1.1 × the WebGL 2 backend's on the same machine (R3-WGPU-REC1, lines 910–915). The fallback shows the
  same features and meets the minimum budget on the minimum machine (R3-GL2-REC1, lines 965–967).
- **No first-sight stutter (3):** `renderer.info.memory.programs` is the same 5 s after deploy and at the end of a
  scripted 5-minute match, and the first appearance of each unit type, visual effect, damage stage and roof fade
  causes no frame above 1.5 × that run's median frame time (R3-WGPU-REC3, lines 937–939).
- **The backend reported (4):** on each browser the overlay and the test log name the backend correctly in 10 of 10
  starts (R3-WGPU-REC4, lines 950–951); the harness reports GPU time on Chrome's WebGL 2 backend and "GPU time not
  available", never 0, on Firefox's and Safari's (R3-GL2-REC3, lines 984–985).
- **Frame time beside the look:** frame time median and p95 with the kept set of techniques on, on both machines,
  against the budget table's frame-time rows (part 7); the full table is the render budget bench's.
- **Needs:** LOOK_V2; both reference machines (OQ-0003-1); Mustafa's time in Brave; a few authored test assets as
  plain glTF/GLB exports (part 11); legacy screenshots at the same framing.

### The render budget bench (parts 6, 7, 8 and 9)
- **Every budget row** holds on its machine in the benchmark scene, for 3 runs of 600 measured frames after 300
  warm-up frames, on Chrome on both machines and on Firefox on the minimum machine, on the WebGPU backend and, on the
  minimum machine, also on the WebGL 2 backend (R3-BUDGET-REC2, lines 1162–1164).
- **Characters (6):** with N = 48 soldiers on screen, C-B renders on both backends with at most 4 draw calls per
  soldier per pass (before the ink); posing and bone upload with every soldier on the full rig stay within 2.0 ms
  (p99) per frame on the minimum machine, and with the budget LOD at Low within 1.0 ms (p99); C-D or C-E is adopted
  only if it cuts total frame time by at least 20 % against C-B on both machines (R3-CHAR-REC1, lines 1044–1048). No
  soldier inside its padded sphere is ever culled while visible (600 frames rendered with culling off and on give
  identical pixels; R3-CHAR-REC2, lines 1061–1063). Zero bytes allocated per frame in the character update path over
  60 s, and health bars for 48 soldiers in at most 2 draw calls (R3-CHAR-REC3, lines 1074–1075).
- **The spectacle budget (6, 7):** the stress scene's four very large units on screen with its soldiers add at most
  2.0 ms (p99) GPU time and at most 200 draw calls, before any ink or AO pre-pass, on the recommended machine at High;
  render bundles stay only if they cut the WebGPU backend's CPU frame time (R3-CHAR-REC4, lines 1088–1092).
- **The fallback's worst case (1):** on Firefox's WebGL 2 backend, `drawCalls` stays within the Low budget and the
  median frame time within 33.3 ms on the minimum machine's Linux run (R3-GL2-REC2, lines 975–977).
- **Tiers (7):** switching from High to Low on the minimum machine cuts GPU frame time by at least 50 %, with no
  shader compile during the switch (R3-BUDGET-REC3, lines 1176–1177).
- **Measurement (8):** the harness's GPU total agrees with the CPU frame time minus CPU work within 2 ms (median) when
  the scene is GPU-bound, and two back-to-back runs give medians within 3 % (R3-MEASURE-REC1, lines 1191–1193); two
  runs of the benchmark scene give identical `drawCalls` and `triangles` on every frame and identical screenshots at
  three fixed frames (R3-MEASURE-REC2, lines 1203–1204); the CI job fails on a planted change of +10 % draw calls or
  +10 % GPU memory (1 of 1) and runs in under 5 minutes (R3-MEASURE-REC3, lines 1214–1215); the overlay costs at most
  1 % of median frame time closed and 3 % open (R3-MEASURE-REC4, lines 1224–1225).
- **The page's cost of the sim (N12):** the budget row is the p99 of the page's sim work per frame (≤ 1.0 ms,
  R3-BUDGET-REC2, line 1127); ADR 0002's boundary spike also reports R1-BND-REC1 mark 1's p95 frame-time difference
  (≤ 1.0 ms, r1-simulation-core.md lines 2259–2261). Both statistics are reported; the test-engineer's table says
  which one gates (decision map LE-B).
- **Audio (9):** with 32 concurrent equal-power voices, a 5-minute recorded capture has no dropout (checked by a
  listener and by the AudioContext staying "running") and main-thread audio work stays under 0.5 ms (p99) per frame on
  the minimum machine; HRTF for 4 priority voices raises that p99 by less than 0.3 ms (R3-AUDIO-REC1, lines
  1241–1243). In the benchmark fight the live source nodes never exceed the cap, shots at the player are never dropped
  (0 in 3 runs), and no AudioBuffer is created after warm-up (R3-AUDIO-REC2, lines 1254–1256). Only if O-B is
  proposed: at most 1 ray per voice per 100 ms and 0.2 ms (p99) per frame for all rays at the cap, and 4 of 5 testers
  tell an occluded source from an open one in a blind test (R3-AUDIO-REC3, lines 1267–1269). Sound onset trails the
  drawn event by at most one rendered frame plus the output latency over 100 shots, with zero autoplay warnings on the
  four browsers (R3-AUDIO-REC4, lines 1279–1280).
- **The machines (7):** both are named with GPU, CPU, RAM, screen and `devicePixelRatio`, every mark is reported on
  both, and on the minimum laptop's Linux run the overlay names the WebGL 2 backend in Firefox (R3-BUDGET-REC1, lines
  1114–1116).
- **Later, at each three.js upgrade (2):** a step from one pinned release to the next changes the benchmark scene's
  p95 GPU and CPU frame time by no more than 5 % without an explained cause (R3-WGPU-REC2, lines 925–926).
- **Needs:** the look proof test's kept techniques; ADR 0002's boundary spike block (per-instance data from it); a
  recorded match for the benchmark scene (ADR 0004's sim core spike, or a synthetic recording); the stress scene's
  counts (`docs/SPIKE_PLAN.md`).

### Pass marks
Written by test-engineer (plan step 4).

## Migration plan

The netcode proof test runs first, before the foundation (SLICE-ORDER, condition 3); it needs no part of this record.
Each phase ends on a milestone that proves it done; the legacy game stays playable in `legacy/` throughout (ADR 0002).

1. **Drafts approved** by Mustafa and the Advisor; the two reference machines named (OQ-0003-1).
2. **The look proof test**, in parallel with the netcode proof test (no shared dependency). *Milestone:* the
   technique table and both ink families measured on both machines; Mustafa's verdict on every LOOK_V2 item and on
   OQ-0003-2, -3, -5, -6, -8, -12 and -15, recorded in this ADR.
3. **The foundation** ([GLOSSARY §8](../research/GLOSSARY.md#the-foundation); ADR 0002). This record's share: the
   renderer starts with `await renderer.init()`, reports its backend in the overlay and in every test screenshot, and
   CI gates the render counts (W-A). *Milestone:* an empty scene runs through the full stack, CI green, and the
   preview link names its backend.
4. **The render budget bench**, after the look proof test and ADR 0002's boundary spike, before slice 1's renderer
   work. *Milestone:* every row of the budget table on both machines, or a recorded change of tier for each technique
   that misses.
5. **Slice 1, move & shoot:** one skinned soldier, the toon materials and ink kept in step 2, the feel kit's visual
   effects and voices for the slice's weapons, split feedback. *Milestone:* programs constant over a match, zero
   per-frame allocation in the render path, Mustafa's Brave check.
6. **Slice 2, bots and three teams, played locally:** up to N = 48 drawn from the block, the budget LOD, instanced
   health bars, the three-team HUD. *Milestone:* the bench's character marks hold at the slice's N on both machines.
7. **Slice 3, online:** remote soldiers drawn from ADR 0001's interpolation and snapshot fields. *Milestone:* 0
   rendered frames with an entity in view whose state the client lacks (ADR 0001's pop-in mark, R2-INT-REC1 (a),
   r2-netcode.md lines 2437–2439), drawn through this renderer.
8. **Slice 4, the loop:** the glow language on objectives, harvesters as big vehicles, the upgrade track display.
   *Milestone:* Mustafa's Brave check.
9. **Slice 5, Octopon-3 and Editor v2:** buildings with interiors and floors, the roof fade, damage stages, moods per
   map, the lush countryside; ADR 0005's asset spike before the first authored model enters the game. *Milestone:* the
   look proof test's building, roof-fade and damage-stage items pass in the game.
10. **Slice 6, scale to 48:** the render budget bench re-run on the real game in the stress scene's fight on screen;
    client field data switched on. *Milestone:* every budget row holds on both machines.

## Test plan

- **Renders:** screenshots at fixed frames of the benchmark scene on both backends before and after every visual
  change; LOOK_V2 items side by side with legacy screenshots; Mustafa's Brave check is final.
- **CI (W-A, headless):** zero console errors; draw calls, triangles, programs and GPU memory in the benchmark scene
  within their rows; zero per-frame allocation in the render path; a planted regression is caught. No frame time from
  CI is ever reported as a hardware result.
- **Reference machines (W-C):** each mark from the harness, 3 runs of 600 frames after 300 warm-up frames, in a results
  log with backend, adapter, browser, `devicePixelRatio` and resolution.
- **Backend matrix:** Chrome, Edge, Firefox (Windows) and Safari 26 on WebGPU; the same with WebGL 2 forced; Firefox on
  Linux for the real fallback; Brave on Mustafa's PC.
- **Determinism (ADR 0004's):** the benchmark scene replays a recorded match; identical counts on every frame and
  identical screenshots at three frames; if client replay is not identical, a fixed-seed bot match stands in.
- **Soak:** JS heap at most 150 MB after 10 minutes, no growth over the last 5; programs constant over a 5-minute
  match.
- **Network (where it applies):** remote entities drawn under ADR 0001's network profiles (P2: 100 ms round trip,
  ±20 ms jitter, 2 % loss: r2-netcode.md, Open questions, R2-XU-OQ12, lines 3220–3225), checked for pop-in frames with ADR 0001's harness.
- **Audio:** the voice counter in the benchmark fight; onset latency over 100 shots; a blind listening test only if
  O-B is proposed.
- **Playtests:** Mustafa's Brave check at every slice that changes what a player sees or hears.

## Rollback

- **The WebGPU backend misses its marks** (for example R3-WGPU-REC1 mark 3): run the same TSL code on the WebGL 2
  backend everywhere (`forceWebGL`), which stays within the settled renderer. If WebGPURenderer itself fails, the
  renderer is a settled input: the question goes to Mustafa and the Advisor; option C is not taken silently.
- **The screen-space ink fails** (its cost, aliasing or Mustafa's look): switch to the hull family, already measured;
  the Low cap becomes about 1,100 draws or the hull a High-only setting.
- **Single-pass AO fails:** the documented pre-pass on High only, AO off on Low.
- **C-B misses the character marks:** tighten the budget LOD first; then the instanced candidate that measured best
  (C-E on both backends, or C-D on WebGPU with C-B as its fallback).
- **A three.js upgrade regresses:** stay on the previous pin (V1).
- **HRTF or occlusion costs too much:** equal power only; zones or none.
- **Any technique misses on Low:** a tier change in data moves it to High only, with no code change.
