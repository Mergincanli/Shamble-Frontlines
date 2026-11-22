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
- Step 3 DONE. N4 plan.md:1254 split on <br>: C-U6-1..8 at bullets 24-31. C-U6-8's bullet ends with no period.
- Step 4 DONE. N5 u6a-brief.md: condition 5 (:52-53) StealReader pub(crate) unless the probe lands outside npt-server;
  condition 3 (:37-40) hold form; :180-183 U6c's list and pins; :163-165 host_step; section 6 :294-317; gate :30-34.
- Step 5 DONE. N6 u6b-brief.md condition 4 (:60-66) hash file; Block 6 (:210-212) WS cert files from npt-cert under a
  size cap; U8 sets the key's group (:241-242). ConnCap N + 8.
- Step 6 DONE. N7 C10 (u8-brief.md:97-101); Q-U8-4 (:144-149); Q-U8-5 (:150-153); C4 (:65-71); Block 2 (:203-204);
  npt-cert (:227-232). N8 STALE u8-brief.md :521, :383, :505-507 (knob as U6c's) superseded by C10. N9 probe block
  checks C-U8-2/-6 (:249-250); R1 probe 0.20 h, calibration 0.10 h (:389-394).
- Step 7 DONE. N10 security-r1-deps.md: 17 new (:82-85, :228-230); instant-acme `ring` turns rcgen on with `pem`
  (:70-72); DefaultClient (:76-78); SEC-R1D-4 (:212-214); binding check (:249-253) for "U6" whole. N11 hyper has no
  `server` feature (:73-74): page host on tokio-rustls + httparse (locked 1.10.1). N12 one hash URL ambiguous with
  several rooms (U8 C3). N13 plan.md:964-983 (staging profile lists recorded; key 0640 group of room user and npt-host).
  N14 docs.rs instant-acme 0.8.5: builder()/builder_with_root/builder_with_http; Account::profiles(); new_order;
  finalize() only under feature `rcgen`; finalize_csr(&[u8]); poll_ready/poll_certificate(&RetryPolicy);
  ChallengeHandle key_authorization/set_ready; HttpClient::request; BytesResponse pub parts/body, From<Response<B>>.
- Step 8 (code) DONE, at 7ae83dd. N15 SERIES.md:497-501 already fixes the probe's home: "npt-server's binary in a probe
  mode (process `probe`), on a free slot port of RS with its own `ImpairSocket`"; RTT "the echo's release minus the
  original send"; pacer "a dedicated thread with 1 ns slack, never tokio (M13)"; "LG's side, npt-bots (probe), sends
  up, echoes, and writes no probe series". SERIES.md:111-117 the seven `second.s.probe_*` rows (L2 writer named).
- N16 ADR 0001:1590 (-3's row): "10,000 packets per second per direction"; injected delay (profile RTT - base RTT)/2,
  base = median of 10 s of pings at session start; L2Config.base_rtt_ns exists (config.rs:24-26).
- N17 -3's bites are all npt-l1's (l1/src/lib.rs:780-782) plus not-net-of-base (MP's part, MOVED: replan-marks.md:20):
  U6c builds no bite.
- N18 The E machinery is test code today: start_poller socket_tests.rs:676-713, samples :735, nearest_rank :759,
  excess :775, forms :793, agree :904, pre-touch :1384-1408 (test-only access to `fates`), fate_entries :992 (Fate and
  FateLog::entries pub(crate)/cfg(test): stats.rs:426-505; canonical 56 B :446-466). The probe needs them shipped:
  promote into an npt-net module; socket_tests then calls the shipped ones (T1/T2/T5 re-run by construction).
- N19 Slack is set through /proc (thread.rs:1-60, pub(crate) own_tid/set_own_slack), no unsafe: pacer helper in npt-net.
- N20 C-U6-8 sites: up lock :501 then clock :503; down stamp `sh.now_ns()` after try_send :724-726; T7's doc
  socket_tests.rs:1263-1265; agree :904-910.
- N21 npt-bots already depends on npt-net (bots/Cargo.toml); clippy.toml bans Instant/SystemTime and tokio::time::Instant
  crate-wide ("U6's runtime half adds its reviewed allows (§4.11)"). npt-host: only npt-log, no clippy.toml.
- N22 check.mjs deps step: npt-host must not reach sim/room/proto/client (check.mjs:101-103); no external allow-list for
  host or bots, so instant-acme passes the deps step.
- N23 deny.toml: multiple-versions = "warn" (base64 0.22/0.23 and pem 3/4 duplicates add warnings, not errors); the
  lock's existing `"base64"` and `"pem"` references gain version suffixes when the second versions enter.
- N24 SERIES.md:432-433 + README:269-276: npt-load writes `second.s.inputs_count` (one per client, slot order) and
  `second.v.cpu_permille` (LG CPU, < 800: lg-80/V-S). plan.md:473 E13: 5-minute calibration under 80 % CPU every second,
  inputs within 1 %.
- N25 plan.md:1117-1135 §10.4: allow-list (index, scripts, Worker, Wasm, cert-hash.json, ssh-host-keys.txt, robots),
  GET/HEAD, CSP text, nosniff, Referrer-Policy, no-store on the two files, application/wasm. Cut: limits, bite, /iso/,
  robots, noindex.
- N26 CONTRADICTION (U8/re-plan, touches U6c): plan.md:286-290 (§1.6) has the laptop fetch
  `/npt/ssh-host-keys.txt` over the production certificate BEFORE the first SSH; under DNS-01 by hand the order is
  started by the laptop session over SSH (replan-units.md:110-111; u8-brief.md:416-417 certificate after bring-up), so
  the certificate cannot exist before the first SSH. u8-brief.md has no step for it.
- N27 GAP (U8): no wrapper starts `npt-host serve` as `npt-host` (u8-brief.md Block 3 :205-240 lists none).
