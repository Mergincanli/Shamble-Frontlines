# Advisor: BRAWL-1 re-run set P3 ruling (2026-11-15)

## Plan
1. Read main's advisor.md (done), my rulings: plan review (cond 14, 15), r1 (merge step), r2 (A1-A6), m6 (M7-1..M7-4).
2. Read the card's NOW block (M7 record).
3. Check P3 set myself: log header (HEAD, tree), runs.txt, verdict.json vs U4r's P3 verdict.json; -57 values side by side;
   building figure; raw-log bytes vs projection; TICK16-WORDING.
4. Rule on the M7-3 vs results-commit conflict; the per-set check.
5. Rule on P2, then P0 (split).
6. Anything for Mustafa.

## Notes (a line per point)
- N1 read: advisor.md (main), plan review cond 14/15, r1 merge step M7 (:277 "each set's results committed before the
  next"), r2 A1-A6 (A6 runners unchanged), m6 M7-1..M7-4. Plan section 9 expected lines (brawl1-plan.md:258-271).
- N2 set log P3-fd4612050cd6.log: 42 lines, exit 0 (peak.py), wall 1020.5 s, 973.2 MiB. NOT via main-run.sh, so the
  log has NO "HEAD/diff/untracked" header (same invocation as U4r's sets: c-u4r-runs.mjs, flock, CARGO_TARGET_DIR
  unset). runs.txt head: commit fd46120, spikes/netcode-proof tree ebbfeb40. Runner 99dd070e.
- N3 NEW vs U4r P3: overflow resyncs s1 = 27, s3 = 1 (s2 0), each flagged "(overflow resyncs: to the advisor before it
  counts, carry 58)". U4r P3: 0/0/0. Carry 58 (u4s-carries.md:269-273): any overflow resync in a judged -17/-18 run
  comes to the advisor before it counts. Must read the raw logs (when, which slots, kick?).
- N4 raw logs (lab.jsonl overflow_resync events): s1 27 events, ALL ph "measured", t 76,910-77,844, 25 distinct slots
  (44 and 4 twice); s3 1 at t 76,803 (slot 35); s2 none. Measured window 64,036-82,035. No kick: room.jsonl has 48
  admits and no other event; snapshot_clients_count min 48 on all 18,000 measured ticks of s1-s3.
- N5 the cause, read: room.rs:990-1033: gameplay events = Hit/Kill with shooter or victim me or seen; with everyone in
  view every client queues every hit. Room reliable events/s (room.jsonl "second", 30 s bins): s1 460-561, peak 554-561
  at 390-450 s (the resyncs at 439-470 s); U4r s1 350-483 (falling after its onset). Per client events_queued ~318k /
  18,334 ticks = 17.3 a tick; cap 1,024 (EventQueue::PROVISIONAL_CAP, "provisional until U4f measures", events.rs:23)
  = about 59 ticks of backlog.
- N6 the EVIDENCE run brawl1-ev-p3-s1 (m7-ev.log:125) already had "overflow resyncs 27": its summary line equals
  l1-p3-s1's (fate log b99096c62a5c51dd, same counts): deterministic. The card's evidence record says "pass every bar"
  and gives P0's incomplete only; it omits P3's 27 resyncs and P3's incomplete (43,295). Condition 14 asked each run's
  -17 mismatches AND incomplete as facts. The card's "Open, for the advisor" list omits the carry-58 flags too.
- N7 s2 network figures equal U4r's s2 exactly (commands 724,309; steps without 155,292; late 1,136; incomplete 43,210;
  decided lost 62). s1 differs (+178 incomplete, -285 commands), s3 (+13, -31): only the runs with resyncs differ.
- N8 -57 side by side (µm / µdeg): pos max s1 6,952 vs 6,984 (-0.458 %), s2 6,963 vs 6,984 (-0.301 %), s3 6,970 vs
  6,984 (-0.200 %); pos p99 5,111 = 5,111 x3; yaw 2,750 = x3; pitch 88,063 = x3. Verdicts L1 P3 PASS both; mark
  NOT-RUN both; bites pos-4cm, yaw-12bit, pitch-8bit not_run in both sets (P3 carries none). No -57 difference.
- N9 other verdicts: -2 SS runs FAIL (0.352778/0.085444/0.203556) -> PASS (1/1/1); -2 mark FAIL -> NOT-RUN; -17 L1 P3
  INVALID (pre:PM-0001-2) -> PASS (0 mismatches; compared 835,238/835,419/835,088; judge incomplete 44,395/44,253/
  44,535 whole-run); -17 mark INVALID -> NOT-RUN; -17 bites not_run both. All as plan section 9 expects.
