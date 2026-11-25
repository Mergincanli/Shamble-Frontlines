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

5. The advisor agent: APPROVE with 9 conditions (plan §7) · notes advisor-plan.md
6. Built in the copy .gate/guard-wrappers/hooks/: red 92 fails on the old guard, green 1437/1437; 87 added lines · no commit (copy is gitignored)

7. Applied: guard and tests copied over (sha256 equal to the tested copy: 7eb507c2…, c770dc59…); kit suites 1437/127/33; gate PASS --no-smoke; one commit · 5df545c

8. security-auditor round 1 on 5df545c: REVISE (SA-d Critical older; SA-a, SA-g Medium; Lows); the plugin review: High (an optional-value long option) · fix pass built in the copy: 1485/1485, 115 lines (plan §8)

9. The advisor agent on the plan change: APPROVE with conditions; SA-D-DIRECT pre-approved if Mustafa says yes · notes advisor-plan.md
10. Both versions built: .gate/guard-wrappers/hooks/ (without the direct half: 1486/1486; red 19 on 5df545c) and hooks-direct/ (with it: 1490/1490; its rows red on the other copy)

11. Mustafa (2026-11-25 20:35): SA-D-DIRECT (a); MAIN-WALL queued next (before review tracks); each kit write asked with its exact text
12. Applied hunk by hunk with Edit (each asked with its text); sha256 equal to hooks-direct (guard dc20706c…, tests e81f2bc5…); 1490/127/33; gate PASS --no-smoke; one commit · 52e3c7e · 125 added lines

13. security-auditor round 2 on 52e3c7e: APPROVE; open: R2-1 Medium (older: a program named through the input-fed wrapper's replace string passes, agents included), R2-2 Low (new false asks on git-<name> words), R2-3 Low (case/.exe, L9); the plugin's SA-g refinement (Medium) · notes security-auditor-r2.md

14. The advisor agent: R2-1 raised to Critical (an agent's kit write passes; same class as SEC-U7B-1 and SA-d): a third round on what changed allowed; the SA-g refinement may ride; R2-2 may not (follow-up) · notes advisor-plan.md
15. Round-3 copy .gate/guard-wrappers/hooks-r3/: R2-1 + SA-g refinement; green 1499/1499; red 10 on 52e3c7e; 136 added lines vs main · no commit (copy)

## Current step
Apply hooks-r3 hunk by hunk (each asked of Mustafa with its text), sha256 check, kit suites, one commit, round 3 (security-auditor, on what changed).

## Next steps
1. On APPROVE (and ≤200 added lines of code and tests): the guard edit (asks Mustafa), then the tests; push both together.
2. Kit tests (baseline 1247/127/33 on c1da1bd) and gate; security-auditor; verifier; docs-keeper; report; PR; his go.
3. Then MAIN-WALL (his message of 2026-11-25 20:35: a rule that only his account can update main, measured on a throwaway repository first; plan first, his clicks listed once; one or two change-sets with review tracks, the wall first either way; the gap's detail stays out of pushed notes; sources: .checkpoints/step-2-6-previews/ on claude/optimistic-albattani-mdoonz, questions-2026-11-25.md item 6 and security-auditor-plan.md, read with git show).
4. Then the queued change-set "review tracks and who merges" (his message of 2026-11-25 18:25; brief:
   `.checkpoints/guard-wrappers/queued-review-tracks.md`): plan first, no rule file edited before his go on the plan.

## Open findings
- The open forms (SAME-GAP, SA-R1-3) · scratchpad only · this change-set
- Interim rule until merged: git and gh always start their own command; nothing tried live; rows pushed with the fix

## Key numbers
- Kit tests on main c1da1bd's kit (= 311c0484's): routine-guard 1247, settings 127, checkpoint 33
- After 5df545c: routine-guard 1437/1437 (190 new), settings 127, checkpoint 33; added lines guard +33/-14, tests +54/-3 = 87
- Claude Code in this container: 2.1.332 (onFailure needs 2.1.295+)

## Decisions taken
- CAP (a): his line limits on guard work count code and tests only · Mustafa · 2026-11-25 17:05
- SAME-GAP (a): now, as the next change-set, from the new main · Mustafa · 2026-11-25 17:05
- SA-R2-2: find out, change nothing without his word · Mustafa · 2026-11-25 17:05
- SA-D-DIRECT (a): the direct helper check in this change-set · Mustafa · 2026-11-25 20:35
- MAIN-WALL next, before the review-tracks plan; each kit write asked with its exact text · Mustafa · 2026-11-25 20:35

## Running work
- none
