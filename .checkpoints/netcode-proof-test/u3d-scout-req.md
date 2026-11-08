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
