# R2: the final check, the agent's report verbatim

Workflow run wf_0ad9f0c0-59d (script `v2-research-fix2-run1.js`, prompt `prompts/verifier-r2-final-check.txt`), returned 2026-10-28 (about 20:24 UTC). Notes: `verifier-r2-final-check.md`.

VERDICT: PASS WITH NOTES. All four items the second pass found wrong or unsupported are now fixed to the source's words, and the ROOM-REC3 re-check passes. The round introduced one new wrong-as-worded clause (in BUD-REC5), but no recommendation depends on it. Everything left is a nit.

Gate and symbol checks: skipped, as the task said (research check). Every source was read on 2026-10-28 with WebFetch, on allow-listed domains only. Line numbers below are from the working tree of `/home/user/Shamble-Frontlines/docs/research/r2-netcode.md`.

**1. Items the second pass found wrong or unsupported, and problems in recommendations**
- **SYNC-F36 movement clause (wrong):** closed. Corrected to g_active.c's words at :416-419, with the verifier's note at :426-431.
- **ROOM-REC3 header reason (wrong):** closed. The reason is reworded at :2744-2748, the bracket at :2750-2754, ROOM-F12 at :1488-1503 and option 3.2 at :1908-1910. Re-check done: every part matches its source.
- **TRN-REC3 "documented only for Safari" (unsupported):** closed, at :2604-2606.
- **"WebSocket has no transport RTT" (unsupported):** closed. It is marked as our reading in SYNC-REC5 (:2136-2137) and TRN-REC3 (:2618-2619).
- **BUD-REC1:** closed. Lead sentence at :2284-2289, pass mark at :2300-2303, Rests on at :2296-2297.
  - Still open (nit): R2-XU-OQ1 at :3070 still quotes the old mark, "R2-BUD-REC1's pass mark asks for "rewind ≤ 200 ms for 100 % of shots"".
- **Ranked finding 4:** closed. Option 3.5 A and B at :1645-1646; the CD2 row at :1586 now matches ROOM-REC8 (:2820-2822).
- **Ranked finding 5:** closed. TRN-F36 at :1273-1275 and TRN-REC1 at :2540-2544.
- **Ranked finding 6 (unmarked reasoning):** closed. SYNC-REC2 :2029-2030, REC3 :2075, REC5 :2134, REC8 :2236-2237.
- **The verifier's minor notes:**
  - INT-REC6 at :2500-2502: closed. VISION_V2 §6 item 9 reads "A spectacle budget: the renderer handles a few very large units…".
  - TRN-F40 at :1299-1304 and TRN-REC1 at :2533-2534: closed.
  - BUD-REC5 at :2363-2365: `incomingMaxAge` is added (closed), but the new "only `outgoingMaxAge`" wording is wrong as worded (row 14 below).
- **2V-§6** (BUD-REC4, BUD-REC5 and TRN-REC6 name no VISION_V2 §6 item): not applied by design. It stays with the Advisor (R2-XU-OQ18).

**2. Claims the round re-sourced or added** (id · claim · source · verdict · location and exact words)
1. SYNC-F36 · the 200 ms cap applies only to what goes to the timer actions; movement runs through Pmove on the clamped command · Q3 g_active.c · **confirmed** · ClientThink_real: "if ( msec > 200 ) {" "msec = 200;" "ClientTimerActions( ent, msec );" "pm.cmd = *ucmd;" "Pmove (&pm);". No other line uses `msec`.
2. SYNC-F36 note · Pmove's own cap is 1,000 ms, run in 66 ms slices · bg_pmove.c · **confirmed** · Pmove: "if ( finalTime > pmove->ps->commandTime + 1000 ) {" "pmove->ps->commandTime = finalTime - 1000;" "if ( msec > 66 ) {". Nit: 66 ms applies only without `pmove_fixed`, which uses "if ( msec > pmove->pmove_msec ) {".
3. TRN-F36 and TRN-REC1 · an Origin is required only from browsers · draft-ietf-webtrans-http3-16 §3.2 "Creating a New Session" · **confirmed** · "If the WebTransport session is coming from a browser client, an Origin header [RFC6454] MUST be provided within the request." "Otherwise, the header is OPTIONAL." Nit: the file drops "[RFC6454]" from the quote without an ellipsis.
4. TRN-F40 · three of the six newest (October 12) are patched only from 0.11.18 · rustsec.org package page and the 0338, 0340, 0343 pages · **confirmed** · all six are listed "October 12, 2026"; 0338, 0340 and 0343 each say ">=0.11.18".
5. TRN-F40 note · 0339, 0341, 0342 ">=0.11.17"; 0185 ">=0.11.15"; 0.11.18 covers all ten · rustsec advisory pages · **confirmed** · exactly those strings, plus 2024-0373 ">=0.11.7" and 2023-0063 "^0.9.5" / ">=0.10.5".
6. TRN-REC1 · "three of the newest are patched only from 0.11.18" · the same pages · **confirmed** · as rows 4 and 5.
7. ROOM-F12 (and its note, ROOM-REC3, option 3.2, source row 10) · the spec defines a `headers` option · W3C TR (Candidate Recommendation Snapshot, 30 July 2026) §6.9 "Configuration" · **confirmed** · "HeadersInit headers = {};" "Optionally, a Headers object, an object literal, or an array of two-item arrays to set the request header list" "to use when establishing a WebTransport session."
8. ROOM-F12 · MDN's five-option list is incomplete · the same WebIDL · **confirmed** · the dictionary also has `headers`, the two `anticipatedConcurrentIncoming…Streams` members and `datagramsReadableType`.
9. ROOM-F12 (and ROOM-REC3, option 3.2, source row 21) · whether any browser ships `headers` is not known; the compat data has no `headers` key · BCD api/WebTransport.json · **confirmed** · the constructor's option keys are exactly the five `options_…_parameter` keys; lowercase "headers" is absent (only "responseHeaders").
10. ROOM-REC3 and option 3.2 · the WebSocket constructor takes only a URL and sub-protocols · MDN WebSocket() · **confirmed** · "new WebSocket(url)" "new WebSocket(url, protocols)". "No auth header" stays inferred, as F12 marks it; REC3 does not repeat that mark (nit).
11. TRN-REC3 · the W3C constructor defines the HTTP/2 path (§6.2) · W3C TR · **confirmed** · under the body heading "6.2. Constructor", before "6.3. Attributes": "If the connection is an HTTP/2 connection [WEB-TRANSPORT-HTTP2], set transport's [[Reliability]] to "reliable-only"." (One WebFetch call guessed §6.5 from the table of contents; two body-order calls settled it as §6.2.)
12. TRN-REC3 · only the WebKit post documents a browser shipping it · webkit.org/blog/17862 and BCD · **confirmed** for the post: "When the underlying network environment doesn't support QUIC, WebTransport can run over HTTP/2 and TCP" "as a fallback with the same API." The "only" is a claim about the sources read (nit).
    - BCD has no "HTTP/2" string.
    - Its `supportsReliableOnly` key ("version_added": "157" / "158" / "26.4") is only a probe. The editor's draft §6.3 says it "Returns true if the user agent supports WebTransport sessions over exclusively reliable connections, otherwise false."
13. BUD-REC5 · `incomingMaxAge` is in all three engines · BCD WebTransportDatagramDuplexStream.json · **confirmed** · Chrome "97", Firefox "114", Safari "26.4", "standard_track": true.
14. BUD-REC5 · "Of the browser-side sending settings, only `outgoingMaxAge` exists in all three engines" · the same file · **wrong as worded** · `outgoingHighWaterMark` is also listed for Chrome ("version_added": "97"), Firefox ("114") and Safari ("26.4"), with "standard_track": false and "deprecated": true. Not load-bearing, because the recommendation uses only `outgoingMaxAge`. Fix: "of the standards-track sending settings".
15. INT-REC6 · §6.9 is the renderer's spectacle budget · docs/VISION_V2.md §6 · **confirmed** · "A spectacle budget: the renderer handles a few very large units and their effects without dropping frames."

Counts: confirmed 14 · wrong 1 · unsupported 0.

**3. Rewritten reasons, pass marks and numbers**
- **BUD-REC1:** closed. It now claims only what its findings say.
  - 286 against 146 kbps comes from the worked table (:1675, :1772).
  - 150–200, 200–250 and 250–300 ms are 50 or 100 ms plus the interpolation delay.
  - "A cap with no source" matches R2-XU-OQ1, and the pass mark is marked proposed.
- **TRN-REC1:** closed. The policy is marked as ours.
- **TRN-REC3:** closed, with the "only" nit.
- **ROOM-REC3:** closed. The design is marked as our choice.
- **Option 3.5 A:** closed. 1.8 m = (100 + 100 ms) × 9.0 m/s, from the legacy `T` row at :452, and it is marked as our reading.
- **BUD-REC5:** one wrong-as-worded clause (row 14).
- No unmarked new number.

**4. Header and part 7's "Second verifier pass" subsection**
- **Header (:8-11):** matches the file, except its last sentence ("ROOM-REC3's reworded reason awaits the short re-check"); that re-check is now done.
- **Part 7 counts:** they match: 17 + 2 + 2 + 25 + 3 = 49 claims, 6 fixed + 43 unchanged = 49, 22 rows.
- **Part 7 nits:**
  - :3582 says "the 33 changed basis findings"; the scope file lists 32.
  - :3640-3641, "rows 10, 11, 21 and 115", leaves out row 114 (the TRN-F40 rewording) and row 22 (BUD-REC5, through TRN-F19).
  - :3643 says one recommendation's reason changed (ROOM-REC3). The owner's own change log counts four reasons (BUD-REC1, TRN-REC1, TRN-REC3, ROOM-REC3) plus one pass mark.

**What remains:** only nits.
1. BUD-REC5's "only" wording (row 14).
2. R2-XU-OQ1's stale pass-mark quote at :3070.
3. The three part 7 nits at :3582, :3640-3641 and :3643.
4. TRN-F36's dropped "[RFC6454]".
5. The unqualified 66 ms in the bg_pmove.c note.
6. TRN-REC3's "only" (it means "of the sources read").
7. ROOM-REC3 and option 3.2 repeat WebSocket's "no auth header" without F12's "inferred" mark.

**Header label:**
- Now: "verified (every recommendation claim; final check 2026-10-28: nits only, R2-BUD-REC5's sending-settings wording pending)".
- After the nits are fixed: "verified (every recommendation claim)". If BUD-REC5 is reworded to "of the standards-track sending settings", it needs no further pass, because the BCD quotes above support exactly that.

**Not verified:** line numbers in the code sources, because WebFetch shows none; locations are given by function or section.

Notes, ending with DONE: /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r2-final-check.md
