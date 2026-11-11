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
