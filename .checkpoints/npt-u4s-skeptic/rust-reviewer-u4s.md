Task: npt-u4s-skeptic — rust-reviewer skeptic check of U4s (f080551 vs 5f61f8f), night 2026-11-10

Started 06:37 Amsterdam; findings due 07:05 (28 min). Notes only.

## Plan (blocks, in order)
B1. Skim prior reviews (arch, netcode, rust, fix, fix2) + carries list — titles only.
B2. Determinism + HitHistory: history.rs, ticks.rs (const asserts), seam.rs float use.
B3. Grid: grid.rs tie rule + edge snap.
B4. wasm32 hazards (usize/u64 casts) across the new files.
B5. Manifests + Cargo.lock edges.
B6. check.mjs new steps + check.test.mjs coverage.
B7. Answer.

## Notes
