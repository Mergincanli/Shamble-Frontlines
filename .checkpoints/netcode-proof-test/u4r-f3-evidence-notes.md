# U4r-F3 evidence: notes (agent; plan first, then one line per staging with its log)

Started 2026-11-14 on claude/netcode-proof-test-plan-hcky5g at HEAD 993a5ec. F3 uncommitted in the main folder:
c-u4r-runs.mjs sha256 99dd070e... and c-u4r-r2.mjs 4e007500... (both checked at start). What to prove: u4r-f3-plan.md
section 7 as advisor-u4r-f3-plan-full.md's conditions 3-6 and 10 bind it (the evidence brief's items 1-3). On a restart:
read this file, check /dev/shm/u4r-f3/ and .gate/u4r-f3/logs/, continue at the first item without a "done" line.

## Plan
- Files (Write/Edit only): .gate/u4r-f3/harness/harness.mjs (prep, shas, run, collect) and shims/ (F2's; npt-judge
  adds a fixture write: --out verdict.json or verdict-bites.json gets the committed verdict file of the --logs set's
  profile, copied into /dev/shm/u4r-f3/fixtures at prep; and U4R_F3_JUDGE_WRITE_ON=<n> with U4R_F3_JUDGE_WRITE=notjson|
  emptyobject for (iii)). F1's r2 stagings folded in (recheck on r2 with real git; cmp-r2 as F2's).
- Sources on /dev/shm/u4r-f3/src/{head,f3,head-limit,f3-limit,f3-big}: HEAD's by git show; f3's two scripts from the
  main folder's working copy. By hand with Edit: *-limit RUN_LIMIT_MS = 2_000; f3-big BYTES_10MIN_57.P0 =
  1_450_000_000 (3 x 1.45e9 B = 4,148.5 MiB, measured call 10,011 MiB room; bites call 4,308.7 MiB, 10,386 MiB over;
  disk 8,457 MiB + 4 GiB fits /dev/shm's 16,022 MiB free, so the judge line is the stop, not the disk).
- Scratch: .gate/u4r-f3/scratch/expected_parts.py (python3 -I, the independent reading of the verdict files' fields)
  and u4r-f3-helpers.test.mjs (node --test).
- Stagings (logs/<name>.log; summary.log one PASS/FAIL line each; sources.log the sha256s):
  - basics: check (node --check x2), redact (tests with TMPDIR on /dev/shm/u4r-f3, then the scan on the spike, as
    check.mjs's redact step), gate (node tools/gate.mjs --no-smoke).
  - helper: (a)-(d) of item 2.
  - cmp-P3, cmp-B1, cmp-P2, cmp-P0: HEAD vs F3, git shim; expected F3 runs.txt built from HEAD's by inserting the
    harness's own lines (judge projection lines from RATES-1's figures, on-disk lines from its own walk, parts from
    expected_parts.py); line-for-line equal asserts both "only differences" and condition 5's order. Judge argv,
    npt-lab argv, cargo, git calls, layout, results, stdout's last line equal.
  - cmp-precheck, cmp-r2: identical masked.
  - fail-P3, fail-P0-measured, fail-P0-bites, fail-both, lim-P0 (+ lim-P0-bites): HEAD and F3 both, same mode;
    stderr word for word (seconds compared as given), runs.txt by the transform with no parts under a failed call.
  - parts-P3-notjson, parts-P3-empty, parts-P0-bites-notjson, parts-P0-bites-empty: one error line, measured parts
    present on P0, file copied, end and exit as HEAD's.
  - big-plan: f3-big on P0: exit 1 with the bites call's over-the-line line; no cargo call, no bin/runs/results.
  - recheck-P3, recheck-P0-measured, recheck-P0-bites, recheck-r2: real git, unstaged edit; F3 quotes " M spikes/...",
    HEAD's (contrast) "M spikes/...".
- collect to .gate/u4r-f3/logs/, then shas (sources.log with the harness's sha256). The harness is not edited after its
  final run (ADV-F2H-1).

## Progress
- Harness written (.gate/u4r-f3/harness/: harness.mjs; shims git/cargo/npt-lab byte-equal to F2's, npt-judge adapted);
  scratch expected_parts.py (its counts on the five files: P3 3/6/0, B1 4/9/0, P2 3/4/0, P0 3/2/0, P0 bites 0/0/6, as
  condition 6 (c)) and u4r-f3-helpers.test.mjs. prep done (HEAD 993a5ec; f3 copies = working copies 99dd070e /
  4e007500; fixtures = committed files); edited copies by Edit: head-limit, f3-limit (RUN_LIMIT_MS 2_000), f3-big (P0
  1_450_000_000).
- Trial run (not the record): 23 stagings, 22 PASS; cmp-precheck FAILED one check: the HARNESS was at fault (its "no
  judge line" check grepped /judge/ across runs.txt, which HEAD's own marks.json head line contains: "the judge read
  those copies"); fixed in the harness to look for judge projection, judge and parts lines only; then 23/23 PASS.
- Found (not a failure): the cmp's runs.txt also differs from HEAD's in the disk projection's "projected <n> MiB"
  figure (RATES-1: P3 360 -> 524, B1 545 -> 834, P2 629 -> 781, P0 3073 -> 3615), a planned change the brief's list of
  differences leaves out. The harness checks it against its own figure from condition 7's rates, not masked.
- Harness settled; trial logs cleared (rm of /dev/shm/u4r-f3/{logs,recs,stage,helper}).
- FINAL RUN (2026-11-14 23:31 UTC), harness.mjs sha256 e8516622... (unchanged before and after; never edited since):
  25 stagings, 25 PASS, 0 FAIL (`node harness.mjs run all` exit 0). Scripts run: f3 99dd070e... / 4e007500... (= the
  main folder's working copies), head c5cd4d9c... / 94564f8c...
  - basics-check 2/2 (node --check both); basics-redact 2/2 (tests 10/10; scan "287 item(s) checked, 0 finding(s)");
    basics-gate 1/1 (GATE PASS, 5a skipped: --no-smoke; base origin/main 4d21c1a).
  - helper 2/2 (node --test: 6 tests, 6 pass): (a) setParts = python3 -I reading on all five files with the counts;
    19 part/bite verdicts match u4r-report.md's one table; (b) six points within 0.1, 4,246 room / 4,247 over;
    (c) 524.4 / 833.7 / 780.8 / 3,614.6 MiB, each >= measured; P0 calls 1,727.2 / 1,887.4; (d) "reads:  M spikes/...".
  - cmp-P3 16/16, cmp-B1 16/16, cmp-P2 16/16, cmp-P0 17/17, cmp-precheck 11/11, cmp-r2 10/10.
  - fail-P3 9/9, fail-P0-measured 9/9, fail-P0-bites 9/9, fail-both 9/9, lim-P0 15/15, lim-P0-bites 15/15.
  - parts-P3-notjson 10/10, parts-P3-empty 10/10, parts-P0-bites-notjson 12/12, parts-P0-bites-empty 12/12.
  - big-plan 9/9 (bites call 4,308.7 MiB -> 10,386 MiB over; measured 4,148.5 -> 10,011 room; no cargo, no folders).
  - recheck-P3 9/9, recheck-P0-measured 9/9, recheck-P0-bites 9/9, recheck-r2 7/7 (F3 quotes " M ..."; HEAD's "M ...").
- shas then collect done: .gate/u4r-f3/logs/ (25 staging logs, summary.log, sources.log, ran-scripts.txt,
  edited-copies/{head-limit,f3-limit,f3-big}). /dev/shm/u4r-f3 left in place (stage folders; the main session may
  remove it). DONE.
