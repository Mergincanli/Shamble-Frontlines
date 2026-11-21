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

(pending)

## 4. Marks and runs

(pending)

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

(pending)

## 7. Open questions (no answers)

(pending)
