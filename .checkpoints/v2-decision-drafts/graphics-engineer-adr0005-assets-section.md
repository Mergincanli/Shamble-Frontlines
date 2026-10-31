## The asset pipeline: GPU rules (graphics-engineer)

*Written by graphics-engineer for ADR 0005, 2026-10-31; engine-architect places it verbatim. Lines are at 9e7db28;
r3 = `docs/research/r3-rendering.md`. Rendering itself (how the look is drawn) is ADR 0003's; this section says only
what an imported model must carry and what the GPU budget allows it.*

LOOK_V2 §8 gives the flow as a default: a concept sketch, a model in Spline, a glTF/GLB export, "an automated step
that compresses it, builds LODs and renames materials into the faction colour slots", and "the engine applies the
comic look"; budgets per unit (triangles, materials, textures); never Spline's own runtime. This section takes that
flow as given and proposes only what the research supports: R3-CHAR (skinning, instancing, LOD) and R3-BUDGET (the
frame totals). The research has no findings on compression, LOD generation, export checks or slot renaming: its one
glTF mention is three.js's own example, which loads a glTF skinned mesh into the WebGPU renderer (r3, Findings Q4,
R3-CHAR-F8, lines 476–480). Everything else is an open question marked "not researched", grouped under one research
unit, the **asset-pipeline research** (graphics-engineer, verified like R3, in its own daytime session), which
`docs/SPIKE_PLAN.md` places before the asset spike (ASSET-RESEARCH (a), the advisor's ruling of 2026-10-31).

### What this section decides
1. **An imported model carries no look; the engine applies it.** A model brings geometry, its skin (joints and
   weights) where it moves, UVs and textures where it has them, and named material slots. It brings no baked lighting
   and no ink. At load the engine gives each slot the shared node toon material of that slot's material class (ADR
   0003, part 5.a), one material per slot content and never one per mesh. *Why:* each distinct ramp is a distinct
   material, so materials must be shared by content or batching breaks (r3, Findings Q3, R3-LOOK-F3, lines 331–334);
   legacy converted per mesh and got one material per mesh, which defeated its first merge attempt (CL4,
   `docs/LEGACY_INVENTORY.md` line 113); and a model's own shader code could not run anyway, since custom GLSL and
   `onBeforeCompile` are "not supported in WebGPURenderer" (r3, Findings Q1, R3-WGPU-F6, lines 97–102). *Need:*
   LOOK_V2 §8 (step 5) only.
2. **Material slots name a material class.** Every slot's name maps to one class from a small fixed list (for
   example environment, unit, faction colour, emissive); the list and the names are this ADR's content data (decision
   map I-8), consumed by ADR 0003. Faction colour slots take the faction's colour at load, as one shared material per
   slot and faction, or as per-instance colour where a mesh is instanced (`instanceColor` on `InstancedMesh` and
   `BatchedMesh`: r3, Findings Q4, R3-CHAR-F4, F6, lines 452–467; R3-CHAR-REC3, Recommendations Q4, lines
   1065–1075). Emissive slots write the emissive output that drives the glow, so only they glow (r3, Findings Q3,
   R3-LOOK-F12, lines 392–395; LOOK_V2 §6.4). *Need:* VISION_V2 §6.7 (faction uniforms and variants as data);
   LOOK_V2 §2 (uniform details), §6.4.
3. **Soldiers are skinned; very large units are part groups.** A soldier model is one skinned mesh with skin
   indices and weights as attributes ("requires skin indices and weights as buffer attributes in its geometry": r3,
   Findings Q4, R3-CHAR-F1, lines 436–439) and any weights the artist paints (option C-B, R3-CHAR-REC1, lines
   1032–1048). A very large unit's parts are separate meshes under one group, each part named to match the sim's
   vehicle-part records (option V-A, R3-CHAR-REC4, lines 1077–1092; the records are ADR 0004's). *Need:* VISION_V2
   §6.4, §6.9; LOOK_V2 §3, §5.
4. **The derived per-unit caps.** Two caps follow from ADR 0003's totals by arithmetic; the rest are open (OQ-0005-AS5):
   - *A soldier as drawn* (body, headgear and held weapon) uses at most 4 materials per pass: option C-B draws "one
     per material per soldier" (r3, Options Q4, line 825) and ADR 0003 caps a soldier at 4 draw calls per pass
     before the ink (R3-CHAR-REC1, lines 1044–1045).
   - *A very large unit* draws at most 25 parts × materials per pass: ADR 0003 allows the four very large units 200
     draw calls over the shadow and the main pass (R3-CHAR-REC4, lines 1088–1090), and 200 ÷ 4 units ÷ 2 passes = 25.
     R3's own test load, 20 parts each with one material, fits (4 × 20 × 2 = 160, lines 1089–1090).
   - The frame totals every asset shares stay ADR 0003's (r3, R3-BUDGET-REC2, lines 1121–1137): at Low, ≤ 800 draw
     calls, ≤ 500k triangles, ≤ 512 MB of GPU memory; at High, ≤ 1,500, ≤ 1.5M, ≤ 1 GB.
   *Need:* VISION_V2 §4, §6.9; LOOK_V2 §8 (budgets per unit).
5. **LOD levels are optional data; switching is the engine's.** A model may carry mesh LOD levels. The engine
   switches them through three.js's `LOD`, which switches "at the distances specified" with a `hysteresis` "to avoid
   flickering" (r3, Findings Q4, R3-CHAR-F10, lines 490–493), only where the camera's framing spreads units over a
   wide range of distances (R3-CHAR-REC2, lines 1050–1063); the framing is a design question LOOK_V2 does not settle
   (r3, Open questions Q4, R3-CHAR-OQ5, lines 1355–1356). Animation detail is never in the asset: the engine's budget
   LOD decides it (R3-CHAR-REC2). How LOD levels are built is open (OQ-0005-AS2). *Need:* LOOK_V2 §5 (readable from
   a distance, depth up close).
6. **Only glTF/GLB, never Spline's runtime.** The game loads models as glTF/GLB files through three.js's loader on
   both backends; it never loads Spline's own runtime (LOOK_V2 §8). *Need:* LOOK_V2 §8 only.
7. **Materials and textures are shared, cached and compiled up front.** One material per class (and per faction for
   faction slots) is created once, cached and reused, "never rebuilt per edit" (EDITOR_V2 §2, rule 3), and never
   disposed while shared; every one is compiled in ADR 0003's warm-up, so a unit's first appearance creates no shader
   program (r3, Recommendations Q1, R3-WGPU-REC3, lines 928–939). *Need:* VISION_V2 §6.9; EDITOR_V2 §2.
8. **The look proof test's models.** ADR 0003's look proof test needs a few authored models (a soldier, a weapon, a
   building with an interior, a very large unit, props). They are plain glTF/GLB exports; the test decides nothing
   about compression or LODs (ASSET-RESEARCH (a)).
9. **Trees wait.** The tree style is LOOK_V2 §8's `[OPEN]` item; tree assets wait for Mustafa's answer (ADR 0003,
   OQ-0003-9). Nothing here decides it.

### Options
"Presentation only" means the option changes nothing the authority decides. Options come from R3's Options section
and the legacy facts it cites; an option the research lacks is marked "not researched".

**How an imported model gets the look**
| Option | Complexity | Cost (money, time, usage) | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort from the current code |
|---|---|---|---|---|---|---|
| **One shared toon material per slot class, applied at load** (proposed, part 1) | Low | one material per class and faction | materials stay few, so sharing and merging work (R3-LOOK-F3) | presentation only | one look on every asset, set in one place | new; legacy's per-mesh conversion is the lesson (CL4) |
| Convert each mesh's material at load (legacy's `toonify`) | Low | one material per mesh | defeats batching and merging (CL4) | presentation only | the same look | ports, then fails the draw-call budget |
| A look baked into each asset by the modelling tool | not researched | – | – | – | – | – |

- **Shared per class.** Pros: bounded materials; one place to tune the look. Cons: every slot must be named into a
  class (OQ-0005-AS4). **Per mesh.** Pros: no naming rule. Cons: one material per mesh, legacy's measured problem.

**How a very large unit is built** (r3, Options Q4, lines 834–836)
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **V-A. Parts as separate meshes in one group** (proposed, part 3) | Low | more draws than V-B | "a few" units absorb it; parts stay destructible and pickable | presentation only | parts move as the sim's parts | the Juggernaut is already multi-part (GC16, `docs/LEGACY_INVENTORY.md` line 255) |
| V-B. One rigid-skinned mesh per unit | Medium | halves draws | – | presentation only | parts cannot break off on their own | new |

- **V-A.** Pros: maps one-to-one onto the sim's vehicle-part records. Cons: more draws. **V-B.** Pros: fewer draws.
  Cons: loses per-part destruction and picking.

**How faction colour reaches a soldier**
| Option | Complexity | Cost | Scalability / performance | Security & anti-cheat | Effect on the look and feel | Effort |
|---|---|---|---|---|---|---|
| **One shared material per slot and faction** (with option C-B, proposed) | Low | one material per faction per slot | one draw per material per soldier (Options Q4, line 825) | presentation only | colour per faction, uniform across a team | new |
| Per-instance colour (`instanceColor`, R3-CHAR-F4, F6) | Low | none per faction | only with instanced meshes (C-D, C-E: measured against C-B in ADR 0003) | presentation only | the same | new |

- The choice follows ADR 0003's skinning option: C-B uses shared materials per faction; an instanced candidate that
  wins its mark uses per-instance colour.

**LOD** (r3, Options Q4, lines 831–832): L-1, distance LOD from the asset's levels, adopted only where the framing
spreads distances; L-2, the engine's budget LOD for animation, always (ADR 0003, part 6). Compression, the LOD tool,
export checks and the slot-renaming step have no options in the research: they are the open questions below.

### Open questions
All six are **not researched**. AS1 to AS5 form one research unit, the asset-pipeline research (graphics-engineer,
verified like R3, its own daytime session), which `docs/SPIKE_PLAN.md` places before the asset spike; the asset spike
then proves its answers.

- **OQ-0005-AS1. Which compression formats for textures and meshes?**
  Why it matters: download size and GPU memory against ADR 0003's ≤ 512 MB at Low (r3, R3-BUDGET-REC2, line 1135).
  Options: none researched (no finding in R1–R6). Decider: the asset-pipeline research, then the asset spike. What
  decides it: support on both backends and in the four browsers, decode cost, memory. Research ids: none.
- **OQ-0005-AS2. Which tool builds the LOD levels, and how many levels?**
  Why it matters: LOOK_V2 §5's "readable from a distance, with real depth up close"; part 5 switches levels the asset
  carries. Options: none researched (only animation LOD is researched, R3-CHAR-REC2). Decider: the asset-pipeline
  research, then the asset spike and Mustafa (silhouettes kept). What decides it: the levels keep the silhouette at
  the farthest framing, within the triangle row. Research ids: none.
- **OQ-0005-AS3. Which checks does an export pass before it enters the game?**
  Why it matters: a model with unnamed slots, the wrong scale or a broken skin fails silently on screen, as legacy's
  unchecked tags did (CL14, `docs/LEGACY_INVENTORY.md` line 123). Options: none researched. Decider: the
  asset-pipeline research. What decides it: the failures the asset spike can plant and catch. Research ids: none.
- **OQ-0005-AS4. How are a model's materials renamed into the slot classes and faction colour slots?**
  Why it matters: part 1 and part 2 depend on every slot carrying a class. Options: none researched (a manual
  convention in the modelling tool, or LOOK_V2 §8's automated step). Decider: the asset-pipeline research. What
  decides it: whether Spline's glTF export keeps material names (not researched). Research ids: none.
- **OQ-0005-AS5. What are the per-unit budgets beyond the two derived caps?**
  Why it matters: LOOK_V2 §8 asks for triangles, materials and textures per unit; only the soldier's 4 materials per
  pass and a very large unit's 25 draws per pass follow from the research (part 4). Options: split ADR 0003's frame
  totals across units, the world and visual effects (not researched). Decider: the asset-pipeline research, then the
  render budget bench with real models. What decides it: the totals still hold in the stress scene's fight on screen
  with the real models. Research ids: none (R3-BUDGET-OQ5, the triangle and memory rows' looseness, is ADR 0003's).
- **OQ-0005-AS6. How many vertices and bones does the authored soldier have?**
  Why it matters: the triangle row and the bone upload in the animation row (≤ 1.0 ms at Low) depend on them.
  Options: none until the model exists ("vertex and bone counts per soldier are unknown until it exists", r3, Open
  questions Q4, R3-CHAR-OQ3, lines 1349–1351). Decider: the character model's author (LOOK_V2 §3, §8), then the render
  budget bench. What decides it: the bench's character marks with the real model. Research ids: R3-CHAR-OQ3.

### What a spike must prove (this section's part)
- **In ADR 0003's look proof test** (plain glTF/GLB exports, no compression or LODs): every test model loads on both
  backends with zero console errors and is drawn with the engine's toon materials per slot, with no material of its
  own left in use; the scene holds no more material objects than the distinct (slot class, faction) pairs it shows
  (proposed mark; reason: sharing by content, R3-LOOK-F3); the test soldier as drawn meets 4 draw calls per pass and
  the test very large unit 25 (part 4's arithmetic, on R3-CHAR-REC1 and R3-CHAR-REC4).
- **In the asset spike**, after the asset-pipeline research: the pipeline's answers to OQ-0005-AS1 to AS5 (its marks
  come from that research and are not set here); the loader on both backends; every slot classed and every faction
  slot coloured; LOD levels that keep the silhouette, judged by Mustafa; and the stress scene's fight on screen built
  from real models within ADR 0003's frame totals.

### Appendix A rows (for the frame)
| id | disposition | where or why |
|---|---|---|
| R3-CHAR-OQ3 | open: OQ-0005-AS6 | the authored soldier (LOOK_V2 §3, §8) sets the vertex and bone counts; the render budget bench measures them against ADR 0003's rows |
