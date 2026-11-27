# DEP-HOLD release, U5's early Block 0 (main's .claude/CHECKPOINTS.md §3; advisor-u5-early.md condition 3)
# security-auditor's answer, read in full (ends DONE): .checkpoints/netcode-proof-test/security-u5-deps.md
# Written 2026-11-21 ~15:3x UTC by the main session; every line below is one the answer cleared at that content.
# The answer's conditions: SEC-U5D-1 (Medium) to the advisor before Block 3; SEC-U5D-4 (Medium) a lock check around
# every cargo-fuzz call (or plain `cargo +nightly-2026-11-01 build --locked`); CUSTOM_LIBFUZZER_PATH and
# CUSTOM_LIBFUZZER_STD_CXX unset, fuzz outputs under .gate/; fuzz_target! only (SEC-U5D-3); cargo-fuzz never
# init/add/--build-std/--careful/--sanitizer=memory, its target dir under .gate/ or the shared target; the nightly only
# by `rustup toolchain install nightly-2026-11-01 --profile minimal`, called as +nightly-2026-11-01; cargo-fuzz by
# `cargo install --locked --version 0.13.2 cargo-fuzz`; cargo-deny 0.22.0 and cargo-audit 0.23.0 for Block 8 only, one
# at a time, build dirs under .gate/, the advisory database never inside the repository or a worktree.
3bf4d76008175aa7e779d2c1f504b09c5979a5e0e49ddcded8380b7045383b04  spikes/netcode-proof/Cargo.toml
31c2c7ea9fdf04f78391b0e6d90165f14b74483c87f235c5daf4ad5f8993a7a8  spikes/netcode-proof/Cargo.lock
fb5324f1ac53251ef03accb89a9182d62506eb98d074aed6165c00cc49efebf1  spikes/netcode-proof/crates/alloc/Cargo.toml
b5385fcdaa5dcc2344c41177b5afec923cea9d2004512245fc6d776a2f0c329a  spikes/netcode-proof/crates/sim/Cargo.toml
670914131fb08661aafdf8299bea21c18a15e2e7a33f754c295b1bbd61ea1b12  spikes/netcode-proof/crates/proto/Cargo.toml
afd0952b15865881f763b1cba9d0feac715d2cf104dc4b2c6d13d3ab87f7b265  spikes/netcode-proof/crates/net/Cargo.toml
7606244c186ebb95c486c2ef8c100740383cdcdf81f9e985b9495c1e328a369d  spikes/netcode-proof/fuzz/Cargo.toml
3117b9305e1ebbf8ed4d802ab3689a6f4296067275e66d84fd8d7b9e66bbf1fd  spikes/netcode-proof/fuzz/Cargo.lock

# DEP-HOLD release, U6a's Block 0 (main's .claude/CHECKPOINTS.md §3; advisor-u6a-brief.md condition 1)
# security-auditor's answers, read in full (each ends DONE): (a) the url 2.5.9 bump:
# .checkpoints/netcode-proof-test/security-r1-deps.md (SEC-R1D-1; binding check :249-253); (b) the manifest edits and
# step 5: .checkpoints/netcode-proof-test/security-u6a-deps.md (SEC-U6AD-1 to -6; binding check :119-137; section 6:
# cargo-deny 0.22.0 and cargo-audit 0.23.0 cleared for this Block 0; section 7: deny.toml's allow-wildcard-paths line,
# committed alone at 6aa78ff). Written 2026-11-22 ~14:2x UTC by the main session after binding checks 1-5 passed:
# cargo-deny "advisories ok, bans ok, sources ok" (exit 0, five duplicate warnings); cargo-audit 0 vulnerabilities, one
# allowed warning RUSTSEC-2026-0441 (evidence: u6a-block0.md). Every line below is at the cleared content.
5ebfbedfdf3015e1928aa140bbda1585ad39012521c11ede62f40dfcbcd0d36a  spikes/netcode-proof/Cargo.toml
c55bab3c93d86c0068f572d9b20d152fe0080a6f29f278617615db04836e6b4d  spikes/netcode-proof/Cargo.lock
e694b595ef02d4e0f01e87df61ed083ba3ae254ab0b46687f6183a11df28a60d  spikes/netcode-proof/crates/server/Cargo.toml
8ba544d368e0e35c509868a233dc0201cf8d4e0c3a19fa217f101724a5759086  spikes/netcode-proof/crates/super/Cargo.toml
90404ee6b66b3884129195ad1d6eaef8a97a023159d4ff2b8f888242d7369ae1  spikes/netcode-proof/deny.toml

# DEP-HOLD release, U8's browser package (main's .claude/CHECKPOINTS.md §3; u8-brief.md rev 1.1 section 5)
# security-auditor's answers, read in full (each ends DONE): .checkpoints/netcode-proof-test/security-r1-deps.md (B5;
# webdriverio 10.2.0 standalone CLEAR WITH CONDITIONS, :241-247; binding check :256-257) and
# .checkpoints/netcode-proof-test/security-u8-deps.md (cleared with conditions: the 167 entries match the registry,
# all published before the 2026-11-15 cut-off; install scripts only edgedriver and geckodriver; npm's 4 moderates are
# GHSA-g3hv-75wg-m249 on minimatch 10.2.6 and the three packages that pull it in, accepted (code never called);
# conditions: the only install is `npm ci --ignore-scripts` with npm_config_ignore_scripts=true, no .npmrc, no npx,
# npm exec or npm create, specs run as `node <script>.mjs`, a later audit on the laptop shows exactly those 4,
# SEC-R1D-2 and -3 in the spec config before any laptop run, no @wdio/cli). Made by `env npm_config_ignore_scripts=true
# npm install --package-lock-only --ignore-scripts --before=2026-11-15`. Written 2026-11-22 ~15:0x UTC.
8370495dee2ed77e6a937c6f40c72a5485015428d57f90505f4e440504293c90  spikes/netcode-proof/ops/browser/package.json
a0efce8e511a8865d50008172b3f88e9cc072efbe92a85a035340562b7c49524  spikes/netcode-proof/ops/browser/package-lock.json

# DEP-HOLD release, U7's Block 0a (web/'s npm files) (main's .claude/CHECKPOINTS.md §3; advisor-u7-brief.md C10)
# security-auditor's answer, read in full (ends DONE): .checkpoints/netcode-proof-test/security-u7-deps.md
# (sha256 3de813ec01f72f8ea6e4ca89adff950d538f6f41bbcd05d7caec75964a39c29c; Block 0a CLEAR WITH CONDITIONS).
# Written 2026-11-23 ~03:3x UTC by the main session. The conditions: item (3)'s commands with `--no-audit --no-fund`;
# item (4)'s substitute (U8's C2: no .npmrc under spikes/, ignore-scripts on every npm command; no
# npm-shrinkwrap.json); item (5)'s OSV in place of npm audit; any difference goes back to security.
# Records: node v22.23.4; npm 10.9.9; `npm config get registry ignore-scripts omit before --prefix <web>`:
# registry=https://registry.npmjs.org/, ignore-scripts=false (set per command), omit= (no dev), before=null.
# Made by `npm_config_ignore_scripts=true npm install --package-lock-only --ignore-scripts --no-audit --no-fund
# --before=2026-11-16 --prefix /home/user/Shamble-Frontlines/spikes/netcode-proof/web` ("up to date in 1s").
# web-lock-check.mjs (sha256 99bbae398d5de67ef15e8cf259aab40f523a6b06c67ee07592b6639d99786d21): 13 PASS, last line
# "/home/user/Shamble-Frontlines/spikes/netcode-proof/web: all PASS".
# OSV at 2026-11-23T03:26:50Z, request {"package":{"ecosystem":"npm","name":"typescript"},"version":"6.0.3"} to
# https://api.osv.dev/v1/query, answer {}. npm audit not run (permission refused); OSV, GHSA-backed, substitutes
# (security-u7-deps.md item (5)); tarball read by security: no install script, no dependency, integrity re-derived.
# `npm_config_ignore_scripts=true npm ci --ignore-scripts --no-audit --no-fund --prefix <web>`: "added 1 package in
# 486ms"; typescript installed 6.0.3; check.mjs --only redact: "PASS redact 11 of 11 tests passed; redact-check: 342
# item(s) checked, 0 finding(s)"; git status: "?? spikes/netcode-proof/web/package-lock.json" and "??
# spikes/netcode-proof/web/package.json" only (node_modules ignored).
8c3b69497de87ad1c3e2fb528914b06700902f59414108ec2732dafc8da76bc0  spikes/netcode-proof/web/package.json
86cdb96a21f50b95a295c8cea8db08ae355d6259160aa822991cc76e9eeaf3fb  spikes/netcode-proof/web/package-lock.json

# DEP-HOLD release, U6b's Block 0 (main's .claude/CHECKPOINTS.md §3; u6b-brief.md rev 1.1 section 5)
# security-auditor's answer, read in full (ends DONE): .checkpoints/netcode-proof-test/security-u6b-deps.md (items 1-8
# CLEAR WITH CONDITIONS; SEC-U6BD-1: seven packages go, not six; binding check :269-307). Written 2026-11-23 ~07:0x UTC
# by the main session after steps 0-6 passed (evidence u6b-block0.md): the lock diff `1 65`, 235 packages, sha256 equal
# to security's model; the feature tree as modelled (ring only, no aws-lc-rs); `cargo check` no warning; check.mjs all 11
# steps PASS (fmt after rustfmt's one hunk), measured 754 (750 + the four wt tests by name), bites 850; cargo-deny
# "advisories ok, bans ok, sources ok" (the same five duplicate warnings); cargo-audit 0 vulnerabilities, one allowed
# warning RUSTSEC-2026-0441; both databases fetched 2026-11-23 07:00 UTC.
7d0f39f2483644b6253b9a71cb2594d58acb491cf7046abeb2f76faffc8e241f  spikes/netcode-proof/Cargo.toml
210bf2df2d968dc457a76a0c75f20f7dd06fd2a0f9c33ef05e5c7c2a3b2ca219  spikes/netcode-proof/Cargo.lock
6d6ede543c1799abfc66b7299a2482a61653ca4766f5d8f494f9b3b9c5da3ea5  spikes/netcode-proof/crates/net/Cargo.toml
3caf76bcc256af569d37910f2eb97cebf4eb9868b03827427acc443b7701403c  spikes/netcode-proof/crates/server/Cargo.toml

# DEP-HOLD release, U6c's Block 0 (main's .claude/CHECKPOINTS.md §3; u6c-brief.md rev 1.1 section 5)
# security-auditor's answers, read in full (each ends DONE): .checkpoints/netcode-proof-test/security-u6c-deps.md
# (items 1-8 CLEAR WITH CONDITIONS; SEC-U6CD-1 to -4 Low; binding check :222-270; Conditions :210-220: Block 0's
# content exactly as item 1 plus SEC-U6CD-2's comment; the issue mode on `Account::builder()` (the platform verifier),
# `builder_with_http` under #[cfg(test)] only, no install_default or set_default, SSL_CERT_FILE and SSL_CERT_DIR unset,
# each ACME call under a timeout; TLS 1.3 only on every server and pinning client configuration; npt-load adds no tokio
# feature and npt-server reaches npt-bots by the dev edge only; cargo-deny 0.22.0 and cargo-audit 0.23.0 with both
# database dates) and .checkpoints/netcode-proof-test/security-u6c-lock.md (the lock refresh: CLEAR WITH CONDITIONS,
# SEC-U6CL-1 to -3 Low; its addendum accepts step 4's three extra feature lines). Written 2026-11-24 ~06:5x UTC by the
# main session after steps 0-6 passed (evidence u6c-block0.md). The refresh took hyper 1.13.0, pinned back by `cargo
# update -p hyper --precise 1.12.0` (step2b-pin.log), which moved three Windows-only edges windows-sys 0.61.2 -> 0.52.0
# (quinn-udp, rustls-platform-verifier, winapi-util). Step 3 as amended: `229 6`, 252 packages, the 17 new checksums as
# cleared, sha256 equal to security's after-lock. Step 4's lists as amended: hyper client, default, http1, http2;
# hyper-util client, client-legacy, default, http1, http2, tokio; hyper-rustls http1, http2, native-tokio, ring,
# rustls-native-certs, rustls-platform-verifier, tls12. `cargo check` no warning; check.mjs all 11 steps PASS, measured
# 807, bites 904 (equal to U6b's merged head); cargo-deny "advisories ok, bans ok, sources ok" (the seven duplicate
# warnings); cargo-audit 252 crate dependencies, 0 vulnerabilities, one allowed warning RUSTSEC-2026-0441; databases
# fetched 2026-11-24 06:33:09 and 06:33:11 UTC.
7992c8e10da940a2f610be7223bcf0d6d3e4c2e31c7b78a61edbe2e88925ca03  spikes/netcode-proof/Cargo.toml
3394b451b06492758621445c250f44e93572e0b33a0269e68b7c9021b0f54873  spikes/netcode-proof/Cargo.lock
a06c090c7ee4b3b2f0fdf4ef9e319943957de549fb291da5aeefd6feb062748f  spikes/netcode-proof/crates/host/Cargo.toml
3cf4b077b8c9f8a440c74759124720ba1f9a30bdda9370c90b7c6a0bad28f45f  spikes/netcode-proof/crates/bots/Cargo.toml

# DEP-HOLD release, U7's Block 0b (main's .claude/CHECKPOINTS.md §3; u7-brief.md rev 1.1 section 5)
# security-auditor's answers, read in full (each ends DONE): .checkpoints/netcode-proof-test/security-u7-deps.md
# (items (7)-(10); Block 0b CLEAR WITH CONDITIONS; its binding check :281-316) and
# .checkpoints/netcode-proof-test/security-u7-0b-answer.md (Q1 (b) the .crate fetched by `cargo info` and its sha256
# checked before the install; Q2 (a) this commit before the CLI; Q3 (b) fetch times only). Written 2026-11-25 11:38 UTC
# by the main session after the binding check's steps 0-6 passed (evidence u7-block0b.md), for commit 2701220: the
# manifests exactly as item (7) (+3; +6 -1); `cargo metadata --format-version 1 --offline` exit 0; the lock +4 -0,
# npt-client-wasm's block only, 252 packages; the five trees as written (wasm-bindgen features default and std only;
# one shipped dependent; 20 names; the three dev-dependencies); `cargo check --workspace --all-targets --locked` 0
# warnings; check.mjs test and deps equal before and after (measured 861, bites 958); cargo-deny 0.22.0 "advisories ok,
# bans ok, sources ok" (the seven duplicate names of U6c's run); cargo-audit 0.23.0, 252 crate dependencies, 0
# vulnerabilities, one allowed warning RUSTSEC-2026-0441; redact 371 items, 0 findings; databases fetched 2026-11-25
# 05:44:45 and 05:44:47 UTC.
# The CLI (step 7): not yet installed; under security-u7-0b-answer.md Q1 (b); recorded in an addendum below after its install
bd1e23353bdcf6606256619217965bcbe3931f508fafc92300372a45417bf59a  spikes/netcode-proof/Cargo.toml
55604a5fae45e50ee0fb262d743bc9e967b7caa3b23482de3b61de4cb28b9f5d  spikes/netcode-proof/Cargo.lock
d8af72f767830e066b360725f200dbdf44b1df77d2c7e0bcb5150661c77c000e  spikes/netcode-proof/crates/client-wasm/Cargo.toml
# Addendum, U7's Block 0b step 7 (the CLI), 2026-11-25 11:45 UTC, by security-u7-0b-answer.md Q1 (b) and Q2
# condition 2 (`#` lines only). One main-run.sh call, log .gate/u7-0b/step7-cli.log: `env CARGO_NET_OFFLINE=false
# cargo info wasm-bindgen-cli@0.2.130 --locked` exit 0 (11:42:15); `sha256sum -c .gate/u7-0b/cli-crate.sha256` OK for
# /root/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-cli-0.2.130.crate, whose sha256 is R1's pin
# c37f6904387d19a319525c74af274c73912f3871c35555aa49f3ea64ebb2fc7a; then `cargo install wasm-bindgen-cli --version
# 0.2.130 --locked --root .gate/u7-0b/cli-root --bin wasm-bindgen` (target .gate/u7-0b/cli-target) exit 0 at
# 11:44:09: 48 crates downloaded, 161 compiled.
# Q1's checks, all before the CLI's first use: (a) `.gate/u7-0b/cli-root/bin/wasm-bindgen --version` prints exactly the
# line "wasm-bindgen 0.2.130", and the bin folder holds wasm-bindgen only. (b) The packaged lock
# (~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-cli-0.2.130/Cargo.lock) has 254 [[package]]
# blocks and no git source; the log has no "Updating git repository" line; all 48 "Downloaded" names and versions are
# in that lock (a print-only scratch script, python3 -I). (c) `cargo audit --file` on that lock, its own runner call
# (log .gate/u7-0b/step7-audit.log, 11:45:13-11:45:17): exit 1, as expected; 254 crate dependencies; the database
# fetched again (1,568 advisories); 1 vulnerability, url 2.5.8 RUSTSEC-2026-0592, and 5 allowed warnings, buf_redux
# RUSTSEC-2023-0028, chrono RUSTSEC-2026-0543, multipart RUSTSEC-2023-0050, safemem RUSTSEC-2023-0081, twoway
# RUSTSEC-2021-0146: R1's six crates only. (d) spikes/netcode-proof/Cargo.lock still 55604a5f...9f5d.
# All four pass: the CLI may be used once this addendum is committed (Q2 condition 3); every workspace cargo call
# stays --locked (Q1 condition 5).

# --- U7's 0b amendment (2026-11-25 23:5x UTC, the main session): two in-repo dev-dependencies of npt-client-wasm,
# npt-link and npt-proto, for R3's room test. Cleared by security-u7-0b-amend-answer.md (sha256
# e8d66e66accf2daac783c694050f0a23d6f5967040d1735888a8758e8bb6fe5a): Q1 yes with conditions 1-7. C6's wording is
# the advisor's re-rule (advisor-main-66-head.md).
# 1. Base hashes on 5da5f7d: Cargo.toml bd1e2335...f59a, Cargo.lock 55604a5f...9f5d, crates/client-wasm/Cargo.toml
#    d8af72f7...000e, all three equal to the answer's.
# 2. The same two Edits (the Edit tool, never cargo): client-wasm Cargo.toml f949f2bf...dc25a, Cargo.lock
#    6e6529a8...00bc; `git diff --numstat` 2 0 for each; `git status --short -- spikes/` those two files only.
# 3. Through main-run.sh (.gate/u7-amend-checks/c345.log; lock 23:54:31 UTC, HEAD 5da5f7d, diff 5f710af8...), every
#    call --locked --offline: (a) `cargo tree -p npt-client-wasm -e normal,dev --depth 1`: normal edges npt-client and
#    wasm-bindgen, dev edges npt-link, npt-proto, npt-room, npt-server and npt-sim, nothing else; exit 0.
#    (b) `cargo tree -p npt-client-wasm -e normal --target wasm32-unknown-unknown --prefix none`: 31 lines, equal to
#    .gate/u7-0b/step4-trees.log :199-229 (the shipped set unchanged); exit 0.
# 4. `cargo deny --locked check bans sources`: exit 0, "bans ok, sources ok"; the same seven duplicate names as Block
#    0b's run (base64, getrandom, pem, syn, synstructure, windows-link, windows-sys); no new warning.
# 5. `node check.mjs --only deps`: PASS (11.3 s), exit 0.
# 7. No difference from the answer: nothing goes back to security.
f949f2bfbffa972d4cc0e9beae5a1ecbc4430cd4f70a9566e4f4306b427dc25a  spikes/netcode-proof/crates/client-wasm/Cargo.toml
6e6529a86d8c7577ad5905a75c5ff8a157f46eed61ae68b1c1fa530eff5500bc  spikes/netcode-proof/Cargo.lock

# UM-4 (2026-11-27 16:0x UTC): ops/browser's real install, um4-plan.md rev 0.3 steps 3-4. No manifest or lockfile
# changes, so DEP-HOLD holds nothing, and this section has no bare sha256 line.
# - The form, as run, in its own call: `env npm_config_ignore_scripts=true npm ci --ignore-scripts --no-audit --no-fund
#   --prefix /home/user/Shamble-Frontlines/spikes/netcode-proof/ops/browser` (the web step's cleared form,
#   security-u7-deps.md items (3)-(4); UM4-7).
# - Before and after, unchanged: package.json sha256 8370495dee2ed77e6a937c6f40c72a5485015428d57f90505f4e440504293c90,
#   package-lock.json sha256 a0efce8e511a8865d50008172b3f88e9cc072efbe92a85a035340562b7c49524; `git status --short`
#   unchanged (the root .gitignore's node_modules/ covers the real folder).
# - npm printed "added 167 packages in 7s" and one deprecation notice (whatwg-encoding@3.1.1). npm audit not run
#   (`--no-audit`, SEC-U7D-3); security's U8 reading of this lock stands (security-u8-deps.md C2, C3 (c)).
# - The count (UM4H-1, in the security read's corrected words, review-um4-security.md:136-138): 167 = the lock's
#   non-root entries (160 distinct name@version and 7 duplicate placements, security-u8-deps.md C0);
#   security-r1-deps.md:158's 162 is a resolver's distinct count that adds two packages npm never places
#   (security-u8-deps.md C1).
# - The tools (SEC-UM4-2): npm 10.9.9, node v22.23.4; registry not recorded at the install (the lock's sha512
#   integrity binds every tarball). Read on 2026-11-27 after the install, not recorded with it.
