# Advisor: MAIN-60 pushed-head check (head bb7a5ed; the take-in 0f12d9e)
2026-11-18 23:54 to 2026-11-19 00:05 UTC. Read only. Acting by main's advisor.md (`git show
origin/main:.claude/agents/advisor.md`, 335 lines, b763ba3). Checked against the plan's "As approved" text
(main-60-plan.md :3-12) and my conditions C1-C6 (advisor-main-60.md :14-70), as at MAIN-54's head check of 20048a5
(history.md:2552-2564; advisor-main-54-head.md).

## Verdict: APPROVE WITH FINDINGS (head checked: bb7a5ed336a0731d37ea96e1d47da2f31cba9124)
The take-in is what the plan said it would be, and C1-C6 are met. Merge commit
0f12d9ec75f192d118a79257edfeea2ca2ea4534 has parents 64a0e28 (H0) and b763ba3. Outside spikes/, .checkpoints/ and
docs/, its tree equals main's. spikes/ and .checkpoints/ equal H0's. In docs/ it differs from main only in the branch's
six docs, and each of them carries exactly the branch's own change. All 81 marks' rows hash to marks.json's values. CI
is green on bb7a5ed: gate and smoke on both events, and toolchain on both. The two commits after it (3ef41a8, aedafe3)
change only the card. The findings are six Lows, all on the record and the card, with no push of their own (Review
budget). **The lane may go on from this head.** U4f's snapshot 74ae26aa passes `git apply --check` on it (28 spike
files and 1 docs/design file). U4f's panel still waits for Mustafa's SIZE-U4F answer (ruling R4).

## Conditions, one by one
- **C1 met.** H0 = 64a0e28. HEAD's reflog: 286d9d5 at 23:39:06 (the saver's "autosave (terminated)"), 64a0e28 at
  23:39:49, then "commit (merge)" 0f12d9e at 23:41:27. The remote's reflog: 20b3b33, then 64a0e28 at 23:39:51, so HEAD
  equalled the remote branch before the merge. `git cat-file -p 0f12d9e`: parents 64a0e28ac7b7… and b763ba3e5f34….
  `git diff --quiet 64a0e28 0f12d9e -- spikes/` exits 0. The header of light-base.log reads "HEAD 64a0e28".
- **C2 met in substance.** (a) The resume line is on the card in 286d9d5 (card :171-176), word for word as C2 (a)
  wrote it, so it was in place before TaskStop's save. (b) 286d9d5 is the SIGTERM save ("(terminated)"). Git shows no
  save of any kind between 23:39:49 and bb7a5ed at 23:43:27. One `checkpoint.mjs watch` process runs now (PID 19333,
  started 23:43:44); the old one is gone. The record does not say which proof it used: ADV-M60H-4. (c) 286d9d5 was
  never pushed by itself (the remote went from 20b3b33 straight to 64a0e28). The foreground save 64a0e28 pushed it, and
  the pre-checks came after that: the base run started at 23:40:14.
- **C3 met.** The guard's full pathspec on `origin/main...64a0e28`, `origin/main...bb7a5ed`, `origin/main bb7a5ed` and
  `origin/main aedafe3` prints nothing every time. `git diff --raw --no-renames origin/main...bb7a5ed`: 1,291 files are
  100644 and 2 are 100755 (`.checkpoints/…/u4d-fix1-run.sh` and `spikes/…/u3c-kernel-check/run.sh`, both older than
  this change). No symlink, no submodule.
- **C4 met.** The merge's output and `git status` are in the chat only, but git shows the result. H0..0f12d9e has
  24 A and 25 M (main's 24 M plus the resolved STATUS.md). The commit message is git's own, with "# Conflicts:" and
  "#\tdocs/STATUS.md", the cosmetic lines C4 expected. The three whole-tree checks, which I re-ran:
  (1) `git diff --quiet origin/main 0f12d9e -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'`
  exits 0; (2) the docs/ names against main are exactly the six (DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE,
  design/netcode-proof-messages, design/ws-reconnect); (3) `git diff --name-only 64a0e28 0f12d9e` equals
  `git diff --name-only 82c605e b763ba3` (49 names), and both shortstats read +6,835 / -137. Both light runs, read line
  by line, meet the bar. light-base.log (HEAD 64a0e28, 23:40:14-23:40:26, exit 0): refs 1,474, marks 81 with "rows in
  this branch's docs/adr/: all unchanged", self 78 of 78, redact 10 of 10 and 307 items with 0 findings, "check: all 4
  steps pass". light.log (HEAD 0f12d9e, 23:42:18-23:42:21, exit 0): refs 1,498 (N0 + 24) with the exception line, the
  same marks line, self 78 of 78, redact 10 of 10 and 307 items (M0) with 0 findings, "all 4 steps pass". In both, diff
  and untracked are e3b0c442…b855 and free space is above 4,096 MiB (5,076 and 5,043). Tracked files from `git ls-tree`:
  1,474 at 64a0e28 and 1,498 at 0f12d9e; spikes/netcode-proof has 307 at each.
- **C5 met.** (a) bb7a5ed reached the remote at 23:43:29 (remote reflog "update by push"), so no "NOT pushed" stop.
  (b) HEAD's reflog goes 64a0e28, 0f12d9e, bb7a5ed. No save fell inside the merge.
- **C6 met.** The plan has its "As approved" header (:3-12); the facts' HEAD line reads "4ec4a9e at the plan, 20b3b33
  at the review", with times (:22-25); F15 (:26-27); DEP-HOLD's reach in the Risks (:88-90). The card has DEP-HOLD on
  Rules in force (:49-51) and the Base line (:5-7: "main b763ba3 merged in at 0f12d9e (MAIN-60, 2026-11-18 23:41 UTC
  …)").
- **Step 6 met.** bb7a5ed has no [skip ci], so CI ran. Its message names MAIN-52 and the take-in and carries both
  attribution lines. The saver was restarted with both `--trailer` lines (the running command line shows them) after
  the push (23:43:44) and before this check. That closes ADV-M60-8: 286d9d5 is the last one-trailer commit.

## The merge's tree (beyond C4)
- docs/STATUS.md: `git diff origin/main 0f12d9e` changes one line, the #40 row, which becomes the branch's
  2026-11-15 text. The changed lines equal the branch's own patch (82c605e..64a0e28). The #52 row is out and main's
  #60 row is word for word. The file has 120 lines (STATUS-DIET). No conflict marker in any docs/ file (`git grep` on
  0f12d9e), and `git diff --check` is clean.
- DECISIONS.md, STATUS_ARCHIVE.md and both design docs: the patch-id against main equals the branch's own patch-id
  against the base (identical text). REPO_MAP.md: the same +/- lines (17 added) at offsets moved by main's new rows;
  read at the three insertion points, each sits in its right table. So the merge alters no word of Mustafa's in
  DECISIONS.md: main's text plus the branch's own lines.
- The 81 marks' rows: a read-only script hashed each mark's row at 0f12d9e, 64a0e28 and aedafe3 (`git show`, no
  checkout; scratchpad `marks_rows.mjs`, the same rule as check.mjs:1869-1909, expectedMarks and marksProblems). All 81 match marks.json on each
  commit (63 in ADR 0001, 1 in 0004, 17 in 0006), none missing and none extra. marks/ and npt-judge equal H0's.
- The branch against main after the merge (`origin/main...bb7a5ed`): 980 .checkpoints/, 6 docs/, 307 spikes/,
  nothing else. merge-base(aedafe3, origin/main) = b763ba3, and `git ls-remote` shows main still at b763ba3, so the
  branch is current with main.

## Reproduced myself (main folder at aedafe3; legacy/ and the kit equal bb7a5ed's and main's)
- `node tools/gate.mjs --no-smoke`: GATE PASS, base b763ba3, symbols +0 -0, every line equal to step5-outputs.txt's.
  Smoke comes from CI.
- `node .claude/scripts/repo-map-check.mjs`: "repo map OK: 149 entries cover 110 required paths".
- `node --test .claude/scripts/checkpoint.test.mjs`: 33 tests, 33 pass, 0 fail (main's count).
- CI, read through `gh api`. On bb7a5ed: gate 44423167627 (push) and 44423174642 (pull_request), each with jobs
  `gate` and `smoke` (the step "Smoke (check 5a)") at success; toolchain 44423167841 and 44423174707 at success; all
  attempt 1, finished 23:44:16-23:51:56 UTC. The commit's check-runs: 6, all completed and successful. The toolchain
  runs took about 41 s, as main's own toolchain runs on b763ba3 did (about 37-44 s). On 3ef41a8 and aedafe3: 0 check
  runs ([skip ci]).
- PR #40: head aedafe3, draft, open, mergeable true, mergeable_state "blocked", as the record says.
- `git apply --check` of U4f's snapshot (wip/u4f-build.patch, sha256 74ae26aa…) on the working tree: exit 0.

## Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-M60H-1 | Low | The card's Running work (:183-186) still says "Restart it after step 6 with both attribution lines…", but the saver was restarted at 23:43:44 (bpcoh9gqm, both trailers). | Next card save: cut that sentence (the card stays within 200 lines) |
| ADV-M60H-2 | Low | The card's Open findings (:138-139), "At the next main take-in: … keep this branch's #40 row and main's #55 row", is out of date. MAIN-60 resolved it with main's #60 row, and the branch's STATUS.md now differs from main only in the #40 row. | Next card save: "keep this branch's #40 row; every other row as main has it (MAIN-60's resolution)" |
| ADV-M60H-3 | Low | The "Last update" line (:8-9) still reads 23:43, "CI and step 7 next", after aedafe3 recorded CI green and 3ef41a8 the hand runs (as ADV-M54H-1). | Next card save |
| ADV-M60H-4 | Low | The BUILT block doesn't say how C2 (b) was confirmed: the saver's "watch stopped: terminated" line, or the reflog. Git proves it (286d9d5 "(terminated)", no later save before the merge, no old process left), so the condition is met. | One clause in the next save (ADV-M54H-2's rule) |
| ADV-M60H-5 | Low | bb7a5ed's message is a shorter MAIN-52 record than 20048a5's. It doesn't cite his words (history.md:2055-2057) or say why 0f12d9e keeps git's own message without attribution lines. The plan and the card say both. | None: a commit is never rewritten (Never), and #40 squash-merges (its closing checklist) |
| ADV-M60H-6 | Low | CI's green belongs to bb7a5ed. The later heads (3ef41a8, aedafe3, and saves to come) are [skip ci] with 0 check runs (as ADV-M54H-3). | The CI record names bb7a5ed; PR #40's ready head runs its own CI (CHECKPOINTS.md §9) |
| ADV-M60-7 | Low (open) | STATUS's #40 row still says "main 53272a1 merged in" (the branch's row, kept word for word, which is right for a merge). | PR #40's closing docs pass, as ruled |
| ADV-M54-8 | Low (open) | CLAUDE.md's Never line does not list a plain `git merge origin/main`, which the guard allows. | The kit's follow-up after the freeze, as ruled |

## What I did not check
- The chat-only steps: the pre-checks' own outputs, the merge's printed lines and exit code, the 49 `git status`
  lines, the Edit of STATUS.md, `subscribe_pr_activity` for #40, and whether any call asked. Git's result matches
  each of them (above).
- How TaskStop signalled the old saver. I checked what followed instead (C2 (b)).
- Smoke locally: it comes from CI's runs on bb7a5ed. check.mjs whole, or any cargo run: the spike tree is unchanged
  since H0, so none was needed. That U4f's snapshot builds on this head: only `git apply --check`, as the plan's
  "After it" lists.
- The four R3 (iii) hand runs (3ef41a8's record). They ran on builder 3's worktree, are outside this check, and do
  not touch the head.
- Commit signatures beyond their presence (0f12d9e carries an SSH signature; I did not verify the key).
- Process notes. The gate, repo-map-check and checkpoint tests wrote only temporary folders that they remove; `git
  status` stayed clean. Once, by mistake, I sent a list of git names into a scratchpad file by shell redirect (outside
  the repository; nothing used it). Every other file here, this one included, was written with the Write tool.

## Decisions needed
None. MAIN-60 ran under his standing MAIN-52 rule. The one conflict was resolved by the plan's mechanical rule. No
pass mark, judge, marks.json, money or main is touched. SIZE-U4F is already with him in the day's message: it is his
call, not this check's.

## For Mustafa
Nothing.
