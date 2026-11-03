// test-engineer: can div_euclid / rem_euclid return -0.0 where the exact Euclidean value is the integer 0?
use std::hint::black_box;
fn main() {
    for (x, y) in [(0.5f32, -1.0f32), (-1.0, 1.0), (-1.0, -1.0), (-0.0, 1.0), (1.0, -1.0)] {
        let (x, y) = (black_box(x), black_box(y));
        let q = x.div_euclid(y); let r = x.rem_euclid(y);
        println!("f32 ({:?}, {:?}): div_euclid {:?} bits {:08x}; rem_euclid {:?} bits {:08x}", x, y, q, q.to_bits(), r, r.to_bits());
    }
}
