# Advisor: MAIN-68 pushed-head check (step 7; merge 6b4a0d3, pushed 7971da0)
2026-11-26/27, day session. Advisor agent (the one that checked rev 1 and rev 3), effort xhigh. I act by main's
advisor.md (read whole earlier in this run). The form: advisor-main-66.md Q1 (d), with C68B-1 to C68B-6
(advisor-main-68b.md) as rev 4 applied them. Read only, no cargo.

## Plan of this check
1. SHAs and parents: H0 d2d55b4, merge 6b4a0d3 (parents H0 and b9aad01), pushed 7971da0; the first-parent line; the
   later commits (U7's 6968215 among them) by rev-list and name-only.
2. The plan as run: rev 4 against C68B-1 to C68B-6 (the values, DEP-HOLD, C6, the record).
3. The merge by SHA: H0..merge equals main's 87 paths and size lines; both STATUS files resolved as C68B-4 says (rules
   and bars); the whole tree, docs/ names, spikes/ and docs/design/, the kit, raw modes, markers; the anchor T.
4. The records: step5-outputs.txt, the light-run logs, and the remote branch's reflog (no save in the no-save window).
5. CI on 7971da0 by run id (four runs, every job), and toolchain's summary lines against main's job 140114848934.
6. Re-runs in the main folder (gate, repo map, checkpoint tests, guard tests), only if the precondition holds.
7. Verdict, findings, what is owed to him, Decisions needed, For Mustafa, Status line.

## Block 1. SHAs and parents (2026-11-27 00:5x UTC)
- The merge 6b4a0d3e5dcf… has parents d2d55b45f38f… (H0) and b9aad01954be… (main). Its message is git's own ("Merge
  remote-tracking branch 'origin/main' into claude/netcode-proof-test-plan-hcky5g"), 20:18:04 UTC, with no attribution
  lines.
- The pushed save 7971da075d03… has one parent, 6b4a0d3, and touches only .checkpoints/:
  - the card;
  - security-u8-f1.md (M);
  - review-nb2-c10.md (A).

  Its message names the take-in, and it carries both trailers.
- main is still b9aad01 (ls-remote). The first-parent line from H0 runs: 6b4a0d3, 7971da0, then 18 later commits with
  one parent each, U7's 6968215 among them.
- The later commits, 7971da0..HEAD: `git diff --name-only 7971da0 HEAD -- . ':(exclude)spikes/' ':(exclude).checkpoints/'`
  prints nothing. Under spikes/: U7's 43 files (+5,193 / -47). That is A's read, not mine.

## Block 2. The plan as run (rev 4)
- main-68-plan.md rev 4 (sha256 5111e8ff…5f7f at my read) has "As checked: C68B-1 to C68B-6" applied, and in the steps:
  - C68B-1: the rev-parse at step 2;
  - C68B-2: the values, with C4 = b9aad01 and its run and job ids;
  - C68B-3: DEP-HOLD's `git diff --quiet` before step 6;
  - C68B-4: C6 as a rule with its bars;
  - C68B-5: the record line naming all six;
  - C68B-6: the Q answers.
- Its header says that where a rev 3 line differs from "As checked", "As checked" governs. That is how it ran: rev 3's
  2671595 values are kept as the record only (the numbers' heading).

## Block 3. The merge by SHA (read only)
- **H0..merge is main's move, name for name.** `git diff --name-status d2d55b4 6b4a0d3` gives the 87 paths (A 60, M
  27), the same list as `07d673b b9aad01`.
  - Whole: "87 files changed, 51544 insertions(+), 271 deletions(-)".
  - Without the two STATUS files: "85 files changed, 50926 insertions(+), 223 deletions(-)".
  - Both equal C68B-2's values. Every docs file's numstat equals main's own (DECISIONS 745/0 … STATUS 46/46,
    STATUS_ARCHIVE 572/2).
- **STATUS.md (C68B-4's rule):**
  - 120 lines, equal to main's;
  - against main, "1 1";
  - `-U0` shows main's #40 row out (the 2026-11-14 text) and H0's #40 row in (byte for byte H0's `07d673b..H0` line);
  - nothing else.
- **STATUS_ARCHIVE.md (C68B-4's rule, main's bullets first):**
  - 2,733 lines (2,720 + 13);
  - against main 18/5, equal to H0's own `07d673b..H0`;
  - `-U0` against main holds exactly H0's own five hunks' + and - lines: the Safari wording, the freeze wording, the
    9-line netcode bullet, the checkpoint.mjs citation and the two table rows;
  - the order: "## Open bugs and risks" :2384, MAIN-WALL :2385, guard-wrappers :2399, netcode proof test :2410, CI smoke
    :2419.
- **The other two shared docs.** DECISIONS.md 24/1 and REPO_MAP.md 17/0 against main: H0's own lines, both auto-merged.
- **No conflict markers** under docs/, .claude/, .github/, crates/ or packages/.
- **The anchor.** T = 83d650e2fb3d2c6a3a3c8505480fb92936f949ac (step1-record.txt).
  `git diff --quiet T 6b4a0d3 -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'
  ':(exclude)docs/STATUS_ARCHIVE.md'` exits 0.
- **The whole tree.**
  - It equals main outside spikes/, .checkpoints/ and docs/ (exit 0).
  - docs/ against main: the usual six names.
  - spikes/, docs/design/ and .checkpoints/ equal H0's (exit 0).
  - The kit, CI and manifest files are 100644 and equal main's blobs: routine-guard.mjs 4bb6411, its test 9a9ffd0,
    toolchain.yml 4f2e88d, Cargo.lock 47176e0, content's Cargo.toml 597dfb5, devtools' ccdad81.
- **Trees.** 2,042 files at H0, 2,102 at the merge (+60): refs N0 + 60, as C68B-2 says.

## Block 4. The records
- **step1-record.txt:**
  - ls-remote b9aad01 before step 1;
  - the resume line by Edit;
  - the saver stopped by TaskStop. Its "terminated" autosave d2d55b4 is H0, pushed by the saver itself before the window
    opened; pgrep exit 1;
  - the fetch: C4 holds;
  - T with its bar: two CONFLICT files, the four auto-merge lines, one block each;
  - C68B-4's four preconditions, each with its command;
  - the base light run;
  - 7,610 MiB free.
- **step5-outputs.txt:** steps 2-6, every bar of C68B-2 and C68B-4 met as section 3 here reads them by SHA.
  - 5a: GATE PASS (base b9aad01).
  - 5b: "repo map OK: 245 entries cover 110 required paths".
  - 5c: 33/33.
  - 5d: refs 2102 = 2042 + 60, the rest word for word.
  - 5e: 1551/1551, 0 fail (35.0 s).
  - Step 6: C68B-3's diff exit 0; the save not held.
- **The light runs.** light-base.log (HEAD d2d55b4) and light-after.log (HEAD 6b4a0d3): both exit 0, header hashes
  e3b0c442… both, refs 2042 then 2102. Marks "81 marks read and checked (drafts at 626df678…); rows … all unchanged",
  self 78 of 78, and redact 11 of 11 with 431 items and 0 findings, equal word for word. They differ only in HEAD, refs,
  times and free disk.
- **No save in the no-save window (M65-1).** The remote-tracking reflog reads fe08e5f (20:13:46), then d2d55b4 (20:14:24,
  the fetch), then 7971da0 (20:20:00, push). 6b4a0d3 was never a pushed tip on its own.

## Block 5. CI on 7971da0 (GitHub MCP tools; logs read as data)
- **Six of six jobs green,** attempt 1, 20:20:05-20:29:11 UTC:
  - push: gate run 45890271684 (gate 140150941394, smoke 140150941755) and toolchain 45890271634 (job 140150941524);
  - pull request: gate 45890273559 (gate 140150948118, smoke 140150947871) and toolchain 45890273394 (job
    140150947074).
- **#74's two steps (rev 4's step 6 bar):**
  - On the pull-request toolchain job, step 11 succeeded and printed "5 files, 892209 bytes", and step 12 succeeded
    ("Artifact preview-site successfully finalized. Artifact ID 15151327798"). The run's artifact list shows
    preview-site, 250,473 B, digest sha256:4e115adb…, expiring 2026-12-03. These equal step5-outputs.txt.
  - On the push job both were skipped.
  - No preview workflow exists on main, so nothing consumed the artifact. I did not read its contents.
- **Toolchain against step 7's baseline** (main's job 140114848934 on b9aad01). I read all three logs whole: each runs
  from the runner header ("Current runner version: '2.338.2'") to "Cleaning up orphan processes". They hold the same
  lines in the same order:
  - 103 `test result: ok.` lines (35 + 35 + 33), every one with 0 failed, 0 ignored and 0 filtered out;
  - five vitest `Test Files` lines (12, 1, 8, 2, 5 passed) with their `Tests` lines (95, 1, 51, 7, 33 passed);
  - Playwright's "1 passed", "18 passed" (Chromium) and "4 passed" (Firefox).

  None of them has a `##[error]` or a FAILED. Each has the same four "panicked at crates/devtools/src/wasm.rs:114:5:
  attempt to add with overflow" lines, which are a test's recorded data: the overflow check reported as its result.
  Only the times differ (Chromium 3.8 m on main, then 3.5 m and 3.3 m).
- **Durations.** Build-and-test took 8 min 37 s (push) and 7 min 5 s (pull request), against main's 9 min 20 s, of the
  job's 15-minute limit.

## Block 6. Re-runs in the main folder (00:5x-01:0x UTC, 2026-11-27)
- **The precondition.** `git diff --quiet 6b4a0d3 HEAD -- . ':(exclude)spikes/' ':(exclude).checkpoints/'
  ':(exclude)docs/design/'` exits 0 on HEAD 4e3d1fb. `git status --short` shows only three notes files (this file,
  advisor-u7-head.md, advisor-u8-f1-sec.md).
- `node tools/gate.mjs --no-smoke`: GATE PASS. Base merge-base = b9aad01; checks 1-4 and 5b-5d PASS; symbols +0 -0;
  5a skipped.
- `node .claude/scripts/repo-map-check.mjs`: "repo map OK: 245 entries cover 110 required paths".
- `node --test .claude/scripts/checkpoint.test.mjs`: tests 33, pass 33, fail 0. The 33rd is the DEP-HOLD case this
  take-in used: "a manifest or lockfile whose content in the push equals its content on main is not held".
- `node --test .claude/hooks/routine-guard.test.mjs` (main's file): tests 1551, pass 1551, fail 0 (32.6 s).
- Each equals step5-outputs.txt.

## Verdict: APPROVE. The merge is as recorded, and the lane may go on from this head.
- **The conflicts.** STATUS.md and STATUS_ARCHIVE.md were resolved exactly as C68B-4 says. STATUS.md is main's file
  with H0's #40 row. STATUS_ARCHIVE.md is main's file with H0's own five hunks, the netcode bullet after main's two
  new bullets.
- **Nothing else changed.** Against main, the tree differs only in spikes/, .checkpoints/ and the usual six docs names.
  Against H0, it differs only by main's 87 paths with main's own counts. The anchor holds.
- **Every bar is met:**
  - C68B-2's values;
  - C68B-3: DEP-HOLD's diff exit 0 and the save not held;
  - C68B-4: 120 and 2,733 lines, "1 1", 18 5, the content lines and the order;
  - 5a-5e;
  - no save in the window.

  My own re-runs and reads agree with the record.
- **CI.** All six jobs on the pushed head are green. Toolchain's summary lines equal the baseline's. #74's two steps met
  their bar on the pull-request run.
- **Severity.** Nothing new: nothing Critical, High, Medium or Low. Risk tier 1 for PR #40: after the take-in, its diff
  against main holds no kit or CI file.

## Findings (CLAUDE.md's scale)
| Id | Severity | What | Where it stands |
|---|---|---|---|
| ADV-M68B-4 | Low (on record) | #75's merge message and main's STATUS row record no go of his. | Owed to him as #75-GO in msg-2026-11-27.md (:79-82), with TAKEIN-CODE (:71-77) |
| ADV-M68B-5 | Low (on record) | Toolchain time: 7-9 of the job's 15 minutes. | Read before each step; nothing now |
| ADV-M68B-6 | Low (on record) | Main's stale STATUS rows (MAIN-WALL "none yet", #75 "(draft)") came in as main's text. | Main's next docs pass |
| ADV-M68-5 | On record (main's) | R4-1 Critical, open on main by his R4-END (1); ADV-R4-2. | Main's next guard change-set |
| ADV-M60-7, ADV-M54-8 | Low (open) | As before. | As ruled |

## What I did not check
- The merge and Edit commands' own outputs, which were in the chat only. I checked the committed results instead.
- The preview-site artifact's contents (not to be read). Its listing and digest match the record.
- The later commits under spikes/ (U7's 6968215 and its saves): that is A's read.

## What is owed to him
- His one line naming the six pull requests: C68B-5's record line is in msg-2026-11-27.md:55-57.
- His two questions: TAKEIN-CODE and #75-GO, msg-2026-11-27.md:71-87.

Both are drafted in the day's message. Nothing is owed beyond them.

## Decisions needed
- **TAKEIN-CODE** (as advisor-main-68b.md put it): may a reviewed code pull request merged on main come into this
  branch at step 1 by re-derived numbers, without a new advisor check?
  - Options: (a) no, a short advisor check of the new merges, then step 1 at once; (b) yes, within the named limits.
  - My recommendation: (a). Nothing waits on it.
- **#75-GO:** did #75 (cs4b-1 Content, merged on main at 18:53 UTC) merge with his go?
  - Why it matters: neither its merge message nor main's STATUS row records one, and MAIN-68 took it in with main's
    tip.
  - Options: (a) yes, nothing more; (b) no, the take-in's record gets one line, and what happens on main is his.
  - No recommendation: it is a fact only he holds.

## For Mustafa
1. Answer TAKEIN-CODE, (a) or (b), whenever it suits you.
2. Answer #75-GO, (a) or (b).

Nothing to check in a browser.

Status: COMPLETE 2026-11-27T01:02Z
