# U4d-F build notes: the estimate scene's fix (N2-M1, N2-M2, T2-L1)

Builder: workflow agent (general-purpose, opus, xhigh), isolated worktree
/home/user/Shamble-Frontlines/.claude/worktrees/wf_9ce88fa3-452-1. The harness made it at main (bb2d817), which does not
hold 39f4290; it was clean, so it was put on the spike head with a new local branch (`git switch -c u4df-build dad9566`;
dad9566 differs from 39f4290 only under .checkpoints/, and its .claude/, CLAUDE.md, .github/ and tools/gate.mjs equal
main's). `git merge-base --is-ancestor 39f4290 HEAD`: yes. Plan: the advisor's U4d close ruling
(advisor-u4d-close.md, B1.1, B1.2, B1.3, read whole with its "What I checked" end). Files: crates/sim only (world.rs,
scene.rs, replay.rs, tests/world.rs, tests/replay.rs). Write/Edit only; commit nothing. Shared target
(.gate/shared-target), offline; counted runs idle-waited, touched, 2 s toucher, provenance pre-run.
Base sha256 (first 16): world 2f3cde3d, scene c452d329, replay 3b80b11b, tests/world 721c1c96, tests/replay e483603f;
unchanged files: greybox 2d75cb0b, grid a0e24c08, hash 4663b2a8, history f11e9b59, lib 396e663d, rng d6b61bbe, seam
0c41b406, ticks 5d852f1a, trajectory 2c0968b3, tests/seam 4900af4e, sim Cargo.toml fb98df48, check.mjs 7eddc285,
workspace Cargo.toml 75b6c160, Cargo.lock 3da1e15f. Base counts (39f4290): measured 365, bites 412, self 52.

## Plan (B1.2's steps, in order; each step runs npt-sim's suites in both builds; a pin moves only in its own step)
0. T2-L1's two unit tests in world.rs's module: (a) `part_center` at hull yaw 90 degrees maps offset (+120, 0, 100) cm
   to hull + (0, +120, 100) within 1 cm; (b) phase 3's soldier boxes equal `hit_volumes_from` of the post-move records,
   and a soldier that moved out of its old box is hit at its new place by a no-Rewind shot in the same step. Plants O1
   (s terms' signs flipped), O2 (yaw ignored), O3 (phase 3 skips the refresh): each fails one by name. No pin moves.
1. One home for the move rule: `pub(crate) fn move_velocity(move_speed, buttons, move_xy, gain) -> [f32; 2]` in
   world.rs (crouch halves; sprint x 3 / 2 only when not crouching and not holding fire; the vector scaled to 127;
   `gain` keeps PM-0001-41/server-speed-1pct's x 1.01 exactly where it was, so the bites build is bit-identical too).
   `run_command` calls it; behaviour-neutral: every test and D-7 pin unchanged.
2. N2-M2 (B1.3): `Scene::fight_vehicles` (pub(crate); SS 4, ES 15): the first that many hulls loop 5 to 14 m round the
   cluster's centre (radius 5 + 9 v / (n - 1): SS's 5, 8, 11, 14 m bit for bit), the rest 40 + 3v m as now. Only ES's
   D-7 pin (its world before the first tick) moves.
3. The reckoning (one generator, one model; `Recording::GENERATOR` 1 -> 2): each tick's phase-0 inputs are drawn first
   (the World applies them before the tick's commands): the scripted MoveSpeed recorded per slot, brought into range
   as `apply_inputs` brings it; teleports and spawns reset the estimate. The estimate then moves by `move_velocity` at
   that speed with the command's own buttons and vector, once per command the World runs this tick: a held-back
   command moves it when it is sent (beside the next), a dropped one never. D-7's recordings and SS's after-tick pins
   re-pinned (every scene's recording changes); D-6 and the CO/TC tests pass unchanged.
4. The measurement (the acceptance): seeds 1, 2, 3, ES, 21,600 ticks from tick 1,000,000; window = the last 18,000
   ticks. Per seed: the four rates over the window, the two 300 s halves' event rates, the twelve 60 s event rates,
   T-M1's 300 s from tick 0 beside them, X2's form (ticks 0-17,999, seed 2) beside 7.578; 48 soldiers and 15 vehicles
   on every tick; every ES hull within 1,400 cm. 3b (the bounded re-home, ES only) only if the drift check fails.
5. `aim_one_in` (`ES_AIM_ONE_IN`) set from step 4's three seeds; step 4's checks pass at the value chosen. If no
   setting puts every seed's window and both halves in band: STOP and report the numbers (never move a band).
6. Tests: t_m1 becomes seed 2's window test (21,600 ticks; window bands, both halves, hulls, 48/15); the three-seed
   form is an `#[ignore]` test run once by name. Plants: the old reckoning (scene speed only) fails t_m1's drift check;
   one ES hull at 40 m fails the hull bound.
7. Docs: scene.rs's measured numbers, the "10 gameplay events a second" lines name the window; the grenade-fuse doc
   (B1.1's words); replay.rs's sky-aim comment.
End: fmt --check; clippy -D warnings host and wasm32, both builds; check.mjs twice, counted; final sha256.

## Progress
- Builder for steps 0-1 (a fresh one; the first was cut before any Progress line). Checked at start: branch
  u4df-build at dad9566; `git status`: only crates/sim/src/world.rs changed (+100 / -2) and this file untracked; the
  diff equals the saved wip patch (.checkpoints/netcode-proof-test/wip/u4df.patch at 4ad80a9, `diff` but the index
  line: same). scene, replay, tests/world, tests/replay sha256 = the base above; world.rs ae9ca4f8 (base 2f3cde3d).
  The partial edit is step 0 only: the module doc names T2-L1, `use crate::seam::{AttributeChange, Placement}`, and
  the two tests (a) `part_center_turns_a_parts_offset_by_its_hulls_yaw` (offset (+120, 0, 100) -> hull + (0, +120,
  100), and (0, +120, 100) -> hull + (-120, 0, 100), within 1 cm, at grid yaw 511 = 90 degrees) and (b)
  `a_shot_with_no_rewind_meets_a_soldier_where_this_ticks_move_put_it` (TC, one step: shooter and target teleported,
  the target at 100 m/s runs 3.33 m east, a no-Rewind shot hits it at its new chest and misses at its old). Nothing
  in it belongs to another step: kept whole, nothing undone. (b) takes B1.2's shot form ("or" the volumes form); the
  plan line above says "and", narrowed here to B1.2's words.
- **Step 0 DONE** (T2-L1's two tests; no pin moved). Logs under /home/user/Shamble-Frontlines/.gate/u4df/.
  - Suites, -x, 21:14:44-21:16:47 UTC (s0-suites.log, "Compiling npt-sim (this copy's path)" x2): measured 49 pass
    (lib 5, replay 6, seam 8, world 30; base 47 = lib 3 + 44), bites 57 pass (lib 5, replay 7, seam 10, world 35; base
    55), 0 failed, 0 ignored; both new tests named "ok" in both builds. `cargo fmt --all --check` exit 0 (s0-fmt.log).
  - Plants, each by Edit over world.rs ae9ca4f8, the whole measured suite --no-fail-fast, restored by Edit, sha256 back
    to ae9ca4f8d7d2d116 each time; diffs beside the logs (s0-plant-o1/o2/o3.diff):
    - O1 (s terms' signs flipped, part_center), 21:17:15-21:18:10 (s0-plant-o1.log): FAILED `part_center_turns_a_parts_
      offset_by_its_hulls_yaw` ("offset [120, 0, 100] at yaw 90 degrees: [1000, -620, 100], not [1000, -380, 100]")
      and d7 (tests/replay.rs:351); the rest pass (lib 4, replay 5, seam 8, world 30).
    - O2 (c = 1, s = 0), 21:18:28-21:19:19 (s0-plant-o2.log): FAILED the same test ("[1120, -500, 100], not [1000, -380, 100]") and d7;
      the rest pass.
    - O3 (present_volumes' refresh loop runs 0 times), 21:19:31-21:20:24 (s0-plant-o3.log): FAILED `a_shot_with_no_rewind_meets_a_
      soldier_where_this_ticks_move_put_it` ("a shot at the target's new chest [-5667, -500, 140] hits it") and d7;
      the rest pass.
  - After the plants, one comment edit in test (b): the earlier builder's "plant O3 fails both" said more than the
    run shows (the test stops at its first assertion), so it now reads "misses it at its new chest". Comment only;
    world.rs now 0193e01b (step 1's suite runs cover it).
- **Step 1 DONE** (one home for the move rule; behaviour-neutral, no pin moved). world.rs only:
  - Added `pub(crate) fn move_velocity(move_speed: u32, buttons: u16, move_xy: [i8; 2], gain: f32) -> ([f32; 2], bool,
    bool)` after `part_box`: the old run_command lines 1293-1320 moved as they were (crouch, fire held, sprint only
    when not crouching and not holding fire, speed_cm, the scale above 127, the velocity), with `speed = speed_cm as
    f32 / 100.0 * gain`. It returns the velocity (m/s) and the stance (crouch, sprint), because run_command's record
    flags read the same two values: so the sprint rule has one home, not two, and no second function was added.
  - run_command: keeps `fire_held` (the trigger reads it at the weapon) and the bite's `#[allow(unused_mut)]`/cfg
    block, now setting `gain` (1.0, or 1.01 under PM-0001-41/server-speed-1pct for the authority), then `let (v,
    crouch, sprint) = move_velocity(s.move_speed, c.buttons, c.move_xy, gain);`. Every later line is unchanged.
    Bit-identity by reading: the measured build multiplies by 1.0 (exact for every finite f32); the bitten path is
    (speed_cm / 100) x 1.01, the old `speed *= 1.01` exactly. No World line changed beyond moving the rule and the
    call (the rust-reviewer trigger of B1.2 is not reached; the reviewers judge).
  - Suites, -x, 21:23:07-21:24:55 UTC (s1-suites.log, "Compiling npt-sim (this copy's path)" x2): measured 49 pass
    (lib 5, replay 6, seam 8, world 30), bites 57 pass (lib 5, replay 7, seam 10, world 35), 0 failed; D-7 and every
    pin unchanged (no test file touched). Lint, 21:25:14-21:25:23 (s1-lint.log): `cargo fmt --all --check`; clippy
    -D warnings npt-sim host --all-targets and wasm32, measured and --all-features; rustdoc -Dwarnings --no-deps both
    builds: all exit 0, "Checking npt-sim (this copy's path)" x4, "Documenting" x2.
  - For step 3's builder: the generator does not call it yet (that is step 3's change: calling it at step 1 would move
    the recordings). The doc says "read by [`World`]'s commands"; step 3 adds the generator to it. Call form:
    `crate::world::move_velocity(speed, buttons, move_xy, 1.0).0` (the module is private in lib.rs; pub(crate) reaches
    replay.rs).
  - Files now (sha256, first 16): world e57308405dee7aa9; scene c452d329, replay 3b80b11b, tests/world 721c1c96,
    tests/replay e483603f (= base). `git status`: world.rs modified, this file untracked; nothing committed.
- Builder for steps 2-3 (a fresh one). Checked at start: branch u4df-build at dad9566; `git status`: world.rs modified
  (+141 / -24) and this file untracked; the diff holds step 0's module doc, import and two tests and step 1's
  `move_velocity` and its call in run_command, nothing else; sha256 world e57308405dee7aa9, scene c452d329, replay
  3b80b11b, tests/world 721c1c96, tests/replay e483603f, and every unchanged file as the base list above. The copy holds
  what steps 0-1 record.
- Step 2, the edit (scene.rs, world.rs): `Scene::fight_vehicles: u8` (pub(crate), after `large_vehicles`; SS 4, ES 15,
  CO 0, TC CO's), its doc, and ES's doc names it ("all 15 loop inside the fight: ... B1.3"). World::new: `let fight =
  scene.fight_vehicles; radius = if v < fight { 5.0 + 9.0 * f32::from(v) / f32::from(fight.saturating_sub(1).max(1))
  } else { 40.0 + 3.0 * f32::from(v) }`; the comment names B1.3. SS bit for bit: 9 v / 3 is exact for v = 0..3 (0, 3,
  6, 9), as 3 v was. ES: 5 + 9 v / 14 m, 5.0 to 14.0 m.
  - First run, -x, 21:30:15-21:31:07 UTC (s2-suites.log; measured only: the runner stops at the first failing
    command): lib 5 ok, seam 8 ok; FAILED d7 (tests/replay.rs:331, "ES", left (1944, 8237017275248454161), right (1944,
    8780045697152956090)): the step's named re-pin, ES's world before its first tick; FAILED t_m1 (tests/world.rs:1060,
    "gameplay events: 7250"; its line "T-M1: ES over 300 s: shots 70840, gameplay events 7250, spawns 21120 (1066
    thrown) thousandths a second; live projectiles 32300 thousandths a tick"; before step 2: 9.85). World 29 others
    ok, replay 5 others ok.
  - **TE2-5**, re-run at once, each build in its own runner call, --no-fail-fast, nothing changed (s2-rerun-measured.log
    21:31-21:32, s2-rerun-bites.log): the same two tests fail in both builds with the same values (d7 ES left (1944,
    8237017275248454161); t_m1 "gameplay events: 7250", the same T-M1 line); measured lib 5, replay 5 + 1 failed, seam
    8, world 29 + 1 failed; bites lib 5, replay 6 + 1 failed, seam 10, world 34 + 1 failed. Deterministic. Cause of t_m1:
    the only change is ES's hull radii (SS's are bit-identical: D-6 passes, D-7's SS pin passes before its ES one), so
    the 15 hulls now in the fight take the drop: their parts are cover (a part hit is no gameplay event) and block moves,
    as the ruling says ("both move the event rate", B1.3 Order). t_m1 is the old window at the old `aim_one_in` = 11:
    step 5 sets `aim_one_in` and step 6 rebuilds t_m1. Its band is not moved, and t_m1 stays failing until then (the
    builders of steps 4-7 should know). Named here so the main session and the advisor can rule on it; not retried.
- **Step 2 DONE** (N2-M2, ES's vehicles in the fight; one pin moved, ES's world before its first tick).
  - Re-pin (its one cause, step 2's radii): tests/replay.rs d7's `(Scene::estimate(), 1_944, 8_780_045_697_152_956_090)`
    -> `8_237_017_275_248_454_161` (the dump's 1,944 words unchanged). No other pin touched.
  - Suites after the re-pin, -x, each build in its own runner call (s2-repin-measured.log, s2-repin-bites.log, "Compiling
    npt-sim (this copy's path)" once each): measured lib 5 ok, replay 6 ok (d7 passes: SS's, CO's and TC's
    before-first-tick pins, SS's after-tick pins at 1, 30 and 300 and the three recording digests all unchanged), seam 8
    ok, world 29 ok + t_m1 FAILED (7250, as above); bites lib 5, replay 7, seam 10 ok, world 34 ok + t_m1 FAILED (7250).
    D-6 (SS's hull bounds, 1,401 / 5,199 cm) passes in both. 0 ignored.
  - Lint 21:36:19-21:36:25 UTC (s2-lint.log): fmt --check; clippy -D warnings host --all-targets and wasm32, measured
    and --all-features; rustdoc -Dwarnings both builds: all exit 0, "Checking npt-sim (this copy)" x4, "Documenting" x2.
  - Files (sha256, first 16): world 397fd1f806740ea0, scene a79408b38c192586, tests/replay 5418dbd8608751b7; replay
    3b80b11b and tests/world 721c1c96 (= base).
- Step 3, the edit (replay.rs; world.rs's `move_velocity` doc only). `Recording::GENERATOR` 1 -> 2 (its doc names
  U4d-F). `synthetic`: imports `MOVE_SPEED_MAX_CM_S` and `crate::world::move_velocity`; `Client` gains `speed_cm`
  (the scene's move speed, then each MoveSpeed it emits, `.min(MOVE_SPEED_MAX_CM_S)` as apply_inputs); the old
  `speed` (scene speed only) is gone; `one_tick(speed_cm, buttons, move_xy)` = move_velocity(.., 1.0).0 times 1 /
  TICK_HZ (= world.rs's DT). Per tick: the MoveSpeed and the spawn/teleport draws now come first (the World applies
  phase 0 before the commands), the spawn/teleport resetting the estimate to home as before; per slot: a held-back
  command's move is added first (its own buttons and vector at the speed held now), then the heading, then `after` =
  estimate + this command's move; the sky aim reads `after` (where the World fires it from); the estimate becomes
  `after` only when the command is sent (now, or beside the held one), never for a held-back one (it moves next tick)
  or a lost one. A doc paragraph states the rule and what the reckoning cannot see (blocked moves, deaths and
  respawns, the snap to the wire grid). The sky-aim comment's words are step 7's (not touched).
  - First run, -x, each build in its own runner call (s3-first-measured.log, s3-first-bites.log): both builds fail
    only d7 and t_m1. d7 (tests/replay.rs:351): SS's after-tick pins at 1 and 30 unchanged, at 300 left (300, 6612,
    3337416615378533976), right (300, 6612, 923332457071349262). Why 1 and 30 hold: the draw stream is the old one up
    to tick 157, the first phase-0 draw (311 k + 157), and SS always fires (no sprint) and has no sky aim, so the
    commands are the same until then. t_m1: "gameplay events: 8566" (its line: shots 70786, events 8566, spawns 21120
    (1133 thrown), live 32300); the same cause as step 2's note; band not moved. Every other test passes unchanged:
    D-6, D-1, D-2, carry 7/9/13/15/38, h1, n_m5, a_m1, w1_s1, the CO and TC tests (d8-d10 and the rest). measured lib
    5, replay 5 + d7, seam 8, world 29 + t_m1; bites lib 5, replay 6 + d7, seam 10, world 34 + t_m1.
  - Re-pins so far (tests/replay.rs d7, cause: step 3's generator): (300, 6_612, 923_332_457_071_349_262) ->
    3_337_416_615_378_533_976; "(generator version 1)" -> "(generator version 2)"; `assert_eq!(Recording::GENERATOR,
    1)` -> 2. The digests next.
- **Step 3 DONE** (the reckoning; `Recording::GENERATOR` 2; D-7's recordings and SS's after-tick pin re-pinned, one
  cause: step 3's generator).
  - D-7 alone (s3-d7-digests.log, measured): the three digests, left [(16694921, 35974458070191737), (16695272,
    2102134247371836914), (799693, 9126256229328065459)], right the old [(16697066, ...191497), (16697144,
    ...984890), (798759, ...409324)]. Re-pinned: SS (16_697_066, 2_695_722_420_930_191_497) -> (16_694_921,
    35_974_458_070_191_737); ES (16_697_144, 7_136_420_146_508_984_890) -> (16_695_272, 2_102_134_247_371_836_914); TC
    (798_759, 3_177_023_422_367_409_324) -> (799_693, 9_126_256_229_328_065_459). No other file quotes the old sizes or
    pins (grep over the spike: none).
  - Suites after the re-pins, -x, each build in its own runner call (s3-repin-measured.log, s3-repin-bites.log), and
    again on the final files after the probes (s3-final-measured.log 21:47-21:48:37, s3-final-bites.log -21:49:35 UTC;
    "Compiling npt-sim (this copy's path)" once each): measured lib 5, replay 6 (d7 passes), seam 8 ok, world 29 ok + t_m1
    FAILED ("gameplay events: 8566"); bites lib 5, replay 7, seam 10 ok, world 34 ok + t_m1 FAILED (8566). So 48 of 49
    and 56 of 57 pass, 0 ignored; D-6 and every CO and TC test pass with their assertions unchanged (B1.2 step 3).
  - Lint 21:43:38-21:43:44 UTC (s3-lint.log): fmt --check; clippy -D warnings host --all-targets and wasm32, measured
    and --all-features; rustdoc -Dwarnings both builds: all exit 0, "Checking npt-sim (this copy)" x4, "Documenting" x2.
  - Read, for the ruling's "not checked" item (a held-back command's move on its delivery tick): `World::step`
    (world.rs:2108 apply_inputs, then :2113-2122) runs every command of `input.commands` through `run_command` in
    order, each one tick's move (:1344-1345, `move_velocity` then `collide` by v x DT); the clamp is :1157. A command
    held back at e is delivered at e + 1 before e + 1's own, so it moves the soldier at e + 1, first: what the
    reckoning now does. A dead soldier's command does not move it (:1330), which the reckoning cannot see.
  - **Probe (diagnostic, not a step's evidence; for step 4's builder)**: a temporary `#[ignore]` test in tests/world.rs
    (s3-probe.diff) compared each ES sky aim (= the generator's estimate after the command's move, x and y) with the
    World's soldier record after the step, alive soldiers only, per minute over 21,600 ticks from 1,000,000, seeds 1-3.
    Removed by Edit after; tests/world.rs sha256 back to 721c1c966f7ad603 (= base).
    - As built (s3-probe.log): median error 7.1, 7.2, 8.3 m in minute 0 (seeds 1, 2, 3), growing to 22.1, 19.8, 22.2 m
      in minute 11 (p90 34.4, 31.2, 37.0 m). The error still grows.
    - With the World's collision taken out (diagnostic plant in run_command, s3-probe-nocollide.diff, restored by Edit,
      world.rs sha256 back to 6e01101323ce57a1): median 2.7, 2.9, 2.6 m in minute 0 (the spawn grid's offset from home,
      which the estimate does not start from: about 3 m for each team's first 16 grid points), then a plateau of about
      5-7 m from minute 3 or 4 to 11 (seed 1: 7.6, 7.0, 6.6, 5.7, 5.0, 5.5, 7.3, 7.3, 7.2 m for minutes 3-11; seed 2:
      5.5-7.0 m; seed 3: 5.3-6.8 m), max 17-21 m: bounded (deaths and respawns, and the 1 cm snap, which the reckoning
      cannot see). So the reckoning follows the commands. What still grows as built is blocked moves (most likely the
      15 ES hulls now in the fight, 60 part boxes in the ring the soldiers walk in, plus the greybox). The ruling's
      words for this case: "the measurement decides whether they need the re-home" (B1.2), 3b if step 4's drift check
      fails. Step 4's measurement decides; nothing here moves a band or sets a value.
  - Files (sha256, first 16): world 6e01101323ce57a1, scene a79408b38c192586, replay 4889af09aa159ab0, tests/replay
    66bd0e2544a52a9d, tests/world 721c1c966f7ad603 (= base). `git status`: replay, scene, world, tests/replay
    modified; this file untracked; nothing committed. Every unchanged file as the base list (not touched).
  - For steps 4-7: t_m1 fails at 8.566 (300 s from tick 0 at `aim_one_in` 11: T-M1's figure, "reported, never used to
    set anything"); step 6 rebuilds it. The sky-aim comment (replay.rs, "ES only ... so their recordings are as
    before") is step 7's to reword: the other scenes' recordings changed at step 3 anyway. ES's doc paragraph in
    scene.rs (`Scene::estimate`) has one short line after step 2's insertion ("µs), 32 live projectiles at 0.44 spawns
    per player per second (21.12 a"): step 7 rewords that paragraph's "10 gameplay events" and can rewrap it then.
- End of the step 2-3 builder: steps 2 and 3 DONE, nothing committed; next is step 4 (the measurement) on these files.
- Builder for steps 4-5 (a fresh one, after a usage-limit cut). Checked at start: branch u4df-build at dad9566; `git
  status`: replay, scene, world, tests/replay modified, this file untracked; diff stat +240 / -72 over those four; the
  diffs hold steps 0-3 as recorded above and nothing else (read whole); sha256 world 6e01101323ce57a1, scene
  a79408b38c192586, replay 4889af09aa159ab0, tests/replay 66bd0e2544a52a9d, tests/world 721c1c966f7ad603 (= base). The
  copy holds what steps 0-3 record. Method for step 4: a temporary `#[ignore]` probe in tests/world.rs (public API
  only, t_m1's counting), one 21,600-tick run per seed from tick 1,000,000, bucketed per 60 s; T-M1's 300 s and X2's
  600 s are that run's first 9,000 and 18,000 ticks (the generator draws nothing from `ticks`, and the World's tick k
  reads only inputs up to k), checked against t_m1's own 8.566 at 1 in 11. Removed after, sha256 back to base.
- **Step 4, measurement A (after step 3, `aim_one_in` 11, no re-home)**: s4-probe-11.log (21:54:07-21:54:38 UTC, probe diff
  s4-probe.diff; exit 0, "test result: ok. 1 passed", "Compiling npt-sim (this copy's path)" once). Rates in units a
  second (live: a tick); the window is ticks 3,600-21,599 (600 s after the 2-minute warm-up). Bands: shots 68.4-75.6,
  events 9.5-10.5, live 30.4-33.6, spawns 20.064-22.176.

  | seed | window: shots / events / live / spawns | half 1 / half 2 events | T-M1 300 s from tick 0 | 600 s from tick 0 |
  |---|---|---|---|---|
  | 1 | 70.758 / **5.483** / 32.383 / 21.120 | **6.113 / 4.853** | 7.640 | 6.711 |
  | 2 | 70.746 / **6.076** / 32.383 / 21.120 | **6.676 / 5.476** | 8.566 (= t_m1's own) | **7.108** (X2's form; X2: 7.578) |
  | 3 | 70.786 / **4.800** / 32.383 / 21.120 | **5.086 / 4.513** | 6.726 | 5.546 |

  The twelve 60 s event rates (minutes 0-11):
  - seed 1: 11.500, 8.850, 7.483, 5.233, 5.133, 5.833, 6.883, 5.433, 5.833, 4.933, 3.816, 4.250
  - seed 2: 11.516, 9.350, 8.133, 8.000, 5.833, 5.333, 6.083, 5.833, 6.300, 4.700, 5.033, 5.516
  - seed 3: 8.666, 7.783, 7.050, 5.083, 5.050, 4.200, 4.050, 4.783, 4.716, 4.083, 4.900, 4.083
  Split of the window's events (seed 1 / 2 / 3): shot hits 2.283 / 2.438 / 2.195, blast hits 3.043 / 3.463 / 2.480,
  kills 0.156 / 0.175 / 0.125. Every tick of every seed: 48 soldiers, 15 vehicles; the farthest hull 1,400.6 cm from
  the centre (the 14.0 m loop on the 1 cm wire grid: 1,400 by D-6's truncating form, whose SS bound is 1,401).
  Verdict: shots, live projectiles and spawns in band; gameplay events far below band in every window, and the drift
  check FAILS (each seed's halves differ by 0.57-1.26 a second, and the rate decays from minute 0 to minute 11 in every
  seed). The drift is not a share question: a smaller `aim_one_in` scales both halves, and the halves' ratio (1.13-1.26)
  cannot fit both inside 9.5-10.5 (ratio at most 1.105). So 3b (the bounded re-home, ES only) applies, as B1.2 step 4
  says; then measure again. No band moved; no value set.
- **3b, the edit (replay.rs `Recording::synthetic` only)**: after the tick's random spawn/teleport draws, ES only
  (`scene.aim_one_in > 1`, the knob only ES sets above 1, as the sky aim reads it): for each non-plate slot k, when e
  mod 1,800 (`ticks_for_ms(60_000)`) = k × 1,800 / slots (integer division: floor), a recorded teleport to its home and
  its estimate reset to home; it draws nothing. A comment names B1.2 step 4's 3b. (The World ignores a teleport of a
  dead soldier, world.rs:1149; it respawns near home anyway.) Teleports have no order rule (canonical order covers
  commands only, world.rs:255).
- **Step 4, measurement B (3b, `aim_one_in` 11)**: s4-probe-3b-11.log (21:56:24-21:56:56 UTC, exit 0, "test result: ok. 1 passed", "Compiling
  npt-sim (this copy's path)" once).

  | seed | window: shots / events / live / spawns | half 1 / half 2 events | T-M1 300 s from tick 0 | 600 s from tick 0 |
  |---|---|---|---|---|
  | 1 | 70.565 / 11.095 / 32.383 / 21.120 | 11.186 / 11.003 | 11.590 | 11.356 |
  | 2 | 70.571 / 11.586 / 32.384 / 21.120 | 11.886 / 11.286 | 12.716 | 12.186 (X2: 7.578) |
  | 3 | 70.555 / 11.680 / 32.383 / 21.120 | 11.733 / 11.626 | 12.656 | 11.860 |

  The twelve 60 s event rates:
  - seed 1: 10.116, 14.116, 10.516, 11.650, 11.550, 10.833, 11.383, 10.516, 12.116, 10.766, 12.066, 9.550
  - seed 2: 13.116, 14.283, 13.000, 11.616, 11.566, 12.000, 11.250, 11.700, 12.583, 10.750, 10.300, 11.100
  - seed 3: 12.116, 13.450, 12.133, 12.933, 12.650, 9.983, 10.966, 11.816, 11.500, 11.050, 12.083, 11.683
  Window split (seed 1 / 2 / 3): shot hits 4.215 / 4.433 / 4.345, blast hits 6.525 / 6.791 / 6.961, kills 0.355 /
  0.361 / 0.373. 48 soldiers, 15 vehicles, hulls at most 1,400.6 cm on every tick. Verdict: drift bounded (halves
  0.11-0.60 apart, no trend across minutes 2-11); events above band in every window and half; the other three in band.
  Step 5 sets the share from here.
- **Step 5, the sweep (3b in place; `ES_AIM_ONE_IN` edited per value, the same probe; logs s5-probe-<N>.log, each exit 0,
  "1 passed", "Compiling npt-sim (this copy's path)" once)**. Events a second: windows seed 1 / 2 / 3; halves (1st/2nd)
  seed 1, 2, 3. Shots 70.55-70.61, live 32.383-32.384, spawns 21.120, 48 / 15 and hulls <= 1,400.6 cm at every value.
  - 11 (measurement B): 11.095 / 11.586 / 11.680; 11.186/11.003, 11.886/11.286, 11.733/11.626: all above.
  - 16: 10.441 / 10.591 / 10.963; 10.710/10.173, 10.736/10.446, 11.063/10.863: 5 of 9 above.
  - 26: 10.003 / 10.381 / 10.531; 10.173/9.833, 10.543/10.220, 10.623/10.440: 3 of 9 above.
  - 34: 10.166 / 10.073 / 10.435; 10.423/9.910, 9.946/10.200, 10.306/10.563: 1 of 9 above (seed 3's 2nd half).
  - 48: 9.718 / 9.725 / 9.978; 9.900/9.536, 9.770/9.680, 9.980/9.976: all 9 in band, the floor margin 0.036.
  - 40: 9.878 / 9.743 / 10.098; 10.176/9.580, 9.880/9.606, 10.220/9.976: all 9 in band; margins 0.080 (floor, seed
    1's 2nd half) and 0.280 (top).
  - 36: 10.048 / 9.638 / 10.408; 10.100/9.996, 9.673/9.603, 10.026/10.790: 1 of 9 above (seed 3's 2nd half).
  - 38: 10.141 / 9.833 / 10.256; 10.126/10.156, 9.906/9.760, 9.970/10.543: 1 of 9 above (seed 3's 2nd half).
  The share moves only the aimed-shot hits (about 2.5 + 15.7 / N a second, fitted on 26 and 34); blast hits (6.5-7.5)
  and kills (0.33-0.38) do not follow it, and each value's new draws reshuffle the halves by up to about ±0.4 (seed
  3's 2nd half: 10.563, 10.790, 10.543, 10.220 at 34, 36, 38, 40). With the seeds' mean near 10.0 (37-38), six halves
  at that spread fit 9.5-10.5 only about half the time, so a pass is a realisation, not a margin.
  - **Choice: 40.** The rule, stated before choosing: of the values that pass, the one with the largest smallest
    margin to either band edge. 40 and 48 pass; 40's smallest margin 0.080 > 48's 0.036, and 40's seed mean (9.906) is
    the nearer to 10.0. The sweep stops here (more values only select on noise). A whole 1-in-N passes, so the finer
    share (B1.1) is not needed. No band moved.
  - Confirmation on the files step 5 hands on (scene.rs at 40 = 5cb36660b7eff48e, replay.rs 91ed626c2d580641, world.rs
    6e01101323ce57a1, tests/world.rs with the probe 4f8dfa117abb9ed0, tests/replay.rs 66bd0e2544a52a9d):
    s5-confirm-40.log, exit 0, "1 passed"; its S4 lines equal s5-probe-40.log's (diff: none).
- Probe removed by Edit; tests/world.rs sha256 back to 721c1c966f7ad603 (= base). `ES_AIM_ONE_IN` put back to 11 by
  Edit, scene.rs back to a79408b38c192586 (= step 3's), so step 4's re-pin has one cause (3b) and step 5's its own.
- Step 4's suites (3b, aim 11), -x, each build in its own runner call: s4-3b-measured.log, s4-3b-bites.log ("Compiling
  npt-sim (this copy's path)" once each). Both fail the same two, every other test passes: measured lib 5, replay 5 +
  d7, seam 8, world 29 + t_m1; bites lib 5, replay 6 + d7, seam 10, world 34 + t_m1.
  - **TE2-5**, re-run at once, nothing changed, --no-fail-fast, each build (s4-3b-rerun-measured.log 22:06:47,
    s4-3b-rerun-bites.log 22:07:39 UTC): the same two, the same values. (1) `d7_the_hash_and_dump_vectors_are_pinned`,
    tests/replay.rs:382 (the recordings' digests; every earlier d7 assertion, SS's after-tick pins included, passes):
    left [(16694921, 35974458070191737), (16700849, 84585687253498977), (799693, 9126256229328065459)], right the
    pinned [.., (16695272, 2102134247371836914), ..]: only ES's recording changed (SS's and TC's draw nothing for 3b).
    Cause: 3b (step 4's change), the named re-pin. (2) `t_m1_the_estimate_scene_meets_pm_0001_2s_es_bands`,
    tests/world.rs:1060, "gameplay events: 12716" (T-M1's line: shots 70680, events 12716, spawns 21120 (1072
    thrown), live 32301), the probe's seed 2 300 s figure exactly. Cause: 3b raises the 300 s from tick 0 at aim 11;
    t_m1 is the old window at the old value, carried failing since step 2 (above) until step 5 sets the share and
    step 6 rebuilds it. Band not moved. Deterministic in both builds; not retried into green.
  - Re-pin (tests/replay.rs d7, its one cause 3b): ES's recording (16_695_272, 2_102_134_247_371_836_914) ->
    (16_700_849, 84_585_687_253_498_977). No other pin touched.
- **Step 4 DONE** (the measurement; 3b added because the drift check failed; one pin moved, ES's recording digest,
  cause 3b). Suites after the re-pin, -x, each build in its own runner call (s4-repin-measured.log 22:08:57-22:09:49,
  s4-repin-bites.log -22:10:46 UTC; "Compiling npt-sim (this copy's path)" once each): measured lib 5, replay 6 (d7
  passes), seam 8 ok, world 29 ok + t_m1 FAILED ("gameplay events: 12716", carried, above); bites lib 5, replay 7, seam
  10 ok, world 34 ok + t_m1 FAILED (12716). 48 of 49 and 56 of 57, 0 ignored; D-6 and every CO and TC test pass
  unchanged. Lint 22:10:53-22:10:58 UTC (s4-lint.log): fmt --check; clippy -D warnings host --all-targets and wasm32,
  measured and --all-features; rustdoc -Dwarnings both builds: all exit 0, "Checking npt-sim (this copy)" x4,
  "Documenting" x2. Files (sha256, first 16): world 6e01101323ce57a1, scene a79408b38c192586, replay 91ed626c2d580641,
  tests/replay e7bca77127fba437, tests/world 721c1c966f7ad603 (= base).
  - For step 6's builder: the window test can take the probe's counting (s4-probe.diff beside the logs; it splits
    shot hits from blast hits by the tick's Shot events). The hull bound needs D-6's truncating form (1,400.6 cm on the
    exact grid distance; D-6 holds SS to <= 1,401) or a 1,401 bound. Plant "one ES hull at 40 m" would show 4,000+.
- Step 5, the edit (scene.rs): `const ES_AIM_ONE_IN: u32 = 11` -> `40` (the constant only; its doc lines, :77-79 and
  `Scene::estimate`'s "one command in 11", are step 7's). scene.rs 5cb36660b7eff48e (= the confirmed file above).
  - First run, -x, each build in its own runner call (s5-first-measured.log 22:11:17-22:12:10, s5-first-bites.log
    -22:13:06 UTC): t_m1 now PASSES in both (world 30 ok, 35 ok; seed 2's 300 s from tick 0 at 40 is 10.440 by the
    probe, in band; t_m1 is still the old 300 s test, so this pass is not step 6's window test). Only d7 fails.
  - **TE2-5**, re-run at once, nothing changed, --no-fail-fast (s5-rerun-measured.log 22:13:13, s5-rerun-bites.log
    22:14:03 UTC): the same one test, the same values in both builds. `d7_the_hash_and_dump_vectors_are_pinned`,
    tests/replay.rs:382: left [(16694921, ..), (16700849, 15077317756489230056), (799693, ..)], right the pinned
    [.., (16700849, 84585687253498977), ..]: only ES's recording hash changed (its length did not: an aim is written in
    fixed width whichever way it points). Cause: step 5's value, the named re-pin. Every other test passes: measured
    lib 5, replay 5 + d7, seam 8, world 30; bites lib 5, replay 6 + d7, seam 10, world 35.
  - Re-pin (tests/replay.rs d7, its one cause step 5): ES's recording (16_700_849, 84_585_687_253_498_977) ->
    (16_700_849, 15_077_317_756_489_230_056). No other pin touched.
- **Step 5 DONE: `aim_one_in` = 40** (`ES_AIM_ONE_IN`, a whole 1-in-N; one pin moved, ES's recording hash, cause step
  5's value). Step 4's checks at 40, on these files (s5-probe-40.log = s5-confirm-40.log), every one in band:
  - windows (600 s after the 2-minute warm-up), shots / events / live / spawns: seed 1 70.578 / 9.878 / 32.383 /
    21.120; seed 2 70.586 / 9.743 / 32.384 / 21.120; seed 3 70.573 / 10.098 / 32.383 / 21.120;
  - halves' events: seed 1 10.176 / 9.580; seed 2 9.880 / 9.606; seed 3 10.220 / 9.976 (smallest margin 0.080, seed 1's
    2nd half above 9.5; largest value 10.220, 0.280 under 10.5);
  - twelve 60 s event rates: seed 1 8.583, 11.783, 8.733, 9.866, 10.783, 10.333, 11.166, 10.000, 9.616, 8.766,
    10.083, 9.433; seed 2 10.816, 11.733, 9.800, 9.633, 10.216, 9.516, 10.233, 10.333, 10.600, 8.916, 9.216, 8.966;
    seed 3 10.250, 10.900, 10.466, 10.866, 10.950, 9.000, 9.816, 10.350, 9.800, 9.350, 9.333, 11.050 (a single minute
    is not judged; it swings 8.6-11.8);
  - T-M1's 300 s from tick 0 (reported, never used to set anything): 9.950, 10.440, 10.686; X2's form (seed 2, ticks
    0-17,999): 10.180 events a second, beside X2's 7.578 (shots 70.640, live 32.342, spawns 21.120);
  - every tick of every seed: 48 soldiers, 15 vehicles, every hull at most 1,400.6 cm from the centre;
  - window split: shot hits 2.741 / 2.685 / 2.771, blast hits 6.785 / 6.715 / 6.973, kills 0.351 / 0.343 / 0.353.
  Suites after the re-pin, -x, each build in its own runner call (s5-repin-measured.log 22:15:12-22:16:02,
  s5-repin-bites.log 22:16:04-22:16:57 UTC; exit 0 both; "Compiling npt-sim (this copy's path)" once each): measured
  lib 5, replay 6, seam 8, world 30, all ok (49 of 49); bites lib 5, replay 7, seam 10, world 35, all ok (57 of 57); 0
  failed, 0 ignored. Lint 22:17:06-22:17:10 UTC (s5-lint.log): fmt --check; clippy -D warnings host --all-targets and
  wasm32, measured and --all-features; rustdoc -Dwarnings both builds: all exit 0, "Checking npt-sim (this copy)" x4,
  "Documenting" x2. Files (sha256, first 16): world 6e01101323ce57a1, scene 5cb36660b7eff48e, replay 91ed626c2d580641,
  tests/replay a5f123b8a2ddde1d, tests/world 721c1c966f7ad603 (= base); every unchanged file = the base list. `git
  status`: replay, scene, world, tests/replay modified; this file untracked; nothing committed. No public item added.
- For the main session, the advisor and steps 6-7 (end of the step 4-5 builder):
  - The pass at 40 is thin and partly luck: each value's draws move a half by up to about ±0.4 (seed 3's 2nd half
    10.563, 10.790, 10.543, 10.220 at 34, 36, 38, 40), and 34, 36 and 38 each failed on one half. A fourth seed, or any
    later change to the ES draw stream, can put a half out of band at 40. Step 6's seed-2 window test has margins 0.106
    (2nd half 9.606) and 0.243 (window 9.743).
  - The events are now mostly blasts (6.7-7.0 a second, which no share moves) plus about 2.5 shot hits a second that
    stay as the share grows: sky shots meeting soldiers on the exact home point after 3b's teleports, the event source
    the ruling foresaw (B1.2, "Re-home alone"). The share moves only about 0.4 a second at 40.
  - 3b's "ES only" switch reuses `aim_one_in > 1` (the knob only ES sets above 1, as the sky aim does), not a new
    scene knob: a new knob is not among B1.2's symbols.
  - The hull bound: 1,400.6 cm on the exact distance of the 1 cm grid (14.0 m loop); B1.3 says "within 1,400 cm",
    D-6 holds SS to <= 1,401 in its truncating form. Step 6 needs one of those forms (the advisor's word if neither).
  - TE2-5's stop clause: every npt-sim failure in steps 4-5 was a named, planned cause (3b's re-pin, step 5's re-pin,
    and t_m1 carried failing since step 2, now passing at 40); each was re-run at once, deterministic, recorded, and not
    retried into green. The group did not stop for them, as the step 2-3 builder did not; the main session rules.
  - Step 7's docs now stale: scene.rs:77-79 (measured at 1 in 11), :119 and `Scene::estimate`'s "10 gameplay events"
    and "one command in 11"; replay.rs's sky-aim comment ("so their recordings are as before", "T-M1"). t_m1 passes
    at 40 as the old 300 s test; step 6 rebuilds it as seed 2's window test.
- Builder for steps 6-7 and the End checks (a fresh one, after a usage-limit cut). Checked at start: branch u4df-build
  at dad9566; `git status`: replay, scene, world, tests/replay modified, this file untracked; diff stat +258 / -73 over
  those four; each diff read whole: steps 0-5 as recorded above and nothing else (step 0's two unit tests, step 1's
  `move_velocity` and its call, step 2's `fight_vehicles` and radii, step 3's reckoning and GENERATOR 2, 3b's re-home,
  `ES_AIM_ONE_IN` 40, D-7's five re-pins); sha256 world 6e01101323ce57a1, scene 5cb36660b7eff48e, replay
  91ed626c2d580641, tests/replay a5f123b8a2ddde1d, tests/world 721c1c966f7ad603 (= base): step 5's closing line. The
  copy holds what steps 0-5 record. check.mjs counts `passed` only (cargoCounts), so an `#[ignore]` test adds nothing.
- Step 6, the edit (tests/world.rs only): t_m1's body replaced; its name kept. New in the test file, private to it and
  serving only t_m1 and the ignored test: `struct EsRun` (`new(seed)`: the ES recording of 21,600 ticks from tick
  1,000,000, t_m1's counting per 60 s, the fewest soldiers and vehicles on any tick, the farthest hull by its integer
  squared distance, then the root; `rates(range)`, `line(range)`, `check()`), `ES_WINDOW` (minutes 2..12), `ES_HALVES`
  (2..7, 7..12) and `fn milli`. `check()` asserts in this order, each naming its seed and check: 48 soldiers and 15
  vehicles on every tick; the hull bound (D-6's form, whole cm rounded down, `<= 1_400`: B1.3's number; the exact
  maximum 1,400.6 cm is printed beside it, the 14.0 m loop's records on the 1 cm wire grid; D-6 holds SS to 1,401 in
  the same form); shots, live projectiles, spawns over the window; then one assertion for the gameplay events over the
  window and each 300 s half (the drift check), which prints all three. t_m1 = `EsRun::new(2)`, a print line, `check()`.
  `t_m1_seeds_1_2_and_3_meet_the_es_bands_and_report_them` (`#[ignore]`, with its reason): seeds 1-3, every seed's
  report lines printed before any `check()`. `f64::max` is on npt-sim's clippy ban list (clippy.toml:52), hence the
  integer maximum. tests/world.rs b2c3a8c8a072b490.
  - Lint 22:24:30-22:24:31 UTC (s6-lint0.log, after one rustfmt diff applied by Edit, s6-fmt1.log, and the `f64::max`
    fix): fmt --check; clippy -D warnings npt-sim host --all-targets, measured and --all-features: exit 0.
  - Suites, -x, each build in its own runner call (s6-suites-measured.log 22:24:36-22:25:32, s6-suites-bites.log
    22:25:37-22:26:40 UTC; "Compiling npt-sim (this copy's path)" once each; exit 0 both): measured lib 5, replay 6,
    seam 8, world 30 passed + 1 ignored (the three-seed test), 0 failed; bites lib 5, replay 7, seam 10, world 35 + 1
    ignored, 0 failed. No pin moved (tests/replay.rs untouched).
  - The three-seed test once by name, then t_m1 by name, measured, --nocapture (s6-three-seeds.log 22:26:44-22:27:27,
    exit 0, "1 passed" each; 30.5 s and 11.1 s). Its lines equal step 5's probe at 40 (s5-probe-40.log) on every figure:
    windows 9.878 / 9.743 / 10.098, halves 10.176/9.580, 9.880/9.606, 10.220/9.976, the twelve minutes, T-M1's 9.950 /
    10.440 / 10.686, X2's form 10.180 for seed 2 (and 9.963, 10.175 for seeds 1 and 3), 48 / 15, hulls 1,400 in D-6's
    form (1,400.6 exactly). t_m1's line: "T-M1: ES seed 2 over the window (600 s after 120 s): shots 70.586 a second,
    gameplay events 9.743 a second, live projectiles 32.384 a tick, spawns 21.120 a second (2080 thrown); gameplay
    events 9.880 and 9.606 a second over its halves; hulls at most 1400.6 cm out".
  - Plants, each by Edit, the whole measured suite --no-fail-fast, -x, restored by Edit, sha256 checked; diffs beside
    the logs:
    - P1, the old reckoning (scene speed only: every command moves the estimate at the scene's 600 cm/s with no
      buttons, sent, held or lost alike, no extra move for a held-back one; 3b and 40 kept), over replay.rs 91ed626c
      (s6-plant-p1.diff, s6-plant-p1.log 22:28:15-22:29:17 UTC): FAILED `t_m1_the_estimate_scene_meets_pm_0001_2s_es_
      bands` at its events assertion ("seed 2, gameplay events and the drift check: 9298 thousandths a second over the
      window, 9240 and 9356 over its 300 s halves (each in 9,500-10,500)"); also d7 (SS's after-tick pin at 300:
      (300, 6624, 8955962681012353194), every recording changes) and `a_m1_the_authoritys_volumes_are_the_function_of_
      its_records` (tests/world.rs:940, "not vacuous: dead soldiers' boxes are not live": P1 changes SS's recording
      too, and seed 5's 1,200 ticks then hold no death; a plant side effect on a_m1's precondition, which passes on the
      real code in every run). The rest pass (lib 5, replay 5, seam 8, world 28). Reading: under 3b the two reckonings'
      errors both reset every 60 s, so P1 shows as a level (halves 0.12 apart, both under 9.5), not a decay across the
      window: the drift check's assertion is what fails, by level. The ruling wrote this plant before 3b was needed;
      named here for the reviewers. replay.rs back to 91ed626c2d580641.
    - P2, ES's last hull (v = 14) at 40 m (s6-plant-p2.diff, s6-plant-p2.log 22:29:55-22:30:53 UTC): FAILED t_m1 at
      the hull bound ("seed 2, the hull bound: a hull 4000 cm from the cluster's centre (4000.7 cm exactly)"); also d7
      (ES's world before its first tick: (1944, 12206132157615861571)). The rest pass (lib 5, replay 5, seam 8, world
      29). world.rs back to 6e01101323ce57a1.
- **Step 6 DONE** (t_m1 is seed 2's window test; the three-seed form is ignored and ran once by name; two plants fail by
  name; no pin moved). Files: tests/world.rs b2c3a8c8a072b490; world 6e01101323ce57a1, scene 5cb36660b7eff48e, replay
  91ed626c2d580641, tests/replay a5f123b8a2ddde1d, as the s6 suite runs read them (the plants restored to them).
- Step 7, the edit (docs and comments only; no code line changed):
  - scene.rs `ES_AIM_ONE_IN`'s doc: the 1-in-11 figures replaced by the window's (9.878 / 9.743 / 10.098 for seeds
    1-3, the halves 9.580-10.220, the split about 2.7 / 6.8 / 0.35 from step 5's probe at 40, the shares tried and why
    40: the rule step 5 stated).
  - scene.rs `grenade_fuse_ms`'s doc: B1.1's words, verbatim ("A thrown grenade's fuse, ms: 2 s, and ES's 1.515 s (ADR
    0001:1478-1479's 32 live projectiles at 21.12 spawns a second give each a life of 32 / 21.12 = 1.515 s, so every ES
    projectile fuses after it).").
  - scene.rs `aim_one_in`'s doc (the ruling's :114) and `Scene::estimate`'s doc (its :177): "10 gameplay events a
    second" now names the judged window (the last 10 minutes of a 12-minute recording) and each 300 s half;
    `Scene::estimate` says one command in 40 and the 60 s re-home, and its short line is rewrapped; `aim_one_in`'s doc
    adds that above 1 it also turns on 3b's re-home (3b reuses it as ES's switch: the step 4-5 builder's note).
  - replay.rs, the sky-aim comment (the ruling's :458-461): "so their recordings are as before" dropped (every scene's
    recording changed at step 3), the aim named as where this command's move puts the shooter (`after`), and the
    events named over the window and its halves. One clause added to `Recording::synthetic`'s doc (B1.2's symbol): in
    ES each soldier is also teleported home once every 60 s (3b), so the public doc lists every input it emits.
  - Cites checked against docs/adr/0001-multiplayer-netcode.md in the copy: :1478 "10 reliable gameplay events a
    second", :1478-1479 "32 live projectiles at 0.44 spawns per player per second", :1600 "ES at P2 for 10 minutes".
  - Lint 22:32:37-22:32:43 UTC (s7-lint.log): fmt --check; clippy -D warnings npt-sim host --all-targets and wasm32,
    measured and --all-features; rustdoc -Dwarnings --no-deps both builds: all exit 0, "Checking npt-sim (this copy)"
    x4, "Documenting" x2.
  - Suites, -x, each build in its own runner call (s7-suites-measured.log 22:32:48-22:33:46, s7-suites-bites.log
    22:33:47-22:34:49 UTC; "Compiling npt-sim (this copy's path)" once each; exit 0 both): measured lib 5, replay 6,
    seam 8, world 30 + 1 ignored; bites lib 5, replay 7, seam 10, world 35 + 1 ignored; 0 failed. No pin moved.
- **Step 7 DONE.** Files: scene b99b5595b46f3386, replay 30a17b58d365baea, world 6e01101323ce57a1, tests/world
  b2c3a8c8a072b490, tests/replay a5f123b8a2ddde1d. Next: the End checks.
- End, preliminary check.mjs run (counted, -x; end-check1.log 22:35:31-22:40:50 UTC, exit 0): "check: all 10 steps
  pass"; test: measured 367 passed, 0 failed, 0 reported a skip; bites 414, 0, 0; self 52 of 52. Its log has no
  "Compiling npt-sim" line (check.mjs captures cargo's output and prints each step's last line), so it does not meet
  the provenance rule alone; its test binaries were rebuilt inside the run (world-cd6b6cd529823938 22:36:31,
  world-66d53ef2cee32866 22:38:36, the names of the s7 runs' binaries). The two counted runs below each start with
  `cargo test --locked --workspace --no-run` in both builds in the same runner call (u4j-merge's form), so their logs
  carry the provenance lines.
- End, first pair of counted runs (end-check2.log 22:41:15-22:46:13, end-check3.log 22:46:19-22:51:15 UTC; exit 0
  both; "Compiling npt-sim (this copy's path)" x2 each): every step PASS, measured 367, bites 414, self 52 both times.
  Then a width check found 6 new comment lines at 121-122 characters (tests/world.rs 1120, 1155, 1168-1170, 1189) and
  1 in scene.rs (79; scene.rs never went past 120 before; the other files' bases hold some 121-123 lines; no check
  enforces it). Rewrapped by Edit (comments and one string continuation, whose message text is unchanged), so those
  two runs read files that are no longer final: scene.rs 1d42cd960701f764, tests/world.rs 0a340fb5db383d05 now. Redone
  on the final files: lint (end-lint2.log 22:53:02-22:53:07: fmt --check, clippy -D warnings host --all-targets and
  wasm32 in both builds, rustdoc -Dwarnings both builds, all exit 0); suites, -x, each build (end-suites-measured.log
  -22:54:11, end-suites-bites.log -22:55:13; exit 0; measured 5 / 6 / 8 / 30 + 1 ignored, bites 5 / 7 / 10 / 35 + 1
  ignored, 0 failed); the three-seed test by name (end-three-seeds.log -22:55:43, "1 passed"; its 21 "ES seed" lines
  equal s6-three-seeds.log's, diff: none). The two final counted check.mjs runs follow.
- **End DONE, on the final files.**
  - check.mjs run 1, counted (-x; end-check4.log 22:56:02-23:00:51 UTC; runner exit 0; "Compiling npt-sim v0.0.0
    (/home/user/Shamble-Frontlines/.claude/worktrees/wf_9ce88fa3-452-1/spikes/netcode-proof/crates/sim)" x2 from the
    two `--no-run` builds): PASS fmt 0.5 s; PASS clippy 3.1 s; PASS clippy-wasm32 1.1 s; PASS doc 41.4 s; PASS test
    233.0 s "measured build: 367 tests passed, 0 failed, 0 reported a skip; bites build: 414 tests passed, 0 failed, 0
    reported a skip"; PASS deps 0.7 s; PASS refs 0.0 s (656 tracked files); PASS marks 0.7 s (81 marks, rows all
    unchanged); PASS self 1.2 s "52 of 52 tests passed"; PASS redact 0.8 s (10 of 10; 183 items, 0 findings);
    "check: all 10 steps pass".
  - check.mjs run 2, counted (-x; end-check5.log 23:00:56-23:05:45 UTC; runner exit 0; the same provenance x2): PASS
    fmt 0.5 s; PASS clippy 3.6 s; PASS clippy-wasm32 0.9 s; PASS doc 41.1 s; PASS test 234.3 s, measured 367 / 0 / 0,
    bites 414 / 0 / 0; PASS deps 0.7 s; PASS refs 0.0 s; PASS marks 0.5 s; PASS self 1.1 s, 52 of 52; PASS redact 0.7 s;
    "check: all 10 steps pass".
  - Counts as deltas on the base (39f4290 = dad9566's spikes/): measured 365 -> 367 (+2), bites 412 -> 414 (+2), self
    52 -> 52 (+0). The +2 are step 0's two unit tests in world.rs's module (they run in both builds); t_m1 stays one
    test; the three-seed test is `#[ignore]`, so cargoCounts (which counts "passed") does not count it.
  - fmt --check and clippy -D warnings, host and wasm32, both builds: end-lint2.log (npt-sim, with rustdoc) and the
    check.mjs runs' fmt, clippy and clippy-wasm32 steps (the workspace), all exit 0 / PASS.
  - Final sha256 (`sha256sum` form, paths from the copy's root; this notes file is new too, and is left out of the list,
    as it cannot hold its own hash):

```
30a17b58d365baea91310a16e1130b41e95429faedc7a8212c7cb001663ee12e  spikes/netcode-proof/crates/sim/src/replay.rs
1d42cd960701f764d41fc937c5d694db3fd6c6061f29cea27a0c9e0ea9fd4900  spikes/netcode-proof/crates/sim/src/scene.rs
6e01101323ce57a14f3978014d98a2d07d5e37fcaa1c8e503c6cf76ed0dfcb4e  spikes/netcode-proof/crates/sim/src/world.rs
a5f123b8a2ddde1deb26437cb981f714d891dbf35fca1fe5daaee536c3799bdd  spikes/netcode-proof/crates/sim/tests/replay.rs
0a340fb5db383d056e9768f7263f58fe6ff4e5009b03ba6b391f15e55bb28c87  spikes/netcode-proof/crates/sim/tests/world.rs
```
  - Every other file in the base list above unchanged (sha256 first 8 equal: greybox 2d75cb0b, grid a0e24c08, hash
    4663b2a8, history f11e9b59, lib 396e663d, rng d6b61bbe, seam 0c41b406, ticks 5d852f1a, trajectory 2c0968b3,
    tests/seam 4900af4e, sim Cargo.toml fb98df48, check.mjs 7eddc285, workspace Cargo.toml 75b6c160, Cargo.lock
    3da1e15f). `git status`: those five modified, this file untracked; nothing committed; diff stat 5 files, +503 / -146.
  - Public items: none added in steps 6-7 (`EsRun`, `ES_WINDOW`, `ES_HALVES` and `milli` are private to
    tests/world.rs); none in U4d-F as a whole (`move_velocity` and `fight_vehicles` are pub(crate); `GENERATOR`'s value
    changed, not its signature).
- For the main session and the reviewers (end of the step 6-7 builder):
  - P1 under 3b: the old reckoning fails t_m1's events assertion by a level (halves 9.240 / 9.356, both under 9.5), not
    by a decay, because 3b resets both reckonings every 60 s; the ruling wrote that plant before 3b was needed.
  - a_m1's non-vacuity (a death within SS seed 5's 1,200 ticks) depends on SS's recording draws: P1 broke it. It holds
    on the real code in every run; a later generator change could break it with no defect (a test-engineer point).
  - The hull bound reads D-6's form (rounded down) against B1.3's 1,400; the exact maximum is 1,400.6 cm (the 1 cm grid).
  - The margins at 40 stay thin (seed 2's window test: 2nd half 9.606, 0.106 over the floor), as the step 4-5 builder
    noted; any later change to ES's draw stream re-runs this measurement.
