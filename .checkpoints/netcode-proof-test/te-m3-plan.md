# TE2-5 (c): TE-M3's cause and its fix (the main session's planned change; plan rev 1, 2026-11-13)

Owner: the main session (npt-net is U3's code; no U4 lane holds crates/net). Route: advisor-u4d2-r1.md Block 5 (TE-M3:
"a planned change of the main session's ..., plan first with me, after U4d2's merge and before U4r's merge ... Its
first step is the panic ... Never a retry, and never a looser bound without its cause"). U4d2 merged at cc8595d; U4r
has not merged. Every spikes/ file:line below is at cc8595d (spikes/ is unchanged since: `git diff --quiet cc8595d
HEAD -- spikes/netcode-proof` exits 0).
Risk tier 1 (a spike's test file); no Brave check; reviewers per TE-Q4 (advisor-te-m3.md AM-6).

## 0. In plain words
npt-net's test `l2::delay::tests::a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning` checks that a
connection whose far end never finishes closing is given up at its deadline without busy-looping. Under heavy CPU load
it failed within seconds: S's close was asked 3 times while the test counted 2 pump runs. The pump did nothing wrong
(2 runs, ended at the deadline, every byte delivered). The test compares two counts taken over different time
windows: a pump run already under way when the shutdown began calls S once more but is not in the run count. The fix
is one assertion in the test (and its comment), with the cause stated; delay.rs does not change.

## 1. The evidence (logs under .gate/te-m3/; the runner te_m3_loop.sh, run through main-run.sh, lane lock held)
- **Loop 1** (11:47-12:05 UTC; loop.log): npt-net's library tests in the bites build (`--all-features`, as
  check.mjs's bites test step; binary npt_net-301de27e05915ce7, build.log), the whole binary 60 times (two loops of 30
  at once): every run exit 0, "169 passed; 0 failed; 12 ignored". No failure.
- **Loop 2** (12:06:50-12:06:52 UTC; one-h6.log): the test alone (`--exact`, `--nocapture`, one test thread), two
  loops, six busy loops on the machine's four CPUs. Eight runs passed, then loop 1's run 4 exited 101
  (one-h6-1-4.log): "panicked at crates/net/src/l2/delay_tests.rs:2605:5: 3 shutdown calls"; that run's own line:
  "the pump ran 2 times during the wait (1 at or after the deadline), S's poll_shutdown 3 times; ended 3691096 ns
  after the deadline". The eight passing runs: 2 runs, 1 at or after the deadline, 2 calls; ended 0.17 to 2.27 ms
  after the deadline. The VM's steal time over the failing run: 6 jiffies (/proc/stat).
- **The failing assertion:** delay_tests.rs:2605, `assert!(s_calls <= polls, "{s_calls} shutdown calls");`. In that
  run every assertion before it held: the timeout counted (:2595), the pump ended (:2596), S had the whole wait
  (:2599), the first run at or after the deadline ended it (:2600-2603), no spin (:2604, 2 runs).
- **U3c's failure** (u3c-fix2-c2b.md:74: "panicked at delay_tests.rs:2002 (b2e879b's own code ...)"): b2e879b's line
  2002 is the test's last statement (the reader's count, today's :2606). Today's backtrace gives that same statement
  for tokio's block_on frame (frame 14, "delay_tests.rs:2606:72") while the panic is at :2605:5. So that record does
  not show which assertion failed then. The proof runs (P2, P3) watch every assertion of the test.

## 2. The cause
- **What each count covers.** `polls = life.polls - before.polls` (delay_tests.rs:2580, :2583) counts the pump's runs
  that started after `before = ds.life()`: a run is counted at its start, under the lock (delay.rs:1703-1710).
  `s_calls` (:2584) counts every call of S's poll_shutdown since S was made (NeverShut, :2538-2541).
- **Who calls S's poll_shutdown.** Only the pump, which owns S (delay.rs:1640-1642), at most once per run, in its
  shutdown block, and only once a shutdown began (delay.rs:1848-1858; `c.shutdown` is set by DelayStream's
  poll_shutdown, :1621-1622).
- **So** a pump run that started before `before` was read and reached its shutdown block after the shutdown began
  calls S once without being counted in `polls`. Such a run exists when the pump runs as the test's wait ends:
  `until`'s own check (delay_tests.rs:94-105, every 5 ms) calls model_idle and snapshot, which step the model on the
  test's thread and may wake the pump (DelayStream::current, delay.rs:1163-1175). The run it wakes can still be under
  way when `until` returns, `before` is read and the shutdown begins: the test runs on a two-worker runtime
  (delay_tests.rs:2551), the pump on a worker, the test body on the runtime's own thread. Under load that run is
  preempted between its start and its shutdown block, which widens the overlap. Then (polls, s_calls) = (2, 3): the
  shutdown's own wake and the deadline's, plus the run under way.
- **Not a defect of DelayStream.** The pump did not spin (2 runs in the wait), the deadline bounded the wait, the
  timeout was counted, S was dropped and the client read every byte. The test's doc comment (delay_tests.rs:2547-2549:
  "the pump runs at most 3 times ... and S's poll_shutdown no more often") is right about the runs; the assertion
  compares two counts taken over different windows.

## 3. The change (one file, crates/net/src/l2/delay_tests.rs, the test only; delay.rs unchanged)
1. :2605 `assert!(s_calls <= polls, "{s_calls} shutdown calls");` becomes
   `assert!(s_calls <= polls + 1, "{s_calls} shutdown calls for {polls} pump runs: at most one per run, and one more
   for a run already under way when the wait began");` (rustfmt's layout, applied by Edit).
2. The doc comment (:2544-2550) gains one sentence after "... and S's poll_shutdown no more often": "S's poll_shutdown
   is called at most once per run, and only once the shutdown began, so a run already under way when it began (woken
   by the wait's own last check, and counted before `before`) can call it once more: at most the runs plus one."
3. Nothing else changes: `polls <= 3` (:2604) and `polls_after_deadline == 1` (:2600-2603) stay; no test is added or
   removed, so the counts stay 491 / 561 / 71 of 71.
- **Why `polls + 1`:** it ties S's calls to the pump's runs (a pump that called S twice per run fails it from 2 runs
  on: P4 (b)), and it is the tightest bound that holds with a run under way. With `polls <= 3` it allows at most 4;
  every run seen gave at most 3.
- **Rejected:** reading `calls` when `before` is read (both counts over one window) does not help: the run under way
  calls S after `before` is read. Waiting a fixed time before `before` would make the test timed (its comment:
  "Counted, not timed").

## 4. Proof (each through main-run.sh; logs under .gate/te-m3/; pass or fail from the exit code and libtest's own
summary line)
- **P1, before the change:** loop 2's failure (done: one-h6-1-4.log).
- **P2, after it, the same load:** te_m3_loop.sh, mode `one`, 300 runs in each of 2 loops, 6 busy loops: every run
  exit 0; the (runs, at or after the deadline, S calls) triples tallied from each run's own line, showing how often
  (2, 1, 3) occurs. Any other failure stops the loop, is kept whole, and comes to the advisor: never a retry.
- **P3, the whole binary under load:** mode `all`, 10 runs in each of 2 loops, 4 busy loops: every run exit 0.
- **P4, the test still bites** (B6: each plant by Edit in place, restored by Edit, its sha256 checked against the
  start; the test alone, mode `one`, a few runs): (a) the old spin: in delay.rs's shutdown block, a pump that wakes
  itself at once while the deadline is ahead (`cx.waker().wake_by_ref()` in the branch that neither completes nor
  times out): the test must fail by name at :2604 ("no spin while waiting"); (b) two S calls per run: S's
  poll_shutdown called twice in the shutdown block: the test must fail by name at :2605.
- **P5:** check.mjs counted (main-run.sh): all 11 steps PASS, 491 / 561 / 71 of 71, marks 81, redact 0.
- **P6:** the gate (`--no-smoke`) PASS; CI green on the commit.

## 5. After it
- One commit on the spike branch ("TE2-5 (c): TE-M3's cause ..."), after the advisor's APPROVE of this plan; the
  advisor's next ruling on the branch checks it. The record: this file's "Done" section and te_m3_loop.sh's logs.
- U4r's brief names the change in its TE-M3 line.

## 6. Questions for the advisor
- **TE-Q1, the bound:** `s_calls <= polls + 1` (recommended, section 3), or `s_calls <= 3`?
- **TE-Q2, TE2-5's rule** (advisor-u4j-r2.md B2.3; advisor-u4d2-r1.md Block 5: "The rule stays in force until TE-M3's
  cause is found and fixed"): does it end with this commit (recommended: yes, once P2-P6 hold), or at U4r's merge?
- **TE-Q3, U3c's record:** accept that it does not name the assertion that failed then, with P2 and P3 watching every
  assertion (recommended), or more runs first?
- **TE-Q4, review:** this plan's review and the advisor's check of the commit (recommended), or also a test-engineer
  seat on the diff?

## 7. Done (the record; the advisor's conditions 1 to 7, advisor-te-m3.md)
- **The advisor's review:** APPROVE with conditions 1 to 7 (advisor-te-m3.md; workflow wf_5a565912-f9b, done about
  12:24 UTC). TE-Q1: `s_calls <= polls + 1`. TE-Q2: TE2-5's rule ends at U4r's merge, and only if no check.mjs
  test-step failure happens in P5 or in U4r's counted and judged runs. TE-Q3: accepted, with its words below. TE-Q4:
  no test-engineer seat now; the advisor checks the commit (the diff, conditions 1-7, the P2-P6 logs, and its own run
  of the fixed test under the same load).
- **P1, twice (condition 7):** .gate/te-m3/one-h6-1-4.log (the main session's loop 2, 12:06:52 UTC, the 9th run) and
  .gate/te-m3/adv-h6-2-44.log (the advisor's own reproduction, 89 runs: the same panic at delay_tests.rs:2605:5 and the
  same triple (2, 1, 3), ending 0.19 ms after the deadline).
- **The mechanism (AM-3):** a pump run already under way when the shutdown began started before `before` was read, so
  `polls` does not count it; it reaches its shutdown block after the shutdown began and calls S's poll_shutdown once.
  Two wakers can start such a run as the wait ends: the clock's fire for the model's last event, and `until`'s step
  (DelayStream::current, delay.rs:1163-1175). The bound needs neither: tokio polls a task on one thread at a time, so
  at most one run can be under way.
- **The edit (condition 2):** delay_tests.rs only. The doc comment, from "so the pump runs at most 3 times" onward,
  replaced by AM-2's text, wrapped to the file's width; the assertion as section 3.1 gives it, in rustfmt's layout
  (`rustfmt --check` exit 0). delay_tests.rs sha256 17c63a39… before, 74d49de7… after; delay.rs unchanged
  (aaa6a588…).
- **P4, by message (condition 3):** each plant made by Edit in delay.rs's shutdown block, run through main-run.sh
  (te_m3_loop.sh, mode `one`, the first run failing as wanted), restored by Edit, delay.rs's sha256 then aaa6a588…
  (its start) and `git diff` clean.
  - (a) the old spin, `} else { cx.waker().wake_by_ref(); }` after the timeout branch: exit 101, "panicked at
    crates/net/src/l2/delay_tests.rs:2607:5: no spin while waiting: 22632 runs"; its own line: 22,632 runs, 22,632 S
    calls (p4a.log, p4a-1-1.log).
  - (b) S's poll_shutdown called twice in the shutdown block: exit 101, "panicked at
    crates/net/src/l2/delay_tests.rs:2608:5: 4 shutdown calls for 2 pump runs: at most one per run, and one more for a
    run already under way when the wait began"; its own line: 2 runs, 4 S calls (p4b.log, p4b-1-1.log).
- **TE-Q3's words (the advisor's, for this record, the commit's notes and U4r's brief's TE-M3 line):** "U3c's
  (u3c-fix2-c2b.md:74) and U4d2's (u4d2-review-test.md:182-186) failures of this test: their assertion is not shown (no
  panic kept). If U3c's :2002 was the panic site, it was 'every byte, then EOF', a different failure. P2, P3 and every
  later counted run watch every assertion."
- **P2 (conditions 1 and 6): PASS** (12:26:43-12:28:57 UTC; .gate/te-m3/p2.log; `te_m3_loop.sh p2 one 300 2 6` through
  main-run.sh, the lane lock held exclusively; the tally by `python3 -I .checkpoints/netcode-proof-test/te_m3_tally.py
  p2`). At its start: delay_tests.rs sha256 74d49de7…, delay.rs aaa6a588… (the committed blob), "Compiling npt-net
  v0.0.0 (/home/user/Shamble-Frontlines/spikes/netcode-proof/crates/net)". 600 run logs, 600 own lines (exactly one per
  run), 600 exits 0. The tally (runs, at or after the deadline, S calls): (2, 1, 2) 591; (2, 1, 3) 1; (3, 1, 2) 3;
  (3, 1, 3) 5. One passing run with S calls = runs + 1: the case the fix covers, met (AM-1).
- **P3 and P5:** running (p3.log, then p5-check.log, in one chain: P5 only if P3 exits 0).
