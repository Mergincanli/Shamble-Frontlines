# Advisor: MAIN-67 pushed-head check (step 7; pushed e23c783, merge 07cc9b7)
2026-11-26, 04:5x UTC window. Advisor agent, day session, effort xhigh. Acting by main's advisor.md (read whole in this
run). The form: advisor-main-66.md Q1 (d), with M67-3 (advisor-main-67.md). HEAD at start: 92ccad1 (= origin); main
07d673b (unmoved).

## Plan of this check
1. SHAs and parents; the first-parent line H0..pushed; later commits by rev-list and diff --name-only.
2. Section 3's numbers by SHA: 8 paths, the lines, the anchor, STATUS.md, the whole tree, the kit, raw modes, trees.
3. The records: step5-outputs.txt, light-base.log, light.log; the reflog (no save in the window).
4. CI by run id; toolchain lines against main's job 139502338223.
5. Re-runs in the main folder (gate, repo map, checkpoint tests, guard tests) if the precondition holds.
6. Verdict, DONE. Then U8's ruling (its own file).

## 1. Verdict: APPROVE. The lane may go on from this head.
Heads checked: H0 b518460e885d…, merge 07cc9b732b55…, pushed e23c78323b7d…; the branch at 92ccad1 at the end.
- **The commits.** The merge 07cc9b7 has parents H0 and 07d673b (git's own message, "# Conflicts: docs/STATUS.md", no
  attribution lines, signed). e23c783 is the card and the plan only (+5/-2), with both trailers.
- **Main's change, name for name.** H0..merge is main's 8 paths (A 1, M 7): "8 files changed, 555 insertions(+), 29
  deletions(-)"; without STATUS.md, "7 files changed, 543 insertions(+), 17 deletions(-)".
- **The anchor (M67-2).** T 151318fe… is recorded in the plan at e23c783, and `git diff --quiet T 07cc9b7 -- .
  ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'` exits 0.
- **STATUS.md.** It has 120 lines. Against main, one line out (main's #40 row, the 2026-11-14 text) and one in (the
  branch's #40 row).
- **The tree.** The pushed tree equals main's outside spikes/, .checkpoints/ and docs/. docs/ differs in the usual six
  names. spikes/ and docs/design/ equal H0's. The kit pathspec prints nothing: toolchain.yml is main's, byte for byte.
  The raw listing shows six 100644 lines: no link, no submodule, no marker.
- **Trees.** 1,930 files, then 1,931 (refs N0 + 1).
- **No save in the window (M65-1).** The remote-tracking reflog goes b518460 (00:04:22), then e23c783 (00:27:07). 07cc9b7
  was never a pushed tip.
- **The resume line (M67-1)** is on the card at H0, word for word. The card has 199 lines at e23c783.
- **The records.** light-base.log (HEAD b518460) and light.log (HEAD 07cc9b7): refs 1930, then 1931. Marks 81, rows
  unchanged; self 78/78; redact 11/11 with 378 items and 0 findings; exit 0 both. They are equal but for HEAD, refs,
  times and free disk. step5-outputs.txt records every step 1-5 bar.
- **CI on e23c783: 6 of 6 jobs green,** attempt 1, 00:27:11-00:32:55 UTC:
  - push: gate run 45728829441 (gate 139605430605, smoke 139605430875) and toolchain 45728829428 (139605431147);
  - pull request: gate 45728836685 (gate 139605456334, smoke 139605455757) and toolchain 45728836660 (139605455428).
- **Toolchain against M67-3's baseline** (main's job 139502338223; all three logs read whole, as data). They hold the
  same lines in the same order: 63 `test result: ok.` lines, all 0 failed / 0 ignored / 0 filtered; 5 vitest
  `Test Files` lines (10, 1, 7, 2, 2); Playwright's "1 passed", "16 passed" and Firefox's "2 passed". There is no
  `##[error]` and no FAILED.
- **My re-runs in the main folder.** The precondition `git diff --quiet 07cc9b7 HEAD -- . ':(exclude)spikes/'
  ':(exclude).checkpoints/' ':(exclude)docs/design/'` exits 0, and the working tree was clean.
  - GATE PASS (base 07d673b, symbols +0 -0, 5a skipped);
  - "repo map OK: 207 entries cover 110 required paths";
  - checkpoint tests 33/33;
  - guard tests 1,247/1,247, 0 fail (35.5 s).
  - Each equals step5-outputs.txt.
- **Later commits.** e23c783..92ccad1 is 6 commits in a straight line, one parent each, all under .checkpoints/: the
  card, u8-merge-plan.md, u8-report.md, wip/u7-build-rs-notes-0030.md and wip/u7-build-rs.patch. U8's stopped merge
  committed nothing: spikes/ equals H0's.
- **Findings.** Nothing new. Still open (Low): ADV-M67-2 (main's own ci-after-3a row still reads "none yet"),
  ADV-M60-7 and ADV-M54-8.
- **Not checked.** The merge's own output and the step-3 Edits were recorded only in the chat; the committed results
  match. Main is still 07d673b (ls-remote), so no take-in is due.

## Decisions needed
None.

## For Mustafa
Nothing.

## Notes (one line after each check)
- SHAs: e23c783 parent 07cc9b7; 07cc9b7 parents b518460 and 07d673b. The first-parent line H0..e23c783 holds those
  two. Later: 47756a3, 0d66dc5, 699eff2, 6e22e28, 37badaf, 92ccad1, notes only (`git diff --name-status e23c783 HEAD`:
  five .checkpoints/ paths).
- By SHA: name-status = main's 8; the lines as above; the anchor and the whole-tree check exit 0; docs/ six names;
  spikes/ and docs/design/ = H0's; kit empty; raw six lines, 100644; no markers; STATUS.md 120 lines, "1 1".
- Records read: step5-outputs.txt (24 lines), light-base.log, light.log. The reflog and the resume line as section 1
  says. Card 199 lines at e23c783.
- CI: six jobs by run id, all success. Logs: main 139502338223 and #40's 139605431147 and 139605455428, 63 + 5 + 3
  summary lines equal.
- Re-runs 04:5x-05:0x UTC: gate, repo map 207/110, checkpoint 33/33, guard 1247/1247. git status clean before and
  after.

DONE
