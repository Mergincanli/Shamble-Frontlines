# Advisor: MAIN-65 pushed-head check (pushed 82bd694; merge 39a0d04; branch head bae6232 at launch)
2026-11-25, from 03:5x UTC. Read only, effort xhigh. Acting by main's advisor.md (`git show
origin/main:.claude/agents/advisor.md`, read whole). Brief: main-65-step7-request.txt (sha256 b03fc992…1847ca, matches).
Measured against main-65-plan.md, advisor-main-65.md (M65-1 to M65-5, its section 3 and section 4 (d)), and the form
of advisor-main-64-head.md. In progress: plan and notes below; the verdict comes last.

## Plan (in this order)
1. Item 4 first: the precondition `git diff --quiet 39a0d04 HEAD -- . ':(exclude)spikes/' ':(exclude).checkpoints/'
   ':(exclude)docs/design/'`. If exit 0: `node tools/gate.mjs --no-smoke`, `node .claude/scripts/repo-map-check.mjs`,
   `node --test .claude/scripts/checkpoint.test.mjs`, each alone, in the main folder. Notes line the moment they end.
2. Item 2: `git log --format=%H 39a0d04..HEAD`, `git rev-list --parents 82bd694..HEAD`, and each commit's
   `git show --stat --format= <sha>`: only spikes/, .checkpoints/, docs/design/.
3. Item 1: H0, the merge and the pushed head by SHA (`git cat-file -p`); the branch's commits H0..82bd694 are the merge
   and the card save only; section 3's numbers again by SHA (names, shortstat, the anchor, STATUS.md 120 lines and the
   #40 row, the whole-tree check, spikes/ = H0's, the kit pathspec, raw modes); the record in .gate/main-65/ and the
   card's MAIN-65 line.
4. Item 3: CI on 82bd694 by SHA (check runs; the four runs by id); toolchain's summary lines (push and PR jobs) against
   main's run 45503161333, job 138831415745.
5. Verdict, findings, what I did not check, DONE.

## Notes (one line per check)
