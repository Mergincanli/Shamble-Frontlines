# U4f2 review round 1: test seat (test-engineer)

## Plan
1. Read the spec: u4f2-brief.md rev 1.2 (done), the advisor's brief check (conditions, Q1-Q5), u4f-brief.md 2.10, 5.1-5.4.
2. Read the builder's record u4f2-build.md (done) and the patch; inventory the 60 new tests by file and name.
3. Map 2.10's 51 + 9 extras + 22 negatives + Q4 + C-U4F2-10 to the tests; check each test can fail (asserts real,
   not vacuous), and each plant's log fails the named tests.
4. Weigh plant 9 (F-E), plant 1's history, f16's shared pair vs 2.10's "one forger, 15 honest", 702/799 counts,
   F-C's margin, the test-step budget.
5. Check c-u4f-r2.mjs's rows (p1, p7, p14, p17, p19 expect) and c-u4f-runs.mjs set 21 against 5.1-5.4.
6. F-B: would -21's set as built fail; what stays unproven until -21's set.
7. Findings TE-U4F2-n, verdict.

## Notes (one line per finding as I go)
