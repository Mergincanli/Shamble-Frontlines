# U3d scout: what U3d (the timing self-test) must contain, and what exists (netcode-architect)
Branch claude/netcode-proof-test-plan-hcky5g, head 95dac9c. Read-only: notes only, no code, no builds, no runs.

## Plan
1. Requirement sources: u3-build-plan.md:45-46 and :5-12; advisor-u3q.md M1, M12-M15, :30; Mustafa's U3-Q1 (c) and
   U3-Q9 words (checkpoint "Facts from Mustafa" / "Decisions taken"); advisor-u3c-plan.md 2026-11-08 rulings
   (condition 10, diagnostic, UP-STAMP, ImpairSocket up E ~10 us); plan.md §12.1 U3/U6/U8 rows + other self-test,
   R1 run plan and U3-Q9 mentions.
2. The code as built: l2/socket.rs (ImpairSocket, release thread), socket_tests.rs (M13 run), stats.rs (E),
   delay_tests.rs (lateness_evidence_run), crates/lab (probe binary), marks.json + SERIES.md, crates/log, ops/runs/.
3. Latest outputs: results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt.
4. Per requirement: source (file:line quoted), code (file:line), gap, here-vs-RS. Smallest U3d; engine-feature flags.

## Notes
- R-SRC1 build plan: u3-build-plan.md:45-46 "U3d, the timing self-test (U3-Q1 (c)): expected excess lateness beyond
  0.25 ms at 20,000 releases/s, inbound measured where quinn polls, p50/p99/p99.9/max reported; here it is reported
  only (RS judges at R1, U3-Q9 facts read)." Mapping :10-12 "U3d M1, M13, M14" (also listed under U3b :10).
- R-SRC2 Mustafa's words (checkpoint .checkpoints/netcode-proof-test.md:468-469, verbatim): "I agree with the
  recommendations on U3-Q1, Q2, Q4, Q5, Q6, Q7, Q8 and Q9." The session's record of what that means (:473-474):
  "Q1 (c) expected excess lateness beyond 0.25 ms under about 10 us at the tightest profile, at 20,000 releases/s on
  RS, percentiles reported"; (:481) "Q9 RS's kernel, HZ, cpuidle and clocksource read at R1 (R1's run plan)".
  The questions as asked: u3-scout-summary.md:66-67 (Q1: "p99 or p99.9 (offset recorded), or the expected excess at
  P1; measured at 20,000 releases/s; inbound where quinn polls; on RS at R1") and :81 (Q9: "RS facts to read at R1
  before judging timing: kernel (6.8 GA or HWE), CONFIG_HZ, cpuidle driver, clocksource"). The verifier's reading
  behind (c): :50-52 "E[(L - 0.25 ms)+] about 10 us or less at P1 (the tightest band) ... 10,000/s per direction =
  20,000 releases/s ... inbound ... where quinn polls, not in the release thread". NOTE: Mustafa's literal words
  carry no number; "about 10 us" and "the tightest profile" come from the recommendation (summary :50-52).
- R-SRC3 advisor M1 (advisor-u3q.md:233-236), M13 (:267-269), M14 (:270-273), M12 (:265-266), M15 (:274-276); the
  course change :30-34 (Mustafa's answers decide).
- R-SRC4 advisor 2026-11-08 (advisor-u3c-plan.md): condition 10 RULING (a) ACCEPT :97-100; carried to R1 :101-104
  ("DelayStream's self-test on RS under the WS part's real load (48 connections, P2, room running), >= 10,000 samples
  per direction; judged E under about 10 us ... and p99 <= 1 ms per direction; p99.9 and max reported; each release
  split ..."); "condition 10 again" :112-116 ("a max over 10 ms there comes to me with the result"); the diagnostic
  ordered :117-121 (ImpairSocket's M13 lateness run interleaved, "reported, not judged"); diagnostic read :142-167,
  ImpairSocket :163-165 ("a scale check, not like for like: its lateness covers its whole path (send, the kernel's
  loopback, recv) at 10,000 per second each way, its thread at 0.39-0.41 of a core. Its own up E (11.6 / 16.9 us) is
  now over U3-Q1 (c)'s "about 10 us" in this container too: U3d reports it, RS judges it."); E does not add across
  parts :166-167; C10-B fallback order :175-180; UP-STAMP item 5 :191-192 ("Record steal at R1: /proc/stat's steal
  read before and after each DelayStream and ImpairSocket self-test run (two reads, nothing in the measured path).");
  UP-STAMP items 1-3 :182-188 = U6 (WsServer reader stamp), NOT U3d. Plan text change :193-196 (U6/U8 rows).
  FINDING: the steal rule says "at R1", so in U3d it is a facility (the self-test run records steal before/after) that
  R1 uses; U3d's dry run here records it too (cheap; the container's /proc/stat has a steal column: :150 "steal
  0.47 %").
- RESUME (run 2, 2026-11-09, head 32563de = 95dac9c + checkpoint-only commits): R-SRC1..4 re-checked at the cited
  lines: all hold. Checkpoint :511-515 lists U3d's inputs the same way; :814 "U3-Q1's E (< about 10 us) is met" is
  DelayStream's run 1, not ImpairSocket.
- R-SRC5 the bar's origin: u3-verify-timing.md:42-48: -3's P1 up p999 band [29.75, 30.25] ms (marks.json, series
  second.s.probe_up_delay_ns, now :785-860) holds iff E[(L' - 0.25 ms)+] <= about 0.001 x 10 ms = 10 us AT P1 (P1 =
  50 ms, +-10 ms: npt-l1 lib.rs:118-121; a = 5 ms each way). So the 10 us budget scales with the jitter width: P2's
  would be about 20 us. -3's L2 series come from "npt-server's probe mode ... from npt-net's ImpairSocket stamps"
  (SERIES.md:111-116); the probe is "a free slot port with its own ImpairSocket, 10,000 packets per second per
  direction" (docs/adr/0001-multiplayer-netcode.md:1590). The hand-off to a task adds about 10-19 us of E here
  (u3-verify-timing.md:155, :158-159; design problem 4 :190-192).
- R-SRC6 plan.md: the U3 row (:1222) names U3a-U3c, no U3d. U8 (:1227) has DelayStream's RS self-test and bar, and
  ImpairSocket only inside the steal clause ("steal recorded (/proc/stat read before and after each DelayStream and
  ImpairSocket self-test run)"). 0 hits for U3-Q9, cpuidle, clocksource, CONFIG_HZ. -3 at R1: plan.md:695 "R1 and the
  start of every L2 block (R-6's probe)"; hours: :781 "L2 probes (R-6) 1.0: R1 0.2", R1 row :790 "L2 probe 0.2".
  §4.7 npt-net public list :580-585 (ImpairSocket, DelayStream, L2Config; rest pub(crate)); binaries :588-589.
  E18 (:466) already lists "steal and affinity reads" as instrumentation (server, net).
- C1 E statistic: stats.rs:3-8 (doc), :20-21 ALLOWANCE_NS 250,000, :51-62 Summary (n, p50, p99, p999, max, excess,
  over), :108-159 summarize (L0 = the window's own p50, :129-130). Samples: impair.rs:680-684 (every fate but Oversize,
  outbound send errors included). Windows: run (socket.rs:394-415 run_summary, impair.rs late_run :246, never reset)
  and per second (socket.rs:371-390). FINDING: Q1 (c)'s recorded words say "beyond 0.25 ms" with no L0; L0 = p50 is
  M1's (advisor-u3q.md:235) and the verifier's C15 offset. U3d's plan must say the judged window is the run's
  (run_summary), not a second's (each window has its own L0).
- C2 measurement points: inbound socket.rs:501 (`now` read once per poll_recv call, under the lock) and :525
  (finish Delivered with it); module doc :19-21 says "at poll_recv's return" (the gap is the copy loop); outbound
  :722-726 (stamp after try_send); arrival at drain :607. Test of the inbound point: socket_tests.rs:181-212 (with a
  bite in its doc :180-182). M1's points: MET.
- C3 the M13 run: socket_tests.rs:507-639, #[ignore] (:511), NPT_L2_SECS (:513-516, default 10 s), Profile::p2()
  fixed (:519-520), pacers on dedicated threads with 1 ns slack (pace :485-505), inbound poller `quinn_side` (:561-571)
  = the test's `recv` helper (:62-88): a std thread parked/unparked by a Wake (Unpark :55-60), one buffer per call.
  Asserts throughput > 9,900/s (:616), no queue/pool/oversize/shaper drops (:617-620), accounting and P2's 2 % loss
  (:621-627), no send errors (:629-632); lateness, CPU, slack only printed (:579-614). Never asserts lateness: right
  for "reported only".
- C4 GAP "where quinn polls": quinn polls from EndpointDriver, a spawned tokio task (quinn-0.11.12 src/endpoint.rs:
  152-160, :366-378), poll_recv at :794 with BATCH_SIZE buffers. M13's thread poller omits the tokio wake-to-poll hop
  (the hop DelayStream's split shows at p50 0.065-0.070 ms: c10-diagnostic :27, :39, :85, :97). Smallest fix: the
  self-test polls poll_recv from a task on a multi-thread tokio runtime (poll_fn), with quinn's batch of buffers.
  A real quinn::Endpoint at 10,000/s each way needs QUIC traffic (congestion control, ACKs change the release
  count): heavier, no extra need traced.
- C5 GAP "tightest profile": M13 runs P2 only (:519); the bar is P1's (R-SRC5). P1 keeps jitter, loss and the reorder
  step on (lib.rs:103-121, reorder PART_13 :110), so one P1 run meets M13's words too; its loss check must come from
  the profile (1 %), not P2's fixed 2 % (:624-627).
- C6 M14 DONE in U3b: thread.rs:1-41 (own_tid :20-26, set_own_slack :39), sched :58-84, cpu_ns :86-; release thread
  socket.rs:648-656, header-time re-read :667-676; header_numbers :312-348, header_texts :351-366 (l2_sched_policy);
  release_thread_id pub :268 (so chrt applies from outside); tests thread.rs:99-140, socket_tests.rs:353-386; SERIES
  .md:468-473. Small gap: the M13 run prints the slack (:604-613) but not the policy (header_texts): one line.
- C7 GAP steal: neither evidence run reads /proc/stat (M13 nothing; delay_tests.rs:3091-3094 /proc/loadavg only).
  second.v.steal_permille is npt-server's per-second row (SERIES.md:98, U6). The container's /proc/stat "cpu" line has
  the steal field (8th value; read 2026-11-09: 243 ticks). README.md:177-178 (V-S: steal <= 10 permille, a validity
  rule for marks naming V-S) and :183 ("steal is recorded and reported, never a condition"): the ruling says record.
- C8 GAP U3-Q9 facts: kernel is a header field (README.md:28; npt-lab machine.rs:67 reads osrelease). HZ, cpuidle,
  clocksource: read nowhere. This container (read 2026-11-09 with cat/zcat, no code): kernel 6.18.44-fc-v310,
  CONFIG_HZ=250 (only in /proc/config.gz; /boot is empty), cpuidle driver "none" (governor menu), clocksource tsc.
  Ubuntu keeps CONFIG_HZ in /boot/config-<release>; a Rust reader of /proc/config.gz would need a gzip dependency.
  So: a shell step in R1's run plan (U8), as U3-Q9 says, and in U3d's dry-run record; no code, no log names.
  Why it matters: SERIES.md:470 (slack reads 0 under a real-time policy since ed4fb6d7ef68, 6.12 and 6.6.84).
- C9 the container's numbers (c10-diagnostic, f6cd1d7, quiet, P2, 10 s, n about 98k/dir): run 1 up E 11.610 us
  p99 163.4 p99.9 3,457.4 max 8,100.3 us; down E 2.672 us (:62-63); run 2 up E 16.921 us p99 526.5 p99.9 4,242.1 max
  7,986.9 us; down E 12.939 us p99.9 4,124.0 max 8,207.7 us (:120-121); release thread 0.393 / 0.414 core (:64,
  :122) against M13's "0.27 core here at lead 0" (advisor-u3q.md:269); slack 1. FINDING: run 2's DOWN E (12.9 us) is
  over about 10 us too; the advisor's line (:163-165) names only up. No steal, no policy, no Q9 facts in that file.
- C10 R1 route: no binary runs ImpairSocket's self-test on RS today (npt-lab is L1-only and does not depend on
  npt-net: crates/lab/Cargo.toml; main.rs:1-12). -3's L2 probe (U6, npt-server's probe mode: SERIES.md:401-406) runs
  the same load (10,000/s each way, dedicated 1 ns pacer) on RS at R1 and its ImpairSocket's run_summary carries
  l2_late_* (SERIES.md:422-428, :447-453). So RS's judged run can be the probe's P1 leg (0 added hours). U6 carries:
  one ImpairSocket (or a reset run window) per profile leg, since late_run never resets (impair.rs:246, :682;
  socket.rs:401); the probe's inbound poller a tokio task like quinn's driver (C4).
- C11 GAP plan text: U8 (plan.md:1227) needs ImpairSocket's RS self-test: where (the -3 probe's P1 leg, or its own
  leg), load (10,000/s each way), window (the run's), JUDGED per direction E < about 10 us (U3-Q1 (c)), REPORTED p50,
  p99, p99.9, max, release CPU, slack, policy; steal before/after; Q9's four facts. Not ruled: a sample floor (DelayStream
  has >= 10,000/dir) and whether "a max over 10 ms comes to me" (advisor-u3c-plan.md:115-116) covers ImpairSocket
  (here max 8.1-8.2 ms).
- C12 NOT U3d: UP-STAMP 1-3 (U6, plan.md:1225), DelayStream's RS run under 48 connections (U8 :1227), C10-B fallback
  (R1), S14 kernel check (u3-build-plan.md:47), M12 pass-through (U3b, socket_tests.rs:295-351), M15 rows (U3b; U3d adds
  none if it only prints).
- C13 ENGINE-FEATURE FLAGS (no traced need: REVISE if a plan adds one): a self-test binary or npt-lab subcommand
  (U6's probe already runs it on RS; §4.7 binaries plan.md:588-589; the verifier's design problem 6,
  u3-verify-timing.md:200-201); new npt-log header or second names for HZ/cpuidle/clocksource/steal (Q9 puts them in
  the run plan; they would need SERIES rows and a gzip reader); a lateness assertion or a marks.json mark here
  (u3-build-plan.md:46 "reported only"; a mark is ADR text); a lead/spin self-test (M14: dropped,
  advisor-u3q.md:272-273); code applying SCHED_FIFO (M14: from outside); a per-release split for ImpairSocket (the
  advisor called it a scale check, :163); a pub run-window reset on ImpairSocket (U6's call, §4.7).
- VERDICT: U3d is mostly packaging. Smallest U3d: test-only edits to the M13 run (or a sibling #[ignore] test sharing
  pace): P1 with the loss check from the profile; the inbound poller a tokio task with a batch of buffers; /proc/stat
  steal before and after the measured part; l2_sched_policy and /proc/loadavg printed. One dry run here, committed under
  results/ with the container's four Q9 facts (shell) and the commands. Plan text for U8 (C11) and U6 (C10). The
  report: E per direction, both directions over about 10 us here (C9), reported; RS judges. No production code, no pub
  item, no log name, no dependency.
DONE
