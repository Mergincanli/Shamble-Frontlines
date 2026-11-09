//! npt-client's planted defects (bites builds only; u4-build-plan.md §4.14: each planted where its code lives). U4s
//! plants PM-0001-17/client-rounding's id here, at the client's call into the wire grid; U4e's endpoint adds the rest.
#![cfg(feature = "bites")]

use npt_sim::grid::{Config, POSITION_XY};

#[test]
fn pm_0001_17_client_rounding_is_planted_here_alone_and_moves_about_half_of_all_positions() {
    let id = "PM-0001-17/client-rounding";
    assert_eq!(npt_client::BITES, [id]);
    let bitten = npt_client::grid_with_bite(id).expect("the client plants -17's");
    assert_eq!(bitten, Config::floor_rounding());
    // No id of another site: the own-copy snap's (-41), the encoder's (-57), an unknown one.
    for other in [
        "PM-0001-41/server-quant-differs",
        "PM-0001-57/pos-4cm",
        "PM-0001-57/yaw-12bit",
        "PM-0001-57/pitch-8bit",
        "PM-0001-17/no-such-bite",
    ] {
        assert!(npt_client::grid_with_bite(other).is_none(), "{other}");
    }
    // u4-build-plan.md §4.14's unit proof: the floor variant differs from the wire grid on about half of all position
    // values (from 0 to 20 m, in sixteenths of a centimetre).
    let wire = Config::default();
    let (mut differ, mut all) = (0u32, 0u32);
    for k in 0..2_000 {
        for sixteenth in 0..16 {
            let x = ((f64::from(k) + f64::from(sixteenth) / 16.0) / 100.0) as f32;
            differ += u32::from(bitten.to_grid(POSITION_XY, x) != wire.to_grid(POSITION_XY, x));
            all += 1;
        }
    }
    let share = f64::from(differ) / f64::from(all);
    assert!((0.45..=0.6).contains(&share), "{share}");
}
