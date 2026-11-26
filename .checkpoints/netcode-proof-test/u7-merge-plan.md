# U7's merge into the lane: the plan (M0-M9), written 2026-11-26 17:1x UTC by the main session
Form: U8's M0-M9 (u8-merge-plan.md: its steps, with the changes that won over them there) with U7's terms, and the
snapshot read's conditions U7-M-1 to U7-M-8 (advisor-u7-snapshot.md section 4) applied. Its check: the advisor
a0530735048f1620a, who wrote those conditions, in the 19:50 UTC window (its item 5: "else I bring it at 19:50"). The
merge into the lane needs no go of his (as U4r's, U4f's and U8's); PR #40 keeps its own go.

## Facts (read 17:1x UTC; HEAD b556316 = origin, clean)
- wip/u7-build-rs.patch: sha256 6535c18130cad99881b798a332973b37215c1edad27929dd54178efc00342728, 267,380 B, 43 files,
  base 80b6748 (U7's builder's worktree .claude/worktrees/agent-a9873cdba07a3219e, at 80b6748). By `git apply --numstat
  --summary`:
  - 10 modified: check.mjs, check.test.mjs, crates/bots/src/lib.rs and load.rs, crates/client-wasm/src/lib.rs,
    crates/client/src/clock.rs and lib.rs, crates/client/tests/trace.rs, crates/lab/src/scenario.rs,
    crates/server/tests/carry.rs;
  - 33 created, every one 100644: crates/client-wasm/clippy.toml, src/engine.rs and src/script.rs;
    crates/client/tests/sent.rs; web/ (bites/hooks.js, build.mjs, nine e2e/ files, index.html, four src/ files, thirteen
    test/ files with fixtures/boundary.json, tsconfig.json);
  - 5,193 added, 47 removed (all paths under spikes/netcode-proof/).
  - No manifest or lockfile, no marks.json, nothing under crates/judge. U7's 0b dev-dependencies are on the lane since
    56b3e1b (dependency-check.md).
- The notes: wip/u7-build-rs-notes-1055.md (a65282fa…). The fix pass's end check, end-2: 12 of 12 (test 884/981, web
  70/70, self 80/80, redact 0 findings in 454 items, refs 1,988).
- Run 4 (2026-11-26T10-48-42-618Z, in the builder's worktree's .gate/u7/t5ii/): its three runner headers carry diff
  551e9033… and untracked b4811e75…. summary.json: marks 1-7 met; mark 8 a fallback at 4.067 s with the plant (1.564 s
  normal); mark 9 NOT-RUN.
- C1's input (U7-M-5): run 4's 6-mds/result.json `pages[0]`. u8-c1.mjs:12 names run 3's folder, so u7-c1.mjs is
  u8-c1.mjs with run 4's folder at :12 and .gate/u7-merge/ in place of .gate/u8-merge/, nothing else (Write, in the
  notes folder, before M0).
- check.mjs and check.test.mjs: U7's patch adds the `web` step (12 STEPS), and check.test.mjs:2785's list gains "web".
  The `ops` step (choice 15 (ii), U7-M-6) lands here as M4b, on U7's tree, so U7's apply check meets no edit of ours.
- ops/browser/node_modules: the install kept in agent-abf71d95fc07b3522 (U8's first build worktree; package.json
  8370495d…, package-lock.json a0efce8e…), linked for M5 as at U8's merge (choice 15 (i)) and removed before M7.
- Order (U7-M-1): MAIN-68 first (main-68-plan.md rev 2: #70, #73, #74). Builder 2's merge (46 files: crates/judge,
  marks/ and its notes; none of U7's 43) may come before or after, never overlapping.
- The guard: after MAIN-68, #73's rules check every call. No command below has a program in front of git or gh, and none
  runs xargs at the session level (main-run.sh's and u7-run.sh's own git and xargs calls run inside the scripts, which
  the guard does not read: INPUT-RUN). If one asks or is refused, the step stops and the advisor rules.

## Steps
- **M0, preconditions:**
  - MAIN-68's step 6 done (its head pushed, CI green); no agent or workflow on the lane; the lane lock free; HEAD =
    origin, clean.
  - `git fetch origin main` in its own call: main still the tip MAIN-68 took in. A code, CI or rules move stops here
    (MAIN-52, plan first); a docs-only move is noted, nothing more.
  - Free disk at least 8,192 MiB; under it, the part-clean in the card's Tools form, then measure again; still under:
    stop, the advisor rules.
  - u7-report.md drafted (U7-M-7's items; M5's and C1's lines to fill) and u7-c1.mjs written.
  - The saver stopped by TaskStop with its proof; H0 = HEAD, named in the chat. The no-save window runs from here to
    M8's push: a Stop hook gets one line, no save.
  - The base light run at H0: `bash .checkpoints/netcode-proof-test/main-run.sh
    /home/user/Shamble-Frontlines/.gate/u7-merge/light-base.log node check.mjs --only refs,marks,self,redact`
    (run_in_background, timeout 7200000): N0, the marks line, self, redact items.
- **M1:** the patch's sha256 is 6535c18130cad99881b798a332973b37215c1edad27929dd54178efc00342728.
- **M2:** `git diff --quiet 80b6748 H0 --` the 43 paths: exit 0 (each unchanged on the lane since 80b6748).
- **M2b, the tie to run 4 (U7-M-3), before M4:** `bash
  /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u7-run.sh
  /home/user/Shamble-Frontlines/.claude/worktrees/agent-a9873cdba07a3219e
  /home/user/Shamble-Frontlines/.gate/u7/m2b-header.log true`. Its header must read "diff 551e9033…; untracked
  b4811e75…", run 4's tree. Anything else stops.
- **M3 and M4:** `git apply --check --index`, then `git apply --index`. `git status --short` lists exactly the 43 paths
  staged (10 M, 33 A). Each is cmp-equal to its copy in U7's worktree, with the same mode (`stat -c %a`; `git ls-files
  -s` 100644).
- **M4b, the `ops` step, test first (U7-M-6):** check.test.mjs:2785's list (which now holds "web") gains "ops" at its end
  (Edit); `node --test check.test.mjs` fails on that assert. Then check.mjs's STEPS gains `ops` at the same place (Edit,
  about 10 lines): `node --test 'ops/**/*.test.mjs'` (the glob form), then `bash ops/tests/wrappers.test.sh`, both in
  HERE, failing if either fails, summarised with nodeCounts and the wrappers' last line. Then the self tests pass.
- **M4c, the link:** package.json 8370495d… and package-lock.json a0efce8e… equal in agent-abf71d95fc07b3522 first; then
  `ln -s` of its ops/browser/node_modules into the main folder's ops/browser/.
- **C1 (U7-M-5):** `node .checkpoints/netcode-proof-test/u7-c1.mjs <H0>` (run 4's 6-mds `pages[0]` through the applied
  ops/browser/lib.mjs's save()), then `bash .checkpoints/netcode-proof-test/main-run.sh
  /home/user/Shamble-Frontlines/.gate/u7-merge/c1-judge.log cargo run -q --locked -p npt-judge -- --marks
  marks/marks.json --logs /home/user/Shamble-Frontlines/.gate/u7-merge/c1-logs --out
  /home/user/Shamble-Frontlines/.gate/u7-merge/c1-verdicts.json --only PM-0001-36`. The bar: a PM-0001-36 entry with a
  verdict, its "ML* (rough, reported)" part counting 1 log, and no "npt-judge: " line. A read error is M6. NC-6
  changed the Recorder; run 4's records hold the same two kinds, `event` and `second`.
- **M5 (U7-M-4):** one main-run.sh call, cheap runs first: `bash .checkpoints/netcode-proof-test/main-run.sh
  /home/user/Shamble-Frontlines/.gate/u7-merge/m5-check.log node --test 'ops/**/*.test.mjs' :: bash
  ops/tests/wrappers.test.sh :: env CARGO_INCREMENTAL=0 node check.mjs` (run_in_background, timeout 7200000). All
  pass:
  - ops: every test passes, 0 fail (101 at U8's merge; more if U8-F1 merged first), the count recorded;
  - wrappers: "315 ok, 0 failed";
  - check.mjs: all 13 steps (U7's `web` and M4b's `ops` included).
  - The counts recorded as found: test against end-2's 884/981 (MAIN-68 and Builder 2 may move it); web 70; self 80;
    marks 81, rows unchanged; redact 0 findings, items as counted; refs = N0 + 33 (the 33 new files); wasm as end-2.
- **M6:** a failing test or bar is never retried unnamed. It stops the merge: `git apply -R --index` of the patch,
  check.mjs and check.test.mjs restored to H0 (`git diff --quiet H0 -- spikes/` exits 0 after), the link removed, the
  saver restarted. The failure and its log go to the advisor in the next window. Nothing is committed.
- **M7:** unlink (the install stays in its worktree); `node tools/gate.mjs --no-smoke` PASS; `node
  .claude/scripts/repo-map-check.mjs` OK; `git status --short` shows the 43 paths (check.mjs and check.test.mjs as `MM`
  after M4b) and notes only.
- **M8, one commit:** `git add` of exactly those paths; `git rm` of wip/u7-build-rs.patch (WIP-CLEAN), and of
  wip/u7-t-build-t5i.patch if it is still tracked (the T-half's older snapshot, inside this one). u7-report.md
  (U7-M-7):
  - marks 1-8 read from run 4's summary.json against the brief's words; mark 8 under his restated words (u7-brief.md's
    addendum), with both runs' numbers (run 3: 1.60 s and 4.06 s; run 4: 1.56 s and 4.07 s);
  - mark 9 NOT-RUN, never PASS, with the stand-ins and their limits as advisor-u7-snapshot.md section 1 gives them;
  - the wording items (NC-3, NC-8, TE-6, TE-11's records, T5ii-C's note);
  - the departures: F3's plant; HONEST_MAX_LEAD in bots/load.rs and server/tests/carry.rs, outside the file list;
    `npt_client::wire`'s re-exports; C6 reworded; TE-11's limits literal kept (FULL_STATE_MAX is `pub(crate)` in
    npt-net);
  - the carries: TE-5 to Stop B; U6d's per-slot list and T5i-2; CLIPPY_TWINS; DROP-not-REJECT to R8 and Stop B;
    ADV-U7S-5 and -6 to U8's next change;
  - the slips (ADV-U7S-3, -4); the size (5,193 added, 47 removed, of 6,000); C1's line; M4b's `ops` step.
  Pushed by a checkpoint save with the card updated.
- **M9, after the push (U7-M-8):** the saver restarted; CI on the pushed head (gate with gate and smoke, toolchain; push
  and pull request), all green; the advisor's read of the pushed head in the next window (not a round). After that read:
  the builder's worktree comes off; .gate/u7-amend comes off by undoing its two edits by Edit, then a plain `git worktree
  remove`; cli-target (433 MiB) is cleaned by `cargo clean --target-dir` (0b's carry).

## Risks
- M5's counts move with MAIN-68 and Builder 2: each bar comes from this merge's own base run on its own H0 (MAIN-68's
  Q3 (a)), never another merge's numbers.
- Disk: after MAIN-68's light runs (and Builder 2's whole run, if first), the cache may be warm; M0's 8,192 MiB bar with
  the part-clean covers it. M5 builds the web half and both workspaces' tests (about 20-45 minutes).
- U8-F1's builder, if resumed after his U8F1-SIZE answer, works in its own worktree on ops/ files (r1.sh and its
  tests), none of U7's 43. Its merge comes before or after U7's, one at a time; if first, M5's ops count is its count.
- #73's rules: a refusal or an ask stops the step where it is (M6's form if after M4).

## Questions for the checker
- Q1: M4b (the `ops` step) inside U7's merge commit, as U8's plan had it, or in the first change after it (U7-M-6 allows
  both)? My lean: inside, test first.
- Q2: C1 through u7-c1.mjs (u8-c1.mjs with run 4's folder and the u7-merge output folder): enough, or should the script
  take the folder as an argument?
- Q3: M2b's header call runs `true` through u7-run.sh in the builder's worktree (no build): the right form for U7-M-3,
  and under #73's rules (no git or gh word on the line)?
