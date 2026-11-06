//! npt-net: the native transports of the netcode proof test (plan E6, E15; ADR 0001 part 10).
//!
//! `WtServer` and `WtClient` over quinn through wtransport or web-transport-quinn (one feature each), `WsServer` and
//! `WsClient`, and L2: `ImpairSocket` under quinn's socket and the `DelayStream` wrapper in the WebSocket accept path.
//! Built in units U3 and U6. npt-client never depends on this crate (plan §4.2).

// The Linux-faithful TCP model DelayStream runs on (U3a); DelayStream itself joins in U3c, until then only the tests
// use it.
#[cfg_attr(not(test), allow(dead_code))]
mod tcp_model;
