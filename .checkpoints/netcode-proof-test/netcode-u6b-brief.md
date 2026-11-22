# netcode-architect: U6b build brief, rev 1 (plan and notes)

Task (main session, 2026-11-22): draft U6b's build brief rev 1 into `u6b-brief.md`; the main session sends it to the
advisor agent for its one check (NPT-TRIM). I build nothing and edit no code. (No Edit tool in this run: this notes file
is rewritten whole with Write at each step.)

## Plan
1. Read replan-units.md §1-§2 (U6b row :59, C-U6-12..19 dispositions :42-50, §7 lane), replan.md:61-65 and :93.
2. Read u6-brief.md rev 2.1, u6-facts.md, advisor-u6-brief.md (conditions on U6b's blocks by number).
3. Read plan.md:1254 (U6's row: U3c carries for WsServer, C-U6-12..19, C-U6-17's form).
4. Read u6a-brief.md rev 1.1 + advisor-u6a-brief.md (cond 6 carries, cond 2 C-U6-19 gate line, section 6 rules).
5. Read security-r1-deps.md (futures-util, rcgen, DEP-HOLD :249-253) and u6a-block0.md (how Block 0 ran).
6. Check the code on the branch for what U6b touches (npt-net, npt-server, link, registry sources) to name symbols.
7. Write u6b-brief.md in u6a-brief.md's shape; sections 1-8 of the task.
8. Hand back a 15-line summary with contradictions cited.

## Notes (one line per finding)
- Steps 1-5 DONE. No netcode-architect memory dir (.claude/agent-memory-local holds security-auditor only); earlier
  decisions are this folder's notes (netcode-u6a-brief.md, u6a-brief.md rev 1.1, advisor-u6a-brief.md).
- N1 row drift: U6's row is plan.md:1254 (replan-units.md:3 cites :1253); quotes from :1254 (split on `<br>`).
- N2 U6b's row (replan-units.md:59): 450+280+300+700+400+75 = 2,205. T6 left U6b's uncut list: `Send` in U6a, `loss`
  and space to U6d with rate adaptation (Q-U6A-4).
- N3 cut from the uncut T1-T5 (replan-units.md:11-31): second crate's new code, C-U6-12, Origin checks, -25's
  DelayStream question (:26), per-session SetProfile/TriggerBurst/BlockUdp controls (:24-25), yet :59 keeps "SetProfile"
  and "B1 range check" in T5: read as the DelayStream method + WsServer per-peer hook and the range check before
  add_burst, called later by U6e's ControlSocket (RoomControl has SetPeerProfile and PeerBurst ≤ 1,000 ms:
  proto/src/control.rs:363-384).
- N4 S8b (:59, 75) keeps the cert at start; live swap CUT (:31). U6-Q17 terms: (1) per-slot 0700 dirs cut (:28-30) ->
  lean key in memory only; (2) swap cut; (3) cert-hash.json is U6c's H1; (4) PM-0006-27 MOVED (replan-marks.md:98), so
  no 15-day setting. PM-0006-25 KEPT at R1 (replan-marks.md:96): reads header.params.cert_validity_ns <= 14 d.
- N5 advisor-u6a condition 6 carries; condition 2 gate line; condition 3 OPEN move text.
- N6 Code (step 6): l2/wt.rs:177-179 wtq provider is aws-lc-rs (not ring, contra replan-units.md:77-78 "as l2/wt.rs
  does"); wt_tests.rs:586-592 tests aws-lc-rs. web-transport-quinn 0.13.6 features: default ["aws-lc-rs"], `ring`
  exists; its crypto::default_provider panics with both its features on. Lock: aws-lc-rs depended on by quinn-proto,
  rustls, rustls-webpki only; wtransport defaults ring; quinn default rustls-ring. So wtq default-features off +
  ["ring"] likely REMOVES aws-lc-rs/aws-lc-sys from the lock (removal-only diff) and breaks wt.rs:179's compile ->
  Block 0 must carry the 4 provider lines. Edition 2024 needs the workspace entry's default-features = false too.
- N7 npt-net has no `default` feature (wt-wtq optional, net/Cargo.toml:12-15); check.mjs measured build = no
  features (default on), bites = --all-features (check.mjs:2017, :2057): WtServer needs `default = ["wt-wtq"]` to be
  in the measured build; that moves wt_tests' wtq tests into the measured count at Block 0.
- N8 no rcgen workspace entry; rcgen has no `time` re-export, and CertificateParams.not_before/after are
  time::OffsetDateTime: lean `rcgen::date_time_ymd` from a civil date, else a manifest stop. Calendar validity needs one
  reviewed SystemTime::now allow (server/clippy.toml bans it).
- N9 Link (link/src/lib.rs:64-88) is per-message sync poll; no tick boundary (W18): lean a provided
  `begin_tick(&[usize], now_ns) -> SendResult` default Ok. Link has no close: Kick-before-close must live in Drop
  (a bounded closing task holding the permit).
- N10 pump.rs: JOIN_DEADLINE_NS 2 s from accept_ns given to Decoder::new (:13-14, :253-260): the accept stamp must
  travel with the link (TCP accept / first QUIC packet); U6a's seam may lack it.
- N11 DelayStream: no set_profile; header_numbers(&self) per connection, blocks (F6) -> lean an assoc reading from
  `&L2Config` once at start; split_rows pub(crate) -> pub for `ws_split`; POOL never reset (delay.rs:185-195); per
  second take_second returns summaries, not histograms (:1419-1460): R1's self-test needs L pooled over 48
  connections -> lean add L per direction to the split's pool.
- N12 ws_gate.rs's Bite is cfg(test) only (:58-69); bites feature absent; PM-0001-18/ws-budget-x2 fails the WS median
  check on L2 B1 ws (marks.json), judged at R3.
- N13 web-transport-quinn's ServerFingerprints is private (client.rs:283); tls_test.rs's pinning verifier is cfg(test):
  promote it for WtClient/WsClient.
- N14 security-r1-deps.md:249-253 is written for "U6" whole (17 instant-acme packages = U6c's; url done in U6a):
  U6b's slice adds no package; its own content (wtq features, rcgen entry, features, aws-lc removal) needs a short read.
- Step 6 DONE. Step 7 (brief) next.
