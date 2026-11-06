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
- CHECKED: /proc/self/timerslack_ns exists here (reads 50000); /proc/thread-self/timerslack_ns does not (verifier right).
- CHECKED: quinn-proto 0.11.19 PTO = (smoothed + max(4 x rttvar, 1 ms)) x 2^pto_count + max_ack_delay x 2^pto_count,
  from the last ack-eliciting send (connection/paths.rs:331-333; connection/mod.rs:1831-1858; MAX_BACKOFF_EXPONENT 16
  at :4053). DATAGRAM frames are congestion-controlled (mod.rs:604-622: only loss probes bypass the window) and a probe
  packet carries pending datagrams (populate_packet, mod.rs:3350-3360).
- COURSE CHANGE (coordinator, 2026-11-06): Mustafa answered U3-Q1..Q9 himself; his answers decide, I do not rule on
  them. Q3 = (b) Linux-faithful; -18 keeps 500 ms for WebTransport; the WebSocket fallback gets its own limit from
  the faithful model plus a margin (about 1.5 s), as a docs PR for his go; Q6 migration off. My remaining work:
  (1) the faithful model's exact spec + L1's QUIC-like streams vs U2 C1 (450 ms vs 1.1 s); (2) the docs PR's exact
  words and the rows/lines it touches; (3) which design problems U3's build plan must fix.
- CHECKED in the scout's downloaded Linux v6.8 sources (session scratchpad linux-src-v6.8/): TCP_RTO_MIN HZ/5,
  TCP_RTO_MAX 120 s, TCP_TIMEOUT_MIN_US 2 ms, TCP_TIMEOUT_INIT 1 s (include/net/tcp.h:144-150); __tcp_set_rto =
  srtt/8 + rttvar (tcp.h:706-709); tcp_rtt_estimator: first sample rttvar = max(2 x rtt, rto_min), then mdev_max reset
  to rto_min once per RTT and rttvar decays only toward it, so rttvar >= 200 ms always (tcp_input.c:861-927); RACK
  reo_wnd and skb timeout, reorder timer + 2 ms (tcp_recovery.c:5-36, 58-112); TLP = srtt/4 (2 x SRTT) + rto_min if one
  packet out else + 2 ms, capped by the RTO's remaining time (tcp_output.c:2829-2874); RTO from the head's
  (re)transmission (tcp.h:2390-2397, tcp_input.c:3214-3233); backoff x2 to 120 s (tcp_timer.c:654-655). All as the
  scout and verifier said.
- RECONCILED (U2 C1's 450 ms vs the scouts' 1.1 s): mine = E15's constant max(200, 2 x 100) = 200 ms doubling: retries
  0.2, 0.6, 1.4 s after a send at the burst's start -> 1.45 s, 450 ms after the link returns. Linux: RTO = SRTT +
  max(RTTVAR, 200 ms) ~ 300 ms at P2, doubling, from the head's send (~-0.05 s: the last segment whose ACK fell in the
  burst): ~0.25, 0.85, 2.05 s -> delivered ~2.1 s, ~1.1 s after. My "TCP's backoff" wording in U2 was loose: it was
  E15's constant with RFC 6298 doubling, not Linux's RTO. QUIC (quinn): PTO ~157 ms at P2 from the LAST ack-eliciting
  send, doubling: with nothing else sent, probes 0.16, 0.47, 1.10 -> ~150 ms after; with datagrams sent until the
  window fills (~0.45 s), probes 0.61, 0.92, 1.55 -> ~600 ms after. Three different models, three numbers; none wrong.
