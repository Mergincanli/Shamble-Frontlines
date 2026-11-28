# Advisor: head check of 717b802 (the merged head), notes

Seat: advisor, checking a pushed head (not a review round). Read only. Rules: origin/main:.claude/agents/advisor.md (read).

## Plan
1. Merge 2bd44a5: name-status vs main's 4d21c1a..53272a1; STATUS.md resolution per merge_main_53.py (one #40 row, state
   cell updated; main's #52 row; #48 dropped; 120 lines). Commits 2bd44a5..717b802 touch only .checkpoints/.
2. Evidence under .gate/refs-kit/: gate-merge.log, counted-merge.log, kl-cache.log, counted-merge-2.log (pass bar);
   refs count 1,275 vs bar 1,273 (two disk-ruling notes?); CI on 6db2d0c; u3c-review-target gone, runs/ unchanged.
3. Readiness for the first real list and pack: folder, closed ruling (file:line), pre-checks.

## Notes (one line per point)
- 1a: 2bd44a5 parents 98ffb08, 53272a1; merge-base 4d21c1a. name-status 98ffb08..2bd44a5 == 4d21c1a..53272a1 (same 12
  files, same status letters; --stat 12 files, +1327 -108 both).
- 1b: 8 of 12 blob-equal to 53272a1; DECISIONS, REPO_MAP, STATUS_ARCHIVE differ from main's blob because the branch had
  its own edits; `git merge-tree --write-tree 98ffb08 53272a1` (tree 5456a4a) equals 2bd44a5 on all three (git's own
  auto-merge, no hand edit). Only conflict: docs/STATUS.md. diff tree 5456a4a vs 2bd44a5 = docs/STATUS.md only.
- 1c: STATUS.md: only lines 28-34 (the conflict) changed vs git's conflicted file; rows #40, #52 (branch had #40, #48);
  #52 byte-equal to main's; #40 differs from main's in cell 3 (state) only; 120 lines, trailing newline. Matches
  merge_main_53.py (committed e1e97e9). Cited 71ee13d, b30314a exist and are ancestors of 98ffb08.
- 1d: 6db2d0c, ea95701, 717b802 touch only .checkpoints/ (card; 6db2d0c also the two disk-ruling notes). PASS.
- 2a: gate-merge.log: GATE PASS, 5a skipped (--no-smoke), symbols +0 -0 vs 53272a1. CI's gate+smoke cover 5a (2c).
- 2b: counted-merge.log: HEAD 2bd44a5, free 2866 MiB, empty hashes, "STOP: under 4 GB free" (20:57:10). As reported.
- 2c: kl-cache.log: free 2863 -> 6914, removed .gate/u3c-review-target, exit 0. Folder gone (ls). kept-logs.mjs,
  test and CHECKPOINTS.md on disk equal origin/main 53272a1 (git diff empty); cache mode removes only CACHE (header).
- 2d: counted-merge-2.log: lock 21:02:53, free 6913 MiB (>= 4096), HEAD ea95701 (= 2bd44a5 + .checkpoints-only
  commits), diff and untracked = e3b0c442... (sha256 of empty input), all 11 PASS (test 538 + 616, 0 failed), refs
  "1275 tracked files ... §4.9's one exception", self 78/78, redact 10/10 + 0 findings, exit 0 at 21:16:35.
- 2e: tracked count: 914798a 1268, 98ffb08 1270, 2bd44a5 1273, 6db2d0c 1275 (+ advisor-disk-deadlock.md and -full.json),
  ea95701 1275. My bar's 1,273 was the merge commit; the 2 are my notes. Both kit files in ls-tree ea95701. Explained.
- 2f: CI 6db2d0c: gate, toolchain, smoke success x2 (21:02-21:03 UTC). ea95701 and 717b802 [skip ci], 0 runs; their
  tree differs from 6db2d0c in .checkpoints/netcode-proof-test.md only, so 6db2d0c's green stands for the head.
- 2g: runs/: 10 folders, nothing newer than 20:50 UTC (find -newermt), sizes = draft table. Nothing running (ps).
  Free now 5,889 MiB (card: 5,891 after the run).
- 3a: first folder: u4r-precheck-30-faf42aeb034c (17 MiB, 2 files: SS-P0-s4.rec, marks.json); results
  spikes/netcode-proof/results/u4r-precheck-30/faf42aeb034c (runs.txt, d666aeb). Fingerprint now 3ae12298...c1ce3 =
  KL PR dry-run evidence (kl-evidence/evidence-1.txt:19). Dry run there: 2 of 2 read back, archive 3.4 MB.
- 3b: closed ruling: advisor-u4r-sets-r2-full.md:45 ("U4r's judged runs close once the report and the card carry this
  record. There is no further run at any U4r set commit.") with :63 ("The -30 pre-check as recorded."); closed on the
  card and report at 6b22cab.
- 3c: P0: two folders (051995db9cea 3,249 MiB; 93509dfd7fd1 3,393 MiB). His words (history.md:2163ff) say "the kept P0
  set" (singular): keep BOTH unpacked until the re-judge has run (or its brief names one). Low.
- 3d: pre-checks: free >= folder + 1024 MiB (script refuses otherwise; 17 + 1024 << 5,889); pgrep empty; no build or
  run set; saver state; list file is pure hex + sizes + short paths (redact skips pure hex).
## Verdict: APPROVE; first pack ready.
