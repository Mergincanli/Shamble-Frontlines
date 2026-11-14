# U4r-F2 evidence: notes (agent; plan first, then one line per staging with its log)

Started 2026-11-14 on claude/netcode-proof-test-plan-hcky5g at HEAD e0b3c7c (P2's results committed at 1eacb60; F2
uncommitted in the main folder: c-u4r-runs.mjs only, 52+/22-). What to prove: u4r-f2-plan.md section 5 items 2-4,
as advisor-u4r-f2-plan-full.md's conditions 2, 3, 4 and 5 extend them. On a restart: read this file, check
/dev/shm/u4r-f2/ and .gate/u4r-f2/logs/, continue at the first staging without a "done" line.

## Plan
- Files (Write only): .gate/u4r-f2/harness/harness.mjs (prep, shas, run, collect), shims/{cargo,git,npt-lab,
  npt-judge}.cjs copied from F1's (env prefix U4R_F1_ kept, so they are byte-for-byte F1's except npt-judge.cjs, which
  adds start/end time records and a per-call fail switch U4R_F1_JUDGE_FAIL_ON=<n> (exit 1) and sleep switch
  U4R_F1_JUDGE_SLEEP_ON=<n>).
- prep: /dev/shm/u4r-f2/src/{head,f2,f2-limit}/: HEAD's c-u4r-runs.mjs and c-u4r-r2.mjs, check.mjs, loader.mjs,
  marks.json, SERIES.md, precheck-recordings.json by `git show HEAD:`; f2's c-u4r-runs.mjs from the main folder's
  working copy (c-u4r-r2.mjs HEAD's). f2-limit: an Edit by hand to RUN_LIMIT_MS = 2_000 (sha256 + one-line diff in
  sources.log).
- Stagings (logs/<name>.log, summary.log one line each):
  - cmp-{precheck,P0,P2,P3,B1,r2}: git shim; HEAD vs F2 argv and runs.txt/r2.txt compare (times masked). Cond. 4 (a).
    cmp-P0 also carries (i) (layout, two calls, --logs/--out/--marks/--only, both verdicts in results/, SPLIT_LINE
    directly above the two judge lines, raw-logs line) and (d) (judge shim times: measured first, no overlap).
    cmp-P3 is (v) (one call, <logs>/<run> layout, runs.txt identical masked).
  - fail-measured (ii), fail-bites (iii), fail-both (cond. 4 (b)): judge shim exits 1 on call n; exit line forms
    of cond. 3 (d); which verdict file is in results/ (cond. 4 (c)).
  - lim-measured (iv): f2-limit copy, judge call 1 sleeps: "stopped at <s> s (no verdict)", call 2 runs, exit line.
  - recheck-bites (vi): real git in /dev/shm scratch repo; the judge shim makes a tracked edit under
    spikes/netcode-proof during call 1; stop names "the judge (bite runs)", call 1 ran, call 2 did not, no results/.
  - helper (item 3): node --test on .checkpoints/netcode-proof-test/u4r-f2/u4r-f2-helpers.test.mjs.
  - realjudge (item 4, cond. 5): r2's p0, tick16, whole-as-lost copied to /dev/shm/u4r-f2/real/combined/{measured/p0,
    bites/tick16,bites/whole-as-lost}; three judge calls under flock; python3 -I compare
    (.checkpoints/netcode-proof-test/u4r-f2/compare.py); verdicts and compare output kept in .gate/u4r-f2/real/.
- collect to .gate/u4r-f2/logs/ (+ sources.log, summary.log), then rm -rf /dev/shm/u4r-f2.

## Progress
