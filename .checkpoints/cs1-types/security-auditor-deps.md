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
