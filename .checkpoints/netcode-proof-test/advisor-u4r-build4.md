# Advisor: U4r build ruling 4 (u4r-build4-questions.md), notes, 2026-11-14

## Plan
1. Read: advisor.md from main (done), the questions (done), copy 5's notes (done), ruling 3 point 3 and point 6 (1)
   (done), the brief's rows for -17 tick16 / 7.1 / section 4's lanes, carries 62 (e) and 73, marks.json's -17/tick16
   text, history.md for Mustafa's words.
2. Point 1: check c5-p3.log and r2/tick16d/ (room.jsonl flags, overflow_resync ticks, run_lengths, summary); check
   scenario.rs's after-phase exit path and cap exception; decide whether after_capped true is a needed outcome.
3. Point 2: read contract.rs's audit and room/src/lib.rs:32-46; brief section 4's spelling rule; check.mjs's scan.
4. Point 3: read c-u4r-runs.mjs's tick16 and -18 lines; ruling 3 points 1 and 3 main-session items.
5. Write a block per point here as I go; then StructuredOutput.

## Progress
- (blocks follow)

## Point 1 (P3-CAP): facts checked
- c5-p3.log: tick16d 946 ticks (46 after), overflow resyncs 48, sentinels 2,880, measured incomplete 720, decided lost
  at the stop 1,488; contract PASS (class audit 0); judge: -17/tick16 shown, run valid, mismatch 2,880 (verdict json).
- room.jsonl flag segments (my python read): 48 64,652-65,566 (warm-up, measured, after), 0 65,567-65,595, 48 at
  65,596; phases 300/600/46. lab.jsonl: 48 overflow_resync all at 65,596; run_lengths after_capped false, after 46.
- client.jsonl: 65,536-65,595 null on all 48 (60 ticks), 65,596 decoded (= server_hash slot 0), 60 orphans at
  2^32+0..59 (2,880 = 48 x 60). Split consistent with ring exit: 65,536-65,564 (29) resolved lost after the full state,
  65,565-65,595 (31) at the stop (1,488 = 48 x 31). measured incomplete 720 = 48 x 15 (65,536-65,550).
- scenario.rs:1033-1052: normal stop = k >= measured_end && written_through(last_measured); cap branch only when
  not. after_capped read by nothing else (grep: SERIES.md row, c-u4r-r2.mjs text, scenario.rs only; judge, contract
  don't read it).
- tick16b/c (tick_start 65,506): overflow resyncs at 65,605 (48) then 92 more; after_capped true, 150 (the cap path
  shown there).
- Decision: (a). The flag is a path record, not a result; my ruling 3 expectation mis-predicted. No re-run, no new
  setting. Correct my words; c-u4r-r2.mjs expect text follows. Severity Low (my wrong expectation; nothing load-bearing
  differs). Not blocking. Not Mustafa.
- FURTHER (found here): measured incomplete 720 on a short L1 P0 run vs carry 62 (h)/U4R-Q2 (c): if the setup check
  "L1 P0 loses no group" is added to -17's L1 P0 part, the tick16 bite run is INVALID by it (eval.rs:1189-1208: setup
  fail -> invalid; :1476/1660 bite Invalid -> mark INVALID), contradicting carry 73. Also whole-as-lost as a bite entry
  would be INVALID (expect-invalid bites name only validity rules: marks.rs:862-876). Medium; main session's 7.1
  decision; to me before the check is added.
