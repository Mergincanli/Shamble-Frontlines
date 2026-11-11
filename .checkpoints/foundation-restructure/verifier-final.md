Task: verifier, foundation step 2.1a, PR #50, pinned 0590d8c05227146ca255822269a8df5b997eac45, base main bb2d817.

## Plan (blocks, in order)
- B1 F1 write (3f29cb8 stat, blob, patch reproduces blob, smoke.mjs blob, kit diff empty)
- B2 full E3 compare (f1 vs recorded) + sample re-run
- B3 base cases and decoy
- B4 rows red/green + green re-run
- B5 gate on pinned head with smoke
- B6 his words (decisions check + appendix A vs sources)
- B7 docs facts
- B8 CI facts / blob equality
- B9 open findings handling
- B10 report

## Notes
### B1 F1 write: DONE, PASS
- `git show 3f29cb8 --stat`: only tools/gate.mjs (+9 -1). Blob 3f29cb8:tools/gate.mjs = 7a8480ce82..., parent 2bfab53.
- GNU patch 2.7.6 `patch -o` of f1-gate.patch onto 2bfab53 copy (.gate/foundation-restructure/f1check/) -> hash 7a8480ce8225f320de7627575321c052b8f76bd3 (match).
- 0590d8c:tools/smoke.mjs = 18058e49..., 0590d8c:tools/gate.mjs = 7a8480c.
- `git diff bb2d817...0590d8c -- .claude CLAUDE.md .github`: empty.
- 0590d8c..HEAD(9a937b0) touches only .checkpoints/.
- next: B2
### B2 (in progress) E3 compare part: recorded vs f1, own script verifier-e3-compare.py: 31/31 ids, 0 diffs (verdict, exit, failing ids, basePrefix, per-check PASS/FAIL word, all 3 columns). 30 PASS + PC1 "PASS (judged on check 3)". Sample runner diff vs e3-run.mjs: only header comment, SAMPLE filter, OUT path. Worktrees clean before: fdn-old 44446a5, fdn-new 3f29cb8 (gate 7a8480c), fdn-newleg 7c22abf (parent 3f29cb8, 5 pure renames 100%). Sample run launched (log .gate/foundation-restructure/e3-sample/run.log).
### B8 CI facts / blobs: DONE, PASS
- tools tree f23190c and five game blobs (e528dca, 24e86d5, fce6f37, 008c66f, 5ba72df) identical at 3f29cb8, 0607b45, 76544aa, 0590d8c; game blobs equal main bb2d817.
- bb2d817..0590d8c outside docs/.checkpoints: only tools/{README.md,gate.mjs,smoke.mjs,village.py}.
- checkpoint steps 13-20 consistent: 13/16 3f29cb8 gate green, smoke failed at runner DNS/library step (smoke step never ran); 17 8d8c422 smoke failed (mirror); 19 0607b45 green (push 43063326391, pr 43063335859); 20 76544aa green (pr 43067074308, push 43067068538). Cannot read GitHub: not independently verified.
- next: B6, B7, B9 while E3 sample runs; then B2 finish, B3, B4, B5.
