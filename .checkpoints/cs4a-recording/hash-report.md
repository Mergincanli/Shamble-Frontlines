# The state hash's function (PM-0004-21), change-set 4a: rust-reviewer, 2026-11-25, before the first recording
**Verdict: keep FNV-1a 64.** Revisit when the hash first runs on a live tick (server or Worker) or the shared state
passes 64 KiB; the replacement then is XXH64, written in our own code (no crate). Readings: items 8 and 9.
1. **Cost, far below the tick.** About 1.25 ns a byte, natively and in WebAssembly alike (Wasm has a native 64-bit
   multiply): 3.8 us at Walk's 3 KiB, 20 us at the planned 16 KiB (p99 about 40 us): 0.011% and 0.06% of a 33.3 ms
   tick. No live tick calls it today, only replays (devtools/src/replay.rs:120-125, sim-worker/src/recording.ts:145).
2. **Detection, enough for an accidental divergence.** A change in one byte is always caught: each step can be undone,
   so the two inputs never meet. Anything else is missed only by coincidence: at worst 1 in 7 x 10^16 per compared
   tick, when an even number of changes all sit in the top bit of bytes (two sign flips), which the lowest 8 hash bits
   cannot see (RFC 9923 section 1.4); an ideal 64-bit hash gives 1 in 1.8 x 10^19. A divergence usually stays, so a
   miss reports it a tick late; it does not hide it.
3. **The same digest on wasm32 and x86-64.** Bytes in, u64 arithmetic, no usize. In the session's runs (plants-4a.txt)
   the committed hashes and vector line replay byte for byte in Wasm, and a planted usize shows at tick 1.
4. **A second implementation is four lines** (TypeScript with BigInt too; XXH64: about 80). Mine (rr_known_answers.py),
   written from the format only, gives world.rs's 8 known answers, RFC 9923's 4 values and the vector line: 13 of 13.
5. **Changing later costs one change-set.** `make_recordings` re-makes every .hashes file; by hand: world.rs's 8 known
   answers, the vector file's label and digest (files.rs:30), the float vectors' digest (f32_ops.rs:174, unless
   fnv1a_64 stays for them). The .rec files stay. Before the swap, the old hashes must still replay green.
6. **What XXH64 would buy:** 13 to 16 times the speed from 3 KiB up (0.25 us at 3 KiB, 1.3 us at 16 KiB, native and
   Wasm; mine matches its 4 published values) and all 64 bits working (item 9). It counts only if every room hashes
   every tick: at 16 KiB and 30 Hz FNV takes 0.06% of a core per room (6% at 100 rooms), XXH64 0.004% (0.4%). Not
   measured: XXH3 (several times longer), wyhash and rapidhash (a 128-bit product, several steps in Wasm).
7. **Don't** feed FNV 8-byte words for speed: two top-bit flips in different words then cancel (10,000 of 10,000).
8. **Cost, method.** `rr_hash_bench.rs` (state_hash.rs's loop beside XXH64), rustc 1.99.1 opt-level 3, no crate, on a
   quiet 4-vCPU cloud Xeon at 2.10 GHz; 3 runs of 20,000 calls a figure. Native: per-call mean and p99 (timer included)
   and a block mean. Wasm: the same file, no wasm-opt, Node v22.23.5 (`rr_hash_bench_wasm.mjs`): block mean inside
   Wasm, p99 around each call from JavaScript. FNV: both means, ranges over the 3 runs:
   | Bytes  | FNV native: mean; p99     | FNV Wasm: mean; p99       | XXH64 block: native; Wasm |
   | 3,076  | 3.77-3.99 us; 6.5-8.9 us  | 3.57-3.99 us; 4.2-7.3 us  | 0.25; 0.24-0.26 us        |
   | 16,384 | 20.2-21.1 us; 31.7-40.9   | 19.2-21.0 us; 33.5-39.1   | 1.24-1.75; 1.22-1.42 us   |
   The session's readings agree (hash_cost.rs, release: 16,384 B 20.7 us; Wasm release module: 19.9). Writing the bytes
   (a proxy: 48 rows of 16 four-byte fields through extend_from_slice) costs 0.49 us at 3 KiB.
9. **Detection, method.** A 3,076-byte state (tick, 48 rows of 16 four-byte fields, plausible floats) and 910,999
   changed copies (910,230 distinct): every one-bit flip, field moved one unit in the last place, row swap, pair of
   fields moved, pair of sign flips, row removed, zeroed or copied. None hashes like the original; among the 910,231
   distinct states (4.1 x 10^11 pairs) no two share a hash. Top 32 bits alone: 87 shared, a random function gives 96.5
   (XXH64: 100): measured, a miss is below about 2^-32. Low 32: 2,787 (XXH64: 93). Two sign flips leave the low 8 bits
   unchanged in 3,000 of 3,000 trials, the 9th in 1,512: 56 working bits, and the 2^-56 is reasoned from that.
**Not measured:** Chromium, Firefox and Safari (Wasm is Node only); the real soldier table (a proxy); a server running
many rooms; ARM; the players' file after wasm-opt; a TypeScript implementation.
