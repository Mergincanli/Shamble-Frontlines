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
