# Claims ledger: ADR 0005, "The asset pipeline: GPU rules (graphics-engineer)", 2026-10-31

A notes file beside `graphics-engineer-adr0005-assets-section.md`, not part of it. One row per claim or number in the
section. Lines are at 9e7db28; r3 = `docs/research/r3-rendering.md`, INV = `docs/LEGACY_INVENTORY.md`. Tiers as in
`graphics-engineer-adr0003-claims.md` (its header lists the R3 findings the R3 verifier checked: those are `research
verified`; other R3 findings, legacy facts and "not researched" claims are `research, source to check`; R3's
recommendation numbers are `proposed mark`).

| # | the claim, as written (short) | section | citation (file, id, lines) | tier |
|---|---|---|---|---|
| 1 | LOOK_V2 §8's flow, quoted (sketch, Spline, glTF/GLB, automated compress/LODs/renaming, the engine applies the look); budgets per unit; never Spline's runtime | intro, part 6 | LOOK_V2 §8 | design doc |
| 2 | The research has no findings on compression, LOD generation, export checks or slot renaming | intro | grep of docs/research/r*.md at 9e7db28 (0 whole-word hits for KTX2, meshopt, meshoptimizer, Draco, Spline, gltf-transform, Basis Universal: advisor-verdicts.md §1, "What I checked myself"); decision map h, G1 | research, source to check |
| 3 | The one glTF mention: three.js's example loads a glTF skinned mesh into the WebGPU renderer | intro | r3 R3-CHAR-F8, 476–480; advisor-verdicts.md §1 condition 4 | research, source to check |
| 4 | The asset-pipeline research: graphics-engineer, verified like R3, own daytime session, placed before the asset spike | intro, OQ | ASSET-RESEARCH (a), advisor-verdicts.md §1 condition 4; decision map J4 | settled (the advisor's ruling) |
| 5 | Each distinct ramp is a distinct material; share by content or batching breaks | part 1, Options | r3 R3-LOOK-F3, 331–334 | research, source to check |
| 6 | Legacy converted per mesh, one material per mesh, which defeated its first merge attempt | part 1, Options | INV CL4, line 113 (AUDIT_BUGS_PERF CS12) | research, source to check |
| 7 | Custom GLSL and onBeforeCompile "not supported in WebGPURenderer" | part 1 | r3 R3-WGPU-F6, 97–102 | research verified |
| 8 | The engine gives each slot the shared node toon material of its class (ADR 0003, 5.a) | part 1 | ADR 0003 draft, part 5.a (its ledger rows 60–64) | as those rows |
| 9 | Slot list and names are ADR 0005's content data, consumed by ADR 0003 | part 2 | decision map I-8, seam S9 | settled (the map's assignment) |
| 10 | instanceColor on InstancedMesh and BatchedMesh | part 2, Options | r3 R3-CHAR-F4, 452–455; R3-CHAR-F6, 463–467 | research verified (F4); research, source to check (F6) |
| 11 | Per-instance data, faction colour included, from the block (R3-CHAR-REC3) | part 2 | r3 R3-CHAR-REC3, 1065–1075 | proposed mark (R3's) |
| 12 | Emissive output drives a selective glow ("use the `emissive` material property…") | part 2 | r3 R3-LOOK-F12, 392–395; LOOK_V2 §6.4 | research, source to check |
| 13 | Skinned mesh "requires skin indices and weights as buffer attributes in its geometry" | part 3 | r3 R3-CHAR-F1, 436–439 | research verified |
| 14 | One skinned mesh per soldier (C-B), any weights | part 3 | r3 R3-CHAR-REC1, 1032–1048 | proposed mark (R3's) |
| 15 | Very large units as part groups matching the sim's vehicle-part records (V-A) | part 3, Options | r3 R3-CHAR-REC4, 1077–1092; Options Q4, 834–836 | proposed mark (R3's) |
| 16 | C-B draws "one per material per soldier" | part 4, Options | r3 Options Q4, 825 | research, source to check |
| 17 | Cap of 4 draw calls per soldier per pass before the ink | part 4 | r3 R3-CHAR-REC1 mark, 1044–1045 | proposed mark (R3's) |
| 18 | So a soldier as drawn uses at most 4 materials per pass | part 4 | rows 16 and 17 (one draw per material; 4 draws) | arithmetic (4 draws ÷ 1 draw per material = 4 materials) |
| 19 | Four very large units: 200 draw calls over the shadow and main pass | part 4 | r3 R3-CHAR-REC4 mark, 1088–1090 | proposed mark (R3's) |
| 20 | 200 ÷ 4 units ÷ 2 passes = 25 per unit per pass | part 4 | row 19 | arithmetic |
| 21 | R3's test load fits: 4 × 20 × 2 = 160 | part 4 | r3 1089–1090 | arithmetic |
| 22 | Frame totals: Low ≤ 800 draws, ≤ 500k triangles, ≤ 512 MB; High ≤ 1,500, ≤ 1.5M, ≤ 1 GB | part 4, OQ-AS1 | r3 R3-BUDGET-REC2, 1121–1137 (memory 1135) | proposed mark (R3's) |
| 23 | three.js LOD switches "at the distances specified" with hysteresis "to avoid flickering" | part 5 | r3 R3-CHAR-F10, 490–493 | research, source to check |
| 24 | Distance LOD only where the framing spreads distances; budget LOD for animation | part 5, Options | r3 R3-CHAR-REC2, 1050–1063 | proposed mark (R3's) |
| 25 | Camera framing is a design question LOOK_V2 does not settle | part 5 | r3 R3-CHAR-OQ5, 1355–1356; decision map b (owner DESIGN) | research, source to check |
| 26 | Only animation LOD is researched | OQ-AS2 | decision map h, G9 (mesh LOD generation not researched) | research, source to check |
| 27 | Materials cached and reused, "never rebuilt per edit" | part 7 | EDITOR_V2 §2 rule 3 | design doc |
| 28 | Never disposed while shared | part 7 | CLAUDE.md code rules ("never dispose pooled/shared materials") | settled (project rule) |
| 29 | Every material compiled in the warm-up; first appearance creates no program | part 7 | r3 R3-WGPU-REC3, 928–939 | proposed mark (R3's) |
| 30 | The look proof test needs a few authored models; plain glTF/GLB; decides nothing on compression or LODs | part 8 | ADR 0003 draft, part 11; ASSET-RESEARCH (a) | settled (the advisor's ruling) |
| 31 | The tree style is [OPEN]; tree assets wait | part 9 | LOOK_V2 §8; decision map e2 (§8 OPEN row) | design doc |
| 32 | Options rows (shared per class; per mesh; baked look not researched; V-A, V-B; shared per faction; per-instance colour; L-1, L-2) | Options | r3 Options Q4, 820–836; R3-LOOK-F3; INV CL4 | research, source to check |
| 33 | The Juggernaut is multi-part already | Options | INV GC16, line 255 | research, source to check |
| 34 | Instanced candidates (C-D, C-E) are measured against C-B in ADR 0003 | Options | r3 R3-CHAR-REC1, 1032–1048 | proposed mark (R3's) |
| 35 | Legacy's unchecked tags fail silently | OQ-AS3 | INV CL14, line 123 | research, source to check |
| 36 | Whether Spline's glTF export keeps material names is not researched | OQ-AS4 | grep of docs/research for "Spline": 0 hits (row 2) | research, source to check |
| 37 | R3-BUDGET-OQ5 (triangle and memory rows loose) is ADR 0003's | OQ-AS5 | decision map b (R3-BUDGET-OQ5 owner 0003) | settled (the map's assignment) |
| 38 | Animation row ≤ 1.0 ms at Low includes the bone upload | OQ-AS6 | r3 R3-BUDGET-REC2, 1126 | proposed mark (R3's) |
| 39 | "vertex and bone counts per soldier are unknown until it exists" | OQ-AS6, A | r3 R3-CHAR-OQ3, 1349–1351 | research, source to check |
| 40 | Spike: material objects ≤ distinct (slot class, faction) pairs | spike | reason R3-LOOK-F3 (row 5) | proposed mark (this section's, reason sharing by content) |
| 41 | Spike: soldier ≤ 4 draws per pass, very large unit ≤ 25 | spike | rows 18, 20 | arithmetic |
| 42 | Spike: zero console errors on both backends with the test models | spike | r3 R3-WGPU-REC1 mark 1, 910–912 (zero console errors) | proposed mark (R3's) |
| 43 | Appendix row: R3-CHAR-OQ3 is ADR 0005's | A | engine-architect-decision-map-ids.tsv (R3-CHAR-OQ3, owner 0005) | settled (the map's assignment) |

## Counts
- Rows: 43.
- By tier (first tier named): research verified 3 · research, source to check 16 · arithmetic 4 · settled 8 ·
  design doc 3 · proposed mark 12 · restating another file's rows 1 (row 8).
