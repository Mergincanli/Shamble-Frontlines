# U3c skeptic round — netcode-architect review notes

Scope: git diff 2c68bf9..989118e -- spikes/ (U3c), against u3c-plan.md rev 2 [C1]-[C16], advisor-u3c-plan.md, advisor-u3q.md S0-S14.
Focus: DelayStream vs S0-S14; ws_gate vs fixes.rs:777 and W22; TickWriter newest-only; per-return record S12 times; SERIES rows; loopback tests + mechanism test (c) vs rev 2 1.4-1.5; kernel cross-check classification.
Rules: no network changes; CARGO_TARGET_DIR=.gate/u3c-review-target; edit no source.

## Plan
1. Read u3c-plan.md, advisor-u3c-plan.md, advisor-u3q.md S0-S14.
2. Read delay.rs, delay_clock.rs, ws_gate.rs, fixes.rs, ws_config.rs.
3. Read tests (delay_tests, ws_gate_tests), SERIES.md.
4. Kernel check compare.py + verdict.
5. Run tests.

## Findings (one line each, as found)
- (resume 2026-11-08) Prior session left only the plan. Read u3c-plan rev 2, advisor-u3c-plan (incl. side rulings), advisor-u3q S0-S14, build-a + fix notes, critic-a.
- ws_gate.rs:36-38 may_write == W22 / old fixes.rs:775-776 (owed 0, free >= size, outstanding + size <= n x size); fixes.rs:778-785 now calls it (n = scale x snapshot_budget, u64::MAX when not adaptive): one definition. snapshot_budget table (a) = [6,3,4,6,8,15] checked by hand (2 x TICK = 66,666,666). OK.
- TickWriter (ws_gate.rs:116-137) holds nothing outside the cfg(test) bite: newest-only and never-later are structural; (b) sweep asserts iff/newest/never-later/counts/-31; bites bite. OK.
- delay.rs:1100-1111 poll_write: one record, n = min(len, 16,384, free-22), Pending at free <= 22, never Ok(0) for non-empty; model_write +22. 30,000/40,000 test (delay_tests.rs:504-537) exact. OK.
- delay.rs:358-411 harvest: releases whole records in order (model_end <= rcv_nxt). OK. Pump writes one record per S.poll_write (delay.rs:1225-1267), split counted. OK.
- delay.rs:1286 up read floor: up_room >= read_floor (= max_frame + 22 = 549). OK; move_up (delay.rs:472-487) only moves a whole record that fits.
- F-held (low): down held bound is 64 KB + one record's acked-unreleased prefix (<= 16,405 B), not 64 KB: free_budget(0) = budget - (write_seq - snd_una) frees the acked prefix of the front unreleased record (tcp_model.rs:2211-2214) while its bytes stay in bytes[DOWN] (delay.rs:283-287 counts only released_model). Doc delay.rs:39, SERIES ws_*_held_bytes row and plan 1.2 say "at most 64 KB". Test delay_tests.rs:580-581 is lossless so never exercises it.
- Kernel check: compare.py:25-43 + tests.rs:1391-1459 apply run.sh:19-37's rule (half gap, nearest-rank p50, >1 ms prefilter, bound RTT+gap+2ms+1ms) identically to kernel and model; header committed in c5d7260 before the run (1a9fd73). Classification follows the pre-written rule. PASS stands.
- F-power (low): check (2) has almost no power: band(172, p) contains k=6 for p in ~[0.01, 0.085] (computed); the model's 246 "other" are ~125 timeouts + 2 probes + ~119 double losses, so a model with NO timeout path (double losses only, p ~0.02, band 0-9) passes too. Check (1) on loopback: model L = gap + tcpi_rtt (33.458) vs kernel gap + 11 us (33.344); the 2 ms mark is 16x the RTT, so it tests "repair at the next send's SACK", not RACK's RTT/reo_wnd terms. Neither is wrong vs the rule; the verdict should say what it can't see (S14 binding).
- run.sh:56-57 band wording "smallest lo ... with P(X<lo) <= 0.005" (nit): the code (compare.py:86-101) takes the largest such lo (the smallest is always 0).
