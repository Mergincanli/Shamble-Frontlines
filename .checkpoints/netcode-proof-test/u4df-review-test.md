# U4d-F round 1: test-engineer

Copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_9ce88fa3-452-1 (branch u4df-build, HEAD dad9566, change
uncommitted: 5 M + the builder's record untracked). Base: .gate/u4df-prefix (worktree at dad9566). Packet:
u4df-packet.md (read whole). Plan: advisor-u4d-close.md B1.1-B1.3. Role: main's .claude/agents/test-engineer.md (read).
Runner: u4df-run.sh, log prefix te- under .gate/u4df/. Rules: plants by Edit in place, restored by Edit, sha256 checked;
TE2-5 for any failure I did not plant (re-run that crate with --no-fail-fast at once, name and panic here).
Notes are rewritten whole each time (Write).

## Plan
1. sha256 of every changed file (and every file under spikes/netcode-proof) first.
2. Read netcode-architect's notes (u4df-review-netcode.md) and the builder's record (claims only); read the base's
   run_command / apply_inputs / World::new loop placement against the diff.
3. Step 0: plants O1, O2, O3 (u4d-round2-test.md [5]) each fail T2-L1's two tests by name (whole npt-sim measured
   suite, --no-fail-fast; -x).
4. Step 6's plants: P1 the old reckoning restored (scene speed only) fails t_m1's drift check by name; P2 one ES hull at
   40 m fails the hull bound by name.
5. Run the ignored three-seed test by name; each seed's lines against B1.2 step 4's acceptance and the builder's table.
6. Every re-pinned value in tests/replay.rs and tests/world.rs: one named cause in its own step; every pin and
   assertion the ruling keeps unchanged did (from the diff, and from the builder's step logs).
7. Settle by run any netcode-architect finding a run can settle (NF-M1: seeds 4-9 through the official test code).
8. check.mjs once at the end (-x, counted): every step's line; counts as deltas on 365 / 412 / 52.
9. Final sha256 = step 1's.

## Progress
- [1] 23:31:43 UTC: sha256 of the 5 changed files and the builder's record (scratchpad te-ref.sha256), and of all 183
  files under spikes/netcode-proof (te-ref-all.sha256):
  replay.rs (src) 30a17b58d365, scene.rs 1d42cd960701, world.rs (src) 6e01101323ce, tests/replay.rs a5f123b8a2dd,
  tests/world.rs 0a340fb5db38, u4df-build.md 9fc9bdd71868. git status: 5 M + 1 ?? (the record). The diff base ->
  copy over spikes/netcode-proof is exactly these 5 files and equals the packet's diff. No cargo/rustc running; 10 GB.
- [2] Read: netcode notes whole (APPROVE WITH FINDINGS; NF-M1 Medium: the half check fires on noise at ES's
  stationary ~9.88, out-of-sample seed 6's 2nd half 9.446; NF-L1..L4); the builder's record whole (claims). Read in
  the copy: `synthetic` whole (replay.rs:367-605), apply_inputs (world.rs:1134-1164), revive (:1176-1188: does not
  reset move_speed, so the generator's held speed_cm survives a spawn or respawn, as the World's does), present_volumes
  (:1497-1502), part_center (:446-456). The generator's mv is scaled to length <= 127 (replay.rs:489-495), so
  move_velocity's scale branch never fires on generated commands. Builder's P1 form (s6-plant-p1.diff) routes the old
  reckoning through `one_tick(scene.move_speed, 0, mv)`: same rule as the base's lines up to float rounding; I plant
  the base's own two lines instead (below) to remove that doubt.
- [3] te-base-suite.log (23:34:13-23:35:38 UTC, -x, runner exit 0; "Compiling npt-sim (this copy's path)" x2):
  measured suite --no-fail-fast: lib 5, replay 6, seam 8, world 30 passed + 1 ignored, 0 failed (49 + 1 ignored). The
  ignored three-seed test by name (--ignored --exact --nocapture): "test result: ok. 1 passed" (30.75 s). Its 21 lines
  equal the builder's table (u4df-build.md:322-334) and the netcode's [8] on every figure (see [4] below).
