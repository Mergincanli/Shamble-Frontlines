# docs-keeper, 4c-1 docs pass (notes)

## Plan (brief-docs-keeper-4c1.md, plus the caller's additions)
1. `place-4c.mjs fix-16`, then `loop-check-4c.mjs` and `section14-4c.mjs check` (both must pass).
2. ROADMAP: one terse entry for 4c-1 under "Recent work"; keep 8-12, fold the oldest.
3. STATUS: 4c's state; follow-ups GE-F2 (with GE2's few-pixels note), GE-F3+P-2, GE-F4, AD-4, TE2-1..TE2-5 (owners);
   4B-TYPES-READER, 4B-MEMBER-KINDS, 4B-BOOT-CALLS met; his item 12 b and c quoted by script; HEADROOM-FEET open.
4. REPO_MAP: new/moved/removed files; run repo-map-check.
5. Drift fixes (START_HERE, MODULE_MAP, TECHNICAL) with file:line.
6. Checks' output here. Never commit.

## Log
- Step 1 (fix-16), outputs, all exit 0:
  - `node .checkpoints/cs4c-soldier/place-4c.mjs fix-16`: `docs/LOOP_V2.md:16: his pointer moved to the end of its sentence, right after "centre."`
  - `node .checkpoints/cs4c-soldier/loop-check-4c.mjs`: `docs/LOOP_V2.md: main's 260 lines unchanged but the header's one sentence and his pointer at lines 16, 33, 35, 52, 151, 154, 156, 181; section 14 from line 262` / `docs/EDITOR_V2.md: main's 134 lines, his pointer at lines 47`
  - `node .checkpoints/cs4c-soldier/section14-4c.mjs check`: `docs/LOOP_V2.md: section 14 stands once, last, after section 13; 517 words and signs, his item 8's, in order`
  - `git diff --stat`: docs/LOOP_V2.md 1 line changed (rr-body-proto-7.rs modified beforehand: rust-reviewer's, not mine).
- Facts read (2026-11-28): #79 = dae6d107, committer date 2026-11-27T23:12:03Z (git log -1 dae6d107); risk tier 1,
  needs-mustafa (advisor-reread-4c.md:53); 4B-TYPES-READER "Met for the keys", 4B-BOOT-CALLS "Met" (review-s-engine-architect.md:31,
  :41, :50-51), 4B-MEMBER-KINDS met (:29, :50-51); his item 12 b/c: answers file lines 25-26 = DECISIONS.md:220-221;
  no change-set named for PM-0004-60's nightly 10 million or PM-0004-20's 48-soldier run (build plan
  docs/reports/2026-11-17-build-plan-slice-1.md:654 "its nightly run is CI work", :626 "the stress scene's form at
  Slice 6"; reread-sheet.md and cs4-walk/plan.md name none); no 4c report in docs/reports yet; HEADROOM-FEET =
  question file lines 331-347, open. GE-F4 and TE2-1..5: no change-set named by the review.
- Drift grep: START_HERE, MODULE_MAP, TECHNICAL hold no claim on the v2 page (empty-scene, buildEmptyScene, LOCAL_MAP,
  setContent, render counts, Picture): nothing to fix there. REPO_MAP rows 96, 99, 100, 113, 122, 123, 124-126, 128,
  129, 133 stale or missing; content-map.test.mjs has no row.
