# security-auditor: U6b's one review round (xhigh), 2026-11-23

Subject: wip/u6b-build.patch (49f4488f...c14c, base 21765e3, 29 files, +5,498 -254), built tree W =
.claude/worktrees/agent-a2661dc320c9e6d00/spikes/netcode-proof (read only). Brief u6b-brief.md rev 1.1 (32baaebd...),
advisor-u6b-brief.md (3c0f8121...), security-u6b-deps.md (3799ac81...), builder's notes u6b-build.md (be6807f4...),
U8's tree .claude/worktrees/agent-ac59cce3e5d002441 (u8-build.patch 0e7fbab2...). All six hashes checked at the start:
equal. (No Edit tool in this agent: this file is rewritten whole with Write after each block.)

## 1. Plan
1. Inputs: hashes (done), brief §0.1/§1/§7, advisor check, Block 0 read, builder notes, replan.md:56-60, :93; the
   patch's file list; W equals the snapshot (spot check).
2. Accept paths: ConnCap (cap.rs), WtServer's accept loop (wt_server.rs + l2/wt.rs), WsServer's accept path
   (ws_server.rs): permit before TLS, refuse/drop, deadline over QUIC + HTTP/3 and TCP + TLS + upgrade, permit return.
3. Size limits at the socket and panic-free framing: WtLink's control stream (2 B length, 512 B), FullState stream
   (4 B, 65,541 cap), datagrams; WsLink's tag and tungstenite limits (server 513 B; client limits per condition 7);
   bounded queues; any `unwrap`/`expect`/index/slice reachable from bytes.
4. The pin (pin.rs): SHA-256 of the end-entity, signature still verified through the provider; both clients.
5. The dev certificate and hash file (cert.rs, main.rs): key in memory, `create_new` + 0644 at creation, one
   `SystemTime::now`, exit 2 not panic, the hash file's fields; P15's evidence.
6. The public key file's read (WsServer::bind_pem / main.rs): size cap, path, permissions, no key in logs.
7. The bites gate: `Bite::BudgetX2` cfg, per-instance arming, the binary's `--bite` path, measured build.
8. U6a's token minting + token file + Join path (condition 6), and U8's `npt-ctl rooms tokens NN` as `npt-room`.
9. security-u6b-deps.md's build-time conditions: one TLS provider (ring), no install_default/ClientBuilder/
   ServerBuilder/Client::default/.origin(), no tracing subscriber, no Url::origin on client input.
10. Log hygiene: no key, token, address, request line or URL in what U6b writes (event rows, header, stderr).
11. Evidence: check5.log and the plant logs I lean on (P1, P3, P4, P14, P15); a test run only if a claim needs it.
12. Verdict, table, not verified, DONE.

## 2. Notes (one line per finding, saved as I go)
- Step 1 DONE: all 29 files of W equal the patch's post-images (git hash-object against each `index` line); U8's
  npt-ctl, npt-lib.sh, rs.yaml, firewall/rs.txt, R1.md, r1.sh equal u8-build.patch's.
- Exposure (firewall/rs.txt:10-11 in U8's tree): UDP 30001-30032 and TCP 31000-31032 admit only <HOME_IPV4>/32 and
  <LG_IPV4>/32. So in R1 a hostile client must sit on his home network or on LG; only UDP can be spoofed from outside.
  Every client-side finding below is rated with that reach.
- cap.rs: one Semaphore of N + 8, `try_acquire_owned` at the `Incoming` / TCP accept, refused counted per transport;
  the permit is an owned guard held by the closing task. CLEAR.
- pin.rs:34-47 hashes the end entity and compares it to the 32 B pin; :58-70 verifies the TLS 1.3 CertificateVerify
  through the provider's algorithms; TLS 1.2 refused; TLS 1.3 only. The hash is public, so a non-constant-time compare
  is fine. CLEAR (code); see SEC-U6B-4 for its test.
- Accept paths: wt_server.rs:201-232 takes the permit at `Incoming`, `refuse()` with none free, and runs
  `incoming.await` (QUIC) + `Request::accept` (HTTP/3 SETTINGS and CONNECT) + `req.ok()` (the answer) + the hand-off
  under one `timeout_at(start + 2 s)`; ws_server.rs:422-451 takes it at the TCP accept, drops with none free, and runs
  TLS + DelayStream::wrap + the upgrade + the hand-off under `timeout_at(accept + 2 s)`. A timeout drops the future and
  its permit. Room side: pump.rs:382-385 closes an un-joined session at accept_ns + 2 s; the accept stamp reaches
  `Decoder::new` through `Connector::connect` (tick_loop.rs, Connect::connect). Plants P3 (plant-p3.log: FAILED at
  wt_server_tests.rs:269) and P4 (plant-p4.log: FAILED at ws_server_tests.rs:254) hold. CLEAR.
- Framing (server): control length u16 checked `2 + n > 512` before the allocation (wt_server.rs:445-452); the server
  reads no client unidirectional stream; WebSocket frames and messages at 513 B from one `&L2Config` (ws_config.rs
  :51-57), the upgrade request at 8,192 B on DelayStream's path (config.rs:88) and 64 KiB in tungstenite on the plain
  path (tungstenite 0.30.0 handshake/machine.rs:169); an empty, unknown-tag or text message closes (ws_server.rs
  :769-794); inbox 256 messages (session.rs:50); send queue 128 KiB and 256 messages. web-transport-quinn's capsule
  reader caps a frame at 64 KiB (web-transport-proto 0.6.4 capsule.rs:291-293). No panic reachable from bytes in U6b's
  readers; the one `panic!`/`assert!` on `send_reliable(Channel::Datagram)` (wt_server.rs:380, ws_server.rs:715) is
  the trait's documented caller contract (link/src/lib.rs:96-97), never reached from the network. tokio's
  `try_reserve_many(n)` returns Err above capacity, never panics (tokio 1.54.1 bounded.rs:1435-1436). CLEAR.
- **SEC-U6B-1 (Medium), the closing task has no bound:** after a link is dropped, WtLink's writer drains its queue
  with `write_all` / `open_uni` that have no deadline (wt_server.rs:496-509); only `stopped()` after `finish` is under
  CLOSE_WAIT (:515-519). The permit returns only when that task ends (:292-296). The room queues `JoinRejected`
  (pump.rs:325-328, from `join_ok` :485-497) or `Kick(malformed)` (:364-367, called at :420, :427, :444) for a client
  with no token, so a client that grants credit on the CONNECT stream but none (or 1 B) on its control stream
  (quinn-proto 0.11.19 streams/send.rs:68-70 returns `Blocked` at zero credit) and keeps the connection alive holds one
  permit for ever; 56 of them refuse every client on both transports. The brief's ban "every wait under a deadline"
  (u6b-brief.md:424) is broken; not a cut item (the kept fixed cap and Join deadline are what it defeats). WsLink's
  drain (ws_server.rs:828-839, before the SHUTDOWN_WAIT timeout at :848) has the same shape, but a pre-Join message
  fits in the kernel's send buffer, so there only a joined client that stops reading can hold it. Reach in R1: his
  LAN or LG (firewall); honest clients end within quinn's 30 s idle timeout. Fix: from the link's drop, run the drain
  and the close under one timeout (e.g. CLOSE_WAIT), then `session.close` (pending writes fail) and the permit drops;
  the same around WsLink's drain loop; a test with a peer that never grants control-stream credit (the permit back
  within the bound) and a plant (the bound removed) failing it.
- **SEC-U6B-2 (Medium), quinn's default stream limits stand on the server:** the server's TransportConfig sets only cc
  and the datagram buffer (wt_server.rs:57-75; l2/wt.rs:10 "quinn's defaults"), so a client may open 100 bidi + 100
  uni streams at a 1.25 MB window each with an unlimited connection window (quinn-proto config/transport.rs:373-378;
  its own doc :66-67: worst case proportional to streams x window). The server accepts one bidi stream and no uni
  stream after the handshake (wt_server.rs:273; web-transport-quinn 0.13.6 session.rs:259-290 accepts only on
  request), so the rest sit buffered: about 250 MB per connection, about 14 GB at 56 connections once SEC-U6B-1 holds
  them, past what one RS (CCX23, plan.md:677) holds. "Allocation bounded for anything a client sizes" holds only at
  quinn's ceiling. Fix (in the server's `transport()` only): `max_concurrent_bidi_streams` small (CONNECT + control +
  slack), `max_concurrent_uni_streams` small (HTTP/3 control, QPACK x2, grease slack; check against U7's headless
  Chromium), `stream_receive_window` 64 KiB, `receive_window` 1-2 MiB; a test that reads the four values off the
  config (or a client's extra stream left blocked).
- **SEC-U6B-3 (Low, to the security baseline):** the permit is taken at the first QUIC Initial, before address
  validation (wt_server.rs:201-207), as the brief asks. A spoofed UDP source from the allow-list can make each Initial
  hold a permit to its 2 s deadline: about 28 Initials a second keep the cap full. In R1 the spoofer must know the two
  allow-listed addresses (not public), so R1 does not need it; this is the flood family the re-plan cut
  (replan.md:56-57). Fix for the baseline: `incoming.retry()` unless `remote_address_validated()`, the permit after
  (RFC 9000 §8.1).
- **SEC-U6B-4 (Low), the pin's signature check is untested:** b1_the_pin_takes_the_hash_and_refuses_another
  (ws_server_tests.rs:161-183) and tests/binary.rs:294-312 prove the hash half only; a plant making pin.rs:58-70
  return `Ok(HandshakeSignatureValid::assertion())` would pass every test, and the pin alone then lets anyone replay the
  public certificate. Fix: a unit test in pin.rs calling `verify_tls13_signature` with a garbled
  `DigitallySignedStruct::new(ECDSA_NISTP256_SHA256, ..)` over the pinned certificate (Err expected), and that plant.
- **SEC-U6B-5 (Low), P15's test reads one folder:** u6b_open... asserts only that the hash file's folder holds
  room.json alone (tests/binary.rs:316-329); a key written beside the log or the token file passes. By reading, the key
  is never written (main.rs:542-585: `dev.key` reaches only the two binds; cert.rs:96-120). Fix: list the whole run
  folder after the run (settings, tokens, room.jsonl, hash/room.json) and scan each file for the PKCS#8 P-256 prefix.
- Dev certificate: cert.rs:65-76 the one `SystemTime::now` (allowed, reason stated); days to civil tested on the fixed
  dates; the year range checked before `rcgen::date_time_ymd`, so its `expect` cannot fire (cert.rs:82-94); rcgen
  ECDSA P-256 key from ring, X.509 v3, 13 days (VALIDITY_NS 1,123,200,000,000,000); the key kept as DER in memory;
  hash file from a three-field struct, `create_new` + mode 0644, `write_all` + `sync_all` (cert.rs:131-145), exit 2 on
  an existing path (main.rs:547-554). P14 and P15 fail their test (plant-p14.log binary.rs:235; plant-p15.log :325).
  CLEAR (with SEC-U6B-5 on the test).
- Public key file's read (main.rs:565-571, :676-688): `take(cap + 1)` with PEM_CAP 64 KiB, exit 2 above it, fixed
  error strings (ws_server.rs:293-300; "a certificate file: {e}" carries no path or bytes); the key reaches only
  rustls; nothing in the header or the log. U8 gives it group npt-room, 0640, in a 2750 folder (rs.yaml:164-166).
  CLEAR.
- Bites gate: `Bite::BudgetX2` and `WS_BUDGET_BITE` under `cfg(any(test, feature = "bites"))` (ws_gate.rs:63,
  ws_server.rs:240-259), the Queueing bite test-only; npt-net's `bites = []`, never in `default = ["wt-wtq"]`; the
  binary's `bite` field exists only in bites builds and `deny_unknown_fields` refuses it elsewhere (main.rs:126-129);
  armed per WsServer instance from the operator's settings, never from a client message; the header records the
  features. CLEAR (SEC-U6BD-2's wording holds).
- U6a's tokens and Join (condition 6): `mint` reads n x 16 B from /dev/urandom by `read_exact`, distinct
  (main.rs:315-325); the token file `create_new` + 0600 at creation, one `write_all` + `sync_all` (:355-367); no token
  in any message (fail's lines carry io::Error text only); the roster compares each slot in constant time and reads
  all slots (roster.rs:18-25, :96-125); Join's Debug redacts the token (proto msg.rs:85-94); version and content hash
  checked first (pump.rs:485-497). CLEAR.
- U8's reader, `npt-ctl rooms tokens NN` (npt-ctl:147-154): sudoers anchors `^rooms (start|tokens) (0[0-9]|[12][0-9]
  |3[01])$` (sudoers.d/npt:8), npt-ctl checks NN again (npt-lib.sh:31-32), the environment is cleared
  (npt-ctl:14-16), and the file is read by `setpriv --reuid=npt-room ... head -c 1048576` to stdout only.
  /var/lib/npt/rooms is npt-room:npt-room 0755 (rs.yaml:163), so the operator user cannot plant a link there; a link
  the room plants reaches only what npt-room already reads. r1.sh pipes the tokens to LG (r1.sh:352) and states
  "never a log" (:21). CLEAR.
- Block 0's conditions in the build: W's Cargo.lock sha256 210bf2df...a219 equals my modelled after-lock
  (security-u6b-deps.md:123), 235 entries, no aws-lc/cmake/dunce/fs_extra/pkg-config/jobserver line; no Cargo change
  in the patch; ring named per configuration (l2/wt.rs:176-179, ws_config.rs:22-24, pin.rs:86); no
  `install_default`, `ClientBuilder`, `ServerBuilder`, `Client::default` or `.origin()` in crates/net or crates/server
  (the `set_default` hits are ImpairSocket's own); WtClient uses `Client::new` with our config (wt_server.rs:589); no
  tracing subscriber. CLEAR.
- Log hygiene: U6b's event rows, header params and npt-super's records are counts, fixed names and the server's own
  settings; `Accepted.peer` never reaches a row (main.rs:626-636 passes link and stamp only); ImpairSocket's and
  DelayStream's header texts name modes and clocks (socket.rs:351-367, delay.rs:1369-1375); every `println!` in the
  patch is in a test file. CLEAR.
- Evidence: check5.log, all 11 steps PASS (measured 799, bites 898, redact 0 findings); plant logs P1, P3, P4, P14,
  P15 each fail the named test with its own diff hash. No test run by me (none of my claims is one an existing test
  decides; a hostile-client demo needs new code, outside this round's rules).

## 3. Verdict
**REVISE.** No Critical or High, so no second security round (NPT-TRIM). Two Mediums block the merge until the fix
pass lands them: SEC-U6B-1 (a bound on the closing task, both transports) and SEC-U6B-2 (the server's QUIC stream
limits), each a timeout or four config lines, with its test and a plant; the advisor's check of the fix diff is
enough. The three Lows may go to STATUS follow-ups (SEC-U6B-3 to the security baseline). R1's safety behind its
firewall needs none of the re-plan's cut items. Everything else in section 7's scope is CLEAR: both accept paths' cap
and deadlines, the socket limits and framing, the pin's code, the dev certificate and hash file, the public key file's
read, the bites gate, U6a's tokens and Join, U8's token reader, one TLS provider, and log hygiene.

## 4. Findings table
| ID | Sev | What | Exploit sketch | Evidence (W unless named) | Smallest fix |
|---|---|---|---|---|---|
| SEC-U6B-1 | Medium | A dropped link's closing task drains with no deadline; the permit returns only at its end | No-token client: CONNECT, junk first message, zero control-stream credit, keep-alives: the permit is held for ever; 56 lock out both transports | wt_server.rs:292-296, :496-509, :515-519; pump.rs:325-328, :364-367, :495; quinn-proto send.rs:68-70; ws_server.rs:828-839, :848 | From the drop, the drain and close under one timeout, then `session.close`; the same on WsLink; a test and a plant |
| SEC-U6B-2 | Medium | Server QUIC keeps quinn's 100 + 100 streams x 1.25 MB, connection window unlimited; extra streams never read | Open 199 extra streams, fill each: about 250 MB per connection, about 14 GB at the cap | wt_server.rs:57-75, :273; l2/wt.rs:10; quinn-proto config/transport.rs:66-67, :373-378; web-transport-quinn session.rs:259-290 | Server `transport()`: small stream counts, 64 KiB stream window, 1-2 MiB connection window; a test |
| SEC-U6B-3 | Low | Permit taken at the first Initial, before address validation | Spoofed allow-listed source: about 28 Initials/s keep the cap full | wt_server.rs:201-207 | Security baseline: `retry()` unless validated, the permit after |
| SEC-U6B-4 | Low | The pin's signature check has no test or plant | A regression to a hash-only pin passes CI; anyone replays the public certificate | pin.rs:58-70; ws_server_tests.rs:161-183 | A unit test with a garbled signature, and that plant |
| SEC-U6B-5 | Low | P15's test checks only the hash folder | A key written beside the log passes | tests/binary.rs:316-329 | List the whole run folder; scan for the PKCS#8 P-256 prefix |

Anti-cheat and abuse coverage (U6b's transport slice):
| Client action | Where the server checks it | Gap |
|---|---|---|
| Open a QUIC or TCP connection | ConnCap at Incoming / TCP accept (cap.rs:63-71; wt_server.rs:204-207; ws_server.rs:435-438) | SEC-U6B-3 (spoofed Initials; baseline) |
| Handshake (QUIC, HTTP/3, TLS, upgrade) | One 2 s deadline from the first packet / accept (wt_server.rs:227; ws_server.rs:445) | none |
| Send Join (token) | pump.rs:485-497 version and hash; roster.rs:96-125 constant-time token, bind; 2 s Join deadline (pump.rs:382-385) | none |
| Control-stream messages | 2 B length, 512 B (wt_server.rs:445-452); WS 513 B, tag (ws_config.rs:51-57; ws_server.rs:769-794); decoder closes bad ones | none |
| Datagrams (inputs, pings, pongs) | Bounded inbox, drop counted (session.rs:65-74); decoder's buckets (pump.rs) | none in U6b |
| Extra QUIC streams | quinn defaults only | SEC-U6B-2 |
| Stop reading / withhold credit | 128 KiB send queue, BufferFull | SEC-U6B-1 (close unbounded) |
| Arm a bite | Settings file only, bites builds only | none |

## 5. What I did not verify
- No run demonstrates SEC-U6B-1 or SEC-U6B-2: this round's runner takes `cargo test` in W only, and a hostile client
  is new code. Both rest on source reads: quinn-proto 0.11.19 streams/send.rs:68-70 and config/transport.rs:66-67,
  :373-378; web-transport-quinn 0.13.6 session.rs:127-320, :740-990. Confidence: high on the code paths, medium-high
  that a stock quinn client with a 1 B stream window gets the CONNECT answer inside 2 s.
- RS's memory: plan.md:677 names a CCX23 without its RAM; I did not read Hetzner's page today.
- How many unidirectional streams Chrome, Edge and Firefox open over HTTP/3 (for SEC-U6B-2's numbers): not read.
  U7's headless Chromium run should confirm them.
- U6a's code outside U6b's diff (pump.rs, roster.rs, msg.rs) read for the Join path only, not re-reviewed whole.
- U8's r1.sh and R1.md beyond the token lines: Stop B's read.
- Each plant's own diff text: the logs carry only its hash. I read P1, P3, P4, P14 and P15's failures; the others are
  the test seat's.
- What quinn and web-transport-quinn's `tracing` events say: no subscriber is installed, so none reaches a log today.
- Section 3's container readings (not built: the fix pass).

DONE
