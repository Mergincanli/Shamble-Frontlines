# U4f brief rev 3.1: security-auditor round 2 check (what changed only)

Task: round 2 on `git diff e1508bc 9f76a02 -- .checkpoints/netcode-proof-test/u4f-brief.md`. Head 9f76a02.
Read only; no builds, no scripts except read-only python3 -I under u4f-r3/r2/ (none was needed). No Edit tool here:
this file is rewritten whole with Write after each block.

## Plan
1. Read the diff (R1-R10) and the reviser's notes.
2. Item 1: SA3-1..SA3-8 each, fixed in full / in part / not, with rev 3.1 line.
3. Item 2: SA3-2 rejection (state hash carries seq?): sim/src/world.rs:1922-1925, :1979, :1995-2009; proto/src/input.rs encoder rules; twin construction.
4. Item 3: SA3-1 fix as a defence: join anchor (room.rs:681), W = CMD_CATCHUP, bank cases (idle, rejoin, replaced session, kick-rejoin, u32 wrap, signed key), honest first command before snapshot (client/src/lib.rs:429), arrival cap, LEAD-BOUND text.
5. Item 4: per-kind forging table 2.6, kind 3 construction, kind 4 widened arithmetic.
6. Item 5: packing rule vs CHECKPOINTS.md §11, PACK-READ (a) history.md:2212, R4; no-socket rule.
7. Item 6: anything broken / inconsistent.
8. Findings + coverage table; return schema.

## Blocks done

### B1 read (done)
- Head 9f76a02; brief 1,624 lines. Diff e1508bc..9f76a02 touches only the brief (stat: 389+, 172-). Reviser notes
  u4f-brief-r3-notes.md:132-249 read (account, not evidence). My r3 check-sa-r3-full.json re-read (SA3-1..SA3-8).
- Brief sections read at rev 3.1: change list :14-42; 2.1-2.6 :278-446; 2.10 :542-613; 2.12-2.13 :647-683; 4
  :715-831; 5.5 :949-988; 5.6 :990-1018; 7 :1044-1076; Q2 :1206-1228; Q4/Q6 :1240-1279; Q7-Q18 :1300-1475; 11
  :1479-1502; 12 :1506-1601.
- Code read at head: sim/src/world.rs:1922-2017 (write_state), :1325-1399 (one command = one tick of move and weapon
  clock; edge = counter differs from `claimed`), :1490, :1550-1553 (seq = c.seq / f.seq); room/src/input.rs whole
  (offer :80-125, take :129-151); room/src/room.rs:363-369 (canonical), :637-695 (join: fresh ClientSlot with
  InputBuffer::default() at :653 on EVERY join, replaced or not; JoinAccepted.tick = next_tick :681; leave sets None
  :693), :744-801 (receive_input), :805-846 (tick, fill_input); room.rs:439 (next_tick starts at sim tick + 1);
  proto/src/input.rs:1-30 (rules), :92-96 (view offset = u16 of tick - view_tick), :130-199 (encoder: TooMany,
  NotCanonical seq :155, OutOfRange step :158); proto/src/wrap.rs:43-65 (after8/16/32 signed distance);
  client/src/lib.rs:414-490 (stamp :429 newest.map_or(0,..); view clamp :436-441; Input every step :458-470);
  bots/src/lib.rs:14-24 (none until a snapshot; view_tick = newest), :30-49 (MOVES within 127^2);
  lab/src/scenario.rs:458-472 (drop_slot: replaced -> Kick, no respawn); l1/src/lib.rs:60-79, :179 (P0 zero delay,
  never the same instant); Cargo.toml:116-120 (release overflow checks only npt-sim, npt-proto).
- history.md :2125, :2127, :2187, :2212, :2048-2051, :2067-2075 read; CHECKPOINTS.md §11 :231-249; card :184-208 (R4,
  RECS-1043 "B1 first, then P3"); advisor-c62h-bound-report-head-full.json:123 (R4); c62h-bound-brief.md:485-501 (8.1).

### B2 item 1: SA3-1..SA3-8 (done)
- SA3-1 (High): FIXED IN FULL as my fix asked: anchor under both options (2.3 :316; 2.4 item 4 :343-345; kind 2 row
  :373; f13 (iii) :583); W by U4F-Q18 (a) recommended (:315, :1454-1475), (b) the written-up exposure; f38 :608 and
  p20 :909 conditional; Q8 :1317, B3 :999 and LEAD-BOUND :1566-1569 carry "bound plus W after an idle"; 7 item 2
  :1050-1051. As a defence it has new gaps (B4 below): the rejoin, the honest stamp 0, W's arithmetic.
- SA3-2 (Medium): FIXED IN PART, rest REJECTED RIGHTLY. (i) kind 1 stored copy: endpoint keeps honest (2.6 :427);
  (ii) kind 3 added copy of an accepted seq (:432, 2.4 item 6 :353-355, 2.12 :652); (iii) twin keeps the skip, later
  Inputs leave out older (2.12 :651, Q13 (b) :1390-1398); (iv) later copy of a refused seq = repeat (2.4 item 2
  :332-333); f15 :585, f16 :586 assert encoding. My twin variant was wrong (see B3).
- SA3-3 (Low): FIXED: 2.4 kind 4 row :374; f10 :580.
- SA3-4 (Low): FIXED: 2.4 item 2 :333-335; f12 (v) :582.
- SA3-5 (Medium): FIXED: 5.5 items 1-3 :968-981, never-list :984-986; Q2 :1212-1227 (d); 12 item 6 KL-U4F
  :1571-1577; item 13 :1598-1601.
- SA3-6 (Low): FIXED all four: (1) section 4 :830-831; (2) D3 :1490; (3) Q4 (a) :1248; (4) Q6 (b) :1274-1275 and
  2.10 :546-548 (+4).
- SA3-7 (Low): FIXED: 2.6 fast-clock row :444 (ping_limited above 0 expected, reported).
- SA3-8 (Low): FIXED: 2.3 :318; 2.4 item 3 :340-342; kind 10 row :377; f7 :577.

### B3 item 2: the rejected twin variant (done)
- world.rs:1922-1925 leaves out only the press counters; :1979 hashes each projectile's seq; :1995-2009 each Shot
  {slot, seq} and Hit {shooter, seq}; seq = the command's (:1490 `seq: c.seq`, :1550-1553 `seq: f.seq`). A twin
  that renumbers to stay consecutive shifts every later shot's seq by one: hashes part at the first shot. My SA3-2
  claim cited only :1922-1925 and was wrong. **The rejection is right.**
- The kept construction (skip index and stamp, send nothing; later Inputs leave out any command older than a skip):
  encodes: each Input's commands consecutive (input.rs:155), the newest whole, older ones 0-255 ticks below (stamps
  rise at most 2 a step), view offsets as the honest ones. Room side: twin room sees n-1 then n+1 (after16 true,
  input.rs:82); forged room refuses n after setting newest_seq (:89); `accepted` (press reference, kind 2
  reference) is n-1 in both; covered ticks for n+1 the same; option B repeats n-1 at s_n in both, option A runs
  nothing at s_n in both; the sim's `claimed` sees n-1 then n+1 in both. Forger's later copy of honest n is a
  repeat of a refused seq (counted, no state). Hashes equal at P0, given twin_left_out = 0 (f16 asserts it).
  Snapshot headers differ (ack n vs n-1) and datagram sizes differ: neither is hashed; f16 leaves sizes out.

### B4 item 3: the anchor and W as a defence (done)
- Idle under (a): stamps run in an episode a..b lie in [a - W, b + B] and rise strictly, so at most E + W + B: the
  excess over "ticks + bound" is at most W; refilling needs an idle of about W + B ticks with nothing run. No net
  gain. LEAD-BOUND's "(up to 8 more again)" is right for this.
- u32 wrap: after32 is signed distance (wrap.rs:60-65); a stamp at d <= 0 from the reference is kind 2, d < -W from
  t is late (if W is written as a signed distance). Accepted stamps all lie in [t - W, t + B]: the signed-distance key
  is total for them.
- **Rejoin / replaced / kicked-then-rejoined (GAP, finding SA31-1):** `Room::join` builds a fresh ClientSlot with
  `InputBuffer::default()` on every join (room.rs:648-653, :685), replaced or not; the soldier is slot-indexed in the
  sim and persists (no respawn in lab drop_slot, scenario.rs:458-472; spawns only by script). So each join re-anchors
  at the new JoinAccepted tick J' (room.rs:681) while the old session may already have stamped up to t + B. The new
  session sends J'+1 .. t+B at once, 8 run a tick: about B = 12 extra commands (12 ticks of movement, weapon clock,
  reload: world.rs:1333-1345, :1389-1399) per rejoin. Rejoin every 30 ticks: about 42 commands per 30 ticks, 1.4x.
  Option B: one per tick, no gain. -21 has no rejoining cheat; -47 rejoins honestly about 21 times per slot.
- **Honest command before the first snapshot (GAP, finding SA31-2):** client/src/lib.rs:429 stamps 0 when no snapshot
  is held; J >= 1 always (next_tick = sim tick + 1, room.rs:439); after32(J, 0) is false, so kind 2 (forged) under
  both options. Rev 3: late under B (input.rs:107-113), and ran under A. Bots avoid it today (bots/src/lib.rs:14-16),
  but rev 3.1's controller rule (2.2 :307) replaces the "when" rule with "the commands the endpoint says are due" and
  restates no snapshot floor; Q7 (b) (recommended :1303) can make a command due from a Pong alone; a page gives one
  every frame.
- **W's arithmetic (finding SA31-3):** 2.3 :315 and Q18 :1467-1469 say "stamped more than W ticks before the tick it
  would run at" with no form; next_tick starts at 1, so a plain `t - W` underflows for t < 8: debug and test builds of
  npt-room panic on the first command (network input), release (overflow checks off: Cargo.toml:116-120) wraps and
  drops every honest command of ticks 1-7 as late. Same class for kind 1's `t + bound` (only near u32::MAX).
- Arrival cap: holds whatever the pump passes: each new command after 8 held for the coming tick is refused on
  arrival (2.3 :318), nothing carries over a tick under A; list <= 8. Refusal records per tick stay bounded only by
  the pump's bucket (unchanged since rev 3; not a finding).
- LEAD-BOUND (12 item 6 :1566-1569): accurate for the idle; "at most 12 more commands ... per 60 s" holds per
  session only while SA31-1 is open.

### B5 items 4-6 (done)
- Kind 4 widening (:374): i32 before squaring, no abs: max 2 x 128^2 = 32,768, no overflow. f10 cases. OK.
- Kinds 2, 4, 5, 6: each forged newest command breaks only its check, in the 1, 2, 4, 5, 6 order; encode. OK.
- Kind 3: forged copy of n-1 with changed move or aim, tick unchanged; repeat filter finds accepted n-1 with other
  content -> kind 3; n accepted. Encodes when the honest datagram holds n alone. If the honest datagram already holds
  n-1 (unacked), "plus one older copy" duplicates n-1: NotCanonical (input.rs:155). (SA31-4 (b))
- Kind 1: "the lead bound plus 1 to 100 ticks ahead" names no anchor. At P0 the forger's newest is T and the room's
  tick at arrival is T+1 or T+2 (never the same instant: l1 :69-70); newest + B + 1 or + 2 is within the bound, so it
  is accepted and runs, and the forger's next honest stamps at or below it become kind 2. (SA31-4 (a))
- Kind 10 under A: stamps t-5 .. t+2 with view_tick = newest = t-1 (bots :22-24): t-5 .. t-2 give tick - view_tick
  negative, u16::try_from fails, OutOfRange (input.rs:92-95), unless clamped as client/src/lib.rs:436-441 does at
  stamping. The table's "Encodes because" misses it; the twin's first datagram the same. (SA31-4 (c))
- No new kick path (refusals never kick; kicks stay event-cap and replace). Kept window K = 16, arrival list 8,
  anchor one u32 per slot: bounded. Kind 3 compare bounded by K per command.
- Packing (item 5): 5.5 item 1 packs only u4r-P3-0f066338dd8d, by 8.1's procedure (c62hb-brief :494-499; ruling
  advisor-u4r-sets-r2-full.md:45), as PACK-READ (a)'s second folder (history :2212; B1 first: card :204-205) under R4
  (report-head json :123): covered. Item 3: a U4f set's folder only after KL-U4F, "no ruling may widen it alone":
  right. Never-list :984-986 names both P0 folders, fc411116c1f1, fd4612050cd6 and the c-u2 folders = R4's list.
  §11 :236-238 (never during a build or set; main session) kept in item 1 and :987. KL-U4F's text is fair to
  PACK-READ ("one or two older folders"). The other folders on disk now (u4r-P2-8ee453c22882, u4r-P3-d66e23cd78f2)
  are not packed by the brief. No-socket rule at section 4 :830-831. No finding.
- Item 6: Q17's rev 3.1 sentence (:1450-1452) gives the bite run's triple T+1, T+2^30, T+2^31+5; with W as a signed
  distance T+2^31+5 is late (d = -2^31+5 < -8), so that example is no longer reachable; a reachable one in the
  no-lead-bound bite run under A is t-5, t+2^30, t+2^31-2 in one tick (each newer than the last, none late), whose
  pairwise wrap-aware order cycles. (a) stays right; the example is wrong. Folded into SA31-3.
- Counts: 102 + f12 (v) + f13 (iii) + f36 (2) + f37 = 107; neg 61 + 2 = 63. Consistent.

## Findings (final)
- SA31-1 Medium: rejoin re-anchors option A's bank (B4).
- SA31-2 Medium: honest stamp 0 before the first snapshot is kind 2 under the anchor (B4).
- SA31-3 Low: W's and kind 1's tick arithmetic unstated; Q17's example (B4, B5).
- SA31-4 Low: 2.6 kind 1 anchor, kind 3 duplicate copy, kind 10 view offset (B5).
Verdict: PASS WITH FINDINGS (both rev 3 Highs of mine fixed; no new High).
