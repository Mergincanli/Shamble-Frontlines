//! The static world's proof, natively (PM-0004-32): the trees this build makes and the rays it casts are the
//! committed lines. The same check runs in WebAssembly through the test-only module
//! (`packages/sim-worker/boundary/static-world.test.mjs` under Node, `packages/e2e/tests/rules/static-world.spec.ts`
//! in the browsers), so a target whose tree or whose one ray differs is named by its map.

use std::path::{Path, PathBuf};

use devtools::files::{read_plain, write_plain};
use devtools::world_proofs::{RAYS, SHAPES, check, made};

const FILE: &str = "static-world.txt";

fn folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("vectors")
}

#[test]
fn this_build_makes_the_committed_trees_and_rays() {
    let made = made();
    // SHAMBLE_WRITE_WORLD=1, and no other value, writes the file again (after a change that is meant to change a
    // tree or a ray; the change-set's report says why).
    if std::env::var("SHAMBLE_WRITE_WORLD").as_deref() == Ok("1") {
        write_plain(&folder(), FILE, made.as_bytes()).expect("the proof's file");
    }
    let bytes = read_plain(&folder(), FILE).unwrap_or_else(|why| panic!("{why}"));
    let committed = String::from_utf8(bytes).expect("text");
    assert_eq!(check(&committed), "");
    // The file is a proof of something: every map built, the full worlds among them, and rays that met shapes.
    let lines: Vec<&str> = committed.lines().collect();
    assert_eq!(lines.len(), 1 + SHAPES.len());
    assert_eq!(
        lines[0],
        format!("shamble-static-world 1 {} {RAYS}", SHAPES.len())
    );
    let mut met = Vec::new();
    for (line, count) in lines[1..].iter().zip(SHAPES) {
        let words: Vec<&str> = line.split(' ').collect();
        assert_eq!(words.len(), 11, "{line}");
        assert_eq!(
            (words[2], words[3]),
            ("shapes", count.to_string().as_str()),
            "{line}"
        );
        met.push(words[9].parse::<u32>().expect("a count"));
    }
    // No shape, no hit; a full world is met by more than a third of the rays.
    assert_eq!(met[0], 0);
    assert!(met[7] > RAYS / 3 && met[8] > RAYS / 3, "{met:?}");
}

#[test]
fn the_check_names_the_first_map_that_differs() {
    let made = made();
    let line = made.lines().nth(8).expect("the eighth map");
    let (head, digest) = line.rsplit_once(' ').expect("a digest");
    let other = made.replace(line, &format!("{head} {}", "0".repeat(digest.len())));
    assert_eq!(
        check(&other),
        format!("this build gives\n  {line}\nand the file has\n  {head} 0000000000000000")
    );
    let short = made.lines().take(5).collect::<Vec<&str>>().join("\n");
    assert_eq!(
        check(&short),
        "this build gives 13 lines and the file has 5"
    );
    assert_eq!(check(&made), "");
}
