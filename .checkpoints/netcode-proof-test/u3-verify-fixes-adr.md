# u3-verify-fixes-adr: skeptic of the ADR 0001 conflict analysis for the WebSocket fallback's three fixes

Branch claude/netcode-proof-test-plan-hcky5g, head 689c81f (41bd8a0 + checkpoint). No tracked file edited.

## Plan
1. Read Mustafa's request (checkpoint "Decisions taken", 2026-11-06 ~21:40) and the decisions the main session sent.
2. Read ADR 0001 parts 10 (admission, tokens, Q22), 11/12 (transport, line 422), 13 (key, 64 KB), line 424, 450, 507;
   ADR 0006 (kernel settings, socket options).
3. Read plan.md §10.3 (join tokens, roster).
4. Read fixes.rs for what fix 1/2/3 actually do (reconnect period, attempts in parallel, buffer sizes).
5. Verdict per claim (1)-(5); conflicts missed; exact lines a docs PR changes for WS-FIX (a) + RESUME-CRED (a).
6. DONE.

## Findings (a line each, as found)
- F0 (scope): the token, reconnect and roster rules the main session calls "part 10's admission rules" sit in ADR 0001
  part 11 (Rooms), "Join:" 0001:494-505 and "Reconnect:" 0001:506-509; part 10 (0001:419-473) holds the transport,
  hostile-input and per-address rules (422, 424, 445-453). Q21 0001:952-957, Q22 0001:959-964. Wording slip only.
- F1 (fix 1, the 64 KB): ADR 0001's only WebSocket 64 KB is IN ADR 0001 itself, the pass marks' Profiles bullet,
  0001:1462-1467 ("the write side bounded at 64 KB, returning Pending beyond"), the simulator spec, not part 13
  (0001:568-585, no 64 KB) and not only the plan's. 0001:443's 64 KB is QUIC stream windows (hostile-input limits),
  unrelated. That bullet's RTO rule (0001:1465-1466, "one retransmission timeout later: max(200 ms, 2 x RTT)") is
  already superseded by Q3 (b)'s Linux-faithful model, so the docs PR must rewrite 1462-1467 anyway (advisor's E4).
- F2 (WS-FIX (a) = the pinned recommendation, fixes.rs:334-357): TcpConfig::default() (no thin linear timeouts, rto_min
  200 ms) + adaptive budget + scaled reconnect. So fix 2 is NOT in the recommendation; claim (3) matters only if
  Mustafa picks a fix-2 variant. (Assumption: WS-FIX (a) is that row; the options' text is only in chat.)
- F3 (proof test's own tokens, plan.md:1014-1029): NOT ADR 0001's lobby tokens. Bots' tokens are reusable for their
  slot (plan:1018), and a Join on a slot with a live session replaces it (`Kick(replaced)`, plan:1018-1019);
  browsers bind at first use and rejoin with "a resume secret kept in page memory" (plan:1022-1023), i.e. Q22 option
  B's shape already, though only named "after a Worker trap"; the Safari runner holds "a single-use public token"
  (plan:1028), so a reconnect there has no credential unless it also gets the resume secret. So fix 3 has NO token
  conflict inside the proof test; the conflict is with the real game's rules (0001:494-508, Q22 0001:962-963).
- F4 (scope): reconnect is out of the proof test's scope: plan.md:297 ("a lobby, real join tokens, reconnecting ...")
  and ADR 0001:1640-1641 ("Part 11's lobby, join tokens, reconnect ... are slice 2 and slice 3 milestones; the proof
  test admits clients with test tokens"). If WS-LIMIT is set from the fix-3 numbers, PM-0001-18's WebSocket half
  only holds if the proof test builds the client reconnect (bots and the browser page): a scope change to plan:297
  and a note against 0001:1640-1641. The main session did not raise this.
- F5 (fix 1 vs 0001:424): fixes.rs:99 `pending` is one bool, so EVERY row, defaults included, already obeys 424
  ("only the newest snapshot ... drops any unsent older one"); fix 1 only adds WHEN the room counts the socket as
  backed up (socket bytes unacknowledged, fixes.rs:39-44, 180-185). 424 is silent on bytes already in the kernel
  queue; a docs PR should say it (claim (2) holds, with that wording addition).
- F6 (fix 1, ADR text at 1467): WS-FIX (a) keeps the 64 KB write bound (TcpConfig::default(), fixes.rs:336-341;
  tcp_model.rs:118-126) and the advisor's E4 words keep it ("The write side is bounded at 64 KB", advisor-u3q.md:214);
  only the fixed 3- or 8-snapshot budgets (fixes.rs:241-243) would change ADR 0001:1467 / E4. Code cite slip:
  tcp_model.rs:47 cites "ADR 0001:442" for the 64 KB write bound; 0001:442-443 is QUIC stream windows; the WebSocket
  bound is 0001:1467.
- F7 (fix 3's credential in the model): fixes.rs:18 and :23 model the Join as "a resume, with a token held in
  advance" (140 B, no lobby round trip). So the measured 399/611/738 ms assumes a credential already in hand; under
  0001:507 ("the client fetches a fresh token") fetched after the return, a lobby round trip (1 RTT on a live
  connection to the lobby, 2-3 on a new one) sits before the Join unless it runs in parallel with the room's
  TCP+TLS+upgrade (the model's 3 round trips before the Join, fixes.rs:131-153). Option (c) as worded ("each fetching
  a fresh token first") serialises it; a parallel fetch is a fourth option the decision lacks.
- F8 (diagnostic, worktree .gate/u3-adr-diag at 2dba6b4, a copy of `phase` with counters appended to fixes.rs; same
  1,000 phases; reproduces the pinned 399/610/737): in the RECOMMENDED config only 32 of 1,000 phases recover after
  a Join on a new connection; 968 recover on the OLD connection. Yet every phase starts attempts: by the first fresh
  snapshot mean 3.92, max 5 (histogram 1:9, 3:65, 4:916, 5:10). Up to 2 Joins are already sent by then (943 none,
  52 one, 5 two). Fix 3 alone: 889 of 1,000 recover on a reconnect.
- F9 (no cancel in the model): fixes.rs never closes losing attempts or the old connection, and every attempt sends
  its own Join as soon as upgraded (fixes.rs:151-153, 161-173). Run on for 2 s past the result: Joins sent and taken
  mean 3.91, max 5 per outage (recommended) — each would replace the session under 0001:507 (4 session switches, 4
  full states, 4 credentials) after the old connection had already recovered. Not in the measured number (it stops
  at the first fresh message), but the real client must (i) cancel attempts once data resumes, (ii) send Join on one
  connection only. RESUME-CRED (a)'s "spare tokens ... topped up as used" sizes nothing without that rule.
- F10 (server-side load per client, recommended): peak connections open at the server for one client mean 4.92,
  max 6 (old + 5 attempts); peak accepted-but-no-Join (handshakes in progress) max 3. A B1 that hits 48 clients at
  once (one LG address) gives up to ~288 connections and ~144 handshakes in progress in about a second.
- F11 (the omitted row): the main session's table leaves out "adaptive budget (1.5 round trips), no reconnect":
  p50 399, p99 645, max 1,645 ms; over 1 s 8, over 1.5 s 1 (ws_limit_table, same run). Fix 1 in its adaptive form
  ALONE meets "99 % well under 1 s" and misses "every recovery within 1.5 s" by 1 phase in 1,000. So fix 3, the
  only fix that conflicts with ADR 0001, buys the tail: p99 645 -> 610, max 1,645 -> 737, over 1 s 8 -> 0. Also
  omitted: "adaptive + thin linear timeouts" = identical 399/645/1,645 (thin LT never fires: ~5-6 segments out at
  P2, not thin, tcp.h:2119's < 4), and "fix 3 with an 8-snapshot budget" 405/593/658 (beats the pinned p99 and max,
  worse side effects at P3/P4: 10.5 %/26.9 % skipped, 11/53 false triggers per 3 min vs adaptive's 2/2).
- F12 (false reconnects): side_effects_table, adaptive with scaled trigger: 2 silences in 3 minutes at P3 and at P4
  (3 seeds x 60 s) = 0.67/min per client, as reported. For a 48-player room at P3: ~32 reconnects a minute, each a
  new TLS connection, a Join (one credential) and a full state, and under 0001:507 the healthy-but-slow connection
  closed.
- F13 (fix 2 moot under (a)): tcp_stream_is_thin is `packets_out < 4 && !tcp_in_initial_slowstart` (v6.8
  include/net/tcp.h:2118-2121; tcp_timer.c:640-648 applies linear timeouts only then). The adaptive budget at P2 is
  6 snapshots = 6,300 B = 5 segments (fixes.rs:39-44), so the stream is not thin and thin LT never fires: hence the
  identical 399/645/1,645 with and without it. Claim (3) stands but is academic for the recommendation.
- (checking next) RFC 6455 §4.1: at most one WebSocket connection in CONNECTING state per host:port; browsers'
  endpoint locks. If so, "parallel attempts every 300 ms" cannot run in a browser against one room port.
- SOURCE RFC 6455 §4.1 item 2 (rfc-editor.org/rfc/rfc6455.html, read 2026-11-06): "the client MUST wait until that
  connection has been established or for that connection to have failed. There MUST be no more than one connection
  in a CONNECTING state. If multiple connections to the same IP address are attempted simultaneously, the client
  MUST serialize them so that there is no more than one connection at a time running through the following steps."
  NOTE: it makes script DoS harder; "There is no limit to the number of established WebSocket connections". Sure:
  high (the spec's words). Whether browsers key it by IP or IP:port: next, Chromium's source.
- SOURCE Chromium net/socket/websocket_endpoint_lock_manager.{h,cc} (chromium.googlesource.com, main, read
  2026-11-06): implements RFC 6455 4.1.2; the lock map keys on (IPEndPoint, NetworkAnonymizationKey), i.e. IP and
  port; a locked endpoint's attempt waits (ERR_IO_PENDING) until granted; `kUnlockDelayInMs = 10` ("This delay
  prevents DoS attacks"). net/websockets/websocket_basic_handshake_stream.cc: the only UnlockEndpoint call is in
  `Upgrade()`, after the handshake is validated; OnFailure does not unlock (the EndpointLock destructor does, when
  the job or socket is destroyed). Sure: high that Chromium (so Brave, Mustafa's browser, and Edge) holds one
  CONNECTING WebSocket per IP:port until its handshake completes or the attempt is destroyed (I read the main
  branch, not a pinned release; the caller of Upgrade() was not read).
- SOURCE RFC 6455 §7.2.3 "Recovering from Abnormal Closure" (same page, read 2026-11-06): clients "SHOULD use some
  form of backoff when trying to reconnect after abnormal closures"; the first retry delayed by a random time (0 to
  5 s "a reasonable initial delay", clients may choose otherwise), then e.g. truncated binary exponential backoff,
  against many clients reconnecting at once. A SHOULD, about reconnects after a closure; fix 3 reconnects while
  the old connection is still open. Sure: high on the words; medium on how far they bind fix 3.
- SOURCE Firefox netwerk/protocol/websocket/WebSocketChannel.cpp (github mozilla-firefox/firefox main, read
  2026-11-06): nsWSAdmissionManager "1) Ensures that only one websocket at a time is CONNECTING to a given IP
  address (or hostname, if using proxy), per RFC 6455 Section 4.1. 2) Delays reconnects to IP/host after connection
  failure, per Section 7.2.3"; IndexOf compares mAddress and mOriginSuffix, NOT the port (so a second port does not
  help in Firefox); later ones are CONNECTING_QUEUED. FailDelayManager: after a failed connection, 200-400 ms random,
  x1.5 per repeat, max 60 s; "Don't count close() before connection as a network error"; pref
  network.websocket.delay-failed-reconnects. Sure: high on the queueing; medium on whether mAddress becomes the
  resolved IP (OnLookupComplete not read).
- F14 (MISSED, decisive for fix 3): the model's "attempt every 300 ms, in parallel" (fixes.rs:196-212) cannot run in
  Chromium (Brave, Edge) or Firefox against one room endpoint: a second `new WebSocket()` to the same IP:port
  (Chromium) or address (Firefox) is queued until the first finishes its handshake or is destroyed; during the
  outage the first's SYN is lost, so the rest wait on the OS's SYN retries. The browser can only abort-and-retry
  (close() the pending attempt, then open the next: one at a time), and JS cannot tell a lost SYN from a handshake
  in progress (WebSocket exposes no TCP-connected event), so a 300 ms cadence aborts handshakes that take ~3 round
  trips (~300 ms at P2, ~600 ms at P4). Parallel attempts need distinct endpoints: a second TCP port (Chromium only)
  or the VM's IPv4 and IPv6 addresses (both browsers, ADR 0006:225), against ADR 0001:429-432's "one TCP port for the
  WebSocket fallback" per room. Measuring the serial variant next.
- F15 (diagnostic 2, serial abort-and-retry = what Chromium/Firefox allow; same 1,000 phases, worktree copy):
  recommended config with each new attempt killing the earlier unfinished ones:
    every 300 ms (the pinned cadence): p50 399, p99 629, max 1,645 ms; over 1 s 8, over 1.5 s 1 (= NO tail gain over
      the adaptive budget alone, 399/645/1,645);
    every 500 ms: 399/632/878, 0 over 1 s; every 700 ms: 399/576/1,145, 2 over 1 s; every 1,000 ms: 399/632/930.
  fix 3 alone serial 300 ms: 652/852/1,845 (3 over 1.5 s); serial 700 ms: 460/828/1,160.
  So the pinned 399/611/738 is a parallel result no browser runs against one endpoint; the browser-runnable variant
  needs a cadence above the ~3-round-trip handshake (500 ms at P2 meets Mustafa's goal in this model: max 878 ms).
  A serial client also holds at most one CONNECTING attempt, so fewer server connections and one Join at a time.
  Caveat: my kill drops the attempt at once; a real abort leaves the server an accepted socket until the FIN/RST
  arrives or the 2 s Join deadline (0001:445). Not pinned, not committed: a probe only.
- F16 (the trigger's RTT): trigger() reads the client's TCP srtt (fixes.rs:46-51, `srtt_ns(1)`, "what TCP_INFO's
  tcpi_rtt reports", tcp_model.rs:642-647). A page cannot read TCP_INFO for a WebSocket; it has the app's ping RTT
  (ADR 0001 part 7's clock sync), which includes head-of-line waits. The server side's adaptive budget may use
  TCP_INFO/SIOCOUTQ (Linux); the client side may not. Effect on the numbers: unknown (direction: a longer trigger).
- F17 (reconnect vs WebTransport-first, MISSED): 0001:422-423 "WebTransport is tried first with a timeout (proposed
  1.5 s), then WebSocket" draws no line between a first connect and a reconnect; read literally, a fallback session's
  reconnect spends up to 1.5 s on WebTransport before any WebSocket attempt, so no fix-3 number holds. The docs PR
  must say a WebSocket session reconnects over WebSocket at once.
- F18 (catalogue and rows, MISSED): `Join` "once, within 2 s" (0001:670), `FullState` "at join and on resync"
  (0001:672), Lobby calls "per visit or room ... token request" (0001:682), "A player drops | ... fresh token; newest
  connection wins" (0001:758), "The lobby is down | no new joins; running rooms go on" (0001:759) all assume a
  fetch-then-join reconnect; RESUME-CRED (a) changes 682 (a refresh per player about every 25 s: 30 s expiry,
  0001:496) and 758, and a reconnect's first message is a FullState (fixes.rs:164-169, RESUME), not a delta, so
  E1's "decodes deltas again" (PM-0001-18, 0001:1556) must say whether a resumed session's FullState counts.
- F19 (RESUME-CRED (a) vs expiry): "topped up as used" is not enough: a spare expires 30 s after issue (+5 s leeway,
  0001:496), so the client must refetch about every 25 s used or not (about 2 lobby calls a minute per player; ~1.6/s
  for a 48-player room), or the ADR gives spares a longer expiry (a change to 496 and to "short-lived", 0001:494-495).
  Single use (0001:496 "a unique id"; R2-ROOM-F10 "usable for exactly one connection", r2:1479-1486), audience
  (room id, 0001:495-496) and the replay cache (0001:503-504) all hold if each Join presents its own spare.
- F20 (claim (1) as framed): no ADR 0001 rule says WHEN the fresh token is fetched (0001:507; R2-ROOM-REC4,
  r2:2812-2815, "the client fetches a new token for the same room"), so a token fetched ahead (a) or alongside the
  handshake (c') breaks no token rule; Q22 B's rejection (0001:963) bites only option (b). The real token-side
  tensions: 507 newest-wins against fix 3's first-wins (resolved only by one Join at a time) and 496's expiry.
- SOURCE WHATWG WebSockets Standard (websockets.spec.whatwg.org, read 2026-11-06): handlers onopen, onerror,
  onclose, onmessage only; `open` fires "When the WebSocket connection is established" (after the handshake);
  readyState CONNECTING = "The connection has not yet been established" (no sub-stages); Ping/Pong "not currently
  exposed in the API"; failure causes hidden from scripts (close code 1006). Sure: high. So a page cannot tell a lost
  SYN from a handshake in progress, nor read TCP's RTT (backs F14 and F16).
- F21 (diagnostic 3, deltas again): counting only a delta (a resumed session's FullState not counted, as E1's
  "decodes deltas again" reads): recommended parallel 399/611/751 ms (max +14); serial 500 ms 399/632/904 (max +26).
  Small, but the pinned maximum is not E1's measure as written.
- F22 (anti-cheat line): 0001:782-784 "the guest secret out of script's reach". A spare token (a) sits in script for
  at most its 35 s; a room-issued resume secret (b), if reusable for the match, is a longer-lived bearer credential
  in script (as plan:1022-1023's proof-test secret is). A point against (b), and for short-lived spares.

## Verdicts (finished block)
1. Fix 3 conflicts with the token rules (single use, replay cache, fresh token on reconnect, Q22 B rejected):
   REFUTED as framed. No rule says when the fresh token is fetched (0001:507; r2:2812-2815); a token fetched ahead or
   alongside the handshake keeps single use, audience and the replay cache (F19, F20). What does bite: 496's 30 s
   expiry (a refresh about every 25 s), 507's newest-wins against "whichever connects first" (one Join at a time,
   cancel the rest: F9), and conflicts the analysis missed (F4, F14, F17, F18). The model itself assumes "a token
   held in advance" (fixes.rs:18).
2. Fix 1 does not conflict; 424 is its own rule; the 64 KB is the plan's: CONFIRMED with corrections: the 64 KB is
   ADR 0001's own text at 1467 (the pass marks' Profiles key, advisor's E4 keeps it), a simulator bound, kept by WS-FIX
   (a) (F1, F6); every row incl. the defaults already obeys 424 (F5); 424 should gain the budget's wording.
3. Fix 2 does not conflict with ADR 0001 (or 0006): CONFIRMED (no kernel or socket-option rule in either; the
   per-socket TCP_THIN_LINEAR_TIMEOUTS needs no privilege, tcp.c:3530-3534; rto_min is per route in v6.8,
   tcp.h:741-748, root at VM setup, every TCP flow on the route), and moot: not in WS-FIX (a), and thin LT never fires
   under the adaptive budget (F13).
4. RESUME-CRED options: (a) holds single use, audience, replay cache; fails "topped up as used" against the 30 s
   expiry (F19); needs one Join at a time (F9); per-address and new-session limits must be sized (no number to break,
   0001:447-450). (b) reopens Q22 B (0001:963); the proof test already runs one (plan:1022-1023); against 0001:783's
   spirit if long-lived (F22). (c) ADR-literal, and browsers force one attempt at a time anyway (F14); its "fetch
   first" is avoidable by fetching alongside the handshake (c'), not offered and not measured.
5. Missed: F4 (proof test scope), F14 (browsers serialise; one TCP port per room), F17 (WebTransport-first on
   reconnect), F18 (catalogue rows, failure rows, E1's deltas), F11 (the omitted adaptive-alone row), F3 (the
   runner's single-use token). ADR 0006: none (no kernel/socket rule; 0006:225's dual-stack address is one way out
   of F14).

## The ADR 0001 lines a docs PR changes for WS-FIX (a) + RESUME-CRED (a) (finished block)
Beside the advisor's E1-E4 (1556, 747, 1295, 1462-1469):
- 422-423: a reconnect stays on the session's transport (a WebSocket session reconnects over WebSocket at once);
  the old and the new connection overlap only until the Join replaces the old.
- 424: the newest-only rule gains its budget: the room writes a snapshot only while the socket's unacknowledged
  bytes fit about 1.5 round trips of snapshots (at least 3), from the socket's own RTT and queue.
- 429-432: only if parallel attempts are kept: a second endpoint per room (port, or the IPv6 address); unchanged for
  a serial client.
- 445-450: the per-address cap, the new-session rate limit and the handshake cap sized for reconnects (a client's
  2-3 attempts a second, players sharing an address, a whole room at once).
- 494-497: the spare token's life: 30 s with a refresh about every 25 s, or a longer expiry for spares.
- 506-509: *Reconnect:* the client holds a spare token; after max(300 ms, 2 x RTT + 100 ms) of silence it opens a
  new connection (cadence as decided), sends Join on one connection at a time, drops the rest once data resumes;
  newest Join wins (unchanged).
- 527-528 (*Why*): the lobby stays off the join path; a periodic refresh per player in a match is new.
- 670 `Join` "once" -> once per connection; 672 `FullState` also on a reconnect's Join; 682 Lobby calls add the
  spare-token refresh and size its per-client and per-address limits.
- 758 "A player drops": spare token held in advance; 759 "The lobby is down": reconnects work until the spare
  expires; a new row for false reconnect triggers (about 0.7 a minute per client at P3/P4).
- 962 Q22 A: "rejoin with a fresh token (fetched ahead and held as a spare)".
- 1640-1641: the proof test builds the WebSocket reconnect (test tokens: the bots' slot tokens, the browsers' resume
  secret, plan §10.3), if WS-LIMIT is measured with it. Plan: 297 (scope) and §10.3 1022-1023, 1028.

## Must correct before Mustafa decides (finished block)
1. Fix 3 as pinned (parallel attempts) is not what Brave, Edge or Firefox can run against one room endpoint (F14,
   RFC 6455 §4.1, Chromium and Firefox sources); the serial version at the pinned 300 ms cadence gives no tail gain
   (399/629/1,645, 1 over 1.5 s); at 500 ms: 399/632/878 (my probe, not pinned). WS-FIX and WS-LIMIT need the
   browser-runnable variant measured and pinned.
2. Send the omitted row (F11): the adaptive budget alone (no ADR conflict) is 399/645/1,645; fix 3 buys only the tail.
3. Correct claim (1) (F20): the token rules don't forbid a spare; the conflicts are 507 (one Join), 496 (expiry),
   422-423 (WebTransport-first), 429-432 (one port, if parallel), 1640-1641 and plan:297 (scope).
4. RESUME-CRED (a) must say "refreshed about every 25 s" (F19) and "one Join at a time, the rest cancelled" (F9);
   add (c'): fetch the token alongside the handshake.
5. Claim (2)'s "the plan's own number": the 64 KB is ADR 0001:1467 (F1). Code cite slip tcp_model.rs:47.
6. E1's "deltas again" vs a FullState after a reconnect: max +14 ms parallel, +26 ms serial (F21).
Worktree .gate/u3-adr-diag removed (its file reset to HEAD first); probes kept in the scratchpad (adr_diag.rs,
serial_diag.rs, patch_delta.py); build output in .gate/u3-adr-diag-target; nothing committed or pushed.

DONE
