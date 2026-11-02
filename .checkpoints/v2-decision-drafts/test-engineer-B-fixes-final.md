# test-engineer unit B, final fix round: pass marks of ADR 0003, 0004, 0005, 0006 (doc text at 75615d1)

### FIX PA-4, GE-6, GE-2, PA-14 (the intro: the four rows added)
FILE: docs/adr/0003-rendering.md
BEFORE:
Written by test-engineer (plan step 4). One row per mark: the look proof test's first (PM-0003-1 to PM-0003-17), then
the render budget bench's (PM-0003-18 to PM-0003-42), then the check at each later three.js upgrade (PM-0003-43). Every
AFTER:
Written by test-engineer (plan step 4). One row per mark: the look proof test's first (PM-0003-1 to PM-0003-17), then
the render budget bench's (PM-0003-18 to PM-0003-42, then PM-0003-44 to PM-0003-47, added at the final round), then
the check at each later three.js upgrade (PM-0003-43). Every
END

### FIX EA-11 (routed: BS records the Worker's load), AD-5 (the test view)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **Scenes.** **BS**, the benchmark scene (part 8; R3-MEASURE-REC2, lines 1195–1197): a recorded match replayed at the
  stress scene's fight on screen (N = 48 soldiers in three teams of 16, the 4 very large units of at least 20 parts
  each, 300 live projectiles with their visual effects), with a scripted camera, started by a URL flag. Until ADR
  0004's sim core spike gives a recorded match, BS is a synthetic recording at the same counts (the draft's own
  fallback, the bench's "Needs"). **LS**, the look-test scene ("one look-test scene", R3-LOOK-REC1, line 991): the look
  proof test's scripted scene with its authored test assets (part 11) at the game camera, at BS's counts wherever a
  mark asks for the fight on screen. It runs before any recorded match exists, so it is a synthetic recording (one
  script, one fixed seed).
AFTER:
- **Scenes.** **BS**, the benchmark scene (part 8; R3-MEASURE-REC2, lines 1195–1197): a recorded match replayed at the
  stress scene's fight on screen (N = 48 soldiers in three teams of 16, the 4 very large units of at least 20 parts
  each, 300 live projectiles with their visual effects), with a scripted camera, started by a URL flag. Until ADR
  0004's sim core spike gives a recorded match, BS is a synthetic recording at the same counts (the draft's own
  fallback, the bench's "Needs"). Every BS run records the Worker's load beside its numbers: a replay, a local match or
  an online client (part 8: a replay runs the whole sim at N = 48, as a local match does, which is heavier than an
  online client's Worker); a result holds for the load it ran with, and once the boundary spike's online client
  exists PM-0003-18 runs once more with that load, reported beside it (proposed: the load a networked player's laptop
  carries). **LS**, the look-test scene ("one look-test scene", R3-LOOK-REC1, line 991): the look proof test's
  scripted scene with its authored test assets (part 11) at the test view, at BS's counts wherever a mark asks for the
  fight on screen. It runs before any recorded match exists, so it is a synthetic recording (one script, one fixed
  seed).
- **The test view.** Wherever a row or a checklist step says "the game camera", it means legacy's framing as a test
  view: height 40, boom 56.6, field of view 15° and yaw 45° (CD5, `LEGACY_INVENTORY.md` line 86). It decides no camera:
  the v2 framing is a design question (R2-INT-OQ2, r2-netcode.md lines 3010–3013), and once it is answered a mark or a
  step judged at the test view is taken again at the v2 framing (proposed: silhouettes, the ink's width in pixels and
  LOD all depend on the view).
END

### FIX PA-6 (the p99 rows' tail form and its validity rule), PA-7 (the soaked set; OQ-0003-20)
FILE: docs/adr/0003-rendering.md
BEFORE:
- **H**, a harness run set (part 8; R3-MEASURE-REC1, lines 1181–1185): 3 runs of 600 measured frames after 300
  warm-up frames, each run labelled as R3-MEASURE-REC1 asks (backend, adapter, browser version, `devicePixelRatio`,
  resolution, GPU times quantized or not) plus the build's commit, the display's refresh rate, mode U or P, the tier,
  and mains power with the operating system's power mode (proposed: a laptop's power state moves its numbers). Every
  one of the 3 runs must meet the mark ("holds … for 3 runs", R3-BUDGET-REC2, lines 1162–1163).
- **Statistics.** Percentiles by nearest rank over a run's 600 frames: p99 is the 594th smallest value, p95 the 570th,
  the median the 300th (proposed: one definition, so two people get the same number). A time mark that names no
  statistic is a p99 ([time statistics](../research/GLOSSARY.md#time-statistics)); p99s are never added, so every
  total is measured directly. A count mark holds on every frame, so its statistic is the run's max (proposed reading:
  counts are exact, and the replay repeats them, PM-0003-35).
- **V**, the validity of a timed run set: its 3 run medians lie within 3 % of each other (R3-MEASURE-REC1's mark,
  lines 1192–1193, applied to every timed set: proposed). An invalid set is neither a pass nor a fail: its cause is
  found and written down before it is measured again.
AFTER:
- **H**, a harness run set (part 8; R3-MEASURE-REC1, lines 1181–1185): 3 runs of 600 measured frames after 300
  warm-up frames, each run labelled as R3-MEASURE-REC1 asks (backend, adapter, browser version, `devicePixelRatio`,
  resolution, GPU times quantized or not) plus the build's commit, the display's refresh rate, mode U or P, the tier,
  the Worker's load on BS, and mains power with the operating system's power mode (proposed: a laptop's power state
  moves its numbers). Every one of the 3 runs must meet the mark ("holds … for 3 runs", R3-BUDGET-REC2, lines
  1162–1163). **H's tail form:** where a mark's number is a p99 of per-frame values from H (PM-0003-11, -19, -20, -21,
  -26, -27, -30, -33 and -40), each of the 3 runs measures 3,000 frames after the same 300 warm-up frames (proposed: at
  600 frames a p99 is the 6th-largest frame, so one garbage-collection pause or one operating-system task inside a
  20-second run decides a 1.0 ms row; at 3,000 frames 30 frames lie above it, and a run takes 100 s at 30 fps). The
  median and p95 marks keep R3's 600 frames.
- **S, the soaked set** (ML only): the same run set taken again after at least 15 minutes of BS looping with the
  Worker's sim running, in one browser session with no restart, mains power and power mode recorded (proposed: a match
  runs about 20 minutes, LOOP_V2 §8, and in a thin laptop the CPU and the integrated GPU share one power and heat
  budget, so a run on a cool laptop can fit inside a short boost window; the chosen laptop's boost behaviour is not
  researched). PM-0003-18 to -21 run S on ML beside their cold set. Which of the two gates is OQ-0003-20; until it is
  answered the soaked set gates and the cold set is reported beside it (its recommended option, held as the interim).
- **Statistics.** Percentiles by nearest rank over a run's 600 frames: p99 is the 594th smallest value, p95 the 570th,
  the median the 300th; over the tail form's 3,000 frames, the 2,970th, 2,850th and 1,500th (⌈q × n⌉, arithmetic;
  proposed: one definition, so two people get the same number). A time mark that names no
  statistic is a p99 ([time statistics](../research/GLOSSARY.md#time-statistics)); p99s are never added, so every
  total is measured directly. A count mark holds on every frame, so its statistic is the run's max (proposed reading:
  counts are exact, and the replay repeats them, PM-0003-35).
- **V**, the validity of a timed run set: its 3 run medians lie within 3 % of each other (R3-MEASURE-REC1's mark,
  lines 1192–1193, applied to every timed set: proposed), and, for a set in the tail form, its 3 run p99s lie within
  10 % of their median (proposed: the medians cannot see the tail; 10 % is half the smallest p99 change a row here
  compares, PM-0003-27's 20 %, as ADR 0001's V-S takes half its tightest tolerance). An invalid set is neither a pass
  nor a fail: its cause is found and written down before it is measured again.
END

### FIX GE-14 (PM-0003-2: the device's error events counted)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-2 | One build runs on both backends in every target browser (part 1) | 0 console errors in every start | ML: Chrome, Edge and Firefox (Windows), each on WebGPU and with `forceWebGL`; MAC: Safari, both backends | LS played through its whole script; the page's own error hook writes every console error and page error to the results log (no browser driver needed) | the 10 starts of PM-0003-5 per browser and backend; 0 in each. Bites: a `console.error` 2 s after load (R5-GATE-REC3's G12 mutation, r5-toolchain-quality.md line 1723) | R3-WGPU-REC1's mark 1 (lines 910–912) |
AFTER:
| PM-0003-2 | One build runs on both backends in every target browser (part 1) | 0 console errors and 0 device errors in every start | ML: Chrome, Edge and Firefox (Windows), each on WebGPU and with `forceWebGL`; MAC: Safari, both backends | LS played through its whole script; the page's own error hook writes every console error and page error to the results log (no browser driver needed), and counts as a device error each `uncapturederror` event of the WebGPU device, its `lost` promise resolving, and each `webglcontextlost` event on the canvas (proposed, not researched: a WebGPU validation error reaches the page through the device's error event rather than as a thrown error, and whether each browser also prints it to the console, as an error, a warning or not at all, was not verified; the spike records how each browser shows the planted one) | the 10 starts of PM-0003-5 per browser and backend; 0 in each. Bites: a `console.error` 2 s after load (R5-GATE-REC3's G12 mutation, r5-toolchain-quality.md line 1723); on the WebGPU backend, a planted bind group on the renderer's device that leaves out one entry of its layout, which must count as a device error whatever the console shows | R3-WGPU-REC1's mark 1 (lines 910–912); the device events proposed (the graphics review's finding 14) |
END

### FIX TE-5 (PM-0003-3: every kept effect on; routed by graphics-engineer)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-3 | The fallback shows the same look (parts 1, 5; decides R3-GL2-OQ3) | at each of 3 fixed frames, at most 1 % of pixels differ by more than 8/255 in any channel between the two backends (a tolerance the research marks proposed); effects that exist on one backend only are off | ML: Chrome and Firefox, WebGPU against `forceWebGL`; MAC: Safari, the same pair | LS's three fixed frames read from the canvas on each backend, same machine and browser; a pixel-diff script counts the pixels with any channel difference above 8 | 1 capture per frame and backend. Control: the same backend captured twice gives 0 differing pixels (else the scene does not repeat and the set is invalid). Bites: the ink switched off on the WebGL 2 backend only | R3-WGPU-REC1's mark 2 (lines 912–913); R3-GL2-REC1's mark (lines 965–967) |
AFTER:
| PM-0003-3 | The fallback shows the same look (parts 1, 5; decides R3-GL2-OQ3) | at each of 3 fixed frames, at most 1 % of pixels differ by more than 8/255 in any channel between the two backends (a tolerance the research marks proposed), with every kept effect on: under F-A no effect exists on one backend only (part 1), and an F-B extra, if one ever enters, is compared through its named fallback | ML: Chrome and Firefox, WebGPU against `forceWebGL`; MAC: Safari, the same pair | LS's three fixed frames read from the canvas on each backend, same machine and browser; a pixel-diff script counts the pixels with any channel difference above 8 | 1 capture per frame and backend. Control: the same backend captured twice gives 0 differing pixels (else the scene does not repeat and the set is invalid). Bites: the ink switched off on the WebGL 2 backend only | R3-WGPU-REC1's mark 2 (lines 912–913), changed: its "post effects that exist on one backend only are switched off" (line 913) would remove the failure part 1 forbids, an effect that silently does not run on WebGL 2; R3-GL2-REC1's mark (lines 965–967) |
END

### FIX GE-9 (PM-0003-6: a status per frame record, tied to its own query)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-6 | GPU timing on the fallback never fakes a number (part 4) | Chrome's WebGL 2 backend: a GPU time above 0 in 600 of 600 frame records; Firefox's and Safari's WebGL 2 backends: "GPU time not available" in 600 of 600, never 0 | ML: Chrome and Firefox with `forceWebGL`; ML-Linux: Firefox; MAC: Safari with `forceWebGL` | one H run on LS per browser; the frame records read from the results log | 1 run per browser (a count over every frame). Bites: a harness that writes 0 when the timer is missing fails on Firefox | R3-GL2-REC3's mark (lines 984–985) |
AFTER:
| PM-0003-6 | GPU timing on the fallback never fakes a number (part 4) | every frame record carries a status: timed (from the query its own frame issued), disjoint, or "GPU time not available". Chrome's WebGL 2 backend: at least 99 % of the 600 records timed (proposed: a disjoint frame is an event the extension itself reports, so 600 of 600 could fail a correct harness), every timed value above 0 and tied to its own frame, the disjoint records counted and reported; Firefox's and Safari's WebGL 2 backends: "GPU time not available" in 600 of 600, never 0 or a number | ML: Chrome and Firefox with `forceWebGL`; ML-Linux: Firefox; MAC: Safari with `forceWebGL` | one H run on LS per browser; the frame records read from the results log. three.js reads timestamps later (`async resolveTimestampsAsync`, R3-MEASURE-F4, lines 624–631), so each record names the frame number its query was issued in, and a timed record counts only when that number is its own | 1 run per browser (a count over every frame). Bites: a harness that writes 0 when the timer is missing fails on Firefox; a harness that copies the last resolved value into each new frame's record fails the tie on Chrome | R3-GL2-REC3's mark (lines 984–985); R3-MEASURE-F4 (lines 624–631); the status per record proposed (the graphics review's finding 9) |
END

### FIX GE-3 (PM-0003-7: the pipeline count beside programs)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-7 | The match compiles nothing after the loading screen (part 3) | `renderer.info.memory.programs` equal 5 s after deploy and at the end of a scripted 5-minute match | CI (a count); ML: Chrome on WebGPU and with `forceWebGL`, Firefox on WebGPU | the 5-minute script shows every unit type, visual effect, damage stage and roof fade at least once (part 3's warm-up list); the counter read at both points | 3 runs; equal in each. Bites: the roof fade's blend variant left out of the warm-up | R3-WGPU-REC3's mark 1 (lines 937–938); R3-MEASURE-F3 (lines 617–622) |
AFTER:
| PM-0003-7 | The match compiles nothing after the loading screen (part 3) | `renderer.info.memory.programs` equal 5 s after deploy and at the end of a scripted 5-minute match, and so is the pipeline count: the calls to `createRenderPipeline` and `createRenderPipelineAsync` on the WebGPU device, and to `linkProgram` on WebGL 2 (proposed: a blend variant that reuses its shader adds a render pipeline while `programs` stays the same, r3-rendering.md, Findings Q1, legacy facts, lines 201–202, so `programs` alone could not fail on the bite below) | CI (a count); ML: Chrome on WebGPU and with `forceWebGL`, Firefox on WebGPU | the 5-minute script shows every unit type, visual effect, damage stage and roof fade at least once (part 3's warm-up list); both counters read at both points, the pipeline count from a spy that wraps those calls in the measurement build (proposed) | 3 runs; equal in each. Bites: the roof fade's blend variant left out of the warm-up must change the pipeline count on the WebGPU backend (ML: Chrome); on WebGL 2 a blend state is not a program (inferred, not researched), so there the bite is PM-0003-8's material created at a unit type's first sight | R3-WGPU-REC3's mark 1 (lines 937–938); R3-MEASURE-F3 (lines 617–622); R3-WGPU-F22 (lines 188–192: the async pipeline call) |
END

### FIX GE-3 (PM-0003-33: the pipeline count across the tier switch)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-33 | The quality tiers bite (part 7) | switching High to Low cuts GPU frame time by at least 50 % (p99: no statistic named, GLOSSARY §1), with `programs` unchanged across the switch | ML: Chrome on WebGPU and with `forceWebGL` | H on BS at High, then at Low after a live switch | H, V. Bites: a Low tier whose table equals High's | R3-BUDGET-REC3's mark (lines 1176–1177) |
AFTER:
| PM-0003-33 | The quality tiers bite (part 7) | switching High to Low cuts GPU frame time by at least 50 % (p99: no statistic named, GLOSSARY §1), with `programs` and PM-0003-7's pipeline count unchanged across the switch (proposed: a tier that changes MSAA samples or a target's format needs other pipelines, inferred from r3-rendering.md lines 201–202 and not researched, which `programs` cannot see) | ML: Chrome on WebGPU and with `forceWebGL` | H on BS at High, then at Low after a live switch; both counters read before and after the switch | H, V. Bites: a Low tier whose table equals High's; Low's pipeline variants (its MSAA samples) left out of the warm-up must change the pipeline count | R3-BUDGET-REC3's mark (lines 1176–1177) |
END

### FIX AD-10, PA-10 (routed by graphics-engineer: PM-0003-9's overlay as frame time, memory at both sample counts)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-9 | Each look technique's cost, alone (part 5; decides R3-LOOK-OQ1 and R3-LOOK-OQ4) | reported, no pass number: per technique and variant, GPU ms (median and p95), extra draw calls, extra render-target memory (`renderer.info.memory`, with `texturesSize`); passes when every cell holds a number or "not available" | ML at Low and RD at High, Chrome on WebGPU and with `forceWebGL`, 1920 × 1080 | LS with only that technique on, against LS with every technique off; the techniques of part 5: toon ramps (5.a), both ink families (5.b), AO (5.c), glow (5.d), the shadow map (5.f), the roof fade (5.i), visual effects (5.k), halftone and line boil (5.l), MSAA (5.n) | H per technique and variant, V | R3-LOOK-REC1's mark (lines 998–1000); R3-LOOK-OQ4 (lines 1330–1331) |
AFTER:
| PM-0003-9 | Each look technique's cost, alone (part 5; decides R3-LOOK-OQ1 and R3-LOOK-OQ4) | reported, no pass number: per technique and variant, GPU ms (median and p95), extra draw calls, extra render-target memory (`renderer.info.memory`, with `texturesSize`), each attachment's memory given both single-sampled and at 4× MSAA (5.d, 5.n); the halftone overlay costed as frame time (median and p95) with it and without it, since the browser composites a CSS layer outside the renderer's timestamp queries (5.l); passes when every cell holds a number or "not available" | ML at Low and RD at High, Chrome on WebGPU and with `forceWebGL`, 1920 × 1080 | LS with only that technique on, against LS with every technique off; the techniques of part 5: toon ramps (5.a), both ink families (5.b), AO (5.c), glow (5.d), the shadow map (5.f), the roof fade (5.i), visual effects (5.k), halftone and line boil (5.l), MSAA (5.n); the overlay's frame time in mode U | H per technique and variant, V | R3-LOOK-REC1's mark (lines 998–1000); R3-LOOK-OQ4 (lines 1330–1331); R3-LOOK-F14 (lines 405–409: 4 × MSAA multiplies the multisampled attachments by four); CD16's "not a render pass" (`LEGACY_INVENTORY.md` line 97) |
END

### FIX PA-8 (routed by graphics-engineer: PM-0003-12 points to part 7's 799 to 817)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-12 | Both ink families are measured, and each fits its draw cap at Low (part 5.b; part 7) | reported: GPU and frame time (median, p95) at 12, 24 and 48 soldiers on screen for each family; gated: at 48 soldiers, `drawCalls` at most 800 with the screen-space ink and at most 1,100 with the hull (the research's "about 1,100", read as 1,100), on every frame. The count is the whole frame's, full-screen post passes included (the ink's own pass and any other pass the scene runs; the same counter, so the same holds in PM-0003-22 and PM-0003-32): R3's budget is for "all passes of one frame", and `drawCalls` is "The number of draw calls of the current frame." R3's 786 and 1,058 count no post pass, which leaves 14 draws for them under 800 (800 − 786) and 42 under 1,100 (1,100 − 1,058); bloom alone, if the look brief keeps it, takes 12. Reported beside it: the post passes' share (the same frame counted with post-processing off, subtracted; 0 would mean `drawCalls` does not count them, which is not researched) | ML at Low: Chrome on WebGPU; ML-Linux: Firefox (the count's worst case, R3-GL2-REC2) | LS at 12, 24 and 48 soldiers on screen, each family alone, mode U | H, V for the times; every frame for the counts. Bites: a fifth draw per soldier per pass (5 × 48 × 2 = 480 instead of 384: 882 draws with the screen-space ink and 1,202 with the hull, which outlines 48 more objects, both before post passes) | R3-LOOK-REC3's mark (lines 1026–1027); R3-BUDGET-REC2 (lines 1133, 1142–1148): 4 × 48 × 2 + 202 + 200 = 786; 786 + 192 + 80 = 1,058; its table head, "all passes of one frame" (line 1121); R3-MEASURE-F3 (lines 617–618); R3-LOOK-F13, bloom's 12 passes, a count R3 inferred (lines 397–402) |
AFTER:
| PM-0003-12 | Both ink families are measured, and each fits its draw cap at Low (part 5.b; part 7) | reported: GPU and frame time (median, p95) at 12, 24 and 48 soldiers on screen for each family; gated: at 48 soldiers, `drawCalls` at most 800 with the screen-space ink and at most 1,100 with the hull (the research's "about 1,100", read as 1,100), on every frame. The count is the whole frame's, full-screen post passes included (the ink's own pass and any other pass the scene runs; the same counter, so the same holds in PM-0003-22 and PM-0003-32): R3's budget is for "all passes of one frame", and `drawCalls` is "The number of draw calls of the current frame." R3's 786 and 1,058 count neither the post passes nor the frame's other draws; part 7's sum of the whole frame (the health bars, the pooled effect kinds, the projectile kinds, and 2 to 16 post passes) is 799 to 817 with the screen-space ink and 1,071 to 1,089 with the hull, so 800 holds only with the leanest post chain and unshadowed projectiles; if the Low tier's kept effects do not fit, the draft weighs a cap of about 820 against a leaner Low tier, and switching an effect off follows OQ-0003-16 (part 7). Reported beside it: the post passes' share (the same frame counted with post-processing off, subtracted; 0 would mean `drawCalls` does not count them, which is not researched) | ML at Low: Chrome on WebGPU; ML-Linux: Firefox (the count's worst case, R3-GL2-REC2) | LS at 12, 24 and 48 soldiers on screen, each family alone, mode U | H, V for the times; every frame for the counts. Bites: a fifth draw per soldier per pass (5 × 48 × 2 = 480 instead of 384: 882 draws with the screen-space ink and 1,202 with the hull, which outlines 48 more objects, both before post passes) | R3-LOOK-REC3's mark (lines 1026–1027); R3-BUDGET-REC2 (lines 1133, 1142–1148): 4 × 48 × 2 + 202 + 200 = 786; 786 + 192 + 80 = 1,058; its table head, "all passes of one frame" (line 1121); R3-MEASURE-F3 (lines 617–618); part 7's whole-frame sum: 786 + 2 + 5 + (4 to 8) + (2 to 16) = 799 to 817 |
END

