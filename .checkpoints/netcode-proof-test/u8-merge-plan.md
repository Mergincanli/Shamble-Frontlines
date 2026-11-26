# U8's merge into the lane: the plan (M0-M9), written 2026-11-25 20:2x UTC by the main session
Form: U4f's M1-M9 (advisor-u4f-r1.md:325-358) with U8's terms. Inside it, before the commit: C1 of the U7 brief check
(advisor-u7-brief.md:118-119, "Before U8 merges ... the log must read (a verdict, never a read error)"), the `ops` step
(u8-brief.md:333-335, :524) and the merged-state run (u8-build.md:594, :628). Allowed now: MAIN-66's check, Q1 yes after
step 6 (advisor-main-66.md; CI green on 69d9b5f at 19:42). No agent before the 23:50 window; there the advisor
(a3de649cd25e474df, "I read each pushed head", advisor-u8-stopb.md:224) reads the pushed head. The merge into the lane
needs no go of his (as U4r's and U4f's); PR #40 keeps its own go.

## Facts (read 20:1x UTC, HEAD beed8b1 = origin, clean)
- wip/u8-build.patch: sha256 09cb4a07928cf2a48c8516df18d68fd45e0b22a6666e04b65610934a9a72e660, 43 files, all under
  spikes/netcode-proof/ops/. None is a manifest or lockfile (ops/browser's package.json and package-lock.json are on the
  branch since the DEP-HOLD save and stay as they are); none is marks.json or under crates/judge (D10).
- U8's base 096808b. Since then the lane changed, under ops/, only ops/operator/redact-check.test.mjs (not one of the 43);
  outside ops/, U6b, U6c, 0b and MAIN-66 changed crates (server and super main.rs among them: test 7 reads them).
- `git apply --check` exit 0 on beed8b1 (20:1x), all 43 "Checking patch" lines, no error.
- U8's tests need ops/browser/node_modules (browser.test.mjs:136-152 runs the scripts, which import webdriverio). The
  installed copy is in agent-abf71d95fc07b3522 (U8's first build worktree; `npm ci --ignore-scripts`, u8-build.md:155),
  linked for the run as the builder did, removed after. git does not ignore that path; redact-check skips node_modules
  (redact-check.mjs:277); refs and test 4 read `git ls-files` only.
- C1's input: U7's T5 (ii) run 3 (clean), leg 6-mds: `.claude/worktrees/agent-a9873cdba07a3219e/.gate/u7/t5ii/
  2026-11-25T19-53-08-358Z/6-mds/result.json`, its `pages[0]` (records, nptClockResNs, nptDropped, version, status).
  PM-0001-36 (marks.json:7324) has the part "ML* (rough, reported)" (suite mds, machine.name ML*, min 1): the name
  legHeader writes (config.mjs:148). Its other parts (ML, MAC-b) stay unmet: any verdict is the bar, no read error.
- The judge: .gate/shared-target/debug/npt-judge (built 19:33 by MAIN-66's 5d on 6a4de15; crates/judge unchanged since);
  flags --marks, --logs, --out, --only (crates/judge/src/main.rs:35-42). r1.sh:797 treats a non-zero exit as a read
  failure.
- check.mjs has 11 STEPS (check.mjs:2614-2649), pinned by check.test.mjs:2785.

## The advisor tool's six fixes (20:2x UTC), folded in below
1. `git apply --index`: the 43 paths are staged, so `refs` and browser.test.mjs:109's `git ls-files` see the new
   files; M6's undo is `git apply -R --index`.
2. webdriverio for every later whole run: a real install in the main folder by U8's one install form
   (`npm_config_ignore_scripts=true npm ci --ignore-scripts` in ops/browser: u8-brief.md:28; a real folder, so the
   root .gitignore's `node_modules/` covers it; npm ci checks each package against the lock's sha512 and never writes
   the lock). No link. When the folder is absent the `ops` step fails with one line naming that install; U7's
   builder's new-base message carries the same line for its worktree, and NB2's whole run is in the main folder.
3. C1: `pages[0]` read (flat: 14 records, nptClockResNs 100000, nptDropped 0, version "Chromium 153", no timed_out);
   the script makes RECORDS first (save() makes no folder) and passes only the fields legHeader and legLog read. The
   judge exits 0 whatever the verdicts and 1 on a read error with an "npt-judge: " line (main.rs:99-123), so the bar is
   a PM-0001-36 entry with the ML* part counting 1 and no "npt-judge: " line. It runs through main-run.sh as `cargo
   run -q --locked -p npt-judge --` (the lane lock; the binary built from this tree).
4. Q1 (advisor-main-66.md:125-140) lifts only MAIN-66's hold; U8's own ruling and M3's apply check govern U8's
   steps, and the `ops` step is the approved brief's (u8-brief.md:333-335, :524: "merge, then the `ops` step").
5. The order (changed 23:2x, the advisor tool): U8 last, after the 0b amendment, U7's builder's new base (by 00:20
   UTC) and MAIN-67 (main moved at 20:50 by a CI change: M0 stops on it). U7's tree (wip/u7-build-rs.patch) changes
   check.mjs and check.test.mjs too, so the builder's new base does not carry U8, and the `ops` step's two edits meet
   U7's at U7's merge (its apply check). A choice inside the plan (choice 14).
6. The card carries this plan's pointer and a resume line, saved before M0; each step gets a "done" line here.
- Also: HEAD = origin re-checked after the saver stops; M3 compares modes too; ops' count is derived, not pinned
  (the glob also takes redact-check.test.mjs, the one ops file the lane changed: 11 top-level tests at 096808b and
  now, so 100 is expected and a difference is named); u8-report.md drafted before M0; the report says C1's log is a
  form test (a container loopback leg stamped R1, eu-path and ML*, kept only under .gate/), from U7's T-builder tree
  (7e2d6f3 plus its unsaved R-half, run 3's t5-dist). M5's command is U6c's form: `bash main-run.sh <log> cat
  /proc/loadavg :: env CARGO_INCREMENTAL=0 node check.mjs :: cat /proc/loadavg` (u6c-merge/m5-check.log, 20.4 min).

## Changes before M0 (2026-11-26 00:3x UTC, the advisor tool's second read; they win over the steps below)
- M0's main is now 07d673b (MAIN-67 merged it at 07cc9b7, pushed e23c783); M0 also waits for CI on e23c783 green
  (gate done; toolchain was still running at 00:3x) and PR #40 mergeable.
- The `ops` step waits until after U7's merge (u8-brief.md:524, "merge, then the `ops` step"): U7's tree changes the
  same pin (check.test.mjs:2785 gains "web" in wip/u7-build-rs.patch), so M4b tonight would break U7's apply check.
  M5 tonight is `node check.mjs` (the 11 steps) plus `node --test 'ops/**/*.test.mjs'` and `bash
  ops/tests/wrappers.test.sh` in the same main-run.sh call. No install tonight: ops/browser/node_modules linked from
  agent-abf71d95fc07b3522 as U8's builder did, removed before M7. Choice 15 for the day message.
- M5's bars: a light base run at H0 first (as MAIN-67's), then refs = N0 + 43 (all 43 of U8's files are new), marks,
  self and redact items as counted then (redact's item count rises by U8's text files); test, ops 100 pass expected.
- The part-clean runs before M0 (free 7,207 MiB at 00:26, under 8,192), in the card's Tools form character for
  character; if the guard asks or refuses, stop with nothing staged and one line for him.
- A "done" line goes here after each step.
- DONE 00:3x: the part-clean, in the card's form (no ask): "Removed 6564 files, 7.3GiB total"; the lock and CACHEDIR.TAG
  there, no debug folder; free 7,207 to 14,039 MiB. M5 will build cold.
- DONE 00:3x: MAIN-67's step 6 (CI on e23c783 all green: gate 45728829441 and 45728836685, toolchain 45728829428 and
  45728836660); M0's checks (no agent runs, no cargo or rustc, main still 07d673b, branch = origin and clean, free 14,039
  MiB); M1 (the patch 09cb4a07...e660); ops/browser's package.json 8370495d... and package-lock.json a0efce8e... equal in
  agent-abf71d95fc07b3522. Next: the saver stopped, H0, the base run, then M3.

## Steps
- **M0, preconditions:** no agent or workflow running; the lane lock free; HEAD = origin, clean; `git fetch origin
  main`: main still c1da1bd (a code, CI or rules move stops here: MAIN-52, plan first; a docs-only move is noted, nothing
  more); free disk at least 8,192 MiB. Stop the saver bu59it97m by TaskStop (its output ends "watch stopped"); H0 = HEAD.
  The no-save window runs from here to M8's push: a Stop hook gets one line, no save.
- **M1:** the patch's sha256 is 09cb4a07…e660.
- **M2:** `git diff --quiet 096808b H0 --` the 43 paths: exit 0.
- **M3 and M4:** `git apply --check --index`, then `git apply --index`. `git status --short` lists exactly the 43
  paths, staged; each is cmp-equal to its copy in U8's worktree (agent-ac59cce3e5d002441, the C9 tree the snapshot was
  taken from), with the same mode (`stat -c %a`).
- **M4b, the `ops` step, test first:** check.test.mjs:2785's list gains "ops" at the end (Edit); `node --test
  check.test.mjs` fails on that assert; then check.mjs's STEPS gains `ops` after `redact` (Edit, about 10 lines):
  `node --test 'ops/**/*.test.mjs'` (the glob form: u8-build.md:46), then `bash ops/tests/wrappers.test.sh`, both in
  HERE, failing if either fails, summarised with nodeCounts and the wrappers' last line; then the self tests pass.
- **M4c:** the install (fix 2): package.json 8370495d… and package-lock.json a0efce8e… checked first; then
  `npm_config_ignore_scripts=true npm ci --ignore-scripts --prefix spikes/netcode-proof/ops/browser`; its "added N
  packages" line kept; both files' sha256 unchanged after; `git status --short` unchanged (the folder is ignored). An
  install that fails stops here (M6's undo), to the advisor.
- **C1:** `node .checkpoints/netcode-proof-test/u8-c1.mjs` (Write, in the notes folder): it imports `save` from the
  applied ops/browser/lib.mjs and `logDate` and `ROOMS` from config.mjs; reads leg 6-mds's `pages[0]`; calls
  `save(run, leg, state, summary)` with run = { run: "R1", date: logDate(new Date()), commit: H0, wasm_sha256: the
  sha256 of the t5-dist's npt_client_wasm_bg.wasm the leg loaded }, leg = { suite: "mds", browser: "chrome", transport:
  "wt", nn: ROOMS.mds.nn, attempt: 0, slot: 0 } (mds36.mjs:31's form) and a summary naming the check; then copies that
  one .jsonl into a fresh .gate/u8-merge/c1-logs/ and prints both paths and the log's sha256. Then `bash main-run.sh
  .gate/u8-merge/c1-judge.log cargo run -q --locked -p npt-judge -- --marks marks/marks.json --logs
  <M>/.gate/u8-merge/c1-logs --out <M>/.gate/u8-merge/c1-verdicts.json --only PM-0001-36`. Bar (fix 3): a
  PM-0001-36 entry with a verdict, its "ML* (rough, reported)" part counting 1 log, and no "npt-judge: " line. A read
  error is M6.
- **M5:** one counted `node check.mjs` run through main-run.sh (lane lock, incremental off): all 12 steps pass. Counts
  recorded against MAIN-66's 5d (refs 1921 plus the new tracked files; marks 81, rows unchanged; self 78 of 78; redact 11
  of 11, items 378 plus U8's; ops 100 pass, 0 fail and wrappers 315 ok, 0 failed, as C9's end): test 7's merged-state
  run is inside it (it reads the post-U6c server and super main.rs).
- **M6:** a failing test or bar is never retried unnamed. It stops the merge: `git apply -R --index` of the patch,
  the two check files restored by Edit and sha256-checked, the saver restarted; the failure and its log go to the
  advisor in the 23:50 window. Nothing is committed. The install stays (ignored, the cleared lock).
- **M7:** `node tools/gate.mjs --no-smoke` PASS and `node .claude/scripts/repo-map-check.mjs` OK; `git status --short`
  shows the 43 paths, the two check files and the notes only.
- **M8, one commit** (`git add` of exactly those paths; `git rm` of wip/u8-build.patch, WIP-CLEAN), pushed by a
  checkpoint save with the card updated: the 43 files; check.mjs and check.test.mjs; u8-build.md's merge section;
  u8-report.md (files, tests and plants, size, carries by number, the dependency record: u8-brief.md:524), with the C1
  line, the D10 line ("no marks.json or judge change"), and what stays open at its merge (the card's U8 item: F28,
  U6b's null certificate pair, ds-selftest's place in C1's run list, U6c's names, Stop B's laptop reads, `sudo -l` on
  RS, SEC-SB-9's rest, NC-R3-1).
- **M9, after the push:** the saver restarted; CI on the pushed head (gate with gate and smoke, toolchain; push and
  pull request); the advisor's read of the pushed head at 23:50 with: the policy-name INVALID (plan.md:1256), C9 on
  096808b, test 7's merged-state run, C1's result, NC-R3-1, the open items, and the 05:2x clean incident
  (u7-block0b.md:40).

## Risks
- Test 7 fails on U6c's or U6b's names (the alignments were built against briefs): M6, to the advisor.
- The judge refuses the saved log (C1's purpose): M6; U8 does not merge until the advisor rules.
- The lane: M5 takes it about 20-45 minutes; U7's builder waits for the amended base in the 23:50 window, so the lane is
  free until then. If M5 is not done by 23:30, it stops at M6's form and moves after the window.
