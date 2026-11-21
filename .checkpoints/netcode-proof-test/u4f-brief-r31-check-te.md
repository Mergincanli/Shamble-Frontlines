# U4f brief rev 3.1: test-engineer check, round 2 (what changed only)

Target: .checkpoints/netcode-proof-test/u4f-brief.md at 9f76a02 (rev 3.1; my script: sha256
738dc680eef994bdc3a8300109ec1bebd8c19eaa3f6f94f0aee14e74a52d90ed, 1,624 lines), against rev 3 (e1508bc). Code cites at
the head (spikes/ unchanged: `git diff --stat e1508bc 9f76a02 -- spikes` prints nothing). No Edit tool in this run:
this file is rewritten whole with Write after each block. Scripts (read-only, print only) under N/u4f-r3/r2/:
p3_jumps.py, r31_te_check.py, r31_te_sums.py. Built and ran nothing else.

## Plan
1. Read my round-1 findings (check-te-r3-full.json, TE3-1..15) and the diff e1508bc..9f76a02 of the brief.
2. Per finding: fixed in full / in part / not, with the rev 3.1 line.
3. Soundness of the two Highs' fixes (TE3-1, TE3-2).
4. New items: (a) join anchor; (b) option A's past window W; (c) 5.3 by name; (d) f36-f38, p20, p21, 2.10, 5.5.
5. Counts. 6. Anything broken. 7. Findings + StructuredOutput.

## Blocks

### B1 (done) round-1 findings and rev 3.1 read whole (change list :21-32, counts :34-42, sections 0-12, NA table)

### B2 (done) code reads at the head
- client/src/lib.rs:425-444: a command is stamped only once the slot is known; `newest.map_or(0, |n| n + lead)` (:429):
  a command given before the first snapshot is stamped 0. :458-470: an `Input` goes every step when acks or unacked
  commands exist; an ack-only `Input` encodes (proto/src/input.rs:129-139 writes count 0). :443 keeps 8 unacked.
- bots/src/lib.rs:14-21 and :220-227: no command until the endpoint holds a snapshot (the stamp would be 0), none when
  the newest did not move. Rev 3.1's controller rule (2.2 :302, :307) does not restate "none until a snapshot" and
  gives no first stamp (no "previous stamp" exists for the first command).
- room.rs:681 JoinAccepted.tick = next_tick; :653 a fresh InputBuffer per join; :841-846 `take` for every joined
  slot; :791-799 oldest first. input.rs:89 newest_seq set before the press/late/full checks; :123 `accepted` only on
  Buffered, so the kind-2 and kind-5 references stay at the last accepted command (twin rows 1,2,4,5,6 hold).
- proto/src/input.rs:155-159: the two codec rules confirmed. 0 commands encode.
- l1/src/lib.rs:146, :173: every profile carries ReorderStep::PART_13 (1 % of adjacent pairs swapped); P1 50 ms ±10
  1 %; P3 150 ±30 5 % (:182-192). f36's model names delay, jitter and loss only.
- server/src/pump.rs:20, :84: the Input bucket is 8 datagrams, refilled 2 x tick rate a second.
- marks.json -20 :4710-4767 (two shares, 10,000 presses: 48 x 15/s x 20 s = 14,400 passes short, 0 lost, 0 doubled);
  -21 :4934-5073 (forged: 1,000 per kind, 11 kinds, unrejected, twin figure :4970-4979; clamp: 1,000, never refused,
  twin figure :5004-5013; fast clock: 10 runs, cmds, move :5038-5049; lag switch: 100 bursts, excess move); -47
  :9096-9111. SERIES.md:61, :63, :155-156, :33-34.
- judge figures.rs:195-203: no twin in the call -> Lack::Gap; eval.rs:346-347 a Gap makes the run invalid;
  :1475-1478 the bite shown by any failing check.
- sim/src/world.rs:1979, :1995-2009: projectile seq, Shot and Hit seq in the hash (Q13's rejection of SA3-2 holds).
- results/u4r/0f066338dd8d/P3/runs.txt:18-20 (178,595 / 155,292 / 163,861 steps without a command; late 1,112 /
  1,136 / 1,224), results/u4r/051995db9cea/P0/runs.txt:22 (0): the brief's cites hold.

### B3 (done) TE3-1 model (p3_jumps.py; 3 seeds x 300,000 ticks; P3 as L1 has it incl. PART_13)
- Newest moves per client step: 0 in 21.2 % (U4r measured 17.7-20.3 %: the model holds), 1 in 58.8 %, 2 in 18.8 %,
  3 in 1.10 %, 4 in 0.056 %, 5 in 0.002 %.
- Under U4F-Q7 (a) (target = newest + L) and 2.2's words "at most 2 a step; a target more than 2 ahead leaves the
  stamps between unused": 11,001 unused stamps over 899,951 steps = 1.22 % of ticks with no stamped command, against
  -20's 0.1 %. Raising the lead cannot fix it, so f36 fails and the builder stops (1 item 5) under (a).
- Under a catch-up reading (owed stamps carried, 2 a step) none is unused; the largest backlog is 3.
- Under Q7 (b) (ClockSync's estimate) the target moves 1 a step but at a filter crossing (0 or 2): no unused stamp at
  steady state; a margin or reset step of 3 or more would leave stamps unused under the same words.
- On time: with contiguous stamps a command is resent every step until acked; a margin of about 2 ticks past the
  one-way delay gives about 0.05^3 = 0.0125 % late: -20's 99.9 % is reachable within the bound and option B's 9.

### B4 (done) TE3-2 per kind on a correct build, and each bite
- Kinds 1,2,4,5,6: forged newest refused under its kind; the forger stores the honest command, so later copies are
  repeats of a refused sequence and never run (2.4 item 2 :331-333); rule (:351-352) passes. Bites remove the check:
  the command runs or is refused under another kind (no-newer-tick under B: Late at input.rs:107-113), so the rule fails.
- Kind 3: copy of accepted s-1 with changed content, processed first (room.rs:791): repeat filter -> kind 3; the
  original ran; rule (:353-355) passes; no-seq-forward: the copy is not a kind-3 refusal (kind 2 or runs), rule fails.
- Kind 7: the command runs, the sim records (slot, seq, kind): rule (:356-358) passes; no-cooldown: no record, fails.
- Kind 10: under A the 9th-16th refused on arrival, stamps t-5..t+10 inside [t-W, t+12]; no-catchup-cap fails it.
- Kinds 8, 9, 11 by count: unchanged. Twins (2.12): equal command streams and sequence numbers in both runs at P0.
- Wording gaps only: kind 1's "the lead bound plus 1 to 100 ticks ahead" names no reference (newest + bound + 1 is not
  past the bound at the next tick to run at P0); kind 3 when s-1 already rides unacked in the honest datagram.

### B5 (done) the new items
- (a) Join anchor: as built the bots cannot stamp at or before the join tick (no command before a snapshot; the first
  snapshot is of a tick at or after next_tick at the join). The controller rule's wording drops that clause and names
  no first stamp; the endpoint's map_or(0) remains; lab runs start at --tick-start 63,736 (c-u4r-r2.mjs:64-65), so a
  stamp of 0 is far before any join tick. f13 (iii) tests only far-past stamps: no boundary (= join tick refused,
  join tick + 1 accepted) and no honest positive case.
- (b) W = 8 under A: resent copies are repeats (never reach W); P3 losses would need about 15 straight losses; the
  judged lag switch at P1 (200 ms, 6 commands, lead about 3) has oldest age about 4-5: none dropped; p13 unchanged.
  p19 (400 ms, 12 commands): oldest 3-4 are 9-12 ticks old at arrival and dropped `late` before the cap; and the
  bucket (8 datagrams, pump.rs:20, :84) lets at most 8 new commands reach the burst tick, so the cap refuses nothing
  and lagswitch_cmds_count's max is about 5, not 8: p19's reading fails on a correct build (and Q15 (c)'s reason).
  p20: the idled fast clock (Q11 (v) (b): previous + 1) starts about 1,800 behind and closes 0.5 a tick: every command
  of its episode is dropped late; "excess at most W" holds for any W below about 900 (fails only with no W at all).
  -20's shares and carry 74's late share: unchanged (W only drops commands already late).
- (c) 5.3 by name: each named check fails on its bitten run and passes on its clean partner (p13 the stated
  exception). But "every check but the length-failing ones above passes on both" fails on a correct build: the forged
  and clamp twin checks have no twin in 5.3 (Gap, run invalid) on p1, p2, p7, p8, and p1's new "the twin check
  passing" cannot be met; p9's bite also fails the movement check, p12's the missing check. "The same" in p2-p6 now
  carries p1's forged and twin bars.
- (d) f36 can fail as worded (good), omits PART_13 and its cost is outside 2.10's wall-time rule; f37 "as arithmetic"
  should call the lab's own schedule; f38 sound (it pins W + bound); p21 sound. 2.10's scale: f16's 2 s + 4 s pairs
  with no "at least one forged item of the kind in the window" (kind 10's episodes are about 1 per 4.8 s at the
  judged rate), f29's 2 rounds cannot meet carry 75's bound under Q4 (a) or (c); no stop on the test step's total.
  5.5 vs today's reading (5,468 MiB, 1,372 above the floor): about 1,752 above it after the u4r-P3 pack (380 net), not
  1,798; KL-U4F's "3.6 to 3.7 GB" is 3,608-3,728 MiB = 3.5-3.6 GiB; C62H-BOUND's two closed sets (P2-8ee453c22882 741,
  P3-d66e23cd78f2 298) are in neither the pack plan nor the protected list nor KL-U4F's options.

### B6 (done) counts and consistency (r31_te_check.py, r31_te_sums.py)
- 2.10: 38 rows; measured Δ sum 107, negatives 63; f30 bites only, f38 conditional: +107 / +122 / 63 hold.
- 5.3: 18 rows (p1-p14, p15-p18, p19, p20, p21): 15 base pre-runs hold, conditionals +4/+1/+1 hold.
- P24 key once (df-ruling :462), bare words twice (:313, :462): TE3-9 fixed.
- No 7-word run of his lines outside 12 item 3 (TE3-15 fixed). The session's PACK-READ (a) wording (:2212, not his
  words) recurs at brief :985, :1572, :1576 (by design, no finding).
- 12 item 4 (:1557) still says "U4F-Q1 to Q17"; 12 item 7 (:1579) and 7 item 12 (:1070) say Q18.
- 5.5's -21 disk 2,623.2 vs 49 x 53.4 + 6.2 = 2,622.8 (0.4 MiB; nit).
- Recheck rows 50, 9.1 rows 19, questions 18: hold.

### Status of TE3-1 to TE3-15
| Id | Status | rev 3.1 lines |
|---|---|---|
| TE3-1 | in part | 2.2 :302, :305-307; 1 item 5 :239-240; f5 :575; f36 :606; p5/p6 :897-898; :918; D15 :1502; Q7 (iii) :1300-1304. Sound under Q7 (b) only: the "unused" words leave 1.22 % of P3's ticks unstamped under (a) (F1) |
| TE3-2 | full | 2.4 item 2 :331-333, item 6 :348-363; 2.6 :425-437; 2.12 :651-652; f33 :603; f16 :586 |
| TE3-3 | in part | 5.3 :882-889, :893-910; D12 :1499. The "every other check passes on both" rule fails (F2) |
| TE3-4 | full | 2.6 :415-423 and the "Encodes because" column; f15 :585; f16 :586; Q11 (v) :1365-1369; row 46 :1133 |
| TE3-5 | full | f16 :586; 9.1 :1166; row 48 :1135 |
| TE3-6 | full | 2.6 :444; 2.7 :453-456; f37 :607; 5.1 :852-854; 5.3 :877-878; 5.5 :954 (f37's form: F10) |
| TE3-7 | in part | 2.10 :558-567; 2.7 :453-456; 6 item 1 :1027. No stop on the step's total; f16/f29 vacuity (F6, F10) |
| TE3-8 | full | 5.5 :959, :962-988; Q2 (a) :1216-1221; 12 item 13 :1598-1601 (figures: F8) |
| TE3-9 | full | 0.5 :189-190; 12 item 3 :1541-1543 |
| TE3-10 | full | 5.5 :964-966 |
| TE3-11 | full | 2.10 :553-556 |
| TE3-12 | full | D3 :1490 |
| TE3-13 | full | 2.4 item 2 :333-335; f12 (v) :582 |
| TE3-14 | full | 7 item 4 :1057-1058 |
| TE3-15 | full | Q8 :1314-1315 |

### Findings (round 2, what changed only)
- F1 Medium: 2.2 :302's "at most 2 a step ... leaves the stamps between unused" under Q7 (a) (1.22 % unstamped).
- F2 Medium: 5.3 :887-889 / D12 :1499 "every check but the length-failing ones passes on both"; p1's twin check.
- F3 Medium: p19 :908 and Q15 (c) :1423-1424 under W and the bucket.
- F4 Medium: join anchor first-stamp rule; f13 (iii) boundary.
- F5 Medium: 2.10's short scale: f16 pairs may hold no forged item; f29 under Q4 (a)/(c).
- F6 Low: p20 :909 cannot show W's size.
- F7 Low: "allowance after an idle" (R3 :25, B3 :999, Q8 :1317-1318, 12 item 6 :1568-1569).
- F8 Low: 5.5 :962-966, Q2 :1210, KL-U4F :1574-1575 figures and options vs today's reading.
- F9 Low: 12 item 4 :1557 "Q1 to Q17".
- F10 Low: f36 :606 (PART_13, cost), f37 :607 (arithmetic), 2.10 :565-567 (no step-total stop).
- F11 Low: 2.6 :427 (kind 1's reference), :432 (kind 3 when s-1 rides).
Verdict: PASS WITH FINDINGS (no Critical or High open; 5 Mediums, 6 Lows).
