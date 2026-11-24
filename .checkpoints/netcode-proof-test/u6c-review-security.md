# U6c's review round, the security seat (security-auditor, xhigh), 2026-11-24

## 1. Plan
- Inputs' hashes checked first (all seven match the launch: brief 8f56f37a..., advisor 9f5ab8d0..., launch bf8016a8...,
  deps c51fbdbb..., lock ae9883dd..., patch a4e63455..., build notes 426b31da...). Worktree W =
  /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof; check W equals the
  snapshot by `git hash-object` against the patch's post-image lines (no `git -C`).
- Read in this order, a notes line per finding as I go:
  1. crates/host: main.rs (dispatcher), serve.rs (allow-list, exact match, query, files and caps, TLS 1.3, ALPN,
     headers, one 8 KiB buffer and 32 header slots, no request log, per-connection deadline or none), serve_tests.rs
     (the TLS-1.2-only-client test, the 404 set, the buffer test).
  2. crates/host issue.rs and issue_tests.rs: the fixed words, two arguments, the compiled-in directory, the directory
     check, the constant URLs, `Account::builder()` vs `builder_with_http` under cfg(test), the key in memory, the
     profile failing closed (production without `shortlived` exit 3), no retry (RetryPolicy, Invalid exit 4), the
     per-call timeouts and the 30-minute bound, the stdin `go` read under a byte bound, `create_new` and 0640/0644,
     INF-SB-6's temporary name then rename, SSL_CERT_FILE/SSL_CERT_DIR, stdout lines exact, no secret in any print.
  3. crates/bots load.rs: the token and pin reads under size caps, never logged; settings rules (SEC-U6AD-2).
  4. crates/server main.rs and probe.rs: probe-mode settings, the one pinned peer (set_peer, other sources dropped and
     counted), ProbeDatagram decode panic-free, rate and allocation bounds; npt-net l2/probe.rs's probe_peer.
  5. Conditions 2-4 of security-u6c-deps.md against the code, each by file:line (grep for install_default,
     set_default, builder_with_http, with_protocol_versions, verify_tls12_signature, tokio features).
  6. F-REDACT: grep W and .gate/u6c logs for PEM blocks, key-like strings, nptj_ tokens; F-TERM and F-LG-SHA views.
  7. A test run only where a claim needs it, through u6c-run.sh, one command per call.
- Output order as asked: plan, notes, verdict with the Stop B line, table, Conditions 2-4, not verified, DONE.
- No Edit tool in this session: this file is rewritten whole with Write after each block.

## 2. Notes (one line per finding, saved as I go)
- N0 (fact): W equals the snapshot: all 24 files' `git hash-object` equal the patch's post-image index lines; Cargo.toml,
  Cargo.lock, crates/bots/Cargo.toml and crates/host/Cargo.toml hash equal to 3b75d00's (f376134a, 46d2498c, 474afe77,
  3a2142df): no manifest or lock change. Files touched after the counted run's start (09:46 UTC, mtime 10:07, check.mjs's
  doing) differ from 8c81952 only in the patch's own files, each equal to the snapshot.
- N1 (cleared): serve.rs allow-list is the seven U7 names, compiled in (serve.rs:34-46), read once at start under 32 MiB
  each, a missing one exit 2 (:132-139, :277-280); path up to the first `?` matched by `==` (:191-192), the query never
  read; per-room hash `/npt/cert-hash-NN.json` NN 00-31 under 1 KiB, `no-store`, absent 404, oversized 500 (:174-182,
  :199-209); no plain `/npt/cert-hash.json`; host keys under 4 KiB with `no-store` (:195-198); GET/HEAD else 405
  (:187-190); CSP word for word, nosniff, no-referrer on every answer (:150-171); one 8 KiB buffer, 32 header slots, a
  request that does not fit closed (:215-238); TLS 1.3 only by builder_with_provider(ring) + with_protocol_versions(TLS13),
  ALPN http/1.1 (:123-131); no per-request log (stderr only at start :292 and fatal :268). No panic reachable from a
  request (no indexing past `n < buf.len()`; tasks hold their own panics). The TLS-1.2-only client test
  (serve_tests.rs:333-343) compiles only with rustls's tls12 on, so it refuses a TLS 1.2 that exists.
- N2 SEC-U6C-1 (Low): serve.rs:253-255 `let Ok(..) = listener.accept().await else { continue; }` retries at once. Under
  fd exhaustion (EMFILE: U8's npt-host.service sets no LimitNOFILE, so systemd's soft 1,024; no connection cap and no
  handshake or read deadline, by the cut) accept fails at once, again and again, and the loop spins one of the two workers
  at 100% on RS beside the measured rooms. hyper documents the case (AddrIncoming::set_sleep_on_errors: EMFILE, sleep
  1 s, default on). Fix: on Err sleep 100 ms before the next accept.
- N3 SEC-U6C-2 (Low): serve.rs:193 with :166-169 copies each body into a fresh Vec per answer (the Wasm, up to FILE_CAP
  32 MiB); with no connection cap and no write deadline (cut), a client on an allow-listed source that requests the Wasm
  on many connections and never reads holds one copy each (connections x Wasm size, up to the fd limit). Fix: write the
  head, then the shared body slice from `self.files`, with no copy.
- N4 (cleared, considered): per-request opens follow symlinks and would block on a FIFO (serve.rs:78-85), but only
  npt-room writes /var/lib/npt/hash and npt-room already reads production.key.pem by design (2750 npt-host:npt-room, key
  0640), so a planted link to the key gives npt-room nothing it lacks. No finding.
- N5 SEC-U6C-3 (Low, nit): serve.rs:202 `n.parse::<u8>()` accepts a leading `+` (core's from_str_radix: "an optional `+`
  sign followed by only digits" for unsigned types; src/core/num/mod.rs), so `/npt/cert-hash-+5.json` passes the
  two-character filter and opens `room-+5.json` (absent: 404). No traversal (two characters, no `/` or `.`), but the
  per-room set is 42 names, not 32. Fix: `n.bytes().all(|b| b.is_ascii_digit())`. The same `+` passes load.rs:265's room
  check (operator settings only).
- N6 (cleared): issue mode: exactly two arguments (main.rs:20-21, args.rs's cases incl. `issue staging /tmp`); the word
  matched before any path is built (issue.rs:162-169), constant URLs `LetsEncrypt::Staging|Production.url()` (:163-164),
  compiled-in DIR and NAME (:33-34, :300); the directory check owner = euid and no group/other write (:51-72); every
  output name, the temporary one included, refused before any call when it exists (:171-176); `Account::builder()`
  outside tests (:177-178), `builder_with_http` only under cfg(test) (:179-180, the field :45-46); `create` makes the
  account key in memory (instant-acme account.rs:432-446, `Key::generate_pkcs8`) and its credentials are dropped
  (:186-193); no contact (:182); production without `shortlived` exits 3 before any order (:197-203); each call under
  60 s and both polls under 360 s (:123-134, :187-252), the whole under 30 minutes (:283-294); `go` read on a std thread
  under 16 bytes, exactly `go\n` (:137-153); a non-Ready poll exits 4, nothing re-ordered (:233-237); a fresh P-256 key
  and CSR by rcgen, `finalize_csr` (:238-245); the key PKCS#8 0640 by create_new with the mode set at creation and again
  by fchmod, write_all, sync_all (:111-120, :272); stdout holds only local strings (the constant profile word, NAME, the
  computed TXT value, the serial and hash parsed locally: :223-229, :276-279); the DER serial parse is panic-free
  (:75-107). PROFILES and error texts on stderr carry server text, but only Let's Encrypt behind the platform verifier
  can supply it.
- N7 (cleared, INF-SB-6's security side): issue.rs:273-275 writes the chain to `.<word>.chain.pem.new` (0644, synced),
  then `hard_link` to `<word>.chain.pem`, then unlinks the temporary name. The chain's name appears at once with its
  whole, synced content after the key's sync_all, as a rename would, and link(2) refuses an existing chain (EEXIST, exit
  2) where rename(2) would replace it: stronger than the words. No directory fsync: durability only; a crash leaves the
  key without the chain, and the next run refuses (the key exists), fail closed. The netcode seat reads the wording.
- N8 (cleared): npt-load's reads: settings under 64 KiB, deny_unknown_fields, every text <= 256 B without control
  characters, ranges checked (load.rs:41, :193-301, :320-327); the token file under 8 KiB, one token a slot 0..n-1,
  22 base64url characters to 16 bytes with the pad bits zero, an error naming the line only, never the token
  (:42, :338-388, :509-519), not read in probe mode (:509-510); the pin is `pin_hex`, 64 lowercase hex checked at parse
  (:288), decoded at :330-336, given to WtClient/WsClient, both on pin.rs's client_config (wt_server.rs:605,
  ws_server.rs:943). No token in the header params (:441-448) or any print; tests/load.rs:183-194 asserts none in the log,
  stdout or stderr. The log made by create_new (:520-536).
- N9 (cleared): the probe's settings: `mode` room|probe, probe fields refused in room mode, peer an IPv4, legs from P0 in
  order once each, packets 1-10,000,000, warm-up 1-600 s, port 0 refused (main.rs:316-367; test :1201-1293); no token,
  no hash file, no TCP bind (probe.rs binds one ImpairSocket a leg). Allocation is sized by the settings only
  (probe.rs:454-469), the intake's records bounded with a drop count (:297-301). ProbeDatagram::decode is length- and
  magic-checked with no panic (net probe.rs:295-310); leg_series takes the echo index from the wire through
  `down_t_in.get(i)` and the hold through checked_sub (net probe.rs:746-760): panic-free. The pin's IPv4 check means the
  down pacer can aim only at LG's address, so the probe is no reflector to a third party. LG's reader uses a connected
  socket (net probe.rs:480-481), so only RS's address reaches it.
- N10 SEC-U6C-4 (Low): server probe.rs:279-281 pins the first datagram whose source IPv4 is LG's before any decode, as
  C2 words it. A stray or spoofed datagram from LG's IPv4 with another port, arriving between OPEN (or a leg's bind) and
  LG's first datagram, pins that port: the down pacer aims at it, LG's real datagrams count as `probe_other_source_count`
  and the leg fails (P0: "no round trip") or, for a later leg, runs without echoes. Loud, never a false pass, and blind
  spoofing of a source the firewall admits is needed. Fix: pin only on `ProbeDatagram::decode(b, None).is_some()` (not
  `Some(self.leg)`: LG still sends the previous leg's number until the new leg's first down).
- N11 SEC-U6C-5 (Low, process): issue.rs:181-185 sends `terms_of_service_agreed: true`: each order agrees to Let's
  Encrypt's Subscriber Agreement on Mustafa's behalf when he runs `npt-cert issue`, and no plan, brief or U8 printout
  names it (grep of plan.md, history.md, docs/: no hit). RFC 8555 section 7.3 lets the server require it, so the line is
  needed; the gap is that he is not told. Fix: one line in R1.md's certificate block (or npt-cert's first printout)
  naming the agreement and its URL from the directory's `meta.termsOfService`.
- N12 (views): F-TERM: no security effect; a missing footer is the honest "stopped" record, and TERM comes only from
  npt-load-ctl. F-LG-SHA: provenance holds through U8's `sha256sum /opt/npt/bin/*` at LG's bring-up
  (u8-build.patch:822), copied into the results (:2296); a log cannot name its binary by itself, acceptable for R1.
- N13 (F-REDACT, confirmed): the snapshot patch holds no PEM marker (0 matches of BEGIN or PRIVATE KEY), no `nptj_`
  followed by 22 characters (the token texts are format strings and a 0..15 byte fixture, load_tests.rs:79), and no
  secret-like literal (the only 40-hex string is the test commit 0123...4567); keys and certificates are made at run time
  by rcgen in temporary folders. In .gate/u6c (gitignored, .gitignore:2) only all-fmt.log:1184 and b56-fmt.log:409 hold
  the old assert's header literal, no key body; no token in any log. end-check2.log: redact PASS, 371 items, 0 findings;
  issue_tests.rs:266-271 now parses the key as PKCS#8.
- N14 (input for Stop B's SEC-SB-9 rest, from the code): `npt-host serve` reads /etc/npt/host.json, the production chain
  and key, the seven dist files at start and, per request, /var/lib/npt/hash/room-NN.json and the host-key file; it
  writes nothing, reads no /proc, makes no child, uses no JIT, binds one AF_INET TCP socket. So ProtectSystem=strict,
  PrivateDevices=yes, MemoryDenyWriteExecute=yes, RestrictAddressFamilies=AF_INET, SystemCallFilter=@system-service,
  ProtectProc=invisible and (SEC-U6C-1's other half) an explicit LimitNOFILE fit it.

## 3. Verdict: APPROVE WITH FINDINGS
Five Lows, no Critical, High or Medium. The page host and the issue mode meet the brief's section 7 reads and Conditions
2-4 (section 5 below); npt-load's token and pin reads and the probe's settings and pin are bounded and panic-free.
Under the review budget the Lows go to STATUS as follow-ups and trigger no push on their own; while open they keep the
PR needs-mustafa (advisor.md, PR review step 4), unless the fix pass takes the one-line fixes of SEC-U6C-1, -3 and -4.
**Stop B line:** under Q-U6C-8 (2) (C4, lever (2)), this read of the page host (serve.rs) and the issue mode (issue.rs)
at snapshot a4e63455 stands as Stop B's read of "U6c's host" (u8-brief.md:466-469); Stop B's round reads U8's package and
wrappers, and re-reads the host only if a fix pass changes serve.rs or issue.rs (then only what changed).

## 4. Findings
| Id | Severity | What | Evidence (W) | Exploit sketch | Fix |
|---|---|---|---|---|---|
| SEC-U6C-1 | Low | Accept errors retried at once: EMFILE spins a worker | crates/host/src/serve.rs:253-255 | ~1,020 idle TLS connections from an allow-listed source exhaust the 1,024 soft fd limit; the loop then spins at 100% CPU on RS beside the rooms | On Err `tokio::time::sleep(100 ms)` before the next accept; LimitNOFILE in U8's unit |
| SEC-U6C-2 | Low | Each answer copies its body (the Wasm up to 32 MiB) | crates/host/src/serve.rs:166-169, :193 | Many `GET /npt_client_wasm_bg.wasm` that never read hold one copy each | Write the head, then the shared slice; no copy |
| SEC-U6C-3 | Low | `+` accepted in NN (std's unsigned parse) | crates/host/src/serve.rs:202; crates/bots/src/load.rs:265 | `/npt/cert-hash-+5.json` opens `room-+5.json` (404); no traversal | `n.bytes().all(\|b\| b.is_ascii_digit())` |
| SEC-U6C-4 | Low | The probe pins any datagram from LG's IPv4, before decode | crates/server/src/probe.rs:279-281 | A spoofed datagram from LG's IP with another port before LG's first pins the wrong port; the leg fails loudly | Pin only when `ProbeDatagram::decode(b, None)` is Some |
| SEC-U6C-5 | Low | Let's Encrypt's terms agreed in code; no doc tells Mustafa | crates/host/src/issue.rs:181-185 | None (process): he agrees to the Subscriber Agreement without being told | One line in R1.md's certificate block or npt-cert's printout |

Anti-cheat coverage (U6c's surfaces; no game input path changes in U6c):
| Action | Where the server validates it | Gap |
|---|---|---|
| Page fetch | serve.rs:186-210 (exact path, GET/HEAD, fixed files) | SEC-U6C-1, -2 (abuse, cut items' reach) |
| Hash or host-key fetch | serve.rs:174-182, :195-209 (capped, verbatim, no-store) | SEC-U6C-3 (nit) |
| Certificate issue | issue.rs (fixed word, URL, name, dir; timeouts; create_new) | SEC-U6C-5 (process) |
| Probe datagram from LG | server probe.rs:274-302 (IPv4 pin, decode, counted, bounded) | SEC-U6C-4 |
| npt-load's Join and commands | npt-server's roster and decoders (U6a, U6b; unchanged here) | none new |

## 5. Conditions 2-4 of security-u6c-deps.md, against the code
- **Condition 2: MET.** `Account::builder()` in the shipped path (issue.rs:177-178); `builder_with_http` only under
  `#[cfg(test)]` (:179-180; field :45-46; main's :307-308), no other call in the tree (grep); no `install_default` in any
  crate, and `set_default` only as ImpairSocket's L2 method (socket.rs:251, impair.rs:714), not a rustls or tracing
  default; SSL_CERT_FILE and SSL_CERT_DIR cleared by U8's npt-cert (`env -i`, u8-build.patch:3285; setpriv
  `--reset-env`, :3329), the binary's own refusal (SEC-U6CD-4's recommended half) not built; every ACME call under
  `tokio::time::timeout` (issue.rs:123-134 for account+directory :187-193, order :209, authorization :212-218,
  set_ready :231, poll_ready :234, finalize_csr :245, poll_certificate :246-252), the whole under 30 minutes (:283-294).
- **Condition 3: MET.** npt-host's ServerConfig `builder_with_provider(ring)` + `with_protocol_versions(&[&TLS13])`
  (serve.rs:123-127) with its TLS-1.2-only-client refusal test (serve_tests.rs:333-343); the other server configs
  unchanged and TLS 1.3 only (ws_config.rs:31-32, wt.rs:76-77, pin.rs:132-133 in tests); the pinning client config TLS
  1.3 only (pin.rs:86-87) with `verify_tls12_signature` returning Err (pin.rs:49-56), and LG's both clients built on it
  (wt_server.rs:605, ws_server.rs:943; load.rs:572-587).
- **Condition 4: MET.** crates/bots/Cargo.toml:27 `tokio.workspace = true` with no feature key; npt-server only under
  [dev-dependencies] (:31-32); both manifests and the lock equal 3b75d00's (N0); check.mjs's deps step PASS
  (end-check2.log).

## 6. What I did not verify
- I ran no test: the counted run (end-check2.log: 11 steps PASS, 859/956, redact 0 findings) and the builder's plant logs
  are my evidence; I read P12, P13, P14, P17, P20 and P21's targets in the tests but did not re-run the plants.
- The EMFILE spin (SEC-U6C-1) and the copy's memory (SEC-U6C-2) are read from the code and tokio/hyper behaviour, not
  reproduced; the Wasm's real size (U7's dist) not read.
- instant-acme's internals beyond `create`, `builder` and `poll_ready` (account.rs:52-58, :432-446; order.rs:166-181):
  the Retry-After cap inside RetryPolicy not read (our 360 s timeout bounds it anyway).
- U8's side (npt-cert, the systemd units, rs.txt) read only where a condition or exposure leans on it; that is Stop B's.
- No real ACME order, DNS, browser, RS or LG; staging's and production's API chains not checked.
- F-P8, the A2 command and the netcode and test seats' items are outside this seat.

DONE
