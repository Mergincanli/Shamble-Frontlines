# rust-reviewer, cs3b round 1 (head bbbf09e4, base 28254a73)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/rust-reviewer-cs3b (detached at bbbf09e4).
Memory: none found (.claude/agent-memory-local/rust-reviewer absent); 3a scratch in .gate/rust-reviewer-cs3a.

## Plan
1. Read the diff of the Rust files (protocol, sim/world.rs, room, client-endpoint, sim-wasm, devtools f32_ops, manifests).
2. Q1 panics from link bytes: read protocol/room/endpoint for index, slice, copy_within/rotate, expect, overflow,
   reserve; run cargo clippy --locked --workspace --all-targets; check where the 4 denied lints apply, what they miss.
3. Q2 allocation: read both counting allocators and the tests; find paths their inputs never take.
4. Q3 determinism: usize in hashed/sent values, `as` casts, iteration order, floats; rules_scan; f64 host time.
5. Q4 idiom/API: pub items, must_use, const fn, error types, LocalMatch generic + GAT, Loopback, queue rotate.
6. Q5 decoder test generator: xorshift64*, Kind::valid, mutate, catch_unwind/black_box/alloc window.
7. Q6 3A-FROM-F64: the 20 cases, --check, cargo test -p devtools.
8. Q7 wasm size: build if the bindings tool exists; judge cuts.
9. Answer (<= 40 lines), DONE.

## Findings (one line each, as found)
