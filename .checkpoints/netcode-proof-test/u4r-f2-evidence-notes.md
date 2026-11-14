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
- prep done (main folder HEAD 14b8f10: checkpoint commits only since e0b3c7c; spikes/ unchanged). F2 working copy
  18c1dcf6..., HEAD's c5cd4d9c...; f2-limit 4fa42fe3... (one line: RUN_LIMIT_MS 3_600_000 -> 2_000, line 142).
  Shims cargo/git/npt-lab byte-equal to F1's (sha checked); npt-judge.cjs new (7eba9bac...). /dev/shm/u4r-f2/logs/sources.log.
- cmp-{precheck,P0,P2,P3,B1,r2}: all PASS (9, 20, 13, 13, 13, 9 checks; logs cmp-<set>.log). P0 carries (i) and (d);
  P3 carries (v). Precheck, P2, P3, B1, r2: argv, runs.txt/r2.txt, cargo and git calls identical masked.
- fail-measured (ii) 7/7, fail-bites (iii) 7/7, fail-both (4 b) 6/6, fail-P3 (3 e, HEAD = F2 exit line) 3/3: PASS.
- lim-measured (iv) 10/10 ("stopped at 2.0 s", call 2 ran, exit line), lim-bites 10/10: PASS.
- recheck-bites-edit (vi) 5/5, recheck-bites-commit 5/5, recheck-measured-edit (control) 4/4: PASS.
- helper: # tests 5, # pass 5, # fail 0 (exit 0): PASS.
- realjudge (cond. 5) 11/11 PASS: judge 65acacfe..., marks 9fb5be57... checked; three calls under flock -x on the lane
  lock, each exit 0; compare.py: for -2, -17, -57 parts/report/bites equal: True; tick16 and whole-as-lost shown in
  both; "RESULT: ALL EQUAL". Kept: .gate/u4r-f2/real/{verdict-combined,verdict-measured,verdict-bites}.json, compare.out.
- collect done: .gate/u4r-f2/logs/ (17 staging logs, summary.log, sources.log, edited-copies/f2-limit). The
  `rm -rf /dev/shm/u4r-f2` was DENIED by the permission system: 66 MB left on tmpfs, not worked around; for the main
  session. DONE otherwise.
