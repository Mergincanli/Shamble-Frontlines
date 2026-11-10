# U4d build proof: test-engineer

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-1 (uncommitted, base 097f89704d99).
Plan: u4-build-plan.md §5 U4d, §4.2, §4.10, §4.13-4.15, §7.1. Reviewer notes: u4d-review-netcode.md, -rust.md,
-arch.md. Builder notes: <worktree>/.checkpoints/netcode-proof-test/u4d-build.md. Memory: none (memory: local, no folder).
Rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target, CARGO_NET_OFFLINE=true; `cd <wt> || exit 1`
on its own line; touch crates before each build and every 2 s during counted runs; no other cargo/rustc before a
counted run; df > 2 GB; plants in place, one at a time, restore + sha256; commit nothing. No Edit tool in this run:
plants by scratch scripts (plan §7.2's form, as u3d-test-bites.py): u4d-test-bites.py, u4d-test-bites2.py,
u4d-test-probe.py (this folder); notes rewritten whole with Write.

## Plan
1. Reference sha256 of every crates/sim file (done below). Check no other cargo/rustc, df.
2. check.mjs run A (unchanged): ten step lines, counts (expect measured 304, bites 327, 0 failed).
3. Bites from §5 U4d: B1 hash-map via D-2; B2 std-sin site; B3 -41 speed; B4 -41 quant; B5 D-4 0.0/0.0; B6 clippy allows.
4. Extra bites: D-1, D-3 streams, D-5 trace, D-6 thresholds, D-7 pins, D-8 slack, D-9 reference, D-10 traces.
5. Probes for reviewer findings (temporary test file, deleted).
6. Read tests vs plan: can each fail, vacuous checks, numbers vs sources.
7. check.mjs run B after fresh touch; sha256 all equal reference; git status unchanged.

## Reference sha256 (full; taken 2026-11-10 at start, equal to the builder's report)
- src/greybox.rs 2d75cb0b6949680c98463817046b15610c48454968d845e1ce65c414504f35e5
- src/grid.rs a0e24c08224e5dd412ad78e8caca870d92c6f2e03e36765bea5261ffcc999484
- src/hash.rs 4663b2a8d7c771d05ac33d9af8a9c1bcb951f7d35ce34440717d8699cce45f06
- src/history.rs 0c9012ace74fa3bf449d7aab6de363ab76071e0e4457ac26d2d901f29fee9cc7
- src/lib.rs 70eb8262e18f6461c81db8a7a9eb17710fbe209ab8e800ea18ede9448f01ba6f
- src/replay.rs 8c566b8ae73dd9b946de0b3bad73ce12c7348f25d8fe5bba1655efbe25e2c3ff
- src/rng.rs d6b61bbe47569943af598ce34b549cfc7297cefc27652cdd91414557bfc2cb7d
- src/scene.rs 4d8a04b5769cbaff76dac41c60be0364a78d1a797167886ea78a73e8362b2d56
- src/seam.rs a176cd0d8868e070a2591483efe1d3de661adba173d1e4bd32bf6e2f731a410a
- src/ticks.rs 5d852f1a81f72e958a89f5489bce5d556da92d096e86aebc4bd130e447dece5b
- src/trajectory.rs 2c0968b3ce8fef64991c2d2962193fdb821b8d117b52c7761c31dc5cd2c602ef
- src/world.rs 57b0cf9c212c8033b810dd724d4b61c017fdc9d06fa9801171326a6b605b476d
- tests/replay.rs a547314c1ea9593ee62bb079fb8d4f351b06ceb347e715224a30df0322b6b636
- tests/seam.rs d91b8c0c9fe872a0322126ecc6d6a72ba218bb63e3c24995abae177955da163b
- tests/world.rs 41ffc20ff978ad9a1fbdfc2c6f9c427f3e45bd9a0e4629b227ae6158431409ca
- Cargo.toml fb98df480b8d4970..., clippy.toml 4fd9facc4c0dde20..., check.mjs 6d1ed1888882c84b...
- git status at start: 5 M (grid, history, lib, seam, tests/seam), 10 ?? (builder notes + 7 src + 2 tests).

## Progress
- [0] Notes started; no earlier notes of mine (fresh). sha256 all equal the builder's final list.
- [1] Runner: scratchpad/u4d-te-run.sh (waits 8 s idle, df, touch, 2 s toucher, output to stdout only).
- [2] check.mjs run A, 10:32:05-10:36:57 UTC: all 10 steps PASS, but measured 318 (expected 304), bites 327 (=).
  NOT COUNTED: two other lanes ran cargo during it (agent-a929... `cargo test --workspace`; wf_...-e0d-2 npt-l1).
- [3] Test read findings: std-sin unit test cannot fail (B2b); D-9 tolerance (E6); ES bands untested (probe G); d10
  clamp's shot not asserted (probe H: it does shoot at 10 and 30); D-2 no timeout (nit); carry14 MB = 2^20 vs
  PM-0001-7's 1,000,000 ("MB = 10^6 B", marks.json:2688-2694, :2739): 231 x 4,536 = 1,047,816 B; MEDIUM.
- [4] Run 1 (u4d-test-bites.py) summary, every file restored and sha256 = reference at the end:
  B1 BITES (D-2 measured: first Some((0, 1234, "spawner[0].acc"))); B1-clippy BITES (disallowed type HashMap,
  world.rs:1818); B2 BITES weakly (D-7: only the 30-tick pin differs); B2-clippy BITES (f32::sin, world.rs:719);
  B3 BITES (D-3 CO step 0 record); B4 BITES (D-3 CO step 3 record); B5 BITES (D-4 tick 1150 soldier[0].pitch);
  E1 BITES (D-1 SS); E2 BITES (D-3 rng[0].position.lo); E3 BITES (D-5 step 16); E4a-d BITES (14 vehicles; 3 large;
  299 projectiles; 625.65 shots/s); E5 BITES (D-7 SS); E6 NOT CAUGHT (D-9 with / 255); E7 BITES (step 100); E8 BITES
  (step 5); E9 caught by the test's own shots_of check ([10, 11, 20]); E10 BITES (step 10); E11 BITES (step 15); E12
  BITES (case 5 at 60.019 m not clamped); B1b PASSES (digests fe045de3aed9a645 vs 7636a7bbf8fae345, first (0, 1234,
  bite[0].hash_map_order)); B1c BITES (fold left out of the dump: first None, 27,000 compared); B2b NOT CAUGHT (std-sin
  unit test passes with its site disarmed: first differing tick None); B3b BITES (0 moving ticks differ); B4b BITES (0
  ticks off the record's grid); B6a BITES (disallowed type, world.rs:434); B6b BITES (f32::sin, world.rs:451).
- [5] Run 2 (u4d-test-bites2.py): E9b BITES (Kill event: "step 11: the event lists"); E13 NOT CAUGHT (NaN yaw
  before the own-copy snap at elapsed 150: D-4 passes; the snap turns NaN into 0, grid.rs:176-182, and non_finite reads
  write_state only, world.rs:1751-1756); E13-c9 NOT CAUGHT (carry 9 passes too); C1 PASSES (D-6: 68,362 shots in 100 s,
  683.620/s; 100 portal changes, 1.00/s: the builder's numbers). End: all files = reference.
- [6] Probes (u4d-test-probe.py; 9 passed; file deleted; all = reference):
  - A: clamp idempotent at feet z 0/3900/3955/4000 cm; at 4095 cm clamped again ([6000,-1500,4095] -> [5998,...]).
  - B: catch-up tick (2 commands, first fires): records differ in field 8 (flags) 4 vs 0; control equal. M3 confirmed.
  - C: CO cover 7 -> 408 after load_movers of one part record of index 400 + a step (MAX_COVER 160). rust M1 confirmed.
  - D: size_of::<TickInput>() 104 B; 60 B header claiming 2^20 ticks: Err("a recording cut inside a tick"); VmPeak
    137,480 kB -> 178,460 kB (peak, not the size). rust L1 confirmed (109 MB reserved by arithmetic).
  - E: decode Ok of a non-canonical recording; Replayer panics (world.rs:1801). rust L2 confirmed.
  - F: SS shots/s by minute over 13 min: 684.10 682.30 683.36 682.95 683.93 682.71 682.63 684.95 684.63 683.93 682.26
    685.21 676.31; kills 329; 1 soldier at ammo 0 at the end. netcode M1 real but small by 13 min.
  - G: ES 100 s: shots 70.62/s (in 68.4-75.6); live projectiles mean 39.91 (OUT of 30.4-33.6); spawns 25.06/s (OUT
    of 20.064-22.176); vehicles min 15; soldiers min 48; kills 50. Cause: the recording's throws (1 in 400 per command,
    replay.rs:439: 48 x 30 / 400 = 3.6/s, fuse 2 s, world.rs:79) on top of the 21.12/s stand-ins. NEW, MEDIUM.
  - H: d10 clamp stream shots at [10, 30]: the test's shot ran (not asserted: nit).
  - K: SS records changed per tick: Soldier 47.8/48, Vehicle 14.9/15, VehiclePart 123.5/124, Projectile 3.1/307.1,
    Door 0/6, Wall 0.5/1, Effect 384/384 (574 changed records a tick): supports netcode H1 and the effect-row finding.
- [7] check.mjs run B, 10:54:29-10:58:25 UTC (COUNTED: no other cargo seen during it by the 5 s watcher): all 10 PASS;
  fmt 0.4 s; clippy 7.5 s; clippy-wasm32 1.1 s; doc 18.2 s; test 204.0 s measured 304 passed 0 failed 0 skip, bites
  327 passed 0 failed 0 skip; deps 0.5 s (93 crate files, 1 config); refs 0.0 s (562 tracked); marks 2.2 s (81 marks,
  drafts 626df678); self 1.2 s (48 of 48); redact 0.8 s (10 of 10; 166 items, 0 findings).
- [8] --list (10:58:55): measured npt-sim 37 (replay 6, seam 8, world 23), bites 45 (7, 10, 28), each "Compiling
  npt-sim (this copy's crates/sim)"; clippy wasm32 -p npt-sim both builds: "Checking npt-sim (this copy)", 0 warnings.
- [9] check.mjs run C (final, fresh touch), 10:59:12-11:02:43 UTC (COUNTED: no other cargo seen): all 10 PASS; fmt
  0.4 s; clippy 3.1 s; clippy-wasm32 0.6 s; doc 17.8 s; test 185.1 s measured 304 / bites 327, 0 failed, 0 skip; deps
  0.5 s; refs 0.0 s (562); marks 0.7 s (81); self 1.2 s (48 of 48); redact 0.8 s (166 items, 0 findings).
  After: every crates/sim file, Cargo.toml, clippy.toml and check.mjs at the reference sha256; tests/ holds replay.rs,
  seam.rs, world.rs only; git status identical to the start; HEAD 097f8970. The worktree is unchanged by me.

## Findings (ranked)
- M1 ES misses two PM-0001-2 bands on its synthetic recording (probe G); no ES test (D-6 is SS only).
- M2 carry 14's 1 MB = 2^20 vs PM-0001-7's 1,000,000 B (tests/world.rs:1346, :1357-1361; history.rs:5-6).
- M3 D-9 reference holds to the 6 cm box only: / 255 passes (E6; world.rs:1266; tests/world.rs:811-815, :861-887).
- M4 catch-up tick: predict clears FLAG_FIRED per command, step per tick (probe B; world.rs:756, :1821-1824); D-3
  is one command a tick only (tests/world.rs:184-234). Confirms netcode M3.
- M5 load_movers grows movers to any record index (probe C: 408 cover > 160; world.rs:770-773 vs :1760-1797).
  Confirms rust M1.
- L1 D-4 misses a NaN laundered by the own-copy snap (E13, E13-c9; world.rs:1101-1111, :1751-1756; grid.rs:176-182).
- L2 std-sin unit test cannot fail (B2b; tests/world.rs:1661-1682); D-7 catches a native std-sin swap on 1 of 3 pins.
- L3 clamp idempotence breaks for feet above about 40.57 m (probe A; world.rs:1009-1015). Confirms rust L4.
- L4 Replayer panics (debug) on a non-canonical recording that decode accepts (probe E). Confirms rust L2.
- L5 decode reserves 2^20 x 104 B from a 60 B header (probe D; replay.rs:320). Confirms rust L1.
- L6 SS magazine: 676.31 shots/s in minute 13, 1 soldier dry (probe F; scene.rs:105-107, :127). Confirms netcode M1
  as small; D-6's 100 s cannot see it.
- N1 d10 clamp test does not assert its shots (probe H shows them). N2 D-2's pipes have no timeout. N3 carry 13
  storage identity in place of a counting allocator (agreed).
- Info: probe K's 574 changed records a tick (parts 123.5/124, effects 384/384) for U4e's budget (netcode H1).
Verdict: REVISE (M1-M3 are U4d test or data fixes for the fix pass netcode's H1 already needs; M4, M5 routed with
their reviewers' findings; Lows as follow-ups).
DONE (2026-11-10 11:04 UTC)
