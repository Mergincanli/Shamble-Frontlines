# U4d round 2 + W1-S1 (npt-sim fix pass + npt-proto half): netcode-architect (read only)

Copy: /home/user/Shamble-Frontlines/.gate/u4d-s1 (detached 21a8597, uncommitted). Round 1 state: .gate/u4d-prefix.
Rulings: advisor-u4-wave1.md W1-S1 (:45-111), W1-U4D (:113-304), W1-PUB (:353-436).

## Plan
1. Read fixer's record (u4d-build.md "Fix pass, round 1") and s1-proto.md.
2. diff -ru u4d-prefix sim vs u4d-s1 sim; git diff 21a8597 -- spikes/netcode-proof/crates/proto.
3. Check H1 (parts relative to hull; one compose function; offsets fit; tests), N-M1 (magazine), N-M3 (FLAG_FIRED),
   N-M4 (in the fight), N-M5 (effect end tick), N-M2 code half (Refusal/RefusalKind/refusals per command, cleared each
   step, outside hash and dump; D-10 asserts).
4. W1-S1: owner-only kind fields + ranges, one per soldier, SOLDIER = catalogue remote row; proto GROUPS own-state rows,
   section rule, enforcement in encoder+decoder, tests (3 checks), -57 bite.
5. Fixer's points A, B, C; downstream U4e/U4g/U4h impact (own-state record size/churn).
6. Findings with file:line, verdict.

## Progress
- [1] Read u4d-build.md "Fix pass, round 1" (:135-374) and s1-proto.md whole. Sim diff saved (scratchpad simfix.diff,
  3,103 lines); read src diff (history, lib, replay, scene, seam, world). Open checks: part damage at health 0 (churn
  over an S30?), cluster centre = origin?, FLAG_DEAD vs alive, throw authority-only, wait's range, phase-1 records.
  (My slip: line [1] was first appended by a shell heredoc to this absolute path, not a file tool; notes only. The
  Edit tool is not available to me here, so this file is rewritten whole by Write after each block.)
- [2] Read sim tests diff (D-3 catch-up, h1 x2, n_m5, a_m1, r_m1, t_m1, w1_s1, D-5/D-10 refusals, carry 9/14, D-6
  in-fight, D-8 bound) and world.rs run_movers/run_command/throw/refuse. Checked: part health wraps to 200 (never
  dies: churn steady over an S30, world.rs:1579); cluster centre = origin (homes 10 m out, scene.rs:136); FLAG_DEAD
  set iff dead (world.rs:1612, revive :1142); throw and refuse authority-only (:1381-1418); wait in [1-U, cost]
  (:1371, :1401); phase 1 writes records then load_movers_into (:1169-1171); refusals cleared per step (:2080).
- [3] Proto diff read (snapshot.rs Section/carries, put/get_delta, put/get_record, both GROUPS rows, the 3-check
  test; wire.rs w1_s1 28/14; bites -57 own_state; common section_kinds). Sound. Update codes changed fields at full
  width (snapshot.rs:289-312). Plan text §4.2/§4.10 already carries W1-U4D's words (u4-build-plan.md:369).
- [4] (B) weighed: marks' default window is `measured` (marks/README.md:100); PM-0001-12 judges "ES at P2 for 10
  minutes" (ADR 0001:1600) with pre:PM-0001-2; the fixer's 50 s windows (13.4 ... 8.7 at 1 in 10) put a measured
  10-min window after warm-up near 8.8-9.3/s at 1 in 11 < 9.5. Generator (replay.rs:402-432) reckons scene speed only,
  no crouch, no dropped command, no scripted speed, no death; est reset only by its own teleports/spawns (~ every
  8.4 min per slot, replay.rs:522-530). Also ES "all 15 vehicles in view" (ADR 0001:1475-1477) vs ES's vehicles: ES
  sets large_vehicles 0 (scene.rs:183), so all 15 loop at 40 + 3v m = 40-82 m (world.rs:760-764). Own-state size vs
  R2's 16 B own-state assumption (r2-netcode.md:1756): ~10-14 B delta; fine.

## Round-1 findings (mine): CLOSED / NOT CLOSED
- H1 CLOSED: PART = offset x, y, z, yaw relative to hull + health + parent (seam.rs PART doc; world.rs write_records
  part block, Val::F(cm(offset)), yaw 0); one composition part_center (world.rs:446) used by compose_volumes (:483,
  phases 3 and 8, hit_volumes_from :882, load_movers) and load_movers_into (:2031); offsets asserted in World::new;
  tests h1 x2 (tests/world.rs:~760, :800): 2,850 part records changed = 2,850 parts hit over 600 SS ticks (4.75 a tick,
  was 123.5); Q6 plant fails both.
- N-M1 CLOSED: SS magazine 32,000 (scene.rs:63-70), const assert both scenes (:167-170), D-6 lines; Q25 fails build.
- N-M3 CLOSED: FLAG_FIRED gone, flags dense (world.rs:116-120); D-3 catch-up every 9th step, >40 such ticks; Q1 fails.
- N-M4 CLOSED for SS (the ruling's scope): large hulls 5-14 m round the centre, others 52-82 m, stand-ins ±12 m
  (world.rs:760-764, spawn_stand_in); D-6 asserts; Q4, Q5. U4r carry present (u4s-carries.md:193). ES: see N2-M2.
- N-M5 CLOSED: effect record = kind, end tick (TICK), magnitude; n_m5 test; 7.74 of 384 a tick; Q7.
- N-M2 code half CLOSED: Refusal/RefusalKind (seam.rs:498-520), InputReports::refusals (:531), refuse() authority
  only, cleared each step (world.rs:2080), write_state excludes reports; D-5/D-10 assert records; Q3, Q13, Q14, Q28,
  Q29. Module doc words = §4.2's replacement text.
- My round-1 Lows: ES events source folded into T-M1 (closed for its 300 s window; see N2-M1); 1 MB = 10^6 (T-M2)
  CLOSED; clamp on non-firing commands, PARENT redundant, held-back Rewind leaving the ring, non-finite ray (carry 45):
  NOT CLOSED, named follow-ups (u4d-build.md:367-372); carry 13 counting half: to U5 (C13).

## W1-S1
- Sim half MET: OwnState last in ALL (seam.rs:204, :297), OWN_STATE 8 fields (:252) with ranges holding every
  reachable value (wait in [1-U, interval x TICK_HZ]; reload <= RELOAD_MAX; attributes brought into range on entry;
  scene starts asserted; record() debug_assert; w1_s1 test :1810); one record per slot, generation the soldier's
  (world.rs:1767); SOLDIER = 8 fields = catalogue :137 remote row (seam.rs:249); reloading stays a flag; FIELDS 12.
- Proto half MET: OwnState in both own-state rows (snapshot.rs:492, :648); carries() (:175) = owner rows for Own,
  other rows for Entities; encoder OutOfRange before writing, decoder Malformed at the kind (:222-:245, :266-:340);
  3-check test (:726: 92-bit row, 129 bits on the wire, P7 shows 48 bits); w1_s1 wire test 28 encodes + 14 decodes
  refused (tests/wire.rs:188); -57 sends OwnState (tests/bites.rs:104); P1-P8.
- Together: OwnState and Effect can no longer travel in the entity section or inside a remote soldier record. Which
  slot's own-state records go to client c is the room's (U4e) selection, which the codec cannot see: N2-L1.

## New findings
- N2-M1 MEDIUM (fixer's point B): ES's event rate falls over a run; T-M1 (tests/world.rs:1005-1014) measures 300 s
  from tick 0, not the judged window (PM-0001-12's 10-min ES runs, measured after warm-up). Measured 10-min mean likely
  ~8.5-9.3 < 9.5: PM-0001-2 fails on ES, PM-0001-12's ES part INVALID. Same drift may spread SS's cluster over an S30
  (everyone-in-view, the precondition of -17/-18/-20/-21): unmeasured. Not before merge; fix before U4d2 pins the
  recordings; report wording now.
- N2-M2 MEDIUM: ES's 15 vehicles loop 40-82 m out (scene.rs:183 large_vehicles 0 -> world.rs:760-764 else branch)
  against ES's "all 15 vehicles in every client's set on >= 99 %" (ADR 0001:1475-1477; SERIES.md:314). Pre-existing
  (round 1: 40 + 3v m for all), missed in my round 1, untouched by the fix. Not before merge; before U4d2 pins ES.
- N2-L1 LOW: owner routing is the room's; U4e carry with a test (own section names only c's slot; effects index/8).
- N2-L2 LOW: ES sky aim (replay.rs:458-466) keeps pitch (and a standing soldier's yaw) still on most ticks vs R2's
  "yaw 16, pitch half the time" (r2-netcode.md:1751): ES's downstream slightly lighter than the estimate it stands for.
- N2-L3 LOW: downstream notes: U4g reset lacks respawn_left (world.rs:274; predict ignores it: :1289); U4h must blend
  composed boxes or interpolate hull yaw wrap-aware (:1185); U4e: own-state record 160 bits, changes every tick in SS.
- N2-L4 LOW: text: "(9 fields)" slip (8 is right); Weapon, Scene::weapon, Scene::health into §4.1/W1-PUB's list.
- (A): grenade_fuse_ms 1,515 = ADR 0001:1478's ES fuse applied to grenades (SS/CO/TC 2,000): not new tuning in
  substance. aim_one_in = 11: new stand-in tuning; acceptable as a knob, value to be re-calibrated with N2-M1.

## Verdict
APPROVE WITH FINDINGS: no Critical or High; every round-1 finding of mine the fix claims is closed; W1-S1 both halves
met; two Mediums and four Lows as follow-ups with carries (no round 3).
DONE
