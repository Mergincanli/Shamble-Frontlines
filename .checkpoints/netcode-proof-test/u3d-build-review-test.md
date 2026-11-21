# U3d build review (test-engineer): proving the timing self-test, 2026-11-09

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_2c76ec82-8fc-1 on dae4f0f (uncommitted build). I am the
only one touching it now. Build rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target on every
cargo call (check.mjs included); touch the crate sources (mtime only) before each build; df -h /home/user before each
build (stop under 2 GB); check each run's test count against the one before. No iptables, no tc, no network; git
without -C / -c.

## Plan
1. Record the builder's sha256 of every changed file (baseline), git status.
2. check.mjs, unchanged, first: nine step lines and counts.
3. Bites, in place, one at a time, my own script (u3d-test-bites.py in this folder): T1a-b, T2a-b, T3a-b, T4a-e,
   T5a-d, T6a-d, T7a-c as §4 names them; then bites the plan missed: the self-test's run conditions (rate, drops,
   accounting, loss band, send errors, poller count, fate_log_full, slack, length bounds), the pre-touch, prints
   before asserts under a failing condition. Each: plant, run its test, record fail + how, restore, sha256 == builder's.
4. Reviewers' settleable findings: F1 slack under RT (chrt if the container allows), env NotUnicode, touched,
   samples() panic path, run.sh touch scope.
5. Self-test with NPT_L2_SELFTEST_SECS=10 (release): prints vs §3.1 and §5.1.
6. run.sh short (U3D_CHECK=1): U3-Q9 facts, THP, every ^cpu before/after; move its record out of results/.
7. Fresh touch, check.mjs again; git status + sha256 unchanged.
8. Findings, verdict, DONE.

## Progress
- Baseline (03:00 UTC): git status = M socket_tests.rs, M SERIES.md, ?? results/u3d-timing/ (run.sh only), ?? 4 builder
  notes under the worktree's .checkpoints/. sha256: socket_tests.rs 31294953...0625 (== builder's), SERIES.md
  b06b67a0...fc45, run.sh 32babfb1...602e, socket.rs 7ad7cb22...76f9, stats.rs 27dfd326...9e22, impair.rs
  f79b5084...ffca, Cargo.lock d0158c8a...6397. No .gate inside the worktree. df 14 GB free. No other cargo process.
- check.mjs run 1 (touch 03:00:10, shared folder): 9/9 PASS in 84 s. fmt 0.3 s; clippy 2.6 s (both builds); clippy-wasm32
  0.4 s; test 78.2 s, measured 267 passed 0 failed 0 skips, bites 281 passed 0 failed 0 skips; deps 0.5 s; refs 481
  tracked files; marks 81 marks; self 43/43; redact 10/10, 150 items 0 findings. Counts = builder's (267/281).
- clippy's 2.6 s after a touch looked too fast: re-ran `cargo clippy --workspace --all-targets` after a touch: every
  member "Checking" in 1.4 s (debug incremental reuses the unchanged content). So the touch does re-lint; fine.
- chrt -f 1 works in this container (RT throttle 950000/1000000): F1 can be settled by a run.
- Bites script: u3d-test-bites.py (this folder), my own anchors; several bites in other forms than the builder's.
- Unit bites (debug, 1 test each, output u3d-test-bites-unit.txt), every restore sha256 == builder's:
  - T1a BITES (spawn_blocking + parked waker): lateness 92,716 ns vs ≥ 50 ms. T1b BITES (Handle::block_on on a
    blocking thread): 204,931 ns. T1x (extra: blocker lets go at the drain) BITES: 174,656 ns. So T1's lower bound comes
    from the held worker, and both poller bites planted in start_poller fail it.
  - T2a BITES (E 24,835,574 vs 25,584). T2b (mine: Lost in in_e_set) BITES (2,000 vs 1,978). T2x (extra: drive's
    accounting wait removed) PASSES: the 200 ms idle sink already outlasts the 60 ms max delay, so the wait is a guard
    a clean run never exercises (informative, not a defect).
  - T3a BITES ([MISS,MISS] vs [PASS,PASS]); T3b BITES ([PASS,PASS] vs [MISS,PASS]).
  - T4a (mine: saturating_div) BITES 999 vs 1,000; T4b (mine: Hazen q n - 0.5) BITES 356,405 vs 356,400; T4c BITES
    (0, 1,000,000); T4d BITES (9,500, 10,500); T4e (mine: f64 sqrt of the integer variance) BITES (9,503, 10,497); T4f
    (the plan's ±497 form) BITES.
  - T5a-d BITE: (10,100/10,000 swapped), (10,000,000), (p50 110,250 -> 9,989), (10,250/10,350).
  - T6a BITES (p99.9 29,980,000 PASS); T6b (mine: Lost kept via saturating_sub, no panic) BITES (n 1,010, p001 0);
    T6c BITES (count 1,000); T6d (mine: p001 at d - 0.9a) BITES; T6e (mine: mean within 10 %) BITES.
  - T7a BITES (down 0 ns late); T7b BITES (up 0 ns late); T7c BITES (over 61 < 146); T7x (extra: the 50 ms window
    without the lock) BITES (up 63,180 ns late < m 311,528): T7 fails when its premise is removed.
- Self-test run-condition bites (release, NPT_L2_SELFTEST_SECS=3, 1 test each; u3d-test-bites-self.txt), every restore
  sha256 == builder's. Each failing run printed the same 45 lines before its panic as the passing S6 printed in all
  (count includes cargo's own indented lines), so every print comes before the first assert:
  - S1 pacers at 9,800/s: INVALID "the rate fell to 9800 per second per direction". BITES.
  - S2 stop without the d + a + 500 ms wait: INVALID "up: every datagram accounted for" 29,756 vs 30,000. BITES.
  - S3 loss band at 2 x ppm: INVALID "up: 325 lost, outside P1's band 480-720". BITES.
  - S4 fate log capacity n: INVALID, but by the loss band ("up: 325 lost, outside P1's band 90-210"), not by "the fate
    log filled": the band is computed over the log's own fate count, so a short log trips it first. BITES (cause
    mislabelled).
  - S5 pre-touch drain removed (log full of zero entries from the start): INVALID by the loss band again (490-730), and
    the printed E reads 0 ns, "would PASS" both ways. BITES (cause mislabelled).
  - S6 pre-touch skipped: PASSES. The pre-touch is not a run condition (confirms netcode F3 / rust nit): only the
    printed "pre-touched with N entries" shows it.
  - S7 release thread keeps the default slack (socket.rs, set_own_slack(t, 0)): INVALID "the slack read back at start
    under SCHED_OTHER 0" left 50,000 right 1. BITES.
  - S8 poller counts one per batch: INVALID "the poller's count" 22,018 vs 29,675. BITES.
- Clean release build after the self bites (touch first; u3d-test-runs.sh, output u3d-test-runs-output.txt): binary
  npt_net-f72f21a42e60b4c9 built 03:10:50 (after the touch), sha256 d0531316...; --list 172 not ignored, T1-T7 each
  listed by name; --ignored lists l2_timing_self_test and M13's run.
- Step 3, self-test at 10 s (u3d-test-run-selftest10.txt): exit 0, 39 print lines (as the code gives at < 100 s), every
  §3.1 / §5.1 line present: poller kind; fate log 201,000 entries 11.3 MB, pre-touched 201,000, 200,000 after the stop;
  loadavg start/end; M13-format line per direction (n, p50, p99, p99.9, max, E, over); fate-log forms per direction
  (E at the histogram's p50 == the histogram's own E exactly: up 21,730/21,730, down 4,939/4,939; L0 = 0 22,123 / 5,129;
  exact p50 21,727 / 4,939); a would-line per form per direction vs 10000 inclusive (up MISS, down PASS); -3's six P1
  checks per direction + RTT "not computable"; three agree lines per direction (all agree: up p99.9 33.8 ms FAIL with
  E MISS; down 30.05 ms PASS with E PASS); short-run note; CPU 0.383 core vs 0.27; slack 1/1 set, SCHED_OTHER 0.
  E(L0 = 0) - E(L0 = p50) = 396 ns up, consistent with p50 x P(L > 0.25 ms) ≈ 37.9 µs x 0.0101 (plan §2).
- Bounds: 0, 601, abc, "", -5, 1.5, " 10" each INVALID before bind (exit 101, no self-test print). Non-UTF-8 (\xff):
  still running at 5 s, no INVALID: silently the 100 s default (confirms the rust nit).
- F1 settled by a run (u3d-test-run-chrt-after.txt): `chrt -f -p 1` on npt-l2-release ~1 s into a 6 s run (the
  recorded switch, M14): slack now 0, start 1, SCHED_FIFO 1 -> INVALID "the slack read back at start under SCHED_FIFO
  1" left 1 right 0, after all 39 prints. Whole process born under `chrt -f 1` (3 s): start 0, FIFO -> passes (this
  6.18 kernel forces RT slack to 0). So the recorded switch applied from outside is INVALID by construction: F1 holds.
- Extra coverage bites (debug; u3d-test-bites-extra.txt), restores == builder's: X1 agree() always "disagree": T6 PASSES
  (T6 tests only the disagree case). X2 the loss share over the Delivered count instead of every fate: T6 PASSES (10/1,010
  and 10/1,000 are both inside 0.95-1.05 %). Two report-only branches no test reaches: nits.
- Step 4, run.sh in check mode (U3D_CHECK=1, shared folder, touch first; NPT_L2_SELFTEST_SECS=5, NPT_L2_SECS=2, 3
  rounds): exit 0. Record moved out of results/ to u3d-test-runsh-check-20261109T031214Z.txt (not evidence; results/
  holds run.sh only again). Built once (14 s), binary npt_net-f72f21a42e60b4c9 sha256 d0531316...898d (the same hash as
  my clean build: the touch-rebuild is reproducible). Header: commit dae4f0f + "uncommitted changes", rustc 1.99.1,
  binary + sha256, date, nproc 4, cargo_target_dir, lengths. U3-Q9's four facts all read: kernel 6.18.44-fc-v310;
  clocksource tsc (available tsc kvm-clock); cpuidle driver none, governor menu; CONFIG_HZ 250 (from /proc/config.gz,
  not CLK_TCK); THP "always [madvise] never". Six runs, each between uptime and every ^cpu line (cpu + cpu0-3) before
  and after, a steal share per line (hand-checked SELFTEST 1's cpu line: Δ 58+0+169+1652+4+0+49+4 = 1,936, steal 4,
  0.0021 = the script's), exit status 0 each, summary and loadavg_end. Self-test E (L0 = 0) up per round: 961 / 36,934 /
  27,736 ns: pause-dominated, as plan §2 says.
- Flake look (u3d-test-repeat.txt; debug binary built 03:13:44 after a touch): T1 + T2 + T7 together x 30 runs: 0
  failed; the whole npt-net lib suite x 8 (160 passed, 12 ignored each, ~24.8 s): 0 failed.
- M13's lines: base 483-639 and new 484-640 both sha256 29b21eab4938ffa2...; diff hunks only @@-2 (doc line), @@-3,0 +4
  (import), @@-639,0 +641,992 (appended). No diff in Cargo.lock, socket.rs, stats.rs, thread.rs.
- check.mjs run 2 (fresh touch 03:17:54, shared folder): 9/9 PASS. fmt 0.4 s; clippy 2.5 s; clippy-wasm32 0.4 s; test
  77.1 s, measured 267 passed 0 failed 0 skips, bites 281 passed 0 failed 0 skips; deps 0.4 s; refs 481; marks 81; self
  43/43; redact 10/10, 150 items 0 findings. Both npt_net test binaries newer than the touch (03:18:00, 03:18:36).
  Counts equal run 1's.
- End state: git status identical to the baseline (M socket_tests.rs, M SERIES.md, ?? results/u3d-timing/ with run.sh
  only, ?? the builder's 4 notes); sha256 of socket_tests.rs, SERIES.md, run.sh, socket.rs, stats.rs, impair.rs,
  Cargo.lock all equal the baseline; no .gate inside the worktree. The worktree is unchanged by me.

## Findings (ranked)
1. LOW, proven by a run: socket_tests.rs:1445-1449, :1627-1631. The slack condition demands start 0 under FIFO/RR, so
   the recorded switch (chrt applied to the release thread after it started) is INVALID by construction (chrt-after
   run: start 1, FIFO, left 1 right 0). Fix as netcode F1 / rust LOW.
2. LOW: socket_tests.rs:1605-1610 before :1622-1626. The loss band is computed over the fate log's own count and asserted
   before fate_log_full, so a short or pre-filled log (S4, S5) is INVALID under the wrong cause, and S5's prints read
   E 0 ns "would PASS". Fix: assert the log complete (fate_log_full 0 and sm[d].fates == n) before the loss band.
3. LOW: socket_tests.rs:1355-1374, :1466-1468. The pre-touch is no run condition: S6 (skipped) passes. Fix: assert
   touched == cap after the prints (netcode F3, now shown to matter).
4. NIT: socket_tests.rs:1326-1327. Non-UTF-8 NPT_L2_SELFTEST_SECS runs the 100 s default silently (proven).
5. NIT: socket_tests.rs:900-906, :886, T6 :1231. agree()'s "agree" branch and the loss share's denominator are unbitten
   (X1, X2 pass). Fix: two agree asserts; a 106-of-10,106 loss case (1.049 % PASS vs 1.06 % over Delivered FAIL).
6. NIT: netcode F4 (samples() panic before prints), F5 (run.sh:52 touches *.rs only), F2 (SERIES.md:396 "Its"):
   confirmed by reading.
7. INFO: drive's accounting wait (:967-972) never exercised in a clean run (T2x); CPU dilution (F6): 0.383 core over
   10.533 s vs ~0.404 over the 10 s send window.
VERDICT: APPROVE WITH FINDINGS. Regressions: none.
DONE (2026-11-09 03:20 UTC).
