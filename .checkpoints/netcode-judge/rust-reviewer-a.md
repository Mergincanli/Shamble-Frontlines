# rust-reviewer, part a (MOVED) review round, diff e2c0e896..b22c30cd

## Plan
1. Read plan-a.md sections 2 and 6, marks/README.md 2.6, 3, 5, 6.
2. Read the full diff of crates/judge (marks.rs, eval.rs, lib.rs, tests).
3. Read surrounding unchanged code: eval.rs judge_mark, eval_part, eval_bite, judge_run, Rule::Pre, report_values,
   matched_parts, applies; marks.rs compile_mark, compile_check, compile_report, parse; practice.rs, figures.rs.
4. Grep every match over VerdictKind/BiteStatus and every iteration of parts/checks/bites.
5. Run clippy, fmt --check, tests, Cargo diff stat.
6. Write findings here, one line each, then hand back.

## Findings (one line each, as found)
