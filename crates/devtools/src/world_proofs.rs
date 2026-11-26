//! The static world's proof that every target builds the same tree and casts the same rays (PM-0004-32), written
//! once and run natively and, through the test-only module, in WebAssembly under Node and in the browsers.
//!
//! A seeded generator makes maps of each size a world can have (none, one, two, a few, the full 512), with boxes
//! and ramps on the map's millimetre grid, many of them sharing a face's plane with a neighbour, and casts rays
//! into each: straight down, along an axis, slanted, and down the very plane of a shape's face. Per map one line
//! holds the tree's depth, a digest of the tree's bytes and a digest of every ray's answer (the shape and the
//! distance's bits). The lines of the native build are a committed file (`vectors/static-world.txt`); a build whose
//! tree or whose one ray differs gives another line.

use std::fmt::Write as _;

use content::map::{Bounds, Form, MapDocument, Shape, Toward};
use sim::collision::StaticWorld;
use sim::state_hash::fnv1a_64;
use sim::vec::V3;
use sim::{ChaCha8, SEED_BYTES};

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

/// The proof's text as this build makes it: a first line that says what follows, then one line per map.
#[must_use]
pub fn made() -> String {
    let mut text = format!("shamble-static-world 1 {} {RAYS}\n", SHAPES.len());
    for (number, count) in SHAPES.into_iter().enumerate() {
        let mut rng = Rng::new(SEED, number as u64);
        let world = match StaticWorld::build(&map(&mut rng, count)) {
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
