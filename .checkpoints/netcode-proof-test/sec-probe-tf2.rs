// The security seat's scratch probe (u4d2-review-security.md): sec-probe-tf.rs linked as a wasm32 cdylib with one
// integer export, to see whether the linked module records relaxed-simd in its target_features custom section.
#![crate_type = "cdylib"]
#![deny(unsafe_code)]

#[target_feature(enable = "relaxed-simd")]
fn madd(a: f32, b: f32, c: f32) -> f32 {
    use core::arch::wasm32::{f32x4_extract_lane, f32x4_relaxed_madd, f32x4_splat};
    f32x4_extract_lane::<0>(f32x4_relaxed_madd(f32x4_splat(a), f32x4_splat(b), f32x4_splat(c)))
}

#[allow(unsafe_code, reason = "probe: a listed name, its no_mangle")]
#[unsafe(no_mangle)]
pub extern "C" fn step() -> u32 {
    madd(1.5, 2.0, 0.25).to_bits()
}
