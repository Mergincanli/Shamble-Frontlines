# Advisor: U3d plan review (u3d-plan.md)

Returned by the advisor (read-only) from workflow wf_bbda74db-c79 on 2026-11-09 about 02:20 UTC; recorded verbatim by
the main session, as the verdict's condition 5 asks.

## Verdict: U3d plan rev 2 (2026-11-09)

**APPROVE WITH CONDITIONS.** The plan meets every requirement with test code only. Every test can fail. The evidence
is honest: reported here, judged on RS at R1. It adds no engine feature. All seven questions are mine to rule; none
goes to Mustafa.

I reviewed the plan as the autosave committed it: b2e0980, 547 lines, sha256
3650755dee9447b4b2f94497a6eedfb09e8df1718c9300a918295dad02b2e8a3. That is rev 2 with the gap pass, not the dacdb77
text. The code is unchanged since 768d00b (`git diff --stat 768d00b HEAD -- spikes/` is empty). I edited, built and
ran nothing.

**What I checked (file:line, spike paths under spikes/netcode-proof/):**
- **The statistic:**
  - stats.rs:20-21 and :108-159: E is summed over all n; `over` counts every bucket above bucket_of(p50 + 0.25 ms);
    buckets are 1 µs wide below 8.192 ms.
  - stats.rs:426-505: Fate is 56 B; `push` checks the capacity; `drain` is `Vec::drain` and keeps the capacity.
- **The impairer:**
  - impair.rs:119-123: the delay is d − a + U[0, 2a], and no median is subtracted. U3D-L0's premise holds.
  - impair.rs:477-488 and :573-581: a lost datagram is logged at intake with due 0 and out 0, as T6 assumes.
  - impair.rs:653-697: `late = out − due` for every finished fate except Oversize.
- **The socket:**
  - socket.rs:113 and :256: `now_ns` takes no lock, so T7 can read t0 and t1 while it holds the lock.
  - socket.rs:494-538: `poll_recv` locks, then reads `now`, then finishes the datagram.
  - socket.rs:646-737: the release thread reads `now` and pops under the lock, wakes the poller before its batch, and
    stamps after `try_send`. So T7's claim "nothing due in (t0, t1] leaves before t1" holds in both directions.
  - socket.rs:739-741: the test module is a child module, so the pre-touch and T7's lock are test-only.
- **The existing tests:** socket_tests.rs:17-53, :184-212, :485-505 and M13 at :507-639.
- **quinn-0.11.12:** endpoint.rs:152-160 spawns the driver as a task; :754-759 sizes the buffers; :772-800 is the
  `poll_socket` loop.
- **The profiles:** npt-l1 lib.rs:102-162. P0 is tighter, but marks.json has no L2 P0 percentile check (I parsed every
  mark), so P1 is the tightest profile any check judges.
- **-3's checks:**
  - I parsed all 24 PM-0001-3 checks for "L2 P1". The bands equal T6's rule-derived numbers.
  - Line ranges :752, :763, :881, :999-1003 and :1021-1025 hold.
  - No check carries a `window` key, so -3 reads `measured` records (marks/README.md:100).
- **check.mjs:**
  - :1069-1074: cargo inherits the environment, and no `--target-dir` is passed.
  - :1121-1128: tests run on both builds, so each count rises by 7.
  - :886 and :895: the environment scan does not refuse `CARGO_TARGET_DIR`.
  - The spike's `.cargo/config.toml` target-dir is as §7.1 says.
- **The record:** SERIES.md:114-117, :401-406, :427 and :452. ADR 0001:1590 fixes "10,000 packets per second per
  direction" on the probe.
- **The numbers cited:**
  - The c10 results file :61-63 and :119-121.
  - u3-verify-timing.md:42-48 and :126-159.
  - The checkpoint: :404-413 (the shared-target rule as applied), :488-501 (Mustafa's words carry no number), :523-524
    (260 / 274) and :661-662.
  - plan.md:1222, :1225 and :1227. The U8 row already carries the steal clause for both self-tests, and states
    DelayStream's form with L0.
- **Numbers I re-derived by hand:**
  - T4: 35,640th; 1,000th of 1,001 (floor gives the 999th); 1,000th; isqrt(9,900) = 99 gives [9,505, 10,495];
    isqrt(99) = 9 gives [55, 145].
  - T5: 10,000 / 10,100. The bites give 10,000,000, 9,989 and 10,250.
  - T6:
    - p001 20,000,000; p90 28,990,000; p99.9 30,280,000 FAIL; mean 24,995,600; loss 10/1,010; count 1,010; E 100.
    - The `due − t_in` bite gives 29,980,000, which passes, so the assert fails as intended.
  - T7:
    - m = c + 1 µs falls in bucket first + 1, so `over` ≥ A holds exactly.
    - About 84 entries per direction are later than 8.192 ms, so bite (c) bites.
- **The bar:** p99.9 ≤ d + 1.05a holds exactly when E[(L − 0.25 ms)+] ≤ 0.001 × 2a = 10,000 ns at P1.

**What I did not check:**
- tokio's internals (multi_thread/mod.rs:80-91, notify.rs). I rely on the rust-reviewer's reading.
- That cargo lets `CARGO_TARGET_DIR` override `build.target-dir`. I did not re-read cargo's docs (no network). I know
  the order from cargo's docs, and the plan already checks it after the first build.
- No gate run: this is a plan review, and no code exists yet.

**Conditions:**
1. **Make bypassing the helper impossible.** Gap 1 is closed today only by the diff review. Define `poller` inside
   `start_poller` (an inner fn or an async block built only there), so no other code can call it. T1's bites, planted
   in the helper, then reach the run's poller by construction. The diff review still checks it.
2. **Name the judged form wherever the bar is written.** The new SERIES.md row text (§3.5) and the U8 row text (§3.6)
   say: "E with L0 = 0, ≤ 10,000 ns per direction (U3D-BAR, U3D-L0), over the probe's P1 measured records". Without
   this, the per-second statistic (L0 = that second's median) reads as the judged one.
3. **Scope of U3D-L0.** The ruling covers ImpairSocket's self-test only. DelayStream's judged form in the U8 row
   (plan.md:1227, "E[(L − L0 − 0.25 ms)+] < about 10 µs") is not changed by U3d. Add carry **C-U8-5**: U8's plan
   states DelayStream's judged form against U3D-L0 and brings it to me, so one bar is never read in two forms without
   a ruling.
4. **C-U6-6 must never make the run INVALID by design.** ADR 0001:1590 fixes 10,000 per direction on the probe. If
   -3's sends plus echoes (SERIES.md:405-406) cannot give that through the probe's socket, U6's plan brings it to me
   before R1's run plan is built.
5. **Pin what I approved.** The approved plan is b2e0980 (sha256 above). Conditions 1-4 and the §15 readings are the
   only edits allowed without coming back to me. The main session records this verdict in advisor-u3d.md.
6. **Nits, folded at the builder's discretion:**
   - (a) §3.1 says "below 100 s … does not assert the count", but §3.1 lists no count assert (-3's 1,000,000 is
     report-only). Say which count is meant, or drop the clause.
   - (b) During T7's 50 ms hold, about 100 up datagrams wait in the kernel's receive queue (rmem_default 212,992 B
     here). If the "every datagram accounted for" wait ever fails because the kernel dropped some, lower T7's rate.
     Never drop the assert.
   - (c) T1 can see the first Pending through `s.shared.lock().waker.is_some()` (child module). That keeps a test-only
     signal out of the run's poller.

**What stands as planned:**
- M13's run is untouched.
- The fate log is on and pre-touched.
- Inside the run only `try_send` and `poll_recv` are called, and every read comes after the stop.
- Every print comes before any assert, and INVALID runs stay in the record.
- Loss bands and ranks are integers.
- The dry run is reported, never judged here. A miss is reported, not retried and not a stop
  (advisor-u3c-plan.md:163-165).
- §7.1 matches Mustafa's 2026-11-09 rule as the checkpoint applies it (:404-413). The main folder's own cache stays the
  merge check that counts.
- Reviewers on the final diff: netcode-architect, rust-reviewer, test-engineer. The build agent runs at xhigh.
- Nothing traces to a "for later" feature. The self-test binary, the npt-log names, the reset method, the second
  stats.rs field and the per-release split are all rightly not built.

## Rulings on the plan's questions
- **U3D-BAR:** (a). The bar is E ≤ 10,000 ns per direction, inclusive. This is the exact value of the derivation
  behind the recommendation Mustafa agreed to; "about" is that derivation's approximation, not a tolerance. Mine: it
  is the strict reading of his bar, and -3 is still judged by npt-judge. It goes into §15 as a reading.
- **U3D-L0:** (b). Judge E with L0 = 0 for ImpairSocket; report M1's form as built. Mine: the impairer subtracts
  nothing, so this form predicts -3 exactly; it is never more lenient; it matches his recorded words, which carry no
  L0. It amends my own M1 report form. Its scope is set by condition 3.
- **U3D-WINDOW:** (W1). The probe's E over the P1 leg's measured records, computed from the fate log (C-U6-5), with
  `l2_summary`'s E reported beside it.
- **U3D-POLL:** (b), without the (b+) control round. Nothing decided here rests on the hop's p50 or p99. E is decided
  by long pauses (u3-verify-timing.md:134-139), and C10-B already found the hop does not own the tail. (c) is not built.
- **U3D-JUDGE:** (J3). I read the W1 value against the exact bar. Nothing goes into marks.json (check.mjs:1027-1052).
- **U3D-MISS:** (a).
  - A miss comes to me before any L2 mark counts.
  - The first remedy is the recorded `chrt -f` switch, run and recorded as its own leg, never as a rerun of the missed
    one (plan.md:683).
  - A max over 10 ms is flagged with the result.
  - A remedy that needs a spin lead goes to Mustafa (U3-Q2 (b)).
- **U3D-XCHECK:** (a). Keep -3's checks on the same samples, with the fate log on and pre-touched. T6 now catches a
  wrong delay formula.

## Decisions needed
None. U3D-BAR and U3D-L0 are readings of Mustafa's U3-Q1 (c) that never loosen it: one is the derivation's exact
value, the other is his own words. Every other question is technical, inside the approved spike.

## For Mustafa
Nothing.

## Main session: conditions applied (2026-11-09 about 02:25 UTC)
- Conditions 1-4 are applied in u3d-plan.md (§3.1 the poller inside `start_poller`; §3.5 the SERIES.md row text and
  §3.6 the U8 row text name the judged form; §3.6 C-U8-5; §3.6 C-U6-6's never-INVALID-by-design rule), each marked
  "(the advisor's condition N)". No other edit to the approved text. The §15 readings and the plan.md row texts of
  §3.6 are made in plan.md before any code merges. Condition 6's nits go to the builder's brief.

## Close ruling: U3d (2026-11-09)
(Returned by the advisor, read-only, about 04:20 UTC; recorded by the main session. Formatting compacted; the words
are the advisor's.)

**APPROVE WITH CONDITIONS. U3d closes, and U3 (U3a-U3d) closes with it.** The closing head is ef2bcef (spike code
1ccff06, the dry run's record 78b0c07). The unit rule holds: tests green, the gate run, notes saved, checkpoint pushed;
I reproduced the first two myself. The build meets conditions 1-4 and the plan. The fix pass's slack rule is the
plan's own reading, not a loosening. The dry run is honest and was reported, not judged. All five conditions are text
edits: no code changes, no re-run. One finding goes beyond wording: the INVALID round contains the first case where
the predictor (E) and -3's own check disagree, and the report leaves it out. It corrects one word in my own U3D-L0
reading ("exactly"). It does not change the bar. I edited nothing. I ran check.mjs and the gate in the main folder
(they write build output under `.gate/` only; `git status` clean before and after) and read /proc.

**What I checked (paths under spikes/netcode-proof/ unless noted):**
- Reproduced by me: check.mjs at ef2bcef, main folder, its own cache, CARGO_TARGET_DIR unset (04:10:41-04:11:57
  UTC): 9/9; tests measured 267, bites 281, 0 failed, 0 skips; self 43/43; redact 151 items (150 plus the new
  record), 0 findings. The gate `node tools/gate.mjs --no-smoke` on ef2bcef: GATE PASS (base 626df67; checks 1-4 and
  5b-5d pass, 5a skipped). CI: `gate` and `smoke` success on 1ccff06 and on 78b0c07. /proc at 04:05:56 UTC:
  /proc/net/snmp Udp InErrors 87, RcvbufErrors 87 (unchanged since the main session's 04:02 read); rmem_default
  212,992, rmem_max 4,194,304; /proc/net/udp has a per-socket `drops` column; no /proc/net/snmp6 here.
- The record (results/u3d-timing/selftest-20261109T035556Z.txt): every number in the report's table matches
  (SELFTEST 1 :32-65, M13 1 :94-96, SELFTEST 2 :129-162, M13 2 :191-193, SELFTEST 3 :222-259, M13 3 :306-308; steal
  :76, :107, :173, :204, :288, :319). The INVALID: up fates 999,913 (:237); the fate log holds 1,999,913 entries
  (:224); the accounting assert fired at socket_tests.rs:1657 (:261-264). Down's sink got all 990,091 delivered
  datagrams (:222), so no drop at the sink: the 87 were dropped on ImpairSocket's inbound socket. Every earlier run
  since the container started (the build's, the reviewers', the chrt runs) left no drop, or the counter would be
  above 87.
- The code that ran is the code reviewed: the header commit f50bd54 = 1ccff06 plus the checkpoint file only (git
  diff); the three files at 1ccff06 hash as the fix review's final state (ca17a9f6…, a902d10d…, f4a73a5b…;
  u3d-fix-review-test.md:22-25); the binary sha256 c236b449… equals the fix review's clean build (:35-36).
- The in-progress commits (git reflog): 03:57:04, 03:58:16, 03:59:36, 04:00:49, each pushed about 2 s later (the
  remote ref moved at 03:57:06, 03:58:18, 03:59:37, 04:00:51). None inside an M13 run; no autosave either (03:53:03,
  then 04:03:48).
- Conditions: 1 held (the poller an async block built only inside `start_poller`, socket_tests.rs:676-712; T1 at
  :1007, the self-test at :1409; T2 and T7 through `drive`); 2 held (SERIES.md:396-397, :427, :452; the U8 row,
  plan.md:1227); 3 held (C-U8-5, plan.md:1227); 4 held (C-U6-6, plan.md:1225); 5 held (u3d-plan.md has only
  conditions 1-4's edits since b2e0980); 6: (a) at :1603-1608; (b) T7 stays 2,000/s, its wait never failed
  (u3d-build.md:51-55); (c) at :1008-1013.
- The plan: M13's lines hash 29b21eab… on both sides (768d00b :483-639; 1ccff06 :484-640); no diff in socket.rs,
  impair.rs, stats.rs or Cargo.lock; the U3 row has U3d's sentence (plan.md:1222); C-U6-1..7 at :1225, C-U8-1..5 at
  :1227, R-31 at :1403.
- The reviews: netcode-architect, rust-reviewer, test-engineer APPROVE WITH FINDINGS; the fix review complete, no
  gap, no regression (u3d-fix-review-test.md:79-88).
- Read for the rulings: socket.rs:598-613 (the reader stamps t_in at its drain, `now_ns()` after the receive, :607,
  :613); thread.rs:8-12; quinn-0.11.12/src never calls quinn-udp's `set_recv_buffer_size` (quinn-udp-0.5.16
  unix.rs:263); u3-verify-timing.md:46 (the derivation's min(1, …)); c10-diagnostic-20261108T2152Z.txt:62;
  u3d-build-review-netcode.md:71-73 (its F6).

**What I did not check:** round 3's fate log was not kept, so the cap explanation below is arithmetic on the printed
max, not a measurement; whether a commit-and-push caused round 3's 34 ms stall (the record has no time for the max);
which RT policies ed4fb6d7ef68 covers (I rely on thread.rs:8-12 and the reviewers' chrt runs); I re-ran no bite (I
rely on 22 of 22, 16 of 16 and the test-engineer's own); anything on RS.

**The fix pass's deviation (the slack rule under SCHED_FIFO/RR): accepted.** Not a loosening: plan §3.1 already said
"(0 accepted under a real-time policy)" for the start read. The build before the fix was stricter than the plan and
made the recorded `chrt -f -p` switch INVALID by construction (u3d-build-review-test.md:110-112), against condition
4's rule. A 0 can only be read where the kernel forces a real-time task's slack to 0; under SCHED_OTHER the default
reads 50,000, never 0 (thread.rs:8-12). So under FIFO/RR, 0 and 1 both mean 1 ns or less, and a 0 read already
implies a kernel with ed4fb6d7ef68, as §5.2 asks. SCHED_DEADLINE rightly stays strict (1 and 1).

**The finding the report misses: the predictor's first disagreement with -3.** SELFTEST 3's down direction was
complete (count 1,000,000, :248), yet its lines read E (L0 = 0) 11,383 ns MISS against -3's p99.9 30,055,981 ns PASS:
"disagree" in all three forms (:252-255). The report's "agree in all 12 lines" is true of the valid rounds only. The
cause comes from the derivation's own form, P = E[min(1, (L − 0.05a)+ / 2a)] (u3-verify-timing.md:46): -3's tail
counts a datagram's excess only up to 2a = 10 ms; E counts all of it, so E overstates whenever a lateness passes 2a +
0.25 ms, 10.25 ms at P1. The arithmetic fits: round 3's max was 34.2 ms; one ramp that size at about 9,900 delivered
per second adds about 5,760 ns to E but only about 2,900 ns capped, putting down near 8,500 ns, under the bar, as
-3's p99.9 shows; in rounds 1 and 2 (maxes 20.5 and 26.7 ms) the capped value stays near 11,000 ns, over the bar,
and -3 failed. What it corrects: my plan verdict's line "p99.9 ≤ d + 1.05a holds exactly when E ≤ 10,000 ns" (above)
and R-31's "predicts -3 exactly" are wrong past 10.25 ms; this ruling supersedes that line. What it does not change:
the bar (Mustafa's words about E; E is still never more lenient than -3). At R1, a MISS where -3's p99.9 passes and
the max is over 10.25 ms is this case; it goes the U3D-MISS route as written.

**Conditions (text only; the main session applies them in the close commit; back to me only if the wording goes
beyond this):**
1. Fix the report (u3d-report.md): (a) add round 3's lines (down: E 11,383 ns MISS against p99.9 30,055,981 ns PASS,
   "disagree" ×3, :248-255, with the 10.25 ms cap as the reason; up: MISS against 30,750,757 ns FAIL, "agree",
   :242-245); scope "agree in all 12 lines" to the valid rounds; (b) the CPU dilution note is the netcode-architect's
   F6, not the rust-reviewer's: its 530 ms idle tail is 0.5 % of the self-test's denominator against about 5 % of
   M13's, so the self-test reads about 4-5 % (about 0.02 core) higher than M13 for the same load; the rest of the gap
   to 0.27 core is this container (M13 itself reads 0.385-0.390 here, :96, :193, :308); (c) "best yet" holds for up
   only (c10's run 1 had down at 2.672 µs, c10-diagnostic :62); (d) "loss 0.999 %" is up only; down was 0.991 % (:44,
   :54); (e) the in-progress commits were "commit and push", each; (f) "no host drops": those asserts are
   ImpairSocket's own queue, pool, oversize and shaper drops (:1654-1656); a kernel drop is caught only by the
   accounting assert, as round 3 showed; (g) the 04:02 snmp read is not in the committed record: add my 04:05:56 read
   (87 and 87); the counter is per network namespace, not per socket; (h) known limits gains one line: a stall of
   ImpairSocket's own reader delays t_in (socket.rs:607, :613), so it never enters L, E or -3's L2 delays; under about
   28 ms it leaves no trace; over that it shows only as kernel drops; (i) "a stall that hit both directions at once"
   is consistent with the record (maxes 34.17 and 34.21 ms), not shown: the fate log was not kept.
2. Amend R-31 (plan.md:1403): replace "this form predicts -3 exactly and is never more lenient" with: "this form
   predicts -3's P1 p99.9 exactly while no lateness exceeds 2a + 0.25 ms (10.25 ms at P1); past that it counts more
   than -3 sees, so it can MISS where -3 passes, never the reverse (corrected at U3d's close, advisor-u3d.md)". The
   bar, the form and U3D-MISS stay as they are.
3. Replace the slack rule in the U8 row (plan.md:1227; "slack 1 (or 0 under the recorded real-time switch)") with:
   "slack: under SCHED_OTHER 1 at both reads (start and header); under SCHED_FIFO or SCHED_RR 0 or 1 at both reads (0
   only on a kernel with ed4fb6d7ef68); under any other policy, SCHED_DEADLINE included, 1 at both reads (U3d's built
   rule, socket_tests.rs:1480-1486)".
4. Add C-U8-6 to the U8 row, as the U3D-RCVBUF ruling gives it; add its run condition to C-U8-4's list.
5. Update the checkpoint: its "Last update" header still names U3c (netcode-proof-test.md:6); refresh it, add the
   close entry with the closing head, and push.

**What stands:** the dry run is not re-run (nothing at the µs level rests on this container; the two valid rounds
miss the bar, consistent with c10 and with the build's short runs); at R1, RS judges; for any future dry run here, run
it in the background without ending the turn, so the stop hook never commits during a run.

## Ruling on U3D-RCVBUF
APPROVE recording the counters, with changes. No receive-buffer sizing.
- What to read, before and after every L2 probe leg (P1-P4 and B1), in the probe's network namespace, outside the
  measured path, as the steal reads are: (a) the probe socket's own `drops` column in /proc/net/udp or /proc/net/udp6,
  matched by its local port (it names the socket, which the namespace-wide counter cannot); (b) /proc/net/snmp's Udp
  lines, and /proc/net/snmp6's Udp6 lines when present (the probe may bind IPv6; the IPv4/IPv6 split not checked
  here).
- Run condition: a drop on the probe's P1 socket between the two reads makes ImpairSocket's self-test INVALID, with
  its cause named, as the accounting assert did here. That INVALID leg comes to me before any L2 mark counts, with its
  printed E, its max and the counters, as a miss would. It is never set aside and re-run as noise: a drop means the
  reader stalled for tens of ms (about 28 ms at this buffer), the same class of pause that decides E; re-running such
  legs would keep only the legs without long pauses, a pass by selection.
- No SO_RCVBUF sizing at R1 without my ruling: quinn leaves the OS default (it never calls `set_recv_buffer_size`), so
  the default is what production sees; ImpairSocket stamps t_in at the reader's drain, so a full buffer is the only
  trace of a long reader stall, and a bigger buffer would erase it. If R1's legs go INVALID for this cause, the remedy
  is the stall's (U3D-MISS), not the buffer's. Sizing comes to me only with another trace of reader stalls to replace
  this one.
- Carried in U8 as C-U8-6, read alongside C-U8-2's leg-edge reads by the same mechanism; its run condition joins
  C-U8-4. Not U6, unless U8's plan cannot place the reads at the leg edges; then U6 adds them to C-U6-7's leg-edge
  calls and brings any new npt-log name to me.

## One-line rulings
1. U3D-CLOSE: APPROVE WITH CONDITIONS. U3d closes on ef2bcef, and U3 (U3a-U3d) with it. Five text-only conditions;
   the slack deviation is accepted.
2. U3D-RCVBUF: APPROVE recording, changed: the socket's own drops plus both address families, around every L2 leg. A
   drop in the P1 leg makes the self-test INVALID and comes to me; it is never re-run away. No SO_RCVBUF sizing
   without my ruling. Carried in U8 as C-U8-6.
3. The report: one finding is missing (round 3's disagree lines and the 10.25 ms cap, which corrects R-31's
   "exactly"), plus eight smaller errors or omissions; condition 1 fixes them.

## Decisions needed
None. The bar is unchanged: expected lateness beyond 0.25 ms, measured from zero, at most 10,000 ns per direction.
Only my reading of how exactly it predicts -3 is corrected, and the correction is on the strict side. If R1 shows E
missing while -3 passes because of one long stall, I will bring that case to Mustafa then, with the numbers.

## For Mustafa
Nothing new from this ruling. The main session's open ask, LEFTOVER-DELETE, still stands as it wrote it.
