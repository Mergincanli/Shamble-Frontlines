//! npt-net: the native transports of the netcode proof test (plan E6, E15; ADR 0001 part 10).
//!
//! `WtServer` and `WtClient` over quinn through wtransport or web-transport-quinn (one feature each), `WsServer` and
//! `WsClient`, and L2: `ImpairSocket` under quinn's socket and the `DelayStream` wrapper in the WebSocket accept path.
//! Built in units U3 and U6. npt-client never depends on this crate (plan §4.2).

// The Linux-faithful TCP model DelayStream runs on (U3a, U3c). Its handshake and reconnect paths (`connect`) are
// used by the tests and the fixes harness only until U6's reconnects.
#[cfg_attr(not(test), allow(dead_code))]
mod tcp_model;

// Fix 1's send gate (U3c): U6's WsServer calls it on every tick; until then the mechanism tests and the fixes harness
// do.
#[cfg_attr(not(test), allow(dead_code))]
mod ws_gate;

// The WebSocket server's TLS and tungstenite configurations (U3c): U6's WsServer builds them; until then the loopback
// tests do.
#[cfg_attr(not(test), allow(dead_code))]
mod ws_config;

// L2's impairment layer (U3b, U3c): ImpairSocket, DelayStream and L2Config, plan §4.7's public items.
mod l2;
pub use l2::{DelayStream, ImpairSocket, L2Config};

// U6b: the transports (u6b-brief.md Blocks 1-4): the connection cap, the certificate pin, what both servers share, and
// the servers and clients themselves (plan §4.7's WtServer, WtClient, WsServer, WsClient).
mod cap;
mod pin;
mod session;
pub use cap::ConnCap;
pub use session::{Accepted, Listen};
#[cfg(feature = "wt-wtq")]
mod wt_server;
#[cfg(feature = "wt-wtq")]
pub use wt_server::{WtClient, WtServer};
mod ws_server;
pub use ws_server::{WsClient, WsServer};

// The plain WebSocket path's kernel readings for fix 1's gate, through npt-tcpinfo (U3c 1.8); WsServer calls them at U6.
#[cfg_attr(not(test), allow(dead_code))]
mod tcp_kernel;
