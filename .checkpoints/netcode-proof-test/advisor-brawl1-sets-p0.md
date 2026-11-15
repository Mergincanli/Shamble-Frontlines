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
