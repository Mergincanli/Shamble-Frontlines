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
