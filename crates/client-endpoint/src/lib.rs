//! The client endpoint, with no bindings, no transport, no async runtime and no clock: prediction and
//! reconciliation, the interpolation buffer, snapshot decoding, the aim cast, the block writer and the block's layout
//! table; built for Wasm inside `sim-wasm` and natively for the headless clients (ADR 0002 parts 3 and 5). So far it
//! holds the layout table and its hash (`layout`, foundation step 2.4), the block writer for the header (`block`) and
//! the local schedule (`schedule`), both from the skeleton step's first half (2.5); the rest enters with its second.

pub mod block;
pub mod layout;
#[cfg(test)]
mod layout_ts;
pub mod schedule;
