# security-auditor: U6b Block 0 dependency read (DEP-HOLD), 2026-11-22

Subject: u6b-brief.md rev 1.1, section 0.1 condition 2, Block 0 (section 5); the eight items of
advisor-u6b-brief.md section 4 (:183-216); plus the ring-for-aws-lc-rs ruling. Read only: no install, no build;
read-only `cargo tree --locked --offline` / `cargo metadata --locked --offline` on the lane's current lock; the
after-state modelled from the manifests and the published crates in the local registry cache (a scratch model,
scratchpad simlock.py, reads the lock and writes nothing). (No Edit tool in this agent: the file is rewritten whole
with Write after each block.)

## Plan
1. Baseline: lane head, hashes of the manifests and the lock, the current aws-lc-rs edges in the lock.
2. Item 1: the exact manifest content (workspace, npt-net, npt-server), U6-Q13 term (1).
3. Item 2: the four provider lines (wt.rs, wt_tests.rs, ws_config.rs) and web-transport-quinn's provider resolution.
4. Item 3: the exact lock diff (what reaches aws-lc-rs and its build-only crates; what else changes).
5. Item 4: the feature tree after (rustls, quinn-proto, rustls-webpki, web-transport-quinn, rcgen, time).
6. Item 5: what `default = ["wt-wtq"]` puts in the room binary (url 2.5.9, Url::origin()).
7. Item 6: bites gate.
8. Item 7: ring's key-exchange groups; the ruling on ring vs aws-lc-rs (anything weaker; rustls features).
9. Item 8: cargo-deny 0.22.0 / cargo-audit 0.23.0 yes; what "clean" means here.
10. Verdicts per item; binding check; DONE.

## Notes (saved as I go)

### 1. Baseline (DONE)
Lane head 86f0237 (2026-11-22 14:56 UTC); spikes/netcode-proof's manifests and lock last changed at 3e80e05 (U6a's
Block 0). sha256 before (this read): Cargo.toml 5ebfbedf...0d36a; Cargo.lock c55bab3c...6b4d (= u6a-block0.md's
After); crates/net/Cargo.toml fa2b3c2a...f523; crates/server/Cargo.toml e694b595...a60d; deny.toml 90404ee6...69ae1.
Lock: 242 [[package]] entries (225 registry + 17 members), 2,318 lines. U6b's Block 0 runs at `<base>` after U6a
merges; if any of these hashes differ there, the expected diff below no longer binds (binding check step 0).
- Who turns aws-lc-rs on today (`cargo metadata --locked --offline --all-features`, `cargo tree -e features -i
  aws-lc-rs`): ONE route only: npt-net's `web-transport-quinn = { workspace = true, optional = true }` takes
  web-transport-quinn 0.13.6's `default = ["aws-lc-rs"]` (its Cargo.toml [features]: `aws-lc-rs =
  ["quinn/rustls-aws-lc-rs", "rustls/aws-lc-rs"]`), which turns on quinn's aws-lc-rs / rustls-aws-lc-rs ->
  quinn-proto's aws-lc-rs / rustls-aws-lc-rs -> rustls aws_lc_rs -> rustls-webpki aws-lc-rs. Resolved today (all
  features): rustls [aws-lc-rs, aws_lc_rs, log, logging, ring, std]; quinn-proto [aws-lc-rs, bloom, log,
  platform-verifier, ring, rustls-aws-lc-rs, rustls-ring]; rustls-webpki [alloc, aws-lc-rs, ring, std];
  web-transport-quinn [aws-lc-rs, default]; wtransport [default, quinn, ring, self-signed]; rcgen [crypto, ring];
  tokio-rustls [ring]; cc [parallel]. quinn 0.11.12's own default is rustls-ring.
- Every other edge that could reach it is off, checked in each locked crate whose manifest mentions aws-lc (grep over
  the registry cache for all 225 locked crates: 14 hits): every dependent of rustls takes it with default-features =
  false (quinn, quinn-proto, rustls-platform-verifier, tokio-rustls, tokio-tungstenite, tungstenite,
  web-transport-quinn, wtransport); tokio-rustls's own default (aws_lc_rs) is off (npt-net and tokio-tungstenite take
  it default-features = false); wtransport's `aws-lc-rs`/`fips` off (its default is self-signed + ring); rcgen's
  `aws_lc_rs`/`fips` off; x509-parser's `verify-aws` off; rustls-platform-verifier's `ffi-testing` off; the rest are
  dev-dependencies of dependencies (never resolved). So removing web-transport-quinn's default removes aws-lc-rs.

### 2. Item 1, the exact manifest content (DONE)
- The workspace entry must carry `default-features = false`: with the entry's defaults on, a member's
  `default-features = false` is not honoured (the workspace is edition 2024). The only member that inherits
  web-transport-quinn is npt-net. Exact lines, Cargo.toml [workspace.dependencies]: `web-transport-quinn = { version =
  "=0.13.6", default-features = false }` (replacing `web-transport-quinn = "=0.13.6"`), and a new `rcgen = { version =
  "=0.14.10", default-features = false, features = ["ring"] }` with its comment (no other key; never `pem`,
  `aws_lc_rs`, `fips`, `x509-parser`, `zeroize`). rcgen's `ring` = ["crypto", "dep:ring", "x509-parser?/verify"]:
  ring is already compiled, crypto is a marker; no package added (rcgen 0.14.10, yasna, time, rustls-pki-types
  already locked through wtransport's self-signed).
- crates/net/Cargo.toml: [features] gains exactly `default = ["wt-wtq"]` and `bites = []` (wt-wtransport and wt-wtq
  unchanged); `web-transport-quinn = { workspace = true, optional = true, features = ["ring"] }`; `futures-util.
  workspace = true` moved from [dev-dependencies] to [dependencies] (the [dev-dependencies] table then empty: removed
  or left empty; neither changes a lock line). wtransport's line unchanged.
- crates/server/Cargo.toml: `rcgen.workspace = true` in [dependencies] (no features/default-features/optional key);
  `bites = ["npt-sim/bites", "npt-proto/bites", "npt-room/bites", "npt-net/bites"]`.
- U6-Q13 term (1): today npt-server and npt-bots take `npt-net.workspace = true`, no features key; the workspace's
  `npt-net = { path = "crates/net" }` has no default-features key. Both must stay so (no dependent names a `wt-*`
  feature or turns npt-net's defaults off). npt-lab reaches npt-net only through npt-server and npt-bots. No member
  has a `default =` line today (grep), so npt-net's becomes the only one.
- Nothing else: no other workspace entry, no `time` or `tracing-subscriber` entry (grep: none today), no [profile]
  key, no `[patch]`, no `cargo-features`, no hand edit of the lock.
- Verdict: CLEAR WITH CONDITIONS (exactly these lines; binding check step 1).

### 3. Item 2, the four provider lines (DONE)
Read: wt.rs:1-20, :60-90, :104-110, :170-200; wt_tests.rs:465-540, :572-595; ws_config.rs:1-36; delay_tests.rs:
2756-2804; web-transport-quinn 0.13.6's src/crypto.rs (whole), client.rs:1-300, server.rs:1-60 and :241-262; rustls
0.23.46's crypto/mod.rs:243-275; quinn-proto 0.11.19's crypto/ring_like.rs:1, crypto/rustls.rs:3, config/mod.rs:180.
- The lines that stop compiling without rustls's aws_lc_rs module: wt.rs:177-179 (`rustls::crypto::aws_lc_rs::
  default_provider()` in `wtq_server::provider`, doc "aws-lc-rs (its default feature)") -> `rustls::crypto::ring::
  default_provider()`, doc naming ring and Block 0; wt_tests.rs:586-592 (the `#[cfg(feature = "wt-wtq")]` aws-lc-rs
  assertion) -> removed, or better replaced by an assertion that `wtq_server::provider()`'s kx groups and suites equal
  ring's named directly (delay_tests.rs:2773-2775's form); ws_config.rs:4's comment ("beside wt-wtq's aws-lc-rs") ->
  ring for every configuration; wt.rs's module text :4-6 ("web-transport-quinn: aws-lc-rs, its default") -> ring for
  both crates (condition 2's added line). The workspace comment Cargo.toml:69 ("no aws-lc-rs") stays true.
- Provider choice stays per configuration: wt.rs:77 and ws_config.rs:31 build with `builder_with_provider`; the test
  client (tls_test.rs:150) too. Grep of crates/: no `install_default`, no `set_default`, no rustls
  `ServerConfig::builder()`/`ClientConfig::builder()` (only wtransport's own config builders, which take our TLS).
- web-transport-quinn's `crypto::default_provider()` (crypto.rs:24-44) returns the process default if one is
  installed, else aws-lc-rs if only that feature, ring if only ring, and PANICS when both are on with no default. It is
  used by `ClientBuilder::new` and `ServerBuilder::new` (client.rs:72, server.rs:44); `Client::new(endpoint, config)`
  (client.rs:224, the brief's WtClient route) takes our own `quinn::ClientConfig` and never calls it. After Block 0
  only `ring` is on, so any call resolves to ring; with no aws-lc-rs compiled at all, a process default, if anything
  ever set one, could only be ring. rustls's `get_default_or_install_from_crate_features` (crypto/mod.rs:243-257)
  likewise stops panicking (two providers today) and would install ring. So the switch removes a reachable panic class
  (a library builder called while two providers are compiled) rather than adding one.
- Note for the tests (not security): rustls's implicit install means a test calling a non-provider builder would set a
  process default and break the `get_default().is_none()` assertions (wt_tests.rs:475-537, delay_tests.rs:2761,
  :2804) in the same test binary; the grep finds none today. WtClient keeps `Client::new` with our config, never
  `ClientBuilder` or `Client::default()` (client.rs:275-277 `unwrap`s `with_system_roots`; client.rs:155, :161, :165
  `unwrap` in the builder, :165 binding `[::]:0`).
- Verdict: CLEAR WITH CONDITIONS: the four sites as above; no `install_default`/`set_default` anywhere; no
  web-transport-quinn `ClientBuilder`, `ServerBuilder` or `Client::default()` in npt-net or npt-server (a grep in the
  binding check; the build review reads it).

### 4. Item 3, the exact lock diff (DONE): one more package goes than the brief names (SEC-U6BD-1)
Read: the lock's entries for aws-lc-rs (:60-67), aws-lc-sys (:70-80), cmake (:160-166), dunce (:267-270), fs_extra
(:312-315), pkg-config (:1041-1044), cc, jobserver, quinn-proto (:1106-1125), rustls (:1284-1297), rustls-webpki
(:1349-1357), npt-server, npt-net, rcgen, generator; cc 1.11.1's manifest; `cargo tree -i` for zeroize, cc, generator;
`cargo metadata` for every edge into cc.
- Reverse edges in the lock: "aws-lc-rs" only in quinn-proto, rustls, rustls-webpki; "aws-lc-sys" only in aws-lc-rs;
  "cmake", "dunce", "fs_extra", "pkg-config" only in aws-lc-sys. zeroize stays (rustls, rustls-pki-types); cc stays
  (ring's build-dependency, and generator's, in the lock through loom's cfg(loom) edge).
- **Missed in the brief:** cc 1.11.1's `parallel = ["dep:jobserver", "dep:libc"]` (its Cargo.toml [features]) is
  requested by aws-lc-sys alone (`aws-lc-sys -> cc feats=['parallel']`; ring, cmake and generator take cc with no
  feature). With aws-lc-sys gone, `parallel` goes off, so the lock drops cc's optional edges: ` "jobserver",` and
  ` "libc",` leave cc's array, and jobserver 0.1.35, whose only dependent is cc (`grep -c '"jobserver"'` = 2: its own
  name line and cc's array), leaves the lock. Its own deps (getrandom 0.4.3, libc) stay (quinn-proto and many).
  libc stays in the lock. This is a build-only crate only aws-lc-sys reached (section 5's general wording covers it;
  condition 2's and the advisor's named list do not, and say "nothing else").
- So the after-lock, modelled from the before-lock (blocks removed, array lines removed, one line added):
  - seven [[package]] blocks removed: aws-lc-rs 1.19.0, aws-lc-sys 0.47.0, cmake 0.1.58, dunce 1.0.5, fs_extra 1.3.0,
    jobserver 0.1.35, pkg-config 0.3.34: 242 -> 235 entries;
  - five array lines removed: ` "aws-lc-rs",` from quinn-proto 0.11.19, rustls 0.23.46 and rustls-webpki 0.103.16;
    ` "jobserver",` and ` "libc",` from cc 1.11.1;
  - one array line added: ` "rcgen",` in npt-server's array, between ` "npt-sim",` and ` "serde",`;
  - 65 lines removed, 1 added (2,318 -> 2,254 lines); no version, checksum or source line added or changed;
  - modelled sha256 of the after-lock: 210bf2df2d968dc457a76a0c75f20f7dd06fd2a0f9c33ef05e5c7c2a3b2ca219 (from
    c55bab3c...6b4d; a mismatch is a stop, not a failure of Block 0: the diff then decides, and comes back to me).
- No change from futures-util's move (npt-net's array already lists "futures-util": the lock records dev-deps) nor
  from the features (`default`, `bites`, `ring` are not lock data). npt-net's array keeps "web-transport-quinn".
- Lock-quirk check: cargo puts an optional dependency named only by a weak `dep?/feat` into the lock (rcgen's array
  lists x509-parser today for that reason, though rcgen's `x509-parser` feature is off). The weak features naming
  aws-lc-rs (quinn-proto's `aws-lc-rs?/...`, rustls's `aws-lc-rs?/fips`) sit inside features that are off after
  Block 0, so the quirk keeps nothing.
- After-state OSV sweep (api.osv.dev /v1/querybatch, 2026-11-22 15:05 UTC, 219 registry packages: the 218 left plus
  jobserver): one hit, pin-utils 0.1.1 RUSTSEC-2026-0441 (unmaintained, accepted: security-u6a-deps.md section 6).
  Control query: url's list returns RUSTSEC-2026-0592 (fixed in our 2.5.9). Nothing new since U6a's audit (~14:2x
  UTC).
- Verdict: CLEAR WITH CONDITIONS: exactly this diff (binding check step 3), not the six-package list; anything else
  back to me.

### 5. Item 4, the feature tree after (DONE, modelled)
From the manifests above (cargo cannot be run on the after-state here):
- web-transport-quinn [ring] only (no `default`, no `aws-lc-rs`; its `ring` = quinn/rustls-ring + rustls/ring).
- quinn [bloom, default, log, platform-verifier, ring, runtime-tokio, rustls, rustls-ring]; quinn-proto [bloom, log,
  platform-verifier, ring, rustls-ring]; rustls [log, logging, ring, std]; rustls-webpki [alloc, ring, std];
  tokio-rustls [ring]; no aws-lc-rs package in the lock, so `cargo tree -i aws-lc-rs` must answer "did not match any
  packages".
- rcgen [crypto, ring] in npt-server's default build (its direct edge) and in `--all-features` builds (wtransport's
  `self-signed` adds only `dep:rcgen`, no feature; wtransport's `ring` adds `rcgen?/ring`, the same set). Never
  `aws_lc_rs`, `fips`, `pem`, `x509-parser` from npt-server (rcgen's `pem` would pull pem 4.0.0, not locked).
- time: no workspace manifest names it (grep); it comes through rcgen (default-features = false) and yasna
  (optional `time`, turned on by rcgen); in npt-server's default build with no time feature of its own beyond what
  those ask. Condition 6's `date_time_ymd` route needs no direct `time` (advisor-u6b-brief.md section 5).
- cc loses `parallel` (build-time only).
- Verdict: CLEAR WITH CONDITIONS: binding check step 4 shows exactly this.

### 6. Item 5, what `default = ["wt-wtq"]` puts in the room binary (DONE)
- Measured today: `cargo tree -p npt-server -e normal,build` has 88 packages by default; with `--features npt-net/wt-wtq`
  (today's aws-lc-rs default) 142. Of the 54 added, after Block 0 the aws-lc six and jobserver drop out, leaving 47:
  web-transport-quinn 0.13.6, web-transport-proto 0.6.4, web-transport-trait 0.5.2, kio 0.6.4, url 2.5.9 and its
  IDNA/ICU set (idna 1.1.0, idna_adapter, icu_* 2.4.0, form_urlencoded 1.2.3, percent-encoding, utf8_iter, zerovec,
  yoke, zerofrom, tinystr, litemap, writeable, potential_utf, zerotrie, stable_deref_trait and their derive macros
  with syn 2.0.119 and synstructure 0.14.0), sfv 0.15.0, futures 0.3.34 with its parts, indexmap, hashbrown 0.17.1,
  equivalent, smallvec, base64 0.22.1, displaydoc, ref-cast, tracing-attributes. npt-server's direct rcgen adds
  rcgen, yasna, time (+ time-core, deranged, num-conv, powerfmt). All already locked and in U5's and today's audits;
  none new to the lock. npt-bots gets the same transport set.
- The network surface is the intended one: web-transport-proto parses the client's CONNECT: the HEADERS frame is
  bounded (MAX_FIELD_SECTION_SIZE 16 KiB, connect.rs:12, :437-463), `:scheme` must be `https` (connect.rs:157-161,
  so no `blob:` scheme reaches `Url::parse`), and the URL is `Url::parse`d (connect.rs:191). url 2.5.9 carries the
  RUSTSEC-2026-0592 fix (SEC-R1D-1, closed in U6a's Block 0). Grep of crates/ and of web-transport-quinn,
  web-transport-proto, tungstenite and tokio-tungstenite sources: no `.origin()` call. U6b's brief keeps no Origin
  check and no `Url::origin()` on client input (Block 3, section 6's bans).
- Logging: web-transport-quinn and quinn emit `tracing` events; no member installs a subscriber (no
  tracing-subscriber in any manifest, no `tracing_subscriber` in crates/), so they reach no log. A subscriber would be
  a held manifest change and must keep E22 (web-transport-proto's Debug redacts query and userinfo but prints the
  path: connect.rs:289-300).
- Verdict: CLEAR WITH CONDITIONS: no `Url::origin()` (or `url::Origin`) on client input anywhere in npt-net or
  npt-server (grep at the build review); no tracing subscriber without its own read.

### 7. Item 6, the bites gate (DONE)
- After Block 0 the only `default =` line in any member is npt-net's `["wt-wtq"]`; `bites` is in none. npt-server's
  `bites` gains `npt-net/bites`; npt-lab's `bites` reaches it through npt-server/bites. check.mjs's bites build
  (`--all-features`, check.mjs:2017, :2057) compiles it; its measured build (no features) does not.
- U8's RS build: `cargo build --locked --offline` (u8-brief.md:195), no `--features`, no `--all-features`: the
  measured binaries hold no bite. The run header records the enabled features and the binary's SHA-256
  (plan.md:632-633), so a bites binary in a measured run is visible.
- Wording (SEC-U6BD-2, Low): the advisor's item 6 says "never in a run binary", but the plan does run bites binaries
  on RS (R1's "first bites", R2-R5's bite runs: plan.md:632, :819-822; r1.sh's `bites` block, u8-brief.md:251). The
  right line: `Bite::BudgetX2` is never in a *measured* run binary; a bites binary is a separate `--features bites`
  build of the named package, at its own path with its own hash, used only for bite runs, armed only by the
  operator's `--bite <id>` per WsServer instance, never by a client message or a network control command. What the
  bite does if armed (doubles N on one instance) weakens only our own back-pressure on that run: no secret, no
  authority.
- Verdict: CLEAR WITH CONDITIONS (as worded here; U8's R1.md says which binary each block runs).

### 8. Item 7 and the ruling: ring instead of aws-lc-rs across the TLS stack (DONE)
Sources (read today): rustls 0.23.46 src/crypto/ring/mod.rs:35, :69-90, :170-181 and the signature list;
src/crypto/aws_lc_rs/mod.rs:56-64, :102-128, :243-265 and its signature list (registry cache); quinn-proto 0.11.19
src/crypto/ring_like.rs:1, src/crypto/rustls.rs:3, src/config/mod.rs:180; Google Security Blog, "A new path for Kyber
on the web" (2024-09-13, security.googleblog.com/2024/09/a-new-path-for-kyber-on-web.html: Chrome 131 offers a
hybrid ML-KEM key share, codepoint 0x11EC) and RFC 10024 (the hybrid groups' definitions), both from today's search
summary, not read in full.
- Key exchange: ring's provider offers X25519, P-256, P-384 (ring/mod.rs:177-181). aws-lc-rs's default list is the
  same three plus X25519MLKEM768 last (aws_lc_rs/mod.rs:254-262; first only with `prefer-post-quantum`, which no build
  turns on). So the only loss is the post-quantum hybrid group: protection of recorded traffic against a future
  quantum attacker. For a 4-week test with synthetic game state, per-run Join tokens and a 13-day dev certificate,
  that is no loss that matters. Every R1 browser offers X25519 (Chrome, Edge, Firefox, Safari); if a browser sent
  only a hybrid key share, TLS 1.3's HelloRetryRequest (RFC 8446 section 4.1.4) costs one extra round trip in the
  handshake, inside the 2 s Join deadline, never a failure. Confidence: high on rustls's lists (source read), medium
  on each browser's exact key shares (not read today).
- Cipher suites: the same TLS 1.3 set in both (AES-256-GCM, AES-128-GCM, ChaCha20-Poly1305; TLS 1.2 off: `tls12` off
  in every build), so QUIC's TLS13_AES_128_GCM_SHA256 check (wt.rs:68-75) holds on ring.
- Signatures: ring verifies ECDSA P-256/P-384 (SHA-256/384), Ed25519, RSA PKCS#1 and PSS 2048-8192; aws-lc-rs adds
  ECDSA P-521, ECDSA with SHA-512 and ML-DSA-44/65/87. Ours: rcgen's ECDSA P-256 dev certificate and Let's Encrypt's
  ECDSA (P-256/P-384) or RSA chains: all covered; our clients pin and still check the handshake signature (Block 1).
- quinn-proto's own crypto (reset-token HMAC, retry and token AEAD, initial keys) already runs on ring today: it picks
  aws-lc-rs only when ring is off (ring_like.rs:1, rustls.rs:3, config/mod.rs:180). Unchanged by Block 0.
- Floors: ring 0.17.14 >= PM-0001-35's 0.17.12; OSV: no advisory on ring 0.17.14 or rustls 0.23.46.
- rustls features that change: rustls loses `aws-lc-rs`/`aws_lc_rs` (keeps log, logging, ring, std); rustls-webpki
  loses `aws-lc-rs`; quinn and quinn-proto lose `aws-lc-rs` and `rustls-aws-lc-rs`; web-transport-quinn goes from
  [aws-lc-rs, default] to [ring]. Off before and after: `prefer-post-quantum`, `tls12`, `fips`, `brotli`, `zlib`,
  `custom-provider`, `read_buf`.
- Gains: aws-lc-sys's C build (cmake, its build script, a bundled C library) leaves every build, seven crates leave
  the lock; one provider for all three TLS configurations; the dual-provider panic in library builders is gone.
- **Ruling: nothing weaker for our runs. CLEAR.** Item 7: a note, not a blocker (SEC-U6BD-3, Low, recorded so a
  later production design revisits the hybrid group when rustls's ring provider or another provider offers it).

### 9. Item 8, the tools and the audit (DONE)
- Installed today for U6a's Block 0 (u6a-block0.md step 5): `cargo deny --version` prints cargo-deny 0.22.0,
  `cargo audit --version` prints "cargo-audit-audit 0.23.0" (read-only checks, run now). **Yes again**, for U6b's
  Block 0: use these installed binaries; a reinstall, if one is needed, only as security-u6a-deps.md section 6 (ii)
  words it (`cargo install --locked --version ...`, one at a time, build dirs under .gate/). The advisory databases
  stay in their default home locations, never in the repository or a worktree. SEC-U5D-5 stays accepted.
- Clean, cargo-deny: `cargo deny --manifest-path spikes/netcode-proof/Cargo.toml --locked check advisories bans
  sources` exits 0 with "advisories ok, bans ok, sources ok"; warnings only the multiple-versions ones of U6a's run
  (getrandom, syn, synstructure, windows-link, windows-sys) or a subset; deny.toml unchanged. pin-utils not reported
  (cargo-deny 0.22.0's `workspace` scope for unmaintained, as U6a's run showed).
- Clean, cargo-audit: `cargo audit --file spikes/netcode-proof/Cargo.lock` reports 235 crate dependencies scanned,
  0 vulnerabilities, and exactly one allowed warning, RUSTSEC-2026-0441 (pin-utils 0.1.1, unmaintained). Any other
  warning, any `unsound` or `yanked` entry, or any vulnerability comes back to me before the save.
- The database dates: for cargo-audit, `git -C ~/.cargo/advisory-db log -1 --format=%cI` (or the fetch line it
  prints); for cargo-deny, the same on its database folder under ~/.cargo/advisory-dbs/. Both in the evidence.
- dependency-check.md: a new section in U6a's shape, its header naming this file and its conditions, then `sha256
  path` for the four changed files: spikes/netcode-proof/Cargo.toml, Cargo.lock, crates/net/Cargo.toml,
  crates/server/Cargo.toml (deny.toml unchanged, not listed as changed).
- Verdict: CLEAR WITH CONDITIONS.

## Findings
- **SEC-U6BD-1 (Medium):** the expected lock diff in u6b-brief.md section 0.1 condition 2 and advisor-u6b-brief.md
  section 4 item 3 names six removed packages and "nothing else"; the lock will also lose jobserver 0.1.35 and cc's
  ` "jobserver",` and ` "libc",` lines, because cc's `parallel` feature was requested by aws-lc-sys alone. Run as
  written, the binding check fails closed (a stop on Wednesday, not a wrong lock). Evidence: `cargo metadata` edge
  `aws-lc-sys 0.47.0 -> cc feats=['parallel']`, the only edge asking for it; cc 1.11.1 Cargo.toml [features]
  `parallel = ["dep:jobserver", "dep:libc"]`; the lock's cc entry lists jobserver and libc; jobserver's only dependent
  is cc. Fix: the binding check below (seven packages, five array lines, one added line). Confidence: high (the real
  refresh proves it).
- **SEC-U6BD-2 (Low):** item 6's "never in a run binary" should read "never in a measured run binary" (bites
  binaries run on RS by plan); conditions in section 7.
- **SEC-U6BD-3 (Low, note):** ring has no X25519MLKEM768; no effect on our runs; revisit for production.

## Verdicts
1. Exact manifest content: CLEAR WITH CONDITIONS (section 2).
2. Four provider lines, ring per configuration, no process default: CLEAR WITH CONDITIONS (section 3).
3. Exact lock diff: CLEAR WITH CONDITIONS, on the corrected diff (section 4; SEC-U6BD-1): seven packages, not six.
4. Feature tree: CLEAR WITH CONDITIONS (section 5).
5. What `default = ["wt-wtq"]` adds: CLEAR WITH CONDITIONS (section 6).
6. bites: CLEAR WITH CONDITIONS (section 7; SEC-U6BD-2).
7. ring's key-exchange groups: CLEAR, as a note (section 8; SEC-U6BD-3).
8. cargo-deny 0.22.0 and cargo-audit 0.23.0: yes again; CLEAR WITH CONDITIONS (section 9).
Ruling on ring for aws-lc-rs across the TLS stack: CLEAR; nothing weaker for our runs.

## Binding check for U6b's Block 0 save (the main session; in u6a-block0.md's shape)
Run in spikes/netcode-proof at `<base>`'s tree, before `<base>` is filled. Record each output in u6b-block0.md.
0. Before: `sha256sum Cargo.toml Cargo.lock crates/net/Cargo.toml crates/server/Cargo.toml deny.toml` equals
   5ebfbedf...0d36a, c55bab3c93d86c0068f572d9b20d152fe0080a6f29f278617615db04836e6b4d, fa2b3c2a...f523,
   e694b595...a60d, 90404ee6...69ae1; `grep -c '^\[\[package\]\]' Cargo.lock` = 242. If any differs (U6a's merge
   changed a manifest or the lock), stop: back to me with the new hashes; the rest of this list no longer binds.
1. The manifest edits exactly as section 2 (and the provider lines as section 3); `git diff --stat` touches only the
   three Cargo.toml files plus wt.rs, wt_tests.rs, ws_config.rs.
2. The lock refreshed offline: `cargo metadata --offline --format-version 1 > /dev/null` (without --locked), or
   `cargo check --offline` without --locked. Never `cargo update` (with or without -p) or `generate-lockfile`.
3. The lock diff: `git diff --numstat -- Cargo.lock` prints `1	65`; `git diff -U0 -- Cargo.lock | grep '^+[^+]'`
   prints exactly ` "rcgen",` (one line); the removed lines are the seven blocks (aws-lc-rs 1.19.0, aws-lc-sys
   0.47.0, cmake 0.1.58, dunce 1.0.5, fs_extra 1.3.0, jobserver 0.1.35, pkg-config 0.3.34), ` "aws-lc-rs",` in
   quinn-proto, rustls and rustls-webpki, and ` "jobserver",` and ` "libc",` in cc; `grep -c '^\[\[package\]\]'
   Cargo.lock` = 235; `grep -c -E 'aws-lc|cmake|dunce|fs_extra|pkg-config|jobserver' Cargo.lock` = 0; and
   `sha256sum Cargo.lock` = 210bf2df2d968dc457a76a0c75f20f7dd06fd2a0f9c33ef05e5c7c2a3b2ca219 (my model; if only this
   hash differs while every other line of step 3 holds, send me the diff; anything else in the diff: stop, back to
   me).
4. The feature tree (`--locked --offline` on every call):
   - `cargo tree --workspace --all-features -i aws-lc-rs` fails with "did not match any packages";
   - `cargo tree -p npt-server -e features -i rustls --depth 1` lists rustls features log, logging, ring, std only;
     the same with `--workspace --all-features`;
   - `cargo tree -p npt-server -e features -i web-transport-quinn --depth 1`: feature `ring` only (no `default`, no
     `aws-lc-rs`); `-i quinn-proto`: no `aws-lc-rs`, no `rustls-aws-lc-rs`;
   - `cargo tree -p npt-server -e features -i rcgen --depth 1`: `crypto` and `ring` only; the same with
     `--workspace --all-features`; never `aws_lc_rs`, `fips`, `pem`, `x509-parser`;
   - `cargo tree -p npt-server -e normal --depth 1` lists npt-net and rcgen v0.14.10 and no `time`;
     `cargo tree -p npt-server -e normal -i url` shows url v2.5.9 under web-transport-proto and web-transport-quinn
     only;
   - `grep -n -E '^default *=' crates/*/Cargo.toml` prints only npt-net's `default = ["wt-wtq"]`;
   - `grep -rn -E 'install_default|set_default|ClientBuilder|ServerBuilder|Client::default|\.origin\(\)'
     crates/net crates/server` prints nothing.
5. `cargo check --workspace --all-targets --locked`, 0 warnings; check.mjs's test counts before and after Block 0,
   the wt module's tests joining the measured build listed by name (condition 2's item (4)).
6. `cargo deny --locked check advisories bans sources` and `cargo audit --file Cargo.lock`, clean as section 9 says,
   with both database dates.
7. dependency-check.md: the new section with the four final hashes (section 9). Then the save.
Anything outside steps 0-6 (another package, a version or checksum line, a feature not listed, a manifest key not in
section 2) comes back to me before the save.

DONE
