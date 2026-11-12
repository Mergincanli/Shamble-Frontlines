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
