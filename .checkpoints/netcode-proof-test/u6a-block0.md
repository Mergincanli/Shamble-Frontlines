# U6a Block 0: evidence (the main session, 2026-11-22; u6a-brief.md rev 1.1 section 0.1 condition 1)
Binding check: security-u6a-deps.md:119-137 (steps 1-5), with security-r1-deps.md:249-253 for (a). Run in the main
folder on the lane at 4c3748f's tree (spikes/ unchanged since b4e1f43 outside results/), before `<base>` is filled.

## Before (equal to security-u6a-deps.md:120-121)
5ebfbedfdf3015e1928aa140bbda1585ad39012521c11ede62f40dfcbcd0d36a  spikes/netcode-proof/Cargo.toml
7c551e2b039bc6f45238eddfa3b5e57a2fbfd98af8f3c74439109bf9788eb579  spikes/netcode-proof/Cargo.lock
4a9d75c137428ae7a2ba8f4a1c3d1f6213e419d4193006d3a6b5dcce9fe7a136  spikes/netcode-proof/crates/server/Cargo.toml
2f0c76bd2026f9c4089e79eb481f5268faf2b029a25fb263304f420738616b64  spikes/netcode-proof/crates/super/Cargo.toml
Lock: 242 [[package]] entries.

## Step 1, lock diff (a): PASS
`cargo update --manifest-path spikes/netcode-proof/Cargo.toml -p url --precise 2.5.9` printed "Updating form_urlencoded
v1.2.2 -> v1.2.3" and "Updating url v2.5.8 -> v2.5.9". `git diff -U0` of the lock: exactly the version and checksum
lines of form_urlencoded (1.2.3, afa489199eaffb1211795df1c6236ce1ad0d41a6fa45250add73e3b02420f06e) and url (2.5.9,
f00cd2407bb42c0ef17be88d4cf020f5cceffd121665bac9b8b9493a369b3b1a), both equal to security-r1-deps.md:274-275's index
checksums; nothing else.

## Step 2, the manifest edits (SEC-U6AD-1) and the offline refresh: done
crates/server/Cargo.toml [dependencies] gained exactly `tokio.workspace = true`, `sha2.workspace = true`,
`serde.workspace = true`, `serde_json.workspace = true`; crates/super/Cargo.toml [dependencies] gained exactly
`sha2.workspace = true`; no features, default-features, optional or version key; the workspace Cargo.toml unchanged.
Lock refreshed offline by `cargo tree --offline ... -p npt-super --depth 1` (no network; resolution only).

## Step 3, lock diff (b): PASS
Against the lock after (a): 5 added lines, 0 removed: in npt-server's entry after ` "npt-sim",`: ` "serde",`
` "serde_json",` ` "sha2",` ` "tokio",`; in npt-super's entry after ` "npt-log",`: ` "sha2",`. Still 242 packages; no
version, checksum or source line changed beyond (a); `grep -c -E 'signal-hook-registry|io-uring|digest-io'` 0.

## Step 4, features and links: PASS
`cargo tree --locked --offline -e features -p npt-server -i tokio --depth 1` and the same for `-p npt-net` print the
same set: bytes, default, io-util, libc, macros, mio, net, rt, rt-multi-thread, socket2, sync, time, tokio-macros (the
seven named features plus those they imply; tokio's `default` is empty). `cargo tree --locked --offline -e normal -p
npt-super`: npt-log (serde, serde_json), sha2; no npt-sim, npt-room or npt-proto.

## Step 5, cargo-deny and cargo-audit (security's yes: security-u6a-deps.md section 6)
Tools: `cargo install --locked --version 0.22.0 cargo-deny --target-dir .gate/tool-build-deny` (cargo-deny 0.22.0) and
`cargo install --locked --version 0.23.0 cargo-audit --target-dir .gate/tool-build-audit` (prints "cargo-audit-audit
0.23.0"), one at a time; advisory databases under /root/.cargo (outside the repository).
- `cargo deny --manifest-path spikes/netcode-proof/Cargo.toml --locked check advisories bans sources`: "advisories ok,
  bans FAILED, sources ok". Bans' errors are all `error[wildcard]` on the workspace's own path dependencies (each
  `npt-x.workspace = true` to a root `{ path = "crates/x" }` with no version), pre-existing and in every member; no
  registry crate flagged. Warnings: duplicate getrandom (0.2.17, 0.4.3), syn (2.0.119, 3.0.9), synstructure (0.13.2,
  0.14.0), windows-link (0.2.1, 0.100.0), windows-sys (0.52.0, 0.61.2). pin-utils not reported. Back to
  security-auditor before the save (condition 1): its answer decides the deny.toml line.
- `cargo audit --file spikes/netcode-proof/Cargo.lock`: database fetched from https://github.com/RustSec/advisory-db.git
  on 2026-11-22 about 14:2x UTC, 1,560 advisories loaded; 242 crate dependencies scanned; 0 vulnerabilities; "1 allowed
  warning found", in full: "Crate: pin-utils / Version: 0.1.1 / Warning: unmaintained / Title: `pin-utils` is
  unmaintained / Date: 2026-10-01 / ID: RUSTSEC-2026-0441 / URL: https://rustsec.org/advisories/RUSTSEC-2026-0441 /
  Solution: No fixed upgrade is available!". Clean as security-u6a-deps.md section 6 defines it.

## Step 5, the rerun after security's answer (security-u6a-deps.md section 7): PASS
deny.toml gained `allow-wildcard-paths = true` after `wildcards = "deny"`, committed alone at 6aa78ff. The same
`cargo deny ... --locked check advisories bans sources`: "advisories ok, bans ok, sources ok", exit 0, the same five
duplicate warnings (getrandom, syn, synstructure, windows-link, windows-sys); no wildcard or other error.

## After
90404ee6b66b3884129195ad1d6eaef8a97a023159d4ff2b8f888242d7369ae1  spikes/netcode-proof/deny.toml
5ebfbedfdf3015e1928aa140bbda1585ad39012521c11ede62f40dfcbcd0d36a  spikes/netcode-proof/Cargo.toml
c55bab3c93d86c0068f572d9b20d152fe0080a6f29f278617615db04836e6b4d  spikes/netcode-proof/Cargo.lock
e694b595ef02d4e0f01e87df61ed083ba3ae254ab0b46687f6183a11df28a60d  spikes/netcode-proof/crates/server/Cargo.toml
8ba544d368e0e35c509868a233dc0201cf8d4e0c3a19fa217f101724a5759086  spikes/netcode-proof/crates/super/Cargo.toml
