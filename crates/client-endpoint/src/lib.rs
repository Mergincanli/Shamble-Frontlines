//! The client endpoint, with no bindings, no transport, no async runtime and no clock: prediction and
//! reconciliation, the interpolation buffer, snapshot decoding, the aim cast, the block writer and the block's layout
//! table; built for Wasm inside `sim-wasm` and natively for the headless clients (ADR 0002 parts 3 and 5). So far it
//! holds the layout table and its hash (`layout`, foundation step 2.4); the rest enters from the skeleton step (2.5).

pub mod layout;
#[cfg(test)]
mod layout_ts;
