# U3C-HELD-1: test-engineer review (step 5, one round on the final diff)
Started 2026-11-17 by test-engineer (read only; ran nothing but reads; writes only this file).
Read: `git diff -- spikes/netcode-proof` (3 files, +74 -42), `git log -p -3 -- .checkpoints/netcode-proof-test/u3c-plan.md`
(104b234), the plan draft, advisor-plan-review-full.json (C1-C10, HELD-Q1-Q12) in full, build.md, u3c_held_loop.sh,
main-run.sh, and the logs l1.log, l1-1-94.log, light.log, e3a/e3b/e3c.log, p1-p5.log (+ p1-1-1.log, p3-1-1.log), e5.log.

## Plan
1. Read the final diff and u3c-plan.md's correction.  DONE
2. Read build.md, then each log it cites.  DONE
3. Loop script: binary check, failure output for every failing test (C3).  DONE
4. L1's catch against the plan's Confirmed line, whole (C2).  DONE
5. E3 counts: what they show, what not.  DONE
6. Each plant: predicted vs actual test, message, values; other failing tests; restores.  DONE
7. E5.  DONE
8. C5's pass bar; what the report must name as not covered.  DONE
9. Tests that cannot fail, timing dependence, missing negative cases.  DONE

## Checks (each against its log)
- Loop script (u3c_held_loop.sh): workspace bites build :28; BIN from the one `Executable unittests src/lib.rs
  (.../npt_net-<16 hex>)` line, exactly one or exit 2 (:32-33); exit 2 unless basename npt_net-ed8673eeeaf4db12 (:39);
  hashes of delay.rs, delay_tests.rs, BIN (:41); every failing test's whole stdout block in mode all (:73, range ends
  at the list's `failures:` so all blocks print: p1.log:54 and :103 both present). C3 met, with TE-UH1-2's defect.
- L1 (l1.log): header diff bdc5bba4 (D-1), BIN ed8673 (sha 04c76dc4); 188 runs exit 0 + loop 1 run 94 exit 101 = 189
  (grep counts); run 94: "168 passed; 1 failed", one name (G-NC2), panic delay_tests.rs:2302:5 (HEAD's :2300 + D-1's 2
  println lines; HEAD read by git show), "held <= 64 KB + the front record's acknowledged prefix", left [1, 0], right
  [0, 0]; println (l1.log:227): excess 14326 B, samples above [1, 0], first losses 28, D-1 over_waiting [1, 0],
  over_at_write [0, 0]. Confirmed line (plan :71-72): every item holds. No stop line met. CONFIRMED by a run.
- D-1's over_at_write is the same formula at the same spot as the fix's check_held_bound(dir) (plan :58-62 vs
  delay.rs:538-551, :634-635): run 94 is a direct positive control for the at-write half: where the old check fired,
  the new formula counted 0.
- E3a (e3a.log): header diff a7199f7a, delay.rs 3dbbccf7, delay_tests.rs 8f8c0f9a, BIN sha 2cf1e962; 600 "exit 0", 0
  non-zero, 600 "1 passed; 0 failed" (not vacuous: --exact matched); each println: writes above [0, 0], samples above
  [0, 0], excess 14326, losses 28, stalls 28 in every run (seeded model; only thread timing varies); 15.67-15.72 s each.
- E3b (e3b.log): same header/hashes/BIN sha; 600 exit 0, 600 "1 passed"; 1.51 s; held now [64336, 62651].
- E3c (e3c.log): same; 20 exit 0, 20 "169 passed; 0 failed" (12 ignored: count unchanged vs L1 -> 552/632 holds).
- Plants (each its own header diff and delay.rs hash, delay_tests.rs 8f8c0f9a, distinct BIN sha: 6cb02f1e, 85d5b3d7,
  534010f7, 769991ee, 7ad759f9):
  P1 p1.log: 2 failed; stalled :782 "...at every write" [1, 0] (= table); G-NC2 :2308 same msg [1, 0] (table: PASS;
     C3 pre-rules a FAIL here as evidence). Names: p1-1-1.log:287-289 (the call log's list is cut: TE-UH1-2).
  P2 p2.log: 1 failed; G-NC2 :2308 "...at every write" [121, 0] (= table [k>=1, 0]).
  P3 p3.log: 2 failed; stalled :787 "held only falls..." [134, 0] (table k>=8); G-NC2 :2313 same [1144, 0] (= "too").
     Names: p3-1-1.log:287-289.
  P4 p4.log: 2 failed; stalled :787 [137, 129] (= [k>=1, k'>=1]); G-NC2 :2290 hang guard "186 records, largest excess
     0 B, 48 first losses" (= table); 39.31 s.
  P5 p5.log: 1 failed; stalled :782 "...at every write" [0, 172] (= [0, k>=1]); G-NC2 ok.
  First failing assert of each test = the table in every case (P1's G-NC2 as C3 allows). No stop.
- Restores: intermediate ones rest on the session's sha256sum (build.md:77-95, not logged), corroborated by the
  failure patterns (P2 fails G-NC2 alone -> P1 gone; P3 fails :787 not :782; P5 fails :782 alone, G-NC2 ok -> P3, P4
  gone); final state: e5.log header diff a7199f7a = E3's and light.log's; the diff read now has no plant (down_free
  :478-482 and up_room :485-489 keep saturating_sub; `held > budget + prefix` :546; extend :1597 before model_write
  :1615; model_write's call :634-635).
- E5 (e5.log): diff a7199f7a; npt-net recompiled in release (provenance: Compiling npt-net), npt-lab sha 98c3967e...5113
  = preflight's. Stronger than planned: the npt-net code linked into npt-lab compiled unchanged.
- light.log: diff a7199f7a; fmt, clippy (both builds, --all-targets), doc PASS; "check: all 3 steps pass", exit 0.
- Diff audit (non-doc changed lines in crates/net): Life fields (struct cfg(test) :377), held_now's call (:523-524
  cfg(test)), check_held_bound / check_held_falls (cfg(test) fns), model_write's call (:634-635 cfg(test)), delay_tests.rs.
  All test-only. Doc lines only otherwise.
- Growth sites (grep): bytes extend only at delay.rs:829 (then model_write :830) and :1597 (then :1615); drain only at
  :1526 and :1771; no held_now between extend and model_write -> held_grew has no false positive; the new checks keep
  no timing dependence (deterministic by construction).
- Stalled-client test's post-hold write: poll_write takes min(len, RECORD_MAX, free - 22) and is Pending iff free <= 22
  (delay.rs:1584-1595), so the Pending assert is exactly as strong with RECORD_MAX as with 100 B; owed (now 16,384)
  feeds only probes and the ws gate, not the pump (grep). Asserts on held come before the Pending assert, so P1 names
  the bound, not Pending. Good.
- C5 bar: 552 / 632 right (no test added; 169 + 12 in the bites npt_net binary before and after); "all 11 steps" =
  STEPS' 11 keys (check.mjs:2614-2649); redact counts under spikes/ (notes files outside it do not move 305); refs must
  be read just before (each reviewer notes file committed by the saver adds a tracked file). Not yet written in the
  notes (build.md ends at 11:35): C5 requires it before the call.

## Findings (a line after each)
- TE-UH1-1 Low. "held only falls between writes" (delay_tests.rs:787, :2313; plan :129 "a counted fact") claims more
  than check_held_falls checks (delay.rs:558-564: each sample <= the latest write's held, not monotone). A growth
  outside model_write after an equal drain passes uncounted. The bound claim still holds (sample <= latest write's
  held <= its checked bound). Fix: report/card say "no sample above what the latest write left"; follow-up (C1 fixed
  the text): make it monotone, `held_at_write[d] = held_at_write[d].min(len)` after the compare (P3/P4 still bite; no
  false positive since no sample sits between extend and model_write), with a rename.
- TE-UH1-2 Low. u3c_held_loop.sh:70 takes every 4-space line from the FIRST `failures:` (before the stdout blocks),
  so with backtraces on it collects "at ..." lines and head -40 drops the names: p1.log:13-52 and p3.log:13-52 hold 40
  backtrace lines and no failing-test name; the names are only in p1-1-1.log:287-289, p3-1-1.log:287-289. Evidence
  holds ("2 failed" + two whole blocks). Fix: cite the per-run lists in the report; for reuse, print the list after the
  LAST `failures:` line.
- TE-UH1-3 Low (report wording). E3 shows no false positive of the new checks, not that the old one is gone: the old
  one was seen only in whole-binary runs (1 in 189 here); E3a/E3b ran a mode never shown to reproduce it (L2 not run);
  E3c's 20 whole-binary runs catch a 1-in-100 failure with 18 % chance (1 - 0.99^20); rule of three: < 0.5 % (one
  mode), < 15 % (whole binary) at 95 %. The removal rests on the code plus L1 run 94's over_at_write [0, 0]. Fix: the
  report says so; never "600 of 600 shows the flake is gone".
- TE-UH1-4 Low. P1's G-NC2 FAIL (p1.log:106-109; println writes above [1, 0]) is a one-count timing bite (it needs a
  write while a released record waits untaken), and it refutes the design notes' F15 / my F11 premise "S drains at
  once". Fix: report counts only the stalled-client test (p1.log:56-59) as P1's deterministic bite, records G-NC2's as
  a timing one, and notes the premise corrected.
- TE-UH1-5 Low. Both GATE PASS lines (build.md:59, :101) have no log; every other item has one. Fix: step 8's gate on
  the commit keeps its output (or the record quotes its summary line with the time) and the report cites it.
- TE-UH1-6 Low (follow-up, not this change-set). held_over_bound / held_grew are counted in every DelayStream test but
  asserted in two; in the stalled-client test the asserts sit before the drain phase, so its last write_all
  (delay_tests.rs:838) is unchecked. Fix: STATUS/card follow-up: assert both counts before drop(ds) (:842), later in a
  shared helper.
- TE-UH1-7 Low (nit). G-NC2's rewritten doc keeps "A few seconds" (delay_tests.rs:2272); E3a: 15.67-15.72 s in all
  600 runs, about half HANG = 30 s (delay_tests.rs:38). Pre-existing text C1 fixed exactly; the fix does not move the
  exit (held only grows at writes, so the first >64 KB sample follows a >64 KB write). Follow-up wording only.

## Verdict
APPROVE WITH FINDINGS: 7 Lows, none needs a push on its own. If any edit is made in the fix pass (TE-UH1-1's
monotone form or a message), C8 re-runs P3 and P4 (they bite held_grew) plus E3a/E3b and the light check on it.
