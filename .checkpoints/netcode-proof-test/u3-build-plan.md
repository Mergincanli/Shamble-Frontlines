# U3's build plan (netcode proof test), from Mustafa's U3 answers and the advisor's notes
Inputs: Mustafa's U3-Q1..Q9 answers (checkpoint "Decisions taken", 2026-11-06 about 17:55 UTC); the scouting synthesis
`u3-scout-summary.md`; the advisor's `advisor-u3q.md` §(1) DelayStream's Linux-faithful model S0-S14, §(1b) L1's
QUIC-like streams (C1' replaces U2's C1, due at U4; C6 new, due at U6 before R3), §(2) the docs PR's exact words
(E1-E4) for PM-0001-18's split, §(3) the design problems U3 must fix (pending when this was written).

## Order (each part ends with check.mjs green, the gate, a commit and a checkpoint save)
- **U3a, the TCP model** (`npt-net`, a pub(crate) pure-logic module on ns stamps, no I/O, no clock): S1-S12 exactly as
  advisor-u3q.md §(1) gives them (MSS 1,448 B; initial cwnd 10; delayed ACK 40 ms or the second segment; SACK; Linux's
  RTT estimator with rto_min 200 ms; RACK with reo_wnd = min(min_rtt/4, srtt); the tail-loss probe at 2 x SRTT (+200 ms
  with one segment in flight), capped by the RTO; the RTO doubling to 120 s, reset by a valid sample; slow start from
  1 after a timeout, ssthresh Cubic's 0.7 (verify tcp_cubic.c's beta); every resend its own fate; in-order release;
  the 64 KB write budget; one ChaCha8 stream per (seed, connection, direction), fixed words per transmission; the
  counters of S12). Tests S13 (a)-(f) on simulated time, with the three bites (the old constant in place of RACK; no
  doubling; loss off). **S13 (e)'s swept maximum over at least 1,000 burst phases at P2 gives {L}** = max x 1.25,
  rounded up to 0.1 s: the docs PR's number.
- **The docs PR** (advisor-u3q.md §(2), E1-E4 word for word, {L} and {sha} filled): on `claude/npt-readings`
  restarted from main (Mustafa allowed that branch on 2026-11-06; PR #41 merged, so a fresh PR), needs-mustafa, his go
  before R3's first counted -18 run. Then on the spike branch: marks.json -18 (split parts, the 480 count, the bite
  on both L2 parts, the report line), SERIES.md, a judge fixture, plan §4.3 E15, §6.2, §7.2 R3 (+0.2 h), §15, §0.4,
  npt-l1's doc (C1'), the catalogue's PeerBurst row.
- **U3b, ImpairSocket** (`npt-net`'s l2 module): quinn::AsyncUdpSocket over a real UDP socket (u3-scout-summary.md's
  confirmed shape: never WouldBlock, one datagram per Transmit, GSO/GRO off, due-only poll_recv with a stored waker,
  copies through a buffer pool); the release clock on a dedicated thread with 1 ns timer slack (U3-Q2 (b); the /proc
  path the verifier found; no unsafe code), real-time priority a recorded switch; fates per (peer, direction) ChaCha8
  streams, L1's reorder step (C5), npt-l1's Profile and ReorderStep reused plus one small L2 config type (U3-Q7, plan
  §4.7 updated); the shaper (rate, drop-tail byte limit, payload + 28 B) for -23/-24; B1; the UDP block; the first raw
  packet's stamp per peer for the Join deadline and oversize drops counted before buffering (U3-Q8); `try_send` errors
  logged by kind (quinn-udp's send swallows them); a fate log. Migration off (U3-Q6), recorded in the header.
  Loopback tests: each crate's server on `Endpoint::new_with_abstract_socket` (wtransport's `quinn` feature and
  `with_quic_incoming`; web-transport-quinn's `Request::accept`), a native client, datagrams both ways at P2, the
  injected delay seen in quinn's RTT.
- **U3c, DelayStream**: tokio-rustls pinned (U3-Q5; audited at U5), the wrapper above TLS and below tungstenite
  (U3-Q4 (b)), the U3a model driving it from the release clock; size limits on whole frames before any delay
  buffering; per-connection buffered-byte cap and counter; its free write budget reported as send-buffer space (§4.4).
  Loopback test through tokio-tungstenite; S13's cross-check against the kernel here (u3-tcp-stall-probe.py's pattern:
  RACK-path stalls p50 within 2 ms, timeout share within its binomial band); its stall count checked at P2.
- **U3d, the timing self-test** (U3-Q1 (c)): expected excess lateness beyond 0.25 ms at 20,000 releases/s, inbound
  measured where quinn polls, p50/p99/p99.9/max reported; here it is reported only (RS judges at R1, U3-Q9 facts read).
- **Carried, not U3:** C1' and C2 (U4), C3 (U4), C4 (U4), C6 (U6 before R3), the S14 kernel check (R1's run plan).
