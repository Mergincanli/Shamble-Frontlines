# U4j build notes (the judge and marks change)

Builder: general-purpose agent (opus, xhigh), isolated worktree
/home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50, base 411eeeed50d8 (fast-forwarded from
44446a57). Brief: u4j-brief.md "Prompt" plus the caller's four additions (W1-N2 marks change, shared-target rules,
base counts 286 / 309 / self 48, save as you go). Files: crates/judge, crates/log, marks/ only. Commit nothing.

## Sources read
u4-build-plan.md (sha256 0d69332c… checked) §0-§10 (U4j §5 :626-645, §4.6, §4.12, §2 rows, §4.14, Q11 Q20 Q22);
advisor-u4.md (verdict conditions 2, 4, nits c and f; Rulings; Ruling U4s; Ruling F1/F2); advisor-u4-wave1.md W1-N2
(:305-352), W1-U4E (U4E-Q2: -58 judged after U4g); advisor-ws-docs-r2.md :373-383 (-18 split, ws-practice, practice
record, bound line); advisor-u2.md :101-103 (C4). u4s-carries.md: no U4j section (nothing carried to U4j).
Code: crates/log/src/lib.rs, crates/judge/src/{lib,main,marks,eval,figures,ratio}.rs, tests/{engine,cli,fixtures}.rs,
fixtures/*; marks/README.md, SERIES.md, marks.json (-1, -2, -3, -17, -18, -19, -20, -21, -22, -30, -58, PM-0006-11).
Readers of marks files outside my lane (must keep passing): crates/net/src/l2/{names,delay_names,socket_tests}.rs
(SERIES rows by `contains`; no `second.v.` row for a ws_split name), crates/server/tests/schedule.rs (PM-0004-67 ids),
check.mjs marks step (drift test, --check-marks) and check.test.mjs (every marks.json bite id is a BITE_ID).

## Plan
1. **npt-log `hs`** (§2 `hs`, nit c): `TickRecord.hs: BTreeMap<String, Vec<Option<String>>>` (name -> one entry per
   client slot, 16 lowercase hex digits, `null` where that log has no hash for that slot at that tick); writer and
   reader refuse a bad name or entry; `set_hashes` / `hashes` beside `set_hash` / `hash`. Schema tests.
2. **C4** (always-rule "build"): a run naming a bite has role bite or twin and at least one log built with `bites`; a
   run naming none has no log built with `bites` and is not role bite (run-level: -4's L2 fixture has the bites build
   on RS and LG's bots on the measured build). Twin figures check the twin's always-rules (a twin built wrong is a gap).
3. **Figures:** decoded_hash_mismatch_count / decoded_hash_compared_count (§4.6: `hs.server_hash` from the room,
   `hs.decoded_hash` from the clients, `hs.replay_hash` from the x86-64 re-decode; every phase; sentinel 0 always
   differs; orphan client hash counts; server-only is loss; Arm rule on aarch64 client logs; gaps for missing names,
   uneven lists, two logs at one (slot, tick), a server 0); cross_arch_mismatch_count (the hash kind); twin_hash_
   mismatch_count (h per tick against params.twin; a tick in one only counts); machine_named. "0 compared fails": a new
   Outcome::Empty (the run FAILs; no bite is shown by it: a bite run that compares nothing is INVALID).
4. **ws-practice** (Q20, ruling :375-378): rule `ws-practice:<tolerance>`; checks with `ws_bound` x|y|z scaled by f =
   max(1, M_p / M_m), rounded up to the ms; the practice run (role practice, same build hashes, scene, profile,
   params.burst_spacing_ns, earlier params.started_utc, listing the mark, valid) and its `ws_practice` record (class,
   chance/second rulings, m_p_ns recomputed from its heal events, the bound line verbatim and parsed, moved numbers
   recomputed, marks.json's X/Y/Z/k and tolerance equal to the bound's). Bite runs take the measured build's practice.
5. **marks.json:** -18 split (3 parts, checks (a)-(e) with bf9d248's provisional numbers, reports, bites + ws-budget-x2,
   notes); §4.12 table (-17, -18, -20, -21 L1 parts gain scene SS, clients 48, pre:PM-0001-2; -22 scene and clients
   only); W1-N2 (-58/lt-tick-offset what, the wrap check's name).
6. **README:** §1 `loopback` layer (one line), `hs` with `null` (nit c), roles twin and practice; §2.3 `ws_bound`, the
   empty outcome; §2.5 the bound line and the practice record formats (nit f); §4 C4's build rule, ws-practice,
   -22's exception in the pre:PM-0001-2 sentence (condition 4). **SERIES:** rev 2's rows (snap_incomplete,
   burst_eviction, the clamp events, precheck-30 suite), -18's rows (burst_heal wording + burst, burst_return's room
   fields, ws_skipped_snaps_count, unsent wording, ws_practice fields, burst_spacing_ns, config_hz, transport -18), the
   figure rows' formulas, hs names, Q30 closed.
7. **Fixtures** (real marks.json): pm-0001-17 (six decoded-hash cases + two Arm-rule cases), pm-0001-4 (C4: three),
   pm-0001-21 (twin hash: pass, fail, gap, waiting, empty; C4's twin case), pm-0006-11 (machine_named: PASS with the
   bite shown, FAIL); -18's ws-practice PASS and INVALID built in code (480 heal records) against the real marks.json.
   Engine tests for every edge.
8. Checks: check.mjs twice, --check-marks, clippy host and wasm32 both builds, fmt; counts as deltas on 286/309/48.

## Progress
- 1. Worktree at 411eeee (git log -1: "U4b: the loop policy ..."). No earlier u4j-build.md.
- 2. Sources read; plan written (above).
- 3. Code, first pass (compiles): log/src/lib.rs `hs` + `set_hashes`/`hashes` + check_hs in writer and reader + 2
  schema tests; judge marks.rs (`ws_bound` field, BoundNumber, Rule::WsPractice parse/name, validation), new
  src/practice.rs (bound line parser, ceil_ms, find(): the practice run and its record, 2 unit tests), figures.rs
  (Lack::Empty; twin_of shared by fate_log_twin_equal and twin_hash_mismatch_count; slot_hashes, decoded(),
  decoded_hash_mismatch_count, decoded_hash_compared_count, cross_arch_mismatch_count, machine_named; FIGURES rows),
  eval.rs (Missing::Empty, Outcome::Empty + fails(), C4 in always_rules, Ctx.mark, eval_check with ctx and ws_bound
  scaling via in_force, Rule::WsPractice, empty handling in judge_run/eval_part/fail_lines/eval_bite/reports, the Arm
  rule note). engine.rs LogSpec: features follow the bite (C4), plus hashed ticks, arch, binary.
  First test run (no new tests yet): log 11, judge lib 10, engine 29, fixtures 10, cli 2: all pass.
- 4. marks.json edited (before: sha256 83566c42…): -17 L1 parts (scene SS, clients 48, pre), a report of
  tick.s.snap_incomplete, notes; -58 the wrap check's name and lt-tick-offset's what (W1-N2); -18 split (3 parts,
  checks a-e, 7 reports, ws-budget-x2, notes); -20 L1 parts + notes; -21 8 parts + notes; -22 part (scene, clients
  only) + notes. --check-marks: 81 marks read and checked; fixtures 10/10 still pass.
- 5. README.md edited (before sha256 2f95bccb…): §1 tick row `hs`, roles twin/practice, process replay, the one-line
  `loopback` layer bullet, the `hs` bullet (names server_hash, decoded_hash, replay_hash; null; ordering rule);
  §2.1 "four" rules; §2.3 `ws_bound` and the empty comparison; §2.5 hs pointer, burst_spacing_ns, config_hz, the
  bound line format, the practice record format; §3 step 4; §4 four rules with C4's build rule, -22's exception in
  the pre sentence, `ws-practice:<tolerance>`; §5 empty comparison and build rule. Next: SERIES.md.
- 6. SERIES.md edited (before sha256 a902d10d…): rows changed in place: unsent_snaps_max_count (ruling (6)),
  burst_heal.after_link_ns (V29 wording, (1)), clamp_kind.counted_count (condition 1), transport +-18 ((7)), suite
  +precheck-30, figure rows cross_arch, decoded_hash_compared, decoded_hash_mismatch, machine_named,
  twin_hash_mismatch, fate_log_twin_equal (+ the twin's rules), Q30 answered; new section "Names added with U4j"
  (hs.server_hash, hs.decoded_hash, hs.replay_hash, tick.s.snap_incomplete, event.burst_eviction.evicted,
  event.clamp_report.x/y/z_um, event.burst_heal.burst, burst_return offset_ns/tick_phase_ns/budget_bytes/snap_bytes,
  tick.v.ws_skipped_snaps_count, event.ws_practice.* fields, params.burst_spacing_ns, params.config_hz).
  Next: engine tests (C4, decoded hash, twin hash, machine_named, marks validation), ws_practice.rs, fixtures.
- 7. Tests written, all pass: marks.rs unit `ws_bound_checks_keep_their_shape`; engine.rs (the ws-practice tests
  went into engine.rs to reuse LogSpec, judged with the real marks.json): c4_the_build_matches_the_bite,
  u4j_the_decoded_hash_gaps_and_its_every_phase, u4j_the_arm_rule_counts_an_aarch64_mismatch_only_when_the_x86_64_
  re_decode_misses_too, u4j_an_empty_comparison_fails_the_run_and_shows_no_bite,
  u4j_the_twin_hash_compares_every_tick_and_its_twin_whole, u4j_machine_named_needs_every_field_and_a_vm_its_address,
  u4j_ws_practice_puts_the_practice_runs_numbers_in_force, u4j_ws_practice_refuses_every_number_not_set_by_its_rules
  (20 INVALID cases + BEYOND with a chance ruling). engine 37/37. Removed the untestable "bite run naming no bite"
  sub-rule (such a run is never judged). Next: committed fixtures (pm-0001-17, -4 C4, -21, PM-0006-11).
- 8. Fixture FILES written (not yet run, no test functions yet): fixtures/pm-0001-17/{base (p2-room, p2-client),
  differ, lost-group, refused-group, orphan, none-compared}/p2-client.jsonl; fixtures/pm-0001-4/{measured-bites-build/
  s30-1-room, measured-names-bite/s30-2-room + s30-2-lg, bite-measured-build/bite-room}.jsonl;
  fixtures/pm-0001-21/{base (forged-room, forged-lab, twin-room), twin-differs/twin-room, twin-bites-build/twin-room,
  twin-missing/twin-room, twin-unnamed/forged-room}.jsonl; fixtures/pm-0006-11/{base (rs-room, bite-room),
  unnamed/rs-room}.jsonl.
  HALF-DONE / EXACT NEXT STEPS: (a) add test fns to tests/fixtures.rs for these cases (expected: -17 base NOT-RUN with
  part "L1 P2" run valid, figure value "0", compared report "6"; differ/refused-group/orphan/none-compared FAIL;
  lost-group part run valid, value 0, compared 5, incomplete report 1; -4 the three C4 cases INVALID; -21 base part
  "forged, option A" run valid, twin check value 0; twin-differs FAIL; twin-bites-build INVALID ("its twin
  forged-a-1-twin is invalid: build:"); twin-unnamed INVALID ("names no twin run"); twin-missing NOT-RUN; PM-0006-11
  base PASS with bite shown, unnamed FAIL), and extend the folder-reader test's list with the new marks; (b) update
  fixtures/README.md's table; (c) run judge tests; (d) clippy host + wasm32 both builds, fmt check (cargo +1.99.1 fmt
  --all --check; fix with fmt if needed: Edit-tool rule — rustfmt writes are cargo's, allowed?: prefer fixing by
  hand where few); (e) check.mjs twice with the touch loop and counts; (f) final notes and report.
- 9. Done (a)-(d): tests/fixtures.rs +5 tests (pm_0001_17 x2, pm_0001_4_c4, pm_0001_21, pm_0006_11; folder-reader
  list extended): fixtures 15/15 pass. fixtures/README.md table extended. fmt: ran `cargo +1.99.1 fmt -p npt-judge -p
  npt-log` (my crates only; rustfmt, not a shell write), `fmt --all --check` clean. clippy -D warnings host, default
  and --all-features, judge+log --all-targets: clean after one fix (engine.rs type alias Edit); log wasm32 clean.
  Next: (e) check.mjs twice (counted), then optional plants, then the report.
- 10. check.mjs run A (2026-11-10 21:16-21:18 UTC; no other cargo or rustc running; 2 s touch loop; the pre-run
  `cargo test --no-run` showed "Compiling npt-log (…agent-a3b8dfeb01ed52b50/spikes/netcode-proof/crates/log)" and
  "Compiling npt-judge (…/crates/judge)"): all 10 steps pass; test measured 304 (= 286 + 18), bites 327 (= 309 + 18),
  self 48 of 48; marks 81 read and checked, rows unchanged; redact 182 items, 0 findings. Expected delta +18 per
  build (log +2, judge lib +3, engine +8, fixtures +5): matches. Cut by the spend limit before noting it; resumed
  2026-11-11 09:50 UTC. Next: two fresh counted runs today (other lanes built in between), then plants, report.
- 11. check.mjs run B (2026-11-11 09:50:29-09:56:57 UTC; idle before; touch loop; Compiling npt-log and npt-judge
  from this worktree's path): all 10 steps pass; measured 304, bites 327, self 48, marks 81, redact 182/0. At its
  end a U4d `cargo test -p npt-sim` (pid 11844) was running; counts equal run A's. Next: wait for idle, run C.
- 12. check.mjs run C (09:57:54-10:00:07 UTC; idle before and after; touch loop; Compiling lines from this path): all
  10 pass; measured 304, bites 327, self 48, marks 81, redact 182/0. Runs B and C are the two in a row.
  `--list` (judge + log): 76 tests, the 18 new by name. Hashes before plants: figures.rs c034fc8f…, eval.rs
  4549e885…, practice.rs 1e59bdea…. Next: plants P1 orphan uncounted (figures.rs), P2 C4 off (eval.rs), P3 ceil_ms
  floors (practice.rs), P4 empty bite shown (eval.rs), each restored and sha256-checked.
- 13. Plants, one at a time, in place, `cargo test --no-fail-fast -p npt-judge` after a touch (Compiling npt-judge
  from this path each time): P1 orphans not counted (figures.rs decoded) -> FAILED
  u4j_an_empty_comparison_fails_the_run_and_shows_no_bite, pm_0001_17_fails_on_a_difference_the_sentinel_an_orphan_
  and_nothing_compared; P2 C4's bites-build check off (eval.rs) -> FAILED c4_the_build_matches_the_bite,
  pm_0001_4_c4_the_build_must_match_the_bite; P3 ceil_ms floors (practice.rs) -> FAILED practice::tests::
  moved_numbers_round_up_to_the_millisecond, u4j_ws_practice_puts_the_practice_runs_numbers_in_force,
  u4j_ws_practice_refuses_every_number_not_set_by_its_rules; P4 an empty comparison counted as a failure for bites
  (eval.rs) -> FAILED u4j_an_empty_comparison_fails_the_run_and_shows_no_bite. All restored: eval.rs 4549e885…,
  figures.rs c034fc8f…, practice.rs 1e59bdea… (the hashes runs B and C ran on). After the last plant: touch,
  rebuild, judge + log tests all pass (76), --check-marks 81, fmt clean. Next: the report (DONE after it).
- 14. Final hashes: marks.json 97c5cab5…, README.md 71e4c637…, SERIES.md e95febab…. No Cargo.toml, Cargo.lock or
  check.mjs change. 11 files modified, 1 new source file (practice.rs), 21 new fixture files. Report handed back.
  DONE.

## Fix pass, round 1
Fixer: general-purpose agent (opus, xhigh), 2026-11-11. Source of truth: the advisor's rulings
(main folder .checkpoints/netcode-proof-test/advisor-u4j.md, read whole: FX-1 to FX-14; FU-1 to FU-6 and the carries
are not mine). Start: the copy equals .gate/u4j-prefix (diff -rq of crates/judge, crates/log, marks: no difference).
The test-engineer's probe sources are not on disk (only their outputs): the probes are rewritten here as the named
tests. Runner: u4j-fix/run.sh (idle wait, df check, touch, 2 s touch loop; shared target, offline). Files: crates/judge,
crates/log, marks/ only. For each FX: the test first, its failure on the unfixed code quoted, the fix as ruled, the pass.

### Plan
- FX-1 (High) practice.rs: parse_bound refuses band_low > 1 or band_high < 1; FAR BETTER f = 1; a refusal of any
  f > tolerance before Ok. Tests: practice::tests::the_bound_line_refuses_a_band_that_leaves_out_1 (2.0-2.0, 1.01-1.2,
  0.5-0.9); engine u4j_ws_far_better_past_the_cap_is_invalid (probe A).
- FX-2 practice.rs find: candidates only practice runs started before the run; none: INVALID ("no practice run of its
  build started before it"). Test: engine u4j_ws_a_practice_run_after_the_counted_run_changes_nothing (E0, E1, E2).
- FX-3 eval.rs Ctx carries every run read; find reads them all; one per build over every set, listing the mark or
  not; the latest must list it; a bite run takes the practice run of its set's measured runs (same layer, profile,
  transport; one build). README §4 paragraph verbatim. Tests: engine u4j_ws_one_practice_run_per_build_over_every_set
  (probe F + the unlisted case), u4j_ws_a_bite_run_takes_its_counted_runs_practice_run (cases 1-4).
- FX-4 figures.rs twin_hash_mismatch_count: role twin, else a gap; doc comment. Fixture pm-0001-21/twin-is-measured,
  fixtures.rs pm_0001_21_the_twin_must_be_role_twin; engine twin test gains another profile and another bite (plant).
- FX-5 figures.rs: each hs name from its process; the Arm rule only on LG aarch64 with the room on x86_64; an orphan
  counts everywhere; a missing replay entry a gap. SERIES words. Fixtures pm-0001-17/{lg-cross-arch,
  lg-replay-differs, lg-replay-in-client-log, lg-replay-on-aarch64, l1-aarch64-claim, lg-orphan,
  lg-replay-missing-entry, lg-orphan-no-replay}; fixtures.rs pm_0001_17_the_arm_rule_holds_on_lg_only.
- FX-6 marks.rs: Mark.ws_bound_line (new pub field), checked at load against the ws_bound checks and the rule's
  tolerance; compile_check pins each letter's shape; find refuses a record whose bound is not the line byte for byte.
  marks.json -18: the provisional line, the notes sentence. README §2.1 row, §2.5 words. Tests: marks.rs
  ws_bound_line_agrees_with_its_checks, ws_bound_checks_keep_their_shape (+4 shapes); engine
  u4j_ws_the_record_carries_the_marks_bound_line (probe H + the commit case).
- FX-7 find: heal count = client_bursts_count. Test: refuses test + 479, 481.
- FX-8 parse_bound: at most 6 digits after the point. README §2.5 words. Test:
  practice::tests::the_bound_line_refuses_more_than_6_decimals (probe B).
- FX-9 tests only: engine u4j_ws_the_tolerance_and_band_boundaries (s = 1.00, 1.10, 1.1001, 0.95, Z' only); plant T.
- FX-10 figures.rs decoded: lists of `clients` entries, resolution, spans, the measured-phase cover, the order.
  SERIES words. Fixtures pm-0001-17/{sparse, sampled, slot-unlogged, hashed-and-incomplete, short-lists}, the six
  existing cases on 48-entry lists; fixtures.rs pm_0001_17_every_server_hash_is_resolved_on_the_client_side.
- FX-11 practice.rs describe in the ruled form. SERIES started_utc row + -18. Test: the puts test's assertions.
- FX-12 README §1 hs words, SERIES hs rows; fixture pm-0001-17/permuted-slots (FAIL).
- FX-13 find: ws_tcp_model (= the bound's model), config_hz and RS's kernel equal in both runs. README §2.5 drop.
  Tests: refuses test + 7 cases; the ws headers gain the three params (and an RS room log).
- FX-14 tests: machine_named FAIL list + 6; fixture pm-0001-1/laptop, fixtures.rs pm_0001_1_a_laptop_needs_no_address;
  plant "role not checked".
- End: fixtures/README.md table; fmt --check; clippy host + wasm32 both builds; check.mjs twice counted; hashes.

### Progress
- FX-1 DONE (11:52 UTC; every run below from u4j-fix/run.sh, "Compiling npt-log" and "Compiling npt-judge" from this
  copy's path each time). Before the fix: `the_bound_line_refuses_a_band_that_leaves_out_1` FAILED, "panicked at
  crates/judge/src/practice.rs:527:13: band 2.0 to 2.0 must be refused"; `u4j_ws_far_better_past_the_cap_is_invalid`
  FAILED, "panicked at crates/judge/tests/engine.rs:2078:5: left: Pass right: Invalid" (probe A passes today). Fix:
  parse_bound's band rule 0 < low <= 1 <= high; f = 1 for FAR BETTER (else max(1, s)); `f > tolerance` refused before
  Ok. After: both pass; npt-judge 12 + 0 + 2 + 38 + 15 all pass.
- FX-2 DONE (11:53). Before: `u4j_ws_a_practice_run_after_the_counted_run_changes_nothing` FAILED at engine.rs:2143:9,
  "E1: [\"L2 B1 ws: run ws-1: ws-practice: practice run ws-practice-2 did not start before run ws-1\"] left: Invalid
  right: Fail" (E0 FAILs as it must). Fix: only practice runs started before the run are candidates; none gives
  "run <id>: no practice run of its build started before it"; the later start check removed (unreachable). The two
  refuses-test cases that named the old texts (another build; a practice run started after) now expect the new text.
  After: E0, E1, E2 all FAIL; npt-judge 12 + 2 + 39 + 15 pass.
- FX-3 DONE (11:59; the runner waited 205 s for another lane's `cargo test --workspace`). Before:
  `u4j_ws_one_practice_run_per_build_over_every_set` FAILED at engine.rs:2178:9, "listed true: [] left: Pass right:
  Invalid" (probe F); `u4j_ws_a_bite_run_takes_its_counted_runs_practice_run` FAILED at engine.rs:2241:5 (case (2):
  "left: Shown right: Invalid"; case (1) passed before it). Fix: eval.rs Ctx carries `all` (every run read; in_force
  and the rule pass it); practice.rs practice_of (over every run: role practice, the same binaries, started before the
  counted run; the latest; a second_ruling with more than one; the latest must list the mark), measured_of (a bite
  run's set's measured runs serving the mark with its layer, profile and params.transport; one build, else INVALID;
  none, INVALID), find (a bite run takes their practice run; if they had two different ones in force, INVALID: a
  reading of "the practice run of its set's measured runs", stricter, flagged in the report). README §4's paragraph
  replaced with FX-3's words verbatim. After: both pass (cases 1-4); npt-judge 12 + 2 + 41 + 15 pass.
- FX-4 DONE (12:01). New fixture pm-0001-21/twin-is-measured/{twin-room, twin-lab}.jsonl (the base's twin replaced
  by a second forged measured run naming forged-a-1, equal h). Before: `pm_0001_21_the_twin_must_be_role_twin`
  FAILED at fixtures.rs:354:5 "left: NotRun right: Invalid" ("forged, option A: 2 of 3 valid runs"); the engine twin
  test's new measured-twin case FAILED at engine.rs:1651:9 "run f1: f1-twin has role measured, not twin: [] left:
  Pass right: Invalid". Fix: twin_hash_mismatch_count refuses a twin whose role is not twin as a gap ("run <id>:
  <twin> has role <role>, not twin"); doc comment drops "any role". After: npt-judge 12 + 2 + 41 + 16 pass. TE-F14
  (another profile, another bite: pass today) proven by plants in figures.rs (sha256 dc007d2a before): drop
  "profile" from the fields -> FAILED engine.rs:1651:9 "its twin f1-twin has another profile: []"; restored
  (dc007d2a); drop "bite" -> FAILED "its twin f1-twin has another bite: []"; restored (dc007d2a). NOT EDITED: SERIES's
  twin_hash_mismatch_count row still says "any role (-21's twins are role twin, -30's each other's measured runs)";
  FX-4 gives words for the doc comment only (flagged for the advisor).
- FX-5 DONE (12:11). Fixtures (today's 2-entry form; FX-10 moves the L1 one to 48): pm-0001-17/{lg-cross-arch,
  lg-replay-differs, lg-replay-on-aarch64, lg-orphan, lg-replay-missing-entry}/{lg-room, lg-bots, lg-replay}.jsonl,
  lg-replay-in-client-log/{lg-room, lg-bots}.jsonl (run l2-p2-1: RS room x86_64, LG1 bots aarch64, C replay; clients
  2), l1-aarch64-claim/{p2-client (role C, aarch64, orphan at tick 9), p2-replay}.jsonl. Test
  `pm_0001_17_the_arm_rule_holds_on_lg_only` collects every case. Before: FAILED at fixtures.rs:368:5 with
  "lg-replay-in-client-log: L2 P2 is Pass", "lg-replay-on-aarch64: L2 P2 is Pass", "l1-aarch64-claim: L1 P2 is NotRun
  [the run valid: 1 of 3 valid runs]", "lg-orphan: L2 P2 is Pass", "lg-replay-missing-entry: L2 P2 is Fail" (the
  advisor's "today" results, all five); lg-cross-arch and lg-replay-differs right already. Fix (figures.rs): `source`
  per name, a name in another process's log a gap; the entry's flag is LG && aarch64; the Arm rule only with every
  room log on x86_64; an orphan counts on every machine; a missing replay entry a gap. Engine tests: the "two logs"
  case now uses a second client log (a `lab` log is now the wrong process), + a lab decoded_hash case, + a client
  server_hash case, the Arm test's client is LG, + an aarch64 room case (FAIL). After: npt-judge 12 + 2 + 41 + 17
  pass; folder-reader test lists the six L2 cases. Plant (figures.rs 0201344c): room_x86 forced true -> FAILED
  engine.rs:1470:5 (the aarch64-room case); restored, 0201344c. SERIES: the decoded row's Arm sentence replaced and its
  Gaps gain FX-5's words; the cross_arch row drops the two phrases and says "LG aarch64" (the advisor's "Its count is
  the LG aarch64 mismatches ..."). README §1's hs items: with FX-12. lg-orphan-no-replay: with FX-10 (f).
- FX-6 DONE (12:15). Before: `u4j_ws_the_record_carries_the_marks_bound_line` FAILED at engine.rs:2274:5 "H1: []
  left: Pass right: Invalid" (H0 FAILs as it must; probe H passes today); the marks.rs unit tests first failed to
  compile ("error[E0609]: no field `ws_bound_line` on type `Mark`"); with the field plumbed through only (RawMark,
  Mark, compile_mark) and nothing checked, both FAILED: `ws_bound_line_agrees_with_its_checks` at marks.rs:1097:39
  "its X is 680000000, check \"x\" holds 670000000: Marks { ..." (the mark compiled), `ws_bound_checks_keep_their_shape`
  at marks.rs:1198:65 "x with op >=: Marks { ..." (compiled). Fix: marks.rs `ws_bound_line` (RawMark; new pub field
  `Mark::ws_bound_line: Option<String>`); compile_mark: none with ws_bound checks, one without them, an unparsable
  line, X/k/Y/Z not the checks' (practice::checks_agree, shared with find) or a tolerance not every ws-practice rule's
  are refused; compile_check pins x (p50, <=, no where), y (count, where >, <=), z (max, <=, no where). practice.rs
  find: the record's `bound` must equal mark.ws_bound_line byte for byte, the bound parsed from the mark's line,
  checks_agree kept. marks.json -18: `ws_bound_line` = FX-6 (d)'s provisional line, notes + FX-6 (e)'s sentence
  (after "... before R3's run plan."). README §2.1 row (FX-6's §2.5 words plus (a)'s "A mark with `ws_bound` checks
  must have it, and only such a mark may"), §2.5 sentence and the record bullet's `bound` words replaced. engine.rs:
  WS_BOUND is the provisional line; the refuses test's four bound-edit cases now expect the byte-for-byte refusal.
  After: npt-judge 13 + 2 + 42 + 17 pass; `--check-marks`: 81 marks read and checked.
- FX-7 DONE (12:16). Before: the refuses test's new 479 case FAILED at engine.rs:2085:9 "practice run ws-practice has
  479 heal records, not the bound's 480: [] left: Pass right: Invalid" (probe G); the 481 case follows it (M_p 680 ms
  in both). Fix: Bound keeps client_bursts_count; find refuses heal.len() != it (replacing the empty-list check, which
  it covers: the count is at least 1). After: npt-judge 13 + 2 + 42 + 17 pass.
- FX-8 DONE (12:16). Before: `the_bound_line_refuses_more_than_6_decimals` FAILED at practice.rs:653:13
  "\"band_low\":\"0.9650001\"" (a 7-digit fraction accepted). Probe B itself no longer panics on this copy: FX-1's band
  rule (band_high 0.999... < 1) refuses it before any cross comparison, so the 29-digit panic path in parse_bound is
  already gone; the test keeps probe B and adds a 7-digit case for each of the four fractions and a 6-digit one that
  reads. Fix: parse_bound's `decimal` refuses more than 6 digits after the point before Ratio::parse. README §2.5's
  sentence replaced in FX-8's words. After: npt-judge 14 + 2 + 42 + 17 pass.
- FX-9 DONE (12:18). `u4j_ws_the_tolerance_and_band_boundaries` (s = 1.00 PASS at the bound, f 1; s = 1.10 PASS at
  737 ms / 2.75 s / 7.59 s; T1b s = 1.1001, T2 s = 0.95 below band_low, T3 one heal over Z' only, each INVALID by
  name; the three refusals collected so one plant shows each) passes on the fixed code (12:17). Plant T in place in
  practice.rs (477c715e before; planted b8763832): IN TOLERANCE `s <= tolerance + 1/50` (band_low dropped, cap
  1.10 -> 1.12), BEYOND `s <= tolerance + 1/50`, FX-1's guard `f > tolerance + 1/50` (the cap everywhere it is
  applied: with FX-1's guard unplanted the cap stays 1.10 and T1b could not bite), Z' recheck dropped (`&& worst >=
  0`). Result: FAILED engine.rs:2235:5 with "T1b: Pass []", "T2: Pass []", "T3: Pass []"; every other judge test
  passed (14 + 2 + 42 + 17). Restored by Edit: 477c715e.
- FX-10 DONE (12:25). Fixtures: the six existing L1 cases and l1-aarch64-claim moved to 48-entry lists (slots 2-47
  null in both logs, snap_incomplete 0; each file byte-equal to the generator's output, a scratchpad script printed
  and the files written by Write); the existing tests stayed green on the unfixed code (12:22). New:
  pm-0001-17/{sparse, sampled, slot-unlogged, hashed-and-incomplete}/p2-client.jsonl, short-lists/{p2-room,
  p2-client}.jsonl (the old 2-entry base, byte-identical), lg-orphan-no-replay/{lg-room, lg-bots}.jsonl. Before:
  `pm_0001_17_every_server_hash_is_resolved_on_the_client_side` FAILED at fixtures.rs:416:5 with all five "L1 P2 is
  NotRun, wanted Invalid ... [\"L1 P2: 1 of 3 valid runs\"]" (each run valid today); `pm_0001_17_the_arm_rule_holds_
  on_lg_only` FAILED at fixtures.rs:375:5 "lg-orphan-no-replay: L2 P2 is NotRun ... the run has no x86-64 re-decode"
  (the Waiting returned at once). Fix (figures.rs): `decoded` returns the structural gaps at once (clients from the
  room log; every hs list and every client log's snap_incomplete list of `clients` entries; snap_incomplete 0 or 1),
  then counts, keeping `gap` (FX-5 (d) and the new `coverage`: (b) both at once, (c) inside a span, (d) the measured
  cover) and `waiting`; decoded_hash_mismatch_count orders count > empty > gap > waiting > 0; cross_arch is NOT-RUN
  while waiting. engine.rs: LogSpec gains `clients` (48; hash logs 2); the slot-count case expects the new words.
  SERIES: the decoded row's Gaps gain FX-10's words. After: npt-judge 14 + 2 + 43 + 18 pass. Added after the fix
  (12:25): two engine gap cases, a snap_incomplete list of 3 under clients 2 and a snap_incomplete of 2 (the unfixed
  code read neither); they pass.
- FX-11 DONE (12:26). Before: the puts test's new assertions FAILED at engine.rs:1937:9 "practice run ws-practice: IN
  TOLERANCE: <= 697000000 (X 670000000 x f 1.039755 = 697000000 rounded up to the ms; f = max(1, s), s = M_p 680000000
  / M_m 654000000 = 1.039755; practice run ws-practice)" (no class, band or tolerance). Fix: Bound keeps the band and
  tolerance texts; Practice keeps class, chance_ruling and second_ruling; describe prints FX-11's form, then the
  rulings' texts ({:?}). The test also asserts "f = 1.039755 (max(1, s))" and a new BEYOND case's "practice run
  ws-practice: BEYOND" and "chance ruling: \"advisor-r3.md:20: a lone count\"". SERIES started_utc row + -18. After:
  npt-judge 14 + 2 + 43 + 18 pass.
- FX-12 DONE (12:27). README §1: the `hs` bullet's first sentence, the `server_hash` insertion after "never 0;", the
  `decoded_hash` item's two replacements and the `replay_hash` item's, in FX-12's words; the `twin` role's words.
  SERIES: the hs.server_hash, hs.decoded_hash and hs.replay_hash rows gain the same process words. Fixture
  pm-0001-17/permuted-slots/p2-client.jsonl (slots 0 and 1 swapped on every tick; 48 entries), judged in
  `pm_0001_17_fails_on_a_difference_the_sentinel_an_orphan_and_nothing_compared`: FAIL "= 6, needs == 0" (passes on
  the unfixed code too, as the ruling says: it documents the pairing). npt-judge fixtures 18 pass.
- FX-13 DONE (12:30). Test scaffold first: engine.rs `Setup` (RS kernel, params.config_hz, params.ws_tcp_model) and
  `SETUP` (6.8.0-48-generic, 250, the provisional model's text); `ws_run` builds LG's bots log plus RS's room log (the
  kernel); `edit_run` and `another_practice` now act on whole runs (FX-2/FX-3 tests updated to them); the rest of the
  engine tests stayed green on the unfixed code. Before: the refuses test's first new case FAILED at engine.rs:2247:9
  "practice run ws-practice ran another RS's machine.kernel than run ws-1: [] left: Pass right: Invalid". Fix:
  Bound keeps `model`; practice.rs `rs_kernel` (RS logs' kernel, agreeing; empty is absent); find refuses a run or
  practice run lacking params.ws_tcp_model, params.config_hz or RS's kernel, any of them differing, and a run whose
  ws_tcp_model is not the bound's model. Cases: another kernel, config_hz, ws_tcp_model; a model not the bound's;
  each of the three absent (from both runs). README §2.5's record bullet drops "the header checks". After: npt-judge
  14 + 2 + 43 + 18 pass.
- FX-14 DONE (12:31). Tests only: the machine_named FAIL list gains no role, name, plan, arch, memory, threads per
  core (14 entries); fixture pm-0001-1/laptop/ml.jsonl (ML, every field, no address; its first draft had the footer
  count 0, which npt-log refused: "the footer counts 0 records; 1 came before it", fixed to 1) judged in
  `pm_0001_1_a_laptop_needs_no_address` (the ML part PASS, run valid, machine_named 1); fixtures.rs `verdict` skips a
  missing base folder (PM-0001-1 has none). Plant (figures.rs e9282de6; planted bfb7e0d5): role not checked (`&m.role`
  -> `&m.name`) -> FAILED engine.rs:1737:9 "no role left: Pass right: Fail"; restored, e9282de6. After: npt-judge
  14 + 2 + 43 + 19 pass.
- End checks (12:34). fixtures/README.md table: 17 rows added, the base row's 48-entry words, the engine-test list.
  `cargo fmt --all --check`: rustfmt's diff (marks.rs 6 hunks, practice.rs 5, engine.rs 7, fixtures.rs 1) applied by
  Edit, then clean. clippy -D warnings: host `-p npt-judge -p npt-log --all-targets` measured and --all-features,
  wasm32 `-p npt-log` both: first 3 errors (2 collapsible_if, 1 explicit_auto_deref), fixed by Edit (let chains; the
  build count taken as a length), then all four clean. npt-judge 14 + 0 + 2 + 43 + 19, npt-log 11: pass;
  `--check-marks` 81. New tests: lib +3, engine +6, fixtures +4 = +13 per build (expected 317 / 340 / 48).
  Next: check.mjs twice, counted.
- check.mjs (each from run.sh: idle before, 2 s touch loop, the pre-run `cargo test --locked --no-run -p npt-judge -p
  npt-log` printing "Compiling npt-log" and "Compiling npt-judge" from this copy's path; outputs in the scratchpad):
  - run 1 (12:34:31-12:36:59): all 10 PASS: fmt 0.5 s; clippy 7.5 s; clippy-wasm32 0.6 s; doc 30.8 s; test 104.9 s
    measured 317 passed, bites 340 passed, 0 failed, 0 skips; deps 0.6 s held; refs 597 files; marks 81 read and
    checked, rows unchanged; self 48 of 48; redact 10 of 10, 213 items, 0 findings.
  - run 2 (12:37:16-12:39:15): 9 PASS, test FAIL: measured 317 passed; bites 299 passed, 1 failed in `-p npt-net
    --lib` (not this lane; the failing test's name is not in check.mjs's summary). npt-net's lib on the bites build
    then passed 3 runs in a row alone (169 passed, 12 ignored each, 12:39-12:41): a timing flake, not this change
    (no npt-net file touched; its SERIES readers passed in run 1 and after).
  - run 3 (12:41:29-12:43:35) and run 4 (12:43:44-12:45:47), the two in a row: all 10 PASS both: fmt 0.4/0.4 s;
    clippy 2.8/3.0 s; clippy-wasm32 0.5/0.6 s; doc 32.8/32.3 s; test 85.3/83.6 s measured 317, bites 340, 0 failed;
    deps 0.6/0.6 s; refs 597; marks 81, rows unchanged; self 48 of 48; redact 213 items, 0 findings. Idle after.
  - Counts as deltas: measured 317 = 286 + 31 (304 + 13), bites 340 = 309 + 31 (327 + 13), self 48 = 48 + 0.
- Final: no Cargo.toml, Cargo.lock or check.mjs change (git diff empty); crates/log unchanged by this pass; public
  items added: npt-judge `marks::Mark::ws_bound_line: Option<String>` only (practice.rs is a private module; Ctx is
  pub(crate)); none removed. Hashes (sha256, first 8): eval.rs 09a17b8a, figures.rs e9282de6, marks.rs 294b9007,
  practice.rs e074d58d, engine.rs d3b437b8, fixtures.rs bb4fb3a5, fixtures/README.md 86836e17, marks.json baa69105,
  README.md 2e837f6f, SERIES.md fa03e6dc; fixtures in the report. DONE.

## Merge into the spike branch (the main session, 2026-11-11, under advisor-u4j-r2.md B1's conditions M1 to M9)
Round 2 approved (three APPROVE WITH FINDINGS; F1 closed by all three; no round 3); the advisor's round-2 ruling
APPROVE, merge now. Each condition's result, in the main session's words:
- **M1, the copy is the one reviewed:** the worktree's HEAD is 411eeeed50d8c71e7e1f74366f0e1d5c27388444, and
  `sha256sum -c` of u4j-round2-test/hashes-start.sha256, from the worktree's root, gives 66 OK.
- **M2, the base still fits:** `git diff --quiet 411eeee HEAD -- spikes/netcode-proof/crates/judge
  spikes/netcode-proof/crates/log spikes/netcode-proof/marks` at the branch head 30528c7: true. Nothing under spikes/
  changed since 39f4290.
- **M3, the snapshot and the apply:** no saver running; `worktree_patch.py` listed 64 files, all under
  spikes/netcode-proof/; the new wip/u4j.patch's sha256 (98ee1f63…) equals 4ad80a9's; `git apply --check`, then
  `git apply`, on 30528c7.
- **M4, the merged bytes are the reviewed bytes:** from the main folder's root, `sha256sum -c` of hashes-start.sha256's
  64 `spikes/` lines gives 64 OK.
- **M5, check.mjs counted on the merged tree:** through main-run.sh (new in this folder: the lane lock
  `.gate/shared-target/.lane.lock` held exclusively for the whole sequence, idle at the start, 11 GB free, the 2 s
  touch loop on crates/, the shared target, offline), 19:55:22-20:01:31 UTC, log .gate/u4j-merge/check1.log. The
  pre-run builds `cargo test --locked --workspace --no-run`, and the same with `--all-features`, each show "Compiling
  npt-judge" and "Compiling npt-log" from the main folder's path (every member compiled from it, once in each). Then
  check.mjs, exit 0, "check: all 10 steps pass": test, measured build 396 passed, 0 failed, 0 reported a skip; bites
  build 443 passed, 0 failed, 0 reported a skip; marks 81 read and checked, the ADR rows unchanged; self 52 of 52;
  redact 10 of 10 tests, 236 items, 0 findings; refs 677 files. The counts the ruling expects (365 + 31, 412 + 31, 52).
- **M6:** no test failed, so nothing was re-run.
- **M7, the gate:** `node tools/gate.mjs`: checks 1 to 4 and 5b to 5d PASS; 5a (smoke) FAIL, because Chromium could not
  start: tools/'s Playwright (playwright-core 1.56.1) looks for /opt/pw-browsers/chromium_headless_shell-1194, and this
  container holds only the 1243 builds. That is the container's fault, not this change's (it touches none of the five
  legacy files and nothing under tools/). `node tools/gate.mjs --no-smoke`: GATE PASS, as at U4d's merge; CI runs the
  full gate, smoke included, on the push. Logs: .gate/u4j-merge/gate1.log and gate1-nosmoke.log.
- **B7.3, U4j's public items, re-derived** by `git diff -U0 411eeee -- crates/judge/src crates/log/src` on the applied
  tree (every added `pub` line, and every new variant of a `pub` enum): npt-judge `eval::Outcome::Empty` and
  `Outcome::fails`, `figures::Lack::Empty`, `marks::BoundNumber`, `marks::Check::ws_bound`,
  `marks::Mark::ws_bound_line`, `marks::Rule::WsPractice`; npt-log `TickRecord::hs`, `TickRecord::set_hashes`,
  `TickRecord::hashes`. Not public:
  `eval::Missing::Empty` (pub(crate)) and practice.rs (`mod practice;`, judge/src/lib.rs:9). The ruling's list, so
  §4.1's two rows are its words.
- **The merge commit's text,** by u4j_merge_text.py (this folder; its dry run, then `--apply`; every block copied from
  the rulings by line range, each anchor found once, every new text found in place after, no changed prose line over
  120 characters, each refilled paragraph's words equal to the old ones with only the substitutions):
  - u4s-carries.md: entries 61 (§U4e, after 57), 62 and 63 (§U4r, after 60), the new section "The next change to
    crates/judge (whoever makes it; its first step)" with entry 64, and 65 (§U4d2, after 52), from advisor-u4j-r2.md
    :447-536 (sha256 f18a0692 → 32acca91);
  - u4-build-plan.md: §4.1's npt-log row and the new npt-judge row after it (B7.3); §10's preamble and §11 (B7.5, the
    orphan reading, its quote copied from advisor-u4j.md :538-540 and checked equal to the ruling's block) (7375cdcf →
    4b8eaab6);
  - plan.md: the U6 row's C-U6-9 to C-U6-11, after C-U6-8's last words, inside the cell (B7.4) (4e74a152 → 2bef127b);
  - this file: "Readings, as ruled" below (B5).
- **M8:** this merge commit (the 64 spike files as applied, `git rm` of wip/u4j.patch, the text above), then the text
  commit (T1 to T9 by u4j_t1t9.py, its diff holding nothing else; then check.mjs counted again and the gate), both in
  one push. **M9**, after the push: the saver restarted, the card updated, then U4c's L-c8 judged re-run.
- **Done:** the merge commit 4d2b0ef; the text commit d50396a (u4j_t1t9.py, its dry run, then `--apply`: T1 to T4 in
  marks/README.md, each item refilled with its words otherwise unchanged; T5 in marks.json, which still parses with
  only PM-0001-18's notes changed; T6 to T8 in SERIES.md; T9 in fixtures/README.md; the commit holds those 4 files
  only). Before it, check.mjs counted again (main-run.sh, 20:04:03-20:08:44 UTC, log .gate/u4j-merge/check2.log; the
  pre-run builds compiled npt-judge and npt-log from the main folder's path): all 10 steps pass, measured 396, bites
  443, 0 failed, 0 skips, marks 81 with the ADR rows unchanged, self 52 of 52, redact 0 findings (refs 732 tracked
  files, 677 before the merge commit added 56); `node tools/gate.mjs --no-smoke`: GATE PASS (gate2-nosmoke.log).
  Pushed together, 30528c7..d50396a.

## Readings, as ruled
Copied by script (u4j_merge_text.py), by line range, as the advisor's U4j round-2 ruling asks (advisor-u4j-r2.md, B5;
the main session's words in this paragraph only): advisor-u4j.md :472-506 (R3, whole; its heading one level down), then
advisor-u4j-r2.md :207-218 (B4.2), :220-231 (B4.3) and :174-177 (B3's NC-U4J-R2-N2 line). Line numbers inside the copied
text are the rulings' own.

### R3. The builder's readings (a) to (f)
Quoted from the builder's handback ("Readings for the advisor or reviewers"). They are not in u4j-build.md, so the
main session should copy them into its notes at the merge.
- **(a) "C4's 'at least one log' form", because -4's U1 fixture has RS's room on the bites build while LG's bots run
  the measured build: ACCEPTED.**
  - netcode: right. security F7: sound in the measured direction, and weaker in the bite direction, where it fails
    safe.
  - Recorded as a risk; tightened by FU-1.
- **(b) "A bite run under ws-practice takes its set's measured-build practice run": CHANGED (FX-3 (c)).**
  - netcode: right, under R11.
  - Kept: the bite run is judged at the counted run's numbers.
  - Changed:
    - the practice run is the one in force for its set's measured runs on the same layer, profile and transport
      (one build), found over every run the judge reads, with FX-2's start rule;
    - so a practice run of an older build elsewhere cannot make it INVALID;
    - and the bite run cannot take an f other than its counted run's.
- **(c) "The ruling's `machine.config_hz` is written as `params.config_hz`", since a new Machine field would break
  npt-lab's machine.rs: ACCEPTED.**
  - No reviewer objected.
  - My ruling's text (advisor-ws-docs-r2.md:377, SERIES (5)) reads as `params.config_hz` from now on.
  - FX-13 compares it.
- **(d) "A bite run that compared nothing is INVALID": ACCEPTED.** All three agree. It can only lower a verdict. It
  follows README §2.3's rule for setup checks: a run that did not test the mark shows no bite.
- **(e) "The Arm rule is keyed on the client log's `machine.arch` = aarch64, not on `mark.arm_rule`": CHANGED
  (FX-5).**
  - netcode F6 (partly), security F4, test F5 and F11 (wrong).
  - The key is now:
    - an LG log on aarch64, with the room on x86_64;
    - each name from its own process;
    - an orphan counts everywhere;
    - a missing replay entry is a gap.
  - The mark's flag is FU-5.
- **(f) "The decoded-hash figures read every phase", because -58's wraps fall in its first seconds: ACCEPTED.**
  netcode: right, and it makes NC-5 (ii) a writer rule. That rule is in FX-12's `server_hash` words (from admission,
  in every phase the clients log).

### B4.2 The stricter reading of FX-3 (c): two practice runs in force make a bite run INVALID (TE2-3): ACCEPTED
- The case: a bite run's set holds measured runs of one build, but they have two different practice runs in force
  (one measured run started after a second practice run of the build, which carries a `second_ruling`). The code
  refuses it (practice.rs:458-470, in `find`); the test-engineer's probe BITE2 shows it on the head.
- Why accepted: FX-3 (c) judges a bite run "at its counted run's numbers"; here there are two sets of numbers. A
  smaller f could show the bite falsely, a larger one hide it, so any choice is a guess. INVALID never guesses and can
  only lower a verdict (an INVALID bite run shows no bite, so the mark cannot pass on it). The same holds when one of
  the set's measured runs has no practice run in force at all (`practice_of`'s error, which `find` passes on). The
  cost: re-run the bite after the set's measured runs share one practice run.
- README §4 says it (text commit, T4 (i)). Its case: entry 64 (c).
- A reading of my ruling, not of an ADR row or of Mustafa's words, so it goes into the notes (B5), not the readings
  PR.

### B4.3 FX-10's malformed entries and its order of outcomes (TE2-4): ACCEPTED
- A `tick.s.snap_incomplete` value other than 0 or 1 is a structural gap. README §1 names such a series "a 0/1 flag"
  and SERIES's row says "flag", so this is FX-10 (f)'s "malformed entries", checked first. The fixer added two engine
  cases for it (a list of 3 under `clients` 2; a value of 2).
- The order count > empty > gap > waiting > 0 (figures.rs:538-560). FX-10 (f) fixed the first three and put the
  waiting case after the counted mismatches; it left gap against waiting open. Gap first is right: a run with a
  coverage gap or an incomplete re-decode cannot become valid when its replay log arrives, so NOT-RUN ("wait for the
  re-decode") would send the operator after a log that cannot save the run. Both are non-pass, so the order can never
  loosen a verdict.
- `cross_arch_mismatch_count` NOT-RUN while a mismatch waits for its re-decode: accepted (it cannot be counted
  before the re-decode exists).
- Its case: entry 64 (d).

- **NC-U4J-R2-N2** (a same-build practice run with no readable `params.started_utc` turns a FAIL into INVALID;
  confirmed by the test-engineer's probe2): the behaviour stays, as the security-auditor argues (skipping such a run
  would let a stripped timestamp hide a first practice run from the one-per-build count; it fails closed, never
  looser); README §4 says so in the text commit (T4 (ii)). No code change.
