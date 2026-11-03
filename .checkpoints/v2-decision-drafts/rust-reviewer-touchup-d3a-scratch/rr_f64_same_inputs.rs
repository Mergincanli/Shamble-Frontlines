// Does f64 div_euclid err on the f32 case's values (3 + 2^-22, 1 + 2^-23)? Expect no: the quotient is far from 3 in f64.
use std::hint::black_box;
fn main() {
    let a = black_box(3.0f64 + 2f64.powi(-22));
    let b = black_box(1.0f64 + 2f64.powi(-23));
    println!("f64 same inputs: a/b={:e} div_euclid={} rem_euclid={:e} (exact q 2, r 1)", a / b, a.div_euclid(b), a.rem_euclid(b));
}
