# U4j review, round 1: test-engineer (is it proven?)

Reviewer: test-engineer (opus). Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50,
base 411eeeed50d8c71e7e1f74366f0e1d5c27388444. Started 2026-11-11. No local memory; no earlier notes file.

## Plan
1. Record sha256 of every changed and new file (git status --untracked-files=all; git diff --stat) before anything.
2. Read the plan §5 U4j (u4-build-plan.md:626-646), §4.6, §4.12, the advisor's conditions 2, 4, nits c, f, Q20, Q22,
   W1-N2; the two reviewers' notes (netcode, security).
3. Read the diff: log/src/lib.rs, judge marks.rs, eval.rs, figures.rs, practice.rs, tests/engine.rs, tests/fixtures.rs,
   fixtures, marks.json, README, SERIES.
4. Map each §5 U4j test/fixture to an existing test; judge power; claim · proof · gap table.
5. Plants (in place, one at a time, Edit, restore, sha256 check): for tests whose power I doubt; and to settle the
   other reviewers' findings a run can settle.
6. Check builder's claims: P1-P4, counts (+18), A19 pub items.
7. check.mjs once at the end (idle wait, touch loop, provenance check, counts 304/327/48).
8. Final sha256 identity check. Verdict.

## Findings (a line after each)
