# U7's merge into the lane: the plan (M0-M9), written 2026-11-26 17:1x UTC by the main session
As checked: APPROVE WITH CONDITIONS UM-1 to UM-5, all Low (advisor-u7-merge.md, 19:5x-20:0x UTC), applied word for word
below; no new check.
Form: U8's M0-M9 (u8-merge-plan.md: its steps, with the changes that won over them there) with U7's terms, and the
snapshot read's conditions U7-M-1 to U7-M-8 (advisor-u7-snapshot.md section 4) applied. Its check: the advisor
a0530735048f1620a, who wrote those conditions, in the 19:50 UTC window (its item 5: "else I bring it at 19:50"). The
merge into the lane needs no go of his (as U4r's, U4f's and U8's); PR #40 keeps its own go. The advisor tool's three
fixes (17:2x) are folded in: the `ops` step leaves the merge; a resume line and done lines; the C1 script's changes
named.

## Facts (read 17:1x-17:2x UTC; HEAD 9b84088 = origin, clean)
- wip/u7-build-rs.patch: sha256 6535c18130cad99881b798a332973b37215c1edad27929dd54178efc00342728, 267,380 B, 43 files,
  base 80b6748 (U7's builder's worktree .claude/worktrees/agent-a9873cdba07a3219e, at 80b6748). By `git apply --numstat
  --summary`:
  - 10 modified: check.mjs, check.test.mjs, crates/bots/src/lib.rs and load.rs, crates/client-wasm/src/lib.rs,
    crates/client/src/clock.rs and lib.rs, crates/client/tests/trace.rs, crates/lab/src/scenario.rs,
    crates/server/tests/carry.rs;
  - 33 created, every one 100644: crates/client-wasm/clippy.toml, src/engine.rs and src/script.rs;
    crates/client/tests/sent.rs; web/ (bites/hooks.js, build.mjs, nine e2e/ files, index.html, four src/ files, thirteen
    test/ files with fixtures/boundary.json, tsconfig.json);
  - 5,193 added, 47 removed, all under spikes/netcode-proof/.
  - No manifest or lockfile, no marks.json, nothing under crates/judge. U7's 0b dev-dependencies are on the lane since
    56b3e1b (dependency-check.md).
  - `git apply --check --index` exits 0 on 9b84088 (17:2x).
- The notes: wip/u7-build-rs-notes-1055.md (a65282fa…). The fix pass's end check, end-2: 12 of 12 (test 884/981, web
  70/70, self 80/80, redact 0 findings in 454 items, refs 1,988).
- Run 4 (2026-11-26T10-48-42-618Z, in the builder's worktree's .gate/u7/t5ii/): its three runner headers carry diff
  551e9033… and untracked b4811e75…. summary.json: marks 1-7 met; mark 8 a fallback at 4.067 s with the plant (1.564 s
  normal); mark 9 NOT-RUN.
- C1's script (U7-M-5): u8-c1.mjs names run 3 in four places. u7-c1.mjs is u8-c1.mjs with exactly these changes: :12
  run 4's folder (2026-11-26T10-48-42-618Z); :14 OUT .gate/u7-merge/c1-logs; :47 the check string naming run 4; and its
  header comment (:1-4). WASM (:13) reads the builder's .gate/netcode-proof-test/t5-dist, now the fixed tree's build;
  everything else is unchanged. Written by Write in the notes folder before M0.
- check.mjs and check.test.mjs: U7's patch adds the `web` step (12 STEPS), and check.test.mjs:2785's list gains "web".
- The `ops` step (choice 15 (ii), U7-M-6) is the first change after this merge, not inside it. Inside, M6's `git apply
  -R --index` would refuse once its Edits made the two check files differ from the index, and the commit would no
  longer equal the snapshot. That change carries U8's fix 2: once `ops` is a check.mjs step, every later whole run
  needs webdriverio, so it brings a real install in ops/browser by U8's one form (`npm_config_ignore_scripts=true npm ci
  --ignore-scripts`; a real folder the root .gitignore covers), or the step fails with one line naming that install. It
  goes to the advisor first (the dependency record), before Stop B's fill. Builder 2's whole run, if after it, counts
  13 steps instead of 11.
  - UM-4: the `ops` step's own change. It is its own change by the main session: a plan first and the advisor's check
    in a window, after this merge's M9 and before Stop B's fill, with these parts:
    - (a) **Test first.** check.test.mjs:2785's list gains "ops" after "web", and `node --test check.test.mjs` fails on
      that assert. Then check.mjs's STEPS gains `ops` after `web`: `node --test 'ops/**/*.test.mjs'`, then `bash
      ops/tests/wrappers.test.sh`, both in HERE, failing if either fails, summarised with nodeCounts and the wrappers'
      last line. The self tests pass.
    - (b) **U8's fix 2, the real install.** First, package.json 8370495d… and package-lock.json a0efce8e… by sha256.
      Then `npm_config_ignore_scripts=true npm ci --ignore-scripts --prefix spikes/netcode-proof/ops/browser`, its own
      call. Then both sha256s unchanged and `git status --short` unchanged (the folder is a real one, so the root
      .gitignore's `node_modules/` covers it). npm's "added N packages" and its audit line are recorded; "4 moderate" is
      security's expected chain (security-u8-deps.md C3 (c)).
    - (c) **The dependency record.** One note section in dependency-check.md (the install's form, both sha256s before
      and after, npm's two lines), with no bare sha256 line: no manifest or lockfile changes, so DEP-HOLD holds nothing,
      and no new security read is needed (the form and the lock were cleared at U8).
    - (d) **A missing folder.** When ops/browser/node_modules is absent, the step fails with one line naming that
      install.
    - (e) **Whole runs before it lands** keep choice 15 (i): ops tests and wrappers beside `node check.mjs`.
- ops/browser/node_modules for this merge's M5: the install kept in agent-abf71d95fc07b3522 (U8's first build
  worktree; package.json 8370495d…, package-lock.json a0efce8e…), linked as at U8's merge (choice 15 (i)), removed
  before M7.
- Order (U7-M-1): MAIN-68 first, at main's tip when it runs (b9aad01 at the advisor's check: #70, #73, #74, #76, #77
  and #75; #75 adds a root-workspace crate, nothing under spikes/). Builder 2's merge (46 files: crates/judge, marks/ and
  its notes; none of U7's 43) came first: 48efc9c, 18:02 UTC.
- The guard: after MAIN-68, #73's rules check every call. No command below has a program in front of git or gh, and none
  runs xargs at the session level (main-run.sh's and u7-run.sh's own git and xargs calls run inside the scripts, which
  the guard does not read: INPUT-RUN). If one asks or is refused, the step stops and the advisor rules.

## Resume line (on the card from M0 until M9 is done; the done lines below say where it stands)
"U7-MERGE running (u7-merge-plan.md, its done lines). If HEAD = H0 and `git diff --cached --name-only` lists exactly the
43 paths, M4 is done: resume at the step after the last done line (M4c, C1 or M5). If HEAD^1 = H0, HEAD holds the 43
paths and HEAD is not on the remote, resume at M8's push (the card save). A failing step or bar is M6. Anything else:
the advisor rules."

## Steps
- **M0, preconditions:**
  - MAIN-68's step 6 done (its head pushed, CI green); no agent or workflow on the lane; the lane lock free; HEAD =
    origin, clean.
  - `git fetch origin main` in its own call: main still the tip MAIN-68 took in. A code, CI or rules move stops here
    (MAIN-52, plan first); a docs-only move is noted, nothing more.
  - Free disk at least 8,192 MiB; under it, the part-clean in the card's Tools form, then measure again; still under:
    stop, the advisor rules.
  - u7-report.md drafted (U7-M-7's items; M5's and C1's lines to fill) and u7-c1.mjs written.
  - Before M0 (UM-3): `git diff --no-index` of u8-c1.mjs and u7-c1.mjs (its own call) shows only the four changes Facts
    names: the header (:1-4), :12, :14 and :47.
  - The resume line onto the card by Edit (replacing lines). The saver stopped by TaskStop with its proof; H0 = HEAD,
    named in the chat. The no-save window runs from here to M8's push: a Stop hook gets one line, no save.
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
- **M4c, the link:** package.json 8370495d… and package-lock.json a0efce8e… equal in agent-abf71d95fc07b3522 first; then
  `ln -s` of its ops/browser/node_modules into the main folder's ops/browser/.
- **C1 (U7-M-5):** `node .checkpoints/netcode-proof-test/u7-c1.mjs <H0>` (run 4's 6-mds `pages[0]` through the applied
  ops/browser/lib.mjs's save()), then `bash .checkpoints/netcode-proof-test/main-run.sh
  /home/user/Shamble-Frontlines/.gate/u7-merge/c1-judge.log cargo run -q --locked -p npt-judge -- --marks
  marks/marks.json --logs /home/user/Shamble-Frontlines/.gate/u7-merge/c1-logs --out
  /home/user/Shamble-Frontlines/.gate/u7-merge/c1-verdicts.json --only PM-0001-36`. The bar: a PM-0001-36 entry with a
  verdict, its "ML* (rough, reported)" part counting 1 log, and no "npt-judge: " line. A read error is M6. NC-6 changed
  the Recorder; run 4's records hold the same two kinds, `event` and `second`. At C1 (UM-3): the line "wasm sha256" it
  prints equals 901a649231dc9c217204b121a2800641c3c6aa9cd3cb9d42eb5657452b0a0e77, run 4's npt_client_wasm_bg.wasm.
  Otherwise M6.
- **M5 (U7-M-4):** one main-run.sh call, cheap runs first: `bash .checkpoints/netcode-proof-test/main-run.sh
  /home/user/Shamble-Frontlines/.gate/u7-merge/m5-check.log node --test 'ops/**/*.test.mjs' :: bash
  ops/tests/wrappers.test.sh :: env CARGO_INCREMENTAL=0 node check.mjs` (run_in_background, timeout 7200000). All
  pass:
  - ops: every test passes, 0 fail (101 at U8's merge; more if U8-F1 merged first), the count recorded;
  - wrappers: "315 ok, 0 failed";
  - check.mjs: all 12 steps (U7's `web` included).
  - The counts recorded as found: test against end-2's 884/981 (MAIN-68 and Builder 2 may move it); web 70; self 80;
    marks 81, rows unchanged; redact 0 findings, items as counted; refs = N0 + 33 (the 33 new files); wasm as end-2.
  - UM-2: test: 931 measured and 1,028 bites passed, 0 failed, 0 skipped. If the totals differ: name the difference. If
    `git diff --stat 48efc9c H0 -- spikes/netcode-proof/crates` explains it, record it and go on; otherwise M6.
- **M6:** a failing test or bar is never retried unnamed. It stops the merge: `git apply -R --index` of the patch (then
  `git diff --quiet H0 -- spikes/` exits 0), the link removed, the saver restarted. The failure and its log go to the
  advisor in the next window. Nothing is committed.
- **M7:** unlink (the install stays in its worktree); `node tools/gate.mjs --no-smoke` PASS; `node
  .claude/scripts/repo-map-check.mjs` OK; `git status --short` shows the 43 paths staged and notes only.
- **M8, one commit:** `git add` of the notes below; `git rm` of wip/u7-build-rs.patch (WIP-CLEAN), and of
  wip/u7-t-build-t5i.patch if it is still tracked (the T-half's older snapshot, inside this one). Under spikes/ the
  commit equals the snapshot (`git diff --cached --numstat H0 -- spikes/` = the patch's numstat). u7-report.md
  (U7-M-7):
  - marks 1-8 read from run 4's summary.json against the brief's words; mark 8 under his restated words (u7-brief.md's
    addendum), with both runs' numbers (run 3: 1.60 s and 4.06 s; run 4: 1.56 s and 4.07 s);
  - mark 9 NOT-RUN, never PASS, with the stand-ins and their limits as advisor-u7-snapshot.md section 1 gives them;
  - the wording items (NC-3, NC-8, TE-6, TE-11's records, T5ii-C's note);
  - the departures: F3's plant; HONEST_MAX_LEAD in bots/load.rs and server/tests/carry.rs, outside the file list;
    `npt_client::wire`'s re-exports; C6 reworded; TE-11's limits literal kept (FULL_STATE_MAX is `pub(crate)` in
    npt-net);
  - the carries: TE-5 to Stop B; U6d's per-slot list and T5i-2; CLIPPY_TWINS; DROP-not-REJECT to R8 and Stop B;
    ADV-U7S-5 and -6 to U8's next change; the `ops` step to the first change after this merge;
  - the slips (ADV-U7S-3, -4); the size (5,193 added, 47 removed, of 6,000); C1's line.
  Pushed by a checkpoint save with the card updated.
- **M9, after the push (U7-M-8):** the saver restarted; CI on the pushed head (gate with gate and smoke, toolchain; push
  and pull request), all green; the advisor's read of the pushed head in the next window (not a round). After that read,
  the cleanups (U7's builder's worktree, .gate/u7-amend, cli-target's 433 MiB) in the forms the advisor names: a worktree
  with uncommitted files needs more than a plain `git worktree remove`. UM-5: their forms are named at the advisor's read
  of the pushed head, not here. Never `git worktree remove --force` and never `rm -rf`: the guard refuses both
  (launch-0850.md:103-104). The builder's worktree holds U7's uncommitted files and its notes, so it comes off only by a
  form that keeps every file a copy is not yet committed for.

## Done lines (one per step, by Edit, as at U8's merge)
- DONE 22:0x-22:1x: M0's prep: u7-report.md drafted (U7-M-7's items, the (fill) lines for M2-M7 and C1); u7-c1.mjs
  written; UM-3's `git diff --no-index` of u8-c1.mjs and u7-c1.mjs: only the header (:1-4), :12, :14 and :47; the
  part-clean in the card's form (no ask): "Removed 5674 files, 6.8GiB total", the lock and CACHEDIR.TAG there, no
  debug folder, free 7,622 to 13,886 MiB; saved ea0a20d. The advisor tool 22:0x: five checks before the saver stop,
  all met: C1's log names carry stampNow()'s millisecond ISO stamp (lib.mjs:160, :174-176), so no collision with U8's
  mds-chrome-wt-a0-2026-11-26T00-35-28-859Z.jsonl, and OUT .gate/u7-merge/c1-logs does not exist yet; PR #40's rules
  event read 22:06; TaskStop loaded; `git fetch origin main` fresh, FETCH_HEAD b9aad01 (= MAIN-68's tip: no move);
  u7-m4-cmp.mjs written for M3/M4's compare (the patch's paths, byte-equal and mode 644 on both sides; read only).
  Also known: web/node_modules holds TypeScript 6.0.3 (the `web` step runs no npm ci); main-run.sh makes its log's
  folder; wip/u7-t-build.patch (19 web/ files, every one among the 43) is tracked and inside this snapshot, so M8's
  `git rm` takes it too, under WIP-CLEAN as the T5 (i) one (the plan's M8 left it out; recorded in u7-report.md).
- DONE 22:08-22:1x: M0: MAIN-68's step 6 done (CI green on 7971da0); no agent or workflow running; no cargo, rustc or
  npt process (pgrep exit 1); the lane lock free (flock -n); HEAD ea0a20d = origin before this edit, clean but the
  new notes; main b9aad01; free 13,886 MiB; the resume line on the card (line 8, replaced). The saver bpqdxxpm4
  stopped by TaskStop at 22:08 ("Successfully stopped task"; no index.lock; no `checkpoint.mjs watch` process). M1:
  `sha256sum` 6535c18130cad99881b798a332973b37215c1edad27929dd54178efc00342728 (= the plan's). Next: this line saved
  (H0), the base light run, M2, M2b, then M3.
- DONE 22:08:59-22:09:3x: saved and pushed dc51f0e; H0 dc51f0e387a6a2017d435e4005b9b88e8f1db6cd (= origin, clean);
  the no-save window is open. Base run (.gate/u7-merge/light-base.log, 22:09:17-22:09:29, exit 0, HEAD dc51f0e, the
  lock held exclusively): refs 2106 (N0), marks 81 (drafts at 626df678…; rows unchanged), self 78 of 78, redact 11 of
  11, 431 items, 0 findings. M2: `git diff --quiet 80b6748 <H0> --` the 43 paths (from `git apply --numstat`): exit 0.
  M2b (.gate/u7/m2b-header.log, 22:09:31-32, exit 0): HEAD 80b6748; "diff
  551e9033246e983b62e7fb38db8eb982ce8eac383a20b685e71099549edffcd3; untracked
  b4811e7541216436bf7af76e02ef14c7368d5752a7386fb556dee44e36b9d156" = run 4's tree. Next: M3 and M4.
- 22:1x: M3's first `git apply --check --index` exit 1: "does not match index" for 8 of the 10 changed paths, all
  under crates/ (bots lib.rs and load.rs, client-wasm lib.rs, client clock.rs, lib.rs and tests/trace.rs, lab
  scenario.rs, server tests/carry.rs); check.mjs and check.test.mjs passed. Cause, not content: main-run.sh:48-52
  touches every file under crates/ before and every 2 s during its commands (its header says so), so the base run
  (22:09:17-29) left all of crates/ at 22:09:29.109 while the index held 20:18:26 (MAIN-68's light run), and `git
  apply --index` trusts the index's cached stat data without reading content. Content checks, each exit 0: `git diff
  --quiet -- spikes/` (tree = index), `git diff --quiet HEAD -- spikes/` (tree = H0), and M2 above; `git status --short`
  listed only this file; `find -newermt` showed the whole crates/ tree at 22:09:29. The advisor tool (22:1x): no bar
  failed, M6 not yet; `git update-index --refresh` named in the chat and run alone: exit 1, its one line
  ".checkpoints/netcode-proof-test/u7-merge-plan.md: needs update" (this unsaved line; no spikes/ path); `git ls-files
  --debug` of bots/src/lib.rs now 1795730969:109624894 (22:09:29.109); HEAD still dc51f0e. M3's check is re-run once,
  by name; any second failure is a failing bar (M6, nothing applied, the saver restarted, the log to the advisor at
  00:50). M6's undo form from here (C1's judge run and M5 go through main-run.sh and touch crates/ again): `git
  update-index --refresh`, then `git apply -R --index` of the patch, then `git diff --quiet H0 -- spikes/`. For the
  advisor agent's head read: a ruling on this one retry and on the form's fix (the base run before H0's save, or a
  refresh before M3 and before M6's undo).
- DONE 22:1x: M3, re-run once by name: `git apply --check --index` exit 0; M4: `git apply --index` exit 0. `git status
  --short`: exactly the 43 paths staged (10 M, 33 A) and this file (unstaged). u7-m4-cmp.mjs: "43 paths in the patch;
  43 byte-equal with mode 644 on both sides; 0 not" (exit 0). `git diff --cached --summary`: 33 "create mode 100644"
  lines, no mode change. M4c: package.json 8370495dee2ed77e… and package-lock.json a0efce8e511a8865… equal in
  agent-abf71d95fc07b3522 and in the main folder; ops/browser/node_modules absent, then `ln -s` of the kept install's
  (it shows as `??`: the root .gitignore's `node_modules/` matches folders, not links; M7 removes it). C1, first half
  (22:14:59): u7-c1.mjs <H0> saved .gate/npt-browser/records/mds-chrome-wt-a0-2026-11-26T22-14-59-687Z.jsonl, copied
  to .gate/u7-merge/c1-logs/; log sha256 0d07fa1fcda887f87cd49b3f8a208262eb21452798ab0c47567aca882701f055; 15 page
  lines; wasm sha256 901a649231dc9c217204b121a2800641c3c6aa9cd3cb9d42eb5657452b0a0e77 (= run 4's: UM-3's bar met).
  Next: the judge's call (background btqic8n9s, .gate/u7-merge/c1-judge.log), then M5.
- DONE 22:15:07-08: C1's judge call (.gate/u7-merge/c1-judge.log, exit 0, HEAD dc51f0e; c1-verdicts.json): "PM-0001-36
  NOT-RUN ML chrome loopback: 0 of 1 valid runs"; its parts: the six ML parts NOT-RUN (0 of 1 valid runs each), the two
  MAC-b safari parts MOVED (NPT-TRIM), "ML* (rough, reported)" PASS with 1 run (R1-mds-chrome-wt-a00, valid), its
  report 1024 smallest and 1024 largest; the read-once bite not run; no "npt-judge: " line. The bar is met (a verdict,
  the rough part counting 1 log, no read error), as U8's C1 read run 3. Next: M5 in the background
  (.gate/u7-merge/m5-check.log): ops tests, wrappers, then check.mjs's 12 steps.
- DONE 22:15:37-22:36:58: M5 (background bf9ayfbj1; .gate/u7-merge/m5-check.log, exit 0; the lock held, incremental
  off, cold after the part-clean): `node --test 'ops/**/*.test.mjs'` tests 101, pass 101, fail 0; `bash
  ops/tests/wrappers.test.sh` "315 ok, 0 failed"; `env CARGO_INCREMENTAL=0 node check.mjs` "all 12 steps pass": fmt
  1.2 s; clippy 38.4 s; clippy-wasm32 10.4 s; doc 16.8 s; test 1,114.2 s, measured 931 passed and bites 1,028 passed,
  0 failed, 0 skipped (= UM-2's 931 and 1,028); deps 2.6 s ("npt-client-wasm ships npt-client and wasm-bindgen only"
  among the held rules); wasm 77.0 s, 27,000 ticks native-native and native-Wasm, 0 mismatching, the hash-map bite
  caught at SS-s1 tick 1234; refs 2139 = N0 2106 + 33; marks 81, rows unchanged; self 80 of 80; web 70 of 70 (TypeScript
  6.0.3, no npm ci); redact 11 of 11, 464 items (431 + 33), 0 findings. Next: M7.
- DONE 22:37-22:4x: M7: `unlink` of ops/browser/node_modules (the install still in agent-abf71d95fc07b3522: `test -d`
  OK); `node tools/gate.mjs --no-smoke` GATE PASS (base b9aad01; 1-4, 5b-5d PASS, 5a skipped); "repo map OK: 245
  entries cover 110 required paths"; `git status --short`: the 43 paths staged and notes only (msg-2026-11-27.md, this
  file, u7-report.md). u7-report.md's M5 and M7 lines filled. Next: M8 (`git rm` of the three wip patches, `git add`
  of u7-report.md and this file, the numstat check, one commit), then the card save that pushes it.
- DONE 22:4x: M8: `git rm` of wip/u7-build-rs.patch, wip/u7-t-build-t5i.patch and wip/u7-t-build.patch (WIP-CLEAN;
  the third is inside the snapshot: 19 web/ files, every one among the 43); `git add` of u7-report.md and this file;
  staged exactly the 43 (10 M, 33 A), the 3 D and the 2 M; `git diff --cached --numstat <H0> -- spikes/` = the
  patch's numstat line for line, `--shortstat` "43 files changed, 5193 insertions(+), 47 deletions(-)"; the commit
  69682158790a8d9c6f53f65ede9a2a1b017d5bc3 (48 files; no skip marker), HEAD^1 = H0, `git diff --quiet <it> --
  spikes/` exit 0. Pushed by the card save that carries this line. Next: M9 (the saver restarted, CI on the pushed
  head, push and pull request; the advisor's read of the pushed head at 00:50, with the M3 retry named).
- M9, 22:38:50-: the card save 9c2fcf473f495a93c5852e214e7b0615b3126872 pushed (carries 6968215; its subject has no
  skip marker); the branch clean = origin; the saver restarted as bycyr4c38 (110 minutes, to about 00:29). CI on
  9c2fcf4, all four started 22:38:5x: push gate 45906116335 and toolchain 45906116365; pull request gate 45906121962
  and toolchain 45906121987. Their results, all four success, attempt 1: push gate 45906116335 (22:38:52-22:39:40;
  jobs gate 140203123521 and smoke 140203124576, its "Smoke (check 5a)" step success), push toolchain 45906116365
  (22:38:52-22:46:49), pull request gate 45906121962 (22:38:56-22:39:37), pull request toolchain 45906121987
  (22:38:56-22:46:57); PR #40's check_suite.completed event at 22:48:52 (nothing failed or running). M9's CI bar is
  met. Left in M9: the advisor's read of the pushed head at 00:50 (U7-M-8, with the M3 retry and the form's fix),
  then the cleanups in the forms it names.
- M9, the head read, DONE 01:01 UTC on 11-27 (advisor-u7-head.md, A a0530735048f1620a): the merge stands, no fix; M3's
  retry accepted (a Low departure from M6's letter, not its purpose); the form gains M3a (a refresh right before M3,
  and at the start of M6's undo: its section 3 words, for the next merge of this form); u7-report.md's two Edits done.
  Cleanups: cli-target CLEANED 01:1x (`env CARGO_NET_OFFLINE=true cargo clean --locked`, no cargo running: 1,496 files,
  441.1 MiB; df 7,378 -> 7,802 MiB; cli-root kept). .gate/u7-amend REMOVED 01:09 by the first ruling's form (the two
  Edits undone by Edit, both files then equal to c21582d's blobs f6e0fef and e4d5ff3 and to its sha256 d8af72f7...000e
  and 55604a5f...9f5d; a plain `git worktree remove`, exit 0), about a minute before the re-rule (01:09, "Item 6,
  re-ruled") withdrew that step as a discard-to-pass and kept the worktree until PR #40's close: a recorded departure,
  nothing lost (the Edits' content is committed at 56b3e1b; no evidence in it, the first ruling's section 6 B), for the
  advisor's next read. U7's builder worktree: by the re-ruled form (the main session copies its two notes onto the lane
  at the same paths and saves; ffcheck 35 ADDED and 10 differing; the builder, resumed in a window, `git add`s those 45,
  then `git merge --ff-only`, then `git status --short` empty; the evidence kept first by `cp -a`, `diff -rq` and a
  sha256 list; then a plain remove; any refusal: it stays until PR #40's close), in a later window.
- M9, the builder worktree, the main session's part DONE 08:4x-08:5x UTC on 11-27:
  - The two notes copied onto the lane at their worktree paths (`cp`, each its own call; sha256 a65282fa...a34c and
    d073228f...9c93, equal to the worktree's and the wip copies) and saved: 0efbbef.
  - ffcheck 80b6748..0efbbef: exactly 35 ADDED present (the two notes and U7's 33) and 10 differing (U7's 10), nothing
    else; 68 modified paths present, 68 hash-object ids read. `git diff --name-only 6968215 0efbbef -- spikes/` lists
    only U8-F1's six, none of U7's 43, so the 45 staged paths will equal 0efbbef.
  - The keep-first copy into /home/user/Shamble-Frontlines/.gate/u7/kept-agent-a9873/ (`cp -a` of the worktree's
    .gate/u7/ and of .gate/netcode-proof-test/t5-dist/, dist-bites/ and failed-steps/, each its own call; nothing
    moved); `diff -rq` of each original and copy printed nothing; u7-kept-list.md holds the one `find … -exec sha256sum
    {} +`, 332 lines for 332 files. It was copied by `cp` from the tool's own saved copy of that output (58.4 KB), not
    rewritten by Write: the same bytes, a departure from the form's letter, named to A. Run 3's and run 4's
    summary.json copied into the notes as u7-t5ii-run3-summary.json (b17d2201...) and u7-t5ii-run4-summary.json
    (100b27d1..., as the report cites), both read whole first: hashes, local paths and loopback addresses only.
  - Left, and new: the fast-forward 80b6748..0efbbef also brings MAIN-68's kit and CI files into the builder's worktree
    (.claude/hooks/routine-guard.mjs and its test, .github/workflows/toolchain.yml: main's reviewed versions). The
    rule says agents may not edit kit files, and A's form did not weigh this. So A rules it first (the 10:50 window);
    the builder's three commands go only on A's yes; else the form's fallback (b). Then the plain remove.

## Risks
- M5's counts move with MAIN-68 and Builder 2: each bar comes from this merge's own base run on its own H0 (MAIN-68's
  Q3 (a)), never another merge's numbers.
- Disk: after MAIN-68's light runs (and Builder 2's whole run, if first), the cache may be warm; M0's 8,192 MiB bar with
  the part-clean covers it. M5 builds the web half and both workspaces' tests (about 20-45 minutes).
- U8-F1's builder, if resumed after his U8F1-SIZE answer, works in its own worktree on ops/ files (r1.sh and its tests),
  none of U7's 43. Its merge comes before or after U7's, one at a time; if first, M5's ops count is its count.
- #73's rules: a refusal or an ask stops the step where it is (M6's form if after M4).

## Questions for the checker (answered: advisor-u7-merge.md, "The plan's questions": Q1 UM-4; Q2 enough; Q3 right)
- Q1: the `ops` step as the first change after this merge (out of it, so M6's undo works and the commit equals the
  snapshot), carrying U8's fix 2 install. Rule on that change's form and its dependency record, or say it waits for
  Stop B's fill.
- Q2: C1 through u7-c1.mjs (u8-c1.mjs with the four run-4 changes named in Facts): enough, or should the script take
  the folder as an argument?
- Q3: M2b's header call runs `true` through u7-run.sh in the builder's worktree (no build): the right form for U7-M-3,
  and under #73's rules (no git or gh word on the line)?
