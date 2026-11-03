// fixcheck-d4: rustc 1.97.0's div_euclid / rem_euclid on part 7's case, f32 and f64, with sign mirrors.
fn main() {
    let a32: f32 = 3.0 + 2f32.powi(-22); let b32: f32 = 1.0 + 2f32.powi(-23);
    for (x, y) in [(a32, b32), (-a32, b32), (a32, -b32), (-a32, -b32)] {
        println!("f32 {:e} / {:e}: a/b={:?} div_euclid={:?} rem_euclid={:e}", x, y, x / y, x.div_euclid(y), x.rem_euclid(y));
    }
    let a64: f64 = 3.0 + 2f64.powi(-51); let b64: f64 = 1.0 + 2f64.powi(-52);
    for (x, y) in [(a64, b64), (-a64, b64), (a64, -b64), (-a64, -b64)] {
        println!("f64 {:e} / {:e}: a/b={:?} div_euclid={:?} rem_euclid={:e}", x, y, x / y, x.div_euclid(y), x.rem_euclid(y));
    }
}
