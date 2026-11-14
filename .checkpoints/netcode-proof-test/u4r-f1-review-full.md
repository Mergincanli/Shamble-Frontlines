# U4r-F1's review: infra-engineer, its one round (workflow wf_9ca6c622-b4f), 2026-11-14

Written by script, word for word, from the workflow's journal. The diff reviewed: the two run scripts, uncommitted in the main folder (125 insertions, 48 deletions). Its notes: u4r-f1-review-infra.md. Nothing below is the main session's.

## Verdict: APPROVE WITH FINDINGS

Verdict: APPROVE WITH FINDINGS. The diff does what the plan and the conditions ask: no more, no less. I found no defect in the code. The evidence holds for conditions 1 to 4 and 8, and condition 5's shim side. One Medium finding is about how sets are run, not the code: the lane lock is handled the opposite way from what the card says.

What I checked in the code:
- Item A (scoped re-check): c-u4r-runs.mjs:128-146, :306-307, :368-371; c-u4r-r2.mjs:54, :169-171, :222-223. The tree words are at runs :355 and r2 :200. r2's git readings run from the repository root.
- Item B (folder order): runs :346-353 (marks.json goes into the bin folder and is checked there, then the first run's floor, then mkdir); r2 :192 (floor before mkdir).
- Item C (time limits): RUN_LIMIT_MS is at runs :124-128. timed() at :277-283 matches sh()'s cwd, env, buffer and error text exactly. The stop lines are at :381, :395-400 and :419; the judge's three-way end is at :427-445.
- Item D (build lines): -v and the npt-* filter, runs :260-270 and r2 :137-147.
- Item E (whole-as-lost suite): r2 :91, word for word as ruling c62h step 4.
- Condition 7: the start keeps no whole-repository status; the start refusals stay (runs :296-297, r2 :164-165).
- node --check passes on both scripts (my own run).
- Measured size: 125 insertions and 48 deletions (runs 100/35, r2 25/13), as the card says. Of the 125, 62 are comment lines and 63 are code. That is within the advisor's 110 to 125, so the card's figure is honest.

What I checked in the evidence:
- The sha256 values in sources.log match the files: the main folder's edited scripts equal the f1 copies; HEAD's copies, check.mjs, marks.json and the edited copies all match.
- I read every staging log:
  - Real-git stagings, 18 of 18 PASS. A .checkpoints/ commit stops HEAD's copies; F1's copies complete and name the start commit and the tree. A tracked edit, quoted from the root, stops F1's copies, which proves r2 reads git from the root. A commit under spikes/ also stops them, naming both tree hashes.
  - Floor and marks.json-edit stagings: F1's copies leave no set folder and their re-run completes; HEAD's copies leave the folder and their re-run is refused.
  - The 2-second-limit copy: all five stop lines appear, the whole process group is killed, and a control run without the group kill shows the sleeper survives.
  - The real judge (r2's binary) exits 1 on a cut client.jsonl, naming line 451.
  - The helper test passes 9 of 9.
- I also checked that a run cut exactly at a line boundary (no footer) leaves the judge exiting 0, with that run marked invalid (eval.rs:579). The code covers that case too.
- The checkpoint saver commits only .checkpoints/, through a private index (checkpoint.mjs:150-170), so a save does not stop a set.

Re-run in my own scratch (/dev/shm/u4r-f1-review/): I copied check.mjs's run() function verbatim and ran it under `flock -x`. The finding:
- Node marks the lock's file descriptor close-on-exec (flags 02100000). So the npt-lab call never holds the lane lock.
- If the script is killed mid-call, flock exits and the lock is free at once, while the orphaned call keeps running with no time limit (INF-F1-1).

Risks for real sets:
- Cost: no CI or money cost, and no external provider involved.
- Rollback: revert the one commit; no set has run from these scripts yet.
- Disk: 4,474 MiB free now, down from 4,780 MiB at 17:10. A P0 set would be refused at today's free space (helper test 1), and the pre-check has about 360 MiB above the 4 GiB floor. Step 15's disk plan must come before step 16.

Left over:
- The evidence agent's /dev/shm/u4r-f1 (75 MB) is still there.
- My own scratch (3 small files in /dev/shm/u4r-f1-review/) is still there: the permission system refused my rm. No process of mine is left running (pgrep).
- My notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-f1-review-infra.md

Decisions needed: none from Mustafa. INF-F1-1's optional header sentence (adding a sentence to the module header the advisor ruled in Q5) is the advisor's call.

For Mustafa: nothing.

## Findings

### INF-F1-1 (Medium)

- **Where:** The card's run-procedure rule, condition 6(b): /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test.md:97-98, taken from advisor-u4r-f1-plan-full.md:14 and :35 ('the locked descriptor that flock passes down stays open in it'). Also the lock sentence in c-u4r-runs.mjs:6-11.
- **Wrong:** The rule assumes that if the script is killed while a call hangs, the lane lock stays held until the operator ends npt-lab by hand, so they can check `pgrep -a npt-lab` before another lane takes the lock. The opposite happens. No npt-lab or npt-judge call ever holds the lock; only flock(1) and the node script do. If the script dies mid-call (a kill, Ctrl-C, or a cut session), the lock is freed at once and any lane waiting in flock starts straight away. Meanwhile the call keeps running: it is in its own session, and its 3,600 s limit lived in the dead script, so nothing bounds it now. It then overlaps the next lane's timed or measured work. For example, det-u4 is already at 223.7 s against its 240 s line (INF-R2-2). As worded, the check 'before another lane takes the lock' is not in the operator's hands.
- **Evidence:** My scratch run: check.mjs:1909-1922's run() copied verbatim into /dev/shm/u4r-f1-review/parent.mjs and parent2.mjs, run as `flock -x /dev/shm/u4r-f1-review/lane.lock node parent2.mjs 600000` (node v22.23.4, util-linux flock 2.39.3). The lock's descriptor is open in flock (pid 1773) and in the script (pid 1775; fdinfo flags 02100000, which includes O_CLOEXEC). The call /bin/sleep 6017 (pid 1782, its own sid and pgid) has 0 descriptors on the lock. While the script lived, `flock -n` read HELD. After `kill -9` of the script: flock had exited, the call was still alive, and `flock -n` read FREE. A node grandchild gave the same result. Case A, the 2,000 ms limit: the call stopped at 2.0 s and the lock was FREE afterwards, so a stop by the limit is clean. Only a killed script leaves an orphan.
- **Fix:** 1. Card (condition 6(b), and the run procedure for steps 16 and 17): replace the mechanism with: 'Node marks the lock's descriptor close-on-exec, so no npt-lab or npt-judge call holds the lane lock; flock holds it only while the script lives. If the script dies mid-call, the lock is freed at once and a waiting lane starts, while the call (its own session, its limit gone with the script) runs on.' 2. Add the stop order: to stop a set by hand, end the call's group first (`pgrep -a npt-lab; pgrep -af npt-judge`, then `kill -9 -- -<pid>`, since the call leads its own group), and only then the script. 3. After any killed set or session cut: check `pgrep -a npt-lab` and `pgrep -af npt-judge` at once and kill any orphan. Any lane that held the lock in that window sends its timed or measured results from it to the advisor. 4. Optional, the advisor's call (the advisor ruled the header text in Q5): one sentence saying the same in c-u4r-runs.mjs:6-11. No other code change is needed.

### INF-F1-2 (Low)

- **Where:** c-u4r-runs.mjs:9-10 (the header's run form, which has no log); the card's steps 16 and 17 (/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test.md:34), not yet written
- **Wrong:** Condition 5 says the pre-check set's 'runner log' must show the npt-* Compiling or Fresh lines. These sets run outside main-run.sh, though, and the ruled form `flock -x .gate/shared-target/.lane.lock node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs …` writes no log file. The build lines (console.log, :265) and the per-run lines reach only stdout. Run as a background task, they can be lost with the session. Condition 6(a) (commit each set's results/ before the next set; nothing under spikes/ during a set) is also not yet in a written run procedure. The plan's risks section is unchanged (sha256 774b952d…).
- **Evidence:** main-run.sh:57 sends each command's output into the log ('"$@" >> "$log" 2>&1'), but c-u4r-runs.mjs refuses CARGO_TARGET_DIR (:300-304), so it cannot run through main-run.sh. The card holds condition 6 only inside the conditions block (:96-98).
- **Fix:** In the card's step 16 and 17 text, give the full command with a kept log under .gate/, for example: `flock -x .gate/shared-target/.lane.lock node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs precheck --date <date> > .gate/u4r-sets/precheck-<commit12>.log 2>&1`. Put condition 6(a), and 6(b) as corrected by INF-F1-1, in the same procedure. No code change.

### INF-F1-3 (Low)

- **Where:** c-u4r-runs.mjs:432 and :445
- **Wrong:** Condition 1 asks for `judge: exit <status or signal> (no verdict)`. When the judge is killed by a signal it did not get from us, the code writes 'exit by a signal' and 'exited by a signal' without naming the signal, because check.mjs's run() returns no `signal` field. An out-of-memory kill of the P0 judge (about 3 GB of logs to read) would then read only 'exit by a signal (no verdict): ' with empty output.
- **Evidence:** check.mjs:1921 returns { ok, out, stdout, status, timedOut }, with no signal. r2's step() uses the same 'exit by a signal' words (c-u4r-r2.mjs:157). F1 may not change check.mjs (plan section 1).
- **Fix:** No code change in F1. When step 17 re-checks the judge's time before the P0 set, also record its peak memory. That way an out-of-memory kill can be told apart from a crash. Add a line to the report saying the signal's name is not recorded.

### INF-F1-4 (Low)

- **Where:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test.md:69-70 ('every condition PASS (48 stagings; ...')
- **Wrong:** The card's count is wrong. The logs hold 42 distinct stagings and 47 runs: cmp-P0 ran twice, and lim-run, -pair, -rec and -judge each ran twice. Their first runs FAILed on the harness's own liveness check, because kill(pid, 0) succeeds on a zombie. The harness was fixed to read /proc/<pid>/stat, and all four passed on the re-run.
- **Evidence:** .gate/u4r-f1/logs/summary.log has 47 lines and 42 distinct staging names; there are 42 staging logs besides summary.log and sources.log. The evidence notes (u4r-f1-evidence-notes.md:48) also say 42.
- **Fix:** Card and report: '42 stagings, 47 runs (cmp-P0 run twice; the four lim-* stagings re-run once after the harness's liveness check was fixed), all PASS on their last run'.

### INF-F1-5 (Low)

- **Where:** c-u4r-runs.mjs:370 (the status read through sh() at :239, which trims it); c-u4r-r2.mjs:223 (sh() at :117)
- **Wrong:** The stop message quotes the first status line without its leading space: 'M spikes/netcode-proof/marks/marks.json' instead of ' M …'. That drops the first entry's staged-or-unstaged column. The compare itself is unaffected, since the start reading is always empty.
- **Evidence:** git-b-P0-f1.log:30 and git-b-r2-f1.log:189 quote 'its tracked status now reads: M spikes/netcode-proof/marks/marks.json'. The helper test's pure case, helper.log '(b)', shows '  M'. The evidence agent noted the same.
- **Fix:** No code change. The report says that the first status line in A3's message is shown trimmed.
