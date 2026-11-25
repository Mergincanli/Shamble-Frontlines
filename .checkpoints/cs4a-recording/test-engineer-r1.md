# test-engineer, review round 1 of cs4a "The recording" (head 51efa35, source as at eb14739)

## Plan
1. Read the card, plan section 4 and section 9 row 4a, netcode-architect block 1.6 (marks, tests, planted defects).
2. Read the diff c1da1bd...HEAD -- crates packages: every test 4a adds or changes.
3. Map each mark (plan lines 470, 473; ADR 0001 replay line; PM-0004-17/-18/-19/-21; word-size vector;
   "one planted divergence caught at its tick with the differing field") to a test, a place it runs, and whether it can fail.
4. Read toolchain.yml 140-200: what runs where in CI.
5. Read plants-4a.py / plants-4a.txt: is each catch the right test for the right reason.
6. Think of defects the plants miss; plant a few in a copy (.gate/te-4a, own target dir) if cheap.
7. Flakiness: two_processes.rs, the 100,000-input thresholds, timing readings, order dependence.
8. Rank gaps against the 370-line room; write the verdict.

## Findings (one line each, as found)
