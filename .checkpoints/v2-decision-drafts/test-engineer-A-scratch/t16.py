SUBS = [
("`navbake` or `server` reachable from `sim-wasm` (each when that crate enters);",
 "`navbake` or `server` reachable from `sim-wasm` (each when that crate enters); wasm-bindgen or a transport crate added to `client-endpoint`; a git dependency in any crate (cargo-deny's `sources` at deny, crates.io only: part 3);"),
("a server-only feature on a maths dependency shared with `sim` (the resolved feature lists from `cargo tree -e features` then differ between the `server` and `sim-wasm` builds); control: proptest as a dev-dependency of `sim` passes |",
 "a server-only feature on a maths dependency shared with `sim` (the resolved feature lists from `cargo tree -e features` then differ between any two of the `server`, `devtools`, `navbake` and `sim-wasm` builds); controls pass: proptest as a dev-dependency of `sim`, and a dev-only feature on `devtools` that touches no `sim` dependency |"),
("the draft's part 3 |",
 "the draft's part 3 (the `client-endpoint` crate; `sources`; the same-feature check across four builds); the engine, security and Rust reviews of 2026-11-02 (EA-8, SA-10, RR-10) |"),
]
