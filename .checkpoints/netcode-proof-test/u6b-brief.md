# U6b's brief, rev 1.1 (2026-11-22): the transports (the advisor's APPROVE WITH CONDITIONS 1-8 applied: section 0.1)
**The unit:** the second piece of U6 as Mustafa's NPT-TRIM cut it (history.md:7561-7593; his answers of 2026-11-22,
history.md:7595-7607: RE-PLAN-OK yes, R1 on Thursday 3 December). Its row: replan-units.md:59 (T1, T2, T3, T4, T5,
S8b: 2,205 estimated, 2,950-3,300 likely), first needed by R1 for the smoke, -36 and DelayStream's self-test. Built on
the one cargo lane after U6a's merge (replan-units.md §7: about Wed 25-Thu 26 Nov; this brief's day line: section 8).
Short by TEST-FIRST: what is built, how it is tested, what is open. Reference, never retyped: u6-brief.md rev 2.1,
u6-facts.md, advisor-u6-brief.md, U6's row plan.md:1254 (replan-units.md:3 cites :1253), u6a-brief.md rev 1.1 (U6a's
interfaces and the build rules), advisor-u6a-brief.md, security-r1-deps.md.
**One advisor check** (NPT-TRIM), no re-check unless it finds a Critical or High. Section 9's six questions are the
only asks; every other choice goes by its lean, one line each (section 1).

## 0.1 Rev 1.1: the advisor's conditions (read first; each replaces or adds to the section it names)
**The advisor's check** (advisor-u6b-brief.md, 2026-11-22): APPROVE WITH CONDITIONS; no Critical or High, so no
re-check. Q-U6B-1 to -6 and the seven contradictions are ruled there (its sections 1-2): Q1 (a) `begin_tick`; Q2 (a)
ring, no `install_default`; Q3 (a) aligned to U8 (C-U8-5); Q4 as leaned with terms; Q5 and Q6 the lever taken now (the
per-peer profile, the B1 range check and `burst_return`'s fields to U6e). R1 on Thursday 3 December holds with no
slack once the lever is taken. Its conditions, word for word (advisor-u6b-brief.md:268-333):
1. **W18 (Q-U6B-1; ADV-U6BB-1, -2).** Section 1 adds to Block 4: "npt-link gains `fn begin_tick(&mut self, lens:
   &[usize], now_ns: u64) -> SendResult`, provided, default `SendResult::Sent`, its doc text stating the caller's duty;
   the blanket `impl<L: Link + ?Sized> Link for Box<L>` (link/src/lib.rs:97-119) forwards it. TickLoop's I/O side
   calls it once per session per tick before that tick's datagram sends, with each datagram's length (with a per-tick
   batch boundary if U6a's I/O side has none). WsLink answers every datagram of a refused tick BufferFull, counted in
   `tick.v.ws_skipped_snaps_count`; a datagram send no current `begin_tick` announced, or one past its count, is
   answered BufferFull and counted, never written ungated; control and FullState sends are not gated; WtLink, Loopback
   and L1 keep the default." Section 3 adds a test calling it through `Box<dyn Link + Send>` that sees WsLink's
   refusal, and plant P16 (the Box forward removed) failing it. Section 6's Files line reads "TickLoop's seam for the
   accept stamp and its I/O send site for `begin_tick`", both named in the report. Section 9's Q-U6B-1 reads "Ruled
   (a)".
2. **The provider and Block 0 (Q-U6B-2; ADV-U6BB-7).** Section 5's expected lock diff reads: "removed: aws-lc-rs
   1.19.0, aws-lc-sys 0.47.0, cmake, dunce, fs_extra, pkg-config; `"aws-lc-rs"` leaves quinn-proto's, rustls's and
   rustls-webpki's dependency arrays; npt-server's array gains `"rcgen"`; nothing added; no version, checksum or source
   line changed." Block 0 (d) adds: "no `CryptoProvider::install_default` anywhere; wt.rs's module text (:4-6) says
   ring for both crates." The binding check's item (4) reads "check.mjs's test step PASSES at `<base>` with the wt
   module's tests in the measured build on ring, its counts before and after Block 0 by name". Security-auditor's short
   read covers advisor-u6b-brief.md section 4's eight items; its answer and dependency-check.md exist before the save.
3. **The self-test's series (Q-U6B-3; ADV-U6BB-4).** Block 5's "At the run's end" bullet reads: "`ws_split` once
   through a now-pub `split_rows`, with pooled L per direction: one more `Pooled` per direction in `Pool`, recording each
   release's L (the sum the identity checks); per direction n, p50, p99, p99.9, max and E in both forms with their over
   counts, L0 = 0 (E[(L − 0.25 ms)+], the form the U8 check judges: advisor-u8-brief.md:87-96) and the median form
   (E[(L − p50 − 0.25 ms)+]), `summarize` taking its threshold as a parameter. The pool takes one mark at the measured
   window's start, at a settings time after OPEN (range-checked; absent: no mark, the whole life): a copy of every
   pooled bucket's count and sum, the kinds and the violations, taken once on the I/O runtime, never on the release
   path; each pooled maximum restarts there; `ws_split` writes the difference, so the parts and pooled L share one
   window; the header records the mark's time and whether it was taken. The per-second merge across connections is
   U6e's (S9), before R3." Section 3 adds three tests (both E forms against a brute-force sum; samples before the mark
   excluded, the window's own maximum, the identity exact across the mark) and the loopback example reports pooled L in
   both forms. Section 10 adds: "U8's next edit: R1.md sets the mark's time from its timeline (LG's 48 connections up,
   then the 1 min warm-up) and names `event.ws_split`'s pooled-L fields as the series the self-test reads." Section 9's
   Q-U6B-3 reads "Ruled (a), aligned to C-U8-5".
4. **The hash file (Q-U6B-4).** Block 6 adds: "the hash file is written by serde_json from a struct of exactly
   `algorithm`, `value_hex` (64 lowercase hex digits) and `validity_ns`, no `next` field (PM-0006-26 moved), by
   `create_new` with mode 0644 set at creation, one `write_all` then `sync_all`, after the certificate is made and
   before OPEN; `validity_ns` equals `header.params.cert_validity_ns`; an existing path is refused (exit 2), so U8's
   R1.md removes the previous room's file before any room restart. U6c's host serves it verbatim with `Cache-Control:
   no-store` (plan.md:994-995), read per request or after each room start, under a size cap; U7's page turns
   `value_hex` into `serverCertificateHashes`' bytes." Section 9's Q-U6B-4 reads "Ruled as leaned, with these terms".
5. **The lever and the size (Q-U6B-5, -6; ADV-U6BB-5).** Move `DelayStream::set_profile`, `WsServer::set_peer_profile`,
   `WsServer::peer_burst` (the B1 range check), `burst_return`'s four server fields and their tests and plants P12 and
   P13 from Blocks 4 and 5 to section 2's U6e line, with SP-6, RC-2 and SERIES.md:326-329 named there by number
   (U6e's brief carries them; they stay U6's). Section 8 reads: "About 2,650 (the blocks recounted, the lever out,
   Q-U6B-3's window mark in), likely 3,600-4,000; the stop past 6,000. The lane before R1: about 9,900-11,300 likely
   lines; R1 on Thursday 3 December holds with no slack; one lost day and it slips (three lines to him at once: the
   advisor's text, advisor-u6b-brief.md section 3)." Counts: at least +45 measured and +47 bites; plants P1-P11, P14,
   P15, P16. Section 9's Q-U6B-5 and -6 read "Ruled: moved to U6e; the lever taken now".
6. **The validity dates (the build trap).** Block 6's certificate text reads: "`not_before` = `rcgen::date_time_ymd` of
   today's civil date (00:00 UTC) from one reviewed `SystemTime::now` read in cert.rs (the one allow, its reason
   calendar validity), by a days-to-civil routine tested on fixed dates (1970-01-01, 2000-02-29, 2026-11-22,
   2100-03-01, a year end); `not_after` = `not_before + std::time::Duration::from_secs(13 × 86,400)`, or
   `date_time_ymd` of the civil date 13 days on; a clock before 1970 or past 9999 is exit 2, never a panic;
   `cert_validity_ns` = 13 × 86,400 × 10^9 from the same values, and the test reads the DER's validity back; the read
   feeds no timing series. If neither form compiles without naming `time`, the build stops for a manifest change."
7. **WsClient's limits (ADV-U6BB-3).** Block 3's WsClient text reads "tungstenite's client handshake with the
   client's own limits: frame and message at least the largest room message plus its tag (a FullState of 65,536 + 5 B
   plus 1 B: the catalogue :73-76), from one constant beside L2Config's, never the server's 513 B", with a test that
   a FullState of the largest size reaches the WsClient.
8. **Small fixes (ADV-U6BB-6, -8, -9, -10).** The new pub items add `ConnCap` (or the form both binds share), read by
   the netcode seat. Section 3's container reading reads "before Stop B's read of R1's run plan (U8 builds beside U6:
   replan-units.md:197-198)", and RC-12's order is met that way. The two 20-iteration kick tests fit the 5 s real-clock
   cap (concurrent, or fewer iterations with the reason). Section 2's U6e line adds SERIES.md:616's second clause (what
   the plain path writes) beside the per-second merge; U6b writes `ws_wrapper` 0 and fix 1's inputs on that path.
   Section 7's security reads add the token file's reader, U8's `npt-ctl rooms tokens NN` run as `npt-room`
   (advisor-u8-brief.md:97-98), at U6b's round if U8 has built it, else at Stop B's read. The report notes once that
   replan-units.md:77-78's "as l2/wt.rs does" is wrong (wt.rs:176-179).

## 0. Where things are
- **`<base>`** (filled at launch): U6a's merged head plus Block 0's commits (section 5); its spikes/netcode-proof tree,
  check.mjs counts (measured, bites) and `cargo check --workspace --all-targets --locked` there, 0 warnings.
- **U6a's interfaces U6b builds on, re-read at `<base>`** (each with file:line in the builder's notes before Block 1):
  TickLoop's accept seam (how a session's `Box<dyn Link + Send>` and its peer address reach the I/O runtime, and
  whether an accept stamp travels with them); how the I/O side hands a session its tick's outbound bytes; the `OPEN`
  line's site in server/src/main.rs; the token file (path, format, one token per slot); the settings struct and parser
  (SEC-U6AD-2); the clock item (`server/src/clock.rs`'s `now_ns`, u6a-brief.md:129-130); the writer thread and the
  header's params (u6a-brief.md:140-149).
- **Today** (read at f565555; spikes/ unchanged since aa5c62b outside results/): npt-net has no WtServer, WtClient,
  WsServer or WsClient (net/src/lib.rs:1-28). l2/wt.rs builds each crate's server on a quinn endpoint over ImpairSocket
  with a three-reason `Handshakes`, `PENDING_SESSIONS` 64 and an accept loop with no cap on concurrent handshakes and no
  deadline on the HTTP/3 stage (wt.rs:16-21, "Carried to U6"); its web-transport-quinn provider is aws-lc-rs
  (wt.rs:177-179), not ring. ws_gate.rs holds fix 1 (`snapshot_budget` :25, `may_write` :36, `TickWriter` :74, `Bite`
  cfg(test) only :58-69); ws_config.rs the TLS acceptor and tungstenite's config (:27-58); tcp_kernel.rs the plain
  path's srtt and SIOCOUTQ (:22, :28). DelayStream: no set-profile method; `gate_inputs` :1237; `add_burst` :1136;
  `header_numbers(&self)` blocking (F6) :1349; `split_rows` pub(crate) :1119; POOL never reset :185-195; per-second
  rows as summaries, not histograms (:1419-1460); the closing `wake_all` :1907 (the row's :1884; u6-brief.md:188).
  tls_test.rs's pinning verifier `Pinned` (:92-97) is cfg(test). npt-net's features: `wt-wtransport`, `wt-wtq`; no
  `default`, no `bites` (net/Cargo.toml:12-15); check.mjs's measured build takes default features, its bites build
  `--all-features` (check.mjs:2017, :2057). `Link` sends per message, with no tick boundary and no close
  (link/src/lib.rs:64-88). The Join deadline: 2 s from the `accept_ns` given to `Decoder::new` (pump.rs:13-14,
  :253-260).
- **What R1 asks of U6b** (replan-marks.md:31, :49, :96; plan.md:1214-1220, :1256): the five-browser smoke (WebTransport
  then WebSocket per browser, U7's page); -36 (three browsers' WebTransport sessions for 10 minutes); PM-0006-25 (the
  pinned dev certificate in every browser; its setup check `header.params.cert_validity_ns` <= 14 days); -29 (the
  fallback when QUIC is blocked); DelayStream's self-test on RS (48 connections at P2 with the room running, at least
  10,000 samples per direction, the split plus U6's reader stamp: plan.md:1256, C-U8-3, C-U8-5). Bite built here:
  PM-0001-18/ws-budget-x2 (judged at R3's L2 B1 ws part).

## 1. What U6b builds (blocks in order; each compiles with its tests passing before the next)
File names are leans; the builder may merge modules and reports the names.
- **Block 0, U6b's manifests and the provider lines** (the main session's, on the lane before `<base>`): section 5.
- **Block 1, the shared pieces** [T1, T2, T3] (crates/net; ~150): `ConnCap`, one per process, N + 8 permits (N the
  settings' client count; replan-units.md:17-18), shared by both listeners: a permit taken at the first QUIC packet
  (`Endpoint::accept`'s `Incoming`) or the TCP accept, before any TLS, and returned when the session's closing task
  ends; none free: QUIC `Incoming::refuse()`, TCP dropped at once, counted per transport. F7: N + 8 bounds DelayStream
  at about 0.31 MB a connection; the header records the cap. `pin`: tls_test.rs's `Pinned` promoted to shipped code
  (pub(crate)): the end-entity certificate's SHA-256 equal to a given 32 B hash, the handshake's signature still
  checked through the provider. **Stamps:** both servers take the caller's clock as `fn() -> u64` (npt-server passes
  U6a's `now_ns`, tests an injected clock) and the 2 s deadline as a value (tests pass a shorter one); each
  `Recv::arrival_ns` is read by the session's reader task when the message arrives, never at `poll_recv`; each session
  carries its accept stamp (first QUIC packet or TCP accept) into TickLoop's seam as `Decoder::new`'s `accept_ns`, so
  the Join deadline runs from it (plan.md:516: "never from the end of a delay"). If U6a's seam carries no stamp, U6b
  adds it there (its one TickLoop change, named in the report). DelayStream keeps its own clock for its own stamps;
  the two are never mixed.
- **Block 2, T1 WtServer and T2 WtClient** (crates/net, l2/wt.rs; ~500 + ~200): `WtServer::bind` (std types only, as
  L2Config's rule, plan.md:603-606: address, `&L2Config`, certificate and key DER, `cc`, clock, cap): the quinn endpoint
  over `ImpairSocket::bind` (socket.rs:176; wt.rs's `endpoint`), wtq's accept loop under the cap, the whole handshake
  (QUIC, HTTP/3 SETTINGS and CONNECT, the answer) and the hand-off under `timeout_at(first packet + 2 s)`: this closes
  wt.rs:16-21's carry; ring (Block 0); quinn's congestion controller from `cc` (NewReno or Cubic, E6); migration off as
  built (M9); one handshake-failure count (`Handshakes` summed; its three-reason split stays as built, unwritten: the
  per-reason refusals are cut, replan-units.md:17-18). `WtLink` (`Link + Send`): datagrams by quinn's `send_datagram`
  after `datagram_send_buffer_space()` (ADR 0001:439-442; BufferFull when short); the client's first bidirectional
  stream as Control, 2 B length first, a length above 512 B closing the session as malformed, counted (catalogue
  docs/design/netcode-proof-messages.md:68-71; replan-units.md:34-35 keeps socket size limits); each FullState on its
  own server unidirectional stream with its 4 B length (:72); `rtt_ns` and `max_datagram_size` from quinn's
  connection; `unsent_unreliable` from quinn's datagram send buffer, its reading stated; bounded receive queues (a full
  datagram queue drops and counts; a full stream queue stops reading, so QUIC flow control holds the peer). **Close:**
  dropping a `WtLink` hands the connection to a closing task: queued control messages written (`Kick(replaced)` among
  them), the control stream finished and its acknowledgement awaited (lean bound 2 s), then `close`; the permit
  returns when it ends (advisor-u6a-brief.md:202-205: "an immediate QUIC close can drop unsent stream data").
  `WtClient::connect`: web-transport-quinn's `Client::new(endpoint, config)` on our own quinn endpoint (its builder
  binds `[::]:0` with `unwrap`: replan-units.md:76), ring, ALPN h3, the pin; `Connecting` (sends BufferFull) until the
  session opens; the client half of the framing, FullState capped at 65,541 B.
- **Block 3, T3 WsServer's accept path and T2 WsClient** (crates/net; ~300 + ~130): `WsServer::bind` (address,
  `&L2Config`, the certificate chain and key, clock, cap): TCP accept, permit, accept stamp, then under
  `timeout_at(accept + 2 s)` (S5 b): TLS (ws_config.rs's `acceptor`), `DelayStream::wrap` unless `cfg.pass_through`
  (R20: clean runs go plain), tokio-tungstenite's accept with `ws_config(cfg.ws_max_message_bytes)`: both limits from
  one `&L2Config` (S5 a); the connection index in accept order (delay.rs's CONN_INDEX_RULE); one handshake-failure
  count (TLS or upgrade). The wrapper is on or off for the process (`header.params.ws_wrapper`), so per-peer profiles
  need it on. `WsLink`: one binary message per channel message behind its 1 B channel tag (catalogue :73-76), an
  unknown tag closing the session as malformed, counted; its reader on its own task over split halves
  (`tokio::io::split`), as R1 measures it (UP-STAMP item 3). **Close:** queued reliable messages (`Kick(replaced)`
  first), a Close frame, then shutdown inside DelayStream's 10 s wait; a shutdown error after the peer's close is a
  normal end (C1c). `WsClient::connect`: TCP, tokio-rustls client (ring, ALPN http/1.1, SNI, the pin), tungstenite's
  client handshake with the same limits, masked frames, the tag. No Origin check (cut: replan-units.md:19); nothing
  calls `Url::origin()` on client input (SEC-R1D-1); no request line, header or URL logged (E22).
- **Block 4, T4 the send path with fix 1** (crates/net; ~760): one `TickWriter` per connection; once per tick (W18 by
  Q-U6B-1) one reading of its inputs: `gate_inputs()` on DelayStream (F11: one lock, one model step), or on the plain
  path `tcp_kernel::srtt_ns` and `outstanding_bytes`, owed = tungstenite's buffered bytes plus rustls's unsent TLS, and
  the free value stated with its reason (S5 d); `snapshot_budget` and `may_write` called, never copied; the tick's whole
  snapshot written all or none in the model's bytes (ws_gate.rs:14-17); a skipped tick answers each of its datagrams
  BufferFull (plan.md §4.4: "npt-room sees `SendResult::BufferFull` and the skipped count") and adds to
  `tick.v.ws_skipped_snaps_count` (SERIES.md:330); control and FullState messages are not gated (tungstenite's 128 KB
  bound, ws_config.rs:19). **The per-tick syscalls** run on the session's writer task on the I/O runtime, once per tick,
  never on the tick thread (48 connections x 2 reads x 30 Hz: about 2,900 a second there). `burst_return`'s server
  fields (`offset_ns`, `tick_phase_ns`, `budget_bytes`, `snap_bytes`: SERIES.md:326-329, :612-614) from the gate at each
  of DelayStream's per-return records.
- **Block 5, T5 the L2 extras, the header and the reader stamp** (crates/net l2/delay.rs, delay_names.rs, ws_gate.rs;
  crates/server; ~560):
  - **bites:** `Bite::BudgetX2` compiled under `cfg(any(test, feature = "bites"))`, armed per WsServer instance by id
    (`with_bite("PM-0001-18/ws-budget-x2")`, `None` for another id, as `Decoder::with_bite`, pump.rs:282); the
    queueing bite stays test-only.
  - **SetProfile:** `DelayStream::set_profile(&Profile)` (a new pub item, plan.md:607-608's list gains it): delay,
    jitter, loss and reorder from the next transmission, bursts kept; `WsServer::set_peer_profile(ip, Option<Profile>)`
    for live and new connections from that address. Its caller, `SetPeerProfile` (proto/src/control.rs:363), is U6e's.
  - **B1 range check:** `WsServer::peer_burst(ip, start_ns, len_ns)` refuses, before `add_burst`, a start at or before
    now or past the session's life, a zero length, a length above 1,000 ms (`PeerBurst`'s bound,
    proto/src/control.rs:379-380), counted; U6e's ControlSocket calls it (S7, F1).
  - **The header** (U6a's writer): `ws_wrapper` and DelayStream's numbers once at start from the one `&L2Config` (a new
    reading beside `header_numbers(&self)`, called once before OPEN: F6's "once at start-up"); ImpairSocket's `l2_*`
    from the bound socket (socket.rs:312); `cert_validity_ns` (SERIES.md:241); `transport`, `cc`, the crate
    (replan-units.md:84: "Every header names the crate"), the cap, the POOL rule (one measured run per server process:
    delay.rs:185-188); new SERIES.md rows where none exists. Carry 60's weapon and health are U6a's S4, checked present.
  - **At the run's end:** `ws_split` once through a now-pub `split_rows` (delay.rs:1114-1118, "U6 carry"), with pooled
    L per direction (Q-U6B-3); each connection's `ws_summary` at its end (`run_summary`, :1466); one event per transport
    (accepted, handshake failures, refused at the cap, closed at the deadline before a session).
  - **The reader stamp:** R at the closing `wake_all` (delay.rs:1907); up's woken work splits into pass = R − P and
    reader = (3) − R, the other class's pass from T; a reader that returns before R in its own counted class with no
    reader part; each release's identity checked and counted (`ws_late_<dir>_split_violations_count`); new SERIES.md
    rows for the new parts. This brief reads "the four parts" as up's woken class's clock, hop, pass and reader.
- **Block 6, S8b and the binary's transport wiring** (crates/server main.rs, new cert.rs; ~250): settings gain (on
  SEC-U6AD-2's rules) the WebTransport and WebSocket bind addresses (UDP 30001 + s, TCP 31001 + s: plan.md:909-910),
  `transport` (`wt`, `ws` or `both`: what the run's clients use, recorded as `params.transport`; both listeners always
  bind), `cc`, each server's L2 settings at start, the WebSocket certificate (the public chain and key files from
  U6c's `npt-cert`, each read under a size cap, or the room's own certificate for runs with no browser:
  replan.md:69-71) and the hash file's path. **The dev certificate at start** (U6-Q17; replan-units.md:119-121): rcgen
  ECDSA P-256, X.509 v3, valid 13 days; the key in memory only, never written; its SHA-256 written by `create_new` (an
  existing path refused, exit 2) in Q-U6B-4's format; `cert_validity_ns` in the header. rcgen's validity fields are
  `time::OffsetDateTime` and `time` is no direct dependency: lean, `rcgen::date_time_ymd` from a civil date computed
  from one reviewed `SystemTime::now` read (not_before today 00:00 UTC, not_after 13 days later); if that cannot hold,
  the build stops (a manifest change). **OPEN moves** (advisor-u6a-brief.md condition 3): after the certificate is made
  and both listeners accept, so spawn to Open covers a real start (ADR 0001:484, :1604).
- **Choices by lean (no ask):** rcgen in npt-server's private cert module, WtServer taking DER as std bytes (npt-net's
  surface stays std types); npt-net's own `default = ["wt-wtq"]` names its one crate and no dependent names a `wt-*`
  feature (U6-Q13 term (1)'s form), and with both features on WtServer is web-transport-quinn's, stated and tested
  (term (2)); one measured run per server process for the split's pool, no reset; one handshake-failure count per
  transport; WebTransport's closing bound 2 s; the WebSocket certificate chosen by settings; "the four parts" as above.
- **New pub items** (plan §4.7's whole-surface rule; the netcode seat reads them): npt-net `WtServer`, `WtClient`,
  `WsServer`, `WsClient` (listed in §4.7), `DelayStream::set_profile`, DelayStream's header reading from `&L2Config`,
  `DelayStream::split_rows` (now pub); npt-link `Link::begin_tick` if Q-U6B-1 (a). Each named in the report with its
  reason.

## 2. What U6b does not build
- **U6a (merged first):** TickLoop, the I/O runtime and the routing of a replaced session (C-U6-14's first clause), the
  first form of `OPEN`, the token file, the settings parser, the clock item, the writer thread and the header, carry 60,
  `Send` on links, C-U6-17 and -18.
- **U6c:** the -3 probe (C-U6-1 to -8), npt-bots' npt-net glue, npt-load and calibration (they host WtClient and
  WsClient), the page host serving `/npt/cert-hash.json` (plan.md:994-995), DNS-01 issue (the public chain WsServer
  loads). **U6d:** rate adaptation with T6's `loss` and send-buffer space (Q-U6A-4), C-U6-13, the rest of -14, -16 (a),
  -19 and the `JoinAccepted` change, the interest writers, C-U6-15. **U6e:** ControlSocket (the callers of U6b's
  set-profile, burst and UDP-block hooks), metrics and logs, the per-second merge of DelayStream's rows across
  connections (SERIES.md:589-591) and ImpairSocket's per-second rows in the room process (S9), the injections, the
  density harness, -30's runner over WtClient and WsClient. **U6j:** the judge. **U7:** the browser client and its
  headless Chromium check against a loopback server on both transports (replan-units.md:201), the first browser contact
  with WtServer and the dev certificate. **U8:** the run package, R1's DelayStream self-test plan (C-U8-3, -5), the
  public key file's group so the room's user can read it.
- **Cut or moved, not built:** C-U6-12 (replan-units.md:42); Origin checks (:19); per-reason refusals and the handshake
  cap as a security cap (:17-18); -25's never-joining sessions and their DelayStream (:11-12, :26); new code on the
  second crate, -35's swap form and counted run (:31, :84; register 110-111 moved with U5); the certificate's live swap
  (:31; PM-0006-26) and PM-0006-27's 15-day certificate (replan-marks.md:97-98).

## 3. How it is tested
- **Unit and loopback tests** (127.0.0.1 only; injected clock wherever a time is asserted; real-clock tests at most 5 s
  each, no timing bar in the container): Block 1: the cap across both transports (N + 9th refused on either, a permit
  back after a close), the pin (the hash accepted, another certificate refused, on both clients), arrival stamps taken
  at arrival (3). Block 2: all three channels both ways with their framing; a control length above 512 B closes as
  malformed; BufferFull when quinn has no datagram space; one failure counted for a wrong ALPN; the deadline covers QUIC
  and HTTP/3 (a client that never sends CONNECT is closed and counted, the elapsed time at least the deadline passed
  in, no upper bar); `Kick(replaced)` read before the close, 20 times on ImpairSocket at P2 with 5 % loss; both
  features on gives web-transport-quinn on ring; `cc` reaches quinn; the client `Connecting` until open (9). Block 3:
  `one_source` (ws_config.rs:63) on WsServer's own configs with DelayStream's limits equal; the deadline covers TLS and
  the upgrade (a silent TCP client, a stalled upgrade); the plain path when pass-through (`ws_wrapper` 0);
  accept-order indexes; tag routing and the unknown tag; `Kick(replaced)` before the close, 20 times on DelayStream at
  P2 with loss; the client masks and tags (7). Block 4: U3c's writer properties (b), test (c) and the queueing-writer
  bite (d) (u3c-plan.md:171-191) re-run through WsServer's writer on DelayStream at P2 under B1; inputs read once per
  tick; all-or-none per tick; a skipped tick answered BufferFull and counted; the plain-path check (a client with a
  small receive buffer stops reading; after any write SIOCOUTQ <= N x size, and writes stop); owed counts rustls's
  unsent bytes; a shutdown error after the peer's close is a normal end; `burst_return`'s server fields at each return
  (10). Block 5: set_profile from the next transmission; the per-peer profile on live and new connections; the burst
  range check; the header's fields; the identity with the reader stamp (woken: clock + hop + pass + reader = L; other:
  wait + pass + reader = L); the early-reader class; the misattribution bite (a test-only variant stamping R at the
  release fails the split test, as b5c9009 did); `ws_split` with pooled L once at the run's end; `ws_summary` per
  connection; the loopback self-test (8 WsServer connections at P2 for 3 s, its own reader task on split halves: the
  four parts present and the identity exact, no timing bar) (10). Block 6: the dev certificate ECDSA P-256, v3, 13
  days (a test-local DER read, no new dev-dependency); the hash file equal to the served certificate (WtClient pins it
  and joins; another hash refused); no key bytes on disk; an existing hash path refused (exit 2); `OPEN` only after
  both ports accept (at `OPEN` both accept at once, no retry; the time reported within the 5 s real-clock cap);
  npt-client endpoints over WtClient and WsClient at P2 join with U6a's token file and get snapshots, with no kind-10
  refusal (C-U6-17's form on L2); transport settings refused out of range (7). **At least +46 measured**, beside the
  wt module's existing tests that Block 0's `default = ["wt-wtq"]` brings into the measured build (counted in
  `<base>`, listed by name in Block 0's evidence).
- **Bites** (bites build): PM-0001-18/ws-budget-x2 armed by id on one WsServer instance doubles N while another stays
  at the formula, and the armed instance fails test (c)'s bound on loopback; its marks.json check (the WebSocket median
  on L2 B1 ws) is judged at R3. **+2 bites tests.**
- **Plants** (scratch edits, one `-x -k` runner call each, restored and sha256-checked; each must fail the named test):
  P1 the permit not returned when the closing task ends (the cap test); P2 the arrival stamp read at `poll_recv` (the
  stamp test); P3 the WebTransport deadline over QUIC only (the HTTP/3 deadline test); P4 the WebSocket deadline started
  after TLS (the TLS deadline test); P5 an immediate QUIC close on drop (the WebTransport kick test); P6 the Close frame
  before the queued messages (the WebSocket kick test); P7 tungstenite's limit from a constant (`one_source`); P8
  `gate_inputs()` per message (the once-per-tick test); P9 the gate per message (the all-or-none test); P10 owed without
  rustls's unsent bytes (the owed test); P11 a shutdown error after the peer's close counted as a failure (the C1c
  test); P12 set_profile applied to new connections only (the per-peer test); P13 the burst range check skipped (the
  range test); P14 `OPEN` before the listeners bind (the OPEN test); P15 the key written beside the hash (the key test).
- **check.mjs's 11 steps** (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs, marks, self, redact) PASS on
  both builds; counts as deltas on `<base>`'s: at least +46 measured and +48 bites. The test step's time against
  `<base>`'s; past 1.5 times it stops (u4f-brief.md section 1 item 5).
- **Container readings, reported, never judged:** 20 starts' spawn to `OPEN` with both listeners and the certificate
  (p50, max; U6a's were a lower bound); the WsServer loopback self-test as an example run (crates/server/examples/, no
  manifest change; npt-client is npt-server's dev-dependency): 48 WsClient connections at P2 for 60 s, release build,
  the four parts' p50, p99, p99.9, max and E per direction, the early-reader count, the violations, pooled L, steal
  read before and after (/proc/stat), before U8 builds the run package (plan.md:1254, UP-STAMP item 3); the cap's
  memory bound at N = 48 ((N + 8) x about 0.31 MB plus the fixed part).
- **The report gives:** counts and provenance; each plant's failing test and restore hash; the bite's failing check;
  the SERIES.md rows written (existing: :241, :326-330, the `l2_*` and `ws_*` header rows, `ws_split`, `ws_summary`;
  new: each listed) and every npt-server row left, named for U6c-U6e (advisor-u6-brief.md condition 1); the new pub
  items; added lines per block against section 8; what is not verified (no browser: U7's headless Chromium is the first
  contact with the dev certificate and both transports; no RS, no netem, no load machine; Firefox's hashes untested).

## 4. The carries, word for word, and the advisor's conditions
**From U6's row (plan.md:1254; the labels SP and RC are this brief's), WsServer's send path with fix 1:**
- SP-1: "the gate: `snapshot_budget` and `may_write`, shared, not copied; its inputs come from DelayStream on L2 and
  from npt-tcpinfo on the plain path; owed = everything above the kernel socket (tungstenite's buffer plus rustls's
  unsent TLS); the plain path's free value stated with its reason;" Block 4.
- SP-2: "U3c's writer properties, test (c) and the queueing-writer bite re-run on WsServer;" Block 4.
- SP-3: "a plain-path check: a client with a small receive buffer stops reading; after any write SIOCOUTQ ≤ N × size,
  and writes stop;" Block 4.
- SP-4: "the run-time bite ws-budget-x2 in a npt-net `bites` feature (R11);" Blocks 0 and 5.
- SP-5: "how WsServer sees one tick's whole snapshot through Link's per-message send (W18);" Q-U6B-1, Block 4.
- SP-6: "SetProfile on a DelayStream connection;" Block 5 (Q-U6B-5).
- SP-7: "where the per-tick syscalls run;" Block 4.
- SP-8: "the header records whether the wrapper is on, and the scene's weapon and health (`Scene::weapon`,
  `Scene::health`: the advisor's W1-U4D TUNE-1; carry 60)." Block 5; weapon and health in U6a's S4.
**More carries from U3c's review rounds (2026-11-08):**
- RC-1: "WsServer reads `gate_inputs()` once per tick: one lock, one model step (F11);" Block 4.
- RC-2: "npt-server range-checks a client-triggered B1 burst's start and length before `DelayStream::add_burst` (S7,
  F1);" Block 5, in WsServer for U6e's caller (Q-U6B-5).
- RC-3: "`header_numbers` blocks on the clock thread, so call it through spawn_blocking or once at start-up (F6);"
  Block 5 (once at start-up).
- RC-4: "size the accept cap at about 0.31 MB per DelayStream connection, plus a fixed amount per process (F7; the
  advisor's split ruling (b));" Block 1 (N + 8).
- RC-5: "record whether -25's never-joining sessions get a DelayStream at all (at P0 the wrapper is refused);" CUT
  (replan-units.md:26).
- RC-6: "build tungstenite's limits and DelayStream's from one `&L2Config` (S5 a);" Block 3.
- RC-7: "the Join deadline covers the TLS accept too (S5 b);" Blocks 1-3 (and HTTP/3 on WebTransport).
- RC-8: "the plain path's free value is stated with its reason (S5 d);" Block 4.
- RC-9: "WsServer uses tokio-tungstenite with futures-util as a normal dependency, listed in §4.8 and audited at U5;"
  Block 0 (U5 stopped: security-r1-deps.md:48-54 and :223 are the read).
- RC-10: "the lateness split's histograms are pooled per process and never reset (delay.rs's POOL; C1b's review,
  INFO 5): one measured run per server process, or the pool reset at each run's start, stated in the run's header;"
  Block 5 (one run per process).
- RC-11: "DelayStream's shutdown now returns S's own error when the pump ended on one (C1c): a browser that drops TCP
  without TLS close_notify gives rustls's UnexpectedEof there, so WsServer treats a shutdown error after the peer's
  close as a normal end, not a run failure (C1c's review, INFO);" Blocks 3 and 4.
- RC-12: "**the reader stamp (the advisor's UP-STAMP ruling, 2026-11-08):** stamp R at the reader's wake
  (DelayStream's closing `wake_all`, delay.rs:1884, not at the release, which is already P or T), so up's woken work
  splits into pass = R − P and reader = (3) − R (in the other class the pass is from T); a reader that returns before
  R gets its own counted class with no reader part, so the sum stays exact; an identity test and a misattribution
  bite, as b5c9009 did; U6's loopback self-test of WsServer (its own reader task, split halves) reports the four parts
  before the run package is built; the bar stays on L as defined." Block 5 at delay.rs:1907 (u6-brief.md:188).
**C-U6-12 to -19 (replan-units.md:42-50):** C-U6-12 CUT; C-U6-13, -15, -16 (a), -19 U6d; -16 (b) MOVED, (c) U6a, (d)
CUT; C-U6-14's first clause U6a, U6b's part below; C-U6-17 and -18 U6a. **C-U6-17's form** (plan.md:1254), in force
before U6b's honest L2 clients run: "before an honest client runs on a link other than L1, the controller's cap holds
under both options. The form: the stamp capped at the earliest arrival's next tick + 7 (`next_tick_at(now + lo_rtt / 2)
+ 7`). Its test: one 600 ms pong and a 10 ms path, under option B's buffer of 8, with no kind-10 refusal on the honest
client." Block 6's end-to-end test asserts no kind-10 refusal over both transports at P2.
**U6a's carries (advisor-u6a-brief.md condition 6, :261-263):** "OPEN's move (condition 3); `Kick(replaced)` reaching
the old client before the close on both transports; its security seat reads U6a's token minting and file with the
`Join` path." Blocks 6, 2 and 3, section 7. Condition 3's text (:238-240): "U6a's OPEN means the room built and
TickLoop running; U6b's brief moves it after both listeners accept and the WebTransport certificate is made, so spawn
to Open covers a real start (ADR 0001:484, :1604); U6a's container readings are a lower bound, never -16's evidence."
**C-U6-19's gate line** (advisor-u6a-brief.md condition 2, :233-237): "Until U6d merges, U6a's, U6b's, U6c's, U7's and
U8's briefs and R1's run plan each state in one line that their tests or runs hold no throwing or rejoining hostile
client and no client restarted mid-run, or bring C-U6-19 first. In R1 each browser leg (each browser on each
transport, the QUIC-blocked fallback, -36's three) joins its own slot once, or the room restarts between legs; the
token file holds a slot for every leg. SA-F1-1 stays open until U6d merges." U6b's line: **U6b's tests hold no
throwing or rejoining hostile client and no client restarted mid-run; the two `Kick(replaced)` tests check delivery and
routing only, no press or shot count.**
**advisor-u6-brief.md, on U6b's blocks:** condition 1 (the rows U6b writes, section 3); condition 6 and ADV-U6B-7
(:248-250: "U6b (tokio-tungstenite and futures-util as normal dependencies, npt-net's `bites` and `default` features,
rcgen for the certificate with aws-lc-rs kept off)": section 5); condition 7 (every reviewer at xhigh; the
engine-architect seat replaced by NPT-TRIM's default, as Q-U6A-7 ruled, the netcode seat reading the pub items);
condition 8: U6-Q13's terms (1) and (2) as choices in section 1, (3) and (4) moved with -35; U6-Q17's terms (:208-213)
under the cut: (1) "The key never leaves the slot's 0700 directory; only the hash goes to a path npt-host's user can
read" becomes the key in memory only (the slot directories are cut: replan-units.md:28-30), (2) the swap CUT, (3)
`/npt/cert-hash.json` U6c's, (4) PM-0006-27 moved; ADV-U6B-9's seams: F7's sizing and the handshake cap are the one
connection cap, C-U6-12's both-transport limit cut, carry 60 in U6a's S4. u6-brief.md:188 (Q14): U6b cites
delay.rs:1907. **security-r1-deps.md's pins (:223-224):** "futures-util =0.3.34 (U6, npt-net normal dep, workspace
entry unchanged): CLEAR." and "rcgen =0.14.10 direct (U6): CLEAR WITH CONDITIONS: default-features = false, features =
["ring"]."

## 5. Dependencies: Block 0 (named up front; DEP-HOLD)
- **No new package.** The main session makes Block 0 on the lane before `<base>`: builders run offline, a further
  manifest change stops the build, and `<base>` must compile.
  - **(a) spikes/netcode-proof/Cargo.toml:** `web-transport-quinn = { version = "=0.13.6", default-features = false }`
    (Cargo honours a member's `default-features = false` only when the workspace entry sets it); a new
    `rcgen = { version = "=0.14.10", default-features = false, features = ["ring"] }` with its comment.
  - **(b) crates/net/Cargo.toml:** `[features]` gains `default = ["wt-wtq"]` and `bites = []`;
    `web-transport-quinn = { workspace = true, optional = true, features = ["ring"] }`; `futures-util.workspace = true`
    moves from [dev-dependencies] to [dependencies].
  - **(c) crates/server/Cargo.toml:** `rcgen.workspace = true` in [dependencies]; `bites` gains `"npt-net/bites"`.
  - **(d) The provider lines that stop compiling without aws-lc-rs:** wt.rs:177-179 (`rustls::crypto::ring::
    default_provider()` and its doc line), wt_tests.rs:586-592's aws-lc-rs assertion, ws_config.rs:4's comment.
- **The expected lock diff:** npt-server's entry gains ` "rcgen",`; futures-util's move changes no lock line (the lock
  already lists npt-net's dev-dependencies); if nothing else turns aws-lc-rs on (the lock shows only quinn-proto, rustls
  and rustls-webpki depending on it), aws-lc-rs 1.19.0, aws-lc-sys and the build-only crates only they reach go, with
  their lines in those three entries; no package added; no version, checksum or source line changed.
- **The binding check** (in U6a's Block 0 shape, u6a-block0.md): (1) the manifest edits, then the lock refreshed
  offline (`cargo metadata --offline --format-version 1`, or `cargo check --offline` without `--locked`; never `cargo
  update` without `-p`); (2) the lock diff exactly as above; (3) `cargo tree --locked --offline -e features` shows
  rustls and quinn-proto without aws-lc-rs, web-transport-quinn with `ring` only, rcgen without `aws_lc_rs`, `fips` or
  `pem`; (4) `cargo check --workspace --all-targets --locked` 0 warnings, and check.mjs's test counts before and after
  Block 0 (the wt module's tests joining the measured build, by name); (5) `cargo deny --locked check advisories bans
  sources` and `cargo audit` on the final lock, clean as U6a's Block 0 defined it (u6a-block0.md:36-55), with the
  advisory database's date. Anything else returns to security-auditor before the save.
- **DEP-HOLD** (main's .claude/CHECKPOINTS.md §3): the save waits for security-auditor's answer on exactly this content
  and dependency-check.md (one line per changed file). security-r1-deps.md:223-224 clears futures-util and rcgen's pin;
  its binding check (:249-253) is written for U6 whole (url, done in U6a; instant-acme's 17, U6c's), so (a)'s
  web-transport-quinn line, the `default` and `bites` features, (c) and the aws-lc removal **need their own short
  read**, with its yes to cargo-deny 0.22.0 and cargo-audit 0.23.0 again (cleared for U6a's Block 0 only:
  security-u6a-deps.md section 6).

## 6. The rules that stand (as u6a-brief.md section 6 gives them)
- **Start:** one builder (general-purpose, opus, xhigh), `isolation: worktree`, which starts at main. First step, in
  its worktree: `git merge --ff-only <base>` (if it cannot fast-forward, `git switch -c u6b-build <base>`), then
  `git log -1 --format=%H` shows `<base>`; both in its notes first (u4f-brief.md:292-295). `<base>` is left for the
  main session to fill, after Block 0's save.
- **Notes:** `.checkpoints/netcode-proof-test/u6b-build.md` in the worktree: the task, the blocks, each finished block
  at once, a notes line per finding, `git diff --stat <base>`'s added lines after each block.
- **The shared target** `/home/user/Shamble-Frontlines/.gate/shared-target` and `CARGO_NET_OFFLINE=true`, every cargo
  call through a U6b runner (the main session's names-only copy of u6a-run.sh, logs under .gate/u6b/), the lane lock
  and the disk floors of u4f-brief.md section 4 rules 1-9.
- **Files:** U6b's lane is crates/net, crates/server (main.rs, cert.rs, the settings, tests, examples; TickLoop's seam
  only for the accept stamp), crates/link (`begin_tick` only, under Q-U6B-1 (a)), and marks/SERIES.md new rows only.
  Not U6b's: every Cargo.toml and Cargo.lock (Block 0's; a further change stops the build), proto, room, client, bots,
  lab, l1, log, judge, marks.json, marks/README.md, any existing SERIES.md row, check.mjs. A part that needs one stops,
  is reported, and the rest is built.
- **Bans:** `.claude/rules/netcode.md` (crates/server and crates/net): nothing unbounded keyed by client data (every
  queue bounded, every wait under a deadline); server/clippy.toml's clock and hash-map bans, with one reviewed
  `SystemTime::now` allow in cert.rs (calendar validity) and every other read through U6a's clock item; no client
  string or token in any log (E22); no Origin check and no `Url::origin()` on client input; SEC-U6AD-2's and -3's rules
  for every file U6b reads or writes; the dev key never written; bites per instance; tests on 127.0.0.1 only;
  LEAD-BOUND frozen at 400 ms.
- **SIZE-CHECK** (history.md:2048-2051): past 6,000 added lines the build stops, the end checks run on the tree as it
  stands, and the main session gives Mustafa three lines (what grew, why, the cheaper option).
- **Agents never commit;** the main session snapshots, merges and commits. The builder's brief carries, word for word:
  "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt`
  writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write."

## 7. Review
One round (NPT-TRIM), netcode-architect, test-engineer and security-auditor, each at xhigh; a second only for an open
Critical or High. Security joins because U6b's transports face the internet in every run (replan.md:93). Its reads:
both accept paths (the cap, the deadlines over TLS and HTTP/3, the size limits at the socket, panic-free framing), the
pin (signature still checked), the dev certificate (key in memory, `create_new`, the one wall-clock read), the public
key file's read, the bites gate, and U6a's token minting and file with the `Join` path (condition 6). Netcode: the pub
items against §4.7 (and `begin_tick` if added), fix 1's path, the reader stamp. Test: the plants, the bite, the counts.

## 8. Size
- **The re-plan's figure:** 2,205 estimated, 2,950-3,300 likely (replan-units.md:59).
- **This brief's:** about 2,700 (2,500-2,900): T1 500 (the cap's and deadline's WebTransport half, the closing task,
  +50); T2 330 (the pin promoted, +50); T3 300; T4 760 (`burst_return`'s server fields, +60); T5 560 (pooled L, the
  header reading from `&L2Config`, the per-peer profile, +160); S8b 250 (the binary's transport wiring and settings,
  OPEN's move, the hash file and the end-to-end tests, which u6a-brief.md:175-177 gives U6b and no row counted, +175).
  Block 0: about 12 manifest lines and 4 code lines. **Likely 3,650-4,050** at the units' 1.35-1.5. **The stop: past
  6,000** (SIZE-CHECK; replan.md:25: "each unit under the 6,000 stop at 1.5x").
- **The lane before R1:** U6a 3,150-3,800 + U6b 3,650-4,050 + U6c 2,650-2,950 + U7's Rust 490-540 = about 9,950-11,350
  likely lines (advisor-u6a-brief.md:136-137: 9,250-10,600). U6a's start a day early (Sun 22, the card) gives back
  about the day this adds. **Day line:** build from U6a's merge (Wed 25 if its round closes then), about two days;
  round, fixes and merge the day after. **Lever** (Q-U6B-6): the per-peer profile hook, the burst range check and
  `burst_return`'s server fields go to U6e (no R1 mark reads them): about 250 lines off the lane before R1. If R1 on
  Thursday 3 December slips, he gets three lines at once (U6a's condition 8).

## 9. Open questions (each with my lean; the advisor's check answers them)
- **Q-U6B-1, W18:** how WsServer sees a tick's whole snapshot (`Link` sends per message). (a) A provided
  `Link::begin_tick(lens: &[usize], now_ns) -> SendResult`, default `Ok`, which TickLoop's I/O side calls before a
  session's tick datagrams; WsLink runs the gate on it and answers each datagram of a refused tick BufferFull;
  WtLink, Loopback and L1 keep the default. (b) WsLink infers the tick from `now_ns` and holds sends to the next
  tick's first call: a tick of delay. (c) TickLoop calls WsServer through a transport enum instead of `dyn Link`:
  couples npt-server to npt-net's types. **Lean (a):** one default method on a leaf trait, nothing else changes, the
  room sees BufferFull on its own path; the netcode seat reads it against §4.7 (Q-U6A-7's pattern).
- **Q-U6B-2, aws-lc-rs:** wt.rs:177-179 uses aws-lc-rs for web-transport-quinn (its default), while replan-units.md:77
  says ring. (a) Defaults off, `ring` (Block 0 as written): one provider for every TLS configuration, no C build on RS,
  the lock shrinks, Block 0 carries 4 code lines. (b) Keep aws-lc-rs as built: no lock change, but aws-lc-sys's C build
  enters every measured build once wt-wtq is default. **Lean (a)**, on security-auditor's short read.
- **Q-U6B-3, the series R1's DelayStream self-test reads:** L per direction over 48 connections. DelayStream's
  per-second rows are summaries (delay.rs:1419-1460), so a merge across connections needs histograms. (a) L per
  direction added to the split's process pool and written in `ws_split` at the run's end (about 60 lines; Block 5);
  the per-second merge stays U6e's for R3's -31 and -18 rows. (b) The per-second merge in U6b (about 150 lines). **Lean
  (a)**; U8's brief states the self-test's judged form on it (C-U8-5), so U8's drafter and the main session align them.
- **Q-U6B-4, the hash file and the key** (U6c serves it, U7 reads it): **lean:** the key in memory only, never on disk;
  the room writes `{"algorithm":"sha-256","value_hex":"<64 hex digits>","validity_ns":<u64>}` by `create_new` to the
  settings' path; U6c's host serves it verbatim as `/npt/cert-hash.json`; U6c's and U7's briefs take this form. Else
  the advisor names another before Block 6.
- **Q-U6B-5, T5's SetProfile and B1 range check,** whose per-session controls are cut (replan-units.md:24-25) while
  :59 keeps them in T5. **Lean:** built here as WsServer hooks with their tests (the row's placement), their callers in
  U6e's ControlSocket; or they move with their callers to U6e (the lever, Q-U6B-6).
- **Q-U6B-6, the size:** about 2,700 against the re-plan's 2,205. (a) Build as placed; the lever only if the build
  runs long. (b) Apply the lever now (about 2,450). **Lean (a):** U6a's early start covers it, and the hooks are
  cheapest built beside the code they wrap.

## 10. For the main session
- **Now:** this brief to the advisor (one check); security-auditor's short read of section 5's content.
- **Before launch (on the lane, after U6a merges):** Block 0 (a)-(d), the binding check, cargo-deny and cargo-audit
  with the database date, dependency-check.md, the save; fill `<base>` and its counts; the runner copy u6b-run.sh; the
  size line (section 8) on the card before the first snapshot; the advisor's answers applied as rev 1.1 (no re-check
  unless it finds a Critical or High); U6c's and U7's drafters told Q-U6B-4's form, U8's Q-U6B-3's.
- **Close:** the builder's notes and return list (u4f-brief.md section 7's shape); snapshot; check.mjs in a merge
  worktree; the round; fixes; merge; U6b's report (counts, plants, bite, rows, size, carries by number, Block 0's lock
  diff and audit results, the container readings).
