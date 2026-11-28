# Advisor: the review of U4r-F1's pushed head (workflow wf_6bfb2c19-ce1), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-u4r-f1-head's StructuredOutput result). Its notes: advisor-u4r-f1-head.md. Nothing below is the main session's.

## Verdict: APPROVE

**Head:** f1f131b6e824b6b4ab3ef5e87579be6ea340155c on claude/netcode-proof-test-plan-hcky5g (PR #40, draft). The branch head 30749a5 comes after it and changes only .checkpoints/netcode-proof-test.md.

## Checks

1. Branch: origin's head is 30749a5. The only commit after f1f131b is 30749a5, and it touches only the card (git diff --name-only f1f131b HEAD).
2. Files: f1f131b changes three files, all mode 100644 with no symlink: the card plus spikes/netcode-proof/ops/runs/c-u4r-runs.mjs and c-u4r-r2.mjs. Size: 125 insertions and 48 deletions in the two scripts (my plan review expected 110 to 125). git diff --stat 5e563b9 f1f131b -- spikes/ shows only these two files.
3. sha256 of the scripts at f1f131b: c-u4r-runs.mjs c5cd4d9c…efb712 and c-u4r-r2.mjs 94564f8c…b37479. Both equal sources.log's f1/ copies and the working tree. The harness's head/ copies equal f1f131b^. check.mjs a169a286…, SERIES.md 85ef6453…, marks.json 9fb5be57…, loader.mjs a0507d1e…, precheck-recordings.json 430b295e… and redact-check.mjs 941c6925… all equal f1f131b. The spike tree moves 0b47e881 to a88ee99f.
4. A (INF-R2-1): spikeChanged and spikeReading are exported once (runs :130-146) and imported by r2 (:54), as ruled in Q1. runs reads git through sh(), whose default folder is the repository root. r2 passes ROOT explicitly (:169). The whole-repository start.status is gone (condition 7). Both start refusals keep tracked() (runs :296-297, r2 :164-165). runs.txt and r2.txt name the tree beside the commit.
5. B (INF-R2-3): runs copies marks.json into the bin folder, checks its sha256 there, reads the plan from that copy and calls floor(runs[0]) before mkdirSync(logs) (:346-353). r2 calls floor(suite p0) before mkdirSync(out) (:192).
6. C (INF-R2-4): timed() goes through check.mjs's run(exe, args, ROOT, env, RUN_LIMIT_MS). A non-zero exit that is not a stop throws sh()'s exact message (Q2). The stopped-run line, the stopped-pair line and the recording stop all match the plan (the recording stop is Q4's text exactly). The judge has three endings, verdict.json is copied only on exit 0, and runs.txt is written before the exit-1 line, all as condition 1 says. One gap: a signal is reported as 'by a signal' without its name (INF-F1-3).
7. D (RUN-L1, Q6): both builds run cargo with -v. Lines on stderr matching /^\s*(Compiling|Fresh) npt-/ are printed. Trimmed stdout goes to builtArtifact. The error text has sh()'s form.
8. E: the whole-as-lost suite's marks are 'PM-0001-2,PM-0001-17', and its expect line matches advisor-u4r-c62h-full.md:45 word for word.
9. Condition 1: in lim-run, the shim judge exits 1 on the cut l1-p0-s2/client.jsonl. runs.txt is still written, with 'judge: exit 1 (no verdict): …' and run 2's stopped line, no verdict.json, and the exit-1 line. realjudge: r2's npt-judge (sha256 65acacfe…, which I re-hashed) exits 0 on the uncut copy and 1 on the copy cut at line 451, naming the file and the line. r2's folder is untouched.
10. Condition 2, on real git (the system git on PATH, GIT_CONFIG_GLOBAL=/dev/null): 18 of 18 stagings pass. (a) A .checkpoints/ commit stops HEAD's copies, while F1's complete and name the commit and the tree. (b) An uncommitted edit stops F1's runs and r2 copies, quoting the marks.json status line, which proves r2 reads the status from the repository root. (c) A commit under spikes/netcode-proof stops F1's copies, naming both trees.
11. Condition 3: the floor stagings for P0, the pre-check and r2, and the (ii) stagings for P0 and the pre-check. F1's copies leave no set folder, and their re-run completes. HEAD's copies leave the folder, and their re-run is refused. Each edited copy differs from its source by one line (I checked with diff against f1f131b and f1f131b^).
12. Condition 4: every new stop line is quoted from the 2,000 ms copy (lim-run, lim-pair, lim-rec, lim-judge). After each stop the sleeper is dead (state Z) and pgrep -f finds nothing. lim-control shows the sleeper survives when run()'s group kill is left out. The first lim-* runs failed only the harness's own kill(pid, 0) liveness check, which succeeds on a zombie. The fix reads the state from /proc. That is sound, not a loosened check: pgrep -f was empty in both runs.
13. Condition 5, the shim side: in the cmp-* stagings, cargo's argv equals HEAD's plus -v, run from the spike folder. The npt-* Compiling and Fresh lines are printed and the libm line is not. The check on real builds is due at the pre-check set.
14. Condition 6: the card's run procedure carries (a), and (b) as INF-F1-1 corrects it. The plan's risks section was not updated (still sha256 774b952d…): ADV-F1H-3.
15. Condition 8: the helper test passes 9 of 9 (file sha256 47200f26…, the same as in helper.log), and test 8 asserts JDG-ONLY-1's interim rule. The main session's redact-check on the real tree left no kept log. The result holds by composition: redact-check scans each file line by line; F1's two files give 0 findings (redact.log); and spikes/ differs from 5e563b9 only in those two files, where check.mjs's redact step gave 277 items and 0 findings (.gate/u4r-merge/c62h-check.log:17). See ADV-F1H-2.
16. Counts: summary.log has 47 lines and 42 distinct staging names, with 42 staging logs. The card now says '42 stagings, 47 runs', which fixes INF-F1-4.
17. CI on f1f131b, read with gh api check-runs: gate success (17:30:19 UTC), smoke success (17:30:46), toolchain success (17:30:37). f1f131b touches no .github/ or gate file, so CI's green stands for this change. I did not run the gate myself: this task allows no script runs. This approves the head on the spike branch, not a merge to main. The day-session advisor still runs the full gate on PR #40's head before any merge to main.
18. INF-F1-1 (Medium) is handled rightly. The card's run-procedure items (5) to (7) carry the reviewer's fix words, with no code change. The premise of my plan review's condition 6(b) was wrong: I said the locked descriptor stays open in the call, and the reviewer measured that no npt-lab or npt-judge call holds the lock. That mistake is mine. INF-F1-2 is fixed in the card's items (2) to (4). INF-F1-3 is handled by a report line and by the peak memory that step 17 already records. INF-F1-5 gets a report line. All are right, and none needs a push.
19. Now: 4,470 MiB free (df -m). No npt-lab or npt-judge process is running (pgrep). /dev/shm/u4r-f1 (75 MB) and /dev/shm/u4r-f1-review are still there; they are in RAM (tmpfs), not on disk.

## INF-F1-1's optional header sentence

Add it at the scripts' next change, not now. The header is not wrong: it says only that the set runs under the lane lock (from the repository root) and that each call is bounded. It never claims a call holds the lock. The card's run procedure is the binding rule, as my Q5 ruling said, and adding a comment alone now would be a push for a Low, which the review budget forbids. When c-u4r-runs.mjs next changes, add after the flock sentence: "If this script dies while a call runs, the lane lock is freed at once (no npt-lab or npt-judge call holds it: advisor-u4r-f1-head, INF-F1-1, measured), while the call, in its own session, runs on with no limit: to stop a set, end the call's group first (`kill -9 -- -<pid>`, the pid from `pgrep -a npt-lab; pgrep -af npt-judge`), then the script."

## Findings

### ADV-F1H-1 (Low)

- **Where:** .checkpoints/netcode-proof-test.md, THE SETS' RUN PROCEDURE, item (2)
- **Wrong:** The command sends its log to .gate/u4r-sets/<set>-<commit12>.log but never creates that folder. If the folder is missing, the shell refuses the redirect and the set does not start (that fails safe). The procedure also leaves out step 17's ruled peak-memory reading: python3's resource.getrusage(RUSAGE_CHILDREN).ru_maxrss around each script call (advisor-u4r-r2-full.md:23). A session that follows the card alone would lose the judge-memory figures that the P0 projection needs.
- **Fix:** Card text only, at the next card save (no code push): add `mkdir -p .gate/u4r-sets` before the first set. For the judged sets, wrap the command in the python3 getrusage reading, as step 17 rules, and keep the printed peak in the set's log.

### ADV-F1H-2 (Low)

- **Where:** the card's build line ('redact-check 0 findings; gate --no-smoke PASS'); no file under .gate/u4r-f1/
- **Wrong:** The main session's own runs for condition 8 (redact-check on the real tree) and of gate --no-smoke left no kept log. The claims hold another way: the composition check above for redact, and CI's gate and smoke green on f1f131b for the gate.
- **Fix:** No re-run and no push. The report's U4r-F1 entry names the evidence route it actually took. From now on, keep such runs' output under .gate/<slug>/.

### ADV-F1H-3 (Low)

- **Where:** .checkpoints/netcode-proof-test/u4r-f1-plan.md section 3 (sha256 774b952d…, unchanged)
- **Wrong:** Condition 6 asked for both risks in the plan's risks section and in the card's run procedure. Only the card has them.
- **Fix:** Leave the old plan as it is. Put the two risks, with the corrected lock mechanism (INF-F1-1), in u4r-report.md's U4r-F1 entry, beside the report lines for INF-F1-3 and INF-F1-5.

## Next steps

1. 1. Record this verdict word for word on the card, with its 'For Mustafa', and save. No code push: every open item is a Low, and each rides card or report text.
2. 2. Update u4r-report.md's U4r-F1 entry from 'Pending' to done at f1f131b. Include: the size (125/48); the evidence (42 stagings, 47 runs, all PASS on their last run, the lim-* re-run explained); the one round (infra-engineer, APPROVE WITH FINDINGS); INF-F1-1 fixed in the run procedure; INF-F1-3 (the signal's name is not recorded; home: check.mjs's run() returns `signal` at its next change, and both scripts name it); INF-F1-5 (the first status line is shown trimmed; home: an untrimmed read at the scripts' next change); ADV-F1H-2 and ADV-F1H-3; and the header sentence deferred to the scripts' next change.
3. 3. Fix ADV-F1H-1 in the card's run procedure (mkdir -p .gate/u4r-sets; the getrusage wrapper for the judged sets).
4. 4. Step 14, D4: measure the runner's ticks per second in the main folder's own cache (CARGO_TARGET_DIR unset) with r5's command, then redo the sets' wall-time estimate. Only 4,470 MiB is free: check that at least 4 GiB stays free before D4's build and run. If it would not, write the disk plan and make its approved part-clean first, between lanes with no lane running, and say so.
5. 5. Step 15, the disk plan in writing, before the first c-u4r-runs.mjs set: free space comes only from the approved part-cleans (.gate/shared-target, the main folder's cache), counting any lane that runs during the sets. Never delete a log. If P0 still cannot fit, the smaller sets go first and P0 goes to Mustafa under Decisions needed (LEFTOVER-DELETE).
6. 6. Step 16, the pre-check set, run by the card's procedure: the card committed and pushed, pgrep empty, the tree clean, flock -x from the repository root, the log kept. Condition 5: both npt-lab builds must show Compiling or Fresh npt-* lines naming /home/user/Shamble-Frontlines/spikes/netcode-proof/crates/…; if a build shows none, it comes to me (the set still stands on its sha256s and is not re-run). The late-command pair counts only on its whole line. Anything not as expected, or any 'stopped at' line, comes to me whole. Commit results/u4r-precheck-30/<commit12>/runs.txt before the next set.
7. 7. Step 17, the judged sets one at a time: P3, B1, P2, then P0. Read peak memory with getrusage around each script call. Before P0: re-check RUN_LIMIT_MS against the judge's measured time, and send me any judge-memory projection over 10 GiB. Every run flagged for overflow resyncs comes to me before it counts (P0's tick16 bite run is expected, about 48). Any 'stopped at' line or 'judge: … (no verdict)' comes to me whole. Commit each set's results/ before the next set starts, then fill the report and the card.
8. 8. Housekeeping: /dev/shm/u4r-f1 and /dev/shm/u4r-f1-review are in RAM (tmpfs) and need nothing (LEFTOVER-DELETE; they go at the next container restart).

## For Mustafa

Nothing goes to you now. U4r-F1 is a small fix to two run scripts inside the netcode spike (risk tier 1). It changes nothing you see, hear or feel and no rule or decision of yours, it leaves no security finding open, and it is not a merge to main: PR #40 stays a draft and reaches you later with its own report. In plain words: the fix does what was approved. The end-of-set safety check now looks only at the spike's own files, a stop before the first run leaves nothing that blocks a re-run, every test run and the checker have a one-hour limit, and the build lines reach the log. The evidence holds, and CI is green. One heads-up, not a decision now: the biggest run set (P0) needs about 4 GiB more free disk than today (4,470 MiB free). If the cache cleans you already approved don't cover it, P0 comes to you under Decisions needed (your LEFTOVER-DELETE route). Decisions needed: none. For Mustafa: nothing.
