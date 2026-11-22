# U6a build review, test seat (round 1 of 1, NPT-TRIM), 2026-11-22

Inputs: u6a-brief.md rev 1.1 (sha256 c109691a...05ea, checked), wip/u6a-build.patch (sha256 a9f8b6e0...1d39, checked;
base aa5c62b6), built tree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff (read only).
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff/spikes/netcode-proof
One question: is the unit proven, as section 3 (amended by 0.1) says it must be. No Edit tool used: rewritten whole.

## Plan
1. Check the two input hashes (brief, patch). DONE.
2. Counts: list every new #[test] in the patch by group; compare to section 3's groups; read each test's asserts. DONE.
3. Plants P1-P11, P7b: read each log (failing test name, reason, not a compile error), restore hashes vs clean.
4. Bites: binary.rs's double-load test and its "npt-super form" log vs super/src/main.rs log(); no-smt-map and
   shared-vcpu bite tests; the judge's checks by name. Mostly DONE (see notes).
5. Timing: every test that reads a real clock or sleeps; injected clock where a time is asserted; 5 s cap; OPEN time
   reported; flakiness risks by test name. Mostly DONE.
6. C-U6-17 carry test (carry.rs) and C-U6-18 order test (host_step.rs) against section 4's words. TODO.
7. Header truthfulness: F4 and carry 60 params asserted by a test? DONE.
8. Item 7: -2 everyone flag parity; /proc/stat failed read -> steal 0. Parity DONE; steal: read tick_loop.rs:1000-1040.
9. SERIES.md rows (condition 1's list) written and equal to the code's keys. Mostly DONE.
10. Section 3's not-verified list gaps. TODO.
11. Verdict, findings table, not verified, DONE.

## Notes (one line per finding, as found)
- Hashes: brief c109691a...05ea and patch a9f8b6e0...1d39 both match. 19 files in the patch; SERIES.md is NOT among them.
- COUNTS (by #[test] in the patch): link 2 (T6), proto 1 (channel_of, extra to S1's 10), server lib tick_loop/tests.rs
  15 measured (S1 10: schedule, catch-up, flood, decode-thread, RTT, 262,144 B, replacement, panic, PhaseTimers, smoke;
  S4 1 pinned run; -2 4) + 1 bite (shared-vcpu); proc_read 4 (steal, rss+schedstat, alone, sibling_free); server bin
  main.rs 5 (settings taken, refused, token file = binary 3; cpu_read, header+carry 60 = S4 2); binary.rs 2 measured
  (OPEN<5 s+exit 0+no token; exit codes) + 2 bites (double-load, no-smt-map); log 1 (read_local, S4); super 3 (fake,
  silent, flags); carry.rs 1; bots host_step 1. Measured +35 (T6 2, S1 10, binary 5, S4 8, -2 4, super 2+1, carry 1,
  order 1, proto 1); bites +38 = 35 + 3. Every group at its count. check3.log's 744/844 = 709+35 / 806+38 agrees.
- S1 asserts read: schedule test exact ticks/jitter (injected); catch-up 7 and dropped on first tick, sums = counters;
  flood both ways (inbound_full = 10,000 - 4,096 + 1; out_full > 0; every due tick ran); RTT 42 -> 77 ms and ws None;
  262,148 B frame -> FullState + Kick -> Control; replacement (A's Leave not routed, Kick(replaced) last, link Closed;
  B's Leave frees slot); panic (RoomEnded(crash), Ended::Crash, 8 tick records, tick_panic server_ns, crash footer);
  PhaseTimers exact (800, 250, 400, true, 3; phases 50/600/150; via core reads 100/110/150/170 -> work 70, jitter 100).
  Each asserts what section 3 says. Kicked-session close (O1) has no test (builder's own open item): section 3 says
  "kicked/left/replaced closing"; kicked is untested -> finding TR-U6A-5 (Low/Medium, see table).
- BINARY: main.rs exit mapping finish(): Ended::Clean -> 0, Ok(other) -> 3 (crash), Err -> 1. No test drives the binary
  to exit 3 (crash) or 1; the binary test proves 0 and 2 only. Section 3's "exit codes" and U6-Q17 term (2)'s "non-zero
  exit": TickLoop-level Ended::Crash is tested (P4 bites it), the binary's map to 3 is not -> TR-U6A-6 (Low): fix a
  pure fn exit_code(Result<Ended,_>) -> u8 with a 4-line unit test (Clean 0, Crash 3, Err 1).
- BINARY start() helper (binary.rs:60-80): read_line blocks with no timeout; a room that hangs before OPEN hangs the
  test (the 5 s cap is asserted only after the line arrives) -> TR-U6A-7 (Low): read the first line on a thread with
  recv_timeout(5 s), kill the child on timeout (~8 lines).
- HEADER/F4: s4_the_header_names... asserts all 7 carry-60 params == Scene::weapon()/health() for SS, ES, CO, TC
  (main.rs:730-738), stand_ins [] (:739), cmd_timing; it does NOT assert stand_in_burn_ns 0 or stand_in_top_up_ns 0
  (main.rs:381-382 write them) -> TR-U6A-4 (Low): two assert_eq lines. Key names equal the lab's (scenario.rs:2037-2045).
  Nothing in crates/server reads Scene::stand_ins() (grep), so [] and 0 are true "as run"; the binary's SS scene carries
  StandIns::s1() (scene.rs:188) while the header says [] -- truthful about burning, untested as a pair (no test that
  TickLoop burns nothing; trivially none exists).
- BITE double-load (binary.rs:206-236): builds its "npt-super form" log from the ROOM's header with only process
  changed (:215-218): bite Some, tick_hz 30, scene SS, features [bites]. npt-super's real log() (super/src/main.rs:
  161-193) writes bite None (:181), tick_hz 0 (:186), scene "" (:188), features [] (:173). The judge groups logs by
  run and makes a run INVALID when its logs disagree on bite, tick_hz or scene (judge/src/eval.rs:47-61, :542-547), and
  a run naming a bite needs a bites-build log (:548-564). So the test's log is not the log the real binary writes, and
  the evidence that the bite "fails its mark's check by name on the log it writes" does not hold for npt-super's log
  (same defect as the netcode seat's FINDING C, seen from the test side) -> TR-U6A-1 (High). The check itself is inline
  (max >= 2e9), equal to marks.json -16's "every start, process spawn to Open, under 2 s" (max < 2,000,000,000).
  No npt-super test asserts any header field of log() (super tests read only process "super" and events).
  Smallest test fix (beside netcode's code fix of --bite/--scene/--tick-hz flags): one npt-super test,
  h_the_log_header_carries_the_runs_bite_scene_and_tick_rate (~20 lines): a fake room "sleep 2.1; echo OPEN; cat",
  flags with --bite PM-0001-16/double-load --scene SS --tick-hz 30, then read super.jsonl: header bite/scene/tick_hz as
  given and max spawn_to_open_ns >= 2e9; and binary.rs's bite drops its hand-built super header (asserts only the
  room's OPEN delay >= 2 s).
- BITE no-smt-map (binary.rs:240-254): real binary, real log; machine_named is a local copy (binary.rs:89-107) that
  matches judge/src/figures.rs:742-764 field for field today; a copy can drift (npt-server has no npt-judge dev-dep and
  condition 5 forbids a new one). Holds; note only.
- BITE shared-vcpu (tick_loop/tests.rs:871-916): the log assertion min(alone) == 0 cannot fail in this container
  (every process may run on every vCPU, so alone is 0 without the bite: b4-test1.log's pinned reading alone 0). The
  bite-specific proof is the direct read: allowed(bite_tid) == [tick] and not_alone(tasks("/proc")) contains bite_tid
  (:889-896). So the log half is vacuous here; the read half bites. Whether the START event counts the bite thread
  (bite spawned before the start's read?) is untested here -> part of TR-U6A-3 (see below). Need to check the order
  in tick_loop.rs:1200-1240 (TODO).
- -2 PARITY (item 7a): s5_..._minus_2_counts sets everyone 0 on both sides (tests.rs:1071-1078, and counted() at
  :1006), so the flag is never compared with the lab's rule. Differences found by reading: the server sets the bit only
  when the slot's own soldier is in the records (`own &&`, tick_loop.rs:288-289) and reads ClientSlot::relevant (empty
  for "one with no soldier", room.rs:360-363); the lab's mirror (mirror.rs:324-346) needs only every OTHER soldier (and
  ES vehicles) in the client's rebuilt frame, no own-soldier term, and reads what was delivered, not what was chosen.
  So a dead (despawned) soldier's client, or a tick with no snapshot sent, can read 0 on the server and 1 on the lab
  (or the reverse). The judge counts every measured tick per slot (figures.rs:24-55), so -2's 0.99 share can differ.
  -> TR-U6A-2 (Medium). Test that closes it (~25 lines): in s5_..._minus_2_counts, compute `everyone` in counted() by
  the lab's rule (every other soldier, ES vehicles, index in c.relevant()) and compare the whole SceneRow, over a run
  with a kill (the 90-tick SS run has 48 hits+kills); any difference is then a named design choice for the netcode seat.
- Timing (so far): injected clock in every S1 test that asserts a time (schedule, catch-up, PhaseTimers, RTT, flood,
  replacement, 262,144 B, -2 x3 bench tests). Real-clock tests: decode-thread (<= 3 s wait_for), panic (<= 3 s), pinned
  run (sleep 2.3 s), every-counter (1.3 s), smoke (2 s), binary OPEN (< 5 s asserted, 300 ms hold), double-load (>= 2 s
  by sleeps; plus 300 ms), super fake (~0.9 s), silent (0.5 s). All <= 5 s. No timing bar asserted in the container
  except the 5 s cap and super's (200 ms..5 s) and took < 5 s.
- Flake risks named (TR-U6A-3, Medium, grouped): (a) s5_every_counter_is_on_every_record asserts log.seconds.len() >= 2
  and any everyone_in_view[0] == 1 within 1.3 s of real time (a loaded CI host: the client must join, ack its full
  state and get a snapshot) [tests.rs:1166-1203]; (b) s4_a_pinned_room... asserts seconds.len() >= 2 after a 2.3 s
  sleep (tests.rs:814-837) -- depends on how the writer cuts seconds (TODO: read the writer); (c)
  s1_smoke_... asserts log.ticks.len() >= tl.ticks() -- false if the writer queue (ROWS_CAP 1,024, tick_loop.rs:55)
  dropped rows; 60 ticks in 2 s cannot fill 1,024, so safe; (d) s1_a_tick_thread_panic... asserts exactly 8 tick
  records: rows only drop past 1,024, so safe; the RoomEnded goes to every session (end(), tick_loop.rs:738-757) joined
  or not, so the Join's timing does not matter; safe; (e) npt-super's two fake-room tests write an executable script
  and exec it while the other test may fork (ETXTBSY "Text file busy" race, a known Rust test flake) [super
  main.rs:389-395, :452, :499] -> Low fix: one static Mutex held across fake() and the spawns, or one test for both;
  (f) h_time_starts... `held >= 300 ms` compares a clock started after the ender thread was spawned (:456-463): a
  sub-microsecond margin, Low fix: take `held` before spawning the ender.
- SERIES.md (item 8): no SERIES.md row in the diff; the rows exist at base: loop five (rows 30, 35, 54, 59, 60,
  npt-server), per-second steal/rss/proc/io (98, 93, 88, 81, npt-server), out_queue_count (416), affinity.alone and
  sibling_free (129, 130), cpu_read.changed (151), tick_panic.server_ns (423), room_start.spawn_to_open_ns (200,
  npt-super), room_exit.nonzero (199), the -2 names (22, 31, 43, 51-53, 58, 87, 89, 91, 94, 96, 97, 406; writer cell
  "npt-room" while npt-server writes them: register 98's deferral, brief section 1), carry 60 (352-353, writer cell
  "npt-lab"). Every key the code writes (tick_loop.rs:1020-1094, :760, :1294-1297; main.rs:369-382; super :283-289)
  has a row with the same name. TODO: confirm condition 1's exact list ("five per-second reads" = which fifth; "three
  events" = which) from advisor-u6-brief.md; check header.params.stand_ins's row (SERIES.md:444 is a different table).

## STOPPED AT (for a restart)
Stopped while reading tick_loop.rs (read to :800). Next: tick_loop.rs :800-1447 (writer seconds cut, steal None ->
0 at :1019-1021 and any stderr line, affinity() and the bite thread's order vs the start read :1200-1300, stop());
then carry.rs and host_step.rs (item 5); proc_read.rs tests (fixtures: per-CPU kthread excluded, unbound counted, user
thread counted, per condition 4); the plant logs .gate/u6a/plant-p*.log (names, reasons, restore hashes) and
starts2.log; advisor-u6-brief.md condition 1's list; then verdict, table, not-verified, DONE.
