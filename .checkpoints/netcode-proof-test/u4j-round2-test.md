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
- Step 1 (17:3x UTC): 66 paths hashed (git status --untracked-files=all; u4j-round2-test/hashes-start.sha256). The
  fixer's final list (u4j-build.md "Final", first 8 hex): eval.rs 09a17b8a, figures.rs e9282de6, marks.rs 294b9007,
  practice.rs e074d58d, engine.rs d3b437b8, fixtures.rs bb4fb3a5, fixtures/README.md 86836e17, marks.json baa69105,
  README.md 2e837f6f, SERIES.md fa03e6dc: all 10 match. judge lib.rs ef87e5ee and log/src/lib.rs 51cb181d equal the
  pre-fix copy (.gate/u4j-prefix), as the fixer says ("crates/log unchanged by this pass"). Cargo.toml, Cargo.lock,
  check.mjs: no diff against the base. The fixer lists no fixture hashes ("fixtures in the report"); diff -rq against
  .gate/u4j-prefix shows the 39 fixture files the packet names. Worktree HEAD 411eeeed (the base).
- Reading (no run): F1's paths to Ok in find (practice.rs:555-575): IN TOLERANCE needs s <= tolerance; BEYOND s <=
  tolerance; FAR BETTER s < band_low <= 1 and f = 1; then f > tolerance refused. The line itself is marks.json's
  (practice.rs:498-506) and parse_bound refuses band_low > 1 or band_high < 1 (:103-110) and a tolerance above 1.10.
  So 4 layers: the pin, the band rule, FAR BETTER f = 1 (equivalent to max(1, s) once band_low <= 1), the guard
  (unreachable after the others, by design). The committed probe-A test asserts only the prefix "ws-practice: ", so
  on this head it passes by the pin alone; each other layer needs its own plant (below).
- Reading: Ratio::parse caps 30 digits (ratio.rs:76-78); with <= 6 after the point and the band compared only with 1,
  no comparison in parse_bound or find can overflow i128 now (s.num <= 9.2e18 times den <= 1e6; bound x f <= 8.5e37).
