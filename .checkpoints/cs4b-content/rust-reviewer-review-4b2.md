# rust-reviewer: review of change-set 4b-2 final diff, round 1 (code head c442a80), 2026-11-27

## Plan (blocks)
1. File list of the diff (crates/, Cargo.*), manifests + overflow overrides.
2. collision.rs + vec.rs against step-10 conditions (division slab, zero component inclusive, prune only on >, (t,index) ties, narrowing -0.0, depth refusal 31, ramp clamp).
3. sim-wasm lib.rs constructor checks and order; local_match.rs world.
4. devtools replay.rs checks, files.rs, scripts.rs.
5. content schema.rs, ts_export.rs (folder rules); test-only gating.
6. Runs: clippy -D warnings, fmt --check, cargo test (own target), wasm32 build release; panics reachable from content/network.
7. Players' file size: what grew; test-only module/descriptions reached it?; allow-list entries as built (maths-allowed.mjs).
8. Answer, DONE.

## Notes
- Block 1 done: diff stat 58 files under crates/ Cargo.* packages/client/scripts; root Cargo.toml adds parry3d =0.31.1 (dim3,f32,alloc,enhanced-determinism, default-features off) + 29 release overflow overrides; sim deps content+parry3d; sim-wasm +content; content dev-dep ts-rs only; devtools protocol moved to dev-dep. fmt --check exit 0. clippy running (log .gate/rust-reviewer-4b2-clippy.log).
- Block 2 read collision.rs: conditions 1 (division slab, zero-component inclusive, enter from +0.0, comparisons written), 2 (prune only when entry > best: enter_box(reach=best.t) refuses only enter>leave, so equal kept), 3 ((t,index) keep rule :314), 4 (ramp enter clamped >= box entry :179) all HOLD by reading. narrow -0.0 -> +0.0 (:96-101), refusals before tree (:200-219), depth refusal 31 after build (:232-235); tests cover 0/1/2 shapes, face-plane rays, chain 31/32/34.
- [candidate Low/Medium] enter_ramp (:169-170): products in f32 of finite but huge origin/direction (|v| ~1e37+) overflow to inf; inf-inf -> NaN above or rate; NaN falls through every comparison and returns Some(box entry) though the ray may pass over the slope. Also enter_box with tiny direction and limit=INFINITY returns t=inf hit. cast_ray's doc says only "not finite gives no hit". To prove with a scratch copy.
- Block 3 done: sim-wasm create order seed, mode, content::load, map LOCAL_MAP, StaticWorld::build, padding (lib.rs create); errors carry loader/world messages; LocalMatch holds world, no step reads it; step_alloc test now built on the test map's world. ts_export.rs read: folder rules hold (DirEntry::file_type does not follow links; write mode refuses linked folders; panics only in test code).
- [Low] crates/sim-wasm/src/lib.rs test doc "The four checks run in a fixed order, seed, mode, content, padding" above the_checks_run_in_the_order_seed_mode_content_map_padding: six checks (map and world added); stale.
- clippy --workspace --all-targets --locked -D warnings: exit 0 (own target). fmt --check exit 0.
