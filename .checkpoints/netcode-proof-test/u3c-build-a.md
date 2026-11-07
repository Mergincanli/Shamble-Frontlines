# U3c build A: DelayStream, fix 1's send gate, the mechanism tests, the kernel cross-check's script, SERIES rows,
# fixes.rs leftovers

Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_87550c07-bbd-1, branch u3c-build-a at ebe82b2 (the spike
branch head; `git merge --ff-only` refused: the worktree's own branch had diverged, so `git switch -c` per the brief).
Binding: u3c-plan.md rev 2, conditions C1-C16. Nothing committed: the main session merges this worktree.
Cargo: `cargo +1.99.1`, CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target, every command.

## Plan (blocks, each followed by a Progress line)
B0. tokio-rustls =0.26.6 (default features off, `ring`) in the workspace and npt-net; fetch, `cargo tree -d` (no second
    rustls), Cargo.lock diff [C7].
B1. Shared helpers: `l2::config::one_way_ns(&L2Config)` used by impair.rs's Settings::of and DelayStream [C5];
    L2Config gains `ws_max_message_bytes` (513) and `ws_max_request_bytes` (8,192), profile doc updated [C5].
B2. tcp_model.rs: pub(crate) `add_burst` (refuses a start before the model's now; prunes ended bursts; a cap) [C4];
    `with_one_way_ns` (the netted delay); cfg(test) loss-on-one-direction knob (1.6) and `unsent` (the bite's input).
B3. `ws_gate.rs` (crate level, shipped, pub(crate)): `snapshot_budget`, `may_write`, `TickWriter` (the per-tick
    writer decision: newest only, skip counted, never written later), cfg(test) bites (budget x2, queueing) [C9].
B4. `l2/ws_frame.rs`: the request and frame parser (RFC 6455 §5.1, §5.2, §5.5; caps) with its seeded random-input
    test [C6].
B5. `l2/delay_clock.rs`: the process-wide release clock (dedicated thread, 1 ns slack written and read back, a
    BTreeSet of (due, seq, key) + a BTreeMap key -> (due, seq, waker): one entry per live connection) [C3].
B6. `l2/delay.rs`: DelayStream: wrap, the pump task, records (one per poll_write, C1), the up bound (C2), lifetime
    (C3), bursts after wrap (C4), lateness (C10), limits (C6), held bytes, accepted_ns, the per-return record (C8),
    rows (header, per-second, summary; per-return pub(crate)), pub(crate) srtt_ns/outstanding_bytes/free_bytes/
    owed_bytes. `l2/delay_names.rs` names + SERIES test.
B7. `ws_tls.rs`: the WebSocket server's rustls config (ring, TLS 1.3, ALPN http/1.1, no process default); move the
    test certificate helpers out of wt_tests.rs into `l2/tls_test.rs` (cfg(test), no feature needed).
B8. Tests: DelayStream unit (in-memory duplex), replay, loopback TLS + tungstenite at P2 (30 Hz both ways), TLS config,
    one-process (WT features + WS server), mechanism tests (a)-(e) incl. (c) on DelayStream at P2 under B1.
B9. fixes.rs: adaptive_limit through snapshot_budget + grid test (e); stale three_part() removed with its reason;
    the B5 bite-row pin (fix 1 alone).
B10. SERIES.md rows (header, per second, ws_summary, burst_return's DelayStream part) + the tests that read them.
B11. 1.6's script under spikes/netcode-proof/results/u3c-kernel-check/ (run.sh, probe.py, compare.py; pass rule in the
     header) and the model side as an #[ignore]d test.
B12. check.mjs 9/9, clippy -D warnings (no features, all features), fmt; Cargo.lock diff; notes; DONE.

## Design decisions (as built; deviations listed in Open points)
- Tests drive tungstenite through its sync API over a test-only Read/Write bridge (tokio-tungstenite's Sink/Stream
  need futures-util, not a direct dependency; the brief allows only tokio-rustls in the workspace).
- owed on DelayStream's path = the bytes tungstenite offered in its latest poll_write and DelayStream did not take
  (tungstenite 0.30 always offers its whole out buffer: protocol/frame/mod.rs:273-287).

## Progress
- B0 done: tokio-rustls =0.26.6 (default-features off, ring) in Cargo.toml + npt-net; `cargo fetch` locked 1 package.
  Cargo.lock: +tokio-rustls 0.26.6 (deps rustls, tokio), npt-net gains "tokio-rustls"; nothing else. `cargo tree -d -p
  npt-net --all-features`: rustls only 0.23.46 (only getrandom 0.2/0.4 is doubled, as before). tokio-rustls's
  poll_write (common/mod.rs:278-312) writes the whole buffer into rustls (one record when <= 16,384 B) then the TLS to
  the socket; Ready(n < len) only when the socket blocks.
- B1 done: L2Config gains `ws_max_message_bytes` (513) and `ws_max_request_bytes` (8,192) (pub(crate) consts behind
  them, so no new pub item); the profile doc says bursts are on DelayStream::now_ns on the WebSocket path;
  `l2::config::one_way_ns(&L2Config)` is the one netting helper, impair.rs's Settings::of calls it (was impair.rs:104).
- B2 done: tcp_model.rs gains pub(crate) `with_one_way_ns`, `now_ns`, `add_burst` (refuses start < now or len 0;
  prunes ended bursts; at most MAX_BURSTS = 64 pending), `bursts`; cfg(test) `unsent` (SIOCOUTQNSD) and
  `with_loss_from(end)` (loss on one direction; fate words still drawn). The model's `now` is only ever written
  (tcp_model.rs:2074, :2204), so stepping in several advances equals one: the replay test relies on it.
  Decision: DelayStream runs the model on a relative axis (model 0 = wrap time): Sender::new calls cubic_init(0)
  (tcp_model.rs:640) and HyStart compares now with round_start, so model 0 must be the connection's start, as the
  kernel's cubictcp_init runs at establishment. Stamps are reported on DelayStream's clock (model + origin).
- B3 done: crates/net/src/ws_gate.rs (shipped, pub(crate)): snapshot_budget, may_write (W22's three conditions),
  Inputs, Snap, TickWriter (on_tick, written, skipped, held, budget); cfg(test) Bite {BudgetX2, Queueing}. Tests in
  ws_gate_tests.rs: (a) table [6,3,4,6,8,15] + bite; (b) monotone/floor over every µs 0-3 s + 100,000 seeded values;
  (b) seeded sweeps 64 x 20,000 ticks: 1,162,272 writes, skips by owed 85,941 / room 2,760 / budget 29,027 (write
  iff the three conditions, newest only, skipped never written later, outstanding <= N x size after a write, at most
  one snapshot above the socket); bites: queueing fails (b) and -31's one-above check, budget x2 fails (a) and (b).
- B4 done: crates/net/src/l2/ws_frame.rs: Scanner (client: limits + masks; server: boundaries only), Reason (6),
  Boundary {Head, Frame{message_end}}; fixed state, no allocation. Tests: boundaries in any chunking (1..10,000 B),
  each close at the header's end (payload never consumed), request cap, server direction unlimited, and C6's seeded
  random-input test: 20,000 cases, 24,633 boundaries, closes by reason [request 603, frame 3,044, message 26,
  unmasked 5,545, control 5,424, length 20]; never panics, never over-consumes, every record within its cap.
- B5 done: l2/delay_clock.rs: the process-wide release clock (thread npt-delay-clock, slack written by itself and read
  back through l2::thread, asked again at header time); schedule BTreeSet (due, seq, key) + table BTreeMap key ->
  (due, seq, waker), one entry per connection, replaced on reschedule, removed on fire/cancel/release; census() for
  the tests.
- B6 done: l2/delay.rs (DelayStream; pub: wrap, now_ns (in `impl DelayStream<()>`, so callable as
  `DelayStream::now_ns()`), add_burst, header_numbers, header_texts, take_second, run_summary, AsyncRead/AsyncWrite;
  pub(crate): srtt_ns, outstanding_bytes, free_bytes, owed_bytes, accepted_ns, returns + Return::row) and
  l2/delay_names.rs (names + SERIES tests). Model on a relative axis (origin = wrap). wrap bounds: ws_max_message_bytes
  1..=16,370 (a largest frame + 14 B header fits one TLS record, so the up read floor stays inside the budget: found
  when a 65,535 B limit made read_floor 65,571 > 65,536 and S was never read), ws_max_request_bytes 16..=32,768.
  Up records move into the model as each boundary is found (scan_up), so records before a bad frame enter first.
- B7 done: ws_config.rs (provider ring, tls_config TLS 1.3 + ALPN http/1.1, acceptor, ws_config(max) =
  max_frame_size = max_message_size = max, max_write_buffer_size 128 KB, write_buffer_size 0). TLS helpers moved from
  wt_tests.rs (its lines 19-157) into l2/tls_test.rs (cfg(test), via tokio_rustls::rustls); wt_tests.rs imports them;
  its P2 threshold and pacing untouched. rustls features here: ring + std only (no tls12) in the measured build;
  + aws-lc-rs, logging in the bites build.
- B8 (DelayStream tests) first full pass, 17 tests, 16.5 s with 8 threads, debug build (l2::delay::tests):
  * loopback TLS + tungstenite P2 30 Hz both ways, 300 ticks (10 s): every message in order; down 299 segments,
    4 stalls (= first_lost; 99.9 % band 0-15), rack 4; up 298 segments, 6 stalls (band 0-15), rack 6; receipt −
    release down p50 0.218 / p99 1.029 / max 1.030 ms, up 0.198 / 0.499 / 1.739 ms. Lateness down n 301 p50 0.154
    p99 0.891 p99.9 0.908 max 0.908 ms E 9.8 µs; up n 301 p50 0.174 p99 0.468 p99.9 2.580 max 2.580 ms E 12.9 µs:
    up's p99.9 > 1 ms = CONDITION 10's TRIGGER in this run (debug build, 8 test threads at once, 4 tokio workers);
    the main session's quiet evidence run decides.
  * (c) at P2 under B1 (2 bursts each, 230 ticks = 7.7 s, 4 connections together): faithful A 110 written / 120
    skipped, returns 7,350 B vs N 7 x 1,050 = 7,350 B (both); faithful B 151/79, same; owed at a write 0. Bites:
    budget x2 14,700 vs 7,350 and 16,800 vs 8,400; unsent-only 15,750 vs 7,350 and 15,750 vs 8,400: both caught.
  * replay: 182 steps, 177 releases, equal releases per direction, equal counters and delivered counts.
  * one-record rule: writes 16,406, 16,406, 2,724 (free 30,000), then 16,406 (one record of a 40,000 B write),
    13,594, then Pending with 10,044 owed; outstanding exactly 65,536.
  * held bounds (1 KB duplex, nobody reads): 65,360 B written before Pending; held max down 65,360, up 62,651.
  * Fixed on the way: `now() - MS` overflowed in a fresh process (test only); the flood task's expect at S's drop.
- B8 (one-process): wt_tests.rs gains `all_three_servers_run_in_one_process` (all features): wtransport + wtq routes
  + a WebSocket session (TLS ring, DelayStream at P1, tungstenite, 30 messages each way) in one process, no process
  default before or after; passes (wt tests 9/9 with --all-features, 5.1 s). The existing both_servers test is
  untouched.
- B9 done (fixes.rs, 1.11 / C15): adaptive_limit = ws_gate::snapshot_budget(m.srtt_ns(0), TICK) x SNAP; the
  recommended pin (0x52e7_b772_29d8_5ebb, [467, 77, 32, 10], [5,998, 3,999, 946, 1,056]) still passes, so the
  change moved no recovery. (e): e_the_adaptive_limit_is_the_shipped_snapshot_budget (every 7th µs to 3 s, no sample,
  models settled at P0-P4) passes. three_part() removed with its reason (comment after two_part; F11/B3: round 2's bar,
  the v3 recommended row fails it with certainty, p50 654 > 600) and its two uses (multi_burst_rows' pass lines,
  ws_limit_tail's chain). B5's bite-row pin: the_fix_1_row_is_pinned_at_b1 = (0xb6f3_4ac0_188a_2727,
  [675, 523, 429, 228], [0, 0, 0, 0]) over 6,000 phases (p50 653, p99 2,172, max 5,160 ms).
- B10 done (1.7): SERIES.md gains "## DelayStream (U3c ...)" before "## Open naming questions": 67 per-second rows
  (per direction 20 model counts, 2 held, 7 lateness; 6 close reasons, shutdown timeouts, split writes, dropped
  returns), the ws_summary paragraph + 4 own fields, 9 burst_return fields (DelayStream's part of r2:379 (3)), 21
  header numbers + 7 header strings; the U3b "Carried to U3c (M15)" paragraph now says done. Generated by a scratchpad
  script (series_rows.py, not kept). delay_names' tests (every name's row; README §1 rule; no name twice) pass; the
  rows test in delay_tests checks the real rows carry exactly those names.
- B11 done (1.6's script, NOT run with iptables): spikes/netcode-proof/results/u3c-kernel-check/run.sh (root; the
  pass rule in its header, written before any run; per-run iptables rule on lo --dport PORT only, removed after each
  run and by an EXIT trap; `iptables -S` before/after must match, else exit 3; refuses if PORT is busy or already has a
  rule; records uname -r, CONFIG_HZ from /proc/config.gz, 10 sysctls), probe.py (the scout's probe + TCP_CONGESTION
  cubic on listener, client and accepted socket, read back; JSON with every delay and send lateness and tcp_info),
  compare.py (one implementation of the stall rule for both sides; summary / rtt / judge). Model side:
  tcp_model::tests::kernel_cross_check_model (#[ignore]; env NPT_KC_RTT_NS, _MESSAGES, _RUNS; KCMODEL lines) and the
  cfg(test) knob with_loss_from + its unit test loss_from_one_end_spares_the_other. Rule decision: stall threshold
  half a gap, not one gap (reason in the header); the one-gap count is reported. Dry run of the plumbing only, no
  iptables: probe.py 60 messages on port 47993 without loss: 60/60, cubic/cubic set, rtt 54 µs, send lateness p99
  112 µs; model 5 seeds at 54 µs: 264 episodes, 9 above the RACK bound (2 timeouts + double losses), own repairs
  rack 272 / probe 0 / timeout 2; judge prints MISS as expected with no kernel loss. The main session runs run.sh.
- B12 (fixes found in review of my own code): current() now also wakes a waiting writer when the step freed the
  budget (the pump's clock entry may have moved past events another call stepped: a delayed-wake risk); (c) now also
  asserts S12's per-burst times at each faithful return (first byte down >= one least delay, first message >= it):
  456.9 / 1,789.0 / 474.0 / 571.6 ms in the last run.
- Lateness, release builds, the loopback test alone (10 s, n = 301 per direction, so p99.9 = max), three runs:
  run 1 down p50 0.108 p99 0.339 max 0.360 ms, up p50 0.161 p99 0.944 max 3.450 ms (TRIGGER: up);
  run 2 down 0.107 / 0.292 / 0.419, up 0.167 / 0.401 / 0.564 (no trigger);
  run 3 down 0.107 / 0.269 / 1.150 (TRIGGER), up 0.164 / 0.910 / 1.883 (TRIGGER). Load average 0.6-1.3 of 4 cores
  (build agent B active in parallel). p50 about 0.11-0.17 ms: above the plan's 10-50 µs guess.
- B12 done: check.mjs 9/9 (final run): "check: all 9 steps pass"; test step "measured build: 227 tests passed, 0
  failed; bites build: 241 tests passed, 0 failed" (+34 tests in the measured build, +35 in the bites build, 1 new
  ignored test, the kernel cross-check's model side). clippy -D warnings with and without features: clean. fmt:
  clean.

## Result
- check line: `check: all 9 steps pass` (test: measured 227 passed, bites 241 passed, 0 failed).
- Cargo.lock: +[[package]] tokio-rustls 0.26.6 (checksum c9cc2678c2cdd569ef8215e2afd7954ada2ae20b4fdd2c5fe6139a3b02d105db;
  deps rustls, tokio); npt-net's dependency list gains "tokio-rustls". 11 insertions, nothing else.
- Public surface added: `DelayStream` (wrap, now_ns, add_burst, header_numbers, header_texts, take_second,
  run_summary, AsyncRead, AsyncWrite, Debug) and L2Config's two fields; everything else pub(crate) or cfg(test).

## Open points (decided differently from rev 2, or left)
1. Kernel cross-check stall threshold: half a send gap, not "more than one send gap" (run.sh's header gives why: a
   RACK repair on loopback is one gap + RTT ± pacing noise, so a one-gap threshold sits in its own noise). The
   one-gap count is reported beside it.
2. The per-return row (`Return::row`, `returns()`) is pub(crate), not pub: rev 2's pub list names the header,
   per-second and summary rows; U6's WsServer (in npt-net) reads the returns.
3. wrap bounds ws_max_message_bytes to 1..=16,370 and ws_max_request_bytes to 16..=32,768: a larger message limit
   makes the up read floor exceed the 64 KB budget and S is never read (found in the limits bite).
4. Condition 10's trigger fired in 2 of 3 release runs of the 10 s loopback test here (n = 301, so p99.9 = max;
   maxima 3.45 ms up, 1.15 ms down; p50 0.11-0.17 ms). Not judged by any test (C16); the main session's quiet
   evidence run decides and brings it to the advisor if it holds.
5. Tests drive tungstenite through its sync API over a test-only Bridge (tokio-tungstenite's Sink/Stream need
   futures-util, not a dependency). U6's WsServer will need the same or futures-util: a U6 decision.
6. owed on DelayStream's path is the bytes tungstenite offered in its latest poll_write and DelayStream did not take
   (tungstenite 0.30 offers its whole out buffer each call); the plain path's owed is U6's (C14).
7. Per-second `_max_` fields of the model (backoff, stall, out-of-order held) are the connection's maxima so far, not
   per-window (the model keeps maxima only); SERIES.md says so.
8. Lateness uses l2::stats' histograms: four per connection, about 0.54 MB per connection (48 connections: about
   26 MB). Fixed per connection, not per message.
DONE
