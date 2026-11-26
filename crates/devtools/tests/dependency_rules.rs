//! Which crate may depend on which, read from the manifests' own text (ADR 0005's foundation line; ADR 0002 part 3).
//!
//! `content` depends on nothing: the reader of the game's content files is all our own code (Mustafa's
//! CONTENT-SOURCE (c), 2026-11-26), and the simulation depends on this crate, so anything `content` depended on
//! would be in the simulation's tree too. The rule names direct dependencies only, so the manifest's text is the
//! whole input: no cargo call, and nothing that needs a package to be on disk.
//!
//! `sim` depends on `content` and on the collision library alone (ADR 0002 part 3; ADR 0004 part 10), and the
//! library's line and three locked versions are the ones Mustafa said yes to: a change to any of them fails here
//! by name, so it reaches him before it reaches a build (security-auditor's finding on the batch: the feature
//! compare stays silent when `std` is turned on in every build alike, and a version bump trips nothing).
//!
//! The content crate's rule is stricter than "no normal dependency": its manifest has three tables, `[package]`,
//! `[dev-dependencies]` and `[lints]`, and the second holds exactly one line, the type writer
//! (`ts-rs = { workspace = true }`: its derive must stand beside the types it reads, as in the protocol crate). A
//! dev-dependency is in no build players get (cargo leaves it out of a build with no test in it), and the derive
//! is written under `cfg(test)`, so it puts no code into the crate itself. That one line came in with the step
//! that writes the content kinds' TypeScript types, after security-auditor's answer on its exact content
//! (2026-11-27: the two files by their digests).

use std::path::Path;

/// The table headers of a manifest, in order, and the keys of its `[package]` table.
fn tables_and_package_keys(manifest: &str) -> (Vec<String>, Vec<String>) {
    let mut tables = Vec::new();
    let mut package_keys = Vec::new();
    for line in manifest.lines() {
        let line = line.trim();
        if let Some(header) = line.strip_prefix('[') {
            tables.push(header.trim_end_matches(']').trim_matches('[').to_owned());
        } else if tables.is_empty() && !line.is_empty() && !line.starts_with('#') {
            // A key before the first table (`dependencies.x = "1"`) is a table in another spelling: named as one.
            tables.push(format!("a line before the first table: {line}"));
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

/// Every table the content crate's manifest may have, and the one line its tests' table holds.
const CONTENT_TABLES: [&str; 3] = ["package", "dev-dependencies", "lints"];
const TYPE_WRITER: &str = "ts-rs = { workspace = true }";

#[test]
fn the_content_crate_depends_on_nothing_and_its_tests_on_the_type_writer_alone() {
    let manifest = manifest_of("content");
    let (tables, package_keys) = tables_and_package_keys(&manifest);
    // A dependency table of any other kind (normal, build, per target), a feature table or a patch is a new
    // table, and fails here by its name.
    assert_eq!(
        tables, CONTENT_TABLES,
        "crates/content/Cargo.toml has a table this rule does not allow"
    );
    assert_eq!(
        table(&manifest, "dev-dependencies"),
        [TYPE_WRITER],
        "the content crate's tests depend on the type writer and on nothing else, with no feature of their own"
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
    // Cargo runs a `build.rs` beside the manifest with no key naming it: nothing of that name is there.
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../content/build.rs");
    assert!(
        std::fs::symlink_metadata(&script).is_err(),
        "crates/content has a build script"
    );
}

/// The lines of one table of a manifest that are neither empty nor comments.
fn table(manifest: &str, name: &str) -> Vec<String> {
    let header = format!("[{name}]");
    let lines = manifest.lines().map(str::trim);
    let body = lines.skip_while(|line| *line != header).skip(1);
    let body = body.take_while(|line| !line.starts_with('['));
    body.filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

/// Every version the lockfile holds of one package.
fn locked(lock: &str, name: &str) -> Vec<String> {
    let start = format!("name = \"{name}\"\nversion = \"");
    let entries = lock
        .split("[[package]]\n")
        .filter_map(|entry| entry.strip_prefix(&start));
    entries
        .filter_map(|rest| rest.split('"').next())
        .map(str::to_owned)
        .collect()
}

/// Whether a manifest asks for the collision library: its name on any line that is not a comment (a dependency's
/// line in any table, or a table of its own).
fn names_library(manifest: &str) -> bool {
    manifest
        .lines()
        .any(|line| !line.trim_start().starts_with('#') && line.contains("parry3d"))
}

/// What a failure says: whose decision the pinned text is.
const HIS_YES: &str = "pinned by Mustafa's FEATURE-COMPARE (a) of 2026-11-26 (docs/DECISIONS.md, 14:09:13 UTC): \"a change of parry3d's version or of the two packages' versions needs a new yes from me; the library's std stays off\". Ask him before this changes";
const LIBRARY: &str = r#"parry3d = { version = "=0.31.1", default-features = false, features = ["dim3", "f32", "alloc", "enhanced-determinism"] }"#;
const PINNED: [(&str, &str); 3] = [
    ("parry3d", "0.31.1"),
    ("safe_arch", "1.2.0"),
    ("thiserror", "2.0.21"),
];

#[test]
fn the_simulation_depends_on_the_content_crate_and_the_collision_library_alone() {
    let manifest = manifest_of("sim");
    let (tables, _) = tables_and_package_keys(&manifest);
    assert_eq!(
        tables,
        ["package", "dependencies", "lints"],
        "crates/sim/Cargo.toml has a table this rule does not allow"
    );
    assert_eq!(
        table(&manifest, "dependencies"),
        [
            "content = { path = \"../content\" }",
            "parry3d = { workspace = true }"
        ],
        "the simulation's dependencies are the content crate and the collision library, and nothing else"
    );
    // The library's one line in the root: its version to the digit, default features off (so no `std`), four features.
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |name: &str| {
        std::fs::read_to_string(root.join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
    };
    let line: Vec<String> = table(&read("Cargo.toml"), "workspace.dependencies");
    let line: Vec<&String> = line
        .iter()
        .filter(|line| line.starts_with("parry3d"))
        .collect();
    assert_eq!(
        line,
        [LIBRARY],
        "the root Cargo.toml's parry3d line: {HIS_YES}"
    );
    let lock = read("Cargo.lock");
    for (name, version) in PINNED {
        assert_eq!(
            locked(&lock, name),
            [version],
            "Cargo.lock's {name}: {HIS_YES}"
        );
    }
    // The library has one asker. Another crate naming it could turn a feature on for every build beside the
    // root's pinned line (cargo joins the features of all who ask), and the simulation would link what it never
    // chose. One such feature, `spade`, compiles the library's one use of `rstar` and with it `heapless` 0.8.0,
    // which has nine advisories (2026-11-18): with `spade` off nothing reaches either, and the lockfile shows it.
    let mut askers = Vec::new();
    for entry in std::fs::read_dir(root.join("crates")).expect("the crates folder") {
        let krate = entry.expect("an entry").file_name();
        let krate = krate.to_string_lossy().into_owned();
        if root
            .join("crates")
            .join(&krate)
            .join("Cargo.toml")
            .is_file()
            && names_library(&manifest_of(&krate))
        {
            askers.push(krate);
        }
    }
    assert_eq!(
        askers,
        ["sim"],
        "only the simulation's manifest names the collision library"
    );
    assert_eq!(
        locked(&lock, "spade"),
        Vec::<String>::new(),
        "Cargo.lock holds spade: the collision library's feature of that name is on, and with it the library's use of rstar and heapless (security-auditor's reading of the advisories, 2026-11-27). {HIS_YES}"
    );
    assert_eq!(
        (locked(&lock, "rstar"), locked(&lock, "heapless")),
        (vec!["0.13.1".to_owned()], vec!["0.8.0".to_owned()]),
        "rstar or heapless moved: read the advisories again (heapless is fixed from 0.9.4 on) and the built-file check's UNLINKED_CRATES"
    );
}

#[test]
fn the_rule_reads_what_it_should_refuse() {
    // The check can fail: each kind of dependency table, and a build script, is seen by name.
    for planted in [
        "[dependencies]\nprotocol = { path = \"../protocol\" }\n",
        "[dev-dependencies]\nts-rs = { workspace = true }\nproptest = \"1\"\n",
        "[dev-dependencies]\nts-rs = { workspace = true, features = [\"serde-compat\"] }\n",
        "[dev-dependencies]\nproptest = \"1\"\n",
        "[dev-dependencies.ts-rs]\nworkspace = true\n",
        "[build-dependencies]\ncc = \"1\"\n",
        "[target.'cfg(unix)'.dependencies]\nlibc = \"0.2\"\n",
        "[features]\nloose = []\n",
    ] {
        let manifest =
            format!("[package]\nname = \"content\"\n\n{planted}\n[lints]\nworkspace = true\n");
        let (tables, _) = tables_and_package_keys(&manifest);
        // Refused by its tables, or (the tests' own table) by a line that is not the type writer's.
        let lines = table(&manifest, "dev-dependencies");
        assert!(
            tables != CONTENT_TABLES || lines != [TYPE_WRITER],
            "{planted}"
        );
    }
    // The one table and line that are allowed pass, so the loop above refuses for a reason.
    let allowed = format!(
        "[package]\nname = \"content\"\n\n[dev-dependencies]\n{TYPE_WRITER}\n\n[lints]\nworkspace = true\n"
    );
    assert_eq!(tables_and_package_keys(&allowed).0, CONTENT_TABLES);
    assert_eq!(table(&allowed, "dev-dependencies"), [TYPE_WRITER]);
    // A dependency spelled as a key before any table is seen too.
    let dotted = "dependencies.protocol = { path = \"../protocol\" }\n[package]\nname = \"content\"\n[lints]\n";
    assert_eq!(tables_and_package_keys(dotted).0.len(), 3);
    let (_, keys) =
        tables_and_package_keys("[package]\nname = \"content\"\nbuild = \"build.rs\"\n");
    assert_eq!(keys, ["name", "build"]);
    // The simulation's half: a third dependency, the library's `std`, and a version that moved are each seen.
    let third = "[package]\nname = \"sim\"\n[dependencies]\n# why\ncontent = { path = \"../content\" }\n\nglam = \"0.33\"\n[lints]\n";
    assert_eq!(
        table(third, "dependencies"),
        ["content = { path = \"../content\" }", "glam = \"0.33\""]
    );
    assert_eq!(table(third, "dev-dependencies"), Vec::<String>::new());
    assert_ne!(LIBRARY.replace("\"alloc\"", "\"alloc\", \"std\""), LIBRARY);
    let lock = "version = 4\n\n[[package]]\nname = \"parry3d\"\nversion = \"0.31.2\"\nsource = \"x\"\n\n[[package]]\nname = \"thiserror\"\nversion = \"1.0.0\"\n\n[[package]]\nname = \"thiserror\"\nversion = \"2.0.21\"\n\n[[package]]\nname = \"thiserror-impl\"\nversion = \"2.0.21\"\n";
    assert_eq!(locked(lock, "parry3d"), ["0.31.2"]);
    assert_eq!(locked(lock, "thiserror"), ["1.0.0", "2.0.21"]);
    assert_eq!(locked(lock, "safe_arch"), Vec::<String>::new());
    // A second asker of the library, in each spelling; a comment that only speaks of it is none.
    for asking in [
        "[dependencies]\nparry3d = { workspace = true, features = [\"spade\"] }\n",
        "[dev-dependencies.parry3d]\nworkspace = true\n",
        "[target.'cfg(unix)'.dependencies]\n  parry3d = \"=0.31.1\"\n",
    ] {
        assert!(names_library(asking), "{asking}");
    }
    assert!(!names_library(
        "# The simulation holds parry3d; this crate does not.\n[dependencies]\nsim = { path = \"../sim\" }\n"
    ));
    assert_eq!(
        locked(
            "[[package]]\nname = \"spade\"\nversion = \"2.15.0\"\n",
            "spade"
        ),
        ["2.15.0"]
    );
}
