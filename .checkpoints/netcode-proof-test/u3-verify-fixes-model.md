# u3-verify-fixes-model: skeptic of the WS fallback's three fixes (rust-reviewer)

Branch claude/netcode-proof-test-plan-hcky5g, head 689c81f (41bd8a0 + checkpoint). No memory found (no agent-memory dir).

## Plan
1. Read Mustafa's request (.checkpoints/netcode-proof-test.md, Decisions taken 2026-11-06 ~21:40) and advisor-u3q.md (1) S0-S14.
2. Read tcp_model.rs, fixes.rs, tests.rs.
3. Check each item against Linux v6.8 sources: (1) thin linear timeouts; (2) rto_min plumbing; (3) ssthresh/tcp_enter_loss;
   (4) handshake + reconnect harness; (5) adaptive budget + scaled trigger; (6) both explanations (instrument a worktree copy);
   (7) anything biasing numbers; would 611/738 move.
4. Run the pinned tests; worktree copy under .gate/ for diagnostics; remove when done.
5. Verdicts, bugs, must-correct.

## Findings (a line per finding)
- F0 (reproduction): ran `cargo test --release -p npt-net --lib fixes -- --ignored --nocapture` from spikes/netcode-proof (the
  pinned 1.99.1 toolchain only loads from that folder; the task's --manifest-path form from the repo root fails "rustc 1.97.0
  is not supported"). Every row of the table reproduces exactly (defaults 708/913/2248; fix1 314/1415/4203; +thin 314/570/995;
  rto100 268/495/1528; rto50 280/472/1709; reconnect alone 651/772/868; adaptive+scaled 399/610/737; side effects 3-snap
  21/331/574/651 permille, adaptive 0/24/88/123 permille, 2 silences/3 min at P3 and P4). Output .gate/netcode-proof-test/u3vf-run1.txt.
- F0b: "adaptive + thin LT, no reconnect" is byte-identical to "adaptive, no reconnect" (399/645/1645): thin never applies
  with the adaptive budget at P2 (>= 6 snapshots allowed out > THIN_PACKETS 4). Not a bug, but fix 2 is inert in the
  recommendation.
- F1 (item 1, thin LT): CONFIRMED vs v6.8. tcp_timer.c:640-645 (ESTABLISHED, thin_lto||sysctl, tcp_stream_is_thin, icsk_retransmits
  <= TCP_THIN_LINEAR_RETRIES -> backoff 0, rto recomputed); tcp.h:2119-2121 (packets_out < 4 && !tcp_in_initial_slowstart);
  tcp.h:1276-1279 (ssthresh >= INFINITE); tcp.h:238 (6); tcp_input.c:3886-3888 (icsk_retransmits = 0 when snd_una advances).
  Model tcp_model.rs:1121-1144: ssthresh cut before the check (kernel: tcp_enter_loss at tcp_timer.c:601 before :640), retransmits
  incremented before the check (kernel: tcp_update_rto_stats at :603 before :643), segs.len() == packets_out (segs holds sent,
  not cum-acked, SACKed included). The clamp to tcp_rto_min at :645 is moot (rttvar floor). Matches.
- F2 (item 2, rto_min): CONFIRMED. TLP extra = tcp_rto_min_us(sk) with packets_out == 1 (tcp_output.c:2856-2861) = model
  tcp_model.rs:390-396; rttvar floor via rto_min (model :243, :248). Deploy note: v6.8 has NO per-socket/sysctl rto_min
  (tcp.c:424 icsk_rto_min = TCP_RTO_MIN; only a locked route metric, tcp.h:740-748, or BPF). Rejected anyway.
- F3 (item 3, ssthresh): CONFIRMED in substance. tcp_input.c:2185-2193: cut when ca_state <= Disorder, or !after(high_seq,
  snd_una), or (Loss && !icsk_retransmits). Model :1121 drops the middle clause but on_ack (:1048-1050) returns to Open as soon
  as snd_una >= high_seq, so it is folded in. cwnd = packets_in_flight + 1 (:2194): see F5 (model sets 1).
- F4 (item 6a, Karn explanation): REFUTED for Linux-to-Linux (timestamps on). tcp_input.c:3141-3157 tcp_ack_update_rtt: when
  Karn removes the seq sample, an ACK that newly acks data takes the RTT from TSecr (saw_tstamp && rcv_tsecr && FLAG_ACKED),
  then tcp_rtt_estimator + tcp_set_rto and icsk_backoff = 0 (:3168-3172). tcp_timestamps = 1 by default (tcp_ipv4.c:3428;
  ip-sysctl.rst:897-905). The model's own MSS 1,448 assumes the timestamp option (tcp_model.rs:45-46) yet S11 drops TSecr samples
  (:35). So on the S14 rig and LG (Linux) the backoff resets on the first ACK of the retransmitted head after the return. Holds only
  for a peer without timestamps (Windows default), where the MSS would be 1,460.
- F4b (base model, S4): Linux's Karn rule is stricter and its sample older than the model's. tcp_input.c:3318-3327 and :3390-3391:
  no seq sample at all if ANY cum-acked skb was retransmitted (FLAG_RETRANS_DATA_ACKED), and seq_rtt = now - first_ackt (the OLDEST
  newly acked never-resent skb); min_rtt uses last_ackt (:3392, :3166). Model tcp_model.rs:1024-1029 samples from the NEWEST
  never-resent delivered segment even when resent ones are in the same ACK. So the model's srtt runs low by up to one tick spacing
  when an ACK covers 2 segments (the every-2nd-segment ACK), and it resets the backoff when no-timestamp Linux would not.
  The advisor spec S4 has the same wording ("from the newest segment"), so the spec is wrong too.
