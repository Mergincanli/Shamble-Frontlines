Task: verify guard-env-fix (risk tier 2) on code head c3827f4, base main 3ab9eaec.

Plan (blocks, in order):
B1 orientation: branch state, file list, numstat (checks 3-files, 7)
B2 diff of test file additions-only (check 2)
B3 test suites on head (check 1 head)
B4 test suites on main worktree (check 1 main)
B5 gate run (check 3)
B6 read guard diff + plan §8 conditions vs code/tests (check 5)
B7 DAY_MUTANTS new rows: target once + caught (check 4)
B8 probe-after vs head (check 5)
B9 commit message claims 21615cb, 11887d0, c3827f4 (check 6)
B10 report

Notes:
- B1 done: working tree 2c79f0f differs from c3827f4 only in .checkpoints/guard-env-fix.md; merge-base = origin/main = 3ab9eaec. Files changed vs main: 2 kit files (.claude/hooks/routine-guard.mjs, routine-guard.test.mjs) + 10 under .checkpoints/; no game/legacy/tools file. numstat: guard +47/-6, tests +47/-0 (matches claim). Guard diff read (WRAP_OPTS Map, shownWord, unwrap returns {argv,unreadable}, bash() caller asks/denies agents, scan goes on). Next B2.
- B2 done: test diff vs main is additions only (numstat +47/-0; 0 removed lines); 3 hunks: DAY_CASES rows, 12 DAY_MUTANTS rows, 3 routine forms via roleRows. Next B3.
- B3 done: head (tree = c3827f4 for hooks): routine-guard.test 1247/1247 fail 0; settings.test 127/127; checkpoint.test 33/33. Arithmetic: 39 known forms x2 + 21 unread forms x2 + 14 single rows = 134 day rows, + 3 forms x 2 roles = 6 routine rows; 1107+140 = 1247. Next B4.
- B4 done: main 3ab9eaec worktree .gate/verify-main: routine-guard.test 1107/1107; settings.test 127/127; checkpoint.test 33/33. Next B5 gate.
- B5 done: node tools/gate.mjs (smoke on): checks 1-4, 5b-5d PASS; 5a FAIL: Chromium executable missing /opt/pw-browsers/chromium_headless_shell-1194 (container has -1243 only) -> GATE FAIL, environment only. With --no-smoke: GATE PASS. Symbols +0 -0. git diff --name-only: only .claude/hooks/* and .checkpoints/* (no legacy/tools/docs file). Next B6/B7.
- B7 done (script verifier-mutants.mjs; copies in .gate/guard-env-fix/vmut): 12 new DAY_MUTANTS rows, each target occurs exactly 1 time in the guard and each is caught (cases failing per mutant: 16, 21, 17, 4, 4, 1, 9, 1, 1, 47, 21, 5). Unmutated head guard: 0 of 436 DAY_CASES fail. Head's DAY_CASES on main's guard: 67 fail, all new rows (41 deny->null i.e. main let the force push through silently, 25 ask->null, 1 ask->deny), 0 old rows fail.
- Note (info, per plan cond. 1): the 1 ask->deny row is `env X=1 -i git push --force`: main denied, head asks. GNU env would run the program "-i", not git; plan §8 cond. 1 asks for ask here. Not a defect.
- B8 done: FINDING V-1: probe-after.txt does NOT match the head. `node .checkpoints/guard-env-fix/probe-unwrap-inscope.mjs` on c3827f4 differs in 2 of 21 rows: `env -S x git status` and `env --split-string x git status` now give program `x` (unreadable env -S / env --split-string); probe-after.txt says `git`. probe-after.txt was written at 4a9fb50 and equals the 21615cb guard's output exactly (reproduced via .gate/guard-env-fix/p21); 11887d0 (SA-R1-2: -S value split and read on) changed these rows and the file was not regenerated. Head behaviour is the intended one (GNU env -S x runs x) and is pinned by the SA-R1-2/SA-R2-1 rows; the defect is a stale evidence file that cond. 5 says gets folded into the report. Severity: Medium (stale evidence, behaviour correct and tested; fix = regenerate probe-after.txt before the report). Next B6 conditions + B9 commit messages.
- B9 partial: snapshots .gate/guard-env-fix/at-21615cb: 1237/1237, 11 new mutants (claim 130 new tests = 1237-1107, 11 mutants: holds). at-11887d0: 1244/1244, 12 new mutants (claim 7 rows + 1 mutant: holds; its test diff edits one line added by 21615cb, so vs main still additions only). Auditor extras (verifier-extra.mjs): at 11887d0 X1-X9 + X10 (adapted to shownWord text) all kind-caught (claim "10 extra mutants all caught" holds). At head Y1-Y5, Y8 kind-caught; Y6, Y7 caught only by reason rows; Y9 (tab not replaced) uncaught (= auditor's nit; c3827f4 msg says the reason rows cover cut and newline: holds). c3827f4: +3 rows, 1247: holds.
- B6 in progress. Possible finding (checking): 11887d0's -S re-insertion breaks plan §6's invariant 'argv is a suffix of c.argv' used by dayFolder (routine-guard.mjs:959 prefix = c.argv.slice(0, c.argv.length - argv.length)); with an attached one-word -S value prefix can be [] and a cd inside the value gets tracked. Checking with decide() only.
