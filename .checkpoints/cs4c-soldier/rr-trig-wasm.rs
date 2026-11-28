// rust-reviewer's scratch for block 7 (RR4): which libm names a wasm32 file keeps when it calls sincos and exp once
// each, as the match-start constructor would. Not product code.
#![no_std]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
const K: f64 = core::f64::consts::PI / 180.0;
#[unsafe(no_mangle)]
pub extern "C" fn facing(deg: f64, out: *mut f32) {
    let (q, r) = if deg >= 270.0 { (3, deg - 270.0) } else if deg >= 180.0 { (2, deg - 180.0) } else if deg >= 90.0 { (1, deg - 90.0) } else { (0, deg) };
    let (s, c) = libm::sincos(r * K);
    let (x, z) = (s as f32, c as f32);
    let (a, b) = match q { 0 => (x, z), 1 => (z, 0.0 - x), 2 => (0.0 - x, 0.0 - z), _ => (0.0 - z, x) };
    unsafe { *out = a; *out.add(1) = b; }
}
#[unsafe(no_mangle)]
pub extern "C" fn share(t: f64) -> f32 { (1.0 - libm::exp(-core::f64::consts::LN_10 / 30.0 / t)) as f32 }
