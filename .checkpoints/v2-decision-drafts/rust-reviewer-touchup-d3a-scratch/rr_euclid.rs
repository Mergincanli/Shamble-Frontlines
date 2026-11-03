// rust-reviewer touchup d3a: own check of D3A-1 (div_euclid / rem_euclid), exact reference by integer arithmetic.
use std::hint::black_box;
fn main() {
    // f32: a = 3 + 2^-22, b = 1 + 2^-23. Scale by 2^23: A = 3*2^23 + 2, B = 2^23 + 1 (exact integers).
    let a = f32::from_bits(0x4040_0001);
    let b = f32::from_bits(0x3f80_0001);
    assert_eq!(a, 3.0 + 2f32.powi(-22));
    assert_eq!(b, 1.0 + 2f32.powi(-23));
    let (ai, bi): (i64, i64) = (3 * (1 << 23) + 2, (1 << 23) + 1);
    for (sa, sb) in [(1i64, 1i64), (-1, 1), (1, -1), (-1, -1)] {
        let (x, y) = (black_box(a * sa as f32), black_box(b * sb as f32));
        let (xa, yb) = (ai * sa, bi * sb);
        let q_exact = xa.div_euclid(yb); // integer Euclidean quotient, exact
        let r_exact = xa.rem_euclid(yb);
        let q = x.div_euclid(y);
        let r = x.rem_euclid(y);
        println!(
            "f32 a={:08x} b={:08x} a/b={} div_euclid={} exact_q={} rem_euclid={} exact_r={} (x2^-23) q_ok={} r_ok={}",
            x.to_bits(), y.to_bits(), x / y, q, q_exact, r, r_exact, q == q_exact as f32,
            r == (r_exact as f32) * 2f32.powi(-23)
        );
    }
    // f64: a = 3 + 2^-51, b = 1 + 2^-52. Scale by 2^52.
    let a = f64::from_bits(0x4008_0000_0000_0001);
    let b = f64::from_bits(0x3ff0_0000_0000_0001);
    let (ai, bi): (i128, i128) = (3 * (1i128 << 52) + 2, (1i128 << 52) + 1);
    for (sa, sb) in [(1i128, 1i128), (-1, 1), (1, -1), (-1, -1)] {
        let (x, y) = (black_box(a * sa as f64), black_box(b * sb as f64));
        let (xa, yb) = (ai * sa, bi * sb);
        let q_exact = xa.div_euclid(yb);
        let r_exact = xa.rem_euclid(yb);
        let q = x.div_euclid(y);
        let r = x.rem_euclid(y);
        println!(
            "f64 a={:016x} b={:016x} a/b={} div_euclid={} exact_q={} rem_euclid={:e} exact_r={} (x2^-52) q_ok={} r_ok={}",
            x.to_bits(), y.to_bits(), x / y, q, q_exact, r, r_exact, q == q_exact as f64,
            r == (r_exact as f64) * 2f64.powi(-52)
        );
    }
}
