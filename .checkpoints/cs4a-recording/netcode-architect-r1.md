# netcode-architect, review round 1 of cs4a "The recording" (head 51efa35, source at eb14739)

## Plan
1. Read my design (.checkpoints/cs4-walk/netcode-architect-cs4.md block 1), plan.md s4, s8 rows 1-12, s9 row 4a, the card, plants-4a.txt, hash-report.md, his words (DECISIONS ORDER-3C, DEPART-3, STOP-4A).
2. Read the diff c1da1bd...HEAD -- crates packages: world.rs (StateSink, visit, write_shared_state), room shared_hash, sim-wasm, devtools (recording, replay, compare, scripts, files, sha256, bins, wasm.rs), tests, TS recording + tests, e2e spec, committed recording.
3. Q1 format/order/refusal/hashes file/re-make vs design. Q2 export vs DEPART-3, host.ts. Q3 each 4a mark -> a test that fails. Q4 4c needs. Q5 other wrongs.
4. Run cargo test -p devtools --locked and the bins (read-only); no browser tests; no edits outside this file.
5. Verdict + findings with line counts for fixes (room: 370 lines for both rounds).

## Findings (one line each, as found)
