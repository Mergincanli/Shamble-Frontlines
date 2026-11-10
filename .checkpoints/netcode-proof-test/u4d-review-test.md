# U4d build proof: test-engineer

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-1 (uncommitted, base 097f89704d99).
Plan: u4-build-plan.md §5 U4d, §4.2, §4.10, §4.13-4.15, §7.1. Reviewer notes: u4d-review-netcode.md, -rust.md,
-arch.md. Builder notes: <worktree>/.checkpoints/netcode-proof-test/u4d-build.md. Memory: none (memory: local, no folder).
Rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target, CARGO_NET_OFFLINE=true; `cd <wt> || exit 1`
on its own line; touch crates before each build and every 2 s during counted runs; no other cargo/rustc before a
counted run; df > 2 GB; plants in place, one at a time, restore + sha256; commit nothing.

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
