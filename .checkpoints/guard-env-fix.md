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

5. security-auditor round 1 on 21615cb: REVISE (SA-R1-2 Medium: env -S turned a deny into an ask; Lows SA-R1-1, -3, -4) · notes security-auditor-r1.md
6. Fix pass: SA-R1-2, SA-R1-1, SA-R1-4 fixed; SA-R1-3 deferred (pre-existing loop cap, outside scope) · 11887d0 · 1244/1244; reviewer's 10 extra mutants all caught; gate PASS --no-smoke

7. security-auditor round 2 on 11887d0: APPROVE; Lows SA-R2-1 (3 rows, ridden: c3827f4), SA-R2-2 (long lines vs the hook timeout, same class on main: follow-up), nit (shownWord leaves U+2028/9 and bidi controls; no tab row: follow-up) · notes security-auditor-r2.md
8. The automated plugin review flagged env -S's blank split vs env's own parse (Medium): checked main vs head with unwrap() alone (scratchpad split-check.mjs): where they differ main gave no check, head asks; no deny→ask versus main. Kept (ask, per the approved plan); in the report.

## Current step
The verifier on c3827f4.

## Next steps
1. The verifier on c3827f4 (the final code head).
3. Line budget: code+tests +76/-6; docs-keeper and the report must keep the total ≤150 added (strict reading) or stop and ask Mustafa.
4. docs-keeper; `/advisor-report` (lean); fold the checkpoint and probes into it, delete them; PR (draft → ready) or its text in notes; the full commit id to Mustafa.

## Open findings
- SEC-U7B-1 · security-u7-0b-answer.md · fixed in 21615cb + 11887d0 (pending round 2)
- SA-R1-2 (Medium) · fixed 11887d0 · SA-R1-1, SA-R1-4 (Low) · fixed 11887d0
- SA-R1-3 (Low, on main too: unwrap()'s 20-turn cap) · deferred: STATUS follow-up, worded without forms
- SA-R2-1 (Low) · fixed c3827f4 · SA-R2-2 (Low, on main too: long lines vs hook timeout) · follow-up · SR2 nit (shownWord: U+2028/9, bidi; tab row) · follow-up
- Same-class gaps outside the scope · this session · reported to Mustafa, not fixed (forms kept out of the repo)

## Key numbers
- added lines so far: guard +47/-6, tests +47/-0 = +94 (git diff --numstat origin/main, c3827f4); docs+report must stay ≤56 under the strict reading
- routine-guard tests after: 1237/1237 (21615cb); 1244/1244 (11887d0); 1247/1247 (c3827f4)
- routine-guard tests on main's guard: 1107 tests, 1107 pass, 0 fail, 34 s (node --test, 3ab9eaec's guard files)
- settings.test.mjs 127/127; checkpoint.test.mjs 33/33 (3ab9eaec's kit)

## Decisions taken
- Plan APPROVED with 7 conditions · the advisor agent · 2026-11-25
- CAP (the 150-line cap: code files only, or docs and report too) · open for Mustafa; until then the strict reading
- SAME-GAP (fix timeout, xvfb-run, time, exec too) · open for Mustafa; not fixed here

## Running work
- none
