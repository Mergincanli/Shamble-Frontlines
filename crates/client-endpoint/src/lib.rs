//! The client endpoint, with no bindings, no transport, no async runtime and no clock: prediction and
//! reconciliation, the interpolation buffer, snapshot decoding, the aim cast, the block writer and the block's layout
//! table; built for Wasm inside `sim-wasm` and natively for the headless clients (ADR 0002 parts 3 and 5). So far it
//! holds the layout table and its hash (`layout`), the block writer (`block`), the local schedule (`schedule`), and
//! its side of the message exchange: the join, each tick's command from the page's newest post (`post`, `press`)
//! and the room's snapshots (`endpoint`). Prediction and the interpolation buffer enter with the soldier.

pub mod block;
pub mod endpoint;
pub mod layout;
#[cfg(test)]
mod layout_ts;
pub mod post;
pub mod press;
pub mod schedule;
