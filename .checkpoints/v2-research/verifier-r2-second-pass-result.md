# Second verifier pass on R2: the verifier's report, verbatim

Workflow run wf_64d0bbcd-730 (script `v2-research-second-pass-run1.js`, prompt `prompts/verifier-r2-second-pass.txt`), returned 2026-10-28 (about 16:24 UTC). Notes with every quote: `verifier-r2-second-pass.md`.

**VERDICT: PASS WITH NOTES.** Two claims are wrong: one clause of new finding R2-SYNC-F36, and the reason the fix pass rewrote for R2-ROOM-REC3. No recommendation rests on the wrong part of SYNC-F36. ROOM-REC3's design still holds, but its reason needs rewording.

Gate and symbol checks were skipped, as the task said. All sources were read with WebFetch on allow-listed domains on 2026-10-28.

**Table** (scope · id · claim · source · verdict · location and exact words)

*A. New findings*
- A · SYNC-F36, parts 1–2 · the server enforces the command time; clamp +200 / −1000 ms · Q3 g_active.c (raw.githubusercontent.com) · **confirmed** · ClientThink_real: "// sanity check the command time to prevent speedup cheating"; "ucmd->serverTime = level.time + 200;"; "ucmd->serverTime = level.time - 1000;"
- A · SYNC-F36, part 3 · "runs at most 200 ms of movement from one command" · g_active.c, bg_pmove.c · **wrong**
  - The clamped `msec` is used only in "ClientTimerActions( ent, msec );".
  - Movement runs through "pm.cmd = *ucmd;" and "Pmove (&pm);".
  - Pmove's own cap is 1,000 ms: "if ( finalTime > pmove->ps->commandTime + 1000 ) {" "pmove->ps->commandTime = finalTime - 1000;", run in 66 ms slices ("if ( msec > 66 ) {").
- A · TRN-F40 · ten quinn-proto advisories, eight in 2026, plus the patched floors · rustsec.org package page and 8 advisory pages · **confirmed** · every quote and floor is verbatim.
  - 0037: "8.7 HIGH", ">=0.11.14". 0338, 0340, 0343: ">=0.11.18". rustls 0346/0348: ">=0.23.46". ring: ">=0.17.12". tokio: ">=1.54.1".
  - Extra: 0339, 0341 and 0342 are patched from ">=0.11.17" and 0185 from ">=0.11.15", so the 0.11.18 floor covers all ten.
- A · TRN-F41 · RFC 6455: Origin check with 403, size limits, wss · rfc-editor.org · **confirmed**
  - §10.2: "...SHOULD respond to the WebSocket handshake with a reply containing HTTP 403 Forbidden status code."
  - §10.4: "Such an implementation SHOULD impose a limit on frame sizes" "and the total message size after reassembly from multiple frames."
  - §10.6: "...over TLS (wss URIs)."
- A · TRN-F42 · tungstenite 0.30 defaults · docs.rs WebSocketConfig 0.30.0 · **confirmed** · "The default value is 64 MiB which should be reasonably big…"; "The default value is 16 MiB…"; "The default value is unlimited."
- A · ROOM-F19 · OWASP: no credential in local or session storage; HttpOnly; log a salted hash · OWASP Session Management Cheat Sheet · **confirmed** · all three quotes verbatim under "The localStorage API", "HttpOnly Attribute" and "Logging Sessions Life Cycle…".

*B. Basis claim with no first-pass verdict*
- B · TRN-F36 · Origin MUST be verified, 403, H3_EXCESSIVE_LOAD · draft-ietf-webtrans-http3-16 · **confirmed**
  - §3.2: "the WebTransport server MUST verify the `Origin` header…" and "SHOULD reply with status code 403".
  - §8: "Implementations SHOULD track the use of WebTransport features" and "An endpoint MAY treat activity that is suspicious as a connection error of type H3_EXCESSIVE_LOAD."

*C. Changed findings*
- C · SYNC-F20 · the server seed's purpose · source-sdk-2013 player.cpp · **confirmed** · "When enabled server will populate an additional random seed independent of the client" (default "1", FCVAR_CHEAT).
- C · TRN-F17 · quinn's minimum datagram size · docs.rs quinn 0.11.12 `Connection` · **confirmed** · "…if the peer’s limit is large this is guaranteed to be a little over a kilobyte at minimum."
- C · TRN-F33 · axum 0.8.9 depends on tokio-tungstenite ^0.29.0 · docs.rs/crate/axum/0.8.9 · **confirmed**
  - "tokio-tungstenite ^0.29.0 _normal_ _optional_".
  - tokio-tungstenite 0.30.0 needs "tungstenite ^0.30.0"; 0.29.0 needs "tungstenite ^0.29.0".
  - Cargo book: "0.2.3  :=  >=0.2.3, <0.3.0". This also confirms F42's inferred link.
- C · BUD-F14 and TRN-F10 · the feature at risk is the `congestionControl` option itself · W3C TR §6.9 · **confirmed** · under member `congestionControl`: "This configuration option is considered a feature at risk due to the lack of implementation in browsers" "of a congestion control algorithm, at the time of writing, that optimizes for low latency."
- C · BUD-F35 · the jitter buffer is receive-side, for state packets · Gaffer "State Synchronization" · **confirmed** · "To handle this situation you need to implement a jitter buffer for your state update packets."
- C · ROOM-F15 · 'fill' is not TF2's default and counts the whole game · tf_bot_manager.cpp · **confirmed** · `tf_bot_quota( "tf_bot_quota", "0", …` and `"tf_bot_quota_mode", "normal"`; "keep N players in the game".
- C · **label only (25)** · BUD-F6, F8, F11, F13, F21, F29, F34; ROOM-F7, F17; SYNC-F19, F28, F32, F33, F34; TRN-F3, F11, F12, F13, F19, F20, F28, F30, F32, F34, F38. The quotes are in the first-pass notes (`verifier-r2.md`; TRN-F13's in `security-auditor-r2-protocol.md:53`).

*D. Claims inside changed recommendations*
- D · TRN-REC1 · quinn's retry and refusal calls · docs.rs quinn `Incoming` · **confirmed** · "Reject this incoming connection attempt"; "Respond with a retry packet, requiring the client to retry with address validation".
- D · TRN-REC1 · wtransport has a `dangerous-configuration` feature · docs.rs wtransport 0.7.2 features page · **confirmed** · listed; the defaults are "ring" and "self-signed".
- D · TRN-REC1 · a request without an Origin comes from a non-browser client · draft -16 §3.2 · **confirmed at the source, but this passage is not in F36** · "If the WebTransport session is coming from a browser client, an `Origin` header MUST be provided within the request." "Otherwise, the header is OPTIONAL."
- D · TRN-REC5 · web-transport-quinn lists rcgen only as a dev dependency · docs.rs/crate/web-transport-quinn/0.13.5 · **confirmed** · "rcgen ^0.14 _dev_".
- D · ROOM-REC3 · "neither browser constructor takes a header option … WebTransport's options are exactly …" · W3C TR (Candidate Recommendation Snapshot, 30 July 2026) §6.9 · **wrong at the spec**
  - The WebIDL has "HeadersInit headers = {};": "Optionally, a Headers object, an object literal, or an array of two-item arrays" "to set the request header list to use when establishing a WebTransport session."
  - MDN's five-option list is incomplete. The browser-compat data has no `headers` key, so whether any browser ships it is not known.
- D · TRN-REC3 · the HTTP/2 fallback "is documented only for Safari" · F12 · **unsupported** · F12's own W3C part (§6.2) defines the HTTP/2 path for any browser.
- D · SYNC-REC5 and TRN-REC3 · "the WebSocket fallback has no transport RTT" · no finding · **unsupported** · not marked as inferred.

*E. Number changes*
- Budgets table A (RTT reading), BUD-REC1's 30 Hz and 20 Hz figures, BUD-REC4's 200–250 ms · arithmetic · **all correct** · each is 100 ms plus the interpolation delay (100–150, 50–85 or 150–200 ms).

**Counts:** confirmed 17 · wrong 2 · unsupported 2 · label only 25. All 3 number changes are correct.

**Per changed recommendation (27 read)**
- **SYNC:**
  - REC1 OK (label only).
  - REC2 OK; it uses only the confirmed clamp part of F36.
  - REC3 OK with a note: "a seed the client knows would let a cheat cancel the spread" is unmarked reasoning.
  - REC5 OK with notes: the WebSocket RTT claim is unsourced, and "lets a low-ping client claim far more rewind than it needs" is unmarked reasoning. F34 is fully out of it; 450 = 250 + 200 and 1.8 m = 9.0 m/s × 0.2 s are correct.
  - REC6 OK.
  - REC8 OK with a note: "cannot be told apart from an aimbot" is unmarked reasoning.
- **BUD:**
  - **REC1: problems.**
    - Its lead sentence still calls 30 Hz "the only snapshot rate" that keeps the rewind inside the ~200 ms cap. On the round-trip reading, its own note shows 20 Hz is rejected only by a 50 ms shorter rewind against a cap with no source.
    - Its pass mark "100 ms RTT … rewind ≤ 200 ms for 100 % of shots" was not updated. The same paragraph computes 200–250 ms at that RTT.
  - REC2 OK.
  - REC3 OK; the 1257 B arithmetic re-done.
  - REC4 OK.
  - REC5 OK with a note: "only `outgoingMaxAge` exists in all three engines" is true of the sending settings; `incomingMaxAge` is in all three too.
  - REC4 and REC5 name no VISION_V2 §6 item. This is known and left to the Advisor (R2-XU-OQ18).
- **INT:**
  - REC1, REC2, REC4, REC5 OK.
  - REC6 OK; minor: it still cites §6.9, which the fix removed from REC4 as "not a network need", but §6.4 carries it.
  - REC7 OK as a conditional recommendation.
- **TRN:**
  - REC1 OK; the version floors match F40; note the Origin passage above.
  - REC3 OK with the two unsupported items above.
  - REC4 and REC5 OK.
  - REC6 OK; no §6 item (R2-XU-OQ18).
- **ROOM:**
  - REC2 OK.
  - **REC3: problem.** The header reason is wrong at the spec. The design itself (token in the first stream message) still follows from WebSocket's URL-plus-sub-protocols constructor and from URL leaks (F14).
  - REC5, REC6 and REC8 OK.

**Other findings, ranked**
1. **ROOM-REC3 and ROOM-F12:** the spec's `headers` option contradicts "no header option" and "options are exactly …". F12's inferred "Neither browser transport lets the page set an HTTP auth header" is contradicted for WebTransport. F12's text was not changed in the fix, but it now carries a changed recommendation.
2. **SYNC-F36:** remove or correct the "200 ms of movement" clause. Part 7's line "quotes were matched by exact text search" is true of the quotes but not of this reading.
3. **BUD-REC1:** the lead sentence and the pass mark were not brought into line with the round-trip reading.
4. **Section F:** options table 3.5 still states F34's content (:1623-1624), flagged as unsupported but still there. Row A: "Shooters lead by RTT + interpolation (F27, F34)". Row B: "the victim's 'hit behind cover' window grows with it". The "owner alone" part of ROOM-F17 is gone everywhere except F17's own note. ROOM-REC8 no longer keeps the secret in client storage; the legacy CD2 row (:1564) still opens with "keeps only the guest credential" before its correction.
5. **TRN-REC1:** add the draft §3.2 "OPTIONAL" passage to F36. "Cannot be refused on that ground alone" is an unmarked policy reading.
6. **TRN-REC3 and SYNC-REC5:** the "documented only for Safari" and "no transport RTT" claims; plus the minor unmarked-reasoning notes listed above.

**G. Counts:** all match.
- Header and TL;DR: findings 36 + 35 + 20 + 42 + 19 = 152; recommendations 36; open questions 54 + 18 = 72; sources read 116 (rows 1–116, no gaps); not read 26.
- Part 7: 137 unique rows; 98 no change (94 plus 4 annotated), 36 fixed, 2 re-sourced, 1 now an open question.

**Header label after this pass:** "verified (every recommendation claim; second pass 2026-10-28: 2 fixes pending: R2-SYNC-F36's movement clause and R2-ROOM-REC3's header reason)". Once the owner fixes both: "verified (every recommendation claim)". The ROOM-REC3 rewording changes a recommendation's reason, so by the plan's rule it needs a short re-check.

**Not verified:**
- Line numbers such as player.cpp:3539: WebFetch shows no line numbers. Location only.
- Whether any browser ships WebTransport's `headers` option: the compat data has no entry.

Notes (end with DONE): /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r2-second-pass.md
