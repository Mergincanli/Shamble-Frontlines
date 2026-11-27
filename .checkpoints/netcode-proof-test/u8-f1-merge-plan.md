# U8-F1's merge into the lane: the plan (M0-M9), written 2026-11-27 01:3x UTC by the main session
Status: APPROVED WITH CONDITIONS C1-C3 by B a3de649cd25e474df (advisor-u8-f1-merge.md, 06:00 UTC on 11-27), each
applied by Edit word for word: C1 at M0 and M5, C2 and C3 at M8, its Q3 addition at M9. Nothing ran before it. Form: U7's M0-M9 (u7-merge-plan.md), with the head read's M3a (advisor-u7-head.md section 3) and U8-F1's terms. The
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
    M5's bars below that read only spikes/ are U7's M5 counts; the others are the base run's: C1).
  - The base run at H0: `bash .checkpoints/netcode-proof-test/main-run.sh
    /home/user/Shamble-Frontlines/.gate/u8-f1-merge/light-base.log node check.mjs --only refs,marks,self,redact`
    (run_in_background, timeout 7200000): N0 (refs; the count recorded as N0 (2,139 at U7's M5; notes since may move it)), the marks line, self, redact items (464). A
    failure stops the merge here: nothing is staged; the saver restarted; the log to the advisor. A count off its
    expectation (the advisor tool, 06:0x): refs not 2,139 or redact items not 464: recorded, and on (C1); self not 80
    or marks not 81: stop for the advisor (spikes/ is proven unchanged, so a move there means the environment or
    docs/adr/ moved); any step FAIL: stop, as written.
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
  - ops: 104 tests, 104 pass, 0 fail; wrappers: '315 ok, 0 failed'.
  - check.mjs: all 12 steps pass.
    - test, web and wasm: equal to U7's M5 (931 and 1,028 passed, 0 failed, 0 skipped; 70 of 70; 27,000 ticks, 0
      mismatching).
    - self, marks and refs: equal to M0's base run (refs = N0).
    - redact: 11 of 11 tests, 0 findings, and its items equal to the base run's count. A difference is named with the
      files on disk that explain it; an unexplained drop is M6.
  - Any other differing count is named; unless U8-F1's six ops/ files explain it, M6.
  (C1, advisor-u8-f1-merge.md:47-55, word for word; its table at :34-45 gives each figure's bar and reason.)
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
  - the carries: SEC-U8F1-3's "in the same PowerShell" wording and SEC-U8F1-4's Stop B record (C2: on PR #40's
    closing list now, as security findings left open; no fill and no building of them until he answers U8-FILLS), logq's non-object line (a STATUS follow-up), and the earlier Lows (re-runs keep the last attempt's
    prints only);
  - the proofs by pointer (u8-f1-build.md's FIX PASS HAND-BACK: each item's red, plant and restore hash; item 5's
    scratch proof with private stand-in addresses, since the fixture's documentation addresses are allowed anyway);
  - M5's and M7's lines.
  - C3 (advisor-u8-f1-merge.md:98-125, word for word): these lines too, each a pointer, no new paper:
    1. The rounds and rulings: the head read: advisor-u8-head.md, APPROVE WITH FINDINGS; ADV-U8H-1 to -4; SR-1 and
       SR-2 weighed Low; his RESULTS-PUSH (b) and S14-PUSH (a), at history.md:8068-8106; the brief check:
       advisor-u8-f1-brief.md, C1-C7; the security round: security-u8-f1.md; its ruling: advisor-u8-f1-sec.md; the
       stop's moves: 6,230, then 6,282 by U8F1-SIZE (a).
    2. What leaves the server now, one line for his Stop B read: `collect/`, `probe/presence.txt`,
       `ds-selftest/ws_split.txt`, `bringup/facts.txt`, `s14/` whole and `s14-redact.txt`. The new prints are integers
       only, and the policy reads are numbers. The s14 folder's own check runs with no VM list (S14-VMS), and the hook
       checks the whole push with it.
    3. The D10 and DEP-HOLD lines: no marks.json or judge change; no manifest or lockfile; ops/browser's package.json
       and package-lock.json unchanged.
    4. The first run against the merged trees: the builder's end checks ran at 77ffc95, so M5 is U8-F1's first run
       against NB2's and U7's merges.
    5. The security findings left open, for PR #40's closing list: SEC-U8F1-3 and SEC-U8F1-4 (C2); SEC-SB-9's rest, as
       before. logq's non-object line is a STATUS follow-up, not a security finding.
    6. ADV-U8H-4, applied in the same edit (my head read asked for it; it is not done): :63-64's "U6c's names (NC-R2-4,
       read by test 7 on the merged code: M5 above)" becomes "U6c's names: test 7's item 4 reads fixtures; on real
       output, the head read's run of probe_presence on U6c's 24 Nov log (P0 present, P1 at the drop rule)"; the open
       list gains U7's alignments (the URL contract in config.mjs, `node build.mjs` in rs.yaml), checked at U7's merge:
       their state as found; :70-71's "For the advisor's head read" line points to advisor-u8-head.md (read).
    7. The two wordings for his Stop B read (u8-f1-build.md, "Open"): "runs on `<results>\r1\s14` with no VM list"
       (S14-VMS, choice 25); the save's shorter reason.
  Pushed by a checkpoint save with the card updated.
- **M9, after the push:** the saver restarted; CI on the pushed head (gate with its gate and smoke jobs, toolchain; push
  and pull request), all green; B's read of the pushed head in the next window (not a round; advisor-u8-f1-sec.md
  section 1). After that read, U8-F1's builder's worktree comes off by the re-ruled form for U7's builder
  (advisor-u7-head.md, "Item 6, re-ruled"): its notes on the lane at the same paths, then the builder, resumed in a
  window, `git add`s the ADDED and differing paths and runs `git merge --ff-only`, and `git status --short` prints
  nothing; any refusal: it stays until PR #40's close. Never `--force`, never `rm -rf`. Q3's addition
  (advisor-u8-f1-merge.md:90-95, Low): the builder's worktree also holds the scratch folder
  `.checkpoints/netcode-proof-test/u8-f1/` untracked (drafts.md, list-draft.test.mjs, pin-draft.test.mjs,
  hooks-proof.mjs, missing-log.mjs), beside u8-f1-build.md; step 1's notes save copies each of those files to the main
  folder's same path, sha256-equal to the worktree's; ffcheck then expects exactly those, the six differing ops paths,
  and nothing else; otherwise the fast-forward leaves staged additions, and the fallback (b) applies.

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
- Answered 06:00 (advisor-u8-f1-merge.md): Q1 figure by figure (C1); Q2 C2, and U8-FILLS is his (before Stop B's
  fill); Q3 agree, plus M9's scratch-folder addition; Q4 C3.

## Done lines (one per step, by Edit)
- DONE 06:0x-06:1x: M0's prep: B's conditions C1-C3 and its Q3 addition applied by Edit word for word (ca46870);
  u8-report.md's U8-F1 section drafted (M8's items with C3's seven; ADV-U8H-4's three corrections applied; the commit,
  M5 and M7 lines to fill); `git diff --no-index` of u7-m4-cmp.mjs and u8f1-m4-cmp.mjs: only the header (:1-4), :9,
  :10 and :32; no cargo, rustc or npt process (`pgrep -l`, exit 1 each); the lane lock free (`flock -n`, exit 0); the
  part-clean in the card's form (no ask): "Removed 5765 files, 7.1GiB total", the lock and CACHEDIR.TAG there, no
  debug folder, free 7,796 to 14,370 MiB. The saver b507lg9fc ran out 06:01:14 ("watch stopped: time is up"; its last
  autosave bea1d82), not restarted. PR #40 re-subscribed 05:58, its rules event read 05:59 (0 review threads; no check
  runs on the notes-only head; main b9aad01 in the branch). Next: this line saved, the advisor tool, then M0.
- DONE 06:1x: M0: no agent or workflow running (B and A complete; ListAgents none at 05:5x); no cargo, rustc or npt
  process (`pgrep -l`, exit 1 each, 06:06); the lane lock free (`flock -n`, exit 0); HEAD 0f8aefd = origin before
  this edit, clean but the new notes; `git fetch origin main`: FETCH_HEAD b9aad01 (no move); no .git/index.lock; free
  14,370 MiB; the resume line on the card (line 8); the saver ran out 06:01:14 ("watch stopped: time is up"), no
  `checkpoint.mjs watch` process (`pgrep -f`, exit 1). M1: `sha256sum`
  4d6a248425aebb25b317cf2da363e00915e8f2b13eb6f26a6a259a5e0f3f7164 (= the plan's). The advisor tool (06:0x): go, with
  nine points: the report's U7-alignments line rewritten to what was checked (rs.yaml read at this merge; the URL
  contract not verified, with a proposal for B's head read); M0's base-run expectation line added; M7-M9's U7 done
  lines and the tracked wip/ files read before M7 and M8; UM-4's lines and U8-FILLS checked during M5. Chat about 30 %
  by estimate (no result event in the last two pages). Next: this line saved (H0), `git diff --quiet 6968215 H0 --
  spikes/`, the base run, M2, then M3a.
- DONE 06:11:52-06:12:2x: saved and pushed 833c3f0; H0 833c3f000a6c0a2e4d2deb49ef76d259a6a8a939 (= origin, clean);
  the no-save window is open. `git diff --quiet 6968215 H0 -- spikes/`: exit 0. Base run
  (.gate/u8-f1-merge/light-base.log, 06:12:07-06:12:20, exit 0, HEAD 833c3f0, the lock held exclusively): refs 2,145
  (N0; U7's M5 had 2,139, and notes since moved it: recorded, and on, by C1), marks 81 (drafts at 626df678…; rows
  unchanged), self 80 of 80, redact 11 of 11, 464 items, 0 findings. M2: `git diff --quiet 77ffc95 H0 --` the 6
  paths: exit 0. Next: M3a, then M3 and M4.
- DONE 06:1x: M3a: `git update-index --refresh` exit 1, its one line ".checkpoints/netcode-proof-test/u8-f1-merge-plan.md:
  needs update" (this unsaved file; no spikes/ path): the bar met; `git diff --quiet -- spikes/` exit 0; `git diff
  --quiet HEAD -- spikes/` exit 0. M3: `git apply --check --index` exit 0. M4: `git apply --index` exit 0. `git status
  --short`: exactly the 6 paths staged (6 M) and this file (unstaged). u8f1-m4-cmp.mjs: "6 paths in the patch; 6
  byte-equal with mode 644 on both sides; 0 not" (exit 0). `git ls-files -s` of the 6: all 100644 (blobs 2ad43ca,
  84342b9, 24122c0, c1384ca, e77feb0, eb11e52). `sha256sum` of the 6: the Facts' six. M4c: package.json 8370495d… and
  package-lock.json a0efce8e… equal in agent-abf71d95fc07b3522 and in the main folder; ops/browser/node_modules absent,
  the kept install there (`test -d`), then `ln -s` of it. Next: M5 in the background (.gate/u8-f1-merge/m5-check.log).
- DONE 06:13:46-06:36:32: M5 (background bt9lcars6; .gate/u8-f1-merge/m5-check.log, exit 0; the lock held, incremental
  off, cold after the part-clean; HEAD 833c3f0; the run's diff hash 4d6a2484…7164 = the patch's, untracked e3b0c442…
  = none): ops 104 of 104, 0 fail; wrappers "315 ok, 0 failed"; check.mjs "all 12 steps pass": fmt 1.3 s; clippy
  42.1 s; clippy-wasm32 11.0 s; doc 18.0 s; test 1,181.9 s, 931 and 1,028 passed, 0 failed, 0 skipped; deps 2.6 s
  (the held line as U7's: "npt-client-wasm ships npt-client and wasm-bindgen only"); wasm 88.2 s, 27,000 ticks, 0
  mismatching, the hash-map bite caught at SS-s1 tick 1234; refs 2,145 (= N0); marks 81, rows unchanged; self 80 of
  80; web 70 of 70 (TypeScript 6.0.3); redact 11 of 11, 464 items, 0 findings. Every C1 bar met. Before M7 and M8
  (the advisor tool's point 2): u7-merge-plan.md:214-249 read; the tracked wip/ files listed: only
  wip/u8-f1-build.patch is U8-F1's (the other nine belong to other units).
- DONE 06:3x: M7: `unlink` of ops/browser/node_modules (the install still in agent-abf71d95fc07b3522: `test -d` OK);
  `node tools/gate.mjs --no-smoke` GATE PASS (base b9aad01; 1-4, 5b-5d PASS, 5a skipped); "repo map OK: 245 entries
  cover 110 required paths"; `git status --short`: the 6 paths staged and notes only (msg-2026-11-28.md, this file,
  um4-plan.md untracked). u8-report.md's M5 and M7 lines filled. Next: M8 (`git rm` of wip/u8-f1-build.patch, `git
  add` of u8-report.md and this file, the numstat check, one commit), then the card save that pushes it.
- DONE 06:4x: M8: `git rm` of wip/u8-f1-build.patch (WIP-CLEAN; the only U8-F1 file under wip/); `git add` of
  u8-report.md and this file; staged exactly the 6 (M), the 1 D and the 2 M; `git diff --cached --numstat 833c3f0 --
  spikes/` = the patch's numstat line for line, `--shortstat` "6 files changed, 105 insertions(+), 12 deletions(-)";
  the commit ddb06ffe829495023aaf12bbb9e5d27ce12c5da9 (9 files; no skip marker), HEAD^1 = H0, `git diff --quiet HEAD
  -- spikes/` exit 0. Pushed by the card save that carries this line. Next: M9 (the saver restarted; CI on the pushed
  head, push and pull request; B's read of the pushed head in the 10:50 window; then U8-F1's builder worktree by the
  re-ruled form with Q3's scratch-folder addition).
