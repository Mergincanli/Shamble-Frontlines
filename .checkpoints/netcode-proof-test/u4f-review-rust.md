# U4f review panel, round 1: seat `rust` (rust-reviewer), 2026-11-19

Tree: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a0825a5a2491eb175 (HEAD 07289a4, U4f uncommitted). Read only.
This agent has no Edit tool: the notes file is rewritten whole with Write each time (the brief's rule for that case).

## Plan
1. Brief sections needed for my seat: u4f-brief.md 2.1-2.9 (what to build), 4 (build rules).
2. Rulings: advisor-u4f-stop.md S1-S7/N1 (:198-210), R1-R3 headlines.
3. The Rust diff, file by file, hostile-value paths first: room/src/input.rs, room/src/room.rs, room/src/events.rs,
   proto/src/msg.rs, client/src/clock.rs, client/src/lib.rs, server/src/pump.rs, sim/src/world.rs, bots/src/lib.rs,
   lab/src/scenario.rs + logs.rs (lab: hand-run tooling, lower bar).
4. For each: arithmetic on client values (stamps, ticks, seqs, counts) unchecked/wrapping/casts; indexing/unwrap/expect
   reachable from input; allocation per tick in the room's step; float use; HashMap iteration; `unsafe`.
5. check.mjs's wasm, clippy, clippy-wasm32, fmt, doc, deps lines in end checks A2 and B.
6. Runs only to prove a finding (runner, one command per call).

## Notes, one line per finding as found
- Read done (14:15 UTC): room input.rs whole, room.rs/lib.rs/events.rs diff, client clock.rs whole + lib.rs diff, proto
  msg.rs, server pump.rs, sim world.rs, bots lib.rs diff, lab scenario.rs/logs.rs non-test diff; A2's and B's fmt,
  clippy, clippy-wasm32, doc, deps, wasm lines. Patch: no Cargo.toml/Cargo.lock among its 29 files (`git apply
  --numstat`), no `unsafe`, floats only in tests (lab scenario.rs tests, client trace.rs), no HashMap/HashSet, no clock
  read (grep of the patch's added lines; patch sha256 74ae26aa29c229d6).
- Room hostile-value walk (input.rs:34-36 `dist`, :210-345 offer/filter/check, :350-391 due/take/drain; room.rs:920-1030
  receive_input, :1057-1100 fill_input, :1330-1372 run_one): every tick compare goes through `dist` (i64 of
  `wrapping_sub as i32`); kind 4 squares i8 widened to i32 (max 2 x 128^2 = 32,768); kind 5 u8 wrapping_sub into i64;
  `take`'s `before - len` follows a retain; `kept_stamps[slot]` / `run_presses[slot]` cannot go out of bounds (both
  `vec![..; slots.max(1)]` like `clients`, and `slot` already passed `clients.get_mut`; `join` checks `i <
  clients.len()`); counters are u64 += 1; per-tick press sums are u32 reset each tick (at most 4 x 255 x 32). No panic
  or overflow found from network values in npt-room.
- Room per-tick allocation after warm-up: none new. `drained` is taken and put back (room.rs:1068, :1105), `ran_early`,
  `wraps`, `rewinds_refused` cleared with capacity kept, `incoming`/`refusals` swapped (room.rs:1039-1040), `kept` <= 16,
  `stamps` <= 64, `buffered` <= 8 (32 in bites builds), `refusals` <= 4,096 (more counted in `refusals_dropped`).
  The only heap case: `canonical`'s stable sort (room.rs:444-445, sort_by_key) above about 102 commands in a tick
  (element 40 B: Command 32 B + Option<Rewind> 8 B; std's stable sort keeps a 4 KiB stack scratch, heap above it), so
  option A burst ticks only; the room's other stable sorts (room.rs:1130, :1200, :1387-1388) already do the same. Own
  line, no finding.
- RR-U4F-1 (Medium): client lib.rs:751-774 `drop_unencodable` keeps the newest run of consecutive *indices* (:770) and
  drops commands that fail *alone*; the codec's second pair rule (an older command's tick 0 to 255 below its newer
  neighbour: proto/src/input.rs:158-159, `u8::try_from(n.tick.wrapping_sub(c.tick))` -> OutOfRange) is not checked.
  With a stamp jump > 255 ticks between two unacknowledged neighbours, the second encode (lib.rs:702) fails too, the
  step sends an acks-only `Input` (:703-705), and the stale commands stay in `unacked`, so the next up to 7 steps fail
  the same way and carry no command (about 0.27 s at 30 Hz), then one `Input` of 8 mostly late commands. The comment at
  :697-698 ("the newest consecutive run of the rest is kept (the codec's rule)") claims what is not done. Reach: a
  fixed-lead endpoint (`with_lead`, lab-only: U4r's suites, the fast clock) whose newest tick jumps > 255 after an
  outage with lost commands, or the controller after >= 128 steps with no page command while old commands sit
  unacknowledged; no judged profile reaches it (B1's bursts are 1 s: l1/src/lib.rs:197-200). f25 (carry80.rs:82-99)
  tests the aim case only. npt-bots' own rewrite gets this right by trial-encoding and popping the oldest
  (bots/src/lib.rs:479-489).
- RR-U4F-2 (Low): clock.rs:163 `i64::try_from(offset).unwrap_or(i64::MAX)`: an offset below i64::MIN (server_ns =
  u64::MAX with echo = arrival = 0: offset about -1.8e19) is stored as +i64::MAX, a sign flip. Only a `Pong` matching an
  outstanding (id, echo) pair reaches it, so only a broken or hostile server; saturate by sign.
- RR-U4F-3 (Low): client lib.rs:696 and :702 `encode(&Message::Input(input.clone()), ..)`: the happy path clones the
  `commands` Vec every step (one more heap allocation per client step; the client is the future Wasm hot path) only so
  the rare error path can reuse `input`, which then rebuilds `input.commands` from `unacked` anyway (:701).
- RR-U4F-4 (Low): no seeded hostile-input property test drives `Room::receive` with random `Input`s (arbitrary ticks
  around 0 and 2^31, sequences around the u16 wrap, i8::MIN moves, wrapping press counters, aims at the grid's limits,
  random ack fields) under both options in a debug build (npt-room's overflow checks on there), asserting no panic,
  exactly one counter per offered command, and the bounds (held <= 8, refusals <= 4,096, kept <= 16). The edge cases
  are pinned one by one (commands.rs f8 :143, f10 kind 4 :218, f13 :512-572, f38 input.rs:569; room.rs f8's 10,000
  triples) and my walk found no panic; the property test makes that claim durable.
- RR-U4F-5 (Low, hardening nits, none reachable today): lab scenario.rs:1030-1031 `refused[i] - self.base[i]` (u64;
  underflows, panicking in a debug or test build and wrapping in release, if slot 0's counters reset mid-episode);
  bots lib.rs:407 `now_ns - first` and :421 `now_ns - at` (u64, host time assumed monotone); clock.rs:122-124 the
  `while next <= now_ns` loop (O(gap / 100 ms), endless only at now_ns == u64::MAX: a closed form is one line) and
  clock.rs:219's division by `period_ns`, 0 for tick_hz > 10^9 (`ClockSync::new` is pub: clock.rs:63-65); bots
  `toward_centre`'s doc (lib.rs:250-254, "not for other inputs") now has a caller passing offsets (`lane`, :354),
  harmless in its i64 arithmetic.
- Runs: one, `git status --short` through the runner (.gate/u4f/review-rust/tree.log, 14:17 UTC): diff 54cb6faa...,
  untracked e8dc3663..., equal to review-pre.log and both end checks: I read the tree that was checked. No cargo run:
  every finding is a code read with its line; the fix pass's tests prove them.

## Lines the brief asks for
- Identical results on Wasm and native: A2 (w3-end-check-a2.log:198) and B (w3-end-check-b.log:185) wasm step PASS,
  native leg 1 vs native leg 2 and vs Wasm leg 1: 27,000 ticks each, 0 mismatching, h2 equal; clippy-wasm32 PASS on the
  measured and bites builds; deps: no fma, simd128 or relaxed-simd. U4f's sim change (world.rs:595-609, :944-951,
  :1315-1322, :1426-1460) is bites-only: in the measured build `refuse_far` is the constant false and `ready` unchanged,
  so the fire rule reads as before. npt-client's and npt-room's new code is integer-only with no usize-width-dependent
  result (all `len()` casts widen), so it is target-independent by the language's integer semantics; the wasm step
  runs npt-sim only, so that part is by reading, not by a run.
- `unsafe`: none added (patch grep; deps step: the workspace forbids unsafe_code, no new exception).
- Panics or overflow on hostile values: none found in npt-room; client: RR-U4F-2 (wrong value, no panic); lab and bots:
  RR-U4F-5 (not network-reachable).
- Allocation in the room's per-tick path: none new after warm-up but the stable sort's scratch above about 102 commands
  (option A bursts), the pattern the room's other sorts already have.
- Error handling: RR-U4F-1.
- Dependencies: no manifest or lockfile in the patch, so `cargo audit` / `cargo deny` are unchanged by U4f.
- S1-S7 and N1 (advisor-u4f-stop.md:198-210): no disagreement from this seat.

## Verdict
APPROVE WITH FINDINGS: one Medium (RR-U4F-1), four Lows. No Critical or High.

| Id | Severity | Where | Fix |
|---|---|---|---|
| RR-U4F-1 | Medium | client/src/lib.rs:751-774 (and the claim at :697-698) | after the per-command retain, keep the longest newest prefix of `unacked` that encodes whole (at most 8 trial encodes, rare path), as bots/src/lib.rs:479-489 does, or truncate at the first pair whose `u8::try_from(newer.tick.wrapping_sub(older.tick))` fails; count the rest `left_out`. Test: fixed-lead endpoint, one command left unacknowledged, newest moved +300, one more command: today the `Input` holds 0 commands; after the fix it holds the newest one and `twin_left_out()` (or the counter used) rose by 1 |
| RR-U4F-2 | Low | client/src/clock.rs:163 | `offset.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64`; a unit case with server_ns = u64::MAX |
| RR-U4F-3 | Low | client/src/lib.rs:690-707 | encode `Message::Input(input)` by value; on error build a fresh `Input { ack_tick, ack_groups, commands }` from `unacked` after `drop_unencodable` |
| RR-U4F-4 | Low | room/tests (new case) | the seeded property test above (splitmix64 as commands.rs:411-418, both options, about 10,000 datagrams) |
| RR-U4F-5 | Low | lab scenario.rs:1030-1031; bots lib.rs:407, :421; client clock.rs:65, :122-124, :219; bots lib.rs:250-254 | saturating_sub (or i64 differences) in the lab and bots; a closed-form next ping time; `period_ns.max(1)`; reword toward_centre's precondition |

## Not checked
- No cargo build, test, clippy, bench or Wasm build of my own: the end checks' figures are read from their logs.
- No benchmark exists for `Room::tick` or the client step before and after U4f (no `cargo bench`/criterion in the
  spike); the speed lines are by reading. Test-step time (692.1 s and 685.6 s against 1,207 s) is the only timing.
- Wasm size: npt-client's wasm32 build is linted and documented but has no release build in check.mjs's wasm step, so
  no byte count; ClockSync's i128 division will pull compiler-builtins' 128-bit division routines into a wasm32 client
  (unmeasured, likely small).
- Test files (commands.rs, trace.rs, carry80.rs, pong.rs, lab tests) were read for the hostile-value cases only, not
  for counts or assertion strength (the test seat's); ops/runs/*.mjs not read (not Rust).
- The lab's logic (episodes, attribution-free tallies, -47's read instant) beyond overflow and panics: the netcode and
  test seats'.
