//! The movement table: how a soldier walks, sprints, jumps and tires (`content/movement/<id>.json`).
//!
//! Its fields carry the names and units of Mustafa's Testlines Export, so the check between the two is plain
//! equality (the values come from his Export and from nothing else: his point 8, 2026-11-25). Since version 2, twelve
//! fields hold numbers his sandbox keeps in its code and not in its Export, each a row of the hand-over table that
//! names its place in his sandbox (his NO-SOURCE (b), 2026-11-28: "not from his Export"), and one more comes from his
//! Export's measured block (`body_height_m`). The ranges are wide bounds that keep a typing mistake out of the
//! simulation; they are no tuning. The simulation works out its own constants from these once, when a match starts.

use crate::error::Error;
use crate::json::Node;
use crate::read::{Field, Object, Range};

/// The `format` of a movement file, and its `version`: the one version the typed read takes (an older file is
/// migrated before it, ADR 0005's CD-4; version 1, without the thirteen fields of version 2, has no migration: no such
/// file exists outside the repository's history).
pub const FORMAT: &str = "shamble-movement";
pub const VERSION: u32 = 2;

/// The fields, in the order of [`Movement`]'s own.
#[rustfmt::skip]
pub const FIELDS: [Field; 28] = [
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
    Field { name: "body_height_m", unit: "m", range: Range::closed(0.5, 4.0) },
    Field { name: "foot_radius_m", unit: "m", range: Range::closed(0.01, 2.0) },
    Field { name: "pull_share_per_step", unit: "share", range: Range::above(0.0, 1.0) },
    Field { name: "pull_done_under_m", unit: "m", range: Range::closed(0.0, 1.0) },
    Field { name: "pull_to_foot_pct", unit: "%", range: Range::closed(0.0, 100.0) },
    Field { name: "step_allowance_m", unit: "m", range: Range::closed(0.0, 0.1) },
    Field { name: "snap_down_m", unit: "m", range: Range::closed(0.0, 1.5) },
    Field { name: "push_height_gap_m", unit: "m", range: Range::closed(0.0, 5.0) },
    Field { name: "push_sideways_share", unit: "share", range: Range::closed(0.0, 1.0) },
    Field { name: "push_keep_going_share", unit: "share", range: Range::closed(0.0, 1.0) },
    Field { name: "push_most_m_per_step", unit: "m", range: Range::closed(0.0, 2.0) },
    Field { name: "push_head_on_gate_m_per_s", unit: "m/s", range: Range::closed(0.0, 20.0) },
    Field { name: "push_keep_going_gate_m_per_s", unit: "m/s", range: Range::closed(0.0, 20.0) },
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
    pub body_height_m: f64,
    pub foot_radius_m: f64,
    pub pull_share_per_step: f64,
    pub pull_done_under_m: f64,
    pub pull_to_foot_pct: f64,
    pub step_allowance_m: f64,
    pub snap_down_m: f64,
    pub push_height_gap_m: f64,
    pub push_sideways_share: f64,
    pub push_keep_going_share: f64,
    pub push_most_m_per_step: f64,
    pub push_head_on_gate_m_per_s: f64,
    pub push_keep_going_gate_m_per_s: f64,
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
        body_height_m,
        foot_radius_m,
        pull_share_per_step,
        pull_done_under_m,
        pull_to_foot_pct,
        step_allowance_m,
        snap_down_m,
        push_height_gap_m,
        push_sideways_share,
        push_keep_going_share,
        push_most_m_per_step,
        push_head_on_gate_m_per_s,
        push_keep_going_gate_m_per_s,
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
        body_height_m,
        foot_radius_m,
        pull_share_per_step,
        pull_done_under_m,
        pull_to_foot_pct,
        step_allowance_m,
        snap_down_m,
        push_height_gap_m,
        push_sideways_share,
        push_keep_going_share,
        push_most_m_per_step,
        push_head_on_gate_m_per_s,
        push_keep_going_gate_m_per_s,
    })
}

/// A valid movement file's text with made-up values, for this crate's tests and the tools' (never a content file:
/// no value of the game rests on it).
#[doc(hidden)]
pub const TEST_TEXT: &str = "{\n  \"ads_speed_multiplier\": 0.5,\n  \"air_control_pct\": 100,\n  \"body_height_m\": 1.75,\n  \"body_radius_m\": 0.5,\n  \"foot_radius_m\": 0.2,\n  \"format\": \"shamble-movement\",\n  \"gravity_m_per_s2\": 20,\n  \"id\": \"test-soldier\",\n  \"jump_reach_m\": 3,\n  \"pull_done_under_m\": 0.05,\n  \"pull_share_per_step\": 0.25,\n  \"pull_to_foot_pct\": 80,\n  \"push_head_on_gate_m_per_s\": 0.3,\n  \"push_height_gap_m\": 0.75,\n  \"push_keep_going_gate_m_per_s\": 0.15,\n  \"push_keep_going_share\": 0.4,\n  \"push_most_m_per_step\": 0.2,\n  \"push_radius_m\": 0.5,\n  \"push_sideways_share\": 0.6,\n  \"push_strength_per_s\": 10,\n  \"snap_down_m\": 0.25,\n  \"speed_up_time_to_90pct_s\": 0.25,\n  \"sprint_multiplier\": 1.5,\n  \"sprint_returns_at_pct\": 25,\n  \"sprint_time_from_full_s\": 8,\n  \"stamina_refill_time_s\": 4,\n  \"step_allowance_m\": 0.01,\n  \"step_up_m\": 0.5,\n  \"stop_time_to_10pct_s\": 0.25,\n  \"version\": 2,\n  \"walk_speed_m_per_s\": 6\n}\n";

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
            kind(&TEST_TEXT.replace("\"version\": 2", "\"version\": 2,\n  \"vigour\": 2")),
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

    /// The typed read takes version 2 and no other (the advisor agent's VERSION-RULE ruling, 2026-11-28): version 1
    /// lacks the thirteen fields of version 2 and has no migration, so it is refused as a version, not read as 2.
    #[test]
    fn version_2_is_read_and_every_other_version_is_refused_as_a_version() {
        let kind = |version: &str| {
            let text = TEST_TEXT.replace("\"version\": 2", &format!("\"version\": {version}"));
            read(&parse(text.as_bytes()).expect("valid text"))
                .expect_err("refused")
                .kind
        };
        assert!(read(&parse(TEST_TEXT.as_bytes()).expect("valid text")).is_ok());
        for version in ["1", "3", "0", "1.5", "-1"] {
            assert!(
                matches!(kind(version), Kind::WrongVersion { newest: 2, .. }),
                "version {version}"
            );
        }
    }

    /// The thirteen fields of version 2 read as their own: each field's value lands in its own member.
    #[test]
    fn the_fields_of_version_2_each_land_in_their_own_member() {
        let movement =
            read(&parse(TEST_TEXT.as_bytes()).expect("valid text")).expect("a movement table");
        assert_eq!(
            [
                movement.body_height_m,
                movement.foot_radius_m,
                movement.pull_share_per_step,
                movement.pull_done_under_m,
                movement.pull_to_foot_pct,
                movement.step_allowance_m,
                movement.snap_down_m,
            ],
            [1.75, 0.2, 0.25, 0.05, 80.0, 0.01, 0.25]
        );
        assert_eq!(
            [
                movement.push_height_gap_m,
                movement.push_sideways_share,
                movement.push_keep_going_share,
                movement.push_most_m_per_step,
                movement.push_head_on_gate_m_per_s,
                movement.push_keep_going_gate_m_per_s,
            ],
            [0.75, 0.6, 0.4, 0.2, 0.3, 0.15]
        );
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
