# R3 wave 2 second pass: the verifier's report, verbatim

Workflow run wf_7260c3bd-5b1 (script `v2-research-r3-second-pass-run1.js`, prompt `prompts/verifier-r3-second-pass.txt`), started 2026-10-28, cut off by the spend limit, resumed and returned 2026-10-29 (about 21:15 UTC). Notes: `verifier-r3-second-pass.md`.

VERDICT: PASS WITH NOTES. The fix closes L1, L2, C1 and C2, and C3 to C11 except where noted below. Two minor consistency gaps of the L2 kind remain (A and B, under item 2). No claim is wrong.

Gate: skipped, as the task says (research check; the main session runs the gate).
Symbol diff: not applicable (research).
Evidence: `git diff 605c09d HEAD -- docs/research/r3-rendering.md` (the working tree equals HEAD). 5 WebFetch reads, all on allow-listed domains, plus reads of the repo docs. Exact quotes are in /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r3-second-pass.md, which ends with DONE. Not committed.

**1. Spot-check findings: closed, with the line**
- **L1 (closed, 996–1011):** LOOK-REC2 now counts the hull outline's second draw and the GTAO pre-pass as "planned extras". Its mark is 1.1 × (draws with all effects off + the planned extras), so it no longer contradicts LOOK-REC3.
- **L2 (closed for the hull, 1123, 1132–1139):** the arithmetic is shown and the trade is left to the brief. What it leaves out is item 2, A.
- **L3 (closed, 52, 821, 1025–1026):** the TL;DR, option C-B and CHAR-REC1 no longer say "rigid-skinned".
- **L4 (closed):** AUDIO-REC1 (1214–1222) and CHAR-REC2 (1043–1050) are now conditional on the camera.
- **L5:** no change. The reason is sound (the spot-check marked it note only).
- **C1 (closed, 1086–1106):** BUDGET-REC1's reason is rewritten. Intel Gen12+ now applies to Chrome on Linux only, Windows reaches WebGL 2 only through `forceWebGL`, and a Linux run is added for the real fallback. The new CPU floor is marked as a proposal with no source.
- **C2 (closed, 1038–1040, 1054–1055, 1116, 1139):** 2.0 ms is the cost with every soldier on the full rig and no budget LOD; 1.0 ms is the Low budget with the budget LOD. All three places say which.
- **C3 (closed, 1069–1083):** render bundles enter only with a measured gain, and the mark checks for it.
- **C4 (closed, 953–956):** the new reading of the 31 % figure is confirmed at the source (row 5).
- **C5 (closed):** "Rests on:" entries added at 930, 990, 1006, 1185–1186; R1 ids added at 40 and 1429.
- **C6 (closed):** "No §6 item" notes added at 919–920, 943, 977, 1103, 1198. The audio recommendations' "Serves:" lines are fixed at 1224, 1238, 1249–1250, 1261. I checked them against `docs/VISION_V2.md`: §3 has "12v12v12 or 24v24" (line 44), the file never mentions audio, and §6.2 is floors and line of sight.
- **C7 (turned into open questions):** GL2-OQ5 (1301), LOOK-OQ6 (1319), CHAR-OQ6 (1340), BUDGET-OQ6 (1357), AUDIO-OQ6 (1387). The reason is sound: an option row would need findings with sources, and the fix rule allowed no new claims.
- **C8 (closed, 272, 566, 632).**
- **C9 (closed, 1283, 566, 613, 707, 1079–1082, 1233).**
- **C10:** nothing to do.
- **C11 (closed, 1030–1031).**

**2. Rewritten recommendations**
All nine now follow from their findings and mark what is inferred or proposed. Every finding their reasons and marks use is named in "Rests on:". The 2.0 ms against 1.0 ms contradiction is gone. Still open:
- **A (minor).** LOOK-REC2 keeps the GTAO pre-pass open, but the Low draw-call cap in BUDGET-REC2 (1123, 1132–1139) handles only the hull. A pre-pass adds about one draw per drawn object: 192 for the soldiers, 80 for the large-unit parts, plus the world's main-pass share. That passes both 800 and 1,100 at Low, and the file does not say so. This is the same flaw as L2; one sentence fixes it.
- **B (minor).** CHAR-REC4's mark (1079–1082) caps four large units at 200 draws "against the scene without them" and never says "before outlines". BUDGET-REC2 (1134) adds about 80 hull draws for those same parts on top of the 200. If the brief keeps a hull or a GTAO pre-pass at High, the two marks disagree (about 240 against 200).

**3. Numbers:** all sourced, all arithmetic redone and correct, and every proposal is marked "proposed" (table below).

**4. New open questions:** three of the five state nothing new: GL2-OQ5, BUDGET-OQ6 (both its source checks confirmed) and AUDIO-OQ6. Two nits:
- LOOK-OQ6 names SMAA, but no finding or source backs it.
- CHAR-OQ6 says "either would be a WebGPU-only extra" without marking it inferred. GL2-F2 covers only compute dispatch counts, not indirect draws.

**5. Header, totals, part 7:**
- The totals (line 26: 92 findings / 25 recommendations / 38 open questions) match the file.
- M is still 77 after the fix (counted over all 25 "Rests on:" lists). Part 7 says "before the fix" and could say "unchanged".
- Part 7's final-state column is right (43 unchanged, 1 fixed). It does not yet record this second pass.
- No fix chooses a look or a set of look techniques. The outline, AO, anti-aliasing and shadow trades all stay with the look brief.

**Claims and numbers checked**

| # | Claim (short) | Source or arithmetic | Verdict | Location and exact words |
|---|---|---|---|---|
| 1 | MEASURE-F3: programs and render targets are counts, the other memory figures are bytes | threejs.org/docs/pages/Info.html | confirmed | "The number of active programs."; "The number of active renderTargets."; "The memory size of active textures in bytes."; "The total memory size in bytes."; "…They must then call `renderer.info.reset()` once per frame manually." |
| 2 | BUDGET-F6: the quotes (its last clause is labelled inferred) | same page | confirmed | "The number of draw calls of the current frame."; "The number of rendered triangle primitives of the current frame." |
| 3 | GL2-F11: WebGL 2 versions | raw BCD api/WebGL2RenderingContext.json, top-level __compat | confirmed | chrome "version_added": "56"; "edge": "mirror"; firefox "51"; safari "15" |
| 4 | WGPU-OQ5: release dates | github.com/mrdoob/three.js/releases | confirmed | "24 Sep 14:35"; "01 Jul 23:22"; "16 Apr 16:49"; "20 Feb 15:31"; plain rNNN tags in sequence |
| 5 | GL2-REC1: the 31 % is about Direct3D 11.1, the hardware compatibility mode targets | developer.chrome.com/blog/new-in-webgpu-139, "Origin trial for WebGPU compatibility mode" | confirmed | "However, a significant number of devices still lack support for these newer APIs."; "For example, on Windows, 31% of Chrome users don't have Direct3D 11.1 or higher."; "This mode is designed to run older graphics APIs like OpenGL ES 3.1 and Direct3D11" |
| 6 | LOOK-OQ6: FXAA needs sRGB input | threejs.org/docs/pages/RenderPipeline.html, outputColorTransform | confirmed | "A typical example is FXAA which requires sRGB input." |
| 7 | BUDGET-OQ6: r186 removed PCFSoftShadowMap | three.js wiki Migration-Guide, "185 → 186" | confirmed | "`PCFSoftShadowMap` with `WebGPURenderer` has been removed" |
| 8 | BUDGET-OQ6: legacy uses one 2048² map over 100 m | docs/LEGACY_INVENTORY.md:98 (CD17) | confirmed | "a 2048 x 2048 shadow map over a 100 m square frustum" |
| 9 | BUDGET-REC2: world 202 merged, 303 unmerged | LEGACY_INVENTORY.md:118 (CL9); AUDIT_BUGS_PERF.md:16 | confirmed (nit) | "village 398 to 202"; "Draw calls (main + shadow) … village 303". Nit: CS12 gives 398 as the count just before the merge, and 202 already includes the shadow pass and the world's hulls |
| 10 | CHAR-REC4: four large units in R1-TICK's scene | r1-simulation-core.md:2014 | confirmed | "plus 4 big vehicles and 200 live projectiles" |
| 11 | The R1 ids cited | r1:2583, :2088, :2167 | confirmed | R1-TICK-OQ6 "…need a named minimum laptop CPU"; R1-DET-REC1; R1-DET-REC6 |
| 12 | WGPU-OQ5: gaps of 55, 76 and 85 days | arithmetic | correct | 8+31+16 = 55; 14+31+30+1 = 76; 30+31+24 = 85 (non-leap year) |
| 13 | BUDGET-REC2: Low cap 786, about 1,060 with a hull outline | arithmetic | correct (proposed) | 4×48 = 192, ×2 = 384, +202+200 = 786; +192+80 = 1,058 |
| 14 | CHAR-REC4: 160 draws, 2.0 ms | arithmetic | correct (proposed) | 4×20×2 = 160; 1.5 ms × 4/3 = 2.0 ms |
| 15 | LOOK-OQ6: SMAA as an option | none | unsupported | no finding and no source read |
| 16 | CHAR-OQ6: GPU-driven culling or indirect draws would be WebGPU-only | GL2-F2 | unsupported (inference not marked) | F2 refuses indirect compute dispatch counts only, not indirect draws |

Marked proposed with no source, as they should be: the 4-core, 8-thread CPU floor; the Low cap of about 1,100; CHAR-REC4's 200 draws and 2.0 ms.

**Counts:** 14 confirmed (11 at a source, 3 arithmetic) / 0 wrong / 2 unsupported (both nits in open questions).

**Only nits left?** Not quite. A and B are minor, a step above nits: each needs one sentence and no new source, and the main session can check the wording without another verifier pass. Everything else left is a nit:
- SMAA (LOOK-OQ6);
- the unmarked inference in CHAR-OQ6;
- 202 counts the world's own hulls, so the "without a per-object outline pass" figure is conservative;
- 303 against 398 for the unmerged village;
- part 7 says M "before the fix" (it is unchanged) and does not record this pass.

**Header label the file should carry:** "spot-checked (45 of 77 key claims)". BUDGET-F6 is a key claim (in BUDGET-REC2's "Rests on:") and is now confirmed with quotes; M is still 77.

**Not verified:**
- The first pass's open items: claims off its list (WGPU-F2, F12, F13, F19–F21), the Firefox release dates, the maxDistance parts of AUDIO-F3, and the `trackTimestamp` doc string.
- No source gives GPU timings, so none could be checked.
- The year in WGPU-F1 (2026) is inferred: the releases page shows no year.
