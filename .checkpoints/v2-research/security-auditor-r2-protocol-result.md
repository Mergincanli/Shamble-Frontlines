# security-auditor on R2 protocol: returned report (verbatim), workflow run wf_bce24fc0-5c6, 2026-10-28

**Security and anti-cheat review of R2's protocol (docs/research/r2-netcode.md). Verdict: REVISE.**

Nothing in the design is decided by the client: commands carry inputs only. Hits, damage, economy, team and slot assignment, and the start, end and results of a match are all on the server. So there is no Critical finding. Four High gaps need to be written into the recommendations before the decision drafts (ADRs). I checked 28 claims against their sources: 24 are confirmed, 4 are confirmed with a caveat or partly unsupported, none is wrong. One recommendation (where the guest secret is stored) goes against the source it relies on. Every quote is in the notes file.

Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/security-auditor-r2-protocol.md (ends with DONE). No agent memory folder exists, and rule 1 lets me write only my notes, so the threat model is kept in that file.

## Findings (by severity)

**H1. High: the lag-compensation window lets any client claim the full rewind ("backtrack").**
- What: the server accepts the client's view time if it is within ±200 ms of its estimate. The estimate is round trip plus the client's own interpolation delay. That delay is adaptive on the client (2 to 5 intervals), and the cap is 250 ms.
- Exploit: a 20 ms player (estimate about 120 ms) claims a 250 ms view time, which passes the window and the cap. That is 130 ms of extra rewind, about 1.2 m at sprint speed. The torso is 1.04 m wide, so they can hit targets who already reached cover.
- Evidence: r2-netcode.md:1925-1929, 1890-1892, 2089-2091.
  - Source does the same. Its delay comes from client settings, `float flLerpAmount = Q_atof( QUICKGETCVARVALUE("cl_interp") );` and `this->m_fLerpTime = MAX( flLerpAmount, flLerpRatio / this->m_nUpdateRate );` (player.cpp).
  - The window is a tolerance: `if ( fabs( deltaTime ) > 0.2f )` (player_lagcompensation.cpp).
- Fix:
  - Shrink the tolerance to at most one snapshot interval.
  - Clamp the stated delay to the mode's data range and let it change only slowly.
  - Take round trip from the server's own measurement (quinn `rtt()`, lowest recent value), never from app-level pings alone.
  - Prefer the 200 ms cap (cross-unit question XU-OQ1).
  - Log each player's view-time offset so cheats can be detected.
  - Pass mark: a scripted cheat client gains no more than the tolerance over an honest client at the same round trip.

**H2. High (if a shared seed is chosen): no-spread aim.**
- What: question SYNC-OQ11 (shared spread seed) is left open. If the client knows the seed, a modified client can compute each shot's spread and shift the world aim point it sends to cancel it. The result is a zero-spread AMK at 300 m.
- Evidence: r2-netcode.md:2529-2531, 1997-2016. Source's own help text says its server seed is "independent of the client" (player.cpp, `sv_usercmd_custom_random_seed`). That settles what SYNC-F20 had only inferred.
- Fix (my answer to SYNC-OQ11): authoritative spread comes from a server-only keyed random stream (server secret, tick, shooter). Client tracers are cosmetic and get corrected by the hit event.

**H3. High: no limits on network input.**
- What: none of TRN-REC1 to REC6 or ROOM-REC3 sets any of these:
  - message or datagram rate per connection;
  - size of incoming datagrams and of reliable messages;
  - streams per session;
  - connections per IP, and a cap on connections not yet signed in;
  - a deadline for the Join message;
  - an idle timeout.
- Exploit: quinn's defaults are `max_concurrent_bidi_streams: 100`, `max_concurrent_uni_streams: 100`, `stream_receive_window` 1.25 MB, `receive_window: VarInt::MAX`, and a 1.25 MB datagram buffer (quinn main branch; 0.11.12's values not checked). One client can therefore push about 250 MB per connection. A slow-loris client opens sessions and never sends Join.
- Evidence: r2-netcode.md:2250-2381, 2411-2423. The draft says "Servers SHOULD limit the rate of incoming WebTransport sessions" (§5.2) and "An endpoint that does not monitor use of these features exposes itself to a risk of denial-of-service attack." (§8).
- Fix:
  - Allow 2 client streams in each direction, windows of about 64 KB per stream and 128 KB per connection, and a datagram buffer of about 16 KB.
  - Reject any input datagram larger than the command size times the redundancy cap.
  - Rate-limit datagrams and messages per connection.
  - Require Join within about 2 s; until then allow one stream and drop datagrams.
  - Cap connections per IP modestly (NAT means not 1).
  - Use quinn `Incoming::retry`/`refuse` under load, and `H3_EXCESSIVE_LOAD` for abusers.
  - Add a flood test as a pass mark.

**H4. High: the WebSocket fallback is weaker than WebTransport.**
- What: TRN-REC3 has no Origin check, no wss-only rule and no message or frame size limit.
- Exploit: tungstenite 0.30 defaults to "64 MiB" per message, "16 MiB" per frame and an "unlimited" write buffer. One message is already over the 64 MB room memory ceiling ROOM-REC2 proposes, so a few clients can make a room run out of memory.
- Over WebSocket the server's round-trip estimate comes from app-level pings, which a client can delay selectively. That feeds H1.
- Evidence: r2-netcode.md:2296-2317, 2409. RFC 6455:
  - §10.2: "it SHOULD respond to the WebSocket handshake with a reply containing HTTP 403 Forbidden status code."
  - §10.4: "SHOULD impose a limit on frame sizes".
  - §10.6: "(wss URIs)".
- Fix: the same Origin allow-list, wss only, and the H3 size, rate and deadline limits on both transports, with one test suite run against both.

**M1. Medium: the WebTransport Origin check is sourced (TRN-F36) but appears in no recommendation.**
- Exploit: a third-party site hosts a modified client (overlay or phishing) that connects to our rooms from players' browsers.
- Evidence: r2-netcode.md:1183-1190. wtransport's `origin()` "Returns the `origin` field of the request if present", so the check is our own code. The draft's rule only applies "When the request contains the `Origin` header".
- Fix: check against an allow-list and answer 403 (`forbidden()`). Treat a missing Origin as a non-browser client: tag it for detection, because it cannot be blocked.

**M2. Medium: the list of command checks is incomplete.**
- The pass mark at r2-netcode.md:1854 only covers future ticks, aim range and the command cap. Missing:
  - a limit on move-vector length;
  - a limit on press-counter jumps (at most one edge per button per tick, line 1839);
  - the server re-checking cooldown, ammo and reload on every fire or ability edge;
  - map bounds for the aim point;
  - acks only for snapshots actually sent to that client (line 1810);
  - monotonic sequences that handle wrap-around;
  - a bound on how far in the future a tick may be;
  - rejecting trailing bytes or unknown fields.
- Exploit: a press counter raised by 50 in one command fires 50 queued ability or bow edges.
- Evidence: Quake III clamps on the server, "// sanity check the command time to prevent speedup cheating" `ucmd->serverTime > level.time + 200` (g_active.c), which R2 did not cite.
- Fix: write a validation table into SYNC-REC2 and add hostile cases to the pass mark.

**M3. Medium: lag switch.**
- What: when the server runs commands (SYNC-OQ3) is open, and Source's catch-up allowance is 24 ticks (`sv_maxusrcmdprocessticks` "24").
- Exploit: hold inputs for 200 ms, then send them in one burst; other players see the cheater jump about 1.8 m.
- Evidence: r2-netcode.md:1845-1846, 2501-2505.
- Fix: run one command per client per tick from a buffer, repeat the last command on a miss, drop late ones, and limit catch-up to the redundancy window.

**M4. Medium: the queue of reliable events per client has no bound.**
- Exploit: a client never acks "gameplay events until acked" (r2-netcode.md:2063), so the server keeps and resends every event for the whole match.
- Fix: cap the queue per client; on overflow force a full resync or disconnect.

**M5. Medium: projectile, tracer and shot-event origins reveal hidden shooters.**
- What: a trajectory record carries "start position" (r2-netcode.md:1952), and "shots at or hitting the viewer are always sent" (2221).
- Exploit: a client that logs everything it receives reads the throw position of enemies inside buildings.
- Fix: re-base a trajectory at the tick and position where it becomes relevant, clip events from shooters who are not relevant, and add this case to INT-REC2's reader-client test.

**M6. Medium: the guest secret is stored in localStorage.**
- What: ROOM-REC8 stores the secret in "the KVStore successor" (r2-netcode.md:2469).
- Exploit: any XSS on the game origin (the content security policy is deferred) takes over the guest identity for good.
- Evidence: OWASP: "Do not store authentication tokens, session IDs, JWTs, refresh tokens, or any credential in `localStorage`".
- Fix: a cookie on the lobby origin (HttpOnly, Secure, SameSite=Strict), with an Origin check on the lobby's token endpoint. Or accept the risk in writing until accounts (VISION_V2 §6.10).

**M7. Medium: room codes can be brute-forced.**
- What: lookups are limited "10 per minute per client" (r2-netcode.md:2445), but guests are free and any guest can get a join token, so the code is in practice the access secret.
- Exploit: many guests on many IPs scan the 32^6 (about 2^30) code space and join private rooms.
- Fix: limits per IP, per guest and globally; limit how fast guests can be created; default the host lock to on for organized play; and either 8 or more symbols or tokens for private rooms issued only on host approval. This also answers ROOM-OQ10.

**M8. Medium: the token signature scheme is open (ROOM-OQ2).**
- Exploit: with a shared symmetric key (the netcode.io model, "shared private key between the web backend and the dedicated server instances"), one compromised room process can mint tokens for any player, room or slot.
- Fix:
  - An asymmetric signature: the lobby signs; rooms hold only the public key.
  - One pinned algorithm, a typed token and a key id for rotation (RFC 8725: "Libraries MUST enable the caller to specify a supported set of algorithms"; §3.11 "Explicit typing is RECOMMENDED"; RFC 7519 §6 allows unsecured tokens).
  - Verify the signature before touching the replay cache. Bound the cache: netcode says "A full history refuses new connect tokens, and never evicts an unexpired entry to make room".
  - Compare any MAC in constant time.
  - The room re-validates any pre-assigned team or slot against the mode.

**M9. Medium: a panic in the decoder kills the room.**
- Exploit: one malformed datagram hits an unchecked index or unwrap, the room process dies, and 47 players are disconnected, again every match. Process-per-room (r2-netcode.md:2397-2410) only limits this to one room.
- Fix: decoding that returns errors and never panics, sizes bounded before allocating, a fuzz pass mark (millions of messages, 0 panics), and a written invariant for any `unsafe`.

**M10. Medium: follow-mode spectating gives live enemy positions.**
- What: a follow spectator "receives that player's interest" (r2-netcode.md:2236), with no rule on whom they may follow.
- Exploit: a dead player follows an enemy and calls out positions to their team.
- Fix: players in the match follow only their own team live; following an enemy uses the delayed stream only.

**L1. Low: client-side aim help.**
"gentle aim help ... shapes this point on the client" (r2-netcode.md:2001), so the server cannot tell aim help from an aimbot. If aim help exists, apply it on the server.

**L2. Low (accepted trade-off): the relevance circle covers more than the screen.**
The region is a full circle around the player (r2-netcode.md:2124-2131), so a map hack sees behind the camera. Keep as an accepted risk, with option A4 (shift toward the aim) as a later reduction.

**L3. Low: logs.**
The pass mark checks only that no token appears in access logs. Extend it to guest secrets, Join payloads and raw messages, and log salted hashes (OWASP: "log a salted-hash of the session ID").

**L4. Low: display names.**
The token carries a display name. The lobby should limit its length and characters, and clients must render it with textContent only.

Detection telemetry is part of R6's "anti-cheat basics", which is deferred, so I did not research it. The only ask for R2: keep the view-time offset and the aim data per command in the protocol so detection can use them later.

## Claim table

| Id | Claim (short) | Source | Verdict | Location |
|---|---|---|---|---|
| C1 | WebTransport server MUST verify Origin; 403; H3_EXCESSIVE_LOAD (TRN-F36) | draft-ietf-webtrans-http3-16 §3.2, §8 | Confirmed: "the WebTransport server MUST verify the `Origin` header to ensure that the specified origin is allowed" | r2:1183 |
| C2 | Servers SHOULD rate-limit new sessions (ROOM-F13) | same, §5.2 | Confirmed: "Servers SHOULD limit the rate of incoming WebTransport sessions on HTTP/3 connections" | r2:1351 |
| C3 | Pinned-certificate rules (TRN-F13) | W3C WebTransport CR §6.9, §14.1 | Confirmed, including SHA-256 and "no interstitial allowing bypassing certificate validation", which R2 had marked inferred | r2:956 |
| C4 | Pinning is for non-public hosts; hashes from a trusted intermediary (TRN-F14) | MDN WebTransport() | Confirmed: "The web application might typically fetch the hashes from a trusted intermediary." | r2:968 |
| C5 | wtransport self-signed: P-256, at most 2 weeks (TRN-F15) | docs.rs wtransport 0.7.2 Identity | Confirmed: "This function uses the `ECDSA P-256` algorithm." | r2:978 |
| C6 | Secure contexts only (TRN-F35) | MDN WebTransport API; W3C IDL | Confirmed: "This feature is available only in secure contexts (HTTPS)" | r2:1177 |
| C7 | An oversized datagram is silently not sent (TRN-F18) | W3C §4.3 | Confirmed: "return a promise resolved with undefined" | r2:1010 |
| C8 | requireUnreliable forbids the HTTP/2 fallback (TRN-F12) | W3C §6.9 | Confirmed: "cannot be established over an HTTP/2 connection if an HTTP/3 connection is not possible." | r2:948 |
| C9 | Short-lived, single-use connect tokens; one client id at a time (ROOM-F9) | Fiedler, 2017-02-26 | Confirmed: "This prevents one connect token from being used to connect multiple clients." | r2:1319 |
| C10 | netcode 1.02: single use; expired or other-server tokens ignored (ROOM-F10) | mas-bandwidth STANDARD.md | Confirmed: "A connect token is therefore usable for exactly one connection" | r2:1328 |
| C11 | JWT audience, expiry and leeway, unique id (ROOM-F11) | RFC 7519 §4.1.3, §4.1.4, §4.1.7 | Confirmed: "The "jti" claim can be used to prevent the JWT from being replayed." | r2:1335 |
| C12 | WebSocket and WebTransport constructors take no auth header (ROOM-F12) | MDN constructors | Constructor signatures confirmed ("new WebSocket(url, protocols)"). "No auth header" is unsupported: only the absence of any header option, which R2 already marks inferred | r2:1343 |
| C13 | At least 64 bits of entropy; IDs in URLs leak; no IDs in logs (ROOM-F14) | OWASP Session Management | Confirmed: "Session identifiers must have at least 64 bits of entropy" | r2:1359 |
| C14 | Custom IDs that are too simple let attackers take accounts (ROOM-F16) | PlayFab login best practices | Confirmed: "If they're too simple, you risk hackers stealing the accounts of others." | r2:1377 |
| C15 | Source accepts the client's tick within 200 ms; clamped by sv_maxunlag (SYNC-F28) | player_lagcompensation.cpp | Confirmed: `if ( fabs( deltaTime ) > 0.2f )`. The exploit reading is missing in R2 (H1) | r2:311 |
| C16 | Speed-hack guard on replayed command ticks (SYNC-F33) | player.cpp | Confirmed: "// still make sure to avoid speedhax". The default R2 left unread is "24" | r2:356 |
| C17 | The server ignores input older than its current time (SYNC-F16) | Fiedler, 2004 | Confirmed: "ignores any input received with a time value less than the current time" | r2:206 |
| C18 | Client seed plus a server-only seed (SYNC-F20) | usercmd.h; player.cpp | Confirmed: "// Only the server populates this seed". Its purpose, which R2 inferred, is now sourced: "independent of the client" | r2:245 |
| C19 | Source rewinds only entities sent to and acknowledged by the shooter (SYNC-F32) | player.cpp | Confirmed: "If this entity hasn't been transmitted to us and acked, then don't bother lag compensating it." | r2:344 |
| C20 | DevTools reads JS and Wasm memory (INT-F16) | developer.chrome.com memory inspector | Confirmed: "The Memory Inspector panel lets you inspect such objects in Wasm applications written in C++." | r2:760 |
| C21 | Quake III culls entities by area connection and PVS (INT-F1) | sv_snapshot.c | Confirmed: `continue;		// blocked by a door` | r2:646 |
| C22 | A network field limited to team members (INT-F11) | sendproxy.cpp | Confirmed: "// Purpose: Proxy that only sends data to team members" | r2:721 |
| C23 | TF2 disguise fields are in the shared table (INT-F12) | tf_player_shared.cpp | The table lines are confirmed. That the table reaches every client is unsupported (paraphrase only), as R2 says | r2:726 |
| C24 | SourceTV runs 30 s behind (INT-F13) | hltvdirector.cpp | Confirmed: `tv_delay( "tv_delay", "30"` | r2:735 |
| C25 | panic = "abort" terminates the process (ROOM-F8) | Cargo profiles | Confirmed: "Terminate the process upon panic." The std::thread half was not re-read | r2:1308 |
| C26 | GameLift validates on connect; 60 s timeout (ROOM-F2) | API_PlayerSession | Confirmed: "not validated within the timeout limit (60 seconds)" | r2:1253 |
| C27 | Quake III command time "without allowing cheating" (SYNC-F19) | cl_input.c; g_active.c | The text is confirmed, but the claim is incomplete: the protection is the server clamp in g_active.c, "// sanity check the command time to prevent speedup cheating", which R2 did not cite | r2:236 |
| C28 | "low-latency" is a feature at risk (TRN-F10) | W3C §6.9 | Unsupported (paraphrase only): the fetch puts the at-risk note on the congestionControl option. Not security-relevant | r2:927 |
| Rec | Guest secret kept in the KVStore successor (ROOM-REC8) | OWASP | Contradicted: "Do not store ... any credential in `localStorage`" (M6) | r2:2469 |

## Anti-cheat coverage

| Player action | Where the server validates it | Gap |
|---|---|---|
| Move | The server runs every command through the Rust movement code (SYNC-REC2, REC3) | No limit on move length; burst catch-up (M2, M3) |
| Aim | The server rebuilds the ray from its own fire origin; range check (REC8) | Client-side aim help (L1); aim bounds (M2) |
| Hitscan fire | The server resolves against the hit history; targets must be relevant and acknowledged (REC5) | View-time window (H1); spread seed (H2); cooldown and ammo re-check not stated (M2) |
| Throw or projectile | The server simulates and sweeps it (REC6) | Trajectory origin leaks the thrower (M5); spawn fast-forward (SYNC-OQ6) would reuse H1's lever |
| Reload, ability, weapon switch | Press counters inside commands (REC2) | Counter jumps not limited (M2) |
| Melee | Open (SYNC-OQ9) | H1 applies if judged at the attacker's view time |
| Vehicle | The pilot's commands (REC7) | Seat entry not specified (SYNC-OQ7) |
| Purchases, credits | Server-side by VISION_V2 §6.5 | No message yet; must join the validated message list |
| Join, team, slot | Token; the room assigns team and slot (ROOM-REC3) | Token scheme (M8); Join deadline and limits (H3) |
| Reconnect | Fresh token; newest connection wins (ROOM-REC4) | Guest secret storage (M6) |
| Private room code | Rate-limited per client (ROOM-REC5) | Sybil scanning (M7) |
| Host commands | Checked against the host's id (ROOM-REC5) | None |
| Snapshot and event acks | Delta baseline (REC1, BUD-REC2) | Acks not validated (M2); unbounded event queue (M4) |
| Spectating | Delayed free camera (INT-REC7) | Live follow of enemies (M10) |
| What a player sees | Area/portal hiding, visibility classes per field, reader-client and byte-audit tests (INT-REC2, REC5) | Event origins (M5); circle larger than the view (L2) |
| Transport | Certificates (TRN-REC5); never block on send; only the newest snapshot over WebSocket (REC3, REC6) | Origin (M1); limits (H3); weaker WebSocket fallback (H4); panic (M9) |
| Match results | Written by the room at Ended (ROOM-REC1) | None |

## Verdict: REVISE

Fix H1–H4 and M1–M10 in the R2 recommendations (or defer them explicitly) before the ADR drafts. The authority model itself is sound and needs no change.

## Decisions needed
- **SEC-SPREAD (spread randomness):** should the client know the seed that decides weapon spread? Why it matters: if it does, cheats get perfect accuracy. Options: (a) server-only seed, with cosmetic tracers corrected by the hit event; (b) shared seed, which gives matching tracers but allows no-spread cheats. Recommendation: (a).
- **SEC-REWIND (lag-compensation tolerance and cap):** how much lag compensation may any client claim? Why it matters: it bounds how far behind cover a target can still be hit. Options: (a) a 200 ms cap with a tolerance of one snapshot interval, using the server's own round-trip measurement; (b) R2 as written (±200 ms tolerance, 250 ms cap). Recommendation: (a).
- **SEC-GUEST (where the guest secret lives):** Why it matters: in localStorage, any XSS steals the identity for good. Options: (a) a cookie on the lobby (HttpOnly, Secure, SameSite=Strict); (b) localStorage, with the risk accepted in writing until accounts exist. Recommendation: (a).

## For Mustafa
Nothing.