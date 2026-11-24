# U6c's review round: the test seat (test-engineer, xhigh). Read only.

Inputs checked by sha256 (2026-11-24): brief 8f56f37a…, advisor check 9f5ab8d0…, launch c bf8016a8…, snapshot patch
a4e63455…, builder notes 426b31da…: all five equal the launch's. W = .claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof.

## 1. Plan
1. Tie the counted run to the snapshot: end-check2.log's tree hash and counts against the patch (base 807/904).
2. Count the new tests per block against section 3 and C7 (+52 measured, +52 bites); check none is ignored or
   cfg-gated out of the bites build, and that each test's name matches what it asserts.
3. Read each plant's log (P1-P21, p14b): it ran the named test, failed at the claimed line, and the restore hashes.
4. Read each named test against its plant: can it fail; does it fail for the claimed reason; timing dependence.
5. The fake directory (issue_tests.rs): what Block 6's nine plus C1 (a) cover; the shipped path's reach.
6. F-JOIN-ONCE (launch item 4): the glue's Open-only step, Closed counted, step at once; the tests' no-Join-deadline
   assert; whether a plant of each would fail.
7. The five items: F-P8, P2, P14/P18/P4-P5, F-REDACT, F-SIZE-1/F-FMT-1.
8. Runs only where a claim needs one, through the runner.

## 2. Notes (one line per finding, as found)
