# test-engineer: adversarial review of u3d-plan.md rev 1 (U3d, the timing self-test)

Started 2026-11-09 on branch claude/netcode-proof-test-plan-hcky5g, HEAD f6041cb (95dac9c + checkpoint-only commits;
code as at 768d00b). Read-only: no code edits, builds, test runs or network.

## Plan
1. Read the plan (rev 1), scouts' notes (u3d-scout-req.md, u3d-scout-r1.md), plan-notes (u3d-plan-notes.md).
2. Check every cited source: stats.rs (E definition, L0, windows), impair.rs, socket.rs, socket_tests.rs (M13 run),
   delay_tests.rs, config.rs, npt-l1 lib.rs (P1), marks.json PM-0001-3, marks/README.md, SERIES.md, plan.md rows,
   advisor-u3q.md M1/M13/M14, advisor-u3c-plan.md rulings, u3-build-plan.md, u3-verify-timing.md, c10 result file.
3. Per test T1-T4: can it fail? does the bite fail it for its own reason? any timing dependence?
4. Statistics: E against L0 + 0.25 ms as defined; the L0=0 form; T2's 1,000 ns tolerance; the 5-sd loss band.
5. Dry-run honesty: what is claimed vs what the run can show; vacuous checks.
6. R1 handoff: everything R1 needs is carried (U6/U8 rows), nothing judged here that only R1 can judge.
7. Findings list + verdict.

## Findings log (one line per finding as found)
- V1 Sources checked and holding: stats.rs:3-8, :20-21, :51-62, :108-159 (E = sum over all n of (L - c)+ / n with
  c = p50 + 250,000; p50 a bucket mean; threshold bucket at its mean); impair.rs:653-697 finish (late = out - due for
  every finish but Oversize, :680-684; Delivered + SendError + WouldBlock); Lost/Burst/Blocked/ShaperDrop logged at
  intake (impair.rs ~:573-581, `self.log(Fate{index, ..early(..)})`), never in the histogram; reorder: overtaken due
  = successor's due + 1 (impair.rs:560-570). socket.rs:494-536 (now once per poll_recv under the lock; batch shares
  it), :427-429 drain under lock, :394-415 run_summary, :312-366 header, :268 release_thread_id. socket_tests.rs:
  507-639 M13 as described; :184-212 inbound-stamp test; :485-505 pace. P1 = 50 ms, +-10 ms, 1 % (l1 lib.rs:118-121),
  half jitter = jitter/2 (:156-158). marks.json PM-0001-3 P1 bands: loss share [0.0095, 0.0105] (5 % of 1 %), mean
  [23.75, 26.25] ms, p001 [19.75, 20.25], p999 [29.75, 30.25], p90 [28.75, 29.25] ms, count >= 1,000,000; default
  window `measured` (no "window" key in -3's checks; README.md:100). Derivation u3-verify-timing.md:45-48 re-derived:
  P(D + L > d + a + 0.25 ms) = E[(L - 0.25 ms)+]/2a for L <= 2a, so p99.9 <= d + 1.05a iff E <= 0.002a = 10 us at P1.
  HOLDS. c10 lines :62-63, :120-121 hold. quinn: spawn endpoint.rs:153-160; drive_recv :497 calls poll_socket
  :772-857 (loop until Pending, 32 buffers of ~64 KB each: :754-758); plan's "drive_recv (endpoint.rs:781-800)" names
  the caller, the loop is poll_socket's (nit). checkpoint :468-481 cites hold at 49bd1ac, now :488-501 at HEAD (nit).
  plan.md rows: U3 :1222 (plan says :1221 = U2: nit), U6 :1225, U8 :1227. THP here: madvise (fate log gets 4 KB
  pages; risk 2's estimate stands; by G^2/(2T) sub-0.25 ms faults add ~0 to E).
- F-A (medium) T1 is timing-dependent in the failing direction: the blocker starts only after the test thread sees
  first_seen and spawns it; if that takes > 20 ms (check.mjs runs debug tests in parallel), the release thread wakes
  the idle worker at due and the poller runs first: lateness ~0, T1 FAILS spuriously. The existing :184-212 is safe
  because a late test thread only makes its lateness larger. Fix: occupy the only worker BEFORE the datagram is sent
  (blocker signals "running" on a std channel, then blocks until first_seen + 110 ms), so the lower bound holds by
  construction.
- F-B (medium) T2 is near-vacuous for the exact-E code: at 2,000/s for 1 s on a quiet run both E's are often 0 (few
  or no samples over p50 + 0.25 ms), so "within 1,000 ns" passes for any E code except a gross unit error (the
  delay-for-lateness bite). Wrong L0 (0 vs p50), dividing by the over count instead of n, or a stale p50 all pass.
  Fix: a pure exact-E function tested on fixed vectors with known E for both L0 forms (brute-force style, stats.rs:
  537), PASS/MISS-discriminating values; keep T2 as the live identity, and assert lost > 0 there so its count bite
  is guaranteed to discriminate.
- F-C (medium) -3's cross-check delay has no bite: nothing tests that the delay is out - t_in (D + L). The likely
  bug, due - t_in (the drawn delay only), makes every cross-check line PASS and the "agree with E" line meaningless.
  T4 covers only the rank. Fix: a fixed-input test of the cross-check (entries with known D and L; p001/p90/p999/
  mean/loss share against hand values and against the bands read from marks.json), with bites due - t_in and lost
  entries kept in the delay series.
- F-D (medium) R1 handoff: the L0 = 0 E has no producer at R1 under U3D-L0 (a). §5.2 reports "the other E form" and
  U3D-L0 (a)'s straddle rule needs it, but l2_summary carries only M1's form (stats.rs:58-59) and §3.7 sends a second
  E field to U6 only "if (b) is ruled"; p50 and over (L > p50 + 0.25 ms) cannot bound it (P(L > 0.25 ms) unknown).
  Fix: carry C-U6-5 (the probe computes E with L0 = 0 from its fate log at the leg's end, SERIES row per M15), or
  drop it from §5.2 and let -3's own p99.9 alone break a tie.
- F-E (medium) R1 handoff, window: §2 says the judged window must be the very window -3 reads; -3's P1 checks have no
  "window" key, so they read `measured` records only (marks/README.md:100), while late_run covers everything since
  bind (impair.rs:246, :682). C-U6-2 aligns per leg only: warm-up seconds, base-RTT pings if sent through the same
  socket (marks.json notes :2348), and the after tail would sit in E but not in -3. Fix: C-U6-2 = reset at the
  measured phase's start, summary at its end (or no warm-up in the leg), with a U6 test.
- F-F (low-medium) R1 run conditions lack the rate: §3.1 asserts > 9,900/s here, §5.2 does not; the bar is defined at
  20,000 releases/s and -3's count (>= 1,000,000) does not bound the rate. The probe's up stream composition ("sends
  up, echoes": SERIES.md:405-406) can make RS's releases 20,000 or 30,000/s. Fix: condition from the per-second
  delivered counts; carry the composition (and the poller kind in the record) to U6.
- F-G (low) C-U6-1 and C-U6-2 carry designs without proofs: T1 proves the test helper only. Fix: "with T1's
  blocked-worker test re-run on the probe's poller"; "with a test that two legs' summaries never mix".
- F-H (low) Dry-run confounds: self-test vs M13 differ in profile, poller, length and fate log, so nothing here
  attributes E to the hop or clears the fate log; "about 10-19 us of E" for the hop rests on two single runs each with
  a 40-61 ms pause (u3-verify-timing.md:152-159). Fix: say so in §0/U3D-POLL and the report; optional one-variable
  controls at P1 (poller = thread; fate log off), reported.
- F-I (low) Record/order: steal from `head -1` only; take every `^cpu` line (per-vCPU steal; 4 lines here). Print
  everything (cross-check included) BEFORE the run-condition asserts (M13 prints first, :579-614 then :616-637), so an
  INVALID round still yields numbers; an INVALID round stays in the record with its cause (plan.md:683), never
  replaced.
- F-J (low) M13 refactor into a shared helper risks c10 comparability: require for M13 no runtime built, fate log 0,
  thread poller, same pacing; critics compare behaviour, not only asserts/prints (or copy instead of share).
- F-K (low) C-U6-3 "drained only at the leg's end" is unquantified and clashes with -3's per-second second.s.probe_*
  records; by the plan's own G^2/(2T) a sub-ms per-second drain adds ~0.03 us of E over 100 s. Fix: U6 chooses, with
  the drain pause measured.
- F-L (nit) citations: U3 row is plan.md:1222 (RQ10, §3.6 say :1221 = U2); quinn's loop is poll_socket
  (endpoint.rs:772-857), called by drive_recv :497; quinn's buffers ~64 KB each (:754-758), test's 1,500 B fine for
  40 B payloads (say so); checkpoint cites are 49bd1ac's lines (now :488-501).
- F-M (nit) T2's count identity holds only with no SendError/WouldBlock (they enter the histogram, impair.rs:680-684):
  assert them 0. F-N (nit) T3 bakes U3D-BAR (a)'s "<= 10,000": follows the ruling. F-O (nit, optional) a
  deterministic lower-bound stall test (hold the lock G = 50 ms once; over_count >= datagrams due in [t0, t0+G-c] from
  the fate log) would prove the release path's chain under a burst; the plan's F25 reasoning is otherwise sound.
- Checked, no finding: the bar derivation (exact, 10,000 ns at P1); M1-lenience bound p50 x P(L > 0.25 ms); 5-sd band
  (sd 99.5; exact band underflows: delay_tests.rs:167-170); nearest-rank fixtures (eval.rs:261-265, :1595-1597; f64
  ceil agrees for these q up to n 200,000); THP madvise (risk 2 holds); no npt-net serde_json (Cargo.lock rule), so
  bands are copies: F-C's drift check.
- VERDICT: REVISE (five medium fixes, all local to §4, §5.2 and the U6 carries; no structural change).
DONE 2026-11-09
