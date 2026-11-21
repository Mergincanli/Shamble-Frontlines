# Advisor: TE2-5 (c), TE-M3's cause and fix (te-m3-plan.md rev 1): plan review

Reviewed 2026-11-13 at effort xhigh, from main's `.claude/agents/advisor.md`. Branch head 2f01711; spikes/ equals cc8595d
there (`git diff --quiet cc8595d HEAD -- spikes/netcode-proof` exits 0). Notes: advisor-te-m3-notes.md.

## Verdict: APPROVE, with conditions 1 to 7 below

The cause is right, and it is proven, not guessed. The change is the smallest one that fixes it, and it hides no
defect. The proof plan is sound once P2 is required to show the case the fix covers (condition 1). The other
conditions are wording and record-keeping. No second plan review is needed: my check of the commit checks the
conditions.

Risk tier 1 (a test file in a spike). Nothing for a player to see, hear or feel, so no Brave check. Nothing for
Mustafa.

## What I checked myself
- **The logs.** loop.log: 60 of 60 whole-binary runs exit 0, each "169 passed; 0 failed; 12 ignored". That loop ran
  the runner's earlier form (its arguments "30 2"); da6e42a extended the runner afterwards. one-h6.log: 8 runs passed,
  then loop 1 run 4 exited 101. one-h6-1-4.log panics at delay_tests.rs:2605:5 with "3 shutdown calls". The test's own
  line reads 2 runs, 1 at or after the deadline, 3 calls, 3691096 ns late. The 8 passing runs each read (2, 1, 2),
  0.17 to 2.27 ms late. Both builds made the same binary, npt_net-301de27e05915ce7. The plan's numbers match the logs.
- **My own reproduction of P1** (through main-run.sh with the lane lock held; `te_m3_loop.sh adv-h6 one 150 2 6`;
  .gate/te-m3/adv-h6.log). 89 runs, then loop 2 run 44 exited 101 (adv-h6-2-44.log): the same panic at :2605:5, "3
  shutdown calls", with the triple (2, 1, 3). This run ended only 191416 ns after the deadline, so lateness does not
  trigger it. The 89 runs' own lines: 87 × (2, 1, 2), 1 × (2, 1, 3) (the failure), and 1 × (3, 1, 3), a pass in which
  the doc comment's third run happened. The adv-h6-* logs are mine. The main session's runs use other tags.
- **The code at the head** (every line the plan cites checks out):
  - The pump's run is one pass through blocks, each taking the lock and releasing it. The opening block counts the
    run (delay.rs:1703-1710). The shutdown block reads `c.shutdown` again, under a new lock (:1848-1856). In between,
    S's read and write run with the lock free.
  - S's poll_shutdown has one call site, delay.rs:1858, and the shutdown block runs at most once per run (grep; the
    pass has no loop around it). The pump owns S (:1640-1642).
  - DelayStream::poll_shutdown sets `c.shutdown`, releases the lock, then wakes the pump (:1611-1636).
  - The shutter is woken only once down_done or pump_done is set (wakers(), :844-853), so `ds.shutdown()` cannot
    return before the pump's last run.
  - The test body runs on block_on's thread (frames 4-13 of the backtrace); the pump runs on a worker (`handle.spawn`,
    :944).
- **Tokio polls a task on one thread at a time.** tokio-1.54.1's src/runtime/task/state.rs:117-145: a task becomes
  running only from idle. A wake that arrives during a poll makes exactly one more poll after it (:151-178,
  OkNotified).

## The cause: confirmed
Every S poll_shutdown call comes from a pump run's shutdown block, at most once per run, and only once the shutdown has
begun. `polls` counts the runs that started after `before`. No run starts after `life` is read: the pump has returned
Ready by then. So `s_calls > polls` means at least one run that called S started before `before`. That run was under
way when the shutdown began, and its shutdown block saw `c.shutdown` already set. Because tokio polls a task on one
thread at a time, at most one run can be under way. So `s_calls <= polls + 1` always holds for correct code, and the
triple (2, 1, 3) means exactly this case. That reasoning comes from the code. Both failures with a panic (theirs and
mine) show that triple and nothing else, and every other assertion held in both.

The extra call is correct behaviour, not a defect of DelayStream. The shutdown had begun and every byte had been taken,
so polling S's shutdown at once is what C3 asks for. The pump did not spin, the deadline bounded the wait, the timeout
was counted, S was dropped and the client read every byte.

The plan says the run under way was woken by `until`'s last check. That is one possible waker; the other is just as
likely. The clock thread fires the pump at the model's last event (the final acknowledgement), and that run is the one
that makes the model idle. The bound needs neither (finding AM-3).

## The change: the smallest, and it hides nothing
- It changes only the test. The bound it sets is the exact invariant above, not a looser guess.
- The other way to fix the counting windows is to count each run at its end in delay.rs's cfg(test) code. That would
  be a bigger change to U3's code, with several exit points to cover. I do not ask for it.
- What the +1 gives up is small. A pump that called S twice in only one run would pass if the run under way did not
  happen. Twice in every run still fails: from 2 runs, 4 calls > 3 (P4 (b)).
- `polls <= 3` and `polls_after_deadline == 1` stay, rightly.

## Findings (CLAUDE.md's one scale) and their exact fixes
- **AM-1, Medium: P2 can pass without ever running the case the fix covers.** "Every run exit 0" over 600 runs proves
  nothing about (2, 1, 3) if that case never happens. Fix: P2's pass criterion becomes all of these:
  - every run exits 0;
  - the tally counts exactly one own line per run (600);
  - at least one run with s_calls = polls + 1 (for example (2, 1, 3)) passes.
  If none shows in 600, the same loops go on, under the same load, up to 2,000 runs per loop. If there is still none,
  that comes to me before the commit. The record gives the whole tally (every triple with its count) and the command
  that made it.
- **AM-2, Low: the doc comment would contradict the new assertion.** Section 3.2 adds a sentence after "... and S's
  poll_shutdown no more often", but that clause makes the same claim as the old assertion: no more S calls than runs.
  Section 2 calls the doc comment "right about the runs", which is true, but its S-calls clause is just as wrong as the
  old assertion. Fix: replace the clause. Do not add to it. The doc's text from "so the pump runs at most 3 times"
  onward becomes, wrapped to the file's width:
  "so the pump runs at most 3 times (2 expected; one more allowed for a wake already queued when the wait began). S's
  poll_shutdown is called at most once per run and only once the shutdown began, so at most once per counted run and
  once more: a run already under way when the wait began (counted before `before`; tokio polls a task on one thread
  at a time, so there is at most one such run) can reach its shutdown block after the shutdown began. The old code
  spun here, re-arming the past deadline at once: thousands of runs and no end."
- **AM-3, Low: the comment and the record name one waker as the cause.** Section 3.2's "(woken by the wait's own last
  check, ...)" goes, as in AM-2's text. Fix:
  - the plan's "Done" record states the mechanism without picking a waker;
  - it names both possible wakers: the clock's fire for the model's last event, and `until`'s step;
  - it says that the bound needs neither.
- **AM-4, Low: P4 names lines that will move.** The doc comment's added lines and the assertion's multi-line layout
  shift ":2604" and ":2605". Fix: P4 names each expected failure by its message: "no spin while waiting" for (a), and
  "shutdown calls for" for (b). Each plant's record shows:
  - the exit code (101);
  - the panic's line at the edited head;
  - the test's own line: polls in the thousands for (a), s_calls at least twice polls for (b).
- **AM-5, Low: P3 has no stop rule for another test's failure.** The plan states one for P2 only. Under P3's busy
  loops, a different npt-net test may fail. Fix: the same rule as P2. The runner stops, the run is kept whole, and the
  test's name and panic are recorded under TE2-5 and come to me before the commit. Never a retry.
- **AM-6, Low: the plan names no risk tier.** Fix: one line in the plan. Risk tier 1 (a spike's test file); no Brave
  check; reviewers per TE-Q4.
- **AM-7, Low: tie the runs to the bytes committed.** Fix: at the start of P2, P3 and P5, record the sha256 of
  crates/net/src/l2/delay_tests.rs and delay.rs; each must equal the committed blob. Record each build log's "Compiling
  npt-net (<main folder path>)" line as well. After the P4 plants, delay.rs's sha256 equals its start value (the plan
  already says so).
- **Observation (no finding, no change asked).** `polls <= 3` rests on the doc comment's "one queued wake". I cannot
  rule out an interleaving with two separate wakes queued before the wait (a pending notification, plus a clock fire
  already in flight), each landing in a different run. It would be rarer than TE-M3 by orders of magnitude. P2 and
  every later run watch that line. If it ever fails with polls = 4, it comes to me: never a retry.

## Rulings on TE-Q1 to TE-Q4
- **TE-Q1, the bound: `s_calls <= polls + 1`, as recommended.** It is the exact invariant: one call per run at most,
  one run under way at most. `s_calls <= 3` is wrong two ways:
  - It unties S's calls from the runs, so 3 calls in 1 run would pass.
  - It fails on correct code. Take a run under way that has already been woken again, and suppose the shutdown's own
    wake is delayed past the next run's start (the test's thread preempted between releasing the lock and the wake
    at delay.rs:1631-1633). The runs are then: the one under way, the woken one, one more for the late wake, and the
    deadline's. That gives (3, 1, 4), which passes `polls <= 3` and `polls + 1` but fails `<= 3`.
- **TE-Q2, when TE2-5's rule ends: at U4r's merge, not at this commit.** It ends then only if no failure of check.mjs's
  test step (any test) happens in P5 or in U4r's counted and judged runs. If one does, the rule continues and that
  failure comes to me. Why:
  - The commit proves the cause for the one failure whose panic we have.
  - U3c's failure and U4d2's counted-run failure are not shown to be the same assertion (TE-Q3).
  - My U4d2 ruling placed this change so that U4r's counted runs would "not meet it unnamed". Those runs are the first
    counted runs after P5 on the fixed test.
  - Keeping the rule costs nothing unless a test fails.
  After it ends, FX-10's saved output of a failed step stays. A failing test is still a defect until its cause is
  known (B2.3: "a later green proves nothing about it"). The card's "Rules in force" says so.
- **TE-Q3, U3c's record: accepted as not naming the assertion. No extra runs first.** The arguments:
  - b2e879b's test is byte-identical to the head's (b2e879b's :1940-2003 diffed with the head's :2544-2607: no
    difference).
  - Its :2002 is the last statement, as :2606 is now.
  - The record names 2002 and 1948, the lines of the two frames that carry the test's name (frames 14 and 15 today).
    That supports the plan's reading but does not prove it.
  - The raw output is gone (none under .gate/ or .checkpoints/).
  The record says so in these words, in the commit's notes and in U4r's brief's TE-M3 line: "U3c's (u3c-fix2-c2b.md:74)
  and U4d2's (u4d2-review-test.md:182-186) failures of this test: their assertion is not shown (no panic kept). If
  U3c's :2002 was the panic site, it was 'every byte, then EOF', a different failure. P2, P3 and every later counted
  run watch every assertion."
- **TE-Q4, review: this plan review and my check of the commit. No test-engineer seat now.** Why:
  - The diff is one assertion and one comment in a test.
  - I checked the cause against the code and reproduced the failure myself.
  - P4's plants are the test seat's own method, judged by exit codes and messages.
  - The spike's pull request still gets its final review with the test-engineer seat (CLAUDE.md step 5), and this
    diff is part of it.
  My check of the commit is not a review round. In it I will:
  - read the diff;
  - check conditions 1 to 7;
  - read the P2 to P6 logs;
  - run the fixed test myself under the same load (`te_m3_loop.sh <tag> one 150 2 6` through main-run.sh), expecting
    every run exit 0.

## Conditions (to meet before or in the commit)
1. AM-1: P2's pass criterion as stated.
2. AM-2 and AM-3: the doc comment's text as given in AM-2. The assertion as plan §3.1 gives it (rustfmt's layout,
   applied by Edit; check.mjs's fmt step passes).
3. AM-4: P4 by message, with the new lines recorded.
4. AM-5: P3's stop rule.
5. AM-6: the risk tier line in the plan.
6. AM-7: sha256 and the "Compiling npt-net" line for P2, P3 and P5.
7. The record lists P1 twice: one-h6-1-4.log, and my adv-h6-2-44.log.

## What the main session does next, in order
1. Edit delay_tests.rs: the assertion (plan §3.1) and the doc comment (AM-2's text). Edit nothing else; delay.rs stays
   unchanged.
2. P4 (a) and (b): each plant made by Edit, run (test alone, mode `one`, a few runs), then restored by Edit, with
   delay.rs's sha256 equal to its start value (AM-4).
3. P2 (AM-1, AM-7), then P3 (AM-5, AM-7), each through main-run.sh. Any failure stops the work and comes to me.
4. P5: check.mjs counted. All 11 steps PASS, 491 / 561 / 71 of 71, marks 81, redact 0 (AM-7).
5. P6: `node tools/gate.mjs --no-smoke` PASS.
6. One commit ("TE2-5 (c): TE-M3's cause ..."): delay_tests.rs, plus the plan's "Done" record (AM-3, TE-Q3's words)
   and the logs' summary. Then push.
7. CI green on that commit. Then the commit comes to me for the check described under TE-Q4.
8. The card and U4r's brief:
   - the card's TE-M3 entry points to the commit;
   - its "Rules in force" says TE2-5's rule runs until U4r's merge (TE-Q2);
   - U4r's brief's TE-M3 line carries TE-Q3's words and the rule's end.
   The brief is in the other agent's run, so its file is not touched while that agent works.

## What I did not check, and why
- I ran no P2 to P6 myself: they run on the edited test, which does not exist yet.
- I did not see U3c's or U4d2's panic text: neither was kept.
- I did not rule out polls = 4 on correct code (the Observation). P2 watches for it.

## Decisions needed
None. Nothing here is Mustafa's.

## For Mustafa
Nothing.
