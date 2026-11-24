U6c's build (the probe, the load, the page host and the certificate; u6c-brief.md rev 1.1, Block 2 last per C4).

## Start
- Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3a3e3fdf559de095; `git merge --ff-only 8c81952` fast-forwarded
  from f5030a3; `git rev-parse HEAD` = 8c81952856cdc3c8d44ffcd16ac278312c9ea1c2 (`<base>`). Base counts: measured 807,
  bites 904; base test step 1086.7 s.

## Plan (blocks in order; the launch's added items placed)
1. Block 1 (P1a, crates/net l2/probe.rs + socket.rs `pretouch_fate_log` + stats.rs `FateEntry`): poller, FateEntry,
   samples/forms, ProbeDatagram, Pacer, probe_peer (C2: one socket, sends without pause, errors counted; C3: echo
   once), leg_series (NC-SB-16: packets outside B1's bursts). Tests 10 + C2/C3's.
2. Block 3 (P1b, crates/server probe.rs, main.rs `mode`=probe; crates/bots npt-load probe): C1 (b), C2 (peer gate,
   rebind retry 2 s), leg logs `room-02-<leg>.jsonl` (NC-SB-1), leg headers (NC-SB-3), leg_edge/cpu_stat at warmup
   and after (C1 (d)), NC-SB-9 names, NC-R2-4 records. Tests 8 + C1 (b) + C2.
3. Block 4 (B1, crates/bots glue + npt-load run|calibrate|probe): C1 (c) one argument, settings' mode;
   F-JOIN-ONCE (step only on Open; Closed counted; step at once; no Join-deadline end asserted); NC-SB-7/-2 header
   and footer on every end, no rejoin. Tests 10 + C1 (c). A2's command named.
4. Block 5 (H1, crates/host serve): U7's seven names; `/npt/cert-hash-NN.json` per room only; C1 (e) query ignored;
   CSP, nosniff, no-referrer; TLS 1.3 only + TLS-1.2 client test. Tests 10 + C1 (e).
5. Block 6 (H2, crates/host issue): C1 (a) two arguments, compiled-in dir; stdout lines exact; INF-SB-6 temp name then
   rename for the chain; security conditions 2-4. Tests 9 + C1 (a).
6. Size line after Block 6; if past 5,400, Block 2 not built.
7. Block 2 (P2, C-U6-8) last, as the valve.
8. End: rustfmt diff by Edit; full check.mjs; plants P1-P21, TE2-TE4.

## Interfaces re-read at <base> (file:line)
- StealReader: server/src/proc_read.rs:42-83, pub(crate), one vCPU's `cpu<N>` line; `clock::now_ns` server/src/clock.rs:21.
- Machine::read_local: log/src/lib.rs:52-103. Settings rules (SEC-U6AD-2): server/src/main.rs:69-309 (cap 64 KiB,
  deny_unknown_fields, every value range-checked, texts <= 256 B, no control chars). Exit codes main.rs:22-24.
- Behaviour::host_step: bots/src/lib.rs:887-899. Token file: main.rs:351-365 (`<slot> nptj_<22 b64url>` lines, 0600).
- WtClient::connect(addr, hash, clock fn) net/src/wt_server.rs:562; WsClient::connect(addr, name, hash, clock)
  net/src/ws_server.rs:881; both `Connecting` until `link()` is polled through a &mut call (poll_recv), Closed on a
  failed handshake; Recv::arrival_ns link/src/lib.rs:81-87.
- Hash file: server/src/cert.rs:122-145 (`algorithm`, `value_hex`, `validity_ns`; 0644; settings field `hash_path`).
- WsServer's PEM loader: net/src/ws_server.rs:290-306 (CertificateDer::pem_slice_iter, PrivateKeyDer::from_pem_slice).
- TLS server config shape: net/src/ws_config.rs:27-39; pinning verifier's verify_tls12_signature Err: net/src/pin.rs:49-56.
- ImpairSocket as U6b leaves it: net/src/l2/socket.rs (set_peer :244, Drop :440-446 signals without joining).
- U8's names: host.json keys bind, cert_chain, cert_key, dist, hash_template, host_keys (wip/u8-build.patch:4098-4111);
  load_settings fields (patch :2180-2186, `room` a 2-digit string, `machine` an object, probe pin 64 zeros).

## Findings
- F-SIZE-1: Block 1 added 1,267 lines (probe.rs 778, probe_tests.rs 416, socket.rs 26, socket_tests.rs 35 with 189
  removed as moved, l2.rs 7, lib.rs 5) against the brief's ~560: the moved test code counts as added in probe.rs
  (~150), plus LG's end, the down sender and the leg's series. Later blocks are written tighter to keep the valve.
- F-ECHO-RACE: two downs arriving microseconds apart may both be echoed (each once) if an up datagram leaves between
  them; the probe_peer test accepts [9] or [8, 9], both C3-correct (newest at the time, never twice).

- F-PORT: npt-server cannot name quinn's `AsyncUdpSocket::local_addr` (no quinn dependency), so the leg-edge row is
  matched by the settings' `wt_bind` port, and probe mode refuses port 0 (main.rs `probe_settings`). U8 binds 30003.
- F-POLLER-STATE: `start_poller` takes a state value the task returns at its stop (`(Polled, S)`), so the probe reads its
  echo records back; the tests pass `()`.
- NC-R2-4 (which records carry P1's two reads): the leg log's header params `l2_timer_slack_ns`, `l2_timer_slack_start_ns`
  and `l2_sched_policy` (ImpairSocket's header read, taken at the leg's END edge after the tail, socket bound, so a switch
  inside the window shows) and `probe_start_timer_slack_ns`, `probe_start_sched_policy` (read 10 ms before the window
  starts, the window's start edge). Both in the header record of `<log>-P1.jsonl`. U8's probe_presence reads these.
- NC-SB-9 (names for U8's interface list): per-leg rate counts `event.l2_summary.probe_up_window_count`,
  `probe_down_window_count`, `probe_window_ns`, `probe_down_sent_count`, `probe_polled_count`; header
  `probe_rate_pps`, `probe_sent_count`, `probe_packets_count`; P1's slack and policy reads as in NC-R2-4 plus the pacer's
  `probe_pacer_timer_slack_ns`, `probe_pacer_sched_policy`.
- F-WT-CLOSE: a WebTransport client sees the room's end only at quinn's default 30 s idle timeout (no idle setting in
  wt_server.rs; the room's process exits after closing its sessions, so the CONNECTION_CLOSE is likely not flushed:
  npt-server's end has no `Endpoint::wait_idle`); WebSocket sees the close at once (b4-test2.log: ws 3.31 s, wt 30.16
  s). npt-load ends at its links' close or its duration either way, footer written; R1 costs nothing (LG's duration is
  the room's + 30 s). The wt test now ends at its duration. Taker: U6d/U6e (npt-server's end, U6b's path).
- F-JOIN-DEADLINE-READ: the room's decoder counter (`join_deadline`, pump.rs:150) reaches no log or pub read; the tests
  assert F-JOIN-ONCE (4) by READY (every client holds JoinAccepted, which a session closed at the Join deadline never
  gets) and the room's `deadline_closed_count` 0. Likewise "no kind-10 refusal" (C-U6-17's form) cannot be read: the
  room's log carries no refusal counts yet (U6d/U6e's writers); the tests assert each client gave commands each second.
- F-LG-SHA: npt-bots has no hashing crate, so LG's header `build.binary_sha256` is empty (U8 records LG's binary hash).
- F-TERM: npt-load has no signal handler (no tokio `signal` feature, no libc, unsafe forbidden), so TERM ends it by the
  default action with no footer; its log is written through a LineWriter, so it is cut at a line end (U8's helper keeps
  it as is, the judge reads it as cut short).
- A2's commands (this container, the runner, release; FP-5: each by its full name, the earlier `a2_ … --exact` form
  selected no test), each after `cargo build --locked --release -p npt-server` (the test runs the npt-server binary
  beside npt-load's):
  `cargo test --locked --release -p npt-bots --test load a2_48_websocket_clients_at_p2_against_one_room -- --ignored --nocapture --exact`
  `cargo test --locked --release -p npt-bots --test load a2_48_webtransport_clients_at_p2_against_one_room -- --ignored --nocapture --exact`
  48 clients at P2 on the transport, the split's mark 10 s after OPEN, NPT_A2_SECS (default 60); prints the room's
  ws_split (WebSocket only: pooled E both ways, ws_split_mark_room_ns: F-3), its max rss_bytes (the cap's memory at
  N = 48) and LG's inputs a client a second. Not run by me.
- F-FMT-1: rustfmt's diff over Blocks 1 and 3 is +318/-57 (42 hunks, .gate/u6c/b3-fmt.log); applied by Edit at the end,
  counted in the size now.

## Done
- Block 1 DONE (08:23 UTC): new pub items in npt-net: `start_poller` (+`Polled`), `FateEntry`, `Samples`, `samples`,
  `nearest_rank`, `quantile`, `excess`, `Forms`, `forms`, `ImpairSocket::pretouch_fate_log`, `ProbeDatagram`, `Pacer`,
  `send_down`, `probe_peer` (+`ProbePeer`), `leg_series` (+`LegSeries`). Tests +10 (probe_tests.rs), all pass
  (.gate/u6c/b1-test1.log: 15 passed with T1, T2, T5, T6, T7 on the shipped code); clippy both builds clean
  (.gate/u6c/b1-clippy.log). Added lines: 1,267.
- Block 3 DONE (08:36 UTC): npt-server `run_probe`, `ProbeSettings`, `leg_log`, `PROBE_LEGS`, `PEER_WAIT_NS` (lib);
  main.rs `mode`/probe fields and `probe_mode`; npt-net `L2Config::b1` (npt-server names no npt-l1 type). Tests +8
  (probe_tests.rs 6, main.rs 1, tests/binary.rs 1), all pass (.gate/u6c/b3-test2.log; the loopback probe 4.35 s);
  clippy both builds clean (.gate/u6c/b3-clippy.log). SERIES.md: one new section (28 rows). Added lines: Block 3
  1,120 (probe.rs 598, probe_tests.rs 247, main.rs 186, binary.rs 41, SERIES.md 44, config.rs 7, lib.rs 3; the
  poller-state change in Block 1's files about -6). Running total 2,387 + rustfmt's +261 = 2,648.
- Block 4 DONE (08:47 UTC): npt-bots `Hosted`, `HostState`, `LoadSettings`, `run_load`, `now_ns` (the one clock item,
  reviewed allow); npt-load's binary (one argument). Tests +10 (load_tests.rs 7, tests/load.rs 3) and A2's ignored
  test, all pass (.gate/u6c/b4-test1.log, b4-test3.log: the integration binary 5.15 s); clippy both builds clean
  (.gate/u6c/b4-clippy.log). Added lines 1,019 (load.rs 593, load_tests.rs 195, tests/load.rs 212, lib.rs 4, the bin
  +15) and rustfmt's +308 (+389/-81, .gate/u6c/b4-fmt.log). Running total 3,975.
- Blocks 5 and 6 DONE (08:55 UTC): npt-host `serve` (serve.rs) and `issue` (issue.rs), main.rs the dispatcher (a
  binary only). Tests: Block 5 +11 (serve_tests.rs), Block 6 +13 (issue_tests.rs 12, tests/args.rs 1), all pass
  (.gate/u6c/b56-test3.log: 24); clippy both builds clean. Added lines: Block 5 549 (serve.rs 261, serve_tests.rs 269,
  main.rs part), Block 6 607 (issue.rs 247, issue_tests.rs 343, args.rs 22), rustfmt +298.
- SIZE after Block 6 (exact, 08:56 UTC): tracked +395 (git diff --numstat 8c81952), new files 4,175 lines, so 4,570
  before formatting; rustfmt's diff over npt-net, npt-server, npt-bots, npt-host is +1,100/-244 (.gate/u6c/all-fmt.log),
  so 5,426 after it. PAST 5,400: per C4 and launch item 6, Block 2 (P2, C-U6-8) is NOT built; the main session's lever
  (1) applies (P2 rides in MOVED's brief). Below the 6,000 stop. The end checks run on the tree as it stands.
- Tests in all: +52 new (Block 1 10, Block 3 8, Block 4 10, Block 5 11, Block 6 13), A2's one ignored.
- rustfmt DONE (09:06 UTC): the whole diff of .gate/u6c/all-fmt.log applied by Edit (bots, host, net, server);
  `cargo fmt --check -p npt-net -p npt-server -p npt-bots -p npt-host` exit 0 (.gate/u6c/fmt2.log).
- End check started 09:07 UTC: `node check.mjs` through the runner `-x` (.gate/u6c/end-check.log).
- SIZE, final (formatted tree): added 5,426 (tracked +453/-199 by `git diff --numstat 8c81952`, new files 4,973 by
  `wc -l`). Per block: Block 1 1,326 (probe.rs 790, probe_tests.rs 463, socket.rs 26, socket_tests.rs 35/-189, l2.rs 7,
  lib.rs 5); Block 3 1,311 (probe.rs 649, probe_tests.rs 330, main.rs 229, binary.rs 49, SERIES.md 44, config.rs 7,
  lib.rs 3); Block 4 1,329 (load.rs 724, load_tests.rs 246, tests/load.rs 338, lib.rs 4, npt-load.rs 17/-4); Block 5
  673 (serve.rs 303, serve_tests.rs 343, main.rs 27/-6); Block 6 787 (issue.rs 332, issue_tests.rs 428, args.rs 27);
  Block 2 0 (not built). No Cargo.toml, Cargo.lock, marks.json or judge file in the diff.
- Plants' files before any plant (sha256): net probe.rs 00b3dca6…244654, socket.rs 0046eae2…7243cc, net
  probe_tests.rs 03192a00…929ff27, server probe.rs 5a025434…5724d153, bots load.rs b2aa97ae…6b04, host serve.rs
  a6f3e311…caac7, issue.rs 5cbcbebf…96ac81 (each restore checked against these by sha256sum).
- END CHECK (.gate/u6c/end-check.log, 09:07:04-09:29:33 UTC, runner -x, free 8,114 MiB at start, tree diff hash
  80fb60a7…): fmt PASS 1.1 s; clippy PASS 27.3 s; clippy-wasm32 PASS 7.4 s; doc PASS 38.0 s; **test PASS 1,171.9 s**
  (1.08 times base's 1,086.7 s; measured **859 = 807 + 52**, bites **956 = 904 + 52**, 0 failed, 0 skipped); deps PASS;
  wasm PASS 89.9 s (27,000 ticks, 0 mismatching, both bites caught); refs PASS; marks PASS (81); self PASS (78/78);
  **redact FAIL**: 1 finding, crates/host/src/issue_tests.rs:266 "private key" = the PEM header literal in the
  six-lines test's key assert (no key: a header string).
- F-REDACT: fixed in the tree, NOT re-run: the assert now parses the key file as PKCS#8
  (`PrivateKeyDer::from_pem_slice`, `Ok(PrivateKeyDer::Pkcs8(_))`, one import added), stronger than the header check;
  Grep of the spike for redact-check's regex (ops/operator/redact-check.mjs:43): no match. Not compiled or run.
- **STOP (disk floor), 09:30 UTC:** the confirmation call (.gate/u6c/redact-fix.log: fmt, npt-host tests, `check.mjs
  --only redact`) refused, runner exit 3, "STOP: under 4 GB free" (df: 2.8 G free; shared target debug 7.7 G, release
  510 M, wasm32 187 M, x86_64 605 M). Per U6a's precedent (u6a-build.md:139-146) nothing cleaned or deleted; the main
  session's pack or clean (its alone). Not done: the redact fix's run, the plants P1-P21, the final full check.mjs.
- RESUME (after the clean): (1) runner -x: `cargo fmt --check -p npt-host` :: `cargo test --locked -p npt-host`;
  (2) the plants below, each: Edit (plant), one runner `-x -k` call with the named test, Edit (restore), sha256sum
  against the hashes above (issue_tests.rs is not planted); (3) the full `node check.mjs` once on the restored tree.
  Test commands: `cargo test --locked -p <crate> --lib <name>` (npt-host: `--bin npt-host <name>`).
  - P1 net probe.rs start_poller: `rt.spawn(async move {` → `let h = rt.clone(); rt.spawn_blocking(move ||
    h.block_on(async move {`, its close `}))`; T1 `the_task_poller_takes_lateness_where_the_task_runs` (npt-net).
  - P2 net probe_tests.rs never-mix: the drop and rebind of `a` replaced by `a.set_default(&fixed(1, 64)); let next =
    a;`; `u6c_two_legs_on_two_sockets_never_mix`. A test-body plant: the shipped path binds per leg (server
    probe.rs:456), so no shipped-code plant reaches this test.
  - P3 socket.rs pretouch_fate_log: `while false && k < cap …`; `u6c_the_pretouch_fills_and_empties…`.
  - P4 leg_series: warm-up fates (t_in < start) pushed into `set[d]` before the window filter; P5 e_fields slot 1
    `f.at_zero` → `f.at_p50`; P6 `in_e_set` with 7 (Oversize): all `u6c_c_u6_5_the_window_takes_exactly_its_fates…`.
  - P7 leg_series RTT `checked_sub(hold)` → `checked_sub(hold * 0)`; `u6c_leg_series_files_loss_delay_burst…`.
  - P8 server run_leg: `after_edge` read moved after `drop(sock)`; P10 run_probe `base = quantile(..)` → `let _ =
    quantile(..)`; P18 `Intake::take` with `ours &&` dropped (the pacer starts on any source's first datagram, before
    the peer is known): all `u6c_one_probe_on_loopback_p0_then_p1_with_the_peer_pinned` (npt-server). P8 may not
    bite if ImpairSocket's unjoined threads keep the kernel socket a moment (socket.rs:440-446): record as found.
  - P9 `udp_drops` `.or(Some(0))`; `u6c_the_udp_row_is_found_by_its_local_port…`.
  - P11 `inputs_record` sort removed; `u6c_inputs_count_lists_the_playing_clients_in_slot_order` (npt-bots).
  - P12 `per_request` cached at its first read (a static); `u6c_a_replaced_hash_file_is_served_anew`. P13 `answer`
    `if no_store && false`; `u6c_the_hash_files_are_served_verbatim_with_no_store_per_room`. P14 `respond`
    `.find(|f| path.starts_with(f.0))`; `u6c_targets_outside_the_list_are_404`. P20 `let path = target;`;
    `u6c_a_query_is_ignored`.
  - P15 `Some(_) => NewOrder::new(&ids)`; `u6c_the_production_order_asks_for_shortlived`. P16 a second `poll_ready`
    on a non-Ready status; `u6c_an_invalid_order_exits_4_after_one_poll_and_writes_nothing`. P17 the key `0o644`;
    P21 the PROFILES line to `out`: both `u6c_staging_then_production_in_one_directory…`.
  - P19 probe_peer's `.take()` → a copy of the newest; `u6c_probe_peer_echoes_the_newest_down_once_with_its_hold`.
  - TE2-TE4: Block 2's, not built.
- Size with the fix: 5,428 added (issue_tests.rs 430).
- RESUMED 09:35 UTC (the coordinator: disk freed, 10,559 MiB free). F-REDACT DONE: redact-fix2.log: rustfmt wanted the
  new assert split (applied by Edit); redact-fix3.log (09:36 UTC): `cargo fmt --check -p npt-host` exit 0; `cargo test
  --locked -p npt-host` 23 + 1 passed, 0 failed (the six-lines test with the PKCS#8 parse among them); `check.mjs --only
  redact` PASS, 371 items, 0 findings. Size now 5,431 (issue_tests.rs 433).
- PLANTS (each one runner -x -k call, .gate/u6c/plant-pN.log; restore checked by sha256sum against the hashes above):
  - P1 FAILS T1 `the_task_poller_takes_lateness_where_the_task_runs` (socket_tests.rs:917: lateness 220,047 ns, not
    >= 50 ms); restored, probe.rs 00b3dca6… equal.
  - P2 FAILS `u6c_two_legs_on_two_sockets_never_mix` (probe_tests.rs:150 "a new leg starts empty", 3 not 0);
    restored, probe_tests.rs 03192a00… equal.
  - P3 FAILS `u6c_the_pretouch_fills_and_empties…` (probe_tests.rs:171, 0 not 1,000); restored, socket.rs 0046eae2…
    equal.
  - P4 FAILS `u6c_c_u6_5_the_window_takes_exactly_its_fates_into_both_e_forms` (:210 n 1,001 not 1,000); P5 FAILS it
    (:211 the judged form 10,000 not 10,100); P4's restore and P5's checked by one hash after both (P5 was planted
    before P4's hash was taken; disjoint lines): probe.rs 00b3dca6… equal.
  - P6 FAILS the same test (:210 n 1,001: the Oversize fate in E); restored, equal.
  - P7 FAILS `u6c_leg_series_files_loss_delay_burst…` (:285 RTT 23 ms not 20 ms); restored, equal.
  - **P8 does NOT fail** `u6c_one_probe_on_loopback…` (plant-p8.log: 1 passed, 4.46 s): read after `drop(sock)`, the
    socket's /proc/net/udp row is still there, since ImpairSocket's Drop signals its threads without joining them
    (socket.rs:440-446) and they keep the kernel socket a moment. F-P8: the test cannot tell a read before the drop from
    one just after; a bite needs an observable the drop removes at once (a joined Drop is U6b's code). Restored, server
    probe.rs 5a025434… equal. No retry, no test change.
  - P9 FAILS `u6c_the_udp_row_is_found…` (probe_tests.rs:19, Some(0) not None); restored, equal.
  - P10 FAILS `u6c_one_probe_on_loopback…` (:308 base_rtt_ns 0 not 206,575); restored, equal.
  - P11 FAILS `u6c_inputs_count_lists…` (load_tests.rs:109, [30, 29, 31]); restored, load.rs b2aa97ae… equal.
  - P12 FAILS `u6c_a_replaced_hash_file_is_served_anew` (serve_tests.rs:297, the old bytes); restored, serve.rs
    a6f3e311… equal.
  - P13 FAILS `u6c_the_hash_files_are_served_verbatim…` (:267, Cache-Control None); restored, equal.
  - P14: the first plant (`.as_str()`) did not compile (plant-p14.log, E0658), so no test ran; corrected to
    `path.starts_with(f.0)` and run once: FAILS `u6c_targets_outside_the_list_are_404` (:222, `/../Cargo.toml` 200
    not 404; plant-p14b.log); restored, equal.
  - P15 FAILS `u6c_the_production_order_asks_for_shortlived` (issue_tests.rs:295, Some(None)); restored, issue.rs
    5cbcbebf… equal.
  - P16 FAILS `u6c_an_invalid_order_exits_4…` (:330, refreshes 2 not 1); restored, equal.
  - P17 FAILS `u6c_staging_then_production…` (:272, mode 0o644 not 0o640); restored, equal.
  - P18 FAILS `u6c_one_probe_on_loopback…` (:226 `ran.expect`: "leg P0: no round trip in its window's first 10 s":
    the stranger taken as the peer, the down pacer pointed at it); restored, equal.
  - P19 FAILS `u6c_probe_peer_echoes_the_newest_down_once_with_its_hold` (:437 "no new down: no echo", Some(7));
    restored, net probe.rs equal.
  - P20 FAILS `u6c_a_query_is_ignored` (serve_tests.rs:237, 404 not 200); restored, equal.
  - P21 FAILS `u6c_staging_then_production…` (issue_tests.rs:247, 7 lines on stdout); restored, equal.
  - All seven planted files equal their pre-plant hashes after the last restore (one sha256sum at 09:46 UTC).
  - In all: 20 of 21 plants fail their named test; P8 does not (F-P8). TE2-TE4 go with Block 2: not built.
- FINAL END CHECK DONE (.gate/u6c/end-check2.log, 09:46:09-10:07:45 UTC, runner -x, free 9,234 MiB at start, 3.8 G
  after): all 11 steps PASS: fmt 1.1 s, clippy 37.4 s, clippy-wasm32 7.5 s, doc 37.1 s, **test 1,116.8 s** (1.03 times
  base's 1,086.7 s; measured **859 = 807 + 52**, bites **956 = 904 + 52**, 0 failed, 0 skipped), deps 2.6 s, wasm 82.6 s
  (27,000 ticks, 0 mismatching, both bites caught), refs, marks (81), self (78/78), redact (371 items, 0 findings).
  Tracked diff hash 80fb60a7… as in end-check.log (no tracked file changed since); the untracked hash differs by
  issue_tests.rs's fix and these notes.
- Size, final: 5,431 added, 199 removed (Block 6 now 792 with the fix).

## Fix pass (advisor-u6c-r1.md, REVISE; FP-1 to FP-6 in order)
- Start: ruling sha256 ad72c0b8…faed179 equal; snapshot wip/u6c-build.patch a4e63455…385d2f equal, and
  `git apply -R --check` of it on this tree is clean (24 files, the same `git status`): the tree is the snapshot.
- FP-1 DONE (.gate/u6c/fp1-test.log, 13:10 UTC: npt-net u6c_ 11 passed, npt-server u6c_ 6 passed). `leg_series(fates,
  echoes, zero_ns, start_ns, packets, interval_ns, bursts)`: the peer found first (the first Delivered up fate whose
  record is not OTHER), then per direction the candidates (t_in >= start, up the peer's) sorted by t_in (stable: ties in
  log order), kept until `packets` lie outside the bursts plus every fate at the last kept t_in; the rows from the
  start's second to the last record's; the main loop unchanged in log order over the kept fates (RTT join as before).
  New summary fields `probe_up_window_ns`, `probe_down_window_ns` (last kept t_in − start + interval, 0 with none);
  `probe_window_ns` (the nominal window, total × interval) now written by `run_leg`. `run_leg` passes `zero + warm`,
  `s.packets`, `interval`; the Measured phase runs to the last record's second (else records past the nominal end would
  sit in an After second, which the judge's measured window skips). SERIES.md: the section's text, `probe_window_ns`'s
  and the window counts' meaning, one new row for the two spans. Test `u6c_fp1_each_direction_keeps_exactly_packets_
  records_whatever_the_clocks` (N 120,000 so 5 ppm reaches 60 µs); the C-U6-5 and leg_series tests take the new call
  (packets 1,002 and 4: the same fates kept); the e2e bound `>= s.packets`.
- FP-2 built: `edge_events(_bound: &ImpairSocket, port, ph)`, both calls pass `&sock` (server probe.rs), the doc
  says why; probe_tests.rs's doc no longer calls the e2e test "P8's test". P8c runs at the end.
- FP-3 built: the e2e test asserts P1's `l2_down_delivered_count` <= its `probe_down_sent_count`. P2's shipped-code
  plant runs at the end.
- FP-4 built: (b) tests/load.rs `probe_peer_sent_count >= 5_000`; (c) the echo test reads until 9's echo or 1 s,
  asserts [9] or [8, 9], then ten reads with no echo. P19 runs at the end.
- FP-5 built: `a2_48_webtransport_clients_at_p2_against_one_room` beside the WebSocket one, both calling `a2(transport)`
  (the room's `transport` and LG's from it; the split's mark kept, so WebTransport's run has no `ws_split`: printed
  None); the doc and these notes give both commands by full name. Proof (.gate/u6c/fp5-list.log, 13:15 UTC): each
  `--list` call prints "<name>: test" and "1 test, 0 benchmarks".
- FP-2 to FP-5 run (.gate/u6c/fp2-5-test.log, 13:12-13:15 UTC): npt-net u6c_ 11 passed; `cargo test -p npt-server`
  all binaries pass (lib 32, binary.rs 5 in 84 s, the rest); `cargo test -p npt-bots` lib 30 passed, tests/load.rs 3
  passed and 2 ignored (the two A2s), 0 failed.
- SIZE after FP-5 (13:16 UTC): 5,559 added (tracked +457, new files 5,102), +128 on 5,431; under 5,850.
- FP-6 built (not yet run):
  - SEC-U6C-1: serve.rs's accept error sleeps 100 ms (tokio's timer) before the next accept.
  - SEC-U6C-3: serve.rs's NN takes `n.bytes().all(|b| b.is_ascii_digit())`; `/npt/cert-hash-+5.json` in the 404 list,
    with a `room-+5.json` written first so the entry bites; load.rs's `room` check takes the same (load_tests.rs case
    "room sign").
  - SEC-U6C-4: `Intake::take` pins only when `ProbeDatagram::decode(b, None)` is Some; the e2e test's stranger now sends
    probe datagrams, so P18 (the source check dropped) still has a stranger to pin.
  - NC-U6C-3: `probe_down_unindexed_count` in `leg_series`' summary, its SERIES.md row.
  - NC-U6C-5: the "no READY" line carries `handshake_failed_count=<n>` (HostState::failed()). NOT built: "in LG's
    footer": npt-log's Footer holds only `records` and `ended` (log/src/lib.rs:344-348), so a count there is a schema
    change; a question for the coordinator (Q-FP6-1 below).
  - NC-U6C-6: `w.finish(Ended::Stopped)` before the probe peer's and a client's connect-error returns (load.rs).
  - NC-U6C-8: SERIES.md rows for `probe_peer`'s six counts and `header.params.room`, `header.params.mode`; tests/load.rs'
    probe test checks every `probe_peer` field and header param of LG's log against SERIES.md.
  - F-NO-PEER: `u6c_a_leg_with_no_peer_ends_non_zero_and_writes_no_log` (server probe_tests.rs).
  - F-NAMES: the steal check needs seconds; hosts_four checks every whole measured second (from the second one; the
    WebSocket run's last, the close's part second, left out) and the last second's index (WT: duration − 1; WS: before
    the duration).
  - F-5S: `hosts_four("wt", 4)`. F-JOIN-4 and F-STALE-BIN: doc wording (hosts_four's doc and the accept assert; room()).
- Q-FP6-1 (design, to the coordinator): NC-U6C-5's "in LG's footer". Options: (a) an event before every footer of
  LG's log (a new name and SERIES.md row, e.g. `load_end` with `handshake_failed_count`); (b) the failing line only, as
  built. Lean (a) if the count must reach the log; else (b) stands.
- FP-6 run (.gate/u6c/fp6-test.log, 13:18-13:21 UTC): npt-net u6c_ 11 passed; npt-server lib 33 (+1, F-NO-PEER),
  binary.rs 5, all others pass; npt-bots lib 30, tests/load.rs 3 passed (4.17 s) and 2 ignored; npt-host 23 + 1; 0
  failed. rustfmt's diff (same log, 7 hunks) applied by Edit; .gate/u6c/fp6-fmt-clippy.log (13:22-13:23 UTC): `cargo
  fmt --check` exit 0, `check.mjs --only clippy,doc` both PASS.
- SIZE after FP-6 and rustfmt (13:24 UTC): 5,693 added (tracked +460, new files 5,233), +262 on 5,431 (FP-1 to FP-5
  +128, FP-6 and rustfmt +134); under 5,850, so no rider carried.
- PLANTS of the fix pass (pre-plant sha256: server probe.rs 2cf464a6…9f9c3, net probe.rs c3446e7c…12d45; each
  restore checked equal, and both at the end, 13:27 UTC):
  - P22 (FP-1; the clock span back: break at t_in >= start + packets × interval): FAILS
    `u6c_fp1_each_direction_keeps_exactly_packets_records_whatever_the_clocks` (probe_tests.rs:331, (119,999, 119,999)
    not (120,000, 120,000); plant-p22.log). Restored, equal.
  - P19 (copy, not take): FAILS `u6c_probe_peer_echoes_the_newest_down_once_with_its_hold` on the new form (:506, "no
    new down: no echo", Some(7); plant-p19b.log). Restored, equal.
  - P8: P8 cannot fail a test (plant-p8.log: 1 passed); after FP-2 the compiler holds it (plant-p8c.log, E0382: "value
    borrowed here after move", probe.rs:571). Restored, equal.
  - P18 (the source check dropped; SEC-U6C-4's decode kept): FAILS `u6c_one_probe_on_loopback…` (:274 `ran.expect`,
    "leg P0: no round trip in its window's first 10 s": the stranger's probe datagram pinned; plant-p18b.log).
    Restored, equal.
  - P2 in shipped code (one socket bound at the first leg, kept for every later leg in a static, `set_default` per leg,
    in place of the per-leg bind): FAILS `u6c_one_probe_on_loopback…`, but at :358, `base_rtt_ns` 0 not 525,313 (the
    kept socket's header is P0's), before FP-3's new assert is reached (plant-p2b.log). F-P2-HEADER: the e2e test
    already caught a carried socket by its header; FP-3's assert is a second guard, which this plant did not reach.
    Restored, equal. The test-body P2 (plant-p2.log) stands as the unit half.
- FIX-PASS CHECK DONE (.gate/u6c/fix-check.log, 13:27:21-13:50:32 UTC, runner -x, free 7,735 MiB at start, 2.7 G
  after, so the next runner call would stop at the floor): all 11 steps PASS: fmt, clippy 25.8 s, clippy-wasm32, doc
  44.7 s, **test 1,176.3 s** (1.08 times base's 1,086.7 s; measured **861 = 807 + 54**, bites **958 = 904 + 54**: the
  pass's +2 are FP-1's test and F-NO-PEER; the two A2s ignored), deps, wasm 125.3 s (27,000 ticks, 0 mismatching, both
  bites caught), refs, marks (81), self (78/78), redact (371 items, 0 findings). Tree diff hash 0e3199ae…, untracked
  22bda910… (the plants restored before it; hashes checked at 13:27).
- Open after the pass: Q-FP6-1 (NC-U6C-5's footer half, above); F-P2-HEADER (P2 in shipped code fails at the header's
  base check, not at FP-3's assert); the container readings (F-READINGS) are the main session's; the disk at 2.7 G.
- Resume from: nothing left in the fix pass; the return.
