# Advisor: check of c49773f (TE-M3's cause and fix), notes

Task: the check my TE-M3 ruling asked for (advisor-te-m3.md, TE-Q4 and "What the main session does next" item 7).
Effort xhigh. Read-only on source; notes and ruling via Write/Edit only.

## Plan
1. Read advisor.md (main) - done. Read my TE-M3 ruling (advisor-te-m3.md + its notes) and te-m3-plan.md section 7.
2. Read the diff `git show c49773f`; check it against the plan (symbols, scope, files: only npt-net's test?).
3. Check conditions 1-7 against the record and the logs in .gate/te-m3/ (p2 tally, p3, p4a/p4b + run logs, p5-check,
   p6-gate).
4. Check CI: runs 43428174558 (push) and 43428182600 (pull_request) on c49773f (gh, read only).
5. Run the fixed test myself under the same load (main-run.sh ... te_m3_loop.sh adv-check one 150 2 6); wait for the
   other advisor's lock if held; tally; any failure kept whole, ruled on, never retried.
6. Write the ruling advisor-te-m3-check.md; hand back.

## Log
