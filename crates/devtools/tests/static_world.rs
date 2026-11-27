//! The static world's proof, natively (PM-0004-32): the trees this build makes and the rays it casts are the
//! committed lines, the hostile maps' among them (each written as a file, read by the loader, built or refused). The
//! same check runs in WebAssembly through the test-only module (`packages/sim-worker/boundary/static-world.test.mjs`
//! under Node, `packages/e2e/tests/rules/static-world.spec.ts` in the browsers), so a target whose tree or whose one
//! ray differs is named by its map. Then the maps under `content/` (from 4b's step 8: the test scene), natively.

use std::path::{Path, PathBuf};

use devtools::files::{read_plain, write_plain};
use devtools::world_proofs::{HOSTILE, RAYS, SHAPES, check, hostile, judge_taken, made};
use sim::collision::{MAX_TREE_DEPTH, StaticWorld};
use sim::state_hash::fnv1a_64;

const FILE: &str = "static-world.txt";
/// The world of each map under `content/`, one line a map.
const CONTENT_WORLDS: &str = "content-worlds.txt";

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
    assert_eq!(lines.len(), 1 + SHAPES.len() + HOSTILE.len());
    assert_eq!(
        lines[0],
        format!(
            "shamble-static-world 1 {} {RAYS} {}",
            SHAPES.len(),
            HOSTILE.len()
        )
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
    // The hostile maps: the loader takes every one; the builder refuses the chain alone, and by its depth; the
    // chain's first 31 cubes are the deepest tree a world may have.
    let hostile_lines = &lines[1 + SHAPES.len()..];
    let digest = |word: &str| word.len() == 16 && word.bytes().all(|byte| byte.is_ascii_hexdigit());
    for (at, line) in hostile_lines.iter().enumerate() {
        let shapes = [512, 512, 512, 512, 512, 512, 512, 34, 31][at];
        let head = format!("hostile {at} shapes {shapes} ");
        let rest = line.strip_prefix(&head).unwrap_or_else(|| panic!("{line}"));
        if at == 7 {
            assert_eq!(
                rest,
                "refused: the tree over the map's shapes is 34 deep, and at most 31 is taken: its shapes are spread \
                 too unevenly"
            );
            continue;
        }
        let ["depth", depth, "tree", tree] = rest.split(' ').collect::<Vec<&str>>()[..] else {
            panic!("{line}");
        };
        let depth: u32 = depth.parse().expect("a depth");
        let fits = if at == 8 {
            depth == 31
        } else {
            (10..=31).contains(&depth)
        };
        assert!(fits && digest(tree), "{line}");
    }
}

#[test]
fn every_hostile_map_is_one_the_judge_passes_and_the_list_ends_where_it_says() {
    for (at, name) in HOSTILE.into_iter().enumerate() {
        let map = hostile(at).unwrap_or_else(|| panic!("{name}: not made"));
        // Built, but for the chain (the judge's `false`: a tree too deep, refused by name).
        assert_eq!(judge_taken(&map), Ok(at != 7), "{name}");
    }
    assert!(hostile(HOSTILE.len()).is_none());
    // The judge has teeth where a map is none the loader takes: a shape without thickness is the builder's own
    // refusal, and no tree's depth.
    let mut flat = hostile(0).expect("the first hostile map");
    flat.shapes[3].max[1] = flat.shapes[3].min[1];
    assert_eq!(
        judge_taken(&flat),
        Err(
            "the world refuses it for more than its tree's depth: shape 3 of the map has no thickness on one of its \
             axes"
            .to_owned()
        )
    );
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
        "this build gives 22 lines and the file has 5"
    );
    assert_eq!(check(&made), "");
}

/// The test scene, the first map under `content/` (4b's step 8), through the loader and the world's builder: built
/// within the depth a world may have, passed by the judge (a ray straight down onto the middle of every shape meets
/// what a search of every shape meets), and its tree the committed one (`vectors/content-worlds.txt`, natively only;
/// `SHAMBLE_WRITE_WORLD=1`, and no other value, writes it again after a change to the map that is meant to change
/// it, and the change-set's report says why).
#[test]
fn the_test_scene_builds_the_world_whose_tree_is_committed() {
    let maps = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/maps");
    let text = read_plain(&maps, "test-scene.json").unwrap_or_else(|why| panic!("{why}"));
    let Ok(content::Document::Map(map)) = content::load_file("maps/test-scene.json", &text) else {
        panic!("the loader takes the test scene");
    };
    assert_eq!(judge_taken(&map), Ok(true));
    let world = StaticWorld::build(&map).unwrap_or_else(|why| panic!("{}", why.message()));
    assert!(
        world.tree_depth() <= MAX_TREE_DEPTH,
        "{}",
        world.tree_depth()
    );
    let made = format!(
        "test-scene shapes {} depth {} tree {:016x}\n",
        map.shapes.len(),
        world.tree_depth(),
        fnv1a_64(&world.tree_bytes())
    );
    if std::env::var("SHAMBLE_WRITE_WORLD").as_deref() == Ok("1") {
        write_plain(&folder(), CONTENT_WORLDS, made.as_bytes()).expect("the worlds' file");
    }
    let committed = String::from_utf8(
        read_plain(&folder(), CONTENT_WORLDS).unwrap_or_else(|why| panic!("{why}")),
    )
    .expect("text");
    assert_eq!(
        committed, made,
        "the test scene's world is not the committed one"
    );
}
