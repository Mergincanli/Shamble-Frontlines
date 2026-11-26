//! Which crate may depend on which, read from the manifests' own text (ADR 0005's foundation line; ADR 0002 part 3).
//!
//! `content` depends on nothing: the reader of the game's content files is all our own code (Mustafa's
//! CONTENT-SOURCE (c), 2026-11-26), and the simulation will depend on this crate, so anything `content` depended on
//! would be in the simulation's tree too. The rule names direct dependencies only, so the manifest's text is the
//! whole input: no cargo call, and nothing that needs a package to be on disk.

use std::path::Path;

/// The table headers of a manifest, in order, and the keys of its `[package]` table.
fn tables_and_package_keys(manifest: &str) -> (Vec<String>, Vec<String>) {
    let mut tables = Vec::new();
    let mut package_keys = Vec::new();
    for line in manifest.lines() {
        let line = line.trim();
        if let Some(header) = line.strip_prefix('[') {
            tables.push(header.trim_end_matches(']').trim_matches('[').to_owned());
        } else if tables.last().is_some_and(|table| table == "package")
            && !line.is_empty()
            && !line.starts_with('#')
            && let Some((key, _)) = line.split_once('=')
        {
            package_keys.push(key.trim().to_owned());
        }
    }
    (tables, package_keys)
}

fn manifest_of(krate: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(krate)
        .join("Cargo.toml");
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

#[test]
fn the_content_crate_depends_on_nothing() {
    let (tables, package_keys) = tables_and_package_keys(&manifest_of("content"));
    // Every table the manifest may have. A dependency table of any kind (normal, dev, build, per target), a
    // feature table or a patch is a new table, and fails here by its name.
    assert_eq!(
        tables,
        ["package", "lints"],
        "crates/content/Cargo.toml has a table this rule does not allow"
    );
    // No build script and no native library: neither `build` nor `links` may be set.
    assert_eq!(
        package_keys,
        [
            "name",
            "version",
            "edition.workspace",
            "rust-version.workspace",
            "publish.workspace"
        ],
        "crates/content/Cargo.toml's [package] table has a key this rule does not allow"
    );
}

#[test]
fn the_rule_reads_what_it_should_refuse() {
    // The check can fail: each kind of dependency table, and a build script, is seen by name.
    for planted in [
        "[dependencies]\nprotocol = { path = \"../protocol\" }\n",
        "[dev-dependencies]\nts-rs = { workspace = true }\n",
        "[build-dependencies]\ncc = \"1\"\n",
        "[target.'cfg(unix)'.dependencies]\nlibc = \"0.2\"\n",
        "[features]\nloose = []\n",
    ] {
        let manifest =
            format!("[package]\nname = \"content\"\n\n{planted}\n[lints]\nworkspace = true\n");
        let (tables, _) = tables_and_package_keys(&manifest);
        assert_ne!(tables, ["package", "lints"], "{planted}");
    }
    let (_, keys) =
        tables_and_package_keys("[package]\nname = \"content\"\nbuild = \"build.rs\"\n");
    assert_eq!(keys, ["name", "build"]);
}
