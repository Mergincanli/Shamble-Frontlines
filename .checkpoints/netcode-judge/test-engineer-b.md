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
