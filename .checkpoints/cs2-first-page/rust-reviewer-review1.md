# rust-reviewer, review round 1 of cs2-first-page (head 30a5a13)

Started: 2026-11-19 00:29:08 UTC

Plan: read plan.md s5/s7/s16 + risks 1,7 + resume card steps 11,24-30; read ADR 0004 parts 3,4,5,7,8,9,20, ADR 0002 parts 3-5, register OVERFLOW-ALL; read the Rust diff file by file; run cargo fmt --check, clippy native+wasm32 -D warnings, test, release wasm build, cargo tree (all --locked); check the glue in packages/sim-worker/pkg; recompute known answers independently; write each finding here as made.

## Findings (as made)

Checks run (00:34 UTC): cargo fmt --all --check exit 0; clippy --workspace --all-targets --locked -D warnings exit 0 natively and for wasm32; cargo test --workspace --locked exit 0 (client-endpoint 17, sim 9, sim-wasm 6, protocol 2, devtools 1, room 1).

- F1 Medium: packages/client/scripts/build.mjs:27 reads the Wasm from <root>/target/wasm32-unknown-unknown/release/ while the cargo call at :46 writes wherever CARGO_TARGET_DIR or a cargo config's build.target-dir says. The repository's own rule gives worktrees their own CARGO_TARGET_DIR: then wasm-bindgen reads a missing file (loud) or a stale one from an earlier build (silent: glue and Wasm agree with each other, not with the source, and the page is labelled with HEAD's commit). Fix: pass "--target-dir", path.join(root, "target") to cargo (in a worktree that is still its own folder), or take the artifact path from --message-format=json.
- F2 Low: crates/sim-wasm/src/lib.rs:100 takes mode: u8; wasm-bindgen 0.2.130 converts with js as u8 (registry src/convert/impls.rs:361, type_abi_as_u32!(i8 u8 i16 u16) at :394), and the glue passes +mode with no range check, so a host's 256 (or -256, or 0.9) is read as MODE_LOCAL and accepted. Not reachable today (worker.ts:20 passes the constant). Fix: take mode: u32 (same i32 ABI, no cost) and compare the whole value.
