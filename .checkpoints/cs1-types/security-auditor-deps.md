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
