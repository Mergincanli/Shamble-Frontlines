# U3d plan, rev 1: the timing self-test (how late ImpairSocket hands datagrams over at 20,000 releases per second)

Written 2026-11-09 by netcode-architect on head 49bd1ac (= 95dac9c, U3c's close, plus checkpoint-only commits; code
as at 768d00b). Inputs: the scouts' notes u3d-scout-req.md (netcode-architect) and u3d-scout-r1.md (test-engineer),
both re-checked at the cited lines (my notes: u3d-plan-notes.md N1-N24). Next: the two critics
(u3d-plan-review-test.md, u3d-plan-review-rust.md), rev 2, then the advisor. **No code before the advisor's APPROVE.**

## 0. In plain words
- **What it is.** L2's impairment layer (ImpairSocket) delays every datagram by the profile's amount and hands it over
  when it falls due. "Lateness" is how much later than that the hand-over really happens. If the hand-over is late too
  often, PM-0001-3's tightest delay check on L2 (P1's 99.9th percentile within 0.25 ms) fails for reasons that are our
  machine's, not the profile's. U3-Q1 (c), which Mustafa agreed to, sets the bar: the expected lateness beyond
  0.25 ms stays under about 10 µs at the tightest profile (P1), at 20,000 releases per second, on RS.
- **What U3d does.** It makes the self-test measure exactly that, here: at P1, 10,000 datagrams per second each way,
  with the inbound hand-over polled by a tokio task the way quinn polls (today's run uses a parked thread and misses
  that hop). It prints the percentiles, the statistic in both forms, the CPU, the timer slack and policy, and, from the
  same run, PM-0001-3's own L2 P1 delay checks, so the predictor and the check it predicts are seen side by side.
  A script runs it three times in a quiet container, with the kernel facts U3-Q9 asks for and the steal before and
  after each run. Everything here is reported, never judged: this container is no evidence for a µs-level bar
  (advisor-u3c-plan.md:114). RS judges at R1.
- **Size.** Test code only (one test file), a results script, two SERIES.md row texts, and plan text. No production
  code, no public item, no log name, no dependency.
- **What it expects here.** A miss: ImpairSocket's own E was 11.6 and 16.9 µs inbound (and 12.9 µs outbound in run 2)
  at P2 with the thread poller (results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt:62, :120-121), and the tokio hop
  adds about 10-19 µs of E here (u3-verify-timing.md:155, :159). That is reported to the advisor, not a stop
  (advisor-u3c-plan.md:163-165: "U3d reports it, RS judges it").

## 1. Requirements
| # | Requirement | Source | Exists | Gap U3d closes |
|---|---|---|---|---|
| RQ1 | The statistic, per direction: E[(L − L0 − 0.25 ms)+] beside p50, p99, p99.9, max | u3-build-plan.md:45-46; M1 advisor-u3q.md:233-236 | stats.rs:20-21, :51-62, :108-159; samples impair.rs:680-684; run window socket.rs:394-415 | Say which window is judged: the whole run (each window's L0 is its own p50: stats.rs:129-130), never a second's |
| RQ2 | Inbound lateness "where quinn polls"; outbound at try_send's return | M1 (advisor-u3q.md:233-236); u3-build-plan.md:45 | Stamped inside poll_recv (socket.rs:501, :525) and after try_send (:722-726); tested socket_tests.rs:184-237 | The run's poller is a parked std thread (socket_tests.rs:55-88, :561-571); quinn polls from a tokio task (quinn-0.11.12 endpoint.rs:152-160, :781-794) with 32 buffers (quinn-udp-0.5.16 unix.rs:804-806). The hop it misses is about 10-19 µs of E here (u3-verify-timing.md:155, :159) |
| RQ3 | 10,000 per second each way sustained, jitter, loss and the reorder step on; the release thread's CPU recorded | M13 advisor-u3q.md:267-269 | socket_tests.rs:507-639; 0.393 and 0.414 core (c10 :64, :122) against M13's 0.27 | None: reported beside 0.27 core |
| RQ4 | "At the tightest profile", at 20,000 releases per second | the session's record of Mustafa's U3-Q1 (c), .checkpoints/netcode-proof-test.md:473-474 (his words :468-469 carry no number); the bar's derivation u3-verify-timing.md:42-48 | The run is fixed to P2 (socket_tests.rs:519) and checks P2's 2 % loss (:624-627) | Run at P1 (npt-l1 lib.rs:118-121: 50 ms, ±10 ms, 1 %; the reorder step on :110); the loss check from the profile |
| RQ5 | Percentiles reported; "here it is reported only (RS judges at R1)" | u3-build-plan.md:45-46 | Printed only (socket_tests.rs:579-614) | None |
| RQ6 | 1 ns slack through /proc, read back and tested; real-time priority only a switch applied from outside; no spin | M14 advisor-u3q.md:270-273; U3-Q2 (b), checkpoint :474-475 | Done in U3b: thread.rs:20-90; socket.rs:648-676, :312-366; tests thread.rs:99-140, socket_tests.rs:353-384; `release_thread_id` pub (socket.rs:268) | The run prints the slack (:611-613) but not the policy: one line |
| RQ7 | Steal from /proc/stat before and after each ImpairSocket and DelayStream self-test run, nothing in the measured path | advisor-u3c-plan.md:191-192 (UP-STAMP item 5); plan.md:1227 | Neither run reads it (M13 none; delay_tests.rs:3091-3094 loadavg only) | Two reads around each run in the dry run's script; R1's in the run plan (U8) |
| RQ8 | U3-Q9: RS's kernel, CONFIG_HZ, cpuidle driver and clocksource read at R1 (R1's run plan) | checkpoint :481; asked at u3-scout-summary.md:81 | Kernel only (npt-log `Machine.kernel`, crates/log/src/lib.rs:35) | A shell step in the dry run's script now and in R1's run plan (U8) |
| RQ9 | ImpairSocket's E over about 10 µs here: "U3d reports it, RS judges it" | advisor-u3c-plan.md:163-165 | c10: up 11.610 / 16.921 µs, down 2.672 / 12.939 µs (:62-63, :120-121) | Report both directions (run 2's down is over too, which the ruling does not name) |
| RQ10 | RS judges ImpairSocket at R1 | u3-build-plan.md:46 | No binary runs it on RS. -3's L2 probe (npt-server's probe mode, U6: SERIES.md:401-406; ADR 0001:1590) runs the same load on RS at R1 (plan.md:695, :790) | Plan text: where, window, bar, conditions, who judges, the miss path (the U8 row names ImpairSocket only in the steal clause, plan.md:1227; the U3 row omits U3d, :1221) |

## 2. Rulings and facts this plan rests on
- **The bar's origin** (u3-verify-timing.md:42-48): with the delay uniform on d ± a and an independent lateness L′,
  PM-0001-3's p99.9 upper check (d + 1.05a) holds iff E[(L′ − 0.25 ms)+] ≤ 0.001 × 2a. At P1, a = 5 ms per direction
  (half of the ±10 ms jitter: npt-l1 lib.rs:156-158; impair.rs:105, :122-123), so 2a = 10 ms and the budget is
  **10,000 ns**. P2's would be about 20 µs: P1 binds. The bands: marks.json PM-0001-3 (:752 the 1,000,000 count, :848
  p99.9 ≤ 30,250,000 ns; parts "L2 P1", process `probe`).
- **The predictor has agreed with the check** in all seven of the verifier's m3 runs here (14 run-direction pairs: E
  ≤ 10 µs exactly when -3's P1 p99.9 passed; u3-verify-timing.md:126-151; the verifier's E form, not re-derived).
- **What decides E** is the few long pauses, not the many short ones: E ≈ Σ(G − 0.25 ms)² / (2T); over a 100 s run
  one 45 ms pause alone fails it (u3-verify-timing.md:134-139). So the judged window must be the very window -3 reads.
- **Nothing subtracts L0** (the median lateness): the impairer adds its delay to the arrival stamp as is (config.rs:
  142-144; impair.rs:122-123), so -3 sees L, not L − L0. E with L0 = 0 is the exact predictor; M1's form (L0 = p50)
  is slightly lenient: by at most p50 × P(L > 0.25 ms), about 0.24 µs in c10's run 1 (u3d-scout-r1.md F24), more with
  the tokio hop's higher p50. Question U3D-L0.
- **The R1 host:** -3 runs on L2 at P1-P4 and B1, 1,000,000 packets per profile and direction, on the probe's own
  ImpairSocket at 10,000 per second each way (ADR 0001:1590; marks.json notes :2348), at R1 and the start of every L2
  block (plan.md:695), 0.2 h at R1 (plan.md:790). Its P1 leg is the self-test's natural run: same socket, same rate,
  same samples, 0 added hours.

## 3. What U3d builds
### 3.1 The self-test run (`l2_timing_self_test`, a new `#[ignore]` test in socket_tests.rs)
- **Profile P1, seed 11** (`Profile::p1()`; asserts `reorder == ReorderStep::PART_13` as M13 does at :520). The seed
  fixes the loss draws (the real path moves no fate; both c10 runs lost exactly 2,074 up and 1,983 down: :62-63,
  :120-121).
- **Load:** 10,000 per second per direction, paced by the existing `pace` (dedicated threads, 1 ns slack: :485-505),
  40 B payloads, for `NPT_L2_SELFTEST_SECS` seconds, default **100** (1,000,000 sent per direction: -3's count,
  marks.json:752). Its own variable, so a script setting M13's `NPT_L2_SECS` cannot shorten it.
- **The inbound poller is a tokio task** (RQ2): a multi-thread runtime built in the test (4 workers, as
  delay_tests.rs:3084; RS has 4 vCPUs, u3-verify-timing.md:150) runs one task that calls `poll_recv` inside `poll_fn`
  with `quinn::udp::BATCH_SIZE` buffers of 1,500 B and loops until Pending, as quinn's `drive_recv` does
  (endpoint.rs:781-800). ImpairSocket's release thread then wakes a tokio task's waker: the hop quinn pays. It stops
  on a signal sent after the last datagram's due time plus 500 ms (the thread poller's idle limit, :565); no tokio
  timer on the poll path. Not emulated: quinn's receive work limiter (endpoint.rs:768, :852) and QUIC's own
  processing (risk 3).
- **The fate log on** (`fate_log_capacity` = 2 × sent + 1,000; config.rs:58-60, default 0 :109): about 112 MB
  (2,000,000 entries of 56 B: stats.rs:426-447), reserved at bind (stats.rs:476-480), read only after the run (the
  drain holds the socket's lock: socket.rs:427-429). It gives each delivered datagram's arrival, due and release
  stamps (stats.rs:435-440).
- **Prints** (reported; nothing about lateness is asserted):
  - M13's lines as they are (rate, fates, the histogram's p50, p99, p99.9, max, E, over: :579-604);
  - per direction, from the fate log: E with L0 = the histogram's p50 (equal to the histogram's E within 1 µs, the
    identity test T2 proves) and **E with L0 = 0**;
  - per direction, one line: `E <x> ns vs bar 10,000 ns: would PASS/MISS (reported, not judged here)`, for the form
    U3D-L0 names;
  - **-3's L2 P1 checks from the same run** (U3D-XCHECK): each delivered datagram's delay = release − arrival (the
    probe's stamps: SERIES.md:401-406), by nearest rank (marks/README.md:103-104): p001, p90, p99.9, mean and the loss
    share against PM-0001-3's P1 bands, each `would PASS/FAIL (reported)`, and per direction whether E's line and the
    p99.9 line agree;
  - the release thread's CPU in cores, beside M13's 0.27 (advisor-u3q.md:269) and c10's 0.393 / 0.414;
  - the slack now and at start, whether it was set, and the scheduling policy (`header_numbers`, `header_texts`:
    socket.rs:312-366);
  - /proc/loadavg at the start and the end (as delay_tests.rs:3091-3094).
- **Asserts only run conditions**, each failure message saying "the run is INVALID, not a lateness verdict":
  - the rate held: > 9,900 per second per direction over the send window (as :616; wall-clock, so a condition, never a
    lateness result: u3d-scout-r1.md F20);
  - no queue, pool, oversize or shaper drops (as :617-620); every datagram accounted for (:623); no send error or
    would-block (:629-632); the poller's count equals the delivered count (:633); the fate log never full
    (`l2_fate_log_full_count` 0);
  - the loss inside a 5-standard-deviation normal band of the profile's 1 % (sd 99.5 at 1,000,000). Not the exact
    `band` of delay_tests.rs:164-193: it starts from q^n, and 0.99^1,000,000 ≈ e^−10,050 underflows to 0;
  - the slack read back 1 at start under SCHED_OTHER (0 accepted under a real-time policy: SERIES.md:470);
  - below 100 s it prints "short run: under -3's 1,000,000 per direction" and does not assert the count.

### 3.2 M13's run stays as it is
`l2_sustains_10000_per_second_each_way` keeps P2, seed 11, the thread poller, its prints and its asserts word for
word: it is U3b's evidence and the c10 comparison. The load part both runs share (bind, pacers, sink, poller, join)
may move into one private helper taking the profile, seed, length, poller kind and fate-log capacity; the critics
check that M13's asserts and print formats are unchanged in the diff.

### 3.3 Four fast tests (no `#[ignore]`: they run in check.mjs's test step on both builds, check.mjs:1121-1128)
Table in §4.

### 3.4 The dry run's script (`spikes/netcode-proof/results/u3d-timing/run.sh`)
- **Header:** the commit, `rustc -V`, the test binary's SHA-256, the date (UTC), `nproc`, and U3-Q9's facts, each
  read or written "unreadable", never guessed:
  - kernel: `uname -r`;
  - clocksource: `/sys/devices/system/clocksource/clocksource0/current_clocksource` (and `available_clocksource`);
  - cpuidle: `/sys/devices/system/cpu/cpuidle/current_driver` and `current_governor`;
  - CONFIG_HZ: `^CONFIG_HZ=` from `/boot/config-$(uname -r)`, else from `zcat /proc/config.gz`; **never**
    `getconf CLK_TCK`, which is USER_HZ (u3d-scout-r1.md F17).
  - This container, read 2026-11-09: 6.18.44-fc-v310, tsc, driver none, governor menu, /boot empty, /proc/config.gz
    present with CONFIG_HZ=250 (u3d-scout-req.md C8; my notes N22).
- **Three rounds, interleaved:** the self-test (P1, 100 s), then M13's run (P2, 10 s), each between `uptime` and
  `head -1 /proc/stat` before and after, with the steal share computed beside the raw lines: Δsteal / Δ(user + nice +
  system + idle + iowait + irq + softirq + steal), the 8th field being steal (proc(5); not re-read here, no network:
  medium-high confidence; the raw lines stay in the record so anyone can recompute). Here: "cpu  3942 0 905 321302
  8145 0 155 529 0 0" → steal 529 ticks (N15).
- **Commands** as c10's: `cargo +1.99.1 test --release --locked … -- --ignored --nocapture --exact`, target dir under
  `.gate/`. About 6-7 minutes in all.
- The output goes to `results/u3d-timing/selftest-<UTC>.txt`, the runs' own output unchanged, like c10's file.

### 3.5 SERIES.md: two row texts, no new name
`second.v.l2_late_up_excess_ns` (:427) and `second.v.l2_late_down_excess_ns` (:452) say "(Mustafa's bar: about 10 µs
or less at P1, judged on RS at R1)" on a per-second value, whose L0 is that second's own median. They become: "per
second, reported; the bar (about 10 µs at P1, U3-Q1 (c)) is judged on the P1 leg's whole window, `event.l2_summary`'s
field of the same name, at R1". The `l2_summary` paragraph (:391-396) gains the same sentence.

### 3.6 Plan text (the main session, committed before any code merges)
- **§12.1 U3 row** (plan.md:1221): adds "U3d: the timing self-test: ImpairSocket at P1, 20,000 releases per second,
  inbound polled from a tokio task as quinn polls; reported here, judged on RS at R1."
- **U6 row** (:1225), four carries:
  - **C-U6-1** The probe's inbound poller is a task on npt-server's multi-thread runtime calling `poll_recv` with
    quinn's 32 buffers, like quinn's endpoint driver, so -3's L2 up series and the self-test's up E include the hop
    quinn pays. The pacer stays a dedicated thread (SERIES.md:405-406).
  - **C-U6-2** One `l2_summary` per profile leg: one ImpairSocket per leg, or the run window reset at each leg's start
    (the run window never resets: impair.rs:246, :682; only the second's does, :738).
  - **C-U6-3** If the probe takes its per-datagram delays from the fate log, it is sized for the leg (about 112 MB per
    1,000,000-per-direction leg) and drained only at the leg's end: a mid-leg drain holds the socket's lock
    (socket.rs:427-429) and would add lateness to what it measures.
  - **C-U6-4** The probe process writes `second.v.steal_permille` as npt-server does (SERIES.md:98).
- **U8 row** (:1227), ImpairSocket's R1 self-test as §5.2 gives it, plus:
  - **C-U8-1** U3-Q9's four facts by shell before any timing judgement, with the dry run's commands;
  - **C-U8-2** /proc/stat read before and after the probe's P1 leg (DelayStream's run already has it);
  - **C-U8-3** DelayStream's loaded self-test (48 connections, P2, room running) has no hours in R1's row
    (plan.md:790): U8 sizes it (u3d-scout-r1.md F6).
- **§15** gains U3D-BAR's reading if the advisor rules one.

### 3.7 What U3d does not build
| Not built | Why | Where it is carried |
|---|---|---|
| A self-test binary or npt-lab subcommand | U6's probe runs the same load on RS; new self-test modes are unplanned surface (u3-verify-timing.md:195; plan.md:588-589) | U6 (C-U6-1..4) |
| npt-log names for steal or U3-Q9's facts | No check reads them; Q9 puts them in the run plan; CONFIG_HZ here is only in /proc/config.gz (a gzip reader would be a dependency) | U8 (C-U8-1, -2); per-second steal U6 (C-U6-4) |
| A lateness assertion or a marks.json mark | "Reported only" (u3-build-plan.md:45-46); marks.json holds plan §6.2's marks only (check.mjs:1027-1052) | R1, U3D-JUDGE |
| A spin lead, or code applying SCHED_FIFO | M14 (advisor-u3q.md:270-273); U3-Q2 (b) "never a spin" (checkpoint :474-475) | The miss path (U3D-MISS) |
| A per-release split for ImpairSocket | A scale check (advisor-u3c-plan.md:163); no traced need yet | A U3D-MISS step, if the advisor asks |
| A run-window reset on ImpairSocket | The probe's design is U6's (§4.7 lists no such item) | U6 (C-U6-2) |
| A second E field (L0 = 0) in stats.rs | Only if U3D-L0 (b) is ruled | U6, if ruled |
| A live stall bite | Each piece is proven: E against brute force (stats.rs:537), only beyond the allowance (:593), windows (:613, :656), both stamps (socket_tests.rs:184-237); a live verdict bite cannot discriminate in a container that already misses (u3d-scout-r1.md F25) | Not carried |
| A real quinn endpoint at 10,000 per second each way | U3D-POLL (c) | Not carried |
| UP-STAMP 1-3; DelayStream's RS run; the C10-B fallback; S14; M12; M15 | U6 (plan.md:1225); U8 (:1227); R1; R1's run plan (u3-build-plan.md:47); done in U3b | As listed |

## 4. Tests and their bites
Each bite is planted once in a scratch copy (a script in this folder, as u3c-build-b-bites.py) and must fail its own
test for its own reason.
| Test | What it proves | Fixed inputs | Bite that must fail it |
|---|---|---|---|
| T1 `the_task_poller_takes_lateness_where_the_task_runs` | The new poller measures where a tokio task polls: the hop enters L (RQ2) | Fixed 20 ms one way, no jitter (`fixed(20)`, :17-26); a 1-worker runtime; the poller Pending with its waker stored; one datagram in; once drained (`first_seen`) a second task blocks the only worker for 110 ms. Asserts the up lateness in [50 ms, at − first − 20 ms] (the bound of :207-211) | The poller on its own thread (the old `recv`): it polls at the due time, the lateness stays near 0 |
| T2 `the_fate_log_and_the_histogram_agree` | The exact forms and -3's cross-check read the same samples as the judged E | P1 (jitter, 1 % loss, the reorder step), seed fixed, 2,000 per second per direction for 1 s through the task poller. Asserts per direction: delivered fates = `l2_late_*_count`; the fate log's E with L0 = the summary's p50 within 1,000 ns of `l2_late_*_excess_ns` (stats.rs:3-8's bound) | L taken as release − arrival (the delay, not the lateness) fails E; lost fates not filtered out fails the count |
| T3 `the_verdict_line_is_per_direction_and_inclusive` | The would-line applies "≤ 10,000 ns" per direction, never pooled | E 10,000 → PASS; 10,001 → MISS; up 10,001 with down 0 → up MISS, down PASS | `<` for `≤` fails at 10,000; a pooled mean (5,000.5) fails the mixed case |
| T4 `nearest_rank_matches_the_judge` | -3's cross-check ranks as npt-judge does (⌈q × n⌉) | 36,000 values give the 35,640th for p99 (npt-judge eval.rs:1595-1597; marks/README.md:103-104); 1,001 values give the 1,000th for p99.9 | ⌊q × n⌋ (gives the 999th) or an interpolated quantile |

T1 and T2 inject latency, jitter and loss and decide on identities and a lower bound that a blocked worker sets, never
on how fast this container is. T3 and T4 are pure. The self-test itself (§3.1) asserts only run conditions.

## 5. Evidence and what is judged
### 5.1 Here: the dry run (reported, not judged)
- The main session runs §3.4's script after the merge, quiet (no agent or build alongside, nothing else on loopback),
  release build; the output is committed under `results/u3d-timing/`.
- Reported per run and direction: n, p50, p99, p99.9, max, over; E (histogram, L0 = p50); E exact with L0 = p50 and
  with L0 = 0; the would-line; -3's P1 cross-check (p001, p90, p99.9, mean, loss) and whether it agrees with E's
  line; the CPU against 0.27 core; slack and policy; steal share; loadavg. Once per record: U3-Q9's facts.
- M13's three P2 runs are reported beside c10's (same code path and seed).
- Expected: misses (§0). A miss here is reported to the advisor in U3d's report, not retried into a pass
  (plan.md:683) and not a stop (advisor-u3c-plan.md:163-165).

### 5.2 At R1, on RS (carried to U8's row; judged by the advisor if U3D-JUDGE (J3) is ruled)
- **Where:** the -3 L2 probe's P1 leg in R1's L2 probe block (plan.md:790; ADR 0001:1590): npt-server's probe mode,
  its own ImpairSocket, 10,000 per second each way on RS (20,000 releases per second). 0 added hours.
- **Judged, per direction, over the P1 leg's whole window** (its `l2_summary`, at least 1,000,000 sent per direction):
  E ≤ 10,000 ns (U3D-BAR), in the form U3D-L0 names.
- **Run conditions** (a failure makes the self-test INVALID, never a pass): -3's own count; slack 1 at start and at
  header time, or 0 under the recorded real-time switch on a kernel that carries ed4fb6d7ef68 (SERIES.md:470); no
  queue, pool, oversize or shaper drops; no send errors; the steal reads and U3-Q9's facts present ("unreadable"
  recorded counts as present; a missing read does not).
- **Reported:** p50, p99, p99.9, max, over, the other E form, the release thread's CPU, slack and policy, steal (the two
  reads and the per-second row), U3-Q9's facts, loadavg; a max over 10 ms flagged with the result (U3D-MISS).
- **Ground truth beside it:** -3's own L2 P1 checks, judged by npt-judge as always, with -3's bites.

## 6. SERIES.md and marks.json
- SERIES.md: the two row texts and one sentence of §3.5; no new name, so `every_header_name_has_a_series_row`
  (socket_tests.rs:387-429) is untouched.
- marks.json: unchanged (no new mark, no new check).

## 7. Order and build route
1. **Plan text** (§3.6, §3.5): the main session, committed before the code merges.
2. **Build** (one agent, general-purpose, opus, xhigh, isolated worktree; or the main session itself, the change being
   one test file): §3.1-§3.4. Ends with check.mjs 9/9 (counts stated: measured, bites, skips), clippy -D warnings
   with and without features, fmt, Cargo.lock unchanged; notes in `.checkpoints/netcode-proof-test/u3d-build.md`.
3. **Bites** (§4): planted by a scratch script, each failing its own test; the output kept in the notes.
4. **Merge**, then check.mjs 9/9 and `node tools/gate.mjs --no-smoke`, a commit and a checkpoint.
5. **The dry run** (§3.4, §5.1) by the main session, one evidence run at a time.
6. **One review round on the final diff** (§9), fixes, then U3d's report to the advisor with the numbers reported.

## 8. Risks
1. **This container decides nothing at the µs level** (advisor-u3c-plan.md:114): the release thread's own OS wake
   carries tails as large as the whole (advisor-u3c-plan.md:155-157), and one long pause decides E (u3-verify-timing.md:
   134-139). Mitigation: three runs reported, no verdict, RS judges.
2. **The fate log adds work to the measured path:** about 112 MB reserved at bind; pages touched for the first time
   while the socket's lock is held, roughly 27,000 4 KB pages over 100 s, each a minor fault of a few µs (an estimate,
   not measured), far below the 0.25 ms allowance. The probe at R1 likely does the same (C-U6-3), so it is the faithful
   choice; if a critic wants it shown, one round runs with the log off (E only).
3. **The tokio poller is an idle runtime's hop and does no QUIC work:** under a room's load the hop differs, and
   quinn's per-datagram processing delays its next poll. R1's probe is idle too, so the self-test matches -3; the
   game's real load is measured by DelayStream's loaded run, not this one. Recorded as a limit.
4. **The container runs both ends:** the up pacer and the down sink (LG's job at R1) share the 4 CPUs here, so the
   container is the harder case.
5. **Touching M13's run** could change U3b's evidence: its asserts and prints stay word for word (§3.2).
6. **"About 10 µs" is not a number npt-judge or a reader can apply** (marks/README.md:109-110): U3D-BAR.
7. **E's two forms can straddle the bar** (§2): U3D-L0; -3's own p99.9 is the tie-breaker.

## 9. Change type, evidence rule, reviewers
- **Change type:** netcode (the L2 measurement instrument), test code only. Evidence rule: deterministic tests with
  injected latency, jitter and loss (T1, T2; T3 and T4 pure), each bite failing; the dry run reported, not judged.
- **Gate:** check.mjs 9/9 on both builds, clippy -D warnings with and without features, fmt, `node tools/gate.mjs
  --no-smoke`, Cargo.lock unchanged.
- **Reviewers on the final diff:** netcode-architect (the measurement point, the windows, what R1 judges),
  rust-reviewer (the tokio poller, the test code, clippy), test-engineer (is it proven: bites, run conditions, the
  record). Not security-auditor: no network input path, no dependency, no production code.

## 10. Questions for the advisor
- **U3D-BAR, the exact bar.** "About 10 µs" is not a number anyone can apply.
  (a) E ≤ 10,000 ns per direction over the P1 leg's whole window: the derivation gives exactly 0.001 × 2a = 10,000 ns
  at P1 (u3-verify-timing.md:45-48); "about" is the derivation's own approximation (L independent of the delay), not
  a tolerance; (b) a margin, for example ≤ 11,000 ns; (c) leave it "about" and read it each time.
  **Recommend (a)**, recorded in §15 as a reading. If you read it as changing Mustafa's bar, it goes to him.
- **U3D-L0, which E is judged.** M1 defines E with L0 = the window's median (advisor-u3q.md:235); the recommendation
  Mustafa agreed to says E[(L − 0.25 ms)+] (u3-scout-summary.md:50-52), and since nothing subtracts L0, that form is
  the exact predictor of -3.
  (a) Judge M1's form as built; report L0 = 0 beside it; when the two straddle 10,000 ns, -3's own P1 p99.9 decides
  the reading; (b) judge L0 = 0: needs a second E field in stats.rs, names and SERIES rows (or the probe computing it
  from the fate log at U6); (c) make the impairer subtract its median (no: it changes the delays -3 checks).
  **Recommend (a):** M1 binds (u3-build-plan.md:9), the gap is small (about 0.24 µs in c10's run 1), and -3's own
  check is judged anyway. The dry run will show the gap with the tokio hop.
- **U3D-POLL, the inbound poller.** (a) Keep the parked thread; (b) a tokio task on a multi-thread runtime with
  quinn's 32-buffer batch, and the same shape carried to U6's probe (C-U6-1); (c) a real quinn endpoint at 10,000
  per second each way (QUIC's acknowledgements and congestion control change the release counts; heavier; no traced
  need). **Recommend (b):** it adds the hop quinn pays (about 10-19 µs of E here) without new surface.
- **U3D-JUDGE, how R1 judges.** (J1) setup checks in -3's L2 P1 part (would turn a -3 FAIL into INVALID, or make
  -3 stricter than its ADR row); (J2) R1's run script computes the verdict, with its own fixtures (U8); (J3) you read
  the P1 leg's `l2_summary` against the exact bar, as you read c10. **Recommend (J3):** the self-test checks L2's
  instrument rather than being a plan mark (check.mjs:1027-1052), its miss goes to you anyway, and plan §6.1's "bites
  first" covers automated marks (plan.md:680-682). The would-line (T3) makes the reading mechanical.
- **U3D-MISS, the miss path at R1.** (a) The numbers, steal and U3-Q9's facts come to you before any L2 mark counts;
  the first remedy is the recorded switch (`chrt -f` on the release thread, `release_thread_id` socket.rs:268),
  measured as its own recorded leg, never a retry over the miss (plan.md:683); a remedy that needs a spin lead goes to
  Mustafa (U3-Q2 (b)); a max over 10 ms is flagged with the result, as for DelayStream (advisor-u3c-plan.md:115-116);
  (b) a miss stops every L2 mark until a remedy passes; (c) report only: -3's own check decides L2.
  **Recommend (a).**
- **U3D-XCHECK, -3's checks from the same run.** (a) Keep them, with the fate log on (about 112 MB, risk 2):
  predictor and checked value on the same samples, as the verifier's m3 runs did; (b) drop them: smaller, but the
  container's E then has nothing to be read against. **Recommend (a).**

## 11. For Mustafa
Nothing, unless the advisor reads U3D-BAR as changing his bar.
