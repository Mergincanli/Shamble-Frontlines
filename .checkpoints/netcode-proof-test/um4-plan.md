# UM-4: the `ops` step's own change, the plan (written 2026-11-27 06:2x UTC by the main session)
Status: DRAFT for the advisor's check in the 10:50 UTC window. Nothing is built before that check, and the build also
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

## Steps (each its own call; on the lane, after U8-F1's merge and B's head read)
1. **(a) Test first.** check.test.mjs:2847's list gains "ops" after "web". `node --test check.test.mjs` fails on that
   assert (red, recorded). Then check.mjs's STEPS gains `ops` after `web`:
   `node --test 'ops/**/*.test.mjs'`, then `bash ops/tests/wrappers.test.sh`, both in HERE, failing if either fails,
   summarised with nodeCounts and the wrappers' last line. The self tests pass (green, the count recorded).
2. **(d) A missing folder**, in the same edit: when ops/browser/node_modules is absent, the step fails with one line
   naming that install (UM-4's form, below). Its test passes a stand-in for the folder check (absent: fails with that
   line; present: runs the two commands), so no test touches the real folder. Red first, then green.
3. **(b) U8's fix 2, the real install.** First `sha256sum` of package.json and package-lock.json (the two above). Then
   `npm_config_ignore_scripts=true npm ci --ignore-scripts --prefix spikes/netcode-proof/ops/browser`, its own call.
   Then both sha256s unchanged and `git status --short` unchanged (the folder is a real one, so the root .gitignore's
   `node_modules/` covers it). npm's "added N packages" and its audit line are recorded; "4 moderate" is security's
   expected chain. It replaces the link form of choice 15 (i) for later runs; the kept install in
   agent-abf71d95fc07b3522 stays until PR #40's close.
4. **(c) The dependency record.** One note section in dependency-check.md (the install's form, both sha256s before
   and after, npm's two lines), with no bare sha256 line: no manifest or lockfile changes, so DEP-HOLD holds nothing,
   and no new security read is needed (the form and the lock were cleared at U8).
5. **The counted run:** one main-run.sh call, `env CARGO_INCREMENTAL=0 node check.mjs` alone: all 13 steps pass, the
   `ops` step's counts equal U8-F1's merge's M5 (104 of 104; "315 ok, 0 failed"), self one or two higher than 80
   (the new tests), every other count as that M5 or its base run. Then `node tools/gate.mjs --no-smoke` PASS and the
   repo-map check OK.
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

## Done lines (one per step, by Edit)
(none yet: the plan's check first, at 10:50)
