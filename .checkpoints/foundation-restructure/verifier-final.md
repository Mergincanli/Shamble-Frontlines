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
### B6 his words: DONE, PASS
- `python3 -I .checkpoints/foundation-restructure/docs-keeper-check-decisions.py`: RESULT PASS, exit 0, 12 appendix blocks (A1.1-A4), 13 source blocks each inserted once in DECISIONS, 0 old lines deleted (492/492 unchanged), 3 pointer lines.
- own script verifier-appendix-check.py (independent fence parser): 12 report blocks == 12 source blocks byte for byte (A1.1-A1.3 mustafa-2026-11-10.md; A2.1-A2.3 -answers; A3.1-A3.2 mustafa-2026-11-11.md; A3.3 -f1; A3.4 -prompts; A3.5 -prompts-correction; A4 = bb2d817 message's 5 "> " lines, prefix removed). RESULT PASS.
- notes sources unchanged 0590d8c..HEAD (only foundation-restructure.md and ie6-probe.txt changed).
- next: B7
### B7 docs facts: DONE, PASS (one Low)
- STATUS.md 116 lines (<=120). #49 facts match git: bb2d817 parents 2d22b69 + 2e6e5d2, commit dated 2026-11-11T09:03:10Z (STATUS/archive give merged_at 09:03:11 and say the commit is dated 09:03:10: consistent with checkpoint step 9's read-back); go ~09:02 UTC, session_0173vVo6Vtm2C54toWvTt8sR, PR comment 6779064873, run 42991918886: all equal bb2d817's message / archive row (STATUS_ARCHIVE.md:159).
- STATUS "From foundation 2.1a (PR #50)" bullet (STATUS.md:93) ids = archive group (STATUS_ARCHIVE.md:506-548): 2.1b IE-2, IE-3, IE-4, IE-7, SA-L1; 2.3 IE-1, IE-5 (Medium), TE-2, TE-7, R2-1, A-1, N2, L-4, v2 root page; drafts: five lines; docs passes: stale citations. Cited lines checked: tools/gate.mjs:247 (move-check regex), tools/smoke.mjs:31 ("keep one copy"), ADR 0002 :38 :724 :1115, SPIKE_PLAN :100 :202 all hold "before the foundation".
- ROADMAP 2.1a entry (ROADMAP.md:106-118) matches the diff (smoke findBuild, gate base prefixes + F1 move check, village.py legacy default, README gate-globals line); "Next:" clause deliberately unfinished.
- Pointers: DECISIONS :38, :349 (after ">" blocks, outside them), :567 (italic continuation of list item 3, after the closing quote mark of a quoted sentence; not a ">" block, his words unchanged); STATUS :40; SPIKE_PLAN x3 and ADR 0002 x1 (outside any ">" block). The quoted pointer text "Steps 2.3 to 2.6 keep their order and wait for their results." occurs in mustafa-2026-11-10.md and DECISIONS.
- `node .claude/scripts/repo-map-check.mjs`: repo map OK, 116 entries cover 103 required paths, exit 0.
- FINDING V-1 (Low): STATUS.md "Open risks" CI line says smoke "failed twice on #50's 3f29cb8 (a runner DNS outage)"; per checkpoint steps 13-17 and infra-engineer-round2 "Precision": on 3f29cb8 the push run failed both attempts, the pull_request run failed once and its attempt 2 was cancelled at the job timeout; 8d8c422's pull_request run failed too (mirror). Understated; archive IE-5 names 3f29cb8 and 8d8c422 correctly. Fix with the last commit or leave.
- next: B9
- V-1 revised: infra-engineer-round2.md (R2-2 block) judged the same STATUS line "accurate" (two runs, push and pull_request, failed on 3f29cb8). Downgraded to a note, no severity: the line omits 8d8c422's failure and that 3f29cb8's PR attempt 2 was cancelled (timeout); archive IE-5 has both.
### B9 open findings handling: DONE, PASS (TE-1 re-proved in B3)
- Medium and above across rounds: F1 High (security-auditor r1, advisor r1) -> fixed 3f29cb8 (B1 verifies the write), closed by security-auditor round 2 APPROVE ("F1 (High) CLOSED"). TE-1 Medium (revert-case base evidence not reproducible) -> raw evidence-f1/base-revert-*.{json,exit,stderr}; test-engineer round 2: "TE-1 (Medium) closed"; re-run by me in B3. IE-5 Medium (CI live mirrors; infra-engineer r2 as "R2-3") -> deferred in writing: STATUS_ARCHIVE.md "From foundation 2.1a" -> "For the gate step (2.3)" and STATUS.md:93. No other Medium+ in round 1 notes (infra-engineer r1: none; advisor r1: F1 + TE-1 only) or round 2 (test-engineer: "No Critical, High or Medium open").
- Lows: deferred in archive: IE-1..IE-4, IE-7, SA-L1, TE-2, TE-7, R2-1, A-1, N2, L-4, v2 root page. IE-6 settled not a finding (evidence-f1/ie6-probe.txt: exit 0, no stderr, header "base 89286ed (files in legacy/) | head = working tree (files at the root)"). Not in the archive and left to the report (allowed for Lows): TE-4, TE-5 ("say it in the report"), TE-8 (comment in a checkpoint script), L-1, L-3, SA-L2/L-2 (fixed per SA r2). Report body is deliberately unfinished: these must appear there.
- next: wait for the E3 sample, then B2 finish, B3, B4, B5.
