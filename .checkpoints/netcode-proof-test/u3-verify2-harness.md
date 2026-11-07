# u3-verify2-harness: second skeptic round, the reconnect harness (fixes.rs run() and policy)

Agent: netcode-architect (skeptic). Branch claude/netcode-proof-test-plan-hcky5g, head cc72f6f (+0dae8b7, df4afa1
checkpoints; spikes/ unchanged since cc72f6f). Scope: fixes.rs run() and its policy vs browser behaviour (Chromium,
Firefox sources) and ADR 0001; items (1)-(9) of the task. Rules: no tracked edits, no commits; diagnostics only in the
throwaway worktree .gate/u3v2h-wt (target dir .gate/u3-adr-diag-target). Process slip: the first findings block was
appended with a shell heredoc, not the Write tool; every later save uses Write (content unchanged).

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
- H5 (reproduction, worktree .gate/u3v2h-wt at df4afa1, output scratchpad/u3v2h-run1.txt): the pinned test passes
  (0.28 s); ws_limit_table, side_effects_table and ws_limit_tail reproduce every number the main session will send
  (today 746/947/3,170; fix 1 alone 435/664/1,552/5,106, 434 over 1 s; recommended 435/659/1,025/1,363/1,685, 63 over
  1 s, 2 over 1.5 s; pass chances 0.558/0.910/0.984/1.000; side effects 4/20/74/116 per mille, 0/0/11/18 takeovers).
  Note fix 1 alone has a cluster: 434 over 1 s but 433 over 1.2 s and 82 over 1.5 s (0.7 % of phases in 1.2-1.5 s).
- H6 (diag, recommended row, 60,000 phases): EVERY phase starts an attempt (60,000/60,000); the first attempt starts
  596-786 ms BEFORE the return (p50 -637), i.e. ~360 ms into the 1 s burst (trigger p50 318 ms, p1-p99 306-347 ms), so
  its SYN is always lost and the OS's SYN timer (+1 s) decides when it can connect. Only 680 phases (1.1 %) recover via
  the reconnect; in 59,403 of 60,000 (99 %) the never-abandoned attempt completes AFTER the old connection already
  recovered and the newest Join takes the session over anyway (run 3 s past recovery: 60,171 takeovers, 60,167
  switches). So under B1 every client switches connection after every burst: 480 needless takeovers per proof-test
  run, 48 at once from LG's one address. Not in the side-effects table (it runs without bursts).
- H7 (diag, client policy uses server state): busy = `step != Live` (fixes.rs:330-332); Live is set by the ROOM at its
  tick (:279), which the page cannot see until JoinAccepted arrives. A second attempt starts at once in 415 of 60,000
  phases (the "leak"). Client-visible variant (busy until the client switched to it or closed it): identical numbers
  on 6,000 phases (437/664/1,145/1,317; 67 via reconnect). Policy bug, no effect on the numbers.
- H8 (BUG, cap): `max_attempts` is checked against out.attempts, which never resets (fixes.rs:189, :325, :357), though
  its doc says "in one outage" (:68). Over 30-min runs the cap binds at P3/P4 (capped 8/8/8, uncapped 9/15/13 at P3 and
  17/12/20 at P4); in the 10-min side-effects runs it hid one attempt at P4 (8 vs 9: 18 -> 19 takeovers, 0.60 ->
  0.63/min). In phases (one burst) it never binds (most 3). A real client built to this spec would stop reconnecting
  after 8 outages per session, i.e. inside PM-0001-18's 10-burst run. The spec must say "per outage, reset when data
  resumes".
- H9 (item 3, the input echo): the model's echo = input write to the first snapshot written at a tick at or after the
  input reached the room (fixes.rs:209-221, :284, :299): RTT + the wait for the tick. Measured: P1 58, P2 117, P3 173,
  P4 227 ms mean, vs the room's srtt 57/117/175/214 ms; the tick wait is a fixed per-seed phase (inputs at a fixed
  offset from ticks, :169), not random as in a real client whose clock drifts. A real client gets the same if it times
  the echo from the snapshot header's "newest command received" (ADR 0001:668); from "last processed command" it adds
  the input buffer (each buffered tick 33.3 ms, PM-0001-20), raising the trigger by 2 x that. The trigger is pinned at
  ~300-350 ms at P2 by min_gap anyway. CONFIRMED, with the field to name.
- H10 (item 6, flights): rustls 0.23.46 is built with both providers (Cargo.lock:1245-1257: aws-lc-rs and ring); its
  default features include prefer-post-quantum (docs.rs/crate/rustls/0.23.46/features), under which the aws-lc-rs
  provider puts X25519MLKEM768 first (docs.rs src rustls/crypto/aws_lc_rs/mod.rs:254-262); the ring provider has no
  ML-KEM group (docs.rs rustls::crypto::ring::kx_group: X25519, SECP256R1, SECP384R1). The wss server code is not
  written; the plan names no provider for it (plan.md:879-880 is the ACME client's ring choice). So 3,800 B holds only
  for aws-lc-rs with defaults and no TLS resumption (rustls servers send TLS 1.3 tickets by default; a reconnect from
  the same browser would resume: no certificate, ~1 segment). Sensitivity (60,000 phases): server flight 1,300 B or
  300 B: p99.9 1,025 -> 977, over 1 s 63 -> 57, max 1,685 unchanged. Pessimistic, small. ClientHello 1,800 B: plausible
  (2 segments either way).
- H11 (item 9, the client OS's SYN timer decides the gain): the reconnect helps only through the OS's SYN retry
  after the burst. Windows-like SYN doubling from 1 s (1, 2, 4 s), no timestamps: 436/665/1,069/max 1,691, 64 over 1 s
  (= the Linux no-timestamp row). A 3 s initial SYN timeout (older Windows' InitialRto): 436/670/1,542/max 2,879, 438
  over 1 s: the recommendation then gains nothing over fix 1 alone. Windows' current default is unclear from public
  docs (search 2026-11-07: Set-NetTCPSetting InitialRtoMs range 300-3000 ms, default not stated; a community answer
  says 1,000 ms). Mustafa's laptop runs the browser leg: `Get-NetTCPSetting | Select SettingName, InitialRtoMs,
  MaxSynRetransmissions` settles it. The abort-500 row is robust to it (3 s SYN: max 1,815, 66 over 1 s): it re-SYNs.
- H12 (item 1, Firefox fail delay applied, 60,000 phases each): abort 500 ms: Chromium 435/662/1,015/1,546 (66 over
  1 s); Firefox 435/625/964/1,476 (53 over 1 s; the delay happens to push the 2nd SYN past the 1 s burst: luck of the
  burst length). Abort 1 s: Chromium 435/660/1,028/1,691 (65 over 1 s); Firefox 435/664/1,279/1,691, 370 over 1 s.
  Never-abort: no page-closed attempts, so Firefox's delay never arises (except attempts killed at a switch).
- H13 (item 7, false takeovers): side-effects silences (3 x 10 min, no burst): P3 adaptive 195 silences > 300 ms,
  72.6 s total, longest 869 ms; with the reconnect 136, 51.3 s, 794 ms. P4: 687, 250.0 s, 1,083 ms vs 701, 252.3 s,
  887 ms. So a false takeover costs no extra silence. Post-recovery (3 s after the first fresh delta, 60,000 phases):
  longest silence p50 171 ms in both fix 1 alone and the recommendation (TCP's catch-up after the burst: a ~170 ms
  hitch after "deltas again" in half the bursts, not measured by PM-0001-18); > 200 ms 5,070 vs 5,759 (+1.1 % from the
  post-recovery takeovers), max 1,536 vs 772. What a takeover costs beyond a FullState: a TCP+TLS handshake on the room
  (a signature, a key exchange), a spare token and a lobby refetch, a replay-cache entry, the old connection's queued
  snapshots, and the inputs the client still sends on the old connection until its FullState arrives (fixes.rs:306-311
  writes inputs to client_cur, which switches only at :251-259): about RTT/2 + the FullState's transfer, 3-5 commands
  at P4, lost unless the server reads the old connection until the new one's first command or the redundancy cap covers
  them. 50 of 60,000 phases recovered on the old connection after the room had taken the session over: that needs the
  old connection closed gracefully (queued bytes still sent), not reset.
- H14 (item 4, the tick): ADR 0001:618-623 "One server tick (30 Hz)": commands taken from a queue decoded on the I/O
  side, deltas encoded per client and "hand the datagrams to the I/O runtime without waiting": writes on the tick
  match (fixes.rs:271-305). Joins on the tick, the full state written at once, no delta that tick (:274-295): a
  modelling choice the ADR does not state (0001:672 FullState "at join and on resync"; 0001:747 B1 "the server deltas
  from an older baseline or the defaults"): a resumed session could get a delta against the client's last acked
  baseline instead of a FullState, one tick sooner. Pessimistic, small (the reconnect decides 1.1 % of phases).
  CONFIRMED as a reasonable reading; the docs PR must state what a takeover sends.
- H15 (item 5, "deltas again"): the old connection's fresh delta counts until the client switches (fixes.rs:371-377):
  realistic (the client has not switched; TCP delivers in order; the room wrote it before the takeover). On a new
  connection the first delta after the FullState counts (:300-302, full_state_counts false): matches PM-0001-18's words
  (0001:1556). Under 0001:668/746 the delta after a FullState is coded against an acknowledged baseline (or defaults),
  so it decodes on arrival. CONFIRMED.
- H16 (cite slip): plan.md:297 cites "ADR 0001:1613-1618" for "a lobby, real join tokens, reconnecting"; those lines are
  now PM failure routing; the scope text is ADR 0001:1640-1641.
- H17 (item 9, MAJOR, what the proof test measures): PM-0001-18's WebSocket part runs on L2 with LG's clients
  (advisor-u3q.md:20, E1 at :194-195: "L2 with LG on each transport ... through the accept path's delay wrapper"), and
  the wrapper sits ABOVE TLS, below tungstenite (U3-Q4 (b), checkpoint netcode-proof-test.md:338; E4 advisor-u3q.md:
  208-210; "the TLS handshake now runs undelayed above DelayStream", :158). So in the real run a reconnect's SYN and TLS
  flights cross the unimpaired LG-RS path even during the burst; only the upgrade, the 101, the Join and what follows
  pass the model. The harness loses the SYN and the TLS flights in the burst (fixes.rs:222-244 via TcpModel::connect).
  If DelayStream's new-connection model starts with RTO 1 s (S4: "1 s before the first sample"), the upgrade request
  lost in the burst is resent 1 s later (as the SYN is here) and then needs ~2.5 round trips instead of ~4.5: the L2
  reconnect is about 2 round trips (~200 ms at P2) faster than the modelled one. Direction: the bar set from fixes.rs
  is easier to pass in L2, and L2 does not show a real player's reconnect. Not measured (U3c not built). Sure: high on
  the placement (the plan's and the advisor's words), medium on the size.
- H18 (item 2/9, LG is not a browser): LG's clients are npt-net's Rust WsClient (plan.md:355, :533), which no endpoint
  lock or fail delay binds. Unless the plan requires LG's client to run the browser's policy (one attempt at a time,
  never closed before its 101, the newest Join wins), the L2 run measures a policy no browser runs (the parallel row is
  the reference for that: 637/820/1,507 on 6,000). plan.md:427 (E23) raises the per-address connection cap for LG's
  addresses; the new-session rate and the handshake cap (ADR 0001:445-450) are not mentioned there and H6 sends 48
  Joins from one address within about a second after every burst.
- H19 (minor, silence detection): last_data moves on any delivered byte of the current connection (fixes.rs:202-207),
  a partial message included; a page sees only whole messages (onmessage). A 1,050 B snapshot often spans two 1,448 B
  segments, so the model starts the silence clock slightly late. Direction: real false triggers slightly more frequent
  than the side-effects table; no effect under B1 (nothing arrives in the burst).
- H20 (pointer for the stats skeptic): the 480 client-bursts of a run share 10 burst instants (all 48 LG clients see
  the same burst, same tick phase); the pass chance takes them as independent (fixes.rs:646-649, :699). Correlation
  is the test-engineer's item; not judged here.

## ADR 0001 and plan conflicts of the planned recommendation (serial, never abandoned) vs round 1's list (finished block)
Round 1's list: u3-verify-fixes-adr.md "The ADR 0001 lines a docs PR changes". Status for the serial form:
- 422-423 WebTransport first with a 1.5 s timeout: STANDS (a WebSocket session must reconnect over WebSocket at once).
- 424 newest-only: STANDS (gain the adaptive budget's words).
- 429-432 one TCP port per room: RESOLVED (one endpoint; the two-endpoint row also fails in Firefox, whose queue keys
  the IP without the port: H4).
- 445-450 Join within 2 s from TCP accept, handshake cap, "a modest cap on connections per address (above 1)", new
  sessions rate-limited: STANDS and sharper: per player 2 connections during a reconnect (old + 1 attempt; the model
  briefly 3, H7); after EVERY burst one new session per player (H6); behind one address (LG, CGNAT, a LAN) N players
  make 2N connections and N Joins within ~1 s. The cap must count per player or scale with players; the new-session
  rate must allow a whole room's burst. Plan E23 (plan.md:427) raises only the connection cap for LG.
- 494-497 token expiry 30 s + 5 s leeway, unique id: STANDS: spares refreshed about every 25 s AND topped up at once
  after each use; under H6 each burst uses one spare per player (2 spares: a third outage inside one refresh window has
  none unless the top-up runs on use).
- 506-509 Reconnect: rewrite: spare held in advance; trigger max(300 ms, 2 x echo RTT + 100 ms) from "newest command
  received" (H9); one attempt at a time, never closed by the page; at most 8 per OUTAGE (H8); Join on it; the newest
  Join wins and the older connection is closed gracefully, its queued bytes still sent (H13: 50/60,000 phases need it);
  commands still read from the old connection until the new one's first command (H13). And the consequence to state:
  the room replaces a healthy connection after nearly every outage longer than the trigger (H6).
- 527-528 Why: a periodic lobby refresh per player in a match is new (2.4 calls/min per player; ~2/s per 48-player
  room; after a room-wide burst 48 top-ups at once).
- 616-617 "the Worker owns the connection": NEW wording point: "spare tokens in JS memory" means the Worker's memory.
- 668 Snapshot header: name the field the trigger's RTT reads ("newest command received").
- 670 Join "once" -> once per connection; 672 FullState also on a takeover (or a delta against the last acked
  baseline: H14); 682 Lobby calls: the spare refresh, sized per client and per address.
- 747 / 1295 / 1556 (E1-E3): the WebSocket half's limit; E1 must also say what L2 measures for reconnects (H17).
- 758 "A player drops": spare held in advance; 759 "The lobby is down": reconnects work until the spares expire
  (~25-35 s); a new failure row: false reconnects (0.4-0.6/min per player at P3/P4) and the post-burst takeover.
- 783 "the guest secret out of script's reach": spares sit in the Worker for at most 35 s: compatible if short-lived.
- 959-964 Q22: A's wording "a fresh token (fetched ahead and held as a spare)". If "bound to the session" means the
  live session, that is B's shape (a room-issued credential) and reopens B (see the next block).
- 1640-1641 and plan.md:297 (scope): STANDS: the proof test builds the reconnect for LG (as the browser's policy,
  H18) and the browser runner. plan.md:297's cite is stale (H16). Plan §10.3 (plan.md:1018-1027): bots reuse slot
  tokens and `Kick(replaced)` (fine for reconnects); browsers rejoin with a resume secret in page memory (works for
  reconnects too); the runner holds a single-use public token (plan.md:1027): no credential for a reconnect.
- RESUME-CRED (c') "a token fetched during the handshake": in a real outage the lobby is behind the same broken access
  link, so the fetch finishes only after the return plus its own TCP recovery, and the Join waits for it; the proof
  test has no lobby, so (c') cannot be measured there. A point for (a).

## "Bound to the session" in ADR 0001's token claims (finished block)
- Claims today (0001:494-497): player id, guest flag, display name, room id (audience), team or slot if pre-assigned,
  expiry 30 s + 5 s leeway, a unique id. No session id; the lobby never sees the room's sessions.
- Reading 1, "bound to player and room": already there (player id + audience). Nothing to change but the words.
- Reading 2, "bound to the live session": the room issues a session id in JoinAccepted (0001:671, new field); the
  Worker sends it with each spare request (0001:682, new parameter; the lobby cannot check it, it only signs it); a new
  claim `sid` in 494-497; the room refuses a Join whose `sid` is not that player's current or grace-window session.
  Gain: a spare leaked from one match is useless in a later session of the same player and room; it does NOT stop a
  hijack while the session lives (newest Join wins, 0001:507), which only expiry limits. Cost: one field, one claim,
  one check; the lobby signs a value it cannot verify (bounded by player id and audience anyway).
- Reading 3, a room-issued resume secret bound to the session: Q22 option B (0001:963), rejected in the ADR; the
  proof test already uses one for browsers (plan.md:1022-1023).
- Recommendation for the main session: ask Mustafa which reading he meant; Reading 2 is the concrete form of (a).

## Verdicts (finished block)
1. Endpoint lock as modelled (creation to 101, +10 ms, per IP and port): CONFIRMED for Chromium (H1; per IP, port and
   NAK); Firefox queues per IP without the port and holds until the upgrade's OK (H4); for the one-endpoint never-abort
   row the policy is stricter than the lock, so the lock never binds there.
2. ws.close() on a CONNECTING socket frees the lock: CONFIRMED in Chromium (H1). Firefox's fail delay not applying to
   page-closed attempts: REFUTED: the first page-closed attempt adds a 200-400 ms record; the exemption only stops it
   growing (H4); the abort rows lack it (abort 1 s: 370 over 1 s in Firefox vs 65: H12).
3. "Never abandoned" is the page's default: CONFIRMED (Chromium 240 s handshake, 4 min connect; Firefox 20 s open: H2,
   H4).
4. Policy: one attempt per endpoint until live or closed: CONFIRMED in effect; uses the room's Live state (H7), no
   effect on the numbers. 8-attempt cap: REFUTED as "per outage" (per run: H8). Cadence: moot in the never-abort row.
   Attempts from any current connection, others closed when the full state arrives: CONFIRMED (fixes.rs:251-268,
   :323-328).
5. Trigger from the input echo (EWMA 1/8) and a real client getting the same: CONFIRMED with the header field named
   (H9).
6. Room tick (snapshots and Joins on ticks, full state at once, no delta that tick): CONFIRMED as a reading the ADR
   allows but does not state (H14).
7. "Decodes deltas again" = first delta after the full state; old connection's fresh delta counts until the switch:
   CONFIRMED (H15).
8. Flights: ClientHello 1,800 B CONFIRMED (plausible); server flight 3,800 B with X25519MLKEM768: UNCLEAR (no provider
   chosen; ring gives X25519; resumption gives ~1 segment); pessimistic by 6 of 60,000 over 1 s (H10).
9. Side effects 0/0/0.37/0.6 false reconnects a minute: CONFIRMED within noise (P4 0.63 with the cap fixed: H8); but
   INCOMPLETE: the post-burst takeover after 99 % of B1 bursts is missing (H6). A false takeover costs more than a
   FullState (handshake, spare, inputs, replay entry) but no extra silence (H13).
10. The recommended numbers (435/659/1,025/1,363/1,685; pinned 437/665/1,318; pass chances): CONFIRMED as the model's
    output (H5). Too good where: the client OS retries its SYN after 3 s (H11: 438 over 1 s, max 2,879); Firefox for
    the abort rows (H12). Too bad where: TLS flight size (H10), FullState instead of a delta (H14). Not what L2
    measures (H17, H18).

## Bugs (finished block)
- H8: max_attempts per run, not per outage (fixes.rs:189, :325, :357 vs :68): side effects P4 18 -> 19 takeovers
  (0.60 -> 0.63/min); phases unaffected; a client built to it stops reconnecting after 8 outages.
- H7: the client's busy test reads the room's Live state (fixes.rs:330-332, :279): 415/60,000 phases start a second
  attempt; numbers unchanged.
- H6 (omission): the post-burst takeovers (59,403/60,000) are not reported anywhere.
- H12: abort rows model Chromium only (fixes.rs:339-342): Firefox abort 1 s 370 over 1 s (vs 65).
- H10: server flight assumes PQ and no resumption (fixes.rs:28-33): pessimistic, p99.9 1,025 -> 977, over 1 s 63 -> 57.
- H19: partial messages count as data (fixes.rs:202-207): false triggers slightly undercounted; nil under B1.
- H16: plan.md:297 stale ADR cite.

## Must correct before Mustafa decides (finished block)
1. H6: with "never abandoned", every B1 burst ends in a session takeover for nearly every client (59,403 of 60,000
   phases), after the old connection had already recovered: 480 per proof-test run, 48 at once from LG's address. Tell
   him; size RESUME-CRED (top-up on use) and ADR 0001:445-450 (per-player connection cap, new-session rate, handshake
   cap) for it; or offer "close the pending attempt once fresh data arrives on the old connection" (no takeover;
   Firefox then delays the next attempt by 200-400 ms within 60 s), not measured.
2. H17/H18: the proof test cannot measure the modelled reconnect: DelayStream sits above TLS, so the reconnect's SYN
   and TLS flights are never lost in L2 (about 2 round trips faster), and LG's Rust client is bound by no browser rule.
   The docs PR or plan must say what L2's WebSocket run measures, and require LG's client to run the browser's policy.
3. H11: fix 3's gain rests on the client OS resending a lost SYN after 1 s (Linux: yes). Windows unverified; at 3 s the
   recommendation equals fix 1 alone (438 over 1 s, max 2,879). Mustafa can check his laptop with Get-NetTCPSetting.
4. H8: the cap is per outage; correct the side-effects line for P4 (19, 0.63/min) or fix and re-run.
5. H12: label the abort rows "Chromium"; Firefox's numbers differ (abort 1 s: 370 over 1 s).
6. Name the RTT field (newest command received), the graceful close of the old connection and the command overlap
   (H9, H13) in the reconnect rule.
7. "Bound to the session": the ADR's claims have no session id; give him Reading 1 vs Reading 2 (above).
8. Updated conflict list: 429-432 resolved; 422-423, 424, 445-450, 494-497, 506-509, 616-617, 668-682, 758-759,
   959-964, 1640-1641 and plan.md:297 stand (block above).

Cleanup: worktree .gate/u3v2h-wt reset by reverse-applying its patch and removed (git worktree list shows only the
main folder and the model skeptic's .gate/u3v2-model). The diagnostic patch is kept at
scratchpad/u3v2h/diag.patch (520 lines, applies to df4afa1: counters in Outcome, client-visible busy, Firefox fail
delay, SYN timer knobs, flight knob, diag_harness/diag_gaps/diag_rtt/diag_flight/diag_winsyn/diag_abort/diag_post).
Nothing committed or pushed.

DONE
