# U4d round 2 (with the npt-proto half of W1-S1): engine-architect (the seam)

Copy under review (read only): /home/user/Shamble-Frontlines/.gate/u4d-s1 (detached at 21a8597, nothing committed).
Round 1 state: /home/user/Shamble-Frontlines/.gate/u4d-prefix/spikes/netcode-proof/crates/sim.
Rulings: advisor-u4-wave1.md W1-S1 (:45-111), W1-U4D (:113-304), W1-PUB (:353-436).
Memory: none (no engine-architect folder under .claude/agent-memory).
Read only: nothing built or run, nothing edited in any copy.

## Plan
1. Inventory: git status / diff --stat in u4d-s1; diff -ru u4d-prefix sim vs u4d-s1 sim (file list); fixer's record
   (u4d-build.md "Fix pass, round 1"), s1-proto.md.
2. A-M1: the records-to-volumes function: one public function, called by phase 8 (authority, own records) and by
   load_movers (decoded), geometry consts private; its test (every SS tick, volumes == f(records)); hull o part
   composition in one home (H1).
3. A-M2 (carried to U4g): does the owner-only record + SOLDIER record hold everything to reset the own soldier
   (velocity raw/grid, ammo, wait, reload_left, move_speed, fire_interval)? Anything else per-soldier that decides
   later ticks and sits in neither (claimed counters: by ruling from the client's command; others?)?
4. Freeze: against 21a8597: EntityRecord::FIELDS 12; Command, Rewind, CoverVariant, five grid fields unchanged;
   EntityKind only gains one variant appended last in ALL; KIND_BITS 3.
5. Boundary npt-sim -> npt-proto: proto reads layout() and ALL at run time, copies no layout; the section rule reads
   GROUPS (one home); the GROUPS test's three checks; encoder and decoder refusal.
6. New public items vs W1-PUB and §4.1 (Weapon, Scene::weapon, Scene::health, the volumes function, refusal records,
   the new kind).
7. Fixer's open points A, B, C: weigh.
8. Other round-1 lows (C, D, E, F) status where the fix touched them.
9. Findings + verdict.

## Notes
- Step 1 done. u4d-s1 at 21a8597: proto 5 files modified; sim 5 modified + 9 untracked. diff -rq prefix vs s1 sim:
  9 files differ (history, lib, replay, scene, seam, world; tests replay, seam, world) = the fixer's list
  (u4d-build.md "Files changed by the fix pass"); grid, greybox, hash, rng, ticks, trajectory unchanged since round 1.
- Step 4 (freeze) vs 21a8597, by block extraction (awk) and diff: `struct Command`, `struct Rewind`, `enum CoverVariant`
  byte-identical; `struct EntityRecord` doc line only; `enum EntityKind` docs + `OwnState` appended last; `ALL` 7 -> 8,
  OwnState last (seam.rs:288-298); FIELDS 12 (seam.rs:360); grid.rs diff vs base = one doc paragraph (Config, from
  round 1) only, the five fields untouched; SimCore trait = one doc line (reports). KIND_BITS derived from ALL.len()
  (proto snapshot.rs:182), still 3 (unit assert :739). Freeze: HELD.
- Step 2 (A-M1): pub `World::hit_volumes_from(&self, records, tick, into)` (world.rs:882-891) is a thin wrapper over
  private `compose_volumes` (:483-545); phase 3 `present_volumes` (:1475-1480) and phase 8 (step :2107-2108) call it
  through `compose_own_volumes` (:862-872) on the authority's own records; `load_movers` (:1022-1033) calls the same
  body on decoded records; World::new (:846-848) too. Hull o part: `part_center` (:446-456), used by compose_volumes
  (:540) and by `load_movers_into`'s collision boxes (:2054-2057): one home. Geometry consts stay private
  (SOLDIER_HALF/PLATE_HALF :82-83, PART_HALF_CM :85). Test a_m1 (tests/world.rs:904-941): every tick of 1,200 SS ticks,
  authority == f(own records), a second World == f, storage pointer kept, load_movers == f with its own tick label (8);
  dead boxes seen. Plants Q8, Q9 fail it. A-M1: CLOSED.
- Step 3 (A-M2): Soldier struct (world.rs:271-291) vs the two records: pos/yaw/pitch/flags/health/generation in
  SOLDIER (:1743-1762); vel (pre-snap raw + grid), ammo, wait, reload_left, move_speed, fire_interval in OWN_STATE
  (:1767-1786), each Field-ranged (seam.rs FIRE_TIMER/RELOAD_LEFT ranges from the const-asserted maxima; wait bounded
  by run_command :1371 and :1401; attribute changes clamped :1116-1119; scene starts asserted :689-695). `alive` =
  !(flags & FLAG_DEAD) (set together :1610-1612, cleared together in revive :1137-1142). `claimed` from the client's
  own command (ruled, §4.2:241-248). Not in any record: `respawn_left` (:274) - irrelevant to predict (run_command
  returns for a dead soldier :1289) and 0 whenever alive (revive :1138; only decremented while dead :1719), but a
  D-3-from-reset test of a dead own soldier compares it (named "soldier[0]." includes it). A-M2's data: supplied, as
  W1-S1 says; the entry stays U4g's carry (W1-U4D :240-247). -> note for U4g (Low, new A2-L1).
