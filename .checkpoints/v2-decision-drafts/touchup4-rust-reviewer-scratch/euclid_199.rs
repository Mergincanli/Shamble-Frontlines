// Own check: rustc 1.99.1's core div_euclid body (core/num/f32.rs 2048-2054, f64.rs 2028-2034, saved by verifier
// fixcheck-d3e), compiled with the local rustc 1.97.0, against that toolchain's std div_euclid.
fn de32(x: f32, rhs: f32) -> f32 { let q = (x / rhs).trunc(); if x % rhs < 0.0 { return if rhs > 0.0 { q - 1.0 } else { q + 1.0 }; } q }
fn de64(x: f64, rhs: f64) -> f64 { let q = (x / rhs).trunc(); if x % rhs < 0.0 { return if rhs > 0.0 { q - 1.0 } else { q + 1.0 }; } q }
fn main() {
    let (a, b) = (3.0f32 + 2f32.powi(-22), 1.0f32 + 2f32.powi(-23));
    for (x, y) in [(a, b), (-a, b), (a, -b), (-a, -b)] {
        println!("f32 x={x:e} y={y:e} 1.99.1-steps={} std-1.97.0={} rem={}", de32(x, y), x.div_euclid(y), x.rem_euclid(y));
    }
    let (a, b) = (3.0f64 + 2f64.powi(-51), 1.0f64 + 2f64.powi(-52));
    for (x, y) in [(a, b), (-a, b), (a, -b), (-a, -b)] {
        println!("f64 x={x:e} y={y:e} 1.99.1-steps={} std-1.97.0={} rem={}", de64(x, y), x.div_euclid(y), x.rem_euclid(y));
    }
    println!("f32 (-2^-30).rem_euclid(1.0) = {}", (-(2f32.powi(-30))).rem_euclid(1.0));
}
