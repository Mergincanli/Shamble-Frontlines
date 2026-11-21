//! Moved whole from npt-server's tests/loopback.rs (U4f, section 6 item 4; r-b2's follow-up): it reads every crate's
//! bite list, `npt_client::BITES` among them, and npt-lab's bites feature turns on every one (lab/Cargo.toml), while
//! npt-server's does not turn on npt-client's, so `cargo test -p npt-server --all-features` could not compile it.
#![cfg(feature = "bites")]

/// U4E-Q13 (a): `catalogue-5/owner-to-all` is in npt-room's list and no other crate's (check.mjs's bite-home grep
/// reads only the `PM-<ADR>-<row>/<name>` form, so this test holds its one home); no id is in two lists.
#[test]
fn catalogue_5_owner_to_all_lives_in_npt_room_alone_and_no_id_is_in_two_lists() {
    let id = "catalogue-5/owner-to-all";
    // U4f adds npt-sim's two -21 check bites, a list apart from World::BITES.
    let lists: [(&str, Vec<&str>); 7] = [
        ("npt_room::BITES", npt_room::BITES.to_vec()),
        ("npt_proto::BITES", npt_proto::BITES.to_vec()),
        ("npt_sim::World::BITES", npt_sim::World::BITES.to_vec()),
        (
            "npt_sim::World::CHECK_BITES",
            npt_sim::World::CHECK_BITES.to_vec(),
        ),
        (
            "npt_sim::OWN_COPY_SNAP_BITES",
            npt_sim::OWN_COPY_SNAP_BITES.to_vec(),
        ),
        ("npt_server::BITES", npt_server::BITES.to_vec()),
        ("npt_client::BITES", npt_client::BITES.to_vec()),
    ];
    let homes: Vec<&str> = lists
        .iter()
        .filter(|l| l.1.contains(&id))
        .map(|l| l.0)
        .collect();
    assert_eq!(homes, ["npt_room::BITES"]);
    let mut all: Vec<&str> = lists
        .iter()
        .filter(|l| l.0 != "npt_sim::World::BITES")
        .flat_map(|l| l.1.clone())
        .collect();
    // World::BITES lists the own-copy snap's id beside OWN_COPY_SNAP_BITES (world.rs:605): one site, two lists.
    all.extend(
        npt_sim::World::BITES
            .iter()
            .filter(|b| !npt_sim::OWN_COPY_SNAP_BITES.contains(b)),
    );
    let n = all.len();
    all.sort_unstable();
    all.dedup();
    assert_eq!(all.len(), n, "an id in two lists");
}
