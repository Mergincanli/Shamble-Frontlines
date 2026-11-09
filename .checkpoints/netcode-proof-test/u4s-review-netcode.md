# U4s review: netcode-architect (read-only)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1 (uncommitted; base c8abc857)
Plan: u4-build-plan.md §4.1-§4.4, §4.11, §5 U4s (:492-508); advisor-u4.md conditions 1, 10(a)/(b).

## Plan
1. git status / git diff --stat in the worktree; confirm base and file set.
2. seam.rs vs §4.2 + ADR 0004 (step signature, HitHistory lent &, integer Rewind, reports() outside hash, no World).
3. history.rs vs §4.2/§4.9 (fixed ring, capacity from ticks_for_ms, eviction, get by tick).
4. ticks.rs vs §4.3 (TICK_HZ 30, LOOP_CATCHUP TICK_HZ/4, ticks_for_ms ceil, derived constants, 30/60 values).
5. grid.rs vs §4.4 (1 cm, yaw 16, pitch 10 over ±90, tie rule, error bound, config value not static).
6. tests/seam.rs s1-s5 + bites; check.mjs greps (tick, float).
7. Manifests vs U4s list + condition 10; nothing of U4a-U4h pulled forward.
8. Findings + verdict.

## Notes (one line per finding as found)
