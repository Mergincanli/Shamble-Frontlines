# U4f build notes: clock and commands (builder, from u4f-brief.md rev 3.3, filled at launch)

## Block 0: start (done)

- Launch head given by the prompt: 827518bde465cf208e461da5fb605993c2bb3220 (<base> 5b29296b plus one .checkpoints/-only
  commit). Where the prompt differs from the brief (section 1 item 1 names <base>), the prompt wins: I used the launch head.
- `git merge --ff-only 827518bde465cf208e461da5fb605993c2bb3220`: exit 128 ("Diverging branches can't be fast-forwarded":
  the worktree was made at origin/main). Then `git switch -c u4f-build 827518bde465cf208e461da5fb605993c2bb3220`:
  "Switched to a new branch 'u4f-build'".
- `git log -1 --format=%H`: 827518bde465cf208e461da5fb605993c2bb3220 (the launch head).
- `git diff --quiet 5b29296b03f4e116c0afac8b89d9bc15cc1adaf7 HEAD -- spikes/`: exit 0.
- `git merge-base --is-ancestor 0ad9661 HEAD`: exit 0.
- Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a856ba7ccb7d032c3, branch u4f-build.
- Brief: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-brief.md, sha256
  135325a0bb524104bba9144b366b9ec52525851f5db140b1aca93ba4681a502f (matches the prompt), 2,040 lines, read whole.
- Runner: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-run.sh, sha256
  70eb3da5b8ce9a8e2eda582536eeab93f96fb7fbe98e2a74a28e0492ccf6ddae (matches), `bash -n` exit 0.
- `df -h /home/user` at start: /dev/vda 252G size, 27G used, 12G avail, 70 %.
- Base counts (the prompt's launch fact 2 wins over the brief's section 4 rule 5, which says refs 1382 and redact 306):
  check.mjs 11 steps pass; tests 552 measured, 632 bites, 0 failed, 0 skipped; marks 81; self 78 of 78; refs 1463;
  redact 10 of 10, 307 items, 0 findings; deps 137 crate files; wasm 27000 ticks, 0 mismatching.
- Mustafa's yes (history.md:6791-6806, read there): LEAD-BOUND (a) 400 ms, frozen (u4f-lead-bound.md's fenced value
  sha256 4717cd89...); yes to A1's movement reading; no change to marks.json, a judge file or an existing SERIES.md row
  without asking him first; his size stop past 6,000 added lines.
- Added lines (`git diff --stat 5b29296b`): 0 (notes file only, under .checkpoints/).

## Plan (blocks), in build order; each block compiles and its tests pass before the next

- B1 npt-room: `CmdTiming` A/B; 2.4's order (kind 8 per datagram; repeat filter with the kept window K = 16 =
  REDUNDANCY_CAP + 8 reorder allowance: equal copy = repeat, changed copy of an accepted seq = kind 3, any copy of a
  refused seq = repeat, none kept = `seq_outside_window`; new seq: kinds 1, 2 (join anchor, rejoin overlap), 4, 5, 6,
  then B's late / A's past window W = CMD_CATCHUP, then kind 10); signed distances; per-slot kept stamp across joins;
  lead bound a room setting in ms (400 default, ticks by `ticks_for_ms_at`); refusal records (slot, seq, kind);
  option A run path by arrival order then `canonical` with a total key (signed distance from the tick run); the
  per-tick values (cmd_on_time by stamp, press seen/lost/doubled, ticks run before stamp, commands run); 9 bites.
  Tests f7, f8, f9 (plant), f10-f13, f17, f34, f38, f39 and the 9 bite unit proofs.
- B2 npt-proto + catalogue + npt-server: `Pong.server_ns` (the server's time at the Pong's send, 8 B); the room's Pong
  sets it; the client's Pong sets 0 and the room never reads it; server tests' two literals; loopback.rs's
  `npt_client::BITES` compile fix (P33). Tests +4 (+1 A2).
- B3 npt-client: ClockSync (mean filter, ADR rules, caps), the controller (U4F-Q7 (b) with (i) and (iii) (c)),
  commands due, first stamp, stamps_skipped, twin skip, carry 80 (b)-(e), 3 bites. Tests f1-f5, f24-f27, f31, f32, f36 (i).
- B4 npt-sim: two bite sites (no-cooldown, refuse-not-clamp), bites builds only.
- B5 npt-bots: the 11 forgers, the clamp cheat, the 1.5x clock, the lag switch, the never-acker, the controller rule;
  f15, f18, f36 (ii); TE-B1-R2-1 rides.
- B6 npt-lab: settings, suites, series, per-session counters, attribution, the -47 suite, twins, the reorder event;
  f6, f14, f16, f19-f23, f28, f29, f33, f35, f37, f40; RR-CB-2 and RR-B1-7 (1) ride.
- B7 SERIES.md new rows; B8 ops/runs/c-u4f-*.mjs; B9 D7/D8 runs (the cap measurement); B10 end checks.

Size rule for these notes: added lines = `git diff --stat 5b29296b -- spikes/ docs/` insertions plus every new file's
lines (untracked files do not show in `git diff`); the launch commit's .checkpoints/ changes are not the unit's.

## Block 1: npt-room (done)

- Built: room/src/input.rs rewritten (`CmdTiming`, `Refusal` with `kind()`, `Checks`, `Relax`, the order of 2.4 in
  `offer`/`filter`/`check`, K = `KEPT_WINDOW` = 16, W = `CMD_CATCHUP`, `dist` signed distance, `due` for cmd_on_time,
  `drain` for option A); room/src/room.rs (9 new `RoomBite`s, new `ClientCounters` fields lead_refused, not_newer,
  seq_refused, move_refused, aim_refused, seq_outside_window, rejoin_overlap; `buffer_full` now kind 10 under both
  options; `LEAD_BOUND_MS` 400, `with_cmd_timing`, `with_lead_bound_ms`, `lead_bound()`, `refusals()`,
  `refusals_dropped()`; per-slot `kept_stamps` and `run_presses`; option A's run path by arrival number then
  `canonical(commands, t)` with the total key; `run_one` (press seen/lost/doubled, fire tracking, Rewind); ClientSlot
  accessors `on_time`, `presses`, `ran_early`, `commands_run`); room/src/lib.rs (exports, `BITES` 7 -> 16).
- Late/window placement: after kinds 4-6 (2.4 item 3 says "after kind 2"; placed just before kind 10 so a forged item
  that is also late is attributed to its forged kind). Recorded for the review.
- Tests (measured build): lib +3 (f8 2^31 case, f8 property 10,000 triples x 6 orders, f38 in input.rs); new
  tests/commands.rs 24 (f7, f8 room half, f10 x8, f11 x2, f12 x5, f13 x4, f17, f34, f39); bites build +9 more
  (the 9 room bite unit proofs in commands.rs `mod bites`). `cargo test -p npt-room`: lib 23, commands 24, room 19
  pass; `--all-features`: lib 23, bites 1, catalogue 1, commands 33, room 27 pass (logs .gate/u4f/b1-room-test.log,
  b1-room-bites.log, exit 0).
- Existing tests changed, each with why in its doc: input.rs's five U4e unit tests take `Checks` (expectations moved:
  a never-received sequence is `OutsideWindow`, an equal-to-buffered stamp is kind 2: NA-13); room.rs's wrap_case
  stamps one tick ahead (a first stamp at the join's tick is kind 2 by the join anchor); the lt-cmd-seq bite test
  reads `commands_repeat + seq_outside_window`; bites.rs pins 16 ids; carry 21's test runs under B and A (f9: under A
  the room's list is in arrival order, so the sort is load-bearing). f9's plant run is still to do (B10).
- Finding: under no-newer-tick / not-older-than, two equal-tick commands of one slot in one tick trip npt-sim's debug
  order assert (world.rs:2084); the unit proofs send the equal-tick command a tick later; the judged bite runs are
  release builds (c-u4r-runs.mjs:372), where the assert is compiled out.
- f39 covers leave and replace; a kicked slot's rejoin takes the same `Room::join` path (kept_stamps is never cleared),
  not run as its own case (a kick needs an event overflow): stated, not verified by a test.
- Added lines: spikes/ 1,011 insertions (git diff --stat) + commands.rs 797 = 1,808.
- Next: B2 (Pong server time).

## Block 2: the Pong's server time, the catalogue, the pump, P33 (done)

- proto/src/msg.rs: `Pong.server_ns` (u64, the answering end's time at the send; 24 B, inside 32 B); codec both ways;
  proto/tests/common/mod.rs's generator; the catalogue's `Pong` row (docs/design/netcode-proof-messages.md:92: the
  field, and 24 B in the limit cell) first, as .claude/rules/netcode.md:17 asks. server/src/pump.rs:461-468 sets
  `server_ns: now_ns`; the client's `Pong` sets 0 and the pump's `pong()` never reads it.
- Tests: proto/tests/pong.rs 3 (round trip over 1,000 seeded values and edges; 24 B inside 32, 33 B refused; the
  catalogue row's words); server/tests/pump.rs +2 (the room ignores the field in a client's Pong: neg; A2: the room's
  Pong carries `now` at its send); the two Pong literals get `server_ns`.
- P33 / D9: `cargo test --locked -p npt-server --all-features` (.gate/u4f/server-all-features.log, `-x`): exit 0,
  loopback 6, pump 20, schedule 17, gap 6 pass. Two test-only fixes: (1) the bite-homes test moved whole from
  server/tests/loopback.rs to crates/lab/tests/bite_homes.rs (a moved test: -1 npt-server, +1 npt-lab); (2) the
  tick16 bitten lab takes npt-proto's planted `Quant` through a new `ClientEndpoint::with_quant` (npt-client: my lane;
  a public item beyond 2.13's list, named for the engine-architect), since npt-server's bites feature never turns on
  npt-client's `with_bite` (the brief named E0425 only; E0599 on `with_bite` was behind it).
- The room change against the lab: `cargo test -p npt-lab -p npt-bots` (.gate/u4f/b2-lab-bots.log): exit 0, lab 60
  pass incl. `c62hb_p2_short_every_measured_lost_tick_is_explained` (the C62H-BOUND guard), bots 3 + 5.

## Block 3: npt-client, ClockSync and the controller (done)

- client/src/clock.rs (new): `ClockSync` (NTP sample with the room's exact `server_ns`; the 2 s window's mean; ADR
  0001:240-242's drift 2 ms/step, halve > 100 ms, reset > 500 ms; outstanding 8, samples 64; ping every 100 ms of host
  time at the first step at or after it; `next_tick_at`, `one_way_hi_ns`). The -47 read instant: the lab reads
  `clock().offset_ns()` at the first client step at or after 2 s from the connect's first sample (f1 does the same).
- client/src/lib.rs: `with_clock(max_lead)` (the controller; `with_lead` kept as the lab-only fixed lead, U4F-Q7 (i)),
  `with_quant`, `clock()`, `join_tick()`, `commands_due()`, `lead_now()`, `stamps_skipped()`, `not_due()`,
  `unencodable_commands()`, `twin_left_out()`, `skip_command()` (the twin's skip), `unknown_pongs()` now public;
  `plan()` (2.2: goal = the server's next tick when the command arrives at the latest, by the clock's upper one-way
  estimate plus a margin of one tick period; at least the next tick, at most max_lead past the current; target never
  down, at most +2 a step; due = stamps last+1..=target, at most 2; > REDUNDANCY_CAP owed = a skip, counted; first
  stamp = the target, only after a snapshot, an estimate and a target above the join tick); carry 80 (b) (a refused
  header more than ACK_RING_TICKS past the newest: dropped, no sentinel), (c) (`drop_unencodable`), (e) (`full_state`
  files the sentinel on another hash); BITES 3 -> 6 (newest-sample, tick-quantised, redundancy-1).
- The margin (the builder states it, 2.2): one tick period beyond the clock's one-way upper estimate (max RTT / 2 +
  (max - min) RTT / 2). Without it f36 read 99.800 % on time at P3 (a finding, fixed before any run).
- Tests (bites build): clock.rs unit 6 (f2 x3, f3 x2, f31's clock half), trace.rs 7 measured (f1, f4, f5, f32 x3,
  f36 (i)) + 4 bites-only (newest-sample, tick-quantised, redundancy-1, f30's client half), carry80.rs 4 (f24-f27);
  client/tests/bites.rs pins 6 ids. All pass (.gate/u4f/b3-client.log, exit 0).
- Figures (trace model, seeded): f1 p99 |err| at 2 s on P2 over 1,000 connects 2.380 ms (max 3.434 ms; D4's mean
  2.346); f36 at P3, 101,690 ticks: on time 99.988 %, missing 0.000 %, skipped 0, lead 5; f5's settled leads P0 3, P1
  3, P2 4, B1 4, P3 5, P4 6 (U4R-Q14's 2, 3, 5, 5, 7, 9), not on time 0 except P4 0.067 % and B1 1.879 % (its 1 s
  burst inside the window; budget 3 % for B1 in f5, stated in the test).
- Not done here: f6 (lab), f30's room half (done in B4's commands.rs), the plants of f26 and f27 (B10).

## Block 4: npt-sim's two bite sites (done)

- sim/src/world.rs: `Bite::NoCooldown`, `Bite::RefuseNotClamp`, BITE_IDS 4 -> 6; `World::BITES` stays the pinned 4
  (sim/tests/world.rs:2292-2301 and npt-hash-wasm's indexes are outside my lane); the two -21 ids are a new list
  `World::CHECK_BITES` (bites builds only), planted through `World::with_bite`. no-cooldown: a fire edge in the cooldown
  fires, unrefused; refuse-not-clamp: no clamp report, the shot refused and counted as RefusalKind::Cooldown (no new
  seam kind: a new RefusalKind would be a seam change for the advisor). Both bites-only.
- Unit proofs in room/tests/commands.rs `mod bites` (through a room over a bitten World; npt-sim's tests are outside the
  lane): no-cooldown (Cooldown refusals clean > 0, bitten 0, hashes part), refuse-not-clamp (clean: clamps, no
  refusal; bitten: refusals, no clamp report). Also f31's room half (400 ms = 12 at 30 Hz, 24 at 60 Hz) and f30's room
  half. `cargo test -p npt-sim --all-features` exit 0; room commands (bites) exit 0; lab bite_homes exit 0.
- Added lines after B1-B4: 1,343 insertions (spikes/, docs/) + 2,069 in new files = 3,412 (against 6,000: the
  remaining bots, lab, SERIES.md and scripts must be lean).
- Next: B5 npt-bots.

## fmt and clippy after B1-B4 (done)

- `cargo fmt --all -- --check` printed 27 diffs; each applied by Edit (no `cargo fmt` writing). clippy found: an unused
  bites-only method (`ClockSync::with_newest_sample`, now `#[cfg(feature = "bites")]`), a misnamed getter (the room's
  refusal buffers renamed `incoming` and `refusals`), and unequal hex digit groups in five test seeds (now
  0x5eed_00f1, 0x5eed_0f32, 0x5eed_00f8, 0x5eed_0f12, 0x5eed_009f; the docs name them). Then fmt, clippy host and
  clippy all-features each exit 0 (.gate/u4f/b4-fmt-clippy.log, 12:35 UTC).

## Block 5 (npt-bots) and Block 6 (npt-lab): the core built, the cheats not (partial)

- Built: `Behaviour::controlled()` (the controller runs' command rule: each step gives `commands_due()` commands, the
  press rule kept), `Behaviour::brawl_presses` (U4F-Q11 (i) (a): a fire press every second command). npt-lab:
  `--cmd-timing A|B` (the room's option via `Room::set_cmd_timing`, a new in-place setter), `--controller 0|1`
  (endpoints `with_clock(9)`, behaviours `controlled()`), `--behaviour brawl-presses`; header `cmd_timing` from the
  setting, `lead_ticks_count` only without the controller (TE-20), `controller_max_lead_count`, `lead_bound_ms`;
  logs.rs `room_tick` writes `tick.s.cmd_on_time`, `tick.s.input_missing` (by stamp), `tick.v.press_count`,
  `.press_lost_count`, `.press_doubled_count`; the stop's `client_counts` gains the room's refusal fields by kind, the
  endpoint's controller counts and the session's pump counters (2.5); the run's `reorder` event (D3-1, f40).
- SERIES.md: one new subsection "Names added with U4f" with 5 new rows (no existing row touched): the two header params,
  the client_counts fields (room and endpoint), the client_counts pump fields, `event.reorder.*` (B9).
- Lab tests added: f6, f28 (its -20 half), f40.
- **Not built (goes to the main session and the advisor):** the 11 forgers, the clamp cheat (with its in-map
  points), the 1.5x clock and its episodes, the lag switch, the never-acker, their suites (`forged`, `clamp`,
  `fast-clock`, `lag-switch`, `never-ack`, `clock`), the twins and the twin's skip in the lab (the endpoint's
  `skip_command` exists), the lab's per-kind attribution (`event.forged_kind.*`), the clamp, fast-clock and lag-switch
  series, the -47 suite (connects, `event.connect_clock.offset_err_ns`), -22's series, the per-session counters at
  mid-run session ends (only at the stop), `second.s.buffer_depth_count` and U4F-Q3 (c)'s ran-early series in the
  logs (the room exposes `ran_early()`), the cap measurement's runs (2.8, D8), ops/runs/c-u4f-*.mjs, RR-B1-7 (1),
  RR-CB-2's docs, TE-B1-R2-1. Tests not written: f14-f16, f18-f23, f29, f33, f35, f37; f36 (ii) (bots); the plants of
  f9, f26, f27 and D14. Why: the size so far (about 3,600 added lines against the 6,000 stop, with the cheats and
  their lab plumbing estimated at 2,000 or more) and this builder's remaining working room; a second builder can take
  them from these notes (the room, client, proto, server and sim parts they depend on are built and tested).
- Lab test figures (.gate/u4f/b6-lab-test.log): f28 at P3, 16 clients, 4 s measured: option A on time 1,920 of 1,920,
  presses 961, lost 0; option B 1,920 of 1,920, presses 960, lost 0. f40: `reorder` permille 99 (up 164, down 99:
  RFC 4737's count includes the jitter's reorders at P3, far above the step's 10).

## Plants (procedure; P26: each by Edit in place, its diff in these notes, restored and its sha256 checked)

- f9: room.rs `canonical`'s body replaced by nothing (no sort); `cargo test -p npt-room --test room carry_21` must fail
  under option A; restore; sha256 of room.rs before and after equal.
- **f9 done** (.gate/u4f/plant-f9.log, `-x -k`, 12:43:31 UTC): the plant, room.rs:387-389, was
  `-    commands.sort_by_key(|(c, _)| (c.slot, c.tick.wrapping_sub(t) as i32));`
  `+    let _ = (commands, t); // PLANT f9: the sort removed`; `carry_21_commands_received_in_two_orders_give_one_tick_input`
  FAILED (it panics at npt-sim's canonical-order debug assert, world.rs:2129, under option A: the tick's list arrives
  in arrival order); restored by Edit; room.rs sha256 ff28326061e47cca62c17720a2a45cdeb3f4999059bf1e37d11618c02818dd02
  before and after (equal). Not done: f26's two plants, f27's, D14's old comparator (no time left; the comparator's
  case is f8's property test, which runs the new key only).

## Block 6 check (done)

- `cargo test -p npt-lab -p npt-bots` (.gate/u4f/b6-lab-test.log, exit 0, 12:43:28 UTC): lab main.rs 63 passed, 1
  ignored (140.5 s), the C62H-BOUND short P2 test among them; bots 3 + 5. fmt, clippy host and all-features exit 0
  (.gate/u4f/b6-fmt-clippy.log, 12:44 UTC), after seven fmt diffs applied by Edit.
- Base's test step for the time rule: E6's 804.8 s (u3c-held-1/build.md:149), so the stop line is 1,207 s.
- Added lines at the counted run: 1,595 + 2,147 = 3,742 (git diff --stat spikes/ docs/, plus new files); the fmt edits
  after it add about 20.
- `df -h /home/user` before end check 1: 8.8G avail (78 %).

## End check 1 (done: all 11 steps pass)

- One `-x` runner call (.gate/u4f/end-check-1.log, 12:44:36-12:59:31 UTC; free 8,970 MiB at the start; HEAD 827518b;
  diff 91ab1663..., untracked 13182a28...): the six provenance builds each exit 0 (test --no-run both builds, npt-lab
  release both builds, npt-hash-wasm wasm32 release both builds), then `node check.mjs`: fmt 0.7 s; clippy 4.6 s;
  clippy-wasm32 7.7 s; doc 7.5 s; **test 698.9 s, measured build 605 passed (base 552: +53), bites build 701 (base 632:
  +69), 0 failed, 0 reported a skip**; deps 4.3 s (143 crate files: +6 new .rs files); wasm 47.8 s (27,000 ticks, 0
  mismatching both ways; hash-map and std-sin bites caught as before); refs 1463 tracked files; marks 81; self 78 of
  78; redact 10 of 10, 313 items (base 307: +6), 0 findings; "check: all 11 steps pass"; exit 0. Provenance: every
  npt-* crate "Compiling ... (<my worktree>/spikes/netcode-proof/crates/...)".
- The test step 698.9 s against base's 804.8 s (E6): under 1.5 times.

## End check 2 (done: all 11 steps pass, the same tree)

- .gate/u4f/end-check-2.log, `-x`, 13:01:53-13:14:59 UTC; free 5,179 MiB at the start; HEAD 827518b; diff 91ab1663...,
  untracked 13182a28... (equal to end check 1's: no edit between); the six provenance builds exit 0; fmt 0.6 s, clippy
  4.0 s, clippy-wasm32 1.0 s, doc 4.5 s, **test 693.8 s: 605 and 701 passed, 0 failed, 0 skipped**; deps 143 crate
  files; wasm 27,000 ticks, 0 mismatching both ways; refs 1463; marks 81; self 78 of 78; redact 10 of 10, 313 items, 0
  findings; "check: all 11 steps pass"; exit 0. `df -h /home/user` after: 5.1G avail (87 %).
- The disk: 12G free at my start, 5.1G at my end (the shared target's debug, release and wasm32 builds of both
  builds, cold at launch); above the 4 GB floor throughout. I cleaned and packed nothing.

## Time rule, each new test alone (.gate/u4f/test-times.log, `-x -k`, 13:00-13:01 UTC; "finished in" lines)

- c62hb_p2_short_every_measured_lost_tick_is_explained 11.48 s, so the line is 22.96 s.
- **f6 23.86 s and f28 23.78 s: past the line (each runs two short scenarios, options A and B): a stop for the
  advisor (section 1 item 5; 2.10's rule).** Not changed after end check 1 (splitting each into one test per option
  would put each near 12 s; the advisor's call). f40 12.03 s.
- Under 1.4 s each: f1 1.09, f36 1.32, f5 0.13, f32 (three) 1.10, f8 property 0.03, f38 0.02, commands.rs (25) 0.12,
  its bites (12) 0.01, trace bites (4) 0.01, carry80 (4) 0.00, clock unit (6) 0.00.

## Size, the bars line, what was not verified

- SIZE-CHECK: `git diff --stat 5b29296b -- spikes/ docs/` 1,606 insertions (18 files) + 2,147 lines in 6 new files
  (client/src/clock.rs 348, client/tests/carry80.rs 158, client/tests/trace.rs 556, lab/tests/bite_homes.rs 48,
  proto/tests/pong.rs 84, room/tests/commands.rs 953) = **3,753 added lines** (the plan's M 3,000; the brief's 5,150;
  the stop 6,000). About 60 % of it is tests.
- D10, the bars line: `git diff --stat 5b29296b -- spikes/netcode-proof/marks/marks.json spikes/netcode-proof/crates/judge`
  prints nothing. SERIES.md: 5 new rows in one new subsection, no existing row changed (the reviewer's read is
  recommended: c62h-review :93).
- `git diff --stat 5b29296b -- spikes/netcode-proof/crates/sim`: world.rs 48 insertions, 3 deletions (bites-only sites).
- Not verified: any judged or 5.3 run (none exists for U4f's suites, which are not built); the controller under the
  real lab at P3 for 10 minutes (f28 ran 4 s measured; f36 is the model's); ClockSync on the lab's L1 (f1 is the
  model's, D4's parameters); the plants of f26, f27 and D14; f36 (ii) as its own bots test (f28's 100 % on time and 0
  missing at P3 under both options shows the controller rule at the lab's level); a kicked slot's rejoin case of f39.

## Rulings as applied (U4F-Q1 to Q18; return item 12)

- Q1 (a)+(d): the lane plus crates/sim/src/world.rs (two bite sites, bites only), SERIES.md new rows only, and by Q6 (b)
  crates/proto, the catalogue's Pong row, server/src/pump.rs's Pong, server/tests/pump.rs and loopback.rs. Beyond:
  crates/lab/tests/bite_homes.rs (new: the moved test) and `ClientEndpoint::with_quant` (P33's second error).
- Q2 (a): nothing packed or cleaned by me. Q3 (c): A's depth 0; `ClientSlot::ran_early()` built, its series not
  written (not built). Q4 (b): the -47 suite not built. Q5 (a): the stop clause recorded; no failure outside the lane.
- Q6 (b): `Pong.server_ns` (8 B, exact ns), the mean filter; f1 at 4.0 ms (measured 2.380 ms).
- Q7 (b)+(i)+(iii)(c): the controller on ClockSync; `with_lead` kept as the fixed lab setting; contiguous stamps,
  at most 2 a step, a skip only beyond REDUNDANCY_CAP owed; stamps_skipped 0 on f32 and f36.
- Q8 (i)(b) at 400 ms (12/24 ticks: f31), (ii) (c) a carry to U4h (the main session's), (iii) (e) given (his yes).
- Q9 (a) K = 16. Q10 (a): bite runs' options are the runner's (not built). Q11 (i) (a) `brawl_presses`; (ii)-(v) the
  suites not built. Q12 (b): the cap measurement not run. Q13 (b): `skip_command` built, its lab use not built.
- Q14 (b): the pump's counters reported at the stop; the hold-clamp counter not built (a follow-up as ruled).
- Q15 (c), Q11 (v), the 400 ms hold (p19): not built (the lag switch). Q16 (b): ACK_RING_TICKS (f24). Q17 (a): the
  total key (f8, f9). Q18 (i)(a) W = CMD_CATCHUP and (ii)(d) the kept stamp per slot (f13, f38, f39).

## Carries (return item 8)

- 46 met at the room's level: f17 (room/tests/commands.rs), a forged press stream and its twin give equal hashes;
  the lab's -21 twins not built.
- 80 (a) met: the total key (room.rs `canonical`), f8 (2^31 apart; SA-1's triple on a fresh slot; 10,000-triple
  property), f9 (carry 21's test made to fail without the sort: the plant run). (b) met: client/src/lib.rs `receive`,
  f24. (c) met: `drop_unencodable`, f25. (d) met: f26 as worded (its two plants not run). (e) met: `full_state`, f27
  (its plant not run).
- 86 not met: c-u4f-r2.mjs not written (5.3's pre-runs are the main session's after the merge, on a script that does
  not exist yet).
- 57 not met: the cap measurement (2.8, D8) not run; the room's cap stays provisional at 1,024.
- 74 met for the endpoint: the controller replaces the fixed lead in controller runs; f5's settled leads beside
  U4R-Q14's; the late share by stamp reported in f28 (0 late at P3 short).
- 21 met (option A's several commands per slot sorted: f7, carry 21's test under A). 27 met (CMD_CATCHUP and its
  const assert kept; option A's cap reads it; f7, f10 kind 10).
- 18 not applicable (Q8 (b): no npt-sim constant). 39 met: no local tick constant; the clock's period and ping
  interval, the lead bound's ticks pinned at 30 and 60 Hz (f31 x2); the margin is one period in ns.
- 50 no code change (2.8 item 4's bound applies to a cap not yet measured). 61 (b) completed by 80 (e) (f27).
- 58 not met (the cap runs' high-water marks). 76: kept (r8, r9 pass); no forged kind closes a session (no suite
  built). 77 (a), (c): `stale()` in client_counts as before; not read on U4f runs (no 5.3 runs).

## Public items by crate (return item 9; against 2.13's list)

- npt-room: `CmdTiming`, `Refusal` (+ `kind()`), `InputBuffer::KEPT_WINDOW`; `Room::LEAD_BOUND_MS`, `with_cmd_timing`,
  `set_cmd_timing` (beyond 2.13: an in-place form for the lab), `with_lead_bound_ms`, `cmd_timing`, `lead_bound`,
  `refusals`, `refusals_dropped`; `ClientCounters` fields lead_refused, not_newer, seq_refused, move_refused,
  aim_refused, seq_outside_window, rejoin_overlap; `ClientSlot::on_time`, `presses`, `ran_early`, `commands_run`
  (the last four beyond 2.13's words: the per-tick values -20's series and Q3 (c) read); `BITES` 7 -> 16.
- npt-client: `ClockSync`, `ClockSample` (plan.md:610); `ClientEndpoint::with_clock`, `with_quant` (beyond 2.13: P33),
  `clock`, `join_tick`, `commands_due`, `lead_now`, `stamps_skipped`, `not_due` (beyond: a hold's dropped page
  command), `unencodable_commands`, `twin_left_out`, `skip_command`; `unknown_pongs` now public; `BITES` 3 -> 6.
- npt-bots: `Behaviour::controlled`, `Behaviour::brawl_presses` (the cheats' constructors not built).
- npt-sim: `World::CHECK_BITES` (bites only; beyond 2.13's "two ids in World::BITES": a list apart, so the pins
  outside the lane hold). npt-proto: `Pong::server_ns`. npt-server: none. npt-lab: binary settings `--cmd-timing`,
  `--controller`, `--behaviour brawl-presses`.

## What stops or goes to the main session (return item 10)

- No manifest, Cargo.lock, marks.json, judge or existing SERIES.md row change; no tick constant needed (carry 39); no
  wire change beyond U4F-Q6 (b)'s field; no file outside the lane but the two named under Q1 above.
- The sim-side refusal for refuse-not-clamp uses RefusalKind::Cooldown (no new seam kind): a reading for the advisor;
  a new RefusalKind variant would be a seam change.
- The late/past-window check's place in the order (after kinds 4-6, before 10) for the advisor's read.
- `header.params.behaviour`'s row (SERIES.md:348) lists its values; `brawl-presses` is a new value of that existing
  row, written without touching the row's text: for the reviewer's read (a new row would duplicate the name).
- The summary line's "lead N" prints `lead_for(profile)` even on a controller run (cosmetic; the header is right).
- The unbuilt parts listed under Block 5/6 above, and the plants of f26, f27 and D14.

## Builder 2

Task: U4f's second builder (general-purpose, opus, effort xhigh): finish U4f (clock and commands) from builder 1's partial
build, by the 22 lines of advisor-u4f-build1.md:162-246 (sha256 da98295e...), u4f-brief.md rev 3.3 (sha256 135325a0...)
and the main session's prompt. Where the prompt and the ruling differ, the prompt wins: line 1's worktree is replaced by
a fresh worktree (builder 1's, agent-a856ba7c..., stays untouched as a backup); logs under .gate/u4f/ start `w2-`.

### Start (done)

- Ruling sha256 da98295e0d0eae7c12e46f0a282dce3c452f75dc74346c3f55aec26baa84f7cb, brief 135325a0...2a681a502f, runner
  70eb3da5...ddae6ddae: all three match the prompt (`sha256sum`).
- Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/agent-aa8301a8782f8f434 (made at origin/main 22c6afa0).
  `git merge --ff-only d1b24c3b21cfd6715bcc45231eed3e17f0a28b17`: exit 128 (diverging); then `git switch -c u4f-build2
  d1b24c3b21cfd6715bcc45231eed3e17f0a28b17`: "Switched to a new branch 'u4f-build2'". Branch u4f-build2.
- `git log -1 --format=%H`: d1b24c3b21cfd6715bcc45231eed3e17f0a28b17 (launch head 2).
- `git diff --quiet 5b29296b03f4e116c0afac8b89d9bc15cc1adaf7 HEAD -- spikes/`: exit 0.
- `git merge-base --is-ancestor 0ad9661 HEAD`: exit 0.
- `sha256sum .checkpoints/netcode-proof-test/wip/u4f-build.patch`: 92fc99aa9f01aebf98c21f5d106a827a7a254e024d65702b733654f2eef2e837
  (matches); `git apply --check`: exit 0; `git apply`: exit 0 (the one git write).
- Line 2's checks: `git diff --stat 5b29296b -- spikes/ docs/`: 18 files, 1,606 insertions, 248 deletions; `git status
  --short`: those 18 modified and 6 untracked new files (lab/tests/ holds bite_homes.rs only), lines by the Grep tool's
  count of `^`: clock.rs 348, carry80.rs 158, trace.rs 556, bite_homes.rs 48, pong.rs 84, commands.rs 953 = 2,147;
  3,753 added in all. Equal to the notes: no stop.
- Mustafa's words read at history.md:6801 and :6803 (no marks.json, judge or existing SERIES.md row change without
  asking him first; the size stop past 6,000 added lines).
- `df -h /home/user` at start: 252G size, 27G used, 12G avail (70 %).

### B1: the fixes of lines 6 to 14 (done; lines 12 and 13 apply with the cheats, B7 and B8)

- Line 6 (F3, F11): scenario.rs's `u4f_short(timing)` now makes each option's short run once per test process
  (`OnceLock`, its three logs read into memory, the scratch folder removed); f6 and f28 are one test each per option
  (`f6_option_a_...`, `f6_option_b_...`, `f28_option_a_...`, `f28_option_b_...`); each f28 test runs
  `contract::check` on its run's logs against marks/SERIES.md and asserts no failure at all (stronger than no name
  failure: the (c) rules and the class audit pass too at 16 clients), `lead_ticks_count` absent, every client's
  `pump_input_limited_count` written (16), and -20's series as before; f40 reads option B's shared run.
- Line 7 (F4): `--behaviour brawl` stays the brawl; `--brawl-press-every <n>` (0 off; the brawl only, else a usage
  error) gives npt-bots' new `Behaviour::with_press_every(n)` (replaces builder 1's `brawl_presses`: a fire press on
  every n-th command; n = 2 is the old rule); header `params.brawl_press_every_count` only when on, with a new row in
  U4f's subsection; `brawl-presses` is gone from the settings, the header and the usage line.
- Line 8 (F5): the `header.params.lead_bound_ms` row's meaning cell holds the ruling's words exactly.
- Line 9: f11 (ii) gains, under both options, a late command breaking kind 4 and one breaking kind 6: refused (3, 4)
  and (4, 6), late 1 (the honest one), forged [0, 0, 0, 1, 0, 1, 0, 0] (+0 tests).
- Line 10 (F2): `Bites::of` sets `world` from `World::BITES` or `World::CHECK_BITES`; a bites-build lab test
  `f2_the_sims_check_bites_are_routed_to_the_world_alone` (+1 bites only).
- Line 11 (F6, F7): `event.client_counts.unknown_pongs_count` written (the endpoint's `unknown_pongs()`, so it stays
  public), added to U4f's client_counts row; `Room::set_cmd_timing` debug-asserts no slot has joined.
- Line 14 (F10): the summary line prints "lead the controller's, at most 9" on a controller run.
- Logs: .gate/u4f/w2-b1-test.log (`cargo test --locked -p npt-room -p npt-bots -p npt-lab`, then `-p npt-room -p
  npt-lab --all-features`), 13:51:32-13:57:48 UTC, both exit 0: measured lab main.rs 65 passed, 1 ignored (builder 1's
  63: f6 and f28 split, +2); room commands 25, room 19, lib 23; bots 3 + 5; bites build lab main.rs 76 passed (f2's
  test among them), commands 37, room 27, lib 23, bite_homes 1.
- SIZE-CHECK after B1: `git diff --shortstat 5b29296b -- spikes/ docs/` 1,683 insertions (18 files) + 2,158 lines in
  the 6 new files (commands.rs now 964) = **3,841**.

### B2: -20's rest (line 16), with B3's lab writers (in progress)

- logs.rs: the room log's second records gain `second.s.buffer_depth_count` (each joined client's input buffer depth
  at the second's last tick, `ClientSlot::buffer_depth_ticks`: 0 under A by construction, input.rs:192-196) and
  `second.s.event_queue_max_count` (B3's high-water series: per slot, the most events queued at once in the second, the
  cap less the tick's lowest `headroom()`, so a count before an overflow resync cleared them); every room tick record
  gains `tick.s.cmd_ran_early_count` (U4F-Q3 (c): under A each command run's stamp less the tick; empty under B). Two
  new SERIES.md rows (`tick.s.cmd_ran_early_count`, `second.s.event_queue_max_count`); `buffer_depth_count` has its
  row (:101). `RoomTrack::second` became a `Second` struct (the per-client samples beside the sums).
- scenario.rs: `--event-cap <n>` (B3's opened cap and f21's short run: `Room::with_event_cap`, so the header records
  the cap and `event_queue_cap_provisional` false).
- f28's -20 half asserts both new -20 series written (depth all 0 under A only; ran-early entries under A only).
- f36 (ii): npt-bots' `f36_ii_a_controller_run_gives_every_stamp_and_a_fixed_lead_run_keeps_its_rule` (+1): a
  lossless loop of 300 steps (a snapshot each step but every fifth, pings answered at once with the server's time).
- TE-B1-R2-1 with RR2-B1-1 rode (P35): the leash test's dead input at (0, 0) (bots lib.rs), toward_centre's doc "is at
  most 2^31".
- Finding (fixed, cause known): f36 (ii)'s first run failed at its own count assert, `stamps.len() > 250` with 240
  stamps on the fixed-lead half: 240 is that half's moved steps by construction (300 less every fifth), so the bound
  was the test's error, not the code's; now `>= 240` (.gate/u4f/w2-b2-test2.log:33-34; the controller half passed).
  `cargo fmt --check`'s two diffs (scenario.rs) applied by Edit (w2-b2-test.log).
- The B2 tree's check: .gate/u4f/w2-b2-test3.log (`cargo test --locked -p npt-bots -p npt-lab`, 14:01:58-14:04:05
  UTC, exit 0): bots lib 4 passed, 1 ignored (f36 (ii) among them); lab main.rs 65 passed, 1 ignored.
- **The plants (P26: each by Edit in place, one `-x -k` runner call, restored by Edit, the file's sha256 checked):**
  client/src/lib.rs clean 1f932bd56ffcf2cd77466e945e9b674bf1f0aa036a27c58499345f26353a7a13; room/src/room.rs clean
  8d7ad78f5cb151436716bb84ba7226eb5a0dc2ae852735615cb344e638945c6a.
  - f26 half 1 (w2-plant-f26-1.log, 14:04:27): in `old_header`'s before-the-first-header branch, `self.bad_header(t);`
    replaced by `let i = self.slot_for(t); self.ring[i].filed = Filed::Sentinel; self.out.hashes.push((t, 0));
    self.acknowledge(t, 0);` (a ring slot taken, the bad tick acknowledged): `f26_the_clients_two_too_old_cases`
    FAILED at carry80.rs:112, "filed once", left [(60, 0), (60, 0)]; restored, sha256 1f932bd5... (equal).
  - f26 half 2, the P-form (w2-plant-f26-2.log, 14:04:43): in `old_header`'s from-the-first-header branch, after
    `self.stale += 1;`, `self.out.hashes.push((t, 0));` (the too-old tick given the sentinel too): f26 FAILED at
    carry80.rs:124, "nothing filed: [(105, 0)]"; restored, sha256 1f932bd5... (equal).
  - f27 (w2-plant-f27.log, 14:05:01): in `full_state`, the hash compare's `match` replaced by `let _ = hash;
    self.duplicates += 1;` (the old rule): `f27_a_full_state_of_another_hash_after_a_snapshot_files_the_sentinel`
    FAILED at carry80.rs:147, left [] right [(105, 0)]; restored, sha256 1f932bd5... (equal).
  - D14 (w2-d14.log, 14:05:18, `cargo test --locked -p npt-room --all-features --lib f8_`): `canonical`'s body
    replaced by the head's old comparator (`sort_by` with `(b.tick.wrapping_sub(a.tick) as i32).cmp(&0).reverse()`
    after the slot; 5b29296b room.rs:363-368). **The outcome: it misorders, no panic**, on the toolchain's sort of 3
    elements: `f8_one_slots_commands_2_pow_31_ticks_apart_sort_to_one_order` FAILED at room.rs:1708 "at 0" (the two
    arrival orders sort ticks 0 and 2^31 two ways: left (0, 2^31), right (2^31, 0)), and the property test FAILED at
    room.rs:1755 on case 0 (its order is not by signed distance from the tick run); no panic message in either.
    Restored, sha256 8d7ad78f... (equal).
