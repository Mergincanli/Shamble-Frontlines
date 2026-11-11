# U4j review, round 1: test-engineer (is it proven?)

Reviewer: test-engineer (opus). Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50,
base 411eeeed50d8c71e7e1f74366f0e1d5c27388444. Started 2026-11-11. No local memory; no earlier notes file.
(The Edit tool is not available in this session: this file is rewritten whole with Write after each step.)

## Plan
1. Record sha256 of every changed and new file (git status --untracked-files=all; git diff --stat) before anything.
2. Read the plan §5 U4j (u4-build-plan.md:643-663 now), §4.6, §4.12, the advisor's conditions 2, 4, nits c, f, Q20,
   Q22, W1-N2; the two reviewers' notes (netcode, security).
3. Read the diff: log/src/lib.rs, judge marks.rs, eval.rs, figures.rs, practice.rs, tests/engine.rs, tests/fixtures.rs,
   fixtures, marks.json, README, SERIES.
4. Map each §5 U4j test/fixture to an existing test; judge power; claim · proof · gap table.
5. Probes and plants (in place, one at a time, restore, sha256 check): settle the other reviewers' findings by run;
   plant where a test's power is in doubt. Probes live in a TEMPORARY test file (tests/zz_review_probe*.rs),
   removed after (the Edit tool is missing, so no in-place edit of engine.rs).
6. Check builder's claims: P1-P4, counts (+18), A19 pub items.
7. check.mjs once at the end (idle wait, touch loop, provenance check, counts 304/327/48).
8. Final sha256 identity check. Verdict.

## Findings (a line after each)
- Step 1 done: 34 paths hashed (u4j-review-test/hashes-start.sha256); marks.json 97c5cab5, README 71e4c637, SERIES
  e95febab, eval.rs 4549e885, figures.rs c034fc8f, practice.rs 1e59bdea: all equal the builder's. Note: the plan file
  u4-build-plan.md is now f54125ec (builder read 0d69332c); U4j's section moved to :643-663 (W1 plan edits, 10:28).
- Step 2-3 read: plan §5 U4j, §2 C4/-18/hs rows, §4.6, §4.12; advisor-ws-docs-r2 :373-400 (R3-R10: R6 "f = max(1, s),
  used only when s is at most the tolerance"; R7 "Valid only with 480 heal records"; R8 (v) "FAR BETTER: at the
  bound"; R9 "a counted WebSocket run that fails stands as FAIL"); netcode and security notes; the whole diff.
- Test map (§5 U4j): six decoded-hash cases = fixtures pm-0001-17/{base, differ, lost-group, refused-group, orphan,
  none-compared} (tests/fixtures.rs pm_0001_17_x2): fixture bytes checked, each differs from base by exactly the one
  line it claims (sentinel 0000000000000000, null+snap_incomplete 1, tick 65539, all null). C4 four cases =
  fixtures pm-0001-4/{3} + pm-0001-21/twin-bites-build, and engine c4_the_build_matches_the_bite. --check-marks and the
  drift test = check.mjs marks step (to run). Per figure: decoded (PASS-part/FAIL/gap x5/INVALID/NOT-RUN arm),
  twin (PASS/FAIL/empty/NOT-RUN/gap x7), machine_named (PASS/FAIL x8 + C pass), cross_arch (engine only: report 1).
  -18 ws-practice: PASS, FAR BETTER f=1 FAIL, second practice INVALID/PASS, 20 INVALID cases, BEYOND+chance PASS.
- R1 (reading) the ws-practice tests never put s at the boundaries the ruling lists (s = 1.00, 1.10, 1.1001), never
  give IN TOLERANCE with s < band_low, and never exceed Z' alone: plant T below decides whether that code is proven.
- R2 (reading) the PM-0001-17 fixtures write 2-slot hs lists under header clients 48: README §1 says "one entry per
  client slot of the run"; the judge never ties the list length to `clients`, so a writer that logs 2 of 48 slots
  passes (the base fixture itself is the proof). Joins security F8 (no coverage floor).
- Probe run 1 (u4j-review-test/probe-run1.txt; 11 probes, all pass = today's behaviour confirmed; provenance: Compiling
  npt-log and npt-judge from this copy's path): A (FAR BETTER band 2.0, s 1.9: PASS at X' 1.273 s, f 1.9 > 1.10),
  B (29-decimal band: judge panics, ratio.rs:27), C1 (client log carries its own replay_hash: PASS), C2 (aarch64
  replay: PASS), C3 (arm_rule false still gets the Arm rule: PASS), D (two forged measured runs naming each other:
  PASS), E (FAIL -> INVALID when a later practice run joins the set), F (second practice run in another set unseen:
  PASS), G (479 heals: PASS), H (record's M_m 620 ms turns FAIL into PASS), T1a/T1b/T2/T3 (the code is right at s =
  1.10, s = 1.1001, below band_low, over Z' only).
- F1 HIGH practice.rs:389, :406 (+ parse_bound :87-90; README §4 "f = max(1, s)" for every class): FAR BETTER never
  checks s <= tolerance and parse_bound never checks band_low <= 1 <= band_high, so f = s up to any value (probe A: f
  1.9, X' 1.273 s, a 1.2 s median PASSES). Beyond Mustafa's WS-BAR cap (NETEM-BAR (a)) and R6/R8 (v) ("FAR BETTER: at
  the bound"). Fix: FAR BETTER f = 1; band_low <= 1 <= band_high; refuse f > tolerance. Proof: probe A committed,
  expected FAIL at 670 ms. Before merge. (= netcode F1, security F1.)
- F2 MEDIUM practice.rs:259-286, :325: the latest practice run of the build over ALL candidates is taken, then
  refused if it started after the counted run, so a practice run added later turns a FAIL into INVALID (probe E1,
  E2). R9: a failed counted run stands as FAIL. Fix: candidates only among practice runs started before the run.
  Proof: probe E committed. Before merge (wrong verdict). (= netcode F2.)
- F3 MEDIUM practice.rs:260-263: candidates come from the counted run's set only, so "one practice run per build"
  (R7) misses a second practice run in another set (probe F: PASS, no second_ruling asked). Fix: pass every run to
  find() and count the build's practice runs over all sets. Proof: probe F committed, expected INVALID. Before merge
  (wrong verdict). (= security F3.)
- F4 MEDIUM figures.rs:195-209: twin_hash_mismatch_count accepts a twin of any role (probe D: two forged measured runs
  naming each other PASS -21's "0 effect beyond the valid part"). Fix: role twin (the -30 form at U6). Proof: probe D
  committed, expected INVALID/gap. Before merge. (= netcode F4, security F5.)
- F5 MEDIUM figures.rs:215-219, :271-282: each hs name is read from every log, and the Arm rule from the client log's
  own arch: a client log carrying replay_hash excuses its own mismatch (C1), an aarch64 replay excuses (C2), a mark
  with arm_rule false gets the rule (C3). Fix: server_hash from process room only, replay_hash from process replay on
  x86_64 only (else gap), decoded_hash from client processes only; the Arm rule on L2 (or the mark's arm_rule). Proof:
  probes C1-C3 committed, expected FAIL or gap. Before merge for C1/C2 (the input rule U4r writes to); C3 Low today
  (-58 is C-only). (= security F4, netcode F6/F8.)
- F6 MEDIUM practice.rs:331-338, :383: M_m and the band come from the record's own copy of the bound line; only X, Y,
  Z, k and the tolerance are pinned to marks.json (probe H: a record saying M_m 620 ms turns a FAIL into a PASS, f
  1.0968 for a true 1.0398). Fix: marks.json -18 pins the bound line (or M_m, band and its sha256) and the rule
  requires the record's `bound` to equal it. Proof: probe H committed. Before merge (a marks.json rule and the record
  format later units build on; a wrong verdict). (= security F2, netcode F9.)
- F7 MEDIUM practice.rs:372-376: the practice run's heal count is never checked against the bound's
  client_bursts_count (parsed, unused) though R7 says "Valid only with 480 heal records" (probe G: 479 PASS). Fix:
  heal.len() == client_bursts_count. Proof: probe G committed, expected INVALID. Before merge (cheap; wrong verdict).
  (= netcode F7.)
- F8 MEDIUM practice.rs:56-68, :87-90 with ratio.rs:25-28, :135-141: two long decimals in the bound line panic the
  judge (probe B) instead of INVALID. Fix: README §2.5 and parse_bound cap each fraction at 6 decimals (or the
  comparisons return Err). Proof: probe B as a unit test expecting Err. Before merge (nit f fixes this format now).
  (= security F6.)
