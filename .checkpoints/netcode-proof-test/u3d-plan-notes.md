# U3d plan drafting notes (netcode-architect), 2026-11-09
Branch claude/netcode-proof-test-plan-hcky5g (head 49bd1ac = 95dac9c + checkpoint-only commits). Read and write notes
only; no code, builds, runs or network. Deliverable: .checkpoints/netcode-proof-test/u3d-plan.md (rev 1).
No earlier notes file existed (checked: no u3d-plan*.md in the folder at start).

## Plan
1. Read both scouts' notes (u3d-scout-req.md, u3d-scout-r1.md) and u3c-plan.md's shape. DONE at start.
2. Spot-check the load-bearing claims I build on (file:line): u3-build-plan.md:45-46; checkpoint :468-481;
   advisor-u3q.md M1/M13/M14; advisor-u3c-plan.md :101-104, :112-116, :163-167, :191-192; socket_tests.rs:507-639
   (M13 run), :55-88 (poller); stats.rs Summary/summarize; socket.rs run_summary/header; impair.rs late_run;
   l1 lib.rs P1/P2; plan.md :1221-1227, :680-683, :790; check.mjs :1027-1052, :1121-1128; SERIES.md :388-473;
   marks/README.md :109-110, :183; c10-diagnostic lines; quinn endpoint driver; delay_tests.rs:3077-3097.
3. Decide the smallest U3d: files, symbols, not-built list with carries (U6, U8, R1).
4. Tests with bites; dry-run evidence; R1 judged items; SERIES/marks rows; risks; change type; reviewers; questions.
5. Write u3d-plan.md rev 1; DONE line here.

## Findings (one line each)
- N1 No prior draft; scouts' notes both end DONE (u3d-scout-req.md:131, u3d-scout-r1.md:232). Critics to follow:
  u3d-plan-review-test.md, u3d-plan-review-rust.md (checkpoint .checkpoints/netcode-proof-test.md:515-517).
- N2 Checked: U3d line u3-build-plan.md:45-46; mapping :10-12 ("U3d M1, M13, M14"). Mustafa :468-469 (no number);
  session record :473-474 (Q1 (c)), :481 (Q9). Hold.
- N3 Checked the M13 run socket_tests.rs:507-639: P2 fixed :519-520; seed 11 :521; pacers :530-549 via pace :485-505
  (1 ns slack :486-488); poller `quinn_side` :561-571 = recv :62-88 (std thread, park/unpark :55-60, ONE buffer :67-70);
  asserts :616-637; prints :579-614 (slack printed :611-613, policy not). Holds.
- N4 quinn-0.11.12 spawns EndpointDriver as a runtime task (endpoint.rs:152-160), polls socket.poll_recv with
  BATCH_SIZE buffers (:781-794); BATCH_SIZE = 32 on Linux (quinn-udp-0.5.16 unix.rs:804-806); recv_limiter yields
  (endpoint.rs:768, :852). Workspace tokio =1.54.1 has rt-multi-thread + macros (Cargo.toml:55). So a tokio-task poller
  with 32 buffers is buildable test-side, no new dependency.
- N5 poll_recv stamps `now` once at entry under the lock (socket.rs:501) and finishes every datagram of the batch with
  it (:525): with 32 buffers, all due datagrams in one call share one stamp (fine: they are handed over at that call).
- N6 Statistic: stats.rs:3-8, ALLOWANCE_NS :20-21, Summary :51-62, summarize :108-159 (c = p50 + 0.25 ms :129-130).
  run_summary socket.rs:394-415 (late_run); take_second :371-390. header_texts :351-366 carries l2_sched_policy.
- N7 The probe (SERIES.md:401-406): npt-server probe mode, own ImpairSocket, pacer a dedicated 1 ns thread "never
  tokio (M13)", LG's npt-bots sends up and echoes. It names the PACER only; its inbound poller is unspecified: a U6 carry.
- N8 c10-diagnostic (results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt): ImpairSocket run 1 :61-64 (up E 11.610,
  down 2.672 us; 0.393 core), run 2 :119-122 (up 16.921, down 12.939 us; 0.414 core); losses 2,074 up / 1,983 down in
  both (seed 11 deterministic); commands :4-10; no steal, no policy, no Q9 facts. Holds.
- N9 P1 = 50 ms, jitter 10 ms, 1 % per direction (l1 lib.rs:118-121; loss per direction :95); per-direction half
  jitter = jitter/2 = 5 ms (l1 lib.rs:156-158; impair.rs:105, :122-123); one way = (rtt - base)/2, base 0 by default
  (config.rs:99, :142-144). So -3's L2 P1 bands: p999 [29.75, 30.25] ms, p001 [19.75, 20.25], p90 [28.75, 29.25], mean
  [23.75, 26.25] (marks.json PM-0001-3 checks), count >= 1,000,000 per direction (parts "L1 P1", "L2 P1"; L2 part
  selects process "probe").
- N10 Fate log carries t_in_ns, due_ns, out_ns per datagram (stats.rs:426-443; 56 B canonical :447); Lost never reaches
  finish (finish only for popped datagrams, impair.rs:653-697), so Delivered entries = the lateness samples; capacity
  fixed at bind, Vec::with_capacity (stats.rs:476-480), default 0 (config.rs:109); drain under the shared lock
  (socket.rs:427-429). So a test can compute exact E (any L0) and -3's delay checks (out - t_in, the probe's stamps
  per SERIES.md:401-406) from the same run. 2 x 1,000,000 entries x 56 B = about 113 MB at bind; container 16 GB.
- N11 RISK found: the exact binomial `band` (delay_tests.rs:164-193) starts from q.powi(n): at n = 1,000,000, p = 0.01
  it underflows to 0 (0.99^1e6 ~ e^-10050), so it cannot give P1's loss band at the self-test's length: the builder
  needs a log-space or normal (k sd) band. P1 at 1e6: sd = sqrt(1e6 x 0.01 x 0.99) = 99.5.
- N12 marks/README.md:183's "steal is recorded and reported, never a condition" is PM-0006-13's rule, not a general one
  (:181-183); V-S makes steal <= 10 permille a validity rule only for marks naming V-S (:174-177). The self-test's steal
  rule is the advisor's "Record steal" (advisor-u3c-plan.md:191-192): recorded. Scout A's citation of :183 is a stretch.
- N13 SERIES.md per-second excess rows (`second.v.l2_late_{up,down}_excess_ns`) say "over the second, L0 its median
  (Mustafa's bar: about 10 us or less at P1, judged on RS at R1)": misleading, since each second has its own L0 and the
  bar is the run window's (stats.rs:129-130). A row-text fix (no name change) belongs in U3d.
- N14 F24 matters more with a task poller: the impairer never subtracts L0 (grep config/impair: no offset), so -3's
  check sees L, and E[(L - 0.25 ms)+] (L0 = 0) is the exact predictor; with a tokio hop p50 rises (DelayStream's hop
  p50 0.065-0.070 ms: c10 :27, :85), widening the gap between the two E forms. Report both; the judged form is a
  question (M1 defines L0 = p50: advisor-u3q.md:235).
- N15 Container: 4 CPUs, 16.5 GB (nproc; /proc/meminfo); /proc/stat cpu line has 10 fields, steal the 8th (read
  2026-11-09: "cpu  3942 0 905 321302 8145 0 155 529 0 0" -> 529).
- N16 check.mjs: expectedMarks only plan marks (check.mjs:1027-1052); test step runs cargo test without --ignored on
  both builds (:1121-1128), so #[ignore] runs never run there; non-ignored helper tests do.
- N17 -3 at L2 runs legs P1-P4 and B1, 1,000,000 packets per profile and direction (ADR 0001:1590; marks.json:752,
  :848, notes :2348), so the P1 leg is about 100 s at 10,000/s and the self-test can ride it (plan.md:790 "L2 probe
  0.2"); the l2_summary must be per leg (late_run never resets: impair.rs:246, :682; only late_second resets :738).
- N18 Derivation's words: the verifier's reading Mustafa agreed to says E[(L - 0.25 ms)+] with no L0
  (u3-scout-summary.md:50-52); L0 enters with M1 (advisor-u3q.md:235); the derivation assumed the impairer subtracts L0
  (u3-verify-timing.md:45-46). Since nothing subtracts it (N14), L0 = 0 is the exact predictor: a question (U3D-L0).
- N19 The predictor agreed with -3's P1 p999 in every m3 run the verifier made (u3-verify-timing.md:126-149: E 11.9/
  11.6 FAIL just, 16.8/17.3 FAIL, 6.8/6.7 PASS, 34.9/35.2 FAIL, 9.2/9.4 PASS): calibration evidence for the bar; the
  dry run's -3 cross-check repeats it on U3d's own samples.
- N20 U1's root rule: the advisor's Q10 (advisor-u1.md:89-92): "worse than 0.25 ms (5 % of P1's half jitter) goes back
  to the advisor before R1"; the 2026-11-08 ruling moved the judging to RS (advisor-u3c-plan.md:163-165).
- N21 Decisions for the draft: test-only (socket_tests.rs) + a results script + SERIES.md row text (:427, :452) + plan
  text; no pub item, no log name, no dependency; Q9 facts and steal by shell in the dry run's script (R1: the run plan,
  U8; per-second steal by the probe process, SERIES.md:98, U6); poller a tokio task with quinn's 32 buffers; fate log
  on for the exact E forms and -3's cross-check; 4 fast tests in check.mjs (task-poller stamp, fate-log identity,
  verdict line, nearest-rank) each with a bite; no live stall test (pieces proven: stats.rs:537/:593/:613/:656,
  socket_tests.rs:184-237).
