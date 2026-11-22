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

**Resume here:** blocks 0-7 written (6-7 at the end); next block 8 (verdict, findings, paste text).

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

## 4. Arithmetic (plan.md §7.1: CCX23 0.1386, CAX21 0.0176 EUR/h; cap = hours + 1 per VM, up to the next 0.10)
- Runs: R1 4/4 = 0.62, cap 5/5 = 0.78 -> 0.80; R2 11/11 = 1.72, cap 1.87 -> 1.90; R3 10/10 = 1.56, cap 1.72 -> 1.80
  (8.85 h of blocks: 10 keeps rev 3.3's slack, one more hour than rounding needs, fine); R5 17/56 = 3.34, cap 18/60
  (RS + 4 LG) = 3.55 -> 3.60. Sums 42 / 81 h; 42 x 0.1386 + 81 x 0.0176 = 7.25 (the rows' rounded sum is 7.24, same
  method as rev 3.3's 13.51); caps 8.10. R1's 2.65 h + F3 0.25 + F4 0.2 = 3.1 h (plan.md:818, :821) plus two unsized
  self-tests, inside 4. R5's 12.95 h of blocks stay billed 17 by the 20:00 start of D3 (plan.md:833): right.
- Units: U6a-U6e 9,205-9,455; all eleven kept rows bar U4f2 = 14,565-17,315 (screen: 14,600-17,300); likely bands are
  x1.35-1.5 of each estimate; largest at 1.5x is U6j 4,575, under SIZE-CHECK's 6,000. MOVED 300-600 + 60-120 data =
  360-720 (replan-marks.md:210-211). All hold.
- R1's lane: U6a-U6c likely 7,850-8,750 + U7's Rust 486-540 = 8,336-9,290 (screen: 8,350-9,250; fine) in Mon 23-Sun 29
  = 1,190-1,330 a day against 1,175 measured (block 1): Wed 2 needs a faster week than the last, with no lost day.
  "Planned Thu 3, aim Wed 2" is the honest line. Disk: card :147 shows 8.59 GB free before -21's set; PACK-P0 is real.

## 5. The cut's safety behind his firewall; the certificate
- **Nothing an honest kept run needs is cut.** -29's UDP block stays a room-wide socket control per peer set
  (plan.md:1076), so cutting per-session BlockUdp costs R1 nothing; the connection cap (N + 8) still bounds DelayStream's
  0.31 MB per connection; the Join deadline keeps a stuck own client from holding a slot; panic-free decode and size
  limits are built; strays stay counted and INVALID; the judge's every-run capability rule (eval.rs:598-613) holds with
  no session controls; no kept mark's check reads a cut feature (marks.json: addr_cap, Origin, cycler and capability
  appear only in -25, -43, -44, -51, -53 and in -28/-29 notes). systemd's per-room MemoryMax goes: a leaking room is
  then killed by the kernel and -15/-27 record it; acceptable.
- **Kept only for strangers, but cheap:** the outbound drop after build, the CT ledger, key-only SSH: keep, no cost.
- **DNS-01 by hand meets both rules:** no token exists (plan.md:986's objection gone); the ACME account is fresh per
  VM and the TLS key never leaves RS (plan.md:974-975); TXT records "are always DNS-only" (Cloudflare, cited in
  replan-units.md:93) and `rs.nt` stays DNS only (plan.md:943); his login with two-factor is the only credential and no
  session sees it. Inbound: nothing opens; TCP 80 leaves both lists.
- **One claim does not hold:** replan.md:62 keeps HTTP-01 "only as the fallback", but its responder and port window
  are cut (replan-units.md:32). No fallback is built: a failed DNS-01 order stops the run (ADV-RP-9).
- R2, R3 and R5 have no browser leg: each could skip the public order (LG pins the room's own certificate by hash),
  saving him three orders, six edits and three CT entries; R1 split would make a fifth production order for `rs.nt`
  inside 7 days, at the limit of 5 (plan.md:979) (ADV-RP-12).

## 6. The four decisions, and what else is his
- RE-PLAN-OK: right scope, but its leans hide behind codes (F1-F8, F10, F13) that CLAUDE.md's reporting rule forbids
  unexplained; four readings of his words need a plain line each: R4's two non-hostile rows into R1 (F3, F4),
  OQ-0001-6 leaving this test (F13), U7's Wasm codec under "no sim in Wasm", DNS-01 adding his TXT edits (ADV-RP-6, -15).
- MARK-NOTES: -7 right; -56 incomplete (ADV-RP-4). LAPTOP: misframed (ADV-RP-5). PACK-P0: right (his by PACK-READ (a);
  kept-logs.mjs packs with a sha256 list and a read-back, verdicts kept).
- His and missing: U4G-Q2 with F1 (later, before -58's set: name it, ADV-RP-8); R1's day (a run day is his, TEST-FIRST:
  history.md:2262) and his ~2 h on it; the paste block to the game chat (an action, not a decision: ADV-RP-1). No
  money beyond each run's cap, which each run plan brings; nothing on main before R1 (R-PR #41 merged, W-PR gone).

## 7. Order and R1's date
- The order holds (lane-bound: R1 waits on U6a-U6c's lines whatever the order). Fix three things: MOVED's slot (first on
  the lane after R1, R1's report after it: ADV-RP-2); U4's report Mon 23 with U4k a new unit after R1, as replan.md has
  it, not U4's close after R1 (replan-units.md:205: ADV-RP-14); Stop B's read may start when U6c merges, beside U7's
  Rust half (three agents), which can bring his Stop B steps to Mon 30 if the read is clean (ADV-RP-16).
- Date: keep "planned Thu 3 Dec, aim Wed 2"; nothing found moves it either way.

## 8. Verdict: APPROVE WITH CONDITIONS
The re-plan answers NPT-TRIM, keeps every bar, and its sums hold. Conditions: apply P1-P6 below verbatim to replan.md
before it goes to him (verbatim needs no re-check; other wording of ADV-RP-1 to -9 comes back to me); the Lows ride it.

**Findings** (CLAUDE.md's one scale; cite: what is wrong; fix)
- **ADV-RP-1 High** (replan.md:45-78; history.md:7576): the 29 moved marks and OQ-0001-6 reach no one off this branch;
  the game lane's build plan still says this test picks OQ-0001-6 (:389-391), and its next change-set (3) is the first
  destination and builds its own L1. A rule a session would hit; marks could drop. Fix: P3's hand-off bullet, P6 item 2.
- **ADV-RP-2 Medium** (replan.md:18, :75-77; replan-marks.md:213-214; replan-units.md:205-206): MOVED has three times
  and no slot on the lane. Fix: P2 (first after R1; R1's report after it).
- **ADV-RP-3 Medium** (replan.md:78; history.md:7590): security dropped from units that face the internet in a run
  (U6b transports, U6c page host and ACME client, U6e metrics exporter). Fix: P2.
- **ADV-RP-4 Medium** (replan.md:83; marks.json -56 notes): "count 4" is true only with items 1, 2 and 4 narrowed in the
  notes (his text too). Fix: P5's MARK-NOTES.
- **ADV-RP-5 Medium** (replan.md:84-85; plan.md:188-189, :826): the laptop is ML, the typical student laptop he names
  for the whole game (OQ-0003-1), not his MSI; seven kept marks' judged parts wait on it; R8 is off the screen. Fix: P1,
  P5's LAPTOP.
- **ADV-RP-6 Medium** (replan.md:81; CLAUDE.md, Reporting to Mustafa): codes unexplained in a decision; no "For
  Mustafa" section. Fix: P5, P6.
- **ADV-RP-7 Medium** (replan-marks.md:54-55, :79; history.md:7585): -34/-35 ("RP") and -55 name no change-set or
  slice. Fix: P4.
- **ADV-RP-8 Medium** (replan-marks.md:112-113; u4g-brief.md:249-257): F1 brings back U4G-Q2 (his) before -58's set;
  not named. Fix: P5's RE-PLAN-OK item 5.
- **ADV-RP-9 Medium** (replan.md:62; replan-units.md:32): HTTP-01 "stays only as the fallback", but it is cut. Fix: P4's
  last sentence.
- **ADV-RP-10 Low** (replan.md:64-66): he asked about density marks; say R5's (-27, PM-0006-13, -14) pay the rewind in
  every room, and that U4h's per-shot checks are not in the numbers.
- **ADV-RP-11 Low** (replan.md:70-73): npt-l1 meets change-set 3's own L1 (build plan :321-322; "not built a second
  time"): offer it to that plan or keep it here; give the lab's reason (bound to the stand-in sim; replan-units.md:187).
- **ADV-RP-12 Low** (replan-units.md:117-118): R2, R3, R5 have no browser; their run plans may skip the public order
  (LG pins the room's certificate); a split R1 makes a fifth order for `rs.nt` in 7 days (plan.md:979).
- **ADV-RP-13 Low** (replan.md:35-36): his part per run omits typing his home address into the firewall: add it.
- **ADV-RP-14 Low** (replan-units.md:205 vs :197): U4 closes with its report on Mon 23; U4k is a new unit. Fix: P2.
- **ADV-RP-15 Low** (replan.md:67-69): U7's Wasm codec reads "no sim in Wasm": show it. Fix: P5 item 3.
- **ADV-RP-16 Low** (replan-units.md:201-202): Stop B's read can start at U6c's merge beside U7's Rust half. Fix: P2.

**Paste-ready text**
P1. After replan.md:33 (the All row), add:
`| R8 (later, rev 3.3's own short run; not in the sums) | only if ML is not named by R1: ML's judged legs of -1, -14, -28, -29, -36, PM-0006-16, PM-0006-25; no Safari | 3 | 3 | 0.47 | 0.70 |`

P2. Replace replan.md:74-78 (Order) with:
- **Order** (replan-units.md §7): U4f2's merge and -21's runs (Sun-Mon), then U4's report (Mon); U6a, U6b, U6c and
  U7's Rust half on the build lane; U8 and U7's TypeScript beside it; Stop B's read (security and infra on the run
  package, U6c's host and the wrappers: U8's one round) from U6c's merge, beside U7's Rust half; R1. After R1: MOVED
  first on the lane, then R1's report (a moved part reads NOT-RUN until then, never PASS); then U4k (a new unit), U6d,
  U6e, U6j; R2, R3, R5. One round per unit, netcode and test, plus security on U6b, U6c and U6e (transports, page host
  and certificate client, metrics exporter: they face the internet in every run); one advisor check per brief.

P3. After replan.md:73 (Promote), add:
- **Hand-off, so nothing is dropped:** the moved table (each mark or part, its marks.json bar unchanged, its
  destination) reaches the game lane before change-set 3's plan, its first destination: a paste block for your game
  chat (For Mustafa, item 2), then this test's final report on main. That lane changes, on your go, its build plan's
  OQ-0001-6 line (Slice 3 now picks: :389-391 and Part 6) and takes the moved marks into its change-sets.

P4. Append to replan.md:48: " Destinations by number: Slice 1's change-sets 6 (Look), 7 (Lag), 8 (Shoot) and 9
(Throw), Slice 3, slice 6; -34 per message type as the real protocol adds it (change-set 3's codec first), -35 at
Slice 3 (its swap needs the second transport); -55 at the first slice that adds vehicles (not cut yet); R4's rows at
the security baseline before the first online playtest; Safari later (BROWSER-SET (a)); RD's parts when RD is named."
In replan.md:62, replace "it stays only as the fallback." with "it is not built: a failed order stops the run and
comes back to you."

P5. Replace replan.md:80-87 with:
## Decisions for you (each with my lean)
- **RE-PLAN-OK:** this plan as a whole. It reads your words in five places; say if one is wrong. (1) Two R4 rows that
  are not hostile input stay, in R1 (+0.45 h): the fallback when QUIC is blocked, and the certificate pinned by hash
  in every browser; your question names the fallback. (2) The choice between the two ways of timing commands
  (OQ-0001-6) leaves this test for Slice 3, since its inputs moved; R2 and R5 run the draft's buffered way, R3 tests
  both. (3) U7 runs the bots' own connection code as WebAssembly (codec, acknowledgements, clock); it steps no world,
  so "no sim in Wasm" holds, and there is one implementation, not a second in TypeScript. (4) The certificate comes
  from a DNS record you add and delete, so nothing opens to the internet. (5) -58 (counter wraps) stays, its two
  prediction checks moved; that brings back U4G-Q2, your yes to the wording of -58's two lost-tick checks, before
  -58's set, after R1. The table's other leans: replan-marks.md §2. Lean: yes.
- **MARK-NOTES** (marks.json text, yours). (1) -7: R2 measures lag compensation's time and memory at the room's 2,000 ms
  history, which covers any cap the real game sets (the cap -7 was sized from moved). (2) -56's record counts 4 items,
  not 10: items 5-10 move with their marks; item 1 keeps only the carried crate's results on -11 and -28 (the other
  crate, -25, -35 and the pick move to Slice 3); item 2 keeps each browser's draft and its fallback (with RE-PLAN-OK
  (1)), Safari later; item 4 keeps NewReno against Cubic, while the browsers' rates at P2 to P4 move with the human
  run. Lean: yes to both.
- **LAPTOP:** browser legs are judged only on ML, the typical student laptop with integrated graphics that you name for
  the whole game (OQ-0003-1); your MSI gives rough readings only. Seven kept marks wait on it (-1, -14, -28, -29, -36,
  PM-0006-16, PM-0006-25). (a) Name ML and bring it to R1: judged there. (b) Not yet: R1 reads rough, and R8, a short
  later run (about 3 h, cap EUR 0.70), judges them once ML is named. Lean: (a) only if such a laptop is at hand; else (b).
- **PACK-P0:** may I pack the two closed P0 run folders and the c-u2 folders (about 8.6 GB down to about 2.4 GB, with a
  checksum list and a read-back; their verdicts kept) so the build lane stops hitting the disk floor? Lean: yes.

P6. Append after P5:
**For Mustafa:** 1. Answer the four decisions in one message. 2. After your yes, paste the moved-mark block I give you
into your game chat, before its change-set 3 plan. 3. Say whether Wed 2 or Thu 3 December suits you for about 2 hours
at home (R1). 4. Before R1: Chrome and Firefox on the laptop; Stop B's steps (about Tue 1 December).
