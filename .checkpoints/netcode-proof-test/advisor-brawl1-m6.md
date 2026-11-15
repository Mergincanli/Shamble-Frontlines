# Advisor: BRAWL-1 merge, M6 (review of pushed head 8f0939f; merge commit b30314a)

Started 2026-11-15. Agent file: origin/main:.claude/agents/advisor.md (read).

## Plan
1. Read the merge step of the round-1 ruling (advisor-brawl1-r1-full.md) and the round-2 amendments A1-A6
   (advisor-brawl1-r2-full.md).
2. Check the commit range: b30314a's parent, b30314a's file list, every later commit .checkpoints/-only.
3. M1: spikes/ at b30314a^ equals 352a245's; the patch file at 888ecbb hash c2561b6c...; b30314a's spikes/ diff
   equals the patch (hash of `git diff b30314a^ b30314a --binary -- spikes/`); the 10 files equal the builder
   worktree; the patch removed in b30314a.
4. M2: m2-clean-B/C logs: run lines vs the 2026-11-14 logs; df figures.
5. M3: m3.log header (HEAD, diff/untracked hashes) and every A3 figure it should show.
6. M4: the two gate logs; whether --no-smoke is admissible under the rulings (Chromium missing).
7. M5: push time, CI runs on b30314a (GitHub tools if available; gh CLI otherwise).
8. Since: the card and u4r-report.md's BRAWL-1 section vs my round-2 words.
9. Rule: verdict, findings, M7, for Mustafa.

## Checks (a line each as I go)
- C1 range: HEAD = origin branch = 8f0939fb. b30314a's parent f7dff4bf. Later commits 5b00f44, 8043264, 8f0939f touch only
  .checkpoints/ (git diff --name-only per commit, nothing else). b30314a: the 10 spikes files M + the patch file D.
  b30314a's message has no [skip ci]. spikes/netcode-proof tree ebbfeb40 at b30314a and at 8f0939f. PASS.
- C2 M1 base: spikes tree 63de1632 at b30314a^ and at 352a245 (netcode-proof abad0635 both). PASS.
- C3 M1 bytes: patch at 888ecbb and at b30314a^ both sha256 c2561b6c...; `git diff b30314a^ b30314a [--binary] -- spikes/`
  sha256 c2561b6c (identical to the reviewed patch). 10 files: sha256 equal at b30314a, in the builder worktree
  (HEAD 352a245, 10 M, git diff HEAD c2561b6c) and in the main folder. numstat 1139/127 = 1,266. PASS.
