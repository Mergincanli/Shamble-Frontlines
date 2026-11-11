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
- Step 5 (boundary sim -> proto): proto reads `kind.layout()` at run time everywhere (snapshot.rs:93, :138, :158,
  :199, :215; hash.rs:56; msg.rs:649) and KIND_BITS/put_kind/get_kind from `EntityKind::ALL` (:182-195): no layout
  copied in the codec. The only literal widths are the GROUPS test's remote-row pin [16,16,12,16,10,8,10,4]
  (snapshot.rs:742-747), the security proof form, intended. Section rule: private `carries(groups, section, kind)`
  (:173-179) reads the message's GROUPS table; put_record/get_record (:222-254) and put_delta/get_delta (:266-340)
  check before writing / right after reading the kind; Snapshot::put/get (:544-549, :591-607) and FullState::put/get
  (:681-685, :703-707) pass their own table. One home per message: HELD. OwnState in both own-state rows only
  (:490-498, :646-655). No crate outside sim/proto names EntityKind or a layout (grep of crates/: none), so the new
  variant breaks no other crate's match. Nit: tests/common/mod.rs `section_kinds` re-implements carries()'s
  predicate as the refusal test's oracle (a second copy of the rule, test-only; the GROUPS unit test is the
  catalogue-facing check).
- npt-sim deps: src imports only core, crate, rand_chacha, rand_core (+ libm by path); Cargo.toml unchanged
  (not in git status). Dependency direction: HELD.
- Step 6 (public items): prefix -> s1 pub delta (per-file grep diff): + Refusal{slot,seq,kind}, RefusalKind,
  InputReports::refusals (seam); + World::hit_volumes_from (world); + Weapon (6 pub fields), Scene::weapon,
  Scene::health (scene); ALL 7->8; - lib.rs `pub use replay::Recording` (W1-PUB nit done). §4.1 row :184 already names
  the own-state kind, the volumes function and the refusal records; it does not name Weapon, Scene::weapon,
  Scene::health. They trace to TUNE-1's condition ("each run's header records the weapon ... and the health",
  advisor-u4-wave1.md:252-253): a planned consumer (the run header), not "for later". New weapon-clock consts are
  pub(crate) (seam.rs UNITS_PER_TICK, MOVE_SPEED_MAX_CM_S, FIRE_INTERVAL_MAX_US, RELOAD_MAX_MS). -> Low A2-L2 (text).
- Step 7 (fixer's open points):
  - (A) `grenade_fuse_ms` (scene.rs:110) is read by the World (world.rs:1466); 1,515 ms is 32 / 21.12 (Little's law
    on ADR 0001:1478's two rates), the same derivation the ES doc already states; the field doc cites ":1478" for a
    number the line does not state (wording). `aim_one_in` (scene.rs:115) is read only by `Recording::synthetic`
    (replay.rs:462; grep: no World read): input-source tuning stored in the world's scene data. Both pub(crate): no
    public change. No seam objection; the tuning call is the advisor's (like TUNE-1).
  - (B) the drift: the generator's dead reckoning (replay.rs:431-432: est += mv x scene.move_speed / TICK_HZ) is a
    second, partial movement model (no crouch /2 or sprint x1.5 of world.rs:1296-1302, no scripted MoveSpeed from
    replay.rs:515-519, no collision). Its effect is bounded to the synthetic ES recording (D-2, U4d2, t_m1), whose
    9,000-tick mean t_m1 pins. The judged ES runs (marks.json "ES runs": scene ES, clients 48) are U4r's, driven by
    npt-bots (plan §5 U4r :742; §4.15 :494-495), so neither knob nor drift reaches them; §4.12's contract test resolves
    PM-0001-2 on SS suites only. -> Low A2-L3 (carry to U4r).
  - (C) SOLDIER 8: the ruling's own list (advisor-u4-wave1.md:79, 8 names), its test's widths (:91-92, 8),
    catalogue :137 and ADR 0001:309-311 (8) agree: "(9 fields)" is a count slip. Weapon/Scene::weapon/Scene::health:
    A2-L2. Carry 13: met in part, ruled C13 (:269-277).
- Step 8 (round-1 Lows): C (claimed outside hash) CLOSED by W1-U4D N-M2 + write_state doc (world.rs:1900-1903) and
  field doc (:633); D (two tick owners) NOT CLOSED, a listed follow-up (u4d-build.md "Left for"), load_movers labels
  with its own tick (:1024); E (dump per call) NOT CLOSED, follow-up to U4d2; F: Recording one path CLOSED (lib.rs
  :25-35), SimRng / dm_tri / synthetic ruled by W1-PUB.
- Observation (no finding; for U4e's sizes): FIRE_TIMER and RELOAD_LEFT count down by UNITS_PER_TICK each tick
  (world.rs:1371-1374), so a firing or reloading soldier's own-state record changes every tick (29-bit fields by
  their ranges, seam.rs:232-239): N-M5's pattern, but owner-only, one record per client; and the remote soldier
  record lost 48 bits of velocity. U4e measures.

## Findings (round 2)
- A-M1: CLOSED (step 2 evidence).
- A-M2: carried to U4g by W1-U4D (:240-247); the data W1-S1 promised is there (step 3). Status: data CLOSED, entry
  NOT CLOSED by design (U4g's carry).
- A2-L1 (Low) world.rs:274, :1719, :1289: `respawn_left` is in neither record; predict never reads it (a dead soldier
  returns at :1289) and it is 0 whenever alive, but U4g's reset test (D-3's rule from a reset, comparing
  "soldier[0]." words) would differ on it at a dead tick. Scenario: U4g resets from the records of a tick where the
  own soldier is dead; respawn_left stays at the predicting World's old value; the bit-for-bit compare fails or the
  test author widens it. Cheapest proof: in U4g's reset test, a reset at a dead tick. Fix: U4g's carry says the
  reset sets respawn_left 0 for a live soldier and that a dead own soldier is not predicted (the test resets from
  live ticks or names respawn_left as authority-only). Follow-up, not before merge.
- A2-L2 (Low) lib.rs:28 / scene.rs:46-59, :290-297 vs u4-build-plan.md:184: `Weapon` (6 pub fields),
  `Scene::weapon`, `Scene::health` are public and traced to TUNE-1's header condition (advisor-u4-wave1.md:252-253),
  but §4.1's npt-sim row and W1-PUB do not list them. Fix: one text line in §4.1's row (main session) and U4d's report
  lists them for the advisor. Follow-up (text), not before merge.
- A2-L3 (Low) scene.rs:115 / replay.rs:431-432, :462 vs marks.json "ES runs" / plan §4.12, §5 U4r :742: ES's T-M1
  fix lives in the synthetic generator (aim_one_in, a generator knob stored in the world's Scene, read only by
  replay.rs), and its dead reckoning is a second, partial movement model (the drift). The judged ES runs are
  bot-driven in U4r and the contract test checks PM-0001-2 on SS suites only, so nothing shows the brawl bots meet
  ES's bands before PM-0001-12's ES runs. Fix: a U4r carry, "the contract test also shows PM-0001-2's ES checks on
  a short ES run of the brawl Behaviour (W1-U4D T-M1's bands)"; the drift accepted as stated for the 9,000-tick
  recording. Follow-up.
- A2-L4 (Low, nit) world.rs:1473-1478 vs :1794-1796, :999-1009: phase 3 rewrites `records[slot]` by position
  (assumes write_records puts the soldiers first, a comment only), while predict finds a slot's records by scan: two
  ways to locate a soldier's record, the positional one silent if write_records' order changes. Fix: a debug_assert
  (records[slot] is Soldier with index slot) in present_volumes. Follow-up.
- A2-L5 (Low, nit) proto tests/common/mod.rs `section_kinds` re-implements snapshot.rs:173-179's predicate as the
  refusal test's oracle (test-only duplicate of the rule); the GROUPS unit test (snapshot.rs:726-772) is the
  catalogue-facing check, so the pair still pins it. Note only.
- Fixer's open points: (A) no seam objection, advisor's tuning call; grenade_fuse_ms doc's ":1478" cites a derived
  number (32 / 21.12), wording. (B) accept for the synthetic recording; see A2-L3. (C) 8 is right; "(9 fields)" a
  count slip.
- Seam: freeze HELD (step 4); boundary HELD (step 5); deps HELD.
- Verdict: APPROVE WITH FINDINGS (0 Critical, 0 High, 0 Medium, 5 Low). No round 3 from this lane.
DONE (2026-11-11)
