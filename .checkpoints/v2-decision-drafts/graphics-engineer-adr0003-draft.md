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
