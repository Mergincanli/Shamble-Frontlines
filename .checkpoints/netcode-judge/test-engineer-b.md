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
