# test-engineer: BRAWL-1 round 1 review (PR #40, spike branch claude/netcode-proof-test-plan-hcky5g)

Snapshot under review: wip/brawl1.patch sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (verified by
sha256sum; the read copy's `git diff HEAD | sha256sum` gives the same value).
Packet: brawl1-packet.md sha256 16eab462c4e0fcbcd8360aad7a951207275927c7f0f6813b794df54c12f4b1a4 (verified).
Read copy: .claude/worktrees/brawl1 (352a245 + uncommitted change). Logs of my runs: .gate/u4r/brawl1-review-test/.

## Plan
1. Read packet rulings (conditions 1-18, N2-1, TE-R2-1/2, RR4-L1/L2, EA-U4R-1/2, RR3-L2). DONE
2. Read the diff per file. DONE
3. Check each test against conditions 2-5, 12, 13: can it fail? negative cases? exact texts? DONE (by reading)
4. Read the build logs (p1-*, p2-*, repin*, final-*). DONE for p1 and p2, final-regen-tests
5. Plants (cond 6 / Q5): p1-plant-leave.log, p1-plant-roster.log read. DONE
6. Run the new tests at the snapshot through u4r-run.sh (shared mode): bots lib (unit + short), client brawl1, lab fp1/fp3/
   r10/rr3, full test in release. NEXT
7. Findings, then StructuredOutput.

Process note (honest): one scratch file was written by shell redirect to the session scratchpad (d-after.txt, a copy of
p1-cd.log lines 38-150) before I re-read my write limits; nothing else written outside this notes file. No read-copy change.

## Verified (no finding)
- Proof (a) p1-a-check2.log: all 11 steps, 532/610, 0 failed. (b) p1-b-det.log: uptime first, PASS, old det pins.
- (c) p1-cd.log: SS-P0-s4 regenerated 6933b56a "as the manifest's". (d) my own diff of p1-cd.log:38-150 against
  results/u4r-precheck-30/faf42aeb034c/runs.txt:17-125: only line 1 (build: commit and binary sha256) differs.
- (e) my own normalised diff, p1-before-a/b vs p1-after-e/e2: only the wallclock bite's L1 hash (varies between A and B
  at base) and "52/58 filtered out" -> 53/59. Disclosed by the builder as PROOF-E for the advisor. Holds as evidence.
- Plants: leave plant -> both cases fail with exactly the two ruled names; roster plant -> both cases fail with the new
  check alone. Panic line 2245 vs 2246 is consistent with a deleted line in scenario.rs vs room.rs.
- fp3: all ruled texts exact (TE-R2-2 1-3, cond 12 full digits, RR4-L2 ruled case, bursts 1001/1000, every-s 0 2/1,
  after-max 143165507/143165506). r10_fp3: 36,001 refused with the exact ruled text, before the runner check.
- Re-pin: only sha256 values change in the manifests; final-regen-tests reproduces 1f00d2d5.

## Findings (one line each, as found)
- T1 (Medium) sim-only test never shown to bite: no run with the leash removed; EveryTick in 120 s likely cannot fail
  (P0 open-loop onset took 458 s in U4r's s1). bots/src/lib.rs:471-477 (short), :481-495 (full).
- T2 (Medium) area != 0 predicate and the lab figure have no positive test anywhere (grep: no test shows a Soldier
  record with area != 0; logs.rs:584-599 untested; summary_line's figure untested). The 10-min runs' "building total 0"
  bar and the re-run sets' bar rest on an unexercised predicate.
- T3 (Low) steering unit test's "blocked" asserts cannot fail: slot 5 seed 1 has phase 7, so the walk at commands 4-7
  is MOVES[8] = [-127, 0], equal to the steer at (1450, 0). bots/src/lib.rs:200-207 (patch lines). Also None/dead
  "keeps the leash's state" untested; R_OUT boundary (1400/1401) untested.
- T4 (Low) client test: the Filed::Hash filter and the wrap-rule reduce are untested (103/104 never complete; ring slots
  100-106 are in increasing order mod 32, so `.last()` passes). endpoint.rs brawl1 test.
- T5 (Medium) B1 pattern (iii): 1,832 cm in all three seeds, at three different angles on one radius, each at a burst's
  end: past the 1,800 cm door-entry radius; cause left unchecked by the builder. Under condition 1's floor (R_OUT >=
  1,400) the B1 margin is negative. Needs a direct door-gap figure in U4g's brief.
- T6 (Low) SOLDIER_FLAGS/SOLDIER_DEAD copied in client lib.rs and in the bots test (r.grid[5] & 1<<3); no test ties
  them to npt-sim's FLAG_DEAD (world.rs:120) or SOLDIER_FLAGS (world.rs:92).
- T7 (Low) provenance: runner logs HEAD 352a245 only; no hash of the uncommitted diff per log, so "one code state" (Part
  1) and "at the head that goes to review" (cond 13) are not checkable from the logs.
