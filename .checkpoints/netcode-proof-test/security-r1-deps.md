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
  informational), reached only via loom 0.7.2 (cfg(loom) test tooling); already seen at U5 (security-u5-deps.md:241).
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
  start_position replaced by a clamp. OSV: none for url 2.5.9 or form_urlencoded 1.2.3. pin-utils: as U5 handled it.

### B5 the npm set DONE
- Method: a read-only Node resolver (scratchpad scripts/npm_tree.mjs: npm's bundled semver, registry packuments
  over HTTPS, deps + non-optional peers + optional deps for linux-x64 and win32-x64), then OSV querybatch (npm).
  Approximate like the cargo one: the committed package-lock.json is the binding list.
- typescript 6.0.3: latest 6.0.x (6.0.2, 6.0.3; latest overall is 7.0.2, the native Go port, not the plan's 6.0.x);
  2026-04-16, publisher typescript-bot; 0 dependencies; no preinstall/install/postinstall; OSV none. 1 package.
- vite 8.3.8 (2026-11-11, GitHub Actions with npm provenance): 19 packages over linux-x64 + win32-x64, incl. native
  prebuilt binaries (@rolldown/binding-*-1.2.19, lightningcss-*-1.33.0; lightningcss published by devongovett without
  provenance); no install scripts; OSV none. 8 of the 19 were published in the last 14 days (vite, rolldown and its
  bindings 2026-11-11, postcss 8.5.33 2026-11-13, source-map-js 1.2.4 2026-11-19).
  Not needed for what U7 now is (a counters page, a module Worker, wasm-bindgen output): `tsc` + wasm-bindgen's
  `--target web` ES module serve as static files. Option, not a condition: drop Vite (19 packages and two native
  toolchains less). If kept: `vite build` only, never `vite`/`vite preview` with `--host`.
- @types/node 22.20.5 (latest 22.x, 2026-10-01) only if the TypeScript uses Node types (Node tests).
- WebdriverIO: 10.2.0 (2026-11-06, provenance, maintainers christian-bromann, wdio-user, wswebcreation-nl).
  * standalone `webdriverio@10.2.0`: 162 packages; OSV: minimatch 10.2.6 GHSA-g3hv-75wg-m249 (CVE-2026-118764,
    globparts amplification DoS, CVSS 4.0 VA:L; fixed only in 11.0.1, no 10.x fix). Install scripts: edgedriver 8.0.0
    and geckodriver 8.0.0 postinstall (driver download; `--ignore-scripts` skips them).
  * the testrunner set (@wdio/cli, local-runner, mocha-framework, spec-reporter, webdriverio, all 10.2.0): 321
    packages; adds tsx 4.23.15 -> esbuild 0.28.2 (postinstall `node install.js`, native @esbuild/* binaries), the
    @inquirer/* family published 2026-11-22 (today), glob 13.0.7 GHSA-3g3m-2fjr-6pw4 (CVE-2026-118773, fixed 14.0.2)
    beside minimatch.
  * 9.33.1 standalone: 219 packages, OSV: extract-zip 2.0.1 GHSA-7pqw-9j4j-h8q3 (arbitrary file write via symlink
    archive entries, CVSS 3.1 I:H/A:H, no fix: used for driver downloads), basic-ftp 5.3.1 x3, glob 10.5.0,
    minimatch 9/10. So never the 9.x line.
- WebdriverIO at run time (@wdio/utils 10.2.0 build/index.js, startWebDriver, read from the published tarball):
  `WDIO_SKIP_DRIVER_SETUP` makes it connect to localhost:4321 instead; otherwise for Chrome it calls
  setupPuppeteerBrowser (may download Chrome for Testing) and setupChromedriver (downloads chromedriver) unless
  `wdio:chromedriverOptions.binary` / CHROMEDRIVER_PATH is set; Firefox: setupPuppeteerBrowser + geckodriver
  (downloads unless `wdio:geckodriverOptions.binary`); Edge: edgedriver (downloads unless
  `wdio:edgedriverOptions.binary`). Downloads are not pinned by hash in our config.
  It also defaults chromedriver to `allowedOrigins ["*"]`, `allowedIps ["0.0.0.0"]` and msedgedriver to
  `allowedIps ["0.0.0.0"]` (geckodriver gets allowHosts ["localhost"]). ChromeDriver's own guidance: "By default,
  ChromeDriver only allows local connections", remote access via `--allowed-ips`, "Use firewall to prevent
  unauthorized remote connection to ChromeDriver" (https://developer.chrome.com/docs/chromedriver/security-
  considerations). Whether "0.0.0.0" admits every remote peer I did not verify in chromedriver's source (medium
  confidence); a WebDriver session can start any binary as "the browser", so a reachable driver port on the laptop
  is code execution as the laptop user.

### B6 advisory sweep DONE
- Rust, OSV per pinned version (2026-11-22): none affecting instant-acme 0.8.5 or the 16 crates under it, rcgen
  0.14.10, futures-util 0.3.34, wasm-bindgen(-cli/-macro/-macro-support/-shared) 0.2.130, js-sys 0.3.107,
  web-transport-quinn 0.13.6, tokio-tungstenite/tungstenite 0.30.0, rustls 0.23.46, ring 0.17.14, tokio 1.54.1,
  tokio-rustls 0.26.6, quinn 0.11.12, quinn-proto 0.11.19, http 1.5.0, httparse 1.10.1, x509-parser 0.18.1, time
  0.3.55, rustls-platform-verifier 0.7.1, rustls-native-certs 0.8.5. Whole lane lock: url 2.5.8 (F1), pin-utils
  (informational, known).
- npm, OSV per resolved version: typescript + vite: none; webdriverio 10.2.0: minimatch 10.2.6 (Low here: the
  patterns are our own spec globs, no attacker input); testrunner adds glob 13.0.7 (Low, same reason).
- Not checked: the RustSec repo directly (GitHub API 403 for this session; OSV mirrors it, GHSA aliases shown for h2,
  url); `npm audit` itself (no install); chromedriver's handling of allowed-ips 0.0.0.0 in source.

### B7 answer DONE
Findings:
- SEC-R1D-1 (Medium): url 2.5.8 in the lane lock (RUSTSEC-2026-0592, GHSA-hrj5-67jp-wqp9; Url::origin() stack
  overflow on nested blob: URLs; fixed 2.5.9). Not reached by our crates today; blocks `cargo deny check advisories`
  (deny.toml ignore = []) at stop B and before every run. Exploit sketch: a client sends a URL with ~10,000 `blob:`
  prefixes; any server code that calls `Url::origin()` on it aborts the room process. Fix: U6's lock change runs
  `cargo update -p url --precise 2.5.9` (brings form_urlencoded 1.2.3); no Origin check uses `Url::origin()` on
  client input before that.
- SEC-R1D-2 (Medium): WebdriverIO 10.2.0 widens chromedriver/msedgedriver allow-lists by default (allowedIps
  ["0.0.0.0"], allowedOrigins ["*"], @wdio/utils build/index.js startWebDriver). Exploit sketch: a host on the
  laptop's network reaches the driver's port and starts a session whose "browser" binary is any program. Fix: set
  `wdio:chromedriverOptions` and `wdio:edgedriverOptions` explicitly with allowedIps ["127.0.0.1"] (chromedriver also
  allowedOrigins to none), keep geckodriver's localhost default, and the laptop's firewall blocks inbound.
- SEC-R1D-3 (Medium): WebdriverIO downloads browsers and drivers at run time, unpinned (@puppeteer/browsers 3.2.7,
  edgedriver 8.0.0, geckodriver 8.0.0). Exploit sketch: a compromised CDN or package download replaces chromedriver
  on the laptop. Fix: drivers installed by hand from the vendors with versions and SHA-256 recorded, given by
  `wdio:*Options.binary`; browsers given by goog:chromeOptions.binary / ms:edgeOptions.binary /
  moz:firefoxOptions.binary (the installed stable browsers), so nothing downloads; cacheDir under .gate/.
- SEC-R1D-4 (Low): instant-acme's DefaultClient has no per-request timeout and collects bodies uncapped
  (lib.rs:312-318). Fix: each ACME call under `tokio::time::timeout` inside the wrapper's 30-minute bound; the
  directory URL a constant per staging|production (as npt-cert's no-free-argument plan).
- SEC-R1D-5 (Low): minimatch 10.2.6 (GHSA-g3hv-75wg-m249) in webdriverio 10.2.0's tree, no 10.x fix; our own glob
  patterns only. Fix: record as an accepted risk beside `npm audit`'s output, or an `overrides` to minimatch 11.0.9
  if WebdriverIO's tests pass with it.
- SEC-R1D-6 (Low): freshness. Many npm versions are days old (testrunner's @inquirer/* today; vite 11 days). Fix: make
  each package-lock.json with `--before` 7 days back (security fixes excepted, named), so no version younger than 7
  days enters the lock.

Verdicts and pins:
- futures-util =0.3.34 (U6, npt-net normal dep, workspace entry unchanged): CLEAR.
- rcgen =0.14.10 direct (U6): CLEAR WITH CONDITIONS: default-features = false, features = ["ring"].
- instant-acme =0.8.5 (U6, npt-host only): CLEAR WITH CONDITIONS: default-features = false, features = ["ring",
  "hyper-rustls"]; never aws-lc-rs / fips; DefaultClient (https_only) with the platform verifier or with_roots, no
  custom plain-HTTP client; SEC-R1D-4.
- Its 16 transitive crates (U6): CLEAR at exactly hyper 1.12.0, hyper-util 0.1.21, hyper-rustls 0.27.10, h2 0.4.20,
  http-body 1.1.0, http-body-util 0.1.5, httpdate 1.0.3, tokio-util 0.7.20, want 0.3.2, try-lock 0.2.5,
  atomic-waker 1.1.2, fnv 1.0.7, tower-service 0.3.3, async-trait 0.1.92, pem 4.0.0, base64 0.23.1.
- url 2.5.9 + form_urlencoded 1.2.3 (U6, F1 fix): CLEAR.
- wasm-bindgen =0.2.130 (U7, npt-client-wasm only): CLEAR WITH CONDITIONS: default features (std) only, no
  serde-serialize; no web-sys or wasm-bindgen-futures without another read; js-sys =0.3.107 allowed (locked).
- wasm-bindgen-cli 0.2.130 (U7 here, U8 on RS): CLEAR WITH CONDITIONS: `cargo install wasm-bindgen-cli --version
  0.2.130 --locked` with default features only (never openssl / native-tls / vendored-openssl), CARGO_TARGET_DIR
  under .gate/, version equal to the lock's wasm-bindgen; only the `wasm-bindgen` binary is used (not
  wasm-bindgen-test-runner, whose stack holds the unmaintained crates and url 2.5.8).
- typescript 6.0.3 exact (U7): CLEAR. @types/node 22.20.5 exact if needed: CLEAR.
- vite 8.3.8 exact (U7, if kept): CLEAR WITH CONDITIONS: `vite build` only, never the dev/preview server with
  `--host`; recommendation: drop it.
- webdriverio 10.2.0 exact, standalone (U8): CLEAR WITH CONDITIONS: SEC-R1D-2, -3, -5; specs as plain Node scripts
  (no @wdio/cli testrunner). The testrunner set (321 packages, esbuild postinstall, packages published today) is
  not cleared by this read: it needs its own read if U8 wants it. Never the 9.x line (extract-zip
  GHSA-7pqw-9j4j-h8q3).
- Every npm install: `npm ci --ignore-scripts`; web/.npmrc with `ignore-scripts=true`; package-lock.json committed,
  every `resolved` on https://registry.npmjs.org/ with an sha512 `integrity`; `npm audit` output recorded (SEC-R1D-5
  as the one accepted entry); SEC-R1D-6 when the lock is made.

Binding check for each unit's save (the main session, before writing its dependency-check.md lines):
- U6: the real `cargo update` lock diff adds exactly the 17 packages above (crates.io source; checksums equal the
  index's, listed below), bumps url 2.5.8 -> 2.5.9 and form_urlencoded 1.2.2 -> 1.2.3, and changes no other package.
  Anything else (a newer patch released meanwhile, an extra crate, aws-lc-*) returns here before the save.
  Then `cargo deny check advisories bans sources` and `cargo audit` on that lock, with the database date.
- U7: the lock gains no package from wasm-bindgen; web/package-lock.json holds typescript 6.0.3 (+ @types/node,
  vite's 19 if kept) and nothing more.
- U8: its package-lock.json's set equals webdriverio 10.2.0's tree as resolved that day; a package with an install
  script beyond edgedriver and geckodriver, or any OSV hit beyond minimatch, returns here.
- Index checksums (sha256): instant-acme 0.8.5 9f05ad37c421b962354c358d347d4a6130151df9407978372d3ad7f0c8f71a64;
  hyper 1.12.0 2c3e324da4c95177d6291d4c8730197c0d1822f8a9766814a4a44fa5ab797c9c; hyper-util 0.1.21
  ddc03d96684f9226b8a787cdb71488417b53ab5ea8fdb1dac946cb9431cc8bff; hyper-rustls 0.27.10
  dfa8e654703247911e29c23fbeaa261834bd9bb74efba2f9acddc37bfb127f53; h2 0.4.20
  7d29020232d6aa3fb1daca64c1127cf662cf97f254ae16c18c05b8ab635fc118; http-body 1.1.0
  ca2a8f2913ee65f60facd6a5905613afaa448497a0230cc41ce022d93290bc2c; http-body-util 0.1.5
  23169fe34a5fbcdd3f3862e78fb9b6fccd5f02a6dc6f732547005d45631ce71c; httpdate 1.0.3
  df3b46402a9d5adb4c86a0cf463f42e19994e3ee891101b1841f30a545cb49a9; tokio-util 0.7.20
  e464cf451ba96ebfc6f9b6542f17ee8b8956e33f1e40d9690624e59d7a7f8a4b; want 0.3.2
  ec4cdd0dd910afe868b7ef477227d8d538b46b3075031afee8a9f2acb0a2ed0b; try-lock 0.2.5
  e421abadd41a4225275504ea4d6566923418b7f05506fbc9c0fe86ba7396114b; atomic-waker 1.1.2
  1505bd5d3d116872e7271a6d4e16d81d0c8570876c8de68093a09ac269d8aac0; fnv 1.0.7
  3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1; tower-service 0.3.3
  8df9b6e13f2d32c91b9bd719c00d1958837bc7dec474d94952798cc8e69eeec3; async-trait 0.1.92
  82f6aeea286b8eb4dd3431a1be1b59d290ace00f5bfd8e2a159bc2a05e2c1667; pem 4.0.0
  d354a98a3d1251555de99e8fdd8afda05573c31b82f59063a7b0a29b5527f120; base64 0.23.1
  ac07cdecf99051d9a5238b80f35af32cdeba5b336e55d957b318b50137e18da5; url 2.5.9
  f00cd2407bb42c0ef17be88d4cf020f5cceffd121665bac9b8b9493a369b3b1a; form_urlencoded 1.2.3
  afa489199eaffb1211795df1c6236ce1ad0d41a6fa45250add73e3b02420f06e; wasm-bindgen-cli 0.2.130 (tool)
  c37f6904387d19a319525c74af274c73912f3871c35555aa49f3ea64ebb2fc7a.
- dependency-check.md per unit, in U5's shape (header naming this answer and its conditions, then `sha256  path` for
  every changed Cargo.toml / Cargo.lock / package.json / package-lock.json at the cleared content).
- Blocks: nothing blocks a unit's start. U6's merge needs the url bump to pass its own deny check; U8's specs need
  SEC-R1D-2 and -3 in the spec config before any run on the laptop.

DONE
