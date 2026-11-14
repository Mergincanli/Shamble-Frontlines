//! npt-room's bite list (bites builds only; U4E-Q3 (a), U4E-Q13 (a)): pinned, and each PM-form id checked against
//! marks.json as server/tests/schedule.rs does, the one id outside that form exempt by name (no mark judges catalogue
//! item 5; check.mjs's bite-home grep reads only `PM-<ADR>-<row>/<name>`, so npt-server's tests hold its one home).
#![cfg(feature = "bites")]

const OWNER_TO_ALL: &str = "catalogue-5/owner-to-all";

#[test]
fn the_rooms_bites_are_pinned_and_every_pm_id_is_marks_jsons() {
    assert_eq!(
        npt_room::BITES,
        [
            "PM-0001-18/no-fallback",
            "PM-0001-58/lt-ack",
            "PM-0001-58/lt-cmd-seq",
            "PM-0001-58/lt-press",
            "PM-0001-62/server-echo",
            "PM-0001-22/unbounded",
            OWNER_TO_ALL,
        ]
    );
    let marks = include_str!("../../../marks/marks.json");
    for id in npt_room::BITES {
        if id == OWNER_TO_ALL {
            assert!(!marks.contains(id), "{id} is no mark's");
            continue;
        }
        assert!(
            marks.contains(&format!("\"{id}\"")),
            "{id} is not in marks.json"
        );
    }
}
