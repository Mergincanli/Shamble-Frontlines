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
