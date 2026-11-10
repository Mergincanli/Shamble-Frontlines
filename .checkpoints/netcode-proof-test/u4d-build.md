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
- resumed after the second cut (about 22:35 UTC 2026-11-09; resumed 2026-11-10): HEAD 097f897 checked; every src file
  at its Progress 4 reference sha256 (world.rs 57b0cf9c, seam.rs a176cd0d, history.rs 0c9012ac, replay.rs 8c566b8a,
  scene.rs 4d8a04b5); no plant left (grep). tests/world.rs now 41ffc20ff978ad9a, not the c9f372e8 noted: its carry 9
  test is the strengthened form (per-field off-grid counts); the likely cause is a fmt pass after the note (the names
  array is in rustfmt's wrapped form); to be confirmed by fmt --check and a full run. Files touched at 22:32: the cut
  landed in or before check.mjs run 1. Next: fmt check, npt-sim counts, then check.mjs twice.
- 6 `cargo +1.99.1 fmt --all --check` clean (so tests/world.rs is in fmt form); touched, npt-sim measured 37 (replay 6,
  seam 8, world 23; "Compiling npt-sim (this copy's path)"), --list 37. check.mjs run 1 started (background).
- 7 check.mjs attempt at 06:35 NOT COUNTED (a shared-target clash): measured 304 (right), but the bites build linked
  another copy's npt-sim (HitVolumes with `tick` only: E0560/E0609 in tests/world.rs), and clippy-wasm32 took 1.3 s
  (likely another copy's fresh fingerprint, so my code unchecked there). Cargo keys a workspace member's fingerprint
  without its path and keeps dep-info paths package-relative, so a copy that builds after my touch makes my stale
  sources look fresh. Remedy: re-run with a touch of every crate file every 2 s for the whole run (so each cargo call
  in it rebuilds from this copy), then confirm by separate clippy and --list runs that show this copy's path.
- 8 MY ERROR, 06:38-06:41 UTC, NOT COUNTED: the command `cd $C && df && export && (touch loop) &` backgrounded the
  whole && list, so `cd` and the exports stayed in the background job and check.mjs ran in the MAIN FOLDER
  (/home/user/Shamble-Frontlines), without CARGO_TARGET_DIR or CARGO_NET_OFFLINE: all 10 steps passed there (275/292,
  refs 564, redact 157: the main folder's tree, not mine). What it wrote: 408 build files in the main folder's own
  gitignored cache, .gate/netcode-proof-test/target (its .cargo/config.toml target-dir); check.mjs writes no source
  file (fmt --check, clippy, doc, test, cargo metadata, git ls-files, judge --check-marks, node tests). Not undone (a
  cache; removing it would be another write there). Reported to the main session. Fix: `cd "$C" || exit 1` on its own
  line, the loop alone backgrounded, `pwd` printed.
- 9 check.mjs RUN 1 (counted), 06:42:05-06:45:50 UTC, cwd this copy, shared target, offline, 2 s touch loop: all 10
  steps pass; test: measured 304 (275 + 29), bites 327 (292 + 35), 0 failed, 0 skips; refs 562; marks 81; self 48 of
  48; redact 10 of 10, 166 items, 0 findings. Then clippy -D warnings run alone, touched before each: wasm32 (the 8
  WASM_CRATES) measured and bites, host --workspace --all-targets measured and bites: each prints "Checking npt-sim
  v0.0.0 (this copy's crates/sim)", 0 warnings (incremental: a touch-only recheck takes 0.3-1.5 s, which is why the
  steps are fast). Next: run 2.
- 10 check.mjs RUN 2 (counted), 06:46:28-06:49:59 UTC, same set-up: all 10 steps pass, the same counts (304 / 327, 0
  failed, 0 skips; refs 562; marks 81; self 48; redact 166 items, 0 findings). Then, touched: `cargo test -p npt-sim
  -- --list` measured 37 (replay 6, seam 8, world 23) and bites 45 (replay 7, seam 10, world 28), each "Compiling
  npt-sim (this copy's path)", every new test named; `cargo +1.99.1 fmt --all --check` clean. Final sha256 (first 16):
  greybox 2d75cb0b, grid a0e24c08, hash 4663b2a8, history 0c9012ac, lib 70eb8262, replay 8c566b8a, rng d6b61bbe,
  scene 4d8a04b5, seam a176cd0d, ticks 5d852f1a (unchanged), trajectory 2c0968b3, world 57b0cf9c; tests replay
  a547314c, seam d91b8c0c, world 41ffc20f. Nothing committed.
DONE (2026-11-10 06:52 UTC).

## Fix pass, round 1
Fixer: workflow agent (opus), same worktree, base 097f897, started 2026-11-10 15:55 UTC. Builds to the advisor's
W1-U4D (advisor-u4-wave1.md:113-304) and W1-S1's npt-sim half (:45-112), read whole. Round 1's notes read:
u4d-review-{netcode,rust,arch,test}.md. Files: crates/sim (src, tests) only; commit nothing; Write/Edit only; plants
by Edit, restored, sha256-checked. Reference sha256 at start = the builder's final list (checked 15:50).

### Plan
1. Lows first (they touch few lines): T-M2 (history.rs cap 10^6 B: 220 ticks, 7,333 ms; carry 14's edge and the
   replayer's span refusal move); rust N4 (record() debug-asserts each integer in its field's range); rust L1 (decode
   caps its pre-allocation by the bytes present); rust L2 (Replayer fails a non-canonical tick); test L1 (D-4's finite
   check reads the pre-snap values too); W1-PUB nit (Recording at npt_sim::replay::Recording only).
2. W1-S1 + N-M3: `EntityKind::OwnState` appended last (8 kinds): velocity x, y, z (VELOCITY; raw pre-snap, grid of
   it), ammunition, the fire timer (`wait`), the reload left, move speed, fire interval, each `Field::integer` with a
   range the World cannot leave (attribute values brought into stated bounds on entry, so `wait` fits i32; asserted).
   SOLDIER: position x, y, z, yaw, pitch, flags, health, weapon id (the list the ruling and catalogue :137 give: 8
   fields; the ruling's "(9 fields)" counts 8 names: reported). FLAG_FIRED dropped (flag bits renumbered dense).
   predict refreshes both records of its slot. FIELDS stays 12.
3. H1 + A-M1 + R-M1: a part's record holds its transform relative to its hull (offset x, y, z, yaw; health; parent),
   offsets on the cm grid, asserted in range. One function composes hull ∘ part (grid integers); one public function
   builds `HitVolumes` from records into existing storage (`World::hit_volumes_from`); phase 3 and phase 8 call it on
   the authority's own records; `load_movers` keeps the last hull and part integers per entity (sized at `new`, never
   grown: an unknown index is skipped) and composes its collision boxes with the same function.
4. N-M5: an effect row records the tick it ends; the per-tick expiry check stays.
5. N-M4: the 4 very large units loop inside the fight (radii 4, 7, 10, 13 m round the cluster's centre: parts within
   16.7 m); the other 11 loop 52-82 m out; stand-in projectiles start within ±12 m, horizontal speed ±3 m/s.
   N-M1: SS's magazine 32,000 (covers 35 min at 14.58 shots/s: 30,625), a const assert, and ES's at its own rate.
   TUNE-1: `Scene::weapon()` and `Scene::health()` for the run header.
6. T-M1: ES meets PM-0001-2's four ES bands (72 shots/s, 10 gameplay events/s, 32 live projectiles, 21.12 spawns/s);
   accepted throws count toward both projectile bands, the stand-in spawner fills the rest; measured first.
7. N-M2's code half: `InputReports::refusals`, records `{ slot, seq, kind }`, cleared each step; D-10 asserts them.
8. T-M3: a unit test in world.rs's own module pins `blend`'s boxes against a reference for fractions 0, 1, 127, 128,
   255 (E6's / 255 must fail it).
9. TUNE-2: the z bound measured, stated in World's doc and D-8; an in-module test that no scene's spawn points,
   plates or greybox surfaces come near it.
10. Tests: s6 and carry 12's pins; D-3 compares the own-state record and gains a catch-up tick; carry 9 per field over
    both records; the hull test (parts' records unchanged while the hull moves and turns); probe K's count as a test
    (part records changed ≤ parts whose health changed); A-M1's volumes test; R-M1's test (probe C's case); D-6
    re-measured; ES's band test; D-7 re-pinned.
11. Plants (Edit, run alone, restore, sha256) for each new or changed test; then fmt, clippy, rustdoc, check.mjs twice.

### Progress
- F0 plan written (15:57 UTC). df 12 GB free; no other cargo/rustc running at 15:50.
- F1 RESUMED by a second fixer (2026-11-10 20:40 UTC; the first was cut after F0 without logging its edits). Found:
  9 files differ from the builder's final state, which .gate/u4d-prefix holds (all 15 sha256 there = the builder's
  list); every crates/sim file has mtime 16:24:00 (the run script's touch loop: the cut run's last cargo call ended
  then). Diff read whole. Done in code by the cut run (not yet built or proven by me): T-M2 (history.rs 10^6 B; tests
  carry14 220/7,333, replay span 7,333/7,334), rust N4 (record()'s debug_assert), rust L1 (decode cap by bytes), rust
  L2 (Replayer fails a non-canonical tick), test L1 (non_finite reads `pre`), W1-PUB (re-export dropped; tests use
  npt_sim::replay::Recording), W1-S1 (OwnState last in ALL, 8 fields; SOLDIER 8; FIELDS 12; ranges + seam consts;
  attribute values brought into range), N-M3 (FLAG_FIRED dropped, flags renumbered), H1 + A-M1 + R-M1 (Part holds
  offset_cm; part_center, compose_volumes, pub World::hit_volumes_from; phase 3 present_volumes + phase 8 from the
  records; movers sized at new, never grown), N-M5 (Effect.end, record TICK field), N-M4 (radii 5-14 m for the large,
  52-82 m others; stand-ins ±12 m, ±3 m/s), N-M1 (SS 32,000, const assert both scenes), TUNE-1 (pub Weapon,
  Scene::weapon/health), the refusals (Refusal, RefusalKind, InputReports::refusals, cleared per step), T-M3 (unit
  test in world.rs), TUNE-2 (doc + in-module scene test + D-8 at the bound), T-M1 partly (throws take a spawn from the
  accumulator; a temporary `zz_probe_es` test still in world.rs: ES's spread not yet tuned). Tests added: D-3 catch-up,
  h1 x2, n_m5, a_m1, r_m1 (+ module test), t_m1, w1_s1, D-6 in-fight asserts, D-10 refusals, carry 9 per kind, s6 pins.
  NOT done: D-7's pins (tests/replay.rs unchanged there), ES tuning, any run's result, plants, notes. Next: build and
  run npt-sim as it stands, then finish T-M1 and the rest, then plants.
- F2 first runs of the cut run's tree (20:41-20:45 UTC, this copy's path, no other cargo): builds; measured npt-sim
  all pass but D-7 (pins: SS 6,488 words, was 6,984: 124 parts × 4 transform floats left the state) and T-M1 (ES:
  shots 70.52/s in band, spawns 21.120/s in band, gameplay events 36.83/s and live projectiles 34.07 OUT). Numbers
  printed: H1 part records changed 2,850 = parts hit 2,850 over 600 SS ticks (4.75 a tick of 124; effect rows 7.74 a
  tick of 384); D-6 SS 685.87 shots/s, portal 1.00/s, 28 kills in 100 s (0.28/s), large hulls <= 1,400 cm out,
  others >= 5,199 cm, stand-ins start <= 1,200 cm.
- F3 T-M1 finished. Cause of live 34.07: a grenade fused after 2 s (60 ticks), a stand-in after 1.515 s (46 ticks);
  ADR 0001:1478 says ES's projectiles each fuse after 1.515 s, so Scene gains `grenade_fuse_ms` (2,000; ES 1,515): live
  32.13-32.30. Events: spread cannot reach 10/s (probe: spread 5,000 -> 300,000 µrad gives 45.7 -> 28.3 events/s;
  blast hits alone 6.8-7.8/s), so Scene gains `aim_one_in` (1; ES 11): ES's recording aims among the other teams on
  one command in 11 (and on every command that throws), into the air 40 m above the shooter's reckoning on the rest;
  the extra draw is made for ES only, so SS's, CO's and TC's recordings are byte-identical (D-7: SS and TC lengths and
  FNV unchanged). Measured over 9,000 ticks: 1 in 10 -> 10.39 / 10.36 / 9.91 events/s (seeds 2, 1, 3); 1 in 11 ->
  9.85 / 10.26 / 9.67; 1 in 12 -> 9.52 / 10.09 / 9.51; chose 11. Sky-aimed shots still hit 1.3-1.5/s (soldiers pass
  through each other, so a ray from inside a neighbour's box hits it): docs say so. FINDING for the advisor: ES's
  event rate drifts down over a run (per 50 s window at 1 in 10: 13.4, 11.0, 10.1, 9.4, 9.7, 8.7/s; mean distance from
  home 10.1 -> 16.6 m), because the generator's dead reckoning ignores crouch, sprint and scripted move speeds; the
  9,000-tick mean is in band, a longer or windowed ES run would read lower. Not fixed (it changes every scene's
  recordings; beyond the rulings). The probe (temporary `zz_probe_es` in world.rs, env-driven) is deleted. MY SLIP:
  one probe edit (its seed from the environment) was made by a python one-liner, not Edit (the rules say Write/Edit
  only); probe code only, since deleted.
- F4 A-M1's "load_movers on decoded ones": the cut run's load_movers only refreshed the movers; now it also sets the
  world's present volumes to hit_volumes_from(records), labelled with its own tick counter (next_tick - 1; carry 15);
  the a_m1 test checks it each tick. World's struct doc states the clamp bound (TUNE-2; the module doc is not in
  rustdoc, the module being private). rust L2's test: a disordered tick decodes and the Replayer returns Failed.
  D-7 re-pinned: SS 6,488 / 6,786,077,935,583,382,297; ES 1,944 / 8,780,045,697,152,956,090; CO, TC unchanged; SS after
  1/30/300 ticks (6,772 / 10,412,709,864,748,183,085), (6,628 / 8,845,330,375,396,163,272), (6,612 /
  923,332,457,071,349,262); ES recording 16,697,144 B / 7,136,420,146,508,984,890 (SS 16,697,066 and TC 798,759
  unchanged). Next: full runs (measured, bites), clippy, rustdoc, then plants.
- F5 green (21:00-21:04 UTC, this copy's path each time): measured npt-sim 47 pass (lib 3, replay 6, seam 8, world 30;
  was 37: +10), bites 55 (lib 3, replay 7, seam 10, world 35; was 45: +10); T-M1 70.776 shots/s, 9.853 events/s,
  21.120 spawns/s, 32.300 live; hash-map bite via D-2 still tick 1234 `bite[0].hash_map_order`; std-sin tick 101.
  rustfmt: the cut run's code had 25 unformatted hunks; applied rustfmt's own diff by Edit (not `cargo fmt`, so files
  are written by Edit only); clippy's assertions_on_constants in the TUNE-2 test -> a const assert. fmt --all --check
  clean; clippy -D warnings host all-targets and wasm32 lib, measured and bites; rustdoc -D warnings both: clean.
  Reference sha256 for the plants (full list in the scratchpad's f-ref.sha256; first 16): history f11e9b59, lib
  396e663d, replay 3b80b11b, scene c452d329, seam 0c41b406, world 2f3cde3d; tests replay e483603f, seam 4900af4e,
  world 721c1c96; unchanged from the builder: greybox, grid, hash, rng, ticks, trajectory; sim Cargo.toml fb98df48,
  clippy.toml 4fd9facc, check.mjs 6d1ed188 (= base).
- F6 plants (Edit in place, the one test run alone from this copy, restored by Edit, then `sha256sum -c` of every
  file against the reference: ALL-RESTORED after each):
  - Q1 N-M3, P11's form (a fired flag bit cleared per tick in step, per command in predict): d3 FAILED "CO step 4:
    record" (flags 16 vs 0; step 4 is the first catch-up tick).
  - Q2 W1-S1, predict leaves the own-state record stale: d3 FAILED "CO step 0: own-state record".
  - Q3 refusals, a pool-full refusal names seq + 1: d5 FAILED (seqs 17-20 vs 16-19).
  - Q4 N-M4, the very large units at 40 + 3v m again: d6 FAILED "the very large units: 4900 cm out".
  - Q5 N-M4, stand-ins start within ±60 m again: d6 FAILED "stand-ins start 6000 cm out".
  - Q6 H1, a part record's x is its hull's world x plus its offset: h1 (hull) FAILED "tick 1: the parts' records";
    h1 (health) FAILED too.
  - Q7 N-M5, an effect record holds its ticks left: n_m5 FAILED "tick 1: row 0 ...".
  - Q8 A-M1, phase 8's volumes nudged after the function: a_m1 FAILED "tick 0: the authority's".
  - Q9 A-M1, load_movers no longer builds the volumes (the cut run's form): a_m1 FAILED "tick 0: a client's volumes
    after load_movers".
  - Q10a R-M1, load_movers_into grows the part movers to a decoded index: the module test FAILED (an index panic in
    CO's composition, no hull); Q10a' the doors grown instead: FAILED "CO: no mover grew" ([.., 100] vs [.., 6]).
  - Q10b R-M1 (probe C's case in the new code), compose_volumes grows the cover to an unknown part index: r_m1 FAILED
    at its final hit_volumes_from check, (1, 401) vs (1, 7).
  - Q11a T-M1, a throw takes no spawn from the accumulator: t_m1 FAILED "live projectiles: 37726" (spawns 24,660).
  - Q11b T-M1, ES aims at the teams on every command: t_m1 FAILED "gameplay events: 36406".
  - Q11c T-M1, ES's grenade fuse 2 s: t_m1 FAILED "live projectiles: 33948" (top 33,600).
  - Q12 T-M3, E6's blend by / 255: the blend unit test FAILED "the soldier, fraction 1, axis 0" (3.9 mm off).
  - Q13 refusals not cleared per step: d10 cooldown, ammunition, reload and rewind FAILED (clamp: none to repeat).
  - Q14 an ammunition refusal reported as Cooldown: d10 ammunition FAILED (Cooldown vs Ammunition).
  - Q15 FLAG_RELOADING never set: d10 reload FAILED "step 50: flags 0b0".
  - Q16 carry 9, the own-state record's raw velocity after the snap: carry9 FAILED "vel.x: 0 raw values off the grid".
  - Q17 W1-S1, the own-state ammunition one low: w1_s1 FAILED "CO tick 0 slot 0: ammunition".
  - Q18 rust N4 + W1-S1, a scripted move speed not brought into range: w1_s1 FAILED in record()'s debug_assert
    "OwnState field 6: -1 outside 0..=10000".
