# U4c review (netcode-architect), read-only

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-2 (uncommitted), base 097f8970.
Focus: C1' vs RFC 9002 starting state + advisor rulings (advisor-u2.md:72-103, advisor-u3q.md:160-173), C2, event
order, L-c8 judged re-run needs after U4j.

## Plan
1. Read u4-build-plan.md §5 U4c (+ sections it builds to), advisor-u4.md rulings on U4c.
2. Read advisor-u2.md:72-103, advisor-u3q.md:160-173.
3. git status / git diff in the worktree; read lib.rs (model), tests.rs, tests/l1.rs.
4. Check C1' (a)-(g): spaces, packetization, fates, acks, loss detection, PTO, starting state against RFC 9002 §6.2.2/§5.3/§A.
5. Check C2 (newest packet sampling, ack-eliciting largest newly acked), event order (time, fate index).
6. L-c8 after U4j: what the judged re-run needs.
7. Findings + verdict.

## Progress
- Steps 1-2 done: plan §5 U4c (:552-583), C1'/C2 rows (:144-145), advisor-u2.md:72-103, advisor-u3q.md:160-173,
  advisor-u4.md (:105, :180, :186). Worktree status: 2 M (lib.rs, tests/l1.rs) + new src/tests.rs; sha256s equal the
  report (1647e75d, c21a362f, 639ab9c5). HEAD 097f897.
- Step 3 (lib.rs read in full, 1,895 lines). Observations so far:
  - O1 Rtt::START (lib.rs:610-614) smoothed None -> 333 ms, var 166.5 ms, latest 0; first sample sets both (:619-622);
    pto (:635-637) doubled whole incl. max_ack_delay (:903-909) = RFC A.8 + quinn. loss_delay (:640-642) = §6.1.2. OK.
  - O2 C2 sampling (:1503-1511): sample only when the ack's own packet raised `largest` (eligible) AND is newly
    acked. RFC §A.7/quinn sample whenever ack.largest is newly acked, whichever packet sent the ack. L1 is a strict
    subset: drops the sample when A_largest is overtaken by a later ack of a reordered packet, or lost. Rare; doc
    claims "§5.1's largest acknowledged newly acknowledged". Candidate LOW (doc/fidelity).
  - O3 No handshake: real QUIC's 1-RTT data starts with handshake samples (srtt ~RTT); L1 starts at kInitialRtt, so a
    lost first message on a quiet link waits 1,024 ms (quinn ~3xRTT+25 = ~325 ms at P2). Plan T12 chose this. Doc's
    "left out" list does not say it. Candidate LOW-MED (doc + what U4e/U4r's join deadline must know).
  - O4 detect_lost break on first non-lost (:1552-1560): equivalent to A.10 since pn order = send order. OK.
  - O5 PTO base last_sent includes datagrams (:1128, :1593); armed only with stream data in flight (:1592). OK per C1'.
  - O6 Event rank (time, key, order, link, dir, space, tie) unique (:505-509); apply at e.at_ns (:949-954). OK.
  - O7 rtt_ns/state read without advancing (:1737-1747): stale between calls; doc says "every call that takes now".
    Candidate NIT (doc).
- Source S-RFC9002 (www.rfc-editor.org/rfc/rfc9002.html, fetched 2026-11-10, chars 0-100,000 of 105,234): §6.2.2
  "When no previous RTT is available, the initial RTT SHOULD be set to 333 milliseconds", and that this yields a 1 s
  initial *handshake* PTO (TCP's RFC 6298 §2 RTO); §6.2.1: max_ack_delay is 0 in the Initial and Handshake spaces;
  "An endpoint MUST NOT set its PTO timer for the Application Data packet number space until the handshake is
  confirmed"; A.8 "Skip Application Data until handshake confirmed", "duration += max_ack_delay * (2 ^ pto_count)";
  A.7 RTT update iff newly_acked.largest().pn == ack.largest_acked && IncludesAckEliciting; pto_count = 0 near the
  end of OnAckReceived (after the newly-acked early return), if PeerCompletedAddressValidation. Sure: high (spec).
- O7 dropped: npt-link's trait doc (link/src/lib.rs:79-80) already says rtt_ns is "as of the last call that took
  now_ns".
- O3 confirmed by S-RFC9002: 333 ms is the handshake's starting state; an Application Data PTO is never armed before
  the handshake is confirmed, so quinn's first 1-RTT stream message always has handshake samples (plus the extended
  CONNECT exchange). Plan's Join rule: catalogue :93 "within 2 s of its ... QUIC handshake's first packet"; L1: Join
  lost twice -> resent at 1,024 then 3,072 ms > 2 s (P3: 0.25 %/join; -19's ~100 joins -> ~22 % of P3 runs); quinn
  ~0.2-0.5 s. -> MEDIUM F1.
- O2 confirmed by A.7: RFC samples when ack.largest is newly acked, whichever packet sent the ack. L1 needs the ack to be
  the largest's own. Frequency at 30 Hz small (needs a forward-reordered packet's ack to overtake the newest's):
  ~1 % of samples at P4 by my arithmetic; L-c7's oracle (tests.rs:1120-1129) pins the stricter rule. -> LOW F3.
- tests.rs read (1,331 lines); tests/l1.rs diff read; scratch lc8 harness read (replicates probe.rs send loop only).
- probe.rs:210-241: the probe records only SHA-256 of the WHOLE canonical log (now v2). No code path writes
  SHA-256(v1 tag + datagram entries) from an npt-lab run; crates/lab is outside U4c's lane. -> F2 (L-c8 needs).
- Step 4-5 done. Checked OK (no finding): PTO doubling of the whole PTO incl. max_ack_delay (lib.rs:903-909, A.8);
  pto_count reset only on newly acked (:1515-1517, A.7); sample before detect_lost (:1512-1531, A.7 order); loss
  timer precedence over PTO (:1590-1596, A.8); probe takes the oldest unacked retransmittable piece and strips its
  retransmits (:1642-1660, quinn maybe_queue_probe); detect_lost break equivalence (:1552-1560); datagram path and
  reorder step equal to U2's (diff; HeldDue event :1152-1163 vs old poll-time release); flow keys unchanged
  (:865-867); event ranks unique, applied at e.at_ns (:505-509, :949-954); bites reachable only via with_bite's 4 ids
  or the test-only with_model_bite (:741-763, :1824-1845). L-c1..L-c7, L-c9 oracles read (tests.rs:369-1252).
- Marks: -3's five L1 parts select layer+profile only (no scene), so U4j's pre:PM-0001-2 selects do not touch them.
  -19 is L2 only; L1 joins happen at every L1 run's start (48 clients): -17 P3/P4, -20 P3 -> F1's reach.
- Extra: lib.rs:13 "traffic on one link or flow never moves another's fates" now false for stream and ack flows
  (resend counts follow datagram acks; ack index = arrivals in its direction). -> LOW F4. Plan :750 / advisor-u4.md:105
  "L1 starts at 333 ms" vs rtt_ns None (lib.rs:61, :1745-1747): builder's flag right -> LOW F5. 1,200 B is payload in
  L1, UDP payload in RFC 9000 §14 -> NIT F6. v2 log ~2x entries (P2 3,959,634) ~80 B each in memory -> NIT F7.
- Step 6 (L-c8 after U4j) drafted in the report: source for the datagram hash (F2), C4 provenance, 35-run set
  (nit 13(a)), per-check bite outcomes (advisor-u2.md:20-22), exact figure diff vs results/c-u2-pm-0001-3/81c6d01e371b.
- Verdict: APPROVE WITH FINDINGS (F1 M, F2 M, F3-F5 L, F6-F7 nit).
DONE
