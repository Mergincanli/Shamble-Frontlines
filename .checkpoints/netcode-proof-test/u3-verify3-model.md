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
  6768/1009/205/...; 60k rows, multi-burst, per-burst and side-effect counts; pass 0.227, 0.997 [0.993-0.999], 0.990
  [0.980-0.997]). spikes/ identical 1e020a7..HEAD (re-checked at ee9ad3e).
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
  later (pessimistic), only for an ofo duplicate not at a block's start. Not measured (NoDsack bite below moves the
  recommended row by <= 6 ms at p99, so D-SACK-driven undo is a small lever anyway).
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

## Block 2: the median recommended phase and the p50 rise (worktree .gate/rr3-wt, diagnostic patch
## scratchpad/rr3/patch1.py: env knobs RR3_VARIANT (the model's own bite variants on every connection), RR3_NODISTINCT,
## RR3_TRACE_SEED; tests rr3_rows / rr3_trace; target .gate/rr3-wt-target. Knobs off = the tables exactly.)
- T1 (7) median recommended phase = seed 1218, burst 6,462 ms, 653 ms (rank 500 of set 0; trace .gate/rr3-trace1218.txt,
  times from the return). Steps, each checked against v6.8:
  (a) pre-burst Open, cwnd 6, ssthresh 5, srtt 125, RTO 325, a leftover undo_marker from an earlier repair (kernel
      never clears it on a non-undone exit) - kernel.
  (b) -996..-896 four snapshots sent and lost; a 7th written snapshot held by cwnd; probe at -737 = min(2 srtt + 2 ms,
      head + RTO - now) (output.c:2856-2873) sends that new segment past cwnd (lost); RTO rearmed from now (icsk_pending
      0, output.c:2957-2960) -> RTO -411 resends the head (lost), backoff 1 - kernel.
  (c) +240 second RTO (Loss, retransmits 1: no new ssthresh, cwnd = in flight + 1 = 1, frto armed) resends the head;
      it reaches the client as an OLD duplicate (rcv_wup past it): discarded, dupack + D-SACK - kernel (K1).
  (d) +274.6 the client's first post-burst segment (sent +230; its own window still full of lost inputs) acks
      198450->200550 (two pre-burst segments whose ACKs died in the burst) with TSecr 6433 < retrans_stamp:
      tcp_process_loss -> tcp_try_undo_loss via tcp_packet_delayed (input.c:2879-2881, :2457-2461): LOST cleared on
      the 5 burst-time segments, cwnd back to 6, Open. Karn removes the seq sample; the TSecr fallback (input.c:
      3155-3157) gives a 1.30 s sample: srtt 125 -> 273, RTO 325 -> 1,465 ms, so the probe would wait 548 ms (to
      +823) - kernel (RFC 7323's echo of the last in-order TSval, sent 1.3 s earlier). Without timestamps the same ACK
      undoes through F-RTO 3.b instead (199500 never resent -> FLAG_ORIG_SACK_ACKED in clean_rtx_queue, :3323-3327).
  (e) +303.8 the room writes the first fresh snapshot (budget freed); sent (in flight 5 < cwnd 6); SACKed at +411.5:
      RACK (reo_wnd = min_rtt/4: Open, 1 SACKed < reordering 3) marks the 5 burst-time segments lost -> Recovery,
      ssthresh 4; PRR with in flight 0: sndcnt = max(1, 1), no SND_UNA advance -> cwnd 1 (input.c:2663-2687) - kernel
      (RFC 6937's slow-start bound from an empty pipe).
  (f) PRR ramps 1 -> 2 -> 3 resends per round (+411, +503, +598); the hole closes at +653: fresh delivered - kernel.
  Every step is a v6.8 path. The p50 rise is real model behaviour: the undo hides the burst losses until a fresh
  snapshot is SACKed one round later, then PRR rebuilds from cwnd 1. v2 (no undo) stayed in Loss and slow-started
  from +275. Sensitivity: had the client not sent at +230, the server would have learned the same at +344.5 (the
  dupack to its own +240 resend), about 70 ms later: the median depends on the client stack's own post-burst timing.
- A1 (7) attribution on the recommended row, 60,000 phases each (the model's own bite variants applied to every
  connection, tables otherwise unchanged): faithful 654/1054/1827/max 3013 (over 1 s 675, 1.5 s 85, 2 s 19);
  NoUndo 509/828/1434/2317 (223/53/9); OldSeqProcessed (G1 off) 626/1145/1817/2615 (1353/90/19); NoExitCwnd
  653/1033/1804/3696; NoFrto 650/1057/1823/2798; ProbeInDisorder 654/1057/1559/3013; NoKarn 657/951/1445/2164
  (350/51/4); NoTsecrFallback 655/945/1496/2165 (337/60/6); NoDsack 655/1060/1827/3013; RtoCwndOne and RtoMarksAll
  identical to faithful (inert here); NoCubic 570/1176/1834/2576 (not a kernel option). So of the +219 ms p50 rise
  (435 -> 654) about 145 ms is the undo machinery (Eifel and F-RTO together, all kernel), the rest the other v3
  rules; the stale-TSecr RTT sample (Karn + echo, both kernel) costs about 100 ms at p99.
- D1 (6) distinct arrivals (arrival >= last + 1 ns, model :2506): FAITHFUL in spirit (tcp_rcv_established refreshes
  tcp_mstamp per segment, so two arrivals never share a send stamp); it changes nothing measurable: recommended
  60,000 with ties allowed (RR3_NODISTINCT=1) 654/1054/1827/2066/3013, over 0.8/1/1.2 s 4785/674/255 vs faithful
  4791/675/256. It matters only for the (f) bookkeeping test (the build agent's +73,638 ms case).

## Block 3: S13 (e) explanations, kernel version, the leak (traces .gate/rr3-trace-e1017.txt, -e999.txt = seed 1475;
## patch2.py adds test rr3_e_seed; patch3.py adds RR3_V612 and RR3_NOLEAK)
- T2 (7) S13 (e) median seed 1017 (burst 6,013 ms, 1,125 ms; index 499; DIAG_IDX=500 gives index 500 = seed 1508,
  also 1,125 ms): the build agent's story CONFIRMED step for step in the trace: pre-burst cwnd 6 / ssthresh 4; probe
  -721 sends new data (lost); RTO -395 (lost); the client's resend +199.7 reaches the server as an old duplicate
  (DISCARD at 0, its ACK unread: G1); RTO +255.5 resends the head, which reaches the client as an old duplicate ->
  dupack + D-SACK with TSecr 5999 -> Eifel undo at +359.9 (cwnd 6, Open, srtt 281 from a TSecr sample, RTO 1,545);
  the next new segment's SACK at +469.6 -> RACK marks 5 -> Recovery, PRR from cwnd 1; Open at +766.5; the 64 KB
  stale backlog drains at cwnd 4 -> fresh at +1,125. Each step is the same kernel path as T1.
- T3 (7) S13 (e) worst seed 1475 (5,497 ms): the story CONFIRMED in substance, with one more mechanism the notes do not
  name. Trace: a profile loss of 189000 at -1099 -> RACK Recovery at -958 (resend in the burst, lost); RTO -642 in
  Recovery (frto NOT armed: state = Recovery and retransmits 0, input.c:2214-2217) and RTO -10.7 (frto armed); +342.6
  the client's segment SACKs 191100 (never resent, sent -1032) -> FLAG_ORIG_SACK_ACKED -> F-RTO 3.b undo while the
  head is still lost; SACK RTT sample 1.375 s -> srtt 273, RTO 1,569; Disorder (no probe). The client's SACK of the
  new segment is lost (+391) and the client sends nothing until +1,864 (its own RTO backoff), so the server waits for
  its RTO (+1,912); the next SACK gives a 1.58 s sample (srtt 436, RTO 2,713); second F-RTO undo; RACK then marks the
  R-only head (and one more) lost keeping R and drops them from the list (input.c:1118-1139, rec.c:86-87): L|R, so
  tcp_xmit_retransmit_queue skips them (the "retrans_out >= lost_out" break and the SACKED_RETRANS skip) and the
  connection sits at cwnd 3 sending only new data until the RTO at +4,635. Not in the notes: after that RTO the hole
  at 211626 (a profile loss at +3,179) waits from +4,837 to +5,448 because retrans_out includes a leaked count (retr 1
  with no R segment out, lost 1: the "retrans_out >= lost_out" break) until a later loss raises lost_out. Without the
  leak (RR3_NOLEAK) S13 (e) is 1,125/2,858/5,115 ms (pinned 1,125/2,892/5,498). All kernel paths as v6.8 reads; the
  5.5 s is a cascade of stale RTT samples (1.3-1.6 s) on both ends plus the stuck L|R segments and the leak.
- V1 kernel version: the model is v6.8; the ADR records the rig's kernel on the run day (ADR 0001:1536, PM-0001-1),
  so RS may run a newer one (Ubuntu 24.04 HWE/cloud kernels are 6.11+). Fetched v6.12 (raw.githubusercontent.com/
  torvalds/linux/v6.12, scratchpad linux-src-v6.12-rr3/) and diffed every function the model cites: 4 differ:
  tcp_packet_delayed (v6.12 tcp_input.c:2474-2492: also true when nothing was resent), tcp_init_undo (counts a TLP
  resend in flight), tcp_enter_recovery (tcp_retrans_stamp_cleanup first, :2903), tcp_enter_loss (clears
  tlp_high_seq: the model's RTO already does, v6.8 timer.c:533); tcp_validate_incoming's SYN_RECV pass (irrelevant).
  RACK (tcp_recovery.c), tcp_retransmit_timer, tcp_retransmit_skb, tcp_xmit_retransmit_queue, the probe and all of
  Cubic are identical. Knob RR3_V612=1 (the three undo changes): today, fix 1 alone, fix 3 alone and recommended rows
  (60,000 each) IDENTICAL to the ms and to the count (the knob is live: it flips recovery_exit's unit tests). So the
  numbers hold for v6.8 through v6.12 at least.
- L1 (6) the leak's weight (RR3_NOLEAK=1, a kernel without it; 60,000 each): recommended 654/1054/1823/2066/3013
  (p99.9 -4 ms, nothing else); today p99.9 4029 -> 3368, max 7246 -> 6332; fix 1 alone p99.9 3887 -> 2932, p99.99
  5638 -> 5297. The leak is v6.8 and v6.12 code (tcp_retransmit_skb identical), so it stays; it does not touch the
  recommended row's p99 or max.

## Block 4: (8) the rule tests and their bites (tests.rs:612-1325; mutation runs in the worktree)
- Q1 CONFIRMED (test does what its name says, bite fails on the planted defect): karn (:689-704), echo sample
  (:723-732), ts_recent at rcv_wup (:747-755), RTO marks and cwnd (:776-792), old duplicate discarded unread + D-SACK
  (:814-823; "unread" shown by the client's snd_una staying 0), receiver D-SACK/blocks (:837-869), sender D-SACK +
  dubious (:874-903, no bite but both branches asserted), Eifel undo (:926-942, both echo cases), F-RTO 3.b (:968-976),
  F-RTO 2.b/3.a (:980-1014, no bite), PRR numbers (:1019-1040, hand-checked: 9, 7, 7), recovery exit with/without
  undo (:1082-1091), TLP CWR four cases (:1095-1117), Cubic slow start / 5 % / app-limited (:1134-1157), Cubic curve
  and cubic_root (:1162-1191), HyStart delay (:1195-1214, no bite), Disorder no probe (:1226-1245), handshake spurious
  SYN (:1266-1275), SYN timer (:1306-1314), distinct arrivals (:1318-1325).
- Q2 WEAK: recovery_ends_with_cwnd_at_ssthresh (:1060-1063) cites tcp_end_cwnd_reduction but passes with the
  NoExitCwnd defect too (worktree test rr3_recovery_ends_bite: after_one_loss(NoExitCwnd) == (7, 7, Open, 1), ok):
  PRR already leaves cwnd at ssthresh in that run. The rule is pinned only by the sender-level recovery_exit test and
  its bite. Harmless; its doc comment overclaims.
- Q3 GAPS (no rule test, only the S13 (e) pin moves): the retrans_out leak (RR3_NOLEAK: only (e) fails, max 5,498 ->
  5,115), tcp_try_undo_partial, tcp_try_undo_dsack, rack_reo_wnd with reord_seen / tp->reordering, the stuck-L|R path
  (R-only segment marked lost by RACK), xmit_retransmit_queue's retrans_out >= lost_out break, Cubic's TX_START epoch
  shift, rearm_rto from the head under a pending probe/reorder timer. All read correct against v6.8 (K4-K9); a future
  regression in them would show only as a moved pin.
- Q4 (tests vs kernel version) recovery_exit_sets_ssthresh_unless_undone and its bite FAIL under RR3_V612=1: on v6.10+
  a recovery with nothing resent is undone (tcp_packet_delayed's new branch). The tests pin v6.8 semantics, as the
  model's MODEL string says ("linux-6.8-tcp v3"); worth one line in the run header that RS's kernel must be recorded
  against it.

## Block 5: (9) what is left; would p99 1,051 / max 4,196 move (patch4.py: rr3_tail_top, conn tracing;
## traces .gate/rr3-trace179118.txt and -179118b.txt)
- M1 the 600,000-phase maximum: top phases of the recommended row (600 sets, 1 min 34 s): 4,196 (seed 179118), 3,766
  (67783), 3,375 (556300), 3,013 (10296), 2,882, 2,842 ... = the "4 over 3 s". Seed 179118 traced: a profile loss
  before the burst -> RACK Recovery (resend in burst); RTOs -660 (frto off: in Recovery) and -45 (frto on); +358 a
  stale SACK (sample ~1.3 s, RTO 1,577) -> F-RTO 3.b undo, Disorder; +465 RACK marks the R-only head L|R (stuck, as
  T3) -> nothing resends it until the RTO at +1,935; that resend is lost (draw 10,258 < 20,000 ppm); the next RTO
  would be +4,890. The reconnect (created -746 in the burst) rescues it: SYN 1 lost in the burst, SYN 2's SYN-ACK
  lost (2 %), SYN 3 at +1,254 completes; its RTT sample is the SYN-ACK's echo of SYN 2 (~1.1 s, kernel: Karn plus
  tcp_ack_update_rtt's TSecr path under FLAG_SYN_ACKED, input.c:3155-3157), so its first data's RTO is ~2.4 s; its
  ClientHello is lost (2 %) and resent at +3,754; takeover +4,111; fresh +4,196. Three independent 2 % losses on top
  of kernel paths: a tail draw, not a model defect. Another seed set would move the max by hundreds of ms either way
  (the 60,000 subset's max is 3,013); the p99 is stable (1,054 at 60,000, 1,051 at 600,000).
- M2 model defects that would move p99 1,051 or max 4,196: NONE FOUND. K1-K10 match v6.8; v6.12's changes leave the
  rows identical (V1); the leak (L1) and distinct arrivals (D1) leave them identical (p99.9 -4 ms at most).
- M3 residual, not modelled (direction, size): (a) the client is Linux TCP except the timestamp option and the SYN
  timer; Windows (Mustafa's Brave) has its own delayed-ACK timer (200 ms class), minimum RTO (300 ms class) and
  RACK/TLP for the client's inputs; the median phase shows the client's first post-burst segment sets when the server
  learns (+275 vs +345 via the dupack path: ~70 ms, T1), so the median can move by tens of ms either way; unmeasured.
  (b) HZ 250 vs 1,000 (timer rounding up to 4 ms, Cubic clock): small, unmeasured. (c) Linux's adaptive delayed-ACK
  ato (> 40 ms at 33 ms spacing) makes the model slightly optimistic when the client's own sender is blocked;
  quick-ack mode after a duplicate makes it slightly pessimistic; both listed in S11. (d) tcp_retrans_try_collapse
  (tcp_retrans_collapse = 1 at tcp_ipv4.c:3432; called from __tcp_retransmit_skb, output.c:3349, gate at :3247)
  merges the client's small input segments on resend: NOT in S11's list; upstream only, negligible for the downstream
  metric. (e) SACK compression (<= 1 ms), TSO/GSO, reordering (FIFO paths by design): listed.
- M4 margins of the three-part bar: its first part ("half within 0.7 s") sits 46 ms above the modelled p50 (654);
  residual M3(a) is of that order, so the 0.997 pass chance is conditional on the client stack behaving like the
  Linux client modelled. Its third part ("none over 3.0 s") rests on 4 phases of 600,000 (rate 6.7e-6; M1: three-loss
  tail draws), so its share of the pass chance is a rare-event estimate with a wide relative error (the stats skeptic
  owns the interval).
- B1 (minor, latent, no number moves): rto_ns (tcp_model.rs:719) uses checked_shl, which only fails for shifts >= 64;
  for backoff >= ~46 the shifted RTO wraps to a small value (the kernel caps icsk_rto at each doubling, timer.c:655:
  min(icsk_rto << 1, TCP_RTO_MAX)). Unreachable in these runs (a 60 s phase window holds about 10 RTOs, and every
  valid sample resets the backoff); would need ~70 min of dead link.
- Cleanup: worktree .gate/rr3-wt restored from HEAD with git show and removed (git worktree list: main, the two agent
  worktrees, .gate/te3s-wt of another skeptic). Diagnostic patches kept as scratchpad/rr3/patch1-4.py.

## Verdicts
VERDICT: APPROVE the model v3 for Mustafa's decision, with REVISE-level wording fixes in what is sent. Model v3 matches
Linux v6.8 line for line on every path the brief lists (K1-K10), the numbers reproduce exactly (R0), the p50 rise to
654 ms is Linux's own undo path plus PRR from an empty pipe (T1, A1), the S13 (e) stories hold (T2, T3: the worst
also needs the leak), the numbers hold for v6.12 too (V1), and the 600,000-phase maximum is a three-loss tail draw
(M1). No defect found moves p99 1,051 or max 4,196. What the main session must say: why the p50 rose; that the
"half within 0.7 s" part has a 46 ms margin against unmodelled client-stack timing (M3a, M4); the kernel/HZ scope;
and the leak's role in the S13 (e) worst phase.
DONE
