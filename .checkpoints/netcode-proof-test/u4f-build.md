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
