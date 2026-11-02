SUBS = [
("`x.sin()` on an `f32` in `sim` (clippy, or the reviewed text scan if clippy cannot name it);",
 "`x.sin()` on an `f32` in `sim` (clippy, or the reviewed text scan if clippy cannot name it); `x.powi(2)`, `deg.to_radians()` and `a.min(b)` in `sim`, each failing on its own (methods outside ADR 0004 part 7's allow-list, which std marks non-deterministic or which may return either signed zero); `Instant::now()` in `room` and in `client-endpoint` (the clock ban there: part 7);"),
("`+fma`, `-C target-cpu=native` or `relaxed-simd` in the build flags of `sim`, `room`, `sim-wasm` or `server`; controls pass: `Instant::now()` and a `HashMap` in `server`, `.unwrap()` in a `#[test]` |",
 "`+fma`, `-C target-cpu=native` or `relaxed-simd` in the build flags of `sim`, `room`, `client-endpoint`, `sim-wasm` or `server`, and `-ffm` (`--fast-math`) in the wasm-opt step's command line (part 4); controls pass: `Instant::now()` and a `HashMap` in `server`, `.unwrap()` in a `#[test]`, and in `sim` the three min, max and clamp helpers and the allowed `abs` and `fract` |"),
("the draft's part 4 (the build-flag check) |",
 "the draft's part 4 (the build-flag and wasm-opt checks) and part 7 (the float allow-list's complement, the clock ban in four crates); the Rust review of 2026-11-02, findings 1, 2 and 7 |"),
]
