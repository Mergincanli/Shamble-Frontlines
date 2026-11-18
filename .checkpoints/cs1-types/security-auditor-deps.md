# security-auditor: cs1-types dependency check before the first build (ts-rs =12.0.1 and the nine new Cargo.lock entries)

Rules for this task: no cargo build/test/check/clippy/doc/run; read-only on sources; nothing under spikes/ or uploads; notes only here.

## Plan (blocks, in order)
- B0 Setup: lock diff read; local .crate paths; sha256 of each .crate vs Cargo.lock checksum; cargo tree reach (normal, dev, wasm32) [points 1-local, 6]
- B1 Published data: sparse index lines (cksum, yanked, deps, features, rust-version) for all nine; crates.io owners and repository [point 1-published]
- B2 Advisories: RustSec advisory DB and GitHub advisories for the nine crates/versions [point 3]
- B3 unicode-ident 1.0.27 (licence, build.rs?, contents)
- B4 proc-macro2 1.0.107 (licence, build.rs: what it probes, env vars, files)
- B5 quote 1.0.47 (licence, build.rs?)
- B6 syn 2.0.119 (licence, build.rs?, who asks for it)
- B7 syn 3.0.10 (licence, build.rs?, is it the real syn 3, who asks for it)
- B8 thiserror 2.0.21 (licence, build.rs: probes, env vars, files)
- B9 thiserror-impl 2.0.21 (proc macro: compile-time code path; fs/env/process/net use)
- B10 ts-rs-macros 12.0.1 (proc macro: compile-time code path; env vars at macro time; fs/process/net use)
- B11 ts-rs 12.0.1 (licence, build.rs?, features with default-features=false, runtime API: Config, with_out_dir, with_import_extension, export_all; env vars at run time; what it writes/deletes) [point 5]
- B12 Pin and MSRV notes [point 7]
- B13 Findings + answer draft; DONE

## Notes
### B0 Setup: DONE
- Lock diff (`git diff -- Cargo.lock`): nine new `[[package]]` entries, every `source = "registry+https://github.com/rust-lang/crates.io-index"` (Cargo's id for crates.io, sparse or git). No `~/.cargo/config.toml`, no repo `.cargo/`, no CARGO_REGISTRIES_*/source replacement env: downloads came from default crates.io (index.crates.io + static.crates.io) through the agent proxy (CARGO_HTTP_CAINFO=/root/.ccr/ca-bundle.crt: a TLS-intercepting proxy, so lock checksum and archive came over the same path; independent cross-check in B1).
- sha256sum of each `/root/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f/<name>-<ver>.crate` == Cargo.lock checksum, all nine:
  proc-macro2 1.0.107 985e7ec9...a766d9 | quote 1.0.47 1fbf4db1...d079001 | syn 2.0.119 872831b6...f17297 | syn 3.0.10 4ae9ccbe...faea79 | thiserror 2.0.21 09e52cb8...d189e | thiserror-impl 2.0.21 fe519792...b28524 | ts-rs 12.0.1 75605006...7231d8 | ts-rs-macros 12.0.1 38d90eea...b652aa | unicode-ident 1.0.27 a2c754d6...246563 (full values = Cargo.lock lines; local list in scratchpad local-sha.txt).
- Each archive extracted into its own empty scratch folder: entries all under `<name>-<ver>/`, no symlinks or hardlinks; `diff -r` against `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/<name>-<ver>/` shows only cargo's own `.cargo-ok` marker: the unpacked sources I read ARE the checksummed archives.
- Reach (point 6), `cargo tree --locked --offline` (reads lock + manifests, builds nothing):
  - `--workspace -e normal` (host), `-e normal,build --target all`, `-e normal --target wasm32-unknown-unknown`, `-e normal,build --target wasm32-unknown-unknown`: every one of the six workspace crates shows NO dependency at all. None of the nine is in a shipped graph.
  - `-p protocol -e dev,normal,build`: protocol [dev-dependencies] -> ts-rs 12.0.1 -> { thiserror 2.0.21 -> thiserror-impl 2.0.21 (proc-macro) -> proc-macro2, quote, syn 3.0.10 ; ts-rs-macros 12.0.1 (proc-macro) -> proc-macro2, quote, syn 2.0.119 }; proc-macro2 -> unicode-ident.
  - Inverted: ts-rs only via protocol [dev-dependencies]; syn 3.0.10 asked ONLY by thiserror-impl 2.0.21 (features default = clone-impls, derive, parsing, printing, proc-macro); syn 2.0.119 asked ONLY by ts-rs-macros 12.0.1 (default + "full" + "extra-traits").
  - Features: no `ts-rs feature "default"` node under protocol (default-features=false holds); ts-rs turns on thiserror "default" (= "std") and ts-rs-macros "default".
- Next: B1 published data (sparse index, crates.io API owners).
- Where I am / what is left (12:2x UTC, before compaction): B0 done. Left: B1 (published checksums, yanked, owners), B2 (advisories), B3-B11 (one block per crate: licence, build.rs, proc-macro code, ts-rs API facts), B12 (pin, MSRV), B13 (findings + answer). Nothing built; nothing outside this file written except scratch extracts under the session scratchpad (x/<crate>/).
- Plan addition (coordinator, 12:3x UTC): B14 CI exposure: run 44300471741 (#168), job 134788575344, pushed by checkpoint commit da22f96 on top of 566d1d1 [skip ci]: the `toolchain` workflow built the nine crates (build scripts, thiserror-impl proc macro) before this check. From `.github/workflows/toolchain.yml` (read only): job permissions, checkout token persistence (.git/config), `env -i` + `unshare --net` wrapper, user/group of the step, caches/artifacts that outlive the job; what can persist into later runs or the repo; severity of the process miss. Goes before B13.
### B1 Published data: DONE (index via curl through the agent proxy; crates.io API via curl, User-Agent "dependency-review-script/1.0"; independent WebFetch cross-check noted in B1b)
- Sparse index (https://index.crates.io/{pr/oc/proc-macro2, qu/ot/quote, 3/s/syn, th/is/thiserror, th/is/thiserror-impl, ts/-r/ts-rs, ts/-r/ts-rs-macros, un/ic/unicode-ident}) `cksum` == Cargo.lock checksum == local sha256, all nine; `yanked: false` for all nine. API `checksum` field (https://crates.io/api/v1/crates/<name>/<ver>) equal too, all nine.
- Publisher and owners (API `published_by`, `audit_actions`, `/owners`, and every version's publisher in `/crates/<name>`):
  - proc-macro2 1.0.107 (2026-07-19), quote 1.0.47 (2026-07-19), syn 2.0.119 (2026-07-15), syn 3.0.10 (2026-11-09), thiserror 2.0.21 + thiserror-impl 2.0.21 (2026-09-23), unicode-ident 1.0.27 (2026-10-12): published by dtolnay (David Tolnay); dtolnay is the sole owner and the only publisher of every version ever of these six crates; repos github.com/dtolnay/{proc-macro2,quote,syn,thiserror,unicode-ident}. Each crate is newest = max_stable (no newer version exists today).
  - ts-rs 12.0.1 and ts-rs-macros 12.0.1 (both 2026-01-31, ~17 min after 12.0.0): published by NyxCode, sole owner and only publisher ever of both; repo github.com/Aleph-Alpha/ts-rs. 12.0.1 is newest.
  - No trusted-publishing data on any (trustpub_data null): token-based publishes. Not a finding.
- syn 2.x and 3.x both real: same crate `syn`, same sole owner/publisher. syn 3.0.0 published 2026-07-18 20:35 UTC by dtolnay; thiserror(-impl) 2.0.19 (2026-07-18 22:14 UTC) is the first thiserror-impl to require `syn ^3` (2.0.0-2.0.18 required `^2.0.87`); 2.0.21 requires `^3` -> resolves to 3.0.10. ts-rs-macros 12.0.1 requires `syn ^2.0.28` with features full, extra-traits -> 2.0.119 (last 2.x). Two syn majors = two copies compiled in the test build (time, not risk). Not a typosquat: exact names, canonical owners, 1.7-3.1 billion downloads each.
- Notable: syn 3.0.10 is 9 days old (2026-11-09); thiserror 2.0.21 and syn 3.x only ~2-4 months. Freshness noted, not a finding (canonical publisher, matching checksums).
- Index facts for later blocks: ts-rs 12.0.1 `rust_version` 1.78.0 (index and API; the plan-draft says "MSRV 1.88.0": check in B12); ts-rs-macros no rust_version; ts-rs default = ["serde-compat"] -> ts-rs-macros/serde-compat -> optional termcolor (off with default-features=false; lock has no termcolor); all ts-rs `*-impl` and `format` (dprint-plugin-typescript =0.95) deps optional and off. thiserror default = ["std"]; ts-rs asks thiserror ^2 with default features. ts-rs asks ts-rs-macros `=12.0.1` (exact). thiserror asks thiserror-impl `=2.0.21` (exact).
- Next: B1b WebFetch cross-check (independent path) of key checksums, then B2 advisories.
### B1b Independent cross-check: DONE
- WebFetch (Anthropic-side fetcher, a different network path from the agent proxy that cargo used) of https://crates.io/api/v1/crates/<name>/<ver> for all nine: `checksum`, `yanked: false`, `license`, `created_at`, `published_by.login` equal B1's values character for character (syn 3.0.10 4ae9ccbe...faea79 dtolnay; ts-rs 12.0.1 75605006...7231d8 NyxCode; ts-rs-macros 12.0.1 38d90eea...b652aa NyxCode; thiserror-impl 2.0.21 fe519792...b28524 dtolnay; thiserror 2.0.21 09e52cb8...d189e; proc-macro2 1.0.107 985e7ec9...a766d9; quote 1.0.47 1fbf4db1...d079001; syn 2.0.119 872831b6...f17297; unicode-ident 1.0.27 a2c754d6...246563). So the archives are what crates.io serves; the proxy did not alter them.
- Next: B2 advisories.
### B2 Advisories: DONE
- OSV (aggregates RustSec RUSTSEC-* and GitHub GHSA for crates.io): `POST https://api.osv.dev/v1/querybatch` for the nine exact versions -> `{}` for every one (no vulns); the same query without a version (every version ever) -> `{}` for all eight crate names. Positive control in the same API proves the query shape: time 0.1.43 -> GHSA-wcg3-cvx6-7396 + RUSTSEC-2020-0071; smallvec 1.6.0 -> RUSTSEC-2021-0003; proc-macro-error -> RUSTSEC-2024-0370 (unmaintained notices are included). High confidence: no known advisory (vulnerability, unsound, unmaintained, yanked notice) for any of the nine.
- RustSec site https://rustsec.org/packages/<crate>.html: 404 (no page = no advisory) for all eight crate names (some needed a retry after 503); control `time` = 200.
- GitHub advisory search https://github.com/advisories?query=ecosystem%3Arust+ts-rs (WebFetch): 5 full-text hits, none affecting ts-rs (russh, deno, rustfs, activitypub_federation, sudo-rs).
- Not done: `cargo audit` / `cargo deny` (not installed here; installing them = running new internet code, out of scope). The rustsec/advisory-db repo via gh is not enabled for this session (HTTP 403 from the proxy); OSV + rustsec.org cover it.
- Next: B3 unicode-ident.
