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
3. Bites from §5 U4d: B1 hash-map via D-2 (first mismatch = T, digests differ); B2 std-sin site (state changes at
   arm tick; proof U4d2's); B3 -41 speed 1 %; B4 -41 quant; B5 D-4 planted 0.0/0.0 (independent site); B6 bites-only
   clippy allow (sim/clippy.toml denies HashMap / std sin outside the allow).
4. Extra bites (missed or not done by the builder): D-1, D-5 (other pool), D-6 thresholds (soldiers/teams/vehicles/
   parts/projectiles/effects), D-7 pins, D-8 reach, D-9 blend ref, D-10 ammo, reload, clamp, rewind-outside twins,
   carry 38 event list, D-3 untouched streams.
5. Settle reviewer findings by probe (temporary test, restored): clamp z idempotency (rust L4), FLAG_FIRED catch-up
   mispredict (netcode M3), load_movers extra records (rust M1), decode pre-alloc (rust L1), non-canonical recording
   panic (rust L2), magazine dry at 11.4 min (netcode M1) if affordable.
6. Read tests vs plan: can each fail, vacuous checks, numbers vs sources (PM-0001-2 bands, §4.13, §4.15 sizes).
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
- [2] check.mjs run A, 10:32:05-10:36:57 UTC (80 s idle wait first; 12 GB free): all 10 steps PASS, but test step
  measured 318 (expected 304), bites 327 (expected 327). NOT COUNTED: two other lanes ran cargo during it
  (agent-a929... `cargo test --workspace`; wf_...-e0d-2 `cargo test -p npt-l1`): +14 measured = another copy's test
  binary linked. Step lines otherwise: fmt 0.5 s; clippy 11.5 s; clippy-wasm32 6.3 s; doc 23.8 s; test 244.3 s; deps
  0.6 s (93 crate files); refs 0.3 s (562 tracked); marks 2.5 s (81 marks); self 1.1 s (48 of 48); redact 0.8 s (10 of
  10, 166 items, 0 findings). Re-run needed when the lanes are quiet.
- [3] Test read (tests/world.rs, tests/replay.rs) candidates:
  - std-sin unit test can never fail on an unwired site (only "no diff before arm"; prints first tick) -> plant B2b.
  - D-9 reference tolerance: target box half-width 3 cm -> plant E6.
  - D-6 asserts SS only; PM-0001-2 also has ES bands (72 shots/s 68.4-75.6, 32 projectiles mean 30.4-33.6, 21.12
    spawns/s 20.064-22.176, 10 reliable events/s 9.5-10.5, 15 big vehicles): no U4d test -> probe G.
  - d10 clamp twin never asserts the step-10 shot ran -> probe H.
  - D-2 child protocol: blocking pipes, no timeout (a hung child hangs the suite; no flake source seen).
  - carry14 pins MB = 1 << 20 (history.rs:5-6, tests/world.rs:1346): PM-0001-7's row is `<= 1000000` with "MB = 10^6
    B" (marks.json:2688-2694, :2739; plan.md E11 "at most 1 MB"). 231 x 4,536 = 1,047,816 B > 1,000,000: spans
    7,334-7,700 ms pass for_ms and fail the mark. MEDIUM (small reach). Fix: 1_000_000 -> 220 ticks, 7,333 ms.
- [4] Bite script run 1 (u4d-test-bites.py, all ids; each restored, sha256 checked):
  - B1 BITES: measured D-2, hash-map fold at tick 1234: "Outcome { compared: 1235, first: Some((0, 1234,
    \"spawner[0].acc\")) }" (replay.rs:274).
  - B1-clippy BITES: "use of a disallowed type std::collections::HashMap" + "iteration over unordered hash-based type"
    at world.rs:1818-1819 (measured clippy, this copy's npt-sim checked).
  - B2 BITES (weakly): measured std sin in World::sinf: D-7 fails on the 30-tick pin only (hash 3247652229372912894 vs
    8166482889541977892); the 1- and 300-tick pins and every World::new pin still equal. D-7's native catch of a std
    sin swap rests on 1 of 3 sampled ticks.
  - B2-clippy BITES: "use of a disallowed method f32::sin" world.rs:719.
  - B3 BITES: D-3 "CO step 0: record" (vel grid 600 vs 606).
  - B4 BITES: D-3 "CO step 3: record" (raw pitch differs; the aim origin comes from the floor-snapped position).
  - B5 BITES: D-4 "a non-finite value in the state after tick 1150: soldier[0].pitch" (world.rs:1852).
  - E1 BITES: D-1 "SS" before the first tick (a process-wide counter in World::new).
  - E2 BITES: D-3 "CO step 0: Some(\"rng[0].position.lo\")" (predict drew from the spread stream).
  - E3 BITES: D-5 "step 16: the state hashes" (a refused throw drew from the projectiles' stream).
  - E4a BITES "big vehicles: 14"; E4b BITES "units of 20 parts: 3"; E4c BITES "projectiles: 299"; E4d BITES "shots a
    second: 625650 thousandths" (portal 1.00).
  - E5 BITES: D-7 "SS" World::new pin (FNV prime + 2).
  - E6 NOT CAUGHT: D-9 passes with the blend by fraction / 255 (1 cm off at 255). FINDING (MEDIUM): D-9's reference
    holds to the 6 cm box only (target half 3 cm, spread about 1 cm), world.rs:1266 / tests/world.rs:811-815, :861-887.
  - E7 BITES "step 100: the state hashes"; E8 BITES "step 5: the state hashes"; E10 BITES "step 10: the state hashes";
    E11 BITES "step 15: the state hashes"; E12 BITES "case 5: ... at 60.019 m not clamped".
  - E9 (Shot event on a cooldown refusal): the test FAILS by name, at its own shots_of check ("the presses at 10 and 20
    fire": [10, 11, 20]), before the twins' event check -> rerun as E9b with a Kill event (script 2).
  - P (probe file, 9 tests ran and passed, file deleted): my filter kept only lines starting "PROBE-", so most lines
    were lost; kept: PROBE-A feet z 3900/3955/4000 cm: clamped point sent again not clamped; feet z 4095 cm: first
    [6000,-1500,4095], sent again clamped AGAIN to [5998,-1500,4095] (rust L4 confirmed: idempotence fails for feet z
    above about 40.57 m, origin z above 40.95 + 1.02). PROBE-B field 8 (flags) grid 4 (authority) vs 0 (predicted) on
    a catch-up tick, control (one command a tick) equal: netcode M3 confirmed (FLAG_FIRED = 4, world.rs:92, cleared
    per tick at :1822, per command at :756). PROBE-E decode Ok(()) of a non-canonical recording; the Replayer panics
    (debug assert world.rs:1801) instead of Step::Failed: rust L2 confirmed. -> rerun probes via u4d-test-probe.py.
  - B1b PASSES: bites D-2 hash-map: digests fe045de3aed9a645 vs 7636a7bbf8fae345; first Some((0, 1234,
    "bite[0].hash_map_order")) = T.
- Next: rest of run 1 (B1c, B2b, B3b, B4b, B6a, B6b), then u4d-test-bites2.py (E9b, E13, E13-c9, C1), then
  u4d-test-probe.py, then --list both builds and check.mjs run B when the lanes are quiet.
