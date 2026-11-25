# Checkpoint: guard-wrappers
- **Goal:** SAME-GAP (a): nothing in front of git or gh gets through the day guard unread (tables for every stripped
  option-taking wrapper; leftover option words, the turn limit and unknown wrappers ask) · plan: `.checkpoints/guard-wrappers/plan.md`
  (public, no forms) and the scratchpad detail it names
- **Branch:** claude/confident-gates-tbnq38 (stands in for cs/guard-wrappers) · **Base:** main c1da1bd
- **Last update:** 2026-11-25 evening Amsterdam

## Steps done
1. #69 merged on his go (main 311c0484); branch fast-forwarded to main c1da1bd · no commit
2. SA-R2-2 answered from Claude Code's hooks reference (plan §4) · no commit
3. Plan written · f696d45
4. Plan revised before review: one table for every stripped name (keywords and `command` included), rule L folded in, the existing-row scan (one row and one mutant row change), the build-in-a-copy procedure, SA-R2-2 caveats; his review-tracks paste saved whole in the queued note · this commit

## Current step
The advisor agent's review of the plan.

## Next steps
1. On APPROVE (and ≤200 added lines of code and tests): the guard edit (asks Mustafa), then the tests; push both together.
2. Kit tests (baseline 1247/127/33 on c1da1bd) and gate; security-auditor; verifier; docs-keeper; report; PR; his go.
3. Then the queued change-set "review tracks and who merges" (his message of 2026-11-25 18:25; brief:
   `.checkpoints/guard-wrappers/queued-review-tracks.md`): plan first, no rule file edited before his go on the plan.

## Open findings
- The open forms (SAME-GAP, SA-R1-3) · scratchpad only · this change-set
- Interim rule until merged: git and gh always start their own command; nothing tried live; rows pushed with the fix

## Key numbers
- Kit tests on main c1da1bd's kit (= 311c0484's): routine-guard 1247, settings 127, checkpoint 33
- Claude Code in this container: 2.1.332 (onFailure needs 2.1.295+)

## Decisions taken
- CAP (a): his line limits on guard work count code and tests only · Mustafa · 2026-11-25 17:05
- SAME-GAP (a): now, as the next change-set, from the new main · Mustafa · 2026-11-25 17:05
- SA-R2-2: find out, change nothing without his word · Mustafa · 2026-11-25 17:05

## Running work
- none
