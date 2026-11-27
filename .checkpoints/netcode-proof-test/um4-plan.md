# UM-4: the `ops` step's own change, the plan (written 2026-11-27 06:2x UTC by the main session)
Status: DRAFT rev 0.2 for the advisor's check in the 10:50 UTC window (rev 0.2, 08:5x UTC: the advisor tool's points
folded in: a light base run first, npm's cleared form with no audit, the order after U7's builder worktree, the
node_modules facts, Q4). Nothing is built before that check, and the build also
waits on his U8-FILLS answer (msg-2026-11-28.md: whether UM-4's lines count against U8's 6,282 stop). The parts are
A's UM-4 (advisor-u7-merge.md:39-54), word for word in their order; the file and line numbers are today's (U7's merge
moved check.test.mjs's pin from :2785 to :2847).

## Why
u8-brief.md:524 closes U8 with "merge, then the `ops` step in check.mjs". Until it lands, every whole run keeps
choice 15 (i): U8's ops tests and wrappers.test.sh run beside `node check.mjs` in the same main-run.sh call. The step
puts them inside check.mjs, so a whole run cannot leave them out.

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
  (A's re-ruled form; ffcheck 08:4x). A UM-4 commit to them before that fast-forward makes the staged copies differ
  from the target, and git refuses it. So UM-4's commit waits until that worktree is fast-forwarded, or until A rules
  it stays (the form's fallback (b)).

## Steps (each its own call; on the lane, after U8-F1's merge and B's head read; the commit after U7's builder worktree)
0. **The light base run, at UM-4's H0** (its own save first): one main-run.sh call of check.mjs's self, marks, refs
   and redact steps only, as U8-F1's M0 (.gate/u8-f1-merge/light-base.log's form), into .gate/um4/light-base.log. Its
   counts are step 5's bars by B's C1 rule (advisor-u8-f1-merge.md:47-55).
1. **(a) Test first.** check.test.mjs:2847's list gains "ops" after "web". `node --test check.test.mjs` fails on that
   assert (red, recorded). Then check.mjs's STEPS gains `ops` after `web`:
   `node --test 'ops/**/*.test.mjs'`, then `bash ops/tests/wrappers.test.sh`, both in HERE, failing if either fails,
   summarised with nodeCounts and the wrappers' last line. The self tests pass (green, the count recorded).
2. **(d) A missing folder**, in the same edit: when ops/browser/node_modules is absent, the step fails with one line
   naming that install (UM-4's form, below). Its test passes a stand-in for the folder check (absent: fails with that
   line; present: runs the two commands), so no test touches the real folder. Red first, then green.
3. **(b) U8's fix 2, the real install.** First `sha256sum` of package.json and package-lock.json (the two above). Then
   `env npm_config_ignore_scripts=true npm ci --ignore-scripts --no-audit --no-fund --prefix
   spikes/netcode-proof/ops/browser`, its own call (the web step's cleared form, Facts). Then both sha256s unchanged and
   `git status --short` unchanged (the folder is a real one, so the root .gitignore's `node_modules/` covers it).
   npm's "added N packages" line is recorded; with `--no-audit` there is no audit line (U8's "4 moderate" stays
   security's reading of the same lock, security-u8-deps.md C3 (c)). It replaces the link form of choice 15 (i) for
   later runs; the kept install in agent-abf71d95fc07b3522 stays until PR #40's close.
4. **(c) The dependency record.** One note section in dependency-check.md (the install's form as run, both sha256s
   before and after, npm's "added N packages" line, and the audit line only if Q4's answer is U8's form), with no bare
   sha256 line: no manifest or lockfile changes, so DEP-HOLD holds nothing. The lock was cleared at U8; the command's
   flags are Q4's (U8's form, or the web step's flags on it), so whether a new security read is needed is Q4's too.
5. **The counted run:** one main-run.sh call, `env CARGO_INCREMENTAL=0 node check.mjs` alone: all 13 steps pass. The
   bars, by B's C1 rule: the `ops` step's counts equal U8-F1's M5 (104 of 104; "315 ok, 0 failed"); test, web and
   wasm (they read only spikes/ content) equal U8-F1's M5 strictly, provided `git diff --name-only ddb06ff HEAD --
   spikes/` lists only UM-4's two files (else named first); marks, refs and redact (items and findings) equal step 0's
   base run; self equals step 0's plus the new tests, each named. Any other difference is named; unexplained, it stops.
   Then `node tools/gate.mjs --no-smoke` PASS and the repo-map check OK.
6. **(e) Until it lands,** whole runs keep choice 15 (i) (ops tests and wrappers beside `node check.mjs`). After it,
   the step replaces them, and the plan forms that name choice 15 (i) say so in their next use.
7. **One commit** (check.mjs, check.test.mjs, dependency-check.md's section, this file's done lines), its message
   naming UM-4, no skip marker; pushed by a checkpoint save; CI on the pushed head; the advisor's read of the pushed
   head in the next window.

## Size (for U8-FILLS)
About 20-30 added lines by estimate: the step and its missing-folder line in check.mjs (about 12-18), the list entry
and two tests in check.test.mjs (about 10-14); dependency-check.md is notes. Counted exactly at step 5 by `git diff
--numstat`. If his answer counts UM-4 against the stop, the build stops before step 1 and the size goes to him.

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
(none yet: the plan's check first, at 10:50)
