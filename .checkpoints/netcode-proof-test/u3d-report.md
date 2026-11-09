# U3d report: ImpairSocket's timing self-test

Written 2026-11-09 about 04:10 UTC by the main session; **folded at the close** (about 04:30 UTC) with the advisor's
close ruling (advisor-u3d.md, "Close ruling: U3d"): **U3d CLOSED, and U3 (U3a-U3d) with it**, closing head ef2bcef
(spike code 1ccff06, the dry run's record 78b0c07). For the advisor, then Mustafa.

## What happened
- U3d built the timing self-test the plan asked for: test code only (1ccff06). It runs ImpairSocket at P1 (the
  tightest profile any check judges), 10,000 datagrams per second each way, inbound polled the way quinn polls, and
  prints the judged excess (E with L0 = 0, U3D-L0) against the bar (10,000 ns per direction, U3D-BAR), beside
  PM-0001-3's own L2 P1 checks computed from the same samples.
- Three reviewers approved it with findings and no regression; a fix pass closed every finding, checked complete.
- The dry run here: in both valid rounds the self-test **would miss** the bar in both directions, and **-3's own
  p99.9 check would fail with it every time**: the predictor and the check it predicts agree in all 12 lines of the
  valid rounds. Reported, not judged: this container is no evidence for a µs-level bar; RS judges at R1.
- One round was **INVALID**, correctly: 87 incoming datagrams never reached ImpairSocket; the kernel's UDP counters
  show exactly 87 receive-buffer drops since boot. In that round's complete direction (down) the self-test said MISS
  while -3's p99.9 said PASS: **the first disagreement**, explained by a cap in -3's tail (below). It corrects the
  advisor's reading that the self-test predicts -3 "exactly" (R-31, amended); the bar does not change, and the
  correction is on the strict side.

## What was built (1ccff06; the plan: u3d-plan.md rev 2, the advisor's APPROVE WITH CONDITIONS, advisor-u3d.md)
- `l2_timing_self_test` (#[ignore]): P1, seed 11, 10,000 per second each way for NPT_L2_SELFTEST_SECS (1-600 s,
  default 100; a non-UTF-8 or out-of-range value is INVALID before bind). Inbound polled by a task spawned on a
  4-worker multi-thread runtime, quinn's 32 buffers of 1,472 B until Pending, the poller defined inside
  `start_poller` (condition 1). The fate log pre-touched before the run (asserted); during the run only `try_send`
  and `poll_recv`; every print before any assert. Asserts are run conditions only (INVALID, never a verdict): rate,
  pre-touch, log completeness, ImpairSocket's own drops (queue, pool, oversize, shaper: socket_tests.rs:1654-1656;
  a kernel drop is caught only by the next assert, as round 3 showed), every datagram accounted for, the integer loss
  band, no send error, slack (1 under SCHED_OTHER; 0 or 1 under SCHED_FIFO/RR at both reads).
- Seven fast tests T1-T7, each with bites (the task poller's hop; the fate log against the histogram; the per-
  direction inclusive verdict line; nearest rank and the integer loss band; the exact excess on fixed samples; -3's
  cross-check reading delay plus lateness; a held lock making both directions late).
- `results/u3d-timing/run.sh`: the dry run, three interleaved rounds, with U3-Q9's facts, THP, and every `^cpu` line of
  /proc/stat around each run (read in full by the main session before it ran).
- SERIES.md: the two per-second excess rows name the judged form (condition 2). M13's run unchanged (sha256 of its
  lines equal). No production code, public item or dependency.

## Proof
| What | Result |
|---|---|
| Reviews of the build | netcode-architect, rust-reviewer, test-engineer: APPROVE WITH FINDINGS, no regression (u3d-build-review-*.md) |
| Fix pass | six items (the slack condition under a real-time policy, shown by a `chrt -f -p` run; log completeness before the loss band; the pre-touch asserted; non-UTF-8 length; T6's agree branches and loss denominator; early fates counted after the prints): the test-engineer's check complete, no gap, no regression (u3d-fix-review-test.md). The slack rule's one deviation (0 or 1 at the start read too under FIFO/RR, since a process born under `chrt -f 1` reads 0) accepted by the advisor as the plan's own reading |
| Bites | 22 of 22 at the build, 16 of 16 at the fix pass, each failing its own test for its own cause, every restore sha256-checked |
| check.mjs (main folder, own cache) | all 9 steps pass: 267 measured, 281 bites (260 / 274 + 7), 0 failed, 0 skips; self 43 of 43; redact 150 items, 0 findings. Reproduced by the advisor on ef2bcef: 9/9, 267 / 281, redact 151 items |
| Gate | GATE PASS with --no-smoke (5a skipped: Chromium/Playwright mismatch; CI's smoke job runs it), reproduced by the advisor on ef2bcef; CI `gate` and `smoke` success on 1ccff06 and 78b0c07 |

## The dry run (results/u3d-timing/selftest-20261109T035556Z.txt, 78b0c07; code 1ccff06, header commit f50bd54 = 1ccff06 plus checkpoint files)
Header: kernel 6.18.44-fc-v310; clocksource tsc (available tsc, kvm-clock); cpuidle driver none, governor menu;
CONFIG_HZ 250 (/proc/config.gz); THP madvise; nproc 4; binary sha256 c236b449... (equal to the fix review's clean
build); loadavg 1.17 at start.

| Run (UTC) | Up: E L0 = 0 / -3 p99.9 | Down: E L0 = 0 / -3 p99.9 | Other | Steal |
|---|---|---|---|---|
| SELFTEST 1 (03:56:16-03:57:57) | 17,067 ns MISS / 31,202,395 ns FAIL: agree | 11,540 ns MISS / 30,413,484 ns FAIL: agree | every other -3 check would pass (count; loss up 0.999 %, down 0.991 %; mean; p001; p90); CPU 0.405 core; slack 1/1 SCHED_OTHER | 0.95 % |
| M13 1 (03:57:57-03:58:08, P2) | E 40.3 µs, p99.9 13.9 ms, max 19.0 ms | E 45.7 µs, p99.9 15.9 ms, max 26.4 ms | CPU 0.385 core | 0.53 % |
| SELFTEST 2 (03:58:08-03:59:48) | 19,728 ns MISS / 31,719,157 ns FAIL: agree | 12,450 ns MISS / 30,383,571 ns FAIL: agree | other -3 checks pass; CPU 0.398 core | 0.62 % |
| M13 2 (03:59:48-03:59:59) | E 52.8 µs, p99.9 17.4 ms, max 20.8 ms | E 55.9 µs, p99.9 19.3 ms, max 24.2 ms | CPU 0.389 core | 0.77 % |
| SELFTEST 3 (03:59:59-04:01:40) | **INVALID** (up: 999,913 of 1,000,000 accounted for); printed E 15,655 ns MISS / 30,750,757 ns FAIL: agree; max 34.17 ms | printed E 11,383 ns MISS / **30,055,981 ns PASS: disagree** (all three forms); max 34.21 ms | kernel Udp RcvbufErrors 87 = the 87 missing; CPU 0.397 core | 0.46 % |
| M13 3 (04:01:40-04:01:51) | E 4.27 µs, p99.9 1.9 ms, max 4.1 ms | E 3.35 µs, p99.9 1.8 ms, max 4.2 ms | | 0.43 % |

- **The first disagreement and its cause (the advisor's close finding).** Round 3's down direction was complete
  (count 1,000,000), and its E said MISS while -3's p99.9 said PASS. The derivation's own form is P = E[min(1,
  (L − 0.05a)+ / 2a)] (u3-verify-timing.md:46): -3's tail counts a datagram's excess only up to 2a = 10 ms, while E
  counts all of it, so E overstates whenever a lateness passes 2a + 0.25 ms (10.25 ms at P1). Round 3's max was
  34.2 ms; one ramp that size at about 9,900 delivered per second adds about 5,760 ns to E but only about 2,900 ns
  capped, which puts down near 8,500 ns, under the bar, as -3's p99.9 shows. In rounds 1 and 2 (maxes 20.5 and
  26.7 ms) the capped value stays near 11,000 ns, over the bar, and -3 failed. (Arithmetic on the printed max; round
  3's fate log was not kept.) So the self-test can MISS where -3 passes, never the reverse; R-31 is amended to say so.
- The three E forms agree to within about 0.5 µs in every run (the impairer subtracts no median, so L0 = 0 is a little
  above L0 = p50, as U3D-L0 expected); the fate log's E equals the histogram's to the ns.
- In the valid rounds -3's p99.9 sits 134-1,469 µs above its band's edge here; every other -3 L2 P1 check would pass.
- M13's rounds 1 and 2 are much worse than c10's (E up 11.6 / 16.9 µs then); round 3 is the best up E yet (4.3 µs; for
  down, c10's run 1 had 2.672 µs, c10-diagnostic :62). The container's timing swings between minutes: the same code,
  path and seed.
- **The CPU share** (0.39-0.41 core) is above M13's U3b reference of 0.27 core. Part is the self-test's denominator (the
  netcode-architect's F6): its 530 ms idle tail is 0.5 % of the self-test's denominator against about 5 % of M13's, so
  the self-test reads about 4-5 % (about 0.02 core) higher than M13 for the same load. The rest of the gap to 0.27 is
  this container: M13 itself reads 0.385-0.390 here.
- **The in-progress commits.** The session's stop hook asks for a commit whenever the record is uncommitted, so the
  record was committed and pushed four times while the run was going: 03:57:04 (in SELFTEST 1), 03:58:16 and 03:59:36
  (in SELFTEST 2), 04:00:49 (in SELFTEST 3), each pushed about 2 s later. None fell inside an M13 run (nor did an
  autosave: 03:53:03, then 04:03:48), and M13 1 and 2 had the worst tails of the record anyway, so the commits are not
  the only source of tails; they cannot be ruled out for SELFTEST 3's 34 ms stall (the record has no time for the
  max). Next time the run goes in the background without ending the turn, so the hook never commits during a run.
- **The UDP counters.** The main session read /proc/net/snmp at 04:02 UTC (Udp InErrors 87, RcvbufErrors 87); the
  advisor read it again at 04:05:56 (87 and 87, unchanged); rmem_default 212,992 B, rmem_max 4,194,304 B. The
  counter is per network namespace, not per socket; down's sink got all its delivered datagrams, so the 87 were
  dropped on ImpairSocket's inbound socket; no earlier run since the container started had a drop. That round 3 was
  one stall hitting both directions at once is consistent with the record (maxes 34.17 and 34.21 ms), not shown.

## Known limits
- This container is not evidence for the µs-level bar; RS judges at R1 (U3D-JUDGE (J3)). The miss here is reported,
  not retried and not a stop (advisor-u3c-plan.md:163-165).
- The dry run cannot attribute E to the tokio hop (the advisor ruled no control round, U3D-POLL (b)).
- **A stall of ImpairSocket's own reader** delays t_in (socket.rs:607, :613), so it never enters L, E or -3's L2
  delays: under about 28 ms (the kernel's receive buffer, about 280 small datagrams at 10,000 per second) it leaves
  no trace; over that it shows only as kernel drops, which make the leg INVALID, as round 3 showed.
- E counts excess past 2a + 0.25 ms that -3's p99.9 caps, so after one long stall the self-test can MISS where -3
  passes (never the reverse); at R1 that case goes the U3D-MISS route as written, with -3's p99.9 and the max flag
  beside it.

## Carried forward
- The plan's carries stand as written into plan §12.1 (U3 row; U6 C-U6-1..7; U8 C-U8-1..6) and §15 R-31 (amended at
  the close).
- **U3D-RCVBUF (the advisor's ruling): C-U8-6.** Before and after every L2 probe leg (P1-P4 and B1), in the probe's
  network namespace and outside the measured path: the probe socket's own `drops` column in /proc/net/udp or
  /proc/net/udp6 (matched by its local port), and /proc/net/snmp's Udp lines (and /proc/net/snmp6's Udp6 when
  present). A drop on the probe's P1 socket between the two reads makes the self-test INVALID with its cause named;
  that leg comes to the advisor before any L2 mark counts, never set aside and re-run as noise. No SO_RCVBUF sizing
  without the advisor's ruling (quinn leaves the OS default; a bigger buffer would erase the only trace of a long
  reader stall).

## Decisions needed
- None. U3D-CLOSE and U3D-RCVBUF were the advisor's (both APPROVE, with conditions met at the close).

## For Mustafa
1. LEFTOVER-DELETE, still open (the 4 GB folder and nine small scratch copies `cargo clean` refuses).
