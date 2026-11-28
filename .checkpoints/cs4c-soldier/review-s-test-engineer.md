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
- N1 (provenance, Low candidate): plants-4c.txt's whole run is on acd7d4f (code = bc7720f3); only S1, S1b, S1c and C1
  were re-run, with c1de41e5's volume lines in the tree uncommitted, and its header still names acd7d4f. The text says
  "the whole run is made again on the step's final code before its reviews": not done. acd7d4f..c1de41e5 is +11 lines
  in scene.test.ts alone, so the 28 of 28 carries over by reasoning; the anchors stand at the head (my run of
  --anchors: 31 edits of 31 plants, 0 wrong).
- N2 (gap, own plants on a scratch copy .gate/te-review/, script .gate/te-review/plants-te.py, control 8 of 8 passed):
  NOT CAUGHT by the 8 scene tests: P1 each patch drawn over half its depth; P2 every floor quad wound downward (culled
  from above: ground and patches vanish); P3 the map's ground drawn at the empty floor's 30 m, not the bounds; P4 the
  grid's x lines from z 0, not minZ; P5 the overview ten times farther than the nearest fit; P6 the block greys in four
  levels. None of the floor quads (ground, patches) has a corner or winding check in Node, and E1 checks patches by id
  alone (skeleton.spec.ts, the region line); the volume and outward tests iterate MAP.shapes only (scene.test.ts:71).
- N3 (fix-pass method, measured): a synchronous endless loop is not stopped by Vitest's per-test timeout (probe
  .gate/te-review/hang/spin.test.mjs with a 2 s timeout: no return in 20 s), and subprocess.run's timeout kills only
  the vitest parent: its fork worker kept spinning, orphaned (pid seen, then killed by me). A worker thread with a
  deadline does stop it: .gate/te-review/hang-probe.mjs ran scene.ts's overview in node:worker_threads (Node 22.23's
  own type stripping): 16:9 answered in 1.7 ms; aspect 0 and NaN gave no answer in 3 s and Worker.terminate() ended
  both (6.1 s for the three).
- N4 (checked, no finding): every grey level 0 to 255 reads back from three's Color exactly as written
  (.gate/te-review/client/grey-roundtrip.mjs: 0 of 256 differ), so the inventory's grey is the authored one.
- N5 (checked, no finding): E1 reads the scene after running() (90 blocks, after ready; delivery is synchronous in the
  message handler), E2 asserts content "none" so no later delivery is possible (#kept is null), E3 asserts content
  "none" and scene null: none of the three depends on a race.
