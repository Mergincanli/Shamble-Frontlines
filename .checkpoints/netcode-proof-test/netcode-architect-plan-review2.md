Task: netcode-architect round-2 review of the netcode proof test plan (revision 2), .checkpoints/netcode-proof-test/plan.md

## Plan (blocks, in order)
B1. Read review1-findings.md (what round 1 raised, esp. netcode-architect's findings) and plan 0.1 map.
B2. Read my digest (netcode-architect-digest.md) for each mark's setup + order constraints.
B3. Read plan sections 4 (crates: 4.1, 4.4, 4.7), L1, 10.2, 10.3, R-25 parts.
B4. Read plan sections 6.2, 7.2 (run mapping, hours) and check against digest.
B5. Read plan 8.1 (ports, OQ-0006-16) and infra digest on ports.
B6. Read plan readings R-5..R-26 and section 11 item 3.
B7. Check ADR 0001 lines: 220-275, 419-473, 654-682, 1444-1445, 1631-1681; ADR 0004 part 5 (S17); find R2-XU-OQ7 rule.
B8. Write findings, verdict, picks.

## Notes
- B1 done: round 1 had no netcode-architect review (security-auditor F1-F19, engine-architect EA-1..14, verifier F1-F16). Relevant to my lens: EA-1/2/3 (room/server split, Link, lab), verifier F1 (-14 sessions), F4 (-36 order), F7 (ports), F8 (R-5 per stream), F9 (readings that change marks), F10 (R-6 L2 rate), SA F13 (WS delay in accept path). Next B2 (digest).
- B2 done: digest read (A marks table, H2 reorder, H8 L2 delay, H16 -59 timing, H17 order, H18 P3 3 copies, I hours ~27 RS h). Next B3 plan sections 4.x.
- B3 partial (4.1-4.9 read). Candidate findings so far:
  * C1 Link poll_recv returns (bytes, arrival) with no channel: receiver cannot tell datagram vs control stream vs FullState stream (plan 4.7 :376-377). No max_datagram_size on Link (needed by E4 cap / -36). Check ADR 0001:419-473.
  * C2 L1 step order (plan :396-397): no per-client local clock (offset, rate) for -47 random offsets and -21 1.5x clock; zero-delay (P0) packet sent in an instant: same instant or next undefined; tick schedule 1e9/30 ns not integer (rounding rule).
  * C3 4.4 table (plan :332): missing join-token expiry/binding check placement, view-time tolerance (RTT input from server as data), link timeout; check vs ADR 0001:220-275.
  * C4 L1 reliable-channel model (retransmit/HOL under loss) unstated? check plan/test digest.
- B4 partial (5, 6, 7.1-7.3 read). Candidates:
  * C5 -3 probe "at the start of every L2 block" (6.2 :471) but 7.2 hours list a probe only in R1 and R3; R2 (S30 at P2 on L2), R4, R5/R6 (if impaired), R7 (ML* over L2) have none.
  * C6 -7 history <= 1 MB (cap + margin) measured in R2, before the rule cap exists (R3): plan never says which cap R2 runs at; must be the test cap or stated + recorded. Check ADR -7 row.
  * C7 -49 rule cap computed mid-R3 (unattended): who computes it (npt-judge runs in container per 5)? Needs a named script on RS or the laptop session, recorded.
  * C8 -36: R1 0.5 h for 3 browsers x 2 paths x 10 min = 1.0 h sequential; and which cap R2 runs at (1,000 B until all four browsers measured vs laptop-measured min(.,1150)).
  * C9 clean-run reference for -24 (R3) vs -25 (R4 has its own): check ADR 1495-1496 same-session rule.
  * C10 ports: page host on 31000 unreachable from a TCP-443-only network, so -29's reported 443-only attempt fails at page load; check plan.
- B5 done (8.1 ports, infra digest :144 reason: 32 slots for PM-0006-14 N=18 search, both ranges below Linux ephemeral 32768). Port view: agree with numbers; notes: page host on 31000 vs -29's reported 443-only attempt (page cannot load there; record it as "page unreachable", or say the attempt measures nothing); search reaching 32 rooms -> record ">= 32", stop; ADR 0006 metrics label "room slot 0-7" vs 32 slots (infra's).
- B3 cont: 10.2, 10.3 read. Next: ADR 0001 lines.
- B7 partial: ADR 0001:220-275, 419-473, 654-682 read. Candidates:
  * C11 catalogue (10.2 :741-742) gives channel, authority, size limit but drops the visibility class column ADR 0001:655 requires ("one with none fails the audit test"); RoomState (0001:669) missing from npt-proto list (4.7 :382-383) with no stated omission.
  * C12 per-session controls "acting only on the sender's own session" (10.2 :736) cannot work for BlockUdp (must act before Join, on the next connection, at QUIC level) nor for SetProfile across a reload (checklist step 13 WS at P2; WS delay applied in accept path before Join). L2's keying (per address / per 4-tuple / per slot) and the default profile for a new connection are unstated.
  * C13 "sent by the page on its session's control channel" (10.2 :734) vs E7 rule "the page never touches a packet": page -> Worker -> control stream.
  * C14 Join check (0001:664: signature, audience, expiry, unique id) vs 10.3: test token semantics for rejoin after Worker trap (-19) unstated: single-use vs reusable per slot; second Join with a held token.
  * C15 (likely High) DelayStream (E15 :298, U3 :850, R-25 :975) states delay only; R5-TEST-REC4 (test digest :69) says WS proxy gives "delay and stalls" ("loss on TCP shows as delay"). Plan never says how P2's 2 % loss per direction appears on WS (HOL stall per lost segment), that delivery stays in order under jitter, or that the wrapper's buffer is bounded so the server's newest-only drop (0001:424) sees backpressure. Without it -31 (WS stalls) cannot fail on loss and checklist step 13 is not at P2.
- B7 cont: ADR 0001:1436-1506 read (profiles, S30, V-S, clean run = median of 3 clean runs within 2.5 %, bites). Next: plan 11, 12, 13, 15.
- B6 done (plan 11-16 read). Candidates:
  * C16 (High) R-17 drop-tail queue of "one snapshot interval" (:968): at 128 kbps (-23) = 533 B, at -24's 50 % of ~146 kbps = ~304 B: smaller than one 1,000 B datagram, so drop-tail drops every full datagram whatever the server does; -23 can fail on the queue model and -24's age bound cannot test the delay feedback 0001:439-441 relies on. Change: queue in time >= 4 intervals (or a home-router depth like 200 ms) and never under 4 x 1,500 B; record it.
  * C17 (Medium) R-5 swap is "an explicit L1 reorder step" (:957) but -20 also has an L2 LG leg at P3 whose input reorder share is ~0, so its reordering clause cannot count there; and "input datagrams per direction" muddled: -58's wraps also need snapshot-direction reorder. Change: the swap per flow in both directions (inputs up, snapshot datagrams down), in ImpairSocket too for -20's L2 leg, or judge the reorder clause on L1 only (R-PR).
  * C18 (Low) R-12 "-14", "-26" after PM-0006-11 are PM-0006-14/-26 (not PM-0001): write ids in full; a bite must FAIL its mark ("must come out INVALID" is a validity check, not a bite).
  * C19 (Medium) R-6 L2 probe one-way delay needs LG-RS clock sync (2.5 ms = 5 % of 50 ms) or measuring inside ImpairSocket on RS's one clock; unstated.
  * R-word list drafted in my head: R-5 yes R-PR; R-6 yes R-PR; R-9 read only; R-10 R-PR (adds record item); R-11 read only; R-12 R-PR; R-15 ?; R-16 R-PR; R-17 R-PR for queue only; R-20 ?; R-25 R-PR wording; R-26 R-PR (ADR 0006 text) with one-host caveat.
- Next: ADR 0001 checklist 1631-1681, OQ-0001-9, rows -7/-24/-27, R2-XU-OQ7; ADR 0004 part 5 + PM-0004-67.
- B7 cont: checklist 1631-1681 read. Candidates:
  * C20 (Medium) ADR 0001:1640-1641 "the profile and the cap change from the overlay"; plan 10.2 :729-731 moves the cap to the local socket only. No checklist step changes the cap (7, 10 use the rule cap), so fine, but it is a setup change with no reading: add one (cap set over the socket before step 1, shown read-only) -> R-PR.
  * C21 (Medium) -49's rule cap comes from L1 + LG bots, which have no frame-to-tick wait (a part 0001 lists) and a DC-quiet path; Brave's honest rewind at P2 can exceed it by up to a frame + one interval of extra delay, so steps 7 and 10 could clip honest shots. Change: LG TC shooter runs a 60 Hz frame loop stamping view time per frame; R7 compares ML* Chrome honest-rewind p99 with the rule cap before the checklist, and stops to the advisor if above.
  * C22 (Low) 11 item 3 omits the base-RTT measurement at the start (0001:1641-1642) and the named counters (corrections, snaps, extrapolated frames, clipped shots).
  * C9 confirmed (Medium/High): ADR 0001:1495-1496 clean reference = median of 3 clean runs within 2.5 %; -24 (0001:1537, R3) has no clean runs in R3 (plan grep: only R4 :560 "a clean reference", one run); comparing with R2's S30s crosses a VM re-creation (host/CPU may differ) at a 5 % tolerance. Change: 3 clean 10-min runs at the start of R3 and of R4 (+~0.6 h, +0.4 h).
  * R-20 found: R2-XU-OQ7 = "answered: the history keeps the cap plus a margin, at most 1 MB; load sized at 700 shots/s" (0001:1888, 1821) -> its rule is PM-0001-7's 1 MB at 700 shots/s; it is in -7. Ties to C6: R2's -7 must run the history at the test cap + margin (the largest cap used), recorded.
  * C23 (Medium) R-10 cover: OQ-0001-9 (0001:1159-1164) options "per item, present state or rewound"; R-10 records one share under no named option and no rule; E1 has no destructible-wall stand-in. Change: run both options per item (wall, moving vehicle), record hit-after-cover share AND shooter-side unfair-miss share for each; decision rule: recorded, netcode-architect + Mustafa pick (or a stated rule); add the wall to E1.
- B7 cont: ADR 0004 part 5 (207-228) + PM-0004-67 (1193) read. R-15 word: V-S yes; read jitter over the first 36,000 measured ticks of each S30 (matches "over 36,000 ticks" exactly); stall runs split = setup change -> R-PR (ADR 0004 text).
  * C24 (Medium) 4.4 table omits many ADR checks: sequence forward/wrap, move length <= 1, press edge per tick, aim in map bounds, cooldown/ammo edges (sim), ack is a sent snapshot, trailing bytes/unknown fields, per-connection rate limits, new-session rate limit, Resync one-in-flight, Ping rate + hold-time cap, ClientSettings clamp (1/s), stream/conn windows + 2 streams, Join token/version/content-hash, view-time tolerance with server RTT passed in as data (0001:231-239, 441-452, 661-675).
  * C2 extended (Medium): L1 step order has no frame loop, yet L1 legs of -42 (per frame), -45 (% of frames), -61 (first frame), -62 judge per frame; no per-client clock model (offset, rate) for -47/-21; tick schedule must be base + k*1e9 div TICK_HZ (no accumulated period); delivery strictly after send at P0.
  * C25 (Low) two "catch-up caps": loop ticks per wake (floor(0.25*TICK_HZ), 15 at 60 Hz) vs per-client commands per tick (<= redundancy cap 8, 0001:238-239); 4.6 "every tick-counted timer derives from TICK_HZ" would give 15 > 8 at 60 Hz. Name apart.
  * C14 confirmed: -19's 100 joins into a full 48 room + Worker-trap rejoins need token reuse per slot with replace-old-session semantics.
  * C26 (High) -56's record (0001:1578) wants BOTH OQ-0001-6 options, each with -42's correction share, -48's registration and -49's honest rewind at P2-P4. Plan runs both options only for -20 (6.2 :472; R3 -48/-49 1.0 h fits one option). And no rule says under which option the rule cap is computed, -49's rule-cap run runs, and the checklist plays (buffer wait is a part of the honest rewind, 0001:1569, so the cap differs per option). Order: -48 (both options) -> OQ-0001-6 pick (-20, -21, -42 L1, -48) -> rule cap for the picked option -> -49 rule-cap run -> checklist 7, 10.
  * C21 strengthened: 0001:1569 lists "the wait from the drawn frame to the tick that sent the command (0 to one frame)" as a part; LG/L1 shooter must run a frame loop or that part is 0 and the cap is low by up to a frame.
