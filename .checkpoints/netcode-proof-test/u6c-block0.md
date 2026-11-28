# U6c Block 0: evidence (the main session, 2026-11-24; u6c-brief.md rev 1.1 section 5)
Binding check: security-u6c-deps.md:222-270 (steps 0-7), amended by security-u6c-lock.md (Ruling 3, :68-78; the
addendum, :99-128). Run in the main folder on the lane after U6b's merge (8efb9b2), before U6c's `<base>` is filled.
Every cargo call through main-run.sh (lane lock, 4 GB floor) or run_log.py; logs under .gate/u6c-block0/.

## Step 0, before: PASS (equal to security-u6c-deps.md:223-228)
Hashed from `git show HEAD:<path>` (Block 0 uncommitted, so HEAD holds the before content):
aab4be167675ebea16adfbdb36e87071c2bef068ff737c791ee9bb68aa9ecbd9  spikes/netcode-proof/Cargo.toml
210bf2df2d968dc457a76a0c75f20f7dd06fd2a0f9c33ef05e5c7c2a3b2ca219  spikes/netcode-proof/Cargo.lock
9f7a05e9dda8ba2348eb6e9849f8a46c224e4ec9e20a50a917ef715ab555be74  spikes/netcode-proof/crates/host/Cargo.toml
fa127e4d0befb08d282e0995d36195bff672e82a9478f856427d1dacc6c264c9  spikes/netcode-proof/crates/bots/Cargo.toml
90404ee6b66b3884129195ad1d6eaef8a97a023159d4ff2b8f888242d7369ae1  spikes/netcode-proof/deny.toml (unchanged by Block 0)
Lock: 235 [[package]] entries; `aws-lc|jobserver` 0; `^rcgen = ` prints U6b's entry. Cargo.toml differs from U6b's
Block 0 record (dependency-check.md, 7d0f39f2...) by R2's three profile lines only (`git diff --numstat ae4f57e HEAD`:
`3 0`; the sha2 opt-level, merged at 8efb9b2), no dependency line. Tools: cargo-deny 0.22.0, cargo-audit 0.23.0.

## Step 1, the edits (security-u6c-deps.md item 1, and SEC-U6CD-2's comment): done
- Cargo.toml (`12 2`): the tokio-rustls comment reworded (instant-acme's hyper-rustls turns rustls's tls12 on in any
  build with npt-host, so every server configuration names TLS 1.3 only); after rcgen, a comment and instant-acme
  =0.8.5 (default features off; ring, hyper-rustls), httparse =1.10.1, http =1.5.0, bytes =1.12.1, http-body-util
  =0.1.5.
- crates/host/Cargo.toml (`13 0`): [dependencies] instant-acme, rcgen (features ["pem"]), tokio, tokio-rustls,
  httparse, serde, serde_json, sha2; [dev-dependencies] http, bytes, http-body-util (each from the workspace).
- crates/bots/Cargo.toml (`6 0`): [dependencies] tokio, serde, serde_json; [dev-dependencies] npt-server.
- `git diff --numstat` touches only these three and Cargo.lock.

## Step 2, the lock refreshed with the network: done, with one pin-back
- `env CARGO_NET_OFFLINE=false cargo metadata --format-version 1`, then `cargo fetch --locked`, both exit 0,
  06:24:55-06:24:57 UTC (step2-metadata.log): the resolver took hyper 1.13.0 ("Adding hyper v1.13.0", :15), newer
  than the cleared 1.12.0.
- `env CARGO_NET_OFFLINE=false cargo update -p hyper --precise 1.12.0` ("Downgrading hyper v1.13.0 -> v1.12.0"), then
  `cargo fetch --locked` ("Downloaded hyper v1.12.0"), both exit 0, 06:25:27-06:25:29 UTC (step2b-pin.log): the one
  update step 2 allows.
- The pin re-resolved three Windows-only edges (windows-sys 0.61.2 -> 0.52.0 in quinn-udp, rustls-platform-verifier,
  winapi-util), so the lock differed from security's model and went back to security: security-u6c-lock.md CLEAR
  WITH CONDITIONS (SEC-U6CL-1 and -2, Low), step 3 amended (its Ruling 3).

## Step 3, the lock diff (as amended, security-u6c-lock.md:69-72): PASS
`git diff --numstat -- Cargo.lock`: `229 6`; 252 [[package]] entries; 17 `+checksum` and 17 `+version` lines, the
checksums each equal to security-r1-deps.md:258-274 (the binding check cites :258-273; base64 0.23.1's checksum wraps
onto :274); the six `-` lines ` "base64",` x2, ` "pem",` x1, ` "windows-sys 0.61.2",` x3; the arrays of item 7 as
listed (pem 3.0.6, sfv, wtransport, rcgen, npt-host, npt-bots); `aws-lc|jobserver|cmake|webpki-roots` 0;
`"windows-sys 0.52.0"` 4 and `"windows-sys 0.61.2"` 6; hyper 1.12.0; sha256
3394b451b06492758621445c250f44e93572e0b33a0269e68b7c9021b0f54873, security's after-lock (Ruling 2). The diff:
.gate/u6c-block0/lock.diff (sha256 efb07c46c8d96fc022273e5c224ed9d3a0fcc12c8bb1f718ee5675ba9bf4a070).

## Step 4, the feature tree (`--locked --offline`; step4-tree.log, 06:30:49-06:30:51 UTC): PASS
Against security-u6c-deps.md:244-262, with the addendum's amended lists (security-u6c-lock.md:118-120):
- instant-acme (npt-host): hyper-rustls, rcgen, ring (:4-9).
- hyper: client, default, http1, http2, no server (:10-17); hyper-util: client, client-legacy, default, http1, http2,
  tokio (:18-26); hyper-rustls: http1, http2, native-tokio, ring, rustls-native-certs, rustls-platform-verifier, tls12
  (:27-36). The three lines outside the first lists (hyper's and hyper-util's `default`, both empty; hyper-rustls's
  `rustls-native-certs`, native-tokio's implied optional dependency) went to security: accepted, SEC-U6CL-3 Low.
- rcgen: crypto, pem, ring (workspace, all features); crypto, ring (npt-server) (:37-48).
- rustls: log, logging, ring, std, tls12 (workspace, all features); log, logging, ring, std (npt-server) (:49-66);
  tokio-rustls: ring, tls12 (:67-72).
- npt-bots's tokio: bytes, default, io-util, libc, macros, mio, net, rt, rt-multi-thread, socket2, sync, time,
  tokio-macros, equal to item 4's list (security-u6c-deps.md:98-99): no fs, signal, process, io-std (:73-88).
- npt-host, normal, depth 1: httparse, instant-acme, npt-log, rcgen, serde, serde_json, sha2, tokio, tokio-rustls;
  dev: bytes, http, http-body-util (:89-107).
- npt-bots, normal: no line names npt-server (the log's npt-server lines are :44-66's two `-p npt-server` calls and
  :402); dev, depth 1: npt-server (:399-403).
- `--target all -d --depth 0`: base64, getrandom, pem, syn, synstructure, windows-link, windows-sys, the seven (:404-434).
- `-i aws-lc-rs`: "did not match any packages", exit 101 (:435-437).

## Step 5, `cargo check --workspace --all-targets --locked` then `node check.mjs`: PASS
Through main-run.sh, log .gate/u6c-block0/step5-check.log (06:31:27-06:52:17 UTC; same HEAD and diff hash as step 4's
run). `cargo check`: exit 0, no warning. check.mjs: "all 11 steps pass" (fmt, clippy, clippy-wasm32, doc, test, deps,
wasm, refs, marks, self, redact); measured 807, bites 904, 0 failed, 0 reported a skip, equal to U6b's merged head (its
M5 run 2: 807/904), so Block 0 adds and drops no test; test step 1086.7 s; deps: npt-super and npt-host keep off the
sim, room, proto and client crates.

## Step 6, cargo-deny and cargo-audit: PASS
- `cargo deny --manifest-path spikes/netcode-proof/Cargo.toml --locked check advisories bans sources`
  (step6-deny.log): "advisories ok, bans ok, sources ok" (:693); its warnings are the seven duplicate names only
  (base64 :1, getrandom :38, pem :103, syn :132, synstructure :401, windows-link :472, windows-sys :568); its database
  (~/.cargo/advisory-dbs/advisory-db-3157b0e258782691) fetched 2026-11-24 06:33:09 UTC.
- `cargo audit --file spikes/netcode-proof/Cargo.lock` (step6-audit.log): database fetched from
  https://github.com/RustSec/advisory-db.git (FETCH_HEAD 2026-11-24 06:33:11 UTC), 1,563 advisories, 252 crate
  dependencies, 0 vulnerabilities, "1 allowed warning found": pin-utils 0.1.1, unmaintained, RUSTSEC-2026-0441.
- Both dates by `stat` of each database's FETCH_HEAD, not by `git -C ... log` as step 6 words it (the guard refuses
  `git -C`), as at U6b's Block 0.

## Step 7, after (dependency-check.md's U6c section; then one commit and the save)
7992c8e10da940a2f610be7223bcf0d6d3e4c2e31c7b78a61edbe2e88925ca03  spikes/netcode-proof/Cargo.toml
3394b451b06492758621445c250f44e93572e0b33a0269e68b7c9021b0f54873  spikes/netcode-proof/Cargo.lock
a06c090c7ee4b3b2f0fdf4ef9e319943957de549fb291da5aeefd6feb062748f  spikes/netcode-proof/crates/host/Cargo.toml
3cf4b077b8c9f8a440c74759124720ba1f9a30bdda9370c90b7c6a0bad28f45f  spikes/netcode-proof/crates/bots/Cargo.toml
