//! npt-client: the client endpoint of the netcode proof test (plan E7-E10, §4.7; ADR 0002:190, 313-373).
//!
//! One `input` call and one `step` call per tick: received messages arrive with their channel and the host's arrival
//! stamp, and `step` returns the datagrams and reliable messages to send, the block for the page, and when the next
//! tick is due. Prediction and reconciliation, interpolation, clock sync, the aim cast and the block writer live here.
//! No bindings, no transport, no clock; native for the bots and wasm32 for the Worker. Built in units U4 and U7.
