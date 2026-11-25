# rust-reviewer: change-set 4a "The recording", review round 1 (head 51efa35, source as at eb14739)
Plan: (1) read the plan section 4 + design block 1 + ADR 0004 parts 7-9 + the rules scan; (2) read the diff c1da1bd...HEAD
-- crates file by file (world.rs, state_hash.rs, room, sim-wasm, devtools src + tests); (3) safety: decode/parse_hashes/
replay/wasm exports with hostile bytes, 32-bit usize, memory before count checks, the "never a real seed" rule (writer and
reader); a fuzz-style scratch test in a copy worktree .gate/rr-4a (own --target-dir), incl. a wasm32/i686 build if the
toolchain has it; (4) identical results: usize/float/hash-map/target things reaching bytes, hashes, committed files; the
f64 division and `as` casts in scripts.rs; (5) world.rs rules vs ADR 0004 parts 7-9 and rules_scan.rs; no seed in the
shared buffer; allocation per call; (6) players' file +1,794 B: my own reading (twiggy-less: build base and head, compare
sizes, wasm-objdump/section sizes if available), cheap shrink?; (7) sha256.rs: padding at 55/56/63/64, long lengths on
32-bit, known answers vs Python hashlib; (8) API/quality, dead code, misleading names/comments, lints; (9) verdict; line
budget per fix (370 lines room for both rounds). Read-only on source; no git add/commit/push; cargo --locked only.

## Findings (one line each, as found)
