//! The movement table: how a soldier walks, sprints, jumps and tires (`content/movement/<id>.json`).
//!
//! Its fields carry the names and units of Mustafa's Testlines Export, so the check between the two is plain
//! equality (the values come from his Export and from nothing else: his point 8, 2026-11-25). The ranges are wide
//! bounds that keep a typing mistake out of the simulation; they are no tuning. The simulation works out its own
//! constants from these once, when a match starts.

use crate::error::Error;
use crate::json::Node;
use crate::read::{Field, Object, Range};

/// The `format` of a movement file, and the newest `version` this build reads.
pub const FORMAT: &str = "shamble-movement";
pub const VERSION: u32 = 1;

/// The fields, in the order of [`Movement`]'s own.
#[rustfmt::skip]
pub const FIELDS: [Field; 15] = [
    Field { name: "walk_speed_m_per_s", unit: "m/s", range: Range::above(0.0, 20.0) },
    Field { name: "sprint_multiplier", unit: "times", range: Range::closed(1.0, 3.0) },
    Field { name: "speed_up_time_to_90pct_s", unit: "s", range: Range::above(0.0, 5.0) },
    Field { name: "stop_time_to_10pct_s", unit: "s", range: Range::above(0.0, 5.0) },
    Field { name: "air_control_pct", unit: "%", range: Range::closed(0.0, 100.0) },
    Field { name: "body_radius_m", unit: "m", range: Range::closed(0.1, 2.0) },
    Field { name: "jump_reach_m", unit: "m", range: Range::closed(0.0, 10.0) },
    Field { name: "gravity_m_per_s2", unit: "m/s2", range: Range::above(0.0, 100.0) },
    Field { name: "step_up_m", unit: "m", range: Range::closed(0.0, 1.5) },
    Field { name: "sprint_time_from_full_s", unit: "s", range: Range::above(0.0, 60.0) },
    Field { name: "stamina_refill_time_s", unit: "s", range: Range::above(0.0, 120.0) },
    Field { name: "sprint_returns_at_pct", unit: "%", range: Range::closed(0.0, 100.0) },
    Field { name: "push_strength_per_s", unit: "1/s", range: Range::closed(0.0, 100.0) },
    Field { name: "push_radius_m", unit: "m", range: Range::closed(0.1, 2.0) },
    Field { name: "ads_speed_multiplier", unit: "times", range: Range::above(0.0, 1.0) },
];

/// One movement table, read and checked.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub struct Movement {
    pub id: String,
    pub walk_speed_m_per_s: f64,
    pub sprint_multiplier: f64,
    pub speed_up_time_to_90pct_s: f64,
    pub stop_time_to_10pct_s: f64,
    pub air_control_pct: f64,
    pub body_radius_m: f64,
    pub jump_reach_m: f64,
    pub gravity_m_per_s2: f64,
    pub step_up_m: f64,
    pub sprint_time_from_full_s: f64,
    pub stamina_refill_time_s: f64,
    pub sprint_returns_at_pct: f64,
    pub push_strength_per_s: f64,
    pub push_radius_m: f64,
    pub ads_speed_multiplier: f64,
}

/// Reads a movement file's tree: its header, each field inside its range, and no key beyond them.
pub fn read(top: &Node) -> Result<Movement, Error> {
    let mut object = Object::of(top, "")?;
    let id = object.header(FORMAT, VERSION)?.to_owned();
    let mut values = [0.0; FIELDS.len()];
    for (value, field) in values.iter_mut().zip(&FIELDS) {
        *value = object.number(field)?;
    }
    object.finish()?;
    let [
        walk_speed_m_per_s,
        sprint_multiplier,
        speed_up_time_to_90pct_s,
        stop_time_to_10pct_s,
        air_control_pct,
        body_radius_m,
        jump_reach_m,
        gravity_m_per_s2,
        step_up_m,
        sprint_time_from_full_s,
        stamina_refill_time_s,
        sprint_returns_at_pct,
        push_strength_per_s,
        push_radius_m,
        ads_speed_multiplier,
    ] = values;
    Ok(Movement {
        id,
        walk_speed_m_per_s,
        sprint_multiplier,
        speed_up_time_to_90pct_s,
        stop_time_to_10pct_s,
        air_control_pct,
        body_radius_m,
        jump_reach_m,
        gravity_m_per_s2,
        step_up_m,
        sprint_time_from_full_s,
        stamina_refill_time_s,
        sprint_returns_at_pct,
        push_strength_per_s,
        push_radius_m,
        ads_speed_multiplier,
    })
}

/// A valid movement file's text with made-up values, for this crate's tests and the tools' (never a content file:
/// no value of the game rests on it).
#[doc(hidden)]
pub const TEST_TEXT: &str = "{\n  \"ads_speed_multiplier\": 0.5,\n  \"air_control_pct\": 100,\n  \"body_radius_m\": 0.5,\n  \"format\": \"shamble-movement\",\n  \"gravity_m_per_s2\": 20,\n  \"id\": \"test-soldier\",\n  \"jump_reach_m\": 3,\n  \"push_radius_m\": 0.5,\n  \"push_strength_per_s\": 10,\n  \"speed_up_time_to_90pct_s\": 0.25,\n  \"sprint_multiplier\": 1.5,\n  \"sprint_returns_at_pct\": 25,\n  \"sprint_time_from_full_s\": 8,\n  \"stamina_refill_time_s\": 4,\n  \"step_up_m\": 0.5,\n  \"stop_time_to_10pct_s\": 0.25,\n  \"version\": 1,\n  \"walk_speed_m_per_s\": 6\n}\n";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Kind;
    use crate::json::parse;

    #[test]
    fn a_movement_file_reads_field_by_field() {
        let movement =
            read(&parse(TEST_TEXT.as_bytes()).expect("valid text")).expect("a movement table");
        assert_eq!(movement.id, "test-soldier");
        assert_eq!(
            (
                movement.walk_speed_m_per_s,
                movement.sprint_multiplier,
                movement.ads_speed_multiplier
            ),
            (6.0, 1.5, 0.5)
        );
        assert_eq!(
            (
                movement.jump_reach_m,
                movement.gravity_m_per_s2,
                movement.step_up_m
            ),
            (3.0, 20.0, 0.5)
        );
    }

    #[test]
    fn a_key_beyond_the_fields_a_missing_one_and_a_value_out_of_range_are_refused() {
        let kind = |text: &str| {
            read(&parse(text.as_bytes()).expect("valid text"))
                .expect_err("refused")
                .kind
        };
        assert!(matches!(
            kind(&TEST_TEXT.replace("\"version\": 1", "\"version\": 1,\n  \"vigour\": 2")),
            Kind::UnknownKey { .. }
        ));
        assert_eq!(
            kind(&TEST_TEXT.replace("  \"step_up_m\": 0.5,\n", "")),
            Kind::MissingKey { key: "step_up_m" }
        );
        assert!(matches!(
            kind(&TEST_TEXT.replace("\"step_up_m\": 0.5", "\"step_up_m\": 1.6")),
            Kind::OutOfRange { .. }
        ));
        assert!(matches!(
            kind(&TEST_TEXT.replace("\"walk_speed_m_per_s\": 6", "\"walk_speed_m_per_s\": 0")),
            Kind::OutOfRange { .. }
        ));
        assert!(matches!(
            kind(&TEST_TEXT.replace("shamble-movement", "shamble-map")),
            Kind::WrongFormat { .. }
        ));
    }

    #[test]
    fn every_field_has_finite_bounds_and_its_own_name() {
        // PM-0005-5's and the loader's premise: no number field without a finite range (a number such as 1e300 is
        // refused by its range, not carried into the simulation).
        for field in &FIELDS {
            assert!(
                field.range.min.is_finite()
                    && field.range.max.is_finite()
                    && field.range.min < field.range.max,
                "{}",
                field.name
            );
            assert!(field.range.max.abs() < 9_007_199_254_740_992.0);
            assert!(crate::json::is_key(field.name));
        }
        let mut names: Vec<&str> = FIELDS.iter().map(|field| field.name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), FIELDS.len());
    }
}
