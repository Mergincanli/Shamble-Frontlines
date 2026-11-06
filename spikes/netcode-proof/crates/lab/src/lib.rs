//! npt-lab: ADR 0002's devtools analogue for the netcode proof test (plan §4.7; engine EA-3, EA-11).
//!
//! `SimClock` (integer ns) and `L1Scenario`, which steps one server and 48 headless clients on one simulated clock in a
//! fixed order per instant (due packets, the server's I/O step, the server tick if due, due client frames, due client
//! ticks); the x86-64 re-decode and replay tool for the Arm rule (`ReplayX86`); the native-against-Wasm hash comparer
//! (`HashCompare`). Nothing depends on this crate. Built in units U2 and U4.
