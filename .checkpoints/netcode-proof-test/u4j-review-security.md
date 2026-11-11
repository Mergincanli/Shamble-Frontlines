# U4j review, round 1: security-auditor (validity rules)

Reviewer: security-auditor (opus). Read only: no edit, build or run in the worktree
/home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50 (base 411eeeed50d8c71e7e1f74366f0e1d5c27388444).
Started 2026-11-11. No earlier notes file (fresh run). No agent memory folder found (memory: local; none on disk).

## Plan
1. Read the diff (git diff base, git status untracked) and the files: log/src/lib.rs (hs), judge marks.rs, eval.rs,
   figures.rs, practice.rs, lib.rs; tests engine.rs and fixtures.rs only as needed; marks.json diff; README/SERIES diff.
2. Read the plan's rules: u4-build-plan.md §5 U4j (:626-646), §4.6, §4.12, §2 rows (:147-154); advisor-u4.md
   conditions 2/4, nits c/f, Q20, Q22, :365-371; advisor-u4-wave1.md W1-N2; advisor-ws-docs-r2.md :373-383 (ws-practice).
3. Focus items, one block each, a notes line per finding:
   (1) C4 rules and order; mixed-build hiding a bites log behind an honest one (reading a).
   (2) hs format: writer and reader, 16 lowercase hex or null, sentinel, duplicate / out-of-order ticks, uneven lengths.
   (3) Outcome::Empty and reading (d).
   (4) ws-practice: f bound by tolerance? WS-BAR cap; second practice run without a ruling; can a log supply a ruling.
   (5) reading (e): aarch64 claim dodging a check.
   (6) hostile input on parse_bound, practice record, bound line, hs: panics, overflow, NaN/inf, duplicate keys,
       trailing data.
   (7) redact check and marks drift test unweakened.
4. Anti-cheat-style coverage table for this unit: input -> where the judge validates -> gap.
5. Verdict and findings to StructuredOutput.

## Progress
