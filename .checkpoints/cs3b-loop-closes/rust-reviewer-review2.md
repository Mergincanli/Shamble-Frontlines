# rust-reviewer, cs3b round 2 (head a85b9dbb; round 1 was bbbf09e4)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/rust-reviewer-cs3b2 (detached at a85b9dbb).
Round-1 copy: .gate/rust-reviewer-cs3b (bbbf09e4), scratch probes .gate/exp-fuzz, .gate/exp-lint (if still there).

## Plan
1. `git diff bbbf09e4 a85b9dbb -- crates` (stat, then per file).
2. Gates: cargo fmt --check; clippy --locked --workspace --all-targets -D warnings (native and wasm32);
   cargo test --locked --workspace (debug); the two any_bytes tests (time them, debug and release).
3. RR-1..RR-8 each against its fix (RR-1: rebuild the players' Wasm, 53,783 B? where the growth over 31,718 sits).
4. endpoint.rs refactor: new/set_post/block/block_len/write_block/build_input/on_datagram; Readout late/unwritten;
   local_match.rs; step_alloc.rs count.
5. queue.rs window (newest_run, take_due, ran) + lib.rs order: invariant under insert/take/refuse; rotate_*; tests.
6. world.rs shared_hash: scratch[SHARED_STATE_BYTES..] panic?; state_hash_alloc.rs; PlayerCommand doc.
7. layout.rs end_of/offsets, layout_ts.rs BLOCK_BYTES, codec.rs Join order + unit cases, decoder_inputs.rs caps/generators.
8. any_bytes.rs (both): `as` casts, splitmix64, random.near, slice indexing, cross-platform stability.
9. Lint probe on the new code (plants), if time: re-run the round-1 fuzz probe against the new API.
10. Answer (<= 30 lines), DONE.

## Findings (one line each, as found)
- [evidence] copy at a85b9dbb; cargo fmt --all --check exit 0; clippy --locked --workspace --all-targets -- -D warnings (native): 0 warnings; cargo test --locked --workspace (debug): 113 passed (26,1,3,2,19,8,15,1,17,5,1,13,1,1), any_bytes 1.17 s (endpoint) and 1.15 s (room) in debug.
- [Q3 sound, queue] invariant holds: insert admits only seq_newer(new, base) (queue.rs:72-75); take_due removes the front, base unchanged (101-108); ran(front) sets base = front, and all others lay in (front, old_base+32767] so stay newer (113-115); the room calls ran only on the command just taken and admitted (lib.rs:258-270), so newest_run == last_run.sequence whenever last_run is Some and seq_ahead in the missing count is always Some (lib.rs:261-265). rotate_right(1) on at..=len (len<16, slice non-empty) and rotate_left(1) on ..len (len>=1 by first()) cannot panic; get_mut(at..=16) gives None = Full.
- [evidence] clippy --locked --workspace --all-targets --target wasm32-unknown-unknown -- -D warnings: 0 warnings.
- [RR-1 fixed] local_match.rs:157-160 one assert! with a plain message. Players' Wasm rebuilt here (build.mjs --wasm-only, pinned wasm-bindgen 0.2.130): 53,783 raw, sha256 1e0e2dd5...0a4a (= card), gzip-9 21,984, brotli-11+hint 21,984/18,495 (build-checks.mjs sizeTable method). Card's 21,983/18,368 are 0dfca749's (sha cd63..., other bytes). Round 1 56,717/23,023/19,497; main 31,718/12,642/10,755.
- [RR-1 where] scratch parser (scratchpad/wasm-sizes.mjs: code bodies by crate from the name section): main->head +22,065 raw = code +19,206 (protocol +7,930: decode_input 1,498, encode_input 1,306, decode_control 1,260, encode_control 1,135, encode_snapshot 807, decode_snapshot 721, loopback Queue::new 470; client_endpoint +4,625: on_datagram 1,662, new 1,034, build_input 737, on_control 413, BlockWriter::write 389; room +3,249: step 1,495, on_control 646, queue insert 594, on_datagram 514; alloc +1,398; sim_wasm +1,076; core +693 = str::from_utf8 for the mode id) + name section +2,300 + data +540. Formatting/panic code: 48 functions 6,272 B at head vs 48 / 6,258 on main: the Debug impls of round 1 are gone.
