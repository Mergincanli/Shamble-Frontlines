# verifier-cs1: notes (change-set 1 "Types", PR #58)

Started 2026-11-18. Branch claude/wonderful-hamilton-jdmlnf, head cdaef8e, code commit 65a7b13, base origin/main 22c6afa
(local `main` is stale at 44446a5, 356 behind origin/main; every diff below uses origin/main).

## Plan
A. Gate and hand checks, each with exit code: gate.mjs --no-smoke; repo-map-check; cargo fmt; clippy host + wasm32;
   cargo test workspace; release test sim+protocol; wasm32 build of the five crates; cargo tree protocol (host, wasm32);
   tsc both scratch configs; vitest by path; the three netcode rules; CI gate + toolchain newest runs and their heads.
B. Read plant.sh and runs20.sh first (does each run prove its label?); run plant.sh -> .gate/cs1-types/vf-plant-results.txt
   (want "runs: 30, not as wanted: 0", exit 0), diff against plant-results.txt; run runs20.sh ->
   .gate/cs1-types/vf-runs20-results.txt (20 runs, all rewritten yes, one distinct set, sha256 07cd492f...4bc8, exit 0),
   then git status --short must be clean.
C. Symbol list: plan-draft.md s4 + plan.md item 5(g) vs every pub symbol and test in crates/protocol/src,
   crates/client-endpoint/src, packages/types/src (not generated/); compare with the report's deviations.
D. Report claims vs records: git log times/SHAs, run numbers, Cargo.lock nine entries, test counts, hashes, set hash,
   numstat size, reviewer verdicts/counts, pass marks vs ADR 0002, Mustafa quotes byte for byte, appendix texts,
   25 moved lines.
E. decisions.mjs --check; docs-keeper-blame-check.mjs (0 lines neither lane's nor moved); STATUS <= 120 lines.
F. git diff --name-only origin/main...HEAD: allowed paths only.

## Log (one line per check: command -> result)
