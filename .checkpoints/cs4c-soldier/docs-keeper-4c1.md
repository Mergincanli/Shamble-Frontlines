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
- Steps 2 and 3 (ROADMAP, STATUS, the archive) by `node .checkpoints/cs4c-soldier/docs-4c1.mjs` (it refuses unless each
  file is as read; asserts STATUS <= 120 lines, every moved line in the archive, every folded line once, and his item
  12 b and c equal to DECISIONS.md:220-221): `STATUS 120 lines; archive +68 lines; ROADMAP 471 -> 485 lines; 14 STATUS
  lines moved, 13 ROADMAP lines folded; quotes: 175 and 136 characters`, exit 0.
  - STATUS: :5 Last updated; :13 Last merged (+#79 dae6d107); :34 4c-1's row in place of 4b-2's; :39-40 Next item 2;
    :47 Waiting on Mustafa (4c-1's Brave check, HEADROOM-FEET open in plain words, no answer; MOVE-RULES row 9 open);
    :97-103 the 4b bullet (4B-TYPES-READER, 4B-MEMBER-KINDS, 4B-BOOT-CALLS met) + "From change-set 4c-1" (GE-F2 with
    the few-pixels note, GE-F3/P-2, GE-F4, AD-4, TE2-1..5, owners) + his two marks quoted by script (none named yet);
    :116 open risks + 4c-1's two.
  - Archive: In flight, a dated OLD-LINES block (#79's row, old :5, :13, :39-40, :47, :97-103, :116, byte for byte);
    Follow-ups, "Met by 4c-1" in the 4b-2 block + block "From change-set 4c-1"; Open bugs and risks, block 4c-1 (WebGPU
    unseen; the browser run's 217 s); Health, a 4c-1 row (render counts verified in render-counts.json).
  - ROADMAP: 4c-1's entry on top of Recent work (12 entries kept); the preview hosting check folded into Build history
    after 3a's entry by line range; the #79 pointer under 4b-2's entry; a dated pointer in "Next / open".
- Step 4, REPO_MAP (Edit tool): rows packages/types/src/content-schema.test.mjs, packages/sim-worker/src/ (local-map.ts),
  recording.ts, build-checks.mjs (DEV_MARKERS), picture.ts, packages/client/src/app/, content.ts, content.test.ts, new row
  content-map.test.mjs, content-ties.test.mjs, render/ (scene.ts, inventory.ts, scene.test.ts; empty-scene.ts gone),
  overlay/ (report rows), e2e skeleton/; "Last checked: 2026-11-28".
- Step 5, drift: none in START_HERE, MODULE_MAP, TECHNICAL (no claim on the v2 page there; grep above).
- Step 6, final checks: `node .claude/scripts/repo-map-check.mjs`: `repo map OK: 265 entries cover 110 required paths`
  (exit 0); loop-check-4c.mjs and section14-4c.mjs check: same outputs as step 1 (exit 0); `wc -l docs/STATUS.md`: 120;
  `git diff --stat`: docs/LOOP_V2.md 2, REPO_MAP.md 27, ROADMAP.md 42, STATUS.md 26, STATUS_ARCHIVE.md 68 (plus
  rust-reviewer-read.md, not mine). Nothing committed by me.
- After the advisor tool's final read: STATUS:103's tail reworded with the Edit tool (quotes untouched): the build plan's
  :654 "its nightly run is CI work" and :626 "the stress scene's form at Slice 6" name no change-set or build; whether
  that stress scene is the 48-soldier run [assumed: open]. ROADMAP entry: the overview's no-width case worded as the
  code does it. REPO_MAP: docs/LOOP_V2.md row gains §14 [verified LOOP_V2.md:262]; content.test.ts and recording.ts
  rows tagged [verified]. START_HERE:31, TECHNICAL:249 name LOOP_V2 / DECISIONS only as files: no drift. ROADMAP's
  header holds no date line.
- Re-run: repo-map-check `repo map OK: 265 entries cover 110 required paths` (exit 0); loop-check-4c.mjs and
  section14-4c.mjs check as in step 1 (exit 0); STATUS 120 lines; `git diff --stat` docs: LOOP_V2 2, REPO_MAP 29,
  ROADMAP 43, STATUS 26, STATUS_ARCHIVE 68 (game-designer-read.md, rust-reviewer-read.md, gd-trace-diff.py: not mine).
- Still owed (not mine to pick): the change-set for PM-0004-60's nightly 10 million and the build for PM-0004-20's
  48-soldier run; GE-F4 and TE2-1..5 have no change-set named; perf-auditor's numbers not re-measured; no 4c-1 report yet.

DONE
