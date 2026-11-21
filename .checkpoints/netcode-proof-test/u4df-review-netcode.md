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
  soldiers on the spawn grid (home + 150 cm x (i, j), world.rs:704-712); gone at each slot's first re-home (< 60 s,
  inside the warm-up). Not a finding. A re-home at e beside a held-back command: World teleports, then runs the held
  command from home, then this tick's; the generator resets est, adds the held move, then `after`: same order.
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
  10.440 events a second (in band). U4e/U4r sections (u4-build-plan.md:704-808) and advisor-u4e-brief.md: no ES byte
  figure from the synthetic recording.
- [7] World: blasts only from thrown grenades (world.rs:1653-1661, radius 4 m); stand-ins do nothing on fuse;
  `damage` (:1615) has no team check, shots skip only the shooter (:1576): sky shots and blasts score on teammates.
  team = slot / 16 in ES (scene.rs:297): the ruling's formula re-homes team 0 in s 0-18.75, team 1 in 20-38.75,
  team 2 in 40-58.75 of each minute (a team-phased 60 s cycle; whole-minute windows average it out exactly).
- [8] nc-suite.log (23:19:04-23:20:35 UTC, runner exit 0, "Compiling npt-sim (this copy)" x2): measured lib 5, replay
  6, seam 8, world 30 + 1 ignored, 0 failed; the three-seed test by name "1 passed" (31.9 s): every figure equals the
  builder's (windows 9.878 / 9.743 / 10.098; halves 10.176/9.580, 9.880/9.606, 10.220/9.976; T-M1 9.950 / 10.440 /
  10.686; X2 form 9.963 / 10.180 / 10.175; 48/15; hulls 1,400 (1,400.6)).
- [9] Probe (scratchpad nc-probe-head / nc-probe-base: separate cargo packages with a path dependency on each copy's
  npt-sim, public API only, release with npt-sim's overflow checks; nothing in any copy touched), nc-probe.log
  23:22:57-23:23:14 UTC, runner exit 0, "Compiling npt-sim" once from each path. Calibration: seeds 1-3 equal the
  official figures exactly. Head, seeds 4-9 (out of sample): windows 10.025, 9.808, 9.788, 9.841, 9.693, 10.171;
  halves 10.450/9.600, 9.780/9.836, 10.130/**9.446**, 10.113/9.570, 9.726/9.660, 10.390/9.953: seed 6 OUT (2nd half);
  8 of 9 seeds pass. Mix over the window, a second (head, seeds 1-9): sky-shot hits 1.98-2.11, aimed-shot hits
  0.66-0.72, blast hits 6.61-7.02, kills 0.34-0.36; same-team 42 % of hits (base seed 2: 42 %); teleports 51.9 a
  minute (base 3.9). Base seed 2 (1 in 11, old reckoning, hulls out): window 6.403 (sky 0.745, aimed 2.370, blast
  3.120). Hits by the minute's third x victim team (sum seeds 1-9): [[5875, 5264, 5707], [5739, 6186, 5407], [5105,
  5933, 6319]]: each team peaks in its own re-home third, dips ~13 % in the third before its next (diag / off 1.11);
  total by third 16,846 / 17,332 / 17,357 (3 %). Churn of alive soldier records (window): head yaw grid changed on
  48.0-49.0 % of soldier-ticks, pitch 17.8-18.5 %; base seed 2: yaw 69.1 %, pitch 30.0 %. R2's assumption
  (docs/research/r2-netcode.md:1751): moving delta "yaw 16, pitch half the time".
- [10] Stationarity probe (nc-probe-head --bin long; nc-long.log 23:26:56-23:27:26 UTC, exit 0): ES, 36 minutes,
  seeds 1-9 (minutes 0-11 equal the 12-minute run: the generator draws nothing from its length). Cross-seed mean per
  5-minute block from minute 2: 10.096, 9.692, 9.941, 9.901, 9.799, 9.857, 9.887 (minutes 32-35); slope over minutes
  2-35 -0.0032 a second per minute; mean 9.882. 10-minute windows (2-11, 12-21, 22-31): all 27 in band (lowest
  9.537); paired h2 - h1: -0.404 (t -4.09), -0.041 (t -0.21), +0.059 (t +0.33); halves out of band 1, 1, 4 of 18
  (6 of 54, 11 %). Reading: the drift is FIXED (stationary at ~9.88 over 36 min); the window 2-11's falling halves
  are a seed-independent modulation (the hulls' loops run on `elapsed`, world.rs:1222, 0 at World::new :841, +1 a
  step :2132, so the same in every seed and first tick) that does not recur later; at 9.88 with a 300 s half's sd
  ~0.36, the "each half in band" check fails by noise about 1 half in 9, so three seeds' six halves pass only about
  half the time.
- [11] Focus 2: step 1 by reading: `move_velocity` (world.rs:466-505) holds the old lines 1293-1320 as they were;
  run_command's only other changes are the bite's form (`let mut gain = 1.0`, `gain = 1.01`, :1339-1343): measured
  x 1.0 is exact for every finite f32; bitten `speed_cm as f32 / 100.0 * gain` = the old `(speed_cm / 100) *= 1.01`,
  same operations in the same order; s1-suites.log both builds (49 and 57 pass), the speed bite's unit proofs
  (tests/world.rs:2366, :2421) among them, no pin moved. Returning the stance flags gives the sprint rule one home.
  The rust-reviewer trigger (a World line beyond moving the rule) is NOT reached, in my reading. Step 2: SS radius
  5 + 9 v / 3 in f32 = 5, 8, 11, 14 exactly (9 v exact, / 3 exact for multiples of 3), as 5 + 3 v; s2 logs: D-6 and
  SS's D-7 pins unchanged. ES: 5 + 9 v / 14, v = 14 -> 14.0 exactly; hull bound 1,400 (1,400.6 on the 1 cm grid) on
  every tick (t_m1, three-seed test); the loops do not depend on the seed, so it holds for every seed. MET.
- [12] Files and symbols: every change lies in B1.2's list (world.rs move function, ES loop placement, T2-L1's two
  tests and their import; scene.rs `fight_vehicles`, `ES_AIM_ONE_IN`, docs; replay.rs `synthetic` and `GENERATOR`
  (imports for them); tests/world.rs t_m1, the ignored test and their private helpers EsRun, ES_WINDOW, ES_HALVES,
  milli; tests/replay.rs D-7's pins and its comment). No public item added. 3b uses `aim_one_in > 1` as ES's switch,
  not a new knob (a knob is not in the list). Step 4: acceptance met as stated (3 seeds, last 18,000 of 21,600, four
  bands, both halves, report items printed by the ignored test); no band moved (tests/world.rs:1123-1160 = the base's
  four bands); step 5: 40 by the rule the builder stated before choosing, from step 4's three seeds.

## Findings
- NF-M1 MEDIUM, not before merge (for the advisor): the acceptance's drift check fires on noise at ES's stationary
  level, and 40 sits below the band's centre. Where: scene.rs:78-84 (40 and its measured doc), tests/world.rs:
  1150-1160 (each 300 s half in 9.5-10.5). Evidence: [9]-[10]: the generator is stationary (36 min, 9 seeds, slope
  -0.003 a second per minute) at 9.88; a 300 s half's sd ~0.36 puts 6 of 54 halves out of band by noise; out of
  sample seed 6 fails (9.446); 34, 36, 38 failed one in-sample half each, so 40 is a realisation the sweep selected on
  the seeds that judge it. Scenario: a later legitimate change to ES's draw stream or the World's ES outcome (U4g/U4h
  editing the shot path, a generator v3) re-runs t_m1 (seed 2: about 0.8 to pass) and step 4's three seeds (about 0.5);
  it fails with no drift and no defect, and the session re-tunes `aim_one_in` on noise again, re-pinning ES's
  recording each time. Reach today: none judged (U4d2 pins seed 2's 300 s recording, 10.440). Cheapest proof: my
  probes (nc-probe.log, nc-long.log). Fix (the advisor's call): keep 40 (the acceptance is met); the report states the
  stationary level (9.88, 9 seeds x 36 min), seeds 4-9's numbers and the half check's ~11 % false-alarm rate per half;
  a later re-measure judges drift by a paired-halves or slope test over 9 or more seeds (or longer windows) and sets
  the share by the stationary level nearest 10.0, never by in-sample margins.
- NF-L1 LOW, not before merge: N2-L2 widened. Where: replay.rs:540-541 (the sky aim at `after`), scene.rs:84 (39 of
  40 commands). ES's alive soldier records change yaw on 48 % of ticks and pitch on 18 % (base: 69 % and 30 %), against
  R2's moving-delta assumption (yaw 16 bits every update, pitch half the time; docs/research/r2-netcode.md:1751): the
  reckoning now holds the sky aim's horizontal offset nearly still while the soldier moves. Reach: none (ES's synthetic
  recording feeds only D-2 and U4d2's hash runs; U4r's ES runs are bots). Scenario: a later report takes ES's bytes by
  replaying the synthetic recording through the codec and reads ES lighter than R2's estimate: a bandwidth number
  passes too easily. Fix: one carry line (U4e and U4r): no ES byte or bandwidth figure is taken from
  `Recording::synthetic`'s ES recording; any that is states these churn shares beside R2's.
- NF-L2 LOW, not before merge (what 3b changes in what the recording exercises; report wording): replay.rs:449-465.
  Teleports 51.9 a minute in the window (base 3.9); sky-shot hits on point-blank overlaps 2.0 a second (base 0.75), so
  ES's events are now ~69 % grenade blasts, ~21 % sky-shot overlaps, ~7 % aimed hits, ~3.5 % kills (42 % same-team, as
  before), and `aim_one_in` moves only the aimed ~0.7 a second of 9.9 (scene.rs:124's "the share sets its 10 gameplay
  events" says more); the ruling's period-tied source is real and team-phased (team = slot / 16 makes the formula
  re-home each team inside one 20 s third: its hits peak there and dip ~13 % before its next; diag / off 1.11), and
  every judged window is whole minutes, so it averages out exactly; the teleports force no correction in any consumer
  today (synthetic recordings are replayed sim-only), but a predicting-client test fed ES's synthetic recording would
  meet 52 forced corrections a minute. Fix: the report names these numbers; any later consumer that predicts on the
  synthetic ES recording counts teleport corrections apart.
- NF-L3 LOW, not before merge: 3b's switch is `scene.aim_one_in > 1` (replay.rs:453; scene.rs:126 documents it): the
  re-home is tied to the aim knob. Scenario: a tuning that sets ES's share to 1 turns the re-home off silently (t_m1
  would then fail by a high rate, so it is guarded in practice), and a future scene with a share above 1 gets 48
  teleports a minute unasked. The builder stayed inside B1.2's symbols. Fix: a follow-up: a scene knob for the
  re-home period, if the advisor approves one.
- NF-L4 LOW, not before merge (carry 59's text): B1.3's in-fight hulls cut ES's events at fixed share and old
  reckoning from 9.853 to 7.250 a second over 300 s (s2-suites.log, t_m1's line: -26 %), by blocked moves and cover;
  the same hulls run under U4r's bots. Scenario: U4r's brawl ES suite lands below 9.5 and is read as a bot defect.
  Fix: carry 59 names this number so U4r's contract test expects it.

## Verdict
APPROVE WITH FINDINGS. No Critical or High. N2-M1's cause is removed as B1.2 states (every emitted input reckoned,
one move rule shared with the World, open-loop); 3b was needed (measurement A) and is built as stated; N2-M2's hulls
are inside 14 m on every tick with SS bit for bit; step 4's acceptance is met and reproduced; pins moved one cause per
step; and my 36-minute probe shows the drift is fixed (stationary at ~9.88). One Medium (NF-M1, the half check's
noise at that level) and four Lows, all follow-ups.
DONE
