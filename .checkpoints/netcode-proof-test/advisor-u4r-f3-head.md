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
- C15 infra-engineer round read (u4r-f3-review-full.md): APPROVE WITH FINDINGS, INF-F3-1 Low (multi-line V8 parse message; collapse whitespace at the next change), INF-F3-2 Low (on-disk value proven only by its logbytes-check.mjs). I read logbytes-check.mjs (sound: sha check, logBytes vs find -type f sum, missing folder). My own find sums: P3 543,998,603 B and P0/bites 1,856,944,887 B (1,770.92 MiB) equal the review's figures. logBytes by reading: Dirent isDirectory/isFile, symlinks skipped, same as find -type f. INF-F3-2 accepted, no code change. Its run output was not kept as a log: the review text is the record; my find sums back it.
- C16 conditions 1, 2, 7 words: header :13-17 = INF-F2-1 card words + tag; INF-F2-2's three = u4r-f2-review-full.md :93-95 with Q1 (b); judge comment one sentence, keeps 'each after its own re-check' and 'results/ is made only after both'; cond 2 clause exact; 7.7 = 302.2+2.3403x3248.3 = 7,904 MiB, 8.2 = 8,353 MiB. PASS.
- C17 code reading cond 3: guarded() try/catch wraps logBytes+judgeRoom and readFileSync+JSON.parse+setParts; the arrays are built whole before printing; setParts tolerates missing parts/bites/runs/why (list()), throws only on a non-list top level (caught). block.push of the judge line and copy into results/ are outside, as HEAD. Nothing new can leave judgeCall. PASS. Cond 4: start check after room('before the builds'), before mkdirSync(bin); judgeGroups [] for precheck. PASS. Cond 5: judgeLines = split ? [SPLIT_LINE, ...block0, ...block1] : block0. PASS.
- C18 no set ran at 493b3d7: runs/ and bin/ hold no 493b3d7 folder; results/u4r holds the 4 old commits only. PASS cond 10 last part.
- C19 CI (GitHub tools): gate run 43654541605, event push, head_sha 493b3d7d46d2..., conclusion success; jobs gate success, smoke success (check 5a step success). toolchain run 43654541555, same SHA, success. 7e5ecaa is [skip ci], .checkpoints/ only. PASS.
- C20 size: 157/34 = 191 changed lines; my expectation 100-130/20-30, "well past" > 250. Added lines: 76 comment, 77 code (my count): the excess is the conditions' required text. SIZE-CHECK (his words, history.md:398) not triggered. Accepted.
- C21 rulings on the two readings: (1) one guard as one helper used twice: accepted (a single try around both would wrap the judge call and change HEAD's failure paths; the condition's aim holds and stagings (iii) prove it). (2) on-disk ending "over the line, printed only (...)": accepted (that line only prints; "stopped before the builds" would be false there; helper (b) shows it at 4,247 MiB).
- C22 ADV-F3H-1 confirmed at c-u4r-runs.mjs:143-144 (493b3d7). Low: rides the check.mjs follow-up with INF-F3-1 (:559) and INF-F1-3.

## Verdict: APPROVE (head 493b3d7d46d289f759c990585b330bd34a3144da)
- Findings: ADV-F3H-1 Low (mine), plus infra's INF-F3-1 Low and INF-F3-2 Low (accepted, no code change). No push.
- Next, in order: (1) record this review word for word; (2) U4r-F3 closes at 493b3d7: task #68, the report's F3 entry (size, evidence, CI runs, both readings as ruled, INF-F3-2's evidence = the reviewer's check + my find sums, the slip corrected to 18c1dcf6, cond 8's words if the pre-check sentence is carried); (3) the follow-ups INF-F3-1, ADV-F3H-1, INF-F1-3 into the check.mjs follow-up's list (before PR #40 is marked ready); (4) BRAWL-1: wait for my plan review (wf_533068a9-b2b), then its build and re-run sets on c-u4r-runs.mjs 99dd070e, with the new start figures.
- Mustafa: nothing (tier 1 spike tooling on a draft PR, nothing a player sees, no decision, size not well past plan).

- C14 card: two readings recorded; SIZE-CHECK planned vs actual (cond 10); cond 7 start figures note present; F3P-1 report :521 fixed. Cond 8: the pre-check sentence is not yet carried anywhere in old form (no 372,637 in card/report); the report's F3 entry is still to be written: it must use cond 8's words if it carries the sentence.
