# security-auditor round 3: guard-wrappers, diff 52e3c7e..9ad6bc9 (.claude/hooks), what changed only

Plan (blocks): D1 read the diff and unwrap/bash around it; D2 probe the round-3 fix over spellings (scratch script
from a copy at 9ad6bc9); D3 the new option table, ask-to-pass check; D4 differential vs 52e3c7e (throws, deny->ask/pass);
D5 rows and mutants; D6 answer.

Notes (public folder: ids and severities only; details only in the handback):
- D1 done: diff read.
- D2 done: probed (scratch only, gitignored folder). Checked against findutils' xargs.c (a GitHub mirror; the GNU
  host gave 502): the replace step skips xargs's own command word. So the shapes that count put a real command first.
- R3-1 Critical, open: R2-1 not closed for every spelling (routine-guard.mjs unwrap, the check after the loop, and
  rule U). Rebuilt shapes rerun on main, r1, r2, r3: silent on all four, session and agent. No regression.
- R3-2 Critical, open, pre-existing sibling (same area, not this diff); no ruling on it in plan.md or advisor-plan.md.
- Correction to my round 2: part of R2-1's original shapes do nothing on GNU xargs; the new rows pin those
  (harmless, conservative) plus one real shape.
- D3 done: the new table only turns an ask into a deny, never a pass; no throw path. R3-3 Low: false denies for a
  data word followed by a short option in the table, behind a program not in the read-only list. R3-4 Low (from
  memory, man git not checked): the table misses a few of git 2.43's options; those fall to ask, not pass.
- D4 done: differential 126540 lines (session and agent): 0 throws, 0 weakenings; only stricter.
- D5 done: suite 1499/1499 rerun here; the four new mutants are in-suite. R3-5 Low: no row for the =value form.
  R3-6 Low: one-letter replace strings give false asks.
- Verdict: REVISE; round 3 is the last, so the open Criticals stop the change-set for Mustafa.
DONE
