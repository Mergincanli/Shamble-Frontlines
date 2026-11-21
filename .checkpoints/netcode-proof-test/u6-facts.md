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

(pending)

## 3. Build blocks (proposed grouping, sizes, split)

(pending)

## 4. Marks and runs

(pending)

## 5. New dependencies

(pending)

## 6. Overlap with lanes in flight (U4f2, U4g, U4g2, U4h, U5)

(pending)

## 7. Open questions (no answers)

(pending)
