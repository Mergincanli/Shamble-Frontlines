# Advisor: the review of U4r-F2's pushed head (workflow wf_af1fbc08-0bd), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-u4r-f2-head's StructuredOutput result). Its notes: advisor-u4r-f2-head.md. Nothing below is the main session's.

## Verdict: APPROVE

**Head:** 791bc553ad75d0ea1c2f060697898ba7cfa28247 (U4r-F2, PR #40 draft, branch claude/netcode-proof-test-plan-hcky5g). I approve it with no push. Three commits came later, and each touches only .checkpoints/: 208d7dc, f998e74, and the saver's notes saves. The script meets conditions 1 to 9. The evidence holds up when I check it myself. CI is green. In plain words: the biggest test set (P0) is now judged in two halves. Each half needs about 5.6 and 6.2 GiB of memory, under the 10 GiB line. A direct test on the real judge shows that one pass and two halves give the same verdicts. Risk tier 1: a spike ops script, with no game, kit or gate file. Nothing a player sees changes, and this is not a merge to main. One note: the space cleans (B) and (C) ran at 21:17 to 21:18 UTC, before this review, although the card's own line said 'if APPROVE and CI is green'. My sets ruling's step 6 ties them only to condition 7's result (at most 10 GiB), and they only lend cache space, so no rule is broken and nothing was harmed. It is recorded here and is not a finding.

## Checks

1. Git: 791bc55 changes spikes/netcode-proof/ops/runs/c-u4r-runs.mjs (52 insertions and 22 deletions, which matches the card's SIZE-CHECK line) plus three .checkpoints files. All four files are mode 100644: no symlink, and no .github/ or gate file. The commits after it (208d7dc, f998e74) touch only .checkpoints/. Since P2's results commit 1eacb60 (20:22:31 UTC), 791bc55 is the only commit under spikes/, and the script was edited at about 20:26. Condition 1 is met.
2. Tested copy = committed copy. The script's sha256 at 791bc55 is 18c1dcf6..., the same as sources.log's F2 copy and the main folder's file now. The sha256 at 791bc55~1 is c5cd4d9c..., the same as sources.log's HEAD copy. The shims, compare.py (56be2e1d) and the helper test (ecfc8c84) match sources.log. harness.mjs does not: it was edited at 21:16:18 for INF-F2-3's NOTES path, after the 21:01 run, so it now reads d0dd4ec3 against the run's 99f583a2. That is ADV-F2H-1, a Low.
3. Condition 2, read in the code: judgeCall runs unchanged(what) before each bounded call. The calls run in order, measured then bites. A throw in either re-check comes before mkdirSync(results), so a stop leaves no results/ folder. Each verdict file is copied only on its own call's exit 0.
4. Condition 3, read in the code. (a) The judge lines read 'judge (measured runs): ...' and 'judge (bite runs): ...'. (b) SPLIT_LINE matches the condition's text word for word and sits directly above them. (c) The raw-logs line names 'measured/ and bites/'. (d) The exit line is the catch's 'c-u4r-runs: ' plus '<what> <end>' for each failed call, joined by '; ', then ': runs.txt written, with no verdict from that call|those calls (to the advisor)'. (e) A set without bites keeps HEAD's re-check name 'the judge', its judge line, its exit text and its written line word for word.
5. Condition 8: the header holds INF-F1-1's sentence word for word as written at advisor-u4r-f1-head-full.md:33, right after the flock sentence. Layout: npt-lab creates <logs>/<group>/<run> itself (scenario.rs:771, create_dir_all), and the judge's log_files reads .jsonl files at any depth (judge/src/lib.rs:19-34), so neither marks.json nor verdict.json at <logs> is read.
6. Conditions 4 and 6, the stagings: summary.log lists 17 stagings, all PASS, and I read each one's CHECK lines. cmp-P0 (20 of 20) covers the planned layout; npt-lab argv unchanged except --out; two judge calls with the same --marks and --only (PM-0001-2 named in both); measured first with no overlap; runs.txt differing only in its 3 allowed lines. cmp-P2, cmp-P3, cmp-B1 and cmp-precheck match HEAD. cmp-r2 also matches HEAD: 14 judge argvs, and r2.txt identical once times are masked. The fail-measured, fail-bites and fail-both stagings each show condition 3(d)'s exact line and which verdict file is kept. fail-P3 shows HEAD's exit line. lim-measured and lim-bites show 'stopped at 2.0 s', the other call still running, and the group killed. recheck-bites-edit and recheck-bites-commit show the stop naming 'the judge (bite runs)', call 1 run, call 2 not run, and no results/.
7. Condition 5, checked myself with a python3 -I key-by-key read of real/*.json. For -2, -17 and -57, the measured-alone file's parts, report, verdict and title equal the combined file's, and the bites-alone file's bites equal the combined file's. -17's tick16 and whole-as-lost bites read shown in both. r2's sample exercises only 2 of P0's 6 bites; for the other four the proof rests on the judge's code, which infra-engineer checked (figures.rs). The words in SPLIT_LINE hold: in the bites-alone file every part reads '0 of n valid runs', and in the measured-alone file every bite reads 'no run yet'. The last clause of condition 5 is open: the /dev/shm copies are still there because the rm was denied (INF-F2-3, Low).
8. Condition 6: redact.log shows 284 items and 0 findings. gate-no-smoke.log shows GATE PASS. On GitHub, CI on 791bc55 passed: gate workflow run 43641942150 (job gate success, job smoke success) and toolchain run 43641942137 success. 791bc55 changes no .github/ or gate file, so CI's green stands. I ran no gate myself, since this task allows no script runs. This approves the head on the spike branch, not a merge to main.
9. Condition 7, recomputed from the four judge-apart logs. All four use judge 65acacfe and marks 9fb5be57. My least-squares fit is 212.3 + 2.5062 x MiB (the card's is 212.5 + 2.5059 x; residuals within 10 MiB). F = max(1.44, 1.504, 1.239, 1.5) = 1.504. The measured call: 2,190.7 MiB of logs, 5.57 GiB of judge memory, break-even F 2.75. The bites call: 2,431.7 MiB, 6.16 GiB, break-even F 2.47 to 2.48. One call would have been 11.52 GiB. R = 9,218.6 MiB. The card's figures hold. The runs.txt heads of P3, B1 and P2 all show b3b4a60d / 33e1f49f / 65acacfe / 9fb5be57.
10. infra-engineer's one round (u4r-f2-review-full.md): I agree with all three severities. INF-F2-1 (Medium) is fixed in the card's run procedure item (6) in its own words, with no code change. INF-F2-2 (Low: three comment wordings) and INF-F2-3 (Low: housekeeping) go to the runner's next change or need no push. The round count is one, which fits the budget.
11. FP-P2, checked myself with python3 -I print-only reads of P2's room.jsonl (s2, s3), P3 s1 as a control, and lab.jsonl's client_counts. The details are under fp_p2. Overflow resyncs are 0 and rewinds refused are 0 on all 48 slots of all three runs. The committed verdict.json reads: -2 SS FAIL (0.5875, 0.7295), -17 L1 P2 INVALID by pre:PM-0001-2 on s2 and s3, -57 L1 P2 PASS.
12. Not checked: I did not run the harness, the gate, check.mjs or any binary, as this task allows. I could not compare the run's harness byte for byte, because no copy of 99f583a2 survives. infra-engineer's independent re-run of 15 stagings used that copy before the edit.

## ADV-Q: INF-F2-1's header sentence

At the runner's next change, not now. INF-F2-1's sentence goes into c-u4r-runs.mjs's header in infra-engineer's words, right after INF-F1-1's sentence, at condition 8's home: the one runner change after the P0 set, plan first, before the next judged set after U4r's. It goes together with INF-F2-2's three rewordings (also in infra-engineer's words), INF-F1-5, RUNS-LINE and RATES-1. Why: the binding stop rule is the card's run procedure item (6), and it already carries INF-F2-1's fix word for word. The header is complete for every set without bites. For P0 it is incomplete, but the card covers that, and the main session runs P0 from the card. A comment-only commit now would put an untested copy (a new sha256) in front of P0, and it would need its own CI and a head review. The review budget does not push for a Low or for a Medium already fixed elsewhere. This is my F1 ruling again (advisor-u4r-f1-head-full.md, INF-F1-1's header sentence). The report's follow-ups list adds the two items (INF-F2-1's header sentence, INF-F2-2's wordings) under the runner's next change.

## FP-P2: P2's fingerprint

Ruling: it is BRAWL-1's fingerprint. The same cause shows, with two differences that the logs explain, and nothing of a netcode fault. -17 L1 P2 INVALID is recorded, never counted, and moves with -17 L1 P3 to BRAWL-1's re-run at the fix's commit (before U4g's judged B1 runs), with no stop. What I found in the logs myself:

(1) Onsets. s2's is at tick 73,816, phase 0 on the 360-tick door cycle, a door's close edge. s3's is at 76,325, phase 349, inside door 5's closed half (330 to 150), so a soldier walked into a building whose door was already shut. On each onset tick all 48 slots drop at once, and every slot held on all 9,780 and 12,289 ticks before.

(2) After the onset, every closed half has all 48 slots at 0 in both runs: 0 to 24 slot-ticks of 1 per cycle after a 20-tick margin, out of 7,680.

(3) s3: one soldier in one building. Slot 33 is the slowest in all 15 cycles: its own view comes back 53 to 162 ticks after door 5 opens, while the other 47 slots come back in a median of 2 to 3 ticks (at most 3 to 30). Its 842 held ticks are all in the door-open half.

(4) s2: two soldiers in two buildings. In cycles 1 to 4 the inside soldier is slot 6: its view returns in 70 to 78 ticks, while the others take a median of 1 to 3 and slot 28 takes 2 to 4. From the cycle at 75,436 on (18 cycles), no other slot holds before 60 to 63 ticks into door 0's open half in any cycle. That is door 2's open edge (60 ticks later) plus 1 to 3 ticks of re-add. So a second soldier (slot 28) went into a second building (door 2) and stays door-gated: slot 28 never holds in 8 of those cycles, or holds 114 to 177 ticks in. That is why slot 28, not slot 6, has the least share, and why the next least (slot 6, 10,892) sits close to it.

(5) The difference from P3 is explained by P2's lower loss. At P2 the re-adds finish inside the 180-tick open half: the others' median is 1 to 3 ticks against P3 s1's 4 to 45, and the inside client's own view returns in 53 to 177 ticks, where P3's slot 44 never returned in 27 of 32 open halves. So the least slot holds on 795 of 8,220 and 842 of 5,711 ticks after the onset, every one of them in the door-open half (plus a few ticks of link delay at the closing edge), against P3's 4 to 50. My Q1 (3) wording, 'one slot at 0 from an onset tick on', is to be read as: one slot per soldier inside a shut building; that slot is at 0 on every closed-half tick and holds only late in the open half.

(6) Overflow resyncs 0 (lab.jsonl client_counts, all slots); very large units held in 1.0000, 0.9839 and 0.9937 of sets.

What goes in the report's P2 section:
- -57 L1 P2 PASS with its values.
- -2's SS part at P2 recorded FAIL: s1 1.0; s2 0.5875; s3 0.7295.
- -17 L1 P2 INVALID by pre:PM-0001-2 on s2 and s3 (s1 valid), recorded and never counted, with points (1) to (6) in short: both onsets and their door phases; all slots at 0 in every closed half; s3's single slot (33) and its re-add times; s2's two soldiers (slot 6 from the onset, slot 28 behind door 2 from about tick 75,436 on, the others waiting the 60-tick door-2 gap); the P3 comparison and why; overflow resyncs 0; very large units held.
- The P2 fingerprint was checked by the main session's brawl_fingerprint.py and confirmed by my own reads, and this ruling is cited.
- The new home is BRAWL-1's lane, with the re-run of -17 L1 P2 at the fix's commit (as my sets ruling already says, 'and L1 P2 if it reads INVALID').
- s2 shows that more than one soldier drifts, so BRAWL-1's planned run figure must name every soldier outside the cluster, not only the first.
Nothing goes to Mustafa. My sets ruling already told him that P2 might show the same and would be handled the same way.

## PRE-P0

Yes: condition 7 is met and P0 may start now. The four-point fit, F = 1.504, the projected peak of each call (5.57 and 6.16 GiB) and the break-even F (2.75 and 2.48) are on the card, and I reproduced them. R (9,218.6 MiB) is already met: (B) and then (C) took free space from 8,138 to 16,492 MiB, about 7.1 GiB above R, and (C) was needed, since (B) alone gave only 8,618. My Q3 order (B, then C only if short) was followed. At P0's start, the run procedure applies, plus these points:
(a) The tree is clean. The script's start check reads the whole repository's tracked status, and this notes file is now tracked, so commit and push the card and notes first.
(b) pgrep -a npt-lab and pgrep -af npt-judge return nothing.
(c) df is at least R right before the start.
(d) Between now and P0's start, no check.mjs run, no shared-target build and no other lane. Any of them refills about 9 GiB and would break R.
(e) The run procedure's items (1) to (7), including item (6)'s P0 sentence on stopping a set.
Condition 7's sha256 rule applies to P0's own runs.txt head: npt-lab measured b3b4a60d, npt-lab bites 33e1f49f, npt-judge 65acacfe, marks.json 9fb5be57. Any difference comes to me before P0 counts.

## Findings

### ADV-F2H-1 (Low)

- **Where:** .gate/u4r-f2/harness/harness.mjs (now sha256 d0dd4ec3...) against .gate/u4r-f2/logs/sources.log ('harness.mjs sha256 99f583a2...'); the card's line 'The harness's NOTES path now points at .gate/u4r-f2/scratch/'
- **Wrong:** The kept harness is not the harness that produced the 17 staging logs. It was edited at 21:16:18 for INF-F2-3's NOTES path, after the 21:01 run, and was not re-run. No copy of the run's version survives, and the card does not say the hash changed. A reader re-checking sources.log against the folder would find a mismatch with no explanation. infra-engineer's independent re-run of 15 stagings used the original before the edit, so the evidence itself stands.
- **Fix:** No push and no re-run. Add one line to the card and to u4r-report.md's F2 evidence: the stagings ran on harness.mjs sha256 99f583a2 (sources.log); the kept copy, sha256 d0dd4ec3, differs by the NOTES re-point to .gate/u4r-f2/scratch/ (INF-F2-3), made after the run; infra-engineer's re-run used the original.

## Next steps

1. 1. Record this review word for word by script (advisor-u4r-f2-head-full.md). Put its For Mustafa ('nothing') on the card at once.
2. 2. Card and report lines, with no push: ADV-Q's ruling (INF-F2-1's header sentence and INF-F2-2's three wordings go to the runner's next change, beside INF-F1-5, RUNS-LINE and RATES-1, in the report's follow-ups list); ADV-F2H-1's harness line; and FP-P2's ruling in the report's P2 section, in the words under fp_p2.
3. 3. Before P0: commit and push the card, so the tracked tree is clean (the start check refuses any tracked change). Check that pgrep is empty and that df is at least R (9,218.6 MiB). Run no check.mjs, no shared-target build and no other lane before the start. Remove /dev/shm/u4r-f2 if the permission allows; it is optional (66 MiB of RAM).
4. 4. Start the P0 set by the run procedure, items (1) to (7), with item (6)'s P0 sentence. Make no commit under spikes/ until it ends.
5. 5. After P0, check runs.txt's head sha256s against b3b4a60d, 33e1f49f, 65acacfe and 9fb5be57; any difference comes to me before P0 counts. Check the fixed line, the two judge lines and the raw-logs line naming measured/ and bites/. Read -17's and -57's L1 P0 parts from verdict.json and their bites from verdict-bites.json. These come to me before they count, per the sets ruling's step 7: a failed or stopped judge call (the exit line), the flagged tick16 run, and any -17 L1 P0 INVALID (that one whole). Commit results/u4r/<commit12>/P0/.
6. 6. Then, under the lane lock, with nothing else running: judge-apart each half (<logs>/measured and <logs>/bites) into scratch with peak.py, and record each call's peak, wall time and read_log share against the fit's 5.57 and 6.16 GiB (RR3-L6). Also record peak.py's whole-set maximum from the kept log.
7. 7. Fill u4r-report.md's P0 section and the card. Then the one runner change after P0 (both scripts, plan first, with my plan review) before the next judged set after U4r's. BRAWL-1 stays in the next npt-lab lane.

## For Mustafa

Nothing for you to decide or check. What happened: the small fix that lets the biggest test set be judged in two halves (about 5.6 and 6.2 GB of memory each, under the 10 GB safety line) is built and tested. I approved it. A direct test on the real judge shows that one pass and two halves give the same answers. The automatic checks passed, and the disk now has room. So that last set can start. The set that ran before it (100 ms delay, 2 % loss) showed the same test-bot flaw as the harshest setting. Bots wandered into buildings, and in one run two of them went into two different buildings. That breaks the 'everyone in view' load condition while the doors are shut. That check is recorded as invalid, never counted, and moves to the bot fix, as I told you it might. The accuracy check in those runs passed. It is a spike-branch step, not a merge to main, and nothing a player sees changes.

Decisions needed: none.

For Mustafa: nothing.
