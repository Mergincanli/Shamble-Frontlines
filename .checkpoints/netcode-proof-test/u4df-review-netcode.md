# U4d-F round 1 (the estimate scene fix): netcode-architect (read only, never edits a copy)

Copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_9ce88fa3-452-1 (branch u4df-build, uncommitted).
Base: /home/user/Shamble-Frontlines/.gate/u4df-prefix (dad9566). Packet: u4df-packet.md (read whole, 1,548 lines).
Plan: advisor-u4d-close.md B1.1-B1.3 (steps 0-7; files and symbols; step 4 = the acceptance).
Runner: u4df-run.sh, log prefix nc, 600000 ms timeout; never retry an unplanted failure (TE2-5).
MY SLIP: blocks [5]-[8] below were first appended to this file by a python one-off (a shell write), not by Write;
notes only, no other file; this whole file was then rewritten with Write, and every later block is written so.

## Plan
1. git status / git diff in the copy: confirm the file set is within the plan's list (world.rs, scene.rs, replay.rs,
   tests/world.rs, tests/replay.rs) and nothing else under crates or elsewhere.
2. Read the World's apply_inputs (attribute clamp, teleports/spawns order), run_command, step phase order, the
   respawn, and the generator whole (replay.rs synthetic) in the copy, against the base.
3. Focus 1: does the reckoning follow every emitted input (buttons, speed as clamped, dropped/held-back, teleports
   and spawns), via one shared move rule, open-loop? Check order of phase-0 inputs vs commands; held-back moves at
   the speed the World holds when it runs; MoveSpeed clamp (min and max); teleport of a dead soldier; spawn of an
   alive one; the aim-from-`after` vs the World's fire origin (pre or post move).
4. Focus 2: step 1 behaviour-neutral (World lines beyond the function move? gain vs speed*1.01 float order);
   step 2: SS radius formula bit-for-bit (5 + 9v/3 vs 5 + 3v in f32), ES hull bound on every tick.
5. Focus 3: 3b: was the drift check failing after step 3 (builder's logs under .gate/u4df/)? Built as B1.2 states?
   What changes in what the recording exercises (teleport count, corrections, home-point overlap tied to period).
6. Focus 4: step 4's measurement vs acceptance; aim_one_in chosen per step 5; bands unchanged.
7. Focus 5: pins moved only in their own step with one named cause (builder's step logs).
8. Focus 6: INVALID risks at R2 / marks passing too easily (N2-L2 yaw and pitch churn, home-point events).
9. Runs (through the runner): npt-sim measured suite; the ignored three-seed test by name; an out-of-sample probe
   (a scratch crate outside every copy, public API only, path dependency on the copy's npt-sim; not a plant).
10. Findings with file:line, verdict.

## Progress
- [1] git status in the copy: exactly the 5 plan files modified (src replay/scene/world, tests replay/world) + the
  builder's untracked u4df-build.md; nothing else. HEAD dad9566 = base worktree's HEAD. Packet diffs = git diff (hunks
  world.rs @466, @754, @1290, @2141 match).
- [2] Read in the copy: apply_inputs (world.rs:1134-1164: spawns (revive, any state), then teleports (alive only),
  then attributes (MoveSpeed .min(MOVE_SPEED_MAX_CM_S), plates included)); step (:2083-2139: vel cleared, phase 0,
  movers, then EVERY command of input.commands run in order, no repeat for a missing slot); run_command (:1317-1443:
  dead or plate -> no move; move_velocity then collide by v x DT; pos snapped; origin = snapped pos + CHEST);
  run_respawns (:1735-1752: team spawn point from the respawn stream); DT = 1/TICK_HZ f32 (:73), TICK_HZ 30.
  Generator whole (replay.rs:367-605).
- [3] Focus 1 by reading: MET. Phase-0 draws moved before the commands (replay.rs:421-465); speed_cm =
  value.min(MOVE_SPEED_MAX_CM_S) (:430) = apply_inputs :1157; held-back moves first at the speed held now (:473-476),
  before the heading turn, as the World runs it first at e+1 (canonical order: back's tick e-1 < e); `after` (:508-509)
  becomes est only when sent (:583, :590); lost (:587) never moves; teleports/spawns reset (:441, :462). One rule:
  `move_velocity` (world.rs:466-505) called by run_command (:1344) and one_tick (replay.rs:407-410), dt = DT. Open-loop:
  no World in the generator. Gaps it cannot see, named in its doc (:365-366): blocked moves, deaths (a teleport of a
  dead soldier resets est but the World ignores it, :1149), snap. Unnamed gap: est starts at home, World::new puts
  soldiers on the spawn grid (~3 m off, builder's probe); gone at each slot's first re-home (< 60 s, inside the
  warm-up). Not a finding.
- [4] Builder's record read (claims): measurement A (after step 3, 1 in 11, no 3b) decays 11.5 -> 4.25 a minute, halves
  6.1/4.9, 6.7/5.5, 5.1/4.5: drift check FAILS -> 3b's condition met (to verify in s4-probe-11.log). No-collision probe:
  reckoning error plateaus 5-7 m; with collision grows to ~20 m median: blocked moves dominate (ES hulls in the fight?).
  Sweep 11..48; 40 chosen by largest smallest margin; 34, 36, 38 each failed one half.
- [5] Logs checked (.gate/u4df/): s4-probe-11.log = measurement A (step 3, 1 in 11, no 3b): events decay in every
  seed, blast hits decay with them (seed 1 7.0 -> 2.3 a second, minute 0 -> 11): the cluster disperses; halves 6.113 /
  4.853, 6.676 / 5.476, 5.086 / 4.513: drift check FAILS -> 3b's condition (B1.2 step 4) MET. Pin moves, one per step:
  s2-suites (ES before-first-tick 8780.. -> 8237.., step 2), s3-first-measured (SS (300) 9233.. -> 3337.., step 3),
  s3-d7-digests (three digests, step 3), s4-3b-measured (ES digest -> 84585.., 3b), s5-first-measured (ES digest ->
  15077.., step 5). Each named and re-run once (TE2-5) by the builder. Focus 5 MET.
- [6] Consumers of ES's synthetic recording (u4-build-plan.md:494-495, :608-613, :694): D-2 and U4d2 only (per-tick
  hash, 3 x 9,000 ticks; ES = seed 2 from 1,000,000, tests/replay.rs:26). The judged ES runs are U4r's (bots; carry
  59). So the window's band is a stand-in property, not a judged mark; U4d2's ES recording is 300 s from tick 0 at
  10.440 events a second (in band).
- [7] World: blasts only from thrown grenades (world.rs:1653-1661, radius 4 m); stand-ins do nothing on fuse;
  `damage` (:1615) has no team check, shots skip only the shooter (:1576): sky shots and blasts score on teammates.
  team = slot / 16 in ES (scene.rs:297): the ruling's formula re-homes team 0 in s 0-18.75, team 1 in 20-38.75,
  team 2 in 40-58.75 of each minute (a team-phased 60 s cycle; whole-minute windows average it out exactly).
- [8] nc-suite.log (23:19:04-23:20:35 UTC, runner exit 0, "Compiling npt-sim (this copy)" x2): measured lib 5, replay
  6, seam 8, world 30 + 1 ignored, 0 failed; the three-seed test by name "1 passed" (31.9 s): every figure equals the
  builder's (windows 9.878 / 9.743 / 10.098; halves 10.176/9.580, 9.880/9.606, 10.220/9.976; T-M1 9.950 / 10.440 /
  10.686; X2 form 9.963 / 10.180 / 10.175; 48/15; hulls 1,400 (1,400.6)).
