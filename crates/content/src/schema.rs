//! The JSON Schema of each content kind (ADR 0005 CD-3; PM-0005-7's and PM-0005-8's schema halves): built from the
//! reader's own constants and written by this crate's canonical writer, so a schema's ranges, caps and fixed words
//! are the reader's and its text has one form. The editor's inspector and the Feel Lab are to build their fields from
//! it (CD-3: "minimum and maximum become slider limits, the description becomes the tooltip"); the verdict on a file
//! stays the loader's. Test-only, as the type export is (`ts_export.rs` writes and compares the files): no shipped
//! build holds a word of it, its descriptions included (the advisor agent's ruling of 2026-11-27, Q2).
//!
//! A schema is a content tree, and a content tree has no `true`, `false` or `null`: so an object is closed with
//! `"additionalProperties": {"not": {}}` (a schema that no value matches), an array's length is fixed by `minItems`
//! equal to `maxItems`, and a shape's two forms are a `oneOf` of two closed objects, each with all its own keys. No
//! field has a default (the reader takes none), so no schema has `default` (the ruling's Q1).

#![cfg(test)]

use crate::json::{MAX_NAME_BYTES, Member, Node, Value};
use crate::map::{
    self, AIM_HEIGHT_MAX_CM, AIM_REACH_CM, COORDINATE, FACING, MAX_REGIONS, MAX_SHAPES, MAX_SPAWNS,
    MIN_SPAWNS,
};
use crate::movement::{self, FIELDS};
use crate::read::Range;
use crate::write::canonical;

/// The dialect every schema names.
const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";
/// The id rule of `json::is_id`, once: its grammar, its length, and the names Windows keeps for its devices.
const ID_PATTERN: &str = "^[a-z0-9][a-z0-9_-]*$";
const RESERVED_PATTERN: &str = "^(con|prn|aux|nul|com[0-9]|lpt[0-9])$";

/// What each property means, by its object and its key: printable ASCII, the field's meaning and unit only (the
/// keywords beside it carry its range). The movement fields' words are those of his hand-over table's "what" column
/// (`content/source/testlines-table.json`); the map's are map.rs's own.
#[rustfmt::skip]
const DESCRIPTIONS: &[(&str, &str, &str)] = &[
    ("movement", "format", "The file's kind: shamble-movement."),
    ("movement", "version", "The version of the movement format the file is written in."),
    ("movement", "id", "The table's id, which is its file's name."),
    ("movement", "walk_speed_m_per_s", "Walking speed, in metres per second."),
    ("movement", "sprint_multiplier", "Sprint speed as a share of walking, in times the walking speed."),
    ("movement", "speed_up_time_to_90pct_s", "How fast he reaches his speed: the time to 90 percent of it, in seconds."),
    ("movement", "stop_time_to_10pct_s", "How fast he stops: the time down to 10 percent of his speed, in seconds."),
    ("movement", "air_control_pct", "Steering in the air, in percent."),
    ("movement", "body_radius_m", "His body against walls and ledges: its radius, in metres."),
    ("movement", "jump_reach_m", "The highest ledge a jump reaches, in metres."),
    ("movement", "gravity_m_per_s2", "Gravity, in metres per second squared."),
    ("movement", "step_up_m", "What he walks onto without jumping: the highest step, in metres."),
    ("movement", "sprint_time_from_full_s", "Sprint time on a full bar, in seconds."),
    ("movement", "stamina_refill_time_s", "Time to refill the bar, in seconds."),
    ("movement", "sprint_returns_at_pct", "When sprint comes back after running dry, in percent of the bar."),
    ("movement", "push_strength_per_s", "How stiffly two soldiers push apart, per second."),
    ("movement", "push_radius_m", "Body size for pushing: its radius, in metres."),
    ("movement", "ads_speed_multiplier", "Walking speed while aiming, in times the walking speed."),
    ("movement", "body_height_m", "His height, for headroom and the head bump, in metres."),
    ("movement", "foot_radius_m", "The small circle round his centre that he stands on, so he holds a little past an edge: its radius, in metres."),
    ("movement", "pull_share_per_step", "After a step-up, the share of the carry onto the step left that he gets each step, from 0 to 1."),
    ("movement", "pull_done_under_m", "A carry left shorter than this is given all at once, in metres."),
    ("movement", "pull_to_foot_pct", "The carry brings his centre this share of his foot circle short of the step's edge, in percent."),
    ("movement", "step_allowance_m", "How far above the step-up a top may be and still be walked onto, in metres."),
    ("movement", "snap_down_m", "A drop he walks down instead of falling, plus the slope under his foot, in metres."),
    ("movement", "push_height_gap_m", "Two soldiers whose feet are this far apart in height or more do not push, in metres."),
    ("movement", "push_sideways_share", "How far a head-on push turns sideways, so the two slide round each other, from 0 to 1."),
    ("movement", "push_keep_going_share", "The most of his own step forward a push may undo, from 0 to 1."),
    ("movement", "push_most_m_per_step", "The most one step's push may move him, in metres."),
    ("movement", "push_head_on_gate_m_per_s", "The push turns sideways only when the two close faster than this, in metres per second."),
    ("movement", "push_keep_going_gate_m_per_s", "The keep-going limit applies only when he moves faster than this, in metres per second."),
    ("map", "format", "The file's kind: shamble-map."),
    ("map", "version", "The version of the map format the file is written in."),
    ("map", "id", "The map's id, which is its file's name."),
    ("map", "bounds", "The box every spawn, shape and region lies in."),
    ("map", "spawns", "Where soldiers start: at least one."),
    ("map", "shapes", "The solid pieces of the world."),
    ("map", "regions", "Patches of ground that are only drawn."),
    ("bounds", "min", "The lowest corner, x, y and z, in metres: on whole centimetres, inside the aim's reach on each axis."),
    ("bounds", "max", "The highest corner, x, y and z, in metres: on whole centimetres, above the lowest on every axis, inside the aim's reach."),
    ("spawn", "id", "The spawn's id, once in the map."),
    ("spawn", "at", "The point on the ground between the feet, x, y and z, in metres: on whole millimetres, inside the bounds."),
    ("spawn", "facing_deg", "The way the soldier faces, in degrees: 0 faces +z, and the degrees grow toward +x."),
    ("shape", "id", "The shape's id, once in the map."),
    ("shape", "min", "The box's lowest corner, x, y and z, in metres: on whole millimetres, inside the bounds."),
    ("shape", "max", "The box's highest corner, x, y and z, in metres: on whole millimetres, at least 1 cm above the lowest on every axis, inside the bounds."),
    ("shape", "type", "What fills the box: all of it (box), or a wedge (ramp)."),
    ("shape", "rises_toward", "The side on which a ramp is as high as its box; it meets the floor of the box on the other."),
    ("region", "id", "The region's id, once in the map."),
    ("region", "kind", "The ground drawn there."),
    ("region", "min", "The lowest corner, x and z, in metres: on whole millimetres, inside the bounds."),
    ("region", "max", "The highest corner, x and z, in metres: on whole millimetres, above the lowest, inside the bounds."),
];

fn node(value: Value) -> Node {
    Node {
        line: 0,
        col: 0,
        value,
    }
}

fn text(word: &str) -> Node {
    node(Value::Text(word.to_owned()))
}

fn number(value: f64) -> Node {
    node(Value::Number(value))
}

fn array(items: Vec<Node>) -> Node {
    node(Value::Array(items))
}

fn object(members: Vec<(&str, Node)>) -> Node {
    let member = |(key, node): (&str, Node)| Member {
        key: key.to_owned(),
        line: 0,
        col: 0,
        node,
    };
    node(Value::Object(members.into_iter().map(member).collect()))
}

/// The description of `key` in `object`; a property without one stops the writer, so the table cannot fall behind.
fn description(of: &str, key: &str) -> Node {
    let found = DESCRIPTIONS
        .iter()
        .find(|(object, field, _)| *object == of && *field == key);
    text(found.map_or_else(
        || panic!("{of}.{key} has no description"),
        |(_, _, words)| *words,
    ))
}

/// A number inside `range`, each end inclusive or exclusive as the range has it.
fn ranged(range: Range) -> Vec<(&'static str, Node)> {
    let low = if range.min_inclusive {
        "minimum"
    } else {
        "exclusiveMinimum"
    };
    let high = if range.max_inclusive {
        "maximum"
    } else {
        "exclusiveMaximum"
    };
    vec![
        ("type", text("number")),
        (low, number(range.min)),
        (high, number(range.max)),
    ]
}

/// A list of exactly `len` coordinates, each inside `COORDINATE`.
fn corner(len: usize) -> Vec<(&'static str, Node)> {
    vec![
        ("type", text("array")),
        ("items", object(ranged(COORDINATE))),
        ("minItems", number(len as f64)),
        ("maxItems", number(len as f64)),
    ]
}

/// The bounds' corner on one side: each axis inside `COORDINATE` and the aim's reach on that side (map.rs's `check`:
/// x and z within `AIM_REACH_CM`, y from the floor at 0 up to `AIM_HEIGHT_MAX_CM`).
fn bounds_corner(lowest: bool) -> Vec<(&'static str, Node)> {
    let reach = f64::from(AIM_REACH_CM) / 100.0;
    let height = f64::from(AIM_HEIGHT_MAX_CM) / 100.0;
    let axis = |near: f64| {
        let range = if lowest {
            Range::closed(near, COORDINATE.max)
        } else {
            Range::closed(COORDINATE.min, near)
        };
        object(ranged(range))
    };
    let [x, y, z] = if lowest {
        [-reach, 0.0, -reach]
    } else {
        [reach, height, reach]
    };
    vec![
        ("type", text("array")),
        ("prefixItems", array(vec![axis(x), axis(y), axis(z)])),
        ("minItems", number(3.0)),
        ("maxItems", number(3.0)),
    ]
}

/// A property: its schema's members and its description.
fn property(
    of: &str,
    key: &'static str,
    mut members: Vec<(&'static str, Node)>,
) -> (&'static str, Node) {
    members.push(("description", description(of, key)));
    (key, object(members))
}

/// A closed object of these properties, every one required.
fn closed(properties: Vec<(&'static str, Node)>) -> Vec<(&'static str, Node)> {
    let mut keys: Vec<&str> = properties.iter().map(|(key, _)| *key).collect();
    keys.sort_unstable();
    vec![
        ("type", text("object")),
        ("required", array(keys.into_iter().map(text).collect())),
        ("properties", object(properties)),
        (
            "additionalProperties",
            object(vec![("not", object(vec![]))]),
        ),
    ]
}

/// The id rule, referenced from every id.
fn id_rule() -> Node {
    object(vec![
        ("type", text("string")),
        ("pattern", text(ID_PATTERN)),
        ("maxLength", number(MAX_NAME_BYTES as f64)),
        ("not", object(vec![("pattern", text(RESERVED_PATTERN))])),
    ])
}

fn id(of: &str) -> (&'static str, Node) {
    property(of, "id", vec![("$ref", text("#/$defs/id"))])
}

/// The header of a kind's file: its `format` and the `version` this build reads.
fn header(of: &str, format: &str, version: u32) -> [(&'static str, Node); 2] {
    [
        property(of, "format", vec![("const", text(format))]),
        property(of, "version", vec![("const", number(f64::from(version)))]),
    ]
}

/// A whole schema: the dialect, the title, the kind's own description, the id rule, and the top object.
fn document(title: &str, about: &str, properties: Vec<(&'static str, Node)>) -> Node {
    let mut members = vec![
        ("$schema", text(DIALECT)),
        ("title", text(title)),
        ("description", text(about)),
        ("$defs", object(vec![("id", id_rule())])),
    ];
    members.extend(closed(properties));
    object(members)
}

/// The movement table's schema: every field of `FIELDS` with its range.
pub fn movement_schema() -> Node {
    let mut properties = Vec::from(header("movement", movement::FORMAT, movement::VERSION));
    properties.push(id("movement"));
    for field in FIELDS {
        properties.push(property("movement", field.name, ranged(field.range)));
    }
    document(
        "shamble-movement",
        "One movement table (content/movement/<id>.json): how a soldier walks, sprints, jumps and tires.",
        properties,
    )
}

/// The map's schema: its bounds within the aim's reach, its three lists with their caps, and the fixed words.
pub fn map_schema() -> Node {
    let words = |list: &[&str]| array(list.iter().map(|word| text(word)).collect());
    // A list's fewest entries are written only when there is a fewest (JSON Schema's own default is none).
    let list = |min: u32, max: u32, item: Vec<(&'static str, Node)>| {
        let mut members = vec![
            ("type", text("array")),
            ("items", object(item)),
            ("maxItems", number(f64::from(max))),
        ];
        if min > 0 {
            members.push(("minItems", number(f64::from(min))));
        }
        members
    };
    let spawn = closed(vec![
        id("spawn"),
        property("spawn", "at", corner(3)),
        property("spawn", FACING.name, ranged(FACING.range)),
    ]);
    let shape = |form: &str, more: Vec<(&'static str, Node)>| {
        let mut keys = vec![
            id("shape"),
            property("shape", "min", corner(3)),
            property("shape", "max", corner(3)),
            property("shape", "type", vec![("const", text(form))]),
        ];
        keys.extend(more);
        object(closed(keys))
    };
    let shapes = vec![
        ("type", text("array")),
        (
            "items",
            object(vec![(
                "oneOf",
                array(vec![
                    shape(map::FORMS[0], vec![]),
                    shape(
                        map::FORMS[1],
                        vec![property(
                            "shape",
                            "rises_toward",
                            vec![("enum", words(&map::TOWARDS))],
                        )],
                    ),
                ]),
            )]),
        ),
        ("maxItems", number(f64::from(MAX_SHAPES))),
    ];
    let region = closed(vec![
        id("region"),
        property("region", "kind", vec![("enum", words(&map::GROUNDS))]),
        property("region", "min", corner(2)),
        property("region", "max", corner(2)),
    ]);
    let bounds = closed(vec![
        property("bounds", "min", bounds_corner(true)),
        property("bounds", "max", bounds_corner(false)),
    ]);
    let mut properties = Vec::from(header("map", map::FORMAT, map::VERSION));
    properties.extend([
        id("map"),
        property("map", "bounds", bounds),
        property("map", "spawns", list(MIN_SPAWNS, MAX_SPAWNS, spawn)),
        property("map", "shapes", shapes),
        property("map", "regions", list(0, MAX_REGIONS, region)),
    ]);
    document(
        "shamble-map",
        "One map (content/maps/<id>.json): its bounds, where soldiers start, its solid shapes and its drawn ground.",
        properties,
    )
}

/// The schema files, by name, as the export test writes and compares them.
pub fn schema_files() -> [(&'static str, String); 2] {
    [
        ("map.schema.json", canonical(&map_schema())),
        ("movement.schema.json", canonical(&movement_schema())),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::load_file;
    use crate::json::parse;

    /// A member of an object node, by its key.
    fn get<'a>(node: &'a Node, key: &str) -> &'a Node {
        let Value::Object(members) = &node.value else {
            panic!("not an object where {key} is looked for");
        };
        &members
            .iter()
            .find(|member| member.key == key)
            .unwrap_or_else(|| panic!("no {key}"))
            .node
    }

    fn value_of(node: &Node) -> f64 {
        let Value::Number(value) = node.value else {
            panic!("not a number")
        };
        value
    }

    /// The range a number schema says, read back from its keywords.
    fn range_of(schema: &Node) -> Range {
        let Value::Object(members) = &schema.value else {
            panic!("not an object")
        };
        let find = |key: &str| {
            members
                .iter()
                .find(|member| member.key == key)
                .map(|member| value_of(&member.node))
        };
        let (min, min_inclusive) = find("minimum").map_or_else(
            || (find("exclusiveMinimum").expect("a low end"), false),
            |min| (min, true),
        );
        let (max, max_inclusive) = find("maximum").map_or_else(
            || (find("exclusiveMaximum").expect("a high end"), false),
            |max| (max, true),
        );
        Range {
            min,
            max,
            min_inclusive,
            max_inclusive,
        }
    }

    /// Every property of every object schema in `node`, as (object, key), by the description each carries.
    fn described(node: &Node, found: &mut Vec<(String, String)>) {
        match &node.value {
            Value::Object(members) => {
                for member in members {
                    if member.key == "properties"
                        && let Value::Object(properties) = &member.node.value
                    {
                        for property in properties {
                            let Value::Text(words) = &get(&property.node, "description").value
                            else {
                                panic!("{} has no description", property.key);
                            };
                            let entry = DESCRIPTIONS
                                .iter()
                                .find(|(_, key, text)| *key == property.key && text == words)
                                .unwrap_or_else(|| {
                                    panic!("{} is described by no entry", property.key)
                                });
                            found.push((entry.0.to_owned(), entry.1.to_owned()));
                        }
                    }
                    described(&member.node, found);
                }
            }
            Value::Array(items) => items.iter().for_each(|item| described(item, found)),
            Value::Text(_) | Value::Number(_) => {}
        }
    }

    /// Each file is the canonical writer's text of its tree, and names its dialect. (The content parser takes no key
    /// of JSON Schema's own, `$defs` or `minItems`, as it should: the text's form is held by the Vitest test, with the
    /// engine's own `JSON.parse`, against `JSON.stringify` of the sorted value, the writer's own definition.)
    #[test]
    fn each_schema_is_the_writer_s_text_and_names_its_dialect() {
        for (tree, (name, text)) in [map_schema(), movement_schema()].iter().zip(schema_files()) {
            assert_eq!(canonical(tree), text, "{name}");
            let Value::Text(dialect) = &get(tree, "$schema").value else {
                panic!("{name}")
            };
            assert_eq!(dialect, DIALECT);
            assert!(
                text.bytes()
                    .all(|byte| byte == b'\n' || (0x20..=0x7e).contains(&byte)),
                "{name}"
            );
        }
        // Content's own reader refuses the schema's keys: proof that a schema is never taken for a content file.
        assert!(parse(schema_files()[1].1.as_bytes()).is_err());
    }

    /// Every property of both schemas has exactly one description, and the table holds no other: printable ASCII.
    #[test]
    fn every_property_has_one_description_and_the_table_no_other() {
        let mut found = Vec::new();
        described(&map_schema(), &mut found);
        described(&movement_schema(), &mut found);
        found.sort_unstable();
        found.dedup();
        let mut listed: Vec<(String, String)> = DESCRIPTIONS
            .iter()
            .map(|(object, key, _)| ((*object).to_owned(), (*key).to_owned()))
            .collect();
        listed.sort_unstable();
        let before = listed.len();
        listed.dedup();
        assert_eq!(listed.len(), before, "an entry stands twice");
        assert_eq!(found, listed);
        for (object, key, words) in DESCRIPTIONS {
            assert!(
                words.bytes().all(|byte| (0x20..=0x7e).contains(&byte)),
                "{object}.{key}"
            );
            assert!(words.ends_with('.'), "{object}.{key}");
        }
    }

    /// The movement schema's ranges are the reader's (`FIELDS`), each end in its own form.
    #[test]
    fn the_movement_ranges_are_the_reader_s_in_both_forms() {
        let schema = movement_schema();
        let properties = get(&schema, "properties");
        for field in FIELDS {
            assert_eq!(
                range_of(get(properties, field.name)),
                field.range,
                "{}",
                field.name
            );
        }
        // Both forms of a low end are here: closed and open.
        assert!(FIELDS.iter().any(|field| field.range.min_inclusive));
        assert!(FIELDS.iter().any(|field| !field.range.min_inclusive));
    }

    /// The map schema's numbers are map.rs's: the caps, every coordinate, the facing (open at its top), the aim's
    /// reach on the bounds, the fixed words and the id rule.
    #[test]
    fn the_map_s_numbers_and_words_are_the_reader_s() {
        let schema = map_schema();
        let properties = get(&schema, "properties");
        let cap = |list: &str| value_of(get(get(properties, list), "maxItems"));
        assert_eq!(
            [cap("spawns"), cap("shapes"), cap("regions")],
            [MAX_SPAWNS, MAX_SHAPES, MAX_REGIONS].map(f64::from)
        );
        // The fewest: one spawn (rust-reviewer's round of 2026-11-27); the other lists may be empty and say no fewest.
        assert_eq!(
            value_of(get(get(properties, "spawns"), "minItems")),
            f64::from(MIN_SPAWNS)
        );
        let Value::Object(regions) = &get(properties, "regions").value else {
            panic!("the regions' schema is an object")
        };
        assert!(regions.iter().all(|member| member.key != "minItems"));
        let spawn = get(get(get(properties, "spawns"), "items"), "properties");
        assert_eq!(range_of(get(spawn, "facing_deg")), FACING.range);
        // The facing is the one range open at its top: the schema says so with its own keyword.
        assert!(get(get(spawn, "facing_deg"), "exclusiveMaximum").value == Value::Number(360.0));
        assert_eq!(range_of(get(get(spawn, "at"), "items")), COORDINATE);
        let Value::Array(axes) = &get(
            get(get(get(properties, "bounds"), "properties"), "min"),
            "prefixItems",
        )
        .value
        else {
            panic!("the bounds' low corner has no axes")
        };
        let low: Vec<Range> = axes.iter().map(range_of).collect();
        let reach = f64::from(AIM_REACH_CM) / 100.0;
        assert_eq!(
            low,
            [-reach, 0.0, -reach].map(|near| Range::closed(near, COORDINATE.max))
        );
        let Value::Array(axes) = &get(
            get(get(get(properties, "bounds"), "properties"), "max"),
            "prefixItems",
        )
        .value
        else {
            panic!("the bounds' high corner has no axes")
        };
        let high: Vec<Range> = axes.iter().map(range_of).collect();
        let height = f64::from(AIM_HEIGHT_MAX_CM) / 100.0;
        assert_eq!(
            high,
            [reach, height, reach].map(|near| Range::closed(COORDINATE.min, near))
        );
        assert_eq!([reach, height], [327.67, 655.35]);
        let defs = get(get(&schema, "$defs"), "id");
        assert_eq!(value_of(get(defs, "maxLength")), MAX_NAME_BYTES as f64);
        let words = |node: &Node| match &node.value {
            Value::Array(items) => items
                .iter()
                .map(|item| match &item.value {
                    Value::Text(word) => word.clone(),
                    _ => panic!("not a word"),
                })
                .collect::<Vec<_>>(),
            _ => panic!("not a list of words"),
        };
        let region = get(get(get(properties, "regions"), "items"), "properties");
        assert_eq!(words(get(get(region, "kind"), "enum")), map::GROUNDS);
        let Value::Array(forms) = &get(get(get(properties, "shapes"), "items"), "oneOf").value
        else {
            panic!("no forms")
        };
        let ramp = get(&forms[1], "properties");
        assert_eq!(words(get(get(ramp, "rises_toward"), "enum")), map::TOWARDS);
    }

    /// PM-0005-7's schema half, held to the loader: at a closed end the end is taken and the next double outside
    /// refused; at an open end the end itself is refused and the next double inside taken. The numbers are read back
    /// from the schema, so a schema that drifted from the reader fails here.
    #[test]
    fn at_each_end_the_loader_does_what_the_schema_says() {
        let verdict = |name: &str, value: f64| {
            let line = movement::TEST_TEXT
                .lines()
                .find(|line| line.starts_with(&format!("  \"{name}\": ")))
                .unwrap_or_else(|| panic!("{name} stands in the test table"));
            let comma = if line.ends_with(',') { "," } else { "" };
            let text = movement::TEST_TEXT.replace(
                line,
                &format!("  \"{name}\": {}{comma}", crate::number::number_text(value)),
            );
            load_file("movement/test-soldier.json", text.as_bytes()).is_ok()
        };
        let schema = movement_schema();
        let properties = get(&schema, "properties");
        for field in FIELDS {
            let Range {
                min,
                max,
                min_inclusive,
                max_inclusive,
            } = range_of(get(properties, field.name));
            if min_inclusive {
                assert!(
                    verdict(field.name, min) && !verdict(field.name, min.next_down()),
                    "{} low",
                    field.name
                );
            } else {
                assert!(
                    !verdict(field.name, min) && verdict(field.name, min.next_up()),
                    "{} low",
                    field.name
                );
            }
            if max_inclusive {
                assert!(
                    verdict(field.name, max) && !verdict(field.name, max.next_up()),
                    "{} high",
                    field.name
                );
            } else {
                assert!(
                    !verdict(field.name, max) && verdict(field.name, max.next_down()),
                    "{} high",
                    field.name
                );
            }
        }
        // The facing, the one range open at its top: 360 refused, the double below it taken.
        let facing = |value: f64| {
            let text = map::TEST_TEXT.replace(
                "\"facing_deg\": 90",
                &format!("\"facing_deg\": {}", crate::number::number_text(value)),
            );
            load_file("maps/test-scene.json", text.as_bytes()).is_ok()
        };
        let map = map_schema();
        let spawn = get(
            get(get(get(&map, "properties"), "spawns"), "items"),
            "properties",
        );
        let Range { min, max, .. } = range_of(get(spawn, "facing_deg"));
        assert!(facing(min) && !facing(min.next_down()) && !facing(max) && facing(max.next_down()));
        // The spawns' fewest, read back from the schema: the test map's one spawn is taken, an empty list refused.
        let fewest = value_of(get(get(get(&map, "properties"), "spawns"), "minItems"));
        assert_eq!(fewest, 1.0);
        let from = map::TEST_TEXT
            .find("  \"spawns\": [\n")
            .expect("the spawns");
        let to =
            map::TEST_TEXT[from..].find("\n  ],\n").expect("their end") + from + "\n  ],\n".len();
        let none = format!(
            "{}  \"spawns\": [],\n{}",
            &map::TEST_TEXT[..from],
            &map::TEST_TEXT[to..]
        );
        assert!(load_file("maps/test-scene.json", map::TEST_TEXT.as_bytes()).is_ok());
        assert!(load_file("maps/test-scene.json", none.as_bytes()).is_err());
    }
}
