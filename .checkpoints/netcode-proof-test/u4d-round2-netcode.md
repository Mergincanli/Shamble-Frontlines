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
  8.4 min per slot, replay.rs:522-530). Also ES "all 15 vehicles in view" (ADR 0001:1475-1477) vs ES inheriting SS's
  11 vehicles at 52-82 m (world.rs:974-978 via scene.rs `..Scene::stress()`). Own-state size vs R2's 16 B own-state
  assumption (r2-netcode.md:1756): ~10-14 B delta; fine.
