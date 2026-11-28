# Advisor: NB2-ONEWAY merge plan check (nb2b-merge-plan.md), 2026-11-27
Read only: no cargo, no git write, no fetch. Rules: `git show origin/main:.claude/agents/advisor.md`.
Builder 2's tip at sending: 29a901994f5b65c23a17162422770aaacc0ca44e. Main at sending: b8f02b0 (#78; MAIN-69 owed).

## My plan (blocks, each written when done)
1. Rules and words: advisor.md on main (changed since my last read?); his word of 12:25 (history.md:8133-8151) and of
   13:55 (history.md:8163-8186), word for word.
2. The plan: read whole; its "Since the draft" section and Q1-Q4; its sha256.
3. The tip's facts, by SHA: the tip's ancestry against the merged 48efc9c (Builder 2 took the merged head in?); the
   merge base with HEAD; the diff merge-base..tip (paths, stat, modes, guard pathspec, manifests); which commits are
   code and which notes; the trial merge on HEAD; overlap with this branch since the base.
4. What the code change touches (crates/net? the probe's one-way checks; marks/SERIES?) and whether the "same steps and
   bars" fit: the counts, refs, redact; the re-judge (does a -47-style re-judge apply?); the reviewers' reads (his
   2026-11-15 rule if the judge or marks.json changes).
5. The order: MAIN-69 first; "never overlap"; R1's path; the gap.
6. Q1-Q4.
7. Verdict, conditions, findings, Decisions needed, For Mustafa, Status line.

## Blocks

### Block 1: the rules and his words (done)
- advisor.md on main: last changed c9bd268 (11-12), the version this seat read and follows.
- Plan nb2b-merge-plan.md: sha256 3f2ea2e468b3918969b4f1e44c36a904b87b1bf99da25aa51f703418352f1641 (109 lines,
  "Since the draft" and Q4 included).
- His 12:25 word (history.md:8133-8151) read word for word. The plan's quote of item 3 is exact. Item 2's facts are
  re-read in block 2.
- His 13:55 word (history.md:8163-8186) read word for word: item 1's limit "do not start a step that must not be cut,
  such as a merge's no-save stretch, unless the usage left covers it"; item 4 "name the command as soon as you are
  ready for it".
- His 2026-11-25 words that this unit rests on:
  - his message to Builder 2, item 4 (T:.checkpoints/netcode-judge/mustafa-2026-11-25.md): "plan first, your advisor
    agent's review, one reviewer's read, ready before R1's report. Take the netcode branch's merged head in first.";
  - his answer (a) in this chat (history.md:7906).
- HEAD ccbd53453bf0 = origin; main b8f02b0149cc; T 29a901994f5b present locally.

### Block 2: the tip's facts, by SHA (done)
- **T's history:** 48efc9c..T is this branch's own commits to 591e2cb (Builder 2 fast-forwarded to it), then Builder
  2's nine:
  - notes: 942855b, 4dfdbed, befd4e0, 6ceaa07, f87b8f8, df47b19, 29a9019;
  - code: 1a63740 (the build) and c56afd0 (the fix pass).
  - `git merge-base HEAD T` = 591e2cb0ab07d8483290c98d8f6ab8595a2621e3.
- **His condition holds:** `git rev-parse c56afd0` = c56afd07002f6c13af418c8f016d6087fe806813, his code commit. The
  three commits after it: f87b8f8 (M .checkpoints/netcode-judge.md), df47b19 (the same), 29a9019 (M netcode-judge.md,
  A netcode-judge/q-oneway-merge.md).
- **591e2cb..T** (`--numstat`, `--raw`): 12 paths, +665/-17, all 100644. Notes: netcode-judge.md 54/11 M, and five A
  (na-plant.sh 30, netcode-architect-oneway.md 42, oneway-note.mjs 44, plan-oneway.md 71, q-oneway-merge.md 45). Spike,
  all M: eval.rs 70/2, marks.rs 28/0, tests/engine.rs 263/0, marks/README.md 14/0, SERIES.md 3/3, marks.json 1/1
  (+379/-6, as the plan says).
- **The kit pathspec** (main-61-plan.md:37-40, plus tools, Cargo.toml, Cargo.lock, package*.json and rust-toolchain*)
  over 591e2cb..T prints nothing. The control (the same form plus spikes/netcode-proof/marks) prints the three marks
  files.
- **Ignore rules:** `git check-ignore --no-index -v -n` on the five new notes prints "::" for each: none is ignored.
- **marks.json:** T's blob is 84e382e7972aba928f6a9587bc74631783627794. One hunk (@@ -2368 +2368 @@): PM-0001-3's
  "notes" gains a sentence quoting him twice, both quotes exact (his item 4 of 25 Nov to Builder 2; history.md:7906).
- **This branch since 591e2cb:**
  - in crates/judge, marks and netcode-judge*: nothing (the control, ops/operator, lists PROMPT-R1.md and rules.md);
  - non-notes commits: 6b4a0d3 (MAIN-68), 6968215 (U7), ddb06ff (U8-F1), d8b57b8 (the operator rewordings).
- **Trial merge** on HEAD ccbd534: one line, tree f164a5794880205a428b80975765107ec4214c5e, no conflict (the plan's
  147dec96 was on 7b50a66).
- **What the code does:**
  - `one_way_bound` (eval.rs) returns None for every check except a statistic of place over probe_up_delay_ns or
    probe_down_delay_ns. For such a check it reads `params.base_rtt_ns` from each L2 log that carries the series.
  - marks.rs refuses a pooled or "against" one-way check.
  - judge_mark's early return for a moved mark is untouched, so the -47 re-judge reads MOVED exactly as at 48efc9c.
  - SERIES.md's three rows keep their names.
- **The writer:** HEAD's L2 probe already puts `base_rtt_ns` in its header (net/src/l2/socket.rs:338;
  server/src/probe_tests.rs:359 asserts it), so R1's L2 probe logs carry what the judge now reads.
- **Builder 2's card at T (steps 31-35):**
  - the netcode-architect's read on 1a637408 (APPROVE WITH FINDINGS, five Low);
  - the fix pass c56afd07, read by the advisor agent (APPROVE);
  - the whole check 11 of 11, test 912 and 1009 (908 and 1005 at 591e2cb's code: +4 each); 136 judge tests.
  - The fix pass itself (`--numstat 1a637408 c56afd07`): eval.rs 5/5, engine.rs 26/15, README.md 2/2.
- **Main's move** b9aad01..b8f02b0: M .github/workflows/toolchain.yml, M docs/DECISIONS.md, REPO_MAP.md, ROADMAP.md,
  STATUS.md, A docs/reports/2026-11-27-ci-time.md (#78). Trial merge of b8f02b0 into HEAD: one tree line ea1b2f9e, no
  conflict. PR #40 (gh): head ccbd534, mergeable true, mergeable_state "blocked" (a draft).
- **check.mjs at HEAD:** 12 steps (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs, marks, self, web, redact).
  The redact step scans `HERE` = spikes/netcode-proof only (check.mjs:2717).
- **UM-4's plan (rev 0.3):** step 0 pins origin/main = b9aad01 (:66, a CI move stops). Step 5 holds test, web and wasm
  strictly to U8-F1's M5 with the proviso `git diff --name-only ddb06ff HEAD -- spikes/` (:104-108). Its build waits
  for this file (launch-1550-msgs.md:18).
- **U8-F1's M5** (.gate/u8-f1-merge/m5-check.log): 12 of 12 steps; test 931 and 1028; ops tests 104 of 104 and
  "wrappers.test.sh: 315 ok, 0 failed" beside the run; self 80 of 80; web 70 of 70; redact 464 items; refs 2145.
- **The card (HEAD :9):**
  - NB2-C10 DONE (review-nb2-c10.md: APPROVE WITH FINDINGS, M1 Medium at figures.rs:722-724, Builder 2's to fix; T
    does not touch figures.rs);
  - "Still open from NB2's merge: NB2's report …, then -47's pack".
  - `ls -d` lists the -47 folder's measured and bites (unpacked).

### Block 3: the verdict, APPROVE WITH CONDITIONS NB2B-1 to NB2B-11
The plan is his word, item 3, and every fact it states holds on my own reads:
- his condition (notes only after c56afd07);
- the 12 paths and no kit path;
- marks.json's one field, quoting him exactly;
- a clean trial merge on today's head;
- an untouched -47 path.
Two bars need fixing before it runs: the redact count (NB2B-1) and when the usage is read (NB2B-2). No Critical or
High. Risk tier 1 (spike code and notes). No Brave check.
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

### Block 4: the plan's questions
- **Q1: each NB2 condition, carried or moot.**
  - **C1:** 0b moot (done 11-25). The clean-tree bar is carried: before 1g nothing outside .checkpoints/, and the
    main-run.sh headers of 1i, 4 and 5 read the same empty hashes. The count base is carried as Q3 rules.
  - **C2:** carried, with his item 1's limit (NB2B-2). Re-project after 1f, cold if a part-clean runs (use UM-4's
    counted run's time). It must fit before the next R1-path step, one merge at a time (MAIN-69, UM-4 and this one
    never overlap).
  - **C3:** carried. Name the command, set the board line, end the turn, send only after his reply. NB2B-11 holds the
    prompt's terms.
  - **C4:** carried as written:
    - (a) step 5 not MOVED: three lines to him, nothing undone;
    - (b) a stop in 3, 4 or 6: (i) by default; (ii) only by its terms, Builder 2 told to hold;
    - (c) and (d) the stop and resume lines.
  - **C5:** carried (the card's Tools line form, outside main-run.sh).
  - **C6:** carried word for word: both -47 lines as last time (3 runs, then 2), exit 0, "verdict": "MOVED",
    `env CARGO_INCREMENTAL=0`.
  - **C7:** (a) carried as NB2B-3. (b) carried: 1c's exact 12-path list proves no kit path; my pathspec run with its
    control agrees. (c) carried as NB2B-1 (redact + 0, refs + 5, self as 1i's).
  - **C8:** moot. His item 3 ("If the tip has moved, send no command and two lines to me") replaces it, as NB2-PROMPT
    (a) did.
  - **C9:** carried as a risk line. SERIES.md's three rows keep their names, so the name checks cannot newly fail.
    marks.json changes one notes field, which the room, server and check.test.mjs readers do not judge. Step 4 covers
    the rest.
  - **C10:** carried as Q2.
  - **C11:** moot here (no -57 line in this unit). NB2's report still owes it (NB2B-9).
  - **C12:** moot (NB2's text). This plan's own text fixes are NB2B-1, -2 and -5.
- **Q2: who and when.**
  - A short read-only test-engineer read of `git diff 1a637408671126d6306500b33d5496257f477c75
    c56afd07002f6c13af418c8f016d6087fe806813 -- spikes/netcode-proof/crates/judge spikes/netcode-proof/marks`. That is
    the fix pass, +33/-22: what the netcode-architect's read did not cover.
  - It runs before this merge's report, not before the push: the merge changes no byte of it, and the rule's deadline
    is the unit's close.
  - The plan's whole-diff read (591e2cb..T) is allowed but not needed: the netcode-architect already read 1a637408
    whole, marks.json's line included.
  - Not a review round. His 25 November item 4 set "one reviewer's read" for the build, and this is the 2026-11-15
    rule's read of the later change.
- **Q3: bars on UM-4's counted run: right.**
  - The bars are UM-4's numbers + 4 each for test (935 and 1,032 if UM-4 reads U8-F1's M5 931 and 1,028); every other
    step as UM-4's (13 steps); refs N0 + 5; redact 1i's + 0; self as 1i's (UM-4's).
  - **If UM-4 has not landed:** this merge may go first only if UM-4's step 0 has not begun. Once UM-4 starts, its
    no-save stretch and uncommitted edits hold the main folder until its push, and this merge waits.
  - This merge's bars would then be U8-F1's M5, read on MAIN-69's merged state (MAIN-69 brings no spike file):
    12 steps; test 935 and 1,032; web 70 of 70; wasm as M5; the ops tests (104 of 104) and wrappers (315 ok) beside
    check.mjs (choice 15 (i)); refs N0 + 5; redact 1i's + 0; self as 1i's.
  - UM-4's step 5 bars would then be restated in um4-plan.md before UM-4's step 0, in its own "named first" form:
    `git diff --name-only ddb06ff HEAD -- spikes/` names this merge's six paths beside d8b57b8's three, and test = U8-F1's
    M5 + 4 each. Web, wasm and ops stay strict. That is a value restatement (as NB2's departure 1), named in the day
    message: no new check.
- **Q4: the order. The default carries less risk: MAIN-69, then UM-4, then this merge.**
  - UM-4 is R1's path ("only R1's path"; his 12:25 item 1 freed its build). This merge is due only "before R1's
    report".
  - UM-4's approved bars stay as approved, strict against U8-F1's M5. This merge's bars are relative and ruled here
    for either order.
  - This merge's whole check then also runs UM-4's new `ops` step on a second tree.
  - One thing moves at a time.
  - Going first saves a wait only if UM-4 is held before its step 0 (say, its npm install refused at the proxy, or an
    answer pending) for longer than this merge's stretch. Then Q3's terms apply.
  - In either order, after MAIN-69, UM-4's step 0 pin (um4-plan.md:66, b9aad01) is re-read as MAIN-69's main
    b8f02b0. That is UM-4's own value line; its rule stops on a move, and #78 is taken in by MAIN-69 first.

### Block 5: findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-NB2B-1 | Medium | Step 4's redact bar "+ 5 (the five added notes)": redact scans only spikes/netcode-proof, so a correct run reads + 0 and would stop. | NB2B-1 |
| ADV-NB2B-2 | Medium | Step 1j reads the usage at H0 only, but the stretch that must not be cut starts at step 2's send, after a wait for his reply of unknown length (his 13:55 item 1's limit). | NB2B-2 |
| ADV-NB2B-3 | Low | The tree anchor (NB2-C7 (a)) is not written into steps 1h and 3. | NB2B-3 |
| ADV-NB2B-4 | Low | Step 8's "as before" could carry the last merge's push-only CI reading; PR #40 is mergeable now, so both events count. | NB2B-4 |
| ADV-NB2B-5 | Low | Step 4 names no web or deps bar. The 136's first route (the log's per-crate lines) does not exist for a passing step. | NB2B-5 |
| ADV-NB2B-6 | Low | Step 6's gate output: no kept file (as ADV-NB2H-1). | NB2B-6 |
| ADV-NB2B-7 | Low | The card queues "-47's pack" after NB2's report; this merge's step 5 needs the folder unpacked. | NB2B-7 |
| ADV-NB2B-8 | Low | The reviewer read's scope: the plan's whole diff duplicates a done read; the needed scope is the fix pass. | NB2B-8, Q2 |
| ADV-NB2B-9 | Low | NB2's report is still owed while a second merge of the same lane starts. | NB2B-9 |
| ADV-NB2B-10 | Low | 1g's pgrep pattern is wider than last time's; 1e2 uses a pipe where a printing git form exists. | NB2B-10 |

### Block 6: what I did not check
- I ran no cargo, test, gate, light run or merge, and no fetch: T is the locally present 29a90199 (the coordinator
  read 14:0x). The trial merge is on today's head. MAIN-69's and UM-4's heads do not exist yet (1e re-reads it there).
- The plan's 12:4x permission refusal of the pathspec form: not reproduced. The same read-only form ran here without a
  prompt, control included.
- The netcode-architect's read and Builder 2's 136 tests: taken from Builder 2's card and notes, not rerun.
- UM-4's counted run's numbers do not exist yet. Q3's bars are relative to them.

## Decisions needed
None. The order is the main session's ("Take the next gap that fits"). The bar restatements are values. Nothing here
changes a rule or a word of his.

## For Mustafa
1. When the netcode chat names Builder 2's second merge command (`git merge --no-ff -F
   /home/user/Shamble-Frontlines/.gate/nb2b-merge/merge-msg.txt 29a901994f5b65c23a17162422770aaacc0ca44e`), reply in the
   chat when you are at your screen. Then approve the prompt only if it shows exactly that one command.
2. Nothing to check in Brave.

Status: COMPLETE 2026-11-27T14:16Z
