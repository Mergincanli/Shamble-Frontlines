# u3-verify2-harness: second skeptic round, the reconnect harness (fixes.rs run() and policy)

Agent: netcode-architect (skeptic). Branch claude/netcode-proof-test-plan-hcky5g, head cc72f6f (+0dae8b7 checkpoint).
Scope: fixes.rs run() and its policy vs browser behaviour (Chromium, Firefox sources) and ADR 0001; items (1)-(9) of the task.
Rules: no tracked edits, no commits; diagnostics only in a throwaway worktree under .gate/.

## Plan
1. Read first-round notes (u3-verify-fixes-model.md, u3-verify-fixes-adr.md), the fix plan u3a-fix-plan.md, the commit message.
2. Read fixes.rs (run(), policy, tables, ws_limit_tail), tests.rs S13, relevant parts of tcp_model.rs.
3. Item (1): Chromium websocket_endpoint_lock_manager.cc, transport_connect_sub_job.cc, websocket_transport_client_socket_pool;
   Firefox WebSocketChannel.cpp (FailDelayManager, nsWSAdmissionManager); what ws.close() on CONNECTING frees; timeouts.
4. Item (2)-(5): harness policy vs ADR 0001 text (tick, Join, FullState, "decodes deltas again").
5. Item (6): flights sizes; Cargo.toml TLS stack (rustls? aws-lc-rs? ring?) and its default key share.
6. Item (7): side-effects table; cost of a false takeover vs FullState.
7. Item (8): ADR conflicts compared with round 1's list; token claims "bound to the session".
8. Item (9): anything biasing the numbers; run the pinned test in a worktree; maybe a probe or two.
9. Verdicts, bugs, must-correct; DONE.

## Findings (one line each, appended as found)
- H0 (state): head 0dae8b7 (cc72f6f + checkpoint); first-round notes read (u3-verify-fixes-adr.md F0-F22, A1-A3;
  u3-verify-fixes-model.md F0-F12, D1-D10); fix plan u3a-fix-plan.md read; fixes.rs read whole (706 lines).
- H1 SOURCE Chromium main (saved copies scratchpad/chromium-src, fetched 2026-11-06; plus scratchpad/cr2 fetched
  2026-11-07 from raw.githubusercontent.com/chromium/chromium/main): the lock is taken by WebSocketStreamSocket before
  the TCP connect (transport_connect_sub_job.cc:117-131; wss.cc Connect() locks first), keyed (IPEndPoint, NAK)
  (websocket_endpoint_lock_manager.cc:28-45), released by the EndpointLock destructor (:47-58) or UnlockEndpoint at
  Upgrade (handshake_stream.cc:408-410, first round), each through a 10 ms delayed task (:142-159, kUnlockDelayInMs :26).
  The socket wrapper lives through TLS and the HTTP upgrade, so "held from creation to its 101" holds (strictly from
  the TCP connect, after DNS). ws.close() on CONNECTING: Blink websocket_common.cc CloseInternal -> channel->Fail
  (kConnecting branch) -> posted TearDownFailedConnection (websocket_channel_impl.cc:480-508, :1002-1011) -> Dispose
  resets websocket_ and handshake_client_receiver_ (:1273-1283) -> network service disconnect handlers (websocket.cc:
  415-421, 646-647) -> OnConnectionError -> factory_->Remove(this) (:776-780) -> socket and lock destroyed -> freed
  10 ms later, plus one renderer task and one IPC. Model's abort: +10 ms (fixes.rs:339-342). CONFIRMED (slightly
  generous by a task hop; irrelevant to the never-abort row). Sure: high on the network side, medium on Blink's
  HandleDidClose -> Disconnect step (not read line by line).
- H2 SOURCE Chromium timeouts: WebSocket handshake 240 s (websocket_stream.cc:60-64, timer :249-252, OnTimeout :337-339),
  TCP connect job 4 min (transport_connect_job.cc:223-231). So a page that never closes an attempt keeps it until the
  OS gives up its SYNs (Linux: tcp_syn_retries 6 = 67 s with linear 4, ip-sysctl.rst:892) or 240 s. "Never abandoned"
  is the default. Dual-stack: TransportConnectJob starts an IPv4 sub-job 300 ms after IPv6 (:404-425), each with its own
  endpoint lock; the proof test has no AAAA (plan.md:869-870), so not in play there; ADR 0006's dual stack would be.
- H3 SOURCE Chromium per-renderer throttle (services/network/websocket_throttler.cc:46-55): each new WebSocket waits
  rand(1000..5000) x 2^min(p + f/(s+1), 16) / 65536 ms (p pending, f failed = any attempt destroyed before its
  handshake completed (:27-33), s succeeded; counts roll every 2 min, :70-76, :124-127). p <= 1, f small: < 0.2 ms.
  Not modelled; matters only for abort policies after many killed attempts (f = 8, s = 0: 4-20 ms; f = 16: 1-5 s).
- H4 SOURCE Firefox WebSocketChannel.cpp (saved gecko copy) + dom/websocket/WebSocket.cpp (main, fetched 2026-11-07):
  page close() on CONNECTING -> FailConnection(code) -> CloseConnection -> mChannel->Close (WebSocket.cpp:2600-2601,
  :542, :605); without transport the channel stops with NS_ERROR_NOT_CONNECTED unless code 1001 (Channel.cpp:3644-3652;
  a page may only pass 1000 or 3000-4999). OnStopSession (:389-421): "Don't count close() before connection" applies
  ONLY when a FailDelay record already exists (:400-410); with none, ANY failed stop, a page close included, ADDS one
  (:414-420): the next attempt to that IP/path/port waits 200-400 ms (:114-131, DelayOrBegin :232-255), cleared only by
  a successful open (OnConnected :376-379) or 60 s + delay (:156-159). So in Firefox an abort-and-retry's next attempt
  starts 200-400 ms after the abort, not 10 ms: the abort rows (500 ms, 1 s) are too good for Firefox. The queue keys
  the resolved IP without port (IndexOf :545-552; mAddress from DNS :3093). Open timeout 20 s (mOpenTimeout :1181,
  pref network.websocket.timeout.open :3457-3459): "never abandoned" lasts 20 s in Firefox. Sure: high (read the code).
