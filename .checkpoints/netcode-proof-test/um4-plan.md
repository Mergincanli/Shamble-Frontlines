# UM-4: the `ops` step's own change, the plan (written 2026-11-27 06:2x UTC by the main session)
Status: rev 0.3, APPROVED WITH CONDITIONS UM4-1 to UM4-8 (advisor-um4-plan.md, A2 a0530735048f1620a, 11:15 UTC),
all applied word for word by Edit 11:2x UTC; Q1-Q4 answered there (Q4: the web step's form; npm ci's built-in audit
falls under "npm audit, never retried in any wording"). C ran 11:09 (its result read) and U7's builder worktree was
removed 11:1x. His FULL-PATHS rule (11-27 12:37 UTC, history.md:8155) holds at the build: steps 0, 2b and 5
write /home/user/Shamble-Frontlines/.gate/um4/light-base.log, /home/user/Shamble-Frontlines/.gate/um4/absent.log and
the counted run's log by their full paths, the folder by `mkdir -p /home/user/Shamble-Frontlines/.gate/um4`. (Rev 0.2, 08:5x UTC: the advisor tool's points folded in: a light base run first, npm's cleared form
with no audit, the order after U7's builder worktree, the node_modules facts, Q4.) His U8-FILLS answer came 12:25 UTC: (a), UM-4 outside U8's stop
(history.md:8133-8151); the build runs after the 15:50 window's three seats finish (the advisor tool, 12:3x). It was:
nothing built before his U8-FILLS answer (msg-2026-11-28.md: whether UM-4's lines count against U8's 6,282 stop). The parts are
A's UM-4 (advisor-u7-merge.md:39-54), word for word in their order; the file and line numbers are today's (U7's merge
moved check.test.mjs's pin from :2785 to :2847).

## Why
u8-brief.md:524 closes U8 with "merge, then the `ops` step in check.mjs". Until it lands, every whole run keeps
choice 15 (i): U8's ops tests and wrappers.test.sh run beside `node check.mjs` in the same main-run.sh call. The step
puts them inside check.mjs, so a whole run cannot leave them out.

## Protocol lines (UM4-8, word for word)
- Change type: logic in the spike's check script. Evidence: red-then-green Node tests on the real code, the real
  missing-folder run (2b), the counted run.
- Risk tier 1.
- Mustafa's Brave check: none (nothing a player sees, hears or feels).
- Reviewers, one round on the final diff, in the window of the head read (LEAN-MODE's new-agent rule), each at its
  agent file's model and effort:
  - test-engineer: the step, its tests and 2b's log;
  - security-auditor, a short read: the step's two commands, and step 3's install record against its own clearances
    (security-u7-deps.md items (3)-(4) and SEC-U7D-3; security-u8-deps.md C2 and C3).

## Facts (read 06:2x UTC on 11-27, H0 833c3f0 plus U8-F1's staged six files)
- check.test.mjs:2847 pins the step list: `assert.deepEqual(Object.keys(STEPS), ["fmt", "clippy", "clippy-wasm32",
  "doc", "test", "deps", "wasm", "refs", "marks", "self", "web", "redact"]);`.
- check.mjs's STEPS (about :2690-2720): `web: (runner = run) => webStep(runner),` then `redact`; `nodeCounts` at
  :1984 and `summarized` and `lastLines` beside it.
- The two commands, as M5 runs them from spikes/netcode-proof: `node --test 'ops/**/*.test.mjs'` (Node expands the
  glob itself; `node --test ops/` fails on Node 22.23: u8-report.md) and `bash ops/tests/wrappers.test.sh` (its last
  line "wrappers.test.sh: 315 ok, 0 failed").
- ops/browser's install: package.json 8370495dee2ed77e… and package-lock.json a0efce8e511a8865… (webdriverio 10.2.0
  exact), cleared at U8 (dependency-check.md; security-u8-deps.md C3 (c): "4 moderate" is the expected chain).
  browser.test.mjs needs webdriverio; without it the glob form has 1 failing test (u8-f1-build.md:252).
- CI does not run the spike's check.mjs: the gate job runs tools/gate.mjs, and the toolchain job builds
  packages/client (toolchain.yml:131-182). So the step cannot turn CI red, and CI needs no install.
- A real ops/browser/node_modules folder (in place of M5's link) changes neither the refs nor the redact count:
  redact-check walks folders but skips every folder named node_modules (redact-check.mjs:277, SKIP_DIRS), and the
  refs step lists files by `git ls-files -z` (check.mjs:2696), where the root .gitignore's `node_modules/` leaves it
  out. The kept install ships no `*.test.mjs` file (`find … -name "*.test.mjs"` printed nothing, 08:5x), so the
  `ops` glob matches nothing extra there; whether Node's glob would enter node_modules was not read (moot for this
  install; the counted run shows it).
- npm's form already cleared for this lane: the web step's `npm ci --ignore-scripts --no-audit --no-fund --prefix
  <dir>`, with `npm_config_ignore_scripts=true` in its environment (check.mjs:2656-2667; security-u7-deps.md items
  (3)-(4)). With `--no-audit`, npm sends no audit request at all.
- Order: check.mjs and check.test.mjs are two of the 43 paths U7's builder stages for its fast-forward to 0efbbef
  (A's re-ruled form; ffcheck 08:4x). C's target is the fixed commit 0efbbef9…, so no later lane commit changes what the
  builder fast-forwards to. UM-4's commit still waits until C's result is read (C was sent 11:0x on 11-27), so one
  thing moves at a time (UM4-6).

- Since rev 0.3: commit d8b57b8 (11:3x on 11-27, the operator rewordings, choice 31) changed three spike files after
  ddb06ff: `git diff --name-only ddb06ff HEAD -- spikes/` printed ops/operator/PROMPT-R1.md, ops/operator/rules.md and
  ops/tests/operator.test.mjs (11:3x). Step 5's proviso names them first, as its "else named first" asks.

## Steps (each its own call; on the lane, after U8-F1's merge and B's head read; the commit after U7's builder worktree)
0. **The preconditions, then the light base run, at UM-4's H0.** Step 0 opens with U8-F1's M0 preconditions
   (u8-f1-merge-plan.md:61-71), each in a printing form (exit-silent.md) (UM4-4):
   - no agent or workflow on the lane, the lane lock free, no cargo, rustc or npt process;
   - HEAD = origin, and the tree clean;
   - `git fetch origin main`, then `git rev-parse origin/main` = b9aad01… (a code, CI or rules move stops: MAIN-52);
     re-read 11-27 14:3x: main moved to b8f02b0 (#78, CI and docs), taken in by MAIN-69 (main-69-plan.md rev 0.2, its
     merge on c795079; C's APPROVE WITH CONDITIONS M69-1 to -6), so this bar reads b8f02b0149cc928964e54e194cd9c4d68f52bd94
     once MAIN-69's push is on the remote; any other value stops (MAIN-52). Step 5's proviso also names c16777e's file
     (PROMPT-R1.md:127-129, F1, net zero) beside d8b57b8's three; the part-clean DONE 14:53 UTC as prep (his 14:52
     message): `env CARGO_NET_OFFLINE=true flock -x …/.gate/shared-target/.lane.lock cargo clean --locked
     --manifest-path …/spikes/netcode-proof/Cargo.toml --target-dir …/.gate/shared-target --profile dev` (no cargo,
     rustc or npt process before it, `pgrep -c` 0): "Removed 5765 files, 7.1GiB total"; `ls -la` lists the lock and
     CACHEDIR.TAG, no debug; df 14,574 MiB free (8,037 before), so step 0's 8,192 bar holds and step 5's run is cold;
   - free disk at least 8,192 MiB;
   - the saver stopped by TaskStop with its proof, and the no-save window running from H0 to step 7's push.
   Step 0 runs at the build, after his U8-FILLS answer, the same day as step 5 (choice 28).
   Then the light base run (its own save first): one main-run.sh call of check.mjs's self, marks, refs
   and redact steps only, as U8-F1's M0 (.gate/u8-f1-merge/light-base.log's form), into .gate/um4/light-base.log. Its
   counts are step 5's bars by B's C1 rule (advisor-u8-f1-merge.md:47-55).
1. **(a) Test first.** check.test.mjs:2847's list gains "ops" after "web". `node --test check.test.mjs` fails on that
   assert (red, recorded). Then check.mjs's STEPS gains `ops` after `web`:
   `node --test 'ops/**/*.test.mjs'`, then `bash ops/tests/wrappers.test.sh`, both in HERE, failing if either fails,
   and failing when node's `# tests` count is 0, or when the wrappers' last line is not
   `wrappers.test.sh: N ok, 0 failed` with N above 0 (the web step's U7R-TE-4 rule, check.mjs:2677-2678); each case
   has its own test, red first (UM4-1). Both are summarised with nodeCounts and the wrappers' last line. The self
   tests pass (green, the count recorded). The step is
   `export function opsStep(runner = run, present = <the real folder check>)`, with
   `ops: (runner = run) => opsStep(runner)` in STEPS after `web`, as webStep (check.mjs:2663, :2713). Its tests pass
   a recording runner and a stand-in `present`, and pin both commands' program, arguments and folder (HERE), as
   check.test.mjs:1980-1981 pin npm's. check.mjs's header step list gains `ops` (UM4-2).
2. **(d) A missing folder**, in the same edit: when ops/browser/node_modules is absent, the step fails with one line
   naming that install (UM-4's form, below). Its test passes a stand-in for the folder check (absent: fails with that
   line; present: runs the two commands), so no test touches the real folder. Red first, then green.
2b. One main-run.sh call, `node check.mjs --only ops` (no cargo), into .gate/um4/absent.log: FAIL with step 2's one
   line naming the install. The main folder has no ops/browser/node_modules before step 3 (UM4-3).
3. **(b) U8's fix 2, the real install.** First `sha256sum` of package.json and package-lock.json (the two above). Then
   `env npm_config_ignore_scripts=true npm ci --ignore-scripts --no-audit --no-fund --prefix
   /home/user/Shamble-Frontlines/spikes/netcode-proof/ops/browser` (UM4-7), its own call (the web step's cleared form, Facts). Then both sha256s unchanged and
   `git status --short` unchanged (the folder is a real one, so the root .gitignore's `node_modules/` covers it).
   npm's "added N packages" line is recorded; with `--no-audit` there is no audit line (U8's "4 moderate" stays
   security's reading of the same lock, security-u8-deps.md C3 (c)). It replaces the link form of choice 15 (i) for
   later runs; the kept install in agent-abf71d95fc07b3522 stays until PR #40's close.
4. **(c) The dependency record.** One note section in dependency-check.md (the install's form as run, both sha256s
   before and after, npm's "added N packages" line; Q4's answer is the web step's form, so the note says: "npm audit not run
   (`--no-audit`, SEC-U7D-3); security's U8 reading of this lock stands (security-u8-deps.md C2, C3 (c))"), with no bare
   sha256 line: no manifest or lockfile changes, so DEP-HOLD holds nothing. The lock was cleared at U8; the command's
   flags are Q4's (U8's form, or the web step's flags on it), and Q4 rules no new security read before the install (security's read of the final diff is UM4-8's).
5. **The counted run:** one main-run.sh call, `env CARGO_INCREMENTAL=0 node check.mjs` alone: all 13 steps pass. The
   bars, by B's C1 rule: the `ops` step's counts equal U8-F1's M5 (104 of 104; "315 ok, 0 failed"); test, web and
   wasm (they read only spikes/ content) equal U8-F1's M5 strictly, provided `git diff --name-only ddb06ff HEAD -- spikes/` prints nothing (control:
   `git diff --name-only 6968215 ddb06ff -- spikes/` lists U8-F1's six) and
   `git status --short --untracked-files=all -- spikes/` lists only check.mjs and check.test.mjs, as modified (else
   named first) (UM4-5; named first in Facts: d8b57b8's three, so test and web are read against U8-F1's M5 with those
   three named, and the ops count must still be 104); marks, refs and redact (items and findings) equal step 0's
   base run; self equals step 0's plus the new tests, each named. Any other difference is named; unexplained, it stops.
   Then `node tools/gate.mjs --no-smoke` PASS and the repo-map check OK.
6. **(e) Until it lands,** whole runs keep choice 15 (i) (ops tests and wrappers beside `node check.mjs`). After it,
   the step replaces them, and the plan forms that name choice 15 (i) say so in their next use.
7. **One commit** (check.mjs, check.test.mjs, dependency-check.md's section, this file's done lines), its message
   naming UM-4, no skip marker; pushed by a checkpoint save; CI on the pushed head; the advisor's read of the pushed
   head in the next window.

## Done lines (the build, 11-27 from 15:58 UTC; the advisor tool 15:5x: H0 after B2's file COMPLETE, the four reads,
## a pass run of `--only ops` before the cold run, the resume line by printing reads, no agent inside the stretch)
- Reads before H0: ops/browser holds no node_modules (`ls -la`: cert14d, config, fallback, lib, mds36, package-lock,
  package, smoke; no link); A's (d) wording (advisor-u7-merge.md:53): "the step fails with one line naming that
  install"; `run` joins stdout then stderr into `out` and keeps `stdout` apart (check.mjs:1960-1962), so the wrappers'
  last line is read from `stdout`; check.test.mjs imports check.mjs's exports in one block (:12-79), so `opsStep` joins
  that import only once it exists (else the file fails to load and nothing goes red by name): the order is the list
  assert alone (red), then the plain two-command step with its import and plain test (green), then the zero-count,
  last-line and absent tests (red by name), then the guards (green).
- Step 0 (15:58-16:0x): H0 65b2f2965aac4a4c6316049a7bc8e25f9f085adc (the resume line's save; B2 COMPLETE 15:58 in it);
  ListAgents none; `pgrep -c -f "cargo|rustc|npt-|flock"` 0; HEAD = origin, clean; `git fetch origin main`, then
  `git rev-parse origin/main` b8f02b0149cc928964e54e194cd9c4d68f52bd94 (MAIN-69's tip); df 14,569 MiB; usage 0.12
  (15:58); the saver bwjsbqfnx stopped by TaskStop, `pgrep -c -f "checkpoint.mjs watch"` 0; `mkdir -p
  /home/user/Shamble-Frontlines/.gate/um4`; the light base run into /home/user/Shamble-Frontlines/.gate/um4/light-base.log.
- The base light run (to 15:59:38 UTC, exit 0): refs 2166 tracked files (N0); marks 81 read and checked, rows
  unchanged; self 80 of 80; redact 11 of 11 tests, "464 item(s) checked, 0 finding(s)". Step 5's bars by B's C1 rule.
- Steps 1-2, red by name then green, each `node --test` of check.test.mjs by absolute path:
  - A, the list assert alone ("ops" after "web" at :2847): 79 of 80, test 71 (it holds the assert) failing on
    `deepStrictEqual`, the diff "- 'ops'" at index 11, nothing else (red).
  - B, the plain two-command step (opsStep, OPS_BROWSER_MODULES, the STEPS line after `web`, the header's ops
    sentence), its import (between NOT_TICK_COUNTS and readBackCheck) and its plain test pinning both calls' program,
    arguments and folder (HERE) and the summary line: 81 of 81 (green).
  - C, four more tests: the failing-part guard (passes on B's step), zero count, the wrappers' last line, the missing
    folder: 82 of 85, failing exactly 58 (zero count), 59 (last line) and 60 (missing folder), each by its own name (red).
  - D, the three guards in opsStep (the install check first and running nothing, `# tests` above 0, the wrappers'
    stdout last line `wrappers.test.sh: N ok, 0 failed` with N above 0) and OPS_MISSING's one line naming the install:
    85 of 85 (green). Self: 80 + 5 new tests.
- Step 2b (16:02:31 UTC): `node check.mjs --only ops` through main-run.sh into
  /home/user/Shamble-Frontlines/.gate/um4/absent.log: "FAIL ops ... ops: ops/browser/node_modules is missing (UM-4
  (d)): install it first with `npm ci --ignore-scripts --no-audit --no-fund --prefix ops/browser`,
  npm_config_ignore_scripts=true in its environment", "(1 lines)", exit 1.
- Step 3: sha256 before: package.json 8370495d…3c90, package-lock.json a0efce8e…9524; `git status --short`: the card
  notes, check.mjs, check.test.mjs; `env npm_config_ignore_scripts=true npm ci --ignore-scripts --no-audit --no-fund
  --prefix /home/user/Shamble-Frontlines/spikes/netcode-proof/ops/browser`: "added 167 packages in 7s" (one
  deprecation notice, whatwg-encoding@3.1.1); both sha256 unchanged; `git status --short` unchanged; `ls -d` lists
  ops/browser/node_modules.
- The pass run (the advisor tool's addition, 16:03:17 UTC): `--only ops` into
  /home/user/Shamble-Frontlines/.gate/um4/ops-pass.log: "PASS ops 8.9 s ops: 104 of 104 tests passed; wrappers.test.sh:
  315 ok, 0 failed", exit 0 (U8-F1's M5 counts).
- Step 5 started 16:03 into /home/user/Shamble-Frontlines/.gate/um4/counted.log (cold: the part-clean of 14:53).
  Provisos: `git diff --name-only ddb06ff HEAD -- spikes/` prints d8b57b8's three (ops/operator/PROMPT-R1.md, rules.md,
  ops/tests/operator.test.mjs; F1's c16777e changed PROMPT-R1.md again), named first; `git status --short
  --untracked-files=all -- spikes/` lists only check.mjs and check.test.mjs, modified.
- Step 4 written while step 5 ran (notes only; nothing the run reads): dependency-check.md's UM-4 section.
- Step 5 (16:03-16:26:19 UTC, exit 0, cold): "check: all 13 steps pass". ops 104 of 104 and "wrappers.test.sh: 315 ok,
  0 failed" (U8-F1's M5); test 931 and 1,028, 0 failed, 0 skips (U8-F1's M5; d8b57b8's three named first); web 70 of 70;
  wasm 27,000 ticks, 0 mismatching, both bites caught as before; marks 81, rows unchanged; refs 2166; redact 11 of 11
  and 464 items, 0 findings (the three = step 0's base run); self 85 of 85 = the base's 80 + the five new tests (step
  1's C list); fmt, clippy, clippy-wasm32, doc and deps PASS. GATE PASS (`--no-smoke`, base b8f02b0); "repo map OK: 246
  entries cover 110 required paths". Size: check.mjs +29/-2, check.test.mjs +58/-1 (87 added; the estimate was 20-30:
  five tests, not two, and the guards' comments), outside U8's stop by his U8-FILLS (a).
- Step 7: one commit of check.mjs, check.test.mjs, dependency-check.md and this file (`git commit -F
  /home/user/Shamble-Frontlines/.gate/um4/commit-msg.txt`, no skip marker), pushed by the checkpoint save; then the
  saver's restart and CI on the pushed head; the review round (test-engineer, security-auditor's short read) and the
  advisor's read of the pushed head after the usage settles (the advisor tool, 15:5x).
- Step 7 DONE 16:37 UTC: 8fb32bc8 pushed by the save 787eb984 (no skip marker); CI on 787eb984 all four success: gate
  46034034986 (push) and 46034043528 (pull request), toolchain 46034034877 (push) and 46034043538 (pull request). No
  workflow names spikes/ or netcode-proof (grep over .github/: no match), so CI never runs check.mjs: the ops step is
  proven by step 5's local counted run, not by CI. Left: the review round (test-engineer: the step, its tests and 2b's
  log; security-auditor: the short read) and B's read of 787eb984, in one window of three seats after NB2-ONEWAY's push
  (the advisor tool, 16:3x), or before it if his reply has not come by 17:30 UTC.

## Size (for U8-FILLS)
About 20-30 added lines by estimate: the step and its missing-folder line in check.mjs (about 12-18), the list entry
and two tests in check.test.mjs (about 10-14); dependency-check.md is notes. Counted exactly at step 5 by `git diff
--numstat`. His U8-FILLS (a) (12:25 UTC) puts UM-4 outside U8's stop, so its lines are counted and named, not
stopped on.

## Risks
- The glob form runs browser.test.mjs, which needs the install: step 3 comes before step 5's counted run, and step
  2's line names the install when it is missing.
- npm ci needs the registry through the proxy; a refusal or an ask stops the step where it is (#73's rules); nothing
  is retried in another wording.
- Lines in check.mjs shift with any later merge: the plan's anchors are re-read at the build, by content, not number.

## Questions for the checker
- Q1: the missing-folder line's test by a stand-in for the folder check (no test touches the real folder): enough?
- Q2: the install's place, before the counted run and after the step's red and green: right order?
- Q3: anything in UM-4's (a)-(e) this plan leaves out or adds?
- Q4: two cleared forms exist. U8's for ops/browser (security-u8-deps.md C3 (a) and (c): `npm ci --ignore-scripts`,
  `npm_config_ignore_scripts` in the environment; npm's built-in audit then runs, and its expected output is exactly
  the 4 moderate lines) and the web step's (check.mjs:2666-2667, security-u7-deps.md (3)-(4): the same plus
  `--no-audit --no-fund`). Which form, and does `npm ci`'s built-in audit request fall under "npm audit, never retried
  in any wording"? Rev 0.2 takes the web step's flags on U8's command, so no audit runs and step 3 records no audit
  line.

## Done lines (one per step, by Edit)
(none yet: the plan is approved, UM4-1 to UM4-8 applied; the build waits on his U8-FILLS answer)
