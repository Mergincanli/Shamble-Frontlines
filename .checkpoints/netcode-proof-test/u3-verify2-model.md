# u3 verify round 2: model v2 (tcp_model.rs) skeptic, rust-reviewer

Head: cc72f6f (+ checkpoint commit 0dae8b7). Linux v6.8 sources in the session scratchpad.

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
