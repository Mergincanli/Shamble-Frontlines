# U4j review, round 1: netcode-architect (read only)

Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50, base 411eeeed50d8c71e7e1f74366f0e1d5c27388444.
Started 2026-11-11. No memory found; no earlier notes file.

## Plan
1. Read the sources: u4-build-plan.md §4.6, §4.12, §5 U4j (:626-646), §2 rows (:147-154); advisor-u4.md conditions
   2/4, nits c/f, Q20, Q22, U4S-BUILDER; advisor-u4-wave1.md W1-N2; plan.md PM rows for -17, -18, -20, -21, -22, -58;
   advisor-ws-docs-r2.md :373-383 (bf9d248's -18 numbers, ws-practice); .claude/rules/netcode.md; base README/SERIES.
2. Read the diff: log/src/lib.rs (hs), judge figures.rs (five figures), eval.rs (Empty, C4, ws_bound), marks.rs,
   practice.rs, marks.json changes, README/SERIES deltas, fixtures.
3. Check focus (1): the five figures against §4.6 and SERIES (pairing, complete ticks, sentinel, null, orphan, empty,
   server 0, twin per tick, cross-arch hash kind, machine_named).
4. Check focus (2): marks.json changes against PM rows and §4.12 (selects, -18 split, ws_bound numbers, WS-BAR cap,
   -22 exception, -58 W1-N2 text); no bar loosened.
5. Check focus (3): readings (a), (b), (d), (e), (f).
6. Check focus (4): writer rules for U4r and U4c L-c8 re-judge: complete and unambiguous; any rule a runner can meet
   while the figure misjudges.
7. Findings, one line each below as found; then the verdict.

## Findings (one line each, as found)
