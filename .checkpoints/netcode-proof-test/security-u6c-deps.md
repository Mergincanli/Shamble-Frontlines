# security-auditor: U6c Block 0 dependency short read (DEP-HOLD)

Reader: security-auditor (Opus 5.5), 2026-11-22. Branch claude/netcode-proof-test-plan-hcky5g.
Inputs: u6c-brief.md rev 1.1 (sha256 8f56f37a...) section 5 and section 0.1 C5; advisor-u6c-brief.md section 4
(:283-316, the eight items). Earlier reads reused: security-r1-deps.md (instant-acme, SEC-R1D-4, binding check
:249-253), security-u6b-deps.md (provider, web-transport-quinn).
No cargo fetch/update/build/install, no npm audit; read-only `cargo tree --locked --offline` on today's lock only.
Sources: the lock, the manifests, ~/.cargo/registry, index.crates.io, static.crates.io (.crate read in memory), OSV.
Scratch (read-only scripts, none writes a file) in .checkpoints/netcode-proof-test/security-u6c/: index.mjs (sparse
index), crate-read.mjs (a .crate in memory, sha256 checked), osv.mjs (OSV querybatch), lockmodel.mjs (the lock model).
Abbreviations: IA = instant-acme 0.8.5 source as published (crate sha256 9f05ad37...1a64, read by crate-read.mjs);
REG = ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/.

## Plan
1. Verify the brief's hash; read section 5, section 0.1 C5, the advisor's section 4 items 1-8.
2. Read the current lock and manifests; the earlier reads' relevant parts.
3. Answer each item by number, with sources.
4. Model the expected lock diff (packages, versions, checksums), feature tree, duplicates.
5. Binding check commands for Block 0; conditions; verdict; DONE.

## Notes (one line per finding, saved as I go)
- Step 1 done. u6c-brief.md sha256 8f56f37abc5a6f1f77845fe894f51685da3dcc36b676a73b66edbca79ca6fb52 (= rev 1.1).
  Section 5 at :474-510; C5 at :102-107; advisor's items at advisor-u6c-brief.md:292-316.
- Lane state (sha256 now): Cargo.toml 5ebfbedf...d36a, Cargo.lock c55bab3c...6b4d, crates/net fa2b3c2a...f523,
  crates/server e694b595...a60d, deny.toml 90404ee6...9ae1 = U6a's Block 0 after-state (U6b's read's "before").
  crates/host/Cargo.toml 9f7a05e9...be74 (npt-log only, :16-18); crates/bots/Cargo.toml fa127e4d...64c9 (six npt-*
  deps, :20-26). wip/u6a-build.patch touches no Cargo.toml, Cargo.lock, deny.toml or clippy.toml. U6b's Block 0 has not
  run, so U6c's "before" is U6b's merged lock: my U6b model reproduced today by lockmodel.mjs, 235 packages, 2,254
  lines, sha256 210bf2df2d968dc457a76a0c75f20f7dd06fd2a0f9c33ef05e5c7c2a3b2ca219 (equal to security-u6b-deps.md:123).
- Sparse index (2026-11-22 ~20:30 UTC): all 17 still the newest stable of their lines, checksums equal
  security-r1-deps.md:258-273 one for one, none yanked, rust-versions <= 1.85 (workspace 1.99.1). bytes 1.12.2 exists
  (the pin keeps 1.12.1; OSV: no advisory on 1.12.1).
- OSV querybatch 2026-11-22T20:32:25Z over 35 packages (the 17, httparse, http, bytes, rcgen, rustls, tokio-rustls,
  rustls-platform-verifier, rustls-native-certs, ring, tokio, syn 3.0.9, base64 0.22.1, pem 3.0.6, socket2,
  serde_json, sha2, url 2.5.9, pin-utils): one hit, pin-utils 0.1.1 RUSTSEC-2026-0441 (known, accepted). Clean.
- Lock model done (lockmodel.mjs): parse/emit is an identity on the real lock and its ordering equals cargo's; every
  array name resolves to exactly one package. After U6c: 252 packages, 2,477 lines, sha256 ea6efb00...9111; against
  U6b's lock 226 lines added, 3 removed, 17 checksum lines added.
- Weak-feature quirk confirmed for pem: pem 3.0.6 has only `default` and `std` on today (`cargo tree -e features -i
  pem`) yet its lock array lists serde_core (Cargo.lock:1017-1020), from `std = ["base64/std", "serde_core?/std"]`;
  pem 4.0.0 has the same table, so its array is ["base64 0.23.1", "serde_core"].
- Findings so far: SEC-U6CD-1 (item 2 wording), SEC-U6CD-2 (workspace comment), SEC-U6CD-3 (implicit process
  default), SEC-U6CD-4 (root store and the environment). All Low.

## Item 1. Exactly the content: CLEAR WITH CONDITIONS
- (a) Cargo.toml [workspace.dependencies] gains exactly five entries (comments free): `instant-acme = { version =
  "=0.8.5", default-features = false, features = ["ring", "hyper-rustls"] }`, `httparse = "=1.10.1"`, `http =
  "=1.5.0"`, `bytes = "=1.12.1"`, `http-body-util = "=0.1.5"`. The defaults the last four take add no package:
  httparse [std], http [std], bytes [std], http-body-util `default = []` (index; its `channel` would add tokio's use,
  not asked). The exact pins on locked crates freeze them graph-wide (one semver-compatible version per graph): wanted.
  Plus item 6's comment-only edit of Cargo.toml:68-70 (SEC-U6CD-2).
- (b) crates/host/Cargo.toml [dependencies]: `instant-acme.workspace = true`, `rcgen = { workspace = true, features =
  ["pem"] }`, `tokio`, `tokio-rustls`, `httparse`, `serde`, `serde_json`, `sha2` each `.workspace = true`;
  [dev-dependencies]: `http`, `bytes`, `http-body-util` each `.workspace = true`. host's rcgen inherits U6b's entry,
  which must read exactly `rcgen = { version = "=0.14.10", default-features = false, features = ["ring"] }`
  (security-u6b-deps.md:50-53); if U6b merged anything else, stop. No direct rustls, rustls-pki-types, hyper, time or
  x509-parser entry is needed: tokio-rustls re-exports rustls (REG tokio-rustls-0.26.6/src/lib.rs:49) and PEM parsing
  is `rustls::pki_types::pem::PemObject` (REG rustls-pki-types-1.15.1/src/lib.rs:98-99, behind `alloc`, on), the route
  instant-acme itself uses (IA src/account.rs:18).
- (c) crates/bots/Cargo.toml [dependencies]: `tokio`, `serde`, `serde_json` each `.workspace = true`;
  [dev-dependencies]: `npt-server.workspace = true`. No cycle: npt-server depends on no npt-bots
  (crates/server/Cargo.toml:24-39).
- (d) crates/server and crates/net unchanged. Nowhere a `features`, `default-features`, `optional` or version key
  beyond these, no [patch], [profile], cargo-features or rustflags key, no hand edit of the lock.

## Item 2. The direct entries: CLEAR, one wording fix (SEC-U6CD-1)
- httparse 1.10.1 already parses client bytes in the room process: tungstenite 0.30.0's handshake lists it
  (Cargo.lock:1796-1804). U6c adds a use, not a parser. OSV none. The build's shape (Q-U6C-4): one 8 KiB buffer, `[
  httparse::EMPTY_HEADER; 32]`; `Err(TooManyHeaders)` or any error closes; `Partial` with the buffer full closes (never
  grow, never re-parse a full buffer in a loop); `Complete` then exact path match. Known residual, not reopened: no
  header timeout and no per-address or total caps on TCP 31000 (cut: replan-units.md:13-14, moved to the security
  baseline before the first online playtest), so per-connection memory is bounded but the connection count is not.
- http, bytes and http-body-util are dev-only as OUR entries (the fake directory). But all three, and http-body, are
  non-optional normal dependencies of instant-acme 0.8.5 (index: http ^1, bytes ^1, http-body ^1, http-body-util
  ^0.1.2), so they are in npt-host's release binary through instant-acme whatever our dev entries say. "Reach no shipped
  binary" holds for our fake-directory code only (SEC-U6CD-1, Low, wording; no security consequence).
- The fake directory reaches the binary only through `Account::builder_with_http` (IA src/account.rs:81-84, pub,
  always compiled). Build review: called only under `#[cfg(test)]` (SEC-R1D-4's "no custom plain-HTTP client").

## Item 3. rcgen's `pem` in crates/host: YES
- instant-acme declares rcgen `^0.14.2`, default-features off, features ["pem"], switched on by its `ring` feature's
  non-weak `rcgen/ring` (index features2: `ring = ["dep:ring", "hyper-rustls?/ring", "rcgen/ring"]`). So pem is on in
  every build that contains npt-host whatever crates/host writes; host's `features = ["pem"]` is redundant but documents
  our own call (`KeyPair::serialize_pem`, REG rcgen-0.14.10/src/key_pair.rs:441-447).
- In a workspace build (U8 builds the workspace: u8-brief.md:194-195) npt-server's rcgen is compiled with pem too. What
  that adds is only the `cfg(feature = "pem")` items: PEM encoders and decoders (key_pair.rs:423-447, certificate.rs:4,
  :16, :40, :1120, :1355, csr.rs:3-7, :52, crl.rs:1-8, :74, error.rs:37, :94). npt-server calls none; rcgen there runs
  at start on our own parameters, and no network byte reaches it. pem 4.0.0 has `#![deny(unsafe_code)]` (its
  src/lib.rs:101-107, crate sha256 d354a98a...f120); base64 0.23.1 is compiled with alloc and std only (pem turns its
  defaults off, so no `simd-unsafe`). Both OSV clean. Dead code, accepted; `cargo build -p npt-server` alone keeps it out
  if ever wanted.
- security-r1-deps.md:224's rcgen condition reads from here: default-features = false; `ring` (workspace entry);
  `pem` only through npt-host's line and instant-acme; never `aws_lc_rs`, `fips`, `x509-parser`.

## Item 4. npt-bots: CLEAR WITH CONDITIONS
- tokio from the workspace entry, no local key. U6c's new crates ask tokio only for time, sync, io-util, net and rt
  (index deps of instant-acme, hyper, hyper-util, hyper-rustls, h2, tokio-util), all on already; nothing asks fs,
  signal, process or io-std. npt-bots' own build today: `cargo tree -p npt-bots -e features -i tokio` lists bytes,
  default, io-util, libc, macros, mio, net, rt, rt-multi-thread, socket2, sync, time, tokio-macros; after U6b,
  web-transport-quinn asks io-util and net only (REG web-transport-quinn-0.13.6/Cargo.toml:127-133). So npt-load has no
  TERM handler: TERM's default action ends it at once, with no `DONE` (C1 (c)); stdin is /dev/null. Pre-existing note:
  tokio `fs` is on in `--all-features` builds through wtransport 0.7.2 (npt-net's optional wt-wtransport), never in
  npt-load's build.
- npt-server as a dev-dependency: Cargo compiles dev-dependencies for tests, examples and benchmarks only, never into
  a bin target, and resolver 2+ does not unify their features into a plain build. Checked by the binding check
  (`cargo tree -p npt-bots -e normal` names no npt-server).
- Build review: the token and pin readers under size caps (SEC-U6AD-3's reader side), the one reviewed clock allow.

## Item 5. The root store: the platform verifier (the drafter's lean): YES, with a condition
- What `Account::builder()` does: IA src/account.rs:54-59 -> `DefaultClient::try_new` (IA src/lib.rs:203-209) ->
  hyper-rustls `try_with_platform_verifier` (REG hyper-rustls-0.27.10/src/connector/builder.rs:89-98:
  `ClientConfig::builder()` plus rustls-platform-verifier), then `https_only().enable_http1().enable_http2()` (IA
  src/lib.rs:221-225). On Linux rustls-platform-verifier 0.7.1 is WebPKI over the roots of
  `rustls_native_certs::load_native_certs()` (REG rustls-platform-verifier-0.7.1/src/verification/others.rs:85-110),
  which takes SSL_CERT_FILE / SSL_CERT_DIR when set and openssl-probe's system locations otherwise (REG
  rustls-native-certs-0.8.5/src/lib.rs:118-125, :197-212; src/unix.rs:3-10). No roots -> an error, fail closed
  (others.rs:104-108).
- `Account::builder_with_root(pem_path)` (IA src/account.rs:65-79): the FIRST certificate of one PEM file as the only
  root, through `DefaultClient::with_roots` (IA src/lib.rs:211-219). Immune to the environment, but brittle: one
  pinned root fails the order when the API's chain moves to another root. Whether staging's and production's API
  endpoints chain to different roots I did not verify (this session's TLS goes through a proxy): medium confidence that
  both present publicly trusted chains, so the "apart" gain is small.
- Why the platform verifier is enough here: a party that intercepts the ACME API still cannot get a certificate for
  our name. Every request is a JWS signed by our in-memory account key, and the DNS-01 TXT value is SHA-256 of the key
  authorization, which embeds our account key's thumbprint (RFC 8555 section 8.1, key authorizations; section 8.4,
  DNS challenge). Its worst is a failed order or a chain browsers then reject (visible at the four-browser smoke). The
  trust store guards the run's availability and the result's honesty, not the name.
- Condition: npt-host issue runs with SSL_CERT_FILE and SSL_CERT_DIR unset (U8's npt-cert clears the environment:
  setpriv `--reset-env`, as advisor-u6c-brief.md:305-307 cites). Recommended, not required: the issue mode refuses to
  start (exit 2) when either is set, so the binding sits in the binary (SEC-U6CD-4, Low).
- SEC-U6CD-3 (Low, note): both routes build with `rustls::ClientConfig::builder()` (IA src/lib.rs:213-216;
  hyper-rustls builder.rs:92-96), which installs the one compiled provider (ring, after U6b) as npt-host issue's
  process default. Not our code, harmless with one provider; with two compiled it would panic at `Account::builder()`,
  before any order (fail closed), and the binding check keeps aws-lc-* out. With tls12 now on, the ACME client offers
  TLS 1.3 and 1.2 (rustls's safe defaults): fine for a client.
- webpki-roots is in no lock (grep). Precision: webpki-root-certs is locked (Cargo.lock:1338, :1977) as
  rustls-platform-verifier's wasm32-only dependency (others.rs:113-117), never compiled on Linux.

## Item 6. The feature tree: CLEAR WITH CONDITIONS
- hyper: client, http1, http2; no server (instant-acme asks [client, http1, http2]; hyper-util's client/http1/http2
  map to the same; its server features are off; hyper-rustls takes hyper with no feature). hyper-util: client,
  client-legacy, http1, http2, tokio (no server*, client-proxy*, client-pool). hyper-rustls: http1, http2,
  native-tokio, ring, rustls-platform-verifier, tls12 (no default, aws-lc-rs, fips, logging, webpki-tokio).
- instant-acme: hyper-rustls, ring, and rcgen (cargo also turns on the feature named like an optional dependency when
  a non-weak `rcgen/ring` names it and such a feature exists; medium-high confidence, the tree shows it, and C5's item
  (3) expects it); never default, aws-lc-rs, fips, time, x509-parser. `finalize()` (IA src/order.rs:64-65,
  `cfg(feature = "rcgen")`) is then compiled and never called; we call `finalize_csr` (order.rs:90).
- Changes in shared crates, in any build containing npt-host: rcgen + pem; rustls + tls12 (log, logging, ring, std,
  tls12); tokio-rustls + tls12 (ring, tls12); ring + std (only `std::error::Error` impls: REG
  ring-0.17.14/src/error/unspecified.rs:15, :84, key_rejected.rs:17, :104). npt-server built alone keeps rustls at
  log, logging, ring, std and rcgen at crypto, ring. tokio and socket2 unchanged (socket2's `all` is already on).
  New: base64 0.23.1 [alloc, std]; pem 4.0.0 [default, std], no serde.
- tls12 and our servers: every server configuration names TLS 1.3 only: ws_config.rs:31-32, wt.rs:77-78 (QUIC is
  TLS 1.3 by protocol). ws_config_tests.rs:171 (`a_tls12_only_client_is_refused`) now refuses a TLS 1.2 that is
  compiled in, so it starts to mean something.
- Conditions for U6c's build: npt-host's ServerConfig by `builder_with_provider(ring)` and `with_protocol_versions(&[
  &TLS13])`, with its own TLS-1.2-only-client refusal test; every client configuration with a pinning verifier (U6b's
  WtClient and WsClient, LG's clients) names TLS 1.3 only and its `verify_tls12_signature` returns Err, as
  tls_test.rs:113-120 and :150-151 do. Why: with tls12 compiled, a stub returning Ok under the default versions would
  let an interceptor holding our public leaf finish a TLS 1.2 handshake and read the Join token.
- SEC-U6CD-2 (Low): Cargo.toml:69's "Default features off (no aws-lc-rs, no TLS 1.2, no logging)" becomes false for
  TLS 1.2 in every build with npt-host. Block 0 rewords it (comment only, no lock line), e.g. "Default features off
  (no aws-lc-rs, no logging); this entry asks for no TLS 1.2, but instant-acme's hyper-rustls turns rustls's tls12 on
  in any build with npt-host (U6c), so every server configuration names TLS 1.3 only".

## Item 7. The lock diff: CLEAR WITH CONDITIONS (exactly as below)
- Against U6b's merged lock: 17 [[package]] blocks added (235 -> 252), crates.io source, checksums of
  security-r1-deps.md:258-273: async-trait 0.1.92, atomic-waker 1.1.2, base64 0.23.1, fnv 1.0.7, h2 0.4.20, http-body
  1.1.0, http-body-util 0.1.5, httpdate 1.0.3, hyper 1.12.0, hyper-rustls 0.27.10, hyper-util 0.1.21, instant-acme
  0.8.5, pem 4.0.0, tokio-util 0.7.20, tower-service 0.3.3, try-lock 0.2.5, want 0.3.2. No existing version or checksum
  line changes (every requirement of the 17 is met by a locked version: bytes ^1.7.1, tokio ^1.51, http ^1.1, indexmap
  ^2, slab ^0.4.4, tracing ^0.1.36, futures-* ^0.3.31, libc ^0.2.168, socket2 ^0.6, syn ^3, base64 ^0.22 ...).
- Arrays re-qualified (3 lines out, 3 in): ` "base64",` -> ` "base64 0.22.1",` in pem 3.0.6 and sfv 0.15.0; ` "pem",`
  -> ` "pem 3.0.6",` in wtransport 0.7.2. rcgen's array gains ` "pem 4.0.0",` (before "ring"). npt-host's array:
  bytes, http, http-body-util, httparse, instant-acme, npt-log, rcgen, serde, serde_json, sha2, tokio, tokio-rustls
  (+11). npt-bots' array: npt-client, npt-link, npt-log, npt-net, npt-proto, npt-server, npt-sim, serde, serde_json,
  tokio (+4).
- New arrays (model): instant-acme names "base64 0.22.1" (it asks ^0.22) and 17 others; pem 4.0.0 names "base64
  0.23.1" and serde_core; async-trait names "syn 3.0.9". Totals: 226 lines added, 3 removed; 2,254 -> 2,477 lines;
  modelled sha256 ea6efb003a59f06946c5ba48f3ba73ec1c92d79757e07f6a7ab5b79610a79111 (binds only if the before-lock is
  210bf2df...a219).
- Duplicates: base64 (0.22.1, 0.23.1) and pem (3.0.6, 4.0.0) join getrandom, syn, synstructure, windows-link and
  windows-sys (`cargo tree --workspace --all-features --target all -d --depth 0` today): seven warnings under
  `multiple-versions = "warn"` (deny.toml), no error.

## Item 8. The tools: YES again
- `cargo deny --version` prints cargo-deny 0.22.0; `cargo audit --version` prints cargo-audit-audit 0.23.0 (both run
  now; binaries in ~/.cargo/bin dated 2026-11-22 14:07 and 14:09, U6a's Block 0 install). Use them as installed; a
  reinstall only as security-u6a-deps.md section 6 (ii) words it. Databases in their home locations, never in the
  repository.

## Findings
- **SEC-U6CD-1 (Low):** item 2 and section 5 say http, bytes and http-body-util "reach no shipped binary"; they ship
  in npt-host through instant-acme (its non-optional deps). Exploit sketch: none (instant-acme's own HTTP types); a
  reviewer misled into thinking the HTTP stack is test-only. Evidence: index deps of instant-acme 0.8.5. Fix: word it
  "our direct entries are dev-only; the crates ship in npt-host through instant-acme".
- **SEC-U6CD-2 (Low):** Cargo.toml:69's "no TLS 1.2" turns false with hyper-rustls's tls12. Exploit sketch: a later
  edit trusts the comment, builds a server config with default versions and serves TLS 1.2. Evidence: hyper-rustls
  0.27.10 features `tls12 = ["tokio-rustls/tls12", "rustls/tls12"]`, asked by instant-acme. Fix: reword in Block 0
  (item 6).
- **SEC-U6CD-3 (Low, note):** instant-acme's both client routes install ring as npt-host issue's process default via
  `ClientConfig::builder()`. Exploit sketch: none today; if a second provider were ever compiled, `Account::builder()`
  panics (fail closed). Evidence: IA src/lib.rs:203-219; hyper-rustls builder.rs:89-98. Fix: none beyond keeping
  aws-lc-* out (binding check) and no `install_default` in our code.
- **SEC-U6CD-4 (Low):** the platform verifier's roots follow SSL_CERT_FILE / SSL_CERT_DIR. Exploit sketch: someone
  who sets the issue process's environment points it at their root and intercepts the ACME API: a failed order or a
  bogus chain, never a certificate for our name (RFC 8555 sections 8.1, 8.4). Evidence: rustls-native-certs 0.8.5
  lib.rs:118-125. Fix: U8's environment reset (condition); recommended: the issue mode exits 2 when either is set.

## Conditions
1. Block 0's content exactly as item 1, plus SEC-U6CD-2's comment edit; the binding check below passes as written;
   anything else (another package, a version or checksum line, a feature not listed, another manifest key) comes back
   to me before the save.
2. The issue mode uses `Account::builder()` (the platform verifier); `Account::builder_with_http` only under
   `#[cfg(test)]`; no `install_default` / `set_default` in our code; npt-host issue runs with SSL_CERT_FILE and
   SSL_CERT_DIR unset (U8's npt-cert); each ACME call under `tokio::time::timeout` (SEC-R1D-4).
3. TLS 1.3 only on every server configuration (npt-host's included, with its TLS-1.2-only-client test) and on every
   pinning client configuration, each pinning verifier's `verify_tls12_signature` returning Err (item 6).
4. npt-load: no tokio feature added; npt-server reaches npt-bots by the dev edge only (binding check step 4).
5. cargo-deny 0.22.0 and cargo-audit 0.23.0 as installed, with both database dates recorded.

## Binding check for U6c's Block 0 save (the main session, in spikes/netcode-proof, after U6b's merge)
0. Before: `sha256sum Cargo.lock` equals U6b's merged lock as u6b-block0.md records it (my model 210bf2df2d968dc4
   57a76a0c75f20f7dd06fd2a0f9c33ef05e5c7c2a3b2ca219); `grep -c '^\[\[package\]\]' Cargo.lock` = 235; `grep -c -E
   'aws-lc|jobserver' Cargo.lock` = 0; `grep -n '^rcgen = ' Cargo.toml` prints U6b's entry (item 1 (b));
   `sha256sum crates/host/Cargo.toml crates/bots/Cargo.toml deny.toml` = 9f7a05e9dda8ba2348eb6e9849f8a46c224e4ec9
   e20a50a917ef715ab555be74, fa127e4d0befb08d282e0995d36195bff672e82a9478f856427d1dacc6c264c9, 90404ee6b66b3884129195
   ad1d6eaef8a97a023159d4ff2b8f888242d7369ae1. If the lock hash differs but the package count and greps hold, the list
   in step 3 still binds and only the after-hash is void; anything else: stop, back to me.
1. The manifest edits exactly as item 1 (and the comment). `git diff --stat` touches only Cargo.toml,
   crates/host/Cargo.toml, crates/bots/Cargo.toml (Cargo.lock after step 2).
2. The lock refreshed with the network on, the main session only: `cargo metadata --format-version 1 >/dev/null`
   (no `--locked`, no `--offline`), then `cargo fetch --locked`. Never `cargo update` or `generate-lockfile`, except
   `cargo update -p <name> --precise <cleared version>` to pin back a newer patch of one of the 17, then step 3 again.
3. The lock diff:
   - `git diff --numstat -- Cargo.lock` prints `226	3`; `grep -c '^\[\[package\]\]' Cargo.lock` = 252;
   - `git diff -U0 -- Cargo.lock | grep -c '^+checksum'` = 17, each equal to security-r1-deps.md:258-273 for the 17
     names and versions of item 7; `git diff -U0 -- Cargo.lock | grep -c '^+version'` = 17;
   - `git diff -U0 -- Cargo.lock | grep '^-[^-]'` prints exactly three lines: ` "base64",` twice and ` "pem",` once;
   - the arrays of item 7 (pem 3.0.6, sfv, wtransport, rcgen, npt-host, npt-bots) read as listed there;
   - `grep -c -E 'aws-lc|jobserver|cmake|webpki-roots' Cargo.lock` = 0;
   - `sha256sum Cargo.lock` = ea6efb003a59f06946c5ba48f3ba73ec1c92d79757e07f6a7ab5b79610a79111 (my model; if only
     this differs while every other line holds, send me the diff).
4. The feature tree (`--locked --offline` on every call):
   - `cargo tree --workspace --all-features -i aws-lc-rs` fails with "did not match any packages";
   - `cargo tree -p npt-host -e features -i instant-acme --depth 1`: features hyper-rustls, rcgen, ring only;
   - `cargo tree -p npt-host -e features -i hyper --depth 1`: client, http1, http2 only (no server);
     `-i hyper-util`: client, client-legacy, http1, http2, tokio only; `-i hyper-rustls`: http1, http2,
     native-tokio, ring, rustls-platform-verifier, tls12 only;
   - `cargo tree --workspace --all-features -e features -i rcgen --depth 1`: crypto, pem, ring only;
     `cargo tree -p npt-server -e features -i rcgen --depth 1`: crypto, ring only;
   - `cargo tree --workspace --all-features -e features -i rustls --depth 1`: log, logging, ring, std, tls12 only;
     `cargo tree -p npt-server -e features -i rustls --depth 1`: log, logging, ring, std only;
     `cargo tree --workspace --all-features -e features -i tokio-rustls --depth 1`: ring, tls12 only;
   - `cargo tree -p npt-bots -e features -i tokio --depth 1 --prefix none` equals today's list (item 4): no fs,
     signal, process, io-std;
   - `cargo tree -p npt-host -e normal --depth 1` lists instant-acme, httparse, npt-log, rcgen, serde, serde_json,
     sha2, tokio, tokio-rustls; `-e dev --depth 1` lists bytes, http, http-body-util;
   - `cargo tree -p npt-bots -e normal --prefix none` has no line naming npt-server; `cargo tree -p npt-bots -e dev
     --depth 1` lists npt-server;
   - `cargo tree --workspace --all-features --target all -d --depth 0` names exactly base64, getrandom, pem, syn,
     synstructure, windows-link, windows-sys.
5. `cargo check --workspace --all-targets --locked`, 0 warnings; check.mjs's test and deps steps PASS with counts equal
   before and after Block 0.
6. `cargo deny --locked check advisories bans sources`: exit 0, "advisories ok, bans ok, sources ok", warnings only the
   seven multiple-versions names of step 4; `cargo audit --file Cargo.lock`: 252 crate dependencies, 0
   vulnerabilities, exactly one allowed warning RUSTSEC-2026-0441 (pin-utils 0.1.1). Database dates: `git -C
   ~/.cargo/advisory-db log -1 --format=%cI` and the same on cargo-deny's folder under ~/.cargo/advisory-dbs/.
7. dependency-check.md: a new section in U6a's shape naming this file and its conditions, then `sha256  path` for
   Cargo.toml, Cargo.lock, crates/host/Cargo.toml, crates/bots/Cargo.toml (deny.toml unchanged). Then the save.

## Verdict: CLEAR WITH CONDITIONS
Items 1-8 answered: 1 CLEAR WITH CONDITIONS; 2 CLEAR (SEC-U6CD-1 wording); 3 YES; 4 CLEAR WITH CONDITIONS; 5 YES to
the platform verifier, with the environment condition; 6 CLEAR WITH CONDITIONS; 7 CLEAR WITH CONDITIONS (17 packages,
226/3 lines); 8 YES. Findings: four Low (SEC-U6CD-1 to -4), no Critical, High or Medium. Not verified: the real lock
refresh (the model is the expectation; the refresh decides), cargo's activation of instant-acme's `rcgen` feature (the
tree shows it), and the chains Let's Encrypt's API endpoints present.

DONE
