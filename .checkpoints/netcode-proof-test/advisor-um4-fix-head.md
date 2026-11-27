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
- The refs step (check.mjs:2729-2734, Read): `run("git", ["ls-files", "-z"], ROOT)` with process.env, then readFileSync of
  each listed path. Under an exported GIT_DIR naming another repository, the list is that repository's. A path missing
  under ROOT throws out of the step, and check.mjs's top level prints "check: …" with exit code 2 (:2808-2809).
- `git grep` at a7c4b25b (the parent): um4-plan.md:215-222 holds "THE FIX PASS, recorded before any edit", naming
  operator.test.mjs (:157), redact-check.test.mjs (:161, :197), `withoutGitVars(env)` and opsStep's `base`. My ruling's
  section 6 condition is met.
- `git diff --name-status a7c4b25b f1b46682`: all seven files are M; nothing added, moved or deleted. There is no
  .gate/um4-fix/ log of the repo-map check (Glob `*map*`: none).
- Commit message (`git log -1 --format=%B`, read whole): "red first" names only the three tests in red-first.log; V1-V3
  as each helper site restored alone; mutants by name with their killing tests; the counted run's lines and diff
  prefix as in counted.log; "GATE PASS (--no-smoke); repo map OK"; the 3-of-5 correction of 8fb32bc8 word for word in
  substance; carries T-3, UM4R-2, T-5. Session and co-author lines present.
- CI (`gh run view`, my own read): gate 46066802730 (push) and 46066808752 (pull_request), toolchain 46066803492 (push)
  and 46066808757 (pull_request). All four: headSha 1d4e81a6…, completed, success, attempt 1.
- PR #40 (`gh api repos/{owner}/{repo}/pulls/40`; GraphQL is refused in this seat): open, draft, head 8d4e917e (the
  session's later [skip ci] notes saves), mergeable true, mergeable_state "blocked" (not "dirty"). The body's "Closing
  list: security findings left open" names SEC-U6B-3, SEC-U6C-R2-1, SEC-U6C-R2-2 and U6c's page-host item. No UM-4 entry.

## 1. My ruling's items against the commit: every item HOLDS (DONE)
| Ruling item (advisor-um4-r1.md) | In f1b46682 | Holds |
|---|---|---|
| 2(a) the `GIT_*` wildcard, pure exported function, `base` as webStep's, both calls, keys left out, no ceiling, redact step untouched | check.mjs `withoutGitVars` (Object.fromEntries + filter), `opsStep(runner, present, base = process.env)`, `env` to both runner calls, STEPS.ops unchanged, no CEILING, redact step's diff empty | yes |
| 2(b) three sites rewritten in place at 0 net | operator.test.mjs:157 `env:` added on the same line (numstat 1 1); redact-check.test.mjs:161, :197 the filtered spread (2 2); GIT_AUTHOR_*, GIT_COMMITTER_* kept; :162's pre-push spawn untouched | yes; U8 stays 6,282 of 6,284 |
| T-1 stand-ins past the neighbouring guards, exact messages | `# tests 104/# pass 103/# fail 1` status 1, the valid summary status 1; `(103 of 104 tests passed) failed`, `wrappers test failed`; `calls.length === at + 1` kept | yes |
| The web step's Node stand-in | `# tests 3/# pass 2/# fail 1`, `(2 of 3 tests passed) failed` | yes (npm ci and tsc tightened too: 3b (iii)) |
| T-2's two cases | one test: stderr noise after the summary in `out` passes; `extra` fails "its last line: extra" | yes (3b (iv)) |
| T-4's pin | `assert.equal(OPS_BROWSER_MODULES, join(HERE, "ops", "browser", "node_modules"))` | yes |
| SEC-UM4-3's prefix, its pin built by join | OPS_MISSING template with `${join(HERE, "ops", "browser")}`; the pin is `assert.equal` of a template using the same join; one line kept | yes |
| UM4H-1 in S's words | dependency-check.md: S's clause word for word | yes |
| SEC-UM4-2's line | S's words plus "Read on 2026-11-27 after the install, not recorded with it." | yes (the addition is true and honest) |
| Section 6: the plan records the file list and symbols before any edit | um4-plan.md:215-222 at the parent a7c4b25b | yes |
| Carries | T-3 and UM4R-2 (STATUS), T-5 (infra-engineer): in the commit message and um4-plan.md | yes |

## 2. The logs: they hold (DONE)
- red-first.log: 3 of 3 failing, each on its own assert.
  - The missing-folder pin: a relative against an absolute prefix.
  - The env pin: `undefined` where the kept set was expected, so the shipped opsStep passed no env.
  - The victim test: the byte-for-byte assert. The victim gained commits and objects, a `side` branch and a moved
    `master`.
- base-t1, -web, -t2, -t4, -pin and -victim: each 1 test, 0 fail, exit 0, under its own title. No skip or todo.
- The mutants, each KILLED for the right assert:
  - M1 and M3: the either-part test's cases 0 and 1 (`s.ok` true, :2044).
  - W1: the web test's Node case (:2000).
  - M4 and M10: T-2's two cases, at two distinct lines (:2083, :2089).
  - M8: the path pin (:2020).
  - M-env and M-wild: the env pin (:2103). M-wild's diff shows GIT_DIR in the env of the `node --test ops/**/*.test.mjs`
    call.
  - V1, V2 and V3: the victim test's byte-for-byte assert (stack :2136), never the title assert after it.
- fixed.sha256 and the copies: each kept copy's blob id equals `git rev-parse f1b46682:<path>` for all four files. The
  mutants ran on the bytes that were committed, and the tree was restored to them.
- counted.log against the fresh base:
  - self 85 to 88, the three new tests.
  - ops, redact and refs unchanged word for word: 104 of 104 and "315 ok, 0 failed"; 11 of 11, 464 items, 0 findings;
    2181.
  - The real ops step ran with every `GIT_*` left out, the container's GIT_CONFIG_* included, and passed. So 2(a)'s stop
    rule was never needed.
- gate.txt: GATE PASS (5a skipped by --no-smoke; CI's smoke job ran on the push, below).
- Not reproduced in my seat: counted.log's diff hash 446f383d… against the commit's spike diff, which needs a pipe. The
  tie holds another way: all four code files are byte-identical by blob id, and the commit changes no other spike file
  (name-status).

## 3. (a) and (b) against S's probe, then the whole spike folder (DONE)
- The victim test exercises S's hazard itself, not an analogue.
  - S's probe put an absolute GIT_DIR in the env and ran operator.test.mjs's three calls in a temporary folder.
  - The victim test runs the REAL operator.test.mjs (those calls) and the real redact-check.test.mjs. Their env holds
    an absolute GIT_DIR and GIT_INDEX_FILE naming a scratch victim outside this repository (asserted by `relative`).
  - On the shipped helpers the victim changed (red-first.log), so the probe's effect is reproduced through the real
    files. It also shows UM4R-1's `side` branch, which was only reasoned in my ruling.
  - With the fix, the victim is byte-for-byte unchanged (base-victim.log), and each site restored alone breaks that
    again (V1-V3).
- (a) is covered by the env pin, not by the victim test. That is enough:
  - the pin shows opsStep hands both calls an env with no `GIT_*`;
  - `run` hands that env to spawnSync (check.mjs:1951-1952);
  - M-env and M-wild show the pin catches a missed or partial filter.
- The whole spike folder (Notes). Every place git runs by name, sorted:
  - Temporary-repository writers in tests: check.test.mjs:1512 (u3c C2b F2) and :2122 (the new victim test);
    operator.test.mjs:157; redact-check.test.mjs:167 and :203 (env from :161, :197); test_siocoutq_root_check.py:30.
    All leave `GIT_*` out now.
  - Other writers: ops/cloud-init/lg.yaml:113-161 and rs.yaml:145-234 (`git init`, fetch, checkout, under `-C`).
    - These extend the main session's list.
    - They run once, by cloud-init on a freshly booted VM, as root, from the cloud-init file: no caller's environment
      reaches them.
    - The tests only read the YAML. Outside this hazard: no finding.
  - Reads by design: redact-check.mjs:291, which must honour the GIT_DIR a pre-push hook gets.
  - Reads inside opsStep's filtered env: browser.test.mjs:109.
  - Reads in run scripts, not tests: ops/runs/c-u2-pm-0001-3.mjs, c-u4f-cap.mjs, c-u4f-r2.mjs, c-u4r-r2.mjs,
    c-u4r-runs.mjs; siocoutq-root-check.py:164; u3d-timing/run.sh:102-103; ops/sbin/npt-userdata:17, :20 (also an
    extension of the list; a sudo wrapper on the VM, reads only).
  - Read with the inherited env inside check.mjs: the refs step (check.mjs:2730). See UM4H2-2 below.
  - No Rust code runs git.
- This repository after the four vulnerable runs (red-first, V1-V3):
  - no `refs/heads/side` (show-ref: "not a valid ref");
  - HEAD is refs/heads/claude/netcode-proof-test-plan-hcky5g;
  - the last six commits are the session's checkpoint saves only;
  - the tree is clean apart from this file.
  - So the vulnerable runs touched only their scratch victims.

## 3b. The main session's departures from my ruling: APPROVE all seven (DONE)
- (i) A fresh base at H0 instead of .gate/um4/light-base.log: APPROVE.
  - It is the counted run's own parent, with diff and untracked both empty.
  - Its refs count (2181) already holds the notes added since 8fb32bc8, so the compare is like for like. The old base
    would differ for reasons that are not the fix.
- (ii) The counted run before the commit, on the uncommitted tree, tied by the diff hash: APPROVE.
  - This was UM-4's own accepted form.
  - The tie holds by blob identity of the four code files and by name-status (no other spike file changed), even
    though my seat did not recompute the hash.
- (iii) The web test's npm ci and tsc parts also assert exact messages: APPROVE. Stricter, in the same test, with no
  new behaviour.
- (iv) T-2 as one test with two cases: APPROVE. T asked for two cases. M4 and M10 fail at two distinct lines (:2083,
  :2089), so each case is shown on its own.
- (v) The missing-folder pin red first as well: APPROVE.
  - SEC-UM4-3 changed that message (new behaviour), so red first is the right proof there.
  - The commit message counts it among the three.
- (vi) check.test.mjs +88 net against my estimate of +45 to 60: APPROVE.
  - The estimate was not a stop, and these lines sit outside U8's stop (U8-FILLS (a)).
  - The excess is the victim test's three exact titles with their escaping and its outside-the-repository assert, plus
    the table forms of T-1 and the web test.
  - Name it in the report as "+88 against an estimate of +45 to 60".
- (vii) The namespace import (`checkModule.withoutGitVars`): APPROVE, and it is needed.
  - A named import of a missing export fails the whole file at link time. On the shipped check.mjs every test would
    then be red, none by its own name.
  - With the namespace import only the env pin failed (red-first.log: 3 tests, each on its own assert).

## 4. The commit message: HOLDS, one Low (DONE)
- "Red first" is claimed only for the three tests in red-first.log.
- V1-V3 are described as "each helper site restored alone".
- T-1, T-2, T-4, the web test and the env filter are claimed through named mutants and their killing tests, never as
  "red first".
- The counted run's figures equal counted.log.
- The 3-of-5 correction of 8fb32bc8 is present: three of five red first, the failing-part and call-order tests arrived
  green.
- It does not claim T-3, T-5 or UM4R-2; it carries them.
- UM4H2-1 (Low): "repo map OK" has no kept output under .gate/um4-fix/ (Glob: none).
  - Nothing is wrong: the commit adds, moves and deletes no file (name-status: 7 × M), so the repo map cannot move.
  - The report says "printed, not kept", or the next counted run keeps it. It rides the report, with no push.

## 5. CI and PR #40 (DONE)
- My own `gh run view` of all four runs: gate (push 46066802730, pull_request 46066808752) and toolchain (push
  46066803492, pull_request 46066808757). Each: headSha 1d4e81a6…, completed, success, attempt 1.
- The pushed save 1d4e81a6 changes only .checkpoints/netcode-proof-test.md and um4-plan.md over f1b46682, so CI ran on
  the fix's code unchanged.
- PR #40 (REST; GraphQL is refused in this seat): open, draft, head 8d4e917e (later [skip ci] notes saves, notes only
  since f1b46682), mergeable true, mergeable_state "blocked" (draft and protection), not "dirty".
- The PR's final head will need its own CI before any close, since [skip ci] saves get none. That is noted for the
  close, not for this read.
