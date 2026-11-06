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
- CHECKED further (v6.8): tcp_send_loss_probe sends new data if the windows allow, else resends the highest segment,
  one probe per episode, then "Reset s.t. tcp_rearm_rto will restart timer from now" (tcp_output.c:2900-2956);
  tcp_enter_loss marks all outstanding lost, cwnd = in flight + 1 (tcp_input.c:2174-2194); backoff reset on a valid
  RTT sample (tcp_input.c:3172); TCP_INIT_CWND 10 (tcp.h:241); TCP_DELACK_MIN/ATO_MIN HZ/25, TCP_DELACK_MAX HZ/5
  (tcp.h:134-139); defaults reordering 3, sack 1, early_retrans 3, RACK, frto 2, min_rtt_wlen 300 s
  (tcp_ipv4.c:3414-3452); quinn-proto initial window 14,720 clamped to 10 x 1,200 = 12,000 B (congestion/cubic.rs:266).
- FINDING (refines the scouts' 1.1 s): the faithful B1 recovery depends on the burst's phase. Example at P2: the head
  of the unacked queue is often a segment that WAS delivered just before the burst (only its ACK, sent in the burst,
  was lost); TLP ~0.25 s, RTO from the probe 0.55 s, doubled 1.15 s (outside the burst) -> ACK ~1.25 s -> the first
  burst-lost segment resent then, delivered ~1.3 s: ~0.3 s after the link returns. If a resend lands just before the
  burst ends, the next is 1.2 s later: ~1.2-1.4 s after. So 1.1 s is one phase, not the bound; the WebSocket limit
  must come from the model's longest recovery over swept phases. My arithmetic on the kernel's timers; U3 measures.
- FINDING (QUIC side): quinn's datagrams are window-limited; during a 1 s blackout the 12,000 B window fills after
  ~0.3 s at 30 Hz, so quinn waits for its backed-off PTO (from the last ack-eliciting send): e.g. 0.46, 0.77, 1.40 s
  -> ~0.4-0.45 s after the link returns; phase decides, about 0.15-0.6 s. -18's 500 ms on WebTransport is at risk from
  the transport itself, not the room. L1 has no window, so it cannot show this; L2 (real quinn) decides. Derived from
  quinn-proto's code, not run: worth a loopback check at U6 before R3.

## (1) DelayStream's Linux-faithful model: the spec for U3 (Mustafa's U3-Q3 (b), 2026-11-06)
Reference kernel: Linux v6.8 (Ubuntu 24.04's GA kernel) with default sysctls; RS's kernel and sysctls are read at R1
(U3-Q9) and any difference is reported. Sources below: v6.8 files as downloaded by the scout (session scratchpad
linux-src-v6.8/), line numbers checked by me unless marked.
S0. Shape. One model per WebSocket connection, both directions (down: RS's TCP sends; up: the client's TCP, emulated
  as Linux too: right for LG, recorded as a simplification for browsers on Windows and macOS). Pure logic on ns stamps
  (no I/O, no clock), a pub(crate) module of npt-net, unit-tested on simulated time; DelayStream drives it from the
  release clock. Placement as decided (U3-Q4 (b)): above TLS, below tungstenite; the model counts each written TLS
  record as its plaintext + 22 B (TLS 1.3 record header 5 B, inner content type 1 B, AEAD tag 16 B; RFC 8446 §5.2),
  records of at most 16,384 B. No new public symbol (§4.7 keeps DelayStream).
S1. Segments: MSS 1,448 B (1,500 MTU - 20 IP - 20 TCP - 12 timestamps; E15's figure); Nagle off (TCP_NODELAY on the
  real socket too); each segment's one-way delay = the profile's half latency + uniform jitter of +- half its jitter,
  drawn per transmission; loss drawn per transmission at the profile's rate; every transmission inside a burst lost.
  The stream itself never reorders (R-5's step is the datagrams'; RACK's reorder window therefore stays at its base).
S2. Window: in flight <= cwnd segments, initial 10 (TCP_INIT_CWND, tcp.h:241); outside the timeout path the 30 Hz
  flows are app-limited, so growth and PRR are not modelled (recorded); after a timeout, slow start (+1 per segment
  acknowledged) from cwnd = in flight + 1 = 1 (tcp_enter_loss, tcp_input.c:2174-2194), ssthresh = Cubic's
  max(0.7 x cwnd, 2) (beta 717/1024, net/ipv4/tcp_cubic.c: NOT in the downloaded set; U3 verifies the line). The
  receiver's window = DelayStream's free read budget (64 KB); the persist timer is not modelled (recorded).
S3. Acknowledgements (receiver): at once for a segment that arrives out of order or fills a hole (tcp_input.c:5690-5707;
  RFC 5681 §4.2), with SACK blocks for every out-of-order range (tcp_sack = 1, tcp_ipv4.c:3426); in-order data at the
  earliest of 40 ms after arrival (TCP_DELACK_MIN = HZ/25, tcp.h:138), the next reverse-direction segment (piggyback),
  or a second unacknowledged segment's arrival; ato adaptation above 40 ms not modelled (recorded). ACKs travel the
  reverse direction with its own delay and loss draws (cumulative and SACK state repeats in later ACKs).
S4. RTT and RTO: Linux's estimator, integer us, exactly as tcp_rtt_estimator (tcp_input.c:861-927): first sample
  srtt = m, rttvar = max(2m, rto_min); then mdev and mdev_max per RTT with mdev_max reset to rto_min, so rttvar >=
  200 ms always; RTO = srtt + rttvar (__tcp_set_rto, tcp.h:706-709), rto_min 200 ms (TCP_RTO_MIN = HZ/5, tcp.h:145),
  1 s before the first sample (TCP_TIMEOUT_INIT, :150), at most 120 s (:144). So RTO ~ SRTT + 200 ms: about 250, 300,
  350, 400 ms at P1-P4. Samples: Karn's rule (none from a resent segment), one per ACK from the newest segment it newly
  acknowledges or SACKs (Linux also samples by timestamps; recorded). min_rtt = the run's minimum (min_rtt_wlen 300 s,
  tcp_ipv4.c:3452).
S5. RACK (tcp_recovery.c:5-112; RFC 8985, recovery = RACK, tcp_ipv4.c:3430): RACK.rtt = the RTT of the most recently
  (re)sent segment that has been (s)acknowledged; reo_wnd = min(min_rtt / 4, srtt), 0 in recovery or once 3 segments
  are SACKed (reordering 3, tcp_ipv4.c:3414; reord_seen stays false); segment S is lost when a segment sent after it
  is (s)acknowledged and now >= S.xmit + RACK.rtt + reo_wnd; else the reorder timer fires at that time + 2 ms
  (TCP_TIMEOUT_MIN_US). Lost segments are resent at once, window permitting. At P2 a single loss in a 30 Hz stream is
  repaired about RTT + 33 ms after its send (the scout's loopback measurement: 45 of 52 stalls at gap + RTT).
S6. Tail-loss probe (early_retrans 3, tcp_ipv4.c:3429; tcp_schedule_loss_probe, tcp_output.c:2829-2874): only in the
  Open state with data in flight; timeout 2 x SRTT + (200 ms if one segment is in flight, else 2 ms), capped at the
  RTO's remaining time (from the head's (re)transmission after a send; a full RTO after an ACK of new data); re-armed
  after each send and each ACK of new data. On firing (tcp_send_loss_probe, :2900-2956): one new segment if any is
  unsent and both windows allow, else the highest-sequence segment again; at most one probe per episode
  (tlp_high_seq); then the RTO restarts from that moment.
S7. RTO: armed when no probe is (tcp_set_xmit_timer, tcp_input.c:3230-3234), at the head's last (re)transmission +
  RTO (tcp_rearm_rto, :3200-3226; tcp_rto_delta_us, tcp.h:2390-2397). On expiry (tcp_timer.c:650-657): every
  outstanding segment marked lost, S2's window collapse, the head resent, RTO doubled (capped at 120 s), re-armed from
  now; the backoff resets at the next valid RTT sample (tcp_input.c:3172). Thin-stream linear timeouts stay off
  (default; tcp_timer.c:636-645).
S8. Every resend (RACK, probe, timeout) is a new transmission: new delay and loss draws, lost inside a burst.
S9. Delivery: the receiver releases bytes in order, when the segment completing the in-order prefix arrives (E15's
  "max(previous delivery, own time)" follows). The send side: the 64 KB write budget = unsent + unacknowledged bytes;
  Pending beyond, never Ready(Ok(0)); its free space is what npt-server reports to npt-room as send-buffer space (§4.4).
S10. Fates: one ChaCha8 stream per (run seed, connection index, direction), a fixed number of words per transmission
  (L1's rule), so the same event sequence replays the same fates. Stamps on RS's monotonic clock.
S11. Not modelled, recorded in every WebSocket run's header with the model's name and version: Cubic's growth curve
  and PRR (app-limited), F-RTO's undo (losses are real), DSACK-driven reo_wnd growth (no reordering), ato above 40 ms,
  TSO/GSO, ECN, the persist timer, HZ's jiffy rounding (4 ms at HZ = 250).
S12. Counted per run and direction: segments; transmissions lost by cause (profile, burst); repairs by path (RACK,
  probe, timeout and its backoff count); each stall's length; held bytes and the longest hold; per burst and
  connection, the time from the link's return to the first in-order byte and to the first whole message released.
S13. Proof in U3 (this container): unit tests on simulated time for (a) a single loss at P2 under 30 Hz traffic
  (stall = RTT + spacing, within the jitter); (b) a tail loss on a quiet stream (probe at 2 x SRTT + 200 ms, capped);
  (c) a lost resend (timeout path, about SRTT + 200 ms after it); (d) doubling and the backoff reset; (e) a 1 s burst
  at P2 swept over at least 1,000 phases with seeded jitter, the writer being the room's (one snapshot message of
  about 1 KB per 33 ms, newest-only above the 64 KB write budget): per phase, the time from the link's return to the
  in-order delivery of the first message WRITTEN at or after the return (the docs PR's number is its maximum), and to
  the first byte (reported): the stale backlog queued during the burst drains first, in slow start from one segment; (f) the stall count equal to the fate draws, and inside the binomial band at P2 (U3's row); bites:
  the old constant in place of RACK (fails (a)'s length), no doubling (fails (d) and (e)), and loss off (0 stalls).
  Plus a cross-check against the real kernel here: the scout's loopback probe (iptables statistic 2 % drop, 30 Hz,
  200 B, TCP_NODELAY; u3-tcp-stall-probe.py) against the model at the loopback's RTT: RACK-path stalls p50 within
  2 ms of the kernel's, timeout-path share within its binomial band.
S14. On RS at R1 (in R1's run plan, inside its first-day slack, no new money): the same pattern over a veth pair in a
  network namespace with netem at 50 ms each way and 2 % loss (no netem jitter: netem's jitter reorders, which the
  model leaves out), 60 s, then 10 one-second total-loss bursts; the kernel's stall and burst-recovery distributions
  against the model's at zero jitter, p50 and p99 within 10 % (proposed). A miss goes to the advisor before the
  WebSocket part of -18 counts. This is what makes "faithful" a measured claim, not a reading.

## (1b) L1's QUIC-like streams, my U2 C1, and the three numbers
- Why the numbers differ (a message sent at a 1 s B1 burst's start, P2):
  | Model | First timeout | Retries | First delivery after the link returns |
  |---|---|---|---|
  | E15's constant + RFC 6298 doubling (my U2 C1) | max(200, 2 x 100) = 200 ms | 0.2, 0.6, 1.4 s | ~450 ms |
  | Linux 6.8 (the scouts) | SRTT + max(RTTVAR, 200 ms) ~ 300 ms, from the head | ~0.3, 0.9, 2.1 s | ~1.1 s for that phase |
  | Linux 6.8, other phases (my reading of S6-S7) | probe ~0.25 s, then timeouts from the probe | e.g. 0.25, 0.55, 1.15 s | ~0.3 s to ~1.4 s |
  | QUIC as quinn does it | PTO ~157 ms at P2, from the last ack-eliciting send | e.g. 0.46, 0.77, 1.40 s | ~0.15 to ~0.6 s |
  My U2 wording "about 450 ms with TCP's backoff" was loose: it was E15's constant with RFC 6298 doubling, never
  Linux's RTO. The scouts' 1.1 s is right for the phase they took, but not the bound. None of the three is an error;
  they model different senders. The WebSocket limit therefore comes from S13 (e)'s swept maximum, not from any of them.
- WebSocket path: Mustafa's (b) makes DelayStream Linux-faithful (S0-S14). B1 over WebSocket is a new part of -18
  (none ran B1 there before); the other WebSocket rows that read repair timing keep their words: -31's 250 ms p99 at
  P2 (single losses: RACK repairs about RTT + 33 ms, so its p99 sits near 140-150 ms by my arithmetic, inside the
  bound) and -29's 3 s connect (the TLS handshake now runs undelayed above DelayStream, U3-Q4 (b): recorded).
- L1's streams are WebTransport's shape (datagrams plus Control and FullState streams on one link), so they model
  QUIC, not Linux. C1 (U2-R2's carried condition) is REPLACED by this, due before an L1 mark reads stream timing (U4):
  C1' (a) messages cut into packets of at most 1,200 B (QUIC's smallest maximum datagram size, RFC 9000 §14; quinn's
  starting MTU), each packet's loss drawn on its own; (b) a packet is declared lost once a later packet of its link
  and direction is acknowledged and either 3 later packets are acknowledged or 9/8 x max(smoothed RTT, latest RTT)
  has passed since its send (RFC 9002 §6.1; quinn-proto config/transport.rs:382-383), and is resent then; (c) else at
  the PTO, smoothed RTT + max(4 x RTT variance, 1 ms) + 25 ms (the max_ack_delay default, RFC 9000 §18.2), doubling at
  each consecutive PTO, reset by an acknowledgement, counted from the last ack-eliciting packet sent (RFC 9002
  §6.2.1; quinn-proto connection/paths.rs:331-333, connection/mod.rs:1831-1858); (d) every resend draws its own fate
  and a burst takes it; (e) each stream delivers in order; (f) unit tests: a single loss under 30 Hz traffic at P2,
  a lost message on a quiet link (PTO ~155-160 ms at P2), a 1 s burst; (g) the crate doc says what L1 leaves out: no
  congestion window, so after a burst L1 sends at once, while quinn, its window full of lost packets, waits for its
  next backed-off PTO (about 0.15-0.6 s after a burst at P2 by quinn's formula, the phase deciding). The old E15
  sentence in L1's doc ("E15's rule for the WebSocket stream") goes. C2 (RTT samples) stands.
- C6 (new, carried to U6, before R3): a loopback B1 check on real quinn, both crates, through ImpairSocket at P2: a
  few connections with 30 Hz datagrams both ways, 10 bursts each at seeded random phases; record the time from each
  burst's end to the first datagram delivered each way. A maximum above 400 ms (80 % of -18's WebTransport limit) goes
  to the advisor before R3: it would be the transport's finding, not the room's, and the mark's words stay.
- Words that change: only -18's (Mustafa's split) and R-25's model sentence (E15's constant leaves the draft). -31,
  -29, -3 and PM-0002-49 keep theirs.
- FINDING (the bite on L2): "decodes deltas again" read as "the first delta decoded after the link returns" can be met
  by STALE deltas: on WebSocket the 64 KB send budget holds the snapshots written during the burst (encoded against a
  baseline still valid then), and on quinn the datagram send buffer (1 MiB default, quinn-proto transport.rs:396)
  does the same. Both are decoded after the return even with the bite "a baseline that never falls back", so the
  bite would be Missed on L2. Reading needed (mine, inside the row being rewritten): a delta counts once it was
  encoded at or after the link's return. It also ties U6's datagram_send_buffer_size (verify-wt design problem 9) to
  -18 on WebTransport: a large buffer delays the first fresh datagram behind the stale ones.

## (2) The docs PR that splits PM-0001-18 (for Mustafa's go)
Route: its own branch from main (this session pushes only to its own branch, so it needs Mustafa's OK for one more,
e.g. `claude/npt-readings-2`, as R-PR had; or another session makes it); classed needs-mustafa (ADR edits); opened
only after U3 has measured S13 (e), so the number is in it, never a placeholder; merged on his go before R3's first
counted -18 run. Main's ADR 0001 (d8c485a) equals the spike branch's, so the line numbers hold on both.
Edits, all in docs/adr/0001-multiplayer-netcode.md ({L} and {sha} filled from U3's run; everything else exact):
E1. Line 1556, the PM-0001-18 row, replaced whole by:
| PM-0001-18 | A loss burst heals (part 6) | after B1's burst, every client decodes deltas again within a plain limit of the link coming back, per client and burst: 500 ms on WebTransport; {L} s on the WebSocket fallback, the Linux-faithful stream model's longest recovery after B1 at P2 over at least 1,000 burst phases (U3, {sha}) plus 25 %, rounded up to 0.1 s (Mustafa, 2026-11-06: "the WebSocket fallback gets its own realistic recovery limit, set from the faithful model with a margin"). A delta counts once it was encoded at or after the link's return, so deltas queued during the burst do not (the advisor's reading, 2026-11-06). Reported, not judged: on WebSocket, the time until a decoded snapshot's extra delay is back under PM-0001-31's 250 ms | L1 (WebTransport's shape); L2 with LG on each transport | B1 with 48 clients, 10 bursts per run (proposed); the WebSocket run through the accept path's delay wrapper (part 13's key) | L1: 3 seeds; L2: 1 run per transport; every client after every burst. Bites, on each transport: a baseline that never falls back (the advisor's reading, 2026-11-06) | r2 R2-SYNC-REC1 mark 2 (line 2020), WebTransport's 500 ms; the WebSocket limit, Mustafa's decision (2026-11-06); the draft, "Failure modes", row "A burst of total loss (B1)" |
E2. Line 747, the failure-modes row, replaced whole by:
| A burst of total loss (B1) | the client holds; on return the server deltas from an older baseline or the defaults | deltas again within 500 ms on WebTransport (R2-SYNC-REC1 mark 2); on the WebSocket fallback within PM-0001-18's own limit, set from a Linux-faithful model of its recovery (Mustafa, 2026-11-06) |
E3. Line 1295: "deltas again within 500 ms after B1," becomes "deltas again within 500 ms after B1 on WebTransport
  (on the WebSocket fallback within PM-0001-18's own limit: Mustafa, 2026-11-06),".
E4. Lines 1463-1468 (part 13's key, R-25's sentence): from "For the proof test, the WebSocket delay is a stream
  wrapper" to "the message size limit applies before buffering (the advisor's reading, 2026-11-06)." replaced by:
  "For the proof test, the WebSocket delay is a stream wrapper inside the server's accept path, above TLS and below
  the WebSocket library, so the room sees real source addresses and the message size limit applies to whole frames
  before any delay buffering (Mustafa, 2026-11-06; the advisor's reading of the same date). Per direction it models
  Linux's TCP as RS's kernel runs it (Ubuntu 24.04's 6.8 defaults; RS's own kernel read at R1), so the fallback's
  results show how it really behaves (Mustafa, 2026-11-06): segments of at most 1,448 B, each delayed by the
  profile's one-way delay plus jitter and lost at the profile's rate on its own, and acknowledgements crossing the
  other direction with their own delay and loss. A lost segment is resent when Linux would resend it: once a segment
  sent after it is acknowledged and one recent round trip plus a reordering window has passed since it was sent
  (RACK, RFC 8985); by a tail-loss probe about two smoothed round trips after the last send; or at the retransmission
  timeout, the smoothed round trip plus its variance, the variance never below 200 ms (about 300 ms at P2), doubling
  at each consecutive expiry, after which the sender restarts from one segment. Every resend draws its own fate, and
  bytes are delivered in order, so a lost segment holds back every later byte. The stream never reorders (the reorder
  step below is the datagrams'). The write side is bounded at 64 KB, returning Pending beyond. Stalls are counted per
  run, by repair path. U3 checks the model against the kernel on loopback, and R1 against RS's own kernel. The Join
  deadline runs from TCP accept. Bytes buffered per connection are capped and counted."
  (This removes "max(200 ms, 2 x the profile's round trip; U3 verifies the floor)": U3 verified it, and the floor is
  on the variance, not on the timeout.)
Nothing else in docs/ on main states the heal limit or E15's constant (git grep: lines 747, 1295, 1556 and 1466
only; docs/research/ is the source and stays). Rows whose words stay: -31, -29, -3, PM-0002-49, and the checklist's
steps 11 (WebTransport) and 13 (WebSocket, no burst).
After the merge, on the spike branch (not in the docs PR): marks.json -18 (the new row's hash; parts L1 B1, L2 B1 with
`transport` wt, L2 B1 with `transport` ws; the 500 ms check on the first two, {L} s on the third, the 480 count on
each, the bite on both L2 parts, the WebSocket report line), SERIES.md (burst_heal's definition: the first delta
encoded at or after the link's return; the new report series), a judge fixture for the split, plan §4.3 E15 (S0-S14's
words), §6.2's -18 row, §7.2 R3 (+0.2 h for the WebSocket run: 10.15 -> 10.35, inside its 11 billed hours and
EUR 1.90 cap), §15 (the fresh-delta reading; R-25's row: the faithful model), §0.4 (Mustafa's U3 answers), npt-l1's
crate doc (C1'), and the catalogue's PeerBurst row (both transports).
