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
- Read: git diff (11 files + 22 untracked), log/src/lib.rs, judge marks.rs, eval.rs, figures.rs, practice.rs, ratio.rs;
  u4j-build.md; u4j-brief.md; u4-build-plan §5 U4j, §2 C4 row, §4.6, §4.12; advisor-u2 C4 (:101-103);
  advisor-ws-docs-r2 :375-378, R7 :397, R8 :398; ADR 0001 -18 row :1608; marks.json -18, -21, -30; README diff.
- F1 HIGH practice.rs:386-406 + parse_bound :87-90: FAR BETTER branch caps nothing. class_ok "FAR BETTER" needs only a
  chance_ruling and s < band_low; parse_bound accepts any band_low in (0, band_high] (no band_low <= 1 <= band_high),
  then f = max(1, s). Bound line band_low "2.0"/band_high "2.0", class FAR BETTER, any non-empty chance_ruling, M_p =
  1.9 x M_m -> f = 1.9: X/Y/Z moved 90 % past Mustafa's 10 % (R8 (v): "FAR BETTER: at the bound"). Fix: FAR BETTER
  forces f = 1; parse_bound refuses band_low > 1 or band_high < 1; assert f <= tolerance before Ok. Proof: engine test
  (the ws-practice PASS case with that bound line and class) must come out INVALID. Before merge.
