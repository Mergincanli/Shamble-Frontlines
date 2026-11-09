# U3d plan, rev 2: the timing self-test (how late ImpairSocket hands datagrams over at 20,000 releases per second)

**Revision 2, 2026-11-09** (netcode-architect, on head dacdb77; code as at 768d00b). It folds in the test-engineer's
review (REVISE: u3d-plan-review-test.md, findings 1-12 = F-A to F-L, plus the nits F-M to F-O) and the rust-reviewer's
(APPROVE WITH FINDINGS: u3d-plan-review-rust.md, F1-F10). I checked each finding against the code myself (notes
u3d-plan-rev2-notes.md M0-M17). It also applies Mustafa's shared-target rule of 2026-11-09 (§7.1). What changed from
rev 1:
- §0.1 maps every finding to where rev 2 meets it.
- The tests: T1's order fixed; two new pure tests (T5 exact E, T6 -3's cross-check); T4 in integers.
- The run: the poller is spawned onto a runtime the test builds; the fate log is pre-touched; a window rule (no observer
  call inside the run); M13's run is not edited.
- R1 judges E as computed by the probe over -3's own measured records (C-U6-5), which also gives the L0 = 0 form a
  producer. So U3D-L0's recommendation moves to (b), and a new question, U3D-WINDOW, asks which value R1 reads.
- The build rules: §7.1.

Rev 1 was written on 49bd1ac (scouts u3d-scout-req.md, u3d-scout-r1.md; notes u3d-plan-notes.md N1-N25). Next: the
completeness check (u3d-plan-rev2-check.md), then the advisor. **No code before the advisor's APPROVE.**

## 0. In plain words
- **What it is.** L2's impairment layer (ImpairSocket) delays every datagram by the profile's amount and hands it over
  when it falls due. "Lateness" is how much later than that the hand-over really happens. If the hand-over is late too
  often, PM-0001-3's tightest delay check on L2 (P1's 99.9th percentile within 0.25 ms) fails for reasons that are our
  machine's, not the profile's. U3-Q1 (c), which Mustafa agreed to, sets the bar: the expected lateness beyond
  0.25 ms stays under about 10 µs at the tightest profile (P1), at 20,000 releases per second, on RS.
- **What U3d does.** It makes the self-test measure exactly that, here: at P1, 10,000 datagrams per second each way,
  with the inbound hand-over polled by a task spawned on a tokio runtime, the way quinn polls. Today's run uses a parked
  thread and misses that hop. The run prints:
  - the percentiles and the statistic in both forms (from the median and from zero);
  - the CPU, the timer slack and the scheduling policy;
  - from the same samples, PM-0001-3's own L2 P1 delay checks, so the predictor and the check it predicts sit side by
    side.

  A script runs it three times in a quiet container. It records the kernel facts U3-Q9 asks for, the huge-page mode,
  and the steal on every CPU line before and after each run. Everything here is reported, never judged: this container
  is no evidence for a µs-level bar (advisor-u3c-plan.md:114). RS judges at R1, on the probe's P1 leg (§5.2).
- **Size.** Test code only: in one test file, one new ignored run, six fast tests and their helpers. Plus a results
  script, two SERIES.md row texts and plan text. No production code, no public item, no log name, no dependency. M13's
  run is not edited.
- **What it expects here.** A miss. ImpairSocket's own E was 11.6 and 16.9 µs inbound, and 12.9 µs outbound in run 2,
  at P2 with the thread poller (results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt:62, :120-121). The tokio hop
  added about 10-19 µs of E in the verifier's two single runs, each hit by a 40-61 ms pause (u3-verify-timing.md:152-159).
  So that number is context, not a measurement of the hop. The hop also reaches the outbound side: the release thread
  wakes the poller before it sends its batch (socket.rs:713, :724). A miss is reported to the advisor, not a stop
  (advisor-u3c-plan.md:163-165: "U3d reports it, RS judges it").

## 0.1 The reviews' findings and where rev 2 meets them
Severity as each critic gave it. "Holds" means I re-checked the claim at the lines given and it is right.

| # | Finding (severity) | Checked | Rev 2 |
|---|---|---|---|
| T-1 (F-A) | T1 races: the blocker starts only after `first_seen`; a late start lets the poller run at due (medium) | Holds: the old test polls only at first + 110 ms (socket_tests.rs:184-211); a stored waker is woken at due (socket.rs:713) | §4 T1: the blocker runs first, signals, then holds the only worker until `first_seen` + 110 ms: the lower bound by construction |
| T-2 (F-B) | T2 near-vacuous: both E's are often 0 at 2,000/s for 1 s (medium) | Holds: E is 0 with no sample above p50 + 0.25 ms (stats.rs:129-155) | §4 T5: exact E on fixed samples, both L0 forms, PASS/MISS either side of the bar; T2 also asserts lost > 0 and no send error |
| T-3 (F-C) | -3's cross-check delay formula has no bite (medium) | Holds: rev 1 tests only the rank (T4) | §4 T6: fixed fates with known D and L; bites `due − t_in` and lost entries kept; bands derived from P1 and pinned to marks.json's numbers |
| T-4 (F-D) | No producer for the L0 = 0 E at R1 (medium) | Holds: one E field (stats.rs:58-59). The probe must read the fate log anyway: it is the only per-datagram stamp source (socket.rs:427), and -3's L2 series need those stamps (SERIES.md:401-406) | C-U6-5: the probe computes both forms from those entries; §5.2 |
| T-5 (F-E) | The judged window is not -3's: -3 reads `measured` records only, `late_run` covers the socket's life (medium) | Holds: no `window` key in -3's checks (marks.json:752-1021), so `measured` (marks/README.md:100); `late_run` never resets (impair.rs:246, :682) | C-U6-5 computes E over -3's measured records; C-U6-2 one socket per leg; U3D-WINDOW |
| T-6 (F-F) | R1 lacks a rate condition; the probe's up stream may carry echoes too (low-medium) | Holds: "sends up, echoes" (SERIES.md:405-406); the count (marks.json:752) does not bound the rate | §5.2 run conditions gain the rate; C-U6-6 (composition and poller kind recorded); C-U8-4 |
| T-7 (F-G) | C-U6-1 and C-U6-2 carry designs without proofs (low) | Holds | C-U6-1: T1 re-run on the probe's poller; C-U6-2: a two-legs test; C-U6-5: a warm-up-outside-the-window test |
| T-8 (F-H) | The dry run's comparisons are confounded (profile, poller, length, fate log); "10-19 µs" rests on two single runs (low) | Holds (u3-verify-timing.md:152-159) | §0, §5.1 and U3D-POLL say what the rounds can show. A P1 thread-poller control round is offered as U3D-POLL (b+), not built by default. Fate log off: not needed after the pre-touch (§3.7) |
| T-9 (F-I) | Steal from `head -1` only; prints after asserts; INVALID rounds (low) | Holds (M13 prints first: socket_tests.rs:579-614, then asserts :616-637) | §3.4 every `^cpu` line; §3.1 every print before any assert; an INVALID round stays in the record with its cause |
| T-10 (F-J) | Folding M13 into a shared helper risks the c10 comparison (low) | Holds | §3.2: M13's function is not edited; the self-test is its own function, reusing `pace`, `transmit`, `value`, `local`, `Unpark` |
| T-11 (F-K) | C-U6-3's "drained only at the leg's end" clashes with -3's per-second records and is unquantified (low) | Holds: the probe writes per-second `second.s.probe_*` (SERIES.md:401-406) | C-U6-3: U6 chooses the cadence; the closure only copies; the lock hold is measured; the Σ(G − 0.25 ms)²/(2T) budget stated |
| T-12 (F-L) | Citations: U3 row is plan.md:1222; quinn's loop is `poll_socket`; buffers ~64 KB; checkpoint lines moved (nit) | Holds (plan.md:1221 is U2) | Fixed throughout: U3 :1222; drive_recv endpoint.rs:497-512 calls `poll_socket` :772ff (loop :793); 1,472 B (rust F8); checkpoint :488-501 |
| T-12a (F-M) | T2's count identity needs no SendError/WouldBlock (they enter the histogram) (nit) | Holds (impair.rs:680-684) | §4 T2 asserts send errors and would-blocks 0; the sample sets as rust F5 |
| T-12b (F-N) | T3 bakes "≤ 10,000" from U3D-BAR (a) (nit) | Holds | T3's constant follows the advisor's U3D-BAR ruling |
| T-12c (F-O) | Optional deterministic stall test (hold the lock 50 ms once) (nit) | The critic calls rev 1's reasoning otherwise sound | Deferred: §3.7 gives the reason |
| R-F1 | The precedent `#[tokio::test]` runs its body under `block_on` on the test thread, a park/unpark hop, not quinn's (medium) | Holds: tokio-1.54.1 multi_thread/mod.rs:80-91; delay_tests.rs:3084; quinn spawns its driver (endpoint.rs:152-160) | §3.1: `rt.spawn` onto `Builder::new_multi_thread().worker_threads(4).enable_all()`; one poller fn shared by T1 and the run; T1 bites a `block_on` poller too |
| R-F2 | Fate-log pages are first touched under the state lock (medium) | Holds: `Vec::with_capacity` at bind (stats.rs:476-480); pushes under the lock from `finish` and intake drops (impair.rs:338-343, :500-506, :576-579, :686-697) | §3.1 pre-touch before the pacers (test-only: the child module sees `shared`, `im` and `fates`: socket.rs:63-64, :739-741; impair.rs:248). `Vec::drain` keeps the pages (stats.rs:495-500). THP mode in the header; risk 2 reworded ("a VM guest"); C-U6-3 |
| R-F3 | Observer calls inside the window take the state lock, and the release thread reads /proc under it (low) | Holds: socket.rs:667-677 (slack ask), :394-415, :262-264, :427-429 | §3.1 window rule: inside the run only `try_send` and `poll_recv`; the stop from the pacers' joins, never by polling `run_summary`; every read after the stop; C-U6-7 |
| R-F4 | T1's race (low) | = T-1 | §4 T1 takes the test-engineer's form: rust's 200 ms sleep form holds only if the drain comes within 130 ms of the signal |
| R-F5 | The fate-log forms must select the histogram's sample set; `0 − t_in` panics in debug (low) | Holds (impair.rs:680-684; stats.rs:437-440) | §3.1/§4: E over finished fates but Oversize; -3's delays from Delivered only, `checked_sub` after filtering |
| R-F6 | The hop is not inbound-only: the release thread wakes the poller before its sends (low) | Holds: socket.rs:713 then :724-726 | §0, §5.1, risk 3: down compared only within the self-test's rounds |
| R-F7 | A run-window reset is a new public method (low) | Holds: plan.md:580-584 | C-U6-2: one ImpairSocket per leg; no reset method |
| R-F8 | Cites: stamp :503/:527; drive_recv :497-512; 1,472 B buffers (nit) | Holds (quinn-proto-0.11.19 config/mod.rs:60; ImpairSocket's 1 segment: socket.rs:549-551) | Fixed |
| R-F9 | The stop branch's order: a `Notify` polled while waiting locks its waiter list (nit) | Holds: tokio sync/notify.rs:1224-1247 | §3.1: `poll_recv` first, the stop polled only after Pending |
| R-F10 | Bounds and integers: cap the length; integer loss band; u128 sums; no HashMap (nit) | Holds (Cargo.toml lints: `iter_over_hash_type` deny) | §3.1: length 1-600 s or INVALID; `u64::isqrt` band; u128 sums; per-direction arrays |

## 1. Requirements
| # | Requirement | Source | Exists | Gap U3d closes |
|---|---|---|---|---|
| RQ1 | The statistic, per direction: E[(L − L0 − 0.25 ms)+] beside p50, p99, p99.9, max | u3-build-plan.md:45-46; M1 advisor-u3q.md:233-236 | stats.rs:20-21, :51-62, :108-159; samples impair.rs:680-684; run window socket.rs:394-415 | Both L0 forms printed. The judged window named: -3's measured records at R1 (C-U6-5), never a second's (each second's L0 is its own p50: stats.rs:129-130), never the socket's life |
| RQ2 | Inbound lateness "where quinn polls"; outbound at try_send's return | M1 (advisor-u3q.md:233-236); u3-build-plan.md:45 | Stamped inside poll_recv after its lock (socket.rs:501, :503, finish :527) and after try_send (:724-726); tested socket_tests.rs:184-237 | The run's poller is a parked std thread (socket_tests.rs:55-88, :561-571). quinn polls from a spawned task (quinn-0.11.12 endpoint.rs:152-160), whose `drive_recv` (:497-512) calls `poll_socket` (:772ff) with 32 buffers (quinn-udp-0.5.16 unix.rs:804-806) of 1,472 B (endpoint.rs:754-759; quinn-proto-0.11.19 config/mod.rs:60) |
| RQ3 | 10,000 per second each way sustained, jitter, loss and the reorder step on; the release thread's CPU recorded | M13 advisor-u3q.md:267-269 | socket_tests.rs:507-639; 0.393 and 0.414 core (c10 :64, :122) against M13's 0.27 | None: reported beside 0.27 core |
| RQ4 | "At the tightest profile", at 20,000 releases per second | the session's record of Mustafa's U3-Q1 (c), .checkpoints/netcode-proof-test.md:493-494 (his words :488-489 carry no number); the bar's derivation u3-verify-timing.md:42-48 | The run is fixed to P2 (socket_tests.rs:519) and checks P2's 2 % loss (:624-627) | Run at P1 (npt-l1 lib.rs:118-121: 50 ms, ±10 ms, 1 %; the reorder step on :110); the loss band from the profile |
| RQ5 | Percentiles reported; "here it is reported only (RS judges at R1)" | u3-build-plan.md:45-46 | Printed only (socket_tests.rs:579-614) | None |
| RQ6 | 1 ns slack through /proc, read back and tested; real-time priority only a switch applied from outside; no spin | M14 advisor-u3q.md:270-273; U3-Q2 (b), checkpoint :494-495 | Done in U3b: thread.rs:20-90; socket.rs:648-676, :312-366; tests thread.rs:99-140, socket_tests.rs:353-384; `release_thread_id` pub (socket.rs:268) | The run prints the slack (:611-613) but not the policy: one line, read after the stop (R-F3) |
| RQ7 | Steal from /proc/stat before and after each ImpairSocket and DelayStream self-test run, nothing in the measured path | advisor-u3c-plan.md:191-192 (UP-STAMP item 5); plan.md:1227 | Neither run reads it (M13 none; delay_tests.rs:3091-3094 loadavg only) | Every `^cpu` line around each run in the dry run's script; R1's in the run plan (C-U8-2) |
| RQ8 | U3-Q9: RS's kernel, CONFIG_HZ, cpuidle driver and clocksource read at R1 (R1's run plan) | checkpoint :501; asked at u3-scout-summary.md:81 | Kernel only (npt-log `Machine.kernel`, crates/log/src/lib.rs:35) | A shell step in the dry run's script now (plus the THP mode: R-F2) and in R1's run plan (C-U8-1) |
| RQ9 | ImpairSocket's E over about 10 µs here: "U3d reports it, RS judges it" | advisor-u3c-plan.md:163-165 | c10: up 11.610 / 16.921 µs, down 2.672 / 12.939 µs (:62-63, :120-121) | Report both directions (run 2's down is over too, which the ruling does not name) |
| RQ10 | RS judges ImpairSocket at R1 | u3-build-plan.md:46 | No binary runs it on RS. -3's L2 probe (npt-server's probe mode, U6: SERIES.md:401-406; ADR 0001:1590) runs the same load on RS at R1 (plan.md:695, :790) | Plan text: where, which value, window, bar, conditions, who judges, the miss path. The U8 row names ImpairSocket only in the steal clause (plan.md:1227); the U3 row omits U3d (:1222) |
| RQ11 | From U3d on, every copy builds in one shared Cargo target folder | Mustafa, 2026-11-09 (checkpoint :404-405), as applied (:406-413) | The spike's config puts the cache inside each copy (spikes/netcode-proof/.cargo/config.toml: `target-dir = "../../.gate/netcode-proof-test/target"`) | §7.1 |

## 2. Rulings and facts this plan rests on
- **The bar's origin** (u3-verify-timing.md:42-48): with the delay uniform on d ± a and an independent lateness L′,
  PM-0001-3's p99.9 upper check (d + 1.05a) holds iff E[(L′ − 0.25 ms)+] ≤ 0.001 × 2a. At P1, a = 5 ms per direction
  (half of the ±10 ms jitter: npt-l1 lib.rs:156-158; impair.rs:105, :119-123), so 2a = 10 ms and the budget is
  **10,000 ns**. P2's would be about 20 µs: P1 binds. The bands: marks.json PM-0001-3 P1 checks (:752-1021: count
  ≥ 1,000,000; loss share [0.0095, 0.0105]; mean [23.75, 26.25] ms; p001 [19.75, 20.25]; p999 [29.75, 30.25]; p90
  [28.75, 29.25]; RTT mean [47.5, 52.5]; parts "L2 P1", process `probe`). Both critics re-derived the bar (test V1).
- **The predictor has agreed with the check** in all seven of the verifier's m3 runs here: 14 run-direction pairs, with
  E ≤ 10 µs exactly when -3's P1 p99.9 passed (u3-verify-timing.md:126-151; the verifier's E form, not re-derived).
- **What decides E** is the few long pauses, not the many short ones: E ≈ Σ(G − 0.25 ms)² / (2T), whatever the rate.
  Over a 100 s run, one 45 ms pause alone fails it (u3-verify-timing.md:134-139), and a pause under 0.25 ms adds
  nothing. So the judged window must be the very window -3 reads.
- **Nothing subtracts L0** (the median lateness): the impairer adds d − a + U[0, 2a] to the arrival stamp as is
  (impair.rs:119-123; config.rs:142-144), so -3 sees L, not L − L0. E with L0 = 0 is the exact predictor, and it is
  never smaller than E with L0 = p50 (the excess only shrinks as the threshold rises). M1's form (L0 = p50) is lenient by
  at most p50 × P(L > 0.25 ms): about 0.24 µs in c10's run 1 (u3d-scout-r1.md F24), more with the tokio hop's higher
  p50 (0.065-0.070 ms for DelayStream's hop: advisor-u3c-plan.md:158). Question U3D-L0.
- **The R1 host:** -3 runs on L2 at P1-P4 and B1, 1,000,000 packets per profile and direction, on the probe's own
  ImpairSocket at 10,000 per second each way (ADR 0001:1590; marks.json notes :2348). It runs at R1 and at the start of
  every L2 block (plan.md:695), 0.2 h at R1 (plan.md:790). Its P1 leg is the self-test's natural run: same socket, same
  rate, same samples, 0 added hours. -3's L2 series need each datagram's drain and release stamps (SERIES.md:401-406),
  and the fate log is ImpairSocket's only per-datagram source (socket.rs:427). So the probe reads it anyway, and E over
  -3's own records costs a few lines at U6 (C-U6-5).

## 3. What U3d builds
### 3.1 The self-test run (`l2_timing_self_test`, a new `#[ignore]` test in socket_tests.rs)
- **Profile P1, seed 11** (`Profile::p1()`; asserts `reorder == ReorderStep::PART_13` as M13 does at :520). The seed
  fixes the loss draws (both c10 runs lost exactly 2,074 up and 1,983 down: :62-63, :120-121).
- **Load:** 10,000 per second per direction, paced by the existing `pace` (dedicated threads, 1 ns slack: :485-505),
  40 B payloads, for `NPT_L2_SELFTEST_SECS` seconds. The default is **100** (1,000,000 sent per direction: -3's count,
  marks.json:752). Values outside 1-600 fail with "INVALID: … the fate log reserves about 1.12 MB per second" before
  bind (R-F10). It is its own variable, so a script setting M13's `NPT_L2_SECS` cannot shorten it.
- **The inbound poller is a task spawned on a runtime the test builds** (RQ2; R-F1):
  `tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all()`, as npt-server's runtime will be (4
  workers: RS has 4 vCPUs, u3-verify-timing.md:150). The test thread calls `rt.spawn(poller(..))`. Never the test body,
  never `block_on`: either would run the poller on a parked thread, M13's hop and not quinn's (tokio
  multi_thread/mod.rs:80-91).
  - One private fn, `poller`, is shared by the run, T1 and T2. Inside `poll_fn` it calls `poll_recv` with
    `quinn::udp::BATCH_SIZE` (32) buffers of 1,472 B, quinn's size (R-F8), and loops until Pending, as quinn's
    `poll_socket` does (endpoint.rs:772ff, loop :793).
  - The release thread then wakes a spawned task's waker: the hop quinn pays.
  - **Stop:** a `tokio::sync::Notify`. Its `notified()` is polled only after `poll_recv` returned Pending, so its lock
    never comes before a stamp (R-F9); `notify_one` keeps a permit, so no stop is lost.
  - Not emulated: quinn's receive work limiter (endpoint.rs:768, :852) and QUIC's own processing (risk 3).
- **The fate log on and pre-touched** (R-F2):
  - Capacity 2 × sent + 1,000 (config.rs:58-60, default 0 :109): about 112 MB (2,001,000 entries of 56 B:
    stats.rs:426-447), reserved at bind (stats.rs:476-480).
  - Before the pacers start, the test fills the log to capacity with zero entries through the crate-visible log
    (`s.shared.lock().im.fates.push(..)`: the test module is socket's child, socket.rs:739-741; `fates` pub(crate),
    impair.rs:248) and empties it with `drain_fate_log(|_| {})`.
  - `Vec::drain(..)` keeps the capacity and its now-resident pages (stats.rs:495-500), so no page is first touched
    under the lock during the run.
  - The log gives each finished datagram's arrival, due and release stamps (stats.rs:435-440). It is read only after
    the stop.
- **The window rule** (R-F3):
  - From the first send to the poller's stop, the only ImpairSocket calls are `try_send` (the down pacer) and
    `poll_recv` (the poller).
  - The stop comes after both pacers have returned (joined), plus P1's largest one-way delay (d + a = 30 ms), plus
    500 ms (the thread poller's idle limit, :565). It is never found by polling `run_summary` (`summary_when`, :43-53,
    stays out).
  - `run_summary`, `header_numbers`, `header_texts`, the fate log and the CPU are all read after the stop.
    `l2_timer_slack_start_ns` is the start value whenever it is read.
  - /proc/loadavg is read at the start and the end on the test thread (no socket call; as delay_tests.rs:3091-3094).
- **Prints, every one before any assert** (T-9; nothing about lateness is asserted):
  - the poller kind ("tokio task, 4 workers, 32 × 1,472 B") and the fate log's capacity and pre-touch;
  - M13's per-direction line format (rate, fates, the histogram's p50, p99, p99.9, max, E, over: :579-604);
  - per direction, from the fate log, over the histogram's sample set (finished fates but Oversize: impair.rs:680-684;
    R-F5): n; E with L0 = the histogram's p50 (equal to the histogram's E within 1,000 ns, the identity T2 proves);
    **E with L0 = 0**; E with L0 = the exact nearest-rank p50;
  - per direction, one line per form: `E <x> ns vs bar 10,000 ns: would PASS/MISS (reported, not judged here)` (T3's
    function);
  - **-3's L2 P1 checks on the same samples** (U3D-XCHECK; T6's function): delays = `out − t_in` of Delivered fates
    only (`checked_sub` after the filter; the probe's stamps: SERIES.md:401-406). By nearest rank in integers (T4):
    p001, p90, p99.9, mean, and the loss share (Lost over all fates of the direction), each `would PASS/FAIL
    (reported)` against P1's bands; the count against 1,000,000. Then, per direction, whether E's line (in each form)
    and the p99.9 line agree. The RTT mean check is not computable here (no echo) and is named as such;
  - the release thread's CPU in cores, beside M13's 0.27 (advisor-u3q.md:269) and c10's 0.393 / 0.414;
  - the slack now and at start, whether it was set, the error if any, and the scheduling policy (`header_numbers`,
    `header_texts`: socket.rs:312-366);
  - loadavg at the start and the end.
- **Asserts only run conditions**, after every print. Each failure message says "the run is INVALID, not a lateness
  verdict":
  - the rate held: > 9,900 per second per direction over the send window (as :616; wall-clock, so a condition, never a
    lateness result: u3d-scout-r1.md F20);
  - no queue, pool, oversize or shaper drop (as :617-620); every datagram accounted for (:623); no send error or
    would-block (:629-632); the poller's count equals the delivered count (:633); `l2_fate_log_full_count` 0;
  - the loss count inside a 5-standard-deviation band around n × 1 %, in integers (`u64::isqrt`; R-F10). Not the exact
    `band` of delay_tests.rs:164-193: it starts from q^n, which underflows to 0 at this n (0.99^1,000,000 ≈ e^−10,050),
    so every pmf is 0 and it returns the vacuous (0, n) (rust-reviewer's confirmation);
  - the slack read back 1 at start under SCHED_OTHER (0 accepted under a real-time policy: SERIES.md:470);
  - below 100 s it prints "short run: under -3's 1,000,000 per direction" and does not assert the count.
- **Integers and order:** sums in u128 as `summarize` does (stats.rs:131-152); per-direction values in `[_; 2]` arrays
  indexed by UP/DOWN, never a HashMap (clippy `iter_over_hash_type` deny: Cargo.toml lints).

### 3.2 M13's run is not edited
`l2_sustains_10000_per_second_each_way` (socket_tests.rs:507-639) keeps P2, seed 11, the thread poller, no fate log,
its prints and its asserts. Its lines are not touched in the diff (T-10): it is U3b's evidence and the c10 comparison.
The self-test is its own function; it reuses the existing private helpers `pace`, `transmit`, `value`, `local` and
`Unpark` unchanged, and adds its own (`poller`, the E forms, the cross-check, the verdict line, the nearest rank, the
loss band).

### 3.3 Six fast tests (no `#[ignore]`: they run in check.mjs's test step on both builds, check.mjs:1121-1128)
Table in §4.

### 3.4 The dry run's script (`spikes/netcode-proof/results/u3d-timing/run.sh`)
- **Build once, run that binary:**
  - `cargo +1.99.1 test --release --locked -p npt-net --lib --no-run --message-format=json` in the main folder, with
    `CARGO_TARGET_DIR` unset (its own cache, §7.1). The test executable's path comes from cargo's JSON (the
    `executable` field).
  - The script records the binary's SHA-256 and checks that `<binary> --ignored --list` names `l2_timing_self_test`
    and `l2_sustains_10000_per_second_each_way`.
  - Every run executes that binary directly (`<binary> --ignored --nocapture --exact l2::socket::tests::<name>`), so
    the hashed binary is the one that ran.
- **Header:** the commit, `rustc -V`, the SHA-256, the date (UTC) and `nproc`. Then U3-Q9's facts and the THP mode,
  each read or written "unreadable", never guessed:
  - kernel: `uname -r`;
  - clocksource: `/sys/devices/system/clocksource/clocksource0/current_clocksource` (and `available_clocksource`);
  - cpuidle: `/sys/devices/system/cpu/cpuidle/current_driver` and `current_governor`;
  - CONFIG_HZ: `^CONFIG_HZ=` from `/boot/config-$(uname -r)`, else from `zcat /proc/config.gz`. **Never**
    `getconf CLK_TCK`, which is USER_HZ (u3d-scout-r1.md F17);
  - THP: `/sys/kernel/mm/transparent_hugepage/enabled` (R-F2);
  - this container, read 2026-11-09: 6.18.44-fc-v310, tsc, driver none, governor menu, /boot empty, /proc/config.gz
    present with CONFIG_HZ=250 (u3d-scout-req.md C8), THP madvise (both critics). It is a VM guest: steal 529 ticks
    since boot (N15).
- **Three rounds, interleaved:** the self-test (P1, 100 s), then M13's run (P2, 10 s).
  - Each run sits between `uptime` and `grep '^cpu' /proc/stat`, before and after: every line, the total and each vCPU
    (T-9).
  - Beside the raw lines, the steal share per line: Δsteal / Δ(user + nice + system + idle + iowait + irq + softirq +
    steal), steal being the 8th field (proc(5); not re-read here, no network: medium-high confidence). The raw lines
    stay in the record, so anyone can recompute.
  - Each run's exit status is recorded. An INVALID run stays in the record with its cause and the script goes on
    (never retried into a pass: plan.md:683).
  - About 6-7 minutes in all.
- The output goes to `results/u3d-timing/selftest-<UTC>.txt`, the runs' own output unchanged, like c10's file.

### 3.5 SERIES.md: two row texts, no new name
`second.v.l2_late_up_excess_ns` (:427) and `second.v.l2_late_down_excess_ns` (:452) say "(Mustafa's bar: about 10 µs
or less at P1, judged on RS at R1)" on a per-second value, whose L0 is that second's own median. They become: "per
second, reported; the bar (about 10 µs at P1, U3-Q1 (c)) is judged at R1 on the probe's P1 leg over its measured
records (plan §12.1 U8), never per second". The `l2_summary` paragraph (:391-396) gains: "its lateness fields cover
the socket's whole life; R1's bar reads the probe's own computation (U6)".

### 3.6 Plan text (the main session, committed before any code merges)
- **§12.1 U3 row** (plan.md:1222) adds: "U3d: the timing self-test: ImpairSocket at P1, 20,000 releases per second,
  inbound polled from a spawned tokio task as quinn polls; reported here, judged on RS at R1 (U8)."
- **U6 row** (:1225), seven carries:
  - **C-U6-1 (the poller; R-F1, T-7)** The probe's inbound poller is a task spawned on npt-server's multi-thread
    runtime (never `block_on`), calling `poll_recv` with quinn's 32 buffers of 1,472 B until Pending. That way -3's L2
    up series and the leg's E include the hop quinn pays. T1's blocked-worker test is re-run on the probe's own poller.
    The pacer stays a dedicated thread (SERIES.md:405-406).
  - **C-U6-2 (one socket per leg; R-F7, T-7)** One ImpairSocket per profile leg, so each `l2_summary` is one leg's.
    There is no reset method (plan §4.7 lists none: plan.md:580-584), and `set_default` (socket.rs:251) never carries
    a socket across legs. A test shows two legs' summaries never mix.
  - **C-U6-3 (the fate log; R-F2, T-11)** U6 chooses the drain cadence:
    - per second, as -3's `second.s.probe_*` records need, with capacity for one to two seconds, its pages touched in
      the warm-up;
    - or one log per leg, pre-faulted before the measured phase. A bind-time pre-fault inside `FateLog::new` needs no
      new public item.

    Either way, the drain's closure only copies into a buffer the probe owns: no I/O under the socket's lock
    (socket.rs:427-429). The lock hold per drain is measured and reported. By Σ(G − 0.25 ms)²/(2T), holds under
    0.25 ms add nothing to E.
  - **C-U6-4** The probe process writes `second.v.steal_permille` as npt-server does (SERIES.md:98).
  - **C-U6-5 (the judged E's producer; T-4, T-5)** At the P1 leg's end, the probe computes per direction, from the
    same fate-log entries it writes -3's series from, and over exactly -3's measured records:
    - n, E with L0 = 0, E with L0 = the window's nearest-rank p50, the over count, and p50, p99, p99.9 and max;
    - on the sample set of the histogram (finished fates but Oversize); -3's delays use Delivered only;
    - written in the leg's summary event, under names U6 adds to SERIES.md (M15).

    Two tests: a planted late datagram in the warm-up stays out of E; with no warm-up, E with L0 = the histogram's p50
    equals `l2_summary`'s within 1,000 ns (U3d's T2 identity, re-run).
  - **C-U6-6 (rate and composition; T-6)** U6 states the up stream's composition (LG's fresh sends against its echoes:
    SERIES.md:405-406), so that the probe's socket sees 10,000 per second each way (20,000 releases per second). If it
    sees another rate, that rate is recorded and named with the verdict. The header records the composition and the
    poller kind.
  - **C-U6-7 (observer calls; R-F3)** Inside the measured window the probe calls only `try_send`, `poll_recv`,
    `take_second` once per second and the drain. `header_numbers`, `header_texts` and `run_summary` run at the leg's
    edges: `header_numbers` makes the release thread read /proc under the state lock (socket.rs:667-677). This is the
    same rule plan.md:1225 already carries for DelayStream (F6).
- **U8 row** (:1227), ImpairSocket's R1 self-test as §5.2 gives it, plus:
  - **C-U8-1** U3-Q9's four facts and the THP mode by shell before any timing judgement, with the dry run's commands;
  - **C-U8-2** every `^cpu` line of /proc/stat read before and after the probe's P1 leg (DelayStream's run already
    has a steal read);
  - **C-U8-3** DelayStream's loaded self-test (48 connections, P2, room running) has no hours in R1's row
    (plan.md:790): U8 sizes it (u3d-scout-r1.md F6);
  - **C-U8-4** the self-test's run conditions at R1 (§5.2), the rate among them (T-6).
- **§15** gains the advisor's rulings on U3D-BAR, U3D-L0 and U3D-WINDOW as readings.

### 3.7 What U3d does not build
| Not built | Why | Where it is carried |
|---|---|---|
| A self-test binary or npt-lab subcommand | U6's probe runs the same load on RS; new self-test modes are unplanned surface (u3-verify-timing.md:195; plan.md:588-589) | U6 (C-U6-1..7) |
| npt-log names for steal, THP or U3-Q9's facts | No check reads them; Q9 puts them in the run plan; CONFIG_HZ here is only in /proc/config.gz (a gzip reader would be a dependency) | U8 (C-U8-1, -2); per-second steal U6 (C-U6-4) |
| A lateness assertion or a marks.json mark | "Reported only" (u3-build-plan.md:45-46); marks.json holds plan §6.2's marks only (check.mjs:1027-1052) | R1, U3D-JUDGE |
| A spin lead, or code applying SCHED_FIFO | M14 (advisor-u3q.md:270-273); U3-Q2 (b) "never a spin" (checkpoint :494-495) | The miss path (U3D-MISS) |
| A per-release split for ImpairSocket | A scale check (advisor-u3c-plan.md:163); no traced need yet | A U3D-MISS step, if the advisor asks |
| A run-window reset, or a second E field in stats.rs | A reset is a new public item (R-F7). The L0 = 0 form and -3's window come from the probe's fate log at U6 (C-U6-5), so stats.rs needs neither | U6 (C-U6-2, -5) |
| A production pre-fault of the fate log | The test pre-touches it with crate-visible code (§3.1). The probe's choice depends on its drain cadence | U6 (C-U6-3) |
| Reading marks.json's bands at test time (T-3's drift check) | npt-net has no JSON parser (crates/net/Cargo.toml dev-dependencies: futures-util only), and one would change Cargo.lock. The bands are derived from npt-l1's P1 by the rule each check's name states, and T6 pins them to marks.json's numbers (:763-1021) as cited literals. The cross-check is report-only, and npt-judge, reading marks.json itself, judges -3 at R1 | Not carried |
| A live stall test (T-12c), or a fate-log-off control round (T-8) | Each piece is proven: E against brute force (stats.rs:537), only beyond the allowance (:593), windows (:613, :656), both stamps taken after the lock (socket.rs:501-503, :724-726; socket_tests.rs:184-237), a blocked consumer's delay entering L (T1). A live verdict bite cannot discriminate in a container that already misses (u3d-scout-r1.md F25). After the pre-touch, the log costs one 56 B copy per datagram under the lock, far under 0.25 ms, which adds nothing to E (§2) | Not carried; the advisor may ask |
| A P1 thread-poller control round (T-8) | Optional: U3D-POLL (b+) | The advisor's choice |
| A real quinn endpoint at 10,000 per second each way | U3D-POLL (c) | Not carried |
| UP-STAMP 1-3; DelayStream's RS run; the C10-B fallback; S14; M12; M15 | U6 (plan.md:1225); U8 (:1227); R1; R1's run plan (u3-build-plan.md:47); done in U3b | As listed |

## 4. Tests and their bites
Each bite is planted once in a scratch copy (a script in this folder, as u3c-build-b-bites.py, under §7.1's rules) and
must fail its own test for its own reason.

| Test | What it proves | Fixed inputs | Bite that must fail it |
|---|---|---|---|
| T1 `the_task_poller_takes_lateness_where_the_task_runs` | The shared `poller` measures where a spawned task polls: the hop enters L (RQ2) | Fixed 20 ms one way, no jitter (`fixed(20)`, :17-26); a 1-worker runtime (`enable_all`). (1) Spawn `poller`; wait on a std channel for its first Pending (waker stored). (2) Spawn a blocker task: it signals "running" on a std channel, then holds the only worker (std sleeps of 1 ms) until `first_seen(peer)` + 110 ms has passed; it gives up after 2 s and the test fails with "no drain". (3) Only after "running", the peer sends one datagram. Asserts the up lateness in [50 ms, at − first − 20 ms], with `at` stamped by the poller on Ready (the bound of :207-211). The lower bound holds by construction: due = first + 20 ms, poll ≥ first + 110 ms (T-1, R-F4) | (a) `poller` on its own std thread (the old `recv`): it polls at due, lateness near 0. (b) `poller` under `block_on` on a std thread (R-F1): the same |
| T2 `the_fate_log_and_the_histogram_agree` | The fate-log forms and -3's cross-check read the same samples as the instrument's own E | P1 (jitter, 1 % loss, the reorder step), seed fixed, 2,000 per second per direction for 1 s through `poller`, the fate log on. Asserts per direction: lost > 0 (fixed by the seed; P(0) ≈ 2 × 10⁻⁹ for any seed), so the count bite always discriminates (T-2); send errors and would-blocks 0 (T-12a); fate-log entries of the E set (Delivered, SendError, WouldBlock) = `l2_late_*_count`; Delivered entries = `l2_*_delivered_count`; the fate log's E with L0 = the summary's p50 within 1,000 ns of `l2_late_*_excess_ns` (stats.rs:3-8's bound). Identities only: no timing assert | (a) L taken as `out − t_in` (the delay, not the lateness): E off by about 25 ms. (b) Lost fates not filtered out: the count differs |
| T3 `the_verdict_line_is_per_direction_and_inclusive` | The would-line applies "≤ 10,000 ns" (U3D-BAR's ruling, T-12b) per direction, never pooled | E 10,000 → PASS; 10,001 → MISS; up 10,001 with down 0 → up MISS, down PASS | `<` for `≤` fails at 10,000; a pooled mean (5,000.5) fails the mixed case |
| T4 `nearest_rank_matches_the_judge` | -3's cross-check ranks as npt-judge does (⌈q × n⌉ in integers: eval.rs:261-265) | 36,000 values give the 35,640th for p99 (eval.rs:1595-1597; marks/README.md:103-104); 1,001 values give the 1,000th for p99.9; 1,000,000 give the 1,000th for p001 | ⌊q × n⌋ (gives the 999th of 1,001) or an interpolated quantile |
| T5 `the_exact_excess_on_fixed_samples` (new; T-2) | The exact E forms: the right L0, divided by n, (L − L0 − 0.25 ms)+ | 1,000 lateness samples: 999 of 100,000 ns and one of 10,350,000 ns; nearest-rank p50 = 100,000. E with L0 = p50 = 10,000,000 / 1,000 = **10,000 ns** (would PASS); E with L0 = 0 = 10,100,000 / 1,000 = **10,100 ns** (would MISS). Asserts both values exactly and both would-lines | L0 swapped (both asserts fail); divided by the over count (10,000,000); L0 = the mean (110,250 → 9,989); the allowance not subtracted, (L − L0)+ above the threshold (10,250) |
| T6 `the_cross_check_reads_delay_plus_lateness` (new; T-3) | -3's P1 cross-check: delay = `out − t_in` (D + L), Delivered only, nearest rank, loss share, P1's bands | Up fates: 1,000 Delivered with D_i = 20,000,000 + 10,000 × i ns (i = 0..999), L = 0 except i = 998, 999 with L = 300,000 ns; 10 Lost (due 0, out 0). Hand values: p001 20,000,000 PASS; p90 28,990,000 PASS; p99.9 **30,280,000 FAIL** (> 30,250,000); mean 24,995,600 PASS; loss 10 / 1,010 PASS; n 1,000. E (both forms) 100 ns: the agree line says "disagree". Also: P1's bands derived from `Profile::p1()` (d = 25 ms, a = 5 ms: d − a, d + 0.8a, d + a each ± 0.05a; mean and loss ± 5 %) equal marks.json's numbers (:763-1021) | `due − t_in` (D only): p99.9 29,980,000 → PASS, the assert fails. Lost entries kept: n 1,010 and p001 0 (or a `checked_sub` panic naming a lost entry). A band rule off by one term fails the pinned numbers |

T1 and T2 inject latency, jitter and loss. They decide on identities and on a lower bound that a blocked worker sets,
never on how fast this container is or on the order threads happen to start in. T3-T6 are pure. The self-test itself
(§3.1) asserts only run conditions. check.mjs's counts rise by exactly six per build (§7.1).

## 5. Evidence and what is judged
### 5.1 Here: the dry run (reported, not judged)
- The main session runs §3.4's script after the merge. The container is quiet (no agent or build alongside, nothing
  else on loopback); the build is release. The output is committed under `results/u3d-timing/`.
- Reported per run and direction:
  - n, p50, p99, p99.9, max, over;
  - E (histogram, L0 = p50); E exact with L0 = p50, with L0 = the exact p50 and with L0 = 0; the would-lines;
  - -3's P1 cross-check (p001, p90, p99.9, mean, loss, count) and whether it agrees with each E line;
  - the CPU against 0.27 core; slack and policy; the steal shares; loadavg.

  Once per record: U3-Q9's facts and the THP mode.
- M13's three P2 runs are reported beside c10's (same code, path and seed).
- **What the rounds can and cannot show** (T-8, R-F6):
  - The self-test and M13 differ in poller, profile, length and fate log. p50 and p99 compare roughly. E does not
    compare: one long pause decides it (§2).
  - Down differs too, because the release thread wakes the task before its sends (socket.rs:713, :724-726). So down is
    compared only within the self-test's own rounds.
  - Nothing here attributes E to the hop. That needs U3D-POLL (b+), or RS.
- Expected: misses (§0). A miss here goes to the advisor in U3d's report; it is not retried into a pass (plan.md:683)
  and it is not a stop (advisor-u3c-plan.md:163-165).

### 5.2 At R1, on RS (carried to U8's row; judged by the advisor if U3D-JUDGE (J3) is ruled)
- **Where:** the -3 L2 probe's P1 leg in R1's L2 probe block (plan.md:790; ADR 0001:1590). That is npt-server's probe
  mode with its own ImpairSocket, one per leg (C-U6-2), at 10,000 per second each way on RS (20,000 releases per
  second). 0 added hours.
- **Judged, per direction:** the probe's E over the P1 leg's measured records, at least 1,000,000 sent per direction
  (C-U6-5; U3D-WINDOW (W1)), against E ≤ 10,000 ns (U3D-BAR), in the form U3D-L0 names.
- **Run conditions.** A failure makes the self-test INVALID, never a pass:
  - -3's own count;
  - the rate: each direction's releases over the measured records at 10,000 per second within 1 % (C-U6-6, T-6);
  - slack 1 at start and at header time, or 0 under the recorded real-time switch on a kernel that carries
    ed4fb6d7ef68 (SERIES.md:470);
  - no queue, pool, oversize or shaper drop; no send error;
  - the steal reads and U3-Q9's facts present ("unreadable" recorded counts as present; a missing read does not).
- **Reported:**
  - p50, p99, p99.9, max, over, the other E form, and `l2_summary`'s E (the socket's whole life);
  - the release thread's CPU, slack and policy, the poller kind and the stream composition;
  - steal (both reads of every `^cpu` line, and the per-second row), U3-Q9's facts, THP, loadavg;
  - the drain's lock hold (C-U6-3);
  - a max over 10 ms, flagged with the result (U3D-MISS).
- **Ground truth beside it:** -3's own L2 P1 checks, judged by npt-judge as always, with -3's bites.

## 6. SERIES.md and marks.json
- SERIES.md: the two row texts and one sentence of §3.5. No new name, so `every_header_name_has_a_series_row`
  (socket_tests.rs:388) is untouched. C-U6-5's names are U6's rows.
- marks.json: unchanged (no new mark, no new check).

## 7. Order and build route
1. **Plan text** (§3.6, §3.5): the main session, committed before the code merges.
2. **Build:** one agent (general-purpose, opus, xhigh) in an isolated worktree, under §7.1; or the main session itself,
   the change being one test file and a script. It covers §3.1-§3.4 and ends with:
   - check.mjs 9/9, with the counts stated: measured 266 and bites 280 (U3c's close: 260 and 274, checkpoint
     :523-524, plus T1-T6), skips 0;
   - clippy -D warnings with and without features, fmt, Cargo.lock unchanged;
   - M13's lines unchanged in the diff;
   - notes in `.checkpoints/netcode-proof-test/u3d-build.md`.
3. **Bites** (§4): planted by a scratch script under §7.1, each failing its own test by name; the output kept in the
   notes.
4. **Merge**, then, in the main folder with its own cache: check.mjs 9/9, `node tools/gate.mjs --no-smoke`, a commit
   and a checkpoint.
5. **The dry run** (§3.4, §5.1) by the main session, one evidence run at a time.
6. **One review round on the final diff** (§9), fixes, then U3d's report to the advisor with the numbers reported.

### 7.1 Build rules (Mustafa's shared-target rule)
Mustafa, 2026-11-09 (verbatim, checkpoint "Decisions taken", .checkpoints/netcode-proof-test.md:404-405): "from U3d
on, point all copies at one shared Cargo target folder (CARGO_TARGET_DIR), so each unit doesn't leave a 4–6 GB cache
behind." The main session's application (:406-413) for U3d's copies:
1. **One folder for every copy.** In the builder's worktree and in the bites' scratch copy, every cargo call runs with
   `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target` in its environment: cargo test, clippy, fmt,
   and check.mjs's steps. check.mjs spawns cargo with the inherited environment and passes no `--target-dir`
   (check.mjs:1070-1071, :1115-1128).
   - Without the variable, the spike's `.cargo/config.toml` (`target-dir = "../../.gate/netcode-proof-test/target"`,
     relative to the copy) would build a 4-6 GB cache inside the copy. The environment variable overrides that
     setting (cargo's documented order; not re-read here, no network: medium-high confidence).
   - Checked once, after the first build: the copy has no `.gate/netcode-proof-test/target`, and the shared folder has
     the new artifacts.
2. **Touch before each build** (the G-METHOD safeguard, checkpoint :661-662). Before every build (each check.mjs run,
   each clippy run, each bite build), the copy touches its own crate sources, mtime only:
   `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +`. The reason: cargo's fingerprint ignores a copy's
   location, so another copy's newer artifact can read as fresh.
3. **Check each run's test count.** check.mjs's summary must read measured 266, bites 280, skips 0.
   `cargo test -p npt-net --lib -- --list` must name T1-T6, and `--ignored --list` must name `l2_timing_self_test` and
   M13's run. A bite run must show its own test failing by name. A count or name that does not match means another
   copy's artifact ran: touch, rebuild, and never report that run.
4. **Bites and clean builds alternate safely.** After the last bite, the clean copy touches and rebuilds before any
   further claim, because the bite's artifact is the newest in the folder.
5. **Cargo's own lock** serializes two copies' builds in the folder ("Blocking waiting for file lock …"): expected, not
   an error.
6. **The main folder keeps its own cache** (.gate/netcode-proof-test/target, from the spike's config). The merge checks
   (check.mjs, the gate) and the dry run run there with `CARGO_TARGET_DIR` unset, so they never read a copy's
   artifacts.
7. **No copy cleans the shared folder.** Its cleanup is the main session's (`cargo clean --target-dir`; cargo created
   the folder, so it carries CACHEDIR.TAG: checkpoint :406-408).

## 8. Risks
1. **This container decides nothing at the µs level** (advisor-u3c-plan.md:114). The release thread's own OS wake
   carries tails as large as the whole (advisor-u3c-plan.md:155-157), and one long pause decides E
   (u3-verify-timing.md:134-139). Mitigation: three runs reported, no verdict; RS judges.
2. **The fate log adds work to the measured path.** About 112 MB is reserved at bind. Without the pre-touch, about
   27,400 4 KB pages (THP is madvise here) would be first touched under the socket's lock: by the release thread, the
   reader and the down pacer, since drops are logged at intake too (impair.rs:500-506, :576-579). In a VM guest a first
   touch can also fault on the host. Mitigation: the pre-touch (§3.1) moves every first touch before the run. What is
   left is one 56 B copy per datagram, far under 0.25 ms. The probe at R1 meets the same question (C-U6-3).
3. **The tokio poller is an idle runtime's hop and does no QUIC work.** Under a room's load the hop differs, and quinn's
   per-datagram processing delays its next poll. R1's probe is idle too, so the self-test matches -3. The game's real
   load is measured by DelayStream's loaded run, not by this one. The hop also changes down's lateness (R-F6): faithful
   to quinn, not like-for-like with M13. Recorded as a limit.
4. **The container runs both ends.** The up pacer and the down sink (LG's job at R1) share the 4 CPUs here, so the
   container is the harder case.
5. **M13's evidence:** not touched (§3.2).
6. **"About 10 µs" is not a number npt-judge or a reader can apply** (marks/README.md:109-110): U3D-BAR.
7. **E's two forms can straddle the bar** (§2): U3D-L0. -3's own p99.9 is reported beside it.
8. **The judged value moves to U6's code** (C-U6-5): a new computation, not ImpairSocket's histogram. Mitigation: it
   reads the same stamps the histogram records. T2's identity (re-run at U6) and the warm-up test pin it, and
   `l2_summary`'s E is reported beside it.

## 9. Change type, evidence rule, reviewers
- **Change type:** netcode (the L2 measurement instrument), test code only. Evidence rule: deterministic tests with
  injected latency, jitter and loss (T1, T2; T3-T6 pure), each bite failing; the dry run reported, not judged.
- **Gate:** check.mjs 9/9 on both builds with the counts of §7.1, clippy -D warnings with and without features, fmt,
  `node tools/gate.mjs --no-smoke`, Cargo.lock unchanged, all under §7.1's build rules.
- **Reviewers on the final diff:**
  - netcode-architect: the measurement point, the windows, what R1 judges;
  - rust-reviewer: the spawned poller, the pre-touch, the test code, clippy;
  - test-engineer: is it proven (bites, run conditions, the record).

  Not security-auditor: no network input path, no dependency, no production code.

## 10. Questions for the advisor
- **U3D-BAR, the exact bar.** Plain words: "about 10 µs" is not a number anyone can apply.
  - (a) E ≤ 10,000 ns per direction: the derivation gives exactly 0.001 × 2a = 10,000 ns at P1
    (u3-verify-timing.md:45-48). "About" is the derivation's own approximation (L independent of the delay), not a
    tolerance.
  - (b) A margin, for example ≤ 11,000 ns.
  - (c) Leave it "about" and read it each time.

  **Recommend (a)**, recorded in §15 as a reading. T3's constant follows your ruling. If you read it as changing
  Mustafa's bar, it goes to him.
- **U3D-L0, which E is judged.** Plain words: the bar counts how far hand-overs run past 0.25 ms. M1 measures that past
  each window's typical lateness, its median (L0 = p50, `l2_late_*_excess_ns`). Mustafa's recorded words and -3 itself
  measure it from zero (L0 = 0). The facts:
  - nothing subtracts the median (impair.rs:119-123), so -3 sees the full lateness;
  - the zero form is never smaller, so judging it is never more lenient;
  - the session's record of U3-Q1 (c) says "expected excess lateness beyond 0.25 ms" (checkpoint :493-494);
  - M1 asks to *report* E with L0 (advisor-u3q.md:235);
  - with C-U6-5, both forms cost the same at R1, and both are printed here.

  The options:
  - (a) Judge M1's form; report the zero form; when the two straddle 10,000 ns, -3's own P1 p99.9 decides the reading.
  - (b) Judge the zero form; report M1's form as built (stats.rs and the per-second rows unchanged).
  - (c) Make the impairer subtract its median. No: it changes the delays -3 checks.

  **Recommend (b):** it is the exact predictor of the check the bar protects, it errs on the strict side, and it
  matches Mustafa's recorded words. M1's report stays as built. This moves from rev 1's (a): there, the zero form had no
  producer at R1 (T-4). If you read (b) as changing M1, it is your ruling; if you read it as changing Mustafa's bar, it
  goes to him.
- **U3D-WINDOW, which value R1 reads** (new; T-5).
  - (W1) The probe's E over the P1 leg's measured records, computed from the fate log (C-U6-5): -3's own window and
    samples, by construction.
  - (W2) `l2_summary`'s E, with one socket per leg and nothing through it outside the measured phase. It still holds
    the datagrams released after the last measured second, and it gives only M1's form.

  **Recommend (W1)**, with `l2_summary` reported beside it.
- **U3D-POLL, the inbound poller.**
  - (a) Keep the parked thread.
  - (b) A task spawned on a multi-thread runtime with quinn's 32-buffer batch, the same shape carried to U6's probe
    (C-U6-1).
  - (b+) As (b), plus one P1 control round per dry-run round with the same 32-buffer loop on a parked thread. That is
    about 20 lines of test code and 5 minutes per round. It gives the hop's p50 and p99 like-for-like here. E stays
    pause-dominated (T-8).
  - (c) A real quinn endpoint at 10,000 per second each way. QUIC's acknowledgements and congestion control change the
    release counts; it is heavier, and no need traces to it.

  **Recommend (b):** it adds the hop quinn pays without new surface. Take (b+) if you want the hop's typical cost
  measured before R1.
- **U3D-JUDGE, how R1 judges.**
  - (J1) Setup checks in -3's L2 P1 part. They would turn a -3 FAIL into INVALID, or make -3 stricter than its ADR row.
  - (J2) R1's run script computes the verdict, with its own fixtures (U8).
  - (J3) You read the U3D-WINDOW value against the exact bar, as you read c10.

  **Recommend (J3):** the self-test checks L2's instrument rather than being a plan mark (check.mjs:1027-1052). Its miss
  goes to you anyway, and plan §6.1's "bites first" covers automated marks (plan.md:680-682). The would-line (T3) makes
  the reading mechanical.
- **U3D-MISS, the miss path at R1.**
  - (a) The numbers, steal and U3-Q9's facts come to you before any L2 mark counts. The first remedy is the recorded
    switch (`chrt -f` on the release thread, `release_thread_id` socket.rs:268), measured as its own recorded leg, never
    a retry over the miss (plan.md:683). A remedy that needs a spin lead goes to Mustafa (U3-Q2 (b)). A max over 10 ms
    is flagged with the result, as for DelayStream (advisor-u3c-plan.md:115-116).
  - (b) A miss stops every L2 mark until a remedy passes.
  - (c) Report only: -3's own check decides L2.

  **Recommend (a).**
- **U3D-XCHECK, -3's checks from the same run.**
  - (a) Keep them, with the fate log on and pre-touched: the predictor and the checked value on the same samples, as the
    verifier's m3 runs did. T6 now bites the delay formula.
  - (b) Drop them: smaller, but the container's E then has nothing to be read against.

  **Recommend (a).**

## 11. For Mustafa
Nothing, unless the advisor reads U3D-BAR or U3D-L0 as changing his bar.
