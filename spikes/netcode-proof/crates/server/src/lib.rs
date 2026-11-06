//! npt-server: the room process of the netcode proof test, as a library (plan §4.1, §4.4, §4.6; ADR 0004 part 5).
//!
//! The S17 loop through the pure function `due_ticks`, the tick thread and the I/O runtime (where every inbound message
//! is decoded), the wall-time limits from an injected `now`, the per-slot local control sockets, metrics and logs, and
//! the panic catch. npt-lab imports this library so L1 runs the real server code on simulated time. Built in U4 and U6.
