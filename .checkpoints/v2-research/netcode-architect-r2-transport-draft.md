# R2 unit transport: the WebTransport server stack in Rust plus the WebSocket fallback

Brief question (quoted): "the WebTransport server stack in Rust plus the WebSocket fallback" (docs/V2_PHASE0_BRIEF.md §4, R2 row,
one of its six questions). Unit scope from the wave-1 prompt: the specification status (W3C API, IETF protocol over HTTP/3 and its
drafts), browser support in Chrome, Edge, Firefox and Safari, datagrams versus streams and the datagram size limit, certificates,
the Rust crates, the WebSocket fallback and head-of-line blocking, one transport interface over both, congestion control, and
what a spike must prove.

Author: netcode-architect, 2026-10-27. Status: unit draft (wave 1), not yet verified. Every source was read on 2026-10-27.
"Confirmed in the source" is used only where the exact words were quoted back from the page (the quotes are in the finding);
anything seen only in a fetch summary is marked "inferred".

## Findings

### Specification status

**R2-TRN-F1.** The W3C WebTransport API is a Candidate Recommendation Snapshot dated 30 July 2026. It "will remain a Candidate
Recommendation at least until 30 October 2026". It is not yet a W3C Recommendation.
Source: https://www.w3.org/TR/webtransport/, status line ("W3C Candidate Recommendation Snapshot, 30 July 2026") and "Status of
this document". Tier: official. Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F2.** The wire protocol, WebTransport over HTTP/3, is still an Internet-Draft, not an RFC. The page reads:
"draft-ietf-webtrans-http3-16", last updated "2026-07-06", intended status "Proposed Standard", WG state "In WG Last Call", IESG
state "I-D Exists", "Expires 7 January 2027". The history page lists 17 revisions (-00, 2021-02-17, to -16), and the WG Last Call
began on 2026-03-08.
Source: https://datatracker.ietf.org/doc/draft-ietf-webtrans-http3/ (document fields) and /history/. Tier: official. Read 2026-10-27.
How sure: confirmed in the source for the -16 fields; inferred (fetch summary) for the revision list and the WGLC date.

**R2-TRN-F3.** WebTransport over HTTP/2, the TCP variant, is also a draft: draft-ietf-webtrans-http2-15 (2026-07-06), in WG Last
Call, intended Proposed Standard.
Source: https://datatracker.ietf.org/doc/draft-ietf-webtrans-http2/. Tier: official. Read 2026-10-27. How sure: inferred (fetch
summary; only the abstract sentence was quoted).

**R2-TRN-F4.** Draft versions are not wire-compatible. Each version is negotiated through an HTTP/3 SETTINGS codepoint, and the
highest version both sides support wins. Draft -07, Appendix A.1: "The following changes make the draft-02 and draft-07
versions of this protocol incompatible". Draft -16, §7.1 (to be removed before publication): "Each draft version defines a
distinct codepoint for SETTINGS_WT_ENABLED" and "The highest version supported by both endpoints is selected." The -16 codepoint is
0x2c7cf000 (§9.2). Draft -02 used SETTINGS_ENABLE_WEBTRANSPORT = 0x2b603742, and draft -07 used
SETTINGS_WEBTRANSPORT_MAX_SESSIONS = 0xc671706a.
Sources: https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3-07 (App. A.1, §8.2);
https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 (-16, §7.1, §9.2);
https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3-02 (§8.2). Tier: official. Read 2026-10-27. How sure: confirmed in
the source (quoted fragments and codepoints).

**R2-TRN-F5.** QUICHE, the QUIC and HTTP/3 library Chromium uses, defines only the draft-02 and draft-07 WebTransport settings
in its HTTP constants header: "SETTINGS_WEBTRANS_DRAFT00 = 0x2b603742," and "SETTINGS_WEBTRANS_MAX_SESSIONS_DRAFT07 = 0xc671706a,".
That file has no -16 SETTINGS_WT_ENABLED.
Source: https://raw.githubusercontent.com/google/quiche/main/quiche/quic/core/http/http_constants.h (main branch). Tier: official
(source code). Read 2026-10-27. How sure: confirmed in the source for the constants. The step to "Chrome speaks draft-02/-07 today"
is inferred: this file was the only one read, and the page itself does not say that Chrome uses QUICHE.

**R2-TRN-F6.** Both Rust WebTransport stacks advertise the same draft-02 and draft-07 settings, and neither defines the -16
codepoint:
- wtransport-proto (master) defines `SETTINGS_ENABLE_WEBTRANSPORT: VarInt = VarInt::from_u32(0x2b60_3742)` and
  `SETTINGS_WEBTRANSPORT_MAX_SESSIONS: VarInt = VarInt::from_u32(0xc671_706a)`.
- web-transport-proto 0.6.3 has `enable_webtransport()`, which sends all of them, with the comments
  "ENABLE_DATAGRAM_DEPRECATED = 0xFFD277, // still used by Chrome", "// Removed in draft 06" (on 0x2b603742 and 0x2b603743) and
  "// TODO remove when 07 is in the wild".

Sources: https://raw.githubusercontent.com/BiagioFesta/wtransport/master/wtransport-proto/src/settings.rs;
https://docs.rs/web-transport-proto/latest/src/web_transport_proto/settings.rs.html. Tier: official (source code); the
"still used by Chrome" comment is the maintainer's claim (other). Read 2026-10-27. How sure: confirmed in the source.

### Browser support (VISION_V2 §4 targets)

**R2-TRN-F7.** All four target browsers ship WebTransport today, with versions and release dates from MDN's compatibility data:

| Browser | WebTransport since | Released | serverCertificateHashes since | Current today |
|---|---|---|---|---|
| Chrome | 97 | 2022-01-04 | 100 (2022-03-29) | 156 (2026-10-20) |
| Edge | "mirror" of Chrome | (as Chrome) | (as Chrome) | (as Chrome) |
| Firefox | 114 | 2023-06-06 | 125 (2024-04-16) | 158 (2026-10-13); ESR 153 (2026-07-21) |
| Safari (macOS; iOS "mirror") | 26.4 | 2026-03-24 | 26.4 | 27 (2026-09-14) |

MDN: "Baseline 2026" "Newly available" "Since March 2026, this feature works across the latest devices and browser versions." It
adds: "Some parts of this feature may have varying levels of support."
Sources: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransport.json (`"chrome": … "version_added": "97"`,
`"firefox": … "114"`, `"safari": … "26.4"`, `"edge": "mirror"`, `"safari_ios": "mirror"`, serverCertificateHashes "100" / "125" /
"26.4"); browsers/chrome.json, firefox.json, safari.json in the same repository (the release_date values quoted above);
https://developer.mozilla.org/en-US/docs/Web/API/WebTransport_API (last modified 2026-10-21). Tier: official. Read 2026-10-27. How
sure: confirmed in the source. Edge's Edge-specific version is not given: "mirror" means MDN copies Chrome's data.

**R2-TRN-F8.** Sending datagrams uses a different API per browser.
- Chrome and Edge have only `datagrams.writable`: `"chrome": { "version_added": "97" }`. MDN now marks it
  `"standard_track": false, "deprecated": true`, and Safari has `"version_added": false`.
- Safari has only the standard `datagrams.createWritable()`: `"safari": { "version_added": "26.4" }`, and Chrome has
  `"version_added": false`.
- Firefox has both: `writable` since 114 and `createWritable` since 155 (2026-09-01).

Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransportDatagramDuplexStream.json. Tier: official.
Read 2026-10-27. How sure: confirmed in the source (quoted JSON lines).

**R2-TRN-F9.** Connection statistics (`getStats()`) are missing on two of the four targets.
- Chrome: `"version_added": false`, `"impl_url": "https://crbug.com/41492543"`.
- Firefox: `"partial_implementation": true`, `"notes": "Method is defined but throws a not-implemented error."`
- Safari: "26.4".

So the browser cannot read its RTT or estimated send rate from the API on Chrome, Edge or Firefox today.
Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransport.json. Tier: official. Read 2026-10-27. How
sure: confirmed in the source for the data; the consequence is inferred.

**R2-TRN-F10.** The browser congestion-control hint is weak.
- The `congestionControl` option is `"version_added": false` in Chrome, and 114 / 26.4 in Firefox / Safari.
- MDN: "This is a hint to the user agent."
- W3C §6.9: "low-latency" is "considered a feature at risk due to the lack of implementation in browsers".

Sources: BCD WebTransport.json (as above); https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport (last modified
2026-10-12); https://www.w3.org/TR/webtransport/ §6.9. Tier: official. Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F11.** Chrome and Edge lack several newer options.
- `requireUnreliable` is `"version_added": false` in Chrome (Firefox 114, Safari 26.4).
- The fetch summary of the same data also showed these: `allowPooling`, stream `sendOrder` and `createSendGroup` false in Chrome;
  `protocols` in Chrome 143 (2025-12-02), Firefox 155 and Safari 26.4; `reliability` and `supportsReliableOnly` in Chrome 157,
  which is still beta (release 2026-11-03); Firefox's `createUnidirectionalStream` partial ("Returns a `WritableStream` instead of
  a `WebTransportSendStream`").
- Firefox ESR 153 (2026-07-21) is older than Firefox 155, so it lacks the 155 additions (`protocols`, `createWritable`,
  `createSendGroup`).

Source: BCD WebTransport.json and browsers/*.json. Tier: official. Read 2026-10-27. How sure: confirmed in the source for
requireUnreliable and the release dates; inferred (fetch summary) for the other rows.

**R2-TRN-F12.** Safari can run a WebTransport session over HTTP/2 and TCP, and the API says which kind a session got.
- WebKit's Safari 26.4 post: "WebTransport can run over HTTP/2 and TCP as a fallback with the same API". Such a session is
  reliable only. W3C §6.5: "If the connection is an HTTP/3 connection, set transport.[[Reliability]] to "supports-unreliable"",
  and "If the connection is an HTTP/2 connection" it becomes "reliable-only".
- `requireUnreliable` forbids the fallback. A session "cannot be established over an HTTP/2 connection if an HTTP/3 connection
  is not possible" (W3C §6.9; MDN: "By default, the value is `false`.").

Sources: https://webkit.org/blog/17862/ ("WebKit Features for Safari 26.4", 2026-03-24); https://www.w3.org/TR/webtransport/ §6.5,
§6.9; MDN WebTransport() constructor page. Tier: official. Read 2026-10-27. How sure: confirmed in the source.

### Certificates

**R2-TRN-F13.** Certificates pinned by hash (`serverCertificateHashes`) are tightly constrained. W3C §6.9:
- "the certificate MUST be an X.509v3 certificate";
- "the total length of the validity period MUST NOT exceed two weeks";
- "it MUST include ECDSA with the secp256r1 (NIST P-256) named group";
- "It MUST NOT contain RSA keys".

Only SHA-256 is checked. MDN: "This option is only supported for transports using dedicated connections (`allowPooling` is
`false`)."
Sources: https://www.w3.org/TR/webtransport/ §6.9; MDN WebTransport() constructor page. Tier: official. Read 2026-10-27. How sure:
confirmed in the source. Two details come from fetch summaries and are inferred: the SHA-256-only rule (§6.9/§14.4), and "no
interstitial" for certificate errors (§14.1).

**R2-TRN-F14.** Hash pinning is meant for hosts without a public certificate.
- MDN: "This feature allows developers to connect to WebTransport servers that would normally find it challenging to obtain a
  publicly trusted certificate, such as hosts that are not publicly routable, or ephemeral hosts like virtual machines."
- The hashes come from somewhere trusted: "The web application might typically fetch the hashes from a trusted intermediary."
- Without hashes, the normal web certificate check applies: "If empty, the user agent uses the same PKI certificate verification
  procedures it would use for a normal fetch operation."

Source: https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport. Tier: official. Read 2026-10-27. How sure:
confirmed in the source.

**R2-TRN-F15.** wtransport can create a certificate that meets these rules. `Identity::self_signed`: "This function uses the `ECDSA
P-256` algorithm." and "The total length of the validity period MUST NOT exceed two weeks." Its builder example uses
`.validity_days(14)`. This is the default `self-signed` feature (rcgen 0.14.5).
Sources: https://docs.rs/wtransport/latest/wtransport/struct.Identity.html (0.7.2);
https://raw.githubusercontent.com/BiagioFesta/wtransport/master/wtransport/Cargo.toml (`default = ["self-signed", "ring"]`). Tier:
official (docs, source). Read 2026-10-27. How sure: confirmed in the source.

### Datagrams and their size

**R2-TRN-F16.** QUIC datagrams are unreliable, congestion-controlled and cannot be fragmented. RFC 9221 (March 2022):
- §2: "QUIC datagrams are subject to QUIC congestion control."
- §5.4: "The sender MUST either delay sending the frame until the controller allows it" "or drop the frame without sending it".
- §5.2: "Although DATAGRAM frames are not retransmitted upon loss detection, they are ack-eliciting".
- §5: "DATAGRAM frames cannot be fragmented; therefore, application protocols need to handle cases" where data does not fit.
- §5.3: "DATAGRAM frames do not provide any explicit flow control signaling".

WebTransport over HTTP/3 carries its datagrams this way, and each one starts with a session prefix. Draft -16, §4.5: "the payload
field directly follows the Quarter Stream ID field, which is at the start of the QUIC DATAGRAM frame payload". §4.1: they "are not
retransmitted".
Sources: https://www.rfc-editor.org/rfc/rfc9221.html; https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 (-16). Tier:
official. Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F17.** The size floor is 1,200 bytes of UDP payload.
- RFC 9000 §14: "QUIC MUST NOT be used if the network path cannot support a maximum datagram size of at least 1200 bytes."
- quinn's `initial_mtu`: "Must be at least 1200, which is the default, and known to be safe for typical internet applications"
  (`min_mtu` also defaults to 1200; MTU discovery is "Enabled by default.").
- The usable application payload is smaller once the QUIC packet header, the frame header and the Quarter Stream ID are
  subtracted. No source read gives that exact number.

Sources: https://www.rfc-editor.org/rfc/rfc9000.html §14; https://docs.rs/quinn/latest/quinn/struct.TransportConfig.html (0.11.12).
Tier: official. Read 2026-10-27. How sure: confirmed in the source for the 1,200-byte rules; the smaller payload is inferred.

**R2-TRN-F18.** The browser's datagram size limit is not fixed, and a datagram over the limit vanishes without an error.
- W3C §5.1: "The maximum datagram size depends on the protocol that is in use." (for HTTP/3 it follows the path MTU estimate).
  "The user agent MAY update" it while connecting or connected.
- Over HTTP/2 "there is no equivalent limit."
- §4.3: when the limit "is less than data's [[ByteLength]]", writing returns "a promise resolved with undefined", and nothing is
  sent.

Source: https://www.w3.org/TR/webtransport/ §4.3, §5.1. Tier: official. Read 2026-10-27. How sure: confirmed in the source (quoted
fragments).

**R2-TRN-F19.** Browser datagram queues drop rather than block.
- Incoming: the queue has a limit "beyond which datagrams are dropped from the head of the queue" (§5.1).
- Outgoing: datagrams older than `outgoingMaxAge` are removed ("If more than duration milliseconds have passed since timestamp,
  then:", §4.3).
- `outgoingMaxAge` and `incomingMaxAge` exist in Chrome 97, Firefox 114 and Safari 26.4. The newer `*MaxBufferedDatagrams` limits
  exist only in Chrome 151 (Safari: preview; Firefox: none).

Sources: https://www.w3.org/TR/webtransport/; BCD WebTransportDatagramDuplexStream.json. Tier: official. Read 2026-10-27. How sure:
confirmed in the source for the W3C rules; the per-browser rows are inferred (fetch summary).

**R2-TRN-F20.** Fiedler on oversized and fragmented packets. Games have seen oversized packets where "suddenly all packets start
getting dropped!"; a router "has the option of a) fragmenting that packet, or b) dropping the packet." He gives a conservative
bound of "1200 bytes" and splits larger packets into "1024 byte fragments". Losing any fragment loses the whole packet, so loss
compounds with the fragment count (his example, per the fetch summary: 1 % loss over 100 fragments is about a 63.4 % chance of
losing the packet).
Source: https://gafferongames.com/post/packet_fragmentation_and_reassembly/ ("Packet Fragmentation and Reassembly", Glenn
Fiedler, 2016-09-06). Tier: shipped-game practitioner. Read 2026-10-27. How sure: confirmed in the source for the quoted fragments;
the 63.4 % figure is inferred.

### Streams, ordering and head-of-line blocking

**R2-TRN-F21.** WebSocket runs over TCP, so one lost packet stalls everything behind it. WebTransport streams do not block each
other. WebKit, about WebSockets: "head-of-line blocking can cause delays when packets are lost". About WebTransport: "It runs
over HTTP/3 and QUIC, providing multiple streams that won't block each other."
Source: https://webkit.org/blog/17862/. Tier: official (vendor). Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F22.** Fiedler, from shipped-game practice, on TCP for real-time games:
- "whenever a packet is dropped it has to stop and wait for that data to be resent";
- "On the client game objects stop receiving updates so they appear to be standing still";
- "My recommendation is not only that you use UDP, but that you only use UDP for your game protocol";
- "TCP tends to induce packet loss in UDP packets", hence "Don't mix TCP and UDP!".

The article dates from 2008, before QUIC. The mixing warning is about running TCP and UDP flows side by side.
Source: https://gafferongames.com/post/udp_vs_tcp/ ("UDP vs. TCP", Glenn Fiedler, 2008-10-01). Tier: shipped-game practitioner. Read
2026-10-27. How sure: confirmed in the source.

**R2-TRN-F23.** Priorities within a session are up to the application.
- Draft -16 §3.4: "WebTransport does not provide any priority signaling mechanism for streams and datagrams" within a session.
  RFC 9218 scheduling is "RECOMMENDED".
- The W3C API adds `sendOrder` and `sendGroup` on streams, where higher send order "MUST starve" lower within a group. Per the
  BCD summary, Chrome lacks `sendOrder` (Firefox 119, Safari 26.4).

Sources: https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 §3.4; https://www.w3.org/TR/webtransport/; BCD. Tier:
official. Read 2026-10-27. How sure: confirmed in the source for the quoted fragments; the browser rows are inferred.

### Congestion control

**R2-TRN-F24.** QUIC's standard congestion control can shrink the send window to two packets. RFC 9002 (May 2021), §7:
- "This document specifies a sender-side congestion controller for QUIC similar to TCP NewReno."
- "A sender can unilaterally choose a different algorithm to use, such as CUBIC".
- §7.7: "A sender SHOULD pace sending of all in-flight packets based on input from the congestion controller."
- §7.6: after persistent congestion, "the sender's congestion window MUST be reduced to the minimum congestion window
  (kMinimumWindow)", where §7.2 says "The RECOMMENDED value is 2 * max_datagram_size".
- §7.2: the initial window is "ten times the maximum datagram size (max_datagram_size)," "while limiting the window to the larger
  of 14,720 bytes or twice the maximum datagram size."

Source: https://www.rfc-editor.org/rfc/rfc9002.html. Tier: official. Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F25.** The server picks its own congestion controller in quinn 0.11.12.
- The `congestion` module offers Cubic ("The RFC8312 congestion controller, as widely used for TCP"), NewReno ("A simple,
  standard congestion controller") and Bbr ("Experimental! Use at your own risk."), behind pluggable `Controller` and
  `ControllerFactory` traits. The default is not stated there, and the `TransportConfig` example uses `NewRenoConfig`.
- `Connection::send_datagram`: "Transmit `data` as an unreliable, unordered application datagram"; "They may be lost or delivered
  out of order".
- `send_datagram_wait` "will wait for buffer space during congestion conditions".
- `rtt()`: "Current best estimate of this connection's latency (round-trip-time)".
- `congestion_state` exists "for debugging purposes".

Sources: https://docs.rs/quinn/latest/quinn/congestion/index.html; https://docs.rs/quinn/latest/quinn/struct.Connection.html;
https://docs.rs/quinn/latest/quinn/struct.TransportConfig.html (all 0.11.12). Tier: official. Read 2026-10-27. How sure: confirmed in
the source.

**R2-TRN-F26.** All WebTransport sessions on one connection share its congestion control. Draft -16 §8: "all of the WebTransport
sessions share both congestion control and flow control context". Datagrams and streams on one connection therefore compete for
the same window.
Source: https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 (-16) §8. Tier: official. Read 2026-10-27. How sure:
confirmed in the source for the quote; the competition is inferred.

**R2-TRN-F27.** The W3C stats dictionary defines `smoothedRtt`, `rttVariation`, `minRtt`, `estimatedSendRate` and
`atSendCapacity`, plus datagram counters (`droppedIncoming`, `expiredIncoming`, `expiredOutgoing`, `lostOutgoing`).
`bytesAcknowledged` is at risk.
Source: https://www.w3.org/TR/webtransport/ (stats section). Tier: official. Read 2026-10-27. How sure: inferred (fetch summary).
Usable only on Safari today (F9).

### Rust crates (versions on 2026-10-27)

**R2-TRN-F28.** quinn, the QUIC base both WebTransport crates build on, is mature and active: `"newest_version":"0.11.12"`,
`"updated_at":"2026-09-14T10:08:47.662258Z"`, `"created_at":"2018-10-02T20:18:56.824108Z"`, `"downloads":379896223`.
Sources: https://crates.io/api/v1/crates/quinn; docs.rs quinn 0.11.12. Tier: official (registry). Read 2026-10-27. How sure:
confirmed in the source.

**R2-TRN-F29.** wtransport 0.7.2: `"newest_version":"0.7.2"`, `"updated_at":"2026-08-11T22:08:08.540291Z"`,
`"created_at":"2023-05-25T19:45:52.658071Z"`, `"downloads":1083504`.
- Cargo.toml: `rust-version = "1.88"`, quinn 0.11.6 with runtime-tokio, rustls 0.23.23, rcgen 0.14.5 optional, and features
  ring (default), aws-lc-rs, fips.
- Its README: "The *WTransport* library, while functional, is not considered completely production-ready." "It should be used
  with caution and may undergo changes as the WebTransport specification evolves."
- Its API offers `send_datagram`, `receive_datagram`, `max_datagram_size`, `rtt`, open and accept for both stream kinds, and
  `quic_connection()` ("Returns a reference to the inner QUIC connection.", feature `quinn`), so quinn's stats and congestion
  state stay reachable.
- The GitHub page shows 716 stars and 29 open issues (fetch summary).

Sources: https://crates.io/api/v1/crates/wtransport; https://raw.githubusercontent.com/BiagioFesta/wtransport/master/wtransport/Cargo.toml;
https://raw.githubusercontent.com/BiagioFesta/wtransport/master/README.md; https://docs.rs/wtransport/latest/wtransport/struct.Connection.html
(0.7.2); https://github.com/BiagioFesta/wtransport. Tier: official (registry, source, docs); README and repository page: other
(the maintainer's own words). Read 2026-10-27. How sure: confirmed in the source, except the star and issue counts (inferred).

**R2-TRN-F30.** web-transport-quinn 0.13.5 belongs to a family of crates.
- Registry: `"newest_version":"0.13.5"`, `"updated_at":"2026-10-24T00:53:07.167572Z"`,
  `"created_at":"2024-04-06T05:33:00.667490Z"`, `"downloads":2033705`.
- It moves fast. The registry summary lists six releases from 0.13.0 (2026-09-27) to 0.13.5, and 0.12.0 on 2026-08-07. Whether
  these minor bumps broke the API was not checked (no changelog read).
- README:
  - "web-transport-quinn mirrors the Quinn API, abstracting away the HTTP/3 setup."
  - "web-transport-wasm wraps the browser API".
  - "web-transport-trait defines an async trait, currently implemented by web-transport-quinn and qmux."
  - "qmux implements QMux (draft-ietf-quic-qmux) over TCP/TLS/WebSocket, with backwards compatibility" "for the legacy
    WebTransport-over-WebSocket wire format."
- The README makes no stability statement.

Sources: https://crates.io/api/v1/crates/web-transport-quinn; https://raw.githubusercontent.com/moq-dev/web-transport/main/README.md;
https://github.com/moq-dev/web-transport. Tier: official (registry); README: other. Read 2026-10-27. How sure: confirmed in the source
for the newest version and the README quotes; inferred for the release list.

**R2-TRN-F31.** The h3 route is experimental and has gone quiet.
- h3's README: "The `h3` crate is still very experimental."
- h3-webtransport: `"newest_version":"0.1.2"`, `"updated_at":"2025-05-06T21:21:35.491571Z"`, three releases since 2023-11-06.

Sources: https://github.com/hyperium/h3; https://crates.io/api/v1/crates/h3-webtransport. Tier: other (README); official
(registry). Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F32.** QMux is an early IETF draft that offers QUIC's operations over a TCP-like stream. draft-ietf-quic-qmux-02
(2026-07-06) is a "WG Document" of the quic WG, intended Proposed Standard. Abstract: "QMux version 1 provides, over
bi-directional streams such as TLS," "the same set of stream and datagram operations that applications rely upon in QUIC version
1."
Source: https://datatracker.ietf.org/doc/draft-ietf-quic-qmux/. Tier: official. Read 2026-10-27. How sure: confirmed in the source for
the abstract; the state fields are inferred.

**R2-TRN-F33.** The WebSocket crates are mature and widely used.
- tokio-tungstenite: `"newest_version":"0.30.0"`, `"updated_at":"2026-07-11T12:58:44.438987Z"`,
  `"created_at":"2017-03-17T17:27:49.033103Z"`, `"downloads":330093365`.
- axum: `"newest_version":"0.8.9"`, `"updated_at":"2026-04-14T07:55:20.258398Z"`, `"downloads":548097362`. It has a WebSocket
  extractor: "Handle WebSocket connections.", "Available on crate feature `ws` only.", and its dependency list includes
  tokio-tungstenite ^0.29.0.

Sources: https://crates.io/api/v1/crates/tokio-tungstenite; https://crates.io/api/v1/crates/axum;
https://docs.rs/axum/latest/axum/extract/ws/index.html (0.8.9). Tier: official. Read 2026-10-27. How sure: confirmed in the source;
"axum's ws runs on tokio-tungstenite" is inferred from the dependency list.

### Browser platform and protocol details

**R2-TRN-F34.** WebSocket has been in all four browsers for over a decade and works in workers. Top-level `"version_added"`:
chrome "5", edge "12", firefox "11" (with the `"Moz"` prefix from 7 to 11), safari "5". `worker_support`: chrome "5", firefox
"37", safari "5". The fetch summary adds binaryType since Chrome 15, Firefox 11 and Safari 6.
Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebSocket.json. Tier: official. Read 2026-10-27. How sure:
confirmed in the source (quoted JSON); binaryType inferred.

**R2-TRN-F35.** WebTransport runs in Web Workers and needs HTTPS. MDN: "Note: This feature is available in Web Workers."; "This
feature is available only in secure contexts (HTTPS)"; "Note that the scheme needs to be HTTPS, and the port number needs to be
explicitly specified."
Source: https://developer.mozilla.org/en-US/docs/Web/API/WebTransport_API. Tier: official. Read 2026-10-27. How sure: confirmed in the
source.

**R2-TRN-F36.** The server must check where a connection comes from and may cut off abuse.
- Draft -16 §3.2: "the WebTransport server MUST verify the Origin header to ensure that the specified origin is allowed", and
  should answer 403 otherwise.
- §8: "Implementations SHOULD track the use of WebTransport features", and an endpoint "MAY treat activity that is suspicious as
  a connection error of type H3_EXCESSIVE_LOAD".

Source: https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 (-16). Tier: official. Read 2026-10-27. How sure: confirmed in
the source (quoted fragments).

**R2-TRN-F37.** Sessions can be drained before they close. Draft -16 §6 defines WT_CLOSE_SESSION (0x2843) and WT_DRAIN_SESSION
(0x78ae): "either endpoint can send a WT_DRAIN_SESSION (0x78ae) capsule". The W3C `draining` promise exists in Firefox 155 and
Safari 26.4, not in Chrome.
Sources: draft -16 §6; BCD WebTransport.json. Tier: official. Read 2026-10-27. How sure: confirmed in the source for the draft
quote; inferred for the browser rows.

**R2-TRN-F38.** The `protocols` option can carry the application protocol version. MDN: "The server may also reject the connection
if it doesn't support any of the protocols, in which case `ready` will reject with an error." Browser support: Chrome 143,
Firefox 155, Safari 26.4.
Sources: MDN WebTransport() constructor page; BCD. Tier: official. Read 2026-10-27. How sure: confirmed in the source for MDN; inferred
for the versions.

**R2-TRN-F39.** QUIC connections can survive a change of client address. RFC 9000 §1: QUIC "allows connections to continue after
changes in network topology or address mappings"; §5.1: "The primary function of a connection ID is to ensure that changes in
addressing at lower protocol layers" do not misroute packets.
Source: https://www.rfc-editor.org/rfc/rfc9000.html. Tier: official. Read 2026-10-27. How sure: confirmed in the source for the
fragments.

## Legacy facts

- **CS1** (fixed tick and input-command groundwork, `shamble-core.js:1479-1481`; marked **drop**). The legacy build declares an
  input command (seq, tick, move, aim, buttons bitmask) but never sends or reads it. There is no networking code at all. For v2,
  the transport carries no legacy format. Its smallest and most frequent message is the per-tick input command, which belongs in
  an unreliable datagram (it is small and the next one supersedes it). The format itself belongs to the snapshots and
  prediction unit.
- **GP2** (input mapping and listeners; `shamble-game.js:3252-3294`). Many actions fire at event time (reload, ability, grenade
  release, bow release, E, weapon switch). The inventory's v2 note turns them into per-tick input bits and events. For the
  transport this means one-shot actions must survive the loss of a datagram. Either they ride as bits repeated in every
  redundant input copy, or they go on a reliable stream. Which one is a protocol choice for the prediction unit.
- **CS11** (headless SIM fence). The v2 simulation is a Rust crate with no browser dependency, so the transport stays outside
  the simulation. It decodes messages into simulation inputs and encodes snapshots out, and the simulation never touches a
  socket.
- **STATUS.md:70, ADR-001 inputs.** Three of them come in over the transport's reliable messages: humans take over bot slots,
  the server repeats the team allow-list (`teamOf`), and map data needs a full schema check on the server (SEC-MDL-2). So
  reliable messages from clients need server-side validation. No transport measurement exists in the legacy build.

## Options

### Q1. The WebTransport server stack in Rust

| Option | What it is | Complexity and effort | Maturity and maintenance | Draft and interop | Risk | Fit with VISION_V2 |
|---|---|---|---|---|---|---|
| A. wtransport 0.7.x on quinn | Its own session, stream and datagram API; self-signed certificate helper; access to the inner quinn connection | Low: one crate, tokio | Since 2023-05; 0.7.2 (2026-08-11); about 1.08 M downloads; README "not considered completely production-ready" (F29) | Draft-02 and -07 settings (F6) | Pre-1.0 API; one project; lags when browsers move to a new draft | Serves the rooms and authoritative server of §6.1 |
| B. web-transport-quinn 0.13.x on quinn | Mirrors the quinn API; sibling crates: a browser wrapper, an async trait, qmux over WebSocket | Low to medium | Since 2024-04; 0.13.5 (2026-10-24); about 2.03 M downloads; six releases in four weeks; no stability statement (F30) | Draft-02 and -07 settings (F6) | Frequent pre-1.0 releases (API stability not checked); its own trait and qmux are young | Same; the trait and qmux could cover the fallback too (Q4) |
| C. h3 + h3-quinn + h3-webtransport | hyperium's HTTP/3 with a WebTransport extension | Medium: lower level | h3 "still very experimental"; h3-webtransport 0.1.2, idle since 2025-05 (F31) | Not checked | Highest: experimental and idle | Same; nothing extra |
| D. Our own minimal WebTransport handshake on quinn | Write the HTTP/3 SETTINGS and CONNECT exchange ourselves (as web-transport-proto does) | High: protocol work and upkeep on every draft change | Ours | Whatever we write | High effort for no game-specific gain | Weak: an engine feature the game does not need |

All four run on quinn (0.11.12, mature, F28), whose congestion control the server can choose (F25).

### Q2. The fallback when WebTransport fails

| Option | What it is | Pros | Cons |
|---|---|---|---|
| A. Binary WebSocket (tokio-tungstenite, directly or through axum's `ws`) | A second, TCP-based transport into the same server, carrying the same messages | Mature crates (F33); in all four browsers since long ago, also in workers (F34); simple | Head-of-line blocking: one loss stalls every later snapshot (F21, F22); no unreliable channel |
| B. The browser's own WebTransport over HTTP/2 (Safari 26.4) | The same WebTransport API over TCP (F12) | No second client code path | Needs server support for draft-ietf-webtrans-http2 (still a draft, F3). None of the Rust crates read claims it (absence: unverified). Only Safari is documented as doing it. Still reliable only, with the same head-of-line blocking |
| C. qmux over WebSocket (web-transport's `qmux`) | QUIC-style streams and datagrams carried over a WebSocket (F30, F32) | One stream and datagram API over both transports | QMux is draft -02, a WG document (F32); young crate; "datagrams" over TCP are still head-of-line blocked |
| D. No fallback | WebTransport only | Simplest | Players on networks where "the underlying network environment doesn't support QUIC" (WebKit, F12) cannot play; the brief names the fallback (§0) |

### Q3. Mapping the game's messages to datagrams and streams

| Message | Option A: datagram | Option B: stream | Notes |
|---|---|---|---|
| Input commands (client to server, every tick) | Yes: small, newest wins; repeat the last few in each datagram to survive loss (F16: no retransmission) | No: a lost packet would delay every later input (head-of-line blocking within a stream, F21) | Legacy CS1 and GP2 |
| Snapshots (server to client, 20–30 Hz) | Yes, if each fits under the size limit (F17, F18) and is decodable on its own; deltas are built against the last acknowledged snapshot, so a lost one costs nothing extra | Only over the WebSocket fallback | Splitting one snapshot into fragments multiplies loss (F20). Split by entity group into independent datagrams instead |
| Reliable messages (join, match setup, team and slot changes, purchases, chat, end of match) | No | Yes: one bidirectional control stream per session, validated by the server (STATUS.md:70) | Use priorities where supported (F23) |
| Bulk data (map definition, tuning tables at join) | No | A separate unidirectional stream, so it never blocks the control stream (F21) | Chrome lacks `sendOrder` (F11, F23) |

### Q4. One transport interface over both

- **A. Our own thin interface**, written twice. The client side is TypeScript (one implementation on WebTransport, one on
  WebSocket); the server side is a Rust trait or enum. It needs four operations: send unreliable, send reliable on a named
  channel, receive, and a link-state or RTT probe. The client must feature-detect the datagram writer (F8). Smallest, fully
  ours, game-specific.
- **B. web-transport-trait plus qmux.** These cover the server side (F30) and give real stream semantics over WebSocket. They
  also tie us to a fast-moving 0.x family and an early draft (F32).
- **C. A WebTransport polyfill over WebSocket in the browser.** The only lead (webtransport-ponyfill-websocket, named on the old
  Chrome page) was not read. Not assessed.

### Q5. Certificates

- **Development: self-signed certificates pinned by hash** (`serverCertificateHashes`): ECDSA P-256, at most 14 days, SHA-256
  hash (F13, F15). Supported in Chrome 100, Firefox 125 and Safari 26.4 (F7). Pooling is not allowed (F13).
- **Production option A: a publicly trusted certificate** for each game server hostname. The browser does its normal check
  (F14). Issuance and renewal belong to R6.
- **Production option B: hash pinning in production.** MDN names ephemeral virtual machines as the use case. The hashes come
  from a trusted intermediary, here our HTTPS matchmaker (F14). The certificate must be re-issued every 14 days or sooner, and
  any cached hash invalidated (F13).

### Q6. Congestion control for a real-time stream

- Datagrams obey the connection's congestion controller and are delayed or dropped when the window is full (F16). The browser
  hint is a no-op in Chrome, and "low-latency" is at risk (F10). The window can fall to 2 × max_datagram_size after persistent
  congestion (F24). What a real-time stream can control is on our side:
  - **A. Server controller choice** in quinn: NewReno, Cubic, or experimental BBR (F25).
  - **B. Drop stale data instead of queueing.** Never wait for send space for a snapshot: use `send_datagram`, not
    `send_datagram_wait` (F25). Use a short `outgoingMaxAge` for client inputs (F19).
  - **C. Adapt the rate.** Lower snapshot rate or size when quinn's RTT or loss rises. The policy belongs to the tick and
    bandwidth unit.

## Recommendations

**R2-TRN-REC1. Build the server on quinn 0.11.x. Pick the WebTransport layer by spike between wtransport 0.7.x and
web-transport-quinn 0.13.x, behind our own thin session interface. Rule out h3-webtransport and a hand-written handshake.**
Reason:
- Both candidates sit on the same mature QUIC stack (F28) and speak the same draft-02 and -07 settings as QUICHE (F5, F6).
- Neither claims production readiness, and each has a different risk: wtransport says it is not production-ready (F29);
  web-transport-quinn moves fast in 0.x (F30). A thin interface keeps a later swap cheap.
- h3 is "still very experimental" and h3-webtransport is idle (F31).
- Option D is generic protocol work no VISION_V2 §6 item asks for.

Rests on: F4, F5, F6, F25, F28, F29, F30, F31.
Serves: VISION_V2 §6.1 (rooms and private matches need a client-to-server link); §6.5 (economy server-side), §6.6 (telemetry
recorded server-side) and §6.8 (moments recognized server-side), which all assume an authoritative server reached over this
transport; and the §4 targets (four browsers, fair to about 100 ms).
What a spike must prove (proposed pass marks):
1. Interop: Chrome 156, Edge 156, Firefox 158 and Safari 27 (macOS) each open a WebTransport session to the Rust server with
   each candidate crate, and exchange datagrams plus one bidirectional stream. At least 99 of 100 connection attempts succeed per
   browser on a clean local network (proposed). Record the negotiated draft per browser; this answers OQ1.
2. Load: one server process holds 48 WebTransport sessions. Each sends 60 input datagrams/s (≤ 100 bytes) and receives 30
   snapshot datagrams/s (≤ 1,000 bytes; placeholder numbers until the tick and bandwidth unit sets them). Proposed limits: at
   most 1 CPU core for the transport layer, and p99 server send-call time ≤ 1 ms, on reference server hardware R6 names
   (proposed).
3. Swap cost: the thin interface hides the crate. Switching crates in the spike changes no code outside the transport module
   (checked by diff).

**R2-TRN-REC2. Send inputs and snapshots as datagrams, and reliable traffic on streams. Cap each snapshot datagram at a proposed
1,000 bytes of payload and make every datagram decodable on its own (no fragmenting of one snapshot). Put reliable control on
one bidirectional stream and bulk data on its own unidirectional stream.**
Reason:
- Datagrams are never retransmitted and never block one another, which suits newest-wins data (F16, F21).
- The guaranteed floor is 1,200 bytes of UDP payload minus headers (F17). An oversized datagram vanishes silently (F18). Fiedler
  shows fragment loss compounds (F20).
- Separate streams keep a big map download from delaying a slot change (F21).
- Input redundancy covers loss of one-shot actions (GP2).

Rests on: F16, F17, F18, F19, F20, F21, F22.
Serves: §6.1 (the per-mode match flow over the control stream); the §4 fairness target (no head-of-line stalls on the
WebTransport path); the 48+ design point of VISION_V2 §3.
What a spike must prove (proposed pass marks):
1. `maxDatagramSize` is recorded at session start and every 10 s on each of the four browsers. Pass: ≥ 1,000 bytes on all four,
   on loopback and on a real EU path; otherwise the cap drops to the smallest value measured.
2. Under a network simulator at 100 ms RTT, ±20 ms jitter and 2 % loss, the share of snapshots lost or late (arriving more than
   two snapshot intervals after sending) is at most the injected loss + 1 percentage point. Proposed.
3. With 5 % loss, no one-shot action is lost: 0 dropped out of 10,000 scripted actions, whether through input redundancy or
   the reliable stream.
4. A 1 MB bulk transfer on its own stream during play adds ≤ 5 ms to the p99 delay of control-stream messages. Proposed.

**R2-TRN-REC3. Fall back to binary WebSocket (tokio-tungstenite 0.30.x, or axum 0.8.x `ws` if the server already serves HTTP
through axum). Choose one transport per client, never both at once. Ask for `requireUnreliable: true` where the browser supports
it. Do not serve WebTransport over HTTP/2.**
Reason:
- Some networks don't support QUIC (F12). WebSocket is everywhere, including workers (F34), and its crates are mature (F33).
- Safari's own HTTP/2 fallback would need a server-side draft implementation that none of the crates read offers (F3, F12),
  and it is reliable-only anyway.
- `requireUnreliable` makes a WebTransport session fail cleanly, so our WebSocket path takes over (F12; Chrome ignores the
  option, F11).
- Running TCP and UDP side by side hurts the UDP flow (F22).
- Over WebSocket, the server must send only the newest snapshot. It drops any unsent older one, so stalls do not stack up
  (F21, F22).

Rests on: F3, F11, F12, F21, F22, F33, F34.
Serves: the §4 browser target (all four browsers, on all networks); §6.1.
What a spike must prove (proposed pass marks):
1. With UDP blocked to the server, each of the four browsers reaches a playing WebSocket session ≤ 3 s after the page starts
   connecting. Proposed: WebTransport attempt timeout 1.5 s, then WebSocket.
2. The same recorded input replay drives both transports to identical server state hashes on every tick (the per-tick state
   hash of the default principles).
3. At 100 ms RTT and 1 % loss over WebSocket: report the head-of-line stall distribution. Proposed pass: p99 extra snapshot delay
   ≤ 250 ms, and the server never holds more than one unsent snapshot per client.

**R2-TRN-REC4. Use our own thin transport interface on both sides, with two backends each, and keep it outside the simulation.**
- Client (TypeScript): WebTransport and WebSocket backends. Feature-detect the datagram writer (`createWritable()` on Safari and
  Firefox 155+, `writable` on Chrome and Edge). Measure RTT with our own ping messages: `getStats()` is missing on Chrome and Edge
  and throws on Firefox.
- Server (Rust): one session type per backend, feeding the same per-client inbound and outbound queues.
- Carry our protocol version in the `protocols` option where supported (Chrome 143+, Firefox 155+, Safari 26.4) and in the
  first control message everywhere else.

Reason:
- The browser API differs today (F8, F9, F11, F38). The simulation must not depend on any I/O (CS11).
- web-transport-trait and qmux are young (F30, F32). Four operations do not justify a dependency. The question stays open
  (OQ6).

Rests on: F8, F9, F11, F30, F32, F35, F38.
Serves: §6.1; R1's simulation in a Web Worker (WebTransport and WebSocket both work in workers, F34, F35).
What a spike must prove (proposed pass marks):
1. One client build runs unchanged on all four browsers over both backends. The feature-detection path is covered by a test per
   browser in the end-to-end suite (R5).
2. App-level RTT agrees with quinn's server-side `rtt()` within ±5 ms at 100 ms injected RTT. Proposed.
3. The simulation crate has no dependency on the transport crates (checked with `cargo tree`).

**R2-TRN-REC5. Certificates.**
- Development and previews: self-signed ECDSA P-256 certificates valid ≤ 14 days, made by the server at start (wtransport's
  `Identity::self_signed`, or rcgen with web-transport-quinn), pinned through `serverCertificateHashes`. The hash is fetched from
  the page's own HTTPS origin.
- Production: a publicly trusted certificate per game server hostname. Hash pinning in production stays an option, only if R6
  picks short-lived ephemeral servers.

Reason:
- The pinning rules (X.509v3, ≤ 2 weeks, ECDSA P-256, no RSA, no pooling) are strict, and every target supports them (F7, F13).
- MDN frames pinning as the tool for non-public and ephemeral hosts, with hashes from a trusted intermediary (F14).
- wtransport already generates compliant certificates (F15).

Rests on: F7, F13, F14, F15.
Serves: §6.1 (private matches on real servers); the §4 browsers.
What a spike must prove (proposed pass marks):
1. All four browsers connect with a pinned self-signed certificate, and all four with a publicly trusted one: 100 % of 20
   attempts each.
2. Certificate rotation with 48 live sessions: 0 sessions dropped, and new connections succeed with the new hash within one
   reload.
3. A certificate valid for 15 days is rejected by all four browsers (a negative test of the rule).

**R2-TRN-REC6. Treat congestion control as ours to manage on the server.**
- Start with quinn's NewReno or Cubic. Pick between them by measurement; BBR is ruled out while quinn labels it experimental.
- Never block the simulation tick on a send, and never queue stale snapshots: always the non-waiting `send_datagram`, with a
  check of `datagram_send_buffer_space`.
- Give client input datagrams a short `outgoingMaxAge`, proposed 100 ms.
- Feed quinn's RTT and loss figures into the tick and bandwidth unit's rate adaptation.

Reason:
- Datagrams are congestion-controlled and are dropped or delayed under pressure (F16).
- The browser hint does nothing in Chrome, and "low-latency" is at risk (F10).
- The window can collapse to two packets (F24). All streams and datagrams share it (F26).
- quinn offers a pluggable controller and the non-blocking send (F25).

Rests on: F10, F16, F19, F24, F25, F26.
Serves: the §4 fairness target (about 100 ms) at the §3 scale (48+ players).
What a spike must prove (proposed pass marks):
1. With a bottleneck set below the snapshot stream's rate (proposed: 50 % of it), the newest snapshot the client holds is never
   older than 3 snapshot intervals. Server memory per session stays flat (no queue growth over a 10-minute soak).
2. Simulation tick duration at p99 is unchanged (within 5 %) between a clean network and the bottleneck run.
3. NewReno and Cubic are compared under the REC2 netem profile. Pick the one with the lower p99 snapshot age; record the numbers.

## Open questions

**R2-TRN-OQ1. Which WebTransport draft does each browser negotiate today, and when do they move to -16?** Only Chromium's QUICHE
constants were read (F5). Nothing read says what Firefox 158 or Safari 27 send. Both Rust crates speak the -02/-07 settings (F6).
If a browser drops them before the crates add -16's `SETTINGS_WT_ENABLED`, connections fail. Decided by: the REC1 interop
spike (log the negotiated settings), plus a recurring test against Chrome Beta, Firefox Nightly and Safari Technology Preview
(R5's CI).

**R2-TRN-OQ2. Does Chrome or Firefox fall back to WebTransport over HTTP/2 the way Safari does, and does any Rust crate serve it?**
The sources read document only Safari's fallback (F12). Chrome lacks `requireUnreliable` (F11), so its behaviour without UDP is
unknown. Decided by: the REC3 spike with UDP blocked, and the crates' own documentation (none of the pages read mentions HTTP/2).

**R2-TRN-OQ3. What `maxDatagramSize` do the four browsers report on real paths?** The spec lets it vary and change (F18). Decided
by: the REC2 measurement.

**R2-TRN-OQ4. Production certificates: public certificates per hostname or short-lived pinned ones?** This depends on how R6
hosts game servers (fixed hosts or ephemeral machines) and on the HTTPS matchmaker as the trusted intermediary (F14). Decided by:
R6's hosting choice, then the REC5 spike.

**R2-TRN-OQ5. wtransport or web-transport-quinn?** The maintenance risk differs: "not considered completely production-ready"
(F29) versus fast 0.x churn (F30). Contributor counts and issue response times were not read. Decided by: the REC1 spike pass
marks, plus a short maintenance check by rust-reviewer (releases in the last 6 months, open security issues on rustsec.org).

**R2-TRN-OQ6. Is qmux worth adopting as the fallback framing later?** It would give real streams over WebSocket and one API over
both (F30, F32), but it is draft -02. Decided by: whether REC4's thin interface needs more than one ordered channel over WebSocket
in the spike. If it does, re-assess qmux.

**R2-TRN-OQ7. How many players will need the WebSocket fallback?** No source read gives a share of networks that block UDP or
QUIC. Decided by: connection telemetry once a playtest runs (R6 monitoring). This sizes how good the fallback must feel.

**R2-TRN-OQ8. Does the browser's own congestion controller limit client inputs in practice?** Inputs are small, but they are
congestion-controlled too (F16), and Chrome ignores the hint (F10). Decided by: the REC6 netem spike, measured on the client
side.

**R2-TRN-OQ9. Valve's Source networking page could not be read (HTTP 403).** It was not needed for any transport
recommendation, and the snapshots and prediction unit covers it.

## Sources

### Sources read (all read 2026-10-27)
1. https://www.w3.org/TR/webtransport/ "WebTransport", W3C Candidate Recommendation Snapshot, 30 July 2026 (four fetches: status,
   §4.3, §5.1, §6.5, §6.9, §14, references).
2. https://datatracker.ietf.org/doc/draft-ietf-webtrans-http3/ and /history/ "WebTransport over HTTP/3", -16, 2026-07-06.
3. https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 (text of -16, 2026-07-06).
4. https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3-02 (2021-10-25).
5. https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3-07 (2023-06-13).
6. https://datatracker.ietf.org/doc/draft-ietf-webtrans-http2/ "WebTransport over HTTP/2", -15, 2026-07-06.
7. https://datatracker.ietf.org/doc/draft-ietf-quic-qmux/ "QMux", -02, 2026-07-06.
8. https://www.rfc-editor.org/rfc/rfc9221.html RFC 9221 "An Unreliable Datagram Extension to QUIC", March 2022.
9. https://www.rfc-editor.org/rfc/rfc9000.html RFC 9000 "QUIC: A UDP-Based Multiplexed and Secure Transport", May 2021 (§1, §5.1,
   §14).
10. https://www.rfc-editor.org/rfc/rfc9002.html RFC 9002 "QUIC Loss Detection and Congestion Control", May 2021 (§7).
11. https://developer.mozilla.org/en-US/docs/Web/API/WebTransport_API "WebTransport API", MDN, last modified 2026-10-21.
12. https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport "WebTransport() constructor", MDN, last modified
    2026-10-12.
13. https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransport.json (MDN browser-compat-data, main).
14. https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransportDatagramDuplexStream.json (main).
15. https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebSocket.json (main).
16. https://raw.githubusercontent.com/mdn/browser-compat-data/main/browsers/chrome.json, firefox.json, safari.json (main; release
    dates).
17. https://webkit.org/blog/17862/ "WebKit Features for Safari 26.4", 2026-03-24.
18. https://developer.chrome.com/docs/capabilities/web-apis/webtransport "Using WebTransport", page dated 2020-06-08 (old; read
    but not cited for any finding).
19. https://raw.githubusercontent.com/google/quiche/main/quiche/quic/core/http/http_constants.h (QUICHE main).
20. https://crates.io/api/v1/crates/wtransport (0.7.2).
21. https://github.com/BiagioFesta/wtransport (repository page).
22. https://raw.githubusercontent.com/BiagioFesta/wtransport/master/README.md (master).
23. https://raw.githubusercontent.com/BiagioFesta/wtransport/master/wtransport/Cargo.toml (0.7.2).
24. https://raw.githubusercontent.com/BiagioFesta/wtransport/master/wtransport-proto/src/settings.rs (master).
25. https://docs.rs/wtransport/latest/wtransport/ (0.7.2, page 2026-10-15).
26. https://docs.rs/wtransport/latest/wtransport/struct.Identity.html (0.7.2).
27. https://docs.rs/wtransport/latest/wtransport/struct.Connection.html (0.7.2).
28. https://crates.io/api/v1/crates/quinn (0.11.12).
29. https://docs.rs/quinn/latest/quinn/congestion/index.html (0.11.12).
30. https://docs.rs/quinn/latest/quinn/struct.Connection.html (0.11.12).
31. https://docs.rs/quinn/latest/quinn/struct.TransportConfig.html (0.11.12).
32. https://crates.io/api/v1/crates/web-transport-quinn (0.13.5).
33. https://github.com/moq-dev/web-transport (repository page).
34. https://raw.githubusercontent.com/moq-dev/web-transport/main/README.md (main).
35. https://docs.rs/web-transport-proto/latest/web_transport_proto/ , struct.Setting.html and src/web_transport_proto/settings.rs.html
    (0.6.3).
36. https://crates.io/api/v1/crates/h3-webtransport (0.1.2).
37. https://github.com/hyperium/h3 (repository page).
38. https://crates.io/api/v1/crates/tokio-tungstenite (0.30.0).
39. https://crates.io/api/v1/crates/axum (0.8.9).
40. https://docs.rs/axum/latest/axum/extract/ws/index.html (0.8.9).
41. https://gafferongames.com/post/udp_vs_tcp/ "UDP vs. TCP", Glenn Fiedler, 2008-10-01.
42. https://gafferongames.com/post/packet_fragmentation_and_reassembly/ "Packet Fragmentation and Reassembly", Glenn Fiedler,
    2016-09-06.

Counts by tier (42 entries): official 35 (specifications, RFCs, MDN pages and data, vendor posts, registries, docs.rs, source
code); shipped-game practitioner 2 (Fiedler); other 5 (the GitHub README and repository pages of wtransport, web-transport and
h3, where they speak for their maintainers).

### Sources not read
- https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking: the fetch returned HTTP 403 Forbidden (the domain is on
  the allow list; the server refused). Not needed for this unit.
- The pages of the press search results about Safari 26.4 (9to5mac.com, digitaltoday.co.kr, iphonesoft.fr and others): outside the
  allow list. The official WebKit post was read instead.
- webtransport-ponyfill-websocket (named on the Chrome page, no URL followed): not read. No recommendation rests on it.

## Cross-unit notes
- **Snapshots, prediction, interpolation and lag compensation unit:** clock sync and RTT must come from our own ping messages,
  since `getStats()` is unusable on Chrome, Edge and Firefox (F9); quinn's server-side `rtt()` is available (F25). Deltas against the
  last acknowledged snapshot fit lossy datagrams. One-shot actions need redundancy or the reliable stream (GP2, REC2).
- **Tick rate and bandwidth budgets unit:** the per-datagram payload cap (proposed 1,000 bytes, REC2) and the congestion behaviour
  (F24: the window can collapse to 2 × max_datagram_size) bound the snapshot budget. Rate adaptation can read quinn's RTT and loss
  figures (REC6).
- **Interest management unit:** splitting a snapshot by entity group into independent datagrams (REC2) pairs with per-client
  relevance sets.
- **Rooms and private matches unit:** the join flow, slot takeover from bots and team allow-list checks run on the reliable
  control stream (STATUS.md:70). The protocol version can ride in `protocols` (F38). WT_DRAIN_SESSION can drain a session
  before a server shuts down (F37).
- **R1 (Rust ↔ TypeScript boundary, Web Worker):** both WebTransport and WebSocket work in workers (F34, F35). Where the transport
  lives (the simulation worker or its own worker) is R1's choice.
- **R5 (toolchain and quality):** an end-to-end test per browser for the transport paths, plus a recurring run against browser
  beta channels for draft changes (OQ1).
- **R6 (infrastructure):** UDP on the chosen EU hosts and load balancers (QUIC needs UDP; F39: connection IDs let sessions
  survive address changes, which matters for load balancer routing), publicly trusted certificates and their renewal, the
  HTTPS matchmaker that hands out certificate hashes (OQ4), and connection telemetry for the fallback share (OQ7).
- **security-auditor (R2 protocol review):** the server must check the Origin header on WebTransport (F36). The WebSocket Origin
  check was not sourced here (RFC 6455 not read). Per-session rate limits and H3_EXCESSIVE_LOAD (F36). Every reliable client
  message needs server-side validation (STATUS.md:70).
