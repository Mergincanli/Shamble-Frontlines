# security-auditor: dependency answer for cs2-first-page step 2 (notes started 2026-11-18 22:38:51 UTC)
Plan: hash the nine files (start and end) -> read diffs/compare -> per new package: registry checksum/integrity, yanked/deprecated, publisher, licence (crates.io API + registry.npmjs.org, second route via WebFetch) -> build-time code (Rust build.rs / proc-macro read from published .crate in .gate/cs2-first-page/dep-check/; npm scripts/bin from packument + tarball) -> advisories (OSV querybatch with positive control, rustsec pages, GHSA) -> oddities -> 37 flag-only -> six overflow overrides -> tool route 5a vs Mustafa's words -> answer.
## 22:39:06 UTC start hash check
- All nine files in .gate/cs2-deps hash to deps-sha256.txt (diff empty). No target/, no node_modules/; git status shows only the nine modified files, nothing ignored or untracked.
## 22:41:33 UTC R1 crates.io API (route 1: curl, crates.io/api/v1/crates/<name>/<version>, saved in .gate/cs2-first-page/dep-check/meta/)
- Lock checksum == API checksum (full-string compare, script cmp_lock_api.py in session scratchpad sec-deps/) for all 7; none yanked; all MIT OR Apache-2.0.
  bumpalo 3.20.4: published 2026-11-13T19:44Z by fitzgen (Nick Fitzgerald, id 696), crate 99,233 B, no trusted publishing. FIVE DAYS OLD at review.
  cfg-if 1.0.5: 2026-09-16 by rust-lang-owner (id 55123), 9,578 B.
  rustversion 1.0.23: 2026-07-07 by dtolnay (id 3618), 21,013 B.
  wasm-bindgen / -macro / -macro-support / -shared 0.2.130: 2026-10-30T06:14:57-06:15:04Z, published_by null, trustpub github wasm-bindgen/wasm-bindgen run 41011884567 sha c93b8ae848d4d9e6beaeaf9516e18a7cadd818a2 (same run+commit as the CLI crate and the release tag, tool notes S12/S13). wasm-bindgen-shared lib_links "wasm_bindgen". wasm-bindgen features include empty "rustversion" and "msrv" (to read in Cargo.toml).
