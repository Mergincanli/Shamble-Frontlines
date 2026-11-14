# Advisor: the review of the pushed head 71ee13d, U4r's merge into the spike branch (workflow wf_ed59da85-818), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-u4r-merge's StructuredOutput result; the run was cut by the spend limit at 14:14 and resumed at 15:32). Its notes: advisor-u4r-merge.md. Nothing below is the main session's.

## Verdict: APPROVE

Reviewed: U4r's commit 71ee13d29dc66fdbca682ce4a1db96044d264c0d (one parent, 5e0eae1) and the branch head 1284dad4013b171ae2debaf5d110464bce09d258. The five commits after 71ee13d (94dc9e3, f0a1e60, 9830132, e5bd633, 1284dad) each touch only .checkpoints/: the card or my notes file. origin/main is still 4460ef5. Risk tier 1 (spike code plus .checkpoints/). This is a commit on the spike branch, not a merge to main, and nothing a player sees, hears or feels changes. The spikes/ change is byte for byte the patch reviewed in round 2. M1 to M9 hold against the logs, and I re-checked the key figures myself. CI is green on 71ee13d. The report is accurate and complete against step 9's list, with three Low gaps that ride its next edit. No push is needed for them.

## Checks

- Patch bytes: git show 450af1a:.checkpoints/netcode-proof-test/wip/u4r.patch has sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce, the reviewed bytes, and so does the parent of 71ee13d.
- spikes/ equals the patch: I split `git diff --no-renames 71ee13d^ 71ee13d -- spikes/` per file and compared it with the patch. Both have 26 files, with the same names, and all 26 bodies are identical (index lines aside). 13 added and 13 modified, all mode 100644, no symlink or submodule, no marks.json path.
- Other paths: .checkpoints/netcode-proof-test.md (M), u4r-build.md (M), u4r-report.md (M; drafted at 5e0eae1), wip/u4r.patch (D). Nothing else.
- Base (M2): spikes/ at 71ee13d^ equals 2fbec77's, and the commits in between touch only .checkpoints/. origin/main is 4460ef5, not merged in (MAIN-52).
- Merged bytes (M4): I compared each of the 26 files at 71ee13d with .gate/u4r-snap2's copy (26 of 26 equal) and with the working tree (26 equal). I did not re-run the builder's tree fingerprint, because my seat runs no script. The byte comparison covers the same bytes.
- Counted run (M5, .gate/u4r-merge/m5.log, 13:44:35-14:06:39 UTC, exit 0): lane lock exclusive. `uptime` ran first (up 3:09, load 0.27, 0.09, 0.10). Every provenance line is under /home/user/Shamble-Frontlines/spikes/netcode-proof/crates/. check.mjs reports 'check: all 11 steps pass': test 531 and 609, 0 failed, 0 skips; self 77 of 77; marks 81 with the ADR rows unchanged; redact 0 findings. The deps line matches c6-end2.log:173. The wasm line (43.5 s) matches c6-end2.log:174 apart from the times and the native hash-map digests, which vary from run to run because that bite plants hash-map order; the catch points are as expected.
- det-u4 against 240 s: deps PASS; det-u4 PASS, 18,000 ticks and 0 mismatching on both comparisons. Non-cargo sum: npt-lab hash 223.8 + Wasm 2.5 + 0.9 + compare and h2 0.0 = 227.2 s, under the 240 s limit (round-2 base 223.7 s). The step's 240.3 s total includes the builds (11.6 + 1.4 s).
- Regeneration: m5.log prints 'regenerated SS-P0-s4: 16950444 bytes, sha256 6933b56a...bab3, as the manifest's', and the native legs are equal over 9,000 ticks. I hashed .gate/u4r-merge/regen/rec/SS-P0-s4.rec myself: 6933b56a...bab3, equal to precheck-recordings.json at 71ee13d.
- TE2-5 (M6): no test failed in either build, so nothing was re-run and TE2-5's rule ends at this merge (TE-Q2), as recorded.
- Gate (M7): in gate.log every check passes except 5a smoke, which could not start: the tools look for chromium_headless_shell-1194, and /opt/pw-browsers holds 1243 (I listed the folder). gate-no-smoke.log ends 'GATE PASS (5a smoke skipped: --no-smoke)'. My limits allow no gate run of my own.
- CI on 71ee13d, read by me through GitHub: gate run 43603457356 (job gate success; job smoke success, including its step 'Smoke (check 5a)'), toolchain run 43603457370 (job toolchain success), all with head_sha 71ee13d, attempt 1. The diff changes no .github/ or gate file, so CI's green stands.
- u4r-build.md's 'Merge into the spike branch' (lines 2113-2137): every figure in M1-M8 matches the logs. The card commit 0eded3c is m5.log's HEAD.
- Report (u4r-report.md, 220 lines): every figure, count and line reference I checked holds: the numstat, both rounds' counts and seat verdicts, every advisor-u4r-r2-full.md reference, the history.md references to Mustafa's SIZE-U4R and SIZE-CHECK words (the main session's reading labelled as its own), and the u4r-build.md and code references. Everything step 9 asks for is present, including round 1's 12 FP-9 item 4 follow-ups and all 13 of round 2's findings, each with its home.
- M9: the card at 1284dad records the merge at 71ee13d, 531/609/77, TE2-5's end, the follow-ups and SEC-U4R-R2-1 as open. The saver was restarted (b02nlg0ph). Copy 6's worktree and the .gate/u4r logs are kept.

## Findings

### MRG-L1 (Low)

- **Where:** .checkpoints/netcode-proof-test/u4r-report.md:95 (and :88)
- **Wrong:** Line 95 says det-u4's non-cargo 227.2 s ran 'at load average 0.27'. That uptime is the main-run.sh call's first command (13:44:36). det-u4 started at 14:00:17, after check.mjs's 13.7 minutes of builds and tests, so 0.27 is not the load at det-u4's start. The base it is compared with (1.32, c6-det-u4.log) was taken at det-u4's own start, so the two loads cannot be compared. Line 88 calls the provenance counts (npt-lab 5, npt-bots 5, npt-hash-wasm 4, npt-sim 7) 'the six builds'', but they count the whole call: seven builds, including the regeneration's npt-lab build. The 240 s bar itself holds.
- **Fix:** Line 95: 'under 240 s; the call's uptime at 13:44:36, 15 min 41 s before det-u4 started (after check.mjs): load average 0.27, 0.09, 0.10; the load at det-u4's own start was not measured'. Line 88: 'counted over the whole call (seven builds, the regeneration's npt-lab build included)'.

### MRG-L2 (Low)

- **Where:** .checkpoints/netcode-proof-test/u4r-report.md:125-127
- **Wrong:** Carries 20 and 69 are left 'not verified here' and handed to this review, so the report's 'carries met, by number' is incomplete. I checked both. Carry 20 is met: crates/lab/src names grid::Config only at mirror.rs:23 and :152 (a local Config::default(), used by value), and replay's re-decoder takes npt-client's own grid_with_bite (replay.rs:8-10); nothing hashes, records or compares a Config. Carry 69 is met: Recording::synthetic appears only in hash.rs's regeneration (hash.rs:384), and neither crates/lab/src nor the run scripts report an ES byte or bandwidth figure; the scripts' 'bytes' are disk sizes (c-u4r-r2.mjs:55-78).
- **Fix:** Move 20 and 69 into 'Met by U4r's code' with those references, citing this review (advisor-u4r-merge.md, C13 (b)), and delete the 'Not found by number' line.

### MRG-L3 (Low)

- **Where:** .checkpoints/netcode-proof-test/u4r-report.md:52-57 (the round-1 bullet) and :173
- **Wrong:** Round 1's FP-9 item 3 has npt-lab's report name EA-U4R-2 as a deviation from U4-SEAM (u4-build-plan.md:162-163). That deviation appears only in u4r-build.md:1930-1934. The report lists the follow-up (the Sim type and one server constructor, :173) but not the deviation itself, so U4's report, which folds this one, could lose it. Sites re-checked at 71ee13d: npt_sim::World named at logs.rs:97, :472, :595 and precheck.rs:296; Room::new at scenario.rs:751 and precheck.rs:301.
- **Fix:** Add one sentence to the round-1 bullet: 'EA-U4R-2: a deviation from U4-SEAM (u4-build-plan.md:162-163): npt-lab names npt_sim::World at logs.rs:97, :472, :595 and precheck.rs:296, and builds the room twice (scenario.rs:751, precheck.rs:301); its follow-up is below (u4r-build.md:1930-1934).'

## Next steps

1. Record this verdict word for word by script (advisor-u4r-merge-full.md). Put its For Mustafa ('nothing') and the three Lows (MRG-L1 to MRG-L3, homed in the report's next edit) on the card. Refresh the card's stale lines: 'Last update' still reads 11:25, and 'Next: M9' is done.
2. Commit and push my notes file and the card. Then run step 11, the r2 record run from the merge commit (`node ops/runs/c-u4r-r2.mjs --date <today>` through main-run.sh, lock exclusive), under INF-R2-1's interim rule: card pushed right before, saver stopped, no agent writing in the main folder, no commit or tracked edit there until r2.txt is written; then save and restart the saver. b1, es and b1-nofallback are expected to fail as ruled: record them, never re-run them. Any 'stopped at' or 'exit by a signal' line comes to the advisor whole.
3. After r2.txt is written, fold MRG-L1 to MRG-L3 into u4r-report.md in the same edit that fills its judged-runs section. That is report wording only: no new review round, and no push of its own.
4. Then the round-2 ruling's steps 12 to 17 in order: carry 62 (h) from r2; U4r-F1 (plan first, then the advisor's plan review, the build, infra-engineer's one round, the advisor's review of its head) before any c-u4r-runs.mjs set; D4; the disk plan in writing; the pre-check set; the judged sets P3, B1, P2, then P0. Step 18's follow-ups wait for their homes. Copy 6's worktree stays until F1's evidence is in.
5. For PR #40 later: this review read CI and the main session's gate logs, because my limits allowed no gate run. The day-session advisor runs the full gate itself on the head of PR #40 before any merge to main, and SEC-U4R-R2-1 keeps PR #40 needs-mustafa until it is fixed.

## For Mustafa

Nothing for you to decide or do. U4r (the lab's runner for the L1 test network, its pre-check and the run scripts) is now on the spike branch, and I checked it independently. The code that landed is exactly the code reviewed in round 2, file for file and byte for byte, with nothing extra. The counted run on the merged code passed all 11 checks: 531 and 609 tests, 0 failed. The determinism check stayed under its 240-second limit (227.2 s). The regenerated recording matches its pinned fingerprint. GitHub's gate, smoke and toolchain checks are green on that commit. The unit report is accurate. I found three small wording gaps in it: a load figure attached to the wrong moment, two carried items it left unchecked (I checked them, and both are met), and one recorded deviation it should repeat. They get fixed in the report's next edit, with no extra review round. This needs no go from you because it is not a merge to main and nothing a player sees changes. As before: one Low security weakness in the spike's own code checker stays open as a follow-up, and it will be listed in PR #40's report, which needs your go anyway.

Decisions needed: none.

For Mustafa: nothing.
