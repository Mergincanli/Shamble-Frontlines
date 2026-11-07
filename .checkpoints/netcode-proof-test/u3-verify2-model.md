# u3 verify round 2: model v2 (tcp_model.rs) skeptic, rust-reviewer

Head: cc72f6f (+ checkpoint commit 0dae8b7). Linux v6.8 sources in the session scratchpad (linux-src-v6.8, -more).
No agent memory dir found. (One progress line was first appended by a shell heredoc by mistake; from here on, Write only.)

## Plan
1. Read prior skeptic notes (u3-verify-fixes-model.md) and fix plan (u3a-fix-plan.md) for what was claimed fixed.
2. Read tcp_model.rs on_ack / sampling, compare with tcp_ack -> tcp_sacktag_write_queue -> tcp_clean_rtx_queue -> tcp_ack_update_rtt (v6.8).
3. Timestamp option: tcp_select_window (rcv_wup), tcp_replace_ts_recent, tcp_sequence, PAWS, TSval ms, tcp_rtt_tsopt_us.
4. RACK: tcp_rack_advance, tcp_rack_detect_loss, min_rtt.
5. on_rto: tcp_timeout_mark_lost, tcp_enter_loss cwnd.
6. Handshake: tcp_check_req, SYN-ACK timer, tcp_init_metrics 3 s fallback, tcp_init_transfer cwnd.
7. Seed 1762 worst phase: instrument a worktree copy (.gate/u3v2-model).
8. S13 tests (a)-(f) and their bites.
9. Anything biasing numbers; F-RTO; whether 659 / 1,685 move.

## Findings (one line each, as found)
- (progress) Read tcp_model.rs v2 in full, fixes.rs, tests.rs; kernel reads: tcp_clean_rtx_queue 3279-3420, tcp_ack_update_rtt
  3135-3172, tcp_sacktag_one 1378-1440, tcp_rack_advance recovery.c 118-144, tcp_timeout_mark_lost 2143-2168, tcp_enter_loss
  2174-2220, tcp_check_req minisocks.c 650-866, tcp_init_metrics metrics.c 464-538, tcp_process_loss 2873-2935, undo 2521-2640.
- V1 (item 1, on_ack order/sampling): CONFIRMED. Kernel: snd_una moves before sacktag (tcp_ack 3911-3931 via
  tcp_ack_update_window), sacktag skips skbs at/below the new snd_una (tcp_input.c:1395-1396) = model filter g.end > ack.cum
  (tcp_model.rs:1115). FLAG_RETRANS_DATA_ACKED for any cum-acked skb with TCPCB_RETRANS, SACKed or not (3306-3309) = model
  :1133-1134. first/last_ackt only from !RETRANS && !SACKED_ACKED (3310-3318) = :1135-1138. first/last_sackt only for
  never-resent newly SACKed (1418-1428) in seq order = :1182-1186. seq sample suppressed by the flag (3390) = :1176-1181;
  seq<0 -> sack sample (3146-3147) = :1187; TSecr only if FLAG_ACKED (3155-3157) = :1188-1195 (any_acked); ca_rtt
  last_ackt, overridden by last_sackt (3391-3407) = :1180, :1184-1186. min_rtt from ca (3168) = :246-247.
  Not modelled (minor): min_rtt is a 300 s windowed min with the FLAG_ACK_MAYBE_DELAYED filter (3119-3133) [listed].
- V2 (item 2, timestamps): CONFIRMED with one gap. rcv_wup = rcv_nxt in tcp_select_window (tcp_output.c:288-289) for every
  non-SYN transmission = model transmit_any (:963). tcp_replace_ts_recent: seq <= rcv_wup and PAWS (tcp_input.c:3746-3758,
  tcp.h:1635-1651) = :877-883; in-window end_seq >= rcv_wup (tcp_sequence 4424-4433) = :879. TSval ms = ts_of (:550-552);
  tcp_rtt_tsopt_us: whole ms, at least 1 (689-703) = :555-558. GAP G1: a segment failing tcp_sequence (end_seq < rcv_wup: an
  old duplicate) is DISCARDED whole by tcp_validate_incoming (5882-5900, 'goto discard'): its ACK field is not processed and a
  dupack is sent. Model always runs on_ack on it (:884). Affects spurious duplicates after the burst (their ACK info); not listed
  in S11.
