# U4d2 review round 1: engine-architect seat (the boundary)

Read only. Read copy: /home/user/Shamble-Frontlines/.gate/u4d2-r1 (51520b4 + U4d2's snapshot patch). Base copy:
/home/user/Shamble-Frontlines/.gate/u4d2-prefix (51520b4). No cargo or node runs by this seat; runs I would want are
listed for the main session at the end.

## Plan
1. Read the packet's heads and the judged-against texts (u4d2-packet.md): §5 U4d2, §4.1, §4.11, §4.14, §4.15, Lanes,
   §7.1; plan.md §4.1 hash-wasm line, §4.2, §4.5, §4.7's npt-lab line; advisor rulings named; carries routed to U4d2.
2. Edges: crates/hash-wasm/Cargo.toml, crates/lab/Cargo.toml, the workspace Cargo.toml; check.mjs's deps step (LEAF,
   WASM_CRATES, rules), against the plan's edges.
3. The Rust-to-JS boundary: hash-wasm's lib.rs (seven exports, status constants), loader.mjs (export list, one home),
   check.mjs's wasm step's use of it; reuse by U7's Chromium leg and wasm-opt (C-U7-1).
4. HashCompare and TickHash (lab): shape, Empty, reuse by U4r's -30 pre-check (condition 2).
5. Public items: every pub item added (Rust, check.mjs exports, loader exports) from the diff, each with its reason.
6. Seam (u4-build-plan.md §4.2) and lane (Lanes): the file list of the change vs the lane; anything built that no line
   asks for.
7. Verdict and the run list for the main session.

## Notes (one line per finding, as found)
