# U6b Block 0: evidence (the main session, 2026-11-23; u6b-brief.md rev 1.1 section 5)
Binding check: security-u6b-deps.md:269-307 (steps 0-7). Run in the main folder on the lane at 096808b's tree (U6a
merged at 824d8e7), before `<base>` is filled. The cut of 03:59-06:36 UTC fell between step 1's manifest edits and its
provider lines (history.md, "Cut by the spend limit (2026-11-23 about 03:59 to 06:36 UTC)").

## Step 0, before: PASS (equal to security-u6b-deps.md:271-273)
5ebfbedfdf3015e1928aa140bbda1585ad39012521c11ede62f40dfcbcd0d36a  spikes/netcode-proof/Cargo.toml
c55bab3c93d86c0068f572d9b20d152fe0080a6f29f278617615db04836e6b4d  spikes/netcode-proof/Cargo.lock
fa2b3c2a6dd410b0a7c0e899c4a09ae6ef3c596d2e60f82227b1999affccf523  spikes/netcode-proof/crates/net/Cargo.toml
e694b595ef02d4e0f01e87df61ed083ba3ae254ab0b46687f6183a11df28a60d  spikes/netcode-proof/crates/server/Cargo.toml
90404ee6b66b3884129195ad1d6eaef8a97a023159d4ff2b8f888242d7369ae1  spikes/netcode-proof/deny.toml
Lock: 242 [[package]] entries. Tools: cargo-deny 0.22.0, cargo-audit 0.23.0 (installed for U6a's Block 0).

## Step 1, the edits (security-u6b-deps.md sections 2 and 3): done
- Cargo.toml: `web-transport-quinn = { version = "=0.13.6", default-features = false }` (was `"=0.13.6"`), with a
  comment; a new `rcgen = { version = "=0.14.10", default-features = false, features = ["ring"] }` with its comment.
- crates/net/Cargo.toml: `default = ["wt-wtq"]` and `bites = []` in [features]; `web-transport-quinn = { workspace =
  true, optional = true, features = ["ring"] }`; `futures-util.workspace = true` moved to [dependencies] (the empty
  [dev-dependencies] table removed).
- crates/server/Cargo.toml: `rcgen.workspace = true` in [dependencies]; `bites` gains `"npt-net/bites"`.
- The provider lines: l2/wt.rs:4-7 (module text: ring for both crates) and :177-179 (`wtq_server::provider()` is
  `rustls::crypto::ring::default_provider()`); l2/wt_tests.rs's wt-wtq block (the aws-lc-rs assertion replaced by
  groups and suites equal to ring's named directly, delay_tests.rs:2765-2775's form); ws_config.rs:4's comment.
- `git diff --stat` touches only the three Cargo.toml files, Cargo.lock (step 2), wt.rs, wt_tests.rs, ws_config.rs.

## Step 2, the lock refreshed offline: done
`cargo metadata --offline --format-version 1` through main-run.sh (lane lock; log .gate/u6b-block0/step2-metadata.log).

## Step 3, the lock diff: PASS
`git diff --numstat -- Cargo.lock`: `1 65`; 235 [[package]] entries; `grep -c -E
'aws-lc|cmake|dunce|fs_extra|pkg-config|jobserver'` 0; sha256 210bf2df2d968dc457a76a0c75f20f7dd06fd2a0f9c33ef05e5c7c2a3b2ca219,
equal to security's model, so the one added line is ` "rcgen",` and the removed lines are the seven blocks and their
array lines as modelled.

## Step 4, the feature tree (`--locked --offline`): PASS
- `-i aws-lc-rs` (workspace, all features): "did not match any packages" (exit 101).
- rustls (npt-server; and workspace all features): features log, logging, ring, std only.
- web-transport-quinn (npt-server): feature `ring` only; quinn-proto: bloom, log, platform-verifier, ring,
  rustls-ring (no aws-lc-rs, no rustls-aws-lc-rs).
- rcgen (npt-server; and workspace all features): crypto and ring only.
- npt-server `-e normal --depth 1`: npt-link, npt-log, npt-net, npt-proto, npt-room, npt-sim, rcgen v0.14.10, serde,
  serde_json, sha2, tokio (no `time`); `-i url`: url v2.5.9 under web-transport-proto and web-transport-quinn only.
- `^default *=` in crates/*/Cargo.toml: npt-net's `default = ["wt-wtq"]` only.
- The grep for `install_default|set_default|ClientBuilder|ServerBuilder|Client::default|\.origin\(\)` over crates/net
  and crates/server: no install_default, no builder, no Client::default, no .origin(); its only hits are `set_default`
  in npt-net's own L2 API (`ImpairSocket::set_default` and its `Impair::set_default`: l2/config.rs:12, impair.rs:714,
  impair_tests.rs:465, socket.rs:27, :251-252, there since U3b), per-socket impairment settings, not rustls's process
  default. Recorded as the pattern's false hit, not a finding.

## Step 5, `cargo check --workspace --all-targets --locked` then `node check.mjs`: PASS
Through main-run.sh after a part-clean (6.2 GiB; free 5,052 MiB before), log .gate/u6b-block0/step5-check.log
(06:40:10-06:59:27 UTC). `cargo check`: exit 0, no warning. check.mjs: every step PASS but fmt, which asked for one hunk
in wt_tests.rs (the `groups` closure on one line); applied by Edit, then `check.mjs --only fmt` PASS. The counts against
U6a's M5 on this tree less Block 0 (.gate/u6a-merge/m5-check.log: measured 750, bites 850): measured 754, bites 850, 0
failed, 0 skipped; test step 954.8 s. The four that join the measured build, by name (`cargo test --locked -p npt-net
-- --list`, .gate/u6b-block0/step5-wt-list.log): l2::wt::tests::the_server_configs_turn_migration_off_and_keep_datagrams_on,
l2::wt::tests::the_test_certificate_is_well_formed, l2::wt::tests::wtq_route::web_transport_quinn_counts_a_failed_handshake,
l2::wt::tests::wtq_route::web_transport_quinn_serves_a_session_over_impair_socket_at_p2 (npt-net's new default turns the
wt module on in the measured build; the bites build had them already).

## Step 6, cargo-deny and cargo-audit: PASS
- `cargo deny --manifest-path spikes/netcode-proof/Cargo.toml --locked check advisories bans sources`: "advisories ok,
  bans ok, sources ok", with the same five duplicate warnings as U6a's Block 0 (getrandom, syn, synstructure,
  windows-link, windows-sys); its database folder updated 2026-11-23 07:00:31 UTC.
- `cargo audit --file spikes/netcode-proof/Cargo.lock`: database fetched from https://github.com/RustSec/advisory-db.git
  (FETCH_HEAD 2026-11-23 07:00:33 UTC), 1,561 advisories, 235 crate dependencies scanned, 0 vulnerabilities, "1 allowed
  warning found": pin-utils 0.1.1, unmaintained, RUSTSEC-2026-0441 (2026-10-01; no fixed upgrade), as U6a's Block 0.

## Step 7, after (dependency-check.md's U6b section; then one commit and the save)
7d0f39f2483644b6253b9a71cb2594d58acb491cf7046abeb2f76faffc8e241f  spikes/netcode-proof/Cargo.toml
210bf2df2d968dc457a76a0c75f20f7dd06fd2a0f9c33ef05e5c7c2a3b2ca219  spikes/netcode-proof/Cargo.lock
6d6ede543c1799abfc66b7299a2482a61653ca4766f5d8f494f9b3b9c5da3ea5  spikes/netcode-proof/crates/net/Cargo.toml
3caf76bcc256af569d37910f2eb97cebf4eb9868b03827427acc443b7701403c  spikes/netcode-proof/crates/server/Cargo.toml
