# NB2-ONEWAY: take Builder 2's one-way delay work into the netcode branch (plan, 2026-11-27 12:4x UTC)
CHECKED 14:16 UTC: APPROVE WITH CONDITIONS NB2B-1 to NB2B-11 (advisor-nb2b-merge.md, last line "Status: COMPLETE
2026-11-27T14:16Z"; NB2B-1 and -2 Medium: redact bar base + 0, not + 5; the usage re-read just before step 2's send,
after his reply). Its conditions govern wherever this file differs; they are applied here by Edit before step 1. Q4: MAIN-69,
then UM-4, then this merge. Q2: a test-engineer read of `git diff 1a637408 c56afd07` over the judge and marks folders,
before the report.
Status was: DRAFT, for the advisor agent's check in the 15:50 window (a10848a10f73ed39e, the seat that wrote NB2-C1 to
-C12; launch-1550-msgs.md, section NB). Nothing is run before that check. A delta on nb2-merge-plan.md: its steps,
numbers, stop rules and risks hold except where this file differs. The order (the advisor tool, 12:3x): UM-4's build
first, after the 15:50 seats finish; this merge after UM-4's push, in the next gap that fits, before R1's report.

## Since the draft (14:0x UTC on 11-27, before NB's check)
- His message of 13:55 UTC (history.md:8163-8186, word for word there). Item 1, NO WAITING FOR WINDOWS: the next step
  starts whenever usage allows, with "One limit: do not start a step that must not be cut, such as a merge's no-save
  stretch, unless the usage left covers it." Item 4: "Builder 2's merge: name the command as soon as you are ready for
  it. It does not have to wait for the evening." So the 15:50 seats run now (pulled forward), and this merge's command
  is named as soon as the steps before it allow, not about 18:30.
- Main moved to b8f02b0149cc928964e54e194cd9c4d68f52bd94 (#78 ci-time: .github/workflows/toolchain.yml
  timeout-minutes 15 to 25, and docs/DECISIONS.md, REPO_MAP.md, ROADMAP.md, STATUS.md and
  docs/reports/2026-11-27-ci-time.md; no manifest or lockfile; `git merge-tree --write-tree --name-only HEAD b8f02b0`
  one tree line, no conflict). MAIN-52 takes it in first: MAIN-69, a full take-in (CI is outside TAKEIN-CODE (b),
  his 13:55 item 2, and outside light-takein-recipe.md:9-10's limits), plan first with its check. UM-4's step 0 pins
  main (um4-plan.md:66), so UM-4 follows MAIN-69. This merge's H0 then sits on MAIN-69's merge or UM-4's commit:
  steps 1d, 1e and 1i are re-read there (1d's merge-base stays 591e2cb while T stays 29a90199, as T holds no main
  commit after 591e2cb: its whole list above).
- Builder 2's tip re-read at 14:0x: 29a901994f5b65c23a17162422770aaacc0ca44e (unchanged).
- New step 1j (his item 1's limit): just before H0, the session's rate-limit event read (utilization and resetsAt);
  the stretch from H0 to step 7's push starts only if the usage left covers it; the figure and the go or no-go on the
  card's resume line. A no-go waits for the reset, nothing begun.
- Q4 (new): the order. Default: MAIN-69, then UM-4, then this merge (step 4's bars on UM-4's counted run, as Q3). May
  this merge go right after MAIN-69, before UM-4 (step 4's bars then U8-F1's M5 numbers as re-read on MAIN-69's merged
  state, and UM-4's bars from this merge's run), if that saves a wait? Which order carries less risk?

## As checked: NB2B-1 to NB2B-11 (advisor-nb2b-merge.md, block 3), word for word; they govern the steps below
- **NB2B-1 (Medium): step 4's redact bar is 1i's items + 0, not + 5.**
  - The redact step walks spikes/netcode-proof only (check.mjs:2717; redact-check.mjs:277-288).
  - The five added files are all under .checkpoints/, and the six spike files are modified, none added.
  - As written, a correct run would stop at step 4. refs stays N0 + 5 (`git ls-files` counts every tracked file).
  - Met: step 4's bar line.
- **NB2B-2 (Medium): the usage read (step 1j) is repeated just before step 2's send, after his reply.**
  - The stretch that must not be cut starts at the merge commit and ends at step 8's push.
  - His reply can come long after H0, so a reading taken only at H0 can be stale.
  - The read must cover a cold step 4 (1f is likely under 8,192 MiB: 8,008 at 12:4x, and UM-4's run first), the two
    -47 calls, the gate and the push.
  - A no-go after his reply: no command, one line to him, and the command waits for the reset (H0 is pushed: nothing
    to undo).
  - Allowed and better: name the command after 1f, then run 1g-1j on his reply, so no save is held while he answers.
  - Met: 1j, step 2.
- **NB2B-3 (Low): the tree anchor (NB2-C7 (a)), written in.** After 1h, run `git merge-tree --write-tree --name-only H0
  T`: one line, tree X. At step 3, `git rev-parse HEAD^{tree}` = X. Met: 1h, 3.
- **NB2B-4 (Low): step 8's CI bar is advisor-main-65.md §4 (a) whole again.**
  - Push and pull-request runs on the pushed SHA, all success, and PR #40 mergeable true.
  - The last merge's push-only reading lapsed with MAIN-68: PR #40 reads mergeable true now.
  - A pull-request run that is red or never starts is not green.
  - Met: step 8.
- **NB2B-5 (Low): step 4 names every bar, and the 136 is read for real.**
  - web as UM-4's (70 of 70); deps passes; wasm, ops and wrappers as UM-4's.
  - check.mjs's log keeps no per-crate lines for a passing step. So the judge's 136 comes from one main-run.sh call,
    `env CARGO_INCREMENTAL=0 cargo test --locked -p npt-judge --no-fail-fast`, into
    /home/user/Shamble-Frontlines/.gate/nb2b-merge/judge-tests.log: 19 + 4 + 87 + 26 passed, 0 failed.
  - Met: step 4.
- **NB2B-6 (Low): step 6's gate output is kept** as /home/user/Shamble-Frontlines/.gate/nb2b-merge/gate.txt (by Write
  from the call's output). Last time only the claim was kept (ADV-NB2H-1). Met: step 6.
- **NB2B-7 (Low): the -47 folder stays unpacked until this merge's step 5 is done.** The card's "then -47's pack"
  (after NB2's report) waits for it too. Met: the card's queue.
- **NB2B-8 (Low): Q2's reviewer read** (section below), before this merge's report. The report's own line under his
  2026-11-15 rule names what covered each part:
  - the netcode-architect read 1a637408 (his "one reviewer's read");
  - the advisor agent read c56afd07;
  - this read covers the fix pass;
  - NB2-C10's M1 (figures.rs:722-724) is not in this tip and stays Builder 2's forward fix.
  - Met: before the report.
- **NB2B-9 (Low): NB2's own report goes out before this one, or with it, in separate blocks.** It is still owed (card
  :9), with advisor-nb2-head.md's C10 and C11 lines. Met: the reports.
- **NB2B-10 (Low): printing forms.**
  - 1g: `pgrep -c -f "checkpoint.mjs watch"` printing 0, as the last merge's narrower pattern.
  - 1e2 may use `git rev-parse 29a901994f5b65c23a17162422770aaacc0ca44e:spikes/netcode-proof/marks/marks.json` printing
    84e382e7972aba928f6a9587bc74631783627794, in place of the pipe. Step 3's on-disk `sha256sum` against a41ee583…
    stays.
  - Met: 1e2, 1g.
- **NB2B-11 (Low): his NB2-PROMPT (a) terms, carried by "the same steps".**
  - The prompt must show one `git merge --no-ff` of 29a901994f5b65c23a17162422770aaacc0ca44e and nothing else, in the
    named -F form.
  - The tip is re-read just before the send. Moved: no command, two lines to him (his item 3).
  - Met: step 2.
- Since NB's check (the main session, 14:4x): MAIN-69's merge (on c795079) comes under this merge's H0 with F1's
  c16777e, and UM-4 before it (NB's Q4); steps 1d, 1e and 1i are re-read on that H0, as "Since the draft" says.

## His word (history.md:8133-8151, word for word there)
- Item 3: "merge that branch at exactly that tip, by the same steps and bars as the last merge (the command named
  first and my reply to it, afterwards the whole check). Take the next gap that fits, before R1's report. If the tip
  has moved, send no command and two lines to me."
- So T is 29a901994f5b65c23a17162422770aaacc0ca44e exactly. Any other tip at step 1 or just before step 2's send: no
  command, two lines to him (as NB2-PROMPT (a): no other route).
- Item 2's facts, each re-read below; his main Advisor checked the code commit c56afd07; "Not run by my Advisor: its
  136 judge tests" (step 4 runs them).

## Facts checked (12:3x-12:4x UTC, read only, printing forms)
- `git fetch origin claude/practical-ride-igo41n`, then `git rev-parse FETCH_HEAD`: 29a901994f5b65c23a17162422770aaacc0ca44e.
- `git merge-base HEAD 29a90199…` (HEAD 7b50a66): 591e2cb0ab07d8483290c98d8f6ab8595a2621e3.
- After the code commit: `git rev-list --count c56afd07..29a90199` printed 3; `git diff --name-only c56afd07 29a90199`
  printed only .checkpoints/netcode-judge.md and .checkpoints/netcode-judge/q-oneway-merge.md ("only its notes").
- `git diff --name-status 591e2cb 29a90199`: 12 paths, the whole list. Six notes: .checkpoints/netcode-judge.md (M);
  na-plant.sh, netcode-architect-oneway.md, oneway-note.mjs, plan-oneway.md, q-oneway-merge.md under
  .checkpoints/netcode-judge/ (A). Six spike files, all M (`--numstat`): crates/judge/src/eval.rs 70/2,
  crates/judge/src/marks.rs 28/0, crates/judge/tests/engine.rs 263/0, marks/README.md 14/0, marks/SERIES.md 3/3,
  marks/marks.json 1/1 (+379/-6). No kit, CI, gate, manifest or lockfile path is in that whole list. (A pathspec form
  over the kit paths was refused at its permission prompt, 12:4x, and not retried; the whole list carries the answer.)
- marks.json: `git show 29a90199:spikes/netcode-proof/marks/marks.json | sha256sum` printed
  a41ee58398274840c20eac3f09ab144fe8bc84dda5f7c7233fd5fd7589ea24f2 (his). `git diff 591e2cb 29a90199 -- …/marks.json`:
  one hunk (:2365-2371), one line, PM-0001-3's "notes", which gains the NPT-L2-ONEWAY text (his 2026-11-25 words to
  Builder 2 and his answer (a)) and the pointer "marks/README.md 2.3"; the next object is PM-0001-4. No other field.
- This branch since 591e2cb in those folders: `git diff --name-only 591e2cb HEAD -- spikes/netcode-proof/crates/judge
  spikes/netcode-proof/marks .checkpoints/netcode-judge.md .checkpoints/netcode-judge` printed nothing; control, the
  same form over ops/operator: PROMPT-R1.md and rules.md.
- `git merge-tree --write-tree --name-only HEAD 29a90199`: one line, tree 147dec9688e561e4e112845096584938907dc1c3, no
  conflict.
- Builder 2's card at T (.checkpoints/netcode-judge.md:241-249): its whole check on its head (based on 591e2cb): 11 of
  11, test 912 and 1009, 0 failed, 0 skipped; the judge's own 136 tests (19 + 4 + 87 + 26), clippy and fmt exit 0.
  NB2's first merge read 908 and 1005 (its step 4), so this merge adds 4 and 4.
- The -47 set: `ls -d` lists …/runs/u4f-47-b57221ed139d/measured and …/bites (still unpacked).
- Free disk: 8,008 MiB (12:4x), under the 8,192 bar.

## Steps (nb2-merge-plan.md's numbers; only what differs; each command its own call, printing forms only)
1. a. T as above, else no command and two lines to him.
   b. `git merge-base c56afd07 T` prints c56afd07… (replaces `--is-ancestor` exit 0); `git rev-list --count
      c56afd07..T` = 3; `git diff --name-only c56afd07 T` only Builder 2's notes. Anything else: no merge, two lines
      to him.
   c. `git diff --name-status 591e2cb T`: exactly the 12 paths above; more: stop, the advisor rules.
   d. `git merge-base HEAD T` = 591e2cb0….
   e. merge-tree: one tree line, no conflict.
   e2. marks.json at T: the pipe hash a41ee583…, and its diff against 591e2cb the one hunk above; else stop, to him.
   f. Free disk at least 8,192 MiB: the approved dev part-clean first (UM-4's run leaves it under again), measured.
   g. The saver stopped by TaskStop, then `pgrep -c -f checkpoint.mjs` printing 0 (replaces `pgrep -f` exit 1).
   h, i. As before: the card's resume line, H0, then `mkdir -p /home/user/Shamble-Frontlines/.gate/nb2b-merge` and the
      base light run into /home/user/Shamble-Frontlines/.gate/nb2b-merge/light-base.log (refs, marks, self, redact).
      Every file a shell command writes is named by its full path (his FULL-PATHS rule, 11-27, history.md:8155); the
      merge message file is written by Write.
2. The command named to him first, the board line set, the turn ended, his reply awaited (NB2-PROMPT (a)); T read
   again just before sending: `git merge --no-ff -F /home/user/Shamble-Frontlines/.gate/nb2b-merge/merge-msg.txt
   29a901994f5b65c23a17162422770aaacc0ca44e`, the message file naming his word (history.md:8133-8151), T and c56afd07.
3. Content checks: `git rev-parse HEAD^1 HEAD^2` = H0 and T; `git diff --stat H0 HEAD` lists the same 12 paths and
   counts as `git diff --stat 591e2cb T`; `git diff --name-only T HEAD -- spikes/netcode-proof/crates/judge
   spikes/netcode-proof/marks .checkpoints/netcode-judge.md .checkpoints/netcode-judge` prints nothing, with the
   control `git diff --name-only 591e2cb T -- spikes/netcode-proof/marks` listing the three marks files;
   `sha256sum spikes/netcode-proof/marks/marks.json` = a41ee583….
4. The whole check, the part-clean first if under 8,192, counted as cold: `bash .checkpoints/netcode-proof-test/main-run.sh
   /home/user/Shamble-Frontlines/.gate/nb2b-merge/check.log env CARGO_INCREMENTAL=0 node check.mjs`. Bars, on UM-4's
   counted run (its push lands first): every step passes (13 with UM-4's `ops`); test = UM-4's + 4 each (931 + 4 =
   935 and 1,028 + 4 = 1,032 if UM-4 reads U8-F1's M5 numbers); 0 failed, 0 skipped; the judge's 136 (19 + 4 + 87 +
   26) read from the log's npt-judge test lines, or, if the log does not show them, one `cargo test --locked -p
   npt-judge` through main-run.sh; marks 81 read, rows unchanged; self as UM-4's; ops and wrappers as UM-4's; redact 0
   findings, its items step 1i's + 5 (the five added notes); refs step 1i's N0 + 5; wasm as UM-4's.
5. The -47 re-judge, kept ("the same steps"), as before: both calls MOVED.
6-9. As before: gate PASS; nothing edited by hand; the save pushes the merge, CI on the pushed head (gate, smoke and
   toolchain green, PR #40 with no conflict); the saver restarts; Builder 2 is told the merged SHA; the advisor's read
   of the pushed head in the next window; and a reviewer's read of `git diff 591e2cb T -- spikes/netcode-proof/crates/judge
   spikes/netcode-proof/marks` before the merge's report (his rule of 2026-11-15, history.md:2127; NB2-C10's form).

## Change type, tier
A merge of another lane's branch, its code commit checked by his main Advisor, nothing edited by hand: 12 files, six
of them spike files in the judge's code and marks/ (+379/-6). Risk tier 1 (spike code and notes). No Brave check.

## Done lines
- Step 1 re-reads, 16:30-16:31 UTC on HEAD 787eb984 (UM-4's commit 8fb32bc8 pushed; its gate runs 46034034986 and
  46034043528 success, the two toolchain runs still running), each its own call, printing forms:
  - 1a: `git fetch origin claude/practical-ride-igo41n`, `git rev-parse FETCH_HEAD`: 29a901994f5b65c23a17162422770aaacc0ca44e.
  - 1b: `git merge-base c56afd07 T`: c56afd07002f6c13af418c8f016d6087fe806813; `git rev-list --count c56afd07..T`: 3;
    `git diff --name-only c56afd07 T`: .checkpoints/netcode-judge.md and .checkpoints/netcode-judge/q-oneway-merge.md.
  - 1c: `git diff --name-status 591e2cb T`: the 12 paths above, exactly.
  - 1d: `git merge-base HEAD T`: 591e2cb0ab07d8483290c98d8f6ab8595a2621e3.
  - 1e: `git merge-tree --write-tree --name-only HEAD T`: one line, tree 17461528611fbb5d2348548bbab81083bd30e6dd, no
    conflict (12:4x's 147dec96 was on HEAD 7b50a66; the tree anchor X is taken on H0 at 1h, NB2B-3).
  - 1e2: `git rev-parse T:spikes/netcode-proof/marks/marks.json`: 84e382e7972aba928f6a9587bc74631783627794 (NB2B-10);
    `git diff -U0 591e2cb T -- …/marks.json`: one hunk, line 2368, PM-0001-3's "notes" gaining the NPT-L2-ONEWAY text
    and "marks/README.md 2.3".
  - 1f: 7,946 MiB free (under 8,192); `pgrep -c` cargo, rustc, npt- each 0; the dev part-clean (the card's Tools form,
    `--locked`): "Removed 5467 files, 7.0GiB total"; `ls -la` lists .lane.lock and CACHEDIR.TAG, no debug; 14,491 MiB
    free, so step 4 runs cold.
  - Usage 16:31: utilization 0.21, resetsAt 1795812600 (20:50 UTC). Step 2's naming after the four CI runs on
    787eb984 read success; 1g-1j and NB2B-2's re-read on his reply.
- Step 2 named 16:4x UTC (T re-read just before: 29a90199; board tags needs-musti, due 17:30 UTC). His reply "yes"
  17:03:02 UTC (history.md, transcript line 116029, msgcheck3 "history.md holds it exactly: true"; event ee06aedb).
- On his reply, 17:03-17:06 UTC: NB2B-2's usage read: 0.23 at 16:39:14 (the newest), resetsAt 20:50, so GO (0.77 left
  against about 0.2); the go line on the card. 1g: TaskStop b3rvvohgw; `pgrep -c -f "checkpoint.mjs watch"` printed
  0. 1h: the saver's own last save on termination, d24340b7 "autosave (terminated)", carried the card's resume line and
  history.md's entry, so the explicit save printed "nothing to save"; H0 = d24340b74ce6f5b211c670cf0f705b155b78f93c =
  `git ls-remote` = `git rev-parse HEAD`. 1i: light-base.log at H0, exit 0: refs 2166 (N0), marks 81, self 85 of 85,
  redact 11 of 11 and 464 items, 0 findings; free 14,487 MiB. NB2B-3: `git merge-tree --write-tree --name-only H0 T`
  one line, X = dd5973f4841187a124aa2d6b09d09c3ab207e97e. T re-read: 29a90199.
- The merge, 17:06 UTC: `git merge --no-ff -F /home/user/Shamble-Frontlines/.gate/nb2b-merge/merge-msg.txt
  29a901994f5b65c23a17162422770aaacc0ca44e`: "Merge made by the 'ort' strategy.", 12 files, +665/-17.
- Step 3, all hold: `git rev-parse HEAD^1 HEAD^2` = H0 and T; `git rev-parse HEAD^{tree}` = X; `git diff --stat H0
  HEAD` = `git diff --stat 591e2cb T` (the same 12 paths and counts, +665/-17); `git diff --name-only T HEAD -- …judge
  …marks .checkpoints/netcode-judge.md .checkpoints/netcode-judge` printed nothing, the control over marks/ listing
  README.md, SERIES.md and marks.json; `sha256sum …/marks/marks.json` = a41ee583….
- Step 4, 17:20:52-17:45:01 UTC (the log's lock line: lock taken 17:20:52, idle after 0 s; launched after the worker
  restart, not 17:07 as first written here), /home/user/Shamble-Frontlines/.gate/nb2b-merge/check.log (background
  bzbs6nbmq; no part-clean: 14,487 MiB free), HEAD cb600edee64b74a7e3bf82bd85791b61058d28bd (the merge), exit 0, "check:
  all 13 steps pass". Bars, all met: test 935 and 1,032 (UM-4's 931 and 1,028 + 4 each), 0 failed, 0 skips; refs 2,171
  = N0 2,166 + 5; redact 11 of 11, 464 items = 1i's 464 + 0 (NB2B-1), 0 findings; marks 81, rows unchanged; self 85;
  web 70 of 70; ops 104 of 104 and "315 ok, 0 failed"; wasm 27,000 ticks, 0 mismatching, the bite caught as before;
  fmt, clippy, clippy-wasm32, doc, deps PASS. The tree after it: only this file modified.
- NB2B-5, 17:45:42-17:45:51 UTC: `env CARGO_INCREMENTAL=0 cargo test --locked -p npt-judge --no-fail-fast` through
  main-run.sh into /home/user/Shamble-Frontlines/.gate/nb2b-merge/judge-tests.log, exit 0: lib 19, cli 4, engine 87,
  fixtures 26 passed (136), 0 failed, as Builder 2's card.
- Step 5, 17:46:10-17:46:31 UTC: the -47 folders listed by `ls -d` (still unpacked, NB2B-7); 7,807 MiB free (above
  main-run.sh's 4 GiB). Both calls (env CARGO_INCREMENTAL=0, the first merge's C6) into rejudge-47-measured.log and
  rejudge-47-bites.log, each exit 0, each headline "PM-0001-47 MOVED moved to Slice 1 change-set 7 (Lag) …" (3 runs
  and 2 runs listed: counted, not judged); both out files `"verdict": "MOVED"`. The -47 pack may now go (NB2B-7).
- Step 6: `node tools/gate.mjs --no-smoke` GATE PASS, base b8f02b0 (origin/main); output kept whole as
  /home/user/Shamble-Frontlines/.gate/nb2b-merge/gate.txt (NB2B-6). Step 7: nothing edited by hand (`git status
  --short` lists only this file).
- Step 8, 17:49-18:00 UTC: the save pushed b81061cf3884cd249ee4d8765814f7643cedd8ae (no skip marker) on the merge; the
  saver bsiqyfwt6 restarted. CI on b81061cf, all four success (NB2B-4): gate 46044847948 (push) and 46044855530 (pull
  request), toolchain 46044848099 (push) and 46044855642 (pull request); PR #40 mergeable_state "blocked" (draft and
  branch protection), not "dirty": no conflict. Builder 2 told the merged SHA through his For Mustafa list (18:0x).
  Step 9: B a3de649cd25e474df resumed 18:0x for both pushed heads (advisor-um4-nb2b-head.md).

## Questions for the checker
- Q1: each of NB2-C1 to -C12 (advisor-nb2-merge.md): carried here, or moot (and why)?
- Q2: the reviewer's read of the judge and marks diff: who, and when (before the report, or before the push)?
- Q3: step 4's bars on UM-4's counted run, as the order puts UM-4 first: right? If UM-4 has not landed when a gap
  comes, may this merge go first on U8-F1's M5 numbers (then UM-4's bars take this merge's run)?
