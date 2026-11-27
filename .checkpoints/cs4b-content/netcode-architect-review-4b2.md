Task: netcode-architect review of change-set 4b-2 "The world", round 1, final diff b8f02b0..c442a80 (steps 9 and 10 above all).

## Plan (blocks, in order)
B1. Orientation: diff stat of b8f02b0..c442a80 (excluding .checkpoints); the card's 4b-2 section for steps 9 and 10.
B2. Route (i) as built, native: crates/devtools/src/recording.rs, files.rs, replay.rs, scripts.rs, make_recordings
    (header digest + map id, frozen content file, check before Sim::new).
B3. Route (i) in the test module (sim-wasm tests), Node runner, Chromium spec (packages/sim-worker boundary).
B4. "Loading content changed no state": recordings diff e1e2dd4/b8f02b0; plants C17-C19 in plants-4b2.py and their
    results in plants-4b2.txt.
B5. One content set per match through the constructor only (CD-7): Sim::new(seed, mode, pad_bytes, bundle) in
    crates/sim-wasm/src/lib.rs and crates/sim; Worker start; any second path that sets content.
B6. Replay of another map's recording refused (replay.rs / local_match.rs; CreateError::MapMissing, ::World).
B7. The world held by the local match and queried by no step (collision.rs, local_match.rs, step code); condition 4
    carried to 4c (card / sheet).
B8. Run tests: cargo test --locked for devtools/sim/sim-wasm natively with my target dir; the Node boundary replay test.
B9. Findings written; verdict; DONE.

## Notes (a line per finding, a line per finished block)
- B1 done: 100 files; card's step 9 cuts A/B/C (ccde0d4, e1e2dd4, fbbd84d) and step 10 wiring (885d382, c68b8d5, dcb5e6b) read (card lines 1104-1255). Next B2.
- B2 done: native route (i) as designed. scripts.rs Content{sha256,map_id} into header (scripts.rs:136-137); RECORDINGS names map (scripts.rs:154); files.rs made(folder) reads frozen as INPUT, manifest lists it first, check() accepts it as read-not-made; freeze native-only, one writer; replay.rs check() (zero header refused, map != LOCAL_MAP refused) then check_content (sha256, loader, map held) then Sim::create; replay tool reads <stem>.content.json. tests/recordings.rs: header==sha256(frozen), map_id, frozen gone/flipped refused by name. Fine.
- B3 done: test module replay_hashes(rec,bundle)/replay_state_at(rec,bundle,tick) (wasm.rs); Node replay.test.mjs: checkContent (WebCrypto) + flip refused, new Sim(seed,0,0,content), replayOn -> checkReplayable; Chromium determinism.spec.ts: checkReplayable, checkContent, content posted to Worker, new Sim(...,content). TS checkContent = sha256 only (loader + map-held left to Sim constructor: MapMissing; header map pinned to LOCAL_MAP by checkReplayable) -> equivalent to Rust's three checks.
- Note (Low candidate): Node runner order is checkContent -> new Sim -> replayOn(checkReplayable): the map/zero-header refusal comes after the constructor (replay.test.mjs:47-53), not "before any call" as card line ~1226 says; Rust (replay.rs:120-122) and Chromium (determinism.spec.ts:75-78) check first. No wrong result (the constructor ignores the header; refusal still precedes input/step). Next B4.
- B4 done: own check (node on git show b8f02b0 copy): rec 122487->122497, last 1800*68 bytes equal, header bytes 0..50 equal, header hash = sha256(first-loop.content.json) = b9901bc9...7fd3, map id "test-scene"; git diff b8f02b0..c442a80 of first-loop.hashes = line 1 only; vector.txt unchanged; git diff e1e2dd4..c442a80 -- recordings empty; content/ and content crate's readers unchanged since the freeze (only schema/ts_export test code, map.rs consts pub(crate)). Plants C17/C18/C19 (FOLD = sim-wasm lib.rs start_tick from bundle[0]) CAUGHT at step 1 natively / Node / Chromium (plants-4b2.txt:65-67). Fine.
- B5 done: CD-7 holds. Bundle crosses only at Sim::create/new (lib.rs:105-126,145); no other wasm export takes content; Worker start once (worker.ts !starting; host.ts:98 refuses a later start); isBundle 1..8 MiB plain ArrayBuffer (messages.ts). Room::new(setup, NO_CONTENT) kept as planned (reread-sheet row 10; join hash is cs8's, CD-8).
- Finding L1 (Low): local_match.rs:20 doc says NO_CONTENT is held "while there is no content data; the content crate brings the real one": stale now the match is made on a bundle with a real sha256 (header names it). Fix: say the join names zeros until change-set 8 (CD-8, join hash = bundle sha256).
- B6 done: another map's recording refused before Sim::create natively (replay.rs check, map != LOCAL_MAP) and in TS (recording.ts checkReplayable); LOCAL_MAP tied Rust<->TS (sim-wasm.test.mjs:194-196); bundle without LOCAL_MAP -> MapMissing (lib.rs tests); plants C28-C31 caught (plants-4b2.txt:76-79). Next B7.
