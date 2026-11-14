# Advisor: the review of U4r-F3's pushed head (workflow wf_a90fed9f-b0b), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-u4r-f3-head's StructuredOutput result). Its notes: advisor-u4r-f3-head.md. Nothing below is the main session's.

## Verdict: APPROVE

**Head:** 493b3d7d46d289f759c990585b330bd34a3144da (U4r-F3, PR #40 draft, branch claude/netcode-proof-test-plan-hcky5g). The branch tip is now 7e5ecaa, a [skip ci] checkpoint commit that touches only .checkpoints/ (the card, my notes file, a writer .py). Risk tier 1: two spike ops scripts, no game, kit or gate file.

## Checks

1. The tested copies: git show 493b3d7 gives c-u4r-runs.mjs 99dd070eb20f... (608 lines) and c-u4r-r2.mjs 4e007500c5d1... (236 lines). They equal the working copies and sources.log's f3/ copies. The parent's copies are 18c1dcf6 and 94564f8c, the same as sources.log's head/ copies. All 25 stagings in ran-scripts.txt name these sha256s, or one of the three edited copies. I diffed each edited copy: each differs by exactly one line (f3-big :152 P0 rate 1.45e9; f3-limit :171 and head-limit :142 RUN_LIMIT_MS 2,000). PASS.
2. The harness was not edited after its run (ADV-F2H-1): harness.mjs e8516622, its 4 shims and the 2 scratch files hash today exactly as sources.log records them, and harness.mjs was last changed at 23:30:00, before the run started at 23:31. The trial run's narrowed check was the harness's own false grep, fixed before the final run. PASS.
3. Condition 1: the header at :13-17 carries INF-F2-1's card words plus the tag '(U4r-F2's review, INF-F2-1)'. The three INF-F2-2 wordings match u4r-f2-review-full.md:93-95, with the Q1 (b) change I allowed. The judge comment (:542-546) is one sentence and keeps both 'each after its own re-check' and 'results/ is made only after both'. PASS.
4. Condition 2: the clause is exact. I checked the arithmetic: 302.2 + 2.3403 x 3,248.3 = 7,904 MiB (7.7 GiB), and 212.3 + 2.5062 x 3,248.3 = 8,353 MiB (8.2 GiB). The report line at :521 now carries the F3P-1 wording. PASS.
5. Condition 3, in code: guarded() (:550-563) wraps logBytes + judgeRoom, and also readFileSync + JSON.parse + setParts. Each array is built in full before any line prints. setParts tolerates a missing parts, bites, runs or why array, and throws only when the top level is not a list, which the guard catches. Pushing the judge line and copying into results/ stay outside the guard, as in HEAD. In the evidence: parts-P3-notjson, parts-P3-empty, parts-P0-bites-notjson and parts-P0-bites-empty each show one error line, the P0 measured parts still present, the file still copied, and HEAD's end and exit. PASS.
6. Condition 4: the start check sits after room('before the builds') and before mkdirSync(bin) (:441-451), and the pre-check has no judge groups. big-plan: the measured call's 4,148.5 MiB is room, the bites call's 4,308.7 MiB projects 10,386 MiB and is over, exit 1, with no cargo call, no bin, raw-log or results folder, and nothing under .gate/. cmp-precheck: 0 of 28 lines differ, and there are no judge lines. PASS.
7. Condition 5: judgeLines = split ? [SPLIT_LINE, ...block0, ...block1] : block0. I read F3's runs.txt in cmp-P3 and cmp-P0: on-disk line, then the judge line exactly as HEAD's, then the parts heading and lines; P0 has SPLIT_LINE first, then the measured block (3 parts, 2 whys), then the bites block (6 bites). The fail-* and lim-* stagings show no parts under a failed call, the other call's parts present, and the exit lines word for word. PASS.
8. Condition 6: cmp-precheck and fail-P3 are present. The independent reading expected_parts.py reads the fields only and never imports the runner. My own python3 -I count of the five committed verdict files gives P3 3/6, B1 4/9, P2 3/4, P0 3/2/0 and P0-bites 0/0/6, the same verdicts as helper.log. The 19 verdicts were cross-checked against the report's table. PASS.
9. Condition 7: I re-measured every kept run folder with du -sb. Every figure in the comment at :139-150 holds, each rate is that profile's largest run rounded up to 0.1 MB, and the five folder names exist. I checked the projections by hand: P3 524.4, B1 833.7, P2 780.8 and P0 3,614.6 MiB; P0's calls 1,727.2 MiB (4,344 projected peak) and 1,887.4 MiB (4,719); the line falls at 4,246.4 MiB. The card carries the BRAWL-1 start figures. PASS. Exception: my one Low, ADV-F3H-1.
10. Condition 8: the pre-check sentence has not been carried anywhere in its old form (no 372,637 in the card or the report). It binds the report's F3 entry, which is still to be written.
11. Condition 9: the evidence agent's task output records its model as claude-opus-5-5. Its effort is not recorded there; the card says high was stated in the call, and I cannot verify that. The card counts 3 agents, at most. No agent edited a script. PASS, with that one limit.
12. Condition 10: node --check passes on both scripts; redact-check runs 10/10 tests and its scan finds 0 in 287 items; gate --no-smoke PASS. All three logs are kept. The card shows the planned size beside the actual one. No set has run at 493b3d7: runs/ and bin/ have no folder for it, and results/u4r holds only the 4 old commits. PASS.
13. CI, checked through the GitHub tools: gate run 43654541605 (push, head_sha 493b3d7d46d2...) concluded success, with jobs gate and smoke both success; toolchain run 43654541555 on the same SHA, success.
14. INF-F3-2: I read the reviewer's logbytes-check.mjs and its design is sound. I reproduced its find sums myself: P3 543,998,603 B and P0/bites 1,856,944,887 B (1,770.92 MiB). By reading the code, logBytes counts the same files as find -type f (symlinks skipped). Accepted with no code change.
15. The evidence agent's slip: c5cd4d9c is the sha256 of F1's copy (f1f131b), which I computed. sources.log:5 records 18c1dcf6, and cmp-P0's HEAD runs make two judge calls, which F1's copy could not do. So the slip is in the summary only, and the evidence holds.

## The two readings of the conditions

Both readings are accepted. (1) Condition 3's 'one guard' is built as one helper, guarded(), used before and after the judge call. That is the only sound way to build it: a single try block around both pieces would have to wrap the judge call too, and that would change HEAD's failure paths. The condition's aim holds: nothing F3 adds can throw out of judgeCall or stop runs.txt being written. The four parts-* stagings prove it on P3 and on P0's bite-runs call. (2) Condition 4's on-disk line ends 'over the line, printed only (advisor-u4r-sets-r1, Q2: to the advisor)'. The ending 'stopped before the builds' belongs to the start-check lines. The on-disk line only prints, so claiming a stop there would be false. The line keeps the same form otherwise, and helper (b) shows the ending at 4,247 MiB on disk.

## Size

157 insertions and 34 deletions (191 changed lines): c-u4r-runs.mjs 153/31, c-u4r-r2.mjs 4/3. I expected 100-130 and 20-30, so this is about 27 insertions over. It stays under my 'well past' line of more than 250 changed lines, so SIZE-CHECK (Mustafa's words, history.md:398) is not triggered. By my count the added lines are 76 comment and 77 code, so the excess is the conditions' required text (condition 2's clause, condition 7's comment, the guard and block-order comments, the two header bullets). Accepted. Nothing goes to Mustafa for size.

## Findings

### ADV-F3H-1 (Low)

- **Where:** spikes/netcode-proof/ops/runs/c-u4r-runs.mjs:143-144 (the RATES-1 comment, at 493b3d7)
- **Wrong:** The comment says 'the builder's rates from 20 to 30 s runs understated them'. u4r-build.md:739-751 shows that the rates the runner used came from the d13 runs: P0 509.1, P2 219.8 and P3 125.8 MB, each from 1,500 measured ticks, which is 50 s at 30 ticks/s. Only B1's 190.6 MB came from 900 ticks (30 s). So '20 to 30 s' is wrong for three of the four rates. The figures and behaviour are unaffected: it is comment wording only.
- **Fix:** At the next change to c-u4r-runs.mjs (the check.mjs follow-up, which edits judgeCall for INF-F1-3 anyway), change it to 'the builder's rates from 30 to 50 s runs (u4r-build.md:739-751)'. No push for this alone.

### INF-F3-1 (Low)

- **Where:** spikes/netcode-proof/ops/runs/c-u4r-runs.mjs:559 (the catch in guarded())
- **Wrong:** Infra-engineer's finding, which I confirm. V8's JSON.parse message can carry raw newlines, so the 'one error line' can span several physical lines in runs.txt. It is cosmetic and happens only with a malformed verdict file; runs.txt is still written.
- **Fix:** Collapse whitespace in the catch (String(e?.message ?? e).replace(/\s+/g, ' ')). This rides the check.mjs follow-up, before PR #40 is marked ready. No push now.

### INF-F3-2 (Low)

- **Where:** .gate/u4r-f3/logs/cmp-*.log (the on-disk lines read 0.0 to 0.4 MiB on the shim logs); c-u4r-runs.mjs:252-260 (logBytes)
- **Wrong:** Infra-engineer's finding. The stagings prove where the on-disk line sits and how it reads, but not its value on real logs. Its run output was not kept as a log; the review's text is the record.
- **Fix:** No code change. The report's F3 entry cites .gate/u4r-f3-review/logbytes-check.mjs and its figures, plus my own find sums (P3 543,998,603 B; P0/bites 1,856,944,887 B), which match them.

## Next steps

1. 1. Record this head review word for word (the writer u4r_f3_head_review_full.py is ready). There is no finding of Medium or higher to read in full.
2. 2. Close U4r-F3 at 493b3d7. Mark task #68 completed. Add the F3 entry to u4r-report.md with: the size (157/34 against the planned 100/25 and my 100-130/20-30, under the 250 line); the evidence (25/25 stagings, the helper test, the basics); CI runs 43654541605 (gate and smoke) and 43654541555; infra-engineer's APPROVE WITH FINDINGS; my APPROVE of this head; both readings as ruled here; INF-F3-2's evidence (the reviewer's check plus my find sums); HEAD's copy named as 18c1dcf6, not c5cd4d9c; and condition 8's exact words if the entry carries the pre-check sentence.
3. 3. Put INF-F3-1, ADV-F3H-1 and INF-F1-3 on the check.mjs follow-up's list (INF-R2-5 and the rest, before PR #40 is marked ready). That follow-up edits judgeCall's 'by a signal' phrases anyway, so all three ride it. No push now.
4. 4. BRAWL-1's lane comes next. Wait for my BRAWL-1 plan review (wf_533068a9-b2b) and follow it. Its build comes first. Then the re-run sets run on c-u4r-runs.mjs 99dd070e at the fix's commit: P3, then P2, then P0 split, as BRAWL-1's plan sets out. Its disk plan uses the new start figures (P3 524.4, P2 780.8, P0 3,614.6 MiB, each plus the 4,096 MiB floor) and the judge start check (P0's bites call is projected at 4,719 MiB, against the 10,240 MiB line).
5. 5. Housekeeping when convenient: remove /dev/shm/u4r-f3, a few MB of stage folders.

## For Mustafa

Nothing goes to you. U4r-F3 is a small change to the scripts that drive the test runs, on a spike branch inside a draft PR. Nothing a player sees changes, there is nothing to decide, and it grew about 1.5 times its plan, under the "well past its plan" line, so SIZE-CHECK does not apply. I checked that the committed scripts are exactly the tested ones, that each of my 10 conditions is met in the code and in the evidence, and that CI is green on this head. I found one comment-wording slip; it waits for the next script change.

Decisions needed: none.

For Mustafa: nothing.
