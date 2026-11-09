//! npt-client: the client endpoint of the netcode proof test (plan E7-E10, §4.7; ADR 0002:190, 313-373).
//!
//! One `input` call and one `step` call per tick: received messages arrive with their channel and the host's arrival
//! stamp, and `step` returns the datagrams and reliable messages to send, the block for the page, and when the next
//! tick is due. Prediction and reconciliation, interpolation, clock sync, the aim cast and the block writer live here.
//! No bindings, no transport, no clock; native for the bots and wasm32 for the Worker. Built in units U4 and U7.

/// The bite ids this crate plants (bites builds only). u4-build-plan.md §4.14 plants each bite where its code lives:
/// PM-0001-17/client-rounding's site is the client's call into the wire grid, so its id lives here and not in npt-sim's
/// grid. U4e's endpoint takes the grid for its every call into it from [`grid_with_bite`].
#[cfg(feature = "bites")]
pub const BITES: [&str; 1] = ["PM-0001-17/client-rounding"];

/// The grid a client with the planted defect `bite` uses for its every call into it (bites builds only): for
/// PM-0001-17/client-rounding ("a client quantiser that rounds differently from the server's") the grid's one defect,
/// `npt_sim::grid::Config::floor_rounding`; `None` for an id this crate does not plant.
#[cfg(feature = "bites")]
pub fn grid_with_bite(bite: &str) -> Option<npt_sim::grid::Config> {
    BITES
        .contains(&bite)
        .then(npt_sim::grid::Config::floor_rounding)
}
