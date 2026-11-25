# Checkpoint: guard-env-fix
- **Goal:** close SEC-U7B-1: the day guard's `unwrap()` reads env, nice and stdbuf options with their values; an option
  it does not know or follow asks (a subagent is refused). Tests for every form. · plan: `.checkpoints/guard-env-fix/plan.md`
- **Branch:** claude/confident-gates-tbnq38 (stands in for cs/guard-env-fix) · **Base:** main 3ab9eaec
- **Last update:** 2026-11-25 (afternoon) Amsterdam

## Steps done
1. Read CLAUDE.md, REPO_MAP, STATUS, advisor.md, CHECKPOINTS.md and the finding (git show) · no commit · gap confirmed
   by calling unwrap() alone (`probe-before.txt`)
2. Plan written · this commit · waiting for the advisor agent

## Current step
The advisor agent's review of the plan.

## Next steps
1. On APPROVE: edit `.claude/hooks/routine-guard.mjs` (WRAP_OPTS, unwrap, the bash() caller); each edit asks Mustafa.
2. Add the test rows and mutants; run the kit tests (baseline 1107/1107) and `node tools/gate.mjs`.
3. security-auditor on the final diff; the verifier; docs-keeper; `/advisor-report`; the full commit id to Mustafa.

## Open findings
- SEC-U7B-1 · security-u7-0b-answer.md · open (this change-set)
- Same-class gaps outside the scope · this session · reported to Mustafa, not fixed (forms kept out of the repo)

## Key numbers
- routine-guard tests on main's guard: 1107 tests, 1107 pass, 0 fail, 34 s (node --test, 3ab9eaec's guard files)

## Decisions taken
- The 150-line cap read as the guard and test files; docs and report counted separately · this session · open for Mustafa

## Running work
- none
