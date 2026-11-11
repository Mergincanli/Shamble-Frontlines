Task: npt-u3d-skeptic (night item 5), test-engineer angle: the seven fast tests of U3d (1ccff06) and their bites.

## Plan (blocks, in order)
- B1: read u3d-plan.md (the seven tests, bites, U3D-BAR), advisor-u3d.md (plan verdict, conditions, close ruling :146-250, U3D-RCVBUF :251-271), plan.md R-31 (~:1403) and U8 row (~:1227).
- B2: read socket_tests.rs at 1ccff06 (the diff d012437..1ccff06), the seven tests and the self-test helpers.
- B3: read ImpairSocket (the code under test) enough to construct defects.
- B4: read bite records (u3d-bites.py, outputs, u3d-test-bites*, fix-bites outputs, fix-review-bites*.py, u3d-build.md) and check against code.
- B5: read prior reviews (build reviews x3, fix review, plan review, rev2 check) to avoid re-reporting.
- B6: run.sh and SERIES.md rows, selftest record 78b0c07.
- B7: per-test verdicts, defects that pass all seven, self-refutation; summary; DONE.

## Notes
### B1 done (sources read)
- u3d-plan.md §4 (:334-351): T1 task poller lateness (bites: std-thread poller, block_on poller, planted in start_poller); T2 fate log vs histogram (bites: L = out - t_in; Lost not filtered); T3 verdict line <= per direction (bites: <, pooled mean); T4 nearest rank + loss_band (bites: floor rank, (0,n) band, sd rounded up/f64); T5 exact E on fixed samples (999 x 100,000 + 1 x 10,350,000; E p50 = 10,000, E0 = 10,100; bites: L0 swapped, /over, mean L0, no allowance); T6 cross-check (1,000 Delivered D_i = 20ms + 10us*i, L 300us at i=998,999; 10 Lost; p99.9 30,280,000 FAIL; count 1,010; E 100; bands pinned; bites: due - t_in, lost kept, band rule, count over Delivered); T7 held lock both directions (fixed(100), 2,000/s, lock 50 ms at ~400 ms; B >= 20, each B late >= m; over >= A; bites: down stamp = due, up stamp = due, summarize skipping log buckets).
- §3.1 asserts only run conditions: rate > 9,900/s, no queue/pool/oversize/shaper drop, every datagram accounted for, no send error/would-block, poller count = delivered, fate_log_full 0, loss in loss_band, slack start 1 (0 under RT).
- advisor close ruling (:146-250): R-31 corrected: E (L0=0) predicts p99.9 exactly only while no L > 2a + 0.25 ms = 10.25 ms; past that E overstates (can MISS where -3 passes, never reverse). Round 3 INVALID: 87 kernel drops (RcvbufErrors), caught only by accounting assert at socket_tests.rs:1657. Cond 1(f): "no host drops" asserts are ImpairSocket's own; kernel drop caught only by the accounting assert. Cond 1(h): a stall of ImpairSocket's own reader delays t_in, never enters L/E/-3 delays.
- U3D-RCVBUF (:251-271): C-U8-6: probe socket drops column + snmp around every L2 leg; P1 drop -> INVALID, never re-run.
- plan.md:1423 R-31 amended as condition 2 (text present). plan.md:1247 U8 row has C-U8-1..6, slack rule citing socket_tests.rs:1480-1486.

### B2 done (socket_tests.rs at 1ccff06 = df8975b for this file; only the selftest record changed after)
- The seven: T1 `the_task_poller_takes_lateness_where_the_task_runs` :1002-1052; T2 `the_fate_log_and_the_histogram_agree` :1059-1109; T3 `the_verdict_line_is_per_direction_and_inclusive` :1113-1119; T4 `nearest_rank_and_loss_band_on_fixed_inputs` :1124-1140; T5 `the_exact_excess_on_fixed_samples` :1146-1163; T6 `the_cross_check_reads_delay_plus_lateness` :1175-1261; T7 `a_held_lock_makes_both_directions_late` :1272-1339. Self-test `l2_timing_self_test` :1350-1694 (#[ignore]).
- Helpers: start_poller :676-712; in_e_set :715; samples :735-756; nearest_rank :759; quantile :767; excess :775; forms :793; would_one :804; loss_band :816; bands :836; cross_check :870-897; agree :904; drive :917-989; fate_entries :992.
- Self-test wiring: sm = [samples(UP), samples(DOWN)] indexed by d (UP = 0, DOWN = 1: impair.rs:55-56, consistent); judged = f[d].at_zero; asserts :1637-1693. Sink count got_down printed only (:1497), never asserted (not E-relevant).
- T2's E identity uses excess(&sm.late, histogram p50), not forms(); forms().at_zero is pinned only by the pure T5/T6.

### B3 done (code under test: impair.rs, socket.rs, stats.rs, npt-l1)
- impair.rs:121-124 delay = d - a + below(word, 2a+1); :558-572 reorder step (1 %: npt-l1 lib.rs:83): an overtaken held datagram gets due = next's due + 1 (delay up to inter-arrival + d + a, about 30.1 ms at 10,000/s, under T = 30.25 ms; it lowers the effective allowance for about 1e-4 of datagrams: about 1e-6 x n extra above T: negligible, not a finding); :655-697 finish: histogram and fate log from the same out_ns and e.due_ns.
- socket.rs:463-466 try_send reads now before the lock; :500-503 poll_recv locks, then reads now; :606-608 reader stamps t after recv, before the lock; :678 release reads now under the lock and pops; :711-714 drops the lock, wakes the poller first; :724-726 stamps out after each try_send; :729-735 finishes under the lock.
- stats.rs:98-159 summarize: E exact for buckets wholly above c, the threshold bucket at its mean (worst-case error 1 us; typical about 1 ns).
- Structural: E reads out - due, -3's L2 delay out - t_in, from the same fate entry; due - t_in = D. A stamp defect moves E and -3 together. Divergences: (i) the 2a cap (advisor, ruled); (ii) the sampling variance of D (TE1); (iii) the reorder step's moved dues (negligible).

### B4 done (bite records vs code)
- u3d-bites.py (22 bites, build state socket_tests.rs 31294953...): all BITES (u3d-bites-output.txt:5-72), each for its own reason; names and messages match the code at 1ccff06 (T1 :1047-1051, T2 :1088-1092 / :1104-1107, T7 :1322-1336); line numbers shifted by the fix pass (expected). Restored copy 18 passed.
- The fix pass (final ca17a9f6...) changed samples() (early count), T6 (down early fate, two agree asserts, the 106/10,106 loss case), the self-test's asserts; its bites on the final state (u3d-fix-review-bites-output.txt, u3d-fix-bites-unit-output.txt) are consistent with the code (:1688 slack, :1642 touched, :1647 fate log full, :1667 early, :1241 agree, :1256 loss, :1244 early count).
- T6a's anchor (u3d-bites.py:81, `let d = f.out_ns.checked_sub(f.t_in_ns);`) no longer exists at 1ccff06 (now `match ...` :747): T6a was bitten on the pre-fix file only; by reading it still bites (p999 29,980,000 != 30,280,000). Confirmed by reading, not a finding.
- Self-test run conditions bitten by the test reviewer (S1-S8) and the fix review (F1-F6); the accounting assert by S2 (29,756 vs 30,000) and organically in round 3.

### B5 done (prior reviews)
- Build reviews (test, netcode, rust), fix review (no gap), flake look (T1+T2+T7 x30, suite x8: 0 failed). Raised and settled: slack under RT, loss band order, touched, NotUnicode, agree/loss unbitten (X1/X2, fixed), samples panic, run.sh touch, drive's wait unexercised, CPU dilution, recv_buf first touch, kernel-buffer margin in T7 (advisor nit 6(b)).
- Not raised in any u3d-*.md: TE1, TE2, TE3; TE4 is the residual of X1; TE5/TE6 not raised.

### B6 done (run.sh, SERIES.md, the record)
- run.sh: set -euo pipefail; build once; one npt_net test executable from cargo's JSON; `--ignored --list` must name both runs (so `--exact` cannot silently run 0 tests); each run's status kept, INVALID kept; header facts read or "unreadable"; steal from fields 2..9 (steal = $9) correct; refuses CARGO_TARGET_DIR unless U3D_CHECK. No ignored failing step found. No /proc/net/snmp or per-socket drops read around runs (TE7). No run log outside the record (re-runs are excluded only by git history).
- Record 78b0c07: header commit f50bd54, binary sha c236b449... = fix review's clean build; round 3 INVALID at :1657 (up 999,913 + 87 = 1,000,000); round 3 down printed "disagree" x3 for (MISS, PASS) (:253-255), so the code's agree() is right today (TE4 is a test gap only); round 1 fate-log E at the histogram p50 equals the histogram's E exactly (16,611 = 16,611; 11,303 = 11,303), so T2's 1,000 ns tolerance is about 1,000x the real error (TE6).
- SERIES.md (:396-397, :427, :452): text equals plan §3.5 and the advisor's condition 2. The down rows (:446-453) say L is taken "at try_send's return": the claim TE2 shows no test pins.

### B7 done: findings
Severity scale as CLAUDE.md. Paths: socket_tests.rs, socket.rs = spikes/netcode-proof/crates/net/src/l2/ at df8975b (= 1ccff06 for these files).

**TE1 (Medium). R-31's "never the reverse" holds for the expected count, not for one leg: E can PASS while -3's p99.9 FAILs.**
- Where: plan.md:1423 (R-31: "predicts -3's P1 p99.9 exactly while no lateness exceeds 2a + 0.25 ms ... so it can MISS where -3 passes, never the reverse"); advisor-u3d.md:214-217 ("E is still never more lenient than -3"); u3d-report.md:69 and :99-100; the derivation u3-verify-timing.md:46-48 is an expectation ("P(U + L' > d + 1.05a) = E[min(1, (L' - 0.05a)+ / 2a)]").
- Scenario: given the lateness samples, the count of delays above T = d + 1.05a is a sum of independent Bernoulli(p_i) over the delay draws D_i (impair.rs:121-124: uniform on 2a + 1 grid values), p_i = min(1, (L_i - 0.25 ms)+ / 2a), so its mean is at most n x E / 2a (equal without the cap). -3's p99.9 passes iff at most n - ceil(0.999 n) values lie above T: 990 at round 1's n = 990,012. At E = 10,000 ns (PASS: the bar is inclusive, socket_tests.rs:804-806) the mean is 990.0: -3's p99.9 FAILs about half the time. Below the bar the odds fall with the count's spread (at most sqrt(990), about 31, i.e. 3 % of the bar; less when long pauses dominate, since p_i = 1 entries carry no variance), so the band is roughly E in [9,000-9,700, 10,000] ns. By the same noise E can MISS where -3 passes with every lateness under 10.25 ms, which the close ruling's reading ("a MISS where -3 passes and max > 10.25 ms is this case") does not cover.
- What goes wrong: at R1, a P1 leg with E just under the bar and -3's p99.9 FAIL reads, by R-31, as impossible, so a session concludes the machine's lateness is cleared and hunts the impairer or the probe; U3D-MISS has no route for "E PASS, -3 FAIL". The bar and -3's own verdict are unaffected (npt-judge still judges -3), which is why this is Medium, not High.
- Refutation attempt (for my own claim): (1) Is "never the reverse" meant only in expectation? No: the ruling applies it to realized R1 outcomes (advisor-u3d.md:216-217). (2) Does the fixed seed remove the noise? No: the D_i are fixed per index, but which indices fall in a pause is random run to run. (3) Is the variance negligible? Not at the bar: there the mean equals the allowance, so about 50 % whatever the spread. (4) Does the "+0.01 ms" of u3-verify-timing.md:48 absorb it? No: the pass test is a count above T; the uniform's own p99.9 does not enter it. (5) Already raised? Grep of every u3d-*.md for variance/sampling: none. Holds.
- Cheapest proof (no run needed, arithmetic): the expected count n x E / (2a + 1) = 990,012 x 10,000 / 10,000,001 = 990.0 against the allowance 990. Optional by a run (day): a pure test on fixed lateness (for example 1,000,000 samples, 10,000 of them at L = 1.25 ms and the rest 0, so E = 10,000 ns), drawing D with ChaCha8 over 1,000 seeds through `cross_check`, counting p99.9 FAILs; expect about 50 %. Fix: R-31 and the report say "in expectation over the delay draws; within about 3 % of the bar either side, either can disagree by chance", and U8's miss path gains the "E PASS, -3 FAIL" case.

**TE2 (Medium). Down's release stamp moved to the pop time passes all seven tests.**
- Where: socket.rs:726 (`results.push((kind, errno, sh.now_ns()))`); claimed proven by T7's doc (socket_tests.rs:1263-1264, "down after try_send: :724-726"), plan §4 T7, and SERIES.md's down rows ("at try_send's return").
- Scenario: replace `sh.now_ns()` at :726 with `now` (the pop time read under the lock at :678, in scope). T7 still passes: B's entries are popped only after t1, so out >= t1 either way; T1 and T2 never look at down's stamp point; the older outbound_datagrams_leave_at_their_due_time (:216-236) checks wall time and counts. Down's E would then miss the poller wake that precedes every batch (:712-714, R-F6) and each send syscall, preemption during the batch included: E lenient, and -3's L2 down delays with it (same stamp), so the self-test could not see it.
- Proof: needs a run (day): plant the change, then `cargo +1.99.1 test --locked -p npt-net --lib -- l2::socket::tests::` (expect every test to pass). A deterministic test that would bite: register a waker whose `wake()` sleeps 50 ms (one poll_recv with it, Pending); hold the state lock (T7's way) across one up and one down datagram's due, so the release thread pops both in one pass; then down's L must be at least (t1 - due) + 45 ms; at the pop time it is about t1 - due.
- Not carried.

**TE3 (Low). Up's clock read moved before poll_recv's lock passes all seven.**
- Where: socket.rs:501-503 (lock, then now). Swap them: T1 has no contention (same value); in T7 the poller runs only after the release thread's wake after t1, so its now is >= t1 either way. Up's E would drop the lock waits (a preempted lock holder: reader, pacer, release thread, at U6 the drain). The netcode-architect owns the clocks; recorded as a test blind spot only.
- Proof: needs a run (day): plant the swap, run the line above (expect all pass). A bite: put an up datagram in `ready`, take the state lock, start poll_recv on another thread, hold 50 ms, release: L must be >= 50 ms.

**TE4 (Low). agree()'s (E MISS, -3 PASS) case, the cap case the close ruling found, is unpinned.**
- Where: socket_tests.rs:1240-1242 pin (PASS, FAIL) -> disagree, (MISS, FAIL) -> agree, (PASS, PASS) -> agree. The defect `if p999_pass || e_would == "MISS" { "agree" } else { "disagree" }` gives all three and prints "agree" for (MISS, PASS), hiding round 3's disagreement. The code at :904-910 is right, and the record printed "disagree" (selftest-20261109T035556Z.txt:253-255). The residual of the fixed X1.
- Proof: needs a run (day): plant it at :905, run `cargo +1.99.1 test --locked -p npt-net --lib -- l2::socket::tests::the_cross_check_reads_delay_plus_lateness --exact` (expect a pass). Fix: `assert_eq!(agree("MISS", true), "disagree")`.

**TE5 (Low). Several cross-check verdict words are pinned on one side only.**
- Where: T6 :1235 pins pass[0] (count) false only, at 1,010; a pass[0] computed over n (Delivered) also reads false there but would print FAIL at a full 1,000,000 run. mean, p001 and p90 are pinned true only, loss true only (the 106/10,106 case), and p99.9 false only. forms()'s p50 rank is unpinned (T5's 999 equal values and T6's 998 zeros give the same p50 at any rank under 999). All are report-only lines; the code is right (:889 uses s.fates).
- Proof: by reading. Fix: one full-count case (1,000,000 fates, 990,000 Delivered: pass[0] true) and one FAIL value per band.

**TE6 (Low). T2's 1,000 ns tolerance is about 1,000x the real error.**
- Where: socket_tests.rs:1104-1107. stats.rs:3-8 gives the worst case (every sample in the threshold bucket); the record shows the fate-log E equal to the histogram's to the ns (selftest:34-35, :228-229). At T2's 2,000 samples, any disagreement summing to under 2 ms of total excess passes, and E is often 0 there (the plan's T-2). T7 and T1 carry the real weight; this is a weak identity, not a vacuous test (its count asserts bite: T2b).
- Fix: tolerance = 1 + 1,000 x (fate-log samples in c's bucket) / n, computable from the fate log.

**TE7 (Low). run.sh records no UDP drop counters around its runs.**
- Where: run.sh:130-155 reads /proc/stat and uptime only. Round 3's cause (87 RcvbufErrors) came from reads outside the record (u3d-report.md:86-89; the advisor's cond 1(g)). A re-run of this script would again leave an accounting INVALID without its cause in the record. R1 is carried (C-U8-6); the dry-run script is not. Also no run log outside the record: a re-run kept instead of an earlier one is excluded only by git history. Report-only here, so Low.
- Proof: by reading (`grep -n snmp spikes/netcode-proof/results/u3d-timing/run.sh` finds nothing).

## Per test
- T1 `the_task_poller_takes_lateness_where_the_task_runs` (:1002-1052): HOLDS. The worker is held until first_seen + 110 ms, so L >= 90 ms by construction; both bites in start_poller fail (0.12 and 0.09 ms); no vacuous path (2 s give-ups fail loudly).
- T2 `the_fate_log_and_the_histogram_agree` (:1059-1109): HOLDS for counts; its E identity is weak (TE6).
- T3 `the_verdict_line_is_per_direction_and_inclusive` (:1113-1119): HOLDS.
- T4 `nearest_rank_and_loss_band_on_fixed_inputs` (:1124-1140): HOLDS.
- T5 `the_exact_excess_on_fixed_samples` (:1146-1163): HOLDS for the judged form; the p50 rank is unpinned (TE5).
- T6 `the_cross_check_reads_delay_plus_lateness` (:1175-1261): HOLDS for the delay formula, filter, count and bands; one-sided verdict words (TE4, TE5).
- T7 `a_held_lock_makes_both_directions_late` (:1272-1339): HOLDS for "a stall enters L and over in both directions"; it does not prove where down is stamped (TE2) or that up is stamped after the lock (TE3).

## Defects that pass all seven (constructed)
1. socket.rs:726 `sh.now_ns()` -> `now`: down's E (and -3's down delays) lose the poller wake and the sends (TE2).
2. socket.rs:501 and :503 swapped: up's E loses lock waits (TE3).
3. agree() as `p999_pass || e_would == "MISS"`: the (MISS, PASS) disagreement prints "agree" (TE4).
4. No code defect needed: E PASS with -3 FAIL by the delay draws' noise near the bar (TE1).
- None found that hides a P1 kernel drop from the self-test: drops never reach intake, so delivered + lost and the fate log's count both fall short of n (:1657, :1662); S2 and round 3 show the assert bites. At R1 that trace is C-U8-6's /proc reads (carried, not built).

## Summary
- Counts: Critical 0, High 0, Medium 2 (TE1, TE2), Low 5 (TE3-TE7).
- Top: TE1 (R-31's "never the reverse" is true in expectation only; at E = 10,000 ns -3's p99.9 fails about half the time); TE2 (down's stamp point unproven; one-word plant passes all seven).
- Confirmed: every recorded bite matches the code at 1ccff06 and fails for its own reason (22 build bites, the fix review's 16 plus 5); the self-test's run conditions bite (S1-S8, F1-F6); the accounting assert is the self-test's kernel-drop trace and fired correctly in round 3; run.sh has no ignored failing step, and `--exact` cannot run 0 tests because of the list check; SERIES.md's rows equal the plan and condition 2; UP/DOWN indexing in the self-test is consistent; no timing-dependent assert beyond the noted kernel-buffer margin (advisor nit 6(b)); flake look 30x clean (reviewer's).
- Needs a run (day): TE2 and TE3 plants with `cargo +1.99.1 test --locked -p npt-net --lib -- l2::socket::tests::`; TE4 plant with the T6 line above; TE1's optional seed-sweep test.
DONE
