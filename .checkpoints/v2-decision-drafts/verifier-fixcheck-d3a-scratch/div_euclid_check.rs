// d3a: does f32::div_euclid return the rounded infinite-precision result? (local rustc, no fetch)
fn main() {
    let x = 2f32.powi(-23);
    let a: f32 = 3.0 + 2.0 * x; // 3 + 2^-22, representable
    let b: f32 = 1.0 + x;       // 1 + 2^-23, representable
    // exact: a = 2*b + 1.0 with 0 <= 1.0 < b, so the Euclidean quotient is exactly 2
    let q = a.div_euclid(b);
    let r = a.rem_euclid(b);
    println!("a={:e} b={:e} a/b={} div_euclid={} rem_euclid={} q*b+r==a? {}", a, b, a / b, q, r, q * b + r == a);
    println!("check 2*b+1 == a exactly in f64: {}", (2.0f64 * b as f64 + 1.0) == a as f64);
    let a64: f64 = 3.0 + 2.0 * 2f64.powi(-52);
    let b64: f64 = 1.0 + 2f64.powi(-52);
    println!("f64: a/b={} div_euclid={} rem_euclid={} exact quotient 2: 2*b+1==a? {}", a64 / b64, a64.div_euclid(b64), a64.rem_euclid(b64), 2.0 * b64 + 1.0 == a64);
}
