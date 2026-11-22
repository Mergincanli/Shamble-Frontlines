# Advisor: the one review of the NPT-TRIM re-plan (replan.md), 2026-11-21/22
Role: main's .claude/agents/advisor.md (plan review). Rule: NPT-TRIM, history.md:7561-7593 (over rev 3.3 and TEST-FIRST).
Read only; no builds. Severity on CLAUDE.md's one scale.

## 0. Plan of the review (blocks; each written when finished)
1. NPT-TRIM's asks, one by one, against replan.md (screen, date, table, MOVED, promote, U6 cut, crate, certificate,
   lag compensation's piece).
2. The marks: kept bars and run counts unchanged; every moved mark has a real-game destination that exists in the
   build plan; per-part marks name their parts; spot-check marks.json and plan.md §6.2/§7.2 lines.
3. Flags F1-F13: rule each; mark which are Mustafa's.
4. Arithmetic: unit sizes and sums; run hours and caps by plan.md §7.1 (:786-788, :794-795); R1's date vs the pace.
5. The cut's safety under his firewall; DNS-01 by hand vs "no session handles secrets" and "DNS only".
6. The four decisions; anything missing that is his.
7. Order and R1's date.
8. Verdict, findings ADV-RP-n, paste-ready replacement text.

**Resume here:** blocks 0-3 written; next block 4.

## 1. NPT-TRIM's asks, each in its own terms
| Ask (history.md) | Answered? | Where / gap |
|---|---|---|
| One screen: units+sizes, runs+hours+caps, R1's date (:7584) | yes | replan.md:5-43 |
| R1 by Wed 2 Dec, else three lines why (:7584) | yes | :38-43 (line 3 says what would bring it closer, not why: acceptable) |
| All 81 marks, kept (unit/run) or moved (change-set or slice) (:7585) | yes, 3 gaps | -34/-35 "RP" and -55 "where vehicles enter" carry no number (ADV-RP-7) |
| MOVED never PASS, reviewer's read (:7585) | yes | replan-marks.md §6; timing unplaced (ADV-RP-2) |
| Promote list (:7586) | yes, 1 gap | npt-l1 overlaps change-set 3's own L1; the lab's "why not" missing (ADV-RP-11) |
| U6 cut named item by item (:7571) | yes | replan.md:49-53; replan-units.md §1 |
| One WT crate and why (:7571) | yes | :54-58 |
| Certificate behind the firewall (:7571) | yes | :59-63 (block 5) |
| Lag compensation's smallest piece, said (:7577) | yes, 1 gap | names budget marks, not the density marks he asked about (ADV-RP-10) |
| Nothing dropped (:7576) | **no hand-off** | moved marks reach no one outside this branch (ADV-RP-1) |
| Review seats per unit (:7590) | **departs** | security dropped from internet-facing U6b/U6c (ADV-RP-3) |
Spot-checks held: room.rs:32 HISTORY_MS 2,000 and rewind.rs:18-33 rewind every command; marks.json -7, -28, -29, -36,
-56, -58 parts and checks; build plan :371-419 (CS6-CS9), :893-902 (Slice 3), :317-318 (CS3's codec and L1); git log
of crates/ 15-21 Nov: about 8,226 added lines = 1,175 a day (replan.md:40 holds).

## 2. The marks: fates checked
- Counts hold: 48 + 24 + 9 = 81; leans 52/29; per run R1 9 (+2), R2 23, R3 18, R5 13, L1 9 (I recounted each list).
- No kept mark's bar or run count changes except where it goes to him: -7's span (notes) and -56's count (check value)
  are MARK-NOTES; -58's Predictor check and lt-tick-offset bite move under NPT-TRIM's MOVED. R2-R5 at 1,000 B as rev 3.3.
- Every moved mark has a destination; the build plan has CS6-CS9 and Slice 3 as cited. Unnumbered: -34 and -35 ("RP":
  the codec and transport interface are change-set 3's, build plan :317-318); -55 (vehicles have no slice yet; the
  build plan puts them in Slices 4-6, :889). SB, Safari and RD are his own milestones (BASELINE-WHEN (b), BROWSER-SET
  (a), OQ-0003-1): acceptable.
- **-56 is wrong as framed to him:** item 1 is "both crates' results on -11, -25, -28, -35 and their pick" (marks.json
  -56 notes): with one crate, only the carried crate's -11 and -28 results stay; item 4's "each browser's received
  snapshot rate and age at P2 to P4" rode R7's ML legs (plan.md:740), which move; item 2's UDP half rests on F3. So
  "count 4" holds only with items 1 and 4 narrowed in the notes text, which is his too (ADV-RP-4).
- Kept marks whose judged parts need ML (the named minimum laptop, not his MSI: plan.md:188-189): -1, -14, -28, -29,
  -36, PM-0006-16, PM-0006-25 (marks.json parts "ML ..."). Without ML they read NOT-RUN plus rough ML\* readings until
  R8 (rev 3.3's later run, plan.md:826, :1293), which the screen omits (ADV-RP-5).
- PM-0001-28 per part (replan.md:56-58) over moved whole (replan-units.md:82-83): **per part is right.** His rule of
  14 Nov measures a mark "at the first build that can show it"; this build can show the carried crate's parts (and
  its control "a wrong certificate hash fails 100 of 100", which has no machine filter and guards every R1 WebTransport
  session); it cannot show the other crate's, so those move. R2-TRN-OQ1 stays open, as written.

## 3. The flags, ruled
| Flag | Ruling | Whose |
|---|---|---|
| F1 -58 kept, Predictor check and bite moved to CS7 | accept: R5 crosses every 16-bit wrap | plan; **U4G-Q2 (his, -58's two lost-tick checks, u4g-brief.md:249-257) returns before -58's set** (ADV-RP-8) |
| F2 -28 per part | accept (above) | plan (MOVED, under NPT-TRIM) |
| F3 -29 and F4 PM-0006-25 into R1 | accept: his question names the fallback (:7567); +0.45 h inside R1's 4 h | his to see (reads his R4 line): in RE-PLAN-OK |
| F5 PM-0006-26/-27, F6 PM-0006-15 move with R4 | accept | plan |
| F7 -54 to CS9 | accept; -12 re-runs if E3b (ADR 0001:1653) | plan |
| F8 -55 | accept; destination worded as ADV-RP-7 | plan |
| F9 -7 at the room's 2,000 ms span | accept: the rule cap is P2's honest p99 (ADR 0001:1646), far under 2 s; memory only grows with span | his (marks.json notes) |
| F10 carry 93 on the kept build | **my ruling:** the carry's "U4h's build with lag compensation" is met by the kept room, which rewinds every firing command (rewind.rs:18-33); the rule's record and -12's ES header name what is absent (no estimate, tolerance or cap) | plan (an advisor carry) |
| F11 -56 to 4 items | accept the direction; texts of items 1, 2 and 4 narrow (ADV-RP-4) | his (check value and notes) |
| F12 the laptop | reframe (ADV-RP-5) | his (OQ-0003-1) |
| F13 OQ-0001-6's pick to Slice 3; R2/R5 on option B | accept; the game lane's build plan (:389-391, :909: "the proof test still chooses") must hear it (ADV-RP-1) | his to see: in RE-PLAN-OK |
