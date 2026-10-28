Task: R3 wave 2 owner fix step (apply verifier-r3 spot-check to docs/research/r3-rendering.md)

## Progress
- started 2026-10-28; no earlier notes. Next: save verifier report verbatim, read topic file.
- saved verifier report verbatim to verifier-r3-result.md. Read r3-rendering.md in full (1512 lines).
- plan: L1 LOOK-REC2 (count planned extra draws, keep hull+GTAO open); L2 BUDGET-REC2 Low cap conditional + arithmetic;
  L3 "rigid-skinned" wording (TL;DR, CHAR-REC1, C-B row); L4 conditional camera wording AUDIO-REC1, CHAR-REC2; L5 note only;
  C1 BUDGET-REC1 reason + Linux run + CPU class (proposed); C2 CHAR-REC1 2.0 ms = no LOD vs 1.0 ms with LOD; C3 CHAR-REC4
  bundles only with measured gain; C4 GL2-REC1 F13 wording; C5 Rests-on adds; C6 Serves notes; C7 as 5 new OQs (not options:
  no new claims allowed); C8 GL2-F11/BUDGET-F6/MEASURE-F6; C9 OQ5 gaps, counts not bytes, CHAR-REC4 4 units (200 draws,
  2.0 ms), AUDIO-REC2 reason; C11 CHAR-REC1 wording. No re-sourcing needed (0 wrong/unsupported).
## Change log (type · what)
- nit · header status -> "spot-checked (44 of 77 key claims)"; totals OQ 33 -> 38 (C7's five new OQs)
- nit · TL;DR line 4 "rigid-skinned soldiers" -> "one skinned mesh per soldier" (L3)
- nit · index table OQ ranges for GL2, LOOK, CHAR, BUDGET, AUDIO (+1 each)
- nit · GL2-F11 now quotes the version_added values; "everywhere" marked inferred (C8)
- nit · BUDGET-F6 quotes Info (counts vs bytes), last clause marked inferred (C8, C9)
- nit · MEASURE-F3 programs/renderTargets are counts, quotes from verifier (C9)
- nit · MEASURE-F6 certainty -> inferred from GL2-F4, F5 (C8)
- nit · AUDIO-F4: "keeps costing a voice" split; voice cost is independent of gain (inferred, spike checks) (C9)
- nit · option C-B row: rigid weights only for legacy-style parts, any weights otherwise (L3)
- nit · WGPU-REC2, WGPU-REC4, GL2-REC3 Serves: "No VISION_V2 §6 item" note (C6)
- nit · WGPU-REC3 Rests on + R3-MEASURE-F3 (C5)
- recommendation · GL2-REC1 reason: F12 platform groups, share unknown (OQ2); F13 read as D3D11.1 stat only (C4)
- nit · LOOK-REC1 Rests on + R3-MEASURE-F3 (C5)
- recommendation · LOOK-REC2: hull outline and GTAO pre-pass are planned extras, not excluded; mark = 1.1 x (effects-off + planned extras); Rests on + LOOK-F5, MEASURE-F3 (L1)
- recommendation · CHAR-REC1: "rigid-skinned" -> skinned mesh, weights by model (L3); WebGPU skinning unconfirmed, mark checks both backends (C11); mark: 2.0 ms = full rig no LOD, 1.0 ms with budget LOD at Low (C2)
- recommendation · CHAR-REC2: budget LOD under any camera; distance LOD conditional on the brief's camera (L4); mark defines animation = posing + upload (C2)
- recommendation · CHAR-REC4: bundles only with a measured gain (C3); Rests on + GL2-REC1 (cited)
- number · CHAR-REC4 mark: 3 -> 4 large units; 150 -> 200 draws; 1.5 -> 2.0 ms GPU (proposed, arithmetic shown) (C9)
- recommendation · BUDGET-REC1: reason rewritten (F4 is Linux-only; Windows reaches WebGL 2 only via forceWebGL; real fallback = Firefox on Linux/Intel Mac); adds a Linux run of the same laptop and a proposed CPU floor; Rests on + WGPU-F17, F18, GL2-F12, R1-TICK-OQ6; Serves note (C1, C6)
- number · BUDGET-REC1: laptop CPU floor 4 cores / 8 threads (proposed, no source) (C1)
- recommendation · BUDGET-REC2: arithmetic for the Low draw-call cap shown; cap holds only without a per-object outline pass; with hull at Low the brief's trade (L2); animation row tied to budget LOD and CHAR-REC1's 2.0 ms explained (C2); Rests on + LOOK-F5, CHAR-REC1/REC4 cited
- number · BUDGET-REC2 Low draw calls: "≤ 800" -> "≤ 800 without a per-object outline pass; about ≤ 1,100 with one" (proposed, arithmetic) (L2)
- nit · MEASURE-REC2 cites R1-DET-REC1, R1-DET-REC6 (text, Rests on, pointer) (C5)
- nit · MEASURE-REC3 Serves: no §6 item note (C6)
- recommendation · AUDIO-REC1: listener at player made conditional on a legacy-like camera (L4); Serves fixed: brief's R3 row, §3 = largest mode (C6)
- recommendation · AUDIO-REC2 reason: cap needed because voices cost regardless of gain, not because curves never reach zero (C9)
- nit · AUDIO-REC2, AUDIO-REC4 Serves: brief's R3 row + "no §6 item names audio" (C6)
- nit · AUDIO-REC3 Serves: brief's R3 row; §6.2 is reused data, not served (C6)
- number · WGPU-OQ5: "two to five months" -> "55, 76 and 85 days apart (about two to three months)" (C9)
- nit · C7: five new open questions instead of new options (no new claims allowed): GL2-OQ5 (no fallback), LOOK-OQ6 (anti-aliasing), CHAR-OQ6 (GPU culling, indirect), BUDGET-OQ6 (shadow technique), AUDIO-OQ6 (AudioWorklet)
- next: part 7 table + critical-read list + counts
- nit · part 7 written: verifier table (44 rows) + final-state column, critical-read table with actions, counts, not-verified list
- disagreement (partial): C7 applied as five open questions, not option rows: the fix rule allows no new claims and an option row needs sourced findings. L5 note only (no change). C10 no action.
- nit · "How to read" wave-1 id list + R1-TICK-OQ6, R1-DET-REC1, R1-DET-REC6
- check: 92 findings, 25 recs, 38 OQs counted in file; header matches. No re-sourcing, no fetches.
- tally: nit 21 · recommendation 11 · number 5 · source 0
DONE
