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
- Clean partner verified from tick16-clean-verdict.json: -2 11 of 11 pass, -17 0, -57 6,981/2,750/88,044, valid;
  ticks_below_wrap 600; run_lengths after 0, capped false.

## Point 2 (CR-ID): facts checked
- contract.rs:15-19 (module doc), :248-256 (room_class_filter_bite: npt_room::BITES.contains && !starts_with("PM-"),
  false in a measured build), :258-301 (audit), test :540-575 (6th doctored case; bites-build exemption for
  owner-to-all, another bite fails). room/src/lib.rs:32-46 documents the one non-PM id (7 ids).
- Brief section 4 (:1023-1026): "npt-lab's sources never spell another crate's bite id as a string literal" (all ids,
  not only the PM- form); check.mjs BITE_ID (:887) = PM- form only, so the scan would not catch it: the rule would
  still be broken. Tests may name ids (scenario.rs:1852-1856 and contract.rs:569 are in test modules).
- A measured build cannot make an owner-to-all run (C4; --bite in a measured build is a usage error), and both run
  scripts run each suite's contract with that suite's own build (c-u4r-r2.mjs:129-130). r4's owner-to-all runs are
  unit tests (scenario.rs:1811-1860), no contract over them. So the measured-build no-exemption is fail-closed.
- Risk: a second non-PM id in npt_room::BITES would widen the exemption silently. Guard: assert the count is 1 in
  the bites-build test (rides the current work: Low).
- Decision: keep the run-time lookup. Severity none (+ one Low). My ruling 3 words read "npt-room's class-filter bite
  (catalogue-5/owner-to-all), exempt by a bites-build contract".

## Point 3: facts checked
- Ruling 3 point 3 Decision (build3-full :27) and main-session item 4 (:29); point 1 Decision (:13: "-18's bitten
  judged run moves to U4g") and main-session item 2 (:15). Both items are mine: confirmed.
- Snapshot (wip/u4r.patch 707cc3df): TICK_START 63_736 for every run; bitesOn -18 -> "B1"; no-fallback run on B1.
- Live worktree c-u4r-runs.mjs (copy 5, resumed): :20-27 header, :60-65 TICK16_START = 65,536 + 15 - (10+600)*30 =
  47,251; :68-69 NOT_AT_U4R no-fallback; :187-193 bitesOn -18 -> null; clean and client-rounding at 63,736. Correct.
  Wrap at measured tick 17,985, 15 before the end; ticks_below_wrap 18,285.
- Carry 58 applies to the judged tick16 bite run (expected resyncs ~48 at about wrap + 60, after phase): with the set.

## Further finding (setup check vs carry 73 and whole-as-lost): decided form
- Plan §11 (u4-build-plan.md:1081-1093) lists stricter readings for Mustafa's go (e.g. FX-5 (c)); carry 62 (h)'s check is
  not on it. Decision: the check goes into marks.json as a plain check of -17's L1 P0 part (no "setup": true, the name
  without "setup: "), stricter on clean runs (FAIL, not INVALID), both bites then Shown (tick16: mismatch 2,880 and
  incomplete 720; whole-as-lost: incomplete 48). Amends carry 62 (h)'s JSON and ruling 1 point 4 ("JSON stays word
  for word"; "setup check FAIL by name"): both the advisor's words. Added only at 7.1 after the record run's clean
  short L1 P0 sums are 0; onto plan §11's readings-PR list. Medium; not blocking; Mustafa via readings PR.

## Status
- All three points ruled; result returned by StructuredOutput (2026-11-14).
