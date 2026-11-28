# U3c build A: completeness critic (test-engineer)

Started 2026-11-07. Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_87550c07-bbd-1, base ebe82b2,
uncommitted. Plan: u3c-plan.md rev 2, scope 1.1-1.5, 1.7, 1.11, 1.6's script; C1-C11, C15, C16.

## Plan
1. Read diff + new files (ws_gate, delay*, ws_frame, ws_config, tls_test, tcp_model changes, fixes.rs, SERIES.md, results script).
2. Item by item: met / missing / wrong / unproven with file:line.
3. Run check.mjs with CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target.
4. Bite checks: do the bites actually bite (reasoning + scratch probes in a COPY, never the worktree).
5. Verdict.

## Findings (as I go)
- (resume 2026-11-07 late) Prior session left only the plan block. Re-read plan rev 2, advisor-u3c-plan.md, build-a notes.
  Build's own open points: stall threshold half-gap (not plan's one gap); per-return row pub(crate); wrap bounds
  ws_max_message 1..=16,370; C10 trigger fired in 2/3 release runs; tests use sync tungstenite over a Bridge, not
  tokio-tungstenite (plan 1.5 says tokio-tungstenite).
- CHECK RUN 1 (23:35, machine up 3 min, load 0.3, nothing else running): `check: 1 step(s) failed`. test step FAIL:
  measured build npt-net lib "129 passed; 1 failed; 10 ignored", failing test
  l2::delay::tests::an_added_burst_loses_every_transmission_inside_it_both_ways (delay_tests.rs:857-889). check.mjs
  keeps only the last lines, so the panic text is lost. Other 8 steps PASS. Log: scratchpad check-run1.log.
- CHECK RUN 2 (after ~100 test runs): `check: all 9 steps pass`, "measured build: 227 tests passed, 0 failed; bites
  build: 241 tests passed, 0 failed" = the build's claim. So the claim reproduces only sometimes: flaky.
- ws_gate.rs: snapshot_budget/may_write match fixes.rs:775-776 (outstanding + SNAP <= N x SNAP, owed 0, free >= SNAP).
  fixes.rs's room step still inlines the three conditions instead of calling may_write (only adaptive_limit goes
  through ws_gate): drift risk; rev 2 1.4 (e) only names adaptive_limit. Minor.
- ws_gate_tests (b): "writes happen only on a tick" is structural (only on_tick writes); "outstanding <= N x size after a
  write" is implied by the iff check with the same N. Queueing bite + budget x2 bite bite (asserted error text). (a)
  table pinned with literals [6,3,4,6,8,15].
- tcp_model.rs: add_burst refuses start < now and len 0, prunes ended, MAX_BURSTS 64. bursts() doc says "not yet ended"
  but the list is only pruned on add (doc drift, minor). with_loss_from cfg(test) zeroes the drawn loss only.
- delay.rs wrap converts profile bursts from DelayStream's clock to the model axis (delay.rs:554-568). OK.
- Loops: measured lib suite alone 6/6 + 8/8 pass (19.45 s each); bites binary 2 copies x 3 rounds pass; measured
  binary 3 copies at once x 3 rounds: 1 FAIL of 9 -> a SECOND flaky test:
  l2::delay::tests::a_replay_on_a_pure_model_gives_the_same_counters_and_stamps panicked at delay_tests.rs:944:9
  "dir 0": the pure model released 5 more down records (85,848..89,936 B at model 2,969,294,719 ns) than DelayStream's
  logged releases. ROOT CAUSE (code): the test reads ds.logs() (delay_tests.rs:901), ds.model_state() (:902) and
  ds.model_now() (:903) in three separate locks; each goes through current() (delay.rs:704-719), which steps the model
  to now while the pump is alive. The pure model is stepped to end_t = the LAST read but compared with the releases
  logged at the FIRST read. Fix: one locked snapshot, or wait for pump_done before reading.
- More runs: 3 copies x 6 rounds (18) pass; delay tests only, 6 copies x 4 rounds (24) pass. STALL PROBE (scratchpad
  stall.sh: SIGSTOP the test process): burst test frozen at 0.15 s for 1.2 s -> FAILS at delay_tests.rs:887 "both
  ways: [0, 0]"; frozen at 0.6 s for 0.3 s -> passes [9, 9]. The burst is placed at now()+500 ms (:862) on the wall
  clock while the transmissions come from traffic's wall-clock ticks: the test needs the process scheduled inside a
  600 ms wall-clock window (C16). Likely cause of check run 1: a start-up stall (first run after boot, cold 72 MB
  debug binary; the test is among the first alphabetically). Run 1's exact panic line is not recoverable.
- Same probe on dropping_it_ends_the_pump_and_drops_s (stall 1.3 s at 3 ms): FAILS at delay_tests.rs:982 "nothing was
  released", left 64: the test needs the drop within the 1 s one-way delay (fixed(2_000)). Low severity.
- SHUTDOWN BOUND NOT ENFORCED (C3) + SPIN: delay.rs:1237-1249: when every down record is taken and flushed but S's
  own poll_shutdown stays Pending, the else-if deadline branch is never reached; due() (delay.rs:395-399) keeps the
  past deadline, so reschedule fires the clock at once on every poll: a hot loop. Scratch probe (npt-copy in the
  scratchpad, S = duplex whose poll_shutdown is always Pending, wait 300 ms): shutdown not complete after 3.0 s,
  0 timeouts counted, pump alive, 332 USER_HZ ticks (3.3 CPU-s) in 3 s. Real case: tokio-rustls's poll_shutdown
  Pending while close_notify can't be written (non-reading peer, full socket). No test covers it.
- C7 confirmed: cargo tree -d -p npt-net --all-features: rustls once (0.23.46); only getrandom 0.2/0.4 doubled.
  Measured build rustls features ring + std.
- Public surface = rev 2's list exactly. Met.
- ws_config(): no test reads max_frame_size / max_message_size / max_write_buffer_size / write_buffer_size: unproven.
- TLS test: ring + TLS 1.3 negotiated + h3 refused + no process default: met; no TLS 1.2-client refusal (minor).
- Loopback P2 test: asserts r >= wrote + least and got >= r (bounds), not rev 2's identity "delay = model delivery
  stamp - write stamp + lateness" (delay_tests.rs:1229-1240): partial.
- (c): pre-write owed check (delay_tests.rs:1332) can't fail (may_write needs owed == 0); post-send half is real.
  Faithful outstanding exactly 7,350 = N 7 x 1,050 at both returns (tight), bites over it: met.
- Limits bite only for the Frame reason (limit raised, delay_tests.rs:772-780); other 5 reasons have no bite.
- run.sh: the DROP rule is appended with `iptables -A` (RULE at run.sh:69, -A in the run loop): an earlier
  `-i lo -j ACCEPT` in INPUT would make it dead (fails safe as MISS, but a wasted evidence run); half-gap threshold
  deviates from rev 2's one gap (declared and reasoned in run.sh's header): needs the advisor's ruling before the run,
  since the rule is fixed pre-run.
- C10: trigger fired in the build's own runs (debug up p99.9 2.58 ms; release up max 3.45 / 1.88 ms, down 1.15 ms):
  rev 2 says stop and bring it to the advisor before U3c closes: open.

## Verdict
gaps. Ranked: (1) burst test wall-clock flake (made check fail in my first run); (2) replay test non-atomic snapshot
flake; (3) C3 shutdown bound not enforced + spin; (4) C10 trigger open; (5) C11 threshold deviation + -A rule;
then the low items. DONE.
