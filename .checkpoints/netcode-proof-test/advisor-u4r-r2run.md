# Advisor: ruling on U4r's r2 record run (step 11) and carry 62 (h) (step 12), 2026-11-14

## Plan
1. Read advisor.md from main (done), the r2 ruling steps 11-12 (done), the merge review, build3/build4 rulings
   (points 1-4 and 8), carry 62 in u4s-carries.md.
2. Check the run's provenance: r2.txt's head (commit e59ece9, sha256s of binaries, marks.json, SERIES.md copies),
   that spikes/ is the same at 71ee13d and e59ece9 (git diff --quiet), that e59ece9 descends from 71ee13d with only
   .checkpoints/ changes; the runner log r2.log (exit 0, times, no 'stopped at' or 'exit by a signal').
3. Read c-u4r-r2.mjs: the suites, their commands, the expected lines, how measured incomplete is summed.
4. Per suite: compare what came against its expected line (r2.txt + verdict JSONs + logs), one notes line each.
5. Step 12: identify the "clean short L1 P0 suites", sum tick.s.snap_incomplete over each measured phase from the
   logs myself (python3 -I print-only reads), and rule on lag-switch's 300 from its design and logs.
6. Rule next steps, the marks.json change and the npt-judge --only PM-0001-17 check, or the cause first, and with whom.
7. What goes to Mustafa.

## Notes (a line per suite as I go)
