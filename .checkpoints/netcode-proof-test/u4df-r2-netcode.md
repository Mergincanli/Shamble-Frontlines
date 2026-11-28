# U4d-F round 2, netcode-architect (read only; log prefix nc2)

Started 2026-11-12. Packet: u4df-r2-packet.md. Copy: .claude/worktrees/wf_9ce88fa3-452-1 (uncommitted). Round 1 files:
.gate/u4df-r1. No Edit tool in this seat: this file is rewritten whole with Write each time.

## Plan
1. Confirm the diff (copy vs round 1) is exactly FX-1 (replay.rs tests module) and FX-2 (scene.rs two docs); nothing
   else changed (git diff in the copy, diff --no-index against .gate/u4df-r1).
2. Read the generator (`Recording::synthetic`, replay.rs) and the World's input path (world.rs `apply_inputs`,
   `move_velocity`, phase order, held-back / late commands, MoveSpeed clamp, spawn / teleport) and check FX-1 integrates
   what the World does with the recording's inputs, from the recording alone, through the World's own move rule.
3. Check each of the six counts measures what its name says and that each plant Q1-Q5 would be caught (reason from
   the generator code; read the builder's fx-* logs as claims; run FX-1 once in host through the runner). If a count
   lumps cases, measure the split with a read-only probe (a scratch crate on npt-sim's public API, outside every copy,
   run through the runner), as round 1's probes were.
4. Check FX-2's words against T3 (exact), against the measured facts (round 1 notes [9]-[10], NF-L2 numbers), and the
   120-char wrap.
5. Verdict per finding: TF-M1, NF-M1 (wording point), NF-L2 (wording point). New defects with severity, file:line.

## Notes (one line per finding as it lands)
- [1] Scope: copy vs .gate/u4df-r1: only src/replay.rs (802842b0..) and src/scene.rs (a5ce8ac2..) differ; world.rs
  6e011013.., tests/replay.rs a5f123b8.., tests/world.rs 0a340fb5.. byte-identical (sha256sum both sides). Whole
  spikes/netcode-proof diff --no-index: 2 files, +164 -4 (= the packet). Nothing outside FX-1 and FX-2. Re-checked at
  the end: the same five hashes, git status unchanged.
- [2] World read (world.rs, unchanged): step :2083-2139 = phase 0 (apply_inputs :1134-1164: spawns revive, teleports
  alive only, attributes; MoveSpeed `.min(MOVE_SPEED_MAX_CM_S)` :1157), movers, then EVERY recorded command in order
  through run_command (:1317): dead/plate -> no move; move_velocity(s.move_speed at this tick, c.buttons, c.move_xy,
  gain 1) :1344; collide(v x DT) :1345; s.pos = 1 cm snap :1378. c.tick read only by the canonical assert (:2084):
  a held-back command runs on the carrying tick at that tick's speed, after that tick's phase 0. Generator
  (replay.rs:367-605): phase 0 drawn first (:421-465), held move at the carrying tick's speed first (:473-476), aim
  from `after` (:508-509, :540-541), est = after only when sent (:583, :590), lost never (:587).
- [3] FX-1 against them, by reading: reads only rec.ticks (attributes, spawns, teleports, commands with tick/seq/
  buttons/move_xy/aim), scene.slots, scene.move_speed (= World::new's start speed, world.rs:751), MOVE_SPEED_MAX_CM_S,
  TICK_HZ; moves through crate::world::move_velocity (no copy of the rule; dt = 1/TICK_HZ f32 = World's DT :73).
  Phase 0 (attributes, then spawns, then teleports) before the commands; position and speed are independent, so the
  order attributes-first vs the World's spawns-teleports-attributes is equivalent; spawns before teleports as the
  World. `before` (= `own`, the state after the previous tick's phase 0) is right for a held-back aim: a slot that
  held back at t-1 sent nothing else at t-1 (held_back.take() arm), so its state after t-1's phase 0 is its state at
  the aim. Moves at now[k].speed_cm on the carrying tick, recorded order. Float order equal to the generator's (est +
  d per axis; `(after * 100.0) as i32`). Lost: never in rec.ticks, so never moved; detected as a seq gap. Checks start
  at the slot's first placement (est None before), never from the generator's initial home. Recording = t_m1's (seed
  2, 12 x 1,800 = 21,600 ticks from 1,000,000, SPAN_MS 2,000: tests/world.rs:15, :1035, :1176). Encode/decode is
  lossless (tests/replay.rs:456, :461), so rec.ticks = what the Replayer feeds the World. MET.
- [4] Not modelled, by design (B1.2 step 3, the generator's doc :365-366): collision, a dead soldier's teleport (the
  World ignores it, :1149), deaths and respawns, and the World's 1 cm own-copy snap of s.pos on every move (:1357-1378).
  FX-1 is exact against the input-only reckoning the ruling defines, not against the World's position. Not a finding
  of this round (round-1 design, ruled). Note for later: the snap is deterministic from inputs absent collision.
- [5] MoveSpeed clamp: MOVE_SPEED_MAX_CM_S = 10,000 (seam.rs:171); the generator emits 400 + draw(400) = 400-799
  (replay.rs:424), so `.min()` never bites in any recording: FX-1 takes it as apply_inputs does (form right), but no
  plant of the clamp can change a recording or fail FX-1. Not a defect (an unreachable branch, nothing to guard).
- [6] Count 6 lumps two cases the ruling names separately in the rule ("moves the estimate on the tick that carries
  it, at that tick's speed"): re-homed and re-sped. Expected per seed-2 recording: re-homes ~645 (3b 48 x 12 + ~69
  random) x ~1/64 held = ~10; re-sped ~69 MoveSpeeds x ~1/64 = ~1.1 (P(none) ~ 1/3). The builder's line gives 11.
- [7] Runs (nc2-fx1-and-probe.log, 01:04:42-01:05:01 UTC, shared lock, runner exit 0; provenance "Compiling npt-sim
  (this copy)" x2): (a) FX-1 by name, measured, `--exact --nocapture`: "test result: ok. 1 passed; 0 failed; ... 5
  filtered out; finished in 1.72s"; its line = the builder's (954628; 97974; 14860; 465658; 14996; 11). (b) Probe
  (scratchpad nc2-probe: a separate package, path dependency on the copy's npt-sim, public API only, steps no World):
  seed 2: held-back carried 15,927; carried on a touched tick with an estimate 11 = re-homed 11, re-sped 0; MoveSpeeds
  69, on a tick carrying the slot's held-back command 0 (so FX-1's 11 = 11 re-homes: probe calibrated on FX-1's
  count). Seeds 1-9: re-sped carries 1, 0, 0, 0, 1, 1, 0, 2, 0 (seed 1: tick 1014152, slot 47, crouched, 438 -> 415
  cm/s, effective 219 -> 207, move [108, 66], next sky aim before any re-home: yes).
- [8] FINDING NC2-L1 (Low, not before merge): count 6 does not certify the held-back speed clause. Seed 2 holds no
  MoveSpeed on a tick that carries its slot's held-back command [7b], so a generator that moves a held-back command at
  its own tick's speed (say, the speed stored with `held_back` at hold time) writes a byte-identical seed-2 recording:
  FX-1, D-7 and t_m1 all pass it, by construction. Reach: nil on anything seed 2 makes (U4d2's ES recording is seed 2;
  the same draw stream), one command's move of ~0.3 cm on seeds 1, 5, 7, 8 (RM's runs). Cheapest proof: the
  test-engineer's plant Q6 (store c.speed_cm in `held_back` at replay.rs:586 and use it at :474): FX-1 passes on seed
  2. Fix (follow-up): split count 6 into re-homed and re-sped (each asserted above 0) and, by R2's own fall-back, add
  seed 1 (one case; run time ~3.5 s in debug, under R2's 5 s).
- [9] Count 5 nit (no finding): the lost flag is set by a seq gap seen after the slot's re-home, so a command lost
  just before a re-home is counted (seed 2: 4 gaps detected on a re-homing tick, of 14,996 counted). The comment at
  replay.rs:838-839 says "since ... its last re-home: a lost command". Non-vacuity unaffected.
- [10] Plants (builder's logs, claims; the test-engineer proves): fx-plant-q1..q5.log each show "test
  replay::tests::the_sky_aims_reckon_every_input_the_recording_emits ... FAILED" with lib "test result: FAILED. 5
  passed; 1 failed"; the panics at :885, :884, :882, :883, :886 = the assert_eq at :884 shifted by each plant's line
  delta (Q1 +1, Q2 0, Q3 -2, Q4 -1, Q5 +2): consistent. By reading, each plant changes a sky aim of seed 2 with a
  non-zero move: Q1 (no crouch, no held move, no MoveSpeed: all three), Q2 (97,974 crouched sky aims), Q3 (14,860
  held-back; the next aim carries the missing move), Q4 (the first MoveSpeed at e = 157), Q5 (14,996 aims after a gap).
  Other wrong reckonings, by reading: phase 0 after the commands (645 re-homes, 69 MoveSpeeds) caught; a held move
  after the current command's aim caught (every carried pair); a spawn not resetting caught (~22 spawns); sprint under
  held fire caught by volume (ES always fires). The one not caught: [8].
- [11] FX-2 words: a read-only script joined each doc's /// lines in both files: `ES_AIM_ONE_IN`'s new doc = old +
  " " + T3 (a) exactly; `aim_one_in`'s new doc = old with "sets its 10 gameplay events a second" -> "tunes its
  gameplay events toward 10 a second" and T3 (b) inserted after "U4d close ruling, B1.2)" exactly. Widest changed
  line 120 characters. Truth: (a) level 9.882, slope -0.0032 a second per minute, 9 seeds x 36 min (round 1 [10]);
  half noise: my sd ~0.36, the test-engineer's 0.19-0.26 -> "0.2-0.36" holds; "one passing set of halves, not a
  margin" holds; RM exists (ruling R1, carry 67). (b) aimed hits 0.66-0.72, total ~9.9 (round 1 [9]). "Only": the
  share's complement, the sky shots, moves the other way by about 1/14 as much (sky hits ~2.0 x (N-1)/N): 40 -> 16
  predicts aimed +1.05 and sky -0.08; seed 1's window shot hits 2.74 -> 3.46 a second, blasts 6.78 -> 6.63, kills
  0.35 -> 0.35 (s5-probe-40.log, s5-probe-16.log): "only" holds within the noise. My round-1 wording; no finding.
  The parenthesis leaves kills (0.35) unnamed; they follow the hits; no finding. Scene::estimate()'s doc
  (scene.rs:195-199) still says "10 gameplay events ... from its recording's aim ... and its re-home": true as
  written (every event comes from a recorded aim or throw); outside R2's FX-2; no finding.
- [12] Closures: TF-M1 CLOSED (FX-1 is the deterministic exact reckoning test TF-M1 asked for, counted in both builds;
  P1 = Q1 and P3 = Q2 fail it by name per the logs; residual gap = NC2-L1, Low). NF-M1's wording point CLOSED by
  FX-2 (a) (the doc no longer reads 40's 0.080 as a margin; the report half is T1's, not judged here). NF-L2's wording
  point CLOSED by FX-2 (b) (scene.rs:124's "sets its 10 gameplay events" replaced; the share's reach named).

## Verdict
APPROVE WITH FINDINGS: one Low (NC2-L1), a follow-up. No Critical, High or Medium. FX-1 integrates the recording's
inputs as the World takes them (phase 0 first, held-back aimed on its own tick and moved on the carrying tick at that
tick's speed, lost never, spawns and teleports resetting, MoveSpeed clamped as apply_inputs clamps it), from the
recording alone, through the World's own move_velocity; FX-2's words are T3's exactly and true.
DONE
