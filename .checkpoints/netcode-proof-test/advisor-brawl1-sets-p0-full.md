# Advisor: the ruling on BRAWL-1's P0 re-run set (workflow wf_abd08482-f78), 2026-11-15

Written by script, word for word, from the workflow's journal. Its notes: advisor-brawl1-sets-p0.md. Nothing below is the main session's.

## Summary

Verdict: P0 STANDS AND COUNTS. The tick16 bite counts as shown, and its flag clears under a corrected TICK16-WORDING. Commit P0's three result files. With P0, BRAWL-1's re-run sets are complete: P3, P2 and P0 all count.

The P0 set ran at 93509df (spike tree 07baa855) after pre-P0 passed.
- pre-P0.log (sha256 2e0bb273…) meets the amended M7-3 on every line.
- runs.txt names the same commit and tree.
- I re-checked at HEAD 6041c49: the spike tree is still 07baa855, only the card and my notes have changed since 93509df, and the only untracked files are P0's three results files.

Verdicts against U4r's P0 (051995db9cea):
- -2 SS went from FAIL to PASS (everyone in view 1 / 1 / 1; U4r's s1 had 0.861889).
- -17 L1 P0 went from INVALID to PASS: 0 mismatches of 878,352 compared on each run, and 0 incomplete.
- -57 L1 P0 is PASS in both.
- All six bites are shown. -17's three went from invalid to shown, which is the expected change. -57's three are shown in both sets, and each fails only its own check.
- The measured runs have 0 resyncs, 0 incomplete and a building figure of 0.
- Every summary figure on the measured runs and the five clean bite runs equals U4r's same-seed line. The exceptions are the leash's expected ones: very large units held 1.0000, projectiles, audit counts and fate logs.

tick16: I found the cause of the 94 resyncs in its room.jsonl.
- After the wrap the room has no command stamped for the real tick, so it re-runs each soldier's last command (option B, input.rs:20/:127/:201). Four soldiers walk into buildings during the after phase. This is outside the building figure, which counts measured ticks only.
- Slot 5 has been inside a building since tick 65,588, so it no longer sees the fight. Its event queue had not reached the cap when the other 47 overflowed at 65,611.
- The run waits until every measured tick is resolved, and slot 5's resolve only with its own full state at 65,791. In that wait, the 46 clients whose header was still wrapped overflowed a second time at 65,729. That gives 47 + 46 + 1 = 94.
- No client was kicked. On the measured ticks before the wrap, all 863,280 snapshot checks match. All 12,151 mismatches are rebuilds filed under the wrapped tick numbers. The 720 incomplete are 48 clients × 15 ticks, by the bite's design.

Findings:
- **ADV-P0-1 (Low, my error).** TICK16-WORDING's "one per client" was wrong, and so was my computed "wrap + 54-60". The first resync came at wrap + 75, because the room's event rate falls after the wrap (574 a second at the end of the window, 198-432 a second after it).
- **ADV-P0-2 (Medium).** The card's P2 record says "-57's values equal U4r's… largest change 0.0 %". -57's reported p99 moved from 5,257 / 5,280 / 5,283 to 5,111 on all three runs. The largest change at P2 is s3's position p99, -3.26 %. No verdict is affected; the report must carry the corrected figure.
- **ADV-P0-3 (Low).** No redact-check log was kept for the P3 and P2 results commits, as U4r's sets kept one. I ran it read-only on all three folders: 7 items, 0 findings.

What I checked myself (read-only):
- the pre-P0 and P0 set logs;
- runs.txt, verdict.json and verdict-bites.json for both P0 sets, compared mark by mark, part by part and bite by bite by inline print-only python;
- tick16's lab.jsonl, room.jsonl and client.jsonl (hashes compared per tick and slot), and U4r's tick16 lab.jsonl and room.jsonl event rates;
- every P0, P2 and P3 room.jsonl for building entries in any phase;
- room.rs, scenario.rs, logs.rs, client and input.rs at the cited lines;
- du and df; P2's verdicts against U4r's P2; git trees.

I changed nothing in any tree. My notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-sets-p0.md.

## Condition 15 for P0

Condition 15, applied to P0. Figures are for runs s1 / s2 / s3.

1. **Verdict and shown-bite differences against U4r's P0** (results/u4r/051995db9cea/P0/):
   - **-57: none.** L1 P0 is PASS in both sets, with all three runs valid. The mark overall is NOT-RUN in both. pos-4cm, yaw-12bit and pitch-8bit are shown in both, each failing only its own check. Nothing about -57 comes to me whole.
   - **The rest, all as the plan's §9 and my P3 ruling expect:**
     - -2 SS runs: FAIL to PASS (everyone in view 0.861889 / 1 / 1 to 1 / 1 / 1);
     - -17 L1 P0: INVALID (s1 by pre:PM-0001-2) to PASS;
     - -17's three bites: invalid (pre:PM-0001-2, 0.861889) to shown.
       - tick16: 12,151 mismatches and 720 incomplete, both checks fail.
       - client-rounding: 878,352 mismatches fail; incomplete 0 passes.
       - whole-as-lost: 0 mismatches passes; incomplete 48 fails.
     - The marks overall: -2 FAIL to NOT-RUN and -17 INVALID to NOT-RUN. Their other parts are not in a P0 call, so this is expected.

2. **-57's values beside U4r's** (µm and µdeg):
   - position max error 6,984, yaw 2,750 and pitch 88,063 on every run, equal to U4r's;
   - position p99: 5,737 / 5,736 / 5,739, against U4r's 5,719 / 5,718 / 5,715 (+0.31 / +0.31 / +0.42 %);
   - pos-4cm: 34,856 against 34,830 (+26, +0.075 %). Yaw 2,750 and pitch 88,063 pass.
   - yaw-12bit: 46,692 against 46,692 (0 %). Position 6,984 and pitch 88,063 pass.
   - pitch-8bit: 440,313 against 440,312 (+1, +0.0002 %). Position 6,984 and yaw 2,750 pass.
   - **Largest relative change:** s3's position p99, 5,715 to 5,739, +0.42 %.

3. **The building figure:** 0 slot-ticks on all nine runs, bite runs included (runs.txt summary lines). The figure is defined per measured tick (plan §3).
   - **Recorded, decides nothing:** tick16's after phase has building entries on 204 ticks, 514 slot-ticks in all:
     - slot 5 in area 1, ticks 65,588-65,791;
     - slot 17 in area 3, ticks 65,664-65,791;
     - slot 14 in area 5, ticks 65,679-65,791;
     - slot 18 in area 4, ticks 65,713-65,781.
   - They are the bite's effect. Commands stop landing on real ticks after the wrap, so the room re-runs each soldier's last command and the leash cannot act.
   - No other P0, P2 or P3 run has an entry in any phase.

4. **The bytes against 3,614.6 MiB:**
   - The logs on disk are 3,556,179,495 B, which is 3,391.4 MiB: measured runs 1,621,260,833 B (1,546.2 MiB) and bite runs 1,934,918,662 B (1,845.3 MiB). The ratio is 0.938.
   - U4r's P0 had 3,248.3 MiB, so the logs are 4.4 % larger now.
   - Per run: s1 538,352,938 / s2 540,448,893 / s3 542,459,002 B; tick16 56,560,048; client-rounding 55,695,879; whole-as-lost 55,695,195; pos-4cm 628,646,092; yaw-12bit 592,199,534; pitch-8bit 546,121,914.
   - peak.py's peak for the whole set is 4,504.3 MiB, against 4,621 MiB projected for the bite judge call: -116.7 MiB, on the safe side.
   - Wall time 2,566.1 s (U4r 2,316.2 s). Free space now 7,722.8 MiB.

5. **TICK16-WORDING:** the flagged run is ruled under "The tick16 bite" below, with the wording corrected there.

## The tick16 bite

**Ruling: bite PM-0001-17/tick16 is SHOWN and counts.** The carry-58 flag clears. TICK16-WORDING is corrected below.

**What I read myself** (print-only `python3 -I -c` on .gate/netcode-proof-test/runs/u4r-P0-93509dfd7fd1/bites/bite-tick16/):

- **Resyncs (lab.jsonl):** 94 overflow_resync events.
  - All are in the after phase. The first is at 65,611 (wrap + 75) and the last at 65,791. The measured window ends at 65,550.
  - Three ticks: 47 at 65,611, 46 at 65,729 and 1 at 65,791.
  - 48 distinct slots. 46 slots resynced twice (65,611 and 65,729). Slot 17 resynced once, at 65,611. Slot 5 resynced once, at 65,791.
  - Per client, from client_counts at 65,791:
    - steps without a command: 253-255 (12,151 in all);
    - rewinds refused: 78 on 46 clients, 137 on slot 17, 196 on slot 5 (3,921 in all);
    - late, stale, buffer full, press refused and duplicate stamps: 0.
  - Other events:
    - run_lengths: after 241 ticks, not capped;
    - class_audit: 0 breaches;
    - counter_wrap: event_seq at 48,780, and a new "ack" wrap at 65,612. That is the first acknowledgement past 65,536, of the 65,611 full state. U4r's run ended on its resync tick, so it never got that far.
- **Kicks (room.jsonl): none.** Its only events are the 48 admits in warmup. snapshot_clients_count is 48 on all 18,000 measured and 241 after ticks.
  - The kick rules are a second overflow inside an unacknowledged forced resync (room.rs:1021-1026) and a forced full state unacknowledged past FULL_STATE_ACK_TICKS (:1056-1061). Neither fired, so each second resync followed an acknowledged full state.
  - Snapshot datagrams that tick: 1 at 65,611, 2 at 65,729 and 47 at 65,791 (the full states went in their place).
- **Mismatches.** I compared client.jsonl's decoded hashes with room.jsonl's server hashes per tick and slot:
  - measured ticks before the wrap: 863,280 equal (48 × 17,985), 0 differ;
  - 65,536-65,550: 720 client-ticks with no hash. These are the 720 incomplete.
  - after phase: 94 equal (each forced full state rebuilt equal to the server) and 11,474 with no client hash;
  - 256 ticks the room never ran (the wrapped stamps): 12,151 sentinels, which are the judge's 12,151 mismatches, plus 137 with no hash.
  - So no mismatch falls inside the measured window, and every mismatch is a wrapped rebuild.

**Why 94 and not 48.**
- The run stops at the first tick where every measured tick is resolved (scenario.rs:1121-1125). A client's last measured ticks resolve with its own forced full state.
- In U4r all 48 queues reached the cap of 1,024 on one tick (65,615). The run ended there, 65 ticks after the window, with one resync each.
- Here the clients' queues did not all fill together. Commands stop landing on real ticks after the wrap, so option B re-runs each soldier's last command (input.rs:20, :127, :201). Each soldier walks its last heading for about 8 s, and four walk into buildings.
- Slot 5 has been in building 0 (area 1) since 65,588, before the first overflow. From there it no longer sees the fight. A client queues only hits and kills it is part of or can see (room.rs:990-1012), so its queue fills more slowly: 326,945 events queued over the run, against 328,613 on 45 slots.
- So slot 5 had not overflowed at 65,611, and the run waited for it. In those 180 extra ticks, the 46 other clients, whose header was still wrapped, filled their queues again and overflowed at 65,729, 118 ticks after the first.
- Slot 17 entered a building at 65,664 and had not refilled by 65,791. Slot 5 overflowed at 65,791, and the run ended.
- **The offset:** the room's event rate falls after the wrap: 528-585 a second at the window's end; 432, 307, 300, 285, 265, 198, 340 and 289 a second after it. The queue fills at about 13.7 events a tick to the first overflow and 8.7 to the second.

**Why 720 measured incomplete.**
- The wrap at 65,536 falls 15 ticks before the measured window ends (65,550). Those 15 ticks' rebuilds are filed under the wrapped ticks 0-14, so the real ticks never resolve: 48 × 15 = 720.
- It is fixed by the bite's tick_start of 47,251 (ticks_below_wrap 18,285, the same in both headers). It equals U4r's 720 and r2's record's 720, and it does not depend on the resyncs.

**Why it still counts as shown.**
- The run is valid: everyone in view 48 on all 18,000 measured ticks, and pre:PM-0001-2 passes.
- Both -17 checks fail, and both failures come from the planted 16-bit header alone. Nothing differs before the wrap.
- Every resync is in the after phase. No client was kicked. A resync is triggered by the queue, never by a mismatch.
- The extra resyncs change only after-phase counts: mismatches 12,151, against 3,792 for U4r's run and 2,880 for r2's; steps without a command; rewinds refused; and decided lost at the stop, 4,520 against 1,488. None of these decides "shown".
- This is not EVQ-LOAD. The queue fills here because the bitten header stops acknowledgements, not because of the event load.
- tick16's "shown" now rests on this 10-minute run, not only on r2's 30 s record.

**TICK16-WORDING needs correcting.** From now on, the expected line reads, word for word:

"tick16's run is valid and the bite shown, failing both -17 checks. Its measured incomplete is 720: 48 clients times the 15 measured ticks from the wrap (65,536) to the window's end (65,550). Every client has at least one overflow resync, all in the after phase, none in the measured window, and no kick. The count and the ticks are not fixed: the run ends only when every client's last measured ticks resolve, and while one client's queue fills slowly, the others can resync again, each after its forced full state was acknowledged."

**ADV-P0-1 (Low, my error):**
- **Where:** TICK16-WORDING's "one overflow resync per client" (advisor-u4r-sets-r2-full.md), and my P3 ruling's "about wrap + 54-60" (advisor-brawl1-sets-p3-full.md, The next sets).
- **What was wrong:** "one per client" holds only when every client's queue fills on the same tick. My offset used the measured event rate, but the rate falls after the wrap.
- **Effect:** none. No verdict rested on it, and the main session stopped exactly as the wording told it to.
- **Fix:** the corrected wording above, on the card and in u4r-report.md, replacing the line at :562-564.

## The commit and the close

**1. Commit P0's results now (yes).** The commit holds:
- exactly the three untracked files under spikes/netcode-proof/results/u4r/93509dfd7fd1/P0/:
  - runs.txt, sha256 ada5a58a50e3…;
  - verdict.json, sha256 e926b812cbe5…, equal to the raw-log folder's copy (cmp);
  - verdict-bites.json, sha256 b472784f2c72…, also equal (cmp);
- the card, which may ride along.

The commit's diff (`git diff --name-only HEAD^ HEAD`) touches only that folder and .checkpoints/.

Before the commit, keep one read-only redact-check log covering the three BRAWL-1 results folders as .gate/u4r-sets/redact-BRAWL1-sets.log (ADV-P0-3): `node spikes/netcode-proof/ops/operator/redact-check.mjs spikes/netcode-proof/results/u4r/fd4612050cd6/P3 spikes/netcode-proof/results/u4r/fc411116c1f1/P2 spikes/netcode-proof/results/u4r/93509dfd7fd1/P0`. My own run gave 7 items and 0 findings.

Then push.

**2. BRAWL-1's re-run sets are COMPLETE.**
- P3 at fd4612050cd6 (committed at fc41111) counts. Carry 58's s1 and s3 count, as my P3 ruling decided.
- P2 at fc411116c1f1 (committed at 93509df) counts. It has no flagged run, so -17 L1 P2 counts as it stands.
- P0 at 93509dfd7fd1 counts, with tick16 shown (this ruling).

Across the nine measured runs and six bites, every expected line is met:
- -2 SS PASS at P3, P2 and P0, with everyone in view 1 on every run;
- -17 L1 P3, P2 and P0 PASS with 0 mismatches, and 0 incomplete at P0;
- -17's three bites shown;
- -57 L1 P3, P2 and P0 PASS, its three bites shown, each failing only its own check;
- the building figure 0 on every run;
- the bytes inside every projection: 4,304.3 MiB in all, against 4,919.8 MiB projected.

-2 passing everywhere is the fix's proof (BRAWL-1 (d)). -17's L1 P0, P2 and P3 parts now count from the fix's commit. -17 L1 B1 stays with U4g's judged B1 runs.

**3. Still to do before BRAWL-1 closes:**
- **a. The card:**
  - correct P2's record (ADV-P0-2): -57's p99 went from 5,257 / 5,280 / 5,283 to 5,111 on all three runs, and the largest change is s3's position p99, -3.26 %;
  - record this ruling and the corrected TICK16-WORDING;
  - record EVQ-LOAD word for word from advisor-brawl1-sets-p3-full.md (it is not on the card or in the report yet).
- **b. u4r-report.md's BRAWL-1 section.** Today it holds only round 2 (:614-655). It needs:
  - the plan and the build: the size, 1,266 changed lines; the pins; pattern (iii);
  - round 1 and its ruling;
  - the merge step M1-M7 and M6's head review;
  - the two evidence runs, with ADV-P3-1's added P3 figures;
  - the re-run sets, in the words of report_words;
  - EVQ-LOAD;
  - the corrected TICK16-WORDING, replacing the old line at :562-564;
  - an update to the line saying tick16's "shown" rests on r2's 30 s record;
  - the corrections:
    - ADV-BP-1 and ADV-BP-2 in their own words (condition 18), with ADV-BP-3 and -4;
    - ADV-PT-M1's home (u4g-brief-carry.md);
    - ADV-R1-1 and -2;
    - ADV-M6-L1 to -L3;
    - ADV-P3-1 and -2;
    - ADV-P0-1 to -3;
  - the follow-ups with their homes: TE-B1-R2-1 with RR2-B1-1; TE-B1-R2-2; ADV-PT-L3; EVQ-LOAD for U4f and U4g;
  - its Decisions needed and For Mustafa.
  - The BRAWL-1 follow-up entry at :215-239 is then marked done, with a pointer to the section.
- **c.** That section comes to me for a check against the files before BRAWL-1 is recorded closed. That check is a read, not a review round.

**Then, outside BRAWL-1:**
- TE-B1-R2-2's runner edit (A6). P0 was the last set, so it is now due, before U4g's first run. It is u4r-run.sh:67 and main-run.sh:42, in the ruled form, each with a .diff and `bash -n`.
- KEPT-LOGS before U4g's build (For Mustafa).
- U4g's brief carries u4g-brief-carry.md and EVQ-LOAD.

Unchanged:
- PR #40 stays a draft, and main is taken in before it is marked ready (MAIN-52).
- Nothing here merges to main.

## The report words

### The re-run sets (P3, P2, P0) at the fix's commit

All three sets ran with c-u4r-runs.mjs 99dd070e, the runners unchanged (A6), each set's results committed before the next set started, and the amended M7-3 met at each start (pre-P2.log, pre-P0.log). Figures are for runs s1 / s2 / s3; "U4r" means U4r's same-profile set.

**P3:** commit fd4612050cd6, spike tree ebbfeb40. Log .gate/u4r-sets/P3-fd4612050cd6.log (sha256 cc2f626c…), exit 0, wall 1,020.5 s, peak 973.2 MiB. Results results/u4r/fd4612050cd6/P3/ (runs.txt 5361fd53…, verdict.json 21c9abbf…), committed at fc41111.
- -2 SS runs: FAIL to PASS. Everyone in view went from 0.352778 / 0.085444 / 0.203556 to 1 / 1 / 1.
- -17 L1 P3: INVALID to PASS, with 0 mismatches of 835,238 / 835,419 / 835,088 compared. The judge counts 44,395 / 44,253 / 44,535 incomplete over the whole run; npt-lab counts 43,295 / 43,210 / 43,454 in the measured window.
- -57 L1 P3: PASS, as in U4r's. Position max 6,952 / 6,963 / 6,970 against 6,984; p99 5,111; yaw 2,750 and pitch 88,063, unchanged. Largest change: s1's position max, -0.46 %.
- Carry 58: overflow resyncs 27 / 0 / 1 (U4r 0 / 0 / 1… no: U4r 0 / 0 / 0).
  - s1: 25 clients at ticks 76,910-77,844. s3: slot 35 at 76,803.
  - All are in the measured window. No client was kicked, and there are 0 mismatches.
  - The advisor ruled that s1 and s3 count (EVQ-LOAD).
- Building figure 0 on every run. Bytes 290.7 MiB against 524.4 projected (0.554); U4r's P3 had 518.8.

**P2:** commit fc411116c1f1, spike tree 079ffffc. Log .gate/u4r-sets/P2-fc411116c1f1.log (sha256 a24d5856…), exit 0, wall 1,046.7 s, peak 1,760.3 MiB. Results results/u4r/fc411116c1f1/P2/ (runs.txt c2995379…, verdict.json 1d2ff9e5…), committed at 93509df.
- -2 SS runs: FAIL to PASS. Everyone in view went from 1 / 0.5875 / 0.7295 to 1 / 1 / 1.
- -17 L1 P2: INVALID (s2 and s3 by pre:PM-0001-2) to PASS, with 0 mismatches of 861,996 / 862,011 / 861,731 compared. The judge counts 17,739 / 17,566 / 18,027 incomplete over the whole run; npt-lab counts 17,260 / 17,089 / 17,532 in the measured window.
- -57 L1 P2: PASS, as in U4r's. Max 6,984 / 2,750 / 88,063 on every run, equal to U4r's. Position p99 5,111 on every run, against 5,257 / 5,280 / 5,283. Largest change: s3's position p99, -3.26 % (ADV-P0-2 corrects the card's '0.0 %').
- Overflow resyncs 0, so no flag. Every other summary figure equals U4r's same-seed line.
- Building figure 0. Bytes 622.2 MiB against 780.8 projected (0.797); U4r's P2 had 778.9.

**P0 (split):** commit 93509dfd7fd1, spike tree 07baa855. Log .gate/u4r-sets/P0-93509dfd7fd1.log (sha256 e07e882e…), exit 0, wall 2,566.1 s, peak 4,504.3 MiB. Results results/u4r/93509dfd7fd1/P0/ (runs.txt ada5a58a…, verdict.json e926b812…, verdict-bites.json b472784f…).
- -2 SS runs: FAIL to PASS. Everyone in view went from 0.861889 / 1 / 1 to 1 / 1 / 1. Shots per second 693.09 / 692.97 / 693.06 (U4r 694.16 / 694.19 / 693.98).
- -17 L1 P0: INVALID (s1 by pre:PM-0001-2) to PASS. 0 mismatches of 878,352 compared on each run; 0 incomplete; 0 overflow resyncs.
- -17's bites: invalid in U4r's set, now shown.
  - tick16 fails both checks: 12,151 mismatches and 720 incomplete.
  - client-rounding fails mismatches (878,352); incomplete 0 passes.
  - whole-as-lost fails incomplete (48); 0 mismatches passes.
- -57 L1 P0: PASS, as in U4r's. Max 6,984 / 2,750 / 88,063, equal. Position p99 5,737 / 5,736 / 5,739 against 5,719 / 5,718 / 5,715. Largest change: s3's position p99, +0.42 %.
- -57's bites: shown in both sets, each failing only its own check. pos-4cm 34,856 (U4r 34,830, +0.075 %); yaw-12bit 46,692 (equal); pitch-8bit 440,313 (U4r 440,312).
- Building figure 0 on all nine runs. Bytes 3,391.4 MiB against 3,614.6 projected (0.938); U4r's P0 had 3,248.3.
- **The tick16 bite (carry 58):** 94 overflow resyncs, against U4r's 48.
  - All are in the after phase: 47 at 65,611, 46 at 65,729 and 1 at 65,791. The window ends at 65,550. 48 slots.
  - No kick: 48 snapshot clients on every tick. No mismatch inside the measured window.
  - The 720 incomplete are 48 × 15 ticks, by design.
  - The cause: after the wrap, the room re-runs each soldier's last command. Four soldiers walk into buildings in the after phase. Slot 5, inside since 65,588, fills its queue more slowly, and the run waits for its resync while the others resync a second time.
  - The advisor rules that tick16 is shown and counts. Its 'shown' now rests on this 10-minute run. TICK16-WORDING is corrected (below).

**Overall:**
- -2's SS part passes at every profile: that is the fix's proof.
- -17's L1 P0, P2 and P3 parts and its three bites count from the fix's commit.
- -57 passes everywhere. Its largest change across the sets is P2's s3 position p99, -3.26 %.
- The logs total 4,304.3 MiB against 4,919.8 projected.
- -17 L1 B1 stays with U4g's judged B1 runs.

**TICK16-WORDING (corrected by advisor-brawl1-sets-p0):** "tick16's run is valid and the bite shown, failing both -17 checks. Its measured incomplete is 720: 48 clients times the 15 measured ticks from the wrap (65,536) to the window's end (65,550). Every client has at least one overflow resync, all in the after phase, none in the measured window, and no kick. The count and the ticks are not fixed: the run ends only when every client's last measured ticks resolve, and while one client's queue fills slowly, the others can resync again, each after its forced full state was acknowledged."

**EVQ-LOAD:** word for word from advisor-brawl1-sets-p3-full.md, The next sets.

**Advisor corrections:**
- ADV-P3-1 (Medium): the P3 flags were not raised.
- ADV-P3-2 (Low, the advisor's): M7-3's wording.
- ADV-P0-1 (Low, the advisor's): TICK16-WORDING's 'one per client' and the computed 'wrap + 54-60'.
- ADV-P0-2 (Medium): P2's -57 p99 was left out of the card's record, so its '0.0 %' was wrong.
- ADV-P0-3 (Low): no redact-check log was kept for the P3 and P2 commits. The advisor's own read-only run on all three folders found 0.

NOTE TO THE MAIN SESSION: in the P3 carry-58 line above, write 'U4r 0 / 0 / 0'. My draft slipped; the true U4r figure is 0 / 0 / 0, as in my P3 ruling.

## For Mustafa

In plain words:
- The bot fix has now passed all three re-run sets. Every bot stayed in view in every run, none walked into a building while the measurements ran, and the main snapshot check passes at every network level with zero errors.
- Every planted fault was caught, including the one that tests the tick counter wrapping round. Its 10-minute run now counts.
- That test sent more "resend the full game state" messages than I had predicted. I traced the cause in the logs: once the planted fault breaks the bots' commands, a few bots walk into buildings after the measurements end. While the run waits for one of them, the others are resent the state a second time. Nothing broke: no client was dropped, and no snapshot inside the measured part went wrong. I corrected my own wording of what that test should show.
- I also corrected one figure in the record of the second set. One of the position-accuracy figures, the 99th-percentile error, improved by up to 3.3 %; the record had said nothing changed. It decides nothing.
- What remains is paperwork: committing the last results and finishing the write-up. Nothing to check in a browser and nothing to merge.

Decisions needed:
- **KEPT-LOGS** (your open question, unchanged; new numbers, and now in the next unit's way): should a narrow kit rule let a session clear out old test-run logs once their results are saved and their rulings closed?
  - Why it matters now: kept raw run logs take about 11.4 GiB (U2 1.9, U4r 5.3, the bot fix's re-runs 4.2), plus 0.6 GiB of evidence runs. Free space is 7.5 GiB against the 4 GiB safety floor, and the next unit's (U4g's) build cache refills about 8-9 GiB. Its build cannot start until space is freed.
  - Options:
    - (a) Keep everything. U4g cannot build.
    - (b) Delete each closed run set's raw-log folder. This frees up to about 11.4 GiB. Those runs could never be re-judged; their verdicts stay saved.
    - (c) The same, but first pack each set into one compressed file, checked readable. Computed with the draft's measured ratio of about 3.5 to 1, this frees about 8 GiB and keeps about 3.3 GiB, and a re-judge stays possible.
  - My recommendation: (c), with the same rule naming the old 4 GB build cache, as in the draft. It comes as its own reviewed kit pull request (advisor plus security review) and merges only on your go. Nothing is deleted before then.

For Mustafa:
1. Answer KEPT-LOGS (a, b or c) before U4g's build needs the disk. The bot fix's remaining steps do not wait for it.
