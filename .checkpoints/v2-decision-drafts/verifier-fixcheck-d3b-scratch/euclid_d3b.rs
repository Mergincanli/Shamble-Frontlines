// d3b's own check: div_euclid / rem_euclid against an exact reference (integer arithmetic on scaled inputs).
fn exact_euclid(an: i128, bn: i128) -> (i128, i128) {
    // a = an * u, b = bn * u (u = common unit); q = euclid quotient, r = an - q*bn in units of u
    let mut q = an / bn; // truncates toward zero
    let mut r = an - q * bn;
    if r < 0 { if bn > 0 { q -= 1; r += bn; } else { q += 1; r -= bn; } }
    (q, r)
}
fn main() {
    // f32: unit u = 2^-23; a = 3 + 2^-22 = (3*2^23 + 2) u ; b = 1 + 2^-23 = (2^23 + 1) u
    let a32: f32 = 3.0 + (2.0f32).powi(-22);
    let b32: f32 = 1.0 + (2.0f32).powi(-23);
    let (an, bn): (i128, i128) = (3 * (1 << 23) + 2, (1 << 23) + 1);
    assert_eq!(a32 as f64 * (1u64 << 23) as f64, an as f64);
    println!("f32 a={:e} bits={:#x} b={:e} a/b={:.10} trunc(a/b)={}", a32, a32.to_bits(), b32, a32 / b32, (a32 / b32).trunc());
    for (sa, sb) in [(1i128, 1i128), (-1, 1), (1, -1), (-1, -1)] {
        let x = a32 * sa as f32; let y = b32 * sb as f32;
        let (q, r) = exact_euclid(sa * an, sb * bn);
        let rr = r as f64 / (1u64 << 23) as f64;
        println!("f32 ({:+}a,{:+}b): div_euclid={} exact_q={} | rem_euclid={:e} exact_r={:e} | rem==exact {} | a%b={:e}",
            sa, sb, x.div_euclid(y), q, x.rem_euclid(y), rr, (x.rem_euclid(y) as f64) == rr, x % y);
    }
    // f64: unit u = 2^-52; a = 3 + 2^-51 ; b = 1 + 2^-52
    let a64: f64 = 3.0 + (2.0f64).powi(-51);
    let b64: f64 = 1.0 + (2.0f64).powi(-52);
    let (an, bn): (i128, i128) = (3 * (1i128 << 52) + 2, (1i128 << 52) + 1);
    println!("f64 a={:e} b={:e} a/b={:.20} ", a64, b64, a64 / b64);
    for (sa, sb) in [(1i128, 1i128), (-1, 1), (1, -1), (-1, -1)] {
        let x = a64 * sa as f64; let y = b64 * sb as f64;
        let (q, r) = exact_euclid(sa * an, sb * bn);
        let rr = r as f64 / (1u64 << 52) as f64;
        println!("f64 ({:+}a,{:+}b): div_euclid={} exact_q={} | rem_euclid={:e} exact_r={:e} | rem==exact {}",
            sa, sb, x.div_euclid(y), q, x.rem_euclid(y), rr, x.rem_euclid(y) == rr);
    }
    // f32 pair widened to f64 (rust-reviewer's point 2)
    let (x, y) = (a32 as f64, b32 as f64);
    println!("f32 pair in f64: a/b={:.17} div_euclid={}", x / y, x.div_euclid(y));
    // floor remark
    println!("(a/b).floor() f32 = {}", (a32 / b32).floor());
}
