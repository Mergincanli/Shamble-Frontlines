# U6a build review, test seat (round 1 of 1, NPT-TRIM), 2026-11-22 (resumed 2026-11-23 after the usage cut)

Inputs: u6a-brief.md rev 1.1 (sha256 c109691a...05ea, checked), wip/u6a-build.patch (sha256 a9f8b6e0...1d39, checked;
base aa5c62b6), built tree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff (read only).
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff/spikes/netcode-proof
One question: is the unit proven, as section 3 (amended by 0.1) says it must be. No Edit tool used: rewritten whole.
Nothing was run: no cargo, no plant, no runner call; every claim below is from reading the tree and the logs.

## Plan
1. Check the two input hashes (brief, patch). DONE.
2. Counts: every new #[test] by group vs section 3's groups; each test's asserts. DONE.
3. Plants P1-P11, P7b: each log (failing test name, reason, not a compile error), restore hashes vs clean. DONE.
4. Bites: double-load's "npt-super form" log vs super's log(); no-smt-map; shared-vcpu. DONE.
5. Timing: real-clock tests, 5 s cap, OPEN time reported, flake risks by name. DONE.
6. C-U6-17 carry test (carry.rs) and C-U6-18 order test (host_step.rs). DONE.
7. Header truthfulness: F4 and carry 60 params asserted? DONE.
8. Item 7: -2 everyone flag parity; failed /proc/stat -> steal 0. DONE.
9. SERIES.md rows (condition 1's list) vs the code's keys. DONE.
10. Section 3's not-verified list gaps. DONE.
11. Verdict, findings table, not verified, DONE. DONE.

## Notes (one line per finding, as found)
- Hashes: brief c109691a...05ea and patch a9f8b6e0...1d39 both match. 19 files in the patch; SERIES.md is NOT among them.
- COUNTS (by #[test] in the patch): link 2 (T6), proto 1 (channel_of, extra to S1's 10), server lib tick_loop/tests.rs
  15 measured (S1 10: schedule, catch-up, flood, decode-thread, RTT, 262,144 B, replacement, panic, PhaseTimers, smoke;
  S4 1 pinned run; -2 4) + 1 bite (shared-vcpu); proc_read 4 (steal, rss+schedstat, alone, sibling_free); server bin
  main.rs 5 (settings taken, refused, token file = binary 3; cpu_read, header+carry 60 = S4 2); binary.rs 2 measured
  (OPEN<5 s+exit 0+no token; exit codes) + 2 bites (double-load, no-smt-map); log 1 (read_local, S4); super 3 (fake,
  silent, flags); carry.rs 1; bots host_step 1. Measured +35 (T6 2, S1 10, binary 5, S4 8, -2 4, super 2+1, carry 1,
  order 1, proto 1); bites +38 = 35 + 3. Every group at its count. check3.log:335 (744 / 844, 0 failed, 0 reported a
  skip) = 709+35 / 806+38 agrees; all 11 steps PASS (check3.log:331-341).
- S1 asserts read: schedule test exact ticks/jitter (injected); catch-up 7 and dropped on first tick, sums = counters;
  flood both ways (inbound_full = 10,000 - 4,096 + 1; out_full > 0; every due tick ran); RTT 42 -> 77 ms and ws None;
  262,148 B frame -> FullState + Kick -> Control; replacement (A's Leave not routed, Kick(replaced) last, link Closed;
  B's Leave frees slot); panic (RoomEnded(crash), Ended::Crash, 8 tick records, tick_panic server_ns, crash footer);
  PhaseTimers exact (800, 250, 400, true, 3; phases 50/600/150; via core reads 100/110/150/170 -> work 70, jitter 100).
  Each asserts what section 3 says, but kicked-session closing (send_outputs, tick_loop.rs:728-731) has no test (the
  builder's O1) although section 3 names "kicked/left/replaced closing" -> TR-U6A-5.
- BINARY exit codes: finish() (main.rs:531-537) maps Clean -> 0, Ok(other) -> 3, Err -> 1. No test drives the binary
  to 3 or 1; binary.rs proves 0 and 2 only. TickLoop's Ended::Crash is tested (P4 bites it); the binary's map to 3,
  U6-Q17 term (2)'s "non-zero exit", is not -> TR-U6A-6.
- BINARY start() helper (binary.rs:60-80): read_line blocks with no timeout; a room that hangs before OPEN hangs the
  test (the 5 s cap is asserted only after the line arrives). Same shape: s5_the_tick_never_waits_on_a_stuck_writer
  (tests.rs:1138-1151) would hang, not fail, if the row offer became a blocking send (no reader drains the bench's
  queue) -> TR-U6A-7.
- HEADER/F4: s4_the_header_names... asserts all 7 carry-60 params == Scene::weapon()/health() for SS, ES, CO, TC
  (main.rs:730-738), stand_ins [] (:739), cmd_timing; it does NOT assert stand_in_burn_ns 0 or stand_in_top_up_ns 0
  (written at main.rs:381-382) -> TR-U6A-8. Names equal the lab's (scenario.rs:2037-2045). Nothing in crates/server
  reads Scene::stand_ins() (grep), so [] and 0 are true "as run" although the binary's SS scene carries StandIns::s1()
  (scene.rs:188). header() is the binary's one path, so the unit test covers the binary's params.
- BITE double-load (binary.rs:206-236): builds its "npt-super form" log from the ROOM's header with only process
  changed (:215-218): bite Some, tick_hz 30, scene SS, features [bites]. npt-super's real log() (super/src/main.rs:
  161-193) writes bite None (:181), tick_hz 0 (:186), scene "" (:188), features [] (:173); the real 20-start reading
  shows it (.gate/u6a/starts-super.jsonl header: bite null, tick_hz 0, scene "", features []). The judge makes a run
  INVALID when its logs disagree on bite, tick_hz or scene (judge/src/eval.rs:47-61, :542-547), and a run naming a
  bite needs a bites-build log (:548-564); -16's checks read every log of the run (marks.json -16: part select {}, no
  `from`). So the test's log is not the log the real binary writes: the evidence that the bite "fails its mark's check
  by name on the log it writes" does not hold for npt-super's log (the netcode seat's FINDING C, from the test side)
  -> TR-U6A-1 (High). The inline check (max >= 2e9) equals -16's "every start, process spawn to Open, under 2 s"
  (max < 2,000,000,000). No npt-super test asserts any header field of log().
- BITE no-smt-map (binary.rs:240-254): real binary, real log; machine_named is a local copy (binary.rs:89-107) equal
  field for field to judge/src/figures.rs:742-764 today (a copy can drift; no npt-judge dev-dep, and condition 5
  forbids a new one). Holds.
- BITE shared-vcpu (tick_loop/tests.rs:871-916): the log assertion min(alone) == 0 cannot fail in this container
  (alone is 0 without the bite: b4-test1.log's pinned reading). The bite-specific proof is the direct read:
  allowed(bite_tid) == [tick] and not_alone(tasks("/proc")) contains bite_tid (:889-896); it bites. Order checked:
  pin_tick spawns and pins the bite thread (tick_loop.rs:1238-1266) before affinity("start") (:1212-1214); the end
  read (:1397-1401) runs before stop is set and the bite thread joined (:1402-1411): both reads would see it on RS.
  Holds; the vacuous log half goes to not-verified.
- -2 PARITY (item 7a): s5_..._minus_2_counts sets everyone 0 on both sides (tests.rs:1071-1078; counted() :1006), so the
  flag is never compared with the lab's rule. By reading: the server sets the bit only when the slot's own soldier is
  in the records (`own &&`, tick_loop.rs:277-291) and reads ClientSlot::relevant (empty for "one with no soldier",
  room.rs:360-366); the lab's mirror (mirror.rs:324-346) needs only every OTHER soldier (and ES vehicles) in the
  client's rebuilt frame, no own-soldier term, and reads what was delivered, not what was chosen. The judge counts
  every measured tick per slot (figures.rs:24-55). A dead (despawned) soldier's client, or a tick whose chosen set was
  not delivered, can read 0 on the server and 1 on the lab or the reverse, moving -2's 0.99 share -> TR-U6A-2.
  everyone_in_view's own test (1 then 0, tests.rs:1099-1134) holds for the radius case only.
- STEAL failed read (item 7b): Seconds::baseline reads /proc/stat with unwrap_or_default (tick_loop.rs:983) and write()
  writes steal.map_or(0) (:1019-1021), no stderr line. Q-U6A-5's term (advisor-u6a-brief.md:82-92, as the netcode seat
  quotes it) is "written as 0 and on stderr, never hidden": the 0 is there, the stderr half is not, and no test covers
  the path (s4_the_steal_reader... covers StealReader::read's None for a missing or short line, proc_read.rs:311-313,
  not what the record then says). -1's "steal recorded every second" (count >= 1) passes on a run whose every read
  failed -> TR-U6A-9 (test half; the code ruling is the netcode seat's).
- sibling_free with an empty SMT map or one without the tick vCPU returns true (proc_read.rs:151-162); a failed /proc
  walk gives an empty task list and alone 1 (proc_read.rs:165-197, netcode's FINDING B); neither has a fixture.
  machine_named catches an empty map in the header for -1 and PM-0006-11. -> folded into TR-U6A-9.
- ADV-U6A-4 fixtures (proc_read.rs:346-366): per-CPU kernel thread on the tick vCPU excluded, unbound kernel thread
  counts, user thread counts, a thread kept off does not; flags read after the last ')' (fixture "(odd (name) x)").
  Condition 4's three fixtures present. Holds.
- C-U6-17 (carry.rs): test-local Delay link over Loopback (not L1: condition 5 holds), 10 ms path (5 ms each way,
  :42), the 3rd Ping or Pong 600 ms late (:115), option B only by the room's default (room.rs:601; not set in the
  test), controller max lead 9 (:127); asserts buffer_full (CommandRefusal::Catchup, room.rs:1037: kind 10) == 0,
  ran > 60, seen > 3. The literal carry leg (600 ms PONG, down) passes WITHOUT the cap (builder's F9; plant-p7.log: 0
  refused, exit 0), so only the added up leg (slow Ping) proves the cap; P7b bites it (plant-p7b.log: up leg 6 refused
  at lead 9, test FAILED by name). The carry's first sentence says "the controller's cap holds under both options";
  kind 10 exists under option A too (input.rs:13, :319); the test runs B only -> TR-U6A-3.
- C-U6-18 (host_step.rs): input before the step (command in the Input, :96-99), outgoing after the step (never-acker's
  ack zeroed, :100-105), honest acks (101, 1); P10 bites it. host_step also runs `reliable` after the step
  (bots lib.rs:897), which its module text names as part of the contract; no case has a reliable hook that acts (the
  forge behaviour's, lib.rs:652-656), so dropping or moving that call passes -> TR-U6A-10.
- PLANTS (logs read): P1 plant-p1.log FAILS s1_the_bounded_channels... at tests.rs:347 (left 0, right 5,905); P2
  FAILS s1_take_controls... at :579 (Control vs FullState); P3 FAILS s1_the_tick_thread_takes... at :553 (None vs
  Some(42 ms)); P4 FAILS s1_a_tick_thread_panic... at :458 (wait_for: no RoomEnded in 3 s; the tick thread's own
  panic at tick_loop.rs:539 shown); P5 FAILS s1_a_stall... at :286 (30 vs 7); P6 FAILS s1_a_replaced_session... at
  :624 ("A's Leave was not routed"); P8 FAILS proc_read s4_alone... at proc_read.rs:357 ("alone"); P9 FAILS
  s4_the_steal_reader... at :301 (Some(27) vs Some(100)); P10 FAILS c_u6_18_host_step... at host_step.rs:101
  ((101, 1) vs (0, 0)); P11 FAILS tests::s4_the_header_names... at main.rs:726:25 in the planted file (the params'
  index: "no entry found for key"); P7b FAILS c_u6_17_... (6 vs 0). Each: exit 101, "test result: FAILED. 0 passed; 1
  failed", the named test only (-k filter), no compile error, the runner's diff/untracked hash moved off the clean
  850abb56.../3c0a31cb... (the plant was in place). Each plant stands for its scope item and bites it. Restore: the five
  planted files hash now (2026-11-23) to the clean values in u6a-build.md (tick_loop.rs e2908a65...5c79, proc_read.rs
  c562e475...6ae9, bots lib.rs 5202e036...9334, main.rs 9ac7d649...1e34, client lib.rs 1613a3f0...1ec7). P7b ran at
  15:12 on the pre-fmt tree (runner diff 285f015d..., carry.rs's failing line 194 against :211 now), not on the final
  tree that check3 passed -> TR-U6A-11.
- STARTS: starts2.log builds the measured npt-server, runs npt-super time-starts --count 20, exit 0; starts-super.jsonl:
  20 room_start, p50 634.0-634.6 ms, max 978.6 ms, 20 room_exit all nonzero 0, footer clean. A debug build in this
  container: a reading, never judged.
- Timing: injected clock in every S1 test that asserts a time (schedule, catch-up, PhaseTimers, RTT, flood,
  replacement, 262,144 B, -2 x3 bench tests); carry.rs on an injected clock too. Real-clock tests: decode-thread
  (<= 3 s wait_for), panic (<= 3 s), pinned run (sleep 2.3 s), every-counter (1.3 s), smoke (2 s), binary OPEN (< 5 s
  asserted and printed, then 300 ms), double-load (>= 2 s by sleeps, + 300 ms), super fake (~0.9 s), silent (0.5 s).
  All within 5 s. No timing bar in the container beyond the 5 s cap, super's 200 ms..5 s window and took < 5 s.
  Condition 5's OPEN: asserted < 5 s and printed (binary.rs:113-121). Holds.
- Flake risks by name -> TR-U6A-4: (a) s5_every_counter_is_on_every_record asserts log.seconds.len() >= 2 after 1.3 s
  of wall time (tests.rs:1166-1203): the writer cuts a second record only when a tick starts in the next second
  (tick_loop.rs:987-1000) and the last at close, so the tick thread must run a tick at loop time >= 1.0 s before the
  stop (a 300 ms margin; check.mjs's test step runs binaries in parallel); it also needs the client joined and
  snapshotted within 1.3 s for everyone_in_view[0] == 1; (b) s4_a_pinned_room... asserts seconds >= 2 after
  sleep(2.3 s) (tests.rs:814-837): same mechanism, 1.3 s margin; (c) npt-super's two fake-room tests write an
  executable script and exec it while the other test may fork (ETXTBSY "Text file busy", a known Rust test-harness
  race) (super main.rs:389-395, :452, :499); (d) h_time_starts... `held >= 300 ms` uses a clock started after the
  ender thread was spawned (:456-463): a microsecond margin. Safe by reading: smoke's log.ticks.len() >= ticks
  (ROWS_CAP 1,024 vs ~60 rows); panic's exactly 8 records (rows drop only past 1,024); RoomEnded reaches every session,
  joined or not (end(), tick_loop.rs:738-757). The pinned tests print SKIPPED below 2 vCPUs, which check.mjs reports
  as a skip (check3.log:335 "0 reported a skip"), so a 1-vCPU runner shows it rather than passing silently.
- SERIES.md (item 8): no row in the diff, and none is needed: the rows exist at base. Loop five (rows 30, 35, 54, 59,
  60, writer npt-server); steal, rss, proc, io (98, 93, 88, 81, npt-server); out_queue_count (416); affinity.alone and
  .sibling_free (129, 130); cpu_read.changed (151); tick_panic.server_ns (423); room_start.spawn_to_open_ns (200,
  npt-super); room_exit.nonzero (199, npt-super); the 15 -2 names (22, 31, 43, 51-53, 58, 87, 89, 91, 94, 96, 97, 406,
  stand_ins 444; writer cell "npt-room" while npt-server writes them: register 98's deferral, brief section 1); carry
  60 (352-353, writer "npt-lab"); stand_in_burn_ns/top_up_ns (351, "npt-lab"); cmd_timing (445). Every key the code
  writes (tick_loop.rs:760, :1020-1094, :1294-1297; main.rs:369-382; super :283-289) has a row of the same name.
  Condition 1 (advisor-u6-brief.md:292-294): "each piece's brief names the SERIES.md rows it writes, and U6's close
  report shows every npt-server row written or named for a later unit". Gaps in the list section 3 gives: it says
  "the five per-second reads" and `out_queue_count` apart, but the code writes four second.v reads (steal, rss, proc,
  io) plus second.s.out_queue_count; and npt-super's event.room_exit.nonzero (row 199) is written but not listed
  -> TR-U6A-12. "Every other npt-server row named for U6b-U6e" is the report's, not yet written: not verified.
- NOT-VERIFIED list (section 3: "no RS, no load, no transport; the tick vCPU alone only as far as this container's
  processes go"): missing items are listed under "What section 3's not-verified list lacks" below.
- Mustafa's lane: nothing proposed changes a pass mark, the judge or marks.json. Named only: making the judge accept a
  super log that differs on bite, tick_hz or scene (eval.rs:47-61) would be a judge change and his; TR-U6A-1's fix
  goes the other way (npt-super writes the run's fields).

## Verdict: REVISE
One High (TR-U6A-1: the -16 bite's evidence is a log npt-super never writes, and the real one cannot sit in its run),
which by NPT-TRIM allows a second round on it alone. Everything else is APPROVE WITH FINDINGS: the counts hold (+35
measured, +38 bites against +33 and +36), every group is at its count, every plant fails its named test for its planted
reason with its file restored to the clean hash, the timing rules hold (injected clock where a time is asserted, each
real-clock test within 5 s, OPEN under 5 s and printed), and check3 passed all 11 steps.

## Findings (worktree paths under W)
| id | severity | worktree file:line | failure scenario | smallest fix |
|---|---|---|---|---|
| TR-U6A-1 | High | crates/server/tests/binary.rs:215-221; crates/super/src/main.rs:173, :181, :186, :188 | The double-load bite "fails -16's check" on a log built from the room's header (bite set, tick_hz 30, scene SS, features [bites]); npt-super's real log writes bite None, tick_hz 0, scene "" and features [] (starts-super.jsonl shows it). Put in the room's run, the judge makes the run INVALID (eval.rs:47-61, :542-547); in its own run, no bite is named and the bite can never be judged. The test passes either way. | With netcode's code fix (npt-super takes `--bite`, `--scene`, `--tick-hz` into log()): one npt-super test `h_the_log_header_carries_the_runs_bite_scene_and_tick_rate` (about 20 lines: a fake room `sleep 2.1; echo OPEN; cat > /dev/null`, flags with `--bite PM-0001-16/double-load --scene SS --tick-hz 30`; super.jsonl's header carries those three and max spawn_to_open_ns >= 2,000,000,000); binary.rs's bite drops its hand-built super log and asserts only the room's OPEN delay >= 2 s plus the room header's bite and features ["bites"] (about -10 lines). |
| TR-U6A-2 | Medium | crates/server/src/tick_loop/tests.rs:1006, :1071-1078; crates/server/src/tick_loop.rs:277-291 | -2's everyone_in_view is never compared with the lab's rule (both sides forced to 0). The server needs the slot's own soldier and reads the chosen set; the lab (mirror.rs:324-346) needs only every other soldier in the delivered frame. A dead soldier's client, or a chosen set not delivered, reads differently, and -2's 0.99 share per slot (figures.rs:24-55) moves with no test showing it. | In counted(), compute `everyone` by the lab's rule over `c.relevant()` (every other soldier; ES vehicles) and compare the whole SceneRow, dropping the `everyone: 0` override (about 20-25 lines), over the 90-tick SS run that already has 48 hits and kills. Any difference that appears is a rule for the netcode seat to name. |
| TR-U6A-3 | Medium | crates/server/tests/carry.rs:111-112, :196-212 | C-U6-17's words: "the controller's cap holds under both options". Kind 10 exists under option A too (room input.rs:13, :319); the test runs option B only, by the room's default, so the report would close the carry with half its claim unproven, and a later default change would silently move the test to A. | Loop both legs over `[CmdTiming::A, CmdTiming::B]` with `.with_cmd_timing(t)` on the room (about 4 lines); B is then set explicitly. |
| TR-U6A-4 | Medium | crates/server/src/tick_loop/tests.rs:1166-1203, :814-837; crates/super/src/main.rs:389-395, :452, :456-463, :499 | Flake risks: (a) s5_every_counter_is_on_every_record wants 2 second records and a seen-everyone flag after 1.3 s of wall time: a starved tick thread on a loaded runner gives 1 record; (b) s4_a_pinned_room... the same after 2.3 s; (c) npt-super's two fake-room tests write an executable then exec it while the other may fork (ETXTBSY); (d) h_time_starts... compares `held` against a sleep started before `held`'s clock. | (a)(b) wait on the loop's own progress (until `tl.ticks() >= 40`, at most 4 s) instead of a fixed time (about 4 lines each); (c) one `static` Mutex held across `fake()` and the spawns, or one test for both rooms (about 5 lines); (d) take `held` before spawning the ender (1 line). |
| TR-U6A-5 | Low | crates/server/src/tick_loop.rs:728-731 | Section 3 names "kicked/left/replaced closing"; kicked has no test (the builder's O1), so a change that stops closing a kicked session passes. | A bench test that marks a slot kicked (by the room's overflow path, or a test-only hook as `panic_at` is) and asserts the session's link Closed after its Kick (about 15 lines), or record it in the report's not-verified list. |
| TR-U6A-6 | Low | crates/server/src/main.rs:531-537 | The binary's crash exit (3) and failure exit (1) are never exercised; a change mapping a crash to 0 passes every test. | A pure `fn exit_code(r: io::Result<Ended>) -> u8` used by finish(), and a unit test: Clean 0, Crash 3, Err 1 (about 8 lines). |
| TR-U6A-7 | Low | crates/server/tests/binary.rs:60-66; crates/server/src/tick_loop/tests.rs:1138-1151 | A room that hangs before OPEN hangs the test instead of failing at the 5 s cap; a row offer turned into a blocking send hangs the stuck-writer test instead of failing it. | Read the first line on a thread with `recv_timeout(5 s)`, kill the child on timeout (about 8 lines); the stuck-writer test runs its wakes on a thread joined with a timeout (about 6 lines). |
| TR-U6A-8 | Low | crates/server/src/main.rs:739 (with :381-382) | F4's "burn 0" is written but not asserted: a header that wrote a burn while none runs would pass. | Two lines: `assert_eq!(p["stand_in_burn_ns"], 0)` and `assert_eq!(p["stand_in_top_up_ns"], 0)`. |
| TR-U6A-9 | Low | crates/server/src/tick_loop.rs:983, :1019-1021; crates/server/src/proc_read.rs:151-162, :165-197 | Failed reads read as the best values with no stderr line and no test: /proc/stat unreadable gives steal_permille 0 (and -1's "steal recorded every second" passes); an empty /proc walk gives alone 1; an empty SMT map gives sibling_free 1. | After the netcode seat's ruling on the code: a pure `steal_entry(reader, text) -> (i64, bool)` and a test feeding "" and a text without `cpu<N>` (0 and the failed flag that drives one stderr line, about 12 lines); fixtures `not_alone(&[], ..)` and `sibling_free(&[], ..)` pinning the ruled values (about 4 lines). |
| TR-U6A-10 | Low | crates/bots/src/lib.rs:897; crates/bots/tests/host_step.rs:85-113 | host_step's contract includes `reliable` after the step, but no case has a reliable hook that acts, so dropping or moving the call passes. | One more leg with the forge behaviour, asserting `out.reliable` holds the forged message (about 6 lines). |
| TR-U6A-11 | Low | .gate/u6a/plant-p7b.log (diff 285f015d..., carry.rs:194) | P7b ran at 15:12 on the pre-fmt tree, not the final tree check3 passed (diff 850abb56...); carry.rs has changed since (its failing assert is :211 now). | Re-run P7b once on the final tree at the fix pass (one runner call; restore and hash as before). |
| TR-U6A-12 | Low | brief section 3 "the report gives" | The rows list says "the five per-second reads" plus `out_queue_count`; the code writes four second.v reads plus second.s.out_queue_count, and also npt-super's `event.room_exit.nonzero` (SERIES.md:199), which the list omits. Writer cells of the -2 rows (npt-room) and carry 60's (npt-lab) differ from the writer (npt-server): register 98's known deferral. | The report lists the rows as written: four per-second reads, out_queue_count, the events affinity, cpu_read, tick_panic, room_start, room_exit, and names the writer-cell deferral; no SERIES.md edit. |

Counts: High 1, Medium 3, Low 8.

## What section 3's not-verified list lacks (for the report)
1. The -16 bite on npt-super's real log (TR-U6A-1); the double-load reading in binary.rs is the test's own timer.
2. The shared-vcpu bite's log half is vacuous in this container (alone is 0 without it); only the direct read bites.
3. everyone_in_view's parity with the lab's rule (TR-U6A-2).
4. C-U6-17 under option A (TR-U6A-3); the carry's literal 600 ms-pong leg passes without the cap (F9).
5. The binary's exit 3 and 1 (TR-U6A-6); kicked-session closing (TR-U6A-5, O1).
6. Failed reads (steal, /proc walk, SMT map) and their stderr lines (TR-U6A-9); Q-U6A-5's I/O-set-against-SMT exit 2 is
   not built (netcode FINDING A), so untested.
7. Register 96's room half (F1): the RTT reaches the tick thread, not the room.
8. Machine::read_local parses only this container's 1-thread-per-core SMT map; a 2-thread map from /sys (RS) is not
   exercised, and its parity with lab/src/machine.rs's copy is not tested.
9. The judge never runs on any U6a log: each bite's check is an inline copy (machine_named copied; -16's and -1's
   checks re-typed).
10. The 20-start reading ran the measured debug binary only; no bites-build start went through npt-super.

## What I did not verify
- I ran nothing (no cargo, no plant, no runner call): the counts, plant results and readings are the logs' and the
  builder's; I checked the five planted files' hashes now and read every plant log, check3.log's summary, starts2.log
  and starts-super.jsonl.
- carry.rs's change since P7b (fmt only or more): not compared (no pre-fmt copy kept).
- rebuild.log beyond the builder's note; check3.log's per-test output.
- The netcode seat's code findings (A, B, C and its Lows) as code: only their test halves are here.
- Whether the -2 rule difference (TR-U6A-2) shows on a real SS run's numbers: read, not run.

DONE
