# test-engineer, round 1 review of cs1-types (head 9b8c668, code 82142fa, base main 22c6afa)

## Plan
1. Read: the diff (code only), plan.md + plan-draft.md sections 2, 5, 6; ADR 0002 marks PM-0002-15..19, -43, -63,
   migration step 4; build plan card 1, Part 3 rows, Part 7; checkpoint steps 6-7; plant.sh + results; runs20.
2. Per mark: claim (met / shown) vs evidence; name uncovered clauses.
3. Bite analysis: read each Rust and Vitest test; look for tests that cannot fail.
4. Own defects (>= 3 not in plant.sh) in worktree .gate/cs1-types/te-plant (CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/target),
   restore each file, delete planted files one by one, `git worktree remove` without --force.
5. Vitest: run by path; tsc with the scratch configs.
6. CI: what toolchain.yml runs today; the drafted CI edit's commands; what is wrong in the main session's draft.
7. Robustness: threads, --release, dirty/missing generated folder, CRLF/autocrlf, second run, scratch leftovers.

## Findings (one line each, as found)
