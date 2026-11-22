# security-auditor: U6c Block 0 dependency short read (DEP-HOLD)

Reader: security-auditor (Opus 5.5), 2026-11-22. Branch claude/netcode-proof-test-plan-hcky5g.
Inputs: u6c-brief.md rev 1.1 (sha256 8f56f37a...) section 5 and section 0.1 C5; advisor-u6c-brief.md section 4
(:283-316, the eight items). Earlier reads reused: security-r1-deps.md (instant-acme, SEC-R1D-4, binding check
:249-253), security-u6b-deps.md (provider, web-transport-quinn).
No cargo fetch/update/build/install, no npm audit. Sources: the lock, the manifests, ~/.cargo/registry if present,
docs.rs, crates.io, OSV. Scratch under .checkpoints/netcode-proof-test/security-u6c/ (index.mjs: sparse-index reader;
crate-read.mjs: a .crate read into memory; neither writes a file).

## Plan
1. Verify the brief's hash; read section 5, section 0.1 C5, the advisor's section 4 items 1-8.
2. Read the current lock and manifests; the earlier reads' relevant parts.
3. Answer each item by number, with sources.
4. Model the expected lock diff (packages, versions, checksums), feature tree, duplicates.
5. Binding check commands for Block 0; conditions; verdict; DONE.

## Notes (one line per finding, saved as I go)
- Step 1 done. u6c-brief.md sha256 8f56f37abc5a6f1f77845fe894f51685da3dcc36b676a73b66edbca79ca6fb52 (= rev 1.1).
  Section 5 at :474-510; C5 at :102-107; advisor's items at advisor-u6c-brief.md:292-316.
- Lane state (git log; sha256 now): Cargo.toml 5ebfbedf...d36a, Cargo.lock c55bab3c...6b4d, crates/net fa2b3c2a...f523,
  crates/server e694b595...a60d, deny.toml 90404ee6...9ae1 = U6a's Block 0 after-state, the same as U6b's read's
  "before". crates/host/Cargo.toml 9f7a05e9...be74 (npt-log only, host/Cargo.toml:16-18); crates/bots/Cargo.toml
  fa127e4d...64c9 (six npt-* deps, bots/Cargo.toml:20-26). U6a's snapshot (wip/u6a-build.patch) touches no Cargo.toml,
  Cargo.lock, deny.toml or clippy.toml (grep of its diff headers). U6b's Block 0 has not run: U6c's "before" is U6b's
  merged lock (my model in security-u6b-deps.md: 235 packages, sha256 210bf2df...a219).
- Sparse index today (index.crates.io via index.mjs, 2026-11-22 ~20:3x UTC): all 17 still the newest stable of their
  lines, checksums equal security-r1-deps.md:258-273 word for word, none yanked; rust-versions all <= 1.99.1
  (highest 1.85: hyper-util, hyper-rustls, tokio-util). bytes 1.12.2 exists (newer than the locked, pinned 1.12.1).
- Requirements read from the index (deps with features): instant-acme 0.8.5 takes base64 ^0.22 (so the locked 0.22.1,
  not 0.23.1), rcgen ^0.14.2 with df=false features ["pem"], hyper-rustls ^0.27.7 df=false [http1, http2,
  native-tokio, tls12, rustls-platform-verifier], hyper ^1.3.1 [client, http1, http2], hyper-util [client,
  client-legacy, http1, http2, tokio]; its features: ring = [dep:ring, hyper-rustls?/ring, rcgen/ring]. pem 4.0.0 takes
  base64 ^0.23.0 (alloc) and an optional serde_core named by `std`'s weak `serde_core?/std`. async-trait 0.1.92 takes
  syn ^3 (syn 3.0.9 is already locked beside 2.0.119). tokio-util's `codec` = ["libc"] (libc locked). hyper-util's
  base64 ^0.23 is optional under client-proxy (off).
