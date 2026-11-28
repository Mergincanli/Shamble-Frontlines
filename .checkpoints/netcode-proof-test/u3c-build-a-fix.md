# U3c build A: the completeness critic's fixes (u3c-critic-a.md) and the advisor's 2026-11-08 rulings

Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a7fafe27dcdee96cd. Base 4f3a560 (spike branch head;
`git merge --ff-only 4f3a560` fast-forwarded from 626df67), then build A's patch wip/wf_87550c07-bbd-1.patch applied
(made on ebe82b2; applied clean). Nothing committed: the main session merges this worktree.
Cargo: `cargo +1.99.1`, CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target, every command.
Never iptables/tc. Binding: u3c-plan.md rev 2 (C1-C16), the advisor's side rulings of 2026-11-08 (advisor-u3c-plan.md,
"Side rulings: U3c evidence").

## Start
- check.mjs on the applied patch, run 1: all 9 pass (measured 227, bites 241; test step 81.0 s).
- run 2: all 9 pass (measured 227, bites 241; test step 56.9 s).

## Plan (one block per fix; a progress line after each)
F1. W1 (flaky burst test): the burst is taken from the model's own stamps. Traffic runs as a loop (client task writes
    a frame per tick until told to stop; the test writes down per tick); the burst is added at the clock's now
    (retried if the model has passed it), read back from the logged Step::Burst (model axis); traffic runs until the
    model's now is past the burst's end. If scheduling left a direction without a transmission inside it, another burst
    is added (counted, every burst judged). Judged on one atomic snapshot of sent_log + counters + logged bursts: every
    transmission inside a burst lost, every other one not, lost_burst == inside, both ways > 0.
F2. W2 (racy replay): one cfg(test) `snapshot()` taking logs, origin, counters, delivered, model now and sent_log under
    one lock (one `current()`); the replay and the burst test read only that.
F3. W3 + M1 (shutdown bound): the pump checks the deadline first-class: at or past it, S's shutdown gets one last poll
    (if every down byte was taken), else S is dropped and counted; no path re-arms a past deadline. Test (M1): an S
    that takes everything and never completes poll_shutdown: the shutdown ends with a counted timeout, the pump ended
    at its first poll at or after the deadline, and the pump's polls and S's poll_shutdown calls during the wait are
    counted (cfg(test) counters) and small (a busy loop gives thousands). Bite: verified on the old code (scratch
    mutation, reverted).
F4. M2 (a bite per limit reason): a cfg(test) switch in ws_frame::Scanner turning one check off; DelayStream gets a
    cfg(test) setter. For unmasked, control (size and fragmented), 64-bit length, request cap and fragmented message:
    with that check off, the same input's payload enters the model (the "nothing entered" assertion fails). The length
    bite needs the frame check off too (a top-bit length is always above 513): stated in the test.
F5. W4 (kernel script, run nothing): `iptables -I INPUT 1`; probe.py reads tcpi_total_retrans and
    /proc/net/netstat TCPRetransSegs before and after; compare.py: zero kernel retransmissions = VOID; judged
    episodes > tcpi_total_retrans = VOID (advisor 1a); VOID/INVALID rerun once on a quiet container, a second goes to
    the advisor; a MISS is never rerun (1b), written in the header and the verdict; every run's out-<UTC>/ kept (no
    overwrite); half a gap judged, one gap reported (1c), the header records the 2026-11-08 ruling (1d).
F6. W5 (hidden 1 s threshold in the drop test): judged on state: pump_done and S's EOF, with the 2 s waits replaced by
    hang guards (generous, never judged) and the "nothing was released" claim checked on the model's logs (no down
    release logged) instead of on the clock.
F7. W7 (one room-step definition): fixes.rs's room step calls ws_gate::may_write (n = scale x snapshot_budget, or
    u64::MAX when not adaptive; the "no takeover this tick" rule stays in fixes.rs). Both pins re-run: must be
    unchanged (advisor 3).
F8. W8 (bursts() doc): the doc matches the behaviour (pruned only in add_burst), or bursts() filters.
F9. Claims made proven:
    - ws_config "one source": a test reads max_frame_size, max_message_size (513 from L2Config), max_write_buffer_size
      (128 KB), write_buffer_size (0); a bite with WebSocketConfig::default() fails it.
    - "TLS 1.3 only": a TLS 1.2-only client (rustls with the tls12 feature? check what the measured build has; else a
      raw ClientHello offering only 1.2) is refused.
    - Loopback identity: each message's receipt = its model delivery stamp + the measured release lateness, within the
      measurement's resolution, judged on stamps the code records.
    - (b)/(c) tautologies: given teeth (an independent recomputation) or stated as tautologies in the test comment.
    - base_rtt_ns netting: a test with a non-zero base shows the model's one-way = (rtt − base) / 2 through the
      shared helper, on the DelayStream path.
F10. Advisor 2: tokio-tungstenite in the tests. futures-util =0.3.34 as a dev-dependency (default features off,
    ["sink", "std"]); Bridge deleted unless a test states why it needs the sync API; (c) and the loopback test through
    tokio-tungstenite's adapter; one loopback test with split read/write halves in separate tasks. Lock diff stated.
F11. Advisor 4: the loopback test's tick count from an env var in an #[ignore]d evidence variant (n >= 10,000 per
    direction), the check.mjs version short.
F12. End: check.mjs 9/9 three runs in a row (counts each); clippy -D warnings with and without features; fmt; each
    previously flaky test (the burst test, the replay test) 30 times in a release loop; notes; DONE.

## Progress
- F10 (advisor 2) done: futures-util =0.3.34 dev-dependency (workspace pin, default features off, ["sink", "std"]);
  Cargo.lock diff: one line, "futures-util" under npt-net's dependencies (no package added). The Bridge is deleted;
  every WebSocket test runs through tokio-tungstenite (accept_async_with_config / client_async, its Sink and Stream).
  New: split_halves_in_separate_tasks_both_ways (90 msgs each way at P2, halves in 4 tasks): passes.
- F2 (W2) done: DelayStream::snapshot() (cfg(test)) reads origin, now, counters, delivered, logs, sent_log and the
  bytes S took under one lock after one step; the replay and burst tests judge only that. The replay now also runs
  until it holds a burst loss down and a profile loss (it asserted them on a 90-tick run before: ~2.6 % chance of no
  profile loss at 2 % over ~180 transmissions, a second flake source).
- F1 (W1) done: the burst is added at the clock's now + 1 ms (retried if refused), read back from the log (model
  axis), traffic runs until the model is past it, another burst if a direction had no transmission inside; judged on
  one snapshot. First run: 1 burst, [9, 10] inside, all lost, 50 ticks.
- F3 (W3 + M1) done: the pump ends at its first run at or after the deadline whatever S waits for (S's shutdown
  included). New test a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning: 2 pump runs during the
  300 ms wait (1 after the deadline), 2 S poll_shutdown calls. Bite checked on the old code (scratch revert, then
  restored): no end in 30 s, 685,286 pump runs and as many S poll_shutdown calls.
- F4 (M2) done: Scanner cfg(test) set_off + DelayStream::bite_check_off(&[Reason]); per case: Unmasked, Control
  (126 B and fragmented), Message, Request (read limit lifted too) enter with their check off; Frame alone off is
  counted as Message, Frame+Message off enters; Length off is counted as Frame (a top-bit length can never end, so
  nothing can enter: stated in the test).
- F6 (W5) done: the drop test judges that the client reads exactly what S took, then EOF; waits are 30 s hang guards.
- F7 (W7) done: room step = takeovers == 0 && ws_gate::may_write(outstanding, owed, free, SNAP, n), n = scale x
  snapshot_budget (u64::MAX without the adaptive limit). Pins: to run.
- F8 (W8) done: bursts()'s doc now says what it holds (pruned only by add_burst).
- F9 partly: ws_config one_source() + the_tungstenite_config_comes_from_l2config (default config fails every field)
  + tungstenite_enforces_the_limit_on_the_plain_path (513 ok / 514 MessageTooLong; default: 514 passes) + the real
  server's get_config() checked in the TLS test. a_tls12_only_client_is_refused: raw ClientHellos (no
  supported_versions; supported_versions [1.2]) get alert 21 [2, 70], acceptor errors SupportedVersionsExtension
  Required / Tls12NotOfferedOrEnabled; control (1.3 offer) gets a ServerHello (22, type 2). Base-RTT test: 45 ms
  in one_way_ns, ImpairSocket's Settings, the header and the model's own release stamp (50 ms without the base).
  Loopback up identity: hand-over = delivery + lateness exactly, inside the yielding poll's [t0, t1] (DelayStream now
  hands one record per poll_read); first run receipt − (delivery + lateness) p50 0.024 max 0.059 ms, window max
  0.166 ms. (c): pre-write owed = 0 dropped (tautology, said in the comment), owed AFTER each write asserted.
- F7 pins: both unchanged after the swap to may_write (advisor 3): the_recommended_fixes_are_pinned_at_b1
  (0x52e7_b772_29d8_5ebb, [467, 77, 32, 10], [5,998, 3,999, 946, 1,056]) and the_fix_1_row_is_pinned_at_b1
  (0xb6f3_4ac0_188a_2727, [675, 523, 429, 228], [0, 0, 0, 0]) pass; e_the_adaptive_limit... passes.
- F9 (b) done: the post-write "outstanding <= N x size" check removed from ws_gate_tests' sweep and both by-construction
  properties named as such in its comment; the parts that can fail stay (iff, newest, never later, counts, -31).
- F5 (W4 + advisor 1) done, run nothing with iptables: run.sh inserts with `iptables -I INPUT 1` (deletes with -D
  INPUT by spec), header records the 2026-11-08 ruling (half a gap judged, one gap reported), the VOID/INVALID
  rules (zero tcpi_total_retrans; judged episodes > tcpi_total_retrans; lateness p99 > 1 ms; not all delivered; no
  model lines), the rerun rule (enforced by `compare.py may-run` from earlier out-*/verdict.json: refuses after a
  PASS or MISS or a second VOID/INVALID; a folder without a verdict blocks until an ABORTED file says why), out-<UTC>/
  only, never overwritten, loadavg before/after each run. probe.py records /proc/net/snmp RetransSegs and TcpExt
  TCPFastRetrans/TCPLossProbes/TCPTimeouts/TCPLostRetransmit before and after (Linux has no TCPRetransSegs in
  /proc/net/netstat; RetransSegs lives in /proc/net/snmp). Checked: bash -n; a 30-message loss-free probe on lo
  (no rule): delivered 30/30, cubic/cubic, total_retrans 0 -> VOID as written; 15 synthetic checks of compare.py
  (PASS, VOID x2 kinds, INVALID x3 kinds, MISS, may-run after each, ABORTED) all ok (scratchpad kc_synth.py).
- F11 (advisor 4) done: loopback() shared; loopback_tls_tungstenite_at_p2_30_hz_both_ways = 300 ticks (check.mjs);
  #[ignore] lateness_evidence_run reads NPT_WS_LATENESS_TICKS (default 10,050 = 5.6 min), prints loadavg at start
  and end and asserts >= 10,000 samples per direction.
- Design note (DelayStream, shipped): poll_read now hands one record per call. Why: the up receipt identity needs the
  poll that yields a message to be the one that took its last byte. Effect on C10's up lateness: records released at
  the same model stamp are now each timed at their own hand-over (the reader's next poll), not all at the first.
- fmt clean; clippy -D warnings clean with and without features; the one-process test (all features) passes.
- redact step caught "RFC 5246 §7.4.1.2" in ws_config_tests.rs as an IPv4 address: reworded ("RFC 5246's
  ClientHello"); fmt + redact re-checked.
- F12 done. check.mjs three runs in a row (00:27-00:30 UTC, loadavg 0.94 / 1.33 / 1.97 at their starts): each
  "check: all 9 steps pass", test step "measured build: 233 tests passed, 0 failed; bites build: 247 tests passed,
  0 failed" (+6 each: the shutdown-bound, base-RTT, split-halves and three ws_config tests; lateness_evidence_run
  ignored). clippy -D warnings with and without features: clean. fmt: clean.
- Flaky loop (release binary, each test 30 times, npt-net's debug lib suite looping beside it as load; loadavg
  1.44-2.11 of 4 cores; scratchpad flaky_loop.sh): an_added_burst_loses_every_transmission_inside_it_both_ways 30 of
  30 (each: 1 burst, 50 ticks, [9, 10] inside, all lost); a_replay_on_a_pure_model_gives_the_same_counters_and_stamps
  30 of 30 (each: 91 ticks, 1 burst).
- Cargo.lock vs base 4f3a560: + tokio-rustls 0.26.6 (build A's) and npt-net's list gains "tokio-rustls" and
  "futures-util" (mine); no other change.

## Not done (not this agent's)
- C10's quiet lateness run (lateness_evidence_run is ready for it), the kernel cross-check run, the C12e root run, the
  wt_tests P2 flake: the main session's.

## Open points for the main session
1. DelayStream's poll_read hands one record per call now (shipped behaviour change, for the up receipt identity);
   C10's up lateness is then timed per record at its own hand-over.
2. Two limit checks are backed by a second check, so their single-check bites move the counted reason instead of
   letting the payload in: Frame (also a Message), Length (also a Frame; a top-bit length can never end, so no
   setting lets its payload in). Frame+Message off lets it in; stated in the test.
3. Down receipt on the loopback test stays a bound (receipt >= release) with the rest printed: after S's poll_write
   the bytes are in the kernel's TCP and the client's TLS, which no in-process stamp sees. The up receipt is the
   identity (inside the yielding poll's own window).
DONE
