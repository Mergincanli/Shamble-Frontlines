# netcode-architect: U7 build brief rev 1 (notes)

Task: draft u7-brief.md rev 1 (U7 "a plain browser client") for the one advisor check NPT-TRIM allows.
Branch claude/netcode-proof-test-plan-hcky5g. Started 2026-11-22. No Edit tool: this file is rewritten whole by Write.

## Plan
1. Read sources. 2. Browser facts (W3C, MDN, BCD). 3. Draft u7-brief.md in u6c-brief.md's shape (0-10), lines <=120.
4. Quote check script in u7-brief/; run; count. 5. Return report.

## Progress
- Block 1 (reads) DONE.
- Block 2 (browser facts) DONE: see F25-F29.
- Coordinator message (~20:3x UTC): advisor-u6c-brief.md rulings bind U7 (C9, Q-U6C-3 (a) no alias, C1 (e), section 5
  items 3 and 5, C1 (c)). Read whole: F30-F33.
- Block 3 (draft u7-brief.md): next.

## Findings (one line each, as I go)
- F1 NPT-TRIM history.md:7572; R1-DAY Thu 3 Dec :7604; TEST-FIRST rule 3 :2267.
- F2 replan.md:18, :77-79, :89-94, :101-102 (reading 3).
- F3 replan-units.md §4 :123-149; §7 :192-222.
- F4 plan.md line drift +1 past :379 (advisor-u6c-brief.md:18-21, N0: 4d71571): U7 row :1255; -36 row :740; R1 row
  :819; E7 web :495-504; hash file :995-996; smoke check :1214-1223 (before :379? no: after, but the brief cites live).
- F5 plan.md:643-644 npm pins; U7 needs neither Vite nor three.js.
- F6 C-U7-1..4 NOT in u4s-carries.md (0 hits); home plan.md:1255; C-U7-3/-4 origin advisor-u4f-r1.md:146-154.
  u4s-carries.md:831 `ws://`/wrong-Origin pages "go with U7's web harness" -> CUT (replan-units.md:19).
- F7 u4s-carries.md:870-871 cert-hash "is U6d's" -> U6c Block 5.
- F8 u6c-brief.md:126-131 allow-list of U7's dist names; CSP plan.md:1123-1126 word for word on every answer.
- F9 CSP: no inline script/style; 'wasm-unsafe-eval'; worker-src 'self'; connect-src 'self' + rs.nt https/wss.
- F10 C-U6-19 gate line (advisor-u6a-brief.md:233-237 = u6a-brief.md:30-34): U7 states one line.
- F11 U6b cond 4 (u6b-brief.md:60-66) hash JSON; "U7's page turns `value_hex` into `serverCertificateHashes`' bytes."
  Ports UDP 30001+s / TCP 31001+s (plan.md:909-910). WS tag values: no home (lean npt-link).
- F12 Token line `{slot} nptj_{base64url}` (wip/u6a-build.patch:789); URL `#t=<token>` (config.mjs:115-123).
- F13 PROTOCOL_VERSION/CONTENT_HASH private in U6a main.rs (patch :548-551) -> lean pub consts in npt-proto.
- F14 bind: catalogue :112-115.
- F15 Endpoint API: input lacks prev_send_ns; with_clock(9) bots/lab; ClockSync hardening sites clock.rs:122-124,
  :163, :209; pub surface list.
- F16 client-wasm stub; check.mjs LEAF (:39) lacks npt-client-wasm.
- F17 U8 interfaces; save() writes no header -> not npt-log (log lib.rs:585). Lean: U8's save composes header+footer.
- F18 rs.yaml dist build (patch :884-890).
- F19 cert14d.mjs/R1.md `/npt/cert-hash.json` stale -> fix pass item 3 (advisor-u6c-brief.md:339-340).
- F20 marks: -36 mds_read.size_bytes; -29 fallback.playing_ns <=3 s, 20 attempts, bite no-wt-timeout; PM-0006-25
  attempt.ok = "session ready and one echo" (marks.json:11463); -14 second.v.recv_over_max_count (:3583-3597); -28
  wt-attempts + wrong-hash control.
- F21 CONTRADICTION replan-units.md:138 (connect.ts goes, -28 moved) vs replan.md:63-65 / replan-marks.md:48 (-28's
  carried-crate parts kept, scripted page on the laptop after U7). Lean: page accepts suite `wt-attempts` + wrong-hash.
- F22 security-r1-deps.md:245 web/.npmrc vs advisor-u8-brief.md:185-189 C2 (no .npmrc; U7 takes it). C2 rules.
- F23 Chromium 1243 at /opt/pw-browsers/chromium-1243/chrome-linux64; tools/ playwright-core 1.56.1 wants 1194.
- F24 wss in container needs a container-only cert workaround; never on the laptop (plan.md:1222-1223).
- F25 MDN WebTransport() (read 2026-11-22): hashes only with allowPooling false; "X.509v3 certificate with a validity
  period of less than 2 weeks"; ECDSA P-256 minimum, no RSA; algorithm `sha-256`; requireUnreliable = no HTTP/2;
  "available in Web Workers"; secure contexts only.
- F26 W3C WebTransport TR: "[Exposed=(Window,Worker), SecureContext]"; 6.9 "the total length of the validity period
  MUST NOT exceed two weeks"; maxDatagramSize "The maximum size data that may be passed to a
  WebTransportDatagramsWritable". 13 days meets both.
- F27 BCD (raw mdn/browser-compat-data, read 2026-11-22): WebTransport Chrome 97, Firefox 114, Safari 26.4;
  serverCertificateHashes Chrome/Edge 100, Firefox 125; requireUnreliable Chrome false(!), Firefox 114;
  maxDatagramSize Chrome 97, Firefox 114; datagrams.writable deprecated, createWritable Chrome 159, Firefox 155 ->
  feature-detect. Worker options.type module: Chrome 80, Edge mirror, Firefox 114, Safari 15.
- F28 MDN connect-src lists fetch() and WebSocket, not WebTransport; "connect-src 'self' does not resolve to websocket
  schemes in all browsers" -> CSP names wss explicitly (plan's does). MDN Using web workers: worker CSP = the worker
  script's own response header (host sends CSP on every answer: Worker gets it).
- F29 Playwright docs: executablePath "use at your own risk"; "Playwright only works with the bundled Chromium";
  ignoreHTTPSErrors "Whether to ignore HTTPS errors when sending network requests" (WebSocket not stated).
- F30 advisor-u6c-brief.md C9 (:221-224): "U7's drafter takes C1 (e): the page fetches `/npt/cert-hash-<room>.json`
  with the `room` of its own query, and the host ignores queries."
- F31 C1 (e) (:151-154): host matches path up to first `?`, ignores query, never reads/logs it; serves
  `/npt/cert-hash-NN.json` NN 00-31, no `/npt/cert-hash.json`; Q-U6C-3 (:237-240) no alias (a page could pin a room it
  did not join; -29 would read a WT failure as the fallback working).
- F32 Section 5 item 3 (:339-340) U8 renames to per-room URL; item 5 (:350-355) npt-load pin_hex (wt: value_hex; ws:
  production SHA256); C1 (c) (:138-146) npt-load settings. Item 8: npt-super flags after U6a merges.
- F33 Advisor (:262-266): growth 1.9-2.3x (U8 1.94, U6a 2.33), not 1.35-1.5; lane: U7's Rust half Sun 29; Stop B
  off-lane from ~Sat 28; R1 Thu 3 Dec ~1 day slack; triggers (U6b by Thu 26, U6c by Sun 29, SIZE-CHECK). F26 redact
  false positive on npm lock `resolved` names (:367-388): "U7's lock will be the next".
- F34 Earlier Rust half? Lean no: each lane slot before U6c delays U6c's merge; agent cap 3 full in rounds. Gain
  without lane: TS block T5 early transport contact against U6b's merged binary (off-lane, runs a built binary).
