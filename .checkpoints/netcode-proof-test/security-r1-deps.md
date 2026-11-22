Task: security-auditor DEP-HOLD read of every dependency the netcode proof test's R1 path adds (PR #40, lane
claude/netcode-proof-test-plan-hcky5g; replan-units.md:196 schedules it Sun 22 Nov). Started 2026-11-22.
Rule: main's .claude/CHECKPOINTS.md §3 DEP-HOLD. Candidates: futures-util, rcgen direct, instant-acme + hyper stack,
wasm-bindgen, the npm set (U7 TypeScript, U8 browser scripts / WebdriverIO). No installs, no builds: metadata,
advisories and source only. (No Edit tool in this agent: the notes are rewritten whole with Write at each block.)

## Plan (blocks, in order)
- B0: baseline. Lane head SHA; spikes/netcode-proof Cargo.toml + Cargo.lock: which candidates are already locked, at
  which versions, pulled by whom; any package.json in the spike. Units' intended versions (replan.md:61-72,
  replan-units.md §3-§5, plan.md:1256). Output: the exact NEW set.
- B1: futures-util (U6 WsServer) - in lock or not; if a direct dependency only, what changes.
- B2: rcgen direct (U6 WebTransport certificate at start) - in lock at which version; features; advisories.
- B3: instant-acme + hyper stack (U6 DNS-01 order) - version, the new transitive crates against the lock, features
  (ring vs aws-lc-rs, hyper-rustls, no native-certs?), maintainers, build scripts, advisories.
- B4: wasm-bindgen (U7) - in lock; what the U7 crate adds (wasm-bindgen-futures, web-sys, js-sys); build scripts,
  proc macros; wasm-bindgen-cli version pin.
- B5: npm set (U7 TypeScript, U8 WebdriverIO specs) - versions, install scripts, advisories, transitive size.
- B6: RustSec / GHSA sweep for every new crate at its pinned version; what could not be checked.
- B7: verdicts, conditions, dependency-check.md shape per unit; DONE.

## Notes (findings and block completions)

### B0 (baseline) DONE
- Lane head 948d6fc3d3a1 (2026-11-22 13:23 UTC). spikes/netcode-proof/Cargo.lock: 242 packages, lock v4. No
  package.json anywhere in spikes/netcode-proof (find, node_modules excluded): the whole npm set is new.
- Already locked, at the versions the units use (Cargo.toml workspace deps; lock read):
  web-transport-quinn =0.13.6 (npt-net, optional `wt-wtq`), tokio-tungstenite =0.30.0 + tungstenite 0.30.0 (npt-net),
  futures-util =0.3.34 (workspace dep, default-features off, ["sink","std"]; npt-net dev-dep today; also pulled by
  tokio-tungstenite, js-sys, futures), rcgen 0.14.10 (via wtransport 0.7.2's default `self-signed` feature:
  wtransport Cargo.toml [features] default = ["self-signed","ring"], self-signed = ["dep:rcgen"]; npt-net does not
  turn wtransport's defaults off), wasm-bindgen 0.2.130 + js-sys 0.3.107 (wasm32 deps of getrandom 0.2/0.4 and
  web-time), rustls 0.23.46, ring 0.17.14, tokio-rustls 0.26.6, rustls-platform-verifier 0.7.1,
  rustls-native-certs 0.8.5, http 1.5.0, httparse 1.10.1, pem 3.0.6, x509-parser 0.18.1, time 0.3.55.
- NOT locked: instant-acme, hyper, hyper-util, hyper-rustls, h2, http-body(-util), wasm-bindgen-futures, web-sys.
- Units' intents: plan.md:965-966 (instant-acme 0.8.5, default features off + ring + hyper-rustls, "keeps the ring
  floor and avoids aws-lc-rs's C build"); plan.md:643-644 (Node 22, TypeScript 6.0.x, Vite, WebdriverIO, three.js
  exact; `npm ci --ignore-scripts`); replan-units.md:139 (U7 drops render.ts and three.js), :134 (wasm-bindgen new),
  :147 (Node tests with a fake transport), :159 (WebdriverIO specs Chrome/Edge/Firefox on the laptop);
  plan.md:1171 (`cargo install wasm-bindgen-cli --locked` at Cargo.lock's version, on RS).
- Method for the new graph: no cargo run (the brief allows writes only by Write/Edit): a read-only resolver
  (scratchpad scripts/resolve.py: sparse index over HTTPS, lock versions preferred, newest index version with
  rust-version <= 1.99.1 otherwise, feature unification incl. `dep/feat` enabling optional deps; Linux + wasm targets).
  It is an approximation: the main session's real `cargo update` lock diff is the binding list (condition).
- Advisory source: OSV (api.osv.dev /v1/query and /v1/querybatch, ecosystem crates.io / npm; OSV imports RustSec and
  GHSA), queried 2026-11-22. GitHub's API for rustsec/advisory-db is closed to this session (HTTP 403, "GitHub
  access to this repository is not enabled"), so RustSec was read through OSV, not the repo.

### B1 futures-util DONE
- Already a workspace dep at =0.3.34 with default-features off, ["sink","std"] (Cargo.toml workspace.dependencies);
  already locked and already compiled (tokio-tungstenite 0.30.0 depends on it). Moving it from npt-net's
  [dev-dependencies] to [dependencies] adds no package and no checksum to Cargo.lock. crates.io: 0.3.34 is
  max_stable, published 2026-08-11 by taiki-e; owner rust-lang-owner (crates.io API /crates/futures-util, /owners).
  No build.rs (registry src). OSV: none affecting 0.3.34 (history RUSTSEC-2020-0059, -0062, both old ranges).
- Verdict: CLEAR (no new code).

### B2 rcgen direct DONE
- 0.14.10 is locked (checksum 8774e05a... = the cached .crate's sha256) and max_stable (2026-08-28, djc); owners est31
  + github:rustls:rcgen-publish (crates.io API). `#![forbid(unsafe_code)]` (src/lib.rs:29); no build.rs.
- Features (rcgen Cargo.toml [features]): default = ["crypto","pem","ring"]; aws_lc_rs pulls aws-lc-sys (C build);
  pem = dep:pem with `version = "4"` (Cargo.toml:117-118), NOT the locked pem 3.0.6. So a direct dependency
  `default-features = false, features = ["ring"]` (ring implies crypto) adds no crate. With defaults on, pem 4.0.0 +
  base64 0.23.1 come in (they come anyway with instant-acme, B3).
- The WT certificate needs only DER (rustls/quinn take DER). Never the `aws_lc_rs`/`fips` features.
- OSV: no advisory for rcgen at all. Verdict: CLEAR WITH CONDITIONS: `=0.14.10`, default-features = false,
  features = ["ring"]; the lock gains no package from it.

### B3 instant-acme + hyper stack DONE
- instant-acme 0.8.5: max_stable, 2026-02-24, published by djc; owners djc, cpu (crates.io API); repo djc/instant-acme;
  Apache-2.0; rust-version 1.70; checksum 9f05ad37c421b962... (index = the downloaded .crate's sha256).
  Features (Cargo.toml in the .crate): default = ["aws-lc-rs","hyper-rustls"]; ring = ["dep:ring",
  "hyper-rustls?/ring", "rcgen/ring"] (non-weak: `ring` also turns rcgen on, with instant-acme's own `features =
  ["pem"]` on that dep); hyper-rustls = [dep:hyper, dep:hyper-rustls, dep:hyper-util, dep:rustls]; hyper-rustls dep is
  default-features off with http1, http2, native-tokio, tls12, rustls-platform-verifier; hyper with client, http1,
  http2 (so h2 always comes with hyper-rustls; no feature turns HTTP/2 off).
- Source read (3,012 lines src/): no unsafe, no build.rs, no fs/env/process use; network only to the directory URL the
  caller gives. DefaultClient (lib.rs:199-226): `https_only()`, `try_with_platform_verifier()` (OS roots) or
  `with_roots(RootCertStore)`; HTTP/1 + HTTP/2. Response bodies collected whole with no size cap (lib.rs:312-318)
  and no request timeout in the client; RetryPolicy default timeout 30 s on order polling (order.rs:581-586).
  Exposure: only Let's Encrypt over validated TLS answers it, during a one-order egress window: Low.
  `NewOrder::profile` (types.rs:456-462) and `Account::revoke` (account.rs:155) exist; `KeyAuthorization::dns_value`
  (order.rs:555).
- New crates against the lane lock (resolver, features ring + hyper-rustls, default off): 17 = instant-acme 0.8.5,
  hyper 1.12.0, hyper-util 0.1.21, hyper-rustls 0.27.10, h2 0.4.20, http-body 1.1.0, http-body-util 0.1.5,
  httpdate 1.0.3, tokio-util 0.7.20, want 0.3.2, try-lock 0.2.5, atomic-waker 1.1.2, fnv 1.0.7, tower-service 0.3.3,
  async-trait 0.1.92 (proc macro), pem 4.0.0, base64 0.23.1 (via pem 4). Every other crate reached (73) is already
  locked: rustls 0.23.46 with ring, rustls-platform-verifier 0.7.1, rustls-native-certs 0.8.5, tokio-rustls 0.26.6,
  rcgen 0.14.10, http 1.5.0, httparse, serde, serde_json, tokio 1.54.1, futures-*, socket2, etc. No aws-lc-rs/
  aws-lc-sys activated by this path.
- Each new crate (streamed .crate, Cargo.toml read): no build.rs, no `links`, no native code; one proc macro
  (async-trait, dtolnay); unsafe lines: hyper 67, base64 61, tokio-util 31, hyper-util 18, try-lock 10, others 0-4
  (all widely used hyperium/tokio code). Licenses MIT / Apache-2.0 / ISC alternatives only.
- Owners and latest (crates.io API, version endpoint): hyper 1.12.0 2026-10-06 seanmonstar (owners seanmonstar,
  github:hyperium:core); hyper-util 0.1.21 2026-09-24 seanmonstar; hyper-rustls 0.27.10 2026-09-20 ctz (ctz, djc,
  github:rustls:publishers); h2 0.4.20 2026-10-06 seanmonstar (carllerche, seanmonstar, olix0r); http-body 1.1.0
  2026-07-13; http-body-util 0.1.5 2026-08-12; tokio-util 0.7.20 2026-10-09 Darksonn (github:tokio-rs:core); want
  0.3.2 2026-10-05; try-lock 0.2.5 2023-12-07; atomic-waker 1.1.2 2023-09-25 (taiki-e, smol-rs admins); fnv 1.0.7
  2020-05-14 (servo); httpdate 1.0.3 2023-08-13 (pyfisch, sole owner); tower-service 0.3.3 2024-08-13; async-trait
  0.1.92 2026-08-08 dtolnay; pem 4.0.0 2026-07-28 jcreekmore (sole owner); base64 0.23.1 2026-08-04.
- OSV at each pinned version: none affecting any of the 17. History: h2 RUSTSEC-2023-0034, -2024-0003, -2024-0332,
  RUSTSEC-2026-0258 ("h2 unbounded empty DATA frames", fixed 0.4.16) - all fixed below 0.4.20; hyper's seven are all
  0.x/old ranges; base64 RUSTSEC-2017-0004 old.
- Verdict: CLEAR WITH CONDITIONS (B7).

### B4 wasm-bindgen + wasm-bindgen-cli DONE
- wasm-bindgen 0.2.130 (+ -macro, -macro-support, -shared 0.2.130, js-sys 0.3.107, bumpalo 3.20.3) already locked;
  resolver: adding `wasm-bindgen = "=0.2.130"` (default ["std"]) adds 0 crates. 0.2.130 is max_stable (2026-10-30).
  Owners RReverser, daxpedda, guybedford, github:wasm-bindgen:wasm-bindgen-publish; repo moved to
  github.com/wasm-bindgen/wasm-bindgen (crates.io API).
- Build scripts: wasm-bindgen/build.rs reads CARGO_CFG_* env, prints cfgs (rustversion check), no fs/net;
  wasm-bindgen-shared/build.rs (`links = "wasm_bindgen"`) hashes its own src/lib.rs and src/tys.rs and spawns
  `git rev-parse HEAD` in its own dir (Command::new("git"); output only sets WBG_VERSION); no network, no writes.
  wasm-bindgen-macro is a proc macro (code generation at build; its output is the npt-client-wasm glue the unsafe
  allow covers: plan.md:440).
- U7 needs only wasm-bindgen (+ js-sys if a Uint8Array crosses): the Worker owns WebTransport/WebSocket in TypeScript
  (replan-units.md:134-137), so no web-sys, no wasm-bindgen-futures (they would be new: a further read).
- wasm-bindgen-cli 0.2.130 (tool, `cargo install --locked`): checksum c37f6904387d19a3...; rust-version 1.86; ships a
  Cargo.lock of 254 packages, crates.io + path only. Default feature rustls-tls (ureq + rustls/ring); openssl,
  native-tls, vendored-openssl optional (never enable). Network code is only in wasm-bindgen-test-runner (ureq to a
  local WebDriver, rouille server; it does not download drivers: headless.rs:960-975); the `wasm-bindgen` binary has
  none. OSV over its lock: 6 hits, all in the test-runner's stack: buf_redux, multipart, safemem, twoway, chrono
  (RUSTSEC-2026-0543 unmaintained), url 2.5.8 (RUSTSEC-2026-0592, see F1). None reach the `wasm-bindgen` binary's
  input path (it reads our own .wasm). OSV: no advisory for wasm-bindgen-* or js-sys at all.
- Verdict: wasm-bindgen CLEAR WITH CONDITIONS; wasm-bindgen-cli CLEAR WITH CONDITIONS (B7).

### F1 (found on the way, not a new dependency): url 2.5.8 in the lane lock, RUSTSEC-2026-0592 / GHSA-hrj5-67jp-wqp9
- OSV querybatch over the lane lock (225 registry packages): 2 hits. url 2.5.8 RUSTSEC-2026-0592 (published
  2026-11-20): `Url::origin()` recursion on nested `blob:` URLs, stack overflow aborts the process; fixed 2.5.9;
  affected functions url::Url::origin, url::quirks::origin. pin-utils 0.1.1 RUSTSEC-2026-0441 (unmaintained,
  informational), reached only via loom 0.7.2 (cfg(loom) test tooling).
- url users in the lock: web-transport-proto 0.6.4, web-transport-quinn 0.13.6, wtransport 0.7.2, wtransport-proto
  0.7.2 (network-facing). grep of their src and tungstenite/tokio-tungstenite: no `Url::origin()` call (wtransport's
  `origin()` returns the header string: endpoint.rs:690-691, session.rs:175). So not reachable today: Medium, because
  (a) deny.toml has `ignore = []` (deny.toml [advisories]), so plan.md:645-650's `cargo deny check advisories` at
  stop B and before every run fails until it is handled, and (b) any Origin check written with `Url::origin()` on a
  client URL would make it a one-request process abort.
- Fix: in U6's lock change, `cargo update -p url --precise 2.5.9` (needs form_urlencoded 1.2.3, index: url 2.5.9 deps
  `form_urlencoded ^1.2.3`; percent-encoding stays 2.3.2). url 2.5.9: 2026-11-20 by Manishearth, owners SimonSapin,
  Hoverbear, seanmonstar, servo teams; checksum f00cd2407bb42c0e...; diff vs 2.5.8 read (crate_diff.py): origin.rs
  takes an inner URL's origin only for http/https/file (the fix), parser.rs percent-encode set reordering, no new
  dep. form_urlencoded 1.2.3 (checksum afa489199eaffb12..., 2026-11-20 Manishearth): one change, a panic on a too-long
  start_position replaced by a clamp. OSV: none for url 2.5.9 or form_urlencoded 1.2.3. pin-utils: record it as an
  accepted informational advisory in deny.toml with its reason (test-only via loom), or confirm cargo-deny's
  unmaintained scope excludes it; not a runtime risk.
