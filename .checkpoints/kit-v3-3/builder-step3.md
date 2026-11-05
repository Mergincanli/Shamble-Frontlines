TASK: kit-v3-3 builder, plan rev2 section 8 step 3: the --output fix and git rev-list (4.1, 4.4) with tests (5).

Plan (blocks, in order):
B1 read plan 0/4/5/8 + advisor notes on output/rev-list
B2 read routine-guard.mjs sites (GIT_LONG_DANGER, GIT_ROUTINE, GIT_READ, gitArgsCheck) and settings.json:177, tests
B3 baseline test counts
B4 edits: routine-guard.mjs, settings.json
B5 tests added (routine-guard.test.mjs group "rev-list and --output", real-git extension, settings NEVER list)
B6 test run after
B7 mutation rows in .gate/kit-v3-3/
B8 answer

Notes:
- B1 done: plan 4.1 (GIT_LONG_DANGER +rev-list/diff/log/show --output; GIT_ROUTINE, GIT_READ +rev-list), 4.4 (settings deny Bash(git rev-list *--output*) after :177), 5 group "rev-list and --output", settings NEVER list +1, MUTANTS rows for the --output entries.
- B2 done: guard GIT_LONG_DANGER :184-197, GIT_ROUTINE :213-215, GIT_READ :217-219, gitArgsCheck :221 (all sessions, via git() :500). settings deny :175-177 diff/log/show --output. MUTANTS array test file :370-411; real-git test :304.
- finding: git 2.43 --git-completion-helper lists --output= for rev-list and diff only; log and show list none, so today `git log --outp=x` passes the abbreviation rule.
- finding: settings.json has no allow rule for `git rev-list` (allow :15-69); plan 4.4 says "nothing else"; flag only, not changed.
- decision: plan says "the real-git test (:304) extended"; C6 says existing tests unedited -> add a NEW real-git test instead of editing :304.
- B3 done: baseline routine-guard 192/192, settings 122/122, combined 314/314. Next B4 edits.
- B4 done: routine-guard.mjs :197-201 (comment + 'rev-list'/diff/log/show: ['--output'] in GIT_LONG_DANGER), :220-221 GIT_ROUTINE +'rev-list' (line rewrapped: :220 ends "'symbolic-ref'," and :221 "'rev-list']);"), :225 GIT_READ +'rev-list'; settings.json :178 deny "Bash(git rev-list *--output*)". Existing 314/314 still pass, json parses. Next B5 tests.
- B5 written: CASES +16 (section before "unknown role"), new real-git test after :304 test, MUTANTS +4, existsSync as a separate import line, settings NEVER.push. Next B6 run.
- finding (E4, git 2.43.0, probe .checkpoints/kit-v3-3/e4-probe.mjs, throwaway repo in OS tmpdir): --output=<f> writes a file for rev-list, log, show, diff (exit 0). The abbreviation --outp does NOT write: rev-list and diff exit 129 (ambiguous with --output-indicator-*; --outpu too), log and show exit 128 "unrecognized argument". So the plan's "--outp writes" prediction is not shown (false on this git); the guard refuses --outp anyway.
- the first draft of the new real-git test asserted --outp writes: failed (129). Rewriting it: full spelling writes for 4 subcommands; --outp only guard-refused (git behaviour not asserted, may differ by version).
- the Bash tool's permission rules refused a direct shell probe (git log --output in a scratch repo): settings deny works for the full spelling (same as reader-routines B5).
- B6 done: after: routine-guard 209/209 (192 + 16 CASES + 1 real-git test; MUTANTS rows run inside the existing mutation test), settings 123/123 (122 + 1 NEVER), combined 332/332. No existing line removed or changed in either test file (git diff -U0 shows no - lines). Next B7 mutations in .gate/kit-v3-3/.
- B7 done (script .checkpoints/kit-v3-3/mut-step3.mjs; copies in .gate/kit-v3-3/mut-step3-0..6/, git-ignored; the rm was refused by the permission system, so they stay):
  base HEAD guard + new tests: 195/209; failing = 12 new CASES (advisor and night rev-list --count; advisor log/diff/show --output; night log --outp; day rev-list/log/diff/show --output; day log/show --outp), the new real-git test, the mutation test (new MUTANTS targets absent). New cases passing on base: advisor/night rev-list --output and --outp (rev-list was off the lists), day diff --output-indicator-new (null).
  rev-list --output removed: 204/209; fails advisor/night/day rev-list --output, real-git test, mutation test.
  diff --output removed: 205/209; fails advisor/day diff --output, real-git test, mutation test.
  log --output removed: 203/209; fails advisor log --output, night/day log --outp, day log --output, real-git test, mutation test.
  show --output removed: 204/209; fails advisor/day show --output, day show --outp, real-git test, mutation test.
  extra (not plan rows): rev-list off GIT_ROUTINE 206/209 (advisor/night rev-list --count); off GIT_READ 207/209 (advisor rev-list --count).
- line refs at the end: routine-guard.test.mjs :13 import existsSync (new line); CASES :275-291 (comment + 16 rows); new real-git test :341-363 (comment :341-343, test :344); MUTANTS :452-455; settings.test.mjs :55 NEVER.push.
- B8: answer written from these notes.
DONE
