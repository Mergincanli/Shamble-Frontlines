# U4j round 2: test-engineer (is the fix pass proven?)

Copy under review: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50 (uncommitted fix pass).
Pre-fix copy (round 1): /home/user/Shamble-Frontlines/.gate/u4j-prefix.
Runner: bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4j-fix/run.sh <cmd...> (exit code + the test runner's own summary line decide; never a grep).

## Plan
1. Read the packet (diffs, plan's U4j section, rulings FX-1..FX-14, "How round 2 proves F1 closed", round-1 findings, bars), my round-1 notes (u4j-review-test.md), and the builder's "Fix pass, round 1" list (claims only).
2. Hash: sha256 of every changed and new file in the copy (git status vs base 411eeee); compare with the fixer's final list. Save to u4j-round2-test/hashes-start.sha256.
3. F1: re-run round-1 probe A (FAR BETTER cap) and the advisor's three plants on this head; each must be caught by a named test (runner exit code + summary line).
4. Sample FX-2, FX-3, FX-5, FX-6, FX-8, FX-10 (+ others as time allows): the named test exists, asserts the ruling, and fails on pre-fix code (re-plant by Edit where in doubt; restore; sha256-check).
5. Close or not-close each of my round-1 findings F1..F16 with proof.
6. Final counted check.mjs run once; counts as deltas on 286 / 309 / 48, against the fixer's.
7. Byte-identity re-check against step 2's hashes.
8. Verdict.

## Log
