// test-engineer: does rustc 1.97.0's rem_euclid return |b| for a tiny negative dividend (exact remainder not representable)?
use std::hint::black_box;
fn main() {
    let x = black_box(-(2f32.powi(-30)));
    let y = black_box(1.0f32);
    let r = x.rem_euclid(y);
    println!("f32 rem_euclid(-2^-30, 1) = {:?} bits={:08x} r==|b|:{} exact 1-2^-30 rounds to 1.0", r, r.to_bits(), r == y.abs());
    let q = x.div_euclid(y);
    println!("f32 div_euclid(-2^-30, 1) = {:?} (exact -1)", q);
    // the D3A-1 pair again, as a cross-check of rr_euclid.out
    let a = black_box(f32::from_bits(0x4040_0001)); let b = black_box(f32::from_bits(0x3f80_0001));
    println!("f32 div_euclid(3+2^-22, 1+2^-23) = {:?} rem_euclid = {:?}", a.div_euclid(b), a.rem_euclid(b));
}
