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
