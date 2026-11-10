# U4c review, rust-reviewer (read only)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-2 (uncommitted), base 097f8970.
Plan: u4-build-plan.md §5 U4c (:552-583), §4.1 (npt-l1 row); advisor-u4.md; advisor-u3q.md:160-173 (C1'); advisor-u2.md (C2).

## Plan
1. git status / diff stat in the worktree; confirm only crates/l1 changed; sha256 of the three files vs the report.
2. Read lib.rs whole: model, event queue, ordering, integer time math, allocation, panics, public items, docs.
3. Read tests.rs + tests/l1.rs: L-c1..L-c9 coverage vs the plan; bites per instance; cfg gating.
4. Determinism: RNG streams (ChaCha8 per flow), event order (time, fate index), no HashMap order, floats.
5. Arithmetic: overflow, saturating vs checked, PTO doubling shift overflow, division.
6. Findings -> verdict.

## Progress
