# U4r-F2's review: infra-engineer, its one round (workflow wf_8a10f9e4-993), 2026-11-14

Written by script, word for word, from the workflow's journal. The diff reviewed: c-u4r-runs.mjs, uncommitted in the main folder (52 insertions, 22 deletions; sha256 18c1dcf6...). Its notes: u4r-f2-review-infra.md. Nothing below is the main session's.

## Verdict: APPROVE WITH FINDINGS

Verdict: APPROVE WITH FINDINGS. The runner's code has no defect. One Medium is in the operator's stop procedure, and its fix touches the card only. Two Lows are about comment wording and evidence housekeeping.

**What happened.** I read the diff against HEAD and the advisor's nine conditions. The diff is 52 insertions and 22 deletions in one file, and the working copy's sha256 is 18c1dcf6…, the copy the harness tested. I re-ran the harness's cmp, fail, lim and recheck stagings myself: 15 stagings, 0 FAIL, exit 0. I ran the scratch helper test by hand: 5 of 5, exit 0. I re-ran node --check and redact-check (284 items, 0 findings). I also wrote three scratch checks of my own under /home/user/Shamble-Frontlines/.gate/u4r-f2-review/. Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-f2-review-infra.md.

**The runner's mechanics** (line numbers are in the working copy):
- **Order:** both calls go through judgeCall (:445-455), and each runs its own re-check before its bounded call. Measured runs first, then bites (:456-457).
- **results/:** the folder is made only after both calls (:459). Each verdict file is copied only on its own exit 0 (:460).
- **Exit lines:** :474 keeps HEAD's words for a set without bites; :475 is condition 3(d)'s line.
- **Layout:** run folders are `<logs>/<group>/<run>` (:436). npt-lab creates the parent folders itself (create_dir_all, scenario.rs:771).
- **What each call reads:** the judge picks up .jsonl files at any depth (judge/src/lib.rs:19-46), so the verdict.json and marks.json at `<logs>` are never read.
- **Exit 0 means a verdict file exists:** the judge returns exit 0 only after writing its --out (main.rs:110-121).
- **Unchanged:** the exists-check (:329), the lock and check.mjs's run() (process group, SIGKILL at RUN_LIMIT_MS).
- **Memory:** the calls run one after the other (spawnSync), so the peak is the larger call, not the sum.

**Conditions 2 to 5: is each one proven?**
- **Condition 2: proven.** The real-git stagings recheck-bites-edit and recheck-bites-commit show the stop message names "the judge (bite runs)", call 1 ran, call 2 did not, and no results/ folder exists (not even results/u4r/). recheck-measured-edit is the control.
- **Condition 3: proven.**
  - (a) All three endings hold: exit 1 in the fail stagings, "stopped at" in the lim stagings, and "exit by a signal" in my own check (a child that SIGKILLs itself through check.mjs's run(), standing in for an out-of-memory kill).
  - (b) SPLIT_LINE equals the advisor's text word for word.
  - (c) The raw-logs line names measured/ and bites/.
  - (d) The exit line is right for one failed call, both failed, and a stopped call.
  - (e) cmp-P2, cmp-P3 and cmp-B1 match HEAD, and fail-P3's exit line is HEAD's.
- **Condition 4: proven.** All six cmp stagings pass, as do fail-both, the which-file-is-kept checks, and the judge stand-in's times (measured first, no overlap).
- **Condition 5: proven as specified.** Three real-judge calls under the lane lock print RESULT: ALL EQUAL. My own key-by-key compare finds only the by-design differences.
  - The sample covers only 2 of P0's 6 bites. The other four read not_run in all three files, so their equality rests on the judge's code.
  - I checked that code beyond eval.rs:958-996. Marks -2, -17 and -57 have no "against" threshold, no ws_bound and no validity rule, and their four figures ignore the other runs (figures.rs:24, :542, :563, :571).
  - Its clean-up clause is still open (INF-F2-3).

**Other conditions I could check:**
- **Condition 1 holds.** P2's results were committed at 1eacb60 (20:22:31 UTC), the script was edited at 20:26, and nothing under spikes/ was committed since.
- **Condition 6 holds so far.** node --check passes, redact-check and gate --no-smoke both pass with their logs kept, and the size is on the card. CI waits for the push.
- **Condition 8 holds.** The header sentence matches advisor-u4r-f1-head-full.md:33 word for word.

**What P0's 70-minute set would lose under each failure:**
- **One judge call fails** (an exit code, the time limit, or a signal such as an out-of-memory kill): the other call still runs, and results/ gets runs.txt plus the good call's verdict. The exit line names the failed call. The raw logs stay in measured/ and bites/, so that half can be judged again by hand on the advisor's ruling without re-running the set. HEAD lost both halves here.
- **Both calls fail:** only runs.txt; the exit line names both calls.
- **A re-check stop before either call:**
  - Nothing goes into results/ and no runs.txt is written.
  - The run lines survive only in the kept log (.gate/u4r-sets/…), and call 1's verdict (if it ran) only in `<logs>/verdict.json`.
  - The set re-runs at the new commit. This is the same as HEAD, plus one more stop point in the minute or so between the two calls.
- **An npt-lab refusal:** the set stops at that run, as in HEAD.
- **The script killed or the session cut:** no results/, and the lane lock is freed at once. During the measured call, the written stop order also starts the bites call first (INF-F2-1).
- **A container restart:** everything stops. .gate/ survives (it did in the 19:13-20:21 restart) and /dev/shm does not.
- **Disk:** nothing changes (the same logs, plus one small verdict file).

**Cost and rollback:** nothing per run or per month (it runs in this container, with no provider), so no provider outage can affect it. Nothing is exposed: local files only, no network, no secrets. Rollback is reverting the one commit. No set has run on it yet, and the P3, B1 and P2 layouts and results are untouched.

**Leftovers:**
- My scratch folder /home/user/Shamble-Frontlines/.gate/u4r-f2-review/ holds three scripts and the stop staging's tree (1.7 MB).
- My harness re-run rewrote /dev/shm/u4r-f2/stage, /dev/shm/u4r-f2/logs and /dev/shm/u4r-f2/recs. The kept logs in /home/user/Shamble-Frontlines/.gate/u4r-f2/logs/ are untouched: I did not run collect.
- No processes are left (pgrep is empty).

**Decisions needed:** none from Mustafa. One call for the advisor (code ADV-Q, INF-F2-1's header sentence): add it to the script's header now, before the commit, or at the runner's next change? It matters because a comment added now changes the tested copy's sha256. The options are now, or the next change. I recommend the card now and the header at the next change.

**For Mustafa:** nothing.

## Findings

### INF-F2-1 (Medium)

- **Where:** c-u4r-runs.mjs:12-13 (the stop-order sentence this diff adds) with :456-457 (both calls run whatever the first's end); /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test.md:350-351 (run procedure item (6))
- **Wrong:** In a set with bites (P0), the written stop order ('end the call's group first, then the script') does not stop the set during the measured judge call:
- Ending call 1's group makes the script start call 2 at once. Q2 (a) wants that for an out-of-memory kill.
- Killing the script next leaves call 2 orphaned: it runs in its own session with no time limit, and the lane lock is freed. With the real judge, that is the bite-runs call at about 6.2 GiB for about a minute (P2's 779 MiB folder took 23 s), overlapping whatever lane takes the lock next. This is exactly the case INF-F1-1 was about.
- HEAD had one call: ending its group made the script write runs.txt and exit on its own.
- npt-lab runs are not affected: timed() throws on a killed run, so the set ends.
- **Evidence:** My scratch staging /home/user/Shamble-Frontlines/.gate/u4r-f2-review/stop-order.mjs used F2's copy (sha256 18c1dcf6…), the harness's stand-ins and a judge stand-in that sleeps on every call.
- After `kill -9 -- -11524` (judge call 1's group), judge call 2 started 81 ms later, with the script still alive.
- After `kill -9 11384` (the script), call 2 (pid 11551, state S) and its sleeper were still running 0.5 s later. `pgrep -af u4r-f1-sleeper-infra-stop` found the sleeper, and no results/ folder existed.
- I then ended call 2's group; pgrep is empty.
- **Fix:** No code change.
- **Card, run procedure item (6), now:** 'In a set with bites (P0), ending the measured judge call's group starts the bite-runs call at once: pgrep again and end that call's group too; the script then writes runs.txt and exits 1 on its own. Kill the script only when no npt-lab or npt-judge call is left.' Item (7)'s orphan check stays.
- **Header:** add the same sentence after the advisor's words at the runner's next change (condition 8's), so the committed script stays the tested copy. The advisor may instead allow it now, before the commit, as a comment that changes no behaviour.

### INF-F2-2 (Low)

- **Where:** c-u4r-runs.mjs:35-36 and :443-444 (the new header paragraph and the judge comment); :66-69 (the stop paragraph)
- **Wrong:** Three comments overstate or understate what the code does:
- **'No run's verdict reads another run, so each call's verdicts are what one call gives'** is true only for the checks of marks -2, -17 and -57 at marks.json 9fb5be57. It is not true in general: the judge's twin figures read other runs of the set, so a later set whose bites fall under such a mark could not be split by the same rule.
- **'Both calls always run'** reads as if a re-check stop still runs call 2. It does not (condition 2).
- **:66-69** still says a judge failure leaves 'no verdict', where a split set keeps the other call's verdict.
- **Evidence:** - In marks.json, none of -2, -17 or -57 has an 'against' threshold, a ws_bound or a validity rule.
- Their four figures ignore the other runs (`_set` unused): figures.rs:24, :542, :563, :571.
- twin_of reads the set (figures.rs:99-120), and fate_log_twin_equal and twin_hash_mismatch_count use it (:156-158, :195-197).
- My keys.py on /home/user/Shamble-Frontlines/.gate/u4r-f2/real/*.json finds only the by-design differences.
- **Fix:** Comment only, at the runner's next change (condition 8's). Suggested wording:
- 'No run's verdict reads another run for -2's, -17's and -57's checks (no against threshold, no ws-practice rule, figures that read one run); a mark that compares runs (a twin figure) needs one call.'
- 'Both calls run whatever the first's end; a re-check stop ends the set before either, with no results/.'
- At :66-69: 'no verdict from that call'.

### INF-F2-3 (Low)

- **Where:** /dev/shm/u4r-f2/ (the last sentence of condition 5); /home/user/Shamble-Frontlines/.gate/u4r-f2/harness/harness.mjs:37 (NOTES), used by helper() and realjudge()
- **Wrong:** Two pieces of evidence housekeeping are open:
- Condition 5 asks for the /dev/shm copies to be removed afterwards. /dev/shm/u4r-f2/real (30 MiB of r2's logs) and the rest (66 MiB in all) are still there; the card records that the rm was denied.
- The harness's helper and realjudge stagings still point at .checkpoints/netcode-proof-test/u4r-f2/, but those files were moved to .gate/u4r-f2/scratch/. So the helper or realjudge stagings cannot be re-run as kept. I ran the helper test by hand instead (5 of 5).
- **Evidence:** - du: real 30 MiB, stage 34 MiB.
- df /dev/shm: 66 MiB used of 16,088.
- .checkpoints/netcode-proof-test/u4r-f2/ does not exist.
- The moved files' sha256 (ecfc8c84…, 56be2e1d…) equal sources.log's.
- **Fix:** No push.
- Remove /dev/shm/u4r-f2 when the permission allows, or let the next restart do it. It is 66 MiB of RAM out of 16 GiB, so it does not touch P0's 10 GiB judge line.
- If the harness is kept, point NOTES at .gate/u4r-f2/scratch/ (scratch, never committed).
