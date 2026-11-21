# Advisor: the ruling on BRAWL-1's P3 re-run set (workflow wf_04e44b32-d5e), 2026-11-15

Written by script, word for word, from the workflow's journal. Its notes: advisor-brawl1-sets-p3.md. Nothing below is the main session's.

## Summary

Verdict: P3 STANDS AND COUNTS. Commit its results, then P2, then P0 (split), with the amendments below.

The P3 re-run set is the fix's proof at P3. It ran at fd46120, whose spikes/netcode-proof tree is ebbfeb40, the same as b30314a. Everyone was in view on every run (1 / 1 / 1; U4r had 0.35 / 0.09 / 0.20). -2's SS part went from FAIL to PASS. -17 L1 P3 went from INVALID to PASS, with 0 mismatches. -57 is unchanged: L1 P3 PASS, and no bite runs in a P3 set, in either version. The building figure is 0 on every run. The logs are 290.7 MiB against 524.4 projected.

One thing came to me that nobody raised: carry 58. Runs s1 and s3 are flagged in runs.txt with "(overflow resyncs: to the advisor before it counts, carry 58)".
- s1 had 27 forced resyncs on 25 clients, at ticks 76,910 to 77,844. s3 had 1, at tick 76,803 (slot 35). s2 had none. U4r's P3 had 0 / 0 / 0.
- All of them fall inside the measured window, which runs from 64,036 to 82,035.
- No client was kicked. room.jsonl holds only the 48 admits, and snapshot_clients_count is 48 on all 18,000 measured ticks of each run.

The cause is the fix itself. With everyone in view, every client queues every hit and kill (room.rs:990-1033). That is about 17.3 events per client per tick, and the room's rate peaked at 554-561 a second in the 30 s bins that hold the resyncs. The provisional queue cap is 1,024 (events.rs:23, 'provisional until U4f measures'). Plan §4.7 sized it from about 350 events a second (net F121). Every snapshot already sits at the 1,000 B window (mean 999.2 B).

Ruling on carry 58: s1 and s3 count for -17 L1 P3.
- The resync is the designed recovery path (catalogue Q8).
- The forced full states' ticks are recorded as incomplete and not compared (§4.6). They cost 228 snapshot client-ticks in s1.
- The resync is triggered by the queue, never by a mismatch, so it cannot hide one. 0 mismatches are recorded on 835,238 / 835,419 / 835,088 compared snapshots.
- Only the runs with resyncs differ from U4r's same-seed network figures: s1 has +178 measured incomplete and s3 +13. s2 equals U4r's s2 exactly (commands 724,309; incomplete 43,210; decided lost at the stop 62).

The capacity finding goes forward as EVQ-LOAD (see next_sets).

Findings:
- **ADV-P3-1 (Medium):** carry 58's flags were not raised.
  - The P3 evidence run is deterministic and identical to l1-p3-s1 (m7-ev.log:125): the same 27 resyncs at the same ticks and slots, and the same -17 report.
  - The card's evidence record ('pass every bar'; '-17's facts: 0 mismatches in both runs; P0 loses no group') omits the resyncs and P3's incomplete count, which condition 14 asked for: 43,295 measured, 44,395 whole-run.
  - The card's 'Open, for the advisor' list and this task's text omit the two set flags. Plan §9 says 'anything else comes to you whole'.
  - No effect: nothing counted, and the set stopped for this ruling anyway.
  - Fix: add both figures to the card's evidence record. From now on, check every runs.txt flag and every summary-line figure against U4r's same-seed line, not only the verdict lines.
- **ADV-P3-2 (Low, my error):** M7-3 cannot hold as I wrote it.
  - The sets run outside main-run.sh (plan §7.1 item 6; c-u4r-runs.mjs:53-84), so no set log carries a diff/untracked header.
  - Its 'tree ebbfeb40' contradicts my own r1 M7, condition 15 and plan §7 (:235): each set's results are committed under spikes/netcode-proof/results/ before the next set.
  - Corrected under conflict.
- **Low:** P3's pre-launch readings (pgrep, porcelain, df) exist only on the card. That is ADV-M6-L1's lesson again: keep them as command output in a log. The amended check below does that.

What I checked myself (read-only):
- the set log, both runs.txt files and both verdict.json files;
- the raw lab.jsonl and room.jsonl of all three runs (resync events, phases, slots, kicks, snapshot sizes, -57 sample counts, events per second), and U4r's P3 s1 raw logs;
- the evidence run's logs and verdict;
- the trees of b30314a, fd46120 and HEAD (all ebbfeb40). `git diff --name-only b30314a HEAD` lists only .checkpoints/ files. `git diff --quiet b30314a HEAD -- spikes/netcode-proof ':!spikes/netcode-proof/results'` exits 0;
- the runner (sha256 99dd070e) and its start and pre-judge checks.

I changed nothing in any tree. My notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-sets-p3.md.

## Condition 15 for P3

Condition 15, applied to P3.

1. **Verdict and bite differences against U4r's P3** (results/u4r/0f066338dd8d/P3/verdict.json):
   - **-57: none.** L1 P3 is PASS in both sets, with all three runs valid. The mark overall is NOT-RUN in both, since its P0, P2 and B1 parts are not in a P3 call. Its bites (pos-4cm, yaw-12bit, pitch-8bit) are not_run in both, because a P3 set carries no bite. Nothing about -57 comes to me whole.
   - **The rest, all as plan §9 expects:**
     - -2 SS runs: FAIL to PASS (everyone in view 0.352778 / 0.085444 / 0.203556 to 1 / 1 / 1).
     - -17 L1 P3: INVALID (pre:PM-0001-2) to PASS.
     - The marks overall: -2 FAIL to NOT-RUN and -17 INVALID to NOT-RUN. The other parts are unrun in a P3 call, so this is expected.
     - -17's bites: not_run in both sets.
   - The one thing outside the expected lines is the carry-58 resyncs. They are ruled in the summary: s1 and s3 count.

2. **-57's values beside U4r's** (µm and µdeg, runs s1 / s2 / s3):
   - position max error: 6,952 / 6,963 / 6,970, against U4r's 6,984 on every run;
   - position p99: 5,111 on every run, both sets;
   - yaw max error: 2,750 on every run, both sets;
   - pitch max error: 88,063 on every run, both sets.
   - **Largest relative change:** s1's position max error, 6,984 to 6,952 µm, -0.46 %. The others are s2 -0.30 % and s3 -0.20 %; yaw, pitch and the p99 do not move.
   - **Recorded fact, decides nothing:** -57's position samples fell. In s1 there are 3,517,033 now against U4r's 13,882,157; before tick 70,336, where U4r still had everyone in view, 1,785,967 against 3,739,269. lab.jsonl is 42.6-46.9 MB against 122.9-127.2 MB. The cause is not established: either fewer moving entities under the leash, or less room in the full 1,000 B window.

3. **The building figure:** 'in a building: 0 slot-ticks (slots none)' on l1-p3-s1, s2 and s3 (runs.txt summary lines). It is also 0 on both evidence runs.

4. **The raw-log bytes against the projection:**
   - The raw-log folder holds 304,793,576 B, which is 290.7 MiB (the runner's on-disk line). The projection was 524.4 MiB, so the ratio is 0.554.
   - Per run: 98,642,608 / 102,899,955 / 102,859,801 B.
   - U4r's P3 had 543,998,603 B (518.8 MiB).
   - The judge peaked at 973.2 MiB against 982 MiB projected from the bytes on disk. That is -8.8 MiB, inside the fit's residuals of -90 to +70.
   - Plan §10's 'bytes grow with everyone in view' went the other way. The projections stay conservative.

5. **TICK16-WORDING:** it does not apply to P3. The tick16 bite is a P0 bite, and the P3 set has none. The P3 line that applies is plan §9's '-17 L1 P3 PASS with 0 mismatches', and it is met. The wording holds for P0 unchanged (see next_sets).

## The M7-3 conflict

Ruling: M7-3 is amended, and the corrected reading replaces my words. 'At the fix's commit' means the code under test equals b30314a's. It does not mean the whole spikes/netcode-proof tree stays ebbfeb40, and it does not mean set logs carry main-run.sh's header, which they cannot.

**M7-3 (amended):**
- (a) For a set to be valid, its HEAD must satisfy three conditions:
  - `git diff --quiet b30314a HEAD -- spikes/netcode-proof ':!spikes/netcode-proof/results'` exits 0;
  - every line of `git diff --name-only b30314a HEAD` is under .checkpoints/ or under spikes/netcode-proof/results/u4r/<c12>/<P>/, and each such folder is a BRAWL-1 re-run set's;
  - `git diff --name-status b30314a HEAD -- spikes/netcode-proof/results` shows only A (added) lines.
- (b) The set's runs.txt head names that commit and its spike tree.
- (c) The runner's own start refusals and its pre-judge re-check stand in for the diff/untracked header that set logs never carry:
  - at start it refuses any uncommitted tracked change and any untracked file under crates/ (c-u4r-runs.mjs:401-406);
  - before the judge it re-checks the spike's tree and tracked status (:181-196);
  - committing the previous set before the next one starts is exactly the order this re-check is built around (INF-R2-1).
- (d) The evidence logs keep M7-3's original form, and both already meet it.
- Any other value voids the run, and it comes to me whole.

**P3 under the amended M7-3: valid.** I checked it myself:
- fd46120's tree is ebbfeb40, as is b30314a's;
- b30314a..HEAD touches only .checkpoints/;
- the runner started, so there was no tracked change and no untracked file under crates/;
- it printed no stop at the pre-judge re-check.

**Commit P3's results now**, as their own commit:
- exactly the two untracked files: runs.txt (sha256 5361fd539c60…) and verdict.json (sha256 21c9abbf74ea…, equal to the raw-log folder's copy);
- the card may ride with them;
- the commit's diff (`git diff --name-only HEAD^ HEAD`) touches only spikes/netcode-proof/results/u4r/fd4612050cd6/P3/ and .checkpoints/.
- P2's results will then land under a new <commit12> folder (the runner uses HEAD's commit; A6 keeps the runner unchanged). The report maps each set to its folder.

**What the main session checks before each set** (P2, then P0): one read-only main-run.sh call, logged as .gate/u4r-sets/pre-<P>.log, so every reading is command output (M7-2):
1. The header shows `diff e3b0…; untracked e3b0…`. This proves the previous set's results are committed and nothing under spikes/netcode-proof is modified or untracked.
2. `git rev-parse HEAD HEAD:spikes/netcode-proof`.
3. The three checks of (a). From the spike folder, the first one is `git diff --quiet b30314a HEAD -- . ':(exclude)results'`; it must exit 0.
4. `sha256sum ops/runs/c-u4r-runs.mjs` gives 99dd070e…, and main-run.sh f0b7a3d0 and u4r-run.sh 2f754d12 are unchanged (A6).
5. `pgrep -a` for cargo, rustc, npt-lab, npt-judge and loader.mjs finds nothing, and no agent is running.
6. `df -B1 /home/user` shows at least the set's start figure plus 4,096 MiB: P2 needs 4,877 MiB (780.8 + 4,096), P0 needs 7,711 MiB. 11,757 MiB is free now.

Then launch exactly as P3 was launched. After the set:
- runs.txt's commit equals the pre-check's HEAD, or a later commit whose changes since touch only .checkpoints/;
- its tree equals the pre-check's tree;
- no stop line appears.

## The next sets

**P2: GO**, after P3's results commit and P2's pre-check (see conflict).

P2's expected lines:
- -2 SS PASS on every run (everyone in view at 0.99 or more);
- -17 L1 P2 PASS with 0 mismatches;
- -57 L1 P2 PASS, its values recorded beside U4r's P2 (results/u4r/ab10f058a828/P2/) with the largest relative change named. P2 has no bites; any -57 verdict difference comes to me whole;
- the building figure 0 on every run;
- the bytes against the 780.8 MiB projection.

Overflow resyncs at P2 (100 ms, 2 % loss) are now possible inside the measured window, from the full-view load. For each flagged run, the main session records, by print-only `python3 -I` reads:
- the resync count, first and last tick, phases and distinct slots (lab.jsonl overflow_resync events);
- that no client was kicked: room.jsonl has no event but the 48 admits, and snapshot_clients_count is 48 on every measured tick;
- -17's 0 mismatches.

If all of that holds, P0 may follow without waiting for me, and the flagged P2 runs come to me with P0's verdict, before -17 L1 P2 counts (carry 58). These stop the lane and come to me whole: a kick, a resync in warmup, a mismatch, -2 FAIL, a building figure above 0, a -57 difference, or anything else outside these lines. P2's -57 values go to the report.

**P0 (split, as U4r-F2): GO after P2**, on the same pre-check, with P0's 7,711 MiB floor. P0's expected lines:
- -2 SS PASS on every run;
- -17 L1 P0 PASS, with 0 mismatches and 0 measured incomplete on the measured runs;
- -17's three bites shown: tick16; client-rounding; whole-as-lost with measured incomplete 48;
- -57 L1 P0 PASS, with its three bites shown, each failing only its own check;
- -57's values recorded beside U4r's P0: 6,984 / 2,750 / 88,063, p99 5,719 / 5,718 / 5,715, bites pos-4cm 34,830, yaw-12bit 46,692, pitch-8bit 440,312. Any -57 verdict or shown-bite difference comes to me whole;
- tick16 as TICK16-WORDING: 'one overflow resync per client, all in the after phase, none in the measured window, no kick'. It has no fixed offset. At the full-view rate I compute the resyncs at about wrap + 54-60, earlier than U4r's wrap + 79. That is still after the measured window's end, 65,550, and it is no deviation;
- 0 overflow resyncs on P0's measured runs: P0 acknowledges every snapshot, and the P0 evidence run had 0. One on a P0 measured run comes to me whole;
- the building figure 0 on every run, bite runs included;
- the bytes against 3,614.6 MiB.

**Unchanged:**
- A floor stop is reported and never cleaned around.
- TE-B1-R2-2's runner fix comes after P0 (A6).
- KEPT-LOGS stays open with Mustafa.

**Carry line EVQ-LOAD**, for U4f's and U4g's briefs, recorded on the card and in U4r's report:

"With everyone in view, every client queues every hit and kill. The stress scene's room-wide reliable gameplay events run at about 460-560 a second at P3 (30 s bins), about 17.3 queued per client per tick, against the provisional cap of 1,024 (events.rs:23). Plan §4.7 sized the cap from about 350 a second (net F121). Every snapshot sits at the 1,000 B window (mean 999.2 B). In BRAWL-1's P3 set, the cap overflowed inside the measured window: s1 had 27 forced resyncs on 25 clients at ticks 76,910-77,844, where the room's rate peaked at 554-561 a second; s3 had one at 76,803; s2 none. There was no kick and 0 mismatches. B1 adds 1 s of total loss on P2's backlog, about 520-570 more events per client. So net F121's warning is live for U4g's judged B1 runs: a cap too low resyncs every client on every burst and changes -18's heal path. U4f's cap measurement (§4.7) uses this full-view rate. U4g's plan reads carry 58's per-burst high-water marks from a short B1 run before -18's judged B1 runs count."

## The report's figures

BRAWL-1 re-run set P3. Commit fd4612050cd6, spikes/netcode-proof tree ebbfeb40. Runner c-u4r-runs.mjs 99dd070e. Log .gate/u4r-sets/P3-fd4612050cd6.log (sha256 cc2f626c…), exit 0, wall 1,020.5 s. Results: results/u4r/fd4612050cd6/P3/, runs.txt 5361fd53… and verdict.json 21c9abbf…. All figures are listed for runs s1 / s2 / s3.

**Verdicts against U4r's P3** (0f066338dd8d):
- -2 SS runs: FAIL to PASS. Everyone in view went from 0.352778 / 0.085444 / 0.203556 to 1 / 1 / 1. Shots per second 693.18 / 693.26 / 693.37 (U4r 695.02 / 695.09 / 695.22).
- -17 L1 P3: INVALID (pre:PM-0001-2) to PASS.
  - 0 mismatches of 835,238 / 835,419 / 835,088 compared.
  - The judge counts 44,395 / 44,253 / 44,535 client-ticks incomplete over the whole run; npt-lab counts 43,295 / 43,210 / 43,454 in the measured window.
- -57 L1 P3: PASS = PASS.
- Bites: none in a P3 set, in either.

**-57 beside U4r's** (µm and µdeg):
- position max error: 6,952 / 6,963 / 6,970 against 6,984 on every run;
- position p99: 5,111 on every run, both sets;
- yaw: 2,750 on every run, both sets;
- pitch: 88,063 on every run, both sets.
- Largest relative change: s1's position max error, -0.46 %.
- Position samples in s1: 3,517,033 against 13,882,157.

**Building figure:** 0 slot-ticks on every run.

**Carry 58:** overflow resyncs 27 / 0 / 1, against U4r's 0 / 0 / 0.
- s1: 25 clients, ticks 76,910-77,844, all in the measured window. s3: slot 35, at tick 76,803.
- No kick: 48 snapshot clients on every measured tick.
- s1 lost 228 snapshot client-ticks to the forced full states.
- The advisor rules that s1 and s3 count for -17 L1 P3.

**Same seeds as U4r:** s2's network figures are identical (commands 724,309; measured incomplete 43,210). s1 and s3 differ by +178 and +13 measured incomplete.

**Bytes:** 290.7 MiB (304,793,576 B) against 524.4 MiB projected, a ratio of 0.554. U4r's P3 had 518.8 MiB. The judge peaked at 973.2 MiB against 982 MiB projected. Free space afterwards: 11,757 MiB.

**TICK16-WORDING:** not applicable to P3 (no tick16 run).

**The evidence run brawl1-ev-p3-s1** is identical to l1-p3-s1: 27 resyncs at the same ticks and slots, 835,238 compared, 44,395 incomplete.

**Advisor corrections:**
- ADV-P3-1 (Medium): the flags were not raised.
- ADV-P3-2 (Low, the advisor's): M7-3's wording.

## For Mustafa

For Mustafa: nothing.

In plain words:
- The bot fix works on the first of the three re-run sets, the one with the worst network. Every bot stayed in view the whole time, none entered a building, and the snapshot check that was blocked before now passes.
- The run also showed that the server's queue for game events is too small now that every player sees every hit. Under heavy network loss, about half the clients had to be resent the full game state once, for a moment. Nothing broke: no client was dropped and no snapshot went wrong. This is a sizing question for the next test steps, not something you need to decide.
- I also corrected one of my own instructions, which could not be followed as written.
- The other two sets run next.

Decisions needed: none. KEPT-LOGS, the kept-logs decision, is still open with you, unchanged.

Nothing here is something a player sees, hears or feels. This is test-branch spike work, not a merge to main.
