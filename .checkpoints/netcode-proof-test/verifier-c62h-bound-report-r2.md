# Verifier round 2: C62H-BOUND report draft (working copy vs e2d06ec)

## Plan
- Read only. No Edit tool here: this file is rewritten whole with Write after each check.
- 1. Round 1's W1-W5 and M1: RESOLVED / NOT RESOLVED with evidence (git log b423a7d; card at 67de11e; history.md
  :2113, :2131, :2163, :2177, :2189; c62h-bound-build.md:79-81; .gate/u4r/c62h-bound/merge.log; P6 check run here).
- 2. Every added/changed line of `git diff e2d06ec -- draft build-notes`, claim by claim: P1-P7 vs the ruling's
  conditions; the decision entry (C62HB-Q4 in c62h-bound-design-full.json, brief :589-596; marks.json PM-0001-58 parts;
  section 14 (ii) + its gaps fixture; advisor-c62h-review-full.md:90); For Mustafa item 4 vs the card; P6 check: git
  rev-parse HEAD, git diff --quiet ddd38ba HEAD -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'.
- 3. Outside the hunks unchanged: e2d06ec's draft sha256 9397900…296c77, 538 lines; diff hunks only.
- 4. Drafter's gaps list in order; the last one: u4r-report.md :997, :1008, :1035, :1167, section ends :1199.

## Results (one line per check)
- Setup: e2d06ec draft sha256 9397900330ce…296c77, 538 lines (equal to the brief). Working draft 558 lines, sha256
  5d8fb338…94c3. HEAD = 40dcbb761a789f2a2287dc5f4428d31d2388c806 (two checkpoint commits after e2d06ec: e0fd859,
  40dcbb7; the draft changed in both committed and uncommitted text, all inside `git diff e2d06ec`).
- W1 RESOLVED: `git log -1 --format='%ad | %cd' b423a7d` = 2026-11-16 12:18:58 +0000 both; draft :48 "(12:18)";
  build notes :243 "12:18 (git's time, 12:18:58; first written here as 11:54)". 12:18 truncates like 11:01 (11:01:06).
- W2 RESOLVED: key :11 pins card lines to 67de11e. Card at 67de11e (4129 lines): U3C-HELD-1 :3760-3773, CHECK-NFF-1
  :3774-3775, ten Lows :3776-3779, 17:09-20:36 entry :3198-3211, slip 16 :3209-3211, 12:44-14:22 :3212-3217, slip 17
  :3601-3602, commit 4 / boot :55-56: every draft ref (:242, :248, :454, :506, :510, :511, :519, :521) equals them.
