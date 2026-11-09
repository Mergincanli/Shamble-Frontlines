# U3d report: ImpairSocket's timing self-test

Written 2026-11-09 about 04:10 UTC by the main session. For the advisor (the U3d close), then Mustafa.

## What happened
- U3d built the timing self-test the plan asked for: test code only (1ccff06). It runs ImpairSocket at P1 (the
  tightest profile any check judges), 10,000 datagrams per second each way, inbound polled the way quinn polls, and
  prints the judged excess (E with L0 = 0, U3D-L0) against the bar (10,000 ns per direction, U3D-BAR), beside
  PM-0001-3's own L2 P1 checks computed from the same samples.
- Three reviewers approved it with findings and no regression; a fix pass closed every finding, checked complete.
- The dry run here: in both valid rounds the self-test **would miss** the bar in both directions, and **-3's own
  p99.9 check would fail with it every time**: the predictor and the check it predicts agree in all 12 lines. That is
  what the self-test is for. Reported, not judged: this container is no evidence for a µs-level bar; RS judges at R1.
- One of the three rounds was **INVALID**, correctly: 87 incoming datagrams never reached ImpairSocket. The kernel's
  UDP counter shows exactly 87 receive-buffer drops since boot (RcvbufErrors 87), during a stall of about 34 ms that
  hit both directions at once. The run condition caught it as designed.

## What was built (1ccff06; the plan: u3d-plan.md rev 2, the advisor's APPROVE WITH CONDITIONS, advisor-u3d.md)
- `l2_timing_self_test` (#[ignore]): P1, seed 11, 10,000 per second each way for NPT_L2_SELFTEST_SECS (1-600 s,
  default 100; a non-UTF-8 or out-of-range value is INVALID before bind). Inbound polled by a task spawned on a
  4-worker multi-thread runtime, quinn's 32 buffers of 1,472 B until Pending, the poller defined inside
  `start_poller` (condition 1). The fate log pre-touched before the run (asserted); during the run only `try_send`
  and `poll_recv`; every print before any assert. Asserts are run conditions only (INVALID, never a verdict): rate,
  pre-touch, log completeness, no host drops, every datagram accounted for, the integer loss band, no send error,
  slack (1 under SCHED_OTHER; 0 or 1 under SCHED_FIFO/RR at both reads).
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
| Fix pass | six items (the slack condition under a real-time policy, shown by a `chrt -f -p` run; log completeness before the loss band; the pre-touch asserted; non-UTF-8 length; T6's agree branches and loss denominator; early fates counted after the prints): the test-engineer's check complete, no gap, no regression (u3d-fix-review-test.md) |
| Bites | 22 of 22 at the build, 16 of 16 at the fix pass, each failing its own test for its own cause, every restore sha256-checked |
| check.mjs (main folder, own cache) | all 9 steps pass: 267 measured, 281 bites (260 / 274 + 7), 0 failed, 0 skips; self 43 of 43; redact 150 items, 0 findings |
| Gate | GATE PASS with --no-smoke (5a skipped: Chromium/Playwright mismatch; CI's smoke job runs it); CI green on 1ccff06 |

## The dry run (results/u3d-timing/selftest-20261109T035556Z.txt, 78b0c07; code 1ccff06, header commit f50bd54 = 1ccff06 plus checkpoint files)
Header: kernel 6.18.44-fc-v310; clocksource tsc (available tsc, kvm-clock); cpuidle driver none, governor menu;
CONFIG_HZ 250 (/proc/config.gz); THP madvise; nproc 4; binary sha256 c236b449...; loadavg 1.17 at start.

| Run (UTC) | Up: E L0 = 0 / -3 p99.9 | Down: E L0 = 0 / -3 p99.9 | Other | Steal |
|---|---|---|---|---|
| SELFTEST 1 (03:56:16-03:57:57) | 17,067 ns MISS / 31,202,395 ns FAIL: agree | 11,540 ns MISS / 30,413,484 ns FAIL: agree | every other -3 check would pass (count, loss 0.999 %, mean, p001, p90); CPU 0.405 core; slack 1/1 SCHED_OTHER | 0.95 % |
| M13 1 (03:57:57-03:58:08, P2) | E 40.3 µs, p99.9 13.9 ms, max 19.0 ms | E 45.7 µs, p99.9 15.9 ms, max 26.4 ms | CPU 0.385 core | 0.53 % |
| SELFTEST 2 (03:58:08-03:59:48) | 19,728 ns MISS / 31,719,157 ns FAIL: agree | 12,450 ns MISS / 30,383,571 ns FAIL: agree | other -3 checks pass; CPU 0.398 core | 0.62 % |
| M13 2 (03:59:48-03:59:59) | E 52.8 µs, p99.9 17.4 ms, max 20.8 ms | E 55.9 µs, p99.9 19.3 ms, max 24.2 ms | CPU 0.389 core | 0.77 % |
| SELFTEST 3 (03:59:59-04:01:40) | **INVALID** (up: 999,913 of 1,000,000 accounted for); printed E 15,655 ns, max 34.17 ms | printed E 11,383 ns, max 34.21 ms | kernel RcvbufErrors 87 since boot = the 87 missing | 0.46 % |
| M13 3 (04:01:40-04:01:51) | E 4.27 µs, p99.9 1.9 ms, max 4.1 ms | E 3.35 µs, p99.9 1.8 ms, max 4.2 ms | | 0.43 % |

- The three E forms agree to within about 0.5 µs in every run (the impairer subtracts no median, so L0 = 0 is a little
  above L0 = p50, as U3D-L0 expected); the fate log's E equals the histogram's to the ns.
- -3's p99.9 sits 130-1,470 µs above its band's edge here; every other -3 L2 P1 check would pass.
- M13's rounds 1 and 2 are much worse than c10's (E up 11.6 / 16.9 µs then); round 3 is the best yet (4.3 / 3.4 µs).
  The container's timing swings between minutes: the same code, path and seed.
- The CPU share (0.39-0.41 core) is above M13's U3b reference of 0.27 core; part is the self-test's denominator,
  which includes about 530 ms after the pacers stop (the rust-reviewer's F6), part is this container.
- **The in-progress commits.** The session's stop hook asks for a commit whenever the record is uncommitted, so the
  record was committed four times while the run was going: 03:57:04 (in SELFTEST 1), 03:58:16 and 03:59:36 (in
  SELFTEST 2), 04:00:49 (in SELFTEST 3), each about a second of git on one of the 4 vCPUs. None fell inside an M13
  run, and M13 1 and 2 had the worst tails of the record anyway, so the commits are not the only source of tails; they
  cannot be ruled out for SELFTEST 3's 34 ms stall. Next time the record is committed only after the run.

## Known limits
- This container is not evidence for the µs-level bar; RS judges at R1 (U3D-JUDGE (J3)). The miss here is reported,
  not retried and not a stop (advisor-u3c-plan.md:163-165).
- The dry run cannot attribute E to the tokio hop (the advisor ruled no control round, U3D-POLL (b)).
- SELFTEST 3 shows a host-level failure the probe at R1 can meet too: a stall longer than the kernel's receive
  buffer (rmem_default 212,992 B here, about 280 small datagrams at 10,000 per second, so about 28 ms) drops
  datagrams before ImpairSocket sees them. The run condition makes such a leg INVALID, as designed.

## Carried forward
- The plan's carries stand as written into plan §12.1 (U3 row; U6 C-U6-1..7; U8 C-U8-1..5) and §15 R-31.
- **New, for the advisor's ruling (U3D-RCVBUF):** the probe at R1 records the kernel's UDP counters (/proc/net/snmp:
  InErrors, RcvbufErrors) before and after each leg, so a host drop is named, not inferred (two reads, outside the
  measured path; as the steal reads). Whether the probe also sizes its socket's receive buffer (SO_RCVBUF, to hold
  a stall of about 100 ms at 10,000 per second) is a question: it changes the socket from what quinn's default gives
  in production, so I recommend recording only, and sizing only if R1's legs go INVALID for this cause.

## Decisions needed
- None for Mustafa. For the advisor: U3D-CLOSE (close U3d, and with it U3, on this evidence?) and U3D-RCVBUF (record
  the UDP counters at R1; size the buffer only on need).

## For Mustafa
1. LEFTOVER-DELETE, still open (the 4 GB folder and nine small scratch copies `cargo clean` refuses).
