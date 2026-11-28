# U4d-F round 1: test-engineer

Copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_9ce88fa3-452-1 (branch u4df-build, HEAD dad9566, change
uncommitted: 5 M + the builder's record untracked). Base: .gate/u4df-prefix (worktree at dad9566). Packet:
u4df-packet.md (read whole). Plan: advisor-u4d-close.md B1.1-B1.3. Role: main's .claude/agents/test-engineer.md (read).
Runner: u4df-run.sh, log prefix te- under .gate/u4df/. Rules: plants by Edit in place, restored by Edit, sha256 checked;
TE2-5 for any failure I did not plant (re-run that crate with --no-fail-fast at once, name and panic here).
Notes are rewritten whole each time (Write).
MY SLIP: blocks [11] and [12] were first inserted into this file by a python one-off (a shell write), not by Write or
Edit; notes only, no other file; this whole file was then rewritten with Write.

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
  reckoning through `one_tick(scene.move_speed, 0, mv)`: same rule as the base's lines up to float rounding (x (1/30)
  against / 30); I plant the base's own two lines instead (below) to remove that doubt.
- [3] te-base-suite.log (23:34:13-23:35:38 UTC, -x, runner exit 0; "Compiling npt-sim (this copy's path)" x2):
  measured suite --no-fail-fast: lib 5, replay 6, seam 8, world 30 passed + 1 ignored, 0 failed (49 + 1 ignored). The
  ignored three-seed test by name (--ignored --exact --nocapture): "test result: ok. 1 passed" (30.75 s). Its 21 lines
  equal the builder's table (u4df-build.md:322-334) and the netcode's [8] on every figure (acceptance table in [8]).
- [4] Step 0's plants, each by Edit in world.rs, the whole measured suite --no-fail-fast, -x, restored by Edit,
  `sha256sum -c te-ref.sha256` all OK after each (the builder's forms, = round 2's O1-O3):
  - O1, part_center :452-453, old `cm(hull[0]) + ox * c - oy * s` / `cm(hull[1]) + ox * s + oy * c`, new
    `cm(hull[0]) + ox * c + oy * s` / `cm(hull[1]) - ox * s + oy * c` (te-plant-o1.log 23:36:05-23:37:03, exit 101):
    FAILED `world::tests::part_center_turns_a_parts_offset_by_its_hulls_yaw` (world.rs:2188, "offset [120, 0, 100] at
    yaw 90 degrees: [1000, -620, 100], not [1000, -380, 100]") and d7 (tests/replay.rs:351, SS's after-tick pins);
    lib 4 + 1 failed, replay 5 + 1 failed, seam 8, world 30 + 1 ignored.
  - O2, part_center :449, old `let (c, s) = (libm::cosf(yaw), libm::sinf(yaw));`, new
    `let (c, s) = (1.0 + 0.0 * yaw, 0.0 * yaw);` (te-plant-o2.log 23:37:16-23:38:12, exit 101): FAILED the same test
    ("[1120, -500, 100], not [1000, -380, 100]") and d7 (:351); the rest pass.
  - O3, present_volumes :1498, old `for slot in 0..self.soldiers.len() {`, new
    `for slot in 0..self.soldiers.len() * 0 {` (te-plant-o3.log 23:38:25-23:39:21, exit 101): FAILED
    `world::tests::a_shot_with_no_rewind_meets_a_soldier_where_this_ticks_move_put_it` (world.rs:2267, "a shot at the
    target's new chest [-5667, -500, 140] hits it") and d7 (:351); the rest pass.
  Each of O1, O2, O3 fails its T2-L1 test by name. MET. (By reading, test (b) also catches the subtler stale form,
  records refreshed after phase 0 but before the moves: the box would sit at the teleported point, so hit_new fails.)
- [5] Step 6's P1 in my form (the base's own reckoning lines restored; 3b and 40 kept), two Edits in replay.rs:
  (i) held-back block :473-476, old `let d = one_tick(c.speed_cm, back.buttons, back.move_xy);` +
  `c.est = [c.est[0] + d[0], c.est[1] + d[1]];`, new `let _ = back;`; (ii) :506-507, old
  `let d = one_tick(c.speed_cm, c.buttons, c.mv);` + `let after = [c.est[0] + d[0], c.est[1] + d[1]];`, new
  `let speed = scene.move_speed as f32 / 100.0;` + `c.est[0] += f32::from(c.mv[0]) / 127.0 * speed / TICK_HZ as f32;`
  + the same for [1] + `let after = c.est;` (every command moves the estimate at the scene's speed, sent, held or
  lost, as GENERATOR 1 did). te-plant-p1.log 23:39:51-23:40:46, exit 101 (one warning: unused `one_tick`): FAILED
  `t_m1_the_estimate_scene_meets_pm_0001_2s_es_bands` (tests/world.rs:1151, "seed 2, gameplay events and the drift
  check: 9846 thousandths a second over the window, 9240 and 10453 over its 300 s halves (each in 9,500-10,500)");
  d7 (:351, SS (300, 6624, 8955962681012353194)); `a_m1_the_authoritys_volumes_are_the_function_of_its_records`
  (tests/world.rs:940, "not vacuous: dead soldiers' boxes are not live"). Restored by two Edits; sha256 all OK.
  Reading: in this form the WINDOW is in band (9.846) and only the first half fails (9.240, 0.260 under the floor):
  the half check bites where the window check does not. The builder's form (x 1/30 in place of / 30, otherwise the
  same rule) gave window 9.298, halves 9.240 / 9.356: the first half is identical, the second differs by 1.097 a
  second from last-ulp float differences in the estimate (a 1 cm aim truncation flips a hit, then the run diverges).
- [6] P1 (my form, re-planted by the same two Edits, restored, sha256 OK) on the three seeds, the ignored test by name
  (te-plant-p1-seeds.log 23:42:19-23:42:51, exit 101): seed 1 window 9.358 (halves 9.253 / 9.463), seed 2 9.846
  (9.240 / 10.453), seed 3 9.235 (9.553 / 8.916); shots, live, spawns in band; 48/15; hulls 1,400. Mean of the three
  windows 9.480 under the old reckoning against 9.906 as built: the reckoning fix is worth about 0.43 events a second
  under 3b (measurement A, no 3b: 4.8-6.1). Every seed fails under P1, but t_m1 (seed 2 alone, the counted suite's
  only ES check) fails it only by one half, 0.26 under the floor, with the window in band.
- [7] P2 in my form (ES's FIRST hull, v = 0, at 40 m; the builder's was v = 14), world.rs :801, old
  `let radius = if v < fight {`, new `let radius = if fight == 15 && v == 0 {` / `40.0` / `} else if v < fight {`
  (te-plant-p2.log 23:43:30-23:44:30, exit 101): FAILED t_m1 at the hull bound (tests/world.rs:1131, "seed 2, the hull
  bound: a hull 4000 cm from the cluster's centre (4000.6 cm exactly)") and d7 (:331, "ES" (1944,
  18389669540720445003)); D-6 and the rest pass (SS's radii untouched). Restored by Edit; sha256 all OK. MET. By
  reading the bound is tight: floor(sqrt(max x^2 + y^2)) <= 1,400 passes the 14.0 m loop's 1,400.6 cm and fails any
  hull at 14.01 m or more.
- [8] The acceptance (B1.2 step 4) against te-base-suite.log's lines, every figure = the builder's table:
  | seed | window shots / events / live / spawns | halves | T-M1 300 s | X2 form | 48/15, hulls |
  |---|---|---|---|---|---|
  | 1 | 70.578 / 9.878 / 32.383 / 21.120 | 10.176 / 9.580 | 9.950 | 9.963 | yes, 1,400 (1,400.6) |
  | 2 | 70.586 / 9.743 / 32.384 / 21.120 | 9.880 / 9.606 | 10.440 | 10.180 (X2 7.578) | yes, 1,400 (1,400.6) |
  | 3 | 70.573 / 10.098 / 32.383 / 21.120 | 10.220 / 9.976 | 10.686 | 10.175 | yes, 1,400 (1,400.6) |
  Bands 68.4-75.6 / 9.5-10.5 / 30.4-33.6 / 20.064-22.176: every window and half in band; twelve 60 s rates printed per
  seed (8.583-11.783). MET as ruled. Smallest margin 0.080 (seed 1, half 2).
- [9] NF-M1 settled by run, through the test's own EsRun (probe: the ignored test's `[1, 2, 3]` -> `[4, 5, 6, 7, 8,
  9]`, tests/world.rs:1194, restored by Edit, sha256 OK; te-probe-seeds4-9.log 23:45:14-23:46:18, exit 101): windows
  10.025, 9.808, 9.788, 9.841, 9.693, 10.171; halves 10.450/9.600, 9.780/9.836, 10.130/**9.446**, 10.113/9.570,
  9.726/9.660, 10.390/9.953; panic "seed 6, gameplay events and the drift check: 9788 ... 10130 and 9446": = the
  netcode's [9] on every figure. CONFIRMED. Over seeds 1-9: windows mean 9.894 (sd 0.166, all in band); first halves
  mean 10.096 (sd 0.255), second halves 9.692 (sd 0.185); h2 - h1 negative in 8 of 9 seeds (mean -0.404): inside the
  judged window the rate falls by 0.4 a second, the same in every seed (netcode [10]: not after minute 12). From
  these: a fresh seed's second half is under 9.5 about 15 % of the time; three seeds pass every check about half the
  time.
- [10] Re-pins (item 6), from the final diff (tests/replay.rs: 5 one-line hunks in d7 only; tests/world.rs: t_m1's
  hunks only; every band literal unchanged: 68_400..=75_600, 9_500..=10_500, 30_400..=33_600, 20_064..=22_176, 48 / 15)
  and the builder's step logs' panic lines (read, not re-run):
  - ES before-first-tick (1_944, 8_780_045_697_152_956_090 -> 8_237_017_275_248_454_161): step 2 (s2-suites.log,
    :331 "ES" left (1944, 8237017275248454161)); after it, s2-repin-measured: d7 passes, so SS's, CO's, TC's pins, SS's
    after-tick pins and all three digests were unchanged by step 2.
  - SS after 300 ticks (6_612, 923_332_457_071_349_262 -> 3_337_416_615_378_533_976), GENERATOR 1 -> 2 (and its
    comment), digests SS (16_697_066, ..191_497) -> (16_694_921, 35_974_458_070_191_737) and TC (798_759, ..409_324)
    -> (799_693, 9_126_256_229_328_065_459): step 3 (s3-first-measured :351; s3-d7-digests :382); SS at 1 and 30
    unchanged.
  - ES's digest, three moves, one cause each: step 3 -> (16_695_272, 2_102_134_247_371_836_914); 3b ->
    (16_700_849, 84_585_687_253_498_977) (s4-3b-measured :382: SS's and TC's digests equal, so 3b drew nothing in
    them); step 5 -> (16_700_849, 15_077_317_756_489_230_056) (s5-first-measured :382; length unchanged).
  - Nothing else in tests/ moved; s1-suites (step 1): every test passes with no test file touched. MET.
  (Process note, no finding: t_m1 stayed failing, named, from step 2 to step 5 by design; each re-run once, TE2-5.)
- [11] P3, a partial regression (the reckoning ignores crouch only; replay.rs:408, old
  `let v = move_velocity(speed_cm, buttons, move_xy, 1.0).0;`, new
  `let v = move_velocity(speed_cm, buttons & !Command::CROUCH, move_xy, 1.0).0;`), the whole measured suite with
  --include-ignored --nocapture, -x (te-plant-p3.log 23:48:01-23:49:19, exit 101; restored by Edit, sha256 OK): FAILED
  t_m1 (:1151; "T-M1: ES seed 2 ... gameplay events 9.866 a second ...; 10.363 and 9.370 ... over its halves"), the
  three-seed test (seed 1 window 9.526, halves 9.723 / 9.330; seed 2 as t_m1; seed 3 9.636, 9.990 / 9.283), d7
  (:351, SS at 300), a_m1 (:940, "not vacuous"); 28 others pass (carry8's panics are its should_panic's). Every
  window in band; the second half (which sits ~0.4 under the first in every seed, [9]) fails, by 0.13 in t_m1. So the
  half check does catch a partial reckoning regression, by level, at margins inside one half's noise (sd about 0.19).
- [12] a_m1 probe (tests/world.rs:940, old `assert!(dead > 0, "not vacuous: dead soldiers' boxes are not live");`, new
  `assert!(dead > 1_000_000_000, "PROBE dead soldier-ticks {dead}");`, a_m1 by name; te-probe-a-m1.log
  23:49:45-23:49:49, exit 101 as planted; restored by Edit, sha256 OK): "PROBE dead soldier-ticks 58" on the real
  code. SS respawns after 1,000 ms (30 ticks, scene.rs:181), so a_m1's non-vacuity rests on about two deaths in its
  1,200 ticks; P1 and P3 (generator-only plants) both took them away.
- [13] check.mjs, the one counted run (te-check.log 23:50:19-23:55:12 UTC, -x, runner exit 0; first `cargo test --locked
  --workspace --no-run` and `--all-features --no-run` in the same runner call for provenance: "Compiling npt-sim
  (this copy's path)" x2; idle 0 s, 10 GB, HEAD dad9566; no other cargo at the end): PASS fmt 0.6 s; PASS clippy 3.5 s
  (both builds); PASS clippy-wasm32 1.0 s (both); PASS doc 43.0 s (both builds, host and wasm32, warnings as errors);
  PASS test 233.4 s "measured build: 367 tests passed, 0 failed, 0 reported a skip; bites build: 414 tests passed, 0
  failed, 0 reported a skip"; PASS deps 0.7 s (109 crate files, 1 cargo config); PASS refs 0.0 s (656 tracked files);
  PASS marks 0.6 s (81 marks, drafts 626df678, rows unchanged); PASS self 1.3 s "52 of 52"; PASS redact 0.8 s (10 of
  10; 183 items, 0 findings); "check: all 10 steps pass". Deltas on the base: measured 365 -> 367 (+2), bites 412 ->
  414 (+2), self 52 -> 52 (+0). The +2: the diff adds 4 #[test] and removes 1: step 0's two module tests (both builds,
  lib 3 -> 5), t_m1 replaced under its name (net 0), the three-seed test #[ignore] (not counted as passed). = the
  builder's end-check4/5.
- [14] Final: `sha256sum -c te-ref.sha256` all 6 OK and `te-ref-all.sha256` all 183 OK (23:55:23 UTC); no PROBE or
  PLANT text in crates/sim; git status the same 5 M + 1 ??; HEAD dad9566; diff stat 5 files, +503 / -146. The copy is
  byte-identical to [1]. Nothing committed; nothing edited beyond restored plants and probes.
- Other netcode findings by log or arithmetic: NF-L4's 7.250 is s2-suites.log's t_m1 line ("gameplay events: 7250"):
  CONFIRMED. NF-L2's 51.9 teleports a minute = 3b's 48 + about 3.9 random: CONFIRMED by arithmetic. NF-L1's churn
  shares and NF-L3: not run (reading only; no run of mine settles them).

## Findings
- TF-M1 MEDIUM, not before merge (for the advisor; a new test is outside B1.2's symbols): the counted suite guards
  B1.2 step 3's reckoning, N2-M1's fix, only by t_m1 on one seed, which tells a broken reckoning from the right one by
  less than one 300 s half's noise. Where: tests/world.rs:1150-1160 (the events and drift assertion, t_m1 = seed 2
  alone at :1174); replay.rs:405-410, :470-476, :506-507 (the reckoning). Evidence: P1 (the old reckoning) fails t_m1
  by 0.260 on its first half, window in band (9.846); P3 (crouch ignored only) fails it by 0.130 on its second half,
  window 9.866; a half's sd across seeds is about 0.19-0.26 and a last-ulp float change swings it by 1.1 ([5]). The
  three-seed test fails P1 and P3 on every seed, but it is #[ignore] and check.mjs counts `passed` only. Under 3b the
  reckoning is worth about 0.43 events a second (P1 mean 9.480 against 9.906, [6]). What holds the reckoning for
  certain is D-7's recording digest: T2-L1's lesson again (an invariant held only by a pin that a later legitimate
  change re-pins). Scenario: a later edit to `synthetic` (a GENERATOR 3, a new input kind) half-breaks the reckoning
  and re-pins D-7 for its own named cause; t_m1 passes with a probability of roughly a quarter to a half and the
  regression lands. Cheapest proof: te-plant-p1.log, te-plant-p3.log. Fix: one deterministic test of the reckoning,
  independent of the World's chaos: over ES's synthetic recording, each sky aim's x, y equals the integration of the
  commands the recording itself sends for that slot (move_velocity at the MoveSpeed it emitted, clamped; a held-back
  command moved when sent; a lost one never; teleports and spawns resetting), truncated to cm, exactly; P1 and P3
  each fail it on the first crouched or lost command.
- TF-L1 LOW, not before merge: a_m1's non-vacuity (tests/world.rs:940) rests on about two deaths in SS seed 5's 1,200
  ticks (58 dead soldier-ticks, [12]); both generator-only plants (P1, P3) remove them, so any later generator change
  can fail a_m1 with no defect. Pre-existing test; U4d-F moved SS's recording (GENERATOR 2). Fix: a follow-up: a_m1
  scripts its death (a kill by input) or runs long enough for several.
- TF-L2 LOW, not before merge (report wording): B1.2 step 4 says "A stationary generator makes the warm-up's length
  immaterial; that is what this step shows". Inside the window it is not shown: the second half is lower than the
  first in 8 of 9 seeds, mean -0.404 a second ([9]), so the window's thin margin sits in its second half (mean 9.692,
  0.19 over the floor). The netcode's 36-minute probe (its [10], not re-run by me) finds the level flat from about
  minute 7 at about 9.85-9.9. The report should say so, beside NF-M1.
- TF-L3 LOW, not before merge (the builder's record's reading): "P1 shows as a level (halves 0.12 apart), not a decay"
  (u4df-build.md:403-406, :492-493) holds for its form only: the base's own lines give halves 9.240 / 10.453 (the
  window in band), and the two forms differ only by x (1/30) against / 30. A plant's 300 s half is not a stable
  property; the report should give P1's three-seed figures ([6]) rather than one form's one seed.

## Netcode findings settled
- NF-M1: CONFIRMED by run through the test's own EsRun (seed 6's second half 9.446; [9]); I agree Medium.
- NF-L2 (teleports), NF-L4 (7.250): CONFIRMED (arithmetic, log). NF-L1, NF-L3: not settled by a run (reading).

## Verdict
APPROVE WITH FINDINGS. No Critical or High. Every claim proven by its own plant or run here: step 0's two tests catch
O1, O2, O3 by name; step 6's P1 (in the base's own lines) fails t_m1's drift check and P2 (a different hull) the hull
bound, by name; the three-seed test reproduces the acceptance table exactly; every re-pin has one named cause in its
own step and every kept band and pin is unchanged; check.mjs 367 / 414 / 52 (+2 / +2 / +0); the copy byte-identical
at the end. One Medium (TF-M1: the reckoning's counted guard is one noisy seed) and three Lows, all follow-ups.
DONE (2026-11-11 23:56 UTC)
