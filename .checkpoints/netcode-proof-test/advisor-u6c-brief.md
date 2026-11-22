# The advisor's check of U6c's brief, rev 1 (2026-11-22)
Brief: u6c-brief.md rev 1, 505 lines, sha256 13b0fe58777585525188adbd9d61cde4bc3a1d70df2306ae78cc88c6ba26ed5b (checked
equal at the start of this check). Drafter: netcode-architect (netcode-u6c-brief.md). Working by main's
`.claude/agents/advisor.md` (read with `git show origin/main:.claude/agents/advisor.md`). Rules read word for word:
NPT-TRIM history.md:7561-7593; his answers of 2026-11-22 history.md:7595-7607; TEST-FIRST history.md:2234-2282.

## Plan
1. Read the rules, the card, the brief and the drafter's notes. DONE (brief hash checked).
2. Read U8's brief rev 1.1 (C10, C1-C4, Q-U8-4/-5, :55-71, :200-240, :383, :416-423, :466, :505-521) and its check.
3. Read U6a's and U6b's briefs rev 1.1 and their checks (conditions the brief cites; Block 0 shape u6b-brief.md:393-400).
4. Read security-r1-deps.md (instant-acme, the 17, SEC-R1D-4, :249-253) and security-u6b-deps.md.
5. Grep the spike's code for each fact the brief leans on (stubs, socket_tests.rs lines, stats.rs, thread.rs, SERIES.md
   rows, check.mjs, clippy.toml, ws_config.rs, config.rs, lab probe, lock entries).
6. Rule the six contradictions, Q-U6C-1 to -8, size and R1's date, Block 0, and the rest; note each finding here.
7. Verdict, paste-ready conditions, answers. Hand back.

## Notes (one line per finding, as found)
- N0 plan.md gained one line at :379 on 2026-11-22 08:50 (commit 4d71571, C-U4F2-8's text), after replan-units.md
  (0d9ccac, 2026-11-21 23:17) was written: every plan.md cite past :379 in replan-units.md and replan.md is one low
  today (U6's row is :1254, U8's :1256; `/npt/cert-hash.json` is :995-996). This explains contradiction (f)'s row
  number; u6b-brief.md's "plan.md:994-995" and this brief's copies of it carry the same off-by-one.
- N1 (f) checked: socket_tests.rs last changed 1ccff06 (2026-11-09), so the carry's :1263-1264 still points at T7's
  doc; the doc block is :1263-1271 today, its stamp claims at :1263-1265 and again in the bite sentence :1270-1271.
  Stamp sites socket.rs:501-503 and :724-726 hold. Carries stay word for word (no document change); Block 2 rewords
  the whole block :1263-1271. Low.
- N2 instant-acme 0.8.5 API (docs.rs, read 2026-11-22): `Account::builder()` and `builder_with_root(pem_path)` are
  hyper-rustls-gated; `builder_with_http(Box<dyn HttpClient>)` and `Account::profiles()` exist; `Order::finalize()` is
  rcgen-gated, `finalize_csr(&[u8])`, `poll_ready(&RetryPolicy) -> OrderStatus`, `poll_certificate` exist;
  `HttpClient::request(Request<BodyWrapper<Bytes>>) -> BytesResponse`; `LetsEncrypt::{Production, Staging}.url()`.
  Block 6's calls all exist. But instant-acme's `ring = [.., "rcgen/ring"]` is non-weak (security-r1-deps.md:70-72),
  and Cargo turns on a same-named feature when a non-weak `dep/feat` names an optional dep, so `rcgen` (and
  `finalize()`) is very likely on: binding check (3)'s "instant-acme with `ring` and `hyper-rustls` only" would bounce
  the save to security for nothing. Low (fails safe); condition.
- N3 tokio-rustls's workspace entry is default-features off, "no TLS 1.2" (Cargo.toml); hyper-rustls brings rustls's
  `tls12`; ws_config.rs:31-32 names TLS 1.3 explicitly, so WsServer holds; Block 5 must name it the same way (brief
  says so). QUIC is 1.3 by protocol. OK.
- N4 Medium: the probe's peer port. `ImpairSocket::set_peer` takes a full SocketAddr (socket.rs:244); the brief pins
  "LG's IPv4" and gives `probe_peer` an ephemeral port, and never says the down pacer waits for LG's first up datagram.
  On the VMs `fw-lg` admits only TCP 22 from RS and ICMP (u8-brief.md:200-201): down datagrams RS sends before LG's
  flow exists are dropped at Hetzner's firewall; loopback tests cannot see it, so it would surface as an INVALID -3 at
  R1. Fix: LG sends first; RS takes the port from the first datagram from LG's IPv4, pins it, then starts the down
  pacer; LG keeps one socket for all legs and tolerates ECONNREFUSED between legs.
- N5 Medium: the echo rule. L1's probe echoes "the newest up datagram the server has received and not yet echoed"
  (lab/src/probe.rs:7) and L2 "keeps this design" (:10-11); SERIES.md:115 files "one per probe round trip". The
  brief's LG echoes "the newest down datagram it has received" in every up datagram (Block 1, C-U6-6), so a down
  datagram is echoed again whenever the next one is late or lost: duplicate samples in a judged series (PM-0001-3's
  L2 P1 RTT mean, marks.json:1009-1030). Fix: not yet echoed, else none.
