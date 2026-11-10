# U4d build review: engine-architect (the seam)

Worktree (read only): /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-1, base 097f8970.
Plan: .checkpoints/netcode-proof-test/u4-build-plan.md (§5 U4d, §4.1, the freeze); rulings: advisor-u4.md.
Memory: none found (memory: local, no folder).

## Plan
1. Read plan §4.1 (npt-sim public rows), §5 U4d, the freeze rules, the advisor's rulings on U4d.
2. git status / diff --stat in the worktree: files touched, nothing outside crates/sim, no manifest/lock.
3. SimCore trait as built (seam.rs) vs the plan: signature, frozen items unchanged.
4. Public items vs §4.1 (lib.rs re-exports, each pub item).
5. npt-sim depends on nothing above it (Cargo.toml deps, use statements).
6. Other lanes: nothing touched outside crates/sim.
7. Hidden coupling, state ownership, boundary cost.
8. Findings + verdict.

## Notes
- Step 1-2 done. Worktree files and sha256 (first 16) match the builder's report; base 097f8970; changed files all
  under crates/sim (5 modified, 9 new src/tests); workspace Cargo.toml, Cargo.lock, sim Cargo.toml, clippy.toml equal
  to base. Builder checked plan 7ec1b324; current plan 0d69332c (F1 edits §4.15 sizes only: no effect on U4d's rules).
- Step 3: SimCore trait unchanged from U4s (seam.rs:465-486 no diff lines). Freeze: Command fields unchanged (consts
  added via impl block: carry 17 allowed); Rewind, CoverVariant unchanged; EntityKind variants unchanged (docs only);
  EntityRecord fields unchanged, FIELDS 12; layout() appended U4d fields after U4s's (no reorder), within 12; grid diff
  doc only; non_finite dropped (allowed). TickInput gains spawns/teleports/attributes (allowed). HitVolumes gains
  soldiers/cover, derive Clone -> hand-written, + Eq (not frozen; §4.2 makes volumes U4d's).
- plan.md §4.7 npt-sim names Greybox, Areas, Portals, SimRng, trajectory_at, StandIns, Scene::stress, World,
  HitHistory, SimCore "with its plain-data input and output types": so the builder's "for the advisor" list overstates
  the gap (Greybox, StandIns, trajectory_at are named). SimRng named in §4.7 but kept private (mod rng, no re-export).
- U4a (agent-a929… worktree) uses Command, EntityKind, EntityRecord, SimEvent, grid; reads layout() dynamically
  (snapshot.rs:86,131,151,...; hash.rs:56); a8 pins only the record-free vectors (wire.rs:569, :654-655). No U4a use
  of TickInput, HitVolumes, InputReports. So U4d's additions do not break U4a's code at merge (by inspection).
- Step 4-5: deps: sim src uses only crate::, core, libm, rand_chacha, rand_core; std only in the bites HashMap
  (world.rs:435) and String/format (hash.rs, dump names). No std::time/fs/io/env/process in src. Manifest unchanged.
- SimCore impl world.rs:1799-1878: step order assert (debug), events cleared per tick, records reuse storage
  (world.rs:1543 mem::take+clear), volumes rebuilt in place. Hash and dump one canonical walk, three sinks (hash.rs).
- FINDING A (medium): no public way to build `HitVolumes` from decoded records: build_volumes private
  (world.rs:1211-1249) with private geometry consts (SOLDIER_HALF, PLATE_HALF :63-64, PART_HALF_CM :66); predict and
  load_movers never rebuild volumes, so a client World's hit_volumes() is stale from new(). HitVolumes doc
  (seam.rs:330-333) says "a client can build the same volumes from what it decoded (ray_cast's movers)": no path.
  U4h's AimCast would copy the sim's geometry into npt-client (U4-HIT: all float geometry stays in npt-sim).
- FINDING B (medium): no entry to reset the predicting World's own soldier to server state (U4g Predictor: "reset to
  the server's state for the last command run"); D-3 (tests/world.rs:201) clones the authority each step. Soldier
  record (12 = FIELDS, full) lacks ammo, wait, reload_left, move_speed, fire_interval (world.rs:245-265), and the
  claimed counters (:481); catalogue :136 own state wants weapon state (ammo, fire timers, reload) and the attribute
  rows (PM-0001-60). FIELDS frozen at 12 in wave 1 -> seam-shape question for the main session/advisor before U4g.
- FINDING C (low): `claimed` press counters decide later ticks but are outside hash and dump (world.rs:480-481,
  :1645-1647; hash.rs:8-9) while write_state's doc says "every field that decides a later tick"; condition 1's words
  "never what the client claimed": the build keeps the claim (outside the hash). Meets the intent (twin equal);
  a reading -> state it to the advisor; doc the exception.
