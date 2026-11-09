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
