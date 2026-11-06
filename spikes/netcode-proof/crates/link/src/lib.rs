//! npt-link: the thin transport interface of the netcode proof test (plan §4.7; ADR 0001:431-435; ADR 0002:188).
//!
//! Poll-based, with every call taking `now`, so L1 can drive it on simulated time; channels carry Datagram, Control and
//! FullState traffic. Its public surface (`Link`, `Listener`, `Channel`, `Recv`, `SendResult`, `Loopback`) is built in
//! unit U2. No transport crate, no async runtime, no clock: it builds for wasm32.
