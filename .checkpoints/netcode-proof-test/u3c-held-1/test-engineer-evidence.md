# U3C-HELD-1: test-engineer's evidence and costs

Agent: test-engineer. READ ONLY (no builds, tests, cargo, main-run.sh). Writes only this file (Write tool).
Branch: claude/netcode-proof-test-plan-hcky5g (PR #40). Started 2026-11-17.

## Plan
1. Read the card (netcode-proof-test.md:3876-3892), the kept evidence (merge.log:51-52,
   c62h-bound-build.md:426-455, advisor-c62h-bound-merge-stop-full.json C3-C4).
2. check.mjs: the test step, how the measured and bites builds are built and named (features, profile, target dir),
   the cargo test command line for each, FX-10 (2655-2657), any --no-fail-fast.
3. main-run.sh: usage, the lane lock, the 4 GB floor, the crates touch, log format (free lines).
4. Logs: time per run of the bites build's npt-net tests; free-disk lines before/after (merge.log, merge-2.log,
   other recent whole runs); number of whole runs so far that ran this test (basis for N).
5. delay.rs:528-543 and the test, to know what the unit test asserts and where the plant goes (only enough for the
   evidence part; the design is netcode-architect's).
6. Brief 8.1 (packing) and R4's list; du of the packable folders.
7. Write the answer blocks: diagnostic commands and N; confirm/stop criteria; fix evidence; CHECK-NFF-1 rule; disk;
   steps and reviewers.

## Findings (one line per finding, appended as found)
- F1 The bites test build is `cargo test --locked --workspace --all-features` from spikes/netcode-proof
  (check.mjs:2057); measured is `cargo test --locked --workspace` (:2053). Neither has --no-fail-fast. Target: main-run.sh
  exports CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target (main-run.sh:15,18); profile `test`
  (unoptimized + debuginfo, merge.log:26), so the binary is .gate/shared-target/debug/deps/npt_net-<hash>.
- F2 The failing binary was npt_net-ed8673eeeaf4db12 (merge.log:48). The same hash in te2-5/run1.log (`cargo test
  --locked --workspace --all-features --no-fail-fast`) and u4e/b11-list.log: the hash follows features/profile, not
  source (9e0f103, 561bf42 alike). Measured build's npt-net: 088534cbcaf56b90 (u4r/c6-end3.log).
- F3 TRAP: `cargo test --locked -p npt-net --lib --all-features` (te_m3_loop.sh:25; u4d2/r1-te-te25-npt-net.log) builds
  a DIFFERENT binary, npt_net-301de27e05915ce7: -p enables only npt-net's own features (wt-wtransport, wt-wtq,
  crates/net/Cargo.toml:12-15), while --workspace --all-features also turns on npt-l1's `bites` (crates/l1/Cargo.toml:12-15)
  and unifies the other members' dependency features. The loop must build with `--workspace --all-features --no-run`
  and pick the npt_net executable line, then assert its name is npt_net-ed8673eeeaf4db12.
- F4 Time per run of the ed8673 binary, whole (169 passed, 12 ignored): 29.43-31.05 s, 14 kept runs (te2-5/run1-10.log,
  u4r/c2-rul-reg.log, c3-reg.log, c2-b10-reg.log, c2-b10a-2.log; "finished in" lines). The 301de27 binary in 2 parallel
  loops: 32.52-35.51 s each (te-m3/loop.log:4-18). The build of 301de27 from touched sources: 58.39 s (te-m3/build.log);
  the bites workspace test build in merge.log: 21.30 s after the measured build (merge.log:26).
- F5 Runs on record of this test in a bites build: 1 FAILED (merge.log:51); ok in 14 direct ed8673 runs + 81 runs of
  301de27 (te-m3 run-1/2 60, p3 20 with 4 busy loops, u4d2 1) + the bites half of about 54 check.mjs test steps kept as
  summary lines (.gate/**/*.log "PASS  test"), whose test output is not kept. So 1 in roughly 97 to 150 bites runs
  (the advisor's 1 in 97 counts only logs with the "... ok" line).
- F6 The test (delay_tests.rs:2266-2308) is multi_thread with 2 workers; its println at :2292-2299 (largest excess,
  "samples above the bound [d, u]", first losses, stalls) shows only under --nocapture or on failure. The check
  (delay.rs:521-544) counts only; it keeps no state of the over-bound sample. So a loop of the unchanged code can show
  the failure again and its counts, but cannot by itself show the release-pending state the cause names.
- F7 main-run.sh: lock .gate/shared-target/.lane.lock exclusive (:27-28), waits for no cargo/rustc (:30-37), header with
  free MiB, HEAD, diff and untracked hashes (:39-43), STOP exit 3 under 4 GiB free (:44-47), touches every file under
  crates/ first and every 2 s (:48-53), runs each command without a shell from spikes/netcode-proof (:22,57-61),
  stops at the first non-zero exit (:66-72), provenance at the end (:77-84).
- F8 Precedent TE-M3 (card :2964-3002; te_m3_loop.sh): mode all 60 whole-binary runs caught nothing; mode one (the test
  alone, --nocapture, 2 loops x 300, 6 busy loops) caught it on the 9th run. Its fix's proof: P2 (the fixed test 600
  runs), P3 (mode all 10 x 2, 4 busy loops), P4 (two plants, each exit 101, restored by Edit, hashes), P5 (counted
  check.mjs), P6 (gate); the advisor's commit check ran 300 more.
