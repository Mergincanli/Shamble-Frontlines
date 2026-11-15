# advisor: REFS-KIT head check (914798a)

Seat: advisor checking a pushed head (not a review round). Rules: origin/main:.claude/agents/advisor.md (read).

## Plan
1. Read the ruling (advisor-refs-clash-full.json): fix.check_mjs (3 replacements), fix.check_test_mjs (1), plan.md lines (2).
2. git show --stat 914798a; compare the spikes/ diff to the ruling's replacements word for word.
3. git show 63cc7c3 -- plan.md: the ruling's two lines word for word.
4. Evidence under .gate/refs-kit/: refs-self.log, fixed.tap, P1..P3.tap, plant copies, gate-no-smoke.log, counted.log (diff hash vs git diff 63cc7c3 914798a -- spikes/).
5. Anything that must change before the merge of main.
6. Verdict.

## Notes
- P1 commit: 914798a (parent 63afbbf) touches only spikes/netcode-proof/check.mjs (+15 -2) and check.test.mjs (+11). 63cc7c3..63afbbf touched only .checkpoints/. Script cmp.py (scratchpad refs-kit-head/): 63cc7c3's two files + the ruling's 3+1 replacements (each old found once) == 914798a's blobs byte for byte (157516 / 209527 chars). PASS.
- P1 plan.md: 63cc7c3^'s plan.md + the ruling's 2 plan_md replacements == 63cc7c3's plan.md exactly (whole file, so nothing else changed); 914798a's plan.md equals 63cc7c3's. PASS.
- P2 refs-self.log (20:15): refs PASS, 1268 tracked files, line names §4.9's exception; self 78 of 78; all 2 steps pass. 1268 = tree count of 63cc7c3 and 914798a (kit files not yet in the tree: main not merged; the exception is exercised only after the merge).
- P2 copies: .gate/refs-kit/fixed/{check.mjs,check.test.mjs} == 914798a's blobs; P1/P2/P3 check.test.mjs == 914798a's; each P check.mjs differs from 914798a in line 1862 only, exactly the ruling's plant (P1 startsWith(".claude/scripts/"), P2 continue, P3 line deleted).
- P2 taps: fixed 2/2; P1, P2, P3 each 1 pass 1 fail, the failing test the new one, diffs as the ruling predicts. REPRODUCED myself (node --test --test-name-pattern on the four copies): same results.
- P2 kit files on main 53272a1: kept-logs.mjs 2 'spikes/', kept-logs.test.mjs 6; 0 left in each after removing spikes/netcode-proof/results/ — the exception fits the real files.
- P2 gate-no-smoke.log: GATE PASS (1-4, 5b-5d PASS; 5a skipped by --no-smoke), base 4d21c1a.
- P2 counted.log: lock held 20:34:34, HEAD 63cc7c3, all 11 steps PASS (test 538 + 616 passed, 0 failed; refs 1268 files with the §4.9 line; self 78/78; redact 10/10), exit 0 at 20:48:30. Diff hash 56351922…ab9e REPRODUCED: `git diff 63cc7c3 914798a -- .` in spikes/netcode-proof = 56351922a90c472552253279be0490b95d56bdb6b9aa5ae4bed38ec30504ab9e (main-run.sh line 42 hashes `git diff HEAD -- .` there); untracked hash = empty-input sha256 (no untracked files). So the counted run ran exactly 914798a's spike content.
- P2 CI on 914798a (gh api check-runs): gate success 20:36:23, toolchain success 20:36:31, smoke success 20:36:44. Two check suites (cursor, claude apps) queued with no runs: app noise, not a failure.
- P3 before the merge of main: nothing to change on 914798a. Trial merge (git merge-tree 914798a/ed3d9fd vs origin/main 53272a1): one conflict, docs/STATUS.md, which the ruling's step 2 already assigns to merge_main_53.py (present, tracked). Main touches no spikes/ file. Kit files arrive only with the merge, so the exception's real-tree run is step 2's check.mjs whole.
- Branch head is now ed3d9fd (checkpoint card only, [skip ci]) on top of 914798a: no spikes/ change after 914798a.
- Verdict: APPROVE. No findings above Low. Conditions = the ruling's step 2.
