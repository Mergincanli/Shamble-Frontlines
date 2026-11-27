//! The static world's proof that every target builds the same tree and casts the same rays (PM-0004-32), written
//! once and run natively and, through the test-only module, in WebAssembly under Node and in the browsers.
//!
//! A seeded generator makes maps of each size a world can have (none, one, two, a few, the full 512), with boxes
//! and ramps on the map's millimetre grid, many of them sharing a face's plane with a neighbour, and casts rays
//! into each: straight down, along an axis, slanted, and down the very plane of a shape's face. Per map one line
//! holds the tree's depth, a digest of the tree's bytes and a digest of every ray's answer (the shape and the
//! distance's bits). The lines of the native build are a committed file (`vectors/static-world.txt`); a build whose
//! tree or whose one ray differs gives another line.
//!
//! Then the maps that lean on the tree's builder where a seeded map does not ([`HOSTILE`]: 512 times the same box,
//! boxes inside one another, slabs a centimetre thin and a map long, the chain of cubes whose tree is too deep),
//! each written as a file, read by the loader and built or refused: one line each. And [`judge_taken`] holds every one of these worlds, and every map the content fuzz
//! run's loader takes ([`fuzz_worlds`]), to what the loader and the builder promise together.

use std::fmt::Write as _;

use content::map::{Bounds, Form, MapDocument, Shape, Spawn, Toward};
use content::{Document, load, load_file};
use sim::collision::{MAX_TREE_DEPTH, RayHit, Solid, StaticWorld, WorldError};
use sim::state_hash::fnv1a_64;
use sim::vec::V3;
use sim::{ChaCha8, SEED_BYTES};

use crate::content_proofs::{hold, map_file};

/// The committed file's seed, the shapes of each map, and the rays cast into each.
pub const SEED: u64 = 20_261_127;
pub const SHAPES: [usize; 12] = [0, 1, 2, 3, 17, 64, 200, 512, 512, 300, 100, 31];
pub const RAYS: u32 = 2_000;

/// The generator's random numbers: the simulation's own stream type, on a seed and a stream per map.
struct Rng(ChaCha8);

impl Rng {
    fn new(seed: u64, stream: u64) -> Self {
        let mut bytes = [0; SEED_BYTES];
        bytes[..8].copy_from_slice(&seed.to_le_bytes());
        Self(ChaCha8::new(&bytes, stream))
    }

    /// A whole number from `low` up to, not including, `high` (a span far below 2^32).
    fn between(&mut self, low: i32, high: i32) -> i32 {
        let span = u64::from(high.abs_diff(low));
        low + ((u64::from(self.0.next_u32()) * span) >> 32) as i32
    }
}

const TOWARDS: [Toward; 4] = [Toward::PlusX, Toward::MinusX, Toward::PlusZ, Toward::MinusZ];

/// A map of `count` shapes on the millimetre grid, inside 45 m of the origin. Every third shape starts where the
/// one before it ends on x, so their faces share a plane; ids are given so that no shape's rank is its place.
fn map(rng: &mut Rng, count: usize) -> MapDocument {
    let metres = |millimetres: i32| f64::from(millimetres) / 1000.0;
    let mut shapes: Vec<Shape> = Vec::with_capacity(count);
    let mut last_end = 0;
    for at in 0..count {
        let size = [
            rng.between(10, 8_000),
            rng.between(10, 3_000),
            rng.between(10, 8_000),
        ];
        let x = if at % 3 == 1 {
            last_end
        } else {
            rng.between(-45_000, 37_000)
        };
        let low = [x, rng.between(0, 2_000), rng.between(-45_000, 37_000)];
        last_end = low[0] + size[0];
        let form = match rng.between(0, 4) {
            0 => Form::Ramp {
                rises_toward: TOWARDS[rng.between(0, 4) as usize],
            },
            _ => Form::Box,
        };
        shapes.push(Shape {
            id: format!("s{:03}", (at * 7_919) % 1_000),
            min: low.map(metres),
            max: [low[0] + size[0], low[1] + size[1], low[2] + size[2]].map(metres),
            form,
        });
    }
    let bounds = Bounds {
        min: [-50.0, 0.0, -50.0],
        max: [50.0, 50.0, 50.0],
    };
    MapDocument {
        id: "proof".to_owned(),
        bounds,
        spawns: Vec::new(),
        shapes,
        regions: Vec::new(),
    }
}

/// One ray: from a point on the centimetre grid, in one of the ways a game casts one. A fifth of them go straight
/// down the plane of some shape's own face, where a direction has a zero and a box is touched, not crossed.
fn ray(rng: &mut Rng, world: &StaticWorld) -> (V3, V3) {
    let centimetres = |rng: &mut Rng, low: i32, high: i32| rng.between(low, high) as f32 / 100.0;
    let origin = [
        centimetres(rng, -4_600, 4_600),
        centimetres(rng, 0, 3_000),
        centimetres(rng, -4_600, 4_600),
    ];
    let (origin, direction) = match rng.between(0, 5) {
        0 => (origin, [0.0, -1.0, 0.0]),
        1 => (
            origin,
            [
                [1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, -1.0],
            ][rng.between(0, 4) as usize],
        ),
        2 => (
            origin,
            [
                rng.between(-3, 4) as f32,
                rng.between(-3, 1) as f32,
                rng.between(-3, 4) as f32,
            ],
        ),
        3 => (
            origin,
            [
                rng.between(-10, 11) as f32 / 10.0,
                -1.0,
                rng.between(-10, 11) as f32 / 10.0,
            ],
        ),
        _ => match world
            .solids()
            .get(rng.between(0, 512) as usize % world.solids().len().max(1))
        {
            Some(solid) => {
                let x = if rng.between(0, 2) == 0 {
                    solid.min[0]
                } else {
                    solid.max[0]
                };
                (
                    [x, 40.0, solid.min[2] + (solid.max[2] - solid.min[2]) / 2.0],
                    [0.0, -1.0, 0.0],
                )
            }
            None => (origin, [0.0, -1.0, 0.0]),
        },
    };
    (
        V3::new(origin[0], origin[1], origin[2]),
        V3::new(direction[0], direction[1], direction[2]),
    )
}

/// One node of [`StaticWorld::tree_bytes`]: six `f32`, the leaf's mark, the leaf's shape.
const NODE_BYTES: usize = 29;

/// A map whose shapes keep the map kind's rules, through the world's builder, held to what the loader and the
/// builder promise together. The builder's one refusal of such a map is a tree deeper than the cap: the map kind's
/// own rules (512 shapes, corners on the millimetre grid, a centimetre of thickness, all inside bounds of a few
/// hundred metres) are the builder's other three. A world that is built holds the map's shapes in the order of
/// their ids, each by its own corners; it is the same world when it is built again; and a ray straight down onto
/// the middle of every shape meets what a search of every shape says it must. Of the shapes whose floor plan holds
/// the point it meets one, no higher than the highest top among them and no lower than the first surface that must
/// stop it (a box's top, a ramp's floor); when all of them are boxes that is one distance to the last bit, and the
/// shape is the lowest index of those as high.
///
/// `true` for a world that was built, `false` for a tree that is too deep.
///
/// # Errors
/// The promise that does not hold, in words.
pub fn judge_taken(map: &MapDocument) -> Result<bool, String> {
    let world = match StaticWorld::build(map) {
        Ok(world) => world,
        Err(refusal @ WorldError::TooDeep(depth)) if depth > MAX_TREE_DEPTH => {
            let message = refusal.message();
            let plain =
                message.len() <= 200 && message.bytes().all(|byte| (0x20..=0x7e).contains(&byte));
            hold(plain, || {
                format!("a refusal that is not short and printable: {message:?}")
            })?;
            return Ok(false);
        }
        Err(other) => {
            return Err(format!(
                "the world refuses it for more than its tree's depth: {}",
                other.message()
            ));
        }
    };
    let solids = world.solids();
    let mut ranked: Vec<&Shape> = map.shapes.iter().collect();
    ranked.sort_by(|a, b| a.id.as_bytes().cmp(b.id.as_bytes()));
    let narrowed = |corner: [f64; 3]| corner.map(|value| value as f32);
    let own = |(solid, shape): (&Solid, &&Shape)| {
        solid.min == narrowed(shape.min)
            && solid.max == narrowed(shape.max)
            && solid.form == shape.form
    };
    hold(
        solids.len() == ranked.len() && solids.iter().zip(&ranked).all(own),
        || "the world's shapes are not the map's own, in the order of their ids".to_owned(),
    )?;
    let (depth, bytes) = (world.tree_depth(), world.tree_bytes());
    let (nodes, rest) = bytes.as_chunks::<NODE_BYTES>();
    let leaves = nodes.iter().filter(|node| node[24] == 1).count();
    hold(
        depth <= MAX_TREE_DEPTH && rest.is_empty() && leaves == solids.len(),
        || {
            format!(
                "a tree {depth} deep of {leaves} leaves over {} shapes",
                solids.len()
            )
        },
    )?;
    let again = StaticWorld::build(map).map(|world| world.tree_bytes());
    hold(again.as_ref() == Ok(&bytes), || {
        "built a second time, the tree differs".to_owned()
    })?;
    // Above every shape: the loader holds a map's shapes inside its bounds.
    let top = map.bounds.max[1] as f32 + 1.0;
    let limit = top - map.bounds.min[1] as f32 + 1.0;
    for (index, solid) in solids.iter().enumerate() {
        let middle = |axis: usize| solid.min[axis] + (solid.max[axis] - solid.min[axis]) / 2.0;
        let (x, z) = (middle(0), middle(2));
        let holds = |other: &&Solid| {
            other.min[0] <= x && x <= other.max[0] && other.min[2] <= z && z <= other.max[2]
        };
        let hit = world.cast_ray(V3::new(x, top, z), V3::new(0.0, -1.0, 0.0), limit);
        let named =
            |why: String| format!("the ray down onto the middle of shape {index}: {why} ({hit:?})");
        let Some(RayHit { shape, t }) = hit else {
            return Err(named("it meets nothing".to_owned()));
        };
        // The search of every shape: the highest top (the least distance), the first index that has it, and the
        // first surface that must stop the ray.
        let (mut highest, mut stop, mut ramps): (Option<(usize, f32)>, f32, bool) =
            (None, limit, false);
        for (place, other) in solids.iter().enumerate().filter(|(_, other)| holds(other)) {
            let to_top = top - other.max[1];
            if highest.is_none_or(|(_, least)| to_top < least) {
                highest = Some((place, to_top));
            }
            let must_stop = match other.form {
                Form::Box => to_top,
                Form::Ramp { .. } => top - other.min[1],
            };
            if must_stop < stop {
                stop = must_stop;
            }
            ramps |= other.form != Form::Box;
        }
        let Some((first, least)) = highest else {
            return Err(named("no shape holds its own middle".to_owned()));
        };
        let met = solids.get(shape as usize).filter(holds);
        let own_height = met.is_some_and(|met| top - met.max[1] <= t && t <= top - met.min[1]);
        hold(own_height && least <= t && t <= stop, || {
            named(format!(
                "a search of every shape says between {least} and {stop}, on a shape that holds the point"
            ))
        })?;
        hold(ramps || (shape as usize == first && t == least), || {
            named(format!(
                "a search of every shape says shape {first} at {least}"
            ))
        })?;
    }
    Ok(true)
}

/// What the fuzz run tallies of the maps it sends on to the world's builder.
pub const WORLD_VERDICTS: [&str; 2] = ["a world built", "a tree too deep"];

/// Every map of an input the loader takes, through [`judge_taken`]: how many gave each of [`WORLD_VERDICTS`]. An
/// input the loader refuses, and one that holds no map, give none.
///
/// # Errors
/// The first map that breaks a promise, by its id.
pub fn fuzz_worlds(path: Option<&str>, bytes: &[u8]) -> Result<[u64; 2], String> {
    let maps = match path {
        Some(path) => match load_file(path, bytes) {
            Ok(Document::Map(map)) => vec![map],
            _ => Vec::new(),
        },
        None => load(bytes)
            .map(|set| set.maps().to_vec())
            .unwrap_or_default(),
    };
    let mut tally = [0; 2];
    for map in &maps {
        let built = judge_taken(map).map_err(|why| format!("its map {}: {why}", map.id))?;
        tally[usize::from(!built)] += 1;
    }
    Ok(tally)
}

/// Maps the loader takes that lean on the tree's builder, or on a ray's rule for two shapes at one distance, where a
/// seeded map does not. The first seven hold the most shapes a map may, and their shapes share a middle, a plane,
/// or all six of their faces. In the row every box is as high as every other and lies over the middle of every
/// other, and the lowest index is the box in the middle of the row: a ray onto any of them meets all 512 at one
/// distance, and whichever end the tree's walk starts from, the box it visits first is not the one that must
/// answer (the same box 512 times does not show that: there the walk happens to start with the lowest index).
/// The last two are the chain: the one map here that the builder refuses, and the deepest tree a world may have.
pub const HOSTILE: [&str; 9] = [
    "512 times the same box",
    "512 boxes inside one another",
    "512 slabs a centimetre thin, as high and as long as a map may be",
    "512 cubes of a centimetre in the map's far corner",
    "two piles of the same cube a map apart, and one cube between them",
    "a stair of 512 ramps, each starting where the last one ends",
    "a row of 512 boxes of one height, each over the middle of every other",
    "34 cubes, each so much farther out than the ones before it that the tree is a chain",
    "the first 31 cubes of that chain",
];

/// The chain's cubes by their low corners, in millimetres: the same 34 as in `sim`'s own collision tests (found by
/// rust-reviewer's search on the pinned library, 2026-11-27). Each is one centimetre wide.
#[rustfmt::skip]
const CHAIN: [[i32; 3]; 34] = [[-5, 327665, -5], [-5, 327666, -5], [-5, 327667, -5], [-5, 327663, -5], [-5, 327671, -5], [-14, 327665, -5], [4, 327665, -5], [-5, 327665, -14], [-5, 327665, 4], [-5, 327648, -5], [-5, 327746, -5], [-5, 327530, -5], [-162, 327665, -5], [-5, 327665, -162], [189, 327665, -5], [-5, 327665, 189], [-5, 328456, -5], [-5, 326420, -5], [-1272, 327665, -5], [-5, 327665, -1272], [1924, 327665, -5], [-5, 327665, 1924], [-5, 335079, -5], [-5, 316071, -5], [-11621, 327665, -5], [-5, 327665, -11621], [18096, 327665, -5], [-5, 327665, 18096], [-5, 396773, -5], [-5, 219673, -5], [-108019, 327665, -5], [-5, 327665, -108019], [168719, 327665, -5], [-5, 327665, 168719]];

/// The hostile map number `at`, or nothing past the list's end. Its bounds are the widest a map may have (the
/// aim's reach); its corners are whole millimetres; its ids are given so that no shape's rank is its place (in the
/// row: so that the lowest rank is the box in the middle).
#[must_use]
pub fn hostile(at: usize) -> Option<MapDocument> {
    // The aim's reach on x and z, and its height, in millimetres.
    const REACH: i32 = 327_670;
    const HEIGHT: i32 = 655_350;
    let corners = |place: i32| -> ([i32; 3], [i32; 3]) {
        match at {
            0 => ([-1_000, 0, -1_000], [1_000, 2_000, 1_000]),
            1 => {
                let half = 10 * (place + 1);
                ([-half, 0, -half], [half, half, half])
            }
            2 => {
                let from = 10 * place - 2_560;
                ([from, 0, -REACH], [from + 10, HEIGHT, REACH])
            }
            3 => {
                let high = [
                    REACH - 10 * (place % 8),
                    HEIGHT - 10 * (place / 8 % 8),
                    REACH - 10 * (place / 64),
                ];
                (high.map(|corner| corner - 10), high)
            }
            4 if place < 255 => ([-REACH, 0, -REACH], [10 - REACH, 10, 10 - REACH]),
            4 if place < 511 => (
                [REACH - 10, HEIGHT - 10, REACH - 10],
                [REACH, HEIGHT, REACH],
            ),
            4 => ([-5, 0, -5], [5, 10, 5]),
            5 => {
                let from = 100 * place - 25_600;
                (
                    [from, 20 * place, -1_000],
                    [from + 100, 20 * place + 20, 1_000],
                )
            }
            6 => {
                let middle = 10 * place - 2_555;
                ([middle - 6_000, 0, -1_000], [middle + 6_000, 2_000, 1_000])
            }
            _ => {
                let low = CHAIN[place as usize % CHAIN.len()];
                (low, low.map(|corner| corner + 10))
            }
        }
    };
    let count = match at {
        7 => 34,
        8 => 31,
        _ => 512,
    };
    let rank = |place: i32| match at {
        6 => (place + 256) % 512,
        _ => (place * 7_919) % 1_000,
    };
    let metres = |millimetres: i32| f64::from(millimetres) / 1000.0;
    let shape = |place: i32| {
        let (min, max) = corners(place);
        Shape {
            id: format!("h{:03}", rank(place)),
            min: min.map(metres),
            max: max.map(metres),
            form: if at == 5 {
                Form::Ramp {
                    rises_toward: Toward::PlusX,
                }
            } else {
                Form::Box
            },
        }
    };
    (at < HOSTILE.len()).then(|| MapDocument {
        id: "hostile".to_owned(),
        bounds: Bounds {
            min: [-REACH, 0, -REACH].map(metres),
            max: [REACH, HEIGHT, REACH].map(metres),
        },
        spawns: vec![Spawn {
            id: "s".to_owned(),
            at: [0.0; 3],
            facing_deg: 0.0,
        }],
        shapes: (0..count).map(shape).collect(),
        regions: Vec::new(),
    })
}

/// A hostile map's line: written as a file, read by the loader, built and judged.
fn hostile_line(at: usize, document: &MapDocument) -> String {
    let (path, text) = map_file(document);
    let read = match load_file(&path, &text) {
        Ok(Document::Map(read)) if read == *document => read,
        Ok(_) => return format!("hostile {at} read as another document"),
        Err(error) => return format!("hostile {at} not taken: {}", error.record()),
    };
    let shapes = read.shapes.len();
    match (judge_taken(&read), StaticWorld::build(&read)) {
        (Err(why), _) => format!("hostile {at} shapes {shapes} broken: {why}"),
        (Ok(_), Err(why)) => format!("hostile {at} shapes {shapes} refused: {}", why.message()),
        (Ok(_), Ok(world)) => format!(
            "hostile {at} shapes {shapes} depth {} tree {:016x}",
            world.tree_depth(),
            fnv1a_64(&world.tree_bytes())
        ),
    }
}

/// The proof's text as this build makes it: a first line that says what follows, then one line per seeded map and
/// one per hostile map.
#[must_use]
pub fn made() -> String {
    let mut text = format!(
        "shamble-static-world 1 {} {RAYS} {}\n",
        SHAPES.len(),
        HOSTILE.len()
    );
    for (number, count) in SHAPES.into_iter().enumerate() {
        let mut rng = Rng::new(SEED, number as u64);
        let document = map(&mut rng, count);
        if let Err(why) = judge_taken(&document) {
            let _ = writeln!(text, "map {number} shapes {count} broken: {why}");
            continue;
        }
        let world = match StaticWorld::build(&document) {
            Ok(world) => world,
            Err(why) => {
                let _ = writeln!(
                    text,
                    "map {number} shapes {count} refused: {}",
                    why.message()
                );
                continue;
            }
        };
        let (mut answers, mut met) = (Vec::with_capacity(RAYS as usize * 8), 0_u32);
        for _ in 0..RAYS {
            let (origin, direction) = ray(&mut rng, &world);
            let hit = world.cast_ray(origin, direction, 200.0);
            met += u32::from(hit.is_some());
            answers.extend_from_slice(&hit.map_or(u32::MAX, |hit| hit.shape).to_le_bytes());
            answers.extend_from_slice(&hit.map_or(0, |hit| hit.t.to_bits()).to_le_bytes());
        }
        let _ = writeln!(
            text,
            "map {number} shapes {count} depth {} tree {:016x} rays {met} {:016x}",
            world.tree_depth(),
            fnv1a_64(&world.tree_bytes()),
            fnv1a_64(&answers)
        );
    }
    for at in 0..HOSTILE.len() {
        if let Some(document) = hostile(at) {
            let _ = writeln!(text, "{}", hostile_line(at, &document));
        }
    }
    text
}

/// Holds this build against the committed lines: empty when they are the same, and the first line that differs,
/// from both sides, when they are not.
#[must_use]
pub fn check(expected: &str) -> String {
    let made = made();
    if made == expected {
        return String::new();
    }
    let mut pairs = made.lines().zip(expected.lines());
    match pairs.find(|(ours, theirs)| ours != theirs) {
        Some((ours, theirs)) => format!("this build gives\n  {ours}\nand the file has\n  {theirs}"),
        None => format!(
            "this build gives {} lines and the file has {}",
            made.lines().count(),
            expected.lines().count()
        ),
    }
}
