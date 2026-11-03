# touchup4 test-engineer fixes (D3E-2, D3E-3, D3C-1)

Branch claude/laughing-lovelace-ox61se, docs as at 8c68179 (the last commit touching docs/adr/0001, 0003). Every BEFORE copied by script
(touchup4-test-engineer-scratch/mk_pm2.py, mk_step7.py). No doc edited, no commit, no push, no fetch.

D3E-2 and D3E-3 edit the same table line (ADR 0003 PM-0003-2), so they share one block: a second block's BEFORE could
not be copied from the file on disk.

### FIX D3E-2+D3E-3
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-2 | One build runs on both backends in every target browser (part 1) | 0 console errors and 0 device errors in every start | ML: Chrome, Edge and Firefox (Windows), each on WebGPU and with `forceWebGL`; MAC: Safari, both backends | LS played through its whole script; the page's own error hook writes every console error and page error to the results log (no browser driver needed), and counts as a device error each `uncapturederror` event of the WebGPU device, its `lost` promise resolving, and each `webglcontextlost` event on the canvas (proposed, not researched: a WebGPU validation error reaches the page through the device's error event rather than as a thrown error, and whether each browser also prints it to the console, as an error, a warning or not at all, was not verified; the spike records how each browser shows the planted one) | the 10 starts of PM-0003-5 per browser and backend; 0 in each. Bites: a `console.error` 2 s after load (R5-GATE-REC3's G12 mutation, r5-toolchain-quality.md line 1723); on the WebGPU backend, a planted bind group on the renderer's device that leaves out one entry of its layout, which must count as a device error whatever the console shows | R3-WGPU-REC1's mark 1 (lines 910–912); the device events proposed (the graphics review's finding 14) |
AFTER:
| PM-0003-2 | One build runs on both backends in every target browser (part 1) | 0 console errors and 0 device errors in every start | ML: Chrome, Edge and Firefox (Windows), each on WebGPU and with `forceWebGL`; MAC: Safari, both backends | LS played through its whole script; the page's own error hook writes every console error and page error to the results log (no browser driver needed), and counts as a device error, until LS ends, each `uncapturederror` event of the WebGPU device, its `lost` promise resolving, and each `webglcontextlost` event on the canvas (the count closes before any teardown, since the device's `destroy()` also resolves `lost`, with reason "destroyed": WebGPU §22.1; the hook is proposed; as read at WebGPU §22 (basis), a validation error is not thrown but dispatched to the device, to an open error scope of its kind or else, if the browser chooses, as the device's `uncapturederror` event (§22.2–22.3); whether each browser also prints it to the console, as an error, a warning or not at all, was not verified; the spike records how each browser shows the planted one) | the 10 starts of PM-0003-5 per browser and backend; 0 in each. Bites: a `console.error` 2 s after load (R5-GATE-REC3's G12 mutation, r5-toolchain-quality.md line 1723); on the WebGPU backend, a planted bind group on the renderer's device that leaves out one entry of its layout, made where a spy on `pushErrorScope` and `popErrorScope` shows no error scope open (an error made inside an open scope of its kind goes to that scope and fires no event: WebGPU §22.3) and with a bare `uncapturederror` listener of the bite's own beside the hook, which must count as a device error whatever the console shows; if the hook counts none by LS's end, the same plant inside the bite's own `pushErrorScope('validation')` tells why: no error popped, the plant is wrong; an error popped and the bare listener hit, the hook missed it; an error popped and the bare listener silent, the browser fired no event, which the spec allows (it fires one "If the user agent chooses" and may limit how many, §22.3–22.4); in none of the three does the pass count | R3-WGPU-REC1's mark 1 (lines 910–912); the device events proposed (the graphics review's finding 14); the error and loss model: WebGPU, W3C Candidate Recommendation Draft of 15 September 2026, §22.1–22.4 and `destroy()` (read by the verifier, fixcheck-d3e, 2026-11-03) |
END

### FIX D3C-1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
7. At P2, with the rewind cap at the honest rewind's p99 the proof test measured at P2 (PM-0001-49; the overlay names
   it, so the cap clips almost no honest shot), shoot the sprinting target for two minutes. Expected: when his crosshair is on the target as he sees it, the
   shot hits; when it is off, it misses. The shot (flash, tracer) is instant; the hit marker comes a moment later, when
   the server confirms ("the shot feels instant; hit feedback arrives when the hit is confirmed", LOOP_V2 §7).
AFTER:
7. At P2, with the rewind cap at the honest rewind's p99 the proof test measured at P2 (PM-0001-49; the overlay names
   it, so the cap clips almost no honest shot), shoot the sprinting target for two minutes. Expected: when his crosshair
   is on the target as he sees it, the shot hits; when it is off, it misses. The shot (flash, tracer) is instant; the
   hit marker comes a moment later, when the server confirms ("the shot feels instant; hit feedback arrives when the hit
   is confirmed", LOOP_V2 §7).
END

D3C-1b is not in fixcheck-d3c's list. Found while checking D3C-1: steps 10 and 11 of the same checklist, from the same
commit (d1f7e13), have lines of 152 and 136 characters (0001:1526, 1528). Same words, re-wrapped; the session may
skip it.

### FIX D3C-1b
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
10. At P2, repeat step 7 with the rewind cap at 200 ms, then at 250 ms, then again at step 7's measured cap (the overlay
    names the cap). Expected: with 200 and 250 ms, shots that looked like hits but missed, since at P2 an honest shot
    needs 233–383 ms of rewind (part 8) and both caps clip most of them; with the measured cap, hits as in step 7. Record: what he noticed with each cap
    (for OQ-0001-5's later Feel Lab weighing; no pick here).
11. At P2, run from open ground into cover while bots shoot at him, 10 runs with each of step 10's three caps. Expected: being hit after
    reaching cover on his own screen is rare, and never long after. Record: how often it felt unfair with each cap
    (OQ-0001-5, the victim's side).
AFTER:
10. At P2, repeat step 7 with the rewind cap at 200 ms, then at 250 ms, then again at step 7's measured cap (the overlay
    names the cap). Expected: with 200 and 250 ms, shots that looked like hits but missed, since at P2 an honest shot
    needs 233–383 ms of rewind (part 8) and both caps clip most of them; with the measured cap, hits as in step 7.
    Record: what he noticed with each cap (for OQ-0001-5's later Feel Lab weighing; no pick here).
11. At P2, run from open ground into cover while bots shoot at him, 10 runs with each of step 10's three caps. Expected:
    being hit after reaching cover on his own screen is rare, and never long after. Record: how often it felt unfair
    with each cap (OQ-0001-5, the victim's side).
END

### NOTE D3E-skeptic (what the PM-0003-2 block adds to fixcheck-d3e's smallest fixes, and why)
- D3E-2 holds at the source (d3e scratch webgpu-excerpt.raw.txt 11273, 11281-11283, 11384-11388; excerpt2 11222-11223,
  11267-11270). d3e's wording lets the hook's own error scope decide "whether the browser or the hook missed it", but a
  scope only shows that the plant raised an error: with the error popped, the hook and the browser are still both
  suspects. So the block adds a bare `uncapturederror` listener of the bite's own (hit: the hook missed it; silent: the
  browser fired none), and a spy on `pushErrorScope` and `popErrorScope` so "outside any error scope" is shown, not
  assumed (a scope takes only errors of its kind, its `[[filter]]`, 11222-11223). The wait is "by LS's end", bounded by
  the script, not a clock. In all three outcomes the pass does not count (ADR 0003's Bites rule, lines 1126-1127).
- D3E-3 holds (excerpt2 11126-11128; raw 2327-2329). The block closes the count at LS's end, before any teardown,
  rather than counting only reason "unknown": a `destroy()` during LS is itself a defect (PM-0003-45 keeps the renderer
  across teardowns), and "unknown" alone would miss it.
- The block also turns the row's "proposed, not researched" into "the hook is proposed; as read at WebGPU §22
  (basis)": the new text cites §22.1-22.4, and appendix lines 620-621 already say fixcheck-d3e read PM-0003-2's error
  model at its source. The console part stays "not verified" (§22.3 says only "should", "for example as warnings").
END
