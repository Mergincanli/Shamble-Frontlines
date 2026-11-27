//! The map: the static world of a match, and where its soldiers start (`content/maps/<id>.json`).
//!
//! `bounds` is the playable box. `spawns` (at most 16) are the places soldiers start at, each with the way it faces.
//! `shapes` (at most 512) are the solid pieces, each a box or a ramp inside its box, all axis-aligned: no angle
//! enters the static world. `regions` (at most 64) are patches of ground that are only drawn. Lengths are metres and
//! y is up.
//!
//! Beyond its fields' ranges a map has rules of its own, each exact, with no tolerance anywhere:
//! - every corner lies on the millimetre grid and the bounds on the centimetre grid, so that a thickness is a whole
//!   number of steps and "inside" is a plain comparison of two doubles;
//! - the bounds, every shape and every region are at least 1 cm thick on every axis (so a ramp rises);
//! - every shape, spawn and region lies inside the bounds; there is at least one spawn; an id stands once in its list;
//! - the bounds lie inside the aim's reach. Mustafa's words (2026-11-26): "until the aim codec widens, the loader
//!   refuses a playable map larger than its reach".
//!
//! [`check`] holds these and [`read`] runs it, so no map reaches a match without them. A map built in code, for a
//! test of the simulation alone, never passes the loader and is no playable map.

use crate::error::{Echo, Error, Kind};
use crate::json::Node;
use crate::read::{Field, Object, Range};

/// The `format` of a map file, and the newest `version` this build reads.
pub const FORMAT: &str = "shamble-map";
pub const VERSION: u32 = 1;
/// The most entries each list of a map may hold.
pub const MAX_SPAWNS: u32 = 16;
/// The fewest spawns a map may hold (the shapes and the regions may be none).
pub const MIN_SPAWNS: u32 = 1;
pub const MAX_SHAPES: u32 = 512;
pub const MAX_REGIONS: u32 = 64;

/// How far from the map's origin an aim point reaches on x and on z, in centimetres either way: a command's aim
/// holds each as a signed 16-bit count of centimetres. One centimetre short of that type's own negative end
/// (-32,768), so that the reach is the same both ways: 327.67 m. `devtools`'s `tests/aim_reach.rs` ties this and the
/// height to the aim point's own field types, and fails when a field widens.
pub const AIM_REACH_CM: i32 = 32_767;
const AIM_REACH_MIN_CM: i32 = -AIM_REACH_CM;
/// How high an aim point reaches, in centimetres above the floor at 0 (an unsigned 16-bit count): 655.35 m.
pub const AIM_HEIGHT_MAX_CM: i32 = 65_535;

/// A grid: its steps a metre, and the least thickness of a box on it, 1 cm, in those steps.
#[derive(Debug, Clone, Copy)]
pub struct Grid {
    pub per_metre: u32,
    pub thinnest: i64,
}

/// The bounds lie on whole centimetres (an aim is whole centimetres, so a client that rounds and clamps in whole
/// numbers never lands outside the map); every other corner on whole millimetres.
#[rustfmt::skip]
pub const BOUNDS_GRID: Grid = Grid { per_metre: 100, thinnest: 1 };
#[rustfmt::skip]
pub const CORNER_GRID: Grid = Grid { per_metre: 1_000, thinnest: 10 };

/// Any coordinate: wide bounds that keep a typing mistake out. The map's own bounds are held to the aim's reach.
pub const COORDINATE: Range = Range::closed(-10_000.0, 10_000.0);
/// The way a spawn faces, in degrees from 0 up to 360. **0 faces +z, and the degrees grow toward +x**: 90 faces +x,
/// 180 faces -z, 270 faces -x (seen from above with +x to the right and +z toward the viewer, that is a turn
/// against the clock). It is the old game's own convention, kept so that a place and a facing mean what they meant
/// there: its soldiers turn by `rotation.y = Math.atan2(dir.x, dir.z)` (`legacy/shamble-game.js:2710`, `:2758`; the
/// harvester the same, `:871`). Pinned here before the first map file is written (the advisor agent's condition on
/// 4b-2); no code reads a facing yet, and the change that first does turns it into the simulation's own direction
/// without a sine or a cosine of the standard library (ADR 0004 part 7).
#[rustfmt::skip]
pub const FACING: Field = Field { name: "facing_deg", unit: "deg", range: Range { min: 0.0, max: 360.0, min_inclusive: true, max_inclusive: false } };

/// The fixed words of a map, in the order of the types below (the schema writer lists the same words).
pub(crate) const FORMS: [&str; 2] = ["box", "ramp"];
pub(crate) const TOWARDS: [&str; 4] = ["+x", "-x", "+z", "-z"];
pub(crate) const GROUNDS: [&str; 2] = ["street", "grass"];

/// One map, read and checked.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct MapDocument {
    pub id: String,
    pub bounds: Bounds,
    pub spawns: Vec<Spawn>,
    pub shapes: Vec<Shape>,
    pub regions: Vec<Region>,
}

/// A box by its lowest and its highest corner, x, y and z.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Bounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// Where a soldier starts: `at` is the point on the ground between the feet, the soldier's own origin.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Spawn {
    pub id: String,
    pub at: [f64; 3],
    pub facing_deg: f64,
}

/// A solid piece of the world.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Shape {
    pub id: String,
    pub min: [f64; 3],
    pub max: [f64; 3],
    /// In the file its keys stand beside the others: `type`, and a ramp's `rises_toward`.
    #[cfg_attr(test, ts(flatten))]
    pub form: Form,
}

/// What fills a shape's box: all of it, or a wedge that is as high as the box on the side it rises toward and
/// meets the floor of the box on the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(tag = "type", rename_all = "lowercase"))]
pub enum Form {
    Box,
    /// A named field, as the file has it (`rises_toward`), so that the generated types can say the same.
    Ramp {
        rises_toward: Toward,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub enum Toward {
    #[cfg_attr(test, ts(rename = "+x"))]
    PlusX,
    #[cfg_attr(test, ts(rename = "-x"))]
    MinusX,
    #[cfg_attr(test, ts(rename = "+z"))]
    PlusZ,
    #[cfg_attr(test, ts(rename = "-z"))]
    MinusZ,
}

/// A patch of ground that is only drawn, by its lowest and highest corner in x and z.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Region {
    pub id: String,
    pub kind: Ground,
    pub min: [f64; 2],
    pub max: [f64; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(rename_all = "lowercase"))]
pub enum Ground {
    Street,
    Grass,
}

/// The entries of the list `key`, each as an object to read, at most `max` of them.
fn entries<'a>(
    object: &mut Object<'a>,
    key: &'static str,
    max: u32,
) -> Result<Vec<Object<'a>>, Error> {
    let (items, path) = object.array(key, max)?;
    let entry = |(at, item)| Object::of(item, &format!("{path}[{at}]"));
    items.iter().enumerate().map(entry).collect()
}

/// Reads a map file's tree: its header, each field inside its range, no key beyond them, and the map's own rules.
pub fn read(top: &Node) -> Result<MapDocument, Error> {
    let mut object = Object::of(top, "")?;
    let id = object.header(FORMAT, VERSION)?.to_owned();
    let mut inner = object.object("bounds")?;
    let (min, max) = (
        inner.numbers("min", COORDINATE)?,
        inner.numbers("max", COORDINATE)?,
    );
    inner.finish()?;
    let bounds = Bounds { min, max };
    let mut spawns = Vec::new();
    for mut entry in entries(&mut object, "spawns", MAX_SPAWNS)? {
        let id = entry.id("id")?.to_owned();
        spawns.push(Spawn {
            id,
            at: entry.numbers("at", COORDINATE)?,
            facing_deg: entry.number(&FACING)?,
        });
        entry.finish()?;
    }
    let mut shapes = Vec::new();
    for mut entry in entries(&mut object, "shapes", MAX_SHAPES)? {
        let id = entry.id("id")?.to_owned();
        let (min, max) = (
            entry.numbers("min", COORDINATE)?,
            entry.numbers("max", COORDINATE)?,
        );
        // A box has no `rises_toward`: the key is read only for a ramp, so on a box it is a key the kind does not have.
        let form = match entry.one_of("type", &FORMS, "box or ramp")? {
            0 => Form::Box,
            _ => Form::Ramp {
                rises_toward: match entry.one_of("rises_toward", &TOWARDS, "+x, -x, +z or -z")? {
                    0 => Toward::PlusX,
                    1 => Toward::MinusX,
                    2 => Toward::PlusZ,
                    _ => Toward::MinusZ,
                },
            },
        };
        entry.finish()?;
        shapes.push(Shape { id, min, max, form });
    }
    let mut regions = Vec::new();
    for mut entry in entries(&mut object, "regions", MAX_REGIONS)? {
        let id = entry.id("id")?.to_owned();
        let kind = if entry.one_of("kind", &GROUNDS, "street or grass")? == 0 {
            Ground::Street
        } else {
            Ground::Grass
        };
        regions.push(Region {
            id,
            kind,
            min: entry.numbers("min", COORDINATE)?,
            max: entry.numbers("max", COORDINATE)?,
        });
        entry.finish()?;
    }
    object.finish()?;
    let map = MapDocument {
        id,
        bounds,
        spawns,
        shapes,
        regions,
    };
    check(&map)?;
    Ok(map)
}

/// A refusal by one of the map's own rules: it names the field, and no place in the text.
fn refuse(path: String, kind: Kind) -> Result<(), Error> {
    Err(Error::at(0, 0, &path, kind))
}

/// A coordinate's place on a grid, or `None` when it lies between two steps: the
/// count of steps, rounded, must give the same double back when it is divided again.
fn on_grid(value: f64, grid: Grid) -> Option<i64> {
    let per_metre = f64::from(grid.per_metre);
    let steps = (value * per_metre).round();
    (steps / per_metre == value).then_some(steps as i64)
}

/// A box's own rules, on as many axes as it has: both corners on the grid, and at least 1 cm between them.
fn check_box(path: &str, min: &[f64], max: &[f64], grid: Grid) -> Result<(), Error> {
    for (axis, (&low, &high)) in min.iter().zip(max).enumerate() {
        let place = |value: f64, end: &str| {
            let off = Kind::OffGrid {
                bits: value.to_bits(),
                per_metre: grid.per_metre,
            };
            on_grid(value, grid)
                .ok_or_else(|| Error::at(0, 0, &format!("{path}.{end}[{axis}]"), off))
        };
        if place(high, "max")?.saturating_sub(place(low, "min")?) < grid.thinnest {
            let thin = Kind::TooThin {
                min_bits: low.to_bits(),
                max_bits: high.to_bits(),
            };
            return refuse(format!("{path}.max[{axis}]"), thin);
        }
    }
    Ok(())
}

/// `values` from `low` to `high`, axis by axis; `field` names a value's field for a refusal.
fn check_inside(
    values: &[f64],
    low: &[f64],
    high: &[f64],
    field: impl Fn(usize) -> String,
) -> Result<(), Error> {
    for (axis, ((&value, &low), &high)) in values.iter().zip(low).zip(high).enumerate() {
        let bound = match (value < low, value > high) {
            (true, _) => low,
            (_, true) => high,
            _ => continue,
        };
        let kind = Kind::OutsideBounds {
            bits: value.to_bits(),
            bound_bits: bound.to_bits(),
        };
        return refuse(field(axis), kind);
    }
    Ok(())
}

/// An id stands once in its list: the later of two equal ones is named.
fn check_ids<'a>(list: &str, ids: impl Iterator<Item = &'a str>) -> Result<(), Error> {
    let mut order: Vec<(&str, usize)> = ids.enumerate().map(|(at, id)| (id, at)).collect();
    order.sort_unstable();
    for pair in order.windows(2) {
        if let [(first, _), (second, at)] = *pair
            && first == second
        {
            return refuse(
                format!("{list}[{at}].id"),
                Kind::DuplicateId {
                    id: Echo::of(second.as_bytes()),
                },
            );
        }
    }
    Ok(())
}

/// The map's own rules (the list at the top of this file), in a fixed order; the first one broken is the refusal.
pub fn check(map: &MapDocument) -> Result<(), Error> {
    let Bounds { min, max } = map.bounds;
    check_box("bounds", &min, &max, BOUNDS_GRID)?;
    // The aim's reach, in whole centimetres: the bounds are on that grid by the rule before.
    let [min_x, min_y, min_z] = min;
    let [max_x, max_y, max_z] = max;
    #[rustfmt::skip]
    let reach = [
        ("bounds.min[0]", min_x, AIM_REACH_MIN_CM, true), ("bounds.max[0]", max_x, AIM_REACH_CM, false),
        ("bounds.min[1]", min_y, 0, true), ("bounds.max[1]", max_y, AIM_HEIGHT_MAX_CM, false),
        ("bounds.min[2]", min_z, AIM_REACH_MIN_CM, true), ("bounds.max[2]", max_z, AIM_REACH_CM, false),
    ];
    for (field, value, reach_cm, lowest) in reach {
        let steps = on_grid(value, BOUNDS_GRID).unwrap_or(i64::MAX);
        if (lowest && steps < i64::from(reach_cm)) || (!lowest && steps > i64::from(reach_cm)) {
            return refuse(
                field.to_owned(),
                Kind::BeyondAim {
                    bits: value.to_bits(),
                    reach_cm,
                },
            );
        }
    }
    // Held to MIN_SPAWNS itself, so that the schema's fewest and the loader's are one number.
    let spawns = u32::try_from(map.spawns.len()).unwrap_or(u32::MAX);
    if spawns < MIN_SPAWNS {
        return refuse(
            "spawns".to_owned(),
            Kind::TooFew {
                len: spawns,
                min: MIN_SPAWNS,
            },
        );
    }
    for (at, spawn) in map.spawns.iter().enumerate() {
        let field = |axis: usize| format!("spawns[{at}].at[{axis}]");
        if let Some((axis, value)) = spawn
            .at
            .iter()
            .enumerate()
            .find(|(_, value)| on_grid(**value, CORNER_GRID).is_none())
        {
            return refuse(
                field(axis),
                Kind::OffGrid {
                    bits: value.to_bits(),
                    per_metre: CORNER_GRID.per_metre,
                },
            );
        }
        check_inside(&spawn.at, &min, &max, field)?;
    }
    for (at, shape) in map.shapes.iter().enumerate() {
        check_box(
            &format!("shapes[{at}]"),
            &shape.min,
            &shape.max,
            CORNER_GRID,
        )?;
        check_inside(&shape.min, &min, &max, |axis| {
            format!("shapes[{at}].min[{axis}]")
        })?;
        check_inside(&shape.max, &min, &max, |axis| {
            format!("shapes[{at}].max[{axis}]")
        })?;
    }
    // A region lies on the ground: its two axes are the bounds' x and z.
    let (ground_min, ground_max) = ([min_x, min_z], [max_x, max_z]);
    for (at, region) in map.regions.iter().enumerate() {
        check_box(
            &format!("regions[{at}]"),
            &region.min,
            &region.max,
            CORNER_GRID,
        )?;
        check_inside(&region.min, &ground_min, &ground_max, |axis| {
            format!("regions[{at}].min[{axis}]")
        })?;
        check_inside(&region.max, &ground_min, &ground_max, |axis| {
            format!("regions[{at}].max[{axis}]")
        })?;
    }
    check_ids("spawns", map.spawns.iter().map(|spawn| spawn.id.as_str()))?;
    check_ids("shapes", map.shapes.iter().map(|shape| shape.id.as_str()))?;
    check_ids(
        "regions",
        map.regions.iter().map(|region| region.id.as_str()),
    )
}

/// A valid map file's text in canonical form with made-up sizes, for this crate's tests and the tools' (never a
/// content file: no value of the game rests on it). Its id is the one local play asks for, `test-scene`, so a bundle
/// made of it (`bundle::test_bundle`) serves every caller that replays no committed recording. Its shapes are no
/// map's of the game: a test that rests on a map's shapes names the map it uses.
#[doc(hidden)]
pub const TEST_TEXT: &str = "{\n  \"bounds\": {\n    \"max\": [\n      50,\n      20,\n      50\n    ],\n    \"min\": [\n      -50,\n      0,\n      -50\n    ]\n  },\n  \"format\": \"shamble-map\",\n  \"id\": \"test-scene\",\n  \"regions\": [],\n  \"shapes\": [\n    {\n      \"id\": \"step\",\n      \"max\": [\n        1,\n        0.55,\n        1\n      ],\n      \"min\": [\n        0,\n        0,\n        0\n      ],\n      \"type\": \"box\"\n    }\n  ],\n  \"spawns\": [\n    {\n      \"at\": [\n        0,\n        0,\n        -5\n      ],\n      \"facing_deg\": 90,\n      \"id\": \"a\"\n    }\n  ],\n  \"version\": 1\n}\n";

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::json::parse;

    /// A small valid map on one line (a file has it in canonical form; the typed read does not ask).
    pub(crate) const TEXT: &str = r#"{"bounds": {"max": [50, 20, 50], "min": [-50, 0, -50]}, "format": "shamble-map", "id": "yard", "regions": [{"id": "street", "kind": "street", "max": [10, 2], "min": [-10, -2]}], "shapes": [{"id": "step_055", "max": [1, 0.55, 1], "min": [0, 0, 0], "type": "box"}, {"id": "ramp_a", "max": [4, 1, 2], "min": [2, 0, 0], "rises_toward": "-z", "type": "ramp"}], "spawns": [{"at": [0, 0, -5], "facing_deg": 90, "id": "a"}], "version": 1}"#;

    fn read_text(text: &str) -> Result<MapDocument, Error> {
        read(&parse(text.as_bytes()).expect(text))
    }
    fn map() -> MapDocument {
        read_text(TEXT).expect("the valid map")
    }
    /// One change to the valid map, with the field and the reason the map's own rules must give it.
    type Case = (fn(&mut MapDocument), &'static str, Kind);

    /// The field and the reason `change` gets from the map's own rules.
    fn refused(change: impl FnOnce(&mut MapDocument)) -> (String, Kind) {
        let mut changed = map();
        change(&mut changed);
        let error = check(&changed).expect_err("refused");
        assert_eq!((error.line, error.col, &error.file), (0, 0, &None));
        (error.path, error.kind)
    }

    #[test]
    fn a_map_file_reads_list_by_list() {
        let map = map();
        assert_eq!(
            (map.id.as_str(), map.bounds.min, map.bounds.max),
            ("yard", [-50.0, 0.0, -50.0], [50.0, 20.0, 50.0])
        );
        let spawn = Spawn {
            id: "a".to_owned(),
            at: [0.0, 0.0, -5.0],
            facing_deg: 90.0,
        };
        assert_eq!(map.spawns, [spawn]);
        let shape = |id: &str, min, max, form| Shape {
            id: id.to_owned(),
            min,
            max,
            form,
        };
        let ramp = shape(
            "ramp_a",
            [2.0, 0.0, 0.0],
            [4.0, 1.0, 2.0],
            Form::Ramp {
                rises_toward: Toward::MinusZ,
            },
        );
        assert_eq!(
            map.shapes,
            [
                shape("step_055", [0.0; 3], [1.0, 0.55, 1.0], Form::Box),
                ramp
            ]
        );
        let region = Region {
            id: "street".to_owned(),
            kind: Ground::Street,
            min: [-10.0, -2.0],
            max: [10.0, 2.0],
        };
        assert_eq!(map.regions, [region]);
        for (word, toward) in [
            ("+x", Toward::PlusX),
            ("-x", Toward::MinusX),
            ("+z", Toward::PlusZ),
        ] {
            let map = read_text(&TEXT.replace("\"-z\"", &format!("\"{word}\""))).expect(word);
            assert_eq!(
                map.shapes.last().map(|shape| shape.form),
                Some(Form::Ramp {
                    rises_toward: toward
                })
            );
        }
        let grass =
            read_text(&TEXT.replace("\"kind\": \"street\"", "\"kind\": \"grass\"")).expect("grass");
        assert_eq!(
            grass.regions.first().map(|region| region.kind),
            Some(Ground::Grass)
        );
    }

    #[test]
    fn the_fields_of_a_map_refuse_by_name() {
        let spawn = r#"{"at": [0, 0, -5], "facing_deg": 90, "id": "a"}"#;
        let many = |count: usize| vec![spawn; count].join(", ");
        let not_one_of = |value: &str, allowed| Kind::NotOneOf {
            value: Echo::of(value.as_bytes()),
            allowed,
        };
        #[rustfmt::skip]
        let cases: [(&str, String, &str, Kind); 12] = [
            (spawn, many(17), "spawns", Kind::TooMany { len: 17, max: 16 }),
            ("\"type\": \"box\"", "\"type\": \"sphere\"".to_owned(), "shapes[0].type", not_one_of("sphere", "box or ramp")),
            ("\"-z\"", "\"up\"".to_owned(), "shapes[1].rises_toward", not_one_of("up", "+x, -x, +z or -z")),
            ("\"kind\": \"street\"", "\"kind\": \"water\"".to_owned(), "regions[0].kind", not_one_of("water", "street or grass")),
            ("\"rises_toward\": \"-z\", ", String::new(), "shapes[1]", Kind::MissingKey { key: "rises_toward" }),
            ("\"type\": \"box\"", "\"rises_toward\": \"+x\", \"type\": \"box\"".to_owned(), "shapes[0]", Kind::UnknownKey { key: Echo::of(b"rises_toward") }),
            ("[0, 0, -5]", "[0, 0]".to_owned(), "spawns[0].at", Kind::TooFew { len: 2, min: 3 }),
            ("[10, 2]", "[10, 2, 0]".to_owned(), "regions[0].max", Kind::TooMany { len: 3, max: 2 }),
            ("\"id\": \"step_055\"", "\"id\": \"Step 055\"".to_owned(), "shapes[0].id", Kind::BadId { id: Echo::of(b"Step 055") }),
            ("\"bounds\": {\"max\": [50, 20, 50], ", "\"bounds\": {\"centre\": 0, \"max\": [50, 20, 50], ".to_owned(), "bounds", Kind::UnknownKey { key: Echo::of(b"centre") }),
            ("\"regions\": [{", "\"regions\": [1, {".to_owned(), "regions[0]", Kind::WrongType { expected: "an object", found: "a number" }),
            ("\"format\": \"shamble-map\"", "\"format\": \"shamble-movement\"".to_owned(), "format", Kind::WrongFormat { found: Echo::of(b"shamble-movement"), expected: FORMAT }),
        ];
        for (old, new, path, kind) in cases {
            assert_eq!(TEXT.matches(old).count(), 1, "{old}");
            let error = read_text(&TEXT.replace(old, &new)).expect_err(old);
            assert_eq!((error.path.as_str(), error.kind), (path, kind), "{new}");
        }
        // A facing is from 0 up to 360, and a coordinate inside wide bounds: both ends, to the last bit.
        let out = |text: String| {
            matches!(
                read_text(&text).map_err(|error| error.kind),
                Err(Kind::OutOfRange { .. })
            )
        };
        assert!(
            read_text(&TEXT.replace("90", "0")).is_ok()
                && read_text(&TEXT.replace("90", "359.999")).is_ok()
        );
        assert!(out(TEXT.replace("90", "360")) && out(TEXT.replace("90", "-1")));
        assert!(out(TEXT.replace("[0, 0, -5]", "[10000.001, 0, -5]")));
        // The lists hold their most: 16 spawns here (512 shapes and 64 regions by the same reader).
        let ids: Vec<String> = (0..16)
            .map(|at| spawn.replace("\"a\"", &format!("\"s{at}\"")))
            .collect();
        assert_eq!(
            read_text(&TEXT.replace(spawn, &ids.join(", "))).map(|map| map.spawns.len()),
            Ok(16)
        );
        assert_eq!((MAX_SPAWNS, MAX_SHAPES, MAX_REGIONS), (16, 512, 64));
        // Every number of a map lies in a range with finite ends, as every field of the movement kind does.
        for range in [COORDINATE, FACING.range] {
            assert!(range.min.is_finite() && range.max.is_finite() && range.min < range.max);
            assert!(
                range.min.abs() < 9_007_199_254_740_992.0
                    && range.max.abs() < 9_007_199_254_740_992.0
            );
        }
    }

    #[test]
    fn a_maps_own_rules_are_exact() {
        let off = |value: f64, per_metre| Kind::OffGrid {
            bits: value.to_bits(),
            per_metre,
        };
        let thin = |min: f64, max: f64| Kind::TooThin {
            min_bits: min.to_bits(),
            max_bits: max.to_bits(),
        };
        let outside = |value: f64, bound: f64| Kind::OutsideBounds {
            bits: value.to_bits(),
            bound_bits: bound.to_bits(),
        };
        let twice = |id: &str| Kind::DuplicateId {
            id: Echo::of(id.as_bytes()),
        };
        #[rustfmt::skip]
        let cases: [Case; 16] = [
            // The grids: the bounds on whole centimetres, every other corner on whole millimetres.
            (|map| map.bounds.max[0] = 50.005, "bounds.max[0]", off(50.005, 100)),
            (|map| map.shapes[0].min[2] = 0.0005, "shapes[0].min[2]", off(0.0005, 1_000)),
            (|map| map.spawns[0].at[1] = 0.0001, "spawns[0].at[1]", off(0.0001, 1_000)),
            (|map| map.regions[0].max[1] = 2.00001, "regions[0].max[1]", off(2.00001, 1_000)),
            // At least 1 cm, counted in whole steps: 9 mm is too thin, and so is a box turned inside out.
            (|map| (map.shapes[0].min[1], map.shapes[0].max[1]) = (0.14, 0.149), "shapes[0].max[1]", thin(0.14, 0.149)),
            (|map| map.shapes[0].max[0] = -1.0, "shapes[0].max[0]", thin(0.0, -1.0)),
            (|map| map.bounds.max[1] = 0.0, "bounds.max[1]", thin(0.0, 0.0)),
            (|map| map.regions[0].min[0] = 10.0, "regions[0].max[0]", thin(10.0, 10.0)),
            // Inside the bounds: one millimetre past is outside.
            (|map| map.shapes[1].max[0] = 50.001, "shapes[1].max[0]", outside(50.001, 50.0)),
            (|map| map.shapes[0].min[1] = -0.001, "shapes[0].min[1]", outside(-0.001, 0.0)),
            (|map| map.spawns[0].at[2] = -50.001, "spawns[0].at[2]", outside(-50.001, -50.0)),
            (|map| map.regions[0].max[1] = 50.001, "regions[0].max[1]", outside(50.001, 50.0)),
            // At least one spawn, and an id once in its list.
            (|map| map.spawns.clear(), "spawns", Kind::TooFew { len: 0, min: 1 }),
            (|map| map.spawns.push(map.spawns[0].clone()), "spawns[1].id", twice("a")),
            (|map| map.shapes[1].id = "step_055".to_owned(), "shapes[1].id", twice("step_055")),
            (|map| map.regions.push(map.regions[0].clone()), "regions[1].id", twice("street")),
        ];
        for (change, path, kind) in cases {
            assert_eq!(refused(change), (path.to_owned(), kind));
        }
        // Exactly 1 cm is taken, where the subtraction of the two doubles gives less (0.15 - 0.14 < 0.01).
        let mut exact = map();
        (exact.shapes[0].min[1], exact.shapes[0].max[1]) = (0.14, 0.15);
        assert!(0.15 - 0.14 < 0.01 && check(&exact).is_ok());
        // A shape may touch the bounds, and a spawn may stand on them.
        let mut touching = map();
        (
            touching.shapes[0].min,
            touching.shapes[0].max,
            touching.spawns[0].at,
        ) = ([-50.0, 0.0, -50.0], [50.0, 20.0, 50.0], [50.0, 20.0, -50.0]);
        assert_eq!(check(&touching), Ok(()));
    }

    #[test]
    fn the_bounds_lie_inside_the_aims_reach_to_the_centimetre() {
        // Six bounds at the reach's own edge are taken.
        let mut widest = map();
        widest.bounds = Bounds {
            min: [-327.67, 0.0, -327.67],
            max: [327.67, 655.35, 327.67],
        };
        assert_eq!(check(&widest), Ok(()));
        let beyond = |value: f64, reach_cm| Kind::BeyondAim {
            bits: value.to_bits(),
            reach_cm,
        };
        // One centimetre past each of the six is refused, each by its own field and edge.
        #[rustfmt::skip]
        let cases: [Case; 6] = [
            (|map| map.bounds.min[0] = -327.68, "bounds.min[0]", beyond(-327.68, -32_767)),
            (|map| map.bounds.max[0] = 327.68, "bounds.max[0]", beyond(327.68, 32_767)),
            (|map| map.bounds.min[1] = -0.01, "bounds.min[1]", beyond(-0.01, 0)),
            (|map| map.bounds.max[1] = 655.36, "bounds.max[1]", beyond(655.36, 65_535)),
            (|map| map.bounds.min[2] = -327.68, "bounds.min[2]", beyond(-327.68, -32_767)),
            (|map| map.bounds.max[2] = 327.68, "bounds.max[2]", beyond(327.68, 32_767)),
        ];
        for (change, path, kind) in cases {
            assert_eq!(refused(change), (path.to_owned(), kind));
        }
        // 0.55 m is a whole number of centimetres although 0.55 times 100 is not 55 to the last bit.
        assert!(0.55_f64 * 100.0 != 55.0 && on_grid(0.55, BOUNDS_GRID) == Some(55));
        assert_eq!(
            (on_grid(-327.67, BOUNDS_GRID), on_grid(3.071, CORNER_GRID)),
            (Some(-32_767), Some(3_071))
        );
        assert_eq!(
            (on_grid(0.005, BOUNDS_GRID), on_grid(f64::NAN, CORNER_GRID)),
            (None, None)
        );
    }
}
