// The security seat's scratch probe (u4d2-review-security.md), compiled once for wasm32 into .gate/u4d2/sec-probe/:
// (1) under a crate-level deny of unsafe_code, a placed allow on an export lets an unsafe block in its body compile
// (so inside npt-hash-wasm's export bodies the scan is the only layer); (2) a module-local `use ... f64 as u32` gives
// an export the source scan reads as `u32` and Wasm types as f64.
#![deny(unsafe_code)]

mod shadow {
    use core::primitive::f64 as u32;

    #[allow(unsafe_code, reason = "probe: a listed name, its no_mangle")]
    #[unsafe(no_mangle)]
    pub extern "C" fn feed(word: u32) -> u32 {
        word * 2.0
    }
}

#[allow(unsafe_code, reason = "probe: a listed name, its no_mangle")]
#[unsafe(no_mangle)]
pub extern "C" fn begin(bite: u32, arm_tick: u32) -> u32 {
    let b = unsafe { core::ptr::read_volatile(&bite) };
    b ^ arm_tick
}
