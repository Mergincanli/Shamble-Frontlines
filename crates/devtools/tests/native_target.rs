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
        "the tools crate is built with {on:?}: unset RUSTFLAGS, target-cpu and target-feature (MUL-ADD)"
    );
}

/// No file of the repository hands the compiler a flag. A cargo config file (`.cargo/config.toml`) sets compiler
/// flags, a profile or an environment for every build with no word in a manifest; the players' build gives cargo
/// its own flags and would not take a config file's, the native tests would. None is committed, beside the
/// workspace or beside a crate, and no manifest names a compiler flag.
#[test]
fn the_repository_holds_no_cargo_config_and_no_manifest_names_a_compiler_flag() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut crates: Vec<std::path::PathBuf> = std::fs::read_dir(root.join("crates"))
        .expect("the crates folder")
        .map(|entry| entry.expect("an entry").path())
        .filter(|path| path.join("Cargo.toml").is_file())
        .collect();
    crates.sort();
    // A scan of the wrong folder would find nothing and pass.
    assert!(
        crates.len() >= 7 && crates.iter().any(|path| path.ends_with("sim")),
        "not the workspace's crates: {crates:?}"
    );
    let mut folders = vec![root.clone(), root.join("crates"), root.join("packages")];
    folders.extend(crates.iter().cloned());
    for folder in &folders {
        assert!(
            std::fs::symlink_metadata(folder.join(".cargo")).is_err(),
            "{} holds a .cargo folder: a config file there changes every build with no word in a manifest",
            folder.display()
        );
    }
    let mut manifests = vec![root.join("Cargo.toml"), root.join("rust-toolchain.toml")];
    manifests.extend(crates.iter().map(|path| path.join("Cargo.toml")));
    for manifest in &manifests {
        let text = std::fs::read_to_string(manifest)
            .unwrap_or_else(|error| panic!("{}: {error}", manifest.display()));
        for word in ["rustflags", "target-cpu", "target-feature"] {
            assert!(
                !text.contains(word),
                "{} names `{word}`: the builds take no compiler flag from the repository",
                manifest.display()
            );
        }
    }
}
