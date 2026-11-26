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
(none yet)

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
