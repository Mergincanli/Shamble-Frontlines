# Advisor: U4f builder brief rev 3.2 check (notes)

Started 2026-11-17 (after D2 18:54 UTC). Effort xhigh. Instructions read from origin/main:.claude/agents/advisor.md.
No cut run found (no notes file existed).

## Plan
1. Read the brief rev 3.2 whole (u4f-brief.md, 1,734 lines): sections 0-12, change lists (R11 block, R1-R10, C1-C28),
   questions U4F-Q1..Q18 (+ any added), D-items, section 12.
2. Read the plan's §5 U4f (u4-build-plan.md:805-816) and U4-BRIEFS (:1054-1056), plus the parts the brief cites.
3. Read u4s-carries.md "## U4f (clock and commands)" and the carries named.
4. Read my earlier brief checks (advisor-u4r-brief.md, advisor-u4e-brief.md) for form; my rulings that bear on U4f
   (grep advisor-*.md / -full.json for U4f, ClockSync, lead bound, B3, KL-U4F, KEPT-LOGS, SIZE-CHECK, command forms).
5. Read the round 1 and round 2 findings (TE3, SA3, TE31, SA31) and the reviser's answers (r3-notes, "Revision for
   round 2"); check each answer in the brief text.
6. Read D4 outputs (d4-output.md, d4b-notes.md, d4b-r31-full.json, d4-compare.md) for Q6.
7. Check the code at the head (spikes/netcode-proof) for the cites the brief leans on (ClockSync, Pong, commands,
   lead bound, controller).
8. Run the read-only anchor scripts (stamps_anchors.py, r31_check.py) for his words' anchors.
9. Rule on Q1-Q18 (+added); bars for Mustafa (5.6 table); build rules; D-items; missing items.
10. Write ruling file advisor-u4f-brief.md; hand back.

## Blocks
- B1 done: brief rev 3.2 read whole (1,734 lines; sections 0-12, NA table). Counts at 3.2: +109/+124, 64 neg,
  18 questions, 54 recheck rows, planned size 4,850, disk 3,607-3,727 MiB vs 1,372 above floor. Next: plan §5 U4f.
- (resumed after a cut, same day) B1 re-read into context whole (1,735 lines incl. NA table). Continuing at B2.
- B2 done: plan §5 U4f :805-816 read (Builds; Marks moved: -47 1,000 connects P2 2 bites; -20 P3 both options bite
  redundancy-1; -21 8 parts + twins + 13 bites; -22 one never-acker 10 min, no pre, load stated, bite unbounded S30;
  reviewers netcode, security, test, rust); U4-BRIEFS :1054-1056; §9 run count (-20 6, -21 24+12, -22 3, -47 3);
  §7.1/7.2. Brief 5.1 matches (36+12+17=65). Carries §U4f :507-539 (46, 80 (a)-(e), 86) read. Form of my brief checks
  read (advisor-u4r-brief.md). Next: B3 round 1/2 findings + reviser's answers.
- B3 done: round 1 (TE3-1..15 FAIL; SA3-1..8 FAIL; Highs TE3-1, TE3-2, SA3-1) and round 2 (TE31-1..11: 5 M, 6 L;
  SA31-1..4: 2 M, 2 L) read from the JSONs; reviser's "Revision for round 2" read (each DONE; TE31-1's "f36 under both
  while Q7 open" not taken: open Q stops the builder; TE31-11's anchor taken as SA31-4's). Slip in form: one
  `python3 -I -c` print of the JSONs (read only, inline, not a Write-made script); from here on scripts by Write under
  u4f-r3/. Next: B4 D4 files; B5 code cites.
- B4 done: d4-compare.md read (99,000 connects; every verdict agrees; exact-stamp mean 2.346, its bite 11.927;
  exact lowest-RTT 4.856 sd3k 0.100; as built mean 5.975/8.867; (e) 12.781). ADR 0001:239-251 read: "from its own
  ping samples", no pick named; Pong fields not fixed by the ADR; catalogue :91-92 (Pong 16 B doubled to 32 B; status
  Proposed, spike design doc, not an ADR) -> Q6 (b) is a spike wire change inside the plan, the advisor's call, the
  catalogue updated first (.claude/rules/netcode.md). marks.json -47 :9095-9111 (count >= 1000; p99 <= 5,000,000 ns).
  pump.rs: ping bucket cap 10 refill 10/s (D4's 10/s honest pings never limited); INPUT_BUCKET 8 refill 2xTICK_HZ.
  Next: B5 code cites.
- B5 done (code at the head): client lib.rs:414-490 (stamp newest.map_or(0)+lead :429; view clamp :436-441; unacked
  truncate 8 :442-443; steps on the host grid :485-488; JoinAccepted keeps only slot :496; client sends no Ping today);
  input.rs whole (offer order: seq :81-89, press :90-106, late :107-113 incl. equal-to-buffered :110, Full :114);
  room.rs:637-695 (fresh InputBuffer :653, JoinAccepted.tick = next_tick :681, leave None :689-695), :744-801.
  Soldiers collide with static boxes, doors, wall, vehicle parts (world.rs:1230-1264), not each other; respawn is a
  teleport to a free team point (world.rs:1734-1742; respawn 1,000 ms; move 600 cm/s = 20 cm/tick, scene.rs:184-186).
  marks.json -21 :4805-5145 read: lag switch "no tick moves further than the catch-up cap" max <= 0 of
  tick.v.lagswitch_excess_move_um; fast clock movement max <= 0 per episode. **New finding (mine): neither the brief nor
  SERIES.md:34/:156 says what "movement" is on a respawn tick; a respawn of the lag-switch or fast client (47 brawlers
  fire at it) moves it metres in one tick vs 8 x 20 = 160 cm -> the check fails a correct build; p3/p4 would catch it
  before the sets (Medium).** ADR 0001:1611 "moves no further in any tick": a reading. Next: B6 anchors/scripts.
- B6 done: stamps_anchors.py run (read only): every stamp equals 0.2's; P1-P36 each found once (P24 by its key at
  df-ruling :462). r31_check.py: brief e9f33e61… 1,734 lines; 7-word runs outside 12 item 3: 0. My own
  u4f-r3/adv_words_check.py (8-word runs against ALL of history.md): 2 hits, neither his words: :414 (a path) and
  :2212 (PACK-READ's option text, the session's, reused in the KL-U4F draft). Check 2 holds.
- B7 done (rulings since rev 2): merge-stop C3 (U3C-HELD-1 before U4f's merge: closed), its pooled taker for the
  deferred Lows (RR-CB-2 taken; judge ones left), head-check ADV-CBH-1/2, H2 (5.4), report-head R1 (U4g), R4 (8.1's
  list), c62h :20 (P31), c62h-review :82-93 (5.6), U3C-HELD-1 plan review :37/:89 (pack before U4f's counted run),
  r-b2 :26-28 (P33), r-check U4R-Q2 (c)/Q4/Q14: applied as ruled.
- B8 done (more code): **Pong literals at server/tests/pump.rs:276, :310 and proto/tests/common/mod.rs:366; the room's
  Pong is built in pump.rs:458-467: so Q6 (b) opens npt-server src AND tests (P33's trigger "manifest or tests") -> the
  brief's lane and section 6 item 4 miss it (Medium).** Decoder::pump holds now_ns and tick, not the tick's due instant:
  a server-ns field needs no signature change; a us-since-due field does. lab io(now) passes tick = last+1 (:475-480).
  **cmd_on_time (SERIES.md:61 "one per command due this tick") is undefined for a command never arrived; per-arrived
  counting makes p12's named check (redundancy-1 at P3) pass, against the bite's "fails the 99.9 %" (marks.json:4795)
  (Medium).** 60 Hz pings every 3 steps = 20/s > 10/s limit unless the interval is ns or pinned (Low). Order: 5.3's
  pre-runs build in the shared target via main-run.sh (c-u4r-r2.mjs:25-26), so 5.5 item 2's part-clean goes after them.
- B9 (rulings, drafted): Q1 (a)+(d)+Q6(b)'s server files; Q2 (a), decide after item 2; Q3 (c); Q4 (b); Q5 (a);
  Q6 (b) averaging, f1 4.0 ms with loss; Q7 (b)+(i)+(c); Q8 (b) 400 ms, (c), (e); Q9 (a) K=16; Q10 (a); Q11 (i)(a)
  (ii)(a) (iii)(c) with A1 (iv)(a) (v)(b); Q12 (b); Q13 (b); Q14 (b); Q15 (c); Q16 (b); Q17 (a); Q18 (a)+(d).
  Counts after: measured >= +118, bites >= +133, neg >= 67; pre-runs 21; disk 3,727. To him: LEAD-BOUND only (KL-U4F
  only if short after item 2). Verdict: APPROVE WITH CONDITIONS. Next: write the ruling file.
- B10: fold anchors counted by u4f-r3/adv_phrases.py (read only): 21 of 22 found once; "after U4f's merge and before
  its first set," is bold in the brief ("2. **After U4f's merge and before its first set,**"), cited that way. HEAD
  b40ba07; `git diff --stat 0ad9661 HEAD -- spikes/netcode-proof` prints nothing. Writing advisor-u4f-brief.md now.
- B11 done: advisor-u4f-brief.md written (APPROVE WITH CONDITIONS; A1-A3 Medium, A4-A7 Low; A7 found late: three
  netcode.md cites off by its 6-line front matter, :15 -> :17, :13-14 -> :14-15). Q11 (iv)'s disk corrected to about
  60 MiB (5.5's 195 -> 255). u4f-r3/adv_words_ruling.py (read only) on the ruling: 5 history hits, all the session's
  own text (seat and launch lines :1498, :1921, :1979, :2032; PACK-READ's option :2212), none his words. DONE: hand back.
