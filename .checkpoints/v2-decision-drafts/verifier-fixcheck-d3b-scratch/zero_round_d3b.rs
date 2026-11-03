fn main() {
    let t = -(2.0f32).powi(-30);
    println!("(-2^-30).rem_euclid(1.0) = {} bits {:#x}", t.rem_euclid(1.0), t.rem_euclid(1.0).to_bits());
    println!("0.5.div_euclid(-1.0) = {} bits {:#x}", 0.5f32.div_euclid(-1.0), 0.5f32.div_euclid(-1.0).to_bits());
    println!("(-1).rem_euclid(1.0) = {} bits {:#x}", (-1f32).rem_euclid(1.0), (-1f32).rem_euclid(1.0).to_bits());
}
