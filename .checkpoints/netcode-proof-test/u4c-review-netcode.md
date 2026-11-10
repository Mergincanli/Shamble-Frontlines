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
