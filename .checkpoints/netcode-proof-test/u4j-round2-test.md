# U4j round 2: test-engineer (is the fix pass proven?)

Copy under review: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50 (uncommitted fix pass).
Pre-fix copy (round 1): /home/user/Shamble-Frontlines/.gate/u4j-prefix.
Runner: bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4j-fix/run.sh <cmd...> (exit code + the test runner's own summary line decide; never a grep).
The Edit tool is not available in this session: this file is rewritten whole with Write after each step. Plants: ONE
whole-file Write of the planted file holding every plant behind an env switch (a plant is live only when its name is
in ZZ_PLANT; with no variable the code is the original), one run per plant set, then one whole-file restore Write,
sha256-checked. Each planted file was diffed against the original rebuilt from the branch's wip/u4j.patch (it rebuilds
practice.rs to e074d58d and figures.rs to e9282de6, the start hashes): only the plant lines differ.

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
  alone is pinned by its own test (pin: H1/commit + 4 refuses cases; band: 3 lib tests; IN TOLERANCE's band_low and
  cap: T1b, T2; guard: unreachable by design). F1: CLOSED (see the verdict).
- Run 5, ZZ_PLANT=START (FX-2: practice runs started after the counted run are candidates again), 17:28:27-17:28:31,
  rc=101: engine "FAILED. 41 passed; 2 failed": u4j_ws_a_practice_run_after_the_counted_run_changes_nothing
  (engine.rs:2488 "E2: [... 2 practice runs of one build ...] left: Invalid right: Fail") and the refuses test
  (engine.rs:2260, "run ws-1: no practice run of its build started before it: [] left: Pass"). CAUGHT. FX-2 proven.
- Run 6, ZZ_PLANT=SET (FX-3 (a): candidates from the counted run's set only, the pre-fix scope), 17:28:31-17:28:36,
  rc=101: engine "42 passed; 1 failed": u4j_ws_one_practice_run_per_build_over_every_set (engine.rs:2525 "listed true:
  [] left: Pass right: Invalid", probe F). CAUGHT.
- Run 7, ZZ_PLANT=SERVES (FX-3 (b): candidates only practice runs listing the mark, the pre-fix filter),
  17:28:36-17:28:41, rc=101: the same test, "listed false: [] left: Pass right: Invalid". CAUGHT. FX-3 (a), (b) proven;
  (c)'s cases (2), (3) read against the pre-fix bite code (packet :965-975: any-build candidates in the set, kinds > 1
  refused): case (3) is INVALID there, so it fails on the pre-fix code, as the advisor says.
- Run 8, ZZ_PLANT=PIN (FX-6: the bound taken from the record's own line, no byte compare), 17:28:41-17:28:47, rc=101:
  engine "41 passed; 2 failed": u4j_ws_the_record_carries_the_marks_bound_line ("H1: [] left: Pass", probe H) and the
  refuses test (engine.rs:2266, the byte-for-byte cases). CAUGHT. FX-6 proven (its marks-load half by run 2's lib test
  and the fixer's record of ws_bound_line_agrees_with_its_checks failing before the check).
- practice.rs restored by Write: sha256 e074d58d (= start).
- FX-8 by reading, no plant: the_bound_line_refuses_more_than_6_decimals' four 7-digit cases (band_low 0.9650001,
  band_high 1.0350001, tolerance 1.1000000, guard 0.0000000) each pass every other parse_bound rule (low <= 1 <= high,
  1 <= tolerance <= 1.10, guard <= 0.01), so only the 6-digit cap refuses them: the test fails without the cap, as the
  fixer recorded ("practice.rs:653 band_low 0.9650001"). Its probe-B case is now refused by FX-1's band rule first
  (band_high 0.999... < 1), so probe B no longer reaches an overflow; with the band compared only with 1 and the cap,
  no overflow path is left (reading above). FX-8 proven.
- Planted figures.rs written (sha256 534ebed5...; diff vs the rebuilt original = only the plant lines).
- Run 9, ZZ_PLANT=ROLE (FX-5 (b): the Arm rule's LG key dropped, aarch64 alone), 17:32:35-17:32:42, rc=101 ONLY from
  my probe: lib 14 ok, cli 2 ok, engine "43 passed; 0 failed", fixtures "19 passed; 0 failed"; zz_probe_role_key
  FAILED: C, ML and MAC aarch64 clients with a mismatch now PASS ("aarch64 only=1"). The plant SURVIVES the whole
  committed suite: FX-5 (b)'s role conjunct is not pinned by any committed test (finding TE2-1 below).
- Run 10, ZZ_PLANT=ORDER (FX-10 (f): a coverage gap returned before the count and the empty comparison),
  17:32:42-17:32:47, rc=101: engine "41 passed; 2 failed": u4j_the_decoded_hash_gaps_and_its_every_phase
  (engine.rs:1429, the warm-up case: "left: Invalid right: Fail") and u4j_an_empty_comparison_fails_the_run_and_shows_
  no_bite (engine.rs:1505, "Invalid ... Fail"). CAUGHT: the order is pinned.
- Run 11, ZZ_PLANT=COVC (FX-10 (c) removed), 17:32:47-17:32:52, rc=101: fixtures "18 passed; 1 failed":
  pm_0001_17_every_server_hash_is_resolved_on_the_client_side (fixtures.rs:425, "sampled: L1 P2 is NotRun, wanted
  Invalid with \"slot 0, tick 2: ...\""), the other four cases still right. CAUGHT. FX-10 (a), (b), (d) by the same
  test's other cases (each expects its own message; the fixer recorded all five failing on the unfixed code).
- figures.rs restored by Write: sha256 e9282de6 (= start). Probe file removed (rm by absolute path). git status: 66
  paths, all 66 sha256 equal hashes-start.sha256 (17:3x). No probe or plant left in the copy.
- Fixer's check.mjs run 2 (u4j-build.md :322-325): "bites 299 passed, 1 failed in `-p npt-net --lib` (not this lane;
  the failing test's name is not in check.mjs's summary)", then npt-net's lib passed 3 runs alone and two check.mjs
  runs passed: a test that failed once was retried into green and called "a timing flake" with no name and no cause.
  Not U4j's code (no npt-net file touched), but a defect to root-cause (finding TE2-5).
- Step 6, check.mjs counted, once (run.sh: idle 0 s waited, df 11 GB free, touch + 2 s loop; the pre-run `cargo test
  --locked --no-run -p npt-judge -p npt-log` printed "Compiling npt-log v0.0.0 (/home/user/Shamble-Frontlines/.claude/
  worktrees/agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/log)" and "Compiling npt-judge v0.0.0 (.../crates/
  judge)"), 17:36:26-17:40:37, rc=0, "check: all 10 steps pass": fmt 1.1 s; clippy 33.2 s; clippy-wasm32 9.3 s; doc
  36.6 s; test 165.1 s, measured 317 passed 0 failed 0 skips, bites 340 passed 0 failed 0 skips; deps 2.5 s held; refs
  0.3 s (597 files); marks 0.7 s (81 marks read and checked, ADR rows unchanged); self 48 of 48; redact 10 of 10, 213
  items, 0 findings. Deltas: measured 317 = 286 + 31, bites 340 = 309 + 31, self 48 = 48 + 0; equal to the fixer's
  runs 3 and 4 (317 / 340 / 48). The +13 over round 1's 304 / 327: judge lib +3 (14), engine +6 (43), fixtures +4
  (19). The npt-net failure of the fixer's run 2 did not recur in this one run (one run proves nothing about it).
- Step 7: 17:40:44, git status 66 paths, all 66 sha256 equal the start; HEAD 411eeeed; no Cargo.toml, Cargo.lock or
  check.mjs diff. The copy is byte-identical to step 1.
- Settled by run for the other seats (a TEMPORARY tests/zz_r2_probe2.rs, Write, run 17:41:31-17:41:32 via run.sh with
  provenance, rc=0, then removed; 17:41:41: 66 of 66 hashes equal again): netcode N2 CONFIRMED: E0 (counted median 700
  ms over X' 697 ms) is FAIL; adding a practice run of the same build in another set, listing no mark, with no
  params.started_utc turns it INVALID ("run ws-practice-x has no params.started_utc"). Fails safe (never a pass); Low,
  as netcode rates it. Netcode N3 and security N1 (the advisor's plant 3 stays INVALID through the guard) CONFIRMED by
  runs 3 and 4 above.
- Reading: SERIES.md:294's twin_hash_mismatch_count row still says the twin may have "any role (-21's twins are role
  twin, -30's each other's measured runs)", while figures.rs:198-203 now refuses any role but twin (FX-4). The fixer
  flagged it (u4j-build.md, FX-4: "NOT EDITED ... flagged for the advisor"). Finding TE2-6 (Low).

## My round-1 findings: closed or not
| Id (sev.) | Routed | Status | Proof |
|---|---|---|---|
| F1 (High) | FX-1 | CLOSED | probe A INVALID on this head (pin), A-pinned INVALID (class rule); grid: 200 paths, largest X in force 737 ms; plant T caught by u4j_ws_the_tolerance_and_band_boundaries (T1b, T2, T3 by name); band plant caught by 3 lib tests; plant 3 caught by the pin tests, and u4j_ws_far_better_past_the_cap_is_invalid bites once the guard goes too (run 4); fixer's record: both FX-1 tests failed on the unfixed code |
| F2 | FX-2 | CLOSED | E0/E1/E2 test; plant START caught (E2 + refuses test) |
| F3 | FX-3 (a), (b) | CLOSED | probe F committed; plants SET and SERVES caught by name |
| F4 | FX-4 | CLOSED | fixture twin-is-measured INVALID with "has role measured, not twin"; fixer's record: NotRun before the fix |
| F5 | FX-5 (a); C3 to FU-5 | CLOSED (C1, C2); C3 deferred by the advisor | lg-replay-in-client-log, lg-replay-on-aarch64 INVALID by the source rule (fixer's record: Pass before) |
| F6 | FX-6 | CLOSED | probe H committed (H1, commit case); plant PIN caught; marks-load agreement by ws_bound_line_agrees_with_its_checks |
| F7 | FX-7 | CLOSED | refuses test's 479 and 481 cases (M_p unchanged, so only the count refuses); fixer's record: Pass before |
| F8 | FX-8 | CLOSED | 6-digit cap test, by reading cap-only refusals; no overflow path left (bound analysis) |
| F9 | FX-9 | CLOSED | plant T caught by name (run 1); s = 1.00 and 1.10 positive cases |
| F10 | FX-10 | CLOSED | 5 fixtures INVALID by their own messages; plants COVC and ORDER caught; fixer's record: all 5 valid before |
| F11 | FX-5 (c) | CLOSED | lg-orphan and l1-aarch64-claim FAIL (fixer's record: Pass and NotRun before) |
| F12 | FX-5 fixtures | CLOSED | 8 Arm-rule fixtures on the real marks.json's L2 P2 and L1 P2 parts, through the folder reader too |
| F13 (Low) | FX-14 | CLOSED | 14 machine_named conditions; fixture pm-0001-1/laptop PASS; fixer's role plant caught |
| F14 (Low) | FX-4 | CLOSED | "another profile", "another bite" cases; fixer's plants caught |
| F15 (Low) | FX-3 (c) | CLOSED | bite cases (2) to (4); case (3) INVALID on the pre-fix code by reading |
| F16 (Low) | FX-13 | CLOSED | 7 setup cases in the refuses test (kernel, config_hz, model, model not the bound's, each absent) |
| R1, R2 | FX-9, FX-10 (a) | CLOSED | boundaries test; short-lists fixture and 48-entry lists |

## New findings (round 2)
- TE2-1 MEDIUM (not before merge; cheap; rides any push, else a STATUS follow-up for U6-c): FX-5 (b)'s LG conjunct is
  unpinned. figures.rs:248 `h.machine.role == "LG" &&` dropped (plant ROLE) survives every committed judge test (lib
  14, cli 2, engine 43, fixtures 19); on this head an aarch64 client of role C, ML or MAC with a mismatch and an
  agreeing x86-64 replay FAILs, and under the plant it PASSes. The test pm_0001_17_the_arm_rule_holds_on_lg_only and
  fixtures/README.md's l1-aarch64-claim row ("the Arm rule is LG's only, and an orphan counts everywhere") claim the
  LG key, but that fixture differs only by an orphan, which FX-5 (c) counts everywhere. Reach today: -17's L2 parts
  select LG, so only a non-LG aarch64 client log (an L1 client on an Arm machine; ML's browser at U6-c, -19 judged "as
  it stands") would meet it. Fix: one case in engine.rs:1434-1490 (role C or ML, aarch64, mismatch at a server hash,
  replay agreeing: FAIL), and the README row's words. Cheapest proof: plant ROLE then fails that case.
- TE2-2 LOW (follow-up; = netcode N3, security N1, settled by runs 3-4): u4j_ws_far_better_past_the_cap_is_invalid
  (engine.rs:2289-2317) asserts only the prefix "ws-practice: "; on this head it passes by the pin's refusal, and the
  advisor's plant 3 leaves it passing (the guard holds); it fails only with all four layers gone. Not a hole: each
  layer is pinned by its own test except the guard, unreachable by design. Optional: assert the pin's reason.
- TE2-3 LOW (follow-up; a reading for the advisor): the fixer's stricter FX-3 (c) reading, practice.rs:458-470 (a bite
  whose set's measured runs have two different practice runs in force is INVALID), has no committed test; probe BITE2
  shows it on this head. One case in u4j_ws_a_bite_run_takes_its_counted_runs_practice_run.
- TE2-4 LOW (follow-up): FX-10 (f)'s "a coverage gap before a waiting Arm mismatch" (figures.rs:553-558) has no case;
  swapped it gives NOT-RUN for INVALID, both non-pass.
- TE2-5 MEDIUM (outside U4j; not before U4j's merge; owner the main session with infra-engineer): the fixer's check.mjs
  run 2 had 1 failed test in `-p npt-net --lib` (bites build), retried into green and called a timing flake with no
  name and no cause (u4j-build.md:322-325); check.mjs's test summary does not name a failing test, so the first
  failure left no name. Root-cause it (a flake is a defect, never retried into green); have check.mjs print the
  failing test names. Not reproduced in my one counted run.
- TE2-6 LOW (wording; rides any push, else follow-up with U6-a): SERIES.md:294 says the twin of
  twin_hash_mismatch_count may have "any role (... -30's each other's measured runs)"; the judge now refuses any role
  but twin (figures.rs:198-203, FX-4), so the formula row a writer follows contradicts the code until U6-a adds -30's
  form. Fix: "role twin (-30's form, each other's measured runs, comes with U6)".

## Claim · proof · gap
| Claim (the fix pass) | Proof | Gap |
|---|---|---|
| FX-1: no path moves X, Y, Z past 1.10 | probe A, grid (737 ms max), plants T, BAND, 3, 3+GUARD | guard unreachable (by design); TE2-2 |
| FX-2: a later practice run changes nothing | E0-E2 test; plant START | NC N2: a practice run with no start still flips FAIL to INVALID (Low, fails safe) |
| FX-3: one per build over every run; bite at its counted run's numbers | over-every-set test; bite test (1)-(4); plants SET, SERVES | two-in-force bite reading untested (TE2-3) |
| FX-4: twin role twin | twin-is-measured fixture; profile/bite cases | SERIES row still says any role (TE2-6) |
| FX-5: sources, LG + room x86_64, orphan everywhere, missing replay entry a gap | 8 fixtures, engine room case | LG conjunct unpinned (TE2-1) |
| FX-6: bound line pinned, checked at load | H1 + commit; marks test; plants PIN, BAND | none |
| FX-7: exactly 480 heals | 479, 481 cases | none |
| FX-8: at most 6 decimals, no panic | 6-digit test; reading | probe B now refused by the band rule first |
| FX-9: cap and band boundaries | boundaries test; plant T | none |
| FX-10: coverage rule and order | 5 fixtures; plants COVC, ORDER | gap-before-waiting (TE2-4) |
| FX-11: outcome printed whole | puts test (IN TOLERANCE, BEYOND + ruling) | none |
| FX-12: README writer rules; pairing by slot | permuted-slots | writer timing words (netcode N1, carry) |
| FX-13: same setup | 7 cases | none |
| FX-14: machine_named whole; -1 laptop | 14 conditions; laptop fixture | none |
| +13 tests per build, all green | check.mjs 317 / 340 / 48, 10 of 10 | none |

## Verdict
APPROVE WITH FINDINGS. F1 (the High) CLOSED by run: probe A INVALID, a 200-path grid at most 737 ms, and each of the
advisor's plants caught by name (plant 3 through the guard and the pin's tests; its named test bites once the guard
goes too). F2-F16, R1, R2 CLOSED (C3 deferred to FU-5 by the advisor). No new Critical or High: no round 3 from this
seat. New: TE2-1 Medium (LG key unpinned; cheap, not before merge), TE2-5 Medium (npt-net flake retried into green;
outside U4j), TE2-2, TE2-3, TE2-4, TE2-6 Low. DONE (17:4x UTC).
