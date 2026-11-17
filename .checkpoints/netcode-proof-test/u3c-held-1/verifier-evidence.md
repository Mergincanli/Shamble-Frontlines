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
  "end 11:28:05 UTC; ... first failure: none", exit 0. E3a/b/c test binary sha256 the same in all three (2cf1e962...).
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
  are the test fn and those asserts. Predictions match the plan's table (plan :154-158).
- P5's restore: e5.log's header (11:33:54, after P5) has diff a7199f7a... = the committed fix: proven.
- E5 HOLDS. e5.log 36 lines: header free 4955 MiB, HEAD 1df9072, diff a7199f7a...; `cargo build --locked --release -p npt-lab`
  exit 0 (11:33:54-11:34:23); npt-lab 98c3967e04e4260985d24173b1fb2126ea1444636f4725e12cf76333d90b5113. Earlier log:
  .gate/u4r/c62h-bound/preflight.log:24 (2026-11-16 20:45:36 UTC, HEAD ea08f89, diff e3b0... clean), the same hash.
  Search: grep -rl --include='*.log' over .gate/u4r .gate/u3c-held-1 .gate/u4e .gate/u4r-d4 finds just these two logs
  (the whole-.gate grep was stopped: 6.2 GB shared-target + 13 GB netcode-proof-test); .checkpoints hits are notes.
- E6 HOLDS, every item. counted.log 29 lines (wc -l), read whole: header HEAD 0ad9661, diff and untracked e3b0c442...,
  free 5552 MiB (df 5,822,070,784 B at 12:18:37); 11 PASS lines (fmt, clippy, clippy-wasm32, doc, test 804.8 s "552 ...
  0 failed, 0 reported a skip; bites 632 ... 0 failed, 0 reported a skip", deps "137 crate files and 1 cargo config",
  wasm "27000 ticks compared, 0 mismatching" both ways, refs "1382 tracked files", marks "81 marks", self "78 of 78",
  redact "10 of 10 ... 306 item(s) checked, 0 finding(s)"); "check: all 11 steps pass"; "=== 12:34:46 UTC exit 0: node
  check.mjs"; "--- end 12:34:46 UTC, exit 0". Free after 5,781,766,144 B (5513.9 MiB; 40,304,640 B = 38.4 MiB net).
  1382 = `git ls-tree -r --name-only 0ad9661 | wc -l`. No new test: the diff adds/removes no #[test]/#[tokio::test];
  552/632 equal C62H's second call (c62h-bound-build.md:389, :449). build.md's pre-call "5,822,124,032 B" is a separate
  reading (53,248 B off the log's df; not contradicted). The task's own exit code: not visible to me (log says exit 0).
- Fix commit HOLDS: 3 files, 74 insertions, 42 deletions (delay.rs 73, delay_tests.rs 39, SERIES.md 4); parent 978202d;
  message = .gate/u3c-held-1/fix-commit-msg.txt (but git's trailing newline). sha256 at 0ad9661: delay.rs 3dbbccf7...,
  delay_tests.rs 8f8c0f9a..., SERIES.md 1dd67e2f...: equal to build.md (and to the worktree now). aab3712 vs 0ad9661:
  .checkpoints only.
- cfg(test) HOLDS. delay.rs: changed non-doc lines are (a) held_now's call, under its existing #[cfg(test)]; (b) Life's two
  new fields, Life is #[cfg(test)]; (c) check_held_bound(d) and the new check_held_falls, each #[cfg(test)] fn; (d) the
  new `#[cfg(test)] self.check_held_bound(dir);` in model_write. The rest is //! and /// doc. delay_tests.rs is
  `#[cfg(test)] #[path = "delay_tests.rs"] pub(crate) mod tests;` (delay.rs:1915-1917), so all its lines are test code.
  SERIES.md: two table rows (doc). Shipped code unchanged; E5's equal lab hash agrees.
- "(both directions)" in the message holds: bytes[UP].extend at :829 then model_write(UP) :830; bytes[DOWN].extend :1597
  then model_write(DOWN) :1615; model_write ends with check_held_bound(dir) :635.
- D-1 names: `git log --all -S over_waiting` and `-S over_at_write` each list 6 commits (3a28107, 43a6f1f, 801d284, ed15263,
  ec70096, 6d24666), every one touching .checkpoints/ only (0 hits under spikes/ in each commit's diff); `git grep` at
  aab3712 -- spikes: none; worktree spikes/: none. So D-1's code was never committed; the NAMES are in notes commits:
  the caller's literal "appear in no commit" does not hold, the build record's "D-1 ... NEVER committed" does.
- Commit message other claims: C62H-BOUND's first counted run's one failure (c62h-bound-build.md:428-435, merge.log:48-51)
  holds; reviews 1 Medium (NA-UH1-1) + 12 Low (NA 3, TE 7, RR 2), all APPROVE WITH FINDINGS, file hashes 015aa739,
  834bf936, ac2cf470 hold; plan review APPROVE WITH CONDITIONS, C1-C10, 627bfc6c holds; NA-UH1-1 met in plan :132 holds;
  light.log (09:51:51-09:53:06, diff a7199f7a): fmt, clippy, doc PASS, "all 3 steps pass", exit 0 holds.
  Wording: "caught the failure in run 94 of 189" (also 801d284's subject): it was loop 1's run 94 of 2 parallel loops;
  in finish order it was the 188th of 189 runs. Facts hold, phrasing reads as the 94th run overall (Low).
- Step 5 (pack): b1-redact.log "306 item(s) checked, 0 finding(s)" exit 0; 978202d one file (raw-logs.sha256, +12);
  archive .gate/netcode-proof-test/kept-logs/u4r-B1-64f3307d5c7f.tar.gz 227,889,786 B; holds.
- build.md gained 7 lines after aab3712 (12:37 push, 12:48 CI green; remote now 9f415dc, notes-only after aab3712).
  CI checked by `gh run view`: 44084854535 gate push aab3712 success (gate, smoke), 44084863595 gate pull_request
  aab3712 success (smoke, gate), 44084854552 / 44084863752 toolchain push / pull_request aab3712 success: holds.
- Gate (this run, `node tools/gate.mjs --no-smoke`, base 82c605e): [1]-[4], [5b]-[5d] PASS, [5a] SKIP; last line
  "GATE PASS (5a smoke skipped: --no-smoke)". build.md's 09:53 and 11:34 gate PASS lines have no saved log; legacy/ is
  untouched by the fix and both this run and CI pass, so nothing rests on them.

## Not verified
- D-1's "+17 / +4 -2" stat (the patch was never saved; only its hashes and the +2 line shift agree).
- The background tasks' own exit codes (each log's final "exit N" line was read instead).

## Findings
- VER-UH1-1 Low: "run 94 of 189" (0ad9661's message; 801d284's subject) reads as the 94th run overall; it was loop 1's
  run 94 of two parallel loops (finished 188th of 189). Fix: say "loop 1's run 94 (189 runs in all, two loops)" in the
  report; the commit stays (no history rewrite).
- VER-UH1-2 Low: "D-1's names appear in no commit" is not literally true: 6 notes-only commits name them. Fix: in the
  report write "no commit's code (nothing under spikes/) holds D-1"; the build record's own words already say that.

## Verdict
PASS (two Low wording notes; every number, hash, count, assert and line checked holds).
