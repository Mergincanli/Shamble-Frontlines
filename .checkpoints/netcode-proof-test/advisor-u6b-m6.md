# Advisor: U6b's merge, step M6 (a failing test at M5), 2026-11-24

Instructions: main's `.claude/agents/advisor.md` (`git show origin/main:.claude/agents/advisor.md`), read at the start.
My earlier ruling: advisor-u6b-fix.md (APPROVE WITH CONDITIONS C1-C2, for the head whose spikes/ and docs/ diff
against 21765e3 equals wip/u6b-fix.patch e29c003c…ac64af1c). Risk tier 1 (spikes/). Read only; no cargo run by me.

## 1. Plan
1. Inputs: the patch's hash; the applied tree's binary.rs (main folder) against the builder's (byte-equal claim);
   m5-check.log's header and steps; failed-steps/test.log at the failure (:2219-2249) and the run's timing lines;
   fx-check1/2.log's test lines; u6a-r2-test.md :60-115 (TR2-U6A-4); u6a-report.md :66; u6b-report.md (its M5 line,
   its carries); u6b-review-test.md N7; my own T-4/F-2; m6-probe.log.
2. Item 1 (cause): what the bitten start's time is made of (the two 1 s sleeps, then U6b's start: the certificate and
   both listeners), the earlier readings of it (U6a's 2,660 ms; U6b's builder runs if printed), what the failed run
   shows (the cap hit, not a wrong OPEN), and what it does not (the start's actual time on this run).
3. Item 2 (fix): the lines in binary.rs that may change; the other caps near their margins (T-4/F-2's 5 s, N7, any
   other 5 s cap in the bites build); in this edit or carried.
4. Item 3 (evidence before merge): the probe's form, when, and a plant for the new cap.
5. Item 4 (M5 again): the disk rule from M5's own numbers.
6. Item 5 (my approval's terms): the head, what I read before M7-M9.
7. Item 6 (the carry gap): where TR2-U6A-4 is recorded and closed; the report's text.
8. Item 7 (Mustafa): the scope line and the stop, a test-margin fix in the plan or not.
9. Findings, verdict, conditions, Decisions needed, For Mustafa, DONE.

## 2. Items
**Provenance (read 2026-11-24, after M5 and the part-clean):** the main folder's `git status`: the 20 modified and 8
untracked files of the patch, nothing else under spikes/ or docs/. `git diff HEAD -- spikes/.../tests/binary.rs` is the
patch's binary.rs hunks (the U6b header line, the settings' transport fields and hash path, the exit-2 test's hash-file
removal, free_port, hash_of, test_clock, the u6b_open test); start() (:54-99) and the bites module (:414-460) are
`<base>`'s. Between 21765e3 and HEAD (8418275) one commit touches spikes/ or docs/: 07d5d31 (SEC-F26-1,
ops/operator/redact-check.test.mjs +9, test file only, 07:13 on 2026-11-23, after the builder's worktree was cut at
21765e3); the rest are checkpoint commits. So the main folder's tree = the builder's + 07d5d31's test lines (no cargo
input), and my earlier term "diff against 21765e3 equals e29c003c" reads, exactly, "the working tree's spikes/ and docs/
diff against HEAD equals e29c003c's 28 files" (item 5 words the new term that way). C1 of my fix check is met (the 20
plant diffs in the main .gate/u6b/, 04:34; their sha256 in u6b-report.md:42-63).

### (1) The cause: TR2-U6A-4's margin, crossed by U6b's heavier start in a slower run; not a defect of U6b's code
**What the evidence shows:**
- The failure is the cap, not the bite or the room's output: libtest's panic at binary.rs:82 "(Timeout)"
  (failed-steps/test.log:2226-2227, stack :2233-2236 into the bite test's `start` call at :426). A room that exited
  would have closed stdout (read_line gives an empty line, failing at :430's assert, not at the cap); a wrong first line
  fails at :430 too. So the bitten room was alive and silent for the cap's 5 s.
- The cap holds 2 s of the bite's own sleeps: main.rs:503-512 (bites builds only) sleeps 1 s and reads the settings,
  twice, before `run()`. The rest of the start is the measured start's own work (run(): machine read, tokens, Net::bind
  with the certificate, hash file and both listeners, the binary's SHA-256, the log, TickLoop, Net::serve: main.rs:699-784).
  So the bitten start has 3 s for work the measured start does in 5 s.
- U6a's reading of that margin: measured start 789.8 ms, bitten 2,660.4 ms (.gate/u6a/b3-test2.log:29, :32): about
  0.66-0.79 s of own work, a 2.34 s margin; TR2-U6A-4 (u6a-r2-test.md:67-69, :112) named the risk then.
- U6b adds to every start the certificate, the hash file, both listeners bound and served before OPEN (main.rs:542-581,
  :617-647, :723-726, :775-781), and its new dependencies grow the debug binary that `binary_sha256` reads whole at
  opt-level 0 (main.rs:368-379; Cargo.toml:117-120 raises only rand_chacha and ppv-lite86): heavier own work, in debug.
- The same 28 files passed this test in both of the builder's counted runs (fx-check1.log:151, fx-check2.log:151: bites
  904 passed). In M5 itself every other start passed: s8a's measured start under the same 5 s cap in both builds
  (test.log:909, :2217), the sibling bite pm_0006_11 (:2218) and u6b_open (:910, :2220). Only the start whose cap holds
  2 s of planted sleep crossed it.
- M5's run differed from the builder's: the main folder's npt-* crates are other package paths in the shared target, so
  the test step rebuilt them (test.log:2251-2268, "Finished `test` profile ... in 35.66s" for the bites half);
  main-run.sh exports no CARGO_INCREMENTAL (main-run.sh:17-18), where u6b-run.sh exports 0 (u6b-run.sh:24), so M5 ran
  with the dev profile's incremental default (by the script's text: m5-check.log:2 does not print it); its test step
  took 1,013.6 s and still stopped before the bites build's later binaries (m5-check.log:9, :45-46), against the
  builder's whole 848.5-863.0 s and U6a's main-folder M5 854.9 s (.gate/u6a-merge/m5-check.log:9). The bites build's
  binary.rs file took 10.03 s for 5 tests (test.log:2249), the measured build's 6.47 s for 3 (:912).
**What it does not show:**
- The bitten start's time in this run (killed at the cap; its print at :435 comes after the cap), or U6b's debug start
  time anywhere: check.mjs captures test output, and fx-starts.log's 20 starts are release (p50 19.46 ms).
- What made the run slower: no load reading; the step's 1,013.6 s includes compiling, so "19 % slower" is the step's
  wall time, not the tests'.
- That the room would have opened after 5 s. Inferred, not shown: the same tree passed twice, and the same start path
  passed three times in this run.
- That the debug binary grew, or by how much the hash costs: an inference from the code, not a measurement.
**Ruling.** The cause is the test's margin (TR2-U6A-4): the bitten start's cap counts the bite's own 2 s, and U6b's
heavier debug start under a slower run used up the 3 s left. Nothing in the evidence points to U6b's product code: the
bite's path before OPEN is two sleeps and two settings reads (a failure there exits 2 and fails at :430, not at the cap),
and every other start in the run opened. The fix is U6b's to make now: U6b's start made a known Low fire, binary.rs is
in U6b's diff, and the merge cannot pass M5 without it. **Re-rated Low → Medium (my change, in writing):** it fired in a
counted run on a correct tree and stopped the merge's M5: a real defect, small reach, no acceptable workaround (re-running
a counted check to green is not evidence). The probe (item 3) tests the residual "something else" (a slow or hung start
apart from the cap): four more starts of the bite, printed.

### (2) The fix and its form: TR2-U6A-4's fix as worded, by the main session, in the main folder
- **Who and where:** the main session, with the Edit tool, in the main folder on top of the applied patch (CLAUDE.md
  protocol step 3: implement in the main session). Not the builder: a new snapshot and a second M3/M4 buy nothing for a
  test-harness edit of a few lines, and the main folder already holds the byte-checked tree (28/28). Before the edit the
  main session copies the applied binary.rs to .gate/u6b-merge/binary.rs.pre (the patch's post-image; M4 showed it
  byte-equal to the builder's), and after it saves `git diff --no-index` of the two as .gate/u6b-merge/m6-hunk.diff with
  its sha256.
- **The lines that may change (binary.rs only; nothing else in it, no other file):**
  (a) one new constant after spawn() (:52-53): `const OPEN_CAP: Duration = Duration::from_secs(5);` with a one-line doc
      naming u6a-brief.md section 3's 5 s real-clock cap (:51, :198) and TR-U6A-7;
  (b) start(): its doc (:54-56) may say the first line is awaited at most `cap`; the signature (:63) takes
      `cap: Duration` after `settings`; the comment (:67-68) says `cap` for "5 s"; `rx.recv_timeout(cap)` (:77); the
      panic text names the cap (`{cap:?}` for "5 s", :82);
  (c) the three callers: s8a (:131) and pm_0006_11 (:448-449) pass `OPEN_CAP`; the double-load test (:426-429) passes
      `OPEN_CAP + Duration::from_secs(2)` with a one-line comment: the bite's two 1 s sleeps (server/src/main.rs:505-506)
      come on top of the start's own 5 s (TR2-U6A-4);
  (d) optional: one phrase in the module doc (:1-2) and in the bite test's doc (:419-422) naming the bitten start's cap;
  (e) rustfmt's reflow of exactly these lines.
  Unchanged: s8a's assert `open_ns < 5_000_000_000` (:137-140), the bite's `open_ns >= 2_000_000_000` (:436-439: the
  bite's evidence, untouched by the cap), every settings, assert and u6b_open line. At most 15 added lines; U6b's
  total (5,952) stays under his 6,000 stop.
- **Why this form:** the measured start's 5 s is a brief term (u6a-brief.md:51, :198) and stays; the bitten start gets
  the same 5 s for the same work plus the 2 s it plants. No bound is loosened: the start's own work is held to 5 s in
  both. The cap is a hang guard (TR-U6A-7), not the test's duration: a passing bitten run still takes about 2 s plus
  the start (the probe records it against the brief's "real-clock tests at most 5 s each", u6a-brief.md:193).
- **The other caps in U6b's tree, read:**
  - T-4 / F-2 (b4_the_writer_keeps_outstanding_within_n_snapshots_through_ws_link, ws_server_tests.rs:823-845): its
    only failing bounds are the 900-tick hang guard (:776, about 30 s at 30 Hz) and the byte asserts, which time does
    not move; the 5 s is the brief's duration rule, not an assert. It cannot fail a correct build by time. **Stays
    carried** (F-2 to U6e: its time alone).
  - N7 (u6b-review-test.md:58-60): the same test (moved by T-4) and the two ws-budget-x2 bite tests, gone with A1
    (U6e: T-4's bite half). Nothing of it in U6b's tree. **Stays carried.**
  - The measured starts (s8a :128-177, pm_0006_11 :445-459): the same 5 s cap, no sleep; their margin is what the
    probe prints (item 3's 2.5 s line). Not edited.
  - u6b_open (:261-412): its first line is read with no cap at all (:279-282), so a room that never opens hangs the
    test step (check.mjs's test runner passes no timeout: check.mjs:1922, :2052-2057), against TR-U6A-7's rule that
    start() keeps; and its two handshakes run against the room's 2 s deadline in a debug build under the file's parallel
    starts with no reading of their margin (passed in four counted runs). Not a cap near its margin but a missing one,
    with its own fix and evidence: **F-M6-3 (Low), carried to U6e**, which edits binary.rs for N4.
  - The net crates' real-clock waits (ws_server_tests.rs and wt_server_tests.rs: 5 s `until`/timeout over events that
    come in milliseconds, or over the 2 s deadline and CLOSE_WAIT with 3 s and 2 s to spare, e.g. wt_server_tests.rs:269,
    :413): inside one test binary, no process spawn; not near their margins by the bounds they test. Nothing to add.
**Item (2): the hunk above, nothing more.**

### (3) The evidence before the merge: the probe on the fixed tree, then M5; no plant
- **When:** after the edit, before M5 (M5 then runs on the probed tree; the probe's bites-build artifacts are the ones
  check.mjs's bites half uses, since `--workspace --all-features` is check.mjs's own form: check.mjs:2057).
- **Form:** one main-run.sh invocation in the main folder, log .gate/u6b-merge/m6-probe2.log (m6-probe.log keeps its
  STOP), CARGO_INCREMENTAL=0 (item 4), free at least 8,192 MiB at its start (item 4):
  `cat /proc/loadavg :: cargo fmt --all --check :: cargo test --locked --workspace --all-features --test binary --
  --nocapture` three times `:: cargo test --locked --workspace --all-features --test binary -- --nocapture --exact
  bites::pm_0001_16_double_load_fails_every_start_under_2_s :: cat /proc/loadavg`. The three runs put the file's 5 tests
  in parallel on 4 vCPUs (check.mjs's own contention for this file: its two builds run in turn, check.mjs:2052-2060);
  the last runs the bite alone. fmt first, so a reflow is fixed by Edit (rustfmt's printed diff) before M5.
- **It passes when:** fmt passes; each parallel run 5 passed 0 failed, the alone run 1 passed; the header's untracked
  hash is 4a61ef21… (the 8 new files untouched) and its diff hash differs from be52ac4e… (binary.rs edited); every
  printed s8a "spawn to OPEN" is at most 2,500 ms and every "double-load: spawn to OPEN" less 2,000 ms is at most
  2,500 ms: the start's own work inside half its 5 s budget, so a run twice as slow still passes.
- **If not:** stop, no M5, the log to me. A start's own work above 2.5 s means the measured starts' 5 s cap (a brief
  term, not this hunk's) is near its margin too, and the remedy lies outside this hunk (for example the file's
  process-spawning tests in turn under one lock, as npt-super's fake-room tests: u6a-r2-test.md:59-62); that is a new
  ruling, not a widening of this edit.
- **Readings recorded, not judged:** the four bitten times and three measured ones; the alone run's "finished in"
  against the brief's "real-clock tests at most 5 s each" (u6a-brief.md:193): above 5 s, a Low for U6e with its number,
  not a stop; both loadavg lines.
- **No plant.** The cap's property is visible in the hunk: one `recv_timeout(cap)`, the cap from each caller, 5 s of own
  work for both starts. A plant of a timing cap needs a planted delay in main.rs whose pass or fail depends on the same
  load-dependent start time the probe prints, so it would be either non-deterministic or a restatement of the probe;
  and the bite's evidence (`open_ns >= 2 s`) is not touched by the cap. The hang guard (TR-U6A-7) is read in the hunk.

### (4) M5 again, and the disk rule
- **The run:** a full `node check.mjs` through main-run.sh in the main folder on the probed tree, log
  .gate/u6b-merge/m5-check2.log, with CARGO_INCREMENTAL=0 and `cat /proc/loadavg` before and after (`::` commands).
  The header's diff and untracked hashes equal the probe's.
- **CARGO_INCREMENTAL=0** for the probe and M5, shown in the log: main-run.sh exports it and prints it in its second
  header line as u6b-run.sh does (u6b-run.sh:24, :68; a .checkpoints/ script, the main session's), or each cargo and
  check.mjs command is given as `env CARGO_INCREMENTAL=0 …`, which the "run:" line shows. Why: the builder's counted
  runs used it; incremental output is cache only (no result changes, u6b-run.sh:7-9), and it was 4.3 GB of a 10.4 GB
  dev profile on 2026-11-23 (history.md:7699-7700), the likely bulk of M5's 5,683 MiB.
- **The disk rule:** M5's first run took 6,657 → 974 MiB = 5,683 MiB, with the dependencies already built and
  incremental on. The part-clean since removed the whole dev profile (10.0 GiB), dependencies included, so the next
  build is colder than M5's was. Hence: **at least 8,192 MiB free when the probe starts** (5,683 MiB plus about 2.5 GiB
  for the rebuilt dependencies; now 10,709: met), and **at least 6,144 MiB free when M5 starts** (the probe will have
  built the bites half's tree). Under either: the approved dev part-clean first (`cargo clean --manifest-path
  …/spikes/netcode-proof/Cargo.toml --target-dir …/.gate/shared-target --profile dev`), then at least 8,192 MiB before
  M5 (it builds cold again; the probe's evidence stands: the same tree). main-run.sh's 4 GB floor stays the hard stop.
  After M5, the free space goes on the card.
- **It passes when:** 11 of 11 steps; measured 807 and bites 904 (the hunk adds and removes no test), 0 failed, 0
  skips; the test step under 1.5x base's 954.8 s. A failing test: stop and send it to me; no re-run to green (a second
  failure on a correct tree is evidence, not noise).

### (5) My approval of the changed head: its terms and what I read
- **The head I will approve:** the U6b commit whose spikes/ and docs/ change against its parent equals
  wip/u6b-fix.patch (e29c003c…ac64af1c) in 27 of its 28 files, and in binary.rs the patch's post-image plus exactly
  m6-hunk.diff; no other spikes/ or docs/ path (07d5d31's test lines are already in HEAD, so they are not in the
  commit's change).
- **Before M7 (one read):** m6-hunk.diff and its sha256 (the lines against item 2's list); the 28-file check re-run
  (cmp_files.py: 27 byte-equal to the builder's worktree, binary.rs = binary.rs.pre + the hunk) with each file's
  sha256 recorded (.gate/u6b-merge/m6-files.txt); m6-probe2.log; m5-check2.log; u6b-report.md's changed lines. My
  APPROVE of that tree is the approval M7-M9 run on.
- **M8:** one commit as planned (the 28 files, u6b-report.md, `git rm` wip/u6b-fix.patch; wip/u6b-build.patch kept for
  U6e). Before M9's push the main session checks the commit's 28 blobs (`git show <sha>:<path>` hashed) against
  m6-files.txt and its file list (`git show --stat --format= <sha>`).
- **After M9:** my head read of the pushed SHA (that check, CI's check-run conclusions on it): the review of every
  pushed head, not a round.

### (6) The carry gap: where TR2-U6A-4 went, and where it closes
- **Raised:** u6a-r2-test.md:67-69, :93, :112 (U6a's test seat, round 2; Low; its fix code: "Pass the cap to start()").
- **Lost:** u6a-fix-pass.md:91-97 sent round 2's Lows "to the merge report", this one as text without its id ("the
  double-load bite shares the 5 s cap with its own 2 s of sleeps", :96-97); u6a-report.md:66 put that text under "Not
  verified". Its siblings sent there (TR2-U6A-3, -5) had report text as their fix, so the text met them; TR2-U6A-4's fix
  was code, so the text closed nothing, and no builder was ever handed it. My own U6a checks did not catch it. It is
  recorded as closed nowhere today; the card names it as open (netcode-proof-test.md:86).
- **Closes in U6b:** U6b's report (condition C6) closes it by id, with that history, the M5 firing, the re-rating,
  the hunk, the probe's numbers and M5's second run; U6a's "Not verified" line is superseded there. The card's Open
  findings close it at U6b's commit.
- **The rule this teaches (F-M6-2, Low, report text):** a finding whose fix is code closes only by that code or by a
  carry by id with a taker, never by a "Not verified" line. One line in U6b's report.
- **The draft today:** u6b-report.md does not state TR2-U6A-4 at all (its M5 line is PENDING, :37; the "Closed in U6b"
  row, :125, and "Not verified", :127-133, lack it).

### (7) Mustafa: nothing to decide
- No mark, judge rule, marks.json text, run value, money or stop changes: a test-harness edit in binary.rs, the bite's
  own assert unchanged, the measured starts' 5 s unchanged. U6b ends near 5,960 of his 6,000 stop.
- The 5,950 scope line: I agree it does not govern this fix, for two reasons. It was my ruling's line for starting the
  fix pass's optional Lows (advisor-u6b-r1.md item 7; advisor-u6b-fix.md item 6); this is a merge step's repair of a
  red counted check. And the finding is no longer a Low (re-rated Medium, item 1). His 6,000 stop governs, and holds.
- One line in the day's message as a choice taken without him (NPT-TRIM): M5's first run failed on the double-load
  bite's 5 s cap; the bitten start's cap now covers the start's own 5 s plus the bite's 2 s; no mark or run value moves.

## 3. Findings (CLAUDE.md's scale)
Critical 0, High 0.
- **F-M6-1 (Medium; TR2-U6A-4 re-rated from Low by me, item 1): the bitten start's first-line cap counts the bite's own
  2 s.** binary.rs:77 (5 s for every start) with main.rs:503-512 (2 s of sleeps); fired at M5 (m5-check.log:9,
  failed-steps/test.log:2219-2249). Fix: item 2's hunk. Closed by this edit once C3 and C4 pass.
- **F-M6-2 (Low, process): a code-fix Low closed by report text.** u6a-fix-pass.md:96-97 and u6a-report.md:66 turned
  TR2-U6A-4 into a "Not verified" line with no id and no taker. Fix: C6's lines; the rule in one line of U6b's report.
- **F-M6-3 (Low): u6b_open reads its first line with no cap** (binary.rs:279-282; check.mjs's test step has no timeout,
  check.mjs:1922, :2052-2057), so a room that never opens hangs the step (TR-U6A-7's rule); and its handshakes' margin
  against the room's 2 s deadline in a debug build under the file's parallel starts is unmeasured. Taker: U6e (it edits
  binary.rs for N4): the first line through start()'s capped reader, the handshake time printed.
- **F-M6-4 (Low, evidence): M5 ran with incremental builds on** (main-run.sh exports no CARGO_INCREMENTAL; u6b-run.sh
  exports 0), unlike the builder's counted runs: a disk cost and a difference the evidence cannot separate from the
  slower run. Fix: C2.
- Not checked by me: any run (none made, as asked); the debug binary's size and the hash's share of the start (an
  inference from main.rs:368-379); the machine's load during M5 (no reading exists); the bites build's later binaries
  in M5 (cargo stopped at binary.rs: they ran in the builder's two counted runs on the same 28 files, and run again in
  C4).

## 4. Verdict: APPROVE WITH CONDITIONS (the fix and the path to the merge; the changed head's approval is C5)
1. **C1, the edit:** the main session, with the Edit tool in the main folder, changes only binary.rs's lines in item 2
   (OPEN_CAP; start()'s `cap`; the three callers, the bitten one `OPEN_CAP + Duration::from_secs(2)` with its reason;
   optional doc phrases; rustfmt's reflow), at most 15 added lines, no other file; binary.rs.pre saved before, and
   m6-hunk.diff with its sha256 after.
2. **C2, the runner:** CARGO_INCREMENTAL=0 for the probe and M5, visible in each log (main-run.sh's export and header,
   or `env CARGO_INCREMENTAL=0` on each command).
3. **C3, the probe:** item 3's invocation (fmt, the file x3 in parallel with --nocapture, the bite alone, loadavg before
   and after), at least 8,192 MiB free at its start; pass as item 3 says (all pass; untracked 4a61ef21…; each measured
   start and each bitten start less 2 s at most 2,500 ms). Otherwise stop: no M5; the log to me.
4. **C4, M5 again:** full check.mjs through main-run.sh on the probed tree (same hashes), at least 6,144 MiB free at its
   start (else the dev part-clean, then at least 8,192 MiB); 11/11, measured 807, bites 904, 0 failed, 0 skips, the
   test step under 1.5x 954.8 s. A failing test: stop and send it to me; no re-run to green.
5. **C5, my read before M7:** m6-hunk.diff (+ sha256), the 28-file check with m6-files.txt, m6-probe2.log,
   m5-check2.log and the report's changed lines. My APPROVE then names the head: 27 files = e29c003c's, binary.rs =
   its post-image + m6-hunk.diff, nothing else in spikes/ or docs/. Before M9's push, the commit's 28 blobs against
   m6-files.txt and its file list; after M9, my read of the pushed SHA and its CI.
6. **C6, the report and the card:** U6b's report closes TR2-U6A-4 by id with its history (u6a-r2-test.md:112;
   u6a-fix-pass.md:96-97; u6a-report.md:66), the M5 firing, the re-rating to Medium, the hunk, the probe's numbers and
   both M5 runs (the "M5 in the main folder" line); carries F-M6-3 to U6e by id; states F-M6-2's rule in one line. The
   card closes TR2-U6A-4 at U6b's commit.
7. **C7, the day's message:** one line (NPT-TRIM), as in item 7.

**Decisions needed:** none. A test-margin fix inside the plan: no mark, judge rule, marks.json text, run value, money
or stop of his changes; U6b stays under his 6,000 stop; the 5,950 scope line does not govern it (item 7).
**For Mustafa:** nothing.

DONE (first ruling)

## 5. Second ruling (2026-11-24, after C3's probe missed its 2,500 ms bar)
### Plan
1. Read m6-probe2.log myself; check the coordinator's facts (binary sizes, Cargo.toml:115-120, main.rs:368-370).
2. What the probe changes in my item 1 and item 3.
3. The options (a)-(d): lane, rules check.mjs keeps on profiles, evidence.
4. The ruling, conditions, Mustafa, DONE.

### (1) The probe, read (m6-probe2.log, 05:14:43-05:16:01 UTC)
- Header: free 10,705 MiB; HEAD 8418275; diff 907ffd42… (binary.rs edited); untracked 4a61ef21… (:1-3). loadavg
  0.00 0.07 0.54 before, 2.38 after (:5, :263). fmt PASS (:7-8). Every cargo line has `env CARGO_INCREMENTAL=0` (:9):
  C2 met.
- Three parallel runs, 5 passed 0 failed each (:186, :212, :238); the bite alone 1 passed, finished in 5.26 s (:259).
- s8a: 2,957.8; 3,018.9; 2,984.0 ms (:179, :206, :232). Bitten: 5,049.7; 4,932.8; 4,908.1 ms, alone 4,918.4 ms (:182,
  :208, :234, :256). The start's own work: 2,908-3,050 ms, the same alone and in parallel, and the same in the measured
  and the bitten start. I agree with the coordinator: it is the debug start's own cost, not contention.
- Facts checked: debug npt-server 116,112,832 B, release 7,936,032 B (`ls -l` of both); `binary_sha256` hashes
  `current_exe()` with sha2 in 64 KiB reads (main.rs:368-379, called at :727); the dev profile optimises only rand_chacha
  and ppv-lite86 (Cargo.toml:115-120), so sha2 v0.11.1 (probe2:159) runs at opt-level 0. This CPU reports sha_ni on all
  4 vCPUs (/proc/cpuinfo), so sha2 likely takes its SHA-NI path, unoptimised: how much of the 3 s the hash is stays
  unknown. Not measured here or by the coordinator.
- **New fact: the host changed.** /proc/cpuinfo now reads "Intel(R) Xeon(R) Processor @ 2.10GHz"; fx-starts.log (23:19
  on 2026-11-23, the builder's counted-run window) read "@ 2.30GHz". When it changed is not shown (the 23:22-02:36 cut is
  a likely moment, not proven).

### (2) What this corrects in my first ruling
- **Item 1:** "in a slower run" is withdrawn. On this host the bitten start opens at 4,908-5,050 ms on a quiet machine
  (loadavg 0.00): the old 5 s cap sat on the edge (one of four probe starts over it), so M5 needed no extra load. The
  builder's passing runs were on the 2.30 GHz host. The cause stands (TR2-U6A-4's margin; U6b's heavier debug start),
  and so does the Medium. The hunk works: 4 of 4 bitten starts under its 7 s, with 2 s to spare.
- **Item 3:** the fallback I named (the file's spawning tests in turn) is withdrawn: the alone run equals the parallel
  ones. My 2,500 ms bar stays the measure of the headroom the 5 s cap should have (2x). Probe2 shows 1.6x.

### (3) The options
- **(a) `[profile.dev.package.sha2] opt-level = 3`.** It follows the manifest's own precedent (Cargo.toml:115-116:
  third-party code optimised in test builds; "the code under test keeps its own debug build and checks"). Release is
  untouched, so no run value moves (R1 runs release), and the digest is the same at any opt-level. It adds no
  dependency, feature, version, source or lock line. check.mjs refuses a profile set outside the manifests (env
  CARGO_PROFILE_*, a config file's profile: check.mjs:1443-1446), so the manifest is the sanctioned place. Lane: U6b's
  brief keeps every Cargo.toml out, and "a further change stops the build" and is reported (u6b-brief.md:433-435). This
  is that report, and the ruling is mine: a dev-profile line for one third-party crate is not Block 0's subject
  (dependencies). But it is worth a manifest line only if it actually moves the start.
- **(b) Keep the 5 s cap with the 3.0 s reading.** Honest, but 1.6x headroom on a host that has already changed once:
  the next counted run (U6c's, U6e's, U8's merges each run this file twice) can fail s8a or the bite with no defect,
  and a failing counted run stops for a ruling, as here. Acceptable only if (a) does not help.
- **(c) Measure first, without new code or a file change:** cargo's `--config` sets the same profile line for one
  run: `--config profile.dev.package.sha2.opt-level=3` (a cargo flag, not a config file or an env variable; check.mjs
  does not run then). It measures (a)'s effect directly, which is the remedy's value, not only the hash's share.
  Tracing the hash with strace would give the share, but it needs a standalone start with its own settings and stdin,
  for an answer (c) does not need.
- **(d) Others, rejected:** skipping the hash in debug builds (the tests would no longer run the product's start);
  ring's SHA-256 in main.rs (product code and maybe a direct dependency, for the same effect as (a)); raising the
  measured 5 s cap (a brief term: u6a-brief.md:51, :198).

### (4) Ruling: (c), then (a) if it meets the bar, else (b); inside U6b's merge
1. **R1, the measurement (no file change):** one main-run.sh invocation, log .gate/u6b-merge/m6-probe3.log, on probe2's
   tree (the same diff and untracked hashes), at least 8,192 MiB free at its start (else the dev part-clean first):
   loadavg :: `env CARGO_INCREMENTAL=0 cargo test --locked --workspace --all-features --config
   profile.dev.package.sha2.opt-level=3 --test binary -- --nocapture` three times :: the same with `--exact
   bites::pm_0001_16_double_load_fails_every_start_under_2_s` :: loadavg.
2. **R2, if every s8a start and every bitten start less 2,000 ms in probe3 is at most 2,500 ms, (a) goes in:**
   spikes/netcode-proof/Cargo.toml, after :120 only: a comment line (the room's start hashes its own debug executable,
   116 MB; test builds only, release unchanged) and `[profile.dev.package.sha2]` / `opt-level = 3`. Nothing else in any
   manifest; Cargo.lock unchanged (`git diff --quiet` on it). Then probe4: probe2's exact form on the edited tree (log
   m6-probe4.log, no --config), with my bar kept (at most 2,500 ms each); then M5 (C4 as ruled; check.mjs's deps step
   must PASS with the line). U6b's commit then holds 29 files.
3. **R3, if probe3 misses the bar, (b):** no manifest edit. My bar is replaced by the stated reading: the start's own
   work 2,908-3,050 ms against the 5 s cap (1.6x headroom on this host), recorded in the report. M5 runs on probe2's
   tree with no further probe. A new Medium, F-M6-5 (the debug start is about 4x U6a's 634-790 ms and holds 3.0 s of
   the 5 s cap), is carried by id to U6e (it edits binary.rs): find the start's cost by part, cut it or state the
   headroom.
4. **C5 extends:** under R2, m6-files.txt covers 29 files and Cargo.toml's diff is exactly R2's lines; the rest of C5 is
   unchanged. Under R3, C5 as ruled. Size: at most 3 more added lines, under his 6,000 stop.
5. **C6 adds:** probe2's numbers, the host change, probe3's result and the branch taken; under R2, the manifest line
   with its reason and this ruling's lift of u6b-brief.md:433 for that line only; the bite alone's 5.26 s against the
   brief's "real-clock tests at most 5 s each" (a Low for U6e with that number unless probe4 brings it under 5 s).
6. **Stops:** any failing test, or a probe4 value above 2,500 ms after (a): stop and send me the log.
- **No review seat:** no dependency moves. The evidence is the lock's empty diff plus check.mjs's deps step. No plant:
  the A/B is the evidence.
- **Mustafa:** nothing. No mark, judge rule, marks.json text, run value (release untouched), money or stop. Under R2,
  one more line in the day's message (NPT-TRIM): sha2 optimised in test builds, so the debug start fits its 5 s.
- **Findings added:** F-M6-5 (Medium, only under R3; closed by R2). Item 1's "slower run" corrected (above).
**Decisions needed:** none. **For Mustafa:** nothing.

DONE
