# security-auditor round 2: guard-wrappers, diff 5df545c..52e3c7e (.claude/hooks), what changed only

Plan (blocks): C1 read the diff; C2 rerun round-1 case files and the exhaustive pass on 52e3c7e; C3 probe the helper
check and the other tables' optional-value long options; C4 rows and mutants; C5 answer.

Notes (public folder: ids and severities only, forms in the handback):
- C1 done: diff read.
- C2 done: round-1 case files rerun on 52e3c7e: SA-a, SA-b, SA-d, SA-g, SA-h closed; SA-f rows present.
  Exhaustive pass (36582 lines, three guards): 0 throws; every drop against round 1 or main is a shape where the real
  program is not git, or git's helper now read as git with plain words after it, or SA-g's ask.
- C3 done: helper check maps names correctly, outside git's folder too; other tables' long options have no
  optional-value trap left.
- C4 done: every fix pinned by a row and a mutant.
- Findings (ids only, forms in the handback; cases in .gate/guard-wrappers/sa-r2cases.txt):
  R2-1 Medium (open in every version, missed in round 1); R2-2 Low (false asks); R2-3 Low (pre-existing, L9 area).
- Verdict: APPROVE, R2-1 as a follow-up or a small fix (his call).
DONE
