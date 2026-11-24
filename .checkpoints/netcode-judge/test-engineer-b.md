# test-engineer, part b (U6j) review, one round on the final diff 0e0f6547..a3476e6c. 2026-11-24
## Plan
1. Run the four commands (tests, clippy, fmt, --check-marks); marks.json unchanged check.
2. Read the diff: figures.rs, eval.rs, ratio.rs, lib.rs, main.rs; tests engine.rs (u6j_*, c_u6_11_*, alloc_*),
   fixtures.rs (pm_0001_30_*), cli.rs; SERIES.md, README.md, fixtures/README.md.
3. The 11 conditions: each against code and its test.
4. The 9 decisions: right / wrong / needs a ruling.
5. Plants (>= 6, my own) in a scratch copy under .gate/te-plant-b/, each run against the named test.
6. Findings, verdict.
## Blocks done (one line each)
- B1 commands at a3476e6c: cargo test -p npt-judge --locked --no-fail-fast: lib 18, main 0, cli 4, engine 75,
  fixtures 25, doc 0 = 122 pass, 0 fail. clippy --all-targets -D warnings: exit 0. cargo fmt --check: exit 0.
  --check-marks: "81 marks read and checked (drafts at 626df678...)", exit 0. marks.json: git diff --quiet = unchanged.
- B2 read: figures.rs whole (twin, coverage, 19 figures, table), eval.rs diff (Parts first pass, Missing::Fail,
  fail_lines, report detail, LoggingOff fix, judge_only), ratio.rs cmp_wide, lib/main; tests c_u6_11 x2, alloc x2,
  u6j x16, pm_0001_30 x2, cli only; SERIES/README/fixtures README diffs. Only removed test line: the far-better
  `says("ws-practice: ")`, replaced by a stronger exact-reason assert (TE2-2): no expectation changed.
- B3 plants: scratch copy .gate/te-plant-b (tar of the workspace), script te-plant-b.mjs + te-plant-b-run.sh;
  11 plants (1 control) running; results in .gate/te-plant-b/plant-results.txt.
- B4 plant results (te-plant-b-run.sh): control pb0 (last session gets a tail) CAUGHT by c_u6_11_coverage_runs...;
  my 10 plants ALL SURVIVE the 122 tests: pb1 session_end from any window, pb2 last end in window, pb3 admit from
  any log, pb4 sync slot 0 only, pb5 sync last tick only, pb6 R2 E = largest slot, pb9 on pool every set, pb17 Parts
  keep undecided, pb18 Parts keep moved, pb19 Parts ignore setup checks.
- B5 probes (te-probe-b.rs, appended to the copy's engine.rs): 6 probes pass on the real code; each of pb1-pb6, pb9
  fails its probe (pb1 also fails pb2's). So the code is right there; the suite does not pin it.
- B6 overflow probes (copy's engine.rs): 4 valid N=24 runs at 10-minute sizes (Sum r ~1.64e7, coprime) PANIC at
  ratio.rs:27 "a fraction overflowed i128" (probe_sync_four_valid_ten_minute_references); 3 runs at 30 min: a value;
  the existing large-sums case + a 4th run: a value (its denominators share factors). The panic aborts the binary.

## The 11 conditions: code and proof
| # | Code | Proof | Gap |
|---|---|---|---|
| 1 | block 1 commits ab1074e3..0d31500e precede 24c3ba21 | git log order | report's one line: not checkable here |
| 2 | eval.rs:1767-1794 first pass (kept parts, validity, undecided, setup) | block 2 diff touches no test (git diff 0d31500e 24c3ba21 -- tests: empty); the cut-log cases (engine.rs:4082-4091, :4555-4570) | undecided, moved, setup-failed exclusions untested: pb17-19 survive (M3) |
| 3 | R2Counts figures.rs:1301-1388 (E - V form) | golden E 62 V 15 -> 146122.56, 3,156,247,296 B (engine.rs:4194-4280); marks.json unchanged | E's averaging unpinned (M1) |
| 4 | SERIES keeps "(proposed method)" | report item | not checkable here |
| 5 | sessions() figures.rs:587-601 (no admit: one span) | c_u6_11_a_mid_run_join... (no admit: old words; no end: INVALID); -17 fixtures pass | parenthetical (L9) |
| 6 | kill_p99_ratio figures.rs:1628-1658, built last (a3476e6c) | u6j_a_surviving_rooms... pass/fail/gap/wait/two; SERIES -27 row and README 2.5 state the writer need | none |
| 7 | README 2.5: deferred, "exempts every log that says killed", home before R5's -27/PM-0006-22 | text | none |
| 8 | SERIES admit/session_end row (own row); room_end `slot` | text | room_end edited in place (L8); board post not checkable |
| 9 | eval.rs:958-967 always_rules on off runs | u6j_a_logging_off_run_holds_the_rules... asserts the rule's own words on all 3 runs | none |
| 10 | fail_lines eval.rs:1327-1333 | exact line `P: run f1: closed: ... (needs <= 1000000000)` (engine.rs:4413-4418) | none |
| 11 | fixtures/README.md paragraph | report item | per-figure wording not checkable here |

## The 9 decisions
1 right (strict); last-session rule needs a ruling (L1); its boundaries untested (M2). 2 right. 3 right for the
compare (cmp_wide: exhaustive small grid + wide cases), but the sum and product before it still overflow (H1).
4 right; the per-set pool untested (M3). 5 right; C4-KILL deferral worded in README 2.5. 6 right (exact line).
7 right at the leans; QWIN wording nit (L7). 8 not universal (L2). 9 tests and texts of -10 consistent (its tests use
raw counts, so they are agnostic); the -10 consequence needs the advisor's ruling (M4).
No existing expectation changed: the only removed test line (far-better `says("ws-practice: ")`) became a stronger
exact-reason assert; fixtures.rs and cli.rs are additions only.

## Findings
H1 (High) figures.rs:1282-1293 sync_cost_ratio sums every valid reference run's U as exact fractions, then
  own x (den/num) x n: with 4 valid runs in N=24 at 10-minute sizes (Sum r ~1.64e7, no common factor) Ratio::mul
  panics (ratio.rs:27), which aborts the judge for EVERY mark. A 4th valid run in a part is plausible (a re-run;
  the mean takes all valid runs, not `min`). The large-sums test (engine.rs:4137-4150) proves only 3 runs.
  Fix: take exactly the part's `min` valid runs (more: a gap naming the count, as logging_cost_ratio does with its
  off runs; marks.json's -10 notes say "the 3 runs"), or a wide mean. Tests: 4 valid N=24 runs -> the chosen outcome,
  never a panic; 3 runs per part at 60-minute sizes -> a value. Probe: probe_sync_four_valid_ten_minute_references.
M1 (Medium) engine.rs:4029-4037, :4182-4184: the -10 and R2 made-up logs are one tick with every entity in slot 0,
  and 48 equal entries on two equal ticks, so "sum over every (tick, slot)" and "mean entry" are unpinned: pb4 (slot 0
  only), pb5 (last tick only), pb6 (E = largest slot) pass all 122 tests. Code right (probes pass). Fix/test: my
  probes probe_pb4_pb5_the_unit_cost_sums_every_tick_and_every_slot (66.666667) and probe_pb6_r2s_e_is_the_mean_entry.
M2 (Medium) figures.rs:633, :702: C-U6-11's boundaries are unpinned: a later session's session_end saving a silent
  session (pb1), the last instead of the first end in a window (pb2), admit read from a client log (pb3) all pass.
  README/SERIES claim "a client-side log's record never counts" for both records; only session_end is tested.
  Fix/test: probe_pb1_..., probe_pb2_..., probe_pb3_... (te-probe-b.rs), each fails under its plant.
M3 (Medium) eval.rs:1775-1790, figures.rs:1546: condition 2's exclusions (undecided pre:, moved part, failed setup
  check) and ON-POOL-SET (this set only) have no test: pb17, pb18, pb19, pb9 survive. Tests: probe_pb9_each_set_pools_
  its_own_on_runs; on sync_mark: a reference whose pre: is undecided and one failing a setup check stay out of the
  mean (N=24 waits under min); a moved part's runs never reach Inputs.parts.
M4 (Medium, ruling) decision 9 / SERIES.md:407: with hulls in -10's denominator, U = W / (N (N-1+V)). A server
  linear in soldiers with near-free hulls reads (47/62)/(23/38) = 1.25 at N=48 vs N=24 (V = 15, ADR 0001:1476's SS)
  and FAILS 1.1; with a soldiers-only denominator an equal-cost server reads 0.80. Either choice moves -10 by more
  than its 10 % margin. Texts and tests agree with the chosen meaning; the advisor should confirm (the architect's
  open Medium) before R5's -10 runs.
L1 figures.rs:701-704: the slot's last session gets no tail even with a session_end: a -19 bot that leaves for good
  before the stop reads INVALID. Code = README = SERIES; rule, or a run-script note ("every slot's last session
  runs to the stop").
L2 decision 8 exceptions: one_shot_dropped_count sums all room logs (figures.rs:1113-1141, logs_of), logging_cost
  reads work_ns from any process (measured_work, eval.rs:267-270), alloc_counts accepts several room logs. Use
  room_log/only_from or say so in SERIES.
L3 figures.rs:214-217 stale doc ("-30's form comes with U6").
L4 figures.rs:1153-1159: raw i128 on `sec` (u64): a malformed sec >= 1.3e19 overflows x*x (panic in debug, wrap in
  release). Gap when the span exceeds a bound, or checked ops -> Gap. Test: a second at sec 2^63.
L5 figures.rs:1270: part names "N=24"/"N=36" hard-coded: a marks.json rename makes -10 NOT-RUN forever. Test on the
  real marks.json that -10 has both names.
L6 untested gaps: logging_cost (on run without work_ns; off p99 0), kill (measured p99 0; no room log),
  egress_vs_r2 (formula 0), crash (0 clients).
L7 SERIES queue_growth_count "fewer than 1,200 measured seconds" vs code's span check (figures.rs:1205).
L8 SERIES.md:420: room_end's `slot` added inside the existing row, not its own (condition 8; U6c's 51-row merge).
L9 condition 5's "(a mid-run joiner with no admit record still fails)": only a lagging one fails; a prompt one
  passes, as before C-U6-11 (no regression).
Note outside my area: every part-b commit carries "Co-Authored-By: Claude Fable 5.2" (CLAUDE.md: agents and the
main model stay Opus 5.5).

## Verdict: REVISE (H1: a crash of the whole judge on a plausible log set). With H1 fixed and the probes for M1-M3
added as tests (each shown failing under its plant), APPROVE WITH FINDINGS (M4 a ruling; Lows as follow-ups).
STATUS: COMPLETE (reviewed head a3476e6c; HEAD dc589f62 adds checkpoint files only). Probes: te-probe-b.rs (H1 probe
appended); plants: te-plant-b.mjs; scratch copy .gate/te-plant-b (reset to the worktree's src, probes appended).

## Re-check of 7c3855c7 (head 67892d39: marks.json's 5 lines and notes only; not a new full round)
- Ran: cargo test 19 + 0 + 4 + 81 + 26 = 130 pass; --check-marks 81, exit 0. Plants: te-plant-b2.mjs on the copy.
1. CLOSED: H1 (exactly `min` runs, else a gap; checked add/mul: an overflow is a gap: with(3) a value, 4/6 runs the
   count gap, min 4/6 the "does not fit" gap; my plant r6, the min gap off, fails it), M1 (pb4, pb6 fail
   u6j_the_unit_cost_and_r2s_entities...), M2 (pb3, r3 = pb1 fail c_u6_11_each_session_has_its_own_end...), M3 (pb9
   fails ..._pools_one_set_at_a_time; pb17, pb18, pb19 fail u6j_only_valid_runs_of_kept_parts...). L5, L6 tests seen.
2. Last-session rule: proven both sides. Pass: `gone` (end 6, ticks 5-6 in flight) valid. Guards: no end gives "(no
   session_end records that it left)"; past the end "(its session_end is at tick 5)"; no admit the old words.
   Plants r1 (no-admit tail), r2 (past-end allowed), r5 (session last = next admit) each fail a c_u6_11 test. README
   section 1 and the SERIES clause say what figures.rs:703-710 does (first end in the session; no admit: no tail).
3. Still passing with logic broken: r4, the (c) lookup returning the slot's FIRST session (figures.rs:657-661, new
   partition_point code), passes all 130: no test has a hole inside a later session's span. Code right: my probe
   probe_r4_a_hole_inside_a_later_session (te-probe-b.rs) passes on the head, fails under r4 (Medium; in round 1's
   suite too: I missed it). Changed expectations: large-sums now exact (stronger), mean-0 gap reworded: fine.
- New Medium (writer text): any session_end now gives the last session a tail, so an L2 writer that records one for
  every session at the stop lets a client that went silent mid-run pass (end at or past the room's last hash). L1 is
  safe (stop writes client_counts, lab scenario.rs:1818; f23 :4414). Fix: SERIES.md:418 "a session still connected
  at the stop gets no session_end"; or a judge guard (an end at the room's final tick gives no tail).
VERDICT on 67892d39: APPROVE WITH FINDINGS (two Mediums: probe_r4 as a test; the writer sentence: follow-ups).
