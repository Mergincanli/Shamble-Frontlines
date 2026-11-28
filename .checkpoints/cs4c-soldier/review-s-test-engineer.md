# Step S review: test-engineer (4c-1, the page's scene), code head c1de41e5 on main dae6d107

## Plan
1. Read the diff `git diff dae6d107 c1de41e5 -- packages crates content`, the tests first (scene.test.ts,
   content.test.ts, content-map.test.mjs, content-ties.test.mjs, report.test.ts, content-schema.test.mjs, build-checks)
   and the browser specs (skeleton.spec.ts, content.spec.ts, report.ts, render-counts.json).
2. For each claim the change makes: the test that proves it and its plant in `plants-4c.py` / `plants-4c.txt`; whether
   the plant targets the defect the test names, and whether the "wanted text" criterion can be met by a different
   failure (a test that fails for a reason other than the claim).
3. Check the plants run's provenance: it ran on acd7d4f, not on c1de41e5; whether the anchors still stand at the head.
4. Browser specs E1 to E3: what each asserts, timing dependence (waits, timeouts), and whether each can fail.
5. What is not proven: greys "never darker", the volume check's blind spot (GE-F5, not repeated), G-3 (not repeated),
   anything else found.
6. The fix pass's tests for GE-F1/EA-1 (the endless loop at aspect 0) and for G-3 / GE-F5: what they must hold and how
   a plant of a synchronous endless loop is run (outside the runner's own thread).
7. Run where a claim needs it: the anchors check, the Vitest files, under `.gate/` only.

## Notes (one line per finding, as found)
- N1 (provenance): plants-4c.txt's whole run names acd7d4f (whose code is bc7720f3's); only S1, S1b, S1c and C1 were
  re-run with the volume check in, and that re-run's header names acd7d4f too, while the volume check is c1de41e5's.
  The file says "the whole run is made again on the step's final code before its reviews": no such run is recorded.
  acd7d4f..c1de41e5 is 11 lines added to scene.test.ts alone, so the 28 of 28 carries over by reasoning; the anchors
  stand at the head (my run of `--anchors`: 31 edits of 31 plants, 0 wrong).
- N2 (gap, own plants on a scratch copy `.gate/te-review/`, script `.gate/te-review/plants-te.py`, control 8 of 8
  passed): NOT CAUGHT by the 8 scene tests: P1 each patch drawn over half its depth; P2 every floor quad wound downward
  (culled from above: ground and patches vanish); P3 the map's ground drawn at the empty floor's 30 m, not the bounds;
  P4 the grid's x lines from z 0, not minZ; P5 the overview ten times farther than the nearest fit; P6 the block greys
  in four levels. None of the floor quads (ground, patches) has a corner or winding check in Node; the volume and
  outward tests iterate MAP.shapes only (scene.test.ts:71).
- N3 (fix-pass method, measured): a synchronous endless loop is not stopped by Vitest's per-test timeout (probe
  `.gate/te-review/hang/spin.test.mjs` with a 2 s timeout: no return in 20 s), and subprocess.run's timeout kills only
  the vitest parent: its fork worker kept spinning, orphaned (pid seen, then killed by me). A worker thread with a
  deadline does stop it: `.gate/te-review/hang-probe.mjs` ran scene.ts's overview in node:worker_threads (Node 22.23's
  own type stripping): 16:9 answered in 1.7 ms; aspect 0 and NaN gave no answer in 3 s and Worker.terminate() ended
  both (6.1 s for the three).
- N4 (checked, no finding): every grey level 0 to 255 reads back from three's Color exactly as written
  (`.gate/te-review/client/grey-roundtrip.mjs`: 0 of 256 differ), so the inventory's grey is the authored one.
- N5 (checked, no finding): E1 reads the scene after running() (90 blocks, after ready; delivery is synchronous in the
  message handler), E2 asserts content "none" so no later delivery is possible (#kept is null), E3 asserts content
  "none" and scene null: none of the three depends on a race.
- N6 (scope of N2): P1 to P6 ran against the 8 Node scene tests only. That the browser specs miss P1 to P3 is by
  reading, not by a run: E1 compares patches by id (skeleton.spec.ts:61), solids by x set and top y (:62-67), and the
  render counts count submitted triangles, so a quad wound downward or moved keeps them. Not verified in a browser.
- N7 (no seam): no test imports render/three-picture.ts (grep: only render/index.ts does). The frame on resize
  (three-picture.ts:102), the camera's far rule (:51) and the swap (:69-77) are proven only through E1's counts and
  E1b's plant; the camera is not readable from outside.
- N8 (fix-pass method, measured inside Vitest): `.gate/te-review/client/src/render/hang-thread.test.mjs` runs overview
  in a worker thread from a Vitest test: 16:9 passes; aspect 0 fails in 3.0 s on its own assertion and the run exits
  (3.24 s). So the hang plant can be scored by the test's own words. It rests on Node's own type stripping (on by
  default in this Node 22.23.5); `engines` is 22.x and CLAUDE.md says 22.13+, which admits versions where type
  stripping sits behind a flag: unverified there.
- N9 (GE-F5's planned fix, measured): on a box raised to (5, 5, 5), the volume from the origin finds a missing floor
  (9.333 against 6) but not GE-F5's own case, the floor as one triangle drawn twice (6.000); measured from the box's
  centre that case is 6.000 too (a rectangle's two halves have equal area). A closed-mesh check (each directed edge
  once and its reverse once) finds both: 4 and 7 edges unpaired, 0 on the right box (node -e scripts, this session).

## Claim, proof, gap
| Claim of the change | Proof at c1de41e5 | Gap |
|---|---|---|
| The map is read field by field, closed key sets, own properties, fixed sentences | content.test.ts (7 new tests), content-map.test.mjs; plants T1 to T8 caught | none found |
| Formats, versions and folders are the loader's | content-ties.test.mjs; T9 caught | EA-3 (the word lists) |
| Member kinds match the schema | content-schema.test.mjs with its own negative case; T10 caught | none found |
| Each solid at its own f32 corners | scene.test.ts:47-67; S1 caught | the browser compares x and top y only (TE-6) |
| Faces outward, flat normals, each solid closed | scene.test.ts:69-104; S1b, S1c caught | blind on origin planes (GE-F5); a duplicated triangle passes even on a raised map (TE-2) |
| Each patch and the ground at their own corners, facing up | none: ids only (scene.test.ts:106-113, skeleton.spec.ts:61) | TE-1 (P1, P2, P3 not caught) |
| Ground and grid at bounds.min[1] | scene.test.ts:149 checks the literal 0 | G-3 |
| Grid: one line per metre over the bounds | scene.test.ts:143-150 (count, y); S4 caught | endpoints unchecked (TE-4, P4) |
| Greys: neutral, higher never darker, tallest lightest, ground darker | scene.test.ts:115-141; S3, S3b caught; every grey reads back exactly (N4) | strictly lighter held at one pair (TE-5, P6) |
| Old floor freed once, floors share nothing | scene.test.ts:152-179; S5, S5b, E1b caught | none found |
| The overview shows the whole floor, as near as fits | scene.test.ts:181-201 at three aspects; S6 caught | aspect 0 hangs (GE-F1); nearest not held (TE-3, P5) |
| The inventory reads what is drawn | scene.test.ts:203-213; S7, S7b caught | none found |
| The development cut | build-checks.test.mjs; E4 caught | none found |
| The drawn scene equals the built map (E1) | skeleton.spec.ts:54-82; E1 caught | patches by id, solids by x and top (TE-1, TE-6) |
| A refused bundle leaves the empty floor (E2) | content.spec.ts:86-93; E2 caught | none found |
| A broken Worker leaves the content none (E3) | skeleton.spec.ts:145-150; E3 caught | none found |
| The plants: 28 of 28 caught, 3 controls held | plants-4c.txt | two runs, both headed acd7d4f; the whole re-run not recorded (TE-8) |

## What the fix pass's tests must hold (GE-F1 / EA-1, G-3, GE-F5)
1. The overview at aspect 0, NaN, a negative aspect and Infinity, and the loader's largest map (655.34 m a side) at
   0.001: each returns, and its value is asserted (the camera left as it was, or a stated "no view"), not only that it
   returned; 16:9, 4:3 and 1:1 still give today's values.
2. Two plants: the cap removed (hangs), and the aspect guard removed with the cap kept (returns a far or non-finite
   camera: caught only by the value assertion).
3. How the hang plant is run: inside the test, by a worker thread with a deadline and Worker.terminate() (N8: aspect 0
   fails in its own words in 3.0 s; 16:9 answers in 1.7 ms, a margin above 1,000 times; such a deadline can fail
   falsely under heavy load but never pass falsely). State the Node floor it needs (N8). If it is not used,
   plants-4c.py's run() (plants-4c.py:28-32, no timeout) hangs for good: it then needs a deadline with
   start_new_session and os.killpg (N3: killing only the vitest parent leaves its fork worker spinning), its own
   scoring line ("killed after N s" counts as caught only when the clean control finished well under N), and a check
   afterwards that the files are back and no node process is left.
4. Where the guard sits: no test reaches three-picture.ts (N7). Put the decision in `overview` (or another function a
   Node test imports), or the guard has no proof.
5. G-3 and GE-F5: the raised map, with plants of floorY = 0 at each use (ground, patches, grid, overview target), and a
   closed-mesh check (N9) with plants "the floor as one triangle drawn twice" and "one face missing": the raised map
   alone does not close GE-F5's own failure case.
6. TE-1, TE-3, TE-4 in the same pass. Then the whole plant set on the fix head, `--anchors` first. Any new browser test
   reports the browser run's new total against 217 s and is never thinned to fit CI's limit (his rule, 2026-11-28).

## Verdict: REVISE
1. TE-1 Medium. The ground and the patches have no corner or winding check: `scene.ts:170-176` (floorQuad), `:210`
   (the ground), `:223-227` (regionMesh). The outward and volume tests iterate MAP.shapes only (`scene.test.ts:71`),
   the names test reads ids (`:106-113`), E1 reads patch ids (`skeleton.spec.ts:61`). Failure case, planted on a copy:
   each patch over half its depth (P1), every floor quad wound downward so the ground and street vanish from above
   (P2), the ground at 30 m, not the bounds (P3): all 8 scene tests pass. On today's test scene Mustafa's Brave check
   would be the only catch. Fix: each quad's four corners as the bounds' or region's min and max (f32), its triangles'
   right-hand normals +y, at bounds.min[1] (with G-3); plants P1 to P3.
2. TE-2 Low. GE-F5's planned fix does not close GE-F5's own case: on a raised box the volume from the origin, and from
   the centre, is 6.000 with the floor as one triangle drawn twice (N9). Fix: a closed-mesh check (each directed edge
   once, its reverse once) beside the volume (`scene.test.ts:79-89`). Claiming GE-F5 closed by the raised map alone
   would be evidence that does not hold.
3. TE-3 Low. "As near as fits" (`scene.ts:258-261`) is not held: P5 (the camera ten times farther) passes
   `scene.test.ts:181-201`. Failure case: the map drawn small in the middle of the screen. Fix: at each aspect, some
   corner within 1e-3 of the 0.92 margin.
4. TE-4 Low. The grid's "spans the bounds" (`scene.test.ts:148`) is not checked: P4 (x lines from z 0) passes. Fix:
   each line's two ends at the bounds.
5. TE-5 Low. Greys: the comment's rule at `scene.test.ts:125-128` (lighter wherever tops differ by more than a level's
   share) is held at one pair; P6 (four grey levels) passes. Confirms the brief's "not proven"; fix: the rule over
   every pair of the made map's tops and a sweep of tops.
6. TE-6 Low. E1 says each solid is drawn "at its own corners" but compares the x set and the top only
   (`skeleton.spec.ts:62-67`): z, the bottom and a ramp's direction go unchecked in the browser. Fix: compare each
   solid's whole sorted point set with the map's corners computed in the spec (no extra browser time).
7. TE-7 Low. No test imports `three-picture.ts`: the frame on resize (`:102`), the far rule (`:51`) and the swap are
   proven only by E1's counts and E1b. It decides where GE-F1's guard can be tested (fix-pass item 4).
8. TE-8 Low. `plants-4c.txt:1` names acd7d4f for the whole run; only S1, S1b, S1c and C1 were re-run with the volume
   check, under the same header, and the promised whole re-run is not recorded. It carries over by reasoning
   (acd7d4f..c1de41e5 is 11 lines added to scene.test.ts; the anchors stand, 31 of 31). Fix: the whole run on the fix
   head.

Not verified: P1 to P6 against the browser specs (by reading only, N6); the worker-thread method below Node 22.18
(N8); the WebGPU path. Checked correct: the reading, ties, schema kinds, report rows, disposal, inventory and
development-cut tests each fail on their plant; E1 to E3 depend on no race (N5).

DONE
