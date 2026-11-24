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
- A2's command (this container, the runner, release): `cargo test --locked --release -p npt-bots --test load a2_ --
  --ignored --nocapture --exact` after `cargo build --locked --release -p npt-server` (the test runs the npt-server
  binary beside npt-load's): 48 WebSocket clients at P2, the split's mark 10 s after OPEN, NPT_A2_SECS (default 60);
  prints the room's ws_split (pooled E both ways, ws_split_mark_room_ns: F-3), its max rss_bytes (the cap's memory at
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
