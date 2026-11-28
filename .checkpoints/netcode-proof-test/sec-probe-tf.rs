// The security seat's scratch probe (u4d2-review-security.md): under a crate-level deny of unsafe_code and with no
// rustflag, does a source-level #[target_feature(enable = "relaxed-simd")] on a safe function compile on wasm32, called
// from safe code, and emit relaxed-simd instructions? Emitted as assembly into .gate/u4d2/sec-probe/.
#![crate_type = "lib"]
#![deny(unsafe_code)]

#[cfg(target_arch = "wasm32")]
#[target_feature(enable = "relaxed-simd")]
fn madd(a: f32, b: f32, c: f32) -> f32 {
    use core::arch::wasm32::{f32x4_extract_lane, f32x4_relaxed_madd, f32x4_splat};
    f32x4_extract_lane::<0>(f32x4_relaxed_madd(f32x4_splat(a), f32x4_splat(b), f32x4_splat(c)))
}

#[cfg(target_arch = "wasm32")]
pub fn step(a: f32, b: f32, c: f32) -> f32 {
    madd(a, b, c)
}
