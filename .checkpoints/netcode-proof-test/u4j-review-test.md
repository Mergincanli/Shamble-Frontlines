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
- (resume 2026-11-11, after a usage-limit cut: start hashes re-checked, 34/34 match; no probe file left in the copy.
  Process note: the F9 line was first appended with a shell heredoc by mistake; from here this file is rewritten whole
  with Write, F9 kept word for word.)
- F9 MEDIUM tests/engine.rs (the -18 ws-practice tests): plant T (u4j-review-test/plant-t.txt; practice.rs :387-388
  cap 1.10 -> 1.12 and band_low dropped, :436 Z' recheck dropped) SURVIVES all 65 committed judge tests; the probes
  T1b (s = 1.1001), T2 (s below band_low) and T3 (over Z' only) each bite it. Mustafa's 10 % cap (WS-BAR) is not
  pinned by any committed test. Fix: commit T1a/T1b/T2/T3. Before merge (the bar's own number; a loosened cap passes).
- Step 6 (builder's claims): counts +18 = log 2 (hs_holds_one_hash_or_null_per_slot, hs_refuses_a_bad_entry_...) +
  judge lib 3 (ws_bound_checks_keep_their_shape, the_bound_line_reads_as_readme_writes_it, moved_numbers_round_up_...)
  + engine 8 (c4, decoded gaps, arm rule, empty, twin hash, machine_named, ws puts, ws refuses) + fixtures 5 (pm_0001_17
  x2, pm_0001_4_c4, pm_0001_21, pm_0006_11): matches its notes :113. Public items added (diff grep): npt-log
  TickRecord.hs, set_hashes, hashes; npt-judge marks::BoundNumber, Check.ws_bound, Rule::WsPractice (variant),
  eval::Outcome::Empty (variant) + Outcome::fails, figures::Lack::Empty (variant); practice.rs is `mod practice`
  (private, pub(crate) items). The builder's notes file lists none by crate (A19 asks the report to): engine-architect's
  check, from this list. P1-P4: the named tests exist and each asserts the behaviour the plant breaks (read); not re-run.
- Probe run 2 (u4j-review-test/probe-run2.txt; 2 probes pass = today's behaviour; provenance from this copy's path;
  probe file removed, 34/34 hashes match): S (one client hash of six, null elsewhere, snap_incomplete 0: run valid,
  value 0, compared 1) and O (aarch64 orphan at a tick the server never logged, replay without that tick: valid,
  value 0, "aarch64-only" 1, on an L1 part).
- F10 MEDIUM figures.rs:298-333 with marks.json -17 (compared and incomplete only reported): no coverage floor and nulls
  never reconciled with tick.s.snap_incomplete: probe S passes "100 % of decoded snapshots rebuild" on 1 of 6 hashes
  while the client's own count says 0 incomplete. A runner that samples (netcode F5 (i)) or a client that writes null
  for a refused group passes. Fix (the advisor's call, beyond the plan's words): a check per run that compared +
  incomplete = the server's (slot, tick) pairs, or at least a null on a tick whose snap_incomplete entry is 0 is a gap.
  Proof: probe S committed as a fixture (pm-0001-17/sparse) expecting INVALID or FAIL. Before merge? Advisor's ruling
  (a marks.json rule later units build on). (= security F8, netcode F5 (i), R2.)
- F11 MEDIUM figures.rs:327: `at(rep, t, slot) == theirs` treats a replay with no entry at an orphan tick as agreeing
  with a server that has none, so an aarch64 orphan is excused (probe O) where the x86-64 one FAILs (fixture orphan).
  The Arm rule excuses a hash the x86-64 re-decode reproduces as the server's; an orphan has no server hash to
  reproduce. Fix: an orphan always counts (or needs a replay entry equal to Some(server)); fold into F5. Proof: probe O
  committed, expected FAIL. Before merge with F5. (= security F4 (iii).)
- F10 addendum: README:50-52 itself defines `null` as "a tick with a group lost and none refused (counted in
  tick.s.snap_incomplete)", so the reconciliation check enforces the README's own writer rule, not a new one.
- F9 addendum: advisor-ws-docs-r2.md:378 (the ruling on the outcome record) asks for "A unit test with a fixture for
  each class and the boundaries s = 1.00, 1.10 and 1.1001": the classes are there (built in code), the boundaries are
  not (engine.rs's nearest is s = 1.131498, :2017, which is why a 1.12 cap survives).
- F12 MEDIUM fixtures/ (none for the Arm rule): §5 U4j builds cross_arch_mismatch_count "with fixtures only since no U4
  run feeds them"; the builder's own plan (u4j-build.md:46) says "pm-0001-17 (six decoded-hash cases + two Arm-rule
  cases)", but fixtures/pm-0001-17 holds the six only; the Arm rule is proven by engine.rs:1401-1448 alone, never
  through the real marks.json's L2 parts (select machine.role LG) and the folder reader. Fix: two fixtures (LG aarch64
  client + RS room + x86-64 replay agreeing: part PASS, "aarch64-only" 1; replay wrong: FAIL), plus C1/C2/O as
  fixtures with F5's fix. Before merge with F5 (the same files prove it).
- F13 LOW engine.rs:1661-1670 (machine_named): the FAIL list names 8 of the 14 conditions; role, name, plan, arch,
  mem_mb and threads_per_core are never emptied (by reading: explicit list; no plant run), and PM-0001-1's five parts
  (RS, LG, ML, RD, MAC selects, which §5 U4j names) have no fixture (only PM-0006-11's RS part). Fix: six more list
  entries; one -1 fixture with an ML/MAC log (no address needed). Follow-up.
- F14 LOW figures.rs:197 twin_hash_mismatch_count's "profile" and "bite" field checks: no test anywhere gives a twin
  another profile or bite (grep "another profile|another bite": none); dropping either survives. Fix: two cases in
  u4j_the_twin_hash_... . Follow-up (with F4's role check).
- F15 LOW practice.rs:269-279: a bite run whose set holds practice runs of two builds ("cannot tell whose numbers are
  in force") is never tested. Fix: one case in u4j_ws_practice_refuses_... . Follow-up.
- F16 LOW practice.rs:316 (scene/profile only): the practice run is never compared with the counted run on RS's
  machine.kernel, params.config_hz or params.ws_tcp_model, the "header checks" the ruling (advisor-ws-docs-r2.md:378)
  lists in the record; README §2.5 leaves them "for people". A practice run on another RS kernel sets f for this one
  (within the cap). Fix: compare the three (gap if absent). Advisor's call; follow-up.
- Step 7 (u4j-review-test/check-final.txt): check.mjs 11:17:31-11:19:57 UTC, exit 0, all 10 steps pass; idle before
  and after; touch + 2 s loop; Compiling npt-log and npt-judge from this copy's path; measured 304 = 286 + 18, bites
  327 = 309 + 18, self 48 of 48; marks 81 read and checked, ADR rows unchanged (drift test); redact 10/10, 182 items, 0
  findings; deps: npt-judge ships npt-log and serde only.
- Step 8: after the run, 34 paths (same set), all 34 sha256 equal the start (marks.json 97c5cab5, README 71e4c637,
  SERIES e95febab); Cargo.toml, Cargo.lock, check.mjs: no diff against the base. The copy is byte-identical.
- Others' findings not settleable by run, weighed: netcode F3 (whose slot index) and F5 (ii)-(iv) (writer rules:
  room logs server_hash in every phase; "two different hashes"; never drop a late hash) are README text U4r writes
  to: agree, before merge. Security F9 / netcode F14 (describe() prints neither class, rulings, band nor tolerance):
  agree; Medium, follow-up unless the advisor reads R8 (v) as the judge's report. Security F7, F10, F11, F12: agree,
  follow-ups.

## Claim · proof · gap
| Claim | Proof | Gap |
|---|---|---|
| hs: 16 lowercase hex or null; anything else refused | log tests hs_holds_one_hash_or_null_per_slot, hs_refuses_a_bad_entry_or_name_... | none |
| decoded hash, §4.6's six cases | fixtures pm-0001-17 x6 (2 tests); P1 bites | no coverage floor (F10); source per name (F5) |
| decoded gaps (name missing, slot counts, two logs, server 0) | engine u4j_the_decoded_hash_gaps_and_its_every_phase | within-name uneven lengths untested (Low, in F13's spirit) |
| every phase read | engine warm-up FAIL | none |
| Arm rule | engine arm-rule test (PASS, FAIL, NOT-RUN, x86 FAIL) | no fixture (F12); log's own arch, replay from any log, orphan excused (F5, F11) |
| Empty fails, shows no bite | engine empty test; P4 bites | none |
| C4, four cases | engine c4 + fixtures pm-0001-4 x3, pm-0001-21/twin-bites-build; P2 bites | bite direction "at least one log" (security F7, Low) |
| twin hash | engine twin test + fixtures pm-0001-21 x5 | twin role (F4); profile/bite untested (F14) |
| machine_named | engine 8 FAIL + C PASS; fixtures pm-0006-11 x2 | 6 of 14 conditions, -1's parts (F13) |
| ws-practice numbers in force | engine puts test; P3 bites | FAR BETTER f uncapped (F1); cap and band unpinned (F9) |
| ws-practice refuses what its rules do not set | engine 20 INVALID cases | later practice (F2), other set (F3), record's M_m (F6), heal count (F7), panic (F8), two-build bite (F15), header checks (F16) |
| marks.json compiles; drift | check.mjs marks: 81, rows unchanged | none |
| +18 tests, both builds | check.mjs 304 / 327 / 48 | none |

## Verdict
REVISE (round 1). Before merge: F1 (High); F2, F3, F4, F5 (with F11, F12), F6, F7, F8, F9; F10 needs the advisor's
ruling on the form (it is the README's own rule). Follow-ups: F13-F16, security F7/F10/F11/F12, netcode F6 (if not
folded into F5), F8-F14. DONE.
