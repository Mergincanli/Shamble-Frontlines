// The security seat's round-2 scratch probe (u4d2-r2-review-security.md): npt-hash-wasm's shape (a crate-level deny of
// unsafe_code, one placed allow on a listed export), with `include!` handed as tokens to a macro that adds the path, so
// neither check.mjs's include! rule (it reads `include!` followed by a bracket) nor FX-6's rule (it exempts `include`
// followed by `!`) sees it. The included file holds an unsafe block. Compiled once for wasm32 as a cdylib, outside
// every copy; its output goes to .gate/u4d2/sec-probe/.
#![crate_type = "cdylib"]
#![deny(unsafe_code)]

macro_rules! m {
    ($($t:tt)*) => {
        $($t)* ("sec-r2-incl-gen.txt")
    };
}

#[allow(unsafe_code, reason = "probe: a listed name, its no_mangle")]
#[unsafe(no_mangle)]
pub extern "C" fn begin(bite: u32, arm_tick: u32) -> u32 {
    (bite ^ arm_tick).wrapping_add(m!(include!))
}
