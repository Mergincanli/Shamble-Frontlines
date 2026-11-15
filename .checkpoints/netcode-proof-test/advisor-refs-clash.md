# advisor: refs clash (check.mjs referenceProblems vs PR #53's kept-logs kit files)

## Plan
1. Read advisor.md from main (done), the two kit files on origin/main, CHECKPOINTS.md section 11, check.mjs referenceProblems
   and its tests, plan.md 4.2 and 4.9, history.md 2131-2175, c62h-bound-freeze.txt.
2. Q1: purpose vs letter: what each kit file needs from spikes/.
3. Reproduce the failure on a scratch worktree under .gate/ (merge origin/main, no commit), run --only refs.
4. Q2: weigh fixes (a) exact two-path skip, (b) broader skip, (c) others; write exact old/new lines; test the fix in the
   scratch worktree (the test plus the planted bug).
5. Q3: Mustafa's word? Q4: order. Q5: deletion follow-up.
6. Return structured output.

## Notes
