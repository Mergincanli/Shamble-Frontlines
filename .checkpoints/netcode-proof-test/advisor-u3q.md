# Advisor: U3's nine design questions (netcode proof test)

Scope: plan rev 3.2 §12.1 U3, §4.3 E15, §4.4, §4.7, §4.8; the scouting synthesis u3-scout-summary.md and its four notes.
Branch claude/netcode-proof-test-plan-hcky5g, head e884fa4. Not a merge review: rulings U3-Q1..Q9, the Q3/U2-C1
reconciliation, and which design problems U3's build plan must fix.

## Plan (my steps)
1. Read the summary, the four notes, plan §4.3/§4.4/§4.7/§4.8/§12.1, advisor-u2.md (done).
2. Read advisor-u1.md's 0.25 ms condition; ADR 0001 rows -3, -18, -23, -24, R-25 text; marks.json -3, -18; l1 crate doc.
3. Spot-check the load-bearing claims myself (TCP RTO formula lines, -18's heal path, L1's stream code).
4. Rule Q1..Q9, one notes line after each; reconcile Q3 with U2 C1; list the design problems U3 must fix.

## Notes
- (start) head e884fa4 checked; working tree clean; notes file created.
- Read: summary, u3-scout-wt.md, u3-verify-wt.md, u3-verify-timing.md, u3-scout-timing.md (F2-F4, F1e, R, D, Q, T1-T3).
- Read: advisor-u1.md Q10 (my 0.25 ms condition: "U3 measures ImpairSocket's release precision; worse than 0.25 ms
  (5 % of P1's half jitter) goes back to the advisor before R1"); ADR 0001 rows -3 (1538), -18 (1556), -31 (1571),
  R-25 text (1461-1467), failure-modes row B1 (747), part 6 (276-296), checklist steps 11 and 13 (1697-1704);
  marks.json -18 (parts L1 B1 and L2 B1 role LG; max heal <= 500 ms; count >= 480); l1 crate doc (lib.rs:22-30).
- FACT: B1 never runs over WebSocket. -18's parts are L1 and L2-with-LG (R3 VS4: LG's WebTransport clients);
  checklist step 11 (B1) is on WebTransport; step 13 (WebSocket) repeats steps 3, 5, 7 only. The WebSocket marks that
  read stream repair timing are -31 (P2, p99 extra snapshot delay <= 250 ms) and the -29 connect (3 s), not -18.
- FACT: -18's heal path on WebTransport is datagrams (part 6: "deltas from an older baseline or the defaults"); a
  stream enters only if a resync (FullState) is forced, e.g. by an event-queue overflow during the burst.
