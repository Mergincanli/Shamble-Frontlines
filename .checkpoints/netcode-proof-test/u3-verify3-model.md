# u3-verify3-model: third skeptic round on TCP model v3 (rust-reviewer)

Branch claude/netcode-proof-test-plan-hcky5g, head 1e020a7 (+ checkpoint commits). Read-only on tracked files.
No agent memory dir found. Kernel = Linux v6.8 copies in the session scratchpad (linux-src-v6.8/, -more/, -extra/,
-v3/tcp_cong.c). "input.c" = net/ipv4/tcp_input.c, "output.c" = tcp_output.c, "rec.c" = tcp_recovery.c,
"cubic.c" = tcp_cubic.c, "timer.c" = tcp_timer.c; "model" = spikes/netcode-proof/crates/net/src/tcp_model.rs.

## Plan
1. Read prior notes (u3-verify-fixes-model, u3-verify2-model, u3-build-v3-model) and the merge message of 1e020a7.
2. Read tcp_model.rs in blocks; for each topic (1)-(6) open the matching Linux v6.8 function and compare line by line.
   Record a line per finding here at once.
3. Run the model's unit tests with my own target dir (.gate/rr3-target).
4. Worktree .gate/rr3-wt at HEAD: instrument a median recommended phase (P2, adaptive budget + one-endpoint
   reconnect) and the S13(e) seeds 1017 / 1475; trace each step, mark kernel / not kernel.
5. Check each new rule test + its bite test (tests.rs) tests what its name says.
6. Section (9): residual biases, would p99 1,051 / max 4,196 move.
7. Verdicts, bugs, must-correct. End with DONE.

## Findings (one line each, as found)
- R0 reproduction: `cargo +1.99.1 test --release -p npt-net --lib tcp_model --target-dir .gate/rr3-target` from
  spikes/netcode-proof: 49 passed, 7 ignored (.gate/rr3-unit.txt). The ignored tables (`--lib fixes -- --ignored
  --nocapture --test-threads 1`, 2 min 51 s wall, 11 min CPU) reproduce .gate/netcode-proof-test/u3v3-tables1.txt
  byte for byte except one trailing blank line (.gate/rr3-tables.txt). Every number in the brief is in that output
  (600k: today 1121/2686/12068; fix1 654/2164/11384; fix3 844/1374/5343; REC 654/1051/1859/2092/4196, over 1/1.5/2/3 s
  6768/1009/205/...; pass 0.227, 0.997 [0.993-0.999], 0.990 [0.980-0.997]).
- K1 (1) old duplicates: CONFIRMED. tcp_sequence before(end_seq, rcv_wup) (input.c:4424-4433) = model :2375 (end =
  data end, or seq for a pure ACK: tcp_acceptable_seq = snd_nxt); tcp_validate_incoming discards after
  tcp_send_dupack (input.c:5882-5900) = model :2380-2383 (returns before ts_recent and on_ack). PAWS never fails on
  FIFO paths with rising TSvals (tcp_paws_discard needs ts_recent > tsval): holds. tcp_oow_rate_limited exempts
  data (it only limits data-less segments): model ignoring it is right for data. send_dupack's D-SACK (seq <
  rcv_nxt, end clipped to rcv_nxt, input.c:4618-4637) = model :2443-2450.
- K2 (1) receiver D-SACK: CONFIRMED. tcp_data_queue: rx_opt.dsack = 0 first (input.c:5167), whole duplicate
  dsack_set(seq,end) (:5226-5231), partial dsack_set(seq, rcv_nxt) then queued (:5242-5252) = model :1900-1909;
  ofo duplicate covered by a held range -> D-SACK (tcp_data_queue_ofo) = :1929-1936. tcp_dsack_set (:4565-4583)
  = :1855-1861. Blocks: D-SACK first then selective_acks, room 3 with TS / 4 without = :1836-1848.
- K3 (1) sender D-SACK: CONFIRMED with one kernel quirk not modelled (minor). tcp_check_dsack (input.c:1282-1323)
  = model :1499-1516 (both tests, dumb counting); tcp_dsack_seen (:1021-1057) = :1395-1414 (max_window check and
  rack.dsack_seen omitted, both inert/listed); tcp_is_sackblock_valid (:1245-1280) = :1417-1434; tcp_sacktag_one's
  dup path (:1385-1393, :1445-1452) = :1446-1456, :1488-1490; walk-order: blocks below prior_snd_una skipped
  (sacktag_write_queue) = :1524. Quirk: when a D-SACK block lies inside a later-sorted SACK block (D.start >
  block.start) the kernel tags those skbs twice (once as next_dup of the covering block, once as the D-SACK block
  itself), so undo_retrans drops by 2 for one D-SACK; the model tags once (:1533-1539). Direction: model undoes
  later (pessimistic), only for an ofo duplicate not at a block's start. Not measured.
- K4 (2) undo machinery: CONFIRMED. tcp_init_undo (input.c:2126-2131) = :1028-1032; tcp_retransmit_skb sets
  RETRANS, retrans_out += 1 even if already R ("leaked", FASTRETRANS_DEBUG = 1 at tcp.h:17), retrans_stamp at the
  first (attempted) resend from the skb's ms stamp, undo_retrans = max(0, u_r) + 1 (output.c:3407-3428) = :2683-2690;
  the probe uses __tcp_retransmit_skb (output.c:2944): EVER only, total_retrans++ (output.c:3361, :3393) = :2676-2682
  with marked false. tcp_packet_delayed/tcp_tsopt_ecr_before (input.c:2438-2461) = :1040-1043; tcp_may_undo
  (:2550-2553) = :1046-1052 (undo_retrans -1 is "not zero" in both); tcp_undo_cwnd_reduction (:2521-2548), Cubic's
  undo_cwnd = reno max(cwnd, prior_cwnd) = :1055-1069; try_undo_recovery (:2571-2598, SACK: always Open) =
  :1072-1082; try_undo_dsack (:2600-2613) = :1085-1096; try_undo_loss (:2616-2639) = :1099-1116; try_undo_partial
  (:2935-2966) = :1119-1137. retrans_stamp is cleared only at input.c:2740 (try_to_open), :2955 (partial), :3028
  (alert in Open), :3361 (SYN acked), :6591/:6723 (handshake): the model has the first three (:1203, :1130, :1707);
  the rest do not arise. CONFIRMED.
- K5 (2) dispatch: CONFIRMED. tcp_fastretrans_alert D/E (input.c:2999-3117) = model :1697-1761, including Loss ->
  fallthrough only when Open or FLAG_LOST_RETRANS, and Recovery's partial undo only with SND_UNA_ADVANCED (the local
  flag copy). tcp_ack's order (input.c:3839-4019: snd_una, window, sacktag + count_delivered, no_queue, clean_rtx,
  process_tlp_ack, dubious -> alert, set_xmit_timer, newly_delivered/lost, cong_control, xmit_recovery) = model
  :2746-2837; dubious test (:3509-3513) and num_dupack = :2797-2800. no_queue path (:4008-4021) = :2785-2792.
- K6 (3) F-RTO: CONFIRMED. tcp_process_loss (input.c:2873-2924) = model :1665-1694 line for line (3.b on ANY ACK
  in Loss with FLAG_ORIG_SACK_ACKED; 3.a; 2.b with unsent data; recovered -> try_undo_recovery); tcp_enter_loss's
  frto arm (input.c:2214-2217, retransmits before tcp_update_rto_stats, timer.c:601-603) = :1287 and :2860-2861;
  tcp_xmit_recovery REXMIT_NEW (input.c:3806-3820) = :2729-2743.
- K7 (4) Disorder / probe / RACK: CONFIRMED. try_keep_open, try_to_open (input.c:2719-2748) = :1187-1208;
  tcp_schedule_loss_probe only Open/CWR, 2 x srtt + rto_min with 1 packet out else +2 ms, capped by the RTO left
  (output.c:2829-2876) = :738-769; tcp_send_loss_probe (output.c:2900-2961) = :2883-2910; tcp_process_tlp_ack
  (input.c:3764-3792) = :1644-1662; tcp_rack_reo_wnd (rec.c:5-30, steps 1) = :1297-1304; tcp_rack_detect_loss's
  list walk with break (rec.c:58-93) = model's sequence walk with continue: equivalent because the tsorted list is
  in send order and the skip test is monotone in it; rack_mark_lost only in tcp_identify_packet_loss and the reorder
  timer (rec.c:95-112, :149-167) = :1347-1372, :1783-1798, :2914-2927; tcp_check_sack_reordering callers
  (sacktag end input.c:2023-2024, clean_rtx :3443-3444, partial undo :2944) = :1540-1542, :1629-1631, :1125.
- K8 (5) PRR / reductions / Cubic: CONFIRMED. tcp_init_cwnd_reduction (input.c:2649-2661) = :1140-1148;
  tcp_cwnd_reduction (:2663-2687) = :1151-1171; tcp_end_cwnd_reduction (:2689-2703) = :1174-1184; tcp_enter_recovery
  (:2839-2859) = :1211-1219; tcp_enter_loss (:2174-2218) and tcp_timeout_mark_lost (:2143-2171) = :1242-1292.
  tcp_slow_start / tcp_cong_avoid_ai (tcp_cong.c:458-488) = :895-914; cubictcp_cong_avoid, bictcp_update incl. the
  u64 wrap and u32 truncation of delta/bic_target, friendliness with beta_scale 15, cnt >= 2 (cubic.c:214-339) =
  :827-936; cubic_root table + one Newton step in u32 (cubic.c:167-209) = :482-498; recalc_ssthresh with fast
  convergence (:341-356) = :803-813; cubictcp_state Loss reset (:358-364) = :1010-1016; hystart_ack_delay and
  hystart_update with SK_PACING_NONE halving, delay thresh clamp 4-16 ms (:375-446) = :939-988; cubictcp_acked's
  1 s discard after an epoch start (:448-474) = :947-959; tcp_update_pacing_rate 200/120 % x mss x max(cwnd,
  packets_out) / srtt8 = :992-1005; tcp_is_cwnd_limited (tcp.h:1336-1348) = :695-697; tcp_cwnd_validate first half
  (output.c:1905-1924) = :701-711. Pkts_acked's rtt is rs->rtt_us = ca_rtt_us set in tcp_ack_update_rtt = :1618.
  Assumption flagged by the model itself: HZ = 1,000 (Debian ships 250; Cubic's jiffies and all timer rounding move).
- K9 (6) retrans_out leak: CONFIRMED as kernel behaviour (output.c:3413-3420 adds even when R is set; only the
  FASTRETRANS_DEBUG block at input.c:3459-3478 zeroes it when packets_out = 0; tcp_undo_cwnd_reduction clears only
  LOST, :2526-2533) = model Sender.retrans_leak :558-562, :666, :676, :1586-1588, :2684-2686.
- K10 (6) handshake: CONFIRMED. tcp_try_undo_spurious_syn (input.c:6319-6332; client at :6411 before tcp_ack and
  tcp_init_transfer, server at :6722 before :6724) compares retrans_stamp (client: first SYN, output.c:4093; server:
  first SYN-ACK, tcp_minisocks.c:594-606 with num_timeout) with the echo = model :2273-2282, :2316-2320, :2292-2298;
  tcp_init_transfer cwnd 1 iff total_retrans > 1 && undo_marker (input.c:6202-6205) = :2347; server undo only if
  num_timeout (minisocks.c:594) = :2319. SYN timer: total_rto > tcp_syn_linear_timeouts (timer.c:648-650) = :2406-2410
  (SYNs at 0,1,2,3,4,5,7,11,19,35 s for Linux; for syn_rto 3 s / linear 0: 0,3,9,21 s).
