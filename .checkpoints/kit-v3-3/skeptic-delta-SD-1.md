Task: skeptic on finding SD-1 (security-auditor delta, A3 kit-v3-3): night switch check reads refs before the line runs (routine-guard.mjs:842-844, 893-928).
Plan (blocks): B1 read code at ed3ea8c (switchTargetCheck, bash()); B2 read rulings (advisor-fix-round, mustafa-words WT-NIGHT, builder-step9, NIGHT_SHIFT.md); B3 read sad-probe rows C3b/C3c and re-run own probe; B4 verdict.
- B1 done: code at ed3ea8c :842-844 calls switchTargetCheck at hook time; :918-919 resolves refs/heads then refs/remotes/origin; nothing tracks earlier commands in the line. Next B2.
- B2 done: rulings: limits L1/L4/L9/L12-L15 name no 'refs moved earlier in the line' limit; NIGHT_SHIFT.md:24-29 handoff = ls-remote then git switch <branch>; mustafa: (a) exception holds only when files match main. Not out of scope. Next B3 rerun probe.
- B3 done: reran sad-probe.mjs against the worktree guard (code identical to ed3ea8c, diff excl. .checkpoints empty) in .gate/kit-v3-3/skSD1, git 2.43: C3b null -> real lands settings {"a":3}; C3c null -> real lands {"a":2}. Control C6c (switch alone) denies, so only the in-line ref move bypasses. C3c needs no stale ref.
- B4 verdict: NOT refuted. Stands as major. Fix as proposed is sound (refuse a night switch unless every earlier command in the line is cd/pushd/popd; NIGHT_SHIFT.md:24-29 says fetch in its own call; rows C3b/C3c deny; mutant dropping the flag).
DONE
