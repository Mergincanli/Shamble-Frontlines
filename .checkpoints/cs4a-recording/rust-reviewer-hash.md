# rust-reviewer: state hash function (PM-0004-21), change-set 4a
Plan: (1) read state_hash.rs, world.rs visit/shared plumbing, room + sim-wasm shared_hash, devtools hash_cost test,
ADR-0004 part 9 + PM-0004-21; (2) check the 32/64-bit sameness and the known answers; (3) take my own reading of
cost and of candidate replacements (scratch test under this folder, builds under .gate/); (4) reason detection
quality for accidental divergence; (5) verdict; (6) write hash-report.md (<=40 lines) + proposed header comment;
(7) at most 5 lines of plumbing defects seen on the way. Read-only on source; no git add/commit/push.

## Findings (one line each, as found)
