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
- F5 (base model, S7): v6.8's RTO does NOT mark every outstanding segment lost: tcp_timeout_mark_lost (tcp_input.c:2160-2168) skips
  any non-head skb still inside RACK's window (tcp_rack_skb_timeout > 0, tcp_recovery.c:32-36: rack.rtt + 0 - elapsed), and
  tcp_enter_loss sets cwnd = packets_in_flight + 1 (tcp_input.c:2194), not 1. Model tcp_model.rs:1124-1129 marks all and sets 1.
  Effect: only on an RTO with segments sent in the last ~RTT (the first RTO in a burst); later RTOs agree. Size unclear, small.
- F6 (base model, F-RTO): tcp_frto = 2 by default (tcp_ipv4.c:3437) and tcp_enter_loss arms it (tcp_input.c:2215-2217). S11 records
  only "F-RTO's undo"; F-RTO also reorders the send: step 2b sends NEW data after the first ACK in Loss when new data waits
  (tcp_process_loss / tcp_xmit_recovery REXMIT_NEW), retransmitting the rest only after the next ACK. Not modelled. Direction:
  the old connection's in-order recovery is too good by about one RTT when unsent data waits (64 KB budget always; small budgets
  when the room wrote into a freed slot). Not instrumented (needs the kernel's tcp_process_loss, not downloaded).
- F7 (item 4/7, MAJOR): browsers serialize WebSocket connection attempts to one IP:port until the handshake completes, so fix 3's
  "one attempt every 300 ms, in parallel" (fixes.rs:196-212) cannot happen in Chromium (Brave, Chrome) or Firefox.
  Chromium main (fetched 2026-11-06 23:45 UTC, raw.githubusercontent.com/chromium/chromium/main): websocket_endpoint_lock_manager.h:24-32
  quotes RFC 6455 4.1.2 ("no more than one connection in a CONNECTING state ... MUST serialize them"); .cc:138-139 queues a second
  lock request (ERR_IO_PENDING); transport_connect_sub_job.cc:117-126 takes the lock BEFORE the TCP connect; the lock is released at
  WebSocketBasicHandshakeStream::Upgrade() (websocket_basic_handshake_stream.cc:408-410), i.e. after the 101, plus 10 ms
  (websocket_endpoint_lock_manager.cc:26, :80), or on failure (transport_connect_sub_job.cc:137). Firefox (gecko-dev master):
  WebSocketChannel.cpp:2903-2904 ("1 session at a time ... in the CONNECTING state per server IP address"), :325-354 (queued),
  OnConnected only after mGotUpgradeOK (:3795-3806, ~:4059); and a FailDelay of 200-400 ms growing x1.5 after a failed attempt
  (:114-139). So an attempt whose SYN died in the burst blocks every later one until its own SYN retry (+1 s on Linux) gets
  through. Numbers to follow from the worktree (D2).
- D1 (worktree .gate/u3vf-wt, knobs off = pinned 399/610/737 exactly; output .gate/netcode-proof-test/u3vf-diag1.txt): in the
  RECOMMENDED run the old connection delivers the first fresh snapshot in 978 of 1,000 phases; a reconnect wins only 22. So its
  p50/p99 are the adaptive budget's TCP recovery; fix 3 only trims the tail (adaptive alone 399/645/1645). 9 phases had a false
  takeover BEFORE the burst, after which fixes.rs:112/:198 (`current == 0`) switch reconnects off for the rest of the phase.
- D2 browser endpoint lock (one attempt CONNECTING at a time, lock freed at the 101; 10 ms ignored = generous): REC 399/632/803
  (+22 p99, +66 max). Restart policy (close the pending attempt every 300 ms) = no reconnect at all (max 1645): a TCP+TLS+upgrade
  takes 4 RTT (~400 ms at P2) > 300 ms, so each attempt is killed before it opens.
- D3 app-level RTT in the trigger (a browser cannot read TCP srtt): +17 ms -> 399/620/771; +33 ms -> 399/593/780.
- D4 Linux's Karn/oldest-sample rule (F4b): REC 429/647/701 (p50 +30, p99 +37). With TSecr too: 428/651/697. rto_mark_rack (F5):
  no change on REC. All three model fixes + lock: 429/666/846; + app RTT 17 ms: 429/672/880. Server srtt at P2 108 ms as built vs
  118 ms with Linux's rule (diag_srtt, 20 seeds).
- D5 (item 6a) Karn explanation REFUTED in the model itself: fix 1 alone with TSecr samples (backoff reset on any ACK of new data,
  resets did fire) stays p99 1416 (as built 1415), max 3723. The worst phases have max_backoff == max icsk_retransmits == 4 (s1010,
  s1081): four RTOs with snd_una never advancing, i.e. no ACK at all between them, so no RTT rule could reset anything; it is
  consecutive lost retries. With Linux's real sampling rule the max is 1899 (not 4203): the 4.2 s max is a seed coincidence.
- D6 (item 6b) rto_min explanation REFUTED: the max phases have max_backoff 0 (never doubled) and icsk_retransmits 4-5 (< 6):
  rto100 s1538 1528 ms b0 rx4; rto50 s1287 1709 ms b0 rx5. Phases that did exhaust the 6 (rx7) recovered in 542-592 ms. With
  TSecr samples, rto50's max falls 1709 -> 584 ms. Cause being traced (D7).
- D7 (traces, .gate/netcode-proof-test/u3vf-trace1.txt): the rto_min max phases are a MODEL BUG, not exhausted linear retries.
  rto100 s1538: seg 125 (sent 9 ms before the burst, never resent, its SACK lost in the burst) is cum-acked after the return
  together with the resent seg 124; the model (tcp_model.rs:1025) takes 125 as an RTT sample of ~1.1 s -> srtt 114 -> 207 ms,
  RTO 214 -> 1,246 ms at backoff 0, so one later loss waits 1.25 s. rto50 s1287: same (RTO 179 -> 1,312 ms). Linux takes no
  seq sample from that ACK (FLAG_RETRANS_DATA_ACKED, tcp_input.c:3318-3322, :3390) and the TSecr fallback echoes the resend's
  TSval (ts_recent only updates for seq <= rcv_wup), ~1 RTT. So F4b's deviation inflates RTOs right after every outage.
  fix1 s1508: the RTO resend at +286 ms is lost (2 %), nothing is acked, the next RTO is doubled (+1,573): no RTT rule involved.
  REC s1545: same pattern on conn 0; the reconnect wins at 737.
- D8 (.gate/netcode-proof-test/u3vf-diag2.txt) rto_min with Linux's sampling rule: rto100 236/515/639, rto50 289/498/855, against
  thin rto200 350/606/1002 (same rule). So "rto_min 100/50 worsen the max (1,528, 1,709)" is an artifact of the model's Karn bug:
  REFUTED. (Some phases do exhaust the 6 linear retries, rx7, but recover in 570-855 ms.)
- D9 seed robustness of the pinned recommendation (as built; 6 sets of 1,000 phases: seeds 1000/2000/3000 same burst timings,
  4000/5000/6000 shifted timings): p50 399-433, p99 571-646, max 674-1,059 ms; set 3000 has 1 phase over 1 s (1,059) and 3 over
  0.8 s. So "0 of 1,000 over 1 s" and max 738 are one draw; the proposed "every one within 1.0 s" fails on another seed set of the
  same model. Faithful variant (Linux Karn + TSecr + browser lock + app RTT +17 ms): p50 422-457, p99 618-722, max 815-1,441;
  over 1 s in 2 of 6 sets (4 and 1 phases). Mustafa's goal (all within 1.5 s, 99 % well under 1 s) still holds in all 12 sets.
- D10 side effects of the recommendation exactly (thin OFF; fixes.rs:428-432 ran it with thin ON): identical, 0/24/88/123 per mille
  skipped, silences 0/0/2/2 in 3 min: CONFIRMED (0.67/min at P3/P4 from 2 events each; Poisson 95 % CI 0.08-2.4/min). The phase
  harness also shows 9 false takeovers in ~100 min of P2 play (about 0.09/min), where the side-effects table reports 0 at P2.
