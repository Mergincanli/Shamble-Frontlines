# infra-engineer: U4r-F1 review (one round, final diff + evidence), 2026-11-14

## Plan
1. Read plan u4r-f1-plan.md, advisor-u4r-f1-plan-full.md (conditions 1-8, Q1-Q6), rulings r2-full INF-R2-1/-3/-4, r2run-full step 7, c62h-full step 4, findings u4r-review-r2-full.md.
2. Read the diff (git diff -- spikes/netcode-proof/ops/runs), both edited scripts whole around the changes.
3. Check each item A-E + conditions 1-8 against code file:line.
4. Read evidence logs .gate/u4r-f1/logs/ (summary, sources, edited-copies, per-staging) and harness; check notes' claims against logs.
5. Re-run doubtful parts in /dev/shm/u4r-f1-review/ (scratch copies only).
6. Risks for real sets: lane lock, process groups, disk, stopped run logs, commits during a set.
7. Size: measured diff --stat vs plan/advisor.
8. Verdict + findings.

## Progress
- step 1: plan + advisor plan review read.

## Findings (one line each as found)
