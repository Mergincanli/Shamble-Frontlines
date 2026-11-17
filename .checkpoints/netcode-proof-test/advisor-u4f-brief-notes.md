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
