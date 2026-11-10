# U4d build proof: test-engineer

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-1 (uncommitted, base 097f89704d99).
Plan: u4-build-plan.md §5 U4d, §4.2, §4.10, §4.13-4.15, §7.1. Reviewer notes: u4d-review-netcode.md, -rust.md,
-arch.md. Builder notes: <worktree>/.checkpoints/netcode-proof-test/u4d-build.md. Memory: none (memory: local, no folder).
Rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target, CARGO_NET_OFFLINE=true; `cd <wt> || exit 1`
on its own line; touch crates before each build and every 2 s during counted runs; no other cargo/rustc before a
counted run; df > 2 GB; plants in place, one at a time, restore + sha256; commit nothing. No Edit tool in this run:
plants by a scratch script (plan §7.2's form, as u3d-test-bites.py), notes rewritten whole with Write.

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
  - D-9 reference tolerance: target box half-width 3 cm; a /255 blend denominator (1 cm off at f=255) likely passes
    -> plant E6.
  - D-6 asserts SS only; PM-0001-2 also has ES bands (72 shots/s 68.4-75.6, 32 projectiles mean 30.4-33.6, 21.12
    spawns/s 20.064-22.176, 10 reliable events/s 9.5-10.5, 15 big vehicles): no U4d test -> probe G.
  - d10 clamp twin never asserts the step-10 shot ran (events equal could both be shot-less) -> probe H.
  - D-2 child protocol: blocking pipes, no timeout (a hung child hangs the suite; no flake source seen).
- [4] Bite script: u4d-test-bites.py (this folder). Next: run it (bites B*, E*, probe P).
