# U4r brief rev 3: the netcode-architect's check

Started 2026-11-13. Brief: .checkpoints/netcode-proof-test/u4r-brief.md (read only). Branch head 83c7801. No Edit tool
in this seat: the notes are rewritten whole with Write after each block. Read-only commands only (git reads and greps,
sed/grep reads, print-only `python3 -I -` scripts from stdin; nothing written but this file).

## Plan
- B0: read the brief whole (structure, sections, placeholders, recheck rows, questions, D-items).
- B1: (1) u4-build-plan.md §5 U4r as it stands: every test, negative case, bite, judged run, carry; map to the brief.
- B2: (2) placeholders: anchors found once, first and last words (python3 -I print-only).
- B3: (3) rulings since rev 2 naming U4r (advisor-*.md minus *-brief.md, plus advisor-u4e-brief.md / advisor-u4d2-brief.md as named): applied as ruled; open questions answered.
- B4: (4) >= 10 recheck rows vs code at head (R2, R4, R15, R33, R34 + U4e room, pump, client rows).
- B5: (5) O1 (carry 40), -30 pre-check, RTT rule, -18's heal record vs U4e's merged room/server/client.
- B6: (7) build rules: runner u4r-run.sh, W1-TARGET item 3 as amended, lessons, TE2-5, B6, A13.
- B7: (6) missing / untestable / contradicting / scope creep; findings ranked.

## Blocks
- B0 done: brief read whole (1,412 lines; sections 0-12; C1-C26; R1-R44; U4R-Q1..Q13 open set; D1-D12). Stamps at
  head 83c7801: plan 01ede63d, plan.md 3ca4e12a, carries 8b7f165e, history 88bc195f, u4r-run.sh 349b51ad: all equal
  the brief's section 0. All 13 ruling files' sha256 equal section 0's stamps. (The brief says head fa8e335;
  checkpoint commits since touch .checkpoints/ only.)
- B1 done (plan §5 U4r :750-803 vs brief): every Builds item, test, judged run, bite and Evidence line has a home in
  the brief (2.1-2.4, 7.1-7.4); 7.3's arithmetic re-done (15,600 s, 468,000 ticks; the R table) and holds.
- B2 done (placeholders, print-only): P1 carries :255-432, heading once, entries exactly the 17; P2 history :292-293
  once, line before :291 as asserted; P3 df :462-464 (start phrase also at :313: only a range-scoped search finds it
  once; u4d2-brief used the same form); P4 df :465-468 once; P5 j-r2 :168-171; P8 j-r2 :298; P9 m9 :252-256; P10 m9
  :259-264; P11 d2-r2 :304-308; P12 d2-r1 :308-310; P13-P15 carries :72-75, :116-119, :135-136: each once, first and
  last words as named.
- B3 done: every ruling naming U4r is applied as ruled (list in the summary below); no open U4R-Q is answered by a
  ruling; Q2 (ii)/(iii), Q3, Q5 answered and moved to the answered table.
- B4 done: rows R2, R4, R13, R15, R16, R17, R18, R19, R30, R33, R34, R36, R37, R38, R43, R44 and the check.mjs cites
  hold at the head (file:line in the findings and the summary).
- B5 done: O1/carry 40 statement holds against rewind.rs:1-33, room.rs:133-135, :846, :860-864 and D10's grep; the
  RTT rule's premises hold (pump reads no RTT; rtt_ns None before a sample; at P0 the server end's sample from
  JoinAccepted's packet is applied by the next net call at or after its send, l1 :947-959, :1418-1465, and the client
  sees JoinAccepted only at a later instant, poll's gate_ns < now :1667-1681), so D12 holds; -18's FullState rule and
  carry 58's route hold; the late-command bite now counts late (input.rs:84-89 then :111-117). New: F-k, F-l, F-o.
- B6 done: u4r-run.sh = u4d2-run.sh but the lane's names (diff read: header :2-3, :9, usage :16, log case :41-42);
  CACHEDIR.TAG check :46-49 before mkdir :50; exec 9> and flock with exit 5 :52-53; 4 GB check :68-71; touch loop
  :72-78. W1-TARGET item 3 as amended (P9), m9's two lessons (P10), Block 6's lesson (P11), TE2-5 (P5 + P12), B6
  (P8), A13 (2.2, lane), T4 items 1-2 (P3, P4), USAGE-SAVINGS (P2) all carried. New: F-d, F-e, F-h, F-i.
- B7 done: no unmarked scope creep (r6, r7, 7.2's added field, 8.4's switch and the two extra review seats are each
  marked as the brief's own for the advisor). Findings below, ranked.

## Findings (ranked)
1. F-j HIGH - carry 74's lead is one-way; the stamp needs the round trip (client/src/lib.rs:390-392, :336;
   input.rs:111-117). Model: P2 lead 3 -> 97.7 % late, 4 -> 2.3 %, 5 -> 0; P3 lead 4 -> 98.2 %, 5 -> 55.8 %, 6 -> 0.
   Fix: new U4R-Q14 to the advisor before launch; proposed lead ceil((rtt + jitter)/tick) + 1 = P0 1, P2/B1 5, P3 7.
2. F-a MEDIUM - the fourth recording's scope in the determinism step, and what regenerating a runner entry means.
3. F-k MEDIUM - -18's heal "decode" must be a non-zero hash (client :196, :711-712).
4. F-l MEDIUM - the brawl's view time is unstated; a fraction > 0 at lead 1 makes every firing command refused
   (rewind.rs:8-11; room.rs:805-819), so the pre-check is VOID by carry 40 on both legs.
5. F-m MEDIUM - U4R-Q6 (a) needs `Decoder::with_bite` and a BITES change in schedule.rs, not "nothing else".
6. Lows: F-n (Decoder::accept/reject and U4e's lab glue as the model), F-o (late bite within REDUNDANCY_CAP), F-p
   (Loopback leg's instants), F-q (recorded scripts into both rooms), F-r (commands per step, not per frame), F-s
   (warm-up and after phase), F-g (r1's bite selection and allow), F-b (carry 49's refusal untested), F-d (stale P3
   clause), F-e (P4 replacement and npt-server), F-f (df R6 slip line), F-h (judged runs' target), F-i (TE-M3 and
   section 5's two runs).

Verdict: REVISE (schema: FAIL) - one High and four Mediums, each fixable in the planned one revision; nothing needs
Mustafa.
