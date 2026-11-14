# infra-engineer: U4r review round 2 (PR #40), notes

Snapshot under review: wip/u4r.patch sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce (checked 2026-11-14).
Packet u4r-r2-packet.md b1526bde... (checked), tests file 637249a5... (checked), runner u4r-run.sh 6efb9157... (checked).
Read copy: .gate/u4r-snap2 (2fbec77 + patch); compare .gate/u4r-snap1 (572c4f69).
Free space at start: 4,913,340,416 B (4.58 GiB): no builds unless a finding needs one.

## Plan
My list (ruling "Who reads what"): FP-0, FP-5 (a) to (f), INF-U4R-5, INF-U4R-10 (1) and (3), FP-8's times against budgets.
Plus my round-1 ids INF-U4R-1..11: confirm fixed / deferred as ruled with file:line in snap2.
Report words to check: INF-U4R-7 (and FP-9 list incl. INF-U4R-6, INF-U4R-10 (2)).
1. FP-0: runner diffs (u4r-run.sh, main-run.sh, .diff files), bash -n, copy 6's first log MiB line.
2. FP-5 (a)-(f) in c-u4r-runs.mjs and c-u4r-r2.mjs (snap2 vs snap1 diff).
3. INF-U4R-5 det-u4.mjs; INF-U4R-10 (1)(3) check.mjs run()/wasmStep.
4. FP-8: c6 end-check logs, det-u4 and precheck regen logs, times vs 200 s / 240 s / 300 s.
5. Report section "Fix pass, round 1: report": INF-U4R-7 words, follow-up list, each run checked in its log.
6. Helper test output (FP-5 evidence) for my items.

## Findings log (one line each as found)
