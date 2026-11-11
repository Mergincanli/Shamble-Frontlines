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
