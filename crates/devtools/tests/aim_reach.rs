//! The aim's reach is what the aim point's own number types hold. Mustafa's note on the plan's sixth departure
//! (2026-11-26): "until the aim codec widens, the loader refuses a playable map larger than its reach". The content
//! crate holds the reach as two counts of centimetres and may depend on nothing, so it cannot name the aim point;
//! this test ties the two. Every edge of the reach fits the field that carries it, and one centimetre past the
//! type's own end does not: when the codec widens a field, "one past" fits, this test fails, and the reach in
//! `crates/content/src/map.rs` moves with it (STATUS, 4B-AIM-REACH, names when).

use content::map::{AIM_HEIGHT_MAX_CM, AIM_REACH_CM};
use protocol::messages::AimPoint;

/// Whether `cm` fits the number type of the field that `field` reads. A field that is no whole number does not
/// compile here.
fn fits<T: TryFrom<i32>>(_field: fn(&AimPoint) -> T, cm: i32) -> bool {
    T::try_from(cm).is_ok()
}

/// `cm` in a field's own type.
fn cm<T: TryFrom<i32>>(cm: i32) -> T {
    T::try_from(cm).unwrap_or_else(|_| panic!("{cm} cm does not fit the aim point's field"))
}

const WIDENED: &str = "the aim codec widened: move the reach in crates/content/src/map.rs";

#[test]
fn the_reach_is_what_the_aim_points_fields_hold() {
    // An aim point holds the reach's two far corners, each field in its own type.
    let low = AimPoint {
        x: cm(-AIM_REACH_CM),
        y: cm(0),
        z: cm(-AIM_REACH_CM),
    };
    let high = AimPoint {
        x: cm(AIM_REACH_CM),
        y: cm(AIM_HEIGHT_MAX_CM),
        z: cm(AIM_REACH_CM),
    };
    assert_eq!(
        (i32::from(low.x), i32::from(low.y), i32::from(low.z)),
        (-AIM_REACH_CM, 0, -AIM_REACH_CM)
    );
    assert_eq!(
        (i32::from(high.x), i32::from(high.y), i32::from(high.z)),
        (AIM_REACH_CM, AIM_HEIGHT_MAX_CM, AIM_REACH_CM)
    );
    // One centimetre past the reach does not fit, upward on every axis and below y 0 (the aim's zero; the bounds'
    // bottom, the ground, lies at or above it): a wider field takes it.
    assert!(
        !fits(|aim| aim.x, AIM_REACH_CM + 1) && !fits(|aim| aim.z, AIM_REACH_CM + 1),
        "{WIDENED}"
    );
    assert!(
        !fits(|aim| aim.y, AIM_HEIGHT_MAX_CM + 1) && !fits(|aim| aim.y, -1),
        "{WIDENED}"
    );
    // Downward on x and z the type holds one centimetre more than the reach (-32,768), which the rule leaves out so
    // that the reach is the same both ways; one past the type's own end does not fit.
    assert!(fits(|aim| aim.x, -AIM_REACH_CM - 1) && fits(|aim| aim.z, -AIM_REACH_CM - 1));
    assert!(
        !fits(|aim| aim.x, -AIM_REACH_CM - 2) && !fits(|aim| aim.z, -AIM_REACH_CM - 2),
        "{WIDENED}"
    );
    // That one centimetre is "no aim in this command" (netcode-architect's block 8.3): its x lies one past the reach,
    // so no map's bounds can hold it.
    assert_eq!(i32::from(AimPoint::NONE.x), -(AIM_REACH_CM + 1));
    assert_eq!((AimPoint::NONE.y, AimPoint::NONE.z), (0, 0));
}
