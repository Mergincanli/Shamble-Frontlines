# U6a round 2, test seat (NPT-TRIM: what the fix pass changed only), 2026-11-23

Inputs: u6a-fix-pass.md (FP-1 to FP-12), u6a-build.md (FP-n lines), wip/u6a-build.patch (sha256 022ddb84...98e1,
checked; 19 files; 5,843 added lines counted in the patch), .gate/u6a/fp*.log, check4.log, fp-restored.log. Tree read
only: W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff/spikes/netcode-proof
Round 1: u6a-review-test.md (TR-U6A-1 to -12). Nothing run: no cargo, no plant, no runner call.

## Plan
1. Check the patch hash; read u6a-fix-pass.md and the FP lines in u6a-build.md. DONE.
2. (1) TR-U6A-1: super's new header test and the reworked double-load bite vs -16's check and the judge's rules. DONE.
3. (2) Each new or changed test's assertions (FP-2, -3, -4, -6, -7, -8, -9, -10, -11, -12). DONE.
4. (3) Plants: P7/P7b on the final tree and option A; P8, P9 (+FP-4), P12-P14; fp-restored.log's hashes. DONE.
5. (4) check4's counts against section 3's bars. DONE.
6. (5) Anything the fixes broke in round 1's scope. DONE.
7. Verdict, table, not verified, DONE. DONE.

## Notes
- Patch sha256 022ddb84...98e1 matches; 5,843 added lines in it; the five new test functions, `static FAKE` and
  `with_cmd_timing(timing)` are in it (the snapshot is the fix-pass tree).
- (1) FP-1: npt-super takes --bite (optional), --scene and --tick-hz (both required; 1-1,000) (super main.rs:97-107,
  :136-137) and log() writes them (:201-208); features stay [] (npt-super has no bites feature). New test
  h_the_log_header_carries_the_runs_bite_scene_and_tick_rate (:560-593) runs the real time_starts and log() against a
  fake room opening after 2.1 s and asserts on npt-super's own super.jsonl: header (Some(bite), "SS", 30) and max
  spawn_to_open_ns >= 2e9, the check named in the message. P12 (log() back to None/""/0) fails it (fp-plant-p12.log:
  main.rs:573, left (None, "", 0)). binary.rs's double-load bite (binary.rs:222-240) drops the hand-built log and
  asserts the room's OPEN >= 2 s, the room header's bite and features ["bites"]. So -16's check now fails by name on
  the log npt-super itself writes, and that header can sit in the room's run (the judge's eight consistency fields:
  set, role, layer, profile and date by the old flags; bite, tick_hz and scene by the new). TR-U6A-1 CLOSED. Left by
  design: the judge never runs on it (an inline copy of the check), and no test runs npt-super against the real bites
  room (two links each tested: the room's OPEN >= 2 s; npt-super's timer and header). --bite and --scene are free
  text, not checked against the room's (a mismatch makes the run INVALID, never a pass): note only.
- (2) FP-2 (main.rs:431-446, test :785-807): sibling_refusal on a fixed 2-thread map: io [1,7] refused with the line,
  [1,2] and no pinning not. P13 bites it. The call in run() (:444-446, exit 2 before minting) is reached by no test:
  this container's map is 1 thread a core, so no binary test can trigger it; a removed call passes -> TR2-U6A-3.
- FP-3 (proc_read.rs:166-193, fixtures :418-447): empty walk, walk without tid 1, walk without the tick thread -> Err;
  whole walk Ok(true); empty SMT map and a None list -> Err. P14 bites it. Not fixtured: the TASKS_MAX branch (:167);
  affinity()'s map of Err to 0 with its stderr line (tick_loop.rs:1326-1342) is read, not tested -> in TR2-U6A-3.
- FP-4 (tick_loop.rs:945-1097, test tests.rs:868-897): fed "" and a text without cpu3: no steal and no rss key, flags
  [true, true, false], CPU shares 500 and 100; then a good text: steal 100 (the StealReader kept its baseline across
  the failures), rss 4,096, CPU None -> no CPU keys and flag set. Asserts what FP-4 says. P9 also fails it (:889).
- FP-4 SIDE EFFECT (new): StealReader::read returns None also when no jiffy passed between two reads
  (proc_read.rs, `total ... filter(|t| *t > 0)?`), and values() now treats that None as a failed read: key left out
  and a "failed" stderr line (tick_loop.rs:1037-1046). The pinned test still asserts steal_permille on EVERY second
  record (tests.rs:843-851). The second-0 record's read happens when the writer thread handles tick 30's row, the last
  record's at close (stop); a writer lagging the tick thread by about 300 ms (rows handled in one burst at the stop)
  puts both reads in one jiffy (10 ms), so the last record lacks the key and the test fails. Round 1 wrote 0 there, so
  the flake is new (the builder's note, u6a-build.md:253-254, saw the case but only for a stop within 10 ms of a write,
  not for writer lag). On RS the same case gives a false "failed" stderr line and a gap in the last record ->
  TR2-U6A-1 (Medium).
- FP-6 (tests.rs:1033-1062, :1118-1149): counted() computes everyone by the lab's rule (every other soldier, ES
  vehicles; no own-soldier term) over c.relevant(); the override is gone, the whole SceneRow is compared on all 90
  ticks, plus seen > 0. This proves the server's `own &&` rule equals the lab's rule on the chosen set (they differ only
  for a slot with no soldier in a room with no other soldier). Still untested by construction: the lab reads the
  delivered frame, and here relevant() stands for it (the test's own comment, :1033-1034). Not-verified line.
- FP-7 (carry.rs:99-228): four legs (A and B, each down and up), B now explicit (:111-114), all run before the
  asserts, every leg's refusals printed in the failure message.
- FP-8: (a) s5_every_counter (tests.rs:1225-1235) waits until 40 ticks AND a client snapshot, at most 4 s, then asserts
  both: tied to the loop's progress; seconds >= 2 now follows from 40 ticks (1.3 s of loop clock). (b) pinned test
  (:814-819) the same, 40 ticks. (c) `static FAKE` Mutex (super main.rs:407-414) held by all three fake-room tests
  across fake() and the spawns (:485, :534, :562); the only forking tests in that binary. (d) `held` taken before the
  ender spawns (:493). Each holds. The three fake tests now run in turn (~3.5-4.4 s together; the builder's 4.37 s for
  the binary): the last to get the lock takes about 3.7 s of wall time, inside 5 s.
- FP-9 (main.rs:546-564, test :716-722): exit_code Clean 0, Crash 3, Err 1; finish() uses it. Holds (not planted;
  by reading it bites a wrong mapping).
- FP-10: binary.rs start() (:60-81) reads the first line on a thread, recv_timeout(5 s), kills and waits the child, then
  panics: no hang. The stuck-writer test (tests.rs:1193-1211) runs 1,064 wakes on a thread awaited 5 s: no hang. Both
  hold. New effect: the double-load bite's start shares the 5 s cap although 2 s of it are the bite's own sleeps
  (bitten OPEN 2,660 ms in this container, b3-test2.log); under load it fails at the cap where it used to pass slowly
  -> TR2-U6A-4 (Low).
- FP-11 (main.rs:775-777): burn 0 and top-up 0 asserted. Holds.
- FP-12 (host_step.rs:114-136): the forger leg's second step puts its forged Ping last on out.reliable, on Control.
  Dropping or moving `self.reliable(..)` (bots lib.rs:897) fails it by reading (no plant). Holds.
- (3) P7 on the final tree (fp-plant-p7.log, 03:07): cap removed -> FAILS at carry.rs:224, "B, its ping slow (up)" 6
  refused; A down, A up, B down 0 refused at lead 9. Why A never bites: under option A a command runs at the first
  tick after it arrives, and kind 10 counts the commands held for the coming tick at arrival (room input.rs:42-43,
  :316-326, :336); the stamp, the only thing the controller's cap moves, plays no part in A's kind 10. So "the cap holds
  under option A" is true by A's own rule, not by the cap: no plant of the cap can bite the A legs, which are a
  non-regression run of the honest client under A. What closes C-U6-17's A half: the report states it as proven by
  reading (input.rs:316-326, :336) with the netcode seat's yes, and names the A legs as non-regression, not proof;
  the test's name still says "under_option_b" -> TR2-U6A-2 (Low).
- P8 (fp-plant-p8.log): FAILS proc_read s4_alone... at :390 (4 passed, 1 failed). P9 (fp-plant-p9.log): FAILS
  s4_the_steal_reader... (:335, Some(27) vs Some(100)) and s4_a_failed_per_second_read... (tests.rs:889). P12, P13,
  P14 as above. Each fails its named test(s), exit 101, no compile error; the runner's diff or untracked hash is off
  the clean tree's while planted. fp-restored.log: client 1613a3f0, proc_read b231a3f6, super 0e2a4269, server main
  ef115163, tick_loop 7dd0a9bf; tree diff 0fc8865b / untracked 421bff63 = check4's tree; the five files hash the same
  now (my sha256sum, 2026-11-23). The fix-pass plant calls ran `cargo test` without `--locked` (round 1's had it); the
  next call's diff hash, which covers Cargo.lock, is the clean tree's, so the lock did not move -> in TR2-U6A-5.
- (4) check4.log: runner diff 0fc8865b / untracked 421bff63 (the fix-pass tree); both --no-run builds exit 0
  (:74, :230); all 11 steps PASS (:232-242); measured 749 = 709 + 40, bites 849 = 806 + 43 (bars +33 and +36: met;
  +5 each since round 1 = the five new tests); 0 failed, 0 reported a skip; test step 793.8 s = 0.74 of base's
  1,070.2 s (stop at 1.5x: clear).
- (5) What the fixes broke in round 1's scope: TR2-U6A-1 (FP-4's no-time gap makes the pinned test flaky) and
  TR2-U6A-4 (FP-10's cap on the bitten start). Also: FP-1 makes --scene and --tick-hz required, so starts2.log's
  recorded 20-start command no longer runs as written (U8's run plan and any re-reading must pass them; the reading is
  not re-taken) -> in TR2-U6A-5. FP-2 adds a pub item, `TickLoop::io_on_a_tick_sibling` (tick_loop.rs:1447), for the
  report's pub-items list (the netcode seat's read) -> in TR2-U6A-5. Nothing else in round 1's scope moved: the
  shared-vcpu and no-smt-map bites, the S1 tests, C-U6-18's first legs and the counts all hold.

## Verdict: APPROVE WITH FINDINGS
TR-U6A-1 (High) is closed: -16's check now fails by name on npt-super's own log (P12 bites it), and the room-side bite
asserts the delay, the bite and the bites build. No fix opens a Critical or High. One new Medium (TR2-U6A-1: FP-4 turned
a no-time steal read into a gap that the pinned test forbids) is a timing-dependent test opened by a fix; it needs
about 8 lines, so it can be fixed before merge or recorded as a carry within the budget. The rest are Lows for
carries.

## Findings (worktree paths under W)
| id | severity | worktree file:line | failure scenario | smallest fix |
|---|---|---|---|---|
| TR2-U6A-1 | Medium | crates/server/src/tick_loop.rs:1037-1046, :1079; crates/server/src/proc_read.rs (StealReader::read's `total > 0`); crates/server/src/tick_loop/tests.rs:843-851 | FP-4 counts StealReader's "no time passed" None as a failed read: the key is left out with a "failed" stderr line. When the writer thread lags (rows handled in a burst at the stop), the second-0 write's and the close's /proc/stat reads fall in one jiffy, the last record lacks steal_permille, and s4_a_pinned_room... fails although nothing failed. On RS the last record gets a false failure line. | Tell the two apart: StealReader::read returns a "no time" result apart from a missing line (an enum or a `(Option<u32>, bool)`); values() leaves the key out on "no time" without a stderr line or setting the failed flag; the pinned test asserts steal on every record but the last (the close's partial second), and FP-4's test gains one "no time" case (two equal texts: no key, no flag). About 8-10 lines. |
| TR2-U6A-2 | Low | crates/server/tests/carry.rs:198-228 | The A legs pass with or without the cap (fp-plant-p7.log), because option A's kind 10 counts arrivals per tick, not stamps (room input.rs:316-326, :336). C-U6-17's "holds under both options" is proven for A only by that rule, which no test states; the test's name says "under_option_b". | The report states C-U6-17's A half as proven by reading (input.rs:316-326, :336) with the netcode seat's yes and calls the A legs a non-regression run; optionally rename the test (…_under_options_a_and_b). No new test can make a cap plant bite A. |
| TR2-U6A-3 | Low | crates/server/src/main.rs:444-446; crates/server/src/tick_loop.rs:1326-1342; crates/server/src/proc_read.rs:167 | FP-2's call in run() and FP-3's map of Err to 0 (with its stderr line) in affinity() are reached by no test (this container's SMT map is 1 thread a core; affinity() reads the live /proc); the TASKS_MAX branch has no fixture. Removing the call or the map passes. | Not-verified lines in the report; optionally a `preflight(&Settings, &[Vec<u32>]) -> Option<(u8, String)>` test, and a 65,536-task fixture for the cap (about 6 lines). |
| TR2-U6A-4 | Low | crates/server/tests/binary.rs:74 (with :222-240) | The double-load bite's start shares the 5 s first-line cap although 2 s of it are the bite's own sleeps (2,660 ms here); a loaded runner can push it past 5 s and fail a correct bite. | Pass the cap to start(): 5 s for the measured start, 5 s + 2 s for the bitten one (about 3 lines). |
| TR2-U6A-5 | Low | .gate/u6a/fp-plant-p*.log; .gate/u6a/starts2.log; crates/server/src/tick_loop.rs:1447 | Report items: the fix-pass plant calls ran without `--locked` (the lock did not move: the next diff hash is clean); starts2.log's recorded 20-start command lacks the now-required --scene and --tick-hz; FP-2 adds the pub item `TickLoop::io_on_a_tick_sibling`. | Name all three in the report (pub-items list; the starts command as it now must be); no code change. |

Counts: High 0, Medium 1, Low 4.

## Not verified
- I ran nothing: plant results, counts and timings are the logs'; I read every fp-plant log, fp-restored.log and
  check4.log's provenance and summary lines, and hashed the five planted files now.
- Not tested in the unit, by construction or by this container: the judge on any U6a log (each check is an inline
  copy); npt-super against the real bites room end to end; a run whose -16 bite logs are only npt-super's (needs a
  bites-build room log in the run: U8's run plan); everyone_in_view on the delivered frame rather than the chosen set;
  FP-2's refusal through the binary on a 2-thread machine; affinity()'s Err-to-0 path on a real failed walk.
- That TR2-U6A-1's flake has happened: it is read from the code and the test, not seen; check4 passed.
- FP-9 and FP-12 have no plants; their biting is by reading.

DONE
