# NPT-TRIM re-plan: the units (netcode-architect, read only, 2026-11-21)
Rule: Mustafa's NPT-TRIM (history.md:7561-7593), over plan rev 3.3 where they differ. Sizes are added lines (code and
tests); "likely" = the estimate x 1.35-1.5 (u6-brief.md:109). `row` = plan.md:1253 (U6's row).

## 0. Plan (a restart continues after the last block marked DONE)
1 U6 cut DONE · 2 one WT crate DONE · 3 certificate DONE · 4 U7 DONE · 5 U8 PENDING · 6 promote PENDING ·
7 path to R1 PENDING.
**Resume here (22:4x UTC, after the 20:20-22:40 cut):** blocks 1-4 done; next 5, 6, 7. Notes already gathered:
- 5: U8 row plan.md:1255; Stop B :1267-1280; laptop session §12.3 :1295-1343; Mustafa's checks §11 :1213-1241.
- 6: ADR 0002's crate list still to read (docs/adr/0002-code-structure-build-toolchain.md, around :180-200).
- 7: merges 11-10..11-14 about 38,000 lines in 5 days; 11-15..11-21 about 8,000 in 7 (git log of crates/); U4f2 built
  since 11-20 14:10, part B still running 11-21 evening; daily spend-limit cuts (card :163-171); one cargo lane on the
  shared target; disk about 8.6 GB free, floor 4 GiB (card :147-148). No web/ or package.json exists yet (ls).

## 1. U6, cut (DONE)
**Test applied:** the firewall admits only his home IPv4 and the LGs (block 3 covers issuance), so code whose only job
is stopping strangers goes; what keeps an honest run correct, measured or bounded stays.
**Cut** (each moves to the security baseline before the first online playtest, NPT-TRIM's R4 line, unless noted):
- E14's floods, U6d's B2: 1,000 never-joining sessions, oversize datagrams, stream openers (plan.md:473; u6-brief.md:35;
  marks -25, -34: R4 and C). No kept run sends them.
- Slow-loris deadlines on the game ports, TCP 31000 and 80; the page host's 5 s header timeout, 8 KB cap, per-address
  and total caps (plan.md:515, :1129-1130; S6, u6-brief.md:34). Only strangers hold sockets open.
- The /64 cycler; the bounded /32 and /64 address table with its 100,000-address test; the per-address cap and its LG
  raise control (plan.md:465, :515, :1073, :821). One LG address carries all its sessions: no cap, no raise.
- The handshake cap as a security cap (S6 with F7's sizing, u6-brief.md:47): replaced by one fixed per-process
  connection cap (roster N + 8), which still bounds DelayStream's ~0.31 MB per connection (row, F7).
- Origin checks on both transports, the `ws://` and wrong-Origin pages (plan.md:1132-1135, :473; T3).
- Tokens minted through the socket, capabilities, first-use binding, the resume secret, URL-fragment delivery with
  `replaceState`, the runner token (plan.md:1083-1114; S7). Kept instead: the room's built roster check (roster.rs:7-24)
  with per-slot tokens written at start to a 0600 file the run script hands to LG and the laptop; strays still counted,
  and a run admitting one stays INVALID (plan.md:1113-1114).
- Per-session controls (SetProfile, BlockUdp, TriggerBurst, the form) with their gate and bites (plan.md:1083-1095):
  R3 is unattended (plan.md:820), so L2 settings go per peer set through the control socket only.
- -25's "do never-joining sessions get a DelayStream" (row; R4); the host's 404 bite and `/iso/` COOP/COEP path
  (PM-0001-43/-44 are R7/ML: moved); robots and noindex (plan.md:1126-1131). The host keeps exact-path matching, which
  has no traversal by construction.
- systemd template units per slot user, ProtectSystem, the empty CapabilityBoundingSet, per-slot 0700 dirs and the
  room-unit sudo controls (plan.md:1078-1082, :1139-1144): npt-super starts rooms as its own children under one
  unprivileged user and SIGKILLs them itself (-27 needs the kill, not systemd). security-auditor confirms at Stop B.
- The second WebTransport crate and -35's swap form (block 2; -35 moves with U5); the WT certificate's live swap
  (S8b, for PM-0006-26: R4); HTTP-01's responder, `npt-cert`'s port window and `revoke` (block 3).
**Kept** (cheap, honest-run correctness or the VM itself): sshd's drop-in and key-only logins; nftables default drop;
the 2 s Join deadline covering TLS (a stuck own client must not hold a slot: plan.md:515); socket size limits and
panic-free decode (built in npt-proto and pump.rs's Decoder: u6-facts.md:78); the control socket's size-limited parse;
E22's no-verbatim-client-string logs and PM-0006-21's canaries (R5); `redact-check`.

**Carries** (row):
- C-U6-1 to -7 KEEP: R1's L2 probe block and ImpairSocket's R1 self-test (plan.md:818, :1255). C-U6-8 KEEP: the probe's
  stamps claim only what a test proves (150 lines).
- C-U6-9 KEEP (U6j): -30's twin form (R3, C). C-U6-10 KEEP (U6j): PM-0006-20 (R2). C-U6-11 KEEP for LG clients (-19,
  R3); its ML-browser `decoded_hash` writer MOVES with -19's R7/ML parts (the plain client has no hash).
- C-U6-12 CUT (one full-state stream, control-message limit: hostile only) -> baseline.
- C-U6-13 KEEP: R3's -19 rejoins past 32,768 events. C-U6-14 KEEP: -19's bots leave and rejoin (plan.md:1103-1105);
  the test form ("no closed session's message reaches the room") unless TickLoop needs a generation.
- C-U6-15 KEEP: before -4 (R2), the test's core question. C-U6-16: (a) KEEP, before -19's runs; (b) MOVED, no kept run
  runs a room at other than 30 Hz; (c) KEEP, the control bytes' channel (correctness); (d) CUT, hostile frame -> baseline.
- C-U6-17 KEEP: every VM run has honest clients off L1. C-U6-18 KEEP: npt-load hosts Behaviours. Both in U6a.
- C-U6-19 KEEP: the row's own gate reads "closed before -19's runs" besides the hostile and restarted clients, and
  -19 runs in R3; the brief-line gate (u6-brief.md:98-101) is moot with U4g, U4h and U5 stopped. Same JoinAccepted
  change and review as -13 and -16; the room-only form stays open to U6d's brief.
- **Lag compensation's server cost:** the ring (sim/src/history.rs:23-39) and `rewind_at`'s integer half
  (room/src/rewind.rs:13-33) are built and run in the room's tick (u4h-brief.md:69-70), so -4 and the density marks pay
  them already. If the mark table keeps -7, the smallest piece is its two series writers in U6e's S9 (~100 lines).

**Pieces, R1's needs first** (u6-brief.md:32-35's estimates, cut as above; H4 moves to U8):
| Piece | Blocks (estimate) | Sum | Likely | First needed by |
|---|---|---|---|---|
| U6a server core | S1 TickLoop, PhaseTimers (750); S8a binary, settings, panic catch, exit (175); C-U6-17, -18 (200); T6 `Link` loss, space, `Send` (150); S4 instrumentation, header, carry 60, C-U6-4 (400) | 1,675 | 2,250-2,500 | R1: -1, -2, -16, PM-0006-11 |
| U6b transports | T1 WtServer on one crate, one handshake-failure counter, the connection cap (450); T2 WtClient, WsClient (280); T3 WsServer accept, Join deadline, one `&L2Config` (300); T4 send path, fix 1 (700); T5 `bites`, SetProfile, B1 range check, header fields, POOL rule, reader stamp and its self-test (400); S8b WT certificate at start (75) | 2,205 | 2,950-3,300 | R1: smoke, -36, DelayStream self-test |
| U6c probe, load, host | P1 the -3 probe, C-U6-1 to -7 (750); P2 C-U6-8 (150); B1 npt-bots glue, npt-load, calibration (600); H1 page host, trimmed (300); H2 instant-acme DNS-01 issue (150) | 1,950 | 2,650-2,950 | R1: L2 probe, load, pages, certificate |
| U6d room for R2-R3 | S1b interest writers (350); S2 C-U6-13, -14, -16 (a)(c), -19 (650); C-U6-15's wins with the bench (400); rate adaptation (400) | 1,800 | 2,450-2,700 | R2: -4, -8 to -13; R3: -19, -23, -24 |
| U6e control, metrics, rooms | S7 ControlSocket, room-wide controls only (300); S5 metrics, logs, allocator twin (350); S9 series writers, panic and stall injections (400-650); H3 npt-super (300); B3 -30's runner (225) | 1,575-1,825 | 2,150-2,750 | R2: PM-0004-67, PM-0006-17/-19/-20; R3: -26, -59, -30; R5 |
| U6j judge | J1 C-U6-9 to -11 (400); J2 19 of the 20 "U6" figures, -33's moving (1,250-2,650) | 1,650-3,050 | | R2, R3, R5's verdicts; not R1 |
U6a-U6e: about 9,200-9,450 (was 11,200-12,450). R1 needs U6a-U6c: 5,830, likely 7,850-8,750. Each piece stays under
SIZE-CHECK's 6,000 at 1.5x.

## 2. One WebTransport crate: web-transport-quinn 0.13.x (DONE)
- **Why it:** it is built for a caller-made quinn endpoint (`Server::new(endpoint)`, public `Request::accept(conn)`:
  u3-scout-wt.md:29), which ImpairSocket needs; wtransport reaches that only through its `quinn` feature and keeps its
  endpoint private (:24-26, :46). Its `Session` derefs to `quinn::Connection`, so `datagram_send_buffer_space` (ADR
  0001:439-442's check) is direct; wtransport offers it only via `quic_connection()` (u3-verify-wt.md:40). wtransport
  skips validating the client's SETTINGS and hits `unreachable!()` when datagrams are disabled (u3-verify-wt.md:72,
  :84), and calls itself "not considered completely production-ready" (ADR 0001:927). U3 proved both routes equal on
  loopback (u3-scout-wt.md:56-57, u3-verify-wt.md:25), so nothing measured favours wtransport.
- **Its costs, handled:** frequent 0.x releases (ADR 0001:928: pin exactly, plan.md:635-653); its client builder binds
  `[::]:0` with `unwrap` (u3-scout-wt.md:47: npt-load uses `Client::new(endpoint, config)`); its default provider is
  aws-lc-rs, so rustls is built with `builder_with_provider(ring)` (u3-verify-wt.md:43, as l2/wt.rs does per crate:
  u6-facts.md:126-128). wtransport's self-signed helper is not needed: rcgen is in the lock (u6-facts.md:286-287).
- **What stays built:** the `wt-wtransport` feature and U3's tests as they are; U6 builds nothing new on it.
- **Must a kept run compare both?** No. -30 compares WebTransport with WebSocket (`params.transport`, row C-U6-9), not
  crates. PM-0001-28 runs on ML*, MAC-r and R4 (marks.json `where`), none of R1, R2, R3, R5; -35 moves with U5.
- **PM-0001-28 and R2-TRN-OQ1:** -28's text needs both crates in every browser (ADR 0001:1620), so one crate cannot pass
  it here: it MOVES to the real game's transport change-set, unchanged, and R2-TRN-OQ1 stays open until it runs there.
  Every header names the crate, and R1's smoke results go to that change-set as evidence, not a verdict. PM-0001-35
  (:1627) moves with U5.

## 3. The certificate behind the firewall: DNS-01 by hand (DONE)
**Sources.** HTTP-01 "can only be done on port 80", TLS-ALPN-01 runs "via TLS on port 443", DNS-01 is a TXT record at
`_acme-challenge.<domain>` and suits servers "not exposed to the public internet"; DNS API credentials on a server raise
"the impact if that web server is hacked" (https://letsencrypt.org/docs/challenge-types/). Let's Encrypt does not
"publish a list of IP addresses we use to validate, and these IP addresses may change at any time", and validates "from
multiple network perspectives" (https://letsencrypt.org/docs/faq/). `shortlived`: ~160 h, authorizations reused 7 h,
still a CRL URL (https://letsencrypt.org/docs/profiles/). Cloudflare TXT records "are always DNS-only"
(https://developers.cloudflare.com/dns/proxy-status/). instant-acme 0.8.5 gives the TXT value
(`KeyAuthorization::dns_value`, https://docs.rs/instant-acme/0.8.5/instant_acme/struct.KeyAuthorization.html).
`serverCertificateHashes`: X.509v3, ECDSA P-256, no RSA, "a validity period of less than 2 weeks", secure contexts only
(https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport); Chrome and Edge 100, Firefox 125, Safari
26.4 (https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransport.json; spec:
https://www.w3.org/TR/webtransport/#certificate-hashes). The WebSocket constructor has no hash option (URL and
protocols only: https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket). All high confidence.
**Weighed.** HTTP-01 or TLS-ALPN-01: unlisted, changing validators mean port 80 (or 443) open to every address per
order, so `fw-rs` keeps "TCP 80 from any" with nftables gating it (plan.md:913, :961), plus a root wrapper editing
nftables and a responder: automatic, but a hole to the world and more code. DNS-01 with an API token: a secret on RS or
the laptop (plan.md:986 rejects it; no session handles secrets). `serverCertificateHashes` alone: WebTransport only, and
it needs an HTTPS page, so the page and `wss` still need a browser-trusted certificate; it stays for the WT leg
(plan.md:991-996). DNS-01 by hand (plan.md:989's fallback, promoted): nothing inbound opens; the only credential is his
own Cloudflare login with two-factor (plan.md:869), which no session sees; the TXT value is public by nature.
**Recommendation: DNS-01 by hand; the HTTP-01 window is the fallback.** Inbound: nothing opens; `fw-rs` loses TCP 80
(plan.md:922). Outbound: `npt-cert issue` (root wrapper, no free arguments) opens npt-host's egress to the ACME host and
DNS for one order and closes it after (plan.md:961's window, minus the port). Per order:
1. Laptop session: the CT ledger check (plan.md:976-981); `sudo npt-cert issue staging|production` on RS, which opens a
   fresh ACME account (no secret kept: plan.md:975) and prints the TXT name `_acme-challenge.rs.nt` and its value.
2. Mustafa, in Cloudflare: add that TXT record, TTL 1 min (about 2 minutes).
3. The wrapper polls Cloudflare's authoritative servers directly until they show the value (at most 30 min, no retry
   loop: plan.md:949-951, :972-973), readies the challenge, requires `shortlived` in production (fails closed:
   plan.md:966-967), writes the key 0640 and the chain, closes egress, prints serial and hash for the ledger.
4. Mustafa deletes the TXT record; the laptop session confirms its absence over DNS-over-HTTPS (as plan.md:898-899).
R1: staging, then production (two edits inside its 0.25 h block, plan.md:818); R2, R3, R5: one each at bring-up, when
he is there to create the VMs (plan.md:879-886). No `revoke`: the key faces no stranger, lives 6 days, dies with the
VM, and the DNS record goes at teardown. The WT certificate: made by the room at start (U6-Q17, u6-brief.md:183), ECDSA
P-256, valid 13 days (MDN reads "less than 2 weeks"; plan.md:992's "at most 14" sits on the edge), its hash in
`/npt/cert-hash.json` over the trusted name (plan.md:994-995).

## 4. U7, a plain browser client (DONE)
**What R1 asks of it:** the smoke check, "WebTransport · Playing" then "WebSocket · Playing" in Brave, Chrome, Edge and
Firefox (plan.md:1214-1220); -36's `event.mds_read.size_bytes` from process `browser`, at least 61 reads over 10 min,
three browsers at once (marks.json PM-0001-36 checks; plan.md:739, :818); -14's ML browser parts (marks.json).
**Decoding: a Wasm build of npt-client's endpoint, not a TypeScript twin.** `ClientEndpoint` (client/src/lib.rs,
1,210 lines, with clock.rs 348) already holds the codec, the ack ring, ClockSync and the tick-lead controller that
stamps commands (`with_clock`, :354-365; `input`, :532-558); the bots run it, so browser and LG speak through one
reviewed implementation. It uses npt-sim's grid and record types only and steps no World (lib.rs:20-21), so "no sim in
Wasm" holds in meaning; even npt-proto alone depends on npt-sim for the wire grid (plan.md:426-428), so no Wasm codec
avoids that crate. A TypeScript twin would be a second bit layout and clock to prove equal (golden vectors), with no
cargo saved (the vectors need it).
**It needs:** npt-client-wasm's exports, create, input, step and one counters read (client-wasm/src/lib.rs is a stub;
wasm-bindgen is not in Cargo.toml: a new dependency, DEP-HOLD); a Worker holding Wasm and transport: WebTransport with
`serverCertificateHashes` from `/npt/cert-hash.json` and a 1.5 s timeout, then `wss` (plan.md:495-496), arrival stamps,
`maxDatagramSize` read at start and every 10 s; a timer from `next_due_ns`, at most 7 ticks a wake (plan.md:497); a fixed
press script (buttons and press counters, no aim from a cursor); acknowledgements come from the endpoint's ring; a page
of counters by `textContent` (transport, state, snapshots and bytes per second each way, refused, stale, RTT, size
reads); records as npt-log JSON lines kept in page memory for the WebDriver spec to collect (U8).
**Goes:** render.ts and three.js itself, block.ts, the overlay's form and record.ts, connect.ts (-28 moved), AimCast,
BlockWriter, `/iso/`, the Worker restart and rejoin after a trap (PM-0004-61's online half moved), the hash check.
**Carries:** C-U7-1 (wasm-opt hash check) and C-U7-2 (npt-hash-wasm in Chromium) GO with the hash check (NPT-TRIM:
change-set 3 checks determinism). C-U7-3 KEEPS: the endpoint takes the previous step's send time and ClockSync stamps
each `Ping` with it; the cap's source: my lean is the room's bound compiled into the same commit's Wasm (`with_clock`
takes it as a number, lib.rs:362; LEAD-BOUND is frozen), no wire change; else the ruling's `JoinAccepted` field in
U6d's S2 (plan.md:1254). C-U7-4 KEEPS: ClockSync's clamps and trimmed surface, now that it runs in a Worker.
**Flag for the mark table:** -52's R1 parts (ML hit feedback, marks.json) need drawing and hit feedback, which this
client does not have: move them with change-set 8 (Shoot).
**Size:** Rust (exports and their tests 200, C-U7-3 60, C-U7-4 100) 360; TypeScript (net 350, Worker host 150, page and
counters 120, records 120, press script and smoke 80, config with exact pins 60, Node tests with a fake transport 200)
1,080. About 1,450 estimate, likely 1,950-2,200. Its Wasm build needs the cargo lane; the TypeScript does not.
