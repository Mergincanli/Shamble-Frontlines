# U6a round 2, netcode seat (NPT-TRIM: what the fix pass changed only), 2026-11-23

Inputs: u6a-fix-pass.md (FP-1 to FP-12); u6a-build.md's fix-pass lines (:212-346); wip/u6a-build.patch (sha256
022ddb84d01ec3b913917919d2052bc91c8f54ac1c2c1a8754bbf018bbef98e1 checked; 5,843 added); .gate/u6a/fp*.log, check4.log,
fp-restored.log. Tree: W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff/spikes/netcode-proof
(read only; every file:line below is under W). No Edit tool here: this file is rewritten whole with Write after each
block. No cargo run made.

## Plan
1. NR-U6A-1: super/src/main.rs flags, Flags, log() header; the new npt-super test; binary.rs's double-load bite;
   against judge eval.rs:47-61, :542-564 (and :576-584 footer); is the U8 carry all U8 needs.
2. NR-U6A-2: TickLoop::io_on_a_tick_sibling and main.rs sibling_refusal(), its place in run(), its test.
3. NR-U6A-3 and -4: proc_read alone(), sibling_free_read(), cpu_ns Option; tick_loop affinity(), Seconds values/write;
   the "no jiffy" gap side effect.
4. FP-5: the two counted offers.
5. FP-6: counted()'s lab rule; the 90-tick outcome.
6. Anything the fixes broke in round 1's scope (FP-7 to FP-12 only as they touch my scope).
7. Verdict, NR2 table, not verified, DONE.

## Notes (one line per item, as checked)
- (1) NR-U6A-1 CLOSED. super/src/main.rs: `--scene`, `--tick-hz` (1 to 1,000) and optional `--bite` parsed (:97-107),
  `--scene` and `--tick-hz` required (:136-137); `log()` writes `bite: f.bite`, `tick_hz: f.tick_hz`, `scene: f.scene`
  (:201-208). Each of the judge's eight same-run fields (eval.rs:49-58: set, role, bite, layer, profile, tick_hz, scene,
  date) is now a flag value (set h[1], role h[2], date h[3], layer h[5], profile h[6], plus the three), so a super log
  given the room's values sits in the room's run with no `inconsistent` entry (eval.rs:542-547). In a bite run the
  room's log supplies the bites build (`bites_build` is any log's, eval.rs:548-564), so npt-super's `features: []` is
  fine. eval.rs:734 reads `run.logs[0].header.tick_hz`, which now holds whichever log comes first. figures.rs:584-585's
  `clients` agreement filters room logs only, so super's `clients: 0` is fine. The new test
  `h_the_log_header_carries_the_runs_bite_scene_and_tick_rate` (:561-593) reads npt-super's REAL header form back
  (Some(bite), "SS", 30) and shows max spawn_to_open_ns >= 2 s on that log. binary.rs:223-240 now asserts only the room's
  own facts (OPEN >= 2 s, bite, features ["bites"]). The two halves hold on real forms. P12 fails the npt-super test
  (u6a-build.md:322-324).
- (1, U8 carry) Not quite all. A super log in its own run id cannot hold a bite run: it names the bite with no
  bites-build log, which eval.rs:560-564 makes INVALID. And a value that differs between npt-super's flags and the
  room's settings makes the run INVALID. So U8's carry needs one more clause: npt-super's log takes the room's run id
  AND the same set, role, layer, profile, date, scene, tick_hz and bite as the room's settings, from one source in the
  run script, for every run, bite runs included. Plus, as listed, it ends by its stdin closing, never by a kill
  (a log with no footer is INVALID: eval.rs:579-584). -> NR2-U6A-1 (Low, carry U8).
- (2) NR-U6A-2 CLOSED. `TickLoop::io_on_a_tick_sibling` (tick_loop.rs:1447-1449) is `!sibling_free(smt, tick,
  &[io_vcpus])`, the same rule `event.affinity.sibling_free` reports. main.rs:431-439 `sibling_refusal` returns None with
  no tick vCPU; run() (:442-446) reads this machine's map first and exits 2 with one stderr line BEFORE mint and
  write_tokens, so no token file or log is left, as for other bad settings. It uses `machine.smt`, the real map, before
  header() clears it under the no-smt-map bite (main.rs:464-465). The test (main.rs:796-806) covers a fixed map
  refused, a clean set and no pinning; P13 fails it. That is Q-U6A-5's term (advisor-u6a-brief.md:90-91). An empty map
  (failed /sys read) refuses nothing here, but then the start's affinity writes `sibling_free` 0 with a stderr line
  (FP-3) and `machine_named` reads 0, so it is not hidden; fine.
  New pub item: `TickLoop::io_on_a_tick_sibling` is beyond the brief's pub list, a pure function placed so the
  binary can reach a crate-private rule; acceptable, and U6a's report names it -> NR2-U6A-4 (Low, report line).
- (3) NR-U6A-3 CLOSED. proc_read.rs:166-175 `alone()` is Err unless the walk holds the tick tid AND tid 1 AND is under
  TASKS_MAX. A tick tid of 0 (a failed thread_tid) is also Err. :179-193 `sibling_free_read` is Err for an empty map
  or any I/O allowed list not read. affinity() (tick_loop.rs:1326-1357) maps Err to 0 plus one stderr line, and P14
  fails the new fixture. Q-U6A-5's "failed read written as 0 and on stderr" holds. Side effect found: the I/O lists
  changed from `filter_map` to `map` (:1331-1338), so a tid that has EXITED now reads as a failed read and
  `sibling_free` is 0. `on_thread_start` (:1230) records every thread the runtime starts, tokio's blocking-pool threads
  included, and those exit after 10 s idle. U6a's runtime starts none (no spawn_blocking, no tokio fs; the workers live
  until shutdown, and the end read runs before it, :1461-1465), so nothing breaks in U6a. U6b's transports may start
  some (tokio fs for the certificate, spawn_blocking), and then every run's end read would say `sibling_free` 0.
  -> NR2-U6A-2 (Low, carry U6b): leave out a tid that is no longer under /proc/self/task; None only for a live one.
- (3) NR-U6A-4 CLOSED. Seconds (tick_loop.rs:950-1103): each failed read leaves its keys out, with a stderr line at
  its first failure; CPU is None when either end's read failed; cpu_ns (proc_read.rs:290-311) is None, never a 0, and
  a thread that ended mid-read is left out (a better rule than round 1's 0). How the judge reads it: SecondV collects
  by `filter_map` (eval.rs:193-199), so a key missing from one record is skipped, and only an all-empty series is a
  Gap (eval.rs:314-327). A failed read is therefore never a best value, and an always-failing read is a visible Gap.
  Side effect: a steal read with no jiffy since the last read (StealReader "no time passed") is now a missing key.
  For the judge that is benign (the second is skipped, which is right: steal over no time is undefined). Two small
  costs:
  - it prints the "failed" stderr line for a read that did not fail;
  - the pinned test asserts `steal_permille` on EVERY second record, the end's close record included
    (tests.rs:843-851). That record's read lands about 300 ms after its baseline when the 40 ticks finish near 1.3 s,
    but on a loaded host that only reaches 40 ticks just past the 2.0 s edge, it can land under 10 ms after the
    baseline: a rare flake.
  -> NR2-U6A-3 (Low).
- (4) FP-5 CLOSED. tick_loop.rs:661-669 (Kick(replaced)) and :753-761 (RoomEnded) add 1 to out_full when the offer
  fails; the session is closed either way, and nothing else changed there. No test, as the list said. out_full has
  no series in U6a, so the count waits for U6d's or U6e's reader.
- (5) FP-6 CLOSED, and the rule named. counted() (tests.rs:1033-1050) applies the lab's rule (mirror.rs:324-346:
  every other soldier, and ES's vehicles, in the set; no own-soldier term) over `c.relevant()`, and the whole SceneRow
  is compared on all 90 ticks; the builder reports no tick differed, and a bit was set on every tick. Why they agree:
  find_relevant leaves the set empty for a client with no soldier (room.rs:1172-1174), so the server's `own &&` and the
  lab's rule give the same bit whenever another soldier exists. They differ only in a room with no other soldier,
  which no SS or ES run has. Ruling-ready: keep the server's rule (the room's relevant set with its own-soldier term),
  since the mark's words name "every client's relevant set" (SERIES.md:273). The lab read its mirrored frame (its
  baseline plus deltas), a different source with the same rule. Nothing for Mustafa.
- (6) Nothing broken in round 1's scope. The function offsets in tick_loop.rs move only by the FP-3, -4 and -5 lines:
  wake :498, tick :520, apply :586 and join :651 are unchanged, and send_outputs, end, poll and shutdown shift by +3,
  +6 and +64 with bodies as before (shutdown read whole, :1460-1500). run() keeps the round-1 order after the new
  refusal: tokens, SHA-256, log create_new, header, TickLoop, cpu_read, OPEN. `local_machine` moved before mint, still
  inside -16's time. exit_code (main.rs:548-554) maps as round 1 did (Clean 0, otherwise 3, Err 1). FP-7 (carry.rs
  under A and B) leaves the cap and its B legs unchanged, and P7 still fails the B up leg. Under A, buffer_full stays
  0 even with the cap removed (u6a-build.md:313-314), so the A legs are coverage, not a second proof of the cap; the
  carry names only B's test, so nothing is missing. FP-8, -10 and -12 are test-only; FP-11 is an assert. No clock,
  hash-map, socket or LEAD-BOUND change; no token or client string reaches any new stderr line (the lines name keys,
  vCPU lists and reasons only).
- Evidence: check4.log:232-243 all 11 PASS (test 793.8 s; measured 749 = 709 + 40, bites 849 = 806 + 43; 0 failed, 0
  skipped); fp-restored.log's tree (diff 0fc8865b, untracked 421bff63) is check4's; the five planted files are at
  their clean sha256.

## Verdict: APPROVE WITH FINDINGS
NR-U6A-1 (High) is closed. npt-super's header now carries the run's bite, tick_hz and scene, so its log can sit in the
room's run, and the -16 bite is shown on npt-super's real header form. NR-U6A-2, -3, -4 and -8 are closed as the
rulings ask, and FP-6 found no tick differing. No fix opens a Critical or High. Four Lows follow, each a carry or a
report line; none needs a fix in U6a before merge.

## Findings (round 2)
| id | sev | where | the failure scenario | smallest fix / taker |
|---|---|---|---|---|
| NR2-U6A-1 | Low (carry U8) | crates/super/src/main.rs:181-213 with crates/judge/src/eval.rs:47-61, :560-564 | npt-super's header fields now come from flags, separate from the room's settings file. If U8's run script lets any of set, role, layer, profile, date, scene, tick_hz or bite differ, the run is INVALID. A super log in its own run id cannot hold a bite run (it names a bite with no bites-build log). | U8's carry, amended: npt-super's log takes the room's run id AND those eight values from the same source as the room's settings, for every run, bite runs included; it ends by its stdin closing, never a kill. |
| NR2-U6A-2 | Low (carry U6b) | crates/server/src/tick_loop.rs:1331-1339 (`map(allowed)`), with proc_read.rs:179-193 | An I/O-runtime tid that has exited (a tokio blocking-pool thread, gone after 10 s idle) now reads as a failed read, so the end's `sibling_free` is 0. U6a starts no such thread. U6b's transports may (tokio fs, spawn_blocking), and then every run's end read fails -1 and PM-0006-11. `io_tids` also keeps dead tids up to its cap of 256. | U6b: in affinity(), leave out a tid with no /proc/self/task/<tid> (exited), so `None` means a failed read of a live thread only; and U6b's test starts a blocking thread before the end read. |
| NR2-U6A-3 | Low | crates/server/src/tick_loop.rs:1037-1046 (the `failed` line), proc_read.rs:70-76 (StealReader's "no time passed"), tick_loop/tests.rs:843-851 | A steal read with no jiffy since its baseline is now a missing key. The judge simply skips that second (eval.rs:193-199), which is right, but the stderr line calls it a failed read. And the pinned test, which asserts `steal_permille` on every second record including the end's, can flake on a loaded host where 40 ticks finish just past a second's edge. | Either keep "no time passed" out of the failed flag (no stderr line), or let the pinned test skip the close record's steal. Merge-report line if not fixed. |
| NR2-U6A-4 | Low (report line) | crates/server/src/tick_loop.rs:1445-1449 | `TickLoop::io_on_a_tick_sibling` is a new pub item beyond the brief's list (plan §4.7's whole-surface rule). | U6a's report names it with its reason: the binary's exit-2 check reaches the crate-private sibling rule through it. |

## What I did not verify
- No cargo or judge run: (1) is read from eval.rs and the header code. No judge was run over a room log beside a super
  log written with matching flags.
- That tokio's `on_thread_start` fires for blocking-pool threads: from tokio's design as I know it, not read in this
  tree's vendored source. NR2-U6A-2 rests on it.
- The no-jiffy flake's odds on a loaded host (reasoned, not measured).
- FP-7, -8, -10 and -12's test quality: the test seat's lane.
- The size figure 5,843 is the builder's derived count (fp-size2.log stopped at the disk floor), not a measured one.

DONE
