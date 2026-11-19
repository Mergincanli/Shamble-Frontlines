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

### B2: -20's rest (line 16), with B3's lab writers (done)

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
- B2 done (its tree's tests above). The run was cut by the org's spend limit at 14:06:57 UTC and resumed at 16:3x UTC
  (the coordinator's message: no plant left in place, room.rs 8d7ad78f... and lib.rs 1f932bd5..., the last Edit of
  SERIES.md whole; the worktree snapshotted as wip/u4f-build.patch, 25 files). Before the cut, B3's script
  (ops/runs/c-u4f-cap.mjs) and part of B4 (below) were already written, so B2's own count was not taken alone.
- SIZE-CHECK at the resume (B1, B2, B3's script and lab writers, B4 in part): `git diff --shortstat 5b29296b --
  spikes/ docs/` 1,949 insertions (18 files) + 2,298 lines in 7 new files (the 6, and c-u4f-cap.mjs 140) =
  **4,247**.

### B3: the event cap's measurement (line 17; 2.8 items 3-5; U4F-Q12 (b))

- The series: `second.s.event_queue_max_count` (B2's logs.rs edit, its new row). The script:
  ops/runs/c-u4f-cap.mjs (on c-u4r-r2.mjs's model; its doc states the method): one release build of npt-lab, then the
  nine runs, SS, 48 clients, the brawl under U4f's controller (`--controller 1`, option B), a 10 s warm-up and 60 s
  measured (two 30 s bins), B1 with 2 bursts (`--bursts 2 --burst-first-s 10 --burst-every-s 30`), seeds 1 to 3, each
  with the cap opened (`--event-cap 8192`) so no overflow resync clears a queue: the high-water marks are the need, not
  the provisional cap's (a run at 1,024 would cut them there). Per run: the room-wide reliable events a second (mean
  and max per 30 s bin), each client's mean queued rate (`events_queued_count` over the run's seconds), the per-client
  high-water mark per 30 s bin (`second.s.event_queue_max_count`) and per burst (`event.burst_heal.queue_max_count`).
- The run: `bash <runner> -x <worktree> .gate/u4f/w2-b3-cap.log node ops/runs/c-u4f-cap.mjs --date 2026-11-18 --out
  .gate/u4f/w2-cap` (16:44:12 UTC, exit 0; `df -h /home/user` 11G free before; HEAD d1b24c3, `git diff HEAD` sha256
  fc1f11ba..., builder evidence on an uncommitted tree). Figures (.gate/u4f/w2-cap/cap.txt), every run exit 0, 0
  overflow resyncs, about 35 s wall each:
  | Run | rate a second, bin 0 / bin 1 (mean; max) | client queued rate (low-high) | high-water bin 0 / bin 1; bursts |
  |---|---|---|---|
  | P2 s1 | 559.0 / 553.3 (631; 615) | 543.1-547.9 | 161 / 156 |
  | P2 s2 | 568.1 / 557.7 (631; 643) | 549.7-554.7 | 166 / 152 |
  | P2 s3 | 572.5 / 550.2 (652; 644) | 545.4-550.5 | 157 / 188 |
  | P3 s1 | 538.7 / 511.2 (610; 597) | 514.7-518.5 | 689 / 1,001 |
  | P3 s2 | 529.8 / 503.4 (603; 563) | 509.2-513.4 | 683 / 449 |
  | P3 s3 | 546.9 / 513.0 (598; 593) | 517.5-521.6 | **1,045** / 935 |
  | B1 s1 | 542.3 / 545.7 (598; 607) | 533.7-538.6 | 747 / 651; bursts 727, 615 |
  | B1 s2 | 547.5 / 537.6 (597; 613) | 532.6-537.6 | 669 / 695; bursts 628, 661 |
  | B1 s3 | 560.6 / 536.2 (598; 596) | 534.7-539.8 | 739 / 633; bursts 706, 616 |
- **The value:** the largest per-client high-water mark, 1,045 (P3 seed 3, bin 0: above the provisional 1,024, as
  P32's 27 forced resyncs at P3 showed), times the stated margin 1.5, rounded up to a multiple of 64: **1,600**; carry
  50's bound, the lowest client's mean queued rate (509.2 a second, P3 seed 2) x 255 / 30 = **4,328**: 1,600 is below
  it, so a never-acker meets the cap before its events expire. Into the room's default (room/src/events.rs
  `EventQueue::CAP` = 1,600, replacing `PROVISIONAL_CAP`; room.rs `Room::new`: `event_cap_provisional` false), its
  derivation in the constant's doc; room/tests/room.rs:1139's default pin changed to (1,600, false) (the test's own
  claim changed with the plan, its doc line says so). Plan §4.7's text is the main session's. The figures and the
  value go to the advisor before any -22 run (U4F-Q12's ruling).
- Not measured here: P0 (the -22 suite's profile) and 10-minute windows; the short runs' bins are 30 s, so a peak
  later in a long run (P32's s1 overflow came about 7 minutes in) is outside them: the margin covers what they did not
  see, by statement, not by measurement.

### B4: -47's clock suite (line 18) (code and tests written before B3's runs; inert outside `--suite clock`)

- An order note: B4's lab code went in before B3's nine runs (it was half written at the cut), so B3's runs are made
  on a tree holding it; it acts only under `--suite clock` (each use is behind `s.suite == "clock"`), which no B3 run
  names. Recorded as a deviation from line 4's order, with no effect on B3's figures.
- npt-bots: `Behaviour::never_fires` (a new `Kind::NoFire`: the open-loop walk, `buttons` 0, no press), the clock
  suite's script (U4F-Q4 (b); 2.13's "a no-fire clock-suite script").
- npt-lab: `--suite clock --connects <n>` (each needs the other: a usage error otherwise); the clock suite turns the
  controller on (ClockSync pings only under it); `params.connects_count` in its header and no `behaviour` param (the
  never-firing script is a value SERIES.md:348's row does not list: F4's rule), a new row; each client's estimate read
  at its first step at or after 2 s from its first sample (`event.connect_clock.offset_err_ns`, fields `slot` and
  `seq`, |`clock().offset_ns()` - the seeded host offset|; row :149), then, while connects remain, the slot connects
  again: a fresh L1 link (the old one dropped, so the room frees the slot: `Room::leave` on the closed link), a fresh
  endpoint (`endpoint_of`, the bite as before) and a fresh seeded host offset. No client hash is filed (`logs.filed`
  skipped) and each admitted slot is `gone` at once, so no tick waits on a client; the warm-up's carry 62 (d) check is
  skipped for the suite. The endpoint's construction moved into `endpoint_of(s, slot)` (one place for both uses).
- Tests (+2, measured): `f29_the_clock_suite_reads_each_connect_once_and_stamps_no_event` (16 slots, 32 connects, P2,
  1 s + 8 s: 32 reads, connect numbers 0..32 once each, 32 admits, every second's shots and reliable events 0, no
  client tick record) and `f28_the_clock_suite_writes_each_name_with_its_row` (the contract checker's (b): no name
  failure; `connects_count` 32, no `behaviour`; its one-admit rule does not hold for a suite that reconnects), sharing
  one run (`clock_short`). Log: .gate/u4f/w2-b4-test.log (fmt --check exit 0, then the bots and lab tests,
  16:39:41-16:43:58 UTC, exit 0: bots lib 4 passed, 1 ignored; lab main.rs 67 passed, 1 ignored).

### B5: -22, the never-acker and 2.5's counts at every session end (line 19) (written during B3's runs: see B3)

- npt-bots: `Behaviour::never_acks()` and `Behaviour::outgoing(&mut [Vec<u8>])`, the cheats' hook on what the
  endpoint sends at a step (2.6's form: the honest datagram decoded and re-encoded with only the cheat's change): the
  never-acker's `Input`s get ack tick 0 and ack groups 0; an honest behaviour changes nothing. npt-lab calls it on every
  client step's datagrams.
- npt-lab, `--suite never-ack`: slot 0 brawls and never acknowledges, every other client brawls; the room log writes
  `tick.v.ack_starved_headroom_count` on every room tick record (slot 0's `ClientSlot::headroom`, the cap while no slot
  of its is held; row :21) and `event.ack_starved.dropped` (field `slot`; row :128) at each forced resync (0) and at the
  kick (1); `Logs::starved` names the slot.
- 2.5 (NA-15): `Server::session_ends` keeps each joined session's counts at its end, read before the room frees the
  slot or the `Decoder` drops: a `Leave` before it is routed to the room; a closed link before `Room::leave`; a
  replaced session's room counts read before the replacing join resets the slot (each held slot's, cloned before
  `Room::join`) and its pump's before `drop_slot`; a kicked slot's at its kick in `Server::send` (its room counts
  survive). The run writes each as a lab-log `session_end` event (fields `slot`, `ended`, and the counts), one new row.
  `counts(e, room, pump)` writes the counts from two tables, `ROOM_COUNTS` (every refusal field of `ClientCounters`,
  20: builder 1's 15 and kind 8's commands, the old acks, the `Resync`s dropped and limited, the kicks) and
  `PUMP_COUNTS` (every refusal field of npt-server's `Counters`, 21: builder 1's 9 and 12 more), used by
  `client_counts` too (its U4f rows gain the new names).
- f39's kicked-slot rejoin case (F9; +0): room/tests/commands.rs `kicked_rejoin(timing)`, run inside f39 under both
  options: a stress-scene room, slot 0 (never acknowledging, as every `Input` of those tests) firing at an enemy under
  an event cap of 2, until the room kicks it (one forced resync, then the kick); it rejoins, and its commands after the
  new join's tick up to its kept stamp count `rejoin_overlap`, none forged, the one past it run.
- Tests: f21 `f21_the_never_acker_reaches_the_cap_resyncs_once_then_is_kicked`; f14 x3 (the two structs on `counts`
  with hand-made counters, and the kicked session's `session_end` from f21's run, which also runs f28's -22 half: the
  contract checker's (b)); f35 `f35_each_ended_sessions_counts_are_kept_once_before_its_slot_is_freed` (a Loopback
  server: `Leave`, a closed link, a replacement).
- The B3-B5 tree's check: .gate/u4f/w2-b5-test.log (`-k`: `cargo fmt --all -- --check` exit 1, 12 diffs, applied by
  Edit after the run; `cargo test --locked -p npt-room -p npt-bots -p npt-lab` 16:51:00-16:53:35 UTC exit 0: lab
  main.rs 72 passed, 1 ignored (f21, f14 x3, f35 new), room commands 25 (f39 with its kick case), room 19 (the cap pin
  at 1,600), lib 23, bots 4; then the `--all-features` run, below).

### The counted checks (B6's one, then B10's two)

- **Check 0 (not one of the two: a step failed)**, .gate/u4f/w2-end-check-1.log (`-x`, 17:18 UTC on; `df -h` 8.6G
  free before): the six provenance builds, then `node check.mjs`: fmt, clippy, clippy-wasm32 PASS; **doc FAIL**:
  "public documentation for `EventQueue` links to private item `EventQueue::CAP`" (room/src/events.rs:23, B3's doc
  line linked the crate-private constant; `-D rustdoc::private-intra-doc-links`); its whole output kept by check.mjs
  at .gate/netcode-proof-test/failed-steps/doc.log in this worktree. Cause known; fixed by Edit (the link made plain
  code text), so the two counted checks below are whole new runs. Its other steps all passed, the test step 679.5 s:
  measured 619, bites 716, 0 failed, 0 skipped; refs 1466, marks 81, self 78 of 78, redact 316 items, 0 findings.
- **End check A (B6's counted check and section 6's first)**, .gate/u4f/w2-end-check-a.log (`-x`, 17:33:14-17:45:55
  UTC; free 4,785 MiB at the start, `df -h` 4.7G before; HEAD d1b24c3, diff 5c1ddd52..., untracked a3b69979...): the
  six provenance builds exit 0, then `node check.mjs`: fmt 0.6 s, clippy 3.8 s, clippy-wasm32 1.2 s, doc 4.4 s,
  **test 668.2 s: measured 619 passed (base 552: +67; builder 1's 605: +14), bites 716 (base 632: +84; builder 1's
  701: +15), 0 failed, 0 reported a skip**; deps 1.4 s (143 crate files); wasm 41.8 s (27,000 ticks, 0 mismatching both
  ways; hash-map and std-sin caught as before; warm); refs 1466 tracked files; marks 81; self 78 of 78; redact 10 of
  10, 316 items, 0 findings; **"check: all 11 steps pass"**, exit 0. Provenance: npt-room, npt-client, npt-bots and
  npt-lab each "Compiling ... (<this worktree>/spikes/netcode-proof/crates/...)" x 4. The test step 668.2 s against
  1,207 s (1.5 x E6's 804.8 s): under. The short P2 C62H-BOUND test passed in it.
- **End check B (section 6's second, the same tree)**, .gate/u4f/w2-end-check-b.log (`-x`, 17:46:09-17:59:06 UTC;
  free 4,668 MiB at the start, `df -h` 4.6G before and after; HEAD d1b24c3, diff 5c1ddd52..., untracked a3b69979...:
  equal to A's, no edit between): the six provenance builds exit 0; fmt 0.7 s, clippy 4.7 s, clippy-wasm32 0.9 s, doc
  5.1 s, **test 682.9 s: 619 and 716 passed, 0 failed, 0 skipped**; deps; wasm 41.5 s (27,000 ticks, 0 mismatching);
  refs 1466; marks 81; self 78 of 78; redact 316 items, 0 findings; **"check: all 11 steps pass"**, exit 0.
- **Final SIZE-CHECK** (the notes' counting rule): `git diff --shortstat 5b29296b -- spikes/ docs/` 2,813 insertions
  (20 files) + 2,702 lines in 9 new files (clock.rs 348, carry80.rs 158, trace.rs 556, bite_homes.rs 48, pong.rs 84,
  commands.rs 1,067, c-u4f-cap.mjs 140, c-u4f-r2.mjs 168, c-u4f-runs.mjs 133) = **5,515 added lines**: M's 3,000 x
  1.84, under the 6,000 stop (builder 1 3,753, builder 2 +1,762). Not reached, so line 5's stop record does not apply;
  the build stopped by this builder's working room (B8's fast clock and B9 unbuilt, above).
- Disk: 12G free at this builder's start, 4.6G at its end (the shared target's release, wasm32 and both debug builds,
  cold after the main session's part-clean); never below the 4 GiB floor; nothing cleaned or packed.
- Builder 2's test deltas (by the counts): measured +14 (lab +11: f6 and f28 split +2, f29, f28 clock, f21, f14 x3,
  f35, f18 (ii), f20; bots +2: f36 (ii), f18 (i); room +1: f18 (iii)); bites +15 (those and f2's routing test).
  Changed, +0: f11 (ii), f39 (its kick case), carry 57's room pin, U4r's b5 (the clamp point). Negative cases among the
  new, by 2.10's table: f14 x3, f20, f21, f35 (6). Against 2.10's minimums (measured +119, bites +134, 67 negative):
  measured +67, bites +84, not met: the unbuilt rows (f15, f16 x24, f19, f22, f23, f33 x13, f37, U4F-Q11 (iii) (c)'s
  test) hold most of the rest.

### Builder 2: carries, rulings and what goes to the main session

- Carries moved by builder 2: **57 met** (2.8: the cap measured, 1,600, into the room's default, not provisional;
  plan §4.7's text is the main session's); **58 met for the cap runs** (per-client high-water marks per 30 s bin and
  per burst, c-u4f-cap.mjs, cap.txt; each run's overflow resyncs 0); **50** no code change (1,600 below its 4,328 bound,
  checked by the script); **76** kept and widened (2.5's counts at every session end, the kick path through the
  never-acker, r8 and r9 pass); **86 not met** (c-u4f-r2.mjs exists for 5.3's -20, -47 and the clamp and lag-switch
  rows; -21's forged and fast-clock rows not, their suites unbuilt; the runs are the main session's after the merge);
  **74** as builder 1's (f28's on-time share, the controller's leads). 46 and 80 as builder 1's (the plants of f26, f27
  and D14 now run: B2).
- Rulings: U4F-Q3 (c) built (`second.s.buffer_depth_count` 0 under A, `tick.s.cmd_ran_early_count`); Q4 (b) built
  (the clock suite: no client hash, the never-firing script); Q11 (i) (a) by line 7 (`--brawl-press-every 2`), (ii)
  (a) (the -21 rows at P0, the lag switch at P1), (iii) (c) and (v) (b) not built (the fast clock); Q12 (b) built;
  Q14 (b) (every refusal counter written, the hold clamp uncounted: the named follow-up); Q15 (c) (p19's 400 ms row).
- Deferred Lows (9.2): TE-B1-R2-1 with RR2-B1-1 rode (B2); **RR-CB-2's docs and RR-B1-7 (1)'s `room_tick` loop
  rewrite left** (line 15: they ride only where cheap; the room_tick edits were additions inside the loop, and the
  rewrite would touch C62H-BOUND's reviewed lines: for their next taker).
- To the main session: the cap's value and figures go to the advisor before any -22 run (U4F-Q12); the order notes
  (B3 on a tree with B4's code, B6's checks on B1-B7); the build stop (the fast clock and B9 unbuilt) and a next
  builder; no marks.json, judge, README or existing SERIES.md row changed (D10 below); no manifest change; no wire
  change beyond builder 1's `Pong.server_ns`.

- D10, the bars line: `git diff --numstat 5b29296b -- spikes/netcode-proof/marks spikes/netcode-proof/crates/judge`
  prints only `18 0 spikes/netcode-proof/marks/SERIES.md`: marks.json, README and the judge unchanged; SERIES.md 18
  lines added, 0 removed (every row of U4f's subsection is new; no existing row touched).

### Builder 2's public items (line 11; against 2.13)

- npt-bots (2.13: the cheats' and the never-acker's constructors on `Behaviour`): `Behaviour::with_press_every`
  (replaces builder 1's `brawl_presses`), `never_fires` (2.13's "a no-fire clock-suite script"), `never_acks`,
  `with_points` (the clamp twin), `lag_switch`; **beyond 2.13's words:** `Behaviour::outgoing` (the cheats' hook on
  the endpoint's datagrams, called by the lab at every step) and `Behaviour::take_bursts` (the lag switch's burst
  record, which the lab writes as `event.lag_burst.held_ns`).
- npt-room: none new (`EventQueue::CAP` is crate-private; `Room::set_cmd_timing` only gains its debug assertion).
- npt-client: none new; `unknown_pongs()` stays public, now written as `event.client_counts.unknown_pongs_count`;
  builder 1's `join_tick()` and `lead_now()` are **beyond 2.13's list** (2.13: "the endpoint keeps its `JoinAccepted`
  tick, no new public item"; `lead_now` has no 2.13 line; its consumer is `event.client_counts.lead_now_count`), named
  for the engine-architect.
- npt-lab (binary settings only): `--brawl-press-every`, `--event-cap`, `--connects` (with `--suite clock`),
  `--twin-logs` (the clamp twin), `--lag-hold-ms`, `--lag-every-ms`; the suites `clock`, `never-ack`, `clamp` and
  `lag-switch` read by `run`.

### B6: the run scripts and one counted check (line 20)

- ops/runs/c-u4f-r2.mjs (5.3's pre-runs, on c-u4r-r2.mjs's model; its header states its order, reading and limits):
  rows p5, p6 (-20 A and B, P3, the controller, `--brawl-press-every 2`), p10, p11 (-47's two bites, the clock suite
  with one round of 48 connects), p12 (redundancy-1 under B), p21 (the clock suite clean); each row's scenario, then
  `contract`, then the judge in 5.4's form (`--only PM-0001-2,PM-0001-20`, or `--only PM-0001-47`); exports its
  helpers (`sh`, `sha256`, `floor`, `build`, `step`, `committed`) for c-u4f-runs.mjs.
- ops/runs/c-u4f-runs.mjs (the judged sets 47, 22 and 20, on c-u4r-runs.mjs's model, ADV-CBH-2's hardenings:
  binaries built in the call, marks.json's and npt-judge's sha256s re-checked before each judge call with the
  spike's tree and status, the full HEAD as --commit, a 10 s warm-up; every set judged in two calls, measured/ then
  bites/, STOP_NOTE at runs.txt's head; -22's 5.2 load report as a third call, `--only PM-0001-2`, never counted;
  CARGO_TARGET_DIR refused). -21's sets join with its suites (B7-B9).
- **Each npt-lab command line by hand on a short setting (section 6 item 5)**: .gate/u4f/w2-hand.log (`-x -k`,
  17:09:40-17:14:14 UTC, every command exit 0), the release builds through `cargo run --release --locked -q`, each at
  `--warmup-s 4 --measured-s 2` (48 clients, seed 1), outputs under .gate/u4f/w2-hand/: c-u4f-r2.mjs's forms p5, p6
  (-20 A, B), p21 (clock, 48 connects), p2, p15 (clamp A, B), p4, p18, p19 (lag switch A, B, A with 400 ms), and the
  bites build's p10, p11 (-47's two), p12 (redundancy-1), p8 (refuse-not-clamp), p13 (no-catchup-cap);
  c-u4f-runs.mjs's forms 47 (1,000 connects, role measured), 22 (never-ack, role measured) and its bite
  (PM-0001-22/unbounded), 20 (B, role measured); `contract` on p5 (PASS); the judge in 5.4's form on p5 (`--only
  PM-0001-2,PM-0001-20`: -20 FAIL only by length, "10,000 scripted presses = 1446"), p21 (`--only PM-0001-47`: FAIL by
  length, "1,000 connects ... = 48"), s22 (`--only PM-0001-22`: NOT-RUN, 1 of 3 runs, its two checks **pass**: headroom
  min 0, ack_starved 2, at the room's new cap 1,600 in 6 s; and 5.2's `--only PM-0001-2`: FAIL, "a connected client on
  every soldier = 47", the designed drop, never counted) and p2 (`--only PM-0001-2,PM-0001-21`: FAIL by length, "1,000
  points beyond reach ... = 177"). The scripts themselves (`node ...`) need a committed tree and the main folder, so
  they were not run whole; their command lines were.
- **Section 6 item 4 (P33; F8)**: `cargo test --locked -p npt-server --all-features` on this tree (.gate/u4f/
  w2-times.log, 17:14:15-17:15:17 UTC): exit 0, gap 6, loopback 6, pump 20, schedule 17 passed. The moved test, -1
  npt-server and +1 npt-lab, by `--list`: npt-server `--all-features` lists 6 + 6 + 20 + 17 = 49 and no
  `catalogue_5_owner_to_all_lives_in_npt_room_alone_and_no_id_is_in_two_lists`; npt-lab's `--test bite_homes --list`
  names it, "1 test" (w2-times.log:186-205).
- **The time rule, each new or changed test alone** (w2-times.log, `-x -k`, "finished in" lines): the reference
  `c62hb_p2_short_every_measured_lost_tick_is_explained` 11.13 s, so the line is 22.26 s; f6 A 11.36, f6 B 11.47, f28 A
  11.31, f28 B 11.34, f40 11.29, f29 9.34, f28 clock 9.42, f21 9.96, f14 kicked 9.91, f14 room 0.00, f14 pump 0.00, f35
  0.01, f18 (ii) 14.76 (its two runs), f20 9.85, f2 routing (bites) 0.00; bots f36 (ii) 0.01, f18 (i) 0.00; room f18
  (iii) 0.01, f39 0.29, f11 0.00 s. Every one under the line (R1's split holds: f6 and f28 near 1 times the reference).
- An order note: B7's code (below) was written while B5's tests ran, before B6's hand runs and counted check, so
  those run on B1-B7 (its code acts only under `--suite clamp`, and the clamp script's new point only under
  `--behaviour clamp`, which no other test or run names). Recorded as a deviation from line 4's order.

### B7: -21's clamp suite (line 21; 2.4's last row, 2.6, 2.12; line 12 of the ruling)

- npt-bots: the clamp script's point (`FAR_CM`) moves from 32,000 cm (outside the map: kind 6 refuses it, NA-8) to
  11,000 cm in x and y: inside ±12,800, beyond the 6,000 cm reach from anywhere near the fight; it holds fire with no
  press edge and stands still (line 12: the clean sim refuses none of its commands). `Behaviour::with_points` (the
  clamp twin: each listed sequence's aim replaced, in every `Input` carrying it, through `outgoing`).
- npt-lab: `--suite clamp`: slot 0 runs the clamp script, every other client the run's `--behaviour` (the run
  scripts pass `steady`, an open-loop script, 2.12); every run's room log writes each clamp report as
  `event.clamp_report.x_um`, `.y_um`, `.z_um` (fields `slot`, `seq`; row :324), from `Room::reports()`; a clamp run
  (not its twin) writes one lab-log `clamp_kind` (the sim has one weapon kind, hitscan): `sent_count` the cheat's
  sequences run with a point beyond reach of its fire origin, `counted_count` of those with any sim refusal record
  whatever its kind (line 12; refuse-not-clamp records Cooldown), `rerun_reports_count` (SA-10's reruns, a new row).
  The twin: `--twin <run> --role twin --twin-logs <the clamp run's folder>` (the clamp suite only) reads that run's
  slot-0 clamp reports into the cheat's `with_points` (µm / 10,000 = grid cm; a rerun's first).
- Why the twin holds: npt-sim reads a command's aim only for its shot (world.rs:1298-1316, :1450: `clamp`), so a
  command that did not shoot leaves no trace of its aim, and one that shot gets the clamp run's reported point, which
  lies within the reach and is used as given.
- Tests (+3): f18 (i) bots `f18_the_clamp_scripts_points_lie_beyond_reach_and_inside_the_map`; f18 (ii) lab
  `f18_a_clamp_run_and_its_twin_give_equal_hashes_and_count_no_refusal` (the clamp run and its twin, 16 clients P0
  2 s + 4 s, option A: equal per-tick hashes on the same ticks; `sent_count` above 0, `counted_count` 0, reruns 0, slot
  0's reports above 0; f28's clamp half: no name without its row); f18 (iii) room
  `f18_under_option_b_a_rerun_clamp_command_gives_two_reports_for_one_sequence`.
- c-u4f-r2.mjs: 5.3's p2 (clamp A), p8 (refuse-not-clamp, A) and p15 (clamp B) rows.
- SIZE-CHECK after B7's code (before its test run): 2,624 insertions (19 files: events.rs joined) + 2,696 lines in 9
  new files = **5,320**.
- Its check, .gate/u4f/w2-b7-test.log (`-k`, 16:58:25-17:02:45 UTC): fmt --check exit 1 (9 diffs, applied by Edit);
  both clippy runs exit 101 on one finding, `clippy::type_complexity` on the two count tables (fixed: a `Field<T>`
  alias); the measured test run exit 101 at bots/tests/behaviour.rs:230, U4r's b5 test's pin of the clamp script's
  32,000 cm point (a pin this block re-pins, P25: the point moved by design, NA-8; re-pinned to 11,000 with a line
  saying why); the `--all-features` run exit 0: lab main.rs 84 passed, 1 ignored (f18 (ii) among them), commands 38
  (f18 (iii)), room 27, lib 23, bite_homes 1. The fixes are re-checked in B8's first run (below).

### B8: -21's lag switch and fast clock (line 21)

- Lag switch, npt-bots: `Behaviour::lag_switch(hold_ns, every_ns)` and `take_bursts()`; `outgoing` takes the step's
  host time: in each hold window (the first `hold_ns` of every `every_ns` from its first send, the first window one
  period in) the client's `Input` datagrams are held (`Vec::extract_if`), then sent in one burst ahead of the next
  step's own, each burst's hold kept (`event.lag_burst.held_ns`, lab log, field `slot`; row :170).
- Lag switch, npt-lab: `--suite lag-switch` makes slot 0 the lag switch (every other client the run's
  `--behaviour`); `--lag-hold-ms` (200) and `--lag-every-ms` (6,000), in the header as `lag_hold_ns` and
  `lag_every_ns` (a new row); the room log writes on every room tick record `tick.v.lagswitch_cmds_count` (row :33,
  `ClientSlot::commands_run`), `tick.v.lagswitch_excess_move_um` (row :34: the lag soldier's movement this tick, µm,
  less `CMD_CATCHUP` x 600 cm/s / TICK_HZ, the top speed a copy of npt-sim's private `Scene::move_speed`) and
  `tick.v.lagswitch_move_apart` (A1: 1 when it is dead at the tick's start or end, its movement then 0; a new row);
  `logs::moved_um` the movement rule (health 0 at either record: none).
- f20 `f20_the_lag_switch_runs_at_most_the_cap_a_tick_and_moves_within_it` (+1): `moved_um` on hand-made records (a
  walk, a respawn, a death) and a short lag-switch run (16 clients, P1, 2 s + 6 s, A, a hold every 2 s): at least 3
  bursts each held at least 200 ms, the lag client's commands a tick at most 8, its excess at most 0, f28's names.
- c-u4f-r2.mjs: 5.3's p4, p13 (no-catchup-cap), p18 and p19 (the 400 ms hold) rows.
- The check of B7's fixes and B8's lag switch: .gate/u4f/w2-b8a-test.log (`-k`, 17:03:15 UTC on): fmt --check exit 1
  (2 diffs, applied by Edit after its run started: formatting only); clippy host and `--all-features` exit 0; the
  measured tests (room, bots, lab) exit 0: lab main.rs 74 passed, 1 ignored (f18 (ii), f20 among them), commands 26,
  room 19, lib 23, bots lib 5 (f18 (i)), behaviour 5 (b5 re-pinned); the `--all-features` run below.
- SIZE-CHECK after B8's lag switch: `git diff --shortstat 5b29296b -- spikes/ docs/` 2,813 insertions (20 files:
  bots/tests/behaviour.rs joined) + 2,702 lines in 9 new files = **5,515** (M's 3,000; the brief's 5,150; the stop
  6,000).
- **The fast clock is not built, and B9 (the forged suite) is not started: this builder's working room.** The added
  lines stay under 6,000 (line 5's stop is not reached), so this is a stop of the build by the builder, recorded as
  builder 1's was, not the size stop: the rest goes to a next builder from these notes. Not built: the 1.5x host
  clock and its stamp rule (line 13, as F1 amended), the episodes (`--fast-clock-episode-s`,
  `params.fast_clock_episode_ns`, `event.fast_clock.excess_cmds_count` and `.excess_move_um` with each episode's
  kind-1 and kind-2 refusals), the honest reference and U4F-Q11 (iii) (c)'s test, f19, f37; 5.3's p3, p9, p16, p20;
  the 11 forgers, `--forge-every`, the twins and the twin's skip in the lab, `event.forged_kind.*`, f15, f16 (24), f22,
  f23, f33 (13); 5.3's p1, p7, p14, p17; -21's set in c-u4f-runs.mjs (its measured runs, twins, the clamp twins'
  `--twin-logs`, and its 13 bite runs). What they can build on: `Behaviour::outgoing` (the cheats' hook on the
  endpoint's datagrams, with the host time), `logs::moved_um` (A1's movement rule), the clamp twin's points route,
  `--event-cap`, the suite switch in `run`, and the room's checks and records (B1-B4 of builder 1).

## Builder 3

Task: U4f's third builder (general-purpose, opus, effort xhigh): finish U4f's build (clock and commands) from builder 2's
partial build, by the advisor's 22 instruction lines (advisor-u4f-build1.md:162-246, sha256 da98295e...), u4f-brief.md
rev 3.3 (sha256 135325a0...) and the main session's prompt (which wins where they differ). Commit nothing, push nothing.

### Start (done)

- Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a0825a5a2491eb175 (made at origin/main 22c6afa0).
  `git merge --ff-only 07289a4ec42bdc9adf2d236642cf0ea5148365fc`: exit 128 ("Diverging branches can't be
  fast-forwarded"); `git switch -c u4f-build3 07289a4ec42bdc9adf2d236642cf0ea5148365fc`: "Switched to a new branch
  'u4f-build3'"; `git log -1 --format=%H`: 07289a4ec42bdc9adf2d236642cf0ea5148365fc.
- sha256: wip/u4f-build.patch 3d341c58e64b32442f3f70a02ef6764e84efc685f84512dae09c4f9220aff155, the ruling
  da98295e0d0eae7c12e46f0a282dce3c452f75dc74346c3f55aec26baa84f7cb, the brief
  135325a0bb524104bba9144b366b9ec52525851f5db140b1aca93ba4681a502f, the runner
  70eb3da5b8ce9a8e2eda582536eeab93f96fb7fbe98e2a74a28e0492ccf6ddae: all four match the prompt. `git apply --check`
  exit 0, `git apply` exit 0 (the one git write).
- Line 2's checks: `git diff --stat 5b29296b -- spikes/ docs/`: 20 files, 2,813 insertions, 302 deletions; `git status
  --short`: those 20 modified and 9 untracked new files, `wc -l`: clock.rs 348, carry80.rs 158, trace.rs 556,
  bite_homes.rs 48, pong.rs 84, commands.rs 1,067, c-u4f-cap.mjs 140, c-u4f-r2.mjs 168, c-u4f-runs.mjs 133 = 2,702;
  **5,515 added**. Equal to the prompt and to builder 2's notes (:529-531): no stop.
- Notes read whole (:1-690). Builders 1's and 2's worktrees untouched (backups).
- `df -h /home/user` at start: 252G size, 27G used, 12G avail (70 %).
- Plan by the prompt's item 5: B8f, the fast clock (one block); then B9a, B9b, ... of at most about 400 added lines
  each; SIZE-CHECK after each; at the first block end past 6,000: build nothing more, end checks twice, the stop record.

### B8f: the fast clock (in progress; no plant in place; nothing run yet)

- Design (read from the code): the fast client is slot 0 of `--suite fast-clock`; a fixed-lead endpoint (`with_lead`,
  so no controller's due rule refuses its commands; it sends no `Ping`: a deviation from 2.6's "its pings come at its
  steps", `ping_limited` is report-only), a command at every step (`every_step`), its host clock rate 1.0 to the
  measured start, 1.5 after (the lab's host/sim mapping per client); each new sequence's stamp set in
  `Behaviour::outgoing` by F1's rule (`fast_stamp`), every copy keeping its sequence's stamp (no kind 3), the view tick
  clamped to the stamp; an older copy that would break the codec's 255 rule left out (`fast_left_out`, 0 expected).
  The walk: U4F-Q11 (iii) (c)'s collision-free lane y -1,050 cm, x 2,600 to 3,600 cm (east of the fight vehicles'
  sweep, r at most 17.71 m; south of building 0; inside the outer loops from 48.3 m; no shot segment of a steady client
  reaches it: shooters and aim points lie within about 16 m), approached in the warm-up, at [±127, 0] (20 cm a tick).
- Done (edits, bots/src/lib.rs): consts LANE_Y_CM, LANE_X_CM, LANE_BAND_CM, STAMPS_KEPT; `Kind::Lane`; `Fast` state,
  `later`, `fast_stamp`; `Behaviour::fast_clock(scene, slot, seed, bound)`, `fast_left_out()`, `lane()`; `outgoing`'s
  stamp rewrite and encode-or-leave-out loop; `command()`'s `Kind::Lane` walk; tests
  `f19_the_fast_clocks_stamps_never_fall_and_every_tenth_lies_past_the_bound` and
  `q11_iii_c_the_fast_clocks_lane_moves_full_walking_speed_every_live_tick` (World, 10 s + 610 s, every tick after the
  warm-up 20 cm).
- Next exact step: the lab (scenario.rs): Settings `fast_episode_ns`, `fast_idle_ns`; parse `--fast-clock-episode-s`
  (60), `--fast-clock-idle-s` (0); params `fast_clock_episode_ns`, `fast_clock_idle_ns`; Client `fast_from` with
  `host()`/`sim()`; the fast slot's endpoint and behaviour; idle skip; `episodes()` and the `FastClock` tally writing
  lab-log `fast_clock` events (excess_cmds_count, excess_move_um, lead_refused_count, not_newer_count,
  move_apart_count, field slot); tests f37, f19 (lab half, hand-made records + a short run), f28 fast-clock A and B;
  SERIES.md's two new rows; c-u4f-r2.mjs's p3, p9, p16, p20; then build and test through the runner.
- Done since (edits): scenario.rs Settings `fast_episode_ns`/`fast_idle_ns`, parse and its usage line, params;
  `Client::fast_from`, `host()`, `sim()`; `episodes()`; `FastClock` with `tick()`; the fast slot's endpoint
  (`with_lead`) and behaviour; the tally call in the tick loop; the idle skip; the host mapping at the three arrival
  sites and the next step; logs.rs `MOVE_CM_S` pub(crate); tests f37, f19 (lab), f28 fast-clock A and B (shared
  `fast_short`); SERIES.md 2 new rows at the end of U4f's subsection; c-u4f-r2.mjs FAST, p3, p9, p16, p20 and its
  header comment. Next: `df`, then the runner: fmt --check, then `cargo test --locked -p npt-bots -p npt-lab`.
- w3-b8f-test1.log (`-k`, 18:26:31-18:27:19 UTC; free 11,859 MiB): fmt --check exit 1 (6 diffs, applied by Edit after
  it); the bots lib test exit 101, E0282 in the lane test (`prev`'s type: fixed, `None::<[i32; 3]>`); npt-lab compiles,
  f37 passed (exit 0). Next: the bots lib tests f19_ and q11_, then the lab's f19_ and f28 fast-clock tests.
- **B8f's check** (w3-b8f-test2.log, `-k`, 18:27:52-18:28:16 UTC): fmt --check exit 0; `cargo test --locked -p
  npt-bots --lib fast_clock` exit 0, 2 passed (f19's stamps; q11 (iii) (c)'s lane: every tick of 610 s after the 10 s
  warm-up moved exactly 20 cm), 12.21 s for both; `cargo test --locked -p npt-lab --bin npt-lab fast_clock` exit 0, 4
  passed (f37, f19, f28 fast-clock A and B), 7.62 s.
- Figures (f19's short run, A, 2 s episodes, P0, 16 clients): per episode excess commands -36 and -36, excess movement
  -7,387,637 and -7,200,000 µm, lead-bound refusals 24 and 24, not-newer 29 and 30, apart ticks 0. The fast client
  runs about 0.6 commands a tick under F1's rule: each 10th stamp's jump of 3 past the cap leaves 2 to 3 stamp values
  never used, so the clean excess sits far below 0 (about -700 a 60 s episode by the same rate). Under
  not-older-than the held equal stamps (about 15 a second here) run as well: about 1.1 a tick, so a 60 s episode's
  excess is about +180 (p9 fails it), while a 2 s episode's would stay near -6 (the bound's 12 outweighs it): p9 is
  read at 5.3's 60 s episode, not at f19's scale. Under B the fast client's lead (up to 12) exceeds the buffer's 8:
  4 kind-10 refusals in the B run (expected, reported).
- **SIZE-CHECK after B8f:** 3,273 insertions (20 files) + 2,709 lines in 9 new files (c-u4f-r2.mjs 175) = **5,982**
  (under 6,000: B9a next, one block of at most about 400 lines, after which the stop applies).
- clippy (w3-b8f-clippy.log, 18:29:03-18:29:14 UTC): `cargo clippy --locked -p npt-bots -p npt-lab --all-targets -- -D
  warnings` exit 0, and with `--all-features` exit 0. **B8f done.**

### B9a: the room-whole forgers, kinds 1, 2, 4, 5 and 6, with f15's five cases (in progress; no plant)

- Plan: npt-bots `Behaviour::forger(kind, every, bound)` (`None` for a kind this block does not build: 3, 7 to 11),
  forging in `outgoing` the newest command of every `every`-th new sequence, only in the first `Input` that carries it
  (2.6's table: the endpoint stores the honest command, so later copies are honest, repeats of a refused sequence);
  `take_forged()` for the lab's attribution (B9b); f15's five cases in bots (decoded: only that check's field changed,
  the rule broken, every other copy and the acknowledgement equal, every forged `Input` decoding). Not in B9a: kinds 3,
  7 to 11, the lab's suite, `--forge-every`, the twins, the attribution, f16, f22, f23, f33.
- Edits done (bots/src/lib.rs): `Forge` state, `forged()`, `Behaviour::forger`, `take_forged`, the forger's part of
  `outgoing`, the helper `f15(kind)` and its five tests. Next: the runner, fmt --check and `cargo test -p npt-bots
  --lib f15_`.
- w3-b9a-test1.log (`-k`, 18:32:02-18:32:04 UTC): fmt --check exit 1 (8 diffs, applied by Edit after it); `cargo test
  --locked -p npt-bots --lib f15_` exit 0, 5 passed (0.01 s). Next: fmt --check, clippy both builds, bots tests whole.
- w3-b9a-test2.log (`-k`, 18:32:38-18:32:55 UTC): fmt exit 0; both clippy runs exit 101 on one finding,
  `clippy::manual_is_multiple_of` at `forged()`'s `n % 2 == 0` (x3; fixed by Edit: `n.is_multiple_of(2)` once, as
  `even`); `cargo test --locked -p npt-bots` exit 0: lib 12 passed, 1 ignored (12.92 s), behaviour 5. Next: fmt and
  both clippy runs again, then SIZE-CHECK.
- w3-b9a-test3.log (`-k`, 18:33:23-18:33:28 UTC): fmt exit 0; clippy host and `--all-features` exit 0; f15's five
  exit 0. **B9a done.**
- **SIZE-CHECK after B9a:** 3,503 insertions (20 files) + 2,709 lines in 9 new files = **6,212: past 6,000.** Line
  5's stop: nothing more is built; next, section 6's end checks twice on this tree (`-x`, `df` before each), then the
  stop record.
- End check A starting: `df -h /home/user` 11G avail (73 %); one `-x` runner call, log .gate/u4f/w3-end-check-a.log:
  the six provenance builds (as w2-end-check-a.log's), then `node check.mjs`.
- **That run stopped by me at 18:35 UTC** (TaskStop, during its third provenance build; the two `--no-run` builds
  exit 0; no check.mjs step ran; not one of the two counted checks): reading B8f's docs I found
  `Behaviour::fast_clock`'s doc (public) linking the private `fast_stamp`, which check.mjs's doc step
  (`RUSTDOCFLAGS=-D warnings`, check.mjs:2032) refuses as builder 2's check 0 showed. Fixed by Edit (plain code text);
  no other public item of mine links a private one (Grep). No process left (`ps`). Next: `node check.mjs --only doc`
  (`-x`), then end checks A and B anew.
- w3-doc.log (`-x`, 18:35:50-18:36:20 UTC): `node check.mjs --only fmt,doc`: fmt PASS, doc PASS (both builds, host
  and wasm32), exit 0. `df -h /home/user` before end check A: 6.4G avail (84 %). End check A (w3-end-check-a2.log)
  starting.
- **End check A** (.gate/u4f/w3-end-check-a2.log, `-x`, 18:36:2x-18:50:00 UTC; free 6,503 MiB at the start; HEAD
  07289a4, diff 54cb6faa..., untracked e8dc3663...; the run spanned the org's spend-limit cut at 18:47 and ended on
  its own): the six provenance builds exit 0; fmt 0.6 s, clippy 10.0 s, clippy-wasm32 1.8 s, doc 4.0 s, **test 692.1
  s: measured 630 passed (base 552: +78; builder 2's 619: +11), bites 727 (base 632: +95; builder 2's 716: +11), 0
  failed, 0 reported a skip**; deps 3.0 s (143 crate files); wasm 41.2 s (27,000 ticks, 0 mismatching both ways;
  hash-map and std-sin caught as before); refs 1468; marks 81; self 78 of 78; redact 10 of 10, 316 items, 0 findings;
  **"check: all 11 steps pass"**, exit 0. Provenance: npt-client, npt-room, npt-bots, npt-lab "Compiling ... (<this
  worktree>/spikes/netcode-proof/crates/...)" in both `--no-run` builds. The test step 692.1 s against 1,207 s: under.
  The main session confirmed the tree's two hashes unchanged after it (its message). `df -h /home/user` before end
  check B: 5.3G avail (87 %).
- **End check B** (.gate/u4f/w3-end-check-b.log, `-x`, 22:4x-22:59:25 UTC; free 5,349 MiB at the start; HEAD 07289a4,
  diff 54cb6faa..., untracked e8dc3663...: equal to A's, no edit between): the six provenance builds exit 0; fmt 1.1
  s, clippy 37.5 s, clippy-wasm32 6.1 s, doc 17.7 s, **test 685.6 s: 630 and 727 passed, 0 failed, 0 skipped**; deps
  3.8 s (143 crate files); wasm 43.9 s (27,000 ticks, 0 mismatching both ways); refs 1468; marks 81; self 78 of 78;
  redact 316 items, 0 findings; **"check: all 11 steps pass"**, exit 0.
- Builder 3's test deltas: measured +11 and bites +11 (the same tests in both builds): bots +7 (f19's stamps, U4F-Q11
  (iii) (c)'s lane test, f15 x5), lab +4 (f37, f19, f28 fast-clock A and B). Negative cases among them: f15 x5, f19
  (lab) = 6 (builder 2's 6 before: 12 of 2.10's 67 by its own count, plus builder 1's).
- **The time rule, each new test alone** (.gate/u4f/w3-times.log, `-x -k`, 22:59-23:00:51 UTC, "finished in"): the
  reference `c62hb_p2_short_every_measured_lost_tick_is_explained` 11.41 s, so the line is 22.82 s; lab f19 7.19, f28
  fast-clock A 6.99, B 7.17, f37 0.00; bots q11 (iii) (c)'s lane 12.18, f19's stamps 0.00, f15 kinds 1, 2, 4, 5, 6 0.00
  each. Every one under the line; the test step 692.1 s and 685.6 s, under 1,207 s.
- D10, the bars line: `git diff --numstat 5b29296b -- spikes/netcode-proof/marks spikes/netcode-proof/crates/judge`
  prints only `20 0 spikes/netcode-proof/marks/SERIES.md` (builder 3: +2 rows, both new, in U4f's subsection);
  marks.json, README and the judge unchanged. crates/sim: world.rs 48 added, 3 removed (unchanged by builder 3).
- **Builder 3 returns here** (the size stop, line 5): nothing more built after B9a.

### The stop record (line 5; his size stop past 6,000)

- **The count:** 6,212 added lines (3,503 insertions in 20 files by `git diff --shortstat 5b29296b -- spikes/ docs/`,
  plus 2,709 lines in 9 new files), at B9a's end: M's 3,000 x 2.07; the brief's 5,150 x 1.21; builder 3 +697 (B8f
  467, B9a 230).
- **Each component against SIZE-CHECK's table (brief :927-939),** by `git diff --numstat` and the new files' lines
  (the rev 3.1, 3.2 and 3.3 rows' items sit inside the crates they touched):

  | Component | Estimate | Now | What it holds |
  |---|---|---|---|
  | npt-client | about 700 | 1,421 | lib.rs 349, tests/bites.rs 10, clock.rs 348, carry80.rs 158, trace.rs 556: all built |
  | npt-room | about 950 | 1,956 | input.rs 431, room.rs 401, lib.rs 13, events.rs 9, tests bites 9, room 26, commands.rs 1,067: all built |
  | npt-bots | about 650 | 702 | lib.rs 700, tests/behaviour.rs 2: the never-acker, clamp, lag switch, fast clock, 5 of the 11 forgers |
  | npt-lab | about 1,400 | 1,453 | scenario.rs 1,263, logs.rs 142, tests/bite_homes.rs 48: every suite but `forged` |
  | npt-sim | about 40 | 48 | world.rs's two bite sites |
  | SERIES.md | about 25 | 20 | new rows only (U4f's subsection) |
  | ops/runs/c-u4f-*.mjs | about 600 | 448 | cap 140, r2 175, runs 133: -21's set and the forged rows not yet |
  | proto, catalogue, server (rev 3.3's U4F-Q6 (b) and P33) | in the 300 | 164 | msg.rs 6, common 1, pong.rs 84, the catalogue 1, pump.rs 2, loopback.rs 8, pump tests 62 |
  | **Total** | **about 5,150** | **6,212** | |

- **Built, by mark and suite:** -47 (the clock suite, ClockSync, its two bites, f1-f5, f29, f31, p10, p11, p21);
  -20 (option A and B, the controller, `--brawl-press-every`, every -20 series, f6, f28's -20 halves, f34, f36, f40,
  redundancy-1, p5, p6, p12); -22 (the never-acker, its suite and series, 2.5's counts at every session end, f14, f21,
  f35, the cap measured at 1,600: U4F-Q12 with the advisor); -21's room and sim side whole (kinds 1-6, 8 and 10 with
  their counters and records, option A's run path, the 13 room and sim bites' unit proofs, f7-f13, f17, f38, f39);
  -21's clamp suite (f18, p2, p8, p15), lag-switch suite (f20, p4, p13, p18, p19) and fast-clock suite (B8f: f19 in
  bots and lab, f37, f28 A and B, U4F-Q11 (iii) (c)'s test, p3, p9, p16, p20); and B9a, npt-bots' forgers of kinds 1,
  2, 4, 5 and 6 with f15's five cases.
- **Not built (the forged suite, -21's last part):** the forgers of kinds 3, 7, 8, 9, 10 and 11 (and f15's six
  cases); the lab's `--suite forged` (11 forger slots, 37 honest open-loop clients) and `--forge-every` with its header
  param and row; the attribution by 2.4 item 6's per-kind rules and `event.forged_kind.sent_count`,
  `.unrejected_count`; the twins and the twin's skip in the lab (`ClientEndpoint::skip_command` exists, unused by the
  lab), kind 10's and kind 3's twin rows; f16 (24), f22 (2), f23, f33 (13); 5.3's p1, p7, p14, p17; -21's set in
  c-u4f-runs.mjs (its 24 measured runs, 12 twins with the clamp twins' `--twin-logs`, its 13 bite runs, two judge
  calls). Also not done: f28's clamp and lag-switch halves under option B (builder 2 folded each A half into f18 (ii)
  and f20: line 6 asks one per suite and option).
- **The estimate for the rest:** npt-bots about 250 (six forgers, f15's six cases); npt-lab about 950 to 1,100 (the
  suite, the attribution, the twins, f16's 24 through one helper, f22, f23, f33's 13 on hand-made records); SERIES.md
  about 5; c-u4f-r2.mjs about 10; c-u4f-runs.mjs about 150; f28's two B halves about 40: about 1,400 to 1,550, so U4f
  whole at about 7,600 to 7,800 added lines.

## Fix pass (round 1): builder 4

Task: U4f's fourth builder, the fix pass after review round 1: FP1 to FP19 of the advisor's ruling
(advisor-u4f-r1.md:156-273), in its order, each with its named acceptance check; the end checks (:287-299, brief
section 6). Commit nothing, push nothing.

**Where these notes live (a departure, for the main session):** the prompt says to append to the main folder's
u4f-build.md by its absolute path. The harness refused that Edit ("This agent is isolated in the worktree ... Edit the
worktree copy of this file instead of the shared-checkout path"). I did not route around it with a shell write (the
prompt's Write/Edit-only rule). So this block is in W's copy,
/home/user/Shamble-Frontlines/.claude/worktrees/agent-a5b1297a320917dfb/.checkpoints/netcode-proof-test/u4f-build.md,
which was byte-equal (`cmp`) to the main folder's file before this block (80,449 bytes, :1-866): the block can be
copied over whole. It is outside spikes/netcode-proof, so the runner's tree hashes do not see it.

### Start (done, 14:58 UTC)

- Worktree W: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a5b1297a320917dfb. `git merge --ff-only
  b25b96a6...`: fast-forwarded (b763ba3..b25b96a, exit 0); `git switch -c u4f-fix1 b25b96a6...`: "Switched to a new
  branch 'u4f-fix1'"; `git log -1 --format=%H`: b25b96a63f1f06aff73f4b1e6ab767e8f0a68df5 (LANE).
- wip/u4f-build.patch sha256 74ae26aa29c229d64e884c8fa2b88c45678cb3c919448b6a7ae20c7b747eadb1 (matches); `git apply
  --check` exit 0; `git apply` exit 0 (the one git write).
- Runner hashes before any edit (.gate/u4f/w4-start.log:3): diff 54cb6faaf0dac1ce..., untracked e8dc3663a8f3df5e...:
  equal to review-pre.log:3. Free 11,748 MiB.
- Read whole: the ruling advisor-u4f-r1.md (407 lines) and u4f-review-r1-full.md (332 lines); advisor-u4f-build1.md
  :155-246; u4f-run.sh; the brief's sections 1, 2.10 and 6.
- Size at start: 3,504 insertions (20 files, `git diff --shortstat b25b96a6 -- spikes/ docs/`) + 2,709 lines in 9 new
  files = 6,213 (as the prompt). The guard: past 500 added in the pass (about 6,713), stop at that item's end.
- `df -h /home/user` at start: 12G avail (71 %).

### Plan

FP1 to FP19 in the ruling's order, one item at a time: read the sites; where the ruling asks, a fails-first runner
call on the reviewed code (its log kept: the new test added first, run, then the fix); the fix by Edit; its
acceptance run; fmt --check and clippy on the crates touched; then its line here and SIZE-CHECK. Plants (FP4, FP15)
by Edit in place, one `-x -k` call, restored by Edit, sha256 checked against the pre-plant hash. Then the end checks
on one tree: check.mjs twice (`df` first, one `-x` call in w3-end-check-a2.log's form), each new test alone, item 4,
item 5's hand lines and `node --check`.

### Items

- **FP1 (RR-U4F-1, RR-U4F-3), done 15:01 UTC.** Files: client/src/lib.rs (`step()` encodes the `Input` by value
  through one closure, rebuilt only on the error path, no `input.clone()`; the comment states the codec's two rules;
  `drop_unencodable` keeps the longest newest run that encodes whole by trial encodes, the rest counted in `left_out`;
  `twin_left_out`'s doc names both causes), client/tests/carry80.rs (new
  `f25_ii_an_older_command_past_the_codecs_tick_step_is_left_out`). Fails first: .gate/u4f/w4-fp1-first.log (15:00
  UTC, exit 101): "assertion `left == right` failed: Input { ack_tick: 405, ack_groups: 1, commands: [] }, left: (405,
  0), right: (405, 1)". Acceptance: .gate/u4f/w4-fp1.log (15:01 UTC, exit 0): fmt --check 0; carry80 "5 passed; 0
  failed" (f25 and f25 (ii) ok); clippy `-D warnings` both builds 0. Added in the item: +23 (lib.rs +7 net, carry80
  +16). SIZE-CHECK: 3,511 insertions + 2,725 new-file lines = **6,236** (pass +23).
- **FP2 (NC-U4F-1), done 15:06 UTC.** Files: client/src/lib.rs (`plan()`'s arm `Some(t) if dist(goal, t) >
  Limits::REDUNDANCY_CAP as i64 => goal` before the 2-a-step arm; `plan()`'s, `with_clock`'s and `stamps_skipped`'s
  docs: a goal past the cap, a clock reset or a host stall, moves the target at once), client/tests/trace.rs (the
  trace's `run` gains `stall` (no input or step for a span, then the host makes the steps due at once:
  `host = next_due_ns.max(host)`, unchanged for every other run, whose next due time is always later) and
  `check_from` (the lead and each stamp's first arrival checked from the n-th step at or after a host time, by
  sequence: the n-th command given is sequence n); f32 (ii) asserts, after its 12 s jump, `stamps_skipped() > 0`, the
  least lead from 2 s plus 10 steps after it at least 1, and every stamp first arriving from then on before its tick;
  a stall run in the same test (P0, 1 s at 10 s, seed 32) asserts skipped above 0 and every stamp from the third step
  after the stall on time; +0 tests). Fails first: .gate/u4f/w4-fp2-first.log (15:05 UTC, exit 101): "f32 (ii): most
  ahead 7, resets 20, skipped 0, least lead Some(-287), late 340 of 340; stall: skipped 0, late 24 of 296" (both cases
  fail; both runs made before the asserts). Acceptance: .gate/u4f/w4-fp2b.log (15:06 UTC, exit 0): fmt 0; trace "7
  passed; 0 failed": "f32 (ii): ... skipped 339, least lead Some(3), late 0 of 170; stall: skipped 29, late 0 of 269";
  f36 "skipped 0"; f32 (i) passed (asserts skipped 0 over its 1,000 cases); clippy both builds 0. f5 asserts no
  `stamps_skipped` (the seat read that it did): FP3 prints it per profile. w4-fp2.log (15:05 UTC): the same trace
  figures and `cargo test -p npt-client` whole exit 0, fmt exit 1 there (two diffs, rewritten shorter by Edit).
  Added in the item: +64 (lib.rs +6, trace.rs +58). SIZE-CHECK: 3,517 + 2,783 = **6,300** (pass +87).
- **FP3 (TE-U4F-1), done 15:07 UTC.** File: client/tests/trace.rs (f5: late = arrived after its tick, counted apart
  from missing; late at most 1 % on all six profiles; the combined share (not on time) at most 1 % on the five
  without a burst; the line prints late, missing, not on time and `stamps_skipped` per profile; the 0.03 literal gone
  (Grep `0\.03|budget`: 0); the doc says what the code checks). Acceptance: .gate/u4f/w4-fp3b.log (15:07 UTC, exit
  0): fmt 0, f5 passed, clippy both builds 0; per profile (late, missing, not on time, skipped): P0 0.00000, 0,
  0.00000, 0; P1 0.00000, 0, 0.00000, 0; P2 0.00000, 0, 0.00000, 0; **B1 0.00336, 23, 0.01879, 0**; P3 0.00000, 0,
  0.00000, 0; P4 0.00067, 0, 0.00067, 0 (so f5 keeps `stamps_skipped` 0 after FP2, by its line). w4-fp3.log: the same
  figures, fmt exit 1 (one diff, rewritten shorter by Edit). Added: +4. SIZE-CHECK: 3,517 + 2,787 = **6,304** (pass
  +91).
- **FP4 (TE-U4F-2), done 15:09 UTC.** Files: room/src/input.rs (new in-crate test
  `q9_a_the_kept_window_never_holds_more_than_k_under_a_flood_of_new_sequences`: 10,000 new sequences from 60,000,
  across the u16 wrap, `kept.len() <= KEPT_WINDOW` after each and equal from the 16th on; +1 test in both builds, a
  negative case), room/tests/commands.rs (f12 (iv): with 1 to 20 offered, a changed copy of seq 5 is kind 3 and one
  of seq 4 is `seq_outside_window`, forged `[0, 0, 1, 0, 0, 0, 0, 0]` and outside 1, under both options; +0).
  Acceptance: .gate/u4f/w4-fp4b.log (15:08 UTC, exit 0): fmt 0; q9_a "1 passed" measured and `--all-features`; f12
  "5 passed" in both builds; clippy both builds 0 (w4-fp4.log: the same passes, fmt exit 1, one diff rewritten by
  Edit). Plant: input.rs sha256 before ea382f2700d92812dbd0478885ee719c30f841b7373c765cd8d02d13ee0b4e1b; the three
  lines of `pop_front` (offer's `if self.kept.len() >= InputBuffer::KEPT_WINDOW { self.kept.pop_front(); }`) removed
  by Edit; .gate/u4f/w4-fp4-plant.log (`-x -k`, 15:08:56-15:08:57 UTC, exit 101): "test
  input::tests::q9_a_the_kept_window_never_holds_more_than_k_under_a_flood_of_new_sequences ... FAILED", panic "16:
  17"; restored by Edit, sha256 after ea382f27...e0b4e1b (equal). Added: +20 (input.rs +16, commands.rs +4).
  SIZE-CHECK: 3,533 + 2,791 = **6,324** (pass +111).
- **FP5 (SEC-U4F-1), done 15:10 UTC.** File: bots/src/lib.rs (`forger` returns `None` for `every < 2`, in place of
  `every.max(1)`; its doc states the precondition: kinds 2 and 5 forge against the previous command, which must have
  been accepted, which every interval of 2 or more gives on a lossless profile; new test
  `tests::f15_a_forgers_interval_below_2_is_refused` in f15's module: `forger(k, 1, 12)` None and `forger(k, 2, 12)`
  Some for kinds 1, 2, 4, 5, 6: +1 test in both builds; no caller passes 1: Grep `\.forger\(`). Acceptance:
  .gate/u4f/w4-fp5.log (15:10 UTC, exit 0): fmt 0; `cargo test -p npt-bots --lib f15_` "6 passed; 0 failed" (the new
  case and the five kinds); clippy both builds 0. Added: +14. SIZE-CHECK: 3,547 + 2,791 = **6,338** (pass +125).
- **FP6 (NC-U4F-2), done 15:12 UTC (its hand lines at the end checks, item 5).** Files: ops/runs/c-u4f-runs.mjs
  (SETS[22].args `["--profile", "P0", "--controller", "1", "--suite", "never-ack"]`, so its bite run, which takes
  `set.args` too, runs it as well; the header's -22 sentence names U4f's controller and that the bite runs under it),
  lab/src/scenario.rs (`never_ack_short`'s settings gain `--controller 1`, its doc says why). Check:
  .gate/u4f/w4-fp6.log (15:11-15:12 UTC, exit 0): fmt 0; `node --check ops/runs/c-u4f-runs.mjs` 0; f21 and f14 (iii)
  (= f28's -22 half) "2 passed" in the measured build, 12.23 s; the short run's summary "lead the controller's, at
  most 9 ... overflow resyncs 1". Both builds' end checks and the two hand lines follow. Added: +2. SIZE-CHECK: 3,548
  + 2,792 = **6,340** (pass +127).
- **FP7 (SEC-U4F-2 = TE-U4F-6, EA-U4F-5's header half), done 15:14 UTC.** Files: lab/src/scenario.rs (the header's
  `lead_bound_ms` is written in `run()` from `server.room.lead_bound().0`, beside `event_queue_cap_count` (also read
  from the live room), and no longer in `params()`, which runs at parse time before the room exists; the means of the
  named fix, not a departure), marks/SERIES.md (U4f's new row: its "Written by" cell reads "npt-lab (from the room's
  `lead_bound()`)"; its meaning cell unchanged). Acceptance: .gate/u4f/w4-fp7.log (15:13 UTC, exit 0): fmt 0; f6 A and
  B "2 passed" (each asserts `lead_bound_ms` 400, scenario.rs:3639); clippy both builds 0; Grep `LEAD_BOUND_MS` in
  scenario.rs: 0 matches. Added: +3 net (6 in, 3 out). SIZE-CHECK: 3,551 + 2,792 = **6,343** (pass +130).
- **FP8 (EA-U4F-8), done 15:14 UTC.** File: ops/runs/c-u4f-r2.mjs (the header comment's bullet, two physical lines,
  and r2.txt's head line in `lines`, both reading "-21's rows (p2-p4, p8, p9, p13, p15, p16, p18-p20) from a commit
  before U4f2's merge are an early read of the built suites, not -21's 5.3 record (advisor-u4f-stop.md:90-94)").
  Acceptance: .gate/u4f/w4-fp8.log (15:14 UTC, exit 0): `node --check` 0; `grep -n "early read"` finds :24 (header)
  and :145 (r2.txt's head). Added: +3. SIZE-CHECK: 3,551 + 2,795 = **6,346** (pass +133).
- **FP9 (S4, S5, S6), done 15:16 UTC.** Files: lab/src/scenario.rs (S4: USAGE gains `[--lag-hold-ms <ms>]
  [--lag-every-ms <ms>] (with --suite lag-switch)`), marks/SERIES.md (S5: U4f's subsection title now reads "...
  report-only rows that no check reads, but the `event.reorder.*` row, which the existing `reorder` rule reads"; a
  title in U4f's own new subsection, no existing row touched), ops/runs/c-u4f-runs.mjs (S6: :7 reads "in R2's order
  (advisor-u4f-stop.md:126-127: -47, -20, -22, then -21's)"). Acceptance: .gate/u4f/w4-fp9.log (15:15 UTC, exit 0):
  fmt 0, `node --check` 0, the three greps hit (scenario.rs:1978, SERIES.md:372, c-u4f-runs.mjs:7); `node check.mjs
  --only refs,marks` PASS (refs 1512 tracked files; marks 81). That check.mjs call ran under the shared lock, against
  u4f-run.sh's rule (`-x` for a check.mjs run), so it was re-run under `-x`: .gate/u4f/w4-fp9-check.log (15:15:40 UTC,
  exit 0) "check: all 2 steps pass". From here on every check.mjs and clippy call is `-x`. Added: +2. SIZE-CHECK:
  3,552 + 2,796 = **6,348** (pass +135).
- **FP10 (NC-U4F-3's docs), done 15:17 UTC.** Docs only: lab/src/scenario.rs (`CONTROLLER_MAX_LEAD`'s doc),
  client/src/lib.rs (`with_clock`'s doc), room/src/input.rs (the kind-10 comment above :321-323's refusal): each says
  the cap of 9 keeps every honest command inside option B's 8 only while the up delay is at least about 2 ticks
  whenever the cap binds, which L1's bounded jitter always gives, other links C-U6-17's. Check: .gate/u4f/w4-fp10.log
  (15:16 UTC, exit 0): fmt 0 on the three crates; grep `C-U6-17` hits scenario.rs:50, lib.rs:358, input.rs:317 (the
  doc step runs at the end checks). Added: +5. SIZE-CHECK: 3,557 + 2,796 = **6,353** (pass +140).
- **FP11 (EA-U4F-5's tie), done 15:18 UTC.** File: lab/src/scenario.rs (`CONTROLLER_MAX_LEAD` and its doc moved below
  the `use` lines; `const _: () = assert!(CONTROLLER_MAX_LEAD as usize <= npt_room::InputBuffer::CMD_CATCHUP + 1 &&
  CONTROLLER_MAX_LEAD < npt_sim::ticks_for_ms_at(npt_sim::TICK_HZ, Room::<Sim>::LEAD_BOUND_MS))`). Acceptance:
  .gate/u4f/w4-fp11b.log (`-x`, 15:18 UTC, exit 0): fmt 0, `cargo build -p npt-lab` both builds 0, clippy `-D
  warnings` both builds 0 (w4-fp11.log: the same builds and clippy 0, fmt exit 1 on the assert's line, rustfmt's
  split applied by Edit). Added: +6. SIZE-CHECK: 3,563 + 2,796 = **6,359** (pass +146).
- **FP12 (EA-U4F-1 in the open files), done 15:21 UTC.** Files: client/src/lib.rs (`ClientEndpoint::join_tick()`
  removed; `plan()` reads the field; Grep `join_tick\(\)`: no caller), room/src/input.rs (`InputBuffer::KEPT_WINDOW`
  pub(crate)), bots/src/lib.rs (f36 (ii) asserts `e.not_due()` 0, its doc names 2.2's bot rule: `not_due()` stays
  public with this consumer), lab/src/scenario.rs (`FastClock` carries the fast behaviour's `fast_left_out()` beside
  the kind-1 and kind-2 counts and writes each episode's `left_out_count`; the call site reads slot 0's
  `Behaviour::fast_left_out`; f19's hand-made records check it (1) and its short run asserts it 0 in both episodes:
  `fast_left_out()` stays public with this consumer), marks/SERIES.md (U4f's own `event.fast_clock.*` row gains
  `.left_out_count`, its source and meaning). Acceptance: .gate/u4f/w4-fp12b.log (`-x`, 15:20-15:21 UTC, exit 0): fmt
  0; clippy `-D warnings` on npt-client, npt-room, npt-bots, npt-lab, both builds, 0; f36 (ii) passed; `cargo test -p
  npt-client` whole 0. w4-fp12.log (`-x`, 15:20 UTC): the same clippy 0; f19 and f28 fast-clock A and B "3 passed"
  (f19's run: both episodes `"left_out_count": Number(0)`; f28's contract check finds the name's row); fmt exit 1 (one
  diff in bots, the message shortened by Edit). Added: +4 net. SIZE-CHECK: 3,567 + 2,796 = **6,363** (pass +150).
- **FP13 (EA-U4F-4), done 15:22 UTC.** File: bots/src/lib.rs (f18 and f15's kind-6 case read the extent the room
  reads, `World::new(&scene, 1, FIRST).greybox().areas().rects[0]` as room.rs:544 does; f18 asserts each clamp point
  inside it, f15's kind 6 that the forged aim lies outside it and inside the grid's ±32,767; FAR_CM's doc names f18).
  Acceptance: .gate/u4f/w4-fp13.log (`-x`, 15:22 UTC): fmt 0; f18 and f15 kind 6 "2 passed"; clippy both builds 0;
  `grep -n "12_800" crates/bots/src/lib.rs` exit 1, no match (so the runner's exit is 1, grep's). Added: +0 net.
  SIZE-CHECK: 3,567 + 2,796 = **6,363** (pass +150).
- **FP14 (EA-U4F-6), done 15:25 UTC.** File: lab/src/scenario.rs (new test
  `scenario::tests::ea_u4f_6_one_second_of_the_sims_full_walk_is_move_cm_s`: TC's slot 0, whose home lies 60 m west
  of the centre, clear of every box, building and plate (greybox.rs:92-150; scene.rs:262, :339-343), walks [-127, 0]
  with no button for 30 `World` steps (1 s at 30 Hz); `logs::moved_um` of its start and end records equals
  `MOVE_CM_S` x 10,000 µm (600 cm); +1 test in both builds). Acceptance: .gate/u4f/w4-fp14b.log (`-x`, 15:25 UTC,
  exit 0): fmt 0; the test "ok" in the measured and bites builds (0.00 s); clippy both builds 0 (w4-fp14.log: the same
  passes, fmt exit 1, two diffs, applied by Edit). Added: +38. SIZE-CHECK: 3,605 + 2,796 = **6,401** (pass +188).
- **FP15 (TE-U4F-3), done 15:26 UTC.** File: room/tests/commands.rs (`bites::pm_0001_21_not_older_than_...` also
  sends, on a bitten room under A, seq 1 at j + 2, a tick, then seq 2 at j + 1, a tick, and asserts the refusal
  records read `[(2, 2)]`: still kind 2 under the bite; +0 tests). Acceptance: .gate/u4f/w4-fp15.log (`-x`, 15:25
  UTC, exit 0): fmt 0; the ten -21 room and sim bite proofs `--all-features` "10 passed"; clippy `--all-features` 0.
  Plant: input.rs sha256 before 50ee780a59a67c194c6bb1f16dc7cce1434ba0de4bcd2f09fc1dc14cff7454e6; :272 read as `let
  newer = if r.not_older_than { true } else { d > 0 };` by Edit; .gate/u4f/w4-fp15-plant.log (`-x -k`,
  15:26:06-15:26:07 UTC, exit 101): "test bites::pm_0001_21_not_older_than_lets_an_equal_tick_command_run ...
  FAILED" (left `[]`, right `[(2, 2)]`), while `pm_0001_21_no_newer_tick_...` stays ok: the proof tells the two bites
  apart; restored by Edit, sha256 after 50ee780a...7454e6 (equal). Added: +7. SIZE-CHECK: 3,605 + 2,803 = **6,408**
  (pass +195).
- **FP16 (TE-U4F-4), done 15:27 UTC.** File: client/tests/carry80.rs (f24, from the newest 130: a refused header at
  newest + 33 is dropped, `refused()` + 1, the newest unchanged, no hash; one at newest + 32 is taken, the sentinel
  `(162, 0)` filed, the newest 162; so a bound of 31 or 33 fails it; +0 tests). Acceptance: .gate/u4f/w4-fp16.log
  (`-x`, 15:26 UTC, exit 0): fmt 0; f24 ok; clippy 0. Added: +9. SIZE-CHECK: 3,605 + 2,812 = **6,417** (pass +204).
- **FP17 (TE-U4F-7), done 15:27 UTC.** File: lab/src/scenario.rs (`f2_the_sims_check_bites_are_routed_to_the_world_alone`
  loops over the two literal ids `["PM-0001-21/no-cooldown", "PM-0001-21/refuse-not-clamp"]`, not
  `Sim::CHECK_BITES`; its doc says why). Acceptance: .gate/u4f/w4-fp17.log (`-x`, 15:27 UTC, exit 0): fmt 0; the test
  ok in the bites build; clippy `--all-features` 0. The literals sit in scenario.rs's `#[cfg(test)] pub(crate) mod
  tests` (:2414), which check.mjs's one-home rule does not read (check.mjs:925-945): .gate/u4f/w4-fp17-deps.log (`-x`,
  exit 0) `node check.mjs --only deps` PASS. Added: +1. SIZE-CHECK: 3,606 + 2,812 = **6,418** (pass +205).
- **FP18 (TE-U4F-8, TE-U4F-5's phrase), done 15:29 UTC.** Docs only, client/tests/trace.rs: f4 "Seed 4", and its
  negative half named, "U4e's e9 (server/tests/pump.rs:193-212)" (e9's head read there: :192-194); f5 "Seed 5"; f32
  (ii) "Seed 32, both runs" (written in FP2); f32 (iii) "Seed 33"; f36 (i) "Seed 36"; the bites newest-sample "Seed
  47", tick-quantised "Seed 48", redundancy-1 "Seed 20"; f30 "Seed 30, both". Acceptance: .gate/u4f/w4-fp18.log
  (`-x`, 15:28 UTC, exit 0): fmt 0; the grep finds all ten lines (:402-403, :414, :462, :499, :518, :550, :569, :591,
  :608); clippy `--all-features` 0. Added: +4. SIZE-CHECK: 3,606 + 2,816 = **6,422** (pass +209).
- **FP19 (RR-U4F-5 (1), (2), (5)), done 15:30 UTC.** Files: lab/src/scenario.rs (`FastClock::tick`'s three episode
  differences, kind 1, kind 2 and FP12's left-out, by `saturating_sub`, computed once as `episode`), bots/src/lib.rs
  (the lag switch's `now_ns - first` and `now_ns - at` by `saturating_sub`; `toward_centre`'s doc says `lane` passes
  offsets from decoded positions, x - x0 and y - LANE_Y_CM, and why the i64 arithmetic still cannot overflow). No new
  test. Check: .gate/u4f/w4-fp19b.log (`-x`, 15:30 UTC, exit 0): fmt 0; clippy npt-bots and npt-lab both builds 0;
  lab f19, f20 (the lag switch) and f28 fast-clock A "3 passed". w4-fp19.log (`-x`, 15:29 UTC): `cargo test -p
  npt-bots` whole 0 (lib 13 passed, 1 ignored; behaviour 5), f19 and f20 passed, clippy 0; fmt exit 1 (one diff,
  rewritten by Edit). Added: +3. SIZE-CHECK: 3,609 + 2,816 = **6,425** (pass +212).
- **The pass's own diff** (`git diff --no-index --shortstat` of builder 3's tree, the reviewed one, against W's
  spikes/netcode-proof): 10 files, 330 insertions, 121 deletions (a changed line counts in both); docs/design
  untouched. By the notes' counting rule: 6,213 to 6,425, +212 (the 3 more than 330 - 121 are lane-head lines FP1
  rewrote, which moved the deletions from 307 to 310). Under the advisor's 500.

### End checks (on one tree, after FP19)

- Pre-check (not counted): .gate/u4f/w4-pre-doc.log (`-x`, 15:31 UTC): `node check.mjs --only
  fmt,doc,clippy,clippy-wasm32` "check: all 4 steps pass" (doc 38.4 s), so no counted run is spent on a doc link.
  Tree hashes there: diff 2f07abc2275887936f014184e1aa0c96289f321b6f78b591db66dadbf2803fad, untracked
  1a767d5b4a3febbc8886e11f2c340db7f94382ae3bca9bbe7623847890bee5de.
- End check A starting 15:32 UTC: `df -h /home/user` 8.6G avail (78 %); one `-x` call, .gate/u4f/w4-end-check-a.log,
  in w3-end-check-a2.log's form (the two `--no-run` test builds, the two release lab builds, the two wasm hash
  builds, `node check.mjs`).

### State at 15:39 UTC (the coordinator's usage check, 92 %)

- **Where:** the main folder's u4f-build.md was tried again by Edit at 15:39 and refused by the harness again ("This
  agent is isolated in the worktree ..."), so this block stays in W's copy (this file), byte-equal to the main
  folder's :1-866 above it.
- **Done:** FP1 to FP19, each with its line above (acceptance log, fails-first or plant log, added lines).
- **No plant in place.** Both plants are restored and checked: FP4's (input.rs, pre-plant sha256 ea382f27...e0b4e1b,
  equal after) and FP15's (input.rs, pre-plant 50ee780a...7454e6, equal after).
- **SIZE-CHECK so far:** 6,425 (3,609 insertions + 2,816 new-file lines): the pass +212 by the notes' rule; the
  pass's own diff against the reviewed tree 330 insertions, 121 deletions.
- **Current step:** end check A, running in the background since 15:32 UTC (`-x`, w4-end-check-a.log; at 15:33 still in
  its second `--no-run` build, cold). No edit is made while it runs.
- **Next steps:** read end check A's result; `df`; end check B (.gate/u4f/w4-end-check-b.log, the same seven
  commands); then, in `-x` calls: the `--list` lines of the four new tests' binaries; each new test alone with the
  C62H-BOUND reference (.gate/u4f/w4-times.log); item 4 `cargo test --locked -p npt-server --all-features`; item 5's
  hand lines s22 and s22b with `--controller 1` (`-x -k`, out .gate/u4f/w4-hand/) and their header and line checks,
  and `node --check` of both run scripts; then the return.
- **End check A's first attempt stopped (not a counted check, no failure):** I gave the background call a 600 s
  limit, and the Bash tool killed it at about 15:42 UTC, inside check.mjs's test step. The log
  (.gate/u4f/w4-end-check-a.log) holds the six builds exit 0 and fmt, clippy, clippy-wasm32 and doc PASS, then nothing
  (no `--- end` line). `ps` after it: no cargo, rustc, check.mjs, runner or toucher process left. My error (the limit),
  not the tree's. Re-run whole as end check A, .gate/u4f/w4-end-check-a2.log, with a 3,600 s background limit.
- Disk before it (15:45 UTC): `df -BM /home/user` 4,541 MiB available (445 above the 4,096 MiB floor; it was 11,748
  at 14:57 and 8.6G at 15:31; the shared target now holds 7,195 MiB by `du -sm`, after the cold rebuild of both
  builds' tests, both release labs and both wasm builds). Above the floor, so A starts; B starts only if `df` then
  shows at least 4,096 MiB, else I stop and report (brief section 4 rule 6: no clean, pack or delete is mine).
- **End check A** (.gate/u4f/w4-end-check-a2.log, `-x`, 15:43:29-16:00:09 UTC, exit 0; free 4,540 MiB at the start;
  HEAD b25b96a; diff 2f07abc2275887936f014184e1aa0c96289f321b6f78b591db66dadbf2803fad, untracked
  1a767d5b4a3febbc8886e11f2c340db7f94382ae3bca9bbe7623847890bee5de): the six builds exit 0; fmt 0.9 s, clippy 4.6 s,
  clippy-wasm32 1.1 s, doc 6.3 s, **test 882.8 s: measured 634 passed (base 552: +82), bites 731 (base 632: +99), 0
  failed, 0 reported a skip**; deps 1.7 s (143 crate files; "no bite id is in two crates' sources"); wasm 54.5 s
  (27,000 ticks, 0 mismatching both ways; hash-map and std-sin caught); refs 1512 tracked files; marks 81; self 78 of
  78; redact 10 of 10, 316 items, 0 findings; **"check: all 11 steps pass"**. Provenance: every npt-* crate "Compiling
  ... (/home/user/Shamble-Frontlines/.claude/worktrees/agent-a5b1297a320917dfb/spikes/netcode-proof/crates/...)"
  (npt-bots, -client, -lab, -room 4 each). The test step 882.8 s against 1,207 s: under (builder 3's A and B: 692.1
  and 685.6 s). The run ended 1 s before the org's spend-limit cut (16:00:10 UTC); resumed 17:43 UTC by the
  coordinator, who checked: no process left, the worktree's runner hashes equal A2's start, no plant in place, 4,509
  MiB free.
- End check B starting (17:4x UTC): `df -BM /home/user` 4,509 MiB available (413 above the floor); one `-x` call,
  .gate/u4f/w4-end-check-b.log, the same seven commands, a 3,600 s background limit. No edit while it runs.
- **End check B** (.gate/u4f/w4-end-check-b.log, `-x`, 17:44:46-18:04:57 UTC, exit 0; free 4,507 MiB at the start;
  HEAD b25b96a; diff 2f07abc2..., untracked 1a767d5b...: equal to A's, no edit between): the six builds exit 0; fmt
  0.8 s, clippy 38.2 s, clippy-wasm32 4.8 s, doc 16.4 s, **test 834.0 s: 634 and 731 passed, 0 failed, 0 skipped**;
  deps 2.5 s ("no bite id is in two crates' sources"); wasm 53.4 s (27,000 ticks, 0 mismatching both ways); refs 1512;
  marks 81; self 78 of 78; redact 316 items, 0 findings; **"check: all 11 steps pass"**. Provenance as A's (every
  npt-* crate compiled from this worktree's crates/). Test step 834.0 s against 1,207 s: under.
- **Counts against U4f's share** (deltas on 5b29296b's 552 and 632): measured +82 against +73; bites +99 against +88.
  The pass's own +4 in both builds (builder 3's +78 and +95 before): f25 (ii), U4F-Q9 (a)'s flood test, f15's
  interval case, EA-U4F-6's walk test. **Negative cases, by row: 46 against the share's 45:** f3 2, f4 1, f7 1, f8 3,
  f10 8, f11 2, f12 5, f13 4, f14 3, f15 5, f19 1, f20 1, f21 1, f24 1, **f25 2** (f25 and the pass's f25 (ii)), f26
  1, f27 1, f35 1, f39 1, U4F-Q6 (b)'s Pong field 1, **U4F-Q9 (a) 1** (the pass's flood test). Not counted as negative:
  f15's interval case (the forger's own parameter, no hostile input to the room); f4's negative half is U4e's e9
  (pump.rs:193-212), named in f4's doc, not a new test.
- **`--list` lines** (.gate/u4f/w4-list.log, `-x -k`, 18:05 UTC, every call exit 0): "f25_ii_an_older_command_past_the_
  codecs_tick_step_is_left_out: test" (client carry80, 5 tests); "input::tests::q9_a_the_kept_window_never_holds_
  more_than_k_under_a_flood_of_new_sequences: test" (room lib, 24 tests); "tests::f15_a_forgers_interval_below_2_is_
  refused: test" (bots lib, 14 tests); "scenario::tests::ea_u4f_6_one_second_of_the_sims_full_walk_is_move_cm_s: test"
  (lab bin 80 tests, and with `--features bites` 91).
- **Each new test alone** (.gate/u4f/w4-times.log, `-x -k`, 18:05-18:06 UTC, measured build, "finished in"): the
  reference `c62hb_p2_short_every_measured_lost_tick_is_explained` 13.59 s, so the line is 27.18 s; f25 (ii) 0.00 s;
  q9_a 0.00; f15's interval case 0.00; EA-U4F-6's walk 0.00. Changed, for the record: f32 (ii) with its stall run
  0.03 s, f5 0.16 s, f21 (its never-ack run now under the controller) 11.69 s, lab f19 8.64 s. All under the line.
- **Item 4** (.gate/u4f/w4-item4.log, `-x`, 18:07 UTC, exit 0): `cargo test --locked -p npt-server --all-features`:
  lib 0, main 0, gap 6, loopback 6, pump 20, schedule 17 passed, 0 failed.
- **Item 5** (.gate/u4f/w4-hand.log, `-x -k`, 18:08 UTC, every command exit 0): s22 (`... --profile P0 --controller 1
  --suite never-ack --role measured --marks PM-0001-2,PM-0001-22`, release): "hand-s22 SS P0 seed 1: 180 ticks; lead
  the controller's, at most 9; ... rewinds refused 33; ... overflow resyncs 1"; s22b (bites, `--role bite --bite
  PM-0001-22/unbounded`): "hand-s22b SS P0 seed 1: 1080 ticks (900 after, ...); lead the controller's, at most 9; ...
  rewinds refused 1018; ... overflow resyncs 0"; `node --check` of c-u4f-runs.mjs and c-u4f-r2.mjs exit 0.
  FP6's checks: both room.jsonl headers hold `"controller_max_lead_count":9` (and `lead_bound_ms` 400,
  `event_queue_cap_count` 1600), and no header of the four (room and lab, both runs) holds `lead_ticks_count` (grep -c
  0). s22: `ack_starved_headroom_count` reaches 0 once (t 91), never below; one `overflow_resync` (slot 0, t 91), then
  `session_end` `"ended":"kicked"` (slot 0, t 152, `overflow_resyncs_count` 1, `kicked_count` 1). s22b: no
  `overflow_resync` and no `session_end` (its lab log's kinds: class_audit, client_counts x48, fate_log, reorder,
  run_lengths); its headroom runs below 0 (to -3,765), the unbounded queue being the bite.
- D10's read: `git diff --numstat b25b96a6 -- spikes/netcode-proof/marks spikes/netcode-proof/crates/judge` prints
  only `20 0 spikes/netcode-proof/marks/SERIES.md` (U4f's new lines; the pass's three edits are inside them).
- Final tree: runner hashes (B's header) diff 2f07abc2275887936f014184e1aa0c96289f321b6f78b591db66dadbf2803fad,
  untracked 1a767d5b4a3febbc8886e11f2c340db7f94382ae3bca9bbe7623847890bee5de; `git status --short`: the 20 modified
  files under spikes/ and docs/ as at the start, plus this notes file, and the 9 untracked new files (as at the
  start). No plant in place. Disk 4,388 MiB free at 18:08 UTC.

### Return (builder 4, 18:10 UTC)

Every FP item FP1-FP19 done as named, with its acceptance check met; the items' lines above hold the files and logs.
Departures: none from a named fix. Notes on means: FP7 writes the header param in `run()` (the room exists only
there); FP2's stall case sits inside f32 (ii) (+0 tests) and the trace harness lets the host make its overdue steps
at once after a stall; FP5's case is a new test (+1). Process notes: the notes are in W's copy (the harness refused
the main folder's file twice); one check.mjs partial call ran under the shared lock and was re-run under `-x`
(FP9); end check A's first attempt was stopped by my 600 s background limit, re-run whole as A2. Not done: no
snapshot with worktree_patch.py (not in my prompt), no judge call on the hand runs (FP6 names none), FP20 (the main
session's).
