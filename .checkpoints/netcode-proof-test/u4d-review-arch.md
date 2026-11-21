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
- Other lanes: no reference to non_finite, `HitVolumes {`, `TickInput {` outside crates/sim in U4a's, U4c's, U4b's
  worktrees or the main folder (grep). The builder's main-folder slip (u4d-build.md:112-119) wrote only that folder's
  gitignored cache; rust reviewer saw main git status clean of source.
- FINDING D (low): two tick owners (TickInput::tick label vs World::next_tick, seam.rs:112-113; world.rs:1806); ring
  keyed by world ticks, Rewinds by the room's; nothing checks agreement; carry 15 test pins the label is ignored
  (tests/world.rs:1377-1403). Fix: room-side debug assert (U4e carry) + Replayer refuses non-consecutive labels.
- FINDING E (low): dump() builds a fresh StateDump (words + names) per call (world.rs:738-742, replay.rs:618-620);
  hash.rs:63-64 points at npt-hash-wasm's per-word `dump_word`: per-word dump = O(n^2) across the Wasm boundary.
  Carry to U4d2: dump once per mismatching tick, export from the held dump.
- FINDING F (nit): SimRng narrower than plan.md §4.7 (rng.rs:12 documented) -> list in report; Recording at two
  paths (lib.rs:17 + :28); Greybox::dm_tri pub (greybox.rs:83) = second route to the static graph beside
  World::greybox (world.rs:728); Recording::synthetic in library (replay.rs:351) vs "test support" (§5 U4d, §4.15):
  OK for U4d2's regeneration, say so. Builder's "for the advisor" list overstates: Greybox, Areas, Portals,
  StandIns, trajectory_at named at plan.md:582-585; Placement/AttributeChange/Attribute/VolumeBox/FieldDiff/Ray/
  RayHit/replay::Step are plain-data types of named items.
- Concur: netcode H1 (part world transforms) is fixable inside the freeze (same fields, parent-relative values) and
  then makes A's one-home records->volumes function more needed. Netcode/rust M2 = my C (ownership angle only).
- Verdict draft: APPROVE WITH FINDINGS (seam kept; A, B routed as carries; C-F small).
- Final: APPROVE WITH FINDINGS. 2 medium (A volumes-from-records home; B own-state reset and FIELDS full), 3 low
  (C claimed ownership, D two tick owners, E dump per word), 1 nit group (F). Read only; nothing built or run.
DONE (2026-11-10)
