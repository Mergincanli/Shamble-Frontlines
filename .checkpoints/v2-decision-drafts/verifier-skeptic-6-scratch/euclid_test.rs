// skeptic-6: div_euclid / rem_euclid against the exact Euclidean result (local rustc; reference computed by euclid_ref.py)
use std::hint::black_box;
#[inline(never)] pub fn de32(a: f32, b: f32) -> f32 { a.div_euclid(b) }
#[inline(never)] pub fn re32(a: f32, b: f32) -> f32 { a.rem_euclid(b) }
#[inline(never)] pub fn de64(a: f64, b: f64) -> f64 { a.div_euclid(b) }
#[inline(never)] pub fn re64(a: f64, b: f64) -> f64 { a.rem_euclid(b) }
struct Lcg(u64);
impl Lcg { fn next(&mut self) -> u64 { self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); self.0 >> 11 } }
fn main() {
    // 1. the finding's cases
    let a = 3.0f32 + 2f32.powi(-22); let b = 1.0f32 + 2f32.powi(-23);
    println!("FIX f32 a={:08x} b={:08x} a/b={} div_euclid={} rem_euclid={} 2*b+1==a:{} q*b+r==a:{}",
        a.to_bits(), b.to_bits(), black_box(a) / black_box(b), de32(black_box(a), black_box(b)), re32(a, b),
        2.0 * b + 1.0 == a, de32(a, b) * b + re32(a, b) == a);
    let a = 3.0f64 + 2f64.powi(-51); let b = 1.0f64 + 2f64.powi(-52);
    println!("FIX f64 a={:016x} b={:016x} a/b={} div_euclid={} rem_euclid={} 2*b+1==a:{} q*b+r==a:{}",
        a.to_bits(), b.to_bits(), black_box(a) / black_box(b), de64(black_box(a), black_box(b)), re64(a, b),
        2.0 * b + 1.0 == a, de64(a, b) * b + re64(a, b) == a);
    // negative-dividend and negative-divisor mirrors of the same case
    let a = 3.0f32 + 2f32.powi(-22); let b = 1.0f32 + 2f32.powi(-23);
    for (x, y) in [(a, b), (-a, b), (a, -b), (-a, -b)] {
        println!("MIRROR f32 {:08x} {:08x} {:08x} {:08x}", x.to_bits(), y.to_bits(), de32(x, y).to_bits(), re32(x, y).to_bits());
    }
    // 2. sweep: f32 pairs with quotients in a game-like range (|a/b| up to ~2^12), printed for the exact check
    let mut g = Lcg(0x5348_414d_424c_4500);
    for _ in 0..200000 {
        let ma = (g.next() as u32 & 0x7f_ffff) | 0x80_0000; let mb = (g.next() as u32 & 0x7f_ffff) | 0x80_0000;
        let ea = (g.next() % 13) as i32; let sa = g.next() & 1; let sb = g.next() & 1;
        let x = f32::from_bits(((127 + ea) as u32) << 23 | (ma & 0x7f_ffff)) * if sa == 1 { -1.0 } else { 1.0 };
        let y = f32::from_bits(127u32 << 23 | (mb & 0x7f_ffff)) * if sb == 1 { -1.0 } else { 1.0 };
        println!("S32 {:08x} {:08x} {:08x} {:08x}", x.to_bits(), y.to_bits(), de32(x, y).to_bits(), re32(x, y).to_bits());
    }
    for _ in 0..200000 {
        let ma = g.next() & 0xf_ffff_ffff_ffff; let mb = g.next() & 0xf_ffff_ffff_ffff;
        let ea = (g.next() % 13) as u64; let sa = g.next() & 1; let sb = g.next() & 1;
        let x = f64::from_bits((1023 + ea) << 52 | ma) * if sa == 1 { -1.0 } else { 1.0 };
        let y = f64::from_bits(1023u64 << 52 | mb) * if sb == 1 { -1.0 } else { 1.0 };
        println!("S64 {:016x} {:016x} {:016x} {:016x}", x.to_bits(), y.to_bits(), de64(x, y).to_bits(), re64(x, y).to_bits());
    }
}
