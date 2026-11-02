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

### FIX RR-13 (PM-0004-60: a small scene, a nightly budget, the corpus kept), SA-13 (the room sequences: a pointer; the row is ADR 0001's)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-60 | No panic from what a client can send (S20; part 20) | 0 panics over 10 million fuzzed commands fed to a running sim (proposed count: ADR 0001's per-message-type decoder fuzz, R2-TRN-REC4 mark 4), every field over its full decoded range, NaN, infinities and claims on ids that do not exist included | CI (nightly, Linux) | cargo-fuzz on a target that feeds decoded command values into a running sim (R5-TEST-REC4's fuzz targets) | nightly. Bites: a planted `.unwrap()` on a client-sent pickup id is found within the run | the draft's S20 (proposed there); R2-TRN-REC4 mark 4 (r2 line 2696); R1-RR-OQ3 (lines 2936–2940) |
AFTER:
| PM-0004-60 | No panic from what a client can send (S20; part 20) | 0 panics over 10 million fuzzed commands fed to a running sim (proposed count: ADR 0001's per-message-type decoder fuzz, R2-TRN-REC4 mark 4), every field over its full decoded range, NaN, infinities and claims on ids that do not exist included | CI (nightly, Linux) | cargo-fuzz on a target that feeds decoded command values into a running sim (R5-TEST-REC4's fuzz targets), in a small scene: a 1v1 on LB (proposed: coverage comes from the command values, not from the crowd; at the stress scene 10 million ticks would take about 23 hours, 10,000,000 × 8.33 ms = 83,300 s, arithmetic); the corpus is kept between nights, so the 10 million accumulate within a nightly budget of 60 minutes (proposed), the running total recorded. Sequences of decoded control messages into the `room` crate (a pick on a closing offer, a host kicking itself, settings floods, join, leave and reconnect races) are fuzzed beside it once `room` exists, in ADR 0001's pass marks (beside PM-0001-34), since a panic there ends the match too (part 20) | nightly. Bites: a planted `.unwrap()` on a client-sent pickup id is found within the run | the draft's S20 (proposed there); R2-TRN-REC4 mark 4 (r2 line 2696); R1-RR-OQ3 (lines 2936–2940) |
END

### FIX RR-9 (routed by rust-reviewer: PM-0004-61 checks the panic message arrives)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-61 | A trap online ends in a fresh instance back in step (S20; part 20) | after a trap injected into the client's Worker: a fresh instance runs and holds the server's state (its [client-held state's hash](../research/GLOSSARY.md#state-hash) equal to the server's for that tick), the resync itself within ADR 0001's join mark (a complete world within 300 ms at P2); reported: the time from the trap to a running instance; the trap reaches error reporting (Sentry with Rust function names if ADR 0006's EU organisation exists by then, else the page's own error log) | ML: Chrome, Edge, Firefox; MAC: Safari | the boundary spike's Worker against the netcode proof test's server at P2 (ADR 0002's simulator); 20 traps per browser at random ticks (proposed) | 20 of 20 per browser. Bites: a host that does not restart the instance | R1-RR-OQ3 (lines 2936–2940); R2-BUD-REC3 (r2 line 2359: ADR 0001's join mark); R6-OBS-REC3 (r6 lines 765–766); GL lines 546–550 |
AFTER:
| PM-0004-61 | A trap online ends in a fresh instance back in step (S20; part 20) | after a trap injected into the client's Worker: a fresh instance runs and holds the server's state (its [client-held state's hash](../research/GLOSSARY.md#state-hash) equal to the server's for that tick), the resync itself within ADR 0001's join mark (a complete world within 300 ms at P2); reported: the time from the trap to a running instance; the trap reaches error reporting (Sentry with Rust function names if ADR 0006's EU organisation exists by then, else the page's own error log), carrying the panic message the hook wrote into its fixed buffer (part 20), which the Worker's host reads after the trap: 20 of 20 messages arrive whole, each naming the planted panic's file and line | ML: Chrome, Edge, Firefox; MAC: Safari | the boundary spike's Worker against the netcode proof test's server at P2 (ADR 0002's simulator); 20 traps per browser at random ticks (proposed) | 20 of 20 per browser. Bites: a host that does not restart the instance; a host that reports the trap without reading the hook's buffer (the message must be missing from the report, so the check can fail) | R1-RR-OQ3 (lines 2936–2940); R2-BUD-REC3 (r2 line 2359: ADR 0001's join mark); R6-OBS-REC3 (r6 lines 765–766); GL lines 546–550 |
END

### FIX RR-10 (routed by engine-architect: PM-0004-74 compares every build that runs the sim, with a control)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-74 | One feature set builds the sim on every target (S22; part 7; the id is ADR 0002's) | the resolved feature list of every dependency of `sim` (`cargo tree -e features`) identical for the `server` build and the `sim-wasm` build: 0 differences | CI | `cargo tree -e features` run for `sim` in each build's target and features; the two lists sorted and compared by script | 1 run; the foundation re-proves it as a gate check (ADR 0002). Bites: a planted extra feature on a shared dependency in `server` makes the check fail (R5-LAYOUT-REC4's own) | R5-LAYOUT-REC4 (r5 lines 1216–1218); the draft's S22 |
AFTER:
| PM-0004-74 | One feature set builds the sim on every target (S22; part 7; the id is ADR 0002's) | the resolved feature list of every dependency of `sim` (`cargo tree -e features`) identical for every build that runs the sim (`server`, `devtools`, `navbake` and `sim-wasm`), each resolved by the exact command CI and the release use: 0 differences | CI | `cargo tree -e features` run for `sim` in each build's target and features; the two lists sorted and compared by script | 1 run; the foundation re-proves it as a gate check (ADR 0002). Bites: a planted extra feature on a shared dependency in `server` makes the check fail (R5-LAYOUT-REC4's own); control: a dev-only feature on `devtools` that touches no `sim` dependency passes | R5-LAYOUT-REC4 (r5 lines 1216–1218); the draft's S22 |
END

### FIX RR-4, POOL-FULL (the intro: PM-0004-75 named)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
Written by test-engineer (plan step 4). One row per mark, grouped by the spike that runs it: the sim core spike
(PM-0004-1 to PM-0004-61, then PM-0004-73 and PM-0004-74, added at the fix round), the content and ability spike
(PM-0004-62 to PM-0004-66), the netcode proof test (PM-0004-67), the boundary spike (PM-0004-68 to PM-0004-70), and
AFTER:
Written by test-engineer (plan step 4). One row per mark, grouped by the spike that runs it: the sim core spike
(PM-0004-1 to PM-0004-61, then PM-0004-73 and PM-0004-74, added at the fix round), the content and ability spike
(PM-0004-62 to PM-0004-66, then PM-0004-75, added at the final round), the netcode proof test (PM-0004-67), the
boundary spike (PM-0004-68 to PM-0004-70), and
END

### FIX RR-4, POOL-FULL (new PM-0004-75: the overflow rule per kind, once OQ-0004-11 is answered; routed by rust-reviewer and game-designer)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| **The netcode proof test** | | | | | | |
AFTER:
| PM-0004-75 | Each pool's overflow rule holds, per kind, as OQ-0004-11 answers it (parts 1, 16, 18; ADR 0005 AB-1's proposal) | only once Mustafa confirms OQ-0004-11; for the proposal as ADR 0005 AB-1 writes it: an effect the content bounds on its target (its own picks, its own ability's and weapon's effects, spawn protection) is never refused or ended at a full pool, 0 in 1,000 scripted pile-ups (proposed count, as PM-0004-66); when the share of effects others apply is full, the oldest such row on the target, by start tick and then sequence number (part 4), ends early, in 1,000 of 1,000 pile-ups on native and Wasm alike; the projectile pool's overflow counter stays 0 at the content's worst case at N = 48 (for legacy's Molotov, ⌊1.4 ÷ 1.05⌋ + 1 = 2 per thrower, 96 at N = 48: AB-1's arithmetic); a drop into a full pickup pool ends the pickup that has lain longest | CI | scripted pile-ups (48 sources on one target; every thrower throwing at its fire interval; deaths into a full pickup pool) on the replay runner, natively and in Node; equal per-tick state hashes and overflow counters on both targets | 1 run per kind (deterministic). Bites: a pool that refuses a content-bounded effect; a full effect share that ends the newest row instead of the oldest; a projectile pool sized from the stress scene's 300 instead of the content's worst case, which must count overflows | OQ-0004-11; ADR 0005 AB-1 ("When a pool is full"); part 1; R1-ABL-REC6 (lines 2471–2472: no heap allocation in steady state) |
| **The netcode proof test** | | | | | | |
END

### FIX RR-4 (the fail route: PM-0004-75 conditional)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
**When a mark fails.** A spike passes when every mark that applies passes: a conditional mark (PM-0004-14, -26, -71,
-72) only when its condition holds; a "reported" mark (PM-0004-4, -44, -58) when every cell holds a number; a decision
AFTER:
**When a mark fails.** A spike passes when every mark that applies passes: a conditional mark (PM-0004-14, -26, -71,
-72, and PM-0004-75 until OQ-0004-11 is answered) only when its condition holds; a "reported" mark (PM-0004-4, -44,
-58) when every cell holds a number; a decision
END

### FIX TE-6 (routed by rust-reviewer: part 17's line points to the test plan and migration phase 6)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
- Part 17 (economy and objectives): no spike mark; they arrive in slice 4, where a recorded full match replaying with
  identical per-tick hashes on every target is their check (the draft's migration phase 6).
AFTER:
- Part 17 (economy and objectives): no spike mark; they arrive in slice 4, where a recorded full match replaying with
  identical per-tick hashes on every target is their check, with the test plan's planted divergence that changes only
  one economy value, caught at its tick, and migration phase 6's scripted cases for LOOP_V2 §2 to §5's rules.
END

### FIX TE-6 (part 19's line points to the test plan's event divergence)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
- Part 19 (events, moments, telemetry): no spike mark; the event list is in the state hash, so PM-0004-18 and -19 cover
  its order, and the moment mechanism has no research behind it.
AFTER:
- Part 19 (events, moments, telemetry): no spike mark; the event list is in the state hash, so PM-0004-18 and -19 cover
  its order, the test plan's planted divergence that changes only one event is caught at its tick, and the moment
  mechanism has no research behind it.
END

### NOTE PA-5 (my half; unit A's half)
fixed in PM-0004-8: ADR 0003's minimum-laptop frame rows (median ≤ 28 ms, p95 ≤ 33.3 ms, mode U) are the drafts' one
reading of R1-TICK-REC2 P2's "30+ fps on Low". Unit A's PM-0002-28 must read: "ADR 0004's PM-0004-8, measured here
(its reading of '30+ fps on Low', mode U)", dropping its own "every 1-second window holding at least 30 presented
frames" and its mode P.
END

### NOTE RR-11 (routed)
fixed in PM-0004-21 (a digest and a serialization vector on native x86-64 and wasm32; a planted `usize` field as the
bite). routed: rust-reviewer's part 9 adds the clause its finding asks for: "integers at fixed width, little-endian; no
`usize` in serialized state or in the bytes the hash reads".
END

### NOTE SA-13 (routed; the row is ADR 0001's)
PM-0004-60 points to it; the target itself needs `room`, which no ADR 0004 spike builds. Unit A adds it beside
PM-0001-34: a cargo-fuzz target that drives `room` with random sequences of decoded control messages (a pick on a
closing offer, a host kicking itself, settings floods, join, leave and reconnect races): 0 panics, the same count as
PM-0001-34; bite: a planted `.unwrap()` on a departed slot's record. netcode-architect's test plan lists it.
END

### NOTE RR-4, POOL-FULL (routed)
fixed in PM-0004-6 (the burst past capacity, under part 1's test fixture) and the new conditional PM-0004-75 (the rule
per kind as ADR 0005 AB-1 proposes it, run once Mustafa confirms OQ-0004-11). Nothing more to route.
END

### FIX IE-9, IE-10 (the LG key: a calibration run sizes the load generator; the per-address cap raised; routed by infra-engineer)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  same location. **LG**, the load generator: a separate VM in the same location that runs every bot client, never the
  VM under test (proposed: bot clients on the VM under test would spend the vCPUs the marks measure); a CX33 unless
  its own CPU use (recorded every second) passes 80 % in a run, which makes that run invalid (proposed). **MV**, a
AFTER:
  same location. **LG**, the load generator: one or more separate VMs in the same location that run every bot client,
  never the VM under test (proposed: bot clients on the VM under test would spend the vCPUs the marks measure): CX33s
  (or a larger plan), as many as a 5-minute calibration run at the session's load shows keep each under 80 % of its
  CPU, each one's CPU use recorded every second, and a run in which any LG VM passes 80 % is invalid (proposed: 384
  encrypted sessions each receiving 30 snapshots a second may need more than one 4-vCPU VM; the server side alone may
  spend up to 1 vCPU per 48 sessions, ADR 0001); their count goes into the run plan and PM-0006-18. On LG's addresses
  the per-address connection cap (ADR 0001) is raised to the run's session count, and the raised value is recorded
  (What a spike must prove, item 2). **MV**, a
END

### FIX IE-16 (PM-0006-11: SMT siblings), IE-17 (routed by infra-engineer: created after PM-0006-12)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-11 | The reference server exists and is the machine every server row ran on (part 5; OQ-0006-1, the precondition) | a CCX23 and a CX33 created in `nbg1` or `hel1`; recorded for each: type, location, CPU model, vCPU count, memory, kernel; the CPU model read again at every session (a change is recorded and named in the results) | RS, SH | Mustafa creates the VMs or the API token that creates them (his account); a script reads `/proc/cpuinfo` and the plan at each session start | every session | R6-HOST-REC2 (r6 lines 645–650); R6-HOST-F20 (r6 lines 180–182); GLOSSARY, the reference server (lines 225–231) |
AFTER:
| PM-0006-11 | The reference server exists and is the machine every server row ran on (part 5; OQ-0006-1, the precondition) | a CCX23 and a CX33 created in `nbg1` or `hel1`, after PM-0006-12's round trips and OQ-0006-2's pick of the location; recorded for each: type, location, CPU model, vCPU count, memory, kernel, and `lscpu`'s threads per core with the map of which vCPUs are SMT siblings (if a core carries 2 threads, the transport runtime of ADR 0001's pinned rows is kept off the tick thread's sibling, and that placement is recorded: proposed, since a sibling shares the physical core and "one vCPU with no other room on it" would not isolate the tick; the CCX23's topology is not researched); the CPU model read again at every session (a change is recorded and named in the results) | RS, SH | Mustafa creates the VMs or the API token that creates them (his account); a script reads `/proc/cpuinfo` and the plan at each session start | every session | R6-HOST-REC2 (r6 lines 645–650); R6-HOST-F20 (r6 lines 180–182); GLOSSARY, the reference server (lines 225–231) |
END

### FIX IE-2, EA-7 (PM-0006-13: the rooms-per-VM runs' own validity, steal reported; with unit A)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-13 | Rooms per VM: 8 rooms at N = 48 on each machine (part 5; with ADR 0001's R2-ROOM-REC2 marks 1 and 2: one set of runs serves both records) | each of 8 rooms' server tick p99 at most 1.10 × the single-room figure; on SH also R2-ROOM-REC2 mark 1 for every room (p99 at most 16.67 ms, max under 33.33 ms, 0 overruns: [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 30 Hz) and every room start-up (process spawn to Open) under 2 s (R6-HOST-REC2 names both machines; the RS half of marks 1 and 5 is ADR 0001's) | RS and SH, each with LG | 8 room processes under the supervisor, no CPU pinning (as production would run them; recorded); the load at N = 48 (384 bot clients on LG); bracket B; each room process's CPU share of one vCPU per second also recorded (it holds the [transport runtime time](../research/GLOSSARY.md#transport-runtime-time)) | 3 sessions per machine at the 3 times of day; every valid session passes. **On SH it is an adoption mark:** a fail, or an invalid session, keeps the fleet on dedicated vCPUs (K1) and K3 out, as part 8 says; it is not a fail of this record. Bite: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row (proposed) | R6-HOST-REC2's mark (r6 lines 667–671); R2-ROOM-REC2 marks 1, 2, 5 (r2 lines 2774–2780); A-2 (r6 lines 516–522); N32 (0006 the figure, 0001 the test) |
AFTER:
| PM-0006-13 | Rooms per VM: 8 rooms at N = 48 on each machine (part 5; with ADR 0001's R2-ROOM-REC2 marks 1 and 2: one set of runs serves both records) | each of 8 rooms' server tick p99 at most 1.10 × the single-room figure; on SH also R2-ROOM-REC2 mark 1 for every room (p99 at most 16.67 ms, max under 33.33 ms, 0 overruns: [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 30 Hz) and every room start-up (process spawn to Open) under 2 s (R6-HOST-REC2 names both machines; the RS half of marks 1 and 5 is ADR 0001's) | RS and SH, each with LG | 8 room processes under the supervisor, no CPU pinning (as production would run them; recorded); the load at N = 48 (384 bot clients on LG); bracket B; each room process's CPU share of one vCPU per second also recorded (it holds the [transport runtime time](../research/GLOSSARY.md#transport-runtime-time)) | 3 sessions per machine at the 3 times of day; every valid session passes. Validity is bracket B's and LG's 80 % rule, not ADR 0001's V-S: eight tick threads on 4 vCPUs cannot each be alone on a vCPU, and on SH steal time is what the mark measures, so each vCPU's steal is recorded per second and reported, never a validity condition (proposed; the infra review's finding 2). ADR 0001's PM-0001-27 is this set of runs, read the same way. **On SH it is an adoption mark:** a fail, or an invalid session, keeps the fleet on dedicated vCPUs (K1) and K3 out, as part 8 says; it is not a fail of this record. Bite: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row (proposed) | R6-HOST-REC2's mark (r6 lines 667–671); R2-ROOM-REC2 marks 1, 2, 5 (r2 lines 2774–2780); A-2 (r6 lines 516–522); N32 (0006 the figure, 0001 the test) |
END

### FIX TE-2 (routed by infra-engineer: PM-0006-14 uses the padded stand-in, OQ-0006-19), IE-2
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-14 | Rooms per VM counted, the figure that replaces A-2 (parts 5, 8; decides R6-COST-OQ1) | reported, no pass number: on each machine and at N = 48, 36 and 18, the most rooms k for which every room meets R2-ROOM-REC2 marks 1 and 2 (the draft's counting rule). The figure counts as the goal budget's density only if each room's sim step time p99 in the run is at least the sim core spike's figure for the stress scene at that N (proposed: a stand-in cheaper than the real sim would overstate k); otherwise it is labelled "on the stand-in" and part 8 keeps both densities until slice 6 re-measures (the draft's migration step 7) | RS and SH, each with LG | per N, in one session with bracket B: k = 4 (A-2's case to budget, one room per vCPU), then 6, 8 and up by 2 until a k fails, then the odd k between the last pass and the first fail; if k = 4 fails, 3, 2, 1. Then the count and count + 1 again at the other 2 times of day. The load as PM-0006-13 with N set to 48, 36 and 18 (the goal modes and the three-team start mode's upper end) | the figure is the lowest count of the 3 sessions (proposed: a budget takes the worst case). A run plan with every session's VM-hours is written before the first session (it feeds PM-0006-18) | R6-COST-REC1's mark (r6 lines 714–716); the counting rule (draft, spike 2, proposed there); R6-COST-OQ1 (r6 lines 880–882); J6 (2) and OQ-0001-2 (the stand-in's load) |
AFTER:
| PM-0006-14 | Rooms per VM counted, the figure that replaces A-2 (parts 5, 8; decides R6-COST-OQ1) | reported, no pass number: on each machine and at N = 48, 36 and 18, the most rooms k for which every room meets R2-ROOM-REC2 marks 1 and 2 (the draft's counting rule). In these runs the stand-in's sim step is padded by a busy-wait to the sim step share's ceiling, 8.33 ms per tick (ADR 0001 part 3), or replays the step times the sim core spike recorded, so the figure is a lower bound on rooms per VM for any sim inside its share, labelled "a lower bound, provisional until slice 6" (OQ-0006-19's option (a), the interim; proposed: a stand-in cheaper than the real sim would overstate k); slice 6 re-measures on the real sim (the draft's migration step 7). Each room's padded step time p99 is recorded, so a run whose padding fell short is invalid | RS and SH, each with LG | per N, in one session with bracket B: k = 4 (A-2's case to budget, one room per vCPU), then 6, 8 and up by 2 until a k fails, then the odd k between the last pass and the first fail; if k = 4 fails, 3, 2, 1. Then the count and count + 1 again at the other 2 times of day. The load as PM-0006-13 with N set to 48, 36 and 18 (the goal modes and the three-team start mode's upper end) | the figure is the lowest count of the 3 sessions (proposed: a budget takes the worst case); validity as PM-0006-13 (bracket B, LG's 80 % rule, steal reported, never ADR 0001's V-S). A run plan with every session's VM-hours is written before the first session (it feeds PM-0006-18) | R6-COST-REC1's mark (r6 lines 714–716); the counting rule (draft, spike 2, proposed there); R6-COST-OQ1 (r6 lines 880–882); J6 (2) and OQ-0001-2 (the stand-in's load) |
END

### FIX IE-5 (routed by infra-engineer: PM-0006-15 checks the rule list per VM role)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room on RS; with no rule for a port, a probe to a listening service on that port gets no answer (a TCP connect times out, never "refused"; a UDP probe gets no reply); the firewall's rule list holds the UDP range, the TCP range and part 9's metrics rule (PM-0006-16), and 0 other rules. The `wss` endpoint presents a certificate the four browsers accept with no warning, and which one is recorded: the draft names none for development, and the WebSocket constructor takes only a URL and sub-protocols (R2-ROOM-F12, r2 lines 1494–1500), so a self-signed certificate cannot be pinned for `wss` | RS; ML: Chrome, Edge, Firefox; MAC: Safari; a probe from ML | WebdriverIO opens the netcode proof test's client page with the transport forced by a URL flag; an attempt succeeds when the session is ready and one datagram or message echoes back; the probe opens a TCP connection and sends a UDP datagram to one closed port of each kind, with a test listener on each; the rule list is read from Hetzner's console or API and pasted into the results | 20 attempts per browser and transport (proposed: R2-TRN-REC5's count); 1 probe pair. Bite: a rule added for the probed TCP port makes the probe connect (the negative test can fail) | R6-HOST-REC2's mark (r6 lines 669–671); part 5 ("and nothing else"); R6-HOST-F18 (r6 lines 169–174); R2-TRN-REC3 (r2 lines 2639–2659) |
AFTER:
| PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room on RS; with no rule for a port, a probe to a listening service on that port gets no answer (a TCP connect times out, never "refused"; a UDP probe gets no reply); the firewall's rule list for each VM role equals part 5's list for that role, with no rule outside it and every source limited as listed (the game VM under test: the UDP range, the TCP range, part 9's metrics rule with MV as its only source, PM-0006-16, and SSH by key from the named source addresses only; staging's delivery path and TCP 80 for http-01 only where part 5 names them). The `wss` endpoint presents a certificate the four browsers accept with no warning, and which one is recorded: the draft names none for development, and the WebSocket constructor takes only a URL and sub-protocols (R2-ROOM-F12, r2 lines 1494–1500), so a self-signed certificate cannot be pinned for `wss` | RS; ML: Chrome, Edge, Firefox; MAC: Safari; a probe from ML | WebdriverIO opens the netcode proof test's client page with the transport forced by a URL flag; an attempt succeeds when the session is ready and one datagram or message echoes back; the probe opens a TCP connection and sends a UDP datagram to one closed port of each kind, with a test listener on each; the rule list is read from Hetzner's console or API and pasted into the results | 20 attempts per browser and transport (proposed: R2-TRN-REC5's count); 1 probe pair. Bites: a rule added for the probed TCP port makes the probe connect (the negative test can fail); an SSH rule opened to every source fails the list comparison | R6-HOST-REC2's mark (r6 lines 669–671); part 5 ("and nothing else"); R6-HOST-F18 (r6 lines 169–174); R2-TRN-REC3 (r2 lines 2639–2659) |
END

### FIX IE-13 (new OQ-0006-20: the spike's bill, Mustafa)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - *Carries:* no research id (the test review's finding 2 and its question 2; R6-COST-OQ1 stays with the netcode
    proof test).
AFTER:
  - *Carries:* no research id (the test review's finding 2 and its question 2; R6-COST-OQ1 stays with the netcode
    proof test).
- **OQ-0006-20. What may the netcode proof test's VMs cost?**
  - *Why it matters:* R6 sized "the spike's own bill (a few hours of one or two VMs) under 5 EUR" (r6 lines 716–717),
    but the pass marks now plan far more: on RS alone about 23 hours for this record's rows (the infra review's
    arithmetic on the rows' run plans: PM-0006-14 about 14.3 h, PM-0006-13 3 × 1.1 h, PM-0006-17 about 2.2 h, the soak
    and logging rows about 3 h), 23 × 0.1386 = 3.19 EUR, before ADR 0001's 3-run rows (perhaps 15 to 30 RS-hours,
    about 2 to 4 EUR) and the CX33 hours of SH, LG and MV; so a 5 EUR cap over everything would send the plan back to
    the draft at its planning check.
  - *Options:* (a) 5 EUR for this record's own operations runs (R6's scope), with ADR 0001's runs on their own line
    of the run plan; (b) one cap set from the run plan, written as arithmetic at R6-HOST-F15's prices before the first
    session; (c) 5 EUR over everything, as written. Until it is answered, PM-0006-18's planning check writes the run
    plan's cost and the plan goes to Mustafa before any VM is created. The test review recommends (b): both records'
    runs bill to one Hetzner project.
  - *Decider:* Mustafa (money).
  - *What decides it:* the run plan's hours (PM-0006-14 writes it before the first session) at the hourly prices.
  - *Carries:* R6-COST-REC1's mark (r6 lines 716–717; it stays adopted, no new disposition); R6-HOST-F15 (r6 lines
    150–159).
END

### FIX IE-13 (PM-0006-18: the cap points to OQ-0006-20)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-18 | The spike's own hosting bill (part 8) | under 5 EUR excluding VAT for every Hetzner VM-hour and traffic block the netcode proof test creates (all its marks, ADR 0001's included); planning check before any VM is created: the run plan's hours at 0.1386 EUR (CCX23) and 0.0144 EUR (CX33) an hour sum under 5 EUR (5 ÷ 0.1386 = 36 CCX23-hours if nothing else ran), else the plan goes back to the draft first (proposed: a bill found too high afterwards cannot be undone) | the Hetzner project of the spike; MP (Mustafa reads the bill) | VMs are created per session and deleted after it (billing rounds up to the hour, R6-HOST-F17); Mustafa reads the project's costs (the operations checklist, step 2) | 1 reading after the last session | R6-COST-REC1's mark (r6 lines 716–717); R6-HOST-F15 hourly prices (r6 lines 150–159); R6-HOST-F17 (r6 lines 164–168); prices exclude VAT (r6 lines 40–41) |
AFTER:
| PM-0006-18 | The spike's own hosting bill (part 8) | under the cap OQ-0006-20 sets, excluding VAT, for every Hetzner VM-hour and traffic block the netcode proof test creates (all its marks, ADR 0001's included, LG's calibrated VMs counted); planning check before any VM is created: the run plan's hours at 0.1386 EUR (CCX23) and 0.0144 EUR (CX33) an hour, the arithmetic shown, sum under that cap, else the plan goes back to the draft first (proposed: a bill found too high afterwards cannot be undone). R6's 5 EUR (5 ÷ 0.1386 = 36 CCX23-hours if nothing else ran) predates the run counts: the rows' own plans come to about 23 RS-hours for this record alone (OQ-0006-20), so until it is answered the plan's cost goes to Mustafa before the first session | the Hetzner project of the spike; MP (Mustafa reads the bill) | VMs are created per session and deleted after it (billing rounds up to the hour, R6-HOST-F17); Mustafa reads the project's costs (the operations checklist, step 2) | 1 reading after the last session | R6-COST-REC1's mark (r6 lines 716–717); R6-HOST-F15 hourly prices (r6 lines 150–159); R6-HOST-F17 (r6 lines 164–168); prices exclude VAT (r6 lines 40–41) |
END

### FIX SA-15 (routed by infra-engineer: the match seed in PM-0006-21's canary)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-21 | Logs keep no secrets or addresses (parts 9, 11) | 0 matches in a 30-minute soak's logs for every join token, every guest secret and every client address the soak used, and 0 matches for an IPv4 or IPv6 address pattern other than the VMs' own | RS, LG | the soak of PM-0006-22; the spike logs each token, secret and client address it creates into a canary file on LG (never on RS); a grep script searches every log line the supervisor shipped | 1 soak. Bite: a planted log line that prints one client's address and one token must be found | R6-OBS-REC1's mark (r6 lines 737–738); R2-ROOM-REC3 mark 3 (r2 lines 2807–2809) |
AFTER:
| PM-0006-21 | Logs keep no secrets or addresses (parts 9, 11) | 0 matches in a 30-minute soak's logs for every join token, every guest secret and every client address the soak used, and for each match's random seed (ADR 0004 part 8: never sent, logged or recorded; parts 9 and 12), and 0 matches for an IPv4 or IPv6 address pattern other than the VMs' own | RS, LG | the soak of PM-0006-22; the spike logs each token, secret and client address it creates into a canary file on LG (never on RS), and the supervisor's test build writes each match's seed into a canary file on RS's local disk outside the shipped log path (proposed: the grep needs the value it must not find); a grep script searches every log line the supervisor shipped | 1 soak. Bite: a planted log line that prints one client's address and one token must be found | R6-OBS-REC1's mark (r6 lines 737–738); R2-ROOM-REC3 mark 3 (r2 lines 2807–2809) |
END

### FIX IE-20 (routed by infra-engineer: PM-0006-6 tries the tombstone first)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-6 | PREV-DELETE: a closed pull request's build is gone from every link (part 3; decides PREV-DELETE, R5-PREV-OQ2) | 24 h after each test pull request closes (a plain deadline, GLOSSARY §1), none of its URLs (the branch alias and the hash URL of every deployment it ever had) answers HTTP 200 with that build's marker; an answer of HTTP 404, any other response without the marker, or [the access wall](../research/GLOSSARY.md#the-access-wall) counts as gone. Run with the access policy **off** (proposed: with it on, every leftover link sits behind the wall and the mark could not fail; off is the state of slices 1 and 2) | CF, TR; the probe runs from ML | four test pull requests on TR, closed by the spike's throwaway close workflow (`wrangler pages deployment delete … --force` per deployment, R5-PREV-REC3): (a) branch `cs/pd-one`, 1 deployment, closed without merge, branch kept (the `/` becomes `-` in the alias, R5-PREV-F12); (b) `cs/pd-three`, 3 deployments, merged, branch deleted; control (c), an open pull request; bite (d), a closed one with the close workflow switched off. A cookie-less probe reads every URL every hour from the close to 24 h after | 1 run per case (a one-off deadline). Control: (c)'s URLs serve its build at every probe. Bite: (d) is reported as still serving at 24 h. The hourly probes record when each URL stopped serving | R5-PREV-REC3's mark (r5 lines 1622–1624); PREV-DELETE (r6 lines 970–976); R5-PREV-F13 (r5 lines 845–849); R6-HOST-F4: a hash URL per deployment plus a branch alias (r6 lines 80–86) |
AFTER:
| PM-0006-6 | PREV-DELETE: a closed pull request's build is gone from every link (part 3; decides PREV-DELETE, R5-PREV-OQ2) | 24 h after each test pull request closes (a plain deadline, GLOSSARY §1), none of its URLs (the branch alias and the hash URL of every deployment it ever had) answers HTTP 200 with that build's marker; an answer of HTTP 404, any other response without the marker, or [the access wall](../research/GLOSSARY.md#the-access-wall) counts as gone. Run with the access policy **off** (proposed: with it on, every leftover link sits behind the wall and the mark could not fail; off is the state of slices 1 and 2) | CF, TR; the probe runs from ML | four test pull requests on TR, closed by the spike's throwaway close workflow (`wrangler pages deployment delete … --force` per deployment, R5-PREV-REC3): (a) branch `cs/pd-one`, 1 deployment, closed without merge, branch kept (the `/` becomes `-` in the alias, R5-PREV-F12); (b) `cs/pd-three`, 3 deployments, merged, branch deleted; control (c), an open pull request; bite (d), a closed one with the close workflow switched off. A cookie-less probe reads every URL every hour from the close to 24 h after | 1 run per case (a one-off deadline); if a case fails, part 3's first workaround, a tombstone page with no build marker deployed as the branch's last deployment before the rest are deleted, is tried in the same spike, and the mark is read again on it. Control: (c)'s URLs serve its build at every probe. Bite: (d) is reported as still serving at 24 h. The hourly probes record when each URL stopped serving | R5-PREV-REC3's mark (r5 lines 1622–1624); PREV-DELETE (r6 lines 970–976); R5-PREV-F13 (r5 lines 845–849); R6-HOST-F4: a hash URL per deployment plus a branch alias (r6 lines 80–86) |
END

### FIX IE-18 (routed by infra-engineer: PM-0006-7's result has a stated consequence)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-7 | Whether a direct upload counts against Pages' 500 builds a month (part 1; R6-HOST-REC1's question for the spike) | reported, no pass number: the project's monthly build count before and after 10 direct uploads; counts if it rose by 10, does not if by 0, anything else recorded as read | CF; MP (Mustafa reads the count in his own dashboard) | 10 deployments by the throwaway deploy workflow between two readings (the hosting checklist, step 4) | one pair of readings; passes when both are recorded | R6-HOST-REC1 (r6 lines 633–634); R6-HOST-F3 (r6 lines 72–79) |
AFTER:
| PM-0006-7 | Whether a direct upload counts against Pages' 500 builds a month (part 1; R6-HOST-REC1's question for the spike) | reported, no pass number: the project's monthly build count before and after 10 direct uploads; counts if it rose by 10, does not if by 0, anything else recorded as read; if uploads count, part 1's consequence follows (only the newest push per pull request is deployed, and the month's deploys are counted) | CF; MP (Mustafa reads the count in his own dashboard) | 10 deployments by the throwaway deploy workflow between two readings (the hosting checklist, step 4) | one pair of readings; passes when both are recorded | R6-HOST-REC1 (r6 lines 633–634); R6-HOST-F3 (r6 lines 72–79) |
END

### FIX SA-1 (routed by infra-engineer: a new row, PM-0006-32, for spike 1's B9 test)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| **The operations marks inside the netcode proof test** (ADR 0001's spike) | | | | | | |
AFTER:
| PM-0006-32 | B9's interim position holds: a pull request cannot reach a deploy secret or production (part 13, B9; spike 1; with ADR 0002's PM-0002-66) | 3 of 3: a test pull request on TR whose deploy job is edited to print a marker gets no secret until the `preview` environment's required reviewer approves it (the job prints only whether the secret is set, never its value); a job in a pull request that names the `production` environment is refused by its `main`-only deployment-branch rule; a deploy of the production branch tried with the `preview` environment's token is refused (the preview token cannot publish production; if no token scope the check finds can refuse it, the result is recorded as not refused and goes to OQ-0006-13); recorded: how the branch rule treats a pull request's merge ref (not read), and Mustafa's revocation of the test token when the check ends | TR, CF; MP (Mustafa approves or withholds the review, and revokes the token) | three throwaway workflow runs on TR; each run's log and the environment's deployment history pasted into the results log | 1 run per case. Control: after the reviewer approves, the same job reports the secret set. Bite: with the required reviewer removed, the edited job must report the secret set (the check can fail) | the draft's spike 1, B9's bullet (proposed there, from the security review's finding 1); part 13, B9; R5-PREV-REC1 (the deploy token held only by its job, as part 13 cites it) |
| **The operations marks inside the netcode proof test** (ADR 0001's spike) | | | | | | |
END

### FIX SA-1 (the intro: PM-0006-32 named)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
Written by test-engineer (plan step 4). One row per mark: the preview hosting check's first (PM-0006-1 to PM-0006-10),
then this record's operations marks inside the netcode proof test (PM-0006-11 to PM-0006-27), then the crash-report
marks that run only if the EU Sentry organization exists (PM-0006-28 to PM-0006-31). Every number is a research mark
AFTER:
Written by test-engineer (plan step 4). One row per mark: the preview hosting check's first (PM-0006-1 to PM-0006-10,
then PM-0006-32, added at the final round), then this record's operations marks inside the netcode proof test
(PM-0006-11 to PM-0006-27), then the crash-report marks that run only if the EU Sentry organization exists (PM-0006-28
to PM-0006-31). Every number is a research mark
END

### NOTE IE-2, EA-7 (my half; unit A's half)
fixed in PM-0006-13 and -14: their validity is bracket B and LG's 80 % rule, with steal recorded per second and
reported, never ADR 0001's V-S. Unit A's PM-0001-27 must read: "8 rooms as 8 processes: this is PM-0006-13's set of
runs, read the same way (no pinning, bracket B against the single-room figure, 3 sessions on SH, each process's CPU
recorded, steal reported); V-S's 'alone on its vCPU' and steal clauses do not apply", and PM-0001-1's bite exempts
the rooms-per-VM rows (PM-0001-27, PM-0006-13, -14). engine-architect's SPIKE_PLAN line ("one set of runs, read the
same in both tables") then holds as written.
END

### NOTE IE-9/10 (unit A's half)
fixed in ADR 0006's LG key. Unit A's ADR 0001 LG (its key) follows the same sentence, and PM-0001-25 runs twice: the
per-address cap off for LG (the never-`Join` flood), then on, as a bite (the second connection over the cap from one
address is refused).
END

### NOTE IE-13 (new question; routed)
the 5 EUR cap is a cost, so it is not decided here: OQ-0006-20 (Mustafa), and PM-0006-18 points to it. routed:
infra-engineer's spike item 2 ("the spike's own hosting bill under 5 EUR") reads "under the cap OQ-0006-20 sets";
engine-architect's docs/adr/README.md lists OQ-0006-20 (Mustafa, money).
END

### NOTE IE-16
fixed in PM-0006-11 (`lscpu`'s threads per core and the sibling map; the transport kept off the tick's sibling). Unit A's
PM-0001-1 records the same and pins the same way.
END

### FIX engine-architect's ADR 0005 frame batch (b88714d) and follow-up (72cfcc3): PM-0005-5's pool-report leg, per target
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-5 | No file sizes an allocation past a reviewed bound (CD-2; content item 2) | for every bounded collection, a file one element over its bound refused (100 %) and a file at exactly its bound accepted (100 %, a control: proposed); 0 collections in the content types without a maximum length | CI | the bound of every collection read from the generated schema (`maxItems`, `maxProperties`) by script; a file per bound added to C; the schema scan lists any collection with no bound | 1 run. Bites: a `Vec` field added without a bound | R1-RR-OQ2 (lines 2933–2935); CD-2 (proposed there: "one element over its bound") |
AFTER:
| PM-0005-5 | No file sizes an allocation past a reviewed bound (CD-2; content item 2) | for every bounded collection, a file one element over its bound refused (100 %) and a file at exactly its bound accepted (100 %, a control: proposed); 0 collections in the content types without a maximum length; and the whole-set check's pool report (CD-2): for each test mode, every pool's reported worst case, and for active effects the reserved worst case per target (the pick effects of the tracks that can apply at once, one per track level; the unit's own ability and weapon effects; spawn protection), equals a count by hand from the test content's numbers in 100 % of cases, and every pool the content leaves without a bound (effects others apply, weapon pickups, big vehicles) is named with its missing cap, never filled with a default (content item 2) | CI | the bound of every collection read from the generated schema (`maxItems`, `maxProperties`) by script; a file per bound added to C; the schema scan lists any collection with no bound; the hand counts written beside the test content before the run (proposed: a count made after the report could copy it) | 1 run. Bites: a `Vec` field added without a bound; a projectile lifetime doubled in the test content while the report stays the same must fail the comparison; a pool given a default cap where the content sets none | R1-RR-OQ2 (lines 2933–2935); CD-2 (proposed there: "one element over its bound") |
END

### FIX EA-9 (PM-0005-10: the local-play leg)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-10 | Development reload: checked at once, simulation numbers at the next match, client content live (CD-7; content item 5; shared with PM-0005-21) | each of 20 saves of a weapon's damage change (the Shabzon's 8 → 9) validated on the development server within 1 s of the save (a plain limit, max over the 20), and the next match runs on it; the running match's per-tick state hashes after the save equal a control run with no save (proposed: CD-7's "nothing in a running match's simulation changes"); each of 20 malformed saves leaves the running match on its numbers and shows the error (file, with line and column or field path) within 1 s; each of 20 saved feel-kit changes shows on the page within 1 s, in the running match | ML: the development server native; the page in Chrome, Edge and Firefox | the development server's watcher (notify with a debouncer); a script saves the files on a fixed schedule; validation time from the file's modification time to the server's "valid" or "error" log line on the same clock; the page's arrival time stamped on the same machine (the file's route to the Worker is the boundary spike's, PM-0002-31) | 20 saves of each kind per browser. Bites: a watcher that validates only on the next match start (fails the 1 s); a reload that swaps the numbers inside the running match (fails the hash comparison) | R1-DATA-REC5 marks 1, 4 (lines 2559–2563), restated for CD-7; CD-7 (proposed there: the feel-kit 1 s) |
AFTER:
| PM-0005-10 | Development reload: checked at once, simulation numbers at the next match, client content live (CD-7; content item 5; shared with PM-0005-21) | each of 20 saves of a weapon's damage change (the Shabzon's 8 → 9) validated on the development server within 1 s of the save (a plain limit, max over the 20), and the next match runs on it; the running match's per-tick state hashes after the save equal a control run with no save (proposed: CD-7's "nothing in a running match's simulation changes"); each of 20 malformed saves leaves the running match on its numbers and shows the error (file, with line and column or field path) within 1 s; each of 20 saved feel-kit changes shows on the page within 1 s, in the running match; and the same in local play, where slices 1 and 2 have no server crate: each of 20 saves pushed to the Worker (PM-0002-31's route) is validated there within 1 s, the next local match runs on it, and the running local match's per-tick hashes equal a control run with no save (CD-7: "on the development server and in local play alike") | ML: the development server native, and the Worker's local play; the page in Chrome, Edge and Firefox | the development server's watcher (notify with a debouncer); a script saves the files on a fixed schedule; validation time from the file's modification time to the server's "valid" or "error" log line on the same clock; the page's arrival time stamped on the same machine (the file's route to the Worker is the boundary spike's, PM-0002-31) | 20 saves of each kind per browser. Bites: a watcher that validates only on the next match start (fails the 1 s); a Worker that applies the pushed numbers to the running local match (fails the hash comparison); a reload that swaps the numbers inside the running match (fails the hash comparison) | R1-DATA-REC5 marks 1, 4 (lines 2559–2563), restated for CD-7; CD-7 (proposed there: the feel-kit 1 s) |
END

### FIX TE-8 (routed by game-designer: PM-0005-18 also reorders the definitions)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-18 | The test content runs with no ability-specific Rust code (AB-1; ability item 1) | 0 matches for the test content's ids and legacy names in the sim crate's non-test code; every test content item, fired or pressed in a scripted scenario, produces its expected event log; with every test id replaced by a random string, the scenario's event log is byte-identical after mapping the ids back (proposed: a search for names misses code keyed by a constant spelled otherwise); reported: the list of operations with the test items that use each, for the rust-reviewer | CI | `rg` over the sim crate excluding tests; the scenario in the replay runner; the id rename by script over the test content | 1 run. Bites: a planted `if id == "molotov"`, and a planted constant `FIREBOMB` keyed on the Molotov's handle (only the rename test sees it) | R1-ABL-REC1 mark 1 (lines 2365–2368); AB-1 |
AFTER:
| PM-0005-18 | The test content runs with no ability-specific Rust code (AB-1; ability item 1) | 0 matches for the test content's ids and legacy names in the sim crate's non-test code; every test content item, fired or pressed in a scripted scenario, produces its expected event log; with every test id replaced by a random string and the definitions reordered in data, the scenario's event log is byte-identical after mapping the ids back (proposed: a search for names misses code keyed by a constant spelled otherwise); reported: the list of operations with the test items that use each, for the rust-reviewer | CI | `rg` over the sim crate excluding tests; the scenario in the replay runner; the id rename by script over the test content | 1 run. Bites: a planted `if id == "molotov"`, and a planted constant `FIREBOMB` keyed on the Molotov's handle (only the rename and reorder test sees it, whatever order the handles are given in) | R1-ABL-REC1 mark 1 (lines 2365–2368); AB-1 |
END

### FIX GD-8 (review-game-designer finding 9: PM-0005-24's minute-5 Tech 1 floor)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-24 | Tech tiers gate abilities and purchases (AB-5; ability item 4) | an ability press before the team's Tech 1, and a T2 purchase before Tech 2, refused by the authority in 1,000 of 1,000 scripted attempts each; the same presses and purchases after the tier accepted in 1,000 of 1,000 (a control: proposed) | CI | scripted team state at Tech 0, 1 and 2; the authority's verdict log | 1 run. Bites: an ability with no tier tag | AB-5 (proposed there; LOOP_V2 §4 lines 50–52, `[DECIDED]`; INV GE4, line 290); the count 1,000 from R1-ABL-REC4 (line 2444) |
AFTER:
| PM-0005-24 | Tech tiers gate abilities and purchases (AB-5; ability item 4) | an ability press before the team's Tech 1, and a T2 purchase before Tech 2, refused by the authority in 1,000 of 1,000 scripted attempts each; the same presses and purchases after the tier accepted in 1,000 of 1,000 (a control: proposed); and the minute-5 floor (LOOP_V2 §4 `[DECIDED]`: "Every team reaches Tech 1 by minute 5 at the latest"): a team with no cashout reaches Tech 1 at the floor's tick, read from data (5 × 60 × 30 = 9,000 ticks at 30 Hz: arithmetic), and its presses are refused at tick 8,999 and accepted from tick 9,000 | CI | scripted team state at Tech 0, 1 and 2; the authority's verdict log | 1 run. Bites: an ability with no tier tag; a floor that is never applied (the team without a cashout stays at Tech 0 past tick 9,000) | AB-5 (proposed there; LOOP_V2 §4 lines 50–52, `[DECIDED]`; INV GE4, line 290); the count 1,000 from R1-ABL-REC4 (line 2444) |
END

### FIX GD-8 (review-game-designer finding 8: PM-0005-25's third leg), GD-7 (routed by game-designer: the immunity case)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-25 | The counters reproduce from data alone (AB-6, AB-7; ability item 5) | from the imported tables: the Warrior kills a Conscript in 0.86 s within one tick (13 SMG body hits, 12 × 0.072 = 0.864 s); the Peacekeeper kills a Warrior in 0.85 s within one tick when 11 of its 16 pellets land over two shots (11 × 11 = 121 ≥ 120 HP), and 10 of 16 leave it alive (a control: proposed); a Peacekeeper behind the Shield takes exactly 63 SMG body hits (62 leave 1.2 HP) and 19 without it; the knockback operation changes a target's velocity by the test weapon's value within 1e-6 relative, with 0 matches for the test weapon's id in the sim's non-test code | CI | scripted duels on the converted data (the pellet hits scripted, since spread draws from an [authority-only stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream)); the times from the sim's events; the knockback weapon is test-only (the Javelin's numbers are Mustafa's) | 1 run (deterministic). Bites: the Shield block applied to blasts only | AB-6, AB-7 (proposed there); R1-ABL-REC3's mark (lines 2416–2418: the 0.86 s, ADR 0004's, PM-0004-9); INV CD13 (line 94); arithmetic in AB-7 |
AFTER:
| PM-0005-25 | The counters reproduce from data alone (AB-6, AB-7; ability item 5) | from the imported tables: the Warrior kills a Conscript in 0.86 s within one tick (13 SMG body hits, 12 × 0.072 = 0.864 s); the Peacekeeper kills a Warrior in 0.85 s within one tick when 11 of its 16 pellets land over two shots (11 × 11 = 121 ≥ 120 HP), and 10 of 16 leave it alive (a control: proposed); a Peacekeeper behind the Shield takes exactly 63 SMG body hits (62 leave 1.2 HP) and 19 without it; the triangle's third leg (the Conscript beats the Peacekeeper, AB-7): a Molotov blast deals a Peacekeeper the same damage with the Shield up as with it down (a blast skips the frontal block, GC7), and at 35 m a Bagpipe pellet deals 0 while an AMK body hit deals 34 (ranges 34 m and 300 m, AMK damage 34: LEGACY_TUNING.md, `WEAPONS`, lines 173 and 181, as ability item 5 cites them); AB-2's immunity rule: a soldier inside its spawn protection takes 0 damage from a body hit and from a blast, and full damage once the effect's 90 ticks end (3 × 30 at 30 Hz), under every option of OQ-0005-AB14; the knockback operation changes a target's velocity by the test weapon's value within 1e-6 relative, with 0 matches for the test weapon's id, or the spawn-protection effect's, in the sim's non-test code | CI | scripted duels on the converted data (the pellet hits scripted, since spread draws from an [authority-only stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream)); the times from the sim's events; the knockback weapon is test-only (the Javelin's numbers are Mustafa's) | 1 run (deterministic). Bites: the Shield block applied to blasts only; the Shield block applied to blasts as well (the third leg fails); an immunity rule that skips blasts; the protection's length taken from a constant instead of the effect's data (changing the data to 60 ticks must move the end) | AB-6, AB-7 (proposed there); R1-ABL-REC3's mark (lines 2416–2418: the 0.86 s, ADR 0004's, PM-0004-9); INV CD13 (line 94); arithmetic in AB-7 |
END

### FIX GD-8 (review-game-designer finding 13; routed by game-designer: PM-0005-29 follows u4, a lost gun)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-29 | u4: a lost gun's track waits for the next gun of its type (AB-8) | the player drops W and takes another W from the world: the player's W picks apply to it; a second player who takes the first gun gets their own W picks and none of the first player's; 1,000 of 1,000 scripted exchanges (proposed count) | CI | scripted drops and claims (ADR 0004's pickup entities, PM-0004-66) | 1 run. Bites: picks stored on the gun | AB-8 (proposed there); UPGRADE-SCOPE |
AFTER:
| PM-0005-29 | u4: a lost gun's track waits for the next gun of its type (AB-8) | the player holds a looted weapon of type X and loses it on death (dropped by the fixture's drop rule, a test rule while OQ-0005-AB9 is open; no voluntary drop or swap, which OQ-0005-AB1 and AB9 leave open), then takes another X the fixture places: the player's X picks apply to it; a second player who takes the first gun gets their own X picks and none of the first player's; 1,000 of 1,000 scripted cycles (proposed count) | CI | scripted deaths, fixture drops and claims (ADR 0004's pickup entities, PM-0004-66) | 1 run. Bites: picks stored on the gun | AB-8 (proposed there; ability item 6, u4); UPGRADE-SCOPE |
END

### FIX GD-5 (PM-0005-31: distinctness left to OQ-0005-AB8), GD-8 (routed by game-designer: an offer of the data's size, equal logs on native and Wasm)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-31 | u6: only offered picks count, and offers repeat (AB-8) | every offer holds the data's offer size (3) of distinct candidates whose requirements hold; a pick that was not offered, or for a closed offer, refused in 1,000 of 1,000 forged picks; 1,000 of 1,000 offered picks accepted (a control: proposed); the same seed and inputs give byte-identical offer logs on two runs, and on native and Wasm only if ADR 0004 puts the offers in a stream the client holds | CI | forged pick commands; the offer log written by the authority | 2 runs (the log); 1 run (the counts). Bites: an authority that accepts any candidate of the track; offers drawn from a clock seed | AB-8 (proposed there); UPGRADE-SCOPE; SEC-SPREAD (a)'s reasoning |
AFTER:
| PM-0005-31 | u6: only offered picks count, and offers repeat (AB-8) | every offer holds the data's offer size (3) of candidates whose requirements hold, distinct or not as the test track's data says (whether candidates are always different, and what a track with fewer left than the offer's size does, are OQ-0005-AB8's: here fixture rules, not game rules), with one test track that has fewer candidates than the offer's size handled as its data says; a pick that was not offered, or for a closed offer, refused in 1,000 of 1,000 forged picks; 1,000 of 1,000 offered picks accepted (a control: proposed); the same seed and inputs give byte-identical offer logs on two runs, and on native and Wasm: the offers stream is authority-only (ADR 0004 part 8), but in local play the authority runs in the browser's Web Worker (ADR 0002) | CI (native; Wasm in Node) | forged pick commands; the offer log written by the authority | 2 runs per target (the log); 1 run (the counts). Bites: an authority that accepts any candidate of the track; offers drawn from a clock seed; an offer size hard-coded as 3 (changing the data to 2 must change every offer) | AB-8 (proposed there; ability item 6, u6); UPGRADE-SCOPE; SEC-SPREAD (a)'s reasoning |
END

### FIX GE-1 (PM-0005-34: the material per slot content; routed by graphics-engineer), GE-10 (a .gltf's external buffers and images)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-34 | The models load on both backends and carry no look of their own (GPU section parts 1, 6) | 0 console errors in every start; 0 meshes of a listed asset drawn with a material other than its slot class's shared material (identity against the material cache); 0 model requests other than listed glTF/GLB files, and 0 requests outside the client's host | ML: Chrome, Edge, Firefox, each on WebGPU and with `forceWebGL`; MAC: Safari, both backends | PM-0003-2's starts; after load, a script walks the scene graph; the page's network log | PM-0003-2's 10 starts per browser and backend. Bites: one slot left with its glTF material | the GPU section's spike part (R3-WGPU-REC1 mark 1, r3 lines 910–912); R5-GATE-REC3's G12 mutation (r5 line 1723) |
AFTER:
| PM-0005-34 | The models load on both backends and carry no look of their own (GPU section parts 1, 6) | 0 console errors in every start; 0 meshes of a listed asset drawn with a material other than the shared material of its slot's content (identity against the material cache, which is keyed by slot content: the class, the faction for faction slots, and the slot's own base and emissive colour, textures, side and alpha mode, GPU part 1); 0 model requests other than listed glTF/GLB files and the buffers and images a listed `.gltf` references, and 0 requests outside the client's host | ML: Chrome, Edge, Firefox, each on WebGPU and with `forceWebGL`; MAC: Safari, both backends | PM-0003-2's starts; after load, a script walks the scene graph; the page's network log | PM-0003-2's 10 starts per browser and backend. Bites: one slot left with its glTF material | the GPU section's spike part (R3-WGPU-REC1 mark 1, r3 lines 910–912); R5-GATE-REC3's G12 mutation (r5 line 1723) |
END

### FIX GE-1 (PM-0005-35 counts distinct slot contents; routed by graphics-engineer)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-35 | Materials are shared by content (GPU section parts 1, 2, 7) | the material objects on listed assets' meshes number at most the distinct (slot class, faction) pairs those meshes show, on every frame | CI (a count); ML: Chrome on WebGPU and with `forceWebGL` | LS with every test model in each faction's colours; distinct material objects counted by identity, pairs counted from the slots' classes and factions | 1 run per backend (counts repeat, PM-0003-35). Bites: a loader that clones the class material per mesh (legacy's per-mesh conversion, CL4) | the GPU section's spike part (proposed there; R3-LOOK-F3, r3 lines 331–334) |
AFTER:
| PM-0005-35 | Materials are shared by content (GPU section parts 1, 2, 7) | the material objects on listed assets' meshes number at most the distinct slot contents those meshes show (GPU part 1's key: the class, the faction for faction slots, and the slot's own base and emissive colour, textures, side and alpha mode), two meshes with the same slot content share one material object, and each mesh's material carries its own slot's base colour and textures (read back and compared with the loaded file), on every frame | CI (a count); ML: Chrome on WebGPU and with `forceWebGL` | LS with every test model in each faction's colours; distinct material objects counted by identity, distinct slot contents counted from each slot's key fields in the loaded files | 1 run per backend (counts repeat, PM-0003-35). Bites: a loader that clones the class material per mesh (legacy's per-mesh conversion, CL4); a loader that keys materials by (slot class, faction) only, which must fail the colour comparison (two slots of one class with different base colours then share one material) | the GPU section's spike part (proposed there; R3-LOOK-F3, r3 lines 331–334); GPU part 1; legacy's content signature (CL4, `LEGACY_INVENTORY.md` line 113) |
END

### FIX FB-12 (PM-0005-42: the faction's colour value in its data, CD-13), GE-11 (a pixel probe per backend)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-42 | Every slot is classed and every faction slot coloured, on both backends (GPU section parts 2, 6) | 0 slots without a class; every faction colour slot's colour equal to its faction definition's colour value, for every faction and test model; 0 console errors | ML: Chrome, Edge, Firefox on both backends; MAC: Safari | the research's processed test models in LS; colours read back from the materials and compared with the faction data | 10 starts per browser and backend (PM-0003-2's count). Bites: a faction slot given another faction's colour | the GPU section's spike part |
AFTER:
| PM-0005-42 | Every slot is classed and every faction slot coloured, on both backends (GPU section parts 2, 6) | 0 slots without a class; every faction colour slot's colour equal to the faction's colour value in its data (CD-13's client definition), for every faction and test model, read from the materials; and drawn on each backend: in an unlit debug view, a pixel probe at a faction slot of each test model returns that colour within 2/255 per channel (proposed tolerance: reading materials proves the data path, which is the same on both backends, not that each backend draws the colour); 0 console errors | ML: Chrome, Edge, Firefox on both backends; MAC: Safari | the research's processed test models in LS; colours read back from the materials and compared with the faction data | 10 starts per browser and backend (PM-0003-2's count). Bites: a faction slot given another faction's colour; a backend path that drops the slot's colour (the probe must see it while the material still reads right) | the GPU section's spike part |
END

### FIX AD-5 (the LOD checklist: the test views, not a camera decision)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
#### The LOD checklist (Mustafa, in Brave, in the asset spike; only with PM-0005-44's condition)
1. Each test model with LOD levels at the closest and the farthest camera framing (LOOK_V2 §5). Expected: simple and
   readable from a distance, real depth up close; the silhouette kept at the farthest framing.
2. A slow zoom through each switch distance and back. Expected: no visible pop or flicker at a switch.
AFTER:
#### The LOD checklist (Mustafa, in Brave, in the asset spike; only with PM-0005-44's condition)
Every view is at ADR 0003's test view (legacy's framing, CD5: height 40, boom 56.6, field of view 15°, yaw 45°) or a
named variant of it, never a camera decision: the v2 framing is R2-INT-OQ2's, and both steps are taken again at its
own closest and farthest once that question is answered.
1. Each test model with LOD levels at two test views: the test view itself, as the far case, and the same camera at
   half its distance to the player, as the close case (ADR 0003's look checklist, step 9) (LOOK_V2 §5). Expected:
   simple and readable from a distance, real depth up close; the silhouette kept at the far view.
2. The camera moved slowly from the close view out to twice the test view's distance and back (a test path through
   each switch distance, proposed: legacy has no zoom). Expected: no visible pop or flicker at a switch.
END

### FIX GD-8 (review-game-designer finding 14: Feel Lab step 3, refused with no start shown)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
4. Record: his answer to ABL-PREDICT (does the client predict the player's own abilities), and whether it differs per
   ability.
AFTER:
3. Before his team reaches Tech 1, press each ability with A and with B. Expected: the press is refused in both, with
   no start shown, since the client knows its team's tier (team state goes to the team, ADR 0001) and the ability's
   required tag (AB-5), so a predicted variant can refuse the press locally. Record: any start that shows, in either
   variant, as a defect of that variant.
END

### FIX AD-5 (families step 1: the test view), AD-12 (families step 4: GW18's builders)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
1. The test building as an authored model and as the ported kit, at the game camera, beside legacy's house (LOOK_V2
   §2, §3). Expected: the half-timbered house with the brick-tile roof reads in both. Pick: authored, kit, or not yet.
2. The props (barrels, carts, hay bales, logs) as authored models and as the ported kit, beside legacy (LOOK_V2 §2).
   Pick: authored, kit, or not yet.
3. The ground and its textures as an authored asset (if shown) and as the ported procedural textures. Pick: authored,
   kit, or not yet.
4. Trees: not judged; the tree style is LOOK_V2 §8's `[OPEN]` item. Expected: trees shown are labelled placeholders.
AFTER:
1. The test building as an authored model and as the ported kit, at ADR 0003's test view (legacy's framing, CD5; not a
   camera decision, R2-INT-OQ2), beside legacy's house (LOOK_V2
   §2, §3). Expected: the half-timbered house with the brick-tile roof reads in both. Pick: authored, kit, or not yet.
2. The props (barrels, carts, hay bales, logs) as authored models and as the ported kit, beside legacy (LOOK_V2 §2).
   Pick: authored, kit, or not yet.
3. The ground and its textures as an authored asset (if shown) and as the ported procedural textures. Pick: authored,
   kit, or not yet.
4. Trees: no pick; the trees shown are legacy's tree builders ported as labelled placeholders (GW18), judged in ADR
   0003's look checklist (step 15) for variety and lushness only; the tree style is LOOK_V2 §8's `[OPEN]` item.
END

### NOTE ADR 0005's BEFOREs (for the session's script)
ADR 0005's blocks copy their BEFORE from the doc at 72cfcc3, as the coordinator asked; "### Pass marks" is unchanged
between 75615d1 and 72cfcc3, so each also occurs once at 75615d1.
END

### NOTE GE-1 (routed back)
fixed in PM-0005-34 and -35 (the distinct slot contents, a colour read-back that a (class, faction) key fails). Nothing
more to route: graphics-engineer's GPU parts 1 and 7 already carry the key.
END

### NOTE AD-5 (0003 and 0005)
fixed: ADR 0003's key defines the test view (CD5's framing, not a camera decision), step 9 names its two test views,
and ADR 0005's families and LOD checklists use them; every step is taken again at the v2 framing once R2-INT-OQ2 is
answered. routed (optional): graphics-engineer's part 11 may say the look proof test runs at that test view.
END

### NOTE EA-9, GD-5, FB-12, GE-10, GE-11
fixed in PM-0005-10 (local-play leg), PM-0005-31 (distinctness left to OQ-0005-AB8, a short track handled as its data
says), PM-0005-42 (the colour from CD-13's client definition; a pixel probe per backend), PM-0005-34 (a `.gltf`'s
buffers and images). No routing.
END

