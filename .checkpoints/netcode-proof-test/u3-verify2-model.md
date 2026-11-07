# u3 verify round 2: model v2 (tcp_model.rs) skeptic, rust-reviewer

Head: cc72f6f (+ checkpoint commits to df4afa1; spikes/ identical, git diff cc72f6f df4afa1 -- spikes/ empty).
Linux v6.8 sources in the session scratchpad (linux-src-v6.8, -more). No agent memory dir found.
(One progress line was first appended by a shell heredoc by mistake; from here on, Write only.)
Worktree: .gate/u3v2-model (HEAD df4afa1), target .gate/u3v2-model-target. Outputs under .gate/: u3v2-model-run1.txt
(reproduction), u3v2-trace1762.txt (S13 harness trace), u3v2-trace1762-fixes-{built,undo}.txt, u3v2-trace13945-{built,undo}.txt,
u3v2-diag-undo6.txt, u3v2-diag-undo60.txt. Diagnostic patch scripts: scratchpad u3v2/patch_undo.py, diag_*.rs.

## Plan
1. Read prior skeptic notes and fix plan. 2. on_ack order/sampling vs tcp_ack. 3. Timestamps. 4. RACK. 5. on_rto.
6. Handshake. 7. Seed 1762 trace (worktree). 8. S13 tests and bites. 9. Biases; F-RTO/undo; would 659 / 1,685 move.

## Findings (one line each, as found)
- R0 reproduction: worktree, own target dir, `cargo +1.99.1 test --release -p npt-net --lib fixes -- --ignored --nocapture
  --test-threads 1` (18.3 s): every number the main session will send reproduces exactly (today 746/947/3170, 1031 over 0.8 s;
  fix 1 60k 435/664/1552/5106, 434 over 1 s, 82 over 1.5 s, 8 over 2 s; REC 60k 435/659/1025/1363/1685, 279/63/2 over
  0.8/1/1.5 s; pins 437/665/1318; abort 1 s 660/1028/1691; abort 500 662/1015/1546; two endpoints 637/880/1918; no-TS rec
  max 1691, fix1 max 4553; P = 0.558/0.910/0.984/1.000, fix1 0.379/0.686; side effects 4/20/74/116 permille, 0/0/11/18).
  Non-ignored tcp_model tests: 13 pass. Not sent but seen: today over 60,000 phases has max 4,884 ms (u3v2-diag-undo60.txt).
- V1 (item 1, on_ack order/sampling): CONFIRMED. snd_una moves before sacktag (tcp_input.c:3927 tcp_ack_update_window, then
  3929-3931 sacktag); sacktag skips skbs at/below the new snd_una (1397) = model filter g.end > ack.cum (tcp_model.rs:1115).
  FLAG_RETRANS_DATA_ACKED for any cum-acked skb with TCPCB_RETRANS, SACKed or not (3319-3322) = :1133-1134. first/last_ackt
  only from !RETRANS && !SACKED_ACKED (3323-3327) = :1135-1138. first/last_sackt only for never-resent newly SACKed
  (1418-1428, seq order) = :1182-1186. seq sample suppressed by the flag (3390) = :1176-1181; seq<0 -> sack sample (3146-3147)
  = :1187; TSecr only if FLAG_ACKED (3155-3157) = :1188-1195; ca_rtt last_ackt, overridden by last_sackt (3391-3407) =
  :1180, :1184-1186; min_rtt from ca (3168) = :246-247. Minor, listed: min_rtt 300 s windowed + delayed-ACK filter (3119-3133).
- V2 (item 2, timestamps): CONFIRMED with one gap. rcv_wup = rcv_nxt in tcp_select_window (tcp_output.c:288-289) for every
  non-SYN transmission = transmit_any (:963). tcp_replace_ts_recent: seq <= rcv_wup and PAWS (tcp_input.c:3746-3758,
  tcp.h:1635-1651), called from tcp_ack (3904) = :877-883; in-window end_seq >= rcv_wup (tcp_sequence 4424-4433) = :879.
  TSval ms = ts_of (:550-552); tcp_rtt_tsopt_us whole ms, at least 1 (689-703) = :555-558. GAP G1: a segment failing
  tcp_sequence (end_seq < rcv_wup: an old duplicate) is DISCARDED whole by tcp_validate_incoming (5882-5900): its ACK field is
  not processed, a dupack goes back. Model always runs on_ack on it (:884). Not listed in S11. Small.
- V3 (item 3, RACK): CONFIRMED. tcp_rack_advance sets rack.rtt_us on every advance except a resent skb with rtt < min_rtt
  (recovery.c:118-144) = :1161-1165; mstamp only if sent after = :1166-1169; cum path skips tcp_skb_spurious_retrans
  (tcp_input.c:3339-3341; def 2447-2452) = :1158; SACK path calls it unconditionally (1400-1401). min_rtt as before this ACK =
  :1154. reo_wnd (recovery.c:5-29) = :1231-1235. Minor: Linux runs RACK only when rack.advanced and only on dubious ACKs
  (recovery.c:96-112 via tcp_fastretrans_alert); model on every ACK with progress (:1212).
- V4 (item 4, on_rto): CONFIRMED. tcp_timeout_mark_lost skips non-head skbs with rack.rtt_us - elapsed > 0 (2164-2166,
  recovery.c:32-36) = :1283-1291; cwnd = packets_in_flight + 1 (2194) = :1296; cut condition (2185-2193) = :1278. One head
  resend: matches tcp_retransmit_skb(head).
- V5 (item 5, handshake): mostly CONFIRMED. Repeated SYN: SYN-ACK at once, timer timeout << num_timeout, req->ts_recent not
  replaced (minisocks.c:678-716; the update at 808-809 is after the early return) = :782-795. Echo of the first arriving
  SYN = :783-788. Client sample from the SYN else TSecr = :805-811. Server tcp_synack_rtt_meas(child, req) (minisocks.c:866;
  input.c:3177-3186), first SYN-ACK stamp (tcp_output.c:3719-3720) = :831-837. 3 s fallback (metrics.c:528-538) = :292-298.
  cwnd 1 (input.c:6202-6203), child undo_marker only with num_timeout (minisocks.c:594-596) = :839-853. GAP G2 (minor, too
  slow): tcp_try_undo_spurious_syn (6319-6332, called 6411 and 6727) clears undo_marker when the echo equals the first
  SYN / SYN-ACK stamp (tcp_output.c:4093; minisocks.c:598-605): only the reply was lost -> no cwnd 1. Model always cwnd 1.
  GAP G3 (minor, listed): cached crtt replaces the 3 s fallback (metrics.c:521-527); no-TS rows only.
- D1 (item 6, seed 1762): the explanation is CONFIRMED AS MODEL BEHAVIOUR (trace u3v2-trace1762.txt, S13 harness, and the
  fixes harness gives the same 3,170): head [215250,216300) first sent at -1,017.1 ms (TSval 6866) and delivered at -967.4,
  its ACK lost in the burst; RTO at -380.1 (backoff 1, all 10 marked, cwnd 1) resends it, lost; ACK at +223.0 cum 216300,
  TSecr 6866 -> Karn removes the seq sample, TSecr sample 1,240,000 us, srtt 258, RTO 1,413 ms, backoff 0; Loss, cwnd 2:
  two resends at +223.0 both lost; next RTO +1,636.8; fresh delivered +3,170.5. BUT NOT LINUX: at that same ACK Linux runs
  tcp_process_loss (2873-2881): snd_una advanced and tcp_may_undo (2550-2553) holds via tcp_packet_delayed (2458-2461:
  TSecr 6866 before retrans_stamp, the first retransmission at -380.1 = 7503 ms, set at tcp_output.c:3423-3424) ->
  tcp_try_undo_loss (2616-2639) -> tcp_undo_cwnd_reduction(unmark_loss) (2521-2548): LOST cleared, cwnd = prior 10, Open.
  Linux would not be in Loss with cwnd 2 there. With the undo modelled (diag knob) this phase is 2,461 ms, by another path.
- B1 (MAJOR, item 8): the model has NO loss-recovery undo: neither the timestamp (Eifel) undo of tcp_try_undo_loss(sk,false)
  (2879-2881), nor F-RTO's step 3b (FLAG_ORIG_SACK_ACKED -> tcp_try_undo_loss(sk,true), 2883-2890; frto armed at every RTO
  from Open or recurring, 2214-2217, tcp_frto = 2), nor the DSACK undo. S11 lists only "F-RTO (its new data first after an RTO,
  and its undo)", as if rare. It is the COMMON path after a 1 s burst: the segments sent in the last ~50 ms before the burst
  arrive but their ACKs die, so the post-burst ACK echoes a pre-burst TSval (< retrans_stamp) and/or (s)acks never-resent
  data below high_seq. My approximation (worktree knob: Eifel via TSecr + F-RTO 3b, no DSACK, no step 2b) fires 116,155
  times in 60,000 REC phases (~1.9 per phase, both ends). Effect on REC 60,000: 435/659/1025/1685 -> 448/665/1074/3,296 ms;
  over 1.5 s 2 -> 7, over 2 s 0 -> 1; P(0.8,1.0) 0.558 -> 0.498, P(1.0,1.5) 0.984 -> 0.945, P(1.0,2.0) 1.000 -> 0.992.
  Today (60k): p50 746 -> 641. Fix 1 alone: max 5,106 -> 3,296, over 1.5 s 82 -> 140. (u3v2-diag-undo60.txt)
- D2 (the new 3,296 ms REC tail, seed 13945, u3v2-trace13945-undo.txt vs -built.txt 343 ms): post-burst ACK SACKs two
  never-resent pre-burst segments (sent -1,066/-1,033) -> SACK RTT sample 1.24 s, rack.rtt_us 1,204 ms (tcp_rack_advance)
  -> F-RTO 3b undo (LOST cleared, Open, cwnd 13); the old RTO fires at +182.7 (snd_una did not advance, no rearm): it now
  spares every burst-time segment sent within rack.rtt (1.2 s) (2164-2166), cwnd = in flight + 1 leaves room for the head
  only; head resend lost (2 %); the doubled RTO is 2,779 ms; the fresh snapshot (written +200) cannot be sent. The
  reconnect attempt (started -779 in the burst) did not win either. A Linux-plausible tail the model as built cannot produce.
- B2 (item 8, S2 text): "App-limited flows neither grow nor shrink it" is REFUTED for Linux: tcp_end_cwnd_reduction sets cwnd
  = ssthresh at every Recovery exit when undo_marker is set (2689-2703; undo_marker from tcp_enter_recovery 2851-2852), so
  each RACK repair under 2 % loss cuts cwnd x0.7 (Cubic beta 717/1024); the model keeps cwnd 10-13 forever (only on_rto and
  establish change it, :1296, :852). Knob (cwnd = ssthresh at exit + Reno growth +1/cwnd delivered when cwnd-limited, a lower
  bracket for Cubic): REC 60k 437/671/1050/1871; with B1's undo too 449/832/1113/1905, 910 over 0.8 s, P(0.8,1.0) 0.071.
