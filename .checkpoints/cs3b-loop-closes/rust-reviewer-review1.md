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
- [evidence] copy at bbbf09e4; clippy --locked --workspace --all-targets: 0 warnings, 0 errors; cargo fmt --check clean; cargo test --locked --workspace (debug): 106 passed.
- [evidence] decoder_inputs --release: 8 passed in 5.72 s; 10M+10M per type, 0 panics; plants found at input 17 (index) and 0 (alloc). Matches the context.
- [evidence] make_f32_vectors.py --check: OK (38 ops, 10068 cases); first 120,576 input bytes identical to main's file (cmp -n); the 16 pin lines identical; devtools tests pass.
- [evidence] players' Wasm rebuilt with the pinned wasm-bindgen 0.2.130: 56,717 raw / 23,023 gzip-9 / 19,497 brotli-11 (Node zlib), same as the main session's. Name section 11,037 B (19.5%; known follow-up 3A-NAME-SECTION).
- [M? size] local_match.rs:172-177: the two "cannot happen" panics format `{error:?}`/`{reason:?}`: pulls DecodeError/EndpointError/RejectReason Debug, debug_tuple, PadAdapter, u8 hex. Variant without the two `{:?}` (scratch copy, same flags): 53,596 raw (-3,121), 21,932 gzip (-1,091), 18,385 brotli (-1,112). Cheap, safe cut.
- [Low] context choice 9 says the 4 lints are denied in the three endpoint modules; post.rs:3-6 and press.rs:6-9 deny only indexing_slicing + arithmetic_side_effects (no unwrap/expect there today).
- [evidence Q1] lint probe (scratch copy .gate/exp-lint, plants before each `mod tests`): clippy flags all four in protocol (crate-wide), room lib.rs + queue.rs, endpoint.rs; only indexing+arithmetic in post.rs, press.rs; none in block.rs. Never flagged anywhere: split_at, copy_from_slice, rotate_right(n), copy_within, chunks(0), iter sum(), assert!/debug_assert!, Vec::with_capacity(n), `as` casts, pow, div_ceil, step_by(0). Real code uses only guarded forms (loopback.rs:51,59,72; queue.rs:93,103; codec.rs:300,306 constant caps).
- [evidence Q1/Q2] scratch probe .gate/exp-fuzz (own workspace, release + overflow checks + debug assertions): room 4 runs x 2M iterations (start ticks 0, 65,530, 131,069, u32::MAX-3M; valid/mutated/random datagrams, mutated joins, steps): 0 panics, 0 allocations in on_datagram and step after warm-up (decode errors, refused acks, stale, full queue all taken); on_control allocates per decoded Join token / JoinAccepted mode id (28-37 per 2M). Endpoint 3 x 2M: 0 panics, 0 allocations in build_input/on_datagram; on_control decodes a JoinAccepted (String) before refusing it as Unexpected (~49k per 2M; per control message, bounded 64 B).
- [evidence] LocalMatch<Rough> (generic over a TransportPair that loses 10%, holds back 10%, repeats 5% of datagrams): 100k ticks x 2 seeds, 0 panics, 99,997-99,999 commands run, room refused 0.
- [Low, netcode lane] corruption that still decodes (10% of datagrams mutated): one snapshot with a later server_tick is taken (endpoint.rs:236, any later tick), its acks then name a tick the room never sent and the room drops every input whole (lib.rs:177-185): commands run 16 of 100,000, refused_ack 91,812. Not reachable on an authenticated transport or the loopback; matters for 3c's link simulator (model corruption as loss) or a plausibility bound.
- [Low] devtools/Cargo.toml:14-16: client-endpoint, protocol, room are declared and unused (-W unused-crate-dependencies names all three); they serve steps 7-8, now 3c. No effect on the players' file.
- [Q3 clean] world.rs integer-only, fixed-width LE field by field; rules_scan 5/5; no HashMap/clock/float in protocol, room, endpoint, post, press; lengths go on the wire via try_from (`fits`); casts are const-asserted (room lib.rs:31-34, local_match.rs:27-30) or widening (block.rs:19,57). f64 host time: stored (local_match.rs:131-135) and only feeds bad_times, which is not in the block or any hash; LocalSchedule only says when the host calls next.
- [Q2 sound] both GlobalAlloc impls pass through to System with the caller's arguments, realloc/alloc_zeroed overridden (no double count), const-initialised Cell TLS (no lazy init, no Drop: try_with cannot fail, no recursion). step_alloc covers the loopback happy path only (1 command per Input, no refusals); the probe covers the rest.
- [evidence Q3] cross-target: the players' sim_wasm_bg.wasm in Node 22 (V8, initSync, not a browser) and sim_wasm::Sim natively, same 1M posts (LCG buttons/presses, every 97th post stale, every 89th time NaN): FNV-1a 64 over every tick's 44 block bytes equal, d4f18fe53057837e on both (and 8cad3c58179f9ba4 over 100k). Scripts: .gate/exp-fuzz/src/bin/blocks.rs, .gate/exp-fuzz/blocks.mjs.
- [evidence speed] 1M ticks x3: native 379/458/387 ns per tick (input+step+block hash, release); Wasm in Node 535/538/572 ns per input+step (hrtime around the two calls); Wasm memory 1,179,648 B before and after (no growth). 4 cores, load 0.24.
- [evidence Q7] nothing test-only reaches the players' file (buffer_bytes, counters, readout, state_hash, ChaCha8 absent from the name section); the only Debug impls present are the six the two panics reach (DecodeError, EndpointError, RejectReason, &u8 + hex, &dyn Debug, fmt::Error). Profile knobs do not help: LTO + 1 CGU 57,306 raw (+589), opt-level s + LTO 64,192, z + LTO 69,862.
