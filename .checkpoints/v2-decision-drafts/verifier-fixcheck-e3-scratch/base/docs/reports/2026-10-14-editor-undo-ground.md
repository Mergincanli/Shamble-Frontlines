# Advisor report: editor-undo-ground (2026-10-14, branch cs/editor-undo-ground, base main ef019e0)

## Goal
In the map editor, Ctrl+Z / Ctrl+Y / Reset / Load no longer change how the map looks, and undo no longer quietly
duplicates every tree and prop.

## Outcome
DONE as planned, including the Advisor's draw-call addition. Both reviews PASS WITH NOTES, nothing to fix. Waiting on the
Advisor's APPROVE and Mustafa's Brave screenshots (visual change).

## What changed (shamble-game.js only, 7 insertions / 4 deletions; symbols +0 −0, matches the plan)
- Root cause: `stylizeScene()` runs once at boot. Every editor rebuild (`buildMap`) re-created roads, crates, walls,
  HQ / refinery / power, barracks and the resupply terminal on plain `MeshStandardMaterial` with no ink outline. That is
  the "ground textures change": 83 road tiles lost their cel shading and ink.
- `addBox`, `addPath`, `makeBuilding` (body + trim), `makeBarracks` (lintel, floor, roof, panel), the `buildMap` terminal:
  now call `stylizeOne()` themselves, guarded as `makePlatform` already does. At boot nothing changes: `stylizeScene`
  skips meshes that are already toon.
- `clearMap`: now removes and disposes `trees[]` and `props[]` like `covers[]`. Before, each rebuild stacked another copy
  of all 70 trees and 36 props, and the copies went into undo snapshots and saved maps.

## Evidence (base ef019e0 → head a4d7289)
| Measure (Cabana, `tools/preview.js eval`, real code) | Base | Head |
|---|---|---|
| Rebuild harness assertions (look, counts, snapshot round-trip) | FAIL | PASS |
| Roads / crates / walls / buildings / barracks / terminal after a rebuild | MeshStandard, no ink | MeshToon + ink (= boot) |
| Trees / props at boot, after 1, after 2 rebuilds | 70/36 · 140/72 · 280/144 | 70/36 at every step |
| **Draw calls, full map: boot / 1 / 2 rebuilds** (Advisor addition) | 1992 / 2406 / 3502 | 1992 / 1992 / 1992 |
| Triangles, full map: boot / 2 rebuilds | 206,304 / 581,868 | 206,304 / 206,304 |
| Draw calls, gameplay views spawn / village / base, boot → after rebuilds | 187/135/127 → 171/134/127 (ink lost) | 187/135/127 throughout |
| Geometries, boot / 10 rebuilds · JS heap after GC, boot / 10 rebuilds | 1037 / 3777 · 42.9 / 119.5 MB | 1037 / 1037 · 42.7 / 44.5 MB |
| Undo chain (re-serialized each time) after 5 rebuilds | 18,846 calls, 278 MB heap | 1992 calls, flat heap |
| Real editor.html page, 3D canvas pixels changed by one Ctrl+Z | 57,854 px | 124 px (noise ≈ 70 px) |
- Raw files: harness `eug/rebuild.js`, results `eug/rebuild-{base,head}.json`, page shots `eug/pg-{base,head}_{0boot,1undo,2redo}.png`
  (session scratchpad). The perf numbers are the perf-auditor's independent measurements on detached worktrees.
- Boot render A/B at gameplay framing: spawn and village 0 px changed. The friendly-base framing has 61 px from a
  draw-order tie on one ink edge. The manor framing differs run to run on *both* builds (unseeded house textures,
  queued as house-textures-seeded).
- Gate on a4d7289: PASS, all 8 checks (1–4, 5a smoke, 5b names, 5c layering, 5d script order). No sim.js: no gameplay or
  AI code changed (the diff touches only map builders and `clearMap`, which never run per frame).

## Reviews (workflow: art-director and perf-auditor, each finding verified by a second agent)
- **art-director: PASS WITH NOTES.** A1: newly placed roads, crates, HQ and barracks now get the boot cel shading and
  ink (intended, Brave check). A2: rebuilt map matches boot. A3 (low): accept the 61 px tie, don't reorder the build.
  A4: the outline on flat road tiles reads fine. A5, A6: pre-existing, out of scope (the editor playtest camera clips the
  barracks roof; road grey vs the documented dirt colour, which belongs to the painted-ground step). A2 also noted a 2 cm
  terminal snap from `serializeMap`'s 0.1 m rounding (pre-existing).
- **perf-auditor: PASS WITH NOTES.** The rebuild leak is gone (table above). Rebuild time is the same within noise
  (median 181–187 vs 188–212 ms). Head compiles no shader on the first rebuild; base links one extra program, which is a
  first-Ctrl+Z hitch in Brave. `SIM.statics` stays at 220 (base: 1180 after 10 rebuilds, making line-of-sight rays about
  8× slower in an editor playtest).
  - **Deferred: PERF-2 (low).** `_disposeObj3D` disposes the shared, cached `BK` textures. Houses and covers already did
    this on base; trees and props now add the well roof's `tile` texture. r128 re-uploads it on the next frame
    (1 extra 256×256 upload per rebuild) and nothing breaks. Fix later: skip `BK._tex` textures in `_disposeObj3D`. It
    is a small follow-up, outside this plan.

## Decisions and deviations
- None from the plan. Calibration notes and baselines were not saved to agent memory (reviewers are read-only).

## Risks and open questions
1. Maps saved after an undo on old builds still contain duplicate trees and props. This fix stops further growth. The
   next change-set, map-dedupe-on-load, drops exact duplicates when a map loads.
2. PERF-2 above. Recommendation: a one-line follow-up when convenient. It is not urgent.
3. Not verified: Brave on a real GPU, the size of the base build's first-Ctrl+Z hitch, refinery / power / hostile
   building variants (same builder as HQ), the editor's ortho view.

## For Mustafa in Brave (editor.html)
1. Note the roads, crates, HQ and barracks at the start (screenshot). Place a crate, press Ctrl+Z, Ctrl+Y, Ctrl+Z.
   Everything must look exactly like the first screenshot, black edges included (screenshot).
2. Press Reset map: same check.
3. Place a new road tile and a new crate: they should now have the same shading and black outline as the existing ones
   (screenshot).
4. Optional: after several Ctrl+Z, the DEV-panel FPS should match the FPS right after boot.

## Proposed next step
map-dedupe-on-load: when a saved map loads, drop exact duplicates only (same kind, position, rotation and seed).
