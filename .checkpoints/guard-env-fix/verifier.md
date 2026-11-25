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
- FINDING V-2 (rating pending sweep): since 11887d0 the -S re-insertion means unwrap()'s argv is no longer always a suffix of c.argv (plan §6 says it is); dayFolder (routine-guard.mjs:959) computes its prefix from the length difference, so with a one-word attached split value the prefix can come out empty and a folder change inside the value is tracked as if it ran. One decide() probe: main session ASK (reason: the env -S form), agent DENY; only the shown reason changes. Exact form: only in the answer to the caller. Follow-up: compute the prefix in unwrap() itself, or mark the folder unknown when unreadable.
- B6 done: conditions vs code/tests:
  c1 met: routine-guard.mjs `if (a[0]?.startsWith('-')) unreadable ??=` after env's = words and nice/stdbuf options; rows `env X=1 -i` (ask/agent deny) + force-push ASK row; mutant present and caught.
  c2 met: ask reason "does not follow <form> in front of a command (where git runs, on which words, or whether it runs)"; agent deny names the form; 2 rows assert reason text + every unread row asserts /does not follow (env|nice|stdbuf) /.
  c3 met: unread flag set before the value read-on, unknown flag set before the drop; rows `env --block-signal git push --force` DENY and `env -- - X=1 git push --force` DENY (known-forms list).
  c4 met: 12/12 new mutant targets occur once (B7); test diff additions only (B2); baselines 127/33 equal (B3/B4).
  c5 PARTLY: probe updated for {argv, unreadable} and after-output kept, but probe-after.txt is stale (V-1).
  c7 met as far as checkable: checkpoint commits follow each code commit (4a9fb50 after 21615cb, da7266e after 11887d0, 2c79f0f after c3827f4); gate + kit tests reproduced here; interim rule (no live tries) not checkable from the repo.
  c6 docs step: not yet due.
  Routine path: unwrapRole/R-PROG not in the diff; 6 routine rows pass. unwrap() has one caller (routine-guard.mjs:755).
- B9 done: commit-message claims of 21615cb, 11887d0, c3827f4 all hold (counts, mutants, SA-R1-2 deny as on main for separate -S: main deny/head deny; attached/quoted forms main null -> head deny).
- V-2 sweep (decide() only, inline, form kept out of notes): 51 main-session DENY rows of DAY_CASES x 7 folder targets x 2 separators = 714 lines with the split-value folder change in front: 0 flips (all still DENY). V-2 stays Low.
- Option table check: help text embedded in /usr/bin/env, nice, stdbuf (coreutils 9.4-3ubuntu6.3; read with grep -a, binaries not run): env -0/--null, -C/--chdir=DIR, -S/--split-string=S, -i/--ignore-environment, -u/--unset=NAME, -v/--debug, signal options; nice -n/--adjustment=N; stdbuf -i/-o/-e with --input/--output/--error=MODE. WRAP_OPTS matches; signal options, --help, --version fall to unknown (ask). Holds.
- Symbols: added WRAP_OPTS (plan §2) and shownWord (SA-R1-4, named in the brief); changed unwrap() return shape and bash() caller; nothing removed. Matches plan + security round.
- V-1 rated High on the project scale (evidence that does not hold; cond. 5 folds it into the report). Fix: regenerate probe-after.txt from the head (checkpoint-only commit), re-run the probe diff; code head unchanged.
- Size: 94 added lines in the two kit files (47+47), under the 150-line cap.
- Not verified: smoke 5a (Chromium build 1194 missing, container has 1243); interim rule (no live tries) not checkable from the repo; docs step (cond. 6) not yet due.
- B10: verdict FAIL (V-1 only; code head c3827f4 clean on checks 1-4, 6, 7 and conditions 1-4, 7). .gate/verify-main removed with git worktree remove.
Re-run (after a163b88, V-1 fix):
- probe-unwrap-inscope.mjs output vs probe-after.txt: no diff (21/21 rows; the two split rows now show program x). V-1 closed.
- git diff --name-only c3827f4 HEAD: only .checkpoints/ files (guard-env-fix.md, probe-after.txt, verifier notes and scripts); .claude/hooks diff empty. Code head still c3827f4.
- V-2 (Low) goes to STATUS as a follow-up (caller). Final verdict: PASS WITH NOTES.
DONE
