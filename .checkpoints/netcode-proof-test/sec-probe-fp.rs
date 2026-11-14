// The security seat's scratch probe (u4d2-review-security.md): do `-C llvm-args` float options change codegen?
// Emitted as assembly into .gate/u4d2/sec-probe/ with and without each option, and the two compared.
#![crate_type = "lib"]

#[unsafe(no_mangle)]
pub extern "C" fn is_nan(x: f64) -> bool {
    x != x
}

#[unsafe(no_mangle)]
pub extern "C" fn neg_zero(x: f64) -> f64 {
    x + 0.0
}

#[unsafe(no_mangle)]
pub extern "C" fn mul_add(a: f64, b: f64, c: f64) -> f64 {
    a * b + c
}
