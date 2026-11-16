# C62H-BOUND build notes (the brief: c62h-bound-brief.md, 663 lines, sha256 7936074c15c5eb49…)

## Plan (the main session, 2026-11-16)
- Route W: one builder launch per commit step, in `/home/user/Shamble-Frontlines/.claude/worktrees/c62h-bound`
  (branch c62h-bound-build), no isolation; the main session commits each step, checks its diff hash and SIZE-CHECK.
- Order: (0) base checks, runner hashes, the worktree, the built check shown to fail, insert part 1 (done, below);
  (1) step 1's launch, commit 1; (2) step 2's launch, commit 2; (3) insert part 3 and the built check, step 3's
  launch (B-2's four fixture files and check.mjs), commit 3, the built check again; (4) step 4's launch, commit 4;
  (5) the review round (test-engineer, netcode-architect, rust-reviewer, xhigh), one fix pass; (6) the merge by
  cherry-pick, the counted main-run.sh call, the gate, the built check, push, CI, the advisor's head check; (7) the P0
  re-judge on 93509dfd7fd1 with the compare script (self-test first); (8) P2, then P3, each with the set check (shown
  to fail first); (9) the report.
- Expected test counts from LANE_BASE's 538 measured / 616 bites: step 1 +7 / +9; step 2 +6 / +6; step 3 +1 / +1;
  step 4 0 / 0; final 552 / 632. A different count stops for the advisor.

## Log (one line per item; times UTC)
- 09:18 LANE_BASE = 044fab188419e8859dbca699d851563c44c078c9 (the branch head at the build's start: 442a284 plus card
  commits, the second pack's list 6c8c7cf and its card save). `git diff --quiet 442a284 044fab1 -- .
  ':(exclude).checkpoints' ':(exclude)spikes/netcode-proof/results'` exit 0; under results/ only
  `A results/u4r/ab10f058a828/P2/raw-logs.sha256` (a kept-logs list).
- 09:18 Runner sha256s equal the brief's: u4r-run.sh e9437ae2…, main-run.sh 8df2dee3…, peak.py dfacdcc0…,
  c-u4r-runs.mjs 99dd070e… (LANE_BASE's). Texts v3 3f21c9b7…, freeze 9cb8be17…, paste cfac7502…, brief 7936074c…;
  marks.json 9fb5be57…, SERIES.md 3411eee7…, README.md b49f3441… (LANE_BASE's).
- 09:19 Worktree made by hand: `git worktree add -b c62h-bound-build .claude/worktrees/c62h-bound 044fab1`.
- 10:2x The main session's scripts written: c62h_bound_insert.py and c62h_bound_built_check.py (notes folder).
- 10:2x The built check shown to fail on the worktree as LANE_BASE left it
  (.gate/u4r/c62h-bound/built-check-negative.log): exit 1; texts v3, freeze, paste and all 8 block hashes PASS (12);
  both marks.json blocks, the four rows, both header lines, the sentence, the three file hashes and the word diff FAIL
  (13), as the brief requires.
- 10:2x Insert part 1 on the worktree (.gate/u4r/c62h-bound/insert-part1.log): exit 0; SERIES.md 3411eee7… →
  89295b57d5c9ceec… (the brief's expected); numstat +2 -1 (unstaged, for step 1's launch to stage with its files).
- 10:2x Part 3 dry run on a scratch copy only (.gate/u4r/c62h-bound/dry-p3/, log insert-part3-dry.log; not the
  worktree): all three expected hashes reproduced (marks.json c1935a5b…, SERIES.md 6898e119…, README.md f3c3c804…;
  json.loads check equal; README refilled into 16 lines, the longest 120). The real part 3 runs before step 3's launch.
