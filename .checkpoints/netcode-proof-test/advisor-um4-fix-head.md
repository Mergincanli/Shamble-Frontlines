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
- red-first.log (Read whole, 207 lines): tests 3, pass 0, fail 3. Each failed on its own assert:
  - the missing-folder pin (:2069): strictEqual, relative against absolute prefix;
  - the env pin (:2093): deepStrictEqual at :2103, `undefined` where the kept set was expected;
  - the victim test (:2109): "the repository GIT_DIR names is unchanged, byte for byte" at :2136.
  - The victim's diff shows new objects, a changed HEAD, COMMIT_EDITMSG and index, a new `refs/heads/side` and a moved
    `master`. So UM4R-1's effect (a `side` branch, commits in the named repository) is now SHOWN on a scratch victim,
    no longer only reasoned. keep.txt is unchanged (the work tree was the temp folder).
- base-t1, -web, -t2, -t4, -pin and -victim .log: each "exit 0; tests 1, fail 0: PASS", one `ok 1 - <its title>`,
  skipped 0, todo 0.
- Mutants, each "exit 1; tests 1, fail 1: KILLED", on the right assert:
  - M1: case "0", s.ok true !== false, :2044.
  - M3: case "1", same assert.
  - W1: case "node tests", :2000.
  - M4: T-2's noise case, :2083, "its last line: noise on stderr".
  - M10: T-2's extra case, :2089.
  - M8: the path pin, :2020, ops/browser against ops/browser/node_modules.
  - M-env: the env pin, :2103, all five stand-in GIT_ names in the call's env.
  - M-wild: the same assert. The diff shows `+ GIT_DIR: '/tmp/npt-sec-um4-1-no-such-repo/.git'` in the call's env for
    `/opt/node22/bin/node --test ops/**/*.test.mjs`.
- V1, V2, V3: each KILLED, with the location :2109 and the stack at :2136, which is the byte-for-byte assert. The
  three-title assert sits after it, so it was not reached. V3's diff shows `refs/heads/side` added in the victim.
  (Read through Bash grep before the 21:07 note; from here on logs are read with Read and Grep only.)
- Blobs (`git rev-parse f1b46682:<path>` against `git hash-object` of each .gate/um4-fix/fixed-*.mjs, and of the working
  files): equal for all four, d6498e4e, 17d27fc0, e78deb4a and 8f604833. The kept copies (20:52) and today's tree ARE the
  committed bytes. fixed.sha256 (Read) lists the four paths' sha256; I did not recompute its sha256s, since the blob ids
  tie the copies to the commit more strongly.
- counted.log (Read):
  - Run at 20:53:11 on HEAD a7c4b25b. diff 446f383d…; untracked e3b0c442… (empty).
  - self 88 of 88; ops "104 of 104 tests passed; wrappers.test.sh: 315 ok, 0 failed"; redact "11 of 11 tests passed;
    redact-check: 464 item(s) checked, 0 finding(s)"; refs "2181 tracked files"; exit 0.
  - main-run.sh:42 hashes `git diff HEAD -- .` from the spike folder, so the diff line covers spikes/netcode-proof against
    a7c4b25b.
  - NOT re-done by me: hashing `git diff a7c4b25b f1b46682 -- spikes/netcode-proof` needs a pipe, and this seat keeps
    Bash to single git commands. The blob identity above holds the same tie for the four code files.
  - The other spike files: `git diff --name-only` between a7c4b25b and f1b46682 lists only those four plus notes.
- light-base.log (Read): 20:46:00 at HEAD a7c4b25b with diff and untracked empty. self 85 of 85; ops, redact and refs the
  same lines as counted.log. counted.log against base: self +3 (85 to 88, the three new tests: T-2, the env pin, the
  victim); the other three lines are equal word for word.
- gate.txt (Read): base b8f02b0 (origin/main); [1]-[4] and [5b]-[5d] PASS, [5a] SKIP (--no-smoke); "GATE PASS".
- Since the fix: `git diff --name-only f1b46682 HEAD` lists notes only (.checkpoints/...).
  `git diff --name-only f1b46682 1d4e81a6` lists .checkpoints/netcode-proof-test.md and um4-plan.md, so the pushed save
  carries the fix unchanged.
- This repository's state (single git commands):
  - `git show-ref --verify refs/heads/side`: "fatal: 'refs/heads/side' - not a valid ref" (exit 128): no side branch.
  - `git symbolic-ref HEAD`: refs/heads/claude/netcode-proof-test-plan-hcky5g.
  - `git log -6`: only the session's checkpoint saves since 20:59; no "results", "a leak", "base", "side" or "merge side"
    commit.
  - `git status --short --branch`: even with origin; only this file modified.
- Whole-spike search (Grep, .gitignore respected, so node_modules is left out): `["']git["']` gives exactly the main
  session's list. A second Grep for `git <subcommand>` in every non-.md file adds:
  - ops/cloud-init/lg.yaml:113-161 and rs.yaml:145-234: WRITERS (`git init -q /opt/npt/clone`, fetch, `checkout
    --detach`), run by cloud-init on a fresh VM at boot. No inherited caller env there, and the tests read these files
    only as text.
  - ops/sbin/npt-userdata:17, :20: reads (`rev-parse HEAD`, GIT_CONFIG_NOSYSTEM=1).
  - Comments and permission strings only: check.test.mjs:1527, redact-check.mjs:15 and :332, redact-check.test.mjs:217,
    ops/operator/settings.json (git push rules), operator.test.mjs:17-98 (assert strings), r1plan.test.mjs:211,
    c-u4f-cap.mjs:18 and :110, c-u4f-runs.mjs:34.
  - siocoutq-root-check.py:153-179 is the status read at :164.
  - No Rust `Command::new("git")` anywhere (the quoted grep covers .rs).
