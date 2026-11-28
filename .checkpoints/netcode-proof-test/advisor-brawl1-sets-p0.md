# Advisor: BRAWL-1 re-run set P0 (split) ruling (2026-11-15)

## Plan
1. Read main's advisor.md (done), my P3 ruling (done: P0's expected lines and stops, TICK16-WORDING, carry 58).
2. Read the card's NOW block (P3, P2, pre-P0 records).
3. Check the P0 set myself: set log (exit, wall, peak), pre-P0.log (M7-3 amended checks), runs.txt head (commit, tree),
   verdict.json and verdict-bites.json against U4r's P0 (051995db9cea); every runs.txt flag and summary figure against
   U4r's same-seed line (ADV-P3-1's lesson).
4. -57: values and the three bites beside U4r's, the largest relative change.
5. Building figure; bytes against 3,614.6 MiB.
6. The tick16 bite: print-only reads of lab.jsonl / room.jsonl (count, first/last tick, phases, slots, kicks,
   mismatches); U4r's tick16 raw logs if kept; why 94 not 48; why 720 measured incomplete. Does tick16 count as shown;
   TICK16-WORDING correction.
7. Commit of P0's three files; completeness of the re-run sets (P3, P2, P0).
8. Report words; what is left before BRAWL-1 closes; For Mustafa.

## Notes (a line per point)
- N1 read: main's advisor.md; my P3 ruling (P0 expected lines :140-149); card NOW :240-400 (P3, P2, pre-P0, P0).
- N2 pre-P0.log (sha256 2e0bb273…): header e3b0/e3b0, HEAD 93509dfd7fd1 tree 07baa855; diff --quiet exit 0; name-only
  only .checkpoints/ + P3's and P2's four results files; name-status 4 A lines; scripts 99dd070e / f0b7a3d0 / 2f754d12;
  pgrep empty; df 11,665,391,616 B. Amended M7-3 met.
- N3 set log P0-93509dfd7fd1.log (60 lines, sha256 e07e882e…): exit 0, wall 2566.1 s, peak 4,504.3 MiB; projections
  printed (3,615 MiB set; judge 4,344 / 4,719 MiB before, 3,921 / 4,621 from disk); bytes on disk measured 1,546.2 +
  bites 1,845.3 = 3,391.5 MiB. Verdict copies in the raw folder equal the results' (cmp).
- N4 verdict diff vs U4r P0 (051995db9cea), script: -2 SS FAIL -> PASS; -17 L1 P0 INVALID (s1 pre:PM-0001-2 0.861889)
  -> PASS (3 valid); -57 L1 P0 PASS = PASS. Marks overall: -2 FAIL -> NOT-RUN, -17 INVALID -> NOT-RUN (other parts not
  in a P0 call), -57 NOT-RUN = NOT-RUN. Bites: -17's three invalid (pre:PM-0001-2 0.861889) -> shown; -57's three
  shown = shown, each failing only its own check: pos-4cm 34,856 (U4r 34,830, +26, +0.075 %), yaw-12bit 46,692 (=),
  pitch-8bit 440,313 (U4r 440,312, +1). tick16 shown: mismatches 12,151 FAIL, incomplete 720 FAIL. client-rounding:
  878,352 mismatches FAIL, incomplete 0 pass. whole-as-lost: 0 mismatches pass, incomplete 48 FAIL.
- N5 tick16 lab.jsonl (BRAWL-1): 94 overflow_resync, all ph after; ticks 65,611 x47, 65,729 x46, 65,791 x1; 48
  distinct slots: 46 slots twice (65,611 + 65,729), slot 17 once (65,611), slot 5 once (65,791). client_counts at
  65,791: steps without 253-255 a client (sum 12,151), rewinds refused 78 (x46) / 137 / 196 (sum 3,921), commands
  18,285-18,286. counter_wrap event_seq at 48,780 (measured) and NEW: ack at 65,612 (after). run_lengths after 241,
  after_capped false. class_audit breaches 0. U4r: 48 resyncs, all at 65,615 (after), 48 slots once each; steps
  without 79 each (3,792), rewinds refused 20 each (960); after 65, not capped; no ack wrap event.
- N6 the after phase (scenario.rs:1121-1145): a run stops at the first tick k >= measured_end with every measured tick
  written through (logs.written_through); a bite run that never resolves stops at the cap (after_capped). So after 241
  = the tick (65,791) when slot 5's only resync resolved its last measured ticks.
- N7 tick16 room.jsonl: events = 48 admits (warmup) only; snapshot_clients_count 48 on all 18,000 measured and 241
  after ticks: NO KICK. Kick rule room.rs:1021-1026 (second overflow inside an unacknowledged forced resync) and
  :1056-1061 (forced full state unacked past FULL_STATE_ACK_TICKS): neither fired, so the 46 second resyncs came after
  each first full state was acknowledged. Snapshot datagrams 1 at 65,611, 2 at 65,729, 47 at 65,791 (the full states).
  everyone_in_view 48 on all measured ticks; in the after phase 48 to 65,566, 0 from 65,567 (after only).
- N8 THE CAUSE OF 94 (read myself): in_building_slot_area is written in any phase but counted only on measured ticks
  (logs.rs:584-598; summary scenario.rs:1677-1690). tick16's after phase has entries on 204 ticks: (slot 5, area 1)
  65,588-65,791 (204 ticks, from 23 ticks BEFORE the first overflow); (17, 3) 65,664-65,791 (128); (14, 5)
  65,679-65,791 (113); (18, 4) 65,713-65,781 (69). 514 slot-ticks, all after. Every other P0 run's room.jsonl has no
  entry (grep -c 0; they have 0 after ticks). Mechanism: past the wrap the room has no command stamped for the real
  tick, so option B re-runs each soldier's last command (input.rs:20, :127, :201; 253-255 steps without one a client):
  each soldier walks its last vector for ~8 s, the leash cannot act; four walk into buildings. Slot 5, in building 0
  from 65,588, no longer sees the fight, so its queue (gameplay events only for shooter/victim/seen, room.rs:990-1012)
  had not reached 1,024 at 65,611 (fewest events queued: 326,945 vs 328,613 for 45 slots). The run waits until every
  measured tick is written through; slot 5's last measured ticks resolve only with its own full state (65,791). In
  those 180 extra ticks the 46 clients whose header still wraps refill and overflow again (65,729, 118 ticks after the
  first; the first full state acknowledged, so no kick). Slot 17 (in building 2 from 65,664, 327,519 queued) had not
  refilled by 65,791. 47 + 46 + 1 = 94. U4r: all 48 at 65,615 at once, run ended at after 65, so one each.
- N9 720 = 48 clients x 15 measured ticks (65,536-65,550) whose rebuilds are filed under the wrapped ticks 0-14, so the
  real ticks never resolve: the bite's design, equal to U4r's 720 and r2's record 720; independent of the resyncs.
  Mismatches 12,151 = sentinels 12,151 = steps without one (sum of 253-255 a client over the 256 post-wrap ticks); r2's
  30 s record 2,880 (48 x 60); U4r's P0 would have been 3,792 (48 x 79). It scales with the post-wrap length.
  decided lost at the stop 4,520 (U4r 1,488): also post-wrap length. New counter_wrap 'ack' at 65,612: the first full
  state re-anchored the client past 65,536, so its ack's high half changed (room.rs:760-766 path; client wraps): U4r's
  run stopped at its resync tick, so it never got there.
- N10 tick16 hashes, client.jsonl vs room.jsonl per (tick, slot), print-only: pre-wrap measured 863,280 equal (48 x
  17,985), 0 differ; 65,536-65,550 720 client-null (= the 720 incomplete); after 65,551-65,791: 11,474 client-null,
  94 equal (the 94 forced full states, each rebuilt equal to the server); 256 ticks the room never ran (widened
  2^32 + 0..255): 12,151 sentinels (= the judge's 12,151 mismatches) + 137 null. So no mismatch inside the measured
  window; every mismatch is a wrapped rebuild. Room events: 48 admits only.
- N11 the building figure is defined per measured tick (brawl1-plan.md §3 :70) and is 0 on all nine runs (runs.txt);
  the after-phase entries (N8) are outside it and only in tick16 (the only run with after ticks).
- N12 -57 measured: max 6,984 / 2,750 / 88,063 on every run = U4r; p99 5,737 / 5,736 / 5,739 vs 5,719 / 5,718 / 5,715
  (+0.315 / +0.315 / +0.420 %). Largest relative change: s3 position p99, +0.42 %. Bites: pos-4cm 34,856 (+26,
  +0.075 %), yaw-12bit 46,692 (=), pitch-8bit 440,313 (+1, +0.0002 %); each fails only its own check. -2 shots
  693.09 / 692.97 / 693.06 (U4r 694.16 / 694.19 / 693.98); everyone in view s1 1 (U4r 0.861889), s2, s3 equal. -17
  measured: 0 mismatches of 878,352 compared, 0 incomplete, each run. Stand-in spawns equal U4r's to 6 decimals.
- N13 bytes: measured 1,621,260,833 B (1,546.2 MiB) + bites 1,934,918,662 B (1,845.3 MiB) = 3,556,179,495 B =
  3,391.4 MiB against 3,614.6 projected: 0.938. U4r's P0 3,406,088,069 B = 3,248.3 MiB (+4.4 % now). Per run: s1
  538,352,938 / s2 540,448,893 / s3 542,459,002; tick16 56,560,048; client-rounding 55,695,879; whole-as-lost
  55,695,195; pos-4cm 628,646,092; yaw 592,199,534; pitch 546,121,914. Judge: projected from disk 3,921 (measured)
  and 4,621 (bites); peak.py's whole-set max 4,504.3 MiB (-116.7 against the larger, on the safe side). Free now
  8,098,000,896 B = 7,722.8 MiB (floor 4,096).
- N14 summary-line figures vs U4r same-seed (ADV-P3-1's lesson): measured runs and client-rounding, whole-as-lost, the
  -57 bites: commands 878,352, steps without 0, late 0, rewinds 0, stale 0, resyncs 0, heals 0, sentinels 0, measured
  incomplete 0 (whole-as-lost 48, client hashes 878,304), decided lost 0: all equal U4r's. Differ (expected, BRAWL-1
  changes the walk): holding every very large unit 864,000 (1.0000) on every run (U4r s1 and seed-1 bites 861,542 /
  0.9972); projectiles per set; audit counts; fate logs (U4r's s1 3dd20188 vs now 234eb9b5). tick16 differs as N5.
- N15 the post-wrap event rate (room.jsonl second records, tick16): BRAWL-1 last measured seconds 528-585 events/s,
  after seconds 432, 307, 300, 285, 265, 198, 340, 289 (U4r: 438, 270). The rate falls once commands stop landing on
  real ticks (option B re-runs the last command), so the queue fills at about 13.7 a tick to the first overflow
  (wrap + 75) and 8.7 a tick to the second (118 ticks): my P3 ruling's computed wrap + 54-60 used the measured rate
  and was wrong (ADV-P0-1, Low, mine).
- N16 HEAD 6041c49 (saver commits since 93509df touch only the card and my notes); HEAD's spike tree 07baa855 =
  93509df's; amended M7-3 (a) holds; untracked = only P0's three files. redact-check (read-only, tracked script) on
  P0, P2 and P3 results folders: 7 items, 0 findings. No redact log kept for the P3 (fc41111) and P2 (93509df)
  commits, unlike U4r's sets (.gate/u4r-sets/redact-<set>.log): ADV-P0-3 (Low).
- N17 P2 CORRECTION: the card says P2's "-57's nine values equal U4r's P2 ... largest change 0.0 %". -57's report
  p99 moved: 5,257 / 5,280 / 5,283 (U4r ab10f058a828) -> 5,111 x 3 (-2.78 / -3.20 / -3.26 %). Largest relative
  change at P2: s3's position p99, -3.26 %. Max values equal (6,984 / 2,750 / 88,063). ADV-P0-2 (Medium). P2 and P3
  room logs: no building entry in any phase. P2 results sha256: runs.txt c2995379…, verdict.json 1d2ff9e5….
- N18 disk: kept raw logs 11,683.8 MiB (U2 1,995.6; U4r 5,383.4; BRAWL-1 sets 4,304.7) + evidence 607.5 MiB; free
  7,722.8 MiB. U4g's refill ~9 GiB does not fit above the 4,096 floor: KEPT-LOGS now blocks U4g's build.
- RULING drafted: P0 STANDS AND COUNTS; tick16 shown (flag cleared under corrected TICK16-WORDING); commit P0's
  three files; re-run sets complete; report words for P3, P2, P0; ADV-P0-1 (Low, mine), ADV-P0-2 (Medium), ADV-P0-3
  (Low). For Mustafa: KEPT-LOGS restated (now in U4g's way), nothing to check in a browser.
- N19 close items found: u4r-report.md's BRAWL-1 section (:614-655) holds only round 2; condition 18's ADV-BP-1/-2
  words, round 1, the merge step, the evidence runs, the sets, EVQ-LOAD (not yet in the report) and the corrections are
  still to write; :562 (TICK16-WORDING) and the "tick16's shown rests on r2's 30 s record" line need updating.
- DONE: result returned in StructuredOutput. I changed nothing in any tree (only this notes file, by Write/Edit; the
  session's saver commits it).
