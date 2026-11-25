# docs-keeper: the docs pass for #67 (ADR 0006's small revision; lane 2)

Branch `claude/optimistic-albattani-mdoonz`, HEAD a4152ad, main 09faefa merged in (checked: `git merge-base
--is-ancestor 09faefa HEAD`). Facts checked on GitHub's REST API, 2026-11-25: #66 `merged_at` 2026-11-25T11:10:11Z,
merge commit 09faefa7f7cd1bca939bd2f98237e65e622d4737 (parents 28254a7 and d7937de), head d7937de; #67 open, draft.

## Rules that bound this pass (his words: `docs/DECISIONS.md`)
- RECORD-FILES (a): only lane 2's own lines (the preview hosting check #66, this PR #67).
- OLD-LINES (a): lines about merged PRs may move by script, byte for byte; every moved line listed below.
- REC-OLD (a): a line that records what happened or what he decided gets a dated pointer, never a rewording.
- STATUS at most 120 lines (STATUS-DIET).
- Left alone on purpose: lane 1's lines (the rows for #40 and change-set 3a, "Next" items 1-2, the "Waiting on" bullets
  for 3a and the netcode test, every lane-1 follow-up and risk); the shared lines: STATUS :5 "_Last updated" and :13
  "Last merged".

## Plan (edits in order; all by one Python script with asserts, in the scratchpad)
1. STATUS "In flight": #66's row (line 26) out; archive "## In flight" gets it at the top (before #64's row), the PR
   cell dropped, Work and Branch cells byte for byte, State cell and Waiting cell replaced (his "correct that", #64's
   precedent). #67's row in at the same place.
2. STATUS "Next" item 3: #66 merged (09faefa); #67 waiting for his go; step 2.6's plan after #67's merge.
3. STATUS "Waiting on Mustafa", lane 2's bullet: #67's go with PASS-WORDS; his word on step 4; the clean-up at his own
   timing.
4. STATUS "Follow-ups": new bullet "From ADR 0006's revision"; archive block after "From the preview hosting check"
   (texts from plan R3-7 and the review notes they cite); dated pointer on "in flight, its pull request #66".
5. STATUS "Open risks": PHC-HEADER revised by #67; archive PHC-HEADER gets a dated pointer line.
6. ROADMAP: #66's entry gets a dated pointer; new top entry for #67.
7. REPO_MAP: report row; `node .claude/scripts/repo-map-check.mjs` must print OK.
8. SPIKE_PLAN entry 6: one dated pointer after the existing one.

## Log (filled as each block lands)
