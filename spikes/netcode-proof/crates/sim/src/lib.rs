//! npt-sim: the netcode proof test's stand-in sim (plan E1, §4.5, §4.6; ADR 0001:1428-1435).
//!
//! Movement and hitscan, its soldiers driven by protocol clients (no bot logic here), the stress scene's counts and
//! S1's stand-ins as data behind one narrow seam, and a per-tick state hash. It follows ADR 0004 part 7's float
//! allow-list (enforced by this crate's clippy.toml) and builds for wasm32. Built in unit U4.
