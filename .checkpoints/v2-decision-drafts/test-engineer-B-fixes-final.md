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

### FIX PA-10 (routed: PM-0003-16 records the MSAA memory), GE-8 (MP's devicePixelRatio), GE-14 (how each browser shows a device error)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-16 | The look proof test's records are complete (parts 1, 4, 5; decides R3-WGPU-OQ2, R3-MEASURE-OQ1, R3-MEASURE-OQ3) | recorded: `renderer.hasFeature` for timestamp queries on Firefox and Safari; `renderer.info.memory` and the adapter's `memoryHeaps` (Chrome's developer flag on) on ML; Brave's backend on MP (PM-0003-5) | ML, MAC, MP | read once per browser and written to the results log | passes when complete | R3-WGPU-OQ2 (lines 1290–1293); R3-MEASURE-OQ1 (lines 1382–1383); R3-MEASURE-OQ3 (lines 1386–1388) |
AFTER:
| PM-0003-16 | The look proof test's records are complete (parts 1, 4, 5; decides R3-WGPU-OQ2, R3-MEASURE-OQ1, R3-MEASURE-OQ3) | recorded: `renderer.hasFeature` for timestamp queries on Firefox and Safari; `renderer.info.memory` and the adapter's `memoryHeaps` (Chrome's developer flag on) on ML; whether `renderer.info.memory.total` changes when MSAA is switched on, and by how much against 5.n's arithmetic (199.2 MB of multisampled attachments at 1920 × 1080: not researched, 5.n); how each browser shows PM-0003-2's planted device error (console error, warning or nothing); Brave's backend on MP (PM-0003-5), and MP's `devicePixelRatio` and screen resolution (step 28 depends on them) | ML, MAC, MP | read once per browser and written to the results log; the MSAA reading as `info.memory.total` on LS with MSAA off, then at 4 samples | passes when complete | R3-WGPU-OQ2 (lines 1290–1293); R3-MEASURE-OQ1 (lines 1382–1383); R3-MEASURE-OQ3 (lines 1386–1388); R3-BUDGET-F3 (lines 551–555: `devicePixelRatio`); R3-LOOK-F14 (lines 405–409) |
END

### FIX PA-14 (PM-0003-19: what the CPU span cannot see, and its HUD count)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-19 | The main-thread CPU rows hold (parts 6, 7) | CPU per frame p99 at most 16 ms (ML, Low) and 8 ms (RD, High); of which character animation (posing and bone upload, budget LOD on) p99 at most 1.0 ms (ML) and 2.0 ms (RD) | as PM-0003-18 | H on BS; per frame, `performance.now()` from the animation-loop callback's start to the end of its work, plus that frame's block message handler; animation timed around posing and bone upload, on a build with the isolation flag on | H, V; rule B | R3-BUDGET-REC2's rows (lines 1125–1126); R3-CHAR-REC2's mark (lines 1061–1062) |
AFTER:
| PM-0003-19 | The main-thread CPU rows hold (parts 6, 7) | CPU per frame p99 at most 16 ms (ML, Low) and 8 ms (RD, High); of which character animation (posing and bone upload, budget LOD on) p99 at most 1.0 ms (ML) and 2.0 ms (RD) | as PM-0003-18 | H on BS; per frame, `performance.now()` from the animation-loop callback's start to the end of its work, plus that frame's block message handler; animation timed around posing and bone upload, on a build with the isolation flag on. The span cannot see the style, layout and paint the browser runs after the callback for the DOM HUD (legacy measured about 30 DOM writes per frame as a cost, `docs/AUDIT_BUGS_PERF.md` line 27, P3), which show only in frame time (PM-0003-18), so PM-0003-47 counts the HUD's DOM writes beside this row | H in its tail form, V; rule B; S on ML | R3-BUDGET-REC2's rows (lines 1125–1126); R3-CHAR-REC2's mark (lines 1061–1062) |
END

### FIX AD-1, GE-4 (routed by graphics-engineer: PM-0003-23's shadow map and the look floor)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-23 | The tiers are set as the table says (part 7) | Low: pixel ratio at most 1, scene resolution scale 0.75 to 1, one shadow map at 1024² or none; High: pixel ratio at most 2 | ML, RD | read from the renderer (its pixel ratio, the scene pass's resolution scale, the shadow map's size) at the start and end of every BS run | every run of PM-0003-18. Bites: a tier file that gives Low a ratio of 2 | R3-BUDGET-REC2's rows (lines 1129, 1136); R3-BUDGET-REC3 (lines 1166–1169) |
AFTER:
| PM-0003-23 | The tiers are set as the table says (part 7) | Low: pixel ratio at most 1, scene resolution scale 0.75 to 1, one 1024² shadow map (none only on Mustafa's answer to OQ-0003-16), and every technique that carries a LOOK_V2 `[DECIDED]` item on (the toon materials, the ink, the sun shadow map and the emissive slots' colour: part 7's look floor, unless OQ-0003-16 answers otherwise); High: pixel ratio at most 2 | ML, RD | read from the renderer (its pixel ratio, the scene pass's resolution scale, the shadow map's size) and from the tier table's on/off switches at the start and end of every BS run | every run of PM-0003-18. Bites: a tier file that gives Low a ratio of 2; a Low row with the shadow map switched off | R3-BUDGET-REC2's rows (lines 1129, 1136); R3-BUDGET-REC3 (lines 1166–1169; its "Which effects exist at all is the look brief's call", lines 1172–1173, is the look floor, part 7) |
END

### FIX PA-7 (PM-0003-18: the soaked set on ML)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-18 | The frame-time rows hold (part 7) | ML at Low: frame time median at most 28 ms, p95 at most 33.3 ms; RD at High: median at most 14 ms, p95 at most 16.7 ms | ML: Chrome and Firefox, each on WebGPU and with `forceWebGL`; RD: Chrome on WebGPU at 1920 × 1080 and 2560 × 1440 | H on BS with the kept techniques, mode U | H, V; rule B | R3-BUDGET-REC2's rows (lines 1123–1124) and its mark (lines 1162–1164) |
AFTER:
| PM-0003-18 | The frame-time rows hold (part 7) | ML at Low: frame time median at most 28 ms, p95 at most 33.3 ms; RD at High: median at most 14 ms, p95 at most 16.7 ms | ML: Chrome and Firefox, each on WebGPU and with `forceWebGL`; RD: Chrome on WebGPU at 1920 × 1080 and 2560 × 1440 | H on BS with the kept techniques, mode U | H, V; rule B; S on ML | R3-BUDGET-REC2's rows (lines 1123–1124) and its mark (lines 1162–1164) |
END

### FIX PA-6, PA-7 (PM-0003-20: the tail form, the soaked set on ML)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-20 | The page's cost of the sim (part 7, N12; part 10) | p99 at most 1.0 ms per frame on both machines. **Which statistic gates (decision map LE-B): both, each in its own spike:** this p99 here, and R1-BND-REC1's mark 1 (the page's p95 frame time with the sim in the Worker no worse than with no sim by more than 1.0 ms, at N = 12, 24, 36, 48 on ML) in ADR 0002's boundary spike. They measure different things: this row times the page's own sim work; R1's difference also catches what timing the page's code cannot see (garbage collection, the Worker sharing the laptop's cores). Passing one does not imply the other | ML at Low and RD at High: Chrome on WebGPU; ML: Firefox on WebGPU | H on BS with the boundary spike's Worker and block (its sim at 30 Hz); per frame, `performance.now()` around the page's block intake (message handler, typed-array views, returning old blocks to the pool) and the command post; isolation flag on | H, V; rule B | R3-BUDGET-REC2's row (line 1127); R1-BND-REC1's mark 1 (r1-simulation-core.md, lines 2259–2261); GLOSSARY §1, the client's sim share (lines 214–218) |
AFTER:
| PM-0003-20 | The page's cost of the sim (part 7, N12; part 10) | p99 at most 1.0 ms per frame on both machines. **Which statistic gates (decision map LE-B): both, each in its own spike:** this p99 here, and R1-BND-REC1's mark 1 (the page's p95 frame time with the sim in the Worker no worse than with no sim by more than 1.0 ms, at N = 12, 24, 36, 48 on ML) in ADR 0002's boundary spike. They measure different things: this row times the page's own sim work; R1's difference also catches what timing the page's code cannot see (garbage collection, the Worker sharing the laptop's cores). Passing one does not imply the other | ML at Low and RD at High: Chrome on WebGPU; ML: Firefox on WebGPU | H on BS with the boundary spike's Worker and block (its sim at 30 Hz); per frame, `performance.now()` around the page's block intake (message handler, typed-array views, returning old blocks to the pool) and the command post; isolation flag on | H in its tail form, V; rule B; S on ML | R3-BUDGET-REC2's row (line 1127); R1-BND-REC1's mark 1 (r1-simulation-core.md, lines 2259–2261); GLOSSARY §1, the client's sim share (lines 214–218) |
END

### FIX PA-6, PA-7 (PM-0003-21: the tail form, the soaked set on ML)
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-21 | The GPU-time rows hold (part 7; with OQ-0003-4, decides R3-BUDGET-OQ6's shadow row) | p99, ML at Low / RD at High: total at most 25 / 12 ms; shadow pass 3 / 2 ms; main scene pass 16 / 6 ms; post-processing (ink, AO, glow, comic touches) 4 / 3 ms; visual effects, sprites and HUD in the 3D view 2 / 1 ms; each row measured directly, never as a sum | ML: Chrome on WebGPU and with `forceWebGL`; RD: Chrome on WebGPU at both resolutions; Firefox only where PM-0003-16 found timestamp queries | H on BS; the total and each pass from timestamp queries, developer flag on (WebGPU backend) | H, V; rule B (an extra full-screen pass) | R3-BUDGET-REC2's rows (lines 1128–1132); R3-BUDGET-OQ6 (lines 1375–1378) |
AFTER:
| PM-0003-21 | The GPU-time rows hold (part 7; with OQ-0003-4, decides R3-BUDGET-OQ6's shadow row) | p99, ML at Low / RD at High: total at most 25 / 12 ms; shadow pass 3 / 2 ms; main scene pass 16 / 6 ms; post-processing (ink, AO, glow, comic touches) 4 / 3 ms; visual effects, sprites and HUD in the 3D view 2 / 1 ms; each row measured directly, never as a sum | ML: Chrome on WebGPU and with `forceWebGL`; RD: Chrome on WebGPU at both resolutions; Firefox only where PM-0003-16 found timestamp queries | H on BS; the total and each pass from timestamp queries, developer flag on (WebGPU backend) | H in its tail form, V; rule B (an extra full-screen pass); S on ML | R3-BUDGET-REC2's rows (lines 1128–1132); R3-BUDGET-OQ6 (lines 1375–1378) |
END

### FIX PA-12 (PM-0003-36: the allocation check's method and bite), GE-2 (routed by engine-architect: the backend per CI engine), GE-3, GE-14
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-36 | CI catches render regressions, never with a time (part 8; the gate v2's check G12) | the CI job fails on a planted +10 % in draw calls and on a planted +10 % in GPU memory in BS (1 of 1 each); a control under the limit (+5 %, proposed) and R5-GATE-REC3's G12 control pass; the job runs in under 5 minutes; it also fails on a console error, on triangles or programs beyond the committed baseline file, and on any per-frame allocation in the render path | CI (headless; counts only) | BS in headless Chromium against the committed baseline file (R5-GATE-REC4); proven by `gate --prove`'s mutations and controls (R5-GATE-REC3) | every `--prove` run; 1 of 1 per mutation | R3-MEASURE-REC3's mark (lines 1214–1215); R5-GATE-REC1's G12 (r5-toolchain-quality.md line 1670); R5-GATE-REC3 (r5-toolchain-quality.md, lines 1706–1732; G12's row, line 1723); R5-GATE-REC4 (r5-toolchain-quality.md, lines 1734–1744) |
AFTER:
| PM-0003-36 | CI catches render regressions, never with a time (part 8; the gate v2's check G12) | the CI job fails on a planted +10 % in draw calls and on a planted +10 % in GPU memory in BS (1 of 1 each); a control under the limit (+5 %, proposed) and R5-GATE-REC3's G12 control pass; the job runs in under 5 minutes; it also fails on a console error or a device error (PM-0003-2's hook), on triangles, programs or PM-0003-7's pipeline count beyond the committed baseline file, and on any per-frame allocation in the render path; every run records the backend each CI engine gets, or none (ADR 0002 part 9: probably WebGL 2 everywhere on a GPU-less runner), and the WebGPU backend's own route is PM-0003-46 | CI (headless; counts only) | BS in headless Chromium against the committed baseline file (R5-GATE-REC4); proven by `gate --prove`'s mutations and controls (R5-GATE-REC3). The allocation check uses PM-0003-29's method: an inspector sampling heap profile that keeps the objects minor and major collections free (legacy TL15, `tools/perfbots.js` lines 9–10), over 60 s of BS, with the allocations attributed to the render path's functions | every `--prove` run; 1 of 1 per mutation. Bites (the allocation check): one planted 64-byte allocation per frame in the render path must be reported, as in PM-0003-29 (a sampling profile proves "zero" only if it sees a small leak) | R3-MEASURE-REC3's mark (lines 1214–1215); R5-GATE-REC1's G12 (r5-toolchain-quality.md line 1670); R5-GATE-REC3 (r5-toolchain-quality.md, lines 1706–1732; G12's row, line 1723); R5-GATE-REC4 (r5-toolchain-quality.md, lines 1734–1744) |
END

### FIX PA-4 (PM-0003-44, the presented-frame count; OQ-0003-17), GE-6 (PM-0003-45, teardown between matches), GE-2 (PM-0003-46, the WebGPU route; routed by engine-architect), PA-14 (PM-0003-47, the HUD's DOM writes)
FILE: docs/adr/0003-rendering.md
BEFORE:
| **Later, at each three.js upgrade** | | | | | | |
AFTER:
| PM-0003-44 | The very large units drop no frame (part 7's presented-frame count; VISION_V2 §6.9; feeds OQ-0003-17) | per run of 600 presented frames with the four very large units and their visual effects on screen: the frames whose interval passes 1.5 refresh periods, 25 ms on a 60 Hz display (1.5 × 16.7), counted. Gated at the share OQ-0003-17 sets; until it is answered, reported, with 0 of 600 as the proposed reading. On ML at Low the same count reported against a 33.3 ms period (intervals over 1.5 × 33.3 = 50 ms) | RD at High, Chrome on WebGPU, on a 60 Hz display (its rate recorded); ML at Low, Chrome on WebGPU (reported) | BS with the units and their effects on screen, mode P (normal vsync); each frame's interval from successive animation-frame timestamps (the loop follows the display, R3-BUDGET-F2, lines 545–549; that timestamp has "a minimal precision of 1 millisecond", R3-MEASURE-F7, lines 641–646, well inside the 8.3 ms between one refresh period and 1.5) | H (presented), V. Bites: a 20 ms busy-wait on every 60th frame must be counted 10 of 10 per 600 frames | R3-CHAR-REC4 (lines 1088–1092; it stays adopted); part 7's spectacle budget; OQ-0003-17 |
| PM-0003-45 | Nothing outlives its match (parts 3, 7, 10; "disposal carries over as a v2 rule", Consequences; ADR 0005's GPU part 7) | after each of 3 cycles (BS with one faction set, torn down to an empty page with the renderer kept, as a return to the menu will be; then LS with the other factions and its own test assets, torn down again): `renderer.info.memory`'s `total`, `texturesSize`, `attributesSize`, `programs` and `renderTargets` equal to the first cycle's values after the same teardown, and the JS heap after a forced garbage collection within 0.5 MB of it (proposed: an empty page holds only what the renderer itself keeps, the same every cycle, while the heap varies; 0.5 MB is PM-0003-24's flat rule); 0 shared materials disposed while a scene still uses them | CI (counts); ML: Chrome on WebGPU and with `forceWebGL` (the counts and the heap) | the cycle script; the counters read after each teardown (R3-MEASURE-F3, lines 617–622); the heap through the Chrome DevTools Protocol, as PM-0003-24; a spy on `dispose()` of every cached material (proposed) | 1 run of 3 cycles per backend (counts repeat, PM-0003-35). Bites: one cycle's soldier model left undisposed must raise the next teardown's counts; a teardown that disposes a shared material the next scene uses must be caught by the spy | the graphics review's finding 6 (proposed there); the Consequences' disposal rule; ADR 0005 GPU part 7 ("never disposed while shared") |
| PM-0003-46 | The WebGPU backend is exercised on every change to the render path (parts 1, 8; ADR 0002 part 9) | recorded on every CI run: the backend each CI engine gets, or none (PM-0003-36). If one gets WebGPU (the Chromium job with SwiftShader's WebGPU adapter that ADR 0002's boundary spike tries; not researched), PM-0003-36's checks also run on it; if none does, every change-set that touches the render path carries a WebGPU smoke run: BS for 60 s on ML (Chrome on WebGPU) with 0 console and device errors (PM-0003-2's hook), PM-0003-36's counts within the committed baseline, and the overlay naming "WebGPU" | CI; otherwise ML | the CI job, or the smoke script on ML with its log attached to the change-set's report (a stated review step, ADR 0002 part 9) | from the render budget bench on, every change-set that touches the render path. Bites: a planted shader node that builds for WebGL 2 and fails only on WebGPU (proposed) fails the route in use, 1 of 1 | ADR 0002 part 9 (the route and the smoke run); R3-WGPU-F17 (lines 159–162: Chrome on Linux has WebGPU only on Intel Gen12+ GPUs); R3-MEASURE-F8 (lines 648–653: headless Chrome on SwiftShader) |
| PM-0003-47 | The HUD touches the DOM only when a value changes (part 10, step 7; legacy's lesson) | reported: the HUD's DOM mutations per frame in BS (median and max), beside PM-0003-19; gated: 0 HUD mutations in a scripted 5 s stretch of BS with the match timer hidden and no change of score, ammo, health or kill feed (proposed: a write only when the value changed, as legacy's cached setters do) | CI (a count) | BS with the HUD the spike has (its elements recorded); a `MutationObserver` on the HUD's root (child lists, attributes, text, subtree) counts the records per frame (proposed tool, its own cost not researched); legacy's `_sd`, `_so` and `_st`, which write only on a change (`shamble-game.js` lines 2606–2608), are the model | 1 run (counts repeat, PM-0003-35). Bites: one HUD element whose `textContent` is set every frame to its own value (the setter replaces the text node, so the observer sees it) | `docs/AUDIT_BUGS_PERF.md` line 27 (P3: about 30 DOM writes per frame, most unconditional, so style and layout work each frame); the perf review's finding 14 |
| **Later, at each three.js upgrade** | | | | | | |
END

### FIX AD-1 (the fail route: a [DECIDED] item leaves Low only under OQ-0003-16; routed by graphics-engineer), PA-4, PA-7 (the reported and cold sets)
FILE: docs/adr/0003-rendering.md
BEFORE:
**When a mark fails.** A spike passes when every mark that applies passes: a conditional mark (PM-0003-40) only when
its condition holds; a "reported" mark (PM-0003-9, -14, -16, -31, -42) when its record is complete. A failing run is
never re-run until green: its data, build commit, machine, browser and backend are kept, and its cause is found and
written down (R5-TEST-REC6, r5-toolchain-quality.md lines 1490–1498). A failed mark goes back to the draft:
graphics-engineer revises the decision part the row names, by the draft's own Rollback and "Revisit when" routes (for
example a technique moved to High only in the tier data, the other ink family, the AO pre-pass on High only, a
tighter budget LOD), and the revised part returns to the advisor for review and to Mustafa and the Advisor with the
measured numbers. A mark's number changes only in the draft, with its reason, never inside a spike (budgets are data,
and raising one is a reviewed change: R5-GATE-REC4, r5-toolchain-quality.md lines 1734–1744). A run set that fails V is neither a pass nor a
fail. An item Mustafa marks "not yet" goes back to the draft in the same way, and the look proof test shows it again.
AFTER:
**When a mark fails.** A spike passes when every mark that applies passes: a conditional mark (PM-0003-40) only when
its condition holds; a "reported" mark (PM-0003-9, -14, -16, -31, -42, and PM-0003-44 until OQ-0003-17 sets its
share) when its record is complete; the cold set of PM-0003-18 to -21 is reported beside the soaked set that gates
until OQ-0003-20 is answered. A failing run is
never re-run until green: its data, build commit, machine, browser and backend are kept, and its cause is found and
written down (R5-TEST-REC6, r5-toolchain-quality.md lines 1490–1498). A failed mark goes back to the draft:
graphics-engineer revises the decision part the row names, by the draft's own Rollback and "Revisit when" routes (for
example a technique lowered in the tier data, or a `[DEFAULT]` item's technique moved to High only, the other ink
family, the AO pre-pass on High only, a tighter budget LOD), and the revised part returns to the advisor for review
and to Mustafa and the Advisor with the measured numbers. A technique that carries a LOOK_V2 `[DECIDED]` item (the
toon materials, the ink, the sun shadow map, the emissive slots' colour: part 7's look floor) is never moved to High
only by this route: it leaves Low only on Mustafa's answer to OQ-0003-16, given on the Low frame (look checklist
step 31) with the measured costs. A mark's number changes only in the draft, with its reason, never inside a spike
(budgets are data, and raising one is a reviewed change: R5-GATE-REC4, r5-toolchain-quality.md lines 1734–1744). A
run set that fails V is neither a pass nor a fail. An item Mustafa marks "not yet" goes back to the draft in the same
way, and the look proof test shows it again.
END

### FIX AD-2 (the checklist's intro: the Low frame), AD-5 (the test view), AD-8 (legacy views in Brave), the tier per step (routed by graphics-engineer)
FILE: docs/adr/0003-rendering.md
BEFORE:
Beside it, the measurable marks: each technique's cost (PM-0003-9), the two ink families (PM-0003-12), the frame time
with the kept look on both reference machines (PM-0003-13) and the pixel-ratio costs (PM-0003-14). His PC is far above
the recommended class, so what he sees is the look, never the budget. Each step: what to look at, beside a legacy
screenshot at the same framing (taken by the session with legacy's pageshot, headed), and the expected result in
LOOK_V2's words. He records "achieved" or "not yet" (with a note), or his pick where a step offers one.
AFTER:
Beside it, the measurable marks: each technique's cost (PM-0003-9), the two ink families (PM-0003-12), the frame time
with the kept look on both reference machines (PM-0003-13) and the pixel-ratio costs (PM-0003-14). His PC is far above
the recommended class, so what he sees is the look, never the budget; so that he also judges the look the minimum
class sees, step 31 shows Low's starting row on his PC beside High (Low is the default on every integrated adapter
and every WebGL 2 player, part 7). Each step names its tier: High, his PC's own, unless the step says Low. Every view
is at the test view ("How to read the table": legacy's framing, CD5) unless the step names another. Each step: what
to look at, beside the legacy view at the same framing, and the expected result in LOOK_V2's words. The legacy view
is legacy itself opened in a second Brave window on his PC (it runs from `index.html`) at the same framing, because
the session's container has no GPU and its pageshot renders on the CPU, so its multisampling, shadow filtering and
fonts differ from Brave's (the art review's finding 8); the session's pageshot images serve only to match the
framing, each labelled with the renderer pageshot prints. He records "achieved" or "not yet" (with a note), or his
pick where a step offers one.
END

### FIX AD-4 (step 4: the ground-contact line; routed by graphics-engineer)
FILE: docs/adr/0003-rendering.md
BEFORE:
4. The two ink families side by side, beside legacy's ink (§2, §6.2; OQ-0003-2): A, screen-space edges (proposed); B,
   the hull. Expected: thick black outlines, especially strong on buildings; thick ink on silhouettes, thinner lines
   inside. Pick: A or B, and whether the buildings' ink is heavy enough.
AFTER:
4. The two ink families side by side, beside legacy's ink (§2, §6.2; OQ-0003-2): A, screen-space edges (proposed); B,
   the hull. Expected: thick black outlines, especially strong on buildings; thick ink on silhouettes, thinner lines
   inside; the ink at the base of buildings, and where soldiers stand, is as heavy as their silhouette (5.b names this
   contact line as a visible difference between the families). Pick: A or B, and whether the buildings' ink is heavy
   enough.
END

### FIX AD-5 (step 9: the close and far test views), AD-14 (steps 10 to 12: the soldier at the 4-material cap; routed by graphics-engineer), AD-9 (step 13: the budget LOD picks by importance), AD-12 (step 15: the trees are GW18's builders)
FILE: docs/adr/0003-rendering.md
BEFORE:
9. One unit at the closest and at the farthest camera framing (§5). Expected: simple and readable from a distance, real
   depth up close; never "a block on wheels with a tube".
10. The new soldier beside legacy's soldier (§3). Expected: slimmer, serious proportions (XIII, Valiant Hearts); no
    chunky body, no round or goofy head.
11. A conscript in each faction's colours, beside legacy's conscript (§2). Expected: the round shoulder pieces, the
    centre stripe and the long trench-coat jacket; the factions told apart by colour.
12. Weapons and headgear at the game camera (§3, §6.3). Expected: real weapon silhouettes, no plain tubes or
    cylinders; weapons and headgear read first from above.
13. A soldier strafing on the full rig, then on the cheaper rig of the budget LOD (§2). Expected: the body reacts to
    strafing on the full rig; the cheaper rig's difference does not distract at the distance it is used.
14. Both houses beside legacy's (§2, §3). Expected: the half-timbered house with the brick-tile roof and the plastered
    house with the silo tower; no thatched roof anywhere.
15. The lush patch: fields, logs and trees (§1, §2, §4). Expected: lush; logs and trees with variety. The trees are
    labelled placeholders: the tree style is `[OPEN]` and not judged here.
AFTER:
9. One unit at two test views (§5): the test view itself, legacy's only framing, as the far case, and a close view,
   the same camera at half its distance to the player (proposed: legacy has no zoom range, CD5, and the v2 framing is
   R2-INT-OQ2's); both are taken again at the v2 framing's own closest and farthest once that question is answered.
   Expected: simple and readable from a distance, real depth up close; never "a block on wheels with a tube".
10. The new soldier beside legacy's soldier (§3), built at ADR 0005's cap of 4 materials per pass (its GPU part 4),
    so the soldier judged is the one the budget allows. Expected: slimmer, serious proportions (XIII, Valiant
    Hearts); no chunky body, no round or goofy head.
11. A conscript in each faction's colours, beside legacy's conscript (§2), built at the same cap. Expected: the round
    shoulder pieces, the centre stripe and the long trench-coat jacket; the factions told apart by colour.
12. Weapons and headgear at the game camera (§3, §6.3), on the soldier built at the same cap. Expected: real weapon
    silhouettes, no plain tubes or cylinders; weapons and headgear read first from above.
13. A soldier strafing on the full rig, then the fight on screen at Low's animation budget, where the budget LOD gives
    the cheaper rig by importance (own squad, nearest, shooting: part 6), not by distance, so cheaper-rig soldiers
    stand beside full-rig ones (§2). Expected: the body reacts to strafing on the full rig; on the soldiers the
    cheaper rig is used for, its difference does not distract.
14. Both houses beside legacy's (§2, §3). Expected: the half-timbered house with the brick-tile roof and the plastered
    house with the silo tower; no thatched roof anywhere.
15. The lush patch: fields, logs and trees (§1, §2, §4). Expected: lush; logs and trees with variety. The trees are
    legacy's tree builders ported as placeholders (GW18, `LEGACY_INVENTORY.md` line 206), judged for variety and
    lushness only (§2's `[DECIDED]` "trees with variety"): the tree style is `[OPEN]` (§8) and not judged here.
END

### FIX GE-7 (step 26: the timing half to ADR 0001's checklist), AD-10 (step 27: the flash over the HUD; routed), GE-8 (step 28: MP's devicePixelRatio), AD-2 (new step 31: the Low frame; routed by graphics-engineer), AD-3 (new step 32: the busy fight at day and dusk)
FILE: docs/adr/0003-rendering.md
BEFORE:
26. The Imperial Warrior's SMG firing at a target, beside legacy's (§7; LOOP_V2 §7). Expected: punchy: kick, muzzle
    flash, tracer, impact, hit marker, sound, target flinch and kill confirm; the shot feels instant and the hit
    feedback comes when the hit is confirmed.
27. One big moment with the halftone flash and ink line boil (§7; OQ-0003-8). Expected: comic impact touches, used
    sparingly. Pick: keep or drop each (its cost is in PM-0003-9).
28. The same frame at pixel ratio 1 and 2, with PM-0003-14's GPU times beside it (OQ-0003-12). Pick: ratio 1 or 2 on
    Low.
29. The legacy touches LOOK_V2 does not name, each shown beside legacy (OQ-0003-15): comic words, the kill vignette,
    the toy-soldier death fall, the skin rim, the camera-player cutaway (structures between the camera and the player
    fade, beyond LOOK_V2 §4's roofs; walk the player behind a building), and the ground grid, the red arena border and
    the world text labels. Pick: keep, change or drop each.
30. Nothing to judge for LOOK_V2 §8: the test assets are plain glTF/GLB exports given the comic look by the engine
    (steps 2 to 29 show it); compression, LODs and slot renaming are not tested here (ASSET-RESEARCH (a)); the
    per-unit budgets are counted in PM-0003-15 and PM-0003-22.
AFTER:
26. The Imperial Warrior's SMG firing at a target, beside legacy's (§7; LOOP_V2 §7). Expected: punchy: kick, muzzle
    flash, tracer, impact, hit marker, sound, target flinch and kill confirm, each read for its look and sound. Not
    judged here: whether the shot feels instant and the hit feedback comes when the hit is confirmed, since LS has no
    Worker, tick, input path or server to confirm a hit; that timing is judged on the real path, in ADR 0001's
    netcode checklist (its step 7, at P2) and at slice 1's Brave check.
27. One big moment with the halftone flash and ink line boil (§7; OQ-0003-8), with the flash over the HUD, which the
    overlay also dots (5.l). Expected: comic impact touches, used sparingly; the dotted HUD still reads. Pick: keep
    or drop each (its cost is in PM-0003-9); and, for OQ-0003-18, whether the concussion shake of step 25, the
    flashes and line boil soften for players who ask for reduced motion (its options).
28. At Low, the same frame at pixel ratio 1 and 2, with PM-0003-14's GPU times beside it (OQ-0003-12). If MP's
    `devicePixelRatio` (PM-0003-16) is below 1.5, ratio 2 there is supersampling, not the sharper HiDPI view the
    question is about (R3-BUDGET-F3, lines 551–555; inferred), so this step is shown on ML's own panel at its default
    Windows scaling, or on MAC, its ratio recorded. Pick: ratio 1 or 2 on Low.
29. The legacy touches LOOK_V2 does not name, each shown beside legacy (OQ-0003-15): comic words, the kill vignette,
    the toy-soldier death fall, the skin rim, the camera-player cutaway (structures between the camera and the player
    fade, beyond LOOK_V2 §4's roofs; walk the player behind a building), and the ground grid, the red arena border and
    the world text labels. Pick: keep, change or drop each.
30. Nothing to judge for LOOK_V2 §8: the test assets are plain glTF/GLB exports given the comic look by the engine
    (steps 2 to 29, 31 and 32 show it); compression, LODs and slot renaming are not tested here (ASSET-RESEARCH (a));
    the per-unit budgets are counted in PM-0003-15 and PM-0003-22.
31. The Low frame (part 7's look floor; OQ-0003-16): Low's starting row (pixel ratio 1, scene resolution scale 0.75,
    one 1024² sun shadow map, the rest as Low's tier data sets it), shown on his PC beside the same view at High and
    beside legacy: the whole frame (step 2), the ink families (step 4), the shadows (step 6), the frame with ink and
    ambient occlusion on, then off (step 7), and the two-floor building's corners (step 20). Expected: every
    `[DECIDED]` item still reads at Low: cel shading with bold ink, thick outlines especially strong on buildings, soft
    shadows, emissive accents. Record: achieved or not yet, per item. Pick (OQ-0003-16): per item, whether Low may only
    lower it or may switch it off, with PM-0003-9's costs on the minimum laptop beside it; and whether AO stays on at
    Low.
32. The busy fight: LS at the fight on screen (48 soldiers in three teams, the 4 very large units, 300 projectiles with
    their visual effects) at High, by day and at dusk (§5, §6.1, §6.4, §6.5). Expected: his own soldier, the three
    factions and a thrown grenade read at a glance; units, faction colours and effects stay the brightest things on
    screen, and the glow and the effects do not wash out the units, at day and at dusk. Record: achieved or not yet,
    for each mood; for OQ-0003-19, whether the three factions read apart by colour alone (his pick among its options).
END

### FIX PA-7 (new OQ-0003-20, HEAT-SOAK: the Advisor)
FILE: docs/adr/0003-rendering.md
BEFORE:
  fight view through a colour-blindness simulation, and Mustafa judges it; (c) faction shape cues, left to a later
  design session. Decider: Mustafa. What decides it: his judgement in the look proof test. Research ids: none.
AFTER:
  fight view through a colour-blindness simulation, and Mustafa judges it; (c) faction shape cues, left to a later
  design session. Decider: Mustafa. What decides it: his judgement in the look proof test. Research ids: none.
- **OQ-0003-20. Is the minimum laptop's budget judged after 15 minutes of play, or on a cold laptop?**
  Why it matters: a match runs about 20 minutes (LOOP_V2 §8), while one H run takes about 30 s at 30 fps (900 frames;
  about 110 s in the tail form); in a thin laptop the CPU and the integrated GPU share one power and heat budget, so a
  cold run can measure a short boost the same laptop no longer holds at minute 15 and pass a row it then fails
  (engineering experience, not researched: the chosen laptop's boost behaviour is unknown). Options: (a) the soaked
  set S gates PM-0003-18 to PM-0003-21 on ML, and the cold set is reported beside it; (b) the cold set only, R3's 3
  runs as written; (c) both must pass. The pass marks hold to (a) until it is answered. Recommendation: (a), the
  perf review's question HEAT-SOAK. Decider: the Advisor. What decides it: the Advisor's reading of VISION_V2 §4's
  "30+ fps on Low" over a whole match; the bench's cold and soaked sets, once measured, show how far apart they lie.
  Research ids: R3-BUDGET-REC2 (lines 1162–1164, its 3 runs; it stays adopted, no new disposition).
END

### NOTE PA-4 (routed)
fixed in PM-0003-44 (the presented-frame count, reported until OQ-0003-17 sets the share) and the fail route. routed:
engine-architect's ENGINE_OVERVIEW §6.9 row and the README's OQ-0003-17 row, as graphics-engineer's NOTE PA-4 already
asks; nothing more.
END

### NOTE PA-7 (routed; new question)
fixed in the key (S, the soaked set), PM-0003-18 to -21 and the fail route; the choice is OQ-0003-20 (HEAT-SOAK,
decider the Advisor), with (a) held as the interim. routed: engine-architect's docs/adr/README.md lists OQ-0003-20
under the decisions the drafts ask for (the Advisor); graphics-engineer's part 8 harness bullet may add "and, on the
minimum laptop, a soaked set after 15 minutes of looping (OQ-0003-20)" (optional: the key already carries it).
END

### NOTE GE-6 (routed)
fixed in PM-0003-45 (3 cycles, the counters back to the first teardown's values, the heap within 0.5 MB, a spy on
shared materials). routed: graphics-engineer's part 7 or part 10 names the teardown rule the row tests ("a match's
models, textures, render targets and pooled effects are disposed when it ends; shared materials are disposed only when
no loaded content uses them"), as GE-6's own fix asks.
END

### NOTE GE-2 (my half; unit A's half)
fixed in PM-0003-36 (the backend recorded per CI engine) and PM-0003-46 (the WebGPU route: the CI job if the
boundary spike finds one, else the smoke run on ML per render-path change-set). Unit A's PM-0002-51 and PM-0002-64
record the same backend per engine and point to PM-0003-46 for the route rather than restating it.
END

### NOTE GE-14 (a question, kept open)
fixed as far as it can be without the source: PM-0003-2 counts the device's `uncapturederror` and `lost` events and
`webglcontextlost` as device errors, with a planted invalid bind group as the bite, and PM-0003-16 records how each
browser shows it. The WebGPU specification's error model was not read here (this round fetches nothing), so the row
says "not researched"; the verifier may confirm it against the specification's error-handling section.
END

### NOTE AD-12 (routed)
fixed in step 15. routed (optional): graphics-engineer's part 11 may name the lush patch's trees as legacy's ported
tree builders (GW18), so the test assets and the step agree.
END

### NOTE PA-14 (routed)
fixed in PM-0003-19 (what its span cannot see) and PM-0003-47 (the HUD's DOM mutations, 0 in a still stretch).
routed (optional): graphics-engineer's part 10 step 7 may add "written only when a value changes, as legacy's cached
setters do (PM-0003-47)".
END

### NOTE AD-2, OQ-0003-16 (routed)
the Low frame is look checklist step 31 (appended after step 30, so no step is renumbered). routed: graphics-engineer's
OQ-0003-16 "What decides it" and its Rollback line may name "the look checklist's step 31".
END

### FIX TE-1 (routed by rust-reviewer: PM-0004-2 counts S1's stand-in effect rows)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-2 | The stress scene carries its whole load, so no budget passes on a lighter scene (S1, S14; parts 6, 15) | on every measured tick: N soldiers alive in the one cluster (48, or the scale step), 15 big vehicles with the 4 very large units of at least 20 parts, 300 live projectiles, 8 active effects per soldier (384 at N = 48); every bot logs at least one target scan, one shot and one route followed in every minute of game time, and the carried uplink cache moves (S14's bot set: legacy GN rows' target scans, combat movement, gunfire and route following, plus the cache; the per-minute floor proposed: an idle bot lightens the scene) | the counts are machine-free: read on every T run, on RS and ML | the bench's per-tick count log and the bots' action log | every T run; a run that fails it is invalid, not failed. Bites: a build where dead soldiers stay dead fails it within its first minute | GL §3 (lines 397–420); the draft's S14 (proposed there); R4-NAV-REC2 (r4 lines 859–861: the carried cache); R1-ECS-REC1 mark 1 (lines 1923–1926) |
AFTER:
| PM-0004-2 | The stress scene carries its whole load, so no budget passes on a lighter scene (S1, S14; parts 6, 15) | on every measured tick: N soldiers alive in the one cluster (48, or the scale step), 15 big vehicles with the 4 very large units of at least 20 parts, 300 live projectiles, 8 active effects per soldier (384 at N = 48; in the sim core spike these are S1's 384 stand-in rows in the effect pool, which count as the stress scene's effects, with the effect step's 1 ms ceiling burned and the 2.80 ms remainder topped up on every tick, scaled by N / 48 at the other scale steps, and at 60 Hz 1 ms and 0.30 ms; the run's record names each stand-in and its per-tick log shows each ran); every bot logs at least one target scan, one shot and one route followed in every minute of game time, and the carried uplink cache moves (S14's bot set: legacy GN rows' target scans, combat movement, gunfire and route following, plus the cache; the per-minute floor proposed: an idle bot lightens the scene) | the counts are machine-free: read on every T run, on RS and ML | the bench's per-tick count log and the bots' action log | every T run; a run that fails it is invalid, not failed. Bites: a build where dead soldiers stay dead fails it within its first minute; a run with the effect step's stand-in burn switched off fails it on its first tick | GL §3 (lines 397–420); the draft's S14 (proposed there); R4-NAV-REC2 (r4 lines 859–861: the carried cache); R1-ECS-REC1 mark 1 (lines 1923–1926) |
END

### FIX RR-4 (PM-0004-6: the pool burst; routed by rust-reviewer)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-6 | The step allocates nothing in steady state (part 6) | 0 heap allocations in total over the measured ticks of a run at N = 48 | RS (native); ML: Chrome (Wasm) | a counting global allocator in the bench build, read before and after the measured ticks | 1 run per target (the count repeats: PM-0004-18). Rule A | the draft's part 6 (proposed there from R1-ABL-REC6's rule, lines 2471–2472) |
AFTER:
| PM-0004-6 | The step allocates nothing in steady state, even past a pool's capacity (parts 1, 6) | 0 heap allocations in total over the measured ticks of a run at N = 48; and in a burst of 30 ticks (one second) that asks the projectile and effect pools for twice their capacity (at least twice the stress scene's counts, 600 live projectiles and 768 active effects: arithmetic; proposed, so every overflow path runs): 0 heap allocations, per-tick state hashes equal on native and Wasm, and each pool's overflow counter equal on both targets and equal to the rows its rule turned away (part 1's test fixture, refusing the new row, until OQ-0004-11 is answered) | RS (native); ML: Chrome (Wasm); CI (the burst's hashes and counters: native, Wasm in Node) | a counting global allocator in the bench build, read before and after the measured ticks and around the burst; the burst scripted at a fixed tick of SS's recording | 1 run per target (the count repeats: PM-0004-18). Rule A; bites: a pool that grows its buffer when full (a `Vec::push` past the capacity) must be counted in the burst | the draft's part 6 (proposed there from R1-ABL-REC6's rule, lines 2471–2472); part 1 (capacities and the overflow counter); the burst proposed (the Rust review's finding 4) |
END

### FIX PA-5 (PM-0004-8: one reading of "30+ fps on Low"; unit A's PM-0002-28 cites it)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-8 | The browser runs the whole sim within [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share) (S2; part 6) | p99 sim step time ≤ 13.33 ms (40 % of 33.33 ms) in Wasm in a dedicated Worker at the N that R1-TICK-OQ7 sets, while the page meets ADR 0003's minimum-laptop row over the whole run (frame time median ≤ 28 ms and p95 ≤ 33.3 ms: the reading of "holds 30+ fps on Low", since R1-TICK-REC2 P2 leaves the render budget to R3); until R1-TICK-OQ7 is answered: reported at N = 12, 24, 36 and 48, no pass | ML: Chrome and Firefox on Windows 11 (Edge reported; Safari on MAC reported, no budget row) | T on SS in the boundary spike's Worker with the sim core spike's sim; the page draws the sim's output at Low (ADR 0003's renderer if it exists by then, else the boundary spike's page: recorded which); step times from the Worker's timer around the step, isolation flag on; frame times from the page's harness, mode U (ADR 0003: vsync off) | T, V; rule B | R1-TICK-REC2 P2 (lines 2054–2057); R1-TICK-OQ7 (lines 2667–2670); R3-BUDGET-REC2's frame rows (r3 lines 1123–1124); GL lines 214–218 |
AFTER:
| PM-0004-8 | The browser runs the whole sim within [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share) (S2; part 6) | p99 sim step time ≤ 13.33 ms (40 % of 33.33 ms) in Wasm in a dedicated Worker at the N that R1-TICK-OQ7 sets, while the page meets ADR 0003's minimum-laptop row over the whole run (frame time median ≤ 28 ms and p95 ≤ 33.3 ms, mode U: the reading of "holds 30+ fps on Low", since R1-TICK-REC2 P2 leaves the render budget to R3; it is the drafts' one reading of P2's "30+ fps", so ADR 0002's PM-0002-28 cites this row and sets none of its own, and a presented-frame count on ML, if OQ-0003-17 asks for one, is ADR 0003's PM-0003-44); until R1-TICK-OQ7 is answered: reported at N = 12, 24, 36 and 48, no pass | ML: Chrome and Firefox on Windows 11 (Edge reported; Safari on MAC reported, no budget row) | T on SS in the boundary spike's Worker with the sim core spike's sim; the page draws the sim's output at Low (ADR 0003's renderer if it exists by then, else the boundary spike's page: recorded which); step times from the Worker's timer around the step, isolation flag on; frame times from the page's harness, mode U (ADR 0003: vsync off) | T, V; rule B | R1-TICK-REC2 P2 (lines 2054–2057); R1-TICK-OQ7 (lines 2667–2670); R3-BUDGET-REC2's frame rows (r3 lines 1123–1124); GL lines 214–218 |
END

### FIX RR-11 (PM-0004-21: the hash and the serializer independent of usize width)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-21 | The suite catches a divergence at the tick it happens; the hash function is fixed (S7; part 9) | each injected divergence (one std `sin` call, one std `HashMap` iteration, one presentation random draw inside the sim) reported at its arming tick T with a field-level diff: 3 of 3; reported: the hash function picked, and its cost per tick (mean, p99) | CI (native ×2, Node, the three engines); RS (the cost) | each injection armed at a known tick T with a value that differs between the two runs (proposed: an injection that changes no state proves nothing): the `sin` input taken from PM-0004-22's table where std's `sin` differs between native and Wasm (if none differs, the injection is recorded as not catchable on those targets and a 1-ulp planted change stands in); the `HashMap` holding at least 32 keys whose order decides an outcome, with its two orders logged | 1 run per injection. Control: a presentation draw outside the sim passes (R5-GATE-REC3's G10 row, r5 line 1721) | R1-DET-REC6 (lines 2222–2240); R5-TEST-REC2's mark (r5 lines 1427–1428); R5-GATE-REC3 (r5 line 1721) |
AFTER:
| PM-0004-21 | The suite catches a divergence at the tick it happens; the hash function is fixed (S7; part 9) | each injected divergence (one std `sin` call, one std `HashMap` iteration, one presentation random draw inside the sim) reported at its arming tick T with a field-level diff: 3 of 3; reported: the hash function picked, and its cost per tick (mean, p99); and neither the hash nor the serializer depends on the target's word size: the picked function gives the same digest for a committed byte string on native x86-64 and on wasm32, and one committed state serializes to the same bytes on both (proposed: `usize` is 32 bits on wasm32 and 64 bits on x86-64, so a length or index written as `usize`, or a hasher fed through `write_usize`, would make PM-0004-19 fail on its first tick for no game reason) | CI (native ×2, Node, the three engines); RS (the cost) | each injection armed at a known tick T with a value that differs between the two runs (proposed: an injection that changes no state proves nothing): the `sin` input taken from PM-0004-22's table where std's `sin` differs between native and Wasm (if none differs, the injection is recorded as not catchable on those targets and a 1-ulp planted change stands in); the `HashMap` holding at least 32 keys whose order decides an outcome, with its two orders logged | 1 run per injection; the digest and serialization vectors join the unit vectors (20 identical verdicts). Control: a presentation draw outside the sim passes (R5-GATE-REC3's G10 row, r5 line 1721). Bites (the word size): a planted `usize` field in the serialized state must change the bytes between the two targets | R1-DET-REC6 (lines 2222–2240); R5-TEST-REC2's mark (r5 lines 1427–1428); R5-GATE-REC3 (r5 line 1721) |
END

### FIX RR-1 (optional, routed by rust-reviewer's follow-up b: PM-0004-22's helper vectors)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-22 | The maths library is one implementation everywhere (S7; part 7; decides R1-DET-OQ8 and R1-DET-OQ10) | for each `libm` function the sim uses, and for `sqrt`: at least 10,000 fixed inputs (a committed table of edge cases and seeded values: proposed) give bit-identical outputs (`to_bits`) on native x86-64 and in Wasm in Node and in each browser engine; `sqrt` also equals a correctly rounded reference; reported: each function's largest error in units in the last place against a higher-precision reference (R1-DET-OQ8's "enough" read as PM-0004-30's 1 mm apex and PM-0004-54's 1 cm cursor marks passing with `libm`: proposed), and `libm`'s cost against std's, native and in Wasm | CI; MAC (Safari); RS and ML (the cost) | unit vectors in `cargo test` and wasm-bindgen-test (R5-TEST-REC1) | 20 identical verdicts. Bites: one output moved by one unit in the last place on one target | R1-DET-REC2 (lines 2165–2168); R1-DET-OQ8 (lines 2704–2705); R1-DET-OQ10 (lines 2710–2712) |
AFTER:
| PM-0004-22 | The maths library is one implementation everywhere (S7; part 7; decides R1-DET-OQ8 and R1-DET-OQ10) | for each `libm` function the sim uses, and for `sqrt`: at least 10,000 fixed inputs (a committed table of edge cases and seeded values: proposed) give bit-identical outputs (`to_bits`) on native x86-64 and in Wasm in Node and in each browser engine; `sqrt` also equals a correctly rounded reference; part 7's `min`, `max` and `clamp` helpers give the same bits on every target for equal signed zeros, (+0, −0) and (−0, +0), and a clamp with its bounds the wrong way round returns a bound with no panic (part 7: one defined argument on equal inputs); reported: each function's largest error in units in the last place against a higher-precision reference (R1-DET-OQ8's "enough" read as PM-0004-30's 1 mm apex and PM-0004-54's 1 cm cursor marks passing with `libm`: proposed), and `libm`'s cost against std's, native and in Wasm | CI; MAC (Safari); RS and ML (the cost) | unit vectors in `cargo test` and wasm-bindgen-test (R5-TEST-REC1) | 20 identical verdicts. Bites: one output moved by one unit in the last place on one target | R1-DET-REC2 (lines 2165–2168); R1-DET-OQ8 (lines 2704–2705); R1-DET-OQ10 (lines 2710–2712) |
END

### FIX RR-1 (PM-0004-23: the allow-list enforced, its bites and controls; routed by engine-architect and by rust-reviewer's follow-up b)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-23 | The float rules and [the hash-map ban](../research/GLOSSARY.md#the-hash-map-ban-in-the-sim-crate) are enforced by the build (S7; parts 7, 9) | the build fails on each of: a std transcendental call, an `algebraic_*` method, the `relaxed-simd` feature, a std `HashMap` or `HashSet` in the sim crate, a server build with `+fma` or `-C target-cpu=native`; 0 such uses in the spike's own build | CI | the gate v2's G2 lints and its build-flag check (ADR 0002) | every gate run. Bites: each forbidden item planted once (R5-GATE-REC3's G2 row: a `HashMap` field in `sim`, `x.sin()`); control: a reviewed `#[allow]` for a lookup-only map outside the state passes | R1-DET-REC2 (lines 2146–2150, 2166–2168); R1-ECS-REC2 mark 2 (lines 1969–1970); R1-RR-OQ1 (lines 2929–2931); R5-GATE-REC3 (r5 line 1714); GL lines 729–734 |
AFTER:
| PM-0004-23 | The float rules and [the hash-map ban](../research/GLOSSARY.md#the-hash-map-ban-in-the-sim-crate) are enforced by the build (S7; parts 7, 9) | the build fails on any std `f32` or `f64` method outside part 7's allow-list (`powi`, `hypot`, `cbrt`, `to_degrees`, `to_radians`, `min`, `max` and `clamp` named), as well as on each of: a std transcendental call, an `algebraic_*` method, the `relaxed-simd` feature, a std `HashMap` or `HashSet` in the sim crate, a server build with `+fma` or `-C target-cpu=native`; 0 such uses in the spike's own build | CI | the gate v2's G2 lints (ADR 0002's `disallowed-methods`, the complement of part 7's allow-list) and its build-flag check (ADR 0002) | every gate run. Bites: each forbidden item planted once (R5-GATE-REC3's G2 row: a `HashMap` field in `sim`, `x.sin()`), plus `deg.to_radians()` and `a.min(b)` in `sim`, each failing G2, as in PM-0002-41; controls: a reviewed `#[allow]` for a lookup-only map outside the state passes, and so do part 7's three helpers and one allowed method (`x.abs()`, `x.fract()`) in `sim` | R1-DET-REC2 (lines 2146–2150, 2166–2168); R1-DET-F9 (lines 648–654) and R1-DET-F14 (lines 679–683), as part 7 cites them; R1-ECS-REC2 mark 2 (lines 1969–1970); R1-RR-OQ1 (lines 2929–2931); R5-GATE-REC3 (r5 line 1714); GL lines 729–734 |
END

### FIX RR-12 (PM-0004-24: the finite check works on release builds)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-24 | NaN never enters the state (part 7) | 0 non-finite values in the state over every run of PM-0004-13 and PM-0004-18 to -21, with the finite-state assertion on (debug and test builds); every non-finite value in PM-0004-60's fuzzed commands rejected where it enters | CI | the assertion each tick; NaN and infinities in the fuzzed commands' float fields | as the runs it rides on. Bites: a planted `0.0 / 0.0` in one phase trips the assertion at its tick | R1-DET-REC2 (lines 2150–2152); the draft's part 7 |
AFTER:
| PM-0004-24 | NaN never enters the state (part 7) | 0 non-finite values in the state over every run of PM-0004-13 and PM-0004-18 to -21, read by the replay runner itself on the release builds those runs use (it already reads every float by `to_bits` for the state hash, part 9, so it flags any non-finite value with no sim feature, which PM-0004-74 would show as a difference), with the finite-state assertion on in debug and test builds; every non-finite value in PM-0004-60's fuzzed commands rejected where it enters | CI | the assertion each tick; NaN and infinities in the fuzzed commands' float fields | as the runs it rides on. Bites: a planted `0.0 / 0.0` in one phase trips the assertion at its tick | R1-DET-REC2 (lines 2150–2152); the draft's part 7 |
END

### FIX EA-3 (routed by rust-reviewer: PM-0004-29 names the prediction step), SA-15 (optional bite)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-29 | [Authority-only streams](../research/GLOSSARY.md#shared-stream-and-authority-only-stream) stay off the client (S8; part 8) | 0 occurrences of any authority-only stream's seed or state bytes in the predicting client's serialized state, after every tick of M10 | CI (Wasm in Node) | the client-mode instance built as ADR 0001's prediction scope makes it; each tick's serialization searched for every authority-only stream's seed and state | 1 run. Bites: the spread stream copied into the client's state | the draft's S8 (proposed there: SEC-SPREAD (a)); GL lines 525–532 |
AFTER:
| PM-0004-29 | [Authority-only streams](../research/GLOSSARY.md#shared-stream-and-authority-only-stream) stay off the client (S8; part 8) | 0 occurrences of any authority-only stream's seed or state bytes in the predicting client's serialized state, after every tick of M10 | CI (Wasm in Node) | the prediction step (part 3's second entry point, which draws from no stream) on a client-mode instance, as ADR 0001's prediction scope uses it; each tick's serialization searched for every authority-only stream's seed and state | 1 run. Bites: the spread stream copied into the client's state; a seed derived from the room id instead of drawn by the host from the operating system's random source (part 8) must be found, since the search also derives a seed from every identifier the client receives (room id, match id) by the build's own derivation and compares it with each authority-only seed | the draft's S8 (proposed there: SEC-SPREAD (a)); GL lines 525–532 |
END

### FIX GD-8 (routed by rust-reviewer: PM-0004-65 starts money at $300)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-65 | Every match starts with empty records, sized by content (S16; part 18) | 100 of 100 consecutive matches in one process start with every slot's record empty, field by field (proposed: a leak between matches shows only when one process runs several); 0 heap allocations in the record table after match start over M10 with random picks and pickup claims | CI | the match-start phase called 100 times; the counting allocator | 1 run. Bites: a record table reused without its reset; rule A | UPGRADE-SCOPE; LOOP_V2 §8 ("Nothing carries into the next match"); the draft's part 18 |
AFTER:
| PM-0004-65 | Every match starts with empty records, sized by content (S16; part 18) | 100 of 100 consecutive matches in one process start with every slot's record at its start, field by field: empty tracks and money at its starting amount, $300 (part 18, S16) (proposed: a leak between matches shows only when one process runs several); 0 heap allocations in the record table after match start over M10 with random picks and pickup claims | CI | the match-start phase called 100 times; the counting allocator | 1 run. Bites: a record table reused without its reset; money carried from the last match instead of reset to $300; rule A | UPGRADE-SCOPE; LOOP_V2 §8 ("Nothing carries into the next match"); the draft's part 18 |
END

### FIX NA-8 (routed by rust-reviewer: PM-0004-68 reads jitter online against the server-time estimate's schedule)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-68 | The Worker paces the tick in every browser (S18; part 5) | foreground Worker tick start jitter p99 ≤ 4 ms over 18,000 ticks (10 minutes at 30 Hz) | ML: Chrome, Edge, Firefox; MAC: Safari | the boundary spike's Worker (W2: `setTimeout` waking a `performance.now()` accumulator) with the sim core spike's sim at N = 48; start times logged in the Worker; the isolation flag as shipped (recorded); if W2 fails, W3 is measured on an isolated build (ADR 0002, OQ-0002-4) | 3 runs per browser; each meets the mark. Bites: ticks counted per timer callback instead of from elapsed time | R1-TICK-REC5 (lines 2096–2097, 2109–2110) |
AFTER:
| PM-0004-68 | The Worker paces the tick in every browser (S18; part 5) | foreground Worker tick start jitter p99 ≤ 4 ms over 18,000 ticks (10 minutes at 30 Hz), read against the Worker's own schedule: in local play the accumulator's, online the schedule the step returns from the client endpoint's server-time estimate (part 5, S18), with the estimate's changes over the run recorded | ML: Chrome, Edge, Firefox; MAC: Safari | the boundary spike's Worker (W2: `setTimeout` waking a `performance.now()` accumulator) with the sim core spike's sim at N = 48; start times logged in the Worker; online, against the netcode proof test's server at P2 (ADR 0002's simulator); the isolation flag as shipped (recorded); if W2 fails, W3 is measured on an isolated build (ADR 0002, OQ-0002-4) | 3 runs per browser; each meets the mark. Bites: ticks counted per timer callback instead of from elapsed time; online, a Worker that keeps its local schedule and ignores the server-time estimate must fail the jitter read against that schedule under P2 | R1-TICK-REC5 (lines 2096–2097, 2109–2110) |
END

