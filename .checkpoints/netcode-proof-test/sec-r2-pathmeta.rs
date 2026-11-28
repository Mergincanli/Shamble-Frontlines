// The security seat's round-2 scratch probe (u4d2-r2-review-security.md): npt-hash-wasm's shape (a crate-level deny of
// unsafe_code, one placed allow on a listed export), with a `#[path]` built from a `meta` fragment: the attribute's
// text is `#[$a]`, and `path = "..."` is a macro argument, not an attribute, so pathProblems (it reads `path =` inside
// attributes) never reads it. The module is declared inside the export's body, so its code sits under the export's
// allow. Its file (sec-r2-pathmeta-mod.txt) is not a .rs file and holds an unsafe block. Compiled once for wasm32 as a
// cdylib, outside every copy; its output goes to .gate/u4d2/sec-probe/.
#![crate_type = "cdylib"]
#![deny(unsafe_code)]

macro_rules! m {
    ($a:meta) => {
        #[$a]
        mod g;
    };
}

#[allow(unsafe_code, reason = "probe: a listed name, its no_mangle")]
#[unsafe(no_mangle)]
pub extern "C" fn begin(bite: u32, arm_tick: u32) -> u32 {
    m!(path = "sec-r2-pathmeta-mod.txt");
    (bite ^ arm_tick).wrapping_add(g::seven())
}
