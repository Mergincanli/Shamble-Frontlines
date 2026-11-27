# U8-F1's merge into the lane: the plan (M0-M9), written 2026-11-27 01:3x UTC by the main session
Status: DRAFT for its check by B a3de649cd25e474df in the 05:50 UTC window (launch-0550-msgs.md). Nothing runs before
it. Form: U7's M0-M9 (u7-merge-plan.md), with the head read's M3a (advisor-u7-head.md section 3) and U8-F1's terms. The
merge into the lane needs no go of his; PR #40 keeps its own. Order: this merge first, with the linked install (choice
15 (i)); the `ops` step change (UM-4) after it, plan first. The advisor tool read the outline (01:2x): its six points
are folded in; its read of the written plan (01:2x): the base run back to the four light steps (no check.mjs step calls
the CLI) and the CLI checked directly.

## Facts (read 01:1x-01:3x UTC on 11-27; HEAD 46c964f = origin, clean)
- wip/u8-f1-build.patch: sha256 4d6a248425aebb25b317cf2da363e00915e8f2b13eb6f26a6a259a5e0f3f7164, 23,152 B, 6 files,
  base 77ffc95 (U8-F1's builder's worktree .claude/worktrees/agent-a7fe1b9f436f36bb1, detached at 77ffc95), taken 01:1x
  by worktree_patch.py with a notes prefix matching nothing (its "u8-f1" prefix would have copied the base's older
  u8-f1-brief.md over the main folder's). `git apply --numstat --summary`: 6 modified, none created, no mode change;
  105 added, 12 removed (+93), all under spikes/netcode-proof/ops/:
  - operator/PROMPT-R1.md 8/2; runs/R1.md 13/1; runs/r1.sh 17/6; tests/r1plan.test.mjs 7/0; tests/r1sh.test.mjs 17/2;
    tests/s14.test.mjs 43/1 (= the builder's table in u8-f1-build.md's FIX PASS HAND-BACK).
  - No manifest, lockfile or package file; nothing under crates/, web/, marks/ or the judge.
  - `git apply --check --index` exits 0 on 46c964f (01:2x).
- The six files in the builder's worktree, sha256 (M4's bar): PROMPT-R1.md
  a3e9d53f22821217c06e3659e82248cd4903f8c8579e6f5ee41284783418fd35; R1.md
  f77e68316ae2c88d9723e25bc51236d76a45ca9f7acde377c6d02fb9d2044c33; r1.sh
  a6f1dded5c12d96380ff5aafd2190d080b030896102e300f1bad508cdc6f3a3a (unchanged by the fix pass: its first build's
  restore hash); r1plan.test.mjs 9bea42963c0e3b012d06bcaebe69cfcc8e84062e94c00c6ed709372ad3053ba7; r1sh.test.mjs
  768739fa910a48201fc049f0534bf6ebeea1728f4e0d5fce8d46cb6a936c9c37; s14.test.mjs
  768f349d9977098c2e05674063939b7a358fad10bfd656fb7628b2bc0c391865. All six are 644 on disk and 100644 in the lane's
  index.
- On the lane since 77ffc95: `git diff --stat 77ffc95 HEAD -- spikes/netcode-proof/ops/` prints nothing (01:00 and
  01:2x). NB2's merge (48efc9c) and U7's (6968215) came after the base and touch nothing under ops/.
- The notes: u8-f1-build.md (6220ff52c1292983c55a74ffc30a1c6a7701b8b89b8543b974931a61fba2e605, copied 01:1x; the main
  copy was its exact prefix), ending "Status: COMPLETE 2026-11-27T01:14:16Z". Its end checks ran on the worktree at
  77ffc95, without NB2's and U7's merges: the 8 browser-free ops test files 95 of 95; wrappers "315 ok, 0 failed";
  redact over the spike 421 items, 0 findings; `check.mjs --only refs,self,redact` 3 of 3. Its glob form there: 104
  tests, 1 fail (browser.test.mjs: no webdriverio in that worktree; u8-f1-build.md:252).
- Tests: the base's 92 browser-free + U8-F1's three (the ws_split test, the list test, the pin test) = 95; the fix pass
  adds none. With ops/browser's install linked, the whole ops glob counts 104 = U7's M5's 101 + 3.
- The install for M5: agent-abf71d95fc07b3522's ops/browser/node_modules (U8's first build's worktree; `test -d` OK
  01:2x). package.json 8370495dee2ed77e… and package-lock.json a0efce8e511a8865… are equal there and in the main folder
  (01:2x). Linked as at U7's merge (choice 15 (i)), removed before M7.
- No C1 and no M2b. U8-F1 changes nothing that save(), config.mjs or the judge reads, and runs no browser leg; M1's
  hash and M4's byte compare against the end-checked worktree tie the commit to the tree the builder checked.
- M3a kept for uniformity: the head read scoped it to patches that change tracked files under crates/, and this one
  changes none; here it is a no-cost refresh before `--index`.
- Disk: 7,802 MiB free (01:1x), under M0's 8,192: the part-clean first.
- The CLI: cli-target was cleaned 01:1x; cli-root is kept, and its `bin/wasm-bindgen --version` prints "wasm-bindgen
  0.2.130" (01:3x), the version web/build.mjs pins (:4, :24). No check.mjs step calls it: the web step runs npm ci when
  needed, tsc and web/test's node tests (check.mjs:2656-2679), and the wasm step builds npt-hash-wasm with cargo
  (:2242-2250, :2378). Only build.mjs uses it (Stop B's two dist builds), so this merge does not depend on it.
- The guard: no command below has a program in front of git or gh; main-run.sh's own git calls run inside the script
  (INPUT-RUN). If one asks or is refused, the step stops and the advisor rules.
- u8f1-m4-cmp.mjs (written 01:3x) is u7-m4-cmp.mjs with its header (:1-4), :9 (the worktree), :10 (the patch) and :32
  (6, not 43) changed: `git diff --no-index --stat` "7 insertions(+), 7 deletions(-)".

## Resume line (on the card from M0 until M9 is done; the done lines below say where it stands)
"U8F1-MERGE running (u8-f1-merge-plan.md, its done lines). If HEAD = H0 and `git diff --cached --name-only` lists
exactly the 6 paths, M4 is done: resume at the step after the last done line (M4c or M5). If HEAD^1 = H0, HEAD holds the
6 paths and HEAD is not on the remote, resume at M8's push (the card save). A failing step or bar is M6. Anything else:
the advisor rules."

## Steps
- **M0, preconditions:**
  - No agent or workflow on the lane; the lane lock free (`flock -n`); no cargo, rustc or npt process (`pgrep` exit 1);
    HEAD = origin, clean.
  - `git fetch origin main` in its own call: main still b9aad01. A code, CI or rules move stops here (MAIN-52, plan
    first); a docs-only move is noted, nothing more.
  - Free disk at least 8,192 MiB: the part-clean in the card's Tools form, then measure again; still under: stop, the
    advisor rules.
  - u8-report.md's U8-F1 section drafted (M8's items; the M5 and M7 lines to fill). `git diff --no-index` of
    u7-m4-cmp.mjs and u8f1-m4-cmp.mjs, its own call: only the header (:1-4), :9, :10 and :32.
  - The resume line onto the card by Edit (replacing lines). The saver stopped by TaskStop with its proof; H0 = HEAD
    after the save that carries M0's done line, named in the chat. The no-save window runs from there to M8's push: a
    Stop hook gets one line, no save.
  - `git diff --quiet 6968215 H0 -- spikes/`, its own call: exit 0 (nothing under spikes/ moved since U7's merge, so
    M5's bars below are U7's M5 counts).
  - The base run at H0: `bash .checkpoints/netcode-proof-test/main-run.sh
    /home/user/Shamble-Frontlines/.gate/u8-f1-merge/light-base.log node check.mjs --only refs,marks,self,redact`
    (run_in_background, timeout 7200000): N0 (refs; 2,139 expected), the marks line, self, redact items (464). A
    failure stops the merge here: nothing is staged; the saver restarted; the log to the advisor.
- **M1:** the patch's sha256 is 4d6a248425aebb25b317cf2da363e00915e8f2b13eb6f26a6a259a5e0f3f7164.
- **M2:** `git diff --quiet 77ffc95 H0 --` the 6 paths: exit 0 (each unchanged on the lane since 77ffc95).
- **M3a (its own call, right before M3; advisor-u7-head.md section 3, word for word):** "`git update-index --refresh`.
  main-run.sh:48-52 touches every file under crates/, so a run through it leaves the index's cached times stale, and
  `git apply --index` refuses such a path without reading its content. The bar: exit 0, or exit 1 whose 'needs update'
  lines name only paths under .checkpoints/. Then each alone: `git diff --quiet -- spikes/` (tree = index) and `git diff
  --quiet HEAD -- spikes/` (tree = H0), both exit 0. A refusal at M3 after this is a failing bar (M6)."
- **M3 and M4:** `git apply --check --index`, then `git apply --index`. `git status --short` lists exactly the 6 paths
  staged (6 M) and notes only. `node .checkpoints/netcode-proof-test/u8f1-m4-cmp.mjs`: "6 paths in the patch; 6
  byte-equal with mode 644 on both sides; 0 not" (exit 0). `git ls-files -s` of the 6: all 100644. `sha256sum` of the
  6 in the main folder: Facts' six.
- **M4c, the link:** package.json and package-lock.json equal in agent-abf71d95fc07b3522 and in the main folder (Facts'
  hashes); ops/browser/node_modules absent in the main folder; then `ln -s` of the kept install's into the main
  folder's ops/browser/.
- **M5:** one main-run.sh call, cheap runs first: `bash .checkpoints/netcode-proof-test/main-run.sh
  /home/user/Shamble-Frontlines/.gate/u8-f1-merge/m5-check.log node --test 'ops/**/*.test.mjs' :: bash
  ops/tests/wrappers.test.sh :: env CARGO_INCREMENTAL=0 node check.mjs` (run_in_background, timeout 7200000). All
  pass:
  - ops: 104 tests, 104 pass, 0 fail (U7's M5's 101 + U8-F1's three);
  - wrappers: "315 ok, 0 failed";
  - check.mjs: all 12 steps, each count equal to U7's M5 (u7-merge-plan.md's M5 done line): test 931 measured and
    1,028 bites passed, 0 failed, 0 skipped; web 70 of 70; self 80 of 80; marks 81, rows unchanged; redact 11 of 11,
    464 items, 0 findings; refs 2,139 (= N0: no new file); wasm 27,000 ticks native-native and native-Wasm, 0
    mismatching; deps' held rules. A count that differs is named; unless U8-F1's six ops/ files explain it, M6.
- **M6:** a failing test or bar is never retried unnamed. It stops the merge, by advisor-u7-head.md section 3's words:
  "The undo starts with `git update-index --refresh` (the same bar) and `git diff --quiet -- spikes/`, then `git apply
  -R --index` of the patch, then `git diff --quiet H0 -- spikes/`." Then the link removed and the saver restarted; the
  failure and its log go to the advisor in the next window. Nothing is committed.
- **M7:** `unlink` of ops/browser/node_modules (the install stays in its worktree: `test -d`); `node tools/gate.mjs
  --no-smoke` PASS; `node .claude/scripts/repo-map-check.mjs` OK; `git status --short`: the 6 paths staged and notes
  only.
- **M8, one commit:** `git add` of u8-report.md and this file; `git rm` of wip/u8-f1-build.patch (WIP-CLEAN). Under
  spikes/ the commit equals the snapshot (`git diff --cached --numstat H0 -- spikes/` = the patch's numstat, line for
  line). The commit's message names U8-F1 and carries no skip marker. u8-report.md's U8-F1 section:
  - what U8-F1 built (its brief's items on the six files) and the fix pass (SEC-U8F1-1's (a)-(c), SEC-U8F1-2's parts,
    item 6's save wording);
  - the size: +93 net (105 added, 12 removed) of 93; U8 at 6,282 of his 6,282 stop (U8F1-SIZE (a));
  - S14-VMS: the advisor's ruling (inside his "the same redaction check"; his to reverse, choice 25);
  - the carries: SEC-U8F1-3's "in the same PowerShell" wording and SEC-U8F1-4's Stop B record (their home by the
    checker's Q2), logq's non-object line (a STATUS follow-up), and the earlier Lows (re-runs keep the last attempt's
    prints only);
  - the proofs by pointer (u8-f1-build.md's FIX PASS HAND-BACK: each item's red, plant and restore hash; item 5's
    scratch proof with private stand-in addresses, since the fixture's documentation addresses are allowed anyway);
  - M5's and M7's lines.
  Pushed by a checkpoint save with the card updated.
- **M9, after the push:** the saver restarted; CI on the pushed head (gate with its gate and smoke jobs, toolchain; push
  and pull request), all green; B's read of the pushed head in the next window (not a round; advisor-u8-f1-sec.md
  section 1). After that read, U8-F1's builder's worktree comes off by the re-ruled form for U7's builder
  (advisor-u7-head.md, "Item 6, re-ruled"): its notes on the lane at the same paths, then the builder, resumed in a
  window, `git add`s the ADDED and differing paths and runs `git merge --ff-only`, and `git status --short` prints
  nothing; any refusal: it stays until PR #40's close. Never `--force`, never `rm -rf`.

## Risks
- M5's whole run is cold after the part-clean: about 20-45 minutes (U7's: 21 minutes).
- The builder's end checks ran at 77ffc95, so M5 is the first run of U8-F1's tests against U7's and NB2's merged
  trees: a red there is M6, never a retry.
- #73's rules: a refusal or an ask stops the step where it is (M6's form if after M4).

## Questions for the checker
- Q1: M5's bars as U7's M5 counts, given `git diff --quiet 6968215 H0 -- spikes/` exit 0, instead of a fresh base for
  check.mjs's counts; the base run kept for N0 and for the CLI check. Right, or should the base run give every bar?
- Q2: the two carried Lows (SEC-U8F1-3's wording; SEC-U8F1-4's Stop B record) each need a new line, and U8 sits at
  exactly 6,282, so "taken at Stop B's fills" would pass his stop. Both are security findings, and PR #40's closing
  list says units add theirs as they merge. Do they go on that list now, or only if still open after Stop B (as SR-1
  and SR-2 were)? Does building them at Stop B's fill count against U8's stop?
- Q3: no C1 and no M2b, and M3a kept for uniformity: agree?
- Q4: u8-report.md's U8-F1 section (M8's list): anything missing?

## Done lines (one per step, by Edit)
(none yet: M0 not started)
