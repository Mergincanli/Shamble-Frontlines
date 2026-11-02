# graphics-engineer: the final fix round (ADR 0003 except "### Pass marks"; ADR 0005's GPU rules)

Prompt: prompts/final-graphics-engineer.txt. BEFORE blocks are whole lines copied by script from the docs at
47eadd9 (each occurs exactly once; no two overlap); apply in any order. Notes:
graphics-engineer-final-notes.md.

### FIX AD-1, GE-4 (part 5: the look floor on the tier switch)
FILE: docs/adr/0003-rendering.md
BEFORE:
   read buffers never re-render the scene; a second geometry pass is a planned, counted extra; every effect has a
   quality-tier switch (off, or a lower resolution scale) from the start (R3-LOOK-REC2, Recommendations Q3, lines
   1002–1017). Every technique below is measured alone in the look proof test: GPU time per pass and memory per
   target, on both reference machines and both backends (R3-LOOK-REC1, lines 991–1000, now serving LOOK_V2's items).
AFTER:
   read buffers never re-render the scene; a second geometry pass is a planned, counted extra; every effect has a
   quality-tier switch (off, or a lower resolution scale) from the start (R3-LOOK-REC2, Recommendations Q3, lines
   1002–1017). The "off" setting has a floor: "Which effects exist at all is the look brief's call; the tier table
   only says how much each may cost" (R3-BUDGET-REC3, Recommendations Q5, lines 1172–1173). So no tier switches off
   a technique that carries a LOOK_V2 `[DECIDED]` item (the toon materials, 5.a; the ink, 5.b; the sun shadow map,
   5.f; the emissive slots' colour, 5.d) unless Mustafa answers OQ-0003-16 that way; until he does, a tier may only
   lower it (resolution scale, map size, samples). Every technique below is measured alone in the look proof test:
   GPU time per pass and memory per target, on both reference machines and both backends (R3-LOOK-REC1, lines
   991–1000, now serving LOOK_V2's items).
END

### FIX GE-1 (5.a: one material per slot content, the content defined)
FILE: docs/adr/0003-rendering.md
BEFORE:
   - **a. Toon materials, applied by the engine.** At load, each material slot of an asset (slot names are ADR 0005's,
     decision map I-8) gets the shared node toon material of its material class (`MeshToonNodeMaterial`, R3-LOOK-F1,
     lines 318–321) with that class's ramp; one material per slot content, never one per mesh. *Why:* the ramp is one
     texture lookup inside lighting, no extra pass, on both backends; each distinct ramp is a distinct material, so
     materials are shared by content (R3-LOOK-F2, F3, lines 323–334; legacy CL4's lesson). The toon model reads only
AFTER:
   - **a. Toon materials, applied by the engine.** At load, each material slot of an asset (slot names are ADR 0005's,
     decision map I-8) gets a shared node toon material of its material class (`MeshToonNodeMaterial`, R3-LOOK-F1,
     lines 318–321) with that class's ramp; one material per slot content, never one per mesh. A slot's content is
     its class (ramp, colour gain, ink weight: 5.b, 5.h), its faction for faction colour slots, and the slot's own
     base and emissive colour, textures, side and alpha mode, as legacy keys its shared materials by content (CL4,
     `LEGACY_INVENTORY.md` line 113; `_matSig`, `shamble-core.js` lines 356–359). Two slots share a material only
     when all of these match, so an asset keeps its own colours and textures inside a class; how colour is authored
     (material colour, vertex colour or an atlas) is ADR 0005's OQ-0005-AS4, not a rule here. *Why:* the ramp is one
     texture lookup inside lighting, no extra pass, on both backends; each distinct ramp is a distinct material, so
     materials are shared by content (R3-LOOK-F2, F3, lines 323–334; legacy CL4's lesson). The toon model reads only
END

### FIX AD-4, AD-6, AD-11, AD-1, PA-8 (5.b: the contact line, the structure class, the width across tiers, the hull at Low)
FILE: docs/adr/0003-rendering.md
BEFORE:
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
     mark folds inside a shape (inferred from R3-LOOK-F7; OQ-0003-2), and must be masked where the roof fade hides structures (r3-rendering.md, Options Q3,
     lines 810–813). If he picks the hull, the Low cap becomes about 1,100 (proposed) or the hull a High-only setting
     (R3-BUDGET-REC2, lines 1146–1148). Its known failures: threshold tuning and thin lines that alias (line 813).
AFTER:
   - **b. The ink: screen-space edges, proposed.** Edges from depth and normals, a custom TSL node over the shared
     pass's MRT outputs (R3-LOOK-F7, lines 360–364): depth edges carry the silhouette ink, normal edges the inner
     lines, so LOOK_V2 §6.2's two weights can map onto two edge kinds (inferred from F7; inner lines are not
     researched: OQ-0003-2). The weight per material class (buildings heaviest, LOOK_V2 §2, through the structure
     class, 5.h) needs a per-pixel class value in an MRT output (not researched: OQ-0003-2). The width is set in CSS
     pixels and scaled by the pixel ratio × the scene resolution scale, so a tier changes the ink's cost, never its
     weight on screen (proposed: the weight is Mustafa's pick, OQ-0003-2, and must not move with the tier). *Why this
     family, on cost and robustness:* its cost is one full-screen pass whatever the number of soldiers (R3-LOOK-F8,
     lines 366–370), it shares the normal output with the ambient occlusion (5.c), and it keeps the Low draw-call cap
     at 800 at N = 48 (tightly: part 7's arithmetic), where the hull family needs about 1,100 (R3-BUDGET-REC2's
     arithmetic, lines 1142–1148). *The measured alternative:* the inverted hull, legacy's family (three.js's
     `toonOutlinePass`, R3-LOOK-F4, F5, lines 337–350, or our own TSL hull in world units with legacy's ground
     contact line, CL6, `LEGACY_INVENTORY.md` line 115). The look proof test measures both families at 12, 24 and 48
     soldiers on screen on the minimum laptop at Low (R3-LOOK-REC3, lines 1019–1028). *The visible difference is
     Mustafa's call:* the hull draws a line of even width around each object, with legacy's ink contact line where
     objects meet the ground (CL6); the screen-space pass draws a line in pixels from depth and normal changes, can
     mark folds inside a shape (inferred from R3-LOOK-F7; OQ-0003-2), and must be masked where the roof fade hides
     structures (r3-rendering.md, Options Q3, lines 810–813). Where a wall or a pair of boots meets the ground, the
     depth runs on and only the surface direction turns, so this pass draws that contact line at the thin inner
     weight, and the base of a building gets a thinner line than its silhouette (an inference from F7's two edge
     kinds, not research); an edge where the per-pixel class value changes, structure against ground, could draw it
     at the silhouette weight (not researched: OQ-0003-2). If he picks the hull, the Low cap becomes about 1,100
     (proposed) or the hull a High-only setting (R3-BUDGET-REC2, lines 1146–1148), and what Low draws then is his
     call too (OQ-0003-16). Its known failures: threshold tuning and thin lines that alias (line 813), more so at
     Low's lower render resolution (inferred).
END

### FIX GE-4, AD-1 (5.c: AO off on Low is asked, not assumed)
FILE: docs/adr/0003-rendering.md
BEFORE:
     whose extra draws are counted (R3-LOOK-REC2). Low may switch AO off (tier data, part 7).
AFTER:
     whose extra draws are counted (R3-LOOK-REC2). Low may switch AO off (tier data, part 7): AO is LOOK_V2 §6.5's
     `[DEFAULT]`, but the trace table also uses it for §2's `[DECIDED]` "geometry that reads clearly", so OQ-0003-16
     asks Mustafa whether that switch holds.
END

### FIX PA-10 (5.d: the emissive attachment with MSAA)
FILE: docs/adr/0003-rendering.md
BEFORE:
     line 817), against LOOK_V2 §6.4's list and §6.1's value hierarchy. *Cost:* twelve full-screen passes at falling
     resolution (R3-LOOK-F13, lines 397–402) plus one emissive attachment, 16.6 MB at 1920 × 1080 in half float
     (R3-LOOK-F14, lines 405–409).
AFTER:
     line 817), against LOOK_V2 §6.4's list and §6.1's value hierarchy. *Cost:* twelve full-screen passes at falling
     resolution (R3-LOOK-F13, lines 397–402) plus one emissive attachment: 16.6 MB at 1920 × 1080 in half float
     single-sampled, and with 5.n's 4× MSAA 66.4 MB multisampled (4 × 16.6) plus a 16.6 MB single-sample resolve that
     the bloom reads (R3-LOOK-F14, lines 405–409: 4 × MSAA "multiplies the multisampled attachments by four"; the
     resolve is inferred).
END

### FIX AD-1, GE-4 (5.f: soft shadows stay on Low until Mustafa says otherwise)
FILE: docs/adr/0003-rendering.md
BEFORE:
   - **f. Soft shadows.** One sun shadow map, as R3-BUDGET-REC2's shadow row budgets it (Low: one map, 1024² or off,
     ≤ 3 ms p99; High: ≤ 2 ms; line 1129), using `PCFShadowMap`, which r186 made soft (R3-WGPU-F3, lines 78–83), with legacy's
     texel-snapped frustum that follows the player (GW4; `LEGACY_INVENTORY.md` line 192). Cascades and other setups
     were not researched (R3-BUDGET-OQ6, lines 1375–1378): OQ-0003-4.
AFTER:
   - **f. Soft shadows.** One sun shadow map, as R3-BUDGET-REC2's shadow row budgets it (Low: one map, 1024² or off,
     ≤ 3 ms p99; High: ≤ 2 ms; line 1129), using `PCFShadowMap`, which r186 made soft (R3-WGPU-F3, lines 78–83), with legacy's
     texel-snapped frustum that follows the player (GW4; `LEGACY_INVENTORY.md` line 192). Cascades and other setups
     were not researched (R3-BUDGET-OQ6, lines 1375–1378): OQ-0003-4. Soft shadows are LOOK_V2 §2 `[DECIDED]`, so the
     row's "or off" is used only if Mustafa answers OQ-0003-16 that way; until then Low keeps one 1024² map, measured
     against the row's 3 ms.
END

### FIX AD-6 (5.h: a structure class carries the buildings' ink weight)
FILE: docs/adr/0003-rendering.md
BEFORE:
   - **h. The value hierarchy.** Each material class (environment, units, faction colours, visual effects) carries a
     colour gain held as tuning data, so the environment can sit one step calmer (LOOK_V2 §6.1); legacy's nature gain
AFTER:
   - **h. The value hierarchy.** Each material class (environment, structures, units, faction colours, visual
     effects) carries a colour gain and an ink weight held as tuning data, so the environment can sit one step calmer
     (LOOK_V2 §6.1) and buildings, as the structure class, carry the heaviest ink (LOOK_V2 §2; 5.b); legacy's nature gain
END

### FIX AD-10, REDUCED-MOTION (5.l: the overlay's cost and the HUD)
FILE: docs/adr/0003-rendering.md
BEFORE:
   - **l. Comic impact touches, sparingly.** Halftone flashes as legacy's DOM overlay (CD16: a CSS multiply layer,
     "not a render pass", `LEGACY_INVENTORY.md` line 97), triggered by big-moment events; ink line boil's technique
     was not researched (OQ-0003-8). Both are built as defaults and costed in the look proof test against the
     post-processing row. *Need:* LOOK_V2 §7 only.
AFTER:
   - **l. Comic impact touches, sparingly.** Halftone flashes as legacy's DOM overlay (CD16: a CSS multiply layer,
     "not a render pass", `LEGACY_INVENTORY.md` line 97), triggered by big-moment events; ink line boil's technique
     was not researched (OQ-0003-8). Both are built as defaults and costed in the look proof test: line boil against
     the post-processing row, the overlay as frame time with it and without it, since the browser composites a CSS
     layer outside the renderer's timestamp queries (inferred from CD16's "not a render pass"). The overlay also dots
     the HUD ("a screen-space CSS overlay also dots the HUD/UI", `docs/ART_DIRECTION.md` line 121), so the look
     checklist shows it over the HUD. Whether the flashes, line boil and the concussion (5.m) soften for players who
     ask for reduced motion is OQ-0003-18. *Need:* LOOK_V2 §7 only.
END

### FIX PA-10 (5.n: render-target memory with MSAA)
FILE: docs/adr/0003-rendering.md
BEFORE:
     default", R3-WGPU-F15, lines 150–152), which multiplies the multisampled MRT attachments' memory (R3-LOOK-F14, lines 405–409).
AFTER:
     default", R3-WGPU-F15, lines 150–152), which multiplies the multisampled MRT attachments' memory by four
     (R3-LOOK-F14, lines 405–409): colour, normals and emissive in half float come to 3 × 66.4 = 199.2 MB multisampled
     at 1920 × 1080 before their resolves, depth, bloom, AO and the shadow map (arithmetic; each attachment's real
     format is unmeasured, F14), and about 112 MB at Low's scale of 0.75 if Low keeps 4× (199.2 × 0.75²), against
     ≤ 512 MB at Low and ≤ 1 GB at High (part 7). Whether `renderer.info.memory.total` counts the multisampled
     textures is not researched; the look proof test records it.
END

### FIX EA-11 (engine-architect #14: per-instance data from the interpolated blocks), GE-1 (part 6's faction material)
FILE: docs/adr/0003-rendering.md
BEFORE:
   - Per-instance data (transform, faction colour, hit flash, health bar) is written from the per-tick block into
     preallocated typed arrays, never by traversing scene objects; health bars and flashes are instanced or
     per-instance attributes (R3-CHAR-REC3, lines 1065–1075). Faction colour comes from the faction colour slots
     (ADR 0005, I-8): one shared material per slot and faction, or per-instance colour where meshes are instanced
AFTER:
   - Per-instance data (transform, faction colour, hit flash, health bar) is written from the blocks, interpolated
     each frame (part 10), into preallocated typed arrays, never by traversing scene objects; health bars and
     flashes are instanced or per-instance attributes (R3-CHAR-REC3, lines 1065–1075). Faction colour comes from the
     faction colour slots (ADR 0005, I-8): one shared material per slot content, the faction included (5.a), or
     per-instance colour where meshes are instanced
END

### FIX AD-1, GE-4 (the budget table's shadow row)
FILE: docs/adr/0003-rendering.md
BEFORE:
     | of which: shadow pass | ≤ 3 ms (one map, 1024² or off) | ≤ 2 ms |
AFTER:
     | of which: shadow pass | ≤ 3 ms (one map, 1024²; the research's "or off" only on Mustafa's answer to OQ-0003-16) | ≤ 2 ms |
END

### FIX PA-8 (part 7: the Low draw-call sum counts the whole frame)
FILE: docs/adr/0003-rendering.md
BEFORE:
     The arithmetic behind the Low draw-call cap, at N = 48 (R3-BUDGET-REC2, lines 1142–1152): 4 draws per soldier
     per pass × 48 soldiers = 192 per pass; a shadow and a main pass make 384; the merged world about 202; four very
     large units up to 200; about 786, inside 800. A hull outline adds one draw per outlined object: 192 for the
     soldiers and about 80 for the very large units' parts, about 1,060. The research's N = 54 sum (about 834) no
     longer applies (MODES-3 (a); it supersedes r3-rendering.md lines 1144–1145, "at N = 54, MODES-3 (b), … about
     834, over 800"). The triangle and memory rows are the loosest: no source gives them (R3-BUDGET-F7, lines
     575–577; R3-BUDGET-OQ5, lines 1373–1374). *Need:* VISION_V2 §6.9; §4.
AFTER:
     The arithmetic behind the Low draw-call cap, at N = 48 (R3-BUDGET-REC2, lines 1142–1152): 4 draws per soldier
     per pass × 48 soldiers = 192 per pass; a shadow and a main pass make 384; the merged world about 202; four very
     large units up to 200; about 786. The research's sum stops there, but the same frame draws more: the health
     bars (at most 2, R3-CHAR-REC3's mark, lines 1074–1075); the 5 pooled visual-effect kinds of 5.k at one draw each
     (assumed for the count: one draw per kind in the main pass, as one instanced pool per kind would give; the
     technique is OQ-0003-7); the live projectiles' 4 kinds (bolt, grenade,
     Molotov, rocket, as GLOSSARY §3 lists them) in the main pass, 8 draws if they also cast shadows; and the
     full-screen post passes, from 2 (the ink and the output, inferred) to 16 (with the glow's twelve, 5.d, and the
     AO pass and its denoise, 5.c), if `drawCalls` counts them (not researched: PM-0003-12 reports their share). That
     is 786 + 2 + 5 + (4 to 8) + (2 to 16) = 799 to 817. So 800 holds only with the leanest post chain and
     unshadowed projectiles. The Low cap stays 800 (proposed: the research's number), the render budget bench counts
     the whole frame (PM-0003-12, PM-0003-22), and if the Low tier's kept effects do not fit, the draft weighs a cap
     of about 820 (817 rounded up, proposed) against a leaner Low tier, where switching an effect off follows
     OQ-0003-16. A hull outline adds one draw per outlined object: 192 for the soldiers and about 80 for the very
     large units' parts, about 1,060 (1,058), or 1,071 to 1,089 with the items above, inside 1,100. The research's
     N = 54 sum (about 834) no longer applies (MODES-3 (a); it supersedes r3-rendering.md lines 1144–1145, "at N =
     54, MODES-3 (b), … about 834, over 800"). The triangle and memory rows are the loosest: no source gives them
     (R3-BUDGET-F7, lines 575–577; R3-BUDGET-OQ5, lines 1373–1374). *Need:* VISION_V2 §6.9; §4.
END

### FIX PA-4 (part 7: the spectacle budget's "without dropping frames")
FILE: docs/adr/0003-rendering.md
BEFORE:
   - *The spectacle budget* (VISION_V2 §6.9): the stress scene's four very large units on screen with its soldiers add
     at most 2.0 ms (p99) GPU time and at most 200 draw calls against the scene without them, before any ink or AO
     pre-pass, on the recommended machine at High (4 units × 20 parts × 2 passes = 160 draws, plus margin;
     R3-CHAR-REC4, lines 1088–1092); their visual effects sit inside the visual-effects row, and their first
     appearance compiles nothing (part 3).
AFTER:
   - *The spectacle budget* (VISION_V2 §6.9): the stress scene's four very large units on screen with its soldiers add
     at most 2.0 ms (p99) GPU time and at most 200 draw calls against the scene without them, before any ink or AO
     pre-pass, on the recommended machine at High (4 units × 20 parts × 2 passes = 160 draws, plus margin;
     R3-CHAR-REC4, lines 1088–1092); their visual effects sit inside the visual-effects row, and their first
     appearance compiles nothing (part 3). Those are differences, and the frame rows are p95s, which let 30 of 600
     frames (5 %) run over, so §6.9's "without dropping frames" also gets a presented-frame count (proposed): on the
     recommended machine at High, Chrome on WebGPU, with the four units and their visual effects on screen and normal
     vsync on a 60 Hz display, the frames whose interval passes 1.5 refresh periods are counted (the loop follows the
     display, R3-BUDGET-F2, lines 545–549, so such a frame missed at least one refresh); the same count is reported
     on the minimum laptop at Low against a 33.3 ms period. How many such frames a run may hold (0 of 600, or a small
     share) is OQ-0003-17; the pass-mark row is test-engineer's.
END

### FIX AD-1, AD-2 (part 7: the tier table's look floor and the Low frame)
FILE: docs/adr/0003-rendering.md
BEFORE:
   - *Quality tiers as data* (R3-BUDGET-REC3, lines 1166–1177): one table holds, per tier (Low, Medium, High), the
     pixel-ratio cap, the scene resolution scale, MSAA samples, shadow map size and distance, each post effect's
     on/off and resolution scale, the animation budget and the visual-effect caps. Low is the default when the adapter
     is integrated or the backend is WebGL 2. Dynamic resolution (option B-3) is a measured option, not the start. The
     table's file format is ADR 0005's content data; its rows are this record's. *Need:* VISION_V2 §4 only.
AFTER:
   - *Quality tiers as data* (R3-BUDGET-REC3, lines 1166–1177): one table holds, per tier (Low, Medium, High), the
     pixel-ratio cap, the scene resolution scale, MSAA samples, shadow map size and distance, each post effect's
     on/off and resolution scale, the animation budget and the visual-effect caps. Low is the default when the adapter
     is integrated or the backend is WebGL 2. Dynamic resolution (option B-3) is a measured option, not the start. The
     table's file format is ADR 0005's content data; its rows are this record's. *The look floor:* the table "only
     says how much each may cost" (R3-BUDGET-REC3, lines 1172–1173), so its on/off switch never takes out a technique
     that carries a LOOK_V2 `[DECIDED]` item (part 5) unless Mustafa answers OQ-0003-16 that way. Low's starting row
     is the budget table's (pixel ratio capped at 1, scene scale 0.75–1, one 1024² sun map), and the look checklist
     shows that frame beside High and legacy, so Mustafa judges the look the minimum class sees (the step is
     test-engineer's). *Need:* VISION_V2 §4; LOOK_V2 (the floor).
END

### FIX EA-11 (engine-architect #12: the benchmark scene records the Worker's load)
FILE: docs/adr/0003-rendering.md
BEFORE:
     match with a fixed seed, less exact (r3-rendering.md, Pointers Q6, lines 1504–1505).
AFTER:
     match with a fixed seed, less exact (r3-rendering.md, Pointers Q6, lines 1504–1505). Each run records the
     Worker's load (a replay, a local match or an online client), because a replay runs the whole sim at N = 48, as a
     local match does, which is heavier than an online client's Worker, and on an integrated laptop the CPU shares
     power and heat with the GPU (an inference, not researched).
END

### FIX EA-11 (engine-architect #11: the copied world is O-B's cost; the Worker-side variant)
FILE: docs/adr/0003-rendering.md
BEFORE:
     because the page makes no call into the sim (r1-simulation-core.md, Recommendations, R1-BND-REC1 mark 2, lines 2261–2262).
AFTER:
     because the page makes no call into the sim (r1-simulation-core.md, Recommendations, R1-BND-REC1 mark 2, lines 2261–2262).
     That copy is O-B's cost: a second collision world, in TypeScript, tied to the map document's collider format. A
     Worker-side variant keeps one world: the Worker casts the rays, as it already casts the aim ray (ADR 0001 part
     5), and writes an occlusion factor per audible event into the block's event section. It was not researched, and
     it is weighed beside O-B if O-B is ever proposed.
END

### FIX EA-11 (engine-architect #15: the stale hedge on the buffer's place)
FILE: docs/adr/0003-rendering.md
BEFORE:
    newest ticks (r1-simulation-core.md, Recommendations, R1-TICK-REC5, lines 2094–2116), remote entities on ADR 0001's delay wherever its buffer lives
AFTER:
    newest ticks (r1-simulation-core.md, Recommendations, R1-TICK-REC5, lines 2094–2116), remote entities already delayed in the Worker (ADR 0001 part 7)
END

### FIX AD-7 (part 11: both LOOK_V2 §2 houses in the test assets)
FILE: docs/adr/0003-rendering.md
BEFORE:
    a few plain glTF/GLB exports (one soldier in LOOK_V2 §3's proportions with its faction colour slots, one weapon,
    one two-floor building with an interior and its damage stages, one very large unit of at least 20 parts, a few
    props), because LOOK_V2 §3's proportions and weapon silhouettes cannot be judged on legacy's primitive parts
AFTER:
    a few plain glTF/GLB exports (one soldier in LOOK_V2 §3's proportions with its faction colour slots, one weapon,
    LOOK_V2 §2's two houses, one very large unit of at least 20 parts, a few props): the half-timbered house with the
    brick-tile roof as the two-floor building with an interior and its damage stages, and the plastered house with
    the silo tower, authored too or, if only one building is authored, drawn by ADR 0005's ported kit builders, which
    drop the thatched roof (ADR 0005's CM7 row). Authored models are needed because LOOK_V2 §3's proportions and
    weapon silhouettes cannot be judged on legacy's primitive parts
END

### FIX AD-6, AD-4 (trace table: the ink on buildings)
FILE: docs/adr/0003-rendering.md
BEFORE:
| §2 `[DECIDED]` thick black outlines, especially strong on buildings | ink weight per material class, buildings heaviest (5.b) | R3-LOOK-REC3, R3-LOOK-OQ2 (1326–1327); a per-class weight in screen space not researched: OQ-0003-2 | buildings' ink beside legacy's, in both families | 0005 (material classes in the slots) |
AFTER:
| §2 `[DECIDED]` thick black outlines, especially strong on buildings | ink weight per material class, the structure class heaviest (5.b, 5.h) | R3-LOOK-REC3, R3-LOOK-OQ2 (1326–1327); a per-class weight in screen space not researched: OQ-0003-2 | buildings' ink beside legacy's, in both families, at their silhouettes and where they meet the ground | 0005 (material classes in the slots) |
END

### FIX PA-10 (Q5 table: the normal attachment with MSAA)
FILE: docs/adr/0003-rendering.md
BEFORE:
| **Screen-space depth and normal edges** (proposed, 5.b; F7) | Medium: a custom TSL node | one full-screen pass plus one normal attachment, about 16.6 MB at 1080p (F14) | scales with pixels only; Low stays at 800 draws | presentation only | lines in pixels; can mark folds inside a shape (inferred: OQ-0003-2); threshold tuning; thin lines alias; needs masking for the roof fade | new |
AFTER:
| **Screen-space depth and normal edges** (proposed, 5.b; F7) | Medium: a custom TSL node | one full-screen pass plus one normal attachment, about 16.6 MB at 1080p single-sampled, 66.4 MB multisampled at 4× MSAA plus its resolve (F14) | scales with pixels only; Low stays at 800 draws | presentation only | lines in pixels; can mark folds inside a shape (inferred: OQ-0003-2); threshold tuning; thin lines alias; needs masking for the roof fade | new |
END

### FIX AD-1 (trade-off: the hull at Low)
FILE: docs/adr/0003-rendering.md
BEFORE:
  both families are measured, and why the look call is Mustafa's: if he prefers the hull, the record keeps it at the
  measured cost (about 1,100 draws at Low, or High only).
AFTER:
  both families are measured, and why the look call is Mustafa's: if he prefers the hull, the record keeps it at the
  measured cost (about 1,100 draws at Low, or High only, with what Low draws then his call too: OQ-0003-16).
END

### FIX AD-1 (Consequences: Low is a data change inside the floor)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **Easier:** one shader language for both backends; every look technique has a tier switch from the start, so Low
  is a data change; the budget rows are data in the repository (ADR 0002's I-14), so a regression shows in CI as a
  count; per-unit asset budgets derive from one table (ADR 0005); legacy's draw-call problem is designed out (4 draws
  per soldier per pass against about 120).
AFTER:
- **Easier:** one shader language for both backends; every look technique has a tier switch from the start, so Low
  is a data change inside the look floor (part 7; OQ-0003-16); the budget rows are data in the repository (ADR
  0002's I-14), so a regression shows in CI as a count; per-unit asset budgets derive from one table (ADR 0005);
  legacy's draw-call problem is designed out (4 draws per soldier per pass against about 120).
END

### FIX AD-1 (Revisit when: what a missed frame-time mark may change)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **Revisit when:** the look proof test fails a frame-time mark on the minimum laptop (drop a technique to High only,
  or change family); a three.js upgrade changes a pass's cost by more than R3-WGPU-REC2's 5 %; client field data shows
  the fallback share (R3-GL2-OQ2), which may allow F-B extras; the camera
  framing is decided (R2-INT-OQ2), which re-decides distance LOD and the listener's place; Mustafa picks the hull
  family.
AFTER:
- **Revisit when:** the look proof test fails a frame-time mark on the minimum laptop (lower a technique in the tier
  data, change family, or move a `[DEFAULT]` item's technique to High only; a technique that carries a `[DECIDED]`
  item leaves Low only on Mustafa's answer to OQ-0003-16); a three.js upgrade changes a pass's cost by more than
  R3-WGPU-REC2's 5 %; client field data shows the fallback share (R3-GL2-OQ2), which may allow F-B extras; the
  camera framing is decided (R2-INT-OQ2), which re-decides distance LOD and the listener's place; Mustafa picks the
  hull family.
END

### FIX GE-1 (legacy rows CL2, CL4, CL8, CL14: one material per slot content)
FILE: docs/adr/0003-rendering.md
BEFORE:
| CL2, CL4, CL8, CL14 (111, 113, 117, 123) | redesign (look brief pending) | redesign: shared node toon materials per material class, applied by the engine to each asset's slots at load (part 5.a); no runtime conversion of authored materials | LOOK_V2 §1, §8 step 5; CL4's one-material-per-mesh lesson |
AFTER:
| CL2, CL4, CL8, CL14 (111, 113, 117, 123) | redesign (look brief pending) | redesign: shared node toon materials, one per slot content (the slot's class with its ramp, its faction, its own colours and textures: part 5.a), applied by the engine to each asset's slots at load; no runtime conversion of authored materials | LOOK_V2 §1, §8 step 5; CL4's one-material-per-mesh lesson and its content signature |
END

### FIX FC-9 (the CC10 row: LOOK_V2's words, then this record's outcome)
FILE: docs/adr/0003-rendering.md
BEFORE:
| CC10 (139) | port to TypeScript (pending) | port to TypeScript as the row says: the arm-mode enum in the animation state, the rest pose folded into the animation port on the authored, skinned soldier (part 6); pose values are tuning; pending removed | LOOK_V2 §3 (slimmer, serious proportions: soldiers become authored, skinned models, as for CL10, CC1–CC5); §2 keeps small movement touches |
AFTER:
| CC10 (139) | port to TypeScript (pending) | port to TypeScript as the row says: the arm-mode enum in the animation state, the rest pose folded into the animation port on the authored, skinned soldier (part 6); pose values are tuning; pending removed | LOOK_V2 §3 (slimmer, serious proportions), so soldiers become authored, skinned models (part 6, as for CL10, CC1–CC5); §2 keeps small movement touches |
END

### FIX FA-L2 = B25 (the GW17 row: the camera-player half stays pending)
FILE: docs/adr/0003-rendering.md
BEFORE:
| GW17 (205) | port to TypeScript (pending) | port to TypeScript: the roof fade by flag over the player's enclosure and the camera-player cutaway (OQ-0003-15), precompiled blend variant, ink masked (part 5.i); pending removed | LOOK_V2 §4 `[DEFAULT]` roofs fade |
AFTER:
| GW17 (205) | port to TypeScript (pending) | port to TypeScript: the roof fade by flag over the player's enclosure and the camera-player cutaway (OQ-0003-15), precompiled blend variant, ink masked (part 5.i); pending stays for the camera-player cutaway: "Mustafa's call: LOOK_V2 §4 names only roofs (OQ-0003-15)" | LOOK_V2 §4 `[DEFAULT]` roofs fade |
END

### FIX AD-4 (OQ-0003-2: the contact line is one of the ink's options)
FILE: docs/adr/0003-rendering.md
BEFORE:
  shipped pass has one thickness for all objects, R3-LOOK-F5).
AFTER:
  shipped pass has one thickness for all objects, R3-LOOK-F5); the contact line where buildings and soldiers meet
  the ground from an edge where the class value changes (screen-space, not researched: 5.b), or the hull's ground
  clamp (CL6).
END

### FIX AD-1, GE-4, PA-4, AD's REDUCED-MOTION and COLOUR-TELL (new open questions OQ-0003-16 to -19)
FILE: docs/adr/0003-rendering.md
BEFORE:
  side by side with legacy. Research ids: none (legacy rows only).
AFTER:
  side by side with legacy. Research ids: none (legacy rows only).
- **OQ-0003-16. The look floor: which LOOK_V2 items may a quality tier switch off on the minimum laptop?**
  Why it matters: Low is the default on every integrated adapter and every WebGL 2 player (part 7), so the whole
  minimum class sees it. The research's tier table has an on/off switch per effect (R3-BUDGET-REC3, lines
  1166–1168) and its shadow row reads "1024² or off" (R3-BUDGET-REC2, line 1129), while "Which effects exist at all
  is the look brief's call" (R3-BUDGET-REC3, lines 1172–1173), and LOOK_V2 decides cel-shaded rendering with bold
  ink and emissive accents (§1), soft shadows and thick outlines (§2). No source gives any technique's GPU time
  (R3-LOOK-F15, lines 411–413), so whether the floor fits Low's budget is unknown until the look proof test
  measures it. Options: (a) no `[DECIDED]` item is switched off at any tier: the toon materials (5.a), the ink
  (5.b), one sun shadow map (5.f) and the emissive slots' colour may only be lowered (resolution scale, map size,
  samples), while the `[DEFAULT]` items (AO, the glow's bloom, the comic touches) may be off on Low; (b) Low may
  switch named items off where its budget needs it, listed in the tier table; (c) Mustafa decides item by item in
  the look proof test, on the Low frame beside High and legacy, with each technique's measured cost (PM-0003-9).
  Parts 5 and 7 hold to (a) for the `[DECIDED]` items until he answers. Recommendation: (c), with (a) as the rule
  meanwhile, since the costs are unknown today. Decider: Mustafa. What decides it: the look checklist's Low
  frame (test-engineer's step) and PM-0003-9's costs on the minimum laptop. Research ids: R3-BUDGET-REC3,
  R3-BUDGET-REC2, R3-LOOK-F15 (no new disposition: both recommendations stay adopted in Appendix A).
- **OQ-0003-17. How is VISION_V2 §6.9's "without dropping frames" read as a mark?**
  Why it matters: the spectacle budget (part 7) is a difference in GPU time and draw calls, and the frame rows are
  p95s, so with the very large units on screen 30 of 600 frames (5 %) may run over one 60 Hz refresh, each a
  dropped frame once presented. Options: (a) the presented-frame count part 7 proposes: on the recommended machine
  at High, with the four units and their visual effects on screen, at most a stated number of frames per 600 whose
  interval passes 1.5 refresh periods (0, or a small proposed share); (b) the p95 rows as written. Decider: the
  Advisor, confirmed by Mustafa at the stop review (VISION_V2 is theirs). What decides it: their reading of §6.9,
  then the render budget bench's counts. Research ids: R3-CHAR-REC4 (lines 1088–1092; it stays adopted, no new
  disposition).
- **OQ-0003-18. Do the concussion shake, the halftone flashes and line boil soften for players who ask for reduced
  motion?**
  Why it matters: LOOK_V2 §2 makes the concussion shake a decided feel and §7 the flashes a default, and legacy
  honours the system's `prefers-reduced-motion` only in the war-room menus (`shamble-game.js` line 3039,
  `index.html` line 321), never for the camera shake. Options (not researched): (a) as legacy, menus only; (b) a
  setting that softens the shake and switches the flashes and line boil off; (c) follow the system setting by
  default. Each leaves every `[DECIDED]` item as it is for players who keep the default; (b) or (c) would trace to
  LOOK_V2 §2 and §7 only (no VISION_V2 §6 item). Decider: Mustafa, in the look proof test (parts 5.l, 5.m). What
  decides it: his judgement. Research ids: none.
- **OQ-0003-19. Are the three factions told apart by colour alone?**
  Why it matters: in a three-team fight, players with a colour-vision deficiency may not tell the teams apart;
  LOOK_V2 §2 makes uniform details the seed of faction identity, and part 6 carries faction colour through the
  faction colour slots. Options (not researched): (a) colour only, for now; (b) the look proof test also shows the
  fight view through a colour-blindness simulation, and Mustafa judges it; (c) faction shape cues, left to a later
  design session. Decider: Mustafa. What decides it: his judgement in the look proof test. Research ids: none.
END

### FIX AD-1, AD's questions (the calls Mustafa makes in the look proof test)
FILE: docs/adr/0003-rendering.md
BEFORE:
  (OQ-0003-8), the pixel-ratio cap (OQ-0003-12) and the legacy touches (OQ-0003-15). The test also records which
AFTER:
  (OQ-0003-8), the pixel-ratio cap (OQ-0003-12), the legacy touches (OQ-0003-15), the look floor on Low
  (OQ-0003-16), reduced motion (OQ-0003-18) and telling the factions apart (OQ-0003-19). The test also records which
END

### FIX TE-5 (look proof test, both backends: every kept effect on)
FILE: docs/adr/0003-rendering.md
BEFORE:
  backends' screenshots of the same frame differ in at most 1 % of pixels by more than 8/255 per channel (a proposed
  tolerance; effects on one backend only are off for this test); the WebGPU backend's median CPU frame time is no
AFTER:
  backends' screenshots of the same frame differ in at most 1 % of pixels by more than 8/255 per channel (a proposed
  tolerance), with every kept effect on: under F-A no effect exists on one backend only (part 1), and an F-B extra
  is compared through its named fallback; the WebGPU backend's median CPU frame time is no
END

### FIX AD-1, AD's questions (migration step 2: the new calls recorded)
FILE: docs/adr/0003-rendering.md
BEFORE:
   OQ-0003-2, -3, -5, -6, -8, -12 and -15, recorded in this ADR.
AFTER:
   OQ-0003-2, -3, -5, -6, -8, -12, -15, -16, -18 and -19, recorded in this ADR.
END

### FIX AD-1 (migration step 4: a change of tier inside the floor)
FILE: docs/adr/0003-rendering.md
BEFORE:
   work. *Milestone:* every row of the budget table on both machines, or a recorded change of tier for each technique
   that misses.
AFTER:
   work. *Milestone:* every row of the budget table on both machines, or a recorded change of tier for each technique
   that misses, inside the look floor (part 7; OQ-0003-16).
END

### FIX AD-1 (Rollback: the ink and the AO at Low)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **The screen-space ink fails** (its cost, aliasing or Mustafa's look): switch to the hull family, already measured;
  the Low cap becomes about 1,100 draws or the hull a High-only setting.
- **Single-pass AO fails:** the documented pre-pass on High only, AO off on Low.
AFTER:
- **The screen-space ink fails** (its cost, aliasing or Mustafa's look): switch to the hull family, already measured;
  the Low cap becomes about 1,100 draws, or the hull a High-only setting with what Low draws then Mustafa's call
  (OQ-0003-16).
- **Single-pass AO fails:** the documented pre-pass on High only, AO off on Low (a `[DEFAULT]` item; OQ-0003-16 asks
  whether that holds).
END

### FIX AD-1 (Rollback: any technique that misses on Low)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **Any technique misses on Low:** a tier change in data moves it to High only, with no code change.
AFTER:
- **Any technique misses on Low:** a tier change in data lowers it (resolution scale, map size, samples) or, for a
  `[DEFAULT]` item's technique, moves it to High only, with no code change; a technique that carries a LOOK_V2
  `[DECIDED]` item leaves Low only on Mustafa's answer to OQ-0003-16, given on the Low frame with the measured costs.
END

### FIX AD-1 (Appendix A: R3-BUDGET-REC3's own reason is the floor)
FILE: docs/adr/0003-rendering.md
BEFORE:
| R3-BUDGET-REC3 | adopted | part 7 |
AFTER:
| R3-BUDGET-REC3 | adopted | part 7; its own reason, "Which effects exist at all is the look brief's call" (lines 1172–1173), is the look floor (OQ-0003-16) |
END

### FIX GE-1, AD-10, REDUCED-MOTION (Sources: the files newly cited)
FILE: docs/adr/0003-rendering.md
BEFORE:
  records it.
AFTER:
  records it.
- `docs/ART_DIRECTION.md` line 121 (the halftone overlay dots the HUD); legacy code read for two facts:
  `shamble-core.js` lines 356–359 (`_matSig`, the content signature) and `shamble-game.js` line 3039 with
  `index.html` line 321 (reduced motion in the menus only).
END

### FIX GE-1 (GPU part 1: one material per slot content, the content defined)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   and no ink. At load the engine gives each slot the shared node toon material of that slot's material class (ADR
   0003, part 5.a), one material per slot content and never one per mesh. *Why:* each distinct ramp is a distinct
   material, so materials must be shared by content or batching breaks (r3, Findings Q3, R3-LOOK-F3, lines 331–334);
AFTER:
   and no ink. At load the engine gives each slot a shared node toon material of that slot's material class (ADR
   0003, part 5.a), one material per slot content and never one per mesh. A slot's content is its class (ramp,
   colour gain, ink weight), its faction for faction colour slots, and its own base and emissive colour, textures,
   side and alpha mode, as legacy keys its shared materials by content (CL4, `docs/LEGACY_INVENTORY.md` line 113;
   `_matSig`, `shamble-core.js` lines 356–359); so an asset keeps its own colours and textures inside a class, and
   how colour is authored is OQ-0005-AS4's, not a rule. *Why:* each distinct ramp is a distinct material, so
   materials must be shared by content or batching breaks (r3, Findings Q3, R3-LOOK-F3, lines 331–334);
END

### FIX AD-6 (GPU part 2: a structure class)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
2. **Material slots name a material class.** Every slot's name maps to one class from a small fixed list (for
   example environment, unit, faction colour, emissive); the list and the names are this ADR's content data (interface
AFTER:
2. **Material slots name a material class.** Every slot's name maps to one class from a small fixed list (for
   example environment, structure, unit, faction colour, emissive; the structure class gives buildings their heavier
   ink, LOOK_V2 §2, ADR 0003 parts 5.b and 5.h); the list and the names are this ADR's content data (interface
END

### FIX AD-14 (GPU part 4: the 4-material cap bounds a soldier's surface kinds)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   - *A soldier as drawn* (body, headgear and held weapon) uses at most 4 materials per pass: option C-B draws "one
     per material per soldier" (r3, Options Q4, line 825) and ADR 0003 caps a soldier at 4 draw calls per pass
     before the ink (R3-CHAR-REC1, lines 1044–1045).
AFTER:
   - *A soldier as drawn* (body, headgear and held weapon) uses at most 4 materials per pass: option C-B draws "one
     per material per soldier" (r3, Options Q4, line 825) and ADR 0003 caps a soldier at 4 draw calls per pass
     before the ink (R3-CHAR-REC1, lines 1044–1045). Each material carries one class ramp (part 1), so at most 4
     surface kinds (skin, cloth, faction colour, metal, emissive) shade differently on one soldier, where legacy used
     separate skin, cloth, metal and lacquer ramps (CL2, `docs/LEGACY_INVENTORY.md` line 111); the look proof test's
     soldier is built at this cap.
END

### FIX PA-11 (GPU part 4: the triangle ceiling the same arithmetic gives)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
     R3's own test load, 20 parts each with one material, fits (4 × 20 × 2 = 160, lines 1089–1090).
AFTER:
     R3's own test load, 20 parts each with one material, fits (4 × 20 × 2 = 160, lines 1089–1090).
   - *A triangle ceiling, not a budget:* ADR 0003's harness resets `renderer.info` once per frame (its part 8), so the
     triangle row counts every pass of the frame, the shadow pass included, as the draw-call arithmetic does (assumed:
     how three.js counts the shadow pass's triangles is not researched). At Low, 500k ÷ (48 soldiers × 2 passes) ≈
     5.2k triangles per soldier as drawn, if soldiers had the whole row before the world and the very large units
     take their share (arithmetic on R3-BUDGET-REC2's triangle row, r3 line 1134): a planning input for OQ-0005-AS5
     and AS6.
END

### FIX GE-1 (GPU part 7: cached by the slot content)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
7. **Materials and textures are shared, cached and compiled up front.** One material per class (and per faction for
   faction slots) is created once, cached and reused, "never rebuilt per edit" (EDITOR_V2 §2, rule 3), and never
   disposed while shared; every one is compiled in ADR 0003's warm-up, so a unit's first appearance creates no shader
   program (r3, Recommendations Q1, R3-WGPU-REC3, lines 928–939). *Need:* VISION_V2 §6.9; EDITOR_V2 §2.
AFTER:
7. **Materials and textures are shared, cached and compiled up front.** One material per slot content (part 1: the
   class, the faction for faction slots, and the slot's own colours, textures, side and alpha mode) is created once,
   cached by that content and reused, "never rebuilt per edit" (EDITOR_V2 §2, rule 3), and never disposed while
   shared; every one is compiled in ADR 0003's warm-up, so a unit's first appearance creates no shader program (r3,
   Recommendations Q1, R3-WGPU-REC3, lines 928–939). *Need:* VISION_V2 §6.9; EDITOR_V2 §2.
END

### FIX AD-7 (GPU part 8: both houses)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
8. **The look proof test's models.** ADR 0003's look proof test needs a few authored models (a soldier, a weapon, a
   building with an interior, a very large unit, props). They are plain glTF/GLB exports; the test decides nothing
   about compression or LODs (ASSET-RESEARCH (a)).
AFTER:
8. **The look proof test's models.** ADR 0003's look proof test needs a few authored models (a soldier, a weapon,
   LOOK_V2 §2's two houses, one of them with an interior and its damage stages, a very large unit, props; ADR 0003
   part 11 lists them and says where the second house comes from if only one is authored). They are plain glTF/GLB
   exports; the test decides nothing about compression or LODs (ASSET-RESEARCH (a)).
END

### FIX GE-1 (options table: the proposed row)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| **One shared toon material per slot class, applied at load** (proposed, part 1) | Low | one material per class and faction | materials stay few, so sharing and merging work (R3-LOOK-F3) | presentation only | one look on every asset, set in one place | new; legacy's per-mesh conversion is the lesson (CL4) |
AFTER:
| **One shared toon material per slot content, applied at load** (proposed, part 1) | Low | one material per distinct slot content (class, faction, the slot's own colours and textures) | materials stay as few as the assets' distinct contents, so sharing and merging work (R3-LOOK-F3) | presentation only | one look on every asset, set in one place; each asset keeps its own colours | new; legacy's per-mesh conversion is the lesson (CL4) |
END

### FIX GE-1 (options: pros and cons)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **Shared per class.** Pros: bounded materials; one place to tune the look. Cons: every slot must be named into a
  class (OQ-0005-AS4). **Per mesh.** Pros: no naming rule. Cons: one material per mesh, legacy's measured problem.
AFTER:
- **Shared per slot content.** Pros: materials bounded by the assets' distinct contents; one place to tune each
  class's look. Cons: every slot must be named into a class (OQ-0005-AS4); an asset with many distinct colours brings
  as many materials, and ADR 0003's static merge groups only equal contents. **Per mesh.** Pros: no naming rule.
  Cons: one material per mesh, legacy's measured problem.
END

### FIX GE-1 (OQ-0005-AS4: how colour is authored is an option here)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  Why it matters: part 1 and part 2 depend on every slot carrying a class. Options: none researched (a manual
  convention in the modelling tool, or LOOK_V2 §8's automated step). Decider: the asset-pipeline research. What
  decides it: whether Spline's glTF export keeps material names (not researched). Research ids: none.
AFTER:
  Why it matters: part 1 and part 2 depend on every slot carrying a class; and since part 1 makes one material per
  slot content, how an asset carries its colours sets how many materials it brings. Options: none researched (a
  manual convention in the modelling tool, or LOOK_V2 §8's automated step; for colour, each slot's material colour,
  vertex colours, or one texture atlas). Decider: the asset-pipeline research. What decides it: whether Spline's
  glTF export keeps material names (not researched), and each colour route's material count against part 4's caps.
  Research ids: none.
END

### FIX GE-1 (this section's spike part: count distinct slot contents)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  own left in use; the scene holds no more material objects than the distinct (slot class, faction) pairs it shows
  (proposed mark; reason: sharing by content, R3-LOOK-F3); the test soldier as drawn meets 4 draw calls per pass and
AFTER:
  own left in use; the scene holds no more material objects than the distinct slot contents it shows (part 1;
  proposed mark; reason: sharing by content, R3-LOOK-F3), and two meshes with the same slot content share one
  material object; the test soldier as drawn meets 4 draw calls per pass and
END

### NOTE PA-4 (routed)
routed: test-engineer adds the pass-mark row part 7's spectacle sentence names: RD at High, Chrome on WebGPU, BS with
the 4 very large units and their visual effects on screen, mode P on a 60 Hz display; counted, the frames whose
animation-frame interval passes 1.5 refresh periods; at most the share OQ-0003-17 sets (until then reported, with 0
of 600 as the proposed reading); the same count reported on ML at Low against a 33.3 ms period. engine-architect:
ENGINE_OVERVIEW's §6.9 row (line 112) adds "part 7's presented-frame count (OQ-0003-17)"; the README gets a row for
OQ-0003-17 (decider: the Advisor, confirmed by Mustafa at the stop review).
END

### NOTE PA-8 (routed)
routed: test-engineer's PM-0003-12 note ("R3's 786 and 1,058 count no post pass, which leaves 14 draws …") now has
part 7's full sum to point to: 786 + 2 + 5 + (4 to 8) + (2 to 16) = 799 to 817 with the screen-space ink, 1,071 to
1,089 with the hull; the gate stays 800 and 1,100, counted over the whole frame.
END

### NOTE PA-10 (routed)
routed: test-engineer's PM-0003-16 also records whether `renderer.info.memory.total` changes when MSAA is switched on
(5.n now says the look proof test records it), and PM-0003-9's memory cells give both the single-sample and the 4×
MSAA size per attachment.
END

### NOTE GE-1 (routed)
routed: test-engineer's PM-0005-35 counts material objects against "the distinct slot contents" (GPU part 1's key:
class, faction for faction slots, the slot's own base and emissive colour, textures, side and alpha mode), not the
(slot class, faction) pairs, and keeps its per-mesh-clone bite; PM-0005-34 reads the same way where it counts
materials.
END

### NOTE AD-1, GE-4 (routed)
routed: test-engineer's "When a mark fails" (0003:1028–1030) drops "a technique moved to High only in the tier data"
as a general route and reads: "a technique lowered in the tier data, or a `[DEFAULT]` item's technique moved to High
only; a technique that carries a `[DECIDED]` item leaves Low only on Mustafa's answer to OQ-0003-16"; PM-0003-23's
"one shadow map at 1024² or none" reads "one 1024² shadow map (none only on Mustafa's answer to OQ-0003-16)". The
look checklist gains the Low-frame step (AD-2; GE-4's fix): the whole frame, the ink, the shadows and the AO views
(steps 2, 4, 6, 7, 20) at Low's starting row (ratio 1, scene scale 0.75, one 1024² map) beside High and legacy,
recording per item whether each `[DECIDED]` item still reads; each step names its tier. engine-architect: README rows
for OQ-0003-16 (Mustafa), which is GE Q1 and AD's LOOK-FLOOR in one.
END

### NOTE AD-4 (routed)
routed: test-engineer's look checklist step 4 adds to its Expected line: "the ink at the base of buildings, and where
soldiers stand, is as heavy as their silhouette" (5.b now names the contact line as a visible difference; OQ-0003-2
carries the class-value edge as a not-researched option).
END

### NOTE AD-6 (routed)
routed: engine-architect's AP-3 (0005:1392) example list reads "(for example environment, structure, unit, faction
colour, emissive)", as GPU part 2 now does.
END

### NOTE AD-7 (routed)
routed: engine-architect's SPIKE_PLAN look proof test assets (SPIKE_PLAN:236–237) list both LOOK_V2 §2 houses as ADR
0003 part 11 now does (the second authored, or drawn by the ported kit builders if only one building is authored).
Who makes the test models stays GE Q2, with the synthesis.
END

### NOTE AD-10 (routed)
routed: test-engineer's PM-0003-9 costs the halftone overlay as frame time with it and without it (the GPU timers
cannot see a CSS layer), and checklist step 27 shows the flash over the HUD.
END

### NOTE AD-11
fixed in 5.b (the FIX block for AD-4, AD-6, AD-11, AD-1, PA-8): the width is set in CSS pixels and scaled by pixel
ratio × resolution scale, proposed, so a tier changes cost, not weight.
END

### NOTE AD-14 (routed)
fixed in GPU part 4; routed: test-engineer's checklist steps 10–12 say the soldier shown is built at the 4-material
cap, so Mustafa judges what the budget allows.
END

### NOTE TE-5 (routed)
routed: test-engineer's PM-0003-3 drops "effects that exist on one backend only are off" and compares with every kept
effect on; an F-B extra, if one ever enters, is compared through its named fallback.
END

### NOTE EA-11 (routed)
fixed in parts 6, 8, 9 and 10 (four blocks); routed: test-engineer's BS definition (0003:928–931, "How to read the
table") records the Worker's load with each run (replay, local match or online client), and PM-0003-18 may run once
more with the online client's load (engine-architect #12's option).
END

### NOTE REDUCED-MOTION, COLOUR-TELL (routed)
added as OQ-0003-18 and OQ-0003-19 (decider: Mustafa, in the look proof test); engine-architect: README rows for both.
If Mustafa picks COLOUR-TELL (b), test-engineer's look checklist adds the fight view through a colour-blindness
simulation.
END
