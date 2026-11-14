# infra-engineer: U4r-F3 review (round 1 of at most 2), 2026-11-14

Scope: the uncommitted diff to spikes/netcode-proof/ops/runs/c-u4r-runs.mjs and c-u4r-r2.mjs, against the plan
(u4r-f3-plan.md) and the advisor's ten conditions (advisor-u4r-f3-plan-full.md). Seat: runner mechanics.
Scratch: .gate/u4r-f3-review/ only. No edits to spikes/, the harness or tracked files.

## Plan
1. Read the diff, HEAD's copies, the working copies (line numbers).
2. Judge projection start check: place (after room('before the builds'), before mkdirSync(bin)), form (cond. 4).
3. judgeCall's one guard: what sits inside it; can anything throw out; is runs.txt always written (cond. 3).
4. runs.txt block order (cond. 5), exit lines, re-checks, results/, process groups unchanged.
5. INF-F1-5 keepLead in both scripts; re-check compare.
6. RATES-1 rates in start check and per-run floor (cond. 7).
7. P0 loss under each failure.
8. Evidence: summary.log, per-staging logs, sources.log vs working-copy sha256, helper test, python reading, basics.
   Re-run selected stagings and the helper test myself.
9. Verdict and findings.

## Progress (a line per block / finding)
