# U6 facts for the brief (gathered 2026-11-21, read-only; main folder, branch claude/netcode-proof-test-plan-hcky5g)

All cites are relative to /home/user/Shamble-Frontlines unless they start with `crates/` (= spikes/netcode-proof/crates/).
`row:N` = line N of the split copy of plan.md:1253 (scratchpad u6-row.txt).

## 1. U6's crates and parts: where each is defined

The unit row: plan.md:1253 (size M; row:1). Layout §4.1 (plan.md:366-417); dependency rules §4.2 (:421-455); each
crate's whole public surface §4.7 (:569-633). `crates/server/` and `crates/net/` fall under `.claude/rules/netcode.md`
(plan.md:418-419).
- **npt-server** (library + thin binary): plan.md:381-383 (§4.1); public items `due_ticks`, `Schedule`, `TickLoop`,
  `PhaseTimers`, `Counters`, `ControlSocket`, `ConnLimits`, `AddrTable`, `Decoder` (plan.md:600-601); E2 the room
  process (tick thread alone on one vCPU, bounded channels, `panic = "unwind"`, top-level catch to RoomEnded(crash),
  the S17 loop) plan.md:461; ADR 0004 part 5 (docs/adr/0004-simulation-core.md:207) and ADR 0002's `server` row
  (docs/adr/0002-code-structure-build-toolchain.md:192). Decoding on the I/O runtime, the room/server check split and
  what the room gets per tick: §4.4 plan.md:507-526 (the check table :515, wall-time column = npt-server's; loss and
  send-buffer space "join at U6", :517-521).
- **npt-net** (WtServer, WtClient, WsServer, WsClient): plan.md:384-386; public items plan.md:602-608; E6 transports
  (per-address table bounded and tested at 100,000 addresses, `wss` only, `params.cc`, refused handshakes counted per
  reason) plan.md:465; E15 L2 and fix 1's three conditions plan.md:474; ADR 0001 part 10
  (docs/adr/0001-multiplayer-netcode.md:419).
- **npt-super** (supervisor stand-in, process control, exit log, SIGKILL): plan.md:395; E16 density harness (8 rooms,
  bracket B, SIGKILL, 384 sessions) plan.md:475; depends on no sim/room/proto (plan.md:429, ADR 0002:193).
- **npt-host** (page host on TCP 31000 and the ACME client): plan.md:396; E21 plan.md:480; §10.4 allow-list, headers,
  limits, 404 bite, Origins plan.md:1116-1135; depends on none of sim/room/proto/client (plan.md:429-430).
- **WebTransport dev certificate** (ECDSA P-256, at most 14 days, made at start by room server or supervisor; hash
  published by npt-host in `/npt/cert-hash.json`): plan.md:991-996 (§8.5); ADR 0006:302-305.
- **`npt-cert issue` / `revoke`** (root-owned oneshots, open TCP 80 and npt-host's ACME egress for the order only):
  plan.md:960-963 (§8.4); sudo list plan.md:1137-1140 (§10.5).
- **instant-acme** 0.8.5, default features off plus ring and hyper-rustls; requires LE's 6-day `shortlived` profile,
  fails closed via `NewOrder::profile`; "U6 confirms its revocation call"; a 90-day fallback needs security-auditor's
  and the advisor's OK at U6 (C11): plan.md:964-969. Staging profile lists read at R1: plan.md:970-971. Revocation at
  teardown: plan.md:982.
- **`npt-ctl peers <validated IPv4s>`** (fills nftables' `metrics_src`, writes `PermitOpen`, `sshd -t`, reload):
  plan.md:1162-1165 (§10.5). Note: the U6 row names "the `npt-ctl` and `npt-cert` wrappers" (row:1).
- **Per-slot control sockets** `/run/npt/ctl/NN`, dir 0750 `root:npt-ctl`, socket 0660, `SO_PEERCRED` uid check,
  same size limits and panic-free parsing as network messages, what they cover: plan.md:1066-1077 (§10.2); E23
  plan.md:482. `ControlSocket` is the public item (plan.md:600). Room-wide controls `RoomControl` already in npt-proto
  (plan.md:594-595).
- **systemd template units and capabilities** (`User=npt-room-NN` or DynamicUser, `NoNewPrivileges`,
  `ProtectSystem=strict`, empty `CapabilityBoundingSet`, `MemoryMax`; supervisor unprivileged via `npt`'s sudo
  wrappers; slot dirs 0700): plan.md:1078-1082; users plan.md:1137-1146; B12's interim ADR 0006:542.
- **Per-session capability** (128 random bits per browser slot, minted through the socket, constant-time compare;
  `SetProfile`/`BlockUdp` keyed on source address; `TriggerBurst` on the live session; parsing only on a joined
  rostered session; pre-Join control closes the session; bites): plan.md:1083-1097.
- **Roster and tokens** (128 bits, bound to room and slot, constant-time, expire at run end; version and content hash
  checked; joins beyond N refused; bot token reusable, a Join on a live slot replaces it with `Kick(replaced)`; browser
  token in URL fragment + resume secret; runner single-use token; strays counted, a run admitting one is INVALID):
  plan.md:1100-1114 (§10.3).
- **Interest under load:** E5 plan.md:464 (marks -8, -9, -10, -12); room-side code exists (section 2). The U6 row
  names it only (row:1).
- **Instrumentation:** E18 (monotonic reads at tick/phase edges, clock resolution, per-thread CPU and RSS per second,
  queue lengths, bytes at UDP level payload+28 B, packet capture, steal and affinity) plan.md:477.
- **Metrics and logs:** E22 (`metrics` + `metrics-exporter-prometheus` bound to RS's public IPv4, scraped from LG;
  bounded labels; `tracing` JSON lines, no client string verbatim; PM-0006-21 canaries) plan.md:481; ADR 0006 part 9
  (docs/adr/0006-infrastructure-operations.md:398).
- **WsServer's send path with fix 1** (U3c's carries, the advisor's C14): row:1-9; fix 1 "Wired: U6 puts it in
  WsServer's send path, with npt-tcpinfo's kernel values on the plain path" plan.md:217; §4.4's budget rule
  plan.md:522-524; the reader stamp row:22; U3c's review-round carries row:10-21.
- **U3d's probe carries C-U6-1..8:** row:23-31 (source: u3d-plan.md §3.6, advisor-u3d.md).
- **Judge-side carries C-U6-9..11:** row:32-34 (the advisor's U4j ruling).
- **U4e's carries C-U6-12..16:** row:35-40. **U4f's C-U6-17, -18:** row:41-43. **U4f-F1's C-U6-19:** row:44-45.
- **-35's counted run:** row:46; register entries 110-111 (u4s-carries.md:817-824).
- **Register entries outside "## U6" that name U6** (u4s-carries.md): 60 TUNE-1's header, "U6's npt-server header
  does the same" (:285-290; also row:9); 62 (d) a later-admitted client is -19's join, C-U6-11 (:296-301); 64 judge
  follow-ups "at the latest before U6 builds -19's figure" (:455-457); 75 "The wire field is U6's (C-U6-13)"
  (:359-367); 76 A6/SEC-U4E-5 ending a session, U4r's lab stopgap (:368-375; U6's half is C-U6-14); 88 "-30 stays
  NOT-RUN until U6" (:678-685); 96 `pump_report`'s RTT, "U6's TickLoop makes the same hand-off" (:628-632); 97 npt-sim
  pub items, "U6 at the latest" (:633-637); 103 (2)-(3) -35 (:770-777); 104 (4) the counting allocator as a normal
  dependency "(U6's PM-0006-20) is U6's change with its own read" (:785-790); C-U6-19 one-liners in U4f2/U4g/U4h/U5
  sections (:557, :603-607, :609-611, :759-761).
- **Unit order context:** U7 browser harness (plan.md:1254), U8 run package and R1 plan, needs U6's reader stamp
  (plan.md:1255). Stop B before any VM reviews the control plane, page host and sudo wrappers (plan.md:1267-1270).

## 2. What exists now, per crate

Read in the main folder on 2026-11-21 (wc -l). Workspace members include server, net, super, host, tcpinfo
(spikes/netcode-proof/Cargo.toml:6-24).
- **npt-server: exists, library partial, binary a stub.** src/lib.rs 17 (exports `ConnLimits, Counters, Decoder,
  RttSource` and `Dropped, Due, LoopCounters, Schedule, due_ticks`, `BITES` under `bites`: lib.rs:14-17; doc "Built in
  U4 and U6", :5); src/main.rs 6 (prints "not built yet (unit U6)", exits 2: main.rs:1-6); src/pump.rs 519 (U4e's I/O
  step: `ConnLimits` :71, `Counters` :112, `RttSource` :183, `Decoder` :204 with `new` :253, `accept` :318, `reject`
  :325, `take_sent_bytes` :332, `rtt_ns` :339, `pump` :372); src/schedule.rs 269 (U4b: `Schedule` :22, `due_ticks`
  :149, `BITES` :252). Tests: tests/pump.rs 548, schedule.rs 428, loopback.rs 338 (U4e's Room + pump + real client on
  an injected clock, loopback.rs:1-2), gap.rs 263, common/mod.rs 403 (the test host `Lab`; `Decoder::new` at :206,
  :243; its own `stream_of` :47). Cargo.toml: deps link, proto, room, sim, net, log; dev-dep client; `bites` feature.
  **Not present** (grep over crates/*.rs): `TickLoop`, `PhaseTimers`, `ControlSocket`, `AddrTable`, any tokio runtime,
  tick thread, panic catch, metrics; u4-build-plan.md:963-967 lists these as U6's ("the tokio runtime, `TickLoop`'s
  thread, `PhaseTimers`, `ControlSocket`'s socket, the metrics, the panic catch, npt-server's binary").
- **Where Room + pump are hosted today:** on L1 by npt-lab (crates/lab/src/scenario.rs 4,276 lines; `Decoder::new`
  at :250 and :530; its own `stream_of` :158, the copy C-U6-16 (c) bans); in npt-server's tests by common/mod.rs.
  U4r's lab also carries the stopgaps U6 replaces: event starts via `ClientEndpoint::with_starts` (client/src/lib.rs:369)
  and `Room::with_event_start` (room/src/room.rs:665) (register 75), and the replacement/kick handling (register 76).
- **npt-room (U4e, exists):** room.rs 1,825, roster.rs 188, interest.rs 245, input.rs 636, encoder.rs 350, ack.rs 299,
  events.rs 150, rewind.rs 33, lib.rs 56. Calls U6 drives: `Room::join` -> `Result<(u8, bool), JoinRejected>` (bool =
  replaced) room.rs:814; `leave` :867; `receive(slot, &Message)` :890 (no generation); `tick` :1028;
  `take_datagrams` :772; `take_control(slot) -> Vec<Vec<u8>>` :784 (no channel beside the bytes); `pump_report(slot,
  sent_bytes, max_datagram)` :877 (no RTT: register 96); `reports` :762. The Resync limit's clock is in `ClientSlot`
  (`last_full_state: None` rebuilt by every Join, room.rs:838; check at :905-909, counter `resyncs_limited` :130).
  `run_presses` lives in the room (room.rs:510, :584, :1088, :1105) and is not on the wire. Kind 5's first command is
  unchecked (`if let Some(prev) = &self.accepted`, input.rs:287). Roster: per-slot token + bind + live, constant-time
  `same` (roster.rs:7-24); "minting is U6's" (roster.rs:1-3). Interest: `InterestGrid` :13, `AreaFlood` :77,
  `Priority` :158 (interest.rs).
- **npt-proto:** `JoinAccepted` has slot, team, squad, mode_id, mode_hash, content_hash, tick, tick_hz, snapshot_hz,
  no event base and no press counters (proto/src/msg.rs:98-113). `full_state_ack_ticks_at` already exists
  (sim/src/ticks.rs:56, :88).
- **npt-net: exists (U3b, U3c, U3d), no WtServer/WsServer/clients yet.** lib.rs 28 exports only `DelayStream,
  ImpairSocket, L2Config` (lib.rs:24); doc names U6 for the rest (:1-5). Modules held `allow(dead_code)` "until U6":
  `tcp_model` (lib.rs:7-10), `ws_gate` (:12-15), `ws_config` (:17-20), `tcp_kernel` (:26-28); `wt` "WtServer joins at
  U6" (l2.rs:30-33). u4-build-plan.md:146-148, :170-171, :208 record the same U6 hand-offs (`loss` C3/Q6, no `Send`
  on `Box<dyn Link>`, AddrTable and the handshake cap with -25). Files and sizes:
  - ws_gate.rs 160 (fix 1: `snapshot_budget` :25, `may_write` :36, `Inputs` :43, `Snap` :56, `Bite` :64, `TickWriter`
    :74 with `on_tick` :116, `written/skipped/held` :140-150; `NO_SAMPLE_SRTT_NS` :20, `MIN_SNAPSHOTS` :22); tests 239.
  - tcp_kernel.rs 174 (`srtt_ns(&TcpStream)` :22, `outstanding_bytes` :28, through npt-tcpinfo).
  - ws_config.rs 83 (`MAX_WRITE_BUFFER_BYTES` :19, `provider` :22, `tls_config` :27, `acceptor` :42, `ws_config` :52,
    `one_source` :63); tests 191.
  - tcp_model.rs 3,110 + tcp_model/fixes.rs 2,150 + tests 1,460 (U3a's `linux-6.8-tcp v3`).
  - l2/delay.rs 1,917 (`DelayStream` :934, `wrap` :959, `now_ns` :1110, `add_burst(start_ns, len_ns)` :1136,
    pub(crate) `srtt_ns/outstanding_bytes/free_bytes/owed_bytes/gate_inputs` :1214-1237, `header_numbers` :1349,
    `header_texts` :1396, `take_second` :1424, `run_summary` :1466, AsyncRead :1503, AsyncWrite :1570; per-process
    `static POOL` :195; the pump's `poll` :1709 with its closing `wake_all` at :1907 (the row cites :1884, row:22);
    no set-profile method found); delay_clock.rs 273; delay_names.rs 488; ws_frame.rs 682; tls_test.rs 158 (tests);
    delay_tests.rs 3,429.
  - l2/socket.rs 741 (`ImpairSocket` :157, `bind` :176, `set_peer` :244, `set_default` :251, `now_ns` :256,
    `first_seen` :262, `release_thread_id` :268, `header_numbers` :312, `header_texts` :351, `take_second` :371,
    `run_summary` :394, `drain_fate_log` :427, quinn `AsyncUdpSocket` impl :458, `try_send` :463, `poll_recv` :494; the
    stamp sites C-U6-8 names: up's lock :501 and clock read :503, down's stamp after `try_send` :724-726); impair.rs 767;
    shaper.rs 185; stats.rs 725; thread.rs 155; names.rs 156; config.rs 162 (`L2Config` :15, `new` :95, `clean` :116,
    `with_throttle` :126, `with_bottleneck` :133); socket_tests.rs 1,694 (U3d's `start_poller` :676, the task-poller
    test :1003, T7's doc :1263-1265, `l2_timing_self_test` :1352, `agree` :904); impair_tests.rs 834.
  - l2/wt.rs 232 (each crate's server on a quinn endpoint over ImpairSocket: `Failure` :31, `Handshakes` :42,
    `PENDING_SESSIONS` 64 :60, `endpoint` :88, per-crate `provider`/`config` :108-184; `MIGRATION = false`,
    l2.rs:39-42); wt_tests.rs 595.
  - Cargo.toml: features `wt-wtransport`, `wt-wtq`; deps link, l1, rand_chacha, rand_core, quinn, tokio,
    tokio-tungstenite, npt-tcpinfo, tokio-rustls, optional wtransport (feature `quinn`) and web-transport-quinn;
    futures-util a dev-dep only (net/Cargo.toml).
- **npt-tcpinfo (U3c, exists):** lib.rs 53 (`pub use linux::{srtt_us, unacked_and_unsent}` :44, stub :50); linux.rs
  237 (`srtt_us(BorrowedFd) -> io::Result<Option<u32>>` :46, `unacked_and_unsent` :123); stub.rs 28; tests.rs 459;
  examples/siocoutq_probe.rs 669.
- **npt-link:** lib.rs 327; `Link` trait :64-88 has no `loss`, no send-buffer space, no `Send` bound; `Listener`
  :92-93; `Loopback` :132. (Plan: `loss` and send-buffer space join at U6, plan.md:577-579; "`+ Send` on L2 links",
  u4-build-plan.md:964-965.)
- **npt-super: crate exists, binary stub only** (src/main.rs 8: "not built yet (unit U6)"); Cargo.toml deps npt-log only.
- **npt-host: crate exists, binary stub only** (src/main.rs 8); Cargo.toml: npt-log only, comment "instant-acme 0.8.5
  joins at U6 (plan §8.4)".
- **npt-bots:** lib.rs 1,461 (U4r's `Behaviour` :113); bins npt-bots.rs and npt-load.rs are 6-line stubs ("not built
  yet (units U4, U6)" / "(unit U6)"). u4-build-plan.md:966-967 gives U6 "npt-bots' npt-net glue and npt-load, with the
  clock bans' reviewed allows (§4.11)"; the U6 row (row:1) does not name them (see section 7).
- **npt-log:** lib.rs 1,039; no weapon/health header field found (grep); `Scene::weapon` and `Scene::health` exist
  (sim/src/scene.rs:312, :317); U4r's lab reads them (lab/src/scenario.rs:1901).
- **npt-judge:** 20 figures are `planned(.., "U6", ..)` with `compute: None` (judge/src/figures.rs:866-966; `planned`
  :781): sync_unit_cost, sync_cost_ratio (-10); down_client_mean_max_bits, down_mean_ipv6_bits, r2_down_formula_bits
  (-12); up_client_mean_max_bits (-13); rss_slope_bytes_per_10min, queue_growth_count (-15); session_memory_growth_count
  (-24); kill_p99_ratio (-27); rtt_agreement_p99_ns (-33); bulk_control_p99_rise_ns, one_shot_dropped_count (-38);
  crash_close_max_ns (-59); egress_interface_bytes, egress_metric_vs_interface_ratio, r2_egress_formula_bytes,
  egress_vs_r2_ratio (PM-0006-17); logging_cost_ratio (PM-0006-19); alloc_twin_diff_count (PM-0006-20).
  `twin_hash_mismatch_count` takes role `twin` only, "-30's form comes with U6" (figures.rs:190-200).
- **ops/:** ops/runs holds container run scripts only (c-u2, c-u4f-*, c-u4r-*, det-u4.mjs); ops/operator holds
  redact-check.mjs and its test; no cloud-init, firewall, systemd units or sudo wrappers exist yet (ls ops/*).

## 3. Build blocks (proposed grouping, sizes, split)

Sizes are rough added lines (code + tests), scaled on comparable files read in section 2. Size rule: M about 3,000
added lines, the stop past 6,000 (SIZE-CHECK, history.md:2048-2051, as u4g-brief.md:217 quotes it).
- **T1 WtServer, both crates, on L2 loopback** (row:1; E6): Listener + Link over each crate's session (datagrams,
  control and FullState streams), handshake failures counted per reason; C-U6-12's stream cap and control-message
  limit. crates/net (l2/wt.rs, new wt server file, tests). ~400 + ~350 tests: wt.rs (232) already builds the endpoints;
  wt_tests.rs (595) is the test scale.
- **T2 WtClient + WsClient** (plan.md:602; E12 bots ride them). crates/net. ~350: thin Link adapters on existing
  configs.
- **T3 WsServer accept path** (row:16-19): TCP -> TLS (ws_config.rs `acceptor`) -> DelayStream or plain ->
  tungstenite from one `&L2Config` (S5 a); Join deadline covers TLS (S5 b); accept cap 0.31 MB per DelayStream
  connection + fixed (F7); Origin check (plan.md:1132-1135); -25's never-joining sessions and the wrapper (row:15).
  crates/net. ~400: ws_config.rs (83) and DelayStream exist; this is the glue plus its loopback tests.
- **T4 WsServer send path, fix 1** (row:1-8, :11, :21): `TickWriter` per connection, `gate_inputs()` once per tick
  (F11), owed = tungstenite buffer + rustls unsent, plain path via tcp_kernel.rs, free value stated (S5 d), the whole
  tick snapshot through Link (W18), where the per-tick syscalls run, shutdown error after peer close (C1c); tests:
  writer properties, test (c), queueing bite re-run, plain-path SIOCOUTQ check. crates/net. ~300 + ~400 tests:
  ws_gate.rs (160) holds the logic; siocoutq_probe.rs (669) shows the plain-path test's weight.
- **T5 L2 extras + reader stamp** (row:5-7, :9, :12, :20, :22): npt-net `bites` feature with ws-budget-x2 (R11);
  SetProfile on a DelayStream connection (no method exists today); B1 burst range check before `add_burst` (S7, F1);
  header fields (wrapper on, `Scene::weapon`/`health`, carry 60); pool reset or one run per process; the reader stamp
  at delay.rs's closing `wake_all` (:1907) with identity test, misattribution bite and the loopback self-test's four
  parts. crates/net (delay.rs, delay_tests.rs, Cargo.toml). ~400.
- **T6 Link gains `loss`, send-buffer space, `+ Send` on L2 links** (plan.md:577-579; u4-build-plan.md:964-966).
  crates/link, its Loopback, npt-l1's L1Net, callers. ~150.
- **S1 TickLoop on Loopback** (plan.md:600; E2 :461; register 96; C-U6-16 (c)): tokio runtime, tick thread alone,
  bounded channels, `Decoder::pump` per session on the I/O side, `due_ticks`, `pump_report` with RTT, control bytes
  sent by a channel npt-proto or the room gives (no `stream_of` copy), panic catch -> RoomEnded(crash), non-zero exit;
  `PhaseTimers`. crates/server (new tick_loop file, lib.rs, tests). ~400 + ~350 tests: pump.rs (519) and the test host
  common/mod.rs (403) are the scale.
- **S2 the session and `JoinAccepted` changes, paired** (C-U6-13, -14, -16, -19; row:37-40, :45 ":103-105, :111"):
  event base and run press counters on the wire (npt-proto msg.rs:98, catalogue), client and bots start from them,
  kind-5 first-command check (input.rs:287), the Resync limit's form outside `ClientSlot`, session generation or a test
  that no closed session's message reaches the room, `Kick(replaced)` then close, (b) the rate form, (d) the pump's
  wrong-direction case. crates/proto, room, client, bots, server, docs/design catalogue. ~400 + ~400 tests (each plant
  failing by name).
- **S3 C-U6-15 and C-U6-17/-18**: the room's cheap wins, each with the bench before and after (room.rs, encoder.rs;
  the judged bench itself is on R2); the controller's cap off L1 (client); one entry point for a Behaviour's host order
  (bots). ~400 + ~200.
- **S4 instrumentation and header** (E18 plan.md:477; carry 60; C-U6-4): per-thread CPU/RSS per second, queue
  lengths, UDP-level bytes, steal, clock resolution, header through npt-log. crates/server, log. ~400.
- **S5 metrics and logs** (E22 plan.md:481): metrics exporter on RS's IPv4, bounded labels, tracing JSON lines with
  client strings cut or hashed, canaries; PM-0006-20's allocator twin (register 104 (4)). crates/server, Cargo.toml,
  Cargo.lock. ~350.
- **S6 AddrTable, handshake cap, per-reason refusals, slow-loris deadlines, rate limits** (E6 plan.md:465, :515;
  u4-build-plan.md:965). crates/server, net. ~350 incl. the 100,000-address test.
- **S7 ControlSocket, roster tokens, capabilities** (§10.2-10.3 plan.md:1066-1114): per-slot Unix socket,
  `SO_PEERCRED`, size-limited panic-free parse of `RoomControl`, token and capability minting, session-control gate,
  per-peer-set L2 settings. crates/server, room (roster.rs). ~400 + ~300 tests.
- **S8 npt-server binary + WT dev certificate** (main.rs; §8.5 plan.md:991-996): settings (`cmd_timing`, `cc`,
  transport), certificate made at start, lifecycle. ~300.
- **P1 the -3 probe** (C-U6-1 to -7, row:24-30): poller task on the server runtime, one ImpairSocket per leg, the fate
  log's drain, steal, the judged E's producer with SERIES.md names, rate and composition, observer calls; tests (T1 and
  T2 re-run, fixed fate entries). crates/server or lab (home not fixed), SERIES.md. ~400 + ~350.
- **P2 C-U6-8's stamp tests** (row:31; socket.rs:501, :503, :726; socket_tests.rs:904, :1263-1265). crates/net. ~150.
- **J1 judge-side carries C-U6-9 to -11** (row:32-34), after register 64's follow-ups (u4s-carries.md:455-457):
  figures.rs, fixtures, SERIES.md rows. crates/judge. ~400.
- **H1 npt-host page server** (§10.4 plan.md:1116-1135; E21). crates/host. ~400 + ~250 tests.
- **H2 instant-acme client behind `npt-cert`** (plan.md:960-969). crates/host, Cargo.toml/lock. ~250.
- **H3 npt-super** (plan.md:395, :1078-1081; E16). crates/super. ~300.
- **H4 systemd template unit, `npt-ctl`/`npt-cert` wrappers, sudoers list** (plan.md:1078-1082, :1137-1141,
  :1162-1165). ops/ (new). ~250 of text, checked at stop B (plan.md:1267-1270).

**Total:** about 9,500-10,000 added lines for the blocks above. Not counted, scope to confirm (section 7): npt-bots'
npt-net glue and npt-load (~600, u4-build-plan.md:966-967) and the 20 judge figures labelled "U6" (figures.rs:866-966;
~1,000-1,500 with fixtures).
**Natural splits:** three sub-units do not each fit under about 3,000 at these estimates; four do:
- U6a transports on loopback: T1-T6 (~2,900).
- U6b room process core: S1, S2, S3, S8 (~2,850); S2 carries C-U6-19's gate on other briefs (row:45).
- U6c control plane, instrumentation, process control: S4-S7, H3, H4 (~2,700).
- U6d probe, judge, host: P1, P2, J1, H1, H2 (~2,200).
Order constraints seen: S1 can run on Loopback before T1-T4; T4 and P1 need S1's runtime (row:24 "spawned on
npt-server's multi-thread runtime"); J1 waits on register 64 (u4s-carries.md:457); H1-H4 feed stop B (plan.md:1267).

## 4. Marks and runs

marks.json (12,630 lines, 81 marks) gives each mark a `where` list; plan §6.2 (plan.md:713-757) the run per mark.
- **Container rows U6's own tests touch:** -30's C part "C: loopback, 3 recordings x 2 transports" (marks.json:6575;
  where R3, C; bites wallclock, late-command); register 88 "-30 stays NOT-RUN until U6" (u4s-carries.md:682);
  u4-build-plan.md:964 "-30's marked C part on both real transports". -35 (where C only, marks.json:7226; "once, after
  the swap", :7321): its one counted run follows U6's merge on U5's tool (row:46; entries 110-111). Bites U6 builds:
  PM-0001-18/ws-budget-x2 (marks.json:4472; row:5, R11; today only a `#[cfg(test)]` `Bite::BudgetX2`,
  net/src/ws_gate.rs:60-69, and npt-net has no `bites` feature, net/Cargo.toml:12-15); PM-0006-20/planted-format;
  the host's 404 bite (plan.md:1131); the control-plane bites (plan.md:1094-1095).
- **Judged runs that need U6 merged first:** -3's L2 parts (process `probe`, marks.json:626ff.; R1 and the start of
  every L2 block, plan.md:723; U3 row "judged on RS at R1 (U8)", plan.md:1250); -18's ws part and -31 through
  DelayStream with U6's reader stamp (U8 row, plan.md:1255); -19 (R3, R7, ML*; marks.json:4486; gated by C-U6-11,
  -13, -16, -19: row:34, :37, :40, :45); PM-0004-61's online half (not a marks.json row; ADR 0004:1182; row:40);
  PM-0001-4 (R2) after C-U6-15's bench (row:39); -35's counted run (row:46); every run with a throwing or rejoining
  hostile client or a client restarted mid-run (C-U6-19's gate, row:45); any honest client off L1 (C-U6-17, row:42).
- **VM runs vs container:** C rows are 26 (-1, -3, -17, -57, -58, -18, -20, -21, -22, -30, -34, -35, -41, -42, -60,
  -45, -61, -47, -48, -49, -50, -51, -62, -54, -55, -56); VM run counts by `where`: R1 10, R2 22, R3 21, R4 15, R5 14,
  R6 7, R7 24 (node read of marks.json). Runs R1-R7 and their blocks: plan.md:815-834. U6-built figures (20, section
  2) are all VM-run marks: -10, -12, -13, -15, -24 (R2/R3), -27 (R5), -33 (R7), -38, -59 (R3), PM-0006-17, -19, -20
  (R2); -25 (R4), -28, -29, PM-0006-15, -25 to -27 (R4/ML*/MAC) need npt-host, AddrTable and the certificates.

## 5. New dependencies

Read: `grep -n '^name = "<crate>"' spikes/netcode-proof/Cargo.lock` (242 packages). Caveat: Cargo.lock records the
resolve across optional features, so "in the lock" is not "compiled"; e.g. aws-lc-rs is in the lock (:60, 1.19.0,
pulled by quinn-proto, rustls, rustls-webpki) while npt-net keeps it off (Cargo.toml:68-71 comment). Rule: every new
external crate is pinned exactly and reviewed (Cargo.toml:45 comment, plan §4.8 plan.md:635-653); audits at U5, stop
B and before every run, network-facing advisories block (plan.md:643-649).

**Absent from Cargo.lock today (would be new packages):**
- `instant-acme` (absent) and its HTTP stack `hyper-rustls`, `hyper`, `hyper-util`, `http-body-util` (all absent):
  plan.md:964-965 ("instant-acme 0.8.5 inside the locked workspace, with its default features off plus ring and
  hyper-rustls"); npt-host's Cargo.toml comment "instant-acme 0.8.5 joins at U6 (plan §8.4)". `hyper` is on §4.8's
  network-facing audit list (plan.md:648). `webpki-roots` absent too (whichever root store instant-acme's
  hyper-rustls uses is a choice; `rustls-native-certs` 0.8.5 is in the lock, :1295, via wtransport,
  web-transport-quinn, rustls-platform-verifier).
- `metrics` and `metrics-exporter-prometheus` (both absent; `metrics-util` absent): plan.md:481 (E22).
- `h3` absent (named only in the audit list, plan.md:648).
- No systemd or capability crate (`sd-notify`, `libsystemd`, `caps`, `nix` all absent). The plan's systemd units and
  empty `CapabilityBoundingSet` are unit-file settings (plan.md:1078-1082); no plan line names a crate for them.
- The counting allocator as a normal dependency for PM-0006-20 (`alloc_twin_diff_count`): "a normal dependency (U6's
  PM-0006-20) is U6's change with its own read" (u4s-carries.md:788-789, entry 104 (4)); U5 builds the allocator
  crate (crates/alloc per entry 104 (3), :785-787); it is not on main yet (crates/ has no alloc folder).

**Already in Cargo.lock (no new package; may need new direct deps, features or pins):**
- `tokio-tungstenite` 0.30.0 (:1717), `tungstenite` 0.30.0 (:1791), `futures-util` 0.3.34 (:389, today a dev-dep of
  npt-net): row:19 "WsServer uses tokio-tungstenite with futures-util as a normal dependency, listed in §4.8 and
  audited at U5"; Cargo.toml:72-75 comment "U6's WsServer takes it as a normal dependency".
- `wtransport` 0.7.2 (:2143), `web-transport-quinn` 0.13.6 (:1942), `quinn` 0.11.12 (:1081), `quinn-proto` 0.11.19
  (:1101), `rustls` 0.23.46 (:1279), `ring` 0.17.14 (:1241), `tokio-rustls` 0.26.6 (:1707), `libc` 0.2.191 (:690),
  `socket2` 0.6.5 (:1540): already direct or transitive (Cargo.toml:58-71). Floors PM-0001-35 (plan.md:639-640).
- `tracing` 0.1.44 (:1729; via quinn, quinn-proto, quinn-udp, wtransport, web-transport-quinn), `tracing-subscriber`
  0.3.23 (:1773; via loom only), `tracing-core` (:1752): E22 names `tracing` JSON lines (plan.md:481); a JSON
  subscriber's feature set is not fixed by any plan line found.
- `rcgen` 0.14.10 (:1191) and `x509-parser` 0.18.1 (:2179), `time` 0.3.55 (:1635), `pem` 3.0.6 (:1008): pulled by
  wtransport; §8.5's ECDSA P-256 dev certificate made at start (plan.md:992) has no named crate.
- `rand` 0.10.3 (:1155), `getrandom` 0.2.17/0.4.3 (:420, :433), `subtle` 2.6.1 (:1556), `sha2` 0.11.1 (:1480): the
  128-bit tokens and capabilities (plan.md:1084-1086, :1100) name no RNG crate; roster.rs already has its own
  constant-time `same` (roster.rs:15-24).
- `http` 1.5.0 (:459), `bytes` 1.12.1 (:119), `base64` 0.22.1 (:83), `rustls-pki-types` 1.15.1 (:1307).
- tokio's workspace features today: rt-multi-thread, net, time, sync, io-util, macros (Cargo.toml:64); Unix sockets
  and `peer_cred` sit under `net`; no `signal`, `process` or `fs` feature is on.

## 6. Overlap with lanes in flight (U4f2, U4g, U4g2, U4h, U5)

- **U4f2** (u4f2-brief.md, 266 lines): lane npt-bots, npt-lab, ops/runs, SERIES.md new rows (:170); names
  room/src/input.rs, room/tests/commands.rs, lab contract.rs/scenario.rs, bots/src/lib.rs (block list :49-94).
  Overlap with U6: crates/bots (U6's npt-bots glue and npt-load, u4-build-plan.md:966-967) and crates/room/src/input.rs
  (C-U6-19's first-command check sits where kind 5 skips it, input.rs:287; U4f2 records the bypass, C-U4F2-9,
  u4f2-brief.md:142-145).
- **U4g** (u4g-brief.md, 308): open crates/client, sim, room (room.rs packing/recovery :1437-1519, ack.rs), lab,
  bots, ops/runs, SERIES.md new rows; shut proto, server, net, judge, Cargo.toml/lock, check.mjs (:43-50). Overlap:
  room/src/room.rs (C-U6-14's generation, -16's limit and slot clock, -19's `JoinAccepted` counters, -13's event base,
  C-U6-15's cheap wins all touch room.rs), client/src/lib.rs (C-U6-13/-19 the client starting from `JoinAccepted`),
  crates/bots.
- **U4g2** (no brief file yet; scope per msg-2026-11-21.md:86-89: drawing other players, event timing, the server's
  recovery after a loss burst, about 1,600 lines): likely room.rs and client again; its files are not yet listed.
- **U4h** (u4h-brief.md, 386): open crates/sim, room, server ("one accessor and its tests": the ping-estimate accessor
  in pump.rs, block 4 :111-114), client, proto (one pub item: aim widths, proto/src/input.rs:53-54), bots, lab;
  crates/judge only under U4H-JUDGE (a) (:53-63). Overlap: crates/server/src/pump.rs and tests/common/mod.rs,
  loopback.rs (U6's TickLoop takes the same RTT hand-off, register 96; u4h-brief.md:114, :377); room.rs; npt-proto
  (U6 changes `JoinAccepted`, msg.rs:98); judge/src/figures.rs (U6's C-U6-9/-10 and its 20 planned figures, if
  U4H-JUDGE (a)).
- **U5** (u5-brief.md, 438): new fuzz/ and crates/alloc; crates/lab; ops/runs; check.mjs and check.test.mjs;
  crates/proto (the -34 bite); crates/net (the scanner's fuzz entry in l2/ws_frame.rs and a `fuzz` feature in
  crates/net/Cargo.toml, :57, :66, :110); manifests and locks of Block 0 (Cargo.toml, Cargo.lock, crates/*/Cargo.toml
  incl. server and net); SERIES.md header rows (:55-59). Overlap: crates/net/Cargo.toml (U6 adds a `bites` feature and
  normal deps), Cargo.toml/Cargo.lock (U6's new crates, section 5), check.mjs (U6's deps-step entries, e.g. npt-host
  and npt-super reach checks at check.mjs:99-102), crates/alloc (U6's PM-0006-20 normal dep). U5's -35 tool counts
  every changed path outside crates/net (u5-brief.md:197), which fixes U6's swap form (entry 111).
- **Shared by all:** SERIES.md (U6 adds C-U6-5's names, -9's row text, the reader-stamp parts); README's status line.

## 7. Open questions (no answers)

1. Size: the blocks come to about 9,500-10,000 added lines against M's about 3,000 and the stop past 6,000
   (history.md:2048-2051). Split into sub-units (section 3), and is that Mustafa's call as U4F2-SIZE and U5-SIZE were
   (history.md:7271; u5-brief.md:1-31)?
2. Scope: npt-bots' npt-net glue and npt-load are U6's by u4-build-plan.md:966-967 and the stubs
   (bots/src/bin/npt-load.rs:1), but the U6 row (row:1) does not name them. In U6 or a later unit?
3. Scope: 20 judge figures say unit "U6" (judge/src/figures.rs:866-966), none in the U6 row. U6's, or each run's
   unit (U8 and the runs)?
4. "Interest under load" (row:1): what does U6 build beyond npt-room's interest code (interest.rs) for -8/-9/-10/-12,
   which are R2 runs (plan.md:724-725)?
5. C-U6-3: which drain cadence, per second with 1-2 s capacity, or one pre-faulted log per leg (row:26)?
6. C-U6-16 (a): which limit form, the bucket kept in the roster entry or every full state counted against the
   downstream window (row:40); chosen with the advisor before -19's runs.
7. C-U6-16 (c): the control bytes' channel from a public read of a framed message's channel, or each message's
   `Limits::TYPES` row beside its bytes (row:40)? `Room::take_control` returns bare `Vec<Vec<u8>>` (room.rs:784).
8. C-U6-19: `JoinAccepted`'s counters, or the room-only form if every session's counters start at 0, with the netcode
   and security seats' read (row:45)?
9. C-U6-14: a session generation checked in `receive`/`leave`, or a test that no closed session's message reaches the
   room; and does npt-server's library need an end-session call (row:38)?
10. C-U6-11: how does a client admitted mid-run start its span (row:34)? And does J1 wait on register 64's judge
    follow-ups landing first ("at the latest before U6 builds -19's figure", u4s-carries.md:457)?
11. C-U6-6: does -3's sends plus echoes give 10,000 per direction through the probe's socket? If not, the advisor
    before R1's run plan (row:29; ADR 0001:1590).
12. The probe's home: row:24 puts its poller on npt-server's runtime; §4.7 lists npt-lab's `probe` subcommand (U2,
    plan.md:612) and no probe item in npt-server (plan.md:600-601). Which crate?
13. -35's swap "inside crates/net (npt-net's own selection)" (row:46): today the crate is chosen by npt-net features
    (net/Cargo.toml:12-15) that a dependent must turn on, and U5's tool counts "only crates/server/Cargo.toml changed"
    as a change outside (u5-brief.md:202). How does U6 make the selection npt-net's own?
14. Drift: the reader stamp's site is cited as delay.rs:1884 (row:22); the pump's closing `wake_all` is now delay.rs:1907
    (`poll` at :1709). Same site, moved?
15. C-U6-15: the room-tick bench is "measured on R2" (row:39), a VM run; are the cheap wins (code) inside U6 and the
    bench a pre-run step of R2's plan?
16. The systemd unit and the `npt-ctl`/`npt-cert` wrappers: U6's row names them (row:1), U8 builds the run package
    (plan.md:1255), and §4.1's ops/ lists cloud-init/, firewall/, runs/, operator/, checklists/ only (plan.md:399-403).
    Where do they live, and `User=npt-room-NN` or `DynamicUser` (plan.md:1078)?
17. §8.5's WT dev certificate: made by "the room server or supervisor" (plan.md:992). Which?
18. instant-acme: propose the 90-day fallback now (needs security-auditor's and the advisor's OK at U6,
    plan.md:969), or not? Which root store for hyper-rustls (section 5)?
19. Row:20: one measured run per server process, or the POOL reset at each run's start (delay.rs:195)? Row:15: do
    -25's never-joining sessions get a DelayStream?
20. Carry 60: weapon and health go in npt-log's `Header` (no such field today) or in params (row:9; u4s-carries.md:285-290)?
21. C-U6-19's gate: U4g, U4h and U5 state "no throwing or rejoining hostile client" (u4g-brief.md:187,
    u4h-brief.md:228, u5-brief.md:256-282 section); U4g2 has no brief yet. Does U4g2's brief need the same line?

DONE
