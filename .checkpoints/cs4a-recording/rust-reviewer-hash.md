# rust-reviewer: state hash function (PM-0004-21), change-set 4a
Plan: (1) read state_hash.rs, world.rs visit/shared plumbing, room + sim-wasm shared_hash, devtools hash_cost test,
ADR-0004 part 9 + PM-0004-21; (2) check the 32/64-bit sameness and the known answers; (3) take my own reading of
cost and of candidate replacements (scratch test under this folder, builds under .gate/); (4) reason detection
quality for accidental divergence; (5) verdict; (6) write hash-report.md (<=40 lines) + proposed header comment;
(7) at most 5 lines of plumbing defects seen on the way. Read-only on source; no git add/commit/push.

## Findings (one line each, as found)
- F1 (read) state_hash.rs:15-21 FNV-1a 64 bytewise, u64 wrapping_mul, reads bytes only; world.rs:182-193 both hashes go through one Vec scratch; Room keeps its own scratch reserved at 22 B (room/lib.rs:120); Sim::shared_hash returns u64 (BigInt), no buffer out (sim-wasm/lib.rs:145).
- F2 (read) ADR 0004 PM-0004-21 (line 1142) asks: function picked, cost per tick (mean, p99), same digest for a committed byte string on x86-64 and wasm32. Wasm reading in replay.test.mjs:108-121 gives a mean only (one timed block of 20,000 calls), no p99.
- F3 (read) other users of fnv1a_64: devtools/src/files.rs:30-32 writes the label "fnv1a64" into the committed .vector.txt; devtools/src/rules/f32_ops.rs:174 and vectors/make_f32_vectors.py:292 digest the float vectors with it. A later swap must keep fnv1a_64 for those or re-make them.
- F4 (measured) independent Python FNV-1a 64 + canonical writer (rr_known_answers.py, written from the documented format, not the Rust): the 4 RFC vectors, all 8 known answers in world.rs:367-394 and the committed first-loop.vector.txt line (22 B -> 9928339d0eca76ae) all agree: 13 of 13.
