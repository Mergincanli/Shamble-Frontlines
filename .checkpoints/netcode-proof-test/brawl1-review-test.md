# test-engineer: BRAWL-1 round 1 review (PR #40, spike branch claude/netcode-proof-test-plan-hcky5g)

Snapshot under review: wip/brawl1.patch sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (verified by
sha256sum; the read copy's `git diff HEAD | sha256sum` gives the same value).
Packet: brawl1-packet.md sha256 16eab462c4e0fcbcd8360aad7a951207275927c7f0f6813b794df54c12f4b1a4 (verified).
Read copy: .claude/worktrees/brawl1 (352a245 + uncommitted change). Logs of my runs: .gate/u4r/brawl1-review-test/.

## Plan (all steps done)
1. Read packet rulings (conditions 1-18, N2-1, TE-R2-1/2, RR4-L1/L2, EA-U4R-1/2, RR3-L2).
2. Read the diff per file.
3. Check each test against conditions 2-5, 12, 13: can it fail? negative cases? exact texts?
4. Read the build logs (p1-*, p2-*, repin*, final-*).
5. Plants (cond 6 / Q5): p1-plant-leave.log, p1-plant-roster.log.
6. Run the new tests at the snapshot through u4r-run.sh (shared mode).
7. Findings, then the result.

Process note (honest): one scratch file was written by shell redirect to the session scratchpad (d-after.txt, a copy of
p1-cd.log lines 38-150) before I re-read my write limits; nothing else written outside this notes file and my two run
logs. No read-copy change (the runner's own touch loop aside).

## Verified (no finding)
- Proof (a) p1-a-check2.log: all 11 steps, 532/610, 0 failed. (b) p1-b-det.log: uptime first, PASS, old det pins.
- (c) p1-cd.log: SS-P0-s4 regenerated 6933b56a "as the manifest's". (d) my own diff of p1-cd.log:38-150 against
  results/u4r-precheck-30/faf42aeb034c/runs.txt:17-125: only line 1 (build: commit and binary sha256) differs.
- (e) my own normalised diff, p1-before-a/b vs p1-after-e/e2: only the wallclock bite's L1 hash (varies between A and B
  at base) and "52/58 filtered out" -> 53/59. Disclosed by the builder as PROOF-E for the advisor. Holds as evidence.
- Plants: leave plant -> both cases fail with exactly the two ruled names; roster plant -> both cases fail with the new
  check alone. Panic line 2245 vs 2246 is consistent with a deleted line in scenario.rs vs room.rs. Restored: room.leave
  present at scenario.rs:533, room.rs not in the diff.
- fp3: all ruled texts exact (TE-R2-2 1-3, cond 12 full digits, RR4-L2 ruled case, bursts 1001/1000, every-s 0 2/1,
  after-max 143165507/143165506). r10_fp3: 36,001 refused with the exact ruled text, before the runner check.
- Re-pin: only sha256 values change in the manifests; final-det reproduces both det pins (non-cargo 214.5 s, uptime
  first); final-regen-tests reproduces 1f00d2d5.
- Conditions 2, 3 (i)-(v), 4, 13 met in form; short copy 10.6-11.1 s per build.

## Findings
- T1 (Medium) sim-only test never shown to bite: no run with the leash removed; EveryTick in 120 s likely cannot fail
  (P0 open-loop onset took 458 s in U4r's s1). bots/src/lib.rs:633-640 (short), :644-659 (full), :495 sim_run.
- T2 (Medium) area != 0 predicate and the lab figure have no positive test anywhere (grep: no test shows a Soldier
  record with area != 0; logs.rs:584-599 untested; summary_line's figure scenario.rs:1627 untested). The 10-min runs'
  "building total 0" bar and the re-run sets' bar rest on an unexercised predicate.
- T3 (Low) steering unit test's "blocked" asserts cannot fail: slot 5 seed 1 has phase 7, so the walk at commands 4-7
  is MOVES[8] = [-127, 0], equal to the steer at (1450, 0). bots/src/lib.rs:365-371. Also None/dead "keeps the leash's
  state" untested (:387-391); R_OUT boundary (1400/1401) untested.
- T4 (Low) client test: the Filed::Hash filter and the wrap-rule reduce are untested (103/104 never complete; ring slots
  100-106 are in increasing order mod 32, so `.last()` passes). client/tests/endpoint.rs:397-441; client/src/lib.rs:350-369.
- T5 (Low) B1 pattern (iii): 1,832 cm in all three seeds, at three different angles on one radius, each at a burst's
  end: past the 1,800 cm door-entry radius; a limit set by something other than the leash. Kinematic bound under a 1 s
  burst about 2,000 cm. Pattern (iii)'s "0 in a building" is not evidence of B1 safety; U4g's brief needs a door-gap figure.
- T6 (Low) SOLDIER_FLAGS/SOLDIER_DEAD copied in client lib.rs:247-248 and in the bots test (:478); no test ties them to
  npt-sim's FLAG_DEAD (world.rs:120) or SOLDIER_FLAGS (world.rs:92).
- T7 (Low) provenance: runner logs HEAD 352a245 only; no hash of the uncommitted diff per log.
- T8 (Low) step()'s glue (bots lib.rs:223 passes endpoint.own_soldier()) has no test; only the pin change shows it.

## My runs at the snapshot (shared mode, not counted)
- .gate/u4r/brawl1-review-test/t1-tests.log, exit 0: bots lib 2 passed 1 ignored (unit + short copy, prints identical
  to final-regen-tests.log); client brawl1 1 passed; lab fp1/fp3/r10 x3/rr3/p6/fp6 8 passed.
- .gate/u4r/brawl1-review-test/t2-full-release.log, exit 0: full test 1 passed in 24.23 s; all nine prints identical to
  final-regen-tests.log (deterministic; cond 13 reproduced at this snapshot).
- T3 computed: slot 5 seed 1 phase 7 (splitmix64 as Behaviour::new); walk at commands 4-7 = [-127, 0]. The three blocked
  asserts are at commands 4, 5, 6. toward_centre(1450, 200) = [-125, -17] (in no MOVES entry) for the fix.
- T5 kinematics: SS move_speed 600 cm/s (scene.rs:184), fire held so no sprint: 20 cm per tick; a 1 s burst re-runs 30
  ticks = 600 cm, so from R_OUT 1,400 cm up to about 2,000 cm, past the door entries (buildings 0 and 3: 1,800 cm;
  buildings 1, 2, 4, 5: about 1,864 cm, greybox.rs:92-99 with ROOM 400).

## Verdict: APPROVE WITH FINDINGS (no Critical or High). Result returned by StructuredOutput.
