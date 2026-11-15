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
ok 2 (GNU tar) ran with no SKIP; ok 5 (second walk). ~4.1 s. Load check: 4 suites at once, all 9 pass / 1 skip
(5.9-6.0 s each), 0 kept-logs-test-* folders left in /tmp.
### B2 planted bugs (done)
All 14 copies diff against head by exactly one replacement (M12 = the 3-line second-walk block deleted; M2/M10 = one
inserted line). Re-ran all 14: each fails exactly the tests recorded in kl-evidence/evidence-1.log lines 2-15
(M4 -> t4; M12, M13, M14 -> t5; M10 -> t4,t5; M2 -> t1,t2,t4,t5; M5 -> t7,t8). Same as the record and as the homes in
advisor-kl-change-full.md:94-101.
### B3 Q3a preload (done)
Sound. Hook only in test 5's spawnSync argv (test.mjs:177), file in fixture tmp root, removed by t.after; parent and other
tests never load it. Script unchanged (no seam): syncBuiltinESMExports updates the live named binding the script imports.
Only createReadStream call on a *.tar.gz.part path is kept-logs.mjs:252 (after write :234-249 and fsync :250-251; before
readBack and the second walk :261). Messages asserted are unique to the second walk (:262 text; "late-link ... not a
plain file" only from walk). If the hook ever stops firing, pack exits 0 and test 5 goes red (loud). No timers: no flake;
passed 4x concurrent plus 9 mutant runs.
### B4 Q3b KLC-F2 (done)
The two branches: walk's name check (kept-logs.mjs:88-89) and "holds no files" (:98) on the second walk. Both keep the
folder under any single change: :98 removed -> differs() still catches the empty list; :89 rewired to refuse -> exit 2,
folder stays; :89 made to skip names -> test 7 bites (shared walk). Follow-up enough (Low): a 4th case (late `.hidden`).
### B5 evidence items (done)
(a) holds (my run + full.tap). (b) holds: dry-*.tap ok, no SKIP; fingerprints equal; test 10 also asserts real file sha
== list (test.mjs:321). Fingerprint = path/size/mtime of regular files only, no pipefail (find failure would hash
empty input; before-values are not the empty hash, so the record holds). Sound together with :321; alone it is not a
content proof. (c) ok 5 line present. Ready-report items (2159): head, files, script compare all in PR body.
PR body: many links wrapped in backticks with spaces -> do not render as links (Low).
### B6 coverage gaps (done)
Untested refusal/failure paths (all fail safe by reading): pack routine refusal (only list/cache tested, root() shared);
detached HEAD :206; free-space :230; :237 and :245 during packing (read-back is the backstop); parseList :110/:113 (no
mutant); got.length branch :255; readBack size NaN :196; rmSync failure :273; cache worktree-list failure :293; a link
INSIDE the cache (cache test plants only a file); test 7 mkfifo case can vanish without a SKIP (:229); kept-logs tests
are not in CI (.github/workflows/gate.yml runs no kit test; same as the other kit tests).
### Verdict: APPROVE WITH FINDINGS (Lows only)
