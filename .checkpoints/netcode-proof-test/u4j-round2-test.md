# U4j round 2: test-engineer (is the fix pass proven?)

Copy under review: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50 (uncommitted fix pass).
Pre-fix copy (round 1): /home/user/Shamble-Frontlines/.gate/u4j-prefix.
Runner: bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4j-fix/run.sh <cmd...> (exit code + the test runner's own summary line decide; never a grep).
The Edit tool is not available in this session: this file is rewritten whole with Write after each step. Plants: ONE
whole-file Write of practice.rs holding every plant behind an env switch (a plant is live only when its name is in
ZZ_PLANT; with no variable the code is the original), one run per plant set, then one whole-file restore Write,
sha256-checked. The planted file was diffed against the original reconstructed from the branch's wip/u4j.patch
(its practice.rs hunk rebuilds to e074d58d, the start hash): only the plant lines differ.

## Plan
1. Read the packet (diffs, plan's U4j section, rulings FX-1..FX-14, "How round 2 proves F1 closed", round-1 findings, bars), my round-1 notes (u4j-review-test.md), and the builder's "Fix pass, round 1" list (claims only).
2. Hash: sha256 of every changed and new file in the copy (git status vs base 411eeee); compare with the fixer's final list. Save to u4j-round2-test/hashes-start.sha256.
3. F1: re-run round-1 probe A (FAR BETTER cap) and the advisor's three plants on this head; each must be caught by a named test (runner exit code + summary line).
4. Sample FX-2, FX-3, FX-5, FX-6, FX-8, FX-10 (+ others as time allows): the named test exists, asserts the ruling, and fails on pre-fix code (re-plant where in doubt; restore; sha256-check).
5. Close or not-close each of my round-1 findings F1..F16 with proof.
6. Final counted check.mjs run once; counts as deltas on 286 / 309 / 48, against the fixer's.
7. Byte-identity re-check against step 2's hashes.
8. Verdict.

## Log
- Step 1 (17:1x UTC): 66 paths hashed (git status --untracked-files=all; u4j-round2-test/hashes-start.sha256). The
  fixer's final list (u4j-build.md "Final", first 8 hex): eval.rs 09a17b8a, figures.rs e9282de6, marks.rs 294b9007,
  practice.rs e074d58d, engine.rs d3b437b8, fixtures.rs bb4fb3a5, fixtures/README.md 86836e17, marks.json baa69105,
  README.md 2e837f6f, SERIES.md fa03e6dc: all 10 match. judge lib.rs ef87e5ee and log/src/lib.rs 51cb181d equal the
  pre-fix copy (.gate/u4j-prefix), as the fixer says ("crates/log unchanged by this pass"). Cargo.toml, Cargo.lock,
  check.mjs: no diff against the base. The fixer lists no fixture hashes ("fixtures in the report"); diff -rq against
  .gate/u4j-prefix shows the 39 fixture files the packet names. Worktree HEAD 411eeeed (the base).
- Process note: Step 1 and the two Reading lines were first appended with a python one-off by mistake (the rule is
  the Write and Edit tools only); from then on this file is rewritten whole with Write. Content unchanged.
- Reading (no run): F1's paths to Ok in find (practice.rs:575-606): IN TOLERANCE needs s <= tolerance; BEYOND s <=
  tolerance; FAR BETTER (:578) s < band_low <= 1 and f = 1 (:597); then f > tolerance refused (:602). The line itself
  is marks.json's (:494-506) and parse_bound refuses band_low > 1 or band_high < 1 (:105-111) and a tolerance > 1.10.
  So 4 layers: the pin, the band rule, FAR BETTER f = 1 (equivalent to max(1, s) once band_low <= 1), the guard
  (unreachable after the others, by design). The committed probe-A test asserts only the prefix "ws-practice: ", so
  on this head it passes by the pin alone; each other layer needs its own plant (below).
- Reading: Ratio::parse caps 30 digits (ratio.rs:76-78); with <= 6 after the point and the band compared only with 1,
  no comparison in parse_bound or find can overflow i128 now (s.num <= 9.2e18 times den <= 1e6; bound x f <= 8.5e37).
- Reading: the engine Arm test (engine.rs:1434-1490) and the fixtures give no aarch64 client that is NOT LG with a
  mismatch against a server hash: l1-aarch64-claim's only difference is an orphan (tick 9), which FX-5 (c) counts on
  every machine. So FX-5 (b)'s role key may be unpinned: plant ROLE below decides.
- Reading: FX-10 (f)'s order "count before a coverage gap" is pinned by the engine warm-up case (engine.rs, the
  decoded gaps test's last case: slot 0 resolves tick 1 only, a (d) gap, and slot 1's mismatch: expected FAIL);
  "empty before a gap" by none-compared (all null, a (d) gap, expected FAIL empty); "count before waiting" by
  lg-orphan-no-replay. "Gap before waiting" has no case (INVALID vs NOT-RUN, both non-pass: Low, not planted).
- Run 0, head + a TEMPORARY probe file tests/zz_r2_probe.rs (Write; removed after the plant runs), ..-17:23:27
  UTC via run.sh (idle, touch, 2 s loop, shared target, offline): suite `cargo test --locked --no-fail-fast -q -p
  npt-judge -p npt-log` rc=0 (summary lines seen: engine 43 passed, fixtures 19, log lib 11, docs 0/0; judge lib 14,
  main 0 and cli 2 were above my tail's cut, shown in runs 1-4). Probe binary rc=0, 4 passed:
  - PROBE A (round 1's exact form: the record's own line with band 2.0 to 2.0, FAR BETTER, chance "x", M_p 1242.6
    ms, s 1.9, moved 1.273 s / 4.75 s / 13.11 s, counted median 1.2 s): part INVALID, "practice run ws-practice's
    record: its bound line is not PM-0001-18's ws_bound_line, byte for byte" (the pin). Round 1: PASS at X' 1.273 s.
  - PROBE A-pinned (the same with marks.json's own line): INVALID, "record is \"FAR BETTER\" at s = 1.9 (band 0.965
    to 1.035, tolerance 1.1): its numbers are not in force" (class rule).
  - F1 GRID: 200 combinations (class IN TOLERANCE / BEYOND / FAR BETTER / INVALID / other x 10 values of s from 0.5
    to 1.9, incl. 0.965, 1.00, 1.10, 1.1001 x chance ruling or not x record moved at max(1, s) or at 1), each judged
    with the real marks.json: 30 give a valid run; the largest X in force is 737000000 (= 1.10 x 670 ms, IN TOLERANCE
    or BEYOND at s = 1.10); every FAR BETTER valid row is at 670 ms (f = 1). No path moves X past Mustafa's 10 %.
  - ROLE: an aarch64 client of role C, ML or MAC with a mismatch against a server hash and an x86-64 replay agreeing:
    FAIL ("0 mismatches = 1"), aarch64-only 0; role LG: PASS, aarch64-only 1. The head's code keys on LG, as FX-5 (b).
  - BITE2 (the fixer's stricter reading, FX-3 (c)): two measured runs of one build with two practice runs in force
    (the second with a second_ruling): the ws-budget-x2 bite is INVALID, "have practice runs ws-practice and
    ws-practice-2 in force". Matches the fixer's note; no committed test has this case (Low, below).
- Planted practice.rs written (sha256 355ccf90...; diff vs the original = only the plant lines). Every plant run below:
  run.sh (idle 0 s waited, df 11 GB free, touch + 2 s loop), the no-run printing "Compiling npt-log" and "Compiling
  npt-judge" from this copy's path, then `ZZ_PLANT=<set> cargo test --locked --no-fail-fast -q -p npt-judge`; rc and
  the summary lines are the runner's own.
- Run 1, ZZ_PLANT=T (the advisor's plant T: IN TOLERANCE band_low dropped and cap 1.12, BEYOND cap 1.12, the guard at
  1.12, the Z' recheck dropped), 17:26:47-17:26:55, rc=101: lib 14 ok, cli 2 ok, fixtures 19 ok; engine "FAILED. 42
  passed; 1 failed": u4j_ws_the_tolerance_and_band_boundaries at engine.rs:2426 with "T1b: Pass []", "T2: Pass []",
  "T3: Pass []" (each refusal by name). My grid probe also failed (X in force 738 ms). CAUGHT by FX-9's test, alone.
- Run 2, ZZ_PLANT=BAND (parse_bound's band check removed), 17:27:04-17:27:09, rc=101: lib "FAILED. 11 passed; 3
  failed": practice::tests::the_bound_line_refuses_a_band_that_leaves_out_1 (practice.rs:750 "band 2.0 to 2.0 must be
  refused"), marks::tests::ws_bound_line_agrees_with_its_checks (marks.rs:1145 "a band that is not 0 < low <= 1 <=
  high"), practice::tests::the_bound_line_reads_as_readme_writes_it ("a band upside down"); engine 43 ok, fixtures 19
  ok, probe 4 ok (probe A stays INVALID by the pin). CAUGHT by FX-1's unit test and FX-6's marks test, as the advisor
  says.
- Run 3, ZZ_PLANT=BAND,PIN,FBMAX (the advisor's third plant: FAR BETTER f = max(1, s), band check and pin removed;
  the guard kept), 17:27:18-17:27:22, rc=101: lib the same 3 fail; engine "FAILED. 41 passed; 2 failed":
  u4j_ws_the_record_carries_the_marks_bound_line (engine.rs:2583 "H1: [] left: Pass right: Invalid") and
  u4j_ws_practice_refuses_every_number_not_set_by_its_rules (engine.rs:2266, the byte-for-byte cases). BUT
  u4j_ws_far_better_past_the_cap_is_invalid PASSED, and probe A stayed INVALID: FX-1 (c)'s guard (f 1.9 > 1.10)
  refuses it. So plant 3 is caught by name, by FX-6's and FX-1's other tests; the advisor's "caught by
  u4j_ws_far_better_past_the_cap_is_invalid" does not hold as written while the guard stands (the guard is live).
- Run 4, ZZ_PLANT=BAND,PIN,FBMAX,GUARD (plant 3 with the guard removed too: the pre-fix F1 logic), 17:27:32-17:27:36,
  rc=101: engine "FAILED. 40 passed; 3 failed": u4j_ws_far_better_past_the_cap_is_invalid (engine.rs:2314 "left:
  Pass right: Invalid"), plus the two pin tests; lib the same 3; my probe A: PASS at "<= 1273000000 (X 670000000 x f
  1.9 ...)", round 1's result. So the committed probe-A test bites exactly when all four layers are gone; each layer
  alone is pinned by its own test (pin: H1/commit + 4 refuses cases; band: 3 lib tests; class rule: probe A-pinned and
  T2; guard: unreachable by design). F1: CLOSED (see the verdict).
