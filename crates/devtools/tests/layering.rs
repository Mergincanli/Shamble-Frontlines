//! Layering: the room's and the client endpoint's code never names `content` (engine-architect's N26; change-set 4c's
//! dependency batch, F2, worded by security-auditor's third answer and its amendment). Both crates take the collision
//! world from the constructor that loads the content, through the simulation's own `StaticWorld::build`, so the
//! authority's path has one reader of content. Their tests may build a world from a map document: `content` is their
//! dev-dependency, which cargo keeps out of every build players get (this test is the readable second guard).
//!
//! Three halves. The manifests: no table but `[dev-dependencies]` names `content`, and that table holds at most the
//! batch's one line. The sources: every `.rs` under the two crates' `src/`, walked whole, holds no token `content` in
//! code the game runs (read with the narrow marking of `crates/sim/tests/support/marking.rs`, the one the content
//! crate's panic scan uses), and brings in no file the walk does not read (`#[path]`, `include!`, `include_bytes!`,
//! `include_str!`). The simulation: what it hands on of `content` with `pub use` is one constant, the time unit.

use std::path::{Path, PathBuf};

#[path = "../../sim/tests/support/marking.rs"]
mod marking;

/// The two crates whose code never names `content`.
const NEVER_NAMING: [&str; 2] = ["room", "client-endpoint"];
/// The one line of `content` their manifests may hold, in their tests' table: the batch's.
const DEV_LINE: &str = "content = { path = \"../content\" }";
/// The crates whose code never calls libm's routines as the simulation reaches them (rust-reviewer's block 17, Q1):
/// only the simulation's own `fmath.rs` and the vectors (devtools) call `exp_of` and `sincos_of`, so a call from the
/// room, the endpoint or the players' module is refused here, test code included (inside the simulation the rule
/// scan holds it).
const NEVER_CALLING: [&str; 3] = ["room", "client-endpoint", "sim-wasm"];
const RAW_NAMES: [&str; 2] = ["exp_of", "sincos_of"];
/// What the simulation hands on of `content` today (crates/sim/src/time.rs): one constant.
const HANDED_ON: [&str; 1] = ["time.rs: pub use content :: time :: UNITS_PER_SECOND ;"];

fn crates() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// Every `.rs` file under a folder, recursively, by its path below that folder, in order.
fn rust_files(folder: &Path, below: &Path, out: &mut Vec<(String, PathBuf)>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(folder)
        .unwrap_or_else(|error| panic!("{}: {error}", folder.display()))
        .map(|entry| entry.expect("an entry").path())
        .collect();
    entries.sort();
    for path in entries {
        let name = below.join(path.file_name().expect("a name"));
        if path.is_dir() {
            rust_files(&path, &name, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push((name.display().to_string(), path));
        }
    }
}

/// Each line of a manifest that names `content` outside its tests' table, or more than the batch's line there.
fn manifest_breaches(manifest: &str) -> Vec<String> {
    let mut table = String::new();
    let mut found = Vec::new();
    for (index, line) in manifest.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            table = line.trim_matches(|c| c == '[' || c == ']').to_owned();
            if table.contains("content") {
                found.push(format!(
                    "line {}: the table [{table}] names content",
                    index + 1
                ));
            }
        } else if line.contains("content") && !(table == "dev-dependencies" && line == DEV_LINE) {
            found.push(format!(
                "line {}: [{table}] names content: {line}",
                index + 1
            ));
        }
    }
    found
}

/// Each token `content` (a raw one too) in code the game runs, and each way of bringing in a file the walk does
/// not read, as "file:line: what".
fn source_breaches(file: &str, source: &str) -> Vec<String> {
    let t = marking::tokens(source);
    let tests = marking::test_only(&t);
    let text = |at: Option<usize>| {
        at.and_then(|at| t.get(at))
            .map_or("", |token| token.text.as_str())
    };
    let mut found = Vec::new();
    for (i, token) in t.iter().enumerate() {
        if tests[i] {
            continue;
        }
        let (before, next) = (text(i.checked_sub(1)), text(Some(i + 1)));
        let name = token.text.as_str();
        if name == "content" {
            found.push(format!(
                "{file}:{}: `content` is named in code the game runs",
                token.line
            ));
        } else if matches!(name, "include" | "include_bytes" | "include_str") && next == "!" {
            found.push(format!(
                "{file}:{}: `{name}!` brings in a file this walk does not read",
                token.line
            ));
        } else if name == "path" && next == "=" && matches!(before, "[" | "(" | ",") {
            found.push(format!(
                "{file}:{}: `#[path]` brings in a file this walk does not read",
                token.line
            ));
        }
    }
    found
}

/// Each `pub use` and `pub extern crate` outside the tests that names `content`, as "file: its words".
fn handed_on(file: &str, source: &str) -> Vec<String> {
    let t = marking::tokens(source);
    let tests = marking::test_only(&t);
    let mut found = Vec::new();
    for (i, token) in t.iter().enumerate().skip(1) {
        let starts = !token.raw && matches!(token.text.as_str(), "use" | "extern");
        if !starts || tests[i] || t[i - 1].text != "pub" || t[i - 1].raw {
            continue;
        }
        let end = (i..t.len())
            .find(|&at| t[at].text == ";")
            .unwrap_or(t.len() - 1);
        if t[i..=end].iter().any(|token| token.text == "content") {
            let words: Vec<&str> = t[i - 1..=end]
                .iter()
                .map(|token| token.text.as_str())
                .collect();
            found.push(format!("{file}: {}", words.join(" ")));
        }
    }
    found
}

#[test]
fn the_room_and_the_endpoint_never_name_content_in_code_the_game_runs() {
    for krate in NEVER_NAMING {
        let folder = crates().join(krate);
        assert_eq!(
            manifest_breaches(&read(&folder.join("Cargo.toml"))),
            Vec::<String>::new(),
            "crates/{krate}/Cargo.toml"
        );
        let mut files = Vec::new();
        rust_files(&folder.join("src"), Path::new(""), &mut files);
        // A walk of the wrong folder would find nothing and pass.
        assert!(
            files.iter().any(|(name, _)| name == "lib.rs"),
            "crates/{krate}/src: {files:?}"
        );
        let found: Vec<String> = files
            .iter()
            .flat_map(|(name, path)| source_breaches(&format!("{krate}/src/{name}"), &read(path)))
            .collect();
        assert_eq!(
            found,
            Vec::<String>::new(),
            "crates/{krate}/src names content"
        );
    }
}

/// Each token `exp_of` or `sincos_of` (a raw one too), test code included, as "file:line: what".
fn raw_breaches(file: &str, source: &str) -> Vec<String> {
    marking::tokens(source)
        .iter()
        .filter(|token| RAW_NAMES.contains(&token.text.as_str()))
        .map(|token| {
            format!(
                "{file}:{}: `{}` is libm's routine as the simulation reaches it: only fmath.rs and the vectors call it",
                token.line, token.text
            )
        })
        .collect()
}

#[test]
fn the_room_the_endpoint_and_the_players_module_never_call_libm_s_routines() {
    for krate in NEVER_CALLING {
        let mut files = Vec::new();
        rust_files(&crates().join(krate).join("src"), Path::new(""), &mut files);
        assert!(
            files.iter().any(|(name, _)| name == "lib.rs"),
            "crates/{krate}/src: {files:?}"
        );
        let found: Vec<String> = files
            .iter()
            .flat_map(|(name, path)| raw_breaches(&format!("{krate}/src/{name}"), &read(path)))
            .collect();
        assert_eq!(
            found,
            Vec::<String>::new(),
            "crates/{krate}/src calls libm's routines"
        );
    }
    // Its plant: one call in a copy of a room source, and one in the players' module; a longer name passes.
    for file in ["room/src/lib.rs", "sim-wasm/src/lib.rs"] {
        let copy = read(&crates().join(file)) + "\nfn f() -> f64 { sim::fmath::exp_of(1.0) }\n";
        assert_eq!(
            raw_breaches(file, &copy).len(),
            1,
            "{file} with the planted call"
        );
    }
    assert_eq!(
        raw_breaches(
            "control.rs",
            "fn f(exp_offset: u32, my_sincos_of: u32) -> u32 { exp_offset }"
        ),
        Vec::<String>::new()
    );
}

#[test]
fn the_simulation_hands_on_one_constant_of_content() {
    let mut files = Vec::new();
    rust_files(&crates().join("sim/src"), Path::new(""), &mut files);
    assert!(
        files.iter().any(|(name, _)| name == "lib.rs"),
        "crates/sim/src: {files:?}"
    );
    let found: Vec<String> = files
        .iter()
        .flat_map(|(name, path)| handed_on(name, &read(path)))
        .collect();
    assert_eq!(found, HANDED_ON, "the simulation hands on what of content");
}

#[test]
fn each_planted_naming_of_content_is_found_and_the_allowed_ones_pass() {
    // T-G: the line in a copy of a room source and of an endpoint source.
    let line = "\nuse content::map::MapDocument;\n";
    for file in ["room/src/lib.rs", "client-endpoint/src/endpoint.rs"] {
        let copy = read(&crates().join(file)) + line;
        assert_eq!(
            source_breaches(file, &copy).len(),
            1,
            "{file} with the planted line"
        );
    }
    #[rustfmt::skip]
    let bites: [&str; 19] = [
        "use content::map::MapDocument;",
        "#[cfg(any(test, feature = \"x\"))]\nuse content::map::MapDocument;",
        "#[cfg(not(test))]\nuse content::map::MapDocument;",
        "#[cfg_attr(test, allow(unused))]\nuse content::map::MapDocument;",
        "extern crate content as c;",
        "use ::content as c;",
        "use r#content::map::MapDocument;",
        // F4's leaks: a test-only mark in a field list, a parameter list and a generic list hides nothing.
        "struct S { #[cfg(test)] r#type: u8, m: content::map::MapDocument }",
        "fn f(#[cfg(test)] r#fn: u8) { content::x(); }",
        "fn f<#[cfg(test)] const N: usize>() { content::x(); }",
        "fn f<#[allow(unused)] #[cfg(test)] const N: usize>() { content::x(); }",
        // A macro may drop the attribute: a mark inside a macro's delimiters marks nothing.
        "strip! { #[cfg(test)] mod leak { use content::map::MapDocument; } }",
        // `#[cfg(test)]` on an item that is no module is read as shipped (the ruling: it moves into the tests module).
        "#[cfg(test)] use content::map::MapDocument;",
        "#[cfg(test)] fn h() { content::x(); }",
        // Delimiters in literals and comments open nothing: the module ends at its own brace.
        "#[cfg(test)] mod tests { const A: char = '{'; const B: &str = \"{\"; const C: &str = r#\"{\"#; /* { */ }\nuse content::map::MapDocument;",
        // A test module inside a shipped module is read as shipped (a false red, accepted: none exists).
        "mod m { #[cfg(test)] mod tests { use content::map::MapDocument; } }",
        "#[path = \"../x.rs\"] mod x;",
        "include!(\"../x.rs\");",
        "const B: &[u8] = include_bytes!(\"x\");",
    ];
    for source in bites {
        let found = source_breaches("planted.rs", source);
        assert_eq!(found.len(), 1, "{source}\nfound: {found:#?}");
    }
    #[rustfmt::skip]
    let controls: [&str; 5] = [
        "#[cfg(test)] mod tests { use content::map::MapDocument; }",
        "#[cfg(test)]\n#[allow(unused)]\npub(crate) mod tests { use content::map::MapDocument; }",
        "//! A test-only file.\n#![cfg(test)]\nuse content::map::MapDocument;",
        "fn content_hash() -> u64 { 0 }",
        "// content in a comment\nfn f() -> &'static str { \"content\" }",
    ];
    for source in controls {
        assert_eq!(
            source_breaches("control.rs", source),
            Vec::<String>::new(),
            "{source}"
        );
    }
    // The manifests: a normal, a renamed, a table and a per-target naming are seen; the batch's dev line passes.
    #[rustfmt::skip]
    let manifests: [&str; 5] = [
        "[dependencies]\ncontent = { path = \"../content\" }\n",
        "[dependencies]\nc = { package = \"content\", path = \"../content\" }\n",
        "[dependencies.content]\nworkspace = true\n",
        "[target.'cfg(unix)'.dependencies]\ncontent = { path = \"../content\" }\n",
        "dependencies.content = { path = \"../content\" }\n[package]\nname = \"room\"\n",
    ];
    for manifest in manifests {
        assert_eq!(manifest_breaches(manifest).len(), 1, "{manifest}");
    }
    assert_eq!(
        manifest_breaches(&format!(
            "[package]\nname = \"room\"\n# content for the tests\n[dev-dependencies]\n{DEV_LINE}\n"
        )),
        Vec::<String>::new()
    );
    // The simulation: a document type handed on beside the time unit is seen; the time unit alone is the control.
    let time = read(&crates().join("sim/src/time.rs"));
    assert_eq!(handed_on("time.rs", &time), HANDED_ON);
    let more = time + "\npub use content::map::MapDocument;\n";
    assert_eq!(handed_on("time.rs", &more).len(), 2);
    assert_eq!(
        handed_on(
            "x.rs",
            "pub(crate) use content::map::MapDocument;\npub extern crate content;"
        )
        .len(),
        1
    );
}
