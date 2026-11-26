# NB2-C10: test-engineer's read of 72651a3c's last judge lines

- **What I read:** `git diff 67892d39 72651a3c -- spikes/netcode-proof/crates/judge spikes/netcode-proof/marks`
  (figures.rs's no-tail guard, two engine tests, README and SERIES text). Around it, by `git show 72651a3c:<path>`:
  - figures.rs = `spikes/netcode-proof/crates/judge/src/figures.rs`: lines 561-735;
  - engine.rs = `spikes/netcode-proof/crates/judge/tests/engine.rs`: lines 77-161, 1311-1331 and 1607-1904;
  - README.md = `spikes/netcode-proof/marks/README.md`: lines 70-90;
  - SERIES.md = `spikes/netcode-proof/marks/SERIES.md`: rows 267 and 418, from the diff;
  - scenario.rs = `spikes/netcode-proof/crates/lab/src/scenario.rs`: lines 1385-1594, 1800-1829 and 4405-4435.
- **Method:** I only read. I ran no cargo, no tests and no fetch, and edited nothing but this file. Every "would fail"
  below comes from tracing the code by hand; I did not run it.
- **Code by SHA, notes from the working tree:** I read all code and docs by SHA. Three notes files I read from the
  working tree, so their line numbers can move:
  - .checkpoints/netcode-proof-test/advisor-nb2-merge.md;
  - .checkpoints/netcode-judge.md;
  - .checkpoints/netcode-judge/test-engineer-b.md.
- **Line numbers** are at 72651a3c. They are the same at 48efc9c: `git diff --stat 72651a3c 48efc9c --
  spikes/netcode-proof/crates/judge spikes/netcode-proof/marks` shows only SERIES.md (+51), all of it new probe rows
  at line 908 and after. No judge byte and no marks.json byte changes, and none of the 67 lines changes.

## VERDICT: APPROVE WITH FINDINGS
- **One Medium (M1):** a wrong reason text, with the verdict unchanged. Under NB2-C10 it gets a forward fix, plan
  first. It does not hold the merge: the merge changes no judge byte, and every verdict is unchanged.
- **Four Lows**, and two notes that are not findings (N1, N2):
  - N1 restates two edges already on record and does not raise them again;
  - N2 is my own new observation; it needs no action.

## The four questions
1. **Is the guard right? Yes.**
   - figures.rs:717: `left` becomes the old `left && !at_stop(end)`. That only narrows the rule: these lines can make a
     run INVALID, never VALID.
   - figures.rs:640-642: `stop` is the highest `t` over every room log's tick records, in any phase.
   - `b` is a measured room tick record, so `b <= stop` always holds. Any end at or past `stop` therefore also meets
     `b <= end`, so it is the new guard that decides those cases.
   - `is_none_or` on a run with no room tick record gives no tail. That case cannot be reached: with no room tick
     record, `measured` is empty and step (d) never runs.
   - The guard is the one the test-engineer's re-check proposed (test-engineer-b.md:118-121, "or a judge guard (an end
     at the room's final tick gives no tail)"), and the one the advisor agent ruled for.
   - Its new message arm is wrong in one path: M1.
2. **Would each new test fail without it?**
   - **`c_u6_11_a_session_end_at_the_stop_gives_no_tail` (engine.rs:1814-1836): yes.**
     - The room's last tick record is 12: slot 1 has a hash at every tick, engine.rs:1613.
     - Without the guard, the cases with end 12 and end 13 come out VALID (`b = 12 <= end`), so `!valid` fails.
     - The end-12 case pins `>=`: a `>` plant makes that run VALID, so the test fails. The end-13 case pins "or
       later".
     - The end-11 control (engine.rs:1821-1825) pins the reference point as the room's last tick record. It is not
       `b` (a guard `end >= b` would turn the control INVALID) and not `stop - 1`.
     - The reason text is pinned with `ends_with`.
   - **`c_u6_11_a_hole_inside_a_later_session_is_a_gap` (engine.rs:1790-1812): it does not test the guard.**
     - The lookup it pins was already in place at 67892d39 (test-engineer-b.md:114-117). That is why the diff shows no
       figures.rs change for it: the test adds the missing proof for code that already existed.
     - It passes with or without the guard: its failing case stops at step (c) (figures.rs:660-678), before (d)'s tail.
     - It does bite the plant the commit names: look up the slot's first session for every tick. Then tick 8 falls
       outside the first session's span (1, 4), the second session (5 to 12, span (5, 12)) passes step (d), and the run
       comes out VALID, so the assert fails.
     - It does not bite a lookup that takes the slot's last session (L1).
   - The commit message says the two guard plants and the hole plant each failed their test. I did not re-run them.
3. **Does any README or SERIES line claim more than the code does? No.**
   - README.md:86-88 and SERIES.md:267 say what figures.rs:640-642 and :717 do.
   - "the room log's last tick record": the code takes the highest value over every room log of the run. With one
     room log the two are the same.
   - SERIES.md:418 gives the reason ("it would give a client that went silent a tail it never had") and the judge's
     rule, as the code has it.
   - The writer half of README.md:87-88 and SERIES.md:418 is a requirement on a writer, not a claim about the judge:
     - L1's writer meets it. It writes each end at the tick the session ends (scenario.rs:1417, :1578). At the stop
       it writes `client_counts` and no `session_end` (scenario.rs:1818).
     - A test pins that: f23 (scenario.rs:4412-4427) asserts the lab log holds 0 `session_end` records in a run where
       every session is kept to the stop.
     - No L2 writer of `session_end` exists at 72651a3c or at 48efc9c: `git grep session_end 48efc9c --
       spikes/netcode-proof/crates` finds the lab and the judge only.
   - The code docs match too: figures.rs:606-613 and :710-713, and engine.rs:1814-1815.
   - figures.rs:711 says "a session still connected at the stop left nothing in flight". That holds for measured ticks,
     given README.md:89-90 (the clients keep logging until every measured tick is resolved).
   - The one text that says more than the run shows is a reason string, not README or SERIES: M1.
4. **Can anything here make a verdict wrong? Not by my trace.**
   - The logic change only narrows the tail.
   - M1 changes the reason text, not whether the run is valid.
   - The only new direction is a false INVALID for a real leave on the final tick record (N2). That is the safe side.

## Claim · proof · gap
| Claim | Proof | Gap |
|---|---|---|
| An end at the room log's last tick record, or later, gives no tail | engine.rs:1826-1835 (ends 12 and 13: INVALID, reason pinned) | The phase of the stop record is not pinned: every test tick is Measured (L2) |
| An end before the last tick record is still a leave, with its tail | engine.rs:1821-1825 (end 11, room hash at 11, tick 12 recorded: VALID) | none |
| The reason names the stop only when the end is at the stop | engine.rs:1829-1834 covers the at-stop path | The same text also fires for a real leave that fails on a late first tick; no test covers that path (M1) |
| The (c) step looks up the session that holds the tick | engine.rs:1793-1812 bites "the slot's first" | No test has a hole in an earlier session while a later one exists (L1) |
| L1's writer writes no `session_end` at the stop | scenario.rs:1818, f23 at :4412-4427 | No L2 writer is built yet: for L2 the text is a requirement on that writer |
| The merge changes no judge or marks.json byte | `git diff --stat 72651a3c 48efc9c` (SERIES.md only, +51 rows at 908+) | none |

## Findings, ranked
- **M1 (Medium): figures.rs:722-724. The new reason arm says "the stop" for a real leave.**
  - The arm `Some(end) if b <= *end` does not test `at_stop`.
  - The error block (:718) is also entered when `left` is true and `c > a`, that is, a session whose first resolved
    tick is late but which left properly. `left` already requires `b <= end`, so this arm then always fires.
  - Trace, in engine.rs's terms: `session_run(&[5,6,7,8,9,10], &[7,8,9], &[("admit",5,"room"),
    ("session_end",10,"room")])`. The room's last tick record is 12, a = 5, b = 10, c = 7, e = 9, and `left` is true.
    The run is INVALID because c > a, and its reason ends "(its session_end at tick 10 is at the room's last tick or
    later: the stop, not a leave)".
  - That is false: the room's last tick record is 12. Before the diff the same path printed "(its session_end is at
    tick 10)".
  - The verdict is unchanged: INVALID either way, and rightly so. But a session reading this reason is sent to the
    writer's stop handling when the real cause is the late first tick.
  - Small reach: a late start together with a real leave. There is a workaround: the same line's tick numbers show
    c > a.
  - Fix: make the arm `Some(end) if at_stop(end)`. Because `b <= stop`, this implies `b <= end`. Optionally, also add
    `|| left` to the first arm, so a failure on the start alone carries no tail note.
  - Test: the trace above, asserting INVALID and no "the stop" in `why`.
- **L1 (Low): engine.rs:1790-1812. Half of the lookup is unpinned.**
  - The hole test rules out "the slot's first session". No c_u6_11 test has a hole inside an earlier session while a
    later session exists.
  - So a lookup at figures.rs:663-666 that takes the slot's last session passes every c_u6_11 test, by my trace.
    Under it, the `rejoined` run with the hole at tick 3 instead of 8 reads VALID. With today's code it gives "slot 0,
    tick 3: the room logged a hash and no client log resolves it".
  - The code is right. Only the test is missing.
- **L2 (Low): engine.rs:1322 (through `hash_log`) and figures.rs:640-641. The phase of the stop is unpinned.**
  - Every tick that `session_run` builds is Measured. The code takes the last tick record in any phase, as README.md:88
    and SERIES.md:267 say.
  - A change to measured-only (stricter: a real leave in the After phase would lose its tail) would pass every test.
  - Fix: one case with After-phase room records after the end.
- **L3 (Low): engine.rs:1820. Typo.** "ticks 11 was in flight" should read "tick 11 was in flight".
- **L4 (Low): README.md:88. Not re-wrapped.** It is one line of about 180 characters in a file wrapped near 120.

## Notes, not findings
- **N1 (not new): two edges stay open, as recorded at .checkpoints/netcode-judge.md:239-241.**
  - A writer that ends a still-connected session before the room's last tick record. engine.rs:1821-1825 now pins
    stop − 1 as an allowed tail, which is right for a real leave and is exactly that recorded edge for a writer that
    ends sessions at teardown.
  - The tail has no length limit.
  - No README or SERIES line claims the guard closes either one. Their home is U6d's writer brief and the readings
    change before R5.
- **N2 (my own observation, new): the one new direction is a false INVALID, never a false pass.**
  - A real leave whose `session_end` lands on the room's last tick record now gets no tail. L1 stamps ends at the tick
    just run (scenario.rs:1417, :1578), so a scripted leave on the final tick, with hashes still in flight, reads
    INVALID.
  - README.md:88 states the rule. No action is needed.

## Not verified
- Every "fails without it" and "passes under a plant" above is a trace by hand. I ran no cargo and no plant. The plants
  the 72651a3c commit message reports are its own record.
- Whether any recorded run would change verdict under the guard. I re-judged nothing and did not look for recorded
  sets. (For the -47 set alone, netcode-judge.md:252-255 says its raw logs are not in the repository.)
