# Advisor: U4r build, copy 3's eight points (2026-11-14)

Notes file for the advisor's ruling on u4r-build3-questions.md. Plan first, then a line per item as it is done.
On a restart: read this file, continue after the last item marked done.

## Plan
1. Read sources: questions (done), builder notes :571-906, the brief's relevant sections (2.4 r2, 7.1, conditions L1-L6,
   lanes), carries 49/57-62/73/77/78, the U4 plan §4.5/4.12/4.14, rulings 1 and 2, history.md (Mustafa's words on
   PM-0001-2, review panels, Low never triggers push, LEFTOVER-DELETE, condition 5).
2. Check key code/log claims myself per point (spot-check, read-only).
3. Rule on each point 1-8 and write the ruling here, a line per item.
4. Return the structured result.

## Items
- [done] Read: advisor.md (main), questions, notes :571-906, ruling 2, brief 2.4 r1-r10 and 2.5-2.7, carries 40-78,
  history.md :280-420 (Mustafa: "The 9.5 floor (PM-0001-2) does not move." :338-339; "A Low never triggers a push."
  :340; "The review panels stay as they are until I decide." :310; LEFTOVER-DELETE :404-406). pre:PM-0001-2 on -17/-18
  is the advisor's Q22 ruling (plan §4.12 :416-424), not Mustafa's. ADR 0001:1589 (PM-0001-2: "every client's relevant
  set holds all 47 other soldiers on at least 99 % of its snapshots ... proposed share"; downstream "every run of
  PM-0001-4 to PM-0001-27"). SERIES.md:368 flag's writer "npt-room"; U4r's lab computes it from the mirror frame
  (logs.rs:538-541; mirror.rs:1-8, "the frame the room meant that client to hold").
- [done] Restart (copy 2 of this advisor, fresh context): re-read advisor.md, questions, notes :571-906, ruling 2
  whole, brief 2.4 and 7.1 and section 4's lanes, carries 40-78, history.md :281-345, :364-415, :1928-2027 (also
  SIZE-CHECK :398-399, USAGE-SAVINGS :291-299).
- [done] Code/doc checks for point 1: mirror.rs:1-8, :314-337 (flag = the mirror frame holds every other soldier; ES
  and vehicles), :396-414 (a missing baseline frame is a breach and returns early: that tick's stats not updated);
  room.rs:1163-1170 (baseline, stale under no-fallback, else defaults age 0), :1208-1235 (events oldest first before
  any entity change), :1236 on (nearest 8 then the rest). ADR 0001 is "Status: Proposed" (:3); :1589 scopes the share
  "for the downstream rows", last column "every run of PM-0001-4 to PM-0001-27"; :705 the downstream estimate
  "everyone in view". marks.json: -17 and -18 L1 parts validity [pre:PM-0001-2] (notes: "the advisor's Q22 ruling");
  -57 parts none; -58 window all. Plan §4.12 :416-424 (Q22: -17 "the codec under the real load", -18 "B1 ... inside the
  band"), §4.5 :296-300, §4.14 :461 (-18 bite "a gap evicts B"), §5 U4g :818-833, order :882 (U4r, then U4f, U4g,
  U4h); RQ1 :114 is U4-wide ("bursts (-18)"). Plan §10/§11 :970-976, :1080-1093: the readings list has Q22's -22
  exception only, not Q22's share on -17/-18. Carry 78 (u4s-carries.md:546-553): "so no U4e or U4r run meets it".
- Point 1 draft: High (npt-room's recovery, not U4r's code). Q22 stands, no bar moves; the FAIL stands (condition 5);
  B1 judged runs stay in U4r's set for -57 (no pre), -17/-18 B1 recorded INVALID, never counted; their passing evidence
  moves to U4g with carry 78 widened (relevant soldiers the baseline lacks first; form is U4g's plan; target share
  >= 0.99 at 10 bursts in 600 s = at most 18 flag-0 ticks per client per burst on average). Readings PR gains Q22's
  share on -17/-18 (ADR's "for the downstream rows"). Not to Mustafa now.
- [done] Own checks: b1 room.jsonl: least slot 246 of 900 (0.273333), window means 47.9 ... 46.3, 96 burst_eviction
  (first t 64217 age 0). tick16c: flag 48 for 65,507-65,566, 0 from 65,567; segments as the fact check. lab class_audit
  breaches: b1-nofallback 2,314 (first "tick 64218: client 23: its baseline frame (age 35) is not mirrored"), b1 0,
  tick16c 0. judge figures.rs:236-266 slot_hashes reads every phase; :316-440 decoded() counts an orphan as a
  mismatch; :542-554 mismatches before gap. scenario.rs:989-1012 heal record (h != 0, due >= return, not a FullState
  tick since the burst start); :1034-1049 the cap exception for any run with a bite id. bots lib.rs:226-245 brawl aims
  at the target's position. contract.rs:16 doc vs :140-147 code (carry 60 fails); :350-351 read errors exit 1.
  room.rs:238 `ClientSlot::last_acked()` is public (the mirror can keep that frame). marks.json -17/tick16: "the bite
  run must cross it". Plan U4f builds the cheats' Behaviours, U4h the TC shooter (both open npt-bots); ADR 0001:1600
  -12's ES runs at P2.

## Rulings (the result)
- P1 High, not blocking, not Mustafa. Designed eviction + defaults (ADR part 6; plan §4.5, §4.14), carry 78's known
  packing defect (premise now false), and a plan gap (no text on when the set is whole again; my Q22 checked shots
  only). FAIL stands (condition 5). No bar moves: Q22 stays, share stays, the flag's input stays. Rejected: exempt or
  window B1, read interest set, longer ring, full state now. Fix and -17/-18 L1 B1 passing evidence to U4g (carry 78
  widened; target share >= 0.99 at 10 bursts in 600 s, at most 18 flag-0 ticks per client per burst on average; short
  B1 suite of U4g's build first; form is U4g's plan, ADR words via U4g's plan and the readings PR). U4r: B1 judged runs
  stay (for -57), -17/-18 B1 recorded INVALID, never counted; -18's bitten judged run moves to U4g. Readings PR gains
  Q22's share on -17/-18 (ADR's "for the downstream rows").
- P2 Medium, not blocking, not Mustafa. Band stays (9.5 his floor; 10.5 marks.json); SS's brawl unchanged; ES-only
  seeded hit rule, value by advisor-u4d-close.md:90-105's form at P2, 3 seeds, 10-min window; owner U4h (opens
  npt-bots; lag compensation changes hits). U4r: report only.
- P3 Medium, blocking (r2's carry 73 item). Wrap inside the measured phase 15 ticks before its end: short bitten run
  --tick-start 64651 (10 s + 20 s), clean partner --tick-start 64936 (measured build); judged bite run tick_start =
  65,551 - (warm-up + measured ticks) (47,251); clean -17 runs and client-rounding stay at 63,736. No code, no bar.
- P4 Medium, not blocking, not Mustafa. Brief's sentence wrong (contradicts carry 77 (b), which it cites). r2 reads:
  bitten max fails (1.688 s), fast heals counted apart (14 age 0; 11 ages 33-34, a second case, derived), 71 slow after
  FullStates; clean 96 of 96. U4g closes the hole (ack.rs; Low).
- P5 Low. Copy 3's reading accepted (exception at the recording's last L - 1 ticks); two Lows on r6 (literal lead 2;
  first-64 compare) ride the next precheck.rs edit.
- P6 Medium, blocking. Design accepted (contract and precheck beyond 2.7's list, named). Conditions: breach check
  (fail > 0 unless catalogue-5/owner-to-all; sixth doctored case); doc :16; read errors own exit; c-u4r-r2.mjs so the
  main session re-makes r2 from the merge commit (the record) before -17's judged runs and carry 62 (h)'s decision.
- P7 none. Both fixes accepted; carry 62 (e)'s reading for bite runs; point 3's clean partner shows (b) past the wrap.
- P8 Medium (the mirror). Mirror keeps each client's frame at last_acked() beyond FRAMES_KEPT; short bitten B1 run 0
  breaches; clean P0 equal record for record. Lows: baseline_age row words; 21 rows not 22; 65,715 not 65,746; 0.48 not
  "as the clean b1"; regen/ labels; burst_spacing_ns written on B1 headers; bytes rounding; SERIES.md:368 writer.
- Further: Medium, -18's heal reading passes a blind room (96 of 96 within 130 ms beside 0.273): readings PR, his go.
- [done] Result returned by StructuredOutput.
