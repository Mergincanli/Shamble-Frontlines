// rust-reviewer's scratch for block 7 (RR2 to RR4): the spawn facing through libm 0.2.16's sincos, built offline from
// the cargo cache. Prints K's bits, the quarter turns both ways, and every remainder's f32 pair for the reference check.
// Not product code. Build: rustc -O --edition 2024 --extern libm=<rlib>; run: <bin> summary | <bin> dump
const K: f64 = core::f64::consts::PI / 180.0;

/// Route A: whole quarter turns out first (three compares), the remainder through sincos in f64, narrowed once.
/// The facing as (x, z): 0 faces +z, 90 faces +x (map.rs:58-62): x = sin, z = cos.
fn facing_a(deg: f64) -> (f32, f32) {
    let (q, r) = if deg >= 270.0 { (3, deg - 270.0) } else if deg >= 180.0 { (2, deg - 180.0) } else if deg >= 90.0 { (1, deg - 90.0) } else { (0, deg) };
    let (s, c) = libm::sincos(r * K);
    let (x, z) = (s as f32, c as f32);
    match q { 0 => (x, z), 1 => (z, -x), 2 => (-x, -z), _ => (-z, x) }
}
/// Route B: no quarter turns out, sincos in f64 of the whole angle, narrowed.
fn facing_b(deg: f64) -> (f32, f32) { let (s, c) = libm::sincos(deg * K); (s as f32, c as f32) }
/// Route C: sincosf on the f32 angle.
fn facing_c(deg: f64) -> (f32, f32) { libm::sincosf((deg as f32) * (K as f32)) }

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    if mode == "dump" {
        // The remainders 0 to 89.999 in 0.001 steps (as the map's millimetre-free degrees parse: k / 1000), and the
        // edges; for each, route A's and route C's (x, z) bits, checked against a correctly rounded reference outside.
        for k in 0..90_000u32 {
            let deg = f64::from(k) / 1000.0;
            let (ax, az) = facing_a(deg);
            let (cx, cz) = facing_c(deg);
            println!("{} {:08x} {:08x} {:08x} {:08x}", deg, ax.to_bits(), az.to_bits(), cx.to_bits(), cz.to_bits());
        }
        return;
    }
    println!("K bits {:016x} (expected 3f91df46a2529d39); f64::to_radians(1.0) bits {:016x}", K.to_bits(), 1.0f64.to_radians().to_bits());
    for d in [0.0, 45.0, 90.0, 120.0, 180.0, 240.0, 270.0, 359.999] {
        let (a, b, c) = (facing_a(d), facing_b(d), facing_c(d));
        println!("deg {d}: route A ({:e}, {:e})  route B ({:e}, {:e})  route C ({:e}, {:e})", a.0, a.1, b.0, b.1, c.0, c.1);
    }
    // Sterbenz: deg - 90 q exact for deg in [90 q, 90 (q + 1)) and q = 1, 2, 3: checked over every 1/1000 degree and
    // 100,000 random doubles in [90, 360).
    let mut bad = 0;
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    for i in 0..(270_000 + 100_000) {
        let deg = if i < 270_000 { 90.0 + f64::from(i) / 1000.0 } else {
            seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17;
            90.0 + (seed >> 11) as f64 / (1u64 << 53) as f64 * 270.0
        };
        let q = if deg >= 270.0 { 270.0 } else if deg >= 180.0 { 180.0 } else { 90.0 };
        let r = deg - q;
        if r + q != deg || r < 0.0 || r >= 90.0 { bad += 1; }
    }
    println!("quarter-turn remainders not exact or out of [0, 90): {bad} of 370000");
    // Unit length of route A's pairs (no normalising), the largest |x^2 + z^2 - 1| in f32 over the dump's domain.
    let mut worst = 0.0f32;
    for k in 0..360_000u32 { let (x, z) = facing_a(f64::from(k) / 1000.0); worst = worst.max((x * x + z * z - 1.0).abs()); }
    println!("route A largest |x^2 + z^2 - 1| over 360,000 facings: {worst:e} (f32 epsilon {:e})", f32::EPSILON);
}
