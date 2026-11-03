# Checkpoint: kit-v3-1

- **Goal:** A1 (Mustafa, 2026-11-03): PR #30 onto main, its conflicts resolved, the GUARD-FILE (a) fix inside it,
  re-reviewed (security-auditor on the fix, then the advisor on the whole PR), then wait for Mustafa's go · plan:
  `.checkpoints/kit-v3-1/a1-plan.md` (revision 2) · verdict: pending
- **Branch:** `cs/kit-v3-1` (PR #30), head `4def8d2` · **Base:** origin/main `e4d8b22`
- **Session:** main folder `C:\dev\shamble-fresh` stays on `main`; build in worktree `<scratchpad>/kit-v3-1`
- **Last update:** 2026-11-04 (see commit time) Amsterdam

## Steps done
1. Context recovered (this session): REPO_MAP, STATUS, CHECKPOINTS read; the frozen session's plan revision 1 and
   checkpoint on the branch read; Mustafa's task (A1, A2, B1, B2) and the GUARD-FILE (a) + working-rule decisions.
2. Verified against live code: conflicts are exactly STATUS/ROADMAP/REPO_MAP (`git merge-tree`); `.claude/` + CLAUDE.md
   merge cleanly (main untouched since 07ab240); guard.mjs already fails safe on a missing *rules* file but not on
   itself missing (the freeze); main's settings.json has no guard hook (this session is safe).
3. BOOT mechanism validated in a scratch harness on Node v24.17.0 (no repo file touched): {present,missing,broken} ×
   {day,night-shift,pr-advisor,unknown} → exactly the intended exits. Chose the `require`-free promise-only BOOT.
4. Found + diagnosed Q3: `routine-guard.test.mjs` is 183/192 on this Windows machine — 9 pre-existing failures from
   forward-slash-only credential regexes vs Windows backslash paths; `routine-guard.mjs` untouched by A1; Linux/CI
   path is 192/192; CI does not run the hook tests.
5. Worktree created at `<scratchpad>/kit-v3-1` (local `cs/kit-v3-1` tracks origin). Plan revision 2 written.

## Current step
Advisor review of plan revision 2 (next action: launch the `advisor` agent).

## Next steps
1. Advisor verdict → record verbatim here. APPROVE → plan step 1 (merge origin/main e4d8b22, take main's side in the
   three docs' conflict hunks); REVISE → fix the plan; ESCALATE → ask Mustafa. Also get the advisor's calls on Q1, Q2, Q3.

## Open findings
- Q3 (new): 9 pre-existing Windows-only failures in `routine-guard.test.mjs` (path-separator root cause); recommend
  documenting as a follow-up, not fixing in A1. Status: open, for the advisor.

## Key numbers
- settings.test.mjs: 82/82 (this machine, Windows/Node24).
- routine-guard.test.mjs: 183/192 (Windows/Node24; 9 pre-existing, Q3) · 192/192 on Linux per #30's report.
- BOOT matrix: all cases as intended (Node v24.17.0).

## Decisions taken
- GUARD-FILE (a): fix inside #30 as part of A1 · Mustafa · 2026-11-03
- Working rule: the main folder is never switched; other branches go in a worktree · Mustafa · 2026-11-03
- Pushes for A1 go to `cs/kit-v3-1`; the worktree is `<scratchpad>/kit-v3-1`, outside the project · this session
- BOOT uses the promise-only `import('node:url')` form (require-free, CJS/ESM-safe) · this session · 2026-11-03

## Running work
- none
