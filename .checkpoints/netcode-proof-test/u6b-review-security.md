# security-auditor: U6b's one review round (xhigh), 2026-11-23

Subject: wip/u6b-build.patch (49f4488f...c14c, base 21765e3, 29 files, +5,498 -254), built tree W =
.claude/worktrees/agent-a2661dc320c9e6d00/spikes/netcode-proof (read only). Brief u6b-brief.md rev 1.1 (32baaebd...),
advisor-u6b-brief.md (3c0f8121...), security-u6b-deps.md (3799ac81...), builder's notes u6b-build.md (be6807f4...),
U8's tree .claude/worktrees/agent-ac59cce3e5d002441 (u8-build.patch 0e7fbab2...). All six hashes checked at the start:
equal. (No Edit tool in this agent: this file is rewritten whole with Write after each block.)

## 1. Plan
1. Inputs: hashes (done), brief §0.1/§1/§7, advisor check, Block 0 read, builder notes, replan.md:56-60, :93; the
   patch's file list; W equals the snapshot (spot check).
2. Accept paths: ConnCap (cap.rs), WtServer's accept loop (wt_server.rs + l2/wt.rs), WsServer's accept path
   (ws_server.rs): permit before TLS, refuse/drop, deadline over QUIC + HTTP/3 and TCP + TLS + upgrade, permit return.
3. Size limits at the socket and panic-free framing: WtLink's control stream (2 B length, 512 B), FullState stream
   (4 B, 65,541 cap), datagrams; WsLink's tag and tungstenite limits (server 513 B; client limits per condition 7);
   bounded queues; any `unwrap`/`expect`/index/slice reachable from bytes.
4. The pin (pin.rs): SHA-256 of the end-entity, signature still verified through the provider; both clients.
5. The dev certificate and hash file (cert.rs, main.rs): key in memory, `create_new` + 0644 at creation, one
   `SystemTime::now`, exit 2 not panic, the hash file's fields; P15's evidence.
6. The public key file's read (WsServer::bind_pem / main.rs): size cap, path, permissions, no key in logs.
7. The bites gate: `Bite::BudgetX2` cfg, per-instance arming, the binary's `--bite` path, measured build.
8. U6a's token minting + token file + Join path (condition 6), and U8's `npt-ctl rooms tokens NN` as `npt-room`.
9. security-u6b-deps.md's build-time conditions: one TLS provider (ring), no install_default/ClientBuilder/
   ServerBuilder/Client::default/.origin(), no tracing subscriber, no Url::origin on client input.
10. Log hygiene: no key, token, address, request line or URL in what U6b writes (event rows, header, stderr).
11. Evidence: check5.log and the plant logs I lean on (P1, P3, P4, P14, P15); a test run only if a claim needs it.
12. Verdict, table, not verified, DONE.

## 2. Notes (one line per finding, saved as I go)
(none yet)
