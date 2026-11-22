# netcode-architect: U6c build brief, rev 1 (plan and notes)

Task (main session, 2026-11-22 15:1x UTC): draft U6c's build brief rev 1 into `u6c-brief.md`; the main session sends it
to the advisor agent for its one check (NPT-TRIM). I build nothing and edit no code. (No Edit tool in this run: this
notes file is rewritten whole with Write at each step.)

## Plan
1. Read replan-units.md §1 (U6c row :60, dispositions), §3 (certificate by DNS-01 by hand), §7 (lane); replan.md:61-65,
   :89-94 and :93 (security seat).
2. Read u6-brief.md rev 2.1, u6-facts.md, advisor-u6-brief.md (conditions on U6c's blocks by number).
3. Read plan.md:1254 (U6's row: C-U6-1..8 word for word, the -3 probe's carries).
4. Read u6a-brief.md rev 1.1 (section 0.1: C-U6-4, `StealReader`, npt-super's hold form; section 6 rules) and
   advisor-u6a-brief.md.
5. Read u6b-brief.md rev 1.1 (section 0.1: condition 4, the hash file) and advisor-u6b-brief.md.
6. Read u8-brief.md rev 1.1 section 0.1 (C10, Q-U8-4, Q-U8-5, C4, C-U8-2, C-U8-6) and advisor-u8-brief.md.
7. Read security-r1-deps.md (instant-acme =0.8.5, 16 crates, SEC-R1D-4, DEP-HOLD :249-253) and dependency-check.md.
8. Check the code on the branch for what U6c touches (npt-server, probe, npt-bots, npt-load, page host) to name symbols.
9. Write u6c-brief.md in u6b-brief.md's shape; the task's sections 1-8.
10. Check every word-for-word quote as a substring of its source; hand back a 15-line summary with contradictions cited.

## Notes (one line per finding)
- Step 0: no netcode-architect memory dir (.claude/agent-memory-local holds security-auditor only); earlier decisions are
  this folder's notes (netcode-u6a-brief.md, netcode-u6b-brief.md) and the briefs.
- Step 1 DONE. N1 U6c row replan-units.md:60 = 750+150+600+300+150 = 1,950, likely 2,650-2,950 (x1.35-1.5 = 2,633-2,925).
  Cut list (:11-32) touching U6c: page host's 5 s header timeout, 8 KB cap, per-address/total caps (:13-14); 404 bite,
  `/iso/`, robots, noindex (:26-27); HTTP-01 responder, npt-cert's port window, `revoke` (:32); E14 floods (:11). Kept
  (:33-36): control socket parse etc. (U6e); lag comp series to U6e. §3 (:86-121): DNS-01 by hand, 4 steps; wrapper
  polls Cloudflare's authoritative servers directly <= 30 min, no retry loop; `shortlived` required in production,
  fails closed; key 0640 + chain; prints serial and hash; staging then production at R1; no revoke. §7 (:192-216):
  U6c Fri 27-Sat 28; Stop B's read from U6c's merge (reads page host, control plane). replan.md:93: security on U6c.
- N2 replan.md:70-71: R2, R3, R5 may skip the public order (LGs pin the room's own cert). replan.md:68: "the server
  checks Cloudflare's own name servers and finishes"; :69-70 "it is not built: a failed order stops the run".
- Step 2 DONE. N3 old-U6 conditions on U6c's blocks (old U6d names): U6-Q2 (B1 in U6d after C-U6-18, with clock-ban
  allows); U6-Q17 term 3 (cert-hash.json = U6d's, now U6c H1); U6-Q18 (no 90-day fallback; root store at Block 0 by
  security's read; revocation call confirmed by name -- now moot: revoke CUT, replan-units.md:32; fake directory in
  tests); Q11/C-U6-6 (10,000 a direction else advisor before R1's run plan); Q12 (engine-architect if the probe lands in
  npt-server); conditions 1 (SERIES.md rows named), 6 (Block 0), 7 (xhigh). u6-brief.md:124 old U6d reviewers.
- Step 3 DONE. N4 plan.md:1254 split on <br>: C-U6-1..8 at bullets 24-31 (scratch copy not written; quotes checked
  against the line itself). C-U6-8's bullet ends with no period ("drops out").
- Step 4 DONE. N5 u6a-brief.md: condition 5 (:52-53) "`StealReader` stays pub(crate); U6c's brief makes it pub if its
  probe lands outside npt-server"; condition 3 (:37-40) npt-super hold form (stdin held open, U8 starts every room
  through npt-super); :180-183 U6c's list and pins; :163-165 "U6c's npt-load hosts through it" (host_step); section 6
  rules :294-317 incl. Write/Edit line :314-317; condition 2 gate line :30-34; U6e owns npt-super's density harness.
- Step 5 DONE. N6 u6b-brief.md condition 4 (:60-66): hash file struct algorithm/value_hex/validity_ns, create_new 0644,
  "U6c's host serves it verbatim with `Cache-Control: no-store` (plan.md:994-995), read per request or after each room
  start, under a size cap"; Block 6 (:210-212) WS certificate = public chain and key files from U6c's `npt-cert`, read
  under a size cap; U8 sets the key file's group (:241-242). WtClient/WsClient with pin (Block 1-3) are what npt-load
  hosts. ConnCap N + 8 per process.
- Step 6 (u8) read. N7 C10 (u8-brief.md:97-101): U6c takes Q-U8-4's interface, Q-U8-5's reads, the probe's ports and
  npt-load's control interface for C4; not the knob. Q-U8-4 (:144-149): one process both phases, "go" on stdin, no
  account key or order state on disk between phases, every authoritative server (cora, steven), one 30-min bound, dig
  no caller argument. Q-U8-5 (:150-153). C4 (:65-71): npt-load-ctl starts, feeds and stops npt-load, copies token
  files, runs C5's check; "Its verbs follow the npt-load interface U6c's brief names". Block 2 (:203-204): probe port
  outside UDP 30001-30032 / TCP 31000-31032 must be named. Block 3 npt-cert (:227-232): runs U6c's issue mode as
  `npt-host`, name `_acme-challenge.rs.nt.shamblefrontlines.com`, dig every 15 s <= 30 min, egress to the one ACME host.
- N8 STALE in u8-brief.md rev 1.1 (superseded by 0.1 C10, which says to replace them): :521 "U6c's takes ... Q-U8-6
  (i)'s knob", :383 "(Q-U8-6's knob, if taken, is U6c's)", :505-507 Q-U8-6 (i) lean "rides in U6c on the lane". Not a
  real contradiction (0.1 wins by its own rule) but the advisor should know U6c takes no knob.
- N9 u8-brief.md :249-250 `probe` block checks C-U8-2/-6 reads present; :392 R1's L2 probe 0.20 h "(0 added hours)";
  :389 calibration 0.10 h "load spread"; C-U8-4 run conditions (rate, C-U8-6 no-drop) checked by probe block.
- Step 7 DONE. N10 security-r1-deps.md: 17 new = instant-acme + 16 (:82-85, :228-230); instant-acme's `ring` feature
  is non-weak and turns rcgen on WITH instant-acme's own `features = ["pem"]` (:70-72): after U6c's Block 0 rcgen
  unifies to crypto+pem+ring workspace-wide, so U6b's Block 0 check item (3) "rcgen without ... `pem`" no longer holds
  after U6c (expected, pem 4.0.0 + base64 0.23.1 are 2 of the 17). DefaultClient: https_only, platform verifier or
  with_roots (:76-78); no request timeout, bodies uncapped (:77, SEC-R1D-4 :212-214); RetryPolicy 30 s default
  (order.rs:581-586); NewOrder::profile types.rs:456-462; dns_value order.rs:555. Binding check :249-253 written for
  "U6" whole (url bump done in U6a's Block 0): U6c's slice is the 17 packages only. webpki-roots absent; root store at
  Block 0 by security's read (U6-Q18): lean platform verifier (rustls-native-certs already locked).
- N11 hyper is client+http1+http2 only (:73-74): no `server` feature, so the page host must be written on
  tokio-rustls + httparse (both locked) or turn hyper's `server` on (a feature change, its own read). Lean: small
  GET-only HTTP/1.1 over tokio-rustls, httparse.
- N12 multi-room hash path: U8 C3 (u8-brief.md:55-63) runs a room per mark and browser, each room makes its own dev
  certificate (U6-Q17), so one `/npt/cert-hash.json` is ambiguous with >1 room: Q for the brief (per-slot path).
- N13 plan.md:964-983 (§8.4): staging "R1 reads and records both directories' profile lists"; if staging lacks
  shortlived the dry run uses staging's default, production still requires it; key 0640 readable by a group of the
  room user(s) and npt-host; new ACME account per VM. Stale there (U8 notes): http-01, revoke, TCP 80.
- N14 docs.rs instant-acme 0.8.5 (read 2026-11-22): `Account::builder()` (hyper-rustls), `builder_with_root(pem_path)`,
  `builder_with_http(Box<dyn HttpClient>)`; `Account::profiles() -> impl Iterator<Item = ProfileMeta>`; `new_order(&NewOrder)`;
  `Order::authorizations()`, `poll_ready(&RetryPolicy) -> OrderStatus`, `finalize()` ONLY under instant-acme feature
  `rcgen` (features page: rcgen, ring, hyper-rustls, time, x509-parser, aws-lc-rs, fips, default), `finalize_csr(&[u8])`,
  `poll_certificate(&RetryPolicy) -> String`; `ChallengeHandle::key_authorization()`, `set_ready()`; HttpClient::request(
  Request<BodyWrapper<Bytes>>) -> Pin<Box<dyn Future<Output=Result<BytesResponse,Error>>+Send>>; BytesResponse pub
  fields parts, body; From<Response<B>>. So: finalize_csr with an rcgen CSR (not relying on the `rcgen` feature's
  cfg), and a fake HttpClient in tests (needs dev-deps http, bytes, http-body-util).
