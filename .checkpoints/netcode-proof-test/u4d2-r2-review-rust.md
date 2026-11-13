# U4d2 review round 2: rust-reviewer (notes)
Seat: rust-reviewer, read only. Log prefix rr (.gate/u4d2/r2-rr-*.log). Copy: .claude/worktrees/u4d2-build (runner only).
Round 1's tree: .gate/u4d2-r1. Read copy: .gate/u4d2-r2. Judged against: advisor-u4d2-r1.md Blocks 1-8 (packet).

## Plan
1. Read the packet (u4d2-r2-packet.md) and its tests side file; my round-1 notes (u4d2-review-rust.md).
2. FX-1: cargo's rustflags sources and precedence (Cargo book config.html build.rustflags, environment-variables.html;
   cargo source in the registry/toolchain if present): CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, target.<triple>.rustflags,
   target.<cfg>.rustflags, build.rustflags, CARGO_BUILD_RUSTFLAGS, CARGO_TARGET_<TRIPLE>_RUSTFLAGS, profile.rustflags
   (unstable), host.rustflags, --config CLI, the configs cargo reads (cwd ancestors, CARGO_HOME); the encoded form
   (0x1f-separated); read rustflagSources and buildInputProblems; does "every source with a flag is refused" cover each;
   is the read-back now given exactly what cargo gives rustc. Also build.target forms (CARGO_BUILD_TARGET, build.target,
   `target` in a config as array). Run r1-rr-superset.log's first environment: deps must FAIL naming both sources.
3. FX-2: loader.mjs targetFeatures: LEB128 (overlong, past end, 32-bit edge: 5th byte > 0x0f), the prefixes, missing
   or duplicated section, trailing bytes, count larger than possible; run it on both built modules (measured, bites)
   and on hand-built edge cases (a scratch .mjs under my notes folder, run by the runner).
4. FX-8: hash.rs digest_text, arm_for, compare's `on` default; panics, `as` casts, the unit test (could it pass for the
   wrong reason?); h6's test following.
5. FX-12: crates/hash-wasm/clippy.toml: clippy host measured/bites and wasm32 measured/bites as check.mjs runs it; does
   the ban bite (a scratch check of clippy reading the file: plant-free method, e.g. clippy's config parse error on a
   malformed path? read only, so only reading/clippy's own behaviour).
6. FX-11: Cargo.toml libm comment vs libm 0.2.16 registry source (configure.rs, arch/x86.rs, arch/x86/fma.rs).
7. Bounds: no new public Rust item, no manifest/Cargo.lock change (cmp vs .gate/u4d2-r1), lib.rs doc line only.
8. Runs (runner only): clippy both builds and wasm32, fmt --check, hash-wasm and lab tests, deps step with r1 env.
9. Verdict.

## Progress
