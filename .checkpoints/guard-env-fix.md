# Checkpoint: guard-env-fix
- **Goal:** close SEC-U7B-1: the day guard's `unwrap()` reads env, nice and stdbuf options with their values; an option
  it does not know or follow asks (a subagent is refused). Tests for every form. · plan: `.checkpoints/guard-env-fix/plan.md`
- **Branch:** claude/confident-gates-tbnq38 (stands in for cs/guard-env-fix) · **Base:** main 3ab9eaec
- **Last update:** 2026-11-25 (afternoon) Amsterdam

## Steps done
1. Read CLAUDE.md, REPO_MAP, STATUS, advisor.md, CHECKPOINTS.md and the finding (git show) · no commit · gap confirmed
   by calling unwrap() alone (`probe-before.txt`)
2. Plan written · 20902d9 · the advisor agent: APPROVE with 7 conditions (plan §8)
3. Baselines before the edit: settings.test 127/127, checkpoint.test 33/33 · c11b68f
4. Mustafa's "Go" (CAP and SAME-GAP not answered: strict cap reading kept, other wrappers left alone) · the edit: WRAP_OPTS (a Map), unwrap() → { argv, unreadable }, the bash() ask/agent deny; 130 test rows, 11 mutants · 21615cb · guard 1237/1237, settings 127/127, checkpoint 33/33; gate PASS --no-smoke (smoke can't start: Playwright 1.56.1's Chromium build missing here); probe-after.txt

## Current step
security-auditor's review of the final diff (round 1).

## Next steps
1. security-auditor on `git diff origin/main...21615cb` (give it the scratchpad out-of-scope notes; ask whether the forms and mutants are enough).
2. At most one fix pass, then a second round on what changed; then the verifier.
3. Line budget: code+tests +76/-6; docs-keeper and the report must keep the total ≤150 added (strict reading) or stop and ask Mustafa.
4. docs-keeper; `/advisor-report` (lean); fold the checkpoint and probes into it, delete them; PR (draft → ready) or its text in notes; the full commit id to Mustafa.

## Open findings
- SEC-U7B-1 · security-u7-0b-answer.md · open (this change-set)
- Same-class gaps outside the scope · this session · reported to Mustafa, not fixed (forms kept out of the repo)

## Key numbers
- added lines so far: guard +38/-6, tests +38/-0 (git diff --numstat origin/main)
- routine-guard tests after: 1237/1237 (21615cb)
- routine-guard tests on main's guard: 1107 tests, 1107 pass, 0 fail, 34 s (node --test, 3ab9eaec's guard files)
- settings.test.mjs 127/127; checkpoint.test.mjs 33/33 (3ab9eaec's kit)

## Decisions taken
- Plan APPROVED with 7 conditions · the advisor agent · 2026-11-25
- CAP (the 150-line cap: code files only, or docs and report too) · open for Mustafa; until then the strict reading
- SAME-GAP (fix timeout, xvfb-run, time, exec too) · open for Mustafa; not fixed here

## Running work
- none
