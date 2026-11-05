Task: skeptic on delta finding V-1 (verifier, minor): day cd tracking misses function/trap/alias cd (routine-guard.mjs 682-691, R-e), kit-v3-3 A3 at ed3ea8c.
Plan: B1 read rulings (advisor-fix-round R-d/R-e/L15) | B2 read code 660-720 | B3 reproduce probe in shell | B4 verdict
- B1 done: R-c/R-d/R-e/L1/L15 read (advisor-fix-round.md:41-62,129-145). B2 done: dayFolder routine-guard.mjs:672-718; no day handling of function/trap/alias (only routine CODE_RUNNERS :399). Next B3 probe.
- B3 done: reproduced (skeptic-v1-probe.mjs/.out, bash 5.2.21): one-line `function f { cd <proj>; }; f`, `trap "cd <proj>" DEBUG`, in-line alias (newline) -> allow for switch and for `echo > CLAUDE.md` after `cd /`; real bash ends in <proj>. Newline-split function and `f () {` forms ask (later cd seen / forked). Cause: in the one-line form cd is an argument of the `function` command word; trap/alias are ordinary commands to dayFolder (routine-guard.mjs:688-691); trap/alias only refused for routines (CODE_RUNNERS :399).
- Scope: L15 (advisor-fix-round.md:144) names profile only; L1 (:129) names runners, not trap/alias/function; R-e (:51-53) claims a closed set. Not covered -> in scope, stands.
- Fix test in a scratch copy of the hooks was refused by the permission system (copy of .claude); narrower fix left untested.
- B4 verdict: NOT refuted. Fix refinement: add 'trap','alias','function' to R-e's list at :690 (they act only as command words; `builtin`/`command` already unwrapped) rather than "any word", which would ask on `grep function x`.
DONE
