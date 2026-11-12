# U4e build notes: the room, the server I/O step and the client (u4e-brief.md rev 3.3; advisor-u4e-brief.md APPROVE, L1-L4)

Builder: general-purpose, opus, effort xhigh, isolated worktree (workflow agent).

## Start (1.1, as replaced by the launch task)
- Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_0fa6595a-3e2-1
- The worktree came up at bb2d817 (main's head, no spikes/), not at 97bb9d3 as the task said. `git merge --ff-only
  97bb9d3` (HEAD was an ancestor) brought it to the branch head.
- HEAD: 97bb9d32aa7bd96d45ee0ac5c48a35e9dfeb0864 (the branch head).
- `<base>`: 28a85e773e665f9944c3ab8bd606337298bd3273 (the carry-51 commit).
- `git diff --stat 28a85e7 HEAD -- spikes/`: empty (checked after the fast-forward). Base counts (7.2 item 1): check.mjs
  10 steps, measured 399, bites 446, self 52 of 52.
- Copy 2 (after the cut): read these notes, `git status --short` (only this file untracked), `git diff --stat` empty,
  HEAD 97bb9d3. No code from copy 1. Continue at the plan. `pub fn effect_rows` is in scene.rs:529 at HEAD (2.7: holds).
  df -h /home/user: 8.9 G free (copy 2's first reading).

## Design decisions (read from the brief, the plan and the code; each a choice where the plan is silent)
- Room's static graph route (W1-PUB): the caller passes `&Greybox` (the tests: `World::greybox()`); the room reads
  `areas()` and `portals()` from it.
- Room's outputs: per slot, datagrams (encoded snapshots) and control messages as npt-proto `Message`s (npt-room has
  no npt-link: it cannot name `Channel`; the glue sends each on its type's row channel, `Limits::TYPES`).
- The pump's per-client report to the room before each tick: bytes sent since the last tick (payload + 28 each), the
  link's max datagram size (the group limit is min(Limits::SNAPSHOT, it): e2's forced split is a small Loopback), RTT.
- FullState at join and at each resync; deltas (snapshots) resume once it is acknowledged (catalogue's FullState row).
  The FullState's tick is a sent tick of one group in the AckRing; both sides file a hash for it (client_hash over its
  own and entity records, no events).
- Window: per-tick budget floor((k+1)*32000/hz) - floor(k*32000/hz) (the fractional remainder carries; unused bytes do
  not), less the pump's bytes.
- Event numbers: one room-wide 32-bit counter, injectable start; the client's event rebuild starts near its own
  injectable start value (the run's setting both ends share).
- Option B: the command stamped T runs at T; on a miss the last command run repeats; commands stamped before T are
  dropped and counted late; the buffer holds at most CMD_CATCHUP commands (a bound; U4f's catch-up check is separate).

## Plan (blocks)
- B1 npt-room scaffolding (lib.rs: deny float arithmetic, modules, BITES), the two clippy.toml bans, Roster, ClientSlot,
  the token compare; unit tests.
- B2 AckRing and the baseline rule; e4 unit tests.
- B3 EventQueue: cap as data (1,024 provisional), overflow resync, age 255, numbering; unit tests (carries 23, 50).
- B4 InterestGrid, AreaFlood, Priority; unit tests.
- B5 InputBuffer (option B, CMD_CATCHUP + const assert, buffer_depth_ticks), the three -58 compares, rewind_at; unit
  tests (carries 22, 27; e9's room negatives).
- B6 Encoder (window, groups, own-state filter, mirror hash, encoder errors) and Room<S>; room tests (carries 21, 53,
  54, 57; e1; e10's window; e8's Room).
- B7 npt-server: Decoder::pump, ConnLimits, Counters, one path in lib.rs; tests (e9's server negatives, e10's bucket and
  Pong limit).
- B8 npt-client: ClientEndpoint, StepOut, frames, hashes, acks, rebuilds, BITES; tests (e3, carry 61, wrong channel,
  lt-tick-offset, lt-event-seq, client-rounding).
- B9 Loopback tests in server/tests: e2, e5a, e5b, 2.5, e6, e7, e11, tick16, server-echo, -22, carry 56's measurement.
- B10 bites: each part (i) twin; part (ii) scratch switches, one at a time, restored and sha256-checked.
- B11 end checks: check.mjs twice as counted runs, provenance, clippy, fmt; the return list.

## Log
- B1-B5 code written (room/src: lib.rs, roster.rs, ack.rs, events.rs, interest.rs, input.rs, rewind.rs, encoder.rs,
  room.rs; both clippy.toml bans). First build (.gate/u4e/b1-room.log): exit 0, 18 unit tests passed, 3 dead-code
  warnings (fixed: Sent::full_state and AckRing::last_acked removed, Priority::retain used). Next: npt-client, then
  the pump, then the tests.
- B7/B8 code written: server/src/pump.rs (Decoder, ConnLimits, Counters), server/src/lib.rs (modules private, one
  path at the root), client/src/lib.rs (ClientEndpoint, StepOut, BITES of 3), client/tests/bites.rs pin. Built clean
  (.gate/u4e/b2-three.log).
- e7 first run (.gate/u4e/b3-e7.log): 48,432 pairs compared, 0 differ, 70 s. Finding: the endpoint dropped every
  `Input` whose page command had a view time after its stamp (encode OutOfRange), so acks stalled for 3 clients:
  fixed in the endpoint (a view time the wire cannot carry is taken as the command's own tick) and in the lab's script
  (keyed by the command's own tick). Finding: on zero-delay Loopback no event is resent and no tick splits at the
  budget (A2's case, and the same arithmetic for resends): e7 prints both, the gap test asserts resends, e2 the split.
- Room tests (room/tests/room.rs, bites.rs, catalogue.rs): 21 pass in the bites build, 14 in the measured build
  (.gate/u4e/b4-room3.log). Carry 53: SS seed 1, 300 ticks from tick 1,050 (the world warmed from tick 200 through
  850 ticks of its recording: a soldier first dies after about 1,000 hits, tick 1,287 for seed 1), 48 clients, 2
  generation changes. Carry 56: SS 18.94 B mean, 30 B p99, 121 B max own-state section per snapshot (14,352
  snapshots), OwnState 9.03 B mean, changed in 0.9961 of snapshots; ES 15.55 / 21 / 25 B, OwnState 8.27 B, 1.0000.
  Finding: the lt-ack bite needs a lagging ack to bite (the wrap case's client acks two ticks late).
- B9 tests written: client/tests/endpoint.rs, server/tests/{common/mod.rs, pump.rs, gap.rs, loopback.rs}, room.rs's
  in-crate tests (carries 21, 23). Bites build, all but e7 (.gate/u4e/b5-all3.log): every binary ok. Findings fixed:
  the window moved only on ticks with a snapshot and carried the pump's bytes into the next one (e6 caught it: now it
  moves every tick and the bytes count against their own tick); e2's 4-client SS run never split at 300 B (147 B a
  snapshot): 48 clients, 3,024 split ticks, 3,023 compared equal (one is the dropped group's tick).
- e5a (clean, gap = ACK_RING_TICKS periods): AfterSend T 330, gap 2,000,000,001..3,066,666,667 ns, F 363, B 329 at
  age 34, evicted; BeforeSend gap 1,999,999,999..3,066,666,665, F 362, B 329 age 33, evicted. 1 s gap (reported):
  AfterSend F 361, B 330 age 31, not evicted; BeforeSend F 360, B 329 age 31, not evicted.
- 2.5 clean: AfterSend first fresh delta tick 363 at 3,100,000,000 ns (33 ms after the gap's end); BeforeSend tick
  362 at 3,066,666,666 ns (0 ms after); 236 pairs, 0 differ; 462 / 439 events resent. Bitten (no-fallback): F coded at
  age 34 / 33 against B 329, no fresh delta, 29 / 30 sentinels from F (the client refuses: B is outside its ring by
  age, so not a baseline it holds).
- tick16 bitten (both ids): server hash at 65,537 present, client none (14 post-wrap groups dropped as stale).
- B9 done. Clippy host both builds, wasm32 both builds, rustdoc both builds host and wasm32 on the three crates: clean
  (.gate/u4e/b6-*.log). rustfmt's diffs applied by Edit, file by file (about 130 hunks). Added: the room's Join
  replacement test (plan silent), the decoded-value share of client-rounding (reported).
- Pre-check (not counted; .gate/u4e/b7-precheck.log, -x): every step PASS but fmt (one new line, fixed by Edit):
  measured 458, bites 524, self 52 of 52; test step 459.6 s. df: 7.9 G free. Next: B10 (part (ii) switches), then
  B11 (two counted runs).
- B10 DONE. Part (ii): 15 scratch switches, one at a time, each `-x`, run by name in the bites build (`--all-features
  -p <crate> --test <file> <name>`), each FAILED by name, each file restored by Edit and its sha256 equal to the clean
  file's. Logs .gate/u4e/s01..s15:
  S1 -18/no-fallback on e5b: `e5b_the_first_snapshot_after_the_gap_is_coded_against_the_defaults ... FAILED`
  ("AfterSend: F 363 coded at age Some(34)"). S2 -18 on 2.5: `pm_0001_18_a_fresh_delta_decodes_within_500_ms_of_the_
  gaps_end ... FAILED` ("no fresh delta within 500 ms of 3066666667 ns: None"). S3 -17/tick16, S4 -58/tick16:
  `pm_0001_17_and_58_tick16_the_client_files_tick_65_537_under_itself ... FAILED` (Some(2768209054929083115) None).
  S5 -21/no-trailing, S6 -21/no-channel: `pm_0001_21_the_pump_counts_trailing_bytes_and_the_wrong_channel ... FAILED`
  (left (0,1,1) / (1,0,1)). S7 -17/client-rounding: `tests::the_endpoints_grid_integers_equal_the_servers_over_a_sweep_
  with_ties_and_negatives ... FAILED` (left 7920). S8 lt-tick-offset: `pm_0001_58_the_last_run_rebuild_names_the_right_
  command_across_the_wrap ... FAILED` (left Some(131071)). S9 lt-event-seq: `pm_0001_58_event_numbers_are_rebuilt_
  across_the_16_bit_wrap ... FAILED`. S10 lt-ack, S11 lt-cmd-seq, S12 lt-press: `pm_0001_58_the_rooms_ack_sequence_and_
  press_compares_accept_across_their_wraps ... FAILED` (left (1,0,0) / (0,24,0) / (0,0,34)). S13 server-echo:
  `pm_0001_62_the_room_never_sends_a_shooter_its_own_cosmetic_shot ... FAILED` ("232 own shots sent"). S14 unbounded:
  `pm_0001_22_the_never_ackers_headroom_never_goes_below_0 ... FAILED` (left -234 right 0). S15 owner-to-all:
  `carries_51_53_and_56_ss_each_client_gets_exactly_its_own_state_rows ... FAILED` ("tick 1050: client 0's full state
  holds Soldier 1 gen 0"). Clean shas: room/tests/room.rs 3f15161b...1499, server/tests/gap.rs b5e6b92d...655b,
  loopback.rs 9c936263...04a4, pump.rs 7d8fa132...9513, client/src/lib.rs bd83870c...d9cfd9, client/tests/endpoint.rs
  7e50dce3...d506 (all six re-checked after S15). Next: B11 (`--list`, then two counted runs).
- B11 DONE. `--list` (-x, .gate/u4e/b11-list.log): exit 0; npt-room lib 20, room/tests bites 1, catalogue 1, room 22;
  npt-client lib 2, bites 1 (existing), endpoint 9; npt-server gap 6, loopback 6, pump 11 (schedule 17, U4b's). New in
  the bites build: 78 (= 524 - 446); in the measured build 59 (= 458 - 399; 20 of the new are bites-only).
  Counted run 1 (-x, .gate/u4e/b11-counted1.log, 05:20:20-05:28:35 UTC, df 7.8 G free): both `--no-run` builds and
  check.mjs exit 0; fmt, clippy, clippy-wasm32, doc, test (measured 458 passed 0 failed, bites 524 passed 0 failed),
  deps, refs, marks, self 52 of 52, redact all PASS; "check: all 10 steps pass". Provenance: "Compiling npt-room /
  npt-server / npt-client v0.0.0 (<this worktree>/spikes/netcode-proof/crates/...)" in both cargo builds.
  Counted run 2 (-x, .gate/u4e/b11-counted2.log, from 05:28:46 UTC, df 8.0 G free): the same, every step PASS, 458 /
  524, self 52 of 52, "check: all 10 steps pass"; the same provenance lines (each crate twice).
  e7 printed (shared, .gate/u4e/b11-e7print.log): 48,432 pairs, 0 differ; per client at least 1,009; split ticks 0;
  skipped 17,184; events carried 562,794; resent 0; non-default baselines 48,384 (every snapshot); 69.2 s.
