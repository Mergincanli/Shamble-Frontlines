# Checkpoint: npt-u3d-skeptic
- **Goal:** a skeptic check of U3d's merged code (spike commit 1ccff06 against d012437: ImpairSocket's timing
  self-test and its seven fast tests, socket_tests.rs +1,057, SERIES.md, results/u3d-timing/run.sh; and the dry run's
  record 78b0c07): netcode-architect and test-engineer in parallel, then one refutation pass by the night main
  session; notes only, no build, no test run, no pull request · plan: item 5 of `night-2026-11-10.md` at 2dc1f31
  (`git show 2dc1f3131c6b20c089045ae8611ad8a883940084:.checkpoints/netcode-proof-test/night-2026-11-10.md`),
  approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u3d-skeptic · **Base:** origin/main 2d22b69
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited)
- **Last update:** 2026-11-11 03:44 Amsterdam
- **State: DONE.** Both notes files and this summary are committed and pushed. The day session folds them into PR #40
  (plan rule 2: no pull request, delete nothing).

## Summary (findings by severity, each with its proof)
**Critical 0 · High 0 · Medium 4 (NA1 = TE1, found by both skeptics; TE2; NA4; NA5) · Low 8 (TE3-TE7, NA2, NA3,
NA6; TE7 and NA6 overlap in part).** 13 finding ids in all (TE1-TE7, NA1-NA6), 12 distinct. U3d has had its two review rounds, so every
finding below is a follow-up for U3d's report (plan rule 5); none opens a third round. Line cites are at df8975b;
`plan.md`, `advisor-u3d.md`, `u3d-report.md` are under `.gate/npt-spike/.checkpoints/netcode-proof-test/`; code is
under `spikes/netcode-proof/crates/net/src/l2/`.

**Refutation pass (the night main session):** no Critical or High to refute. I re-did the arithmetic of the one
finding both skeptics reached on their own (NA1 = TE1), and it holds: at n = 990,091 delivered per direction, -3's
p99.9 fails at 991 or more delays above 30.25 ms; 1,980 datagrams late by 5.25 ms give E (L0 = 0) = 1,980 × 5,000,000
/ 990,091 = 9,999 ns (PASS), while the expected count above is 1,980 × 5,000,000 / 10,000,001 = 990 (so -3 fails
about half the time). I also read socket.rs:726 for TE2: the down stamp is `sh.now_ns()` after `try_send`, as the
finding says.

Medium:
- **NA1 = TE1: R-31's "predicts -3 exactly" and "never the reverse" hold on average over the delay draws, not for
  one run** (plan.md:1423; advisor-u3d.md:214-217, :235-237; u3d-report.md:69, :99-100; the agree line,
  socket_tests.rs:904-910, :1593-1600). The count of delays above T has mean n × E / 2a and a spread of about 18-31
  datagrams (about 180-320 ns of E). At E = 10,000 ns (PASS) -3's p99.9 fails about half the time; at about 10,300 ns
  (MISS) it passes about 10 % of the time with no lateness near 10.25 ms. U3D-MISS has no route for "E PASS, -3 FAIL",
  so a session at R1 would look for a bug that is not there. The bar stands; the fix is R-31's wording plus one more
  U3D-MISS case. Becomes High only if R1's E lands within about 1,000 ns of the bar (the dry run's 11.4-19.7 µs is
  far from it). Proof (day): `python3 -I .checkpoints/npt-u3d-skeptic/na1_sampling.py` (stdlib only, reads and writes
  nothing; written by the netcode-architect, not run at night), and optionally TE1's seed sweep through
  `cross_check` at E = 10,000 ns (expect about 50 % FAIL).
- **TE2: down's send stamp is not proven by any of the seven tests** (socket.rs:726). Changing `sh.now_ns()` to `now`
  (the pop time) passes all seven: T7 pops only after the lock is released, so the stamp is late either way; T1 and
  T2 never read down's stamp. Down's E (and -3's down delays, same stamp) would then lose the poller wake and the
  send calls. T7's doc (socket_tests.rs:1263-1264) and SERIES.md's down rows claim this point is proven. Proof (day):
  plant it, then `cargo +1.99.1 test --locked -p npt-net --lib -- l2::socket::tests::` (expect all to pass).
- **NA4: the release thread's scheduling policy is read only at the run's end** (socket_tests.rs:1465, :1476-1486,
  :1688-1693; socket.rs:351-361). A `chrt -f` switch partway through a run passes and is recorded as a SCHED_FIFO
  run, so U3D-MISS's first remedy could be judged on a mixed leg. Proof: already in the record,
  u3d-fix-review-test.md:38-40 (switch at 1 s into a 6 s run, exit 0).
- **NA5: C-U8-6's /proc/net/udp read can find no row** (plan.md:1247 C-U8-6 and C-U8-4, with C-U6-2;
  u3d-plan.md:386). /proc/net/udp lists open sockets only, and C-U6-2 binds one socket per leg; if U8 reuses the
  steal reads' rule ("unreadable counts as present"), the drop condition is vacuous and a P1 drop goes unseen. Fix
  (U8's text): both reads while the socket is bound; a missing row makes the leg INVALID. Carried (C-U8-6): this
  tightens its wording.

Low:
- TE3: swapping socket.rs:501 and :503 (up's clock read before the lock) passes all seven; up's E would lose lock
  waits. Proof (day): plant it, the same cargo line as TE2.
- TE4: `agree("MISS", true)` is not asserted; `agree()` rewritten as `p999_pass || e_would == "MISS"` would print
  "agree" for round 3's case (the code today is right: the record printed "disagree", selftest-20261109T035556Z.txt
  :253-255). Proof (day): plant it, then `cargo +1.99.1 test --locked -p npt-net --lib --
  l2::socket::tests::the_cross_check_reads_delay_plus_lateness --exact`.
- TE5: some verdict words are pinned on one side only (the count false only; mean, p001, p90 and loss true only;
  p99.9 false only); T5's p50 rank unpinned.
- TE6: T2's 1,000 ns tolerance is about 1,000 times the real error (the record matches to the ns).
- TE7 / NA6 (part): run.sh reads no UDP drop counter around its runs (R1 is covered by C-U8-6; the dry-run script is
  not).
- NA2: the reorder step (impair.rs:558-572, :430) carries a reader or pacer stall into -3's delay for the held
  datagram but never into the lateness, so point (h)'s "never enters -3's L2 delays" is not exact (negligible);
  "about 28 ms" is this container's buffer, RS's threshold will be its own.
- NA3: E is rounded down before the inclusive compare (socket_tests.rs:781, :805): lenient by under 1 ns.
- NA6: the leg-edge drop reads also count warm-up drops, and the socket's counter also counts checksum and filter
  drops; both err on the strict side.

**Confirmed holding:** the code computes U3D-BAR's quantity exactly (E with L0 = 0, over the histogram's sample set,
per direction, inclusive; NA3 aside); one monotonic clock throughout, each stamp conservative or off by about 15 ns
of E; the spawned poller's stalls enter the lateness and never touch t_in (the reader is its own OS thread); no other
case (drops, duplicates, clock behaviour, the first and last datagrams) makes E more lenient than -3; the advisor's
5,760 / 2,900 ns arithmetic re-derived (5,762 / 2,895 ns); C-U8-6 picks the right counter for reader stalls; a P1
kernel drop never reaches intake, so the accounting assert (socket_tests.rs:1657) fires, as bite S2 and round 3 show;
all seven tests hold for their planted defects (T1 :1002, T2 :1059, T3 :1113, T4 :1124, T5 :1146, T6 :1175, T7
:1272); all 22 build bites and the fix review's bites match the 1ccff06 code and fail for their own reasons; run.sh
ignores no failing step and its list check means `--exact` never runs 0 tests; SERIES.md's two new rows match the
plan and condition 2.

## Steps done
1. Branch set up (worktree reused from item 4); diff d012437..1ccff06 confirmed: 3 files, +1,227 −5; 78b0c07 adds
   results/u3d-timing/selftest-20261109T035556Z.txt (+113) · a47de32 · ok
2. test-engineer (Opus 5.5, xhigh): the seven fast tests and their bites, `test-engineer-u3d.md` (DONE) · 131f655 ·
   0 Critical, 0 High, 2 Medium (TE1, TE2), 5 Low
3. netcode-architect (Opus 5.5, xhigh): the measurement's validity, `netcode-architect-u3d.md` (DONE), scratch
   `na1_sampling.py` · this commit · 0 Critical, 0 High, 3 Medium (NA1, NA4, NA5), 3 Low
4. Refutation pass and this summary (the night main session) · this commit · nothing Critical or High to refute; the
   convergent NA1 = TE1 arithmetic re-done and holding; TE2's line read

## Current step
none: the item is done.

## Next steps (for the day session)
1. Fold `.checkpoints/npt-u3d-skeptic/` into PR #40's notes; add the Mediums and Lows to U3d's follow-ups.
2. Run the "needs a run (day)" lines: NA1's script, TE2, TE3 and TE4's planted changes (each with its cargo line
   above).
3. Take NA1 = TE1 (R-31's wording and a U3D-MISS case for "E PASS, -3 FAIL") and NA5 (C-U8-6's missing-row rule) to
   the advisor with U8's plan, since both change ruled text.

## Open findings
- All 13 finding ids above (12 distinct): follow-ups for U3d's report (review budget), none Critical or High.

## Key numbers
- the delivered count per direction in the dry run: 990,091 (selftest-20261109T035556Z.txt:33); -3's p99.9 fails at
  ≥ 991 delays above 30.25 ms; spread of that count about 18-31 datagrams, about 180-320 ns of E

## Decisions taken
- Items worked one at a time, top to bottom, in the main folder (`git switch -c <new> origin/main` per item) · night
  shift · 2026-11-11 02:18

## Running work
- none: the two agents finished (03:40, 03:43); the saver (bqpwo2txe) stopped with TaskStop at 03:43 after its
  autosave (2aa8bac)
