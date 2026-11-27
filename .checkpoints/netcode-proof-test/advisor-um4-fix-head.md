# Advisor: UM-4 fix pass, head read (fix f1b46682, pushed in 1d4e81a6), 2026-11-27, effort xhigh

Plan: main's advisor.md unchanged since c9bd268 (re-checked against b8f02b01); my ruling advisor-um4-r1.md sections 2-4
and 6. Then the commit: `git show --stat`, `git diff --numstat` and `git diff a7c4b25b f1b46682 -- <file>` per file;
um4-plan.md's "FIX PASS DONE LINES". Then each log under .gate/um4-fix/ printed, with fixed.sha256 against
`git rev-parse f1b46682:<path>` and `git hash-object` of each fixed copy, and counted.log's diff hash against the diff
I print myself. Then the hazard re-read: the victim test against sec-um4-1-probe.md; my own whole-spike search for git
run by name (spikes/netcode-proof, node_modules aside), beside the main session's list; m-wild.log's diff; this
repository's state (`git branch --list side`, `git symbolic-ref HEAD`). Then 3b's seven departures, one ruling each;
the commit message; CI per run with `gh run view`; PR #40's state. Last: verdict, findings on the scale, UM-4's
close, SEC-UM4-1 on PR #40's closing list, Decisions needed, For Mustafa, status line. One simple command per Bash
call, git and gh first; no test, cargo, gate, mutant or plant run; no git variable exported.

## Notes (one line per check)
- advisor.md on main: last changed c9bd268f (`git log -1 origin/main -- .claude/agents/advisor.md`); main is b8f02b01.
- f1b46682: parent a7c4b25b, 7 files, 160/18 (`git show --stat`). numstat: check.mjs 13/5, check.test.mjs 98/10,
  operator.test.mjs 1/1, redact-check.test.mjs 2/2, dependency-check.md 6/0, um4-plan.md 17/0, um4-fix-mutants.mjs 23/0
  (that file existed at a7c4b25b, index d12985cf; this adds its baseline mode).
- check.mjs diff:
  - withoutGitVars(env) is exported: a copy built by Object.fromEntries, filtering `!k.startsWith("GIT_")`.
  - opsStep has a third parameter `base = process.env`. `env = withoutGitVars(base)` goes to both runner calls.
  - STEPS.ops is unchanged.
  - OPS_MISSING is a template that prints `${join(HERE, "ops", "browser")}`.
  - The comment names SEC-UM4-1.
  - No GIT_CEILING_DIRECTORIES. The redact step is untouched, as ruled.
- ops/ diff: operator.test.mjs:157 adds `env: Object.fromEntries(...filter(!GIT_))` in place. redact-check.test.mjs:161
  and :197 change `...process.env,` to the filtered spread. The GIT_AUTHOR_* and GIT_COMMITTER_* lines are kept.
  The pre-push spawn (:162) is unchanged, as ruled.
- check.test.mjs diff:
  - It imports OPS_BROWSER_MODULES and adds `import * as checkModule`.
  - The web test: all three parts now have exact messages. The Node stand-in is `# tests 3/# pass 2/# fail 1`.
  - T-4's pin is at the head of the first ops test.
  - T-1: stand-ins `# tests 104/# pass 103/# fail 1` and the valid summary, status 1. Exact regexes; `calls.length` kept.
  - The missing-folder pin is now `assert.equal` on the whole string, built by join.
  - T-2 is one test with two cases: stderr noise after the summary in `out` passes; `extra` fails with "its last line:
    extra".
  - The env pin: the base holds a nonexistent tmpdir path for GIT_DIR, plus WORK_TREE, INDEX_FILE, CONFIG_COUNT and
    EDITOR. It asserts each call's env deepEqual the kept set, the base unchanged, and the filter directly.
  - The victim test: the C2b F2 form. A mkdtemp victim outside the repository (asserted by `relative`). The child env
    leaves out `GIT_*` and NODE_TEST_CONTEXT, then adds GIT_DIR and GIT_INDEX_FILE naming the victim. The cwd is a
    mkdtemp folder. Both files run with `--test-reporter=tap`. The snapshot assert comes BEFORE the three exact-title
    asserts (`^(?:not )?ok \d+ - <title>$`).
- dependency-check.md: S's UM4H-1 clause word for word (my compare with review-um4-security.md:136-138: same words,
  framed as "The count (UM4H-1, in the security read's corrected words ...)"). SEC-UM4-2's line is S's words plus "Read
  on 2026-11-27 after the install, not recorded with it."
- um4-fix-mutants.mjs (read whole):
  - It applies each mutant in place to the real file, runs check.test.mjs filtered by name, then restores the file,
    throwing if its sha256 differs.
  - KILLED means exit not 0, fail > 0 and tests > 0.
  - Every log is a literal full path. No git variable is set; NODE_TEST_CONTEXT is removed from the env.
