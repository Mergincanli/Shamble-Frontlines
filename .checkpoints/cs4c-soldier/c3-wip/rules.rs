//! The match's movement rules, worked out once from the content's movement table when a match starts (rust-reviewer
//! block 11.4): each number derived in f64 from the content's own doubles and narrowed to f32 once, the stamina in
//! whole units. Nothing here runs in a tick. The content's ranges keep a typing mistake out; what they let through
//! and no rule can use is refused here, by name.

use content::movement::Movement;
use content::time::{DurationError, units_from_seconds};

use crate::TICK_HZ;
use crate::fmath::share_per_tick;
use crate::time::UNITS_PER_TICK;

/// The tick's length in seconds, as a double.
const DT_S: f64 = 1.0 / TICK_HZ as f64;

/// Where the headroom test stands a soldier's feet (his question 13, HEADROOM-FEET, open until his word).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadroomFeet {
    /// (b), his sandbox's own test: the default until he answers (the advisor agent's Q2, 2026-11-28).
    SandboxTest,
    /// (a): a top he stands on is never a wall to him.
    OwnTopNeverWall,
}

/// The mode every match plays until his word on HEADROOM-FEET.
pub const HEADROOM_FEET: HeadroomFeet = HeadroomFeet::SandboxTest;

/// A match's movement rules: metres, seconds and shares unless a name says otherwise, each narrowed once.
#[derive(Debug, Clone, PartialEq)]
pub struct Rules {
    /// Walking speed, in metres a second; sprinting speed (the walk times the sprint multiplier); and the walk while
    /// aiming down the sights (the walk times its multiplier: 4d's, carried until then).
    pub walk: f32,
    pub sprint: f32,
    pub ads: f32,
    /// The share of the gap to the wished speed that one tick closes when he speeds up, and when he slows or stops.
    pub ease_up: f32,
    pub ease_stop: f32,
    /// The share of that easing he keeps in the air.
    pub air_share: f32,
    /// His body: its radius and its height, and the radius of his foot.
    pub radius: f32,
    pub height: f32,
    pub foot_radius: f32,
    /// The highest step he walks up without a jump.
    pub step_up: f32,
    /// What gravity adds to the downward speed in a tick, and half of it times the tick (the fall in a tick from rest).
    pub g_dt: f32,
    pub half_g_dt2: f32,
    /// The jump's upward speed: enough to lift his feet to the jump's reach less a step.
    pub take_off: f32,
    /// One over twice gravity, for the arc's true top.
    pub inv_two_g: f32,
    /// The stamina in whole units: full; what a tick of sprint takes; what a tick without it gives back; and the level
    /// at which the sprint returns after he ran dry.
    pub stamina_full: u64,
    pub drain: u64,
    pub refill: u64,
    pub returns_at: u64,
    /// The soft push between two soldiers: the share of the overlap one tick undoes, and the reach between centres.
    pub push_share: f32,
    pub push_reach: f32,
    /// The step's pull toward the foot (his sandbox's): its share a step, under what it is done, and how far.
    pub pull_share: f32,
    pub pull_done_under: f32,
    pub pull_to_foot: f32,
    /// How much above the step-up still counts as a step, and how far down he keeps to the ground.
    pub step_allowance: f32,
    pub snap_down: f32,
    /// The push's guards (his sandbox's numbers).
    pub push_height_gap: f32,
    pub push_sideways_share: f32,
    pub push_keep_going_share: f32,
    pub push_most_per_step: f32,
    pub push_head_on_gate: f32,
    pub push_keep_going_gate: f32,
    /// Where the headroom test stands his feet.
    pub headroom_feet: HeadroomFeet,
}

/// A movement table that no match can be played on, though every value is inside its range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RulesError {
    /// The jump's reach is below the step-up: the take-off speed would be the root of a negative.
    ReachBelowStepUp {
        reach_m: f64,
        step_up_m: f64,
    },
    /// A sprint time or a refill time that is no whole number of units above zero (`None`: it rounds to none).
    SprintTime(Option<DurationError>),
    RefillTime(Option<DurationError>),
    /// A body shorter than twice its radius.
    BodyTooShort {
        height_m: f64,
        radius_m: f64,
    },
    /// A foot wider than the body.
    FootTooWide {
        foot_m: f64,
        radius_m: f64,
    },
    /// A pull share outside 0 to 1 (1 included, 0 not).
    PullShare(f64),
    /// A step allowance or a snap outside 0 to the step-up.
    StepAllowance {
        allowance_m: f64,
        step_up_m: f64,
    },
    SnapDown {
        snap_m: f64,
        step_up_m: f64,
    },
}

impl RulesError {
    /// The refusal in plain words.
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::ReachBelowStepUp { reach_m, step_up_m } => {
                format!(
                    "the jump's reach ({reach_m} m) is below the step-up ({step_up_m} m): no jump could reach it"
                )
            }
            Self::SprintTime(_) => {
                "the sprint time from full is no whole number of units above zero".to_owned()
            }
            Self::RefillTime(_) => {
                "the stamina's refill time is no whole number of units above zero".to_owned()
            }
            Self::BodyTooShort { height_m, radius_m } => {
                format!(
                    "the body is {height_m} m tall with a radius of {radius_m} m: it must be at least twice its radius"
                )
            }
            Self::FootTooWide { foot_m, radius_m } => {
                format!("the foot's radius ({foot_m} m) is above the body's ({radius_m} m)")
            }
            Self::PullShare(share) => {
                format!("the pull's share a step ({share}) is outside 0 to 1")
            }
            Self::StepAllowance {
                allowance_m,
                step_up_m,
            } => {
                format!(
                    "the step allowance ({allowance_m} m) is outside 0 to the step-up ({step_up_m} m)"
                )
            }
            Self::SnapDown { snap_m, step_up_m } => {
                format!("the snap down ({snap_m} m) is outside 0 to the step-up ({step_up_m} m)")
            }
        }
    }
}

/// A duration as whole units above zero, or why not.
fn units(seconds: f64) -> Result<u32, Option<DurationError>> {
    match units_from_seconds(seconds) {
        Ok(0) => Err(None),
        Ok(units) => Ok(units),
        Err(error) => Err(Some(error)),
    }
}

impl Rules {
    /// The rules of a match on this movement table.
    ///
    /// # Errors
    /// A table whose values no rule can use, by name ([`RulesError`]).
    pub fn take(m: &Movement) -> Result<Self, RulesError> {
        let (reach, step_up) = (m.jump_reach_m, m.step_up_m);
        if reach < step_up {
            return Err(RulesError::ReachBelowStepUp {
                reach_m: reach,
                step_up_m: step_up,
            });
        }
        if m.body_height_m < 2.0 * m.body_radius_m {
            return Err(RulesError::BodyTooShort {
                height_m: m.body_height_m,
                radius_m: m.body_radius_m,
            });
        }
        if m.foot_radius_m > m.body_radius_m {
            return Err(RulesError::FootTooWide {
                foot_m: m.foot_radius_m,
                radius_m: m.body_radius_m,
            });
        }
        let pull_in_range = m.pull_share_per_step > 0.0 && m.pull_share_per_step <= 1.0;
        if !pull_in_range {
            return Err(RulesError::PullShare(m.pull_share_per_step));
        }
        let allowance_in_range = m.step_allowance_m >= 0.0 && m.step_allowance_m <= step_up;
        if !allowance_in_range {
            return Err(RulesError::StepAllowance {
                allowance_m: m.step_allowance_m,
                step_up_m: step_up,
            });
        }
        let snap_in_range = m.snap_down_m >= 0.0 && m.snap_down_m <= step_up;
        if !snap_in_range {
            return Err(RulesError::SnapDown {
                snap_m: m.snap_down_m,
                step_up_m: step_up,
            });
        }
        let sprint_units =
            u64::from(units(m.sprint_time_from_full_s).map_err(RulesError::SprintTime)?);
        let refill_units =
            u64::from(units(m.stamina_refill_time_s).map_err(RulesError::RefillTime)?);
        // Two u32 multiplied fit a u64; the content's ranges keep `full` far below 2^53, where the double is exact.
        let stamina_full = sprint_units * refill_units;
        let g = m.gravity_m_per_s2;
        Ok(Self {
            walk: m.walk_speed_m_per_s as f32,
            sprint: (m.walk_speed_m_per_s * m.sprint_multiplier) as f32,
            ads: (m.walk_speed_m_per_s * m.ads_speed_multiplier) as f32,
            ease_up: share_per_tick(core::f64::consts::LN_10 / m.speed_up_time_to_90pct_s, DT_S),
            ease_stop: share_per_tick(core::f64::consts::LN_10 / m.stop_time_to_10pct_s, DT_S),
            air_share: (m.air_control_pct / 100.0) as f32,
            radius: m.body_radius_m as f32,
            height: m.body_height_m as f32,
            foot_radius: m.foot_radius_m as f32,
            step_up: step_up as f32,
            g_dt: (g * DT_S) as f32,
            half_g_dt2: (g * DT_S * DT_S / 2.0) as f32,
            take_off: (2.0 * g * (reach - step_up)).sqrt() as f32,
            inv_two_g: (1.0 / (2.0 * g)) as f32,
            stamina_full,
            drain: u64::from(UNITS_PER_TICK) * refill_units,
            refill: u64::from(UNITS_PER_TICK) * sprint_units,
            returns_at: (stamina_full as f64 * m.sprint_returns_at_pct / 100.0).ceil() as u64,
            push_share: share_per_tick(m.push_strength_per_s, DT_S),
            push_reach: (2.0 * m.push_radius_m) as f32,
            pull_share: m.pull_share_per_step as f32,
            pull_done_under: m.pull_done_under_m as f32,
            pull_to_foot: (m.pull_to_foot_pct / 100.0) as f32,
            step_allowance: m.step_allowance_m as f32,
            snap_down: m.snap_down_m as f32,
            push_height_gap: m.push_height_gap_m as f32,
            push_sideways_share: m.push_sideways_share as f32,
            push_keep_going_share: m.push_keep_going_share as f32,
            push_most_per_step: m.push_most_m_per_step as f32,
            push_head_on_gate: m.push_head_on_gate_m_per_s as f32,
            push_keep_going_gate: m.push_keep_going_gate_m_per_s as f32,
            headroom_feet: HEADROOM_FEET,
        })
    }

    /// The same rules with the other headroom mode, for the tests that run each layout under both.
    #[cfg(test)]
    #[must_use]
    pub fn with_headroom_feet(self, headroom_feet: HeadroomFeet) -> Self {
        Self {
            headroom_feet,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The made-up movement table of the content crate's tests (never his values).
    fn table() -> Movement {
        let tree = content::json::parse(content::movement::TEST_TEXT.as_bytes())
            .expect("the test text parses");
        content::movement::read(&tree).expect("the test text is a movement table")
    }

    /// Every rule by name, as a number: the destructuring names every field, so a field added to `Rules` and left
    /// out here does not compile.
    fn numbers(rules: &Rules) -> Vec<(&'static str, f64)> {
        let Rules {
            walk,
            sprint,
            ads,
            ease_up,
            ease_stop,
            air_share,
            radius,
            height,
            foot_radius,
            step_up,
            g_dt,
            half_g_dt2,
            take_off,
            inv_two_g,
            stamina_full,
            drain,
            refill,
            returns_at,
            push_share,
            push_reach,
            pull_share,
            pull_done_under,
            pull_to_foot,
            step_allowance,
            snap_down,
            push_height_gap,
            push_sideways_share,
            push_keep_going_share,
            push_most_per_step,
            push_head_on_gate,
            push_keep_going_gate,
            headroom_feet,
        } = rules;
        let float = |value: &f32| f64::from(*value);
        let whole = |value: &u64| *value as f64;
        vec![
            ("walk", float(walk)),
            ("sprint", float(sprint)),
            ("ads", float(ads)),
            ("ease_up", float(ease_up)),
            ("ease_stop", float(ease_stop)),
            ("air_share", float(air_share)),
            ("radius", float(radius)),
            ("height", float(height)),
            ("foot_radius", float(foot_radius)),
            ("step_up", float(step_up)),
            ("g_dt", float(g_dt)),
            ("half_g_dt2", float(half_g_dt2)),
            ("take_off", float(take_off)),
            ("inv_two_g", float(inv_two_g)),
            ("stamina_full", whole(stamina_full)),
            ("drain", whole(drain)),
            ("refill", whole(refill)),
            ("returns_at", whole(returns_at)),
            ("push_share", float(push_share)),
            ("push_reach", float(push_reach)),
            ("pull_share", float(pull_share)),
            ("pull_done_under", float(pull_done_under)),
            ("pull_to_foot", float(pull_to_foot)),
            ("step_allowance", float(step_allowance)),
            ("snap_down", float(snap_down)),
            ("push_height_gap", float(push_height_gap)),
            ("push_sideways_share", float(push_sideways_share)),
            ("push_keep_going_share", float(push_keep_going_share)),
            ("push_most_per_step", float(push_most_per_step)),
            ("push_head_on_gate", float(push_head_on_gate)),
            ("push_keep_going_gate", float(push_keep_going_gate)),
            (
                "headroom_feet",
                if *headroom_feet == HeadroomFeet::SandboxTest {
                    0.0
                } else {
                    1.0
                },
            ),
        ]
    }

    /// Each of the 28 fields reaches its own rules and no other: one value moved at a time changes exactly the rules
    /// that use it (rust-reviewer 11.7 row 2; its plant, the stop's easing taken from the speed-up time, is red here).
    #[test]
    fn each_field_reaches_its_own_rules_and_no_other() {
        type Move = fn(&mut Movement);
        #[rustfmt::skip]
        let moves: [(&str, Move, &[&str]); 28] = [
            ("walk_speed_m_per_s", |m| m.walk_speed_m_per_s *= 0.9, &["walk", "sprint", "ads"]),
            ("sprint_multiplier", |m| m.sprint_multiplier *= 0.9, &["sprint"]),
            ("speed_up_time_to_90pct_s", |m| m.speed_up_time_to_90pct_s *= 0.9, &["ease_up"]),
            ("stop_time_to_10pct_s", |m| m.stop_time_to_10pct_s *= 0.9, &["ease_stop"]),
            ("air_control_pct", |m| m.air_control_pct *= 0.9, &["air_share"]),
            ("body_radius_m", |m| m.body_radius_m *= 0.9, &["radius"]),
            ("jump_reach_m", |m| m.jump_reach_m *= 0.9, &["take_off"]),
            ("gravity_m_per_s2", |m| m.gravity_m_per_s2 *= 0.9, &["g_dt", "half_g_dt2", "take_off", "inv_two_g"]),
            ("step_up_m", |m| m.step_up_m *= 0.9, &["step_up", "take_off"]),
            ("sprint_time_from_full_s", |m| m.sprint_time_from_full_s *= 0.9, &["stamina_full", "refill", "returns_at"]),
            ("stamina_refill_time_s", |m| m.stamina_refill_time_s *= 0.9, &["stamina_full", "drain", "returns_at"]),
            ("sprint_returns_at_pct", |m| m.sprint_returns_at_pct *= 0.9, &["returns_at"]),
            ("push_strength_per_s", |m| m.push_strength_per_s *= 0.9, &["push_share"]),
            ("push_radius_m", |m| m.push_radius_m *= 0.9, &["push_reach"]),
            ("ads_speed_multiplier", |m| m.ads_speed_multiplier *= 0.9, &["ads"]),
            ("body_height_m", |m| m.body_height_m *= 0.9, &["height"]),
            ("foot_radius_m", |m| m.foot_radius_m *= 0.9, &["foot_radius"]),
            ("pull_share_per_step", |m| m.pull_share_per_step *= 0.9, &["pull_share"]),
            ("pull_done_under_m", |m| m.pull_done_under_m *= 0.9, &["pull_done_under"]),
            ("pull_to_foot_pct", |m| m.pull_to_foot_pct *= 0.9, &["pull_to_foot"]),
            ("step_allowance_m", |m| m.step_allowance_m *= 0.9, &["step_allowance"]),
            ("snap_down_m", |m| m.snap_down_m *= 0.9, &["snap_down"]),
            ("push_height_gap_m", |m| m.push_height_gap_m *= 0.9, &["push_height_gap"]),
            ("push_sideways_share", |m| m.push_sideways_share *= 0.9, &["push_sideways_share"]),
            ("push_keep_going_share", |m| m.push_keep_going_share *= 0.9, &["push_keep_going_share"]),
            ("push_most_m_per_step", |m| m.push_most_m_per_step *= 0.9, &["push_most_per_step"]),
            ("push_head_on_gate_m_per_s", |m| m.push_head_on_gate_m_per_s *= 0.9, &["push_head_on_gate"]),
            ("push_keep_going_gate_m_per_s", |m| m.push_keep_going_gate_m_per_s *= 0.9, &["push_keep_going_gate"]),
        ];
        let base = numbers(&Rules::take(&table()).expect("the test table plays"));
        let mut named: Vec<&str> = moves.iter().map(|(field, _, _)| *field).collect();
        named.sort_unstable();
        let mut fields: Vec<&str> = content::movement::FIELDS
            .iter()
            .map(|field| field.name)
            .collect();
        fields.sort_unstable();
        assert_eq!(
            named, fields,
            "every field of the movement kind is moved once"
        );
        for (field, change, uses) in moves {
            let mut moved = table();
            change(&mut moved);
            let rules =
                Rules::take(&moved).unwrap_or_else(|error| panic!("{field}: {}", error.message()));
            let changed: Vec<&str> = numbers(&rules)
                .iter()
                .zip(&base)
                .filter(|((_, now), (_, before))| now.to_bits() != before.to_bits())
                .map(|((name, _), _)| *name)
                .collect();
            assert_eq!(changed, uses, "{field}");
        }
    }

    /// What the ranges let through and no rule can use is refused, each by its own name; each limit itself plays.
    #[test]
    fn what_no_rule_can_use_is_refused_by_name() {
        type Move = fn(&mut Movement);
        let refused = |change: Move| {
            let mut moved = table();
            change(&mut moved);
            Rules::take(&moved).map(|_| ())
        };
        let plays = |change: Move| refused(change).is_ok();
        assert_eq!(
            refused(|m| m.jump_reach_m = 0.4),
            Err(RulesError::ReachBelowStepUp {
                reach_m: 0.4,
                step_up_m: 0.5
            })
        );
        assert!(plays(|m| m.jump_reach_m = m.step_up_m));
        assert_eq!(
            refused(|m| m.sprint_time_from_full_s = 1e-9),
            Err(RulesError::SprintTime(None))
        );
        assert_eq!(
            refused(|m| m.stamina_refill_time_s = 1e-9),
            Err(RulesError::RefillTime(None))
        );
        assert_eq!(
            refused(|m| m.stamina_refill_time_s = f64::INFINITY),
            Err(RulesError::RefillTime(Some(DurationError::NotFinite)))
        );
        assert_eq!(
            refused(|m| m.body_height_m = 0.9),
            Err(RulesError::BodyTooShort {
                height_m: 0.9,
                radius_m: 0.5
            })
        );
        assert!(plays(|m| m.body_height_m = 1.0));
        assert_eq!(
            refused(|m| m.foot_radius_m = 0.6),
            Err(RulesError::FootTooWide {
                foot_m: 0.6,
                radius_m: 0.5
            })
        );
        assert!(plays(|m| m.foot_radius_m = m.body_radius_m));
        assert_eq!(
            refused(|m| m.pull_share_per_step = 0.0),
            Err(RulesError::PullShare(0.0))
        );
        assert_eq!(
            refused(|m| m.pull_share_per_step = 1.5),
            Err(RulesError::PullShare(1.5))
        );
        assert!(plays(|m| m.pull_share_per_step = 1.0));
        assert_eq!(
            refused(|m| m.step_allowance_m = 0.6),
            Err(RulesError::StepAllowance {
                allowance_m: 0.6,
                step_up_m: 0.5
            })
        );
        assert_eq!(
            refused(|m| m.step_allowance_m = -0.1),
            Err(RulesError::StepAllowance {
                allowance_m: -0.1,
                step_up_m: 0.5
            })
        );
        assert!(plays(|m| m.step_allowance_m = m.step_up_m));
        assert_eq!(
            refused(|m| m.snap_down_m = 0.6),
            Err(RulesError::SnapDown {
                snap_m: 0.6,
                step_up_m: 0.5
            })
        );
        assert!(plays(|m| m.snap_down_m = 0.0));
        // Each refusal says what is wrong in its own words.
        let errors = [
            RulesError::ReachBelowStepUp {
                reach_m: 0.4,
                step_up_m: 0.5,
            },
            RulesError::SprintTime(None),
            RulesError::RefillTime(None),
            RulesError::BodyTooShort {
                height_m: 0.9,
                radius_m: 0.5,
            },
            RulesError::FootTooWide {
                foot_m: 0.6,
                radius_m: 0.5,
            },
            RulesError::PullShare(0.0),
            RulesError::StepAllowance {
                allowance_m: 0.6,
                step_up_m: 0.5,
            },
            RulesError::SnapDown {
                snap_m: 0.6,
                step_up_m: 0.5,
            },
        ];
        let mut messages: Vec<String> = errors.iter().map(RulesError::message).collect();
        messages.sort_unstable();
        messages.dedup();
        assert_eq!(messages.len(), errors.len());
    }

    /// The shares against the correctly rounded f32 of 1 - exp(-x), x the double the rule works out
    /// (`.checkpoints/cs4c-soldier/share-ref-c3.py`, Python decimal at 60 digits): the easing over its times to a
    /// tenth from 0.01 to 5 s, the push over strengths from 0.5 to 100 a second, each within one f32 step. That holds
    /// for every easing time in (0, 5] and for a push strength of 0 or at least 1e-7 a second (rust-reviewer's block
    /// 17, Q4, measured); below that, libm's exp gives 1 + x and the share falls on a coarser grid: the same bits on
    /// every target, only less exact.
    #[test]
    fn the_shares_are_within_one_step_of_a_decimal_reference() {
        #[rustfmt::skip]
        let easing: [(f64, u32); 8] = [
            (0.01, 0x3f7f_e195), (0.05, 0x3f48_d8b2), (0.1, 0x3f09_2ce2), (0.25, 0x3e87_59e6),
            (0.5, 0x3e11_b82b), (1.0, 0x3d97_4f10), (2.0, 0x3d1a_363b), (5.0, 0x3c79_954b),
        ];
        #[rustfmt::skip]
        let push: [(f64, u32); 5] = [
            (0.5, 0x3c87_66e0), (1.0, 0x3d06_4869), (10.0, 0x3e91_22cf), (50.0, 0x3f4f_a5d9), (100.0, 0x3f76_de12),
        ];
        let step = |got: f32, reference: u32| got.to_bits().abs_diff(reference);
        for (time, reference) in easing {
            let mut moved = table();
            moved.speed_up_time_to_90pct_s = time;
            moved.stop_time_to_10pct_s = time;
            let rules = Rules::take(&moved).expect("plays");
            assert!(
                step(rules.ease_up, reference) <= 1,
                "{time}: {:08x}",
                rules.ease_up.to_bits()
            );
            assert_eq!(rules.ease_up.to_bits(), rules.ease_stop.to_bits(), "{time}");
        }
        for (strength, reference) in push {
            let mut moved = table();
            moved.push_strength_per_s = strength;
            let rules = Rules::take(&moved).expect("plays");
            assert!(
                step(rules.push_share, reference) <= 1,
                "{strength}: {:08x}",
                rules.push_share.to_bits()
            );
        }
    }

    /// The stamina in whole units: from full, a sprint lasts its time in ticks, and a refill from empty its time.
    #[test]
    fn the_stamina_counts_whole_units() {
        let rules = Rules::take(&table()).expect("plays");
        // The test table: 8 s of sprint, 4 s of refill, the sprint back at 25 % (24,000 and 12,000 units).
        assert_eq!(rules.stamina_full, 24_000 * 12_000);
        assert_eq!(rules.stamina_full / rules.drain, 8 * u64::from(TICK_HZ));
        assert_eq!(rules.stamina_full / rules.refill, 4 * u64::from(TICK_HZ));
        assert_eq!(rules.returns_at, rules.stamina_full / 4);
        assert_eq!(rules.headroom_feet, HEADROOM_FEET);
        assert_eq!(
            rules
                .clone()
                .with_headroom_feet(HeadroomFeet::OwnTopNeverWall)
                .headroom_feet,
            HeadroomFeet::OwnTopNeverWall
        );
    }
}
