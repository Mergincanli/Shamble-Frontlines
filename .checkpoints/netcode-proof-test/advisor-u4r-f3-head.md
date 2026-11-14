# Advisor: review of U4r-F3's pushed head (493b3d7), 2026-11-14

Notes as I go. Plan first; one line per check. On a restart: read this, continue from the last line.

## Plan
1. Head and branch: 493b3d7 is HEAD; commits after it touch only .checkpoints/; files changed by 493b3d7.
2. sha256 of the committed scripts at 493b3d7 vs the tested ones (99dd070e runs, 4e007500 r2) and vs the logs' records.
3. Read the diff (both scripts) whole.
4. Conditions 1 to 10, each in code and in evidence.
5. The two readings (cond 3 one guard as one helper used twice; cond 4 on-disk line ending "over the line, printed only (...)").
6. Size: 157/34 vs 100-130 / 20-30.
7. Evidence: summary.log, sources.log, ran-scripts.txt, staging logs, basics logs, edited-copies, harness, scratch helper test + independent reading.
8. Evidence agent's notes: spot-check claims; the c5cd4d9c slip.
9. infra-engineer's round: INF-F3-1, INF-F3-2 (the reviewer's logbytes-check.mjs).
10. CI on 493b3d7: gate run 43654541605 (gate, smoke), toolchain 43654541555.
11. Follow-ups: INF-F3-1, INF-F1-3 waiting for check.mjs follow-up.
12. Verdict, next steps, Mustafa.

## Checks
- C1 head: origin branch tip 7e5ecaa; 493b3d7..7e5ecaa touches only .checkpoints/ (card, this notes file, a writer .py). 493b3d7 touches 4 .checkpoints files + the two scripts. PASS.
- C2 sha256 at 493b3d7: c-u4r-runs.mjs 99dd070e... (608 lines; parent 18c1dcf6..., 486), c-u4r-r2.mjs 4e007500... (236 lines; parent 94564f8c..., 235). Worktree equal. Match the tested ones. PASS (logs' records checked below).
- C3 size: numstat runs 153/31, r2 4/3; total 157/34.
- C4 tested copy: sources.log: f3/ copies 99dd070e / 4e007500, equal to the working copies at 23:31:59; head/ copies 18c1dcf6 / 94564f8c (= 493b3d7^). All 25 stagings' f3 runs name 99dd070e / 4e007500 (or the two edited copies). Edited copies each differ by exactly one line from the committed script (diff by me: f3-big :152 P0 rate 1.45e9; f3-limit :171 RUN_LIMIT_MS 2000; head-limit :142 from 18c1dcf6). Harness.mjs + 4 shims + 2 scratch files: sha256 now = sources.log's; harness mtime 23:30:00 before the run at 23:31. PASS (ADV-F2H-1 held).
- C5 read the diff whole (both scripts). Notes on each condition below.
- C6 basics: basics-check (node --check both, sha 99dd070e/4e007500, exit 0 no output), basics-redact (tests 10/10, scan 287 items 0 findings), basics-gate (GATE PASS --no-smoke, 5a skipped). Logs kept. PASS (cond 10 pre-commit part).
- C7 stagings read (25/25 PASS in summary.log): cmp-P3/B1/P2/P0 (order per cond 5 seen in runs.txt; P0 SPLIT_LINE then measured block then bites block; parts 3+2 / 6 bites), cmp-precheck (0 differing of 28, no judge lines), cmp-r2 (0 of 177), fail-P3 (exit line word for word, no parts), fail-P0-*/fail-both/lim-P0/lim-P0-bites (no parts under failed call; other call's present; exit lines HEAD's), parts-P3/P0-bites notjson + {} (one error line each, P0 measured parts present, file copied, end/exit HEAD's), big-plan (measured 4148.5 MiB room, bites 4308.7 MiB -> 10386 over; exit 1; no cargo, no bin/logs/results, nothing under .gate/), recheck-P3/P0-measured/P0-bites/r2 (HEAD trims, F3 keeps " M"; between-call stop: call 1 ran, no results/).
- C8 helper + independent reading: expected_parts.py reads fields only (python3 -I), no import of the runner. helper.log: counts P3 3/6, B1 4/9, P2 3/4, P0 3/2/0, P0-bites 0/0/6; 19 verdicts vs report table; six fit points; 4246 room / 4247 over (both endings); projections 524.4/833.7/780.8/3614.6; untrimmed quote. My own python3 -I count of the 5 committed files: identical counts and verdicts. PASS cond 6 (c).
- C9 du -sb by me of all kept run folders: every figure in the RATES-1 comment holds (P3 178.9-183.26, B1 283.1-291.32, P2 271.2-272.82, P0 meas 514.0-520.3, bites 518.8/567.5/603.63, -17 bites 55.60-55.84). Rates = largest rounded up to 0.1 MB. Folder names exist. PASS cond 7 (code).
- C10 projections by hand: P0 3,790.2e6 B = 3614.6 MiB; measured 1727.2 -> 4344; bites 1887.4 -> 4719; line at 4246.4 MiB; big-plan 4148.5 room / 4308.7 over. PASS.
- C11 FINDING candidate ADV-F3H-1 (Low): RATES-1 comment says "the builder's rates from 20 to 30 s runs understated them"; u4r-build.md:739-751: the rates used (P0 509.1, P2 219.8, P3 125.8) came from d13 runs of 1,500 measured ticks = 50 s at 30 Hz; only B1's 190.6 from 900 ticks (30 s). So "20 to 30 s" is wrong for three of four; "30 to 50 s" is right. Comment only; no behaviour.
- C12 evidence notes: slip "c5cd4d9c" at notes :53 is F1's script (f1f131b sha c5cd4d9c, I computed); sources.log:5 and ran-scripts give 18c1dcf6 (=791bc55, =493b3d7^); cmp-P0 HEAD runs show F2's two calls, impossible at F1's copy. Slip in the summary only. Evidence task output: agent model claude-opus-5-5 (general-purpose); effort not recorded in the task output (card says high, stated in the call): unverifiable by me, not a finding.
- C13 trial run: cmp-precheck's harness check narrowed before the final run (notes); final run harness unchanged after. Acceptable: the narrowing was the harness's own false grep (HEAD's marks.json line contains "the judge").
- C14 card: two readings recorded; SIZE-CHECK planned vs actual (cond 10); cond 7 start figures note present; F3P-1 report :521 fixed. Cond 8: the pre-check sentence is not yet carried anywhere in old form (no 372,637 in card/report); the report's F3 entry is still to be written: it must use cond 8's words if it carries the sentence.
