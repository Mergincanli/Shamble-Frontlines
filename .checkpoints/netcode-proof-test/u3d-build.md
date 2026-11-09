# U3d build notes (the timing self-test)

Builder: workflow agent in worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_2c76ec82-8fc-1, base
dae4f0f396d655d194e4300e8d56f07b619be908 (fast-forwarded, `git log -1` checked). Plan: u3d-plan.md (APPROVED WITH
CONDITIONS, advisor-u3d.md). Build rules: every cargo call with
CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target; touch every .rs under spikes/netcode-proof/crates
before each build; `df -h /home/user` before each build (stop under 2 GB); check each run's test count. Nothing
committed; no iptables, no tc, no network.

## Plan (one line per item)
1. Helpers in socket_tests.rs: `Polled`, `start_poller` (the poller defined inside it: condition 1), `excess`,
   `nearest_rank`, `quantile`, `loss_band`, `verdict` (T3's would-line), `Dir` arrays, the cross-check (`cross_check`).
2. §3.1 `l2_timing_self_test` (#[ignore]): P1 seed 11, NPT_L2_SELFTEST_SECS 1-600 (default 100), pre-touched fate log,
   tokio poller on a 4-worker runtime via `start_poller`, window rule, every print before any assert, run-condition
   asserts only.
3. §3.3 / §4 T1-T7 (not ignored).
4. Build + clippy (both builds) + fmt; first-build check of the shared folder and that no in-copy target appears.
5. Bites: each planted once, run, failure recorded, restored, sha256 checked.
6. §3.4 run.sh (dry-run script); checked with a short length only.
7. §3.5 SERIES.md row texts (:427, :452) and the l2_summary sentence (condition 2's wording).
8. Short self-test run (NPT_L2_SELFTEST_SECS=5), output recorded.
9. Final: check.mjs 9/9 twice in a row (counts 267 / 281 expected), clippy -D warnings with and without
   --all-features, `cargo +1.99.1 fmt --all --check`, Cargo.lock unchanged, M13's lines unchanged.

## Progress
- Items 1-3: helpers, the self-test and T1-T7 written at the end of socket_tests.rs (M13's lines untouched; one
  import line added at the top). First build created /home/user/Shamble-Frontlines/.gate/shared-target (CACHEDIR.TAG,
  debug/); no `.gate` inside the worktree. One fix during build: up has no send-error counters (names.rs:83-85), so T2
  checks down's only. `--list`: T1-T7 named; `--ignored --list`: l2_timing_self_test and M13's run. socket tests 3
  runs: 18 passed, 2 ignored each.
- Item 4: fmt applied (only socket_tests.rs changed). clippy -D warnings, --workspace --all-targets, without and with
  --all-features: clean. Short self-test (release, NPT_L2_SELFTEST_SECS=5) ran and passed its run conditions: up E
  (L0 = 0) 19,369 ns, down 15,633 ns, both would MISS, -3's p99.9 FAIL both: agree; CPU 0.366 core; slack 1/1,
  SCHED_OTHER. Print labels tidied after it. T4 changed to values 10 × rank: with 1..n a floor-rounded interpolated
  quantile equals the nearest rank on all three inputs, so the interpolation bite could not bite.
- Item 5: bites, 22 of 22 BITE (u3d-bites.py; output u3d-bites-output.txt). Each run built and ran exactly 1 test, which
  failed by name for its own reason; each restore's sha256 equals the original (socket_tests.rs 31294953...0625,
  socket.rs 7ad7cb22...76f9, stats.rs 27dfd326...9e22). The restored copy: socket tests 18 passed, 2 ignored.
  T1a 0.12 ms, T1b 0.09 ms late (bound 50 ms); T2a E 24,817,457 vs 12,037 ns; T2b E set 2,000 vs 1,978; T3a/b; T4a
  999; T4b 10,009; T4c (0, n); T4d ±500; T4e ±497; T5a-d 10,100/10,000, 10,000,000, 9,989, 10,250; T6a p99.9
  29,980,000 PASS; T6b panic naming a Lost fate; T6c count 1,000; T6d p90 band 29.25-29.75 ms; T7a "down: ... 0 ns
  late"; T7b "up: ... 0 ns late"; T7c "up: over 118, under the 204 samples".
- Item 6: run.sh written. `bash -n` OK (no shellcheck here). With CARGO_TARGET_DIR set and no U3D_CHECK it refuses
  (exit 2). Check run (U3D_CHECK=1, shared folder, NPT_L2_SELFTEST_SECS=3, NPT_L2_SECS=2, 3 rounds): built once,
  hashed the binary, listed both ignored runs, header facts all read (kernel 6.18.44-fc-v310, tsc, cpuidle none/menu,
  CONFIG_HZ 250 from /proc/config.gz, THP madvise), 6 runs exit 0, 5 steal lines per run, summary. Self-test E with
  L0 = 0 per round: up 6,753 / 61,237 / 9,867 ns, down 5,086 / 60,029 / 2,081 ns (round 2 one long pause: E is
  pause-dominated, as §2 says). Record moved to u3d-build-runsh-check-20261109T024229Z.txt (not evidence).
- Item 7: SERIES.md :427 and :452 row texts per §3.5 with condition 2's words; the l2_summary paragraph gains its
  sentence (reflowed to 120 columns).
- Condition 6's nits: (a) the clause dropped as moot: -3's 1,000,000 count is report-only at every length (its line
  reads FAIL by length below 100 s, and the run prints a "short run" note); the count the run asserts at any length is
  "every datagram accounted for" (delivered + lost = sent), as M13's :623. (b) T7 kept at 2,000 per second: its
  accounting wait passed in every run (3 clean runs, 3 bite runs, check.mjs), so no kernel drop showed; the assert
  stays. (c) applied: T1 waits on `s.shared.lock().waker.is_some()`, nothing test-only in the poller.
- Condition 1: the poller is an async block built only inside `start_poller`; the self-test, T1, T2 (via `drive`) and
  T7 (via `drive`) all start it there. T1's two bites, planted there, both bite.
- Item 9: check.mjs twice in a row, each after a touch, with the shared folder: run 1 9/9 (test 119.8 s: measured 267,
  bites 281, 0 failed, 0 skips; self 43/43; redact 10/10), run 2 9/9 (test 77.1 s: 267 / 281). 260 / 274 + 7 each,
  as planned. Both test binaries' mtimes (02:46:08, 02:46:44) follow run 2's touch (02:46:03); the wasm32 rmeta files
  too (02:46:06); no other cargo process ran. Then fmt --check OK and clippy -D warnings with and without
  --all-features clean (after a touch). Shared folder 3.5 GB; disk 14 GB free.
- M13's lines: the base's lines 483-639 (pace + M13) and the new 484-640 hash the same (29b21eab...2d89). Cargo.lock:
  no diff.

DONE (2026-11-09 about 02:50 UTC). Changed: spikes/netcode-proof/crates/net/src/l2/socket_tests.rs,
spikes/netcode-proof/marks/SERIES.md; new: spikes/netcode-proof/results/u3d-timing/run.sh. Notes and scratch:
u3d-build.md, u3d-bites.py, u3d-bites-output.txt, u3d-build-runsh-check-20261109T024229Z.txt. Nothing committed.
