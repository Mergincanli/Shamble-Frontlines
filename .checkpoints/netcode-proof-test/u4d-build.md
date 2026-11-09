# U4d build notes: the stand-in sim (npt-sim)

Builder: workflow agent (general-purpose, opus), isolated worktree
/home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-1, base 097f89704d9925dee0dc62f7d7373cb13fbbc9a0
(fast-forwarded; `git log -1` checked). Plan: u4-build-plan.md rev 2 (sha256 7ec1b324..., checked), §5 U4d, §4.2, §4.5,
§4.9, §4.10, §4.11, §4.13, §4.14, §4.15; the advisor's U4 verdict and U4s ruling (advisor-u4.md); carries 7-20, 36-38
(u4s-carries.md). Files: crates/sim only. Commit nothing.

## Plan
Modules (crates/sim/src):
- seam.rs: button bits and press indices on `Command` (carry 17); docs (carry 16); `TickInput` gains `spawns`,
  `teleports` (`Placement`) and `attributes` (`AttributeChange`, `Attribute`), integers only (carry 37); layouts
  completed within FIELDS (carry 12); `HitVolumes` holds `soldiers` and `cover` (`VolumeBox`, integer cm), a
  hand-written `Clone` with a field-by-field `clone_from` (carry 13), `MAX_SOLDIERS`, `MAX_COVER`, `MAX_BYTES`;
  `InputReports::non_finite` dropped (carry 37). Frozen items untouched (Command, Rewind, CoverVariant, EntityKind,
  EntityRecord, the five grid fields).
- history.rs: `for_ms` refuses a ring above 1 MB (carry 14).
- rng.rs: `SimRng`, ChaCha8, one stream per purpose (spread: authority-only; effects; projectiles; respawn).
- greybox.rs: `Greybox` (static boxes, buildings, doors, the wall), `Areas`, `Portals` (plain integer data).
- scene.rs: `Scene` (stress, estimate, course, targets; id and by_id), `StandIns` (burn and top-up as data).
- trajectory.rs: `trajectory_at` (closed form).
- world.rs: `World`: phases (inputs, movers, movement + aim + weapon per command, shots, projectiles, effects,
  respawns, records, volumes), own-copy snap, clamp, rewind blend, `predict`, `load_movers`, `ray_cast`, `RayHit`,
  `with_bite` + `BITES` (bites builds), the finite assert each tick (D-4), the canonical-order assert (carry 8).
- hash.rs: the canonical serializer (u32 words, LE, floats by to_bits), FNV-1a-64, `StateDump` (exactly the hashed
  words; claimed press counters and reports outside: carry 38).
- replay.rs: `Recording` (header with scene, seed, TICK_HZ, first tick, ring span ms, pools; fixed-width LE),
  `Recording::synthetic` (the seeded open-loop generator), `replay::Replayer` (words in, step, hash, dump).
Rules: no new tick-count constant (timers are ms turned into ticks by ticks_for_ms at the call; carry 18 then has
nothing to join; check.mjs is U4a's). Press counters: the last seen counter is the client's claim, kept outside the
hash and the dump; a refused press's twin is the stream with that press removed (its counter one lower from then on).
A firing command with no Rewind, or a refused one, fires at the present volumes (one rule).
Tests (crates/sim/tests/world.rs, replay.rs; seam.rs updated): D-1..D-10, carries 7, 8, 9, 10, 13, 14, 15, 36.
Bites (bites builds): PM-0004-21/hash-map (order folded, digest logged; D-2 reports T), PM-0004-21/std-sin (site
only; proof U4d2's), PM-0001-41/server-speed-1pct, PM-0001-41/server-quant-differs (via own_copy_snap_with_bite).
Scratch plants (Edit, restore, sha256): D-4's 0.0/0.0, plus a few test-guard plants.

## Progress
- 0 base fast-forwarded to 097f897; plan sha256 7ec1b324 checked; read plan §4-§5, advisor-u4.md, u4s-carries.md,
  the sim crate, check.mjs's greps.
- 1 library written (seam.rs edits, history.rs cap, rng.rs, greybox.rs, scene.rs, trajectory.rs, hash.rs, world.rs,
  replay.rs, lib.rs); clippy -D warnings clean, lib, host measured and bites, and wasm32. Base sha256s: grid.rs ede88bbe,
  history.rs 1c62e5aa, lib.rs 073aba27, seam.rs a2031501, ticks.rs 5d852f1a, tests/seam.rs 384746ab (grid.rs, ticks.rs,
  Cargo.toml and clippy.toml unchanged). Note: carry 13's counting allocator cannot compile in npt-sim (workspace
  `unsafe_code = "forbid"`; a GlobalAlloc impl is `unsafe impl`); a storage-identity test stands in, reported as not met.
- resumed after the spend-limit cut (2026-11-09, after 21:44 UTC): HEAD 097f897 checked; files match Progress 1 (the
  library modules present, no test file yet under crates/sim/tests beyond seam.rs); next: the tests (D-1..D-10, carries),
  then bites, then check.mjs twice.
- 2 re-read plan §4-§5, the library and check.mjs. Edits: grid.rs `Config` doc gains carry 20's rule (doc only);
  `HitVolumes::MAX_BYTES` made target-independent (56 + 224 × 20 = 4,536 B on host and wasm32, with a const assert;
  cap 231 ticks = 7,700 ms at 30 Hz), so a header span replays alike natively and in Wasm; tests/seam.rs: `volumes()`
  helper for the new fields, carry 36's exhaustive-match walk in place of `ALL.len() == 7`, carry 12's lists (all seven
  kinds pinned). Next: tests/world.rs and tests/replay.rs; SS health to 1,000 and no reload presses in SS/ES if D-6's
  shot mean needs it (measure first).
- 3 tests written: tests/world.rs (23 measured + 5 bites), tests/replay.rs (6 measured + 1 bites). world.rs fix: a
  soldier killed in the shot phase has `pre[v].vel` zeroed too (its record's grid stays the wire grid of its snapped
  copy: carry 9). First run: D-6 measured 542.15 shots/s (reload presses pause fire 2 s; deaths) -> SS health 1,000 and
  no reload presses in SS/ES (the courses keep them) -> 683.62 shots/s, portal changes 1.00/s: in band. D-8: a reach
  point above 40.95 m saturates in z (the grid's range), so "at the reach" is checked where the reach point is in range;
  idempotence for all. carry 7: per-tick assert_ne wrong on empty ticks -> count. D-2 passes (27,000 ticks, ~28 s
  debug). Measured sizes: SS 9,000 ticks 16,697,066 B (425,345 commands, 47.26 a tick), ES 16,698,782 B, TC 798,759 B;
  a command 39 B; 48 commands 1,872 B + 12. D-7 pinned (world hashes per scene, SS after 1/30/300 ticks, the three
  recordings' lengths and FNV-1a-64). One shared-target clash seen (another copy's npt-sim linked: no `replay` module):
  touched, rebuilt, that run not counted.
- 4 npt-sim green, both builds: measured 37 (seam 8, world 23, replay 6: base 8, so +29), bites 45 (seam 10, world
  28, replay 7: base 10, so +35). Expected workspace totals: 275 + 29 = 304 measured, 292 + 35 = 327 bites. Bites in
  the bites build: hash-map through D-2 reports recording 0, tick 1234 (its arming tick), field
  `bite[0].hash_map_order`, digests 1241f5788e6067a5 vs 995fce5cbed53e85; std-sin first changes the state at tick 101
  on x86-64 (armed 100); -41 speed and quant unit proofs pass; per-instance threads pass. fmt (cargo +1.99.1 fmt -p
  npt-sim, then --all --check clean); clippy -D warnings host all-targets and wasm32 lib, both builds, each showing
  "Checking npt-sim (this copy's path)"; rustdoc -D warnings both builds. Clippy fixes: as_chunks::<4>, two type
  aliases, the constant assert dropped. Plant reference sha256 (first 16): world.rs 57b0cf9c212c8033, seam.rs
  a176cd0d8868e070, history.rs 0c9012ace74fa3bf, replay.rs 8c566b8ae73dd9b9, scene.rs 4d8a04b5769cbaff.
- 5 scratch plants (Edit, run alone, restore, sha256): list P1-P16 in the report; results below, one line each.
  - P1 D-4 `vehicles[0].yaw = 0.0 / 0.0` at elapsed 100 (movers phase): d4_the_state_holds... FAILED "a non-finite
    value in the state after tick 1100: vehicle[0].yaw". Restored, world.rs 57b0cf9c.
  - P2 carry 13 `clone_from` as `*self = source.clone()`: carry13 FAILED "tick 2: soldiers' buffer". seam.rs a176cd0d.
  - P3 carry 7 weapon cost + view_fraction: carry7 FAILED "SS tick 0" (state hash).
  - P4 D-10 a refused cooldown press adds 1 to the weapon clock: d10 cooldown FAILED "step 11: the state hashes".
  - P5 carry 8 order check reduced to slot <=: carry8 FAILED "case 1: an out-of-order input stepped".
  - P6 D-8 CLAMP_SLACK 0.0: d8 FAILED "case 7: [2768, -6822, 0] clamped twice".
  - P7 D-9 blend fraction × 0: d9 blend FAILED "fraction 32: the shot at the reference blend hits".
  - P8 carry 10 `held` ignores tick_a + 1: d9 outside FAILED "tick_a 101, fraction 1: counted" (0 vs 1).
  - P9 carry 14 cap one tick loose: carry14 FAILED "one ms past the edge". history.rs 0c9012ac.
  - P10 carry 15 volumes labelled input.tick: carry15 FAILED "tick 0: the label ignored".
  - P11 D-3 predict keeps FLAG_FIRED: d3 FAILED "CO step 6: record".
  - P12 D-6 doors toggle at hz / 2: d6 FAILED "portal changes a second: 200 hundredths".
  - P13 D-5 pool_full not counted: d5 FAILED "four throws refused and counted" (0 vs 4).
  - P14 carry 9 raw pos taken after the snap: NOT caught at first (the off-grid count summed all 8 fields; yaw etc.
    hid it). Test strengthened (per field: x, y, yaw, pitch, vel.x, vel.y each > 1,000 off grid; z, vel.z none);
    re-run: carry9 FAILED "pos.x: 0 raw values off the grid". Restored 57b0cf9c; the clean test passes.
  - P15 carry 38 / D-10 the hash reads `reports.cooldown`: carry38 FAILED "reports[0].cooldown"; d10 cooldown FAILED
    "step 11: the state hashes".
  - P16 D-2 (measured build) a heap address folded into `spawn_acc` at elapsed 500: first form (page number masked
    to 16 bits) NOT exercised (glibc thread arenas are 64 MB-aligned: about 2 random bits; ASLR is on, mode 2); full
    address: d2 FAILED "first: Some((0, 500, \"spawner[0].acc\"))" after 501 ticks compared.
  - all restored: every src file at its reference sha256; tests/world.rs c9f372e8f7393c97 (the carry 9 change).
