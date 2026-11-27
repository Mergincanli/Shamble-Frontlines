//! The native half of the built-file check: this crate's tests are built for the plain processor, without the
//! instruction sets whose code differs from WebAssembly's (Mustafa's MUL-ADD (a), 2026-11-24: "The fused
//! multiply-add stays banned in the simulation"). The players' file is read by the build itself
//! (`packages/client/scripts/build-checks.mjs`); a native build has no such file to read, so the compiler is asked
//! what it was told. A flag from anywhere (a variable, a cargo config file) that turns one of these on fails here.

/// The target features that change what the collision library's tree compiles to: the fused multiply-add, and the
/// wide vector set that comes with it on every processor that has either. AMD's older `fma4` is not asked: the
/// pinned compiler tells a stable build nothing about it (planted, `-Ctarget-feature=+fma4` left `cfg!` false and
/// only warned), so an assertion on it could never fail.
#[test]
fn the_native_build_has_no_fused_multiply_add_and_no_wide_vectors() {
    let features = [
        ("fma", cfg!(target_feature = "fma")),
        ("avx2", cfg!(target_feature = "avx2")),
    ];
    let on: Vec<&str> = features
        .iter()
        .filter(|(_, on)| *on)
        .map(|(name, _)| *name)
        .collect();
    assert!(
        on.is_empty(),
        "the simulation is built with {on:?}: unset RUSTFLAGS, target-cpu and target-feature (MUL-ADD)"
    );
}
