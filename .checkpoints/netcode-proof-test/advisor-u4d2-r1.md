# The advisor's ruling on U4d2's round 1 (2026-11-12, from advisor-u4d2-r1-brief.md)

Seat: the advisor agent, run by the main session's workflow, working from main's `.claude/agents/advisor.md`
(read at origin/main). It rules; it edits no code and runs nothing that builds.

## Plan
1. Read the five seats' notes whole (rust, security, infra, arch, test), then the builder's record (claims only).
2. Read my own rulings that bind the form: advisor-u4d2-brief.md (L1 to L5, the conditions), advisor-u4e-r1.md and
   advisor-u4e-r2.md (M1 to M9's form).
3. Read the rules cited by place: CLAUDE.md's review budget (in context), u4-build-plan.md §7.1 items 1 and 7,
   u4s-carries.md entry 52 (the 9.5 floor), §5 U4d2, B2.3 / TE2-5 in u4d2-brief.md, carry 65.
4. Spot-check the load-bearing claims against the copy (`.gate/u4d2-r1`, the worktree's diff) and the seats' logs
   (`.gate/u4d2/r1-*`): RR-M2's run, SEC-U4D2-1's run, SEC-U4D2-7's run, TE-M3's logs, the counts.
5. Rule, one block per item (1 to 9 of the brief), each written to this file as it is finished.
6. End with the two lists (before the merge; for Mustafa) and the summary for the caller.

## Notes (one line per check, as done)
- Read whole: advisor.md (origin/main, 334 lines); the five seats' notes (rust 124, security 172, infra 150, arch 112,
  test 266 lines); the builder's record u4d2-build.md (256 lines, its claims).
- Read whole: my brief check advisor-u4d2-brief.md (756 lines: A1-A15, Q1-Q12, L1-L5; Q4 (a) chose the superset,
  "fail-closed"; A15 found the superset's first cost; block 7 set U4d2's review in batches).
- Read the form: advisor-u4e-r1.md R8 (fix pass, its builder, round 2's seats, the outcome rule) and R9 (M1-M9);
  advisor-u4e-r2.md Q3 (M1-M9 as amended), Q4 (background run, >= 1,500,000 ms), "Before the merge".
