# U3d build review (netcode-architect), 2026-11-09

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_2c76ec82-8fc-1 on dae4f0f (uncommitted). Read only.
Plan: u3d-plan.md (b2e0980 + conditions 1-4); advisor-u3d.md.

## Plan
1. Read plan, advisor verdict, critics' notes (done: plan, advisor).
2. Read the diff of socket_tests.rs (self-test, start_poller, helpers, T1-T7).
3. Check against: P1/seed/reorder; rate and length; poller shape (spawned, 32 x 1,472, loop until Pending, stop
   after Pending, defined inside start_poller); window rule; pre-touch; E forms + sample set; -3 cross-check formulas;
   integer bands; verdict lines; prints before asserts; run conditions.
4. Read run.sh (U3-Q9 facts, THP, steal per ^cpu line, build once, hash, interleaving, exit status).
5. Read SERIES.md diff (condition 2 wording).
6. Anything beyond the plan.
7. Findings, DONE.

## Progress
