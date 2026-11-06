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
