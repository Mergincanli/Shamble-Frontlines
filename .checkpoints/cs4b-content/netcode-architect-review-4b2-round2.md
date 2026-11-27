# netcode-architect: round 2 of change-set 4b-2 (c442a80 -> 70f11e1), 2026-11-27

## Plan (blocks)
B1. Diff stat c442a80..70f11e1 (excl. .checkpoints); confirm HEAD's later commits touch only .checkpoints.
B2. NA-1 as built: packages/sim-worker/boundary/replay.test.mjs (checkReplayable before checkContent and new Sim); run the boundary replay test.
B3. NA-2 as built: crates/sim-wasm/src/local_match.rs NO_CONTENT doc (zeros until cs8, ADR 0005 CD-8).
B4. RR-1 for the simulation: collision.rs reach/cap/NaN guard: same answer on every target (comparisons only; no new linked code), any committed recording / ray line changed (git diff of recordings, ray-line files, vectors), condition 4 still carried to 4c (report draft, PR text, card).
B5. Runs: sim + sim-wasm + devtools tests natively (own target, --locked); boundary replay/static-world tests (Node).
B6. Findings, verdict, DONE.

## Notes
- B1 done: diff stat c442a80..70f11e1 = 23 files (Rust: collision.rs +93, map.rs, schema.rs, bundle.rs, movement.rs, lib.rs doc, local_match.rs doc, content_files.rs; TS/JS; map.schema.json +1). git diff --stat 70f11e1..HEAD outside .checkpoints: empty (later commits notes only).
- B2 NA-1 as built: replay.test.mjs:48 `checkReplayable(recording)` now first, then checkContent (:49), then the flipped-content refusal, then `new Sim` (:54) and replayOn (re-checks, recording.ts:167). Order now matches native (replay.rs check before Sim::create) and Chromium. The two-instances test (:77-99) still builds Sims before replayOn(second) checks; second's steps are a bijection of recording's (index+600 mod len) with the same header, so checking second checks recording; same as round 1, not a runner of the recording's claim; no finding. CLOSED.
- B3 NA-2 as built: local_match.rs:20-21 says zeros until change-set 8 brings the bundle's sha256 into the join (ADR 0005, CD-8), and the match is made on a bundle from 4b on. Build plan (docs/reports/2026-11-17-build-plan-slice-1.md:502) puts "the bundle and hash" in local play at change-set 8; :735 the hash at a real join at Slice 3. NO_CONTENT's only uses: local_match.rs:66-67 (Room::new, Endpoint::new). Wording true. CLOSED.
