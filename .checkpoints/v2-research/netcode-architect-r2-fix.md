Task: R2 netcode wave 1 step 4 (Fix): apply verifier, security-auditor and rust-reviewer results to docs/research/r2-netcode.md

## Progress
- 2026-10-28: read plan, amendments, the three reviewer reports, topic file parts 3-6 (recs, options, OQs). Fix list drafted in my head; writing it down next.

## Fix list (planned)
A. Unsupported basis: SYNC-F34 (SYNC-REC5 reason + mark 2); ROOM-F17 "owner alone" (ROOM-REC5 "sole write rights", options 3.4).
B. Verifier other: BUD-REC4/REC5 getStats (TRN-F9) -> Safari only, XU-OQ10 resolved by its own rule; BUD-REC1/REC4 one-way vs RTT -> correct text, XU-OQ2 update; SYNC-REC5 cap applies to final view time; XU-OQ18 add INT-REC7, INT-REC4 (6.9 renderer); TRN-REC3 HTTP/2 claim; ROOM-REC6 TF2 fill opt-in; BUD-REC4 input buffer 1-3 ticks proposed; TRN-REC4 Rests on + F34; label upgrades (item 10); TRN-F12 location §6.2.
C. Security: H1 SYNC-REC5; H2 SYNC-OQ11 + SYNC-F20 quote + option 3.2C/GC5 row; H3 limits (TRN-REC2/REC1 marks); H4 WS fallback (TRN-REC3); M1 Origin (TRN-F36 into a REC); M2 SYNC-REC2 validation table + SYNC-F19 g_active.c; M3 SYNC-OQ3 + REC2 mark; M4 BUD-REC2 event queue bound; M5 SYNC-REC6/INT-REC6 origins + INT-REC2 test; M6 ROOM-REC8 storage (OWASP); M7 ROOM-REC5 brute force; M8 ROOM-OQ2; M9 fuzz; M10 INT-REC7 follow rule; L1 SYNC-REC8 aim help; L2 INT OQ; L3 ROOM-REC3 logs; L4 display name; C28 TRN-F10 wording.
D. Rust: TRN-F28 "mature" + RustSec (new finding); TRN-F17 quinn min datagram size; ROOM-F7 label; TRN-OQ5 answer; TRN-REC5 rcgen dev-dep; axum ^0.29 vs 0.30; panic/unwind vs process; dangerous-configuration off.
E. Part 7 merged table; header; totals.

## Sources read in this fix pass (2026-10-28; curl copies in scratchpad r2fix/dl/, exact-text grep)
- Valve wiki Lag_compensation: WebFetch HTTP 403 again. Fiedler copies (verifier's r2v/dl) have no victim-side sentence. Search on allow-listed domains: only unvetted GitHub repos. => SYNC-F34 cannot be re-sourced: becomes R2-SYNC-OQ15.
- Q3 g_active.c (raw.githubusercontent.com): :772 "// sanity check the command time to prevent speedup cheating"; :773-774 `if ( ucmd->serverTime > level.time + 200 ) {` `ucmd->serverTime = level.time + 200;`; :777-778 `if ( ucmd->serverTime < level.time - 1000 ) {` `ucmd->serverTime = level.time - 1000;`; :788-789 `if ( msec > 200 ) {` `msec = 200;` => new R2-SYNC-F36.
- Source player.cpp :3539 `ConVar sv_usercmd_custom_random_seed( "sv_usercmd_custom_random_seed", "1", FCVAR_CHEAT, "When enabled server will populate an additional random seed independent of the client" );` :3567-3574 seed assignment; :103 sv_maxusrcmdprocessticks default "24".
- RustSec quinn-proto package page: 10 advisories; 2026: 0037 (Mar 9, HIGH), 0185 (Jun 22, HIGH), 0338-0343 (Oct 12; 0340, 0341, 0343 HIGH). Advisory pages: 0338 "Patched >=0.11.18" "Remote peer can panic quinn-proto with zero-length STREAM frames during unordered reads"; 0340 7.5 HIGH ">=0.11.18"; 0343 7.5 HIGH ">=0.11.18" "Outgoing datagram buffer accounting can panic the sender in quinn-proto", "This can be triggered without a malicious peer"; 0037 8.7 HIGH ">=0.11.14" "Receiving QUIC transport parameters containing invalid values could lead to a panic."; rustls 0346/0348 ">=0.23.46"; ring 2025-0009 ">=0.17.12" "In the QUIC protocol, an attacker can induce this panic by sending a specially-crafted packet."; tokio 2026-0384 ">=1.54.1". => new R2-TRN-F40.
- RFC 6455 §10.2 "...it SHOULD respond to the WebSocket handshake with a reply containing HTTP 403 Forbidden status code."; §10.4 "...MUST protect themselves against exceeding those limits." "Such an implementation SHOULD impose a limit on frame sizes and the total message size after reassembly from multiple frames."; §10.6 "Connection confidentiality and integrity is provided by running the WebSocket Protocol over TLS (wss URIs)." => new R2-TRN-F41.
- OWASP Session Management CS: "[!WARNING] Do not store authentication tokens, session IDs, JWTs, refresh tokens, or any credential in localStorage or sessionStorage. These APIs are accessible to any JavaScript executing in the origin, so a single XSS vulnerability discloses every token. Use HttpOnly; Secure; SameSite=Strict cookies (preferred) or a Backend-for-Frontend (BFF) pattern."; "It is recommended to log a salted-hash of the session ID instead of the session ID itself" => new R2-ROOM-F19.
- docs.rs quinn 0.11.12 Connection: max_datagram_size "...if the peer's limit is large this is guaranteed to be a little over a kilobyte at minimum."; rtt "Current best estimate of this connection's latency (round-trip-time)"; min_rtt "Minimum RTT seen on this path, ignoring ack delay".
- tf_bot_manager.cpp (verifier copy) :26 `ConVar tf_bot_quota( "tf_bot_quota", "0", FCVAR_NONE, "Determines the total number of tf bots in the game." );` :27 mode default "normal".
Next: edit the topic file top to bottom.

## Change log (type · what · where)
Batch 1 (sync findings), applied:
- source · SYNC-F19 incomplete (C27): note + new R2-SYNC-F36 (g_active.c clamp, read 2026-10-28) · Findings, after SYNC-F35
- source (reviewer-verified) · SYNC-F20 purpose quoted from player.cpp:3539 · SYNC-F20
- nit · SYNC-F28 verifier notes (RTT vs flow; clamp on estimate, +0.2 s) · SYNC-F28
- nit · SYNC-F32 combination now read (near OR cone) · SYNC-F32
- nit · SYNC-F33 default "24" (verifier + C16) · SYNC-F33
- nit · SYNC-F34 marked unsupported, re-source attempt failed, pointer OQ15 · SYNC-F34
- nit · GC5 row: shared seed caveat (H2) · sync legacy table
- nit · T row: F34 pointer replaced by arithmetic + OQ15 · sync legacy table
Batch 2 (budgets findings), applied:
- nit · BUD-F6, F8, F11, F13 (+getStats caveat), F21, F34, F35 labels upgraded per verifier · budgets findings
- nit · BUD-F14 confirmed verbatim; wording: at-risk note is on congestionControl (C28) · BUD-F14
- nit · BUD-F29 0.2 s meaning confirmed; "outgoing" = call name, header says RTT, one-way is assumption · BUD-F29
Batch 3 (transport findings), applied:
- nit · TRN-F3, F11, F13 (SHA-256, no interstitial), F19, F20, F30, F32, F34, F38 labels upgraded per verifier/security · transport findings
- nit · TRN-F10 wording (C28): at-risk feature is congestionControl itself · TRN-F10
- nit · TRN-F12 location §6.5 -> §6.2 (verifier) · TRN-F12 text and sources line
- source (reviewer-verified, re-read) · TRN-F17 quinn max_datagram_size "a little over a kilobyte at minimum" · TRN-F17
- source · TRN-F28 "mature" qualified -> new R2-TRN-F40 (RustSec, read 2026-10-28) · TRN-F28, new TRN-F40
- nit · TRN-F33 axum ^0.29 vs 0.30 (Rust review) · TRN-F33
- source · new R2-TRN-F41 (RFC 6455 §10.2, §10.4, §10.6, read 2026-10-28) · after TRN-F39
Batch 4 (rooms findings), applied:
- nit · ROOM-F7 link upgraded (verifier, Rust review) · ROOM-F7
- nit · ROOM-F15 defaults "0"/"normal", totals not per team (verifier) · ROOM-F15
- nit · ROOM-F17 "owner alone" unsupported -> source's words (verifier) · ROOM-F17
- source · new R2-ROOM-F19 (OWASP localStorage warning, HttpOnly, salted-hash logging; read 2026-10-28) · after ROOM-F18
- nit · CD2 row: guest secret not in client storage · rooms legacy table
Batch 5 (index for Q6 and options), applied:
- nit · Q6 index: F34 unsupported; F36 added to Quake 3 row
- nit · option 3.2 C shared-seed caveat (H2); option 3.5 A/B F34 pointers -> OQ15
- number · budgets table A note: RTT reading gives T1/T2 200-250, T3 150-185, T4 250-300 ms (arithmetic); T1 cell qualified
- nit · transport Q1 "mature" -> F40 pointer; rooms 3.4 F17 wording; 3.5 A TF2 default note; 3.7 A localStorage note
Next: recommendations, sync block (REC2, REC3, REC5, REC6, REC8, REC1 label).
Batch 6 (sync recommendations), applied:
- nit · SYNC-REC1 full-snapshot path confirmed (verifier, sv_snapshot.c:62-63)
- recommendation · SYNC-REC2 catch-up cap <= redundancy cap (M3); validation list (M2); F36 in Rests on; marks 3 extended, 4 added
- recommendation · SYNC-REC3 shot feedback without shared seed (H2); OQ11 for Mustafa
- recommendation · SYNC-REC5 tolerance <= one snapshot interval, server-own RTT, delay clamp (H1); WS lowest ping (H4); cap on final rewind (verifier 3); logging (H1); reason no longer rests on F34; F34 out of Rests on, TRN-F25 in; mark 2 moved to OQ15; marks 5, 6 added
- recommendation · SYNC-REC6 re-based trajectories (M5) + mark 4
- recommendation · SYNC-REC8 aim help applied on the server (L1)
Next: budgets recommendations.
Batch 7 (budgets recommendations), applied:
- recommendation+number · BUD-REC1 reason: one-way assumption flagged; RTT reading 30 Hz 200-250, 20 Hz 250-300 ms; cap not sourced
- recommendation · BUD-REC2 bounded event queue + mark (M4)
- nit · BUD-REC3 F11 confirmed; Serves + §6.1 (fixer, XU-OQ18)
- recommendation+number · BUD-REC4 "one-way" -> server's latency estimate (XU-OQ2); input buffer proposed, F35 corrected; getStats Safari only (TRN-F9 added to Rests on); reason gives both readings (200-250 ms RTT)
- recommendation · BUD-REC5 counters only via getStats (TRN-F9, F19 added); F14 wording corrected
Next: interest recommendations (INT-REC1 L2 note, INT-REC2 reader test M5, INT-REC4 Serves, INT-REC5 Spy wording, INT-REC6 M5, INT-REC7 M10 + conditional).
