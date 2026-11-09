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

## Fix pass

Fixer: workflow agent, same worktree, 2026-11-09 from 03:24 UTC. Inputs: the three build reviews
(u3d-build-review-netcode.md, -rust.md, -test.md in the main folder's notes), the plan and the advisor's verdict. Same
build rules as above (shared folder, touch every .rs under crates before each build, df before each build, each run's
test count checked against the one before; no iptables, tc or network; nothing committed). Baseline sha256 at 03:24:
socket_tests.rs 31294953...0625, SERIES.md b06b67a0...fc45, run.sh 32babfb1...602e, socket.rs 7ad7cb22...76f9,
stats.rs 27dfd326...9e22, Cargo.lock d0158c8a...6397 (= the builder's and the test-engineer's).

### Plan (one line per item)
1. Slack condition: under SCHED_FIFO/RR accept 0 or 1 ns both at start and now (start 1 when chrt comes from outside
   after the thread's birth; 0 when the whole process is born under chrt on a kernel with ed4fb6d7ef68, as thread.rs:12
   names that form too); under any other policy 1 and 1. The accepted values printed on the slack line. Prove: the
   chrt-after run (now valid), the chrt-born run (still valid), bites S7 (start 50,000), a "now" bite, S7 under chrt.
2. Asserts reordered: rate, pre-touch, `fate_log_full` 0, per direction drops, accounted, the log's fates = n, then the
   loss band. Prove: S3, S4, S5 re-run, plus S5b (one entry left in the log: only the new fates assert catches it).
3. `touched == cap` after the prints (INVALID). Prove: S6.
4. NotUnicode → the INVALID panic; only NotPresent takes the default. Prove: \xff exits INVALID at once.
5. T6: agree's two "agree" branches; the 106-of-10,106 loss case. Prove: X1, X2 now fail T6.
6. Nits: samples() counts Delivered fates with out < t_in (`early`) instead of panicking, the self-test asserts 0 after
   the prints, T6 asserts the count (up 0; down one planted early fate counted, not a delay); run.sh:52 touches every
   file under crates (§7.1 item 2); SERIES.md:396 "The event's lateness fields"; FATE_BYTES imported; agree keeps &str
   (item 5's asserts as written, X1's anchor kept); the poller's buffer unchanged. Prove: early bites (self-test, T6),
   the builder's T6b form.
7. Short self-test (NPT_L2_SELFTEST_SECS=5); check.mjs 9/9 twice; clippy -D warnings with and without --all-features;
   fmt --check; M13's lines and Cargo.lock unchanged.

### Progress
- Edits made (03:30): all six items in socket_tests.rs, SERIES.md:396, run.sh:29 and :52. fmt --check clean (no
  rewrite needed). First debug build after a touch: socket tests 18 passed, 2 ignored (= the builder's count).
- Clean runs (u3d-fix-proof.py clean; output u3d-fix-clean-output.txt, logs u3d-fix-*.log), release binary built
  after a touch (newer than it), fixed socket_tests.rs sha256 ca17a9f6...b2b9: --list 172 tests, T1-T7 by name,
  --ignored names M13's run and l2_timing_self_test.
  - SHORT (5 s): exit 0, 39 prints; up E (L0 = 0) 55,338 ns, down 47,330 ns, both would MISS, -3's p99.9 FAIL both:
    agree; CPU 0.364 core; slack line "now 1 ns, at start 1 ns ... SCHED_OTHER 0: the run condition accepts [1] ns".
  - CHRT-AFTER (item 1; chrt allowed here): `chrt -f -p 1` on npt-l2-release ~1 s into 6 s (SCHED_OTHER -> SCHED_FIFO,
    slack read from outside 0): exit 0 (INVALID before the fix: the test-engineer's u3d-test-run-chrt-after.txt); slack
    "now 0 ns, at start 1 ns ... SCHED_FIFO 1: accepts [0, 1]".
  - CHRT-BORN: the whole process under `chrt -f 1`, 3 s: exit 0; "now 0, at start 0 ... SCHED_FIFO 1" (still valid).
  - ENV (item 4): b'\xff' exit 101 in 0.07 s, "INVALID: NPT_L2_SELFTEST_SECS="\xFF" is not UTF-8 ...", no self-test
    print; 0, 601, abc INVALID as before; unset: still running at 3 s, no INVALID (the 100 s default).
- Bites (u3d-fix-proof.py bites; outputs u3d-fix-bites-unit-output.txt, -self1-, -self2-; logs u3d-fix-bite-<id>.log):
  each planted alone, built after a touch (binary newer than the touch every time), ran exactly 1 test, failed by
  name, restored with sha256 == the fixed state (socket_tests.rs ca17a9f6...b2b9, socket.rs 7ad7cb22...76f9). 16 of
  16 BITE (5 unit, 11 self-test). Every self-test bite printed all 39 lines before its panic.
  - Item 5 (T6, debug): X1 (agree always "disagree") fails "E MISS and p99.9 FAIL" (left "disagree"); X2 (loss share
    over Delivered) fails "106 lost of 10,106 ..." (pass[1] false). Both PASSED T6 before the fix (u3d-test-bites-extra).
  - Item 6 (T6, debug): E1 (early fate taken as a delay of 0) fails "down: the one fate released before it arrived is
    counted" (0 vs 1); the builder's T6b form (Lost taken as Delivered) fails "up: a fate released before it arrived"
    (10 vs 0); the test-engineer's T6b form (saturating_sub over Delivered|Lost) fails the Cross assert (n 1,010,
    p001 0).
  - Items 2-3 (self-test, release, 3 s): S2 (no stop wait) still "up: every datagram accounted for" 29,757 vs 30,000
    (order kept); S3 (2 x ppm) "up: 325 lost, outside P1's band 480-720"; S4 (capacity n) now "the fate log filled"
    30,000 (was the loss band); S5 (drain removed) now "the fate log filled" 60,000 (was the loss band); S5b (new: one
    pre-touch entry left after the drain) "up: the fate log holds 30001 fates of the 30000 sent" (by reading, the old
    code would pass it: band over 30,001 is 215-385 with ~325 lost); S6 (pre-touch skipped) now "the fate log
    pre-touched with 0 of its 61000 entries" (passed before).
  - Item 1 (slack): S7 (start left at the default) "the slack read back 50000 ns at start and 50000 ns now under
    SCHED_OTHER 0, outside [1]"; S9 (new: the header-time read writes 50,000) "1 ns at start and 50000 ns now ...
    outside [1]"; S7c (S7 plus chrt -f -p 1 from outside ~1 s in) "50000 ns at start and 0 ns now under SCHED_FIFO 1,
    outside [0, 1]": the real-time branch still checks the start.
  - Item 6 (self-test early assert): E2 (delays under 40 ms read as early) "up: Delivered fates released before they
    arrived" 29,675 vs 0, after all prints.
  - Item 4: U4 (the old Err(_) => 100 arm) with b'\xff': still running at 5 s, no INVALID: the new arm is what makes it
    INVALID.
- After the bites: socket_tests.rs, socket.rs, stats.rs and Cargo.lock hash as the fixed state / baseline; no npt_net
  process left.
- check.mjs run 1 (touch 03:33:48, shared folder, df 14 GB): 9/9 PASS; test 81.1 s, measured 267 passed 0 failed 0
  skips, bites 281 passed 0 failed 0 skips; self 43/43; redact 10/10, 150 items 0 findings; refs 481; marks 81. Both
  npt_net debug test binaries newer than the touch (03:33:54, 03:34:33).
- check.mjs run 2 (touch 03:35:21): 9/9 PASS; test 76.4 s, 267 / 281, 0 failed, 0 skips (= run 1 = the builder's);
  binaries 03:35:25, 03:36:01.
- clippy --workspace --all-targets -D warnings, after a touch each: default features exit 0; --all-features exit 0.
  `cargo +1.99.1 fmt --all --check` exit 0. run.sh `bash -n` OK.
- M13's lines: base 483-639 and new 484-640 both sha256 29b21eab4938ffa2...; socket_tests.rs hunks still only @@-1 (doc
  line + import) and the block appended after :637; Cargo.lock d0158c8a...6397 (no diff); no production file changed;
  the added lines carry no unsafe, libc, HashMap or pub item (two doc comments name HashMap and pub(crate), as before).
- Left as they are: socket_tests.rs:256 (an older test outside U3d's block) still names FATE_BYTES by path, kept so the
  diff touches nothing before M13; agree() keeps its &str (item 5's asserts as written); the poller's buffer; the CPU
  dilution note (netcode F6) is for U3d's report. SCHED_DEADLINE stays under the strict branch (1 and 1): the recorded
  switch is `chrt -f`, and whether ed4fb6d7ef68 covers DEADLINE is not verified (rust-reviewer), so a DEADLINE run
  reading 0 is INVALID, never a pass.
- Final sha256: socket_tests.rs ca17a9f6f6be0322...b2b9, SERIES.md a902d10dc16b80b8...0bd1, run.sh
  f4a73a5b2b558a50...dfe3.

DONE (fix pass, 2026-11-09 about 03:40 UTC). Nothing committed.
