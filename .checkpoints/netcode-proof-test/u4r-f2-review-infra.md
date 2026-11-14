# infra-engineer: U4r-F2 review (round 1 of at most 2), notes

Started 2026-11-14. Target: the uncommitted diff to spikes/netcode-proof/ops/runs/c-u4r-runs.mjs (HEAD 6498d97).

## Plan
1. Read the plan (u4r-f2-plan.md), the advisor's plan review with its nine conditions (advisor-u4r-f2-plan-full.md),
   the Q2 ruling (advisor-u4r-sets-r1-full.md), context (u4r-f1-review-full.md, the card's run procedure).
2. Read the runner whole (current copy) with line numbers: order of re-checks, the two calls, results/, run() in
   check.mjs (condition 1's endings), exit lines, lock, process groups, timeouts, layout, exists-check.
3. Read the harness (.gate/u4r-f2/harness/) and its logs; re-run stagings myself; run the scratch helper test.
4. Map each of conditions 2-5 to evidence; note gaps.
5. P0 70-minute set: what is lost under each failure (judge call 1 fails, call 2 fails, both, timeout, kill, disk).
6. Findings INF-F2-n, verdict.

## Findings (one line each, as found)

