# engine-architect, review round 1 of cs3a (head 5c923119, base origin/main 1ec6f6c)

Plan: read plan §4 + §8 + §12, my design note blocks 2 and 4, the resume card items 4-10, STATUS_ARCHIVE "For change-set 3"
(EA-F8, EA-F10 TS part, EA-F13 minus one-writer, EA-N3, EA-N6 + VF-L7); then the diff file by file in this order:
messages.ts, worker-link.ts, boot.ts, overlay/report.ts, sim-worker index.ts -> panic.rs, panic-message.ts, host.ts,
worker.ts -> crates/devtools, build.mjs (pkg-rules, wasmProblems, workerEntryChunk, bundleProblems, checkCrates,
remap) -> sim rng.rs + path-form rule -> new import edges (cargo tree, TS imports) -> ADR 0002 part 3/5/7, ADR 0004
part 8 vs plan §12 q4. One line per finding below as I go; verdict last.

## Findings (as found)
