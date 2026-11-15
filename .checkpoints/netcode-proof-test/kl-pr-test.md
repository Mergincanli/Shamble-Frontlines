# kl-pr-test: test-engineer review of PR #53 (KEPT-LOGS), head a637c7a

## Plan
1. Read section 11, kept-logs.mjs, kept-logs.test.mjs, the diff.
2. Re-run the suite (TAP); record counts.
3. Re-check M4, M12, M13, M14 (+ others if cheap): diff vs head (one replacement), run suite with KEPT_LOGS_SCRIPT, compare evidence-1.log and build_plan_changes 2.
4. Q3a: test 5's --import preload. Q3b: KLC-F2 untested branches.
5. Evidence items (history.md:2139, 2159): GNU tar not skipped, dry-run fingerprints, test 5 line; fingerprint soundness.
6. Coverage gaps on refusal/failure paths.
7. Verdict + findings.
(No Edit tool in this run: notes rewritten whole with Write.)

## Blocks
### B1 suite (done)
Worktree at a637c7a. node v22.23.4, GNU tar 1.35. TAP: 10 tests, pass 9, fail 0, skipped 1 (test 10 dry run, SKIP);
ok 2 (GNU tar) ran with no SKIP; ok 5 (second walk). ~4.1 s.
### B2 planted bugs (done)
All 14 copies diff against head by exactly one replacement (M12 = the 3-line second-walk block deleted; M2/M10 = one
inserted line). Re-ran all 14: each fails exactly the tests recorded in kl-evidence/evidence-1.log lines 2-15
(M4 -> t4; M12, M13, M14 -> t5; M10 -> t4,t5; M2 -> t1,t2,t4,t5; M5 -> t7,t8). Same as the record and as the homes in
advisor-kl-change-full.md:94-101.
