# U3d scout (test-engineer): how R1 runs, records and judges U3d's self-test

Branch claude/netcode-proof-test-plan-hcky5g, head 95dac9c. Read-only: no code, no builds, no runs, no network.
Memory: no .claude/agent-memory/test-engineer folder exists (checked 2026-11-08).

## Plan
1. u3-build-plan.md:5-12, :45-46 (U3d line, M-mapping M1, M13, M14).
2. advisor-u3q.md M1 (:233-236), M12-M15 (:265-275), course change :30; Mustafa's U3-Q1 (c) and U3-Q9 answers (checkpoint).
3. advisor-u3c-plan.md rulings 2026-11-08: condition 10, diagnostic, UP-STAMP, ImpairSocket up E ~10 us.
4. plan.md §12.1 U3, U6, U8 rows (~:1220-1230); other mentions of self-test, R1 run plan, U3-Q9.
5. Code: l2/socket.rs, socket_tests.rs (:505+), stats.rs, delay_tests.rs; crates/lab; marks/marks.json, SERIES.md; crates/log; ops/runs/.
6. results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt.
7. Synthesis: where R1 expects it; what recorded where; judged vs reported; output format; U3d's evidence here; bites; exists vs gaps.

## Findings (one line each, file:line)
- F1 U3d's line: u3-build-plan.md:45-46 "expected excess lateness beyond 0.25 ms at 20,000 releases/s, inbound measured where quinn polls, p50/p99/p99.9/max reported; here it is reported only (RS judges at R1, U3-Q9 facts read)". Mapping U3d = M1, M13, M14 (:12-13). S14 kernel check carried to R1's run plan (:47).
- F2 Mustafa's answers recorded: checkpoint .checkpoints/netcode-proof-test.md:468-481 ("Decisions taken"; "So: Q1 (c) ... under about 10 us at the tightest profile, at 20,000 releases/s on RS, percentiles reported" :473-474; "Q9 RS's kernel, HZ, cpuidle and clocksource read at R1 (R1's run plan)" :481). His literal words: "I agree with the recommendations on U3-Q1, ... and Q9" (:468-469); the "So:" is the session's record of those recommendations. Original Q list: u3-scout-summary.md:65-77 (Q1 :66-67, Q9 :77).
- F3 M1 (advisor-u3q.md:233-236): inbound at poll_recv's return, outbound at try_send's return, arrival stamped at kernel drain; both directions E[(L-L0-0.25ms)+] beside p50/p99/p99.9/max; "on loopback here it is a dry run, RS decides (Q1, Q9)". M13 (:266-268): 10,000/s each way sustained, jitter+loss+reorder on, release thread CPU recorded (0.27 core at lead 0). M14 (:269-271): slack 1 ns via /proc/<tid>, read back and tested, RT priority a recorded switch, no spin lead; T3 lead self-test dropped. M12 (:263-265) pass-through same path; header records mode. M15 (:272-274) SERIES rows for slack, sched policy, lateness per direction, writing binary named.
- F4 advisor-u3c-plan.md 2026-11-08: DelayStream self-test carried to R1 (:101-104): RS, 48 conns, P2, room running, >=10,000 samples/dir; JUDGED E < ~10 us AND p99 <= 1 ms per direction; p99.9 and max reported; max > 10 ms to advisor (:114-116). UP-STAMP item 5 (:191-192): steal from /proc/stat before and after each DelayStream AND ImpairSocket self-test run, nothing in measured path. ImpairSocket up E 11.6/16.9 us here "U3d reports it, RS judges it" (:163-165). E does not add across parts (:166-167). Fallback order for a DelayStream miss (:177-180).

## Resume 2026-11-09 (after the spend-limit cut; branch head now 32563de, checkpoint-only commits after 95dac9c)
- Re-checked F1-F4 against the files. Corrections: F1's mapping is u3-build-plan.md:10-12 (not :12-13). F2's Q9 as asked
  is u3-scout-summary.md:81 (not :77); Q1 :66-67, the verifier's reading :50-52. F3's lines are advisor-u3q.md M12
  :265-266, M13 :267-269, M14 :270-273, M15 :274-276 (M1 :233-236 right; course change :30-34). F4 lines confirmed
  (advisor-u3c-plan.md :101-104, :112-116, :163-167, :175-180, :191-196).
- F2 nuance: Mustafa's literal words (.checkpoints/netcode-proof-test.md:468-469) carry no number; "under about 10 us at
  the tightest profile" is the session's record of the recommendation (:473-474), from u3-scout-summary.md:50-52.
- F5 plan.md §12.1: the U3 row (plan.md:1221) names U3a, U3b, U3c only, no U3d; U3d exists only in u3-build-plan.md:45-46.
  The U8 row (plan.md:1227) carries R1's run plan with S14's veth+netem check and DelayStream's self-test on RS (48
  conns at P2, room running, >= 10,000 samples/dir, JUDGED per direction E[(L - L0 - 0.25 ms)+] < ~10 us and p99 <= 1 ms;
  REPORTED p50, p99.9, max; split from U3c b5c9009 + U6's reader stamp; steal /proc/stat before+after each DelayStream
  AND ImpairSocket self-test run; max > 10 ms to the advisor; a miss to the advisor before -18/-31 WS parts count;
  fallback order). U6 row (plan.md:1225) carries UP-STAMP 1-3 (reader stamp; U6's loopback WsServer self-test reports
  the four parts before the run package). GAP-P1: plan.md has no row text for ImpairSocket's self-test at R1 (its
  bar, its sample count, its load) beyond the steal clause; no "U3-Q9" anywhere in plan.md (grep: 0 hits for U3-Q9,
  clocksource, cpuidle, CONFIG_HZ); grep "lateness" hits only :1225 and :1227.
- F6 plan.md's R1 hours (§7.2 :790): "L2 probe 0.2; calibration 0.1"; §7.1 :781 "L2 probes (R-6) 1.0: R1 0.2, R2 0.2, R3
  0.4, R7 0.2". §6.2 :695 "-3 | C (L1, U2); R1 and the start of every L2 block (R-6's probe)". So ImpairSocket's
  self-test naturally rides R-6's L2 probe block (PM-0001-3's L2 half). GAP-P2: DelayStream's self-test under 48 conns
  with the room running (U8 row) has no hours in R1's row (:790); it needs the room and bots (LG), i.e. it is more than
  the 0.2 h probe. U3d's plan should name where it sits (R1's run plan, U8) and size it; not U3d's to build.
- F7 What exists for ImpairSocket's self-test (code at 32563de; spikes/netcode-proof/crates/net/src/l2/): the M13 run
  `l2_sustains_10000_per_second_each_way` (socket_tests.rs:507-639) is a lib #[test] #[ignore] (:510-511), length from
  env NPT_L2_SECS (:513-516, default 10 s), P2 (:519-521), paced on dedicated threads with 1 ns slack (pace :485-505).
  It ASSERTS only throughput > 9,900/s (:616), zero queue/pool/oversize/shaper drops (:617-620), accounting (:623),
  P2's 2 % loss within 0.4 points (:624-627), no send errors (:629-632). Lateness, CPU and slack are PRINTED free text
  only (:579-614), never asserted, never written to a log. It reads no steal, no loadavg, no U3-Q9 fact.
- F8 Inbound fidelity gap (M1, "inbound measured where quinn polls"): ImpairSocket stamps inbound at poll_recv
  (socket.rs:494-536: `now` read under the lock at entry :501, `finish(.., Delivered, now, ..)` :525; module doc
  :19-21; stats.rs:3-4 says "at poll_recv's return"). In M13 the poller is the test's `recv` helper (socket_tests.rs:
  62-88): a dedicated OS thread parked/unparked by a Wake (Unpark :55-60), not quinn's endpoint driver on a tokio worker.
  So M13's up lateness omits tokio's wake-to-poll hop (the very part the C10 diagnostic separated for DelayStream,
  c10-diagnostic:27, :39). The real-quinn tests (wt_tests.rs:68-125 check_p2; :249-251, :434-436) run 200 datagrams at
  30 Hz (:40, :49) and assert only the sample count = delivered + refused (:109-115), no rate, no lateness printout.
  GAP-C1: no run measures inbound lateness through a real quinn endpoint at 20,000 releases/s; U3d must either make
  one or state (with a reason) that the thread poller stands in, for R1 to judge against.
- F9 The E statistic (stats.rs): fixed histogram, 1 us buckets to 8.192 ms then 8/octave (:3-8, :13-31); Summary has n,
  p50, p99, p999, max, excess = E[(L - p50 - 0.25 ms)+], over (:51-62); L0 is THE WINDOW'S OWN p50 (:6-7, :125-126),
  so a per-second window's E differs from the run's: R1's bar must name the run window (`run_summary`, socket.rs:394-405)
  not `take_second` (:371-386). ALLOWANCE_NS 250,000 (:20-21). Nearest-rank quantiles as bucket means (:111-124).
  Per the advisor (advisor-u3c-plan.md:166-167) E does not add across parts.
- F10 What exists for DelayStream's self-test: `lateness_evidence_run` (delay_tests.rs:3077-3097), a lib
  #[tokio::test(multi_thread, 4 workers)] #[ignore] (:3084-3085), ticks from env NPT_WS_LATENESS_TICKS (default 10,050,
  :3087-3090), loopback (one connection, P2, 30 Hz), reads /proc/loadavg before/after (:3091-3094), asserts only
  >= 10,000 samples/dir (:3096). Prints `late_line` with condition 10's TRIGGER p99.9 > 1 ms or max > 10 ms
  (:195-198, :200-217): NOT R1's bar (E < ~10 us and p99 <= 1 ms; max > 10 ms to the advisor; plan.md:1227). The split
  lines (part_line :219-231, split_lines :233+) are reported, never judged. No steal read, no npt-log output.
- F11 The latest outputs (results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt, f6cd1d7, release build, quiet):
  ImpairSocket M13 run 1 (:61-64): up n 97,926 p50 27.5 p99 163.4 p99.9 3,457.4 max 8,100.3 us E 11.610 us over 791;
  down n 98,017 p50 32.5 p99 97.4 p99.9 1,250.4 max 3,092.4 us E 2.672 us; release thread 0.393 core; slack 1.
  Run 2 (:119-122): up p99 526.5 us p99.9 4,242.1 max 7,986.9 E 16.921 us; down p99 286.7 p99.9 4,124.0 max 8,207.7
  E 12.939 us; 0.414 core. 10 s each (n ~98k/dir). DelayStream (:24, :36, :82, :94): E down 11.5/16.6 us, up 26.8/35.4
  us; up p99 1.439/1.772 ms (over R1's p99 <= 1 ms); max up 22.281 ms. The record format is free text plus `uptime`
  lines added by hand (:11-13), commands in the header comment (:4-10). No steal line (the advisor read steal 0.47 %
  since boot separately, advisor-u3c-plan.md:150).
- F12 Where the bar comes from (the judging logic R1 must keep): the verifier derived U3-Q1 (c) from -3's L2 P1 p99.9
  band (u3-verify-timing.md:42-48): P1 is 50 ms, +-10 ms, 1 % (crates/l1/src/lib.rs:119-121), so the per-direction half
  jitter a = 5 ms and -3's 5 % tolerance on a is exactly 0.25 ms (marks.json PM-0001-3 "jitter's upper end (99.9th
  percentile)" P1 band 29,750,000-30,250,000 ns); the p99.9 upper check holds iff E[(L' - 0.25 ms)+] <= ~0.001 x 10 ms
  = 10 us. So "the tightest profile" = P1, and the ~10 us bar is a PREDICTOR of -3's L2 P1 p99.9 check, which is the
  ground truth (the container's m3 run at 10,000/s/dir: E 11.9/11.6 us and -3 P1 p999 30.457/30.475 ms FAIL "just",
  u3-verify-timing.md:131-133). GAP-C2: M13 runs at P2 only (socket_tests.rs:519-520); U3-Q1 (c)'s statistic is
  defined at P1 (checkpoint :473-474 "at the tightest profile"). U3d's self-test should run P1 (P2 too, if cheap) and
  report beside E the -3 P1 p99.9 check computed from the same run's delays, so the predictor and the mark agree.
- F13 -3's row (marks.json, id PM-0001-3; notes field): "L2's count runs on the probe: a free slot port with its own
  ImpairSocket, 10,000 packets/s per direction, one-way delays stamped on RS's clock (`process` = probe)", "at the start
  of every L2 block". ADR 0001's -3 row says the same (docs/adr/0001-multiplayer-netcode.md:1590). -3 has NO lateness
  check (grep marks.json for late/excess: 0 hits); its L2 parts select process "probe". So R1's L2 probe (plan.md:790,
  0.2 h) is the natural host of ImpairSocket's self-test: same socket, same 20,000 releases/s, same RS.
- F14 The probe's writer (marks/SERIES.md:401-406): "npt-server's binary in a probe mode (process `probe`) ... its own
  ImpairSocket, writes every probe series from the socket's stamps on RS's clock ... its pacer is a dedicated thread
  with 1 ns slack, never tokio (M13). LG's side, npt-bots (probe), sends up, echoes" - "proposed at U3b, built at U6".
  So the R1 self-test binary is U6's npt-server probe mode, NOT a cargo test: RS runs the run package's binaries
  (plan §10.5, U8). GAP-R1: the self-tests today are #[ignore] lib tests (socket_tests.rs:510-511; delay_tests.rs:
  3084-3085) that need cargo and the source tree; no binary or ops/runs/ script runs them, and no npt-log file comes
  out. U3d's plan must say which: (a) U3d only proves the measurement and its record format on a test harness here and
  hands U6 the writer, or (b) U3d adds a small binary/subcommand now. (a) fits "no generic features for later".
- F15 What R1 records, as the format stands (marks/SERIES.md): per-second second.v.l2_late_{up,down}_{count,p50_ns,
  p99_ns,p999_ns,max_ns,excess_ns,over_count} (:422-428, :447-453) and l2_release_cpu_permille (:462); the `l2_summary`
  event once at the run's end with the same names over every sample plus l2_release_cpu_ns (:390-396); header params
  l2_mode, l2_sched_policy, l2_timer_slack_ns / _start_ns / _set / _error (:468-473). DelayStream: ws_late_* per
  connection, merged by npt-server's writer (U6), the `ws_split` event pooled per process (:495-517), header
  ws_clock_timer_slack_* and ws_clock_sched_policy (:711-713, :726, :730). BOTH sections say "names no mark reads yet"
  (:388 and the DelayStream heading). npt-net writes no npt-log record itself (no npt_log use in crates/net/src/l2;
  the writer is npt-server, U6); socket.rs:309-366 (header_numbers/texts) and :392-405 (run_summary) hand the values.
  npt-judge can read them once written: series forms event.<ev>.<field> and header.params.<name> (marks/README.md:
  93-99); a report may print header.params / header.machine strings (agreed for U4, :132-136).
- F16 GAP-REC1 steal: second.v.steal_permille exists (SERIES.md:98, npt-server, RS, per second) and is PM-0001-1's
  validity check (marks.json:114-131, "RS, single-room runs"), but the advisor's two reads per self-test run
  (advisor-u3c-plan.md:191-192) have no row. /proc/stat's aggregate `cpu` line: steal is the 8th value, in USER_HZ
  ticks (proc(5); medium-high confidence, not re-fetched: no network here). Record before/after ticks and the total
  ticks over the run, so the share is checkable; per-CPU lines too if the release thread's vCPU matters.
- F17 GAP-REC2 U3-Q9: Machine has `kernel` only (crates/log/src/lib.rs:35; container value from
  /proc/sys/kernel/osrelease, crates/lab/src/machine.rs:67). No field or SERIES row for CONFIG_HZ, the cpuidle driver
  and governor, or the clocksource (grep SERIES.md: 0 hits). Machine is deny_unknown_fields (lib.rs:21-22) under SCHEMA 1
  (:16), so the facts belong in header.params (free map, lib.rs:88-89), each with a SERIES row. Paths (u3-scout-timing.md
  :41-42, :593-595, :122-123): /sys/devices/system/cpu/cpuidle/current_driver (+ current_governor),
  /sys/devices/system/clocksource/clocksource0/current_clocksource, CONFIG_HZ from /boot/config-$(uname -r) ("unconfirmed"
  on RS). Container values: clocksource tsc (available tsc kvm-clock), cpuidle driver none, governor menu, HZ 250
  inferred from the 204 ms RTO (u3-scout-summary.md:57), not read. Trap: getconf CLK_TCK is USER_HZ (100), not
  CONFIG_HZ: never record it as HZ. An unreadable fact is recorded as unreadable, never guessed (a gap, not a value).
- F18 GAP-J1 no judge path: marks.json may hold only plan §6.2's marks (check.mjs:1027-1052: ids from ADR 0001's
  PM-0001 rows + PM-0004-67 + PM-0006-11..27; "nothing else"), so a self-test cannot be its own marks.json row without
  changing check.mjs and the plan. Options for R1's "judged": (J1) setup checks (README §2.3 `setup`: failure or gap =
  INVALID, "the run did not test the mark") in the marks the self-test gates: ImpairSocket's E on -3's L2 P1 part (and
  every L2 mark?), DelayStream's E and p99 on -18's and -31's WS parts; check.mjs hashes only the ADR row lines
  (:1053-1066), so the JSON may gain checks, but it is a reading: the advisor's call; (J2) the R1 run script applies the
  bar and prints PASS/MISS, with its own tests and fixtures (ops/runs/ pattern: c-u2-pm-0001-3.mjs); (J3) the advisor
  reads the numbers, as for c10 (advisor-u3c-plan.md:143-151). Plan §6.1 "Bites first: every automated mark's planted
  defect must FAIL before its pass counts" (plan.md:679-682): a JUDGED self-test (J1 or J2) needs its own bite.
- F19 GAP-MISS no miss path for ImpairSocket's self-test: the U8 row gives DelayStream's (to the advisor before -18/-31's
  WS parts count; fallback order, plan.md:1227; advisor-u3c-plan.md:175-180), nothing for ImpairSocket. Its remedies are
  bounded by Mustafa's U3-Q2 (b): RT priority is "a recorded switch", "never a spin" (checkpoint :475-476), so a miss
  only a spin lead would fix goes to Mustafa, not the advisor. Also on 6.12+ / 6.6.84+ an RT thread's slack reads 0
  (SERIES.md:470): the RT switch changes the slack reading the setup check expects.
- F20 Timing-dependent assertions in the existing runs (both #[ignore], so outside check.mjs's `test` step, which runs
  cargo test without --ignored: check.mjs:1121-1128): M13's pps > 9,900 (socket_tests.rs:616) is wall-clock (a 100 ms
  stall at the end fails it): at R1 it must be a setup condition (INVALID), not a FAIL. Its loss band +-0.4 points
  (:624-627) is deterministic for seed 11 (lost 2,074 up / 1,983 down in BOTH diagnostic runs, c10-diagnostic:62, :120,
  :63, :121), but it does not scale with NPT_L2_SECS (at 1 s, n 10,000: 2.9 sd); delay_tests.rs:165-193's exact
  binomial band is the pattern. Reorder counts differ by 1 between runs (89,146 vs 89,145): timing-dependent, report only.
- F21 Existing proofs U3d can lean on (not redo): E's arithmetic vs brute force within 1 us (stats.rs:537),
  excess only beyond the allowance (:593), running window = fresh histogram (:613), pooled = single (:656); inbound
  stamped at poll_recv not at the drain (socket_tests.rs:184-212: polled 90 ms after due must show >= 50 ms); outbound
  stamped at the release (:215-237); slack read when the header is built (:353-386); every header and l2_summary name
  has a SERIES row (:387-430). The split's identity and misattribution bites for DelayStream (delay_tests.rs ~:3040-3075).
- (line fixes: "Bites first" is plan.md:680-682, "No retries" :683, fixtures :676; Q2 (b) is checkpoint :474-475.)
- F22 The rule's older root: the advisor's U1 ruling Q10 (advisor-u1.md:89-92): "U3 measures ImpairSocket's release
  precision; worse than 0.25 ms (5 % of P1's half jitter) goes back to the advisor before R1." U3-Q1 (c) then made the
  statistic E < ~10 us, and the 2026-11-08 ruling (advisor-u3c-plan.md:163-165) says the container's E over 10 us is
  "U3d reports it, RS judges it". So U3d's dry-run miss is REPORTED to the advisor in U3d's report (not a stop), and an
  R1 miss goes to the advisor before L2 marks count.
- F23 GAP-BAR "about 10 us" is not a number npt-judge can apply (marks/README.md:113-114: an integer in the unit, an op).
  The derivation gives 10,000 ns exactly at P1 (0.001 x 2a, a = 5 ms; the uniform's own p99.9 sits 0.002a = 10 us
  below d + a: u3-verify-timing.md:45-48). U3d's plan should propose the exact form for the advisor: E <= 10,000 ns per
  direction over the run window at P1 (op and value), p99 <= 1,000,000 ns for DelayStream, max > 10,000,000 ns a flag
  to the advisor (never a verdict), each over the run's histogram (n >= 1,000,000 per direction at P1 for ImpairSocket,
  the -3 count; >= 10,000 per direction merged over 48 connections for DelayStream).
- F24 L0 is not subtracted by the impairer: E uses L0 = the window's p50 (stats.rs:6-7, :125-126), but nothing in
  impair.rs, config.rs or socket.rs subtracts an offset from the injected delay (grep L0/offset/p50: 0 hits), while the
  derivation assumed "a constant offset the impairer may subtract (its measured median lateness)" (u3-scout-timing.md:
  331-333). So -3's p99.9 band sees L, not L - L0: the reported E slightly understates -3's exposure (bounded by about
  p50 x the share over 0.25 ms: ~30 us x 0.8 % = ~0.24 us at c10-diagnostic:62). Small here; report beside E the
  absolute form E[(L - 0.25 ms)+] (or p50 and the share with L > 0.25 ms) so the advisor can bound it at R1.
- F25 Live bites cannot discriminate in this container: the clean M13 run already exceeds 10 us (up E 11.6 / 16.9 us,
  c10-diagnostic:62, :120). So "a planted stall makes it MISS" proves nothing here; the bar's bite must be on fixed
  inputs (verdict function or judge fixtures: PASS at 9,999 ns, MISS at 10,001 ns; per direction, not pooled; the run
  window, not a second's), and a live stall bite (e.g. the test holds ImpairSocket's lock for G = 50 ms once in T = 10 s:
  predicted E rise ~G^2/(2T) = 125 us, the verifier's model u3-verify-timing.md:134-137) checks the predicted RISE
  against its clean twin, not the verdict.
- (F23 fix: the op/value rule is marks/README.md:109-110, not :113-114.)

## Synthesis for the U3d drafter (test-engineer's answer)
### S1 Where R1 runs the self-tests (exists | gap)
- ImpairSocket's: R1's L2 probe block (plan.md:790 "L2 probe 0.2"; :781; -3's notes in marks.json and ADR 0001:1590:
  the probe's own ImpairSocket at 10,000/s/dir on RS = 20,000 releases/s). Binary: npt-server's probe mode, process
  `probe`, built at U6 (SERIES.md:401-406); LG's npt-bots (probe) sends and echoes. Run steps: R1's run plan
  (ops/runs/R1.md, plan §7.2 header :787), U8. GAP: no plan text gives ImpairSocket's R1 bar, profile, count or miss
  path (only the steal clause, plan.md:1227); the U3 row omits U3d (:1221).
- DelayStream's: R1 under the WS part's real load (48 conns, P2, room running, >= 10,000 samples/dir; plan.md:1227),
  i.e. npt-server's WsServer (U6) with npt-bots' WS clients; U6's loopback WsServer self-test first (UP-STAMP 3,
  plan.md:1225). GAP: no hours for it in R1's row (:790).
- Today: both exist only as #[ignore] lib tests (socket_tests.rs:510-512; delay_tests.rs:3084-3086), run by hand with
  cargo, printing free text; no ops/runs/ script (only c-u2-pm-0001-3.mjs, the container pattern to copy).
### S2 What gets recorded, where
- Exists (names + values, writer U6): second.v.l2_late_* and the `l2_summary` event (SERIES.md:390-396, :422-453),
  l2_release_cpu (:462), header l2_mode / l2_sched_policy / l2_timer_slack_* (:468-473); DelayStream ws_late_*, `ws_split`,
  ws_clock_timer_slack_* / ws_clock_sched_policy (:495-517, :711-730). Machine.kernel (log lib.rs:35).
- Missing rows U3d should add (SERIES.md + names + a name test, as M15 did): steal before/after per self-test run
  (ticks, plus total ticks, from /proc/stat's `cpu` line; advisor-u3c-plan.md:191-192); U3-Q9 facts as header.params
  strings (cpuidle driver and governor, clocksource, CONFIG_HZ or "unreadable"); the self-test's own identity (rate per
  direction, profile, seconds, poller kind: thread or quinn). Any new pub symbol goes into plan §4.7 first (M6).
- Committed (plan §10.6): numbers, verdicts, hashes, redacted headers; raw JSONL stays in .gate/ or on the VM.
### S3 Judged at R1 vs reported
- JUDGED, ImpairSocket (U3-Q1 (c)): per direction, over the run window at P1, at 20,000 releases/s: E[(L - L0 - 0.25 ms)+]
  <= 10,000 ns (exact form for the advisor: F23). Setup (INVALID, never PASS): rate held (>= 9,900/s/dir), slack 1 (or 0
  under RT on 6.12+/6.6.84+), steal and U3-Q9 facts present, n >= 1,000,000/dir.
- JUDGED, DelayStream (plan.md:1227): per direction, merged over 48 conns: E < ~10 us and p99 <= 1 ms.
- REPORTED: p50, p99 (ImpairSocket), p99.9, max, over count, E with L0 = 0 (F24), release-thread CPU, steal share,
  U3-Q9 facts, slack and policy, the split (DelayStream). FLAG to the advisor: max > 10 ms (DelayStream rule; propose
  the same for ImpairSocket). Ground truth for ImpairSocket stays -3's own L2 P1 checks (F12).
- Judge path: open (F18: J1 setup checks in the gated marks / J2 run-script verdict with fixtures / J3 advisor reads).
### S4 Output format
- For npt-judge: npt-log JSONL from the R1 binary: header (params above) + `l2_summary` / `ws_summary` + `ws_split`
  events; checks read event.l2_summary.l2_late_up_excess_ns etc. (README §2.3 series forms, :93-99).
- For the advisor (and U3d's dry run here): a committed results file in the c10-diagnostic shape (commands, commit,
  binary SHA-256, `uptime`, the runs' own output unchanged) PLUS per run: steal before/after, U3-Q9 facts, slack/policy,
  and one line per direction "E x ns vs bar 10,000 ns: would PASS/MISS (reported, not judged here)".
### S5 U3d's evidence here (dry run, reported, not judged)
- Quiet container (no agent or build alongside), release build, >= 3 runs interleaved (one-run comparisons decide
  nothing: u3-scout-summary.md:45-47), at P1 (the bar's profile) and P2 (comparable with c10-diagnostic:61-64,
  :119-122), 10,000/s/dir; P1 at 100 s (n ~1,000,000/dir, -3's count and R1's length), each with steal before/after,
  loadavg, U3-Q9 facts, header slack/policy, CPU. Optional, strong: -3's P1 L2 jitter checks (p001, p90, p999, mean)
  from the same run's delays, so predictor and mark are seen together (as the verifier's m3, u3-verify-timing.md:131-133).
- The container miss (up E 11.6/16.9 us) is expected and reported (F22); no retry into a pass (plan.md:683).
### S6 Bites (what proves the self-test can fail)
- Verdict on fixed inputs (deterministic): 9,999 vs 10,001 ns E; per direction (up over, down under -> MISS, a pooled
  verdict is the bite); run window, not a second's (samples where they differ); p99 1,000,000 vs 1,000,001 ns
  (DelayStream); max 10,000,001 ns -> flag; each setup gap -> INVALID, never PASS.
- Live, against a clean twin: a planted release stall (G = 50 ms in T = 10 s -> E rise ~125 us predicted); judged on
  the rise, since the clean run already misses here (F25).
- Already proven, cite don't redo (F21): the E arithmetic (stats.rs:537, :593, :613, :656), the inbound stamp at
  poll_recv (socket_tests.rs:184-212), slack read at header time (:354), names have rows (:388).
### S7 Questions for the advisor (via U3d's plan)
- Q-A the judge path (J1/J2/J3, F18) and whether a judged self-test needs plan §6.1's "bites first" (plan.md:680-682).
- Q-B the exact bar (F23) and whether the thread poller (M13's recv, socket_tests.rs:62-88) stands in for quinn's poll
  at the bar, or U3d must measure through a real quinn endpoint at 20,000 releases/s (F8, GAP-C1).
- Q-C ImpairSocket's miss path at R1 (F19; a spin needs Mustafa: U3-Q2 (b)).
- Q-D whether ImpairSocket gets a split like DelayStream's (release wake vs poller hop) for attribution of an R1 miss:
  cheap, traces to the miss path, but not ruled.
- Q-E R1 hours for DelayStream's loaded self-test (F6, GAP-P2), a U8 item.
DONE 2026-11-09
