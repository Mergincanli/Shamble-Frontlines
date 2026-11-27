# Advisor: UM-4's round-1 ruling (head 8fb32bc8, pushed in 787eb984), 2026-11-27, effort xhigh

Plan: read main's advisor.md (its severity scale is CLAUDE.md's; PR review step 4; the review budget);
review-um4-nb2b-test.md block 1 (T-1 to T-5, T's fix); review-um4-security.md whole (SEC-UM4-1 to -3, UM4H-1's
corrected clause, :51-53, :136-138, :165-171); my advisor-um4-nb2b-head.md:56-60; um4-plan.md's "Review round 1";
check.mjs's opsStep and run (the env passed to children), check.test.mjs's ops tests and the web step's test
(:1985-1995); operator.test.mjs's git helper (:157); sec-um4-1-probe.md; checkpoint.mjs's save path (what a save would
push); history.md:8133-8151 (U8-FILLS (a)) and :7561-7593 (NPT-TRIM). Then one block per item 1-6 and the memory line;
Decisions needed; For Mustafa; status line. One simple command per Bash call, git first; no test, cargo or gate run.

## Notes (one line per check)
- advisor.md (origin/main, c9bd268): severity is CLAUDE.md's; PR review step 4 counts "a security finding left open (at
  any severity; one not fixed in the PR, a STATUS follow-up included, counts as left open)" as needs-mustafa; Review
  budget: "A finding's severity is its reviewer's label; only you change it, in writing, with the reason."
- review-um4-security.md:21-55: SEC-UM4-1 Medium; S's fix names six variables (incl. GIT_NAMESPACE) for opsStep and
  the helper, plus GIT_CEILING_DIRECTORIES in the helper; :163-166 "keeps the PR needs-mustafa while open".
- sec-um4-1-probe.md: the probe (stand-in folders only) recorded "real HEAD tree: verdict.txt"; not re-run (read only).
- check.mjs:2685-2706 opsStep passes no env to either call (`run`'s env defaults to process.env, :1951);
  webStep (:2664) already takes `base = process.env` and passes it to each call: the injection form exists.
- check.test.mjs:1910-1917 recorder records `env` per call: an env pin needs no real process.env change.
- operator.test.mjs:151-188: helper :157 has no env; `git add .` stages deletions, which is why S's tree was verdict.txt.
- NEW, outside both reviews: ops/operator/redact-check.test.mjs:157-191 and :193-230 build `{ ...process.env, GIT_AUTHOR_*
  ... }` and run `git init`, `add`, `commit` (message naming a public address, :179), and :212-216 `checkout -b side`,
  `checkout main`, `merge`, in a mkdtemp folder. They run in the ops glob AND in STEPS.redact (check.mjs:2741-2746,
  `run` with process.env), which predates UM-4 (added d9845355/41e55653, 2026-11-06, U1). Reasoned from S's probe, not
  run: an absolute GIT_DIR would put those commits, a `side` branch and a checkout of `main` into the named repository.
- PRECEDENT, check.test.mjs:1497-1525 and results/u3c-kernel-check/test_siocoutq_root_check.py:22-46 (u3c review C2b
  F2): the same hazard met before; fixed by leaving out EVERY GIT_* variable in the test's own git, core.hooksPath an
  empty folder, commit.gpgsign=false, proven by a victim-repository test (GIT_DIR names a scratch victim; byte-for-byte
  unchanged). No one exported a real GIT_DIR for that proof.
- The ops tests' git calls are four, all local (operator.test.mjs:157, browser.test.mjs:109 `ls-files`,
  redact-check.test.mjs:167, :203; redact-check.mjs:291 inherits); wrappers.test.sh names git 0 times (grep -c).
- Repo owner uid 0 = this shell's uid 0 (`ls -ldn`, `id -u`): no ownership refusal expected once GIT_CONFIG_* is gone.
- history.md:8137 U8-FILLS (a), word for word: "The values filled in at Stop B and the `ops` step in the check
  script sit outside U8's stop. The two one-line security notes are built at that fill and count, so the stop becomes
  6,284." msg-2026-11-28.md:74: the ops step is "outside U8's own files"; u8-f1-brief.md:14-15: the stop counts
  numstat of the files U8 touches (ops/). So check.mjs and check.test.mjs lines are outside; ops/ lines count.
- history.md:7561-7593 NPT-TRIM: "One review round per unit ... A second round only for an open Critical or High."
- review-um4-nb2b-test.md:73-130: T-1 Medium (M1, M3 survive; fix shown in scratch), T-2 to T-5 Low; :96-97 the web
  step's W1 survives too (check.test.mjs:1985-1993, stand-in :1988 `out: "boom\n"`, prefix-only assert :1993).
- um4-plan.md:185-212: the round recorded; the commit message's 3-of-5 CORRECTION recorded, "History is not rewritten."
- Files U8 and U8-F1 changed (`git diff --name-only` of 05c63a8a and ddb06ffe against their first parents): ops/ and
  notes only; neither touched check.mjs, check.test.mjs or redact-check.test.mjs (its last commits: 07d5d313 and
  2a9498de, F26, 2026-11-22/23; not traced to a unit's count, so this ruling rests on 0 net, not on whose file it is).
- Node 22 (/opt/node22/bin/node, `grep -ao`): "node:test run() is being called recursively within a test file. skipping
  running files." runs when NODE_TEST_CONTEXT is set: a `node --test` child spawned from check.test.mjs (itself run by
  `node --test` in the self step) skips its files unless NODE_TEST_CONTEXT is removed from the child's env.
- The lane runner is /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/main-run.sh (no copy in the spike
  folder); .gate/um4/ holds absent.log, counted.log, light-base.log, ops-pass.log, commit-msg.txt.
- Read after the advisor tool's second check:
  - ops/operator/pre-push only reads refs from stdin, hashes the copy and pipes to `node "$COPY" --pre-push`. It makes
    no git call of its own.
  - redact-check.mjs's git (:290-292) is called only from stagedItems (:313-318: `diff --cached`, `show`) and
    prePushItems (:321-338: `rev-list`, `log`, `diff`, `diff-tree`, `show`), at :376-377. All of them only read.
  - The folder mode, which is the redact step's second run on HERE, walks the disk (walk, :279-287) and runs no git.
- advisor tool (consulted before the blocks): wildcard over the 15-name list (S's list names GIT_NAMESPACE, not in
  git's 15); (b) in place at 0 net; a victim test for (b), never vacuous; the redact step's wall named; the redact
  tests' effect reasoned, not probed. Taken below; my own reasons are given per item.

## 1. SEC-UM4-1's severity: APPROVE Medium, as labelled (DONE)
- Medium stands. The scale's Medium is "a real defect with a workaround or a small reach". The defect is real: S's probe
  on stand-in folders recorded "real HEAD tree: verdict.txt". The reach is small: no caller exports GIT_DIR today (this
  shell has none, names only; main-run.sh sets none, um4-plan.md:201-203). The damage can be undone: the commits are
  local first, history keeps every earlier tree, main is protected, and `watch` holds back code commits.
- Not High. High is "a wrong result or rule a player, Mustafa or a session would hit". A session hits this only under an
  exported absolute GIT_DIR (a git hook, `rebase -x`). No caller here does that. An explicit `save` would push such a
  commit, but to the lane's own branch, where a revert undoes it.
- The reach is wider than S wrote. This adds no new rating; I name it UM4R-1 (Medium, part of SEC-UM4-1):
  - redact-check.test.mjs:157-191 and :193-230 build their temporary repositories from `...process.env`.
  - They run in the ops glob and also in the redact step, which predates UM-4 (U1, 2026-11-06).
  - Under an absolute GIT_DIR they would commit into the named repository: a message naming a public address (:179),
    then a `side` branch, a checkout of `main` and a merge (:212-216).
  - This is reasoned from S's probe, not probed. The first test uses `git add a.md`, not `git add .`, so it adds a file
    rather than deleting the tree. The second test moves the named repository's HEAD.
  - The same class was met before: u3c review C2b F2, check.test.mjs:1497-1525. The spike knows the hazard, so this is
    a gap in applying a known fix, not a new kind of risk.

## 2. The fix's two halves: APPROVE (a) and (b), both in this fix pass (DONE)

### (a) check.mjs: the env opsStep passes, and which list
- The form:
  - One exported pure function returns a copy of a base env with every `GIT_*` key left out (the name is the
    builder's).
  - opsStep takes `base = process.env` as its third parameter, the form webStep already uses (check.mjs:2664).
  - Both runner calls get the filtered env: the Node tests and wrappers.test.sh.
  - STEPS.ops stays `(runner = run) => opsStep(runner)`.
  - Leave the keys out; never set one to `undefined`. That way the pin can test that a key is absent.
- The list is the `GIT_*` wildcard. It is neither git's 15 local names (`git rev-parse --local-env-vars`, git 2.43.0)
  nor S's six. Four reasons:
  - Neither list holds the other: S's six include GIT_NAMESPACE, which git's 15 lack.
  - The wildcard is the form this spike already reviewed and tests: u3c C2b F2, check.test.mjs:1506 and
    test_siocoutq_root_check.py:46.
  - The ops tests need no `GIT_*` variable. Their four git calls all work on local folders, and wrappers.test.sh has no
    git call.
  - GIT_CONFIG_COUNT and its KEY_n and VALUE_n pairs go too. They carry this container's git settings (names only, never
    read). The repository's owner is this shell's uid, so git will not refuse the folder for its owner.
- No GIT_CEILING_DIRECTORIES in opsStep. browser.test.mjs:109's `git ls-files` must find the repository from the spike
  folder.
- The stop rule: if the real ops run then fails on a git config, ownership, signing or hook error, stop and report.
  - The named fallback is the precedent's `-c core.hooksPath=<empty folder>` and `-c commit.gpgsign=false` in the
    failing helper (check.test.mjs:1508).
  - Never put the container's config back by guess.
- The redact step (check.mjs:2741-2746) gets no env change in this pass:
  - Its wall is (b) below: the source fix plus the victim test, which cover its runs and direct runs alike.
  - What is left under an exported GIT_DIR, once (b) lands, is reads.
    - redact-check.mjs's own git (:291, through main() in the tests and through the pre-push hook) honours GIT_DIR by
      design, since a pre-push hook must read the repository git names. Its calls only read (Notes).
    - The step's second run, the folder scan, runs no git.
    - So the only effect left is that the step's test run reads the wrong repository and fails: the safe direction.
      This is read from the code, not run.
  - Stripping it now would need runner injection, a pin and a mutant, for a step outside UM-4 and for a safe-direction
    FAIL only. Carried as UM4R-2 (Low; below).

### (b) The tests' own git helpers, and U8's stop
- The form: rewrite in place, one line per site, so each site is 0 net by the stop's own count (u8-f1-brief.md:13-15:
  the numstat added column less the removed column).
  - operator.test.mjs:157 adds `env:` to its options, set to process.env without `GIT_*`.
  - redact-check.test.mjs:161 and :197 replace `...process.env,` with the same filtered spread. The GIT_AUTHOR_* and
    GIT_COMMITTER_* lines after it stay.
- The fix log shows `git diff --numstat` reading `1 1` for operator.test.mjs and `2 2` for redact-check.test.mjs. If any
  site needs more, the builder stops before that edit, and OPS-GIT-ENV-B (Decisions needed) goes to Mustafa.
- No GIT_CEILING_DIRECTORIES, against S's suggestion:
  - Each helper's first call is `git init` in its own folder, so every later call finds that folder's own .git.
  - If init fails, execFileSync throws before any other call.
  - Leaving out `GIT_*` already closes the only route to another repository.
- Left as they are: the pre-push spawn (operator.test.mjs:162) and redact-check.mjs:291.
  - They only read. The hook makes no git call of its own, and redact-check's git is `diff --cached`, `show`,
    `rev-list`, `log`, `diff` and `diff-tree`, at :313-338 (Notes).
  - A hook must honour the GIT_DIR git exports to it.
- The stop: U8 stays at 6,282 lines (6,284 once Stop B's two notes are built), since every site is 0 net.
  - This rests on 0 net at all three sites, not on whose file each one is.
  - My reading: a line rewritten in place at 0 net does not pass his stop. It is his to reverse, so he is told in one
    line (For Mustafa).

### May UM-4 close on (a) alone? S's needs-mustafa claim
- S's claim holds. advisor.md (origin/main), PR review step 4, counts "a security finding left open (at any severity;
  one not fixed in the PR, a STATUS follow-up included, counts as left open)" as needs-mustafa. The Review budget
  repeats it: "a security finding left open, which keeps the PR needs-mustafa at any severity".
- So UM-4 may close on (a) alone only as a fallback. In that case (b) is SEC-UM4-1's open half on PR #40's closing list,
  and PR #40 is needs-mustafa for it.
- The ruling is to build both now: (b) costs no line against his stop and needs no decision. The fallback applies only
  if a site cannot stay 0 net and his answer to OPS-GIT-ENV-B has not come.

## 3. The proof bars: APPROVE as ruled here (DONE)

### Red first at the unit level, for new behaviour
- **The env pin, for (a).** In check.test.mjs, through the recorder (check.test.mjs:1910-1917, which records `env` per
  call):
  - It calls `opsStep(r.runner, () => true, base)`. `base` is a plain object: PATH, HOME, one non-git name, and
    GIT_DIR, GIT_WORK_TREE, GIT_INDEX_FILE, GIT_CONFIG_COUNT and GIT_EDITOR.
  - The stand-in git values name a path that does not exist, under os.tmpdir(), never ROOT's .git.
  - It asserts that both calls' env holds no `GIT_*` key and keeps the others. It also asserts the filter function
    directly.
  - It is red by its own name on the shipped check.mjs, then green.
  - It never writes process.env.
- **The victim test, for (b).** In check.test.mjs, in the form of :1497-1525:
  - A mkdtemp victim repository gets one commit, made by a git with no `GIT_*` env and an empty hooks folder.
  - The test runs the real ops/tests/operator.test.mjs and ops/operator/redact-check.test.mjs as
    `node --test --test-reporter=tap` children.
  - Each child's env is process.env without `GIT_*` and without NODE_TEST_CONTEXT, plus GIT_DIR and GIT_INDEX_FILE
    naming the victim. Those two variables exist only in that child env object.
  - It asserts that the victim is unchanged byte for byte (snapshot, .git included).
  - It asserts that the children's TAP output names the three temporary-repository tests, as `ok` or `not ok`, title
    for title:
    - operator.test.mjs:151's "pre-push: passes a clean push, refuses an address, a tampered copy and a missing copy";
    - redact-check.test.mjs:157's "--staged reads the index, and --pre-push reads the pushed commits and their
      messages";
    - redact-check.test.mjs:193's "--pre-push reads a merge commit's own changes".
  - A bare `# tests N > 0` is not enough. A file that dies at import still reports one test, and NODE_TEST_CONTEXT left
    in gives the "called recursively ... skipping running files" pass. Either way the victim would be untouched with
    nothing run.
  - It does not assert the children's exit codes. Their hook-reading tests honour GIT_DIR by design and may fail.
  - It is red by its own name on the shipped helpers.
  - Each of the three sites (operator.test.mjs:157, redact-check.test.mjs:161, :197), restored alone, turns it red. Log
    each one.
  - If, after the fix, only `.git/index` differs (a read-only refresh), the test may instead compare HEAD, every ref
    (`for-each-ref`), `ls-files -s` and the objects, and must say so in the log. Anything else that differs is red.

### Mutants, by name: each must turn a named test red
- M1: drop check.mjs:2696's exit guard. T-1's new Node stand-in kills it: exit 1, `# tests 104 / # pass 103 / # fail 1`,
  exact message.
- M3: drop :2700. T-1's new wrappers stand-in kills it: status 1 with a valid summary, exact message.
- W1: drop webStep's Node exit guard (check.mjs:2677). The web test's new stand-in kills it (check.test.mjs:1985-1993).
- M4: read `w.out` for the summary. T-2's case kills it: stderr noise after the summary in `out`.
- M10: accept the summary on any line. T-2's case kills it: the summary followed by `extra\n`.
- M8: OPS_BROWSER_MODULES set to ops/browser. T-4's path pin kills it.
- M-env: opsStep passes `base` unfiltered. The env pin kills it.
- M-wild: the filter keeps GIT_DIR (for example `k.startsWith("GIT_") && k !== "GIT_DIR"`). The env pin kills it.
- How mutants are run:
  - Each mutant runs on a copy, or is applied and then restored with each file's sha256 equal to the fix's afterwards.
  - Each output goes to a fully named file, for example
    /home/user/Shamble-Frontlines/.gate/um4-fix/m1.log (FULL-PATHS: no variable or wildcard in a written name).
  - The scratch mutate scripts stay text under the notes folder.

### Run bars, after the fix, in this shell
- One counted run: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/main-run.sh running
  `node check.mjs --only self,ops,redact,refs`, on a clean tree at the fix commit, with the diff hash tying the log to
  the commit (as 09c26ef2 did). Its bars:
  - self: all pass, at 85 plus the new tests, each named. The first line of every new test's title is listed in the log
    notes.
  - ops: still `104 of 104 tests passed; wrappers.test.sh: 315 ok, 0 failed`. This is the real step with `GIT_*` left
    out. If it fails on git, see 2(a)'s stop rule.
  - redact and refs: lines equal to .gate/um4/light-base.log's, apart from refs' tracked-file count if a file is added.
    None should be added.
- `node tools/gate.mjs --no-smoke`: PASS. CI's gate and smoke run on the push.
- `node .claude/scripts/repo-map-check.mjs`: PASS.
- A whole cold run (13 of 13) is not needed. No cargo step, wasm step, marks or web step code changes. The web step's
  change is test-only, and self covers it.
- Never export GIT_DIR or any git variable in a shell command, for a probe or a run. Git variables appear only inside
  the victim test's child env object, naming its mkdtemp victim. No end-to-end probe beyond that test is needed.

### The commit message
- It claims only what the logs show:
  - The env pin and the victim test were red first on the shipped code, by name (their logs).
  - T-1, T-2, T-4 and the web fix are proven by the mutants named above, not by "red first". Name each mutant and the
    test that killed it.
  - The ops/ sites are 0 net (numstat).
- It carries one correction sentence for the last commit: 8fb32bc8's "Five new tests, each red by its own name first"
  held for three of the five (zero count, last line, missing folder). The failing-part and call-order tests arrived
  green. History is not rewritten.

## 4. The rest, each taken now or carried (DONE)
- **T-1 (Medium), taken: APPROVE T's fix.** Rewrite check.test.mjs:2021-2030 with stand-ins that pass the neighbouring
  guards:
  - Node: `# tests 104\n# pass 103\n# fail 1\n`, status 1.
  - The wrappers: `wrappers.test.sh: 315 ok, 0 failed\n`, status 1.
  - The asserts check exact messages, `/^the ops step's node tests \(103 of 104 tests passed\) failed/` and
    `/^the ops step's wrappers test failed/`, and keep `calls.length === at + 1`.
  - The label stays Medium. A reader could weigh the commit message's sentence as evidence; that is fixed by item 3's
    correction sentence, not by a re-rating.
- **The web step's test (check.test.mjs:1985-1993), taken: APPROVE.** It is the same shape and the same file, outside
  the stop, and rides a push that is needed anyway.
  - Only the Node part's stand-in must change. npm ci's and tsc's guards have no later guard that could catch their
    failure in their place.
  - The new Node stand-in is `# tests 3\n# pass 2\n# fail 1\n`, status 1, and its message is asserted exactly.
  - W1 is its mutant.
- **T-2 (Low), taken: APPROVE.** Add two cases:
  - A summary on stdout, with stderr noise after it in `out`, passes.
  - A summary followed by `extra\n` on stdout fails with "its last line: extra".
  - These kill M4 and M10.
- **T-3 (Low), carried as a STATUS follow-up: APPROVE carry.**
  - It is forward only. Today there is no skip or todo form in the selected files (T's grep).
  - The web step reads counts the same way, so the rule ("# pass below # tests fails") belongs to both steps at once.
  - Today the counted bar ("N of 104") shows a skip to the reader.
- **T-4 (Low), taken: APPROVE.** Add one assert,
  `assert.equal(OPS_BROWSER_MODULES, join(HERE, "ops", "browser", "node_modules"))`, which kills M8. M7 (`present` as
  `() => true` in STEPS.ops) is not required. The real run's absent.log and ops-pass.log already prove both sides.
- **T-5 (Low, forward), carried to infra-engineer after UM-4's close: APPROVE carry.**
  - A CI job for check.test.mjs is a `.github/` change: risk tier 2, kit, and Mustafa's ask first.
  - It is out of this unit. It goes on the card's follow-ups with T's sizing (85 tests, about 3 s, no install).
- **SEC-UM4-2 (Low), taken: APPROVE.** dependency-check.md's UM-4 record gets S's line: "npm 10.9.9, node v22.23.4;
  registry not recorded at the install (the lock's sha512 integrity binds every tarball)". These are notes, outside the
  stop.
- **SEC-UM4-3 (Low), taken: APPROVE.**
  - OPS_MISSING prints the absolute prefix `join(HERE, "ops", "browser")`, as WEB does.
  - check.test.mjs:2058's pin moves with it: build the expected string from the same join, or escape it, never a
    hand-typed path.
  - It stays one line.
- **UM4H-1 (my Low), taken: APPROVE S's corrected clause word for word** (review-um4-security.md:136-138) in
  dependency-check.md's UM-4 record. It replaces my proposed "counted another way" wording.
- **UM4R-2 (Low, mine, new), carried as a STATUS follow-up.** The redact step (check.mjs:2741-2746) passes the whole
  env.
  - Under an exported GIT_DIR, once (b) lands, its test run would read the wrong repository and FAIL, the safe
    direction. Every git call left there only reads, and the folder scan runs none (Notes; read, not run).
  - Leave `GIT_*` out there too, pinned, when that step is next touched.
  - It is a robustness item, not a security finding: nothing is written and nothing passes wrongly. PR #40's class does
    not hinge on it either way. SEC-UM4-1 alone, if left open, would set that class.

## 5. A second round: not wanted. APPROVE one round (DONE)
- NPT-TRIM (history.md:7590): "One review round per unit ... A second round only for an open Critical or High."
  The round's findings are Critical 0, High 0, Medium 2 (SEC-UM4-1, T-1; UM4R-1 sits inside SEC-UM4-1) and Low 7
  (T-2 to T-5, SEC-UM4-2, SEC-UM4-3, UM4H-1), plus my UM4R-2.
- My head read of the fix commit is not a round (advisor.md, Review budget: "your review of every pushed head ... are
  not rounds"). Neither are the gate and CI.
- At that read I check each item above against its log, the 0-net numstat lines, and the victim test's red and green
  logs, and I re-read (a) and (b) against S's probe. If the read finds a Critical or High, a second round with S
  becomes due then. Otherwise UM-4 closes.

## 6. The fix pass's form and its size: APPROVE (DONE)
- The form: one commit on the lane's branch, as UM-4's fix pass. The main session builds it; no builder worktree is
  needed for this size. One push, then CI, then my head read.
  - Before any edit, um4-plan.md records the fix pass's file list and symbols. The list adds two files the plan does
    not name, ops/tests/operator.test.mjs and ops/operator/redact-check.test.mjs, along with the new exported filter
    function, opsStep's new `base` parameter and the new tests' titles.
  - This ruling approves that scope. The plan line is its record, so it needs no new plan check.
  - The checkpoint saves its steps.
  - Before the commit, the red logs and mutant logs sit under /home/user/Shamble-Frontlines/.gate/um4-fix/.
- The size by estimate, against U8-FILLS (a):
  - check.mjs: about +6 to 10. The filter function, opsStep's `base` and both calls, OPS_MISSING's absolute prefix.
  - check.test.mjs: about +45 to 60. The env pin about 10, the victim test about 25 to 30, T-1's rewrite about +4,
    T-2 about 8, T-4 1, the web fix about +2.
  - ops/ (operator.test.mjs, redact-check.test.mjs): 0 net, as 3 lines changed in place; the numstat lines are shown.
  - dependency-check.md: about +2 to 3 lines of notes.
- Against his words: his U8-FILLS (a) is "The values filled in at Stop B and the `ops` step in the check script sit
  outside U8's stop. The two one-line security notes are built at that fill and count, so the stop becomes 6,284."
  - check.mjs and check.test.mjs sit outside it: the ops step in the check script and its tests, which U8 and U8-F1
    never touched.
  - The ops/ lines are inside it at 0 net. U8 stays at 6,282 of 6,284, and the two notes' 2 lines keep their place.
  - The lines are counted exactly by `git diff --numstat` in the fix log and named in the report, not stopped on.

## The memory line: AGREE (DONE)
- The security seat's pattern goes to the card as a follow-up: "a test that runs git in a temporary folder with the
  inherited environment acts on the repository an absolute GIT_DIR names; check every temp-repository test for a
  `GIT_*`-free env".
- It should name both meetings of this hazard: u3c review C2b F2 (check.test.mjs:1497-1525) and SEC-UM4-1 with
  UM4R-1. The second shows the first fix's pattern was not carried to the other temporary-repository tests.
- The main session writes it into security-auditor's memory when the card's follow-ups are next done; the brief barred S
  from writing it.

## Verdicts, one line each
1. SEC-UM4-1: APPROVE Medium as labelled. The reach is wider (UM4R-1: redact-check.test.mjs's two temporary
   repositories, also run by the redact step since U1), with the same rating.
2. The fix: APPROVE both halves now.
   - (a) opsStep leaves out every `GIT_*` name (the u3c C2b F2 form), through `base` as webStep does. No ceiling.
   - (b) three in-place one-line rewrites (operator.test.mjs:157, redact-check.test.mjs:161, :197) at 0 net, so U8
     stays at 6,282 of 6,284.
   - Closing on (a) alone is a fallback only, and keeps PR #40 needs-mustafa (step 4, quoted). S's claim holds.
3. Proof: APPROVE as ruled.
   - The env pin and the victim test are red first by name.
   - The mutants are M1, M3, W1, M4, M10, M8, M-env and M-wild.
   - One counted run of `--only self,ops,redact,refs` through main-run.sh, plus the gate (--no-smoke) and the repo map.
   - The commit message carries the 3-of-5 correction.
4. The rest:
   - Taken: T-1, the web test, T-2, T-4, SEC-UM4-2, SEC-UM4-3, and UM4H-1 (S's clause).
   - Carried: T-3 (STATUS), T-5 (infra-engineer, after the close) and UM4R-2 (STATUS, Low).
5. Second round: not wanted (NPT-TRIM; no Critical or High). My head read of the fix commit is not a round.
6. Form: one fix-pass commit by the main session. check.mjs about +6 to 10 and check.test.mjs about +45 to 60 sit
   outside the stop; ops/ is 0 net; notes +2 to 3.
7. Memory line: AGREE, as a card follow-up naming both u3c C2b F2 and SEC-UM4-1.

## Decisions needed
- **OPS-GIT-ENV-B (only if it is raised; none is open now).** May U8's test files grow past your 6,284 stop to close a
  security finding?
  - The question in plain words: the fix for SEC-UM4-1 rewrites three lines in U8's tests in place, at 0 net, which
    fits your stop. Should the builder find that one of them needs an extra line, may U8's stop rise by that many
    lines, at most 3?
  - SEC-UM4-1 is the finding that a git setting a git hook exports would make the check's tests commit into the real
    repository.
  - Why it matters: without that half, a hand-run of those tests stays exposed. The finding then stays open on PR #40,
    which keeps it waiting for you.
  - Options:
    - (a) Yes, up to 3 lines past 6,284, counted and named.
    - (b) No. UM-4 closes on the check script's half alone, and the tests' half goes on PR #40's closing list as an
      open security finding.
  - Recommendation: (a). It is the same fix the spike already carries for an earlier test (u3c C2b F2), and it closes
    the finding completely.
  - Until you answer: nothing past 6,284 is built.

## For Mustafa
1. Nothing to act on now.
2. For your reading only: I read a line rewritten in place at 0 net (added less removed, your stop's own count) as not
   passing your 6,284 stop. If you read it otherwise, say so, and the three lines wait for OPS-GIT-ENV-B.

Status: COMPLETE 2026-11-27 20:36 UTC
