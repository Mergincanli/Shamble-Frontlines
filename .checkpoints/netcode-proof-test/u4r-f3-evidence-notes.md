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
