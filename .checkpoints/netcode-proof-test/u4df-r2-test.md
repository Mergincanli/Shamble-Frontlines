# U4d-F round 2: test-engineer (on what changed: FX-1 and FX-2)

Copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_9ce88fa3-452-1 (branch u4df-build, HEAD dad9566, change
uncommitted). Round 1's files: /home/user/Shamble-Frontlines/.gate/u4df-r1. Packet: u4df-r2-packet.md (read whole).
Plan judged against: advisor-u4df.md R2 and T3. Role: main's .claude/agents/test-engineer.md (read, via git show
origin/main). Runner: u4df-run.sh, log prefix te2- under .gate/u4df/. Rules: plants by Edit in place, restored by Edit,
sha256 checked against r2-start.sha256; TE2-5 for any failure I did not plant (re-run that crate with --no-fail-fast at
once, name and panic here). Notes kept by Write/Edit only.

## Plan
1. sha256 of every file under the copy's spikes/netcode-proof/crates/sim (paths from the copy's root) -> Write to
   .gate/u4df/r2-start.sha256.
2. world.rs, tests/world.rs, tests/replay.rs: sha256 = round 1's [1] and = .gate/u4df-r1's; replay.rs and scene.rs:
   diff against .gate/u4df-r1's = FX-1 and FX-2 only (read the diff against R2 and T3, word by word for FX-2; rewrap
   <= 120).
3. FX-1's test by name in both builds (measured and --all-features), --nocapture, its counts line; its run time.
4. Plants Q1-Q5 (R2's forms; the builder's record for reference), each by Edit in replay.rs, the whole measured npt-sim
   suite --no-fail-fast, -x; each must fail FX-1's test by name; restore by Edit; sha256 -c after each. Plus at least
   one wrong-reckoning plant of my own design that R2 does not list (FX-1's reach).
5. The three-seed test once by name: its 21 "ES seed" lines = end-three-seeds.log's.
6. Settle by run any netcode-architect round-2 finding a run can settle (u4df-r2-netcode.md).
7. check.mjs once at the end, -x, counted: every step's line; deltas on 367 / 414 / 52 (expected 368 / 415 / 52).
8. sha256sum -c r2-start.sha256 from the copy's root at the end: all OK.

## Progress
- [1] 01:10:10 UTC: sha256 of all 17 files under the copy's spikes/netcode-proof/crates/sim (paths from the copy's
  root) written by Write to /home/user/Shamble-Frontlines/.gate/u4df/r2-start.sha256: replay.rs 802842b04e99, scene.rs
  a5ce8ac2e9a3, world.rs 6e01101323ce, tests/replay.rs a5f123b8a2dd, tests/world.rs 0a340fb5db38 (the other 12 =
  .gate/u4df-r1's). git status: the same 5 M + 1 ?? (the builder's record); HEAD dad9566; no cargo or rustc running.
- [2] Scope. world.rs 6e01101323ce, tests/replay.rs a5f123b8a2dd, tests/world.rs 0a340fb5db38: = round 1's [1] and =
  .gate/u4df-r1's sha256 (both sides hashed). Every other crates/sim file equal both sides except replay.rs (r1
  30a17b58d365 -> 802842b04e99) and scene.rs (r1 1d42cd960701 -> a5ce8ac2e9a3), = round 1's [1] on the r1 side. `git
  diff --no-index --stat` over the whole spikes/netcode-proof (r1 vs copy): 2 files, +164 / -4; the src diff is
  byte-identical to the packet's (diff of the two: none). replay.rs: one hunk @@ -768,3 +768,158 @@, additions only
  after `impl Replayer`'s last brace (the `#[cfg(test)] mod tests`): nothing above it changed. scene.rs: read-only
  script (scratchpad te2_fx2_check.py, T3 taken from advisor-u4df.md's fences): `ES_AIM_ONE_IN`'s doc (lines 77-86)
  = round 1's + " " + T3 (a) exactly; `aim_one_in`'s doc (124-131) = round 1's with "sets its 10 gameplay events a
  second" -> "tunes its gameplay events toward 10 a second" and T3 (b) inserted after "U4d close ruling, B1.2)"
  before the full stop, exactly; widest changed line 120; every line outside those two docs identical. FX-1's form
  against R2 by reading: place, name, recording (ES, seed 2, 21,600 from 1,000,000, 2,000 ms), the speed
  (.min(MOVE_SPEED_MAX_CM_S) = apply_inputs :1157), the estimate (Placement / 100 as f32, None before the first),
  phase 0 then recorded order, the move (`move_velocity(.., 1.0).0` x `1.0 / TICK_HZ as f32` = the World's DT,
  world.rs:73, :1344-1345; est + d per axis), held-back aimed from `before` and moved on the carrying tick at that
  tick's speed, lost never, the exact check with seed/slot/tick/carrying tick/seq, six counts printed and asserted.
  One addition R2 does not list: the assert that each command's tick is its carrying tick or the one before
  (replay.rs:897-904 region); it is what defines "held" for the reckoning, inside the test, not a new item. MET.
- [3] FX-1 by name, both builds, one runner call (te2-fx1-both.log 01:13:19-01:13:23 UTC, -x, runner exit 0;
  "Compiling npt-sim (this copy's path)" x2): measured "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured;
  5 filtered out; finished in 1.60s"; --all-features the same, "finished in 1.76s". Its line, equal in both: "FX-1: ES
  seeds [2], 21600 ticks each from tick 1000000: sky aims checked 954628; crouched 97974; held back 14860; of a slot a
  MoveSpeed re-sped 465658; the first after a lost command 14996; the first after a held-back command carried on a tick
  that re-homed or re-sped its slot 11" (= the builder's and the netcode's [7]). Under R2's 5 s in both. MET.
- [4] Q1, the old reckoning (u4df-review-test.md [5]'s two Edits), te2-plant-q1.log 01:13:38-01:14:32 UTC, -x,
  --no-fail-fast, runner exit 101 (warning: unused `one_tick`; "Compiling npt-sim (this copy)" x1). Planted diff:
  (i) old `if let Some((back, _)) = &c.held_back {` / `let d = one_tick(c.speed_cm, back.buttons, back.move_xy);` /
  `c.est = [c.est[0] + d[0], c.est[1] + d[1]];` / `}`; new `if let Some((back, _)) = &c.held_back {` / `let _ =
  back;` / `}`. (ii) old `let d = one_tick(c.speed_cm, c.buttons, c.mv);` / `let after = [c.est[0] + d[0], c.est[1] +
  d[1]];`; new `let speed = scene.move_speed as f32 / 100.0;` / `c.est[0] += f32::from(c.mv[0]) / 127.0 * speed /
  TICK_HZ as f32;` / `c.est[1] += ...mv[1]...;` / `let after = c.est;`. FAILED by name
  `replay::tests::the_sky_aims_reckon_every_input_the_recording_emits` (replay.rs:885, "seed 2, slot 1, tick 1000057
  (carried on 1000057), seq 58", left [788, 360], right [798, 343]); lib "test result: FAILED. 5 passed; 1 failed".
  Beside it: d7 (tests/replay.rs:351, SS at 300 (300, 6624, 8955962681012353194)); a_m1 (tests/world.rs:940, "not
  vacuous"); t_m1 (:1151, "9846 ... window, 9240 and 10453 ... halves" = te-plant-p1.log). replay 5 + 1 failed, seam
  8, world 28 + 2 failed + 1 ignored. Restored by two Edits (old texts back); `sha256sum -c r2-start.sha256`: all 17 OK.
- [5] Q2, crouch ignored ([11]'s form), te2-plant-q2.log 01:15:00-01:15:56 UTC, -x, --no-fail-fast, exit 101. Planted
  diff (replay.rs:408, the generator's `one_tick`): old `let v = move_velocity(speed_cm, buttons, move_xy, 1.0).0;`;
  new `let v = move_velocity(speed_cm, buttons & !Command::CROUCH, move_xy, 1.0).0;`. FAILED by name FX-1's test
  (replay.rs:884, "seed 2, slot 2, tick 1000075 (carried on 1000075), seq 76", left [1014, -13], right [1007, -6]);
  lib 5 + 1 failed. Beside it: d7 (:351, SS at 300 (300, 6620, 4722149015725882838)), a_m1 (:940), t_m1 (:1151, "9866
  ... window, 10363 and 9370 ... halves" = te-plant-p3.log). seam 8; world 28 + 2 + 1 ignored. Restored by Edit;
  sha256 all 17 OK.
- [6] Q3, a held-back command's move dropped, te2-plant-q3.log 01:16:14-01:17:11 UTC, -x, --no-fail-fast, exit 101
  (warning: unused `back`). Planted diff (replay.rs:473-476): old `if let Some((back, _)) = &c.held_back {` / `let d =
  one_tick(c.speed_cm, back.buttons, back.move_xy);` / `c.est = [c.est[0] + d[0], c.est[1] + d[1]];` / `}`; new `if
  let Some((back, _)) = &c.held_back {` / `}`. FAILED by name FX-1's test (replay.rs:882, "seed 2, slot 0, tick
  1000004 (carried on 1000004), seq 5", left [1028, -74], right [1036, -92]); lib 5 + 1 failed. Beside it: d7 (:351, SS
  at 300 (300, 6672, 9991609429507691685)), t_m1 (:1151, "9605 ... window, 10103 and 9106"); a_m1 passes; world 29 + 1
  + 1 ignored. Restored by Edit; sha256 all 17 OK.
- [7] Q4, a MoveSpeed not held, te2-plant-q4.log 01:17:28-01:18:24 UTC, -x, --no-fail-fast, exit 101 (warning: unused
  import `MOVE_SPEED_MAX_CM_S`, the lib build). Planted diff (replay.rs:430): the line `clients[usize::from(slot)]
  .speed_cm = value.min(MOVE_SPEED_MAX_CM_S);` removed (the MoveSpeed still pushed). FAILED by name FX-1's test
  (replay.rs:883, "seed 2, slot 1, tick 1000157 (carried on 1000157), seq 158", left [584, 166], right [588, 165]:
  e = 157, the first MoveSpeed); lib 5 + 1 failed. Beside it: d7 only (:351, SS at 300 (300, 6612,
  3564984947766812572)); world "30 passed; 0 failed; 1 ignored" (t_m1 and a_m1 pass). So before FX-1 only D-7's pin
  held Q4. Restored by Edit; sha256 all 17 OK.
- [8] Q5, a lost command moving the estimate, te2-plant-q5.log 01:18:43-01:19:39 UTC, -x, --no-fail-fast, exit 101.
  Planted diff (replay.rs:587): old `1 => {}`; new `1 => {` / `c.est = after;` / `}`. FAILED by name FX-1's test
  (replay.rs:886, "seed 2, slot 1, tick 1000057 (carried on 1000057), seq 58", left [788, 360], right [798, 343]: the
  same first mismatch as Q1, which also moves on lost commands); lib 5 + 1 failed. Beside it: d7 only (:351, SS at
  300 (300, 6652, 6629039644418743120)); world "30 passed; 0 failed; 1 ignored". Restored by Edit; sha256 all 17 OK.
  Q1-Q5: each fails FX-1's test by name, each first mismatch = the builder's record (line, slot, tick, seq, values).
  MET. Under Q4 and Q5 only D-7's pin failed beside FX-1 (t_m1, a_m1 pass): FX-1 is what holds them in the counted
  suite now.
- [9] Own plant R-A (NC2-L1's Q6, to settle it by run): a held-back command moved at its hold-time speed, not the
  carrying tick's. Planted diff, three Edits in replay.rs: (1) struct Client, old `speed_cm: u32,` / `held_back:
  ...`, new `speed_cm: u32,` / `held_speed: u32,` / `held_back: ...`; (2) old `0 => c.held_back = Some((cmd,
  rewind)),`, new `0 => {` / `c.held_speed = c.speed_cm;` / `c.held_back = Some((cmd, rewind));` / `}`; (3) old `let d
  = one_tick(c.speed_cm, back.buttons, back.move_xy);`, new `let d = one_tick(c.held_speed, back.buttons,
  back.move_xy);`. te2-plant-ra.log 01:20:26-01:21:21 UTC, -x, --no-fail-fast, runner exit 0: lib "6 passed; 0
  failed", replay 6 (d7 passes), seam 8, world 30 + 1 ignored. The WHOLE measured suite passes it: FX-1, D-7 and
  t_m1 included (every pinned recording byte-identical). Probe (test's SEEDS [2] -> [1], R-A still planted; FX-1 by
  name, te2-plant-ra-seed1.log 01:21:30-01:21:32, exit 101): FAILED (replay.rs:888, "seed 1, slot 47, tick 1014152
  (carried on 1014152), seq 14153", left [189, -136], right [188, -137]) = the netcode's [7] case. NC2-L1 CONFIRMED by
  run: Low, as it says (one command's ~1 cm on seed 1; nothing on seed 2). Probe 2 (R-A restored; SEEDS -> [1, 2], the
  real generator; both builds by name, te2-probe-seeds12.log 01:21:44-01:21:51, exit 0): "sky aims checked 1907885;
  crouched 191643; held back 29743; ... re-sped 918773; ... lost 30111; ... carried ... 24", ok in 3.09 s (measured)
  and 3.59 s (bites), under R2's 5 s: NC2-L1's fix (add seed 1) is affordable. All restored by Edit (R-A's three
  back, SEEDS back to `const SEEDS: [u64; 1] = [2];`); sha256 all 17 OK.
- [10] Own plant R-B: a MoveSpeed taken one tick late (after the tick's commands, not in phase 0). Planted diff, two
  Edits: (1) the line `clients[usize::from(slot)].speed_cm = value.min(MOVE_SPEED_MAX_CM_S);` removed from the
  MoveSpeed block; (2) old `out.push(input);`, new `for a in &input.attributes {` / `clients[usize::from(a.slot)]
  .speed_cm = a.value.min(MOVE_SPEED_MAX_CM_S);` / `}` / `out.push(input);`. te2-plant-rb.log 01:22:16-01:23:11 UTC,
  -x, --no-fail-fast, exit 101: FAILED by name FX-1's test (replay.rs:886, "seed 2, slot 1, tick 1000157 (carried on
  1000157), seq 158", left [584, 166], right [588, 165]); lib 5 + 1 failed. Beside it: d7 at :382 only (the recording
  digests: SS (16694141, ..) not (16694921, ..), TC (799601, ..) not (799693, ..), ES's length 16700849 equal and its
  hash moved); world "30 passed; 0 failed"; t_m1 and a_m1 pass. FX-1 catches a one-tick phase-0 ordering error that
  the counted suite held only by D-7's digest before. Restored by two Edits; sha256 all 17 OK.
- [11] Own plant R-C: the move rule off by a last ulp (the generator's `one_tick` divides by the tick rate in place
  of the World's `* DT`, world.rs:73, :1345; the same difference as round 1's two P1 forms). Planted diff
  (replay.rs:409): old `[v[0] * dt, v[1] * dt]`, new `[v[0] / TICK_HZ as f32, v[1] / TICK_HZ as f32]`.
  te2-plant-rc.log 01:23:31-01:24:25 UTC, -x, --no-fail-fast, exit 101 (warning: unused `dt`): FAILED by name FX-1's
  test (replay.rs:884, "seed 2, slot 8, tick 1000330 (carried on 1000330), seq 331", left [576, -419], right [576,
  -420]: 1 cm on one axis); lib 5 + 1 failed. Beside it: d7 at :382, ES's digest only (SS's and TC's digests equal);
  world 30 + 1 ignored, t_m1 passes. FX-1 is exact to the float: it holds the reckoning at the last ulp, where t_m1's
  window can move by 1.1 a second either way (round 1 [5]). Restored by Edit; sha256 all 17 OK.
- [12] Own plant R-E: 3b's re-home does not reset the estimate (the teleport still emitted). Planted diff
  (replay.rs:462): the line `clients[usize::from(slot)].est = home.map(|k| k as f32 / 100.0);` (24-space indent,
  inside the 3b loop) removed. te2-plant-re.log 01:24:47-01:25:42 UTC, -x, --no-fail-fast, exit 101: FAILED by name
  FX-1's test (replay.rs:883, "seed 2, slot 1, tick 1000037 (carried on 1000037), seq 38", left [1074, -419], right
  [989, 17]: slot 1's first 3b re-home is e = 37); lib 5 + 1 failed. Beside it: d7 at :382 (ES's digest, length
  16698392 not 16700849), t_m1 (:1151, "9193 ... window, 9396 and 8990 ... halves"). Restored by Edit; sha256 all 17
  OK.
- [13] Own plant R-G: the held-back command's move taken BEFORE the tick's phase 0 (so a re-home wipes it, and a
  MoveSpeed of that tick does not reach it). Planted diff, two Edits: (1) in the slot loop, the block `if let
  Some((back, _)) = &c.held_back {` / `let d = one_tick(c.speed_cm, back.buttons, back.move_xy);` / `c.est = [c.est[0]
  + d[0], c.est[1] + d[1]];` / `}` removed (its two comment lines kept); (2) old `// Phase 0's inputs first: the World
  applies`, new `for c in clients.iter_mut() {` / the same block / `}` / that comment. te2-plant-rg.log
  01:26:09-01:27:04 UTC, -x, --no-fail-fast, exit 101: FAILED by name FX-1's test (replay.rs:886, "seed 2, slot 6,
  tick 1000225 (carried on 1000225), seq 226", left [1019, 0], right [1039, 0]: e = 225 = slot 6's 3b re-home, 6 x
  1,800 / 48); lib 5 + 1 failed. Beside it: d7 at :382 (ES's digest) only; world 30 + 1 ignored (t_m1 passes). This is
  the case count 6 stands for (11 on seed 2), and FX-1 bites on it. Restored by two Edits; sha256 all 17 OK.
- [14] Own plant R-D (FX-1's non-vacuity): no command is ever held back (the hold arm sends it). Planted diff
  (replay.rs:586): old `0 => c.held_back = Some((cmd, rewind)),`, new `0 => {` / `input.commands.push((cmd,
  rewind));` / `c.est = after;` / `}`. te2-plant-rd.log 01:27:23-01:28:17 UTC, -x, --no-fail-fast, exit 101: FAILED
  by name FX-1's test at its count assertion (replay.rs:925, "not vacuous: held back: 0"; its line "sky aims checked
  953622; crouched 94085; held back 0; ... re-sped 510227; ... lost 15327; ... carried ... 0": every aim checked
  still matched, as it should); lib 5 + 1 failed. Beside it: d7 (:351), the canonical-order test (tests/replay.rs:510,
  "0 ticks with a slot's two commands"), a_m1 (:940). FX-1's count asserts bite. Restored by Edit; sha256 all 17 OK.
- [15] Boundary probe R-F (what FX-1 does not hold, by R2's design): the generator's estimate before a slot's first
  placement wrong (old `est: home_of(s).map(|k| k as f32 / 100.0),`, new `est: [0.0, 0.0],`, replay.rs:400).
  te2-plant-rf.log 01:28:35-01:29:29 UTC, -x, --no-fail-fast, exit 101 (warning: unused `s`): FX-1 PASSES (lib "6
  passed; 0 failed"); t_m1 passes; d7 fails (:351, SS's after-tick pins) and a_m1 (:940). As R2 rules ("the slot's
  checks start at its first spawn or teleport"): before it the recording says nothing of where the soldier is, and
  the World starts each soldier on its team's spawn points (world.rs:737-749), not at home, so the generator's
  starting estimate is already open-loop; 3b places every ES slot within the first 60 s (slot 47 at e = 1,762), all
  inside the warm-up the judged window skips. Not a finding. Restored by Edit; sha256 all 17 OK.
- [16] The three-seed test once by name (te2-three-seeds.log 01:29:59-01:30:29 UTC, -x, `--ignored --exact
  --nocapture`, runner exit 0; "Compiling npt-sim (this copy)" x1): "test result: ok. 1 passed; 0 failed; 0 ignored; 0
  measured; 30 filtered out; finished in 29.42s". Its "ES seed" lines: 21; end-three-seeds.log's: 21; `diff` of the
  two (process substitution, no file written): none. FX-1 and FX-2 change no recording. MET. (Process note: my first
  try put the grep output into scratch files by shell redirect; the guard refused the whole command before it ran,
  nothing written; re-run without redirects.)
- [17] check.mjs, the one counted run (te2-check.log 01:30:44-01:35:39 UTC, -x, runner exit 0; `cargo test --locked
  --workspace --no-run` and the same `--all-features` first in the same runner call: "Compiling npt-sim (this copy's
  path)" x2, every npt-* crate x2 from this copy; idle 0 s, 10 GB, HEAD dad9566; no other cargo or rustc at the end):
  PASS fmt 0.4 s; PASS clippy 3.2 s (both builds); PASS clippy-wasm32 0.9 s (both); PASS doc 46.5 s (both builds, host
  and wasm32, warnings as errors); PASS test 234.4 s "measured build: 368 tests passed, 0 failed, 0 reported a skip;
  bites build: 415 tests passed, 0 failed, 0 reported a skip"; PASS deps 0.7 s (109 crate files, 1 cargo config); PASS
  refs 0.0 s (656 tracked files); PASS marks 0.6 s (81 marks, drafts 626df678, rows unchanged); PASS self 1.2 s "52 of
  52 tests passed"; PASS redact 0.8 s (10 of 10; 183 items, 0 findings); "check: all 10 steps pass". Deltas on round
  1's 367 / 414 / 52: measured +1 (368), bites +1 (415), self +0 (52) = R2's expected 368 / 415 / 52 and the
  builder's fx-check1/2. MET.
- [18] Test mutation M-1 (after check.mjs; is FX-1's held-back branch load-bearing?): old `let from = if held {
  before[k] } else { now[k] };`, new `let from = if held { now[k] } else { now[k] };` (replay.rs:~868, the test only).
  FX-1 by name, -x (te2-mut-m1.log 01:36:51-01:36:53 UTC, exit 101; warning unused `before`): FAILED (replay.rs:884,
  "seed 2, slot 6, tick 1000224 (carried on 1000225), seq 225", left [620, -15], right [1019, 0]: the held-back
  command whose carrying tick re-homed slot 6). The aim-from-its-own-tick rule is load-bearing, in the 11 cases count
  6 counts (elsewhere `before` = `now` for a held slot, which sent nothing on its own tick). Restored by Edit; sha256
  all 17 OK (01:37:00). The counted run in [17] was on these same bytes.
- [19] Netcode round-2 findings settled: NC2-L1 CONFIRMED by run ([9]: R-A passes the whole measured suite, FX-1
  fails it on seed 1 at the netcode's case; seeds [1, 2] run in 3.09 / 3.59 s). [5] (the clamp never bites: 400-799
  against 10,000): by arithmetic, agreed; no run changes a recording. [9] (count 5's flag after a re-home): wording
  nit, agreed no finding. [4]: out of scope, agreed.
- [20] End: `sha256sum -c r2-start.sha256` from the copy's root, all 17 OK (01:35:43 after check.mjs, and 01:37:00
  after M-1's restore); no PLANT, PROBE or `held_speed` text under crates/sim; git status the same 5 M + 1 ??; HEAD
  dad9566; diff stat 5 files, +663 / -146 (= the builder's end). Byte-identical to [1]. Nothing committed; nothing
  edited in the copy beyond restored plants, probes and M-1. No failure I did not plant: no TE2-5 entry.

## Closures
- TF-M1: CLOSED. FX-1 is the deterministic, exact reckoning test TF-M1 asked for (ES seed 2, every sky aim's x, y
  against the recording's own inputs integrated by the World's move rule, truncated to cm; MoveSpeed clamped,
  held-back moved when carried, lost never, placements resetting), counted in both builds (check.mjs +1 / +1), 1.6 /
  1.8 s. P1 (= Q1) and P3 (= Q2) fail it by name ([4], [5]), as do Q3-Q5 and my R-B, R-C, R-D, R-E, R-G; under Q4,
  Q5, R-B, R-C and R-G t_m1 passes and only D-7's pin failed beside it. Residual gap: NC2-L1 (Low).
- NF-M1's wording point (FX-2 (a)): CLOSED. `ES_AIM_ONE_IN`'s doc = round 1's + T3 (a) exactly ([2]); it no longer
  reads 40's 0.080 as a margin, and names RM. (The report half, T1, is outside this round.)
- NF-L2's wording point (FX-2 (b)): CLOSED. "sets its 10 gameplay events a second" replaced by "tunes its gameplay
  events toward 10 a second" and T3 (b) inserted, exactly ([2]); widest line 120; rustdoc PASS in [17].

## Findings
- None new from the fix pass. NC2-L1 (the netcode-architect's, Low, not before merge) CONFIRMED by run ([9]): count 6
  lumps re-homed and re-sped carries; seed 2 has no re-sped carry, so a generator that moves a held-back command at its
  hold-time speed passes the whole measured suite. Cheapest proof: R-A's three Edits ([9]) and te2-plant-ra.log /
  te2-plant-ra-seed1.log. Fix (follow-up): split count 6 and add seed 1 (te2-probe-seeds12.log: 3.09 / 3.59 s, every
  count above 0, count 6 24).
- Not findings: R-F ([15], the estimate before a slot's first placement: outside what the recording says, as R2
  rules); FX-1's tick assert (inside the test, defines "held").

## Verdict
APPROVE WITH FINDINGS (Lows only). FX-1 and FX-2 are exactly R2's and T3's, nothing else changed; every round-2 item
proven by its own run here. One Low open (NC2-L1, the netcode-architect's, confirmed by run), a follow-up; none new.
DONE (2026-11-12 01:38 UTC)
