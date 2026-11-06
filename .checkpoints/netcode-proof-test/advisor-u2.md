# Advisor: U2 unit-close review (netcode proof test)

Scope: plan rev 3.2 §12.1 U2 (npt-l1 + unit/property tests; PM-0001-3's L1 half and its bites).
Diff: c104302..6900e52 -- spikes/ (commits 08782eb 921174d 2529675 81c6d01 6900e52).
Not a merge review. Rulings U2-R1..R8, then the unit verdict.

## Plan (my steps)
1. Read plan §4.7, §4.3 E15, §6.1, §6.4, §12.1; ADR 0001 part 13 + PM-0001-3 row; marks.json PM-0001-3.
2. Read the diff: link, l1, l1 tests, lab (main, probe, machine), judge figures + u2 tests, run script, check.mjs, SERIES.md.
3. Read the committed verdict.json and runs.txt; spot-check numbers.
4. Run check.mjs myself (if toolchain present) and reproduce key numbers.
5. Rule R1..R8; unit verdict.

## Notes
- (start) head checked; notes file created.
- Read: plan §4.3 (E15), §4.4, §4.7, §6.1-6.4, §12.1; ADR 0001 part 13 (568-586) and rows -3, -18, -20, -58;
  marks.json -3, -20, -58; the checkpoint (steps 25-29).
- Read: link/src/lib.rs, l1/src/lib.rs, l1/tests/l1.rs, lab/src/{main,probe}.rs, judge figures.rs diff, engine.rs
  u2_ tests, ops/runs/c-u2-pm-0001-3.mjs, results runs.txt + verdict.json.
- verdict.json: 5 L1 parts PASS (P1-P4 25 checks/run, B1 27); L2 parts, MP, not-net-of-base NOT-RUN. Bites:
  rtt-each-way fails mean/p001/p999/p90 high + RTT high (9 checks); loss-per-round-trip fails loss low both ways
  (1.01 %); normal-jitter fails p90 low (54.2 ms) only; clock-seed fails the twin check only. Matches the claims.
- runs.txt: means sit +0.04-0.09 % above the profile (P2 +38-45 us): the reorder step's holds at the probe rate
  (held delay = max(own, succ+0.1 ms), E = d + a/3, x1 % = +a/300: P2 +33 us, P4 +67 us). Explained; inside 5 %.
- runs.txt: same seed gives identical loss counts at P3 and P4 (same ppm, profile not in the key): common random
  numbers across profiles; each profile's part stands alone. Not a defect.
- -58: reorder validity only on its P3 part; -20 at P3 only: no mark names reordering at P0 or P1, so P0/P1 share 0
  blocks nothing.
- REPRODUCED: check.mjs all 9 PASS at a56d4be (97 measured / 102 bites tests; 18 s).
- REPRODUCED: clippy on the measured build (no features), host and wasm32 leaf crates: clean too.
- REPRODUCED: my own release build of npt-lab (binary sha differs from the run's: 8714e453 vs 05a9d4df, same code)
  gives the committed fate-log hashes at P2, B1, P4 seed 1 (913db8e9, feabf2f0, b8a04f2f) and identical summaries.
- REPRODUCED: my python over my P2 / B1 logs gives the judge's values exactly (P2 up loss 0.020191, mean
  50038109.38, p001 40020299, p90 58029785, p999 59981998; RTT mean 100074285.6 over 610746). Max delay 60.099 ms
  (the hold's +0.1 ms at the probe rate).
- R7 finding: check.mjs says "features only add code; none is compiled out by one" - false: probe.rs:231 and :463
  are cfg(not(feature = "bites")), so clippy --all-features never lints them. Today clean either way.
- R6 finding: the judge has no rule that a measured run comes from a build without bites (header build.features
  empty, bite null); plan §4.7 "measured builds never contain them" rests on the run script alone. U2's logs are
  right (measured: features [], bites: ["bites"]).
