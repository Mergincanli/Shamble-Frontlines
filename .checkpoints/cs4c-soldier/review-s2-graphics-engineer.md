# Step S review, round 2: graphics-engineer (4c-1, the page's scene), on what changed: c1de41e5..2b2f44c2

## Plan
1. Read the fix in `packages/client/src/render/scene.ts` (`overview`, `MOST_DOUBLINGS`, `frameCamera`),
   `three-picture.ts` (`frame`, `resize`), `inventory.ts`, `boot.ts`, and the new tests in `scene.test.ts`.
2. GE-F1: does the search end at every aspect a page can produce, and does the cap stay inside the probe's far plane?
   Measure with a small script on the real `scene.ts` (Node 22, strip-types).
3. GE-F5: does the volume check now see every face, and does the edge check close the case the volume cannot?
4. Anything new in what changed: the null path, the far rule's move, the width clamp, the inventory's material check.

## Notes
- N1 (GE-F1, closed). `scene.ts:294-298`: the doubling loop checks `fits(2^k)` for k = 0..19 and returns `null` after
  the 20th miss; the largest distance tried is 2^19 = 524,288 m, and the farthest corner then lies at most about
  524,288 + 930 m away, inside the probe's far of 1e6 m (`:281`), so the overshoot case of round 1 cannot happen below
  the cap. Measured on the real `overview` (`.checkpoints/cs4c-soldier/ge-review2/edges.mts`, run this session, every
  call under 3 ms) on two bounds: a 112 x 24 m stand-in of test-map size, and the largest the loader allows. Aspect 0,
  NaN, Infinity, 1e-300 and 1/4320 give `null` on both; 1/1080 gives null on the largest (the stand-in frames at 141 km);
  0.0015 frames the largest at 509 km; 1, 16:9 and 1920 frame both (largest: 1,010 m at 1:1, 750 m at 1920, the round-1
  numbers). `frameCamera` on a 1x1080 camera with the largest bounds returns false and leaves far at 500.
  `three-picture.ts:97` clamps the width as well as the height, so `innerWidth` 0 gives aspect 1/height, a positive
  number; NaN cannot come from `innerWidth`, and `overview` refuses it anyway. The plant with no limit hangs and is
  ended by the outer timeout (`plants-4c.txt:30`).
- N2 (GE-F5, closed). The volume is now taken from (-1000.5, -1000.25, -1000.125) (`scene.test.ts`, the
  `solidsFaceOutward` helper), off every face's plane on both maps, so a missing or misplaced floor changes it. The case
  I named (a floor of one triangle drawn twice) keeps the volume from any point, because a flat triangle's share is
  area x height / 3 whatever its shape (TE-2 was right); the new edge check (every directed edge once, its reverse once)
  catches it, and the plant does (`plants-4c.txt:37`, F8). The raised map also covers G-3 for the ground, the patches
  and the grid.
- N3 (far rule moved, no finding). `frameCamera` (`scene.ts:314-322`) is the old `frame` body word for word, with the
  null check in front; the test checks `far == max(500, 4 x distance)` and the unchanged camera on false.
- N4 (inventory, no finding). `inventory.ts:14-15,25`: the `satisfies` list and the material check add no GPU cost;
  `readInventory` is development-only (`three-picture.ts:76`).
- N5 (note, no finding; part of the deferred GE-F2). Windows a few pixels wide still frame: at aspect 0.0015 the largest
  map's camera stands 509 km off, far about 2e6 m against near 0.1, so depth precision is gone and everything
  depth-fights; the floor is then about a pixel wide, so nothing a player could judge. GE-F2's follow-up (near scaled to
  the distance, or reversed depth) covers it.
- N6 (unchanged, not verified). `resize` still passes width 0 to `renderer.setSize` (`three-picture.ts:95`); only the
  aspect is clamped. What a 0-wide canvas does on the WebGPU backend (a validation error through the console hook, or
  nothing) was not tested here (no adapter), and it is not in what changed. The comment at `:96` says the window "is held
  at one pixel": true of the aspect only.

## Verdict: APPROVE
- GE-F1: closed. The search ends at every aspect (at most 20 tries, measured under 3 ms), the cap stays inside the
  probe's far plane, a floor it cannot frame leaves the camera as it was, and the width is clamped at the resize.
- GE-F5: closed, by the off-plane volume together with the edge-pairing check (the volume alone cannot catch a
  duplicated floor triangle; the edge check does, plant F8).
- New: nothing at Critical, High or Medium. One note folded into the deferred GE-F2 (N5), one unverified pre-existing
  behaviour (N6, the 0-wide canvas), neither a reason to push.

DONE
