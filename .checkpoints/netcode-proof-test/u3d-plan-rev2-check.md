# U3d plan rev 2: completeness check (test-engineer)

Task: check u3d-plan.md rev 2 against u3d-plan-review-test.md (1-12), u3d-plan-review-rust.md (F1-F10),
the shared-target rule (netcode-proof-test.md "Decisions taken", 2026-11-09), and scouts u3d-scout-req.md, u3d-scout-r1.md.
Plan stage: no edits, builds, runs or network.

## Plan
1. Read checkpoint "Decisions taken" (shared-target rule).
2. Read both critiques; list findings.
3. Read plan rev 2 in full.
4. Per finding: met (quote line) / deferred (reason holds?) / gap.
5. Scouts: list requirements; check each is in rev 2.
6. New inconsistencies (tests that cannot fail, numbers vs source).
7. Verdict + fixes.

## Notes (one line per finding)
- S1 Shared-target rule read: netcode-proof-test.md:404-413 (CARGO_TARGET_DIR=.gate/shared-target; touch own crate sources;
  check each run's test count (--list where it matters); main folder keeps own cache; every workflow brief names folder + 2 steps).
- S2 Critique test: F-A..F-O (numbered 1-12 in brief incl. 12a-c). Critique rust: F1..F10 (+ R0 band vacuity, cite fixes :91).
- S3 Plan rev 2 read in full (513 lines). §0.1 table :49-75 maps all 15 test + 10 rust findings.
- P1 T5 numbers re-derived: p50=100,000; L0=p50 -> 10,000,000/1000=10,000; L0=0 -> 10,100; mean 110,250 -> 9,989; no-allowance 10,250. HOLD.
- P2 T6 numbers re-derived: p001 20,000,000; p90 (900th) 28,990,000; p99.9 (999th) 30,280,000 FAIL; mean 24,995,600; loss 10/1010=0.0099; E 2*50,000/1000=100. Bite due-t_in p99.9 = 29,980,000 PASS. HOLD. Bands d-a, d+0.8a, d+a +-0.05a; mean, loss +-5% match plan §2 :96-98.
- P3 T2 P(0 lost)=0.99^2000=e^-20.1=1.9e-9 HOLDS.
- G-cand-1 (nit) T5 asserts would-lines PASS@10,000 / MISS@10,100: bakes U3D-BAR (a) like T3; plan says only T3 follows the ruling (:304, :449).
- G-cand-2 (low) §5.2 :340-342 rate failure = INVALID vs C-U6-6 :264-265 "another rate is recorded and named with the verdict": contradiction.
- G-cand-3 (low-med) C-U6-5 tests (:261-262) cannot fail for an always-0 E: "warm-up stays out" is negative-only; identity w/ l2_summary near-vacuous at quiet rates (F-B's own point). Needs positive planted-inside case.
- G-cand-4 (nit) F-O deferral reason (:291) answers a verdict bite, not F-O's lower-bound design (over_count >= datagrams due in held window).
- S4 scout-req (C1-C13, R-SRC1-5) all present in rev 2: poller task, P1, steal, Q9, policy, down E, U6/U8 carries, no engine feature. C1's "run window" refined to measured records (W1) by T-5: consistent.
- S5 scout-r1 F5-F25 / S1-S7: all mapped (U8 row, C-U8-3, U3D-POLL/JUDGE/MISS/BAR, zero form, steal raw lines, Q9 shell, CLK_TCK trap). Deferred w/ reason: npt-log names (§3.7 :284). F25/S6 live-rise bite deferred (§3.7 :291).
- G-cand-5 (nit/low) new integer 5-sd loss band (:178-180) has no fixed-input test: could be vacuous, the very trap the plan names for delay_tests band. Also isqrt rounds sd down (isqrt(9,900)=99 -> 495 vs 497.5): say the rounding.
- T4 integer rank vs judge f64 ceil: agree for all n at q in {0.001,0.9,0.999} (rel. error of q < half-ulp): no gap.
- V1 check.mjs:1070-1071 run() = spawnSync w/o env override (inherits CARGO_TARGET_DIR), no --target-dir; :1121-1128 test both builds (no features / --all-features); cargoCounts counts passed (ignored not counted) + "reported a skip". Plan §7.1 :381-383, :393 HOLD.
- V2 checkpoint :524 "260 measured, 274 bites" HOLDS; :661-662 G-METHOD HOLDS; :493-495, :501 Q1/Q2/Q9 HOLD.
- V3 stats.rs FateLog::push (:482-491) returns false at cap, caller counts full -> pre-touch via fates.push bypasses fate_log_full: OK. drain (:493-498) yields canonical bytes; entries() cfg(test). E bound stats.rs:3-8 per-sample 1 us in threshold bucket -> T2's 1,000 ns OK.
- G-cand-6 (low) T2 reads after a timed stop (joins + 530 ms); a lagging release thread (debug, parallel check.mjs) can finish a down datagram between drain_fate_log and run_summary -> identity mismatch = timing flake. Fix: in T2 (outside the run's window rule) wait until down delivered+lost = n (summary_when) and assert accounted-for both dirs before the identities; or read run_summary before and after the drain and require equality.
- V4 marks.json -3 count check :752-756/:881-885 is over second.s.probe_*_lost (one per SENT packet, SERIES.md:117), share where==1 (:763-786). Delay series one per DELIVERED (SERIES.md:116). Bands :763-1003, RTT :1010-1025 HOLD.
- G-cand-7 (low) plan's cross-check "the count against 1,000,000" (:166) does not say which count; T6 hand values give n 1,000 (delays) only. If the builder counts Delivered, 100 s prints ~990,000 "would FAIL" while -3 passes. Fix: count = every fate of the direction (= sent; -3's probe_*_lost count); T6 asserts count 1,010, bite = Delivered count 1,000.
- G-cand-8 (low) R-F1's point was that the SHARED fn makes T1 bite the run's invocation; plan :126 puts rt.spawn at each call site ("The test thread calls rt.spawn(poller(..))"), so a run written with rt.block_on(poller) passes T1. Fix: the shared helper does the spawn (e.g. start_poller(&Handle, ..) -> JoinHandle), used by run, T1, T2.
- V5 check.mjs deps step env scan (:931-935) does not reject CARGO_TARGET_DIR (only *RUSTFLAGS, CARGO_ALIAS_*, COMPILER_ENV, CARGO_TARGET_*_RUNNER): §7.1 safe. No target-dir check in check.mjs.
- V6 u3-verify-timing.md:152-159 (19 us / 10 us hop, 61/40 ms pauses) HOLDS for plan §0 :40-42.
- Per-finding status: T-1..T-12, T-12a, T-12b MET (plan :51-64, :302-307, :155-182, :186-191, :217-223, :238-277, :293);
  T-12c DEFERRED, reason partly mismatched (G-cand-4); R-F1 MET in text, proof gap (G-cand-8); R-F2..R-F10 MET
  (:124-184, :241-270, :411-419); R0 band vacuity MET (:178-180). T-7's C-U6-5 proof weak (G-cand-3).
  Shared-target rule MET (§7.1 :376-405) except the "every brief names it" clause is implicit (nit-9).
- nit-9 §7 :362/:369 say "under §7.1"; checkpoint :412-413 wants every workflow brief to name the folder and the two steps.
- nit-10 plan :96 "(:752-1021)" — RTT high check runs to :1021-1025.
- VERDICT: GAPS (no medium/structural; 2 low-medium, 3 low, 4 nits). Fixes are text-only in §3.1, §3.6, §3.7, §4, §7.
DONE 2026-11-09
