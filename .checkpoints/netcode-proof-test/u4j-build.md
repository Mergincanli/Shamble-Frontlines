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
