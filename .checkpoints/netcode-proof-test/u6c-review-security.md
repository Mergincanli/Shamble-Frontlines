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
  3a2142df): no manifest or lock change.
- N1 (cleared): serve.rs allow-list is the seven U7 names, compiled in (serve.rs:34-46), read once at start under 32 MiB
  each, a missing one exit 2 (:132-139, :277-280); path up to the first `?` matched by `==` (:191-192), the query never
  read; per-room hash `/npt/cert-hash-NN.json` NN 00-31 under 1 KiB, `no-store`, absent 404, oversized 500 (:174-182,
  :199-209); no plain `/npt/cert-hash.json`; host keys under 4 KiB with `no-store` (:195-198); GET/HEAD else 405
  (:187-190); CSP word for word, nosniff, no-referrer on every answer (:150-171); one 8 KiB buffer, 32 header slots, a
  request that does not fit closed (:215-238); TLS 1.3 only by builder_with_provider(ring) + with_protocol_versions(TLS13),
  ALPN http/1.1 (:123-131); no per-request log (stderr only at start :292 and fatal :268). The TLS-1.2-only client test
  (serve_tests.rs:333-343) compiles only with rustls's tls12 on, so it refuses a TLS 1.2 that exists.
- N2 SEC-U6C-1 (Low): serve.rs:253-255 `let Ok(..) = listener.accept().await else { continue; }` retries at once. Under
  fd exhaustion (EMFILE: U8's npt-host.service sets no LimitNOFILE, so systemd's soft 1,024; no connection cap and no
  handshake or read deadline, by the cut) accept fails at once, again and again, and the loop spins one of the two workers
  at 100% on RS beside the measured rooms. Fix: on Err sleep 100 ms before the next accept (hyper's sleep_on_errors).
- N3 SEC-U6C-2 (Low): serve.rs:193 with :150-170 copies each body into a fresh Vec per answer (the Wasm, up to FILE_CAP
  32 MiB); with no connection cap and no write deadline (cut), a client on an allow-listed source that requests the Wasm
  on many connections and never reads holds one copy each (connections x Wasm size, up to the fd limit). Fix: write the
  head, then the shared body slice from `self.files`, with no copy.
- N4 (cleared, considered): per-request opens follow symlinks and would block on a FIFO (serve.rs:78-85), but only
  npt-room writes /var/lib/npt/hash and npt-room already reads production.key.pem by design (2750 npt-host:npt-room, key
  0640), so a planted link to the key gives npt-room nothing it lacks. No finding.
- N5 SEC-U6C-3 (Low, nit): serve.rs:202 `n.parse::<u8>()` accepts a leading `+` (std's FromStr for unsigned), so
  `/npt/cert-hash-+5.json` passes the two-character filter and opens `room-+5.json` (absent: 404). No traversal (two
  characters, no `/` or `.`), but the per-room set is 42 names, not 32. Fix: `n.bytes().all(|b| b.is_ascii_digit())`.
  The same `+` passes load.rs:265's room check (operator settings only).
