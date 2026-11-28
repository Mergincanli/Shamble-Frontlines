# Advisor: M6, the review of BRAWL-1's pushed head (workflow wf_8067a67a-6fe), 2026-11-15

Written by script, word for word, from the workflow's journal (advisor-m6's StructuredOutput result). Its notes: advisor-brawl1-m6.md. Nothing below is the main session's.

**Head:** 8f0939fb4f58786a19b3487bf51f13fb6cb81bab (the reviewed head of claude/netcode-proof-test-plan-hcky5g, PR #40). It contains the merge commit b30314aa8816f0f6cd4492db518bf738d9a91a79. The three commits after the merge (5b00f44, 8043264, 8f0939f) touch only .checkpoints/. The PR's head is now 83fb6ac: the saver's commits since then touch only .checkpoints/ (the card, my notes, brawl1_m6_full.py).

## Verdict: APPROVE WITH FINDINGS

The merge holds. I checked every figure in the merge record myself, and all of them hold except one time range on the card.

- **M1, the bytes:** the merged spikes/ bytes are exactly the reviewed snapshot c2561b6c, and the base held.
- **M2, the cleans:** both used exactly the approved commands.
- **M3, the counted run:** it ran on the committed merge with a clean tree, met every A3 figure and reproduced all three pins. I hashed the regenerated recordings myself.
- **M4, the gate:** it passes. I ran it myself; only the smoke check (5a) cannot start Chromium here.
- **M5, CI:** green on b30314a and on 8f0939f.
- **The report words:** copied word for word.

There are three Lows, all record-keeping. None changes a byte that merged, and none triggers a push. M7 may start as ruled, with four small amendments. Nothing for Mustafa.

My notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-m6.md. My one run: /home/user/Shamble-Frontlines/.gate/brawl1-merge/advisor-m6/gate.log. I changed nothing in any tree: after my run, `git status --porcelain` shows only my notes file.

## The checks

1. **The commit range (A5)** PASS. HEAD = origin = 8f0939fb. b30314a's parent is f7dff4bf. 5b00f44, 8043264 and 8f0939f each touch only .checkpoints/ (git diff --name-only per commit). b30314a changes the 10 spikes files and deletes wip/brawl1.patch (11 files). Its message has no [skip ci]. The spikes/netcode-proof tree is ebbfeb40 at both b30314a and 8f0939f.
2. **M1, the base** PASS. The spikes/ tree is 63de1632 at both b30314a^ and 352a245 (spikes/netcode-proof abad0635 at both).
3. **M1, the bytes (A1)** PASS. The patch at 888ecbb and at b30314a^ both hash to sha256 c2561b6c…. `git diff b30314a^ b30314a [--binary] -- spikes/ | sha256sum` also gives c2561b6c. Each of the 10 files has the same sha256 at b30314a, in the builder worktree (HEAD 352a245, 10 modified, its `git diff HEAD` gives c2561b6c) and in the main folder. numstat: 1,139 + 127 = 1,266 lines. Nothing from round 2 rode.
4. **M2, the cleans** PASS. m2-clean-B.log has 15 'run:' lines and m2-clean-C.log has 6, each identical to .gate/u4r-sets/clean-shared-B.log and clean-shared-C.log (I diffed them). Every command exited 0, and every guard (test -f, test ! -e, ls -A) is present. The df figures convert to 6,591.4, 7,026.2 and 12,950.4 MiB, which match the runner headers (6591, 7026, and M3's 12950). (B) freed 434.8 MiB and (C) freed 5,924.2 MiB; A4 put debug at 5,925.
5. **M3, the header (A2)** PASS. HEAD b30314a, with `diff e3b0c442…; untracked e3b0c442…`. The lane lock was taken exclusively at 08:38:44, idle after 0 s. /proc/uptime reads 227.41 s, so the machine booted about 08:35, after the recorded restart.
6. **M3, the figures (A3, A4, condition 8)** PASS. check.mjs passed all 11 steps: 538 and 616 tests, 0 failed, 0 skips, self 77 of 77. These equal the copy's final-fp-check.log. df after check.mjs read 8,342,093,824 B. uptime came first (08:53:06, load 2.31), then det-u4 PASS in 234.1 s. Its non-cargo sum is 219.4 + 2.5 + 0.9 + 0.0 = 222.8 s, under 240 (the copy measured 210.0 and 211.1 s). df after det-u4 read 8,342,081,536 B, above the 4 GiB floor. The regeneration line is exact and wrote into a new folder (.gate/brawl1-merge/m3, made at 08:57:11). All 9 commands exited 0.
7. **M3, the pins, hashed by me** PASS. det-u4's SS-P2-s1.rec gives 1ec25de7… and ES-P3-s2.rec gives 6abf7643… (written 08:55-08:56, during M3). m3/rec/SS-P0-s4.rec gives 1f00d2d5…. The manifests are a2faf3ea and 5ccf57f0, and they hold those pins. regenerate_all (hash.rs:461-494) checks each entry before it keeps it.
8. **M3, the full test (condition 13)** PASS. I diffed its 9 printed lines against final-fp-regen-tests.log's 13 lines (the short copy's 4 plus the full test's 9). Only the short copy's 4 differ. The 9 full-test lines are byte-identical, dead figures included.
9. **M4, the gate, my own run** PASS. I ran the gate through main-run.sh (.gate/brawl1-merge/advisor-m6/gate.log; HEAD 83fb6ac, diff and untracked both e3b0). `node tools/gate.mjs --no-smoke` exited 0 with GATE PASS (symbols +0 -0). The full gate exited 2 on 5a alone, for the same reason as m4-gate.log: headless_shell-1194 is missing, and /opt/pw-browsers holds only -1243. CI's smoke job covers 5a.
10. **M5, CI (read with the GitHub tools)** PASS. On head_sha b30314a: gate run 43697261383 succeeded, including job gate (132760462244) and job smoke (132760462037, whose step 'Smoke (check 5a)' succeeded). Toolchain run 43697261438 succeeded. On 8f0939f: gate 43697365073 and toolchain 43697365142 both succeeded. The runs for b30314a were created at 09:02:08 and those for 8f0939f at 09:03:22, which fits 'pushed alone'.
11. **A6, the runners** PASS. main-run.sh is f0b7a3d0 and u4r-run.sh is 2f754d12. c-u4r-runs.mjs is 99dd070e, unchanged since 493b3d7.
12. **Since M5: the report words** PASS. The ruling file's sha256 is 64e361ab, both now and at 8f0939f. u4r-report.md gained 43 lines and lost none. Its 36 quoted lines equal the ruling's lines 131-166 (I diffed them).
13. **Since M5: the card** PASS, with one slip. I checked every figure in M1-M5 against the logs. M2's time range is wrong (ADV-M6-L1). The restart record agrees with the logs: no clean ran before 08:37:41, and 434.8 + 5,924.2 MiB freed equals the 6,360 MiB target.
14. **The tree after my checks** PASS. `git status --porcelain` shows only my notes file. spikes/, legacy/ and tools/ are clean.

## Findings

### ADV-M6-L1 (Low)

- **Where:** .checkpoints/netcode-proof-test.md at 8f0939f, the 'M2 DONE (08:38-08:40 UTC)' block
- **Wrong:** The card gives M2 as 08:38-08:40. The logs show (B) at 08:37:41-08:37:47 and (C) at 08:38:04-08:38:09, and M3 started at 08:38:44, so the card's range overlaps M3. The pgrep result and the three df readings exist only on the card; no log holds them. The figures themselves hold: M3's header shows the same 12,950 MiB.
- **Fix:** In the merge record, give M2 as 08:37:41 to before 08:38:44. Keep M7's pgrep and df output in a log (M7-2). This rides the next card or report commit; it triggers no push of its own.

### ADV-M6-L2 (Low)

- **Where:** .gate/brawl1-merge/m3.log footer; the card's M3 block
- **Wrong:** My round-1 M3 said 'Free space is recorded at its start and end'. m3.log ends with no free-space figure, and the card gives none. The A4 readings in the middle of the run are there (8,342,093,824 and 8,342,081,536 B).
- **Fix:** Record as post-M3 figures, labelled 'after M3 and M4, not at M3's end': my reading of 8,320,163,840 B at 09:06:48 (nothing running; the shared target about 5.0 GiB: debug 4,611, release 250, wasm32 122, doc 15 MiB), and M7's first df before (B). This triggers no push.

### ADV-M6-L3 (Low)

- **Where:** .checkpoints/netcode-proof-test/u4r-report.md:614-655, the new BRAWL-1 section
- **Wrong:** My round-2 words said 'U4r's report (where its round 1 sits)'. Round 1's review record is not in u4r-report.md, which holds only round 1's U4g words (:217-222). Round 1 sits in brawl1-build.md (:251 'Report' and :620 'Fix pass, round 1: report'). So the section starts at round 2 and cites ADV-BP-1 to ADV-BP-4 without holding them. The slip in wording is mine.
- **Fix:** When the merge record goes into this section, add one line that points to round 1's record (brawl1-build.md:251 and :620; advisor-brawl1-r1-full.md) before PR #40's report is folded. The quoted words stay unchanged. This triggers no push.

## M7

GO. M7 starts as my rulings set it: cleans (B) and (C) again, with df before and after; then the two 10-minute evidence runs of condition 14 (bars: everyone in view at 0.99 or more, building total 0, bytes within U4r-F3's rates of P0 603.7 MB and P3 183.3 MB; never named u4r-*); then the re-run sets P3, P2 and P0 (split) of condition 15, on c-u4r-runs.mjs 99dd070e, with each set's results committed before the next.

Four amendments:
- **M7-1:** the cleans start only after this M6 workflow has returned, with pgrep empty for cargo, rustc, npt-lab, npt-judge and loader.mjs, and no agent running. Write the commands out literally, as at M2 (the restart lesson). Diff the new logs' 'run:' lines against m2-clean-B.log and m2-clean-C.log; they must be identical.
- **M7-2:** keep every df reading as command output in a log under .gate/brawl1-merge/ (for example, `df -B1 /home/user` as an extra command at the start and end of each clean call), not only on the card. The first reading is also the post-M3 figure (ADV-M6-L2). Expect about 12,950 MiB free after (C), as at M2.
- **M7-3:** 'At the fix's commit' must be checkable. Every evidence log and set log header must show `diff e3b0…; untracked e3b0…` and a HEAD whose spikes/netcode-proof tree is ebbfeb40 (b30314a, or a later commit that touches only .checkpoints/). Any other value voids that run and comes to me whole. The runners stay as they are (A6).
- **M7-4:** put the measured refill on the card for disk planning. The main folder's cold full set refilled about 5.0 GiB, not the 8.2-9 GiB estimated earlier; free space after M3 was about 7.9 GiB against my projection of 3.9-6.6. The existing floors stay as they are (4 GiB before any wasm32 or release build; each set's start figure plus 4,096 MiB, so P0 needs 7,711 MiB). No new stop. KEPT-LOGS is unchanged: it stays open with Mustafa.

Everything else stands as ruled. A floor stop is reported and never cleaned around. A pin mismatch or a missed bar comes to me whole, with no retry. -57 values are recorded beside U4r's, and any verdict or shown-bite difference comes to me.

## For Mustafa

For Mustafa: nothing.

BRAWL-1, the fix that keeps the test bots out of buildings, is now merged into the test branch. I checked the merged version myself:
- the code that went in is exactly the reviewed code;
- the one counted run passed every check;
- the three pinned recordings came out byte for byte the same (I hashed them myself);
- the gate passes, and GitHub's checks are green.

I found three small record-keeping slips (a time range on the progress card, one missing free-disk reading, and a missing pointer to round 1 in the report). They get fixed in the records and change no code.

Next: clear the build caches again, run the two 10-minute check runs, then the re-run sets.

Decisions needed: none. The kept-logs decision (KEPT-LOGS) is still open with you, unchanged.

For Mustafa: nothing. This is spike code on the test branch, not a merge to main. Nothing a player sees, hears or feels changes.
