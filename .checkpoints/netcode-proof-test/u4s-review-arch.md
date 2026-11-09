# engine-architect: U4s build review (module boundaries)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1 (uncommitted; read only)
Plan: u4-build-plan.md §4.1-§4.4, §4.11, §5 U4s; advisor-u4.md (condition 10).

## Plan
1. git status / git diff in the worktree; list changed files.
2. Manifests: the four new edges vs ADR 0002:186-191 and plan.md §4.2; no other edge; bites features and npt-lab
   forwarding (condition 10 a/b); npt-server dev-dep on npt-client; Cargo.lock delta.
3. npt-sim stays free of everything above it (deps, uses).
4. Public items vs §4.1 and plan.md §4.7 (incl. the five extra types).
5. check.mjs greps: tickConstantProblems (§4.3) and roomFloatProblems (§4.11), wired into the deps step; self-tests.
6. Boundary sanity: seam signatures vs §4.2 (step, reports, Rewind, HitHistory), state ownership.
7. Findings + verdict.

## Notes (one line per finding as found)
