# U4d-F round 1 (the estimate scene fix): netcode-architect (read only, never edits a copy)

Copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_9ce88fa3-452-1 (branch u4df-build, uncommitted).
Base: /home/user/Shamble-Frontlines/.gate/u4df-prefix (dad9566). Packet: u4df-packet.md (read whole, 1,548 lines).
Plan: advisor-u4d-close.md B1.1-B1.3 (steps 0-7; files and symbols; step 4 = the acceptance).
Runner: u4df-run.sh, log prefix nc, 600000 ms timeout; never retry an unplanted failure (TE2-5).

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
9. Runs (through the runner): npt-sim measured suite; the ignored three-seed test by name; maybe a probe by reading.
10. Findings with file:line, verdict.

## Progress
