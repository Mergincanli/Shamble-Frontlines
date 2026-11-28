# U6a build notes (builder, worktree agent-a95ebf14368f277ff)

**Slip, recorded first (u4f-brief.md section 4, "An agent that slips says so at the top of its notes"):** my first two
shell calls broke the command-forms rule (P17): one used `git -C` (the guard refused it, nothing ran), the next ran
`cd <worktree> && sha256sum ... && wc ... && pwd && git log ... && git status` as one chained call (read-only; it
printed the brief's sha256, line counts and the pre-merge HEAD f5030a3). Every call after that is one simple command.

**Task:** build U6a (the server core) per u6a-brief.md rev 1.1 (sha256
c109691aba8e9f7edd641efd3b41179bd31c81ce9a3ddc281d6a09baa4fd05ea, checked: matches) in this isolated worktree;
never commit, never push, never touch the main folder. Blocks 1-8 (Block 0 done by the main session).

## Start
- `git merge --ff-only aa5c62b68aecfe2fabd94a74915eb58cf0598fb4`: fast-forwarded (f5030a3..aa5c62b), exit 0.
- `git log -1 --format=%H`: aa5c62b68aecfe2fabd94a74915eb58cf0598fb4 (= `<base>`).
- `git status --short`: clean. `df -h /home/user`: 12G free.

## Blocks (plan)
1. T6 `Send` (crates/link): Loopback Send via Arc<Mutex>; compile-time asserts; 2 tests.
2. S1 TickLoop + PhaseTimers (crates/server, proto `channel_of`): clock.rs, tick_loop.rs, phase.rs; 10 tests.
3. S8a binary (server main.rs): settings JSON, tokens 0600, OPEN, end, exit codes; double-load bite; 5 tests.
4. S4 instrumentation + header (server, log `Machine::read_local`): steal/RSS/schedstat/affinity parsers, events,
   carry 60; no-smt-map + shared-vcpu bites; 8 tests.
5. Room log for -2 (server, room `ClientSlot::relevant`): writer thread, 15 names; 4 tests.
6. -16 start timer (crates/super): `time-starts`, hold form; 2 tests.
7. C-U6-17 (client `plan()`): cap; carry test in npt-server tests; 1 test.
8. C-U6-18 (bots `Behaviour::host_step`); 1 test.
End: check.mjs both builds, plants P1-P11, bites x3.

## Findings (one line each)
- F1 register 96: npt-room's `pump_report(slot, sent, max_datagram)` takes no RTT at <base> (room.rs:891; U4h, which
  would add the room's reader, moved to the real game: checkpoint :23). The room cannot be given `Decoder::rtt_ns`
  without a room API change outside the brief's list. Built: the hand-off to the tick thread per session per tick (kept
  per slot there, tested, P3 plants it); the room's half needs a room input no unit owns now. For the advisor.
- F2 `npt_server::BITES` is pinned at 4 ids by an existing test (server/tests/schedule.rs:350-354); condition 5 keeps
  it unchanged, so U6a's three bite ids live at their npt-server sites, crate-private, not in that list.
- F3 the step's events (shots, hits, kills) are not readable after `Room::tick` without a new room item (SimCore has
  no events getter; `World::dump` allocates a whole state per tick). Built: a crate-private SimCore wrapper in
  npt-server counts them as the room steps (Room is generic over SimCore), so TickLoop builds its room itself.
- F4 S1's CPU stand-in burn (scene.rs:7-8: "the server's tick loop reads these and burns them (U6)") is in no U6a block;
  U6a writes `params.stand_ins` [] and burn/top-up 0 as the lab does (scenario.rs:2037-2039). For the advisor.
- F5 a full state above 262,144 B cannot come from the room (Limits::FULL_STATE 65,536 B), so the 262,144 B test feeds
  a hand-built frame through TickLoop's channel choice (the function P2 plants).
- F6 the link crate's one test that reads the private pipe (`dropping_an_end...`, lib.rs:286-297) must change `borrow()`
  to the mutex's lock: one token, its assertion unchanged (npt-link is not in condition 5's list).

## Block log
- **Block 1 DONE** (T6 `Send`): link/src/lib.rs: Loopback on `Arc<Mutex<Pipe>>` (poison taken as is), const asserts
  (Loopback, `Box<dyn Link + Send>`), module text; tests `t6_a_loopback_end_runs_on_another_thread`,
  `t6_a_loopback_end_and_a_boxed_send_link_are_send` (+2). Log .gate/u6a/b1-test.log: 9 passed, clippy host and
  wasm32 exit 0. `git diff --stat <base>`: 108 added (1 file).
- **Block 2 DONE** (S1 TickLoop, PhaseTimers, `channel_of`): server/src/clock.rs (`now_ns`, the one clock item with
  its reviewed allow; `resolution_ns` for Block 4), phase.rs (`PhaseTimers`), tick_loop.rs (`TickLoop`, `LoopSettings`,
  crate-private `Core`/`SessionIo`/`Tallied`/writer thread), tick_loop/tests.rs (10 S1 tests); proto msg.rs
  `channel_of` + proto/tests/channel.rs (1). Logs .gate/u6a/b2-test6.log (npt-server lib 10 of 10) and b2-test3.log
  (proto 1 of 1, link, existing server tests all pass). Finding F7: the room's `Accept` must follow the tick's
  `JoinAccepted` and full state on the session's queue, or the decoder's first ping reaches the client first
  (seen on the threaded test; fixed by `accept_due`). Finding F8: the room sends no snapshot before the client
  acknowledges its full state, so the outbound-flood half uses a real acknowledging client (stress scene) and a
  4-item session queue (`SessionIo::open`'s `out_cap`; TickLoop passes OUTBOUND_CAP). Kicked-session closing is in
  the code path (`send_outputs`) but not exercised by a test (a room kick needs an event overflow): open item O1.
  Added lines: tracked 142 + new files 1,817 (proto/tests/channel.rs 55, clock.rs 48, phase.rs 97, tick_loop.rs
  934, tick_loop/tests.rs 683) = 1,959.
- **Block 3 DONE** (S8a the binary): server/src/main.rs (settings JSON via `take(CAP+1)`, deny_unknown_fields, range
  checks, tick rate 30 only; tokens from /dev/urandom into a `create_new` 0600 file, `nptj_` + base64url, never
  logged; header with `Machine::read_local` (npt-log, Block 4's item landed here because the header needs it) and the
  binary's SHA-256 by a fixed-buffer loop; TickLoop start; `cpu_read` event; `OPEN`; end by duration, stdin EOF or the
  tick thread ending; exit 0/2/3/1; bites double-load and no-smt-map at their sites); TickLoop gained `finished()` and
  `clock_resolution_ns()` (methods). Tests: main.rs 3 (settings taken, refused, token file), tests/binary.rs 2 (OPEN
  within 5 s + exit 0 + no token in log/stdout/stderr; exit codes) and 2 bites (double-load, no-smt-map). Logs
  .gate/u6a/b3-test1.log (measured 5 of 5), b3-test2.log (bites 7 of 7; spawn to OPEN 789.8 ms, double-load 2,660.4 ms:
  container readings). Open item O2: the `Join`'s protocol version and content hash are binary constants (no plan line
  names the content's files; U6b/U7 take the same). Added lines so far: tracked 824 + new 2,058 = about 2,880.
- **Block 4 DONE** (S4 instrumentation and header): server/src/proc_read.rs (crate-private: `StealReader` (tick
  vCPU's `cpu<N>` line only), `rss_bytes`, `schedstat_ns`, `permille`, `Task::parse`, `not_alone` (ADV-U6A-4's rule),
  `sibling_free`, `tasks`, `thread_tid`, `allowed`, `pin` (/usr/bin/taskset, env cleared, integer args, read back),
  `cpu_ns`); TickLoop: `LoopSettings` gains `tick_vcpu`, `io_vcpus`, `smt`, bites-only `bite`; the starting thread
  pinned to the I/O vCPUs first (every later thread inherits), the tick thread to its vCPU, both read back;
  `affinity` events at start and end; the writer's `Seconds` (steal, rss, proc and io CPU, out_queue_count);
  PM-0001-1/shared-vcpu at its site (a second thread pinned to the tick vCPU). main.rs passes the pins and the real
  SMT map (before the no-smt-map bite clears the header's). Tests: proc_read 4, main.rs 2 (cpu_read, header + carry
  60), npt-log 1 (`read_local`), tick_loop 1 (pinned run) = 8; bite 1 (shared-vcpu). Logs .gate/u6a/b4-test1.log
  (measured all pass; pinned readings: tick vCPU 3, I/O 0-2, alone 0 (container), sibling_free 1, steal 10 and 0),
  b4-test2.log (bites all pass), b4-clippy2.log (clippy host both builds and wasm32: exit 0). Added lines: tracked 909
  + new 2,813 = about 3,722.
- **Block 5 DONE** (the room log for -2): room.rs `ClientSlot::relevant` (pub getter, +8); tick_loop.rs `SceneRow`
  and `SceneTrack` (fixed vectors by entity index, grown only on a new index) counted on the tick thread after the
  tick's end (outside `work_ns`), the 7 `tick.v` names and `tick.s.everyone_in_view` (one flag per roster slot) in
  each tick record, the 6 per-second sums in each second record (`Tallied`'s shots and hits/kills). Tests (tick_loop):
  -2 counts equal the sim's own tick by tick (90 ticks, 2 recorded clients: 88 shots, 48 hits+kills, 2 portal
  changes, 391 spawns), everyone_in_view 1 then 0 (slot 5 moved 75 m), the tick never waits on a stuck writer (40
  rows dropped), every counter on every record = 4. Log .gate/u6a/b5-test1.log (19 of 19). Added: about 4,180.
- **Block 6 DONE** (-16's start timer, condition 3's hold form): super/src/main.rs: `start` (hold until npt-super's
  stdin closes or the room ends; then the room's stdin closed, its exit recorded) and `time-starts --count N`
  (settings `{n}`), the 10 s deadline (`--deadline-ms`) with kill + wait and the elapsed time written, `room_start`
  and `room_exit.nonzero` events in npt-super's npt-log log (process `super`, `Machine::read_local`, own SHA-256 by
  fixed buffer); `Command` path + separate args, stdout read by `take(256)` lines, stderr inherited (never in the
  log); deps unchanged (npt-log, sha2), reaches no sim/room/proto. Tests 3 (fake room by an executable script file:
  starts 207, 203, 203 ms; hold 320 ms; silent room killed at 503 ms, nonzero 1; flags refused). Log
  .gate/u6a/b6-test1.log (3 of 3; clippy exit 0). The container's 20-start reading against the real room: at the end.
- **Block 7 DONE** (C-U6-17): client/src/lib.rs `plan()`: the goal also at most `next_tick_at(now + lo_rtt/2) + 7`
  (lo_rtt the clock's lowest kept round trip; 7 = `Limits::REDUNDANCY_CAP - 1`, option B's buffer of 8), beside
  `max_lead`. Test server/tests/carry.rs (1): a test-local delaying link on Loopback (`Delay`, held messages in arrival
  order), course scene, controller max lead 9, option B, 10 ms path, the 3rd exchange 600 ms slow, run both legs.
  Finding F9: with the pong slow on the way down, the uncapped controller passes too (the slow sample lifts the
  offset estimate by about 15 ms, about half a tick back on the stamps); only the slow-up leg shows the cap's need.
  The test runs both; both 0 refusals with the cap (largest lead 8). Log .gate/u6a/b7-test2.log.
  **P7 done early** (to prove the test): `.gate/u6a/plant-p7b.log`: cap removed (`e == u32::MAX &&` guard) → the
  test FAILS by name `c_u6_17_one_600_ms_pong_on_a_10_ms_path_brings_no_kind_10_refusal_under_option_b` (up leg: 6
  kind-10 refusals at lead 9); restored, sha256 1613a3f0c9b52a74d8d811c701aa62387e23b86ef7d34b93be80f5a2dbd91ec7
  (= clean). (A first plant run, plant-p7.log, passed with the down leg only: the finding above.) Client tests pass
  (b7-test1.log); lab/recording pins checked at the end's test step.
- **Block 8 DONE** (C-U6-18): bots/src/lib.rs `Behaviour::host_step` (input = `step`, the endpoint's `step`,
  `outgoing` then `reliable`) and the module text naming the order as the contract (with the lab's order,
  scenario.rs:1640-1690). Test bots/tests/host_step.rs (1): the never-acker's Input loses its ack only if outgoing
  runs after the endpoint's step; the honest one acks tick 101 group 1. Logs b8-test2.log (pass), b8-clippy.log
  (workspace clippy both builds exit 0; a collapsible_match in SceneTrack fixed).
- Test count so far (by binary): link +2, proto +1, server lib +19 (+1 bites), server bin +5, binary.rs +2 (+2
  bites), carry.rs +1, log +1, super +3, bots +1 = +35 measured, +38 bites.
- **fmt** applied by Edit: 108 rustfmt hunks (fmt2.log) → `cargo fmt --all --check` exit 0 (fmt3.log). Files touched
  only in the lane (git status: 10 modified, 9 new + these notes); no Cargo.toml or Cargo.lock change.
- **End checks next:** check.mjs (-x, with `cargo test --workspace --no-run` both builds first), then plants P1-P6,
  P8-P11, then the 20-start reading.
- check1.log: stopped by me at start (its background limit was 10 min, too short for a ~1,100 s test step; no stray
  process left: pgrep empty). check2.log: the counted run (limit 2 h). Base test step: 1,070.2 s (M5), 1,155.5 and
  1,112.7 s (R6) (u4f2-report.md:83, :149); 1.5 times is about 1,605-1,733 s.
- Clean sha256 before plants: tick_loop.rs 8421e0d3…320a, proc_read.rs c562e475…6ae9, bots lib.rs 5202e036…9334,
  server main.rs 1c44d913…4694, client lib.rs 1613a3f0…1ec7. 20 settings files for the start reading in the
  scratchpad (starts/s1..s20.json).
- check2.log, first lines: fmt, clippy, clippy-wasm32 PASS; doc FAIL: "public documentation for `connect` links to
  private item `SESSIONS_MAX`" (tick_loop.rs, TickLoop::connect's doc). Fix after the run, then a full rerun.
  check2.log test step: PASS 903.6 s, measured 744 (+35), bites 844 (+38), 0 failed, 0 skipped: every existing test
  and pinned recording passes (condition 5 holds); the test step is 0.84 of base's 1,070.2 s.
- check2.log end (15:39:27 UTC, exit 1): fmt, clippy, clippy-wasm32 PASS; doc FAIL; test PASS (above); deps PASS;
  wasm PASS (27,000 ticks, 0 mismatching, both bites caught); refs PASS; marks PASS (81); self PASS (78 of 78); redact
  FAIL: 1 finding, crates/server/src/main.rs:239 "long token" = base64url's 64-letter alphabet literal.
  Provenance: the two `--no-run` builds exit 0 inside the same -x call; "Compiling npt-*" lines from this worktree.
- **Both fixed in the tree, NOT re-run:** main.rs `base64url` computes each letter (no literal); tick_loop.rs
  connect's doc names `SESSIONS_MAX` in plain backticks. precheck3.log (fmt, clippy, doc, redact + bin tests):
  **STOP: under 4 GB free** (runner exit 3; df 3.5 G free). Disk was 12 G at launch, 8.6 G before check2; this
  worktree's two full workspace builds took it. du: shared-target 8.7 G (debug 7.7 G, release 254 M, wasm32 181 M,
  doc 16 M); worktree .gate 44 M; .gate/u6a 504 K. Per the rules: stopped, nothing cleaned or deleted.
- **STOP (disk floor):** not done because every runner call is refused below 4 GiB: the full check.mjs rerun on the
  fixed tree; plants P1-P6 and P8-P11; the 20-start reading. P7 was done (above). The main session's pack or clean
  (its alone), then a fresh builder resumes here: (1) node check.mjs (-x, with the two --no-run builds) on the
  current tree; (2) the plants by the plan below; (3) touch and rebuild; (4) the 20 starts (scratchpad
  starts/s1..s20.json, npt-super time-starts --count 20 --settings .../s{n}.json against shared-target/debug/npt-server).
- Final size (`git diff --numstat <base>` + new files' wc): existing files +1,570 (main.rs 742, super 527, log 111,
  link 106, bots 25, proto msg 24, client 16, server lib 10, room 8, proto lib 1), new files 3,890 (tick_loop.rs
  1,447, tick_loop/tests.rs 1,264, proc_read.rs 383, binary.rs 255, carry.rs 213, host_step.rs 114, phase.rs 102,
  channel.rs 64, clock.rs 48) = **5,460** (stop past 6,000; brief's likely 3,150-3,800).
- Clean sha256 after the two fixes (supersede the earlier ones for these files): tick_loop.rs
  e2908a65ee88a9d6bb35eb8ee6da508f9512d1219c9cf2346abb2e32a3f65c79, server main.rs
  9ac7d64962de10be8dff35a5cdc4c2b97ec5b218e1640d48ea416a385f151e34.
- **RESUMED** (coordinator: part-clean of the dev profile done, 11,333 MiB free; snapshot wip/u6a-build.patch
  a9f8b6e0…1d39, 19 files, 5,460 added). Re-read: df 12 G free; the five plant files' sha256 equal the clean values
  (tick_loop.rs e2908a65…5c79, main.rs 9ac7d649…1e34, proc_read.rs c562e475…6ae9, bots 5202e036…9334, client
  1613a3f0…1ec7). Order: (1) check.mjs (check3.log); (2) plants; (3) touch and rebuild; (4) 20 starts.
- **(1) check3.log DONE: all 11 steps PASS** (exit 0, 19:46:33-20:07:31 UTC; free 11,331 MiB at start; runner diff
  850abb56…63f9, untracked 3c0a31cb…87d6): fmt 4.6 s; clippy 45.4 s; clippy-wasm32 12.3 s; doc 19.3 s; test 866.8 s
  (measured 744 = 709 + 35, bites 844 = 806 + 38, 0 failed, 0 skipped; 0.81 of base's 1,070.2 s); deps 4.6 s; wasm
  66.0 s (27,000 ticks, 0 mismatching; hash-map caught); refs 0.5 s (1,713 files); marks 7.0 s (81); self 2.5 s (78
  of 78); redact 0.9 s (347 items, 0 findings). Provenance: the two `--no-run` builds exit 0; every npt-* crate
  "Compiling" twice from this worktree's path.
- **(2) plants, results** (each one `-x -k` runner call; restore sha256 checked after each):
  P1 plant-p1.log: FAILS `s1_the_bounded_channels_count_a_flood_and_never_block_the_tick` (counted 0, want 5,905);
  restored tick_loop.rs e2908a65…5c79 (clean). P2 plant-p2.log: FAILS
  `s1_take_controls_messages_go_out_on_the_stream_their_framing_names_at_any_length` (Control, want FullState);
  restored e2908a65… P3 plant-p3.log: FAILS `s1_the_tick_thread_takes_each_sessions_decoder_rtt_before_every_tick`
  (None, want Some(42 ms)); first restore Edit matched twice and changed nothing (hash 4ed1a060…, no runner call
  between), second restored e2908a65… P4 plant-p4.log: FAILS
  `s1_a_tick_thread_panic_sends_room_ended_crash_and_writes_the_crash_record_and_footer` (no RoomEnded within 3 s);
  restored e2908a65… P5 plant-p5.log: FAILS `s1_a_stall_runs_at_most_the_catch_up_cap_and_counts_the_dropped_time`
  (30 ticks in the wake, want 7); restored e2908a65… P6 plant-p6.log: FAILS
  `s1_a_replaced_session_is_kicked_and_closed_before_anything_else_is_routed_and_a_left_one_is_closed` ("A's Leave
  was not routed: slot 0 is B's"); restored e2908a65… P8 plant-p8.log: FAILS
  `proc_read::tests::s4_alone_leaves_out_only_the_tick_thread_and_kernel_threads_bound_to_its_vcpu` ("alone");
  restored proc_read.rs c562e475…6ae9 (clean). P9 plant-p9.log: FAILS
  `proc_read::tests::s4_the_steal_reader_reads_the_tick_vcpus_line_only` (27, want 100); restored c562e475…
  P10 plant-p10.log: FAILS `c_u6_18_host_step_runs_input_then_the_endpoints_step_then_outgoing` ((101, 1), want
  (0, 0)); restored bots lib.rs 5202e036…9334 (clean). P11 plant-p11.log: FAILS
  `tests::s4_the_header_names_the_machine_and_carries_the_scenes_weapon_and_health` ("no entry found for key");
  restored main.rs 9ac7d649…1e34 (clean). All five planted files at their clean sha256 after P11 (client lib.rs
  1613a3f0… from P7). (3) next: touch and rebuild.
- **(3) DONE** rebuild.log (20:11:18-20:14:25 UTC): touch + `cargo test -p npt-server -p npt-bots -p npt-client`
  both builds, exit 0 both, 42 test binaries ok, 0 FAILED; npt-server, npt-bots, npt-client "Compiling" twice from
  this worktree. Its tree hashes equal check3's (runner diff 850abb56…63f9, untracked 3c0a31cb…87d6): the final tree
  is the one check3 passed, so no further check.mjs run is needed. Free disk at its start: 4,388 MiB.
- **(4) STOP at the disk floor:** starts.log (20:15:07 UTC): "STOP: under 4 GB free" (3,859 MiB), runner exit 3,
  nothing ran. The 20-start reading is not taken. Two debug builds (measured and bites, check3 then the rebuild)
  took the 11.3 G back down. To resume: part-clean, then the starts.log command again (it builds the measured
  npt-server binary first, since the rebuild's last build left the bites build at shared-target/debug/npt-server).
- **(4) DONE after part-clean 2** (starts2.log, 20:17:15-20:17:52 UTC, free 11,319 MiB at start; tree hashes
  unchanged: diff 850abb56…, untracked 3c0a31cb…): `cargo build -p npt-server --bin npt-server` (no features: the
  measured binary; npt-server compiled from this worktree) exit 0, then `npt-super time-starts --count 20` exit 0.
  Its log .gate/u6a/starts-super.jsonl (header, 20 room_start, 20 room_exit nonzero 0, footer). Spawn to OPEN (debug
  build, container; reported, never judged): p50 633,982,794 ns (nearest rank, 10th of 20; the two middle values
  633.98 and 634.57 ms), max 978,569,265 ns (the 2nd start), min 594,810,726 ns. Free after: 10,646 MiB.
- Final tree: `git diff HEAD --shortstat` 10 files, +1,570 -27 (all under spikes/netcode-proof; runner diff hash
  850abb56…63f9); 9 new files (runner untracked hash 3c0a31cb…87d6) + these notes; added 5,460 in all.
- Plant plan (each: Edit in place, one `-x -k` runner call on the named test, Edit back, sha256 = clean):
  P1 tick_loop.rs `ToTick` on std `channel()`/`send` (unbounded) → s1_the_bounded_channels_count_a_flood...;
  P2 `reliable_channel` = lab's `stream_of` copy → s1_take_controls_messages_go_out_on_the_stream...;
  P3 Report `rtt_ns: None` → s1_the_tick_thread_takes_each_sessions_decoder_rtt...;
  P4 the catch's Err arm `resume_unwind` (as no catch) → s1_a_tick_thread_panic_sends_room_ended_crash...;
  P5 `Schedule::new(TICK_HZ, u32::MAX)` → s1_a_stall_runs_at_most_the_catch_up_cap...;
  P6 the old session not closed at a replacement (still routed) → s1_a_replaced_session_is_kicked...;
  P8 proc_read.rs bound-kthread exclusion removed → s4_alone_leaves_out_only_the_tick_thread...;
  P9 StealReader reads the `cpu` line → s4_the_steal_reader_reads_the_tick_vcpus_line_only;
  P10 bots host_step: outgoing on an empty list before the endpoint's step → c_u6_18_host_step_runs_input...;
  P11 server main.rs: carry 60's seven params dropped → tests::s4_the_header_names_the_machine_and_carries...

## Fix pass (2026-11-23; list u6a-fix-pass.md FP-1 to FP-12; reviews u6a-review-netcode.md, u6a-review-test.md)
- Read whole: the fix list (82 lines) and both reviews (173, 203). Rules as for the build; one FP-n line per item.
- Start: the runner's start line must show diff 850abb56…63f9 and untracked 3c0a31cb…87d6 before the first edit
  (fp-start.log).
- Start DONE: fp-start.log (02:46:59 UTC, `git status --short`) reads diff 850abb56…63f9 and untracked 3c0a31cb…87d6
  (both match), free 10,631 MiB; no edit before it.
- **FP-1 DONE** (fp1.log 02:50-02:51 UTC, both exit 0): npt-super takes `--scene`, `--tick-hz` (1 to 1,000) and an
  optional `--bite` (super main.rs:97-108, Flags :40-53, required checks :135-137); `log()` writes bite, tick_hz and
  scene from them (:201-208; features stay [] as npt-super's own build has none). New test
  `tests::h_the_log_header_carries_the_runs_bite_scene_and_tick_rate` (:549; fake room `sleep 2.1; echo OPEN; cat`,
  `--bite PM-0001-16/double-load`, `--scene SS --tick-hz 30` from the helper `flags`, which takes an `extra` slice
  now): header (Some(bite), "SS", 30) and max spawn_to_open_ns >= 2e9; ok. The refusal test also refuses
  `--tick-hz 0`. binary.rs:207 `bites::pm_0001_16_double_load_fails_every_start_under_2_s` drops the hand-built super
  log (and its EventRecord/Phase/Writer import) and asserts the room's OPEN >= 2 s, the room header's bite and
  features ["bites"]; ok. No judge change.
- **FP-2 DONE** (fp2.log 02:53 UTC, both exit 0): the smaller of the two routes is a pub associated fn beside
  `TickLoop::clock_resolution_ns` (no lib.rs change): `TickLoop::io_on_a_tick_sibling(smt, tick_vcpu, io_vcpus)`
  (tick_loop.rs:1389, `!proc_read::sibling_free` over the I/O set as one allowed list). main.rs:431
  `sibling_refusal(&Settings, smt) -> Option<String>`; `run()` (:439-443) reads the machine first and returns
  `fail(EXIT_SETTINGS, ..)` (one stderr line) before minting, so a refusal leaves no token file or log, as bad settings
  do (my choice of place: the list said "after the SMT map is read"; it is still after it). Module text's exit-2
  line names it. Test `tests::s4_an_io_vcpu_on_the_tick_vcpus_smt_sibling_is_refused` (main.rs:761; fixed map
  [[0,4],[1,5],[2,6],[3,7]], tick 3: io [1,7] refused with the line, [1,2] and no pinning not); ok.
- **FP-3 DONE** (fp3.log 02:54 UTC, exit 0): proc_read.rs:166 `alone(tasks, tick_tid, vcpu) -> Result<bool, &str>`
  (Err unless the walk holds the tick thread and tid 1 and is below TASKS_MAX; else `not_alone` empty) and :175
  `sibling_free_read(smt, vcpu, &[Option<Vec<u32>>])` (Err for an empty SMT map or any I/O list not read; else
  `sibling_free`). `affinity()` (tick_loop.rs:1274-1294) maps each Err to 0 with one stderr line
  ("npt-server: affinity at <when>: <key> written as 0: <why>"); the I/O lists are `map(allowed)`, no longer
  `filter_map`. Fixtures `proc_read::tests::s4_a_partial_walk_or_a_failed_read_is_never_alone_or_sibling_free`
  (:414: whole walk Ok(true); empty walk, no tid 1, no tick thread Err; empty SMT map and a None list Err); ok. The
  pinned room test still passes and printed no such stderr line here (the container's walk holds tid 1; alone 0 is a
  real reading, sibling_free 1).
- **FP-4 DONE** (fp4.log 02:57 UTC, exit 0): `Seconds` gains `failed: [bool; 3]` (tick_loop.rs:958; reads named in
  `SECOND_READS` :962: steal, rss, the two CPU shares) and `cpu: Option<(u64, u64)>`; `baseline(stat)` takes the
  /proc/stat text (callers read it by `proc_text` :969); `values()` (:1006) puts each read's keys in or leaves them
  out, with one stderr line at that read's first failure; `write()` (:1065-1074) hands it steal, `rss_bytes(..)` and
  the CPU delta (None when either end's read failed). proc_read.rs:286 `cpu_ns` returns `Option` (None when
  /proc/self/task or no thread's schedstat reads; a thread that ended mid-read is left out, no longer a 0). Test
  `tick_loop::tests::s4_a_failed_per_second_read_leaves_its_key_out` (tests.rs:866: steal fed "" and a text with no
  cpu3 line, rss None: no steal or rss key, flags [true, true, false], CPU shares 500 and 100; then a good text: steal
  100, rss 4,096; CPU None: no CPU keys, flag set); ok, with the three stderr lines printed once each. The pinned room
  and s5_every_counter tests still pass. Note: a steal read with no jiffy between two reads (StealReader's "no time
  passed") is a gap now too (was 0); only a stop within 10 ms of a second's write can meet it.
- **FP-5 DONE** (fp5-6.log 02:59 UTC, exit 0): a failed offer of `Kick(replaced)` (tick_loop.rs:662-669) and of
  `RoomEnded` (:754-761) adds 1 to `shared.out_full`; the session is closed either way, as before. No new test (the
  list's 2 lines); the replaced-session and panic tests still pass.
- **FP-6 DONE, no tick differs** (fp5-6.log, exit 0): `counted()` (tick_loop/tests.rs:1015-1036) computes `everyone`
  by the lab's rule (mirror.rs:324-346: every other soldier, and ES's vehicles, in the set; no own-soldier condition)
  over `c.relevant()`, for each joined, unkicked slot below LOGGED_SLOTS; the `everyone: 0` override is gone and
  `s5_the_room_logs_minus_2_counts_equal_the_sims_own_tick_by_tick` (:1104) compares the whole SceneRow every tick
  of the 90-tick SS run (shots 88, reliable 48, portal 2, spawns 391, stand-in spawns 390): all 90 equal, and a bit
  was set on all 90 ticks (new assert `seen > 0`). Why they agree here: `relevant()` is empty for a client with no
  soldier (room.rs:361-363), so the server's `own &&` and the lab's rule over this set give the same bit whenever
  another soldier exists (they would differ only for a slot with no soldier in a room with no other soldier). Neither
  rule changed.
- **FP-7 DONE** (fp7.log 02:59, fp7b.log 03:00 UTC, exit 0): carry.rs `run(timing, up)` (:98) sets
  `.with_cmd_timing(timing)` on the room (:111-113, B now explicit); the test
  `c_u6_17_one_600_ms_pong_on_a_10_ms_path_brings_no_kind_10_refusal_under_option_b` (:198-224) runs four legs
  (A and B, each down and up), all before the first assert, and each refusal assert prints every leg's refusals (so a
  plant names them all). Readings: A down/up 146 given, 145 run, 0 refused, lead 8; B down/up 146, 144, 0, lead 8.
- **FP-8 DONE** (fp8.log 03:01-03:02 UTC, both exit 0): (a) `s5_every_counter_is_on_every_record`
  (tick_loop/tests.rs:1207-1218) steps its client until `tl.ticks() >= 40` and the client has a snapshot, at most 4 s,
  then asserts both; (b) `s4_a_pinned_room_...` (:813-819) waits until `tl.ticks() >= 40`, at most 4 s, and asserts
  it (no sleep 2.3 s). 40 ticks span 1.3 s of the loop's clock, so the run crosses a second's edge wherever the
  process clock's origin lies. (c) npt-super's three fake-room tests (FP-1's added) each hold one `static FAKE`
  Mutex (`one_fake()`, super main.rs:409-413) across `fake()` and the spawns (:484, :533, :561); (d) `held` is taken
  before the ender thread spawns (:492). npt-super's 4 tests ok in 4.37 s; both lib tests ok.
- **FP-9 DONE** (fp9-11.log 03:03 UTC, all four commands exit 0): main.rs:545 `exit_code(&io::Result<Ended>) -> u8`
  (Clean 0, any other end EXIT_CRASH 3, Err EXIT_FAILED 1); `finish(tl)` (:554) uses it, its `failed` argument gone
  (both callers passed EXIT_FAILED). Test `tests::s8a_the_exit_code_is_0_clean_3_after_a_crash_and_1_for_a_log_not_finished`
  (:715); ok.
- **FP-10 DONE** (fp9-11.log): binary.rs `start` (:60-81) reads the first line on a thread, `recv_timeout(5 s)`, and
  on timeout kills and waits the child, then panics naming the cap; both binary test files pass (measured 2, bites 4,
  double-load included). `tick_loop::tests::s5_the_tick_never_waits_on_a_stuck_writer` (tests.rs:1181) runs its
  1,064 wakes on a thread, awaited by `recv_timeout(5 s)` then joined; ok.
- **FP-11 DONE** (fp9-11.log): the header test asserts `stand_in_burn_ns` 0 and `stand_in_top_up_ns` 0
  (main.rs:772-774); ok.
- **FP-12 DONE** (fp12.log 03:04 UTC, exit 0): `c_u6_18_host_step_runs_input_then_the_endpoints_step_then_outgoing`
  (bots/tests/host_step.rs:114-133) gains a leg: `Behaviour::brawl(..).forger(11, 2, 12)`, two host steps; the second
  (the forged command) has `out.reliable`'s last entry on Control, decoding (after its 2-byte frame) as a `Ping`;
  ok. Not planted (not in the list's plants).
- Size after FP-1..12, before fmt (fp-size.log 03:04 UTC; same method as the build's): existing files +1,696 (main.rs
  797, super 598, log 111, link 106, bots 25, proto msg 24, client 16, server lib 10, room 8, proto lib 1), new files
  4,085 (tick_loop.rs 1,506, tests.rs 1,317, proc_read.rs 431, binary.rs 259, carry.rs 225, host_step.rs 133, phase.rs
  102, channel.rs 64, clock.rs 48) = 5,781 (+321; under 6,000). Free 9,983 MiB. Next: fmt check, apply by Edit.
- fmt: fp-fmt.log (`cargo fmt --all -- --check`, never writing) showed 13 hunks (host_step.rs 1, proc_read.rs 2,
  tick_loop/tests.rs 3, tick_loop.rs 1, main.rs 2, carry.rs 1, super main.rs 3); all applied by Edit. Clippy (both
  builds, check.mjs's scope `--workspace --all-targets`, `-D warnings`): fp-lint.log and fp-lint2.log each failed on
  one `type_complexity` (tick_loop.rs `values()`'s annotated array, then carry.rs's `legs` Vec): both annotations
  dropped (`Vec<_>`, inferred array). fp-lint3.log (03:07 UTC): fmt check exit 0, clippy measured 0, bites 0.
- Fix-pass final tree (fp-clean.log 03:07 UTC): runner diff 0fc8865b…b60d, untracked 421bff63…92d7; clean sha256 for
  the plants: client lib.rs 1613a3f0…1ec7 (unchanged), proc_read.rs b231a3f6e04d2f7c50a72daf2ad97e583c23f8d7544777f19c06a5698ef51464,
  super main.rs 0e2a426952fbf087df4fee0013d74f01083ed13a12f3ba598750f08f8e63c983, server main.rs
  ef115163047e6a98692dfb97e72863c03c187ebad6f907f16a308fed54cb6bd7, tick_loop.rs
  7dd0a9bf7135ff5623011d2d6c01772767ab28ce3d48fa0a29081a31c3d13552. Free 9,001 MiB.
- Plants (P7 and P7b: one plant, the cap removed in client lib.rs; P7 ran against the down-only test in the build
  and passed, which was F9; the final test runs every leg, so P7 here is the cap removed with the whole test, and
  P7b the same plant with the run's output shown per leg).
  - **P7/P7b** fp-plant-p7.log (03:07 UTC, -x -k, exit 101): client lib.rs `Some(e) if e == u32::MAX && ..` (cap
    off) → FAILS `c_u6_17_one_600_ms_pong_on_a_10_ms_path_brings_no_kind_10_refusal_under_option_b` at carry.rs:224,
    "B, its ping slow (up)": 6 kind-10 refusals at lead 9; the other three legs 0 (A down, A up, B down; lead 9).
    Finding for round 2: under option A the carry's refusal count (`buffer_full`) stays 0 even without the cap, so the
    A legs pass either way (TR-U6A-3's A coverage is the run under A, not a bite). Restored: 1613a3f0…1ec7 (clean,
    fp-plant-p8.log line 5).
  - **P8** fp-plant-p8.log (03:07-03:08, exit 101): proc_read.rs `not_alone`'s bound-kthread filter removed → FAILS
    `proc_read::tests::s4_alone_leaves_out_only_the_tick_thread_and_kernel_threads_bound_to_its_vcpu` (:390 "alone").
  - **P9** fp-plant-p9.log (03:08, exit 101): StealReader's line name `"cpu"` (the all-CPU line) → FAILS
    `proc_read::tests::s4_the_steal_reader_reads_the_tick_vcpus_line_only` (Some(27), want Some(100)) and
    `tick_loop::tests::s4_a_failed_per_second_read_leaves_its_key_out` (tests.rs:889, no steal key on the good read).
    proc_read.rs restored: b231a3f6…1464 (clean, fp-plant-p12.log line 5).
  - **P12** fp-plant-p12.log (03:08, exit 101): super main.rs `log()` writes bite None, tick_hz 0, scene "" → FAILS
    `tests::h_the_log_header_carries_the_runs_bite_scene_and_tick_rate` (:573, left (None, "", 0), right
    (Some("PM-0001-16/double-load"), "SS", 30)). Restored: 0e2a4269…c983 (clean, fp-plant-p13.log line 5).
  - **P13** fp-plant-p13.log (03:08, exit 101): main.rs `sibling_refusal`'s check made `false && ..` → FAILS
    `tests::s4_an_io_vcpu_on_the_tick_vcpus_smt_sibling_is_refused` (:797 "7 is 3's sibling"). Restored:
    ef115163…6bd7 (clean, fp-plant-p14.log line 5).
  - **P14** fp-plant-p14.log (03:08, exit 101): proc_read.rs `alone()` returns Ok(true) on an empty walk → FAILS
    `proc_read::tests::s4_a_partial_walk_or_a_failed_read_is_never_alone_or_sibling_free` (:434 "an empty walk").
    Restored: b231a3f6…1464 (clean).
  - fp-restored.log (03:09 UTC): all five planted files at their clean sha256; runner diff 0fc8865b…b60d and
    untracked 421bff63…92d7, the fix-pass final tree. Next: check4 (check.mjs, -x, through the runner).
- check4 STARTED ~03:09 UTC in the background (limit 2 h): `-x` runner call, `cargo test --locked --workspace
  --no-run`, the same `--all-features`, then `node check.mjs` (check3's form), log .gate/u6a/check4.log. If cut:
  rerun that exact call (the tree must still read diff 0fc8865b…b60d, untracked 421bff63…92d7).
- **check4.log DONE: all 11 steps PASS** (exit 0, 03:09:26-03:25:34 UTC; free 9,011 MiB at start; runner diff
  0fc8865b…b60d, untracked 421bff63…92d7; both `--no-run` builds exit 0 first): fmt 0.7 s; clippy 4.4 s;
  clippy-wasm32 11.9 s; doc 18.1 s; test 793.8 s (measured 749 = 709 + 40, bites 849 = 806 + 43; was +35/+38: the
  five new tests in each build; 0 failed, 0 skipped; 0.74 of base's 1,070.2 s); deps 1.5 s; wasm 62.8 s (27,000
  ticks, 0 mismatching; hash-map and std-sin caught); refs 0.1 s (1,713 files); marks 3.0 s (81); self 1.8 s (78 of
  78); redact 0.8 s (347 items, 0 findings).
- **STOP at the disk floor after check4:** fp-size2.log (03:25:42 UTC): "STOP: under 4 GB free" (3,791 MiB), runner
  exit 3, nothing ran. Nothing cleaned. The final size is therefore derived, not measured: fp-size.log's 5,781
  (03:04, after FP-12) plus the 13 fmt hunks' net lines (fp-fmt.log: host_step +4, proc_read +17, tests.rs +13,
  tick_loop.rs +5, main.rs +9, carry.rs +4, super +10 = +62; the two clippy fixes change no line count) = **5,843**
  added in all (existing +1,715, new 4,128); under 6,000. A measured count needs one runner call after a part-clean.
- **FP-13 DONE** (round 2: TR2-U6A-1, NR2-U6A-3). Start fp13-start.log (03:36 UTC, free 11,210 MiB): diff
  0fc8865b…b60d, untracked 421bff63…92d7 (= check4's tree). Change: proc_read.rs:70 `StealReader::has_line`;
  tick_loop.rs:1006 `baseline()` returns `Result<Option<u32>, ()>` (Err only for a text without the tick vCPU's line;
  Ok(None) for no share: unpinned, first read, or no time); `values()` (:1028) asks for steal only when not Ok(None),
  so no time leaves the key out with no flag and no stderr line; tick() :1062 `let _ =`. Pinned test (tests.rs:843-854)
  exempts the last record's (the close's partial second) steal only. New test
  `tick_loop::tests::s4_a_steal_read_with_no_time_passed_leaves_its_key_out_with_no_failure` (:906: two equal texts,
  no key, flags [false; 3]); FP-4's test now asserts its baseline read is Ok(None). fp13-1.log: fmt 1 hunk (applied by
  Edit), clippy -p npt-server --all-targets --locked both builds 0, the 8 named tests ok (pinned included).
  fp13-2.log: fmt check 0, clippy both builds 0; clean sha256 proc_read.rs 0d0e3eda…d387, tick_loop.rs fea75451…d88a,
  tests.rs 8069aa1d…17f7. P9 (fp13-plant-p9.log, -x -k, exit 101): `"cpu"` line → FAILS
  `proc_read::tests::s4_the_steal_reader_reads_the_tick_vcpus_line_only` (Some(27) vs Some(100)),
  `s4_a_failed_per_second_read_leaves_its_key_out` (:877 Err(()) vs Ok(None)) and the new test (:909); restored,
  0d0e3eda…d387 (fp13-end.log). Final tree: diff 0fc8865b…b60d, untracked d01ae55c…669b. Size measured
  (fp13-end.log): existing +1,715, new 4,160 = **5,875** added in all (FP-13 +32; confirms the derived 5,843 before
  it). Full check.mjs not rerun after FP-13 (not asked).
