Task: engine-architect review of change-set 4b-2 "The world", round 1, code head c442a806d83dceac6a272b22ad67b7f01de476fa (brief: review-brief-4b2.md, section engine-architect).

## Plan (blocks, in order)
B1. My design notes: design 4.5, block 1.9, block 1.10 (engine-architect-cs4.md, engine-architect-read.md); the rulings' sources (advisor-cutC.md Q2/Q3, advisor-step11.md, step9-order.md, STEP12-GO).
B2. The diff's file list and the crate graph (Cargo manifests; dependency_rules.rs): content, sim, sim-wasm, devtools; who depends on whom.
B3. The boundary's shape: sim-wasm/src/lib.rs (Sim::new, the bundle once at the constructor, per-tick plain numbers), the Worker (packages/sim-worker), the page (packages/client/src/app/content.ts, main), the message caps.
B4. Ruling (3): content.ts against design 4.5; confirm or refuse.
B5. The development route: vite.config.mjs /content.json, build.mjs writeContentBundle, against block 1.9.
B6. The schema writer: crates/content/src/schema.rs, packages/types/src/generated/content/schema/, content-schema.test.mjs, against block 1.10 and ADR 0005 CD-3.
B7. The generated content types (ts_export.rs, packages/types/src/generated/content/): no reader in 4b-2; hidden coupling, dead code.
B8. Collision world ownership (collision.rs, local_match.rs, replay.rs): who owns which state; dependency direction.
B9. Runs: cargo test of dependency_rules (own target, --locked), maybe content crate tests.
B10. Verdict and findings written; DONE.

## Notes (findings and block status)
