# Checkpoint: guard-env-fix
- **Goal:** close SEC-U7B-1: the day guard's `unwrap()` reads env, nice and stdbuf options with their values; an option
  it does not know or follow asks (a subagent is refused). Tests for every form. · plan: `.checkpoints/guard-env-fix/plan.md`
- **Branch:** claude/confident-gates-tbnq38 (stands in for cs/guard-env-fix) · **Base:** main 3ab9eaec
- **Last update:** 2026-11-25 (afternoon) Amsterdam

## Steps done
1. Read CLAUDE.md, REPO_MAP, STATUS, advisor.md, CHECKPOINTS.md and the finding (git show) · no commit · gap confirmed
   by calling unwrap() alone (`probe-before.txt`)
2. Plan written · 20902d9 · the advisor agent: APPROVE with 7 conditions (plan §8)
3. Baselines before the edit: settings.test 127/127, checkpoint.test 33/33 · this commit

## Current step
Stopped for Mustafa (his brief: "start with the plan and stop for its review"); CAP open.

## Next steps
1. On Mustafa's word to go on: edit `.claude/hooks/routine-guard.mjs` (WRAP_OPTS, unwrap, the bash() caller); each edit asks Mustafa.
2. Add the test rows and mutants; run the kit tests (baseline 1107/1107) and `node tools/gate.mjs`.
3. security-auditor on the final diff; the verifier; docs-keeper; `/advisor-report`; the full commit id to Mustafa.

## Open findings
- SEC-U7B-1 · security-u7-0b-answer.md · open (this change-set)
- Same-class gaps outside the scope · this session · reported to Mustafa, not fixed (forms kept out of the repo)

## Key numbers
- routine-guard tests on main's guard: 1107 tests, 1107 pass, 0 fail, 34 s (node --test, 3ab9eaec's guard files)
- settings.test.mjs 127/127; checkpoint.test.mjs 33/33 (3ab9eaec's kit)

## Decisions taken
- Plan APPROVED with 7 conditions · the advisor agent · 2026-11-25
- CAP (the 150-line cap: code files only, or docs and report too) · open for Mustafa; until then the strict reading

## Running work
- none
