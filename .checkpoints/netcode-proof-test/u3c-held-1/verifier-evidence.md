# Verifier: U3C-HELD-1 evidence check (PR #40, head aab3712, fix 0ad9661)
Started 2026-11-17. Read-only: no cargo, no check.mjs, no main-run.sh, no writing git command.

## Plan
1. L1: l1.log counts (189 runs, 188 exit 0, 1 exit 101 at 1-94); l1-1-94.log: G-NC2 alone, stdout quoted in build.md.
2. E3a/E3b/E3c: counts by grep -c; headers (binary, hashes); end lines.
3. E4: p1..p5 logs and per-run logs: exit 101, failing tests, messages, left values, lines; restores.
4. E5: npt-lab sha256 in e5.log and the earlier log (grep -rl 98c3967e).
5. E6: counted.log read whole against build.md's 12:18 bar.
6. Fix commit: stat, full diff (every non-doc change under cfg(test)), sha256 of the 3 files at 0ad9661, D-1 names in no commit.
7. gate --no-smoke once; last line.
8. Verdict and findings.

## Results (one line per check)
- L1 HOLDS. l1.log 284 lines: 189 run lines (loop 1: 94, loop 2: 95), 188 exit 0, 1 exit 101 at l1.log:202 (09:47:43 loop 1
  run 94, "FAILED. 168 passed; 1 failed; 12 ignored"); header HEAD e85fdba, diff bdc5bba4..., binary npt_net-ed8673eeeaf4db12
  (sha256 04c76dc4...), D-1 hashes b70ab4e5.../2ccaccc3...; end 09:48:12 UTC "first failure: 1-94"; main-run exit 101; no
  final df (last line "--- end 09:48:12 UTC, exit 101"). l1-1-94.log: 168 ok, 1 FAILED (G-NC2 only, :116), 12 ignored;
  stdout line :188 word for word as build.md; panicked at delay_tests.rs:2302:5, left [1, 0], right [0, 0]. Parent
  0ad9661^ delay_tests.rs:2300 is that `assert_eq!(` (life.held_over_bound, [0, 0]): ":2300, two lines down" holds.
  Parent hashes aaa6a588.../74d49de7... equal build.md's restore facts. 16,406 is the plan's bound (plan :73, :139).
  Side note: the loop's "libtest's failures list" print (l1.log:203-224) shows backtrace lines, not the list (already
  deferred as TE-UH1-2's reuse fix); the per-run log has the real list (l1-1-94.log:237-238).
- E3a HOLDS. e3a.log 616 lines: 600 run lines, 600 exit 0, 0 non-zero, 600 "ok. 1 passed; 0 failed;" (180 filtered out),
  loop 1 runs 1-300 and loop 2 runs 1-300 unique; 0 FAILED/panicked; 600 per-run logs, each "running 1 test", each G-NC2,
  each "ok. 1 passed; 0 failed". Header: binary npt_net-ed8673eeeaf4db12, delay.rs 3dbbccf7..., delay_tests.rs 8f8c0f9a...,
  start 09:53:52, "end 11:12:33 UTC; ... first failure: none", main-run exit 0.
- E3b HOLDS. e3b.log 616 lines: 600 / 600 exit 0, 0 non-zero, 600 "1 passed; 0 failed"; 600 per-run logs, each the
  stalled-client test a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded alone; same binary and
  hashes; start 11:13:04, end 11:20:49 "first failure: none", exit 0.
- E3c HOLDS. e3c.log 36 lines: 20 / 20 "exit 0 ... ok. 169 passed; 0 failed; 12 ignored"; same binary and hashes;
  "end 11:28:05 UTC; ... first failure: none", exit 0.
- Tree tested = tree committed: every E3 header's diff a7199f7a...; `git diff 0ad9661^ 0ad9661 -- .` (from
  spikes/netcode-proof) hashes a7199f7aa9bfcf80...: equal. The E3 HEADs (801d284, ff464c3, 2fe8c74) differ from 0ad9661^
  under spikes/netcode-proof only by results/u4r/64f3307d5c7f/B1/raw-logs.sha256 (978202d, a results list, not code).
- E4 HOLDS, all five. Each plant rebuilt in a pipe from 0ad9661's delay.rs + that one edit (python on stdin to stdout,
  no file written) hashes to its log header's delay.rs: P1 091957eb, P2 0e65d301, P3 dfbf8d2d, P4 7b41e90a, P5 09e3fb98
  (all equal), delay_tests.rs 8f8c0f9a in every header. So each plant is exactly the described edit on the fixed file,
  and the P1-P4 restores are proven by the next plant's header. Results (pN.log and pN-1-1.log):
  P1 exit 101, 167/2 failed: stalled-client :782 "...at every write" [1, 0]; G-NC2 :2308 same message [1, 0].
  P2 exit 101, 168/1: G-NC2 alone :2308 [121, 0]. P3 exit 101, 167/2: stalled-client :787 "held only falls between
  writes" [134, 0]; G-NC2 :2313 [1144, 0]. P4 exit 101, 167/2 (39.31 s): stalled-client :787 [137, 129]; G-NC2 :2290:9
  "hang guard: 186 records, largest excess 0 B, 48 first losses". P5 exit 101, 168/1: stalled-client alone :782
  [0, 172]; G-NC2 ok. Each per-run log: no other FAILED line (12 ignored, rest ok). p1-1-1.log:287-289 and
  p3-1-1.log:287-289 are libtest's failures list (the two names). Fixed delay_tests.rs :747/:782/:787/:2290/:2308/:2313
  are the test fn and those asserts.
