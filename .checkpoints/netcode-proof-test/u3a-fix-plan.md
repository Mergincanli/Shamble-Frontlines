# U3a-fix: the model and harness corrections after the two skeptics (main session, 2026-11-07)

Sources: u3-verify-fixes-model.md (B1-B11, D1-D10), u3-verify-fixes-adr.md; workflow wf_b08132f7-b1c journal.
Kernel reads (v6.8, scratchpad linux-src-v6.8 and linux-src-v6.8-more): tcp_clean_rtx_queue 3279-3420,
tcp_ack_update_rtt 3135-3172, tcp_sacktag_one 1378-1440, tcp_timeout_mark_lost 2143-2168, tcp_enter_loss 2174-2220,
tcp_rack_advance (recovery.c 118-144: rack.rtt_us set on EVERY advance, mstamp only if sent after), tcp_init_transfer
6187-6206 (cwnd 1 if total_retrans > 1 && undo_marker), tcp_metrics.c 528-539 (no 3WHS sample: RTO 3 s),
tcp_minisocks.c 594-596 (server undo_marker only if num_timeout), 678-716 (repeated SYN: SYN-ACK at once, timer
now + 1 s << num_timeout, ts_recent NOT updated), 809 (ts_recent from an in-window segment).

## Model (tcp_model.rs)
1. B1 Linux's sampling: seq sample from first_ackt (oldest cum-acked never-resent, not SACKed) unless any cum-acked
   segment was ever resent (FLAG_RETRANS_DATA_ACKED); else SACK sample (first newly SACKed never-resent, seq order);
   else TSecr (timestamps on, ACK cum-acks something). ca_rtt (last_ackt / last_sackt / TSecr) feeds min_rtt.
2. B2 timestamps (TcpConfig.timestamps, default true; MSS 1,448 with, 1,460 without): packets carry TSval (ms);
   receivers keep rcv_wup (rcv_nxt at their last transmission) and ts_recent (TSval of a segment with seq <= rcv_wup);
   ACKs echo it; TSecr RTT = now_ms - tsecr_ms, at least 1 ms.
3. RACK: rack.rtt_us from every advance (SACKed first, then cum-acked, seq order); spurious-retrans skip on cum path.
4. B5 RTO: non-head segments inside RACK's window stay in flight; cwnd = in flight + 1.
5. B6 handshake: repeated SYN -> SYN-ACK at once, timer now + 1 s << timer-retries (no doubling); SYN-ACK echoes the
   first SYN's TSval; resent handshake primes via TSecr; no sample (timestamps off) -> RTO 3 s; cwnd 1 after > 1
   retransmission (client always; server only if a timer retry happened).
6. Doc: S4/S7/S11 text; ADR 0001:442 -> 1467.

## Harness (fixes.rs)
- Browser lock per endpoint (an attempt holds it from creation to its 101; release + 10 ms to the next).
- Policy: endpoints (1 or 2), cadence, abort_after; one attempt in progress per endpoint (creation to JoinAccepted).
- Client RTT from input echo (EWMA 1/8), trigger max(300 ms, 2 x app RTT + 100 ms).
- Room writes and Join handling only at its 30 Hz ticks (B8).
- Count "decodes deltas again" (E1, 0001:1556): the first delta after a FullState; FullState variant as sensitivity.
- Reconnect allowed on whichever connection is current (B7); client_current switches at the FullState.
- Side effects from the same harness, no burst, 10 min x 3 seeds per profile: snapshots skipped, attempts, takeovers.
- 6 seed sets x 1,000 phases per row; pin the recommended row's pooled p50/p99/max.

## Status
- [ ] model edits  - [ ] harness edits  - [ ] tests green + PINNED_E re-pinned  - [ ] table  - [ ] pin  - [ ] commit
