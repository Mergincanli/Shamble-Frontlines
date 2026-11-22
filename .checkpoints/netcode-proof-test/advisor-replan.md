# Advisor: the one review of the NPT-TRIM re-plan (replan.md), 2026-11-22
Role: main's .claude/agents/advisor.md (plan review). Rule: NPT-TRIM, history.md:7561-7593, over rev 3.3 and TEST-FIRST.
Read only. CLAUDE.md's one scale. Plan: 1 his asks; 2 the marks; 3 the flags; 4 arithmetic; 5 the cut and the
certificate; 6-7 decisions, order, date; 8 verdict, findings, paste text. **Resume here:** all blocks done.

## 1. NPT-TRIM's asks, each in its own terms
- Answered: the one screen (replan.md:5-43); R1's date with three lines (:38-43; line 3 says what brings it closer, not
  why: acceptable); the 81-mark table; MOVED never PASS with a reviewer's read (replan-marks.md §6); the promote list;
  U6's cut item by item (:49-53; replan-units.md §1); one crate and why (:54-58); the certificate (:59-63).
- Gaps: no hand-off of the moved marks to the game (ADV-RP-1); MOVED unplaced (-2); security dropped from
  internet-facing units against :7590 (-3); -34/-35/-55 unnumbered (-7); npt-l1 meets change-set 3's own L1 (-11);
  lag compensation's answer names budget marks, not the density marks he asked about (-10).
- Spot-checks that held: room.rs:32 (HISTORY_MS 2,000) and rewind.rs:18-33 (every command rewound); marks.json parts and
  checks of -7, -28, -29, -36, -56, -58; build plan :317-322 (change-set 3: codec, L1), :371-419 (6-9), :893-902
  (Slice 3); git log of crates/ 15-21 Nov: about 8,226 added lines, 1,175 a day (replan.md:40 holds).

## 2. The marks: fates checked
- Counts hold: 48 + 24 + 9 = 81; leans 52/29; R1 9 (+2), R2 23, R3 18, R5 13, L1 9 (each list recounted). No kept bar or
  run count changes except via him: -7's span and -56's count (MARK-NOTES); -58's Predictor check and lt-tick-offset bite
  move under MOVED; R2-R5 at 1,000 B as rev 3.3. Every moved mark has a destination; CS6-CS9 and Slice 3 exist as cited.
- **-56 as framed is wrong:** item 1 is "both crates' results on -11, -25, -28, -35 and their pick" (marks.json -56):
  one crate leaves only the carried crate's -11 and -28; item 4's "each browser's ... rate and age at P2 to P4" rode R7's
  ML\* legs (plan.md:740, :824), which move; item 2's UDP half rests on F3. "Count 4" needs items 1, 2 and 4 narrowed
  in the notes, his text too (ADV-RP-4).
- Judged parts that need ML (the named minimum laptop, not his MSI: plan.md:188-189): -1, -14, -28, -29, -36,
  PM-0006-16, PM-0006-25. Without ML: NOT-RUN plus rough ML\* readings until R8 (plan.md:826, :1293), off the screen (-5).
- **PM-0001-28 per part (replan.md:56-58) beats moved whole (replan-units.md:82-83).** His rule of 14 Nov measures a mark
  "at the first build that can show it": this build shows the carried crate's parts, and its control (a wrong hash fails
  100 of 100; no machine filter) guards every R1 WebTransport session; it cannot show the other crate's, which move.
  R2-TRN-OQ1 stays open, as written.

## 3. The flags, ruled
| Flag | Ruling | Whose |
|---|---|---|
| F1 -58 kept, Predictor check and bite moved to CS7 | accept: R5 crosses every 16-bit wrap | plan; **U4G-Q2 (his, -58's two lost-tick checks, u4g-brief.md:249-257) returns before -58's set** (ADV-RP-8) |
| F2 -28 per part | accept (block 2) | plan (MOVED, under NPT-TRIM) |
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
- R1 4/4 h = 0.62, cap 0.78 -> 0.80; R2 11/11 = 1.72, cap 1.87 -> 1.90; R3 10/10 = 1.56, cap 1.72 -> 1.80 (8.85 h of
  blocks; 10 keeps rev 3.3's slack); R5 17/56 = 3.34, cap 18/60 = 3.55 -> 3.60. 42 x 0.1386 + 81 x 0.0176 = 7.25 (rows
  sum 7.24; rev 3.3's method); caps 8.10. R1 2.65 + 0.45 = 3.1 h plus two unsized self-tests, inside 4. R5 billed 17 by
  D3's 20:00 start (plan.md:833). Units: U6a-U6e 9,205-9,455; kept rows bar U4f2 14,565-17,315; U6j 4,575 at 1.5x, the
  largest, under 6,000; MOVED 360-720. All hold.
- R1's lane: 7,850-8,750 + U7's Rust 486-540 likely in Mon 23-Sun 29 = 1,190-1,330 a day against 1,175 measured: Wed 2
  needs a faster week with no lost day. "Planned Thu 3, aim Wed 2" is honest. Disk: 8.59 GB free (card :147-148).

## 5. The cut's safety behind his firewall; the certificate
- **Nothing an honest kept run needs is cut:** -29's UDP block stays a socket control per peer set (plan.md:1076); the
  N + 8 connection cap still bounds DelayStream's 0.31 MB a connection; the Join deadline frees a stuck own client;
  decode limits are built; strays stay INVALID; the judge's capability rule (eval.rs:598-613) holds with no session
  controls; no kept mark's check reads a cut feature (addr_cap, Origin, cycler, capability appear only in moved marks
  and in -28/-29 notes). Per-room MemoryMax goes: a leaking room dies by the kernel and -15/-27 record it. Kept for
  strangers only, but cheap: the outbound drop, the CT ledger, key-only SSH.
- **DNS-01 by hand meets both rules:** no token (plan.md:986's objection gone); a fresh ACME account per VM and the key
  never leaves RS (:974-975); TXT records "are always DNS-only" (Cloudflare, replan-units.md:93) and `rs.nt` stays DNS
  only (:943); his two-factor login is the only credential and no session sees it; nothing inbound opens.
- Does not hold: HTTP-01 "stays only as the fallback" (replan.md:62), but it is cut (replan-units.md:32) (ADV-RP-9).
  R2, R3, R5 have no browser and could skip the public order; a split R1 makes a fifth order in 7 days (-12).

## 6-7. The decisions, the order, the date
- RE-PLAN-OK hides its leans behind codes (CLAUDE.md forbids unexplained ones) and shows none of five readings of his
  words (-6, -15, -8). MARK-NOTES: -7 right, -56 incomplete (-4). LAPTOP misframed (-5). PACK-P0 right (his by
  PACK-READ (a); packed with a sha256 list and read back). Also his: R1's day (TEST-FIRST, history.md:2262); the paste
  to the game chat (an action: -1). No money beyond each run's cap; nothing on main before R1 (R-PR #41 merged).
- The order is lane-bound and holds. Fix: MOVED first on the lane after R1 (-2); U4's report Mon 23, U4k a new unit
  (-14); Stop B's read from U6c's merge, beside U7's Rust half (-16). The date stands: planned Thu 3, aim Wed 2.

## 8. Verdict: APPROVE WITH CONDITIONS
The re-plan answers NPT-TRIM, keeps every bar, and its sums hold. Conditions: apply P1-P6 below verbatim to replan.md
before it goes to him (verbatim needs no re-check; other wording of ADV-RP-1 to -9 comes back to me); the Lows ride it.

**Findings** (cite: what is wrong; fix)
- **ADV-RP-1 High** (replan.md:45-78; history.md:7576): the 29 moved marks and OQ-0001-6 reach no one off this branch;
  the game lane's build plan still says this test picks OQ-0001-6 (:389-391), and its next change-set (3) is the first
  destination and builds its own L1. A rule a session would hit; marks could drop. Fix: P3, P6 item 2.
- **ADV-RP-2 Medium** (replan.md:18, :75-77; replan-marks.md:213-214; replan-units.md:205-206): MOVED has three times
  and no slot on the lane. Fix: P2 (first after R1; R1's report after it).
- **ADV-RP-3 Medium** (replan.md:78; history.md:7590): no security seat on units that face the internet in a run (U6b
  transports, U6c page host and ACME client, U6e metrics exporter). Fix: P2.
- **ADV-RP-4 Medium** (replan.md:83; marks.json -56 notes): "count 4" holds only with items 1, 2, 4 narrowed in the
  notes (his text too). Fix: P5 MARK-NOTES.
- **ADV-RP-5 Medium** (replan.md:84-85; plan.md:188-189, :826): the laptop is ML, the typical student laptop he names for
  the whole game (OQ-0003-1), not his MSI; seven kept marks wait on it; R8 is off the screen. Fix: P1, P5 LAPTOP.
- **ADV-RP-6 Medium** (replan.md:81; CLAUDE.md, Reporting to Mustafa): codes unexplained in a decision; no "For
  Mustafa" section. Fix: P5, P6.
- **ADV-RP-7 Medium** (replan-marks.md:54-55, :79; history.md:7585): -34/-35 ("RP") and -55 name no change-set or slice.
  Fix: P4.
- **ADV-RP-8 Medium** (replan-marks.md:112-113; u4g-brief.md:249-257): F1 brings back U4G-Q2 (his) before -58's set;
  not named. Fix: P5 RE-PLAN-OK (5).
- **ADV-RP-9 Medium** (replan.md:62; replan-units.md:32): HTTP-01 "stays only as the fallback", but it is cut. Fix: P4.
- **ADV-RP-10 Low** (replan.md:64-66): add that R5's density marks (-27, PM-0006-13, -14) pay the rewind in every room,
  and that U4h's per-shot checks are not in the numbers.
- **ADV-RP-11 Low** (replan.md:70-73): npt-l1 meets change-set 3's own L1 (build plan :321-322; "not built a second
  time"): offer it to that plan or keep it here; give the lab's reason (bound to the stand-in sim: replan-units.md:187).
- **ADV-RP-12 Low** (replan-units.md:117-118): R2, R3, R5 have no browser; their run plans may skip the public order (LG
  pins the room's certificate); a split R1 makes a fifth order for `rs.nt` in 7 days (plan.md:979).
- **ADV-RP-13 Low** (replan.md:35-36): his part per run omits typing his home address into the firewall: add it.
- **ADV-RP-14 Low** (replan-units.md:205 vs :197): U4 closes with its report on Mon 23; U4k is a new unit. Fix: P2.
- **ADV-RP-15 Low** (replan.md:67-69): U7's Wasm codec is a reading of "no sim in Wasm": show it. Fix: P5 (3).
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
  history, which covers any cap the real game sets (-48's test cap, which -7 was sized from, moved). (2) -56's record
  counts 4 items, not 10: items 5-10 move with their marks; item 1 keeps only the carried crate's results on -11 and
  -28 (the other crate, -25, -35 and the pick move to Slice 3); item 2 keeps each browser's draft and its fallback
  (with RE-PLAN-OK (1)), Safari later; item 4 keeps NewReno against Cubic, while the browsers' rates at P2 to P4 move
  with the human run. Lean: yes to both.
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
