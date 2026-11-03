# touchup4 rust-reviewer fixes (ADR 0004 part 7)

One block for both findings: D3E-1's clause pushes words through every later line of part 7's paragraph, so its
re-wrap (lines 272-290, 19 lines in, 19 out) also carries D3B-2's line. Words changed (word diff, nothing else moves):
"1.97.0's `div_euclid` gives" -> "1.97.0's and the pinned 1.99.1's `div_euclid` give"; "as does its `f64` analogue" ->
"as do their `f64` analogues"; the citation gains "and for 1.99.1 fixcheck-d3e"; "an exact reference" -> "a rounded
exact reference" (D3B-2's smallest fix). Sources: verifier-fixcheck-d3e-verdicts.md, G1 fact 2 rows 2d-2e (1.99.1
core/num/f32.rs 2048-2054, f64.rs 2028-2034; std delegates, std/num/f32.rs 253-254, f64.rs 253-255; excerpts W4);
verifier-fixcheck-d3b-verdicts.md:105-108 (D3B-2); PM-0004-22 (0004:1127, "a correctly rounded Euclidean reference, the
exact quotient and remainder ... rounded once to the type"). Own checks: touchup4-rust-reviewer-scratch/euclid_own.out
and euclid_199.out. The pin is ADR 0002 part 2 (0002:155), not part 4.

### FIX D3B-2+D3E-1
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   (1.99.1's, read by the verifier on 2026-11-02, and 1.97.0's), yet 1.97.0's `div_euclid` gives 3, not 2, for
   (3 + 2^-22) ÷ (1 + 2^-23) in `f32`, as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03). A
   shared error passes a same-bits check, all determinism needs; correct rounding is a separate claim, so both may join
   only once fixed inputs, those cases and their sign mirrors included, equal an exact reference on every target
   (PM-0004-22, extended); no source read states `%`'s precision (the Rust Reference defines it only by truncating
   division and the dividend's sign), and review, not the lint, catches the operator. Transcendental maths only
   through the pure-Rust `libm` crate, pinned; no std `sin`, `cos`, `atan2`, `powf` or `exp`, no `algebraic_*`, no
   fast-math, no `relaxed-simd`; Wasm `simd128` off until the suite passes with it on. NaN never enters sim state: every
   input and data value is checked finite where it enters, and debug and test builds assert finite state each tick. No
   sim value is computed in TypeScript. Fused multiply-add across targets: Rust never fuses on its own, so the risk is a
   dependency's SIMD path (wide's `mul_add` fuses only where the target has FMA); the server therefore builds for
   baseline x86-64, never with `+fma` or `-C target-cpu=native`, glam's `scalar-math` is on if the sim uses glam itself,
   and native AArch64 joins the native-against-Wasm hash test before ADR 0006 may pick ARM hosts (the build flags are
   ADR 0002's). parry3d's own `enhanced-determinism` flag is on for both targets (critic rank 3). Attribute maths
   (ADR 0005's data model) runs in f32 under these rules with a fixed, documented aggregation order per attribute
   (R1-ABL-OQ5). An exactly representable `dt` does not matter for identity: `dt` is one constant with the same bits on
   every target, and timers count integer units (part 4); whether 64 Hz is worth it is R1-TICK-OQ1, ADR 0001's
   (R1-XU-OQ11). Whether browsers run a different Wasm float profile matters only if NaN can enter state, which these
   rules forbid; the suite's browser runs check it (R1-DET-OQ4).
AFTER:
   (1.99.1's, read by the verifier on 2026-11-02, and 1.97.0's), yet 1.97.0's and the pinned 1.99.1's `div_euclid` give
   3, not 2, for (3 + 2^-22) ÷ (1 + 2^-23) in `f32`, as do their `f64` analogues (verifiers fixcheck-d3a and skeptic-6,
   and for 1.99.1 fixcheck-d3e, 2026-11-03). A shared error passes a same-bits check, all determinism needs; correct
   rounding is a separate claim, so both may join only once fixed inputs, those cases and their sign mirrors included,
   equal a rounded exact reference on every target (PM-0004-22, extended); no source read states `%`'s precision (the
   Rust Reference defines it only by truncating division and the dividend's sign), and review, not the lint, catches
   the operator. Transcendental maths only through the pure-Rust `libm` crate, pinned; no std `sin`, `cos`, `atan2`,
   `powf` or `exp`, no `algebraic_*`, no fast-math, no `relaxed-simd`; Wasm `simd128` off until the suite passes
   with it on. NaN never enters sim state: every input and data value is checked finite where it enters, and debug
   and test builds assert finite state each tick. No sim value is computed in TypeScript. Fused multiply-add across
   targets: Rust never fuses on its own, so the risk is a dependency's SIMD path (wide's `mul_add` fuses only where
   the target has FMA); the server therefore builds for baseline x86-64, never with `+fma` or `-C target-cpu=native`,
   glam's `scalar-math` is on if the sim uses glam itself, and native AArch64 joins the native-against-Wasm hash
   test before ADR 0006 may pick ARM hosts (the build flags are ADR 0002's). parry3d's own `enhanced-determinism` flag
   is on for both targets (critic rank 3). Attribute maths (ADR 0005's data model) runs in f32 under these rules with
   a fixed, documented aggregation order per attribute (R1-ABL-OQ5). An exactly representable `dt` does not matter for
   identity: `dt` is one constant with the same bits on every target, and timers count integer units (part 4); whether
   64 Hz is worth it is R1-TICK-OQ1, ADR 0001's (R1-XU-OQ11). Whether browsers run a different Wasm float profile
   matters only if NaN can enter state, which these rules forbid; the suite's browser runs check it (R1-DET-OQ4).
END
