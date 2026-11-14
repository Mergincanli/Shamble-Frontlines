# u3 verify round 2: model v2 (tcp_model.rs) skeptic, rust-reviewer

Head: cc72f6f (+ checkpoint commits to df4afa1; spikes/ identical, git diff cc72f6f df4afa1 -- spikes/ empty).
Linux v6.8 sources in the session scratchpad (linux-src-v6.8, -more). No agent memory dir found.
(One progress line was first appended by a shell heredoc by mistake; from then on, Write only.)
Worktree .gate/u3v2-model (HEAD df4afa1) used, then cleaned (git apply -R of the saved patch) and removed (git worktree list:
main + another skeptic's u3v2-stats). Target .gate/u3v2-model-target. All diagnostic edits: .gate/u3v2-model-diag.patch
(773 lines; applies to cc72f6f's spikes/; knobs TcpConfig.undo / exitcwnd / disorder / oldseq, trace, diag_* tests; env
DIAG_SETS, DIAG_ROW, DIAG_SEED, DIAG_BURST, DIAG_UNDO, DIAG_EXIT, DIAG_OLDSEQ). Outputs under .gate/: u3v2-model-run1.txt
(reproduction), u3v2-trace1762.txt (S13 harness), u3v2-trace1762-fixes-{built,undo,oldseq}.txt, u3v2-trace13945-{built,undo}.txt,
u3v2-trace1053-{built,undo-exit}.txt, u3v2-diag-undo6.txt, u3v2-diag-undo60.txt, u3v2-diag-disorder60.txt, u3v2-diag-se.txt,
u3v2-diag-oldseq60.txt, u3v2-diag-allrows60.txt, u3v2-diag-undo-oldseq60.txt.

## Plan
1. Prior notes and fix plan. 2. on_ack order/sampling. 3. Timestamps. 4. RACK. 5. on_rto. 6. Handshake.
7. Seed 1762 trace. 8. S13 tests and bites. 9. Biases; F-RTO/undo; would 659 / 1,685 move.

## Findings (one line each, as found)
- R0 reproduction: worktree, own target dir, `cargo +1.99.1 test --release -p npt-net --lib fixes -- --ignored --nocapture
  --test-threads 1` (18.3 s): every number the main session will send reproduces exactly (today 746/947/3170, 1031 over 0.8 s;
  fix 1 60k 435/664/1552/5106, 434 over 1 s, 82 over 1.5 s, 8 over 2 s; REC 60k 435/659/1025/1363/1685, 279/63/2 over
  0.8/1/1.5 s; pins 437/665/1318; abort 1 s 660/1028/1691; abort 500 662/1015/1546; two endpoints 637/880/1918; no-TS rec
  max 1691, fix1 max 4553; P = 0.558/0.910/0.984/1.000, fix1 0.379/0.686; side effects 4/20/74/116 permille, 0/0/11/18).
  13 non-ignored tcp_model tests pass. Not sent but seen: today over 60,000 phases has max 4,884 ms.
- V1 (item 1, on_ack order/sampling): CONFIRMED. snd_una moves before sacktag (tcp_input.c:3927, then 3929-3931); sacktag
  skips skbs at/below the new snd_una (1397) = tcp_model.rs:1115. FLAG_RETRANS_DATA_ACKED for any cum-acked RETRANS skb,
  SACKed or not (3319-3322) = :1133-1134; first/last_ackt only !RETRANS && !SACKED_ACKED (3323-3327) = :1135-1138;
  first/last_sackt never-resent, seq order (1418-1428) = :1182-1186; flag kills seq sample (3390) = :1176-1181; sack sample
  (3146-3147) = :1187; TSecr only with FLAG_ACKED (3155-3157) = :1188-1195; ca_rtt (3391-3407) = :1180, :1184-1186;
  min_rtt from ca (3168) = :246-247. Minor, listed: min_rtt windowed + delayed-ACK filter (3119-3133).
- V2 (item 2, timestamps): CONFIRMED except G1. rcv_wup at every non-SYN send (tcp_output.c:288-289) = :963;
  tcp_replace_ts_recent seq <= rcv_wup + PAWS (input.c:3746-3758, 3904; tcp.h:1635-1651) = :877-883; in-window end >= rcv_wup
  (tcp_sequence 4424-4433) = :879; TSval ms (:550-552); tcp_rtt_tsopt_us (689-703) = :555-558.
- G1 (MAJOR after measuring): an old duplicate (end_seq < rcv_wup) fails tcp_sequence and tcp_validate_incoming DISCARDS the
  whole segment (input.c:5882-5900, 'goto discard', tcp_send_dupack 4618): its ACK field is never processed. Model runs on_ack
  on every arrival (:884). Not in S11. After a burst the client's input resend is such a duplicate (its original reached the
  server before the burst): in seed 1762 the +223 ms ACK the explanation rests on is exactly that (seq 17864-17952 < rcv_wup
  18128; trace u3v2-trace1762-fixes-oldseq.txt): Linux would never process it. Knob (discard + dupack): 86,586 discards in
  60,000 REC phases; REC 435/659/1025/1685 -> 479/716/1143/1944, over 0.8/1/1.5 s 395/89/8, P(0.8,1.0) 0.387,
  P(1.0,1.5) 0.937; fix 1 alone p99 664 -> 757.
- V3 (item 3, RACK): CONFIRMED (recovery.c:118-144 = :1161-1169; spurious skip on the cum path only, input.c:3339-3341 vs
  1400-1401 = :1158; min_rtt before the sample = :1154; reo_wnd recovery.c:5-29 = :1231-1235). Minor: Linux runs RACK only when
  rack.advanced, on dubious ACKs (recovery.c:95-112); model on every ACK with progress (:1212).
- V4 (item 4, on_rto): CONFIRMED (input.c:2164-2166 = :1283-1291; 2194 = :1296; 2185-2193 = :1278).
- V5 (item 5, handshake): CONFIRMED (minisocks.c:678-716, 808-809, 866; input.c:3177-3186, 6202-6203; output.c:3719-3720;
  metrics.c:528-538; minisocks.c:594-596 = :782-853), with minor G2: tcp_try_undo_spurious_syn (input.c:6319-6332, called
  6411/6727) clears undo_marker when the echo equals the first SYN/SYN-ACK stamp -> no cwnd 1; model always cwnd 1 (too slow,
  rare). G3 minor, listed: cached crtt replaces the 3 s fallback (metrics.c:521-527).
- D1 (item 6, seed 1762): CONFIRMED AS MODEL BEHAVIOUR (u3v2-trace1762.txt; fixes harness also 3,170): head first sent
  -1,017.1 (TSval 6866), delivered -967.4, ACK lost; RTO -380.1 resend lost; ACK +223.0 cum 216300 TSecr 6866 -> Karn, TSecr
  sample 1,240,000 us, srtt 258, RTO 1,413, backoff 0; Loss cwnd 2, two resends +223.0 both lost; RTO +1,636.8; fresh +3,170.5.
  REFUTED AS LINUX: (i) that ACK rides on an old duplicate Linux discards (G1); (ii) were it processed, tcp_process_loss
  (2873-2881) would undo the loss state: snd_una advances and tcp_may_undo (2550-2553) holds via tcp_packet_delayed (2457-2461:
  TSecr 6866 before retrans_stamp 7503, the first retransmission, output.c:3423-3424) -> tcp_try_undo_loss (2616-2639) ->
  LOST cleared, cwnd 10, Open (2521-2548). Knob results for this phase: undo 2,461 ms, discard 2,181 ms, by other paths.
- B1 (MAJOR, item 8): no loss-recovery undo in the model: neither the timestamp (Eifel) undo (2879-2881, tcp_packet_delayed)
  nor F-RTO step 3b (FLAG_ORIG_SACK_ACKED, 2883-2890; frto armed at RTOs, 2215-2217; tcp_frto = 2, tcp_ipv4.c:3437) nor the
  DSACK undo. S11 lists only F-RTO, as an aside. It is the common path after a burst (the last ~50 ms of pre-burst segments
  arrive, their ACKs die). Knob (Eifel + F-RTO 3b, no DSACK, no step 2b): 116,155 undos in 60,000 REC phases (9,762 of them
  F-RTO 3b). REC -> 448/665/1074/3,296, over 1.5 s 7, over 2 s 1; P(0.8,1.0) 0.498, P(1.0,1.5) 0.945, P(1.0,2.0) 0.992.
  Today p50 746 -> 641. So F-RTO DOES touch the recommended rows, and the Eifel undo (not listed at all) far more.
- D2 (REC's new 3,296 tail, seed 13945, u3v2-trace13945-undo.txt vs built 343 ms): stale SACK of never-resent pre-burst
  segments -> SACK RTT 1.24 s and rack.rtt_us 1,204 ms -> F-RTO 3b undo; the pending RTO (+182.7) then spares every
  burst-time segment within rack.rtt (2164-2166), cwnd = in flight + 1 leaves room for the head only, head lost, doubled RTO
  2,779 ms. Linux-plausible; the model as built cannot produce it.
- B2 (item 8, S2 text): "App-limited flows neither grow nor shrink it" REFUTED: tcp_end_cwnd_reduction sets cwnd = ssthresh at
  every Recovery exit with undo_marker set (2689-2703; tcp_enter_recovery 2851-2852): x0.7 per RACK repair; model keeps 10-13.
  Knob (exit cwnd = ssthresh, Reno +1 per cwnd delivered when cwnd-limited: a pessimistic stand-in for Cubic): REC
  437/671/1050/1871; with B1: 449/832/1113/1905, 910 over 0.8 s, P(0.8,1.0) 0.071 (trace u3v2-trace1053-undo-exit.txt:
  undo restores prior cwnd 4, the 5 lost burst-time segments fill it, nothing moves until the TLP at 2 x inflated srtt).
- B3 (minor): TLP only in Open/CWR (output.c:2845-2850); Linux is in Disorder with SACKed/lost/resent segments out
  (input.c:2719-2731); the model has no Disorder (:425). Knob: REC 659 -> 659 alone; with B1 max 3,296 -> 1,940.
- SACK blocks: no ACK in 6,000 today or REC phases needed more than 3 ranges (the TS option's limit): inert.
- Combined (60,000 phases, u3v2-diag-allrows60.txt, -undo-oldseq60.txt). Definite mechanisms (B1 + G1): REC
  496/723/1143/1978, over 0.8/1/1.5/2 s 442/87/8/0, P(0.8,1.0) 0.358, P(1.0,1.5) 0.937, P(1.0,2.0) 0.999; fix 1 alone
  496/1217/1905/2136, 608 over 1 s (fails a 1 s 99 % bar), P(1.0,1.5) 0.067; abort 1 s 726/1847 P(1.0,1.5) 0.900; two
  endpoints 691/2709 P 0.945. All four knobs: REC 497/787/1155/1890, P(0.8,1.0) 0.243, P(1.0,1.5) 0.907; fix 1 alone p99 1,308.
  No-TS REC with B1+G1: 499/751/1159/1795. Side effects with B1+B2: 4/20/79/126 permille, 0/0/12/14 attempts (unchanged).
  Reading: the browser reconnect becomes essential (fix 1 alone loses the 1 s p99), the recommendation's ranking holds, but
  its p99 is ~720-830 not 659, max ~1.8-2.0 s (3.3 s with B1 alone), and WS-LIMIT (1.0, 1.5) passes ~0.90-0.94, not 0.984;
  (1.0, 2.0) holds at 0.999.
- P1 (pass chance arithmetic): at_most(...) x (1 - r_max)^480 treats "<= 4 over the 99 % bar" and "none over the max" as
  independent; exact is P = sum_k C(480,k) (r99 - rmax)^k (1 - r99)^(480-k): REC (0.8,1.0) 0.585 not 0.558; fix 1 (1.0,1.5)
  0.438 not 0.379; others equal to 3 decimals. Conservative, minor. Sampling noise dominates the max bar: 2 of 60,000 over
  1.5 s (Garwood 95 % 0.24-7.2) puts P(1.0,1.5) anywhere in ~0.94-0.998 from the model alone.
- T1 (item 7): (a) single loss -> RACK, (b) probe capped by RTO, (c) lost resend -> RTO, (e) burst sweep + pin, (f) stall count,
  and the bites (constant repair fails a; no doubling fails d and moves e; loss off fails f) still test what they name. (d)'s
  last claim does not: with timestamps the resent head's ACK already resets the backoff by its TSecr sample (worktree diag:
  backoff 0 before the new message with TS, 3 without), so tests.rs:193-205 passes without the "new message" path; run (d)
  with timestamps off or assert both. No test or bite pins any v2 rule (Karn/first_ackt, TSecr fallback, rcv_wup rule, RTO
  RACK exception, cwnd = in flight + 1, handshake rules): only the (e) and REC pins move if one regresses.

## Verdicts
VERDICT: REVISE. Model v2's line-by-line rules match v6.8 (V1-V5); the numbers reproduce exactly (R0); the seed-1762 story is
right for the model. But three Linux paths that act in nearly every burst phase are missing (G1 old-duplicate discard, B1
undo, B2 cwnd at recovery exit): with them the recommended p99 moves 659 -> ~720-830 ms, max 1,685 -> ~1.8-2.0 s (3.3 s with
the undo alone), WS-LIMIT (1.0 s, 1.5 s) pass chance 0.984 -> ~0.90-0.94; fix 1 alone loses its 1 s p99 (~1.2-1.3 s).
DONE
