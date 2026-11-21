# u3 build: model v3 (tcp_model.rs, Linux v6.8 loss recovery)

(Copied by the main session on 2026-11-07 from the build agent's worktree copy at
.claude/worktrees/agent-a79d7290e0e6a6e50/.checkpoints/netcode-proof-test/u3-build-v3-model.md: the Write tool refuses
the main folder's path from an isolated worktree. Content unchanged below this line.)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a79d7290e0e6a6e50 (branch npt-model-v3-build, made at
72e6fab; the worktree came up at d8c485a, kit files identical between them). Edits: tcp_model.rs and tcp_model/tests.rs
only. Target dir: /home/user/Shamble-Frontlines/.gate/u3v3-model-target. No commits.
Notes location: this worktree's .checkpoints/netcode-proof-test/u3-build-v3-model.md, mirrored after each block to the
MAIN folder's /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u3-build-v3-model.md (coordinator,
after the 10:15 UTC restart). Backups of v2: scratchpad tcp_model_v2.rs, tests_v2.rs.

## Plan
1. Read the kernel paths (tcp_input.c: tcp_validate_incoming, tcp_sequence, tcp_send_dupack, tcp_data_queue dup path,
   tcp_dsack_set/extend, tcp_check_dsack, tcp_sacktag_one, tcp_init_undo, tcp_enter_loss, tcp_enter_recovery,
   tcp_packet_delayed, tcp_may_undo, tcp_try_undo_*, tcp_undo_cwnd_reduction, tcp_fastretrans_alert, tcp_process_loss,
   tcp_try_to_open, tcp_try_keep_open, tcp_end_cwnd_reduction, tcp_cwnd_reduction, tcp_init_cwnd_reduction,
   tcp_clean_rtx_queue flags, tcp_ack's order; tcp_output.c: tcp_retransmit_skb / __tcp_retransmit_skb retrans_stamp,
   tcp_schedule_loss_probe; tcp_recovery.c: tcp_rack_reo_wnd; tcp_cubic.c; tcp_try_undo_spurious_syn).
2. G1 + D-SACK (rules 1, 2). 3. Undo machinery and dispatch (rule 3). 4. F-RTO (4). 5. Disorder, TLP, reo_wnd (5).
6. Recovery exit, PRR, Cubic, slow start (6). 7. Spurious SYN undo (7). 8. SYN-timer knob (8). 9. Module doc (9).
10. Tests (10), PINNED_E re-pin. 11. fmt, clippy, check.mjs, fixes tables. DONE.

## Design (from the kernel read, 2026-11-07)
- Per-segment flags as the kernel's: sacked (S), lost (L), sretrans (R, a resend in flight), ever (EVER_RETRANS),
  tsorted (on RACK's list); in flight = sum(1 - S - L + R) (tcp_packets_in_flight). Undo with unmark_loss leaves R:
  an R segment counts twice; an S|R one stays in flight until cum-acked (tcp_sacktag_one 1428-1434).
- tcp_ack order: SND_UNA_ADVANCED, WIN_UPDATE (constant advertised window: seq > snd_wl1), sacktag (check_dsack
  first), clean_rtx_queue (flags, RTT, Cubic's pkts_acked), process_tlp_ack, fastretrans_alert only when dubious,
  set_xmit_timer, cong_control (PRR in CWR/Recovery, else Cubic when may_raise = DATA_ACKED), xmit_recovery.
  New data only from tcp_write_xmit (writes, data_snd_check, F-RTO REXMIT_NEW, the probe); retransmissions only from
  xmit_recovery, the reorder timer and the RTO (head only).
- RACK detection only inside fastretrans_alert (rack.advanced) and the reorder timer; reo_wnd with reord_seen and
  tp->reordering (tcp_check_sack_reordering from sacktag, clean_rtx_queue, try_undo_partial); steps fixed at 1.
- TLP ack processing (tcp_process_tlp_ack): a resent probe followed by an ACK past tlp_high_seq = CWR, cwnd =
  ssthresh at once. Extra beyond the brief, same functions; noted for the report.
- Receiver: selective_acks in Linux's order (most recent first, 4 kept), option room 3 blocks with timestamps
  (4 without), D-SACK first and cleared once written.
- Cubic with HZ = 1,000 (jiffy clock for Cubic only; an assumption to check at R1), hystart (ack train + delay,
  SK_PACING_NONE, hystart_ack_delay from the pacing rate with gso_max_size 65,536).
- Not modelled, to list: RFC 2861 cwnd validation and slow start after idle (act only above the initial window),
  SACK compression (<= 1 ms), RACK reo_wnd_steps/dsack_seen, reneging, max_window checks (never bind: one-segment
  ranges), the oow rate limit (only for data-less old packets, unreachable on FIFO paths).

## Progress (one line per rule built)
- [restart 10:15 UTC] Resumed. Checked the partial edit: tcp_model.rs is the whole v3 rewrite (2,898 lines, written in
  one Write before the restart; tests.rs untouched). It already contains, each with its kernel lines in comments:
  rule 1 G1 (dispatch: tcp_validate_incoming/tcp_sequence discard + send_dupack, D-SACK; in-window dup processed);
  rule 2 D-SACK both ends (Receiver.dsack/sacks/max_blocks, data_queue's dup paths; Sender dsack_seen, check_dsack,
  sackblock_valid, sacktag_one dup path); rule 3 undo (init_undo, retrans_stamp/undo_retrans in retransmit, may_undo,
  packet_delayed, try_undo_loss/_recovery/_partial/_dsack, undo_cwnd_reduction, fastretrans_alert per state);
  rule 4 F-RTO (enter_loss arms, process_loss 2.b/3.a/3.b, xmit_recovery REXMIT_NEW, ORIG_SACK_ACKED flags);
  rule 5 Disorder (try_keep_open/try_to_open), TLP only Open/CWR, rack_reo_wnd with reord_seen/reordering;
  rule 6 end_cwnd_reduction, PRR (cwnd_reduction/init_cwnd_reduction), Cubic + HyStart + slow start; rule 7
  spurious_syn; rule 8 TcpConfig.syn_rto_us/syn_linear_timeouts; rule 9 module doc (MODEL v3); plus
  tcp_process_tlp_ack and the SACK-RTT FLAG_SET_XMIT_TIMER rule. NOT yet compiled or tested. Next: build, fix,
  then tests.rs.
- Mirror: the Write tool refuses the main-folder path ("isolated in the worktree"); notes stay here (they survived the
  restart on disk); not worked around with a shell copy. Reported to the coordinator.
- Build: compiles clean (no warnings). Existing tests: 10 pass; (e) moved (expected: re-pin), (f) fails "one segment
  per snapshot" (19,805 of 20,000). Added a test-only trace (TcpModel.trace) and diag_trace_phase (DIAG_IDX).
- (e) median phase traced (seed 1017, burst 6,013 ms, 1,125 ms; scratchpad trace500.txt): pre-burst cwnd 6 /
  ssthresh 4 (each warm-up RACK repair ends Recovery at cwnd = ssthresh: B2); TLP -721 (new data, lost); RTO -395
  (head resent, lost), RTO +255 (resent, arrives): the head is an OLD duplicate at the client (rcv_nxt 189000 >
  187950): discarded, dupack + D-SACK, TSecr 5999 < retrans_stamp 6617 -> Eifel undo (cwnd 6 back, Open); then the
  SACK of the next new segment lets RACK mark the 5 burst-time segments -> Recovery, PRR from cwnd 1 (in flight 0);
  Open at +766; the 64 KB stale backlog drains at cwnd 4-6 -> fresh at +1,125. All steps match the kernel paths.
- (f) seed 1: 1-2 spurious RACK repairs per seed (repairs 366 vs lost 364; discards_old 2). Trace (tracef1.txt,
  +73,638): two client ACKs sent at the same ns arrived at the same ns; the first ACK's push sent new data, the
  second's xmit_recovery resent the hole at the SAME stamp; RACK's tie-break by sequence (tcp_skb_sent_after) put the
  resend first, so the new segment's SACK marked the delivered resend lost. A model artifact (Linux processes each
  arrival at its own tcp_mstamp). FIX: arrivals on a path strictly increase (>= last + 1 ns). After: repairs ==
  lost_profile and stalls == first_lost on seeds 1-3; segments 19,799 / 19,866 / 19,840 (writes held by a smaller
  cwnd merge into MSS segments, as tcp_sendmsg appends to the unsent tail): (f)'s "one segment per snapshot" must be
  re-derived (band from the segment count).
- (e) worst phase traced (seed 1475, 5,114 ms; trace999.txt): a late SACK of a never-resent pre-burst segment sets
  FLAG_ORIG_SACK_ACKED -> F-RTO 3.b undo while the head is still lost (Linux tests the flag on ANY ACK in Loss,
  :2883-2890); the stale SACK sample (1.37 s) makes the RTO 1.57 s; after the next RTO and a second F-RTO undo, RACK
  marks the R-only head lost keeping its R (tcp_mark_skb_lost :1118-1139) and drops it from tsorted (recovery.c:86-87),
  so xmit_retransmit_queue skips it (L|R) until the RTO: kernel behaviour as written. Found one more kernel detail:
  tcp_retransmit_skb on a segment already R adds to retrans_out again ("retrans_out leaked", tcp_output.c:3413-3420),
  cleared only when packets_out reaches 0 (FASTRETRANS_DEBUG = 1, tcp_input.c:3459-3478). MODELLED as
  Sender.retrans_leak. (e) now p50 1,125,455,214 / p99 2,891,749,084 / max 5,497,575,887 ns (not yet pinned).
- Next: tests.rs (unit tests on Sender/Receiver for the rules, model-level for G1, F-RTO, exit cwnd, handshake),
  re-derive (f), re-pin (e).
- tests.rs written (not yet run): (d) both TS on/off (TS on: backoff 0 right after the episode, the echo's sample;
  off: 3 then 0 after the new message); (f) band from the segment count, segments 19,000-20,000 (coalescing);
  new: karn (+bite NoKarn), echo sample (+NoTsecrFallback), rcv_wup echo (+TsRecentAlways), RTO marks/cwnd
  (+RtoMarksAll, +RtoCwndOne), old duplicate G1 (+OldSeqProcessed), receiver D-SACK/SACK order (+NoDsack), sender
  D-SACK + dubious, Eifel undo (+NoUndo), F-RTO 3.b (+NoFrto), F-RTO 2.b/3.a, PRR numbers, Recovery exit cwnd
  (+NoExitCwnd), TLP CWR, Cubic slow start/5 %/app-limited (+NoCubic), Cubic curve + cubic_root accuracy, HyStart
  delay, Disorder (+ProbeInDisorder), handshake spurious SYN (+NoSpuriousSyn), SYN timer knob, distinct arrivals.
- Tests run: fixed 3 of my expectations (SYN times: ip-sysctl.rst:958-966 says RTOs 1,1,1,1,1,2,4 -> SYNs at 0,1,2,3,4,
  5,7,11,19,35, as the model (and v2) did; cubic_root tolerance = integer truncation; the exit-cwnd bite moved to a
  sender-level check since PRR already caps cwnd at ssthresh in the model-level run). All 45 S13 + rule tests pass;
  (e) RE-PINNED (1,125,455,214 / 2,891,749,084 / 5,497,575,887). fmt done; clippy --release --all-targets -D warnings
  clean (one manual_abs_diff fixed in bictcp_update).
- fixes.rs: the_recommended_fixes_meet_the_goal_at_p2 now fails its GOAL assertion (fixes.rs:628) before the pin:
  p99 1,126,939,153 ns, max 2,200,893,434 ns. Not edited (brief). Next: check.mjs, then the fixes tables.
- Fixes tables (scratchpad fixes-tables-v3.txt, 4 s): side_effects_table complete; ws_limit_table printed 7 of 8
  rows, then fixes.rs:381 panicked "seed 2513: no fresh message within the run" (parallel reference row); ws_limit_tail
  panicked at its first row (fix 1 alone) "seed 35518". Replayed 35518 (diag_fix1_phase mirrors fix 1's loop): the
  head's resends att 2-5 all drawn lost (draws 13,216 / 2,441 / 14,483 / 18,167 ppm < 20,000: genuine chance), and
  PRR entered Recovery with nothing in flight -> cwnd 1, so each lost resend waits for an RTO (1.19 -> 2.39 -> 4.78 ->
  9.55 s): fresh only after back + 12 s, the harness's window (phase(): back + 12 s). A stuck detector (segs with no
  timer, or unsent with cwnd 0) over the whole ws_limit_table found NO stuck state (2513 included). So: the harness's
  window, not a deadlock; fixes.rs needs a longer window or to count such phases as over the bar (its call).
  Rows (p50/p99/p99.9/max ms; over 0.8/1/1.5 s): today 1119/2672/4014/6498 (5808/4958/688); fix 1 653/2135/3805/7071
  (755/563/227); 500/500 652/1146/1467/1914 (628/86/4); 600/600 635/1223/1724/2087 (679/388/36); never abort (the
  recommended) 652/1126/1767/2200 (542/79/10); two endpoints 631/954/1804/2916 (155/55/14); abort 1 s
  652/1084/1715/2004 (599/72/8). Side effects (per mille skipped / attempts / takeovers in 30 min): 64 KB 0/0/4/80;
  adaptive 4/21/140/218; never abort 4/21/137/214, 0/7/10/15 attempts; abort 1 s same.
- Detector removed; fmt + clippy (release, all targets, -D warnings) clean again; tcp_model filter: 46 pass, 1 fails
  (fixes.rs's pinned goal test, expected). Running node check.mjs (CARGO_TARGET_DIR = .gate/u3v3-model-target) ->
  scratchpad check-v3.txt.
- check.mjs: PASS fmt, clippy, clippy-wasm32, deps, refs, marks, self, redact; FAIL test, only
  tcp_model::fixes::the_recommended_fixes_meet_the_goal_at_p2 (measured and bites builds); workspace with
  --no-fail-fast: 143 passed, 1 failed (that one).
- fixes.rs will need: its goal assertion (max <= 1.5 s, p99 < 1 s) no longer holds on v3 (p99 1,126,939,153 ns, max
  2,200,893,434 ns; p50 about 652 ms, the exact ns printed once the goal check is changed), and phase()'s 12 s window
  is exceeded by rare phases (fix 1 alone seed 35518; parallel row seed 2513), which panics ws_limit_table and
  ws_limit_tail.
DONE
